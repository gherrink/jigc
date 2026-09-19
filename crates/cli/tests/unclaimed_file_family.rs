//! The **unclaimed-file family**: every production site that answers *what is this committed
//! file, and why is jigc not treating it as one of its own?* (M52 Increment 8 / T4;
//! settle-record D10.2; gate-record row 10; baseline-freeze §1.4 / §2.4).
//!
//! # The defect the sweep was written for
//!
//! `cli::orphan::orphaned_instance_finding` computed one thing — *this path sits inside
//! jigc's declared homes, carries a `schema-version:` stamp, and is in no resolved doctype's
//! committed census* — and printed a second: *"no schema in the composed set says what this
//! file is"*. That tail is **false wherever the doctype is composed and the file is merely
//! not at its home**, which is the cell the wave's baseline drove: move `adr`'s `location:`
//! and leave its instance behind, and at the same commit `jigc validate` said no schema knows
//! what the file is while `jigc ingest` raised `ingest.wrong-location` naming the `adr`
//! schema and the home to move it to. Two doors, two stories, one file.
//!
//! # Why the fix is a family sweep and not a sentence
//!
//! *What is this file?* is answered by several producers across four doors, and at HEAD they
//! had no code-side home — `render::GATES_NOWHERE` lists several of their codes, but it is a
//! **gate-label** list (what a code does to an exit), not a family registry, so it cannot be
//! the sweep's subject. A fixed sentence with no enumeration behind it is the incomplete-fix
//! shape this wave exists to correct, so:
//!
//! - **the subject is read off the source tree**, never written down — every production
//!   occurrence of one of the family's codes under `crates/cli/src` and `crates/engine/src`,
//!   spelled as a literal, through the crate constant that carries it, *or* through the one
//!   shared seam that mints a family finding without naming its code
//!   ([`engine::validate::AdoptionInputs::unadopted`]), comments blanked and `#[cfg(test)]`
//!   bodies excluded ([`crate::support::rust_source`], the reader the sibling source fences
//!   share);
//! - **each occurrence must carry a stated row** naming the corpus state it fires in and what
//!   this sweep did to it, so a new producer cannot join the family in silence;
//! - **two cells are driven, not argued** — the relocated-home cell, where the schema *is*
//!   composed, and the departed-doctype cell, where it genuinely is not — each read off the
//!   surfaces the binary printed at one commit.
//!
//! # What the scan found that the wave's baseline did not
//!
//! The baseline counted the family by grepping its **code literals**, which reaches a producer
//! only when it spells its own code. Two do not: `migrate_corpus`'s adoption arm and the
//! **task-scope reconciler** both mint `schema-conformance.unadopted-instance` through
//! `AdoptionInputs::unadopted`, naming no code at either site — and the line the baseline cited
//! for the `migrate_corpus` producer (`migrate_corpus.rs:2425`) is a `#[cfg(test)]` report
//! fixture, not a producer at all. So the scan's token set carries the seam as well as the
//! codes, and the family is **eight** production sites rather than the seven the record named.
//!
//! # The bound this task does not close, stated
//!
//! In the **departed-doctype** cell `jigc validate` refuses a green while `jigc ingest` reports
//! the file *unmanaged — fine to stay plain*. Those two are not in contradiction (nothing can
//! adopt a file whose type no schema defines, and the sweep still refuses to call a stamped
//! file it never read *clean*), and closing the gap would mean teaching `ingest` about stamped
//! orphans — behaviour, not wording. What this task owes there is that the operator is not
//! stranded, so [`the_departed_doctype_cell_keeps_a_true_message_and_a_usable_route`] drives
//! the two exits that *do* apply in that cell.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::{frozen_pack, rust_source};

// ---------------------------------------------------------------------------------------------
// The registry.
// ---------------------------------------------------------------------------------------------

/// What this sweep did to a producer of the family.
#[derive(Clone, Copy)]
enum Verdict {
    /// The sweep changed what this producer says. The reason names the false clause.
    Corrected(&'static str),
    /// Read at this HEAD and left alone — the reason says what makes its wording true of the
    /// state it fires in.
    AlreadyTrue(&'static str),
}

/// What a site naming one of the family's codes *is*.
#[derive(Clone, Copy)]
enum Role {
    /// A **producer**: this site mints a finding of the family, or calls the seam that does.
    Produces(Verdict),
    /// Not a producer — the code's own `const`, a matcher, a gate-label row, a fence witness,
    /// a cross-reference in a neighbouring producer's text.
    NamesIt(&'static str),
}

/// One production site group, keyed `(file, enclosing fn)` with the number of occurrences in
/// it.
///
/// **The key is the enclosing `fn` as [`rust_source::enclosing_fn`] reads it** — the last `fn`
/// declared textually before the site — so an occurrence inside a `const` table reports the
/// function above that table. That is a key, not a claim; each row's own prose names what the
/// site actually is, and the fence prints line numbers when a group moves.
struct FamilySite {
    file: &'static str,
    func: &'static str,
    sites: usize,
    /// The corpus state in which this site's text reaches a surface.
    state: &'static str,
    role: Role,
}

/// Every production site naming a family code, with the state it fires in and its disposition.
const UNCLAIMED_FILE_FAMILY: &[FamilySite] = &[
    // ----- the producers -----
    FamilySite {
        file: "crates/cli/src/orphan.rs",
        func: "orphaned_instance_finding",
        sites: 1,
        state: "a committed `.md` inside jigc's declared homes carrying a `schema-version:` \
                stamp that no resolved doctype's committed census claims — the doctype may \
                have left the composition, or may be composed and homed elsewhere",
        role: Role::Produces(Verdict::Corrected(
            "its tail asserted *no schema in the composed set says what this file is*, which \
             is false in the relocated-home cell the baseline drove; it now states the claim \
             the producer computed — *no resolved doctype claims this path* — and its route \
             leads with the door that tells the two causes apart",
        )),
    },
    FamilySite {
        file: "crates/cli/src/ingest.rs",
        func: "wrong_location_finding",
        sites: 1,
        state: "a committed file that parses clean against a resolved schema while sitting \
                outside that schema's home — the relocated-home cell, at the adoption door",
        role: Role::Produces(Verdict::AlreadyTrue(
            "it names the conformant doctype and the home to move the file to; it was the \
             *correct* half of the contradiction, which is why the orphan message moved to \
             meet it rather than the other way round",
        )),
    },
    FamilySite {
        file: "crates/cli/src/cli.rs",
        func: "validate_store_in_repo",
        sites: 2,
        state: "a committed `.md` the strand walk can name a doctype for — stranded at that \
                doctype's *prior* home after a `docs-root` / `placement-root` re-point — \
                split on whether the file-state record knows the path (`file-state.\
                orphaned-doc`) or never did (`file-state.unregistered-doc`)",
        role: Role::Produces(Verdict::AlreadyTrue(
            "both arms key on the matched doctype's *current* home and say a doctype resolves \
             for the file; neither claims anything about the composed set",
        )),
    },
    FamilySite {
        file: "crates/engine/src/validate.rs",
        func: "unadopted_instance",
        sites: 1,
        state: "a committed file at a resolved doctype's home that jigc never adopted — the \
                one mint of the adoption advisory, reached from the store sweep and from the \
                `AdoptionInputs::unadopted` seam below",
        role: Role::Produces(Verdict::AlreadyTrue(
            "M46 made its tail name the cause the discriminator found (foreign bytes vs an \
             unaddressable identity); it claims nothing about whether a schema exists, only \
             that this file was never adopted",
        )),
    },
    FamilySite {
        file: "crates/engine/src/validate.rs",
        func: "unversioned_doctype",
        sites: 1,
        state: "a committed instance of a doctype the composed set **defines** while its \
                owning pack ships no `schema-manifest.yaml` entry for it",
        role: Role::Produces(Verdict::AlreadyTrue(
            "the resolved half of M51's partition — it leads with the doctype precisely \
             because the doctype *is* composed, so it was never able to make the orphan's \
             claim",
        )),
    },
    FamilySite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "migrate_committed_corpus",
        sites: 1,
        state: "a never-adopted foreign file at a managed home met by the corpus walk — \
                excluded from the fold and reported on the declared `unadopted` key",
        role: Role::Produces(Verdict::AlreadyTrue(
            "it mints nothing of its own: M46 routed it through the one seam so the corpus \
             door and the store door cannot tell two stories about one file. The baseline's \
             cited line for this producer is a `#[cfg(test)]` report fixture; the producer is \
             this call, and it names no code, which is why the scan carries the seam token",
        )),
    },
    FamilySite {
        file: "crates/engine/src/file_state.rs",
        func: "reconcile_committed",
        sites: 1,
        state: "the **task-scope** reconciler meeting an unknown committed file at a managed \
                home that fails the conformance gate and reads foreign",
        role: Role::Produces(Verdict::AlreadyTrue(
            "the eighth site, in no row of the wave's record: it reaches the family through \
             the same seam and therefore inherits the seam's wording verbatim, which is the \
             property that made it safe to leave alone",
        )),
    },
    // ----- the sites that name a code without producing one -----
    FamilySite {
        file: "crates/cli/src/orphan.rs",
        func: "contains",
        sites: 2,
        state: "not a surface — the code's own `const` declaration, which the scan matches \
                twice: once as the identifier and once as the literal it binds",
        role: Role::NamesIt("the family's one spelling of `schema-conformance.orphaned-instance`"),
    },
    FamilySite {
        file: "crates/cli/src/render.rs",
        func: "validation_store",
        sites: 3,
        state: "not a surface — three `STORE_EXIT_FLIPS` cells: the orphaned instance's \
                matcher, and the squatter's matcher plus the code inside its hand-built \
                witness. The orphan member's witness names no code because it *is* the \
                producer, which is that member's own stated claim",
        role: Role::NamesIt(
            "membership in the exit-flip table, which decides an exit rather than printing a \
             claim about a file",
        ),
    },
    FamilySite {
        file: "crates/cli/src/render.rs",
        func: "validation_store_exit_flips",
        sites: 5,
        state: "not a surface — the `GATES_NOWHERE` gate-label list",
        role: Role::NamesIt(
            "what a code does to a *gate*, which is why it cannot serve as this family's \
             registry: it lists codes by their effect on an exit, not by the question they \
             answer about a file",
        ),
    },
    FamilySite {
        file: "crates/cli/src/render.rs",
        func: "unmigrated_corpus_trailer",
        sites: 2,
        state: "not a surface — `UNADOPTED_INSTANCE_CODE`'s own `const` declaration, which \
                sits after this function and which the scan matches twice (the identifier and \
                the literal it binds)",
        role: Role::NamesIt(
            "the family's one spelling of `schema-conformance.unadopted-instance`; the \
             trailer this key falls under prints no claim about an individual file",
        ),
    },
    FamilySite {
        file: "crates/engine/src/validate.rs",
        func: "ahead_route",
        sites: 2,
        state: "not a surface — `UNVERSIONED_DOCTYPE_CODE`'s own `const` declaration, which \
                sits after this function and which the scan matches twice (the identifier and \
                the literal it binds)",
        role: Role::NamesIt(
            "the family's one spelling of `schema-conformance.unversioned-doctype`",
        ),
    },
];

// ---------------------------------------------------------------------------------------------
// The source reader.
// ---------------------------------------------------------------------------------------------

/// The cargo workspace root — the registry's `file` column is workspace-relative, because the
/// family reaches `crates/engine/src` too.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The tokens a family finding is reached by in source: the six codes, the three crate
/// constants that carry one, and the **seam call** — because a producer that mints through
/// [`engine::validate::AdoptionInputs::unadopted`] names no code and would otherwise join the
/// family in silence, which is exactly what two of them did.
const TOKENS: &[&str] = &[
    "schema-conformance.unadopted-instance",
    "schema-conformance.orphaned-instance",
    "schema-conformance.unversioned-doctype",
    "ingest.wrong-location",
    "file-state.orphaned-doc",
    "file-state.unregistered-doc",
    "ORPHANED_INSTANCE_CODE",
    "UNADOPTED_INSTANCE_CODE",
    "UNVERSIONED_DOCTYPE_CODE",
    ".unadopted(",
];

/// One scanned occurrence.
struct Occurrence {
    file: String,
    func: String,
    line: usize,
    token: &'static str,
}

/// Every production occurrence of a [`TOKENS`] member under the two source trees, comments
/// blanked and `#[cfg(test)]` bodies excluded ([`rust_source`]).
fn occurrences() -> Vec<Occurrence> {
    let root = workspace_root();
    let mut out = Vec::new();
    for tree in ["crates/cli/src", "crates/engine/src"] {
        for path in rust_source::rust_files(&root.join(tree)) {
            let body = fs::read_to_string(&path).expect("read a source file");
            let code = rust_source::code_and_strings(&body);
            let regions = rust_source::cfg_test_regions(&code);
            let rel = path
                .strip_prefix(&root)
                .expect("a workspace-relative path")
                .to_string_lossy()
                .replace('\\', "/");
            for token in TOKENS {
                for (at, _) in code.match_indices(token) {
                    if rust_source::is_test_domain(&path, &regions, at) {
                        continue;
                    }
                    out.push(Occurrence {
                        file: rel.clone(),
                        func: rust_source::enclosing_fn(&code, at)
                            .unwrap_or("<top level>")
                            .to_string(),
                        line: body[..at].lines().count(),
                        token,
                    });
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// The fences.
// ---------------------------------------------------------------------------------------------

/// **Every production site that reaches the family carries a stated row.**
///
/// Read in both directions: a site the registry does not name is an unstated member of the
/// family, and a row the source no longer has is a claim about code that moved. The counts are
/// checked too, so a producer added inside an already-listed function cannot slip in under a
/// row written for its neighbour.
#[test]
fn every_production_site_reaching_the_family_has_a_stated_row() {
    let scanned = occurrences();

    let mut unstated: Vec<String> = Vec::new();
    for occ in &scanned {
        if !UNCLAIMED_FILE_FAMILY
            .iter()
            .any(|row| row.file == occ.file && row.func == occ.func)
        {
            unstated.push(format!(
                "  {}:{} in `{}` (`{}`)",
                occ.file, occ.line, occ.func, occ.token
            ));
        }
    }
    unstated.sort();
    unstated.dedup();
    assert!(
        unstated.is_empty(),
        "these production sites reach the unclaimed-file family and carry no row in \
         `UNCLAIMED_FILE_FAMILY` — state the corpus state each fires in and what was done \
         about it:\n{}",
        unstated.join("\n"),
    );

    let mut wrong: Vec<String> = Vec::new();
    for row in UNCLAIMED_FILE_FAMILY {
        let found = scanned
            .iter()
            .filter(|occ| occ.file == row.file && occ.func == row.func)
            .count();
        if found != row.sites {
            wrong.push(format!(
                "  {} in `{}`: the row says {} site(s), the source has {}",
                row.file, row.func, row.sites, found
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the registry's site counts must match the source:\n{}",
        wrong.join("\n"),
    );
}

/// **Every row says what it fires in and what was done about it** — the row is the sweep's
/// unit of work, so an empty column is an unswept member wearing a row.
#[test]
fn every_row_states_its_firing_state_and_its_disposition() {
    for row in UNCLAIMED_FILE_FAMILY {
        assert!(
            row.state.len() > 30,
            "`{}` in `{}` must name the corpus state it fires in",
            row.file,
            row.func,
        );
        let reason = match row.role {
            Role::Produces(Verdict::Corrected(why)) => why,
            Role::Produces(Verdict::AlreadyTrue(why)) => why,
            Role::NamesIt(why) => why,
        };
        assert!(
            reason.len() > 30,
            "`{}` in `{}` must say why its disposition is what it is",
            row.file,
            row.func,
        );
    }
}

/// **The sweep touched exactly one producer, and it is the one the record names.** A sweep
/// whose every member came back *already true* is a sweep that never looked; a sweep that
/// rewrote more than the defect is scope the task does not carry. Both readings are wrong
/// here, and the registry is where that is checkable.
#[test]
fn exactly_one_producer_was_corrected_and_it_is_the_orphan_message() {
    let corrected: Vec<&FamilySite> = UNCLAIMED_FILE_FAMILY
        .iter()
        .filter(|row| matches!(row.role, Role::Produces(Verdict::Corrected(_))))
        .collect();
    assert_eq!(
        corrected.len(),
        1,
        "the defect is one producer's tail clause; corrected rows: {:?}",
        corrected
            .iter()
            .map(|row| format!("{} in `{}`", row.file, row.func))
            .collect::<Vec<_>>(),
    );
    assert_eq!(corrected[0].func, "orphaned_instance_finding");

    let producers = UNCLAIMED_FILE_FAMILY
        .iter()
        .filter(|row| matches!(row.role, Role::Produces(_)))
        .count();
    assert!(
        producers >= 7,
        "the record names seven producers and the scan found an eighth; a registry that \
         dropped below seven has lost one, got {producers}",
    );
}

// ---------------------------------------------------------------------------------------------
// The behavioural half: the fixture.
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-unclaimed-family-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// One driven corpus: an on-disk dev-pack copy (so the methodology pack is **not** composed —
/// `JIGC_PACK_DIR` supersedes the marker), a `$HOME`, and a repo set up against it.
struct Corpus {
    pack: TempDir,
    home: TempDir,
    repo: TempDir,
}

impl Corpus {
    /// A corpus whose pack is the shipped dev pack with `reshape` applied to doctype `ty`'s
    /// schema and that doctype's manifest hash re-pinned — the legal way to ship a shape (or
    /// **home**) change, [`frozen_pack`].
    fn reshaped(tag: &str, ty: &str, reshape: impl FnOnce(&str) -> String) -> Self {
        let pack = TempDir::new(&format!("pack-{tag}"));
        frozen_pack::reshaped_dev_pack(pack.path(), ty, reshape);
        Self::over(tag, pack)
    }

    /// A corpus whose pack is the shipped dev pack, byte-identical.
    fn stock(tag: &str) -> Self {
        let pack = TempDir::new(&format!("pack-{tag}"));
        frozen_pack::copy_dev_pack(pack.path());
        Self::over(tag, pack)
    }

    fn over(tag: &str, pack: TempDir) -> Self {
        let home = TempDir::new(&format!("home-{tag}"));
        let repo = TempDir::new(&format!("repo-{tag}"));

        let root = repo.path();
        git(root, &["init", "-q"]);
        git(root, &["config", "user.email", "test@example.com"]);
        git(root, &["config", "user.name", "Test"]);
        fs::write(root.join("README.md"), "hello\n").expect("write file");
        git(root, &["add", "."]);
        git(root, &["commit", "-q", "-m", "initial"]);

        let corpus = Corpus { pack, home, repo };
        let out = corpus.jigc(&["setup"]);
        assert!(
            out.status.success(),
            "`jigc setup` must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        corpus
    }

    fn jigc(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", self.pack.path())
            .output()
            .expect("run the jigc binary")
    }

    /// `jigc <args>`'s combined streams and whether it exited 0.
    fn run(&self, args: &[&str]) -> (String, bool) {
        let out = self.jigc(args);
        (
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            ),
            out.status.success(),
        )
    }

    /// `jigc <args> --format json`'s parsed stdout.
    fn json(&self, args: &[&str]) -> serde_json::Value {
        let mut argv = args.to_vec();
        argv.extend_from_slice(&["--format", "json"]);
        let out = self.jigc(&argv);
        serde_json::from_slice(&out.stdout).unwrap_or_else(|err| {
            panic!(
                "`jigc {}` must emit JSON ({err}); stdout:\n{}\nstderr:\n{}",
                argv.join(" "),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            )
        })
    }

    /// Commit `body` at the repo-relative `path`.
    fn commit_doc(&self, path: &str, body: &str) {
        let full = self.repo.path().join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).expect("mk the doc's home");
        }
        fs::write(&full, body).expect("write the committed doc");
        git(self.repo.path(), &["add", "-A"]);
        git(self.repo.path(), &["commit", "-q", "-m", "seed the corpus"]);
    }
}

/// Replace `needle` with `replacement` exactly once, asserting it was there.
fn swap(body: &str, needle: &str, replacement: &str) -> String {
    let out = body.replacen(needle, replacement, 1);
    assert_ne!(body, out, "the schema must carry `{}`", needle.trim_end());
    out
}

/// A conformant `adr` body, stamped at the shipped `adr` manifest version.
fn adr_body(title: &str) -> String {
    format!(
        "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# {title}

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
"
    )
}

/// The one finding of `code` in a store report, or a panic naming what the report did carry.
fn finding<'a>(report: &'a serde_json::Value, code: &str, rel: &str) -> &'a serde_json::Value {
    report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the report carries a findings array; got:\n{report:#}"))
        .iter()
        .find(|f| {
            f["code"] == code
                && f["key"]["target"]
                    .as_str()
                    .is_some_and(|target| target.contains(rel))
        })
        .unwrap_or_else(|| panic!("no `{code}` at `{rel}`; the report was:\n{report:#}"))
}

/// The finding the adoption door attached to the candidate at `rel`, or `None` when it
/// attached none — `jigc ingest`'s envelope carries one finding per **candidate row** (its
/// top-level `findings` is the run-level array T3 added), and reading the run-level array
/// instead would silently answer *no finding* for every file.
fn ingest_finding<'a>(report: &'a serde_json::Value, rel: &str) -> Option<&'a serde_json::Value> {
    report["rows"]
        .as_array()
        .unwrap_or_else(|| panic!("`jigc ingest` carries a `rows` array; got:\n{report:#}"))
        .iter()
        .find(|row| row["file"] == rel)
        .unwrap_or_else(|| panic!("`{rel}` must reach the adoption funnel; got:\n{report:#}"))
        .get("finding")
        .filter(|finding| !finding.is_null())
}

/// A finding's `message`, as the machine surface carries it.
fn message(finding: &serde_json::Value) -> &str {
    finding["message"]
        .as_str()
        .unwrap_or_else(|| panic!("a finding carries a message; got:\n{finding:#}"))
}

/// A finding's `route`, as the machine surface carries it.
fn route(finding: &serde_json::Value) -> &str {
    finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("a finding of this family carries a route; got:\n{finding:#}"))
}

/// The claim the corrected producer must state, and the claim it must no longer make.
const COMPUTED_CLAIM: &str = "no resolved doctype claims this path";
const FALSE_TAIL: &str = "no schema in the composed set";

// ---------------------------------------------------------------------------------------------
// The two driven cells.
// ---------------------------------------------------------------------------------------------

/// **The relocated-home cell — the schema IS composed, and no surface may say otherwise.**
///
/// `adr`'s home moves from `decisions/` to `adrs/` in a legally re-pinned pack copy and its
/// committed instance stays behind. At that one commit the arm reads three surfaces: the
/// schema projection (which proves the doctype is composed), the store sweep, and the adoption
/// door. The sweep's message must state what it computed and nothing about the composed set,
/// and its route must lead to the door that can tell the two causes apart — which, driven here,
/// answers with the home to move the file to.
#[test]
fn the_relocated_home_cell_stops_claiming_the_schema_is_absent() {
    let corpus = Corpus::reshaped("relocated", "adr", |shipped| {
        swap(shipped, "location: decisions/\n", "location: adrs/\n")
    });
    let stranded = "docs/decisions/use-sqlite.md";
    corpus.commit_doc(stranded, &adr_body("Use SQLite for sessions"));

    // The composed set *does* define `adr` — the fact every claim below is held against.
    let (schema, schema_ok) = corpus.run(&["doc", "schema", "adr"]);
    assert!(
        schema_ok,
        "`adr` must be composed, or this cell is the departed-doctype one; got:\n{schema}",
    );

    // The store sweep.
    let out = corpus.jigc(&["validate", "--format", "json"]);
    assert!(
        !out.status.success(),
        "a stamped file inside jigc's homes that no doctype claims flips the store exit",
    );
    let report: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("`jigc validate --format json` emits JSON");
    let orphan = finding(&report, "schema-conformance.orphaned-instance", stranded);

    assert!(
        message(orphan).contains(COMPUTED_CLAIM),
        "the message must state the claim the producer computed; got:\n{}",
        message(orphan),
    );
    assert!(
        !message(orphan).contains(FALSE_TAIL),
        "`adr` is in the composed set at this very commit — the message may not say otherwise; \
         got:\n{}",
        message(orphan),
    );

    // The adoption door, at the same commit, over the same file.
    let ingest = corpus.json(&["ingest"]);
    let wrong_home = ingest_finding(&ingest, stranded)
        .unwrap_or_else(|| panic!("the adoption door speaks for this file; got:\n{ingest:#}"));
    assert_eq!(
        wrong_home["code"], "ingest.wrong-location",
        "the conformant-but-misplaced cell is the one this fixture builds",
    );
    assert!(
        message(wrong_home).contains("adr"),
        "the adoption door names the conformant doctype — the schema the sweep's tail used to \
         deny; got:\n{}",
        message(wrong_home),
    );
    assert!(
        message(wrong_home).contains("adrs/"),
        "and it names the home to move the file to; got:\n{}",
        message(wrong_home),
    );

    // The two doors now agree, and the sweep's route leads to the one that can say which cause
    // this is.
    assert!(
        route(orphan).contains("jigc ingest"),
        "the sweep's route must hand the reader to the door that answers *which* cause this \
         is; got:\n{}",
        route(orphan),
    );

    // And the text surface says the same thing the envelope does.
    let (text, _) = corpus.run(&["validate"]);
    assert!(
        text.contains(COMPUTED_CLAIM) && !text.contains(FALSE_TAIL),
        "the agent text and the envelope carry one claim about this file; validate said:\n{text}",
    );
}

/// **The departed-doctype cell — the control, where the schema genuinely is absent.**
///
/// A correction that made the message true in one cell by making it false in the other would
/// be the same defect facing the other way, so the cell M51 built the finding for is driven
/// too: a stamped `docs/roadmap.md` under a composition that carries only the dev pack, where
/// nothing defines `roadmap` at all. The message must still be true, and — because `jigc
/// ingest` cannot place a file whose type no schema defines — the route's other two exits must
/// be the ones the operator is left with.
#[test]
fn the_departed_doctype_cell_keeps_a_true_message_and_a_usable_route() {
    let corpus = Corpus::stock("departed");
    let departed = "docs/roadmap.md";
    corpus.commit_doc(
        departed,
        "---\nschema-version: 3\n---\n\n# Roadmap\n\n## Milestones\n\nNothing yet.\n",
    );

    // The doctype really is gone from this composition.
    let (schema, schema_ok) = corpus.run(&["doc", "schema", "roadmap"]);
    assert!(
        !schema_ok,
        "`roadmap` must be undefined here, or this is not the departed-doctype cell; \
         got:\n{schema}",
    );

    let out = corpus.jigc(&["validate", "--format", "json"]);
    assert!(!out.status.success(), "the store exit flips here too");
    let report: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("`jigc validate --format json` emits JSON");
    let orphan = finding(&report, "schema-conformance.orphaned-instance", departed);

    assert!(
        message(orphan).contains(COMPUTED_CLAIM),
        "one message serves both cells — it states what was computed, which is true in each; \
         got:\n{}",
        message(orphan),
    );

    // The two exits that apply when no resolved schema can accept the file.
    let route = route(orphan);
    assert!(
        route.contains("re-add the pack"),
        "restoring what claims it is the first exit in this cell; got:\n{route}",
    );
    assert!(
        route.contains("jigc unmanage") && route.contains("stamp"),
        "and taking it out of jigc's world names the act that finishes it — `unmanage` leaves \
         the bytes, so the stamp alone keeps the finding alive; got:\n{route}",
    );

    // The adoption door does not claim it can place this file: the route's `ingest` exit is
    // conditional, and this is the condition failing.
    let ingest = corpus.json(&["ingest"]);
    let placed = ingest_finding(&ingest, departed)
        .is_some_and(|finding| finding["code"] == "ingest.wrong-location");
    assert!(
        !placed,
        "no resolved schema accepts this file, so the adoption door must not offer it a home; \
         ingest said:\n{ingest:#}",
    );
}
