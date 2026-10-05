//! M52 Increment 5 / T8 — **`jigc config set <root-knob>`'s relocation gains a rollback, and
//! `config.rollback-conflict`** (`completions/artifacts/M52/settle-record.md` → **D1.5**
//! (population 10) and **§14**; `DECISIONS.md` → 2026-09-17 M52 Increment 1 / T1, the declared
//! residual; `design/validation.md` → The M52 registrations — Increment 5).
//!
//! **The gap this closes.** A root-knob re-point is two acts that only make sense together:
//! *move the committed docs the re-point would strand*, then *land the knob that justifies the
//! move*. `run_set` ran the two move floors and **then** `write_scalar(…)?`, with nothing
//! between a failing knob write and a store whose docs are somewhere the cascade does not
//! point. Driven at the wave's base on a `committed-singletons` rig whose
//! `.jigc/config/manifest.yaml` was made read-only:
//!
//! ```text
//! $ jigc config set placement-root notes
//!   - docs/decisions-log.md → notes/decisions-log.md
//!   - docs/roadmap.md → notes/roadmap.md
//! could not write …/.jigc/config/manifest.yaml: Permission denied (os error 13)   … $? = 1
//! $ git status --porcelain
//! R  docs/decisions-log.md -> notes/decisions-log.md
//! R  docs/roadmap.md -> notes/roadmap.md
//! $ jigc config get placement-root
//! placement-root =   (pack-default)
//! ```
//!
//! — exit 1, every doc moved and staged, every file-state key re-pointed, and the knob that
//! was the whole reason for the move **unchanged**. That is `ROLLBACK_POPULATIONS`'
//! `config-root-relocation` row: the one population in the registry that had no rollback of
//! any kind, carried there *because* it was missing.
//!
//! **Two triggers, one transaction.** The knob write failing is one; a doc that could not be
//! relocated is the other, and it is the **discharge of Increment 1 / T1's declared residual**
//! — the `could not relocate …` narration had no `--format json` carrier and was named
//! nowhere, because a per-doc failure exited 0. A failed move is now a rollback trigger, so
//! the fact reaches the findings arm with a `config.*` identity.
//!
//! **The raced cell is manufactured, and this suite says so.** Every other `FileCas` door's
//! window contains the user's `pre-commit` hook, so its race is driven through git. `config
//! set`'s window spawns `git mv` and `git restore` and **no hook at all**, so the design's
//! named racer cannot enter it: what the compare-and-swap is there for is a *concurrent
//! process* — an editor, a watcher, a second `jigc` — which no test can schedule
//! deterministically. So the raced cell is built at the CAS seam itself, on
//! `config_layer_preimage.rs`'s injector mold (that suite's own shape space is manufactured
//! for the same reason, and says so), driving `cli::rollback`'s entry exactly as the door
//! builds it.

use std::fs;
use std::path::Path;

use crate::support;

use cli::rollback::{CONFIG_DOOR, PreImage, PreImageFamily};
use serde_json::Value;
use support::trial_corpus::{State, TrialCorpus};

/// The transaction's own refusal — one per cause, keyed at the path the cause names.
const REPOINT_FAILED: &str = "config.repoint-failed";

/// The raced-restore refusal this door mints, on the M51 mold with its own code.
const CONFLICT: &str = "config.rollback-conflict";

// ---------------------------------------------------------------------------
// the axis: both root knobs, because a rule applied at one arm is not applied
// ---------------------------------------------------------------------------

/// One root knob's relocating cell: the corpus its re-point strands, the value to set, and
/// the committed docs that move.
struct Knob {
    /// The knob key, as `jigc config set` spells it.
    key: &'static str,
    /// A home nothing is at yet, so the re-point genuinely strands the corpus.
    value: &'static str,
    /// The corpus whose committed docs this knob's re-point relocates.
    corpus: fn() -> TrialCorpus,
    /// The moves the re-point makes, as `(prior home, new home)` — reality this suite
    /// asserts *against*, established non-vacuously by the success control below.
    moves: &'static [(&'static str, &'static str)],
}

/// The `docs-root` cell's corpus: one committed ADR at `docs/decisions/cache-strategy.md`,
/// minted through `record-decision` and landed by a real `jigc task finalize`. A `location`
/// doctype, which is what `docs-root` re-roots — every doc `State::CommittedSingletons`
/// commits is a `placement` doctype, whose home bypasses `docs-root` entirely
/// (`design/storage.md` → Placement), so that state cannot exercise this knob.
fn adr_corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "record a cache decision");
    let title = support::create_title("adr", "Cache Strategy");
    corpus.jigc_ok(&["doc", "create", "adr", "--title", &title, "--task", &task]);
    for slot in ["context", "decision", "consequences"] {
        corpus.set_slot(
            &format!("adr:cache-strategy#{slot}"),
            &task,
            &format!("Prose for {slot}.\n"),
        );
    }
    corpus.finalize(&task, "docs", "record the cache decision", false);
    corpus
}

/// The `placement-root` cell's corpus: the committed singleton set, two of whose instances
/// declare a home with a leading directory component and so move when the placement root does.
fn singleton_corpus() -> TrialCorpus {
    TrialCorpus::build(State::CommittedSingletons)
}

const KNOBS: &[Knob] = &[
    Knob {
        key: "docs-root",
        value: "documents",
        corpus: adr_corpus,
        moves: &[(
            "docs/decisions/cache-strategy.md",
            "documents/decisions/cache-strategy.md",
        )],
    },
    Knob {
        key: "placement-root",
        value: "notes",
        corpus: singleton_corpus,
        moves: &[
            ("docs/decisions-log.md", "notes/decisions-log.md"),
            ("docs/roadmap.md", "notes/roadmap.md"),
        ],
    },
];

// ---------------------------------------------------------------------------
// fixture helpers
// ---------------------------------------------------------------------------

/// Seed the project manifest with a harmless knob and then make it **unwritable**, so the
/// re-point's own `write_scalar` fails *after* both move floors have run — the failure point
/// the population's whole defect hangs on.
///
/// The seed is deliberate: `jigc setup` writes no `manifest.yaml`, so without it the failing
/// write would be a *create* into a directory, which is a different fault at a different path.
fn freeze_manifest(corpus: &TrialCorpus) {
    corpus.jigc_ok(&["config", "set", "default-workflow", "single-task"]);
    let manifest = corpus
        .repo()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    let mut perms = fs::metadata(&manifest)
        .expect("manifest exists")
        .permissions();
    perms.set_readonly(true);
    fs::set_permissions(&manifest, perms).expect("freeze the manifest");
}

/// Freeze an already-seeded manifest again — the read-only bit alone, without a second seed.
fn refreeze_manifest(corpus: &TrialCorpus) {
    let manifest = corpus
        .repo()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    let mut perms = fs::metadata(&manifest)
        .expect("manifest exists")
        .permissions();
    perms.set_readonly(true);
    fs::set_permissions(&manifest, perms).expect("freeze the manifest");
}

/// Undo [`freeze_manifest`] so the corpus can be read back with `jigc config get` — and so the
/// temp-dir teardown does not trip over a read-only file.
fn thaw_manifest(corpus: &TrialCorpus) {
    let manifest = corpus
        .repo()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    if let Ok(meta) = fs::metadata(&manifest) {
        let mut perms = meta.permissions();
        #[allow(clippy::permissions_set_readonly_false)]
        perms.set_readonly(false);
        let _ = fs::set_permissions(&manifest, perms);
    }
}

/// The bytes at `rel`, or `None` when nothing is there.
fn bytes_at(repo: &Path, rel: &str) -> Option<Vec<u8>> {
    fs::read(repo.join(rel)).ok()
}

// ---------------------------------------------------------------------------
// (1) the knob write fails: the transaction is undone, on both knobs
// ---------------------------------------------------------------------------

#[test]
fn a_failed_knob_write_puts_every_relocated_doc_back_and_leaves_git_as_it_found_it() {
    for knob in KNOBS {
        let corpus = (knob.corpus)();
        let repo = corpus.repo();

        // The manifest is seeded and frozen FIRST, so the state snapshotted below is the one
        // the run must be indistinguishable from — the seed writes an untracked
        // `.jigc/config/manifest.yaml`, which `git status` names.
        freeze_manifest(&corpus);

        // The state the run must be indistinguishable from afterwards.
        let before_status = corpus.git(&["status", "--porcelain"]);
        let before_bytes: Vec<(String, Vec<u8>)> = knob
            .moves
            .iter()
            .map(|(prior, _)| {
                (
                    (*prior).to_string(),
                    bytes_at(&repo, prior).unwrap_or_else(|| {
                        panic!(
                            "{}: the corpus must hold `{prior}` before the run",
                            knob.key
                        )
                    }),
                )
            })
            .collect();
        thaw_manifest(&corpus);
        let before_record = fs::read(repo.join(".jigc").join("state").join("file-state.json"))
            .expect("the corpus has a file-state record");
        let before_knob = corpus.jigc_ok(&["config", "get", knob.key]);
        refreeze_manifest(&corpus);

        let out = corpus.jigc(&["config", "set", knob.key, knob.value]);
        thaw_manifest(&corpus);

        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert_eq!(
            out.status.code(),
            Some(1),
            "`jigc config set {} {}` must refuse when the knob it justifies the moves with \
             cannot be written; stderr:\n{stderr}",
            knob.key,
            knob.value,
        );
        assert!(
            stderr.contains(REPOINT_FAILED),
            "the refusal carries the transaction's own identity `{REPOINT_FAILED}`; \
             stderr:\n{stderr}",
        );

        for (prior, new) in knob.moves {
            let expected = &before_bytes
                .iter()
                .find(|(rel, _)| rel == prior)
                .expect("captured")
                .1;
            assert_eq!(
                bytes_at(&repo, prior).as_ref(),
                Some(expected),
                "`{prior}` must be back at its prior home byte-intact after the rollback; \
                 stderr:\n{stderr}",
            );
            assert!(
                bytes_at(&repo, new).is_none(),
                "`{new}` is the copy the re-point created — the rollback removes it, or the \
                 doc is at two homes at once; stderr:\n{stderr}",
            );
        }

        assert_eq!(
            corpus.git(&["status", "--porcelain"]),
            before_status,
            "the staged `git mv` is un-staged too — a rollback that puts the bytes back and \
             leaves the index re-pointed has moved the inconsistency, not removed it; \
             stderr:\n{stderr}",
        );
        assert_eq!(
            fs::read(repo.join(".jigc").join("state").join("file-state.json"))
                .expect("the file-state record survives"),
            before_record,
            "the gitignored file-state record is put back too — `move_doc` re-keys it on every \
             call, and a record keyed to homes no doc is at is the same inconsistency git \
             cannot show; stderr:\n{stderr}",
        );
        assert_eq!(
            corpus.jigc_ok(&["config", "get", knob.key]),
            before_knob,
            "the knob never landed, which is the whole reason the moves are undone",
        );
    }
}

// ---------------------------------------------------------------------------
// (2) the machine arm: one document on stderr, carrying the failure
// ---------------------------------------------------------------------------

#[test]
fn the_refusal_is_one_json_document_on_stderr_and_stdout_is_empty() {
    let knob = &KNOBS[1]; // the two-doc cell, so a driver sees the whole transaction
    let corpus = (knob.corpus)();

    freeze_manifest(&corpus);
    let out = corpus.jigc(&["--format", "json", "config", "set", knob.key, knob.value]);
    thaw_manifest(&corpus);

    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(out.status.code(), Some(1), "the reject exits 1:\n{stderr}");
    assert!(
        stdout.trim().is_empty(),
        "on a reject the document is stderr's and stdout is empty \
         (`design/command-output-contract.md` → Stream discipline); stdout:\n{stdout}",
    );

    let doc: Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
        panic!(
            "stderr must parse WHOLE as exactly one document ({err}) — the relocation \
             narration is withheld under `--format json`, and a finding printed beside the \
             envelope is the shape M52 Increment 1 closed:\n{stderr}"
        )
    });
    let findings = doc
        .get("findings")
        .and_then(Value::as_array)
        .unwrap_or_else(|| {
            panic!(
                "a reject that carries a finding takes the findings arm, with the operational \
                 error itself as a finding; got:\n{doc:#}"
            )
        });
    let codes: Vec<&str> = findings
        .iter()
        .filter_map(|f| f.get("code").and_then(Value::as_str))
        .collect();
    assert!(
        codes.contains(&REPOINT_FAILED),
        "the failure rides the document a driver reads, not a stderr line it must drop; \
         codes: {codes:?}\n{doc:#}",
    );
}

// ---------------------------------------------------------------------------
// (3) the control: an ordinary re-point is untouched
// ---------------------------------------------------------------------------

#[test]
fn a_successful_repoint_still_moves_the_docs_and_acks_what_it_moved() {
    for knob in KNOBS {
        let corpus = (knob.corpus)();
        let repo = corpus.repo();

        let stdout = corpus.jigc_ok(&["--format", "json", "config", "set", knob.key, knob.value]);
        let doc: Value = serde_json::from_str(&stdout).expect("the ack is one JSON document");
        let relocated: Vec<(String, String)> = doc
            .get("relocated")
            .and_then(Value::as_array)
            .expect("`relocated` is present on every arm")
            .iter()
            .map(|row| {
                let field = |name: &str| {
                    row.get(name)
                        .and_then(Value::as_str)
                        .expect("each row is {from, to}")
                        .to_string()
                };
                (field("from"), field("to"))
            })
            .collect();
        let mut expected: Vec<(String, String)> = knob
            .moves
            .iter()
            .map(|(from, to)| ((*from).to_string(), (*to).to_string()))
            .collect();
        expected.sort();
        let mut got = relocated;
        got.sort();
        assert_eq!(
            got, expected,
            "`{}`'s successful re-point must still move exactly what it moved before, and \
             still say so on `relocated`",
            knob.key,
        );

        for (prior, new) in knob.moves {
            assert!(
                bytes_at(&repo, new).is_some() && bytes_at(&repo, prior).is_none(),
                "`{prior}` → `{new}` landed",
            );
        }
        assert!(
            corpus
                .jigc_ok(&["config", "get", knob.key])
                .contains(knob.value),
            "…and the knob that justifies the moves landed with them",
        );
    }
}

// ---------------------------------------------------------------------------
// (4) the second trigger: a doc that could not be relocated
// ---------------------------------------------------------------------------

/// A move that **fails** is the transaction's other trigger, and the discharge of Increment
/// 1 / T1's declared residual: through rc.15 the per-doc failure narrated
/// `could not relocate … — move it by hand` on stderr, was withheld entirely under
/// `--format json`, and the knob landed anyway — so the store pointed at a home the doc was
/// not at, which is the state step 2b's own rationale says must never exist.
///
/// The destination is made un-writable rather than un-trackable: every *un-trackable* shape
/// is refused at the door before anything moves (M49's HIGH), so a permission fault is what
/// is left that reaches the mover at all.
#[test]
#[cfg(unix)]
fn a_move_that_fails_rolls_the_transaction_back_instead_of_landing_the_knob() {
    use std::os::unix::fs::PermissionsExt;

    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let repo = corpus.repo();
    let before_status = corpus.git(&["status", "--porcelain"]);
    let before_roadmap = bytes_at(&repo, "docs/roadmap.md").expect("the corpus holds the roadmap");

    // A destination directory git cannot write into: `create_dir_all` finds it already there
    // and the `git mv` into it fails.
    let dest = repo.join("notes");
    fs::create_dir_all(&dest).expect("create the destination");
    fs::set_permissions(&dest, fs::Permissions::from_mode(0o500)).expect("freeze the destination");

    let out = corpus.jigc(&["config", "set", "placement-root", "notes"]);

    fs::set_permissions(&dest, fs::Permissions::from_mode(0o755)).expect("thaw the destination");

    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(
        out.status.code(),
        Some(1),
        "a re-point that could not move a doc it is re-pointing must not land the knob; \
         stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(REPOINT_FAILED),
        "…and the per-doc failure reaches the surface with an identity, not as a bare \
         narration line; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("parked"),
        "nothing was parked here, so the route says nothing about a parked file; \
         stderr:\n{stderr}",
    );
    assert_eq!(
        bytes_at(&repo, "docs/roadmap.md").as_ref(),
        Some(&before_roadmap),
        "every doc is at its prior home; stderr:\n{stderr}",
    );
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        before_status,
        "…and git is as the run found it; stderr:\n{stderr}",
    );
    assert!(
        !corpus
            .jigc_ok(&["config", "get", "placement-root"])
            .contains("notes"),
        "the knob did not land",
    );
}

/// **A refused re-point says which foreign file it parked and did not put back** (the rc.24
/// fix pass's completion audit, CPL-8).
///
/// The sweep moves a foreign file out of a new home before it moves the doc in, and the
/// undo behind a refusal puts the *docs* back and leaves that file in the gitignored
/// workbench (the declared bound). The refusal's route read *every doc this re-point moved
/// is back at its prior home* and stopped — true, and silent about a file that was no
/// longer where the reader had put it. Driven before this: only a stderr narration line of
/// the refused run named the move, and under `--format json` nothing did.
///
/// The trigger is the pass's own refusal: the second doc of the sweep stands at a
/// committed link, which no door moves, after the first doc's destination held a squatter.
/// Both formats are driven; the squatter-less control is the sibling above, whose route
/// must go on saying nothing about a parked file.
#[test]
#[cfg(unix)]
fn a_refused_repoint_names_the_foreign_file_it_parked_and_did_not_put_back() {
    for json in [false, true] {
        let cell = if json { "--format json" } else { "text" };
        let corpus = TrialCorpus::build(State::CommittedSingletons);
        let repo = corpus.repo();
        corpus.git(&["mv", "docs/roadmap.md", "docs/roadmap-real.md"]);
        std::os::unix::fs::symlink("roadmap-real.md", repo.join("docs/roadmap.md"))
            .expect("make the roadmap's home a link");
        corpus.git(&["add", "docs/roadmap.md"]);
        corpus.git(&["commit", "-q", "-m", "the roadmap home is a link"]);
        fs::create_dir_all(repo.join("handbook")).expect("mk the new home");
        fs::write(repo.join("handbook/decisions-log.md"), "SQUATTER-MARKER\n")
            .expect("write the squatter");
        let before_log = bytes_at(&repo, "docs/decisions-log.md").expect("the corpus holds it");
        // One cell also has an earlier displacement's file already parked under the same
        // name: this door parks through the same function `jigc relocate` does, so the
        // squatter goes beside it, never over it — and the route names where it went.
        let parked = if json {
            fs::create_dir_all(repo.join(".jigc/displaced")).expect("mk the parking home");
            fs::write(
                repo.join(".jigc/displaced/decisions-log.md"),
                "EARLIER-PARKED\n",
            )
            .expect("park an earlier file");
            ".jigc/displaced/decisions-log.md.2"
        } else {
            ".jigc/displaced/decisions-log.md"
        };

        let mut args = vec!["config", "set", "placement-root", "handbook"];
        if json {
            args.splice(0..0, ["--format", "json"]);
        }
        let out = corpus.jigc(&args);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert_eq!(
            out.status.code(),
            Some(1),
            "{cell}: the re-point is refused; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains(REPOINT_FAILED),
            "{cell}: …under its own code; stderr:\n{stderr}",
        );

        // The docs are all-or-nothing, as they were.
        assert_eq!(
            bytes_at(&repo, "docs/decisions-log.md").as_ref(),
            Some(&before_log),
            "{cell}: the doc that had moved is back at its prior home",
        );
        // The squatter is where the sweep parked it, bytes intact…
        assert_eq!(
            fs::read_to_string(repo.join(parked)).ok().as_deref(),
            Some("SQUATTER-MARKER\n"),
            "{cell}: the squatter's bytes survive in the workbench; stderr:\n{stderr}",
        );
        if json {
            assert_eq!(
                fs::read_to_string(repo.join(".jigc/displaced/decisions-log.md"))
                    .ok()
                    .as_deref(),
                Some("EARLIER-PARKED\n"),
                "{cell}: …beside the file already parked under that name, which is intact",
            );
        }
        assert!(
            !repo.join("handbook/decisions-log.md").exists(),
            "{cell}: …and it is not back where it stood (the declared bound)",
        );

        // …and the refusal's own statement of what was undone says so, naming both paths.
        let route = if json {
            let doc: serde_json::Value =
                serde_json::from_str(stderr.trim()).expect("stderr is one JSON document");
            doc["findings"]
                .as_array()
                .expect("a findings array")
                .iter()
                .find(|finding| finding["code"] == REPOINT_FAILED)
                .map(|finding| finding["route"].to_string())
                .unwrap_or_else(|| panic!("{cell}: no `{REPOINT_FAILED}` finding:\n{stderr}"))
        } else {
            stderr
                .lines()
                .filter(|line| line.trim_start().starts_with("route:"))
                .find(|line| line.contains("is unchanged"))
                .unwrap_or_else(|| panic!("{cell}: no route states the undo:\n{stderr}"))
                .to_owned()
        };
        assert!(
            route.contains("handbook/decisions-log.md") && route.contains(&format!("`{parked}`")),
            "{cell}: the route that says what was undone must name the file that was not \
             put back, and where it is; route:\n{route}",
        );

        // The route's own re-run, once what the message names is fixed: the link replaced
        // by the doc itself, as the refusal says. The squatter's old place is free now.
        fs::remove_file(repo.join("docs/roadmap.md")).expect("remove the link");
        fs::rename(
            repo.join("docs/roadmap-real.md"),
            repo.join("docs/roadmap.md"),
        )
        .expect("put the doc itself at its home");
        corpus.git(&["add", "-A", "docs"]);
        corpus.git(&["commit", "-q", "-m", "the roadmap is a regular file again"]);
        let rerun = corpus.jigc(&["config", "set", "placement-root", "handbook"]);
        assert!(
            rerun.status.success(),
            "{cell}: the route's re-run lands; stderr:\n{}",
            String::from_utf8_lossy(&rerun.stderr),
        );
        assert!(
            repo.join("handbook/roadmap.md").is_file()
                && repo.join("handbook/decisions-log.md").is_file(),
            "{cell}: …with both docs at the new home",
        );
    }
}

// ---------------------------------------------------------------------------
// (5) the manufactured raced cell, at the CAS seam
// ---------------------------------------------------------------------------

/// **The raced cell, manufactured — and the module header says why it has to be.**
///
/// The entry is built exactly as the door builds it: the doc's prior home carries the bytes
/// jigc moved away and a post-image of *absence* (`PreImage::removed`), the destination
/// carries an absent pre-image and the bytes jigc wrote. A third party then writes at the
/// prior home before the rollback runs — and the restore must leave those bytes alone, park
/// jigc's pre-image under `.jigc/displaced/config/`, and name both copies.
#[test]
fn a_racer_at_the_prior_home_keeps_its_bytes_and_the_pre_image_is_parked() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let jigc_root = repo.join(".jigc");

    let prior = repo.join("docs").join("note.md");
    let landed = repo.join("notes").join("note.md");
    fs::create_dir_all(prior.parent().expect("parent")).expect("mkdir docs");
    fs::create_dir_all(landed.parent().expect("parent")).expect("mkdir notes");

    // The move, replayed in the door's own order: the destination is captured **before** the
    // move (absent), the move lands, what jigc left there is read back, and the vacated home
    // is registered only once the move has actually succeeded — a removed-entry pushed over a
    // move that failed would raise a conflict about a file jigc never touched (T3's rule).
    let mut family = PreImageFamily::empty(CONFIG_DOOR);
    family
        .push(PreImage::capture("notes/note.md", landed.clone()).expect("capture the destination"));
    fs::write(&landed, b"the doc jigc moved\n").expect("write the landing");
    family.wrote("notes/note.md");
    family.push(PreImage::removed(
        "docs/note.md",
        prior.clone(),
        b"the doc jigc moved\n".to_vec(),
    ));

    // The racer: a concurrent process writes at the home jigc vacated.
    fs::write(&prior, b"THIRD PARTY PROSE\n").expect("the racer writes");

    let conflicts = family.restore(&repo, &jigc_root);

    assert_eq!(
        fs::read(&prior).expect("the racer's file"),
        b"THIRD PARTY PROSE\n",
        "the racer's bytes stand — a restore that overwrote them would be the same loss the \
         family exists to prevent, in the other direction",
    );
    assert_eq!(
        conflicts.len(),
        1,
        "one conflict, keyed at the raced path; got: {:#?}",
        conflicts,
    );
    let conflict = &conflicts[0];
    assert_eq!(conflict.code, CONFLICT, "this door's own identity");
    assert_eq!(
        conflict
            .location
            .as_ref()
            .and_then(|l| l.address.as_deref()),
        Some("docs/note.md"),
        "…keyed at the file path, which is what discriminates two raced paths",
    );
    let route = conflict
        .route
        .as_ref()
        .expect("a raced restore routes a human");
    assert!(
        route.as_str().contains(".jigc/displaced/config/"),
        "the pre-image parks under this door's own sub-directory, so two populations sharing \
         a basename stay distinguishable; route: {}",
        route.as_str(),
    );

    let parked = repo
        .join(".jigc")
        .join("displaced")
        .join("config")
        .join("docs");
    let entries: Vec<_> = fs::read_dir(&parked)
        .unwrap_or_else(|err| panic!("the park directory {parked:?} must exist ({err})"))
        .filter_map(Result::ok)
        .collect();
    assert_eq!(entries.len(), 1, "one parked pre-image; got {entries:#?}");
    assert_eq!(
        fs::read(entries[0].path()).expect("read the parked copy"),
        b"the doc jigc moved\n",
        "…and it holds the bytes the rollback could not put back",
    );

    // The destination copy is jigc's own and nothing raced it, so it is removed.
    assert!(
        !landed.exists(),
        "the copy the re-point created is still removed — the swap held at that path",
    );
}

/// **The conflicts reach the machine surface, not a line beside it** — the carrier half of the
/// arm above, pinned where it can be: the raced cell cannot be driven through the binary (see
/// the module header), so what a driven run cannot show is asserted of the carrier the door
/// hands the funnel.
///
/// `cli::render::envelope_projecting_findings` is what
/// `cli::invocation_log::operational_failure` reads to build the one document, so a refusal
/// that carried conflicts and a funnel that rendered only the head would be exactly the
/// dropped-bytes shape M52 Increment 1 closed.
#[test]
fn a_refusal_that_carries_conflicts_hands_the_funnel_all_of_them() {
    let head = engine::finding::Finding::block(REPOINT_FAILED, "the knob was not set", "re-run it");
    let conflict =
        engine::finding::Finding::block(CONFLICT, "a raced path", "compare the two copies");
    let err = cli::render::envelope_finding_error_beside(&head, vec![conflict]);

    let carried = cli::render::envelope_projecting_findings(&err)
        .expect("the refusal declares the findings arm");
    let codes: Vec<&str> = carried.iter().map(|f| f.code.as_str()).collect();
    assert_eq!(
        codes,
        vec![REPOINT_FAILED, CONFLICT],
        "the funnel is handed the refusal AND every conflict, in that order — the head alone \
         would put the only naming of a parked pre-image outside the document a driver reads",
    );
}
