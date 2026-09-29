//! **One workflow resolution** — the definition a door enforces is the definition the
//! compose door resolved (M52 Increment 9, the validate→fix loop).
//!
//! The cascade resolves *all* customization, workflow definitions included: a
//! project-layer whole-file shadow at `.jigc/config/workflows/<id>.yaml` wins over the
//! pack's copy at file granularity, and a project-layer id **no pack ships** resolves at
//! all (`design/overrides.md` → Resolution algorithm phase 2). Every *composing* door has
//! read it that way since M14 — `CascadeDefs::read_workflow`, which `jigc start`,
//! resume, re-entry, `--explain`, `describe`, `milestone execute` and the store sweep all
//! go through.
//!
//! **Three doors that read the task's *recorded* workflow read the pack instead**, and
//! `jigc config`'s write-time anchor check read the pack too — so the workflow a task was
//! minted on and the workflow its gates enforced were two different documents. This is
//! the exact divergence M49 Increment 3 T2 closed one resource kind over, for **schemas**
//! (`schema_resolution_unified`); the workflow kind
//! was left un-swept, and the consequence is worse than a wrong answer because two of the
//! four doors are *write* doors:
//!
//! | reader | door | driven at `aa16a64e` |
//! |---|---|---|
//! | `TaskArea::recorded_workflow` | `jigc task finalize` | a shadow composing the exit-4 migration hold **committed at exit 0** |
//! | `TaskArea::workflow_def` | `jigc task bind` | a project-layer-only id: *"no pack resource of kind workflows"*, exit 1 |
//! | `DocArea::workflow_gate` | `jigc doc create` / `author` / `set-*` | a doctype the resolved `allows-create:` does **not** grant created at exit 0 |
//! | `config::check_anchor_present` | `jigc config remove-step` | a delta recorded against a step the resolved include list does not contain, exit 0 |
//!
//! The finalize cell is pinned by the T4 axis suite, which gained the layer dimension in
//! the same commit
//! (`migration_review_hold_axis`); the other three
//! are driven here, each through the real binary, plus the **source-level fence**: the
//! property is checked where membership is decided, so a fifth pack-direct read reddens
//! when it is written rather than when a trial finds it (the `schema_resolution_unified`
//! / `test_target_registration` precedent).

use crate::support;
use crate::support::rust_source;

use std::fs;
use std::path::{Path, PathBuf};
use support::trial_corpus::{State, TrialCorpus};

/// A project-layer shadow of the shipped `single-task` id that **narrows** its
/// create-gate: the pack grants `adr` *and* `changelog`, this grants `adr` only.
const GATE_NARROWING_SHADOW: &str = "\
---
when: implement a single scoped change
description: A project-layer shadow granting only the adr create-gate.
usage: the create-gate cell.
creates-task: true
allows-create: [{type: adr, as: decision}]
---
{{ include: step:locate }}
{{ include: step:author-commit }}
{{ include: step:finalize }}
";

/// A project-layer shadow of `single-task` that **adds** a `reads:` role. The pack's
/// `single-task` declares none at all, so a bind that succeeds proves the shadow was the
/// definition read — no test-side reimplementation of the role rule involved.
const READS_DECLARING_SHADOW: &str = "\
---
when: implement a single scoped change
description: A project-layer shadow declaring a read-role the pack's copy does not.
usage: the reads-declaration cell.
creates-task: true
reads: [{role: direction, type: vision}]
allows-create: [{type: adr, as: decision}]
---
{{ include: step:locate }}
{{ include: step:author-commit }}
{{ include: step:finalize }}
";

/// A project-layer shadow of `single-task` whose include list shares **no** step with the
/// pack's, so an anchor is present in exactly one of the two bodies.
const INCLUDE_DIVERGING_SHADOW: &str = "\
---
when: implement a single scoped change
description: A project-layer shadow with a disjoint include list.
usage: the config-anchor cell.
creates-task: true
allows-create: [{type: adr, as: decision}]
---
{{ include: step:author-commit }}
{{ include: step:finalize }}
";

/// Install a project cascade-layer workflow definition at
/// `.jigc/config/workflows/<id>.yaml` — the whole-file shadow `design/overrides.md`
/// declares, written directly because no verb installs one.
fn write_project_layer_workflow(corpus: &TrialCorpus, id: &str, yaml: &str) {
    let dir = corpus.repo().join(".jigc/config/workflows");
    fs::create_dir_all(&dir).expect("create the project layer's workflows dir");
    fs::write(dir.join(format!("{id}.yaml")), yaml).expect("write the project shadow");
}

/// `DocArea::workflow_gate` — the create-gate enforces the **resolved** workflow's
/// `allows-create:`.
///
/// Driven before the fix: `jigc doc create changelog --title Changelog --task <id>`
/// printed `changelog:changelog` and exited **0**, because the gate read the pack's
/// `single-task`, which grants the `changelog` gate the shadow withholds. A create-gate
/// that admits what the resolved workflow forbids is the CLI's own enforcement of the
/// determinism boundary failing open.
#[test]
fn the_create_gate_enforces_the_resolved_workflows_allows_create() {
    let corpus = TrialCorpus::build(State::Fresh);
    write_project_layer_workflow(&corpus, "single-task", GATE_NARROWING_SHADOW);
    let task = corpus.start_workflow("single-task", "gate probe");

    // The granted half still works — otherwise the refusal below could be any refusal.
    corpus.jigc_ok(&[
        "doc", "create", "adr", "--title", "Granted", "--task", &task,
    ]);

    let refused = corpus.jigc(&[
        "doc",
        "create",
        "changelog",
        "--title",
        "Changelog",
        "--task",
        &task,
    ]);
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert!(
        !refused.status.success(),
        "the resolved workflow grants only `adr`, so `changelog` is gate-blocked; got exit \
         {:?}:\n{printed}",
        refused.status.code(),
    );
    assert!(
        printed.contains("create.gate-blocked"),
        "the refusal is the create-gate's own code, not an unrelated failure:\n{printed}",
    );
    assert!(
        !corpus
            .repo()
            .join(".jigc/tasks")
            .join(&task)
            .join("docs")
            .join("changelog.md")
            .exists(),
        "a blocked create stages nothing",
    );
}

/// `TaskArea::workflow_def` — `jigc task bind` enforces the **resolved** workflow's
/// `reads:` roles.
///
/// The pack's `single-task` declares no read-roles at all, so a bind that lands proves
/// the shadow was read. Driven before the fix over a project-layer-**only** id, the same
/// seam did not merely read the wrong body — it had none, and answered *"the recorded
/// workflow `offverb` reads back: no pack resource of kind workflows with id `offverb`"*
/// at exit 1, over a workflow `jigc start --workflow` had resolved at exit 0.
#[test]
fn task_bind_enforces_the_resolved_workflows_reads_declaration() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    write_project_layer_workflow(&corpus, "single-task", READS_DECLARING_SHADOW);
    let task = corpus.start_workflow("single-task", "bind probe");

    let bound = corpus.jigc(&["task", "bind", "direction", "vision:vision", &task]);
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&bound.stdout),
        String::from_utf8_lossy(&bound.stderr),
    );
    assert!(
        bound.status.success(),
        "the resolved workflow declares the `direction` read-role, so the bind lands; got \
         exit {:?}:\n{printed}",
        bound.status.code(),
    );

    // …and a role the resolved workflow does *not* declare is still refused, so the arm
    // is not satisfied by a seam that accepts everything.
    let refused = corpus.jigc(&["task", "bind", "spec", "vision:vision", &task]);
    assert!(
        !refused.status.success(),
        "an undeclared role is still refused:\n{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
}

/// `config::check_anchor_present` — the write-time anchor check adjudicates against the
/// **resolved** include list, which is the list resolution applies the delta to.
///
/// Driven before the fix: over a shadow whose include list is `author-commit · finalize`,
/// `jigc config remove-step 'workflow:single-task#implement'` **recorded the delta at
/// exit 0** — a write-time check whose entire purpose is to refuse an anchor that is not
/// there, accepting one against a body nothing resolves.
#[test]
fn the_config_anchor_check_adjudicates_the_resolved_include_list() {
    let corpus = TrialCorpus::build(State::Fresh);
    write_project_layer_workflow(&corpus, "single-task", INCLUDE_DIVERGING_SHADOW);

    let refused = corpus.jigc(&["config", "remove-step", "workflow:single-task#implement"]);
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert!(
        !refused.status.success(),
        "`implement` is in the pack's body and not in the resolved one, so the anchor is \
         absent; got exit {:?}:\n{printed}",
        refused.status.code(),
    );
    assert!(
        printed.contains("config.anchor-absent"),
        "the refusal is the anchor check's own code:\n{printed}",
    );
    assert!(
        !corpus.repo().join(".jigc/config/manifest.yaml").exists(),
        "a refused write-time check writes nothing",
    );

    // The shadow's own step is accepted, so the arm is not satisfied by a check that
    // refuses everything.
    corpus.jigc_ok(&[
        "config",
        "remove-step",
        "workflow:single-task#author-commit",
    ]);
}

fn cli_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Calls that read a pack resource's **bytes** — the act that resolves a definition, as
/// opposed to `list`ing ids or naming the kind in a const. The same discriminator
/// `schema_resolution_unified` uses.
const READ_CALLEES: &[&str] = &["read", "read_pack", "read_text"];

/// Where a production **workflow** read is allowed to live, each row with its reason.
///
/// - `pack.rs` is the pack layer itself — the loader, the manifest reads and the
///   pack-load fences, which by construction inspect *each pack's own bytes* and must not
///   go through the cascade.
/// - `start.rs::read_workflow` and `start.rs::resolved_workflow` **are** the resolver's
///   pack branch — the read a cascade resolution falls through to when no project layer
///   owns the id. They are the implementation of the rule, not a bypass of it.
///   (`resolved_workflow` reaches `read_pack` rather than `read_workflow` deliberately:
///   the Form-D `workflow-refs.unknown-workflow` mapping answers a *caller-typed* `<X>`,
///   while a *recorded* id the packs no longer ship owes the `pack.resource-missing`
///   answer that names what was searched — `pack_resource_miss_axis` site 4.)
///
/// **`start.rs::selectable_workflows` is no longer among them.** It rode here as a stated
/// exemption carrying its own driven lead — `jigc start` printed the *pack's* `when` for a
/// shipped id the project layer shadows — and M52 Increment 10 / T8 spent it: the router
/// catalog reads each definition through [`CascadeDefs::read_workflow`] like every other
/// composing surface, so the three arms below are the whole list. The datum that retired
/// the row is re-driven at
/// `the_catalog_hides_a_workflow_the_resolved_definition_suppresses`.
const ALLOWED: &[(&str, &str)] = &[
    ("pack.rs", "*"),
    ("start.rs", "read_workflow"),
    ("start.rs", "resolved_workflow"),
];

/// Every production workflow read in `crates/cli/src` goes through the one resolver.
///
/// The four surfaces this fix unified were not written carelessly — each was a local,
/// obvious `pack.read(Workflows, …)`, and the next one would be too. So the property is
/// checked where membership is decided rather than by the list of sites that happened to
/// exist: a new direct read reddens here, naming itself.
#[test]
fn every_production_workflow_read_goes_through_the_shared_resolver() {
    const KIND: &str = "PackResourceKind::Workflows";
    let root = cli_src();
    let mut offenders = Vec::new();
    let mut inspected = 0usize;

    for path in rust_source::rust_files(&root) {
        let body = fs::read_to_string(&path).expect("read a cli source");
        let code = rust_source::code_only(&body);
        let regions = rust_source::cfg_test_regions(&code);
        let file = path
            .file_name()
            .expect("a source file has a name")
            .to_string_lossy()
            .into_owned();

        for (at, _) in code.match_indices(KIND) {
            if rust_source::is_test_domain(&path, &regions, at) {
                continue;
            }
            let Some(callee) = rust_source::enclosing_callee(&code, at) else {
                continue;
            };
            if !READ_CALLEES.contains(&callee) {
                continue;
            }
            inspected += 1;
            let owner = rust_source::enclosing_fn(&code, at).unwrap_or("<top level>");
            if ALLOWED
                .iter()
                .any(|(f, fun)| *f == file && (*fun == "*" || *fun == owner))
            {
                continue;
            }
            let line = code[..at].lines().count();
            offenders.push(format!("  {file}:{line}: in `{owner}` via `{callee}(…)`"));
        }
    }

    assert!(
        inspected >= 3,
        "the fence must actually find the production workflow reads; it saw only \
         {inspected} — the read discriminator has drifted",
    );
    assert!(
        offenders.is_empty(),
        "a production workflow read must go through the shared cascade resolver \
         (`start::CascadeDefs::read_workflow`, reached by `start::resolved_workflow`) — \
         reading the pack directly is shadow-blind, so the reading door enforces a \
         definition no composing door resolved.\n\
         {} offending read(s):\n{}",
        offenders.len(),
        offenders.join("\n"),
    );
}

/// The pack-only seam is reached from **one** place, and that place is the resolver.
///
/// The byte-read fence above cannot see this: `crate::start::read_workflow(pack, id)`
/// names no `PackResourceKind`, so the call that re-opened the hole at
/// `config::check_anchor_present` was invisible to a kind-keyed scan. The seam's own
/// doc-comment states it is the pack-only read; this makes that statement checkable.
#[test]
fn the_pack_only_workflow_seam_has_one_caller_and_it_is_the_resolver() {
    let root = cli_src();
    let mut offenders = Vec::new();

    for path in rust_source::rust_files(&root) {
        let file = path
            .file_name()
            .expect("a source file has a name")
            .to_string_lossy()
            .into_owned();
        if file == "start.rs" {
            // The resolver's own pack branch lives here, beside the seam it calls.
            continue;
        }
        let body = fs::read_to_string(&path).expect("read a cli source");
        let code = rust_source::code_only(&body);
        let regions = rust_source::cfg_test_regions(&code);
        for (at, _) in code.match_indices("start::read_workflow(") {
            if rust_source::is_test_domain(&path, &regions, at) {
                continue;
            }
            let owner = rust_source::enclosing_fn(&code, at).unwrap_or("<top level>");
            let line = code[..at].lines().count();
            offenders.push(format!("  {file}:{line}: in `{owner}`"));
        }
    }

    assert!(
        offenders.is_empty(),
        "`start::read_workflow` is the **pack-only** read — a caller outside `start.rs` \
         bypasses the cascade and enforces a definition no composing door resolved; call \
         `start::resolved_workflow` instead.\n\
         {} offending caller(s):\n{}",
        offenders.len(),
        offenders.join("\n"),
    );
}

/// A project-layer shadow of the shipped `quick-fix` id that **hides** it: the pack's
/// copy is `selectable: true` with its own `when`, this one declares `selectable: false`
/// plus the `suppressed:` block the pack-load fence requires — including a `door:`, which
/// is what makes the two composing doors refuse it by name.
const HIDING_SHADOW: &str = "\
---
when: the project layer's own selection hint
description: A project-layer shadow that hides the id from the router catalog.
usage: the catalog-membership cell.
creates-task: true
selectable: false
suppressed:
  reason: this shadow is reached through its own door
  expires: never
  door: jigc migrate <path> --as adr
---
{{ include: step:locate }}
{{ include: step:author-commit }}
{{ include: step:finalize }}
";

/// A project-layer shadow of `quick-fix` that changes **only** the `when` selection hint
/// — still `creates-task: true`, still selectable, no `suppressed:` block. The catalog
/// must therefore still list the id, with the project's text rather than the pack's.
const WHEN_REWORDING_SHADOW: &str = "\
---
when: the project layer's own selection hint
description: A project-layer shadow that rewords only the selection hint.
usage: the catalog-text cell.
creates-task: true
allows-create: [{type: adr, as: decision}]
---
{{ include: step:locate }}
{{ include: step:author-commit }}
{{ include: step:finalize }}
";

/// The pack's own `quick-fix` selection hint — the text the catalog printed for a
/// shadowed id before the fix, quoted here so a pack reword reddens this arm rather than
/// letting it pass vacuously.
const PACK_QUICK_FIX_WHEN: &str = "apply a small commit-only fix";

/// `start::selectable_workflows` — the router catalog offers only what the composing
/// doors will bind.
///
/// Driven at HEAD (2026-09-20): over a project shadow of `quick-fix` declaring
/// `selectable: false` + `suppressed.door`, `jigc describe --workflows` reported it
/// *hidden from the router catalog* and `jigc start --workflow quick-fix "probe"` refused
/// `workflow.verb-routed` at exit 1 — while bare `jigc start` listed it anyway, under the
/// **pack's** `when`. The catalog read `pack.list` → `read_pack`, the one production
/// workflow read that never went through the cascade.
#[test]
fn the_catalog_hides_a_workflow_the_resolved_definition_suppresses() {
    let corpus = TrialCorpus::build(State::Fresh);

    // The omitting context first: with no shadow, the id is offered under the pack's own
    // hint — so the assertion below is about the shadow, not about an empty catalog.
    let unshadowed = corpus.jigc_ok(&["start"]);
    assert!(
        unshadowed.contains("- quick-fix — ") && unshadowed.contains(PACK_QUICK_FIX_WHEN),
        "with no project shadow the catalog offers `quick-fix` under the pack's hint:\n\
         {unshadowed}",
    );

    write_project_layer_workflow(&corpus, "quick-fix", HIDING_SHADOW);
    let listed = corpus.jigc_ok(&["start"]);
    assert!(
        !listed.contains("- quick-fix — "),
        "the resolved definition is `selectable: false`, so the catalog must not offer \
         `quick-fix`:\n{listed}",
    );

    // …and the door the catalog would have sent the reader to still refuses it, so the
    // two cannot disagree in either direction.
    let refused = corpus.jigc(&["start", "--workflow", "quick-fix", "probe"]);
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert!(
        !refused.status.success() && printed.contains("workflow.verb-routed"),
        "the composing door refuses the same definition the catalog now withholds; got \
         exit {:?}:\n{printed}",
        refused.status.code(),
    );
}

/// The other half of the same read: a shadow that changes only the `when` is still
/// offered — with the **project's** hint, which is the text the pack's copy does not
/// carry. A catalog that dropped every shadowed id would pass the arm above and fail
/// here.
#[test]
fn the_catalog_prints_the_resolved_definitions_selection_hint() {
    let corpus = TrialCorpus::build(State::Fresh);
    write_project_layer_workflow(&corpus, "quick-fix", WHEN_REWORDING_SHADOW);

    let listed = corpus.jigc_ok(&["start"]);
    assert!(
        listed.contains("- quick-fix — the project layer's own selection hint"),
        "the catalog prints the resolved definition's `when`:\n{listed}",
    );
    assert!(
        !listed.contains(PACK_QUICK_FIX_WHEN),
        "the pack's shadowed hint is gone from the catalog:\n{listed}",
    );
}
