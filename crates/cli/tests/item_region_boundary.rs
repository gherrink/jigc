//! **The item's own leaf region is schema-keyed** (M49 Increment 1, T1 + T2) — one root
//! cause, both seams.
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
//!
//! Arms (v)–(viii) are the **parse** seam (T2), where the same root cause made
//! `{multi-slot ∧ nested}` a shape the writer emitted at exit 0 and its own reader
//! rejected. Their header sits above them.

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

// ============================================================================
// T2 — the PARSE seam: `{multi-slot ∧ nested}` becomes legal
//
// The same root cause, read side. `parse_items` bounded a nested-bearing item's own
// leaf region at the **first deeper heading** (`first_nested_heading`) and scanned its
// nested items from the item's `content_start` — so on a template that is BOTH
// multi-slot and nested, the item's own first `#### <Leaf-Title>` sub-label ended the
// leaf region *and* was then read as a nested item head. Driven on `1.0.0-rc.12`, a
// plain top-level `doc add-item` acked at **exit 0** over bytes its own parser rejects:
// `doc show` answered `store.unparseable`, `task validate` raised two
// `conformance.item-slot-label-missing` AND two `conformance.item-heading-unanchored`
// over the writer's own skeleton, and the nested `add-item` was refused
// `write.wrong-shape`.
//
// The fix is the write seam's rule read backwards, and it is one function
// (`parse::is_item_slot_sub_label`): the item's leaf region ends at the first deeper
// heading that is neither a declared slot sub-label of this item's template nor an
// anchored nested item, and the nested scan starts where that region ends. So the shape
// is **fixed rather than fenced** (`completions/artifacts/M49/settle-record.md` → D4).
//
// **The declared bound, and why the last two arms exist.** On a malformed corpus the
// `{#id}` anchor discriminator is undefined for exactly the documents
// `conformance.item-heading-unanchored` / `conformance.item-anchor-malformed` exist to
// report. That is acceptable on the write path (jigc's own writer always anchors) and
// must produce a **conformance finding rather than a guess** on the read path — so the
// unanchored and malformed-anchor arms are part of the acceptance, not an afterthought.
// ============================================================================

/// The manufactured `{multi-slot, nested}` doctype: an item block declaring **two**
/// slots *and* a nested repeatable — the combination no shipped pack declares (across
/// both packs `roadmap.milestones` is the only multi-slot item block and
/// `changelog.releases/changes` the only nested one, and they are different blocks).
const MULTI_SLOT_NESTED_SCHEMA: &str = "\
type: changelog
id-from: title
description: A manufactured multi-slot-and-nested case log — two prose leaves plus a nested note list per case.
usage: the item-region boundary needs an item block that is BOTH multi-slot and nested, which neither shipped pack declares.
sections:
  - id: cases
    repeatable:
      id-from: label
      block:
        - { id: label, type: string }
        - { id: statement, slot: { hint: \"The case, stated.\" } }
        - { id: proves, slot: { hint: \"What the case proves.\" } }
        - id: notes
          repeatable:
            id-from: label
            block:
              - { id: label, type: string }
              - { id: detail, slot: { hint: \"The note.\" } }
";

/// Build a `{multi-slot, nested}` fixture corpus carrying one top-level item with both
/// prose leaves filled, and return it with the live task id and the item address the
/// binary emitted.
fn multi_slot_nested_corpus(pack: &FixturePack) -> (TrialCorpus, String, String) {
    let corpus = TrialCorpus::build_with_pack(State::Fresh, pack);
    let task = corpus.start_workflow("log-finding", "record a case");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "changelog",
        "--title",
        "Case log",
        "--task",
        &task,
    ]);
    let item = corpus.add_item("changelog:case-log#cases", "First case", &task);
    corpus.set_slot(&format!("{item}/statement"), &task, "The statement prose.");
    corpus.set_slot(&format!("{item}/proves"), &task, "The proves prose.");
    (corpus, task, item)
}

/// The staged working copy of the fixture doc — the file the arms below hand-edit to
/// manufacture a malformed corpus.
fn staged_path(corpus: &TrialCorpus, task: &str) -> std::path::PathBuf {
    corpus
        .repo()
        .join(format!(".jigc/tasks/{task}/docs/changelog:case-log.md"))
}

/// `jigc task validate <task>`'s full output (it exits non-zero on a blocking finding,
/// which is the point of the last two arms).
fn validate_text(corpus: &TrialCorpus, task: &str) -> String {
    let out = corpus.jigc(&["task", "validate", task]);
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// (v) A `{multi-slot, nested}` item written by jigc's own writer **parses**: `doc show`
/// reads it back at exit 0, and the gate raises neither of the two codes the seam used
/// to manufacture over the writer's own skeleton.
#[test]
fn a_multi_slot_nested_item_parses_and_reads_back() {
    let pack = FixturePack::from_dev_pack("multi-slot-nested");
    pack.write_schema("changelog", MULTI_SLOT_NESTED_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, _item) = multi_slot_nested_corpus(&pack);

    let show = corpus.jigc(&["doc", "show", "changelog:case-log", "--task", &task]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&show.stdout),
        String::from_utf8_lossy(&show.stderr),
    );
    assert!(
        show.status.success(),
        "a doc jigc's own `doc create` + `doc add-item` just wrote must read back at \
         exit 0 — `store.unparseable` here is the writer acking bytes its own parser \
         rejects:\n{text}",
    );
    assert!(
        text.contains("#### Statement") && text.contains("#### Proves"),
        "both declared slot sub-labels are in the read-back bytes:\n{text}",
    );

    let findings = validate_text(&corpus, &task);
    let over_the_shape: Vec<&str> = findings
        .lines()
        .filter(|line| {
            line.contains("conformance.item-slot-label-missing")
                || line.contains("conformance.item-heading-unanchored")
        })
        .collect();
    assert!(
        over_the_shape.is_empty(),
        "the item's own `#### <Leaf-Title>` sub-labels are slot structure, not nested \
         item heads: {over_the_shape:?}\n--- full output ---\n{findings}",
    );
}

/// (vi) A nested `add-item` on that item **lands**, and the pinned `--format json` read
/// reports the nested item under its parent — the shape is usable, not merely parseable.
#[test]
fn a_nested_add_item_lands_under_a_multi_slot_parent() {
    let pack = FixturePack::from_dev_pack("multi-slot-nested-add");
    pack.write_schema("changelog", MULTI_SLOT_NESTED_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, item) = multi_slot_nested_corpus(&pack);

    let out = corpus.jigc(&[
        "doc",
        "add-item",
        &format!("{item}/notes"),
        "--title",
        "First note",
        "--task",
        &task,
    ]);
    let ack = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        out.status.success(),
        "a nested `add-item` under a MULTI-SLOT parent must land:\n{ack}",
    );
    let nested = ack.trim_end_matches('\n').to_string();

    let json = corpus.jigc_ok(&["doc", "show", &item, "--task", &task, "--format", "json"]);
    let value: serde_json::Value = serde_json::from_str(&json).expect("the item object is JSON");
    let notes = value["notes"]
        .as_array()
        .unwrap_or_else(|| panic!("the parent item carries its nested `notes` array:\n{json}"));
    let ids: Vec<&str> = notes.iter().filter_map(|n| n["id"].as_str()).collect();
    assert_eq!(
        ids.len(),
        1,
        "exactly the one nested item that was added:\n{json}",
    );
    assert!(
        nested.ends_with(ids[0]),
        "the emitted address `{nested}` names the nested item the read reports \
         ({ids:?}):\n{json}",
    );
    // The parent's own leaves survive the nested write — the region widened, it did not
    // move.
    assert_eq!(
        value["statement"].as_str(),
        Some("The statement prose."),
        "the parent's first slot is untouched by the nested write:\n{json}",
    );
    assert_eq!(
        value["proves"].as_str(),
        Some("The proves prose."),
        "the parent's second slot is untouched by the nested write:\n{json}",
    );
}

/// (vii) The declared bound, arm 1 — an **unanchored** deeper heading inside a
/// `{multi-slot, nested}` item is not a slot sub-label and not an anchored nested item,
/// so the boundary refuses to guess which it is: it ends the leaf region there and the
/// parser reports `conformance.item-heading-unanchored`, whose located message *is* its
/// route (the code is route-exempt — no CLI verb repairs a hand-broken byte). The
/// declared sub-labels around it still parse.
#[test]
fn an_unanchored_deeper_heading_is_reported_not_guessed() {
    let pack = FixturePack::from_dev_pack("multi-slot-nested-unanchored");
    pack.write_schema("changelog", MULTI_SLOT_NESTED_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, _item) = multi_slot_nested_corpus(&pack);

    let path = staged_path(&corpus, &task);
    let mut bytes = std::fs::read_to_string(&path).expect("read the staged copy");
    bytes.push_str("\n#### Stray heading\n");
    std::fs::write(&path, &bytes).expect("write the hand-broken staged copy");

    let findings = validate_text(&corpus, &task);
    let unanchored: Vec<&str> = findings
        .lines()
        .filter(|line| line.contains("conformance.item-heading-unanchored"))
        .collect();
    assert_eq!(
        unanchored.len(),
        1,
        "exactly the stray heading is reported — the item's own sub-labels are not: \
         {unanchored:?}\n--- full output ---\n{findings}",
    );
    let message = unanchored[0];
    assert!(
        message.contains("Stray heading"),
        "the finding names the offending heading: {message}",
    );
    assert!(
        message.contains("demote it to `#####`") && message.contains("{#<id>}"),
        "the located message carries BOTH repairs — the route this route-exempt \
         conformance code ships instead of a verb: {message}",
    );
    assert!(
        !findings.contains("conformance.item-slot-label-missing"),
        "the declared sub-labels still parse around the stray:\n{findings}",
    );
}

/// (viii) The declared bound, arm 2 — a **malformed** `{#id}` anchor. The heading
/// carries an anchor, so it is a nested item rather than a sub-label; the anchor is not
/// a slug, so the item has no identity and the parser reports
/// `conformance.item-anchor-malformed` naming the bad text, rather than falling back to
/// reading the heading as slot structure.
#[test]
fn a_malformed_anchor_on_a_deeper_heading_is_reported_not_guessed() {
    let pack = FixturePack::from_dev_pack("multi-slot-nested-malformed");
    pack.write_schema("changelog", MULTI_SLOT_NESTED_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, _item) = multi_slot_nested_corpus(&pack);

    let path = staged_path(&corpus, &task);
    let mut bytes = std::fs::read_to_string(&path).expect("read the staged copy");
    bytes.push_str("\n#### Stray heading  {#Bad Id}\n");
    std::fs::write(&path, &bytes).expect("write the hand-broken staged copy");

    let findings = validate_text(&corpus, &task);
    let malformed: Vec<&str> = findings
        .lines()
        .filter(|line| line.contains("conformance.item-anchor-malformed"))
        .collect();
    assert_eq!(
        malformed.len(),
        1,
        "the anchored-but-malformed heading is reported once: {malformed:?}\n\
         --- full output ---\n{findings}",
    );
    assert!(
        malformed[0].contains("{#Bad Id}") && malformed[0].contains("must be a slug"),
        "the finding names the bad anchor text and the rule it breaks: {}",
        malformed[0],
    );
    assert!(
        !findings.contains("conformance.item-slot-label-missing")
            && !findings.contains("conformance.item-heading-unanchored"),
        "an anchor — even a broken one — is what makes the heading a nested item, so \
         nothing is re-read as a missing sub-label:\n{findings}",
    );
}

// ============================================================================
// T3 — the READ side of the same class: a repeated declared field bullet
//
// T1 stops jigc's own writer from creating the shape. This catches one that already
// exists on disk — every corpus written by a pre-fix binary, and every out-of-band hand
// edit that copies a bullet and forgets to change its key. Through `1.0.0-rc.12` a
// declared key repeated in one sentinelled group had **no check at all**:
// `read_field_block_str` reported a repeated *undeclared* key once and pushed every
// repeat of a *declared* one into the parsed field list unchecked, so three values on a
// `0..1` enum were conformant to the tool, `jigc task validate` said nothing, and the
// pinned `doc show --format json` returned whichever value `find` reached first.
//
// The new `conformance.duplicate-field` is keyed per `(section/item, key)` — the
// granularity `conformance.unknown-field` and the duplicate-`{#id}` guard already use —
// and it **routes**: `conformance.*` is route-exempt rather than route-forbidden, and a
// repeated line is not a diagnosis the reader has to guess a direction for.
// ============================================================================

/// (ix) A repeated declared field bullet **blocks**, through the real binary: `jigc task
/// validate` exits non-zero, names the code once, and prints the repair as a route.
#[test]
fn a_repeated_declared_field_bullet_blocks_with_its_route() {
    let pack = FixturePack::from_dev_pack("multi-slot-duplicate-field");
    pack.write_schema("changelog", MULTI_SLOT_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, item) = multi_slot_corpus(&pack);
    corpus.set_field(&format!("{item}/status"), &task, "draft");
    // Author the task's own provisioned `commit` doc, so the gate's verdict below is
    // attributable to the duplicate bullet and to nothing else: the exit **flips** on the
    // hand edit rather than being non-zero all along.
    corpus.set_field(&format!("commit:{task}#header/type"), &task, "feat");
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "record a finding");
    let clean = corpus.jigc(&["task", "validate", &task]);
    assert!(
        clean.status.success(),
        "the baseline task must validate clean, so the flip below is the duplicate's:\n{}{}",
        String::from_utf8_lossy(&clean.stdout),
        String::from_utf8_lossy(&clean.stderr),
    );

    // The out-of-band hand edit: a second bullet for the same declared key — the shape a
    // human produces by copying a bullet, and the shape a pre-T1 binary wrote itself.
    let path = corpus
        .repo()
        .join(format!(".jigc/tasks/{task}/docs/changelog:findings-log.md"));
    let bytes = std::fs::read_to_string(&path).expect("read the staged copy");
    let patched = bytes.replace("- status: draft\n", "- status: draft\n- status: done\n");
    assert_ne!(bytes, patched, "the item's field group carries the bullet");
    std::fs::write(&path, &patched).expect("write the hand-broken staged copy");

    let out = corpus.jigc(&["task", "validate", &task]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a field carrying two values must FLIP the gate that passed one line earlier — \
         through rc.12 it stayed green:\n{text}",
    );
    let lines: Vec<&str> = text.lines().collect();
    let hits: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("conformance.duplicate-field"))
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "one repeated key is one defect and one repair, so it is reported ONCE:\n{text}",
    );
    let finding = lines[hits[0]];
    assert!(
        finding.starts_with("blocking · ") && finding.contains("`status`"),
        "the finding blocks and names the repeated key: {finding}",
    );
    let route = lines
        .get(hits[0] + 1)
        .copied()
        .unwrap_or_default()
        .trim_start();
    assert!(
        route.starts_with("route: ")
            && route.contains("`status:`")
            && route.contains("yours to hand-edit"),
        "the finding carries its route — delete the stray line, under the shipped \
         hand-repair sanction: {route}",
    );

    // And the machine surface carries the same defect at a key that discriminates the
    // field: `<type>:<slug>#<section>/<item>/<field-key>`.
    let out = corpus.jigc(&["task", "validate", &task, "--format", "json"]);
    let json = String::from_utf8_lossy(&out.stdout).to_string();
    let value: serde_json::Value = serde_json::from_str(&json).expect("a findings envelope");
    let targets: Vec<&str> = value["findings"]
        .as_array()
        .expect("a findings array")
        .iter()
        .filter(|f| f["code"] == "conformance.duplicate-field")
        .filter_map(|f| f["key"]["target"].as_str())
        .collect();
    assert_eq!(
        targets,
        vec![format!("{item}/status").as_str()],
        "the key names the repeated field's own leaf:\n{json}",
    );
}

// ============================================================================
// T4 — `--unset` of an absent field stops dead-ending
//
// The same item region, asked the removal question. Through `1.0.0-rc.12`, on a
// **present** item whose optional field is genuinely absent — the state a freshly
// minted item is in — `jigc doc set-field <item>/<field> --unset` answered
// `blocking · write.not-present — field group for field "status" is not present`,
// routed at `jigc doc show <doc>#<section> --task <id>`. That route is M44's
// **item-miss** route, and it is correct there: the agent addressed an item that was
// never minted, and the section read reveals the live ids. Here the item exists, so
// the route, followed verbatim, shows the item and changes nothing — re-running the
// write reproduces the identical error. A dead end
// (`design/surface-contract.md` → nothing dead-ends; `design/command-output-contract.md`
// → the universal advisory/blocking route floor, which a route that cannot terminate
// satisfies only on paper).
//
// The cause was never the route: `enrich_not_present_route` rewrites EVERY
// `write.not-present` from a write door to the containing-section read, and the two
// misses it cannot tell apart are the **item** miss (an id that was never minted) and
// the **field** miss (a present item, no such bullet). The second is not a miss at all
// — the state the agent asked for **already holds** — so it acks the no-op, the shipped
// idempotent-`rename` precedent (M48: ack the no-op instead of dressing an empty result
// as a failure), with an `already_absent` discriminator on both surfaces so the ack
// distinguishes *removed* from *was never there* (`design/surface-contract.md` law 1,
// the same shape as `create`'s `existed`).
//
// The axis is the **container depth** `--unset` addresses, not the reported repro:
// `{section field, item field}` × `{present container, absent container}`. The item
// arm is the repro; the section arm is its sibling one level out (a header scalar on
// the shipped `adr`); the absent-container column keeps M44's route, asserted here
// rather than assumed.
// ============================================================================

/// `jigc doc set-field <addr> --unset`'s success bit and full output.
fn unset(corpus: &TrialCorpus, address: &str, task: &str) -> (bool, String) {
    let out = corpus.jigc(&["doc", "set-field", address, "--unset", "--task", task]);
    (
        out.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        ),
    )
}

/// (x) The repro, fixed: on a **present** item whose optional field is absent,
/// `--unset` acks the no-op at exit 0, touches no bytes, is idempotent, and says on
/// both surfaces that the field was already absent — while a real removal on the same
/// leaf carries the discriminator's other value.
#[test]
fn an_unset_of_an_absent_item_field_acks_the_no_op() {
    let pack = FixturePack::from_dev_pack("unset-absent-item-field");
    pack.write_schema("changelog", MULTI_SLOT_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, item) = multi_slot_corpus(&pack);
    let address = format!("{item}/status");

    let before = staged_doc(&corpus, &task);
    assert!(
        !before.contains("- status:"),
        "precondition: a freshly minted item carries no `status` bullet:\n{before}",
    );

    let (ok, text) = unset(&corpus, &address, &task);
    assert!(
        ok,
        "`--unset` of an already-absent field on a PRESENT item is the state the agent \
         asked for — it must ack, not block with a route that reproduces itself:\n{text}",
    );
    assert!(
        text.contains(&address) && text.contains("already absent"),
        "the ack distinguishes `already absent` from a removal that happened:\n{text}",
    );
    assert_eq!(
        staged_doc(&corpus, &task),
        before,
        "a no-op changes no bytes",
    );

    // Idempotent: the second call reads exactly like the first.
    let (ok_again, again) = unset(&corpus, &address, &task);
    assert!(ok_again, "the no-op stays a no-op:\n{again}");
    assert_eq!(again, text, "the no-op ack is stable across re-runs");

    // The machine surface carries the same fact, keyed.
    let json = corpus.jigc_ok(&[
        "doc",
        "set-field",
        &address,
        "--unset",
        "--task",
        &task,
        "--format",
        "json",
    ]);
    let value: serde_json::Value = serde_json::from_str(&json).expect("the ack is JSON");
    assert_eq!(value["op"], "set-field", "the op is unchanged:\n{json}");
    assert_eq!(
        value["unset"], true,
        "the unset discriminator holds:\n{json}"
    );
    assert_eq!(
        value["already_absent"], true,
        "the envelope carries the fact the text prints:\n{json}",
    );

    // And a real removal on the same leaf answers the other value — so the key
    // discriminates rather than being a constant.
    corpus.set_field(&address, &task, "draft");
    let json = corpus.jigc_ok(&[
        "doc",
        "set-field",
        &address,
        "--unset",
        "--task",
        &task,
        "--format",
        "json",
    ]);
    let value: serde_json::Value = serde_json::from_str(&json).expect("the ack is JSON");
    assert_eq!(
        value["already_absent"], false,
        "a removal that removed something is not `already absent`:\n{json}",
    );
    assert!(
        !staged_doc(&corpus, &task).contains("- status:"),
        "the removal really removed the bullet",
    );
}

/// (xi) The sibling one container-level out: a **section**-level optional scalar on the
/// shipped `adr` — absent on a freshly created doc — acks the same no-op. The class is
/// the container depth `--unset` addresses, not the item block that reported it.
#[test]
fn an_unset_of_an_absent_section_field_acks_the_no_op() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("single-task", "clear an absent header scalar");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache strategy",
        "--task",
        &task,
    ]);
    let address = "adr:cache-strategy#status/cites-code";

    let before = corpus.jigc_ok(&["doc", "show", "adr:cache-strategy", "--task", &task]);
    assert!(
        !before.contains("cites-code"),
        "precondition: the optional header scalar is absent:\n{before}",
    );

    let (ok, text) = unset(&corpus, address, &task);
    assert!(
        ok,
        "the section arm of the same class must ack too — the field's absence is the \
         requested end state at every container depth:\n{text}",
    );
    assert!(
        text.contains("already absent"),
        "the ack names the fact:\n{text}",
    );
    assert_eq!(
        corpus.jigc_ok(&["doc", "show", "adr:cache-strategy", "--task", &task]),
        before,
        "a no-op changes no bytes",
    );
}

/// (xii) The absent-container column, unchanged: an `--unset` at an item id that was
/// never minted is still M44's **item miss** — `write.not-present` routed at the
/// containing section — and the route, run verbatim, answers. The no-op above must not
/// swallow this cell.
#[test]
fn an_unset_at_a_missing_item_keeps_the_containing_section_route() {
    let pack = FixturePack::from_dev_pack("unset-missing-item");
    pack.write_schema("changelog", MULTI_SLOT_SCHEMA)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, _item) = multi_slot_corpus(&pack);

    let (ok, text) = unset(
        &corpus,
        "changelog:findings-log#findings/9-9-9/status",
        &task,
    );
    assert!(
        !ok,
        "an item id that was never minted is a real miss and still blocks:\n{text}",
    );
    assert!(
        text.contains("write.not-present"),
        "the item miss keeps its code:\n{text}",
    );
    let route = text
        .lines()
        .find(|line| line.trim_start().starts_with("route: "))
        .unwrap_or_else(|| panic!("the block carries a route:\n{text}"));
    let command = route
        .split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("the route leads with a backticked command: {route}"));
    assert_eq!(
        command,
        format!("jigc doc show changelog:findings-log#findings --task {task}"),
        "M44's containing-section route is unchanged: {route}",
    );
    // Run it verbatim — the route the agent is handed answers.
    let args: Vec<&str> = command.split_whitespace().skip(1).collect();
    let shown = corpus.jigc_ok(&args);
    assert!(
        shown.contains("First finding"),
        "the route, run verbatim, reveals the section's live item ids:\n{shown}",
    );
}
