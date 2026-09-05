//! Flow 13 acceptance, T4 — the **contract is satisfied** (the `pack-probe-integrity.*`
//! meta-findings fire) and **severity is cascade-tunable** (a tunable `doc-code` check
//! demotes; the intrinsic meta-finding does not). This closes flow 13's acceptance bar
//! clauses **2** (the contract is satisfied, not waived) and **5** (severity tunes
//! through the cascade) — the last deferred differentiator (`design/worked-examples.md`
//! → flow 13, clauses 2 + 5; `design/validation.md` → Failure semantics / the two-tier
//! rule + the intrinsic floor-lock; `DECISIONS.md` 2026-06-06, M10 inc-3 / T4 floor-lock).
//!
//! The deliberate adversarial probe **stubs** live here and ONLY here (T3 drives the real
//! tree-sitter probe). The CLI selects the probe program via `JIGC_DOC_CODE_PROBE` (the
//! production override env-var, `JIGC_PACK_DIR` precedent), so a test points it at a stub
//! that **sleeps past the wall-clock budget** or **emits non-JSON** to exercise the
//! subprocess-boundary failure modes the determinism contract enforces:
//!
//! - **timeout + floor-lock** — a stub that sleeps past the invoker's budget blocks
//!   `finalize` with an intrinsic-blocking `pack-probe-integrity.timeout` meta-finding;
//!   AND a project `scalar-set` `validation.pack-probe-integrity.timeout.severity:
//!   advisory` is **soft-rejected at cascade resolution** — the meta-finding **stays
//!   blocking** (the floor-lock), so the demotion attempt cannot waive the contract.
//!   One test carries both: proving "stays blocking *with* the advisory delta recorded"
//!   is strictly stronger than "fires", so the firing is proven by the same run.
//! - **malformed-output** — a stub that exits 0 but writes non-JSON to stdout blocks
//!   `finalize` with an intrinsic-blocking `pack-probe-integrity.malformed-output`
//!   meta-finding (the contract is satisfied, not silently passed).
//! - **severity demotion (REAL probe)** — a project `scalar-set`
//!   `validation.doc-code.criterion-maps-to-test.severity: warning` makes a **dangling**
//!   `maps-to-test` criterion anchor **surface as a warning** (visible at `task validate`)
//!   but **NOT block** — `finalize` lands the commit (the M6 post-pass; no code change),
//!   while the `adr.cites-code` `symbol-exists` anchor (left resolving) and the intrinsic
//!   meta-findings stay floor-locked. This is the tunable half of the two-tier rule.
//!
//! No external test crates: the `jigc` path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, the real `doc-code` probe is built from the pack, the stubs
//! are compiled with `rustc` (real processes — the `probe_invoker.rs` precedent), and
//! self-cleaning `TempDir`s keep the developer's repo clean.

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
            "jigc-flow13-t4-{tag}-{}-{:?}",
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

/// Build the pack's **real** `doc-code` probe once (process-wide) and return its binary
/// path — the tree-sitter subprocess the severity-demotion walk drives (the meta-finding
/// walks point `JIGC_DOC_CODE_PROBE` at a stub instead).
fn real_doc_code_probe() -> &'static Path {
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

/// Compile `source` (a tiny Rust program) into an executable at `<dir>/<name>` and return
/// its path — a **real** adversarial stub probe (a separate process the invoker drives,
/// the `probe_invoker.rs` precedent), not an in-test mock.
fn build_stub(dir: &Path, name: &str, source: &str) -> PathBuf {
    let src = dir.join(format!("{name}.rs"));
    fs::write(&src, source).expect("write stub source");
    let bin = dir.join(name);
    let out = Command::new("rustc")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .arg("--edition")
        .arg("2021")
        .output()
        .expect("invoke rustc");
    assert!(
        out.status.success(),
        "rustc failed to build stub `{name}`:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    bin
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

/// HEAD commit count — the no-commit witness the blocking walks assert is unchanged.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer, plus a
/// `tests/` tree carrying a real `#[test]` fn (the `maps-to-test` target) and a `src/`
/// tree carrying a real symbol (the `cites-code` target). Mirrors `flow13_acceptance.rs`.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("tests")).expect("mk tests");
    fs::create_dir_all(repo.join("src")).expect("mk src");
    fs::write(
        repo.join("tests").join("rate_limit.rs"),
        "#[test]\nfn burst_rejected() {\n    assert!(true);\n}\n",
    )
    .expect("write test fixture");
    fs::write(
        repo.join("src").join("limiter.rs"),
        "pub struct TokenBucket {\n    capacity: u32,\n}\n",
    )
    .expect("write src fixture");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and `JIGC_DOC_CODE_PROBE` pointed
/// at `probe` (the real probe or an adversarial stub), capturing output.
fn jigc(repo: &Path, home: &Path, probe: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", probe)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, with `JIGC_DOC_CODE_PROBE = probe`.
fn jigc_doc(
    repo: &Path,
    home: &Path,
    probe: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", probe);
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

/// The combined stdout+stderr of a `jigc` invocation (findings render to either).
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Stage a slot from `prose` for `addr` (optionally scoped to `task`), asserting success.
fn set_slot(repo: &Path, home: &Path, probe: &Path, addr: &str, task: Option<&str>, prose: &[u8]) {
    let mut args = vec!["set-slot", addr, "--from-file", "-"];
    if let Some(task) = task {
        args.push("--task");
        args.push(task);
    }
    let out = jigc_doc(repo, home, probe, &args, Some(prose));
    assert_ok(&out, &format!("set-slot {addr}"));
}

/// Stage a field `value` for `addr` (optionally scoped to `task`), asserting success.
fn set_field(repo: &Path, home: &Path, probe: &Path, addr: &str, task: Option<&str>, value: &str) {
    let mut args = vec!["set-field", addr, "--value", value];
    if let Some(task) = task {
        args.push("--task");
        args.push(task);
    }
    let out = jigc_doc(repo, home, probe, &args, None);
    assert_ok(&out, &format!("set-field {addr}"));
}

/// The slug `id-from: title` derives for the spec authored by the plan task.
const SPEC_SLUG: &str = "gateway-rate-limiting";
/// The criterion's anchor id (the literal `{#id}` token seeded via the editable channel).
const CRITERION_ID: &str = "burst-limit";
/// The repeatable-block-leaf anchor's `<path>#<test>` value — a real `#[test]` fn.
const MAPS_TO_TEST: &str = "tests/rate_limit.rs#burst_rejected";
/// The header-field anchor's `<path>#<symbol>` value — a real top-level symbol.
const CITES_CODE: &str = "src/limiter.rs#TokenBucket";

/// Author + commit a `spec` via the `plan` workflow, then seed its `criteria` block with a
/// criterion carrying `maps-to-test: <anchor>` through the **editable channel** — committed
/// on the base branch BEFORE the work task is minted (the pin-to-base constraint). Mirrors
/// `flow13_acceptance.rs`'s `commit_spec_with_maps_to_test` (the same fixture the T3 walks
/// drive, so the demotion walk lands on the identical surface).
fn commit_spec_with_maps_to_test(repo: &Path, home: &Path, probe: &Path, anchor: &str) {
    let out = jigc(
        repo,
        home,
        probe,
        &["start", "--workflow", "plan", "draft the rate limit spec"],
    );
    assert_ok(&out, "`jigc start --workflow plan`");
    let plan_task = "draft-the-rate-limit-spec";

    let create = jigc_doc(
        repo,
        home,
        probe,
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
        probe,
        &format!("spec:{SPEC_SLUG}#goal"),
        None,
        b"Bound per-client request volume at the gateway.\n",
    );
    set_slot(
        repo,
        home,
        probe,
        &format!("spec:{SPEC_SLUG}#context"),
        None,
        b"Downstream services each enforced limits ad hoc.\n",
    );

    set_field(
        repo,
        home,
        probe,
        &format!("commit:{plan_task}#type"),
        None,
        "docs",
    );
    set_field(
        repo,
        home,
        probe,
        &format!("commit:{plan_task}#scope"),
        None,
        "gateway",
    );
    set_slot(
        repo,
        home,
        probe,
        &format!("commit:{plan_task}#summary"),
        None,
        b"draft the rate limit spec\n",
    );
    set_slot(
        repo,
        home,
        probe,
        &format!("commit:{plan_task}#body"),
        None,
        b"Capture the rate-limit criteria before coding.\n",
    );

    let out = jigc(repo, home, probe, &["task", "finalize", plan_task]);
    assert_ok(&out, "`jigc task finalize` (plan task)");

    // EDITABLE CHANNEL: append a criterion carrying a `maps-to-test` code-anchor, then
    // git-commit it on the base branch BEFORE the work task mints (the pin-to-base
    // constraint — committing after mint would advance HEAD and block the resume).
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
/// spec into its read role (the `maps-to-test` anchor enters effective state), then create
/// and author an `adr` whose `cites-code` header anchor is set in-CLI. Returns the adr slug.
/// Mirrors `flow13_acceptance.rs`'s `mint_bind_and_author_adr` — the same fixture, so the
/// demotion / meta-finding walks land on the identical effective-state surface.
fn mint_bind_and_author_adr(
    repo: &Path,
    home: &Path,
    probe: &Path,
    intent: &str,
    task: &str,
    cites_code: &str,
) -> String {
    let mint = jigc(
        repo,
        home,
        probe,
        &["start", "--workflow", "implement-from-spec", intent],
    );
    assert_ok(&mint, "`jigc start --workflow implement-from-spec`");

    let bind = jigc(
        repo,
        home,
        probe,
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), task],
    );
    assert_ok(&bind, "`jigc task bind spec`");

    let resume = jigc(repo, home, probe, &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose");

    let create = jigc_doc(
        repo,
        home,
        probe,
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
        set_slot(
            repo,
            home,
            probe,
            &format!("adr:{slug}#{section}"),
            Some(task),
            prose,
        );
    }

    set_field(
        repo,
        home,
        probe,
        &format!("adr:{slug}#status/cites-code"),
        Some(task),
        cites_code,
    );

    // Author the commit doc so the only blocking lever is `doc-code` (or the stub).
    set_field(
        repo,
        home,
        probe,
        &format!("commit:{task}#implements"),
        Some(task),
        &format!("spec:{SPEC_SLUG}"),
    );
    set_field(
        repo,
        home,
        probe,
        &format!("commit:{task}#type"),
        Some(task),
        "feat",
    );
    set_field(
        repo,
        home,
        probe,
        &format!("commit:{task}#scope"),
        Some(task),
        "gateway",
    );
    set_slot(
        repo,
        home,
        probe,
        &format!("commit:{task}#summary"),
        Some(task),
        b"enforce the gateway rate limit\n",
    );
    set_slot(
        repo,
        home,
        probe,
        &format!("commit:{task}#body"),
        Some(task),
        b"Bound per-client volume at the edge.\n",
    );

    // A real code change so the empty-commit guard is satisfied by more than the doc.
    fs::write(repo.join("limiter.txt"), "rate limiter\n").expect("write code change");

    slug
}

/// The `findings` array of a **blocked** `task finalize --format json` invocation. The
/// `--format json` report is machine output: it rides **stdout** regardless of the
/// blocking exit (`task.rs` → `finalize`: the `blocked` branch `print!`s the json report
/// to stdout, so `> report.json` captures it), so the JSON is parsed from stdout.
fn finalize_block_findings(out: &std::process::Output) -> Vec<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_slice(&out.stdout)
        .expect("a blocked finalize --format json emits a parseable report on stdout");
    value["findings"].as_array().cloned().unwrap_or_default()
}

const WORK_INTENT: &str = "enforce the rate limit at the gateway";
const WORK_TASK: &str = "enforce-the-rate-limit";

// ─────────── META-FINDING FIRING — timeout (intrinsic, floor-locked) ───────────

/// A stub that sleeps far past the invoker's wall-clock budget — the invoker kills it and
/// the engine synthesizes the intrinsic-blocking `pack-probe-integrity.timeout`
/// meta-finding. (The budget is a fixed const, so the stub must out-sleep it; the invoker
/// kills the child at the budget, so the test does not wait the full sleep.)
const SLEEPER_STUB: &str = r##"
fn main() {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok();
    std::thread::sleep(std::time::Duration::from_secs(120));
    print!(r#"{{"findings":[],"schema_version":3}}"#);
}
"##;

/// A `doc-code` probe that times out blocks `finalize` with an intrinsic-blocking
/// `pack-probe-integrity.timeout` meta-finding — AND a project `scalar-set` demoting that
/// meta-finding to `advisory` is **soft-rejected at resolution** (the floor-lock): the
/// meta-finding **stays blocking**, so the demotion cannot waive the contract. Asserting
/// "stays blocking *with* the advisory delta recorded" is strictly stronger than asserting
/// it merely "fires", so the same run proves both (acceptance bar clauses 2 + 5).
#[test]
fn timeout_meta_finding_fires_and_is_floor_locked_against_demotion() {
    let repo = TempDir::new("timeout-repo");
    let home = TempDir::new("timeout-home");
    let stubs = TempDir::new("timeout-stubs");
    init_repo(repo.path());
    let sleeper = build_stub(stubs.path(), "sleeper", SLEEPER_STUB);

    // Seed the spec + a dangling `maps-to-test` so a `code-anchor` leaf exists for the
    // probe to be scheduled over (the stub controls the outcome, not the anchor).
    commit_spec_with_maps_to_test(repo.path(), home.path(), &sleeper, MAPS_TO_TEST);
    mint_bind_and_author_adr(
        repo.path(),
        home.path(),
        &sleeper,
        WORK_INTENT,
        WORK_TASK,
        CITES_CODE,
    );

    // FLOOR-LOCK lever: record a below-floor demotion of the intrinsic meta-finding. The
    // closed-surface write succeeds (it is a declared knob); the floor is adjudicated at
    // *resolution*, where the delta is soft-rejected (severity_tuning.rs (b) precedent).
    let set = jigc(
        repo.path(),
        home.path(),
        &sleeper,
        &[
            "config",
            "set",
            "validation.pack-probe-integrity.timeout.severity",
            "advisory",
        ],
    );
    assert_ok(
        &set,
        "`jigc config set` (intrinsic meta-finding, below-floor)",
    );

    let before = head_count(repo.path());
    let out = jigc(
        repo.path(),
        home.path(),
        &sleeper,
        &["task", "finalize", WORK_TASK, "--format", "json"],
    );
    let rendered = streams(&out);

    // finalize BLOCKS despite the recorded advisory demotion — the floor-lock holds.
    assert!(
        !out.status.success(),
        "a timed-out probe must block finalize even with an advisory demotion recorded \
         (the intrinsic floor-lock); got:\n{rendered}",
    );
    // The block is the intrinsic timeout meta-finding, STILL graded blocking (not the
    // recorded advisory) — the structured handle (probe, check) is the floor-locked proof.
    let metas = finalize_block_findings(&out);
    let timeout = metas
        .iter()
        .find(|f| f["probe"] == "pack-probe-integrity" && f["check"] == "timeout")
        .unwrap_or_else(|| panic!("the timeout meta-finding must fire; got:\n{rendered}"));
    assert_eq!(
        timeout["severity"], "blocking",
        "the intrinsic timeout meta-finding stays blocking — the advisory scalar-set is \
         soft-rejected at the floor (it cannot waive the contract); got:\n{rendered}",
    );

    // No commit lands — the blocked finalize created nothing.
    assert_eq!(
        before,
        head_count(repo.path()),
        "a meta-finding block must create no commit",
    );
}

// ─────────── META-FINDING FIRING — malformed output (intrinsic) ───────────

/// A stub that exits 0 but writes non-JSON to stdout — the engine's response parse fails
/// and synthesizes the intrinsic-blocking `pack-probe-integrity.malformed-output`
/// meta-finding (the invoker is parse-free; malformed output is the engine's call).
const GARBAGE_STUB: &str = r#"
fn main() {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok();
    print!("this is not json at all <<>>");
}
"#;

/// A `doc-code` probe that emits non-JSON blocks `finalize` with an intrinsic-blocking
/// `pack-probe-integrity.malformed-output` meta-finding — the contract is satisfied, not
/// silently passed (acceptance bar clause 2).
#[test]
fn malformed_output_meta_finding_fires_and_blocks_finalize() {
    let repo = TempDir::new("malformed-repo");
    let home = TempDir::new("malformed-home");
    let stubs = TempDir::new("malformed-stubs");
    init_repo(repo.path());
    let garbage = build_stub(stubs.path(), "garbage", GARBAGE_STUB);

    commit_spec_with_maps_to_test(repo.path(), home.path(), &garbage, MAPS_TO_TEST);
    mint_bind_and_author_adr(
        repo.path(),
        home.path(),
        &garbage,
        WORK_INTENT,
        WORK_TASK,
        CITES_CODE,
    );

    let before = head_count(repo.path());
    let out = jigc(
        repo.path(),
        home.path(),
        &garbage,
        &["task", "finalize", WORK_TASK, "--format", "json"],
    );
    let rendered = streams(&out);

    // finalize BLOCKS — unparseable probe output is an intrinsic-blocking meta-finding.
    assert!(
        !out.status.success(),
        "a probe emitting non-JSON must block finalize (the contract is satisfied, not \
         waived); got:\n{rendered}",
    );
    let metas = finalize_block_findings(&out);
    let malformed = metas
        .iter()
        .find(|f| f["probe"] == "pack-probe-integrity" && f["check"] == "malformed-output")
        .unwrap_or_else(|| panic!("the malformed-output meta-finding must fire; got:\n{rendered}"));
    assert_eq!(
        malformed["severity"], "blocking",
        "the malformed-output meta-finding is intrinsic-blocking; got:\n{rendered}",
    );

    assert_eq!(
        before,
        head_count(repo.path()),
        "a meta-finding block must create no commit",
    );
}

// ─────────── SEVERITY DEMOTION (real probe) — tunable doc-code check ───────────

/// With the project `scalar-set` `validation.doc-code.criterion-maps-to-test.severity:
/// warning` recorded, a **dangling** `maps-to-test` criterion anchor (resolved over the
/// REAL tree-sitter probe) **surfaces as a warning** (visible at `task validate`) but does
/// **NOT block** — `finalize` lands the commit (the M6 post-pass; no code change). The
/// `adr.cites-code` `symbol-exists` anchor is left resolving, so the demoted criterion is
/// the only lever, and `doc-code` IS reached (the warning is the proof it ran — not a
/// silent skip). This is the tunable half of the two-tier rule, the mirror of the
/// floor-locked meta-finding above (acceptance bar clause 5).
#[test]
fn demoting_doc_code_criterion_to_warning_surfaces_but_does_not_block_finalize() {
    let repo = TempDir::new("demote-repo");
    let home = TempDir::new("demote-home");
    let probe = real_doc_code_probe();
    init_repo(repo.path());

    // The criterion's `maps-to-test` points at a test the working tree LACKS — a dangling
    // repeatable-leaf anchor that blocks by DEFAULT (the T3 blocking walk's surface).
    commit_spec_with_maps_to_test(
        repo.path(),
        home.path(),
        probe,
        "tests/rate_limit.rs#burst_rejected_renamed",
    );
    // The `cites-code` symbol resolves — only the criterion anchor dangles.
    mint_bind_and_author_adr(
        repo.path(),
        home.path(),
        probe,
        WORK_INTENT,
        WORK_TASK,
        CITES_CODE,
    );

    // BEFORE the demotion, the dangling criterion blocks by default — `task validate`
    // exits non-zero and renders the finding as `blocking · doc-code.criterion-maps-to-test`
    // (the post-pass default). This pins that the dangling anchor really IS the lever.
    let before_validate = jigc(
        repo.path(),
        home.path(),
        probe,
        &["task", "validate", WORK_TASK],
    );
    let before_rendered = streams(&before_validate);
    assert!(
        !before_validate.status.success(),
        "a dangling criterion must block validate by default; got:\n{before_rendered}",
    );
    assert!(
        before_rendered.contains("blocking · doc-code.criterion-maps-to-test"),
        "the default dangling-criterion finding renders as `blocking`; got:\n{before_rendered}",
    );

    // Demote the TUNABLE `doc-code` criterion check to `warning` (no floor — the demotion
    // applies, unlike the floor-locked meta-finding).
    let set = jigc(
        repo.path(),
        home.path(),
        probe,
        &[
            "config",
            "set",
            "validation.doc-code.criterion-maps-to-test.severity",
            "warning",
        ],
    );
    assert_ok(&set, "`jigc config set` (tunable doc-code demotion)");

    // AFTER the demotion: `task validate` SURFACES the same dangling anchor as a WARNING
    // and no longer blocks (exit 0) — the M6 post-pass re-grades it. The warning is the
    // witness the real probe RAN and reached the criterion (not a silent skip).
    let after_validate = jigc(
        repo.path(),
        home.path(),
        probe,
        &["task", "validate", WORK_TASK],
    );
    let after_rendered = streams(&after_validate);
    assert!(
        after_validate.status.success(),
        "after demoting the criterion check to warning, the dangling anchor must NOT block \
         validate (exit 0); got:\n{after_rendered}",
    );
    assert!(
        after_rendered.contains("warning · doc-code.criterion-maps-to-test"),
        "the demoted dangling criterion must surface as a `warning`; got:\n{after_rendered}",
    );
    assert!(
        !after_rendered.contains("blocking · doc-code.criterion-maps-to-test"),
        "the demoted criterion must NOT render as `blocking`; got:\n{after_rendered}",
    );

    // The headline: finalize LANDS the commit — the demoted (warning) dangling anchor does
    // not gate, no code change required (the M6 post-pass tunes the gate, not the bytes).
    let before = head_count(repo.path());
    let out = jigc(
        repo.path(),
        home.path(),
        probe,
        &["task", "finalize", WORK_TASK],
    );
    let rendered = streams(&out);
    assert_ok(
        &out,
        &format!(
            "finalize must land the commit with the criterion demoted to warning; got:\n{rendered}"
        ),
    );
    assert_eq!(
        head_count(repo.path()),
        before + 1,
        "the demoted-criterion walk must land exactly ONE commit",
    );
}
