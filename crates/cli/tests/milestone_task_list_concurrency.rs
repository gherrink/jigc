//! **The `tasks.json` atomicity rider** (M46 Increment 1, T3 — the un-swept sibling
//! of `DECISIONS.md` 2026-07-23 M45 Settle Decision 9): concurrent writers driving a
//! milestone's task list through the **real writers** must never let a reader observe
//! a file that parses as no task list at all, and must leave no `.tmp` residue behind.
//!
//! `.jigc/milestones/<id>/tasks.json` is the second **shared, non-task-isolated**
//! engine state file (`design/team-ready-state.md` → The `.jigc` layer is shared
//! across worktrees): every sub-task of a fan-out lives under one milestone area, and
//! the whole worktree set shares one `.jigc/`. M45's rationale for `file-state.json`
//! (`engine::file_state::FileStateRecord::save`'s doc-comment) therefore applies to it
//! verbatim, and until this rider all four of its writers — `mint_milestone`,
//! `add_task`, `drop_sub_tasks`, `reseed_cache_from_record` — were a plain
//! `std::fs::write`, which **truncates the target and then fills it**: a reader landing
//! inside that window gets zero or partial bytes, and `read_task_list` fails to parse
//! what it just read.
//!
//! The rider is about **atomicity only**. `tasks.json` is deliberately **excluded from
//! T1's base-relative merge**, on the evidence recorded beside its writers
//! (`engine::milestone::TASKS_FILE`) — the exclusion is from the merge, never from
//! temp+rename.
//!
//! The cell drives `add_task` (append) and `drop_sub_tasks` (retain-and-rewrite)
//! concurrently over disjoint per-writer intent namespaces, so the serialized list keeps
//! swinging in length turn to turn — the property that makes a torn write observable
//! (a short read that happens to be a prefix of the previous, longer file is exactly
//! what an unvarying length would hide). Deltas lost to a concurrent read-modify-write
//! are **not** asserted on: that is the merge property this file excludes by design.

use engine::milestone::{add_task, drop_sub_tasks, mint_milestone, read_task_list};
use engine::state::BasePin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

/// A throwaway `.jigc/` root that removes itself on drop.
struct TempRoot(std::path::PathBuf);

impl TempRoot {
    fn new() -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-tasks-concurrency-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&path).expect("create temp root");
        TempRoot(path)
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Concurrency pressure: four writers, each taking enough turns that the truncate
/// window is entered hundreds of times while the readers spin.
const WRITERS: usize = 4;
const ITERS: usize = 60;

/// How often a writer shrinks the list again — every `DROP_EVERY`-th turn it drops the
/// ids it has appended so far, so the file's byte length swings both ways instead of
/// growing monotonically.
const DROP_EVERY: usize = 5;

#[test]
fn concurrent_task_list_writers_never_yield_an_unparseable_read() {
    let root = TempRoot::new();
    let jigc_root = root.0.clone();
    let base = BasePin::new("0".repeat(40), "0000000");
    let milestone =
        mint_milestone(&jigc_root, "Cache rework", base).expect("the milestone mints once");
    let dir = milestone.dir.clone();

    let stop = Arc::new(AtomicBool::new(false));

    // Writers: `add_task` appends, `drop_sub_tasks` rewrites the survivors. Intents are
    // unique per (writer, turn), so no two writers ever contend for one id — every
    // rewrite is a genuine whole-file write of a differing length.
    let mut handles = Vec::new();
    for w in 0..WRITERS {
        let jr = jigc_root.clone();
        let milestone_id = milestone.id.clone();
        handles.push(thread::spawn(move || {
            let mut mine: Vec<String> = Vec::new();
            for iter in 0..ITERS {
                let intent = format!("Writer {w} area {iter}");
                let added = add_task(&jr, &milestone_id, &intent, "single-task")
                    .expect("the sub-task mints and appends");
                mine.push(added.task.id);
                if iter % DROP_EVERY == DROP_EVERY - 1 {
                    drop_sub_tasks(&jr, &milestone_id, &mine).expect("the drop rewrites the list");
                    mine.clear();
                }
            }
        }));
    }

    // Readers: every `read_task_list` must parse. A truncated or partial file surfaces
    // as `Err` (a serde error mapped to `InvalidData`), which is the torn read.
    let mut readers = Vec::new();
    for _ in 0..WRITERS {
        let dir = dir.clone();
        let stop = Arc::clone(&stop);
        readers.push(thread::spawn(move || {
            let mut reads: u64 = 0;
            while !stop.load(Ordering::Relaxed) {
                read_task_list(&dir).unwrap_or_else(|err| {
                    panic!("a concurrent read of the task list returned an unparseable file: {err}")
                });
                reads += 1;
            }
            reads
        }));
    }

    for h in handles {
        h.join().expect("a writer thread panicked (a write failed)");
    }
    stop.store(true, Ordering::Relaxed);
    let mut reads = 0u64;
    for h in readers {
        reads += h.join().expect("a reader thread panicked (a torn read)");
    }
    assert!(
        reads > 0,
        "the readers never ran, so this cell would pass on a torn-write tree",
    );

    // The settled file parses, and the persist left no temp sibling behind — the
    // atomic write's residue is consumed by the `rename`, never abandoned in the
    // milestone area a `jigc milestone list-tasks` reads.
    read_task_list(&dir).expect("the settled task list parses");
    let residue: Vec<String> = std::fs::read_dir(&dir)
        .expect("read the milestone area")
        .filter_map(|entry| Some(entry.ok()?.file_name().to_string_lossy().into_owned()))
        .filter(|name| name.ends_with(".tmp"))
        .collect();
    assert!(
        residue.is_empty(),
        "the milestone area kept temp residue after the writers settled: {residue:?}",
    );
}
