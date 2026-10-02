//! M41 Increment 9 / T1 — the **M41 rc.5-wave done-picture acceptance suite**, driven
//! end-to-end through the **real `jigc` binary**. Increments 1–8 proved each feature
//! per-feature (`migration_slot_fidelity.rs`, `start_compose.rs`, `doc_write.rs`,
//! `doc_schema.rs`, `migrate_corpus_value_remap.rs`, the `doc-code` Vue probe units,
//! `set_field_unset.rs`); this suite ties them into one arm per fork over the real binary
//! (`design/worked-examples.md` → flow 42; roadmap → M41 Inc 9).
//!
//! The seven arms, each a `#[test]` over the real binary:
//!
//!   (V1) **folded-slot template fidelity.** A migrate skeleton whose slots are `|-` block
//!        scalars round-trips a multi-paragraph + bulleted body through `jigc doc author`
//!        with every `\n` intact (a folding flow scalar collapsed them pre-fix).
//!
//!   (F1) **the driver contract.** `jigc start --format json` carries `task` — the minted
//!        id on a work-workflow, `null` on the router arm — and a `jigc doc set-field
//!        --format json` write-ack carries the decomposed `target{doctype, slug, …}` +
//!        `findings`.
//!
//!   (F2/finding-key) **the stable finding key + the advisory-route floor.** A committed
//!        adr with a `0..*` `supersedes` carrying two dangling targets fans **two**
//!        `schema-conformance.ref-resolves` findings with **distinct** `(code, target)`
//!        keys, and **every** finding the store sweep emits carries a non-null `route`.
//!
//!   (V3/V6) **the schema read surface.** `jigc doc schema adr --format json` projects the
//!        enum members (`of`) on the `status` field and the field→owning-`section` mapping.
//!
//!   (F4) **the first methodology v1→v2 value-remap.** `jigc migrate-corpus` remaps a
//!        committed v1 `deferral-ledger`'s `kind: D` → `kind: Decision` (and `I` → `Idea`)
//!        **byte-faithful** to the real v2 canonical oracle.
//!
//!   (F3) **the first real-binary Vue proof.** A committed arch-doc component whose
//!        `implemented-by` anchor names a **fabricated** `.vue` script symbol **blocks** at
//!        the finalize gate; re-pointed at a **real** `<script setup>` composable, the same
//!        finalize passes.
//!
//!   (V5) **the optional-scalar clear.** `jigc doc set-field <addr> --unset` clears an
//!        optional adr header scalar (`cites-code`) and the doc re-conforms (a follow-up
//!        write over the now-absent field lands).
//!
//! Everything is asserted on the EMITTED bytes / exit codes / committed files of the real
//! binary (`CARGO_BIN_EXE_jigc`). No external test crates beyond `serde_json`.

use crate::support::run_then_parse::stdout_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow42-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer.
fn git_init(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// `[dev ▸ methodology]` via the **listed-pack** mechanism: `packs.yaml` names the on-disk
/// methodology pack OVER the embedded dev base (the flow41 arm-5 harness shape — arm F4's
/// `deferral-ledger` value-remap).
fn init_listed_pack(repo: &Path) {
    git_init(repo);
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the `JIGC_DOC_CODE_PROBE` override
/// removed (the real probe resolves through the production path), optionally piping
/// `stdin`. Never inherits a harness `JIGC_PACK_DIR` — the embedded shipped pack (plus any
/// project-listed pack) is the base, so the arms prove exactly the doctypes that ship.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Trimmed stdout of an invocation.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim()
        .to_string()
}

/// Combined stdout+stderr, for asserting on a blocked invocation's message.
fn streams_of(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the task's commit doc so a finalize renders a clean git message.
fn fill_commit(repo: &Path, home: &Path, task: &str, scope: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), scope);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Driven by the flow-42 acceptance suite.\n",
    );
}

// ─────────────────── Arm V1 — folded-slot template fidelity ───────────────────

/// The multi-paragraph + bulleted body substituted into every `<<…>>` slot: a lead
/// paragraph, a blank-line paragraph break, then three bullets each on their own line.
/// Under a folding flow scalar the bullets collapse to one line and the blank line to a
/// single space; the shipped `|-` block scalar keeps every break (the V1 fix). Mirrors
/// `migration_slot_fidelity::BODY`.
const BODY: &str = "This is the first mapped paragraph of migrated prose.\n\n\
- first bulleted point survives on its own line\n\
- second bulleted point survives on its own line\n\
- third bulleted point survives on its own line";

/// Extract the `doc author <doctype> --from-file - <<'EOF' … EOF` heredoc skeleton from the
/// composed migrate guidance (`migration_slot_fidelity::extract_author_skeleton`).
fn extract_author_skeleton(composed: &str, doctype: &str) -> String {
    let opener = format!("doc author {doctype} --from-file");
    let mut lines = composed.lines();
    for line in lines.by_ref() {
        if line.contains(&opener) && line.contains("<<'EOF'") {
            break;
        }
    }
    let mut body = String::new();
    for line in lines {
        if line == "EOF" {
            return body;
        }
        body.push_str(line);
        body.push('\n');
    }
    panic!("no `doc author {doctype}` heredoc skeleton in the composed guidance:\n{composed}");
}

/// Substitute `BODY` into every `<<…>>` slot of the skeleton — shape-agnostically. The
/// continuation lines are indented to the column of the `<<` marker, which is exactly the
/// block-scalar content indent in the shipped templates (and folded away under a flow
/// scalar). Mirrors `migration_slot_fidelity::fill_slots`.
fn fill_slots(skeleton: &str) -> String {
    let mut out = String::new();
    let mut rest = skeleton;
    while let Some(open) = rest.find("<<") {
        let line_start = rest[..open].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let col = open - line_start; // ASCII prefix ⇒ byte offset == column
        let indent = " ".repeat(col);
        out.push_str(&rest[..open]);
        out.push_str("<<");
        let after = &rest[open + 2..];
        let close = after.find(">>").expect("a matching `>>` closes each slot");
        let filled: String = BODY
            .split('\n')
            .enumerate()
            .map(|(i, line)| {
                if i == 0 || line.is_empty() {
                    line.to_string()
                } else {
                    format!("{indent}{line}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        out.push_str(&filled);
        out.push_str(">>");
        rest = &after[close + 2..];
    }
    out.push_str(rest);
    out
}

/// **Arm V1.** A foreign ADR migrates through `jigc migrate --as adr`; the composed
/// author skeleton's `|-` block-scalar slots keep a multi-paragraph + bulleted body's line
/// breaks through the `jigc doc author` → finalize round-trip — the committed prose carries
/// `BODY` verbatim (a folding flow scalar would collapse the breaks). Proves the V1 fix
/// (`7d32242` — author-migration slot prose as fold-safe block scalars) end-to-end.
#[test]
fn v1_folded_slot_template_round_trips_prose_intact() {
    let repo = TempDir::new("fold");
    let home = TempDir::new("home");
    git_init(repo.path());

    // A committed foreign ADR at an off-canonical path — the migration source.
    let rel = "docs/adr/0001-fidelity.md";
    fs::create_dir_all(repo.path().join("docs").join("adr")).expect("create docs/adr/");
    fs::write(
        repo.path().join(rel),
        "# 1. A decision\n\n## Status\n\nAccepted\n\n## Context\n\nForces.\n\n\
## Decision\n\nWe chose.\n\n## Consequences\n\nTradeoffs.\n",
    )
    .expect("write foreign adr");
    git(repo.path(), &["add", rel]);
    git(repo.path(), &["commit", "-q", "-m", "track foreign adr"]);

    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );
    let composed = jigc(
        repo.path(),
        home.path(),
        &["migrate", rel, "--as", "adr"],
        None,
    );
    assert_ok(&composed, "`jigc migrate --as adr`");
    let composed_text = stdout_of(&composed);

    // Extract the shipped skeleton verbatim, substitute the fields + a multi-line body.
    // The generated `{{schema:adr}}` skeleton (M43): the title is the one fill-me
    // field placeholder; `status`/`date`/`supersedes` render tree-only.
    let skeleton = extract_author_skeleton(&composed_text, "adr");
    let payload = fill_slots(&skeleton.replace("\"<the title>\"", "Fidelity Probe Decision"));

    // The migrate task id is the one the guidance emits (slug-capped) — parse it from the
    // composed `--task <id>` verb rather than reconstruct it (the emitted-artifact contract).
    let task = composed_text
        .split("--task ")
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_else(|| {
            panic!("the composed guidance carries a `--task <id>`:\n{composed_text}")
        });
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "author", "adr", "--from-file", "-", "--task", task],
            Some(payload.as_bytes()),
        ),
        "`jigc doc author adr --from-file -`",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--approve"],
            None,
        ),
        "`jigc task finalize --approve` (adr migration)",
    );

    let committed = fs::read_to_string(
        repo.path()
            .join("docs")
            .join("decisions")
            .join("fidelity-probe-decision.md"),
    )
    .expect("the committed adr is on disk");
    assert!(
        committed.contains(BODY),
        "the committed adr slot prose keeps its paragraph break + per-bullet line breaks \
         verbatim (a folding flow scalar collapses them); committed:\n{committed}",
    );
}

// ───────────── Arm F1 — start `task` + write-ack decomposed `target` + findings ─────────────

/// **Arm F1.** `jigc start --format json` carries `task` — the minted id on a
/// `creates-task: true` work-workflow, `null` on the router arm — and a `jigc doc set-field
/// --format json` write-ack carries the decomposed `target{doctype, slug, section, leaf}` +
/// literal `findings` (`command-output-contract.md` §1 + §2, proven per-feature in
/// `start_compose.rs` / `doc_write.rs`).
#[test]
fn start_json_carries_task_and_write_ack_carries_target_and_findings() {
    let repo = TempDir::new("driver");
    let home = TempDir::new("home");
    git_init(repo.path());

    // The router arm (bare intent, the cascade default-workflow is `creates-task: false`):
    // `task` is present-and-null, no id minted.
    let router = jigc(
        repo.path(),
        home.path(),
        &["start", "--format", "json", "add rate limiter"],
        None,
    );
    assert_ok(&router, "`jigc start --format json <intent>` (router)");
    let router_json: serde_json::Value =
        stdout_json(&router, &[0], "the router composition is json");
    assert!(
        router_json
            .as_object()
            .is_some_and(|o| o.contains_key("task")),
        "the composed JSON carries a `task` key; got:\n{router_json}",
    );
    assert!(
        router_json["task"].is_null(),
        "the router (creates-task: false) arm mints no task, so `task` is null; got:\n{router_json}",
    );

    // The work arm (Form D over `single-task`, `creates-task: true`): `task` is the minted
    // slug of the intent.
    let work = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--format",
            "json",
            "--workflow",
            "single-task",
            "add rate limiter",
        ],
        None,
    );
    assert_ok(
        &work,
        "`jigc start --format json --workflow single-task <intent>`",
    );
    let work_json: serde_json::Value = stdout_json(&work, &[0], "the work composition is json");
    assert_eq!(
        work_json["task"].as_str(),
        Some("add-rate-limiter"),
        "a work-minting compose carries the minted task id in `task`; got:\n{work_json}",
    );

    // The write-ack: a `set-field` over the provisioned commit doc decomposes its address
    // into `op` + `target{doctype, slug, section, leaf}` + `findings`.
    let ack = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:add-rate-limiter#type",
            "--value",
            "feat",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&ack, "`jigc doc set-field --format json`");
    let ack_json: serde_json::Value = stdout_json(&ack, &[0], "the write-ack is json");
    assert_eq!(
        ack_json["op"], "set-field",
        "the ack names the op; got:\n{ack_json}"
    );
    assert_eq!(
        ack_json["target"]["doctype"], "commit",
        "the ack's target decomposes the doctype; got:\n{ack_json}",
    );
    assert_eq!(
        ack_json["target"]["slug"], "add-rate-limiter",
        "the ack's target decomposes the slug; got:\n{ack_json}",
    );
    assert_eq!(
        ack_json["target"]["section"], "header",
        "a header field's target names the header section; got:\n{ack_json}",
    );
    assert_eq!(
        ack_json["target"]["leaf"], "type",
        "the ack's target names the leaf field; got:\n{ack_json}",
    );
    assert_eq!(
        ack_json["findings"],
        serde_json::json!([]),
        "a clean write carries an empty findings envelope; got:\n{ack_json}",
    );
}

// ───────────── Arm F2 — the stable finding key + the advisory-route floor ─────────────

/// Author + finalize a base adr (no `supersedes`) so a canonical, stamped, committed adr
/// exists at `docs/decisions/<slug>.md`; returns its on-disk path.
fn commit_base_adr(repo: &Path, home: &Path) -> PathBuf {
    let task = "record-the-session-cache";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "single-task",
                "record the session cache",
            ],
            None,
        ),
        "`jigc start` (base adr task)",
    );
    let created = jigc(
        repo,
        home,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Shared redis session cache",
        ],
        None,
    );
    assert_ok(&created, "`jigc doc create adr`");
    let slug = "shared-redis-session-cache";
    set_slot(
        repo,
        home,
        &format!("adr:{slug}#context"),
        b"A single node is a single point of failure.\n",
    );
    set_slot(
        repo,
        home,
        &format!("adr:{slug}#decision"),
        b"Replicate the session cache across nodes.\n",
    );
    set_slot(
        repo,
        home,
        &format!("adr:{slug}#consequences"),
        b"Slightly higher write latency for resilience.\n",
    );
    fill_commit(repo, home, task, "cache");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (base adr)",
    );
    repo.join("docs")
        .join("decisions")
        .join(format!("{slug}.md"))
}

/// **Arm F2 (finding-key).** A committed adr whose `0..*` `supersedes` carries two dangling
/// targets fans **two** blocking `schema-conformance.ref-resolves` findings, each with a
/// **distinct** `(code, target)` key (`adr:<slug>#supersedes/<type>:<to-slug>` — a `0..*` ref fans
/// one keyed finding per dangling target, `command-output-contract.md` → the stable finding
/// key), and **every** finding the store sweep emits carries a non-null `route` (the V15 /
/// Fork-2 advisory-route floor — never `null`). Store-scope `validate` is report-only, so it
/// exits 0 even with the blocking content findings present.
#[test]
fn two_dangling_ref_sweep_emits_two_uniquely_keyed_findings_every_advisory_routed() {
    let repo = TempDir::new("dangle");
    let home = TempDir::new("home");
    git_init(repo.path());

    let committed_path = commit_base_adr(repo.path(), home.path());

    // Out-of-band, splice a `0..*` supersedes with two dangling targets into the committed
    // adr's front-matter (before the closing fence — the `doc_code_gate::inject_cites_code`
    // technique) and commit it, so HEAD carries the two dangling edges.
    let body = fs::read_to_string(&committed_path).expect("read the committed adr");
    let with_dangling = body.replacen(
        "\n---\n",
        "\nsupersedes: [adr:ghost-one, adr:ghost-two]\n---\n",
        1,
    );
    assert_ne!(
        body, with_dangling,
        "the committed adr carries a front-matter fence"
    );
    fs::write(&committed_path, &with_dangling).expect("write the dangling adr");
    git(repo.path(), &["add", "."]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "add dangling supersedes"],
    );

    let sweep = jigc(
        repo.path(),
        home.path(),
        &["validate", "--format", "json"],
        None,
    );
    assert_ok(&sweep, "`jigc validate --format json` (report-only exit 0)");
    let report: serde_json::Value = stdout_json(&sweep, &[0], "validate emits json");
    let findings = report["findings"]
        .as_array()
        .expect("`findings` is an array");

    // Exactly two ref-resolves findings, keyed distinctly per dangling target.
    let ref_findings: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| f["code"] == "schema-conformance.ref-resolves")
        .collect();
    assert_eq!(
        ref_findings.len(),
        2,
        "the `0..*` supersedes fans two ref-resolves findings; got:\n{report}",
    );
    let keys: std::collections::BTreeSet<String> = ref_findings
        .iter()
        .map(|f| {
            f["key"]["target"]
                .as_str()
                .expect("a keyed target")
                .to_string()
        })
        .collect();
    assert_eq!(
        keys,
        [
            "adr:shared-redis-session-cache#supersedes/adr:ghost-one".to_string(),
            "adr:shared-redis-session-cache#supersedes/adr:ghost-two".to_string()
        ]
        .into_iter()
        .collect(),
        "the two findings carry DISTINCT `(code, target)` keys per dangling target; got:\n{report}",
    );

    // The advisory-route floor: EVERY finding the sweep emits carries a non-null route.
    for finding in findings {
        assert!(
            !finding["route"].is_null(),
            "every finding routes (the V15 advisory-route floor, never null); got:\n{finding}",
        );
    }
}

// ───────────── Arm V3/V6 — the schema read surface: enum members + field→section ─────────────

/// **Arm V3/V6.** `jigc doc schema adr --format json` projects the per-field enum members
/// (`of`) and the top-level field→owning-`section` mapping (contract-version 7 since M52
/// — the `identity`/`home` pair; 6 was the M50 `ref`'s `to` target, 5 the M48 id-source
/// `write-key`, 4 the M45 settability
/// states; proven byte-verbatim in `doc_schema.rs`):
/// the `status` field carries its enum members, names the `status` section it lives
/// under, and names its concrete `set-field` write address.
#[test]
fn doc_schema_shows_enum_members_and_field_sections() {
    let repo = TempDir::new("schema");
    let home = TempDir::new("home");
    git_init(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "json"],
        None,
    );
    assert_ok(&out, "`jigc doc schema adr --format json`");
    let value: serde_json::Value = stdout_json(&out, &[0], "the schema projection is json");
    assert_eq!(
        value["contract-version"], 7,
        "the projection carries the pinned contract version"
    );

    let fields = value["fields"].as_array().expect("`fields` is an array");
    let status = fields
        .iter()
        .find(|f| f["id"] == "status")
        .unwrap_or_else(|| panic!("the adr `status` field renders; got:\n{value}"));
    assert_eq!(
        status["of"],
        serde_json::json!(["proposed", "accepted", "superseded"]),
        "the enum `status` field projects its members in `of`; got:\n{status}",
    );
    assert_eq!(
        status["section"], "status",
        "a top-level field names its owning section; got:\n{status}",
    );
    assert_eq!(
        status["set-field"], "adr:<slug>#status/status",
        "a settable field names its concrete set-field write address; got:\n{status}",
    );
}

// ───────────── Arm F4 — the first methodology v1→v2 value-remap ─────────────

/// Author one `deferral-ledger` entry (`kind` enum + `trigger` + `body` slot; `date` is
/// CLI-set on add-item) in the already-created singleton (the
/// `migrate_corpus_value_remap::author_entry` shape).
fn author_entry(repo: &Path, home: &Path, title: &str, kind: &str, trigger: &str, body: &[u8]) {
    let item = jigc(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "deferral-ledger:deferral-ledger#entries",
            "--title",
            title,
        ],
        None,
    );
    assert_ok(&item, "`doc add-item deferral-ledger#entries`");
    let addr = stdout_of(&item);
    set_field(repo, home, &format!("{addr}/kind"), kind);
    set_field(repo, home, &format!("{addr}/trigger"), trigger);
    set_slot(repo, home, &format!("{addr}/body"), body);
}

/// **Arm F4.** The first methodology v1→v2 migration through the real binary: a committed
/// **v1** `deferral-ledger` carrying `kind: D` + `kind: I` is remapped `D` → `Decision` /
/// `I` → `Idea` **byte-faithful** by `jigc migrate-corpus` via the CLI-authored old→new map
/// (`corpus-migration.md` → the value-remap kind). The proof is a round-trip: the v1 fixture
/// is the exact-inverse downgrade of a REAL v2 canonical doc (the oracle), so a byte-faithful
/// remap reproduces the oracle exactly (stamp bump `1`→`2` included).
#[test]
fn migrate_corpus_remaps_deferral_ledger_byte_faithful() {
    let repo = TempDir::new("remap");
    let home = TempDir::new("home");
    init_listed_pack(repo.path());

    // Author a REAL v2 canonical deferral-ledger (the byte-faithfulness oracle) off-router
    // through the `planning` workflow's create-gate.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "planning", "M-Test"],
            None,
        ),
        "`jigc start --workflow planning`",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "create",
                "deferral-ledger",
                "--title",
                "Deferral Ledger",
            ],
            None,
        ),
        "`doc create deferral-ledger`",
    );
    author_entry(
        repo.path(),
        home.path(),
        "Cache the index",
        "Decision",
        "M-Store",
        b"Deferred until the store scope lands.\n",
    );
    author_entry(
        repo.path(),
        home.path(),
        "A plugin surface",
        "Idea",
        "M-External",
        b"Parked until a real external domain earns it.\n",
    );
    let canonical_v2 = fs::read_to_string(
        repo.path()
            .join(".jigc/tasks/m-test/docs/deferral-ledger:deferral-ledger.md"),
    )
    .expect("read the staged canonical v2 deferral-ledger");
    assert!(
        canonical_v2.contains("schema-version: 2")
            && canonical_v2.contains("- kind: Decision")
            && canonical_v2.contains("- kind: Idea"),
        "the oracle is the v2 canonical form; got:\n{canonical_v2}",
    );

    // Commit the v1 fixture: the exact-inverse downgrade of the oracle (the enum rename + the
    // stamp bump are the whole v1→v2 delta; every other byte is identical).
    let committed_v1 = canonical_v2
        .replace("schema-version: 2", "schema-version: 1")
        .replace("- kind: Decision", "- kind: D")
        .replace("- kind: Idea", "- kind: I");
    assert_ne!(committed_v1, canonical_v2, "the downgrade actually differs");
    fs::create_dir_all(repo.path().join("docs")).expect("mk docs/");
    fs::write(repo.path().join("docs/deferral-ledger.md"), &committed_v1)
        .expect("write v1 fixture");
    git(repo.path(), &["add", "docs/deferral-ledger.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "seed v1 deferral-ledger"],
    );

    // migrate-corpus: the authored remap drives the migration. Assert the emitted bytes on
    // disk equal the v2 canonical oracle exactly.
    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"], None);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    let migrated = fs::read_to_string(repo.path().join("docs/deferral-ledger.md"))
        .expect("read migrated deferral-ledger");
    assert_eq!(
        migrated, canonical_v2,
        "the migrated doc is byte-faithful to the v2 canonical oracle (D→Decision, I→Idea, \
         stamp 1→2, every other byte preserved)",
    );
}

// ───────────── Arm F3 — the first real-binary Vue proof ─────────────

/// A real Vue single-file component: `export default { name: 'AppLayout' }` in a plain
/// `<script>`, a `useCounter` composable in `<script setup>`, plus template/style noise the
/// extract must ignore (the `doc-code` Vue probe fixture — `resolve.rs` unit `const VUE`).
const APP_LAYOUT_VUE: &str = "\
<template>
  <div class=\"layout-class\">{{ templateOnlyName }}</div>
</template>

<script lang=\"ts\">
export default {
  name: 'AppLayout',
};

function plainScriptFn(): number {
  return 1;
}
</script>

<script setup lang=\"ts\">
defineProps(['title']);

function useCounter(): number {
  return 0;
}

const setupConst = 1;
</script>

<style scoped>
.layout-class {
  color: red;
}
</style>
";

/// **Arm F3.** The first real-binary Vue proof: a committed arch-doc component whose
/// `implemented-by` anchor names a **fabricated** `.vue` script symbol **blocks** at the
/// finalize gate (`doc-code.symbol-exists`, exit non-zero — the gate resolves the SFC's
/// `<script setup>` under the vendored TS grammar, `validation.md` → Vue addressable units);
/// re-pointed at a **real** composable (`useCounter`), the same finalize passes.
#[test]
fn fabricated_vue_script_symbol_blocks_a_real_symbol_resolves() {
    let repo = TempDir::new("vue");
    let home = TempDir::new("home");
    git_init(repo.path());

    // Commit the Vue SFC so the finalize gate (index-scoped) resolves anchors against it.
    fs::create_dir_all(repo.path().join("src")).expect("mk src/");
    fs::write(
        repo.path().join("src").join("AppLayout.vue"),
        APP_LAYOUT_VUE,
    )
    .expect("write .vue");
    git(repo.path(), &["add", "src/AppLayout.vue"]);
    git(repo.path(), &["commit", "-q", "-m", "seed the Vue SFC"]);

    let task = "document-the-app-shell";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "architecture-documentation",
                "document the app shell",
            ],
            None,
        ),
        "`jigc start --workflow architecture-documentation`",
    );
    let created = jigc(
        repo.path(),
        home.path(),
        &["doc", "create", "arch-doc", "--title", "App shell"],
        None,
    );
    assert_ok(&created, "`jigc doc create arch-doc`");
    assert_eq!(stdout_of(&created), "arch-doc:app-shell");
    set_slot(
        repo.path(),
        home.path(),
        "arch-doc:app-shell#overview",
        b"The app shell owns the top-level layout.\n",
    );
    let added = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            "arch-doc:app-shell#components",
            "--title",
            "Layout",
        ],
        None,
    );
    assert_ok(&added, "`jigc doc add-item …#components`");
    let item = stdout_of(&added);
    set_slot(
        repo.path(),
        home.path(),
        &format!("{item}/description"),
        b"The root layout component.\n",
    );
    fill_commit(repo.path(), home.path(), task, "arch-doc");

    // A FABRICATED `.vue` script symbol → finalize BLOCKS at the doc-code gate.
    set_field(
        repo.path(),
        home.path(),
        &format!("{item}/implemented-by"),
        "src/AppLayout.vue#DoesNotExist",
    );
    let blocked = jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    assert!(
        !blocked.status.success(),
        "a fabricated `.vue` script symbol must BLOCK finalize; got:\n{}",
        streams_of(&blocked),
    );
    assert!(
        streams_of(&blocked).contains("doc-code.symbol-exists"),
        "the block is the doc-code symbol-exists finding naming the dangling anchor; got:\n{}",
        streams_of(&blocked),
    );

    // Re-point at a REAL `<script setup>` composable → the SAME finalize PASSES.
    set_field(
        repo.path(),
        home.path(),
        &format!("{item}/implemented-by"),
        "src/AppLayout.vue#useCounter",
    );
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` (real Vue composable resolves)",
    );
    assert!(
        repo.path().join("docs/architecture/app-shell.md").exists(),
        "the finalized arch-doc is committed at its placement home",
    );
}

// ───────────── Arm V5 — the optional-scalar clear ─────────────

/// **Arm V5.** `jigc doc set-field <addr> --unset` clears an optional adr header scalar
/// (`cites-code`) — the field line is gone and the ack names the clear (`unset: true`) — and
/// the doc **re-conforms**: a follow-up `set-field` over the now-absent field lands
/// (`write-commands.md` → the `--unset` verb).
#[test]
fn set_field_unset_clears_an_optional_scalar() {
    let repo = TempDir::new("unset");
    let home = TempDir::new("home");
    git_init(repo.path());

    let task = "clear-the-anchor";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "clear the anchor"],
            None,
        ),
        "`jigc start` (unset task)",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "create",
                "adr",
                "--title",
                "Anchor decision",
                "--task",
                task,
            ],
            None,
        ),
        "`jigc doc create adr`",
    );
    let slug = "anchor-decision";
    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"));
    let addr = format!("adr:{slug}#status/cites-code");

    // Populate the optional scalar, then clear it via `--unset`.
    set_field(repo.path(), home.path(), &addr, "src/lib.rs#present_symbol");
    assert!(
        fs::read_to_string(&staged).unwrap().contains("cites-code:"),
        "precondition: cites-code is populated",
    );
    let unset = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &addr,
            "--unset",
            "--task",
            task,
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&unset, "`jigc doc set-field --unset --format json`");
    let ack: serde_json::Value = stdout_json(&unset, &[0], "the unset ack is json");
    assert_eq!(ack["op"], "set-field", "the ack names the op; got:\n{ack}");
    assert_eq!(
        ack["unset"],
        serde_json::Value::Bool(true),
        "the JSON ack names the clear; got:\n{ack}",
    );
    assert!(
        !fs::read_to_string(&staged).unwrap().contains("cites-code"),
        "the cleared field line is gone",
    );

    // Re-conforms: a fresh write over the now-absent field succeeds.
    set_field(repo.path(), home.path(), &addr, "src/lib.rs#present_symbol");
    assert!(
        fs::read_to_string(&staged).unwrap().contains("cites-code:"),
        "the field re-populates cleanly after the clear (the doc still conforms)",
    );
}
