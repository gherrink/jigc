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
