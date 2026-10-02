//! **A scratch directory that removes itself** — for a suite that needs a throwaway root
//! and no corpus in it.
//!
//! [`unique_root`](super::trial_corpus::unique_root) names a fresh path and removes
//! nothing; a [`TrialCorpus`](super::trial_corpus::TrialCorpus) removes its own root on
//! drop. Suites that minted a bare `unique_root` and kept it — `dev_rig_parity`'s rig
//! driver, `dev_gate_report`'s fixture logs and the gate runs it drives — left it behind,
//! and a full gate run left ~240 entries in `$TMPDIR` that way, `jigc-trial-rig-fence-*`
//! the most of them (`completions/artifacts/M55/gate-speed-measurement.md`). This is the
//! guard they were missing: in-process `remove_dir_all` on drop, never a shell `rm`.

use std::path::{Path, PathBuf};

/// A fresh directory under the OS temp dir, removed (best effort) when dropped.
pub struct ScratchDir(PathBuf);

impl ScratchDir {
    /// Create a fresh, unique scratch directory named for `label`.
    ///
    /// A `/` in `label` is flattened to `-`: kept, it would nest the directory under a
    /// parent named for the label's head, which the drop never removes.
    pub fn new(label: &str) -> Self {
        let root = super::trial_corpus::unique_root(&label.replace('/', "-"));
        std::fs::create_dir_all(&root)
            .unwrap_or_else(|e| panic!("create the scratch dir {}: {e}", root.display()));
        ScratchDir(root)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
