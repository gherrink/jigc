//! `jigc uninstall` stops taking the workbench bytes no index has a copy of — the
//! **added third subject** (M50 Increment 4, T4; `design/project-setup.md` →
//! Teardown / cleanup (G5); `completions/artifacts/M50/settle-record.md` → D8).
//!
//! The teardown's first act is `remove_dir_all(<repo>/.jigc)`. Two guards already
//! stand in front of it — a fan-out worktree holding content
//! (`uninstall.dirty-worktree`, M47) and an open task's staged docs
//! (`uninstall.staged-prose`, M46/M49) — and both rest on **one** ground: no commit
//! has a copy of those bytes. Stated over the whole tree that ground covers a third
//! set neither guard looks at: a file sitting **directly under `.jigc/`**, outside
//! the transient `cli::gitignore::ENTRIES` prefixes, that is in no index. A fresh
//! `jigc setup` tracks every non-transient path it writes, so that set is empty on an
//! ordinary install — and non-empty the moment anything else lands there. `jigc config
//! set` is the shipped door that does exactly that: it writes
//! `.jigc/config/manifest.yaml` and never tracks it, so before this guard an
//! `uninstall` after any `config set` destroyed the project's whole recorded cascade
//! delta at **exit 0**.
//!
//! **It is an ADDED third subject, never a replacement.** `tasks/` and `worktrees/`
//! are *inside* `ENTRIES`, so a guard whose subject were "everything under `.jigc/`
//! that is untracked" would swallow — and re-open — the two defects M46 Inc 2, M47 Inc
//! 3 and M49 built the other two guards for. Arm (d) below drives that separation: a
//! staged doc still draws `uninstall.staged-prose`, never this code.
//!
//! **In the index, not committed, is the line.** `git ls-files --cached` includes a
//! staged add, and a staged add is `git checkout`-recoverable — so a `git add`-ed file
//! under `.jigc/` does **not** block. It is *named by the narration* instead (arm (e)),
//! which is `DESTROYING_DOORS`' law-1 obligation at this door: what the guards let
//! through, the teardown names.
//!
//! **A stated behaviour change** (arm (b)): an `uninstall` after any `jigc config set`
//! now refuses until the manifest is committed or `--force` consents. G5 states the
//! guards' one provable ground — no commit has a copy — and explicitly refuses to
//! discriminate authorship, so a file jigc itself wrote is not exempt from the rule
//! jigc's own guards run on.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::trial_corpus::{State, TrialCorpus};

/// The adapter's owned guide artifact — one of the repo-local files a refused teardown
/// must leave standing.
const GUIDE_PATH: &str = ".claude/skills/jigc/SKILL.md";

/// The finding code the added third subject refuses under.
const CODE: &str = "uninstall.untracked-workbench-file";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-uninstall-workbench-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success.
fn git_ok(cwd: &Path, args: &[&str]) {
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
}

/// A real git repo with a usable identity — `jigc setup` commits its own install
/// footprint, so the identity has to be there before it runs.
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    git_ok(root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "hello\n").expect("seed README");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// An installed corpus: a git repo carrying a real `jigc setup` (its install commit
/// included), so every non-transient `.jigc/` path is tracked and the added third
/// subject starts **empty**.
struct Installed {
    repo: TempDir,
    home: TempDir,
}

impl Installed {
    fn new(tag: &str) -> Self {
        let repo = TempDir::new(&format!("{tag}-repo"));
        let home = TempDir::new(&format!("{tag}-home"));
        init_repo(repo.path());
        let setup = run_jigc(repo.path(), home.path(), &["setup"]);
        assert!(
            setup.status.success(),
            "`jigc setup` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&setup.stderr),
        );
        Installed { repo, home }
    }

    fn repo(&self) -> &Path {
        self.repo.path()
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        run_jigc(self.repo.path(), self.home.path(), args)
    }

    /// Every repo-local artifact `setup` wrote is still standing — the "removes
    /// nothing" half of a refusal.
    fn assert_install_intact(&self, tag: &str) {
        assert!(
            self.repo().join(".jigc").is_dir(),
            "{tag}: a refused teardown must leave `.jigc/` standing",
        );
        let claude = fs::read_to_string(self.repo().join("CLAUDE.md"))
            .unwrap_or_else(|err| panic!("{tag}: CLAUDE.md must still be readable: {err}"));
        assert!(
            claude.contains("@.jigc/AGENT.md"),
            "{tag}: a refused teardown must leave the bootstrap import wired; got:\n{claude}",
        );
        let settings = fs::read_to_string(self.repo().join(".claude/settings.json"))
            .unwrap_or_else(|err| panic!("{tag}: the settings file must still be readable: {err}"));
        assert!(
            settings.contains("Bash(jigc:*)"),
            "{tag}: a refused teardown must leave the allowlist permit; got:\n{settings}",
        );
        assert!(
            self.repo().join(GUIDE_PATH).is_file(),
            "{tag}: a refused teardown must leave the adapter guide artifact",
        );
    }
}

/// The `route:` line of a rendered finding block.
fn route_line(stderr: &str) -> String {
    stderr
        .lines()
        .find(|l| l.trim_start().starts_with("route:"))
        .unwrap_or_else(|| panic!("the refusal must render a `route:` line; got:\n{stderr}"))
        .to_owned()
}

/// **(a)** A hand-dropped `.jigc/notes.md` — bytes no index has a copy of, outside every
/// `ENTRIES` prefix — blocks the teardown by name, and the teardown removes nothing.
#[test]
fn a_hand_dropped_workbench_file_blocks_and_the_install_survives() {
    let site = Installed::new("hand-dropped");
    fs::write(site.repo().join(".jigc").join("notes.md"), "scratch\n").expect("drop notes.md");

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an untracked workbench file must block the teardown; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(CODE),
        "the refusal must carry `{CODE}`; got:\n{stderr}",
    );
    assert!(
        stderr.contains(".jigc/notes.md"),
        "the refusal must name the path it is refusing over; got:\n{stderr}",
    );
    let route = route_line(&stderr);
    assert!(
        route.contains("--force"),
        "the route must name the consent that proceeds anyway; got: {route}",
    );

    assert!(
        site.repo().join(".jigc").join("notes.md").is_file(),
        "the refused teardown must not have removed the very bytes it refused over",
    );
    site.assert_install_intact("hand-dropped");
}

/// **(b)** The stated behaviour change: `jigc config set` writes
/// `.jigc/config/manifest.yaml` and tracks nothing, so an `uninstall` after any
/// `config set` now refuses until that delta is committed (or `--force` consents).
#[test]
fn an_uncommitted_config_delta_blocks_the_teardown() {
    let site = Installed::new("config-delta");
    let set = site.run(&["config", "set", "docs-root", "notes"]);
    assert!(
        set.status.success(),
        "`jigc config set docs-root notes` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&set.stderr),
    );
    let manifest = site
        .repo()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    assert!(
        manifest.is_file(),
        "`config set` must have written the project-layer manifest",
    );

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an uncommitted cascade delta must block the teardown; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(CODE),
        "the refusal must carry `{CODE}`; got:\n{stderr}",
    );
    assert!(
        stderr.contains(".jigc/config/manifest.yaml"),
        "the refusal must name the recorded delta it is refusing over; got:\n{stderr}",
    );
    assert!(
        manifest.is_file(),
        "the refused teardown must leave the delta on disk",
    );
    site.assert_install_intact("config-delta");
}

/// **(c)** `--force` is the single consent: the same corpus tears down at exit 0.
#[test]
fn force_tears_down_the_same_corpus() {
    let site = Installed::new("forced");
    let set = site.run(&["config", "set", "docs-root", "notes"]);
    assert!(set.status.success(), "`config set` must exit 0");
    fs::write(site.repo().join(".jigc").join("notes.md"), "scratch\n").expect("drop notes.md");

    let blocked = site.run(&["uninstall"]);
    assert!(
        !blocked.status.success(),
        "the bare teardown must refuse before `--force` is shown to proceed",
    );

    let out = site.run(&["uninstall", "--force"]);
    assert!(
        out.status.success(),
        "`jigc uninstall --force` must tear the same corpus down; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "`--force` must have removed `.jigc/`",
    );
}

/// **(d)** The added subject is added, not substituted: a staged doc under
/// `.jigc/tasks/<id>/docs/` is inside an `ENTRIES` prefix, so it still draws the
/// shipped `uninstall.staged-prose` and never this code.
#[test]
fn a_staged_task_doc_still_draws_the_staged_prose_code() {
    let site = Installed::new("staged");
    let start = site.run(&["start", "--workflow", "single-task", "probe the guard"]);
    assert!(
        start.status.success(),
        "`jigc start` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an open task's staged doc must block the teardown; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("uninstall.staged-prose"),
        "a staged task doc must keep drawing `uninstall.staged-prose`; got:\n{stderr}",
    );
    assert!(
        !stderr.contains(CODE),
        "`.jigc/tasks/` is inside an ENTRIES prefix — it must NOT draw `{CODE}`; got:\n{stderr}",
    );
}

/// **(e)** In the index is the line: a `git add`-ed `.jigc/kept.md` is
/// `git checkout`-recoverable, so it does not block — and the narration names it before
/// `remove_dir_all` takes it (law 1: what the guards let through, the door names).
#[test]
fn a_staged_add_is_narrated_not_refused() {
    let site = Installed::new("kept");
    fs::write(site.repo().join(".jigc").join("kept.md"), "kept\n").expect("write kept.md");
    git_ok(site.repo(), &["add", "--force", ".jigc/kept.md"]);

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "a file in the index is recoverable — it must not block; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains(CODE),
        "an in-index file must not draw `{CODE}`; got:\n{stderr}",
    );
    assert!(
        stderr.contains(".jigc/kept.md"),
        "the narration must name the tracked workbench file it removes; got:\n{stderr}",
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "the teardown must still have removed `.jigc/`",
    );
}

/// **(f)** The question is **bytes, not path membership**: a tracked workbench file the
/// operator has edited without staging has an index copy of the *old* bytes and none of
/// the new ones, so `git checkout -- <path>` does not bring the edit back — it throws it
/// away. Such a path belongs on the refusing side with the never-tracked ones, and the
/// narration must not call it restorable.
///
/// `.jigc/config/packs.yaml` is the ordinary cell: a user-editable multi-pack listing
/// `jigc setup` tracks and a human hand-edits.
#[test]
fn a_tracked_but_modified_workbench_file_blocks_the_teardown() {
    let site = Installed::new("modified");
    let packs = site.repo().join(".jigc/config/packs.yaml");
    let before = fs::read_to_string(&packs).expect("read packs.yaml");
    fs::write(&packs, format!("{before}# hand-edited, never staged\n")).expect("edit packs.yaml");

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an unstaged edit to a tracked workbench file is in no index — the teardown must \
         refuse; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(CODE),
        "the refusal must carry `{CODE}`; got:\n{stderr}",
    );
    assert!(
        stderr.contains(".jigc/config/packs.yaml"),
        "the refusal must name the modified path; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("brings it back"),
        "no surface may claim the modified path is `git checkout`-restorable; got:\n{stderr}",
    );
    site.assert_install_intact("modified");
    let after = fs::read_to_string(&packs).expect("read packs.yaml after the refusal");
    assert!(
        after.contains("# hand-edited, never staged"),
        "the refused teardown must leave the operator's edit standing; got:\n{after}",
    );
}

/// **(g)** The fix's other side: when the index copy *is* the working-tree copy, the path
/// stays on the narrated side. A tracked workbench file edited **and staged** is
/// `git checkout`-recoverable exactly as arm (e)'s staged add is, so it must not block —
/// otherwise the fix would trade a false green for a false refusal.
#[test]
fn a_tracked_workbench_file_edited_and_staged_is_still_narrated() {
    let site = Installed::new("restaged");
    let packs = site.repo().join(".jigc/config/packs.yaml");
    let before = fs::read_to_string(&packs).expect("read packs.yaml");
    fs::write(&packs, format!("{before}# edited, then staged\n")).expect("edit packs.yaml");
    git_ok(site.repo(), &["add", "--force", ".jigc/config/packs.yaml"]);

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "the index carries these bytes — the teardown must not refuse; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains(CODE),
        "a staged edit must not draw `{CODE}`; got:\n{stderr}",
    );
    assert!(
        stderr.contains(".jigc/config/packs.yaml"),
        "the narration must name the tracked workbench file it removes; got:\n{stderr}",
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "the teardown must still have removed `.jigc/`",
    );
}

/// **(h)** The `ENTRIES` exclusion is a **prefix** exclusion, and a prefix is a directory
/// (M52 Increment 4 / T1, defect L-2 — `completions/artifacts/M52/baseline-destroying.md`
/// §4). `workbench_paths` dropped an entry whose *name* matched a transient prefix with no
/// shape check at all, so a plain file at `.jigc/tasks` — bytes in no index, outside every
/// transient subtree because there is no subtree — was invisible to this guard. It was
/// invisible to the other two in the same stroke (`fanout_worktree_paths` and
/// `staged_task_prose` both start `if !<root>.is_dir() { return empty }`), so all three
/// guards and all three narrations missed it at once: driven at `4572ca7c`, six such files
/// were destroyed at **exit 0**, named by nothing.
///
/// That is M49's `path.is_dir()` shape-vs-bytes error re-appearing as a **name**-vs-shape
/// error one function over, inside the guard M48 built to close it — and against
/// `workbench_paths`' own doc-comment: *"**Every child that is not a directory**, symlinks
/// included: the shape of a path is a reason to recurse into it, never a reason to drop it
/// from the set."*
///
/// The arm iterates `cli::gitignore::ENTRIES` rather than a hand-listed name or the one
/// reported cell, so an eighth transient prefix joins this assertion by existing.
#[test]
fn a_plain_file_at_every_transient_prefix_name_blocks_the_teardown() {
    let site = Installed::new("prefix-shaped-file");
    let jigc = site.repo().join(".jigc");

    let names: Vec<&str> = cli::gitignore::ENTRIES
        .lines()
        .map(|entry| entry.trim_end_matches('/'))
        .collect();
    assert!(
        !names.is_empty(),
        "the transient prefix set must be non-empty, or this arm asserts nothing",
    );

    for name in &names {
        let at = jigc.join(name);
        // `setup` leaves some of these as empty directories; the plant replaces them, which
        // is the state an operator's own stray file would land in.
        if at.is_dir() {
            fs::remove_dir_all(&at).unwrap_or_else(|err| panic!("clear `.jigc/{name}`: {err}"));
        }
        fs::write(&at, format!("PRECIOUS {name} BYTES\n"))
            .unwrap_or_else(|err| panic!("plant `.jigc/{name}`: {err}"));
    }

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a plain file at a transient prefix name is in no index and in no subtree — it must \
         block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(CODE),
        "the refusal must carry `{CODE}` — the third subject's own code; got:\n{stderr}",
    );
    for name in &names {
        assert!(
            stderr.contains(&format!(".jigc/{name}")),
            "the refusal must name `.jigc/{name}`: a door that names one of the things it \
             would destroy is the law-1 half-truth; got:\n{stderr}",
        );
        let at = jigc.join(name);
        assert_eq!(
            fs::read_to_string(&at).ok(),
            Some(format!("PRECIOUS {name} BYTES\n")),
            "the refused teardown must leave `.jigc/{name}` byte-intact",
        );
    }
    site.assert_install_intact("prefix-shaped-file");
}

// ── (R9, F5): the bytes INSIDE a transient prefix (rc.24 fix pass) ──────────────────────
//
// Arm (h) plants a file **at** each `ENTRIES` name and stops one level short. Driven on
// `jigc 1.0.0-rc.24`, a file planted **inside** `.jigc/logs/`, `.jigc/state/` or
// `.jigc/index/`, or directly under `.jigc/tasks/` or `.jigc/milestones/` beside the
// areas, was destroyed at exit 0 and named by nothing — and, with no plant at all, so was
// the opt-in invocation log (`completions/artifacts/M55/per-axis-review-rc24/
// tier1-verification/R9-F5.md`). The subject of the arms below is therefore **every byte
// under an `ENTRIES` directory**, iterated off the production constant, never the three
// directories the finding happened to report.

/// The code a byte jigc did not write refuses under.
const FOREIGN: &str = "uninstall.foreign-bytes";

/// The opt-in invocation log, repo-relative.
const LOG: &str = ".jigc/logs/invocations.jsonl";

/// The transient prefixes, read off the production constant.
fn transient_prefixes() -> Vec<&'static str> {
    let names: Vec<&str> = cli::gitignore::ENTRIES
        .lines()
        .map(|entry| entry.trim_end_matches('/'))
        .collect();
    assert!(
        !names.is_empty(),
        "the transient prefix set must be non-empty, or these arms assert nothing",
    );
    names
}

/// Write `body` at `relative` under the site's repo, creating parents.
fn plant(site: &Installed, relative: &str, body: &str) {
    let at = site.repo().join(relative);
    fs::create_dir_all(at.parent().expect("a planted path has a parent"))
        .unwrap_or_else(|err| panic!("create the parent of `{relative}`: {err}"));
    fs::write(&at, body).unwrap_or_else(|err| panic!("plant `{relative}`: {err}"));
}

/// Every leaf under `<repo>/<relative>`, repo-relative and sorted — read off the disk, so
/// an arm that asks *"was everything there named?"* cannot be answered from a list.
fn leaves_under(repo: &Path, relative: &str) -> Vec<String> {
    let mut stack = vec![repo.join(relative)];
    let mut leaves = Vec::new();
    while let Some(path) = stack.pop() {
        let Ok(shape) = fs::symlink_metadata(&path) else {
            continue;
        };
        if shape.is_dir() {
            for entry in fs::read_dir(&path).expect("read a workbench directory") {
                stack.push(entry.expect("read a workbench entry").path());
            }
            continue;
        }
        leaves.push(
            path.strip_prefix(repo)
                .expect("a leaf under the repo")
                .to_string_lossy()
                .into_owned(),
        );
    }
    leaves.sort();
    leaves
}

/// Turn the opt-in invocation log on and put the knob's delta in the index, so the
/// recorded delta is not itself what the teardown refuses over (arm (b)).
fn turn_the_log_on(site: &Installed) {
    let set = site.run(&["config", "set", "invocation-log", "true"]);
    assert!(
        set.status.success(),
        "`jigc config set invocation-log true` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&set.stderr),
    );
    git_ok(site.repo(), &["add", "--", ".jigc/config"]);
}

/// **(i)** The class axis: a file planted **inside** every transient prefix blocks the
/// teardown, is named, and is byte-intact afterwards — under one of the door's own codes,
/// and under `uninstall.foreign-bytes` at the five positions rc.24 took in silence. One
/// level down and three levels down, because a guard that reads one level is the defect.
///
/// It iterates `cli::gitignore::ENTRIES`, so an eighth transient prefix joins this
/// assertion by existing — and reddens unless some guard answers for it.
#[test]
fn a_file_inside_every_transient_prefix_blocks_the_teardown() {
    let door_codes = cli::milestone::UNINSTALL_DOOR.codes;
    for prefix in transient_prefixes() {
        for relative in [
            format!(".jigc/{prefix}/notes.txt"),
            format!(".jigc/{prefix}/sub/deep/notes.txt"),
        ] {
            let site = Installed::new(&format!("inside-{prefix}"));
            let body = format!("PRECIOUS {relative} BYTES\n");
            plant(&site, &relative, &body);

            let out = site.run(&["uninstall"]);
            let stderr = String::from_utf8_lossy(&out.stderr);
            assert!(
                !out.status.success(),
                "`{relative}` is in no index — the teardown must refuse; stdout:\n{}\nstderr:\n{stderr}",
                String::from_utf8_lossy(&out.stdout),
            );
            let code = door_codes
                .iter()
                .find(|code| stderr.contains(*code))
                .unwrap_or_else(|| {
                    panic!("the refusal over `{relative}` must carry one of {door_codes:?}; got:\n{stderr}")
                });
            // `worktrees/` is the leftover classifier's subject; a directory under `tasks/`
            // or `milestones/` is a working area, answered by the registry's complement.
            // Everything else in this loop is a position no guard reached on rc.24.
            if prefix != "worktrees" {
                assert_eq!(
                    *code, FOREIGN,
                    "`{relative}` is a byte jigc did not write; got:\n{stderr}",
                );
            }
            // The path, or the entry that holds it: a foreign directory inside a working
            // area is named whole (`engine::state::foreign_area_paths`).
            let holder = format!(".jigc/{prefix}/sub");
            assert!(
                stderr.contains(&relative)
                    || (relative.contains("/sub/") && stderr.contains(&holder)),
                "the refusal must name `{relative}`; got:\n{stderr}",
            );
            assert_eq!(
                fs::read_to_string(site.repo().join(&relative)).ok(),
                Some(body),
                "the refused teardown must leave `{relative}` byte-intact",
            );
            site.assert_install_intact(&relative);
        }
    }
}

/// **(j)** A symlink inside a cache directory is a leaf the teardown would take, so it
/// blocks like any other byte — and the walk does not follow it out of the tree.
#[cfg(unix)]
#[test]
fn a_symlink_inside_a_cache_directory_blocks_and_its_target_is_untouched() {
    let site = Installed::new("cache-symlink");
    let outside = site.home.path().join("outside.txt");
    fs::write(&outside, "OUTSIDE\n").expect("write the link target");
    fs::create_dir_all(site.repo().join(".jigc/state")).expect("create state/");
    std::os::unix::fs::symlink(&outside, site.repo().join(".jigc/state/link"))
        .expect("plant the symlink");

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a symlink is a leaf — it must block; got:\n{stderr}"
    );
    assert!(
        stderr.contains(FOREIGN),
        "the code is `{FOREIGN}`; got:\n{stderr}"
    );
    assert!(
        stderr.contains(".jigc/state/link"),
        "the link is named; got:\n{stderr}"
    );
    assert_eq!(
        fs::read_to_string(&outside).ok().as_deref(),
        Some("OUTSIDE\n"),
        "the link's target is none of this door's business",
    );
}

/// **(k)** The subject is the **path**, not the index: a file jigc did not write inside a
/// cache directory blocks whether or not somebody force-added it, so the
/// tracked-then-edited leg — whose index copy is the *old* bytes — is never taken.
#[test]
fn a_tracked_then_edited_file_inside_a_cache_directory_blocks() {
    let site = Installed::new("cache-tracked-edit");
    plant(&site, ".jigc/state/notes.txt", "first\n");
    git_ok(
        site.repo(),
        &["add", "--force", "--", ".jigc/state/notes.txt"],
    );
    plant(
        &site,
        ".jigc/state/notes.txt",
        "first\nPRECIOUS unstaged edit\n",
    );

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the unstaged edit is in no index; got:\n{stderr}"
    );
    assert!(
        stderr.contains(FOREIGN),
        "the code is `{FOREIGN}`; got:\n{stderr}"
    );
    assert!(
        stderr.contains(".jigc/state/notes.txt"),
        "the path is named; got:\n{stderr}"
    );
    assert_eq!(
        fs::read_to_string(site.repo().join(".jigc/state/notes.txt"))
            .ok()
            .as_deref(),
        Some("first\nPRECIOUS unstaged edit\n"),
        "the refused teardown leaves the edit standing",
    );
}

/// **(l)** `--force` is consent to the loss, never to silence: what it takes from inside
/// the transient prefixes, it names.
#[test]
fn force_names_what_it_took_from_inside_the_transient_prefixes() {
    let site = Installed::new("forced-inside");
    let planted: Vec<String> = ["logs", "state", "index", "tasks", "milestones"]
        .iter()
        .map(|prefix| format!(".jigc/{prefix}/notes.txt"))
        .collect();
    for relative in &planted {
        plant(&site, relative, "PRECIOUS\n");
    }

    let out = site.run(&["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "`--force` tears it down; got:\n{stderr}"
    );
    for relative in &planted {
        assert!(
            stderr.contains(relative.as_str()),
            "`--force` must name `{relative}` as taken; got:\n{stderr}",
        );
    }
    assert!(
        !site.repo().join(".jigc").exists(),
        "`--force` removed `.jigc/`"
    );
}

/// **(m)** The foreign refusal's route, followed as printed, clears it: move the file out,
/// re-run, exit 0. The route names no `git add` — the tree is gitignored, so that command
/// exits 1 on these paths (the control below shows it).
#[test]
fn the_foreign_refusals_route_followed_verbatim_clears_it() {
    let site = Installed::new("foreign-route");
    plant(&site, ".jigc/logs/notes.txt", "PRECIOUS\n");

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "the plant blocks; got:\n{stderr}");
    let route = route_line(&stderr);
    assert!(
        !route.contains("git ") && route.contains("move") && route.contains("jigc uninstall"),
        "the route is move-or-delete then re-run, and names no git command; got: {route}",
    );
    let add = Command::new("git")
        .args(["add", "--", ".jigc/logs/notes.txt"])
        .current_dir(site.repo())
        .output()
        .expect("run git add");
    assert!(
        !add.status.success(),
        "the control: `git add` refuses an ignored path, which is why the route must not name it",
    );

    fs::rename(
        site.repo().join(".jigc/logs/notes.txt"),
        site.home.path().join("notes.txt"),
    )
    .expect("move the file out, as the route says");
    let rerun = site.run(&["uninstall"]);
    assert!(
        rerun.status.success(),
        "the re-run the route names must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&rerun.stderr),
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "the re-run removed `.jigc/`"
    );
}

/// **(n)** The own-row completeness control — the M52 `renames.json` lesson as a driven
/// cell: over **every corpus state the trial substrate builds**, nothing jigc itself wrote
/// under `state/` or `index/` is ever called foreign, and every file that was there is
/// named as the teardown takes it. The set is read off the disk before the run, never from
/// a list of what jigc is believed to write — a writer added to either directory and
/// missing from the teardown's own-file row reddens here on the state that exercises it.
#[test]
fn jigcs_own_caches_are_never_foreign_and_every_one_is_named() {
    let mut seen = 0;
    for state in State::ALL {
        let corpus = TrialCorpus::build(*state);
        let repo = corpus.repo();
        let mut own = leaves_under(&repo, ".jigc/state");
        own.extend(leaves_under(&repo, ".jigc/index"));
        seen += own.len();

        // Unforced: whatever else this state refuses over (an open task's staged docs, in
        // `refs-post-hoc`), it is never jigc's own cache under the foreign code.
        let bare = corpus.jigc(&["uninstall"]);
        let stderr = String::from_utf8_lossy(&bare.stderr).into_owned();
        if stderr.contains(FOREIGN) {
            for path in &own {
                assert!(
                    !stderr.contains(path.as_str()),
                    "[{state:?}] `{path}` is jigc's own file and must not be refused as \
                     foreign; got:\n{stderr}",
                );
            }
        }
        // …and the teardown, however it is reached, names each one it takes.
        let (out, stderr) = if bare.status.success() {
            (bare, stderr)
        } else {
            let forced = corpus.jigc(&["uninstall", "--force"]);
            let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
            (forced, stderr)
        };
        assert!(
            out.status.success(),
            "[{state:?}] the teardown must complete; stderr:\n{stderr}",
        );
        for path in &own {
            assert!(
                stderr.contains(path.as_str()),
                "[{state:?}] the teardown must name `{path}` as it takes it; got:\n{stderr}",
            );
        }
        assert!(
            !repo.join(".jigc").exists(),
            "[{state:?}] the teardown removed `.jigc/`",
        );
    }
    assert!(
        seen > 0,
        "at least one corpus state must carry jigc's caches, or this arm asserts nothing",
    );
}

/// **(n′)** The same row over the one member real use leaves only on a fault: the temp a
/// killed atomic save leaves beside a cache (`<name>.<pid>.<nanos>.tmp`,
/// `engine::state::is_temp_sibling`). It is jigc's own residue, so it goes with the tree,
/// named — where a neighbour that merely looks like it (`notes.12.34.tmp`) is a byte jigc
/// did not write, and blocks.
#[test]
fn a_killed_saves_temp_is_jigcs_own_and_a_lookalike_is_not() {
    let site = Installed::new("save-temp");
    let temp = ".jigc/state/file-state.json.4242.1791095459937368000.tmp";
    plant(&site, temp, "{}\n");
    let lookalike = ".jigc/state/notes.4242.1791095459937368000.tmp";
    plant(&site, lookalike, "PRECIOUS\n");

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the lookalike blocks; got:\n{stderr}"
    );
    assert!(
        stderr.contains(FOREIGN) && stderr.contains(lookalike),
        "the lookalike is foreign and named; got:\n{stderr}",
    );
    assert!(
        !stderr.contains(temp),
        "jigc's own temp is not refused over; got:\n{stderr}",
    );

    fs::remove_file(site.repo().join(lookalike)).expect("delete the lookalike, as the route says");
    let rerun = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&rerun.stderr);
    assert!(
        rerun.status.success(),
        "jigc's own temp does not block; got:\n{stderr}"
    );
    assert!(
        stderr.contains(temp),
        "and it is named as it goes; got:\n{stderr}"
    );
}

/// **(o)** The install footprint a failed first `jigc setup` records under `state/` is
/// jigc's own too: the teardown of that half-made install is not refused over it, and it
/// is named.
#[cfg(unix)]
#[test]
fn a_failed_first_setups_footprint_never_blocks_and_is_named() {
    use std::os::unix::fs::PermissionsExt;
    let repo = TempDir::new("footprint-repo");
    let home = TempDir::new("footprint-home");
    init_repo(repo.path());
    // A committed, read-only in-repo hooks dir fails the install at its last write.
    let hooks = repo.path().join(".githooks");
    fs::create_dir_all(&hooks).expect("create the hooks dir");
    fs::write(hooks.join("README"), "project hooks\n").expect("seed the hooks dir");
    git_ok(repo.path(), &["add", "--", ".githooks/README"]);
    git_ok(repo.path(), &["commit", "-q", "-m", "hooks"]);
    git_ok(repo.path(), &["config", "core.hooksPath", ".githooks"]);
    fs::set_permissions(&hooks, fs::Permissions::from_mode(0o555)).expect("lock the hooks dir");

    let setup = run_jigc(repo.path(), home.path(), &["setup"]);
    fs::set_permissions(&hooks, fs::Permissions::from_mode(0o755)).expect("unlock the hooks dir");
    assert!(
        !setup.status.success(),
        "the fixture must fail the first install"
    );
    let footprint = ".jigc/state/setup-install-footprint";
    assert!(
        repo.path().join(footprint).is_file(),
        "the failed install must have recorded its footprint, or this arm asserts nothing",
    );

    let out = run_jigc(repo.path(), home.path(), &["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "jigc's own footprint record must not block the teardown; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(footprint),
        "the footprint is named; got:\n{stderr}"
    );
}

/// **(p)** The opt-in invocation log blocks the teardown — it is sole-copy, operator-opted
/// data no index has a copy of — under the third subject's own code, with a route that is
/// true of a gitignored path: no `git add`. Followed as printed (move the log out, re-run)
/// the teardown exits 0, **does not re-create `.jigc/` to log itself**, and a second run is
/// the clean no-op the door promises (rc.24's `(R9, F3)`).
#[test]
fn the_invocation_log_blocks_the_teardown_and_its_route_clears_it() {
    let site = Installed::new("log-blocks");
    turn_the_log_on(&site);
    for args in [&["validate"][..], &["doc", "list"]] {
        assert!(
            site.run(args).status.success(),
            "`jigc {args:?}` must exit 0"
        );
    }
    let log = site.repo().join(LOG);
    let before = fs::read_to_string(&log).expect("the knob is on, so the log exists");
    assert!(
        before.contains(r#"["doc","list"]"#),
        "the before-control: the log holds the records the teardown would take; got:\n{before}",
    );

    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the invocation log is in no index — the teardown must refuse; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(CODE),
        "the refusal carries `{CODE}`; got:\n{stderr}"
    );
    assert!(
        stderr.contains(LOG),
        "the refusal names the log; got:\n{stderr}"
    );
    let route = route_line(&stderr);
    assert!(
        !route.contains("add --")
            && route.contains("git ignores")
            && route.contains("move")
            && route.contains("--force"),
        "the log is gitignored, so the route is move-or-delete (never a `git add` to run), \
         or the consent; got: {route}",
    );
    let after = fs::read_to_string(&log).expect("the refused teardown leaves the log");
    assert!(
        after.starts_with(&before),
        "the refused teardown must leave every earlier record in place",
    );
    site.assert_install_intact("log-blocks");

    // The route, as printed: move the log out of `.jigc/`, then re-run.
    let kept = site.home.path().join("invocations.jsonl");
    fs::rename(&log, &kept).expect("move the log out, as the route says");
    let rerun = site.run(&["uninstall"]);
    assert!(
        rerun.status.success(),
        "the re-run the route names must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&rerun.stderr),
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "the teardown must not re-create `.jigc/` to record its own invocation",
    );
    assert!(
        fs::read_to_string(&kept)
            .expect("the moved log")
            .contains(r#"["doc","list"]"#),
        "the records the operator moved out are still theirs",
    );

    let second = site.run(&["uninstall"]);
    let stdout = String::from_utf8_lossy(&second.stdout);
    assert!(
        second.status.success() && stdout.contains("nothing to remove"),
        "a second run is a clean no-op; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&second.stderr),
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "and it re-creates nothing either"
    );
}

/// **(q)** The consent arm of (p): `--force` takes the log, names it as not recoverable,
/// and leaves no `.jigc/` behind.
#[test]
fn force_takes_the_invocation_log_names_it_and_leaves_nothing_behind() {
    let site = Installed::new("log-forced");
    turn_the_log_on(&site);
    assert!(
        site.run(&["validate"]).status.success(),
        "`jigc validate` must exit 0"
    );
    assert!(
        site.repo().join(LOG).is_file(),
        "the knob is on, so the log exists"
    );

    let out = site.run(&["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "`--force` tears it down; got:\n{stderr}"
    );
    assert!(
        stderr.contains(LOG),
        "`--force` names the log it took; got:\n{stderr}"
    );
    assert!(
        stderr.contains("not recoverable"),
        "and says the records are gone for good; got:\n{stderr}",
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "the forced teardown must not re-create `.jigc/` to record itself",
    );
}

/// **(r)** The third subject's route is true of **every** path it lists, whichever way git
/// treats it. `git add` is the cheap exit for a path git will take and exits 1 on one git
/// ignores — so a listing that is all ignored names no `git add` at all, and a mixed one
/// says which paths have to be moved instead. The ignore rule here is the operator's own
/// (`.git/info/exclude`), because the class is *a path git ignores*, never *the log*.
#[test]
fn the_third_subjects_route_is_true_of_an_ignored_path() {
    let site = Installed::new("ignored-route");
    let exclude = site.repo().join(".git/info/exclude");
    fs::create_dir_all(exclude.parent().expect("info dir")).expect("create .git/info");
    fs::write(&exclude, ".jigc/private.txt\n").expect("write the operator's ignore rule");
    plant(&site, ".jigc/private.txt", "PRECIOUS\n");

    // All ignored: the route names no `git add`.
    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the ignored file is in no index; got:\n{stderr}"
    );
    assert!(
        stderr.contains(CODE) && stderr.contains(".jigc/private.txt"),
        "got:\n{stderr}"
    );
    let route = route_line(&stderr);
    assert!(
        !route.contains("add --") && route.contains("move") && route.contains("git ignores"),
        "an all-ignored listing says so, routes at move-or-delete and prints no `git add` \
         to run; got: {route}",
    );

    // Mixed: `git add` for the path git takes, and the ignored one is named as the one
    // that has to move.
    plant(&site, ".jigc/notes.md", "scratch\n");
    let out = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "both files block; got:\n{stderr}");
    let route = route_line(&stderr);
    assert!(
        route.contains("add -- <path>") && route.contains("`.jigc/private.txt`"),
        "a mixed listing keeps `git add` and names the ignored path apart; got: {route}",
    );
    // …and the route's two claims are true as printed.
    let add = |path: &str| {
        Command::new("git")
            .args(["add", "--", path])
            .current_dir(site.repo())
            .output()
            .expect("run git add")
            .status
            .success()
    };
    assert!(
        add(".jigc/notes.md"),
        "`git add` keeps the path git does not ignore"
    );
    assert!(!add(".jigc/private.txt"), "and refuses the one it ignores");
    fs::rename(
        site.repo().join(".jigc/private.txt"),
        site.home.path().join("private.txt"),
    )
    .expect("move the ignored file out, as the route says");
    let rerun = site.run(&["uninstall"]);
    assert!(
        rerun.status.success(),
        "both exits taken, the re-run exits 0; stderr:\n{}",
        String::from_utf8_lossy(&rerun.stderr),
    );
}

// --- The teardown never starts the log (the rc.24 fix pass, left open by `(R9, F5)`) ----
//
// `(R9, F5)` made the invocation log block the teardown, and kept a **refused** teardown
// recorded like any other run. Together those two re-armed the refusal: an operator who
// moved the log out of `.jigc/` as the route says, and was then refused for **another**
// reason, found `.jigc/logs/invocations.jsonl` back — holding one record, the refusal's own
// — and blocking the next run. The human's ruling for `(R9, F5)` covers it (`DECISIONS.md`
// → 2026-10-04: *"the teardown's own invocation record must not re-create `.jigc/logs/`, so
// a second `uninstall` … never trips over a log the first one wrote"*): an `uninstall`
// invocation, landing or refusing, never **creates** the log — it appends only to one that
// is already there.
//
// The subject of the arms below is therefore **every way an `uninstall` invocation ends**,
// never the one refusal the finding happened to report: each code the door refuses with
// (read off `UNINSTALL_DOOR.codes`), each answer clap gives before the door runs at all
// (help, a usage error), and the landing.

/// The two shapes *the log is gone* has on disk after an operator follows the log
/// refusal's route (*"move what you need out of `.jigc/`, or delete what you do not"*).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LogGone {
    /// The file moved out; `.jigc/logs/` stands empty. A deleted file is this same shape.
    FileMovedOut,
    /// The whole `.jigc/logs/` removed (`rm -r`), so re-creating the log means re-creating
    /// its directory too.
    DirectoryDeleted,
}

/// A site with the knob **on**, a log the knob has really minted, and that log then cleared
/// the way `how` says — where an operator stands after clearing the log's own refusal.
///
/// `before_clearing` runs while the log is still there, because one of the door's refusals
/// (`uninstall.staged-prose`) is only reachable through a jigc verb, and every verb but the
/// teardown mints the log.
fn logging_site_with_the_log_cleared(
    tag: &str,
    how: LogGone,
    before_clearing: impl FnOnce(&Installed),
) -> Installed {
    let site = Installed::new(tag);
    turn_the_log_on(&site);
    before_clearing(&site);
    assert!(
        site.run(&["doc", "list"]).status.success(),
        "[{tag}] `jigc doc list` must exit 0",
    );
    let log = site.repo().join(LOG);
    assert!(
        log.is_file(),
        "[{tag}] the before-control: the knob is on and a verb has minted the log, so an \
         absent log below is the teardown's doing and not a knob that was never on",
    );
    match how {
        LogGone::FileMovedOut => {
            fs::rename(&log, site.home.path().join("invocations.jsonl"))
                .expect("move the log out, as the route says");
        }
        LogGone::DirectoryDeleted => {
            fs::remove_dir_all(log.parent().expect("the log has a parent"))
                .expect("delete `.jigc/logs/`, as the route allows");
        }
    }
    assert_no_log(&site, &format!("{tag}: cleared"));
    site
}

/// Nothing is at the log's path — asked with `symlink_metadata`, so a dangling link there
/// counts as something.
fn assert_no_log(site: &Installed, when: &str) {
    assert!(
        fs::symlink_metadata(site.repo().join(LOG)).is_err(),
        "[{when}] an `uninstall` invocation must never create `{LOG}`; it holds:\n{}",
        fs::read_to_string(site.repo().join(LOG)).unwrap_or_default(),
    );
}

/// Put the site in the one state that draws `code` — a cell per member of
/// `UNINSTALL_DOOR.codes`, so a fifth refusal cannot join the door without saying here how
/// it is reached.
fn plant_the_refusal(site: &Installed, code: &str) {
    match code {
        "uninstall.dirty-worktree" => {
            plant(site, ".jigc/worktrees/leftover/notes.txt", "PRECIOUS\n");
        }
        "uninstall.staged-prose" => {
            let start = site.run(&["start", "--workflow", "single-task", "probe the guard"]);
            assert!(
                start.status.success(),
                "`jigc start` must exit 0; stderr:\n{}",
                String::from_utf8_lossy(&start.stderr),
            );
        }
        FOREIGN => plant(site, ".jigc/state/notes.txt", "PRECIOUS\n"),
        CODE => plant(site, ".jigc/notes.md", "scratch\n"),
        other => panic!(
            "`UNINSTALL_DOOR` now refuses with `{other}` — give it a cell here, so the \
             teardown's log rule is driven under it too"
        ),
    }
}

/// **(s)** The class axis: under **every** code the door refuses with, and at both shapes
/// of a cleared log, a refused `uninstall` leaves the log gone — so the next run is refused
/// for the same reason again and never for a log the first refusal wrote.
#[test]
fn a_refused_teardown_never_starts_the_log_under_any_code_the_door_refuses_with() {
    for code in cli::milestone::UNINSTALL_DOOR.codes {
        for how in [LogGone::FileMovedOut, LogGone::DirectoryDeleted] {
            let cell = format!("{code} · {how:?}");
            let site = logging_site_with_the_log_cleared(
                &format!("no-mint-{}-{how:?}", code.replace('.', "-")),
                how,
                |site| plant_the_refusal(site, code),
            );
            let logs_dir = site.repo().join(".jigc/logs");
            let directory_stood = logs_dir.exists();
            assert_eq!(
                directory_stood,
                how == LogGone::FileMovedOut,
                "[{cell}] the fixture's own shape",
            );

            for run in ["first", "second"] {
                let out = site.run(&["uninstall"]);
                let stderr = String::from_utf8_lossy(&out.stderr);
                assert!(
                    !out.status.success(),
                    "[{cell} · {run}] the teardown must refuse; stdout:\n{}\nstderr:\n{stderr}",
                    String::from_utf8_lossy(&out.stdout),
                );
                assert!(
                    stderr.contains(code),
                    "[{cell} · {run}] the refusal carries `{code}`; got:\n{stderr}",
                );
                assert!(
                    !stderr.contains(LOG),
                    "[{cell} · {run}] the log was cleared — the refusal must not be over a \
                     log an earlier `uninstall` wrote; got:\n{stderr}",
                );
                assert_no_log(&site, &format!("{cell} · {run}"));
                assert_eq!(
                    logs_dir.exists(),
                    directory_stood,
                    "[{cell} · {run}] and it neither creates nor removes `.jigc/logs/`",
                );
            }
            site.assert_install_intact(&cell);
        }
    }
}

/// **(t)** The invocations clap answers before the door runs — its help and its usage
/// errors — are `uninstall` invocations too: an operator who reads `jigc uninstall --help`
/// between two runs must not find the log back either. The last step is the control: on the
/// very same site any **other** verb still mints the log, so the knob was on throughout.
#[test]
fn an_uninstall_invocation_clap_answers_never_starts_the_log_either() {
    let site = logging_site_with_the_log_cleared("no-mint-clap", LogGone::FileMovedOut, |_| {});
    for (argv, exit) in [
        (&["uninstall", "--help"][..], 0),
        (&["uninstall", "-h"], 0),
        (&["help", "uninstall"], 0),
        (&["--format", "json", "uninstall", "--help"], 0),
        (&["uninstall", "extra"], 2),
        (&["uninstall", "--bogus"], 2),
        (&["uninstall", "--format", "nope"], 2),
    ] {
        let out = site.run(argv);
        assert_eq!(
            out.status.code(),
            Some(exit),
            "`jigc {argv:?}` exits {exit}; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        assert_no_log(&site, &format!("jigc {argv:?}"));
    }

    // An argument that is not valid UTF-8: the argv the log wrapper reads lossily.
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .arg("uninstall")
            .arg(std::ffi::OsStr::from_bytes(b"\xff"))
            .current_dir(site.repo())
            .env("HOME", site.home.path())
            .output()
            .expect("run the jigc binary");
        assert_eq!(
            out.status.code(),
            Some(2),
            "a non-UTF-8 argument is a usage error; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        assert_no_log(&site, "jigc uninstall <non-UTF-8>");
    }
    site.assert_install_intact("no-mint-clap");

    // The control — every other verb logs as before, a clap-answered one included.
    let help = site.run(&["validate", "--help"]);
    assert!(help.status.success(), "`jigc validate --help` must exit 0");
    let log = fs::read_to_string(site.repo().join(LOG))
        .expect("any verb but the teardown mints the log while the knob is on");
    assert_eq!(
        log.lines().count(),
        1,
        "and the minted log holds that one run, none of the teardown's; got:\n{log}",
    );
    assert!(
        log.contains(r#"["validate","--help"]"#),
        "the record is the other verb's own; got:\n{log}",
    );
}

/// **(u)** The drive the finding names, end to end: the log moved out → the teardown
/// refused for **another** reason (a foreign file) → that cleared → the teardown lands,
/// with no log re-created in between and nothing left behind.
#[test]
fn a_log_moved_out_stays_out_across_a_refusal_for_another_reason() {
    let site = logging_site_with_the_log_cleared("log-stays-out", LogGone::FileMovedOut, |_| {});
    plant(&site, ".jigc/state/notes.txt", "PRECIOUS\n");

    let refused = site.run(&["uninstall"]);
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert!(
        !refused.status.success() && stderr.contains(FOREIGN),
        "the foreign file blocks the teardown under `{FOREIGN}`; got:\n{stderr}",
    );
    assert_no_log(&site, "refused over a foreign file");

    // The foreign refusal's route, as printed: move the file out, then re-run.
    fs::rename(
        site.repo().join(".jigc/state/notes.txt"),
        site.home.path().join("notes.txt"),
    )
    .expect("move the foreign file out, as the route says");
    let landed = site.run(&["uninstall"]);
    assert!(
        landed.status.success(),
        "nothing blocks any more — the re-run must exit 0, not refuse over a log the \
         refused run wrote; stderr:\n{}",
        String::from_utf8_lossy(&landed.stderr),
    );
    assert!(
        !site.repo().join(".jigc").exists(),
        "the teardown that landed leaves no `.jigc/` behind",
    );
    assert!(
        fs::read_to_string(site.home.path().join("invocations.jsonl"))
            .expect("the moved log")
            .contains(r#"["doc","list"]"#),
        "the records the operator moved out are still theirs",
    );

    let second = site.run(&["uninstall"]);
    let stdout = String::from_utf8_lossy(&second.stdout);
    assert!(
        second.status.success() && stdout.contains("nothing to remove"),
        "a second run is a clean no-op; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&second.stderr),
    );
}

/// **(v)** The other half of the rule: *append only* still appends. A refused teardown is
/// recorded in a log that is **already there** — the record names the refusal — so the log
/// loses nothing it used to carry; it is only never started by this verb.
#[test]
fn a_refused_teardown_is_recorded_in_a_log_that_is_already_there() {
    let site = Installed::new("log-appends");
    turn_the_log_on(&site);
    assert!(
        site.run(&["doc", "list"]).status.success(),
        "`jigc doc list` must exit 0"
    );
    plant(&site, ".jigc/state/notes.txt", "PRECIOUS\n");
    let log = site.repo().join(LOG);
    let before = fs::read_to_string(&log).expect("the knob is on, so the log exists");

    let refused = site.run(&["uninstall"]);
    assert!(
        !refused.status.success(),
        "the foreign file and the log both block; stderr:\n{}",
        String::from_utf8_lossy(&refused.stderr),
    );

    let after = fs::read_to_string(&log).expect("the refused teardown leaves the log");
    assert!(
        after.starts_with(&before),
        "every earlier record stays in place",
    );
    let appended: Vec<&str> = after[before.len()..].lines().collect();
    assert_eq!(
        appended.len(),
        1,
        "the refused run appends exactly its own record; got:\n{after}",
    );
    let record: serde_json::Value =
        serde_json::from_str(appended[0]).expect("the appended line is one JSON record");
    assert_eq!(record["argv"], serde_json::json!(["uninstall"]));
    assert_eq!(record["exit_code"], serde_json::json!(1));
    assert!(
        record["finding_codes"]
            .as_array()
            .is_some_and(|codes| !codes.is_empty()),
        "the record names what the teardown refused over; got: {record}",
    );
}
