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
/// guards and all three narrations missed it at once: driven at `ffb4064c`, six such files
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
