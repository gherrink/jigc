//! **jigc asks `git status` with its own flags, never through the user's `status.*`
//! configuration** (the rc.24 fix pass's completion audit, worktree-registrations F1).
//!
//! `git status --porcelain` is stable across git versions and *not* across configuration:
//! `status.showUntrackedFiles=no` — set by anyone who keeps a large or a home-directory
//! repository — removes every untracked path from it, and `status.renames` /
//! `diff.renames` decide whether a staged rename is one record or two. A door that decides
//! what it may destroy from that output therefore decided it from the user's dotfiles.
//! Driven on `jigc 1.0.0-rc.24` and on the tree before this suite: with
//! `status.showUntrackedFiles=no`, an un-forced `jigc milestone discard` and an un-forced
//! `jigc uninstall` each took a sub-agent's untracked file at exit 0 — the refusal probe
//! (`cli::milestone::dirty_worktrees`) read an empty listing, while the narration probe,
//! which already passed `--untracked-files=all`, named the file *after* removing it.
//!
//! # The class, and what iterates it
//!
//! * **the mechanism** — every production `git status` argv is built by one function,
//!   `cli::task::status_argv`, whose untracked mode is a required argument and whose
//!   rename detection is always off. [`every_production_git_status_argv_is_built_by_the_seam`]
//!   is the fence: a `"status"` literal anywhere else in production source is either a row
//!   of [`NOT_AN_ARGV`] with its reason, or a red test.
//! * **the configuration axis** — [`STATUS_CONFIGS`]: every `status.*` / `diff.*` key that
//!   moves a porcelain listing, alone and together.
//!   [`the_seam_reads_the_same_listing_under_every_status_configuration`] drives the seam's
//!   own argv, every mode, under each member against the unconfigured answer.
//! * **the door axis** — the three consumers of the refusal probe: `jigc milestone discard`,
//!   `jigc uninstall` and `jigc milestone provision` (which reaches it for a live worktree
//!   this repository has not registered — a copied repository).
//! * **must not refuse** — [`a_clean_fan_out_is_never_refused`]: a fan-out nobody edited
//!   clears every door, under the configuration that hides untracked files crossed with
//!   the conversion settings a checkout can be under, on a plain checkout, in a fresh
//!   clone, and driven from a linked worktree the user made. **Not covered, because these
//!   doors cannot be reached there:** a `--separate-git-dir` checkout — `jigc milestone
//!   create` exits 1 with git's `not a git repository` on this tree and on `1.0.0-rc.24`
//!   alike (driven), since `.jigc/` binds to the relocated git dir's parent — and, by the
//!   same mechanism (read, not driven), a worktree of a bare repository and a submodule.
//!
//! `git ls-files` is the other listing these doors read, and it is **not** a member: it
//! reads no `status.*` key (`--others` lists an untracked file whatever
//! `status.showUntrackedFiles` says — [`ls_files_reads_no_status_configuration`] drives
//! it), so its sixteen production sites inherit nothing from this class.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use cli::task::{Untracked, status_argv};

use crate::support::rust_source;

const MILESTONE: &str = "cache-rework";
const SUB: &str = "area-low";

/// One member of the configuration axis: its label, where it is written, and its pairs.
type ConfigMember = (&'static str, Scope, &'static [(&'static str, &'static str)]);

/// The configuration axis: each member is a set of `git config` pairs a user may carry,
/// at the scope named. Every key here changes what a `git status --porcelain` prints when
/// the caller leaves the matching flag out.
const STATUS_CONFIGS: &[ConfigMember] = &[
    (
        "showUntrackedFiles=no (repository)",
        Scope::Repository,
        &[("status.showUntrackedFiles", "no")],
    ),
    (
        "showUntrackedFiles=no (the user's ~/.gitconfig)",
        Scope::Global,
        &[("status.showUntrackedFiles", "no")],
    ),
    (
        "showUntrackedFiles=all",
        Scope::Repository,
        &[("status.showUntrackedFiles", "all")],
    ),
    (
        "renames=copies",
        Scope::Repository,
        &[("status.renames", "copies"), ("diff.renames", "copies")],
    ),
    (
        "every status.* key at once",
        Scope::Repository,
        &[
            ("status.showUntrackedFiles", "no"),
            ("status.renames", "copies"),
            ("status.renameLimit", "1"),
            ("status.branch", "true"),
            ("status.short", "true"),
            ("status.relativePaths", "false"),
            ("status.aheadBehind", "false"),
            ("status.showStash", "true"),
            ("status.submoduleSummary", "true"),
            ("status.displayCommentPrefix", "true"),
            ("color.status", "always"),
        ],
    ),
];

/// Where a configuration member is written.
#[derive(Clone, Copy, PartialEq)]
enum Scope {
    /// `git config` in the repository — shared by every linked worktree of it.
    Repository,
    /// `$HOME/.gitconfig` of the user running `jigc`.
    Global,
}

/// The conversion settings a checkout can be under — the axis the audit exposed for every
/// guard that asks git about working-tree bytes. Each is applied **before** the first
/// commit, so the fan-out worktrees are checked out under it.
#[derive(Clone, Copy, Debug)]
enum Conversion {
    None,
    AutocrlfTrue,
    AutocrlfInput,
    TextAuto,
    /// The committed blob itself holds CRLF, under `core.autocrlf=true`.
    CrlfBlob,
    /// An identity clean/smudge filter on every `.md` file.
    Filter,
}

const CONVERSIONS: [Conversion; 6] = [
    Conversion::None,
    Conversion::AutocrlfTrue,
    Conversion::AutocrlfInput,
    Conversion::TextAuto,
    Conversion::CrlfBlob,
    Conversion::Filter,
];

/// The repository layouts the fan-out doors can be driven in.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Layout {
    Plain,
    FreshClone,
    /// The doors are run from inside a linked worktree the user made; the fan-out still
    /// lives under the main checkout's `.jigc/`.
    UserLinkedWorktree,
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-status-flags-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path.canonicalize().expect("canonicalize temp dir"))
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

fn git(cwd: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git")
}

fn git_ok(cwd: &Path, args: &[&str]) -> String {
    let out = git(cwd, args);
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A repository carrying `milestone:cache-rework` with one provisioned sub-task.
struct Fixture {
    root: TempDir,
    home: TempDir,
    repo: PathBuf,
    /// Where the doors are run from — `repo`, or the user's linked worktree of it.
    cwd: PathBuf,
}

impl Fixture {
    fn mint(tag: &str, layout: Layout, conversion: Conversion) -> Self {
        let root = TempDir::new(tag);
        let seed = root.path().join("seed");
        fs::create_dir_all(&seed).expect("mk seed");
        git_ok(&seed, &["init", "-q", "-b", "main"]);
        identify(&seed);
        let mut readme: &[u8] = b"hello\nworld\n";
        match conversion {
            Conversion::None => {}
            Conversion::AutocrlfTrue => {
                git_ok(&seed, &["config", "core.autocrlf", "true"]);
            }
            Conversion::AutocrlfInput => {
                git_ok(&seed, &["config", "core.autocrlf", "input"]);
            }
            Conversion::TextAuto => {
                fs::write(seed.join(".gitattributes"), "* text=auto\n").expect("attributes");
            }
            Conversion::CrlfBlob => readme = b"hello\r\nworld\r\n",
            Conversion::Filter => {
                git_ok(&seed, &["config", "filter.same.clean", "cat"]);
                git_ok(&seed, &["config", "filter.same.smudge", "cat"]);
                fs::write(seed.join(".gitattributes"), "*.md filter=same\n").expect("attributes");
            }
        }
        fs::write(seed.join("README.md"), readme).expect("write README");
        git_ok(&seed, &["add", "."]);
        git_ok(&seed, &["commit", "-q", "-m", "initial"]);
        if matches!(conversion, Conversion::CrlfBlob) {
            // The blob holds CRLF as committed; the conversion setting arrives afterwards,
            // which is how a repository gets into this state.
            git_ok(&seed, &["config", "core.autocrlf", "true"]);
        }

        let repo = match layout {
            Layout::Plain | Layout::UserLinkedWorktree => seed,
            Layout::FreshClone => {
                let clone = root.path().join("clone");
                git_ok(
                    root.path(),
                    &[
                        "clone",
                        "-q",
                        seed.to_str().unwrap(),
                        clone.to_str().unwrap(),
                    ],
                );
                identify(&clone);
                // A clone inherits no local configuration, so the conversion is restated.
                match conversion {
                    Conversion::AutocrlfTrue | Conversion::CrlfBlob => {
                        git_ok(&clone, &["config", "core.autocrlf", "true"]);
                    }
                    Conversion::AutocrlfInput => {
                        git_ok(&clone, &["config", "core.autocrlf", "input"]);
                    }
                    Conversion::Filter => {
                        git_ok(&clone, &["config", "filter.same.clean", "cat"]);
                        git_ok(&clone, &["config", "filter.same.smudge", "cat"]);
                    }
                    Conversion::None | Conversion::TextAuto => {}
                }
                clone
            }
        };
        crate::support::mint_project_layer(&repo);
        // The directory the doors are run from.
        let cwd = match layout {
            Layout::Plain | Layout::FreshClone => repo.clone(),
            Layout::UserLinkedWorktree => {
                let linked = root.path().join("linked");
                git_ok(
                    &repo,
                    &[
                        "worktree",
                        "add",
                        "-q",
                        "-b",
                        "side",
                        linked.to_str().unwrap(),
                    ],
                );
                linked
            }
        };

        let fx = Fixture {
            root,
            home: TempDir::new("home"),
            repo,
            cwd,
        };
        fx.jigc_ok(&["milestone", "create", "Cache rework"]);
        fx.jigc_ok(&["milestone", "add-task", MILESTONE, "Area low"]);
        fx.jigc_ok(&["milestone", "provision", MILESTONE]);
        fx
    }

    fn worktree(&self) -> PathBuf {
        self.repo.join(".jigc").join("worktrees").join(SUB)
    }

    /// Apply one configuration member at its scope.
    fn configure(&self, scope: Scope, pairs: &[(&str, &str)]) {
        match scope {
            Scope::Repository => {
                for (key, value) in pairs {
                    git_ok(&self.repo, &["config", key, value]);
                }
            }
            Scope::Global => {
                let file = self.home.path().join(".gitconfig");
                for (key, value) in pairs {
                    git_ok(
                        &self.repo,
                        &["config", "--file", file.to_str().unwrap(), key, value],
                    );
                }
            }
        }
    }

    fn run_in(&self, cwd: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(cwd)
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    fn run(&self, args: &[&str]) -> Output {
        self.run_in(&self.cwd, args)
    }

    fn jigc_ok(&self, args: &[&str]) {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "`jigc {}` must exit 0; stderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

fn identify(repo: &Path) {
    git_ok(repo, &["config", "user.email", "test@example.com"]);
    git_ok(repo, &["config", "user.name", "Test"]);
    git_ok(repo, &["config", "commit.gpgsign", "false"]);
}

fn copy_tree(src: &Path, dst: &Path) {
    let copied = Command::new("cp")
        .arg("-R")
        .arg(src)
        .arg(dst)
        .output()
        .expect("run cp");
    assert!(copied.status.success(), "cp -R {src:?} {dst:?} failed");
}

// ---------------------------------------------------------------------------
// Must refuse: an untracked file is work, whatever the configuration hides.
// ---------------------------------------------------------------------------

/// **An untracked file in a sub-task worktree refuses every un-forced destroying door, under
/// every member of the configuration axis** — and the file is still there afterwards.
///
/// One fixture per member carries all three doors: `discard` and `uninstall` refuse and
/// change nothing, so the second is asked of the state the first left; `provision` reaches
/// the refusal probe only for a live worktree this repository has not registered, which is
/// a `cp -R` copy of the repository.
#[test]
fn an_untracked_file_refuses_every_destroying_door_under_every_status_configuration() {
    for (label, scope, pairs) in STATUS_CONFIGS {
        let fx = Fixture::mint("refuse", Layout::Plain, Conversion::None);
        fx.configure(*scope, pairs);
        let work = fx.worktree().join("untracked-work.rs");
        fs::write(&work, "the sub-agent's only copy\n").expect("write untracked work");

        for (door, args, code) in [
            (
                "milestone discard",
                vec!["milestone", "discard", MILESTONE],
                "milestone.dirty-worktree",
            ),
            ("uninstall", vec!["uninstall"], "uninstall.dirty-worktree"),
        ] {
            let out = fx.run(&args);
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            assert!(
                !out.status.success() && stderr.contains(code),
                "{label} × {door}: an untracked file is work the door must refuse over \
                 (`{code}`); got {:?}\nstderr:\n{stderr}",
                out.status,
            );
            assert!(
                stderr.contains("untracked-work.rs"),
                "{label} × {door}: the refusal must name the file; stderr:\n{stderr}",
            );
            assert_eq!(
                fs::read_to_string(&work).ok().as_deref(),
                Some("the sub-agent's only copy\n"),
                "{label} × {door}: a refusing door must leave the file exactly as it was",
            );
        }

        // The third consumer: a copied repository, whose worktrees are live and registered
        // at the source's path — so `provision` in the copy asks the refusal probe before
        // it clears the path for a worktree of its own.
        let copy = fx.root.path().join("copy");
        copy_tree(&fx.repo, &copy);
        let copied_work = copy
            .join(".jigc")
            .join("worktrees")
            .join(SUB)
            .join("untracked-work.rs");
        let out = fx.run_in(&copy, &["milestone", "provision", MILESTONE]);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            !out.status.success() && stderr.contains("milestone.leftover-holds-work"),
            "{label} × milestone provision (copied repository): the leftover holds an \
             untracked file and must refuse; got {:?}\nstderr:\n{stderr}",
            out.status,
        );
        assert!(
            copied_work.is_file(),
            "{label} × milestone provision: …and the file must still be there",
        );
    }
}

// ---------------------------------------------------------------------------
// Must NOT refuse: a fan-out nobody edited clears.
// ---------------------------------------------------------------------------

/// **A clean, unedited fan-out is refused by no door** — the configuration that hides
/// untracked files crossed with every conversion setting, on each layout these doors run
/// in. A guard that reads git's own status inherits git's own verdict on a converted
/// checkout, and that verdict is *clean*.
///
/// `uninstall` is asked first where the cell asks both: it removes `.jigc/` whole, so a
/// second fixture carries `discard`.
#[test]
fn a_clean_fan_out_is_never_refused() {
    let hidden: &[(&str, &str)] = &[("status.showUntrackedFiles", "no")];
    let mut cells: Vec<(Layout, Conversion)> = CONVERSIONS
        .iter()
        .map(|conversion| (Layout::Plain, *conversion))
        .collect();
    // The other layouts under the two conversions that rewrite bytes on checkout.
    for layout in [Layout::FreshClone, Layout::UserLinkedWorktree] {
        cells.push((layout, Conversion::None));
        cells.push((layout, Conversion::AutocrlfTrue));
    }
    for (layout, conversion) in cells {
        let cell = format!("{layout:?} × {conversion:?}");
        let fx = Fixture::mint("clean", layout, conversion);
        fx.configure(Scope::Repository, hidden);
        assert_eq!(
            git_ok(
                &fx.worktree(),
                &["status", "--porcelain", "--untracked-files=all"]
            ),
            "",
            "{cell}: fixture — git itself must read the fresh fan-out worktree as clean",
        );
        let out = fx.run(&["milestone", "discard", MILESTONE]);
        assert!(
            out.status.success(),
            "{cell}: an un-forced `milestone discard` over a clean fan-out must exit 0; \
             stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !fx.worktree().exists(),
            "{cell}: …and it must have removed the worktree",
        );

        if layout == Layout::Plain {
            let fx = Fixture::mint("clean-uninstall", layout, conversion);
            fx.configure(Scope::Repository, hidden);
            let out = fx.run(&["uninstall"]);
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            assert!(
                !stderr.contains("uninstall.dirty-worktree"),
                "{cell}: `uninstall` must not refuse over a clean fan-out worktree; \
                 stderr:\n{stderr}",
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The seam: one argv, the same listing under every configuration.
// ---------------------------------------------------------------------------

const MODES: [Untracked; 3] = [Untracked::No, Untracked::Normal, Untracked::All];

/// A working tree with one of everything a listing can differ over: an untracked file, a
/// wholly untracked directory, a staged rename, a staged add that is a copy of a file
/// modified beside it (what `status.renames=copies` pairs), and an unstaged edit.
fn busy_repo(tag: &str) -> (TempDir, PathBuf) {
    let root = TempDir::new(tag);
    let repo = root.path().join("repo");
    fs::create_dir_all(&repo).expect("mk repo");
    git_ok(&repo, &["init", "-q", "-b", "main"]);
    identify(&repo);
    let body = "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\n";
    fs::write(repo.join("kept.txt"), body).expect("write");
    fs::write(repo.join("moved.txt"), body.replace("one", "uno")).expect("write");
    fs::write(repo.join("edited.txt"), "before\n").expect("write");
    git_ok(&repo, &["add", "."]);
    git_ok(&repo, &["commit", "-q", "-m", "initial"]);

    git_ok(&repo, &["mv", "moved.txt", "renamed.txt"]);
    fs::write(repo.join("copy.txt"), body).expect("write");
    fs::write(repo.join("kept.txt"), format!("{body}nine\n")).expect("write");
    git_ok(&repo, &["add", "copy.txt", "kept.txt"]);
    fs::write(repo.join("edited.txt"), "after\n").expect("write");
    fs::write(repo.join("loose.txt"), "untracked\n").expect("write");
    fs::create_dir_all(repo.join("notes")).expect("mk notes");
    fs::write(repo.join("notes").join("a.txt"), "untracked\n").expect("write");
    (root, repo)
}

/// `git <args>` in `repo` as the user whose home is `home` — stdout, verbatim.
fn git_as(repo: &Path, home: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The listing the seam's argv prints in `repo`, `-z`-terminated so nothing is quoted.
fn seam_listing(repo: &Path, home: &Path, mode: Untracked) -> String {
    let mut args: Vec<&str> = status_argv(mode).to_vec();
    args.push("-z");
    git_as(repo, home, &args)
}

/// **The seam's argv prints one listing whatever the user configured** — every untracked
/// mode, under each member of [`STATUS_CONFIGS`], against the unconfigured answer.
///
/// The control is the point of the last assertion: the *bare* `git status --porcelain` this
/// seam replaces does change under the same member, so the axis is one that bites.
#[test]
fn the_seam_reads_the_same_listing_under_every_status_configuration() {
    for (label, scope, pairs) in STATUS_CONFIGS {
        let (_root, repo) = busy_repo("seam");
        let home = TempDir::new("seam-home");
        let plain: Vec<String> = MODES
            .iter()
            .map(|mode| seam_listing(&repo, home.path(), *mode))
            .collect();
        assert!(
            plain[2].contains("?? notes/a.txt\0") && plain[1].contains("?? notes/\0"),
            "fixture: the modes must differ where they should; got {plain:?}",
        );
        assert!(
            !plain[0].contains("??"),
            "fixture: `No` lists no untracked path; got {:?}",
            plain[0],
        );
        assert!(
            plain[2].contains("D  moved.txt\0") && plain[2].contains("A  renamed.txt\0"),
            "the seam never pairs a rename: both halves are their own record; got {:?}",
            plain[2],
        );
        let bare_before = git_as(&repo, home.path(), &["status", "--porcelain", "-z"]);

        match scope {
            Scope::Repository => {
                for (key, value) in *pairs {
                    git_ok(&repo, &["config", key, value]);
                }
            }
            Scope::Global => {
                let file = home.path().join(".gitconfig");
                for (key, value) in *pairs {
                    git_ok(
                        &repo,
                        &["config", "--file", file.to_str().unwrap(), key, value],
                    );
                }
            }
        }
        for (mode, before) in MODES.iter().zip(&plain) {
            assert_eq!(
                &seam_listing(&repo, home.path(), *mode),
                before,
                "{label} × {mode:?}: the seam's listing must not move with the configuration",
            );
        }
        let bare_after = git_as(&repo, home.path(), &["status", "--porcelain", "-z"]);
        if pairs.iter().any(|(key, value)| {
            (*key == "status.showUntrackedFiles" && *value != "normal") || key.ends_with("renames")
        }) {
            assert_ne!(
                bare_after, bare_before,
                "{label}: control — the bare argv the seam replaces must move under this \
                 member, or the axis proves nothing",
            );
        }
    }
}

/// **`git ls-files` reads no `status.*` key** — the disposition of the other listing these
/// doors read. `--others` names an untracked file with `status.showUntrackedFiles=no` set,
/// so no `ls-files` site can inherit this class.
#[test]
fn ls_files_reads_no_status_configuration() {
    let (_root, repo) = busy_repo("ls-files");
    let before = git_ok(&repo, &["ls-files", "--others", "--exclude-standard", "-z"]);
    assert!(before.contains("loose.txt\0"), "fixture: {before:?}");
    for (_, _, pairs) in STATUS_CONFIGS {
        for (key, value) in *pairs {
            git_ok(&repo, &["config", key, value]);
        }
    }
    assert_eq!(
        git_ok(&repo, &["ls-files", "--others", "--exclude-standard", "-z"]),
        before,
        "`git ls-files --others` must list the same paths under every status.* key",
    );
    assert_eq!(
        git_ok(&repo, &["status", "--porcelain", "-z"])
            .matches("??")
            .count(),
        0,
        "control: the same configuration does hide them from a bare `git status`",
    );
}

// ---------------------------------------------------------------------------
// The mechanism, fenced where membership is decided.
// ---------------------------------------------------------------------------

/// The function that builds every production `git status` argv.
const SEAM: (&str, &str) = ("crates/cli/src/task.rs", "status_argv");

/// Every other production `"status"` string literal, with why it is not a `git status`
/// argv. A literal that is in neither [`SEAM`] nor a row here reddens the fence: a new
/// `git status` goes through the seam, and a new non-argv use states itself.
const NOT_AN_ARGV: &[(&str, &str, &str)] = &[
    (
        "crates/cli/src/cli.rs",
        "verb_kind",
        "the unknown-verb guesser's word lists (`READ_INTENT_GUESSES` and a `SiblingTip` \
         row): `status` is a verb people type, not one jigc runs — the consts follow \
         `verb_kind`, which is why the scan names that function",
    ),
    (
        "crates/cli/src/setup.rs",
        "ask_dirty_against_head",
        "`ASKED`, the name the install guard's refusal gives the question it could not put \
         to git — the argv beside it is the seam's",
    ),
    (
        "crates/engine/src/milestone.rs",
        "<top level>",
        "`RECORD_STATUS_FIELD`, the milestone record's `status` header field",
    ),
];

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root is two levels above crates/cli")
        .to_path_buf()
}

/// **No production source spells a `git status` argv of its own.**
#[test]
fn every_production_git_status_argv_is_built_by_the_seam() {
    let mut in_seam = 0usize;
    let mut unlisted: Vec<String> = Vec::new();
    let mut used_rows: Vec<bool> = vec![false; NOT_AN_ARGV.len()];
    for dir in ["crates/cli/src", "crates/engine/src"] {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            let body = fs::read_to_string(&path).expect("read source");
            let code = rust_source::code_only(&body);
            let regions = rust_source::cfg_test_regions(&code);
            let rel = path
                .strip_prefix(workspace_root())
                .expect("under the workspace")
                .to_string_lossy()
                .into_owned();
            for literal in rust_source::string_literals(&body) {
                if literal.value != "status"
                    || rust_source::is_test_domain(&path, &regions, literal.offset)
                {
                    continue;
                }
                let owner = rust_source::enclosing_fn(&code, literal.offset)
                    .unwrap_or("<top level>")
                    .to_owned();
                if (rel.as_str(), owner.as_str()) == SEAM {
                    in_seam += 1;
                    continue;
                }
                match NOT_AN_ARGV
                    .iter()
                    .position(|(file, f, _)| *file == rel && *f == owner)
                {
                    Some(row) => used_rows[row] = true,
                    None => unlisted.push(format!("  {rel}::{owner}")),
                }
            }
        }
    }
    assert_eq!(
        in_seam, 1,
        "`{}::{}` must hold the one `\"status\"` argv element",
        SEAM.0, SEAM.1,
    );
    assert!(
        unlisted.is_empty(),
        "these production sites hold a `\"status\"` literal outside `cli::task::status_argv` \
         — a `git status` built by hand inherits the user's `status.*` configuration \
         (`status.showUntrackedFiles=no` hides every untracked file). Build the argv with \
         `status_argv(<Untracked mode>)`, or add a row to `NOT_AN_ARGV` saying why it is \
         not one:\n{}",
        unlisted.join("\n"),
    );
    for ((file, owner, reason), used) in NOT_AN_ARGV.iter().zip(used_rows) {
        assert!(
            !reason.trim().is_empty(),
            "{file}::{owner} carries no reason"
        );
        assert!(
            used,
            "`NOT_AN_ARGV` names {file}::{owner} and no `\"status\"` literal is there — a \
             stale row",
        );
    }
}
