//! **The M49 pre-1.0-completion-wave done-picture acceptance suite** — the wave driven
//! end to end through the **real `jigc` binary** (`design/worked-examples.md` → flow 50;
//! roadmap → Milestone 49, Increment 12).
//!
//! **The claim the wave proves is one claim: a fence applied where its wave pointed is
//! not applied at all — and the cost of that is silent, committed data corruption.** Its
//! centrepiece was found by driving: an item's leaf region ended at the first *deeper*
//! heading, but a multi-slot item's own sub-labels **are** deeper headings, so a
//! `set-field` on any ≥2-slot item block duplicated the field bullet, committed at exit 0,
//! and `jigc validate` called it clean while the pinned `doc show --format json` served
//! the stale first value. It had never fired because the shipped corpus is safe *by
//! accident of shape*. The razor that admitted every item in the wave is the M46 razor
//! rebuilt general, with a necessity leg added because the first draft could not refuse
//! (`completions/artifacts/M49/settle-record.md` → D2), and it is falsifiable on the same
//! terms: **if the razor cannot refuse, the claim is wrong** — it refused five classes
//! with citations, each at a named leg.
//!
//! Increments 1–11 shipped each fix with its own axis suite; this suite is the
//! **composite acceptance** that ties the wave into six done-picture arms over the real
//! binary — **each arm stating which kind of set it iterates**, never pinning the single
//! repro a baseline reported.
//!
//! The six arms — **twelve `#[test]`s** over the real binary, since three of them carry
//! cells that need their own fixture world:
//!
//!   (1) **The item region survives to a commit, over the whole shape space** — and this
//!       arm's set is a **manufactured shape space, not a registry enumeration**, which is
//!       a deliberate departure from M45/M47's pattern and is stated here, in
//!       [`support::shape_space`], and in the flow-50 chapter. Across both embedded packs
//!       the shipped item blocks populate **two** of the eighteen `shape × op` cells with
//!       the ingredients the axis needs, so a suite that looped the registry would sweep
//!       two cells and report an axis it never reached. The six `(slots, nested)` shapes
//!       are **generated**, carried into a real corpus by a [`FixturePack`], and each is
//!       driven past the staged write all the way to a **`git` commit** — because the
//!       wave's claim is about *committed* corruption, and a staged-only proof would stop
//!       one step short of the cost (Inc 1).
//!
//!   (2) **Every door's report matches the state it left behind** — over two code-side
//!       registries and two door repairs: [`cli::cli::VERB_KINDS`] filtered to
//!       [`VerbKind::Read`] (every member driven here or classified against the suite that
//!       drives it, a member with no cell a hard panic),
//!       [`engine::schema::SetKind::ALL`] (the closed `set:` vocabulary, refused at
//!       pack-load with every honored member named), [`engine::state::MINT_DOORS`] (the
//!       registry T1 minted for exactly this — the snapshot disposition was a *remembered
//!       list* and the memory was wrong), plus `add-task --workflow`'s membership check
//!       and the sub-task discard that used to leave the committed record claiming
//!       `status: active` (Inc 2 + Inc 12 / T1).
//!
//!   (3) **The freeze binds at every layer that can change a schema, and the migration
//!       surface can express what the doctypes need** — the layer set is a **derivation
//!       stated as one** (the two layers a schema can arrive from: a pack file, and the
//!       project's own `.jigc/config/schemas/` shadow), and the transform set is
//!       [`engine::schema_diff::SchemaChange`] matched **exhaustively** over the *real*
//!       diff of the shipped `completion-record.v1` snapshot against the shipped v2
//!       schema, so a fourteenth kind cannot compile without deciding what it says
//!       (Inc 3 + Inc 4).
//!
//!   (4) **The limits an adopter hits first are stated truthfully, and the home stops
//!       dictating the layout** — the nesting ceiling is asserted as **one derived
//!       number** ([`engine::schema::MAX_NESTING_DEPTH`], itself derived from
//!       [`engine::address::MAX_FRAGMENT_HOPS`]) rather than as the literal `2`, so the
//!       arm follows the constant instead of pinning today's value; `doc add-item --slug`
//!       decouples an item's id from its title; and a `placement-root` re-point **moves**
//!       the committed doc it would otherwise strand (Inc 5 + Inc 7).
//!
//!   (5) **A pack extends without losing the methodology, and every schema that had to
//!       bump did** — a listed project pack adds a doctype the freeze does not govern and
//!       is **demoted** for one it does; the version set is a **derivation** over both
//!       shipped `config/schema-manifest.yaml` `doctypes:` lists, so the wave's three
//!       bumps are covered by construction rather than by being named; and
//!       `planning-record`'s gate set — **read from the shipped schema through the
//!       binary**, never hand-listed here — blocks `jigc task finalize` until it is filled
//!       (Inc 6 + Inc 9).
//!
//!   (6) **The 1.0 contract is pinned in the shape a driver consumes, and nothing the
//!       reader is told is false** — the item-addressing door set is a **derivation
//!       stated as one** over [`cli::cli::DOCTYPE_DOORS`] (its `doc` rows that take an
//!       address, minus the read verbs and minus `doc rename`, whose address names a
//!       *document* rather than a locus inside one); `doc show` carries the doc's own
//!       stamp as a **number**; a located finding says *where* on the text surface too;
//!       the route exemption is an **enumeration**, not a namespace; and the
//!       unknown-doctype axis is swept over `DOCTYPE_DOORS` with every door disposed
//!       (Inc 8 + Inc 10 + Inc 11).
//!
//! **The declared proof split.** Each arm proves the wave's claim at the *done-picture*
//! altitude; the per-fix mechanism clauses stay with the dedicated axis suites and are not
//! re-proven here: the eighteen `shape × op` cells and the `####`-heading discriminator are
//! `item_region_shape_space.rs`'s; the whole `VerbKind::Read` sweep over a revealing state
//! is `read_verb_acts_nothing.rs`'s, the `set:` vocabulary's projection arms
//! `set_kind_vocabulary.rs`'s, the mint-door completeness fence and its five driven cells
//! `mint_doors.rs`'s, the sub-task record `subtask_discard_record.rs`'s and the workflow
//! membership `milestone_workflow_membership.rs`'s; the freeze's resolution unification is
//! `schema_resolution_unified.rs`'s and `freeze_enforcement.rs`'s, the mis-key mutation
//! matrix `schema_load_strictness.rs`'s, the `AddedItemSlot` arity × requiredness axis
//! `migrate_corpus_item_slot.rs`'s; the three ceilings' reconciliation
//! `record_nesting_cap.rs`'s and the slug arms `add_item_slug.rs`'s, the placement census
//! and its move floor `placement_override.rs`'s; the project-pack precedence
//! `project_pack_composition.rs`'s, the per-doctype version rows `doctype_map_versions.rs`'s,
//! the whole-gate-axis forcing function `planning_gate_forcing.rs`'s; the seven-cell
//! nested-hop matrix `write_miss_shape_axis.rs`'s, the fifteen-door unknown-doctype axis
//! `unknown_doctype_axis.rs`'s, the not-a-git-repo axis `not_in_repo_axis.rs`'s, the clap
//! error-kind axis `clap_error_kind_axis.rs`'s, and the delegation-prose dispositions
//! `methodology_delegation_prose.rs`'s.
//!
//! **Red at the wave's base**, arm by arm — each line below is the behaviour the
//! increment's own `DECISIONS.md` entry recorded as *driven at HEAD* before its fix, not a
//! re-run of this suite against an older binary: a second `set-field` on a multi-slot item
//! appended a **duplicate** bullet at exit 0 and the pinned read served the stale first
//! value, while a multi-slot ∧ nested block could not be written at all; a `set:` typo
//! loaded clean and exempted its field from `required-field-present` forever,
//! `add-task --workflow nosuch` minted a permanently unreachable sub-task at exit 0, a
//! discarded sub-task stayed `active` in the committed record, and the snapshot's door set
//! was a doc-comment naming three of five sites; a project schema shadow dropped four
//! sections from a frozen doctype and validated clean at exit 0, and a mis-keyed leaf
//! inside a `repeatable:` erased its whole section from every surface; there was **no legal
//! path** from a scalar item field to item prose at the 1→2 arity and the terminal route
//! named a file in this workspace; three nesting ceilings disagreed and only one was
//! written down, and a `placement:` home was the one home no knob could reach; a `packs:`
//! list beside the compose marker was a loud refusal, and every audit this project runs was
//! rejected by `completion-record`'s own severity enum; one nested-section miss answered
//! under four codes across six doors, `doc show` carried the stamp only as a string inside
//! `fields`, the agent-text surface of a located finding named no address, and
//! `is_route_exempt` was a **prefix match** over a namespace.
//!
//! Isolation: every arm builds its own throwaway repo (a real `git init`, a per-repo git
//! identity, `$HOME` repointed, `JIGC_PACK_DIR` scrubbed unless the cell deliberately
//! supplies a fixture pack), or rides the shared [`support::trial_corpus`] substrate.
//!
//! [pinning.md]: ../../../implementation/pinning.md

use crate::support;

use cli::cli::{DOCTYPE_DOORS, DoctypeArg, VERB_KINDS, VerbKind};
use cli::doc::DOC_READ_VERBS;
use engine::address::MAX_FRAGMENT_HOPS;
use engine::manifest::Manifest;
use engine::schema::{MAX_NESTING_DEPTH, SetKind};
use engine::schema_diff::{SchemaChange, schema_diff};
use engine::state::{MINT_DOORS, Snapshot};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use support::shape_space::{FIXTURE_WORKFLOW, SHAPES, Shape, shape_schema_at};
use support::trial_corpus::{FixturePack, State, TrialCorpus};

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow50-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home` and **no inherited
/// `JIGC_PACK_DIR`** — the arms that want a fixture pack pass one explicitly.
fn run(repo: &Path, home: &Path, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command.output().expect("run the jigc binary")
}

/// `run`, with the exit asserted and stdout returned.
fn run_ok(repo: &Path, home: &Path, args: &[&str]) -> String {
    let out = run(repo, home, args);
    assert!(
        out.status.success(),
        "jigc {args:?} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Both of a call's streams, joined — the surface a reader actually meets.
fn both_streams(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

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

/// A real git repo with one commit and a per-repo identity.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
}

/// Parse a `--format json` payload, surfacing the bytes on failure.
fn json(payload: &str) -> Value {
    serde_json::from_str(payload)
        .unwrap_or_else(|err| panic!("the payload is JSON ({err}); got:\n{payload}"))
}

/// The workspace root — the checkout both shipped `schema-manifest.yaml` files live in.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — the item region, over a MANUFACTURED SHAPE SPACE, driven to a commit
// ═════════════════════════════════════════════════════════════════════════════

/// The manufactured doctype's home under the fixture pack — a `location:` doctype, so the
/// arm can carry each shape past the staged write into a real commit.
const SHAPE_HOME: &str = "findings/";

/// The address of the manufactured doc and its one item section.
const SHAPE_DOC: &str = "changelog:findings-log";
const SHAPE_SECTION: &str = "changelog:findings-log#findings";

/// The committed path `SHAPE_HOME` resolves to under the default `docs-root`.
const SHAPE_COMMITTED: &str = "docs/findings/findings-log.md";

/// **Arm 1** — the item-region class is closed over its **shape space**, and the proof
/// runs to a `git` commit.
///
/// **The cell set is a MANUFACTURED SHAPE SPACE, and that is stated rather than implied.**
/// [`SHAPES`] is the `(slots, nested)` generator, not a hand-written list and **not** a
/// registry read: across both embedded packs `roadmap.milestones` is the only multi-slot
/// item block and it carries no settable field, and no shipped block is multi-slot **and**
/// nested — so the shipped registry populates 2 of the 18 `shape × op` cells. Looping the
/// registry here would sweep two shapes and report an axis it never reached, which is the
/// exact failure the complete-fix contract exists to prevent, arriving *through* the
/// mechanism that contract prescribes (`completions/artifacts/M49/settle-record.md` →
/// *Acceptance — one correction to how the axis is built*). The fence that the shipped
/// registry cannot supply this axis lives with the generator's other consumer
/// (`item_region_shape_space::the_registry_cannot_supply_this_axis`).
///
/// What this arm adds over that suite is the **cost**: every shape is driven past the
/// second `set-field` — the write that used to duplicate the bullet — through
/// `jigc task finalize` to a real commit, and the committed bytes, the committed
/// `doc show --format json` read-back and the store sweep are each asserted. The wave's
/// claim is *silent, committed data corruption*; a staged-only proof stops one step short
/// of it.
#[test]
fn the_item_region_holds_over_a_manufactured_shape_space_all_the_way_to_a_commit() {
    for shape in SHAPES {
        let label = shape.label();
        let pack = FixturePack::from_dev_pack(&format!("flow50-{label}"));
        pack.write_schema("changelog", &shape_schema_at(shape, Some(SHAPE_HOME)))
            .write_workflow("log-finding", FIXTURE_WORKFLOW);
        let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
        let task = corpus.start_workflow("log-finding", "record a finding");

        corpus.jigc_ok(&[
            "doc",
            "create",
            "changelog",
            "--title",
            "Findings log",
            "--task",
            &task,
        ]);
        let item = corpus.add_item(SHAPE_SECTION, "First finding", &task);
        for slot in shape.slot_ids() {
            corpus.set_slot(&format!("{item}/{slot}"), &task, &slot_prose(slot));
        }
        if shape.nested {
            let note = corpus.add_item(&format!("{item}/notes"), "First note", &task);
            corpus.set_slot(&format!("{note}/detail"), &task, NOTE_PROSE);
        }

        // The insert, then the update: the second write is the one that used to append a
        // duplicate bullet on every ≥2-slot block, at exit 0.
        corpus.set_field(&format!("{item}/status"), &task, "open");
        corpus.set_field(&format!("{item}/status"), &task, "closed");

        // The staged read-back through the pinned contract — the surface that used to
        // contradict the write it had just acknowledged.
        let staged = json(&corpus.jigc_ok(&[
            "doc", "show", SHAPE_DOC, "--task", &task, "--format", "json",
        ]));
        assert_eq!(
            staged["sections"]["findings"][0]["status"], "closed",
            "[{label}] the staged read-back must serve the value the last write acked; got:\n\
             {staged:#}",
        );

        // …and the whole point of the wave: the corruption's cost was COMMITTED.
        corpus.finalize(&task, "docs", "record a finding", false);

        let committed = support::trial_corpus::read(&corpus.repo(), SHAPE_COMMITTED);
        assert_eq!(
            status_bullets(&committed).len(),
            1,
            "[{label}] two writes leave ONE field bullet in the committed bytes; got:\n\
             {committed}",
        );
        assert!(
            committed.contains("- status: closed"),
            "[{label}] the committed bullet carries the second write's value; got:\n{committed}",
        );
        for fragment in surviving_fragments(shape) {
            assert!(
                committed.contains(&fragment),
                "[{label}] the field write must not eat bytes it does not own — \
                 `{fragment}` is gone from:\n{committed}",
            );
        }
        assert!(
            !corpus.git(&["ls-files", SHAPE_COMMITTED]).trim().is_empty(),
            "[{label}] the promoted doc is tracked — the arm ends at a commit, not a stage",
        );

        let shown = json(&corpus.jigc_ok(&["doc", "show", SHAPE_DOC, "--format", "json"]));
        assert_eq!(
            shown["sections"]["findings"][0]["status"], "closed",
            "[{label}] the COMMITTED read-back agrees with the committed bytes; got:\n{shown:#}",
        );
        corpus.jigc_ok(&["validate"]);
    }
}

/// The prose one declared slot is filled with — distinct per slot, so an assertion says
/// *this* leaf survived rather than that some prose did.
fn slot_prose(slot: &str) -> String {
    format!("The {slot}, in prose.")
}

const NOTE_PROSE: &str = "The note's prose.";

/// The `- status:` bullets in a document's bytes. The count is the corruption's byte-level
/// face: two writes must leave one bullet, never two.
fn status_bullets(bytes: &str) -> Vec<&str> {
    bytes
        .lines()
        .filter(|line| line.trim_start().starts_with("- status:"))
        .collect()
}

/// Every fragment `shape`'s document must still carry after the two field writes — its
/// prose leaves, its multi-slot sub-labels, and its nested item with its prose.
fn surviving_fragments(shape: Shape) -> Vec<String> {
    let mut out = vec![String::from("### First finding  {#first-finding}")];
    for slot in shape.slot_ids() {
        out.push(slot_prose(slot));
        if shape.slots >= 2 {
            // Multi-slot items label each leaf; a single slot renders as bare prose.
            let mut label = slot.to_string();
            label[..1].make_ascii_uppercase();
            out.push(format!("#### {label}"));
        }
    }
    if shape.nested {
        out.push(String::from("#### First note  {#first-note}"));
        out.push(String::from(NOTE_PROSE));
    }
    out
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — every door's report matches the state it left
// ═════════════════════════════════════════════════════════════════════════════

/// Where each [`VerbKind::Read`] leaf is proven inert — driven in this arm, or classified
/// against the suite that drives it. A `Read` leaf with no cell is a **hard panic**, which
/// is the point of reading [`VERB_KINDS`] rather than a curated list.
const READ_VERB_CELLS: &[(&[&str], &str)] = &[
    (&["upgrade"], "read_verb_acts_nothing.rs"),
    (&["describe"], "read_verb_acts_nothing.rs"),
    (&["validate"], "read_verb_acts_nothing.rs"),
    (&["doc", "show"], "read_verb_acts_nothing.rs"),
    (&["doc", "schema"], "read_verb_acts_nothing.rs"),
    (&["doc", "list"], "read_verb_acts_nothing.rs"),
    (&["task", "list"], "read_verb_acts_nothing.rs"),
    (&["task", "diff"], "read_verb_acts_nothing.rs"),
    (&["task", "validate"], "read_verb_acts_nothing.rs"),
    (&["config", "get"], "read_verb_acts_nothing.rs"),
    (&["config", "list"], "read_verb_acts_nothing.rs"),
    (&["milestone", "list-tasks"], "driven here"),
];

/// **Arm 2, cell 1** — the milestone doors report the state they left, and the one `Read`
/// verb that used to act is driven over the state that reveals it.
///
/// Three repairs, one shape. `add-task --workflow` is checked against the **loaded packs**
/// before anything mints, so a typo no longer yields a sub-task nothing can compose;
/// `task discard` of a sub-task writes the committed **record**, which used to keep
/// claiming `status: active` over an area that no longer existed; and
/// `milestone list-tasks` — a [`VerbKind::Read`] leaf — no longer re-seeds the discarded
/// area, which is what a read verb fabricating authority looks like.
///
/// The `Read` set itself is the registry: every member of [`VERB_KINDS`] classified `Read`
/// is disposed in [`READ_VERB_CELLS`], and a member with no cell panics here.
#[test]
fn every_milestone_door_reports_the_state_it_left_and_no_read_verb_acts() {
    // ── The registry: every `Read` leaf is disposed ──────────────────────────
    let cells: BTreeMap<Vec<&str>, &str> = READ_VERB_CELLS
        .iter()
        .map(|(path, cell)| (path.to_vec(), *cell))
        .collect();
    let read_leaves: Vec<Vec<&str>> = VERB_KINDS
        .iter()
        .filter(|(_, kind)| *kind == VerbKind::Read)
        .map(|(path, _)| path.to_vec())
        .collect();
    for leaf in &read_leaves {
        assert!(
            cells.contains_key(leaf),
            "`jigc {}` is a VerbKind::Read leaf with no cell — a read verb joins this arm \
             or names the suite that sweeps it; never a silent skip",
            leaf.join(" "),
        );
    }
    assert_eq!(
        cells.len(),
        read_leaves.len(),
        "the cell table must be a bijection with the registry's `Read` rows — a stale cell \
         is a claim about a verb that no longer reads",
    );

    // ── The driven cells ────────────────────────────────────────────────────
    let corpus = TrialCorpus::build(State::Fresh);
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);

    // (a) `--workflow` membership: refused before anything mints.
    let refused = corpus.jigc(&[
        "milestone",
        "add-task",
        "cache-rework",
        "do the thing",
        "--workflow",
        "nosuch",
    ]);
    assert!(
        !refused.status.success(),
        "an unknown `--workflow` must block; got {:?}",
        refused.status,
    );
    let surface = both_streams(&refused);
    assert!(
        surface.contains("workflow-refs.unknown-workflow"),
        "the refusal carries its own finding identity; got:\n{surface}",
    );
    assert!(
        surface.contains("nothing was minted, recorded or committed"),
        "the refusal states what it left behind — the arm's whole subject; got:\n{surface}",
    );
    assert!(
        !corpus.repo().join(".jigc/tasks/do-the-thing").exists(),
        "…and the state matches the report: no working area was minted",
    );

    // (b) the record tells the truth about a discarded sub-task.
    corpus.jigc_ok(&[
        "milestone",
        "add-task",
        "cache-rework",
        "do the thing",
        "--workflow",
        "dev-task",
    ]);
    let record_path = "docs/milestone-records/cache-rework.md";
    let record = support::trial_corpus::read(&corpus.repo(), record_path);
    assert!(
        record.contains("- status: active"),
        "the added sub-task is recorded active; got:\n{record}",
    );
    corpus.jigc_ok(&["task", "discard", "do-the-thing"]);
    let record = support::trial_corpus::read(&corpus.repo(), record_path);
    assert!(
        record.contains("- status: discarded") && !record.contains("- status: active\n"),
        "a discarded sub-task stops being `active` in the committed record; got:\n{record}",
    );
    assert!(
        !corpus.repo().join(".jigc/tasks/do-the-thing").exists(),
        "the discard removed the working area it reported removing",
    );

    // (c) the `Read` verb that used to act: it reports the record and rebuilds nothing.
    let listed = corpus.jigc_ok(&["milestone", "list-tasks", "cache-rework"]);
    assert!(
        !listed.contains("do-the-thing"),
        "the read serves the record's live set, which no longer holds the discarded \
         sub-task; got:\n{listed}",
    );
    assert!(
        !corpus.repo().join(".jigc/tasks/do-the-thing").exists(),
        "a VerbKind::Read leaf may not resurrect a working area a later door reads as \
         authority — this is the cell the registry above points at",
    );
}

/// Where each [`MINT_DOORS`] member's snapshot disposition is proven.
const MINT_DOOR_CELLS: &[(&str, &str)] = &[
    ("crates/cli/src/start.rs::mint_in_repo", "driven here"),
    (
        "crates/cli/src/start.rs::mint_migration_in_repo",
        "mint_doors.rs",
    ),
    ("crates/cli/src/milestone.rs::run_create", "mint_doors.rs"),
    ("crates/engine/src/milestone.rs::add_task", "mint_doors.rs"),
    (
        "crates/engine/src/milestone.rs::reseed_sub_task_areas",
        "mint_doors.rs",
    ),
];

/// **Arm 2, cell 2** — the two closed vocabularies refuse at the door that loads them.
///
/// [`SetKind::ALL`] is the `set:` deriver vocabulary. A typo used to load clean, be
/// serialized by the pinned `doc schema` as a real deriver, and exempt its field from
/// `required-field-present` forever; it is now refused at **schema load**, and the refusal
/// names every honored member — asserted by iterating the enum, so a fourth kind joins the
/// message and this assertion together.
///
/// [`MINT_DOORS`] is the registry Increment 12 / T1 minted because the snapshot
/// disposition was a *remembered list*: `write_staged_snapshot`'s doc-comment said *"every
/// task-minting door"* and named three of the five production sites. The `Written` member
/// an operator reaches by `jigc start` is driven here; every other member is classified
/// against `mint_doors.rs`, which carries the source-level completeness fence and one
/// driven cell per member. A member with no cell is a hard panic.
#[test]
fn the_closed_vocabularies_refuse_at_the_door_that_loads_them() {
    // ── `set:` — the vocabulary, refused at schema load ──────────────────────
    let pack = FixturePack::from_dev_pack("flow50-setkind");
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
    let adr = fs::read_to_string(pack.path().join("schemas").join("adr.yaml"))
        .expect("read the fixture pack's adr schema");
    assert!(
        adr.contains("set: on-create"),
        "the fixture rests on the shipped `adr` carrying a `set:` leaf; got:\n{adr}",
    );
    pack.write_schema("adr", &adr.replace("set: on-create", "set: on-creat"));

    let refused = corpus.jigc(&["describe"]);
    assert!(
        !refused.status.success(),
        "a `set:` typo must refuse the pack load; got {:?}",
        refused.status,
    );
    let surface = both_streams(&refused);
    assert!(
        surface.contains("undeclared `set:` deriver `on-creat`"),
        "the refusal names the undeclared deriver; got:\n{surface}",
    );
    for kind in SetKind::ALL {
        assert!(
            surface.contains(kind.as_str()),
            "the refusal names every honored `set:` kind — `{}` is missing from:\n{surface}",
            kind.as_str(),
        );
    }

    // ── `MINT_DOORS` — every production mint disposed ────────────────────────
    let cells: BTreeMap<&str, &str> = MINT_DOOR_CELLS.iter().copied().collect();
    for door in MINT_DOORS {
        assert!(
            cells.contains_key(door.site),
            "the mint door `{}` ({}) has no cell — a production mint joins this arm or \
             names the suite that drives it",
            door.door,
            door.site,
        );
    }
    assert_eq!(
        cells.len(),
        MINT_DOORS.len(),
        "the cell table must be a bijection with the registry — a stale cell is a claim \
         about a mint that no longer exists",
    );
    let start_door = MINT_DOORS
        .iter()
        .find(|door| door.site == "crates/cli/src/start.rs::mint_in_repo")
        .expect("the front door is a declared mint");
    assert!(
        matches!(start_door.snapshot, Snapshot::Written),
        "the front door declares that it writes the carryover snapshot",
    );

    // Driven: the declared disposition, on disk, through the real front door.
    let fresh = TrialCorpus::build(State::Fresh);
    let task = fresh.start_workflow("dev-task", "probe the front door");
    let snapshot = fresh
        .repo()
        .join(format!(".jigc/tasks/{task}/staged-snapshot.json"));
    assert!(
        snapshot.exists(),
        "`{}` declares `Snapshot::Written`, so the area it mints carries one",
        start_door.door,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — the freeze binds at every layer; the migration surface can say it
// ═════════════════════════════════════════════════════════════════════════════

/// **Arm 3, cell 1** — the freeze binds at **every layer that can change a schema**, and a
/// mis-keyed leaf is refused rather than absorbed.
///
/// **The cell set is a derivation, and it is stated as one.** There is no code-side table
/// of "layers a schema can arrive from" — there are two, and they are the two the resolver
/// reads: the **pack file** a doctype ships as, and the project's own
/// `.jigc/config/schemas/<t>.yaml` **shadow**. Each is driven:
///
///   * the shadow of a manifest-governed doctype is refused **by name**, with a runnable
///     route — where it used to drop four sections from a frozen doctype and validate
///     clean at exit 0, while `doc schema` reported the frozen version for an unfrozen
///     shape and `doc create` wrote a third;
///   * a mis-keyed leaf inside a `repeatable:` block is refused at load, naming the
///     section and the key — where it used to erase the whole section from every surface.
#[test]
fn the_freeze_binds_at_every_layer_a_schema_can_change() {
    // ── The project layer ───────────────────────────────────────────────────
    let corpus = TrialCorpus::build(State::Fresh);
    let shadow = corpus.repo().join(".jigc/config/schemas");
    fs::create_dir_all(&shadow).expect("create the project schema shadow dir");
    fs::write(
        shadow.join("adr.yaml"),
        "type: adr\nid-from: title\nlocation: decisions/\n\
         description: A shrunken adr.\nusage: the shadow drops four sections.\n\
         sections:\n  - id: context\n    slot: { hint: \"The context.\" }\n",
    )
    .expect("write the project schema shadow");

    let refused = corpus.jigc(&["describe"]);
    assert!(
        !refused.status.success(),
        "a project shadow that reshapes a frozen doctype must block; got {:?}",
        refused.status,
    );
    let surface = both_streams(&refused);
    assert!(
        surface.contains("schema-hash mismatch") && surface.contains("adr"),
        "the refusal names the doctype whose hash moved; got:\n{surface}",
    );
    assert!(
        surface.contains(".jigc/config/schemas/adr.yaml"),
        "…and names the shadow file that moved it, which is what makes the route runnable; \
         got:\n{surface}",
    );
    assert!(
        surface.contains("freeze forbids at every layer"),
        "the message states the rule the layer axis exists for; got:\n{surface}",
    );
    fs::remove_file(shadow.join("adr.yaml")).expect("drop the shadow");
    corpus.jigc_ok(&["describe"]);

    // ── The pack layer: a mis-keyed leaf inside a `repeatable:` ─────────────
    let pack = FixturePack::from_dev_pack("flow50-miskey");
    let mis_keyed = TrialCorpus::build_with_pack(State::Fresh, &pack);
    let commit = fs::read_to_string(pack.path().join("schemas").join("commit.yaml"))
        .expect("read the fixture pack's commit schema");
    let sound = "- { id: key, type: string }";
    assert!(
        commit.contains(sound),
        "the fixture rests on the shipped `commit.trailers` block; got:\n{commit}",
    );
    pack.write_schema(
        "commit",
        &commit.replace(sound, "- { id: key, type: string, patern: \"^[A-Z]\" }"),
    );

    let refused = mis_keyed.jigc(&["doc", "schema", "commit"]);
    assert!(
        !refused.status.success(),
        "a mis-keyed leaf must refuse the load rather than erase its section; got {:?}",
        refused.status,
    );
    let surface = both_streams(&refused);
    assert!(
        surface.contains("patern") && surface.contains("trailers"),
        "the refusal names the offending key and the section it sits in; got:\n{surface}",
    );
}

/// Where each [`SchemaChange`] kind is disposed. The match is **exhaustive**, so a new
/// kind cannot compile without an author deciding which side of this arm it is on.
///
/// Two dispositions, and the split is what this arm can honestly claim. The kinds the
/// shipped `completion-record` v1→v2 pair actually produces are **driven here**, end to
/// end through `jigc migrate-corpus`. Every other kind is disposed against the engine's
/// own per-kind fold axis in `crates/engine/src/transform.rs` — whose existence this arm
/// asserts rather than assumes — and the CLI suites that consume it; re-driving them here
/// would be a second copy of an axis that already has one home.
fn change_cell(change: &SchemaChange) -> &'static str {
    match change {
        SchemaChange::AddedItemSlot { .. } | SchemaChange::EnumWidened { .. } => DRIVEN_BY_THIS_ARM,
        SchemaChange::AddedOptionalField { .. }
        | SchemaChange::AddedOptionalSection { .. }
        | SchemaChange::AddedRepeatableSection { .. }
        | SchemaChange::AddedItemField { .. }
        | SchemaChange::WidenedCardinality { .. }
        | SchemaChange::NarrowedCardinality { .. }
        | SchemaChange::ValueRemapped { .. }
        | SchemaChange::FixedSlotToRepeatable { .. }
        | SchemaChange::ProseNeeding { .. }
        | SchemaChange::Relocated { .. }
        | SchemaChange::DisplayTitleChanged { .. }
        | SchemaChange::OptionalRelaxed { .. }
        | SchemaChange::RemovedField { .. }
        | SchemaChange::RemovedItemSlot { .. }
        | SchemaChange::PresentationOnly
        | SchemaChange::Unclassified => TRANSFORM_KIND_AXIS,
    }
}

/// The disposition of a kind this arm drives through the shipped door.
const DRIVEN_BY_THIS_ARM: &str = "driven here: the completion-record 1→2 fold";

/// The disposition of a kind this arm does not produce — proven at the engine's per-kind
/// fold axis, whose section markers are asserted below.
const TRANSFORM_KIND_AXIS: &str = "engine::transform's per-kind fold axis";

/// **Arm 3, cell 2** — the migration surface can express the change the product's own
/// doctypes needed, and a v1-stamped record folds to current through the shipped door.
///
/// **The cell set is [`SchemaChange`], matched exhaustively** — and the values matched are
/// **real**: the diff of the shipped `completion-record.v1` snapshot against the shipped
/// v2 schema, both loaded through the production loader, so this arm cannot go green on a
/// hand-built change list. That pair classifies as exactly
/// `[EnumWidened, AddedItemSlot]` — the kind Increment 4 minted, at the arity the wave's
/// own bump landed in — and both are byte no-ops, so the migration moves **only** the
/// stamp. Before Increment 4 an added item slot diffed to the empty-diff residual and the
/// doc was refused at `migrate-corpus.unclassified-change`, routed at a file in this
/// workspace that no adopter can edit.
#[test]
fn the_migration_surface_expresses_the_change_the_doctypes_needed() {
    // ── The kinds: the real diff, matched exhaustively ──────────────────────
    let pack_tree = support::frozen_pack::methodology_pack_tree();
    let source = cli::pack::FilesystemPack::new(pack_tree.clone());
    let load = |path: PathBuf| {
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        cli::pack::load_pack_schema(&source, &bytes)
            .unwrap_or_else(|e| panic!("{} loads: {e}", path.display()))
    };
    let prior = load(
        pack_tree
            .join("schema-snapshots")
            .join("completion-record.v1.yaml"),
    );
    let current = load(pack_tree.join("schemas").join("completion-record.yaml"));
    let changes = schema_diff(&prior, &current);
    assert!(
        !changes.is_empty(),
        "the shipped v1→v2 pair must produce a diff — an empty one would make the match \
         below vacuous",
    );
    let cells: BTreeSet<&str> = changes.iter().map(change_cell).collect();
    assert_eq!(
        cells,
        BTreeSet::from([DRIVEN_BY_THIS_ARM]),
        "every change this pair produces is one this arm drives; got {changes:?}",
    );
    // The disposition every other kind carries names a real axis, asserted rather than
    // assumed — an unclassified member is a hard panic, never a silent skip.
    let transform = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../engine/src/transform.rs"),
    )
    .expect("read the engine transform module");
    for marker in ["the value-remapped kind", "the added-item-field kind"] {
        assert!(
            transform.contains(marker),
            "`{TRANSFORM_KIND_AXIS}` must exist — `{marker}` is missing from transform.rs",
        );
    }
    assert!(
        changes
            .iter()
            .any(|change| matches!(change, SchemaChange::AddedItemSlot { .. })),
        "the pair carries the kind Increment 4 minted; got {changes:?}",
    );

    // ── The door: a committed v1 record folds to current, stamp-only ────────
    let repo = TempDir::new("added-item-slot");
    let home = TempDir::new("added-item-slot-home");
    init_repo(repo.path());
    let config = repo.path().join(".jigc").join("config");
    fs::create_dir_all(&config).expect("create the project cascade layer the store walk expects");
    // The compose marker `jigc setup` writes: `completion-record` is a METHODOLOGY
    // doctype, so without it the pack that governs the record is not composed at all.
    fs::write(
        config.join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let record = repo.path().join("docs").join("completions").join("M99.md");
    fs::create_dir_all(record.parent().expect("a parent")).expect("mk completions/");
    fs::write(&record, RECORD_V1).expect("write the v1 record");
    let artifact = repo
        .path()
        .join("completions")
        .join("artifacts")
        .join("M99")
        .join("VERDICT.md");
    fs::create_dir_all(artifact.parent().expect("a parent")).expect("mk the artifact home");
    fs::write(&artifact, "the verdict\n").expect("write the owner-artifact");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed a v1 record"]);

    let report = json(&run_ok(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    ));
    let migrated: Vec<&str> = report["migrated"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `migrated[]`; got:\n{report:#}"))
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert_eq!(
        migrated,
        ["docs/completions/M99.md"],
        "the v1 record migrates through the shipped door; report:\n{report:#}",
    );
    assert_eq!(
        fs::read_to_string(&record).expect("read the migrated record"),
        RECORD_V1.replace("schema-version: 1", "schema-version: 2"),
        "the stamp is the fold's only byte delta — a widened enum admits every committed \
         value unchanged, and a lone added item slot renders bare",
    );
    run_ok(repo.path(), home.path(), &["validate"]);
}

/// A conformant, **v1-stamped** committed `completion-record` — the exact shape every
/// record committed before the wave's own bump is in.
const RECORD_V1: &str = "\
---
verdict: green
owner-artifact: completions/artifacts/M99/VERDICT.md
schema-version: 1
---

# M99

## Findings

### A stray finding  {#a-stray-finding}

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: audit.log:12
";

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — the adopter-facing limits, and the home that stops dictating layout
// ═════════════════════════════════════════════════════════════════════════════

/// **Arm 4, cell 1** — the nesting ceiling is **one derived number**, and an item's id can
/// be decoupled from its title.
///
/// The ceiling assertion follows [`MAX_NESTING_DEPTH`] rather than today's literal, and
/// the constant is itself derived from [`MAX_FRAGMENT_HOPS`] — which is the repair: three
/// ceilings disagreed (the loader admitted 4, `add-item` reached 3, leaf writes reached 2)
/// and only "4" was written down, because `MAX_FRAGMENT_HOPS`' own comment had the
/// arithmetic wrong. A schema one level past the cap is refused **naming both numbers**,
/// and a schema **at** the cap loads and its deepest item is addressable by the write path
/// — the half that keeps the fix from being a mere tightening.
#[test]
fn the_nesting_ceiling_is_one_derived_number_and_an_item_id_can_leave_its_title() {
    // ── At the cap: legal, and reachable by the write path ──────────────────
    let pack = FixturePack::from_dev_pack("flow50-depth");
    pack.write_schema("changelog", &nested_schema(MAX_NESTING_DEPTH))
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
    let task = corpus.start_workflow("log-finding", "reach the deepest level");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "changelog",
        "--title",
        "Findings log",
        "--task",
        &task,
    ]);
    let mut address = String::from(SHAPE_SECTION);
    for level in 0..MAX_NESTING_DEPTH {
        address = corpus.add_item(&address, &format!("Level {level}"), &task);
        if level + 1 < MAX_NESTING_DEPTH {
            address = format!("{address}/nested");
        }
    }
    corpus.set_slot(&format!("{address}/detail"), &task, "The deepest prose.");
    let shown = corpus.jigc_ok(&["doc", "show", &address, "--task", &task]);
    assert!(
        shown.contains("The deepest prose."),
        "an item at the declared cap is addressable by the write path AND readable back; \
         got:\n{shown}",
    );

    // ── One level past it: refused, naming the derived numbers ──────────────
    pack.write_schema("changelog", &nested_schema(MAX_NESTING_DEPTH + 1));
    let refused = corpus.jigc(&["describe"]);
    assert!(
        !refused.status.success(),
        "a schema one level past the cap must refuse the load; got {:?}",
        refused.status,
    );
    let surface = both_streams(&refused);
    assert!(
        surface.contains(&format!("depth {}", MAX_NESTING_DEPTH + 1))
            && surface.contains(&format!("cap of {MAX_NESTING_DEPTH} levels")),
        "the refusal states the depth it read and the cap it enforces, both derived; \
         got:\n{surface}",
    );
    assert!(
        surface.contains(&format!("admits {MAX_FRAGMENT_HOPS}")),
        "…and names the address budget the cap is DERIVED from, which is what stopped the \
         three ceilings from drifting apart again; got:\n{surface}",
    );

    // ── `--slug`: an item's id decoupled from its title ─────────────────────
    let slugged = TrialCorpus::build(State::Fresh);
    let task = slugged.start_workflow("architecture-documentation", "document the gateway");
    slugged.jigc_ok(&[
        "doc", "create", "arch-doc", "--title", "Gateway", "--task", &task,
    ]);
    let emitted = slugged.jigc_ok(&[
        "doc",
        "add-item",
        "arch-doc:gateway#components",
        "--title",
        "Retry policy (v2)",
        "--slug",
        "retry-policy-two",
        "--task",
        &task,
    ]);
    let address = emitted.trim_end_matches('\n');
    assert_eq!(
        address, "arch-doc:gateway#components/retry-policy-two",
        "`--slug` decides the item's id; the emitted address is what every later write \
         takes",
    );
    let shown = json(&slugged.jigc_ok(&[
        "doc",
        "show",
        "arch-doc:gateway",
        "--task",
        &task,
        "--format",
        "json",
    ]));
    assert_eq!(
        shown["sections"]["components"][0]["id"], "retry-policy-two",
        "…and the pinned read serves that id, with the authored title intact; got:\n{shown:#}",
    );
    assert_eq!(
        shown["sections"]["components"][0]["title"], "Retry policy (v2)",
        "the title is the author's — decoupling the id never rewrites it; got:\n{shown:#}",
    );
}

/// A manufactured doctype nesting `depth` levels of repeatable, generated from the depth —
/// so the fixture follows [`MAX_NESTING_DEPTH`] instead of pinning a literal shape.
fn nested_schema(depth: usize) -> String {
    let mut yaml = format!(
        "type: changelog
location: {SHAPE_HOME}
id-from: title
description: A manufactured findings log nesting {depth} levels.
usage: the nesting ceiling needs schemas at and past the derived cap.
sections:
  - id: findings
    repeatable:
      id-from: label
      block:
        - {{ id: label, type: string }}
"
    );
    // Each further level indents by six spaces: `- id: nested` under the parent block,
    // then its own `repeatable:` / `id-from:` / `block:`.
    let mut indent = 8;
    for _ in 1..depth {
        let pad = " ".repeat(indent);
        yaml.push_str(&format!(
            "{pad}- id: nested\n{pad}  repeatable:\n{pad}    id-from: label\n\
             {pad}    block:\n{pad}      - {{ id: label, type: string }}\n"
        ));
        indent += 6;
    }
    let pad = " ".repeat(indent);
    yaml.push_str(&format!(
        "{pad}- {{ id: detail, slot: {{ hint: \"The prose.\" }} }}\n"
    ));
    yaml
}

/// **Arm 4, cell 2** — a `placement:` home stops dictating an adopter's repo layout, and
/// the re-point **moves** the committed doc it would otherwise strand.
///
/// The asymmetry this closes: a `location:` home resolves through `docs-root`, a
/// `placement:` home resolved through nothing, and `jigc relocate` refuses frozen
/// doctypes — so an adopter whose docs are not under `docs/` had **no path at all**. The
/// scope is the rule rather than a list: a home declared with a leading directory
/// component re-roots under `placement-root`; a home declared **at** the repo root
/// (`VISION.md`) is the ecosystem-idiomatic case and is never re-rooted.
#[test]
fn a_placement_home_takes_a_project_override_and_the_committed_doc_moves_with_it() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    assert!(
        corpus.repo().join("docs/roadmap.md").exists() && corpus.repo().join("VISION.md").exists(),
        "the fixture rests on both placement shapes: one nested home, one at the repo root",
    );

    corpus.jigc_ok(&["config", "set", "placement-root", "notes"]);

    assert!(
        corpus.repo().join("notes/roadmap.md").exists(),
        "the re-point MOVES the committed instance to the new resolved home",
    );
    assert!(
        !corpus.repo().join("docs/roadmap.md").exists(),
        "the prior home is emptied — a copy left behind is a second source of truth",
    );
    assert!(
        corpus.repo().join("VISION.md").exists() && !corpus.repo().join("notes/VISION.md").exists(),
        "a home declared AT the repo root is not re-rooted, so it is never moved",
    );
    let renames: Vec<String> = corpus
        .git(&["status", "--porcelain"])
        .lines()
        .filter(|line| line.starts_with('R'))
        .map(str::to_owned)
        .collect();
    assert!(
        renames.iter().all(|line| line.contains(" -> notes/")),
        "every move the re-point makes lands as a staged `git mv` into the new root, and \
         nothing else moves; status:\n{}",
        corpus.git(&["status", "--porcelain"]),
    );
    assert!(
        renames
            .iter()
            .any(|line| line.contains("docs/roadmap.md -> notes/roadmap.md")),
        "…including the committed instance whose home the re-point changed; status:\n{}",
        corpus.git(&["status", "--porcelain"]),
    );
    assert!(
        !renames.iter().any(|line| line.contains("VISION.md")),
        "…and never the root-declared home, which no `placement-root` value could reach",
    );

    let shown = corpus.jigc_ok(&["doc", "show", "roadmap:roadmap"]);
    assert!(
        !shown.trim().is_empty(),
        "the doc still reads back at its address after the re-point",
    );
    let report = corpus.jigc_ok(&["validate"]);
    assert!(
        !report.contains("file-state."),
        "the move re-keys the file-state record, so the store carries no file-state \
         finding at the new home; got:\n{report}",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — a pack extends without losing the methodology; the bumps landed
// ═════════════════════════════════════════════════════════════════════════════

/// **Arm 5, cell 1** — a project pack composes **with** the embedded pair, and is demoted
/// for any doctype the freeze governs.
///
/// The combination used to be a loud refusal, and `jigc setup` writes the compose marker
/// into every project — so declaring one project pack bricked every door. The composition
/// is now `[listed… ▸ dev ▸ methodology]` with one demotion: **a project pack may not
/// shadow a doctype the freeze governs**, without which a manifest-less listed pack would
/// shadow a frozen dev doctype while the freeze assert is skip-on-absent for it.
#[test]
fn a_project_pack_extends_the_composition_without_shadowing_what_the_freeze_governs() {
    let repo = TempDir::new("project-pack");
    let home = TempDir::new("project-pack-home");
    init_repo(repo.path());
    run_ok(repo.path(), home.path(), &["setup"]);

    let house = repo.path().join("housepack").join("schemas");
    fs::create_dir_all(&house).expect("mk the house pack");
    fs::write(
        house.join("note.yaml"),
        "type: note\nlocation: notes/\nid-from: title\n\
         description: A house note.\nusage: you want a short house-local note.\n\
         sections:\n  - id: meta\n    header: true\n    fields:\n      \
         - { id: topic, type: string }\n  - id: body\n    slot: { hint: \"The note.\" }\n",
    )
    .expect("write the house `note` doctype");
    fs::write(
        house.join("commit.yaml"),
        "type: commit\ndescription: A house commit shape.\n\
         usage: the house wants its own commit fields.\n\
         sections:\n  - id: header\n    header: true\n    fields:\n      \
         - { id: type, type: enum, of: [feat, fix, chore] }\n  \
         - id: summary\n    slot: { hint: \"The subject line.\" }\n",
    )
    .expect("write the house `commit` shadow");

    let packs_yaml = repo.path().join(".jigc").join("config").join("packs.yaml");
    let marker = fs::read_to_string(&packs_yaml).expect("read packs.yaml after setup");
    assert!(
        marker.contains("compose-embedded-methodology: true"),
        "`jigc setup` writes the compose marker into every project; got:\n{marker}",
    );
    fs::write(
        &packs_yaml,
        format!(
            "compose-embedded-methodology: true\npacks:\n  - {}\n",
            repo.path().join("housepack").display()
        ),
    )
    .expect("write packs.yaml carrying the marker AND the list");

    let note = run_ok(repo.path(), home.path(), &["doc", "schema", "note"]);
    assert!(
        note.contains("doctype: note") && note.contains("topic: string"),
        "the house doctype resolves — the extension the refusal used to deny; got:\n{note}",
    );
    let commit = run_ok(repo.path(), home.path(), &["doc", "schema", "commit"]);
    assert!(
        commit.contains("implements: ref"),
        "`commit` resolves to DEV's frozen shape: a listed pack may not shadow a doctype \
         the freeze governs; got:\n{commit}",
    );
    let idea = run_ok(repo.path(), home.path(), &["doc", "schema", "idea"]);
    assert!(
        idea.contains("doctype: idea"),
        "…and the methodology pack is still there, which is the point of the combination; \
         got:\n{idea}",
    );
    run_ok(repo.path(), home.path(), &["validate"]);
}

/// **Arm 5, cell 2** — every schema that had to bump did, and the surface agrees with the
/// manifest that governs it.
///
/// **The cell set is a derivation**: both shipped `config/schema-manifest.yaml`
/// `doctypes:` lists, read from the checkout, `doc schema <t> --format json` driven per
/// entry. The wave's three bumps (`completion-record` 1→2, `milestone-record` 2→3, and
/// `planning-record` shipping at 1) are covered **by construction** rather than by being
/// named — and named too, because a bump that silently reverted would otherwise leave this
/// arm green.
#[test]
fn every_doctype_reports_the_version_its_manifest_declares() {
    let declared: BTreeMap<String, u32> = ["crates/cli/pack", "packs/methodology"]
        .into_iter()
        .flat_map(|pack| {
            let path = repo_root().join(pack).join("config/schema-manifest.yaml");
            let bytes = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            let manifest: Manifest = serde_yaml_ng::from_str(&bytes)
                .unwrap_or_else(|e| panic!("{} deserializes: {e}", path.display()));
            manifest
                .doctypes
                .into_iter()
                .map(|e| (e.ty, e.schema_version))
        })
        .collect();
    assert!(
        declared.len() > 10,
        "both manifests must parse to a real doctype set; got {declared:?}",
    );

    let corpus = TrialCorpus::build(State::Fresh);
    for (doctype, version) in &declared {
        let projected = json(&corpus.jigc_ok(&["doc", "schema", doctype, "--format", "json"]));
        assert_eq!(
            projected["schema-version"],
            Value::from(*version),
            "`{doctype}` must project the version its manifest declares; got:\n{projected:#}",
        );
    }

    for (doctype, version) in [
        ("completion-record", 2),
        ("milestone-record", 3),
        ("planning-record", 1),
    ] {
        assert_eq!(
            declared.get(doctype).copied(),
            Some(version),
            "the wave's own bump for `{doctype}` is declared at {version}",
        );
    }
}

/// **Arm 5, cell 3** — the planning phase every session ran by hand becomes a **blocking**
/// artifact.
///
/// **The gate set is read from the shipped schema through the binary** — this file
/// hand-lists no gate id — and the arm drives the forcing function: a record missing one
/// gate blocks `jigc task finalize` with `schema-conformance.required-slot-present` naming
/// that gate and committing nothing; filling it lands the identical finalize. The whole
/// axis (every gate in turn) is `planning_gate_forcing.rs`'s; this cell proves the door.
#[test]
fn the_planning_gates_block_the_finalize_until_they_are_filled() {
    let corpus = TrialCorpus::build(State::Fresh);

    let schema = json(&corpus.jigc_ok(&["doc", "schema", "planning-record", "--format", "json"]));
    let gates: Vec<String> = schema["sections"]
        .as_array()
        .unwrap_or_else(|| panic!("the projection carries `sections[]`; got:\n{schema:#}"))
        .iter()
        .filter(|section| section["kind"] == "slot" && section["optional"] != Value::Bool(true))
        .filter_map(|section| section["id"].as_str().map(str::to_owned))
        .collect();
    assert_eq!(
        gates.len(),
        14,
        "the doctype carries one required slot per planning gate; got {gates:?}",
    );

    let task = corpus.start_workflow("planning", "M99 the probe wave");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "planning-record",
        "--title",
        "M99",
        "--task",
        &task,
    ]);
    let (held_out, filled) = gates.split_last().expect("at least one gate");
    for gate in filled {
        corpus.set_slot(
            &format!("planning-record:m99#{gate}"),
            &task,
            "Answered for the probe wave.",
        );
    }

    let before = corpus.git(&["rev-parse", "HEAD"]);
    let blocked = corpus.jigc(&["task", "finalize", &task]);
    assert!(
        !blocked.status.success(),
        "an unfilled gate must block the finalize; got {:?}",
        blocked.status,
    );
    let surface = both_streams(&blocked);
    assert!(
        surface.contains("schema-conformance.required-slot-present") && surface.contains(held_out),
        "the block names the gate that is not answered; got:\n{surface}",
    );
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        before,
        "…and nothing was committed",
    );

    corpus.set_slot(
        &format!("planning-record:m99#{held_out}"),
        &task,
        "Answered for the probe wave.",
    );
    corpus.finalize(&task, "docs", "record the M99 planning gates", false);
    assert!(
        corpus.repo().join("docs/planning-records/m99.md").exists(),
        "the identical finalize lands the record once the gate is filled — the block is a \
         gate, never a dead end",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 6 — the 1.0 contract, and nothing the reader is told is false
// ═════════════════════════════════════════════════════════════════════════════

/// The **item-addressing write doors**, derived and stated as a derivation: the `doc` rows
/// of [`DOCTYPE_DOORS`] that take an address, minus the read verbs ([`DOC_READ_VERBS`]),
/// minus `doc rename` — whose address names a **document identity**, not a locus inside a
/// document, so a nested-section hop is not a miss it can have.
fn item_addressing_doors() -> Vec<&'static str> {
    DOCTYPE_DOORS
        .iter()
        .filter(|(path, arg)| {
            path.first() == Some(&"doc")
                && matches!(arg, DoctypeArg::Address)
                && path
                    .get(1)
                    .is_some_and(|verb| !DOC_READ_VERBS.contains(verb) && *verb != "rename")
        })
        .filter_map(|(path, _)| path.get(1).copied())
        .collect()
}

/// **Arm 6, cell 1** — the pinned contracts, in the shape a driver consumes.
///
/// Four spends, each on a surface a driver keys on. **N1**: one nested-section-hop miss
/// answered under four codes across six doors, and drivers key on `(code, target)`, so one
/// defect under four keys is a contract fault — every item-addressing door now converges
/// on the shipped `write.unknown-section` with the address verbatim in `key.target`.
/// **N2**: `doc show` carries the doc's **own** stamp as a top-level integer, so the
/// upgrade check a driver automates needs no cast, while `fields` stays uniformly stringy.
/// **D1**: the agent-text surface of a located finding carries its address and line — which
/// is what makes the route exemption's stated rationale (*"the located message **is** the
/// repair"*) true rather than false. **D5**: that exemption is an **enumeration**, not a
/// `conformance.` prefix match.
#[test]
fn the_pinned_contracts_answer_in_the_shape_a_driver_consumes() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-change", "note the probe");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "changelog",
        "--title",
        support::create_title("changelog", "Changelog").as_str(),
        "--task",
        &task,
    ]);
    let release = corpus.add_item("changelog:changelog#releases", "1.0.0", &task);

    // ── N1: one nested-section miss, one code, at every item-addressing door ─
    let doors = item_addressing_doors();
    assert!(
        doors.len() >= 5,
        "the derivation must yield the write doors that take an address; got {doors:?}",
    );
    let section_hop = format!("{release}/bogus");
    let leaf_hop = format!("{section_hop}/xyz");
    let mut cells: Vec<(&str, Vec<String>)> = Vec::new();
    for door in &doors {
        let argv: Vec<String> = match *door {
            // `add-item`'s destination IS the undeclared segment; every other verb
            // addresses a leaf beneath it.
            "add-item" => vec![section_hop.clone(), "--title".into(), "X".into()],
            "remove-item" => vec![leaf_hop.clone()],
            "retitle-item" => vec![leaf_hop.clone(), "--title".into(), "Y".into()],
            "set-field" => vec![leaf_hop.clone(), "--value".into(), "v".into()],
            "set-slot" => vec![leaf_hop.clone(), "--from-file".into(), "-".into()],
            other => panic!(
                "`jigc doc {other}` is an item-addressing door with no cell — the \
                 derivation moved and this arm must move with it"
            ),
        };
        cells.push((door, argv));
    }
    // The `--unset` shape of `set-field` answered a fifth code of its own.
    cells.push(("set-field", vec![leaf_hop.clone(), "--unset".into()]));

    for (door, argv) in &cells {
        let mut args = vec!["doc".to_string(), (*door).to_string()];
        args.extend(argv.iter().cloned());
        args.extend([
            "--task".into(),
            task.clone(),
            "--format".into(),
            "json".into(),
        ]);
        let borrowed: Vec<&str> = args.iter().map(String::as_str).collect();
        let out = if argv.iter().any(|a| a == "-") {
            corpus.jigc_stdin(&borrowed, "prose\n")
        } else {
            corpus.jigc(&borrowed)
        };
        assert!(
            !out.status.success(),
            "`jigc doc {door}` over a nested-section hop must block; got {:?}",
            out.status,
        );
        // A blocking write serves its envelope on stderr — the stream discipline the
        // contract pins — so the arm reads the surface rather than assuming a stream.
        let envelope = json(both_streams(&out).trim());
        let finding = &envelope["findings"][0];
        assert_eq!(
            finding["key"]["code"], "write.unknown-section",
            "one nested-section miss, ONE code, at `jigc doc {door}`; got:\n{envelope:#}",
        );
        assert_eq!(
            finding["key"]["target"],
            Value::from(argv[0].as_str()),
            "…keyed on the address the caller typed, verbatim; got:\n{envelope:#}",
        );
    }

    // ── N2: the doc's own stamp, as a number ────────────────────────────────
    let shown = json(&corpus.jigc_ok(&[
        "doc",
        "show",
        "changelog:changelog",
        "--task",
        &task,
        "--format",
        "json",
    ]));
    let stamp = shown["schema-version"]
        .as_u64()
        .unwrap_or_else(|| panic!("`doc show` carries a top-level integer stamp; got:\n{shown:#}"));
    assert_eq!(
        shown["fields"]["schema-version"],
        Value::from(stamp.to_string()),
        "the two keys cannot disagree: the number is read off the `fields` map the \
         projection already built; got:\n{shown:#}",
    );
    let projected = json(&corpus.jigc_ok(&["doc", "schema", "changelog", "--format", "json"]));
    assert_eq!(
        projected["schema-version"],
        Value::from(stamp),
        "…so the upgrade check a driver automates — actual against expected — needs no \
         cast on either side",
    );

    // ── D1 + D5: a located finding says WHERE, and the exemption is a set ────
    let staged = corpus
        .repo()
        .join(format!(".jigc/tasks/{task}/docs/changelog:changelog.md"));
    let bytes = fs::read_to_string(&staged).expect("read the staged changelog");
    fs::write(
        &staged,
        bytes.replace("- date:", "- bogusfield: x\n- date:"),
    )
    .expect("hand-break one field bullet");

    let text = both_streams(&corpus.jigc(&["task", "validate", &task]));
    assert!(
        text.contains("conformance.unknown-field"),
        "the parse diagnostic reaches the agent-text surface; got:\n{text}",
    );
    assert!(
        text.contains("at: changelog:changelog#releases/1-0-0/bogusfield"),
        "…carrying the address it located, which is what makes the route exemption's own \
         rationale true; got:\n{text}",
    );
    assert!(
        text.contains("· line "),
        "…and the line it read it at; got:\n{text}",
    );

    // The gate blocks (exit 3) over the hand-broken bytes, so the envelope is read off a
    // refusing call rather than an `_ok` one.
    let validated = corpus.jigc(&["task", "validate", &task, "--format", "json"]);
    let envelope = json(&String::from_utf8_lossy(&validated.stdout));
    let located = envelope["findings"]
        .as_array()
        .expect("findings[]")
        .iter()
        .find(|f| f["code"] == "conformance.unknown-field")
        .expect("the parse diagnostic is in the envelope");
    assert_eq!(
        located["route"],
        Value::Null,
        "an enumerated exemption carries no route — the located message IS the repair; \
         got:\n{located:#}",
    );
    assert!(
        engine::finding::is_route_exempt("conformance.unknown-field"),
        "…and the exemption is decided by membership",
    );
    assert!(
        !engine::finding::is_route_exempt("conformance.invented-by-this-test"),
        "…of an ENUMERATION, never of the `conformance.` namespace: a prefix match would \
         exempt any future member by accident",
    );
}

/// Where each [`DOCTYPE_DOORS`] member's unknown-doctype answer is proven — one of the two
/// answers driven here, or classified against the axis suite that drives every door.
///
/// **The default arm is sound rather than lazy**, and this is the reason: `unknown_doctype_axis.rs`
/// derives its own rows from [`DOCTYPE_DOORS`] — it iterates the binary's doors rather than a
/// curated list — so a door added to the registry joins that sweep with no edit anywhere. What
/// this arm owes is therefore not a per-door cell but the *pair*: exactly the two doors it drives
/// live are marked driven, which the assertion below holds as an equality rather than a
/// containment.
fn unknown_doctype_cell(path: &[&str]) -> &'static str {
    match path {
        ["doc", "create"] => DRIVEN_CREATE_GATE,
        ["doc", "show"] => DRIVEN_ALREADY_EXISTS,
        _ => "unknown_doctype_axis.rs",
    }
}

const DRIVEN_CREATE_GATE: &str = "driven here: the create-gate answer";
const DRIVEN_ALREADY_EXISTS: &str = "driven here: the already-exists answer";

/// **Arm 6, cell 2** — nothing the reader is told is false, at the door that says it.
///
/// Four surfaces, three of them axis-swept elsewhere and dispositioned here so a new door
/// cannot slip in silently: the **unknown-doctype** answer over [`DOCTYPE_DOORS`] (two
/// answers, split on the door's *kind* — a create-gate door names the authorable set, every
/// other door names the fault in the resolved cascade); **not a git repository**, which
/// answered in three texts and two contract shapes across eleven-plus sites; the **clap**
/// seam, whose read-shaped miss now earns jigc's own answer naming the two read surfaces
/// the packs never named; and the composed **delegation prose**, where a step names the
/// actor and the instrument — asserted with its **omitting axis**, so a sentence cannot be
/// parked in a shared step and counted for a loop that never composes it.
#[test]
fn nothing_the_reader_is_told_is_false_at_the_door_that_says_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("dev-task", "probe the answering doors");

    // ── The unknown-doctype axis: every door disposed, two driven ───────────
    let driven: BTreeSet<Vec<&str>> = DOCTYPE_DOORS
        .iter()
        .map(|(path, _)| path.to_vec())
        .filter(|path| {
            matches!(
                unknown_doctype_cell(path),
                DRIVEN_CREATE_GATE | DRIVEN_ALREADY_EXISTS
            )
        })
        .collect();
    assert_eq!(
        driven,
        BTreeSet::from([vec!["doc", "create"], vec!["doc", "show"]]),
        "exactly the doors this arm drives live are marked driven — a door marked driven \
         and never run is the masking shape this table exists to refuse",
    );
    assert!(
        DOCTYPE_DOORS.len() >= 15,
        "…and the classified remainder is a real set, swept whole by unknown_doctype_axis.rs; \
         got {} doors",
        DOCTYPE_DOORS.len(),
    );
    let created = corpus.jigc(&["doc", "create", "nosuch", "--title", "X", "--task", &task]);
    let surface = both_streams(&created);
    assert!(
        !created.status.success() && surface.contains("create.unknown-doctype"),
        "a create-gate door names the mint it cannot make; got:\n{surface}",
    );
    let shown = corpus.jigc(&["doc", "show", "nosuch:thing"]);
    let surface = both_streams(&shown);
    assert!(
        !shown.status.success() && surface.contains("store.unknown-type"),
        "every other door names the doctype the cascade does not carry; got:\n{surface}",
    );
    assert!(
        !surface.contains("PackResourceKind") && !surface.contains("Schemas with id"),
        "…and no door leaks a Rust enum's Debug rendering at a reader; got:\n{surface}",
    );

    // ── Outside a git repository: one text, one route ───────────────────────
    let bare = TempDir::new("not-a-repo");
    let home = TempDir::new("not-a-repo-home");
    let out = run(bare.path(), home.path(), &["describe"]);
    let surface = both_streams(&out);
    assert!(
        !out.status.success(),
        "running outside a repository must not succeed; got {:?}",
        out.status,
    );
    assert!(
        surface.contains("not inside a git repository"),
        "…with one text naming the fault; got:\n{surface}",
    );
    assert!(
        surface.contains("git init"),
        "…and the route that makes this a repository; got:\n{surface}",
    );

    // ── The clap seam: a read-shaped miss earns a read answer ───────────────
    let missed = corpus.jigc(&["doc", "read"]);
    let surface = both_streams(&missed);
    assert!(
        !missed.status.success(),
        "an unrecognized subcommand must not succeed; got {:?}",
        missed.status,
    );
    assert!(
        surface.contains("jigc doc list") && surface.contains("jigc doc show"),
        "a read intent is answered with the read verbs — the two surfaces the packs named \
         0× before this wave; got:\n{surface}",
    );

    // ── The composed prose: who acts, and with what ─────────────────────────
    let increment = corpus.jigc_ok(&["start", "--workflow", "increment", "the probe increment"]);
    assert!(
        increment.contains("did not build it"),
        "the validate phase names the ACTOR it delegates to; got:\n{increment}",
    );
    assert!(
        increment.contains("cannot edit or commit"),
        "…and the instrument's own bound, which is what makes the delegation checkable",
    );
    assert!(
        !increment.contains("robust-case advocate"),
        "the omitting axis: `increment` composes neither settle nor triage, so the advocate \
         is owed there by nobody and must not appear",
    );
    let planning = corpus.jigc_ok(&["start", "--workflow", "planning", "the probe wave"]);
    assert!(
        planning.contains("independent robust-case advocate"),
        "…while the loop that IS mandated one names it; got:\n{planning}",
    );
}
