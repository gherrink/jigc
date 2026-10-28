//! **No door writes past a link on the way** (2026-10-28, the human's ruling B;
//! `design/storage.md` → *No write through a link on the way*) — every leaf verb, driven
//! through the real binary over a link out of the repository at the workbench and at a
//! doctype's home.
//!
//! The byte table holds the doors the rule was found at, cell by cell, each at the place
//! its row aims a link at (`r1-a-write-follows-a-link-at-a-directory-on-the-way-out-of-the-
//! repository`). This suite holds the guard's **reach**: what the guard does at a command
//! is the parsed command's `cli::cli::LinkGuard`, so every leaf is driven — each from its
//! one minimal argv ([`MINIMAL_ARGV`], which the guard answers before any id or path is
//! resolved) — and a leaf added tomorrow is driven the day it is added, because the argv
//! table is fenced to the clap tree.
//!
//! What one cell claims, by the command's `LinkGuard`:
//!
//! - **`Refuses` — it refuses, and changed nothing**: exit 1, the code
//!   `store.link-on-the-way`, the link named by its repo-relative path, and neither the
//!   repository (its tree, its index, its `HEAD`) nor what the link leads to moved a byte.
//! - **`Reports` — a read form names the link and runs**, writing nothing past it.
//! - **`StandsDown` — `uninstall` writes nothing past the link**: what the link leads to
//!   is byte for byte as it was.
//!
//! **And the review of 2026-10-28, a test a finding**, each red on `63ce9ca5` first.

use crate::support::leaf_argv::{FIXTURE_FILES, MINIMAL_ARGV};
use crate::support::trial_corpus::{State, TrialCorpus};
use cli::cli::{BEHALF_DOORS, LINK_GUARD_EXCEPTIONS, LinkGuard};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// The refusal's code, as the binary prints it.
const CODE: &str = "store.link-on-the-way";

/// Every entry below `dir`, read without following a link: a file's bytes, a link's
/// target, a directory's name — the state a refusal may not change.
fn snapshot(dir: &Path, skip: &[&str]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(at) = pending.pop() {
        let Ok(entries) = fs::read_dir(&at) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let rel = path
                .strip_prefix(dir)
                .expect("below the snapshot's root")
                .to_string_lossy()
                .into_owned();
            if skip.iter().any(|skipped| rel == *skipped) {
                continue;
            }
            let kind = entry.file_type().expect("an entry's type");
            if kind.is_symlink() {
                let target = fs::read_link(&path).expect("read a link");
                out.insert(rel, format!("link -> {}", target.display()));
            } else if kind.is_dir() {
                out.insert(rel, "dir".to_string());
                pending.push(path);
            } else {
                let bytes = fs::read(&path).expect("read a file");
                out.insert(rel, String::from_utf8_lossy(&bytes).into_owned());
            }
        }
    }
    out
}

/// The repository as git and the disk hold it: the tree (but `.git`), the index and `HEAD`.
fn repository(corpus: &TrialCorpus) -> (BTreeMap<String, String>, String, String) {
    (
        snapshot(&corpus.repo(), &[".git"]),
        corpus.git(&["ls-files", "--stage"]),
        corpus.git(&["rev-parse", "HEAD"]),
    )
}

/// A fresh corpus whose directory `at` (made where it is absent) is moved out of the
/// repository with what it holds, and a link left in its place — with the files the
/// minimal argv's tails name. Returns the corpus and the directory the link leads to.
fn linked(at: &str) -> (TrialCorpus, std::path::PathBuf) {
    let corpus = TrialCorpus::build(State::Fresh);
    for (name, body) in FIXTURE_FILES {
        fs::write(corpus.repo().join(name), body).expect("write a tail's file");
    }
    let link = corpus.repo().join(at);
    let outside = corpus
        .repo()
        .parent()
        .expect("the repository has a parent")
        .join("outside");
    if link.is_dir() {
        fs::rename(&link, &outside).expect("move the directory out");
    } else {
        fs::create_dir_all(link.parent().expect("a parent")).expect("mkdir the way");
        fs::create_dir_all(&outside).expect("mkdir outside");
    }
    fs::write(outside.join("the-users-own.md"), "# Mine\n").expect("a file of the user's");
    std::os::unix::fs::symlink(&outside, &link).expect("link it");
    (corpus, outside)
}

/// What the guard does at `argv`, read off the parsed command — the production
/// classification, never a second list here.
fn guard_of(argv: &[&str]) -> LinkGuard {
    let argv: Vec<&str> = std::iter::once("jigc")
        .chain(argv.iter().copied())
        .collect();
    <cli::cli::Cli as clap::Parser>::try_parse_from(&argv)
        .unwrap_or_else(|err| panic!("{argv:?} does not parse: {err}"))
        .command
        .link_guard()
}

/// Every leaf over the link at `at`: a guarded one refuses naming it and changes nothing; an
/// exception writes nothing past it. `uninstall` runs on a corpus of its own, because it
/// removes the workbench, and a copy would carry the link to the first corpus's outside.
fn sweep(at: &str) {
    let (shared, shared_outside) = linked(at);
    let mut failures: Vec<String> = Vec::new();
    for (leaf, tail) in MINIMAL_ARGV {
        let own;
        let (corpus, outside) = if *leaf == ["uninstall"] {
            own = linked(at);
            (&own.0, own.1.clone())
        } else {
            (&shared, shared_outside.clone())
        };
        let argv: Vec<&str> = leaf.iter().chain(tail.iter()).copied().collect();
        let (repo_before, outside_before) = (repository(corpus), snapshot(&outside, &[]));
        let out = corpus.jigc(&argv);
        let said = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        let shown = argv.join(" ");
        if snapshot(&outside, &[]) != outside_before {
            failures.push(format!(
                "`jigc {shown}` wrote past the link at `{at}`: what it leads to changed"
            ));
        }
        let named = format!("`{at}` is a symbolic link");
        match guard_of(&argv) {
            LinkGuard::StandsDown(_) => continue,
            LinkGuard::Reports => {
                if !said.contains(CODE) || !said.contains(&named) {
                    failures.push(format!(
                        "`jigc {shown}` is a read form and did not report the link at `{at}`:\n{said}"
                    ));
                }
                continue;
            }
            LinkGuard::Refuses(_) => {}
        }
        if out.status.code() != Some(1) || !said.contains(CODE) || !said.contains(&named) {
            failures.push(format!(
                "`jigc {shown}` over a link at `{at}` exited {:?} and did not refuse \
                 `{CODE}` naming it:\n{said}",
                out.status.code()
            ));
        }
        if repository(corpus) != repo_before {
            failures.push(format!(
                "`jigc {shown}` refused over a link at `{at}` and changed the repository"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The workbench itself a link out of the repository.
#[test]
fn every_leaf_over_a_linked_workbench_refuses_or_writes_nothing_past_it() {
    sweep(".jigc");
}

/// A doctype's home a link out of the repository — the decisions' directory, which the
/// fresh corpus has not made yet: the link stands where jigc would make it.
#[test]
fn every_leaf_over_a_linked_home_refuses_or_writes_nothing_past_it() {
    sweep("docs/decisions");
}

/// **The read forms are `VERB_KINDS`' and nothing else** (the review's S3): every `Read`
/// leaf's minimal argv reports, and a leaf that reports is a `Read` leaf or one of the
/// read forms of `start` and `workflow`, each named here because the form is a flag, not a
/// leaf — so the guard's read set and the total classification cannot disagree.
#[test]
fn the_guard_reports_at_exactly_the_read_forms() {
    use cli::cli::{VERB_KINDS, VerbKind};
    for (leaf, tail) in MINIMAL_ARGV {
        let argv: Vec<&str> = leaf.iter().chain(tail.iter()).copied().collect();
        let kind = VERB_KINDS
            .iter()
            .find(|(known, _)| known == leaf)
            .map(|(_, kind)| *kind)
            .expect("every leaf is classified");
        let guard = guard_of(&argv);
        if kind == VerbKind::Read {
            assert_eq!(guard, LinkGuard::Reports, "`jigc {}` reads", argv.join(" "));
        }
        if guard == LinkGuard::Reports && kind != VerbKind::Read {
            assert!(
                matches!(*leaf, ["start"] | ["workflow"]),
                "`jigc {}` reports, and is no read leaf nor a read form of one",
                argv.join(" ")
            );
        }
    }
    for (argv, read) in [
        (&["start"][..], true),
        (&["start", "--explain"], true),
        (&["start", "a thing"], false),
        (&["start", "--task", "a-task"], false),
        (&["workflow", "single-task", "--preview"], true),
        (&["workflow", "single-task", "--task", "a-task"], false),
    ] {
        assert_eq!(
            guard_of(argv) == LinkGuard::Reports,
            read,
            "`jigc {}`",
            argv.join(" ")
        );
    }
}

/// The table is every leaf, and every exception a leaf with a reason, named once.
#[test]
fn the_sweep_is_every_leaf_and_every_exception_is_a_leaf_with_its_reason() {
    crate::support::leaf_argv::assert_covers_every_leaf_verb();
    for (leaf, _) in MINIMAL_ARGV {
        assert!(
            BEHALF_DOORS.iter().any(|row| row.door == *leaf),
            "`jigc {}` is swept and is no leaf of the classification",
            leaf.join(" ")
        );
    }
    for (at, row) in LINK_GUARD_EXCEPTIONS.iter().enumerate() {
        assert!(
            BEHALF_DOORS.iter().any(|door| door.door == row.door),
            "the exception `jigc {}` is no leaf",
            row.door.join(" ")
        );
        assert!(
            !row.reason.trim().is_empty(),
            "`jigc {}` has no reason",
            row.door.join(" ")
        );
        assert!(
            !LINK_GUARD_EXCEPTIONS[..at]
                .iter()
                .any(|seen| seen.door == row.door),
            "`jigc {}` is an exception twice",
            row.door.join(" ")
        );
    }
}

// ── The review of 2026-10-28: each finding, red first ───────────────────────────────

/// What a run said, stdout and stderr together.
fn said(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// The directory beside the corpus's repository that a planted link leads to — outside
/// the repository — made, with a file of the user's in it.
fn outside_of(corpus: &TrialCorpus, name: &str) -> std::path::PathBuf {
    let outside = corpus.repo().parent().expect("a parent").join(name);
    fs::create_dir_all(&outside).expect("mkdir outside");
    fs::write(outside.join("the-users-own.md"), "# Mine\n").expect("a file of the user's");
    outside
}

/// Assert a run refused under the rule's code, naming `link`, at exit 1.
fn refused_naming(out: &std::process::Output, link: &str, what: &str) {
    let text = said(out);
    assert!(
        out.status.code() == Some(1)
            && text.contains(CODE)
            && text.contains(&format!("`{link}` is a symbolic link")),
        "{what}: exited {:?} and did not refuse `{CODE}` naming `{link}`:\n{text}",
        out.status.code()
    );
}

/// **B1** — `jigc uninstall` over a `.claude` that is a link out of the repository: it
/// edits the settings file and removes the guide below that directory, so it refuses
/// before its first act, and nothing on either side of the link moves.
#[test]
fn uninstall_over_a_linked_assistants_directory_refuses_before_its_first_act() {
    for force in [false, true] {
        let corpus = TrialCorpus::build(State::Fresh);
        let outside = corpus.repo().parent().expect("a parent").join("assistant");
        fs::rename(corpus.repo().join(".claude"), &outside).expect("move .claude out");
        std::os::unix::fs::symlink(&outside, corpus.repo().join(".claude")).expect("link it");
        let (repo_before, outside_before) = (repository(&corpus), snapshot(&outside, &[]));
        let argv: &[&str] = if force {
            &["uninstall", "--force"]
        } else {
            &["uninstall"]
        };
        let out = corpus.jigc(argv);
        refused_naming(&out, ".claude", &argv.join(" "));
        assert_eq!(
            snapshot(&outside, &[]),
            outside_before,
            "{argv:?} wrote past the link"
        );
        assert_eq!(
            repository(&corpus),
            repo_before,
            "{argv:?} refused and changed the repository"
        );
    }
}

/// **S1** — a link standing at a mover's destination is the user's entry: the move is
/// refused and nothing is carried into the workbench, so no later door meets a link jigc
/// put there.
#[test]
fn a_link_at_a_movers_destination_is_refused_and_never_parked() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let outside = outside_of(&corpus, "roadmap-dir");
    fs::create_dir_all(corpus.repo().join("notes")).expect("mkdir notes");
    std::os::unix::fs::symlink(&outside, corpus.repo().join("notes/roadmap.md")).expect("link");
    let repo_before = repository(&corpus);
    let out = corpus.jigc(&["config", "set", "placement-root", "notes"]);
    let text = said(&out);
    assert!(
        out.status.code() == Some(1) && text.contains("store.home-not-regular-file"),
        "the link at the new home is refused as the user's entry:\n{text}"
    );
    assert!(
        !corpus.repo().join(".jigc/displaced/roadmap.md").exists()
            && !corpus.repo().join(".jigc/displaced").exists(),
        "nothing was carried into the workbench"
    );
    assert_eq!(
        repository(&corpus),
        repo_before,
        "the refusal changed nothing"
    );
    let next = corpus.jigc(&["start", "--workflow", "quick-fix", "fix a typo"]);
    assert_eq!(
        next.status.code(),
        Some(0),
        "no door is locked out:\n{}",
        said(&next)
    );
}

/// **S2** — `config set docs-root` asks the way to every home the new value resolves,
/// before the knob is written, even where no doc moves; and the revert of a knob whose
/// homes a link has since come to stand on is not refused for it.
#[test]
fn a_docs_root_whose_homes_are_past_a_link_is_refused_and_its_revert_is_not() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let outside = outside_of(&corpus, "decisions-out");
    fs::create_dir_all(corpus.repo().join("notes")).expect("mkdir notes");
    std::os::unix::fs::symlink(&outside, corpus.repo().join("notes/decisions")).expect("link");
    let repo_before = repository(&corpus);
    let out = corpus.jigc(&["config", "set", "docs-root", "notes"]);
    refused_naming(&out, "notes/decisions", "config set docs-root notes");
    assert_eq!(
        repository(&corpus),
        repo_before,
        "the refusal changed nothing"
    );

    fs::remove_file(corpus.repo().join("notes/decisions")).expect("unlink");
    let set = corpus.jigc(&["config", "set", "docs-root", "notes"]);
    assert_eq!(
        set.status.code(),
        Some(0),
        "with no link it lands:\n{}",
        said(&set)
    );
    std::os::unix::fs::symlink(&outside, corpus.repo().join("notes/decisions")).expect("link");
    let back = corpus.jigc(&["config", "set", "docs-root", "docs"]);
    assert_eq!(
        back.status.code(),
        Some(0),
        "the revert writes nothing past the link and is not refused:\n{}",
        said(&back)
    );
}

/// **S3** — a read form reports the link at exit 0 and never refuses; orientation, the
/// session hook's command, reports it on stdout.
#[test]
fn every_read_form_reports_the_link_at_exit_0() {
    let corpus = TrialCorpus::build(State::Fresh);
    let minted = corpus.jigc(&["start", "--workflow", "quick-fix", "fix a typo"]);
    assert_eq!(minted.status.code(), Some(0), "{}", said(&minted));
    // An empty directory outside: what the link leads to holds nothing a reading sweep
    // would grade, so each read form's own exit is its answer about the store.
    let outside = corpus
        .repo()
        .parent()
        .expect("a parent")
        .join("decisions-out");
    fs::create_dir_all(&outside).expect("mkdir outside");
    fs::create_dir_all(corpus.repo().join("docs")).expect("mkdir docs");
    std::os::unix::fs::symlink(&outside, corpus.repo().join("docs/decisions")).expect("link");
    let outside_before = snapshot(&outside, &[]);
    for argv in [
        &["start"][..],
        &["start", "--explain"],
        &["upgrade"],
        &["workflow", "single-task", "--preview"],
        &["task", "validate", "fix-a-typo"],
        &["validate"],
        &["doc", "list"],
    ] {
        let out = corpus.jigc(argv);
        let text = said(&out);
        assert!(
            out.status.code() != Some(1)
                && text.contains(CODE)
                && text.contains("`docs/decisions` is a symbolic link"),
            "`jigc {}` exited {:?}: a read form reports the link and does not refuse:\n{text}",
            argv.join(" "),
            out.status.code()
        );
        assert_eq!(
            snapshot(&outside, &[]),
            outside_before,
            "{argv:?} wrote past the link"
        );
    }
    let orient = corpus.jigc(&["start"]);
    assert_eq!(orient.status.code(), Some(0));
    assert!(
        String::from_utf8_lossy(&orient.stdout).contains(CODE),
        "orientation carries the link on stdout, where a session reads it"
    );
}

/// **S4** — a link in a checkout of the user's own outside the main one is named relative
/// to that checkout: no host path is printed.
#[test]
fn a_link_in_a_users_own_worktree_is_named_relative_to_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    let parent = corpus.repo().parent().expect("a parent").to_path_buf();
    let wt = parent.join("wt2");
    corpus.git(&[
        "worktree",
        "add",
        "-q",
        wt.to_str().expect("utf-8"),
        "-b",
        "side",
    ]);
    let outside = outside_of(&corpus, "decisions-out");
    fs::create_dir_all(wt.join("docs")).expect("mkdir docs");
    std::os::unix::fs::symlink(&outside, wt.join("docs/decisions")).expect("link");
    let out = corpus.jigc_stdin_from(&wt, &["start", "--workflow", "quick-fix", "fix a typo"], "");
    refused_naming(
        &out,
        "docs/decisions",
        "a quick-fix from the user's worktree",
    );
    let text = said(&out);
    let host = parent.to_string_lossy().into_owned();
    let canonical = parent
        .canonicalize()
        .expect("canonical")
        .to_string_lossy()
        .into_owned();
    assert!(
        !text.contains(&host) && !text.contains(&canonical),
        "a host path is printed:\n{text}"
    );
}

/// **S5** — one state, one code: `setup` names a link on the way to an install member
/// under the rule's code, and a root knob whose value runs through a link refuses under it
/// too, inside the repository or out of it.
#[test]
fn a_link_on_the_way_has_one_code_at_setup_and_at_a_root_knob() {
    let bare = TrialCorpus::build_never_adopted();
    let outside = outside_of(&bare, "assistant");
    std::os::unix::fs::symlink(&outside, bare.repo().join(".claude")).expect("link");
    let before = snapshot(&outside, &[]);
    let out = bare.jigc(&["setup"]);
    refused_naming(&out, ".claude", "setup over a linked `.claude`");
    assert_eq!(snapshot(&outside, &[]), before, "setup wrote past the link");

    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("handbook")).expect("mkdir");
    std::os::unix::fs::symlink("handbook", corpus.repo().join("linked")).expect("link in");
    let out = corpus.jigc(&["config", "set", "docs-root", "linked/sub"]);
    refused_naming(
        &out,
        "linked",
        "a root through a link inside the repository",
    );
    let outside = outside_of(&corpus, "elsewhere");
    std::os::unix::fs::symlink(&outside, corpus.repo().join("out")).expect("link out");
    let out = corpus.jigc(&["config", "set", "docs-root", "out"]);
    refused_naming(&out, "out", "a root through a link out of the repository");
}

/// **S6** — a link whose target is a directory inside the repository: the reason is true
/// of it (git records the link, never a path past it), and the route moves the target into
/// the link's place rather than copying it.
#[test]
fn an_in_repository_links_reason_and_route_are_true_of_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("kept")).expect("mkdir");
    fs::write(corpus.repo().join("kept/keep.md"), "# Keep\n").expect("write");
    fs::create_dir_all(corpus.repo().join("docs")).expect("mkdir docs");
    std::os::unix::fs::symlink("../kept", corpus.repo().join("docs/decisions")).expect("link");
    let out = corpus.jigc(&["start", "--workflow", "quick-fix", "fix a typo"]);
    refused_naming(&out, "docs/decisions", "an in-repository link");
    let text = said(&out);
    assert!(
        !text.contains("where no commit of this repository holds it"),
        "{text}"
    );
    assert!(
        text.contains("move the directory `kept` into its place"),
        "the route moves the in-repository target into the link's place:\n{text}"
    );
}

/// **S7** — a link where the workbench keeps a directory counts whatever it leads to, a
/// link to nothing included.
#[test]
fn a_dangling_link_where_the_workbench_keeps_a_directory_is_refused() {
    let corpus = TrialCorpus::build(State::Fresh);
    let tasks = corpus.repo().join(".jigc/tasks");
    if tasks.exists() {
        fs::remove_dir_all(&tasks).expect("clear tasks");
    }
    std::os::unix::fs::symlink("nowhere-yet", &tasks).expect("link to nothing");
    let out = corpus.jigc(&["start", "--workflow", "quick-fix", "fix a typo"]);
    refused_naming(
        &out,
        ".jigc/tasks",
        "a dangling link at the tasks directory",
    );
}

/// **S8** — `setup` over a linked home refuses as the next command would, writing nothing.
#[test]
fn setup_over_a_linked_home_refuses_the_same_way() {
    let bare = TrialCorpus::build_never_adopted();
    let outside = outside_of(&bare, "decisions-out");
    fs::create_dir_all(bare.repo().join("docs")).expect("mkdir docs");
    std::os::unix::fs::symlink(&outside, bare.repo().join("docs/decisions")).expect("link");
    let before = repository(&bare);
    let out = bare.jigc(&["setup"]);
    refused_naming(&out, "docs/decisions", "setup over a linked home");
    assert_eq!(
        repository(&bare),
        before,
        "setup refused and changed the repository"
    );
}

/// **N1** — every link of the store is named in one refusal: the workbench's and a
/// home's together, not one per run.
#[test]
fn every_link_is_named_in_one_refusal() {
    let corpus = TrialCorpus::build(State::Fresh);
    let a = outside_of(&corpus, "index-out");
    let index = corpus.repo().join(".jigc/index");
    if index.exists() {
        fs::remove_dir_all(&index).expect("clear index");
    }
    std::os::unix::fs::symlink(&a, &index).expect("link");
    let b = outside_of(&corpus, "decisions-out");
    fs::create_dir_all(corpus.repo().join("docs")).expect("mkdir docs");
    std::os::unix::fs::symlink(&b, corpus.repo().join("docs/decisions")).expect("link");
    let out = corpus.jigc(&["start", "--workflow", "quick-fix", "fix a typo"]);
    refused_naming(&out, ".jigc/index", "two links");
    refused_naming(&out, "docs/decisions", "two links");
}
