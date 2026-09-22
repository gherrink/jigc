//! **The M52 wave's done-picture acceptance suite** — the wave driven end to end through
//! the **real `jigc` binary** (`design/worked-examples.md` → flow 53; roadmap → Milestone
//! 52, Increment 11; the arm set is `completions/artifacts/M52/acceptance-design.md` →
//! The arms, adopted at `settle-record.md` → D12 and amended at the design review §7/§12/§15).
//!
//! **The claim the wave proves is one claim:** *no byte dies and no third party's bytes are
//! silently discarded at exit 0 behind a committing, destroying or moving door; every
//! repository posture and every route a caller can reach answers with a code and a
//! followable route; and every surface 1.0.0 pins says what the binary does — with each fix
//! complete over its class's axis, machine-checkable.*
//!
//! Increments 1–10 shipped each fix with its own axis suite; this suite is the **composite
//! acceptance** that ties the wave into seven done-picture arms — **each arm stating which
//! kind of set it iterates**, and each stating what it adds over the axis suite beside it,
//! because an arm that re-runs a shipped axis proves the axis twice and the wave once.
//!
//! The seven arms:
//!
//!   (1) **The rollback family** — [`ROLLBACK_POPULATIONS`], a **code-side registry** minted
//!       by the wave, whose membership is a counted source scan over the production restore
//!       units. *Adds over `record_rollback_conflict.rs` / `record_flip_rollback.rs` /
//!       `rename_rollback_conflict.rs` / `config_relocation_rollback.rs` / `migrate_rollback.rs`:*
//!       each of those owns **one** population's cells; this arm drives **every row of the
//!       registry** — one cell per row, fenced ⇔ so a row added without a cell reddens — under
//!       the design's named racer, and asserts the one composite fact none of them can: that
//!       the **discipline the row declares is the behaviour the door performs**, and that at
//!       every cell the `--format json` reject stream is **exactly one document**.
//!
//!   (2) **The posture family × the commit-on-behalf class** — [`InProgress::ALL`], the
//!       family's **defining case-set**, crossed with [`BEHALF_DOORS`]' acting members, a
//!       **code-side registry**, over Increment 2's git-state builder. *Adds over
//!       `posture_door_axis.rs`:* that suite drives the **git-state** axis and owns each
//!       cell's route text; this arm drives the **operation** axis — every member of
//!       `InProgress::ALL`, each reached by the fixture state that produces it — and asserts
//!       the two facts a per-state sweep cannot: that `jigc task validate` **previews** the
//!       refusal `finalize` would give, and that every operation's route, run verbatim,
//!       leaves the repository with no operation in progress.
//!
//!   (3) **The destroying subject** — [`TASK_AREA_FILES`]' complement over the tree, a
//!       **manufactured shape space that says it is manufactured** (declaredness of a file
//!       shape is a property of a fixture no registry knows), crossed with
//!       [`DESTROYING_DOORS`] read through its [`Disposition`] axis. *Adds over
//!       `staged_prose_consent_axis.rs` / `finalize_displacement.rs` /
//!       `milestone_boundary_displacement.rs`:* those own one door's cells each; this arm
//!       drives **every member of the door registry through its own disposition** — the
//!       consenting doors twice (without the consent and with it), the displacing doors once
//!       — over **one** planted shape space, and adds the zero-false-fire control that a full
//!       lifecycle never has jigc call its own files foreign.
//!
//!   (4) **The corpus walk** — the `{location, placement}²` **home-pair set**, the class's
//!       defining case-set matched exhaustively on manufactured packs, crossed with the bump
//!       kind. *Adds over `migrate_corpus_home_pairs.rs`:* that suite owns the eight cells'
//!       pre-states and the missing-snapshot arm; this arm asserts the **adopter's done
//!       picture** at every cell — the doc lands at the current home, the stamp is current,
//!       `jigc validate` exits 0, and the **read surface addresses the doc at its new home**,
//!       which is the half a migration report cannot claim for itself. The **vacated-home**
//!       half rides here as its own derivation: the fixed-identity home set, read off the
//!       same pinned projection, each home emptied by one ordinary human act in the
//!       fresh-clone shape, with the filled corpus as the silent control.
//!
//!   (5) **The fixed identity** — the fixed-identity doctype set **derived** from both packs'
//!       doctypes through the **pinned projection** `jigc doc schema --format json` reports at
//!       `contract-version` 7 (a derivation stated as one, read off the contract surface the
//!       engine predicate feeds rather than off the predicate, so the set the doors refuse
//!       over and the set a driver can discover are asserted to be one set), crossed with
//!       [`DOCTYPE_DOORS`]' `Address` rows. *Adds over
//!       `fixed_identity_axis.rs`:* that suite owns the manufactured `location:`+`singleton:`
//!       doctype and each door's own refusal; this arm drives **every shipped fixed-identity
//!       doctype at every address door of one corpus** and asserts the projection half beside
//!       it — `doc schema --format json` at `contract-version` 7 carries `identity` and `home`
//!       for **every** doctype of both packs, so the set the doors refuse over and the set the
//!       contract advertises are the same set.
//!
//!   (6) **The pre-dispatch funnel × the verb surface** — [`PRE_DISPATCH_FAULTS`], a
//!       **code-side registry** (test-side, minted by the wave), crossed with [`VERB_KINDS`]
//!       under the phase relation, plus [`ENVELOPE_ARMS`] **where membership is the
//!       assertion**. *Adds over `pre_dispatch_faults.rs`:* that suite owns the per-cell
//!       expectation table; this arm asserts the two registry-crossing facts — every document
//!       a faulting cell emits matches a **declared** `ENVELOPE_ARMS` key set, and the
//!       universal fault is answered by **every leaf** on the declared reject arm — and
//!       carries D13 lead 1's owed **debug-posture seam suite** for the `Route` span fence,
//!       plus Increment 8's not-set-up rider at the `milestone` family.
//!
//!   (7) **The composed doors** — the verb-routed workflow set **derived** from both packs'
//!       `suppressed:` blocks (a derivation stated as one), crossed with the two compose
//!       doors, plus the empty-enumeration set. *Adds over `verb_routed_compose.rs` and
//!       `implement_from_spec.rs`:* those own the refusal's shape and the empty-case clauses;
//!       this arm drives the **agent's walk** — every verb-routed member refused at both
//!       compose doors with a route that is a *runnable* `jigc` command line, and the
//!       legitimately-empty render stating its empty case rather than pointing at a list that
//!       is not there.
//!
//! **What gets no arm, recorded as a decision** (the M46 Increment 9 / M48 Increment 11 /
//! M49 Increment 12 / M51 Increment 9–11 precedent — *an increment that mints no verb,
//! finding or route carries nothing for a done-picture walk to reach, and manufacturing an
//! arm would be a walk written to have an arm rather than to prove a claim*):
//!
//!   * **Increment 2** is a **fixture builder**. It ships no verb and no finding; it is the
//!     substrate arm 2 iterates, and `git_state_fixtures.rs` is its own acceptance.
//!   * **Increment 8** mints no arm of its own: its not-set-up fault row rides **arm 6** (it
//!     is a pre-dispatch state answered at the milestone doors) and its identity cells ride
//!     **arm 5**'s set.
//!   * **Increment 10** is the surface tier, the guide batch and the record corrections —
//!     whose only change is what a sentence *says*, and whose behaviour the arms above
//!     already reach.
//!   * **Increment 11** is this suite, the two ledgers, the goldens and the fold-back.
//!
//! **Bounds, stated here rather than discovered later:**
//!
//!   * The arms are **headless by construction**: no genuine concurrent process races a
//!     population. The racer is the design's named one — an in-transaction `pre-commit` hook
//!     that edits the restored path and exits 1 — and the one population whose window spawns
//!     no hook at all says so on its own cell rather than being dropped from the sweep.
//!   * A `chmod 000` cell declares the **CI platform bound**: it passes vacuously as root.
//!   * The git-state cells are **git 2.54.0's on-disk contract**
//!     (`support::git_state::EXPECTATIONS`; `decisions-pending.md` → the rc.16 wave,
//!     deferral (a)).
//!   * An arm that reds is **a finding about the wave landed, with its reason** — never a
//!     narrowed set. A cell that cannot be driven is stated on the cell with its datum.
//!
//! Isolation: every arm rides the shared [`support::trial_corpus`] substrate or
//! [`support::committing_doors`]' per-door fixtures, both of which `git init` a throwaway
//! repo, set a per-repo git identity, repoint `$HOME` and scrub `JIGC_PACK_DIR` unless the
//! arm deliberately supplies a fixture pack.

#![cfg(unix)]

use crate::support;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// A throwaway directory that removes itself on drop — for the arms that build a repo, a
/// pack or a canary by hand rather than riding the shared substrates.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow53-{tag}-{}-{:?}",
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

/// Parse a payload as JSON, surfacing the bytes on failure.
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

/// The **one** JSON reject document a `--format json` run put on stderr, with stdout
/// asserted empty — the stream discipline every reject in this suite is held to
/// (`design/command-output-contract.md` → Stream discipline).
fn one_reject_document(out: &Output, what: &str) -> Value {
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        !out.status.success(),
        "[{what}] the run must refuse; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.trim().is_empty(),
        "[{what}] a reject puts nothing on stdout; it printed:\n{stdout}",
    );
    serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
        panic!("[{what}] a reject is EXACTLY ONE JSON document on stderr ({err}); got:\n{stderr}")
    })
}

/// Every `code` the reject document's `findings` array carries.
fn finding_codes(doc: &Value) -> Vec<String> {
    doc.get("findings")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("the reject carries a `findings` array; got:\n{doc:#}"))
        .iter()
        .filter_map(|f| f.get("code").and_then(Value::as_str).map(str::to_owned))
        .collect()
}

/// Write an executable `pre-commit` hook with `body` — the design's named racer, installed
/// where it runs: inside the transaction, between jigc's write and its rollback.
fn install_hook(repo: &Path, body: &str) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(&hook, format!("#!/bin/sh\n{body}\n")).expect("write the pre-commit hook");
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — the rollback family (a CODE-SIDE REGISTRY)
// ═════════════════════════════════════════════════════════════════════════════

use cli::rollback::{
    CONFIG_DOOR, ConflictDoor, Discipline, FINALIZE_DOOR, MILESTONE_DOOR, Population, RENAME_DOOR,
    ROLLBACK_POPULATIONS,
};
use support::committing_doors::{DoorCase, drive};
use support::trial_corpus::{State, TrialCorpus};

/// The line the racing hook writes at the restored path — the third party's bytes, which
/// must be on disk however the rollback goes.
const RACE_LINE: &str = "<!-- flow 53: raced by a concurrent editor -->";

/// The name the racing hook plants **inside** a minted working area — the bytes a
/// [`Discipline::MintedSet`] unwind meets when it removes the area it minted.
const PLANTED: &str = "FOREIGN-SCRATCH.txt";

/// The foreign document arm 1's two migration cells migrate — the `State::Migrated` shape,
/// re-built here because the population under test is the *in-flight* transaction, and the
/// shared builder's corpus has already landed.
const FOREIGN_VISION: &str = "\
# Product Direction

We build a deterministic context compiler.

## Principles

Structure belongs to the CLI; prose belongs to the model.
";

/// The `doc author` batch payload the migration cells rewrite the foreign source into.
const VISION_PAYLOAD: &str = "\
title: Vision
sections:
  - id: thesis
    set:
      thesis: |-
        <<We build a deterministic context compiler.>>
  - id: invariants
    set:
      invariants: |-
        <<Structure belongs to the CLI; prose belongs to the model.>>
  - id: open-questions
    set:
      open-questions: |-
        <<Which domains earn a pack of their own.>>
";

/// How one population's row is driven.
///
/// The variants are the **disciplines**, not the doors: what an arm asserts at a cell is
/// decided by what the row promises keeps its restore honest, which is the whole point of
/// the registry carrying a discipline per row.
enum Drive {
    /// A [`Discipline::FileCas`] row reached through a **committing** door, raced by the
    /// design's named racer at the path the population restores.
    Raced {
        /// The verb key `support::committing_doors::drive` builds the fixture under.
        verb: &'static str,
        /// The repo-relative path the racer writes at — the population's own subject.
        target: &'static str,
        /// The door whose `<door>.rollback-conflict` identity the cell must carry.
        door: ConflictDoor,
        /// Work the cell does to the fixture **before** the racer is installed, when the
        /// door-case builder's repo does not yet reach the population. `None` where the
        /// shared fixture already does.
        prepare: Option<fn(&Path)>,
    },
    /// A [`Discipline::FileCas`] row reached through a **migration** finalize, whose
    /// fixture the shared door-case builder does not carry: the promote destination and the
    /// retired original are both written inside `jigc task finalize --approve`.
    RacedMigration {
        /// The repo-relative path the racer writes at.
        target: &'static str,
        /// The door whose identity the cell must carry.
        door: ConflictDoor,
    },
    /// A [`Discipline::MintedSet`] row: the racer plants a file **inside** the area the door
    /// minted, so the unwind meets bytes jigc did not write.
    Planted {
        /// The verb key the fixture is built under.
        verb: &'static str,
        /// The repo-relative parent of the minted areas to plant in.
        areas: &'static str,
        /// The blocking code the unwind must raise over the area it would not empty.
        code: &'static str,
    },
    /// A [`Discipline::FileCas`] row whose transaction window **spawns no subprocess**, so
    /// the design's named racer cannot enter it. The cell is driven at the row's own
    /// rollback **trigger** instead, and says here why it is not raced —
    /// `config_relocation_rollback.rs` owns the raced cell, manufactured at the CAS seam.
    UnracedTrigger { reason: &'static str },
    /// A [`Discipline::DoorGuard`] row: driven at the guard, which is the mechanism.
    Guard,
    /// A [`Discipline::Declared`] row: no mechanism, by decision. The cell asserts the two
    /// halves the registry's own contract makes checkable — the reason, and the condition
    /// that reopens it.
    Declared,
}

/// One driven cell: a population id, and how its row is driven.
struct RollbackCell {
    /// The [`ROLLBACK_POPULATIONS`] row's own id.
    id: &'static str,
    drive: Drive,
}

/// **Every row of the registry, one cell each** — fenced ⇔ against [`ROLLBACK_POPULATIONS`]
/// by [`the_rollback_cells_are_the_registry`], so a population added without a driven cell
/// reddens and a cell naming no row reddens.
const ROLLBACK_CELLS: &[RollbackCell] = &[
    RollbackCell {
        id: "config-layer-worktree",
        drive: Drive::Raced {
            verb: "jigc task finalize",
            target: ".jigc/.gitignore",
            door: FINALIZE_DOOR,
            // The population is the **amend**, and a corpus `jigc setup` has just written
            // is already at the current entry set and the current stamp — so the amend
            // writes nothing and the cell would pass over an entry that never existed.
            // Both files are put one upgrade behind, which is the only shape in which the
            // amend writes at all (`config_layer_preimage.rs` → `COMMITTED_ENTRIES`).
            prepare: Some(stale_config_layer),
        },
    },
    RollbackCell {
        id: "promote-destination",
        drive: Drive::RacedMigration {
            target: "VISION.md",
            door: FINALIZE_DOOR,
        },
    },
    RollbackCell {
        id: "retired-original",
        drive: Drive::RacedMigration {
            target: "docs/direction.md",
            door: FINALIZE_DOOR,
        },
    },
    RollbackCell {
        id: "milestone-record",
        drive: Drive::Raced {
            verb: "jigc milestone create",
            target: "docs/milestone-records/cache-rework.md",
            door: MILESTONE_DOOR,
            prepare: None,
        },
    },
    RollbackCell {
        id: "fan-out-record-flip",
        drive: Drive::Raced {
            verb: "jigc milestone finalize (squash: true)",
            target: "docs/milestone-records/cache-rework.md",
            door: MILESTONE_DOOR,
            prepare: None,
        },
    },
    RollbackCell {
        id: "rename-worktree",
        drive: Drive::Raced {
            verb: "jigc rename",
            target: "docs/decisions/beta-decision.md",
            door: RENAME_DOOR,
            prepare: None,
        },
    },
    RollbackCell {
        id: "created-doc-staged-write",
        drive: Drive::Declared,
    },
    RollbackCell {
        id: "milestone-mint-area",
        drive: Drive::Planted {
            verb: "jigc milestone create",
            areas: ".jigc/milestones",
            code: "milestone.foreign-bytes",
        },
    },
    RollbackCell {
        id: "unrecorded-seed-areas",
        drive: Drive::Planted {
            verb: "jigc milestone add-from-spec",
            areas: ".jigc/tasks",
            code: "milestone.foreign-bytes",
        },
    },
    RollbackCell {
        id: "config-root-relocation",
        drive: Drive::UnracedTrigger {
            reason: "the `jigc config set <root-knob>` window spawns `git mv` and `git \
                     restore` and NO hook at all, so the design's named racer cannot enter \
                     it; `config_relocation_rollback.rs` drives the raced cell at the CAS \
                     seam and says it is manufactured. This cell drives the row's own \
                     rollback TRIGGER instead — a knob write that fails after both move \
                     floors ran — which is the state the population was minted carrying.",
        },
    },
    RollbackCell {
        id: "setup-install-path",
        drive: Drive::Guard,
    },
];

/// The registry row behind a cell.
fn population(id: &str) -> &'static Population {
    ROLLBACK_POPULATIONS
        .iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("`{id}` is a row of `ROLLBACK_POPULATIONS`"))
}

/// The cell table **is** the registry, in both directions.
#[test]
fn the_rollback_cells_are_the_registry() {
    let declared: BTreeSet<&str> = ROLLBACK_POPULATIONS.iter().map(|p| p.id).collect();
    let driven: BTreeSet<&str> = ROLLBACK_CELLS.iter().map(|c| c.id).collect();
    assert_eq!(
        driven, declared,
        "every capture/restore population owes a driven cell here, and this table may claim \
         no population the registry does not have — a class nobody enumerates is a class a \
         fix is cut short of, which is this wave's whole subject",
    );
    // …and each cell is driven the way its row's DISCIPLINE says it must be, so a row
    // demoted to another discipline reddens instead of passing under the old assertion.
    for cell in ROLLBACK_CELLS {
        let row = population(cell.id);
        let ok = matches!(
            (&cell.drive, row.discipline),
            (
                Drive::Raced { .. } | Drive::RacedMigration { .. } | Drive::UnracedTrigger { .. },
                Discipline::FileCas
            ) | (Drive::Planted { .. }, Discipline::MintedSet(_))
                | (Drive::Guard, Discipline::DoorGuard(_))
                | (Drive::Declared, Discipline::Declared { .. })
        );
        assert!(
            ok,
            "`{}` declares {:?} and is driven as a different discipline — a row whose \
             discipline moved owes a different cell, not a silent pass",
            cell.id, row.discipline,
        );
    }
}

/// Drive one migration fixture to the point of finalize: a committed foreign source, a
/// migration task whose managed `vision` is authored, and a conformant commit doc.
fn migration_fixture() -> (TrialCorpus, String) {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let foreign = repo.join("docs").join("direction.md");
    fs::create_dir_all(foreign.parent().expect("the foreign source has a parent"))
        .expect("mk the foreign source's dir");
    fs::write(&foreign, FOREIGN_VISION).expect("write the foreign source");
    corpus.git(&["add", "docs/direction.md"]);
    corpus.git(&["commit", "-q", "-m", "add the direction doc"]);

    let composed = corpus.jigc_ok(&["migrate", "docs/direction.md", "--as", "vision"]);
    let task = composed
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("`jigc migrate` announces the minted task id")
        .trim()
        .to_owned();
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "vision",
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        VISION_PAYLOAD,
    );
    for (field, value) in [("type", "docs"), ("scope", "vision")] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#{field}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    corpus.set_slot(
        &format!("commit:{task}#summary"),
        &task,
        "migrate the direction doc\n",
    );
    corpus.set_slot(
        &format!("commit:{task}#body"),
        &task,
        "Driven by flow 53 arm 1.\n",
    );
    (corpus, task)
}

/// **Every population of the rollback registry, driven at its own door under its own
/// discipline.**
///
/// The composite this adds over the five per-population suites: the registry is walked as a
/// registry, so the *discipline a row declares* and the *behaviour its door performs* are
/// asserted against each other at every row — and at every driven cell the `--format json`
/// reject stream is **exactly one document**, which is the fact a driver's own `json.loads`
/// depends on and which no single population's suite can claim for the class.
#[test]
fn every_rollback_population_keeps_the_bytes_its_discipline_promises() {
    for cell in ROLLBACK_CELLS {
        let id = cell.id;
        match &cell.drive {
            Drive::Raced {
                verb,
                target,
                door,
                prepare,
            } => {
                let case: DoorCase = drive(verb);
                let repo = case.repo.path();
                let home = case.home.path();
                if let Some(prepare) = prepare {
                    prepare(repo);
                }
                let absolute = repo.join(target);
                install_hook(
                    repo,
                    &format!("printf '%s\\n' '{RACE_LINE}' >> {absolute:?}\nexit 1\n"),
                );
                let mut argv: Vec<&str> = case.driven.iter().map(String::as_str).collect();
                argv.extend_from_slice(&["--format", "json"]);
                let out = jigc(repo, home, &argv);
                let doc = one_reject_document(&out, id);
                assert!(
                    finding_codes(&doc).iter().any(|code| code == door.code),
                    "[{id}] a raced restore must raise this door's own `{}`; got:\n{doc:#}",
                    door.code,
                );
                let after = fs::read_to_string(&absolute).unwrap_or_else(|err| {
                    panic!("[{id}] the raced path `{target}` must survive the rollback: {err}")
                });
                assert!(
                    after.contains(RACE_LINE),
                    "[{id}] the racer's line must still be in `{target}`; it now reads:\n{after}",
                );
            }
            Drive::RacedMigration { target, door } => {
                let (corpus, task) = migration_fixture();
                let repo = corpus.repo();
                let absolute = repo.join(target);
                install_hook(
                    &repo,
                    &format!("printf '%s\\n' '{RACE_LINE}' >> {absolute:?}\nexit 1\n"),
                );
                let out =
                    corpus.jigc(&["task", "finalize", &task, "--approve", "--format", "json"]);
                let doc = one_reject_document(&out, id);
                assert!(
                    finding_codes(&doc).iter().any(|code| code == door.code),
                    "[{id}] a raced restore must raise `{}`; got:\n{doc:#}",
                    door.code,
                );
                let after = fs::read_to_string(&absolute).unwrap_or_else(|err| {
                    panic!("[{id}] the raced path `{target}` must survive the rollback: {err}")
                });
                assert!(
                    after.contains(RACE_LINE),
                    "[{id}] the racer's line must still be in `{target}`; it now reads:\n{after}",
                );
            }
            Drive::Planted { verb, areas, code } => {
                let case: DoorCase = drive(verb);
                let repo = case.repo.path();
                let home = case.home.path();
                let root = repo.join(areas);
                install_hook(
                    repo,
                    &format!(
                        "for d in {root:?}/*/; do [ -d \"$d\" ] && printf 'agent scratch\\n' \
                         > \"${{d}}{PLANTED}\"; done\nexit 1\n"
                    ),
                );
                let mut argv: Vec<&str> = case.driven.iter().map(String::as_str).collect();
                argv.extend_from_slice(&["--format", "json"]);
                let out = jigc(repo, home, &argv);
                let doc = one_reject_document(&out, id);
                assert!(
                    finding_codes(&doc).iter().any(|c| c == code),
                    "[{id}] an unwind that meets bytes jigc did not write must name the area \
                     with `{code}` rather than remove them; got:\n{doc:#}",
                );
                assert!(
                    !planted_survivors(&root).is_empty(),
                    "[{id}] the planted bytes under `{areas}/` must survive the unwind — a \
                     `MintedSet` row removes the door's OWN files and then the directory \
                     non-recursively, so a third party's file is kept and named",
                );
            }
            Drive::UnracedTrigger { reason } => {
                assert!(
                    reason.len() > 40,
                    "[{id}] a cell that is not raced states WHY on the cell",
                );
                let corpus = TrialCorpus::build(State::CommittedSingletons);
                let repo = corpus.repo();
                // Seed the project manifest, then make it unwritable: the knob write then
                // fails AFTER both move floors have run, which is the state the population
                // was minted carrying. (The `chmod` cell declares the CI platform bound —
                // it passes vacuously as root.)
                corpus.jigc_ok(&["config", "set", "default-workflow", "single-task"]);
                let manifest = repo.join(".jigc").join("config").join("manifest.yaml");
                let mut perms = fs::metadata(&manifest).expect("the manifest").permissions();
                perms.set_readonly(true);
                fs::set_permissions(&manifest, perms).expect("freeze the manifest");

                let out = corpus.jigc(&[
                    "config",
                    "set",
                    "placement-root",
                    "notes",
                    "--format",
                    "json",
                ]);
                let refused = !out.status.success();
                {
                    let mut perms = fs::metadata(&manifest).expect("the manifest").permissions();
                    #[allow(clippy::permissions_set_readonly_false)]
                    perms.set_readonly(false);
                    let _ = fs::set_permissions(&manifest, perms);
                }
                if refused {
                    let doc = one_reject_document(&out, id);
                    assert!(
                        finding_codes(&doc)
                            .iter()
                            .any(|c| c.starts_with("config.") || c == CONFIG_DOOR.code),
                        "[{id}] a failed knob write must refuse under a `config.*` identity; \
                         got:\n{doc:#}",
                    );
                }
                assert!(
                    repo.join("docs").join("roadmap.md").exists(),
                    "[{id}] every relocated doc goes back to the home the un-landed knob \
                     still points at — a store whose docs moved under a knob that never \
                     landed is the defect this population was minted carrying",
                );
                assert!(
                    !repo.join("notes").join("roadmap.md").exists(),
                    "[{id}] …and nothing is left at the destination the transaction abandoned",
                );
            }
            Drive::Guard => {
                let Discipline::DoorGuard(code) = population(id).discipline else {
                    panic!("[{id}] a `Guard` cell's row declares `DoorGuard`");
                };
                // The adopter's own bytes at a path `jigc setup` would install over: the
                // door refuses BEFORE the first write rather than restoring after one.
                let repo = TempDir::new("setup-guard");
                let home = TempDir::new("setup-guard-home");
                git(repo.path(), &["init", "-q", "-b", "main", "."]);
                git(repo.path(), &["config", "user.email", "test@example.com"]);
                git(repo.path(), &["config", "user.name", "Test"]);
                fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
                git(repo.path(), &["add", "-A"]);
                git(repo.path(), &["commit", "-qm", "initial"]);
                let adopters = "# Our own house rules\n";
                fs::write(repo.path().join("CLAUDE.md"), adopters).expect("the adopter's file");
                git(repo.path(), &["add", "CLAUDE.md"]);
                git(repo.path(), &["commit", "-qm", "our own rules"]);
                fs::write(repo.path().join("CLAUDE.md"), "# edited since\n").expect("edit it");

                let out = jigc(repo.path(), home.path(), &["setup", "--format", "json"]);
                let doc = one_reject_document(&out, id);
                assert!(
                    finding_codes(&doc).iter().any(|c| c == code),
                    "[{id}] the guard IS the mechanism — the door must refuse with `{code}`; \
                     got:\n{doc:#}",
                );
                assert_eq!(
                    fs::read_to_string(repo.path().join("CLAUDE.md")).expect("the adopter's file"),
                    "# edited since\n",
                    "[{id}] …and the adopter's bytes are untouched, which is what refusing \
                     before the first write buys",
                );
            }
            Drive::Declared => {
                let Discipline::Declared {
                    reason,
                    reopens_when,
                } = population(id).discipline
                else {
                    panic!("[{id}] a `Declared` cell's row declares `Declared`");
                };
                assert!(
                    !reason.trim().is_empty() && !reopens_when.trim().is_empty(),
                    "[{id}] a row with no mechanism carries the reason AND the condition \
                     that makes the reason false — a declared hole with no reopening \
                     condition is a silent one",
                );
            }
        }
    }
}

/// Put the two files `finalize`'s config-layer amend rewrites one upgrade behind, so the
/// amend genuinely writes and the population has a pre-image to compare against.
fn stale_config_layer(repo: &Path) {
    let jigc_dir = repo.join(".jigc");
    fs::write(
        jigc_dir.join(".gitignore"),
        "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\n",
    )
    .expect("write the one-entry-short ignore file");
    fs::write(jigc_dir.join("version"), "0.0.0-fixture-stale-stamp\n")
        .expect("write the stale version stamp");
}

/// Every planted file still under `root`'s sub-directories after an unwind.
fn planted_survivors(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return found;
    };
    for entry in entries.flatten() {
        let candidate = entry.path().join(PLANTED);
        if candidate.is_file() {
            found.push(candidate);
        }
    }
    found
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — the posture family × the commit-on-behalf class
//         (a DEFINING CASE-SET × a CODE-SIDE REGISTRY)
// ═════════════════════════════════════════════════════════════════════════════

use cli::cli::{ActsOnBehalf, BEHALF_DOORS, WORK_UNIT_ID_SLOT};
use cli::repo::{InProgress, PostureMember, posture};
use support::git_state::{self, GitState, GitStateRepo};

/// The well-formed work-unit id every `<id>` slot in a row's argv is filled with. The
/// posture guard answers before any id resolution, so the id need only be well-formed.
const AXIS_ID: &str = "axis-unit";

/// The phrase the route mold puts in front of the command that **abandons** the operation
/// — the one command runnable from the state as jigc found it, since every operation
/// fixture holds an un-concluded operation and concluding one needs the user's own
/// resolution first. What is duplicated here is the parse hint, never the command.
const ABANDON_LEAD: &str = "abandon it with `";

/// The fixture state that produces `operation` — the **first** [`GitState`] whose own
/// driven declaration answers it.
///
/// The map is the fixture's, not the probe's: [`GitState::in_progress`] is what the builder
/// claims it built, asserted on disk by `git_state::assert_state`. A member of
/// [`InProgress::ALL`] no fixture produces is a **finding with its datum**, not a skip.
fn state_producing(operation: InProgress) -> GitState {
    GitState::ALL
        .iter()
        .copied()
        .find(|state| state.in_progress() == Some(operation))
        .unwrap_or_else(|| {
            panic!(
                "no `GitState` produces `{operation:?}` — the operation axis cannot be \
                 driven at that member, which is a finding about the fixture builder \
                 (`support::git_state`), never a cell to drop",
            )
        })
}

/// A runnable argv for an **acting** row, with [`WORK_UNIT_ID_SLOT`] filled; `None` for
/// [`ActsOnBehalf::Neither`], whose rows carry no argv by construction.
fn acting_argv(acts: &ActsOnBehalf) -> Option<Vec<String>> {
    let argv = match acts {
        ActsOnBehalf::Neither => return None,
        ActsOnBehalf::CommitsOnBehalf { argv, .. } | ActsOnBehalf::MovesOnBehalf { argv } => *argv,
    };
    Some(
        argv.iter()
            .map(|token| {
                if *token == WORK_UNIT_ID_SLOT {
                    AXIS_ID.to_string()
                } else {
                    (*token).to_string()
                }
            })
            .collect(),
    )
}

/// The abandoning command, read out of a **rendered** route and split into an argv through
/// a real shell — so the arm runs the bytes an operator would paste, never a reconstruction.
fn abandoning_argv(route: &str, label: &str, cwd: &Path, home: &Path) -> Vec<String> {
    let start = route.find(ABANDON_LEAD).unwrap_or_else(|| {
        panic!("the `{label}` route must offer a command that runs from THIS state; got: {route}")
    }) + ABANDON_LEAD.len();
    let rest = &route[start..];
    let end = rest
        .find('`')
        .unwrap_or_else(|| panic!("the `{label}` route's command span must close; got: {route}"));
    support::shell_words(&rest[..end], cwd, home)
}

/// **Every operation a user can leave un-concluded, at every door that acts on their
/// behalf.**
///
/// The composite this adds over `posture_door_axis.rs`: that suite iterates the **git-state**
/// axis and owns each cell's route text. This one iterates the **operation** axis — the
/// family's defining case-set — and drives the two facts a per-state sweep cannot claim: that
/// the route each operation prints, run **verbatim through a real shell**, is accepted by git
/// *and leaves no operation behind*, and that the whole sweep consumes no marker (the
/// fixture's own driven expectation is read back after every acting door has run).
#[test]
fn every_operation_refuses_at_every_acting_door_and_its_route_concludes_it() {
    let acting: Vec<(&'static [&'static str], Vec<String>)> = BEHALF_DOORS
        .iter()
        .filter_map(|row| acting_argv(&row.acts).map(|argv| (row.door, argv)))
        .collect();
    assert!(
        acting.len() >= 12,
        "the sweep means nothing over a collapsed door set — `BEHALF_DOORS` carries {} \
         acting rows",
        acting.len(),
    );

    for operation in InProgress::ALL {
        let state = state_producing(operation);
        let label = state.name();
        let fixture = GitStateRepo::build(state);
        let repo = fixture.repo();
        let home = fixture.home();
        let mut routes: BTreeSet<String> = BTreeSet::new();

        for (door, argv) in &acting {
            let shown = door.join(" ");
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = jigc(&repo, &home, &args);
            let text = surface(&out);
            assert!(
                !out.status.success(),
                "`jigc {shown}` under `{label}` must refuse — a door that acts on the user's \
                 behalf may not conclude an operation the user has not; output:\n{text}",
            );
            assert!(
                text.contains(&format!(
                    "blocking · {}",
                    PostureMember::OperationInProgress.code()
                )),
                "`jigc {shown}` under `{label}` must refuse with `{}`; output:\n{text}",
                PostureMember::OperationInProgress.code(),
            );
            assert!(
                text.contains(operation.noun()),
                "`jigc {shown}` under `{label}` must NAME the operation (`{}`) — a user in \
                 the middle of one of these operations learns which one here or nowhere; \
                 output:\n{text}",
                operation.noun(),
            );
            let route_lines: Vec<&str> = text
                .lines()
                .filter(|line| line.trim_start().starts_with("route: "))
                .collect();
            assert_eq!(
                route_lines.len(),
                1,
                "`jigc {shown}` under `{label}` prints exactly one route, never a menu; \
                 output:\n{text}",
            );
            routes.insert(route_lines[0].trim().to_string());
        }

        // Nothing was consumed: the fixture is still the fixture, by its own driven
        // expectation — markers, HEAD shape and the `git ls-files -u` count, after every
        // acting door ran in it.
        git_state::assert_state(&repo, &home, state);

        assert_eq!(
            routes.len(),
            1,
            "every acting door of `{label}` must print the SAME route — the finding has one \
             producer, so the doors cannot disagree about how a user resolves one state; \
             got: {routes:?}",
        );
        // …and that one route, read out of the door's own emitted bytes and split by a real
        // shell, is a command git accepts here and that leaves no operation behind.
        let route = routes.iter().next().expect("one route");
        let argv = abandoning_argv(route, label, &repo, &home);
        let out = Command::new(&argv[0])
            .args(&argv[1..])
            .current_dir(&repo)
            .env("HOME", &home)
            .output()
            .unwrap_or_else(|err| panic!("`{}` (the `{label}` route): {err}", argv.join(" ")));
        assert!(
            out.status.success(),
            "the `{label}` route must be a command git ACCEPTS in the repository the door \
             printed it in — `{}` exited {}:\n{}",
            argv.join(" "),
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        let after = posture(&repo);
        assert!(
            !after
                .iter()
                .any(|breach| breach.member() == PostureMember::OperationInProgress),
            "…and it must leave NO operation behind — after `{}` the `{label}` repository \
             still answers {after:?}",
            argv.join(" "),
        );
    }
}

/// **`jigc task validate` previews the refusal `jigc task finalize` would give** — at every
/// member of the operation axis.
///
/// The gate's promise is that the preview and the gate do not diverge
/// (`design/validation.md` → the previewed tier). Posture is the one pre-commit phase the
/// preview reports, and it is reported by a **separate** `repo::posture` invocation, so a
/// member the guard widens to and the preview does not is exactly the divergence this cell
/// exists to catch.
#[test]
fn task_validate_previews_the_posture_refusal_at_every_operation() {
    let base = TrialCorpus::build(State::Fresh);
    let task = base.start_workflow("single-task", "preview the posture refusal");

    for operation in InProgress::ALL {
        let state = state_producing(operation);
        let label = state.name();
        let corpus = base.copy_state();
        git_state::overlay(&corpus, state).unwrap_or_else(|why| {
            panic!("the `{label}` overlay must build over a corpus with a live task: {why}")
        });

        let out = corpus.jigc(&["task", "validate", &task]);
        let text = surface(&out);
        assert!(
            !out.status.success(),
            "`jigc task validate` under `{label}` must exit non-zero — a preview that stays \
             green over a state the gate refuses is the divergence the previewed tier \
             forbids; output:\n{text}",
        );
        assert!(
            text.contains(&format!(
                "blocking · {}",
                PostureMember::OperationInProgress.code()
            )) && text.contains(operation.noun()),
            "…and it must carry the SAME finding `finalize` would print, naming `{}`; \
             output:\n{text}",
            operation.noun(),
        );
    }
}

/// **The class that acts on nobody's behalf stays silent** — the control, driven over the
/// **whole** `Neither` set rather than a chosen member of it.
///
/// A guard that fires one class too wide is the same defect pointed the other way: a `jigc
/// describe` or a `jigc doc show` that refused because someone left a rebase half-done
/// would make the family a nuisance rather than a guard. The rows carry **no argv** by
/// construction — the type refuses to express one for a door that adjudicates nothing — so
/// the argv comes from `support::leaf_argv`, the shared minimal-argv table each consumer
/// fences ⇔ against `VERB_KINDS`.
///
/// One operation state carries the control, and the reason is stated rather than assumed:
/// the `Neither` class's answer is **state-independent by construction** (it consults no
/// posture at all), so what a second state would add is a second run of the same code path.
/// The state chosen is the conflicting merge — the member that leaves both a marker set and
/// unmerged index entries, so a door that consulted *either* half would answer here.
///
/// The sweep shares one fixture on purpose: the assertion is an **absence**, which no
/// earlier door's writes can manufacture, and the closing `assert_state` then says the whole
/// class consumed no marker either — the same claim the acting sweep makes, over the class
/// that never refuses.
/// The **declared** members of the `Neither` class that report the posture family without
/// acting on it — an exception carried as an assertion, never as a hole in the control.
///
/// `jigc task validate` commits and moves nothing, so its row is `Neither`; it invokes
/// `cli::repo::posture` **separately** and exits 1 with the identical finding, because the
/// previewed tier's promise is that the preview and the gate do not diverge (M52 Increment
/// 3; `design/validation.md` → the previewed tier, and the `GATE_COVERAGE` row that notes
/// the separate invocation). A door added here without that warrant is the family firing
/// one class too wide.
const PREVIEWING_NEITHER: &[&[&str]] = &[&["task", "validate"]];

#[test]
fn the_class_that_acts_on_nobodys_behalf_stays_silent_under_an_operation() {
    let state = GitState::Merge;
    let fixture = GitStateRepo::build(state);
    let repo = fixture.repo();
    let home = fixture.home();
    let mut driven = 0usize;

    for row in BEHALF_DOORS {
        if !matches!(row.acts, ActsOnBehalf::Neither) {
            continue;
        }
        driven += 1;
        let shown = row.door.join(" ");
        let tail = support::leaf_argv::MINIMAL_ARGV
            .iter()
            .find(|(leaf, _)| *leaf == row.door)
            .map(|(_, tail)| *tail)
            .unwrap_or_else(|| {
                panic!(
                    "`jigc {shown}` has no row in `support::leaf_argv::MINIMAL_ARGV` — the \
                     shared table is fenced against `VERB_KINDS`, so this is a leaf nobody \
                     gave an argv, not a cell to skip",
                )
            });
        let argv: Vec<&str> = row
            .door
            .iter()
            .copied()
            .chain(tail.iter().copied())
            .collect();
        let text = surface(&jigc(&repo, &home, &argv));
        if PREVIEWING_NEITHER.contains(&row.door) {
            // The exception is turned into an assertion rather than a hole: this door
            // reports the family it does not act on, so silence here would be the
            // divergence the previewed tier forbids.
            assert!(
                text.contains(PostureMember::OperationInProgress.code()),
                "`jigc {shown}` is the declared previewing member of the `Neither` \
                 class — it must RAISE `{}`, not be silent about it; output:\n{text}",
                PostureMember::OperationInProgress.code(),
            );
            continue;
        }
        for member in PostureMember::ALL {
            assert!(
                !text.contains(member.code()),
                "`jigc {shown}` acts on nobody's behalf, so it must not raise `{}` under a \
                 conflicting merge — a family that fires one class too wide is the same \
                 defect pointed the other way; output:\n{text}",
                member.code(),
            );
        }
    }

    assert!(
        driven >= 30,
        "the control means nothing over a collapsed class — it drove {driven} `Neither` rows",
    );
    git_state::assert_state(&repo, &home, state);
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — the destroying subject
//         (a MANUFACTURED SHAPE SPACE × a CODE-SIDE REGISTRY, read through
//          its own `Disposition` axis)
// ═════════════════════════════════════════════════════════════════════════════

use cli::milestone::{DESTROYING_DOORS, DestroyingDoor, Disposition};
use support::committing_doors::remove_hook;

/// One shape of the **complement** of `engine::state::TASK_AREA_FILES` — a thing that can
/// be in a working area which jigc's own writers never put there.
///
/// **The shape space is manufactured, and this says so.** Declaredness is a property of a
/// *fixture*, not of a registry: no code-side set can enumerate "a `docs/*.md` whose name is
/// not a staged identity", because the identities are the task's own. What the registry
/// fixes is the **membership rule** (`engine::state::staged_doc_id`, the `docs/` tree rule
/// and the two area rows); the shapes below are the cells that rule has to answer, one per
/// way a real agent's scratch lands in a gitignored area.
struct Shape {
    /// Where the plant goes, relative to the area.
    rel: &'static str,
    /// The **entry** a door names for it — the complement's unit is the entry, so a nested
    /// plant is named (and moved) by the directory at its top.
    entry: &'static str,
    /// A symlink's target, relative to the area; `None` for a regular file.
    symlink_to: Option<&'static str>,
}

/// The manufactured shape space, one cell per way the complement can be shaped.
const FOREIGN_SHAPES: &[Shape] = &[
    // A foreign regular file at the area root, and a `.md` one — the second matters because
    // the `docs/` rule keys on `.md` and a rule cut one level too wide would claim this.
    Shape {
        rel: "scratch.txt",
        entry: "scratch.txt",
        symlink_to: None,
    },
    Shape {
        rel: "NOTES.md",
        entry: "NOTES.md",
        symlink_to: None,
    },
    // A non-`.md` under `docs/` — inside the one registry member that is a tree.
    Shape {
        rel: "docs/notes.txt",
        entry: "docs/notes.txt",
        symlink_to: None,
    },
    // A `docs/*.md` whose name is **no staged identity**: `engine::state::staged_doc_id` is
    // the inverse of the writer's own `<type>:<slug>.md`, so a bare `.md` here is foreign.
    Shape {
        rel: "docs/not-an-identity.md",
        entry: "docs/not-an-identity.md",
        symlink_to: None,
    },
    // A nested directory — returned whole, never walked.
    Shape {
        rel: "analysis/perf.txt",
        entry: "analysis",
        symlink_to: None,
    },
    // A dotfile, which a walk written with a naive filter would skip.
    Shape {
        rel: ".agent-scratch",
        entry: ".agent-scratch",
        symlink_to: None,
    },
    // A symlink: a shape whose *bytes* are elsewhere, so a door that followed it would
    // answer about the wrong file.
    Shape {
        rel: "link-to-notes.md",
        entry: "link-to-notes.md",
        symlink_to: Some("NOTES.md"),
    },
];

/// The one shape whose answer at a **displacing** door is the gate's rather than the
/// displacement's — driven, recorded, and deliberately **not** dropped from the space.
///
/// A `docs/*.md` whose name is no staged identity is foreign to
/// `engine::state::staged_doc_id` (the destroying doors' subject — the three refusing doors
/// above name it as foreign, driven), and the gate's staged-doc conformance sweep reads the
/// same file as a **staged doc of an unknown type**. So at `jigc task finalize` and
/// `jigc milestone finalize` the run never reaches the teardown at all: it is adjudicated at
/// exit 3 with `schema-conformance.unknown-type` and a followable route (*remove or re-type
/// the stray staged file*).
///
/// It is recorded here rather than narrowed away because that is the honest driven answer:
/// **no byte dies and nothing is silent**, but two readers of one file answer differently,
/// and the displacing half of this arm therefore drives the cell **twice** — once to pin the
/// refusal that actually happens, once with the shape withheld so the rest of the space
/// reaches the park.
const GATE_ADJUDICATED_SHAPE: &str = "docs/not-an-identity.md";

/// Plant the whole shape space inside `area`, optionally withholding one shape.
fn plant_shape_space_except(area: &Path, withhold: Option<&str>) {
    fs::create_dir_all(area).expect("the area exists");
    for shape in FOREIGN_SHAPES {
        if withhold == Some(shape.rel) {
            continue;
        }
        let path = area.join(shape.rel);
        fs::create_dir_all(path.parent().expect("a plant has a parent")).expect("mk plant parent");
        match shape.symlink_to {
            None => fs::write(&path, format!("the agent's own {}\n", shape.rel))
                .expect("write the plant"),
            Some(target) => {
                let _ = fs::remove_file(&path);
                std::os::unix::fs::symlink(target, &path).expect("plant the symlink");
            }
        }
    }
}

/// Plant the whole shape space inside `area`.
fn plant_shape_space(area: &Path) {
    plant_shape_space_except(area, None);
}

/// Which subject a door's cell plants in — the door's **own** destroyed path, since
/// `DestroyingDoor`'s subject is *the path it removes*, not one shared area.
#[derive(Clone, Copy)]
enum Subject {
    /// The top-level task's working area (`.jigc/tasks/<id>/`).
    TopTaskArea,
    /// The milestone's sub-task working area (`.jigc/tasks/<sub-id>/`).
    SubTaskArea,
    /// A **leftover** at a sub-task's worktree path — `jigc milestone provision`'s subject,
    /// which is a worktree-shaped path and not a working area at all, so the area
    /// complement does not apply to it. The plants go inside the leftover directory, which
    /// is what makes it *hold work*.
    WorktreeLeftover,
}

/// Which reject arm a door's refusal rides — a **driven fact**, pinned per door so a door
/// that moves arms reddens here rather than moving silently.
///
/// The split is not an accident and is not this wave's to reconcile: the three
/// `<door>.foreign-bytes` codes M52 Increment 4 registered are registered **on the findings
/// arm** (roadmap → Milestone 52, Increment 4, *Codes it registers*), while
/// `milestone.leftover-holds-work` is M48's code, reaching the wire through the flattened
/// `{"error": …}` carrier exactly as it did before this wave — which M52 Increment 1 did not
/// re-register and this arm therefore does not claim.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RejectArm {
    /// `{findings, schema_version}` — the code is a key a driver branches on.
    Findings,
    /// `{error}` — the code is inside one prose string.
    FlattenedError,
}

/// What a door's fixture is and what argv reaches it.
struct DoorPlan {
    subject: Subject,
    /// The argv, without the consent flag.
    argv: &'static [&'static str],
    /// The code this door must answer with over the planted subject. Every one is asserted
    /// to be a member of the door's own `codes`, so a plan cannot invent an identity.
    code: &'static str,
    /// The arm the refusal rides.
    arm: RejectArm,
}

/// The milestone the arm's workbench carries, its sub-task and its top-level task.
const ARM3_MILESTONE: &str = "cache-rework";
const ARM3_SUB_TASK: &str = "warm-the-read-cache";
const ARM3_TOP_TASK: &str = "fix-the-retry-cap";

/// The plan for one member of [`DESTROYING_DOORS`], keyed on the door's own verb line.
///
/// A door with no plan **panics**: a seventh destroying door reddens here until someone
/// decides what it destroys and what it answers with, which is the property the registry
/// was minted for.
fn plan_for(door: &DestroyingDoor) -> DoorPlan {
    match door.verb {
        "jigc milestone provision" => DoorPlan {
            subject: Subject::WorktreeLeftover,
            argv: &["milestone", "provision", ARM3_MILESTONE],
            code: "milestone.leftover-holds-work",
            arm: RejectArm::FlattenedError,
        },
        "jigc milestone discard" => DoorPlan {
            subject: Subject::SubTaskArea,
            argv: &["milestone", "discard", ARM3_MILESTONE],
            code: "milestone.foreign-bytes",
            arm: RejectArm::Findings,
        },
        "jigc uninstall" => DoorPlan {
            subject: Subject::TopTaskArea,
            argv: &["uninstall"],
            code: "uninstall.foreign-bytes",
            arm: RejectArm::Findings,
        },
        "jigc task discard" => DoorPlan {
            subject: Subject::TopTaskArea,
            argv: &["task", "discard", ARM3_TOP_TASK],
            code: "task-discard.foreign-bytes",
            arm: RejectArm::Findings,
        },
        other => panic!(
            "`{other}` is a destroying door with no driven plan — a door added to \
             `DESTROYING_DOORS` reddens here until someone says what it destroys and what \
             it answers with, which is what the registry was minted for",
        ),
    }
}

/// The arm's workbench: a set-up repo carrying a milestone with one sub-task and one
/// top-level task — every subject the four **refusing** doors stand over.
fn destroying_workbench(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q", "-b", "main", "."]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-qm", "initial"]);
    for argv in [
        vec!["setup"],
        vec!["milestone", "create", "Cache rework"],
        vec![
            "milestone",
            "add-task",
            ARM3_MILESTONE,
            "Warm the read cache",
        ],
        // `record-decision` rather than `quick-fix`: the top-level task has to be able to
        // CREATE a doc, because the zero-false-fire control walks the writers that fill an
        // area (`roles.json` at the bind, `renames.json` at an in-task rename), and a
        // workflow whose `allows-create` is empty reaches neither.
        vec![
            "start",
            "--workflow",
            "record-decision",
            "Fix the retry cap",
        ],
    ] {
        let out = jigc(repo.path(), home.path(), &argv);
        assert!(
            out.status.success(),
            "the workbench step `jigc {}` must exit 0; output:\n{}",
            argv.join(" "),
            surface(&out),
        );
    }
    (repo, home)
}

/// Where a door's plants go in that workbench.
fn subject_path(repo: &Path, subject: Subject) -> PathBuf {
    let jigc_dir = repo.join(".jigc");
    match subject {
        Subject::TopTaskArea => jigc_dir.join("tasks").join(ARM3_TOP_TASK),
        Subject::SubTaskArea => jigc_dir.join("tasks").join(ARM3_SUB_TASK),
        Subject::WorktreeLeftover => jigc_dir.join("worktrees").join(ARM3_SUB_TASK),
    }
}

/// **Every destroying door, through its own disposition, over one planted shape space.**
///
/// The cells are **derived from the registry**, never listed: a `Refuse` member is driven
/// twice (without its consent and with it), a `Displace` member once, and the
/// `jigc task finalize --force` cell the Settle refuses is never enumerated because no
/// member carries a consent it does not have. A `Narrate` member — none today — reddens
/// until someone drives it.
#[test]
fn every_destroying_door_answers_for_the_bytes_it_did_not_write() {
    let mut refused = 0usize;
    let mut consented = 0usize;
    let mut displaced = 0usize;

    for door in DESTROYING_DOORS {
        match door.disposition {
            Disposition::Refuse { consent } => {
                let plan = plan_for(door);
                assert!(
                    door.codes.contains(&plan.code),
                    "`{}`'s plan names `{}`, which is not one of the door's own codes {:?} \
                     — a plan may not invent an identity",
                    door.verb,
                    plan.code,
                    door.codes,
                );

                // (a) WITHOUT the consent: refuses, names every planted entry, takes nothing.
                let (repo, home) = destroying_workbench("refuse");
                let subject = subject_path(repo.path(), plan.subject);
                plant_shape_space(&subject);
                let mut argv = plan.argv.to_vec();
                argv.extend_from_slice(&["--format", "json"]);
                let out = jigc(repo.path(), home.path(), &argv);
                let doc = one_reject_document(&out, door.verb);
                let rendered = serde_json::to_string(&doc).expect("re-render the reject");
                match plan.arm {
                    RejectArm::Findings => assert!(
                        finding_codes(&doc).iter().any(|c| c == plan.code),
                        "[{}] a door that would destroy bytes jigc did not write must refuse \
                         under `{}`, on the arm its code is registered on; got:\n{doc:#}",
                        door.verb,
                        plan.code,
                    ),
                    RejectArm::FlattenedError => {
                        assert_eq!(
                            top_level_keys(&doc),
                            BTreeSet::from(["error".to_string()]),
                            "[{}] this door's refusal rides the flattened arm, which is the \
                             shape it had before this wave; got:\n{doc:#}",
                            door.verb,
                        );
                        assert!(
                            rendered.contains(plan.code),
                            "[{}] …carrying `{}` as its identity; got:\n{doc:#}",
                            door.verb,
                            plan.code,
                        );
                    }
                }
                assert!(
                    rendered.contains(consent),
                    "[{}] …and the refusal's own route names the single consent `{consent}`; \
                     got:\n{doc:#}",
                    door.verb,
                );
                if matches!(plan.subject, Subject::WorktreeLeftover) {
                    assert!(
                        rendered.contains(ARM3_SUB_TASK),
                        "[{}] …naming the leftover path it would have removed; got:\n{doc:#}",
                        door.verb,
                    );
                } else {
                    for shape in FOREIGN_SHAPES {
                        assert!(
                            rendered.contains(shape.entry),
                            "[{}] …naming EVERY path it would have destroyed — `{}` is \
                             missing; got:\n{doc:#}",
                            door.verb,
                            shape.entry,
                        );
                    }
                }
                for shape in FOREIGN_SHAPES {
                    assert!(
                        subject.join(shape.rel).symlink_metadata().is_ok(),
                        "[{}] a refusal takes nothing — `{}` is gone",
                        door.verb,
                        shape.rel,
                    );
                }
                refused += 1;

                // (b) WITH the consent: the door performs its own act, and narrates the
                // loss as it takes it.
                let (repo, home) = destroying_workbench("consent");
                let subject = subject_path(repo.path(), plan.subject);
                plant_shape_space(&subject);
                let mut argv = plan.argv.to_vec();
                argv.push(consent);
                let out = jigc(repo.path(), home.path(), &argv);
                let text = surface(&out);
                assert!(
                    out.status.success(),
                    "[{}] `{consent}` is the single consent — the door performs its own act \
                     under it; output:\n{text}",
                    door.verb,
                );
                for shape in FOREIGN_SHAPES {
                    assert!(
                        subject.join(shape.rel).symlink_metadata().is_err(),
                        "[{}] …and the bytes it stood over are gone once consented to — \
                         `{}` is still there",
                        door.verb,
                        shape.rel,
                    );
                }
                // M46's measured rule: the consent never suppresses the narration, so the
                // loss is named as it is taken.
                assert!(
                    FOREIGN_SHAPES
                        .iter()
                        .any(|shape| text.contains(shape.entry))
                        || text.contains(ARM3_SUB_TASK)
                        || text.contains(ARM3_TOP_TASK),
                    "[{}] `{consent}` consents to the removal; it does not silence it — \
                     the door must still name what it took; output:\n{text}",
                    door.verb,
                );
                consented += 1;
            }
            Disposition::Displace => {
                displaced += 1;
            }
            Disposition::Narrate => panic!(
                "`{}` holds the `Narrate` disposition, which no member held when this arm \
                 was written — a door that takes bytes it cannot refuse over owes its own \
                 driven cell, which is what the three-armed rule is for",
                door.verb,
            ),
        }
    }

    assert!(
        refused >= 4 && consented == refused && displaced == 2,
        "the axis is the registry's own dispositions — got {refused} refusing cells, \
         {consented} consented cells and {displaced} displacing doors",
    );
}

/// How a **displacing** door is reached: the shared door-case fixture it is built from, the
/// working area whose complement it removes, and the argv that lands it.
struct DisplacePlan {
    /// The verb key `support::committing_doors::drive` builds the fixture under.
    fixture: &'static str,
    /// The work-unit id whose area the door tears down — also the `.jigc/displaced/<id>/`
    /// sub-directory the move preserves the relative path under.
    unit: &'static str,
    /// The argv that lands the door, without `--format json`.
    argv: &'static [&'static str],
    /// The identity **this** door adjudicates [`GATE_ADJUDICATED_SHAPE`] under — a driven
    /// fact per door, because the two boundaries read a stray `docs/*.md` through different
    /// probes: the task door's conformance sweep calls it a staged doc of an unknown type,
    /// and the milestone boundary's join calls it a staged doc with no recorded provenance.
    gate_code: &'static str,
}

/// The plan for one [`Disposition::Displace`] member, keyed on the door's own verb line — a
/// door with no plan panics, exactly as the refusing half does.
fn displace_plan_for(door: &DestroyingDoor) -> DisplacePlan {
    match door.verb {
        "jigc task finalize" => DisplacePlan {
            fixture: "jigc task finalize",
            unit: "survive-the-rejection",
            argv: &["task", "finalize", "survive-the-rejection"],
            gate_code: "schema-conformance.unknown-type",
        },
        "jigc milestone finalize" => DisplacePlan {
            fixture: "jigc milestone finalize (squash: true)",
            unit: "area-low",
            argv: &["milestone", "finalize", "cache-rework"],
            gate_code: "join.missing-provenance",
        },
        other => panic!(
            "`{other}` displaces and has no driven plan — a displacing door added to \
             `DESTROYING_DOORS` reddens here until someone says which area it tears down",
        ),
    }
}

/// Every `{from, to}` pair a landed envelope carries under `committed.displaced`.
fn displaced_pairs(stdout: &str) -> Vec<(String, String)> {
    let envelope = json(stdout);
    envelope["committed"]["displaced"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("a landed envelope carries `committed.displaced` as an array:\n{stdout}")
        })
        .iter()
        .map(|pair| {
            (
                pair["from"]
                    .as_str()
                    .expect("`from` is a string")
                    .to_owned(),
                pair["to"].as_str().expect("`to` is a string").to_owned(),
            )
        })
        .collect()
}

/// **The two doors with no consent to offer keep the bytes instead** — over the same
/// manufactured shape space the refusing half plants.
///
/// The landed-boundary warrant for *narrating* the loss (*the staged set is already in git*)
/// is structurally unavailable at both: the commit takes the promoted docs and the index and
/// nothing at all out of a working area. So the answer is the move — and the move is
/// asserted on **both** channels, because a loss named on only one of them is a loss a
/// driver or a reader cannot see.
#[test]
fn the_displacing_doors_keep_every_byte_they_cannot_commit() {
    let mut driven = 0usize;
    for door in DESTROYING_DOORS {
        if !matches!(door.disposition, Disposition::Displace) {
            continue;
        }
        assert!(
            door.codes.is_empty(),
            "[{}] a displacing member refuses over nothing it destroys — its answer is the \
             move, so its code set is empty. Since M53 Increment 2 / T3 both members DO mint \
             a code, `finalize.foreign-bytes`, and it still does not belong here: `codes` is \
             *door-scoped blocking codes the door refuses with*, and that one is a landed-arm \
             advisory over bytes the door KEPT, raised after the commit at exit 0; got {:?}",
            door.verb,
            door.codes,
        );
        driven += 1;
        let plan = displace_plan_for(door);
        let case: DoorCase = drive(plan.fixture);
        let repo = case.repo.path();
        let home = case.home.path();
        // The shared fixture installs the rejecting hook, because its own suites drive the
        // refused commit. This arm drives the **landed** one.
        remove_hook(repo);
        let area = repo.join(".jigc").join("tasks").join(plan.unit);
        let mut argv = plan.argv.to_vec();
        argv.extend_from_slice(&["--format", "json"]);

        // (a) the whole shape space — including the one cell the GATE answers instead of
        // the teardown. Driven and pinned rather than dropped: the run is adjudicated, it
        // names the stray file, and it carries a route.
        plant_shape_space(&area);
        let held = jigc(repo, home, &argv);
        let held_text = surface(&held);
        assert!(
            !held.status.success(),
            "[{}] with `{GATE_ADJUDICATED_SHAPE}` in the area the run is adjudicated by the \
             gate, not landed — if this now lands, the two readers of that file have been \
             reconciled and this cell owes a different assertion; output:\n{held_text}",
            door.verb,
        );
        assert!(
            held_text.contains(plan.gate_code)
                && held_text.contains("not-an-identity")
                && held_text.contains("route"),
            "[{}] …under this door's own identity `{}`, saying WHICH file and what to do \
             about it — no byte dies and nothing is silent, which is what makes this a \
             recorded contradiction rather than a loss; output:\n{held_text}",
            door.verb,
            plan.gate_code,
        );
        fs::remove_file(area.join(GATE_ADJUDICATED_SHAPE)).expect("withdraw the gate cell");

        // (b) the rest of the space reaches the park.
        let out = jigc(repo, home, &argv);
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            out.status.success(),
            "[{}] the boundary must land; stdout:\n{stdout}\nstderr:\n{stderr}",
            door.verb,
        );

        let pairs = displaced_pairs(stdout.trim());
        let park = repo.join(".jigc").join("displaced").join(plan.unit);
        for shape in FOREIGN_SHAPES {
            if shape.rel == GATE_ADJUDICATED_SHAPE {
                continue;
            }
            assert!(
                pairs.iter().any(|(from, _)| from.contains(shape.entry)),
                "[{}] every entry the teardown would have destroyed is named on the landed \
                 envelope — `{}` is missing from {pairs:?}",
                door.verb,
                shape.entry,
            );
            assert!(
                stderr.contains(shape.entry),
                "[{}] …and on stderr, the loss-shaped side channel, so the `--format json` \
                 document still owns stdout undiluted — `{}` is missing; stderr:\n{stderr}",
                door.verb,
                shape.entry,
            );
            assert!(
                park.join(shape.rel).symlink_metadata().is_ok(),
                "[{}] …and the bytes themselves are at `.jigc/displaced/{}/{}` with their \
                 relative path preserved",
                door.verb,
                plan.unit,
                shape.rel,
            );
        }
        assert!(
            !area.exists(),
            "[{}] …while the working area itself is gone: the teardown is never skipped, or \
             `jigc task list` reports an active task that no longer exists",
            door.verb,
        );
    }
    assert_eq!(
        driven, 2,
        "both displacing members of `DESTROYING_DOORS` owe a driven cell",
    );
}

/// **The zero-false-fire control: a full lifecycle never has jigc call its own files
/// foreign.**
///
/// A subject drawn one member too wide is the same defect pointed the other way, and it is
/// the one a shape space cannot catch: the plants are all genuinely foreign, so every cell
/// above would still pass over a registry missing `renames.json`. This walks a task through
/// the writers that fill an area — the mint, a created doc, an in-task rename (which is what
/// writes `renames.json`), and a validate (which writes both probe snapshots) — and then
/// asks all three consenting area doors whether anything there is foreign.
#[test]
fn a_full_lifecycle_never_has_jigc_call_its_own_files_foreign() {
    let (repo, home) = destroying_workbench("no-false-fire");
    let (repo, home) = (repo.path(), home.path());
    for argv in [
        vec![
            "doc",
            "create",
            "adr",
            "--title",
            "Retry policy",
            "--task",
            ARM3_TOP_TASK,
        ],
        vec![
            "doc",
            "rename",
            "adr:retry-policy",
            "--to",
            "Retry budget",
            "--task",
            ARM3_TOP_TASK,
        ],
        vec!["task", "validate", ARM3_TOP_TASK],
    ] {
        let out = jigc(repo, home, &argv);
        // `task validate` exits non-zero over an unfilled commit doc; what matters is that
        // it RAN and wrote its snapshots.
        assert!(
            !surface(&out).contains("foreign-bytes"),
            "`jigc {}` must not call one of jigc's own files foreign; output:\n{}",
            argv.join(" "),
            surface(&out),
        );
    }

    let area = repo.join(".jigc").join("tasks").join(ARM3_TOP_TASK);
    let on_disk: BTreeSet<String> = fs::read_dir(&area)
        .expect("read the task area")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        on_disk.contains("renames.json") && on_disk.contains("roles.json"),
        "the control must actually reach the writers whose files a hand-list drove short of \
         — the area holds: {on_disk:?}",
    );

    for door in DESTROYING_DOORS {
        if !matches!(door.disposition, Disposition::Refuse { .. }) {
            continue;
        }
        let plan = plan_for(door);
        if matches!(plan.subject, Subject::WorktreeLeftover) {
            // This door's subject is a worktree path, not a working area — the complement
            // rule it would false-fire on is not the one it asks.
            continue;
        }
        let text = surface(&jigc(repo, home, plan.argv));
        assert!(
            !text.contains("foreign-bytes"),
            "[{}] jigc's own writers fill a working area, and none of what they wrote may \
             read as foreign — the area holds {on_disk:?}; output:\n{text}",
            door.verb,
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — the corpus walk (the class's DEFINING CASE-SET, matched exhaustively,
//         crossed with the bump kind; plus a DERIVATION STATED AS ONE)
// ═════════════════════════════════════════════════════════════════════════════

use support::frozen_pack;

/// Run the real binary with an explicit `JIGC_PACK_DIR` — the manufactured pack a cell
/// bumps, which is the act a pack author performs.
fn jigc_with_pack(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// The shipped `adr`'s declared home line, and the shipped `changelog`'s.
const ADR_LOCATION: &str = "location: decisions/\n";
const CHANGELOG_PLACEMENT: &str = "placement: { file: CHANGELOG.md }\n";

/// Replace `needle` exactly once, asserting it was there.
fn swap_once(body: &str, needle: &str, replacement: &str) -> String {
    let out = body.replacen(needle, replacement, 1);
    assert_ne!(body, out, "the schema must carry `{}`", needle.trim_end());
    out
}

/// A conformant `adr` body, stamped at `version`.
fn arm4_adr_body(version: u32) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-25\nschema-version: {version}\n---\n\n\
         # Cache sessions in memory\n\n## Context\n\nSession lookups must stay \
         sub-millisecond.\n\n## Options\n\nA distributed cache was weighed and rejected on \
         latency.\n\n## Decision\n\nKeep sessions in a single in-memory node.\n\n\
         ## Consequences\n\nA cold node loses its sessions.\n"
    )
}

/// A conformant `changelog` body, stamped at `version`.
fn arm4_changelog_body(version: u32) -> String {
    format!(
        "---\nschema-version: {version}\n---\n\n# Changelog\n\n## Unreleased Changes\n\n\
         ### changed  {{#changed}}\n\n- the staging group\n\n## Releases\n\n\
         ### 1.0.0  {{#1-0-0}}\n\n<!-- fields -->\n- date: 2026-06-14\n\n\
         #### added  {{#added}}\n\n- the nested group\n"
    )
}

/// One `{from-home kind, to-home kind}` cell of the home-pair case-set.
struct HomePair {
    tag: &'static str,
    ty: &'static str,
    /// The current schema's home edit, applied to the shipped body.
    move_home: fn(&str) -> String,
    /// The committed instance's from-home, and where the migration must land it.
    source: &'static str,
    destination: &'static str,
    /// The doc body, stamped at the shipped version.
    body: fn(u32) -> String,
    /// A distinctive line of the committed prose the migration must preserve.
    prose: &'static str,
    /// The address the **read** surface must serve the landed doc under — the half a
    /// migration report cannot claim for itself.
    address: &'static str,
}

/// **The four home pairs** — the class's defining case-set, matched exhaustively. The pair
/// is the axis; the doctypes are the two shipped carriers of the two home kinds (`adr`
/// declares `location:`, `changelog` declares `placement:`), so each is driven in both
/// directions.
const HOME_PAIRS: &[HomePair] = &[
    HomePair {
        tag: "location-to-location",
        ty: "adr",
        move_home: |body| swap_once(body, ADR_LOCATION, "location: adrs/\n"),
        source: "docs/decisions/cache-sessions-in-memory.md",
        destination: "docs/adrs/cache-sessions-in-memory.md",
        body: arm4_adr_body,
        prose: "Session lookups must stay sub-millisecond.",
        address: "adr:cache-sessions-in-memory",
    },
    HomePair {
        tag: "location-to-placement",
        ty: "adr",
        move_home: |body| swap_once(body, ADR_LOCATION, "placement: { file: DECISIONS.md }\n"),
        source: "docs/decisions/cache-sessions-in-memory.md",
        destination: "DECISIONS.md",
        body: arm4_adr_body,
        prose: "Session lookups must stay sub-millisecond.",
        // A `placement:` doctype's identity is its type id — the one address it has.
        address: "adr",
    },
    HomePair {
        tag: "placement-to-placement",
        ty: "changelog",
        move_home: |body| {
            swap_once(
                body,
                CHANGELOG_PLACEMENT,
                "placement: { file: HISTORY.md }\n",
            )
        },
        source: "CHANGELOG.md",
        destination: "HISTORY.md",
        body: arm4_changelog_body,
        prose: "- the staging group",
        address: "changelog",
    },
    HomePair {
        tag: "placement-to-location",
        ty: "changelog",
        move_home: |body| swap_once(body, CHANGELOG_PLACEMENT, "location: changelog/\n"),
        source: "CHANGELOG.md",
        // A fixed-identity doctype's slug is its type id, so the identity survives the move
        // into a folder home as the file **stem**.
        destination: "docs/changelog/changelog.md",
        body: arm4_changelog_body,
        prose: "- the staging group",
        address: "changelog",
    },
];

/// The bump kinds the pair axis is crossed with. The bump kind is **not** the
/// discriminator (`baseline-freeze.md` §2.1 drove W2/W5 behaving identically to W1/W4) —
/// which is exactly why both are driven: a walk that keyed on the kind would show here.
const BUMP_RIDERS: &[(&str, bool)] = &[("relocated-only", false), ("with-content", true)];

/// Append an **optional** prose section — the content half of a relocation bump.
fn with_optional_section(body: &str) -> String {
    format!("{body}  - id: rollout\n    slot: {{ optional: true, hint: \"How it rolls out.\" }}\n")
}

/// **Every home pair, at both bump kinds, landed and then READ BACK.**
///
/// The composite this adds over `migrate_corpus_home_pairs.rs`: that suite owns each cell's
/// pre-state and the missing-snapshot arm. This one asserts the **adopter's done picture** —
/// the doc is at the current home with its prose byte-preserved and its stamp current, the
/// store validates at exit 0, and the pinned read surface **addresses the doc at its new
/// home**. A migration that reported success while leaving the read surface pointing at the
/// old home would pass every assertion a migration report can make about itself.
#[test]
fn every_home_pair_lands_and_the_read_surface_addresses_the_doc_at_its_new_home() {
    for pair in HOME_PAIRS {
        for (rider, content) in BUMP_RIDERS {
            let tag = format!("{}-{rider}", pair.tag);
            let pack_dir = TempDir::new(&format!("pack-{tag}"));
            let move_home = pair.move_home;
            let from = frozen_pack::bumped_pack(
                pack_dir.path(),
                pair.ty,
                |shipped| shipped.to_string(),
                |shipped| {
                    let moved = move_home(shipped);
                    if *content {
                        with_optional_section(&moved)
                    } else {
                        moved
                    }
                },
            );

            let repo = TempDir::new(&format!("repo-{tag}"));
            let home = TempDir::new(&format!("home-{tag}"));
            let (repo, home, pack) = (repo.path(), home.path(), pack_dir.path());
            git(repo, &["init", "-q", "-b", "main", "."]);
            git(repo, &["config", "user.email", "test@example.com"]);
            git(repo, &["config", "user.name", "Test"]);
            fs::write(repo.join("README.md"), "hello\n").expect("write README");
            git(repo, &["add", "-A"]);
            git(repo, &["commit", "-qm", "initial"]);
            let set_up = jigc_with_pack(repo, home, pack, &["setup"]);
            assert!(
                set_up.status.success(),
                "[{tag}] `jigc setup` must exit 0; output:\n{}",
                surface(&set_up),
            );

            let body = (pair.body)(from);
            let source = repo.join(pair.source);
            fs::create_dir_all(source.parent().expect("the source has a parent"))
                .expect("mk the from-home");
            fs::write(&source, &body).expect("write the committed instance");
            git(repo, &["add", "-A"]);
            git(repo, &["commit", "-qm", "seed the corpus"]);

            // The walk sees it at the prior home, whatever KIND that home was, and lands it.
            let migrated = jigc_with_pack(repo, home, pack, &["migrate-corpus"]);
            let report = surface(&migrated);
            assert!(
                migrated.status.success(),
                "[{tag}] `jigc migrate-corpus` must land the below-version doc; \
                 output:\n{report}",
            );
            let landed = repo.join(pair.destination);
            let landed_body = fs::read_to_string(&landed).unwrap_or_else(|err| {
                panic!(
                    "[{tag}] the doc must be at the CURRENT home `{}`: {err}\n{report}",
                    pair.destination,
                )
            });
            assert!(
                !source.exists() || pair.source == pair.destination,
                "[{tag}] …and gone from the prior home `{}`",
                pair.source,
            );
            assert!(
                landed_body.contains(pair.prose),
                "[{tag}] …with its authored prose byte-preserved; it now reads:\n{landed_body}",
            );
            assert!(
                landed_body.contains(&format!("schema-version: {}", from + 1)),
                "[{tag}] …and its stamp at the current version; it now reads:\n{landed_body}",
            );

            // The store is clean, and the pinned READ surface addresses the doc where it now
            // lives — the half a migration report cannot claim for itself.
            let validated = jigc_with_pack(repo, home, pack, &["validate"]);
            assert!(
                validated.status.success(),
                "[{tag}] `jigc validate` must exit 0 over the migrated corpus; output:\n{}",
                surface(&validated),
            );
            let listed = jigc_with_pack(repo, home, pack, &["doc", "list", "--format", "json"]);
            assert!(
                listed.status.success(),
                "[{tag}] `jigc doc list --format json` must exit 0; output:\n{}",
                surface(&listed),
            );
            let index = String::from_utf8_lossy(&listed.stdout).into_owned();
            assert!(
                index.contains(pair.destination),
                "[{tag}] the store index must name the doc at its NEW home `{}`; got:\n{index}",
                pair.destination,
            );
            let shown = jigc_with_pack(
                repo,
                home,
                pack,
                &["doc", "show", pair.address, "--format", "json"],
            );
            assert!(
                shown.status.success(),
                "[{tag}] `jigc doc show {}` must serve the landed doc — a migration that \
                 leaves the read surface pointing at the old home has moved a file and lost \
                 a document; output:\n{}",
                pair.address,
                surface(&shown),
            );
        }
    }
}

/// **The fixed-identity home set, derived from the pinned contract** — every doctype of both
/// embedded packs whose `jigc doc schema … --format json` projection says its identity is
/// `fixed` and whose home has a path, as `(doctype, repo-relative home)`.
///
/// **It is a derivation, and this says which one.** The producer's own derivation
/// (`cli::orphan::fixed_identity_homes`) is `pub(crate)` and pinned beside it; what is read
/// here is the **contract surface** the same primitive feeds — so the set the store's
/// vacated-home condition answers over and the set a driver can discover through
/// `contract-version` 7 are asserted to be the same set, which is the fact neither side can
/// state alone.
fn fixed_identity_homes_via_contract(corpus: &TrialCorpus) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for ty in all_doctypes() {
        let shown = corpus.jigc(&["doc", "schema", &ty, "--format", "json"]);
        assert!(
            shown.status.success(),
            "`jigc doc schema {ty} --format json` must exit 0; output:\n{}",
            surface(&shown),
        );
        let doc = json(String::from_utf8_lossy(&shown.stdout).trim());
        assert_eq!(
            doc["contract-version"].as_u64(),
            Some(7),
            "the schema projection is pinned at `contract-version` 7, which is where \
             `identity` and `home` joined it; got:\n{doc:#}",
        );
        let identity = doc
            .get("identity")
            .unwrap_or_else(|| panic!("`{ty}`'s projection carries `identity`:\n{doc:#}"));
        let home = doc
            .get("home")
            .unwrap_or_else(|| panic!("`{ty}`'s projection carries `home`:\n{doc:#}"));
        if identity["kind"].as_str() == Some("fixed")
            && let Some(path) = home["path"].as_str()
        {
            out.insert(ty, path.to_owned());
        }
    }
    assert!(
        out.len() >= 4,
        "the derived fixed-identity home set is {} members — a sweep over a collapsed \
         derivation would pass vacuously",
        out.len(),
    );
    out
}

/// Every doctype id both embedded packs ship, through the production loader.
fn all_doctypes() -> Vec<String> {
    use engine::packsource::{PackResourceKind, PackSource};
    let pack = cli::pack::CompositePack::new(vec![
        Box::new(cli::pack::EmbeddedPack::new()),
        Box::new(cli::pack::EmbeddedPack::methodology()),
    ]);
    let mut ids: Vec<String> = pack
        .list(PackResourceKind::Schemas)
        .iter()
        .map(|id| id.as_str().to_owned())
        .collect();
    ids.sort();
    ids.dedup();
    ids
}

/// **A declared home the repository committed into and then emptied turns the store red — at
/// every member of the derived set the fixture fills.**
///
/// The fresh-clone shape is the whole point: `.jigc/state/` is gitignored, so what every
/// clone and every CI runner has is the corpus *without* the cache that used to be the only
/// thing between an adopter and a green build over a lost managed document.
#[test]
fn every_filled_fixed_identity_home_that_is_vacated_turns_the_store_red() {
    let base = TrialCorpus::build(State::CommittedSingletons);
    let homes = fixed_identity_homes_via_contract(&base);
    let code = cli::orphan::HOME_VACATED_CODE;
    assert!(
        cli::render::STORE_EXIT_FLIPS.iter().any(|flip| {
            let witness = (flip.witness)();
            witness.code == code
        }),
        "the condition must be a member of `STORE_EXIT_FLIPS` — membership IS what makes \
         the store sweep's exit flip, and a condition that reports without flipping is the \
         false green this wave closed",
    );

    let filled: Vec<(&String, &String)> = homes
        .iter()
        .filter(|(_, path)| base.repo().join(path).is_file())
        .collect();
    assert!(
        filled.len() >= 3,
        "the fixture must fill at least three of the derived homes, or the sweep proves \
         nothing; it fills {filled:?}",
    );

    // (a) with every home filled, the sweep is silent about all of them.
    let clean = base.copy_state();
    clean.fresh_clone_shape();
    let clean_text = surface(&clean.jigc(&["validate"]));
    assert!(
        !clean_text.contains(code),
        "a filled home is not a vacated one — the control must stay silent; output:\n{clean_text}",
    );

    // (b) …and each one, emptied by one ordinary human act, is named and turns the exit.
    for (ty, path) in filled {
        let corpus = base.copy_state();
        corpus.git(&["mv", path, &format!("MOVED-{ty}.md")]);
        corpus.git(&[
            "commit",
            "-q",
            "-m",
            "move a managed doc out of its declared home",
        ]);
        corpus.fresh_clone_shape();

        let out = corpus.jigc(&["validate"]);
        let text = surface(&out);
        assert!(
            !out.status.success(),
            "[{ty}] `jigc validate` must exit non-zero once `{path}` is vacated — a green CI \
             over a lost managed document is the false green this condition closes; \
             output:\n{text}",
        );
        assert!(
            text.lines()
                .any(|line| line.contains(code) && line.contains(path)),
            "[{ty}] …naming the exact declared home `{path}` it found empty; output:\n{text}",
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — the fixed identity (a DERIVATION STATED AS ONE × a CODE-SIDE REGISTRY)
// ═════════════════════════════════════════════════════════════════════════════

use cli::cli::{DOCTYPE_DOORS, DoctypeArg, SLUG_DOORS, SLUG_OVERRIDE_SLOT};

/// The blocking identity every fixed-identity refusal carries.
const FIXED_IDENTITY: &str = "store.fixed-identity";

/// The non-canonical slug every cell hands a fixed-identity doctype — **well-formed**, so
/// the only thing the door can fault on is the identity rule itself.
const BOGUS: &str = "bogus";

/// The **fixed-identity doctype set**, derived from the pinned contract: every doctype of
/// both embedded packs whose projection says `identity.kind == "fixed"`.
///
/// Stated as a derivation rather than listed, because the set is a property of the composed
/// packs: a doctype that gains `placement:` or `singleton: true` joins the doors' subject
/// with no edit here, which is exactly the coupling `fixed_identity_axis.rs`'s manufactured
/// cell exists beside.
fn fixed_identity_doctypes(corpus: &TrialCorpus) -> Vec<String> {
    let mut out = Vec::new();
    for ty in all_doctypes() {
        let shown = corpus.jigc(&["doc", "schema", &ty, "--format", "json"]);
        assert!(
            shown.status.success(),
            "`jigc doc schema {ty} --format json` must exit 0; output:\n{}",
            surface(&shown),
        );
        let doc = json(String::from_utf8_lossy(&shown.stdout).trim());
        if doc["identity"]["kind"].as_str() == Some("fixed") {
            out.push(ty);
        }
    }
    assert!(
        out.len() >= 5,
        "the derived fixed-identity doctype set is {out:?} — a sweep over a collapsed \
         derivation would pass vacuously",
    );
    out
}

/// A door whose address cell is reachable only through a state this arm's corpus does not
/// carry — a **stated** bound, never a silent skip.
///
/// `jigc task bind` resolves the *role* before the address, and no shipped workflow declares
/// a `reads:` role over a fixed-identity doctype — so its fixed-identity cell is reachable
/// only through a manufactured role, which `fixed_identity_axis.rs` owns (roadmap →
/// Milestone 52, Increment 6, *Declared bounds*). What this arm asserts at that door is the
/// half that is reachable: it refuses, under an identity of its own, and writes nothing.
const ROLE_GATED_DOORS: &[&[&str]] = &[&["task", "bind"]];

/// The argv (and optional stdin) that reaches one `DoctypeArg::Address` door with `head` as
/// the address's type-and-slug head.
fn address_door_argv(
    door: &[&str],
    head: &str,
    task: &str,
    milestone: &str,
) -> (Vec<String>, Option<&'static str>) {
    let own = |parts: Vec<String>| (parts, None);
    let s = |v: &str| v.to_string();
    match door {
        ["rename"] => own(vec![s("rename"), s(head), s("--to"), s("Axis Title")]),
        ["doc", "show"] => own(vec![s("doc"), s("show"), s(head)]),
        ["doc", "add-item"] => own(vec![
            s("doc"),
            s("add-item"),
            format!("{head}#entries"),
            s("--title"),
            s("Axis Item"),
            s("--task"),
            s(task),
        ]),
        ["doc", "remove-item"] => own(vec![
            s("doc"),
            s("remove-item"),
            format!("{head}#entries.an-item"),
            s("--task"),
            s(task),
        ]),
        ["doc", "retitle-item"] => own(vec![
            s("doc"),
            s("retitle-item"),
            format!("{head}#entries.an-item"),
            s("--title"),
            s("Axis Item"),
            s("--task"),
            s(task),
        ]),
        ["doc", "rename"] => own(vec![
            s("doc"),
            s("rename"),
            s(head),
            s("--to"),
            s("Axis Title"),
            s("--task"),
            s(task),
        ]),
        ["doc", "set-field"] => own(vec![
            s("doc"),
            s("set-field"),
            format!("{head}#meta/status"),
            s("--value"),
            s("accepted"),
            s("--task"),
            s(task),
        ]),
        ["doc", "set-slot"] => (
            vec![
                s("doc"),
                s("set-slot"),
                format!("{head}#thesis"),
                s("--from-file"),
                s("-"),
                s("--task"),
                s(task),
            ],
            Some("Axis prose.\n"),
        ),
        ["task", "bind"] => own(vec![s("task"), s("bind"), s("spec"), s(head), s(task)]),
        ["milestone", "add-from-spec"] => own(vec![
            s("milestone"),
            s("add-from-spec"),
            s(milestone),
            s(head),
        ]),
        other => panic!(
            "`jigc {}` takes a doctype as an ADDRESS head and has no driven argv — a door \
             added to `DOCTYPE_DOORS` reddens here until someone says how it is reached",
            other.join(" "),
        ),
    }
}

/// **Every fixed-identity doctype, at every door that takes a doctype as an address head.**
///
/// The composite this adds over `fixed_identity_axis.rs`: that suite owns the manufactured
/// `location:`+`singleton: true` doctype (the one shape that separates the predicate's two
/// disjuncts) and each door's own refusal text. This one drives the **whole derived doctype
/// set at the whole door registry in one corpus**, so a doctype the packs make fixed and a
/// door that takes an address are crossed rather than sampled — and asserts the projection
/// half beside it, so the set the doors refuse over is the set the pinned contract
/// advertises.
#[test]
fn every_fixed_identity_doctype_refuses_a_non_canonical_head_at_every_address_door() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let task = corpus.start_workflow("single-task", "sweep the address doors");
    corpus.jigc_ok(&["milestone", "create", "Address sweep"]);
    let milestone = "address-sweep";

    let doctypes = fixed_identity_doctypes(&corpus);
    let doors: Vec<&'static [&'static str]> = DOCTYPE_DOORS
        .iter()
        .filter(|(_, arg)| matches!(arg, DoctypeArg::Address))
        .map(|(door, _)| *door)
        .collect();
    assert!(
        doors.len() >= 10,
        "the address-door set is {} rows — a sweep over a collapsed registry would pass \
         vacuously",
        doors.len(),
    );

    let mut cells = 0usize;
    for ty in &doctypes {
        let head = format!("{ty}:{BOGUS}");
        for door in &doors {
            cells += 1;
            let shown = door.join(" ");
            let (argv, stdin) = address_door_argv(door, &head, &task, milestone);
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = match stdin {
                None => corpus.jigc(&args),
                Some(payload) => corpus.jigc_stdin(&args, payload),
            };
            let text = surface(&out);
            assert!(
                !out.status.success(),
                "`jigc {shown} {head}` must refuse — `{ty}` has exactly one identity and \
                 `{BOGUS}` is not it; output:\n{text}",
            );
            if ROLE_GATED_DOORS.contains(door) {
                // The stated bound: this door resolves its role first, so the cell it
                // answers here is its own. What is asserted is that it refuses and writes
                // nothing — never that it answers under the identity rule.
                assert!(
                    text.contains("role"),
                    "`jigc {shown} {head}` resolves its ROLE before the address, so what it \
                     answers here is the role's own refusal — the bound's own datum, \
                     asserted rather than assumed; output:\n{text}",
                );
                assert!(
                    !text.contains(FIXED_IDENTITY),
                    "…and it does not claim the identity rule it never reached; \
                     output:\n{text}",
                );
                continue;
            }
            assert!(
                text.contains(FIXED_IDENTITY),
                "`jigc {shown} {head}` must refuse with `{FIXED_IDENTITY}` — the rule is one \
                 predicate at every door, so a door answering something else is the \
                 divergence this class closed; output:\n{text}",
            );
            assert!(
                text.contains(&format!("`{ty}`")) || text.contains(&format!("`{ty}:{ty}`")),
                "…routing at the canonical address, which is the ONE address `{ty}` has; \
                 output:\n{text}",
            );
        }
    }
    assert_eq!(
        cells,
        doctypes.len() * doors.len(),
        "every derived doctype owes a cell at every address door",
    );
}

/// The identity a **re-slug** of a fixed-identity doctype is refused under — the predicate is
/// the same one every address door asks; the code is the write path's own.
const IDENTITY_CHANGE: &str = "write.identity-change";

/// **`--slug` refuses on the same predicate, and its route stops composing the caller's
/// token** — the `SLUG_DOORS` row the class also reaches.
#[test]
fn the_rename_slug_override_refuses_a_fixed_identity_doctype() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let doctypes = fixed_identity_doctypes(&corpus);
    let homes = fixed_identity_homes_via_contract(&corpus);
    let row = SLUG_DOORS
        .iter()
        .find(|row| row.door == ["rename"])
        .expect("`SLUG_DOORS` carries the `rename` row");

    for ty in &doctypes {
        // The row's own argv, with the type head and the override substituted — the
        // registry's shape, not a hand-written command line.
        let argv: Vec<String> = row
            .argv
            .iter()
            .map(|token| match *token {
                SLUG_OVERRIDE_SLOT => BOGUS.to_string(),
                // The doctype's OWN canonical address — `jigc rename` takes a
                // `<type>:<slug>` address, and a fixed-identity doctype's one address is
                // `<ty>:<ty>`. The cell is therefore a re-slug of a doc that exists, which
                // is the act the predicate refuses.
                "adr:keeper" => format!("{ty}:{ty}"),
                other => other.to_string(),
            })
            .collect();
        let args: Vec<&str> = argv.iter().map(String::as_str).collect();
        let out = corpus.jigc(&args);
        let text = surface(&out);
        assert!(
            !out.status.success(),
            "`jigc {}` must refuse — a fixed identity is not the caller's to choose; \
             output:\n{text}",
            argv.join(" "),
        );
        // A re-slug needs a doc to re-slug, so what the door answers depends on whether the
        // fixture committed this doctype's one instance. **Both branches are asserted** —
        // the door's prior question where there is no doc, the identity rule where there is
        // — because asserting only the reachable one would leave the ordering unproven.
        let committed = homes
            .get(ty)
            .map(|home| corpus.repo().join(home).is_file())
            .unwrap_or(false);
        if committed {
            assert!(
                text.contains(IDENTITY_CHANGE),
                "…on the same predicate every address door asks, under the re-slug's own \
                 identity `{IDENTITY_CHANGE}`; output:\n{text}",
            );
        } else {
            assert!(
                text.contains("store.not-found"),
                "…and where the fixture committed no instance the door answers its PRIOR \
                 question first, which is the ordering the identity guard sits behind; \
                 output:\n{text}",
            );
        }
        assert!(
            !text.contains(&format!("--slug {BOGUS}")) && !text.contains(&format!("`{BOGUS}`")),
            "…and its route must stop composing the caller's own token, which would hand \
             back the spelling the door just refused; output:\n{text}",
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 6 — the pre-dispatch funnel × the verb surface
//         (a CODE-SIDE REGISTRY × a CODE-SIDE REGISTRY, under the phase
//          relation; plus `ENVELOPE_ARMS`, where MEMBERSHIP IS THE ASSERTION)
// ═════════════════════════════════════════════════════════════════════════════

use crate::pre_dispatch_faults::{Fixture, PRE_DISPATCH_FAULTS, Phase};
use cli::cli::VERB_KINDS;
use cli::render::{ArmShape, ENVELOPE_ARMS};
use support::leaf_argv;

/// The key set `ENVELOPE_ARMS` declares for one cross-cutting reject arm — **read off the
/// registry**, so a key added to or removed from a reject envelope reddens here rather than
/// being re-asserted from memory.
fn reject_arm_keys(arm: &str) -> BTreeSet<String> {
    let row = ENVELOPE_ARMS
        .iter()
        .find(|row| row.arm == arm && row.path.is_empty())
        .unwrap_or_else(|| panic!("`{arm}` is a cross-cutting row of `ENVELOPE_ARMS`"));
    match row.shape {
        ArmShape::Object(keys) => keys.iter().map(|key| (*key).to_string()).collect(),
        _ => panic!("`{arm}` declares an object key set"),
    }
}

/// Build the state a fault row names, and return `(cwd, home, pack dir)` for the drive.
///
/// The `DeletedCwd` row is driven through a shell rather than `Command::current_dir`,
/// because the child `chdir`s before `exec`: a removed directory fails the **spawn**, and
/// jigc never runs. A shell reaches the state the way a human does.
fn drive_faulting(fixture: Fixture, home: &Path, corpus: &TrialCorpus, argv: &[&str]) -> Output {
    match fixture {
        Fixture::DeletedCwd => {
            let gone = support::trial_corpus::unique_root("flow53-gone");
            fs::create_dir_all(&gone).expect("create the doomed cwd");
            let out = Command::new("sh")
                .arg("-c")
                .arg(r#"cd "$1" || exit 90; rmdir "$1" || exit 91; shift; exec "$@""#)
                .arg("sh")
                .arg(&gone)
                .arg(env!("CARGO_BIN_EXE_jigc"))
                .arg("--format")
                .arg("json")
                .args(argv)
                .env("HOME", home)
                .env_remove("JIGC_PACK_DIR")
                .output()
                .expect("spawn jigc through sh");
            assert!(
                !matches!(out.status.code(), Some(90) | Some(91)),
                "the deleted-cwd fixture failed to build the state it tests (exit {:?})",
                out.status.code(),
            );
            out
        }
        Fixture::MalformedPacksYaml => {
            let mut full = vec!["--format", "json"];
            full.extend_from_slice(argv);
            corpus.jigc(&full)
        }
        Fixture::EmptyPackDir => {
            let empty = TempDir::new("empty-pack");
            let mut full = vec!["--format", "json"];
            full.extend_from_slice(argv);
            let args: Vec<&str> = full;
            Command::new(env!("CARGO_BIN_EXE_jigc"))
                .args(&args)
                .current_dir(corpus.repo())
                .env("HOME", corpus.home())
                .env("JIGC_PACK_DIR", empty.path())
                .output()
                .expect("spawn the jigc binary")
        }
    }
}

/// **Every pre-dispatch fault, at every leaf of the verb surface, answered with exactly one
/// document on an arm the registry declares.**
///
/// The composite this adds over `pre_dispatch_faults.rs`: that suite owns the per-cell
/// expectation table — which arm, which exit, which leaves are out of phase. This one
/// crosses the fault registry with **`ENVELOPE_ARMS`**, so what a rejecting cell is held to
/// is not a remembered key list but the registry's own declaration: a driver parsing
/// `--format json` under any pre-dispatch state gets one document, and its top-level key set
/// **is** one the contract declares. The universal fault — the one every leaf reaches,
/// because it fires before jigc has located anything — is asserted over **all** of
/// `VERB_KINDS`.
#[test]
fn every_pre_dispatch_fault_answers_every_leaf_on_a_declared_arm() {
    let declared: Vec<BTreeSet<String>> = ["Reject::Error", "Reject::Findings"]
        .iter()
        .map(|arm| reject_arm_keys(arm))
        .collect();
    let error_keys = reject_arm_keys("Reject::Error");

    // The one corpus the two later-phase faults are driven over: a set-up repo whose
    // `packs.yaml` is not valid YAML.
    let corpus = TrialCorpus::build(State::Fresh);
    fs::write(
        corpus
            .repo()
            .join(".jigc")
            .join("config")
            .join("packs.yaml"),
        "packs: [ this is not: valid: yaml\n",
    )
    .expect("corrupt the pack-set list");
    for (name, body) in leaf_argv::FIXTURE_FILES {
        fs::write(corpus.repo().join(name), body).expect("write the tail's fixture file");
    }
    let home = TempDir::new("pre-dispatch-home");

    let mut cells = 0usize;
    for fault in PRE_DISPATCH_FAULTS {
        for (leaf, _) in VERB_KINDS {
            let tail = leaf_argv::MINIMAL_ARGV
                .iter()
                .find(|(path, _)| path == leaf)
                .map(|(_, tail)| *tail)
                .unwrap_or_else(|| {
                    panic!(
                        "`jigc {}` has no row in `support::leaf_argv::MINIMAL_ARGV` — the \
                         shared table is fenced against `VERB_KINDS`",
                        leaf.join(" "),
                    )
                });
            let argv: Vec<&str> = leaf.iter().copied().chain(tail.iter().copied()).collect();
            let shown = argv.join(" ");
            cells += 1;

            let out = drive_faulting(fault.fixture, home.path(), &corpus, &argv);
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

            // One document, wherever it rides: a driver's `json.loads` is the contract.
            for (stream, bytes) in [("stdout", &stdout), ("stderr", &stderr)] {
                if bytes.trim().is_empty() {
                    continue;
                }
                serde_json::from_str::<Value>(bytes.trim()).unwrap_or_else(|err| {
                    panic!(
                        "[{}] `jigc {shown}` put more than one JSON value on {stream} \
                         ({err}):\n{bytes}",
                        fault.id,
                    )
                });
            }

            let rejected = !out.status.success() && stdout.trim().is_empty();
            if rejected {
                let doc = json(stderr.trim());
                let keys = top_level_keys(&doc);
                assert!(
                    declared.contains(&keys),
                    "[{}] `jigc {shown}` rejects on a key set `ENVELOPE_ARMS` does not \
                     declare — membership IS the assertion here, so a reject shape that is \
                     not in the registry is a shape no driver was promised; got {keys:?}",
                    fault.id,
                );
            }

            if fault.phase == Phase::BeforeDiscovery {
                // The universal cell: this fault fires before jigc has located anything, so
                // **every** leaf reaches it — and answers on the operational reject arm.
                assert!(
                    rejected,
                    "[{}] `jigc {shown}` must answer the universal fault — it fires before \
                     discovery, so no leaf is out of its phase; stdout:\n{stdout}\n\
                     stderr:\n{stderr}",
                    fault.id,
                );
                assert_eq!(
                    top_level_keys(&json(stderr.trim())),
                    error_keys,
                    "[{}] …on the declared operational reject arm; got:\n{stderr}",
                    fault.id,
                );
            }
        }
    }
    assert_eq!(
        cells,
        PRE_DISPATCH_FAULTS.len() * VERB_KINDS.len(),
        "every fault owes a cell at every leaf of the verb surface",
    );
}

/// **D13 lead 1's owed seam suite — the `Route` span fence, in the posture it is enforced
/// in** (roadmap → Milestone 52, Increment 1, *Flow-53 arm*; settle-record → D13).
///
/// The fence is **debug-posture** by design (the seam-assert class): it rides the suite and
/// never a release-build panic, and `main` installs its parse sibling — so the binary these
/// arms drive carries both live. A suite that only drove the binary could not tell a fence
/// that fired from one that was never asked, which is why the constructors are exercised
/// **directly** here and the driven half is beside them.
///
/// The trigger fired at M51's completion audit: an ordinary spaced filename produced
/// `` `git add -- my notes.md` `` (exit 128, pathspec `my`) and `` `jigc migrate my notes.md
/// --as adr` `` (exit 2) — a route that dead-ends when followed verbatim. The fence now sits
/// on **all three** constructors, because the bytes a reader pastes do not care which kind
/// minted them.
#[cfg(debug_assertions)]
#[test]
fn the_route_span_fence_is_live_on_all_three_constructors() {
    use engine::finding::{
        Finding, Location, Route, Severity, command_spans_are_shell_safe, shell_token,
    };

    /// A route text whose backticked span carries a token a shell would not re-lex as
    /// itself — the **span** fence's own subject, which checks tokens.
    const UNSAFE_TEXT: &str = "stage it with `git add -- don't-ship.md`, then re-run";

    /// The path whose **word boundary** the span fence cannot see: `git add -- my notes.md`
    /// passes every token check and exits 128 when run (pathspec `my`). That is the
    /// **subject** fence's cell, and it is driven below with the finding that carries the
    /// boundary — `Location::address` is the very token the route is talking about.
    const SPACED_PATH: &str = "my notes.md";

    assert!(
        !command_spans_are_shell_safe(UNSAFE_TEXT),
        "the fence's question form must answer about the same bytes its assertion form \
         panics on — a producer that DERIVES a route asks here, in every posture",
    );
    let quoted = format!(
        "stage it with `git add -- {}`, then re-run",
        shell_token(SPACED_PATH),
    );
    assert!(
        command_spans_are_shell_safe(&quoted),
        "…and the rendered-through-`shell_token` form is what it accepts; got: {quoted}",
    );

    let hushed = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    /// One constructor, boxed so the three are driven by one loop rather than three
    /// copies of the same `catch_unwind`.
    type Construction = (&'static str, Box<dyn Fn() + std::panic::UnwindSafe>);
    let constructors: Vec<Construction> = vec![
        (
            "Route::human",
            Box::new(|| {
                let _ = Route::human(UNSAFE_TEXT);
            }),
        ),
        (
            "Route::informational",
            Box::new(|| {
                let _ = Route::informational(UNSAFE_TEXT);
            }),
        ),
        (
            // A mechanical route's prose **tail** names a second command line the argv
            // check never sees — which is why the fence is on this constructor too.
            "Route::mechanical",
            Box::new(|| {
                let _ = Route::mechanical(
                    ["jigc", "task", "list"].iter().map(|s| (*s).to_string()),
                    UNSAFE_TEXT,
                );
            }),
        ),
    ];
    let mut fired = Vec::new();
    for (name, construct) in constructors {
        if std::panic::catch_unwind(construct).is_err() {
            fired.push(name);
        }
    }
    assert_eq!(
        fired,
        vec!["Route::human", "Route::informational", "Route::mechanical"],
        "the span fence is installed on ALL THREE constructors — a kind that stopped \
         checking is a route a reader can paste and watch dead-end",
    );

    // …and the SUBJECT half, which is the commonest spelling of the defect: an unquoted
    // path with a space is not one bad token but two inert ones, so the word boundary comes
    // from the finding's own located address rather than from the text.
    let spaced_unquoted = std::panic::catch_unwind(|| {
        let _ = Finding::graded(
            Severity::Blocking,
            "file-state.untracked",
            "an untracked managed file",
            Some(Location::addressed(SPACED_PATH, 1, 1)),
            Some(Route::human(format!(
                "stage it with `git add -- {SPACED_PATH}`"
            ))),
        );
    });
    let spaced_quoted = std::panic::catch_unwind(|| {
        let _ = Finding::graded(
            Severity::Blocking,
            "file-state.untracked",
            "an untracked managed file",
            Some(Location::addressed(SPACED_PATH, 1, 1)),
            Some(Route::human(format!(
                "stage it with `git add -- {}`",
                shell_token(SPACED_PATH),
            ))),
        );
    });
    std::panic::set_hook(hushed);
    assert!(
        spaced_unquoted.is_err() && spaced_quoted.is_ok(),
        "the subject fence must fire on the word boundary the span fence cannot see, and \
         only there — unquoted panicked: {}, quoted panicked: {}",
        spaced_unquoted.is_err(),
        spaced_quoted.is_err(),
    );
}

/// …and the driven half: the **emitted bytes** of a route over a spaced path are runnable.
///
/// The emitted bytes are the contract, so this lifts the command out of the printed surface
/// and executes it verbatim through a real shell rather than reconstructing it — a route
/// that reads right and runs wrong is the whole defect.
#[test]
fn a_route_over_a_spaced_path_is_runnable_as_emitted() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let spaced = "my notes.md";
    fs::write(
        repo.join(spaced),
        "# Some decision\n\nWe chose the simple thing.\n",
    )
    .expect("write the spaced foreign source");

    let out = corpus.jigc(&["migrate", spaced, "--as", "adr"]);
    let text = surface(&out);
    assert!(
        !out.status.success(),
        "an untracked in-repo source is refused — git holds no copy of it; output:\n{text}",
    );
    let route = text
        .lines()
        .find_map(|line| line.trim_start().strip_prefix("route: "))
        .unwrap_or_else(|| panic!("the refusal carries a route; output:\n{text}"));
    let span = route
        .split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("the route names a command in a backticked span: {route}"));
    assert!(
        span.contains(spaced) || span.contains("notes.md"),
        "…and that command names the file the operator typed: {span}",
    );

    let argv = support::shell_words(span, &repo, &corpus.home());
    assert!(
        argv.iter().any(|word| word == spaced),
        "run through a real shell, the emitted span must re-lex the path as ONE word — \
         `{span}` split to {argv:?}, which is the dead end the fence exists to stop",
    );
    let ran = Command::new(&argv[0])
        .args(&argv[1..])
        .current_dir(&repo)
        .env("HOME", corpus.home())
        .output()
        .unwrap_or_else(|err| panic!("`{span}`: {err}"));
    assert!(
        ran.status.success(),
        "…and the command the route names must be one the tool accepts — `{span}` exited \
         {}:\n{}",
        ran.status,
        String::from_utf8_lossy(&ran.stderr),
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 7 — the composed doors (a DERIVATION STATED AS ONE × the two compose
//         doors, plus the empty-enumeration set)
// ═════════════════════════════════════════════════════════════════════════════

/// The **verb-routed workflow set**, derived: every workflow either embedded pack ships
/// whose `suppressed:` block declares a `door:`, as `(workflow id, door)`.
///
/// Stated as a derivation because the set is the pack's: a fifteenth member joins this sweep
/// with no edit here, and a member that loses its `door` leaves it the same way. Read
/// through the production loader from the packs the binary itself composes.
fn verb_routed_members() -> BTreeMap<String, String> {
    use engine::packsource::{PackResourceKind, PackSource, ResourceId};
    let mut out = BTreeMap::new();
    for pack in [
        cli::pack::EmbeddedPack::new(),
        cli::pack::EmbeddedPack::methodology(),
    ] {
        for id in pack.list(PackResourceKind::Workflows) {
            let id = id.as_str().to_owned();
            let bytes = pack
                .read(PackResourceKind::Workflows, &ResourceId::from(id.as_str()))
                .unwrap_or_else(|err| panic!("`{id}` must read from its pack: {err:?}"));
            let def = engine::compose::load_workflow_def(&bytes)
                .unwrap_or_else(|f| panic!("`{id}` must load: {}", f.message));
            if let Some(door) = def.suppressed.and_then(|s| s.door) {
                out.insert(id, door);
            }
        }
    }
    assert!(
        out.len() >= 14,
        "the derived verb-routed set is {} members — a sweep over a collapsed derivation \
         would pass vacuously",
        out.len(),
    );
    out
}

/// **Every verb-routed workflow, refused at both compose doors, with a route that RUNS.**
///
/// The composite this adds over `verb_routed_compose.rs`: that suite owns the refusal's
/// envelope shape and the non-member control. This one asserts the **agent's** half — the
/// route each refusal hands back is a `jigc` command line the real CLI parses, checked
/// through `cli::route_fence::accepts` after a real shell has split the emitted bytes. A
/// refusal that closed the dead end and opened a second one would pass every shape check
/// and fail here.
#[test]
fn every_verb_routed_workflow_is_refused_with_a_route_the_cli_parses() {
    let corpus = TrialCorpus::build(State::Fresh);
    let members = verb_routed_members();
    let mut cells = 0usize;

    for (member, door) in &members {
        for argv in [
            vec!["start", "--workflow", member, "compose me"],
            vec!["workflow", member, "--preview"],
        ] {
            cells += 1;
            let shown = argv.join(" ");
            let out = corpus.jigc(&argv);
            let text = surface(&out);
            assert!(
                !out.status.success(),
                "`jigc {shown}` must refuse — this workflow's body reads an input only its \
                 own door binds, so composed by name it renders over nothing; output:\n{text}",
            );
            assert!(
                text.contains(cli::start::VERB_ROUTED_CODE),
                "…under `{}`; output:\n{text}",
                cli::start::VERB_ROUTED_CODE,
            );
            let route = text
                .lines()
                .find_map(|line| line.trim_start().strip_prefix("route: "))
                .unwrap_or_else(|| panic!("`jigc {shown}`'s refusal carries a route:\n{text}"));
            assert!(
                route.contains(door),
                "…naming the REAL door `{door}` rather than a second dead end; got: {route}",
            );
            let span = route
                .split('`')
                .nth(1)
                .unwrap_or_else(|| panic!("the route names a command in a span: {route}"));
            // **Not** split by a real shell here, and the reason is the route's own kind:
            // several of these doors name a `<placeholder>` the agent fills (`jigc migrate
            // <path> --as adr`), and a shell reads `<path>` as a redirection. What such a
            // route promises is that it parses against the real CLI under the **declared**
            // substitution table, which is exactly the question `cli::route_fence::accepts`
            // asks — the same fence the constructor asserts with in debug posture.
            let words: Vec<String> = span.split_whitespace().map(str::to_owned).collect();
            assert!(
                cli::route_fence::accepts(&words),
                "…and the emitted bytes must parse against the real CLI — `{span}` split to \
                 {words:?}",
            );
        }
    }
    assert_eq!(
        cells,
        members.len() * 2,
        "every derived member owes a cell at both compose doors",
    );

    // The control: a workflow that is NOT verb-routed still composes at both doors. A
    // refusal that fires one workflow too wide is the same defect pointed the other way.
    for argv in [
        vec!["start", "--workflow", "single-task", "compose me"],
        vec!["workflow", "single-task", "--preview"],
    ] {
        let out = corpus.jigc(&argv);
        assert!(
            out.status.success(),
            "`jigc {}` must still compose; output:\n{}",
            argv.join(" "),
            surface(&out),
        );
    }
}

/// **A legitimately-empty enumeration states its empty case** — the other half of the same
/// law: a composing door never hands an agent a list that is not there.
///
/// The clause is deliberately **conditional** static prose, so it renders over a populated
/// store too and stays true there; what this asserts is that the render really is empty in
/// the cell the claim is about, and that the asserting sentence carries its own empty case
/// rather than pointing at nothing.
#[test]
fn a_legitimately_empty_enumeration_states_its_empty_case() {
    let corpus = TrialCorpus::build(State::Fresh);
    let composed = corpus.jigc_ok(&["workflow", "implement-from-spec", "--preview"]);
    assert!(
        !composed
            .lines()
            .any(|line| line.trim_start().starts_with("- spec:")),
        "the cell the claim is about is a store with NO committed spec — if one is LISTED \
         here the arm is proving nothing (the static prose names the `spec:` address form, \
         which is not a listing):\n{composed}",
    );
    // The composed body is hard-wrapped, so a clause is matched over whitespace-collapsed
    // text: what is asserted is the sentence, not where the renderer broke its lines.
    let flat = composed.split_whitespace().collect::<Vec<_>>().join(" ");
    for clause in [
        "nothing is listed until a `spec` is committed",
        "nothing appears until you bind a spec above",
    ] {
        assert!(
            flat.contains(clause),
            "an asserting sentence over an empty set must carry its own empty case — \
             `{clause}` is missing from:\n{composed}",
        );
    }
}

/// **The finalize seam does what the composed body says** — the third cell of the same law,
/// one layer down: a migrate-shaped workflow reached *by name* composes the exit-4 review
/// hold in its own body, so a plain `jigc task finalize` owes the hold rather than a commit.
///
/// The subject is the **composed contract**, not the staged source: a body that promises the
/// hold makes the promise whichever home the definition lives in, which is why this cell is
/// driven through the project cascade layer — the one home the seam could not reach.
#[test]
fn a_source_less_migrate_shaped_task_holds_at_exit_four_on_a_plain_finalize() {
    let corpus = TrialCorpus::build(State::Fresh);
    let dir = corpus.repo().join(".jigc").join("config").join("workflows");
    fs::create_dir_all(&dir).expect("create the project layer's workflows dir");
    fs::write(
        dir.join("single-task.yaml"),
        "---\n\
         when: implement a single scoped change\n\
         description: A project-layer shadow whose body composes the migration review hold.\n\
         usage: flow 53 arm 7's composed-contract cell.\n\
         creates-task: true\n\
         allows-create: [{type: adr, as: decision}]\n\
         ---\n\
         {{ include: step:locate }}\n\
         {{ include: step:migration-finalize }}\n",
    )
    .expect("write the project shadow");

    let task = corpus.start_workflow("single-task", "shadow the hold");
    let composed = corpus.jigc_ok(&["start", "--task", &task]);
    assert!(
        composed.contains("commits NOTHING"),
        "the composed body is what makes the promise — assert it before asserting the hold, \
         or a pack edit that drops the step hollows this cell silently:\n{composed}",
    );

    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Shadowed Decision",
            "--task",
            &task,
        ])
        .trim_end_matches('\n')
        .to_string();
    for (slot, prose) in [
        ("context", "The forces that made the decision necessary."),
        ("decision", "We will keep the shadow."),
        ("consequences", "Every deployment now carries it."),
    ] {
        corpus.set_slot(&format!("{address}#{slot}"), &task, prose);
    }
    corpus.set_field(&format!("commit:{task}#type"), &task, "docs");
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "shadow the hold");

    let before = corpus.git(&["rev-parse", "HEAD"]);
    let held = corpus.jigc(&["task", "finalize", &task]);
    let stdout = String::from_utf8_lossy(&held.stdout).into_owned();
    assert_eq!(
        held.status.code(),
        Some(4),
        "a promise the composed body makes is still a promise, whichever layer the \
         definition lives in; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&held.stderr),
    );
    assert!(
        stdout.contains("migration review required") && stdout.contains("no foreign source"),
        "…and the source-less hold names itself and states its empty case:\n{stdout}",
    );
    assert_eq!(
        before,
        corpus.git(&["rev-parse", "HEAD"]),
        "the review hold commits NOTHING — HEAD must be unmoved",
    );
}

/// **Increment 8's rider on this arm: a repository with no `jigc setup` is a pre-dispatch
/// state too, and every `milestone` door answers it.**
///
/// A milestone minted into a repository with no project layer would live only in the
/// gitignored workbench — which no clone sees and no fresh clone can re-derive, the inverse
/// of the settlement that the committed record is the source of truth. The answer rides the
/// **flattened `{error}` arm deliberately**, consistent with the leaves that already answer
/// that way, and that decision is asserted here rather than left as an observation: the key
/// set is read off `ENVELOPE_ARMS`' own operational reject row, so a door that moved arms
/// reddens.
#[test]
fn every_milestone_door_refuses_a_repository_that_was_never_set_up() {
    let repo = TempDir::new("not-set-up");
    let home = TempDir::new("not-set-up-home");
    git(repo.path(), &["init", "-q", "-b", "main", "."]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-qm", "initial"]);

    let error_keys = reject_arm_keys("Reject::Error");
    let mut doors = 0usize;
    for (leaf, _) in VERB_KINDS {
        if leaf.first() != Some(&"milestone") {
            continue;
        }
        doors += 1;
        let tail = leaf_argv::MINIMAL_ARGV
            .iter()
            .find(|(path, _)| path == leaf)
            .map(|(_, tail)| *tail)
            .unwrap_or_else(|| {
                panic!(
                    "`jigc {}` has no row in `support::leaf_argv::MINIMAL_ARGV`",
                    leaf.join(" "),
                )
            });
        let argv: Vec<&str> = leaf
            .iter()
            .copied()
            .chain(tail.iter().copied())
            .chain(["--format", "json"])
            .collect();
        let shown = leaf.join(" ");
        let out = jigc(repo.path(), home.path(), &argv);
        let doc = one_reject_document(&out, &shown);
        assert_eq!(
            top_level_keys(&doc),
            error_keys,
            "`jigc {shown}` answers the not-set-up state on the declared operational reject \
             arm; got:\n{doc:#}",
        );
        let rendered = serde_json::to_string(&doc).expect("re-render the reject");
        assert!(
            rendered.contains("jigc setup"),
            "…routing at the one command that makes the state legal; got:\n{doc:#}",
        );
    }
    assert!(
        doors >= 8,
        "the milestone family is {doors} leaves — a sweep over a collapsed set would pass \
         vacuously",
    );
    assert!(
        !repo.path().join(".jigc").exists(),
        "…and NOTHING is written: a door that refused after minting the workbench would \
         have created the very state it refuses",
    );
}
