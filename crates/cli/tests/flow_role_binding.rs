//! M45 Increment 5 / T2 — **copy-on-write binds the role**, and
//! **`create.serial-collision` over a same-identity staged copy acks `existed` and
//! binds** — swept over the class (`DECISIONS.md` 2026-07-23 M45 planning → Fork 2;
//! `design/write-commands.md` → The create-gate; `design/design-altitude-doctypes.md`
//! §7). Everything is asserted on the EMITTED bytes of the real `jigc` binary
//! (`CARGO_BIN_EXE_jigc`) over the `[dev ▸ methodology]` composition.
//!
//! The class is **object-form `{type, as:}` create-gate roles** with a role-dependent
//! author surface, and it has TWO symptom shapes when the doc is reached by a
//! copy-on-write (the *revise* path: edit a committed doc before any `doc create`)
//! rather than by an explicit `doc create`:
//!
//!  1. **silent-empty `@`-slice** — `form-vision`'s `{{ @task.vision.grounded-in#findings }}`
//!     resolves EMPTY because `task.vision` is unbound, even after `grounded-in` is set on
//!     the copied-in vision (`vision_set_field_first_binds_role_and_renders_grounding`).
//!  2. **slug-less `<<author:>>` address** — `plan`'s `<<author: {{ task.spec#goal }}>>`
//!     renders the slug-less pending form `<<author: spec#goal>>` instead of the real
//!     `<<author: spec:<slug>#goal>>` (`spec_copy_on_write_binds_role_renders_author_address`).
//!
//! Both are cured by binding the role when an edit verb copy-on-writes a committed doc of
//! the gate's doctype (`read_or_copy_in` → `bind_role_on_copy_in`, bind-**if-unbound**), and
//! by converting the gated same-identity-staged `create.serial-collision` into an
//! ack-`existed`-and-bind (`state::create_gated`). The sweep
//! (`every_object_form_pair_acks_existed_on_recreate`) ITERATES the composite registry's
//! object-form pairs — no hand list — so a NEW pair is covered by construction.
//!
//! No external test crates: the `jigc` path is `CARGO_BIN_EXE_jigc`, the temp repo is a
//! real `git init`, and self-cleaning `TempDir`s keep the developer's repo clean.

use cli::pack::{CompositePack, EmbeddedPack};
use engine::compose::load_workflow_def;
use engine::packsource::{PackResourceKind, PackSource};
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
            "jigc-role-binding-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The production composition, built the CWD-free way: `[dev ▸ methodology]`.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every **object-form** `{type, as:}` create-gate role, enumerated from the composite
/// registry (no hand list): `(workflow_id, doctype, role)`. A bare-form gate entry (empty
/// `as:`) declares no role and is skipped. This is the sweep's members-by-construction seam.
fn object_form_pairs() -> Vec<(String, String, String)> {
    let pack = composite();
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = pack
            .read(PackResourceKind::Workflows, &id)
            .expect("workflow reads back");
        let def = load_workflow_def(&bytes).expect("workflow front-matter parses");
        for entry in &def.allows_create {
            if !entry.as_role.is_empty() {
                out.push((
                    id.to_string(),
                    entry.doc_type.clone(),
                    entry.as_role.clone(),
                ));
            }
        }
    }
    out
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// then record the methodology pack in `packs.yaml` (the `[dev ▸ methodology]` composition).
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

/// `jigc start --workflow <w> "<intent>"`, returning the minted task id (the
/// `task minted: <id>` line).
fn start(repo: &Path, home: &Path, workflow: &str, intent: &str) -> String {
    let out = jigc(repo, home, &["start", "--workflow", workflow, intent], None);
    assert_ok(&out, &format!("`jigc start --workflow {workflow}`"));
    let stdout = String::from_utf8(out.stdout).expect("utf-8");
    stdout
        .lines()
        .find_map(|l| l.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("start must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, task: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, task: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value, "--task", task],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the provisioned commit doc so a finalize validates clean.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), task, "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), task, "role");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        task,
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        task,
        b"A role-binding record.\n",
    );
}

/// Commit one grounding `research` doc through the real `do-research` workflow, so it is a
/// reachable `grounded-in` target. Returns the committed `research:<slug>` address.
fn commit_research(repo: &Path, home: &Path, intent: &str, title: &str, findings: &[u8]) -> String {
    let task = start(repo, home, "do-research", intent);
    let create = jigc(
        repo,
        home,
        &[
            "doc", "create", "research", "--title", title, "--task", &task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create research`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    set_slot(
        repo,
        home,
        &format!("{addr}#question"),
        &task,
        b"A question.\n",
    );
    set_slot(repo, home, &format!("{addr}#findings"), &task, findings);
    set_slot(
        repo,
        home,
        &format!("{addr}#sources"),
        &task,
        b"Some sources.\n",
    );
    fill_commit(repo, home, &task);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (do-research) — the committed grounding target",
    );
    addr
}

/// Commit the `vision` singleton through the real `form-vision` create-first path, so a
/// later `form-vision` task revises a COMMITTED VISION.md. Grounded in nothing (a valid
/// empty `0..*` anchor), so no research is needed for the precondition.
fn commit_vision(repo: &Path, home: &Path) {
    let task = start(repo, home, "form-vision", "form the project vision");
    let create = jigc(
        repo,
        home,
        &[
            "doc", "create", "vision", "--title", "Vision", "--task", &task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create vision`");
    set_slot(
        repo,
        home,
        "vision:vision#thesis",
        &task,
        b"A context compiler.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#invariants",
        &task,
        b"The CLI owns structure.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#open-questions",
        &task,
        b"What earns a public pack platform?\n",
    );
    fill_commit(repo, home, &task);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (form-vision) — the committed VISION.md",
    );
}

/// Commit a criteria-less-but-valid `spec` through the real `plan` create-first path, so a
/// later `plan` task revises a COMMITTED `specs/<slug>.md`. Returns `spec:<slug>`.
fn commit_spec(repo: &Path, home: &Path, title: &str) -> String {
    let task = start(repo, home, "plan", "draft the spec");
    let create = jigc(
        repo,
        home,
        &["doc", "create", "spec", "--title", title, "--task", &task],
        None,
    );
    assert_ok(&create, "`jigc doc create spec`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    set_slot(
        repo,
        home,
        &format!("{addr}#goal"),
        &task,
        b"Deliver the thing.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#context"),
        &task,
        b"Under these constraints.\n",
    );
    fill_commit(repo, home, &task);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (plan) — the committed spec",
    );
    addr
}

/// **The registry sweep** (`DECISIONS.md` 2026-07-23 M45 Inc 5 T2). For EVERY object-form
/// `{type, as:}` create-gate pair enumerated from the composite registry, a second
/// `doc create` over the already-staged same-identity copy must **ack `existed`** (exit 0,
/// "already existed — copied in for update") — NOT reject with `create.serial-collision`,
/// which would route the agent away from the only repairing action. RED before the fix
/// (the second create rejects); GREEN after. A new pair joins the sweep by construction.
#[test]
fn every_object_form_pair_acks_existed_on_recreate() {
    let pairs = object_form_pairs();
    assert!(
        pairs.len() >= 20,
        "the composite registry must enumerate the object-form create-gate class \
         (a new pair is covered by construction); got {} pairs: {pairs:?}",
        pairs.len(),
    );
    // The class spans both symptom shapes: the `@`-slice (vision) and the `<<author:>>`
    // (spec) roles must both be present, so the sweep genuinely covers both.
    assert!(
        pairs.iter().any(|(_, t, _)| t == "vision"),
        "the `vision` role (the `@`-slice shape) is in the class: {pairs:?}",
    );
    assert!(
        pairs.iter().any(|(_, t, _)| t == "spec"),
        "the `spec` role (the `<<author:>>` shape) is in the class: {pairs:?}",
    );

    for (workflow, doctype, role) in pairs {
        let repo = TempDir::new(&format!("sweep-{workflow}-{doctype}"));
        let home = TempDir::new("home");
        init_repo(repo.path());

        let task = start(repo.path(), home.path(), &workflow, "sweep the class");
        // First create stages the instance (and binds the role — existing behavior).
        assert_ok(
            &jigc(
                repo.path(),
                home.path(),
                &[
                    "doc", "create", &doctype, "--title", "Sweep", "--task", &task,
                ],
                None,
            ),
            &format!("first `doc create {doctype}` under `{workflow}`"),
        );
        // Second create over the same-identity STAGED copy must ack `existed`, not reject.
        let again = jigc(
            repo.path(),
            home.path(),
            &[
                "doc", "create", &doctype, "--title", "Sweep", "--task", &task,
            ],
            None,
        );
        assert!(
            again.status.success(),
            "re-`doc create {doctype}` under `{workflow}` (role `{role}`) over a same-identity \
             staged copy must ack `existed` (exit 0), NOT reject with create.serial-collision; \
             stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&again.stdout),
            String::from_utf8_lossy(&again.stderr),
        );
        let stdout = String::from_utf8(again.stdout).expect("utf-8");
        assert!(
            stdout.contains("already existed"),
            "the re-create ack must say the doc `already existed` (copied in for update) \
             for `{workflow}`/`{doctype}`/`{role}`; got:\n{stdout}",
        );
    }
}

/// **Symptom shape 1** — `form-vision` on the SET-FIELD-FIRST (revise) path renders its
/// grounding. A committed VISION.md is revised by setting `grounded-in` FIRST (a
/// copy-on-write of the committed vision, no prior `doc create`); after re-compose the
/// `{{ @task.vision.grounded-in#findings }}` slice reads the grounding research's findings.
/// RED before the fix (copy-on-write does not bind `task.vision` → the slice is EMPTY).
#[test]
fn vision_set_field_first_binds_role_and_renders_grounding() {
    let repo = TempDir::new("vision-setfield-first");
    let home = TempDir::new("home");
    init_repo(repo.path());

    const FINDINGS: &str = "A single node caps throughput under contention.";
    let research = commit_research(
        repo.path(),
        home.path(),
        "benchmark the cache",
        "Cache Benchmarks",
        format!("{FINDINGS}\n").as_bytes(),
    );
    commit_vision(repo.path(), home.path());

    // Revise: a NEW form-vision task. Before touching the vision, the slice resolves empty.
    let task = start(repo.path(), home.path(), "form-vision", "revise the vision");
    let first = jigc(repo.path(), home.path(), &["start", "--task", &task], None);
    assert_ok(&first, "`jigc start --task` (revise, before set-field)");
    let first = String::from_utf8(first.stdout).expect("utf-8");
    assert!(
        !first.contains(FINDINGS),
        "before `grounded-in` is set, the grounding findings must be ABSENT; got:\n{first}",
    );

    // SET-FIELD-FIRST: this is the copy-on-write of the committed VISION.md — no prior
    // `doc create`. It must bind `task.vision` so the re-compose slice resolves.
    set_field(
        repo.path(),
        home.path(),
        "vision:vision#meta/grounded-in",
        &task,
        &format!("[{research}]"),
    );

    let recomposed = jigc(repo.path(), home.path(), &["start", "--task", &task], None);
    assert_ok(&recomposed, "`jigc start --task` (revise, re-compose)");
    let recomposed = String::from_utf8(recomposed.stdout).expect("utf-8");
    assert!(
        recomposed.contains(FINDINGS),
        "AFTER set-field-first copy-on-write binds `task.vision`, the \
         `@task.vision.grounded-in#findings` slice must READ the grounding research's findings \
         (symptom shape 1: silent-empty `@`-slice, cured); got:\n{recomposed}",
    );
}

/// **Symptom shape 2** — a role-based `<<author: {{ task.spec#goal }}>>` step renders the
/// REAL `spec:<slug>#goal` address after a copy-on-write, not the slug-less pending form.
/// A committed spec is revised by an edit verb (a `set-slot` copy-on-write) before any
/// `doc create`; after re-compose the author directive names `spec:<slug>#goal`.
/// RED before the fix (copy-on-write does not bind `task.spec` → the directive is
/// slug-less `<<author: spec#goal>>`).
#[test]
fn spec_copy_on_write_binds_role_renders_author_address() {
    let repo = TempDir::new("spec-cow");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let spec = commit_spec(repo.path(), home.path(), "Payment retry");
    assert_eq!(spec, "spec:payment-retry", "the committed spec's address");

    // Revise: a NEW plan task. Before touching the spec, the author directive is slug-less.
    let task = start(repo.path(), home.path(), "plan", "revise the spec");
    let first = jigc(repo.path(), home.path(), &["start", "--task", &task], None);
    assert_ok(&first, "`jigc start --task` (revise, before copy-on-write)");
    let first = String::from_utf8(first.stdout).expect("utf-8");
    assert!(
        first.contains("<<author: spec#goal>>"),
        "before the copy-on-write binds `task.spec`, the author directive renders the slug-less \
         pending form `<<author: spec#goal>>`; got:\n{first}",
    );

    // COPY-ON-WRITE: a `set-slot` on the committed spec — no prior `doc create`. It must
    // bind `task.spec` so the re-compose directive carries the real slug.
    set_slot(
        repo.path(),
        home.path(),
        &format!("{spec}#goal"),
        &task,
        b"Revised goal.\n",
    );

    let recomposed = jigc(repo.path(), home.path(), &["start", "--task", &task], None);
    assert_ok(&recomposed, "`jigc start --task` (revise, re-compose)");
    let recomposed = String::from_utf8(recomposed.stdout).expect("utf-8");
    assert!(
        recomposed.contains("<<author: spec:payment-retry#goal>>"),
        "AFTER the copy-on-write binds `task.spec`, the author directive must render the REAL \
         `spec:payment-retry#goal` address (symptom shape 2: slug-less `<<author:>>`, cured); \
         got:\n{recomposed}",
    );
    assert!(
        !recomposed.contains("<<author: spec#goal>>"),
        "the slug-less pending form must be GONE after binding; got:\n{recomposed}",
    );
}

/// **Assertion (c)** — `doc create` over an already-COPY-ON-WRITTEN same-identity staged
/// doc acks `existed` and the role resolves on re-compose, instead of `create.serial-collision`.
/// A committed VISION.md is copy-on-written by a `set-slot`, THEN `doc create vision` is run
/// (the workflow's `{{ cli.create-vision }}` line). RED before the fix (the create rejects
/// with serial-collision, routing away from the repair).
#[test]
fn create_over_copy_on_written_doc_acks_existed_and_binds() {
    let repo = TempDir::new("create-over-cow");
    let home = TempDir::new("home");
    init_repo(repo.path());

    const FINDINGS: &str = "Sharding removes the write-contention ceiling.";
    let research = commit_research(
        repo.path(),
        home.path(),
        "measure sharded writes",
        "Sharded Writes",
        format!("{FINDINGS}\n").as_bytes(),
    );
    commit_vision(repo.path(), home.path());

    let task = start(repo.path(), home.path(), "form-vision", "revise the vision");
    // Copy-on-write the committed vision via a slot edit (no prior `doc create`).
    set_slot(
        repo.path(),
        home.path(),
        "vision:vision#thesis",
        &task,
        b"A revised thesis.\n",
    );

    // Now the workflow's `doc create vision` line runs over the already-staged copy.
    let create = jigc(
        repo.path(),
        home.path(),
        &[
            "doc", "create", "vision", "--title", "Vision", "--task", &task,
        ],
        None,
    );
    assert!(
        create.status.success(),
        "`doc create vision` over an already-copy-on-written staged copy must ack `existed` \
         (exit 0), NOT reject with create.serial-collision; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&create.stdout),
        String::from_utf8_lossy(&create.stderr),
    );
    let create = String::from_utf8(create.stdout).expect("utf-8");
    assert!(
        create.contains("already existed"),
        "the create ack must say `already existed` (copied in for update); got:\n{create}",
    );

    // The role resolves on re-compose: set grounding + re-compose renders the findings.
    set_field(
        repo.path(),
        home.path(),
        "vision:vision#meta/grounded-in",
        &task,
        &format!("[{research}]"),
    );
    let recomposed = jigc(repo.path(), home.path(), &["start", "--task", &task], None);
    assert_ok(
        &recomposed,
        "`jigc start --task` re-compose after ack-existed create",
    );
    let recomposed = String::from_utf8(recomposed.stdout).expect("utf-8");
    assert!(
        recomposed.contains(FINDINGS),
        "after the ack-`existed` create binds `task.vision`, the grounding slice resolves; \
         got:\n{recomposed}",
    );
}

/// The gate-lookup's **wrong-doctype guard** (2026-07-24 mutation audit, finding #2):
/// a copy-on-write of a committed doc whose doctype is **not** in the workflow's
/// create-gate must bind **no** role — the inert arm of `bind_role_on_copy_in`.
/// `single-task`'s gate carries `{adr, as: decision}` + `{changelog, as: change}` and
/// no `spec` entry, so a `set-slot` copy-on-write of a committed spec under it fills
/// nothing. A lookup weakened to `doc_type == … || !as_role.is_empty()` would match
/// the first role-bearing entry for ANY doctype and silently bind
/// `decision → spec:<slug>` — every later `@task.decision` slice then reads the wrong
/// doc. The happy-path binding tests above never drive an out-of-gate doctype, so
/// this omitting-context arm is the one that kills that mutant.
#[test]
fn copy_on_write_of_an_out_of_gate_doctype_binds_no_role() {
    let repo = TempDir::new("out-of-gate-cow");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let spec = commit_spec(repo.path(), home.path(), "Payment retry");
    assert_eq!(spec, "spec:payment-retry", "the committed spec's address");

    // A task under `single-task` — its gate has no `spec` entry.
    let task = start(
        repo.path(),
        home.path(),
        "single-task",
        "tweak the committed spec",
    );

    // The copy-on-write: an ordinary edit verb on the committed out-of-gate spec.
    set_slot(
        repo.path(),
        home.path(),
        &format!("{spec}#goal"),
        &task,
        b"Revised goal.\n",
    );

    // The roles record stays empty: nothing may claim `decision` (or any role) for
    // the out-of-gate spec. An absent roles.json is the same no-binding fact.
    let roles_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("roles.json");
    if roles_path.exists() {
        let raw = fs::read_to_string(&roles_path).expect("read roles.json");
        let parsed: serde_json::Value = serde_json::from_str(&raw).expect("roles.json is JSON");
        let map = parsed["roles"]
            .as_object()
            .expect("roles.json carries a roles map");
        assert!(
            map.is_empty(),
            "a copy-on-write of an out-of-gate doctype must bind NO role; got:\n{raw}",
        );
    }
}
