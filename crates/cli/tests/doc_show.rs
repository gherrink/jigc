//! M39 Increment 1 / T2 — `jigc doc show <ref>`, the committed-doc **read surface**,
//! proven end-to-end over the real `jigc` binary under the `[dev ▸ methodology]`
//! composition (`design/team-ready-state.md` → The read surface; `design/
//! introspection.md` → "Reading *filled* prose stays `jigc doc show`").
//!
//! The load-bearing contract this pins is the **`--format json` shape** — a 1.0 stable
//! contract, a one-way door pinned NOW by its golden: a whole-doc object
//! `{ type, slug, fields, sections }` where a slot section serializes to its prose
//! string and a repeatable section to its item array, a `#section` slice returns the
//! item array, and an `#section/<id>` slice the item object. The milestone-record (Inc 3)
//! is the contract's later witness; here `vision` (whole-doc, slot sections) + `prd`
//! (a repeatable `requirements` section) are the witnesses.
//!
//! Everything is asserted on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`) — the json goldens are matched verbatim, the block envelope of
//! a bad ref is driven through the real exit code + stderr. No external test crates.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-doc-show-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The `schema-version` the shipped methodology manifest declares for `ty` — the stamp a
/// freshly minted instance carries, and so the stamp its served header slice must show.
///
/// **Derived, never spelled**: this arm's subject is that a fields-only header section
/// serves *its field lines*, not that one of those lines equals a particular integer, and
/// a literal made it redden on the M49 `milestone-record` 2→3 bump.
fn declared_schema_version(ty: &str) -> u32 {
    let bytes = fs::read(
        methodology_pack_tree()
            .join("config")
            .join("schema-manifest.yaml"),
    )
    .expect("read the shipped methodology schema-manifest");
    let manifest: engine::manifest::Manifest =
        serde_yaml_ng::from_slice(&bytes).expect("the methodology manifest deserializes");
    manifest
        .doctypes
        .iter()
        .find(|entry| entry.ty == ty)
        .unwrap_or_else(|| panic!("the methodology manifest declares `{ty}`"))
        .schema_version
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// naming the methodology pack in `packs.yaml` (the `[dev ▸ methodology]` composition:
/// both the dev `prd` and the methodology `vision` doctypes resolve).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
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

/// The trimmed stdout of a successful invocation.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

/// Set one prose slot through the binary (stdin), asserting success.
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

/// Set one field through the binary, asserting success.
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

/// Fill the auto-provisioned commit doc so a code-less task finalizes.
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
        b"A managed doc.\n",
    );
}

/// Author + commit the `vision` singleton (no grounding — `grounded-in` is `0..*`), so
/// `VISION.md` is a committed whole-doc read target with three filled slot sections.
fn commit_vision(repo: &Path, home: &Path) {
    let task = "form-the-project-vision";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "form-vision",
                "form the project vision",
            ],
            None,
        ),
        "`jigc start --workflow form-vision`",
    );
    let create = jigc(
        repo,
        home,
        &[
            "doc", "create", "vision", "--title", "Vision", "--task", task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create vision`");
    assert_eq!(stdout_of(&create).trim(), "vision:vision");

    set_slot(
        repo,
        home,
        "vision:vision#thesis",
        b"A deterministic context compiler for coding agents.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#invariants",
        b"The CLI owns every structural write; the LLM writes only prose.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#open-questions",
        b"How far can one methodology pack compose.\n",
    );
    fill_commit(repo, home, task, "vision");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (form-vision) — the committed VISION.md",
    );
}

/// The `prd` batch-author payload: `vision`/`context` slot sections + a repeatable
/// `requirements` section carrying two items (each a `title` field + a `statement`
/// slot). The `<<…>>` markers tag the slot prose.
const PRD_PAYLOAD: &str = r#"title: "Habit tracker"
sections:
  - id: vision
    set:
      vision: "<<A tracker that turns intentions into daily streaks.>>"
  - id: requirements
    items:
      - title: "Log a habit in one tap"
        set:
          statement: "<<Logging a habit takes a single tap from the home screen.>>"
      - title: "Show the current streak"
        set:
          statement: "<<The current streak is shown front and center.>>"
  - id: context
    set:
      context: "<<Built for solo users who abandon heavyweight planners.>>"
"#;

/// Author + commit a `prd` with two `requirements` items, so `docs/prds/habit-tracker.md`
/// is a committed read target with a repeatable section.
fn commit_prd(repo: &Path, home: &Path) {
    let task = "build-a-habit-tracker";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "project-setup",
                "build a habit tracker",
            ],
            None,
        ),
        "`jigc start --workflow project-setup`",
    );
    let authored = jigc(
        repo,
        home,
        &["doc", "author", "prd", "--from-file", "-", "--task", task],
        Some(PRD_PAYLOAD.as_bytes()),
    );
    assert_ok(&authored, "`jigc doc author prd`");
    assert_eq!(stdout_of(&authored).trim(), "prd:habit-tracker");
    fill_commit(repo, home, task, "prd");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (project-setup) — the committed prd",
    );
}

// ---- the pinned `--format json` goldens (the 1.0 contract, byte-verbatim) ----

// `schema-version` joined the pinned witness `fields` when the M40 A1 methodology
// manifest froze `vision` (the declared additive-pre-pin posture —
// design/team-ready-state.md → the pinned witness fields). The **top-level**
// `schema-version` is M49's additive key — the same stamp as a json NUMBER, so the
// cross-surface comparison against `jigc doc schema` needs no cast, while the `fields`
// entry stays the string and the map stays uniformly stringy
// (design/doc-read-surface.md → One name, two json types).
const VISION_JSON: &str = r#"{
  "fields": {
    "schema-version": "1"
  },
  "item-count": 0,
  "schema-version": 1,
  "sections": {
    "invariants": "The CLI owns every structural write; the LLM writes only prose.",
    "open-questions": "How far can one methodology pack compose.",
    "thesis": "A deterministic context compiler for coding agents."
  },
  "slug": "vision",
  "type": "vision"
}"#;

const PRD_JSON: &str = r#"{
  "fields": {
    "schema-version": "1"
  },
  "item-count": 2,
  "schema-version": 1,
  "sections": {
    "context": "Built for solo users who abandon heavyweight planners.",
    "requirements": [
      {
        "id": "log-a-habit-in-one",
        "statement": "Logging a habit takes a single tap from the home screen.",
        "title": "Log a habit in one tap"
      },
      {
        "id": "show-the-current-streak",
        "statement": "The current streak is shown front and center.",
        "title": "Show the current streak"
      }
    ],
    "vision": "A tracker that turns intentions into daily streaks."
  },
  "slug": "habit-tracker",
  "type": "prd"
}"#;

const REQUIREMENTS_JSON: &str = r#"[
  {
    "id": "log-a-habit-in-one",
    "statement": "Logging a habit takes a single tap from the home screen.",
    "title": "Log a habit in one tap"
  },
  {
    "id": "show-the-current-streak",
    "statement": "The current streak is shown front and center.",
    "title": "Show the current streak"
  }
]"#;

// The item `id` is the handle every address into the item takes, and it is NOT the
// slugified heading — the title `Log a habit in one tap` mints `log-a-habit-in-one`
// (the word cap bites), which is exactly the address the item slice below is read at
// (M42 — `design/doc-read-surface.md` → The item `id` closes the json contract).
const ONE_REQUIREMENT_JSON: &str = r#"{
  "id": "log-a-habit-in-one",
  "statement": "Logging a habit takes a single tap from the home screen.",
  "title": "Log a habit in one tap"
}"#;

/// A committed `vision` round-trips through `jigc doc show` (plain carries every section)
/// and its `--format json` matches the pinned whole-doc golden; a committed `prd` shows
/// its whole-doc + repeatable-section slices under the pinned json shape; a bad ref exits
/// non-zero with the routed block envelope.
#[test]
fn doc_show_serves_the_committed_read_surface() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_vision(repo.path(), home.path());
    commit_prd(repo.path(), home.path());

    // (1) Plain whole-doc `vision` carries every section (the byte-exact committed view).
    let plain = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision"],
        None,
    );
    assert_ok(&plain, "`jigc doc show vision:vision`");
    let plain = stdout_of(&plain);
    assert!(
        plain.contains("# Vision"),
        "the display-title H1; got:\n{plain}"
    );
    for needle in [
        "A deterministic context compiler for coding agents.",
        "The CLI owns every structural write; the LLM writes only prose.",
        "How far can one methodology pack compose.",
    ] {
        assert!(
            plain.contains(needle),
            "plain vision must carry `{needle}`; got:\n{plain}"
        );
    }

    // (2) `vision --format json` matches the pinned whole-doc golden verbatim.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision", "--format", "json"],
        None,
    );
    assert_ok(&json, "`jigc doc show vision:vision --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        VISION_JSON,
        "the vision whole-doc json is the pinned 1.0 shape",
    );

    // (3) `prd --format json` — the mixed shape: slot sections → prose, a repeatable
    //     section → its item array.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "prd:habit-tracker", "--format", "json"],
        None,
    );
    assert_ok(&json, "`jigc doc show prd:habit-tracker --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        PRD_JSON,
        "the prd whole-doc json is the pinned 1.0 shape (slots + item array)",
    );

    // (4) A `#requirements` slice returns the item ARRAY under json.
    let json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "prd:habit-tracker#requirements",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&json, "`jigc doc show prd:...#requirements --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        REQUIREMENTS_JSON,
        "a `#section` slice over a repeatable returns the item array",
    );

    // (5) A `#requirements/<id>` slice returns the item — plain (the rendered item) and
    //     json (the item object).
    let item_addr = "prd:habit-tracker#requirements/log-a-habit-in-one";
    let plain = jigc(repo.path(), home.path(), &["doc", "show", item_addr], None);
    assert_ok(&plain, "`jigc doc show <item>`");
    let plain = stdout_of(&plain);
    assert!(
        plain.contains("Log a habit in one tap")
            && plain.contains("Logging a habit takes a single tap from the home screen."),
        "the plain item slice carries the item's title + statement; got:\n{plain}",
    );
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", item_addr, "--format", "json"],
        None,
    );
    assert_ok(&json, "`jigc doc show <item> --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        ONE_REQUIREMENT_JSON,
        "an `#section/<id>` slice returns the single item object",
    );

    // (6) A bad ref exits non-zero with the routed block envelope.
    let bad = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "prd:does-not-exist"],
        None,
    );
    assert!(
        !bad.status.success(),
        "a missing doc must exit non-zero; stdout:\n{}\nstderr:\n{}",
        stdout_of(&bad),
        String::from_utf8_lossy(&bad.stderr),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("route:") && stderr.contains("does-not-exist"),
        "the block envelope names the bad ref + its route; got:\n{stderr}",
    );
}

/// A **placement singleton** (`vision`, homed at the literal `VISION.md`) is addressable
/// only at its canonical slug = the type id (`vision:vision`). `jigc doc show` must route
/// a **non-canonical** slug (`vision:does-not-exist`) as a not-found block — never silently
/// return the singleton's content for an invalid reference — while the canonical read
/// (`vision:vision`) keeps working. Regression guard for the M39 read-surface hardening:
/// `canonical_path` resolves a placement doctype regardless of slug, so without the
/// read-path guard ANY slug returned `VISION.md` at exit 0 (a looseness in the 1.0 read
/// contract). Proven on the emitted bytes + exit code of the real binary.
#[test]
fn doc_show_routes_a_non_canonical_slug_for_a_placement_singleton() {
    let repo = TempDir::new("placement-slug");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_vision(repo.path(), home.path());

    // The canonical slug still resolves the committed VISION.md at exit 0 (the valid read
    // stays working — the guard must not over-reject).
    let ok = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision"],
        None,
    );
    assert_ok(
        &ok,
        "`jigc doc show vision:vision` (the canonical singleton slug)",
    );
    assert!(
        stdout_of(&ok).contains("A deterministic context compiler for coding agents."),
        "the canonical placement read returns the committed VISION.md body",
    );

    // A NON-canonical slug names no committed doc — it must exit non-zero with the routed
    // not-found envelope, exactly like a non-placement bad ref, not return VISION.md.
    let bad = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:does-not-exist"],
        None,
    );
    assert!(
        !bad.status.success(),
        "a non-canonical placement slug must exit non-zero; stdout:\n{}\nstderr:\n{}",
        stdout_of(&bad),
        String::from_utf8_lossy(&bad.stderr),
    );
    assert!(
        !stdout_of(&bad).contains("A deterministic context compiler for coding agents."),
        "the invalid ref must NOT leak the singleton's committed content; stdout:\n{}",
        stdout_of(&bad),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("route:") && stderr.contains("does-not-exist"),
        "the block envelope names the bad ref + its route; got:\n{stderr}",
    );
}

/// A canonical committed ADR (the shipped `write::render` shape, stamped at the adr's
/// frozen `schema-version: 2`): its `status` **header** section is a fields-only section
/// — the shape 11 sections across 10 shipped doctypes carry.
const COMMITTED_ADR: &str = "\
---
status: accepted
date: 2026-05-23
schema-version: 2
---

# Single-node cache

## Context

Forces.

## Options

Alternatives were weighed and rejected.

## Decision

A single in-memory node.

## Consequences

None.
";

/// (M42 inc-8 T2) A **fields-only (header) section slice serves its fields.**
///
/// `jigc doc show 'adr:<slug>#status'` returned the **empty string at exit 0** — a header
/// that genuinely carries `status`/`date`/`schema-version` answering "nothing here": the
/// **wrong-node-exit-0** `design/doc-read-surface.md` forbids by name. Post-fix the slice
/// serves the section's fields — plain, the field lines as the writer renders them
/// (a canonical re-emit: a `ParsedSection` carries no field-group span); `--format json`,
/// the leaves keyed by leaf id, shaped exactly as the whole-doc `fields` project them.
///
/// Proven through the REAL binary on the EMITTED bytes, for the `adr` **and** for
/// `milestone-record` — the pinned contract's own conformance witness — plus the
/// regression guard the fix must not disturb: a fields-only section still contributes to
/// the whole-doc `fields` **alone** and gains no `sections` entry.
#[test]
fn fields_only_header_section_slice_serves_its_fields() {
    let repo = TempDir::new("fields-only");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // This harness's cascade lists the methodology pack, whose knobs whole-file-shadow
    // the dev base's — so `docs-root` is inert and the adr homes flat at `decisions/`.
    let decisions = repo.path().join("decisions");
    fs::create_dir_all(&decisions).expect("mk decisions/");
    fs::write(decisions.join("single-node-cache.md"), COMMITTED_ADR).expect("write committed adr");
    git(repo.path(), &["add", "decisions"]);
    git(repo.path(), &["commit", "-q", "-m", "adr"]);

    // (1) Plain: the header's field lines, as the writer renders them.
    let plain = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:single-node-cache#status"],
        None,
    );
    assert_ok(&plain, "`jigc doc show adr:single-node-cache#status`");
    assert_eq!(
        stdout_of(&plain).trim_end(),
        "status: accepted\ndate: 2026-05-23\nschema-version: 2",
        "the header slice serves its field lines, never the empty string",
    );

    // (2) `--format json`: the leaves keyed by leaf id, shaped as in `fields`.
    let json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "adr:single-node-cache#status",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&json, "`jigc doc show adr:...#status --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        "{\n  \"date\": \"2026-05-23\",\n  \"schema-version\": \"2\",\n  \"status\": \"accepted\"\n}",
        "a fields-only section slice is its leaves keyed by leaf id",
    );

    // (3) REGRESSION (the omitting context): the whole-doc shape is unchanged — the
    //     fields-only section still contributes to `fields` alone, with NO `sections`
    //     entry. A slice is a projection of the addressed node, not a relocation of it.
    let whole = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:single-node-cache", "--format", "json"],
        None,
    );
    assert_ok(
        &whole,
        "`jigc doc show adr:single-node-cache --format json`",
    );
    let whole = stdout_of(&whole);
    let value: serde_json::Value = serde_json::from_str(&whole).expect("valid json");
    assert_eq!(
        value["fields"]["status"], "accepted",
        "the header's leaves stay in the whole-doc `fields`; got:\n{whole}",
    );
    assert!(
        value["sections"].get("status").is_none(),
        "a fields-only section gains NO `sections` entry; got:\n{whole}",
    );
    assert_eq!(
        value["sections"]["decision"], "A single in-memory node.",
        "the slot sections are untouched; got:\n{whole}",
    );

    // (4) The pinned contract's own conformance witness: the `milestone-record` header.
    //     Its `base` pin is HEAD at create — read it BEFORE the create commits the record.
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo.path())
        .output()
        .expect("git rev-parse HEAD");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache rework"],
            None,
        ),
        "`jigc milestone create`",
    );
    let sha = String::from_utf8(head.stdout)
        .expect("utf-8 sha")
        .trim()
        .to_string();
    let short = &sha[..7];

    let plain = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "milestone-record:cache-rework#meta"],
        None,
    );
    assert_ok(&plain, "`jigc doc show milestone-record:cache-rework#meta`");
    assert_eq!(
        stdout_of(&plain).trim_end(),
        format!(
            "base: {sha} {short}\nstatus: active\nschema-version: {}",
            declared_schema_version("milestone-record")
        ),
        "the witness's header slice serves its field lines",
    );

    let json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "milestone-record:cache-rework#meta",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(
        &json,
        "`jigc doc show milestone-record:...#meta --format json`",
    );
    assert_eq!(
        stdout_of(&json).trim_end(),
        format!(
            "{{\n  \"base\": {{\n    \"sha\": \"{sha}\",\n    \"short\": \"{short}\"\n  }},\n  \"schema-version\": \"{}\",\n  \"status\": \"active\"\n}}",
            declared_schema_version("milestone-record")
        ),
        "the witness's leaves are shaped exactly as the whole-doc `fields` project them \
         (the compound `base` pin included)",
    );
}

/// (M47 inc-10 T5 — C6) **The leaf-purity guarantee, held to the emitted bytes.**
/// `doc show --help` now *states* that stdout carries the addressed content and nothing
/// else — no routing footer, no banner — with diagnostics on stderr. A trial worker had
/// already round-tripped a leaf through a shell file and verified it with `tail -c`
/// because the behaviour was stated nowhere; a stated guarantee that no test drives is
/// the same gap one register up, so the statement lands with its witness.
///
/// The axis is the **slice depth**: whole-doc, a slot section, a fields-only section,
/// and an item leaf all serve pure. Byte-exact on stdout, **zero bytes** on stderr.
#[test]
fn doc_show_stdout_carries_the_addressed_content_alone() {
    let repo = TempDir::new("leaf-purity");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let decisions = repo.path().join("decisions");
    fs::create_dir_all(&decisions).expect("mk decisions/");
    fs::write(decisions.join("single-node-cache.md"), COMMITTED_ADR).expect("write committed adr");
    git(repo.path(), &["add", "decisions"]);
    git(repo.path(), &["commit", "-q", "-m", "adr"]);

    // The banner that must never ride a read (`render.rs` — the routing footer).
    const FOOTER: &str = "— jigc · run `jigc start`";

    for (addr, expected) in [
        // A slot-section slice — the case the shell round-trip depends on.
        (
            "adr:single-node-cache#decision",
            "A single in-memory node.\n",
        ),
        // A fields-only (header) slice — a canonical re-emit, equally pure.
        (
            "adr:single-node-cache#status",
            "status: accepted\ndate: 2026-05-23\nschema-version: 2\n",
        ),
        // The whole doc — the store's own bytes. `println!` closes the serve with the
        // newline the stored bytes already end in, so the whole-doc arm is compared
        // with that one trailing newline named rather than folded away: the claim is
        // *no footer, no banner*, and a doubled line terminator is neither.
        ("adr:single-node-cache", &format!("{COMMITTED_ADR}\n")),
    ] {
        let out = jigc(repo.path(), home.path(), &["doc", "show", addr], None);
        assert_ok(&out, "`jigc doc show <addr>`");
        let stdout = String::from_utf8(out.stdout.clone()).expect("utf-8 stdout");
        assert_eq!(
            stdout, expected,
            "`{addr}` serves the addressed content alone, byte for byte",
        );
        assert!(
            !stdout.contains(FOOTER),
            "`{addr}` carries no routing footer; got:\n{stdout}",
        );
        assert!(
            out.stderr.is_empty(),
            "`{addr}` leaves stderr empty; got:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    // The omitting context that makes the guarantee mean something: a **block** on the
    // same verb DOES carry the footer — on stderr, with stdout empty. Purity is a
    // property of the served read, not of the verb.
    let bad = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:single-node-cache#nonesuch"],
        None,
    );
    assert!(!bad.status.success(), "an unresolvable address blocks");
    assert!(
        bad.stdout.is_empty(),
        "a blocked read writes nothing to stdout; got:\n{}",
        String::from_utf8_lossy(&bad.stdout),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains(FOOTER),
        "the block's diagnostic carries the footer, on stderr; got:\n{stderr}",
    );
}

/// A committed changelog at its literal placement home (`CHANGELOG.md`) — the shipped
/// nested-repeatable singleton, in the exact on-disk shape the writer mints.
const COMMITTED_CHANGELOG: &str = "\
# Changelog

## Unreleased Changes

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14

#### Added  {#added}

- initial release
";

/// The single live task id under `.jigc/tasks/` (the harness mints exactly one).
fn only_task(repo: &Path) -> String {
    let mut ids: Vec<String> = fs::read_dir(repo.join(".jigc").join("tasks"))
        .expect("read .jigc/tasks")
        .map(|entry| {
            entry
                .expect("task entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(ids.len(), 1, "exactly one live task; got {ids:?}");
    ids.pop().expect("the one task id")
}

/// (M42 inc-8 T5) **A bare singleton address resolves for the `doc` verbs.**
///
/// A **placement** doctype's slug is fixed to its type id (`design/storage.md` → Placement
/// — `engine::store::read_slice` refuses every *other* slug for one), so `changelog` names
/// its instance as unambiguously as `changelog:changelog` does — yet every `doc` verb
/// rejected the bare form with a bare-grammar *"missing ':' between type and slug"*. Five
/// singletons ship (`changelog`, `vision`, `roadmap`, `decisions-log`, `deferral-ledger`)
/// and the bare form is the natural first guess; the M41 V12 discoverability repair
/// reached `rename` only.
///
/// Proven on the EMITTED bytes of the real binary: the bare read (whole-doc and
/// `#fragment`) is byte-identical to its `<type>:<type>` spelling, a bare **non**-singleton
/// (`adr`) still errors — naming the `<type>:<slug>` form and the `jigc describe` pointer —
/// and a bare singleton on a **write** verb stages the very same instance.
#[test]
fn a_bare_singleton_address_resolves_for_the_doc_verbs() {
    let repo = TempDir::new("bare-singleton");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_vision(repo.path(), home.path());
    fs::write(repo.path().join("CHANGELOG.md"), COMMITTED_CHANGELOG).expect("write changelog");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(repo.path(), &["commit", "-q", "-m", "changelog"]);

    // (1) The bare read is byte-identical to the canonical spelling — whole-doc, for a
    //     dev-pack singleton (`changelog`) and a methodology-pack one (`vision`).
    for (bare, canonical) in [
        ("changelog", "changelog:changelog"),
        ("vision", "vision:vision"),
    ] {
        let bare_out = jigc(repo.path(), home.path(), &["doc", "show", bare], None);
        assert_ok(&bare_out, &format!("`jigc doc show {bare}`"));
        let canonical_out = jigc(repo.path(), home.path(), &["doc", "show", canonical], None);
        assert_ok(&canonical_out, &format!("`jigc doc show {canonical}`"));
        assert_eq!(
            bare_out.stdout, canonical_out.stdout,
            "`doc show {bare}` must emit exactly what `doc show {canonical}` does",
        );
        assert!(
            !stdout_of(&bare_out).trim().is_empty(),
            "the bare read serves the committed doc, not the empty string",
        );
    }

    // (2) A bare head carrying a `#fragment` expands too — plain and json.
    for format in [&[][..], &["--format", "json"][..]] {
        let mut bare = vec!["doc", "show", "changelog#releases"];
        bare.extend_from_slice(format);
        let mut canonical = vec!["doc", "show", "changelog:changelog#releases"];
        canonical.extend_from_slice(format);
        let bare_out = jigc(repo.path(), home.path(), &bare, None);
        assert_ok(&bare_out, "`jigc doc show changelog#releases`");
        let canonical_out = jigc(repo.path(), home.path(), &canonical, None);
        assert_ok(
            &canonical_out,
            "`jigc doc show changelog:changelog#releases`",
        );
        assert_eq!(
            bare_out.stdout, canonical_out.stdout,
            "a bare head with a `#fragment` resolves identically ({format:?})",
        );
        assert!(
            stdout_of(&bare_out).contains("1.0.0"),
            "the `#releases` slice carries the cut release; got:\n{}",
            stdout_of(&bare_out),
        );
    }

    // (3) A bare NON-singleton is still an error — the slug is genuinely missing — and the
    //     route names the address form + the doctype surface (the V12 repair, now here).
    let bad = jigc(repo.path(), home.path(), &["doc", "show", "adr"], None);
    assert!(
        !bad.status.success(),
        "a bare non-singleton must exit non-zero; stdout:\n{}",
        stdout_of(&bad),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("<type>:<slug>") && stderr.contains("jigc describe"),
        "the bare non-singleton error names the address form + the `jigc describe` \
         pointer; got:\n{stderr}",
    );

    // (4) The WRITE verbs take the bare singleton too — it stages the same instance the
    //     canonical spelling would (`.jigc/tasks/<id>/docs/vision:vision.md`).
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "form-vision", "revise the vision"],
            None,
        ),
        "`jigc start --workflow form-vision` (the revision task)",
    );
    let task = only_task(repo.path());
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                "vision#thesis",
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            Some(b"A context compiler, restated.\n"),
        ),
        "`jigc doc set-slot vision#thesis` (the bare singleton on a write verb)",
    );
    let staged = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(&task)
            .join("docs")
            .join("vision:vision.md"),
    )
    .expect("the bare write staged `vision:vision`");
    assert!(
        staged.contains("A context compiler, restated."),
        "the bare-addressed write landed in the singleton's staged instance; got:\n{staged}",
    );
}

/// M49 Increment 8 / T2 — the top-level `schema-version` key is **whole-doc only**, and
/// the `fields` map it sits beside is untouched (`design/doc-read-surface.md` → the
/// `schema-version` additive key: a `#fragment` slice carries no top-level key, the same
/// bound `item-count` and the staged marker already state).
///
/// The discriminating fragment is the header's **fields-only** slice (`vision:vision#meta`),
/// which is the one fragment shaped like an object with a `schema-version` entry in it —
/// and that entry must still be the **string**, exactly as `fields` projects it. A slot
/// slice is a bare string, so it has no object to hang a key on at all.
#[test]
fn the_stamp_key_is_whole_doc_only_and_fields_stays_stringy() {
    let repo = TempDir::new("stamp-fragment");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_vision(repo.path(), home.path());

    // (1) The whole-doc serve: the top-level key is a NUMBER, the `fields` entry the STRING.
    let whole = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision", "--format", "json"],
        None,
    );
    assert_ok(&whole, "`jigc doc show vision:vision --format json`");
    let whole: serde_json::Value = serde_json::from_str(&stdout_of(&whole)).expect("valid json");
    assert_eq!(whole["schema-version"], serde_json::json!(1));
    assert_eq!(whole["fields"]["schema-version"], serde_json::json!("1"));

    // (2) The fields-only header slice is the `fields` shape verbatim — string, no number.
    let meta = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision#meta", "--format", "json"],
        None,
    );
    assert_ok(&meta, "`jigc doc show vision:vision#meta --format json`");
    assert_eq!(
        stdout_of(&meta).trim_end(),
        "{\n  \"schema-version\": \"1\"\n}",
        "a fields-only slice projects the leaves as `fields` does — the stamp stays the \
         string, and the whole-doc-only number does not leak into it",
    );

    // (3) A slot slice is a bare value: no object, no key.
    let thesis = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision#thesis", "--format", "json"],
        None,
    );
    assert_ok(
        &thesis,
        "`jigc doc show vision:vision#thesis --format json`",
    );
    let thesis: serde_json::Value = serde_json::from_str(&stdout_of(&thesis)).expect("valid json");
    assert!(
        thesis.is_string() && thesis.get("schema-version").is_none(),
        "a slot slice is the bare prose string; got:\n{thesis}"
    );
}

/// M51 Increment 9, T10 (EC-11, part 2) — **the pinned whole-doc key set is what the
/// binary emits**, committed and staged.
///
/// The key set existed only inside the whole-output goldens above (`VISION_JSON` /
/// `PRD_JSON`) — an equality that pins the shape but that nothing else can read — so
/// `jigc doc show --help` restated it in prose and the restatement went stale: it
/// named the four keys pinned at M39 while the serve has carried **six** since M49
/// (`item-count` M44, the top-level `schema-version` M49). The set is now
/// [`cli::doc::WHOLE_DOC_KEYS`], the help renders it, and this arm is the half that
/// keeps the const honest against reality: it compares the const to the key set the
/// **real binary prints**, so a seventh key added to the serve reddens here before the
/// help can under-state it.
///
/// Both serves are driven, because the staged one carries the one additive key
/// ([`cli::doc::STAGED_KEY`]) and the committed one must carry it **never** — that is
/// the committed/staged discriminator itself (`design/doc-read-surface.md` → The
/// staged marker key).
#[test]
fn the_driven_whole_doc_key_set_is_the_pinned_const() {
    let repo = TempDir::new("whole-doc-keys");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_vision(repo.path(), home.path());

    let keys_of = |out: &std::process::Output| -> Vec<String> {
        let value: serde_json::Value =
            serde_json::from_str(&stdout_of(out)).expect("the `--format json` serve is json");
        value
            .as_object()
            .expect("a whole-doc serve is a json object")
            .keys()
            .cloned()
            .collect()
    };

    // (1) The committed serve carries exactly the pinned set.
    let committed = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision", "--format", "json"],
        None,
    );
    assert_ok(&committed, "`jigc doc show vision:vision --format json`");
    let mut expected: Vec<String> = cli::doc::WHOLE_DOC_KEYS
        .iter()
        .map(|key| (*key).to_string())
        .collect();
    expected.sort();
    assert_eq!(
        keys_of(&committed),
        expected,
        "the committed whole-doc serve's top-level keys must BE \
         `cli::doc::WHOLE_DOC_KEYS` — the const `jigc doc show --help` renders",
    );

    // (2) The staged serve adds the one marker key and nothing else.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "form-vision", "revise the vision"],
            None,
        ),
        "`jigc start --workflow form-vision` (the revision task)",
    );
    let task = only_task(repo.path());
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                "vision:vision#thesis",
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            Some(b"A context compiler, restated.\n"),
        ),
        "`jigc doc set-slot vision:vision#thesis` (stage the doc into the task)",
    );
    let staged = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "vision:vision",
            "--format",
            "json",
            "--task",
            &task,
        ],
        None,
    );
    assert_ok(
        &staged,
        "`jigc doc show vision:vision --format json --task`",
    );
    let mut staged_expected = expected.clone();
    staged_expected.push(cli::doc::STAGED_KEY.to_string());
    staged_expected.sort();
    assert_eq!(
        keys_of(&staged),
        staged_expected,
        "the staged whole-doc serve's top-level keys must be the pinned set plus the \
         one `{}` marker — the committed/staged discriminator",
        cli::doc::STAGED_KEY,
    );
}
