//! T2 — the methodology pack's `dev-task` workflow composes the flat prose spine,
//! proven through the real binary on a throwaway repo (M12 Inc 1; `design/
//! self-hosting.md` → The dev-workflow sort, Verified friction; `worked-examples.md`
//! → flow 15 setup block).
//!
//! T1 shipped the pack *substrate* (cascade config + vendored commit schema) but no
//! `dev-task` workflow. T2 ships `workflows/dev-task.yaml` + the four prose steps
//! (`scope`/`implement`/`gate`/`finalize`) and proves, end-to-end through the binary:
//!
//!   (a) `JIGC_PACK_DIR=<methodology> jigc setup` then `jigc start "<intent>"` exits
//!       0 and composes the flat spine — the four step bodies in order
//!       scope → implement → gate → finalize.
//!   (b) `{{task.intent}}` resolves to the literal intent *on its own line* (the
//!       lone-line discipline — assert the resolved string appears, NOT the literal
//!       `{{task.intent}}`).
//!   (c) The emitted-bytes contract (hardening #4): the four commit-fill `Run:` lines
//!       (`set-field type`, `set-field scope`, `set-slot summary`, `set-slot body`)
//!       and the `finalize-task` `Run:` line render **verbatim with addresses
//!       resolved** to the minted task's slug — asserted as the emitted lines, never
//!       reconstructed.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/../../packs/methodology`, and a
//! self-cleaning `TempDir` keeps the test off the developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-methodology-compose-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

/// Extract the backtick-quoted body of the *unique* `Run:` line containing `needle`,
/// so the assertion runs over the **emitted bytes** the agent would copy — never a
/// reconstruction. Panics with the full stdout if absent or ambiguous.
fn emitted_run_line<'a>(stdout: &'a str, needle: &str) -> &'a str {
    let mut matches = stdout.lines().filter(|l| {
        let t = l.trim_start();
        t.starts_with("Run:") && t.contains(needle)
    });
    let line = matches
        .next()
        .unwrap_or_else(|| panic!("no `Run:` line containing {needle:?}; got:\n{stdout}"));
    assert!(
        matches.next().is_none(),
        "more than one `Run:` line contains {needle:?}; got:\n{stdout}",
    );
    // The address resolves inside the backtick-quoted command.
    line.split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("`Run:` line not backtick-quoted: {line:?}"))
}

#[test]
fn dev_task_composes_the_flat_spine_resolved_intent_and_verbatim_fill_lines() {
    let repo = TempDir::new("compose");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // The intent → slug derivation is `add rate limiter` → `add-rate-limiter`, so the
    // minted commit doc is `commit:add-rate-limiter` and the task id is
    // `add-rate-limiter` (the same slug the embedded single-task uses in start_compose).
    let intent = "add rate limiter";
    let out = run_jigc(repo.path(), home.path(), &pack, &["start", intent]);
    assert!(
        out.status.success(),
        "`jigc start \"{intent}\"` over the methodology pack must compose dev-task and exit 0; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // (b) `{{task.intent}}` resolves to the literal intent on its own line — the
    // resolved string appears, the literal placeholder never does.
    assert!(
        stdout.lines().any(|l| l.trim() == intent),
        "the composed dev-task must resolve `{{{{task.intent}}}}` to the intent on its own \
         line; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("{{task.intent}}") && !stdout.contains("{{ task.intent }}"),
        "the literal `{{{{task.intent}}}}` placeholder must NOT survive into the composed \
         output; got:\n{stdout}",
    );

    // (a) The flat spine composes the four step bodies in order
    // scope → implement → gate → finalize. Each step contributes a distinctive
    // prose marker; their stdout offsets must be strictly increasing.
    let pos = |needle: &str| {
        stdout
            .find(needle)
            .unwrap_or_else(|| panic!("composed spine missing {needle:?}; got:\n{stdout}"))
    };
    let scope_at = pos("done-criterion");
    let implement_at = pos("failing test");
    let gate_at = pos("test, lint, and build gate");
    let finalize_at = pos("one logical commit");
    assert!(
        scope_at < implement_at && implement_at < gate_at && gate_at < finalize_at,
        "the spine must order scope({scope_at}) < implement({implement_at}) < \
         gate({gate_at}) < finalize({finalize_at}); got:\n{stdout}",
    );

    // (c) Emitted-bytes contract: the four commit-fill Run lines + the finalize-task
    // line render verbatim with addresses resolved to the `add-rate-limiter` slug.
    // Asserted on the EMITTED bytes (extracted from stdout), never reconstructed.
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:add-rate-limiter#type"),
        "jigc doc set-field commit:add-rate-limiter#type --value <TYPE>",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:add-rate-limiter#scope"),
        "jigc doc set-field commit:add-rate-limiter#scope --value <SCOPE>",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:add-rate-limiter#summary"),
        "jigc doc set-slot commit:add-rate-limiter#summary --from-file -",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:add-rate-limiter#body"),
        "jigc doc set-slot commit:add-rate-limiter#body --from-file -",
    );
    assert_eq!(
        emitted_run_line(&stdout, "task finalize"),
        "jigc task finalize add-rate-limiter",
    );

    // The fill lines must precede the finalize line (the agent fills, then renders).
    assert!(
        pos("set-field commit:add-rate-limiter#type") < pos("task finalize add-rate-limiter"),
        "the four commit-fill Run lines must precede `jigc task finalize`; got:\n{stdout}",
    );
}
