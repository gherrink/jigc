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
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// The repo-relative path of the foreign document [`State::Migrated`] migrates —
/// unmanaged furniture, written directly and committed **before** the migration so
/// its retirement is a real deletion rather than an untracked-file removal.
pub const FOREIGN_VISION_PATH: &str = "docs/direction.md";

/// The foreign document's bytes — deliberately non-conformant (an H1 the schema
/// does not name, a free-form `## Principles` section), so `jigc migrate` has real
/// prose to route through the author step.
const FOREIGN_VISION: &str = "\
# Product Direction

We build a deterministic context compiler.

## Principles

Structure belongs to the CLI; prose belongs to the model.
";

/// The line [`State::ChattyHooks`]' foreign `pre-commit` hook echoes to **stdout**
/// on every successful commit. git redirects a hook's stdout onto its own stderr, so
/// this is the byte a suite looks for when it asks what the merged hook stream does
/// to a machine-readable surface.
pub const CHATTY_HOOK_MARKER: &str = "chatty-hook: pre-commit spoke on success";

/// [`State::Vendored`]'s gitignored runtime file — present on disk, invisible to
/// `git ls-files --cached --others --exclude-standard` (the ingest funnel's
/// candidate source).
pub const VENDORED_RUNTIME_FILE: &str = "node_modules/left-pad/index.js";

/// [`State::Vendored`]'s tracked code file — the same walk's *visible* half, so the
/// invisibility of the runtime dir is a discrimination rather than an empty walk.
pub const VENDORED_CODE_FILE: &str = "src/pad.ts";

/// One named corpus state. Iterate [`State::ALL`] to sweep every state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// `jigc setup` only — the baseline.
    Fresh,
    /// The three methodology singletons — `vision`, `roadmap`, `decisions-log` —
    /// each created through its own driving workflow and **finalized**, so all
    /// three are committed at their resolved homes (`VISION.md`,
    /// `docs/roadmap.md`, `docs/decisions-log.md`). Closes the
    /// create-over-committed / copy-in blind spot.
    CommittedSingletons,
    /// A foreign document landed through `jigc migrate … --approve`: the managed
    /// doc is committed and the **foreign source is retired** in the same commit.
    Migrated,
    /// Edges set by `jigc doc set-field` on **committed** docs inside a task that
    /// is still **live** — so the corpus carries `edited-from-base` working-area
    /// provenance and a staged copy diverging from the committed bytes. That
    /// property is erased at finalize, so no finalized state can carry it.
    RefsPostHoc,
    /// A foreign `pre-commit` hook that prints [`CHATTY_HOOK_MARKER`] to stdout and
    /// exits `0` — the chatty-but-non-blocking hook a real repo carries (a linter, a
    /// formatter, a CI shim). Closes the JSON-purity blind spot: git folds a hook's
    /// stdout into its own stderr, so a machine-readable surface meets that stream
    /// whether it wants to or not.
    ///
    /// **The hook deliberately *replaces* `jigc setup`'s own** rather than wrapping
    /// it (`implementation/pinning.md` §4 — replace vs append is a stated call). The
    /// consequence, stated rather than left to be discovered: **every suite running
    /// this state commits without jigc's warn-only doc↔code backstop and without its
    /// out-of-band-rename block** — so the only hook output such a suite can observe
    /// is this fixture's own marker, which is what makes the observation
    /// unambiguous, and the absence of setup's sentinel is what makes "replaced"
    /// mechanically true rather than a comment.
    ChattyHooks,
    /// A gitignored vendored runtime directory ([`VENDORED_RUNTIME_FILE`]) beside
    /// tracked source ([`VENDORED_CODE_FILE`]). Closes the worktree-provisioning /
    /// ingest-funnel blind spot: the candidate walk is
    /// `git ls-files --cached --others --exclude-standard`, so a vendored tree must
    /// be *present on disk yet absent from that walk*.
    Vendored,
}

impl State {
    /// Every named state, in declaration order. A suite that iterates this picks
    /// up a newly added state with no edit.
    pub const ALL: &'static [State] = &[
        State::Fresh,
        State::CommittedSingletons,
        State::Migrated,
        State::RefsPostHoc,
        State::ChattyHooks,
        State::Vendored,
    ];

    /// The state's name, as the goldens and suite labels spell it.
    pub fn name(self) -> &'static str {
        match self {
            State::Fresh => "fresh",
            State::CommittedSingletons => "committed-singletons",
            State::Migrated => "migrated",
            State::RefsPostHoc => "refs-post-hoc",
            State::ChattyHooks => "chatty-hooks",
            State::Vendored => "vendored",
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
    live_task: Option<String>,
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

        let mut corpus = TrialCorpus {
            root,
            state,
            live_task: None,
        };
        corpus.git_init();
        // Managed state through the binary: `setup` installs the `.jigc/` workbench,
        // the adapter files, and its own pre-commit hook, and commits them.
        corpus.jigc_ok(&["setup"]);
        match state {
            State::Fresh => {}
            State::CommittedSingletons => corpus.build_committed_singletons(),
            State::Migrated => corpus.build_migrated(),
            State::RefsPostHoc => {
                let live = corpus.build_refs_post_hoc();
                corpus.live_task = Some(live);
            }
            State::ChattyHooks => corpus.build_chatty_hooks(),
            State::Vendored => corpus.build_vendored(),
        }
        corpus
    }

    /// The id of this state's **live** (unfinalized) task, when it has one — the
    /// handle a suite needs to reach the task's working area under
    /// `.jigc/tasks/<id>/`. Only [`State::RefsPostHoc`] leaves a task live.
    pub fn live_task(&self) -> Option<&str> {
        self.live_task.as_deref()
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

    /// Run `jigc <args>` with `stdin` piped, assert it succeeded, return stdout.
    /// The slot/payload writers all read `--from-file -`.
    pub fn jigc_stdin_ok(&self, args: &[&str], stdin: &str) -> String {
        let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo())
            .env("HOME", self.home())
            .env_remove("JIGC_PACK_DIR")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn jigc");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(stdin.as_bytes())
            .expect("write jigc stdin");
        let out = child.wait_with_output().expect("wait for jigc");
        assert!(
            out.status.success(),
            "jigc {args:?} failed ({}):\n--- stdout ---\n{}\n--- stderr ---\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 jigc stdout")
    }

    /// Compose a workflow and return the id the binary **printed** it minted — read
    /// from the real output, never reconstructed from the intent (the slug rule is
    /// the binary's, and a test-side copy of it would drift silently).
    pub fn start_workflow(&self, workflow: &str, intent: &str) -> String {
        minted_task(&self.jigc_ok(&["start", "--workflow", workflow, intent]))
    }

    /// Set one prose slot from stdin.
    fn set_slot(&self, address: &str, task: &str, prose: &str) {
        self.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                address,
                "--from-file",
                "-",
                "--task",
                task,
            ],
            prose,
        );
    }

    /// Author the task's `commit` doc and finalize it. Every fixture commit is a
    /// `docs` change, so only the scope and summary vary. `approve` passes
    /// `--approve`, which a migration's destructive retire-and-land needs.
    ///
    /// Returns the finalize invocation's **stdout**, so a caller can assert on what
    /// the landed commit printed (the hook relay rides that stream on agent-text).
    pub fn finalize(&self, task: &str, scope: &str, summary: &str, approve: bool) -> String {
        self.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "docs",
            "--task",
            task,
        ]);
        self.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            scope,
            "--task",
            task,
        ]);
        self.set_slot(&format!("commit:{task}#summary"), task, summary);
        self.set_slot(
            &format!("commit:{task}#body"),
            task,
            "Built by the trial-corpus fixture builder.",
        );
        let mut args = vec!["task", "finalize", task];
        if approve {
            args.push("--approve");
        }
        self.jigc_ok(&args)
    }

    /// [`State::CommittedSingletons`]: the three methodology singletons, each
    /// created through **its own driving workflow** (`planning` gates the roadmap
    /// and the decisions log; `form-vision` gates the vision) and finalized, so all
    /// three land committed at their `placement` homes.
    fn build_committed_singletons(&self) {
        let plan = self.start_workflow("planning", "plan the first wave");
        self.jigc_ok(&[
            "doc", "create", "roadmap", "--title", "Roadmap", "--task", &plan,
        ]);
        self.jigc_ok(&[
            "doc",
            "create",
            "decisions-log",
            "--title",
            "Decisions-Log",
            "--task",
            &plan,
        ]);
        self.finalize(&plan, "planning", "mint the running docs", false);

        let vision = self.start_workflow("form-vision", "form the project vision");
        self.jigc_ok(&[
            "doc", "create", "vision", "--title", "Vision", "--task", &vision,
        ]);
        self.set_slot(
            "vision:vision#thesis",
            &vision,
            "A deterministic CLI assembles exactly the context a task needs.",
        );
        self.set_slot(
            "vision:vision#invariants",
            &vision,
            "Structure belongs to the CLI; prose belongs to the model.",
        );
        self.set_slot(
            "vision:vision#open-questions",
            &vision,
            "Which domains earn a pack of their own.",
        );
        self.finalize(&vision, "vision", "form the project vision", false);
    }

    /// [`State::Migrated`]: a foreign document committed as ordinary repo furniture,
    /// then landed through `jigc migrate … --as vision` + `finalize --approve` —
    /// the managed doc committed and the foreign source retired in one commit.
    fn build_migrated(&self) {
        let foreign = self.repo().join(FOREIGN_VISION_PATH);
        fs::create_dir_all(foreign.parent().expect("the foreign source has a parent"))
            .expect("create the foreign source dir");
        fs::write(&foreign, FOREIGN_VISION).expect("write the foreign source");
        self.git(&["add", FOREIGN_VISION_PATH]);
        self.git(&["commit", "-q", "-m", "add the direction doc"]);

        let task = minted_task(&self.jigc_ok(&["migrate", FOREIGN_VISION_PATH, "--as", "vision"]));
        self.jigc_stdin_ok(
            &[
                "doc",
                "author",
                "vision",
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            MIGRATED_VISION_PAYLOAD,
        );
        self.finalize(
            &task,
            "vision",
            "migrate the direction doc into the managed vision",
            true,
        );
    }

    /// [`State::RefsPostHoc`]: the committed singletons **plus** a committed
    /// `research` doc, then a task left **live** whose only write is the
    /// `vision —grounded-in→ research` edge set on the already-committed vision.
    /// Extending `committed-singletons` rather than minting a third managed corpus
    /// is deliberate: the edge needs committed docs on both ends, and a state is
    /// paid for on every sweep.
    ///
    /// Returns the live task's id.
    fn build_refs_post_hoc(&self) -> String {
        self.build_committed_singletons();

        let research = self.start_workflow("do-research", "how agents lose context");
        self.jigc_ok(&[
            "doc",
            "create",
            "research",
            "--title",
            "Context Loss",
            "--task",
            &research,
        ]);
        self.set_slot(
            "research:context-loss#question",
            &research,
            "How does a coding agent lose the context it was given?",
        );
        self.set_slot(
            "research:context-loss#findings",
            &research,
            "Static rules files go stale and are read once, not just in time.",
        );
        self.set_slot(
            "research:context-loss#sources",
            &research,
            "The adoption trial records, 2026.",
        );
        self.finalize(
            &research,
            "research",
            "record the context-loss research",
            false,
        );

        // The edge, set on the COMMITTED vision from inside a task that stays live:
        // the write copies the committed body in (`edited-from-base`) and the staged
        // copy diverges until a finalize that never comes.
        let task = self.start_workflow("form-vision", "ground the vision in research");
        self.jigc_ok(&[
            "doc",
            "set-field",
            "vision:vision#meta/grounded-in",
            "--value",
            "[research:context-loss]",
            "--task",
            &task,
        ]);
        task
    }

    /// [`State::ChattyHooks`]: overwrite `jigc setup`'s `pre-commit` hook with a
    /// foreign one that prints [`CHATTY_HOOK_MARKER`] to stdout and exits `0`.
    ///
    /// Unmanaged repo furniture, so it is written directly — jigc has no verb that
    /// authors a foreign hook. **Overwrite, not append:** setup's installer wraps a
    /// pre-existing foreign hook (bracketing its own block by
    /// `setup::PRECOMMIT_SENTINEL`), so appending would leave *both* streams in play
    /// and a suite observing hook output could not tell whose bytes it read. The
    /// price is stated on [`State::ChattyHooks`]: this state commits without jigc's
    /// own backstop.
    fn build_chatty_hooks(&self) {
        use std::os::unix::fs::PermissionsExt;

        let hook = self.repo().join(".git/hooks/pre-commit");
        fs::write(
            &hook,
            format!(
                "#!/bin/sh\n\
                 # A foreign, chatty, NON-blocking pre-commit hook — the trial corpus's\n\
                 # `chatty-hooks` state. It replaces jigc's own hook outright.\n\
                 echo '{CHATTY_HOOK_MARKER}'\n\
                 exit 0\n"
            ),
        )
        .expect("write the chatty pre-commit hook");
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755))
            .expect("make the chatty hook executable");
    }

    /// [`State::Vendored`]: a gitignored vendored runtime tree beside tracked source.
    ///
    /// All unmanaged furniture, written directly. The `.gitignore` line is
    /// **appended** (never overwritten) so whatever `setup` left in place survives,
    /// and the tracked code file is committed — an untracked file would also appear
    /// in the ingest walk, which would make the contrast prove nothing.
    fn build_vendored(&self) {
        let repo = self.repo();
        let gitignore = repo.join(".gitignore");
        let mut ignored = fs::read_to_string(&gitignore).unwrap_or_default();
        if !ignored.is_empty() && !ignored.ends_with('\n') {
            ignored.push('\n');
        }
        ignored.push_str("node_modules/\n");
        fs::write(&gitignore, ignored).expect("append the vendored ignore rule");

        for (rel, body) in [
            (
                VENDORED_RUNTIME_FILE,
                "module.exports = function pad() { return ''; };\n",
            ),
            (
                VENDORED_CODE_FILE,
                "export function pad(s: string): string {\n  return s;\n}\n",
            ),
        ] {
            let path = repo.join(rel);
            fs::create_dir_all(path.parent().expect("a vendored file has a parent"))
                .expect("create the vendored file's dir");
            fs::write(&path, body).unwrap_or_else(|e| panic!("write {rel}: {e}"));
        }

        self.git(&["add", ".gitignore", VENDORED_CODE_FILE]);
        self.git(&["commit", "-q", "-m", "vendor the runtime, track the source"]);
    }

    /// Run `git <args>` in the repo, assert success, return trimmed stdout.
    pub fn git(&self, args: &[&str]) -> String {
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

/// The `doc author` batch payload [`State::Migrated`] rewrites the foreign source
/// into — the shape the `migrate-vision` step's `{{schema:vision}}` skeleton
/// solicits, with the literal `<<…>>` slot markers it requires.
const MIGRATED_VISION_PAYLOAD: &str = "\
title: Vision
sections:
  - id: thesis
    set:
      thesis: |-
        <<We build a deterministic context compiler.>>
  - id: invariants
    set:
      invariants: |-
        <<Structure belongs to the CLI; prose belongs to the model.>>
  - id: open-questions
    set:
      open-questions: |-
        <<Which domains earn a pack of their own.>>
";

/// The task id a mint printed, read off the binary's `task minted: <id>` line.
fn minted_task(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// Read a repo-relative file from a built corpus.
pub fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}
