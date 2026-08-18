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

use engine::file_state::FileStateRecord;
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
            for iter in 0..SURVIVAL_ITERS {
                let mut rec = FileStateRecord::load(&jr).expect("writer load parses");
                rec.record(survival_key(w, iter), survival_hash(w, iter));
                rec.save(&jr).expect("writer save succeeds");
            }
        }));
    }
    for h in handles {
        h.join().expect("writer thread panicked (a save failed)");
    }

    let settled = FileStateRecord::load(&jigc_root).expect("post-hoc load parses");
    let expected = survival_universe();
    let got: std::collections::BTreeSet<String> = settled.hashes.keys().cloned().collect();
    let lost: Vec<&String> = expected.difference(&got).collect();
    assert!(
        lost.is_empty(),
        "{} of {} concurrently recorded keys were accepted and then discarded by a \
         later save (first lost: {:?})",
        lost.len(),
        expected.len(),
        lost.first(),
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
    seed.save(&jigc_root).expect("seed save succeeds");

    let mut handles = Vec::new();
    for w in 0..SURVIVAL_WRITERS {
        let jr = jigc_root.clone();
        handles.push(thread::spawn(move || {
            for iter in 0..SURVIVAL_ITERS {
                let mut rec = FileStateRecord::load(&jr).expect("writer load parses");
                rec.forget(&survival_key(w, iter));
                rec.save(&jr).expect("writer save succeeds");
            }
        }));
    }
    for h in handles {
        h.join().expect("writer thread panicked (a save failed)");
    }

    let settled = FileStateRecord::load(&jigc_root).expect("post-hoc load parses");
    let resurrected: Vec<&String> = settled.hashes.keys().collect();
    assert!(
        resurrected.is_empty(),
        "{} of {} concurrently forgotten keys were resurrected by a later save \
         (first: {:?})",
        resurrected.len(),
        survival_universe().len(),
        resurrected.first(),
    );
}

/// **Cell (c) — the degrade.** The lock is held from a **second fd** for longer than the
/// spin ceiling (`File::lock` is per-open-file-description, so a second handle in this
/// same process contends exactly like another process would). `save` must then run its
/// critical section **anyway**: return `Ok`, still merge against what is on disk, and
/// return inside a bounded time.
///
/// The elapsed lower bound is what makes this cell non-vacuous: on a tree whose `save`
/// takes no lock it returns immediately and every other assertion here passes. Spinning
/// the full ceiling is the observable proof that the exclusion was attempted, and
/// returning at all is the proof it degrades rather than blocks — a rebuildable cache
/// must never wedge a command behind a lock a crashed sibling still holds.
#[test]
fn a_lock_held_past_the_spin_ceiling_degrades_to_a_merging_save() {
    let root = TempRoot::new();
    let jigc_root = root.0.clone();

    // What is on disk: a key our record has never seen, so a clobbering write would
    // drop it and a merging write must keep it.
    let mut seed = FileStateRecord::new();
    seed.record("docs/theirs.md", format!("{:064x}", 1u128));
    seed.save(&jigc_root).expect("seed save succeeds");

    // Our record: built from scratch, so its base is empty and its one key is its own
    // delta.
    let mut ours = FileStateRecord::new();
    ours.record("docs/ours.md", format!("{:064x}", 2u128));

    // Hold the lock from a second fd for the whole of the save below.
    let lock_path = engine::state::lock_sibling(&FileStateRecord::path_in(&jigc_root));
    let holder = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(&lock_path)
        .expect("open the lock sibling");
    holder.lock().expect("hold the lock from a second fd");

    let ceiling = engine::state::SAVE_LOCK_SPIN * engine::state::SAVE_LOCK_ATTEMPTS;
    let (tx, rx) = std::sync::mpsc::channel();
    let jr = jigc_root.clone();
    let worker = thread::spawn(move || {
        let started = std::time::Instant::now();
        let outcome = ours.save(&jr);
        let _ = tx.send((outcome, started.elapsed()));
    });

    // Bounded: the watchdog is the "not a hang" half of the claim.
    let (outcome, elapsed) = rx
        .recv_timeout(ceiling + std::time::Duration::from_secs(60))
        .expect("a save behind a held lock must return, not block forever");
    worker.join().expect("the saving thread panicked");
    holder.unlock().expect("release the held lock");
    drop(holder);

    outcome.expect("the degrading save reports success");
    assert!(
        elapsed >= ceiling * 9 / 10,
        "the save returned in {elapsed:?}, short of the {ceiling:?} spin ceiling — it \
         never attempted the lock, so this cell would pass on a lock-less tree",
    );

    let settled = FileStateRecord::load(&jigc_root).expect("post-hoc load parses");
    let got: Vec<&str> = settled.hashes.keys().map(String::as_str).collect();
    assert_eq!(
        got,
        vec!["docs/ours.md", "docs/theirs.md"],
        "the degrading save must still merge against what is on disk, never clobber it",
    );
}
