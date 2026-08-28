//! **The item's own leaf region is schema-keyed** (M49 Increment 1, T1) — the write
//! side's half of the item-region class.
//!
//! `item_own_leaf_region` bounded an item's leaf region at *the first deeper heading*,
//! and a **multi-slot** item's own `#### <Leaf-Title>` sub-labels **are** deeper
//! headings. So on any item block declaring ≥2 slots, the item's own trailing field
//! group sits outside the region every field write is scoped to, and the whole
//! splice-or-generate spine mis-reads the item:
//!
//!   * `set-field` finds no bullet → `NotPresent` → the caller falls through to
//!     `insert_item_field`, whose cold-fill arm re-renders the item and **appends a
//!     second bullet for the same key**,
//!   * the write acks at exit 0, `jigc task validate` reports nothing over the doc, and
//!   * the pinned 1.0 `doc show --format json` returns the **first, stale** value —
//!     actively contradicting the write it just acknowledged.
//!
//! The fix keys the boundary on the schema, never on byte-sniffing
//! (`design/surface-contract.md` → *a context-dependent rule takes its context as a
//! fence input*): a deeper heading that is a **declared slot sub-label of this item's
//! template** does not end the region; anything deeper that is not one does — and an
//! `{#id}` anchor is what identifies such a heading as a nested item rather than a
//! defect (`completions/artifacts/M49/settle-record.md` → D3(A)/D4).
//!
//! **The shape space is manufactured, not enumerated** (settle-record → *Acceptance*):
//! across both shipped packs `roadmap.milestones` is the only multi-slot item block and
//! it carries **no settable field**, so the corpus sits on the safe side of the boundary
//! by accident of shape and the registry can populate only two of the axis's six cells.
//! The `{multi-slot, ¬nested}` arms below therefore drive a [`FixturePack`] — a
//! freeze-exempt copy of the dev pack carrying one reshaped doctype — end to end through
//! the real binary.
//!
//! The fourth arm is the **regression** direction of the same seam: on the *shipped*
//! `changelog`, a release's own `link` field must keep landing in the release's own
//! region and never reach into its nested `#### <category>` change-group, whose heading
//! is an anchored nested item and therefore still a boundary.

use crate::support;

use support::trial_corpus::{FixturePack, State, TrialCorpus};

/// The manufactured `{multi-slot, ¬nested}` doctype: an item block declaring **two**
/// slots and **one settable field**. It is written over the dev pack's `changelog` slot
/// in the doctype registry (a fixture pack replaces a schema rather than registering a
/// new doctype, so nothing outside the schema file has to be manufactured too).
const MULTI_SLOT_SCHEMA: &str = "\
type: changelog
id-from: title
description: A manufactured multi-slot findings log — two prose leaves and one settable field per item.
usage: the item-region boundary needs a multi-slot item block carrying a settable field, which neither shipped pack declares.
sections:
  - id: findings
    repeatable:
      id-from: label
      block:
        - { id: label, type: string }
        - { id: status, type: string, optional: true }
        - { id: statement, slot: { hint: \"The finding, stated.\" } }
        - { id: proves, slot: { hint: \"What the finding proves.\" } }
";

/// The workflow whose create-gate admits the fixture doctype — a `creates-task: true`
/// work-workflow, so `jigc start --workflow` mints a task the writes address.
const FIXTURE_WORKFLOW: &str = "\
---
when: record a finding in the findings log
description: Author the findings log.
usage: a finding needs recording in the findings log.
creates-task: true
allows-create: [{type: changelog, as: findings}]
---
{{ include: step:finalize }}
";

/// Build the fixture corpus and return it with the live task id and the item address the
/// binary emitted — both read from the real output, never reconstructed.
fn multi_slot_corpus(pack: &FixturePack) -> (TrialCorpus, String, String) {
    let corpus = TrialCorpus::build_with_pack(State::Fresh, pack);
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
    let item = corpus.add_item("changelog:findings-log#findings", "First finding", &task);
    // Both prose leaves filled: an unfilled required slot is its own finding, and the
    // arm below asserts the *absence* of findings over this doc.
    corpus.set_slot(&format!("{item}/statement"), &task, "The statement prose.");
    corpus.set_slot(&format!("{item}/proves"), &task, "The proves prose.");
    (corpus, task, item)
}

/// Read the fixture doc's staged bytes back through the binary's own staged read.
fn staged_doc(corpus: &TrialCorpus, task: &str) -> String {
    corpus.jigc_ok(&["doc", "show", "changelog:findings-log", "--task", task])
}

/// (i) Two `set-field` writes on one item field leaf leave exactly ONE bullet, carrying
/// the SECOND value — the corruption's byte-level face.
#[test]
fn two_writes_on_a_multi_slot_item_field_leave_one_bullet() {
    let pack = FixturePack::from_dev_pack("multi-slot");
    pack.write_schema("changelog", MULTI_SLOT_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, item) = multi_slot_corpus(&pack);

    corpus.set_field(&format!("{item}/status"), &task, "draft");
    corpus.set_field(&format!("{item}/status"), &task, "done");

    let bytes = staged_doc(&corpus, &task);
    let bullets: Vec<&str> = bytes
        .lines()
        .filter(|line| line.trim_start().starts_with("- status:"))
        .collect();
    assert_eq!(
        bullets,
        vec!["- status: done"],
        "two writes on a multi-slot item's field leaf must leave ONE bullet carrying \
         the second value; the item's own `#### <Leaf-Title>` sub-labels must not bound \
         its leaf region away from its own field group.\n--- staged bytes ---\n{bytes}",
    );
}

/// (ii) The pinned 1.0 `doc show --format json` read returns the value that was written
/// — not the first, stale one.
#[test]
fn the_json_read_returns_the_value_that_was_written() {
    let pack = FixturePack::from_dev_pack("multi-slot-json");
    pack.write_schema("changelog", MULTI_SLOT_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, item) = multi_slot_corpus(&pack);

    corpus.set_field(&format!("{item}/status"), &task, "draft");
    corpus.set_field(&format!("{item}/status"), &task, "done");

    let json = corpus.jigc_ok(&[
        "doc",
        "show",
        &format!("{item}/status"),
        "--task",
        &task,
        "--format",
        "json",
    ]);
    assert_eq!(
        json.trim(),
        "\"done\"",
        "the pinned `--format json` read must return the value the last write \
         acknowledged, never the stale first one",
    );
}

/// (iii) `jigc task validate` raises **no finding over the doc** — the write that
/// corrupted it acked at exit 0, and the gate that is supposed to catch corruption
/// reported nothing, so a green gate over this doc is only meaningful once the bytes
/// are right.
#[test]
fn the_gate_is_silent_over_a_correctly_written_multi_slot_item() {
    let pack = FixturePack::from_dev_pack("multi-slot-gate");
    pack.write_schema("changelog", MULTI_SLOT_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, item) = multi_slot_corpus(&pack);

    corpus.set_field(&format!("{item}/status"), &task, "draft");
    corpus.set_field(&format!("{item}/status"), &task, "done");

    let out = corpus.jigc(&["task", "validate", &task]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let over_the_doc: Vec<&str> = text
        .lines()
        .filter(|line| line.contains("changelog:findings-log"))
        .collect();
    assert!(
        over_the_doc.is_empty(),
        "the gate must raise nothing over the doc: {over_the_doc:?}\n--- full output ---\n{text}",
    );
    // And the doc really does hold the single correct bullet — so the silence above is
    // a clean doc's silence, not the pre-fix silence over a corrupted one.
    let bytes = staged_doc(&corpus, &task);
    assert!(
        bytes.contains("- status: done") && !bytes.contains("- status: draft"),
        "the silent gate must be silent over CORRECT bytes:\n{bytes}",
    );
}

/// (iv) The regression direction, on the **shipped** `changelog`: a release's own `link`
/// field write — and its `--unset` — stay inside the release's own leaf region and never
/// reach the nested `#### <category>` change-group, whose anchored heading is still a
/// boundary. The rest of the file round-trips byte-for-byte.
#[test]
fn a_release_field_write_never_reaches_its_nested_change_group() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let task = corpus.start_workflow("record-change", "amend the release link");
    // The nested change-group's whole block, read through the SAME surface the
    // post-write assertions use (M43 serves the staged and committed copies through one
    // parse/slice/render path), so the comparison is bytes-to-bytes rather than
    // committed-file-to-rendered-read. Nothing is staged in this task yet, so the
    // task-less read is this doc's pre-write state.
    let before = corpus.jigc_ok(&["doc", "show", "changelog:changelog"]);
    let nested_before = before[before.find("#### ").expect("a nested change-group")..].to_string();
    let nested_before = nested_before.as_str();
    corpus.set_field(
        "changelog:changelog#releases/1-0-0/link",
        &task,
        "https://example.com/compare/0.9.0...1.0.1",
    );
    let staged = corpus.jigc_ok(&["doc", "show", "changelog:changelog", "--task", &task]);
    let links: Vec<&str> = staged
        .lines()
        .filter(|line| line.trim_start().starts_with("- link:"))
        .collect();
    assert_eq!(
        links,
        vec!["- link: https://example.com/compare/0.9.0...1.0.1"],
        "the release's own `link` bullet is spliced in place:\n{staged}",
    );
    assert!(
        staged.contains("- date:"),
        "the release keeps its sibling `date` bullet:\n{staged}",
    );
    let nested_after = &staged[staged.find("#### ").expect("a nested change-group")..];
    assert_eq!(
        nested_after, nested_before,
        "the nested change-group's bytes are untouched by a write to its PARENT's field",
    );

    corpus.jigc_ok(&[
        "doc",
        "set-field",
        "changelog:changelog#releases/1-0-0/link",
        "--unset",
        "--task",
        &task,
    ]);
    let unset = corpus.jigc_ok(&["doc", "show", "changelog:changelog", "--task", &task]);
    assert!(
        !unset.contains("- link:"),
        "`--unset` removes the release's own `link` bullet:\n{unset}",
    );
    assert!(
        unset.contains("- date:"),
        "`--unset` of one field keeps the group's sibling bullet:\n{unset}",
    );
    let nested_unset = &unset[unset.find("#### ").expect("a nested change-group")..];
    assert_eq!(
        nested_unset, nested_before,
        "the nested change-group's bytes survive the parent's `--unset` too",
    );
    // The doc still parses against its schema — the round-trip half of the regression.
    let out = corpus.jigc(&["task", "validate", &task]);
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let over_the_changelog: Vec<&str> = text
        .lines()
        .filter(|line| line.contains("conformance.") && line.contains("changelog:changelog"))
        .collect();
    assert!(
        over_the_changelog.is_empty(),
        "the edited changelog must still parse: {over_the_changelog:?}\n--- full output ---\n{text}",
    );
}
