//! The headline install acceptance, exercised through the **real** built `jigc` binary
//! with the `JIGC_DOC_CODE_PROBE` override **UNSET** and **no** `jigc setup`.
//!
//! Since M54 S4 the `doc-code` probe ships inside `jigc` and runs by self-exec
//! (`jigc __probe doc-code --build <version>`), so an install is **one file**
//! ([module-layout.md](../../../implementation/module-layout.md) → Probe boundary, the
//! bundled probe). A `cargo install` relocates only the `[[bin]]` — which is now the whole
//! install. This suite stands that up through the production resolution path, never the
//! env override (the M10 invocation-path-masking lesson: testing only the override masks
//! a probe that doesn't ship): every `jigc` here is a **relocated copy** of the built
//! binary in a fresh directory — or a **symlink** to that copy from another directory —
//! with no sibling beside it, no setup, and the override removed from the child env.
//!
//! Asserts, for the relocated copy and for the symlink to it:
//!   - a doc-code-anchored `task finalize` **blocks** on an anchor whose cited symbol
//!     drifted away in the index, naming it — the probe ran;
//!   - with the symbol restored the same task lands, and `jigc validate` over the store it
//!     committed is **clean** (exit 0, no `doc-code` finding);
//!   - once the cited symbol drifts in the working tree, `jigc validate` surfaces the
//!     **dangling** anchor as its `doc-code.symbol-exists` finding;
//!   - nowhere a probe-missing error or a `pack-probe-integrity` meta-finding.
//!
//! Plus the size envelope — the installed `jigc` is ~MBs, not the ~400MB a re-swept
//! build tree would embed.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init`, and self-cleaning `TempDir`s keep the dev repo clean.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A ceiling on the installed `jigc` binary. The test artifact is the unoptimized
/// **debug** `CARGO_BIN_EXE_jigc` (line-tables-only debuginfo): ~40.7 MB (40,743,912
/// bytes) at M54 Inc 2 T4, when the probe joined the `jigc` bin while `setup` still
/// embedded the ~9.4 MB detached probe, and ~31.3 MB (31,264,520 bytes, macOS) at T5,
/// once that embed retired. The ceiling stays 50 MiB (≈1.68× the T5 size), because the
/// Linux runner's size was not re-measured here: a re-swept build-tree embed (~400MB)
/// still blows it, while a clean debug build stays under.
const SIZE_CEILING_BYTES: u64 = 50 * 1024 * 1024;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-cargo-install-{tag}-{}-{:?}",
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

/// Make `path` owner-executable (`0o755`).
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path).expect("stat for chmod").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("chmod");
}

/// Run the **installed** `jigc` (so the probe it spawns is its own image, not the build tree)
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

/// Run the installed `jigc doc <args>`, optionally piping `stdin` — same no-override
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

/// Copy the built `jigc` into a fresh directory with nothing beside it — a `cargo
/// install` relocates only the `[[bin]]`. Returns the install dir (owns cleanup) + the
/// path to the copied `jigc`.
fn relocated_install() -> (TempDir, PathBuf) {
    let bin = TempDir::new("bin");
    let jigc = bin.path().join("jigc");
    fs::copy(env!("CARGO_BIN_EXE_jigc"), &jigc).expect("copy the built jigc into a fresh dir");
    make_executable(&jigc);
    let entries = fs::read_dir(bin.path())
        .expect("list the install dir")
        .count();
    assert_eq!(entries, 1, "the install is one file: the copied jigc alone");
    (bin, jigc)
}

/// Rewrite the tracked `src/lib.rs` so it defines `symbol` (and nothing else).
fn write_lib(repo: &Path, symbol: &str) {
    fs::write(
        repo.join("src").join("lib.rs"),
        format!("pub fn {symbol}() -> u32 {{\n    42\n}}\n"),
    )
    .expect("write src/lib.rs");
}

/// The probe never went missing and never failed: no pre-flight error, no meta-finding.
fn assert_probe_ran(out: &str, what: &str) {
    assert!(
        !out.contains("`doc-code` probe not found") && !out.contains("pack-probe-integrity"),
        "{what}: the self-exec probe must run — no probe-missing error, no probe-integrity \
         meta-finding; got:\n{out}",
    );
}

/// The whole acceptance over one installed `jigc` (a relocated copy or a symlink to one).
fn anchored_work_runs_the_bundled_probe(installed: &Path) {
    let home = TempDir::new("home");
    let repo = TempDir::new("repo");
    init_task_repo(repo.path());
    let task = "cite-present-no-override";
    stage_anchored_task(installed, repo.path(), home.path(), task, task);

    // --- the cited symbol drifts away in the index → finalize blocks, naming it.
    write_lib(repo.path(), "renamed_symbol");
    git(repo.path(), &["add", "src/lib.rs"]);
    let blocked = jigc(
        installed,
        repo.path(),
        home.path(),
        &["task", "finalize", task],
    );
    let blocked_out = rendered(&blocked);
    assert!(
        !blocked.status.success(),
        "a drifted anchor must block `task finalize`; got:\n{blocked_out}",
    );
    assert!(
        blocked_out.contains("doc-code.symbol-exists") && blocked_out.contains("present_symbol"),
        "the block is the doc-code finding naming the drifted anchor; got:\n{blocked_out}",
    );
    assert_probe_ran(&blocked_out, "task finalize on a drifted anchor");

    // --- the symbol restored → the same task lands, and the store it committed is clean.
    write_lib(repo.path(), "present_symbol");
    git(repo.path(), &["add", "src/lib.rs"]);
    let landed = jigc(
        installed,
        repo.path(),
        home.path(),
        &["task", "finalize", task],
    );
    assert_ok(&landed, "`task finalize` with the cited symbol present");
    let clean = jigc(installed, repo.path(), home.path(), &["validate"]);
    let clean_out = rendered(&clean);
    assert_ok(&clean, "`jigc validate` over the committed, anchored store");
    assert!(
        !clean_out.contains("doc-code"),
        "a valid anchor surfaces no doc-code finding; got:\n{clean_out}",
    );
    assert_probe_ran(&clean_out, "jigc validate over a valid store");

    // --- the cited symbol drifts in the working tree → validate surfaces the dangle.
    write_lib(repo.path(), "renamed_symbol");
    let dangling = jigc(installed, repo.path(), home.path(), &["validate"]);
    let dangling_out = rendered(&dangling);
    assert!(
        dangling.stdout_contains("doc-code.symbol-exists"),
        "a dangling anchor surfaces a doc-code.symbol-exists finding; got:\n{dangling_out}",
    );
    assert!(
        dangling_out.contains("present_symbol"),
        "the dangling finding names the now-stale anchor target; got:\n{dangling_out}",
    );
    assert_probe_ran(&dangling_out, "jigc validate over a dangling anchor");
}

/// The size envelope. The installed `jigc` is ~MBs, not ~400MB; a regressed
/// `include_dir!` that re-swept a gitignored `target/` tree would blow the ceiling.
#[test]
fn installed_jigc_binary_is_megabytes_not_gigabytes() {
    // Measure the relocated *install* itself (not the test harness's own build artifact)
    // so the guard asserts on the thing it describes.
    let (_bin, installed_jigc) = relocated_install();
    let size = fs::metadata(&installed_jigc)
        .expect("stat the installed jigc binary")
        .len();
    assert!(
        size < SIZE_CEILING_BYTES,
        "the installed jigc must be ~MBs, not ~400MB (a re-swept target/ embed blows this \
         ceiling): {size} bytes ≥ {SIZE_CEILING_BYTES} byte ceiling",
    );
}

/// A relocated copy of `jigc`, alone in its directory, no setup, no override: the
/// bundled probe runs by self-exec at task and store scope.
#[test]
fn a_relocated_jigc_runs_its_bundled_probe_with_no_setup() {
    let (_bin, installed) = relocated_install();
    anchored_work_runs_the_bundled_probe(&installed);
}

/// A symlink to that relocated copy, from another directory: the install still resolves
/// its bundled probe.
#[test]
fn a_symlinked_jigc_runs_its_bundled_probe_with_no_setup() {
    let (_bin, installed) = relocated_install();
    let links = TempDir::new("links");
    let link = links.path().join("jigc");
    std::os::unix::fs::symlink(&installed, &link).expect("symlink the installed jigc");
    anchored_work_runs_the_bundled_probe(&link);
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
