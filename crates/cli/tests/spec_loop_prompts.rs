//! The spec loop's prompts — `step:locate-from-spec` (M42 Increment 11, T4;
//! `implementation/decisions-pending.md` → P5; `DECISIONS.md` → 2026-07-13 M42 Settle,
//! fork 4 "prose demand only" + the W8 cheap half).
//!
//! Three defects in one step, all agent-facing prose, all driven here through the REAL
//! binary — the composed bytes an agent reads, never a reconstruction:
//!
//! - **The `maps-to-test` demand.** `spec.criteria/<id>/maps-to-test` is writable, it
//!   round-trips, and its `doc-code.criterion-maps-to-test` check is live and blocking
//!   (`design/validation.md` → the `doc-code` probe) — yet the field has **zero writes
//!   ever**, because the only agent-facing pack text that named it said *"leave it off
//!   the item entirely"* (`author-migration-spec.yaml`, correct in its migration
//!   context) and the step that reads a spec's criteria never named it at all. The
//!   demand's home is `step:locate-from-spec` — the step **only** `implement-from-spec`
//!   includes — not the shared `step:implement`, whose other two workflows bind no spec.
//! - **The state-blind caption.** `"The spec's criteria — empty until you bind a spec and
//!   re-compose"` is a falsehood the moment a spec *is* bound, which is the only state in
//!   which the slice below it has anything to say. Conditionals stay barred by the dialect
//!   invariant, so the repair is to state the **address**, never the state — and the
//!   caption must therefore be true in *both* compositions (unbound and bound), which is
//!   what this suite asserts.
//! - **The open-decisions instruction (W8, cheap half).** `implement-from-spec` grants
//!   `{type: adr, as: decision}`, so a decision the spec left open and the task settled
//!   has a live recording route; the step never named it.
//!
//! The second test runs the WHOLE ARM and moves the zero-writes-ever counter: it extracts
//! the **emitted** `set-field …/maps-to-test` line from the composed body, fills only the
//! agent-fill `<…>` placeholders an agent would fill, runs those bytes verbatim, finalizes,
//! and reads the **committed** spec back out of git — the anchor is promoted onto a
//! `reads`-bound committed doc.

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
            "jigc-spec-loop-{tag}-{}-{:?}",
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

/// The slug `id-from: title` derives for the spec committed by the setup task.
const SPEC_SLUG: &str = "gateway-rate-limiting";
/// The criterion's title — its `{#id}` anchor is read back out of the composed slice.
const CRITERION_TITLE: &str = "Rejects the 101st request";

/// Setup — a `plan` task creates, authors (goal + context + one `criteria` item), and
/// finalizes `spec:gateway-rate-limiting`, so the committed store carries one spec with
/// one criterion for `implement-from-spec` to bind and read.
fn commit_spec_with_criterion(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "plan", "draft the rate limit spec"],
    );
    assert_ok(&out, "`jigc start --workflow plan` (setup task)");
    let task = "draft-the-rate-limit-spec";

    let create = jigc_doc(
        repo,
        home,
        &["create", "spec", "--title", "Gateway rate limiting"],
        None,
    );
    assert_ok(&create, "`jigc doc create spec`");

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

    let add = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            &format!("spec:{SPEC_SLUG}#criteria"),
            "--title",
            CRITERION_TITLE,
        ],
        None,
    );
    assert_ok(&add, "`jigc doc add-item spec:…#criteria`");
    let item_addr = String::from_utf8(add.stdout)
        .expect("utf-8")
        .trim()
        .to_owned();
    set_slot(
        repo,
        home,
        &format!("{item_addr}/statement"),
        b"The gateway rejects the 101st request in a rolling 60s window.\n",
    );

    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"draft the rate limit spec\n",
    );

    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize` (setup task)");
}

/// Mint an `implement-from-spec` task, returning its composed body.
fn mint_implement_from_spec(repo: &Path, home: &Path, intent: &str) -> String {
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "implement-from-spec", intent],
    );
    assert_ok(&out, "`jigc start --workflow implement-from-spec`");
    String::from_utf8(out.stdout).expect("utf-8")
}

/// The composed step prose an agent reads — asserted on BOTH compositions of the same
/// step: the unbound first pass and the bound re-compose. The step is included by exactly
/// one workflow, so there is no gate-blocked context to dodge; the two *states* are the
/// contexts, and a caption that lies in either is the defect (conditionals stay barred by
/// the dialect invariant — the repair states the address, never the state).
#[test]
fn compose_demands_maps_to_test_and_the_adr_route_and_never_lies_about_the_slice() {
    let repo = TempDir::new("compose");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_spec_with_criterion(repo.path(), home.path());

    let task = "enforce-the-rate-limit";
    let unbound = mint_implement_from_spec(repo.path(), home.path(), "enforce the rate limit");

    let bind = jigc(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), task],
    );
    assert_ok(&bind, "`jigc task bind spec`");
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose");
    let bound = String::from_utf8(resume.stdout).expect("utf-8");

    for (state, composed) in [("unbound", &unbound), ("bound", &bound)] {
        // (a) The `maps-to-test` demand — RED today: the composed body contains ZERO
        //     occurrences of the field name, which is why it has zero writes ever.
        assert!(
            composed.contains("maps-to-test"),
            "the {state} compose must name `maps-to-test`; got:\n{composed}",
        );
        // It must be demanded as a WRITE, on the item address the criteria slice anchors.
        assert!(
            composed.contains("jigc doc set-field spec:<slug>#criteria/<id>/maps-to-test"),
            "the {state} compose must emit the `set-field …/maps-to-test` write line; \
             got:\n{composed}",
        );

        // (b) The W8 open-decisions instruction routes through the granted `adr` gate.
        //     Keyed on THIS step's own instruction — `step:implement` already emits a
        //     `create adr` line, so asserting the route alone would pass on prose that
        //     never demands the open decision be recorded.
        assert!(
            composed.contains("leave a decision open"),
            "the {state} compose must instruct recording a settled open decision; \
             got:\n{composed}",
        );
        assert!(
            composed.contains("jigc doc create adr"),
            "the {state} compose must name the adr recording route this workflow grants; \
             got:\n{composed}",
        );

        // (c) The state-blind caption is gone — it is a falsehood in the bound state, and
        //     the repair must not simply move the lie into the unbound one.
        assert!(
            !composed.contains("empty until you bind"),
            "the {state} compose must carry no state-blind caption over the criteria slice; \
             got:\n{composed}",
        );
    }

    // The bound re-compose still dereferences the slice (the caption repair is prose-only
    // — it must not disturb what the placeholder resolves to).
    assert!(
        bound.contains(&format!("> ### {CRITERION_TITLE}")),
        "the bound compose must still slice the committed spec's criteria; got:\n{bound}",
    );
}

/// The `maps-to-test` truth surface (M45 Inc 10, T5; fork 9 prose rider + the lifecycle
/// stated-at). The composed step prose must (a) stop implying a *named* `<test-fn>` is
/// mandatory — a closure-based test framework registers tests with no named symbol, so a
/// file-only `<path>` anchor is accepted (it resolves on the file's presence, the
/// `let symbol = symbol?` short-circuit in the `doc-code` probe) — and (b) state the
/// resolution/lifecycle: the anchor is repo-relative, resolves against the **staged index**,
/// and is re-resolved at finalize by the blocking `doc-code.criterion-maps-to-test` check.
/// Driven through the REAL binary on BOTH compositions (unbound + bound), since the prose is
/// unconditional step text and must be true — and present — in either state.
#[test]
fn compose_states_the_maps_to_test_file_only_fallback_and_lifecycle() {
    let repo = TempDir::new("mtt");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_spec_with_criterion(repo.path(), home.path());

    let task = "enforce-the-rate-limit";
    let unbound = mint_implement_from_spec(repo.path(), home.path(), "enforce the rate limit");

    let bind = jigc(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), task],
    );
    assert_ok(&bind, "`jigc task bind spec`");
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose");
    let bound = String::from_utf8(resume.stdout).expect("utf-8");

    for (state, composed) in [("unbound", &unbound), ("bound", &bound)] {
        // (a) The hint no longer implies a named `<test-fn>` is mandatory: the closure-based
        //     framework case is named, and the file-only fallback (`<path>` alone) is stated.
        assert!(
            composed.to_lowercase().contains("closure"),
            "the {state} compose must name the closure-based framework case; got:\n{composed}",
        );
        assert!(
            composed.contains("file-only"),
            "the {state} compose must name the file-only anchor fallback; got:\n{composed}",
        );

        // (b) The resolution/lifecycle: staged-index gating + the named blocking check at
        //     finalize.
        assert!(
            composed.contains("staged index"),
            "the {state} compose must state the anchor resolves against the staged index; \
             got:\n{composed}",
        );
        assert!(
            composed.contains("doc-code.criterion-maps-to-test"),
            "the {state} compose must name the blocking check that re-resolves the anchor at \
             finalize; got:\n{composed}",
        );
    }
}

/// The whole arm, driving the EMITTED bytes: the composed `set-field …/maps-to-test` line
/// is extracted from the composed body, its agent-fill `<…>` placeholders filled exactly
/// as an agent fills them (the slug it bound, the id the `{#id}` anchor printed, the test
/// it wrote), and run verbatim — then finalize, then the COMMITTED spec is read back out
/// of git carrying the anchor. This is the write the counter never saw.
#[test]
fn the_emitted_maps_to_test_line_wires_a_criterion_to_its_test_through_finalize() {
    let repo = TempDir::new("arc");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_spec_with_criterion(repo.path(), home.path());

    let task = "enforce-the-rate-limit";
    mint_implement_from_spec(repo.path(), home.path(), "enforce the rate limit");
    let bind = jigc(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), task],
    );
    assert_ok(&bind, "`jigc task bind spec`");
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose");
    let composed = String::from_utf8(resume.stdout).expect("utf-8");

    // The id the agent is told to use: the `{#id}` anchor rendered in the criteria slice.
    let anchor_line = composed
        .lines()
        .find(|line| line.contains(CRITERION_TITLE) && line.contains("{#"))
        .unwrap_or_else(|| {
            panic!("the criteria slice must anchor the criterion; got:\n{composed}")
        });
    let item_id = anchor_line
        .split_once("{#")
        .and_then(|(_, rest)| rest.split_once('}'))
        .map(|(id, _)| id.to_owned())
        .expect("the anchor carries an id");

    // The test the criterion maps to — a real `#[test]` fn, which the blocking
    // `doc-code.criterion-maps-to-test` check resolves at the gate.
    fs::create_dir_all(repo.path().join("src")).expect("create src/");
    fs::write(
        repo.path().join("src").join("limiter.rs"),
        "pub fn burst() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn burst_is_capped() {}\n}\n",
    )
    .expect("write the test");
    git(repo.path(), &["add", "src/limiter.rs"]);

    // The EMITTED line, run verbatim — only the agent-fill `<…>` placeholders are filled.
    // Everything else (the address grammar, the `--task <id>` interpolation) is the pack's
    // own bytes; a broken emission fails here rather than passing against a hand-built one.
    let emitted = composed
        .lines()
        .find(|line| {
            line.trim_start().starts_with("jigc doc set-field") && line.contains("maps-to-test")
        })
        .unwrap_or_else(|| {
            panic!("the compose must emit a `maps-to-test` write line; got:\n{composed}")
        })
        .trim()
        .to_owned();
    let filled = emitted
        .replace("<slug>", SPEC_SLUG)
        .replace("<id>", &item_id)
        .replace("<path>#<test-fn>", "src/limiter.rs#burst_is_capped");
    assert!(
        !filled.contains('<') && !filled.contains('>'),
        "every agent-fill placeholder in the emitted line must be filled; got:\n{filled}",
    );
    let args: Vec<&str> = filled.split_whitespace().collect();
    assert_eq!(args[0], "jigc", "the emitted line must invoke jigc");
    assert!(
        filled.contains(&format!("--task {task}")),
        "the emitted line must carry the resolved task id; got:\n{filled}",
    );
    let write = jigc(repo.path(), home.path(), &args[1..]);
    assert_ok(
        &write,
        "the EMITTED `set-field …/maps-to-test` line, run verbatim",
    );

    // Finalize: the anchor resolves (a real test), so the gate passes and the spec — a
    // `reads`-bound COMMITTED doc — is promoted carrying the wiring.
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "feat",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"enforce the gateway rate limit\n",
    );
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(
        &out,
        "`jigc task finalize` — the wired anchor resolves to a real test",
    );

    let committed = git(
        repo.path(),
        &["show", &format!("HEAD:docs/specs/{SPEC_SLUG}.md")],
    );
    assert!(
        committed.contains("- maps-to-test: src/limiter.rs#burst_is_capped"),
        "the COMMITTED spec must carry the criterion's test anchor; got:\n{committed}",
    );
}
