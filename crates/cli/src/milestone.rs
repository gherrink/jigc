//! `jigc milestone <verb>` — the `milestone` work-unit front door
//! (`design/write-commands.md` → Minting a milestone + its task list).
//!
//! Two verbs over the engine's milestone primitives ([`engine::milestone`]):
//!
//! - **`create "<title>"`** — read HEAD (the **single shared base** every sub-task
//!   inherits — "CLI orchestrates, git executes"), then [`mint_milestone`] opens
//!   `.jigc/milestones/<slug>/` with that base pin and an empty task list. A serial
//!   slug collision rejects with the engine's routed blocking finding.
//! - **`add-task <milestone-id> "<intent>"`** — [`add_task`] mints a sub-task
//!   pinned to the milestone's shared base in an isolated `.jigc/tasks/<sub>/`
//!   area and appends it; an unknown milestone or a within-milestone serial slug
//!   collision rejects (the deterministic suffix runs only at the join).
//!
//! The CLI does the git I/O (reading HEAD) and locates the `.jigc/` home; the
//! engine performs no git I/O, so minting stays a pure function of its inputs. As
//! with `tasks/`, the milestone area is gitignored runtime state
//! (`design/storage.md` → repository layout) — a doc-elaboration pin
//! (`DECISIONS.md` 2026-06-04) — so create ensures `.jigc/.gitignore` lists
//! `milestones/`.

use crate::cli::Format;
use crate::pack::make_pack;
use crate::render;
use anyhow::{Context, Result, bail};
use engine::finding::Finding;
use engine::index::load_committed;
use engine::milestone::{
    JoinOutcome, add_from_spec, add_task, join, milestone_dir, mint_milestone, read_base_pin,
    read_task_list,
};
use engine::packsource::PackResourceKind;
use engine::schema::{Schema, load_schema};
use engine::state::BasePin;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// The work-workflow a milestone sub-task is minted from. A sub-task is a `task`
/// work-unit that will be executed end-to-end, so it is minted as the canonical
/// `single-task` work-workflow (the cascade `default-workflow` is the
/// `creates-task: false` router, which is not a work-workflow — `DECISIONS.md`
/// 2026-06-04). The choice of execution workflow per sub-task is an M8 control-plane
/// concern; here it only records the minting workflow.
const SUB_TASK_WORKFLOW: &str = "single-task";

/// The `jigc milestone <verb>` subcommand tree (`design/write-commands.md` →
/// Minting a milestone + its task list).
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum MilestoneCommand {
    /// Mint a milestone work-unit (id = frozen slug from the title), opening its
    /// gitignored area with one shared base pinned at HEAD and an empty task list.
    Create {
        /// The milestone title — slugged into the `milestone:<slug>` id.
        title: String,
    },
    /// Mint a sub-task under an existing milestone (pinned to the milestone's
    /// shared base, isolated `tasks/<sub>/` area) and append it to the task list.
    AddTask {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// The sub-task intent — slugged into the sub-task id.
        intent: String,
    },
    /// Seed a milestone's task list from a committed spec: mint one sub-task per
    /// repeatable `criterion` of the spec (criterion text as intent). A spec with
    /// zero criteria blocks with `milestone.no-criteria` ("nothing to seed from").
    AddFromSpec {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
        /// The committed spec's address (`spec:<slug>`) to enumerate criteria from.
        spec_addr: String,
    },
    /// Emit a milestone's sub-task ids in **canonical id-sorted order** — the
    /// deterministic order the by-task-id join enumerates, surfaced through the
    /// binary (`design/storage.md` → The by-task-id join).
    ListTasks {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
    },
    /// Merge the milestone's sub-task areas into the parent working overlay by the
    /// **by-task-id join** — enumerate sub-areas by sorted task id, disjoint-union
    /// their staged docs (collision-suffixing distinct `created` instances, blocking
    /// a same-doc clash), and report the merged outcome. **Commits nothing** — wiring
    /// the suffix-resolved overlay into `finalize` is a later increment
    /// (`design/storage.md` → The by-task-id join; `design/worked-examples.md` →
    /// flow 9). A blocking finding (a same-doc clash, an unknown milestone) surfaces
    /// on stderr with its route and exits non-zero.
    Join {
        /// The milestone id (the slug under `.jigc/milestones/`).
        milestone_id: String,
    },
}

impl MilestoneCommand {
    /// Dispatch the parsed `milestone` verb against `cwd`, mapping the result to a
    /// process exit code. Success prints a summary on stdout and exits 0; a blocking
    /// finding (serial collision, unknown milestone) surfaces on stderr with its
    /// route and exits non-zero (`design/write-commands.md` → Minting a milestone).
    pub fn dispatch(self, cwd: &Path, format: Format) -> ExitCode {
        // The `join` verb reports a `JoinOutcome` (overlay + findings), not a one-line
        // summary, and a same-doc clash is a *blocking finding inside an Ok outcome*
        // (the merge ran, then routed the contention) — so it has its own dispatch arm.
        if let MilestoneCommand::Join { milestone_id } = self {
            return dispatch_join(cwd, format, &milestone_id);
        }
        let result = match self {
            MilestoneCommand::Create { title } => run_create(cwd, &title),
            MilestoneCommand::AddTask {
                milestone_id,
                intent,
            } => run_add_task(cwd, &milestone_id, &intent),
            MilestoneCommand::AddFromSpec {
                milestone_id,
                spec_addr,
            } => run_add_from_spec(cwd, &milestone_id, &spec_addr),
            MilestoneCommand::ListTasks { milestone_id } => run_list_tasks(cwd, &milestone_id),
            MilestoneCommand::Join { .. } => unreachable!("`Join` is handled above"),
        };
        match result {
            Ok(summary) => {
                println!("{}", render::milestone(format, &summary));
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("{err:#}");
                ExitCode::FAILURE
            }
        }
    }
}

/// `jigc milestone create "<title>"` — read HEAD, then mint the milestone area
/// with that single shared base and an empty task list. Ensures `.jigc/.gitignore`
/// lists `milestones/` (the area is disposable runtime state). Returns the summary
/// line; a serial collision surfaces as the engine's routed blocking finding.
fn run_create(cwd: &Path, title: &str) -> Result<String> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_root = repo_root.join(".jigc");
    ensure_jigc_gitignore(&jigc_root)?;
    let base = read_head(&repo_root)?;

    let minted = mint_milestone(&jigc_root, title, base).map_err(finding_to_err)?;
    Ok(format!(
        "minted milestone:{} (shared base {})",
        minted.id, minted.base.short
    ))
}

/// `jigc milestone add-task <milestone-id> "<intent>"` — mint a sub-task pinned to
/// the milestone's shared base in its own isolated area and append it. Returns the
/// summary line; an unknown milestone or a within-milestone collision surfaces as
/// the engine's routed blocking finding.
fn run_add_task(cwd: &Path, milestone_id: &str, intent: &str) -> Result<String> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_root = repo_root.join(".jigc");

    let added =
        add_task(&jigc_root, milestone_id, intent, SUB_TASK_WORKFLOW).map_err(finding_to_err)?;
    Ok(format!(
        "added task:{} to milestone:{}",
        added.task.id, added.milestone_id
    ))
}

/// `jigc milestone add-from-spec <milestone-id> <spec-addr>` — seed the milestone's
/// task list with one sub-task per repeatable `criterion` of a committed spec (the
/// criterion text as the sub-task intent). The CLI does the I/O — discover the repo
/// root, load the shipped schemas the engine resolves the spec address against — and
/// hands them to [`add_from_spec`], which performs no git I/O. Returns a summary
/// naming the seeded sub-tasks; an unknown milestone, an unknown/transient/
/// unparseable spec, or a **zero-criteria** spec (`milestone.no-criteria`) surfaces
/// as the engine's routed blocking finding and exits non-zero
/// (`design/write-commands.md` → Minting a milestone).
fn run_add_from_spec(cwd: &Path, milestone_id: &str, spec_addr: &str) -> Result<String> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_root = repo_root.join(".jigc");
    let schemas = shipped_schemas()?;

    let added = add_from_spec(
        &jigc_root,
        &repo_root,
        &schemas,
        milestone_id,
        spec_addr,
        SUB_TASK_WORKFLOW,
    )
    .map_err(finding_to_err)?;

    let ids: Vec<&str> = added.iter().map(|a| a.task.id.as_str()).collect();
    Ok(format!(
        "seeded {} sub-task(s) into milestone:{milestone_id} from {spec_addr}: {}",
        ids.len(),
        ids.join(", ")
    ))
}

/// Load every shipped schema keyed by doctype — the set [`add_from_spec`] resolves
/// the spec address's type against (the engine stays domain-empty; the CLI feeds the
/// pack in, the same idiom `TaskArea::schemas` uses).
fn shipped_schemas() -> Result<BTreeMap<String, Schema>> {
    let pack = make_pack();
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
        let schema =
            load_schema(&bytes).with_context(|| format!("the `{}` schema parses", id.as_str()))?;
        out.insert(schema.ty.clone(), schema);
    }
    Ok(out)
}

/// `jigc milestone list-tasks <milestone-id>` — read the milestone's persisted
/// task list and emit its sub-task ids in **canonical id-sorted order** (the
/// deterministic order the by-task-id join enumerates, surfaced through the
/// binary, not just the internal enumerate fn). Returns the summary line; an
/// unknown milestone (no area / unreadable list) surfaces as a context-wrapped
/// error and exits non-zero.
fn run_list_tasks(cwd: &Path, milestone_id: &str) -> Result<String> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_root = repo_root.join(".jigc");
    let dir = milestone_dir(&jigc_root, milestone_id);
    if !dir.is_dir() {
        bail!(
            "milestone `{milestone_id}` does not exist\n  route: create it first with `jigc milestone create \"<title>\"`"
        );
    }
    let list = read_task_list(&dir)
        .with_context(|| format!("could not read the task list for milestone `{milestone_id}`"))?;
    // Enumeration is id-sorted — the order the join reads, not the recorded
    // insertion order.
    let ids = list.enumerate();
    Ok(format!(
        "milestone:{milestone_id} tasks ({}): {}",
        ids.len(),
        ids.join(", ")
    ))
}

/// Dispatch `jigc milestone join <milestone-id>`: run the by-task-id join, render the
/// merged outcome, and map it to the exit code. A locator/IO error or an unknown
/// milestone (an `Err(Finding)`) routes to stderr and exits non-zero **before** any
/// summary. A successful merge always renders the outcome on stdout (so the agent sees
/// the suffix/rewrite decisions); if any **blocking** finding rode inside the outcome —
/// a same-doc clash, an isolation violation — its route also goes to stderr and the
/// process exits non-zero. The verb **commits nothing** (Increment 4 wires the
/// suffix-resolved overlay into finalize); a clash leaves the working tree untouched.
fn dispatch_join(cwd: &Path, format: Format, milestone_id: &str) -> ExitCode {
    let outcome = match run_join(cwd, milestone_id) {
        Ok(outcome) => outcome,
        Err(err) => {
            eprintln!("{err:#}");
            return ExitCode::FAILURE;
        }
    };
    // The merged-overlay summary always prints (the agent reads the suffix/rewrite
    // decisions even when the join is clean).
    println!("{}", render::milestone_join(format, milestone_id, &outcome));

    // A blocking finding inside the outcome (e.g. `join.same-doc-clash`) routes to
    // stderr and gates the exit code — nothing is committed regardless.
    let blocking: Vec<&Finding> = outcome
        .findings
        .iter()
        .filter(|f| f.severity == engine::finding::Severity::Blocking)
        .collect();
    if blocking.is_empty() {
        ExitCode::SUCCESS
    } else {
        for finding in blocking {
            eprintln!("{}", finding.message);
            if let Some(route) = &finding.route {
                eprintln!("  route: {route}");
            }
        }
        ExitCode::FAILURE
    }
}

/// `jigc milestone join <milestone-id>` — the CLI I/O around the engine's by-task-id
/// [`join`]: discover the repo root + `.jigc/` home, read the milestone's **shared
/// base** pin (the stamp the committed edge index is keyed to), load the shipped
/// schemas the per-area overlay derivation resolves staged-doc types against, and load
/// the committed [`EdgeIndex`](engine::index::EdgeIndex) the per-sub-area cross-area
/// ref walk resolves against. The engine performs no git I/O; the CLI feeds it the
/// resolved inputs (`design/storage.md` → The by-task-id join). An unknown milestone
/// (no area) surfaces as the engine's routed `milestone.unknown` block.
fn run_join(cwd: &Path, milestone_id: &str) -> Result<JoinOutcome> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let jigc_root = repo_root.join(".jigc");
    let schemas = shipped_schemas()?;

    // The committed edge index is keyed to the milestone's shared base — the commit
    // every sub-task inherited — so the cross-area ref walk resolves against the store
    // as it stood at that base. An unknown milestone has no base pin; fall back to the
    // engine's routed `milestone.unknown` block by leaving the join to detect it.
    let head = match read_base_pin(&milestone_dir(&jigc_root, milestone_id)) {
        Ok(pin) => pin.sha,
        Err(_) => String::new(),
    };
    let committed = load_committed(&repo_root, &jigc_root, &schemas, &head);

    join(&jigc_root, &repo_root, milestone_id, &schemas, &committed).map_err(finding_to_err)
}

/// Ensure `.jigc/.gitignore` ignores the transient runtime subdirs, including
/// `milestones/` (`design/storage.md` → repository layout; `DECISIONS.md`
/// 2026-06-04 → `milestones/` gitignored like `tasks/`). Idempotent — the file is
/// (re)written only when it is absent or does not already list `milestones/`, so an
/// adapter-written `.gitignore` (which predates `milestones/`) is amended once.
fn ensure_jigc_gitignore(jigc_root: &Path) -> Result<()> {
    const ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\n";
    let path = jigc_root.join(".gitignore");
    let needs_write = match std::fs::read_to_string(&path) {
        Ok(existing) => !existing.lines().any(|l| l.trim() == "milestones/"),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
        Err(err) => return Err(err).with_context(|| format!("could not read {path:?}")),
    };
    if needs_write {
        std::fs::create_dir_all(jigc_root)
            .with_context(|| format!("could not create {jigc_root:?}"))?;
        std::fs::write(&path, ENTRIES).with_context(|| format!("could not write {path:?}"))?;
    }
    Ok(())
}

/// Read HEAD as a [`BasePin`] (full + short SHA) via the user's `git` — the same
/// idiom `crate::start::read_head` uses ("CLI orchestrates, git executes";
/// `DECISIONS.md` 2026-05-31 → Git invocation).
fn read_head(repo_root: &Path) -> Result<BasePin> {
    let sha = git_rev_parse(repo_root, &["rev-parse", "HEAD"])?;
    let short = git_rev_parse(repo_root, &["rev-parse", "--short", "HEAD"])?;
    Ok(BasePin::new(sha, short))
}

/// Run `git <args>` in `repo_root` and return the single trimmed line of stdout.
fn git_rev_parse(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    String::from_utf8(out.stdout)
        .context("`git` produced non-UTF-8 output")
        .map(|s| s.trim().to_string())
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the same
/// discovery `crate::start` / `crate::task` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route —
/// the same envelope minting already uses (a hard block is a blocking-severity
/// finding carrying a route, `DECISIONS.md` 2026-05-31).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    match finding.route {
        Some(route) => anyhow::anyhow!("{}\n  route: {route}", finding.message),
        None => anyhow::anyhow!("{}", finding.message),
    }
}
