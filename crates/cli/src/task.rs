//! `jigc task <verb> <id>` — the task lifecycle surface over a named task's working
//! area (`design/write-commands.md` → Lifecycle: `diff` · `validate` · `discard`).
//!
//! Unlike `jigc doc` (which resolves the *single* active task from cwd), the
//! lifecycle verbs take the task `<id>` **explicitly** — the explicit selector the
//! `doc` surface deferred here. Three preview/abandon verbs land in this task
//! (`finalize` is the next one):
//!
//! - **`diff`** — the working changeset vs base: the git diff of the working tree
//!   against the task's pinned base SHA (code) plus the staged managed-doc instances
//!   (which live in the working area, outside the tree). Read-only, always exit 0.
//! - **`validate`** — run `engine::validate::validate_task` over the working area
//!   and render the findings; the exit code tracks `has_blocking()` so validate
//!   *previews what finalize blocks on* (`design/validation.md` → How it gates
//!   `finalize`; `design/reconciliation.md` → Detection timing: the full sweep).
//! - **`discard`** — remove `.jigc/tasks/<id>/`, abandoning the task. Idempotent on
//!   an already-absent area; always exit 0 on success.
//! - **`finalize`** — the commit boundary: read HEAD, ask the engine planner for the
//!   ordered phase plan (preflight → validate → empty-diff guard → render), and on a
//!   clean plan execute the git steps the plan implies — write the rendered message to
//!   a temp file, `git add` the working-tree code changes, `git commit -F <tmp>`
//!   (**never** `--no-verify`), then post-commit (advance the file-state hashes, remove
//!   the working area). A blocking finding or a hook/git rejection aborts cleanly with
//!   the findings / stderr surfaced and **no** commit (`design/finalize.md` → 5. Stage
//!   / 6. Commit / 7. Post-commit; `design/worked-examples.md` → flow #4).
//!
//! Findings render through the shared `crate::render` finding/format renderers (the
//! determinism boundary unaffected — the engine carries the data, the CLI formats).

use crate::cli::Format;
use crate::pack::EmbeddedPack;
use crate::render;
use anyhow::{Context, Result, bail};
use engine::file_state::{self, FileStateRecord};
use engine::finalize::plan_finalize;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Schema, load_schema};
use engine::state::BasePin;
use engine::validate::validate_task;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// The `jigc task <verb>` subcommand tree. Each verb names a task by its `<id>`
/// (`design/write-commands.md` → Lifecycle).
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum TaskCommand {
    /// Show the working changeset vs base (code diff + staged managed docs).
    Diff {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
    /// Run `validate(task)` and render the findings; exit non-zero iff any blocks.
    Validate {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
    /// Abandon the task — remove its working area `.jigc/tasks/<id>/`.
    Discard {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
    /// The commit boundary — validate, render, stage, `git commit`, post-commit.
    Finalize {
        /// The task id (the working-area slug under `.jigc/tasks/`).
        id: String,
    },
}

impl TaskCommand {
    /// Dispatch the parsed `task` verb against `cwd`, mapping the result to a process
    /// exit code. `validate` maps a blocking report to a non-zero exit (the gate
    /// preview); `diff` / `discard` exit 0 on success. An orchestration error
    /// surfaces on stderr with a non-zero exit.
    pub fn dispatch(self, cwd: &Path, format: Format) -> ExitCode {
        let result = match self {
            TaskCommand::Diff { id } => run_diff(cwd, &id),
            TaskCommand::Validate { id } => return run_validate(cwd, &id, format),
            TaskCommand::Discard { id } => run_discard(cwd, &id),
            TaskCommand::Finalize { id } => return run_finalize(cwd, &id, format),
        };
        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err:#}");
                ExitCode::FAILURE
            }
        }
    }
}

/// `jigc task diff <id>` — print the working changeset vs base.
///
/// Two parts: (1) the git diff of the working tree against the task's pinned base
/// SHA (the code changes — "CLI orchestrates, git executes"), and (2) the staged
/// managed-doc instances under the working area's `docs/` (which live outside the
/// tree, so git does not see them). Read-only.
fn run_diff(cwd: &Path, id: &str) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    let base = task.base()?;

    let diff = git_diff(&task.repo_root, &base.sha)?;
    if !diff.trim().is_empty() {
        println!("# code changes vs base {}", base.short);
        print!("{diff}");
        if !diff.ends_with('\n') {
            println!();
        }
    }

    let staged = task.staged_docs()?;
    if !staged.is_empty() {
        println!("# staged docs");
        for (name, body) in staged {
            println!("--- {name}");
            print!("{body}");
            if !body.ends_with('\n') {
                println!();
            }
        }
    }
    Ok(())
}

/// `jigc task validate <id>` — run the task-scope sweep and render its findings; the
/// exit code tracks `has_blocking()` (`design/validation.md` → How it gates
/// `finalize`: validate previews what finalize blocks on). The findings render
/// through `crate::render::validation` in the selected format.
fn run_validate(cwd: &Path, id: &str, format: Format) -> ExitCode {
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            eprintln!("{err:#}");
            return ExitCode::FAILURE;
        }
    };
    match task.validate() {
        Ok(report) => {
            print!("{}", render::validation(format, &report));
            if format != Format::Json {
                println!();
            }
            if report.has_blocking() {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("{err:#}");
            ExitCode::FAILURE
        }
    }
}

/// `jigc task discard <id>` — remove the task's working area, abandoning it
/// (`design/write-commands.md` → Lifecycle: abandon). Idempotent: an already-absent
/// area is not an error.
fn run_discard(cwd: &Path, id: &str) -> Result<()> {
    let task = TaskArea::resolve(cwd, id)?;
    if task.dir.exists() {
        std::fs::remove_dir_all(&task.dir)
            .with_context(|| format!("could not discard task `{id}` at {:?}", task.dir))?;
    }
    Ok(())
}

/// The commit doctype name — the task's workflow-provisioned doc whose sink is the
/// git message (`design/finalize.md` → Commit-doc rendering). The commit instance is
/// provisioned under `commit:<task-id>`, so the commit slug is the task id.
const COMMIT_TYPE: &str = "commit";

/// `jigc task finalize <id>` — execute the commit boundary (`design/finalize.md` →
/// 5. Stage / 6. Commit / 7. Post-commit; `design/worked-examples.md` → flow #4).
///
/// The engine is a pure planner ([`plan_finalize`]); the CLI owns every git step. The
/// loop: read HEAD (`git rev-parse`), run `validate(task)`, compute the diff-presence
/// signal, load the commit schema, and ask the planner for the ordered plan. On a
/// blocking finding (base-mismatch, validate block, empty diff) the planner returns the
/// findings — rendered, non-zero exit, **no** commit. On a clean plan: write the
/// rendered message to a temp file, `git add` the working-tree code changes between base
/// and tree, `git commit -F <tmp>` (**never** `--no-verify`). A hook / git rejection
/// aborts non-zero with git's stderr surfaced and no working-area change. On a successful
/// commit, run post-commit (advance the file-state hashes, remove the working area) —
/// best-effort: a failure there is logged, not raised (the commit is already truth).
fn run_finalize(cwd: &Path, id: &str, format: Format) -> ExitCode {
    let task = match TaskArea::resolve(cwd, id) {
        Ok(task) => task,
        Err(err) => {
            eprintln!("{err:#}");
            return ExitCode::FAILURE;
        }
    };
    match task.finalize(id, format) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err:#}");
            ExitCode::FAILURE
        }
    }
}

/// A named task's working area: the repo root, the `.jigc/` home, and the task dir.
struct TaskArea {
    repo_root: PathBuf,
    jigc_root: PathBuf,
    dir: PathBuf,
    pack: EmbeddedPack,
}

impl TaskArea {
    /// Resolve the task `id`'s working area from `cwd`. The repo root is the
    /// nearest `.git` ancestor; the task dir is `<repo>/.jigc/tasks/<id>/`. A
    /// task that does not exist rejects with the start-a-task route.
    fn resolve(cwd: &Path, id: &str) -> Result<Self> {
        let repo_root = discover_repo_root(cwd)
            .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
        let jigc_root = repo_root.join(".jigc");
        let dir = jigc_root.join("tasks").join(id);
        if !dir.is_dir() {
            bail!("no task `{id}` — start one with `jigc start \"<intent>\"`");
        }
        Ok(Self {
            repo_root,
            jigc_root,
            dir,
            pack: EmbeddedPack::new(),
        })
    }

    /// Read the task's pinned base commit.
    fn base(&self) -> Result<BasePin> {
        let path = self.dir.join("base.json");
        let bytes = std::fs::read(&path)
            .with_context(|| format!("could not read the base pin at {path:?}"))?;
        serde_json::from_slice(&bytes).with_context(|| format!("malformed base pin at {path:?}"))
    }

    /// Load every shipped schema, keyed by doctype — the set the conformance sweep
    /// resolves staged instances against (the engine stays domain-empty; the CLI
    /// feeds the cascade in).
    fn schemas(&self) -> Result<BTreeMap<String, Schema>> {
        let mut out = BTreeMap::new();
        for id in self.pack.list(PackResourceKind::Schemas) {
            let bytes = self
                .pack
                .read(PackResourceKind::Schemas, &id)
                .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
            let schema = load_schema(&bytes)
                .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
            out.insert(schema.ty.clone(), schema);
        }
        Ok(out)
    }

    /// Run the task-scope validation sweep against the committed-state `file-state`
    /// record (`design/validation.md` → How it gates `finalize`; `design/reconciliation.md`
    /// → Detection timing: the `task validate` full sweep).
    ///
    /// The record is **loaded** (so drift of a copied-in committed doc against its
    /// real committed baseline is detected) but **not persisted** back: the staged
    /// working-area docs are *not* committed state, and "task writes do not update the
    /// committed-state hash" (`reconciliation.md` → Hash re-baselining — the record
    /// advances only at adopt/absorb/commit, none of which is a `validate` of a
    /// working-area edit). So a working-area instance with no committed baseline
    /// baseline-adopts fresh (advisory) each run rather than drifting against a stale
    /// staged hash — clean validate stays clean as the agent fills it. Absorb of
    /// clean OOB drift on a *committed* doc is inc-5.
    fn validate(&self) -> Result<engine::result::ValidationReport> {
        let schemas = self.schemas()?;
        let mut record = FileStateRecord::load(&self.jigc_root).with_context(|| {
            format!(
                "could not load the file-state record under {:?}",
                self.jigc_root
            )
        })?;
        validate_task(&self.dir, &schemas, &mut record)
            .with_context(|| format!("validating task at {:?}", self.dir))
    }

    /// Execute the `finalize` transaction (`design/finalize.md` → 5–7). Returns the
    /// process exit code: `SUCCESS` on a landed commit, `FAILURE` on any planner block
    /// (rendered + surfaced) or hook/git rejection (git's stderr surfaced). An
    /// orchestration error (git unavailable, malformed pin) bubbles as `Err`.
    fn finalize(&self, id: &str, format: Format) -> Result<ExitCode> {
        let base = self.base()?;
        let head = git_head(&self.repo_root)?;

        // The validate report the planner gates on — one engine, two entry points
        // (`design/finalize.md` → 2. Validate: no private check path).
        let report = self.validate()?;

        // The diff-presence signal the planner's empty-commit guard needs: any
        // working-tree change from base, or any staged managed doc.
        let has_diff = !git_diff(&self.repo_root, &base.sha)?.trim().is_empty()
            || !self.staged_docs()?.is_empty();

        let schemas = self.schemas()?;
        let commit_schema = schemas
            .get(COMMIT_TYPE)
            .with_context(|| format!("the embedded pack ships no `{COMMIT_TYPE}` schema"))?;

        // The engine plans; the CLI executes. A blocking branch returns the findings.
        let plan = match plan_finalize(
            &self.dir,
            &base,
            &head,
            &report,
            has_diff,
            commit_schema,
            id,
        ) {
            Ok(plan) => plan,
            Err(findings) => {
                let report = engine::result::ValidationReport::new(findings);
                eprint!("{}", render::validation(format, &report));
                if format != Format::Json {
                    eprintln!();
                }
                return Ok(ExitCode::FAILURE);
            }
        };

        // Phase 5–6: stage + commit. Write the rendered message to a temp file, stage
        // the working-tree code changes (everything differing from base —
        // `design/finalize.md` → Dirty-tree policy), and `git commit -F <tmp>` (never
        // `--no-verify`). A hook/git rejection surfaces git's stderr verbatim, no commit.
        let msg_path = self.dir.join("finalize-message.tmp");
        std::fs::write(&msg_path, &plan.message)
            .with_context(|| format!("could not write the commit message to {msg_path:?}"))?;
        let commit_result = (|| -> Result<()> {
            // The transient `.jigc/` subdirs (`tasks/`/`index/`/`state/`) are gitignored
            // via `.jigc/.gitignore` — the working area is never committed
            // (`design/storage.md` → repository layout). Ensure it exists so the
            // `git add --all` stage below picks up only `config/` + the code changes.
            self.ensure_jigc_gitignore()?;
            git_run(&self.repo_root, &["add", "--all"])?;
            git_commit(&self.repo_root, &msg_path)?;
            Ok(())
        })();
        let _ = std::fs::remove_file(&msg_path);
        if let Err(err) = commit_result {
            eprintln!("{err:#}");
            return Ok(ExitCode::FAILURE);
        }

        // Phase 7 — post-commit (best-effort: a failure here is logged, not raised; the
        // commit is already truth). Advance the file-state hashes for the committed
        // working set, then remove the working area.
        self.post_commit(&plan.hash_updates);

        Ok(ExitCode::SUCCESS)
    }

    /// Ensure `.jigc/.gitignore` ignores the transient subdirs (`tasks/`/`index/`/
    /// `state/`) so a `finalize` stage never commits the working area or the
    /// rebuildable caches (`design/storage.md` → repository layout: `.jigc/` is one home
    /// whose `config/` is committed while `tasks/`/`index/`/`state/` are gitignored).
    /// Idempotent — written only when absent.
    fn ensure_jigc_gitignore(&self) -> Result<()> {
        let path = self.jigc_root.join(".gitignore");
        if path.exists() {
            return Ok(());
        }
        std::fs::create_dir_all(&self.jigc_root)
            .with_context(|| format!("could not create {:?}", self.jigc_root))?;
        std::fs::write(&path, "tasks/\nindex/\nstate/\n")
            .with_context(|| format!("could not write {path:?}"))?;
        Ok(())
    }

    /// Phase 7 (`design/finalize.md` → 7. Post-commit, best-effort). Advance the
    /// `file-state` record for every committed file (the plan's managed-doc hash set
    /// plus the committed working-tree files), then remove the working area. Each step
    /// self-heals on failure — a stale `.jigc/tasks/<id>/` is cleaned by the next
    /// `discard`/`finalize`, and a stale hash re-baselines on the next probe — so a
    /// failure is logged to stderr, never raised.
    fn post_commit(&self, hash_updates: &BTreeMap<String, String>) {
        if let Err(err) = self.advance_file_state(hash_updates) {
            eprintln!("note: post-commit file-state update failed (self-heals): {err:#}");
        }
        if let Err(err) = std::fs::remove_dir_all(&self.dir) {
            eprintln!("note: post-commit working-area removal failed (self-heals): {err:#}");
        }
    }

    /// Record the committed working set into the `file-state` record and save it. The
    /// plan's managed-doc hash set is empty in the commit-only case; the committed
    /// code files (the working-tree changes that just landed) are hashed from `HEAD`'s
    /// tree so the next `file-state` probe sees them in-sync.
    fn advance_file_state(&self, hash_updates: &BTreeMap<String, String>) -> Result<()> {
        let mut record = FileStateRecord::load(&self.jigc_root)
            .with_context(|| format!("loading the file-state record under {:?}", self.jigc_root))?;
        for (path, hash) in hash_updates {
            record.record(path.clone(), hash.clone());
        }
        // Hash the files the just-landed commit touched, reading their committed bytes.
        for path in git_commit_files(&self.repo_root)? {
            if let Ok(bytes) = git_show_file(&self.repo_root, &path) {
                record.record(path, file_state::hash_bytes(&bytes));
            }
        }
        record
            .save(&self.jigc_root)
            .with_context(|| format!("saving the file-state record under {:?}", self.jigc_root))?;
        Ok(())
    }

    /// The staged managed-doc instances under the working area's `docs/`, in
    /// path-sorted filename order, as `(filename, body)` pairs. An absent `docs/`
    /// yields an empty list (nothing staged yet).
    fn staged_docs(&self) -> Result<Vec<(String, String)>> {
        let docs = self.dir.join("docs");
        let read = match std::fs::read_dir(&docs) {
            Ok(read) => read,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => {
                return Err(err).with_context(|| format!("reading staged docs in {docs:?}"));
            }
        };
        let mut names: Vec<String> = Vec::new();
        for entry in read {
            let entry = entry.context("reading a staged-docs entry")?;
            if !entry
                .file_type()
                .context("staged-docs entry type")?
                .is_file()
            {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.ends_with(".md") {
                names.push(name);
            }
        }
        names.sort();
        let mut out = Vec::with_capacity(names.len());
        for name in names {
            let body = std::fs::read_to_string(docs.join(&name))
                .with_context(|| format!("reading staged doc {name}"))?;
            out.push((name, body));
        }
        Ok(out)
    }
}

/// Run `git diff <base>` in `repo_root`, returning the unified diff of the working
/// tree against the pinned base commit (`storage.md` → CLI and git: "CLI
/// orchestrates, git executes"). Shells out to the user's `git`
/// (`DECISIONS.md` 2026-05-31 → Git invocation).
fn git_diff(repo_root: &Path, base_sha: &str) -> Result<String> {
    let out = Command::new("git")
        .args(["diff", base_sha])
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git diff {base_sha}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout).context("`git diff` produced non-UTF-8 output")
}

/// Read HEAD's full SHA via `git rev-parse HEAD` (the supplied HEAD the planner
/// checks the base pin against — `design/finalize.md` → 1. Preflight). Shells out to
/// the user's `git` (`DECISIONS.md` 2026-05-31 → Git invocation).
fn git_head(repo_root: &Path) -> Result<String> {
    git_capture(repo_root, &["rev-parse", "HEAD"])
}

/// Run `git <args>` in `repo_root` for its side effect (e.g. `add`), bailing with
/// git's stderr on a non-zero exit. Shells out to the user's `git`.
fn git_run(repo_root: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// `git commit -F <message-file>` (`design/finalize.md` → 6. Commit). **Never** passes
/// `--no-verify`: the user's `pre-commit` / `commit-msg` hooks are policy and the CLI
/// respects them — a hook rejection surfaces git's stderr verbatim (the correction
/// signal), and no commit lands.
fn git_commit(repo_root: &Path, message_file: &Path) -> Result<()> {
    let out = Command::new("git")
        .arg("commit")
        .arg("-F")
        .arg(message_file)
        .current_dir(repo_root)
        .output()
        .context("could not run `git commit` (is git on PATH?)")?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stdout = String::from_utf8_lossy(&out.stdout);
        bail!(
            "`git commit` was rejected (no commit was made):\n{}{}",
            stdout.trim(),
            stderr.trim()
        );
    }
    Ok(())
}

/// The paths the just-landed `HEAD` commit touched (`git show --name-only`). Used by
/// post-commit to advance the `file-state` hashes for the committed working set.
fn git_commit_files(repo_root: &Path) -> Result<Vec<String>> {
    let out = git_capture(
        repo_root,
        &["show", "--name-only", "--format=", "--no-renames", "HEAD"],
    )?;
    Ok(out
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Read a file's committed bytes at `HEAD` (`git show HEAD:<path>`). Returns an error
/// for a path absent at HEAD (e.g. a deletion); the caller skips those.
fn git_show_file(repo_root: &Path, path: &str) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .arg("show")
        .arg(format!("HEAD:{path}"))
        .current_dir(repo_root)
        .output()
        .context("could not run `git show`")?;
    if !out.status.success() {
        bail!("`git show HEAD:{path}` failed");
    }
    Ok(out.stdout)
}

/// Run `git <args>` in `repo_root`, returning trimmed stdout, bailing on non-zero.
fn git_capture(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout)
        .context("`git` produced non-UTF-8 output")
        .map(|s| s.trim().to_string())
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the same
/// discovery `crate::start` / `crate::doc` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}
