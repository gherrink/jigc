//! M50 Increment 11 / T2 — **the fixer's walk exists as a composed workflow**
//! ([milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md)
//! → §3; `completions/artifacts/M50/settle-record.md` → D11).
//!
//! §3 has said *"each confirmed fix is its own agent"* since M38 and *"triage fixes
//! should ride"* the `fan-out`/`join` primitive since M42 — while the step the loop
//! actually composes (`packs/methodology/steps/fix-gate.yaml`) told the fixer to run
//! **strictly serial over the one shared working tree**. A Fix phase can only fan out
//! if `jigc milestone add-task … --workflow <id>` has an id to take: at HEAD before
//! this task `--workflow fix-task` was refused with `workflow-refs.unknown-workflow`,
//! naming the 33 loaded workflows, so the fan-out had no unit and the prose rule was
//! all there was.
//!
//! This suite pins the unit itself, on the **emitted bytes of the real binary** over
//! the `[dev ▸ methodology]` pack-set a plain `jigc setup` installs:
//!
//!   * `milestone add-task … --workflow fix-task` exits 0 and records the workflow;
//!   * `milestone execute` emits the sub-task's `Spawn:` line naming `fix-task` — and
//!     the test **runs that emitted line verbatim** (its `cd` and its `jigc workflow`
//!     argv, lifted off the composed bytes and executed as written, never rebuilt in
//!     test code), because those bytes are the contract an agent runs;
//!   * the walk that comes back carries the three things the fixer's brief owes: the
//!     **axis-derivation rule** (derive the class, report its membership, never fix
//!     the reported site list), **both isolation constraints** (`git add` your own
//!     paths inside this worktree; never `git commit`, never `jigc task finalize`
//!     here), and the **staged read-back** (`jigc doc show commit:<id> --task <id>`).
//!
//! **The omitting contexts are swept too** (hardening #5), because a hidden workflow's
//! correctness is as much about where it must *not* appear: the dev pack alone must
//! still refuse `--workflow fix-task` with the routed `workflow-refs.unknown-workflow`
//! block and mint nothing (inert, never a panic or a half-added task), and the router
//! front door over the composed pair must not list `fix-task` — it is `selectable:
//! false`, spawned by a Fix-phase fan-out, and a router pick could never land it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-fix-task-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (composition reads HEAD).
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = dir`, `$HOME = home`, and no `JIGC_PACK_DIR` (the
/// embedded / marker-composed path a real `jigc setup` produces).
fn run_in(dir: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(dir)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// A repo a real `jigc setup` installed into — the `[dev ▸ methodology]` pack-set a
/// dogfooding project actually runs, with the compose marker and the `.jigc/` ignore
/// rule the fan-out's worktrees resolve their shared `.jigc/` through.
fn composed_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(&format!("{tag}-repo"));
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());
    let out = run_in(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    (repo, home)
}

/// The same repo with the methodology pack composed **off** — the omitting context:
/// the dev pack alone, which ships no `fix-task`.
fn dev_only_repo(tag: &str) -> (TempDir, TempDir) {
    let (repo, home) = composed_repo(tag);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: false\n",
    )
    .expect("drop the compose marker");
    (repo, home)
}

/// Re-flow a composed body so an assertion may quote a sentence the pack wraps across
/// lines (the `methodology_staging_contract.rs` idiom).
fn flowed(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Extract every `` Spawn: `<launch-line>` `` directive's backtick-wrapped launch line
/// from a composed view — the agent-facing artifact, never a reconstruction.
fn spawn_lines(composed: &str) -> Vec<String> {
    composed
        .lines()
        .filter_map(|l| l.strip_prefix("Spawn: `"))
        .filter_map(|rest| rest.strip_suffix('`'))
        .map(str::to_owned)
        .collect()
}

const MILESTONE: &str = "Audit fix round";
const MILESTONE_ID: &str = "audit-fix-round";
const FINDING: &str = "the leaf boundary truncates slot prose";
const SUB_TASK: &str = "leaf-boundary-truncates-slot";

/// The happy path, end to end on the emitted bytes: a confirmed finding becomes a
/// milestone sub-task minted on `fix-task`, `execute` spawns it, and the launch line
/// it emits — run **verbatim** — composes the fixer's walk with the axis rule, both
/// isolation constraints, and the staged read-back.
#[test]
fn a_confirmed_finding_fans_out_as_a_fix_task_sub_task() {
    let (repo, home) = composed_repo("happy");
    let root = repo.path();
    let home = home.path();

    let out = run_in(root, home, &["milestone", "create", MILESTONE]);
    assert!(
        out.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // 1. The door the Fix phase needs: `--workflow fix-task` is accepted.
    let out = run_in(
        root,
        home,
        &[
            "milestone",
            "add-task",
            MILESTONE_ID,
            FINDING,
            "--workflow",
            "fix-task",
        ],
    );
    assert!(
        out.status.success(),
        "`milestone add-task … --workflow fix-task` must exit 0 — the Fix phase's fan-out \
         has no unit without it; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // 2. `execute` composes the fan-out, and the sub-task's Spawn line names `fix-task`
    //    — the RECORDED minting workflow, so the fan-out needed no new bearer.
    let out = run_in(root, home, &["milestone", "execute", MILESTONE_ID]);
    let view = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "`milestone execute` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let spawns = spawn_lines(&view);
    assert_eq!(
        spawns,
        vec![format!(
            "cd .jigc/worktrees/{SUB_TASK} && jigc workflow fix-task --task {SUB_TASK}"
        )],
        "the fan-out must emit the sub-task's `Spawn:` line on its recorded `fix-task` \
         workflow; got:\n{view}",
    );

    // 3. Provision the worktrees, then run the EMITTED launch line verbatim — its own
    //    `cd` target and its own argv, split off the bytes rather than rebuilt here.
    let out = run_in(root, home, &["milestone", "provision", MILESTONE_ID]);
    assert!(
        out.status.success(),
        "`milestone provision` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let launch = &spawns[0];
    let (cd_part, jigc_part) = launch
        .split_once(" && ")
        .expect("the launch line is `cd <dir> && jigc …`");
    let worktree = root.join(
        cd_part
            .strip_prefix("cd ")
            .expect("the launch line opens with `cd `"),
    );
    let argv: Vec<&str> = jigc_part.split_whitespace().collect();
    assert_eq!(
        argv[0], "jigc",
        "the launch line's command must be `jigc`; got `{jigc_part}`",
    );
    let out = run_in(&worktree, home, &argv[1..]);
    let walk = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "the emitted launch line must compose the fixer's walk; stdout:\n{walk}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // 4. The walk's bytes carry the brief the Fix phase owes its fixer.
    let flat = flowed(&walk);
    assert!(
        flat.contains(FINDING),
        "the walk must restate the finding it is closing; got:\n{walk}",
    );
    assert!(
        flat.contains("Derive the axis"),
        "the walk must carry the axis-derivation rule — an audit reports the sites it hit, \
         not the sites that exist; got:\n{walk}",
    );
    assert!(
        flat.contains("full membership"),
        "the axis rule must demand the class's full membership, not the reported site list; \
         got:\n{walk}",
    );
    assert!(
        flat.contains("`git add` the paths this fix touched, inside THIS worktree"),
        "the walk must carry the first isolation constraint — stage your own paths in your \
         own worktree; got:\n{walk}",
    );
    assert!(
        flat.contains("Never `git commit` and never `jigc task finalize` here"),
        "the walk must carry the second isolation constraint — the milestone's finalize is \
         the only commit boundary; got:\n{walk}",
    );
    assert!(
        flat.contains(&format!(
            "jigc doc show commit:{SUB_TASK} --task {SUB_TASK}"
        )),
        "the walk must carry the staged read-back of the commit doc it just solicited; \
         got:\n{walk}",
    );
    assert!(
        flat.contains("Run your project's own test, lint, and build gate"),
        "the walk must carry the gate the fix has to pass; got:\n{walk}",
    );
}

/// The omitting context, pack-set axis: the **dev pack alone** ships no `fix-task`, so
/// the same `add-task` is refused with the routed `workflow-refs.unknown-workflow`
/// block and nothing is minted — inert and answerable, never a panic and never a
/// half-added sub-task.
#[test]
fn the_dev_pack_alone_refuses_fix_task_and_mints_nothing() {
    let (repo, home) = dev_only_repo("devonly");
    let root = repo.path();
    let home = home.path();

    assert!(
        run_in(root, home, &["milestone", "create", MILESTONE])
            .status
            .success(),
        "`milestone create` must exit 0 on the dev pack alone",
    );
    let out = run_in(
        root,
        home,
        &[
            "milestone",
            "add-task",
            MILESTONE_ID,
            FINDING,
            "--workflow",
            "fix-task",
        ],
    );
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        !out.status.success(),
        "the dev pack alone must refuse `--workflow fix-task`; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("workflow-refs.unknown-workflow") && stderr.contains("route:"),
        "the refusal must carry its code and a route; got:\n{stderr}",
    );
    // Nothing minted: `execute` composes a fan-out with zero `Spawn:` lines.
    let out = run_in(root, home, &["milestone", "execute", MILESTONE_ID]);
    let view = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        spawn_lines(&view).is_empty(),
        "a refused `add-task` must leave the milestone with no sub-task; got:\n{view}",
    );
}

/// The omitting context, surface axis: `fix-task` is `selectable: false`, so the
/// router front door over the composed pair must not offer it — the Fix phase's
/// fan-out spawns it, and a router pick could never land it.
#[test]
fn the_router_front_door_does_not_offer_fix_task() {
    let (repo, home) = composed_repo("router");
    let out = run_in(
        repo.path(),
        home.path(),
        &["start", "close an audit finding"],
    );
    let view = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "the router front door must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !view.contains("fix-task"),
        "a `selectable: false` workflow must stay off the router catalog; got:\n{view}",
    );
}
