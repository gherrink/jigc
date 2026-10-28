//! **No door writes past a link on the way** (2026-10-28, the human's ruling B;
//! `design/storage.md` → *No write through a link on the way*) — every leaf verb, driven
//! through the real binary over a link out of the repository at the workbench and at a
//! doctype's home.
//!
//! The byte table holds the doors the rule was found at, cell by cell, each at the place
//! its row aims a link at (`r1-a-write-follows-a-link-at-a-directory-on-the-way-out-of-the-
//! repository`). This suite holds the guard's **reach**: the dispatch guard stands in
//! front of every leaf but the ones `cli::cli::LINK_GUARD_EXCEPTIONS` names, so every leaf
//! is driven — each from its one minimal argv ([`MINIMAL_ARGV`], which the guard answers
//! before any id or path is resolved) — and a leaf added tomorrow is driven the day it is
//! added, because the argv table is fenced to the clap tree.
//!
//! What one cell claims:
//!
//! - **A guarded leaf refuses, and changed nothing** — exit 1, the code
//!   `store.link-on-the-way`, the link named by its repo-relative path, and neither the
//!   repository (its tree, its index, its `HEAD`) nor what the link leads to moved a byte.
//! - **An exception writes nothing past the link** — whatever it answers, what the link
//!   leads to is byte for byte as it was. The exceptions are the reading leaves, `setup`
//!   (which asks its own members' way) and `uninstall` (which removes a link as the entry it
//!   is), so this is the claim their reasons make, driven.

use crate::support::leaf_argv::{FIXTURE_FILES, MINIMAL_ARGV};
use crate::support::trial_corpus::{State, TrialCorpus};
use cli::cli::{BEHALF_DOORS, LINK_GUARD_EXCEPTIONS};
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

/// Whether the guard stands in front of `leaf`.
fn guarded(leaf: &[&str]) -> bool {
    !LINK_GUARD_EXCEPTIONS.iter().any(|row| row.door == leaf)
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
        if !guarded(leaf) {
            continue;
        }
        let named = format!("`{at}` is a symbolic link");
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
