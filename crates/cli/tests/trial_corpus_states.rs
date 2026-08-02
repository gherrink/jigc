//! M45 Increment 1 / T2–T5 — the **trial-shaped fixture builder**, its isolation
//! fence, the three managed-corpus states, the two repo-furniture states, and the
//! **shape-class coverage rule** they must jointly satisfy
//! (`implementation/pinning.md` §4).
//!
//! The builder is the shared substrate the compose-golden suite and the contract
//! property suites run over, so the substrate itself needs a fence: a fixture that
//! silently reads the *wrong pack* would regenerate a whole golden set of wrong
//! bytes, and nothing downstream would notice.
//!
//! Three arms, all over the real binary (`CARGO_BIN_EXE_jigc`):
//!
//!   (1) **Every named state builds** — the sweep entry point suites use, iterating
//!       [`State::ALL`] so a state added to the builder auto-joins here.
//!
//!   (2) **`fresh` is `jigc setup` only, and really ran it** — the `.jigc/`
//!       workbench and setup's own `.git/hooks/pre-commit` are on disk, built by
//!       driving the binary (the narrowed build rule: managed state through the
//!       binary, unmanaged furniture written directly).
//!
//!   (3) **A leaked `JIGC_PACK_DIR` never reaches a built state** — the parent test
//!       process is poisoned with a bogus pack directory and the built corpus still
//!       composes the embedded `[dev ▸ methodology]` pair. The provenance is read
//!       from a **real invocation's output** (`jigc start`'s `Pack:` line), never
//!       from the env the test itself just set.
//!
//! Then one arm per managed-corpus state, each asserting the state's **defining
//! property off real artifacts** — git's index and log, and the task working area
//! on disk — never off the builder's own bookkeeping:
//!
//!   (4) **`committed-singletons`** — vision/roadmap/decisions-log are tracked at
//!       their resolved `placement` homes and no task is left live (created *and*
//!       finalized, not merely staged).
//!
//!   (5) **`migrated`** — the foreign source was committed *before* the migration
//!       and the landing commit carries both its deletion and the managed doc's
//!       addition, so the retirement is a real destructive retire, not a tidy-up of
//!       an untracked file.
//!
//!   (6) **`refs-post-hoc`** — the `edited-from-base` provenance no finalized state
//!       can carry: a staged copy under `.jigc/tasks/<id>/docs/` holding the edge
//!       the committed bytes do not.
//!
//! Then T4's two **repo-furniture** states, both written directly (the narrowed
//! build rule: jigc has no verb that authors a foreign hook or a vendor dir):
//!
//!   (7) **`chatty-hooks`** — the hook speaks on a **successful** commit driven
//!       through a real jigc write path, and `jigc setup`'s own hook is **gone**, not
//!       wrapped: the assertion reads the real [`cli::setup::PRECOMMIT_SENTINEL`], so
//!       "replace, not append" is mechanically true rather than a comment.
//!
//!   (8) **`vendored`** — the gitignored runtime tree is on disk yet invisible to
//!       `git ls-files --cached --others --exclude-standard` (the ingest funnel's
//!       candidate walk) while the tracked code file is listed — a discrimination,
//!       not an empty walk.
//!
//! Then T5's **shape-class coverage** — the rule that decides whether the state set
//! is *complete*, plus the three arms proving its populations are real rather than
//! shaped to satisfy the derivation:
//!
//!   (9) **the multi-slot cell** — the roadmap milestone with *both* prose leaves;
//!  (10) **the nested / `id-from` enum cells** — a changelog staged group and a cut
//!       release with a nested one, at both depths;
//!  (11) **the code-anchor cell** — a `spec` + `arch-doc` whose anchors resolve
//!       against real tracked symbols, proven by *flipping the lever* (delete the
//!       symbols, watch both checks fire) so the clean sweep cannot be vacuous.

mod support;

use support::trial_corpus::{State, TrialCorpus, unique_root};

/// (0) The isolation fence's own precondition: **two corpora never share a root**.
///
/// The builder's isolation claim ("a tempdir per corpus") is only as strong as the
/// mint that names it. A wall-clock-only disambiguator is not a mint: macOS truncates
/// `SystemTime::now()` to **microsecond** granularity, so a burst of calls inside one
/// process returns the same instant many times over — and two `#[test]` threads of one
/// binary then build *into the same directory*, where the second `git init` dies on
/// `File exists`.
///
/// So the property is asserted over the mint directly, in the two shapes that produce
/// it: a **sequential burst** (the coarse clock repeating on its own) and **concurrent
/// threads** (the parallel-`#[test]` shape the harness actually runs). Distinctness must
/// hold by construction, not by winning a race.
#[test]
fn a_minted_root_is_unique_under_burst_and_across_threads() {
    use std::collections::BTreeSet;

    const BURST: usize = 1_000;
    const THREADS: usize = 8;
    const PER_THREAD: usize = 500;

    let burst: BTreeSet<_> = (0..BURST).map(|_| unique_root("burst")).collect();
    assert_eq!(
        burst.len(),
        BURST,
        "a sequential burst of {BURST} mints must yield {BURST} distinct roots; \
         got {} — the disambiguator is coarser than the call rate",
        burst.len(),
    );

    let handles: Vec<_> = (0..THREADS)
        .map(|_| {
            std::thread::spawn(|| {
                (0..PER_THREAD)
                    .map(|_| unique_root("parallel"))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let concurrent: BTreeSet<_> = handles
        .into_iter()
        .flat_map(|h| h.join().expect("mint thread must not panic"))
        .collect();
    assert_eq!(
        concurrent.len(),
        THREADS * PER_THREAD,
        "{THREADS} threads minting {PER_THREAD} roots each must yield {} distinct roots; \
         got {} — two parallel `#[test]`s would build into one directory",
        THREADS * PER_THREAD,
        concurrent.len(),
    );
}

/// (1) Every named state in the builder constructs a real git repo.
#[test]
fn every_named_state_builds() {
    for state in State::ALL {
        let corpus = TrialCorpus::build(*state);
        assert!(
            corpus.repo().join(".git").is_dir(),
            "state `{}` must build a real git repo",
            state.name(),
        );
        assert_eq!(corpus.state(), *state);
    }
}

/// (2) `fresh` is `jigc setup` only — and the workbench plus setup's own
/// pre-commit hook prove the binary, not the test, put the state there.
#[test]
fn fresh_carries_the_workbench_and_setups_own_pre_commit_hook() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();

    assert!(
        repo.join(".jigc").is_dir(),
        "`fresh` must carry the `.jigc/` workbench setup installs",
    );
    let hook = repo.join(".git/hooks/pre-commit");
    assert!(
        hook.is_file(),
        "`fresh` must carry setup's own pre-commit hook",
    );
    let body = support::trial_corpus::read(&repo, ".git/hooks/pre-commit");
    assert!(
        body.contains("jigc"),
        "the installed pre-commit hook must be jigc's own:\n{body}",
    );
}

/// (3) The isolation fence: a poisoned parent `JIGC_PACK_DIR` does not reach the
/// built state, proven by the pack provenance a real invocation prints.
#[test]
fn a_leaked_pack_dir_never_reaches_a_built_state() {
    let mut bogus = std::env::temp_dir();
    bogus.push(format!(
        "jigc-bogus-pack-{}-{}",
        std::process::id(),
        engine::tempname::unique_nanos(),
    ));
    std::fs::create_dir_all(&bogus).expect("create the bogus pack dir");

    // SAFETY: the Rust harness runs `#[test]` fns in parallel and the environment is
    // process-global, so this is the **sole** in-process writer of `JIGC_PACK_DIR` in
    // this binary, written once and never cleared. Poisoning the parent is the point:
    // the fence is that `TrialCorpus` removes the variable from every child it spawns,
    // which is unprovable unless the parent actually carries it.
    unsafe { std::env::set_var("JIGC_PACK_DIR", &bogus) };

    let corpus = TrialCorpus::build(State::Fresh);
    let stdout = corpus.jigc_ok(&["start"]);
    let pack_line = stdout
        .lines()
        .find(|line| line.starts_with("Pack:"))
        .unwrap_or_else(|| panic!("orientation must print a `Pack:` provenance line:\n{stdout}"));
    assert!(
        pack_line.contains("dev/") && pack_line.contains("methodology/"),
        "the built state must compose the embedded [dev ▸ methodology] pair, \
         not the poisoned JIGC_PACK_DIR: {pack_line}",
    );

    let _ = std::fs::remove_dir_all(&bogus);
}

/// (4) `committed-singletons` — the three methodology singletons are **created and
/// finalized**, so git tracks each at its resolved home and no task is left live.
/// The homes are the schemas' own: `VISION.md` and `docs/roadmap.md` /
/// `docs/decisions-log.md` are `placement` files, so they bypass `docs-root`.
#[test]
fn committed_singletons_tracks_the_three_singletons_at_their_homes() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let tracked = corpus.git(&["ls-files"]);
    for home in ["VISION.md", "docs/roadmap.md", "docs/decisions-log.md"] {
        assert!(
            tracked.lines().any(|line| line == home),
            "`committed-singletons` must track `{home}`; git ls-files:\n{tracked}",
        );
        let body = support::trial_corpus::read(&corpus.repo(), home);
        assert!(
            !body.trim().is_empty(),
            "the committed `{home}` must carry content",
        );
    }
    assert_eq!(
        corpus.live_task(),
        None,
        "every `committed-singletons` task is finalized — none stays live",
    );
    let listed = corpus.jigc_ok(&["task", "list"]);
    assert!(
        listed.contains("no active tasks"),
        "`committed-singletons` must leave no active task; got:\n{listed}",
    );
}

/// (5) `migrated` — the managed doc landed through `jigc migrate … --approve` and
/// the **foreign source is retired in the same commit**. Both halves are read off
/// git: the source was tracked before (so its deletion is a real retirement), the
/// finalize commit carries the add *and* the delete, and the path is gone from the
/// index and the working tree.
#[test]
fn migrated_retires_the_committed_foreign_source_in_the_landing_commit() {
    let corpus = TrialCorpus::build(State::Migrated);
    let foreign = support::trial_corpus::FOREIGN_VISION_PATH;

    let history = corpus.git(&["log", "--oneline", "--all", "--", foreign]);
    assert!(
        !history.trim().is_empty(),
        "the foreign source must have been committed before the migration, \
         so its retirement is a real deletion",
    );

    let landing = corpus.git(&["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        landing.lines().any(|line| line == format!("D\t{foreign}")),
        "the landing commit must retire the foreign source; got:\n{landing}",
    );
    assert!(
        landing.lines().any(|line| line == "A\tVISION.md"),
        "the landing commit must add the managed doc; got:\n{landing}",
    );

    let tracked = corpus.git(&["ls-files"]);
    assert!(
        !tracked.lines().any(|line| line == foreign),
        "the retired foreign source must be gone from the index; git ls-files:\n{tracked}",
    );
    assert!(
        !corpus.repo().join(foreign).exists(),
        "the retired foreign source must be gone from the working tree",
    );
}

/// (6) `refs-post-hoc` — an edge set by `doc set-field` on a **committed** doc
/// inside a **live** task, so the corpus carries the `edited-from-base` provenance
/// a finalized state cannot: a staged copy under `.jigc/tasks/<id>/docs/` whose
/// bytes diverge from the committed ones, carrying the edge the committed doc
/// does not.
#[test]
fn refs_post_hoc_carries_an_edited_from_base_staged_copy() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let repo = corpus.repo();
    let task = corpus
        .live_task()
        .expect("`refs-post-hoc` leaves its edge-setting task live");

    // The edge's target is committed — the edge points at real, tracked evidence.
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked
            .lines()
            .any(|line| line == "docs/research/context-loss.md"),
        "the grounding research must be committed; git ls-files:\n{tracked}",
    );

    let provenance =
        support::trial_corpus::read(&repo, &format!(".jigc/tasks/{task}/docs/provenance.json"));
    assert!(
        provenance.contains("\"vision:vision\": \"edited-from-base\""),
        "the committed vision must be copied in for editing, not created; got:\n{provenance}",
    );

    let staged =
        support::trial_corpus::read(&repo, &format!(".jigc/tasks/{task}/docs/vision:vision.md"));
    let committed = support::trial_corpus::read(&repo, "VISION.md");
    assert_ne!(
        staged, committed,
        "the staged copy must diverge from the committed bytes",
    );
    assert!(
        staged.contains("grounded-in: [research:context-loss]"),
        "the staged copy must carry the edge `set-field` wrote; got:\n{staged}",
    );
    assert!(
        !committed.contains("grounded-in"),
        "the committed doc must NOT carry the edge — it is live in the task only; got:\n{committed}",
    );
}

/// (7) `chatty-hooks` — the foreign hook speaks on a **successful** commit landed
/// through a real jigc write path (`start` → `doc create` → `task finalize`), and it
/// **replaced** setup's own hook rather than wrapping it.
///
/// The replacement half is asserted against the binary's own
/// [`cli::setup::PRECOMMIT_SENTINEL`] — the marker setup's installer keys on — so a
/// future wrap-instead-of-replace regression reddens here instead of quietly
/// restoring jigc's backstop underneath every suite that runs this state.
#[test]
fn chatty_hooks_speaks_on_a_successful_commit_and_replaced_setups_hook() {
    let corpus = TrialCorpus::build(State::ChattyHooks);
    let hook = support::trial_corpus::read(&corpus.repo(), ".git/hooks/pre-commit");
    assert!(
        !hook.contains(cli::setup::PRECOMMIT_SENTINEL),
        "`chatty-hooks` REPLACES setup's hook — its sentinel must be absent:\n{hook}",
    );
    assert!(
        hook.contains(support::trial_corpus::CHATTY_HOOK_MARKER),
        "the installed hook must be the chatty one:\n{hook}",
    );

    // A real jigc write path, landing a real commit — the hook fires on success.
    let task = corpus.start_workflow("planning", "plan the first wave");
    corpus.jigc_ok(&[
        "doc", "create", "roadmap", "--title", "Roadmap", "--task", &task,
    ]);
    let landed = corpus.finalize(&task, "planning", "mint the roadmap", false);

    assert!(
        landed.contains(support::trial_corpus::CHATTY_HOOK_MARKER),
        "the successful commit must relay the chatty hook's stdout; got:\n{landed}",
    );
    assert!(
        landed.contains("--- hook output ---"),
        "the relay must arrive in its delimited section; got:\n{landed}",
    );
    // The commit really landed — a hook that spoke on a *rejected* commit would
    // prove nothing about the success path.
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked.lines().any(|line| line == "docs/roadmap.md"),
        "the finalize must have landed its commit; git ls-files:\n{tracked}",
    );
}

/// (8) `vendored` — the gitignored runtime tree is **present on disk** and **absent
/// from** `git ls-files --cached --others --exclude-standard`, the walk the ingest
/// funnel takes its candidates from, while the tracked code file *is* listed.
///
/// Asserting the file exists is what keeps the invisibility claim from being
/// vacuously true over a directory that was never written.
#[test]
fn vendored_hides_its_runtime_tree_from_the_ingest_walk() {
    let corpus = TrialCorpus::build(State::Vendored);
    let runtime = support::trial_corpus::VENDORED_RUNTIME_FILE;
    let code = support::trial_corpus::VENDORED_CODE_FILE;

    assert!(
        corpus.repo().join(runtime).is_file(),
        "the vendored runtime file must be on disk — otherwise its invisibility is vacuous",
    );

    let walked = corpus.git(&["ls-files", "--cached", "--others", "--exclude-standard"]);
    assert!(
        !walked.lines().any(|line| line.starts_with("node_modules/")),
        "the gitignored runtime tree must be invisible to the ingest walk; got:\n{walked}",
    );
    assert!(
        walked.lines().any(|line| line == code),
        "the tracked code file must be listed by the same walk; got:\n{walked}",
    );
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked.lines().any(|line| line == code),
        "the code file must be committed, not merely untracked-and-visible; git ls-files:\n{tracked}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// T5 — shape-class coverage (`implementation/pinning.md` §4: *shape-class
// coverage is the rule; the doctype list is only today's instance*).
//
// The six charter states cover no dev-pack doctype and miss the shape cells the
// headline defects live in. The rule: **every schema shape class either pack can
// express is populated in at least one named state** — and a class with no
// populated instance is a **missing state**, not an accepted gap.
//
// Both halves are derived, never hand-listed:
//
//   * **expressed** — the class→doctype map is computed from the ENGINE-LOADED
//     schema model of the composite `[dev ▸ methodology]` registry, so a doctype
//     that starts expressing a class joins its class the day it lands.
//   * **populated** — read back out of each built state through the real read
//     surfaces (`doc list --format json` → `doc show <id> --format json`), so a
//     population claim is corpus evidence, never the builder's own bookkeeping.
// ─────────────────────────────────────────────────────────────────────────────

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema, pack_field_types};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Field, FieldType, Leaf, Repeatable, Schema, SectionBody};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// The production composition, built the **CWD-free** way (`pinning.md` §1 —
/// `make_pack()` resolves against the process CWD and is a hazard under parallel
/// tests): `[dev ▸ methodology]`, dev highest-precedence.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every doctype the composite registry ships, loaded through the **CLI** schema
/// loader (so a pack-declared `code-anchor` field resolves to its
/// [`FieldType::Pack`]) against the doctype's own **origin** pack.
fn loaded_schemas(pack: &dyn PackSource) -> BTreeMap<String, Schema> {
    pack.list(PackResourceKind::Schemas)
        .iter()
        .map(|id| {
            let bytes = pack
                .read(PackResourceKind::Schemas, id)
                .unwrap_or_else(|e| panic!("read the `{id}` schema: {e}"));
            let origin = pack.origin_pack(PackResourceKind::Schemas, id);
            let schema = load_pack_schema(origin, &bytes)
                .unwrap_or_else(|e| panic!("load the `{id}` schema: {e}"));
            (schema.ty.clone(), schema)
        })
        .collect()
}

/// One schema **shape class** — a structural cell a doctype either expresses or
/// does not. The list is the *rule*; which doctypes fall in each cell is derived.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ShapeClass {
    /// A repeatable section whose item block carries exactly one prose slot.
    SingleSlotRepeatable,
    /// A repeatable section whose item block carries two or more prose slots.
    MultiSlotRepeatable,
    /// A repeatable nested inside a repeatable item block (`Leaf::Repeatable`).
    NestedRepeatable,
    /// A repeatable whose `id-from` names an **enum** field, so item ids come from
    /// a controlled member set rather than a slugged free-text title.
    IdFromEnum,
    /// A doctype homed at a literal `placement.file`, bypassing `docs-root`.
    Placement,
    /// A field of a **pack-declared** field type — today exactly `code-anchor`,
    /// fenced by [`the_pack_declared_field_type_set_is_exactly_code_anchor`].
    CodeAnchorField,
}

impl ShapeClass {
    const ALL: &'static [ShapeClass] = &[
        ShapeClass::SingleSlotRepeatable,
        ShapeClass::MultiSlotRepeatable,
        ShapeClass::NestedRepeatable,
        ShapeClass::IdFromEnum,
        ShapeClass::Placement,
        ShapeClass::CodeAnchorField,
    ];
}

/// A field carrying a **pack-declared** type (the `code-anchor` cell).
fn is_pack_typed(field: &Field) -> bool {
    matches!(field.ty, FieldType::Pack(_))
}

/// The classes a repeatable expresses **at its own level**: its slot count and
/// whether its `id-from` names an enum member set.
fn level_classes(repeatable: &Repeatable, out: &mut BTreeSet<ShapeClass>) {
    let slots = repeatable
        .block
        .iter()
        .filter(|leaf| matches!(leaf, Leaf::Slot { .. }))
        .count();
    if slots == 1 {
        out.insert(ShapeClass::SingleSlotRepeatable);
    }
    if slots >= 2 {
        out.insert(ShapeClass::MultiSlotRepeatable);
    }
    let id_from_is_enum = repeatable.block.iter().any(|leaf| match leaf {
        Leaf::Field(field) => field.id == repeatable.id_from && field.ty == FieldType::Enum,
        _ => false,
    });
    if id_from_is_enum {
        out.insert(ShapeClass::IdFromEnum);
    }
}

/// Every class this schema **can** express, walked off the loaded model.
fn expressed_classes(schema: &Schema) -> BTreeSet<ShapeClass> {
    let mut out = BTreeSet::new();
    if schema.placement.is_some() {
        out.insert(ShapeClass::Placement);
    }
    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple { fields, .. } => {
                if fields.iter().any(is_pack_typed) {
                    out.insert(ShapeClass::CodeAnchorField);
                }
            }
            SectionBody::Repeatable { repeatable } => expressed_in_repeatable(repeatable, &mut out),
        }
    }
    out
}

fn expressed_in_repeatable(repeatable: &Repeatable, out: &mut BTreeSet<ShapeClass>) {
    level_classes(repeatable, out);
    for leaf in &repeatable.block {
        match leaf {
            Leaf::Field(field) => {
                if is_pack_typed(field) {
                    out.insert(ShapeClass::CodeAnchorField);
                }
            }
            Leaf::Repeatable { repeatable, .. } => {
                out.insert(ShapeClass::NestedRepeatable);
                expressed_in_repeatable(repeatable, out);
            }
            Leaf::Slot { .. } => {}
        }
    }
}

/// A JSON leaf that actually carries content (an authored value, not an absent or
/// blank one) — what makes a population *populated*.
fn filled(value: &Value) -> bool {
    value.as_str().is_some_and(|s| !s.trim().is_empty())
}

/// The classes one committed instance **populates**, read off its
/// `doc show --format json` body against its own schema.
fn populated_classes(schema: &Schema, doc: &Value, out: &mut BTreeSet<ShapeClass>) {
    if schema.placement.is_some() {
        // The instance exists at its literal home — `doc list` only lists committed
        // docs, so its presence *is* the populated placement cell.
        out.insert(ShapeClass::Placement);
    }
    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple { fields, .. } => {
                for field in fields.iter().filter(|f| is_pack_typed(f)) {
                    // A header section's fields project under `fields`; a body
                    // section's under its own section object.
                    if filled(&doc["fields"][&field.id])
                        || filled(&doc["sections"][&section.id][&field.id])
                    {
                        out.insert(ShapeClass::CodeAnchorField);
                    }
                }
            }
            SectionBody::Repeatable { repeatable } => {
                let items = doc["sections"][&section.id]
                    .as_array()
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                if items.is_empty() {
                    continue;
                }
                level_classes(repeatable, out);
                for item in items {
                    populated_in_block(repeatable, item, out);
                }
            }
        }
    }
}

fn populated_in_block(repeatable: &Repeatable, item: &Value, out: &mut BTreeSet<ShapeClass>) {
    for leaf in &repeatable.block {
        match leaf {
            Leaf::Field(field) => {
                if is_pack_typed(field) && filled(&item[&field.id]) {
                    out.insert(ShapeClass::CodeAnchorField);
                }
            }
            Leaf::Repeatable { id, repeatable } => {
                let nested = item[id].as_array().map(Vec::as_slice).unwrap_or_default();
                if nested.is_empty() {
                    continue;
                }
                out.insert(ShapeClass::NestedRepeatable);
                level_classes(repeatable, out);
                for sub in nested {
                    populated_in_block(repeatable, sub, out);
                }
            }
            Leaf::Slot { .. } => {}
        }
    }
}

/// Every class one built state populates, read back through the **real read
/// surfaces**: `doc list --format json` for the managed instances, then
/// `doc show <id> --format json` for each one's body.
fn state_populates(
    corpus: &TrialCorpus,
    schemas: &BTreeMap<String, Schema>,
) -> BTreeSet<ShapeClass> {
    let listing: Value =
        serde_json::from_str(&corpus.jigc_ok(&["doc", "list", "--format", "json"]))
            .expect("`doc list --format json` emits one JSON document");
    let mut out = BTreeSet::new();
    for row in listing["docs"].as_array().expect("`docs` is an array") {
        if row["state"].as_str() != Some("managed") {
            continue;
        }
        let id = row["id"].as_str().expect("a listed row carries its id");
        let ty = id.split(':').next().expect("an id is `<type>:<slug>`");
        let Some(schema) = schemas.get(ty) else {
            continue;
        };
        let shown: Value =
            serde_json::from_str(&corpus.jigc_ok(&["doc", "show", id, "--format", "json"]))
                .expect("`doc show --format json` emits one JSON document");
        populated_classes(schema, &shown, &mut out);
    }
    out
}

/// The fence where the `code-anchor` cell's membership is decided: the class is
/// defined as *a field of a pack-declared type*, and today exactly one such type is
/// declared. A pack declaring a second one reddens **here**, forcing the coverage
/// question rather than silently widening a cell that is already green.
#[test]
fn the_pack_declared_field_type_set_is_exactly_code_anchor() {
    let pack = composite();
    let mut declared: Vec<String> = pack_field_types(&pack)
        .expect("the composed pack's declared field types load")
        .into_iter()
        .map(|decl| decl.name)
        .collect();
    declared.sort();
    assert_eq!(
        declared,
        vec!["code-anchor".to_string()],
        "the `code-anchor` shape class is *the* pack-declared field-type cell; \
         a newly declared type needs its own coverage answer",
    );
}

/// **The T5 done-criterion.** Every shape class either pack can express is
/// populated in at least one named state — derived on both sides, so neither half
/// is a list that can rot.
#[test]
fn shape_class_coverage_is_complete() {
    let pack = composite();
    let schemas = loaded_schemas(&pack);

    // (a) class → the doctypes expressing it, derived from the loaded schema model.
    let mut expressed_by: BTreeMap<ShapeClass, Vec<String>> = BTreeMap::new();
    for (ty, schema) in &schemas {
        for class in expressed_classes(schema) {
            expressed_by.entry(class).or_default().push(ty.clone());
        }
    }
    // Every class in the rule set is expressed by *something* — a class no shipped
    // doctype expresses would make its coverage vacuous.
    for class in ShapeClass::ALL {
        assert!(
            expressed_by.contains_key(class),
            "no shipped doctype expresses {class:?} — the class is vacuous, \
             which is a finding about the rule set, not a free pass",
        );
    }

    // (b) the union of what the named states actually populate, read back through
    //     the real read surfaces.
    let mut populated: BTreeSet<ShapeClass> = BTreeSet::new();
    let mut by_state: BTreeMap<&str, BTreeSet<ShapeClass>> = BTreeMap::new();
    for state in State::ALL {
        let corpus = TrialCorpus::build(*state);
        let found = state_populates(&corpus, &schemas);
        populated.extend(found.iter().copied());
        by_state.insert(state.name(), found);
    }

    let missing: Vec<(&ShapeClass, &Vec<String>)> = expressed_by
        .iter()
        .filter(|(class, _)| !populated.contains(class))
        .collect();
    assert!(
        missing.is_empty(),
        "a shape class with no populated instance is a MISSING STATE, not an \
         accepted gap (pinning.md §4). Unpopulated: {missing:?}\nPopulated per state: {by_state:?}",
    );
}

/// The whole committed instance at `id`, read through `doc show --format json`.
fn shown(corpus: &TrialCorpus, id: &str) -> Value {
    serde_json::from_str(&corpus.jigc_ok(&["doc", "show", id, "--format", "json"]))
        .expect("`doc show --format json` emits one JSON document")
}

/// The **managed** committed instances of `doctype` in a state, by identity — read
/// off `doc list`, so a slug the binary minted is never reconstructed test-side.
fn managed_ids(corpus: &TrialCorpus, doctype: &str) -> Vec<String> {
    let listing: Value =
        serde_json::from_str(&corpus.jigc_ok(&["doc", "list", doctype, "--format", "json"]))
            .expect("`doc list --format json` emits one JSON document");
    listing["docs"]
        .as_array()
        .expect("`docs` is an array")
        .iter()
        .filter(|row| row["state"].as_str() == Some("managed"))
        .map(|row| {
            row["id"]
                .as_str()
                .expect("a listed row carries its id")
                .to_string()
        })
        .collect()
}

/// The `doc-code.*` findings a store-scope sweep raises. The exit code is not
/// asserted: `jigc validate` is report-only at store scope for content findings, and
/// what this reads is the findings themselves.
fn doc_code_findings(corpus: &TrialCorpus) -> Vec<Value> {
    let out = corpus.jigc(&["validate", "--format", "json"]);
    let report: Value =
        serde_json::from_slice(&out.stdout).expect("`jigc validate` emits valid JSON");
    report["findings"]
        .as_array()
        .expect("the report carries a findings array")
        .iter()
        .filter(|f| {
            f["code"]
                .as_str()
                .is_some_and(|c| c.starts_with("doc-code."))
        })
        .cloned()
        .collect()
}

/// (9) `committed-singletons` populates the **multi-slot repeatable** cell: the
/// roadmap's milestone carries *both* prose leaves, not one.
///
/// `milestones` is the only multi-slot repeatable either pack ships, so a milestone
/// with a single leaf filled would leave the class covered on paper and unswept in
/// fact — the read-back is per-leaf for exactly that reason.
#[test]
fn committed_singletons_populates_a_multi_slot_roadmap_milestone() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let doc = shown(&corpus, "roadmap:roadmap");
    let milestones = doc["sections"]["milestones"]
        .as_array()
        .expect("the roadmap carries a `milestones` array");
    assert_eq!(
        milestones.len(),
        1,
        "`committed-singletons` populates exactly one milestone; got {milestones:?}",
    );
    let milestone = &milestones[0];
    for leaf in ["proves", "decomposition"] {
        assert!(
            filled(&milestone[leaf]),
            "the multi-slot cell needs BOTH leaves authored — `{leaf}` is empty: {milestone:?}",
        );
    }
    assert!(
        filled(&milestone["title"]),
        "the item's `id-from: title` source is authored: {milestone:?}",
    );
}

/// (10) `committed-singletons` populates the **nested repeatable** and **`id-from`
/// enum** cells, at *both* depths: the staging area's change-group (shallow) and a
/// cut release's nested one (deep), plus the doctype's optional `link` field.
#[test]
fn committed_singletons_populates_a_nested_changelog_release() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);

    // The dev-pack singleton is committed at its literal `placement` home.
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked.lines().any(|line| line == "CHANGELOG.md"),
        "the changelog lands at its literal placement home; git ls-files:\n{tracked}",
    );

    let doc = shown(&corpus, "changelog:changelog");

    // Shallow: the staging area, id'd from the `category` enum.
    let staged = doc["sections"]["unreleased-changes"]
        .as_array()
        .expect("`unreleased-changes` is an array");
    assert_eq!(staged.len(), 1, "one staged change-group; got {staged:?}");
    assert_eq!(
        staged[0]["id"], staged[0]["category"],
        "an `id-from: <enum>` item takes its id from the enum member, not a slugged title",
    );
    assert!(
        filled(&staged[0]["notes"]),
        "the staged group carries notes"
    );

    // Deep: a cut release with a NESTED change-group.
    let releases = doc["sections"]["releases"]
        .as_array()
        .expect("`releases` is an array");
    assert_eq!(releases.len(), 1, "one cut release; got {releases:?}");
    let release = &releases[0];
    assert!(
        filled(&release["link"]),
        "the OPTIONAL `link` field is authored — an optional leaf empty in every \
         state leaves its write path unswept: {release:?}",
    );
    let nested = release["changes"]
        .as_array()
        .expect("the release carries a nested `changes` array");
    assert_eq!(
        nested.len(),
        1,
        "the release carries one nested change-group; got {nested:?}",
    );
    assert_eq!(
        nested[0]["id"], nested[0]["category"],
        "the nested item is `id-from: <enum>` too — the deep half of the cell",
    );
    assert!(
        filled(&nested[0]["notes"]),
        "the nested group carries notes: {nested:?}",
    );
}

/// (11) `vendored` populates the **code-anchor** cell with anchors that genuinely
/// **resolve** against real tracked symbols.
///
/// The claim is proven by flipping the lever, not by a clean report alone: a clean
/// report over a corpus whose probe never ran looks identical to a clean report over
/// resolving anchors. So the arm reads the sweep clean, then **removes the two
/// anchored symbols from the tracked files** and reads it again — both checks fire,
/// naming the two anchor addresses. That is what makes "resolves" a fact.
#[test]
fn vendored_carries_a_spec_and_arch_doc_whose_code_anchors_resolve() {
    let corpus = TrialCorpus::build(State::Vendored);
    let repo = corpus.repo();

    // Both dev-pack docs are committed and managed.
    let specs = managed_ids(&corpus, "spec");
    let arch_docs = managed_ids(&corpus, "arch-doc");
    assert_eq!(specs.len(), 1, "`vendored` commits one spec; got {specs:?}");
    assert_eq!(
        arch_docs.len(),
        1,
        "`vendored` commits one arch-doc; got {arch_docs:?}"
    );

    // The anchors name TRACKED files — an anchor over an untracked path would not be
    // the tracked-symbol case the coverage rule asks for.
    let tracked = corpus.git(&["ls-files"]);
    for file in [
        support::trial_corpus::VENDORED_CODE_FILE,
        support::trial_corpus::VENDORED_TEST_FILE,
    ] {
        assert!(
            tracked.lines().any(|line| line == file),
            "the anchored `{file}` must be tracked; git ls-files:\n{tracked}",
        );
    }

    let criterion = &shown(&corpus, &specs[0])["sections"]["criteria"][0];
    assert_eq!(
        criterion["maps-to-test"].as_str(),
        Some(
            format!(
                "{}#{}",
                support::trial_corpus::VENDORED_TEST_FILE,
                support::trial_corpus::VENDORED_TEST_SYMBOL,
            )
            .as_str()
        ),
        "the criterion's code-anchor names the tracked `#[test]` fn: {criterion:?}",
    );
    let component = &shown(&corpus, &arch_docs[0])["sections"]["components"][0];
    assert_eq!(
        component["implemented-by"].as_str(),
        Some(
            format!(
                "{}#{}",
                support::trial_corpus::VENDORED_CODE_FILE,
                support::trial_corpus::VENDORED_CODE_SYMBOL,
            )
            .as_str()
        ),
        "the component's code-anchor names the tracked TypeScript symbol: {component:?}",
    );

    // (a) They RESOLVE — the store-scope sweep raises no `doc-code` finding.
    let clean = doc_code_findings(&corpus);
    assert!(
        clean.is_empty(),
        "both anchors resolve against the tracked source; got {clean:?}",
    );

    // (b) The lever: delete the two anchored symbols and the same sweep fires. A
    //     probe that never ran would stay silent here, so this is what stops (a)
    //     from being vacuous.
    std::fs::write(
        repo.join(support::trial_corpus::VENDORED_CODE_FILE),
        "export function trim(s: string): string {\n  return s.trim();\n}\n",
    )
    .expect("rewrite the anchored source");
    std::fs::write(
        repo.join(support::trial_corpus::VENDORED_TEST_FILE),
        "fn helper() {}\n",
    )
    .expect("rewrite the anchored test");

    let broken: Vec<String> = doc_code_findings(&corpus)
        .iter()
        .filter_map(|f| f["code"].as_str().map(str::to_string))
        .collect();
    for code in ["doc-code.symbol-exists", "doc-code.criterion-maps-to-test"] {
        assert!(
            broken.iter().any(|c| c == code),
            "removing the anchored symbols must raise `{code}`; got {broken:?}",
        );
    }
}
