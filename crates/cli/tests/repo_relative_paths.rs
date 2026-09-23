//! M50 Increment 12 / T1 — **a finding's path is repo-relative, and every absolute one that
//! stays says why** (RC-m50 → N25; `design/surface-contract.md` → law 1: *every printed path
//! is repo-real or a typed identity*; `design/write-commands.md` → `jigc rename`'s
//! untrackable-destination arm: *a surface prints no host filesystem*).
//!
//! The pre-v1 trial found the four destroying/provisioning doors naming their subject with the
//! **host** path of the machine they ran on:
//!
//! ```text
//! blocking · milestone.leftover-holds-work — milestone:m: `/private/var/folders/nj/…/T/
//!   jigc-rig-fresh-NAeKi1/repo/.jigc/worktrees/do-a-thing` already holds 1 item(s) …
//!   at: /private/var/folders/nj/…/repo/.jigc/worktrees/do-a-thing
//! ```
//!
//! That is a law-1 break twice over. The locus is the address a driver keys and a reader
//! pastes, and an absolute one is **not portable across the two checkouts of the same repo**
//! that a fan-out is made of; and the same door already prints `.jigc/worktrees/<id>` in its
//! `Spawn:` line, so one screen named one path two ways.
//!
//! **The class is derived, not taken from the report.** Its domain is *what the four doors
//! print*: every production site reachable from `cli::milestone::WORKTREE_DOORS` (the
//! worktree-shaped subset of `DESTROYING_DOORS`, which M52 widened past this suite's four)
//! and from `jigc milestone provision` that renders a filesystem path into a finding (message, locus or
//! route) or into a door's narration. Walking it earned three sites the report did not name —
//! `child_names`' `with_context`, whose bytes ride **verbatim inside** two findings' messages;
//! `remove_milestone_area`'s self-heal note; and `partial_worktree_advisories`, a fifth door's
//! locus in the same shape — and it retired one the report did name, because `dirty_worktrees`
//! is quoting a subprocess invocation rather than addressing a doc.
//!
//! **The declared absolutes are five, and the plan named three.** Two of the plan's three sit
//! **outside** the doors' domain and are in the table anyway, because the plan asked for them
//! to be recorded decisions rather than holes: `--explain`'s `Pack input:` resolving path and
//! the orientation header's cascade homes, neither of which has a repo-relative spelling at
//! all (a `JIGC_PACK_DIR` pack and the team layer live outside the repository). The third is
//! the pack-load freeze block. The derivation added the two the plan did not reach, both
//! inside the doors' own call graph (`milestone::dirty_worktrees`,
//! `milestone::remove_worktrees`).
//!
//! **Domain boundary, stated so it is not mistaken for a sweep of the binary.** `jigc setup`'s
//! own surfaces (the installed-hook report, the probe-extract refusal, the hook script body)
//! render paths too and are **out of this class**: they are a different door's text, with a
//! different subject — the installing binary and the git hooks dir, neither of which is a repo
//! path — and folding them in would put a rule written for *the doc a finding addresses* over
//! *the machine jigc is installed on*. `setup::dirty_worktree_finding` and
//! `setup::workbench_paths` are in, because they are `jigc uninstall`'s own text.
//!
//! **Widened by the M50 completion audit (finding 3), which found the rule enforced HERE and
//! nowhere else.** Two sites outside the four doors printed the host filesystem: `jigc config
//! set placement-root`, through `crate::trackable::untrackable_reason` — the *shared*
//! predicate that hands its text to four doors, three of whose five reasons composed an
//! absolute, and which Increment 12 / T3 had recorded by name while teaching `rename` alone to
//! compose around it — and `store.not-found` on the **1.0-pinned** `doc show --format json`
//! read contract, whose own `route:` already spelled the same doc repo-relative, so one JSON
//! object named one file two ways. The predicate is fixed, not its caller; the read path is
//! fixed at both of its blocks and in `add-from-spec`'s copy of them; and the standing fence's
//! subject becomes a **list of guarded modules** rather than one file, because a fence whose
//! subject is one file is a fence a sibling walks around.
//!
//! Five arms:
//!
//! 1. the **driven** arm — a door table over a fixture root minted by `mktemp -d`, each row
//!    run for real and its whole stdout+stderr scanned for that root's absolute prefix;
//! 2. the **driven arm outside the four doors** — the same scan over `config set
//!    placement-root` (each of the predicate's three absolute-composing reasons) and over the
//!    committed read path's two `store.*` blocks on the pinned JSON contract;
//! 3. the **disposition** arm — one row per site, `Relative` or `DeclaredAbsolute(reason)`,
//!    each verdict checked against the source rather than believed (the
//!    `located_finding_text::MESSAGE_SITES` third-verdict idiom);
//! 4. the **standing fence** — no production `.display()` in any `GUARDED_SRC` module outside
//!    a `DeclaredAbsolute` site, so the next hand-rolled absolute render reddens here rather
//!    than shipping;
//! 5. the **counted remainder** — what this class has NOT swept, one row per file with its
//!    measured site count, so the bound is checked against the source instead of asserted in
//!    prose.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::rust_source;

// ---------------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------------

/// A throwaway root minted by **`mktemp -d`** — the fixture whose absolute prefix the driven
/// arm scans for. It is deliberately the shell's own mint rather than a hand-built temp name:
/// the property under test is *"no host path reaches the surface"*, and a root nothing in the
/// repo can predict is the only fixture that can prove it.
struct MkTemp(PathBuf);

impl MkTemp {
    fn new() -> Self {
        // The template carries the shared temp-mint seam (`temp_mint_fence`) as well as
        // `mktemp`'s own `XXXXXX`: `mktemp` guarantees the suffix, and the seam guarantees the
        // prefix cannot be shared with a sibling test in this binary.
        let template = std::env::temp_dir().join(format!(
            "jigc-repo-relative-{}-{:?}-XXXXXX",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        let out = Command::new("mktemp")
            .arg("-d")
            .arg(template.as_os_str())
            .output()
            .expect("run mktemp -d");
        assert!(
            out.status.success(),
            "mktemp -d failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        let path = String::from_utf8(out.stdout).expect("utf-8 mktemp stdout");
        MkTemp(PathBuf::from(path.trim()))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for MkTemp {
    fn drop(&mut self) {
        // Registered worktrees inside the tree are ordinary directories to `remove_dir_all`.
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

/// Initialize a real git repo with one commit (the milestone mint reads HEAD).
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    git_ok(root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(root);
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

/// Commit the non-transient `.jigc/` files a real `jigc setup` would already have committed,
/// so `jigc uninstall`'s untracked-workbench guard does not answer an arm declared on the
/// **worktree** axis (the `uninstall_worktree_guard::commit_workbench` convention).
fn commit_workbench(repo: &Path) {
    let mut args: Vec<&str> = vec!["add", "--force", "--"];
    for rel in [".jigc/.gitignore", ".jigc/config"] {
        if repo.join(rel).exists() {
            args.push(rel);
        }
    }
    if args.len() == 3 {
        return;
    }
    git_ok(repo, &args);
    let clean = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .current_dir(repo)
        .status()
        .expect("run git diff --cached")
        .success();
    if !clean {
        git_ok(repo, &["commit", "-q", "-m", "workbench"]);
    }
}

/// What sits at `.jigc/worktrees/<sub>` when the door is asked — the two shapes the shared
/// leftover probe answers differently, and therefore the two message families each door can
/// print.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Leftover {
    /// A directory holding a file nothing vouches for — the probed refusal.
    Directory,
    /// A plain file where a worktree directory belongs — the fail-closed refusal, whose text
    /// carries `child_names`' own `with_context` bytes.
    File,
}

/// `<root>/repo` + `<root>/home`: a git repo with one milestone, one sub-task, and `leftover`
/// planted at that sub-task's worktree path.
fn fixture(root: &Path, tag: &str, leftover: Leftover) -> (PathBuf, PathBuf) {
    let repo = root.join(format!("repo-{tag}"));
    let home = root.join(format!("home-{tag}"));
    fs::create_dir_all(&repo).expect("mk repo");
    fs::create_dir_all(&home).expect("mk home");
    init_repo(&repo);

    let out = run_jigc(&repo, &home, &["milestone", "create", "Cache Rework"]);
    assert!(
        out.status.success(),
        "milestone create must exit 0: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let out = run_jigc(
        &repo,
        &home,
        &["milestone", "add-task", "cache-rework", "Area zed"],
    );
    assert!(
        out.status.success(),
        "milestone add-task must exit 0: {}",
        String::from_utf8_lossy(&out.stderr),
    );

    let worktrees = repo.join(".jigc").join("worktrees");
    fs::create_dir_all(&worktrees).expect("mk worktrees root");
    let path = worktrees.join("area-zed");
    match leftover {
        Leftover::Directory => {
            fs::create_dir_all(&path).expect("mk leftover dir");
            fs::write(path.join("precious.txt"), "sole copy\n").expect("write leftover");
        }
        Leftover::File => {
            fs::write(&path, "sole copy\n").expect("write leftover file");
        }
    }
    commit_workbench(&repo);
    (repo, home)
}

// ---------------------------------------------------------------------------------
// Arm 1 — the driven arm
// ---------------------------------------------------------------------------------

/// One driven cell: a door, its argv, and the leftover shape it is asked over.
struct Door {
    label: &'static str,
    argv: &'static [&'static str],
    leftover: Leftover,
}

/// The doors the trial drove, each over both leftover shapes — eight cells, because the two
/// shapes reach two different message families (the probed refusal and the fail-closed one)
/// and only one of them was in the report.
const DOORS: &[Door] = &[
    Door {
        label: "milestone provision",
        argv: &["milestone", "provision", "cache-rework"],
        leftover: Leftover::Directory,
    },
    Door {
        label: "milestone provision --force",
        argv: &["milestone", "provision", "cache-rework", "--force"],
        leftover: Leftover::Directory,
    },
    Door {
        label: "milestone discard",
        argv: &["milestone", "discard", "cache-rework"],
        leftover: Leftover::Directory,
    },
    Door {
        label: "uninstall",
        argv: &["uninstall"],
        leftover: Leftover::Directory,
    },
    Door {
        label: "milestone provision (file leftover)",
        argv: &["milestone", "provision", "cache-rework"],
        leftover: Leftover::File,
    },
    Door {
        label: "milestone provision --force (file leftover)",
        argv: &["milestone", "provision", "cache-rework", "--force"],
        leftover: Leftover::File,
    },
    Door {
        label: "milestone discard (file leftover)",
        argv: &["milestone", "discard", "cache-rework"],
        leftover: Leftover::File,
    },
    Door {
        label: "uninstall (file leftover)",
        argv: &["uninstall"],
        leftover: Leftover::File,
    },
];

/// Both spellings of `root` a surface could carry: the path as minted, and the canonicalized
/// one git and `provision` resolve it to (on macOS `/var/…` lists as `/private/var/…`).
fn host_prefixes(root: &Path) -> Vec<String> {
    let mut out = vec![root.to_string_lossy().into_owned()];
    if let Ok(real) = root.canonicalize() {
        let real = real.to_string_lossy().into_owned();
        if !out.contains(&real) {
            out.push(real);
        }
    }
    out
}

#[test]
fn no_door_prints_the_host_path_of_the_machine_it_ran_on() {
    let root = MkTemp::new();
    let prefixes = host_prefixes(root.path());
    let mut offenders: Vec<String> = Vec::new();

    for (i, door) in DOORS.iter().enumerate() {
        let (repo, home) = fixture(root.path(), &format!("d{i}"), door.leftover);
        let out = run_jigc(&repo, &home, door.argv);
        let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&out.stderr));

        for line in text.lines() {
            if let Some(prefix) = prefixes.iter().find(|p| line.contains(p.as_str())) {
                offenders.push(format!(
                    "  `jigc {}` [{:?}] printed the host path `{prefix}`:\n      {line}",
                    door.argv.join(" "),
                    door.leftover,
                ));
            }
        }
        // The cell has to reach the door's own text, or it proves nothing: an arm that
        // silently exited 0 with an empty screen would pass the scan above vacuously.
        assert!(
            !text.trim().is_empty(),
            "`{}` printed nothing — the cell reached no surface to check",
            door.label,
        );
    }

    assert!(
        offenders.is_empty(),
        "a finding's locus is the address a reader pastes and a driver keys, and a host path \
         is neither repo-real nor a typed identity (`design/surface-contract.md` → law 1; \
         `design/write-commands.md` → *a surface prints no host filesystem*) — it is also not \
         portable across the two checkouts of the same repo a fan-out is made of:\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------------
// Arm 2 — the driven arm, outside the four doors
// ---------------------------------------------------------------------------------

/// What the fixture plants so a cell can reach its refusal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Plant {
    /// Nothing beyond `jigc setup`.
    Nothing,
    /// A second, embedded git repository at `vendored/` — the predicate's **ownership**
    /// reason, which composes the owning repo's toplevel.
    EmbeddedRepo,
    /// The repository re-pointed at a git dir that is **inside the tree but not called
    /// `.git`** — the only shape that reaches the predicate's *git-dir* reason, since the
    /// literal-`.git`-component reason answers first for every ordinary layout.
    SeparateGitDir,
    /// A committed-store file at the `adr` home that does not parse — the read path's
    /// `store.unparseable` block.
    BrokenAdr,
    /// A **directory** at a repo-relative name the operator can type — the shape that
    /// makes `jigc migrate <path>`'s read fault fire over a token that exists, so the
    /// cell cannot be mistaken for a typo answering on some other axis.
    ForeignDir,
}

/// A door outside `DESTROYING_DOORS`, its argv, and the plant it is asked over.
struct ReadDoor {
    label: &'static str,
    argv: &'static [&'static str],
    plant: Plant,
}

/// **The cells the M50 completion audit's finding 3 drove, plus the ones the derivation
/// earned.** The report named two: `config set placement-root ..` and `doc show <missing>
/// --format json`. The class is *what the shared trackability predicate hands its four doors*
/// and *what the committed read path names a file with* — so the three absolute-composing
/// reasons are each driven through the one door that can reach them, and the read path is
/// driven on both of its blocks rather than only the reported one.
const READ_DOORS: &[ReadDoor] = &[
    ReadDoor {
        label: "config set placement-root .. (outside the repository)",
        argv: &["config", "set", "placement-root", ".."],
        plant: Plant::Nothing,
    },
    ReadDoor {
        label: "config set placement-root <embedded repo> (another repository)",
        argv: &["config", "set", "placement-root", "vendored"],
        plant: Plant::EmbeddedRepo,
    },
    ReadDoor {
        label: "config set placement-root <git dir> (this repository's git directory)",
        argv: &["config", "set", "placement-root", "gitstore"],
        plant: Plant::SeparateGitDir,
    },
    ReadDoor {
        label: "doc show <missing> --format json (store.not-found, the pinned contract)",
        argv: &["doc", "show", "adr:nosuch", "--format", "json"],
        plant: Plant::Nothing,
    },
    ReadDoor {
        label: "doc show <unparseable> --format json (store.unparseable, the same read path)",
        argv: &["doc", "show", "adr:broken", "--format", "json"],
        plant: Plant::BrokenAdr,
    },
    // Earned by the M51 completion audit (LOW 3): `migrate`'s read fault composed
    // `repo_root.join(path).display()`, so the operator who typed `adir` was answered with
    // `/private/var/…/repo/adir` — and the source comment two lines above it claimed the
    // opposite ("a read fault is the one refusal whose subject is the string the operator
    // typed"). The door is neither destroying nor a read contract, which is exactly why no
    // earlier arm reached it.
    ReadDoor {
        label: "migrate <dir> --as adr (the read fault on the operator's own token)",
        argv: &["migrate", "adir", "--as", "adr"],
        plant: Plant::ForeignDir,
    },
];

/// `<root>/repo` + `<root>/home`: a set-up jigc project with `plant` in place.
fn read_fixture(root: &Path, tag: &str, plant: Plant) -> (PathBuf, PathBuf) {
    let repo = root.join(format!("repo-{tag}"));
    let home = root.join(format!("home-{tag}"));
    fs::create_dir_all(&repo).expect("mk repo");
    fs::create_dir_all(&home).expect("mk home");
    init_repo(&repo);

    let out = run_jigc(&repo, &home, &["setup"]);
    assert!(
        out.status.success(),
        "jigc setup must exit 0: {}",
        String::from_utf8_lossy(&out.stderr),
    );

    match plant {
        Plant::Nothing => {}
        Plant::EmbeddedRepo => {
            let inner = repo.join("vendored");
            fs::create_dir_all(&inner).expect("mk embedded repo dir");
            git_ok(&inner, &["init", "-q"]);
        }
        Plant::SeparateGitDir => {
            // Re-init in place: git moves the object store to `gitstore/` and leaves `.git`
            // as a pointer file, so the destination `gitstore` is inside the working tree,
            // is git's own dir, and carries no `.git` path component.
            git_ok(&repo, &["init", "-q", "--separate-git-dir=gitstore"]);
        }
        Plant::BrokenAdr => {
            let decisions = repo.join("docs").join("decisions");
            fs::create_dir_all(&decisions).expect("mk decisions dir");
            fs::write(decisions.join("broken.md"), "not an adr at all\n").expect("write broken");
        }
        Plant::ForeignDir => {
            fs::create_dir_all(repo.join("adir")).expect("mk foreign dir");
        }
    }
    (repo, home)
}

#[test]
fn no_read_or_config_door_prints_the_host_path_of_the_machine_it_ran_on() {
    let root = MkTemp::new();
    let prefixes = host_prefixes(root.path());
    let mut offenders: Vec<String> = Vec::new();

    for (i, door) in READ_DOORS.iter().enumerate() {
        let (repo, home) = read_fixture(root.path(), &format!("r{i}"), door.plant);
        let out = run_jigc(&repo, &home, door.argv);
        let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&out.stderr));

        for line in text.lines() {
            if let Some(prefix) = prefixes.iter().find(|p| line.contains(p.as_str())) {
                offenders.push(format!(
                    "  `jigc {}` [{}] printed the host path `{prefix}`:\n      {line}",
                    door.argv.join(" "),
                    door.label,
                ));
            }
        }
        // The cell has to reach a refusal, or it proves nothing — a door that quietly
        // succeeded would pass the scan above vacuously.
        assert!(
            !out.status.success(),
            "`{}` must refuse — the cell reached no refusal to check:\n{text}",
            door.label,
        );
        assert!(
            !text.trim().is_empty(),
            "`{}` printed nothing — the cell reached no surface to check",
            door.label,
        );
    }

    assert!(
        offenders.is_empty(),
        "law 1 is stated universally (`design/surface-contract.md`: *every printed path is \
         repo-real or a typed identity*; `design/write-commands.md`: *a surface prints no host \
         filesystem*) — a shared predicate that hands four doors a host path, and a read \
         contract pinned at 1.0 that carries one, break it just as the destroying doors \
         did:\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------------
// Arm 2b — the kept pin-read producers (M53 Increment 3 / T5)
// ---------------------------------------------------------------------------------

/// How a **legitimate** task's base pin is broken — and therefore which producer the cell
/// reaches.
///
/// M53 Increment 3 / D3 made *carrying a base pin* the predicate every roster and every by-id
/// door asks, so a directory without one is a leftover at all of them. That **narrowed**
/// `cli::task::TaskArea::base`'s two fault arms; it did not delete them
/// (`completions/artifacts/M53/settle-record.md` → §9). The predicate is **existence, never a
/// parse**, so a legitimate area whose pin is torn, corrupt or unreadable passes the door and
/// still fails the read — and both of those arms named the pin with the host path of the
/// machine they ran on, driven on the debug binary at `b8ae481c`:
///
/// ```text
/// malformed base pin at "/private/var/folders/nj/…/repo/.jigc/tasks/probe-the-pin/base.json"
/// ```
///
/// The third cell is the **control that makes the narrowing itself checkable**: a removed pin
/// reaches no pin-read producer at all any more, because `require_task_area` answers
/// `finalize.no-task` with the residual sentence first. Dropping it would leave §9's "narrowed,
/// nothing deleted" as a sentence nothing drives.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PinFault {
    /// Truncated to invalid JSON. The pin is still a file, so the existence predicate passes
    /// and the **parse** arm answers.
    Torn,
    /// Present and unreadable (`chmod 000`). The existence predicate passes and the **read**
    /// arm answers — the sibling site, which the torn cell never reaches.
    Unreadable,
    /// Removed. The control: `require_task_area` answers before either arm is reached.
    Removed,
}

impl PinFault {
    /// The cells this platform can drive. [`PinFault::Unreadable`] needs unix permission
    /// semantics to make a file that exists and cannot be read, so it is the one cell that is
    /// gated rather than the whole arm — the other two hold everywhere.
    fn cells() -> Vec<PinFault> {
        let mut out = vec![PinFault::Torn];
        #[cfg(unix)]
        out.push(PinFault::Unreadable);
        out.push(PinFault::Removed);
        out
    }

    /// The repo-relative spelling the emitted bytes must carry. The two pin-read arms name the
    /// **pin**; the residual control names the **area**, because that is the path an operator
    /// clears by hand.
    fn must_name(self, id: &str) -> String {
        match self {
            PinFault::Torn | PinFault::Unreadable => format!(".jigc/tasks/{id}/base.json"),
            PinFault::Removed => format!(".jigc/tasks/{id}"),
        }
    }
}

/// `<root>/repo-<tag>` + `<root>/home-<tag>`: a git repo carrying **one legitimate task minted
/// through the real binary**, whose base pin is then broken as `fault` says.
///
/// The task is minted rather than planted on purpose: the claim is about the window a *real*
/// working area can be in, and a hand-built directory would prove nothing about it. The id
/// comes off the mint's own `task minted: <id>` line — never reconstructed from the intent,
/// because the slug rule is the binary's.
fn pin_fixture(root: &Path, tag: &str, fault: PinFault) -> (PathBuf, PathBuf, String) {
    let repo = root.join(format!("repo-{tag}"));
    let home = root.join(format!("home-{tag}"));
    fs::create_dir_all(&repo).expect("mk repo");
    fs::create_dir_all(&home).expect("mk home");
    init_repo(&repo);

    let out = run_jigc(
        &repo,
        &home,
        &["start", "--workflow", "single-task", "pin the base"],
    );
    assert!(
        out.status.success(),
        "the mint must exit 0, or the arm has no legitimate area to break: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let id = stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string();

    let pin = repo.join(".jigc").join("tasks").join(&id).join("base.json");
    assert!(
        pin.is_file(),
        "the mint must have written the pin this arm breaks",
    );
    match fault {
        PinFault::Torn => fs::write(&pin, "{\"sha\": ").expect("tear the pin"),
        PinFault::Unreadable => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&pin, fs::Permissions::from_mode(0o000))
                    .expect("make the pin unreadable");
                assert!(
                    fs::read(&pin).is_err(),
                    "this cell needs a pin that exists and cannot be read — `chmod 000` did \
                     not deny the read, which is what happens when the suite runs as root",
                );
            }
            #[cfg(not(unix))]
            unreachable!("the unreadable cell is unix-only — see `PinFault::cells`");
        }
        PinFault::Removed => fs::remove_file(&pin).expect("remove the pin"),
    }
    (repo, home, id)
}

/// The two by-id doors that read the pin, each driven in **both** surfaces — the agent text and
/// the `--format json` envelope. Four runs per cell, so a fix that reaches one stream and not
/// the other cannot pass.
const PIN_DOORS: &[&[&str]] = &[&["task", "diff"], &["task", "finalize"]];

/// §9's producers, driven: a legitimate task whose pin is broken names that pin **repo-relative**
/// on every stream, and the machine's own temp root reaches none of them.
///
/// `design/surface-contract.md` → law 1: *every printed path is repo-real or a typed identity*.
/// A locus is the address a driver keys and a reader pastes, and an absolute one is not portable
/// across the two checkouts of the same repo a fan-out is made of — which is exactly the shape
/// these two arms carried, one line away from a `require_task_area` refusal that already spelled
/// the same area `.jigc/tasks/<id>`.
#[test]
fn the_kept_pin_read_producers_name_the_pin_repo_relative() {
    let root = MkTemp::new();
    let prefixes = host_prefixes(root.path());
    let mut offenders: Vec<String> = Vec::new();

    for (i, fault) in PinFault::cells().into_iter().enumerate() {
        let (repo, home, id) = pin_fixture(root.path(), &format!("p{i}"), fault);
        let expected = fault.must_name(&id);

        for door in PIN_DOORS {
            for json in [false, true] {
                let mut argv: Vec<&str> = door.to_vec();
                argv.push(&id);
                if json {
                    argv.extend_from_slice(&["--format", "json"]);
                }
                let out = run_jigc(&repo, &home, &argv);
                let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
                text.push_str(&String::from_utf8_lossy(&out.stderr));

                // The cell has to reach a refusal, or it proves nothing.
                assert!(
                    !out.status.success(),
                    "`jigc {}` over a {fault:?} pin must refuse — the cell reached no refusal \
                     to check:\n{text}",
                    argv.join(" "),
                );
                assert!(
                    !text.trim().is_empty(),
                    "`jigc {}` over a {fault:?} pin printed nothing — the cell reached no \
                     surface to check",
                    argv.join(" "),
                );

                for line in text.lines() {
                    if let Some(prefix) = prefixes.iter().find(|p| line.contains(p.as_str())) {
                        offenders.push(format!(
                            "  `jigc {}` [{fault:?}] printed the host path `{prefix}`:\n      \
                             {line}",
                            argv.join(" "),
                        ));
                    }
                }
                if !text.contains(&expected) {
                    offenders.push(format!(
                        "  `jigc {}` [{fault:?}] never named `{expected}` — the repo-relative \
                         spelling is what law 1 asks for, and a surface that names the subject \
                         some other way has not been fixed:\n      {}",
                        argv.join(" "),
                        text.trim(),
                    ));
                }
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "law 1 binds the pin-read arms D3 narrowed but kept \
         (`completions/artifacts/M53/settle-record.md` → §9): a legitimate area whose pin is \
         torn or unreadable still fails the read, and that failure may not print the host \
         filesystem:\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------------
// Arm 3 — the disposition table
// ---------------------------------------------------------------------------------

/// What a site does with the path it renders.
#[derive(Clone, Copy, PartialEq)]
enum Disposition {
    /// It renders through the one shared home ([`REPO_RELATIVE`]) — repo-relative, or the
    /// honest absolute the helper itself falls back to for a path outside the repository.
    Relative,
    /// It renders the host path, on purpose, for the stated reason.
    DeclaredAbsolute,
}

/// `(file, fn, disposition, reason)` — every site in the class, with its verdict checked
/// against the source below rather than believed.
const PATH_TEXT_SITES: &[(&str, &str, Disposition, &str)] = &[
    // --- `jigc milestone provision` -------------------------------------------------
    (
        "crates/cli/src/milestone.rs",
        "provision_worktrees",
        Disposition::Relative,
        "phase 1 refuses at a worktree path and phase 2 clears one, and every failure of \
         either names it; the subject is under `.jigc/worktrees/` by construction, so it has \
         a repo-relative spelling always. The pre-flight `canonicalize` context is the one \
         path whose subject IS the jigc home, and the shared home renders that as `.` rather \
         than as the empty string",
    ),
    (
        "crates/cli/src/milestone.rs",
        "leftover_finding",
        Disposition::Relative,
        "the probed refusal's message AND its `at:` locus — the address the report named",
    ),
    (
        "crates/cli/src/milestone.rs",
        "hold_line",
        Disposition::Relative,
        "the one line every refusing door lists a held path with — the fail-closed cell \
         among them, which since M50 Increment 12 / T2 is a hold beside its siblings rather \
         than a producer of its own",
    ),
    (
        "crates/cli/src/milestone.rs",
        "provision_failed_finding",
        Disposition::Relative,
        "the phase-2 block's message AND its `at:` locus (the report named both halves)",
    ),
    (
        "crates/cli/src/milestone.rs",
        "child_names",
        Disposition::Relative,
        "earned by the derivation, not named in the report: its `with_context` bytes ride \
         VERBATIM inside every refusal's `hold_line` and inside \
         `setup::unverified_worktrees_finding`'s message, so a relative locus over an \
         absolute cause names one path two ways on one screen",
    ),
    (
        "crates/cli/src/milestone.rs",
        "partial_worktree_advisories",
        Disposition::Relative,
        "earned by the derivation: a fifth door (`jigc milestone execute`) addressing a \
         worktree path in the identical `Location::addressed` shape, which no report row \
         reached",
    ),
    // --- the narration every destroying door shares -----------------------------------
    (
        "crates/cli/src/milestone.rs",
        "narrate_removal",
        Disposition::Relative,
        "the one loss-narration emitter all four doors print through",
    ),
    (
        "crates/cli/src/milestone.rs",
        "doomed_at",
        Disposition::Relative,
        "the narration's probe: its `child_names` failure text IS the narration's text, and \
         its own one render is the file-name fallback for the pathological path that has \
         none — where `as_os_str()` would put the whole host path on the screen",
    ),
    // --- `jigc milestone discard` -----------------------------------------------------
    (
        "crates/cli/src/milestone.rs",
        "dirty_worktree_finding",
        Disposition::Relative,
        "the abandon's refusal: one listed line per held path, plus the `at:` locus",
    ),
    (
        "crates/cli/src/milestone.rs",
        "remove_milestone_area",
        Disposition::Relative,
        "earned by the derivation: the teardown's self-heal note, a stderr line no report row \
         reached",
    ),
    // --- `jigc milestone finalize`'s conformance gate (M53 completion audit, finding 5) ---
    (
        "crates/cli/src/milestone.rs",
        "milestone_boundary_gate",
        Disposition::Relative,
        "M53 Increment 1 / T1 added a `{path:?}` render here — the `Debug` spelling of a \
         `PathBuf`, in a module this suite lists as SWEPT — and nothing reddened, because the \
         standing fence read `.display()` only and arm 3 reads the functions this table names. \
         Driven, that context prints the host path of the machine the boundary ran on. All six \
         of the function's renders now reach the shared home: the gate staging area, the \
         merged docs dir, the entry it reads the shape of, the entry it stages, the file-state \
         record's root and the merged effective state's own directory. Two of the six leave \
         the repository (`ScratchTree` mints under the system temp dir), and the helper's \
         declared fallback renders those absolute, which is its honest answer rather than a \
         miss",
    ),
    // --- `jigc uninstall` --------------------------------------------------------------
    (
        "crates/cli/src/setup.rs",
        "dirty_worktree_finding",
        Disposition::Relative,
        "the teardown's refusal: one listed line per held path",
    ),
    (
        "crates/cli/src/setup.rs",
        "workbench_paths",
        Disposition::Relative,
        "the teardown's third subject — already repo-relative before this task, by a \
         hand-written strip; it now reads the shared home, which is what makes the home \
         shared rather than a seventh copy",
    ),
    // --- the staged-prose probe, three destroying doors' fail-closed message (M52 Inc 4/T7)
    (
        "crates/cli/src/task.rs",
        "staged_task_prose",
        Disposition::Relative,
        "the one probe behind `task-discard.staged-prose`, `uninstall.staged-prose` and \
         `milestone.staged-prose`; its `docs/` fault composed the host path into all three \
         **blocking** messages (D-3, driven on rc.15). The subject is always under \
         `<repo>/.jigc/tasks/`, so a repo-relative spelling exists by construction. \
         **Disposed, not driven, and the reason is measured:** since M52 Increment 4 / T5 no \
         door can reach this arm — the foreign-byte guard is asked first at all three and \
         probes a superset, because `staged_doc_ids` errors only where `std::fs::metadata` \
         fails on an entry whose name parses as a staged doc id, and every such entry is \
         `!file_type().is_file()`, which `engine::state::foreign_area_paths` calls foreign. \
         Law 1 binds the surface whether or not a door currently reaches it, so the site is \
         fixed and held here by disposition and by the standing fence",
    ),
    // --- the pin-read producers D3 narrowed and kept (M53 Increment 3 / T5) -----------
    (
        "crates/cli/src/task.rs",
        "base",
        Disposition::Relative,
        "the two fault arms `jigc task diff` and `jigc task finalize` reach when a \
         **legitimate** working area's base pin is torn, corrupt or unreadable. D3's own \
         predicate is *existence, never a parse*, so a residual no longer reaches them but \
         this window still does (`completions/artifacts/M53/settle-record.md` → §9) — and \
         both arms spelled the pin `{path:?}`, the host path of the machine they ran on, one \
         line away from a `require_task_area` refusal that already renders the same area \
         repo-relative. The subject is under `<jigc_home>/.jigc/tasks/` by construction, and \
         **jigc_home is the root it is rendered against**, not `repo_root`: the `.jigc/` \
         workbench binds to the main checkout, so a door called from a fanned-out worktree \
         would fall back to the absolute against the worktree's own root",
    ),
    // --- the shared trackability predicate (M50 audit, finding 3) ---------------------
    (
        "crates/cli/src/trackable.rs",
        "untrackable_reason",
        Disposition::Relative,
        "the one predicate `config set placement-root`, `jigc rename`, `jigc setup` and \
         `jigc relocate` all ask, three of whose five reasons composed the host path — the \
         repo root, git's own dir, and the owning repository's toplevel. `rename` had been \
         taught to compose its own message around it one door at a time; fixing the predicate \
         is what makes the other three doors right too. The `..` reason names no path at all \
         now: the subject IS the repository, whose repo-relative spelling is `.`, and `at .` \
         is noise",
    ),
    // --- the committed-doc read path, on the 1.0-pinned contract ----------------------
    (
        "crates/engine/src/store.rs",
        "read_slice",
        Disposition::Relative,
        "`store.not-found`'s message rides `jigc doc show --format json`, the read contract \
         pinned at 1.0 — and its own `route:` already spelled the same doc repo-relative, so \
         one JSON object named one file two ways",
    ),
    (
        "crates/engine/src/store.rs",
        "read_parse_slice",
        Disposition::Relative,
        "`store.unparseable`, the read path's other block, shared by the committed and the \
         staged arm — it takes the repo root so both arms render against the same origin",
    ),
    (
        "crates/engine/src/milestone.rs",
        "read_spec_criteria",
        Disposition::Relative,
        "earned by the derivation, not named in the report: `milestone add-from-spec` composes \
         the SAME two `store.*` blocks in the same shape from its own copy, so a fix at the \
         reported site alone would have left the class open one file over",
    ),
    // --- the five that stay absolute, each saying why ---------------------------------
    (
        "crates/cli/src/milestone.rs",
        "dirty_worktrees",
        Disposition::DeclaredAbsolute,
        "it is QUOTING AN INVOCATION, not addressing a doc: `\\`git status --porcelain\\` in \
         worktree <wt> failed: <git's own stderr>`. The path is the argument jigc handed git, \
         and git's words beside it name that same absolute — rewriting one half would \
         misquote the command that failed",
    ),
    (
        "crates/cli/src/milestone.rs",
        "remove_worktrees",
        Disposition::DeclaredAbsolute,
        "its warning is followed by the `git worktree remove --force <path>` that repairs it, \
         and git resolves a worktree path against the CALLER's cwd, so the remedy must be \
         pasteable from anywhere; the warning one line above names the same path the same \
         way, because one screen naming one path two ways is the law-1 break this task \
         closes, not a fix for it",
    ),
    (
        "crates/cli/src/pack.rs",
        "assert_project_schema_shadows",
        Disposition::DeclaredAbsolute,
        "pack-load has no repo-root subject to be relative to — the check receives a \
         project-config path, and it runs before any verb has resolved a repository — and its \
         `route:` span is bytes the operator pastes into a shell of unknown cwd (`rm <path>`, \
         shell-quoted). The message half stays absolute WITH the route, for the same reason \
         `remove_worktrees` does: one screen, one spelling",
    ),
    (
        "crates/cli/src/render.rs",
        "explain_agent_text",
        Disposition::DeclaredAbsolute,
        "`--explain`'s `Pack input:` line names the pack's RESOLVING path, and a \
         `JIGC_PACK_DIR` pack resolves outside the repository entirely \
         (`design/multi-pack.md` → Provenance under N packs) — there is no repo-relative \
         spelling of it, and the line's whole job is to say which bytes on this machine \
         produced the deterministic outcome",
    ),
    (
        "crates/engine/src/cascade.rs",
        "header",
        Disposition::DeclaredAbsolute,
        "the orientation header's `Project config:` / `Team config:` segments name the \
         cascade layers' homes, and the TEAM layer is a cross-project home outside the \
         repository by definition (`~/.config/jigc`). It is also the engine, which ships \
         empty and filesystem-free by invariant: it renders the path string it was handed \
         and has no repo root to be relative to",
    ),
];

/// The modules the standing fence guards.
///
/// **Widened by the M50 completion audit's finding 3.** The fence's first subject was
/// `crates/cli/src/milestone.rs` alone — where all four destroying/provisioning doors' path
/// text originates — while law 1 is stated **universally**. Two sites outside that one module
/// were then found printing the host filesystem: `config set placement-root`, through the
/// *shared* predicate `rename` had been taught to work around locally, and `store.not-found`
/// on the **1.0-pinned** `doc show --format json` read contract. A module joins this list when
/// its path text has been swept; the modules that have not been swept are named in
/// `UNSWEPT_PRODUCERS` rather than left unsaid.
const GUARDED_SRC: &[&str] = &[
    "crates/cli/src/milestone.rs",
    // The shared trackability predicate — one home, four doors (`config`, `rename`, `setup`,
    // `relocate`), three of whose five refusal reasons composed the host path.
    "crates/cli/src/trackable.rs",
    // The committed-doc read path, whose `store.not-found` / `store.unparseable` blocks ride
    // the pinned `doc show --format json` contract.
    "crates/engine/src/store.rs",
    // `add-from-spec`'s spec read — the same two `store.*` blocks in the same shape.
    "crates/engine/src/milestone.rs",
    // The foreign-source migrate door. Swept by the M51 completion audit (LOW 3): its one
    // production `.display()` rendered `repo_root.join(path)` in the read fault, against a
    // source comment claiming the subject was the operator's own token. Every other path
    // text this module composes already carries the caller's spelling or the adjudicated
    // repo-relative `recorded`, so the module joins the guarded list rather than staying a
    // counted remainder of zero.
    "crates/cli/src/migrate.rs",
];

/// The producer set this fix did **not** close, named with its size so the remainder is a
/// stated bound rather than a silence (the M50 audit's own honest-scoping rule). Each entry is
/// `(file, production sites, why it is not here yet)`, and
/// [`the_unswept_remainder_is_counted_not_described`] checks every count against the source —
/// a bound nothing measures is a sentence, not a bound.
///
/// **The counts are fenced; the reasons are prose, and this note says so** (M52 Increment 10,
/// T10). A count is a claim about a set the source carries, so it is measured — every row
/// against its own file by the test below, and the table's *total* against the two records
/// that state it by `crates/cli/tests/count_fences.rs`. A **reason** is a claim about what
/// those sites *are* — a channel, a subject, a declared exemption — and no registry holds it,
/// so nothing but a re-read catches one drifting. All ten were re-read against the source at
/// M52 Increment 10; **three carried a falsified clause** and are struck below with the datum
/// that falsifies it (`start.rs`, `config.rs`, `setup.rs`), joining the two struck at
/// Increments 1 and 4 (`pack.rs`, `task.rs`). The five that hold — `finalize.rs` (its eleven
/// sites are exactly the five named helpers, and none of them is handed a repo root),
/// `locate.rs`, `doc.rs`, `adapter.rs` and `orient.rs` — stand as written, and this sentence
/// is the record that they were checked rather than assumed.
const UNSWEPT_PRODUCERS: &[(&str, usize, &str)] = &[
    (
        "crates/engine/src/finalize.rs",
        11,
        "five I/O-fault finding helpers (`provenance_io`, `promote_io`, `source_path_io`, \
         `task_missing`, `render_io`) name a task dir or a staged doc in message, route AND \
         `file_location` locus; none of them is handed a repo root, so closing them means \
         threading one through the finalize plan — its own increment, not a triage fix",
    ),
    (
        "crates/cli/src/start.rs",
        23,
        "**[Corrected 2026-09-20 (M52 Increment 10, T10).** This row read *\"The other 20 \
         are `anyhow` load faults over cascade homes and delta manifests\"*. Falsifying \
         datum, read at HEAD: `start.rs:3113`, in `load_project_layer`, is \
         `layer.config_path(project_config.display().to_string())` — the cascade layer's \
         **provenance-header stamp**, which is the very site `orient.rs`'s row below \
         disposes `DeclaredAbsolute`, and a surface render rather than an error channel. \
         The count was right and the reason false for one of the 22.**] \
         TWO of the 22 reach a finding: `overrides.project-step-missing` names the project \
         layer's step file (under `.jigc/config/`, so a repo-relative spelling exists) in \
         message and route. NINETEEN are `anyhow` load faults over cascade homes and delta \
         manifests — an error channel, not a finding surface. The twentieth is the header \
         stamp above, absolute for `orient.rs`'s reason and not for this row's. **The \
         twenty-third joined 2026-09-23 (M53 — the cwd census, C2-08) and is a DECLARED \
         absolute, not a remainder**: `blanket_base_pin_refusal` ends on a `cd <worktree>` \
         the reader pastes into a shell of unknown cwd — `design/surface-contract.md`'s own \
         *pasteable shell bytes* disposition, the same one `remove_worktrees` and \
         `assert_project_schema_shadows` take above — and it is the same absolute the \
         `Spawn:` line emits, because the refusal and that line are two ways of reaching \
         one cwd. It is counted here rather than disposed in `PATH_TEXT_SITES` because \
         `start.rs` is not in `GUARDED_SRC`: the disposition arm governs swept modules \
         only, and this file is not swept",
    ),
    (
        "crates/cli/src/config.rs",
        20,
        "**[Corrected 2026-09-20 (M52 Increment 10, T10).** This row read *\"cascade-layer \
         writes\"*. Falsifying datum, counted against the source: **eight** of the 20 are \
         reads, parses or a missing basename — `could not read source step file` (×2), \
         `{} is not valid YAML` (×2), `could not read {}` (×2) and `source file {} has no \
         basename` (×2) — against twelve creates, writes and serializes. The load-bearing \
         half of the reason is the channel, not the direction, and it held: all 20 are \
         `with_context` on `anyhow`.**] \
         `with_context` I/O faults on cascade-layer reads and writes — an error channel, \
         not a finding surface; the door's own finding text is closed by `trackable.rs`",
    ),
    (
        "crates/cli/src/pack.rs",
        6,
        "**[Corrected 2026-09-17 (M52 Increment 1, T3).** This row read `9` and gave the \
         reason *\"pack-load has no repo-root subject to be relative to\"*. Falsifying \
         datum, driven on rc.15: a corrupt `packs.yaml` printed \
         `/private/var/folders/…/repo/.jigc/config/packs.yaml` — and \
         `discover_project_config()` finds the repo root **one call earlier**, so the \
         subject the reason denied was in hand the whole time. The **four** sites that \
         composed that spelling — the two readers' `could not read` and their two \
         `is not a valid pack-set list` — now reach `repo_relative` through \
         `pack::located`, whose own fallback is the one site back (9 − 4 + 1 = 6).**] \
         Six: the four \
         `assert_project_schema_shadows` sites, disposed `DeclaredAbsolute` above; \
         `located`'s own **fallback**, which is the honest absolute answer for a config \
         dir that anchors no repository; and `resolving_path`, which renders a directory \
         pack's root for `--explain` — a pack home is routinely outside the checkout \
         (`~/packs/house`), so relativizing it would name a different directory",
    ),
    (
        "crates/cli/src/setup.rs",
        8,
        "**[Corrected 2026-09-20 (M52 Increment 10, T10).** This row read *\"install's \
         subject is the installing binary and the git hooks dir, neither a repo path\"*. \
         Falsifying datum, read at HEAD: **two** of the eight name neither — `setup.rs:822` \
         (`resolve_hooks_dir`) composes the **repo root itself** into git's failure text, \
         and `:3878` (`write_compose_marker`) names the cascade's `packs.yaml` under \
         `.jigc/config/`, a path repo-relative by construction. What actually keeps both \
         out of this class is their **channel** — each is a `std::io::Error` a caller \
         wraps, not a finding a surface prints — which is the reason this row should have \
         given for them and did not.**] \
         Declared out of this class by M50 Increment 12 / T1 with a stated reason — six of \
         the eight have the installing binary or the git hooks dir as their subject, \
         neither a repo path, and the other two are `io::Error` channels; `uninstall`'s own \
         two sites are disposed `Relative` above",
    ),
    (
        "crates/cli/src/task.rs",
        2,
        "**[Corrected 2026-09-18 (M52 Increment 4, T7).** This row read `3` and gave the \
         reason *\"two `with_context` promote/probe faults and one `git archive --prefix=`\"*. \
         Falsifying datum, measured against the source: the three sites were \
         `staged_task_prose`'s `docs/` fault, `git archive --prefix=` and **one** \
         `promotion.source` fault — the count was right and the reason false for one of \
         three, and the one it was false about was a **blocking finding surface** rather \
         than an error channel. That site now reaches `repo_relative` and is disposed \
         `Relative` above, so the remainder is two.**] \
         Two: one `with_context` promote fault, and the `git archive --prefix=`, which is an \
         argument handed to a subprocess (the `dirty_worktrees` precedent)",
    ),
    (
        "crates/cli/src/locate.rs",
        1,
        "`not_in_repo_message` is absolute BY ITS SUBJECT — it reports where jigc looked and \
         found no repository, so there is no repo to be relative to",
    ),
    (
        "crates/cli/src/doc.rs",
        2,
        "one hand-written `strip_prefix` that already relativizes, and one `with_context` \
         persist fault",
    ),
    (
        "crates/cli/src/adapter.rs",
        1,
        "the `{{worktree}}` substitution in a spawn template — bytes the sub-agent `cd`s to \
         from an unknown cwd (the `remove_worktrees` precedent)",
    ),
    (
        "crates/cli/src/orient.rs",
        1,
        "the cascade layer's `config_path`, whose surface render is already disposed \
         `DeclaredAbsolute` above (`engine/src/cascade.rs::header`)",
    ),
];

/// The one shared home every `Relative` site must reach.
const REPO_RELATIVE: &str = "repo_relative";

/// Every production `.display()` in `file`, by enclosing function — the same reader the
/// standing fence uses, so "swept" and "unswept" are counted the same way.
fn production_display_sites(file: &str) -> Vec<String> {
    let path = workspace_root().join(file);
    let body = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {file}"));
    let code = rust_source::code_only(&body);
    let regions = rust_source::cfg_test_regions(&code);
    code.match_indices(".display()")
        .filter(|(at, _)| !rust_source::is_test_domain(&path, &regions, *at))
        .map(|(at, _)| {
            format!(
                "{file}:{}: {}",
                body[..at].lines().count(),
                rust_source::enclosing_fn(&code, at).unwrap_or("<top level>"),
            )
        })
        .collect()
}

/// **The remainder is a count, not a sentence.** The audit's honest-scoping rule says a fix
/// that cannot reach its whole producer set must *state the rest with its size*; a stated size
/// nothing checks goes stale the first time someone edits one of these files. So each
/// `UNSWEPT_PRODUCERS` row is measured against the source it names, and a file cannot sit in
/// both lists.
///
/// Closing one of these is meant to redden this test: the row's count moves, and whoever moved
/// it either updates the row or promotes the file into `GUARDED_SRC`.
#[test]
fn the_unswept_remainder_is_counted_not_described() {
    let mut offenders: Vec<String> = Vec::new();

    for (file, count, reason) in UNSWEPT_PRODUCERS {
        assert!(!reason.trim().is_empty(), "{file}: carries no reason");
        assert!(
            !GUARDED_SRC.contains(file),
            "{file} is both guarded and unswept — one of the two rows is a lie",
        );
        let sites = production_display_sites(file);
        if sites.len() != *count {
            offenders.push(format!(
                "  {file}: the remainder says {count} production `.display()` site(s), the \
                 source has {}:\n      {}",
                sites.len(),
                sites.join("\n      "),
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "a bound nothing measures is a sentence — update the row (or promote the file into \
         `GUARDED_SRC` once its path text is swept):\n{}",
        offenders.join("\n"),
    );
}

/// **[Widened 2026-09-22 (M53 completion audit, finding 5).** This was a hand-written list of
/// seven `{…:?}` shapes — `{path:?}`, `{dir:?}`, `{wt:?}`, `{worktree:?}`, `{jigc_home:?}`,
/// `{worktrees_root:?}`, `{area:?}` — consulted by arm 3 alone, over the functions
/// [`PATH_TEXT_SITES`] happens to name. Falsifying datum: M53 Increment 1 / T1 added
/// `format!("could not read the shape of {path:?}")` to `milestone_boundary_gate` in
/// `crates/cli/src/milestone.rs` — a **guarded** module — and nothing reddened. The shape was
/// on the list; the *function* was on no list, and the standing fence read `.display()` only.
/// The declared bound at the foot of this file said exactly that would happen, so the bound
/// bit rather than the rule.**]
///
/// The predicate is now **derived from the binding's name** and applied to every production
/// literal in every guarded module (arm 4) as well as to each `Relative` row (arm 3).
///
/// **The judgment leg, named rather than hidden:** no static reader can type-check a format
/// argument, so membership is decided on what the binding is *called*. That is a judgment,
/// and it is the same judgment the seven-shape list encoded — only applied to the whole
/// module instead of to the functions someone remembered. It is deliberately **not** a ban on
/// every `{…:?}`: `crates/cli/src/milestone.rs` Debug-renders an `Option<i32>` exit code as
/// `{other:?}` and `crates/cli/src/trackable.rs` a subprocess argv as `{args:?}`, neither of
/// which is a path, and a blanket ban would make the fence lie about those.
///
/// **The other two render shapes were measured, not assumed.** `to_string_lossy()` reaches no
/// whole path in any guarded module — every production use there is on an `OsString` from
/// `file_name()` or on an already-relativized tail — and `as_os_str()` appears in none of
/// them. So the class is `.display()` ∪ this predicate, and that is a measurement rather than
/// a hope.
fn is_path_named(binding: &str) -> bool {
    const EXACT: &[&str] = &[
        "path", "parent", "dir", "root", "home", "area", "wt", "worktree", "file",
    ];
    const SUFFIXES: &[&str] = &[
        "_path",
        "_dir",
        "_root",
        "_home",
        "_area",
        "_docs",
        "_file",
        "_wt",
        "_worktree",
    ];
    EXACT.contains(&binding) || SUFFIXES.iter().any(|suffix| binding.ends_with(suffix))
}

/// Every production `{<path-named>:?}` render in `file`, as `(line, enclosing fn, shape)`.
///
/// The scan reads **literal values**, because that is where an inline capture lives, and
/// resolves the owner over the blanked code, because that is where a `fn` keyword lives —
/// the same two-view split [`SiteSource`] draws, for the same reason.
fn production_debug_path_renders(file: &str) -> Vec<(usize, String, String)> {
    let path = workspace_root().join(file);
    let body = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {file}"));
    let code = rust_source::code_only(&body);
    let regions = rust_source::cfg_test_regions(&code);
    let mut out = Vec::new();
    for lit in rust_source::string_literals(&body) {
        if rust_source::is_test_domain(&path, &regions, lit.offset) {
            continue;
        }
        let mut rest = lit.value.as_str();
        while let Some(at) = rest.find(":?}") {
            let head = &rest[..at];
            let open = head.rfind('{');
            if let Some(open) = open {
                let binding = &head[open + 1..];
                if !binding.is_empty()
                    && binding
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && is_path_named(binding)
                {
                    out.push((
                        body[..lit.offset].lines().count(),
                        rust_source::enclosing_fn(&code, lit.offset)
                            .unwrap_or("<top level>")
                            .to_owned(),
                        format!("{{{binding}:?}}"),
                    ));
                }
            }
            rest = &rest[at + 3..];
        }
    }
    out
}

/// **The `{…:?}` renders a guarded module still carries, counted per function** — the
/// remainder arm 4 does not demand a fix for, with the reason it stays.
///
/// **Why there is a remainder at all, stated rather than implied.** Arm 4's `.display()` rule
/// is module-wide inside `GUARDED_SRC` with no channel exemption, and applying the widened
/// predicate at the same strictness measures, for the first time, that
/// `crates/cli/src/milestone.rs` carries **21** production `{…:?}` path renders outside the
/// function this audit fixed. Every one of them is a `with_context` on `anyhow` — an I/O
/// fault wrapped for the top-level error funnel, never a `Finding` a surface composes — which
/// is the same boundary [`UNSWEPT_PRODUCERS`] already draws for `start.rs`, `config.rs` and
/// `setup.rs`, cited here rather than invented. Closing them means threading a repo root into
/// fourteen functions, which is an increment and not a triage fix.
///
/// **The count is the point.** Each row is checked against the source below, so closing one
/// reddens this test and whoever closed it either moves the row or deletes it — the
/// `UNSWEPT_PRODUCERS` contract, applied one module in.
const DEBUG_REMAINDER: &[(&str, &str, usize)] = &[
    (
        "crates/cli/src/milestone.rs",
        "materialize_and_commit_record",
        2,
    ),
    ("crates/cli/src/milestone.rs", "commit_record_only", 1),
    ("crates/cli/src/milestone.rs", "record_pathspec", 1),
    ("crates/cli/src/milestone.rs", "capture_record_pre_image", 1),
    (
        "crates/cli/src/milestone.rs",
        "reconcile_record_preflight",
        2,
    ),
    ("crates/cli/src/milestone.rs", "append_and_commit_record", 2),
    ("crates/cli/src/milestone.rs", "recorded_sub_tasks", 1),
    ("crates/cli/src/milestone.rs", "read_record", 1),
    (
        "crates/cli/src/milestone.rs",
        "worktrees_have_staged_code",
        1,
    ),
    ("crates/cli/src/milestone.rs", "flip_record_for_finalize", 3),
    ("crates/cli/src/milestone.rs", "worktree_staged_patch", 1),
    (
        "crates/cli/src/milestone.rs",
        "worktree_staged_file_count",
        1,
    ),
    ("crates/cli/src/milestone.rs", "discarded_work", 1),
    ("crates/cli/src/milestone.rs", "shared_checkout_staged", 1),
];

/// The cargo workspace root — the table's `file` column is workspace-relative, because the
/// class reaches `crates/engine/src` too (the orientation header the engine composes).
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The byte span of the function whose body encloses `at` — the
/// `located_finding_text::enclosing_fn_body` reader, brace-matched over blanked code so a
/// `{` inside a literal cannot throw it off.
fn enclosing_fn_span(code: &str, at: usize) -> (usize, usize) {
    let start = code[..at].rfind("fn ").expect("an enclosing fn");
    let open = start + code[start..].find('{').expect("a function body");
    let bytes = code.as_bytes();
    let mut depth = 0usize;
    for (i, b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return (open, i + 1 - open);
                }
            }
            _ => {}
        }
    }
    (open, code.len() - open)
}

/// A production function's **code**, with comments and string literals blanked
/// (`rust_source::code_only`) — the view `.display()` and the call to the shared home live
/// in.
///
/// **[Narrowed 2026-09-22 (M53 completion audit, finding 5).** This carried a second view,
/// the decoded values of the function's string literals, for arm 3's hand-written
/// `RAW_DEBUG_RENDERS` check. That check now runs off
/// [`production_debug_path_renders`], which scans a whole **module** rather than one
/// function, so the per-site literal view has no reader left. A doc-comment naming
/// `repo_relative` would satisfy the code view, which is why the literals were read
/// separately in the first place; that reason still holds and is why `code_only` is kept
/// rather than the raw body.**]
struct SiteSource {
    code: String,
}

impl SiteSource {
    /// `None` when `<file>` declares no production `fn <name>` — a row naming nothing.
    fn read(file: &str, name: &str) -> Option<SiteSource> {
        let path = workspace_root().join(file);
        let body = fs::read_to_string(&path).ok()?;
        let code = rust_source::code_only(&body);
        let regions = rust_source::cfg_test_regions(&code);
        let needle = format!("fn {name}(");
        let at = code
            .match_indices(&needle)
            .find(|(at, _)| !rust_source::is_test_domain(&path, &regions, *at))
            .map(|(at, _)| at)?;
        let (start, len) = enclosing_fn_span(&code, at + 3);
        Some(SiteSource {
            code: code[start..start + len].to_owned(),
        })
    }
}

#[test]
fn every_path_a_door_prints_carries_a_disposition_the_source_backs() {
    let mut offenders: Vec<String> = Vec::new();

    for (file, name, disposition, reason) in PATH_TEXT_SITES {
        assert!(
            !reason.trim().is_empty(),
            "{file}: `{name}` carries no reason",
        );
        let Some(site) = SiteSource::read(file, name) else {
            offenders.push(format!(
                "  {file}: `{name}` is disposed here but no production function of that name \
                 exists — a row naming nothing is a claim about nothing"
            ));
            continue;
        };
        match disposition {
            Disposition::Relative => {
                // The shared home counts whether it is reached directly or **through
                // another `Relative` site**. The class's whole claim is that one home
                // renders every path, so factoring a renderer out into one — `hold_line`,
                // the line all three refusing doors list a held path with — must not read
                // as a site that stopped reaching it. The extracted home is itself a row,
                // so the chain always ends at a checked one.
                let reaches = site.code.contains(REPO_RELATIVE)
                    || PATH_TEXT_SITES
                        .iter()
                        .any(|(_, other, other_disposition, _)| {
                            *other_disposition == Disposition::Relative
                                && *other != *name
                                && site.code.contains(&format!("{other}("))
                        });
                if !reaches {
                    offenders.push(format!(
                        "  {file}: `{name}` is declared `Relative` but never reaches \
                         `{REPO_RELATIVE}`, directly or through another `Relative` site"
                    ));
                }
                if site.code.contains(".display()") {
                    offenders.push(format!(
                        "  {file}: `{name}` is declared `Relative` but still renders a host \
                         path with `.display()`"
                    ));
                }
                for (line, owner, shape) in production_debug_path_renders(file) {
                    if owner == *name {
                        offenders.push(format!(
                            "  {file}:{line}: `{name}` is declared `Relative` but still \
                             composes a host path into its text with `{shape}`"
                        ));
                    }
                }
            }
            Disposition::DeclaredAbsolute => {
                if site.code.contains(REPO_RELATIVE) {
                    offenders.push(format!(
                        "  {file}: `{name}` is declared `DeclaredAbsolute` but reaches \
                         `{REPO_RELATIVE}` — the row and the code disagree about which one \
                         it is"
                    ));
                }
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every site the four doors print a path through owes a disposition, and a disposition \
         owes the source to back it (`design/surface-contract.md` → law 1):\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------------
// Arm 4 — the standing fence
// (arm 5, the counted remainder, sits with `UNSWEPT_PRODUCERS` above)
// ---------------------------------------------------------------------------------

/// No production `.display()` in any **guarded** module outside a `DeclaredAbsolute` site.
///
/// **A fence over an empty set is still a fence** — the whole point is that the next
/// hand-rolled absolute render reddens here instead of shipping. `.display()` on a `Path` is
/// the shape every one of these modules' leaks took, so the rule is stated where it binds
/// rather than as a repo-wide grep that would have to except every legitimate absolute in the
/// binary.
///
/// **The subject is a list, not a module** (the M50 completion audit's finding 3). A fence
/// whose subject is one file is a fence a sibling walks around: the shared trackability
/// predicate handed four doors a host path while `rename` alone was taught to compose its own
/// message, and the engine's committed read leaked one onto the 1.0-pinned JSON contract. Each
/// module joins `GUARDED_SRC` when its path text is swept; what is not swept is counted in
/// `UNSWEPT_PRODUCERS`.
///
/// **[Struck 2026-09-22 (M53 completion audit, finding 5).** The declared bound here read
/// *"the fence reads `.display()` only. The `{…:?}` half is checked per-row in arm 3 (over
/// `RAW_DEBUG_RENDERS`) and end-to-end in arms 1 and 2; a `{…:?}` render introduced in a
/// function this class does not name is caught by those only if a door prints it."* It is
/// struck because it **bit**: M53 Increment 1 / T1 introduced exactly that render in
/// `milestone_boundary_gate`, a function this class did not name, in a guarded module, and no
/// arm saw it. The fence now reads both shapes — see [`is_path_named`] for the widened
/// predicate and what it deliberately does not cover.**]
///
/// **Declared bound, replacing it:** the `{…:?}` half is decided on the **binding's name**,
/// because a static reader cannot type-check a format argument. A path held in a
/// non-path-named binding still slips, and `crates/cli/src/milestone.rs` carries a **counted**
/// remainder of pre-existing renders ([`DEBUG_REMAINDER`]) rather than a silence.
#[test]
fn a_guarded_module_renders_no_host_path_outside_a_declared_absolute() {
    let mut offenders: Vec<String> = Vec::new();

    for src in GUARDED_SRC {
        let path = workspace_root().join(src);
        let body = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {src}"));
        let code = rust_source::code_only(&body);
        let regions = rust_source::cfg_test_regions(&code);

        let declared: Vec<&str> = PATH_TEXT_SITES
            .iter()
            .filter(|(file, _, d, _)| file == src && *d == Disposition::DeclaredAbsolute)
            .map(|(_, name, _, _)| *name)
            .collect();

        for (at, _) in code.match_indices(".display()") {
            if rust_source::is_test_domain(&path, &regions, at) {
                continue;
            }
            let owner = rust_source::enclosing_fn(&code, at).unwrap_or("<top level>");
            if declared.contains(&owner) {
                continue;
            }
            offenders.push(format!(
                "  {src}:{}: `{owner}` renders a path with `.display()`",
                body[..at].lines().count(),
            ));
        }

        // …and the half the struck bound left open: the `Debug` render of a path-named
        // binding, which is the shape the audit's own finding took.
        for (line, owner, shape) in production_debug_path_renders(src) {
            if declared.contains(&owner.as_str()) {
                continue;
            }
            if DEBUG_REMAINDER
                .iter()
                .any(|(file, name, _)| file == src && *name == owner)
            {
                continue;
            }
            offenders.push(format!(
                "  {src}:{line}: `{owner}` renders a path with `{shape}`",
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "`.display()` on a path, and the `Debug` render of a path-named binding, are how every \
         one of these modules' host-path leaks reached a surface — render through the shared \
         `engine::path::repo_relative`, declare the site absolute in `PATH_TEXT_SITES` with the \
         reason it stays, or count it in `DEBUG_REMAINDER`:\n{}",
        offenders.join("\n"),
    );
}

/// **The `{…:?}` remainder is a count, not a sentence** — [`the_unswept_remainder_is_counted_not_described`]'s
/// sibling, one module in.
///
/// A guarded module's remaining `Debug` path renders are exempted by *function*, so a row that
/// silently grew a second render would widen its own exemption. Each row's count is therefore
/// measured against the source, in both directions: a function that lost a render reddens too,
/// because an exemption for something that is gone is an exemption nobody re-read.
#[test]
fn the_guarded_debug_remainder_is_counted_not_described() {
    let mut offenders: Vec<String> = Vec::new();

    for (file, name, count) in DEBUG_REMAINDER {
        assert!(
            GUARDED_SRC.contains(file),
            "{file} is not guarded — a remainder row there exempts nothing",
        );
        let sites: Vec<(usize, String, String)> = production_debug_path_renders(file)
            .into_iter()
            .filter(|(_, owner, _)| owner == name)
            .collect();
        if sites.len() != *count {
            offenders.push(format!(
                "  {file}: `{name}` is counted at {count} `{{…:?}}` path render(s), the source \
                 has {}:\n      {}",
                sites.len(),
                sites
                    .iter()
                    .map(|(line, _, shape)| format!("{file}:{line}: {shape}"))
                    .collect::<Vec<_>>()
                    .join("\n      "),
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "a bound nothing measures is a sentence — update the row, or delete it once the \
         function renders through the shared home:\n{}",
        offenders.join("\n"),
    );
}
