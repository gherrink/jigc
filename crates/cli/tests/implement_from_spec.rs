//! Headline acceptance — the M3 intent → spec → implementation arc end-to-end
//! (`design/worked-examples.md` → flow 6; `implementation/roadmap.md` → M3 Increment 4
//! Proves). The mandated acceptance path: a `plan` task authors + commits a spec,
//! then `implement-from-spec` surfaces it, binds it, re-composes to read its criteria,
//! implements, and finalize records + walks `commit —implements→ spec`.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo through the whole
//! inc-4 stack — the `store.specs` committed-instance collection, cross-task binding
//! (`task bind`), the context-slice over a persisted spec's repeatable `criteria`
//! section (re-read on `start --task`), and the forward-ref integrity walk over the
//! `implements` edge at finalize:
//!
//! - **Task 1 (`plan`)** creates + authors + finalizes `spec:gateway-rate-limiting`
//!   — asserts `docs/specs/gateway-rate-limiting.md` is committed at its canonical path.
//!   One `criteria` item is then seeded via the **editable channel** (append +
//!   git-commit), because there is no `add-item` verb yet (`plan` ships a
//!   criteria-less-but-valid spec); this mirrors how `superseding_decision` seeds its
//!   committed ADR. The slice assertion needs ≥1 criterion to dereference.
//! - **Task 2 (`implement-from-spec`)** mints on the spec-bearing HEAD; the
//!   `locate-from-spec` step's `{{store.specs}}` lists the committed spec. `jigc task
//!   bind spec spec:<slug> <id>` records the binding, and `jigc start --task <id>`
//!   re-composes: `{{@task.spec#criteria}}` resolves to the committed spec's `#criteria`
//!   prose — each item's `### <title>` + statement, emitted as a `> ` blockquote (NOT
//!   the bare `{{…}}` placeholder). This is the round-trip over a committed, human-
//!   editable file + the context-slice over the persisted spec (`worked-examples.md`
//!   → flow 6 Task 2).
//! - **Finalize — the edge walk** passes: `commit:<id>#implements → spec:<slug>` finds
//!   its target in the committed store, so finalize lands exactly one commit.
//! - **The dangling variant** points `implements` at a spec in neither surface; the
//!   `schema-conformance.ref-resolves` walk blocks finalize non-zero, naming the
//!   dangling target + the three routing options (fix / create-in-task / drop), and
//!   creates NO commit (`worked-examples.md` → flow 6; `validation.md` → Forward-ref
//!   resolution).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's repo.

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
            "jigc-impl-from-spec-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
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

/// The slug `id-from: title` derives for the spec authored in task 1.
const SPEC_SLUG: &str = "gateway-rate-limiting";

/// The committed criterion's statement prose — the bytes `{{@task.spec#criteria}}`
/// must dereference to on resume (re-read + sliced from the committed spec).
const CRITERION_STATEMENT: &str = "The gateway rejects the 101st request in a rolling 60s window.";

/// Stage a slot from `prose` for `addr`, asserting it succeeds.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc_doc(
        repo,
        home,
        &["set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert_ok(&out, &format!("set-slot {addr}"));
}

/// Stage a field `value` for `addr`, asserting it succeeds.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
    assert_ok(&out, &format!("set-field {addr}"));
}

/// Setup — task 1 (`plan`) creates + authors + finalizes the spec, then seeds one
/// `criteria` item via the editable channel (append + git-commit). After this the
/// committed `docs/specs/<slug>.md` carries one dereferenceable criterion.
fn commit_spec_with_criterion(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "plan", "draft the rate limit spec"],
    );
    assert_ok(&out, "`jigc start --workflow plan` (task 1)");
    let task = "draft-the-rate-limit-spec";

    let create = jigc_doc(
        repo,
        home,
        &["create", "spec", "--title", "Gateway rate limiting"],
        None,
    );
    assert_ok(&create, "`jigc doc create spec` (task 1)");
    let spec = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(spec, format!("spec:{SPEC_SLUG}"));

    set_slot(
        repo,
        home,
        &format!("spec:{SPEC_SLUG}#goal"),
        b"Bound per-client request volume at the gateway.\n",
    );
    set_slot(
        repo,
        home,
        &format!("spec:{SPEC_SLUG}#context"),
        b"Downstream services each enforced limits ad hoc.\n",
    );

    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), "gateway");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"draft the rate limit spec\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Capture the rate-limit criteria before coding.\n",
    );

    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize` (task 1)");

    // The spec is committed at its canonical path (the persisted differentiator).
    let committed = Command::new("git")
        .args(["show", &format!("HEAD:docs/specs/{SPEC_SLUG}.md")])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "task 1 must commit docs/specs/{SPEC_SLUG}.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );

    // Seed one conformant `criteria` item via the EDITABLE CHANNEL — there is no
    // `add-item` verb yet, so the e2e hand-writes a `### <title>  {#id}` + statement
    // block and git-commits it, exactly as `superseding_decision` seeds its committed
    // ADR. This gives the slice assertion ≥1 criterion to dereference on resume.
    let spec_path = repo
        .join("docs")
        .join("specs")
        .join(format!("{SPEC_SLUG}.md"));
    let mut body = fs::read_to_string(&spec_path).expect("read committed spec");
    body.push_str(&format!(
        "\n### Rejects the 101st request  {{#rejects-burst}}\n\n{CRITERION_STATEMENT}\n"
    ));
    fs::write(&spec_path, &body).expect("seed criterion");
    git(repo, &["add", &format!("docs/specs/{SPEC_SLUG}.md")]);
    git(repo, &["commit", "-q", "-m", "seed acceptance criterion"]);
}

#[test]
fn implement_from_spec_arc_slices_criteria_and_passes_the_implements_edge_walk() {
    let repo = TempDir::new("pass");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── Setup: task 1 commits spec:gateway-rate-limiting with one criterion ───────
    commit_spec_with_criterion(repo.path(), home.path());

    // ── Task 2: implement-from-spec mints on the spec-bearing HEAD ────────────────
    let task = "enforce-the-rate-limit";
    let mint = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "enforce the rate limit at the gateway",
        ],
    );
    assert_ok(
        &mint,
        "`jigc start --workflow implement-from-spec` (task 2)",
    );

    // The first compose surfaces the committed spec via `{{store.specs}}` (the
    // locate-from-spec step lists what is bindable).
    let first = String::from_utf8(mint.stdout).expect("utf-8");
    assert!(
        first.contains(&format!("spec:{SPEC_SLUG}")),
        "the first compose must list the committed spec via {{{{store.specs}}}}; got:\n{first}",
    );
    // The unbound criteria slice has nothing to dereference yet — it must NOT carry
    // the criterion prose before the bind.
    assert!(
        !first.contains(CRITERION_STATEMENT),
        "before the bind, {{{{@task.spec#criteria}}}} resolves to nothing; got:\n{first}",
    );

    // ── Bind the spec this work implements, then re-compose ───────────────────────
    let bind = jigc(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), task],
    );
    assert_ok(&bind, "`jigc task bind spec` (task 2)");

    let resume = jigc(repo.path(), home.path(), &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose (task 2)");
    let composed = String::from_utf8(resume.stdout).expect("utf-8");

    // (a) The spec slice resolves on resume: `{{@task.spec#criteria}}` dereferences
    //     to the committed spec's criterion — its `### <title>` + statement prose,
    //     emitted as a `> ` blockquote (re-read + sliced from the committed file).
    assert!(
        composed.contains("> ### Rejects the 101st request"),
        "the criteria slice must render the criterion's `### <title>` as a blockquote; \
         got:\n{composed}",
    );
    assert!(
        composed.contains(&format!("> {CRITERION_STATEMENT}")),
        "the criteria slice must dereference to the criterion's statement prose; got:\n{composed}",
    );
    // The slice dereferenced — the bare placeholder must be gone.
    assert!(
        !composed.contains("{{@task.spec#criteria}}")
            && !composed.contains("{{ @task.spec#criteria }}"),
        "the criteria placeholder must be resolved, not emitted verbatim; got:\n{composed}",
    );

    // ── Author the commit doc + set the implements edge → the bound spec ──────────
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#implements"),
        &format!("spec:{SPEC_SLUG}"),
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "feat",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#scope"),
        "gateway",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"enforce the gateway rate limit\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#body"),
        b"Bound per-client volume at the edge.\n",
    );

    // A real code change, staged (M30 G5) so the narrowed IndexHonoring finalize commits
    // it — the empty-commit guard is satisfied by more than the doc.
    fs::write(repo.path().join("limiter.txt"), "rate limiter\n").expect("write code change");
    git(repo.path(), &["add", "limiter.txt"]);

    // ── Finalize: the implements edge walk passes (target in the committed store) ──
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    // (b) finalize PASSES the implements forward-ref walk and lands one commit.
    assert_ok(
        &out,
        "`jigc task finalize` (task 2) — the implements edge walk must pass",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "task 2 finalize must land exactly ONE commit"
    );

    // The rendered commit subject carries the `feat` type — the task implemented code.
    let message = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert!(
        message.starts_with("feat"),
        "the rendered commit subject must carry the `feat` type; got:\n{message}",
    );
}

#[test]
fn dangling_implements_blocks_finalize_with_the_three_routing_options() {
    let repo = TempDir::new("dangle");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // The committed spec exists, but the dangling implements ref points elsewhere.
    commit_spec_with_criterion(repo.path(), home.path());

    let task = "build-the-second-thing";
    let mint = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "build the second thing",
        ],
    );
    assert_ok(
        &mint,
        "`jigc start --workflow implement-from-spec` (dangling task)",
    );

    // Point implements at a spec in NEITHER surface — the dangling ref.
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#implements"),
        "spec:nonexistent-spec",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "feat",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#scope"),
        "x",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"build the second thing\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#body"),
        b"A second change.\n",
    );
    fs::write(repo.path().join("thing.txt"), "change\n").expect("write code change");

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    // (c) the dangling variant BLOCKS finalize non-zero.
    assert!(
        !out.status.success(),
        "a dangling implements must make finalize exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    // The block is the forward-ref / edge-index check, names the dangling target...
    assert!(
        rendered.contains("schema-conformance.ref-resolves"),
        "the block must be the forward-ref resolution check; got:\n{rendered}",
    );
    assert!(
        rendered.contains("spec:nonexistent-spec"),
        "the block must name the dangling target; got:\n{rendered}",
    );
    // ...and surfaces the three routing options: fix / create-in-task / drop.
    assert!(
        rendered.contains("fix")
            && rendered.contains("create the target in this task")
            && rendered.contains("drop"),
        "the block must surface the three routing options; got:\n{rendered}",
    );

    // No commit was created; nothing promoted.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a dangling-ref block must create no commit");
}

// ── M52 Increment 9 T3 — the legitimately-empty renders state their empty case ────

/// The `{{store.specs}}` render's asserting sentence — the paragraph that tells the
/// agent to pick from a list, and therefore the one that owes the empty case.
const SPECS_ASSERTING_HEAD: &str = "Pick the spec this work implements";

/// The `{{@task.spec#criteria}}` render's asserting sentence.
const CRITERIA_ASSERTING_HEAD: &str = "The bound spec's criteria";

/// The empty-case clause `{{store.specs}}`'s asserting sentence must carry.
/// **Conditional** by construction, so it stays true over a populated store too —
/// the shape the pack already ships at `step:superseded-context` and methodology's
/// `step:author-vision`.
const SPECS_EMPTY_CASE: &str = "nothing is listed until a `spec` is committed";

/// The empty-case clause `{{@task.spec#criteria}}`'s asserting sentence must carry.
const CRITERIA_EMPTY_CASE: &str = "nothing appears until you bind a spec above and re-compose";

/// The composed bytes with every whitespace run collapsed to one space — step prose
/// is hard-wrapped, so a clause legitimately spans a line break and matching the raw
/// bytes would fail on presentation rather than on content.
fn unwrapped(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The blank-line-separated block of `composed` that opens with `head`, paired with
/// the block that follows it — the asserting sentence and the render it introduces.
/// An empty render is an empty block, which is exactly the cell under test.
fn asserting_block_and_render<'a>(composed: &'a str, head: &str) -> (&'a str, &'a str) {
    let blocks: Vec<&str> = composed.split("\n\n").collect();
    let at = blocks
        .iter()
        .position(|block| block.trim_start().starts_with(head))
        .unwrap_or_else(|| {
            panic!("the composed workflow must carry the `{head}` paragraph; got:\n{composed}")
        });
    (blocks[at], blocks.get(at + 1).copied().unwrap_or(""))
}

/// D9(2) — `implement-from-spec` asserts twice over a set it then renders (*"from the
/// committed specs below"*, *"The bound spec's criteria … :"*). Over a corpus with no
/// committed `spec` both renders come out **empty** under an asserting sentence: the
/// agent is pointed at a list that is not there, which is law 1's lie. The fix is the
/// shape the pack already ships twice — a **conditional** clause inside the asserting
/// sentence itself (`step:superseded-context`'s *"nothing appears if it supersedes
/// none"*; methodology `step:author-vision`'s grounding parenthetical).
///
/// Driven on **both** cells, because a green pass over one hides the other:
///
///  - the omitting corpus (no committed `spec`): each render is empty, and each
///    asserting sentence carries its empty-case clause;
///  - the composing corpus (one committed `spec`, then bound): each render is
///    populated, and the same clauses are present **byte-identical** — so a populated
///    compose never claims the set is empty, the clause being conditional, not a
///    state-dependent assertion that could go stale.
#[test]
fn the_two_empty_renders_state_their_empty_case_and_stay_true_when_populated() {
    // ── Cell 1: the omitting corpus — no committed `spec` anywhere ────────────────
    let empty_repo = TempDir::new("emptycase");
    let empty_home = TempDir::new("home");
    init_repo(empty_repo.path());

    let mint = jigc(
        empty_repo.path(),
        empty_home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "enforce the rate limit at the gateway",
        ],
    );
    assert_ok(
        &mint,
        "`jigc start --workflow implement-from-spec` (no spec)",
    );
    let spec_less = String::from_utf8(mint.stdout).expect("utf-8");

    let (specs_say, specs_render) = asserting_block_and_render(&spec_less, SPECS_ASSERTING_HEAD);
    assert!(
        specs_render.trim().is_empty(),
        "over a spec-less corpus `{{{{store.specs}}}}` must render empty — that is the \
         cell this claim is about; got:\n{spec_less}",
    );
    assert!(
        unwrapped(specs_say).contains(SPECS_EMPTY_CASE),
        "the asserting sentence over an empty `{{{{store.specs}}}}` must state its empty \
         case (`{SPECS_EMPTY_CASE}`); got:\n{specs_say}",
    );

    let (criteria_say, criteria_render) =
        asserting_block_and_render(&spec_less, CRITERIA_ASSERTING_HEAD);
    assert!(
        criteria_render.trim().is_empty(),
        "with nothing bound `{{{{@task.spec#criteria}}}}` must render empty; got:\n{spec_less}",
    );
    assert!(
        unwrapped(criteria_say).contains(CRITERIA_EMPTY_CASE),
        "the asserting sentence over an empty `{{{{@task.spec#criteria}}}}` must state its \
         empty case (`{CRITERIA_EMPTY_CASE}`); got:\n{criteria_say}",
    );

    // ── Cell 2: the composing corpus — one committed spec, bound and re-composed ──
    let full_repo = TempDir::new("populated");
    let full_home = TempDir::new("home");
    init_repo(full_repo.path());
    commit_spec_with_criterion(full_repo.path(), full_home.path());

    let task = "enforce-the-rate-limit";
    let mint = jigc(
        full_repo.path(),
        full_home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "enforce the rate limit at the gateway",
        ],
    );
    assert_ok(
        &mint,
        "`jigc start --workflow implement-from-spec` (one spec)",
    );
    let listed = String::from_utf8(mint.stdout).expect("utf-8");

    let bind = jigc(
        full_repo.path(),
        full_home.path(),
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), task],
    );
    assert_ok(&bind, "`jigc task bind spec`");
    let resume = jigc(
        full_repo.path(),
        full_home.path(),
        &["start", "--task", task],
    );
    assert_ok(&resume, "`jigc start --task <id>` re-compose");
    let bound = String::from_utf8(resume.stdout).expect("utf-8");

    let (populated_specs_say, populated_specs_render) =
        asserting_block_and_render(&listed, SPECS_ASSERTING_HEAD);
    assert!(
        populated_specs_render.contains(&format!("spec:{SPEC_SLUG}")),
        "with a committed spec the render must still list it; got:\n{listed}",
    );
    let (populated_criteria_say, populated_criteria_render) =
        asserting_block_and_render(&bound, CRITERIA_ASSERTING_HEAD);
    assert!(
        populated_criteria_render.contains(CRITERION_STATEMENT),
        "with a spec bound the criteria render must still dereference; got:\n{bound}",
    );

    // The clauses are static and conditional: byte-identical across both cells, so a
    // populated compose carries no claim that the set is empty.
    assert_eq!(
        populated_specs_say, specs_say,
        "the `{{{{store.specs}}}}` asserting sentence must read the same over a populated \
         store — a conditional empty case, never a claim about this corpus",
    );
    assert_eq!(
        populated_criteria_say, criteria_say,
        "the `{{{{@task.spec#criteria}}}}` asserting sentence must read the same once a \
         spec is bound — a conditional empty case, never a claim about this task",
    );
}
