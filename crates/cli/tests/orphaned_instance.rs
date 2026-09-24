//! M51 Increment 8 / T3 — **a doctype leaves the resolved set, and jigc stops calling the
//! files it wrote clean.**
//!
//! The condition ([settle-record](../../../completions/artifacts/M51/settle-record.md) → §18,
//! D12; its subject narrowed at the M51 completion audit — arm 8): a committed `.md` **inside
//! jigc's declared homes** (`cli::orphan::Territory`) carrying a **`schema-version:` stamp**
//! that **no resolved doctype claims**. Driven at the wave's baseline, the store surface was silent about it in
//! both directions — `jigc validate` printed *"no findings — the committed store validates
//! clean"* at **exit 0** while `git ls-files` still carried every one of those files, and
//! `jigc doc list` dropped their rows entirely. A green there means *I stopped looking at
//! these files*, on the verb [MIGRATING.md](../../../MIGRATING.md) tells adopters to CI-gate
//! on — which is why the code is **blocking at store scope and joins
//! [`cli::render::STORE_EXIT_FLIPS`]** rather than reporting at exit 0 beside it (§9): without
//! the flip the adopter's CI is still green and the decision's own admission argument is
//! undischarged.
//!
//! **The fixture is the population, not a mock.** `JIGC_PACK_DIR` supersedes the compose
//! marker, so pointing a corpus built over the embedded **pair** at a copy of the **dev** pack
//! alone takes the whole methodology pack out of the resolved set — three committed,
//! jigc-authored, stamped singletons (`VISION.md`, `docs/decisions-log.md`,
//! `docs/roadmap.md`) whose declared doctype is now defined by nothing, with `CHANGELOG.md`
//! left claimed as the control. Today that population is reachable through `JIGC_PACK_DIR` or
//! a PB-1 project pack that drops a doctype it owns, and **the narrowness is a claim about the
//! calendar, not about the hole**: PB-1 shipped at M49 as the documented way an adopter owns
//! doctypes, with a declared bound making divergence expected, and the population is narrow
//! because there are no adopters yet — the population 1.0.0 exists to create.
//!
//! **Arm 3 is the partition, driven.** A *home* that moved is not a *doctype* that left:
//! `file-state.orphaned-doc` already speaks for a strand, whose doctype still resolves, so a
//! stamped strand must be named by that code and by this one **never** — otherwise the new
//! finding tells the reader the pack defining `roadmap` is gone while `jigc describe` lists it.
//!
//! **Arms 6 and 7 drive the partition's other half** (M51 inc-8 T5). The condition §18 settles
//! is asked once — *does the file's declared doctype resolve?* — so this suite owns both
//! answers or owns neither: `schema-conformance.unversioned-doctype` is the *resolved* half,
//! the doc whose doctype is defined and whose owning pack ships no manifest entry for it, and
//! the same fixture pack produces it for free (it drops the dev pack's own manifest on the way
//! to dropping the methodology pack). One corpus, one report, two causes, and the crossings
//! asserted empty in both directions.
//!
//! The kind of set arm 1 iterates: a **code-side registry** — the flip is *found in*
//! [`cli::render::STORE_EXIT_FLIPS`] and every string this suite asserts (the code, the cause
//! the closing line names) is read back off that member, so membership **is** the assertion and
//! a member that stopped flipping the exit reddens here rather than drifting.

use crate::support;

use std::process::{Command, Output};

use cli::render::{STORE_EXIT_FLIPS, StoreExitFlip};
use support::trial_corpus::{FixturePack, State, TrialCorpus};

/// The committed, stamped singletons the methodology pack authored **inside jigc's declared
/// territory** — the population that orphans the moment that pack leaves the composition.
/// Written down because which doctypes a *fixture* pack drops is a property of the fixture,
/// not of any registry.
const ORPHANED: &[&str] = &["docs/decisions-log.md", "docs/roadmap.md"];

/// **The stated residual** (M51 completion audit; `cli::orphan::Territory` → residual 2). The
/// same fixture pack also orphans `VISION.md`, whose departed doctype homed it at the **repo
/// root**. A root-level placement home contributes no directory that is not the whole
/// repository, so narrowing the sweep's subject to jigc's declared homes necessarily leaves
/// that one cell unflagged — its pre-M51 exit-0 status quo. It is asserted here rather than
/// dropped, so the residual is visible in the suite that owns the condition and cannot quietly
/// widen or quietly grow.
const ROOT_HOMED_RESIDUAL: &str = "VISION.md";

/// The control: a committed stamped singleton whose doctype the dev pack still defines, so
/// the same sweep must stay silent about it. Without it a green arm 1 would be satisfied by a
/// check that flags every stamped file it meets.
const STILL_CLAIMED: &str = "CHANGELOG.md";

/// The [`STORE_EXIT_FLIPS`] member this suite drives. Read from the table rather than
/// hand-spelled: a code or a closing-line cause asserted as a literal here could drift from
/// the one the renderer prints, and the point of the member is that it cannot.
fn orphan_flip() -> &'static StoreExitFlip {
    STORE_EXIT_FLIPS
        .iter()
        .find(|flip| flip.id == "orphaned-instance")
        .expect(
            "`orphaned-instance` must be a member of `cli::render::STORE_EXIT_FLIPS` — the \
             store sweep's exit rule is that table, so a blocking store finding that is not a \
             member reports at exit 0 and the adopter's CI stays green over it",
        )
}

/// The code the member's own witness carries — its production identity, by construction.
fn orphan_code() -> String {
    (orphan_flip().witness)().code
}

/// The `blocking · <code> — committed doc \`<rel>\`` head this finding renders for `rel`.
fn orphan_line_head(rel: &str) -> String {
    format!("blocking · {} — committed doc `{rel}`", orphan_code())
}

/// Run `jigc <args>` in `corpus` with `JIGC_PACK_DIR` pointed at `pack` — the corpus's own
/// helper chooses the *embedded* packs, and the whole fixture is that this run composes a
/// different set than the one that authored the docs.
fn jigc_under_pack(corpus: &TrialCorpus, pack: &FixturePack, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(corpus.repo())
        .env("HOME", corpus.home())
        .env("JIGC_PACK_DIR", pack.path())
        .output()
        .expect("spawn jigc")
}

fn printed(out: &Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// **Arm 1 — a doctype leaves the resolved set and the sweep says so, at a non-zero exit.**
#[test]
fn a_doctype_that_leaves_the_resolved_set_orphans_its_committed_instances() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let pack = FixturePack::from_dev_pack("orphaned-instance");
    let flip = orphan_flip();

    let out = jigc_under_pack(&corpus, &pack, &["validate"]);
    let text = printed(&out);

    assert!(
        !out.status.success(),
        "a committed store jigc can no longer say anything about must not validate clean at \
         exit 0 — the false green is the whole condition; got:\n{text}",
    );
    for rel in ORPHANED {
        assert!(
            text.contains(&orphan_line_head(rel)),
            "`{rel}` is committed, stamped, and claimed by no resolved doctype — the sweep \
             must name it; got:\n{text}",
        );
        assert!(
            text.contains(&format!("jigc unmanage {rel}")),
            "the route must be runnable and path-specific for `{rel}` (the `Human` route's \
             second exit); got:\n{text}",
        );
    }
    assert!(
        !text.contains(&orphan_line_head(STILL_CLAIMED)),
        "`{STILL_CLAIMED}`'s doctype is still defined by the composed pack, so the sweep must \
         stay silent about it — a check that flags every stamped file it meets is not this \
         check; got:\n{text}",
    );
    assert!(
        !text.contains(&orphan_line_head(ROOT_HOMED_RESIDUAL)),
        "`{ROOT_HOMED_RESIDUAL}`'s departed doctype homed it at the repo root, which declares \
         no directory — the stated residual, asserted so it stays visible; got:\n{text}",
    );
    assert!(
        text.contains(flip.cause),
        "the closing line must name WHICH condition fired ({:?}) — the promise the preload \
         sends the agent there for; got:\n{text}",
        flip.cause,
    );
    assert!(
        text.contains("re-add the pack"),
        "the route's first exit names the act that restores the doctype; got:\n{text}",
    );
}

/// **Arm 2 — the same corpus under the pack that authored it is silent, and not because the
/// sweep found nothing.** The state carries a standing `repeatable-populated` advisory, so a
/// green here is the check discriminating rather than a sweep that reported nothing at all.
#[test]
fn the_shipped_corpus_under_the_embedded_pair_carries_no_orphan() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);

    assert!(
        out.status.success(),
        "every committed doc of this state is claimed by a resolved doctype, so the store \
         sweep must keep its report-only exit 0; got:\n{text}",
    );
    assert!(
        !text.contains(&orphan_code()),
        "no committed instance of a composed doctype may be called an orphan; got:\n{text}",
    );
}

/// **Arm 4 (M51 inc-8 T4) — the same condition on the store's own index read.** The sweep
/// and the listing are two consumers of one enumerator, so a file `jigc validate` blocks on
/// cannot be a file `jigc doc list` has never heard of — which is what the base did: the
/// listing carried the one still-claimed row and dropped the other three, on the surface the
/// adapter rule makes an agent's only sanctioned route to the corpus.
///
/// The row's three values are read off the emitted json: **`id: null`** (no resolved schema
/// defines the type its stamp was written under, and a stamp carries a version and no type —
/// so there is no honest identity to print), **`state: "orphaned"`**, and **`item-count:
/// null`** (no schema to parse against — a third answer for a third population, not a
/// revision of the best-effort `0` a parse *failure* yields).
///
/// **The two surfaces are asserted to agree as sets**, not merely to be non-empty each: the
/// orphan paths the listing reports are exactly the paths the sweep's findings located.
#[test]
fn doc_list_reports_every_orphaned_instance_the_sweep_blocks_on() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let pack = FixturePack::from_dev_pack("orphaned-instance-list");

    let listed = jigc_under_pack(&corpus, &pack, &["doc", "list", "--format", "json"]);
    assert!(
        listed.status.success(),
        "`doc list` is a report and stays exit 0 — the exit flip is the sweep's; got:\n{}",
        printed(&listed),
    );
    let json = String::from_utf8_lossy(&listed.stdout).into_owned();
    for rel in ORPHANED {
        let row = format!(
            "{{\n      \"id\": null,\n      \"path\": \"{rel}\",\n      \"state\": \
             \"orphaned\",\n      \"item-count\": null\n    }}"
        );
        assert!(
            json.contains(&row),
            "`{rel}` is stamped, committed, and claimed by no resolved doctype — the \
             listing must carry it as an orphaned row with a null identity and no item \
             count; got:\n{json}",
        );
    }
    assert!(
        json.contains(&format!("\"path\": \"{STILL_CLAIMED}\"")) && json.contains("\"managed\""),
        "the control's doctype still resolves, so its row stays a managed one with its \
         identity; got:\n{json}",
    );
    assert!(
        !json.contains(&format!(
            "\"path\": \"{ROOT_HOMED_RESIDUAL}\",\n      \"state\": \"orphaned\""
        )),
        "the listing inherits the sweep's subject bound, so the root-homed residual is absent \
         from both surfaces rather than from one; got:\n{json}",
    );

    // The two consumers agree as SETS: every path the sweep located is listed, and the
    // listing invents none. Both sides are lifted from the emitted bytes.
    let swept = jigc_under_pack(&corpus, &pack, &["validate"]);
    let report = printed(&swept);
    // The located set is read off the ORPHAN findings only, not off every `at:` line: since
    // M51 inc-8 T5 the same sweep also carries the partition's other cause — the resolved-
    // doctype advisory, keyed at a `<type>:<slug>` identity — and this arm's claim is about
    // the two consumers of the ORPHAN enumerator. A finding's `at:` line follows its own
    // head line, so each is attributed to the head above it rather than harvested blind.
    let lines: Vec<&str> = report.lines().collect();
    let mut located: Vec<&str> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            let path = line.trim().strip_prefix("at: ")?;
            let head = i.checked_sub(1).map(|prev| lines[prev])?;
            head.contains(&orphan_code()).then_some(path)
        })
        .collect();
    located.sort_unstable();
    located.dedup();
    let mut orphan_rows: Vec<&str> = json
        .split("\"state\": \"orphaned\"")
        .take(json.matches("\"state\": \"orphaned\"").count())
        .filter_map(|chunk| chunk.rsplit_once("\"path\": \"").map(|(_, tail)| tail))
        .filter_map(|tail| tail.split_once('"').map(|(path, _)| path))
        .collect();
    orphan_rows.sort_unstable();
    assert_eq!(
        orphan_rows, located,
        "the sweep and the listing read one enumerator — a file one of them names and the \
         other drops is the two-stories-about-one-file defect `doc list` was founded to \
         end; listing:\n{json}\nsweep:\n{report}",
    );

    // A doctype-narrowed listing carries no orphan row: an orphan belongs to no doctype.
    let filtered = jigc_under_pack(
        &corpus,
        &pack,
        &["doc", "list", "changelog", "--format", "json"],
    );
    let filtered_json = String::from_utf8_lossy(&filtered.stdout);
    assert!(
        !filtered_json.contains("\"orphaned\""),
        "`doc list <doctype>` narrows to one doctype's instances, and an orphan is an \
         instance of none; got:\n{filtered_json}",
    );
}

/// **Arm 5 (M51 inc-8 T4) — the partition holds on the listing too.** Arm 3 proves a stamped
/// strand is the home-re-point advisory's subject and never the orphaned-instance code's;
/// the listing consumes the same enumerator and inherits the same partition, so it must not
/// print an `orphaned` row for a doc whose doctype `jigc describe` still lists.
#[test]
fn a_stamped_strand_is_never_listed_as_an_orphaned_row() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    std::fs::create_dir_all(corpus.repo().join("notes")).expect("create the out-of-band dir");
    corpus.git(&["mv", "docs/roadmap.md", "notes/roadmap.md"]);
    // The installed `pre-commit` backstop is stepped over, not defeated: since M53 (the
    // pre-v1 usability batch, row 1) its M35 blocking arm reaches the **placement** family,
    // and `roadmap` is one. The claim under test is the sweep's partition, which
    // presupposes the move landed.
    corpus.git(&[
        "-c",
        "core.hooksPath=/dev/null",
        "commit",
        "-q",
        "-m",
        "move the roadmap out of band",
    ]);

    let out = corpus.jigc(&["doc", "list", "--format", "json"]);
    let json = String::from_utf8_lossy(&out.stdout);
    assert!(
        !json.contains("\"orphaned\""),
        "the `roadmap` doctype resolves — only its home moved — so the listing must not \
         call its stranded instance an orphan; got:\n{json}",
    );
}

/// **Arm 3 — the partition: a home that moved is not a doctype that left.** An out-of-band
/// `git mv` strands a stamped singleton at a path its doctype does not home; that is
/// `file-state.orphaned-doc`'s subject, and the doctype still resolves — so the orphaned-
/// instance code must not also claim it, on pain of telling the reader a pack is gone that
/// `jigc describe` still lists.
#[test]
fn a_stamped_strand_is_reported_as_a_strand_and_never_as_an_orphaned_instance() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    std::fs::create_dir_all(corpus.repo().join("notes")).expect("create the out-of-band dir");
    corpus.git(&["mv", "docs/roadmap.md", "notes/roadmap.md"]);
    // The installed `pre-commit` backstop is stepped over, not defeated: since M53 (the
    // pre-v1 usability batch, row 1) its M35 blocking arm reaches the **placement** family,
    // and `roadmap` is one. The claim under test is the sweep's partition, which
    // presupposes the move landed.
    corpus.git(&[
        "-c",
        "core.hooksPath=/dev/null",
        "commit",
        "-q",
        "-m",
        "move the roadmap out of band",
    ]);

    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);

    assert!(
        text.contains("file-state.") && text.contains("notes/roadmap.md"),
        "the stranded doc is the home-re-point advisory's subject and must be reported as \
         one; got:\n{text}",
    );
    assert!(
        !text.contains(&orphan_line_head("notes/roadmap.md")),
        "a strand's doctype RESOLVES — only its home moved — so calling it an orphaned \
         instance would be the law-1 lie the partition exists to prevent; got:\n{text}",
    );
}

/// **Arm 6 (M51 inc-8 T5) — the partition, in one corpus and one report.** The condition
/// §18 settles is asked **once** — *does the file's declared doctype resolve to a schema in
/// the composed set?* — and each answer has exactly one owner. Arms 1–5 drive the *does not*
/// half; this arm drives **both halves at once**, because a partition is only a partition if
/// the two causes can be seen apart in a single sweep over a single corpus.
///
/// The fixture gives both for free and neither by accident: [`FixturePack::from_dev_pack`]
/// takes the methodology pack out of the resolved set (three orphans) **and** drops the dev
/// pack's own `schema-manifest.yaml` (so `changelog` — the one doctype still resolved and
/// still committed — is a doctype the composed set *defines* and no manifest *versions*).
/// Under that pack `engine::validate::SCHEMA_VERSION_CURRENT_CODE`, the M42 check that
/// exists to catch an unmigrated corpus, **can never fire** over `CHANGELOG.md`: nothing
/// stamps it, so nothing can find its stamp stale. Driven at the base the sweep said nothing
/// at all about that file.
///
/// The two rows differ in **every** dimension the split turns on — code, severity, and the
/// address form (the orphan has no identity to claim and is keyed at its path; the
/// unversioned doc's doctype resolves, so it keys at its `<type>:<slug>` identity like every
/// other per-doc finding of this family) — and the arm asserts the crossings are empty in
/// both directions.
#[test]
fn a_resolved_doctype_with_no_manifest_entry_answers_beside_the_orphans() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let pack = FixturePack::from_dev_pack("unversioned-doctype");

    let out = jigc_under_pack(&corpus, &pack, &["validate"]);
    let text = printed(&out);

    assert!(
        text.contains(&format!(
            "advisory · {} — `{STILL_CLAIMED}`:",
            engine::validate::UNVERSIONED_DOCTYPE_CODE
        )),
        "`{STILL_CLAIMED}` is a committed instance of a doctype the composed set defines and \
         no manifest versions — the sweep must say so, and say it as an advisory; got:\n{text}",
    );
    assert!(
        text.contains("at: changelog:changelog"),
        "the doctype RESOLVES, so this doc has a `<type>:<slug>` identity to be keyed at — \
         the orphan's path-keying is a consequence of having none; got:\n{text}",
    );
    assert!(
        text.contains("add a `changelog` entry to its pack's `config/schema-manifest.yaml`")
            && text.contains("`changelog` is deliberately unfrozen"),
        "the route's two exits are declare the version, or state the doctype is deliberately \
         unfrozen — the second is a real exit, not a courtesy: an absent manifest is \
         `unchecked` by the manifest header's own stated design; got:\n{text}",
    );

    // The crossings are empty in both directions — one condition, two causes, one owner each.
    assert!(
        !text.contains(&orphan_line_head(STILL_CLAIMED)),
        "`{STILL_CLAIMED}`'s doctype resolves, so calling it an orphan would be the law-1 lie \
         the partition exists to prevent; got:\n{text}",
    );
    for rel in ORPHANED {
        assert!(
            text.contains(&orphan_line_head(rel)),
            "the orphan half of the partition must still fire in this corpus, or the arm \
             proves nothing about a split; got:\n{text}",
        );
        assert!(
            !text.contains(&format!(
                "{} — `{rel}`:",
                engine::validate::UNVERSIONED_DOCTYPE_CODE
            )),
            "no schema in the composed set defines `{rel}`'s doctype, so no pack can be said \
             to ship no manifest entry for it — the unversioned cause speaks only where the \
             doctype resolves; got:\n{text}",
        );
    }

    // The control: the SAME corpus under the packs that authored it. Both shipped packs
    // carry a manifest listing every doctype they define, so the condition is absent and
    // the advisory must be too — otherwise this is a check that fires over every committed
    // doc it meets rather than over the state it names.
    let composed = printed(&corpus.jigc(&["validate"]));
    assert!(
        !composed.contains(engine::validate::UNVERSIONED_DOCTYPE_CODE),
        "every doctype of the embedded pair is listed in its own pack's manifest, so no \
         committed instance may be called unversioned; got:\n{composed}",
    );
}

/// **Arm 7 (M51 inc-8 T5) — the advisory flips no exit, driven rather than asserted from the
/// table.** §18's two severities are read off the two causes: the orphan's exit flip is
/// *required* by its own admission argument (a green meaning *I stopped looking at these
/// files*, on the verb [MIGRATING.md](../../../MIGRATING.md) tells adopters to CI-gate on),
/// while flipping this one — quoting the human's confirmation — *"would fail every
/// manifest-less project pack's CI for a permitted choice"*.
///
/// So the arm removes the *other* cause from the corpus and drives what is left: with the
/// three orphaned files gone, the manifest-less `changelog` is the only condition in the
/// repo, and `jigc validate` must report it **at exit 0**. Driven at the base this same state
/// printed *"no findings — the committed store validates clean"* — the red both halves of
/// this task were written against.
#[test]
fn the_unversioned_doctype_advisory_reports_at_exit_zero() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let pack = FixturePack::from_dev_pack("unversioned-doctype-exit");
    for rel in ORPHANED {
        corpus.git(&["rm", "-q", rel]);
    }
    corpus.git(&["commit", "-q", "-m", "drop the docs of the departed pack"]);

    let out = jigc_under_pack(&corpus, &pack, &["validate"]);
    let text = printed(&out);

    assert!(
        out.status.success(),
        "a manifest-less pack is `unchecked` by the manifest design's own words, so the one \
         advisory it earns must not fail a CI that gates on `jigc validate`; got:\n{text}",
    );
    assert!(
        text.contains(engine::validate::UNVERSIONED_DOCTYPE_CODE),
        "exit 0 must be a REPORTED advisory, never silence — the base's silence over this \
         exact state is the red this arm was written against; got:\n{text}",
    );
    assert!(
        !STORE_EXIT_FLIPS
            .iter()
            .any(|flip| flip.id == "unversioned-doctype"),
        "the exit-0 fact and the table must agree: this cause is deliberately NOT a member \
         of the store sweep's exit-flip axis",
    );
}

/// A stamped `.md` **outside** jigc's declared territory — a plain team document that happens
/// to carry a `schema-version:` key in its front matter. Written down here rather than derived
/// because the whole point is that no jigc home, knob or registry mentions these paths: one at
/// the repo root, one in an arbitrary nested directory jigc was never pointed at.
const OUTSIDE_TERRITORY: &[&str] = &["api-notes.md", "notes/deep/api.md"];

/// **Arm 8 (M51 completion audit) — the subject is jigc's declared territory, not the
/// repository.** The base predicate took its subject from `committed_markdown` — *every*
/// committed `.md` — and kept whatever carried a `schema-version:` key, so a team document
/// using that key for its own purposes anywhere in the repo flipped `jigc validate` to exit 1
/// with a message claiming the bytes were *"a jigc schema-version stamp"*. The stamp is a bare
/// integer with no namespace, so jigc cannot tell its own stamp from anyone else's — which
/// makes the *location* the only honest discriminator available today: a stamped file inside
/// the docs-root tree, under a non-root `placement-root`, or under a resolved doctype's home
/// directory is jigc's to speak for; a stamped file anywhere else is not.
#[test]
fn a_stamped_doc_outside_jigcs_territory_is_never_claimed() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    for rel in OUTSIDE_TERRITORY {
        let abs = corpus.repo().join(rel);
        std::fs::create_dir_all(abs.parent().expect("every fixture path has a parent"))
            .expect("create the team document's directory");
        std::fs::write(
            &abs,
            "---\ntitle: API notes\nschema-version: 3\n---\n\n# API notes\n",
        )
        .expect("write the team document");
        corpus.git(&["add", "--", rel]);
    }
    corpus.git(&["commit", "-q", "-m", "team notes"]);

    let out = corpus.jigc(&["validate"]);
    let text = printed(&out);
    assert!(
        out.status.success(),
        "a plain team document that carries a `schema-version:` key is not jigc's subject — \
         claiming it flips the verb MIGRATING.md tells adopters to CI-gate on, over bytes jigc \
         never wrote; got:\n{text}",
    );
    for rel in OUTSIDE_TERRITORY {
        assert!(
            !text.contains(&orphan_line_head(rel)),
            "`{rel}` sits under no docs-root, no `placement-root` and no doctype home — jigc \
             has not been told it owns that path, so it may not say what the file is; \
             got:\n{text}",
        );
    }
}
