//! **Which checkout a verb's subject is, when the caller stands somewhere else** — the
//! verb half of the M53 cwd-dependence census
//! ([`completions/artifacts/M53/cwd-census.md`], rows C2-02, C2-06, C2-11).
//!
//! The census drove every leaf from four working directories and found the split its §0
//! named: `discover_repo_root` is a bare walk-up, so inside a fan-out worktree it returns
//! **the worktree**, while `repo::jigc_home` returns **the main checkout** the `.jigc/`
//! workbench and the committed doc-store bind to. Three doors read the walk-up where their
//! subject is the store:
//!
//! * **C2-06** — `jigc milestone finalize` composed a mixed pathspec list (two
//!   repo-relative `.jigc/` entries plus the milestone record's path, which could not be
//!   stripped against the worktree and came out **absolute**) and ran it with `current_dir`
//!   = the worktree. Driven, every finalize from inside a provisioned worktree died on a
//!   raw ``git add -- … failed: '…/loss-probe.md' is outside repository at
//!   '…/.jigc/worktrees/<sub>'`` — no code, no route, no `at:`. The milestone boundary was
//!   unreachable from the one cwd jigc's own spawn template puts a sub-agent in.
//! * **C2-11** — the base-mismatch gate read the **standing checkout's** HEAD. A fan-out
//!   worktree is pinned *to the base* by construction, so from inside one the gate could
//!   not fire; only C2-06's git fault stopped the run, by accident. Fixing C2-06 alone
//!   opens this gate, which is why both land in one commit and both are pinned here.
//! * **C2-02** — `jigc task diff <sub>` diffed the checkout you stand in. From the root it
//!   reported the *milestone-record commits* as the sub-task's work and omitted the code
//!   the sub-task had actually staged, at exit 0, on a `--format json`-pinned verb.
//!
//! **What each arm iterates.** These are not a class with a registry behind them: the
//! subject question is asked once per door, and the census enumerated the doors by driving
//! them. So each arm names **one door × the cwd axis** — root, ordinary subdirectory,
//! the sub-task's own worktree, a sibling worktree where one exists — and the assertion is
//! that the door's answer does not move across it. The cwd axis is the set; the doors are
//! the census's enumeration, cited above.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-cwd-subject-{tag}-{}-{:?}",
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

fn git(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    git(repo, &["config", "commit.gpgsign", "false"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    crate::support::mint_project_layer(repo);
    // The `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads
    // (`milestone_record_finalize.rs`'s idiom). Without it no `milestone-record` schema
    // resolves, the boundary's record pathspec is `None`, and the fixture cannot reach the
    // census's own C2-06 symptom: the record path is the pathspec that could not be
    // stripped against the worktree and came out absolute.
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
}

/// Run `jigc <args>` **from `cwd`** with `$HOME = home`. The cwd is the axis under test,
/// so it is a parameter of its own and never defaulted to the repo root.
fn jigc_in(cwd: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

fn stdout(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn both_streams(out: &std::process::Output) -> String {
    format!("{}{}", stdout(out), String::from_utf8_lossy(&out.stderr))
}

/// A provisioned fan-out: `milestone:cache-rework` with two sub-tasks, each worktree
/// holding one staged code file and each area a complete transient commit doc — the state
/// a real boundary finalizes.
struct FanOut {
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
}

impl FanOut {
    fn new(tag: &str) -> Self {
        Self::new_created_in(tag, false)
    }

    /// The same fixture, with `milestone create` typed **in a linked worktree** whose HEAD
    /// differs from the main checkout's — the cell of the M53 post-review-fix review's
    /// MEDIUM 4. Everything after the create is byte-identical to `new`, so the arm that
    /// uses it compares a milestone born away against the one every other arm drives.
    ///
    /// Main is advanced by a real (non-record) commit first: cut from main's own HEAD the two
    /// checkouts answer the same sha, which is the shape under which the defect is invisible
    /// — and is why it shipped.
    fn new_created_in(tag: &str, born_away: bool) -> Self {
        let root = TempDir::new(tag);
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo dir");
        init_repo(&repo);
        let home = TempDir::new(&format!("{tag}-home"));

        let linked = root.path().join("feat");
        let create_in = if born_away {
            git(
                &repo,
                &[
                    "worktree",
                    "add",
                    "-q",
                    "-b",
                    "feat",
                    linked.to_str().expect("worktree path is UTF-8"),
                ],
            );
            fs::write(repo.join("main-only.txt"), "main moved\n").expect("write main-only");
            git(&repo, &["add", "main-only.txt"]);
            git(&repo, &["commit", "-q", "-m", "advance main"]);
            assert_ne!(
                git(&repo, &["rev-parse", "HEAD"]),
                git(&linked, &["rev-parse", "HEAD"]),
                "the fixture must put the two checkouts on different commits",
            );
            linked.clone()
        } else {
            repo.clone()
        };

        assert_ok(
            &jigc_in(
                &create_in,
                home.path(),
                &["milestone", "create", "Cache rework"],
            ),
            "`jigc milestone create`",
        );
        for intent in ["Area one", "Area two"] {
            assert_ok(
                &jigc_in(
                    &repo,
                    home.path(),
                    &["milestone", "add-task", "cache-rework", intent],
                ),
                "`jigc milestone add-task`",
            );
        }
        assert_ok(
            &jigc_in(
                &repo,
                home.path(),
                &["milestone", "provision", "cache-rework"],
            ),
            "`jigc milestone provision`",
        );

        let me = Self {
            _root: root,
            home,
            repo,
        };
        for sub in ["area-one", "area-two"] {
            me.author_subtask(sub);
        }
        me
    }

    fn worktree(&self, sub: &str) -> PathBuf {
        self.repo.join(".jigc").join("worktrees").join(sub)
    }

    /// Stage one code file in the sub-task's worktree and fill its transient commit doc,
    /// driving the binary from **inside that worktree** — exactly as the shipped spawn
    /// template puts a sub-agent there.
    fn author_subtask(&self, sub: &str) {
        let wt = self.worktree(sub);
        fs::write(wt.join(format!("{sub}.txt")), "work\n").expect("write worktree code");
        git(&wt, &["add", &format!("{sub}.txt")]);
        // Enter the sub-task's working area — the compose that provisions its transient
        // `commit` doc. Run from the worktree, which is the cwd the shipped spawn template
        // puts a sub-agent in (and, per C2-08, the only cwd this door accepts today).
        assert_ok(
            &jigc_in(
                &wt,
                self.home.path(),
                &["workflow", "sub-task", "--task", sub],
            ),
            "`jigc workflow sub-task --task <sub>`",
        );
        for (address, value) in [
            (format!("commit:{sub}#type"), "feat".to_string()),
            (format!("commit:{sub}#scope"), "cache".to_string()),
        ] {
            assert_ok(
                &jigc_in(
                    &wt,
                    self.home.path(),
                    &[
                        "doc",
                        "set-field",
                        &address,
                        "--value",
                        &value,
                        "--task",
                        sub,
                    ],
                ),
                "`jigc doc set-field`",
            );
        }
        for (slot, value) in [
            ("summary", format!("land {sub}")),
            ("body", format!("The {sub} half of the rework.")),
        ] {
            let payload = self.home.path().join(format!("{sub}-{slot}.txt"));
            fs::write(&payload, format!("{value}\n")).expect("write the slot payload");
            assert_ok(
                &jigc_in(
                    &wt,
                    self.home.path(),
                    &[
                        "doc",
                        "set-slot",
                        &format!("commit:{sub}#{slot}"),
                        "--from-file",
                        payload.to_str().expect("payload path is UTF-8"),
                        "--task",
                        sub,
                    ],
                ),
                "`jigc doc set-slot`",
            );
        }
    }

    fn jigc(&self, cwd: &Path, args: &[&str]) -> std::process::Output {
        jigc_in(cwd, self.home.path(), args)
    }
}

// ---------------------------------------------------------------------------------------
// C2-06 — the milestone boundary answers the same from every cwd.
// ---------------------------------------------------------------------------------------

/// The cwd axis for a boundary run: the root, an ordinary subdirectory, the sub-task's own
/// worktree, and a *sibling* sub-task's worktree. Each arm gets its own fixture, because a
/// landed boundary is terminal.
#[test]
fn milestone_finalize_lands_from_every_cwd_in_the_repository() {
    for (tag, cwd_of) in [
        (
            "root",
            (|f: &FanOut| f.repo.clone()) as fn(&FanOut) -> PathBuf,
        ),
        ("subdir", |f: &FanOut| {
            let deep = f.repo.join("docs").join("deep");
            fs::create_dir_all(&deep).expect("mk docs/deep");
            deep
        }),
        ("own-worktree", |f: &FanOut| f.worktree("area-one")),
        ("sibling-worktree", |f: &FanOut| f.worktree("area-two")),
    ] {
        let fan = FanOut::new(tag);
        let cwd = cwd_of(&fan);
        let out = fan.jigc(&cwd, &["milestone", "finalize", "cache-rework"]);
        assert!(
            out.status.success(),
            "`jigc milestone finalize` from {tag} must land; got {:?}\n{}",
            out.status,
            both_streams(&out),
        );
        // The subject was the store, not the standing checkout: the boundary commit is on
        // the MAIN checkout's HEAD and carries both sub-tasks' code.
        let landed = git(&fan.repo, &["diff", "--name-only", "HEAD~1", "HEAD"]);
        for sub in ["area-one", "area-two"] {
            assert!(
                landed.contains(&format!("{sub}.txt")),
                "the boundary run from {tag} must land {sub}.txt; landed:\n{landed}",
            );
        }
    }
}

// ---------------------------------------------------------------------------------------
// C2-11 — the base-mismatch gate's subject is the milestone's store, not the cwd's HEAD.
// ---------------------------------------------------------------------------------------

/// Advance the main checkout past the pinned base with an **external** (non-record) commit,
/// then finalize from inside a fan-out worktree — whose own HEAD still *is* the base. The
/// gate must refuse identically from both cwds.
#[test]
fn the_base_mismatch_gate_fires_from_inside_a_fan_out_worktree() {
    let expected = "finalize.base-mismatch";
    for (tag, cwd_of) in [
        (
            "root",
            (|f: &FanOut| f.repo.clone()) as fn(&FanOut) -> PathBuf,
        ),
        ("own-worktree", |f: &FanOut| f.worktree("area-one")),
    ] {
        let fan = FanOut::new(&format!("mismatch-{tag}"));
        // An external commit on the main checkout — not a milestone-record path, so the
        // record-only-advance carve-out does not apply.
        fs::write(fan.repo.join("outside.txt"), "drift\n").expect("write outside.txt");
        git(&fan.repo, &["add", "outside.txt"]);
        git(&fan.repo, &["commit", "-q", "-m", "chore: external drift"]);

        let cwd = cwd_of(&fan);
        let out = fan.jigc(&cwd, &["milestone", "finalize", "cache-rework"]);
        let seen = both_streams(&out);
        assert!(
            seen.contains(expected),
            "`jigc milestone finalize` from {tag} must refuse with `{expected}`; \
             got {:?}\n{seen}",
            out.status,
        );
        assert!(
            !out.status.success(),
            "a base-mismatched boundary from {tag} must not exit 0\n{seen}",
        );
    }
}

// ---------------------------------------------------------------------------------------
// C2-02 — a sub-task's diff subject is its provisioned worktree, from every cwd.
// ---------------------------------------------------------------------------------------

#[test]
fn task_diff_of_a_sub_task_reads_its_worktree_from_every_cwd() {
    let fan = FanOut::new("diff");
    let deep = fan.repo.join("docs").join("deep");
    fs::create_dir_all(&deep).expect("mk docs/deep");

    for (tag, cwd) in [
        ("root", fan.repo.clone()),
        ("subdir", deep.clone()),
        ("own-worktree", fan.worktree("area-one")),
        ("sibling-worktree", fan.worktree("area-two")),
    ] {
        for format in [&[][..], &["--format", "json"][..]] {
            let mut argv = vec!["task", "diff", "area-one"];
            argv.extend_from_slice(format);
            let out = fan.jigc(&cwd, &argv);
            assert_ok(&out, "`jigc task diff area-one`");
            let seen = stdout(&out);
            assert!(
                seen.contains("area-one.txt"),
                "`jigc task diff area-one{}` from {tag} must report the sub-task's own \
                 staged code; got:\n{seen}",
                if format.is_empty() {
                    ""
                } else {
                    " --format json"
                },
            );
            assert!(
                !seen.contains("milestone-records"),
                "`jigc task diff area-one{}` from {tag} must not report the milestone \
                 record commits as the sub-task's work; got:\n{seen}",
                if format.is_empty() {
                    ""
                } else {
                    " --format json"
                },
            );
            assert!(
                !seen.contains("area-two.txt"),
                "`jigc task diff area-one{}` from {tag} must not report a SIBLING \
                 sub-task's code; got:\n{seen}",
                if format.is_empty() {
                    ""
                } else {
                    " --format json"
                },
            );
        }
    }

    // The pinned `task-diff` envelope's key set does not move — the values became true,
    // the shape did not (`design/command-output-contract.md`; `ENVELOPE_ARMS`).
    let out = fan.jigc(&fan.repo, &["task", "diff", "area-one", "--format", "json"]);
    assert_ok(&out, "`jigc task diff --format json`");
    let value: serde_json::Value =
        serde_json::from_str(&stdout(&out)).expect("the envelope parses");
    let object = value.as_object().expect("the envelope is an object");
    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["base", "code_diff", "findings", "op", "staged_docs", "task"],
        "the pinned `task-diff` envelope's key set must not move",
    );
}

// ---------------------------------------------------------------------------------------
// The door that SETS what the boundary gates on binds the same root the boundary reads.
// ---------------------------------------------------------------------------------------

/// **A milestone created from a linked worktree must be finalizable** (M53 post-review-fix
/// review, MEDIUM 4).
///
/// `f919ea95` moved `jigc milestone finalize`'s subject to `jigc_home` — it reads HEAD with
/// `git_head(&jigc_home)` and probes the index with `git_staged_snapshot(&jigc_home)` — and
/// left `milestone create` reading the **standing** checkout for both. Driven on
/// `committed-singletons` with a branch-attached worktree `feat` and main advanced by a
/// real commit: `create` pinned `feat`'s HEAD, landed its record commit on **main**, and the
/// milestone's **first** `milestone finalize` refused `finalize.base-mismatch` at exit 3
/// from every cwd, with a route asking the operator to rewind the main checkout's HEAD onto
/// another branch's commit. The milestone was born un-finalizable.
///
/// This arm is the pair, not either half: it creates from the linked worktree and finalizes,
/// and the assertion is that the gate the create fed does not refuse it. The cwd axis is the
/// set — create from the worktree, finalize from the root — which is the same axis every
/// other arm in this suite iterates.
#[test]
fn a_milestone_created_in_a_linked_worktree_finalizes() {
    let fan = FanOut::new_created_in("born-away", true);
    let linked = fan
        .repo
        .parent()
        .expect("the repo has a parent")
        .join("feat");

    // The pin the boundary will gate on is `jigc_home`'s HEAD, not the checkout the create
    // was typed in. Before the fix it was the latter, and the first finalize refused
    // `finalize.base-mismatch` at exit 3 from every cwd.
    let pin = fs::read_to_string(
        fan.repo
            .join(".jigc")
            .join("milestones")
            .join("cache-rework")
            .join("base.json"),
    )
    .expect("the milestone's base pin is readable");
    assert!(
        !pin.contains(&git(&linked, &["rev-parse", "HEAD"])),
        "the base pin must not be the STANDING checkout's HEAD; pin was:\n{pin}",
    );

    let landed = jigc_in(
        &fan.repo,
        fan.home.path(),
        &["milestone", "finalize", "cache-rework"],
    );
    let seen = both_streams(&landed);
    assert!(
        !seen.contains("finalize.base-mismatch"),
        "a milestone created in a linked worktree must not be born un-finalizable; got:\n{seen}",
    );
    assert_ok(&landed, "`jigc milestone finalize`");
}

// ---------------------------------------------------------------------------------------
// C2-09 — `jigc task finalize` from an ordinary linked worktree SAYS where it committed.
// ---------------------------------------------------------------------------------------

/// The decision this pins is *keep it, say it* (M53 — the cwd census, row C2-09;
/// `crate::render::CommitSite`). A plain task's code lives in the working tree the agent is
/// editing, so the commit target is the checkout the command was typed in — and the whole
/// rest of the door already agrees with that: the base pin, the posture guard, the carryover
/// snapshot and `task diff`'s plain-task subject are all the standing checkout. Committing
/// to the main checkout instead would commit nothing, or the wrong tree.
///
/// What was wrong was that **nothing said so**: the task was minted and its roster read from
/// the main checkout, the ack named a sha, and the worktree's branch had moved while `main`
/// had not. So the target is named on both surfaces, and named in the same sentence.
#[test]
fn task_finalize_names_the_checkout_it_commits_in_when_that_is_not_the_workbench_home() {
    let root = TempDir::new("c209");
    let repo = root.path().join("repo");
    fs::create_dir_all(&repo).expect("mk repo dir");
    init_repo(&repo);
    let home = TempDir::new("c209-home");
    let linked = root.path().join("feat");

    git(
        &repo,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "feat",
            linked.to_str().expect("worktree path is UTF-8"),
        ],
    );

    // One plain task per checkout, authored and finalized from that checkout.
    for (tag, cwd, task, elsewhere) in [
        ("linked worktree", linked.clone(), "do-a-thing", true),
        ("main checkout", repo.clone(), "second-thing", false),
    ] {
        let intent = if elsewhere {
            "do a thing"
        } else {
            "second thing"
        };
        assert_ok(
            &jigc_in(
                &cwd,
                home.path(),
                &["start", intent, "--workflow", "quick-fix"],
            ),
            "`jigc start --workflow quick-fix`",
        );
        fs::write(cwd.join(format!("{task}.txt")), "change\n").expect("write code");
        git(&cwd, &["add", &format!("{task}.txt")]);
        for (address, value) in [
            (format!("commit:{task}#type"), "fix"),
            (format!("commit:{task}#scope"), "core"),
        ] {
            assert_ok(
                &jigc_in(
                    &cwd,
                    home.path(),
                    &[
                        "doc",
                        "set-field",
                        &address,
                        "--value",
                        value,
                        "--task",
                        task,
                    ],
                ),
                "`jigc doc set-field`",
            );
        }
        for (slot, value) in [("summary", "do the thing"), ("body", "Body prose.")] {
            let payload = home.path().join(format!("{task}-{slot}.txt"));
            fs::write(&payload, format!("{value}\n")).expect("write the slot payload");
            assert_ok(
                &jigc_in(
                    &cwd,
                    home.path(),
                    &[
                        "doc",
                        "set-slot",
                        &format!("commit:{task}#{slot}"),
                        "--from-file",
                        payload.to_str().expect("payload path is UTF-8"),
                        "--task",
                        task,
                    ],
                ),
                "`jigc doc set-slot`",
            );
        }

        let forecast = jigc_in(&cwd, home.path(), &["task", "finalize", task, "--dry-run"]);
        assert_ok(&forecast, "`jigc task finalize --dry-run`");
        // The pinned forecast envelope does NOT grow a key for the commit site — the
        // additive-key window closed at M48, so this is a text-only statement by declared
        // bound (`crate::render::CommitSite`). Asserted on the arm that HAS a site to name.
        let envelope = jigc_in(
            &cwd,
            home.path(),
            &["task", "finalize", task, "--dry-run", "--format", "json"],
        );
        assert_ok(&envelope, "`jigc task finalize --dry-run --format json`");
        let value: serde_json::Value =
            serde_json::from_str(&stdout(&envelope)).expect("the forecast envelope parses");
        let mut keys: Vec<&str> = value
            .as_object()
            .expect("the forecast envelope is an object")
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            ["dry_run", "findings", "left_out", "manifest", "subject"],
            "the pinned `finalize --dry-run` envelope's key set must not move, run from \
             the {tag}",
        );
        let landed = jigc_in(&cwd, home.path(), &["task", "finalize", task]);
        assert_ok(&landed, "`jigc task finalize`");

        for (surface, seen, tense) in [
            (
                "--dry-run",
                stdout(&forecast),
                "would commit in the linked worktree at",
            ),
            (
                "the landed ack",
                stdout(&landed),
                "committed in the linked worktree at",
            ),
        ] {
            assert_eq!(
                seen.contains(tense),
                elsewhere,
                "{surface}, run from the {tag}, must {} name the commit site; got:\n{seen}",
                if elsewhere { "" } else { "not" },
            );
            if elsewhere {
                assert!(
                    seen.contains("on branch `feat`"),
                    "{surface} must name the branch it advances; got:\n{seen}",
                );
            }
        }
    }

    // The behaviour itself is unchanged and that is the decision: each commit landed on the
    // branch of the checkout it was typed in.
    assert!(
        git(&linked, &["log", "-1", "--pretty=format:%s"]).contains("do the thing"),
        "the linked worktree's own branch carries its commit",
    );
}

// ---------------------------------------------------------------------------------------
// C2-08 + C3-01 — the one `cd` jigc emits runs from anywhere, and the refusal that sends an
// agent to that same cwd names it the same way.
// ---------------------------------------------------------------------------------------

/// The `Spawn:` line is the highest-leverage line in the product — the orchestrator is told
/// to copy-run it, and it is the entry to the only composed authoring path that *requires*
/// the worktree cwd. Spelled `cd .jigc/worktrees/<id>` it ran from the repository root and
/// nowhere else (`completions/artifacts/M53/cwd-census.md` → C1-14 / C3-01, driven: `cd: no
/// such file or directory` from an ordinary subdirectory **and** from a sibling sub-task's
/// worktree).
///
/// The arm runs the **emitted bytes verbatim** through a real shell rather than a
/// reconstruction (`implementation/increment-workflow.md` → Validation hardening).
#[test]
fn the_spawn_line_runs_verbatim_from_every_cwd() {
    let fan = FanOut::new("spawn");
    let deep = fan.repo.join("docs").join("deep");
    fs::create_dir_all(&deep).expect("mk docs/deep");

    let mut seen: Vec<String> = Vec::new();
    for (tag, cwd) in [
        ("root", fan.repo.clone()),
        ("subdir", deep.clone()),
        ("sibling-worktree", fan.worktree("area-two")),
    ] {
        let out = fan.jigc(&cwd, &["milestone", "execute", "cache-rework"]);
        assert_ok(&out, "`jigc milestone execute`");
        let spawn = stdout(&out)
            .lines()
            .find(|line| line.starts_with("Spawn:") && line.contains("--task area-one"))
            .unwrap_or_else(|| panic!("no `Spawn:` line for area-one, run from {tag}"))
            .to_string();
        seen.push(spawn.clone());

        // The emitted bytes, run as an agent would paste them — split by a real shell.
        let command = spawn
            .trim_start_matches("Spawn:")
            .trim()
            .trim_matches('`')
            .to_string();
        let argv = crate::support::shell_words(&command, &cwd, fan.home.path());
        let ran = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .current_dir(&cwd)
            .env("HOME", fan.home.path())
            .env("PATH", {
                let bin = Path::new(env!("CARGO_BIN_EXE_jigc"))
                    .parent()
                    .expect("the test binary has a parent dir");
                format!(
                    "{}:{}",
                    bin.display(),
                    std::env::var("PATH").unwrap_or_default()
                )
            })
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the emitted spawn line");
        assert!(
            ran.status.success(),
            "the emitted `Spawn:` line must run verbatim from {tag}; argv was {argv:?}\n\
             stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&ran.stdout),
            String::from_utf8_lossy(&ran.stderr),
        );
    }

    // One line, not three: the directive does not depend on where `execute` was run.
    assert!(
        seen.windows(2).all(|pair| pair[0] == pair[1]),
        "the `Spawn:` line must be byte-identical from every cwd; got:\n{}",
        seen.join("\n"),
    );
    assert!(
        seen[0].contains(&format!(
            "cd {}",
            fan.worktree("area-one")
                .canonicalize()
                .expect("the worktree canonicalizes")
                .display()
        )),
        "the `cd` operand must be the absolute worktree path; got:\n{}",
        seen[0],
    );
}

/// **The second question an absolute raises: does it survive a shell?** (M53
/// post-review-fix review, HIGH 1.) Repo-relative, the `cd` operand was
/// `.jigc/worktrees/<slug>` — shell-inert by construction, so no fixture ever needed a
/// hostile path. Absolute, the operand is the *caller's* filesystem, and `std::env::temp_dir()`
/// has no space in it, so the only cell the arm above drives is the canonical one.
///
/// This arm is that arm's input axis: one repository whose path contains a space, the emitted
/// bytes run verbatim through a real shell, and the `cd` operand asserted to be a **single**
/// shell word. Driven before the fix, the line split at the space and the sub-agent could not
/// enter its worktree at all (`sh: cd: /…/jigc: No such file or directory`, rc=1).
///
/// It also pins the pairing `a8318211`'s commit message claims and no test held: the refusal
/// that sends an agent to that directory (`crate::blanket_base_pin_refusal`) and the `Spawn:`
/// line must render **the same bytes** for it. They did not — one quoted, one did not — and
/// the disagreement was invisible on every unspaced root.
#[test]
fn the_spawn_line_and_its_refusal_survive_a_repository_path_with_a_space() {
    // The space is in the fixture's own directory name, so every path below inherits it.
    let fan = FanOut::new("spawn under a space");
    assert!(
        fan.repo.to_string_lossy().contains(' '),
        "the fixture must actually carry a space; got {:?}",
        fan.repo,
    );
    let deep = fan.repo.join("docs").join("deep");
    fs::create_dir_all(&deep).expect("mk docs/deep");
    let absolute = fan
        .worktree("area-one")
        .canonicalize()
        .expect("the worktree canonicalizes")
        .display()
        .to_string();

    let out = fan.jigc(&fan.repo, &["milestone", "execute", "cache-rework"]);
    assert_ok(&out, "`jigc milestone execute`");
    let spawn = stdout(&out)
        .lines()
        .find(|line| line.starts_with("Spawn:") && line.contains("--task area-one"))
        .expect("a `Spawn:` line for area-one")
        .to_string();
    let command = spawn
        .trim_start_matches("Spawn:")
        .trim()
        .trim_matches('`')
        .to_string();

    // The operand is ONE word to a real shell — the assertion the unspaced arm cannot make,
    // because there every emitted spelling is one word.
    let argv = crate::support::shell_words(&command, &fan.repo, fan.home.path());
    assert_eq!(
        argv.get(1).map(String::as_str),
        Some(absolute.as_str()),
        "the `cd` operand must survive the shell as one word; argv was {argv:?}\nline: {spawn}",
    );

    let ran = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .current_dir(&fan.repo)
        .env("HOME", fan.home.path())
        .env("PATH", {
            let bin = Path::new(env!("CARGO_BIN_EXE_jigc"))
                .parent()
                .expect("the test binary has a parent dir");
            format!(
                "{}:{}",
                bin.display(),
                std::env::var("PATH").unwrap_or_default()
            )
        })
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the emitted spawn line");
    assert!(
        ran.status.success(),
        "the emitted `Spawn:` line must run verbatim under a spaced repository path; argv was \
         {argv:?}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr),
    );

    // One directory, one spelling: the refusal names the `cd` exactly as the spawn line does.
    let cd_fragment = command
        .split(" && ")
        .next()
        .expect("the spawn line leads with its `cd`")
        .to_string();
    let refusal = both_streams(&fan.jigc(&fan.repo, &["start", "--task", "area-one"]));
    assert!(
        refusal.contains(&cd_fragment),
        "the refusal must name the identical `{cd_fragment}`; got:\n{refusal}",
    );
}

/// The refusal an agent meets when it runs a sub-task's door from the shared checkout is the
/// other half of the same fact: it sends the reader to the worktree, so it names the same
/// absolute the `Spawn:` line does — and, while the worktree is already there, it does not
/// lead with `jigc milestone provision`, which in that state changes nothing
/// (`completions/artifacts/M53/cwd-census.md` → C2-08).
#[test]
fn the_requires_worktree_refusal_names_the_cd_and_drops_the_no_op_clause() {
    let fan = FanOut::new("requires");
    let deep = fan.repo.join("docs").join("deep");
    fs::create_dir_all(&deep).expect("mk docs/deep");
    let absolute = fan
        .worktree("area-one")
        .canonicalize()
        .expect("the worktree canonicalizes")
        .display()
        .to_string();

    // (a) provisioned — the ordinary state an agent meets this refusal in.
    for (tag, cwd) in [("root", fan.repo.clone()), ("subdir", deep.clone())] {
        for argv in [
            vec!["start", "--task", "area-one"],
            vec!["workflow", "sub-task", "--task", "area-one"],
        ] {
            let out = fan.jigc(&cwd, &argv);
            let seen = both_streams(&out);
            assert!(
                !out.status.success(),
                "{argv:?} from {tag} must refuse\n{seen}"
            );
            assert!(
                seen.contains(&format!("cd {absolute}")),
                "{argv:?} from {tag} must name the absolute `cd`; got:\n{seen}",
            );
            assert!(
                !seen.contains("milestone provision"),
                "{argv:?} from {tag} must not lead with a provision that would do nothing \
                 — the worktree is already there; got:\n{seen}",
            );
        }
    }

    // (b) un-provisioned — the state where provisioning IS the act that resolves it.
    git(
        &fan.repo,
        &["worktree", "remove", "--force", ".jigc/worktrees/area-one"],
    );
    let out = fan.jigc(&fan.repo, &["start", "--task", "area-one"]);
    let seen = both_streams(&out);
    assert!(
        !out.status.success(),
        "the un-provisioned arm must refuse\n{seen}"
    );
    assert!(
        seen.contains("jigc milestone provision cache-rework"),
        "with no worktree, the route must name the door that cuts one; got:\n{seen}",
    );
    assert!(
        seen.contains(&format!("cd {absolute}")),
        "and it must still end on the absolute `cd`; got:\n{seen}",
    );
}

// ---------------------------------------------------------------------------------------
// LOW 5 / LOW 10 — the two install doors act on jigc_home from every cwd.
// ---------------------------------------------------------------------------------------

/// A git repository with **no jigc install at all** plus a linked worktree `feat` — the
/// only state `jigc setup` can be probed from (`dev/jigc-rig`'s `bare`), and the one
/// `init_repo` cannot give, since it mints the project layer directly.
struct Bare {
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
    linked: PathBuf,
}

impl Bare {
    fn new(tag: &str) -> Self {
        let root = TempDir::new(tag);
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo dir");
        git(&repo, &["init", "-q"]);
        git(&repo, &["config", "user.email", "test@example.com"]);
        git(&repo, &["config", "user.name", "Test"]);
        git(&repo, &["config", "commit.gpgsign", "false"]);
        fs::create_dir_all(repo.join("docs").join("deep")).expect("mk docs/deep");
        fs::write(repo.join("docs").join("deep").join("a.txt"), "x\n").expect("write a.txt");
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-q", "-m", "initial"]);
        let linked = root.path().join("feat");
        git(
            &repo,
            &[
                "worktree",
                "add",
                "-q",
                "-b",
                "feat",
                linked.to_str().expect("worktree path is UTF-8"),
            ],
        );
        let home = TempDir::new(&format!("{tag}-home"));
        Bare {
            _root: root,
            home,
            repo,
            linked,
        }
    }

    fn jigc(&self, cwd: &Path, args: &[&str]) -> std::process::Output {
        jigc_in(cwd, self.home.path(), args)
    }

    /// `jigc setup` at the root, then a milestone with one sub-task, provisioned — the
    /// fan-out cell's fixture. Returns that worktree's path.
    fn with_fan_out(&self) -> PathBuf {
        assert_ok(&self.jigc(&self.repo, &["setup"]), "`jigc setup`");
        assert_ok(
            &self.jigc(&self.repo, &["milestone", "create", "Cache rework"]),
            "`jigc milestone create`",
        );
        assert_ok(
            &self.jigc(
                &self.repo,
                &["milestone", "add-task", "cache-rework", "Area one"],
            ),
            "`jigc milestone add-task`",
        );
        assert_ok(
            &self.jigc(&self.repo, &["milestone", "provision", "cache-rework"]),
            "`jigc milestone provision`",
        );
        self.repo.join(".jigc").join("worktrees").join("area-one")
    }
}

/// The **discriminator** for *which checkout an install landed in*: `.jigc/state/` is
/// gitignored, so no checkout ever receives it from git — only the door that wrote it has
/// one. `.jigc/config/` and `.jigc/AGENT.md` are tracked and therefore check out into every
/// worktree, which is exactly why they cannot answer this question.
fn wrote_an_install(checkout: &Path) -> bool {
    checkout.join(".jigc").join("state").is_dir()
}

/// **LOW 5** — `jigc setup` installs at jigc_home from every cwd.
///
/// The cwd axis, as every arm in this suite iterates it: the root, an ordinary
/// subdirectory, a linked worktree, and a fan-out worktree. Driven red at `353029ce`: from
/// the linked worktree the door exited 0 and wrote a worktree-local
/// `.jigc/{AGENT.md,config,state,version}` **that nothing reads** — the pack loader moved
/// to jigc_home at `f919ea95` and `jigc config set` writes there — leaving the main
/// checkout with no install at all.
///
/// The first three cells share one fixture: `setup` is idempotent, and the assertion is
/// about *where* it writes, not about being the first install. The fan-out cell needs a
/// provisioned worktree, so it brings its own.
#[test]
fn setup_installs_at_the_workbench_home_from_every_cwd() {
    let bare = Bare::new("setup-home");
    let deep = bare.repo.join("docs").join("deep");

    for (tag, cwd) in [
        ("root", bare.repo.clone()),
        ("subdir", deep.clone()),
        ("linked worktree", bare.linked.clone()),
    ] {
        let out = bare.jigc(&cwd, &["setup"]);
        assert_ok(&out, &format!("`jigc setup` from the {tag}"));
        assert!(
            wrote_an_install(&bare.repo),
            "`jigc setup` from the {tag} must install at jigc_home",
        );
        assert!(
            !wrote_an_install(&bare.linked),
            "`jigc setup` from the {tag} must write no install into the linked worktree \
             (LOW 5 — the layer nothing reads)",
        );
    }

    // …and from a linked worktree it SAYS where, because the reader is standing elsewhere.
    let seen = stdout(&bare.jigc(&bare.linked, &["setup"]));
    let home = bare
        .repo
        .canonicalize()
        .expect("the repo canonicalizes")
        .display()
        .to_string();
    assert!(
        seen.contains(&home) && seen.contains("not the worktree you are standing in"),
        "from a linked worktree the ack must name the main checkout it installed at; \
         got:\n{seen}",
    );
    // The ordinary cell says nothing — a line always true is never news.
    let at_root = stdout(&bare.jigc(&bare.repo, &["setup"]));
    assert!(
        !at_root.contains("not the worktree you are standing in"),
        "the same-checkout cell must not print the site line; got:\n{at_root}",
    );

    // The fan-out cell: a sub-agent re-running `setup` inside its own worktree.
    let fan = Bare::new("setup-home-fanout");
    let worktree = fan.with_fan_out();
    let out = fan.jigc(&worktree, &["setup"]);
    assert_ok(&out, "`jigc setup` from a fan-out worktree");
    assert!(
        wrote_an_install(&fan.repo) && !wrote_an_install(&worktree),
        "`jigc setup` from a fan-out worktree must install at jigc_home, not below it",
    );
    assert!(
        stdout(&out).contains("not the worktree you are standing in"),
        "and it must say so; got:\n{}",
        stdout(&out),
    );
}

/// **LOW 10 / C2-07** — `jigc uninstall` removes jigc_home's install from every cwd.
///
/// Driven red at `353029ce` from a fan-out worktree: exit 0, *"repo-local install
/// removed"*, seven removal lines — and the repository-wide `.git/hooks/pre-commit` (the
/// **only** copy, shared by every checkout) gone, while the main checkout's `.jigc/`, its
/// `.claude/skills/jigc/SKILL.md` and its `CLAUDE.md` preload line all stood. A destroying
/// door reporting a completion it did not perform.
///
/// One fixture per cell: a teardown is terminal.
#[test]
fn uninstall_removes_the_workbench_home_install_from_every_cwd() {
    for cell in ["root", "subdir", "linked worktree", "fan-out worktree"] {
        let bare = Bare::new(&format!("uninstall-home-{}", cell.replace(' ', "-")));
        let worktree = if cell == "fan-out worktree" {
            Some(bare.with_fan_out())
        } else {
            assert_ok(&bare.jigc(&bare.repo, &["setup"]), "`jigc setup`");
            None
        };
        let cwd = match cell {
            "root" => bare.repo.clone(),
            "subdir" => bare.repo.join("docs").join("deep"),
            "linked worktree" => bare.linked.clone(),
            _ => worktree.expect("the fan-out cell built one"),
        };

        // The install is really there before the teardown — otherwise a green assertion
        // below would be a fixture fact, not a behaviour.
        assert!(
            wrote_an_install(&bare.repo),
            "{cell}: fixture must be set up"
        );
        let guide = bare.repo.join(".claude").join("skills").join("jigc");
        let hook = bare.repo.join(".git").join("hooks").join("pre-commit");
        assert!(guide.join("SKILL.md").is_file() && hook.is_file(), "{cell}");

        let out = bare.jigc(&cwd, &["uninstall"]);
        assert_ok(&out, &format!("`jigc uninstall` from the {cell}"));
        assert!(
            !bare.repo.join(".jigc").exists(),
            "{cell}: the teardown must remove jigc_home's `.jigc/`",
        );
        assert!(
            !guide.join("SKILL.md").exists(),
            "{cell}: …and jigc_home's guide artifact",
        );
        assert!(
            !hook.exists(),
            "{cell}: …and the repository-wide pre-commit hook it installed",
        );
        // …and jigc_home's preload line — the third artifact the LOW 10 cell left standing.
        // An empty `CLAUDE.md` setup itself created goes with it, so *absent* is a pass.
        assert!(
            !fs::read_to_string(bare.repo.join("CLAUDE.md"))
                .unwrap_or_default()
                .contains("@.jigc/AGENT.md"),
            "{cell}: …and jigc_home's preload line",
        );
        // And when the reader is standing somewhere else, it says which install it took.
        let seen = stdout(&out);
        let elsewhere = cell == "linked worktree" || cell == "fan-out worktree";
        assert_eq!(
            seen.contains("the main checkout this repository's jigc install and `.jigc/`"),
            elsewhere,
            "{cell}: the site line is printed iff jigc_home is not the standing checkout; \
             got:\n{seen}",
        );
        // **The tail branches on whether the standing checkout survived** (the confirmation
        // pass, MEDIUM 2). A linked worktree sits outside `.jigc/` and is genuinely not the
        // subject; a fan-out worktree sits *below* it and went with the workbench, so the
        // one sentence that mentions the standing worktree must not assert the opposite of
        // what just happened.
        let standing_removed = cell == "fan-out worktree";
        assert_eq!(
            seen.contains("which this removed"),
            standing_removed,
            "{cell}: the ack must name the standing worktree as removed iff it was; \
             got:\n{seen}",
        );
        assert_eq!(
            seen.contains("not the worktree you are standing in"),
            elsewhere && !standing_removed,
            "{cell}: the untouched-worktree clause is for the cell where it is true; \
             got:\n{seen}",
        );
        if standing_removed {
            assert!(
                !cwd.exists(),
                "{cell}: the fixture must actually have taken the standing worktree — \
                 otherwise the sentence under test would be the false one",
            );
            // **And git's admin goes with it** (the confirmation pass, LOW 7). The fan-out
            // worktrees lived below the removed tree, so their `.git/worktrees/` records
            // pointed at nothing: `git worktree list` named each one `prunable` and the
            // stale name could collide with a later `git worktree add`. The sibling doors
            // that take a worktree directory already prune; this one was the member of that
            // rule not following it.
            let listed = git(&bare.repo, &["worktree", "list"]);
            assert!(
                !listed.contains("prunable") && !listed.contains(".jigc/worktrees"),
                "{cell}: git's worktree admin must be clean after the teardown; got:\n{listed}",
            );
            assert!(
                !bare
                    .repo
                    .join(".git")
                    .join("worktrees")
                    .join("area-one")
                    .exists(),
                "{cell}: …and the stale registration directory must be gone",
            );
            // The still-live linked worktree is untouched — pruning drops records whose
            // directory is gone, never a worktree that is still there.
            assert!(
                listed.contains("feat") && bare.linked.is_dir(),
                "{cell}: the live linked worktree must survive the prune; got:\n{listed}",
            );
            assert!(
                seen.contains("pruned git's worktree registrations"),
                "{cell}: …and the door must say it pruned them; got:\n{seen}",
            );
        }
    }
}

/// The other half of binding jigc_home at `uninstall`: the four WIP guards now probe **the
/// home's** areas, including the fan-out worktree the caller is standing in.
///
/// Before the binding they were handed the standing checkout, where `.jigc/worktrees/` and
/// `.jigc/tasks/` do not exist at all — so from any worktree every one of them was inert.
/// That is the half of LOW 10 the review did not report: the door was not merely removing
/// the wrong install, it was removing it with no guard running.
///
/// Three refusals, one fixture: each removes nothing, so the state carries forward.
#[test]
fn uninstall_from_inside_a_worktree_refuses_over_what_the_removal_would_take() {
    let bare = Bare::new("uninstall-guards");
    let worktree = bare.with_fan_out();

    // (a) the standing worktree holds uncommitted work.
    fs::write(worktree.join("wip.txt"), "unsaved\n").expect("write wip");
    let out = bare.jigc(&worktree, &["uninstall"]);
    let seen = both_streams(&out);
    assert!(!out.status.success(), "(a) must refuse; got:\n{seen}");
    assert!(
        seen.contains("uninstall.dirty-worktree")
            && seen.contains(".jigc/worktrees/area-one")
            && seen.contains("wip.txt"),
        "(a) must name the standing worktree and what it holds; got:\n{seen}",
    );
    assert!(bare.repo.join(".jigc").is_dir(), "(a) must remove nothing");
    fs::remove_file(worktree.join("wip.txt")).expect("clear wip");

    // (b) the standing worktree is clean, but git has left an operation un-concluded — the
    // M53 post-review-fix leg (`probe_leftover`'s `OwnWorktree` arm), reached here through
    // the home's worktrees set rather than the standing checkout's absent one.
    let out = Command::new("git")
        .args(["bisect", "start"])
        .current_dir(&worktree)
        .output()
        .expect("run git bisect start");
    assert!(out.status.success(), "fixture: git bisect start");
    let out = Command::new("git")
        .args(["bisect", "bad"])
        .current_dir(&worktree)
        .output()
        .expect("run git bisect bad");
    assert!(out.status.success(), "fixture: git bisect bad");
    let out = bare.jigc(&worktree, &["uninstall"]);
    let seen = both_streams(&out);
    assert!(!out.status.success(), "(b) must refuse; got:\n{seen}");
    assert!(
        seen.contains("uninstall.dirty-worktree") && seen.contains("bisect"),
        "(b) must name the un-concluded operation; got:\n{seen}",
    );
    assert!(bare.repo.join(".jigc").is_dir(), "(b) must remove nothing");
    git(&worktree, &["bisect", "reset"]);

    // (c) a file under the home's `.jigc/` that no index has a copy of.
    fs::write(
        bare.repo.join(".jigc").join("config").join("local.yaml"),
        "unsaved: yes\n",
    )
    .expect("write the untracked workbench file");
    let out = bare.jigc(&worktree, &["uninstall"]);
    let seen = both_streams(&out);
    assert!(!out.status.success(), "(c) must refuse; got:\n{seen}");
    assert!(
        seen.contains("uninstall.untracked-workbench-file")
            && seen.contains(".jigc/config/local.yaml"),
        "(c) must name the unsaved file at the home; got:\n{seen}",
    );
    assert!(bare.repo.join(".jigc").is_dir(), "(c) must remove nothing");
}
