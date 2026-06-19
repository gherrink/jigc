//! M20 inc-2 / T4 — the **headline acceptance** for the completed `jigc validate`
//! envelope, exercised through the **real `jigc` binary** (the M10 invocation-path-
//! masking lesson: drive the bytes an operator would actually run, never a
//! reconstructed equivalent). Composes T1 (the workflow↔refs store target), T2 (the
//! read-only file↔CLI-state twin), and T3 (the three-family `validate_store` reshape).
//!
//! Built against `implementation/roadmap.md` → M20 Increment 2 Deliverable + Proves and
//! `design/validation.md` → Completing the envelope. The store is set up via `jigc setup`
//! over a real `git init` temp repo, and the real `doc-code` probe is selected via
//! `JIGC_DOC_CODE_PROBE` (the `validate_command.rs` idiom) so the probe pre-flight passes.
//!
//! The four proofs:
//!
//! - **(i) both content families surface, report-only.** A store seeded with (a) a
//!   project workflow shadow carrying a **dangling workflow-ref** (`{{ include:
//!   step:not-a-step }}`) and (b) a committed ADR **edited out-of-band** so its on-disk
//!   hash diverges from its `FileStateRecord` → `jigc validate` **exits 0** and lists
//!   BOTH a `workflow-refs.*` dangling-ref finding AND a `file-state.hash-matches` drift
//!   finding (read-only, never gating — `validation.md` → Exit semantics).
//! - **(ii) the file-state twin is mutation-free.** `.jigc/state/file-state.json` is
//!   byte-identical before and after the run: the twin detects without absorbing — it
//!   never re-baselines the very drift it reports.
//! - **(iii) the B1 no-false-drift guard.** `jigc validate` over a **clean** real-dev-
//!   pack store emits **no** `workflow-refs.*` finding — the membership-only command-ref
//!   path fabricates no `task-ref-in-no-task-workflow` / `placeholder-resolves` drift over
//!   the pack's every workflow (the design-review B1 fix).
//! - **(iv) the operational-error exit class is intact.** With the `doc-code` probe
//!   unresolvable, `jigc validate` still exits **non-zero** with the one operational error
//!   — the content-only-exits-0 rule did not loosen the probe-missing exit.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init`, the probe is the real built `doc-code`, and a self-cleaning
//! `TempDir` keeps the test off the developer's repo.

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
            "jigc-validate-envelope-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// the **real** tree-sitter subprocess the engine/CLI seam drives, so the `jigc validate`
/// probe pre-flight resolves (the `validate_command.rs` idiom).
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
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe
/// selected via `JIGC_DOC_CODE_PROBE` so the validate pre-flight resolves.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    jigc_with_probe(repo, home, args, doc_code_probe())
}

/// Run `jigc <args>` with an explicit `JIGC_DOC_CODE_PROBE` — lets the operational-error
/// control point the override at a path that does not resolve to an executable.
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

/// Make `root` a real git repo with identity, then run `jigc setup` over it (the
/// project layer + probe extraction). Returns once the store is a clean, set-up repo.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// Fill an ADR's author-required prose slots so a finalize over it validates clean.
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
    set_slot(
        &format!("adr:{slug}#context"),
        b"Session lookups must stay sub-millisecond.\n",
    );
    set_slot(
        &format!("adr:{slug}#decision"),
        b"Keep sessions in a single in-memory node.\n",
    );
    set_slot(
        &format!("adr:{slug}#consequences"),
        b"A cold node loses its sessions.\n",
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
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

/// Run one whole task that creates + finalizes `adr:single-node-cache`, committing it to
/// `decisions/single-node-cache.md` and baselining it in the file-state record — the
/// committed managed doc the OOB edit later drifts (the `superseding_decision.rs` idiom).
fn commit_baselined_adr(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start`");
    let task = "cache-sessions-in-a-single-in-memory-node";

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr, "adr:single-node-cache");

    fill_adr_slots(repo, home, "single-node-cache");
    fill_commit(repo, home, task);

    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize`");

    // The ADR is committed at its canonical path AND baselined in the file-state record.
    assert!(
        repo.join("docs")
            .join("decisions")
            .join("single-node-cache.md")
            .exists(),
        "finalize must commit docs/decisions/single-node-cache.md",
    );
    assert!(
        repo.join(".jigc")
            .join("state")
            .join("file-state.json")
            .exists(),
        "finalize must baseline the committed ADR in the file-state record",
    );
}

/// Seed a project workflow shadow `.jigc/config/workflows/single-task.yaml` carrying a
/// **dangling** `{{ include: step:not-a-step }}` — a native whole-file shadow the cascade
/// owns (`file_owner(single-task) == Project`), so the store-scope workflow↔refs
/// enumeration reads it and the task-independent `include-resolves` check flags it.
fn seed_dangling_workflow_ref(repo: &Path) {
    let workflows = repo.join(".jigc").join("config").join("workflows");
    fs::create_dir_all(&workflows).expect("create project workflows dir");
    fs::write(
        workflows.join("single-task.yaml"),
        "---\n\
         when: implement one scoped change end-to-end\n\
         creates-task: true\n\
         ---\n\
         {{ include: step:not-a-step }}\n",
    )
    .expect("write the dangling workflow shadow");
}

/// (i) + (ii) The headline acceptance: a store carrying a dangling workflow-ref AND an
/// out-of-band committed-doc edit → `jigc validate` exits 0 listing BOTH content findings
/// (report-only), and the file-state record is byte-identical across the run (the twin
/// detects without absorbing).
#[test]
fn validate_surfaces_both_content_families_report_only_and_mutation_free() {
    let repo = TempDir::new("both");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // A committed, baselined ADR — using the real `single-task` workflow (the dangling
    // shadow is seeded AFTER, so this task composes the genuine pack workflow).
    commit_baselined_adr(repo.path(), home.path());

    // (a) The dangling workflow-ref — a project shadow over `single-task`.
    seed_dangling_workflow_ref(repo.path());

    // (b) The out-of-band edit: rewrite the committed ADR so its on-disk hash diverges
    //     from the recorded baseline (an OOB edit no `task validate`/`finalize` would see).
    let committed = repo
        .path()
        .join("docs")
        .join("decisions")
        .join("single-node-cache.md");
    let mut body = fs::read_to_string(&committed).expect("read the committed ADR");
    body.push_str("\nAn out-of-band human edit appended after baseline.\n");
    fs::write(&committed, &body).expect("apply the out-of-band edit");

    // Snapshot the file-state record BEFORE the run (the mutation-free comparand).
    let record_path = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    let record_before = fs::read(&record_path).expect("read file-state.json before validate");

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // (i) report-only: a content-only run (no probe-integrity meta-finding) exits 0.
    assert!(
        out.status.success(),
        "`jigc validate` over content-only drift must exit 0 (report-only); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("workflow-refs.include-resolves"),
        "the dangling workflow-ref must surface a workflow-refs.* finding; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("file-state.hash-matches")
            && stdout.contains("docs/decisions/single-node-cache.md"),
        "the out-of-band committed-doc edit must surface a file-state.hash-matches drift \
         naming the drifted path; stdout:\n{stdout}",
    );
    // A healthy probe over a content-only sweep raises no meta-finding.
    assert!(
        !stdout.contains("pack-probe-integrity"),
        "a content-only sweep must raise no pack-probe-integrity meta-finding; stdout:\n{stdout}",
    );

    // (ii) mutation-free: the file-state record is byte-identical — the twin did NOT
    //      re-baseline the drift it just reported.
    let record_after = fs::read(&record_path).expect("read file-state.json after validate");
    assert_eq!(
        record_before, record_after,
        "the file-state twin must not re-baseline (mutate) the drift it reports",
    );
}

/// (iii) The B1 no-false-drift guard: `jigc validate` over a CLEAN real-dev-pack store
/// (no project workflow shadow, no OOB edit) emits NO `workflow-refs.*` finding — the
/// membership-only command-ref path fabricates no `task-ref-in-no-task-workflow` /
/// `placeholder-resolves` drift over the pack's every workflow definition.
#[test]
fn validate_clean_dev_pack_store_emits_no_workflow_refs_drift() {
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "`jigc validate` over a clean dev-pack store must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("workflow-refs."),
        "the workflow↔refs target must emit NO finding over the clean real dev pack \
         (no fabricated task-ref / placeholder drift); stdout:\n{stdout}",
    );
}

/// (iv) The operational-error exit class is intact: with the `doc-code` probe
/// unresolvable, `jigc validate` still exits non-zero with the one operational error —
/// the content-only-exits-0 rule did not loosen the probe-missing exit (the
/// `validate_command.rs` pre-flight contract, held across the completed envelope).
#[test]
fn validate_unresolvable_probe_still_exits_non_zero_with_one_error() {
    let repo = TempDir::new("probe-absent");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    let missing = repo.path().join("nonexistent-doc-code-probe");
    let out = jigc_with_probe(repo.path(), home.path(), &["validate"], &missing);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        !out.status.success(),
        "an unresolvable probe must exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    let hits = stderr.matches("`doc-code` probe not found").count();
    assert_eq!(
        hits, 1,
        "exactly one `doc-code probe not found` operational error must surface on stderr; \
         stderr:\n{stderr}",
    );
}

/// The cascade-invariant guard: a **project-layer schema shadow** that changes a
/// doctype's `location:` must be honored by the store sweep — schemas resolve through
/// the cascade (`project > team > pack-default` for ALL customization), not pack-only.
///
/// Seed a clean store, then drop a project `schemas/adr.yaml` whole-file shadow that
/// relocates `adr` from the pack's `decisions/` to `adrs/`. The cascade auto-shadows the
/// id (`file_owner(adr) == Project`), so the resolved `adr` schema's `location:` is now
/// `adrs/`. Commit an `adr` doc at the **project-shadowed** location `adrs/cache.md` with
/// no baseline record. The file↔CLI-state twin enumerates committed docs by walking each
/// resolved schema's `location:`, so a cascade-aware sweep walks `adrs/`, finds the
/// un-baselined doc, and surfaces a `file-state.un-baselined` advisory naming it. A
/// **pack-only** sweep walks the pack's `decisions/` instead, never sees `adrs/cache.md`,
/// and emits no such finding — the regression this guards. Content-only, so exit 0.
#[test]
fn validate_honors_project_schema_location_shadow_in_store_sweep() {
    let repo = TempDir::new("schema-shadow");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // The project schema shadow: the shipped `adr` schema body verbatim EXCEPT its
    // `location:` is relocated `decisions/` → `adrs/`. A whole-file shadow the cascade
    // owns by id (`overrides.md` → Authored metadata on a definition resolves by
    // whole-file shadow), so the resolved `adr` schema's location becomes `adrs/`.
    let schemas = repo.path().join(".jigc").join("config").join("schemas");
    fs::create_dir_all(&schemas).expect("create project schemas dir");
    fs::write(
        schemas.join("adr.yaml"),
        "type: adr\n\
         location: adrs/\n\
         id-from: title\n\
         description: A dated architectural decision record.\n\
         usage: a choice is worth preserving with its rationale.\n\
         \n\
         sections:\n\
        \x20 - id: status\n\
        \x20   header: true\n\
        \x20   fields:\n\
        \x20     - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }\n\
        \x20     - { id: date, type: date, set: on-create }\n\
        \x20     - { id: supersedes, type: ref, to: adr, card: \"0..*\", inverse: superseded-by }\n\
        \x20     - { id: cites-code, type: code-anchor }\n\
        \x20 - id: context\n\
        \x20   slot: { hint: \"Why a decision was needed.\" }\n\
        \x20 - id: decision\n\
        \x20   slot: { hint: \"What we decided.\" }\n\
        \x20 - id: consequences\n\
        \x20   slot: { hint: \"Tradeoffs and follow-on effects.\" }\n",
    )
    .expect("write the project adr schema shadow");

    // A committed `adr` at the PROJECT-SHADOWED location `adrs/cache.md` — not baselined,
    // so the read-only twin classifies it un-baselined once it enumerates `adrs/`.
    let adrs = repo.path().join("docs").join("adrs");
    fs::create_dir_all(&adrs).expect("create adrs dir");
    fs::write(
        adrs.join("cache.md"),
        "---\n\
         status: accepted\n\
         date: 2026-06-14\n\
         ---\n\
         \n\
         # The cache decision\n\
         \n\
         ## Context\n\
         Forces.\n\
         \n\
         ## Decision\n\
         Decided.\n\
         \n\
         ## Consequences\n\
         Effects.\n",
    )
    .expect("commit the adr at the shadowed location");

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a content-only sweep (an un-baselined doc) must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("file-state.un-baselined") && stdout.contains("docs/adrs/cache.md"),
        "the store sweep must resolve the `adr` schema through the cascade and walk the \
         project-shadowed `adrs/` location, surfacing the un-baselined doc there; a pack-only \
         sweep walks `decisions/` and misses it; stdout:\n{stdout}",
    );
}

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the listed,
/// highest-precedence pack the two-pack `[dev ▸ methodology]` store composes.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Record the listed pack-set in the in-repo project layer's `packs.yaml` — the
/// pre-cascade selector `make_pack()` CWD-discovers (the `multi_pack_acceptance.rs`
/// idiom). The named directory sits at highest precedence; the embedded dev pack is
/// the implicit base below it.
fn write_packs_yaml(repo: &Path, listed: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.display()),
    )
    .expect("write packs.yaml naming the listed pack");
}

/// (T2b) The store-sweep `workflow↔refs` family must resolve each workflow's
/// command-refs **pack-locally**, against the workflow's own origin pack's catalog —
/// not against one flat precedence-winner catalog (`design/multi-pack.md` → Pack-local
/// body-reference resolution: "`command-ref-resolves` … fire **per-definition against
/// that definition's own pack**").
///
/// Compose the two-pack `[dev ▸ methodology]` store via `packs.yaml` listing the
/// on-disk methodology pack over the embedded dev base. methodology's workflows
/// (`planning` / `completion` / …) include steps whose bodies carry `{{ cli.create-*
/// }}` command-refs (`create-roadmap`, `create-ledger`, `create-log`,
/// `create-completion-record`, `create-dogfood-record`) — defined in **methodology's**
/// `commands.yaml`, ABSENT from dev's catalog (the precedence winner).
///
/// Under a flat-catalog sweep, every one of methodology's workflows resolves its
/// command-refs against dev's catalog and emits a FALSE blocking
/// `workflow-refs.command-ref-resolves` finding for each methodology-only command — the
/// M14 settled-rule violation this task fixes. A pack-local sweep resolves each
/// workflow's refs against ITS OWN pack's catalog, so methodology's command-refs are
/// found and no such finding surfaces. Content-only (no probe-integrity meta-finding),
/// so the run exits 0 either way — the assertion is on the emitted findings.
#[test]
fn validate_store_sweep_resolves_command_refs_pack_locally_over_two_packs() {
    let repo = TempDir::new("pack-local-refs");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a content-only two-pack sweep must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    // No command-ref finding at all — every methodology command-ref resolves against
    // methodology's own catalog, every dev command-ref against dev's.
    assert!(
        !stdout.contains("workflow-refs.command-ref-resolves"),
        "the two-pack store sweep must resolve each workflow's command-refs pack-locally \
         (against its own origin pack's catalog), so methodology's `create-*` refs raise NO \
         false `command-ref-resolves` finding; stdout:\n{stdout}",
    );
    // Belt-and-braces: name each methodology-only command — none may appear in a finding.
    for cmd in [
        "create-roadmap",
        "create-completion-record",
        "create-log",
        "create-ledger",
        "create-dogfood-record",
    ] {
        assert!(
            !stdout.contains(&format!("cli.{cmd}")),
            "methodology's `{cmd}` command-ref must resolve against methodology's catalog, \
             not surface as drift; stdout:\n{stdout}",
        );
    }
}
