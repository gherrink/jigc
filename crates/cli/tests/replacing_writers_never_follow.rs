//! **A file jigc replaces whole is written as a regular file at exactly its path, never
//! through a link** (the rc.24 fix pass; `DECISIONS.md` → 2026-10-04, the symlink fork;
//! `completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R1-F1.md` → class
//! sibling 2).
//!
//! `jigc setup` puts its own content at five paths — `.jigc/AGENT.md`, `.jigc/version`,
//! `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml` and the adapter's guide artifact — and
//! every one of those writers opened its destination through whatever was there. (A sixth
//! joined them on 2026-10-06, the settings record `.jigc/settings-entries.json`; it was
//! written through the no-follow writer from its first day and is driven with the rest.) Driven on
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
//! **And a file jigc merges into is followed through a link only to a regular file inside
//! the repository** (the rc.24 fix pass's completion audit, the end-to-end tester's
//! F2; cells m1–m3). The rule above left the merged-into members unasked, and `jigc setup`
//! wrote through a committed `CLAUDE.md` link into a file outside the repository at exit
//! 0. Cell m1 iterates the member × where its link leads, m2 is the must-not-refuse half —
//! `CLAUDE.md -> AGENTS.md` under each conversion setting and in each layout this door is
//! reachable in — and m3 holds the file behind an accepted link to the dirty-install guard.
//!
//! **A link to a file git ignores is followed too** (cell m4; the human's ruling of
//! 2026-10-06 on the fix pass's item 18, reversing the refusal m1 first pinned). The target
//! is treated as an ignored ordinary file at the member's own path is: merged into, in no
//! commit. `1.0.0-rc.24` did that; the pass refused it under the member's write-failure
//! code, and a merge loses nothing either way. What m1 still refuses is every end that is
//! no working file of this repository: out of it, nowhere, a directory, git's own directory.
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
        ".jigc/settings-entries.json" => "{\n  \"USERMARK\": \"our own notes\"\n}\n",
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
        // The settings record rides the code of the settings merge it belongs to.
        ".jigc/settings-entries.json" => "setup.inject-allowlist",
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
            ".jigc/settings-entries.json",
            GUIDE,
        ],
        "the replacing members of the install — a seventh is driven here the day it is \
         declared"
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

// ──── merged-into members: a link is followed only to a regular file in this repository ────

/// Run `git` in `repo` without asserting.
fn git_try(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git")
}

/// The existing write-failure code a merged-into member's link refusal rides.
fn merge_refusal_code(path: &str) -> &'static str {
    match path {
        "CLAUDE.md" => "setup.inject-reference",
        ".claude/settings.json" => "setup.inject-allowlist",
        ".jigc/.gitignore" => "setup.init-project-layer",
        ".gitignore" => "setup.secrets-gitignore",
        other => panic!("`{other}` is a merged-into member with no pinned refusal code"),
    }
}

/// Bytes an adopter could keep in the file a link at a merged-into member leads to.
fn merged_plant(path: &str) -> &'static str {
    match path {
        ".claude/settings.json" => "{\n  \"env\": { \"TEAM\": \"USERMARK\" }\n}\n",
        _ => "# House rules — USERMARK\n\nNever deploy on a Friday.\n",
    }
}

/// (m1) **A link at a merged-into member that leads anywhere the repository cannot commit
/// refuses before the first write** (the rc.24 fix pass's completion audit, the end-to-end
/// tester's F2).
///
/// Driven on `1.0.0-rc.24` and at `b54b58b2`: a committed `CLAUDE.md` or
/// `.claude/settings.json` link to a file **outside the repository** — `jigc setup` exit 0,
/// that file rewritten with jigc's section, `git status` empty, the ack naming an install
/// no commit carried; **dangling** — the file created wherever the link pointed.
///
/// The axis is the member × where its link leads. For the two host files: out of the
/// repository, nowhere, to a directory in it, into git's own directory. In each, plain and
/// `--force`: exit 1 under the member's own write-failure code, nothing installed, the
/// link's end byte-identical (or still absent). Then the route as printed — the link
/// removed, the removal committed — and one re-run installs a regular file there with the
/// link's old end still untouched.
///
/// **An ignored file inside the repository was a fifth end here and is not one any more**
/// (2026-10-06, the human's ruling on item 18): it is followed — cell m4. *Inside git's own
/// directory* took its place on the axis, because the refusal's sentence had covered both
/// in one clause and only one of them is still true.
#[test]
fn a_merged_into_member_refuses_a_link_the_repository_cannot_commit() {
    let ends = ["outside", "dangling", "directory", "git-dir"];
    std::thread::scope(|scope| {
        for path in ["CLAUDE.md", ".claude/settings.json"] {
            for end in ends {
                scope.spawn(move || {
                    let what = format!("`{path}` → {end}");
                    let (repo, home) = born_repo("merged-link");
                    let elsewhere = TempDir::new("merged-link-elsewhere");
                    let (repo, home) = (repo.path(), home.path());
                    // Where the link leads, and the file whose bytes must not move.
                    let (target, kept): (String, Option<PathBuf>) = match end {
                        "outside" => {
                            let file = elsewhere.path().join("shared");
                            fs::write(&file, merged_plant(path)).expect("plant");
                            (file.display().to_string(), Some(file))
                        }
                        "dangling" => {
                            let file = elsewhere.path().join("nowhere");
                            (file.display().to_string(), Some(file))
                        }
                        "git-dir" => (
                            ".git/description".to_string(),
                            Some(repo.join(".git/description")),
                        ),
                        _ => {
                            write(repo, "shared-dir/keep", "x\n");
                            ("shared-dir".to_string(), None)
                        }
                    };
                    link(repo, path, &target);
                    git(repo, &["add", "-A"]);
                    git(repo, &["commit", "-q", "-m", "our layout"]);
                    let head = git(repo, &["rev-parse", "HEAD"]);
                    let before = kept.as_ref().map(|file| fs::read(file).ok());

                    for args in [&["setup"][..], &["setup", "--force"][..]] {
                        let out = jigc(repo, home, args);
                        let said = said(&out);
                        assert_eq!(out.status.code(), Some(1), "{what} {args:?}: {said}");
                        assert!(
                            said.contains(merge_refusal_code(path))
                                && !said.contains("setup.dirty-install-path"),
                            "{what} {args:?}: under the member's own code: {said}"
                        );
                        assert_eq!(git(repo, &["rev-parse", "HEAD"]), head, "{what}: no commit");
                        assert!(
                            !repo.join(".jigc").exists(),
                            "{what} {args:?}: asked before the first write: {said}"
                        );
                        assert!(is_link(repo, path), "{what}: the link is as it was");
                        assert_eq!(
                            kept.as_ref().map(|file| fs::read(file).ok()),
                            before,
                            "{what} {args:?}: where the link leads is byte-identical"
                        );
                    }

                    // The route as printed: remove the link, commit the removal, re-run.
                    let said_route = route(&jigc(repo, home, &["setup"]));
                    assert!(
                        said_route.contains(&format!("remove the link at `{path}`")),
                        "{what}: {said_route}"
                    );
                    git(repo, &["rm", "-q", "--", path]);
                    git(repo, &["commit", "-q", "-m", "drop the link"]);
                    let out = jigc(repo, home, &["setup"]);
                    assert_eq!(out.status.code(), Some(0), "{what}: {}", said(&out));
                    assert!(is_regular(repo, path), "{what}: a regular file now");
                    assert_eq!(
                        kept.as_ref().map(|file| fs::read(file).ok()),
                        before,
                        "{what}: and the link's old end was never written"
                    );
                    assert_eq!(git(repo, &["status", "--porcelain"]), "", "{what}");
                });
            }
        }
    });

    // `.jigc/.gitignore` is amended in place and never through a link, wherever it leads —
    // its writer said so only after the bootstrap file had been written.
    let (repo, home) = born_repo("merged-link-ignore");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "our-ignores", "scratch/\n");
    link(repo, ".jigc/.gitignore", "our-ignores");
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "our layout"]);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(1), "{}", said(&out));
    assert!(
        said(&out).contains(merge_refusal_code(".jigc/.gitignore")),
        "{}",
        said(&out)
    );
    assert!(
        !repo.join(".jigc/AGENT.md").exists() && !repo.join("CLAUDE.md").exists(),
        "asked before the first write: {}",
        said(&out)
    );
    assert_eq!(read(repo, "our-ignores"), "scratch/\n");

    // The secrets-floor `.gitignore` of a repository with no commit: the same rule.
    let (repo, home) = unborn_repo("merged-link-floor");
    let elsewhere = TempDir::new("merged-link-floor-elsewhere");
    let (repo, home) = (repo.path(), home.path());
    let shared = elsewhere.path().join("gitignore");
    fs::write(&shared, "*.log\n").expect("plant");
    link(repo, ".gitignore", &shared.display().to_string());
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(1), "{}", said(&out));
    assert!(
        said(&out).contains(merge_refusal_code(".gitignore")),
        "{}",
        said(&out)
    );
    assert_eq!(fs::read_to_string(&shared).expect("read"), "*.log\n");
    assert!(!repo.join(".jigc").exists(), "nothing was installed");

    // The `pre-commit` hook, the fifth merged-into member: a link out of the repository
    // and its hooks directory, or to nothing.
    for end in ["outside", "dangling"] {
        let (repo, home) = born_repo("merged-link-hook");
        let elsewhere = TempDir::new("merged-link-hook-elsewhere");
        let (repo, home) = (repo.path(), home.path());
        let shared = elsewhere.path().join("pre-commit");
        if end == "outside" {
            fs::write(&shared, "#!/bin/sh\necho ours # USERMARK\n").expect("plant");
        }
        let before = fs::read(&shared).ok();
        fs::create_dir_all(repo.join(".git/hooks")).expect("hooks dir");
        symlink(&shared, repo.join(".git/hooks/pre-commit")).expect("plant the link");
        for args in [&["setup"][..], &["setup", "--force"][..]] {
            let out = jigc(repo, home, args);
            assert_eq!(out.status.code(), Some(1), "hook → {end}: {}", said(&out));
            assert!(
                said(&out).contains("setup.install-hook"),
                "hook → {end}: {}",
                said(&out)
            );
            assert!(
                !repo.join(".jigc").exists(),
                "hook → {end}: nothing installed"
            );
            assert_eq!(fs::read(&shared).ok(), before, "hook → {end}: untouched");
        }
        fs::remove_file(repo.join(".git/hooks/pre-commit")).expect("the route: remove the link");
        let out = jigc(repo, home, &["setup"]);
        assert_eq!(out.status.code(), Some(0), "hook → {end}: {}", said(&out));
        assert_eq!(
            fs::read(&shared).ok(),
            before,
            "hook → {end}: never written"
        );
    }
}

/// (m2) **…and a link to a file the repository tracks keeps working, and the install
/// answers for the file it wrote** — the must-not-refuse half of (m1). `CLAUDE.md ->
/// AGENTS.md` is an ordinary layout, and `1.0.0-rc.24` installs there at exit 0; what it
/// got wrong is what happened next: `AGENTS.md` was left modified and uncommitted beside an
/// install commit that did not name it.
///
/// Iterated over the conversion settings a checkout can carry × the layouts this door is
/// reachable in (a plain checkout, a fresh clone, a linked worktree the user made): the
/// install lands, says which file it merged into, **commits that file**, leaves the
/// repository clean, and a re-run changes nothing. The link is still a link and the
/// adopter's lines are still in the file behind it. Then the teardown unwires the same
/// file through the same link.
#[test]
fn a_link_to_a_tracked_file_is_merged_through_and_committed() {
    std::thread::scope(|scope| {
        for conversion in [
            "none",
            "core.autocrlf=true",
            "core.autocrlf=input",
            "* text=auto",
        ] {
            for layout in ["plain", "clone", "linked worktree"] {
                scope.spawn(move || {
                    let what = format!("{conversion} / {layout}");
                    let (origin, home) = born_repo("merged-through");
                    let aside = TempDir::new("merged-through-aside");
                    let home = home.path();
                    let configure = |repo: &Path| match conversion {
                        "none" | "* text=auto" => {}
                        setting => {
                            let (key, value) = setting.split_once('=').expect("key=value");
                            git(repo, &["config", key, value]);
                        }
                    };
                    let seed = origin.path();
                    if conversion == "* text=auto" {
                        write(seed, ".gitattributes", "* text=auto\n");
                    }
                    write(seed, "AGENTS.md", merged_plant("CLAUDE.md"));
                    write(
                        seed,
                        "config/claude.json",
                        merged_plant(".claude/settings.json"),
                    );
                    link(seed, "CLAUDE.md", "AGENTS.md");
                    link(seed, ".claude/settings.json", "config/claude.json");
                    git(seed, &["add", "-A"]);
                    git(seed, &["commit", "-q", "-m", "our layout"]);
                    configure(seed);
                    // The checkout `jigc setup` is typed in, and the one it installs at.
                    let (typed_in, installed_at): (PathBuf, PathBuf) = match layout {
                        "plain" => (seed.to_path_buf(), seed.to_path_buf()),
                        "clone" => {
                            let clone = aside.path().join("clone");
                            git(
                                aside.path(),
                                // `--no-local`: over the pack transport, not a file-by-file
                                // copy of the origin's object directory — that copy lost a
                                // race once under the full gate's load (*failed to copy
                                // file … No such file or directory*).
                                &[
                                    "clone",
                                    "-q",
                                    "--no-local",
                                    &seed.display().to_string(),
                                    &clone.display().to_string(),
                                ],
                            );
                            git(&clone, &["config", "user.email", "test@example.com"]);
                            git(&clone, &["config", "user.name", "Test"]);
                            configure(&clone);
                            (clone.clone(), clone)
                        }
                        _ => {
                            let linked = aside.path().join("linked");
                            git(
                                seed,
                                &[
                                    "worktree",
                                    "add",
                                    "-q",
                                    "-b",
                                    "side",
                                    &linked.display().to_string(),
                                ],
                            );
                            (linked, seed.to_path_buf())
                        }
                    };
                    let repo = installed_at.as_path();
                    assert_eq!(git(repo, &["status", "--porcelain"]), "", "{what}: premise");

                    let out = jigc(&typed_in, home, &["setup"]);
                    assert_eq!(out.status.code(), Some(0), "{what}: {}", said(&out));
                    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                    for (member, target) in [
                        ("CLAUDE.md", "AGENTS.md"),
                        (".claude/settings.json", "config/claude.json"),
                    ] {
                        assert!(
                            stderr.contains(&format!("`{member}` is a link"))
                                && stderr.contains(&format!("`{target}`")),
                            "{what}: the ack says which file was written: {stderr}"
                        );
                        assert!(is_link(repo, member), "{what}: `{member}` is still a link");
                        assert!(
                            git(repo, &["show", "--stat", "--format=", "HEAD"]).contains(target),
                            "{what}: the install commit carries `{target}`"
                        );
                        assert!(
                            read(repo, target).contains(MARK),
                            "{what}: the adopter's bytes are still in `{target}`"
                        );
                    }
                    assert!(
                        read(repo, "AGENTS.md").contains("@.jigc/AGENT.md"),
                        "{what}: the reference was merged into the file behind the link"
                    );
                    assert_eq!(git(repo, &["status", "--porcelain"]), "", "{what}: clean");
                    let head = git(repo, &["rev-parse", "HEAD"]);
                    let rerun = jigc(&typed_in, home, &["setup"]);
                    assert_eq!(rerun.status.code(), Some(0), "{what}: {}", said(&rerun));
                    assert_eq!(
                        git(repo, &["rev-parse", "HEAD"]),
                        head,
                        "{what}: no second commit"
                    );
                    assert_eq!(
                        git(repo, &["status", "--porcelain"]),
                        "",
                        "{what}: still clean"
                    );

                    // The teardown goes back through the same link.
                    let out = jigc(repo, home, &["uninstall"]);
                    assert_eq!(out.status.code(), Some(0), "{what}: {}", said(&out));
                    assert!(is_link(repo, "CLAUDE.md"), "{what}: the link survives");
                    assert!(
                        !read(repo, "AGENTS.md").contains("@.jigc/AGENT.md")
                            && read(repo, "AGENTS.md").contains(MARK),
                        "{what}: jigc's section is out of `AGENTS.md`, the adopter's lines are in"
                    );
                });
            }
        }
    });

    // The hook, linked to a script the repository tracks: followed, as before.
    let (repo, home) = born_repo("merged-through-hook");
    let (repo, home) = (repo.path(), home.path());
    write(
        repo,
        "scripts/pre-commit",
        "#!/bin/sh\necho ours # USERMARK\n",
    );
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "our hook"]);
    fs::create_dir_all(repo.join(".git/hooks")).expect("hooks dir");
    symlink(
        "../../scripts/pre-commit",
        repo.join(".git/hooks/pre-commit"),
    )
    .expect("link");
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "a hook linked into the repository: {}",
        said(&out)
    );
    assert!(
        read(repo, "scripts/pre-commit").contains(MARK)
            && read(repo, "scripts/pre-commit").contains("jigc-managed"),
        "jigc's block is spliced into the tracked script, beside the adopter's lines"
    );
}

/// (m3) **Whether git holds the file behind the link is git's to say** — the link's end
/// joins the install's path class, so the dirty-install guard answers for it exactly as it
/// does for a regular `CLAUDE.md`.
///
/// - born `HEAD`, the file **untracked** ⇒ `setup.dirty-install-path` naming *that file*,
///   nothing written; committed as the route says, one re-run installs.
/// - born `HEAD`, the file tracked with an **uncommitted edit** ⇒ the same refusal, the
///   edit byte-identical.
/// - **unborn** `HEAD`, link and file both untracked ⇒ exit 0: the declared on-ramp, and
///   the first commit carries both.
#[test]
fn the_file_behind_a_link_is_asked_about_like_any_install_path() {
    // Untracked behind a committed link.
    let (repo, home) = born_repo("behind-untracked");
    let (repo, home) = (repo.path(), home.path());
    link(repo, "CLAUDE.md", "AGENTS.md");
    git(repo, &["add", "--", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "the link only"]);
    write(repo, "AGENTS.md", merged_plant("CLAUDE.md"));
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(1), "{}", said(&out));
    assert!(
        said(&out).contains("setup.dirty-install-path") && said(&out).contains("  `AGENTS.md`"),
        "the file the merge would land in is named: {}",
        said(&out)
    );
    assert_eq!(
        read(repo, "AGENTS.md"),
        merged_plant("CLAUDE.md"),
        "untouched"
    );
    assert!(!repo.join(".jigc").exists(), "nothing installed");
    git(repo, &["add", "--", "AGENTS.md"]);
    git(repo, &["commit", "-q", "-m", "our rules"]);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "committed, one re-run: {}",
        said(&out)
    );
    assert_eq!(git(repo, &["status", "--porcelain"]), "");

    // Tracked behind the link, with an edit no commit holds.
    let (repo, home) = born_repo("behind-edited");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "AGENTS.md", merged_plant("CLAUDE.md"));
    link(repo, "CLAUDE.md", "AGENTS.md");
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "our layout"]);
    let edited = format!(
        "{}One more rule nobody committed.\n",
        merged_plant("CLAUDE.md")
    );
    write(repo, "AGENTS.md", &edited);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(1), "{}", said(&out));
    assert!(
        said(&out).contains("setup.dirty-install-path") && said(&out).contains("  `AGENTS.md`"),
        "{}",
        said(&out)
    );
    assert_eq!(read(repo, "AGENTS.md"), edited, "byte-identical");

    // No commit yet: the on-ramp.
    let (repo, home) = unborn_repo("behind-unborn");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "AGENTS.md", merged_plant("CLAUDE.md"));
    link(repo, "CLAUDE.md", "AGENTS.md");
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(0), "the on-ramp: {}", said(&out));
    let committed = git(repo, &["ls-tree", "-r", "--name-only", "HEAD"]);
    assert!(
        committed.lines().any(|path| path == "AGENTS.md")
            && committed.lines().any(|path| path == "CLAUDE.md"),
        "the first commit carries the link and the file behind it: {committed}"
    );
    assert!(
        git_try(repo, &["diff", "--quiet"]).status.success(),
        "clean"
    );
}

/// What jigc's merge leaves in the file a member's writes land in.
fn merged_in(path: &str) -> &'static str {
    match path {
        ".claude/settings.json" => "Bash(jigc:*)",
        _ => "@.jigc/AGENT.md",
    }
}

/// Whether git's index or `HEAD` holds `path`.
fn git_holds(repo: &Path, path: &str) -> bool {
    !git(repo, &["ls-files", "--", path]).is_empty()
        || git(repo, &["ls-tree", "-r", "--name-only", "HEAD"])
            .lines()
            .any(|line| line == path)
}

/// (m4) **A link at a merged-into member that leads to a file git ignores is followed, and
/// the file is treated as an ignored ordinary file at the member's own path is** — merged
/// into, in no commit (the human's ruling of 2026-10-06 on the fix pass's item 18).
///
/// The pass refused this end (`setup.inject-reference` · `setup.inject-allowlist`) where
/// `1.0.0-rc.24` followed the link and merged, and where an ignored ordinary `CLAUDE.md`
/// has always been merged into at exit 0. The axis: the two host files × whether git holds
/// the link itself (committed, or covered by the same ignore rule) × plain and `--force`.
/// In each: exit 0, no refusal code, the adopter's bytes and jigc's merge both in the file
/// behind the link, that file in no commit and no index entry, the ack saying so, the
/// repository clean, a re-run moving nothing — and the teardown going back through the same
/// link: jigc's section out of the file behind a `CLAUDE.md` link, and, behind a settings
/// link, every entry left and named, because git does not track the file the entries are in
/// (the ruling on item 21). The control beside it is the ordinary file the ruling compares
/// it to.
#[test]
fn a_link_to_an_ignored_file_is_followed_and_its_target_joins_no_commit() {
    std::thread::scope(|scope| {
        for path in ["CLAUDE.md", ".claude/settings.json"] {
            for link_state in ["committed", "ignored"] {
                for args in [&["setup"][..], &["setup", "--force"][..]] {
                    scope.spawn(move || {
                        let what =
                            format!("`{path}` → an ignored file, the link {link_state}, {args:?}");
                        let (repo, home) = born_repo("ignored-end");
                        let (repo, home) = (repo.path(), home.path());
                        let rules = match link_state {
                            "committed" => "local/\n".to_string(),
                            _ => format!("local/\n/{path}\n"),
                        };
                        write(repo, ".gitignore", &rules);
                        write(repo, "local/mine", merged_plant(path));
                        link(repo, path, "local/mine");
                        git(repo, &["add", "-A"]);
                        git(repo, &["commit", "-q", "-m", "our layout"]);
                        assert_eq!(
                            git_holds(repo, path),
                            link_state == "committed",
                            "{what}: premise — whether git holds the link"
                        );
                        assert!(!git_holds(repo, "local/mine"), "{what}: premise");

                        let out = jigc(repo, home, args);
                        let said_out = said(&out);
                        assert_eq!(out.status.code(), Some(0), "{what}: {said_out}");
                        assert!(
                            !said_out.contains(merge_refusal_code(path))
                                && !said_out.contains("setup.dirty-install-path")
                                && !said_out.contains("setup.forced-install-path"),
                            "{what}: nothing refuses and nothing was consented over: {said_out}"
                        );
                        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                        assert!(
                            stderr.contains(&format!("`{path}` is a link"))
                                && stderr.contains("`local/mine`")
                                && stderr.contains("git ignores that file")
                                && !stderr.contains("the install commit carries"),
                            "{what}: the ack names the file and says no commit carries it: {stderr}"
                        );
                        assert!(is_link(repo, path), "{what}: still a link");
                        let merged = read(repo, "local/mine");
                        assert!(
                            merged.contains(MARK) && merged.contains(merged_in(path)),
                            "{what}: the adopter's bytes and jigc's merge are both there: {merged}"
                        );
                        assert!(
                            !git_holds(repo, "local/mine"),
                            "{what}: the file behind the link is in no commit and no index entry"
                        );
                        assert!(
                            git(repo, &["log", "-1", "--format=%s"]).contains("install jigc"),
                            "{what}: the rest of the install is committed"
                        );
                        assert_eq!(git(repo, &["status", "--porcelain"]), "", "{what}: clean");
                        let head = git(repo, &["rev-parse", "HEAD"]);
                        let rerun = jigc(repo, home, &["setup"]);
                        assert_eq!(rerun.status.code(), Some(0), "{what}: {}", said(&rerun));
                        assert_eq!(git(repo, &["rev-parse", "HEAD"]), head, "{what}: no-op");
                        assert_eq!(read(repo, "local/mine"), merged, "{what}: byte-stable");

                        // The teardown goes back through the same link.
                        let out = jigc(repo, home, &["uninstall"]);
                        assert_eq!(out.status.code(), Some(0), "{what}: {}", said(&out));
                        assert!(is_link(repo, path), "{what}: the link survives");
                        let after = read(repo, "local/mine");
                        if path == ".claude/settings.json" {
                            // The settings entries are left where git does not track the
                            // file they are in (2026-10-06, the human's ruling on item
                            // 21) — and tracked is asked of the file behind the link,
                            // where the entries are, not of the committed link.
                            assert_eq!(
                                after, merged,
                                "{what}: an ignored settings file keeps every entry"
                            );
                            let said_out = said(&out);
                            assert!(
                                said_out.contains("warning: left ")
                                    && said_out
                                        .contains("git does not track `.claude/settings.json`")
                                    && said_out.contains("`Bash(jigc:*)`"),
                                "{what}: and the teardown names what it left: {said_out}"
                            );
                        } else {
                            assert!(
                                after.contains(MARK) && !after.contains(merged_in(path)),
                                "{what}: jigc's merge is out, the adopter's bytes are in: \
                                 {after}"
                            );
                        }
                    });
                }
            }
        }
    });

    // The control the ruling compares it to: the same bytes as an ignored ordinary file.
    for path in ["CLAUDE.md", ".claude/settings.json"] {
        let (repo, home) = born_repo("ignored-ordinary");
        let (repo, home) = (repo.path(), home.path());
        write(repo, ".gitignore", &format!("/{path}\n"));
        write(repo, path, merged_plant(path));
        git(repo, &["add", "-A"]);
        git(repo, &["commit", "-q", "-m", "our layout"]);
        let out = jigc(repo, home, &["setup"]);
        assert_eq!(
            out.status.code(),
            Some(0),
            "ordinary `{path}`: {}",
            said(&out)
        );
        let merged = read(repo, path);
        assert!(
            merged.contains(MARK) && merged.contains(merged_in(path)) && !git_holds(repo, path),
            "ordinary `{path}`: merged into, in no commit: {merged}"
        );
        assert_eq!(git(repo, &["status", "--porcelain"]), "");
    }

    // No commit yet: the link rides the first commit, the ignored file behind it does not.
    let (repo, home) = unborn_repo("ignored-end-unborn");
    let (repo, home) = (repo.path(), home.path());
    write(repo, ".git/info/exclude", "local/\n");
    write(repo, "local/mine", merged_plant("CLAUDE.md"));
    link(repo, "CLAUDE.md", "local/mine");
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(0), "unborn: {}", said(&out));
    assert!(
        git_holds(repo, "CLAUDE.md") && !git_holds(repo, "local/mine"),
        "unborn: the first commit carries the link and not the file git ignores"
    );
    assert!(
        read(repo, "local/mine").contains(MARK)
            && read(repo, "local/mine").contains("@.jigc/AGENT.md"),
        "unborn: merged into"
    );

    // And following the link buys nothing for the link itself: an untracked link on a born
    // `HEAD` is an install path git holds no copy of, refused by name before any write — as
    // an untracked ordinary `CLAUDE.md` is — and committed as the route says, it installs.
    let (repo, home) = born_repo("ignored-end-untracked-link");
    let (repo, home) = (repo.path(), home.path());
    write(repo, ".gitignore", "local/\n");
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "our ignores"]);
    write(repo, "local/mine", merged_plant("CLAUDE.md"));
    link(repo, "CLAUDE.md", "local/mine");
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(1), "{}", said(&out));
    assert!(
        said(&out).contains("setup.dirty-install-path") && said(&out).contains("  `CLAUDE.md`"),
        "the untracked link is named: {}",
        said(&out)
    );
    assert!(!repo.join(".jigc").exists(), "asked before the first write");
    assert_eq!(
        read(repo, "local/mine"),
        merged_plant("CLAUDE.md"),
        "untouched"
    );
    git(repo, &["add", "--", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "our link"]);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "committed, one re-run: {}",
        said(&out)
    );
    assert!(
        !git_holds(repo, "local/mine"),
        "and the file behind it is still in no commit"
    );
}
