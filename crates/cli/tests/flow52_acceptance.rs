//! **The M51 wave's done-picture acceptance suite** — the wave driven end to end through
//! the **real `jigc` binary** (`design/worked-examples.md` → flow 52; roadmap → Milestone
//! 51, Increment 11; the arm set is `completions/artifacts/M51/acceptance-design.md` →
//! Part 1, adopted at `settle-record.md` §19).
//!
//! **The claim the wave proves is one claim:** *no caller-supplied token and no repository
//! posture reaches a door that destroys, commits or moves without that door having
//! adjudicated it — the two M50 exemptions are closed as classes — and every surface 1.0.0
//! pins says what the binary does.*
//!
//! **Two honest bounds ride the claim, and they are symmetric** — each names a set the wave
//! closed *as a classification* rather than *as a rule*:
//!
//!   * the `ArgToken::Plain` family ships as a **classified registry with one stated rule
//!     or one stated no-rule-and-why per member** — the table can still be answered
//!     *wrongly*; it can no longer be answered *silently*;
//!   * the posture family closes over **three of four** driven members, with the `GIT_DIR`
//!     redirect **declared out** and carried as a stated row rather than as a silence.
//!
//! Increments 1–10 shipped each fix with its own axis suite; this suite is the
//! **composite acceptance** that ties the wave into nine done-picture arms — **each arm
//! stating which kind of set it iterates**, and each stating what it adds over the axis
//! suite beside it, because an arm that re-runs a shipped axis proves the axis twice and
//! the wave once.
//!
//! The nine arms:
//!
//!   (1) **The occurrence-keyed path-argument registry** — [`PATH_ARG_OCCURRENCES`], a
//!       **code-side registry** keyed by `(leaf, argument id, conditional arm)` and
//!       ⇔-fenced against the clap tree. *Adds over `path_arg_occurrence_axis.rs`:* that
//!       suite sweeps every arm over all nine escape shapes in three bases and adjudicates
//!       each cell's own outcome; this arm hands five escapes to **every** occurrence in
//!       **one** repository and asserts the **composite cost** — nothing outside the
//!       repository changed, every commit the sweep landed touched the cascade and never
//!       the store, and the store still addresses every doc it did before. EC-28's
//!       name-ceiling code rides here too: a `SLUG_DOORS` occurrence is a registry row.
//!
//!   (2) **The posture family × the commit-on-behalf class** — [`BEHALF_DOORS`], D2's
//!       **total classification** of every clap leaf on the `VERB_KINDS` mold, crossed with
//!       [`PostureMember::ALL`], the family's **defining case-set**. *Adds over
//!       `posture_door_axis.rs`:* that suite drives each row in its own fresh repo and owns
//!       every route's text; this arm drives **every acting door into one repository per
//!       posture state** and asserts the **partition** — a committing door refuses the full
//!       family minus its stated exemptions, a mover refuses an operation in progress and
//!       nothing else — and then that the repository's posture is **still exactly what it
//!       was**: a door that refuses a merge may not half-conclude it.
//!
//!   (3) **`setup`'s own install pathspec** — a **derivation stated as one**: the subject is
//!       the eight paths the install commit actually carries, read back from
//!       `git show --name-only` rather than written down, and the predicate is a *git
//!       query*, not a registry. *Adds over `setup_install_pathspec_guard.rs`:* that suite
//!       drives eighteen hand-built cells; this arm derives the pathspec from the binary
//!       and drives **every derived path**, asserting the one rule that decides them all —
//!       *bytes that survive the install would ride the commit, so the door refuses; bytes
//!       the install regenerates are jigc's own and are exempt* — plus §1's driven red and
//!       the unrelated-staged-file control.
//!
//!   (4) **The config-layer CAS pre-image** — `{stage failure, hook rejection} ×
//!       {unchanged, concurrently edited}`, a **manufactured shape space, and it says so**:
//!       the failure points are *decided* and the second axis is a **race**, which no
//!       code-side set carries. *Adds over `config_layer_preimage.rs`:* that suite owns the
//!       four cells, the absent-pre-image arm and the pathspec totality; this arm walks the
//!       **adopter's loss** in one repository — a private, uncommitted line in
//!       `.jigc/.gitignore` and a rejecting `pre-commit` — and asserts that on the
//!       unraced run the line comes back while `git status` is **not** falsely clean, and on
//!       the raced run **both** versions survive under one `finalize.rollback-conflict`.
//!
//!   (5) **The `EnvelopeArm` registry** — [`ENVELOPE_ARMS`], a **code-side registry** that
//!       is *production-side*: it is what the renderer answers from, not a census of what
//!       the binary happened to emit. *Adds over `format_json_success_axis.rs`:* that suite
//!       drives every declared arm and owns the four proofs; this arm drives the **seven wire
//!       changes the wave made** — the four deletes, `ConfigAck::Set`'s `relocated`, the
//!       forecast's `findings`, and N15's `--task` reject — and asserts each driven
//!       document's top-level key set **equals the registry's declared one**, so a key the
//!       wave removed cannot come back without the registry saying so.
//!
//!   (6) **[`WORK_UNIT_ID_DOORS`] filtered to the unknown-id cell** — an **existing
//!       code-side registry**, filtered to one cell of its own token axis. *Adds over
//!       `work_unit_unknown_envelope.rs`:* that suite owns the per-family key and the route
//!       split; this arm drives the unknown-id cell and M50's malformed-id cell at **every**
//!       door of **one** corpus and then mints a task and finalizes it to a **real commit**,
//!       so the doors that refuse a token that names nothing have not learned to refuse the
//!       ids jigc itself mints.
//!
//!   (7) **[`ManifestKind::ALL`]** — the class's **defining case-set, matched
//!       exhaustively**: each variant is mapped through a compiler-checked `match` to the
//!       fixture act that produces it and the commit model whose forecast carries it, so a
//!       new kind cannot compile until someone decides how to drive it. *Adds over
//!       `count_fences.rs` and `finalize_manifest.rs`:* those fence the *numerals prose
//!       states* and the manifest's own arms; this arm puts **every kind of the vocabulary
//!       on the wire at once**, from one checkout — the ordinary finalize, and since M55 the
//!       doc-only forecast beside it that `left-staged` is reachable on — on both the agent
//!       text and the `--format json` document.
//!
//!   (8) **The derived ambush owe-set** — a **derivation stated as one**, with a stated
//!       exclusion rule: *blocking codes minted by a door in the commit-on-behalf class*,
//!       **minus** rows carrying a stated `Exempt(reason)`, union the declared
//!       non-mint identifiers. *Adds over `stated_at_fence.rs`:* that suite owns the four
//!       tiers and the two registry legs; this arm drives the derivation's **consequence** —
//!       a pack whose declarer is stripped reddens **pack-load at every door**, and the
//!       exempt row is declared by no step of either pack while pack-load stays green,
//!       which is the cell a hand-list could not express.
//!
//!   (9) **The orphan arm over [`STORE_EXIT_FLIPS`]** — a **code-side registry where
//!       membership *is* the assertion**: every string this arm asserts is read back off the
//!       member, so a member that stopped flipping the exit reddens here. *Adds over
//!       `orphaned_instance.rs`:* that suite owns the partition against the strand advisory
//!       and the resolved-but-unversioned sibling; this arm walks the adopter's picture —
//!       `jigc validate` **exits non-zero**, `jigc doc list` prints the same files as
//!       `orphaned` rows with a null identity, and the two consumers agree as sets.
//!
//! **What gets no arm, recorded as a decision** (the M46 Increment 9 / M48 Increment 11 /
//! M49 Increment 12 precedent — *an increment that mints no verb, finding or route carries
//! nothing for a done-picture walk to reach, and manufacturing an arm would be a walk
//! written to have an arm rather than to prove a claim*):
//!
//!   * **Increments 9, 10 and 11.** Increment 9 is the surface-and-route batch whose
//!     behaviour the arms above already reach; Increment 10 is the adopter-doc and
//!     internal-record batch, whose only change is what a sentence *says*; Increment 11 is
//!     this suite, the two ledgers, the goldens and the fold-back.
//!   * **D6** (the release-versioning policy) and **§13**'s batched adopter-doc hash move:
//!     doc-only. The *behaviour* they describe is proven by arms 5 and 9.
//!   * **D11** (the version-stamp fence) and **D8**'s `foldback_truth` re-keying:
//!     build-time fences over this repo's own prose. §15 records D11 as admitted by the
//!     human's boundary and **refused by razor leg 2** — it benefits our process only,
//!     which is precisely why no adopter-facing walk can reach it.
//!   * **D9's invocation-log key-set fence**: the log is a gitignored measurement artifact,
//!     not a surface; its closure is an exhaustive destructure over `Record`.
//!   * **EC-7's floor-scraper repair**: a test-harness fix; the floor's *behaviour* already
//!     has flow 51 arm 7.
//!   * **F-9** (`commit-recording.stale-title`) is **not** an arm: it is pinned by its own
//!     increment suite, which is what the conversion ledger cites.
//!
//! Two in-scope rows ride existing arms rather than earning their own, so they are not
//! lost: **EC-28**'s name-ceiling code at the three `SLUG_DOORS` rides **arm 1** (a
//! `SLUG_DOORS` occurrence is a registry row), and **N15**'s `--task` arm of
//! `store.not-found` rides **arm 5** (it ships as a D5 row on the pinned read surface).
//!
//! Isolation: every arm builds its own throwaway repo — a real `git init`, a per-repo git
//! identity, `$HOME` repointed and `JIGC_PACK_DIR` scrubbed unless the arm deliberately
//! supplies a fixture pack — or rides the shared [`support::trial_corpus`] substrate, which
//! does all four by construction.

#![cfg(unix)]

use crate::support;

use cli::cli::{
    ActsOnBehalf, BEHALF_DOORS, PATH_ARG_OCCURRENCES, PATH_ARG_SLOT, PathArgDisposition,
    PathArgSubject, SLUG_DOOR_SOURCE, SLUG_DOORS, SLUG_OVERRIDE_SLOT, WORK_UNIT_ID_DOOR_PAYLOAD,
    WORK_UNIT_ID_DOORS, WORK_UNIT_ID_SLOT,
};
use cli::pack::{AMBUSH_CONTRACTS, AmbushDisposition, ambush_class_codes};
use cli::render::{
    ArmOutcome, ArmShape, ENVELOPE_ARMS, ManifestKind, STORE_EXIT_FLIPS, StoreExitFlip,
};
use cli::repo::PostureMember;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use support::trial_corpus::{FixturePack, State, TrialCorpus};

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// A throwaway directory that removes itself on drop — for the arms that build a repo, a
/// pack or a canary by hand rather than riding [`TrialCorpus`].
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow52-{tag}-{}-{:?}",
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

/// Both of an invocation's streams, joined — the surface a reader actually meets.
fn surface(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Run `git <args>` in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Run `git <args>` in `repo` without asserting — for the commands whose non-zero exit is
/// the state being built (a conflicting merge, a stopped rebase).
fn git_try(repo: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git")
}

/// A real repository with one commit on `main`, plus an isolated `$HOME`.
fn born_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(&format!("repo-{tag}"));
    let home = TempDir::new(&format!("home-{tag}"));
    git(repo.path(), &["init", "-q", "-b", "main", "."]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-qm", "initial"]);
    (repo, home)
}

/// Run the real binary in `repo` with `$HOME = home` and no inherited `JIGC_PACK_DIR`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// Parse a `--format json` payload, surfacing the bytes on failure.
fn json(payload: &str) -> Value {
    serde_json::from_str(payload)
        .unwrap_or_else(|err| panic!("the payload is JSON ({err}); got:\n{payload}"))
}

/// A JSON object's top-level key set.
fn top_level_keys(value: &Value) -> BTreeSet<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("the document is a JSON object; got:\n{value:#}"))
        .keys()
        .cloned()
        .collect()
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — the occurrence-keyed path-argument registry (a CODE-SIDE REGISTRY)
// ═════════════════════════════════════════════════════════════════════════════

/// Every blocking code this wave's path rules refuse with. A `NoRule` arm claims the token
/// becomes no path component, so **none** of these may ever be its answer; an `Adjudicated`
/// arm that refuses with one of them must refuse with one of *its own*.
const PATH_RULE_CODES: &[&str] = &[
    "migrate.source-untrackable",
    "migrate.source-untracked",
    "config.step-source-untrackable",
    "config.untrackable-root",
    "config.workbench-root",
    "config.unusable-root",
    "finalize.retire-untrackable",
];

/// The bytes planted **outside** the repository: simultaneously the canary whose survival is
/// the composite cost, and a valid `jigc doc author` payload, so the arms that reach it as
/// *content* answer about the content rather than about a parse failure.
const CANARY: &str = "\
title: Probe
sections:
  - id: context
    set: {context: <<Flow 52 prose.>>}
";

/// The in-repo path git holds no copy of — the fifth cell's spelling.
const UNTRACKED_SOURCE: &str = "untracked-source.yaml";

/// The in-repo symlink the third cell reaches through — **relative**, so a copied corpus's
/// link points at the copy's own outside directory.
const SYMLINK_DIR: &str = "linkdir";

/// A well-formed slug over `cli::cli::SLUG_NAME_CEILING` — EC-28's cell, riding this arm
/// because a `SLUG_DOORS` occurrence is a registry row. 300 bytes rather than
/// `ceiling + 1` on purpose: it is the token M51's baseline drove, and it clears every
/// filesystem's own `NAME_MAX` by enough that a door which let it through fails loudly
/// rather than subtly.
const OVER_CEILING: &str = "a-very-long-slug-that-no-filesystem-can-name-as-one-component-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

/// The name-ceiling code EC-28 minted for the whole `--slug` family.
const NAME_CEILING_CODE: &str = "write.slug-name-ceiling";

/// One escape shape. Five of the axis suite's nine: the ones the acceptance design names,
/// which are the shapes that *escape* rather than the two magic spellings and the stdin
/// sentinel — those are the axis suite's, and the sentinel is an arm split rather than an
/// escape at all.
#[derive(Clone, Copy, Debug)]
enum Escape {
    /// An absolute path outside the repository.
    Absolute,
    /// A `../` hop above the repository root.
    Parent,
    /// A path *through* a symlink that leaves the repository.
    Symlink,
    /// A path with a `.git` component.
    GitComponent,
    /// An in-repo path git holds no copy of.
    UntrackedInRepo,
}

impl Escape {
    const ALL: [Escape; 5] = [
        Escape::Absolute,
        Escape::Parent,
        Escape::Symlink,
        Escape::GitComponent,
        Escape::UntrackedInRepo,
    ];
}

/// The token one escape takes at one subject. [`PathArgSubject`] is part of the **registry**
/// rather than of this suite, which is what lets one substitution serve every occurrence: an
/// occurrence whose subject is a *home* answers about a directory, and handing it a file
/// spelling would be asking it something else.
fn escape_token(escape: Escape, subject: PathArgSubject, outside: &Path) -> String {
    let file = |escape: Escape| -> String {
        match escape {
            Escape::Absolute => outside.join("payload.yaml").to_string_lossy().into_owned(),
            Escape::Parent => "../outside/payload.yaml".to_owned(),
            Escape::Symlink => format!("{SYMLINK_DIR}/payload.yaml"),
            Escape::GitComponent => ".git/config".to_owned(),
            Escape::UntrackedInRepo => UNTRACKED_SOURCE.to_owned(),
        }
    };
    match subject {
        PathArgSubject::SourceFile | PathArgSubject::FieldValue => file(escape),
        PathArgSubject::Home => match escape {
            Escape::Absolute => outside.to_string_lossy().into_owned(),
            Escape::Parent => "../outside".to_owned(),
            Escape::Symlink => SYMLINK_DIR.to_owned(),
            Escape::GitComponent => ".git/docs".to_owned(),
            Escape::UntrackedInRepo => "untracked-home".to_owned(),
        },
        PathArgSubject::AddressTail(head) => format!("{head}{}", file(escape)),
    }
}

/// The committed `adr` the `SLUG_DOORS` rows address, with `{stamp}` standing in for the
/// doctype's **current** `schema-version` — read back from the binary rather than written as
/// a literal, so a doctype bump does not turn this arm's fixture into a migration fault.
const ADR_KEEPER: &str = "\
---
status: accepted
date: 2026-06-28
schema-version: {stamp}
---

# Keeper

## Context

Context.

## Options

Alternatives were weighed and rejected.

## Decision

Decided.

## Consequences

Effects.
";

/// The foreign source the `jigc migrate --slug` row adopts, so that row answers about the
/// override and not about a missing file.
const FOREIGN_CHANGELOG: &str = "# Change Log\n\n## v1\n\n- did a thing\n";

/// **Arm 1** — every occurrence of every path-bearing argument, five escapes, one
/// repository, and the composite cost asserted once.
///
/// **The kind of set: a code-side registry.** [`PATH_ARG_OCCURRENCES`] is keyed by
/// `(leaf, argument id, conditional arm)` and derived from [`cli::cli::ARG_TOKENS`], a
/// **total** classification of every argument of every leaf verb, ⇔-fenced against the real
/// clap tree — so a fifteenth occurrence cannot ship without joining it, and each row
/// carries its own runnable argv, which is why this arm writes no cell list of its own.
///
/// **The assertion is the registry's own disposition, never a second expectation table.** A
/// copy of `path_arg_occurrence_axis.rs`'s `(row, cell) → outcome` table here would be the
/// two-lists-that-agree-today shape the complete-fix contract is named for. So this arm
/// asserts what the *registry* claims and the axis suite cannot: an `Adjudicated` arm that
/// refuses with a path-rule code refuses with one of **its own** declared codes, and a
/// `NoRule` arm — whose whole claim is that the token becomes no path component — never
/// answers with a path rule at all.
#[test]
fn every_path_arg_occurrence_is_adjudicated_and_the_sweep_costs_the_store_nothing() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let repo = corpus.repo();

    // The committed `adr` the `--slug` rows address.
    let adr_stamp =
        json(&corpus.jigc_ok(&["doc", "schema", "adr", "--format", "json"]))["schema-version"]
            .as_u64()
            .expect("`doc schema adr` reports the doctype's current schema-version");
    fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
    fs::write(
        repo.join("docs/decisions/keeper.md"),
        ADR_KEEPER.replace("{stamp}", &adr_stamp.to_string()),
    )
    .expect("write the keeper adr");
    fs::write(repo.join(SLUG_DOOR_SOURCE), FOREIGN_CHANGELOG).expect("write the migrate source");
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-qm", "seed the arm's fixtures"]);

    // The fixtures the registry's argvs name, and the canary OUTSIDE the repository.
    let outside = repo
        .parent()
        .expect("the corpus root is the repo's parent")
        .join("outside");
    fs::create_dir_all(&outside).expect("create the outside dir");
    fs::write(outside.join("payload.yaml"), CANARY).expect("plant the canary");
    let link = repo.join(SYMLINK_DIR);
    if !link.exists() {
        std::os::unix::fs::symlink("../outside", &link).expect("plant the symlink");
    }
    fs::write(repo.join(UNTRACKED_SOURCE), CANARY).expect("plant the untracked source");
    fs::write(
        repo.join("replacement-step.yaml"),
        "id: replacement-step\nbody: Replacement.\n",
    )
    .expect("plant the step source");

    // One live task with a staged `adr`, so the in-task rows' argvs are runnable and the
    // token is the only thing their doors can fault on.
    corpus.jigc_ok(&["start", "--workflow", "single-task", "probe"]);
    corpus.jigc_ok(&[
        "doc", "create", "adr", "--title", "Probe", "--task", "probe",
    ]);

    let before_head = git(&repo, &["rev-parse", "HEAD"]).trim().to_owned();
    let before_docs = doc_identities(&corpus);
    let outside_before = read_dir_bytes(&outside);

    let mut driven: BTreeSet<(String, String, String)> = BTreeSet::new();
    for occurrence in PATH_ARG_OCCURRENCES {
        let door = occurrence.door.join(" ");
        for arm in occurrence.arms {
            // The `Literal` arms claim the stdin sentinel, which is not an escape shape —
            // the arm split itself, and `path_arg_occurrence_axis.rs`'s cell.
            if !matches!(arm.token, cli::cli::ArmToken::Caller) {
                continue;
            }
            driven.insert((door.clone(), occurrence.arg.to_owned(), arm.when.to_owned()));
            for escape in Escape::ALL {
                let token = escape_token(escape, arm.subject, &outside);
                let argv: Vec<String> = arm
                    .argv
                    .iter()
                    .map(|part| {
                        if *part == PATH_ARG_SLOT {
                            token.clone()
                        } else {
                            (*part).to_owned()
                        }
                    })
                    .collect();
                let out =
                    corpus.jigc_stdin(&argv.iter().map(String::as_str).collect::<Vec<_>>(), CANARY);
                let text = surface(&out);
                let shown = format!("jigc {door} [{}] {escape:?} `{token}`", arm.when);
                assert_ne!(
                    out.status.code(),
                    Some(101),
                    "{shown}: a token's disposition is a finding or an acceptance, never a \
                     panic\n{text}",
                );
                let fired: Vec<&str> = PATH_RULE_CODES
                    .iter()
                    .copied()
                    .filter(|code| text.contains(*code))
                    .collect();
                match arm.disposition {
                    PathArgDisposition::Adjudicated { codes, .. } => {
                        for code in &fired {
                            assert!(
                                codes.contains(code),
                                "{shown}: an adjudicated occurrence refuses with one of its \
                                 OWN declared codes ({codes:?}) — `{code}` belongs to another \
                                 arm's rule\n{text}",
                            );
                        }
                    }
                    PathArgDisposition::NoRule { why } => {
                        assert!(
                            fired.is_empty(),
                            "{shown}: this occurrence carries a stated NO-RULE — {why} — so no \
                             path rule may be its answer; got {fired:?}\n{text}",
                        );
                    }
                }
            }
        }
    }

    let declared: BTreeSet<(String, String, String)> = PATH_ARG_OCCURRENCES
        .iter()
        .flat_map(|occurrence| {
            occurrence
                .arms
                .iter()
                .filter(|arm| matches!(arm.token, cli::cli::ArmToken::Caller))
                .map(|arm| {
                    (
                        occurrence.door.join(" "),
                        occurrence.arg.to_owned(),
                        arm.when.to_owned(),
                    )
                })
        })
        .collect();
    assert_eq!(
        driven, declared,
        "every caller-token arm of the registry is driven — a row this arm cannot reach is a \
         failure, never a skip",
    );

    // ── EC-28 · the OS name ceiling, at every `--slug` door of the same repository. ──
    for row in SLUG_DOORS {
        let argv: Vec<&str> = row
            .argv
            .iter()
            .map(|part| {
                if *part == SLUG_OVERRIDE_SLOT {
                    OVER_CEILING
                } else {
                    *part
                }
            })
            .collect();
        let out = corpus.jigc(&argv);
        let text = surface(&out);
        let shown = format!("jigc {} --slug <over-ceiling>", row.door.join(" "));
        assert!(
            !out.status.success(),
            "{shown}: a `--slug` the OS cannot name as one path component must be refused at \
             the door\n{text}",
        );
        assert!(
            text.contains(NAME_CEILING_CODE),
            "{shown}: the refusal carries `{NAME_CEILING_CODE}` — never an I/O story, and \
             never the grammar code for a value that obeys the grammar\n{text}",
        );
    }
    assert!(
        !walk_paths(&repo)
            .iter()
            .any(|path| path.contains(OVER_CEILING)),
        "no door minted a path component from the over-ceiling override",
    );

    // ── The composite cost. ──
    assert_eq!(
        read_dir_bytes(&outside),
        outside_before,
        "nothing outside the repository may be created, rewritten or unlinked by any door \
         of this sweep — that outside file is the byte `jigc task finalize --approve` was \
         recorded to delete at the wave's baseline",
    );
    let landed = git(
        &repo,
        &["log", "--format=%H", &format!("{before_head}..HEAD")],
    );
    for sha in landed.split_whitespace() {
        let touched = git(&repo, &["show", "--name-only", "--pretty=format:", sha]);
        for path in touched.split_whitespace() {
            assert!(
                path.starts_with(".jigc/config/"),
                "a commit this sweep landed ({sha}) touched `{path}` — the accepting arms \
                 record a cascade delta, and nothing else reached a commit. If a future \
                 `jigc config set` lands its relocation in the same commit, this is where \
                 that shows up: read the commit before widening the rule, because the \
                 escape tokens are the other thing that could have put a path there",
            );
        }
    }
    assert_eq!(
        doc_identities(&corpus),
        before_docs,
        "the store still addresses exactly the identities it did before the sweep — a doc \
         whose identity no `doc list`, `doc show` or finding can name again is precisely \
         what an unadjudicated token used to cost. The subject is the IDENTITY set and not \
         the listing's bytes, because one accepting cell of the registry legitimately moves \
         a home: `jigc config set docs-root <a new relative directory>` is the ordinary \
         case the root-knob rules admit, and a composite that reddened over it would be \
         asserting that a shipped knob may not be used.",
    );
}

/// Every identity the committed store addresses, with its registration state — the property
/// the token families are about, read off the pinned index projection.
fn doc_identities(corpus: &TrialCorpus) -> BTreeSet<(String, String)> {
    json(&corpus.jigc_ok(&["doc", "list", "--format", "json"]))["docs"]
        .as_array()
        .expect("`doc list --format json` carries a `docs` array")
        .iter()
        .map(|row| {
            (
                row["id"].as_str().unwrap_or("<null>").to_owned(),
                row["state"].as_str().unwrap_or("<null>").to_owned(),
            )
        })
        .collect()
}

/// Every regular file under `dir`, as `(relative path, bytes)` — the outside directory's
/// whole content, so a file created, rewritten **or unlinked** all redden the same way.
fn read_dir_bytes(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            out.insert(
                entry.file_name().to_string_lossy().into_owned(),
                fs::read(&path).unwrap_or_default(),
            );
        }
    }
    out
}

/// Every path under `root`, as repo-relative strings — used to assert that a refused token
/// minted no path component anywhere.
fn walk_paths(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            out.push(path.to_string_lossy().into_owned());
            if path.is_dir() && !path.is_symlink() {
                stack.push(path);
            }
        }
    }
    out
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — the posture family × the commit-on-behalf class
//         (a TOTAL CLASSIFICATION on the VERB_KINDS mold, crossed with the
//          family's DEFINING CASE-SET)
// ═════════════════════════════════════════════════════════════════════════════

/// The well-formed work-unit id every `<id>` slot in a row's argv is filled with. The
/// posture guard answers **before** any id resolution, which is why a well-formed id that
/// names nothing is enough for the door to fault on the posture and nothing else.
const POSTURE_ID: &str = "flow52-unit";

/// A repository in one posture state, with a jigc project layer present so that no door can
/// answer *"no project layer"* instead of answering the posture.
///
/// The layer is created by hand rather than by `jigc setup`, whose install commit would
/// birth `HEAD` and destroy the unborn state.
fn posture_repo(state: PostureMember) -> (TempDir, TempDir) {
    let repo = TempDir::new("posture");
    let home = TempDir::new("posture-home");
    git(repo.path(), &["init", "-q", "-b", "main", "."]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk the project layer");
    if state == PostureMember::HeadUnborn {
        return (repo, home);
    }
    fs::write(repo.path().join("conflict.txt"), "base\n").expect("write the conflict subject");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-qm", "one"]);
    match state {
        PostureMember::HeadUnborn => unreachable!("returned above"),
        PostureMember::HeadDetached => {
            git(repo.path(), &["checkout", "-q", "--detach", "HEAD"]);
        }
        PostureMember::OperationInProgress => {
            git(repo.path(), &["checkout", "-q", "-b", "side"]);
            fs::write(repo.path().join("conflict.txt"), "side\n").expect("write side");
            git(repo.path(), &["add", "-A"]);
            git(repo.path(), &["commit", "-qm", "side"]);
            git(repo.path(), &["checkout", "-q", "main"]);
            fs::write(repo.path().join("conflict.txt"), "main\n").expect("write main");
            git(repo.path(), &["add", "-A"]);
            git(repo.path(), &["commit", "-qm", "main2"]);
            // The conflict IS the state being built, so git's non-zero exit is expected.
            let _ = git_try(repo.path(), &["merge", "--no-commit", "side"]);
            assert!(
                repo.path().join(".git").join("MERGE_HEAD").exists(),
                "a conflicting merge must leave MERGE_HEAD behind",
            );
        }
    }
    (repo, home)
}

/// One acting row's runnable argv, with the work-unit-id slot filled.
fn acting_argv(acts: &ActsOnBehalf) -> Option<Vec<String>> {
    let argv = match acts {
        ActsOnBehalf::Neither => return None,
        ActsOnBehalf::CommitsOnBehalf { argv, .. } | ActsOnBehalf::MovesOnBehalf { argv } => *argv,
    };
    Some(
        argv.iter()
            .map(|token| {
                if *token == WORK_UNIT_ID_SLOT {
                    POSTURE_ID.to_owned()
                } else {
                    (*token).to_owned()
                }
            })
            .collect(),
    )
}

/// Whether this row's class adjudicates `member`, honouring the row's own stated exemptions.
fn adjudicates(acts: &ActsOnBehalf, member: PostureMember) -> bool {
    match acts {
        ActsOnBehalf::CommitsOnBehalf { exempt, .. } => {
            !exempt.iter().any(|row| row.member == member)
        }
        ActsOnBehalf::MovesOnBehalf { .. } => member == PostureMember::OperationInProgress,
        ActsOnBehalf::Neither => false,
    }
}

/// **Arm 2** — the classification's two acting classes, driven into one repository per
/// posture state, with the state asserted intact afterwards.
///
/// **The kinds of set: a total classification crossed with a defining case-set.**
/// [`BEHALF_DOORS`] classifies **every** clap leaf commit-on-behalf / move-on-behalf /
/// neither, ⇔-fenced against the clap tree on [`cli::cli::VERB_KINDS`]' mold — so a verb
/// added anywhere reddens until someone answers it — and [`PostureMember::ALL`] is the
/// family's own case-set. The *neither* class is **fenced, not driven**: its rows carry no
/// argv, because a door that adjudicates no posture has no cell to drive.
///
/// **What this adds over `posture_door_axis.rs`.** That suite drives each row in its own
/// fresh repository and owns every route's text; it can therefore say nothing about what a
/// *sequence* of refusals costs. This arm drives the whole acting classification into **one**
/// mid-merge repository — under an operation in progress *both* classes refuse, so the sweep
/// mutates nothing by construction — and then asserts the adopter's picture: the merge is
/// still exactly where they left it, `MERGE_HEAD` present, `HEAD` unmoved, the commit count
/// unmoved and the conflicted bytes untouched. A door that refuses to conclude someone
/// else's merge may not half-conclude it.
///
/// **Red at the wave's base** (`DECISIONS.md` → the M51 Increment 2 entries): every one of
/// these doors acted. `design/finalize.md` → 1. Preflight had promised *"No in-progress
/// merge/rebase/bisect"* verbatim since it was written, with **zero** probes behind it in
/// either crate.
#[test]
fn every_acting_door_refuses_an_operation_in_progress_and_leaves_it_exactly_as_it_found_it() {
    let (repo, home) = posture_repo(PostureMember::OperationInProgress);
    let (repo, home) = (repo.path(), home.path());
    let head_before = git(repo, &["rev-parse", "HEAD"]);
    let commits_before = git(repo, &["rev-list", "--count", "--all"]);
    let conflict_before = fs::read_to_string(repo.join("conflict.txt")).expect("read conflict");

    let mut driven = 0usize;
    for row in BEHALF_DOORS {
        let Some(argv) = acting_argv(&row.acts) else {
            continue;
        };
        driven += 1;
        let shown = format!("jigc {}", row.door.join(" "));
        let out = jigc(
            repo,
            home,
            &argv.iter().map(String::as_str).collect::<Vec<_>>(),
        );
        let text = surface(&out);
        assert!(
            !out.status.success(),
            "{shown} must refuse a merge the user has not concluded — both acting classes \
             adjudicate this member\n{text}",
        );
        assert!(
            text.contains(&format!(
                "blocking · {}",
                PostureMember::OperationInProgress.code()
            )),
            "{shown}: the refusal carries the member's own code\n{text}",
        );
        let routes: Vec<&str> = text
            .lines()
            .filter(|line| line.trim_start().starts_with("route: "))
            .collect();
        assert_eq!(
            routes.len(),
            1,
            "{shown}: a posture refusal prints exactly one route\n{text}",
        );
        assert!(
            routes[0].contains("git merge --abort"),
            "{shown}: the route names the git command that concludes THIS operation — a \
             posture is a state the user can resolve, so the route is theirs to run; got: \
             {}",
            routes[0],
        );
    }
    assert!(
        driven >= 2,
        "the classification must carry acting rows of both classes for this arm to mean \
         anything; drove {driven}",
    );

    assert!(
        repo.join(".git").join("MERGE_HEAD").exists(),
        "after every acting door has been handed a repository mid-merge, the merge is still \
         in progress — a door that refuses to conclude someone else's merge may not \
         half-conclude it",
    );
    assert_eq!(git(repo, &["rev-parse", "HEAD"]), head_before);
    assert_eq!(git(repo, &["rev-list", "--count", "--all"]), commits_before);
    assert_eq!(
        fs::read_to_string(repo.join("conflict.txt")).expect("read conflict"),
        conflict_before,
        "and the conflicted bytes the user still has to resolve are untouched",
    );
}

/// **Arm 2, second half** — the *partition*: the two acting classes owe different things,
/// and a mover that refused a detached or unborn `HEAD` would be adjudicating a state that
/// is none of its business.
///
/// Each half runs in the state where it mutates nothing: on a detached and on an unborn
/// `HEAD` every commit-on-behalf door refuses (minus its own stated exemptions), so the
/// sweep is non-mutating by construction; each mover gets its own repository, because a
/// mover that *proceeds* is doing the thing it exists to do.
#[test]
fn the_two_acting_classes_owe_different_members_of_the_family() {
    for member in [PostureMember::HeadDetached, PostureMember::HeadUnborn] {
        let (repo, home) = posture_repo(member);
        let (repo, home) = (repo.path(), home.path());
        let head_before = git_try(repo, &["rev-parse", "HEAD"]);
        let mut refused = 0usize;
        let mut exempted = 0usize;
        for row in BEHALF_DOORS {
            if !matches!(row.acts, ActsOnBehalf::CommitsOnBehalf { .. }) {
                continue;
            }
            let Some(argv) = acting_argv(&row.acts) else {
                continue;
            };
            let shown = format!("jigc {} under {}", row.door.join(" "), member.code());
            if !adjudicates(&row.acts, member) {
                // The one stated `Exempt(reason)` row, driven as its own named cell below —
                // running it here would birth `HEAD` and destroy the state for the rest.
                exempted += 1;
                continue;
            }
            refused += 1;
            let out = jigc(
                repo,
                home,
                &argv.iter().map(String::as_str).collect::<Vec<_>>(),
            );
            let text = surface(&out);
            assert!(
                !out.status.success() && text.contains(&format!("blocking · {}", member.code())),
                "{shown} must refuse with the member's own code\n{text}",
            );
        }
        assert!(
            refused > 0,
            "the commit class must be driven under {member:?}"
        );
        assert_eq!(
            git_try(repo, &["rev-parse", "HEAD"]).stdout,
            head_before.stdout,
            "no refusing door moved HEAD under {member:?}",
        );
        if member == PostureMember::HeadUnborn {
            assert_eq!(
                exempted, 1,
                "exactly one row states an exemption from an unborn HEAD — a hole in a \
                 family is a decision or it is a bug, and the registry is where a door says \
                 which",
            );
        }

        // The movers, each in its own repository: this member is none of their business.
        for row in BEHALF_DOORS {
            if !matches!(row.acts, ActsOnBehalf::MovesOnBehalf { .. }) {
                continue;
            }
            let argv = acting_argv(&row.acts).expect("a mover row carries a runnable argv");
            let (mover_repo, mover_home) = posture_repo(member);
            let out = jigc(
                mover_repo.path(),
                mover_home.path(),
                &argv.iter().map(String::as_str).collect::<Vec<_>>(),
            );
            let text = surface(&out);
            for other in PostureMember::ALL {
                assert!(
                    !text.contains(other.code()),
                    "`jigc {}` is a mover: a `git mv` lands in the index and which commit it \
                     joins stays the user's to decide, so `{}` is none of its business\n{text}",
                    row.door.join(" "),
                    other.code(),
                );
            }
        }
    }
}

/// **Arm 2's three named cells** — the stated exemption honoured end to end, the flag whose
/// consent is *carry my staged work* and never *conclude someone else's merge*, and the
/// linked worktree that is **typed**, never sniffed.
#[test]
fn the_exemption_the_flag_and_the_dedicated_worktree_each_answer_as_declared() {
    // (a) `jigc setup` on an unborn HEAD still installs and still commits — the QUICKSTART
    //     on-ramp, which applying the family uniformly would have turned into a refusal.
    let (repo, home) = posture_repo(PostureMember::HeadUnborn);
    let out = jigc(repo.path(), home.path(), &["setup"]);
    let text = surface(&out);
    assert!(
        out.status.success(),
        "the registry's one stated `Exempt(reason)` row is honoured at the door\n{text}",
    );
    assert!(
        repo.path().join(".jigc").join("AGENT.md").is_file(),
        "and the install actually ran\n{text}",
    );
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]).trim(),
        "1",
        "setup owns minting the first commit on a repository that has none\n{text}",
    );

    // (b) `--carry-staged` does not conclude a merge. Its consent is over the caller's own
    //     staged set, and a flag that silently widened to a repository state would be the
    //     ambush the wave is named against.
    let (repo, home) = posture_repo(PostureMember::OperationInProgress);
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", POSTURE_ID, "--carry-staged"],
    );
    let text = surface(&out);
    assert!(
        !out.status.success() && text.contains(PostureMember::OperationInProgress.code()),
        "`--carry-staged` carries staged work; it does not conclude someone else's \
         merge\n{text}",
    );
    assert!(
        repo.path().join(".git").join("MERGE_HEAD").exists(),
        "and MERGE_HEAD is still there\n{text}",
    );

    // (c) A dedicated worktree — the fan-out's own working area — is a repository with an
    //     ordinary attached HEAD, and the probe answers about the state rather than about
    //     the `.git` file's shape. A committing door runs there clean.
    let (repo, home) = born_repo("worktree");
    // The linked worktree lives in its own throwaway root, never beside the repo in the
    // shared temp directory: a sibling path there outlives the `TempDir` that made it and
    // the next run of this arm meets `fatal: '../wt' already exists`.
    let area = TempDir::new("linked-worktree");
    let worktree = area.path().join("wt");
    git(
        repo.path(),
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "side",
            &worktree.to_string_lossy(),
        ],
    );
    let out = jigc(&worktree, home.path(), &["setup"]);
    let text = surface(&out);
    assert!(
        out.status.success(),
        "a linked worktree is a posture the family does not carry — the probe is typed, \
         never sniffed off the shape of `.git`\n{text}",
    );
    for member in PostureMember::ALL {
        assert!(
            !text.contains(member.code()),
            "and no member of the family is raised in it\n{text}",
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — `setup`'s own install pathspec (a DERIVATION, stated as one)
// ═════════════════════════════════════════════════════════════════════════════

/// The door's own blocking code.
const DIRTY_INSTALL: &str = "setup.dirty-install-path";

/// The user's marker — bytes no install writes, so *"did this survive the install?"* is a
/// question about the door's behaviour and never about a coincidence.
const USER_MARKER: &str = "FLOW52-USER-BYTES";

/// The one derived path the rule below does **not** decide, with its reason.
///
/// `.claude/skills/jigc/SKILL.md` is jigc's **own owned artifact**: M48 Increment 10 settled
/// that jigc replaces its copy on update and **refuses to clobber one a user has edited**,
/// so a user edit there survives the install without ever being a candidate for the install
/// commit — which is a decision about ownership, not an outcome of the guard's predicate.
/// `setup_install_pathspec_guard.rs` cell (11) owns it as its own cell; it is named here so
/// the derivation states its exception rather than quietly special-casing a path.
const OWNED_ARTIFACT: (&str, &str) = (
    ".claude/skills/jigc/SKILL.md",
    "jigc's own guide artifact — jigc owns replacing it and refuses to clobber a user-edited \
     copy (M48 Increment 10), so an edit there is never a subject of the install commit",
);

/// The pathspec `jigc setup`'s install commit actually carries, **read back from the
/// binary** — the derivation's subject. Never a written-down list: a ninth install path
/// joins this arm the moment the door starts committing it.
fn derived_install_pathspec(repo: &Path, home: &Path) -> Vec<String> {
    let out = jigc(repo, home, &["setup"]);
    assert!(
        out.status.success(),
        "the derivation starts from a clean install\n{}",
        surface(&out),
    );
    git(repo, &["show", "--name-only", "--pretty=format:", "HEAD"])
        .split_whitespace()
        .map(str::to_owned)
        .collect()
}

/// Put the user's marker into `path` in a shape the file's own format survives — a JSON
/// object gains a key, everything else gains a comment line. A marker that broke the file's
/// syntax would make the install answer about a parse fault instead of about the bytes.
fn plant_user_bytes(repo: &Path, path: &str) {
    let full = repo.join(path);
    let body = fs::read_to_string(&full).unwrap_or_default();
    let planted = if path.ends_with(".json") {
        body.replacen('{', &format!("{{\"flow52Marker\": \"{USER_MARKER}\","), 1)
    } else {
        format!("{body}\n# {USER_MARKER}\n")
    };
    fs::write(&full, planted).expect("plant the user's bytes");
}

/// **Arm 3** — every path the install commit carries, decided by one rule.
///
/// **The kind of set: a derivation, stated as one.** The subject is not a registry and there
/// is no constant to read: it is *the pathspec `jigc setup` settles on*, recovered from the
/// binary by asking git what the install commit named. The predicate is a **git query** —
/// for each of those paths, do its index-or-worktree bytes differ from `HEAD` before the
/// first write? — which is why the guard could never have been a per-path ownership list
/// that a ninth install path gets forgotten out of.
///
/// **The rule this arm drives, over the whole derived set:** *a path carrying bytes that
/// were in no commit before the run refuses, and the refusal leaves those bytes on disk.*
/// [`OWNED_ARTIFACT`] is the derivation's one stated exception, with its reason.
///
/// **The rule this arm used to drive is struck with its falsifying datum** (the M51
/// completion audit): it was *"bytes that survive the install would ride the commit, so the
/// door refuses; bytes the install regenerates are jigc's own, and the door proceeds"* — and
/// the `else` branch asserted **exit 0 over destroyed bytes**. Driven at `da5173a1`, planting
/// prose in `.jigc/AGENT.md` and re-running `setup` left the adopter's lines in no git object
/// at exit 0 with `git status --short` empty, and this arm graded that green. *"What would be
/// committed is jigc's canonical bytes"* was true and beside the point: the loss happened
/// before the commit was considered. The door now asks **before the first write**, so the
/// same plant refuses with the bytes intact at every path but the stated exception.
///
/// **What this adds over `setup_install_pathspec_guard.rs`.** That suite drives twenty-nine
/// cells and owns the route, the re-run, the footprint record, the unborn exemption —
/// held, since the rc.24 fix pass, to each member's declared writer — and the per-path
/// disposition fence. This arm derives the subject from the
/// binary instead of listing it, and asserts that the *same* rule decides **every** derived
/// path — so the day a ninth path joins the install commit, this arm asks it the question
/// rather than passing over it.
///
/// **Red at the wave's base** (`DECISIONS.md` → M51 Increment 3, driven at `442bbd2f`): on a
/// repo whose `CLAUDE.md` carried an edit that was **never staged at all**, `jigc setup`
/// exited 0, `git show HEAD:CLAUDE.md` carried the user's line, and `git status --short` was
/// **empty** — so nothing prompted recovery.
#[test]
fn every_derived_install_path_is_decided_by_the_same_rule() {
    let (probe_repo, probe_home) = born_repo("pathspec-derive");
    let pathspec = derived_install_pathspec(probe_repo.path(), probe_home.path());
    assert!(
        pathspec.len() >= 2 && pathspec.iter().any(|p| p == "CLAUDE.md"),
        "the derivation must recover a real install pathspec from the binary; got \
         {pathspec:?}",
    );

    for path in &pathspec {
        let (repo, home) = born_repo("pathspec-cell");
        let (repo, home) = (repo.path(), home.path());
        assert!(jigc(repo, home, &["setup"]).status.success());
        assert!(
            repo.join(path).exists(),
            "`{path}` rode the install commit, so the install wrote it",
        );
        plant_user_bytes(repo, path);

        let out = jigc(repo, home, &["setup"]);
        let text = surface(&out);

        if path == OWNED_ARTIFACT.0 {
            assert!(
                out.status.success(),
                "`{path}` is the derivation's stated exception — {} — so the door proceeds \
                 whatever became of the bytes\n{text}",
                OWNED_ARTIFACT.1,
            );
            continue;
        }

        assert!(
            !out.status.success(),
            "`{path}` carried bytes that were in no commit before the run, so the install \
             commit would sweep them — the door must refuse\n{text}",
        );
        assert!(
            text.contains(DIRTY_INSTALL) && text.contains(path.as_str()),
            "`{path}`: the refusal carries the door's own code and names the path it \
             refuses over\n{text}",
        );
        assert!(
            fs::read_to_string(repo.join(path))
                .unwrap_or_default()
                .contains(USER_MARKER),
            "`{path}`: and the user's bytes are still on disk — the question is asked \
             BEFORE the first write, which is the only way the refusal's own sentence \
             (`every path listed above still has its pre-run bytes there`) can be true at \
             a path the install rewrites whole",
        );
        assert!(
            !git(repo, &["show", &format!("HEAD:{path}")]).contains(USER_MARKER),
            "`{path}`: the user's bytes rode no commit",
        );
    }
}

/// **Arm 3's done-picture** — §1's driven red in one repository, and the three clean paths
/// beside it, so the guard is asserted in **both** directions: what it refuses, and what it
/// must not.
#[test]
fn the_install_commit_refuses_over_bytes_it_did_not_write_and_stays_inert_otherwise() {
    let (repo, home) = born_repo("install-red");
    let (repo, home) = (repo.path(), home.path());
    fs::write(repo.join("CLAUDE.md"), "user line one\n").expect("write CLAUDE.md");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-qm", "add CLAUDE.md"]);
    // The driven cell: an edit that was never staged at all.
    fs::write(
        repo.join("CLAUDE.md"),
        format!("user line one\n{USER_MARKER}\n"),
    )
    .expect("edit CLAUDE.md");
    // …and an unrelated staged file, which the door has no business touching either way.
    fs::write(repo.join("feature.txt"), "my feature\n").expect("write feature.txt");
    git(repo, &["add", "feature.txt"]);
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    let text = surface(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a dirty install path refuses the install commit\n{text}",
    );
    assert!(
        text.contains(DIRTY_INSTALL) && text.contains("CLAUDE.md"),
        "the refusal carries the boundary's own code and names its path\n{text}",
    );
    assert!(
        text.contains("jigc setup --force"),
        "the route names `--force` as the single consent\n{text}",
    );
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "no install commit was made",
    );
    assert!(
        !repo.join(".jigc").join("AGENT.md").exists(),
        "the question is asked BEFORE the first write, so the refusal installed nothing",
    );
    let staged = git(repo, &["diff", "--cached", "--name-only"]);
    assert!(
        staged.lines().all(|line| line == "feature.txt"),
        "and staged nothing of its own — only what the user had already staged; got:\n{staged}",
    );
    assert!(
        fs::read_to_string(repo.join("CLAUDE.md"))
            .expect("read CLAUDE.md")
            .contains(USER_MARKER),
        "the adopter's bytes are still on disk",
    );
    assert_eq!(
        fs::read_to_string(repo.join("feature.txt")).expect("read feature.txt"),
        "my feature\n",
        "the unrelated staged file is untouched — the guard composed into a context that \
         omits its target is inert",
    );

    // The three clean paths: a fresh install, a re-run over an unchanged install, and the
    // read sweep an adopter runs after an upgrade.
    let (clean, clean_home) = born_repo("install-clean");
    let (clean, clean_home) = (clean.path(), clean_home.path());
    assert!(jigc(clean, clean_home, &["setup"]).status.success());
    let after_install = git(clean, &["rev-parse", "HEAD"]);
    let rerun = jigc(clean, clean_home, &["setup"]);
    assert!(
        rerun.status.success(),
        "a re-run over an unchanged install is clean\n{}",
        surface(&rerun),
    );
    assert_eq!(
        git(clean, &["rev-parse", "HEAD"]),
        after_install,
        "and makes no second commit",
    );
    let upgrade = jigc(clean, clean_home, &["upgrade"]);
    assert!(
        upgrade.status.success(),
        "and the upgrade sweep runs clean over it\n{}",
        surface(&upgrade),
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — the config-layer CAS pre-image
//         (a MANUFACTURED SHAPE SPACE, and it says so)
// ═════════════════════════════════════════════════════════════════════════════

/// The blocking code a restore that meets bytes jigc did not write mints.
const ROLLBACK_CONFLICT: &str = "finalize.rollback-conflict";

/// Bytes no real `jigc` build stamps, so the stamp refresh is guaranteed to write something
/// different and the pre-image entry is live rather than a byte no-op.
const STALE_STAMP: &str = "0.0.0-flow52-stale-stamp\n";

/// The user's own uncommitted lines in `.jigc/.gitignore` — appended **after** the commit,
/// so they exist in no git object. That is what makes them unrecoverable if a rollback
/// overwrites them, and it is the loss this arm walks.
const PRIVATE_LINES: &str = "# my own\nscratch-notes/\n";

/// The committed ignore set, one canonical entry short — so `gitignore::ensure` genuinely
/// amends the file inside the transaction.
const COMMITTED_ENTRIES: &str = "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\n";

/// What the injecting hook writes into `.jigc/.gitignore` before it refuses.
const CONCURRENT_LINE: &str = "# written while finalize was running";

/// Whether the hook rewrites jigc's two files before refusing — the **race** axis, which no
/// code-side set carries and which is why this space is manufactured.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Race {
    /// Nothing touched the files after jigc wrote them — the compare-and-swap holds.
    Untouched,
    /// The hook rewrote both files after jigc wrote them — the compare-and-swap fails.
    ConcurrentEdit,
}

/// One refused finalize, and everything the arm needs to read back off it.
struct RefusedFinalize {
    repo: TempDir,
    #[allow(dead_code)]
    home: TempDir,
    rendered: String,
    status_before: String,
}

/// Build the fixture, mint a task, fill its commit doc, install the refusing hook, and drive
/// `jigc task finalize` into the refusal.
fn refused_finalize(race: Race) -> RefusedFinalize {
    let repo = TempDir::new("preimage");
    let home = TempDir::new("preimage-home");
    let (r, h) = (repo.path(), home.path());
    git(r, &["init", "-q", "-b", "main", "."]);
    git(r, &["config", "user.email", "test@example.com"]);
    git(r, &["config", "user.name", "Test"]);
    fs::write(r.join("README.md"), "hello\n").expect("write README");
    fs::create_dir_all(r.join(".jigc").join("config")).expect("create the project layer");
    fs::write(r.join(".jigc").join("version"), STALE_STAMP).expect("write the stale stamp");
    fs::write(r.join(".jigc").join(".gitignore"), COMMITTED_ENTRIES).expect("write the ignore");
    git(
        r,
        &["add", "README.md", ".jigc/version", ".jigc/.gitignore"],
    );
    git(r, &["commit", "-qm", "initial"]);
    // The user's own lines land AFTER the commit — uncommitted, in no git object.
    fs::write(
        r.join(".jigc").join(".gitignore"),
        format!("{COMMITTED_ENTRIES}{PRIVATE_LINES}"),
    )
    .expect("append the private lines");

    let started = jigc(
        r,
        h,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in one node",
            "--format",
            "json",
        ],
    );
    assert!(
        started.status.success(),
        "the arm mints its own task\n{}",
        surface(&started),
    );
    let task = json(&String::from_utf8_lossy(&started.stdout))["task"]
        .as_str()
        .expect("`jigc start --format json` names the task it minted")
        .to_owned();

    for (addr, value) in [("#type", "feat"), ("#scope", "cache")] {
        let out = jigc(
            r,
            h,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}{addr}"),
                "--value",
                value,
            ],
        );
        assert!(out.status.success(), "fill {addr}\n{}", surface(&out));
    }
    for (addr, prose) in [
        ("#summary", "change the cache"),
        ("#body", "A cache change."),
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args([
                "doc",
                "set-slot",
                &format!("commit:{task}{addr}"),
                "--from-file",
                "-",
            ])
            .current_dir(r)
            .env("HOME", h)
            .env_remove("JIGC_PACK_DIR")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                crate::support::child_stdin::feed(&mut child, prose.as_bytes());
                child.wait_with_output()
            })
            .expect("drive set-slot");
        assert!(out.status.success(), "fill {addr}\n{}", surface(&out));
    }

    // A real code change in the index, so the finalize reaches the commit phase rather than
    // blocking earlier on `finalize.nothing-staged`.
    fs::write(r.join("README.md"), "hello world\n").expect("edit README");
    git(r, &["add", "README.md"]);

    let edit = match race {
        Race::Untouched => String::new(),
        Race::ConcurrentEdit => format!(
            "printf '{CONCURRENT_LINE}\\n' >> .jigc/.gitignore\n\
             printf 'concurrent-stamp\\n' > .jigc/version\n"
        ),
    };
    let hook = r.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        format!("#!/bin/sh\n{edit}echo 'rejected by flow 52' 1>&2\nexit 1\n"),
    )
    .expect("write the pre-commit hook");
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&hook, perms).expect("chmod the hook");

    let status_before = git(r, &["status", "--porcelain"]);
    let out = jigc(r, h, &["task", "finalize", &task]);
    let rendered = surface(&out);
    let _ = fs::remove_file(&hook);
    assert!(
        !out.status.success(),
        "the injector must make the finalize exit non-zero\n{rendered}",
    );
    assert_eq!(
        git(r, &["rev-list", "--count", "HEAD"]).trim(),
        "1",
        "a refused finalize creates no commit\n{rendered}",
    );

    RefusedFinalize {
        repo,
        home,
        rendered,
        status_before,
    }
}

/// **Arm 4** — a refused transaction puts back the two config-layer files it rewrote, and
/// does not overwrite a concurrent edit doing it.
///
/// **The kind of set: a manufactured shape space, and this arm says so.** `{stage failure,
/// hook rejection} × {unchanged since jigc's post-write image, concurrently edited}` is read
/// off no registry: the failure points are **decided** (which of the closure's many `?`s are
/// worth driving), and the second axis is a **race**, which no code-side set carries. The
/// alternative to a manufactured space that says so is a single happy cell that looks like a
/// swept axis — M49's `spec` lesson, one wave over.
///
/// **What this adds over `config_layer_preimage.rs`.** That suite owns all four cells, both
/// injectors, the absent-pre-image arm, the never-written arm and the pathspec totality.
/// This arm walks the **adopter's loss** the space was found by, in one repository: a
/// private, uncommitted line in `.jigc/.gitignore` and a rejecting `pre-commit`. It asserts
/// the property no per-cell check can see — the refused run leaves the worktree **exactly**
/// as it found it, so the door's *"nothing was committed"* is true of the disk too — and, on
/// the raced run, that **both** versions survive.
///
/// **Red at the wave's base** (`DECISIONS.md` → M51 Increment 4, driven at `92755635`): four
/// index axes restored the *index* and the M45 audit's own row said *"worktree untouched"*
/// about the mechanism, so a hook-rejected finalize left ` M .jigc/.gitignore` and
/// ` M .jigc/version` on disk while telling the operator *"nothing was committed"*.
#[test]
fn a_refused_transaction_restores_its_config_layer_writes_without_overwriting_a_concurrent_edit() {
    // ── The headline cell: nothing raced, so the compare-and-swap holds. ──
    let driven = refused_finalize(Race::Untouched);
    let repo = driven.repo.path();
    let ignore = fs::read_to_string(repo.join(".jigc").join(".gitignore")).expect("read ignore");
    assert!(
        ignore.contains("scratch-notes/"),
        "the user's private line exists in no git object — a rollback that dropped it would \
         destroy bytes nothing can recover\n{}",
        driven.rendered,
    );
    assert_eq!(
        fs::read_to_string(repo.join(".jigc").join("version")).expect("read stamp"),
        STALE_STAMP,
        "the stamp is back to the bytes the commit carries",
    );
    assert_eq!(
        git(repo, &["status", "--porcelain"]),
        driven.status_before,
        "the refused run leaves the worktree exactly as it found it — the door's `nothing \
         was committed` is true of the disk too\n{}",
        driven.rendered,
    );
    assert!(
        !driven.rendered.contains(ROLLBACK_CONFLICT),
        "an unraced restore is not a conflict\n{}",
        driven.rendered,
    );

    // ── The raced cell: the restore meets bytes jigc did not write, and overwrites none. ──
    let raced = refused_finalize(Race::ConcurrentEdit);
    let repo = raced.repo.path();
    let ignore = fs::read_to_string(repo.join(".jigc").join(".gitignore")).expect("read ignore");
    assert!(
        ignore.contains(CONCURRENT_LINE),
        "the concurrent editor's bytes are not overwritten — that is the SAME loss the \
         rollback exists to prevent, in the other direction\n{}",
        raced.rendered,
    );
    assert_eq!(
        fs::read_to_string(repo.join(".jigc").join("version")).expect("read stamp"),
        "concurrent-stamp\n",
        "and the raced stamp is the editor's too",
    );
    // A recursive walk: the park is keyed `.jigc/displaced/<door>/<identity>.pre-image.<nanos>`
    // (M52 Increment 5 / T2), so the door owns a directory and the entry's identity keeps its
    // own path inside it — which is what makes two populations sharing a basename tellable
    // apart. A flat listing would see one directory and call it one parked copy.
    let parked: BTreeSet<PathBuf> = {
        let root = repo.join(".jigc").join("displaced");
        let mut found = BTreeSet::new();
        let mut stack = vec![root.clone()];
        while let Some(dir) = stack.pop() {
            for entry in fs::read_dir(&dir)
                .expect("the displaced workbench exists after a rollback conflict")
                .flatten()
            {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    found.insert(path);
                }
            }
        }
        found
    };
    assert_eq!(
        parked.len(),
        2,
        "both pre-images are parked rather than discarded — the user's line is recoverable \
         from `.jigc/displaced/`; got {parked:?}\n{}",
        raced.rendered,
    );
    assert!(
        parked.iter().any(|path| fs::read_to_string(path)
            .unwrap_or_default()
            .contains("scratch-notes/")),
        "and one of them is the ignore file's pre-image, private line and all",
    );
    assert_eq!(
        raced.rendered.matches(ROLLBACK_CONFLICT).count(),
        2,
        "one blocking `{ROLLBACK_CONFLICT}` per conflicted path, each naming both copies\n{}",
        raced.rendered,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — the `EnvelopeArm` registry (a CODE-SIDE REGISTRY, PRODUCTION-SIDE)
// ═════════════════════════════════════════════════════════════════════════════

/// The declared top-level key set of one `(leaf, arm)` row of [`ENVELOPE_ARMS`].
///
/// A row this arm names but the registry does not carry is a **hard panic**: the whole point
/// of driving against a production registry is that the expectation cannot be a literal a
/// reader typed twice.
fn declared_keys(path: &[&str], arm: &str) -> BTreeSet<String> {
    let row = ENVELOPE_ARMS
        .iter()
        .find(|row| row.path == path && row.arm == arm)
        .unwrap_or_else(|| {
            panic!(
                "`jigc {}` has no `{arm}` row in `cli::render::ENVELOPE_ARMS` — the registry \
                 is what declares a pinned document's key set, so an arm this suite drives \
                 must be declared there",
                path.join(" "),
            )
        });
    // Exhaustive on purpose: a seventh `ArmShape` cannot compile until this arm says
    // whether the shape carries a declarable key set, which is the whole question this
    // function answers. A `_` here would let a new root shape join the registry silently.
    match row.shape {
        ArmShape::Object(keys) => keys.iter().map(|key| (*key).to_string()).collect(),
        ArmShape::ArrayOf(_)
        | ArmShape::ArrayOfDataKeyed
        | ArmShape::ArrayOfScalars
        | ArmShape::Scalar
        | ArmShape::DataKeyed => panic!(
            "`jigc {}`'s `{arm}` declares a non-object root — this arm asserts key sets and \
             cannot speak for a scalar or an array root",
            path.join(" "),
        ),
    }
}

/// Assert a driven `--format json` document's top-level key set **equals** the one its row
/// declares — so a key the wave removed cannot come back, and one it added cannot quietly
/// leave, without the registry saying so first.
///
/// **The stream is the row's too.** [`cli::render::ArmOutcome`] names it — a success and an
/// adjudication ride stdout, a reject rides stderr with stdout empty — so a suite that read
/// one stream for every arm would be asserting a stream discipline the registry does not
/// declare, and would miss the arm whose whole identity is the funnel it entered.
fn assert_wire(out: &Output, path: &[&str], arm: &str) -> Value {
    let text = surface(out);
    let row = ENVELOPE_ARMS
        .iter()
        .find(|row| row.path == path && row.arm == arm)
        .expect("declared_keys panics first for an undeclared row");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let payload = match row.outcome {
        ArmOutcome::Reject => {
            assert!(
                stdout.trim().is_empty(),
                "`jigc {}` [{arm}] is a REJECT arm: stdout carries no document\n{text}",
                path.join(" "),
            );
            stderr
        }
        ArmOutcome::Success | ArmOutcome::Adjudicated(_) => stdout,
    };
    let document = json(&payload);
    assert_eq!(
        top_level_keys(&document),
        declared_keys(path, arm),
        "`jigc {}` [{arm}]: the driven key set must equal the registry's declared one\n{text}",
        path.join(" "),
    );
    document
}

/// **Arm 5** — the seven wire changes the wave made, each asserted against the registry that
/// declares them.
///
/// **The kind of set: a code-side registry, and a *production-side* one.**
/// [`ENVELOPE_ARMS`] is what `cli::render` answers from — one arm per distinct `--format
/// json` key set over all forty-seven leaves, part-derived from the result enums' own
/// compile-fenced arm-name tables, the rest hand-enumerated with stated reasons — so it is
/// not a census of what the binary happened to emit. (The row count is deliberately not
/// restated here: it is not fenced, and M52 Increment 1 / T4 moved it when `jigc doc show`'s
/// projection space was driven. The **leaf** count is fenced, by proof 1's `>= 47` floor.) Its four proofs (every leaf has ≥1 row; every production arm has exactly one
/// row; every row is driven; the driven key set equals the declared one) live with
/// `format_json_success_axis.rs`, the one suite that already drives every leaf to a real
/// success.
///
/// **What this arm adds:** the wave's own deltas, driven as a done-picture. The **four
/// deletes** — `installed`, `uninstalled`, `review`, and `hook_output` on a `VerbKind::Read`
/// verb that commits nothing and therefore can never have a hook speak into it — are the
/// pre-1.0 additive-key window closing in the one direction that is not additive, so each is
/// asserted **absent by name** as well as by set equality. The three additions are asserted
/// **present by name** for the same reason.
///
/// **Red at the wave's base** (`DECISIONS.md` → the M51 Increment 5 entries): three constants
/// duplicated the exit code on the wire, a read verb carried a commit-hook key,
/// `ConfigAck::Set` was silent about a relocation it had just performed, and
/// `task finalize --dry-run` withheld the findings of the door it forecasts.
#[test]
fn the_waves_seven_wire_changes_each_match_the_registry_that_declares_them() {
    // ── (1) and (2): the two setup-family deletes, in one repository's own lifetime. ──
    let (repo, home) = born_repo("envelope-setup");
    let (repo, home) = (repo.path(), home.path());
    let installed = jigc(repo, home, &["setup", "--format", "json"]);
    assert!(installed.status.success(), "{}", surface(&installed));
    let document = assert_wire(&installed, &["setup"], "Installed");
    assert!(
        !document
            .as_object()
            .expect("object")
            .contains_key("installed"),
        "`installed` duplicated the exit code on the wire and left it at M51 Increment 5",
    );
    let torn_down = jigc(repo, home, &["uninstall", "--format", "json"]);
    assert!(torn_down.status.success(), "{}", surface(&torn_down));
    let document = assert_wire(&torn_down, &["uninstall"], "TornDown");
    assert!(
        !document
            .as_object()
            .expect("object")
            .contains_key("uninstalled"),
        "`uninstalled` is the same duplication, one door over",
    );

    // ── (3): `migrate`'s composed output carries no `review` key. ──
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(
        corpus.repo().join("legacy-changelog.md"),
        "# Change Log\n\n## v1\n\n- did a thing\n",
    )
    .expect("write the foreign source");
    corpus.git(&["add", "-A"]);
    corpus.git(&["commit", "-qm", "the foreign changelog"]);
    let composed = corpus.jigc(&[
        "migrate",
        "legacy-changelog.md",
        "--as",
        "changelog",
        "--format",
        "json",
    ]);
    assert!(composed.status.success(), "{}", surface(&composed));
    let document = assert_wire(&composed, &["migrate"], "Composed");
    assert!(
        !document.as_object().expect("object").contains_key("review"),
        "`review` was the third exit-code duplicate",
    );

    // ── (4): the read verb that commits nothing carries no commit-hook key. ──
    let milestones = TrialCorpus::build(State::Fresh);
    let minted = milestones.jigc_ok(&["milestone", "create", "Envelope Milestone"]);
    let milestone = minted
        .split_whitespace()
        .find(|token| token.starts_with("envelope-"))
        .unwrap_or("envelope-milestone")
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-')
        .to_owned();
    let listing = milestones.jigc(&["milestone", "list-tasks", &milestone, "--format", "json"]);
    assert!(listing.status.success(), "{}", surface(&listing));
    let document = assert_wire(&listing, &["milestone", "list-tasks"], "Listing");
    assert!(
        !document
            .as_object()
            .expect("object")
            .contains_key("hook_output"),
        "a `VerbKind::Read` verb commits nothing, so no hook can ever speak into it",
    );

    // ── (5): `ConfigAck::Set` names the relocation it performed. ──
    let knobs = TrialCorpus::build(State::Fresh);
    let ack = knobs.jigc(&[
        "config",
        "set",
        "docs-root",
        "documents",
        "--format",
        "json",
    ]);
    assert!(ack.status.success(), "{}", surface(&ack));
    let document = assert_wire(&ack, &["config", "set"], "ConfigAck::Set");
    assert!(
        document
            .as_object()
            .expect("object")
            .contains_key("relocated"),
        "a knob whose landing MOVES managed docs says so on the wire",
    );

    // ── (6): the forecast carries the findings of the door it forecasts. ──
    let forecast_corpus = TrialCorpus::build(State::Fresh);
    let task = forecast_corpus.start_workflow("single-task", "forecast the finalize");
    fill_commit_doc(&forecast_corpus, &task);
    fs::write(forecast_corpus.repo().join("change.txt"), "a change\n").expect("write a change");
    forecast_corpus.git(&["add", "change.txt"]);
    let forecast =
        forecast_corpus.jigc(&["task", "finalize", &task, "--dry-run", "--format", "json"]);
    let document = assert_wire(&forecast, &["task", "finalize"], "Forecast");
    assert!(
        document
            .as_object()
            .expect("object")
            .contains_key("findings"),
        "a forecast that withheld the findings of the door it forecasts was \
         information-free on the wave's #1-ranked v1 gate",
    );

    // ── (7) N15: the `--task` read miss names the copy it looked in. ──
    //
    //     A read under `--task` that finds nothing has to say WHICH copy it looked in.
    //     Until Increment 5 the two reads emitted byte-identical blocks: the staged arm
    //     said *"names no committed doc"* and routed without `--task`, so the route,
    //     followed exactly, served the other copy than the caller had asked for.
    //
    //     **[Re-pointed 2026-09-18 (M52 Increment 6, T2).** This arm drove
    //     `vision:wrong-slug --task <id>`, and that address no longer reaches either read
    //     arm: the nine `doc` doors refuse a non-canonical **fixed-identity** slug at their
    //     shared parse boundary (`store.fixed-identity`), *before* any copy is selected, so
    //     the assertion below could only have been kept by pinning a code the binary no
    //     longer emits there. Two consequences, stated rather than dropped. (1) The subject
    //     moves to a miss that **still** reaches the staged arm — a committed doc this task
    //     does not stage — where N15's property is live and is what is asserted. (2) The
    //     *"the route keeps `--task`"* half is **retired with its datum**: the singleton-slug
    //     staged block was its only producer, and this miss's route correctly names the
    //     **task-less** read, because the committed copy is the one that exists. The full
    //     reading, and both arms driven, live in
    //     `crates/cli/tests/staged_read_miss_arm.rs`.**]**
    let read_miss = TrialCorpus::build(State::CommittedSingletons);
    let task = read_miss.start_workflow("form-vision", "revise the vision");
    let missed = read_miss.jigc(&[
        "doc",
        "show",
        "roadmap:roadmap",
        "--task",
        &task,
        "--format",
        "json",
    ]);
    let text = surface(&missed);
    assert!(
        !missed.status.success(),
        "a read of a doc this task does not stage is a miss under `--task`\n{text}",
    );
    let document = assert_wire(&missed, &[], "Reject::Findings");
    let findings = document["findings"]
        .as_array()
        .expect("the findings envelope carries a findings array");
    let block = findings
        .iter()
        .find(|finding| finding["code"] == "store.not-staged")
        .unwrap_or_else(|| {
            panic!(
                "the miss is a typed finding on the pinned reject funnel, never a bare \
                 `error` key\n{text}"
            )
        });
    assert!(
        block["message"]
            .as_str()
            .is_some_and(|message| message.contains("not staged in this task")),
        "the staged arm names the copy it looked in — a block phrased for the committed \
         store under `--task` is a law-1 lie\n{text}",
    );
    assert!(
        block["route"]
            .as_str()
            .is_some_and(|route| route.contains("jigc doc show roadmap:roadmap")),
        "and its route names the copy that does exist, which is the committed one\n{text}",
    );
}

/// Author a task's `commit` doc through the shipped write verbs, so a finalize (or its
/// forecast) reaches the arm it is being driven for rather than the blocked one.
fn fill_commit_doc(corpus: &TrialCorpus, task: &str) {
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
        "--task",
        task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        "flow52",
        "--task",
        task,
    ]);
    corpus.set_slot(&format!("commit:{task}#summary"), task, "forecast the door");
    corpus.set_slot(
        &format!("commit:{task}#body"),
        task,
        "Driven by the flow 52 acceptance suite.",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 6 — `WORK_UNIT_ID_DOORS`, filtered to the unknown-id cell
//         (an EXISTING CODE-SIDE REGISTRY, filtered to one cell of its own axis)
// ═════════════════════════════════════════════════════════════════════════════

/// A well-formed work-unit id no work unit in the corpus carries.
const UNKNOWN_ID: &str = "no-such-work-unit";

/// The token that removed a repository at the M50 baseline — driven here beside the unknown
/// one, because the two answers must stay **different**: a grammar refusal handed to someone
/// who merely mistyped a live id would be a law-1 misdirection.
const MALFORMED_ID: &str = "../..";

/// The stable `(code, target)` pair an absent work unit projects, per family — read off the
/// crate that mints it, never re-spelled here.
fn unknown_key(arg: &str, id: &str) -> (String, String) {
    match arg {
        "task" | "id" => ("finalize.no-task".to_owned(), format!("task:{id}")),
        "milestone_id" => (
            engine::milestone::UNKNOWN_MILESTONE_CODE.to_owned(),
            format!("milestone:{id}"),
        ),
        other => panic!(
            "`{other}` is a work-unit-id argument with no family — a fourth member of the \
             id-carrying arguments owes this arm the answer its doors give"
        ),
    }
}

/// **Arm 6** — every door that takes a work-unit id, handed one that names nothing, then one
/// that is not an id at all, then a real task driven to a real commit.
///
/// **The kind of set: an existing code-side registry, filtered to one cell of its own token
/// axis.** [`WORK_UNIT_ID_DOORS`] is derived from the clap tree by the argument ids that
/// carry a work-unit id and ⇔-fenced against it, and each row carries its **own** runnable
/// argv — so this arm reads the registry and writes no cell list at all.
///
/// **What this adds over `work_unit_unknown_envelope.rs` and `work_unit_id_axis.rs`.** Those
/// own the per-family key, the route split at `discard`, and the three malformed cells with
/// their messages. This arm drives **both** cells at **every** door of **one** corpus and
/// then asserts the property neither can see: the two answers stay apart — an id that names
/// nothing is `finalize.no-task` / `milestone.unknown` and never the grammar code, and an
/// id that is not an id is the grammar code and never the other — and a task minted **after**
/// the whole sweep still finalizes to a **real commit**, so the doors that refuse those
/// tokens have not learned to refuse the ids jigc itself mints.
///
/// **Red at the wave's base** (`DECISIONS.md` → M51 Increment 6): every task-family door
/// answered `--format json` with the flattened single-key
/// `{"error": "no task `<id>` — …"}`, and a code inside a message is not a key; five of the
/// milestone family's eight answered a flattened `{"error": …}` with no code at all. A
/// driver keying on the `(code, target)` pair the contract pins got an answer at **none** of
/// the twenty-five.
#[test]
fn every_work_unit_id_door_answers_an_unknown_id_and_a_malformed_one_differently() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(
        corpus.repo().join(WORK_UNIT_ID_DOOR_PAYLOAD),
        "title: Axis\nsections: []\n",
    )
    .expect("plant the payload the two `--from-file` rows read");

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    for row in WORK_UNIT_ID_DOORS {
        let shown = row.door.join(" ");
        let argv = |id: &str| -> Vec<String> {
            let mut argv: Vec<String> = row
                .argv
                .iter()
                .map(|token| {
                    if *token == WORK_UNIT_ID_SLOT {
                        id.to_owned()
                    } else {
                        (*token).to_owned()
                    }
                })
                .collect();
            argv.push("--format".to_owned());
            argv.push("json".to_owned());
            argv
        };

        // The unknown cell: the findings envelope, carrying the family's own key.
        let unknown = argv(UNKNOWN_ID);
        let out = corpus.jigc(&unknown.iter().map(String::as_str).collect::<Vec<_>>());
        let text = surface(&out);
        assert!(
            !out.status.success(),
            "`jigc {shown}` with an id nothing carries must block\n{text}",
        );
        let document = assert_wire(&out, &[], "Reject::Findings");
        let (code, target) = unknown_key(row.arg, UNKNOWN_ID);
        let keys: Vec<Value> = document["findings"]
            .as_array()
            .expect("the findings envelope carries a findings array")
            .iter()
            .map(|finding| finding["key"].clone())
            .collect();
        assert!(
            keys.iter()
                .any(|key| key["code"] == code.as_str() && key["target"] == target.as_str()),
            "`jigc {shown}`: a driver keying on the stable `(code, target)` pair must get an \
             answer — expected `({code}, {target})`, got {keys:?}\n{text}",
        );
        assert!(
            !text.contains("work-unit.malformed-id"),
            "`jigc {shown}`: `{UNKNOWN_ID}` IS a well-formed id — answering the grammar here \
             would send someone who merely mistyped a live id to re-read a rule their value \
             already obeys\n{text}",
        );

        // The malformed cell, beside it: the grammar, and never the unknown-id code.
        let malformed = argv(MALFORMED_ID);
        let out = corpus.jigc(&malformed.iter().map(String::as_str).collect::<Vec<_>>());
        let text = surface(&out);
        assert!(
            !out.status.success() && text.contains("work-unit.malformed-id"),
            "`jigc {shown}` with `{MALFORMED_ID}` must refuse with the grammar code — this is \
             the token that resolved the repository root as a working area and removed \
             it\n{text}",
        );
        assert!(
            !text.contains(&code),
            "`jigc {shown}`: a token that is not an id never reaches the resolve that would \
             report it unknown\n{text}",
        );
        driven.insert(row.door.to_vec());
    }

    let registered: BTreeSet<Vec<&str>> = WORK_UNIT_ID_DOORS
        .iter()
        .map(|row| row.door.to_vec())
        .collect();
    assert_eq!(
        driven, registered,
        "every registered work-unit-id door is driven — a row with no cell is a failure, \
         never a skip",
    );

    // The over-firing guard, at the done-picture altitude: the ids jigc mints still land.
    assert!(
        corpus.repo().join(".git").is_dir(),
        "the repository is still here after the whole sweep",
    );
    let task = corpus.start_workflow("single-task", "close the id axis");
    fill_commit_doc(&corpus, &task);
    fs::write(corpus.repo().join("axis.txt"), "closed\n").expect("write the change");
    corpus.git(&["add", "axis.txt"]);
    let landed = corpus.jigc_ok(&["task", "finalize", &task]);
    assert!(
        landed.contains("finalized"),
        "a task minted after the sweep still finalizes to a real commit; got:\n{landed}",
    );
    assert_eq!(
        corpus
            .git(&["log", "-1", "--format=%s"])
            .trim()
            .split(':')
            .next()
            .unwrap_or_default(),
        "docs(flow52)",
        "and the commit is the one the task's own commit doc composed",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 7 — `ManifestKind::ALL` (the class's DEFINING CASE-SET, matched exhaustively)
// ═════════════════════════════════════════════════════════════════════════════

/// When a kind's fixture act has to run, relative to the mint.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    /// Before `jigc start` — the only phase from which a **carried-over** entry can exist.
    BeforeMint,
    /// After the task exists.
    AfterMint,
}

/// Which commit model's finalize a kind reaches the wire on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Model {
    /// The ordinary index commit — the `single-task` the arm mints.
    Index,
    /// The doc-only, path-scoped commit (M55; `design/finalize.md` → The doc-only arm) — the
    /// fixture report task the arm mints beside it, [`DOC_ONLY_WORKFLOW`].
    DocOnly,
}

/// A fixture code-less workflow composing the shipped `step:finalize-doc-only` — the only
/// commit model whose left-out list can carry `left-staged`. A project-layer definition over
/// the default `[dev ▸ methodology]` cascade (the composers the methodology pack ships land
/// at M55 Increment 8; until then a fixture is the arm's way in).
const DOC_ONLY_WORKFLOW: &str = "\
---
when: file one doc while other work is staged in the checkout
description: A fixture code-less report workflow — one idea doc, committed path-scoped.
usage: the flow 52 arm 7 cell for the doc-only forecast, reached by name.
creates-task: true
selectable: false
suppressed:
  reason: fixture-only — the doc-only manifest cell, reached by name
  expires: never
allows-create:
  - { type: idea, as: idea }
---
{{ include: step:author-commit }}
{{ include: step:finalize-doc-only }}
";

const DOC_ONLY_WORKFLOW_ID: &str = "flow52-report";

/// The path [`ManifestKind::LeftStaged`]'s act stages — named for nothing in the
/// vocabulary, so the agent text's mention of it is never mistaken for a tag.
const LEFT_STAGED_PATH: &str = "elsewhere.txt";

/// The fixture act that puts one manifest kind on the wire, and the commit model whose
/// finalize carries it there.
///
/// **Exhaustive by construction:** a new [`ManifestKind`] does not compile until someone
/// decides how a finalize is driven into producing it — which is the difference between a
/// case-set matched exhaustively and a list that happens to be right today.
fn manifest_act(kind: ManifestKind) -> (Model, Phase, fn(&TrialCorpus, &str)) {
    match kind {
        // A path staged in the checkout — for the ordinary task an `added`, and for the
        // doc-only task beside it a path its path-scoped commit leaves staged.
        ManifestKind::LeftStaged => (Model::DocOnly, Phase::AfterMint, |corpus, _task| {
            fs::write(
                corpus.repo().join(LEFT_STAGED_PATH),
                "staged for another task\n",
            )
            .expect("write the staged path");
            corpus.git(&["add", LEFT_STAGED_PATH]);
        }),
        ManifestKind::CarriedOver => (Model::Index, Phase::BeforeMint, |corpus, _task| {
            fs::write(
                corpus.repo().join("carried.txt"),
                "staged before the mint\n",
            )
            .expect("write the carryover");
            corpus.git(&["add", "carried.txt"]);
        }),
        ManifestKind::Promoted => (Model::Index, Phase::AfterMint, |corpus, task| {
            corpus.jigc_ok(&[
                "doc",
                "create",
                "adr",
                "--title",
                "Flow 52 Decision",
                "--task",
                task,
            ]);
            corpus.jigc_ok(&[
                "doc",
                "set-field",
                "adr:flow-52-decision#status",
                "--value",
                "accepted",
                "--task",
                task,
            ]);
            for slot in ["context", "options", "decision", "consequences"] {
                corpus.set_slot(
                    &format!("adr:flow-52-decision#{slot}"),
                    task,
                    "Recorded by the flow 52 acceptance suite.",
                );
            }
        }),
        ManifestKind::Modified => (Model::Index, Phase::AfterMint, |corpus, _task| {
            fs::write(corpus.repo().join("keep.md"), "edited\n").expect("edit the tracked file");
            corpus.git(&["add", "keep.md"]);
        }),
        ManifestKind::Deleted => (Model::Index, Phase::AfterMint, |corpus, _task| {
            corpus.git(&["rm", "-q", "doomed.md"]);
        }),
        ManifestKind::Added => (Model::Index, Phase::AfterMint, |corpus, _task| {
            fs::write(corpus.repo().join("added.txt"), "a new file\n").expect("write the new file");
            corpus.git(&["add", "added.txt"]);
        }),
        ManifestKind::Untracked => (Model::Index, Phase::AfterMint, |corpus, _task| {
            fs::write(corpus.repo().join("stray.txt"), "never staged\n").expect("write the stray");
        }),
    }
}

/// Author the doc-only task's one `idea` and its commit doc, so its forecast has a path set.
fn author_report(corpus: &TrialCorpus, task: &str) {
    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "idea",
            "--title",
            "Flow 52 Report",
            "--task",
            task,
        ])
        .trim()
        .to_owned();
    corpus.set_field(&format!("{address}#trigger"), task, "a report comes back");
    corpus.set_slot(
        &format!("{address}#description"),
        task,
        "Filed by the flow 52 acceptance suite.",
    );
    fill_commit_doc(corpus, task);
}

/// **Arm 7** — every kind of the manifest vocabulary, on one wire, from one checkout.
///
/// **The kind of set: the class's defining case-set, matched exhaustively.**
/// [`ManifestKind::ALL`] is the whole value space the JSON `kind` key can carry, and
/// [`manifest_act`] maps each variant through a compiler-checked `match` — so a new kind
/// cannot be added without deciding what act produces it here, and on which commit model.
///
/// **What this adds over `count_fences.rs` and `finalize_manifest.rs`.** `count_fences.rs`
/// fences the **numerals prose states** about sets the code can move — the committing-door
/// count, the error-code mirror, the manifest-listed doctype totals — reading the registry
/// and asserting the prose; `finalize_manifest.rs` owns the manifest's own arms and the
/// dry-run forecast's honesty. Neither puts the **whole vocabulary on the wire at once**.
/// This arm does, on **both** surfaces the tag reaches — the agent text's manifest lines and
/// the pinned `--format json` `kind` values — so a kind that is reachable only in principle
/// reddens here.
///
/// **Two finalizes, because one kind lives on another commit model (M55).** `left-staged`
/// tags a staged path the **doc-only** arm's path-scoped commit leaves where it is; the
/// ordinary commit is the index and takes every staged path, so no index finalize can carry
/// it. The arm therefore mints a fixture doc-only task beside the ordinary one, in the same
/// checkout over the same staged state, and drives both forecasts: the vocabulary is their
/// union, and each kind must appear on the forecast of the model [`manifest_act`] names.
///
/// The partition [`ManifestKind::in_commit`] states is asserted as an *outcome*, not
/// restated: `untracked` and `left-staged` tag a **left-out** file and never a committed one,
/// which is why the vocabulary has two true sentences (`design/finalize.md` names the
/// committed set's five; `design/command-output-contract.md` the JSON value space's seven).
#[test]
fn every_manifest_kind_reaches_the_wire_from_one_finalize() {
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(corpus.repo().join("keep.md"), "kept\n").expect("write the tracked file");
    fs::write(corpus.repo().join("doomed.md"), "doomed\n").expect("write the doomed file");
    let workflows = corpus.repo().join(".jigc/config/workflows");
    fs::create_dir_all(&workflows).expect("mk the project workflows dir");
    fs::write(
        workflows.join(format!("{DOC_ONLY_WORKFLOW_ID}.yaml")),
        DOC_ONLY_WORKFLOW,
    )
    .expect("write the doc-only fixture workflow");
    corpus.git(&["add", "-A"]);
    corpus.git(&["commit", "-qm", "the arm's tracked files"]);

    for kind in ManifestKind::ALL {
        let (_, phase, act) = manifest_act(kind);
        if phase == Phase::BeforeMint {
            act(&corpus, "");
        }
    }
    let task = corpus.start_workflow("single-task", "put every manifest kind on the wire");
    fill_commit_doc(&corpus, &task);
    for kind in ManifestKind::ALL {
        let (_, phase, act) = manifest_act(kind);
        if phase == Phase::AfterMint {
            act(&corpus, &task);
        }
    }
    let report = corpus.start_workflow(DOC_ONLY_WORKFLOW_ID, "file a report beside it");
    author_report(&corpus, &report);

    // The machine surface first: the forecast is the arm that carries the manifest as data.
    let forecast_of = |model: Model| -> Value {
        let mut args = vec!["task", "finalize"];
        match model {
            Model::Index => args.extend([task.as_str(), "--carry-staged"]),
            // `--carry-staged` is inert on the doc-only arm, so it is not passed.
            Model::DocOnly => args.push(report.as_str()),
        }
        args.extend(["--dry-run", "--format", "json"]);
        assert_wire(&corpus.jigc(&args), &["task", "finalize"], "Forecast")
    };
    let kinds_at = |document: &Value, key: &str| -> BTreeSet<String> {
        document[key]
            .as_array()
            .unwrap_or_else(|| panic!("the forecast carries a `{key}` array; got:\n{document:#}"))
            .iter()
            .filter_map(|entry| entry["kind"].as_str().map(str::to_owned))
            .collect()
    };
    let mut sides: BTreeMap<&str, (BTreeSet<String>, BTreeSet<String>)> = BTreeMap::new();
    for (name, model) in [("index", Model::Index), ("doc-only", Model::DocOnly)] {
        let document = forecast_of(model);
        sides.insert(
            name,
            (
                kinds_at(&document, "manifest"),
                kinds_at(&document, "left_out"),
            ),
        );
    }
    let side = |model: Model| match model {
        Model::Index => &sides["index"],
        Model::DocOnly => &sides["doc-only"],
    };
    let on_the_wire: BTreeSet<String> = sides
        .values()
        .flat_map(|(committed, left_out)| committed.union(left_out).cloned())
        .collect();
    let vocabulary: BTreeSet<String> = ManifestKind::ALL
        .iter()
        .map(|kind| kind.tag().to_owned())
        .collect();
    assert_eq!(
        on_the_wire, vocabulary,
        "one checkout's two forecasts put the whole vocabulary on the wire — a kind \
         reachable only in principle is a value space nothing drives\n{sides:#?}",
    );
    for kind in ManifestKind::ALL {
        let (model, _, _) = manifest_act(kind);
        let (committed, left_out) = side(model);
        let where_it_landed = if kind.in_commit() {
            committed
        } else {
            left_out
        };
        assert!(
            where_it_landed.contains(kind.tag()),
            "`{}` is declared {} the commit, on the {model:?} model's forecast, and that is \
             where it must appear — the partition is why the vocabulary has two true \
             sentences\n{sides:#?}",
            kind.tag(),
            if kind.in_commit() { "in" } else { "outside" },
        );
    }
    assert!(
        !side(Model::Index)
            .1
            .contains(ManifestKind::LeftStaged.tag()),
        "the ordinary model never produces `left-staged` — its commit is the index\n{sides:#?}",
    );

    // …and the agent text, where the same tag is the label a reader sees. The doc-only task
    // lands first, so the ordinary one's commit still finds every staged path in the index.
    let report_landed = corpus.jigc(&["task", "finalize", &report]);
    let report_text = surface(&report_landed);
    assert!(
        report_landed.status.success(),
        "the doc-only finalize this arm forecast actually lands\n{report_text}",
    );
    let landed = corpus.jigc(&["task", "finalize", &task, "--carry-staged"]);
    let text = surface(&landed);
    assert!(
        landed.status.success(),
        "the finalize this arm forecast actually lands\n{text}",
    );
    for kind in ManifestKind::ALL {
        match manifest_act(kind).0 {
            Model::Index => assert!(
                text.contains(kind.tag()),
                "the agent text names `{}` with the same spelling the JSON `kind` carries — \
                 one home for the label, never a second list\n{text}",
                kind.tag(),
            ),
            // The text's left-out section is **path-only on every model** (`left_out_lines`):
            // the kind is the JSON's discriminator, and the doc-only header states the class
            // in words (a staged path stays staged for the task it belongs to). So what the
            // text owes a `left-staged` path is the path, under that section.
            Model::DocOnly => {
                let section = report_text
                    .split_once("  left-out (")
                    .unwrap_or_else(|| panic!("a left-out section\n{report_text}"))
                    .1;
                assert!(
                    section.contains("a staged path stays staged")
                        && section
                            .lines()
                            .any(|line| line == format!("    {LEFT_STAGED_PATH}")),
                    "the doc-only text names the path `{}` tags under its own left-out \
                     section\n{report_text}",
                    kind.tag(),
                );
            }
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 8 — the derived ambush owe-set (a DERIVATION, with a STATED EXCLUSION RULE)
// ═════════════════════════════════════════════════════════════════════════════

/// The shipped dev pack's tree on disk.
fn dev_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// The shipped methodology pack's tree on disk.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Recursively copy `src` into `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let (from, to) = (entry.path(), dst.join(entry.file_name()));
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Every step of `pack` whose `states-constraints:` front-matter declares `code`.
fn declarers_of(pack: &Path, code: &str) -> Vec<PathBuf> {
    support::root_walk::files_in(&pack.join("steps"), |_| true)
        .into_iter()
        .filter(|path| {
            fs::read_to_string(path)
                .unwrap_or_default()
                .lines()
                .any(|line| line.starts_with("states-constraints:") && line.contains(code))
        })
        .collect()
}

/// Withdraw `code` from every step of `pack` that declares it — the mutation the fence must
/// catch: the contract's declarer gone while the binary still refuses under it.
fn withdraw_declarer(pack: &Path, code: &str) -> usize {
    let mut withdrawn = 0usize;
    for path in declarers_of(pack, code) {
        let body = fs::read_to_string(&path).expect("read the copied step");
        let rewritten: String = body
            .lines()
            .map(|line| {
                if line.starts_with("states-constraints:") {
                    "states-constraints: []".to_owned()
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&path, format!("{rewritten}\n")).expect("write the stripped step");
        withdrawn += 1;
    }
    withdrawn
}

/// **Arm 8** — the owe-set is computed, and its consequence is driven at the doors.
///
/// **The kind of set: a derivation, stated as one, with a stated exclusion rule.**
/// `cli::pack::ambush_class_codes()` is `derive(blocking codes minted by a door in the
/// commit-on-behalf class of `BEHALF_DOORS`) − Exempt(<reason>) ∪
/// DECLARED_CONTRACT_IDENTIFIERS`. It replaced a hand-list for the reason D10 was decided
/// on: **nothing reddened when a new ambush-class contract stayed off one** — and a hand-list
/// has no place to put a *decision*.
///
/// **What this adds over `stated_at_fence.rs`.** That suite owns the fence's four tiers and
/// the derivation's two registry legs (every row's code really is minted at the production
/// function it names; the exempt row's door really does classify commit-on-behalf). This arm
/// drives the derivation's **consequence**: a pack whose declarer is withdrawn is refused at
/// **pack-load**, so the refusal meets the operator at whichever door they typed rather than
/// at the one that happens to compose — and the exempt row is declared by **no** step of
/// either shipped pack while pack-load stays green, which is the cell a hand-list could not
/// express at all.
#[test]
fn the_owe_set_is_derived_and_a_withdrawn_declarer_reddens_every_door() {
    // The derivation, read off the code rather than restated: every `Owed` row's code is in
    // the set, and the `Exempt(reason)` row is not.
    let owed = ambush_class_codes();
    let mut exempt_rows = 0usize;
    for row in AMBUSH_CONTRACTS {
        match row.disposition {
            AmbushDisposition::Owed => assert!(
                owed.contains(row.code),
                "`{}` is an Owed row of the source set, so the owe-set carries it",
                row.code,
            ),
            AmbushDisposition::Exempt(reason) => {
                exempt_rows += 1;
                assert!(
                    !owed.contains(row.code),
                    "`{}` states an exemption — {reason} — so the owe-set must NOT carry it: \
                     a derivation that picked it up would redden pack-load at every door, \
                     exit 1, with no legal declarer anywhere in either shipped pack",
                    row.code,
                );
            }
            // The third disposition (the F-10 review's MEDIUM-4): held out of the owe-set
            // because only one pack's composition can reach the door, and **declared
            // anyway** by that pack — so the owe-set does not carry it while the named-fact
            // tier binds at the declarer. `Owed` was driven to redden pack-load for the
            // methodology pack; `Exempt` would have meant *stated nowhere*, which is the
            // opposite of this cell.
            AmbushDisposition::DeclaredWhereReachable {
                reason,
                pack,
                declarer,
            } => {
                assert!(
                    !owed.contains(row.code),
                    "`{}` is owed by no pack — {reason} — so the derivation must not pick \
                     it up",
                    row.code,
                );
                assert!(
                    cli::pack::fenced_contract_codes().contains(row.code),
                    "…but its declaration must buy its facts: `{}` is stated in the `{pack}` \
                     pack's `{declarer}`, and a stated contract that no token map covers is \
                     the receipt-for-nothing MEDIUM-4 found",
                    row.code,
                );
            }
        }
    }
    assert_eq!(
        exempt_rows, 1,
        "the source set carries exactly one stated exemption — the cell a hand-list could \
         not express, because on a hand-list it is indistinguishable from a code somebody \
         forgot",
    );

    // The consequence, driven: one owed code's declarers withdrawn from a pack copy.
    let victim = owed
        .iter()
        .copied()
        .find(|code| !declarers_of(&dev_pack_tree(), code).is_empty())
        .expect("at least one owed code is declared by a dev-pack step");
    let pack = TempDir::new("ambush-pack");
    copy_tree(&dev_pack_tree(), pack.path());
    let withdrawn = withdraw_declarer(pack.path(), victim);
    assert!(
        withdrawn > 0,
        "the mutation must actually remove `{victim}`'s declarer(s)",
    );

    let (repo, home) = born_repo("ambush");
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    for door in [
        vec!["start", "--workflow", "single-task", "owe-set probe"],
        vec!["validate"],
        vec!["describe"],
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(&door)
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env("JIGC_PACK_DIR", pack.path())
            .output()
            .expect("spawn the jigc binary");
        let text = surface(&out);
        assert!(
            !out.status.success(),
            "`jigc {}` must be refused at PACK-LOAD — the obligation is jigc's, so the \
             refusal meets the operator at whichever door they typed\n{text}",
            door.join(" "),
        );
        assert!(
            text.contains(victim),
            "`jigc {}`: the refusal names the undeclared code, which is what makes the \
             owe-set's own message the fence's diagnostic\n{text}",
            door.join(" "),
        );
    }

    // The exempt row's green cell: declared by no step of either shipped pack, and both
    // packs load clean anyway.
    let exempt = AMBUSH_CONTRACTS
        .iter()
        .find(|row| matches!(row.disposition, AmbushDisposition::Exempt(_)))
        .expect("the source set carries its one stated exemption");
    for tree in [dev_pack_tree(), methodology_pack_tree()] {
        assert!(
            declarers_of(&tree, exempt.code).is_empty(),
            "`{}` is declared by no step of `{}` — which is exactly why it cannot be owed",
            exempt.code,
            tree.display(),
        );
    }
    let clean = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "owe-set control"],
    );
    assert!(
        clean.status.success(),
        "and the shipped composition loads clean with the exempt code declared nowhere\n{}",
        surface(&clean),
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 9 — the orphan arm over `STORE_EXIT_FLIPS` (MEMBERSHIP IS THE ASSERTION)
// ═════════════════════════════════════════════════════════════════════════════

/// The committed, stamped singletons the methodology pack authored **inside jigc's declared
/// territory** — the population that orphans the moment that pack leaves the composition.
/// Written down because which doctypes a *fixture* pack drops is a property of the fixture,
/// not of any registry.
const ORPHANED: &[&str] = &["docs/decisions-log.md", "docs/roadmap.md"];

/// **The stated residual** (M51 completion audit; `cli::orphan::Territory` → residual 2). The
/// same fixture pack orphans `VISION.md` too, at a **repo-root** placement home — a home that
/// declares no directory, and so no territory. The sweep's subject is jigc's declared homes
/// rather than the repository (the stamp being an unnamespaced `schema-version:` key that
/// cannot tell jigc's bytes from a team document's), which leaves that one cell at its
/// pre-M51 exit-0 status quo. Asserted below, not dropped.
const ROOT_HOMED_RESIDUAL: &str = "VISION.md";

/// The control: a committed stamped singleton whose doctype the dev pack still defines. A
/// green arm without it would be satisfied by a check that flags every stamped file it meets.
const STILL_CLAIMED: &str = "CHANGELOG.md";

/// The [`STORE_EXIT_FLIPS`] member this arm drives. Read off the table rather than
/// hand-spelled: **membership is the assertion**, so a member that stopped flipping the
/// exit reddens here rather than drifting.
fn orphan_flip() -> &'static StoreExitFlip {
    STORE_EXIT_FLIPS
        .iter()
        .find(|flip| flip.id == "orphaned-instance")
        .expect(
            "`orphaned-instance` must be a member of `cli::render::STORE_EXIT_FLIPS` — the \
             store sweep's exit rule IS that table, so a blocking store finding that is not \
             a member reports at exit 0 and the adopter's CI stays green over it",
        )
}

/// **Arm 9** — a doctype leaves the resolved set, and jigc stops calling the files it wrote
/// clean.
///
/// **The kind of set: a code-side registry where membership *is* the assertion.** The code
/// this arm asserts and the cause the closing line names are both read back off the
/// [`STORE_EXIT_FLIPS`] member's own witness and `cause`, so nothing here is a literal a
/// reader typed twice, and a member that stopped flipping the store sweep's exit fails this
/// arm rather than quietly restoring the false green.
///
/// **What this adds over `orphaned_instance.rs`.** That suite owns the partition against
/// `file-state.orphaned-doc` (a *home* that moved is not a *doctype* that left), the
/// resolved-but-unversioned sibling, and the doctype-narrowed listing. This arm walks the
/// adopter's picture in one corpus: the verb MIGRATING.md tells them to CI-gate on **exits
/// non-zero**, the index read prints the same files as `orphaned` rows with a null identity
/// and no item count, and the **two consumers agree as sets** — a file one of them names and
/// the other drops is the two-stories-about-one-file defect `doc list` was founded to end.
///
/// **Red at the wave's base** (`DECISIONS.md` → M51 Increment 8): the store surface was
/// silent in both directions — `jigc validate` printed *"no findings — the committed store
/// validates clean"* at **exit 0** while `git ls-files` still carried every one of those
/// files, and `jigc doc list` dropped their rows entirely.
#[test]
fn a_doctype_that_leaves_the_resolved_set_flips_the_exit_and_is_listed_as_orphaned() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let pack = FixturePack::from_dev_pack("flow52-orphan");
    let flip = orphan_flip();
    let code = (flip.witness)().code;

    let under_pack = |args: &[&str]| -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(corpus.repo())
            .env("HOME", corpus.home())
            .env("JIGC_PACK_DIR", pack.path())
            .output()
            .expect("spawn the jigc binary")
    };

    let swept = under_pack(&["validate"]);
    let report = surface(&swept);
    assert!(
        !swept.status.success(),
        "a committed store jigc can no longer say anything about must not validate clean at \
         exit 0 — the false green is the whole condition\n{report}",
    );
    for rel in ORPHANED {
        assert!(
            report.contains(&format!("blocking · {code} — committed doc `{rel}`")),
            "`{rel}` is committed, stamped, and claimed by no resolved doctype — the sweep \
             must name it\n{report}",
        );
        assert!(
            report.contains(&format!("jigc unmanage {rel}")),
            "the route is runnable and path-specific for `{rel}`\n{report}",
        );
    }
    assert!(
        !report.contains(&format!(
            "blocking · {code} — committed doc `{STILL_CLAIMED}`"
        )),
        "`{STILL_CLAIMED}`'s doctype is still defined, so the sweep stays silent about \
         it\n{report}",
    );
    assert!(
        !report.contains(&format!(
            "blocking · {code} — committed doc `{ROOT_HOMED_RESIDUAL}`"
        )),
        "the root-homed residual is outside every declared home, so the sweep may not speak \
         for it — the bound stated with the fix, asserted rather than assumed\n{report}",
    );
    assert!(
        report.contains(flip.cause),
        "the closing line names WHICH condition fired ({:?}) — the promise the preload sends \
         the agent there for\n{report}",
        flip.cause,
    );

    let listed = under_pack(&["doc", "list", "--format", "json"]);
    assert!(
        listed.status.success(),
        "`doc list` is a report and stays exit 0 — the exit flip is the sweep's\n{}",
        surface(&listed),
    );
    let index = json(&String::from_utf8_lossy(&listed.stdout));
    let rows = index["docs"]
        .as_array()
        .expect("`doc list --format json` carries a `docs` array");
    let mut listed_orphans: BTreeSet<String> = BTreeSet::new();
    for row in rows {
        if row["state"] == "orphaned" {
            assert!(
                row["id"].is_null() && row["item-count"].is_null(),
                "an orphan belongs to no doctype, so it has no identity to name and no item \
                 count to report; got {row:#}",
            );
            listed_orphans.insert(
                row["path"]
                    .as_str()
                    .expect("every row names its path")
                    .to_owned(),
            );
        }
    }
    assert!(
        rows.iter()
            .any(|row| row["path"] == STILL_CLAIMED && row["state"] == "managed"),
        "the control's doctype still resolves, so its row stays a managed one with its \
         identity; got:\n{index:#}",
    );

    // The two consumers read one enumerator: every file the sweep located is listed, and
    // the listing invents none. Both sides come off the emitted bytes.
    let lines: Vec<&str> = report.lines().collect();
    let mut located: BTreeSet<String> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            let path = line.trim().strip_prefix("at: ")?;
            let head = i.checked_sub(1).map(|prev| lines[prev])?;
            head.contains(code.as_str()).then(|| path.to_owned())
        })
        .collect();
    located.retain(|path| !path.is_empty());
    assert_eq!(
        listed_orphans, located,
        "a file one consumer names and the other drops is the two-stories-about-one-file \
         defect `doc list` was founded to end\nlisting:\n{index:#}\nsweep:\n{report}",
    );
    assert_eq!(
        listed_orphans,
        ORPHANED.iter().map(|rel| (*rel).to_owned()).collect(),
        "and the set is the population the fixture pack actually orphaned",
    );
}
