//! Concurrency-corruption subset (M45 Increment 7, T2 — `DECISIONS.md` 2026-07-23
//! M45 Settle Decision 9): five concurrent `FileStateRecord::save` writers on **one
//! shared `jigc_root`**, with concurrent and post-hoc `FileStateRecord::load`, must
//! never produce an unparseable read.
//!
//! `.jigc/state/*` is **not** isolated per task (unlike task doc areas and worktree
//! code), so two writers hitting `file-state.json` at once is a real race. Before
//! the fix, `save` did a direct `std::fs::write` to the final path (a reader could
//! observe a truncated file) and — even routed through the atomic temp+rename —
//! two writers shared one `<name>.tmp` sibling whose writes interleaved. After the
//! fix, `save` routes through `state::persist` and the temp sibling is
//! **process-unique** (`<name>.<pid>.<nanos>.tmp`), so no reader ever sees a
//! partial or interleaved file: every `load` parses cleanly.
//!
//! **M46 Increment 1 — why the writers now load before they mutate.** `save` merges
//! base-relative against what is on disk, so a writer that saved a *fresh* record
//! would contribute its keys to a union that only ever grows: the on-disk byte length
//! would converge to a constant, and byte-length variance is precisely the property
//! that surfaces a torn write. So each writer is now a real `load → mutate → save`
//! over its **own key namespace** (`docs/w<writer>-doc-*`), reshaping that namespace
//! between 1 and 40 entries per turn — the length keeps swinging, so this suite's
//! M45 claim stays live under the merge.
//!
//! **M46 Increment 1 — the save-scoped lock (T2).** The merge alone closes only the
//! *sequential* interleave. Under true concurrency the read-modify-write window between
//! `save`'s re-read of `theirs` and its `rename` is still open: a sibling that lands its
//! `rename` inside that window is merged against a disk state that predates it, and its
//! delta is overwritten by a `save` that returned `Ok`. So `save` now runs its whole
//! critical section under an advisory lock on a **stable `.lock` sibling** of the target
//! (never the target's own fd — `state::write_atomic` replaces the target inode on every
//! write, so a lock on that fd would guard an inode the next persist orphans). This suite
//! therefore asserts **cross-writer survival** as well as parseability, over the mutation
//! kinds the record has: `record` and `forget`.
//!
//! **Why the survival cells report their own save degrades (2026-09-30), and why a save
//! that cannot take the lock now fails (2026-10-01).** Both cells flaked on 2-vCPU CI
//! runners (and under `dev/runner-faithful --cpus 2`) with a handful of keys lost or
//! resurrected, while 60 isolated runs stayed green. The leading hypothesis was the
//! lock's declared degrade — a spin that exhausted its ceiling ran the critical section
//! unlocked, merged against a stale disk state and could drop a sibling's delta — so each
//! red cell was made to name how many of its saves ran unlocked, from
//! [`engine::state::save_degrades`]. PR #5's CI run `36927086957` then lost one key with
//! *1 ran unlocked (lock timeout 1)*, which proved it, and the degrade is gone: a save
//! that cannot take the lock within `SAVE_LOCK_BUDGET` fails with nothing written, so a
//! survival cell that hits it now fails on that save's own error (its `expect` names the
//! lock file) rather than on a missing key. The tally's lock arms retired with the
//! degrade; what it still counts is the in-lock re-read that found the record
//! unreadable — per thread, so cell (d), which forces one in this same process, cannot
//! pollute a survival cell's count. The record lives in `implementation/decisions-pending.md`
//! → the CI block.

use engine::file_state::FileStateRecord;
use engine::index::EdgeIndex;
use engine::state::{SaveDegrades, save_degrades};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

/// A throwaway `.jigc/` root that removes itself on drop.
struct TempRoot(std::path::PathBuf);

impl TempRoot {
    fn new() -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-fs-concurrency-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
        std::fs::create_dir_all(&path).expect("create temp root");
        TempRoot(path)
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A record whose serialized byte length varies with `n` — so a torn or
/// interleaved write leaves a file whose bytes belong to no single record and
/// fails to parse. Each entry is a valid 64-hex blake3-shaped hash string.
fn record_of_size(writer: usize, n: usize) -> FileStateRecord {
    let mut rec = FileStateRecord::new();
    reshape(&mut rec, writer, n);
    rec
}

/// Reshape `writer`'s **own** key namespace inside `rec` to exactly `n` entries,
/// dropping whatever it held before. Every writer owns a disjoint `docs/w<writer>-`
/// prefix, so a turn's `forget`s and `record`s are genuine per-key deltas against
/// the record the writer just loaded — and the serialized length keeps swinging
/// turn to turn, which is what makes a torn write observable.
fn reshape(rec: &mut FileStateRecord, writer: usize, n: usize) {
    let prefix = format!("docs/w{writer}-doc-");
    let mine: Vec<String> = rec
        .hashes
        .keys()
        .filter(|k| k.starts_with(&prefix))
        .cloned()
        .collect();
    for key in mine {
        rec.forget(&key);
    }
    for i in 0..n {
        rec.record(
            format!("{prefix}{i:04}.md"),
            format!("{:064x}", (writer * 1000 + i) as u128),
        );
    }
}

#[test]
fn five_concurrent_state_writers_never_yield_an_unparseable_read() {
    let root = TempRoot::new();
    let jigc_root = root.0.clone();
    // Seed the file so readers have something to parse from the very first tick.
    record_of_size(0, 1)
        .save(&jigc_root)
        .expect("seed save succeeds");

    let stop = Arc::new(AtomicBool::new(false));
    const WRITERS: usize = 5;
    const ITERS: usize = 200;

    let mut handles = Vec::new();

    // Five writers, each repeatedly saving records of swinging sizes so byte
    // lengths differ turn to turn — the condition that surfaces a torn write.
    for w in 0..WRITERS {
        let jr = jigc_root.clone();
        handles.push(thread::spawn(move || {
            for iter in 0..ITERS {
                // Sizes fan between small and large to maximise length variance.
                let n = 1 + ((iter * 7 + w * 3) % 40);
                // load → mutate → save: a real writer's shape, and the one the
                // base-relative merge is defined against.
                let mut rec = FileStateRecord::load(&jr).expect("writer load parses");
                reshape(&mut rec, w, n);
                rec.save(&jr).expect("writer save succeeds");
            }
        }));
    }

    // Concurrent readers: every load must parse. A serde parse error surfaces as
    // an `Err` from `load` (serde_json::Error -> io::Error via `?`).
    let mut reader_handles = Vec::new();
    for _ in 0..WRITERS {
        let jr = jigc_root.clone();
        let stop = Arc::clone(&stop);
        reader_handles.push(thread::spawn(move || {
            let mut reads: u64 = 0;
            while !stop.load(Ordering::Relaxed) {
                FileStateRecord::load(&jr).unwrap_or_else(|e| {
                    panic!("concurrent load returned an unparseable read: {e}")
                });
                reads += 1;
            }
            reads
        }));
    }

    for h in handles {
        h.join().expect("writer thread panicked (a save failed)");
    }
    stop.store(true, Ordering::Relaxed);
    for h in reader_handles {
        h.join()
            .expect("reader thread panicked (an unparseable read)");
    }

    // Post-hoc: the settled file parses and is one of the writers' records.
    let final_rec = FileStateRecord::load(&jigc_root).expect("post-hoc load parses");
    assert!(
        !final_rec.hashes.is_empty(),
        "the settled record is non-empty"
    );
}

// ---------------------------------------------------------------------------
// The save-scoped lock (M46 Increment 1, T2)
// ---------------------------------------------------------------------------

/// The concurrency pressure the two survival cells run under. Six writers is enough
/// contention that the read-modify-write window is hit many times over within the
/// iteration count, and the record grows to `WRITERS * SURVIVAL_ITERS` keys, so each
/// turn's serialize+write is long enough for a sibling's `rename` to land inside it.
const SURVIVAL_WRITERS: usize = 6;
const SURVIVAL_ITERS: usize = 100;

/// The key one writer contributes on turn `iter`. Namespaces are disjoint per writer,
/// so nothing here is a genuine conflict: every key has exactly one author, and any
/// key missing at the end is a delta a `save` accepted and then threw away.
fn survival_key(writer: usize, iter: usize) -> String {
    format!("docs/survivor-w{writer}-{iter:04}.md")
}

fn survival_hash(writer: usize, iter: usize) -> String {
    format!("{:064x}", (writer * 100_000 + iter) as u128)
}

/// The full key set the two survival cells expect to be accounted for.
fn survival_universe() -> std::collections::BTreeSet<String> {
    (0..SURVIVAL_WRITERS)
        .flat_map(|w| (0..SURVIVAL_ITERS).map(move |i| survival_key(w, i)))
        .collect()
}

/// Run `work` on the **current** thread and return the save degrades it incurred.
/// The tally is per-thread, so this is exactly the degrades of the saves `work` made.
fn degrades_during(work: impl FnOnce()) -> SaveDegrades {
    let before = save_degrades();
    work();
    let after = save_degrades();
    SaveDegrades {
        unreadable_theirs: after.unreadable_theirs - before.unreadable_theirs,
    }
}

/// The field-wise sum of the tallies a cell's threads returned.
fn total(tallies: impl IntoIterator<Item = SaveDegrades>) -> SaveDegrades {
    tallies
        .into_iter()
        .fold(SaveDegrades::default(), |acc, t| SaveDegrades {
            unreadable_theirs: acc.unreadable_theirs + t.unreadable_theirs,
        })
}

/// The clause a red survival cell appends, so the next failure names its own cause. A
/// save that could not take the lock fails rather than running unlocked, so it never
/// reaches this clause — its writer's `expect` panics with the lock error instead.
fn degrade_clause(d: &SaveDegrades) -> String {
    format!(
        "save degrades over this cell's saves: {} re-read an unreadable record",
        d.unreadable_theirs,
    )
}

/// **Cell (a) — the `record` axis.** Six threads each `load → record(own key) → save`
/// against one shared root. Every writer's key must be on disk when they have all
/// joined.
///
/// Red before the lock: a saver that re-read `theirs` before a sibling's `rename` landed
/// merges against a disk state that no longer exists, then `rename`s its own merge over
/// the sibling's — a delta accepted, reported `Ok`, and gone.
#[test]
fn every_concurrent_record_survives_the_shared_save() {
    let root = TempRoot::new();
    let jigc_root = root.0.clone();

    let mut handles = Vec::new();
    for w in 0..SURVIVAL_WRITERS {
        let jr = jigc_root.clone();
        handles.push(thread::spawn(move || {
            degrades_during(|| {
                for iter in 0..SURVIVAL_ITERS {
                    let mut rec = FileStateRecord::load(&jr).expect("writer load parses");
                    rec.record(survival_key(w, iter), survival_hash(w, iter));
                    rec.save(&jr).expect("writer save succeeds");
                }
            })
        }));
    }
    let degrades = total(
        handles
            .into_iter()
            .map(|h| h.join().expect("writer thread panicked (a save failed)")),
    );

    let settled = FileStateRecord::load(&jigc_root).expect("post-hoc load parses");
    let expected = survival_universe();
    let got: std::collections::BTreeSet<String> = settled.hashes.keys().cloned().collect();
    let lost: Vec<&String> = expected.difference(&got).collect();
    assert!(
        lost.is_empty(),
        "{} of {} concurrently recorded keys were accepted and then discarded by a \
         later save (first lost: {:?}); {}",
        lost.len(),
        expected.len(),
        lost.first(),
        degrade_clause(&degrades),
    );
}

/// **Cell (b) — the `forget` axis.** The same shape over the inverse mutation: the
/// universe is seeded first, then six threads each `load → forget(own key) → save`.
/// Every writer's retirement must hold; a resurrected key is the same lost delta with
/// the sign flipped, and it is the one that re-arms a stale baseline.
#[test]
fn every_concurrent_forget_survives_the_shared_save() {
    let root = TempRoot::new();
    let jigc_root = root.0.clone();

    let mut seed = FileStateRecord::new();
    for w in 0..SURVIVAL_WRITERS {
        for iter in 0..SURVIVAL_ITERS {
            seed.record(survival_key(w, iter), survival_hash(w, iter));
        }
    }
    let seeding = degrades_during(|| seed.save(&jigc_root).expect("seed save succeeds"));

    let mut handles = Vec::new();
    for w in 0..SURVIVAL_WRITERS {
        let jr = jigc_root.clone();
        handles.push(thread::spawn(move || {
            degrades_during(|| {
                for iter in 0..SURVIVAL_ITERS {
                    let mut rec = FileStateRecord::load(&jr).expect("writer load parses");
                    rec.forget(&survival_key(w, iter));
                    rec.save(&jr).expect("writer save succeeds");
                }
            })
        }));
    }
    let degrades = total(
        std::iter::once(seeding).chain(
            handles
                .into_iter()
                .map(|h| h.join().expect("writer thread panicked (a save failed)")),
        ),
    );

    let settled = FileStateRecord::load(&jigc_root).expect("post-hoc load parses");
    let resurrected: Vec<&String> = settled.hashes.keys().collect();
    assert!(
        resurrected.is_empty(),
        "{} of {} concurrently forgotten keys were resurrected by a later save \
         (first: {:?}); {}",
        resurrected.len(),
        survival_universe().len(),
        resurrected.first(),
        degrade_clause(&degrades),
    );
}

/// **Cell (e) — the copy-in door under contention** (the rc.24 fix pass, `(R3, F7)`).
/// [`engine::file_state::read_for_copy_in`] made every task's first write a writer of this
/// record — under a fan-out, N sub-agents at once, a population the file never had. Six
/// threads each copy in their **own** docs (a key per turn) while all of them also copy in
/// **one shared** doc on every turn.
///
/// Two properties, both of which the lock carries: no thread's key is lost to another's
/// save, and the shared doc is recorded **once** — record-if-absent is decided under the
/// lock, so exactly one copy-in reports `Adopted` and every other reads the held key.
#[test]
fn concurrent_copy_ins_lose_no_key_and_adopt_a_shared_doc_once() {
    use engine::file_state::{CopyInBaseline, hash_bytes, read_for_copy_in};

    const SCHEMA: &[u8] =
        b"type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: {}\n";
    const ITERS: usize = 25;
    let schema = Arc::new(engine::schema::load_schema(SCHEMA).expect("the fixture schema loads"));
    let body = |title: &str| format!("---\n---\n\n# {title}\n\n## Body\n\nProse.\n");

    let root = TempRoot::new();
    let jigc_root = root.0.join(".jigc");
    let notes = root.0.join("notes");
    std::fs::create_dir_all(&notes).expect("mk notes/");
    let shared_home = notes.join("shared.md");
    std::fs::write(&shared_home, body("Shared")).expect("write the shared doc");
    for w in 0..SURVIVAL_WRITERS {
        for iter in 0..ITERS {
            std::fs::write(
                notes.join(format!("w{w}-{iter:04}.md")),
                body(&format!("W{w} {iter}")),
            )
            .expect("write a writer's doc");
        }
    }

    let handles: Vec<_> = (0..SURVIVAL_WRITERS)
        .map(|w| {
            let (jr, notes, shared_home, schema) = (
                jigc_root.clone(),
                notes.clone(),
                shared_home.clone(),
                Arc::clone(&schema),
            );
            thread::spawn(move || {
                let mut shared_adoptions = 0usize;
                for iter in 0..ITERS {
                    let name = format!("w{w}-{iter:04}.md");
                    let own = read_for_copy_in(
                        &jr,
                        &notes.join(&name),
                        &format!("notes/{name}"),
                        &schema,
                        || false,
                    )
                    .expect("a writer's own copy-in succeeds");
                    assert_eq!(own.baseline, CopyInBaseline::Adopted, "{name}");
                    let shared =
                        read_for_copy_in(&jr, &shared_home, "notes/shared.md", &schema, || false)
                            .expect("the shared copy-in succeeds");
                    if shared.baseline == CopyInBaseline::Adopted {
                        shared_adoptions += 1;
                    }
                }
                shared_adoptions
            })
        })
        .collect();
    let shared_adoptions: usize = handles
        .into_iter()
        .map(|h| h.join().expect("a copy-in thread panicked"))
        .sum();

    assert_eq!(
        shared_adoptions, 1,
        "the shared doc is adopted exactly once — record-if-absent is decided under the lock",
    );
    let settled = FileStateRecord::load(&jigc_root).expect("post-hoc load parses");
    assert_eq!(
        settled.get("notes/shared.md"),
        Some(hash_bytes(body("Shared").as_bytes()).as_str()),
    );
    let lost: Vec<String> = (0..SURVIVAL_WRITERS)
        .flat_map(|w| (0..ITERS).map(move |iter| format!("notes/w{w}-{iter:04}.md")))
        .filter(|key| settled.get(key).is_none())
        .collect();
    assert!(
        lost.is_empty(),
        "{} copy-in baselines were recorded and then lost to a sibling's save (first: {:?})",
        lost.len(),
        lost.first(),
    );
}

/// Hold the advisory lock on `target`'s [`engine::state::lock_sibling`] from a **second
/// fd** (`File::lock` is per-open-file-description, so a second handle in this same
/// process contends exactly like another process would). Released when dropped.
fn hold_save_lock(target: &std::path::Path) -> std::fs::File {
    let holder = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(engine::state::lock_sibling(target))
        .expect("open the lock sibling");
    holder.lock().expect("hold the lock from a second fd");
    holder
}

/// **Cell (c) — a save never runs unlocked.** Both shared-cache writers that take the
/// save lock — [`FileStateRecord::save`] and [`EdgeIndex::save`] — are driven behind a
/// lock held from a second fd for longer than the wait budget. Each save must then
/// **fail**, with an error naming the lock file and a retry route, and **write nothing**:
/// the bytes on disk are the seed's, byte for byte, and the critical section never ran
/// (the tally of in-lock re-reads is untouched).
///
/// Until 2026-10-01 the arm this replaces asserted the opposite — that the save ran its
/// critical section anyway once the spin ran out, which is how PR #5's CI run
/// `36927086957` lost a concurrent writer's key (`decisions-pending.md` → the CI block).
///
/// The elapsed lower bound is what makes this cell non-vacuous: a save that took no lock
/// at all would return at once, and a save that gave up early would not have honoured
/// the budget. The watchdog is the "never a hang" half: the error arrives, it is not
/// waited for forever.
#[test]
fn a_lock_held_past_the_wait_budget_fails_the_save_and_writes_nothing() {
    let root = TempRoot::new();
    let jigc_root = root.0.clone();

    // What is on disk: a seed each writer would change, so a write that slipped through
    // shows up as different bytes.
    let mut seed = FileStateRecord::new();
    seed.record("docs/theirs.md", format!("{:064x}", 1u128));
    seed.save(&jigc_root).expect("seed the file-state record");
    let seed_index = EdgeIndex {
        stamp: "seed".into(),
        edges: Vec::new(),
    };
    seed_index.save(&jigc_root).expect("seed the edge index");
    let record_path = FileStateRecord::path_in(&jigc_root);
    let index_path = EdgeIndex::path_in(&jigc_root);
    let record_before = std::fs::read(&record_path).expect("read the seeded record");
    let index_before = std::fs::read(&index_path).expect("read the seeded index");

    let mut ours = FileStateRecord::new();
    ours.record("docs/ours.md", format!("{:064x}", 2u128));
    let ours_index = EdgeIndex {
        stamp: "ours".into(),
        edges: Vec::new(),
    };

    let record_holder = hold_save_lock(&record_path);
    let index_holder = hold_save_lock(&index_path);

    let budget = engine::state::SAVE_LOCK_BUDGET;
    let (tx, rx) = std::sync::mpsc::channel();
    let record_worker = {
        let (tx, jr) = (tx.clone(), jigc_root.clone());
        thread::spawn(move || {
            let started = std::time::Instant::now();
            let mut outcome = None;
            let degrades = degrades_during(|| outcome = Some(ours.save(&jr)));
            let _ = tx.send((
                "file-state",
                outcome.expect("the save ran"),
                started.elapsed(),
                degrades,
            ));
        })
    };
    let index_worker = {
        let jr = jigc_root.clone();
        thread::spawn(move || {
            let started = std::time::Instant::now();
            let mut outcome = None;
            let degrades = degrades_during(|| outcome = Some(ours_index.save(&jr)));
            let _ = tx.send((
                "edge index",
                outcome.expect("the save ran"),
                started.elapsed(),
                degrades,
            ));
        })
    };

    let deadline = budget + std::time::Duration::from_secs(60);
    let results: Vec<_> = (0..2)
        .map(|_| {
            rx.recv_timeout(deadline)
                .expect("a save behind a held lock must return, not block forever")
        })
        .collect();
    record_worker
        .join()
        .expect("the record-saving thread panicked");
    index_worker
        .join()
        .expect("the index-saving thread panicked");
    drop(record_holder);
    drop(index_holder);

    for (writer, outcome, elapsed, degrades) in results {
        let lock = engine::state::lock_sibling(if writer == "file-state" {
            &record_path
        } else {
            &index_path
        });
        let err = match outcome {
            Ok(()) => panic!(
                "the {writer} save behind a lock held past the {budget:?} budget returned Ok \
                 after {elapsed:?} — it ran its critical section without the lock"
            ),
            Err(err) => err,
        };
        assert_eq!(
            err.kind(),
            std::io::ErrorKind::TimedOut,
            "the {writer} save's error must be a timeout; got {err}",
        );
        let message = err.to_string();
        assert!(
            message.contains(&lock.display().to_string()) && message.contains("retry"),
            "the {writer} save's error must name the lock file {} and a retry route; got: \
             {message}",
            lock.display(),
        );
        assert!(
            elapsed >= budget * 9 / 10,
            "the {writer} save failed after {elapsed:?}, short of the {budget:?} budget — it \
             did not wait the budget out",
        );
        assert_eq!(
            degrades,
            SaveDegrades::default(),
            "the {writer} save's critical section must not have run, so it tallies nothing",
        );
    }

    assert_eq!(
        std::fs::read(&record_path).expect("re-read the record"),
        record_before,
        "the failed file-state save must write nothing",
    );
    assert_eq!(
        std::fs::read(&index_path).expect("re-read the index"),
        index_before,
        "the failed edge-index save must write nothing",
    );
}

/// **Cell (d) — a lock that cannot be opened fails the save too, and the one silent
/// degrade left is tallied on its own arm.** Cell (c) fences the wait budget; this fences
/// the rest of the lock's failure axis and the rest of the axis `save_degrades` reports:
///
/// - **unopenable lock** — the lock sibling cannot be opened (here: a directory squats its
///   path), so the save fails at once, naming the lock file, and writes nothing — it used
///   to run unlocked;
/// - **unreadable re-read** — the record on disk does not parse, so the in-lock re-read
///   degrades to `theirs = ours`, the clobbering pre-merge write. That save still
///   succeeds: the tally observes the degrade, it never turns it into an error.
#[test]
fn an_unopenable_lock_fails_the_save_and_an_unreadable_reread_is_tallied() {
    // Unopenable lock.
    let root = TempRoot::new();
    let jigc_root = root.0.clone();
    let target = FileStateRecord::path_in(&jigc_root);
    let lock_path = engine::state::lock_sibling(&target);
    std::fs::create_dir_all(&lock_path).expect("squat the lock sibling with a directory");
    let mut rec = FileStateRecord::new();
    rec.record("docs/a.md", format!("{:064x}", 1u128));
    let mut outcome = None;
    let degrades = degrades_during(|| outcome = Some(rec.save(&jigc_root)));
    let err = outcome
        .expect("the save ran")
        .expect_err("a save whose lock cannot be opened must fail, never run unlocked");
    let message = err.to_string();
    assert!(
        message.contains(&lock_path.display().to_string()) && message.contains("retry"),
        "the error must name the lock file {} and a retry route; got: {message}",
        lock_path.display(),
    );
    assert!(
        !target.exists(),
        "a save that could not take its lock must write nothing, but {} exists",
        target.display(),
    );
    assert_eq!(
        degrades,
        SaveDegrades::default(),
        "the failed save's critical section must not have run, so it tallies nothing",
    );

    // Unreadable re-read.
    let root = TempRoot::new();
    let jigc_root = root.0.clone();
    let target = FileStateRecord::path_in(&jigc_root);
    std::fs::create_dir_all(target.parent().expect("the record has a parent dir"))
        .expect("create the state dir");
    std::fs::write(&target, b"{ not json").expect("plant an unparseable record");
    let degrades = degrades_during(|| rec.save(&jigc_root).expect("a clobbering save still saves"));
    assert_eq!(
        degrades,
        SaveDegrades {
            unreadable_theirs: 1,
        },
        "an unparseable record re-read under the lock must be tallied as exactly one \
         unreadable re-read",
    );
}
