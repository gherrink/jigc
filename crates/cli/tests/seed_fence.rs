//! **The seed fence: the committed seed is adoptable as it stands** (M55 Increment 9, T7;
//! [findings-channel.md](../../../design/findings-channel.md) → 7; `DECISIONS.md` →
//! *2026-10-03 — M55 Increment 9 planning*, P6).
//!
//! `completions/artifacts/M55/seed/` holds the `jigc-feedback` and `inconsistency` docs the
//! seed's one fan-out filed, laid out as the doctypes' own homes, so M56 places the tree under
//! its `docs-root` unchanged. This suite rehearses that act with the real binary:
//!
//! - **(a)** the committed seed, copied into a fresh corpus under `docs/`, then `jigc ingest`,
//!   then `jigc validate --format json`, reports **zero findings**, and `jigc doc list <ty>
//!   --format json` lists exactly the seed files' ids, all `managed`, so a file the store
//!   did not take in cannot hide;
//! - **(b)** the standing control: the same with one seed file's `kind:` line deleted
//!   carries `schema-conformance.required-field-present` at that doc's `#meta/kind`, so (a)
//!   is green because the seed conforms, not because the probe read nothing.
//!
//! Both verbs exit 0 over a non-conformant file (driven at planning), so the fence reads the
//! JSON `findings` array and never an exit code for its verdict.
//!
//! (a)'s predicate is [`assert_adoptable`], over any directory laid out as the doctypes' homes;
//! flow 58 (`flow58_several_reporters`) runs it over the docs its own join landed.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus};

/// The committed seed, repo-relative.
const SEED: &str = "completions/artifacts/M55/seed";

/// Each doctype and its home, the directory name under both `seed/` and the corpus's
/// `docs-root` (`docs` in every trial state).
const HOMES: [(&str, &str); 2] = [
    ("jigc-feedback", "jigc-feedback"),
    ("inconsistency", "inconsistencies"),
];

fn seed_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(SEED)
}

/// One doc under a home: its doctype, its home and its slug.
struct SeedFile {
    doctype: &'static str,
    home: &'static str,
    slug: String,
}

impl SeedFile {
    fn id(&self) -> String {
        format!("{}:{}", self.doctype, self.slug)
    }

    fn placed(&self) -> String {
        format!("docs/{}/{}.md", self.home, self.slug)
    }
}

/// Every doc laid out under `root` as the doctypes' homes, sorted by home then slug. Each home
/// must hold at least one. The committed seed is one such `root`; a corpus's `docs-root` after
/// a join is another (flow 58).
fn docs_under(root: &Path) -> Vec<SeedFile> {
    let mut files = Vec::new();
    for (doctype, home) in HOMES {
        let dir = root.join(home);
        let mut slugs: Vec<String> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("`{}` is unreadable: {e}", dir.display()))
            .map(|entry| {
                let name = entry.expect("a readable entry").file_name();
                let name = name.into_string().expect("a utf-8 file name");
                name.strip_suffix(".md")
                    .unwrap_or_else(|| panic!("`{}/{name}` is not a `.md` doc", dir.display()))
                    .to_owned()
            })
            .collect();
        slugs.sort();
        assert!(!slugs.is_empty(), "`{}` holds no doc", dir.display());
        files.extend(slugs.into_iter().map(|slug| SeedFile {
            doctype,
            home,
            slug,
        }));
    }
    files
}

/// A fresh corpus with every file of `files` (laid out under `root`) copied under
/// `docs/<home>/`, `edit` applied to each file's bytes on the way, then `jigc ingest` run over
/// it. Returns the corpus.
fn adopt(
    root: &Path,
    files: &[SeedFile],
    edit: impl Fn(&SeedFile, String) -> String,
) -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    for file in files {
        let bytes = fs::read_to_string(root.join(file.home).join(format!("{}.md", file.slug)))
            .expect("a doc to adopt is readable");
        let to = corpus.repo().join(file.placed());
        fs::create_dir_all(to.parent().expect("a placed file has a parent")).expect("the home");
        fs::write(&to, edit(file, bytes)).expect("place the seed file");
    }
    let _: Value = stdout_json(
        &corpus.jigc(&["ingest", "--format", "json"]),
        &[0],
        "jigc ingest over the adopted docs",
    );
    corpus
}

/// `jigc validate --format json`'s `findings` array.
fn findings(corpus: &TrialCorpus) -> Vec<Value> {
    let report: Value = stdout_json(
        &corpus.jigc(&["validate", "--format", "json"]),
        &[0],
        "jigc validate over the adopted docs",
    );
    report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the validate report carries a `findings` array:\n{report:#}"))
        .clone()
}

/// (a) **The committed seed ingests and validates with zero findings, and the store lists
/// exactly its docs.**
#[test]
fn the_committed_seed_ingests_and_validates_with_zero_findings() {
    assert_adoptable(&seed_dir());
}

/// **The seed fence's predicate**, over the docs laid out under `root` as the doctypes' homes:
/// copied into a fresh corpus under its `docs-root`, `jigc ingest` then `jigc validate --format
/// json` carries zero findings, and `jigc doc list <ty> --format json` lists exactly those
/// docs' ids, all `managed`.
pub(crate) fn assert_adoptable(root: &Path) {
    let files = docs_under(root);
    let corpus = adopt(root, &files, |_, bytes| bytes);

    let findings = findings(&corpus);
    assert!(
        findings.is_empty(),
        "the docs adopted from `{}` carry {} finding(s):\n{:#}",
        root.display(),
        findings.len(),
        Value::Array(findings),
    );

    for (doctype, _) in HOMES {
        let listing: Value = stdout_json(
            &corpus.jigc(&["doc", "list", doctype, "--format", "json"]),
            &[0],
            "jigc doc list over the seed",
        );
        let listed: BTreeSet<(String, String)> = listing["docs"]
            .as_array()
            .unwrap_or_else(|| panic!("`doc list {doctype}` carries a `docs` array"))
            .iter()
            .map(|doc| {
                (
                    doc["id"]
                        .as_str()
                        .expect("a listed doc has an id")
                        .to_owned(),
                    doc["state"]
                        .as_str()
                        .expect("a listed doc has a state")
                        .to_owned(),
                )
            })
            .collect();
        let seeded: BTreeSet<(String, String)> = files
            .iter()
            .filter(|f| f.doctype == doctype)
            .map(|f| (f.id(), "managed".to_owned()))
            .collect();
        assert_eq!(
            listed, seeded,
            "`doc list {doctype}` must list exactly the adopted `{doctype}` docs, each managed"
        );
    }
}

/// (b) **The standing control.** One seed doc with its `kind:` line deleted is reported at
/// its `#meta/kind` as `schema-conformance.required-field-present`.
#[test]
fn a_seed_doc_without_its_kind_line_carries_required_field_present() {
    let files = docs_under(&seed_dir());
    let broken = &files[0];
    let corpus = adopt(&seed_dir(), &files, |file, bytes| {
        if file.id() != broken.id() {
            return bytes;
        }
        let without: String = bytes
            .split_inclusive('\n')
            .filter(|line| !line.starts_with("kind: "))
            .collect();
        assert_ne!(
            without,
            bytes,
            "`{}` carries a `kind:` line",
            broken.placed()
        );
        without
    });

    let findings = findings(&corpus);
    let keys: Vec<&Value> = findings.iter().map(|finding| &finding["key"]).collect();
    let want = json!({
        "code": "schema-conformance.required-field-present",
        "target": format!("{}#meta/kind", broken.id()),
    });
    assert!(
        keys.contains(&&want),
        "deleting `{}`'s `kind:` line must carry {want}; the keys were {keys:#?}",
        broken.placed(),
    );
}
