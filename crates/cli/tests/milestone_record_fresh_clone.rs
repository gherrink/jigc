//! M39 Increment 4 / T5 — fresh-clone resume: reseed the demoted `.jigc` cache from the
//! committed record on the milestone-op read path (`design/team-ready-state.md` → Engine
//! capability 2 (read-back): "on a fresh clone (no `.jigc/` working state) the first
//! milestone op parses the record back into `BasePin` + `TaskList` and re-seeds the cache";
//! "Continue" means resume, not WIP recovery). Under a `[dev ▸ methodology]` project whose
//! composed cascade resolves the `milestone-record` schema.
//!
//! One flow, driving the REAL binary against a throwaway git repo:
//!
//!   create → add-task ×2 → **simulate a fresh clone** (`rm -rf .jigc`, then restore the
//!   *tracked* `.jigc` bits — committed `config/` + `.gitignore` — the way a clone would,
//!   leaving the gitignored WIP `.jigc/milestones/` gone). Then:
//!     (a) `jigc doc show milestone-record:<id> --format json` returns the pinned shape
//!         (Inc 1's committed-record read — works without the cache); and
//!     (b) the fresh clone is **answered** without a workbench (`list-tasks` reads the ids
//!         out of the committed record and materializes nothing — it is `VerbKind::Read`,
//!         M49 Inc 2 T4) and a subsequent **operating** op (`provision`) **re-derives the
//!         cache from the record** and continues (resume-from-scratch), rebuilding
//!         `.jigc/milestones/<id>/`.
//!
//! Pre-fix, (b) failed at both halves: every milestone verb bailed "milestone does not exist"
//! because the WIP cache was gone and nothing reseeded it from the committed record.
//!
//! ---
//!
//! **M47 Increment 3 / T3 — the refusal's route is followable from a fresh clone.** The M39
//! reseed above rebuilt only `.jigc/milestones/<id>/{base,tasks}.json`; the per-sub-task
//! working areas `.jigc/tasks/<sub-task-id>/` were **never** rebuilt, so the milestone-execution
//! workflow's own emitted `` Spawn: `cd .jigc/worktrees/<sub> && jigc workflow sub-task --task
//! <sub>` `` line failed with *"no task `<sub>`"* — and its route pointed back at
//! `jigc milestone list-tasks`, which names the very sub-task that has no area. A **loop**.
//! T2's new `milestone.zero-contribution` refusal routes a fresh-clone operator straight into
//! it (`jigc milestone provision <id>` → execute → dead end), so the shared reseed site closes
//! it here: it rebuilds every sub-task area the record names — base pin, verbatim `intent`, and
//! the pack's default sub-task workflow — from the committed record.
//!
//! The three arms below build their fresh-clone state **only by driving the binary** — a real
//! `git clone` of a real origin repo, never a hand-written `.jigc/tasks/…`. That is the point:
//! the lost build's mask was this increment's own suite hand-writing exactly the state the tool
//! could not rebuild (`completions/artifacts/M47/recovery-report.md` → §4.2 finding 2), so a
//! fixture that reaches inside `.jigc/tasks/` would prove nothing here.
//!
//!   (c) the whole route, run as emitted: `finalize` (exit 3) → its route's own `` `…` `` span
//!       verbatim → `execute` → each emitted `Spawn:` span verbatim through a `jigc` shim on
//!       `PATH`, reaching a sub-task compose at exit 0 that carries the record's intent → stage
//!       code in the worktrees → a re-run `finalize` that **lands** it;
//!   (d) the entry-door axis: every milestone verb reaches the reseed through **one** shared
//!       site, so each of `provision` / `execute` / `finalize` rebuilds the areas as the
//!       *first* op on its own fresh clone (`finalize` too — the reseed runs before its
//!       refusal). `list-tasks` left this axis at M49 Inc 2 T4 — it is `VerbKind::Read` and
//!       rebuilds nothing;
//!   (e) the declared bound: `--workflow` is workbench-local and **not** fresh-clone-durable —
//!       the reseed re-derives the pack default, so the operator's own overridden re-entry is
//!       refused **loudly** by the shipped W-equality guard rather than composing the override.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-fresh-clone-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// The `schema-version` the shipped methodology manifest declares for `ty` — the stamp the
/// origin's committed record carries, and so the stamp the fresh clone must read back.
///
/// **Derived, never spelled**: the witness is that the stamp survived the clone, not that
/// it equals a particular integer (a literal reddened this arm at the M49
/// `milestone-record` 2→3 bump).
fn declared_schema_version(ty: &str) -> u32 {
    let bytes = fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("packs")
            .join("methodology")
            .join("config")
            .join("schema-manifest.yaml"),
    )
    .expect("read the shipped methodology schema-manifest");
    let manifest: engine::manifest::Manifest =
        serde_yaml_ng::from_slice(&bytes).expect("the methodology manifest deserializes");
    manifest
        .doctypes
        .iter()
        .find(|entry| entry.ty == ty)
        .unwrap_or_else(|| panic!("the methodology manifest declares `{ty}`"))
        .schema_version
}

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads to
/// assemble the composition dev-highest (so the dev `docs-root` knob applies → `docs/`).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack
/// supersedes the marker).
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert an invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("cache-rework.md")
}

/// The demoted `.jigc` cache dir for milestone `cache-rework`.
fn cache_dir(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join("cache-rework")
}

#[test]
fn fresh_clone_reseeds_cache_from_record_and_resumes() {
    let repo = TempDir::new("resume");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    // The base pin `create` materializes pins the HEAD at create time (the init commit).
    let base_sha = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    let base_short = git(repo.path(), &["rev-parse", "--short", "HEAD"])
        .trim()
        .to_string();

    assert_ok(
        &run_jigc(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache rework"],
        ),
        "`[dev ▸ methodology]` `jigc milestone create`",
    );
    // Commit the *tracked* `.jigc` bits (the compose marker + the generated `.gitignore`)
    // exactly as `jigc setup` would — these survive a clone; the WIP under `.jigc/` does not.
    git(
        repo.path(),
        &["add", ".jigc/config/packs.yaml", ".jigc/.gitignore"],
    );
    git(repo.path(), &["commit", "-q", "-m", "jigc config"]);

    assert_ok(
        &run_jigc(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Warm the read cache",
            ],
        ),
        "add-task #1",
    );
    assert_ok(
        &run_jigc(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Evict cold entries",
            ],
        ),
        "add-task #2",
    );

    let record_before = fs::read_to_string(record_path(repo.path())).expect("record committed");

    // --- Simulate a fresh clone: drop ALL of `.jigc/`, then restore only the *tracked*
    //     bits (committed `config/` + `.gitignore`) the clone would carry. The gitignored
    //     WIP (`.jigc/milestones/…`) is gone — nothing on disk re-derives the milestone.
    fs::remove_dir_all(repo.path().join(".jigc")).expect("rm -rf .jigc");
    git(repo.path(), &["checkout", "--", ".jigc"]);
    assert!(
        !cache_dir(repo.path()).exists(),
        "the WIP cache must be absent after the fresh-clone simulation",
    );

    // (a) `doc show` reads the committed record directly (Inc 1) — works without the cache.
    let show = run_jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "milestone-record:cache-rework",
            "--format",
            "json",
        ],
    );
    assert_ok(&show, "fresh-clone `doc show`");
    let json = String::from_utf8(show.stdout).expect("utf-8 json");
    // The pinned `--format json` shape (§ The read surface): type, slug,
    // fields{base,status,schema-version} (`schema-version` joined the pinned witness
    // fields when the M40 A1 methodology manifest froze milestone-record; the version is
    // READ from that manifest rather than spelled), sections{tasks:[{task-id,intent,
    // status}…]}.
    let stamp = format!(
        "\"schema-version\": \"{}\"",
        declared_schema_version("milestone-record")
    );
    assert!(
        json.contains("\"type\": \"milestone-record\"")
            && json.contains("\"slug\": \"cache-rework\"")
            && json.contains("\"status\": \"active\"")
            && json.contains(&stamp)
            && json.contains("\"task-id\": \"warm-the-read-cache\"")
            && json.contains("\"intent\": \"Warm the read cache\"")
            && json.contains("\"task-id\": \"evict-cold-entries\""),
        "fresh-clone `doc show --format json` returns the pinned record shape; got:\n{json}",
    );
    // The compound `base` pin projects as a structured `{ sha, short }` object — NOT the
    // space-joined scalar the `.md` stores (`DECISIONS.md` 2026-07-07). Parse the json and
    // assert the object shape (a substring check would pass on the scalar too).
    let doc: serde_json::Value = serde_json::from_str(&json).expect("pinned json parses");
    let base = &doc["fields"]["base"];
    assert_eq!(
        base["sha"].as_str(),
        Some(base_sha.as_str()),
        "the pinned json `base` object carries the full SHA; got:\n{json}",
    );
    assert_eq!(
        base["short"].as_str(),
        Some(base_short.as_str()),
        "the pinned json `base` object carries the short SHA; got:\n{json}",
    );
    assert!(
        !base.is_string(),
        "the pinned json `base` must be a structured object, not the space-joined scalar; got:\n{json}",
    );
    // Every task item carries its minted `id` — the handle a fresh-clone driver addresses
    // the item back at (M42 — `design/doc-read-surface.md` → The item `id` closes the json
    // contract). The witness doctype proves the key on the pinned contract's own witness.
    let tasks = doc["sections"]["tasks"]
        .as_array()
        .expect("the pinned record carries a `tasks` item array");
    let ids: Vec<&str> = tasks
        .iter()
        .map(|item| {
            item["id"]
                .as_str()
                .unwrap_or_else(|| panic!("every item object carries its `id`; got:\n{json}"))
        })
        .collect();
    assert_eq!(
        ids,
        vec!["warm-the-read-cache", "evict-cold-entries"],
        "each task item's `id` is its minted item id; got:\n{json}",
    );

    // (b) The fresh clone is answered — and the answer costs no workbench. `list-tasks` is
    // `VerbKind::Read` (M49 Inc 2 T4): it reads the recorded ids and materializes nothing, so
    // the re-seed is proven here by an **operating** door instead.
    let list = run_jigc(
        repo.path(),
        home.path(),
        &["milestone", "list-tasks", "cache-rework"],
    );
    assert_ok(&list, "fresh-clone `list-tasks` (the read answer)");
    let out = String::from_utf8(list.stdout).expect("utf-8 list-tasks stdout");
    assert!(
        out.contains("evict-cold-entries") && out.contains("warm-the-read-cache"),
        "fresh-clone `list-tasks` emits the recorded sub-task ids; got:\n{out}",
    );
    assert!(
        !cache_dir(repo.path()).exists(),
        "the read answered from the committed record — it must materialize no workbench \
         (`crates/cli/tests/read_verb_acts_nothing.rs` sweeps that over the whole `Read` set)",
    );

    // The operating op re-seeds the demoted cache from the committed record (resume, not WIP
    // recovery).
    let provisioned = run_jigc(
        repo.path(),
        home.path(),
        &["milestone", "provision", "cache-rework"],
    );
    assert_ok(
        &provisioned,
        "fresh-clone `provision` (resume-from-scratch)",
    );
    assert!(
        cache_dir(repo.path()).join("tasks.json").is_file()
            && cache_dir(repo.path()).join("base.json").is_file(),
        "the milestone op re-seeded `.jigc/milestones/cache-rework/{{base,tasks}}.json` from the record",
    );

    // The committed record was not disturbed by the reseed (it is the source of truth).
    assert_eq!(
        fs::read_to_string(record_path(repo.path())).expect("record still readable"),
        record_before,
        "the read-path reseed must not rewrite the committed record",
    );
}

// =======================================================================================
// M47 Inc 3 T3 — the refusal's route is followable from a fresh clone.
// =======================================================================================

/// Initialize a `[dev ▸ methodology]` origin repo whose **initial** commit already carries the
/// compose marker — so the milestone's base pin (HEAD at `create`) is that commit and every
/// later commit in the range is record-only, the shape `finalize`'s base guard advances over.
fn init_methodology_origin(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    write_compose_marker(repo);
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Drive the binary to build the origin's milestone state: `create` + two `add-task`s, each
/// landing its own record-only commit. Nothing is hand-written.
fn drive_origin_milestone(repo: &Path, home: &Path) {
    assert_ok(
        &run_jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "`jigc milestone create`",
    );
    for intent in ["Warm the read cache", "Evict cold entries"] {
        assert_ok(
            &run_jigc(
                repo,
                home,
                &["milestone", "add-task", "cache-rework", intent],
            ),
            "`jigc milestone add-task`",
        );
    }
}

/// `git clone <origin> <dir>/clone` — a **real** clone, carrying exactly the tracked bytes a
/// teammate gets: the committed record and config, and none of the gitignored `.jigc/` workbench.
fn clone_origin(origin: &Path, workdir: &Path) -> PathBuf {
    let clone = workdir.join("clone");
    git(
        workdir,
        &[
            "clone",
            "-q",
            &origin.display().to_string(),
            &clone.display().to_string(),
        ],
    );
    git(&clone, &["config", "user.email", "test@example.com"]);
    git(&clone, &["config", "user.name", "Test"]);
    assert!(
        !clone.join(".jigc").join("milestones").exists(),
        "a fresh clone must not carry the gitignored milestone workbench",
    );
    assert!(
        !clone.join(".jigc").join("tasks").exists(),
        "a fresh clone must not carry the gitignored sub-task working areas",
    );
    clone
}

/// The first `` `…` `` span of `text`, split into an argv — the **emitted** route bytes an
/// agent would copy, never a reconstruction.
fn first_command_span(text: &str) -> Vec<String> {
    let (_, rest) = text.split_once('`').expect("the route carries a `…` span");
    let (span, _) = rest.split_once('`').expect("the `…` span closes");
    span.split_whitespace().map(str::to_owned).collect()
}

/// Every `` Spawn: `…` `` span of a composed milestone-execution view, **verbatim** — each is a
/// shell line (`cd <worktree> && jigc workflow <W> --task <id>`), run as emitted below.
fn spawn_spans(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("Spawn: `"))
        .filter_map(|rest| rest.strip_suffix('`'))
        .map(str::to_owned)
        .collect()
}

/// Install a `jigc` shim on a throwaway `PATH` entry, so an emitted `Spawn:` span — which names
/// the bare command `jigc`, as an agent would run it — resolves to the binary under test.
#[cfg(unix)]
fn install_jigc_shim(dir: &Path) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let bin = dir.join("shim-bin");
    fs::create_dir_all(&bin).expect("mk the shim bin dir");
    let shim = bin.join("jigc");
    fs::write(
        &shim,
        format!("#!/bin/sh\nexec {:?} \"$@\"\n", env!("CARGO_BIN_EXE_jigc")),
    )
    .expect("write the jigc shim");
    fs::set_permissions(&shim, fs::Permissions::from_mode(0o755)).expect("chmod the jigc shim");
    bin
}

/// Run an emitted shell span verbatim through `sh -c`, with the `jigc` shim first on `PATH`.
#[cfg(unix)]
fn run_span(cwd: &Path, home: &Path, shim_bin: &Path, span: &str) -> std::process::Output {
    let path = match std::env::var("PATH") {
        Ok(rest) => format!("{}:{rest}", shim_bin.display()),
        Err(_) => shim_bin.display().to_string(),
    };
    Command::new("sh")
        .arg("-c")
        .arg(span)
        .current_dir(cwd)
        .env("HOME", home)
        .env("PATH", path)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the emitted span")
}

/// A sub-task's rebuilt working area — the three files [`engine::state::mint_task`] writes.
fn sub_task_area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub)
}

// ---------------------------------------------------------------------------------------
// (c) The whole route, followed as emitted.
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn the_zero_contribution_route_is_followable_from_a_fresh_clone() {
    let origin = TempDir::new("route-origin");
    let home = TempDir::new("home");
    init_methodology_origin(origin.path());
    drive_origin_milestone(origin.path(), home.path());

    let workdir = TempDir::new("route-clone");
    let clone = clone_origin(origin.path(), workdir.path());
    let shim_bin = install_jigc_shim(workdir.path());

    // --- step 1: the refusal, and its own route bytes ------------------------------------
    let blocked = run_jigc(
        &clone,
        home.path(),
        &["--format", "json", "milestone", "finalize", "cache-rework"],
    );
    assert_eq!(
        blocked.status.code(),
        Some(3),
        "a fresh clone's zero-contribution finalize blocks; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let report: serde_json::Value =
        serde_json::from_slice(&blocked.stdout).expect("the blocked envelope is valid JSON");
    let route = report["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .find(|f| f["code"] == "milestone.zero-contribution")
        .and_then(|f| f["route"].as_str())
        .unwrap_or_else(|| panic!("the refusal carries a route; got:\n{report:#}"))
        .to_string();

    // --- step 2: run the route's own argv, verbatim --------------------------------------
    let argv = first_command_span(&route);
    assert_eq!(
        argv,
        vec!["jigc", "milestone", "provision", "cache-rework"],
        "the route leads with the provision step; got: {route}",
    );
    let provisioned = run_jigc(
        &clone,
        home.path(),
        &argv[1..].iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert_ok(&provisioned, "the emitted `jigc milestone provision` span");

    // The shared reseed rebuilt each sub-task's working area from the record — base pin,
    // verbatim intent, recorded workflow.
    for (sub, intent) in [
        ("warm-the-read-cache", "Warm the read cache"),
        ("evict-cold-entries", "Evict cold entries"),
    ] {
        let area = sub_task_area(&clone, sub);
        assert!(
            area.join("base.json").is_file(),
            "the reseed rebuilds `{sub}`'s base pin",
        );
        assert_eq!(
            fs::read_to_string(area.join("intent")).expect("the rebuilt area carries its intent"),
            intent,
            "the rebuilt intent is the record's, verbatim",
        );
        assert_eq!(
            fs::read_to_string(area.join("workflow")).expect("the rebuilt area names a workflow"),
            "sub-task",
            "the rebuilt area records the pack's default sub-task workflow",
        );
    }

    // --- step 3: compose the execution view and run each emitted `Spawn:` span ------------
    let executed = run_jigc(
        &clone,
        home.path(),
        &["milestone", "execute", "cache-rework"],
    );
    assert_ok(&executed, "`jigc milestone execute` on the fresh clone");
    let view = String::from_utf8(executed.stdout).expect("utf-8 composed view");
    let spans = spawn_spans(&view);
    assert_eq!(
        spans.len(),
        2,
        "the fan-out emits one `Spawn:` span per sub-task; got:\n{view}",
    );
    for span in &spans {
        let composed = run_span(&clone, home.path(), &shim_bin, span);
        assert!(
            composed.status.success(),
            "the emitted span `{span}` must reach a sub-task compose at exit 0; got {:?}\n\
             stdout:\n{}\nstderr:\n{}",
            composed.status,
            String::from_utf8_lossy(&composed.stdout),
            String::from_utf8_lossy(&composed.stderr),
        );
        let text = String::from_utf8_lossy(&composed.stdout).to_string();
        assert!(
            text.contains("Warm the read cache") || text.contains("Evict cold entries"),
            "the composed sub-task carries the record's verbatim intent; got:\n{text}",
        );
    }

    // --- step 4: the sub-agents stage their code, and the re-run finalize lands it ---------
    for (sub, rel) in [
        ("warm-the-read-cache", "src/warm.rs"),
        ("evict-cold-entries", "src/evict.rs"),
    ] {
        let wt = clone.join(".jigc").join("worktrees").join(sub);
        let path = wt.join(rel);
        fs::create_dir_all(path.parent().expect("code parent")).expect("mk code parent");
        fs::write(&path, format!("pub fn {}() {{}}\n", sub.replace('-', "_")))
            .expect("write sub-task code");
        git(&wt, &["add", rel]);
    }

    let before = git(&clone, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse::<u32>()
        .expect("commit count parses");
    let landed = run_jigc(
        &clone,
        home.path(),
        &["milestone", "finalize", "cache-rework"],
    );
    assert_ok(
        &landed,
        "the re-run `jigc milestone finalize` after the route",
    );
    let after = git(&clone, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse::<u32>()
        .expect("commit count parses");
    assert!(after > before, "the re-run finalize lands a commit");
    let committed = git(&clone, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.contains("src/warm.rs") && committed.contains("src/evict.rs"),
        "both sub-tasks' staged code lands in the milestone boundary; got:\n{committed}",
    );
}

// ---------------------------------------------------------------------------------------
// (d) The entry-door axis — one shared reseed site, every milestone verb.
// ---------------------------------------------------------------------------------------

#[test]
fn every_milestone_entry_door_rebuilds_the_sub_task_areas_on_a_fresh_clone() {
    // The axis is the set of verbs that reach [`reseed_cache`]; `create` is excluded by design
    // (it targets an id no record may own yet, so it takes the opposite guard). `finalize` is
    // in even though it refuses — the reseed runs ahead of the refusal, which is precisely what
    // makes the refusal's route followable. `list-tasks` is **out**: M49 Inc 2 T4 took the one
    // `VerbKind::Read` milestone verb off the reseed, so it rebuilds nothing by design. The
    // membership itself is fenced against the code in `subtask_discard_record.rs`
    // (`doors_reaching_the_reseed_site`), which reddens if this set drifts from the call sites.
    for door in ["provision", "execute", "finalize"] {
        let origin = TempDir::new(&format!("door-origin-{door}"));
        let home = TempDir::new("home");
        init_methodology_origin(origin.path());
        drive_origin_milestone(origin.path(), home.path());

        let workdir = TempDir::new(&format!("door-clone-{door}"));
        let clone = clone_origin(origin.path(), workdir.path());

        // The door is the FIRST jigc invocation on this clone — nothing else could have
        // rebuilt the areas.
        let out = run_jigc(&clone, home.path(), &["milestone", door, "cache-rework"]);
        assert!(
            out.status.code() == Some(0) || out.status.code() == Some(3),
            "`jigc milestone {door}` on a fresh clone either serves or refuses, never faults; \
             got {:?}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        for sub in ["warm-the-read-cache", "evict-cold-entries"] {
            let area = sub_task_area(&clone, sub);
            assert!(
                area.join("intent").is_file() && area.join("workflow").is_file(),
                "`jigc milestone {door}` must rebuild `{sub}`'s working area from the record",
            );
        }
    }
}

// ---------------------------------------------------------------------------------------
// (e) The declared bound — `--workflow` is not fresh-clone-durable, and says so loudly.
// ---------------------------------------------------------------------------------------

#[test]
fn an_overridden_workflow_is_refused_loudly_on_fresh_clone_re_entry() {
    let origin = TempDir::new("override-origin");
    let home = TempDir::new("home");
    init_methodology_origin(origin.path());
    assert_ok(
        &run_jigc(
            origin.path(),
            home.path(),
            &["milestone", "create", "Cache rework"],
        ),
        "`jigc milestone create`",
    );
    assert_ok(
        &run_jigc(
            origin.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Tune the eviction clock",
                "--workflow",
                "single-task",
            ],
        ),
        "`jigc milestone add-task --workflow single-task`",
    );

    let workdir = TempDir::new("override-clone");
    let clone = clone_origin(origin.path(), workdir.path());
    assert_ok(
        &run_jigc(
            &clone,
            home.path(),
            &["milestone", "provision", "cache-rework"],
        ),
        "`jigc milestone provision` on the fresh clone",
    );

    // The committed record carries `task-id`/`intent`/`status` and NOTHING about the minting
    // workflow, so the reseed re-derives the pack default. The operator who minted the
    // override re-enters with it — and is refused loudly, naming what the area actually
    // records, rather than being silently composed into the wrong workflow.
    let wt = clone
        .join(".jigc")
        .join("worktrees")
        .join("tune-the-eviction-clock");
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args([
            "workflow",
            "single-task",
            "--task",
            "tune-the-eviction-clock",
        ])
        .current_dir(&wt)
        .env("HOME", home.path())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the overridden re-entry");
    assert!(
        !out.status.success(),
        "an overridden re-entry on a fresh clone must be refused, never composed; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        stderr.contains("workflow-refs.workflow-mismatch") || stderr.contains("was minted with"),
        "the refusal is the shipped W-equality guard, naming the recorded workflow; got:\n{stderr}",
    );
}
