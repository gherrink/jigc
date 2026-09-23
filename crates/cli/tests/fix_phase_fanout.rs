//! M50 Increment 11 / T3 — **the Fix phase becomes a fan-out**
//! ([milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md)
//! → §3 and its Orchestration bullet; `completions/artifacts/M50/settle-record.md`
//! → D11).
//!
//! §3 has said *"each confirmed fix is its own agent"* since M38 and *"triage fixes
//! should ride"* the `fan-out`/`join` primitive since M42, while the step the loop
//! actually composes told the fixer to work **strictly serial over the one shared
//! working tree**. The rule that lives in prose the actor must remember to read has
//! now failed three times (M38, M42, M49), which is the doc's own indictment of it.
//!
//! T2 gave the round a unit (`--workflow fix-task`); this suite pins the round
//! itself, on the **emitted bytes of the real binary** over the `[dev ▸ methodology]`
//! pack-set a plain `jigc setup` installs:
//!
//!   * the composed `completion` walk carries the **partition rule** (by projected
//!     write-set), the **shared-artifact exclusion** stated as a rule rather than as
//!     this repo's instances, the **round's command sequence**, and the **serial
//!     fallback** — and no longer carries the retired serial-over-one-tree rule;
//!   * the emitted `jigc milestone add-task … --workflow fix-task` line **runs
//!     verbatim** against a real milestone at exit 0 (arm 2). It is a mechanical
//!     instruction in prose, so nothing else fences it: a placeholder is substituted
//!     and nothing else, and the argv is split off the composed bytes rather than
//!     rebuilt in test code;
//!   * the **whole round** runs from those bytes end to end over a two-partition
//!     fixture (arm 3) — `config set` · `create` · `add-task` × 2 · `provision` ·
//!     `execute` · the fixers on `execute`'s own `Spawn:` lines · `join` ·
//!     `finalize` — landing **one commit per fix**, each owning its own path, and
//!     pinning what the step says about the knob: the write lands uncommitted and
//!     the milestone's finalize carries the `.jigc/config/` layer into its aggregate
//!     commit, so it stays in effect.
//!
//! **Both composing contexts are asserted** (hardening #5 on the axis this step
//! actually has): `fix-gate` is included by `completion` *and* by `increment`, and a
//! fix round is the same shape in both — a rule parked in a shared step and carried
//! by only one of its composers would be half-shipped.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-fix-fanout-{tag}-{}-{:?}",
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

fn git(root: &Path, args: &[&str]) -> String {
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
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::create_dir_all(root.join("src")).expect("mkdir src");
    fs::write(root.join("src/low.rs"), "pub fn low() {}\n").expect("write");
    fs::write(root.join("src/zed.rs"), "pub fn zed() {}\n").expect("write");
    fs::write(root.join("README.md"), "hello\n").expect("write");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
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

fn ok(dir: &Path, home: &Path, args: &[&str]) -> String {
    let out = run_in(dir, home, args);
    assert!(
        out.status.success(),
        "`jigc {}` must exit 0; stdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// A repo a real `jigc setup` installed into — the `[dev ▸ methodology]` pack-set a
/// dogfooding project actually runs.
fn composed_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(&format!("{tag}-repo"));
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"]);
    (repo, home)
}

/// Re-flow a composed body so an assertion may quote a sentence the pack wraps across
/// lines (the `methodology_staging_contract.rs` idiom).
fn flowed(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every bare `jigc …` command line of the composed round, in emitted order — the
/// agent-facing artifact, lifted off the bytes rather than rebuilt here.
fn round_lines(composed: &str) -> Vec<String> {
    composed
        .lines()
        .map(str::trim_end)
        .filter(|l| {
            l.starts_with("jigc config set finalize.fan-out") || l.starts_with("jigc milestone ")
        })
        .map(str::to_owned)
        .collect()
}

/// Split an emitted command line into argv, honouring the double quotes the line
/// itself carries (its `"<the finding>"` argument is quoted by design).
fn shell_split(line: &str) -> Vec<String> {
    let mut argv = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut started = false;
    for ch in line.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                started = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if started {
                    argv.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            c => {
                cur.push(c);
                started = true;
            }
        }
    }
    if started {
        argv.push(cur);
    }
    argv
}

/// Extract every `` Spawn: `<launch-line>` `` directive's backtick-wrapped launch line.
fn spawn_lines(composed: &str) -> Vec<String> {
    composed
        .lines()
        .filter_map(|l| l.strip_prefix("Spawn: `"))
        .filter_map(|rest| rest.strip_suffix('`'))
        .map(str::to_owned)
        .collect()
}

/// The retired rule, quoted from the bytes it stood in until this task.
const RETIRED: &str = "One fixer per finding, strictly serial over the one shared working tree";

const ROUND_TITLE: &str = "Audit fix round";
const ROUND_ID: &str = "audit-fix-round";

/// Arm 1 — the composed walk states the composition, in both contexts that compose it.
#[test]
fn the_composed_fix_phase_carries_the_partition_rule_the_round_and_the_fallback() {
    let (repo, home) = composed_repo("prose");
    // `completion` mints a task, so its read-without-minting is `--preview`;
    // `increment` mints none, so `start --workflow` IS its read. Both are the
    // composed bytes an agent is handed.
    for (workflow, argv) in [
        ("completion", ["workflow", "completion", "--preview"]),
        ("increment", ["start", "--workflow", "increment"]),
    ] {
        let view = ok(repo.path(), home.path(), &argv);
        let flat = flowed(&view);

        assert!(
            !flat.contains(RETIRED),
            "`{workflow}` must no longer carry the retired serial-over-one-tree rule; got:\n{view}",
        );
        assert!(
            flat.contains("Each fixer is its own agent in its own worktree"),
            "`{workflow}` must name the actor and the instrument of a fix round; got:\n{view}",
        );
        assert!(
            flat.contains("Partition the confirmed findings by each fix's projected write-set"),
            "`{workflow}` must carry the partition rule; got:\n{view}",
        );
        assert!(
            flat.contains("A finding spanning two files is one sub-task; two findings landing in one file are one sub-task"),
            "the partition rule must state both directions; got:\n{view}",
        );
        assert!(
            flat.contains(
                "Keep shared artifacts out of the partition and apply them serially after the join"
            ),
            "`{workflow}` must exclude shared artifacts from the fan-out; got:\n{view}",
        );
        assert!(
            flat.contains(
                "A shared artifact is one every fixer would have to edit to land its own change"
            ),
            "the shared-artifact exclusion must ship as a RULE — the pack ships to adopters, \
             so it cannot be this repo's instances or counts; got:\n{view}",
        );
        assert!(
            flat.contains("If a round's write-sets cannot be made disjoint")
                && flat.contains("fall back to running the fixers one at a time"),
            "`{workflow}` must state the serial fallback up front, not leave it to be \
             discovered at a blocked join; got:\n{view}",
        );
        assert!(
            flat.contains("bounded to three rounds")
                && flat.contains("Advisories are surfaced, not gated"),
            "the three-round cap and the advisory rule are kept verbatim; got:\n{view}",
        );

        assert_eq!(
            round_lines(&view),
            vec![
                "jigc config set finalize.fan-out.squash false".to_string(),
                format!("jigc milestone create \"<the fix round>\""),
                "jigc milestone add-task <fix-milestone-id> \"<the finding>\" --workflow fix-task"
                    .to_string(),
                "jigc milestone provision <fix-milestone-id>".to_string(),
                "jigc milestone execute <fix-milestone-id>".to_string(),
                "jigc milestone join <fix-milestone-id>".to_string(),
                "jigc milestone finalize <fix-milestone-id>".to_string(),
            ],
            "`{workflow}` must carry the round's command sequence, in order; got:\n{view}",
        );
    }
}

/// Arm 2 — the done-criterion's named arm: the emitted `add-task` line runs verbatim.
#[test]
fn the_emitted_add_task_line_runs_verbatim_against_a_real_milestone() {
    let (repo, home) = composed_repo("addtask");
    let root = repo.path();
    let home = home.path();

    let view = ok(root, home, &["workflow", "completion", "--preview"]);
    let line = round_lines(&view)
        .into_iter()
        .find(|l| l.contains("add-task"))
        .expect("the composed round must carry an `add-task` line");

    ok(root, home, &["milestone", "create", ROUND_TITLE]);

    // Only the two placeholders are substituted; the rest of the line is run as written.
    let finding = "the leaf boundary truncates slot prose";
    let filled = line
        .replace("<fix-milestone-id>", ROUND_ID)
        .replace("<the finding>", finding);
    let argv = shell_split(&filled);
    assert_eq!(
        argv[0], "jigc",
        "the emitted line's command must be `jigc`; got `{filled}`",
    );
    let rest: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let out = run_in(root, home, &rest);
    assert!(
        out.status.success(),
        "the emitted `add-task` line must run verbatim at exit 0; line:\n{filled}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // It minted a sub-task on the workflow the line names, so `execute` can fan it out.
    let view = ok(root, home, &["milestone", "execute", ROUND_ID]);
    // The `cd` operand is the ABSOLUTE worktree path since M53 (the cwd census, C1-14 /
    // C3-01) — the line is bytes an orchestrator pastes into a shell of unknown cwd — so
    // the assertion is on the tail, which is what this suite is about.
    assert_eq!(
        spawn_lines(&view),
        vec![format!(
            "cd {}/.jigc/worktrees/leaf-boundary-truncates-slot && jigc workflow fix-task --task leaf-boundary-truncates-slot",
            std::fs::canonicalize(root)
                .unwrap_or_else(|_| root.to_path_buf())
                .display(),
        )],
        "the minted sub-task must fan out on `fix-task`; got:\n{view}",
    );
}

/// Arm 3 — the whole round, driven from the emitted bytes over a two-partition
/// fixture, landing one commit per fix; and the knob truth the step states.
#[test]
fn the_emitted_round_runs_end_to_end_and_lands_one_commit_per_fix() {
    let (repo, home) = composed_repo("round");
    let root = repo.path();
    let home = home.path();

    let view = ok(root, home, &["workflow", "completion", "--preview"]);
    let lines = round_lines(&view);
    let run_line = |line: &str, subs: &[(&str, &str)]| -> std::process::Output {
        let mut filled = line.to_string();
        for (from, to) in subs {
            filled = filled.replace(from, to);
        }
        let argv = shell_split(&filled);
        let rest: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
        let out = run_in(root, home, &rest);
        assert!(
            out.status.success(),
            "`{filled}` must exit 0; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        out
    };

    // The two partitions: disjoint projected write-sets, one file each.
    let partitions = [
        (
            "the low cache path truncates",
            "low-cache-path-truncates",
            "src/low.rs",
        ),
        (
            "the zed cache path truncates",
            "zed-cache-path-truncates",
            "src/zed.rs",
        ),
    ];

    run_line(&lines[0], &[]); // config set finalize.fan-out.squash false
    assert_eq!(
        String::from_utf8_lossy(
            &run_in(root, home, &["config", "get", "finalize.fan-out.squash"]).stdout
        )
        .trim()
        .lines()
        .next()
        .unwrap_or_default(),
        "finalize.fan-out.squash = false  (project)",
        "the knob the round opens with must read back set",
    );

    run_line(&lines[1], &[("<the fix round>", ROUND_TITLE)]);
    for (finding, _, _) in partitions {
        run_line(
            &lines[2],
            &[("<fix-milestone-id>", ROUND_ID), ("<the finding>", finding)],
        );
    }
    run_line(&lines[3], &[("<fix-milestone-id>", ROUND_ID)]); // provision
    let executed = run_line(&lines[4], &[("<fix-milestone-id>", ROUND_ID)]); // execute
    let executed = String::from_utf8(executed.stdout).expect("utf-8 stdout");

    // Each fixer runs its own emitted `Spawn:` line, in its own worktree.
    let spawns = spawn_lines(&executed);
    assert_eq!(
        spawns.len(),
        2,
        "one Spawn line per partition; got:\n{executed}"
    );
    for (launch, (_, task, path)) in spawns.iter().zip(partitions) {
        let (cd_part, jigc_part) = launch
            .split_once(" && ")
            .expect("the launch line is `cd <dir> && jigc …`");
        let worktree = root.join(cd_part.strip_prefix("cd ").expect("`cd ` prefix"));
        let argv: Vec<&str> = jigc_part.split_whitespace().collect();
        ok(&worktree, home, &argv[1..]);

        fs::write(worktree.join(path), "pub fn fixed() {}\n").expect("apply the fix");
        git(&worktree, &["add", path]);
        ok(
            &worktree,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "fix",
                "--task",
                task,
            ],
        );
        let summary = worktree.join("summary.txt");
        fs::write(&summary, format!("{path} stops truncating\n")).expect("write summary");
        ok(
            &worktree,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                summary.to_str().expect("utf-8 path"),
                "--task",
                task,
            ],
        );
        fs::remove_file(&summary).expect("the payload file is not part of the fix");
    }

    run_line(&lines[5], &[("<fix-milestone-id>", ROUND_ID)]); // join
    let finalized = run_line(&lines[6], &[("<fix-milestone-id>", ROUND_ID)]); // finalize
    let finalized = String::from_utf8(finalized.stdout).expect("utf-8 stdout");

    // One commit per fix, each owning its own partition's path — read back off git,
    // never off what the ack claims.
    let head = git(root, &["rev-parse", "HEAD"]).trim().to_string();
    let chain: Vec<String> = git(root, &["rev-list", "--reverse", "-n", "3", "HEAD"])
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        chain.len(),
        3,
        "two per-fix commits in task-id order, then the aggregate boundary",
    );
    assert_eq!(chain[2], head, "the aggregate is the boundary's own commit");
    for (sha, (_, _, path)) in chain[..2].iter().zip(partitions) {
        let touched = git(root, &["show", "--name-only", "--format=", sha]);
        assert_eq!(
            touched.split_whitespace().collect::<Vec<_>>(),
            vec![path],
            "each per-fix commit must own exactly its partition's path; \
             finalize said:\n{finalized}",
        );
    }

    // The knob truth the step states: the `config set` write landed uncommitted, and
    // the milestone's own finalize carried the `.jigc/config/` layer into its
    // aggregate commit — so nothing is left behind and the knob stays in effect.
    let aggregate = git(root, &["show", "--name-only", "--format=", &head]);
    assert!(
        aggregate.contains(".jigc/config/manifest.yaml"),
        "the aggregate commit must carry the config layer the knob write left \
         uncommitted; got:\n{aggregate}",
    );
    assert_eq!(
        git(root, &["status", "--porcelain"]),
        "",
        "the round must leave no uncommitted residue",
    );
    assert!(
        String::from_utf8_lossy(
            &run_in(root, home, &["config", "get", "finalize.fan-out.squash"]).stdout
        )
        .contains("finalize.fan-out.squash = false"),
        "the knob is a project setting, not a per-round flag — it stays in effect",
    );
}
