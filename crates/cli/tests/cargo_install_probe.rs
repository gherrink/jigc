//! M20 inc-1 / T4 — the headline `cargo install` acceptance, exercised through the
//! **real** built `jigc` binary with the `JIGC_DOC_CODE_PROBE` override **UNSET**.
//!
//! This is the cross-cutting Deliverable proof (`implementation/roadmap.md` → M20
//! Increment 1 Deliverable + Proves; `module-layout.md` → Probe distribution): a
//! `cargo install`-style install relocates only the `[[bin]]` `jigc`, so the build-tree
//! `doc-code` sibling does **not** travel. T1 de-bloats the embed, T2 embeds the probe +
//! a neutral `jigc setup` extract step, T3 reports a missing probe once at task scope.
//! T4 stands all three up together **through the production resolution path** — the probe
//! a `<jigc-bin-dir>/doc-code` sibling, never the env override (the M10
//! invocation-path-masking lesson: testing only the override masks a probe that doesn't
//! ship). Every `jigc` here is a copy of the built binary in a **probe-less** bin dir, run
//! with `JIGC_DOC_CODE_PROBE` deliberately removed from the child env, so
//! `current_exe().parent()/doc-code` is the only resolution.
//!
//! Asserts:
//!   - **(i) before setup** — over a committed code-anchor store, `jigc validate` reports
//!     exactly one clean `doc-code` probe-missing operational error (T3's shape), proving
//!     the sibling truly isn't there; the same shape at the **task** scope for
//!     `task validate` / `task finalize`.
//!   - **(ii) after setup** — the `doc-code` sibling exists beside the installed `jigc`;
//!     `jigc validate` over a valid-anchor store exits 0 with no `doc-code` content
//!     finding, and over a dangling-anchor store surfaces the `doc-code` content finding —
//!     resolved purely via the setup-extract (no override, no manual copy).
//!   - **(iii) the size envelope** — the installed `jigc` binary is ~MBs, not ~400MB (the
//!     load-bearing red step for T1's de-bloat: a regressed `include_dir!` that re-swept
//!     the probe's gitignored ~401MB `target/` would blow this ceiling).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init`, and self-cleaning `TempDir`s keep the dev repo clean.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A release-realistic ceiling on the installed `jigc` binary, tightened at M27 so
/// future grammar bloat trips it. The test artifact is the unoptimized **debug**
/// `CARGO_BIN_EXE_jigc` with full debuginfo, ~71.3MB at M27 (it now embeds the
/// ~24.5MB debug five-grammar `doc-code` probe), so 90MB (≈1.26× current) leaves
/// only modest headroom: a sixth tree-sitter grammar or a re-swept probe `target/`
/// embed (~400MB) blows it, while a clean debug build stays comfortably under.
const SIZE_CEILING_BYTES: u64 = 90 * 1024 * 1024;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-cargo-install-{tag}-{}-{:?}",
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

/// Copy the built `jigc` into a fresh, **probe-less** bin dir (no `doc-code` sibling),
/// mirroring a `cargo install` that relocates only the `[[bin]]`. Returns the install
/// dir (owns cleanup) + the path to the copied `jigc`.
fn probe_less_install() -> (TempDir, PathBuf) {
    let bin = TempDir::new("bin");
    let jigc = bin.path().join("jigc");
    fs::copy(env!("CARGO_BIN_EXE_jigc"), &jigc).expect("copy the built jigc into a probe-less dir");
    make_executable(&jigc);
    assert!(
        !bin.path().join("doc-code").exists(),
        "the fresh install dir must start with no doc-code sibling",
    );
    (bin, jigc)
}

/// Make `path` owner-executable (`0o755`).
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path).expect("stat for chmod").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("chmod");
}

/// Run the **copied** `jigc` (so `current_exe()` resolves to the probe-less install dir)
/// with `cwd = repo`, `$HOME = home`, and the `JIGC_DOC_CODE_PROBE` override **removed**
/// from the child env — the production resolution path is the only one exercised.
fn jigc(installed: &Path, repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(installed)
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .output()
        .expect("run the copied jigc binary")
}

/// Run the copied `jigc doc <args>`, optionally piping `stdin` — same probe-less, no-override
/// env. Used to author the staged ADR/commit slots on the task path.
fn jigc_doc(
    installed: &Path,
    repo: &Path,
    home: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(installed);
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE");
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn copied jigc doc");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc doc")
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
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

/// The combined stdout+stderr of an invocation, for substring assertions.
fn rendered(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// A committed `adr` citing `<rel>#<symbol>` from its `cites-code` header anchor (the dev
/// pack's `code-anchor` field type → `doc-code/symbol-exists`).
fn adr(rel: &str, symbol: &str) -> String {
    format!(
        "---\n\
         status: accepted\n\
         date: 2026-06-14\n\
         cites-code: {rel}#{symbol}\n\
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
         Effects.\n"
    )
}

/// Seed a real git repo with the `.jigc/config/` project layer + a committed store whose
/// single `adr` cites `<symbol>` in `crates/engine/src/cache.rs` (which defines
/// `evict_lru`). Passing `"evict_lru"` is the clean case; any other name dangles.
fn seed_store(repo: &Path, cited_symbol: &str) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    let cache = repo.join("crates/engine/src/cache.rs");
    fs::create_dir_all(cache.parent().unwrap()).expect("mk code dir");
    fs::write(&cache, "pub fn evict_lru() {}\nfn helper() {}\n").expect("write cache.rs");

    fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
    fs::write(
        repo.join("docs/decisions/cache.md"),
        adr("crates/engine/src/cache.rs", cited_symbol),
    )
    .expect("write adr");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // The project layer — the locate-preamble's `require_project_layer` gate.
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer and a
/// `src/lib.rs` carrying `present_symbol` (the symbol the staged anchored task resolves
/// against). Mirrors the `doc_code_gate.rs` task fixture.
fn init_task_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("src")).expect("mk src");
    fs::write(
        repo.join("src").join("lib.rs"),
        "pub fn present_symbol() -> u32 {\n    42\n}\n",
    )
    .expect("write lib.rs");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Stage a single-task task whose created ADR carries a `cites-code` anchor over the
/// present symbol, with every author-required slot filled — so the task's effective state
/// carries code-anchor work (the surface the probe gate enumerates) and the only blocking
/// lever is the `doc-code` probe's resolvability. Mirrors `doc_code_gate::stage_anchored_task`.
fn stage_anchored_task(installed: &Path, repo: &Path, home: &Path, task: &str, slug: &str) {
    let intent = task.replace('-', " ");
    assert_ok(
        &jigc(
            installed,
            repo,
            home,
            &["start", "--workflow", "single-task", &intent],
        ),
        "`jigc start`",
    );

    let create = jigc_doc(
        installed,
        repo,
        home,
        &["create", "adr", "--title", &slug.replace('-', " ")],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let minted = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(minted, format!("adr:{slug}"), "minted ADR address");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            installed,
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

    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(
            installed,
            repo,
            home,
            &["set-field", addr, "--value", value],
            None,
        );
        assert_ok(&out, &format!("set-field {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");

    // Inject the `cites-code` anchor into the staged ADR's empty front-matter block
    // (the optional anchor is absent from the created skeleton) — keeps this fixture's
    // concern the gate, not the write path (mirrors `doc_code_gate::inject_cites_code`).
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"));
    let body = fs::read_to_string(&staged).expect("read staged ADR");
    // Insert before the closing front-matter fence (the created ADR now carries
    // materialized `status`/`date` header lines, not an empty fence).
    let with = body.replacen(
        "\n---\n",
        "\ncites-code: src/lib.rs#present_symbol\n---\n",
        1,
    );
    assert_ne!(
        body, with,
        "the staged ADR carries a front-matter block to inject the anchor into"
    );
    fs::write(&staged, &with).expect("inject cites-code anchor");
}

/// Run the copied `jigc setup` so the extract step writes the `doc-code` sibling beside the
/// installed binary.
fn run_setup(installed: &Path, repo: &Path, home: &Path) {
    let out = jigc(installed, repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup` from a probe-less install");
}

/// (iii) The size envelope — the load-bearing red step for T1's de-bloat. The installed
/// `jigc` is ~MBs, not ~400MB; a regressed `include_dir!` that re-swept the probe's
/// gitignored `target/` tree would blow the ceiling.
#[test]
fn installed_jigc_binary_is_megabytes_not_gigabytes() {
    // Measure the copied probe-less *install* itself (not the test harness's own build
    // artifact) so the guard asserts on the thing it describes — the bytes are identical
    // (the install is a `fs::copy` of the built binary), but this stays honest to its name.
    let (_bin, installed_jigc) = probe_less_install();
    let size = fs::metadata(&installed_jigc)
        .expect("stat the installed jigc binary")
        .len();
    assert!(
        size < SIZE_CEILING_BYTES,
        "the installed jigc must be ~MBs, not ~400MB (a re-swept probe target/ embed blows \
         this ceiling): {size} bytes ≥ {SIZE_CEILING_BYTES} byte ceiling",
    );
}

/// The store-scope headline: a `cargo install`-style probe-less `jigc`, override UNSET.
/// (i) before setup, `jigc validate` over a committed code-anchor store reports exactly one
/// clean probe-missing operational error (T3's shape) — the sibling truly isn't there.
/// (ii) after `jigc setup` extracts the probe, the sibling exists and `jigc validate` over a
/// valid-anchor store exits 0 with no doc-code finding, while a dangling-anchor store
/// surfaces the doc-code content finding — resolved purely via the setup-extract.
#[test]
fn store_validate_reports_clean_absence_then_resolves_via_setup_extract() {
    let (bin, jigc_bin) = probe_less_install();
    let sibling = bin.path().join("doc-code");
    let home = TempDir::new("home");

    // --- (i) before setup: a code-anchor store, no probe sibling, no override.
    let valid = TempDir::new("store-valid");
    seed_store(valid.path(), "evict_lru");

    let before = jigc(&jigc_bin, valid.path(), home.path(), &["validate"]);
    let before_out = rendered(&before);
    assert!(
        !before.status.success(),
        "before setup, a missing probe must make `jigc validate` exit non-zero; got:\n{before_out}",
    );
    assert_eq!(
        String::from_utf8_lossy(&before.stderr)
            .matches("`doc-code` probe not found")
            .count(),
        1,
        "before setup, exactly one `doc-code probe not found` operational error must surface \
         (an N-per-anchor regression fails this); got:\n{before_out}",
    );
    assert!(
        !before_out.contains("pack-probe-integrity"),
        "the sweep must not run when the probe is absent — no pack-probe-integrity meta-finding; \
         got:\n{before_out}",
    );

    // --- setup extracts the probe sibling beside the installed jigc.
    let setup_repo = TempDir::new("setup-repo");
    git(setup_repo.path(), &["init", "-q"]);
    run_setup(&jigc_bin, setup_repo.path(), home.path());
    assert!(
        sibling.exists(),
        "`jigc setup` must extract a doc-code sibling beside the installed jigc",
    );

    // --- (ii) after setup: the valid store exits 0 with no doc-code content finding,
    //     resolved purely via the setup-extracted sibling (no override).
    let after_valid = jigc(&jigc_bin, valid.path(), home.path(), &["validate"]);
    let after_valid_out = rendered(&after_valid);
    assert_ok(
        &after_valid,
        &format!("after setup, validate over a valid-anchor store; got:\n{after_valid_out}"),
    );
    assert!(
        !after_valid_out.contains("doc-code"),
        "after setup, a valid-anchor store must surface no doc-code content finding; \
         got:\n{after_valid_out}",
    );

    // …and a dangling-anchor store surfaces the doc-code content finding.
    let dangling = TempDir::new("store-dangling");
    seed_store(dangling.path(), "no_such_symbol");
    let after_dangling = jigc(&jigc_bin, dangling.path(), home.path(), &["validate"]);
    let after_dangling_out = rendered(&after_dangling);
    assert!(
        after_dangling.stdout_contains("doc-code.symbol-exists"),
        "after setup, a dangling anchor must surface a doc-code.symbol-exists finding; \
         got:\n{after_dangling_out}",
    );
    assert!(
        after_dangling_out.contains("no_such_symbol"),
        "the dangling finding names the now-stale anchor target; got:\n{after_dangling_out}",
    );
}

/// The task-scope half: the same probe-less install, override UNSET. (i) before setup, an
/// anchored task's `task validate` / `task finalize` reports exactly one clean probe-missing
/// operational error (T3's shape, via the production sibling resolution). (ii) after
/// `jigc setup` extracts the probe, the same anchored task validates clean (the present
/// symbol resolves) — resolved purely via the setup-extract.
#[test]
fn task_validate_reports_clean_absence_then_resolves_via_setup_extract() {
    let (bin, jigc_bin) = probe_less_install();
    let sibling = bin.path().join("doc-code");
    let home = TempDir::new("home");

    let repo = TempDir::new("task-repo");
    init_task_repo(repo.path());
    let task = "cite-present-no-override";
    stage_anchored_task(
        &jigc_bin,
        repo.path(),
        home.path(),
        task,
        "cite-present-no-override",
    );

    // --- (i) before setup: the probe sibling is absent → one operational error, no crash.
    let before = jigc(
        &jigc_bin,
        repo.path(),
        home.path(),
        &["task", "validate", task],
    );
    let before_out = rendered(&before);
    assert!(
        !before.status.success(),
        "before setup, an anchored task's validate must exit non-zero (missing probe); \
         got:\n{before_out}",
    );
    assert_eq!(
        String::from_utf8_lossy(&before.stderr)
            .matches("`doc-code` probe not found")
            .count(),
        1,
        "before setup, exactly one `doc-code probe not found` operational error at task scope \
         (an N-per-anchor regression fails this); got:\n{before_out}",
    );
    assert!(
        !before_out.contains("pack-probe-integrity"),
        "the task sweep must not run when the probe is absent — no pack-probe-integrity \
         meta-finding; got:\n{before_out}",
    );

    // finalize shares the gate via validate(): the same one operational error, non-zero.
    let before_fin = jigc(
        &jigc_bin,
        repo.path(),
        home.path(),
        &["task", "finalize", task],
    );
    let before_fin_out = rendered(&before_fin);
    assert!(
        !before_fin.status.success(),
        "before setup, an anchored task's finalize must exit non-zero (missing probe); \
         got:\n{before_fin_out}",
    );
    assert_eq!(
        String::from_utf8_lossy(&before_fin.stderr)
            .matches("`doc-code` probe not found")
            .count(),
        1,
        "before setup, exactly one `doc-code probe not found` operational error at finalize \
         (an N-per-anchor regression fails this); got:\n{before_fin_out}",
    );

    // --- setup extracts the probe sibling beside the installed jigc.
    run_setup(&jigc_bin, repo.path(), home.path());
    assert!(
        sibling.exists(),
        "`jigc setup` must extract a doc-code sibling beside the installed jigc",
    );

    // --- (ii) after setup: the same anchored task validates clean — the present symbol
    //     resolves via the setup-extracted sibling (no override, no manual copy).
    let after = jigc(
        &jigc_bin,
        repo.path(),
        home.path(),
        &["task", "validate", task],
    );
    let after_out = rendered(&after);
    assert_ok(
        &after,
        &format!("after setup, the anchored task validates clean; got:\n{after_out}"),
    );
    assert!(
        !after_out.contains("doc-code.symbol-exists"),
        "after setup, the present anchor must surface no doc-code.symbol-exists block; \
         got:\n{after_out}",
    );
}

/// Small extension trait: substring search over an invocation's stdout only (the rendered
/// report lands on stdout; the doc-code content finding is asserted there, not on stderr).
trait StdoutContains {
    fn stdout_contains(&self, needle: &str) -> bool;
}

impl StdoutContains for std::process::Output {
    fn stdout_contains(&self, needle: &str) -> bool {
        String::from_utf8_lossy(&self.stdout).contains(needle)
    }
}
