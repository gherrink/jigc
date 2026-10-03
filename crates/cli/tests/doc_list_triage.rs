//! M55 Increment 6 / T3 — **one `doc list` answers a triage**: *all open findings, grouped by
//! `found-in`* (`design/findings-channel.md` → 5, F18 · F10; `design/doc-read-surface.md` →
//! `jigc doc list`).
//!
//! Before M55 that question cost `doc list` plus one `doc show` per row, because a row carried
//! identity, path and state and no header field; and a hand-deleted defaulted `status` vanished
//! from `doc show`'s `fields`, so a client filtering `status == "open"` missed the row in
//! silence. Each `doc list --format json` row now carries `title` and `fields` — the header
//! fields in `doc show`'s shape, an absent defaulted field at its schema default — so the
//! triage is **one listing through `jq`**, and no `doc show` is run.
//!
//! **The doctype is a fixture.** The shipped findings doctypes land at Increment 7, where this
//! query is re-driven on them; here a [`FixturePack`] carries a `finding` doctype with exactly
//! the two header fields the query reads — a defaulted `status` and a `found-in` string.
//!
//! **The jq filter is run on the emitted bytes**: the binary's stdout is fed to a real `jq`
//! process verbatim (`jq` is on every CI runner and in `dev/runner-faithful`), never re-parsed
//! and re-filtered in test code.
//!
//! The second arm is the shipped doctype's own case: on `adr`, the one shipped defaulted field,
//! a hand-deleted committed `status:` reads `"proposed"` on the `doc list` row exactly as it
//! does on `doc show`, while the file's bytes stay as committed.

use crate::support;

use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::{Value, json};
use support::trial_corpus::{FixturePack, State, TrialCorpus};

/// The fixture `finding` doctype: a header carrying a **defaulted** `status` and a `found-in`
/// string, one prose slot, homed at `findings/`. Unversioned (the fixture pack ships no
/// manifest), so a row is never `unregistered`.
const FINDING_SCHEMA: &str = "\
type: finding
location: findings/
id-from: title
description: One finding, recorded where it was found.
usage: something was found that a later triage must see.

sections:
  - id: header
    header: true
    fields:
      - { id: status, type: enum, of: [open, resolved], default: open }
      - { id: found-in, type: string }
  - id: what
    slot: { hint: \"What was found.\" }
";

/// The workflow whose create-gate admits the fixture doctype.
const LOG_FINDING: &str = "\
---
when: log one finding
description: Record a finding and commit it.
usage: a finding needs recording.
creates-task: true
allows-create: [{type: finding, as: finding}]
---
{{ include: step:finalize }}
";

/// The triage, verbatim: every open finding, grouped by the version it was found in.
const TRIAGE: &str =
    r#"[.docs[] | select(.fields.status == "open")] | group_by(.fields["found-in"])"#;

/// The repo-relative path the committed listing prints for `id` — read off the emitted row,
/// since where a doctype homes is the cascade's to say (`docs-root`), not this suite's.
fn path_of(corpus: &TrialCorpus, id: &str) -> String {
    let listing: Value =
        serde_json::from_str(&corpus.jigc_ok(&["doc", "list", "--format", "json"]))
            .expect("the listing is json");
    listing["docs"]
        .as_array()
        .expect("`docs` is an array")
        .iter()
        .find(|row| row["id"] == json!(id))
        .and_then(|row| row["path"].as_str())
        .unwrap_or_else(|| panic!("a committed row for `{id}`; listing:\n{listing:#}"))
        .to_string()
}

/// File one finding through its own task and finalize it, returning its committed path.
/// `resolved` sets its status before the commit; otherwise it keeps the default `create`
/// wrote.
fn file_finding(corpus: &TrialCorpus, title: &str, found_in: &str, resolved: bool) -> String {
    let task = corpus.start_workflow("log-finding", &format!("log {title}"));
    let created = corpus.jigc_ok(&[
        "doc", "create", "finding", "--title", title, "--task", &task,
    ]);
    let addr = created.trim().to_string();
    corpus.set_field(&format!("{addr}#found-in"), &task, found_in);
    if resolved {
        corpus.set_field(&format!("{addr}#status"), &task, "resolved");
    }
    corpus.set_slot(
        &format!("{addr}#what"),
        &task,
        &format!("{title}, in detail."),
    );
    corpus.finalize(&task, "findings", &format!("log {title}"), false);
    path_of(corpus, &addr)
}

/// Delete the `status:` line of the committed doc at `rel` by hand and commit the edit —
/// the one way a defaulted field goes absent (its `--unset` is refused). Returns the bytes
/// as committed.
fn hand_delete_status(corpus: &TrialCorpus, rel: &str) -> String {
    let path = corpus.repo().join(rel);
    let before = std::fs::read_to_string(&path).expect("read the committed doc");
    let after: String = before
        .split_inclusive('\n')
        .filter(|line| !line.starts_with("status:"))
        .collect();
    assert_ne!(after, before, "`{rel}` carried a `status:` line to delete");
    std::fs::write(&path, &after).expect("write the hand edit");
    corpus.git(&["add", rel]);
    corpus.git(&["commit", "-q", "-m", "drop the status line by hand"]);
    after
}

/// Assert the file at `rel` is byte-identical to its `HEAD` blob and carries no `status:`
/// line — the read surfaces projected a default and wrote nothing.
fn assert_stored_as_committed(corpus: &TrialCorpus, rel: &str, committed: &str) {
    let on_disk = std::fs::read_to_string(corpus.repo().join(rel)).expect("read the doc back");
    assert_eq!(on_disk, committed, "the read never writes `{rel}`");
    assert_eq!(
        corpus.git(&["show", &format!("HEAD:{rel}")]),
        committed.trim(),
        "`{rel}` is its committed blob (the corpus's `git` helper trims its stdout)",
    );
    assert!(
        !on_disk.lines().any(|line| line.starts_with("status:")),
        "no default was written into `{rel}`:\n{on_disk}",
    );
}

/// Run `jq <filter>` over `input`, verbatim, and parse what it prints.
fn jq(filter: &str, input: &str) -> Value {
    let mut child = Command::new("jq")
        .arg(filter)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jq — it is on every runner (`dev/runner-faithful` installs it)");
    child
        .stdin
        .take()
        .expect("jq's stdin")
        .write_all(input.as_bytes())
        .expect("feed jq the listing");
    let out = child.wait_with_output().expect("wait for jq");
    assert!(
        out.status.success(),
        "jq `{filter}` failed:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    serde_json::from_slice(&out.stdout).expect("jq prints json")
}

/// (M55 inc-6 T3) **All open findings, grouped by `found-in`, from one `doc list`.** Three
/// findings are filed — two in `v1`, one in `v2` — one `v1` finding is resolved, and the
/// other has its `status:` hand-deleted and committed. One `jigc doc list finding --format
/// json` through the triage filter yields exactly the two open groups, the hand-deleted row
/// counted open through its schema default. Red at the base: no row carried `fields`, so
/// the filter selected nothing.
#[test]
fn one_listing_groups_the_open_findings_by_found_in() {
    let pack = FixturePack::from_dev_pack("doc-list-triage");
    pack.write_schema("finding", FINDING_SCHEMA)
        .write_workflow("log-finding", LOG_FINDING);
    let corpus = TrialCorpus::build_with_pack(State::Fresh, &pack);

    let resolved = file_finding(&corpus, "Login loops", "v1", true);
    let statusless = file_finding(&corpus, "Logout hangs", "v1", false);
    let open_v2 = file_finding(&corpus, "Export drops rows", "v2", false);
    let committed = hand_delete_status(&corpus, &statusless);

    let listing = corpus.jigc_ok(&["doc", "list", "finding", "--format", "json"]);
    let groups = jq(TRIAGE, &listing);

    let paths: Vec<Vec<&str>> = groups
        .as_array()
        .expect("group_by yields an array")
        .iter()
        .map(|group| {
            group
                .as_array()
                .expect("each group is an array of rows")
                .iter()
                .map(|row| row["path"].as_str().expect("a row has a path"))
                .collect()
        })
        .collect();
    assert_eq!(
        paths,
        vec![vec![statusless.as_str()], vec![open_v2.as_str()]],
        "two open groups — `v1` holding the hand-deleted row alone, `v2` the other — and the \
         resolved finding in neither; listing:\n{listing}",
    );
    assert!(
        !paths.concat().contains(&resolved.as_str()),
        "a resolved finding is not open",
    );

    // Each grouped row carries what a triage prints beside it: its title, and the fields it
    // was grouped on — the hand-deleted `status` at its default.
    assert_eq!(groups[0][0]["title"], json!("Logout hangs"));
    assert_eq!(groups[0][0]["fields"]["status"], json!("open"));
    assert_eq!(groups[0][0]["fields"]["found-in"], json!("v1"));
    assert_eq!(groups[1][0]["title"], json!("Export drops rows"));
    assert_eq!(groups[1][0]["fields"]["found-in"], json!("v2"));

    assert_stored_as_committed(&corpus, &statusless, &committed);
}

/// (M55 inc-6 T3) **On `adr`, the one shipped defaulted field, a hand-deleted committed
/// `status:` reads `"proposed"` on the `doc list` row** — the same effective value `doc show`
/// serves (T2), from the same helper — and the file's bytes stay as committed.
#[test]
fn a_hand_deleted_adr_status_lists_as_its_default() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "record the cache decision");
    let created = corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Single-node cache",
        "--task",
        &task,
    ]);
    let addr = created.trim().to_string();
    corpus.set_field(&format!("{addr}#status"), &task, "accepted");
    for slot in ["context", "decision", "consequences"] {
        corpus.set_slot(&format!("{addr}#{slot}"), &task, &format!("The {slot}."));
    }
    corpus.finalize(&task, "decisions", "record the cache decision", false);
    let rel = path_of(&corpus, &addr);
    let committed = hand_delete_status(&corpus, &rel);

    let listing: Value =
        serde_json::from_str(&corpus.jigc_ok(&["doc", "list", "adr", "--format", "json"]))
            .expect("the listing is json");
    let row = listing["docs"]
        .as_array()
        .expect("`docs` is an array")
        .iter()
        .find(|row| row["id"] == json!(addr))
        .unwrap_or_else(|| panic!("a row for `{addr}`; listing:\n{listing:#}"));
    assert_eq!(
        row["fields"]["status"],
        json!("proposed"),
        "the absent `status` lists at its schema default; row:\n{row:#}",
    );

    let shown: Value =
        serde_json::from_str(&corpus.jigc_ok(&["doc", "show", &addr, "--format", "json"]))
            .expect("the show is json");
    assert_eq!(
        (&row["title"], &row["fields"]),
        (&shown["title"], &shown["fields"]),
        "the row and the whole-doc serve agree on `title` and `fields`",
    );

    assert_stored_as_committed(&corpus, &rel, &committed);
}
