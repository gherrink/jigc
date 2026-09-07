//! T2 acceptance — `doc-code` scheduled at the live `task validate` gate, end-to-end
//! through the built binary against the **real** test-built `doc-code` probe.
//!
//! This is the wiring half of M10's headline (`design/validation.md` → The `doc-code`
//! probe; `implementation/roadmap.md` → Increment 5, T2): the engine enumerates the
//! task's effective-state `code-anchor` leaves, materializes the serializable snapshot,
//! and ingests the probe's findings into the `ValidationReport`; the CLI invokes the
//! real `doc-code` subprocess (the engine stays shell-free). The flow-13 *finalize*
//! walk is T3 — here the proof is `jigc task validate`'s findings + exit code:
//!
//! - **(i) present symbol** — a created ADR whose `cites-code` anchors a symbol the
//!   working tree carries → validate surfaces a **resolving** (non-blocking) state for
//!   `doc-code.symbol-exists` (no finding) and exits clean.
//! - **(ii) absent symbol** — the same anchor pointed at a symbol the tree lacks →
//!   a **blocking** `doc-code.symbol-exists` naming the dangling target, exit non-zero.
//! - **(iii) the omitting-context guard (hardening #5)** — a task whose created ADR
//!   carries **no** `cites-code` anchor produces **zero** `doc-code` findings and
//!   behaves byte-identically to the pre-wiring path (a clean validate), while the
//!   task WITH the anchor (i) is what produces the finding. A green pass over (i)/(ii)
//!   alone would hide a scope bug in every anchor-less task.
//!
//! The probe binary is built once (a `cargo build` of the pack's `doc-code` crate) and
//! selected via `JIGC_DOC_CODE_PROBE` (the production env-var override, `JIGC_PACK_DIR`
//! precedent). No external test crates: the `jigc` path comes from `CARGO_BIN_EXE_jigc`,
//! the temp repo is a real `git init`, and self-cleaning `TempDir`s keep the dev repo clean.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-doc-code-gate-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path.
/// A real subprocess the engine/CLI seam drives — never a mock.
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer, and
/// a `src/lib.rs` carrying a known symbol the `cites-code` anchor can resolve against.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("src")).expect("mk src");
    // A real symbol the present-anchor case resolves against.
    fs::write(
        repo.join("src").join("lib.rs"),
        "pub fn present_symbol() -> u32 {\n    42\n}\n",
    )
    .expect("write lib.rs");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the doc-code probe selected via
/// `JIGC_DOC_CODE_PROBE`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and an **explicit**
/// `JIGC_DOC_CODE_PROBE` (the `validate_command.rs` negative idiom) — pointed at an
/// unresolvable path to drive the probe-absent pre-flight at task scope.
fn jigc_with_probe(repo: &Path, home: &Path, args: &[&str], probe: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", probe)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
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

/// Start a single-task task with `intent`, returning its task id (the intent slug).
fn start_task(repo: &Path, home: &Path, intent: &str) {
    let out = jigc(repo, home, &["start", "--workflow", "single-task", intent]);
    assert_ok(&out, "`jigc start`");
}

/// Create an ADR titled `title`, asserting the minted address.
fn create_adr(repo: &Path, home: &Path, title: &str, expect_addr: &str) {
    let create = jigc_doc(repo, home, &["create", "adr", "--title", title], None);
    assert_ok(&create, "`jigc doc create adr`");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr, expect_addr, "minted ADR address");
}

/// Fill the created ADR's author-required prose slots so the only possible blocking
/// finding is the `doc-code` one (or none).
fn fill_adr_slots(repo: &Path, home: &Path, slug: &str) {
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot(&format!("adr:{slug}#context"), b"Forces at play.\n");
    set_slot(&format!("adr:{slug}#decision"), b"We decided.\n");
    set_slot(&format!("adr:{slug}#consequences"), b"Tradeoffs.\n");
}

/// Fill every author-required field/slot of the provisioned commit doc so a validate
/// over the working area is clean of `schema-conformance` blocks — leaving `doc-code`
/// as the only lever the present/anchorless cases turn.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

/// Inject a `cites-code: <anchor>` line into the staged ADR's empty front-matter block
/// (the optional anchor is absent from the created skeleton; injecting the canonical
/// `key: value` line keeps this test's concern the gate, not the write path — mirrors
/// `superseding_decision::inject_supersedes`).
fn inject_cites_code(repo: &Path, task: &str, slug: &str, anchor: &str) {
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"));
    let body = fs::read_to_string(&staged).expect("read staged ADR");
    // Insert the `cites-code` line before the closing front-matter fence (the created
    // ADR now carries materialized `status`/`date` header lines, not an empty fence).
    let with = body.replacen("\n---\n", &format!("\ncites-code: {anchor}\n---\n"), 1);
    assert_ne!(
        body, with,
        "the staged ADR carries a front-matter block to inject the anchor into"
    );
    fs::write(&staged, &with).expect("inject cites-code anchor");
}

/// (i) A `cites-code` anchored at a PRESENT symbol → validate surfaces no `doc-code`
/// finding (the resolving case) and exits clean.
#[test]
fn present_anchor_resolves_clean_at_validate() {
    let repo = TempDir::new("present-repo");
    let home = TempDir::new("present-home");
    init_repo(repo.path());

    let task = "cite-the-present-symbol";
    start_task(repo.path(), home.path(), "cite the present symbol");
    create_adr(repo.path(), home.path(), "Present cite", "adr:present-cite");
    fill_adr_slots(repo.path(), home.path(), "present-cite");
    fill_commit(repo.path(), home.path(), task);
    inject_cites_code(
        repo.path(),
        task,
        "present-cite",
        "src/lib.rs#present_symbol",
    );

    let out = jigc(repo.path(), home.path(), &["task", "validate", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("validate with a present anchor; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("doc-code.symbol-exists"),
        "a present anchor must surface no doc-code.symbol-exists finding; got:\n{rendered}",
    );
}

/// (ii) A `cites-code` anchored at an ABSENT symbol → a blocking `doc-code.symbol-exists`
/// naming the dangling target, validate exits non-zero.
#[test]
fn absent_anchor_blocks_at_validate_naming_the_target() {
    let repo = TempDir::new("absent-repo");
    let home = TempDir::new("absent-home");
    init_repo(repo.path());

    let task = "cite-an-absent-symbol";
    start_task(repo.path(), home.path(), "cite an absent symbol");
    create_adr(repo.path(), home.path(), "Absent cite", "adr:absent-cite");
    fill_adr_slots(repo.path(), home.path(), "absent-cite");
    fill_commit(repo.path(), home.path(), task);
    inject_cites_code(
        repo.path(),
        task,
        "absent-cite",
        "src/lib.rs#no_such_symbol",
    );

    let out = jigc(repo.path(), home.path(), &["task", "validate", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "an absent anchor must make validate exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces the doc-code.symbol-exists check; got:\n{rendered}",
    );
    assert!(
        rendered.contains("no_such_symbol"),
        "the block names the dangling anchor target; got:\n{rendered}",
    );
}

/// (iii) The omitting-context guard (hardening #5): a created ADR with **no**
/// `cites-code` anchor produces **zero** `doc-code` findings and validates clean —
/// byte-identically to the pre-wiring path. Paired with (i)/(ii) so a green pass over a
/// single anchored context can't hide a scope bug in every anchor-less task.
#[test]
fn anchorless_task_produces_no_doc_code_finding() {
    let repo = TempDir::new("anchorless-repo");
    let home = TempDir::new("anchorless-home");
    init_repo(repo.path());

    let task = "no-anchor-at-all";
    start_task(repo.path(), home.path(), "no anchor at all");
    create_adr(repo.path(), home.path(), "No cite", "adr:no-cite");
    fill_adr_slots(repo.path(), home.path(), "no-cite");
    fill_commit(repo.path(), home.path(), task);
    // Deliberately inject NO cites-code anchor — the omitting context.

    let out = jigc(repo.path(), home.path(), &["task", "validate", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(&out, &format!("validate with no anchor; got:\n{rendered}"));
    assert!(
        !rendered.contains("doc-code"),
        "an anchor-less task must surface zero doc-code findings; got:\n{rendered}",
    );
}

/// Stage an anchored task (a created ADR carrying a `cites-code` over a present symbol,
/// every author-required slot filled) so the effective state carries code-anchor work —
/// the surface the task-scope probe pre-flight gates on. The anchor targets the present
/// symbol so the **control** (real probe) validates/finalizes clean.
fn stage_anchored_task(repo: &Path, home: &Path, task: &str, slug: &str) {
    start_task(repo, home, task.replace('-', " ").as_str());
    create_adr(repo, home, &slug.replace('-', " "), &format!("adr:{slug}"));
    fill_adr_slots(repo, home, slug);
    fill_commit(repo, home, task);
    inject_cites_code(repo, task, slug, "src/lib.rs#present_symbol");
}

/// T3 — the absence-vs-crash fix: with the `doc-code` probe **unresolvable**
/// (`JIGC_DOC_CODE_PROBE` at a non-existent path), an anchored task's `jigc task
/// validate` exits non-zero with **exactly one** `doc-code` probe-not-found operational
/// error on stderr — not N per-anchor `pack-probe-integrity.crash` findings, and no
/// crash meta-finding at all (the sweep never ran). The count is asserted so an
/// N-per-anchor regression fails (`module-layout.md` → Probe distribution, the task-scope
/// absence fix; `design/validation.md` → Distribution bound).
#[test]
fn task_validate_probe_absent_reports_one_operational_error_non_zero() {
    let repo = TempDir::new("task-validate-absent-repo");
    let home = TempDir::new("task-validate-absent-home");
    init_repo(repo.path());

    let task = "validate-probe-absent";
    stage_anchored_task(repo.path(), home.path(), task, "validate-probe-absent");

    let missing = repo.path().join("nonexistent-doc-code-probe");
    let out = jigc_with_probe(
        repo.path(),
        home.path(),
        &["task", "validate", task],
        &missing,
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        !out.status.success(),
        "a missing probe must make task validate exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        stderr.matches("`doc-code` probe not found").count(),
        1,
        "exactly one `doc-code probe not found` operational error must surface on stderr \
         (an N-per-anchor crash regression fails this); stderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("pack-probe-integrity") && !stderr.contains("pack-probe-integrity"),
        "the sweep must not run when the probe is absent — no pack-probe-integrity meta-finding; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// T3 — the same absence-vs-crash fix at the **finalize** commit boundary: an anchored
/// task's `jigc task finalize` with an unresolvable probe exits non-zero with exactly one
/// `doc-code` probe-not-found operational error and no `pack-probe-integrity.crash` floor.
#[test]
fn task_finalize_probe_absent_reports_one_operational_error_non_zero() {
    let repo = TempDir::new("task-finalize-absent-repo");
    let home = TempDir::new("task-finalize-absent-home");
    init_repo(repo.path());

    let task = "finalize-probe-absent";
    stage_anchored_task(repo.path(), home.path(), task, "finalize-probe-absent");

    let missing = repo.path().join("nonexistent-doc-code-probe");
    let out = jigc_with_probe(
        repo.path(),
        home.path(),
        &["task", "finalize", task],
        &missing,
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        !out.status.success(),
        "a missing probe must make task finalize exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        stderr.matches("`doc-code` probe not found").count(),
        1,
        "exactly one `doc-code probe not found` operational error must surface on stderr \
         (an N-per-anchor crash regression fails this); stderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("pack-probe-integrity") && !stderr.contains("pack-probe-integrity"),
        "the sweep must not run when the probe is absent — no pack-probe-integrity meta-finding; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

/// T3 control — the present-probe case still works: the same anchored task with the
/// **real** probe present validates clean (no `doc-code.symbol-exists` block, the anchor
/// resolves) — proving the pre-flight is inert when the probe is resolvable, not a blanket
/// block on anchored tasks.
#[test]
fn task_validate_probe_present_resolves_clean_control() {
    let repo = TempDir::new("task-validate-present-repo");
    let home = TempDir::new("task-validate-present-home");
    init_repo(repo.path());

    let task = "validate-probe-present";
    stage_anchored_task(repo.path(), home.path(), task, "validate-probe-present");

    let out = jigc(repo.path(), home.path(), &["task", "validate", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("validate with the real probe present must stay clean; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("doc-code.symbol-exists"),
        "a present anchor with the real probe must surface no doc-code block; got:\n{rendered}",
    );
}

/// The CLI's own `today_iso` (`doc.rs::today_iso`), reproduced in-test from the **same**
/// `std::time` source — never a hardcoded literal, so the date the materializer stamps
/// and the date this test expects derive from one clock (M22 inc-4 done-criterion).
fn today_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let z = (secs / 86_400) as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let (y, m, d) = (if m <= 2 { y + 1 } else { y }, m, d);
    format!("{y:04}-{m:02}-{d:02}")
}

/// The `adr` schema, loaded with the dev-pack `code-anchor` field-type (so a re-parse of
/// the staged bytes resolves `cites-code`) — mirrors `arch_doc_components_materialize`.
fn adr_schema() -> engine::schema::Schema {
    const ADR_YAML: &[u8] = include_bytes!("../pack/schemas/adr.yaml");
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
        hint: None,
    }];
    let mut schema =
        engine::schema::load_schema_with_types(ADR_YAML, &types).expect("adr.yaml loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// M22 inc-4 (engine work #4) — a freshly-created `adr` carries its schema-declared
/// doc-level materializations: `status: proposed` (from `default`) and `date: <today>`
/// (from `set: on-create`), in schema field order, with **no** stray `supersedes` /
/// `cites-code` line (those have neither default nor set). The staged bytes round-trip
/// byte-stable (`render(parse(staged)) == staged`). The expected date is the CLI's own
/// `today_iso`, computed in-test from the same `std::time` source — never hardcoded.
#[test]
fn created_adr_materializes_doc_level_status_and_date() {
    let repo = TempDir::new("adr-materialize-repo");
    let home = TempDir::new("adr-materialize-home");
    init_repo(repo.path());

    let task = "record-the-cache-call";
    start_task(repo.path(), home.path(), task);
    create_adr(
        repo.path(),
        home.path(),
        "Single-node cache",
        "adr:single-node-cache",
    );

    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("adr:single-node-cache.md");
    let body = fs::read_to_string(&staged).expect("read staged ADR");

    let expected_fm = format!(
        "---\nstatus: proposed\ndate: {}\nschema-version: 2\n---\n",
        today_iso()
    );
    assert!(
        body.starts_with(&expected_fm),
        "created adr front-matter must be exactly status (default) then date (on-create) then the \
         schema-version stamp in schema order, no stray supersedes/cites-code line; expected \
         prefix:\n{expected_fm}\ngot:\n{body}",
    );

    // The materialized doc round-trips byte-stable through the engine.
    let schema = adr_schema();
    let parsed = engine::write::instance_from_source(&schema, &body).expect("staged adr re-parses");
    let rerendered = engine::write::render(&schema, &parsed);
    assert_eq!(
        rerendered, body,
        "render(parse(staged)) must equal the staged bytes",
    );
}

/// Append a top-level `pub fn <name>()` to the tracked `src/lib.rs` (alongside
/// `present_symbol`) — the agent writing a cited code symbol into a tracked file.
fn append_symbol(repo: &Path, name: &str) {
    let lib = repo.join("src").join("lib.rs");
    let mut body = fs::read_to_string(&lib).expect("read src/lib.rs");
    body.push_str(&format!("\npub fn {name}() -> u32 {{\n    7\n}}\n"));
    fs::write(&lib, body).expect("append symbol to src/lib.rs");
}

/// Set up an anchored task whose created ADR `cites-code` the symbol `symbol` in the
/// tracked `src/lib.rs` (every author-required slot/field filled, so the only lever is the
/// `doc-code` gate). The symbol is **not** written or staged here — the caller controls
/// the index state the gate sees (G4).
fn stage_citing_task(repo: &Path, home: &Path, task: &str, slug: &str, symbol: &str) {
    start_task(repo, home, task.replace('-', " ").as_str());
    create_adr(repo, home, &slug.replace('-', " "), &format!("adr:{slug}"));
    fill_adr_slots(repo, home, slug);
    fill_commit(repo, home, task);
    inject_cites_code(repo, task, slug, &format!("src/lib.rs#{symbol}"));
}

/// G4 (a) — a `cites-code` to a symbol the agent wrote into a tracked file **and staged**
/// → `finalize` PASSES: the materialized index carries the staged symbol, so the
/// finalize-scope `doc-code` probe resolves it.
#[test]
fn finalize_passes_when_cited_symbol_is_staged() {
    let repo = TempDir::new("g4-staged-repo");
    let home = TempDir::new("g4-staged-home");
    init_repo(repo.path());

    let task = "cite-a-staged-symbol";
    stage_citing_task(
        repo.path(),
        home.path(),
        task,
        "staged-cite",
        "staged_symbol",
    );

    // The agent writes the cited symbol into the tracked file AND stages it.
    append_symbol(repo.path(), "staged_symbol");
    git(repo.path(), &["add", "src/lib.rs"]);

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("finalize with the cited symbol staged must pass; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("doc-code.symbol-exists"),
        "a staged cited symbol must surface no doc-code.symbol-exists block; got:\n{rendered}",
    );
}

/// G4 (b) — the **same** symbol left **unstaged** (present only on disk, with an unrelated
/// staged file so the narrowed set is non-empty) → `finalize` BLOCKS naming the dangling
/// anchor: the materialized index lacks the symbol, so the finalize-scope `doc-code` probe
/// catches it (RED before the index split — the probe read the working tree and passed —
/// GREEN after). This is the keystone: validated reality == committed reality.
#[test]
fn finalize_blocks_when_cited_symbol_is_unstaged() {
    let repo = TempDir::new("g4-unstaged-repo");
    let home = TempDir::new("g4-unstaged-home");
    init_repo(repo.path());

    let task = "cite-an-unstaged-symbol";
    stage_citing_task(
        repo.path(),
        home.path(),
        task,
        "unstaged-cite",
        "unstaged_symbol",
    );

    // The agent writes the cited symbol into the tracked file but does NOT stage it.
    append_symbol(repo.path(), "unstaged_symbol");
    // An unrelated staged file so the narrowed finalize set is non-empty — the block is
    // the doc-code gate, never the empty-commit guard.
    fs::write(repo.path().join("other.rs"), "pub fn other() {}\n").expect("write other.rs");
    git(repo.path(), &["add", "other.rs"]);

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "an unstaged cited symbol must make finalize block (index, not working tree); got:\n{rendered}",
    );
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces the doc-code.symbol-exists check; got:\n{rendered}",
    );
    assert!(
        rendered.contains("unstaged_symbol"),
        "the block names the dangling anchor target; got:\n{rendered}",
    );
}

/// G4 (d) — no false-block: a cited symbol that **is staged** in a file carrying an
/// **unrelated unstaged hunk** → `finalize` PASSES. The index holds the staged symbol; the
/// later unstaged hunk never reaches the index, so the probe resolves the citation.
#[test]
fn finalize_passes_when_staged_symbol_has_unrelated_unstaged_hunk() {
    let repo = TempDir::new("g4-hunk-repo");
    let home = TempDir::new("g4-hunk-home");
    init_repo(repo.path());

    let task = "cite-a-symbol-with-hunk";
    stage_citing_task(repo.path(), home.path(), task, "hunk-cite", "hunk_symbol");

    // Write + stage the cited symbol.
    append_symbol(repo.path(), "hunk_symbol");
    git(repo.path(), &["add", "src/lib.rs"]);
    // An unrelated unstaged hunk in the SAME file — present on disk, absent from the index.
    append_symbol(repo.path(), "later_unstaged");

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!(
            "a staged symbol in a file with an unrelated unstaged hunk must not false-block; got:\n{rendered}"
        ),
    );
    assert!(
        !rendered.contains("doc-code.symbol-exists"),
        "the staged symbol resolves against the index — no false doc-code block; got:\n{rendered}",
    );
}
