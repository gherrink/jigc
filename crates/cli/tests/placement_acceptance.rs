//! M38 Increment 1 / T4 — the **placement** round-trip through the real `jigc` binary
//! (`design/storage.md` → Placement). The increment's whole Proves in one end-to-end
//! arc over both a repo-root literal home AND a `docs/`-direct literal home:
//!
//!   1. **create → author → finalize → promote.** A task authors two throwaway
//!      **placement** singletons — one `{ file: FOO.md }` (repo root), one
//!      `{ file: docs/bar.md }` (docs-direct) — and `finalize` promotes each to its
//!      **literal** home: `FOO.md` at the repo root and `docs/bar.md`, in ONE commit,
//!      case-preserved, with **no** `docs/foo/foo.md` one-file-folder (the literal is the
//!      home, not `<location>/<slug>.md`). This exercises `finalize::plan_promotions`.
//!   2. **addressable `type:type`.** `jigc doc create` mints each singleton at its
//!      fixed slug = type id — the emitted address is exactly `foo:foo` / `bar:bar`
//!      (never a `--title` slug, never a filename-derived id).
//!   3. **store-readable.** A second task on the promoted HEAD binds each committed
//!      placement doc to a declared `reads` role and re-composes: a `{{@task.<role>#body}}`
//!      ref addressing the placement doc (`foo:foo` / `bar:bar`) **dereferences to the
//!      committed slice** — the body prose, emitted as a `> ` blockquote. This exercises
//!      `store::canonical_path` (the ⚠ census site: without it every `{{@…}}` ref into a
//!      placement doc breaks). The `jigc task bind` step resolves each literal-file
//!      canonical path too.
//!   4. **freeze green.** No *shipped* doctype declares `placement`, so the engine
//!      pack-load freeze assertion recomputes every frozen doctype's `schema-hash`
//!      unchanged — proven implicitly here: every `jigc` invocation (which triggers the
//!      pack-load assertion) exits 0, so the throwaway placement doctypes never trip it.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, the throwaway placement doctypes ride a listed fixture pack
//! (the `finalize_root_render.rs` precedent), and a self-cleaning `TempDir` keeps the
//! test off the developer's repo. The base pack is the binary-embedded dev pack (its
//! `commit` doctype loads); the fixture pack unions the two placement doctypes + their
//! host/read workflows on top.
//!
//! **Increment 2 / T4** (below the round-trip test) reuses the same harness to prove
//! placement **reconciliation + ownership** end-to-end: an OOB edit to a committed
//! placement doc's literal `FOO.md` is detected + routed while sibling root `.md` are not
//! swept as instances, and a hand-authored foreign file at the managed literal path blocks
//! the first `finalize` at the inherited `finalize.promote-clobber` guard.

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
            "jigc-placement-{tag}-{}-{:?}",
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

/// Seed the fixture pack holding the two throwaway **placement** doctypes + their host
/// (create) and read (bind) workflows, and list it (highest-precedence) in the project
/// layer's `packs.yaml`. Each schema declares `placement: { file: … }` + `singleton: true`
/// (no `location:`); the host workflow's create-gate admits both so `jigc doc create`
/// mints in-task; the read workflow declares both as `reads` roles so a bound
/// `{{@task.<role>#body}}` dereferences the committed slice on re-compose.
fn seed_fixture_pack(repo: &Path, pack: &Path) {
    let schemas = pack.join("schemas");
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&schemas, &workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk fixture pack subdir");
    }

    // A repo-root literal home: `FOO.md`.
    fs::write(
        schemas.join("foo.yaml"),
        "type: foo\n\
         placement: { file: FOO.md }\n\
         singleton: true\n\
         description: A throwaway placement singleton whose one instance lives at the repo-root FOO.md.\n\
         usage: proving the placement literal-file round-trip through the binary.\n\
         sections:\n\
        \x20 - id: body\n\
        \x20   slot: { hint: the root page body }\n",
    )
    .expect("seed foo schema");

    // A `docs/`-direct literal home: `docs/bar.md` (no per-type folder).
    fs::write(
        schemas.join("bar.yaml"),
        "type: bar\n\
         placement: { file: docs/bar.md }\n\
         singleton: true\n\
         description: A throwaway placement singleton whose one instance lives at docs/bar.md.\n\
         usage: proving the docs-direct placement literal-file round-trip through the binary.\n\
         sections:\n\
        \x20 - id: body\n\
        \x20   slot: { hint: the docs page body }\n",
    )
    .expect("seed bar schema");

    // The host workflow: creates both placement singletons in one task.
    fs::write(
        workflows.join("author-pages.yaml"),
        "---\n\
         when: author the two placement pages\n\
         description: A host workflow that creates both placement singletons.\n\
         usage: proving the placement round-trip through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: foo, as: rootpage}, {type: bar, as: docspage}]\n\
         ---\n\
         {{ include: step:author-pages }}\n",
    )
    .expect("seed author-pages workflow");
    fs::write(
        steps.join("author-pages.yaml"),
        "Author the two placement pages for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed author-pages step");

    // The read workflow: binds each committed placement doc to a declared role, then a
    // `{{@task.<role>#body}}` ref dereferences the committed slice on re-compose.
    fs::write(
        workflows.join("read-pages.yaml"),
        "---\n\
         when: read the two committed placement pages\n\
         description: A read workflow that reads both placement singletons via bound roles.\n\
         usage: proving the placement docs are store-readable through the binary.\n\
         creates-task: true\n\
         reads: [{role: rootpage, type: foo}, {role: docspage, type: bar}]\n\
         ---\n\
         {{ include: step:read-pages }}\n",
    )
    .expect("seed read-pages workflow");
    fs::write(
        steps.join("read-pages.yaml"),
        "Read the two committed placement pages.\n\n\
         Root page (foo:foo) body:\n\n\
         {{ @task.rootpage#body }}\n\n\
         Docs page (bar:bar) body:\n\n\
         {{ @task.docspage#body }}\n",
    )
    .expect("seed read-pages step");

    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the fixture pack");
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// then seed the fixture pack. Returns the repo + $HOME + fixture-pack temp dirs (all
/// kept alive for the test's duration).
fn init_repo() -> (TempDir, TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = TempDir::new("pack");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    seed_fixture_pack(repo.path(), pack.path());
    (repo, home, pack)
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
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
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// Assert a `jigc` invocation exited 0, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The distinctive body prose authored into each placement singleton — the bytes a
/// bound `{{@task.<role>#body}}` ref must dereference to on the read task's re-compose.
const FOO_BODY: &str = "The root page body, managed directly at the repo root.";
const BAR_BODY: &str = "The docs page body, managed directly under docs/.";

/// Set a slot from `prose` for `addr`, asserting it succeeds.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &run_jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set a field `value` for `addr`, asserting it succeeds.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &run_jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

#[test]
fn placement_singletons_round_trip_to_their_literal_homes() {
    let (repo, home, _pack) = init_repo();
    let repo = repo.path();
    let home = home.path();

    // ── Task 1: author both placement singletons and finalize ─────────────────────
    let start = run_jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "author-pages",
            "author the placement pages",
        ],
        None,
    );
    assert_ok(&start, "`jigc start --workflow author-pages`");
    let task = "author-the-placement-pages";

    // create → the emitted address is EXACTLY `<type>:<type>` (fixed slug = type id,
    // never a `--title` slug) — the addressable proof.
    let create_foo = run_jigc(
        repo,
        home,
        &["doc", "create", "foo", "--title", "Root Page"],
        None,
    );
    assert_ok(&create_foo, "`jigc doc create foo`");
    assert_eq!(
        String::from_utf8_lossy(&create_foo.stdout).trim(),
        "foo:foo",
        "a placement singleton mints at its fixed slug = type id (`foo:foo`), never a --title slug",
    );
    let create_bar = run_jigc(
        repo,
        home,
        &["doc", "create", "bar", "--title", "Docs Page"],
        None,
    );
    assert_ok(&create_bar, "`jigc doc create bar`");
    assert_eq!(
        String::from_utf8_lossy(&create_bar.stdout).trim(),
        "bar:bar",
        "a docs-direct placement singleton also mints at its fixed slug = type id (`bar:bar`)",
    );

    set_slot(
        repo,
        home,
        "foo:foo#body",
        format!("{FOO_BODY}\n").as_bytes(),
    );
    set_slot(
        repo,
        home,
        "bar:bar#body",
        format!("{BAR_BODY}\n").as_bytes(),
    );

    // The dev-pack `commit` doc's two required levers so finalize validates clean.
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"author the placement pages\n",
    );

    let commits_before: u32 = git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap();
    let finalize = run_jigc(repo, home, &["task", "finalize", task], None);
    assert_ok(&finalize, "`jigc task finalize` (author-pages)");

    // Exactly ONE new commit carries BOTH placement docs at their LITERAL homes.
    let commits_after: u32 = git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap();
    assert_eq!(
        commits_after,
        commits_before + 1,
        "the placement finalize must produce exactly ONE new commit",
    );
    let files = git(repo, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "FOO.md"),
        "the root placement doc must promote to the case-preserved literal `FOO.md`; files:\n{files}",
    );
    assert!(
        files.lines().any(|l| l == "docs/bar.md"),
        "the docs-direct placement doc must promote to its literal `docs/bar.md`; files:\n{files}",
    );
    // NO one-file-folder: the home is the literal path, not `<location>/<slug>.md`.
    assert!(
        !files
            .lines()
            .any(|l| l == "docs/foo/foo.md" || l == "foo/foo.md"),
        "a placement doc must NOT land at a `<location>/<slug>.md` folder home; files:\n{files}",
    );
    // The committed FOO.md carries the authored body (the round-tripped managed doc).
    let committed_foo = git(repo, &["show", "HEAD:FOO.md"]);
    assert!(
        committed_foo.contains(FOO_BODY),
        "the committed root `FOO.md` must carry the authored body prose; got:\n{committed_foo}",
    );
    let committed_bar = git(repo, &["show", "HEAD:docs/bar.md"]);
    assert!(
        committed_bar.contains(BAR_BODY),
        "the committed `docs/bar.md` must carry the authored body prose; got:\n{committed_bar}",
    );

    // ── Task 2: read both committed placement docs via bound roles ────────────────
    let read_start = run_jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "read-pages",
            "read the placement pages",
        ],
        None,
    );
    assert_ok(&read_start, "`jigc start --workflow read-pages`");
    let read_task = "read-the-placement-pages";

    // Before the bind the `{{@task.<role>#body}}` refs resolve to nothing — the body
    // prose must NOT appear (proving the later hit is a real dereference, not literal).
    let first = String::from_utf8_lossy(&read_start.stdout);
    assert!(
        !first.contains(FOO_BODY) && !first.contains(BAR_BODY),
        "before the bind, the unbound placement refs resolve empty; got:\n{first}",
    );

    // Bind each committed placement doc to its declared role — `jigc task bind` resolves
    // each literal-file canonical path (`FOO.md` / `docs/bar.md`) in the committed store.
    assert_ok(
        &run_jigc(
            repo,
            home,
            &["task", "bind", "rootpage", "foo:foo", read_task],
            None,
        ),
        "`jigc task bind rootpage foo:foo`",
    );
    assert_ok(
        &run_jigc(
            repo,
            home,
            &["task", "bind", "docspage", "bar:bar", read_task],
            None,
        ),
        "`jigc task bind docspage bar:bar`",
    );

    // Re-compose — each `{{@task.<role>#body}}` dereferences to the committed slice,
    // emitted as a `> ` blockquote (store-readable through `canonical_path`, both the
    // root and the docs-direct literal home).
    let resume = run_jigc(repo, home, &["start", "--task", read_task], None);
    assert_ok(&resume, "`jigc start --task <read-task>` re-compose");
    let composed = String::from_utf8_lossy(&resume.stdout);
    assert!(
        composed.contains(&format!("> {FOO_BODY}")),
        "the root placement doc's `#body` must dereference to its committed slice as a \
         blockquote (store-readable at the literal root home); got:\n{composed}",
    );
    assert!(
        composed.contains(&format!("> {BAR_BODY}")),
        "the docs-direct placement doc's `#body` must dereference to its committed slice as a \
         blockquote (store-readable at the literal docs/ home); got:\n{composed}",
    );
    // The slices dereferenced — the bare placeholders must be gone.
    assert!(
        !composed.contains("{{ @task.rootpage#body }}")
            && !composed.contains("{{ @task.docspage#body }}"),
        "the placement refs must be resolved, not emitted verbatim; got:\n{composed}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────────────
// M38 Increment 2 / T4 — **placement reconciliation + ownership** through the real binary
// (`design/storage.md` → Placement census; `design/reconciliation.md`; `design/finalize.md`).
// The increment's whole Proves end-to-end over the same throwaway `foo { file: FOO.md }`
// singleton the Inc-1 harness above mints — honestly verifying, not trusting, that Inc-2's
// engine/CLI census sites teach the sweep + the clobber guard the exact-path ownership of a
// placement doctype's one literal `placement.file`.
// ─────────────────────────────────────────────────────────────────────────────────────

/// Parse the `findings` array of a `--format json` report envelope (the store-scope
/// `task validate` / finalize envelope shape, `store_sweep_acceptance.rs` precedent).
fn findings_json(out: &std::process::Output) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the report envelope must parse ({e}); got:\n{stdout}"));
    value["findings"]
        .as_array()
        .cloned()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
}

/// Whether any finding's message mentions `needle` (the raw path a swept instance is keyed
/// on) — the discriminator for "was this file swept as an instance?".
fn any_message_mentions(findings: &[serde_json::Value], needle: &str) -> bool {
    findings
        .iter()
        .any(|f| f["message"].as_str().is_some_and(|m| m.contains(needle)))
}

/// The keys of the persisted `file-state` record (`.jigc/state/file-state.json` → `hashes`)
/// — the baseline-adopt manifest, used to prove a sibling root `.md` is **not** adopted.
fn file_state_keys(repo: &Path) -> Vec<String> {
    let record_path = repo.join(".jigc").join("state").join("file-state.json");
    let record = fs::read_to_string(&record_path)
        .expect("the landed finalize persisted the file-state record");
    let value: serde_json::Value = serde_json::from_str(&record).expect("file-state.json parses");
    value["hashes"]
        .as_object()
        .expect("the record carries a `hashes` map")
        .keys()
        .cloned()
        .collect()
}

/// Author the `foo` placement singleton (body = `FOO_BODY`) and `finalize` it, so a
/// committed, **file-state-recorded** `FOO.md` exists at the repo-root literal home. Shared
/// setup for the OOB-drift arm below.
fn author_and_finalize_foo(repo: &Path, home: &Path) {
    let start = run_jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "author-pages",
            "author the root page",
        ],
        None,
    );
    assert_ok(&start, "`jigc start --workflow author-pages`");
    let task = "author-the-root-page";
    let create = run_jigc(
        repo,
        home,
        &["doc", "create", "foo", "--title", "Root Page"],
        None,
    );
    assert_ok(&create, "`jigc doc create foo`");
    set_slot(
        repo,
        home,
        "foo:foo#body",
        format!("{FOO_BODY}\n").as_bytes(),
    );
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"author the root page\n",
    );
    let finalize = run_jigc(repo, home, &["task", "finalize", task], None);
    assert_ok(&finalize, "`jigc task finalize` (author the root page)");
    // Sanity: the committed FOO.md is baseline-recorded (the drift arm needs the baseline).
    assert!(
        file_state_keys(repo).iter().any(|k| k == "FOO.md"),
        "finalize must baseline-record the promoted placement doc at its literal `FOO.md`",
    );
}

/// (M38 inc-2 T4 — arms (a) + (b)) An **out-of-band edit to a committed placement doc's
/// literal `FOO.md` is detected + routed** by the reconcile/validate sweep — AND a sibling
/// root `README.md`/`CLAUDE.md` is **not** swept as an instance. Both live in ONE test so
/// the pair is atomic: arm (a) proves the placement sweep arm ran and visited the root
/// literal (else the drift is silently clean — the pre-Inc-1 `location: None` skip); arm (b)
/// proves it did **not** glob the root dir (exact-path ownership, not a dir-glob). A green
/// (b) alone would pass vacuously if the sweep never ran, so the FOO.md-detected +
/// siblings-untouched conjunction is the non-vacuous exact-path proof
/// (`design/storage.md` → Placement census: `reconcile_committed_store`; ownership is
/// "exact-path equality against `placement.file`, every other root `.md` stays unmanaged").
#[test]
fn placement_oob_edit_detected_and_sibling_root_md_not_swept() {
    let (repo, home, _pack) = init_repo();
    let repo = repo.path();
    let home = home.path();

    // A committed, recorded placement doc at the literal `FOO.md`.
    author_and_finalize_foo(repo, home);

    // A sibling root `.md` the schema declares nothing about — the exact-path ownership
    // subject: a literal `placement.file` is not a root dir-glob, so this must stay
    // unmanaged. (`README.md` is already committed by `init_repo`; add a `CLAUDE.md` too.)
    fs::write(
        repo.join("CLAUDE.md"),
        "# project rules\n\nnot a managed doc\n",
    )
    .expect("write CLAUDE.md sibling");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md sibling"]);

    // The **out-of-band** edit: a human rewrites the committed `FOO.md` through git,
    // nonconformantly (renamed required section heading), outside the CLI.
    let on_disk = fs::read_to_string(repo.join("FOO.md")).expect("read committed FOO.md");
    let drifted = on_disk.replace("## Body", "## Bodyz");
    assert_ne!(
        drifted, on_disk,
        "the OOB edit must actually change the heading"
    );
    fs::write(repo.join("FOO.md"), &drifted).expect("write the OOB-edited FOO.md");
    git(repo, &["add", "FOO.md"]);
    git(
        repo,
        &["commit", "-q", "-m", "human edits FOO.md out of band"],
    );

    // A later task's store-scope sweep reconciles the committed store. `task validate`
    // exits blocking on the recorded-then-drifted nonconformant edit, so capture the
    // envelope regardless of exit and read its findings.
    let vstart = run_jigc(
        repo,
        home,
        &["start", "--workflow", "author-pages", "reconcile the store"],
        None,
    );
    assert_ok(&vstart, "`jigc start` (reconcile pass)");
    let out = run_jigc(
        repo,
        home,
        &[
            "task",
            "validate",
            "reconcile-the-store",
            "--format",
            "json",
        ],
        None,
    );
    let findings = findings_json(&out);

    // (a) the OOB edit to the managed placement file is **detected + routed** — a blocking
    // `reconciliation.conformance-block` naming the literal `FOO.md`, carrying a route
    // (not silently clean, the "managed, not a mirror" promise).
    let block = findings
        .iter()
        .find(|f| {
            f["code"] == "reconciliation.conformance-block"
                && f["message"].as_str().is_some_and(|m| m.contains("FOO.md"))
        })
        .unwrap_or_else(|| {
            panic!("the placement doc's OOB drift must be detected + routed; got:\n{findings:#?}")
        });
    assert_eq!(
        block["severity"], "blocking",
        "a recorded-then-drifted placement doc conformance-blocks: {block:#?}",
    );
    assert_eq!(
        block["location"]["address"], "FOO.md",
        "the block is keyed on the literal placement path: {block:#?}",
    );
    assert!(
        block["route"].is_string(),
        "the conformance-block carries a route to a human: {block:#?}",
    );

    // (b) a sibling root `.md` (`README.md`, `CLAUDE.md`) is **not** swept as an instance —
    // no finding names it (the sweep visited the literal `FOO.md`, never globbed the root).
    assert!(
        !any_message_mentions(&findings, "README.md")
            && !any_message_mentions(&findings, "CLAUDE.md"),
        "an undeclared sibling root .md must not be swept as a placement instance; got:\n{findings:#?}",
    );
    // …and it is not baseline-adopted into the record (a literal file is not a root glob).
    let keys = file_state_keys(repo);
    assert!(
        !keys.iter().any(|k| k == "README.md") && !keys.iter().any(|k| k == "CLAUDE.md"),
        "a sibling root .md must not be baseline-adopted as an instance; record keys: {keys:?}",
    );
}

/// (M38 inc-2 T4 — arm (c)) A **hand-authored foreign file at the managed literal path
/// blocks the first `finalize` with `finalize.promote-clobber`** — the inherited
/// `plan_clobber_guard`, *zero new guard code* (`design/storage.md` → Placement: "the
/// no-silent-data-loss guard is inherited, not rebuilt … the general `plan_clobber_guard`
/// already blocks any create-promote whose destination file already exists"; `finalize.md`).
///
/// The guard is gated to `Provenance::Created`, so the placement doc must be **minted fresh**
/// — the agent `doc create`s it while the literal path is empty (Created), then a human drops
/// a foreign `FOO.md` at that path out-of-band before `finalize`. Promoting the Created doc
/// would overwrite the foreign file → the inherited guard refuses. (A foreign file present at
/// `doc create` time is instead *copied in* for editing — `EditedFromBase`, the reconciliation
/// adoption path — a distinct, also-routed door; this arm exercises the create-promote clobber
/// the increment names.) Proves the guard fires with no new code, commits nothing, and leaves
/// the foreign bytes intact.
#[test]
fn foreign_file_at_placement_path_blocks_finalize_with_clobber_guard() {
    let (repo, home, _pack) = init_repo();
    let repo = repo.path();
    let home = home.path();

    let start = run_jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "author-pages",
            "author the root page",
        ],
        None,
    );
    assert_ok(&start, "`jigc start --workflow author-pages`");
    let task = "author-the-root-page";

    // Mint the placement doc while the literal path is **empty** → `Provenance::Created`.
    let create = run_jigc(
        repo,
        home,
        &["doc", "create", "foo", "--title", "Root Page"],
        None,
    );
    assert_ok(&create, "`jigc doc create foo`");
    set_slot(
        repo,
        home,
        "foo:foo#body",
        format!("{FOO_BODY}\n").as_bytes(),
    );
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"author the root page\n",
    );

    // A human hand-authors a foreign `FOO.md` at the managed literal path, out-of-band,
    // before finalize — the destination the Created doc would promote to now holds a file.
    const FOREIGN: &str = "# Foreign\n\nHand-authored directly, not through jigc.\n";
    fs::write(repo.join("FOO.md"), FOREIGN).expect("hand-author the foreign FOO.md");

    let commits_before: u32 = git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap();
    let out = run_jigc(
        repo,
        home,
        &["task", "finalize", task, "--format", "json"],
        None,
    );
    // The finalize must NOT succeed — it blocks on the clobber guard.
    assert!(
        !out.status.success(),
        "finalize must block on the foreign-file clobber; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let findings = findings_json(&out);
    let clobber = findings
        .iter()
        .find(|f| {
            f["code"] == "finalize.promote-clobber"
                && f["message"].as_str().is_some_and(|m| m.contains("FOO.md"))
        })
        .unwrap_or_else(|| {
            panic!("the foreign FOO.md must block first finalize at finalize.promote-clobber; got:\n{findings:#?}")
        });
    assert_eq!(clobber["severity"], "blocking");

    // Nothing committed past the pre-finalize HEAD, and the foreign bytes are byte-intact.
    let commits_after: u32 = git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap();
    assert_eq!(
        commits_after, commits_before,
        "the refused clobber commits nothing",
    );
    assert_eq!(
        fs::read_to_string(repo.join("FOO.md")).expect("read FOO.md"),
        FOREIGN,
        "the refused clobber leaves the foreign file byte-intact",
    );
}
