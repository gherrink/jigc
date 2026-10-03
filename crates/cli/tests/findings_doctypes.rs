//! M55 Increment 7 / T4 — **the acceptance of the two finding doctypes**, `jigc-feedback`
//! and `inconsistency` ([findings-channel.md](../../../design/findings-channel.md) → 1, 5,
//! 7), driven through the real binary on a fresh adopted corpus.
//!
//! - **(a)** `jigc doc schema` serves both in every `--format` at `schema-version 1`, with
//!   `status` defaulted `open`.
//! - **(b)** **The seed fence's predicate** (§7): hand-placed, stamped, conformant files —
//!   a `repro` holding a fenced `#`-led line, three `sides` — `jigc ingest` cold into the
//!   absent homes as `adoptable`, `jigc validate --format json` reports `"findings": []`
//!   at exit 0, and no byte of either file moves.
//! - **(c)** Increment 6's triage query ([`doc_list_triage::TRIAGE`]) re-driven on real
//!   `jigc-feedback` instances: one `doc list` through a real `jq`, a row with no `status:`
//!   line counted open by projection (§5).
//! - **(d)** The `inconsistency.sides/says` heading-depth gate, which the shipped-verb
//!   sweep in `item_slot_ceiling_axis.rs` cannot reach (no `migrate-inconsistency` ships —
//!   its `UNREACHABLE` entry cites the test here), driven through the shipped
//!   `report-inconsistency` workflow.
//!
//! [`doc_list_triage::TRIAGE`]: crate::doc_list_triage::TRIAGE

use crate::doc_list_triage::{TRIAGE, jq};
use crate::support;

use serde_json::{Value, json};
use support::trial_corpus::{State, TrialCorpus};

/// The two doctypes and the home each lays its instances out under (`docs-root` is
/// `docs` in every trial state).
const HOMES: [(&str, &str); 2] = [
    ("jigc-feedback", "docs/jigc-feedback"),
    ("inconsistency", "docs/inconsistencies"),
];

/// A conformant `jigc-feedback` in the canonical bytes `doc create` + the setters write:
/// stamped `schema-version: 1`, and a `repro` whose fenced block opens on a `#`-led line —
/// a heading anywhere outside the fence.
const FEEDBACK: &str = "\
---
kind: bug
found-in: milestone:findings-channel
jigc-version: 1.0.0-rc.23
status: open
date: 2026-10-03
schema-version: 1
---

# Ingest drops a fence

## Description

A fenced block in a slot was read as headings.

## Repro

```sh
# a hash-led line
jigc ingest
```

## Resolution
";

/// A conformant `inconsistency` with three `sides` — a path, a doc path and an address —
/// one of them carrying `says`.
const INCONSISTENCY: &str = "\
---
kind: code-doc
status: open
date: 2026-10-03
schema-version: 1
---

# Sides disagree

## Sides

### src/side1.rs  {#src-side1-rs}

The function returns early.

### docs/a.md  {#docs-a-md}

### adr:foo#decision  {#adrfoodecision}

## Description

The code and two docs state different behaviour.

## Evidence



## Resolution
";

/// Write `bytes` at `rel` and commit it with plain git — a hand placement, no jigc verb.
fn place(corpus: &TrialCorpus, rel: &str, bytes: &str) {
    let path = corpus.repo().join(rel);
    std::fs::create_dir_all(path.parent().expect("a placed file has a parent"))
        .expect("create the home");
    std::fs::write(&path, bytes).expect("place the file");
    corpus.git(&["add", "--", rel]);
    corpus.git(&["commit", "-q", "-m", &format!("place {rel} by hand")]);
}

/// Run `jigc ingest --format json` and assert each of `rels` was classified `adoptable`
/// against `doctype` and adopted.
fn ingest_adopts(corpus: &TrialCorpus, expected: &[(&str, &str)]) {
    let report: Value = serde_json::from_str(&corpus.jigc_ok(&["ingest", "--format", "json"]))
        .expect("the ingest report is json");
    let rows = report["rows"].as_array().expect("`rows` is an array");
    for (rel, doctype) in expected {
        let row = rows
            .iter()
            .find(|row| row["file"] == json!(rel))
            .unwrap_or_else(|| panic!("ingest classifies `{rel}`; report:\n{report:#}"));
        assert_eq!(
            (&row["best_match"], &row["verdict"], &row["adopted"]),
            (&json!(doctype), &json!("adoptable"), &json!(true)),
            "`{rel}` is adoptable as `{doctype}` and adopted; row:\n{row:#}",
        );
    }
}

/// (a) **`doc schema` states `schema-version 1` and `status` defaulted `open` in every
/// format** — `agent`, `json` and `human`, the verb's whole `--format` set.
#[test]
fn doc_schema_serves_version_one_with_status_defaulted_open_in_every_format() {
    let corpus = TrialCorpus::build(State::Fresh);
    for (doctype, home) in HOMES {
        let schema: Value =
            serde_json::from_str(&corpus.jigc_ok(&["doc", "schema", doctype, "--format", "json"]))
                .expect("doc schema json parses");
        assert_eq!(schema["type"], json!(doctype));
        assert_eq!(schema["schema-version"], json!(1), "`{doctype}` is at v1");
        assert_eq!(
            schema["home"]["path"],
            json!(format!("{home}/<slug>.md")),
            "`{doctype}` homes under `{home}/`",
        );
        let status = schema["fields"]
            .as_array()
            .expect("`fields` is an array")
            .iter()
            .find(|field| field["id"] == json!("status"))
            .unwrap_or_else(|| panic!("`{doctype}` declares `status`; schema:\n{schema:#}"));
        assert_eq!(
            (&status["default"], &status["author-required"]),
            (&json!("open"), &json!(false)),
            "`{doctype}.status` defaults `open` and is not the author's to supply",
        );

        for format in ["agent", "human"] {
            let text = corpus.jigc_ok(&["doc", "schema", doctype, "--format", format]);
            assert_eq!(
                text.lines().next(),
                Some(format!("doctype: {doctype} (schema-version 1)").as_str()),
                "`--format {format}` heads with the version:\n{text}",
            );
            let status_line = text
                .lines()
                .find(|line| line.trim_start().starts_with("- status:"))
                .unwrap_or_else(|| panic!("`--format {format}` lists `status`:\n{text}"));
            assert!(
                status_line.contains("(default: open)"),
                "`--format {format}` states the default: {status_line}",
            );
        }
    }
}

/// (b) **The seed fence's predicate** (findings-channel.md → 7): one stamped, conformant
/// instance of each doctype, placed by hand into its absent home and committed, is
/// classified `adoptable` and adopted by a cold `jigc ingest`; `jigc validate --format
/// json` then reports zero findings at exit 0, and both files keep their placed bytes.
#[test]
fn hand_placed_instances_ingest_cold_and_validate_with_zero_findings() {
    let corpus = TrialCorpus::build(State::Fresh);
    for (_, home) in HOMES {
        assert!(
            !corpus.repo().join(home).exists(),
            "`{home}/` is absent before the placement — the ingest is cold",
        );
    }
    let feedback = "docs/jigc-feedback/ingest-drops-a-fence.md";
    let inconsistency = "docs/inconsistencies/sides-disagree.md";
    place(&corpus, feedback, FEEDBACK);
    place(&corpus, inconsistency, INCONSISTENCY);

    ingest_adopts(
        &corpus,
        &[
            (feedback, "jigc-feedback"),
            (inconsistency, "inconsistency"),
        ],
    );

    let out = corpus.jigc(&["validate", "--format", "json"]);
    let report: Value = serde_json::from_slice(&out.stdout).expect("validate json parses");
    assert!(
        out.status.success(),
        "validate exits 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        report["findings"],
        json!([]),
        "the store reports zero findings over the adopted pair; report:\n{report:#}",
    );

    for (rel, placed) in [(feedback, FEEDBACK), (inconsistency, INCONSISTENCY)] {
        assert_eq!(
            support::trial_corpus::read(&corpus.repo(), rel),
            placed,
            "ingest and validate leave `{rel}` byte-identical to the placed bytes",
        );
    }
}

/// One hand-placed `jigc-feedback`, its `status:` line written only when `status` is set.
fn feedback(title: &str, found_in: &str, status: Option<&str>) -> String {
    let status_line = status.map_or(String::new(), |s| format!("status: {s}\n"));
    let resolution = if status == Some("resolved") {
        "\nFixed by the fence that pins it.\n"
    } else {
        ""
    };
    format!(
        "---\nkind: bug\nfound-in: {found_in}\njigc-version: 1.0.0-rc.23\n{status_line}\
         date: 2026-10-03\nschema-version: 1\n---\n\n# {title}\n\n## Description\n\n\
         {title}, as observed.\n\n## Repro\n\n```sh\n# a hash-led line\njigc start\n```\n\n\
         ## Resolution\n{resolution}"
    )
}

/// (c) **Increment 6's triage, on real `jigc-feedback` instances.** Four are placed over two
/// `found-in` values — in `milestone:alpha` one open, one with no `status:` line and one
/// resolved; in `review:beta` one open — and ingested. One `doc list jigc-feedback --format
/// json` fed verbatim to a real `jq` with [`TRIAGE`] yields exactly the two open groups, the
/// status-less row counted open through its schema default, the resolved one in neither.
#[test]
fn the_triage_query_groups_real_open_feedback_by_found_in() {
    let corpus = TrialCorpus::build(State::Fresh);
    let rows = [
        ("Alpha open", "milestone:alpha", Some("open")),
        ("Alpha unstated", "milestone:alpha", None),
        ("Alpha resolved", "milestone:alpha", Some("resolved")),
        ("Beta open", "review:beta", Some("open")),
    ];
    let mut placed = Vec::new();
    for (title, found_in, status) in rows {
        let rel = format!(
            "docs/jigc-feedback/{}.md",
            title.to_lowercase().replace(' ', "-")
        );
        place(&corpus, &rel, &feedback(title, found_in, status));
        placed.push(rel);
    }
    let adopted: Vec<(&str, &str)> = placed
        .iter()
        .map(|rel| (rel.as_str(), "jigc-feedback"))
        .collect();
    ingest_adopts(&corpus, &adopted);

    let listing = corpus.jigc_ok(&["doc", "list", "jigc-feedback", "--format", "json"]);
    let groups = jq(TRIAGE, &listing);

    let grouped: Vec<Vec<(&str, &str, &str)>> = groups
        .as_array()
        .expect("group_by yields an array")
        .iter()
        .map(|group| {
            group
                .as_array()
                .expect("each group is an array of rows")
                .iter()
                .map(|row| {
                    (
                        row["title"].as_str().expect("a row has a title"),
                        row["fields"]["found-in"].as_str().expect("a found-in"),
                        row["fields"]["status"].as_str().expect("a status"),
                    )
                })
                .collect()
        })
        .collect();
    assert_eq!(
        grouped,
        vec![
            vec![
                ("Alpha open", "milestone:alpha", "open"),
                ("Alpha unstated", "milestone:alpha", "open"),
            ],
            vec![("Beta open", "review:beta", "open")],
        ],
        "two open groups by `found-in`, the status-less row counted open and the resolved \
         row in neither; listing:\n{listing}",
    );
    assert_eq!(
        support::trial_corpus::read(&corpus.repo(), &placed[1]),
        feedback("Alpha unstated", "milestone:alpha", None),
        "the projected default is read-side only — no `status:` was written",
    );
}

/// (d) **The `inconsistency.sides/says` heading-depth gate, through the shipped verb.** A
/// `says` item slot is `###`-reserved (`item_slot_ceiling_axis`'s `expected()` claims
/// `(3, 4)` for it), so a `doc set-slot` carrying an ATX heading at H1, H2 or H3 is refused
/// `write.slot-heading-depth`, and one at H4 lands. The create door is the shipped
/// `report-inconsistency` workflow, since no shipped `migrate-inconsistency` exists for the
/// sweep to take.
#[test]
fn a_says_slot_refuses_a_heading_through_h3_and_lands_h4() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("report-inconsistency", "probe the says ceiling");
    let doc = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "inconsistency",
            "--title",
            "Says ceiling",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    let side = corpus.add_item(&format!("{doc}#sides"), "docs/a.md", &task);
    let says = format!("{side}/says");
    let prose = |depth: usize| format!("A side.\n\n{} Ghost\n\ntrailing\n", "#".repeat(depth));
    let write = [
        "doc",
        "set-slot",
        &says,
        "--from-file",
        "-",
        "--task",
        &task,
    ];

    for depth in 1..=3 {
        let out = corpus.jigc_stdin(&write, &prose(depth));
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`{says}` refuses a heading at H{depth}:\n{stderr}",
        );
        assert!(
            stderr.contains("write.slot-heading-depth"),
            "`{says}` at H{depth} refuses as the ceiling reject:\n{stderr}",
        );
    }

    corpus.jigc_stdin_ok(&write, &prose(4));
    assert_eq!(
        corpus.jigc_ok(&["doc", "show", &says, "--task", &task]),
        prose(4),
        "the H4 write landed in `{says}` as written",
    );
}
