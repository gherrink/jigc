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
//! print*: every production site reachable from `cli::milestone::DESTROYING_DOORS` and from
//! `jigc milestone provision` that renders a filesystem path into a finding (message, locus or
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
//! Three arms:
//!
//! 1. the **driven** arm — a door table over a fixture root minted by `mktemp -d`, each row
//!    run for real and its whole stdout+stderr scanned for that root's absolute prefix;
//! 2. the **disposition** arm — one row per site, `Relative` or `DeclaredAbsolute(reason)`,
//!    each verdict checked against the source rather than believed (the
//!    `located_finding_text::MESSAGE_SITES` third-verdict idiom);
//! 3. the **standing fence** — no production `.display()` in `crates/cli/src/milestone.rs`
//!    outside a `DeclaredAbsolute` site, so the next hand-rolled absolute render reddens here
//!    rather than shipping.

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
// Arm 2 — the disposition table
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
        "unprobeable_leftover_finding",
        Disposition::Relative,
        "the fail-closed refusal's message AND its `at:` locus, shared by `provision` and \
         `discard`",
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
         VERBATIM inside `unprobeable_leftover_finding`'s and \
         `setup::unverified_worktrees_finding`'s messages, so a relative locus over an \
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

/// The module the standing fence guards — where all four doors' path text originates.
const MILESTONE_SRC: &str = "crates/cli/src/milestone.rs";

/// The one shared home every `Relative` site must reach.
const REPO_RELATIVE: &str = "repo_relative";

/// The `{…:?}` render shapes a `Relative` site may not carry inside a message it composes —
/// listed by binding name, because Rust's inline captures are the shape these sites used.
/// (`.display()`, the other half, is checked over the site's *code* rather than its literals.)
const RAW_DEBUG_RENDERS: &[&str] = &[
    "{path:?}",
    "{dir:?}",
    "{wt:?}",
    "{worktree:?}",
    "{jigc_home:?}",
    "{worktrees_root:?}",
    "{area:?}",
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

/// A production function read two ways, because the two halves of the rule live in opposite
/// places: `.display()` and the call to the shared home are **code**, and the `{…:?}` shapes
/// are **string literal values**. Reading one view for both would be wrong in both directions
/// — a doc comment naming `repo_relative` would satisfy the first, and blanked literals would
/// hide the second.
struct SiteSource {
    /// The body with comments and literals blanked (`rust_source::code_only`).
    code: String,
    /// The decoded values of every string literal inside the body.
    literals: Vec<String>,
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
        let literals = rust_source::string_literals(&body)
            .into_iter()
            .filter(|lit| lit.offset >= start && lit.offset < start + len)
            .map(|lit| lit.value)
            .collect();
        Some(SiteSource {
            code: code[start..start + len].to_owned(),
            literals,
        })
    }

    fn literals_contain(&self, needle: &str) -> bool {
        self.literals.iter().any(|value| value.contains(needle))
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
                if !site.code.contains(REPO_RELATIVE) {
                    offenders.push(format!(
                        "  {file}: `{name}` is declared `Relative` but never reaches \
                         `{REPO_RELATIVE}`"
                    ));
                }
                if site.code.contains(".display()") {
                    offenders.push(format!(
                        "  {file}: `{name}` is declared `Relative` but still renders a host \
                         path with `.display()`"
                    ));
                }
                for shape in RAW_DEBUG_RENDERS {
                    if site.literals_contain(shape) {
                        offenders.push(format!(
                            "  {file}: `{name}` is declared `Relative` but still composes a \
                             host path into its text with `{shape}`"
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
// Arm 3 — the standing fence
// ---------------------------------------------------------------------------------

/// No production `.display()` in `milestone.rs` outside a `DeclaredAbsolute` site.
///
/// **A fence over an empty set is still a fence** — the whole point is that the next
/// hand-rolled absolute render reddens here instead of shipping. `.display()` on a `Path` is
/// the shape every one of this module's leaks took, and `milestone.rs` is where all four
/// doors' path text originates, so the rule is stated where it binds rather than as a
/// repo-wide grep that would have to except every legitimate absolute in the binary.
///
/// **Declared bound:** the fence reads `.display()` only. The `{…:?}` half is checked per-row
/// in arm 2 (over `RAW_DEBUG_RENDERS`) and end-to-end in arm 1; a `{…:?}` render introduced in
/// a function this class does not name is caught by arm 1 only if a door prints it.
#[test]
fn milestone_renders_no_host_path_outside_a_declared_absolute() {
    let path = workspace_root().join(MILESTONE_SRC);
    let body = fs::read_to_string(&path).expect("read milestone.rs");
    let code = rust_source::code_only(&body);
    let regions = rust_source::cfg_test_regions(&code);

    let declared: Vec<&str> = PATH_TEXT_SITES
        .iter()
        .filter(|(file, _, d, _)| *file == MILESTONE_SRC && *d == Disposition::DeclaredAbsolute)
        .map(|(_, name, _, _)| *name)
        .collect();

    let mut offenders: Vec<String> = Vec::new();
    for (at, _) in code.match_indices(".display()") {
        if rust_source::is_test_domain(&path, &regions, at) {
            continue;
        }
        let owner = rust_source::enclosing_fn(&code, at).unwrap_or("<top level>");
        if declared.contains(&owner) {
            continue;
        }
        offenders.push(format!(
            "  milestone.rs:{}: `{owner}` renders a path with `.display()`",
            body[..at].lines().count(),
        ));
    }

    assert!(
        offenders.is_empty(),
        "`.display()` on a path is how every one of this module's host-path leaks reached a \
         surface — render through the shared `render::repo_relative`, or declare the site \
         absolute in `PATH_TEXT_SITES` with the reason it stays:\n{}",
        offenders.join("\n"),
    );
}
