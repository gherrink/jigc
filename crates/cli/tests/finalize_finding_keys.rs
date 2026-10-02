//! M42 Increment 9 / T4 — the `finalize.*` family carries its declared **key target**,
//! through the real binary (`design/command-output-contract.md` → the finalize sub-table,
//! and §3's widened consumer list: *a blocked finalize emits its findings as JSON data —
//! it is the acting verb's error payload, and a driver reads it*).
//!
//! Every one of the family's constructors emitted `location: None` at HEAD, so
//! `(finalize.base-mismatch, null)` was **one key for every diverged task in every repo**
//! and `(finalize.promote-clobber, null)` one key for every refused overwrite — the exact
//! degenerate key the contract *claims* cannot exist. The subject **splits**:
//!
//! - the **work unit** takes the work-unit ref — `task:<id>` / `milestone:<id>`,
//! - the **file** takes its path (a promote destination may be a foreign file with no URI
//!   identity — the `file-state` reason exactly).
//!
//! The arms drive the emitted envelope, never a reconstruction: a blocked
//! `jigc task finalize --format json` / `jigc milestone finalize --format json` prints the
//! pinned findings envelope, and these assert on the `key` it carries.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-finalize-finding-keys-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
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
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` against the **embedded** packs (`JIGC_PACK_DIR` removed — the
/// `[dev ▸ methodology]` compose marker requires it absent, and the embedded dev pack is
/// compiled from `crates/cli/packs/dev/`, so every arm reads the same schemas).
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The `[dev ▸ methodology]` compose marker — the key `make_pack` reads to assemble the
/// composition (the `milestone` verbs need the methodology pack's `milestone-record`).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        ok(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            "doc set-field",
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"key the findings\n");
    set_slot(&format!("commit:{task}#body"), b"An M42 change.\n");
}

/// The blocked finalize's pinned findings envelope (stdout, `--format json`), asserting the
/// run blocked.
fn blocked_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    assert!(
        !out.status.success(),
        "`{what}` must block; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`{what}` blocks with the pinned findings envelope on stdout ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The `key` of the one finding carrying `code` in `findings`.
fn key_of(findings: &[serde_json::Value], code: &str) -> serde_json::Value {
    findings
        .iter()
        .find(|f| f["code"] == code)
        .unwrap_or_else(|| panic!("a `{code}` finding rides the envelope; got:\n{findings:#?}"))
        ["key"]
        .clone()
}

/// Two tasks pinned at the same base, blocked by the same moved history, must key at
/// **their own** work-unit refs — `(finalize.base-mismatch, null)` was one key for every
/// diverged task in every repo.
#[test]
fn two_tasks_blocked_on_the_same_moved_base_key_at_their_own_ids() {
    let repo = TempDir::new("base-mismatch");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "alpha task"],
        "jigc start (alpha)",
    );
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "beta task"],
        "jigc start (beta)",
    );

    // HEAD moves on history that OVERLAPS both tasks' footprint: a commit touching
    // README.md, which is then left dirty in the working tree.
    fs::write(repo.path().join("README.md"), "hello\nlanded\n").expect("modify README");
    git(repo.path(), &["add", "README.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "landed after the tasks"],
    );
    fs::write(repo.path().join("README.md"), "hello\nlanded\nlocal\n").expect("dirty README");

    let alpha = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", "alpha-task", "--format", "json"],
            None,
        ),
        "jigc task finalize alpha-task",
    );
    let beta = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", "beta-task", "--format", "json"],
            None,
        ),
        "jigc task finalize beta-task",
    );

    let alpha_key = key_of(&alpha, "finalize.base-mismatch");
    let beta_key = key_of(&beta, "finalize.base-mismatch");
    assert_eq!(
        alpha_key["target"], "task:alpha-task",
        "the diverged task keys at its own work-unit ref; got: {alpha_key}"
    );
    assert_eq!(
        beta_key["target"], "task:beta-task",
        "the second task's identical block keys at ITS own id; got: {beta_key}"
    );
    assert_ne!(
        alpha_key, beta_key,
        "two diverged tasks must not collide on one key"
    );
}

/// The promote-clobber refusal keys at the **destination path** it refused to overwrite —
/// a file that may be foreign (no URI identity), the `file-state` reason exactly.
#[test]
fn a_promote_clobber_keys_at_the_destination_path() {
    let repo = TempDir::new("clobber");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    let task = "record-the-decision";
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "record the decision"],
        "jigc start",
    );
    ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache Strategy",
            "--task",
            task,
        ],
        "jigc doc create adr",
    );
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc(
            repo.path(),
            home.path(),
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_slot("adr:cache-strategy#context", b"The cache is cold.\n");
    set_slot("adr:cache-strategy#decision", b"Warm it on boot.\n");
    set_slot("adr:cache-strategy#consequences", b"A slower boot.\n");
    fill_commit(repo.path(), home.path(), task);

    // A file already occupies the ADR's canonical promote destination
    // (`<docs-root>/<location>/<slug>.md`) — promoting the created doc over it would be
    // irreversible data loss.
    let destination = "docs/decisions/cache-strategy.md";
    fs::create_dir_all(repo.path().join("docs").join("decisions")).expect("mk decisions dir");
    fs::write(
        repo.path().join(destination),
        "# Cache Strategy\n\nhand-authored, not jigc's.\n",
    )
    .expect("write the squatting file");

    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (clobber)",
    );
    let key = key_of(&findings, "finalize.promote-clobber");
    assert_eq!(
        key["target"], destination,
        "the clobber refusal keys at the destination file it refused to overwrite; got: {key}"
    );
}

/// The CLI-side `finalize.nothing-staged` block keys at the task too — the family is scoped
/// to a *concept*, not to `finalize.rs` (two of its members live in `crates/cli/src/task.rs`).
#[test]
fn the_nothing_staged_block_keys_at_the_task() {
    let repo = TempDir::new("nothing-staged");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // A first task lands a real staged edit (so the config layer is already committed and
    // its first-commit term cannot mask the empty narrowed set).
    let first = "land-the-baseline";
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "land the baseline"],
        "jigc start (first)",
    );
    fs::write(repo.path().join("feature.rs"), "pub fn feature() {}\n").expect("write feature.rs");
    git(repo.path(), &["add", "feature.rs"]);
    fill_commit(repo.path(), home.path(), first);
    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", first],
        "jigc task finalize (first)",
    );

    // A second task: the tree is dirty, but nothing is `git add`ed.
    let second = "stage-nothing";
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "stage nothing"],
        "jigc start (second)",
    );
    fs::write(repo.path().join("README.md"), "hello\nunstaged\n").expect("dirty README");
    fill_commit(repo.path(), home.path(), second);

    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", second, "--format", "json"],
            None,
        ),
        "jigc task finalize (nothing staged)",
    );
    let key = key_of(&findings, "finalize.nothing-staged");
    assert_eq!(
        key["target"],
        format!("task:{second}"),
        "the staged-nothing block keys at the task; got: {key}"
    );
}

/// A blocked **milestone** finalize keys at `milestone:<id>` — the same code as the task
/// arm, a different work unit, and (§3, widened) its block is an acting verb's error
/// payload: `--format json` emits the pinned findings envelope a driver reads.
#[test]
fn a_blocked_milestone_finalize_keys_at_the_milestone() {
    let repo = TempDir::new("milestone");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());

    ok(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
        "jigc milestone create",
    );
    ok(
        repo.path(),
        home.path(),
        &["milestone", "add-task", "cache-rework", "Warm the cache"],
        "jigc milestone add-task",
    );

    // Landed code after the milestone's pinned base — the base-guard's real invalidation.
    fs::write(repo.path().join("src.rs"), "fn main() {}\n").expect("write code file");
    git(repo.path(), &["add", "src.rs"]);
    git(repo.path(), &["commit", "-q", "-m", "landed code"]);

    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["milestone", "finalize", "cache-rework", "--format", "json"],
            None,
        ),
        "jigc milestone finalize",
    );
    let key = key_of(&findings, "finalize.base-mismatch");
    assert_eq!(
        key["target"], "milestone:cache-rework",
        "the diverged milestone keys at its own work-unit ref; got: {key}"
    );
}
