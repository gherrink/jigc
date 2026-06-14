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
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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
    let with = body.replacen(
        "---\n---\n",
        &format!("---\ncites-code: {anchor}\n---\n"),
        1,
    );
    assert_ne!(
        body, with,
        "the staged ADR carries an empty front-matter block"
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
