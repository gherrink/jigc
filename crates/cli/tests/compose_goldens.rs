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
//!   * `jigc start --workflow <id> <intent>` for **every** workflow — a
//!     `creates-task: true` member mints (part of its surface), so each arm takes a
//!     fresh corpus copy; a `creates-task: false` member (the router and its kind)
//!     composes with **no mint**, and that composed output — the model-free
//!     selection path, the exact surface the preview refusal routes to — is swept
//!     the same way (the sibling-hunt's finding 5: filtering these by code
//!     structure bypassed [`EXCLUSIONS`] and left them pinned by nothing);
//!   * the **front-door composite surfaces** — bare `jigc start` (orientation),
//!     `jigc start "<intent>"` (the cascade-default router selection), and their
//!     `--format json` variants — all read-only (the router mints nothing);
//!   * `jigc describe` — the whole composite catalog, a memberless surface;
//!   * `jigc doc schema <type> --format json` **and** the human form for **every**
//!     doctype;
//!   * the **AGENT.md render** — the adapter bootstrap file `jigc setup` writes
//!     (`.jigc/AGENT.md`, regenerated whole each run), goldened as the rendered
//!     file bytes rather than an invocation.
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
//! `UPDATE_GOLDENS=1 cargo test -p cli compose_goldens::` (this suite is a module
//! inside the `g_compose` group target since the M47 test-target consolidation, so
//! `--test compose_goldens` names nothing).

use crate::support;

use std::collections::BTreeSet;

use cli::pack::EmbeddedPack;
use engine::packsource::{PackResourceKind, PackSource};

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
    let mut collisions = BTreeSet::new();
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Schemas) {
            let id = id.as_str().to_string();
            if seen.insert(id.clone()) {
                out.push(Member {
                    pack: pack_name,
                    id,
                });
            } else {
                collisions.insert(id);
            }
        }
    }
    // The dedup-site collision fence (confidence-audit minor item 9): first-wins is
    // only honest while every collision it resolves is DECLARED here. `commit` is the
    // one known cross-pack doctype collision (both packs ship it; dev wins, and the
    // two schemas differ in bytes — pinning.md §1 records the fact); any other
    // collision means a member's golden would silently record only the winner.
    let declared: BTreeSet<String> = [String::from("commit")].into();
    assert_eq!(
        collisions, declared,
        "the cross-pack doctype collision set drifted from the declared set — a new \
         collision is being resolved first-wins silently; declare it here (with which \
         pack wins) or rename the doctype",
    );
    out
}

/// Every workflow the composite exposes, tagged with its origin pack — **the whole
/// set, undivided**: no `creates-task` (or any other definition-shape) filter may
/// narrow it, because a code-structure skip bypasses [`EXCLUSIONS`] and leaves a
/// surface pinned by nothing while looking swept (the sibling-hunt's finding 5).
/// Deduped first-wins across the precedence order; the two packs' workflow sets do
/// not overlap — and that premise is FENCED at the dedup site (confidence-audit minor
/// item 9): a workflow id shipped by both packs would be swept under its winner only,
/// with the loser's surface silently unpinned, so a collision panics loudly instead of
/// resolving first-wins (unlike doctypes, no workflow collision is declared today).
fn composite_workflows() -> Vec<Member> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Workflows) {
            let id_str = id.as_str().to_string();
            assert!(
                seen.insert(id_str.clone()),
                "workflow id `{id_str}` ships in more than one embedded pack — a \
                 first-wins dedup would pin only the winner's surface; either rename \
                 the workflow or declare the collision here with which pack wins",
            );
            out.push(Member {
                pack: pack_name,
                id: id_str,
            });
        }
    }
    out
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

/// [`check`] for a `--format json` invocation: the capture goes through the harness's
/// parse gate ([`Capture::of_json`] — confidence-audit minor item 5), so polluted
/// stdout bytes fail loudly at capture/regen time instead of becoming a golden.
fn check_json(
    suite: &GoldenSuite,
    pack: &str,
    surface: &str,
    member: &str,
    state: State,
    repo: &std::path::Path,
    out: &std::process::Output,
) {
    let capture = Capture::of_json(out, repo);
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
        if is_excluded("workflow-preview", &wf.id, state) {
            continue;
        }
        let out = corpus.jigc(&["workflow", &wf.id, "--preview"]);
        check(
            &suite,
            wf.pack,
            "workflow-preview",
            &wf.id,
            state,
            &corpus.repo(),
            &out,
        );
    }

    // 2. `jigc start --workflow <id>` — every workflow, a fresh copy per arm. A
    //    `creates-task: true` member mints (the mutation the copy isolates); a
    //    `creates-task: false` member composes with no mint — the surface the
    //    preview refusal routes to, swept here rather than filtered out.
    for wf in &workflows {
        if is_excluded("start", &wf.id, state) {
            continue;
        }
        let arm = corpus.copy_state();
        let out = arm.jigc(&["start", "--workflow", &wf.id, INTENT]);
        check(&suite, wf.pack, "start", &wf.id, state, &arm.repo(), &out);
    }

    // 3. The front-door composite surfaces — bare orientation, the cascade-default
    //    router selection, and their `--format json` variants. All read-only: the
    //    orientation composes nothing and the default workflow (the router) mints
    //    nothing, so they run against the built corpus directly. Memberless per
    //    form, so each files under its own surface name, like `describe`.
    let front_door: &[(&str, &[&str])] = &[
        ("start-orient", &["start"]),
        ("start-orient-json", &["start", "--format", "json"]),
        ("start-intent", &["start", INTENT]),
        ("start-intent-json", &["start", INTENT, "--format", "json"]),
    ];
    for (surface, args) in front_door {
        if is_excluded(surface, surface, state) {
            continue;
        }
        let out = corpus.jigc(args);
        // The `-json` variants take the parse-gated capture (minor item 5).
        let check_fn = if surface.ends_with("-json") {
            check_json
        } else {
            check
        };
        check_fn(
            &suite,
            COMPOSITE,
            surface,
            surface,
            state,
            &corpus.repo(),
            &out,
        );
    }

    // 4. The AGENT.md render — the adapter bootstrap file `jigc setup` writes whole
    //    (`.jigc/AGENT.md`); the golden is the rendered file bytes, not an
    //    invocation, so it takes the file-render capture.
    if !is_excluded("agent-md", "agent-md", state) {
        let body = support::trial_corpus::read(&corpus.repo(), ".jigc/AGENT.md");
        let capture = Capture::of_file(&body, &corpus.repo());
        suite.check(
            &GoldenKey {
                pack: COMPOSITE,
                surface: "agent-md",
                member: "agent-md",
                state: state.name(),
            },
            &capture,
        );
    }

    // 5. `jigc describe` — the whole composite catalog, a memberless surface.
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

    // 6. `jigc doc schema <type>` — json + human, every doctype.
    for dt in &doctypes {
        if !is_excluded("doc-schema-json", &dt.id, state) {
            let out = corpus.jigc(&["doc", "schema", &dt.id, "--format", "json"]);
            check_json(
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
