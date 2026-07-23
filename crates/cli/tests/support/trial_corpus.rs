//! The **trial-shaped fixture builder** — named corpus states the pinning suites
//! share ([pinning.md](../../../../implementation/pinning.md) §4).
//!
//! Each state is one a real adoption trial hit and a synthetic fixture missed, so
//! the golden and contract suites sweep *corpus reality* rather than a hand-shaped
//! stub. Suites take states **by name**: a state added here auto-joins every suite
//! that iterates [`State::ALL`].
//!
//! **The build rule, as narrowed at cross-review:** managed-doc state and
//! provenance are built **only by driving the binary** — so `edited-from-base`,
//! staged copies, and index state are real, never simulated — while **unmanaged
//! repo furniture** (code files, a foreign hook, gitignored runtime dirs, per-repo
//! git config) is written directly, exactly as the existing flow suites do. jigc
//! has no verb that authors a foreign hook or a vendor dir, so the unqualified
//! "everything through the binary" form would be unsatisfiable.
//!
//! **Isolation is strict and non-optional:** a pid+nanos tempdir per corpus, a
//! per-repo git identity, `$HOME` repointed into the corpus, and `JIGC_PACK_DIR`
//! removed from every child environment — the built state composes the *embedded*
//! packs the suites claim to sweep, whatever the developer's shell carries.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// One named corpus state. Iterate [`State::ALL`] to sweep every state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// `jigc setup` only — the baseline.
    Fresh,
}

impl State {
    /// Every named state, in declaration order. A suite that iterates this picks
    /// up a newly added state with no edit.
    pub const ALL: &'static [State] = &[State::Fresh];

    /// The state's name, as the goldens and suite labels spell it.
    pub fn name(self) -> &'static str {
        match self {
            State::Fresh => "fresh",
        }
    }
}

/// A built corpus state in a throwaway directory that removes itself on drop.
///
/// The directory holds a `repo/` (the git repo under test) and a `home/` (the
/// `$HOME` every child process sees), so one drop cleans both.
pub struct TrialCorpus {
    root: PathBuf,
    state: State,
}

impl TrialCorpus {
    /// Build `state` in a fresh throwaway corpus.
    pub fn build(state: State) -> Self {
        let mut root = std::env::temp_dir();
        root.push(format!(
            "jigc-trial-{}-{}-{}",
            state.name(),
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock after the epoch")
                .as_nanos(),
        ));
        fs::create_dir_all(root.join("repo")).expect("create the corpus repo dir");
        fs::create_dir_all(root.join("home")).expect("create the corpus home dir");

        let corpus = TrialCorpus { root, state };
        corpus.git_init();
        // Managed state through the binary: `setup` installs the `.jigc/` workbench,
        // the adapter files, and its own pre-commit hook, and commits them.
        corpus.jigc_ok(&["setup"]);
        match state {
            State::Fresh => {}
        }
        corpus
    }

    /// The state this corpus was built as.
    pub fn state(&self) -> State {
        self.state
    }

    /// The git repo under test.
    pub fn repo(&self) -> PathBuf {
        self.root.join("repo")
    }

    /// The `$HOME` every child process of this corpus sees.
    pub fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    /// Run `jigc <args>` against this corpus and return its raw [`Output`].
    ///
    /// `cwd` is the repo, `$HOME` the corpus home, and `JIGC_PACK_DIR` is removed
    /// so the child always composes the **embedded** packs — an inherited
    /// `JIGC_PACK_DIR` would silently swap the pack under every sweep.
    pub fn jigc(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo())
            .env("HOME", self.home())
            .env_remove("JIGC_PACK_DIR")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("spawn jigc")
    }

    /// Run `jigc <args>`, assert it succeeded, and return its stdout.
    pub fn jigc_ok(&self, args: &[&str]) -> String {
        let out = self.jigc(args);
        assert!(
            out.status.success(),
            "jigc {args:?} failed ({}):\n--- stdout ---\n{}\n--- stderr ---\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 jigc stdout")
    }

    /// Run `git <args>` in the repo, assert success, return trimmed stdout.
    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(self.repo())
            .env("HOME", self.home())
            .output()
            .expect("spawn git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout)
            .expect("utf-8 git stdout")
            .trim()
            .to_string()
    }

    /// Unmanaged repo furniture: a real git repo with one commit and a per-repo
    /// identity (never the developer's global config).
    fn git_init(&self) {
        self.git(&["init", "-q"]);
        self.git(&["config", "user.email", "trial@example.com"]);
        self.git(&["config", "user.name", "Trial Corpus"]);
        fs::write(self.repo().join("README.md"), "trial corpus\n").expect("write README.md");
        self.git(&["add", "."]);
        self.git(&["commit", "-q", "-m", "initial"]);
    }
}

impl Drop for TrialCorpus {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// Read a repo-relative file from a built corpus.
pub fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}
