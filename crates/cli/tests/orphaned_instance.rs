//! M51 Increment 8 / T3 — **a doctype leaves the resolved set, and jigc stops calling the
//! files it wrote clean.**
//!
//! The condition ([settle-record](../../../completions/artifacts/M51/settle-record.md) → §18,
//! D12): a committed `.md` carrying a jigc **schema-version stamp** that **no resolved
//! doctype claims**. Driven at the wave's baseline, the store surface was silent about it in
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
//! The kind of set arm 1 iterates: a **code-side registry** — the flip is *found in*
//! [`cli::render::STORE_EXIT_FLIPS`] and every string this suite asserts (the code, the cause
//! the closing line names) is read back off that member, so membership **is** the assertion and
//! a member that stopped flipping the exit reddens here rather than drifting.

use crate::support;

use std::process::{Command, Output};

use cli::render::{STORE_EXIT_FLIPS, StoreExitFlip};
use support::trial_corpus::{FixturePack, State, TrialCorpus};

/// The three committed, stamped singletons the methodology pack authored — the population
/// that orphans the moment that pack leaves the composition. Written down because which
/// doctypes a *fixture* pack drops is a property of the fixture, not of any registry.
const ORPHANED: &[&str] = &["VISION.md", "docs/decisions-log.md", "docs/roadmap.md"];

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

    // The two consumers agree as SETS: every path the sweep located is listed, and the
    // listing invents none. Both sides are lifted from the emitted bytes.
    let swept = jigc_under_pack(&corpus, &pack, &["validate"]);
    let report = printed(&swept);
    let mut located: Vec<&str> = report
        .lines()
        .filter_map(|line| line.trim().strip_prefix("at: "))
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
    corpus.git(&["commit", "-q", "-m", "move the roadmap out of band"]);

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
    corpus.git(&["commit", "-q", "-m", "move the roadmap out of band"]);

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
