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
//! This suite asserts **parseability only**, never cross-writer survival: without the
//! save-scoped lock the read-modify-write window between the merge's re-read and the
//! `rename` is still open, so a delta can still be lost under true concurrency. That
//! window is the next task's claim; asserting it here would be flaky-red.

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
