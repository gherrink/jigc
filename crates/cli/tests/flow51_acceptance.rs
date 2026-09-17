//! **The M50 unvalidated-token-wave done-picture acceptance suite** — the wave driven end
//! to end through the **real `jigc` binary** (`design/worked-examples.md` → flow 51;
//! roadmap → Milestone 50, Increment 13).
//!
//! **The claim the wave proves is one claim:** *no caller-supplied token becomes a path
//! component without the door validating it against the grammar, home or value rule that
//! token's own family already declares — and `jigc start` tells the truth about task state
//! on both its text and its versioned envelope*
//! (`completions/artifacts/M50/settle-record.md` → The claim, final). **Four families, two
//! rule kinds:** a slug grammar governs families 1, 2 and 4; a home rule and a value rule
//! govern family 3. The one-predicate framing was conceded wrong before the build —
//! `cli::trackable` refuses to ask about gitignore *by design*, so family 3 cannot ride the
//! trackability predicate — and folding unlike rules under one banner is the
//! false-completeness shape that has bitten four waves running (the Settle → D2, D8).
//!
//! Increments 1–12 shipped each fix with its own axis suite; this suite is the
//! **composite acceptance** that ties the wave into seven done-picture arms — **each arm
//! stating which kind of set it iterates**, and each stating what it adds over the axis
//! suite beside it, because an arm that re-runs a shipped axis proves the axis twice and
//! the wave once.
//!
//! The seven arms:
//!
//!   (1) **Families 4 and 2 — the token that names a file** — over two **code-side
//!       registries**: [`DOCTYPE_DOORS`] filtered to [`DoctypeArg::Address`] (the strongest
//!       set available, already fenced ⇔ against the real clap tree by
//!       `cli_parse::every_doctype_door_is_registered`) and [`SLUG_DOORS`], derived from the
//!       clap tree by [`slug_arg_ids`] and fenced the same way. **Family 2 is not dropped**:
//!       the claim names four families, so an acceptance reaching three of them would be the
//!       wave's own failure shape. *Adds over `address_slug_head_axis.rs` /
//!       `slug_override_axis.rs`:* those sweep each door over the whole token cell space and
//!       adjudicate each refusal; this arm hands **one** traversal to **every door of both
//!       families in one repository** and then asserts the **composite cost** the families
//!       are about — no commit landed, no byte left the docs root, the canary planted
//!       outside the repository reached no stream, and the store still validates and still
//!       addresses its docs.
//!
//!   (2) **Family 1 — the token that names a working area** — over
//!       [`WORK_UNIT_ID_DOORS`], **derived** from the clap tree by
//!       [`work_unit_id_arg_ids`] and fenced ⇔ against it, with each row carrying its own
//!       runnable argv so this arm reads the registry and writes no cell list of its own.
//!       *Adds over `work_unit_id_axis.rs`:* that suite sweeps four token cells per door and
//!       adjudicates codes, messages and routes; this arm drives the **one token that
//!       destroyed a repository** at every door of a **populated, committed** corpus and
//!       then asserts the repository is still there — `.git` present, `git ls-files`
//!       byte-identical, `HEAD` unmoved, the workbench intact — and that a task minted
//!       *after* the sweep still finalizes to a real commit, so the guard refuses the
//!       malformed token without stranding the well-formed one.
//!
//!   (3) **The migration's third locus** — over [`SchemaChangeKind::ALL`] × the loci, with
//!       the locus count **derived** ([`LOCI`] = [`MAX_NESTING_DEPTH`] + 1) and **never a
//!       literal**: a hardcoded *"three loci"* here would re-enact M45's *statement ==
//!       constant* failure inside the test written to prevent it (the Settle → D6, which
//!       prohibits it by name). *Adds over `migrate_locus_axis.rs`:* that suite drives the
//!       classifier per kind and owns the `kind × locus` table's own arms; this arm reads
//!       the table and then walks the **adopter's** path in one line — a committed,
//!       v2-stamped corpus folds at locus 3 through `jigc migrate-corpus`, validates clean
//!       afterwards, and the nested item the fold touched is **readable back at its own
//!       nested address**, which is the hop `jigc validate`'s findings compose.
//!
//!   (4) **Orientation's three states** — over [`OrientationView`], matched
//!       **exhaustively**, so a fourth variant cannot compile without deciding what the
//!       binary says in it. *Adds over `orientation_active_task.rs`:* that suite owns the
//!       active variant's keys, its text, its N>1 shape and its byte-for-byte block; this
//!       arm asserts the **partition** — three corpora, three states, each door's emitted
//!       `state` tag equal to the tag the exhaustive match assigns its variant, and the word
//!       `clean` absent from the whole active document.
//!
//!   (5) **Every write miss** — over [`CELLS`], the write-verb × miss-shape axis, **lifted
//!       to `crates/cli/tests/support/write_miss_cells.rs`** in this increment so there is
//!       exactly one of it: a flow suite compiles into `g_flow` and the axis suite into
//!       `g_finalize`, and the alternative to a lift was a **copy of an axis**, which is the
//!       failure the complete-fix contract is named for. *Adds over
//!       `write_miss_shape_axis.rs`:* that suite runs every row's emitted route verbatim and
//!       totals the rows against a manufactured address-shape × declaredness cross over its
//!       own hand-built corpus; this arm drives the same rows over the **shared
//!       `trial_corpus` substrate** and asserts the property the composite cares about — a
//!       miss is free: every row blocks with its own code, and after the whole sweep the
//!       staged bytes and the committed bytes are both unchanged.
//!
//!   (6) **Family 3 — the two root knobs and the workbench** — over [`ROOT_KNOBS`] (a
//!       code-side registry) for the home and value rules, and over the `ENTRIES`
//!       complement **stated as a derivation, not a registry**: membership is a path-prefix
//!       computation over the transient prefixes `jigc setup` writes into `.jigc/.gitignore`
//!       **conjoined with** a git query, and the untracked conjunct is mandatory — every
//!       non-transient `.jigc/` path is tracked on a fresh install, so without it the guard
//!       fires on every repo. *Adds over `root_knob_rules.rs` / `uninstall_workbench_subject.rs`:*
//!       those own the knob × spelling cross and the third subject's own arms; this arm
//!       walks the **loss** the family was found by — a knob that would home managed docs
//!       inside the tree `uninstall` removes whole, and then the teardown itself — and
//!       asserts the three answers are consistent in one repository.
//!
//!   (7) **The adapter's safety floor** — a **deliberately manufactured** two-member set,
//!       written as one, exactly as flow 49's arm 6 was. There is a registry next door
//!       ([`cli::milestone::DESTROYING_DOORS`], six members since M52 Increment 4) and it is
//!       **the wrong set**:
//!       `milestone finalize` destroys on the ordinary success path and `milestone provision`
//!       was refused from the carve-out on measured grounds — prompting there would park an
//!       unattended fix round on a prompt nobody is watching (the Settle → D9/B6). The two
//!       entries are a **decision**, so the arm iterates the decision and says so. *Adds
//!       over the adapter suites:* it drives the floor's whole life in one repo — installed
//!       by `setup`, **additive** over a user's own deny entry, and removed by `uninstall`
//!       leaving that entry standing.
//!
//! **What gets no arm, recorded as a decision.** Increments 8, 10, 11, 12 and 13 are
//! deliberately unrepresented. Increment 8 is a **pack-load fence** and a schema
//! projection key; Increment 10 is a render-path convergence; Increment 11 is a pack
//! composition; Increment 12 is the tier-2 wording batch; Increment 13 is this suite, the
//! ledgers and the goldens. None of them mints a verb, a finding or a route a done-picture
//! walk can reach, and manufacturing one would be a walk written to have an arm rather than
//! to prove a claim (the M46 Increment 9, M48 Increment 11 and M49 Increment 12 precedent).
//!
//! **Red at the wave's base**, arm by arm — each line is the behaviour the increment's own
//! `DECISIONS.md` entry recorded as *driven at HEAD* before its fix, not a re-run of this
//! suite against an older binary: `jigc doc show 'research:<absolute path outside the
//! repository>'` exited **0** and served the outside file's prose through the 1.0-pinned
//! JSON, and `jigc rename 'research:../../src/planted' --to "Captured Doc"` exited **0**
//! and committed an arbitrary in-repo file into the docs root, while
//! `jigc rename … --slug '../../src/pwned'` committed the doc *out* of the store;
//! `jigc task discard "../.."` resolved the repository root as a working area and removed
//! it — `.git`, `.jigc`, every tracked file — at **exit 0**; every one of the ~10 kinds the
//! second locus carries answered `migrate-corpus.unclassified-change` at the third, routing
//! the adopter into this workspace's engine source tree; `jigc start --format json` emitted
//! `"state": "clean"` — declared as *no active task* — over a repo holding one, with the
//! `refs-post-hoc` golden pinning that falsehood as expected output; write misses answered
//! with bare sentences carrying no code; `jigc config set docs-root .jigc` landed a knob
//! that homed managed docs inside the tree `uninstall` removes whole, and an `uninstall`
//! after any `jigc config set` destroyed the project's recorded cascade delta at exit 0;
//! and the adapter's floor was silent about the two doors that spend bytes no object
//! database has a copy of.
//!
//! Isolation: every arm builds its own throwaway repo — a real `git init`, a per-repo git
//! identity, `$HOME` repointed and `JIGC_PACK_DIR` scrubbed unless the arm deliberately
//! supplies a fixture pack — or rides the shared [`support::trial_corpus`] substrate, which
//! does all four by construction.

use crate::support;

use cli::cli::{
    DOCTYPE_DOORS, DoctypeArg, SLUG_DOOR_SOURCE, SLUG_DOORS, SLUG_OVERRIDE_SLOT,
    WORK_UNIT_ID_DOOR_PAYLOAD, WORK_UNIT_ID_DOORS, WORK_UNIT_ID_SLOT, slug_arg_ids,
    work_unit_id_arg_ids,
};
use cli::config::ROOT_KNOBS;
use engine::result::{ActiveTask, Catalog, OrientationView};
use engine::schema::MAX_NESTING_DEPTH;
use engine::schema_diff::{LOCI, LocusDisposition, SchemaChangeKind, locus_disposition};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use support::shape_space::FIXTURE_WORKFLOW;
use support::trial_corpus::{FixturePack, State, TrialCorpus};
use support::write_miss_cells::{CELLS, CHANGELOG_SCHEMA};

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// The one shipped grammar sentence every slug refusal states — spelled out rather than
/// imported, because a test comparing emitted bytes against the constant that produced
/// them proves only that the constant equals itself.
const GRAMMAR: &str =
    "use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)";

/// The blocking finding a malformed work-unit id earns, at every door of family 1.
const MALFORMED_ID: &str = "work-unit.malformed-id";

/// A throwaway directory that removes itself on drop — for the arms that build a repo, a
/// pack or a canary by hand rather than riding [`TrialCorpus`].
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow51-{tag}-{}-{:?}",
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
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Parse a `--format json` payload, surfacing the bytes on failure.
fn json(payload: &str) -> Value {
    serde_json::from_str(payload)
        .unwrap_or_else(|err| panic!("the payload is JSON ({err}); got:\n{payload}"))
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — families 4 and 2, over two CODE-SIDE REGISTRIES
// ═════════════════════════════════════════════════════════════════════════════

/// The runnable argv for one `DoctypeArg::Address` door, with `<slug>` standing in for the
/// address's slug head.
///
/// Exhaustive over the registered address doors **by panic**: an eleventh address-taking verb
/// joining [`DOCTYPE_DOORS`] with no row here fails this arm naming itself, rather than
/// being silently skipped — which is the only way a registry sweep can lie.
fn address_argv(door: &[&str], task: &str) -> Vec<String> {
    let argv: Vec<&str> = match door {
        ["rename"] => vec!["rename", "adr:<slug>", "--to", "Captured Doc"],
        ["doc", "add-item"] => vec![
            "doc",
            "add-item",
            "roadmap:<slug>#milestones",
            "--title",
            "Captured",
            "--task",
            task,
        ],
        ["doc", "remove-item"] => vec![
            "doc",
            "remove-item",
            "roadmap:<slug>#milestones/m-alpha",
            "--task",
            task,
        ],
        ["doc", "retitle-item"] => vec![
            "doc",
            "retitle-item",
            "roadmap:<slug>#milestones/m-alpha",
            "--title",
            "Captured",
            "--task",
            task,
        ],
        ["doc", "rename"] => vec![
            "doc",
            "rename",
            "adr:<slug>",
            "--to",
            "Captured Doc",
            "--task",
            task,
        ],
        ["doc", "set-field"] => vec![
            "doc",
            "set-field",
            "adr:<slug>#status/status",
            "--value",
            "accepted",
            "--task",
            task,
        ],
        ["doc", "set-slot"] => vec![
            "doc",
            "set-slot",
            "adr:<slug>#context",
            "--from-file",
            PAYLOAD,
            "--task",
            task,
        ],
        ["doc", "show"] => vec!["doc", "show", "adr:<slug>"],
        ["task", "bind"] => vec!["task", "bind", "spec", "spec:<slug>", task],
        // The milestone is real (minted by the arm), so this row reaches the spec read —
        // the seam whose escape served bytes from outside the repository at exit 0.
        ["milestone", "add-from-spec"] => {
            vec![
                "milestone",
                "add-from-spec",
                ADDRESS_MILESTONE,
                "spec:<slug>",
            ]
        }
        other => panic!(
            "`jigc {}` is a registered `DoctypeArg::Address` door with no cell in flow 51 \
             — a new address-taking verb owes this arm the argv that reaches its address \
             parse, or the wave's family-4 claim is unproven for it",
            other.join(" "),
        ),
    };
    argv.into_iter().map(str::to_owned).collect()
}

/// A payload file the `set-slot` rows read, so those doors answer about the token and not
/// about a missing file.
const PAYLOAD: &str = "payload.txt";

/// The committed `adr` every address-carrying row of both families names, with `{stamp}`
/// standing in for the doctype's **current** `schema-version`.
///
/// The stamp is read back from `jigc doc schema adr --format json` rather than written as
/// a literal: a planted doc below the current version is a *migration* fault, and this arm
/// then measures a store that is non-conformant for a reason that has nothing to do with
/// the token families — which is how a fixture literal turns a real assertion into noise
/// the day a doctype bumps.
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

/// The foreign source `jigc migrate --slug` adopts, so that row answers about the override.
const FOREIGN_CHANGELOG: &str = "# Change Log\n\n## v1\n\n- did a thing\n";

/// The milestone `jigc milestone add-from-spec` seeds into — real, so its row reaches the
/// door's spec read rather than refusing on the id.
const ADDRESS_MILESTONE: &str = "axis-milestone";

/// The prose planted **outside** the repository — the bytes a family-4 escape used to serve
/// through the 1.0-pinned JSON contract at exit 0.
const CANARY: &str = "PRIVATE-BYTES-NO-DOOR-MAY-SERVE";

/// **Arm 1** — families 4 and 2, over the two registries, with the composite cost asserted.
///
/// **The sets are code-side registries, and the strongest ones available.**
/// [`DOCTYPE_DOORS`] filtered to [`DoctypeArg::Address`] is bijectively fenced against the
/// real clap tree (`cli_parse::every_doctype_door_is_registered`), and [`SLUG_DOORS`] the
/// same way against [`slug_arg_ids`] — so neither set can drift from the binary's own door
/// set without a fence reddening. Family 2 rides this arm rather than being folded away:
/// the claim names **four** families, and an acceptance that reached three of them would be
/// the wave's own failure shape wearing the wave's own name.
///
/// **What this adds over the two axis suites** is the *cost*, asserted once for the
/// composite rather than per cell: the axis suites sweep each door over the whole token
/// space and adjudicate every refusal's text and route; here one traversal goes to every
/// door of both families **in one repository**, and afterwards the repository is asserted
/// whole — `HEAD` unmoved, the doc still at its own slug, nothing written outside the docs
/// root, no byte of the outside-the-repository canary in any stream, the store validating
/// clean and `jigc doc list` still naming the doc. The traversal goes to **both**
/// families; the absolute outside-the-repository path goes to the address doors, which
/// are the ones that *read* — it is the cell whose exit-0 served private bytes through the
/// pinned JSON.
#[test]
fn no_address_or_override_token_becomes_a_path_component_and_the_tree_is_untouched() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();

    // The committed `adr` both families address, the foreign migrate source, a real
    // source tree to capture, and the `set-slot` payload.
    fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
    let adr_stamp =
        json(&corpus.jigc_ok(&["doc", "schema", "adr", "--format", "json"]))["schema-version"]
            .as_u64()
            .expect("`doc schema adr` reports the doctype's current schema-version");
    fs::write(
        repo.join("docs/decisions/keeper.md"),
        ADR_KEEPER.replace("{stamp}", &adr_stamp.to_string()),
    )
    .expect("write the keeper adr");
    fs::write(repo.join(SLUG_DOOR_SOURCE), FOREIGN_CHANGELOG).expect("write the migrate source");
    fs::write(repo.join(PAYLOAD), "prose\n").expect("write the payload");
    fs::create_dir_all(repo.join("src")).expect("mk src");
    fs::write(repo.join("src/module.md"), "source-tree prose\n").expect("write src");
    // Driven fixture fact (macOS resolves `..` physically): the traversal's destination
    // directory must exist on disk, or the escape is refused for an unrelated reason and
    // the tree assertions prove nothing.
    fs::create_dir_all(repo.join("docs/research")).expect("mk research home");
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-qm", "seed the store"]);

    // The canary lives OUTSIDE the repository — the bytes family 4's escape served.
    let outside = TempDir::new("canary");
    let canary = outside.path().join("private.md");
    fs::write(
        &canary,
        format!("---\nstatus: accepted\ndate: 2026-01-01\nschema-version: 1\n---\n\n# Private\n\n## Context\n\n{CANARY}\n"),
    )
    .expect("write the canary");
    let canary_head = canary.to_string_lossy().trim_end_matches(".md").to_string();

    // `implement-from-spec` is the one shipped workflow declaring a `reads` role, so the
    // `task bind` row reaches its address parse instead of stopping at the role check.
    let task = corpus.start_workflow("implement-from-spec", "the token families");
    // The `milestone add-from-spec` row's milestone is REAL — its escape sits behind the
    // milestone resolve, so a bogus id would refuse for an unrelated reason and the row
    // would pass while proving nothing. Minted before `before_head`: the record-only
    // commit it lands is the fixture, not a door under test.
    let minted = corpus.jigc_ok(&["milestone", "create", "Axis Milestone"]);
    assert!(
        minted.contains(ADDRESS_MILESTONE),
        "the arm's milestone must be `{ADDRESS_MILESTONE}`; got: {minted}",
    );
    let before_head = git(&repo, &["rev-parse", "HEAD"]);

    let traversal = "../../src/planted".to_string();
    let mut driven_addresses: BTreeSet<Vec<&str>> = BTreeSet::new();
    for (door, arg) in DOCTYPE_DOORS {
        if *arg != DoctypeArg::Address {
            continue;
        }
        for token in [&traversal, &canary_head] {
            let argv = address_argv(door, &task);
            let argv: Vec<String> = argv
                .iter()
                .map(|part| part.replace("<slug>", token))
                .collect();
            let out = corpus.jigc(&argv.iter().map(String::as_str).collect::<Vec<&str>>());
            let text = surface(&out);
            let shown = format!("jigc {} [{token}]", door.join(" "));
            assert_ne!(
                out.status.code(),
                Some(101),
                "{shown}: the refusal is a finding, never a panic\n{text}",
            );
            assert!(
                !out.status.success(),
                "{shown}: a `<slug>` head that is not a slug names a file nobody addressed \
                 — the door must block non-zero\n{text}",
            );
            assert!(
                text.contains(GRAMMAR),
                "{shown}: the refusal states the one shipped grammar sentence\n{text}",
            );
            assert!(
                !text.contains(CANARY),
                "{shown}: no byte of a file outside the repository may reach any stream\n{text}",
            );
        }
        driven_addresses.insert(door.to_vec());
    }

    let registered_addresses: BTreeSet<Vec<&str>> = DOCTYPE_DOORS
        .iter()
        .filter(|(_, arg)| *arg == DoctypeArg::Address)
        .map(|(door, _)| door.to_vec())
        .collect();
    assert_eq!(
        driven_addresses, registered_addresses,
        "every registered `DoctypeArg::Address` door is driven exactly once — a row with \
         no cell is a failure, never a skip",
    );

    let mut driven_slugs: BTreeSet<Vec<&str>> = BTreeSet::new();
    for row in SLUG_DOORS {
        let argv: Vec<&str> = row
            .argv
            .iter()
            .map(|part| {
                if *part == SLUG_OVERRIDE_SLOT {
                    traversal.as_str()
                } else {
                    *part
                }
            })
            .collect();
        let out = corpus.jigc(&argv);
        let text = surface(&out);
        let shown = format!("jigc {} --slug [{traversal}]", row.door.join(" "));
        assert_ne!(
            out.status.code(),
            Some(101),
            "{shown}: the refusal is a finding, never a panic\n{text}",
        );
        assert!(
            !out.status.success(),
            "{shown}: a `--slug` override is taken verbatim as a path component — the door \
             must block non-zero\n{text}",
        );
        assert!(
            text.contains(GRAMMAR),
            "{shown}: the refusal states the one shipped grammar sentence\n{text}",
        );
        driven_slugs.insert(row.door.to_vec());
    }
    assert_eq!(
        driven_slugs.len(),
        SLUG_DOORS.len(),
        "every registered `--slug` door is driven exactly once",
    );
    assert!(
        !slug_arg_ids().is_empty(),
        "the derivation vocabulary must be non-empty, else the ⇔ fence is vacuous",
    );

    // ── The composite cost — the half no per-door cell asserts. ──
    assert_eq!(
        git(&repo, &["rev-parse", "HEAD"]),
        before_head,
        "not one door of either family may land a commit",
    );
    assert!(
        repo.join("docs/decisions/keeper.md").is_file(),
        "the committed doc stays at its own slug",
    );
    for stray in [
        "src/planted.md",
        "src/pwned.md",
        "docs/research/captured-doc.md",
    ] {
        assert!(
            !repo.join(stray).exists(),
            "`{stray}` was written — a token became a path component",
        );
    }
    assert_eq!(
        fs::read_to_string(&canary).expect("the canary is still readable"),
        format!(
            "---\nstatus: accepted\ndate: 2026-01-01\nschema-version: 1\n---\n\n# Private\n\n## Context\n\n{CANARY}\n"
        ),
        "a file outside the repository is neither read into the store nor rewritten",
    );
    let listing = corpus.jigc_ok(&["doc", "list", "adr"]);
    assert!(
        listing.contains("adr:keeper"),
        "the store still addresses its docs by the surfaces that name them:\n{listing}",
    );
    let validated = corpus.jigc(&["validate"]);
    assert!(
        validated.status.success(),
        "the store validates clean after the whole sweep:\n{}",
        surface(&validated),
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — family 1, over the DERIVED door set the registry carries argv for
// ═════════════════════════════════════════════════════════════════════════════

/// **Arm 2** — every door that takes a work-unit id refuses the traversal, and the
/// repository is still there afterwards.
///
/// **The set is [`WORK_UNIT_ID_DOORS`] — derived, not remembered.** It is built from the
/// clap tree by [`work_unit_id_arg_ids`] and fenced **⇔** against it by
/// `cli_parse::every_work_unit_id_door_is_registered`, so a twenty-sixth door taking a task
/// or milestone id cannot ship without joining it. Each row carries its **own** runnable
/// argv with [`WORK_UNIT_ID_SLOT`] where the id goes, so this arm writes no cell list of
/// its own: the registry is both the subject and the driver.
///
/// **What this adds over `work_unit_id_axis.rs`** is the loss. That suite sweeps four token
/// cells per door and adjudicates the code, the message and the route at each; this arm
/// hands **the one token that removed a repository** to every door of a corpus that has
/// something to lose — committed docs, a workbench, history — and then asserts the corpus
/// survived it: `.git` present, the tracked file set byte-identical, `HEAD` unmoved, the
/// workbench intact. The second half is the over-firing guard at the done-picture altitude:
/// a task minted **after** the sweep still finalizes to a real commit, so the door that
/// refuses `"../.."` has not learned to refuse the ids jigc itself mints.
#[test]
fn every_work_unit_id_door_refuses_the_traversal_and_the_repository_survives() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let repo = corpus.repo();
    fs::write(
        repo.join(WORK_UNIT_ID_DOOR_PAYLOAD),
        "title: Axis\nsections: []\n",
    )
    .expect("write the door payload");

    let before_head = git(&repo, &["rev-parse", "HEAD"]);
    let before_files = git(&repo, &["ls-files"]);
    assert!(
        before_files.contains("CHANGELOG.md") && before_files.contains("VISION.md"),
        "the corpus must have committed docs to lose:\n{before_files}",
    );

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    for row in WORK_UNIT_ID_DOORS {
        let argv: Vec<&str> = row
            .argv
            .iter()
            .map(|part| {
                if *part == WORK_UNIT_ID_SLOT {
                    "../.."
                } else {
                    *part
                }
            })
            .collect();
        let out = corpus.jigc(&argv);
        let text = surface(&out);
        let shown = format!("jigc {} [../..]", row.door.join(" "));
        assert_ne!(
            out.status.code(),
            Some(101),
            "{shown}: the refusal is a finding, never a panic\n{text}",
        );
        assert!(
            !out.status.success(),
            "{shown}: a token that is not an id resolves to a directory nobody named — the \
             door must block non-zero\n{text}",
        );
        assert!(
            text.contains(MALFORMED_ID),
            "{shown}: the refusal carries `{MALFORMED_ID}`, not the roster answer — the \
             token was never an id\n{text}",
        );
        assert!(
            text.contains(GRAMMAR),
            "{shown}: the refusal states the grammar the token failed\n{text}",
        );
        driven.insert(row.door.to_vec());
    }
    assert_eq!(
        driven.len(),
        WORK_UNIT_ID_DOORS.len(),
        "every registered work-unit-id door is driven exactly once",
    );
    assert!(
        !work_unit_id_arg_ids().is_empty(),
        "the derivation vocabulary must be non-empty, else the ⇔ fence is vacuous",
    );

    // ── The repository is still a repository. ──
    assert!(repo.join(".git").is_dir(), "`.git` survived the sweep");
    assert!(
        repo.join(".jigc").is_dir(),
        "the workbench survived the sweep"
    );
    assert_eq!(
        git(&repo, &["rev-parse", "HEAD"]),
        before_head,
        "no door of family 1 may move HEAD",
    );
    assert_eq!(
        git(&repo, &["ls-files"]),
        before_files,
        "no door of family 1 may remove a tracked file",
    );

    // ── And the guard did not learn to refuse the ids jigc mints. ──
    let task = corpus.start_workflow("single-task", "the work unit that is well formed");
    // Staged AFTER the mint on purpose: a path staged before it is exactly what the
    // carryover gate refuses, so staging first would prove the wrong thing.
    fs::write(repo.join("axis.txt"), "the well-formed unit's work\n").expect("write the work");
    git(&repo, &["add", "axis.txt"]);
    corpus.set_field(&format!("commit:{task}#header/type"), &task, "chore");
    corpus.set_field(&format!("commit:{task}#header/scope"), &task, "axis");
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "close the axis");
    corpus.set_slot(
        &format!("commit:{task}#body"),
        &task,
        "The well-formed id still finalizes.",
    );
    let landed = corpus.jigc_ok(&["task", "finalize", &task]);
    assert!(
        landed.contains("finalized"),
        "a task minted after the sweep still finalizes to a real commit:\n{landed}",
    );
    assert_ne!(
        git(&repo, &["rev-parse", "HEAD"]),
        before_head,
        "the well-formed task's finalize landed a commit",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — the migration's third locus, over a DERIVED locus count
// ═════════════════════════════════════════════════════════════════════════════

/// The shipped `changelog` nested item block's one declared leaf line — the anchor a
/// locus-3 leaf is appended after. Its **indentation** is what distinguishes the nested
/// change-group include from the staging one, which is the same text six columns out, so
/// the anchor is matched with its leading run rather than by name.
const NESTED_INCLUDE: &str = "              - include: change-group\n";

/// A conformant, **v2-stamped** `CHANGELOG.md` carrying one release that **nests** a
/// change-group — the committed shape the locus-3 fold runs over, and the shape whose
/// nested address the read-back resolves.
const CHANGELOG_V2: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group is the control

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14
- link: https://example.com/compare/0.9.0...1.0.0

#### added  {#added}

- the nested change-group the fold reaches
";

/// A dev-pack copy bumping `changelog` **2 → 3** with one optional leaf appended to its
/// **nested** `changes` block: the `changelog.v2` snapshot is the prior shape, the schema
/// carries the added leaf, and the manifest's `schema-hash` is re-pinned through the
/// production loader — the act a pack author performs, never a forged hash.
fn pack_adding_a_nested_leaf(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    support::frozen_pack::copy_dev_pack(dir.path());

    let schema_path = dir.path().join("schemas").join("changelog.yaml");
    let shipped = fs::read_to_string(&schema_path).expect("read the copied changelog.yaml");
    fs::write(
        dir.path()
            .join("schema-snapshots")
            .join("changelog.v2.yaml"),
        &shipped,
    )
    .expect("write the changelog.v2 snapshot");

    // The indentation is READ off the anchor rather than restated: a literal run of spaces
    // here would be a fact that drifts the day the schema is re-indented.
    let indent = &NESTED_INCLUDE[..NESTED_INCLUDE.len() - NESTED_INCLUDE.trim_start().len()];
    let reshaped = shipped.replacen(
        NESTED_INCLUDE,
        &format!("{NESTED_INCLUDE}{indent}- {{ id: ticket, type: string, optional: true }}\n"),
        1,
    );
    assert_ne!(
        reshaped, shipped,
        "the shipped `changelog` must declare a nested change-group include",
    );
    fs::write(&schema_path, reshaped).expect("write the reshaped changelog.yaml");

    let manifest_path = dir.path().join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        "- type: changelog\n    schema-version: 2",
        "- type: changelog\n    schema-version: 3",
        1,
    );
    assert_ne!(
        manifest, bumped,
        "the manifest must carry `changelog` at schema-version 2",
    );
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");
    support::frozen_pack::repin_manifest_hash(dir.path(), "changelog");
    dir
}

/// Run `jigc <args>` with an explicit `cwd`, `$HOME` and `JIGC_PACK_DIR`.
fn jigc_with_pack(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
}

/// **Arm 3** — every schema-change kind has a verdict at every locus the nesting cap
/// admits, and the adopter's path through the third one runs.
///
/// **The set is [`SchemaChangeKind::ALL`] × the loci, and the locus count is DERIVED.**
/// [`LOCI`] is `MAX_NESTING_DEPTH + 1`, and [`MAX_NESTING_DEPTH`] is itself derived from
/// the address grammar's hop budget — so this arm follows the constants rather than
/// pinning today's `3`. The **domain is `1..=LOCI`** — the section itself is locus 1, its item
/// block 2, a nested item block 3 ([`engine::schema_diff::Locus::index`]) — and the arm asserts it
/// walks *that* set rather than one merely of the right size: a `LOCI`-long range based at 0
/// satisfies the cell count below while asking a non-locus, where every kind answers
/// `Unreachable`, and never asking locus 3 — the locus Increments 6/7 built. The derivation is a
/// rule the Settle states by name: *any increment hardcoding "three loci" re-enacts M45's
/// statement-equals-constant failure* (`completions/artifacts/M50/settle-record.md` → D6), and a
/// literal here would re-enact it **inside the test written to prevent it**. The kind set is the
/// discriminant enum the increment minted for exactly this, because `SchemaChange::ALL` cannot
/// exist — every variant carries data — and a count is not a set.
///
/// **What this adds over `migrate_locus_axis.rs`** is the adopter's walk. That suite owns
/// the `kind × locus` table's own arms and drives the classifier kind by kind against a
/// manufactured nested pair; this arm reads the table as a *precondition* and then walks
/// the shipped path end to end: a committed, v2-stamped corpus whose release **nests** a
/// change-group folds at locus 3 through `jigc migrate-corpus`, the stamp is the fold's
/// only byte delta, the migrated corpus validates clean, and the nested item is read back
/// at **its own nested address** — the hop a conformance finding composes, and the hop
/// that used to compose an address the tool's own grammar refused.
#[test]
fn every_change_kind_has_a_verdict_at_every_derived_locus_and_locus_three_folds() {
    // ── The table, over the derived locus count. ──
    assert_eq!(
        LOCI,
        MAX_NESTING_DEPTH + 1,
        "the locus count is DERIVED from the nesting cap — a literal here would be the \
         failure this arm exists to prevent",
    );
    let mut cells = 0usize;
    for locus in 1..=LOCI {
        // A locus is a place a change can be **applied**. An index outside the domain answers
        // `Unreachable`/`DoctypeLevel` for every kind, so this is what tells the arm it is
        // walking the loci rather than an index set of the right SIZE — the count guard below
        // cannot, since any `LOCI`-long range satisfies it.
        let applied = SchemaChangeKind::ALL
            .iter()
            .filter(|kind| matches!(locus_disposition(**kind, locus), LocusDisposition::Applied))
            .count();
        assert!(
            applied > 0,
            "locus {locus} applies no kind at all — that is not a locus, and this arm is \
             iterating the wrong index set (the domain is `1..=LOCI`)",
        );
        for kind in SchemaChangeKind::ALL {
            let verdict = locus_disposition(kind, locus);
            assert!(
                !matches!(verdict, LocusDisposition::Unbuilt),
                "`{}` at locus {locus} has no byte-writing arm — the third locus closes \
                 only when no cell reads `Unbuilt`",
                kind.as_str(),
            );
            if kind == SchemaChangeKind::Unclassified {
                assert!(
                    !matches!(verdict, LocusDisposition::Applied),
                    "the empty-diff backstop may not be a cell the driver applies",
                );
            }
            cells += 1;
        }
    }
    assert_eq!(
        cells,
        SchemaChangeKind::ALL.len() * LOCI,
        "every kind is dispositioned at every locus",
    );

    // ── The adopter's walk through the third locus. ──
    let home = TempDir::new("locus-home");
    let repo = TempDir::new("locus-repo");
    let pack = pack_adding_a_nested_leaf("locus-pack");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "t@example.com"]);
    git(repo.path(), &["config", "user.name", "T"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    fs::write(repo.path().join("CHANGELOG.md"), CHANGELOG_V2).expect("write the changelog");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-qm", "seed the corpus"]);

    let out = jigc_with_pack(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let report = json(&String::from_utf8_lossy(&out.stdout));
    assert!(
        out.status.success(),
        "a clean locus-3 migration exits 0; report:\n{report:#}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        report["migrated"].as_array().map(Vec::len),
        Some(1),
        "the committed changelog migrates; report:\n{report:#}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read the changelog"),
        CHANGELOG_V2.replace("schema-version: 2", "schema-version: 3"),
        "a nested optional add is a byte no-op: the stamp is the fold's only delta",
    );
    let validated = jigc_with_pack(repo.path(), home.path(), pack.path(), &["validate"]);
    assert!(
        validated.status.success(),
        "the migrated corpus validates clean:\n{}",
        surface(&validated),
    );

    // The hop that used to be missing: a nested item's own address, resolved.
    let shown = jigc_with_pack(
        repo.path(),
        home.path(),
        pack.path(),
        &[
            "doc",
            "show",
            "changelog:changelog#releases/1-0-0/changes/added",
        ],
    );
    assert!(
        shown.status.success(),
        "the nested item resolves at the address a finding inside it composes:\n{}",
        surface(&shown),
    );
    assert!(
        surface(&shown).contains("the nested change-group the fold reaches"),
        "the read-back serves the nested item's own prose:\n{}",
        surface(&shown),
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — orientation, over an EXHAUSTIVELY MATCHED enum
// ═════════════════════════════════════════════════════════════════════════════

/// The `state` tag a view renders — matched **exhaustively**, so a fourth
/// [`OrientationView`] variant cannot compile without deciding what the binary says in it.
/// This is the arm's set: the enum itself, checked by the compiler rather than by a list.
fn state_tag(view: &OrientationView) -> &'static str {
    match view {
        OrientationView::UnsetProject { .. } => "unset-project",
        OrientationView::Clean { .. } => "clean",
        OrientationView::ActiveTask { .. } => "active-task",
    }
}

/// One `ActiveTask` row, so the third variant can be constructed for the match above.
fn probe_active_task() -> ActiveTask {
    ActiveTask {
        id: "probe".to_string(),
        workflow: Some("single-task".to_string()),
        intent: "probe".to_string(),
        milestone: None,
        base: None,
        staged: Vec::new(),
        findings: None,
        findings_unavailable: None,
    }
}

/// **Arm 4** — orientation answers in exactly one of its three states, and each state's
/// emitted tag is the tag its own variant carries.
///
/// **The set is [`OrientationView`], matched exhaustively.** [`state_tag`] is the fence: a
/// fourth variant fails to compile there, which is a stronger claim than a census of the
/// three tags the binary happens to emit today. The three variants are **constructed**
/// through the shipped constructors and mapped through that match, so the tags this arm
/// asserts against are the enum's, not literals a reader typed twice.
///
/// **What this adds over `orientation_active_task.rs`** is the partition. That suite owns
/// the active variant — its seven keys, its four `Run:` directives, its N>1 shape, its
/// blocking-finding-at-exit-0 rule and its byte-for-byte block; this arm drives **three
/// corpora** and asserts that each lands in exactly one state, that the tag matches the
/// variant, and that the word `clean` — declared in the result contract as *no active
/// task* — appears nowhere in the document a repo with a live task emits. That last is the
/// wave's second claim in one assertion: the envelope used to say `clean` over a repo
/// holding a task, and the `refs-post-hoc` golden pinned it as expected output.
#[test]
fn orientation_answers_in_exactly_one_of_its_three_variants() {
    let views = [
        OrientationView::unset_project(),
        OrientationView::clean("header", Catalog::default(), Vec::new()),
        OrientationView::active_task(
            "header",
            vec![probe_active_task()],
            Catalog::default(),
            Vec::new(),
        ),
    ];
    let tags: Vec<&'static str> = views.iter().map(state_tag).collect();
    assert_eq!(
        tags.iter().collect::<BTreeSet<_>>().len(),
        views.len(),
        "each variant carries its own tag — two variants sharing one would make the \
         partition unobservable on the wire",
    );

    // (a) No project cascade layer at all.
    let unset_repo = TempDir::new("unset-repo");
    let unset_home = TempDir::new("unset-home");
    git(unset_repo.path(), &["init", "-q"]);
    git(
        unset_repo.path(),
        &["config", "user.email", "t@example.com"],
    );
    git(unset_repo.path(), &["config", "user.name", "T"]);
    fs::write(unset_repo.path().join("README.md"), "hello\n").expect("write README");
    git(unset_repo.path(), &["add", "."]);
    git(unset_repo.path(), &["commit", "-qm", "initial"]);
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--format", "json"])
        .current_dir(unset_repo.path())
        .env("HOME", unset_home.path())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "orientation reports:\n{}",
        surface(&out)
    );
    let unset = json(&String::from_utf8_lossy(&out.stdout));
    assert_eq!(unset["state"], tags[0], "an unset project tags itself");

    // (b) The cascade resolves, no task is live.
    let clean_corpus = TrialCorpus::build(State::Fresh);
    let clean = json(&clean_corpus.jigc_ok(&["start", "--format", "json"]));
    assert_eq!(clean["state"], tags[1], "a task-less project tags itself");
    assert!(
        clean.get("tasks").is_none(),
        "the clean view carries no active set — an empty one would contradict its own tag",
    );

    // (c) The cascade resolves and a task is live.
    let live_corpus = TrialCorpus::build(State::RefsPostHoc);
    let live_task = live_corpus
        .live_task()
        .expect("the refs-post-hoc corpus carries a live task")
        .to_string();
    let payload = live_corpus.jigc_ok(&["start", "--format", "json"]);
    let live = json(&payload);
    assert_eq!(
        live["state"], tags[2],
        "a project holding a task tags itself"
    );
    assert!(
        live["tasks"]
            .as_array()
            .is_some_and(|rows| rows.iter().any(|row| row["id"] == live_task.as_str())),
        "the active set names the live task:\n{payload}",
    );
    assert!(
        !payload.contains("clean"),
        "the word `clean` — declared as *no active task* — reaches nowhere in the document \
         a repo holding a task emits:\n{payload}",
    );
    // The text surface and the envelope agree about the same repository, which is the gap
    // a text-only cut would have shipped through the parity fence's one blind spot.
    let text = live_corpus.jigc_ok(&["start"]);
    assert!(
        text.contains(&live_task),
        "the agent text names the live task too:\n{text}",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — every write miss, over the LIFTED shared axis
// ═════════════════════════════════════════════════════════════════════════════

/// **Arm 5** — a miss is free: every write verb answers its own miss with its own code, and
/// the sweep costs the corpus nothing.
///
/// **The set is [`CELLS`], and it was LIFTED rather than copied.** The rows lived in
/// `write_miss_shape_axis.rs`, which compiles into the `g_finalize` target while a flow
/// suite compiles into `g_flow`, so no cross-group `use` exists: the choice was a lift or a
/// **copy of an axis**, and a copied axis is two lists that agree today and disagree the
/// first time a row joins one of them — the failure the complete-fix contract is named for.
/// The rows now live in `crates/cli/tests/support/write_miss_cells.rs` with the corpus
/// shape they address, on `support::shape_space`'s precedent (M49 Increment 1 lifted the
/// item-region generator there for the same reason).
///
/// **What this adds over `write_miss_shape_axis.rs`** is two things it deliberately does
/// not have. That suite runs each row's emitted route **verbatim** and totals the rows
/// against a manufactured address-shape × declaredness cross, over a corpus it builds by
/// hand. This arm drives the same rows over the **shared `trial_corpus` substrate** — a
/// second, independent path to the same corpus shape, so a fixture-construction accident
/// cannot green both — and asserts the composite property: every row blocks with the code
/// it earns, and after the whole sweep the staged bytes **and** the committed bytes are
/// byte-identical to what they were before it. A miss that costs a byte is a defect no
/// per-row code assertion can see.
#[test]
fn every_write_miss_answers_with_its_own_code_and_the_sweep_costs_nothing() {
    let pack = FixturePack::from_dev_pack("flow51-write-miss");
    pack.write_schema("changelog", CHANGELOG_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
    let task = corpus.start_workflow("log-finding", "log the release");

    let created = corpus.jigc_ok(&["doc", "create", "changelog", "--title", "Changelog"]);
    let addr = created.trim().to_owned();
    let release = corpus.add_item(&format!("{addr}#releases"), "1-3-0", &task);
    corpus.add_item(&format!("{release}/changes"), "Added", &task);
    corpus.add_item(&format!("{addr}#staged"), "Changed", &task);

    let staged = corpus
        .repo()
        .join(".jigc/tasks")
        .join(&task)
        .join("docs")
        .join(format!("{addr}.md"));
    let before_staged = fs::read_to_string(&staged).expect("read the staged changelog");
    let before_head = git(&corpus.repo(), &["rev-parse", "HEAD"]);
    let before_tree = git(&corpus.repo(), &["ls-files"]);

    let mut broken: Vec<String> = Vec::new();
    for cell in CELLS {
        let argv: Vec<String> = cell
            .args
            .iter()
            .map(|arg| arg.replace("{addr}", &addr))
            .chain(["--format".to_owned(), "json".to_owned()])
            .collect();
        let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
        let out = match cell.stdin {
            Some(bytes) => corpus.jigc_stdin(&refs, &String::from_utf8_lossy(bytes)),
            None => corpus.jigc(&refs),
        };
        let text = surface(&out);
        assert_ne!(
            out.status.code(),
            Some(101),
            "`{}`: the reject is a finding, never a panic\n{text}",
            cell.what,
        );
        if out.status.success() {
            broken.push(format!("`{}` exited 0 over a miss", cell.what));
            continue;
        }
        let report = json(String::from_utf8_lossy(&out.stderr).trim());
        let got = report["findings"][0]["code"].as_str().unwrap_or("<absent>");
        if got != cell.code {
            broken.push(format!(
                "`{}` answered `{got}`, not `{}`",
                cell.what, cell.code,
            ));
        }
        assert!(
            report.get("error").is_none(),
            "`{}`: a miss is a finding, not a bare error envelope\n{text}",
            cell.what,
        );
    }
    assert!(
        broken.is_empty(),
        "{} of {} write-miss cells answered wrong:\n{}",
        broken.len(),
        CELLS.len(),
        broken.join("\n"),
    );

    // ── The cost, which is the composite's half. ──
    assert_eq!(
        fs::read_to_string(&staged).expect("read the staged changelog"),
        before_staged,
        "the whole miss sweep persisted nothing into the working area",
    );
    assert_eq!(
        git(&corpus.repo(), &["rev-parse", "HEAD"]),
        before_head,
        "the whole miss sweep landed no commit",
    );
    assert_eq!(
        git(&corpus.repo(), &["ls-files"]),
        before_tree,
        "the whole miss sweep touched no tracked path",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 6 — family 3: the two root knobs (a REGISTRY) and the `ENTRIES` complement
//         (a DERIVATION, stated as one)
// ═════════════════════════════════════════════════════════════════════════════

/// **Arm 6** — neither root knob accepts a home or a value that leaves the store lying, and
/// the teardown gives one answer per kind of byte under `.jigc/`.
///
/// **Two kinds of set, and the second is deliberately not a registry.** The knob half
/// iterates [`ROOT_KNOBS`], a code-side registry that replaced two hand-written `matches!`
/// — the shape M45's complete-fix lens is named for, since a rule applied at one arm and
/// not the other is a rule that is not applied at all. The teardown half's subject is the
/// **`ENTRIES` complement**, and that is a *derivation*: membership is a path-prefix
/// computation over the transient prefixes `jigc setup` writes into `.jigc/.gitignore`,
/// **conjoined with** a git query about whether any index has a copy of the bytes. Nothing
/// enumerates the files, so there is no registry to read — and the arm reads the prefixes
/// off the installed `.gitignore` rather than restating them, so a prefix added to the
/// production constant narrows this arm's subject automatically.
///
/// **The untracked conjunct is mandatory, not decoration.** Every non-transient `.jigc/`
/// path is *tracked* on a fresh install, so a guard whose subject were "everything under
/// `.jigc/`" would fire on every repository; and `tasks/` and `worktrees/` are *inside*
/// `ENTRIES`, so a guard whose subject were "everything untracked under `.jigc/`" would
/// swallow — and re-open — the two defects M46, M47 and M49 built the other two guards
/// for. Both exclusions are asserted here as answers, not as reasoning.
///
/// **What this adds over `root_knob_rules.rs` / `uninstall_workbench_subject.rs`** is the
/// walk the family was found by, in **one repository**: the knob that would have homed
/// managed docs inside the tree the teardown removes whole is refused *before* an adopter
/// can choose it, and the teardown then gives three different, consistent answers about
/// three different kinds of byte — a staged doc (its own door's code), a hand-dropped
/// untracked workbench file (this subject's code), and the same file once an index has a
/// copy of it (narrated, and taken).
#[test]
fn neither_root_knob_lies_and_the_teardown_answers_per_kind_of_byte() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();

    // ── The registry half: both knobs, both rules. ──
    let absolute = repo.to_string_lossy().into_owned();
    fs::write(repo.join("a-file"), "not a directory\n").expect("write the file-shaped root");
    let cells: [(&str, &str); 3] = [
        (".jigc", "config.workbench-root"),
        (absolute.as_str(), "config.unusable-root"),
        ("a-file", "config.unusable-root"),
    ];
    for knob in ROOT_KNOBS {
        for (value, code) in cells {
            let out = corpus.jigc(&["config", "set", knob, value]);
            let text = surface(&out);
            let shown = format!("jigc config set {knob} {value}");
            assert!(
                !out.status.success(),
                "{shown}: a root that leaves the store lying must be refused before it \
                 lands\n{text}",
            );
            assert!(
                text.contains(code),
                "{shown}: the refusal carries `{code}`\n{text}",
            );
            let read_back = corpus.jigc_ok(&["config", "get", knob]);
            assert!(
                !read_back.contains(value),
                "{shown}: a refused value must not land — `config get {knob}` reads back \
                 `{read_back}`",
            );
        }
    }
    assert_eq!(
        ROOT_KNOBS.len(),
        2,
        "the registry ships two root knobs; a change to that count is a change to this \
         family's axis and must be read, not absorbed",
    );

    // ── The derivation half: `.jigc/` bytes, one answer per kind. ──
    // The transient prefixes are READ off the installed `.gitignore`, which `setup` writes
    // from the production constant — so the complement is derived here exactly as the door
    // derives it, never restated.
    let ignored = fs::read_to_string(repo.join(".jigc").join(".gitignore"))
        .expect("setup writes the workbench `.gitignore`");
    let transient: Vec<String> = ignored
        .lines()
        .map(|line| line.trim_end_matches('/').to_string())
        .filter(|line| !line.is_empty())
        .collect();
    assert!(
        transient.iter().any(|p| p == "tasks") && transient.iter().any(|p| p == "worktrees"),
        "the transient set is the one the two shipped guards own:\n{ignored}",
    );

    // (a) A path INSIDE a transient prefix is its own door's subject, never this one.
    let task = corpus.start_workflow("single-task", "the workbench subject");
    let staged_refusal = corpus.jigc(&["uninstall"]);
    let staged_text = surface(&staged_refusal);
    assert!(
        !staged_refusal.status.success(),
        "an open task's staged prose refuses the teardown:\n{staged_text}",
    );
    assert!(
        staged_text.contains("uninstall.staged-prose")
            && !staged_text.contains("uninstall.untracked-workbench-file"),
        "a path under `tasks/` is answered by the guard that owns it, not by the \
         complement's:\n{staged_text}",
    );
    corpus.jigc_ok(&["task", "discard", &task, "--force"]);

    // (b) A hand-dropped file OUTSIDE every transient prefix, that no index has a copy of.
    let dropped = repo.join(".jigc").join("notes.md");
    assert!(
        !transient
            .iter()
            .any(|prefix| dropped.starts_with(repo.join(".jigc").join(prefix))),
        "the planted path must sit outside every transient prefix, or the arm proves \
         nothing about the complement",
    );
    fs::write(&dropped, "bytes no commit has a copy of\n").expect("plant the workbench file");
    let refused = corpus.jigc(&["uninstall"]);
    let refused_text = surface(&refused);
    assert!(
        !refused.status.success(),
        "bytes no index has a copy of refuse the teardown:\n{refused_text}",
    );
    assert!(
        refused_text.contains("uninstall.untracked-workbench-file")
            && refused_text.contains(".jigc/notes.md"),
        "the refusal names its code and the path it will not spend:\n{refused_text}",
    );
    assert!(dropped.is_file(), "a refused teardown removes nothing");
    assert!(
        repo.join(".jigc").is_dir(),
        "…including the workbench itself"
    );

    // (c) The same bytes, once an index has a copy: narrated, and taken.
    git(&repo, &["add", ".jigc/notes.md"]);
    let removed = corpus.jigc(&["uninstall"]);
    let removed_text = surface(&removed);
    assert!(
        removed.status.success(),
        "a recoverable file is narrated, not refused:\n{removed_text}",
    );
    assert!(
        removed_text.contains(".jigc/notes.md") && removed_text.contains("git checkout"),
        "what the guards let through, the teardown names — with the command that brings \
         it back:\n{removed_text}",
    );
    assert!(
        !repo.join(".jigc").exists(),
        "the teardown it was allowed to perform, it performed",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 7 — the adapter floor, a DELIBERATELY MANUFACTURED two-member set
// ═════════════════════════════════════════════════════════════════════════════

/// The two human-owned destroying doors the profile denies.
///
/// # This is a MANUFACTURED set, not a registry read
///
/// There is a registry next door — `cli::milestone::DESTROYING_DOORS`, six members since
/// M52 Increment 4 — and it is **the wrong set**, which is why this arm departs from the
/// pattern on purpose, as flow 49's arm 6 did. Two of that registry's members must not be
/// denied for reasons the record states: `milestone finalize` destroys worktrees on the
/// ordinary **success** path, and `milestone provision` was refused from the carve-out on
/// measured grounds — M48's fail-closed classifier already guards it, and denying it would
/// park an unattended fix round on a prompt nobody is watching
/// (`completions/artifacts/M50/settle-record.md` → D9, re-settled after the design review's
/// B6). The other two — `jigc task discard` and `jigc task finalize` — joined that registry
/// when M52 generalized its subject from a worktree to a **destroyed path**, after this
/// floor was decided, and they are out of it by the same rule the two above are out of it
/// by: membership here is a **decision about who owns the act**, not a property any table
/// computes: each of these spends bytes that exist in no object
/// database, and the `--force` past their refusal is the human's consent, not the agent's.
/// A registry loop written here would deny four doors and break the wave's own fan-out.
const DENIED_DOORS: [&str; 2] = ["Bash(jigc uninstall:*)", "Bash(jigc milestone discard:*)"];

/// A deny entry the *user* owns — the bytes an additive merge must leave standing, and the
/// bytes an `uninstall` must not take with the floor.
const USER_DENY: &str = "Bash(terraform destroy:*)";

/// **Arm 7** — the safety floor is installed, additive, and removed cleanly.
///
/// **What this adds over the adapter suites** is the floor's whole life in one repository:
/// `jigc setup` merges both entries into a settings file that **already carries a user's
/// own deny entry**, that entry survives, and `jigc uninstall` removes exactly the profile's
/// floor and leaves it standing. The additivity is the load-bearing half — `inject_deny` is
/// purely additive, which is what makes this reach installed repos at all, and M48's
/// refuse-to-clobber posture forbids pruning a user-owned file.
#[test]
fn the_adapter_floor_denies_the_two_human_owned_destroyers_additively() {
    let repo = TempDir::new("deny-repo");
    let home = TempDir::new("deny-home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "t@example.com"]);
    git(repo.path(), &["config", "user.name", "T"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    fs::create_dir_all(repo.path().join(".claude")).expect("mk .claude");
    let settings = repo.path().join(".claude").join("settings.json");
    fs::write(
        &settings,
        format!(
            "{{\n  \"permissions\": {{\n    \"deny\": [\n      \"{USER_DENY}\"\n    ]\n  }}\n}}\n"
        ),
    )
    .expect("write the user's own settings");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-qm", "initial"]);

    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    };
    let installed = run(&["setup"]);
    assert!(
        installed.status.success(),
        "setup installs the adapter:\n{}",
        surface(&installed),
    );

    let deny_list = |what: &str| -> Vec<String> {
        let body = fs::read_to_string(&settings).expect("read the settings file");
        let value: Value = serde_json::from_str(&body).unwrap_or_else(|e| {
            panic!("{what}: the settings file stays valid JSON ({e}):\n{body}")
        });
        value["permissions"]["deny"]
            .as_array()
            .unwrap_or_else(|| panic!("{what}: `permissions.deny` is an array:\n{body}"))
            .iter()
            .map(|entry| entry.as_str().unwrap_or_default().to_string())
            .collect()
    };

    let after_setup = deny_list("after setup");
    for door in DENIED_DOORS {
        assert!(
            after_setup.iter().any(|entry| entry == door),
            "`{door}` joins the deny floor — the profile has no prompt, so blocking is the \
             semantics the human's ownership earns; got:\n{after_setup:#?}",
        );
    }
    assert!(
        after_setup.iter().any(|entry| entry == USER_DENY),
        "the merge is ADDITIVE: the user's own deny entry survives it; got:\n{after_setup:#?}",
    );

    let removed = run(&["uninstall"]);
    assert!(
        removed.status.success(),
        "the teardown runs over a clean install:\n{}",
        surface(&removed),
    );
    let after_uninstall = deny_list("after uninstall");
    for door in DENIED_DOORS {
        assert!(
            !after_uninstall.iter().any(|entry| entry == door),
            "`{door}` leaves with the install it came in with; got:\n{after_uninstall:#?}",
        );
    }
    assert!(
        after_uninstall.iter().any(|entry| entry == USER_DENY),
        "…and the user's own entry is not collateral; got:\n{after_uninstall:#?}",
    );
}
