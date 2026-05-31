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
//!
//! Findings render through the shared `crate::render` finding/format renderers (the
//! determinism boundary unaffected — the engine carries the data, the CLI formats).

use crate::cli::Format;
use crate::pack::EmbeddedPack;
use crate::render;
use anyhow::{Context, Result, bail};
use engine::file_state::FileStateRecord;
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

/// Walk up from `start` to the directory holding `.git` (the repo root) — the same
/// discovery `crate::start` / `crate::doc` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}
