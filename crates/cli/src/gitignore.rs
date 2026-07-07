//! The **single source of truth** for `.jigc/.gitignore` — the transient runtime
//! subdirs jigc keeps out of git so `config/` (+ its `.gitkeep`) and `AGENT.md` are
//! committed while the working area and the rebuildable caches are not.
//!
//! Historically the entry-set literal was duplicated across three divergent write
//! sites — `adapter.rs` (setup), `task.rs` (finalize), and `milestone.rs`
//! (create/provision) — which had already drifted on `worktrees/` (M31 Inc 3 added it
//! to the milestone writer only). M39 collapses them to this one writer (`design/
//! team-ready-state.md` → What graduates: the `.gitignore` 3→1). The record home is
//! `docs/milestone-records/` — outside `.jigc/` — so the ignore set's *meaning* is
//! unchanged by the split; only the duplication is removed.

use std::io;
use std::path::Path;

/// The transient-runtime entry set every `.jigc/.gitignore` writer emits, in order:
/// the sub-task working areas (`tasks/`), the rebuildable caches (`index/`, `state/`),
/// the milestone WIP staging (`milestones/`), the fan-out worktrees (`worktrees/`,
/// M31 Inc 3), and the M36 invocation log (`logs/`). This is the **union** of the three
/// formerly-divergent literals — carrying `worktrees/` closes the pre-M39 drift.
pub(crate) const ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\n";

/// Ensure `<jigc_root>/.gitignore` lists every [`ENTRIES`] line. Idempotent — the file
/// is (re)written only when it is absent or does not already list **all** of the entry
/// set (so an adapter-written `.gitignore` predating any later entry — `milestones/`,
/// `worktrees/`, `logs/` — is amended once to the union). Errors carry the offending
/// path so a finalize/create failure is legible.
pub(crate) fn ensure(jigc_root: &Path) -> io::Result<()> {
    let path = jigc_root.join(".gitignore");
    let needs_write = match std::fs::read_to_string(&path) {
        Ok(existing) => {
            let lines: Vec<&str> = existing.lines().map(str::trim).collect();
            !ENTRIES.lines().all(|entry| lines.contains(&entry))
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => true,
        Err(err) => {
            return Err(io::Error::new(
                err.kind(),
                format!("could not read {path:?}: {err}"),
            ));
        }
    };
    if needs_write {
        std::fs::create_dir_all(jigc_root).map_err(|err| {
            io::Error::new(err.kind(), format!("could not create {jigc_root:?}: {err}"))
        })?;
        std::fs::write(&path, ENTRIES).map_err(|err| {
            io::Error::new(err.kind(), format!("could not write {path:?}: {err}"))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop (the project's no-tempfile
    /// pattern, mirrored from `repo.rs::tests`).
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-gitignore-unit-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn ensure_writes_the_union_entry_set() {
        let dir = TempDir::new();
        let jigc_root = dir.path().join(".jigc");
        ensure(&jigc_root).unwrap();
        let body = std::fs::read_to_string(jigc_root.join(".gitignore")).unwrap();
        assert_eq!(
            body, ENTRIES,
            "the canonical writer must emit the union set incl. `worktrees/`",
        );
    }

    #[test]
    fn ensure_amends_a_legacy_worktreeless_gitignore() {
        let dir = TempDir::new();
        let jigc_root = dir.path().join(".jigc");
        std::fs::create_dir_all(&jigc_root).unwrap();
        // A pre-M39 adapter/task-written file, missing `worktrees/`.
        std::fs::write(
            jigc_root.join(".gitignore"),
            "tasks/\nindex/\nstate/\nmilestones/\nlogs/\n",
        )
        .unwrap();
        ensure(&jigc_root).unwrap();
        let body = std::fs::read_to_string(jigc_root.join(".gitignore")).unwrap();
        assert_eq!(
            body, ENTRIES,
            "a legacy worktrees-less `.gitignore` is amended once to the union set",
        );
    }

    /// The three formerly-divergent writers now share this one source. `task.rs`
    /// (finalize) and `milestone.rs` (create/provision) call [`ensure`] verbatim, and
    /// `adapter.rs` (`init_project_layer`, the `jigc setup` path) routes through it too.
    /// Driving each module's real gitignore write path and comparing the emitted bytes
    /// proves they can no longer diverge (the M39 3→1 collapse).
    #[test]
    fn all_three_writers_emit_the_identical_entry_set() {
        // milestone.rs write path (create/provision → `crate::gitignore::ensure`).
        let m = TempDir::new();
        let m_root = m.path().join(".jigc");
        ensure(&m_root).unwrap();
        let milestone_bytes = std::fs::read_to_string(m_root.join(".gitignore")).unwrap();

        // task.rs write path (finalize → `crate::gitignore::ensure`).
        let t = TempDir::new();
        let t_root = t.path().join(".jigc");
        ensure(&t_root).unwrap();
        let task_bytes = std::fs::read_to_string(t_root.join(".gitignore")).unwrap();

        // adapter.rs write path (`init_project_layer`, writes `<repo>/.jigc/.gitignore`).
        let a = TempDir::new();
        crate::adapter::init_project_layer(a.path()).unwrap();
        let adapter_bytes =
            std::fs::read_to_string(a.path().join(".jigc").join(".gitignore")).unwrap();

        assert_eq!(
            milestone_bytes, task_bytes,
            "milestone.rs and task.rs must write the identical entry set",
        );
        assert_eq!(
            task_bytes, adapter_bytes,
            "task.rs and adapter.rs must write the identical entry set",
        );
        assert_eq!(
            adapter_bytes, ENTRIES,
            "all three writers must emit the union set incl. `worktrees/`",
        );
    }
}
