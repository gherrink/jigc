//! **`jigc doc show` over a doc that is committed somewhere else** (M53 — the pre-v1
//! usability batch, row 4 / the M52 per-axis review's `(7, A7-F3)`).
//!
//! # What was broken
//!
//! `engine::store::read_slice` resolves one path and reads it. When a home moves — a
//! `docs-root` / `placement-root` re-point, or a schema bump that relocated the doctype —
//! the document is still committed, still on disk, and `jigc doc list` still calls it
//! `managed`, while the read resolves the *new* home, gets `ENOENT`, and offers the three
//! exits `read_slice` was written for: *create the referenced doc*, *fix the reference*,
//! *read it with `--task`*. None of them is the repair, and `jigc validate` names the
//! repair one command later. The review drove exactly that, on both home kinds:
//!
//! > `doc show adr:use-sqlite` → 1 `store.not-found` @ `docs/adrs/use-sqlite.md` … route
//! > names only *create / fix the reference / read it with `--task`*
//!
//! # The fix, and why the route is the sweep's rather than a new opinion
//!
//! The reroute sits at the **verb boundary** — the same seam, and the same rule, as the
//! shipped `store.unparseable` → adoption-route arm beside it: the read verb asks the two
//! walks that already own *where a managed document lives*, in the order `jigc validate`
//! asks them, and borrows their answer. It invents no third opinion, mints no code, and
//! moves no key.
//!
//! # The class this iterates: **every way a committed doc can be somewhere else**
//!
//! Not the reported cell. There are exactly two mechanisms that strand a document, and
//! they have two different repairs, which is why a route naming one of them unconditionally
//! would be a *misleading* route rather than a useless one:
//!
//! 1. a **recorded prior home** — a schema bump moved the doctype's home and the corpus is
//!    unmigrated (`cli::orphan::prior_home_instances`; the sweep says
//!    `schema-conformance.schema-version-current` → `jigc migrate-corpus`);
//! 2. a **self-discovered strand** — a root knob's re-point left the document behind
//!    (`cli::orphan::orphaned_docs`; the sweep says `file-state.orphaned-doc` → move it /
//!    re-point the knob / `jigc unmanage`).
//!
//! and (2) has **two home kinds**, `location:` and `placement:`, which the reconciler
//! specifically recorded as both affected. So: three strand cells, plus the control that
//! makes the fix a narrowing rather than a blanket swap — a doc that genuinely does not
//! exist must keep the shipped route, byte for byte.
//!
//! Each strand cell asserts the **cross-door agreement** rather than a hand-written
//! sentence: the verb the read names is the verb `jigc validate` names over the same
//! corpus in the same second. That is the property `(7, A7-F3)` reports as absent, and it
//! is checkable without this file restating either route's wording.

use crate::support;

use support::trial_corpus::{State, TrialCorpus};

/// Write the project-layer knob **by hand**, bypassing `jigc config set`.
///
/// Deliberate, and it is the state the M49 census row names: `config set` carries a
/// detect-route-**move** floor that relocates the committed docs it would otherwise
/// strand, so driving the knob through the verb produces a corpus with nothing stranded in
/// it. A hand-written layer is how an adopter reaches this state (an edited
/// `manifest.yaml`, a merge, a config committed from another checkout), and it is the
/// state the sweep's own strand arm exists to report.
fn set_root_knob(corpus: &TrialCorpus, knob: &str, value: &str) {
    let config = corpus.repo().join(".jigc").join("config");
    std::fs::create_dir_all(&config).expect("the project config layer");
    std::fs::write(
        config.join("manifest.yaml"),
        format!("scalar:\n  {knob}: {value}\n"),
    )
    .expect("write the project layer knob");
}

/// The `route:` line of the single finding `jigc doc show <addr>` blocked with.
fn show_route(corpus: &TrialCorpus, addr: &str) -> String {
    let out = corpus.jigc(&["doc", "show", addr]);
    assert!(
        !out.status.success(),
        "`jigc doc show {addr}` must block; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let printed = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        printed.contains("store.not-found"),
        "the block is `store.not-found`; got:\n{printed}",
    );
    printed
}

/// Everything `jigc validate` printed over the same corpus — the door that owns the
/// repair, and the one a cell compares the read's route against.
fn sweep(corpus: &TrialCorpus) -> String {
    let out = corpus.jigc(&["validate"]);
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// **Cell 1 — the recorded prior home.** The `changelog` doctype moved from
/// `location: changelog/` to the root `CHANGELOG.md` at schema-version 2, and
/// `changelog.v1.yaml` is the shipped snapshot that records it. A corpus carrying the
/// v1-era document at the v1 home is the unmigrated case, and the repair is
/// `jigc migrate-corpus` — which is what `jigc validate` says, so it is what the read
/// must say.
#[test]
fn a_doc_at_a_recorded_prior_home_is_routed_at_the_migration() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);

    // Put the committed singleton back at its v1 home, in its v1 shape.
    let prior = corpus.repo().join("docs/changelog/changelog.md");
    std::fs::create_dir_all(prior.parent().expect("a parent")).expect("the v1 home directory");
    corpus.git(&["mv", "CHANGELOG.md", "docs/changelog/changelog.md"]);
    let body = std::fs::read_to_string(&prior)
        .expect("the moved changelog")
        .replace("schema-version: 2", "schema-version: 1")
        .replacen("# Changelog", "# changelog", 1);
    std::fs::write(&prior, body).expect("write the v1-era shape");
    corpus.git(&["add", "-A"]);
    corpus.git(&[
        "-c",
        "core.hooksPath=/dev/null",
        "commit",
        "-q",
        "-m",
        "a v1-era changelog at its prior home",
    ]);

    let swept = sweep(&corpus);
    assert!(
        swept.contains("schema-conformance.schema-version-current")
            && swept.contains("jigc migrate-corpus"),
        "precondition: the sweep routes this corpus at the migration; got:\n{swept}",
    );

    let route = show_route(&corpus, "changelog");
    assert!(
        route.contains("docs/changelog/changelog.md"),
        "the read names where the doc actually is; got:\n{route}",
    );
    assert!(
        route.contains("jigc migrate-corpus"),
        "and routes at the verb the sweep routes at, not at `create the referenced doc`; \
         got:\n{route}",
    );
    assert!(
        !route.contains("create the referenced doc"),
        "the shipped absent-doc route must not stand over a doc that is committed; \
         got:\n{route}",
    );
}

/// **Cell 2 — a `location:` strand.** A `docs-root` re-point the move floor never ran for
/// leaves every located doctype's committed instance behind. The sweep's answer is
/// `file-state.orphaned-doc`, whose repair is move-it / re-point-the-knob / `unmanage` —
/// not a migration, which is why the read cannot route unconditionally.
#[test]
fn a_located_doc_stranded_by_a_docs_root_repoint_is_routed_at_the_strand_repair() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    set_root_knob(&corpus, "docs-root", "docs2");

    let swept = sweep(&corpus);
    assert!(
        swept.contains("file-state.orphaned-doc")
            && swept.contains("docs/research/context-loss.md"),
        "precondition: the sweep reports the strand; got:\n{swept}",
    );

    let route = show_route(&corpus, "research:context-loss");
    assert!(
        route.contains("docs/research/context-loss.md"),
        "the read names where the doc actually is; got:\n{route}",
    );
    assert!(
        route.contains("jigc unmanage") && route.contains("re-point"),
        "and offers the strand repair, not the migration; got:\n{route}",
    );
    assert!(
        !route.contains("jigc migrate-corpus"),
        "a knob re-point is not a schema bump — routing at the migration would be a \
         misleading route, not a useless one; got:\n{route}",
    );
}

/// **Cell 3 — a `placement:` strand**, the home kind the reconciler drove independently
/// and found affected too (`axis-7.md` → R-R2). Same mechanism, different home shape: the
/// identity is the fixed `<ty>:<ty>` singleton wherever the file sits, which is the rule
/// the reroute derives it by.
#[test]
fn a_placement_doc_stranded_by_a_placement_root_repoint_is_routed_the_same_way() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    set_root_knob(&corpus, "placement-root", "docs3");

    let swept = sweep(&corpus);
    assert!(
        swept.contains("file-state.orphaned-doc") && swept.contains("docs/roadmap.md"),
        "precondition: the sweep reports the placement strand; got:\n{swept}",
    );

    let route = show_route(&corpus, "roadmap");
    assert!(
        route.contains("docs/roadmap.md"),
        "the read names where the singleton actually is; got:\n{route}",
    );
    assert!(
        route.contains("jigc unmanage") && route.contains("re-point"),
        "and offers the strand repair; got:\n{route}",
    );
}

/// **The control — the fix is a narrowing, not a blanket swap.** Over the *same* relocated
/// corpus, an address that names no committed document at all keeps the shipped route
/// verbatim: for a doc that genuinely does not exist, *create it / fix the reference / read
/// it with `--task`* is the honest answer, and replacing it would be the mirror-image lie
/// the `store.unparseable` arm beside it is careful not to tell.
#[test]
fn an_address_that_names_nothing_keeps_the_shipped_route() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    set_root_knob(&corpus, "docs-root", "docs2");

    let route = show_route(&corpus, "research:no-such-thing");
    assert!(
        route.contains("create the referenced doc, or fix the reference to an existing one"),
        "the absent-doc route stands unchanged; got:\n{route}",
    );
    assert!(
        route.contains("--task <task-id>"),
        "including its staged-read half; got:\n{route}",
    );
    assert!(
        !route.contains("is committed at"),
        "nothing may claim this doc is committed somewhere; got:\n{route}",
    );
}
