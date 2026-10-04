//! **A file jigc replaces whole is written as a regular file at exactly its path, never
//! through a link** (the rc.24 fix pass; `DECISIONS.md` → 2026-10-04, the symlink fork;
//! `completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R1-F1.md` → class
//! sibling 2).
//!
//! `jigc setup` puts its own content at five paths — `.jigc/AGENT.md`, `.jigc/version`,
//! `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml` and the adapter's guide artifact — and
//! every one of those writers opened its destination through whatever was there. Driven on
//! `1.0.0-rc.24`, on a repository **with commits**: a committed symlink at any of the first
//! four, whose target no git object holds (untracked, gitignored, or outside the
//! repository), had that target overwritten at exit 0 with `findings: []`. `--force` did the
//! same and named nothing. The link is clean against `HEAD`, so the dirty-install guard had
//! nothing to ask; and that guard's own route — *commit the work at those paths* — turned an
//! uncommitted link into exactly this cell, because committing a link commits the link.
//!
//! The same stamp writer runs at `jigc task finalize`, with no ownership question at all:
//! a hand-edited `.jigc/version` no commit held was regenerated at exit 0, and a link there
//! was written through.
//!
//! **The rule, at both doors.** The entry at a replaced path is asked for **without
//! following a link**, and so is every directory on the way to it below the checkout root
//! (`cli::regular_file`). What happens next is the door's:
//!
//! - **`jigc setup` refuses before its first write**, under the member's existing
//!   write-failure code, whatever `--force` says: the consent is to replace a file, never to
//!   follow one. The guide is the one member whose own oracle answers first — an entry that
//!   is not jigc's file is the adopter's, left alone and reported
//!   (`adapter-guide.user-modified`), exactly as an edited copy is.
//! - **`jigc task finalize` leaves the stamp alone and says so.** The stamp is provenance
//!   and gates nothing, so the commit lands: what is at `.jigc/version` is neither written
//!   nor staged unless it is absent or jigc's own one-line stamp, and the landed envelope
//!   carries one advisory under the stamp writer's own code.
//!
//! Every cell drives the real binary (`CARGO_BIN_EXE_jigc`) over a throwaway `git init`.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The marker every planted file carries, so *"are the adopter's bytes still there?"* is
/// one comparison.
const MARK: &str = "USERMARK";

/// The guide artifact's path under the Claude Code profile.
const GUIDE: &str = ".claude/skills/jigc/SKILL.md";

/// An in-worktree hooks dir's `pre-commit` — named so the production table is asked with
/// every conditional on.
const HOOK: &str = ".githooks/pre-commit";

/// The stamp writer's code — the one the finalize door's advisory rides.
const STAMP_CODE: &str = "setup.version-stamp";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-never-follow-{tag}-{}-{:?}",
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

/// Run `git` in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A real `git init` with a per-repo identity and **no** commit yet.
fn unborn_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    (repo, home)
}

/// [`unborn_repo`] plus a seed commit — a **born** `HEAD`.
fn born_repo(tag: &str) -> (TempDir, TempDir) {
    let (repo, home) = unborn_repo(tag);
    write(repo.path(), "README.md", "hello\n");
    git(repo.path(), &["add", "README.md"]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    (repo, home)
}

/// Write `contents` at `relative` under `repo`, creating parent directories.
fn write(repo: &Path, relative: &str, contents: &str) {
    let path = repo.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, contents).expect("write file");
}

/// Read `relative` under `repo` (through a link, like any reader).
fn read(repo: &Path, relative: &str) -> String {
    fs::read_to_string(repo.join(relative)).unwrap_or_default()
}

/// Whether the entry at `relative` is itself a symbolic link.
fn is_link(repo: &Path, relative: &str) -> bool {
    fs::symlink_metadata(repo.join(relative)).is_ok_and(|meta| meta.file_type().is_symlink())
}

/// Whether the entry at `relative` is itself a regular file.
fn is_regular(repo: &Path, relative: &str) -> bool {
    fs::symlink_metadata(repo.join(relative)).is_ok_and(|meta| meta.is_file())
}

/// Run `jigc <args>` with `cwd = repo` and a temp `$HOME`, composing the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run jigc")
}

/// stdout + stderr of an invocation, joined for message assertions.
fn said(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// The refusal's `route:` line, as the text surface prints it.
fn route(out: &std::process::Output) -> String {
    said(out)
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("route:"))
        .unwrap_or_else(|| panic!("the finding prints a route: {}", said(out)))
        .trim()
        .to_string()
}

/// Plant a symlink at `relative` pointing at `target` (a path relative to the repository
/// root, or an absolute one), creating the link's parent directories.
fn link(repo: &Path, relative: &str, target: &str) {
    let at = repo.join(relative);
    if let Some(parent) = at.parent() {
        fs::create_dir_all(parent).expect("create the link's parent");
    }
    let target = if Path::new(target).is_absolute() {
        PathBuf::from(target)
    } else {
        // Relative to the link's own directory: one `..` per directory between it and the
        // repository root.
        let depth = Path::new(relative).components().count() - 1;
        let mut up = PathBuf::new();
        for _ in 0..depth {
            up.push("..");
        }
        up.join(target)
    };
    symlink(target, at).expect("plant the link");
}

/// The members of the install's path class whose writer **replaces** what it finds — read
/// off the production table, with every conditional on.
fn replacing_members() -> Vec<cli::setup::InstallMember> {
    cli::setup::install_path_dispositions(
        "CLAUDE.md",
        ".claude/settings.json",
        Some(GUIDE),
        Some(HOOK),
    )
    .into_iter()
    .filter(|member| member.writer != cli::setup::InstallWriter::Preserves)
    .collect()
}

/// What an adopter could have in the file a link at `path` points to — bytes carrying
/// [`MARK`], in a shape the member's writer can read. **A new replacing member has no plant
/// and panics here**, so the class cell cannot go green over a member nobody drove.
fn target_plant(path: &str) -> &'static str {
    match path {
        ".jigc/AGENT.md" => "# Team notes for agents — USERMARK\n\nAlways run the linter.\n",
        ".jigc/version" => "USERMARK: we pin jigc here\nsee the team wiki\n",
        ".jigc/config/.gitkeep" => "USERMARK\n",
        ".jigc/config/packs.yaml" => "# USERMARK why we pin dev only\npacks:\n- dev\n",
        GUIDE => "# my own skill notes\n\nUSERMARK\n",
        other => panic!(
            "`{other}` is a replacing install member with no plant: give it bytes an adopter \
             could keep behind a link there, so the no-follow cell is driven for it too"
        ),
    }
}

/// The **existing** write-failure code each replacing member's refusal rides — pinned here
/// so the fix cannot mint one, and held against the production table's own declaration.
fn refusal_code(path: &str) -> &'static str {
    match path {
        ".jigc/AGENT.md" => "setup.write-bootstrap",
        ".jigc/version" => STAMP_CODE,
        ".jigc/config/.gitkeep" => "setup.init-project-layer",
        ".jigc/config/packs.yaml" => "setup.compose-marker",
        GUIDE => "setup.write-guide",
        other => panic!("`{other}` is a replacing install member with no pinned refusal code"),
    }
}

/// Assert `out` is the pre-write refusal under `code`: exit 1, that code and not the
/// dirty-install one, `HEAD` unmoved, and **nothing installed** — the host file the install
/// would write second is still absent.
fn assert_refused_before_any_write(
    repo: &Path,
    out: &std::process::Output,
    code: &str,
    head: &str,
    what: &str,
) {
    let said = said(out);
    assert_eq!(out.status.code(), Some(1), "{what}: refuses: {said}");
    assert!(said.contains(code), "{what}: under `{code}`: {said}");
    assert!(
        !said.contains("setup.dirty-install-path"),
        "{what}: the link refusal answers, not the dirty-install guard whose commit and \
         `--force` arms do not work for a link: {said}"
    );
    assert_eq!(
        git(repo, &["rev-parse", "--verify", "-q", "HEAD"]),
        head,
        "{what}: no commit was made"
    );
    assert!(
        !repo.join("CLAUDE.md").exists(),
        "{what}: asked before the first write — nothing else was installed: {said}"
    );
}

/// (1) **The class axis, driven: every replacing member, a committed link whose target git
/// does not hold.** The loss cell on `1.0.0-rc.24` — exit 0, `findings: []`, the target
/// regenerated and its bytes in no object, on a `HEAD` the dirty-install guard reads clean.
///
/// One fresh repository per member of the production table. Plain and `--force`, because the
/// consent is to replace the entry jigc owns, not to follow it. Then the printed route,
/// followed: the link removed and the removal committed, one plain re-run installs a regular
/// file there and the target is still byte-identical.
#[test]
fn every_replacing_member_refuses_a_committed_link_and_its_target_is_untouched() {
    let mut driven = Vec::new();
    for member in replacing_members() {
        let path = member.path.as_str();
        let plant = target_plant(path);
        assert!(
            matches!(
                member.writer,
                cli::setup::InstallWriter::Replaces { refusal, .. } if refusal == refusal_code(path)
            ),
            "`{path}`: the production table declares the existing write-failure code this \
             suite pins for it: {:?}",
            member.writer
        );
        let (repo, home) = born_repo("class");
        let (repo, home) = (repo.path(), home.path());
        write(repo, "kept/target", plant);
        link(repo, path, "kept/target");
        git(repo, &["add", "--", path]);
        git(repo, &["commit", "-q", "-m", "a link at an install path"]);
        let head = git(repo, &["rev-parse", "HEAD"]);
        assert_eq!(
            git(repo, &["status", "--porcelain"]),
            "?? kept/",
            "the premise: the link is committed and clean, and git holds no copy of its target"
        );

        for args in [&["setup"][..], &["setup", "--force"][..]] {
            let out = jigc(repo, home, args);
            if path == GUIDE {
                // The guide's own oracle answers first: an entry that is not jigc's file is
                // the adopter's — left alone and reported, never an install-stopping refusal.
                let said = said(&out);
                assert_eq!(out.status.code(), Some(0), "`{path}` {args:?}: {said}");
                assert!(
                    said.contains("adapter-guide.user-modified"),
                    "`{path}` {args:?}: and named: {said}"
                );
            } else {
                assert_refused_before_any_write(
                    repo,
                    &out,
                    refusal_code(path),
                    &head,
                    &format!("`{path}` {args:?}"),
                );
            }
            assert_eq!(
                read(repo, "kept/target"),
                plant,
                "`{path}` {args:?}: the write did not go through the link — its target is \
                 byte-identical"
            );
            assert!(
                is_link(repo, path),
                "`{path}` {args:?}: and the link is still a link"
            );
        }
        driven.push(path.to_string());
        if path == GUIDE {
            continue;
        }

        // The route, followed as printed: remove the link, commit the removal (git tracks
        // this one), re-run. The commit is part of the route because the removal of a
        // tracked link is itself a change no commit holds — left uncommitted, the re-run's
        // commit-time backstop refuses over the path it has just written.
        let refused = jigc(repo, home, &["setup"]);
        let printed = route(&refused);
        assert!(
            printed.contains(&format!("remove the link at `{path}`"))
                && printed.contains("committing the removal where git tracks the entry")
                && printed.contains("re-run `jigc setup`"),
            "`{path}`: the route names the act, the commit and the re-run: {printed}"
        );
        git(repo, &["rm", "-q", "--", path]);
        git(repo, &["commit", "-q", "-m", "remove the link"]);
        assert!(!is_link(repo, path), "`{path}`: the link is gone");
        let out = jigc(repo, home, &["setup"]);
        assert_eq!(
            out.status.code(),
            Some(0),
            "`{path}`: the route followed, the plain re-run installs: {}",
            said(&out)
        );
        assert!(
            is_regular(repo, path),
            "`{path}`: jigc's own regular file is there now"
        );
        assert_eq!(
            read(repo, "kept/target"),
            plant,
            "`{path}`: and the target was never touched"
        );
        assert_eq!(
            git(repo, &["status", "--porcelain"]),
            "?? kept/",
            "`{path}`: the install is committed whole"
        );
    }
    assert_eq!(
        driven,
        vec![
            ".jigc/AGENT.md",
            ".jigc/version",
            ".jigc/config/.gitkeep",
            ".jigc/config/packs.yaml",
            GUIDE,
        ],
        "the replacing members of the install — a sixth is driven here the day it is declared"
    );
}

/// (2) **Every shape of occupant that is not a regular file, at one member.** The target
/// may be gitignored, outside the repository, or absent; the link may be uncommitted; the
/// entry may be a directory or a FIFO. None is written through or over, and each refuses
/// before the first write.
#[test]
fn every_non_regular_occupant_refuses_and_nothing_is_written_through_it() {
    const PATH: &str = ".jigc/AGENT.md";
    let code = refusal_code(PATH);
    let plant = target_plant(PATH);

    // (a) The target is gitignored — `git status` is empty, so nothing would ever prompt.
    {
        let (repo, home) = born_repo("ignored");
        let (repo, home) = (repo.path(), home.path());
        write(repo, ".gitignore", "kept/\n");
        write(repo, "kept/target", plant);
        link(repo, PATH, "kept/target");
        git(repo, &["add", "--", ".gitignore", PATH]);
        git(repo, &["commit", "-q", "-m", "a link to an ignored file"]);
        let head = git(repo, &["rev-parse", "HEAD"]);
        assert_eq!(git(repo, &["status", "--porcelain"]), "", "the premise");
        let out = jigc(repo, home, &["setup"]);
        assert_refused_before_any_write(repo, &out, code, &head, "ignored target");
        assert_eq!(read(repo, "kept/target"), plant, "ignored target: intact");
    }

    // (b) The target is outside the repository — no git operation here could restore it.
    {
        let (repo, home) = born_repo("outside");
        let (repo, home) = (repo.path(), home.path());
        let elsewhere = TempDir::new("outside-target");
        let target = elsewhere.path().join("agent-notes.md");
        fs::write(&target, plant).expect("plant the outside file");
        link(repo, PATH, target.to_str().expect("utf-8 temp path"));
        git(repo, &["add", "--", PATH]);
        git(
            repo,
            &["commit", "-q", "-m", "a link out of the repository"],
        );
        let head = git(repo, &["rev-parse", "HEAD"]);
        for args in [&["setup"][..], &["setup", "--force"][..]] {
            let out = jigc(repo, home, args);
            assert_refused_before_any_write(repo, &out, code, &head, "outside target");
            assert_eq!(
                fs::read_to_string(&target).expect("read the outside file"),
                plant,
                "outside target {args:?}: a file outside the repository is intact"
            );
        }
    }

    // (c) A dangling link: nothing to lose, and still nothing created through it.
    {
        let (repo, home) = born_repo("dangling");
        let (repo, home) = (repo.path(), home.path());
        fs::create_dir_all(repo.join("kept")).expect("create kept/");
        link(repo, PATH, "kept/absent");
        git(repo, &["add", "--", PATH]);
        git(repo, &["commit", "-q", "-m", "a dangling link"]);
        let head = git(repo, &["rev-parse", "HEAD"]);
        let out = jigc(repo, home, &["setup"]);
        assert_refused_before_any_write(repo, &out, code, &head, "dangling link");
        assert!(
            !repo.join("kept/absent").exists(),
            "dangling link: the writer did not create the link's target"
        );
    }

    // (d) An uncommitted link on a born `HEAD` — the dirty-install guard used to answer
    //     here, with a commit arm that led straight into cell (1) and a `--force` arm that
    //     wrote through. The link refusal answers first now, under both spellings.
    {
        let (repo, home) = born_repo("uncommitted");
        let (repo, home) = (repo.path(), home.path());
        write(repo, "kept/target", plant);
        link(repo, PATH, "kept/target");
        let head = git(repo, &["rev-parse", "HEAD"]);
        for args in [&["setup"][..], &["setup", "--force"][..]] {
            let out = jigc(repo, home, args);
            assert_refused_before_any_write(repo, &out, code, &head, "uncommitted link");
            assert_eq!(read(repo, "kept/target"), plant, "uncommitted link: intact");
        }
    }

    // (e) A directory at the path, holding a file of the adopter's.
    {
        let (repo, home) = born_repo("directory");
        let (repo, home) = (repo.path(), home.path());
        write(repo, ".jigc/AGENT.md/notes.md", plant);
        let head = git(repo, &["rev-parse", "HEAD"]);
        let out = jigc(repo, home, &["setup"]);
        assert_refused_before_any_write(repo, &out, code, &head, "directory");
        assert_eq!(
            read(repo, ".jigc/AGENT.md/notes.md"),
            plant,
            "directory: what it holds is intact"
        );
        assert!(
            route(&out).contains("move the directory at `.jigc/AGENT.md` out of the way"),
            "directory: the route names the act that fits a directory: {}",
            route(&out)
        );
    }

    // (f) A FIFO at the path: a plain open-for-write would park on it with no reader.
    {
        let (repo, home) = born_repo("fifo");
        let (repo, home) = (repo.path(), home.path());
        fs::create_dir_all(repo.join(".jigc")).expect("create .jigc");
        let made = Command::new("mkfifo")
            .arg(repo.join(PATH))
            .status()
            .expect("run mkfifo");
        assert!(made.success(), "mkfifo");
        let head = git(repo, &["rev-parse", "HEAD"]);
        let out = jigc(repo, home, &["setup"]);
        assert_refused_before_any_write(repo, &out, code, &head, "fifo");
    }
}

/// (3) **The laundering route is closed.** On an unborn `HEAD` an untracked link at a
/// replacing member refuses; the dirty-install refusal that used to answer there named
/// *commit them* as an arm, and committing a link commits the link — after which the re-run
/// read a clean path and wrote through it. Followed here to the end: the link committed, the
/// re-run still refuses, and the target is byte-identical throughout.
#[test]
fn committing_the_link_does_not_buy_a_write_through_it() {
    const PATH: &str = ".jigc/AGENT.md";
    let plant = target_plant(PATH);
    let (repo, home) = unborn_repo("launder");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "kept/target", plant);
    link(repo, PATH, "kept/target");

    let out = jigc(repo, home, &["setup"]);
    let first = said(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "unborn, untracked link: {first}"
    );
    assert!(first.contains(refusal_code(PATH)), "the link code: {first}");
    assert_eq!(
        read(repo, "kept/target"),
        plant,
        "intact after the first run"
    );

    git(repo, &["add", "--", PATH]);
    git(repo, &["commit", "-q", "-m", "commit the link"]);
    let head = git(repo, &["rev-parse", "HEAD"]);
    let out = jigc(repo, home, &["setup"]);
    assert_refused_before_any_write(repo, &out, refusal_code(PATH), &head, "committed link");
    assert_eq!(
        read(repo, "kept/target"),
        plant,
        "the commit bought nothing: the target is still the adopter's bytes"
    );
}

/// (4) **A link on the way to a replaced path is a link at it.** `O_NOFOLLOW` guards the
/// last component only, so a symlinked `.jigc` or `.jigc/config` carried every write below
/// it into another directory — driven on this tree's parent: `.jigc -> ../store` had
/// `../store/AGENT.md` regenerated, and the run then failed at its own `git add` (*beyond a
/// symbolic link*), which no route could clear. Refused before the first write now, naming
/// the link.
#[test]
fn a_linked_directory_on_the_way_refuses_before_anything_is_written_below_it() {
    // `.jigc` itself, pointing out of the repository.
    {
        let (repo, home) = born_repo("anc-root");
        let (repo, home) = (repo.path(), home.path());
        let store = TempDir::new("anc-store");
        let plant = target_plant(".jigc/AGENT.md");
        fs::write(store.path().join("AGENT.md"), plant).expect("plant");
        symlink(store.path(), repo.join(".jigc")).expect("link .jigc");
        let head = git(repo, &["rev-parse", "HEAD"]);
        let out = jigc(repo, home, &["setup"]);
        assert_refused_before_any_write(
            repo,
            &out,
            refusal_code(".jigc/AGENT.md"),
            &head,
            "linked `.jigc`",
        );
        assert_eq!(
            fs::read_to_string(store.path().join("AGENT.md")).expect("read"),
            plant,
            "linked `.jigc`: the file behind it is intact"
        );
        assert!(
            said(&out).contains("`.jigc` is a symbolic link"),
            "linked `.jigc`: the refusal names the link, not the leaf: {}",
            said(&out)
        );
    }

    // `.jigc/config`, pointing at a directory that holds a commented `packs.yaml`.
    {
        let (repo, home) = born_repo("anc-config");
        let (repo, home) = (repo.path(), home.path());
        let packs = target_plant(".jigc/config/packs.yaml");
        write(repo, "shared-config/packs.yaml", packs);
        write(repo, "shared-config/.gitkeep", "USERMARK\n");
        link(repo, ".jigc/config", "shared-config");
        let head = git(repo, &["rev-parse", "HEAD"]);
        let out = jigc(repo, home, &["setup"]);
        assert_refused_before_any_write(
            repo,
            &out,
            refusal_code(".jigc/config/.gitkeep"),
            &head,
            "linked `.jigc/config`",
        );
        assert_eq!(
            read(repo, "shared-config/packs.yaml"),
            packs,
            "packs intact"
        );
        assert_eq!(
            read(repo, "shared-config/.gitkeep"),
            "USERMARK\n",
            ".gitkeep intact"
        );
        assert!(
            !repo.join(".jigc/AGENT.md").exists(),
            "and the member that is not behind the link was not written either"
        );
    }

    // A linked directory on the way to the **guide** — a `.claude/skills` shared from
    // elsewhere. The guide's own oracle reads *absent* through it, so the artifact was
    // written into the shared directory and the install then failed at `git add`. It
    // refuses before the first write under the guide writer's code, and creates nothing
    // behind the link.
    {
        let (repo, home) = born_repo("anc-guide");
        let (repo, home) = (repo.path(), home.path());
        fs::create_dir_all(repo.join("shared-skills")).expect("create the shared dir");
        link(repo, ".claude/skills", "shared-skills");
        let head = git(repo, &["rev-parse", "HEAD"]);
        let out = jigc(repo, home, &["setup"]);
        assert_refused_before_any_write(
            repo,
            &out,
            refusal_code(GUIDE),
            &head,
            "linked `.claude/skills`",
        );
        assert!(
            !repo.join("shared-skills/jigc").exists(),
            "nothing was created behind the link"
        );
    }
}

/// (5) **Several at once are one refusal that names them all**, so the route is followed
/// once. The finding's code is the first blocked member's, in install order.
#[test]
fn several_blocked_paths_are_named_together() {
    let (repo, home) = born_repo("several");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "kept/agent", target_plant(".jigc/AGENT.md"));
    write(repo, "kept/version", target_plant(".jigc/version"));
    link(repo, ".jigc/AGENT.md", "kept/agent");
    link(repo, ".jigc/version", "kept/version");
    let head = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    assert_refused_before_any_write(repo, &out, "setup.write-bootstrap", &head, "several");
    let printed = route(&out);
    assert!(
        printed.contains("remove the link at `.jigc/AGENT.md`")
            && printed.contains("remove the link at `.jigc/version`"),
        "the route names both acts: {printed}"
    );

    fs::remove_file(repo.join(".jigc/AGENT.md")).expect("remove");
    fs::remove_file(repo.join(".jigc/version")).expect("remove");
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "both removed, one re-run installs: {}",
        said(&out)
    );
    assert!(read(repo, "kept/agent").contains(MARK) && read(repo, "kept/version").contains(MARK));
}

/// (6) **The control: a regular file jigc wrote is not an occupant.** A committed install
/// re-runs clean, so the rule above costs the ordinary path nothing.
#[test]
fn a_committed_install_still_reruns_clean() {
    let (repo, home) = born_repo("control");
    let (repo, home) = (repo.path(), home.path());
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(0), "install: {}", said(&out));
    let head = git(repo, &["rev-parse", "HEAD"]);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(0), "re-run: {}", said(&out));
    assert_eq!(git(repo, &["rev-parse", "HEAD"]), head, "no second commit");
    for member in replacing_members() {
        assert!(
            is_regular(repo, &member.path),
            "`{}` is a regular file",
            member.path
        );
    }
}

// ───────────────────────── the `jigc task finalize` door ─────────────────────────

/// Run `jigc doc <args>`, optionally piping `stdin`.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR");
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(out.status.success(), "{what} must succeed: {}", said(out));
}

/// The task the finalize cells drive.
const TASK: &str = "cache-sessions-in-a-single";

/// A born repository with a committed `jigc setup` install — the state every finalize cell
/// plants its `.jigc/version` occupant into.
fn installed_repo(tag: &str) -> (TempDir, TempDir) {
    let (repo, home) = born_repo(tag);
    assert_ok(&jigc(repo.path(), home.path(), &["setup"]), "`jigc setup`");
    assert_eq!(
        git(repo.path(), &["status", "--porcelain"]),
        "",
        "the install is committed whole"
    );
    (repo, home)
}

/// Open [`TASK`], author its commit doc and stage one code change — everything a finalize
/// needs to reach its stage step.
fn open_task_with_staged_code(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "single-task",
                "cache sessions in a single in-memory node",
            ],
        ),
        "`jigc start`",
    );
    let set_field = |addr: &str, value: &str| {
        assert_ok(
            &jigc_doc(repo, home, &["set-field", addr, "--value", value], None),
            &format!("set-field {addr}"),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        assert_ok(
            &jigc_doc(
                repo,
                home,
                &["set-slot", addr, "--from-file", "-"],
                Some(prose),
            ),
            &format!("set-slot {addr}"),
        );
    };
    set_field(&format!("commit:{TASK}#type"), "feat");
    set_field(&format!("commit:{TASK}#scope"), "cache");
    set_slot(&format!("commit:{TASK}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{TASK}#body"), b"A cache change.\n");
    write(repo, "README.md", "hello world\n");
    git(repo, &["add", "README.md"]);
}

/// The paths the `HEAD` commit touched.
fn head_paths(repo: &Path) -> Vec<String> {
    git(repo, &["show", "--name-only", "--pretty=format:", "HEAD"])
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

/// Assert the finalize **landed** and said it left the stamp alone: exit 0, the README in
/// the commit, `.jigc/version` not, and one advisory under the stamp writer's code.
fn assert_landed_and_said_so(repo: &Path, out: &std::process::Output, what: &str) {
    let said = said(out);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{what}: the commit lands: {said}"
    );
    let paths = head_paths(repo);
    assert!(
        paths.contains(&"README.md".to_string()),
        "{what}: the task's code is in the commit: {paths:?}"
    );
    assert!(
        !paths.contains(&".jigc/version".to_string()),
        "{what}: what is at `.jigc/version` is not jigc's to stage: {paths:?}"
    );
    assert!(
        said.contains(STAMP_CODE) && said.contains("advisory"),
        "{what}: and the door says so, under the stamp writer's own code: {said}"
    );
}

/// (7) **A hand-edited `.jigc/version` survives a finalize, and the finalize says so.**
/// Driven on `1.0.0-rc.24`: prose there, in no commit, was regenerated at exit 0 with no
/// finding about it and the rewrite committed as jigc's own.
///
/// Leave-and-say-so rather than refuse: the stamp is provenance and gates nothing, so a
/// finalize that stopped over it would hold the agent's whole commit hostage to a file it
/// did not write. The landed envelope carries the advisory under `--format json` too, and
/// the route — move it out, `jigc setup` — is followed.
#[test]
fn a_finalize_leaves_a_hand_edited_stamp_alone_and_says_so() {
    let prose = "USERMARK: we pin jigc here\nsee the team wiki\n";

    let (repo, home) = installed_repo("fin-prose");
    let (repo, home) = (repo.path(), home.path());
    open_task_with_staged_code(repo, home);
    write(repo, ".jigc/version", prose);

    // The forecast says what the commit will do — and makes no write of its own.
    let forecast = jigc(repo, home, &["task", "finalize", TASK, "--dry-run"]);
    assert_eq!(
        forecast.status.code(),
        Some(0),
        "dry-run: {}",
        said(&forecast)
    );
    assert!(
        said(&forecast).contains(STAMP_CODE),
        "the forecast carries the finding the landed door reports: {}",
        said(&forecast)
    );

    let out = jigc(repo, home, &["task", "finalize", TASK, "--format", "json"]);
    assert_landed_and_said_so(repo, &out, "hand-edited stamp");
    assert_eq!(
        read(repo, ".jigc/version"),
        prose,
        "the prose is byte-identical"
    );
    assert_eq!(
        git(repo, &["status", "--porcelain"]),
        "M .jigc/version",
        "and still the adopter's uncommitted edit — `git status` keeps prompting"
    );
    let envelope: serde_json::Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|err| panic!("stdout is one JSON document ({err}): {}", said(&out)));
    let findings = envelope["findings"].as_array().expect("a findings array");
    let stamp: Vec<_> = findings
        .iter()
        .filter(|finding| finding["code"] == STAMP_CODE)
        .collect();
    assert_eq!(stamp.len(), 1, "exactly one stamp finding: {envelope}");
    assert_eq!(stamp[0]["severity"], "advisory", "and it gates nothing");

    // The route, followed: move the file out, `jigc setup` writes the stamp again.
    let printed = stamp[0]["route"].to_string();
    assert!(
        printed.contains("move the file at `.jigc/version` out of the way")
            && printed.contains("`jigc setup`"),
        "the route: {printed}"
    );
    fs::rename(repo.join(".jigc/version"), repo.join("version-notes.txt")).expect("move it out");
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the route's `jigc setup`: {}",
        said(&out)
    );
    assert!(
        read(repo, ".jigc/version").starts_with("jigc-version: "),
        "jigc's stamp is back"
    );
    assert_eq!(
        read(repo, "version-notes.txt"),
        prose,
        "and the notes are kept"
    );
}

/// (8) **A finalize never writes the stamp through a link** — committed or not, and
/// whatever its target holds. The third shape is the one an oracle that *reads* through the
/// link gets wrong: a target that happens to hold a jigc-shaped line is still a file
/// outside `.jigc/` that jigc does not own.
#[test]
fn a_finalize_never_writes_the_stamp_through_a_link() {
    let shapes: [(&str, &str, bool); 3] = [
        ("committed link", "USERMARK: team notes\n", true),
        ("uncommitted link", "USERMARK: team notes\n", false),
        (
            "link to a stamp-shaped file",
            "jigc-version: 0.0.1-USERMARK\n",
            false,
        ),
    ];
    for (what, target, commit_the_link) in shapes {
        let (repo, home) = installed_repo("fin-link");
        let (repo, home) = (repo.path(), home.path());
        write(repo, "kept/version-notes", target);
        fs::remove_file(repo.join(".jigc/version")).expect("remove jigc's stamp");
        link(repo, ".jigc/version", "kept/version-notes");
        if commit_the_link {
            git(repo, &["add", "--", ".jigc/version"]);
            git(repo, &["commit", "-q", "-m", "link the stamp"]);
        }
        open_task_with_staged_code(repo, home);

        let out = jigc(repo, home, &["task", "finalize", TASK]);
        assert_landed_and_said_so(repo, &out, what);
        assert_eq!(
            read(repo, "kept/version-notes"),
            target,
            "{what}: the write did not go through the link"
        );
        assert!(
            is_link(repo, ".jigc/version"),
            "{what}: the link is still a link"
        );
        let mode = git(repo, &["ls-tree", "HEAD", "--", ".jigc/version"]);
        assert_eq!(
            mode.starts_with("120000"),
            commit_the_link,
            "{what}: the commit did not sweep the adopter's link in or out: {mode}"
        );
    }
}

/// (9) **A directory at the stamp's path is left alone too — and the forecast and the door
/// agree about it.** The transaction used to take a pre-image of `.jigc/version` whatever
/// stood there, and that read fails on a directory: with the stamp refresh already leaving
/// the entry alone, `--dry-run` forecast a landed commit and the committing run then
/// refused over *a file it cannot put back* — one it was not going to rewrite. A stamp that
/// is not jigc's to write is not captured, so the commit lands as forecast.
#[test]
fn a_finalize_lands_over_a_directory_at_the_stamp_path_as_its_forecast_says() {
    let (repo, home) = installed_repo("fin-dir");
    let (repo, home) = (repo.path(), home.path());
    git(repo, &["rm", "-q", "--", ".jigc/version"]);
    git(repo, &["commit", "-q", "-m", "drop the stamp"]);
    write(repo, ".jigc/version/notes", "USERMARK\n");
    open_task_with_staged_code(repo, home);

    let forecast = jigc(repo, home, &["task", "finalize", TASK, "--dry-run"]);
    assert_eq!(
        forecast.status.code(),
        Some(0),
        "dry-run: {}",
        said(&forecast)
    );
    assert!(
        said(&forecast).contains(STAMP_CODE),
        "the forecast says the stamp is left alone: {}",
        said(&forecast)
    );

    let out = jigc(repo, home, &["task", "finalize", TASK]);
    assert_landed_and_said_so(repo, &out, "directory at the stamp path");
    assert_eq!(
        read(repo, ".jigc/version/notes"),
        "USERMARK\n",
        "and what the directory holds is intact"
    );
    assert!(
        said(&out).contains("move the directory at `.jigc/version` out of the way"),
        "the route names the act that fits a directory: {}",
        said(&out)
    );
}

/// (10) **The control: jigc's own stamp, stale, is still refreshed and committed.** The
/// oracle's *yes* arm — a one-line `jigc-version:` record is jigc's whichever build wrote
/// it, and a store-writing op keeps it current (`design/storage.md` → Store provenance).
#[test]
fn a_finalize_still_refreshes_a_stale_stamp_that_is_jigcs_own() {
    let (repo, home) = installed_repo("fin-stale");
    let (repo, home) = (repo.path(), home.path());
    write(repo, ".jigc/version", "jigc-version: 0.0.1-earlier\n");
    git(repo, &["add", "--", ".jigc/version"]);
    git(repo, &["commit", "-q", "-m", "an older build's stamp"]);
    open_task_with_staged_code(repo, home);

    let out = jigc(repo, home, &["task", "finalize", TASK]);
    let said = said(&out);
    assert_eq!(out.status.code(), Some(0), "lands: {said}");
    assert!(
        !said.contains(STAMP_CODE),
        "and raises nothing about the stamp: {said}"
    );
    assert!(
        !read(repo, ".jigc/version").contains("0.0.1-earlier"),
        "the stamp is refreshed"
    );
    assert!(
        head_paths(repo).contains(&".jigc/version".to_string()),
        "and rides the commit"
    );
    assert_eq!(
        git(repo, &["status", "--porcelain"]),
        "",
        "nothing left over"
    );
}
