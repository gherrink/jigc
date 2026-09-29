//! M20 inc-2 / T4 — the **headline acceptance** for the completed `jigc validate`
//! envelope, exercised through the **real `jigc` binary** (the M10 invocation-path-
//! masking lesson: drive the bytes an operator would actually run, never a
//! reconstructed equivalent). Composes T1 (the workflow↔refs store target), T2 (the
//! read-only file↔CLI-state twin), and T3 (the three-family `validate_store` reshape).
//!
//! Built against `implementation/roadmap.md` → M20 Increment 2 Deliverable + Proves and
//! `design/validation.md` → Completing the envelope. The store is set up via `jigc setup`
//! over a real `git init` temp repo, and the real `doc-code` probe resolves through the
//! production path (the `JIGC_DOC_CODE_PROBE` override removed) so the probe pre-flight passes.
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
//! is a real `git init`, the probe is the real `doc-code`, and a self-cleaning
//! `TempDir` keeps the test off the developer's repo.
//!
//! **M47 inc-7 / T2 — `blocking_probes`.** The store envelope gains a third top-level key
//! beside `scope`/`report_only`: the sorted, de-duplicated probe names of the findings whose
//! **resolved severity is blocking**, `[]` when none, so a shell consumer can key on severity
//! without reaching into a finding object (`design/command-output-contract.md` → The store
//! sweep's envelope; DECISIONS.md → 2026-07-26 M47, Decision 7 re-settled). Three arms, over
//! real committed stores through the real binary:
//!
//! - **(v) blocking → present, sorted, de-duplicated.** An arch-doc whose **two** components
//!   anchor symbols that are then renamed away yields two blocking `doc-code.symbol-exists`
//!   findings; a dangling workflow shadow adds a blocking `workflow-refs.include-resolves` →
//!   `blocking_probes == ["doc-code", "workflow-refs"]` (one entry per probe, sorted).
//! - **(vi) advisory does NOT promote.** An arch-doc component whose anchor **resolves**
//!   while its title names a different compound identifier raises the *advisory*
//!   `doc-code.title-names-symbol`; with an unrelated blocking finding present,
//!   `blocking_probes` is non-empty and **omits** `"doc-code"` — the false positive the
//!   severity filter exists to prevent.
//! - **(vii) no blocking finding → the empty array.** The same advisory-only store without
//!   the unrelated blocking finding yields `blocking_probes == []` — present and empty, never
//!   absent.

use crate::support::run_then_parse::stdout_json;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-validate-envelope-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the `JIGC_DOC_CODE_PROBE`
/// override removed, so the real probe resolves through the production path.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>` against an on-disk pack tree (`JIGC_PACK_DIR`), otherwise identical
/// to [`jigc`].
fn jigc_with_pack(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
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
        .env_remove("JIGC_DOC_CODE_PROBE");
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
/// project layer). Returns once the store is a clean, set-up repo.
fn setup_repo(repo: &Path, home: &Path) {
    init_repo(repo);
    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// [`setup_repo`], installing over an **on-disk pack tree** selected by `JIGC_PACK_DIR`
/// — the seam the freeze-exempt cascade arm drives (`support::frozen_pack`).
fn setup_repo_with_pack(repo: &Path, home: &Path, pack: &Path) {
    init_repo(repo);
    let out = jigc_with_pack(repo, home, pack, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// A real git repo with identity and one commit — the shared preamble of both setups.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
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
    let task = "cache-sessions-in-a-single";

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
/// **The pack under test ships no freeze manifest** (M49 Increment 3). A `location:` is
/// inside the `schema-hash` since M38, so relocating a *manifest-governed* doctype from
/// the project layer is now refused at pack-load — the freeze binds at every layer that
/// can change a schema, and this arm used to relocate a frozen `adr` in silence, which
/// was the hole rather than the capability. What survives, and is what this guards, is
/// the cascade itself: a pack that declares nothing frozen freezes nothing at either
/// layer, so its doctype's home is the project's to move (`support::frozen_pack` →
/// [`manifest_less_dev_pack`](crate::support::frozen_pack::manifest_less_dev_pack)).
///
/// Seed a clean store over that pack, then drop a project `schemas/adr.yaml` whole-file
/// shadow that relocates `adr` from the pack's `decisions/` to `adrs/`. The cascade
/// auto-shadows the id (`file_owner(adr) == Project`), so the resolved `adr` schema's
/// `location:` is now `adrs/`. Commit an `adr` doc at the **project-shadowed** location
/// `adrs/cache.md` with no baseline record. The file↔CLI-state twin enumerates committed
/// docs by walking each resolved schema's `location:`, so a cascade-aware sweep walks
/// `adrs/`, finds the un-baselined doc, and surfaces a `file-state.un-baselined` advisory
/// naming it. A **pack-only** sweep walks the pack's `decisions/` instead, never sees
/// `adrs/cache.md`, and emits no such finding — the regression this guards. Content-only,
/// and the manifest-less pack declares no version to be stale against, so exit 0.
#[test]
fn validate_honors_project_schema_location_shadow_in_store_sweep() {
    let repo = TempDir::new("schema-shadow");
    let home = TempDir::new("home");
    let pack = TempDir::new("schema-shadow-pack");
    crate::support::frozen_pack::manifest_less_dev_pack(pack.path());
    setup_repo_with_pack(repo.path(), home.path(), pack.path());

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

    let out = jigc_with_pack(repo.path(), home.path(), pack.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // Content-only and freeze-exempt: the manifest-less pack declares no `schema-version`
    // for the fixture to be stale against, so the M42 unmigrated-corpus exit flip
    // (`design/validation.md` → Exit semantics) never fires and the sweep exits 0.
    assert!(
        out.status.success(),
        "a content-only sweep over a freeze-exempt pack exits 0; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("file-state.un-baselined") && stdout.contains("docs/adrs/cache.md"),
        "the store sweep must resolve the `adr` schema through the cascade and walk the \
         project-shadowed `adrs/` location, surfacing the un-baselined doc there; a pack-only \
         sweep walks `decisions/` and misses it; stdout:\n{stdout}",
    );
}

/// The tracked Rust source every `implemented-by` anchor in the `blocking_probes` arms
/// resolves against — two present symbols, so one arch-doc can anchor two components at
/// real code (the `doc-code` probe's Rust grammar).
const CODE_WITH_SYMBOLS: &str = "\
pub fn session_store() -> u32 {
    1
}

pub fn cache_warmer() -> u32 {
    2
}
";

/// The same file after **both** symbols are renamed away — every committed anchor into it
/// now dangles, so the store sweep raises one blocking `doc-code.symbol-exists` per anchor.
const CODE_WITH_SYMBOLS_RENAMED: &str = "\
pub fn store_of_sessions() -> u32 {
    1
}

pub fn warmer_of_caches() -> u32 {
    2
}
";

/// Write + commit `src/lib.rs` — the tracked code the arch-doc components anchor into.
fn commit_code(repo: &Path, body: &str, message: &str) {
    fs::create_dir_all(repo.join("src")).expect("create src dir");
    fs::write(repo.join("src").join("lib.rs"), body).expect("write src/lib.rs");
    git(repo, &["add", "src/lib.rs"]);
    git(repo, &["commit", "-q", "-m", message]);
}

/// Author + finalize one `arch-doc` through the real binary, with one `components` item per
/// `(title, symbol)` pair anchored at `src/lib.rs#<symbol>` — the committed, code-anchored
/// doc the `doc-code` store family adjudicates. Every `add-item` address is captured from
/// stdout and run verbatim (the emitted bytes are the contract).
fn commit_arch_doc(repo: &Path, home: &Path, components: &[(&str, &str)]) {
    let task = "document-the-cache-layer";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "architecture-documentation",
            "document the cache layer",
        ],
    );
    assert_ok(&out, "`jigc start --workflow architecture-documentation`");

    let create = jigc_doc(
        repo,
        home,
        &["create", "arch-doc", "--title", "Cache layer"],
        None,
    );
    assert_ok(&create, "`jigc doc create arch-doc`");
    let arch = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };

    set_slot(
        &format!("{arch}#overview"),
        b"The cache layer owns ephemeral session state.\n",
    );

    for (title, symbol) in components {
        let added = jigc_doc(
            repo,
            home,
            &["add-item", &format!("{arch}#components"), "--title", title],
            None,
        );
        assert_ok(
            &added,
            &format!("`jigc doc add-item …#components` ({title})"),
        );
        let item = String::from_utf8(added.stdout)
            .expect("utf-8")
            .trim_end_matches('\n')
            .to_owned();
        set_slot(
            &format!("{item}/description"),
            b"One part of the cache layer.\n",
        );
        set_field(
            &format!("{item}/implemented-by"),
            &format!("src/lib.rs#{symbol}"),
        );
    }

    fill_commit(repo, home, task);
    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize` (arch-doc)");
}

/// The store envelope `jigc validate --format json` emits, parsed — the machine surface a
/// driver (and the installed pre-commit hook) consumes.
fn store_envelope(repo: &Path, home: &Path) -> serde_json::Value {
    let out = jigc(repo, home, &["validate", "--format", "json"]);
    stdout_json(&out, &[0, 1], "`jigc validate --format json`")
}

/// The envelope's `blocking_probes` as a plain string vector — asserted to be an array of
/// plain strings (the flat, `]`-delimited shape the hook's bounded match relies on).
fn blocking_probes(envelope: &serde_json::Value) -> Vec<String> {
    envelope["blocking_probes"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("the store envelope carries a `blocking_probes` array; json:\n{envelope}")
        })
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| {
                    panic!("`blocking_probes` holds plain probe-name strings; json:\n{envelope}")
                })
                .to_string()
        })
        .collect()
}

/// Every finding code in the envelope's `findings` array, in emission order.
fn finding_codes(envelope: &serde_json::Value) -> Vec<String> {
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("the store envelope carries a `findings` array; json:\n{envelope}")
        })
        .iter()
        .map(|f| {
            f["code"]
                .as_str()
                .expect("a finding carries a code")
                .to_string()
        })
        .collect()
}

/// The two unchanged top-level keys, asserted on every `blocking_probes` arm: the new key is
/// **additive** — it must not disturb `scope`, `report_only`, or the findings list.
fn assert_store_envelope_unchanged(envelope: &serde_json::Value) {
    assert_eq!(envelope["scope"], "store", "json:\n{envelope}");
    assert_eq!(
        envelope["report_only"],
        serde_json::Value::Bool(true),
        "a content-only sweep stays report-only; json:\n{envelope}",
    );
    assert!(
        !finding_codes(envelope).is_empty(),
        "each arm's store carries findings; json:\n{envelope}",
    );
}

/// (v) A **blocking** `doc-code` finding lands in `blocking_probes` — sorted and
/// de-duplicated. Two dangling anchors (two blocking `doc-code.symbol-exists` findings)
/// collapse to ONE `"doc-code"` entry, and the unrelated blocking
/// `workflow-refs.include-resolves` sorts after it.
#[test]
fn blocking_probes_carries_each_blocking_probe_once_sorted() {
    let repo = TempDir::new("blocking-probes");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_code(repo.path(), CODE_WITH_SYMBOLS, "the anchored source");

    // Two components, each anchored at a present symbol; descriptive titles carry no
    // compound identifier, so the advisory `title-names-symbol` guard stays silent here.
    commit_arch_doc(
        repo.path(),
        home.path(),
        &[
            ("Session store", "session_store"),
            ("Cache warmer", "cache_warmer"),
        ],
    );

    // Both symbols renamed away — the two committed anchors now dangle.
    commit_code(
        repo.path(),
        CODE_WITH_SYMBOLS_RENAMED,
        "rename both anchored symbols",
    );
    // A second, unrelated blocking family, so the array's ORDER is a real claim.
    seed_dangling_workflow_ref(repo.path());

    let envelope = store_envelope(repo.path(), home.path());
    assert_store_envelope_unchanged(&envelope);

    let codes = finding_codes(&envelope);
    assert_eq!(
        codes
            .iter()
            .filter(|c| *c == "doc-code.symbol-exists")
            .count(),
        2,
        "precondition: BOTH dangling anchors raise their own blocking finding; json:\n{envelope}",
    );
    assert_eq!(
        blocking_probes(&envelope),
        vec!["doc-code".to_string(), "workflow-refs".to_string()],
        "two blocking findings of one probe yield ONE entry, and the entries are sorted; \
         json:\n{envelope}",
    );
}

/// (vi) An **advisory** `doc-code` finding does NOT promote its probe into
/// `blocking_probes`. The arch-doc component's anchor resolves (no `symbol-exists`), while
/// its title names a different compound identifier → the advisory
/// `doc-code.title-names-symbol`. With an unrelated blocking finding present the array is
/// non-empty and still omits `"doc-code"` — a consumer keying on the array reads severity,
/// not mere presence.
#[test]
fn blocking_probes_omits_a_probe_whose_only_finding_is_advisory() {
    let repo = TempDir::new("advisory-only");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_code(repo.path(), CODE_WITH_SYMBOLS, "the anchored source");

    // `sessionStore` is a compound identifier that is not the anchored `session_store` —
    // the stale-heading guard, advisory by default — while the anchor itself resolves.
    commit_arch_doc(
        repo.path(),
        home.path(),
        &[("sessionStore", "session_store")],
    );
    seed_dangling_workflow_ref(repo.path());

    let envelope = store_envelope(repo.path(), home.path());
    assert_store_envelope_unchanged(&envelope);

    let codes = finding_codes(&envelope);
    assert!(
        codes.iter().any(|c| c == "doc-code.title-names-symbol"),
        "precondition: the advisory stale-heading finding is IN `findings`; json:\n{envelope}",
    );
    assert!(
        !codes.iter().any(|c| c == "doc-code.symbol-exists"),
        "precondition: the anchor resolves — no blocking doc-code finding; json:\n{envelope}",
    );

    let probes = blocking_probes(&envelope);
    assert!(
        !probes.is_empty(),
        "the unrelated blocking finding keeps the array non-empty, so an omission is a \
         severity claim rather than an empty sweep; json:\n{envelope}",
    );
    assert!(
        !probes.contains(&"doc-code".to_string()),
        "an advisory-only `doc-code` must NOT appear in `blocking_probes`; json:\n{envelope}",
    );
}

/// (vii) No blocking finding at all → `blocking_probes` is the **empty array**: present and
/// empty, never absent. Same advisory-only store as (vi), minus the unrelated blocking
/// finding.
#[test]
fn blocking_probes_is_the_empty_array_when_nothing_blocks() {
    let repo = TempDir::new("nothing-blocks");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_code(repo.path(), CODE_WITH_SYMBOLS, "the anchored source");
    commit_arch_doc(
        repo.path(),
        home.path(),
        &[("sessionStore", "session_store")],
    );

    let envelope = store_envelope(repo.path(), home.path());
    assert_store_envelope_unchanged(&envelope);

    assert!(
        finding_codes(&envelope)
            .iter()
            .any(|c| c == "doc-code.title-names-symbol"),
        "precondition: the advisory finding is present — the array is empty on severity, \
         not on an empty report; json:\n{envelope}",
    );
    assert_eq!(
        blocking_probes(&envelope),
        Vec::<String>::new(),
        "no blocking finding yields the empty array; json:\n{envelope}",
    );
}

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`) — the listed,
/// highest-precedence pack the two-pack `[dev ▸ methodology]` store composes.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// A conformant, **current** (`schema-version: 2`) `changelog` at its literal root
/// `CHANGELOG.md` **placement** home — the committed managed doc the OOB edit below drifts.
const ROOT_CHANGELOG: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

## Releases
";

/// The same doc after a **human hand-edit through git** — a *conformant* change (a populated
/// `added` change-group), so **no other family** can catch it: the file↔CLI-state twin is the
/// only detector that ever sees this edit.
const ROOT_CHANGELOG_HAND_EDITED: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### added  {#added}

- A human hand-edited this entry in, outside the CLI.

## Releases
";

/// **M42 Inc-2 T2 — the read-only file↔CLI-state twin gets its placement branch.**
///
/// `detect_committed_store` walked only `location:`-bearing schemas (the stale guard *"a
/// transient (location-less) type has no committed docs"*, true pre-M38 and false since),
/// so the whole **placement** class was invisible to `jigc validate`: an out-of-band edit to
/// a baselined root `CHANGELOG.md` (or `VISION.md`) reported *"no findings — the committed
/// store validates clean"*, exit 0 — a **false green over a tampered managed doc**, voiding
/// [CLAUDE.md]'s *"out-of-band edits are detected and routed"* invariant for that class while
/// the *mutating* twin (`reconcile_committed_store`) had carried its branch since M38.
///
/// Drives the shipped binary end-to-end on the real dev-pack `changelog` doctype: a committed
/// `CHANGELOG.md` at its literal placement home, **baselined by a real landed finalize** (the
/// preflight's committed-store sweep baseline-adopts it; the landed finalize persists the
/// record), then hand-edited through git → `jigc validate` reports a `file-state.hash-matches`
/// drift naming `CHANGELOG.md`, report-only (exit 0).
#[test]
fn validate_detects_an_oob_edit_to_a_committed_placement_doc() {
    let repo = TempDir::new("placement-oob");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // A committed `changelog` at its literal root placement home.
    fs::write(repo.path().join("CHANGELOG.md"), ROOT_CHANGELOG).expect("write CHANGELOG.md");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "seed the root changelog"],
    );

    // A real landed finalize: its committed-store sweep baseline-adopts the pre-existing
    // `CHANGELOG.md` and the landed finalize persists the record.
    commit_baselined_adr(repo.path(), home.path());
    let record = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("state")
            .join("file-state.json"),
    )
    .expect("the landed finalize persisted the file-state record");
    assert!(
        record.contains("CHANGELOG.md"),
        "precondition: the committed placement doc is baselined at its literal home; \
         record:\n{record}",
    );

    // The out-of-band edit: a human rewrites the committed `CHANGELOG.md` through git,
    // conformantly — outside the CLI.
    fs::write(repo.path().join("CHANGELOG.md"), ROOT_CHANGELOG_HAND_EDITED)
        .expect("apply the out-of-band edit");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "human edits CHANGELOG.md out of band"],
    );

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a content-only sweep stays report-only (exit 0); stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("file-state.hash-matches") && stdout.contains("CHANGELOG.md"),
        "the OOB edit to the committed placement doc must surface a file-state.hash-matches \
         drift naming its literal home — not `no findings`; stdout:\n{stdout}",
    );
}

// ---------------------------------------------------------------------------
// M47 inc-8 / T4 — the store trailer classifies by **severity**, not probe family,
// and names the **milestone** door beside the two task doors (N10, both arms;
// `design/validation.md` → The trailer must not claim a gate that does not exist).
//
// Three arms over real committed stores through the real binary:
//
// - **(a) advisory-only → no gate claim.** A store whose sole finding is the
//   advisory-by-default `doc-code.title-names-symbol` printed the row `advisory · …`
//   and then *"these gate at `jigc task validate` / `jigc task finalize`"* — the row and
//   the trailer disagreeing in the same report, because the trailer's count consulted
//   `GATES_NOWHERE` and the un-baselined discriminator but **never severity**.
// - **(b) advisory + blocking sibling → the mixed sentence counts 1, not 2**, and its
//   door list names `jigc milestone finalize` (the milestone-boundary gate drives the
//   same shared `validate_task` entry and blocks exit 3).
// - **(c) the compose-scoped family is scoped OUT of the boundary claim.**
//   `workflow-refs.*` is emitted by the store sweep's family 2 and by compose
//   (`crate::start`) — and by **no** task-scope path: `engine::validate::validate_task`
//   (the shared body of `jigc task validate`, finalize's preflight, and the
//   milestone-boundary gate) never runs the workflow↔refs family. So the boundary
//   sentence must not claim it; the trailer names the door it really has.
// ---------------------------------------------------------------------------

/// `src/lib.rs` after **one** of the two anchored symbols is renamed away: `session_store`
/// still resolves (so its component keeps the *advisory* stale-heading finding and raises no
/// `symbol-exists`), while `cache_warmer` is gone (one **blocking** `doc-code.symbol-exists`).
/// The mixed-severity store the trailer's count has to discriminate.
const CODE_WITH_ONE_SYMBOL_RENAMED: &str = "\
pub fn session_store() -> u32 {
    1
}

pub fn warmer_of_caches() -> u32 {
    2
}
";

/// The store sweep's human/agent text — the emitted bytes an operator reads, run verbatim.
fn validate_text(repo: &Path, home: &Path) -> (bool, String) {
    let out = jigc(repo, home, &["validate"]);
    (
        out.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
    )
}

/// Every door the trailer may name — asserted absent as a set, so a claim can never be
/// withdrawn from one door and left standing on another.
fn assert_names_no_gate(text: &str, why: &str) {
    for door in [
        "jigc task validate",
        "jigc task finalize",
        "jigc milestone finalize",
    ] {
        assert!(
            !text.contains(door),
            "{why} — the trailer must not name `{door}`; got:\n{text}",
        );
    }
}

/// (a) **Advisory-only → no gate claim.** The store's sole finding is the
/// advisory-by-default `doc-code.title-names-symbol` (`pack/config/knobs.yaml` — the
/// brand-name false-positive class has no valid remedy under blocking). Nothing anywhere
/// stops on an advisory, so the trailer must make no gate claim at all — the "gates nowhere"
/// sentence — rather than sending the reader to a gate that will report clean at exit 0.
#[test]
fn the_trailer_claims_no_gate_when_every_finding_is_advisory() {
    let repo = TempDir::new("advisory-only-trailer");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_code(repo.path(), CODE_WITH_SYMBOLS, "the anchored source");
    // The anchor RESOLVES; only the heading names a different compound identifier.
    commit_arch_doc(
        repo.path(),
        home.path(),
        &[("sessionStore", "session_store")],
    );

    let envelope = store_envelope(repo.path(), home.path());
    assert_eq!(
        finding_codes(&envelope),
        vec!["doc-code.title-names-symbol".to_string()],
        "precondition: the advisory stale-heading finding is the ONLY finding; json:\n{envelope}",
    );
    assert_eq!(
        envelope["findings"][0]["severity"], "advisory",
        "precondition: and it is advisory by pack default; json:\n{envelope}",
    );

    let (ok, text) = validate_text(repo.path(), home.path());
    assert!(ok, "a content-only sweep stays exit 0; got:\n{text}");
    assert!(
        text.contains("advisory · doc-code.title-names-symbol"),
        "precondition: the row prints the bare advisory token; got:\n{text}",
    );
    assert_names_no_gate(
        &text,
        "an advisory gates nowhere — `jigc task validate` reports it clean at exit 0",
    );
    assert!(
        text.contains("gates nowhere"),
        "and the trailer states what is actually true of it; got:\n{text}",
    );
}

/// (b) **The mixed sentence counts by severity, and names the milestone door.** Two
/// findings over one committed arch-doc: the *advisory* stale heading (anchor resolves) and
/// a *blocking* `doc-code.symbol-exists` (its sibling component's symbol renamed away).
/// Exactly **one** of the two gates, and the sentence names all three doors the shared
/// `engine::validate::validate_task` entry stands behind — `jigc milestone finalize` drives
/// it over the merged effective state and blocks exit 3 on the same families.
#[test]
fn the_mixed_trailer_counts_by_severity_and_names_the_milestone_door() {
    let repo = TempDir::new("mixed-trailer");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_code(repo.path(), CODE_WITH_SYMBOLS, "the anchored source");
    commit_arch_doc(
        repo.path(),
        home.path(),
        &[
            ("sessionStore", "session_store"),
            ("Cache warmer", "cache_warmer"),
        ],
    );
    commit_code(
        repo.path(),
        CODE_WITH_ONE_SYMBOL_RENAMED,
        "rename ONE anchored symbol away",
    );

    let envelope = store_envelope(repo.path(), home.path());
    let codes = finding_codes(&envelope);
    assert_eq!(
        codes.len(),
        2,
        "precondition: exactly two findings — one advisory, one blocking; json:\n{envelope}",
    );
    assert!(
        codes.iter().any(|c| c == "doc-code.title-names-symbol")
            && codes.iter().any(|c| c == "doc-code.symbol-exists"),
        "precondition: the advisory stale heading AND its blocking dangling-anchor sibling; \
         json:\n{envelope}",
    );

    let (ok, text) = validate_text(repo.path(), home.path());
    assert!(ok, "a content-only sweep stays exit 0; got:\n{text}");
    assert!(
        text.contains(
            "1 of them gate at `jigc task validate` / `jigc task finalize` / \
             `jigc milestone finalize`"
        ),
        "one of the two findings gates — the count is by severity, not by probe family, and \
         the door list names the milestone boundary; got:\n{text}",
    );
    assert!(
        !text.contains("2 of them gate"),
        "the advisory must not be counted into the gating set; got:\n{text}",
    );
    assert!(
        text.contains("the rest are store-scope advisories that gate nowhere"),
        "and the remainder is named for what it is; got:\n{text}",
    );
    // The per-finding label and the trailer are one criterion: only the blocking sibling
    // carries the label.
    assert!(
        text.contains("blocking (gates at finalize) · doc-code.symbol-exists"),
        "the gating row keeps its label; got:\n{text}",
    );
    assert!(
        text.contains("advisory · doc-code.title-names-symbol"),
        "the advisory row claims nothing; got:\n{text}",
    );
}

/// (c) **The compose-scoped family is scoped out of the boundary claim.** A project
/// workflow shadow with a dangling `{{ include: step:not-a-step }}` raises the **blocking**
/// `workflow-refs.include-resolves` at store scope — and `engine::validate::validate_task`,
/// the shared body of `jigc task validate`, the finalize preflight, and the
/// milestone-boundary gate, never runs the workflow↔refs family at all: its emit sites are
/// the store sweep (`validate_store_families`, family 2) and **compose** (`crate::start`).
/// So the boundary sentence must not claim it — the trailer names the door it really has,
/// and the row carries no `(gates at finalize)` label.
#[test]
fn a_workflow_ref_break_is_claimed_at_compose_never_at_the_boundary() {
    let repo = TempDir::new("compose-scoped-trailer");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    seed_dangling_workflow_ref(repo.path());

    let envelope = store_envelope(repo.path(), home.path());
    let codes = finding_codes(&envelope);
    assert!(
        !codes.is_empty() && codes.iter().all(|c| c.starts_with("workflow-refs.")),
        "precondition: every finding is a workflow↔refs break; json:\n{envelope}",
    );

    let (ok, text) = validate_text(repo.path(), home.path());
    assert!(ok, "a content-only sweep stays exit 0; got:\n{text}");
    assert_names_no_gate(
        &text,
        "no task-scope path emits `workflow-refs.*` — neither task door nor the milestone \
         gate can ever see it",
    );
    assert!(
        text.contains("compose (`jigc start`)"),
        "the trailer names the door the finding really has; got:\n{text}",
    );
    assert!(
        !text.contains("(gates at finalize)"),
        "and the row must not claim a finalize block either — the label and the trailer are \
         one criterion; got:\n{text}",
    );
}

// ───── The young-corpus exemption — `changelog#releases` (M47 inc-10 T9 · N18) ─────

/// A **young** changelog: unreleased changes staged, **no release cut yet**. This is the
/// correct steady state of every project between its first change and its first release
/// — not a hollow adoption — and it is what the greenfield trial's corpus looked like
/// when `schema-conformance.repeatable-populated` cried wolf over the empty `## Releases`.
const YOUNG_CHANGELOG: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### added  {#added}

- The first change, staged and not yet cut.

## Releases
";

/// A committed `spec` whose required repeatable `criteria` section holds **zero** items —
/// the exemption's control. `spec#criteria` carries no exemption token, so the advisory
/// must still fire here: the fix is one token, never a disarmed check.
const HOLLOW_SPEC: &str = "\
---
schema-version: 1
---

# Rate limit

## Goal

Bound per-client request volume.

## Context

Downstream services enforced limits ad hoc.

## Criteria
";

/// Commit `body` at `path` (relative to the repo root), creating parent dirs.
fn commit_file(repo: &Path, path: &str, body: &str, message: &str) {
    let target = repo.join(path);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(&target, body).expect("write the committed doc");
    git(repo, &["add", path]);
    git(repo, &["commit", "-q", "-m", message]);
}

/// Every `repeatable-populated` finding's `key.target` in the store envelope.
fn hollow_targets(envelope: &serde_json::Value) -> Vec<String> {
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the store envelope carries a `findings` array; {envelope}"))
        .iter()
        .filter(|f| f["code"] == "schema-conformance.repeatable-populated")
        .map(|f| {
            f["key"]["target"]
                .as_str()
                .expect("a finding key carries a string target")
                .to_string()
        })
        .collect()
}

/// **(N18) A young changelog is not a hollow one.** `## Releases` parses zero items until
/// the project cuts its first release, so `repeatable-populated` fired on every greenfield
/// corpus — an advisory with no action behind it, which is what teaches a reader to ignore
/// the advisory channel. `changelog#releases` joins the pack-default
/// `validation.schema-conformance.repeatable-populated.exempt` token list (the "zero items
/// IS a valid state" rung), so the sweep stays silent about it.
#[test]
fn a_young_changelog_raises_no_hollow_repeatable_advisory() {
    let repo = TempDir::new("young-changelog");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_file(
        repo.path(),
        "CHANGELOG.md",
        YOUNG_CHANGELOG,
        "seed a young changelog",
    );

    let envelope = store_envelope(repo.path(), home.path());
    assert!(
        hollow_targets(&envelope).is_empty(),
        "a young changelog (unreleased changes staged, no release cut) is a correct state — \
         the sweep must raise NO `repeatable-populated` advisory over it; json:\n{envelope}",
    );
}

/// **(N18, the control)** The same sweep over a doctype the token list does **not** name:
/// a committed spec with zero `criteria` still raises the advisory, at its own
/// `<type>:<slug>#<section>` target. The exemption is one token wide, not a disarmed check.
#[test]
fn a_hollow_unexempted_repeatable_still_raises_the_advisory() {
    let repo = TempDir::new("hollow-spec");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_file(
        repo.path(),
        "docs/specs/rate-limit.md",
        HOLLOW_SPEC,
        "seed a hollow spec",
    );

    let envelope = store_envelope(repo.path(), home.path());
    assert_eq!(
        hollow_targets(&envelope),
        vec!["spec:rate-limit#criteria".to_string()],
        "an un-exempted zero-item repeatable must still surface the advisory; json:\n{envelope}",
    );
}
