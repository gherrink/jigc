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
//! **Isolation is strict and non-optional:** a process-unique tempdir per corpus, a
//! per-repo git identity, `$HOME` repointed into the corpus, and `JIGC_PACK_DIR`
//! removed from every child environment — the built state composes the *embedded*
//! packs the suites claim to sweep, whatever the developer's shell carries.
//!
//! **The one declared exception is a [`FixturePack`]** ([`TrialCorpus::build_with_pack`]):
//! a corpus built over one points every child at *that* throwaway directory. The
//! variable is still never inherited — it is set, from a pack this process built — and
//! the reason it exists is that a shape space is not enumerable from the shipped
//! registry (see [`FixturePack`]).
//!
//! **A built state is copied, not rebuilt, for a mutating arm** — the golden sweep's
//! `start` arms mint task dirs, so each state is built once and
//! [`TrialCorpus::copy_state`]'d per arm, and the copy's bytes carry the source's
//! real provenance. Copying is **refused** for a worktree-bearing corpus (§4): those
//! hold absolute paths back into the source, so a copy would read and write the
//! original.
//!
//! **Shape-class coverage extends existing states; it mints none** (pinning.md §4 —
//! *shape-class coverage is the rule; the doctype list is only today's instance*).
//! The six charter states covered no dev-pack doctype and left five of the six
//! shape classes unpopulated, so each missing class was placed in the state it
//! already belonged to rather than in a state of its own — every named state is
//! paid for on **every** sweep (§1 measures ~38 s of serial subprocess time across
//! six states), so a seventh state is the expensive answer:
//!
//!   * [`State::CommittedSingletons`] gains a **populated roadmap milestone** (the
//!     only *multi-slot* repeatable either pack ships) and the **`changelog`
//!     singleton** — simultaneously the only *nested* repeatable, the only
//!     *`id-from` enum*, and the set's first dev-pack member. Both are singletons at
//!     `placement` homes authored through their own driving workflows, which is
//!     exactly what this state already is; the roadmap milestone rides the
//!     **existing** planning task rather than adding one.
//!   * [`State::Vendored`] gains a **`spec` + `arch-doc` pair whose code anchors
//!     resolve**. This is the decisive fit, not a convenience: a *resolving*
//!     code-anchor needs a **real tracked symbol**, and `vendored` is the only state
//!     that carries tracked code. Documenting that code is what the two dev-pack
//!     doctypes are *for*, so the state reads as "a repo with vendored runtime,
//!     tracked source, and the managed docs describing it" rather than as two
//!     unrelated halves.
//!
//! The coverage assertion itself lives in the consuming suite and is derived from
//! the engine-loaded schema model on one side and read back through `doc list` /
//! `doc show` on the other, so **neither** half is a list that can rot: a doctype
//! that starts expressing an unpopulated class reddens the assertion.

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
pub const FOREIGN_VISION: &str = "\
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
/// Also the symbol home the state's `arch-doc` component anchors at
/// ([`VENDORED_CODE_SYMBOL`]).
pub const VENDORED_CODE_FILE: &str = "src/pad.ts";

/// The symbol [`VENDORED_CODE_FILE`] declares — the target of the `arch-doc`
/// component's `implemented-by` code-anchor, resolved by the `doc-code` probe's
/// TypeScript grammar. Also the component's item title, so the `title-names-symbol`
/// guard is satisfied by construction.
pub const VENDORED_CODE_SYMBOL: &str = "pad";

/// [`State::Vendored`]'s tracked **test** file — the target of the `spec`
/// criterion's `maps-to-test` code-anchor. Rust, because `criterion-maps-to-test`
/// adjudicates the canonical Rust `#[test]` attribute and is Rust-only by design.
pub const VENDORED_TEST_FILE: &str = "tests/pad_test.rs";

/// The `#[test]`-attributed function [`VENDORED_TEST_FILE`] declares.
pub const VENDORED_TEST_SYMBOL: &str = "pads_the_input";

/// One named corpus state. Iterate [`State::ALL`] to sweep every state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// `jigc setup` only — the baseline.
    Fresh,
    /// The committed singleton set — `vision`, `roadmap`, `decisions-log` and the
    /// dev pack's `changelog` — each created through its own driving workflow and
    /// **finalized**, so all four are committed at their resolved homes
    /// (`VISION.md`, `docs/roadmap.md`, `docs/decisions-log.md`, `CHANGELOG.md`).
    /// Closes the create-over-committed / copy-in blind spot.
    ///
    /// Two of them are **populated**, not merely minted (pinning.md §4 —
    /// shape-class coverage): the roadmap carries a milestone with **both** its
    /// prose leaves set (the only *multi-slot repeatable* either pack ships), and
    /// the changelog carries a staged change-group **and** a cut release with a
    /// nested one (the only *nested repeatable*, the only *`id-from` enum*).
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
    ///
    /// It is also the **code-anchor** state (pinning.md §4), because a *resolving*
    /// anchor needs a real tracked symbol and this is the only state that has one:
    /// a committed `spec` whose criterion `maps-to-test` names the `#[test]` fn in
    /// [`VENDORED_TEST_FILE`], and a committed `arch-doc` whose component
    /// `implemented-by` names [`VENDORED_CODE_SYMBOL`] in [`VENDORED_CODE_FILE`].
    /// Both landed through `task finalize`, where `doc-code.symbol-exists` /
    /// `doc-code.criterion-maps-to-test` are **blocking** by default — so the state
    /// building at all is the resolution proof, not a claim about it.
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

/// A **manufactured fixture pack**: a throwaway copy of the embedded dev pack whose
/// schemas a suite may reshape freely.
///
/// It exists because a *shape space* is not enumerable from the shipped registry. The
/// item-region axis is `{single-slot, multi-slot, slotless} × {nested, not}`, and across
/// both shipped packs `roadmap.milestones` is the only multi-slot item block and
/// `changelog.releases/changes` the only nested one — **two of six cells**. A suite that
/// iterated the registry would sweep those two and call the axis covered, so the
/// remaining shapes are manufactured here rather than declared unreachable
/// (`completions/artifacts/M49/settle-record.md` → *Acceptance — one correction to how
/// the axis is built*).
///
/// **Why the freeze manifest is dropped.** A copied `config/schema-manifest.yaml`
/// freeze-asserts each dev-pack schema's hash at pack-load, so a reshaped schema would
/// block loudly (correctly — that is the frozen-v1 gate). A manifest-less constituent is
/// freeze-exempt (`pack.rs` scopes the assert to constituents that ship one), so the
/// fixture pack is a *different, unfrozen* pack rather than a forged v1 dev pack.
///
/// This is the declared exception to the module's `JIGC_PACK_DIR`-is-always-removed rule:
/// a corpus built with [`TrialCorpus::build_with_pack`] points every child at **this**
/// directory, explicitly, instead of inheriting whatever a developer's shell carries.
pub struct FixturePack {
    root: PathBuf,
}

impl FixturePack {
    /// Copy the embedded dev pack tree into a throwaway directory and drop its freeze
    /// manifest, leaving a loadable pack a caller may reshape.
    pub fn from_dev_pack(label: &str) -> Self {
        let root = unique_root(&format!("pack-{label}"));
        let dev_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
        copy_tree(&dev_pack, &root);
        fs::remove_file(root.join("config").join("schema-manifest.yaml"))
            .expect("drop the copied freeze manifest");
        FixturePack { root }
    }

    /// Write (or replace) one doctype schema, by its `<doctype>.yaml` file name.
    pub fn write_schema(&self, doctype: &str, yaml: &str) -> &Self {
        fs::write(
            self.root.join("schemas").join(format!("{doctype}.yaml")),
            yaml,
        )
        .unwrap_or_else(|e| panic!("write the {doctype} fixture schema: {e}"));
        self
    }

    /// Write (or replace) one workflow definition, by its `<id>.yaml` file name.
    pub fn write_workflow(&self, id: &str, yaml: &str) -> &Self {
        fs::write(self.root.join("workflows").join(format!("{id}.yaml")), yaml)
            .unwrap_or_else(|e| panic!("write the {id} fixture workflow: {e}"));
        self
    }

    /// The directory every child of a fixture-pack corpus reads as `JIGC_PACK_DIR`.
    pub fn path(&self) -> &Path {
        &self.root
    }
}

impl Drop for FixturePack {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// A built corpus state in a throwaway directory that removes itself on drop.
///
/// The directory holds a `repo/` (the git repo under test) and a `home/` (the
/// `$HOME` every child process sees), so one drop cleans both.
pub struct TrialCorpus {
    root: PathBuf,
    /// The [`State`] this corpus was built as, or `None` for the **never-adopted**
    /// shape ([`TrialCorpus::build_never_adopted`]), which is deliberately not a
    /// `State` member.
    state: Option<State>,
    live_task: Option<String>,
    /// The [`FixturePack`] every child composes, when this corpus was built with one
    /// ([`Self::build_with_pack`]); `None` is the embedded-pack default.
    pack: Option<PathBuf>,
}

impl TrialCorpus {
    /// Build `state` in a fresh throwaway corpus, over the **embedded** packs.
    pub fn build(state: State) -> Self {
        Self::build_over(state, None)
    }

    /// Build `state` in a fresh throwaway corpus whose every child composes `pack`
    /// instead of the embedded dev pack — the manufactured-shape entry point.
    ///
    /// The pack outlives the corpus by construction: the caller holds the
    /// [`FixturePack`], which cleans itself up on drop.
    pub fn build_with_pack(state: State, pack: &FixturePack) -> Self {
        Self::build_over(state, Some(pack.path().to_path_buf()))
    }

    /// A **never-adopted** corpus — `git init` plus the one initial commit, and **no
    /// `jigc setup`**: the shape every real brownfield repository is in the moment before
    /// it meets jigc, and the only shape from which a *pre-jigc* commit history at a path
    /// jigc will later declare as a home can exist at all.
    ///
    /// Deliberately **not** a [`State`] member, for the reason `dev_rig_parity`'s
    /// `RIG_ONLY_STATES` already records for the rig's own `bare`: every `State::ALL`
    /// consumer presupposes [`Self::build_over`]'s `jigc setup`, and `compose_goldens`
    /// sweeps each composed surface × every state — a corpus with no `.jigc/` has no
    /// composed surface to golden. It is a named **fixture act** instead, so a suite that
    /// needs the pre-adoption shape can say so rather than hand-rolling a git repo and
    /// losing the corpus's `$HOME`, pack selection and self-cleaning root with it.
    ///
    /// The caller adopts it when it wants to — `corpus.jigc_ok(&["setup"])` — which is the
    /// point: what happens *between* `git init` and `jigc setup` is what this shape exists
    /// to let a suite write.
    pub fn build_never_adopted() -> Self {
        let root = unique_root("never-adopted");
        fs::create_dir_all(root.join("repo")).expect("create the corpus repo dir");
        fs::create_dir_all(root.join("home")).expect("create the corpus home dir");

        let corpus = TrialCorpus {
            root,
            state: None,
            live_task: None,
            pack: None,
        };
        corpus.git_init();
        corpus
    }

    fn build_over(state: State, pack: Option<PathBuf>) -> Self {
        let root = unique_root(state.name());
        fs::create_dir_all(root.join("repo")).expect("create the corpus repo dir");
        fs::create_dir_all(root.join("home")).expect("create the corpus home dir");

        let mut corpus = TrialCorpus {
            root,
            state: Some(state),
            live_task: None,
            pack,
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

    /// A byte-for-byte copy of this built state in a **fresh** throwaway corpus —
    /// the primitive a *mutating* sweep arm needs: a state is built once and copied
    /// per arm, so the copy carries real provenance rather than a rebuild's.
    ///
    /// **Copying is refused for a worktree-bearing corpus**
    /// ([pinning.md](../../../../implementation/pinning.md) §4). A copied
    /// `.git/worktrees/*/gitdir`, and each linked worktree's `.git` **file**, hold
    /// **absolute** paths back into the *source* fixture — so the copy would silently
    /// read and write the original: cross-test contamination, not a clean failure.
    /// A worktree-bearing state is built fresh per arm, or re-provisioned after the
    /// copy; it is never copied.
    pub fn copy_state(&self) -> TrialCorpus {
        assert!(
            !self.repo().join(".git/worktrees").exists(),
            "refusing to copy the `{}` corpus: it has provisioned git worktrees \
             (`.git/worktrees/`), whose `gitdir` entries hold ABSOLUTE paths back \
             into the source — the copy would read and write the ORIGINAL. Build a \
             worktree-bearing state fresh per arm, or re-provision after the copy \
             (pinning.md §4).",
            self.label(),
        );

        let root = unique_root(&format!("{}-copy", self.label()));
        copy_tree(&self.root, &root);
        TrialCorpus {
            root,
            state: self.state,
            live_task: self.live_task.clone(),
            pack: self.pack.clone(),
        }
    }

    /// The id of this state's **live** (unfinalized) task, when it has one — the
    /// handle a suite needs to reach the task's working area under
    /// `.jigc/tasks/<id>/`. Only [`State::RefsPostHoc`] leaves a task live.
    pub fn live_task(&self) -> Option<&str> {
        self.live_task.as_deref()
    }

    /// The state this corpus was built as.
    ///
    /// Panics on the **never-adopted** shape ([`Self::build_never_adopted`]), which is
    /// not a [`State`] member and must not be able to answer as one: a sweep that reads
    /// this to label or dispatch on a state would otherwise be handed a state the corpus
    /// is not in.
    pub fn state(&self) -> State {
        self.state.expect(
            "this corpus was built by `build_never_adopted` and is in no `State` — it has \
             no `jigc setup`, so nothing that dispatches on a state applies to it",
        )
    }

    /// The corpus's name for throwaway-root and copy labelling — the state's name, or
    /// `never-adopted` for the shape that is in no [`State`].
    fn label(&self) -> &'static str {
        match self.state {
            Some(state) => state.name(),
            None => "never-adopted",
        }
    }

    /// The git repo under test.
    pub fn repo(&self) -> PathBuf {
        self.root.join("repo")
    }

    /// The `$HOME` every child process of this corpus sees.
    pub fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    /// Empty the gitignored file-state cache — **the fresh-clone shape**.
    ///
    /// `.jigc/state/` is gitignored, so what a `git clone` hands a teammate or a CI runner is
    /// this corpus *without* it. A store-scope condition that is only visible through that
    /// cache is therefore invisible to every clone, which is what makes the shape worth a
    /// named fixture act rather than an inline `remove_dir_all` per suite: the suites that
    /// reach for it are asserting a property **about** the clone, not tidying up.
    ///
    /// The directory itself is left in place; only its contents go.
    pub fn fresh_clone_shape(&self) {
        let state = self.repo().join(".jigc").join("state");
        if !state.is_dir() {
            return;
        }
        for entry in fs::read_dir(&state).expect("read the file-state dir") {
            let path = entry.expect("a file-state dir entry").path();
            if path.is_dir() {
                fs::remove_dir_all(&path).expect("clear a file-state subdir");
            } else {
                fs::remove_file(&path).expect("clear a file-state file");
            }
        }
    }

    /// Run `jigc <args>` against this corpus and return its raw [`Output`].
    ///
    /// `cwd` is the repo, `$HOME` the corpus home, and `JIGC_PACK_DIR` is either
    /// **removed** — so the child composes the **embedded** packs, an inherited
    /// `JIGC_PACK_DIR` silently swapping the pack under every sweep — or set to this
    /// corpus's own [`FixturePack`] when it was built with one. Either way the pack the
    /// child reads is chosen here, never inherited.
    pub fn jigc(&self, args: &[&str]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(self.repo())
            .env("HOME", self.home())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        self.select_pack(&mut command);
        command.output().expect("spawn jigc")
    }

    /// Point one child at this corpus's pack: the fixture pack when built with one,
    /// else no `JIGC_PACK_DIR` at all. The single seam both spawn helpers share, so a
    /// fixture-pack corpus cannot compose the embedded pack through one of them.
    fn select_pack(&self, command: &mut Command) {
        match &self.pack {
            Some(dir) => command.env("JIGC_PACK_DIR", dir),
            None => command.env_remove("JIGC_PACK_DIR"),
        };
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

    /// Run `jigc <args>` with `stdin` piped and return its raw [`Output`] — the
    /// shape a suite needs when the invocation is *expected* to be refused (a
    /// gated slot write), where [`Self::jigc_stdin_ok`]'s assert would fire first.
    pub fn jigc_stdin(&self, args: &[&str], stdin: &str) -> Output {
        self.jigc_stdin_from(&self.repo(), args, stdin)
    }

    /// [`Self::jigc_stdin`] from a **named working directory** inside the corpus — the shape
    /// a suite needs when the cwd is the axis (M53 — the cwd census). Every other runner
    /// defaults to the repository root, which is exactly the assumption a cwd axis has to
    /// stop making.
    pub fn jigc_stdin_from(&self, cwd: &std::path::Path, args: &[&str], stdin: &str) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(cwd)
            .env("HOME", self.home())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        self.select_pack(&mut command);
        let mut child = command.spawn().expect("spawn jigc");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(stdin.as_bytes())
            .expect("write jigc stdin");
        child.wait_with_output().expect("wait for jigc")
    }

    /// Run `jigc <args>` with `stdin` piped, assert it succeeded, return stdout.
    /// The slot/payload writers all read `--from-file -`.
    pub fn jigc_stdin_ok(&self, args: &[&str], stdin: &str) -> String {
        let out = self.jigc_stdin(args, stdin);
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

    /// Add one repeatable item and return the address the binary **emitted** —
    /// driven verbatim downstream, never a test-side reconstruction of the slug
    /// rule (the same discipline [`Self::start_workflow`] applies to task ids).
    pub fn add_item(&self, section: &str, title: &str, task: &str) -> String {
        self.jigc_ok(&["doc", "add-item", section, "--title", title, "--task", task])
            .trim_end_matches('\n')
            .to_string()
    }

    /// Set one typed field.
    pub fn set_field(&self, address: &str, task: &str, value: &str) {
        self.jigc_ok(&[
            "doc",
            "set-field",
            address,
            "--value",
            value,
            "--task",
            task,
        ]);
    }

    /// Set one prose slot from stdin.
    pub fn set_slot(&self, address: &str, task: &str, prose: &str) {
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

    /// [`State::CommittedSingletons`]: the committed singleton set, each created
    /// through **its own driving workflow** (`planning` gates the roadmap and the
    /// decisions log; `form-vision` gates the vision; `record-change` gates the
    /// changelog) and finalized, so all four land committed at their `placement`
    /// homes.
    ///
    /// The roadmap's milestone and the changelog's groups are authored here rather
    /// than in a state of their own — they are the *multi-slot repeatable*, *nested
    /// repeatable* and *`id-from` enum* shape cells, and both doctypes are
    /// singletons authored through a driving workflow, which is what this state is.
    fn build_committed_singletons(&self) {
        let plan = self.start_workflow("planning", "plan the first wave");
        self.jigc_ok(&[
            "doc", "create", "roadmap", "--title", "Roadmap", "--task", &plan,
        ]);
        // A POPULATED milestone: `milestones` is the only multi-slot repeatable
        // either pack ships, so both prose leaves are set — an item with one leaf
        // filled would leave the class half-covered.
        let milestone = self.add_item("roadmap:roadmap#milestones", "M-Alpha", &plan);
        self.set_slot(
            &format!("{milestone}/proves"),
            &plan,
            "That the composed loop lands one task end to end.",
        );
        self.set_slot(
            &format!("{milestone}/decomposition"),
            &plan,
            "Increment 1 — the enumeration seam. Increment 2 — the fixture builder.",
        );
        self.jigc_ok(&[
            "doc",
            "create",
            "decisions-log",
            "--title",
            "Decisions Log",
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

        self.build_populated_changelog();
    }

    /// The `changelog` half of [`State::CommittedSingletons`] — the set's first
    /// dev-pack member, authored through `record-change` and finalized to its
    /// literal `CHANGELOG.md` home.
    ///
    /// Both of the doctype's `id-from: category` repeatables are populated (the
    /// staging area *and* a cut release's nested groups), so the *nested repeatable*
    /// and *`id-from` enum* cells are covered at both depths rather than only at the
    /// shallow one. The optional `link` field is authored too — an optional leaf
    /// left empty in every state would make its write path unswept.
    fn build_populated_changelog(&self) {
        let task = self.start_workflow("record-change", "cut the first release");
        self.jigc_ok(&[
            "doc",
            "create",
            "changelog",
            "--title",
            "Changelog",
            "--task",
            &task,
        ]);

        // The staging area: a change-group whose id comes from the `category` enum.
        let staged = self.add_item("changelog:changelog#unreleased-changes", "changed", &task);
        self.set_slot(
            &format!("{staged}/notes"),
            &task,
            "- the fixture builder gained shape-class coverage",
        );

        // A cut release, carrying a NESTED change-group (the `Leaf::Repeatable`).
        let release = self.add_item("changelog:changelog#releases", "1.0.0", &task);
        self.set_field(
            &format!("{release}/link"),
            &task,
            "https://example.com/compare/0.9.0...1.0.0",
        );
        let group = self.add_item(&format!("{release}/changes"), "added", &task);
        self.set_slot(
            &format!("{group}/notes"),
            &task,
            "- the trial-shaped fixture builder",
        );

        self.finalize(&task, "changelog", "cut the first release", false);
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

    /// [`State::Vendored`]: a gitignored vendored runtime tree beside tracked
    /// source, plus the two dev-pack docs that describe that source.
    ///
    /// The repo furniture is written directly (jigc has no verb that authors a
    /// vendor dir). The `.gitignore` line is **appended** (never overwritten) so
    /// whatever `setup` left in place survives, and the code + test files are
    /// committed — an untracked file would also appear in the ingest walk, which
    /// would make the contrast prove nothing, *and* a code-anchor over an untracked
    /// file would not be the tracked-symbol case the coverage rule asks for.
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
            (
                VENDORED_TEST_FILE,
                "#[test]\nfn pads_the_input() {\n    assert_eq!(pad(\"x\"), \"x\");\n}\n",
            ),
        ] {
            let path = repo.join(rel);
            fs::create_dir_all(path.parent().expect("a vendored file has a parent"))
                .expect("create the vendored file's dir");
            fs::write(&path, body).unwrap_or_else(|e| panic!("write {rel}: {e}"));
        }

        self.git(&["add", ".gitignore", VENDORED_CODE_FILE, VENDORED_TEST_FILE]);
        self.git(&["commit", "-q", "-m", "vendor the runtime, track the source"]);

        self.build_code_anchored_docs();
    }

    /// The code-anchored half of [`State::Vendored`] — a `spec` and an `arch-doc`
    /// documenting the state's own tracked source, each finalized with a code-anchor
    /// pointed at a **real tracked symbol**.
    ///
    /// `symbol-exists` and `criterion-maps-to-test` are **blocking** by default and
    /// the `doc-code` probe ships beside the built binary, so a finalize that lands
    /// at all is the resolution proof: a dangling anchor would fail the build here,
    /// loudly, instead of leaving a green-but-hollow fixture. The two anchors are
    /// deliberately *different* checks over different grammars — the component's
    /// `implemented-by` resolves a TypeScript declaration, the criterion's
    /// `maps-to-test` a Rust `#[test]` fn (the predicate is Rust-only by design).
    fn build_code_anchored_docs(&self) {
        let spec = self.start_workflow("plan", "spec the padding helper");
        let spec_id = self
            .jigc_ok(&[
                "doc", "create", "spec", "--title", "Padding", "--task", &spec,
            ])
            .trim_end_matches('\n')
            .to_string();
        self.set_slot(
            &format!("{spec_id}#goal"),
            &spec,
            "A helper that pads a string to a fixed width.",
        );
        self.set_slot(
            &format!("{spec_id}#context"),
            &spec,
            "The vendored runtime shipped its own; the tracked source replaces it.",
        );
        let criterion = self.add_item(&format!("{spec_id}#criteria"), "Pads the input", &spec);
        self.set_slot(
            &format!("{criterion}/statement"),
            &spec,
            "Padding a string returns it unchanged when it is already wide enough.",
        );
        self.set_field(
            &format!("{criterion}/maps-to-test"),
            &spec,
            &format!("{VENDORED_TEST_FILE}#{VENDORED_TEST_SYMBOL}"),
        );
        self.finalize(&spec, "spec", "spec the padding helper", false);

        let arch = self.start_workflow("architecture-documentation", "document the padding layer");
        let arch_id = self
            .jigc_ok(&[
                "doc",
                "create",
                "arch-doc",
                "--title",
                "Padding layer",
                "--task",
                &arch,
            ])
            .trim_end_matches('\n')
            .to_string();
        self.set_slot(
            &format!("{arch_id}#overview"),
            &arch,
            "The padding layer owns string-width normalization for the tracked source.",
        );
        // The component title IS the symbol, so `title-names-symbol` holds by
        // construction rather than by a comment.
        let component = self.add_item(
            &format!("{arch_id}#components"),
            VENDORED_CODE_SYMBOL,
            &arch,
        );
        self.set_slot(
            &format!("{component}/description"),
            &arch,
            "Pads a string to the requested width.",
        );
        self.set_field(
            &format!("{component}/implemented-by"),
            &arch,
            &format!("{VENDORED_CODE_FILE}#{VENDORED_CODE_SYMBOL}"),
        );
        self.finalize(&arch, "arch-doc", "document the padding layer", false);
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
    ///
    /// `-b main` for the same reason as the identity: **nothing about this corpus may
    /// come from ambient git config**. A bare `git init` takes its branch name from
    /// `init.defaultBranch`, which is set on a developer machine and unset on a CI
    /// runner — so a consumer that names `main` passes here and fails there, and one
    /// that only *expects git to refuse* `main` passes in both places for two different
    /// reasons. `dev/jigc-rig`'s `gen_git` mirrors this builder and pins it too.
    fn git_init(&self) {
        self.git(&["init", "-q", "-b", "main", "."]);
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
pub const MIGRATED_VISION_PAYLOAD: &str = "\
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

/// A unique throwaway corpus root: the state's name plus pid and
/// [`engine::tempname::unique_nanos`], so parallel `#[test]`s in any number of test
/// binaries never collide.
///
/// The mint must be unique *by construction*, not by winning a race. A raw
/// `SystemTime::now()` is not: macOS truncates it to microsecond granularity, so two
/// of a binary's `#[test]` threads reaching here in the same microsecond would take the
/// *same* root and build into one directory — where the second [`TrialCorpus::git_init`]
/// dies on `fatal: cannot copy … File exists`. `unique_nanos` is strictly increasing
/// within the process and `pid` separates processes, so the pair cannot repeat.
pub fn unique_root(label: &str) -> PathBuf {
    let mut root = std::env::temp_dir();
    root.push(format!(
        "jigc-trial-{label}-{}-{}",
        std::process::id(),
        engine::tempname::unique_nanos(),
    ));
    root
}

/// Recursive copy, in-process rather than `cp -a`.
///
/// A **file** named `.git` is a linked worktree's pointer — the second half of the
/// hazard [`TrialCorpus::copy_state`] refuses, and the half its `.git/worktrees/`
/// check cannot see (a worktree whose registration was pruned, or one belonging to
/// another repo). It is refused here rather than copied, because copying it would
/// point the copy's worktree at the source's absolute git dir.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst)
        .unwrap_or_else(|e| panic!("create the copy dir {}: {e}", dst.display()));
    for entry in fs::read_dir(src).unwrap_or_else(|e| panic!("read {}: {e}", src.display())) {
        let entry = entry.expect("read a source entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let kind = entry.file_type().expect("stat a source entry");
        if kind.is_dir() {
            copy_tree(&from, &to);
        } else if kind.is_file() {
            assert!(
                entry.file_name() != ".git",
                "refusing to copy {}: a `.git` FILE is a linked git worktree's \
                 pointer, holding an ABSOLUTE path to the source's git dir — the \
                 copy would read and write the ORIGINAL (pinning.md §4).",
                from.display(),
            );
            // `fs::copy` preserves the unix mode, so an executable hook stays one.
            fs::copy(&from, &to).unwrap_or_else(|e| panic!("copy {}: {e}", from.display()));
        } else {
            panic!(
                "refusing to copy {}: the fixture corpora hold only files and \
                 directories, and silently dropping anything else would make a copied \
                 state quietly differ from its source.",
                from.display(),
            );
        }
    }
}

/// Read a repo-relative file from a built corpus.
pub fn read(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}
