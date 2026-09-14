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
//!
//! **The file is the user's too, so the writer amends rather than replaces** (M51
//! Increment 4; `completions/artifacts/M51/settle-record.md` → §6). This doc-comment
//! said *"amended once to the union"* from the day it was written and the writer
//! **replaced** the whole file with [`ENTRIES`] — so a user's own line in
//! `.jigc/.gitignore` was destroyed at `setup` and at `task finalize` *silently and
//! landed* (the rewrite matches `HEAD`, so `git status` reads clean and the bytes
//! survive in no git object), and at `milestone create`/`provision` visibly but with no
//! transaction to restore them. [`ensure`] now carries the settled byte algorithm:
//! preserve all existing bytes exactly; append only missing canonical entries, in
//! [`ENTRIES`] order; insert exactly one separator newline only when required; never
//! normalize or deduplicate existing content; reject a non-regular, symlinked or
//! undecodable file rather than replace it. The result is a **fixed point** — which is
//! what lets `finalize` commit the file without authoring a diff nobody wrote.

use std::io;
use std::path::Path;

/// The transient-runtime entry set every `.jigc/.gitignore` writer emits, in order:
/// the sub-task working areas (`tasks/`), the rebuildable caches (`index/`, `state/`),
/// the milestone WIP staging (`milestones/`), the fan-out worktrees (`worktrees/`,
/// M31 Inc 3), the M36 invocation log (`logs/`), and the **relocation workbench**
/// (`displaced/`, M39 Inc 5 T5 — the parking home for a foreign file moved out of a
/// relocation destination, kept uncommittable; `crate::relocate::WORKBENCH_SUBDIR`).
/// This is the **union** of the three formerly-divergent literals — carrying `worktrees/`
/// closes the pre-M39 drift.
pub const ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\ndisplaced/\n";

/// What [`ensure`] did to `<jigc_root>/.gitignore` — the amend's report, so a door can
/// say what it changed there instead of writing silently, and so the finalize
/// transaction knows the exact image jigc left behind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ensured {
    /// No file was there; jigc wrote one carrying exactly [`ENTRIES`].
    Created,
    /// A file was there and was short of the canonical set: every existing byte was
    /// preserved and these entries — the missing ones, in [`ENTRIES`] order — were
    /// appended.
    Amended { appended: Vec<&'static str> },
    /// The file already listed every entry. Not a byte was written.
    Unchanged,
}

/// Ensure `<jigc_root>/.gitignore` lists every [`ENTRIES`] line, **by amendment**: the
/// existing bytes are preserved exactly and only the missing entries are appended, so a
/// comment, a blank line, CRLF, a missing final newline, a pre-existing duplicate and a
/// line that is none of jigc's business all survive untouched (the module doc's byte
/// algorithm). Idempotent by construction — a second call finds nothing missing and
/// writes nothing.
///
/// Refuses a `.gitignore` it cannot read back byte-for-byte — a symlink (following it
/// would append to a file outside `.jigc/`), anything that is not a regular file, and
/// undecodable bytes. Every error carries the offending path, so a
/// setup/finalize/create failure is legible; an I/O fault mints no finding code.
pub fn ensure(jigc_root: &Path) -> io::Result<Ensured> {
    let path = jigc_root.join(".gitignore");
    let existing = match std::fs::symlink_metadata(&path) {
        Ok(meta) if meta.file_type().is_symlink() => {
            return Err(refuse(&path, "it is a symlink"));
        }
        Ok(meta) if !meta.is_file() => {
            return Err(refuse(&path, "it is not a regular file"));
        }
        Ok(_) => match std::fs::read_to_string(&path) {
            Ok(body) => Some(body),
            Err(err) if err.kind() == io::ErrorKind::InvalidData => {
                return Err(refuse(&path, "it is not valid UTF-8"));
            }
            Err(err) => {
                return Err(io::Error::new(
                    err.kind(),
                    format!("could not read {path:?}: {err}"),
                ));
            }
        },
        Err(err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(err) => {
            return Err(io::Error::new(
                err.kind(),
                format!("could not read {path:?}: {err}"),
            ));
        }
    };

    let Some(existing) = existing else {
        std::fs::create_dir_all(jigc_root).map_err(|err| {
            io::Error::new(err.kind(), format!("could not create {jigc_root:?}: {err}"))
        })?;
        write_ignore(&path, ENTRIES)?;
        return Ok(Ensured::Created);
    };

    // Presence is judged on the TRIMMED line, so a CRLF file and a line carrying
    // trailing spaces are read as listing the entry — and therefore left alone —
    // rather than gaining a duplicate on every run.
    let listed: Vec<&str> = existing.lines().map(str::trim).collect();
    let appended: Vec<&'static str> = ENTRIES
        .lines()
        .filter(|entry| !listed.contains(entry))
        .collect();
    if appended.is_empty() {
        return Ok(Ensured::Unchanged);
    }

    let mut next = existing;
    // Exactly one separator newline, and only when the existing bytes do not already
    // end in one — an empty file needs none either.
    if !next.is_empty() && !next.ends_with('\n') {
        next.push('\n');
    }
    for entry in &appended {
        next.push_str(entry);
        next.push('\n');
    }
    write_ignore(&path, &next)?;
    Ok(Ensured::Amended { appended })
}

/// The refusal an unamendable `.gitignore` draws — the path, then why jigc will not
/// touch it. Stated as one helper so all three legs say the same thing the same way.
fn refuse(path: &Path, why: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "could not amend {path:?}: {why} — jigc appends its entries to this file \
             in place and will not replace it"
        ),
    )
}

/// Write `body` at `path`, carrying the path into the error.
fn write_ignore(path: &Path, body: &str) -> io::Result<()> {
    std::fs::write(path, body)
        .map_err(|err| io::Error::new(err.kind(), format!("could not write {path:?}: {err}")))
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
                engine::tempname::unique_nanos(),
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

    /// The legacy amend, restated as what an amendment actually does: the pre-M39 lines
    /// keep their bytes and their order, and the entries they lack are appended. It
    /// asserted `body == ENTRIES` until M51 Increment 4 — which was the *replace*
    /// writer's signature, and true only because a replaced file cannot differ.
    #[test]
    fn ensure_amends_a_legacy_worktreeless_gitignore() {
        let dir = TempDir::new();
        let jigc_root = dir.path().join(".jigc");
        std::fs::create_dir_all(&jigc_root).unwrap();
        // A pre-M39 adapter/task-written file, missing `worktrees/`.
        let legacy = "tasks/\nindex/\nstate/\nmilestones/\nlogs/\n";
        std::fs::write(jigc_root.join(".gitignore"), legacy).unwrap();
        let report = ensure(&jigc_root).unwrap();
        assert_eq!(
            report,
            Ensured::Amended {
                appended: vec!["worktrees/", "displaced/"],
            },
            "the report names the two entries the legacy file lacked",
        );
        let body = std::fs::read_to_string(jigc_root.join(".gitignore")).unwrap();
        assert_eq!(
            body, "tasks/\nindex/\nstate/\nmilestones/\nlogs/\nworktrees/\ndisplaced/\n",
            "a legacy worktrees-less `.gitignore` keeps its own bytes and gains the \
             entries it lacks",
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
