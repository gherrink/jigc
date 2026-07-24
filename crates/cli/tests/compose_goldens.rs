//! The **compose-golden sweep** — the first full generation
//! ([pinning.md](../../../implementation/pinning.md) §1).
//!
//! **Claim it pins:** the composed surface — everything `jigc start` /
//! `jigc workflow <id> --preview` / `jigc describe` / `jigc doc schema` print —
//! changes only when someone *means* it to, and a pack edit's blast radius is a
//! reviewable diff rather than an invisible propagation. Composition is deterministic
//! by core invariant (same resolved cascade in → same workflow out), so the whole
//! surface is snapshottable.
//!
//! **Enumeration comes from the registries, never a hand list** (pinning.md §1 — the
//! axis principle; *a grep is not a fence*). The workflow and doctype sets are read
//! at runtime from the two embedded packs, deduped in precedence order so a member
//! shipped by both packs is recorded under its winner (`commit` ships in both; dev
//! wins) — the golden doubles as the record of which one won. A workflow added to any
//! pack, or a whole new pack composed in, joins this sweep with zero edits here.
//!
//! **Surfaces swept per member** (pinning.md §1):
//!   * `jigc workflow <id> --preview` for **every** workflow — the mint-free compose
//!     (M44), so it is read-only and runs against the built corpus directly;
//!   * `jigc start --workflow <id> <intent>` for the **`creates-task`** set, where
//!     minting is part of the surface — each arm mutates its corpus (mints a task
//!     dir), so a fresh copy is taken per arm;
//!   * `jigc describe` — the whole composite catalog, a memberless surface;
//!   * `jigc doc schema <type> --format json` **and** the human form for **every**
//!     doctype.
//!
//! **Each surface × every fixture state** ([`State::ALL`]) by default — an exclusion
//! is an entry in [`EXCLUSIONS`] with a stated reason (executable metadata), never a
//! pre-test judgment. The all×all sweep is consciously ~6:1 redundant (`doc schema`
//! and `describe` are byte-identical across states; only two previews vary) — the
//! **invariance is itself worth pinning**: a `doc schema` that starts varying by
//! corpus state is a finding no narrower sweep would notice.
//!
//! **A capture is the whole invocation** — stdout, stderr **and** the exit code — and
//! the only normalization is the repo path → `<REPO>` (the harness owns both). Regen
//! is one step, refused under CI:
//! `UPDATE_GOLDENS=1 cargo test -p cli --test compose_goldens`.

mod support;

use std::collections::BTreeSet;

use cli::pack::EmbeddedPack;
use engine::compose::load_workflow_def;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

use support::goldens::{Capture, GoldenKey, GoldenSuite, update_mode};
use support::trial_corpus::{State, TrialCorpus};

/// The fixed intent every `start` arm mints from. Fixed, so the task id it slugs to
/// is deterministic and the golden is stable; distinct from every fixture-built
/// task's intent, so it never collides with a state's own tasks.
const INTENT: &str = "sweep the compose surface";

/// The pack label a memberless composite surface (`describe`) files under — it
/// composes both packs, so it belongs to neither alone.
const COMPOSITE: &str = "composite";

/// A single (surface, member, state) triple deliberately left unswept, with the
/// reason it cannot produce a stable golden. **Executable metadata, not a pre-test
/// judgment** (pinning.md §1): a genuine exclusion is a combination whose output is
/// non-deterministic (a leaked absolute non-repo path, a stamped date), never one
/// that merely errors — an error is a valid, stable golden (exit code + stderr).
///
/// An empty state (`""`) matches every state. The first full generation found none:
/// every swept combination is deterministic, so this list stands empty as the record
/// of that fact rather than the assumption of it.
struct Exclusion {
    surface: &'static str,
    member: &'static str,
    state: &'static str,
    #[allow(dead_code)] // The reason is documentation-in-code; it is never read.
    reason: &'static str,
}

const EXCLUSIONS: &[Exclusion] = &[];

fn is_excluded(surface: &str, member: &str, state: State) -> bool {
    EXCLUSIONS.iter().any(|e| {
        e.surface == surface
            && e.member == member
            && (e.state.is_empty() || e.state == state.name())
    })
}

/// One swept member: its origin pack and its id.
struct Member {
    pack: &'static str,
    id: String,
}

/// A swept workflow — a [`Member`] plus whether `start` mints from it (the
/// `creates-task` filter that decides the `start` arm).
struct WorkflowMember {
    member: Member,
    creates_task: bool,
}

/// The two embedded packs, in precedence order (dev wins) — built the CWD-free way
/// ([`EmbeddedPack::new`] / [`EmbeddedPack::methodology`] read no ambient state),
/// never `make_pack()`, which resolves against the process CWD.
fn embedded_packs() -> Vec<(&'static str, EmbeddedPack)> {
    vec![
        ("dev", EmbeddedPack::new()),
        ("methodology", EmbeddedPack::methodology()),
    ]
}

/// Every doctype the composite exposes, tagged with its origin pack. Deduped
/// first-wins across the precedence order, so `commit` (shipped by both packs) is
/// recorded under `dev`, its winner. Schema ids are doctype types (`id == type` for
/// every shipped schema), so the id is the `doc schema <type>` argument directly.
fn composite_doctypes() -> Vec<Member> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Schemas) {
            let id = id.as_str().to_string();
            if seen.insert(id.clone()) {
                out.push(Member {
                    pack: pack_name,
                    id,
                });
            }
        }
    }
    out
}

/// Every workflow the composite exposes, tagged with its origin pack and its
/// `creates-task` flag (read from the workflow definition, never a hand list).
/// Deduped first-wins across the precedence order; the two packs' workflow sets do
/// not overlap, so every id resolves to exactly one pack.
fn composite_workflows() -> Vec<WorkflowMember> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Workflows) {
            let id_str = id.as_str().to_string();
            if !seen.insert(id_str.clone()) {
                continue;
            }
            let creates_task = workflow_creates_task(&pack, &id);
            out.push(WorkflowMember {
                member: Member {
                    pack: pack_name,
                    id: id_str,
                },
                creates_task,
            });
        }
    }
    out
}

/// Whether a workflow mints a task, read from its own definition through the
/// production loader — the same `creates_task` the binary composes on.
fn workflow_creates_task(pack: &EmbeddedPack, id: &ResourceId) -> bool {
    let bytes = pack
        .read(PackResourceKind::Workflows, id)
        .unwrap_or_else(|e| panic!("read workflow `{}`: {e}", id.as_str()));
    load_workflow_def(&bytes)
        .unwrap_or_else(|f| panic!("load workflow `{}`: {f:?}", id.as_str()))
        .creates_task
}

/// The production golden suite, rooted at this crate's `tests/goldens`, with regen
/// derived from the two environment variables (and refused under CI).
fn suite() -> GoldenSuite {
    GoldenSuite::new(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens"),
        update_mode(
            std::env::var("UPDATE_GOLDENS").ok().as_deref(),
            std::env::var("CI").ok().as_deref(),
        ),
    )
}

/// Capture `out` normalized under `repo`, and check it against `key`'s golden.
fn check(
    suite: &GoldenSuite,
    pack: &str,
    surface: &str,
    member: &str,
    state: State,
    repo: &std::path::Path,
    out: &std::process::Output,
) {
    let capture = Capture::of(out, repo);
    suite.check(
        &GoldenKey {
            pack,
            surface,
            member,
            state: state.name(),
        },
        &capture,
    );
}

/// Sweep every surface × member for one fixture state against the golden tree.
///
/// The built corpus is used directly for the **read-only** surfaces (`workflow
/// --preview`, `describe`, `doc schema`); each **mutating** `start` arm takes its own
/// [`TrialCorpus::copy_state`] so the mint of one arm never colours the next
/// (single-active-task would otherwise serialize them into an order-dependent result).
fn sweep(state: State) {
    let suite = suite();
    let corpus = TrialCorpus::build(state);
    let workflows = composite_workflows();
    let doctypes = composite_doctypes();

    // 1. `jigc workflow <id> --preview` — mint-free, read-only, every workflow.
    for wf in &workflows {
        if is_excluded("workflow-preview", &wf.member.id, state) {
            continue;
        }
        let out = corpus.jigc(&["workflow", &wf.member.id, "--preview"]);
        check(
            &suite,
            wf.member.pack,
            "workflow-preview",
            &wf.member.id,
            state,
            &corpus.repo(),
            &out,
        );
    }

    // 2. `jigc start --workflow <id>` — the `creates-task` set, a fresh copy per arm.
    for wf in workflows.iter().filter(|w| w.creates_task) {
        if is_excluded("start", &wf.member.id, state) {
            continue;
        }
        let arm = corpus.copy_state();
        let out = arm.jigc(&["start", "--workflow", &wf.member.id, INTENT]);
        check(
            &suite,
            wf.member.pack,
            "start",
            &wf.member.id,
            state,
            &arm.repo(),
            &out,
        );
    }

    // 3. `jigc describe` — the whole composite catalog, a memberless surface.
    if !is_excluded("describe", "describe", state) {
        let out = corpus.jigc(&["describe"]);
        check(
            &suite,
            COMPOSITE,
            "describe",
            "describe",
            state,
            &corpus.repo(),
            &out,
        );
    }

    // 4. `jigc doc schema <type>` — json + human, every doctype.
    for dt in &doctypes {
        if !is_excluded("doc-schema-json", &dt.id, state) {
            let out = corpus.jigc(&["doc", "schema", &dt.id, "--format", "json"]);
            check(
                &suite,
                dt.pack,
                "doc-schema-json",
                &dt.id,
                state,
                &corpus.repo(),
                &out,
            );
        }
        if !is_excluded("doc-schema", &dt.id, state) {
            let out = corpus.jigc(&["doc", "schema", &dt.id]);
            check(
                &suite,
                dt.pack,
                "doc-schema",
                &dt.id,
                state,
                &corpus.repo(),
                &out,
            );
        }
    }
}

// One `#[test]` per fixture state, so the six states sweep in parallel (each builds
// its own corpus once, then runs every surface against it — pinning.md §1 budgets
// this as an ordinary integration suite, ~7 s wall across six parallel tests).

#[test]
fn sweep_fresh() {
    sweep(State::Fresh);
}

#[test]
fn sweep_committed_singletons() {
    sweep(State::CommittedSingletons);
}

#[test]
fn sweep_migrated() {
    sweep(State::Migrated);
}

#[test]
fn sweep_refs_post_hoc() {
    sweep(State::RefsPostHoc);
}

#[test]
fn sweep_chatty_hooks() {
    sweep(State::ChattyHooks);
}

#[test]
fn sweep_vendored() {
    sweep(State::Vendored);
}
