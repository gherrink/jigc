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
        let root = TempDir::new(tag);
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo dir");
        init_repo(&repo);
        let home = TempDir::new(&format!("{tag}-home"));

        assert_ok(
            &jigc_in(&repo, home.path(), &["milestone", "create", "Cache rework"]),
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
/// It also pins the pairing `efe16554`'s commit message claims and no test held: the refusal
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
