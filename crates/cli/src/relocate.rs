//! The standalone **move primitive** — `git mv` a managed doc old→new and re-key its
//! file-state entry — extracted from [`rename::apply_and_commit`](crate::rename) (M39
//! Increment 5, T1).
//!
//! This is the single-concern foundation the M39 relocation floor recomposes: a doc-level
//! *move* is exactly `git mv old→new` (staging the rename) plus, in the gitignored
//! file-state record, forgetting the old path key and recording the new path at its hash.
//! It owns **nothing** else — no referrer repoint, no `# H1` rewrite, no introduced-dangling
//! integrity gate. `rename::apply_and_commit` recomposes it with those concerns layered
//! around it (identical behavior), and the M39 `config set docs-root` / freeze-exempt
//! relocation paths consume it as their whole move (they carry no referrers to repoint).
//!
//! The primitive does **not** author the moved doc's new bytes: a pure relocation preserves
//! them (`git mv` keeps the file byte-identical) and a rename rewrites the `# H1` *after* the
//! move at the caller's layer. The caller therefore supplies the moved doc's final hash
//! (`hash_bytes` of whatever bytes will land at `new_rel`) so the record stays in-sync.

use anyhow::{Context, Result};
use std::path::Path;

use engine::file_state::FileStateRecord;

use crate::task::git_run;

/// Move a committed managed doc `old_rel` → `new_rel` and re-key its file-state entry.
///
/// Runs `git mv old_rel new_rel` (skipped when `old_rel == new_rel` — a retitle-in-place
/// has no move) and then, in the file-state record under `jigc_root`, forgets `old_rel`
/// and records `new_rel` at `new_hash`. Loads and saves the record itself, so a caller with
/// no further file-state work (the relocation paths) needs nothing more; `apply_and_commit`
/// re-loads it to layer its referrer re-keys on top (the two saves compose byte-identically).
pub(crate) fn move_doc(
    repo_root: &Path,
    jigc_root: &Path,
    old_rel: &str,
    new_rel: &str,
    new_hash: &str,
) -> Result<()> {
    if old_rel != new_rel {
        git_run(repo_root, &["mv", old_rel, new_rel])?;
    }
    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
    record.forget(old_rel);
    record.record(new_rel.to_string(), new_hash.to_string());
    record
        .save(jigc_root)
        .with_context(|| format!("could not save the file-state record under {jigc_root:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::file_state::hash_bytes;
    use std::path::PathBuf;

    /// A throwaway git repo that removes itself on drop (the project's no-tempfile pattern).
    struct TempRepo(PathBuf);

    impl TempRepo {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-relocate-unit-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp repo");
            let repo = TempRepo(path);
            repo.git(&["init", "-q"]);
            repo.git(&["config", "user.email", "t@t"]);
            repo.git(&["config", "user.name", "t"]);
            repo
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn git(&self, args: &[&str]) {
            let out = std::process::Command::new("git")
                .arg("-C")
                .arg(&self.0)
                .args(args)
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .env("GIT_CONFIG_SYSTEM", "/dev/null")
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr),
            );
        }

        fn commit_file(&self, rel: &str, body: &str) {
            let abs = self.0.join(rel);
            if let Some(parent) = abs.parent() {
                std::fs::create_dir_all(parent).expect("create parent dir");
            }
            std::fs::write(&abs, body).expect("write file");
            self.git(&["add", "--", rel]);
            self.git(&["commit", "-q", "-m", "add"]);
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The primitive drives a real `git mv` and re-keys file-state: after moving a committed
    /// doc, the file lives at the new path (gone from the old), the record has **forgotten**
    /// the old key, and the new key carries the supplied hash — the whole contract the M39
    /// relocation floor recomposes.
    #[test]
    fn move_doc_moves_the_file_and_rekeys_file_state() {
        let repo = TempRepo::new();
        let body = "# A decision\n\nProse.\n";
        repo.commit_file("decisions/old.md", body);

        let jigc_root = repo.path().join(".jigc");
        let old_hash = hash_bytes(body.as_bytes());
        let mut seed = FileStateRecord::new();
        seed.record("decisions/old.md".to_string(), old_hash.clone());
        seed.save(&jigc_root).expect("seed the file-state record");

        move_doc(
            repo.path(),
            &jigc_root,
            "decisions/old.md",
            "decisions/new.md",
            &old_hash,
        )
        .expect("the move primitive succeeds");

        assert!(
            !repo.path().join("decisions/old.md").exists(),
            "the old file is gone after the move",
        );
        assert!(
            repo.path().join("decisions/new.md").exists(),
            "the moved file lands at the new path",
        );

        let record = FileStateRecord::load(&jigc_root).expect("reload the record");
        assert_eq!(
            record.get("decisions/old.md"),
            None,
            "the old file-state key is forgotten",
        );
        assert_eq!(
            record.get("decisions/new.md"),
            Some(old_hash.as_str()),
            "the new file-state key is recorded at the supplied hash",
        );
    }
}
