//! M52 Increment 7 / T5 — **a declared home jigc committed into is now empty, and the store
//! surface says so.**
//!
//! The condition ([settle-record](../../../completions/artifacts/M52/settle-record.md) → D7.1
//! as amended by §5): at each **exact declared path** a resolved doctype homes its one
//! instance at — a `placement:` doctype's resolved `placement.file`, and a `location:` +
//! `singleton: true` doctype's `<location><ty>.md` — the repository's history touches the
//! path while no committed instance is there.
//!
//! **The baseline it repairs, driven at `1932c00f` and again here before the code:**
//! `git mv CHANGELOG.md HISTORY.md` + commit — one ordinary human act — then
//! `find .jigc/state -mindepth 1 -delete`, which is the **fresh-clone shape** (`.jigc/state/`
//! is gitignored, so it is what every clone and every CI runner has). `jigc validate` printed
//! *"4 finding(s) — report-only at store scope (exit 0)"*, named neither `CHANGELOG.md` nor
//! `HISTORY.md` on any line, and `jigc doc list` dropped `changelog` entirely. The only thing
//! between an adopter and a green CI over a lost managed document was a gitignored cache that
//! does not survive `git clone`.
//!
//! **The kind of set this suite iterates: the class's defining case-set** — the four states an
//! exact declared home can be in: *filled* · *vacated* · *never written to* · *moved with its
//! knob*. A fifth cell, the `location:` + `singleton: true` home, is unreachable on the shipped
//! packs (every shipped `singleton: true` doctype is also `placement:`), so it is exercised on a
//! manufactured schema in `cli::orphan`'s own unit tests rather than claimed here. The
//! *derivation* of which doctypes name an exact path at all lives with the producer
//! (`cli::orphan::fixed_identity_homes`) and is pinned there; what this suite lists are the
//! homes a **fixture state** happens to fill, which is a property of the fixture.
//!
//! **The exit is T6's, and arms 6-8 are it.** T5 minted the finding and asserted no exit in
//! either direction; T6 makes the condition a `cli::render::STORE_EXIT_FLIPS` member, so from
//! arm 6 down the set iterated is a **code-side registry** — the member is *found in* that
//! table and every string those arms assert (the cause a closing line must name, the witness
//! they drive) is read back off it, which makes membership itself the assertion.
//!
//! **The route's named exits are driven, not read** (M46's PT-1 rule — a route that, followed
//! exactly, changes nothing is the defect). Every backticked command the emitted route carries
//! is lifted out of the printed bytes and executed verbatim, and the finding is re-read after
//! each. The one verb the route names as *not* an exit — `jigc unmanage`, which drops a
//! file-state baseline and leaves the home declared — is driven too, so the sentence saying it
//! does not clear this is a measured fact rather than a claim.
//!
//! **Arm 12 iterates a second axis, and it is a code-side registry too** (M52 completion audit,
//! fix 6): `cli::orphan::Removal::ALL`, the removal states the emptiness leg admits — the leg
//! reads the **worktree**, so a committed `git rm`, an uncommitted one and a staged one all
//! satisfy it while what repairs them could not differ more. The cells are ⇔-fenced against
//! that enum, each is produced by an ordinary human act, and the route each draws is run
//! verbatim: a query must answer, a repair must repair, and no state's route may carry another
//! state's advice.

use crate::support;

use std::process::Output;

use cli::orphan::Removal;
use cli::render::{STORE_EXIT_FLIPS, StoreExitFlip};
use support::trial_corpus::{State, TrialCorpus};

/// The code under test. Read from the production const so the suite and the producer cannot
/// drift apart on a string.
fn code() -> &'static str {
    cli::orphan::HOME_VACATED_CODE
}

/// The exact declared homes [`State::CommittedSingletons`] leaves **filled** — the controls
/// that make arm 1 a discriminating check rather than one firing at every fixed home it meets.
/// Written down because which homes a *fixture state* fills is a property of the fixture, not
/// of any registry; the derivation of which doctypes name an exact path at all is pinned with
/// the producer.
const FILLED_HOMES: &[&str] = &["VISION.md", "docs/decisions-log.md", "docs/roadmap.md"];

fn printed(out: &Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Whether the report carries a **blocking** `home-vacated` row naming `rel`. The severity
/// token is matched loosely on purpose: the renderer annotates it (`blocking (gates at
/// finalize) · …`) when a gate exists, and whether *this* finding may carry that annotation is
/// its own assertion in arm 1 rather than something smuggled into the predicate.
fn fires_at(text: &str, rel: &str) -> bool {
    text.lines().any(|line| {
        line.contains("blocking") && line.contains(code()) && line.contains(&format!("`{rel}`"))
    })
}

/// The full rendered row naming `rel` — for the assertions that are about the row's own text.
fn row_at(text: &str, rel: &str) -> String {
    text.lines()
        .find(|line| line.contains(code()) && line.contains(&format!("`{rel}`")))
        .unwrap_or_else(|| panic!("no `{}` row naming `{rel}`:\n{text}", code()))
        .to_string()
}

/// The **emitted** route line of the `home-vacated` finding whose message names `rel` — the
/// bytes the surface printed, never a reconstruction.
fn emitted_route(text: &str, rel: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let at = lines
        .iter()
        .position(|line| {
            line.contains(code()) && line.contains(&format!("`{rel}`")) && line.contains("blocking")
        })
        .unwrap_or_else(|| panic!("no `{}` finding naming `{rel}`:\n{text}", code()));
    lines[at..]
        .iter()
        .find_map(|line| line.trim_start().strip_prefix("route: "))
        .unwrap_or_else(|| {
            panic!(
                "the `{}` finding at `{rel}` carries no route:\n{text}",
                code()
            )
        })
        .to_string()
}

/// Every backticked span of an emitted route — the commands it tells the reader to run.
fn backticked(route: &str) -> Vec<String> {
    route
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// The corpus with `CHANGELOG.md` renamed out from under `changelog`'s declared home and the
/// rename committed — the advocate's driven cell, in the fresh-clone shape.
fn vacated_changelog() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.git(&["mv", "CHANGELOG.md", "HISTORY.md"]);
    // The installed `pre-commit` backstop is stepped over, not defeated: since M53 (the
    // pre-v1 usability batch, row 1) its M35 blocking arm reaches the **placement** family,
    // so this bare `git mv` is exactly what it refuses. That is a different claim than this
    // fixture's, whose subject presupposes the move landed.
    corpus.git(&[
        "-c",
        "core.hooksPath=/dev/null",
        "commit",
        "-q",
        "-m",
        "rename the changelog",
    ]);
    corpus.fresh_clone_shape();
    corpus
}

/// **Arm 1 — the vacated home is named, blocking, and the filled homes are not.**
#[test]
fn a_declared_home_the_repository_committed_into_and_then_emptied_is_named() {
    let corpus = vacated_changelog();
    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);

    assert!(
        fires_at(&text, "CHANGELOG.md"),
        "`changelog` still homes one document at `CHANGELOG.md`, the history touches that \
         path, and nothing is there — the store surface must name it, blocking; got:\n{text}",
    );
    // **The row must not claim a gate it has none of** (M42's condition). This code is minted
    // by the store sweep alone — the condition is that *nothing is at the path*, so there is no
    // document for a task to address and no task-scope door that could raise it — which is why
    // it is a `cli::render`'s `GATES_NOWHERE` member. Without that row the label would be
    // granted on the `schema-conformance.*` prefix and tell the reader `finalize` stops here.
    let row = row_at(&text, "CHANGELOG.md");
    assert!(
        !row.contains("gates at finalize"),
        "no task-scope door can reach this condition, so the row must not label itself as \
         gating at finalize; got:\n{row}",
    );

    for filled in FILLED_HOMES {
        assert!(
            !fires_at(&text, filled),
            "`{filled}` holds its document — a check that fires at every fixed home it meets \
             is not this check; got:\n{text}",
        );
    }
}

/// **Arm 2 — a home whose file never existed stays silent.** The shipped composition homes
/// `deferral-ledger` at `docs/deferral-ledger.md`, which this state never creates: an empty
/// home is the ordinary state of a corpus that has not needed that doctype yet, and only the
/// history leg separates it from a vacated one.
#[test]
fn a_declared_home_the_repository_never_committed_into_stays_silent() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.fresh_clone_shape();
    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);

    assert!(
        !text.contains(code()),
        "every declared home of this corpus is either filled or was never written to — \
         `{}` must not fire anywhere; got:\n{text}",
        code(),
    );
    let never = corpus.git(&[
        "log",
        "HEAD",
        "-1",
        "--format=%H",
        "--",
        "docs/deferral-ledger.md",
    ]);
    assert!(
        never.trim().is_empty(),
        "the control only controls while `docs/deferral-ledger.md` genuinely has no history; \
         got: {never:?}",
    );
}

/// **Arm 3 — an emptied collection directory stays silent.** `adr` is a `location:` doctype
/// with a per-author slug, so it has no exact declared path at all; a corpus that retired its
/// last ADR has done nothing wrong, and reading *"each resolved doctype home"* as a directory
/// would flip `validate` red over it (§5 — the amendment this arm pins).
#[test]
fn an_emptied_collection_directory_is_not_a_declared_home() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let adr = corpus.repo().join("docs").join("decisions");
    std::fs::create_dir_all(&adr).expect("create the adr collection dir");
    std::fs::write(adr.join("first.md"), "# First\n").expect("write an adr");
    corpus.git(&["add", "docs/decisions/first.md"]);
    corpus.git(&["commit", "-q", "-m", "an adr"]);
    corpus.git(&["rm", "-q", "docs/decisions/first.md"]);
    corpus.git(&["commit", "-q", "-m", "retire the adr"]);
    corpus.fresh_clone_shape();

    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);
    assert!(
        !text.contains(code()),
        "`docs/decisions/` has committed history and holds nothing, and it is still not a \
         declared home — a slugged doctype names no exact path; got:\n{text}",
    );
}

/// **Arm 4 — the subject is the *resolved* home, not the declared one.** Re-pointing
/// `placement-root` moves `roadmap`'s home and (through the shipped detect-route-move floor)
/// the documents with it: the vacated *declared* path `docs/roadmap.md` has history and holds
/// nothing, and it must stay silent because it is no longer where the doctype homes.
#[test]
fn a_rerooted_home_whose_file_is_present_stays_silent() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.jigc_ok(&["config", "set", "placement-root", "papers"]);
    corpus.git(&["commit", "-q", "-m", "re-point placement-root"]);
    corpus.fresh_clone_shape();

    assert!(
        corpus.repo().join("papers").join("roadmap.md").is_file(),
        "the re-point must have carried the document to the new home, else this arm is \
         testing an absent file rather than a moved home",
    );
    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);
    assert!(
        !text.contains(code()),
        "`docs/roadmap.md` has history and holds nothing, but `roadmap` now homes at \
         `papers/roadmap.md` and that file is there — the check reads the resolved home; \
         got:\n{text}",
    );
}

/// **Arm 5 — every command the emitted route names is executed verbatim, and the finding
/// clears.** The route's bytes are the contract: a test that rebuilt the repair in test code
/// could pass while the route an agent actually runs is broken.
#[test]
fn the_emitted_routes_named_exits_each_clear_the_finding() {
    let corpus = vacated_changelog();
    let first = printed(&corpus.jigc(&["validate"]));
    let route = emitted_route(&first, "CHANGELOG.md");
    let commands = backticked(&route);

    // The locator the route names, run verbatim: it must actually name the commit that
    // removed the document, else the route sends the reader nowhere.
    let locator = commands
        .iter()
        .find(|c| c.starts_with("git "))
        .unwrap_or_else(|| panic!("the route names no git locator: {route}"));
    let argv: Vec<&str> = locator.split_whitespace().skip(1).collect();
    let found = corpus.git(&argv);
    assert!(
        !found.trim().is_empty(),
        "the route's locator `{locator}` must name the commit that removed the document; \
         it printed nothing",
    );

    // The verb the route names as *not* an exit, lifted out of the emitted bytes and run
    // verbatim: driven, so the sentence saying it clears nothing is measured.
    let non_exit = commands
        .iter()
        .find(|c| c.starts_with("jigc unmanage"))
        .unwrap_or_else(|| panic!("the route names no non-exit to drive: {route}"));
    let argv: Vec<&str> = non_exit.split_whitespace().skip(1).collect();
    corpus.jigc(&argv);
    assert!(
        fires_at(&printed(&corpus.jigc(&["validate"])), "CHANGELOG.md"),
        "the route says `jigc unmanage` does not clear this — if it did, the route would be \
         naming the wrong non-exit",
    );

    // The restore the route names, then the jigc verb it names, each driven.
    corpus.git(&["mv", "HISTORY.md", "CHANGELOG.md"]);
    // Same step-over, same reason (M53, row 1): the restore is itself a bare `git mv` of a
    // placement singleton, and the claim under test is the sweep's, not the hook's.
    corpus.git(&[
        "-c",
        "core.hooksPath=/dev/null",
        "commit",
        "-q",
        "-m",
        "restore the changelog",
    ]);
    let restored = printed(&corpus.jigc(&["validate"]));
    assert!(
        !fires_at(&restored, "CHANGELOG.md"),
        "restoring the document at the declared home is the route's exit — it must clear the \
         finding; got:\n{restored}",
    );

    let ingest = commands
        .iter()
        .find(|c| c.starts_with("jigc ") && *c != non_exit)
        .unwrap_or_else(|| panic!("the route names no jigc verb to run: {route}"));
    let argv: Vec<&str> = ingest.split_whitespace().skip(1).collect();
    let out = corpus.jigc(&argv);
    assert!(
        out.status.success(),
        "the route's jigc verb must run cleanly on the state the route names it for; got:\n{}",
        printed(&out),
    );
    let after = printed(&corpus.jigc(&["validate"]));
    assert!(
        !fires_at(&after, "CHANGELOG.md"),
        "the route's second step must leave the finding cleared, not resurrect it; got:\n{after}",
    );
}

// ---------------------------------------------------------------------------------------
// T6 — the exit
// ---------------------------------------------------------------------------------------

/// The [`StoreExitFlip`] member this condition is, read off the production table rather than
/// hand-spelled: the cause a closing line must name and the witness it is driven through are
/// the member's own, so nothing below can drift from what the renderer prints.
fn vacated_flip() -> &'static StoreExitFlip {
    STORE_EXIT_FLIPS
        .iter()
        .find(|flip| flip.id == "home-vacated")
        .expect(
            "`home-vacated` must be a member of `cli::render::STORE_EXIT_FLIPS` — the store \
             sweep's exit rule *is* that table, so a blocking store finding that is not a \
             member is reported at exit 0 and an adopter's CI stays green over a managed \
             document the repository has lost",
        )
}

/// **Arm 6 — the sweep exits non-zero, and the closing line names this condition.**
///
/// The red this arm was written against, driven on the same fixture at `0556c4f0` (T5's own
/// commit, the finding already shipping): `blocking · schema-conformance.home-vacated — …`
/// on one line and *"5 finding(s) — report-only at store scope (exit 0); each gates nowhere"*
/// on the next, at **exit 0**. A blocking store finding that is not an axis member is a green
/// CI over a lost document — which is the only thing this condition exists to refuse.
#[test]
fn a_vacated_declared_home_flips_the_store_sweeps_exit() {
    let corpus = vacated_changelog();
    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);

    assert!(
        fires_at(&text, "CHANGELOG.md"),
        "the arm only means anything while the condition still fires; got:\n{text}",
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a declared home the repository committed into and then emptied must fail the run \
         an adopter's CI gates on; got:\n{text}",
    );
    assert!(
        !text.contains("report-only at store scope (exit 0)"),
        "the report-only closing line is the sentence that made the base's exit 0 read as a \
         verdict rather than an omission — it must be gone; got:\n{text}",
    );
    assert!(
        text.contains(vacated_flip().cause),
        "the closing line must name which condition flipped the exit ({:?}); got:\n{text}",
        vacated_flip().cause,
    );
}

/// **Arm 7 — the member's witness is the production producer, and its declared
/// trustworthiness is what its own closing line says.**
///
/// The witness rule is `orphaned-instance`'s precedent: a hand-built copy of the finding is a
/// second place for the code, the target form and the route to drift, and the member's whole
/// claim is that they cannot. The trustworthiness rule is the axis's narrowing guard read from
/// the other end — the flag is declared per member, so the flag and the words must agree.
#[test]
fn the_members_witness_is_the_producer_and_its_trailer_matches_its_verdict() {
    let flip = vacated_flip();
    let witness = (flip.witness)();

    assert_eq!(
        witness.code,
        code(),
        "the witness must carry the production code — a hand-built copy is a second place \
         for the identity to drift",
    );
    assert!(
        witness.location.is_some() && witness.route.is_some(),
        "the membership seam refuses to serialize a target-less finding, so a witness \
         without a target and a route is drivable on the agent surface alone; got: {witness:?}",
    );

    assert!(
        !flip.sweep_untrustworthy,
        "this sweep **worked** — it read the schema's declared home, the committed census \
         and the repository's history, and is reporting a standing fact it found; declaring \
         it untrustworthy would state the whole class as one",
    );
    let trailer = (flip.trailer)();
    // The closing line is **one** line for a class whose members are in different removal
    // states, so it may promise nothing that only one of them has. It promised a locator until
    // the M52 completion audit's fix 6 — *"each finding above carries the locator for where it
    // went"* — while an uncommitted removal is a member and no commit removed anything for a
    // locator to name.
    assert!(
        !trailer.contains("locator"),
        "the closing line speaks for every vacated home on the report, and two of the three \
         removal states have no deleting commit at all — it may hand the reader to each \
         finding's own repair, never to a locator it cannot promise; got:\n{trailer}",
    );
    for claim in [
        "could not be trusted",
        "cannot be trusted",
        "not trustworthy",
    ] {
        assert!(
            !trailer.contains(claim),
            "the member declares `sweep_untrustworthy: false`, so its own closing line may \
             not say {claim:?}; got:\n{trailer}",
        );
    }
}

/// **Arm 8 — the position, driven.** Where this member sits in the table is a decision about
/// which closing line the reader is handed when two standing facts about the corpus fire at
/// once, and it is settled here by driving the co-occurrence rather than by asserting an
/// index: the vacated home is a managed document the repository's own history says it had and
/// no longer has, the squatter a file jigc was never handed — loss outranks non-adoption, so
/// this condition precedes it and `foreign-squatter` keeps its stated last place.
///
/// Both findings are on the report either way and each carries its own route, so what the
/// precedence buys is only which sentence closes the report — which is exactly why it is
/// decided on the reader's need rather than on where the row was appended.
#[test]
fn a_vacated_home_closes_the_report_ahead_of_a_never_adopted_file() {
    let squatter = STORE_EXIT_FLIPS
        .iter()
        .find(|flip| flip.id == "foreign-squatter")
        .expect("`foreign-squatter` is the other standing-fact member this one is ordered against");

    let corpus = vacated_changelog();
    // A committed file at a *different* declared home that jigc was never handed. That home
    // has no history of its own, so it is not itself vacated — the two conditions are at two
    // homes, which is the only way they co-occur (a foreign file **at** a vacated home fills
    // the census and the vacated leg goes false).
    std::fs::write(
        corpus.repo().join("docs").join("deferral-ledger.md"),
        "---\ntitle: Deferral ledger\n---\n\n# Deferral ledger\n",
    )
    .expect("plant a foreign file at another declared home");
    corpus.git(&["add", "docs/deferral-ledger.md"]);
    corpus.git(&["commit", "-q", "-m", "plant a never-adopted ledger"]);

    let text = printed(&corpus.jigc(&["validate"]));
    // Both conditions must really be on the report, else the arm discriminates nothing. The
    // squatter is looked for by the **code its own member's witness carries** — its `cause` is
    // the trailer's wording and only one trailer prints, so asking for that here would be
    // asking for the thing under test.
    let squatter_code = (squatter.witness)().code;
    assert!(
        text.lines()
            .any(|line| line.contains(&squatter_code)
                && line.contains("`docs/deferral-ledger.md`")),
        "the never-adopted file must be on this report too; got:\n{text}",
    );
    let closing = text
        .lines()
        .rev()
        .find(|line| line.contains(squatter.cause) || line.contains(vacated_flip().cause))
        .expect("one of the two closes the report");
    assert!(
        closing.contains(vacated_flip().cause),
        "with both firing, the closing line must be the lost document's, not the \
         never-adopted file's; got:\n{closing}",
    );
}

// ---------------------------------------------------------------------------------------
// M52 completion audit, fix 1 — the discriminator: **jigc wrote what the history carries**
// ---------------------------------------------------------------------------------------
//
// The red these arms were written against, driven at `c80b3f8f` on a `bare` rig corpus:
//
//     printf '# Changelog\n\n## 0.1.0\n' > CHANGELOG.md && git add … && git commit
//     git rm -q CHANGELOG.md && git commit
//     jigc setup      # exit 0
//     jigc validate   # exit 1 — blocking · schema-conformance.home-vacated `CHANGELOG.md`
//
// A stock brownfield repository that once had a `CHANGELOG.md`, a `VISION.md`, or any other
// file at a path the shipped packs later declare as a home, was **red at exit 1 the moment
// `jigc setup` ran**, with no exit that clears it: `jigc unmanage <path>` is a no-op there,
// `jigc ingest` classifies nothing, the route's *take the pack out of the composition* is
// unavailable for an embedded doctype, and a project schema shadow moving `placement:` trips
// the freeze assert. Driven, all **five** shipped fixed-identity homes fired at once.
//
// The history leg was `git log HEAD -1 -- <path>` — *any* history at the path — and the
// producer's own doc-comment admitted it: *"It does not claim jigc wrote what the history
// carries."* That is the gap M51's completion-audit HIGH closed one check over, where
// `orphaned-instance` was narrowed to `orphan::Territory` because the `schema-version:` stamp
// is un-namespaced. D7's own words are *a declared home **jigc committed into** that is now
// empty*, and these arms are that predicate.
//
// **The kind of set the two class arms iterate: a derivation stated as one** — every doctype
// either embedded pack declares whose `Schema::projection()` says its identity is `Fixed` and
// whose home has a path, read through `cli::pack::load_pack_schema`, the production loader.
// Never a written list: the producer derives the same set the same way, so a sixth
// fixed-identity doctype joins both sides by being declared.

/// The fixed-identity home set of both embedded packs as `(doctype, repo-relative home)`,
/// address-sorted — the class `cli::orphan::vacated_homes` sweeps, derived here through the
/// production pack loader rather than written down.
fn fixed_identity_homes() -> Vec<(String, String)> {
    use engine::packsource::{PackResourceKind, PackSource};

    let pack = cli::pack::CompositePack::new(vec![
        Box::new(cli::pack::EmbeddedPack::new()),
        Box::new(cli::pack::EmbeddedPack::methodology()),
    ]);
    let mut out: Vec<(String, String)> = Vec::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("a listed schema resource is readable");
        let schema = cli::pack::load_pack_schema(&pack, &bytes)
            .unwrap_or_else(|e| panic!("shipped schema `{}` loads: {e:?}", id.as_str()));
        let projection = schema.projection();
        if projection.identity.kind != engine::schema::IdentityKind::Fixed {
            continue;
        }
        let Some(path) = projection.home.path else {
            continue;
        };
        if out.iter().any(|(ty, _)| ty == &schema.ty) {
            continue;
        }
        out.push((schema.ty.clone(), path));
    }
    out.sort();
    assert!(
        out.len() >= 4,
        "the derived class is {} members — a sweep over a collapsed derivation passes \
         vacuously",
        out.len(),
    );
    out
}

/// Plant `body` at every derived fixed-identity home of `corpus`, commit it, remove all of
/// them, and commit that — the *filled then vacated* history, laid down **before** the corpus
/// is adopted. Returns the class it planted at.
fn plant_then_vacate_every_home(corpus: &TrialCorpus, body: &str) -> Vec<(String, String)> {
    let homes = fixed_identity_homes();
    for (_, home) in &homes {
        let at = corpus.repo().join(home);
        if let Some(parent) = at.parent() {
            std::fs::create_dir_all(parent).expect("create a declared home's parent dir");
        }
        std::fs::write(&at, body).expect("plant a document at a declared home");
    }
    corpus.git(&["add", "-A"]);
    corpus.git(&["commit", "-q", "-m", "the project's own documents"]);
    for (_, home) in &homes {
        corpus.git(&["rm", "-q", home]);
    }
    corpus.git(&["commit", "-q", "-m", "retire them"]);
    for (_, home) in &homes {
        assert!(
            !corpus
                .git(&["log", "HEAD", "-1", "--format=%H", "--", home])
                .trim()
                .is_empty(),
            "the arm only discriminates while `{home}` genuinely has committed history",
        );
    }
    homes
}

/// **Arm 9 — a never-adopted repository's own pre-jigc history at a declared home is not a
/// vacated home, at every member of the class.**
///
/// This is the audit's HIGH. `jigc setup` over a stock brownfield repository that once
/// carried a `CHANGELOG.md` — or any file at a path the shipped packs declare a home at —
/// turned `jigc validate` red at exit 1 with no exit that clears it. The documents jigc is
/// accused of losing were never jigc's: no jigc operation ever wrote at those paths.
#[test]
fn a_never_adopted_repositorys_own_history_at_a_declared_home_is_not_a_vacated_home() {
    let corpus = TrialCorpus::build_never_adopted();
    let homes = plant_then_vacate_every_home(&corpus, "# Notes\n\nthe project's own prose\n");

    corpus.jigc_ok(&["setup"]);
    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);

    for (ty, home) in &homes {
        assert!(
            !fires_at(&text, home),
            "`{home}` has committed history and holds nothing, but nothing jigc wrote was \
             ever there — `{ty}`'s home was never filled *by jigc*, so naming it accuses the \
             adopter of losing a document they never had; got:\n{text}",
        );
    }
    assert_eq!(
        out.status.code(),
        Some(0),
        "a stock brownfield repository must be green the moment it is adopted — a red \
         `jigc validate` with no exit that clears it is a door an adopter cannot walk \
         through; got:\n{text}",
    );
}

/// **Arm 10 — the discriminator is the stamp on the last committed blob, at every member of
/// the class.**
///
/// The complement of arm 9 on the same fixture act: identical history, identical homes, and
/// the only difference is that the blob the history carries at each path bears a
/// `schema-version:` stamp. Every home fires. This is what makes arm 9 a *narrowing* rather
/// than a silencing — the condition still names a home whose last committed document was a
/// managed one.
///
/// **What it still misses, pinned as expected output here rather than left to be found**: a
/// pre-jigc document that happens to carry a `schema-version:` key in its own front matter
/// would be read as jigc's, because the stamp key is un-namespaced. The subject is five
/// **exact declared paths** rather than a tree, and the file must also be *gone*, so the cell
/// is far narrower than the tree-wide one M51's HIGH closed — and the honest fix is the
/// namespaced stamp already deferred with its trigger (`implementation/decisions-pending.md`
/// → the namespaced stamp key), which is a `schema-version` bump this wave does not make.
#[test]
fn a_stamped_last_committed_blob_at_the_same_homes_still_names_every_one() {
    let corpus = TrialCorpus::build_never_adopted();
    let homes =
        plant_then_vacate_every_home(&corpus, "---\nschema-version: 1\n---\n\n# Notes\n\nprose\n");

    corpus.jigc_ok(&["setup"]);
    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);

    for (ty, home) in &homes {
        assert!(
            fires_at(&text, home),
            "`{ty}` homes one document at `{home}`, the last blob the history committed \
             there carried a `schema-version:` stamp, and nothing is there now — the \
             narrowing must not have silenced the condition itself; got:\n{text}",
        );
    }
    assert_eq!(
        out.status.code(),
        Some(1),
        "the condition is a `STORE_EXIT_FLIPS` member — a home whose managed document the \
         store has lost must still fail the run an adopter's CI gates on; got:\n{text}",
    );
}

/// **Arm 11 — an adopted home emptied in the worktree alone still fires.** The deletion is
/// uncommitted, so the last committed blob at the path is the one at `HEAD` — jigc's own
/// stamped document — and the narrowing reads it from there rather than from a parent commit
/// that does not exist for this cell.
#[test]
fn an_adopted_home_emptied_in_the_worktree_alone_still_fires() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    std::fs::remove_file(corpus.repo().join("CHANGELOG.md")).expect("empty the home on disk");
    corpus.fresh_clone_shape();

    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);
    assert!(
        fires_at(&text, "CHANGELOG.md"),
        "`HEAD` still carries jigc's own stamped changelog at `CHANGELOG.md` and the \
         worktree does not — the home is vacated whether or not the removal was committed; \
         got:\n{text}",
    );
}

// ---------------------------------------------------------------------------------------
// The M52 completion audit, fix 6 — the removal's **state**
// ---------------------------------------------------------------------------------------

/// One member of the removal-state axis: the ordinary human act that puts a declared home in
/// that state, and what the route it draws has to do when it is run verbatim.
struct RemovalCell {
    /// The state token, ⇔-fenced against `cli::orphan::Removal::ALL` below.
    kind: &'static str,
    /// The act, on an adopted corpus whose `CHANGELOG.md` is jigc's own committed document.
    act: fn(&TrialCorpus),
    /// Whether running every `git …` command the route names, in order, must leave the
    /// finding **cleared**. False for the committed removal alone: there the git command is a
    /// locator and the restore is a human act the route describes rather than spells.
    route_repairs: bool,
    /// A command no route for this state may name, because it belongs to another state's
    /// repair — the assertion that the route is a *function of* the state rather than one
    /// route carrying every state's advice.
    foreign_command: &'static str,
}

fn commit_the_removal(corpus: &TrialCorpus) {
    corpus.git(&["rm", "-q", "CHANGELOG.md"]);
    corpus.git(&["commit", "-q", "-m", "retire the changelog"]);
}

fn delete_in_the_worktree(corpus: &TrialCorpus) {
    std::fs::remove_file(corpus.repo().join("CHANGELOG.md")).expect("empty the home on disk");
}

fn stage_the_removal(corpus: &TrialCorpus) {
    corpus.git(&["rm", "-q", "CHANGELOG.md"]);
}

/// The states a vacated declared home's **removal** can be in — the class the emptiness leg
/// admits, each produced by an ordinary human act rather than by a manufactured fixture.
const REMOVAL_CELLS: &[RemovalCell] = &[
    RemovalCell {
        kind: "committed",
        act: commit_the_removal,
        route_repairs: false,
        foreign_command: "git checkout",
    },
    RemovalCell {
        kind: "worktree-only",
        act: delete_in_the_worktree,
        route_repairs: true,
        foreign_command: "jigc ingest",
    },
    RemovalCell {
        kind: "staged-removal",
        act: stage_the_removal,
        route_repairs: true,
        foreign_command: "jigc ingest",
    },
];

/// **Arm 12 — the route is a function of the removal's state, and no route names a locator
/// that answers nothing.**
///
/// The defect, driven at `b9ab6a70` before a line changed: the route was written for the
/// committed-removal cell alone, while the emptiness leg reads the **worktree** — so an
/// uncommitted `rm CHANGELOG.md` drew *"`git log --diff-filter=D -1 -- CHANGELOG.md` names the
/// commit that removed it"*, which printed nothing at exit 0, and prescribed a restore-and-
/// commit plus a re-registration the state does not need (`git checkout -- CHANGELOG.md`, the
/// act that actually repairs it, was named by neither this row nor the
/// `reconciliation.rename` row beside it).
///
/// Every backticked command the route carries is lifted out of the **emitted** bytes and run
/// verbatim: a query must answer, a repair must repair, and no state's route may carry another
/// state's advice.
#[test]
fn the_route_is_a_function_of_the_removals_state() {
    // The axis is the producer's own enumeration, not this suite's list of what one fix
    // reached: a state added to `Removal` reddens here until an act that produces it and the
    // repair it needs are driven below.
    let declared: Vec<&str> = cli::orphan::Removal::ALL
        .iter()
        .map(Removal::kind)
        .collect();
    let driven: Vec<&str> = REMOVAL_CELLS.iter().map(|cell| cell.kind).collect();
    assert_eq!(
        declared, driven,
        "every member of `cli::orphan::Removal` is a removal state a declared home can be \
         found in, and each one draws its own repair — the cells below must be that set",
    );

    for cell in REMOVAL_CELLS {
        let corpus = TrialCorpus::build(State::CommittedSingletons);
        (cell.act)(&corpus);
        corpus.fresh_clone_shape();

        let text = printed(&corpus.jigc(&["validate"]));
        assert!(
            fires_at(&text, "CHANGELOG.md"),
            "[{}] the home is empty and its last committed blob is jigc's own stamped \
             document — the finding must fire; got:\n{text}",
            cell.kind,
        );

        let route = emitted_route(&text, "CHANGELOG.md");
        let commands = backticked(&route);
        assert!(
            !route.contains(cell.foreign_command),
            "[{}] the route names `{}`, which repairs a different removal state — a route \
             that carries every state's advice leaves the reader to guess which line is \
             theirs; got: {route}",
            cell.kind,
            cell.foreign_command,
        );
        assert!(
            commands.iter().any(|c| c.starts_with("jigc unmanage")),
            "[{}] every state names `jigc unmanage` as the verb that is *not* an exit — it \
             is what the `reconciliation.rename` row beside this one offers, and the two \
             must not leave the reader with two instructions; got: {route}",
            cell.kind,
        );

        let git_commands: Vec<&String> =
            commands.iter().filter(|c| c.starts_with("git ")).collect();
        assert!(
            !git_commands.is_empty(),
            "[{}] the route names no git command at all; got: {route}",
            cell.kind,
        );
        for command in &git_commands {
            let argv: Vec<&str> = command.split_whitespace().skip(1).collect();
            let out = corpus.git(&argv);
            if matches!(argv.first(), Some(&"show") | Some(&"log")) {
                assert!(
                    !out.trim().is_empty(),
                    "[{}] the route's query `{command}` printed nothing — a locator that \
                     names nothing is worse than no locator",
                    cell.kind,
                );
            }
        }

        let after = printed(&corpus.jigc(&["validate"]));
        assert_eq!(
            !fires_at(&after, "CHANGELOG.md"),
            cell.route_repairs,
            "[{}] running every git command the route names, verbatim and in order, must \
             {} the finding; got:\n{after}",
            cell.kind,
            if cell.route_repairs {
                "clear"
            } else {
                "leave standing (its git command is a locator, not the repair)"
            },
        );
    }
}
