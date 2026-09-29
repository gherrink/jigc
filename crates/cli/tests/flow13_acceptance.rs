//! Flow 13 acceptance — M10's headline: **documentation drift caught deterministically
//! at the task boundary**, proven end-to-end through the built `jigc` binary against the
//! **real** tree-sitter `doc-code` probe (never a stub). This is the mandated acceptance
//! path that completes the last deferred differentiator
//! (`design/worked-examples.md` → flow 13; `implementation/roadmap.md` → M10 Increment 5,
//! T3; `design/validation.md` → blocking-on-dangling).
//!
//! The full loop: a `plan` task authors + commits a `spec`; the spec's `criteria` block
//! is given a `maps-to-test` `code-anchor` via the **editable channel** (raw git edit +
//! commit) **before** the work task is minted (the spiked pin-to-base constraint — the
//! resume re-compose pins to base, so committing after mint advances HEAD and would
//! block). Then an `implement-from-spec` task **binds** that spec (the anchor enters the
//! task's effective state via the read role), **creates** an `adr` whose `cites-code`
//! header anchor is authored in-CLI via `jigc doc set-field`, and `finalize` runs
//! `doc-code` over both anchors. Two anchor shapes are exercised:
//!
//! - **header-field anchor** — `adr:<slug>#cites-code` (a created working-delta doc), and
//! - **repeatable-block-leaf anchor** — `spec:<slug>#criteria/<id>/maps-to-test` (a bound
//!   committed-store doc).
//!
//! **The passing walk** — both the cited symbol and the criterion's test exist → finalize
//! lands exactly ONE commit and the report carries no `doc-code` block. A *resolving*
//! check emits no finding (the engine emits findings only for violations; there is no
//! "checks-ran" rendering — the `✓ doc-code …` lines in worked-examples.md are illustrative
//! notation), so the masking guard (hardening #5 / acceptance bar clause 1 — a silently
//! skipped enumeration must not pass for the wrong reason) is satisfied by the PER-ANCHOR
//! CONTRAST: this passing walk vs. the two blocking walks below, each flipping ONLY one
//! anchor to dangling over the SAME fixture. A skipped enumeration would land the commit
//! in both arms; the contrast isolates each anchor-bearing doc.
//!
//! **The blocking walk** — a dangling anchor (the symbol renamed away / the test deleted)
//! → finalize blocks non-zero, the finding names the dangling target + a route, and
//! `git rev-list --count HEAD` is unchanged (no commit). Both anchor shapes are driven
//! through the blocking case (acceptance bar clause 1 + 4).
//!
//! No external test crates: the `jigc` path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init`, the `doc-code` probe is the real one, resolved through the
//! production path (the `JIGC_DOC_CODE_PROBE` override removed), and self-cleaning
//! `TempDir`s keep the developer's repo clean.

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
            "jigc-flow13-{tag}-{}-{:?}",
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

/// HEAD commit count — the no-commit witness the blocking walk asserts is unchanged.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer, plus a
/// `tests/` tree carrying a real `#[test]` fn (the `maps-to-test` target) and a `src/`
/// tree carrying a real symbol (the `cites-code` target).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("tests")).expect("mk tests");
    fs::create_dir_all(repo.join("src")).expect("mk src");
    // The criterion's `maps-to-test` target — a real `#[test]`-attributed fn in the
    // DOMINANT Rust unit-test layout: nested inside `#[cfg(test)] mod tests { … }` (not a
    // rare top-level fn), so the acceptance proves the real case the resolver must descend
    // into, not the layout a top-level-only walk happens to handle.
    fs::write(
        repo.join("tests").join("rate_limit.rs"),
        "pub fn limit() -> u32 {\n    100\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn burst_rejected() {\n        assert_eq!(limit(), 100);\n    }\n}\n",
    )
    .expect("write test fixture");
    // The adr's `cites-code` target — a real symbol nested as an `impl` method (also
    // invisible to a top-level-only walk), so the header anchor exercises impl-method
    // resolution rather than the top-level struct.
    fs::write(
        repo.join("src").join("limiter.rs"),
        "pub struct TokenBucket {\n    capacity: u32,\n}\n\nimpl TokenBucket {\n    pub fn refill(&mut self) {\n        self.capacity += 1;\n    }\n}\n",
    )
    .expect("write src fixture");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and **no `JIGC_DOC_CODE_PROBE`
/// override** — so the `doc-code` probe resolves through the **production default** path,
/// what a real install hits. `env_remove` guards against an env var leaking in from the
/// test runner. Captures output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
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

/// The slug `id-from: title` derives for the spec authored by the plan task.
const SPEC_SLUG: &str = "gateway-rate-limiting";
/// The criterion's anchor id (the literal `{#id}` token seeded via the editable channel).
const CRITERION_ID: &str = "burst-limit";
/// The repeatable-block-leaf anchor's `<path>#<test>` value — a real `#[test]` fn nested
/// inside `#[cfg(test)] mod tests` (the dominant layout the resolver must descend into).
const MAPS_TO_TEST: &str = "tests/rate_limit.rs#burst_rejected";
/// The header-field anchor's `<path>#<symbol>` value — a real symbol nested as an `impl`
/// method (also invisible to a top-level-only walk).
const CITES_CODE: &str = "src/limiter.rs#refill";

/// Author + commit a `spec` via the `plan` workflow, then seed its `criteria` block with a
/// criterion carrying `maps-to-test: <anchor>` through the **editable channel** —
/// committed on the base branch BEFORE the work task is minted (the pin-to-base
/// constraint). After this, `docs/specs/<slug>.md` is committed with one criterion whose
/// `maps-to-test` leaf is the repeatable-block-leaf `code-anchor`.
fn commit_spec_with_maps_to_test(repo: &Path, home: &Path, anchor: &str) {
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "plan", "draft the rate limit spec"],
    );
    assert_ok(&out, "`jigc start --workflow plan`");
    let plan_task = "draft-the-rate-limit-spec";

    let create = jigc_doc(
        repo,
        home,
        &["create", "spec", "--title", "Gateway rate limiting"],
        None,
    );
    assert_ok(&create, "`jigc doc create spec`");
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

    set_field(repo, home, &format!("commit:{plan_task}#type"), "docs");
    set_field(repo, home, &format!("commit:{plan_task}#scope"), "gateway");
    set_slot(
        repo,
        home,
        &format!("commit:{plan_task}#summary"),
        b"draft the rate limit spec\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{plan_task}#body"),
        b"Capture the rate-limit criteria before coding.\n",
    );

    let out = jigc(repo, home, &["task", "finalize", plan_task]);
    assert_ok(&out, "`jigc task finalize` (plan task)");

    // The spec is committed at its canonical path (the persisted differentiator).
    let committed = Command::new("git")
        .args(["show", &format!("HEAD:docs/specs/{SPEC_SLUG}.md")])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "the plan task must commit docs/specs/{SPEC_SLUG}.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );

    // EDITABLE CHANNEL: append a criterion carrying a `maps-to-test` code-anchor, then
    // git-commit it on the base branch BEFORE the work task mints. The criterion block
    // shape mirrors the parser's fixture: `### <title>  {#id}` + statement, a
    // `<!-- fields -->` marker, then `- maps-to-test: <path>#<test>`.
    let spec_path = repo
        .join("docs")
        .join("specs")
        .join(format!("{SPEC_SLUG}.md"));
    let mut body = fs::read_to_string(&spec_path).expect("read committed spec");
    body.push_str(&format!(
        "\n### Burst limit  {{#{CRITERION_ID}}}\n\nRequests beyond 100/min are rejected.\n\n<!-- fields -->\n- maps-to-test: {anchor}\n"
    ));
    fs::write(&spec_path, &body).expect("seed criterion with maps-to-test anchor");
    git(repo, &["add", &format!("docs/specs/{SPEC_SLUG}.md")]);
    git(
        repo,
        &[
            "commit",
            "-q",
            "-m",
            "seed criterion with maps-to-test anchor",
        ],
    );
}

/// Mint an `implement-from-spec` work task on the spec-bearing HEAD, bind the committed
/// spec into its read role (the bound `maps-to-test` enters effective state), then create
/// and author an `adr` whose `cites-code` header anchor is set in-CLI. Returns the work
/// task id and the adr slug. The two `code-anchor` values are parameters so the passing
/// and blocking walks differ only in whether the targets resolve.
fn mint_bind_and_author_adr(
    repo: &Path,
    home: &Path,
    intent: &str,
    task: &str,
    cites_code: &str,
) -> String {
    let mint = jigc(
        repo,
        home,
        &["start", "--workflow", "implement-from-spec", intent],
    );
    assert_ok(&mint, "`jigc start --workflow implement-from-spec`");

    // Bind the spec — the committed spec (with its `maps-to-test` anchor) enters the
    // task's effective state as the `spec` read role.
    let bind = jigc(
        repo,
        home,
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), task],
    );
    assert_ok(&bind, "`jigc task bind spec`");

    // Re-compose so the workflow re-reads the bound spec (the resume step).
    let resume = jigc(repo, home, &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose");

    // Create the adr in-task via the create-gate (a working delta), author its required
    // prose slots, and set its `cites-code` header anchor in-CLI via `doc set-field`.
    let create = jigc_doc(
        repo,
        home,
        &[
            "create",
            "adr",
            "--title",
            "Token-bucket limiter",
            "--task",
            task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    let slug = adr
        .strip_prefix("adr:")
        .expect("created adr address")
        .to_string();

    for (section, prose) in [
        ("context", b"Forces at play.\n".as_slice()),
        ("decision", b"We chose a token bucket.\n".as_slice()),
        ("consequences", b"Tradeoffs accepted.\n".as_slice()),
    ] {
        let out = jigc_doc(
            repo,
            home,
            &[
                "set-slot",
                &format!("adr:{slug}#{section}"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot adr:{slug}#{section}"));
    }

    // The header-field `code-anchor` — authored in-CLI (the `set-field` wiring of inc-1).
    let out = jigc_doc(
        repo,
        home,
        &[
            "set-field",
            &format!("adr:{slug}#status/cites-code"),
            "--value",
            cites_code,
            "--task",
            task,
        ],
        None,
    );
    assert_ok(&out, &format!("set-field adr:{slug}#status/cites-code"));

    // Author the commit doc so the only blocking lever is `doc-code`.
    set_field(
        repo,
        home,
        &format!("commit:{task}#implements"),
        &format!("spec:{SPEC_SLUG}"),
    );
    set_field(repo, home, &format!("commit:{task}#type"), "feat");
    set_field(repo, home, &format!("commit:{task}#scope"), "gateway");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"enforce the gateway rate limit\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Bound per-client volume at the edge.\n",
    );

    // A real code change so the empty-commit guard is satisfied by more than the doc.
    fs::write(repo.join("limiter.txt"), "rate limiter\n").expect("write code change");
    git(repo, &["add", "limiter.txt"]);

    slug
}

/// The passing walk — both anchors resolve, finalize lands ONE commit, and the report
/// carries BOTH `doc-code` findings *resolving* (the masking guard, acceptance bar
/// clause 1). The two anchor shapes (`symbol-exists` header field on the created adr,
/// `criterion-maps-to-test` repeatable-block leaf on the bound spec) are both exercised.
#[test]
fn flow13_passing_walk_resolves_both_anchors_and_lands_one_commit() {
    let repo = TempDir::new("pass-repo");
    let home = TempDir::new("pass-home");
    init_repo(repo.path());

    // Seed the spec + its `maps-to-test` anchor (pointing at the real `#[test]` fn) on
    // the base branch BEFORE the work task mints.
    commit_spec_with_maps_to_test(repo.path(), home.path(), MAPS_TO_TEST);

    let task = "enforce-the-rate-limit";
    let slug = mint_bind_and_author_adr(
        repo.path(),
        home.path(),
        "enforce the rate limit at the gateway",
        task,
        // The `cites-code` anchor points at the real symbol — it resolves.
        CITES_CODE,
    );

    let before = head_count(repo.path());
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // finalize PASSES — both anchors resolve, so doc-code blocks nothing.
    assert_ok(
        &out,
        &format!("finalize must pass when both anchors resolve; got:\n{rendered}"),
    );
    let after = head_count(repo.path());
    assert_eq!(
        after,
        before + 1,
        "the passing walk must land exactly ONE commit"
    );

    // The masking guard (acceptance bar clause 1, increment-workflow hardening #5).
    //
    // A *resolving* doc-code check emits NO finding — the engine emits findings only for
    // violations, and `validate`/`finalize` render only findings, so a clean report is
    // empty (there is no "checks-ran" surface to read a resolving finding off — the
    // illustrative `✓ doc-code …` lines in worked-examples.md are notation, not an
    // implemented rendering; the actual report carries the *blocking* case, the proof).
    // The masking failure to guard against is therefore: the target-surface enumeration
    // silently skipped a doc, so finalize passes for the wrong reason ("doc-code never
    // ran") rather than the right one ("doc-code ran and both anchors resolved"). The two
    // are indistinguishable from the passing report alone.
    //
    // The implementation-faithful witness is the PER-ANCHOR CONTRAST against this exact
    // composing context: holding the OTHER anchor resolved, flipping ONLY the target
    // anchor to dangling must flip finalize from pass→block. A silently-skipped
    // enumeration would land the commit in BOTH the resolved and the dangling case. The
    // two `flow13_blocking_walk_*` tests below run that contrast over the SAME
    // `commit_spec_with_maps_to_test` + `mint_bind_and_author_adr` fixture — one flips
    // only `maps-to-test` (the bound-spec repeatable-leaf anchor), one flips only
    // `cites-code` (the created-adr header anchor) — proving the enumeration reached each
    // anchor-bearing doc independently. This passing walk + that pair is the masking
    // guard: a zero-`doc-code`-effect run (the silent skip) fails the pair as hard as a
    // missed block. The negative below confirms this passing run carries no doc-code
    // *block* (the contrast's pass arm), so the pair's block arm is unambiguous.
    assert!(
        !rendered.contains("doc-code"),
        "the passing walk resolves both anchors, so the report must carry no doc-code \
         block (the resolved arm of the masking-guard contrast); got:\n{rendered}",
    );

    // The two anchors really did resolve (the targets exist in the working tree).
    let message = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert!(
        message.starts_with("feat"),
        "the landed commit carries the rendered `feat` type; got:\n{message}",
    );
    // The created adr was promoted to its canonical store path as part of the one commit.
    let promoted = Command::new("git")
        .args(["show", &format!("HEAD:docs/decisions/{slug}.md")])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        promoted.status.success(),
        "the created adr must be promoted to docs/decisions/{slug}.md in the landed commit",
    );
}

/// The passing walk through the **production default probe resolution** — finalize must
/// resolve `doc-code` as a sibling of the `jigc` binary (`<bin-dir>/doc-code`) with **no**
/// `JIGC_DOC_CODE_PROBE` override, proving a normal `cargo build` leaves a probe at the
/// production-resolved location. On a tree where the probe is missing,
/// `Command::spawn` errors and the engine raises a floor-locked
/// `pack-probe-integrity.crash` meta-finding, blocking finalize permanently — the failure
/// this guards against. The passing walk (both anchors resolve) must land exactly ONE
/// commit, promote the adr, and carry NO `pack-probe-integrity` block.
#[test]
fn flow13_passing_walk_resolves_via_production_default_probe_path() {
    let repo = TempDir::new("prod-pass-repo");
    let home = TempDir::new("prod-pass-home");
    init_repo(repo.path());

    commit_spec_with_maps_to_test(repo.path(), home.path(), MAPS_TO_TEST);

    let task = "enforce-the-rate-limit";
    let slug = mint_bind_and_author_adr(
        repo.path(),
        home.path(),
        "enforce the rate limit at the gateway",
        task,
        CITES_CODE,
    );

    let before = head_count(repo.path());
    // The probe-bearing finalize runs through production resolution — no env override.
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // A missing probe would surface here as a floor-locked crash meta-finding.
    assert!(
        !rendered.contains("pack-probe-integrity"),
        "production resolution must find a runnable `doc-code` sibling — a missing probe \
         raises a floor-locked pack-probe-integrity meta-finding; got:\n{rendered}",
    );
    assert_ok(
        &out,
        &format!("finalize via production probe resolution must pass; got:\n{rendered}"),
    );
    let after = head_count(repo.path());
    assert_eq!(
        after,
        before + 1,
        "the production-resolution passing walk must land exactly ONE commit"
    );

    // The probe really ran and both anchors resolved — no doc-code block in the report.
    assert!(
        !rendered.contains("doc-code"),
        "both anchors resolve, so the report must carry no doc-code block; got:\n{rendered}",
    );
    // The created adr was promoted as part of the one commit.
    let promoted = Command::new("git")
        .args(["show", &format!("HEAD:docs/decisions/{slug}.md")])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        promoted.status.success(),
        "the created adr must be promoted to docs/decisions/{slug}.md in the landed commit",
    );
}

/// The blocking walk — a dangling `maps-to-test` anchor (the test renamed away) blocks
/// finalize: non-zero exit, the finding names the dangling target + a route, and no
/// commit lands. Exercises the **repeatable-block-leaf** anchor shape.
#[test]
fn flow13_blocking_walk_dangling_maps_to_test_blocks_finalize() {
    let repo = TempDir::new("block-test-repo");
    let home = TempDir::new("block-test-home");
    init_repo(repo.path());

    // Seed the spec with a `maps-to-test` anchor pointing at a test that does NOT exist
    // (the agent renamed it away / never wrote it) — the dangling repeatable-leaf anchor.
    commit_spec_with_maps_to_test(
        repo.path(),
        home.path(),
        "tests/rate_limit.rs#burst_rejected_renamed",
    );

    let task = "enforce-the-rate-limit";
    let _slug = mint_bind_and_author_adr(
        repo.path(),
        home.path(),
        "enforce the rate limit at the gateway",
        task,
        // The `cites-code` anchor resolves — only the `maps-to-test` one dangles, so the
        // block is unambiguously the criterion anchor.
        CITES_CODE,
    );

    let before = head_count(repo.path());
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // finalize BLOCKS — the dangling criterion anchor makes the exit non-zero.
    assert!(
        !out.status.success(),
        "a dangling maps-to-test must make finalize exit non-zero; got:\n{rendered}",
    );
    // The block is the criterion-maps-to-test check, naming the dangling target.
    assert!(
        rendered.contains("doc-code.criterion-maps-to-test"),
        "the block must be the criterion-maps-to-test check; got:\n{rendered}",
    );
    assert!(
        rendered.contains("burst_rejected_renamed"),
        "the block must name the dangling anchor target; got:\n{rendered}",
    );
    // The block names the dangling target + a route: the message identifies the absent
    // symbol and the file it is absent from (the route to fix — add the test / re-point
    // the anchor / drop it).
    assert!(
        rendered.contains("resolves to no") && rendered.contains("tests/rate_limit.rs"),
        "the block must name the dangling target's file + how it fails to resolve \
         (the route); got:\n{rendered}",
    );

    // No commit lands — the dangling block created nothing.
    let after = head_count(repo.path());
    assert_eq!(
        before, after,
        "a dangling-anchor block must create no commit"
    );
}

/// The blocking walk, symmetrically over the **header-field** anchor shape — a dangling
/// `cites-code` anchor (the cited symbol renamed away) blocks finalize: non-zero exit,
/// the finding names the dangling target, and no commit lands.
#[test]
fn flow13_blocking_walk_dangling_cites_code_blocks_finalize() {
    let repo = TempDir::new("block-sym-repo");
    let home = TempDir::new("block-sym-home");
    init_repo(repo.path());

    // The criterion's `maps-to-test` resolves — only the adr's `cites-code` dangles.
    commit_spec_with_maps_to_test(repo.path(), home.path(), MAPS_TO_TEST);

    let task = "enforce-the-rate-limit";
    let _slug = mint_bind_and_author_adr(
        repo.path(),
        home.path(),
        "enforce the rate limit at the gateway",
        task,
        // The cited symbol was renamed away — the dangling header-field anchor.
        "src/limiter.rs#TokenBucketRenamed",
    );

    let before = head_count(repo.path());
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // finalize BLOCKS — the dangling symbol anchor makes the exit non-zero.
    assert!(
        !out.status.success(),
        "a dangling cites-code must make finalize exit non-zero; got:\n{rendered}",
    );
    // The block is the symbol-exists check, naming the dangling target.
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block must be the symbol-exists check; got:\n{rendered}",
    );
    assert!(
        rendered.contains("TokenBucketRenamed"),
        "the block must name the dangling anchor target; got:\n{rendered}",
    );

    // No commit lands — the dangling block created nothing.
    let after = head_count(repo.path());
    assert_eq!(
        before, after,
        "a dangling-anchor block must create no commit"
    );
}
