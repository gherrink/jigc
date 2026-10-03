//! # The address-grammar round-trip + settability-parity suite
//!
//! The contract property suite for the **address grammar** — pinned in
//! [doc-read-surface.md](../../../design/doc-read-surface.md) → The address grammar /
//! Three settability states (contract-version 7), and required by
//! [pinning.md](../../../implementation/pinning.md) §2 (*Address grammar*, both halves).
//! Revising that doc's statement and breaking this fence are visibly the same event —
//! the linkage the §2 naming rule requires.
//!
//! **Two halves, one seam.**
//!
//!  - **Law-1 half (nothing lies):** every address `jigc doc schema --format json`
//!    advertises, for **every doctype in both packs**, round-trips via **set → show →
//!    resolve** on the real binary — plus the documented alias forms (bare single-hop
//!    `#type` ↔ `#header/type`, bare singleton addresses, `{#id}` anchors). A
//!    projection that advertises a **dead** address (one no write accepts / no read
//!    resolves) goes red.
//!  - **Completeness half (advertised-set == accepted-set):** the projection's
//!    settability claim and the write path's actual acceptance agree, **per leaf**,
//!    three-valued — a settable leaf's `set-field` address is advertised **and** a
//!    write is accepted · an `id-from` leaf is advertised under its **owning verb**
//!    (`add-item` [+ `retitle-item` iff a string id-from]) **and** a `set-field` write
//!    is refused *with that verb's route* — with both those verbs driven under the
//!    **advertised payload key** (`write-key`, M48 Inc 7 — F12), never a re-typed flag ·
//!    a machine-maintained absolute is advertised
//!    by **nothing** and a write is refused. `milestone-record` carries the
//!    doctype-level caveat: it is machine-maintained **whole**, so **no** leaf is
//!    advertised and **every** write is refused — a doctype exclusion, not the per-leaf
//!    `set:`-kind rule.
//!
//! **No re-implemented walk.** The advertised set is read from the **emitted**
//! `doc schema --format json` bytes (never a test-side re-derivation of the projection
//! — [pinning.md](../../../implementation/pinning.md) §2, *Deliberately out*); the
//! accepted set is driven through the real `jigc` binary; and the leaf universe is
//! read from the **engine-loaded schema model** (the lib re-export
//! [`cli::pack::load_pack_schema`] over the shipped `[dev ▸ methodology]` composite) so
//! a leaf the projection drops **entirely** cannot hide.

use crate::support;

use std::collections::BTreeSet;
use std::process::Output;

use serde_json::{Value, json};
use support::trial_corpus::{FixturePack, State, TrialCorpus};

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema};
use engine::address::Address;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Leaf, MAX_NESTING_DEPTH, Schema, SectionBody};

// ---------------------------------------------------------------------------
// The doctype set — every doctype both packs ship, split by pack so the two
// walks run in parallel `#[test]`s. `milestone-record` is machine-maintained
// whole (its own test); `commit` is transient (no `doc create` — the task's own
// provisioned commit doc).
// ---------------------------------------------------------------------------

/// Dev-pack doctypes (`commit` included — the transient, provisioned on the task).
const DEV_DOCTYPES: &[&str] = &["adr", "spec", "prd", "arch-doc", "changelog", "commit"];

/// Methodology-pack persisted doctypes (`milestone-record` excluded — its own test).
const METHODOLOGY_DOCTYPES: &[&str] = &[
    "roadmap",
    "decisions-log",
    "deferral-ledger",
    "completion-record",
    "vision",
    "research",
    "idea",
    "dogfood-record",
    "planning-record",
];

// ---------------------------------------------------------------------------
// Concretization — the projection advertises placeheld addresses (`<slug>` for the
// doc, one `<id>` per enclosing item depth). We instantiate them against the real
// slug (from `doc create`) and the real item ids (from each `add-item` emission),
// so what we drive is the concrete form of the exact advertised address.
// ---------------------------------------------------------------------------

/// Instantiate a placeheld projection address: `<slug>` → the real slug, then each
/// `<id>` → its enclosing item's real minted id, in document order (one per depth).
fn concretize(placeheld: &str, slug: &str, ids: &[String]) -> String {
    let mut out = placeheld.replace("<slug>", slug);
    for id in ids {
        out = out.replacen("<id>", id, 1);
    }
    out
}

/// The value written for a field of `ty` — one that the write path accepts and the
/// read path round-trips. Panics on an unhandled type rather than silently skipping a
/// field a new type would introduce (a skipped leaf is exactly the hole this fences).
fn write_value(ty: &str, of: Option<&Vec<Value>>) -> String {
    match ty {
        "enum" => of
            .and_then(|members| members.first())
            .and_then(Value::as_str)
            .expect("an enum field advertises its members")
            .to_string(),
        "int" => "1".to_string(),
        "date" => "2020-01-01".to_string(),
        "bool" => "true".to_string(),
        "string" => "roundtrip-value".to_string(),
        "ref" => "[]".to_string(),
        "code-anchor" => "src/probe.rs#probe".to_string(),
        "owned-location" => "docs/probe-artifact.md".to_string(),
        other => panic!(
            "unhandled field type `{other}` in the round-trip suite — extend \
             `write_value`/`expected_show` so its advertised address is still covered"
        ),
    }
}

/// The `--format json` value a `doc show` of a `set-field`-written leaf must return —
/// the shape the read path re-parses the written value into.
fn expected_show(ty: &str, value: &str) -> Value {
    match ty {
        // A `ref` list value (`[]` here) projects as a JSON array.
        "ref" => json!([]),
        // Every scalar (string/date/int/enum/bool/code-anchor/owned-location) projects
        // as its string.
        _ => Value::String(value.to_string()),
    }
}

// ---------------------------------------------------------------------------
// Binary-driving helpers over the isolated both-packs corpus.
// ---------------------------------------------------------------------------

/// The pinned `--format json` projection of `ty`, as the **real binary** emits it —
/// the advertised-set source (never a test-side re-derivation).
fn projection(corpus: &TrialCorpus, ty: &str) -> Value {
    let raw = corpus.jigc_ok(&["doc", "schema", ty, "--format", "json"]);
    serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("the `{ty}` schema projection parses as json: {e}\n{raw}"))
}

/// Create an instance of `ty` through its own driving workflow, returning
/// `(task, slug)`. `commit` is the task's provisioned transient doc (slug = task id).
fn create_instance(corpus: &TrialCorpus, ty: &str) -> (String, String) {
    if ty == "commit" {
        // The transient commit doc is provisioned by any task-minting workflow; its
        // slug is the task id (`commit:<task-id>`).
        let task = corpus.start_workflow("record-decision", "probe the commit doc");
        return (task.clone(), task);
    }
    let workflow = match ty {
        "adr" => "record-decision",
        "spec" => "plan",
        "prd" => "project-setup",
        "arch-doc" => "architecture-documentation",
        "changelog" => "record-change",
        "roadmap" | "decisions-log" | "deferral-ledger" | "planning-record" => "planning",
        "completion-record" => "completion",
        "vision" => "form-vision",
        "research" => "do-research",
        "idea" => "park-idea",
        "dogfood-record" => "record-dogfood",
        other => panic!("no create workflow mapped for doctype `{other}`"),
    };
    let task = corpus.start_workflow(workflow, &format!("probe {ty}"));
    let created = corpus
        .jigc_ok(&[
            "doc",
            "create",
            ty,
            "--title",
            // A singleton's `# H1` is the schema's own, so a divergent `--title` is
            // refused since M48 (`write.title-ignored`) — the sweep asks the schema.
            &support::create_title(ty, &format!("Probe {ty}")),
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    let slug = created
        .strip_prefix(&format!("{ty}:"))
        .unwrap_or_else(|| panic!("`doc create {ty}` emits `{ty}:<slug>`; got `{created}`"))
        .to_string();
    (task, slug)
}

/// Assert `doc show <addr> --format json` **resolves** (exit 0, not a dead address)
/// and returns `expected`.
fn assert_show_parity(corpus: &TrialCorpus, task: &str, addr: &str, expected: &Value) {
    let out = corpus.jigc(&["doc", "show", addr, "--task", task, "--format", "json"]);
    assert!(
        out.status.success(),
        "advertised address `{addr}` did not resolve on read (a dead address):\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let got: Value = serde_json::from_str(&String::from_utf8_lossy(&out.stdout))
        .unwrap_or_else(|e| panic!("`doc show {addr}` json parses: {e}"));
    assert_eq!(
        &got, expected,
        "advertised address `{addr}` did not round-trip its written value",
    );
}

/// Assert an `Output` is a **routed block**: non-zero exit, the finding `code` and a
/// `route` needle on stderr — never a panic, never exit-0 silence.
fn assert_refused(out: &Output, code: &str, route_needle: &str, ctx: &str) {
    assert!(
        !out.status.success(),
        "{ctx}: expected a routed block ({code}), got exit 0:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(code),
        "{ctx}: the block names `{code}`; got:\n{stderr}",
    );
    assert!(
        stderr.contains(route_needle),
        "{ctx}: the block routes to `{route_needle}`; got:\n{stderr}",
    );
}

// ---------------------------------------------------------------------------
// The recursive walk — one pass builds the instance from the projection and, for
// every leaf, asserts BOTH halves at once: the advertised address round-trips
// (law 1) and its advertised settability state agrees with write acceptance
// (advertised-set == accepted-set).
// ---------------------------------------------------------------------------

/// Walk `ty`'s whole projection over a freshly created instance.
fn walk_doctype(corpus: &TrialCorpus, task: &str, ty: &str, slug: &str, proj: &Value) {
    for field in proj["fields"].as_array().expect("`fields` is an array") {
        assert_top_field(corpus, task, ty, slug, field);
    }
    for section in proj["sections"].as_array().expect("`sections` is an array") {
        assert_section(corpus, task, slug, &[], section);
    }
}

/// A top-level (header/body) field — never an `id-from` leaf (the doc's id is its H1
/// title), so exactly two states: **directly settable** (`set-field` advertised →
/// write accepted, state 1) or a **machine-maintained absolute** (no address → write
/// refused, state 3).
fn assert_top_field(corpus: &TrialCorpus, task: &str, ty: &str, slug: &str, field: &Value) {
    let id = field["id"].as_str().expect("a field has an id");
    let fty = field["type"].as_str().expect("a field has a type");
    let of = field["of"].as_array();
    match field.get("set-field").and_then(Value::as_str) {
        Some(set_field) => {
            // State 1: advertised and set-field accepted.
            let addr = concretize(set_field, slug, &[]);
            let value = write_value(fty, of);
            corpus.jigc_ok(&["doc", "set-field", &addr, "--value", &value, "--task", task]);
            assert_show_parity(corpus, task, &addr, &expected_show(fty, &value));
        }
        None => {
            // State 3: a machine-maintained absolute (the schema-version stamp). Its
            // section is the projection's own `section` key — probe its set-field
            // address and assert the write is refused.
            let section = field["section"].as_str().unwrap_or_else(|| {
                panic!(
                    "a machine-maintained top-level field carries its owning section; got:\n{field}"
                )
            });
            let probe = format!("{ty}:{slug}#{section}/{id}");
            let out = corpus.jigc(&["doc", "set-field", &probe, "--value", "9", "--task", task]);
            assert_refused(
                &out,
                "write.machine-maintained-field",
                "machine-maintained",
                &format!("set-field to the machine-maintained `{id}`"),
            );
        }
    }
}

/// A top-level section — a slot section (`set-slot` round-trips) or a repeatable
/// (delegated to [`assert_repeatable`]).
fn assert_section(corpus: &TrialCorpus, task: &str, slug: &str, ids: &[String], section: &Value) {
    match section["kind"].as_str().expect("a section has a kind") {
        "slot" => {
            let set_slot = section["set-slot"]
                .as_str()
                .expect("a slot section advertises set-slot");
            assert_slot_roundtrip(corpus, task, &concretize(set_slot, slug, ids));
        }
        "repeatable" => {
            let add_item = section["add-item"]
                .as_str()
                .expect("a repeatable advertises add-item");
            assert_repeatable(corpus, task, slug, ids, add_item, &section["item"]);
        }
        other => panic!("unknown section kind `{other}`"),
    }
}

/// A repeatable block (top-level or nested): add one item (minting a real id from the
/// advertised `add-item` address), then walk its item leaves. Proves the `add-item`
/// and (for a string `id-from`) `retitle-item` advertised addresses are live, and that
/// the minted item resolves on read.
fn assert_repeatable(
    corpus: &TrialCorpus,
    task: &str,
    slug: &str,
    ids: &[String],
    add_item_placeheld: &str,
    item: &Value,
) {
    let block_addr = concretize(add_item_placeheld, slug, ids);

    // The item's title is written to its `id-from` leaf — the item field carrying
    // `add-item`. An enum id-from must be a member; any other type takes a value of
    // its shape.
    let id_from = item["fields"]
        .as_array()
        .expect("item fields")
        .iter()
        .find(|f| f.get("add-item").is_some())
        .expect("a repeatable item has an id-from leaf carrying add-item");
    let title = write_value(
        id_from["type"].as_str().expect("id-from has a type"),
        id_from["of"].as_array(),
    );
    // The advertised PAYLOAD KEY, driven rather than re-typed (M48 Inc 7 — F12): the
    // id-source's `write-key` is the flag the mint is run with, so a projection naming
    // a key the verb does not take reddens here, not in the field.
    let write_key = id_from["write-key"]
        .as_str()
        .expect("an id-from leaf advertises its write-key");

    let item_addr = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            &block_addr,
            write_key,
            &title,
            "--task",
            task,
        ])
        .trim()
        .to_string();
    assert!(
        item_addr.starts_with(&format!("{block_addr}/")),
        "add-item at `{block_addr}` emitted an address outside the block: `{item_addr}`",
    );
    // The minted item resolves on read (the add-item address is not dead).
    let show = corpus.jigc(&[
        "doc", "show", &item_addr, "--task", task, "--format", "json",
    ]);
    assert!(
        show.status.success(),
        "the minted item `{item_addr}` did not resolve on read:\n{}",
        String::from_utf8_lossy(&show.stderr),
    );

    let item_id = item_addr
        .rsplit('/')
        .next()
        .expect("an item address ends in its id")
        .to_string();
    let mut inner_ids = ids.to_vec();
    inner_ids.push(item_id);

    for field in item["fields"].as_array().expect("item fields") {
        assert_item_field(corpus, task, slug, &inner_ids, &item_addr, field);
    }
    for slot in item["slots"].as_array().expect("item slots") {
        let set_slot = slot["set-slot"]
            .as_str()
            .expect("item slot advertises set-slot");
        assert_slot_roundtrip(corpus, task, &concretize(set_slot, slug, &inner_ids));
    }
    for nested in item["nested"].as_array().expect("item nested") {
        let add_item = nested["add-item"]
            .as_str()
            .expect("nested advertises add-item");
        assert_repeatable(corpus, task, slug, &inner_ids, add_item, &nested["item"]);
    }
}

/// An item leaf field — the three settability states:
///  - `set-field` advertised → write accepted (state 1);
///  - `add-item` advertised (the `id-from` leaf) → a `set-field` write is refused with
///    the id-source route; a string id-from also advertises `retitle-item` (exercised
///    live), an enum id-from advertises `add-item` alone (state 2);
///  - nothing advertised → the write is refused (state 3).
fn assert_item_field(
    corpus: &TrialCorpus,
    task: &str,
    slug: &str,
    ids: &[String],
    item_addr: &str,
    field: &Value,
) {
    let id = field["id"].as_str().expect("a field has an id");
    let fty = field["type"].as_str().expect("a field has a type");
    let of = field["of"].as_array();
    let leaf_addr = format!("{item_addr}/{id}");

    if let Some(set_field) = field.get("set-field").and_then(Value::as_str) {
        // State 1: advertised and set-field accepted. The advertised address, once
        // concretized, is exactly the leaf address.
        let addr = concretize(set_field, slug, ids);
        assert_eq!(
            addr, leaf_addr,
            "the advertised set-field address does not concretize to the leaf address",
        );
        let value = write_value(fty, of);
        corpus.jigc_ok(&["doc", "set-field", &addr, "--value", &value, "--task", task]);
        assert_show_parity(corpus, task, &addr, &expected_show(fty, &value));
        return;
    }

    if field.get("add-item").is_some() {
        // State 2: the id-from leaf — set-field refused, routed to its owning verb.
        let value = write_value(fty, of);
        let out = corpus.jigc(&[
            "doc",
            "set-field",
            &leaf_addr,
            "--value",
            &value,
            "--task",
            task,
        ]);
        match field.get("retitle-item").and_then(Value::as_str) {
            Some(retitle) => {
                // A string id-from: the retitle-item address is the item, and the
                // set-field refusal routes there.
                assert_eq!(
                    concretize(retitle, slug, ids),
                    item_addr,
                    "the advertised retitle-item address is not the item address",
                );
                assert_refused(
                    &out,
                    "write.id-from-field",
                    "retitle-item",
                    &format!("set-field to the string id-from leaf `{id}`"),
                );
                // Exercise the advertised retitle-item address: it resolves, and the
                // item stays addressable at its frozen id. The probe title is
                // multi-word for every string id-from — except the commit trailers
                // `key`, whose write doors enforce the git-trailer token shape (no
                // internal whitespace or colon; the confidence-audit wave —
                // sibling-hunt finding 6), so its probe is a well-shaped hyphenated
                // key. The discriminator mirrors the rule's own
                // (`doctype == "commit" && id-from == "key"`).
                let new_title = if item_addr.starts_with("commit:") && id == "key" {
                    "Retitled-Round-Trip"
                } else {
                    "Retitled Round Trip"
                };
                // Driven under the advertised payload key, like the mint above.
                let write_key = field["write-key"]
                    .as_str()
                    .expect("an id-from leaf advertises its write-key");
                corpus.jigc_ok(&[
                    "doc",
                    "retitle-item",
                    item_addr,
                    write_key,
                    new_title,
                    "--task",
                    task,
                ]);
                let show =
                    corpus.jigc(&["doc", "show", item_addr, "--task", task, "--format", "json"]);
                assert!(
                    show.status.success(),
                    "after retitle-item, the item `{item_addr}` no longer resolves (anchor not frozen):\n{}",
                    String::from_utf8_lossy(&show.stderr),
                );
            }
            None => {
                // An enum id-from: retitle refuses a member change, so the route is
                // remove + re-add — never retitle-item.
                assert_refused(
                    &out,
                    "write.id-from-field",
                    "remove-item",
                    &format!("set-field to the enum id-from leaf `{id}`"),
                );
                let stderr = String::from_utf8_lossy(&out.stderr);
                assert!(
                    !stderr.contains("retitle-item"),
                    "an enum id-from's block routes to remove+add, never retitle-item; got:\n{stderr}",
                );
            }
        }
        return;
    }

    // State 3: a machine-maintained absolute inside an item — nothing advertised, the
    // write refused.
    let out = corpus.jigc(&[
        "doc",
        "set-field",
        &leaf_addr,
        "--value",
        "9",
        "--task",
        task,
    ]);
    assert_refused(
        &out,
        "write.machine-maintained-field",
        "machine-maintained",
        &format!("set-field to the machine-maintained item leaf `{id}`"),
    );
}

/// Set a slot from stdin and assert `doc show` reads the exact prose back — the slot's
/// advertised `set-slot` address round-trips (and is not dead).
fn assert_slot_roundtrip(corpus: &TrialCorpus, task: &str, addr: &str) {
    let prose = "Round trip prose for the slot.";
    let out = corpus.jigc_stdin(
        &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
        prose,
    );
    assert!(
        out.status.success(),
        "advertised set-slot address `{addr}` did not accept a write:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_show_parity(corpus, task, addr, &Value::String(prose.to_string()));
}

/// **The cross-surface property (M55)**: on a created instance, the `doc list --task` row's
/// `title` and `fields` equal the `doc show --task` whole-doc serve's — both read off the
/// emitted json of the real binary. `fields` is one helper's map on both surfaces
/// (`cli::doc::header_fields_json`) and `title` one reader's, so a divergence here is a
/// second rule grown on one of them (`design/findings-channel.md` → 5).
fn assert_row_matches_show(corpus: &TrialCorpus, task: &str, ty: &str, slug: &str) {
    let id = format!("{ty}:{slug}");
    let listed: Value = serde_json::from_str(
        &corpus.jigc_ok(&["doc", "list", ty, "--task", task, "--format", "json"]),
    )
    .unwrap_or_else(|e| panic!("`doc list {ty} --task` json parses: {e}"));
    let row = listed["docs"]
        .as_array()
        .expect("`docs` is an array")
        .iter()
        .find(|row| row["id"] == Value::String(id.clone()))
        .unwrap_or_else(|| panic!("the staged listing carries `{id}`; got:\n{listed:#}"));
    let shown: Value = serde_json::from_str(
        &corpus.jigc_ok(&["doc", "show", &id, "--task", task, "--format", "json"]),
    )
    .unwrap_or_else(|e| panic!("`doc show {id} --task` json parses: {e}"));
    assert!(
        row["fields"].is_object(),
        "`{id}` serves whole on `doc show`, so it parses and its staged row carries a \
         `fields` map; row:\n{row:#}",
    );
    assert_eq!(
        (&row["title"], &row["fields"]),
        (&shown["title"], &shown["fields"]),
        "the `doc list --task` row and the `doc show --task` serve of `{id}` disagree",
    );
}

// ---------------------------------------------------------------------------
// The tests.
// ---------------------------------------------------------------------------

/// Dev-pack doctypes: every advertised address round-trips and the settability
/// three-state parity holds, on the real binary.
#[test]
fn dev_pack_addresses_round_trip_and_settability_parity_holds() {
    let corpus = TrialCorpus::build(State::Fresh);
    for &ty in DEV_DOCTYPES {
        let (task, slug) = create_instance(&corpus, ty);
        let proj = projection(&corpus, ty);
        walk_doctype(&corpus, &task, ty, &slug, &proj);
        assert_row_matches_show(&corpus, &task, ty, &slug);
    }
}

/// Methodology-pack doctypes: the same walk, run in a parallel `#[test]` (both packs
/// compose in either corpus; the split is only for parallelism).
#[test]
fn methodology_pack_addresses_round_trip_and_settability_parity_holds() {
    let corpus = TrialCorpus::build(State::Fresh);
    for &ty in METHODOLOGY_DOCTYPES {
        let (task, slug) = create_instance(&corpus, ty);
        let proj = projection(&corpus, ty);
        walk_doctype(&corpus, &task, ty, &slug, &proj);
        assert_row_matches_show(&corpus, &task, ty, &slug);
    }
}

/// The documented alias forms resolve to the same node as their advertised form:
/// bare single-hop `#type` ↔ `#header/type` (write-path alias), a bare singleton
/// address ↔ its slug'd form, and the `{#id}` anchor (an item stays addressable by
/// its frozen minted id across a retitle).
#[test]
fn documented_alias_forms_round_trip() {
    let corpus = TrialCorpus::build(State::Fresh);

    // (1) bare single-hop `#type` ↔ `#header/type` — the commit `type` header field.
    // The write accepts the bare single-hop form; the read resolves the qualified one.
    let commit_task = corpus.start_workflow("record-decision", "probe commit aliases");
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{commit_task}#type"),
        "--value",
        "feat",
        "--task",
        &commit_task,
    ]);
    let qualified = corpus.jigc(&[
        "doc",
        "show",
        &format!("commit:{commit_task}#header/type"),
        "--task",
        &commit_task,
        "--format",
        "json",
    ]);
    assert!(
        qualified.status.success(),
        "the `#header/type` form resolves"
    );
    assert_eq!(
        String::from_utf8_lossy(&qualified.stdout).trim(),
        "\"feat\"",
        "the bare single-hop write is read back through the section-qualified form — \
         both address the same header field",
    );

    // (2) bare singleton address ↔ slug'd form — a vision written bare reads back slug'd.
    let vision_task = corpus.start_workflow("form-vision", "probe singleton aliases");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "vision",
        "--title",
        "Vision",
        "--task",
        &vision_task,
    ]);
    corpus.jigc_stdin(
        &[
            "doc",
            "set-slot",
            "vision#thesis",
            "--from-file",
            "-",
            "--task",
            &vision_task,
        ],
        "A deterministic context compiler.",
    );
    let bare = corpus.jigc(&[
        "doc",
        "show",
        "vision#thesis",
        "--task",
        &vision_task,
        "--format",
        "json",
    ]);
    let slugged = corpus.jigc(&[
        "doc",
        "show",
        "vision:vision#thesis",
        "--task",
        &vision_task,
        "--format",
        "json",
    ]);
    assert!(
        bare.status.success() && slugged.status.success(),
        "both the bare singleton and slug'd forms resolve",
    );
    assert_eq!(
        String::from_utf8_lossy(&bare.stdout),
        String::from_utf8_lossy(&slugged.stdout),
        "the bare singleton address `vision#thesis` reads the same node as `vision:vision#thesis`",
    );

    // (3) the `{#id}` anchor — an item keeps its frozen minted id across a retitle, so
    // the address that named it before still resolves it after.
    let cl_task = corpus.start_workflow("record-change", "probe anchor aliases");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "changelog",
        "--title",
        "Changelog",
        "--task",
        &cl_task,
    ]);
    let item = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.0.0",
            "--task",
            &cl_task,
        ])
        .trim()
        .to_string();
    assert_eq!(
        item, "changelog:changelog#releases/1-0-0",
        "the minted id anchors the item at `1-0-0`",
    );
    corpus.jigc_ok(&[
        "doc",
        "retitle-item",
        &item,
        "--title",
        "2.0.0",
        "--task",
        &cl_task,
    ]);
    let after = corpus.jigc(&["doc", "show", &item, "--task", &cl_task, "--format", "json"]);
    assert!(
        after.status.success(),
        "the `{{#id}}` anchor keeps `{item}` addressable after a retitle:\n{}",
        String::from_utf8_lossy(&after.stderr),
    );
}

/// `milestone-record` is machine-maintained **whole** — the doctype-level caveat the
/// parity assertion must carry: its projection advertises **no** write address, and a
/// `set-field` to any of its leaves is refused (the `write.machine-maintained` doctype
/// guard), not the per-leaf `set:`-kind rule.
#[test]
fn milestone_record_is_machine_maintained_whole() {
    let corpus = TrialCorpus::build(State::Fresh);
    let proj = projection(&corpus, "milestone-record");

    // Advertised-set is empty: no write verb key anywhere in the projection.
    let raw = serde_json::to_string(&proj).expect("re-serialize the projection");
    for verb in ["set-field", "set-slot", "add-item", "retitle-item"] {
        assert!(
            !raw.contains(&format!("\"{verb}\"")),
            "a machine-maintained-whole doctype advertises no `{verb}` address; got:\n{raw}",
        );
    }

    // Accepted-set is empty: a write to any leaf is refused by the doctype guard,
    // before any instance need exist.
    let task = corpus.start_workflow("record-decision", "probe milestone-record");
    let out = corpus.jigc(&[
        "doc",
        "set-field",
        "milestone-record:some#tasks/foo/task-id",
        "--value",
        "x",
        "--task",
        &task,
    ]);
    assert_refused(
        &out,
        "write.machine-maintained",
        "milestone",
        "set-field to a milestone-record leaf",
    );
}

// ---------------------------------------------------------------------------
// The engine-loaded-schema anchor — the leaf universe the parity is "over" comes from
// the shipped schema model, so a leaf the projection drops entirely cannot hide.
// ---------------------------------------------------------------------------

/// The exact `[dev ▸ methodology]` composite the shipped binary loads, built the
/// CWD-free way (never `make_pack()`).
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every leaf of an engine-loaded schema, as a canonical `kind:path` key — the leaf
/// **universe**, enumerated (never the address/settability logic re-implemented).
fn engine_leaf_keys(schema: &Schema) -> BTreeSet<String> {
    fn walk_block(id_from_path: &str, block: &[Leaf], keys: &mut BTreeSet<String>) {
        for leaf in block {
            match leaf {
                Leaf::Field(field) => {
                    keys.insert(format!("field:{id_from_path}/{}", field.id));
                }
                Leaf::Slot { id, .. } => {
                    keys.insert(format!("slot:{id_from_path}/{id}"));
                }
                Leaf::Repeatable { id, repeatable } => {
                    walk_block(&format!("{id_from_path}/{id}"), &repeatable.block, keys);
                }
            }
        }
    }

    let mut keys = BTreeSet::new();
    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple { slot, fields } => {
                if slot.is_some() {
                    keys.insert(format!("slot:{}", section.id));
                }
                for field in fields {
                    keys.insert(format!("field:{}/{}", section.id, field.id));
                }
            }
            SectionBody::Repeatable { repeatable } => {
                walk_block(&section.id, &repeatable.block, &mut keys);
            }
        }
    }
    keys
}

/// Every leaf the projection carries, as the same canonical `kind:path` key —
/// consumed from the emitted `doc schema --format json`, never re-derived.
fn projection_leaf_keys(proj: &Value) -> BTreeSet<String> {
    fn walk_item(path: &str, item: &Value, keys: &mut BTreeSet<String>) {
        for field in item["fields"].as_array().into_iter().flatten() {
            keys.insert(format!("field:{path}/{}", field["id"].as_str().unwrap()));
        }
        for slot in item["slots"].as_array().into_iter().flatten() {
            keys.insert(format!("slot:{path}/{}", slot["id"].as_str().unwrap()));
        }
        for nested in item["nested"].as_array().into_iter().flatten() {
            let id = nested["id"].as_str().unwrap();
            walk_item(&format!("{path}/{id}"), &nested["item"], keys);
        }
    }

    let mut keys = BTreeSet::new();
    for field in proj["fields"].as_array().into_iter().flatten() {
        let section = field["section"].as_str().unwrap();
        keys.insert(format!("field:{section}/{}", field["id"].as_str().unwrap()));
    }
    for section in proj["sections"].as_array().into_iter().flatten() {
        let id = section["id"].as_str().unwrap();
        match section["kind"].as_str().unwrap() {
            "slot" => {
                keys.insert(format!("slot:{id}"));
            }
            "repeatable" => walk_item(id, &section["item"], &mut keys),
            other => panic!("unknown section kind `{other}`"),
        }
    }
    keys
}

/// The walk lists above are hand consts, split per pack so the two walks run in
/// parallel — and a hand list is exactly what lets a NEW doctype ship un-walked
/// (confidence-audit minor item 2, the VERDICT-declared residue). This union-equality
/// fence closes them against the registry: the two walk lists plus the one declared
/// exclusion (`milestone-record`, machine-maintained whole — its own test above)
/// together equal **exactly** the doctype set the shipped composite enumerates, so a
/// doctype added to (or renamed in) either pack reddens here until it joins a walk or
/// earns its own declared exclusion.
#[test]
fn the_walk_lists_union_equals_the_composite_registry() {
    let pack = composite();
    let registry: BTreeSet<String> = pack
        .list(PackResourceKind::Schemas)
        .iter()
        .map(|id| id.as_str().to_string())
        .collect();
    let mut walked: BTreeSet<String> = DEV_DOCTYPES
        .iter()
        .chain(METHODOLOGY_DOCTYPES)
        .map(|s| s.to_string())
        .collect();
    // The one declared exclusion: machine-maintained whole, walked by
    // `milestone_record_is_machine_maintained_whole` instead of the per-leaf parity.
    // (`planning-record`'s M49 Inc-9 / T5 exclusion expired at T6, which gave it the
    // `planning` workflow's `allows-create` — it now walks with the rest.)
    walked.insert("milestone-record".to_string());
    assert_eq!(
        walked, registry,
        "the parity walk's hand lists (DEV_DOCTYPES ∪ METHODOLOGY_DOCTYPES ∪ the \
         milestone-record exclusion) drifted from the composite registry — a doctype \
         shipped without joining a settability walk, or a walked doctype left the packs",
    );
}

/// For **every** doctype both packs ship, the projection's leaf set equals the
/// engine-loaded schema's leaf set — so the parity above is genuinely *over the
/// engine-loaded schema model*: a leaf the projection dropped entirely would redden
/// here rather than escape the address round-trip unseen.
#[test]
fn projection_leaf_set_matches_the_engine_loaded_schema() {
    let corpus = TrialCorpus::build(State::Fresh);
    let pack = composite();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .unwrap_or_else(|e| panic!("schema `{}` reads back: {e}", id.as_str()));
        let schema = load_pack_schema(&pack, &bytes)
            .unwrap_or_else(|e| panic!("schema `{}` loads: {e:?}", id.as_str()));
        let proj = projection(&corpus, &schema.ty);
        assert_eq!(
            projection_leaf_keys(&proj),
            engine_leaf_keys(&schema),
            "the `{}` projection's leaf set diverged from its engine-loaded schema — a \
             leaf is advertised that the schema does not carry, or (the hiding case) a \
             schema leaf the projection dropped",
            schema.ty,
        );
    }
}

// ---------------------------------------------------------------------------
// The nesting ceiling — a MANUFACTURED SHAPE SPACE, not a registry enumeration
// (M49 Increment 5, T1). See the module doc's *The nesting ceiling* section for
// why the shipped registry cannot supply this arm's set.
// ---------------------------------------------------------------------------

/// The workflow whose create-gate admits the manufactured doctype — `creates-task:
/// true`, so `jigc start --workflow` mints the task the writes below address.
const NESTING_WORKFLOW: &str = "\
---
when: probe the nesting ceiling
description: Author the nesting probe.
usage: the address grammar's depth ceiling needs a doctype nested to the cap.
creates-task: true
allows-create: [{type: changelog, as: probe}]
---
{{ include: step:finalize }}
";

/// A doctype schema nested to exactly `depth` repeatable levels, **generated** from the
/// depth rather than hand-written — so the fixture follows
/// [`engine::schema::MAX_NESTING_DEPTH`] instead of restating it.
///
/// Every level carries the block's `id-from` leaf (`label`) and one directly settable
/// field (`note`); the deepest level also carries a prose slot (`detail`), so the arm
/// drives an advertised `set-slot` address at full depth. A slot is declared **only**
/// at the deepest level: a leading bare-prose slot on a block that also nests would
/// swallow the nested item headings (`crates/cli/packs/dev/schemas/changelog.yaml` → review
/// finding B1), which is a rendering question, not a ceiling one.
///
/// It is written over the dev pack's `changelog` slot — a fixture pack **replaces** a
/// schema rather than registering a new doctype, so nothing outside the schema file has
/// to be manufactured too.
fn nested_schema(depth: usize) -> String {
    let mut yaml = String::from(
        "type: changelog
id-from: title
description: A manufactured nesting probe — one repeatable per level, to the cap.
usage: the address grammar's depth ceiling needs a doctype nested to the cap.
sections:
  - id: level-1
    repeatable:
      id-from: label
      block:
",
    );
    // The block-item indent of level 1; each nested level sits six spaces deeper.
    let mut indent = 8usize;
    for level in 1..=depth {
        let pad = " ".repeat(indent);
        yaml.push_str(&format!("{pad}- {{ id: label, type: string }}\n"));
        yaml.push_str(&format!(
            "{pad}- {{ id: note, type: string, optional: true }}\n"
        ));
        if level == depth {
            yaml.push_str(&format!(
                "{pad}- {{ id: detail, slot: {{ hint: \"The note.\" }} }}\n"
            ));
        } else {
            let next = level + 1;
            yaml.push_str(&format!("{pad}- id: level-{next}\n"));
            yaml.push_str(&format!("{pad}  repeatable:\n"));
            yaml.push_str(&format!("{pad}    id-from: label\n"));
            yaml.push_str(&format!("{pad}    block:\n"));
            indent += 6;
        }
    }
    yaml
}

/// Every address the projection advertises, collected from the **emitted** json by the
/// write-verb keys that carry one — so a new address key joins this walk by existing,
/// never by being listed here.
fn advertised_addresses(proj: &Value) -> Vec<String> {
    const ADDRESS_KEYS: &[&str] = &["set-field", "set-slot", "add-item", "retitle-item"];
    fn walk(value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (key, child) in map {
                    match child.as_str() {
                        Some(addr) if ADDRESS_KEYS.contains(&key.as_str()) => {
                            out.push(addr.to_string())
                        }
                        _ => walk(child, out),
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|item| walk(item, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(proj, &mut out);
    out
}

/// The `/`-separated hop count of an address's fragment — the budget the grammar caps.
fn fragment_hops(addr: &str) -> usize {
    addr.split_once('#')
        .map(|(_, frag)| frag.split('/').count())
        .unwrap_or(0)
}

/// A doctype nested **to the cap** advertises only addresses the grammar admits — and
/// every one of them round-trips `set` → `show` on the real binary.
///
/// The tightness of the cap is proven against the **grammar itself**, not a second
/// constant: the leaf address a doctype one level deeper would advertise is refused by
/// [`Address::parse`]. So the ceiling is exactly where the addressing budget runs out,
/// and a loader that admitted one more level would be advertising a dead address.
#[test]
fn a_doctype_nested_to_the_cap_advertises_only_addressable_addresses() {
    let pack = FixturePack::from_dev_pack("nesting-at-cap");
    pack.write_schema("changelog", &nested_schema(MAX_NESTING_DEPTH))
        .write_workflow("nest-probe", NESTING_WORKFLOW);
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
    let task = corpus.start_workflow("nest-probe", "probe the nesting ceiling");
    let created = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "changelog",
            "--title",
            "Nesting probe",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    let slug = created
        .strip_prefix("changelog:")
        .unwrap_or_else(|| {
            panic!("`doc create changelog` emits `changelog:<slug>`; got `{created}`")
        })
        .to_string();

    let proj = projection(&corpus, "changelog");
    let advertised = advertised_addresses(&proj);
    assert!(
        !advertised.is_empty(),
        "the manufactured projection advertises no address at all — the fixture is not \
         exercising the ceiling",
    );

    // Law 1, at the grammar: nothing advertised is unaddressable.
    for addr in &advertised {
        Address::parse(addr).unwrap_or_else(|e| {
            panic!(
                "`doc schema changelog --format json` advertises `{addr}`, which the \
                 address grammar REJECTS ({e}) — the projection is offering an address \
                 no write path can take"
            )
        });
    }

    // The fixture really reaches the cap: the deepest advertised address is a leaf
    // write at `MAX_NESTING_DEPTH` (a section hop, two hops per level, a leaf).
    let deepest = advertised
        .iter()
        .max_by_key(|addr| fragment_hops(addr))
        .expect("a non-empty advertised set has a deepest member")
        .clone();
    assert_eq!(
        fragment_hops(&deepest),
        2 * MAX_NESTING_DEPTH + 1,
        "the deepest advertised address `{deepest}` is not a leaf write at the cap — \
         the generated fixture drifted from `MAX_NESTING_DEPTH`",
    );

    // And the cap is TIGHT: one level deeper is refused by the grammar itself.
    let parent = deepest
        .rsplit_once('/')
        .expect("the deepest address has a leaf hop")
        .0;
    let one_deeper = format!("{parent}/level-deeper/<id>/note");
    assert!(
        Address::parse(&one_deeper).is_err(),
        "`{one_deeper}` — the leaf address a doctype one level deeper would advertise — \
         parses, so the loader cap is BELOW the grammar's budget rather than derived \
         from it",
    );

    // Both halves of this suite, over the manufactured shape: every advertised address
    // round-trips set → show, and the settability parity holds at every depth.
    walk_doctype(&corpus, &task, "changelog", &slug, &proj);
}

/// A doctype nested **one level past the cap** is refused at pack-load, and the refusal
/// names the depth it read, the number a pack author must meet, and the constraint that
/// binds — the **address hop budget**, not the `H6` render ceiling (which stopped being
/// the binding cap when the loader was derived from the grammar).
#[test]
fn a_doctype_nested_one_level_deeper_is_refused_at_pack_load() {
    let pack = FixturePack::from_dev_pack("nesting-past-cap");
    // Build the corpus over a pack that LOADS (at the cap), then reshape it: the
    // refusal under test is pack-load's, on the next invocation, not `setup`'s.
    pack.write_schema("changelog", &nested_schema(MAX_NESTING_DEPTH))
        .write_workflow("nest-probe", NESTING_WORKFLOW);
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);
    pack.write_schema("changelog", &nested_schema(MAX_NESTING_DEPTH + 1));

    let out = corpus.jigc(&["doc", "schema", "changelog"]);
    assert!(
        !out.status.success(),
        "a pack nested past the cap loaded at exit 0:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let depth = MAX_NESTING_DEPTH + 1;
    assert!(
        stderr.contains(&format!("depth {depth}")),
        "the refusal names the offending depth ({depth}); got:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("{MAX_NESTING_DEPTH} levels")),
        "the refusal names the number a pack author must meet ({MAX_NESTING_DEPTH} \
         levels); got:\n{stderr}",
    );
    assert!(
        stderr.contains("address hops"),
        "the refusal names the constraint that BINDS — the address hop budget — so a \
         pack author reads why the cap is where it is; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("H6"),
        "the refusal no longer blames the H6 render ceiling: it is not the binding cap \
         (a leaf write runs out of address hops first); got:\n{stderr}",
    );
}
