//! End-to-end integration test for bare `jigc start` orientation (read-only).
//!
//! Drives the built `jigc` binary against throwaway temp repos and asserts the
//! two increment-1 orientation states from `design/bootstrap.md` → Orientation
//! output examples:
//!   1. unset project (no `.jigc/config/`) → `Run: jigc setup`
//!   2. clean, no active task (`.jigc/config/` present) → provenance header +
//!      the workflow catalog with each `when` hint + the routing footer.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, temp repos are built with `std::fs`, and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-start-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Mark `root` as a git repo for repo-root discovery, *without* invoking git —
/// a bare `.git` directory is enough for `locate`, and it proves bare `start`
/// runs git-free.
fn mark_repo(root: &Path) {
    fs::create_dir_all(root.join(".git")).expect("create .git marker");
}

/// Run the built `jigc start` binary with `cwd = repo` and `$HOME = home`.
fn run_start(repo: &Path, home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("start")
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc start --format <format>` with `cwd = repo` and `$HOME = home`.
fn run_start_format(repo: &Path, home: &Path, format: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--format", format])
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

#[test]
fn clean_no_task_json_is_valid_state_tagged_and_carries_no_footer() {
    let repo = TempDir::new("clean-json");
    mark_repo(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    let home = TempDir::new("home");

    let out = run_start_format(repo.path(), home.path(), "json");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(out.status.success(), "must exit 0; got {:?}", out.status);
    assert!(
        !stdout.contains(ROUTING_FOOTER),
        "JSON output must carry no routing footer; got:\n{stdout}",
    );

    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{stdout}"));
    assert_eq!(value["state"], serde_json::json!("clean"), "got:\n{stdout}");
    let workflows = value["workflows"]
        .as_array()
        .unwrap_or_else(|| panic!("workflows must be an array; got:\n{stdout}"));
    let single_task = workflows
        .iter()
        .find(|w| w["id"] == serde_json::json!("single-task"))
        .unwrap_or_else(|| panic!("the catalog must list single-task; got:\n{stdout}"));
    assert_eq!(
        single_task["when"],
        serde_json::json!(
            "implement one scoped change end-to-end, recording its decisions as ADRs and user-facing effects on the changelog"
        ),
        "got:\n{stdout}",
    );
}

#[test]
fn unset_project_json_is_valid_state_tagged_and_carries_no_footer() {
    let repo = TempDir::new("unset-json");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start_format(repo.path(), home.path(), "json");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(out.status.success(), "must exit 0; got {:?}", out.status);
    assert!(
        !stdout.contains(ROUTING_FOOTER),
        "JSON output must carry no routing footer; got:\n{stdout}",
    );
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{stdout}"));
    assert_eq!(
        value["state"],
        serde_json::json!("unset-project"),
        "got:\n{stdout}",
    );
}

#[test]
fn human_format_carries_the_routing_footer() {
    let repo = TempDir::new("clean-human");
    mark_repo(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    let home = TempDir::new("home");

    let out = run_start_format(repo.path(), home.path(), "human");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(out.status.success(), "must exit 0; got {:?}", out.status);
    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "human output must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn clean_no_task_prints_header_catalog_and_routing_footer() {
    let repo = TempDir::new("clean");
    mark_repo(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config"))
        .expect("create .jigc/config project layer");
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path());
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start` must exit 0; got {:?}",
        out.status
    );
    assert!(
        stdout.contains("Pack: "),
        "clean-no-task orientation must print the cascade/provenance header; got:\n{stdout}",
    );
    assert!(
        stdout.contains("single-task"),
        "the workflow catalog must list `single-task`; got:\n{stdout}",
    );
    assert!(
        stdout.contains("implement one scoped change end-to-end"),
        "each catalog entry must carry its `when` hint; got:\n{stdout}",
    );
    assert!(
        stdout.contains("— jigc · run `jigc start` for orientation; all writes through `jigc`."),
        "agent-text output must end with the routing footer; got:\n{stdout}",
    );
}

/// M48 Increment 9 / T5 (RC-pre-1.0 **F14**) — **a granted gate is named where the
/// pick is made.** `single-task` bundles the `changelog` create-gate with the ADR
/// one, so a change with no user-facing effect finalizes with
/// `changelog-recording.gate-granted-unused` — which the trial met as an ambush,
/// because neither routing surface said the changelog was in the bundle. Both must
/// name it: the composed `jigc start` catalog line (all the router shows, one line
/// per workflow) and the `jigc describe` projection (the fuller `description:` +
/// `usage:` story). Naming the gate is the style guide's routing-surface rule —
/// `when:` names the *discriminating trigger axes*, and "does it record a
/// user-facing change?" is one (`design/surface-contract.md` → The surface style
/// guide).
///
/// The member set is **derived from the pack's own `allows-create:`**, never
/// hard-coded, so a future selectable work-workflow that bundles the changelog gate
/// joins the axis with no edit here — and the assertions run over the **emitted**
/// catalog line / describe paragraph, not a reconstruction of them.
#[test]
fn every_selectable_changelog_granting_workflow_names_the_gate_on_both_routing_surfaces() {
    let granting = selectable_workflows_granting_the_changelog_gate();
    assert!(
        !granting.is_empty(),
        "the axis must be non-empty — the dev pack ships at least one selectable \
         work-workflow granting the `changelog` create-gate",
    );

    let repo = TempDir::new("changelog-gate");
    mark_repo(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config"))
        .expect("create .jigc/config project layer");
    let home = TempDir::new("home");

    let catalog = String::from_utf8(run_start(repo.path(), home.path()).stdout)
        .expect("utf-8 stdout from `jigc start`");
    let described = String::from_utf8(run_describe_workflows(repo.path(), home.path()).stdout)
        .expect("utf-8 stdout from `jigc describe --workflows`");

    for id in &granting {
        let line = catalog_line(&catalog, id);
        assert!(
            line.contains("changelog"),
            "`{id}` grants the changelog create-gate, so its catalog line must name it \
             (else `changelog-recording.gate-granted-unused` ambushes at finalize); got:\n{line}",
        );
        let paragraph = describe_paragraph(&described, id);
        assert!(
            paragraph.contains("changelog"),
            "`{id}` grants the changelog create-gate, so its `describe` prose must name it; \
             got:\n{paragraph}",
        );
    }
}

/// Every **selectable work-workflow** of the embedded dev pack whose
/// `allows-create:` admits a `changelog` — the axis
/// `changelog-recording.gate-granted-unused` can fire over from a router pick,
/// read out of the shipped pack rather than listed here.
fn selectable_workflows_granting_the_changelog_gate() -> Vec<String> {
    use engine::packsource::{PackResourceKind, PackSource};

    let pack = cli::pack::EmbeddedPack::new();
    let mut ids: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .into_iter()
        .filter(|id| {
            let bytes = pack
                .read(PackResourceKind::Workflows, id)
                .unwrap_or_else(|e| panic!("workflow `{id}` must read back: {e}"));
            let def = engine::compose::load_workflow_def(&bytes)
                .unwrap_or_else(|f| panic!("workflow `{id}` must load: {f:?}"));
            def.selectable
                && def.creates_task
                && def.allows_create.iter().any(|g| g.doc_type == "changelog")
        })
        .map(|id| id.as_str().to_string())
        .collect();
    ids.sort();
    ids
}

/// The emitted catalog row for `id` (`- <id> — <when>`), panicking if the catalog
/// carries none — the assertion reads the bytes the router prints.
fn catalog_line<'a>(catalog: &'a str, id: &str) -> &'a str {
    let prefix = format!("- {id} — ");
    catalog
        .lines()
        .map(str::trim_start)
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| panic!("the catalog must carry a row for `{id}`; got:\n{catalog}"))
}

/// The emitted `describe` paragraph for `id` (it opens `<id> is …` and runs to the
/// next blank line), panicking if the projection carries none.
fn describe_paragraph<'a>(described: &'a str, id: &'a str) -> &'a str {
    let opener = format!("{id} is ");
    let start = described
        .find(&opener)
        .unwrap_or_else(|| panic!("`describe` must narrate `{id}`; got:\n{described}"));
    let rest = &described[start..];
    match rest.find("\n\n") {
        Some(end) => &rest[..end],
        None => rest,
    }
}

/// Run `jigc describe --workflows` with `cwd = repo` and `$HOME = home`.
fn run_describe_workflows(repo: &Path, home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["describe", "--workflows"])
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// T12 (M45 Inc 10) — the real bare-`start` orientation routes to the preview
/// surface: it names `jigc workflow <id> --preview` as the way to read a workflow's
/// step text before minting (findings §69 — the pull-tier fix itself not
/// pull-discoverable).
#[test]
fn clean_no_task_routes_to_workflow_preview() {
    let repo = TempDir::new("clean-preview");
    mark_repo(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config"))
        .expect("create .jigc/config project layer");
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path());
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start` must exit 0; got {:?}",
        out.status
    );
    assert!(
        stdout.contains("`jigc workflow <id> --preview`"),
        "clean-no-task orientation must route to the preview surface; got:\n{stdout}",
    );
}

#[test]
fn unset_project_prints_run_jigc_setup() {
    let repo = TempDir::new("unset");
    mark_repo(repo.path());
    // No `.jigc/config/` — the project layer is absent (unset project).
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path());
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start` must exit 0; got {:?}",
        out.status
    );
    assert!(
        stdout.contains("isn't set up"),
        "unset-project orientation must say the project isn't set up; got:\n{stdout}",
    );
    assert!(
        stdout.contains("Run: `jigc setup`"),
        "unset-project orientation must route to `jigc setup`; got:\n{stdout}",
    );
}

#[test]
fn bare_start_is_read_only_no_files_created() {
    let repo = TempDir::new("readonly");
    mark_repo(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    let home = TempDir::new("home");

    let before = list_repo_entries(repo.path());
    let _ = run_start(repo.path(), home.path());
    let after = list_repo_entries(repo.path());

    assert_eq!(
        before, after,
        "bare `jigc start` is read-only: it must create no files (orientation never writes)",
    );
}

/// Collect every entry path under `root`, sorted, for a before/after diff that
/// proves bare `start` wrote nothing.
fn list_repo_entries(root: &Path) -> Vec<PathBuf> {
    let mut acc = Vec::new();
    collect(root, &mut acc);
    acc.sort();
    acc
}

fn collect(dir: &Path, acc: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        acc.push(path.clone());
        if path.is_dir() {
            collect(&path, acc);
        }
    }
}
