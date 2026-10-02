//! M16 Increment 5 / T4 — **flow 20: the completion-half encode**, proven end-to-end
//! over the real `jigc` binary against the REAL methodology pack
//! (`JIGC_PACK_DIR=<methodology>`). The companion of `flow19_planning_encode.rs` for the
//! completion half: `design/worked-examples.md` → flow 20; `design/methodology-docs.md` →
//! Acceptance flows (Flow 20) + The engine work, item 3 (the #5 owner-artifact gate);
//! `implementation/roadmap.md` → M16 Increment 5 bullet 3.
//!
//! `jigc start --workflow completion "<milestone>"` mints a task (off-router), the agent
//! walks the completion spine (audit → triage → fix → re-verify), authors the
//! create-fresh per-milestone `completion-record` (the `verdict` + the `owner-artifact`
//! owned-location path + the triaged `findings`), appends a `decisions-log` entry (the
//! triage decisions — the both-halves driver), and `finalize` promotes the owner-artifact
//! in the SAME transaction as the completion-record and runs the intrinsic **#5
//! owner-artifact presence gate** (`owner-artifact.present`, shipped inc-3). This file is
//! the flow-20 instantiation driven over the REAL `completion` workflow + REAL
//! `completion-record` schema — never the inc-3 FIXTURE (which carried only a bare
//! `owner-artifact` field and a fixture host workflow).
//!
//! Every assertion runs on the EMITTED bytes / exit code of the real binary
//! (`CARGO_BIN_EXE_jigc`) over the on-disk methodology pack (the same seam the sibling
//! methodology tests use): `jigc setup` installs the pack, the promotion is read back via
//! `git show HEAD:<path>` / `git show --name-only HEAD`, so the claims are over the bytes
//! git actually holds.
//!
//! The five done-criteria:
//!   (a) **the spine composes with the halts as `Checkpoint:` directives** — `jigc start
//!       --workflow completion` emits `Checkpoint: triage-gate` and
//!       `Checkpoint: fix-rounds-exhausted` (the two human-gated halts the M15 kind made
//!       structural), driven from the real binary's emitted bytes;
//!   (b) **the #5 gate BLOCKS** on an absent / unsafe-path / present-but-untracked
//!       owner-artifact — finalize exits non-zero, surfaces `owner-artifact.present`, and
//!       NO commit lands (the gate FIRES, not a vacuous path-exists; the present-but-
//!       untracked case is the M3-lesson teeth — the artifact must be git-tracked BEFORE
//!       validate runs);
//!   (c) **the #5 gate PASSES** on an artifact durably staged under
//!       `completions/artifacts/<milestone>/` (git-added before finalize) — finalize lands
//!       and the owner-artifact + the promoted completion-record commit together (the
//!       same-transaction promotion);
//!   (d) **the decisions-log is appended by this half** — the triage decision rides into
//!       the promoted `docs/decisions-log.md` (the both-halves driver);
//!   (e) **the audit verdict is an AUTHORED `meta/verdict` field, never an engine
//!       opinion** — finalize lands a `red` verdict exactly as readily as a `green` one
//!       (the engine records the agent's verdict; it does not certify it — the
//!       single-agent-spine bound, the verdict stays orchestration-level).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow20-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` over the methodology pack with `cwd = repo`, `$HOME = home`,
/// `JIGC_PACK_DIR = <methodology>` (the sibling methodology-test seam), capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
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
        .env("JIGC_PACK_DIR", methodology_pack_tree());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
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

/// Run `jigc setup` (installs the methodology pack) over a fresh repo, asserting OK.
fn setup(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(repo, home, &["setup"]),
        "`JIGC_PACK_DIR=<methodology> jigc setup`",
    );
}

/// Start a `completion` task for `milestone` (off-router, invoked directly via
/// `--workflow`), returning the emitted composed stdout (the spine, for the
/// checkpoint-directive assertion — run verbatim, never reconstructed).
fn start_completion(repo: &Path, home: &Path, milestone: &str) -> String {
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "completion", milestone],
    );
    assert_ok(
        &out,
        &format!("`jigc start --workflow completion {milestone}`"),
    );
    String::from_utf8(out.stdout).expect("utf-8 composed spine")
}

/// Fill the commit doc's author-required header + prose so finalize validates clean —
/// leaving the owner-artifact gate as the lever the absent/untracked/staged cases turn.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "chore");
    set_field(&format!("commit:{task}#scope"), "completion");
    set_slot(&format!("commit:{task}#summary"), b"close the milestone\n");
    set_slot(&format!("commit:{task}#body"), b"A milestone completion.\n");
}

/// The committed bytes of `path` at HEAD.
fn committed(repo: &Path, path: &str) -> String {
    let out = Command::new("git")
        .args(["show", &format!("HEAD:{path}")])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        out.status.success(),
        "{path} must be committed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 committed bytes")
}

/// Create the per-milestone `completion-record` fresh, returning the emitted doc address
/// (captured from `create`'s stdout, run verbatim downstream — never reconstructed).
fn create_completion_record(repo: &Path, home: &Path, title: &str) -> String {
    let create = jigc_doc(
        repo,
        home,
        &["create", "completion-record", "--title", title],
        None,
    );
    assert_ok(&create, "`doc create completion-record`");
    String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Author the completion-record's `meta` header — the `verdict` enum + the
/// `owner-artifact` owned-location path.
fn author_meta(repo: &Path, home: &Path, addr: &str, verdict: &str, owner_artifact: &str) {
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &[
                "set-field",
                &format!("{addr}#meta/verdict"),
                "--value",
                verdict,
            ],
            None,
        ),
        "`set-field verdict`",
    );
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &[
                "set-field",
                &format!("{addr}#meta/owner-artifact"),
                "--value",
                owner_artifact,
            ],
            None,
        ),
        "`set-field owner-artifact`",
    );
}

/// Author one triaged `findings` entry — title + `severity`/`disposition` enums +
/// `evidence` STRING (a plain string, NOT a managed ref — `set-field`, not `set-slot`).
fn author_finding(
    repo: &Path,
    home: &Path,
    addr: &str,
    title: &str,
    severity: &str,
    disposition: &str,
    evidence: &str,
) {
    let add = jigc_doc(
        repo,
        home,
        &["add-item", &format!("{addr}#findings"), "--title", title],
        None,
    );
    assert_ok(&add, "`doc add-item completion-record#findings`");
    let item = String::from_utf8(add.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    for (leaf, value) in [
        ("severity", severity),
        ("disposition", disposition),
        ("evidence", evidence),
    ] {
        assert_ok(
            &jigc_doc(
                repo,
                home,
                &["set-field", &format!("{item}/{leaf}"), "--value", value],
                None,
            ),
            &format!("`set-field findings {leaf}`"),
        );
    }
}

/// Append one `decisions-log` triage entry (the both-halves driver) in the running
/// singleton: create-or-update the log, then `add-item` + author its `why` slot.
fn append_decision(repo: &Path, home: &Path, title: &str, why: &[u8]) {
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["create", "decisions-log", "--title", "Decisions Log"],
            None,
        ),
        "`doc create decisions-log`",
    );
    let add = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            "decisions-log:decisions-log#entries",
            "--title",
            title,
        ],
        None,
    );
    assert_ok(&add, "`doc add-item decisions-log#entries`");
    let item = String::from_utf8(add.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-slot", &format!("{item}/why"), "--from-file", "-"],
            Some(why),
        ),
        "`set-slot <entry>/why`",
    );
}

/// Durably stage an owner-artifact under its owned home `completions/artifacts/<milestone>/`
/// and `git add` it (the orchestrator's same-transaction recording, made explicit so
/// validate sees it tracked BEFORE the commit lands — the M3 lesson the gate enforces).
fn stage_owner_artifact(repo: &Path, path: &str, body: &str) {
    let parent = repo
        .join(path)
        .parent()
        .expect("artifact has a parent")
        .to_path_buf();
    fs::create_dir_all(&parent).expect("mk owned artifact home");
    fs::write(repo.join(path), body).expect("write owner-artifact");
    git(repo, &["add", path]);
}

/// (a) The completion spine composes with the two human-gated halts emitted as structural
/// `Checkpoint:` directives — driven from the real binary's emitted bytes, never a
/// reconstruction. The off-router `--workflow completion` path composes a `creates-task:
/// true` work-workflow (exit 0); its `triage` and `fix-gate` steps are the M15
/// `checkpoint` kind, so the spine carries `Checkpoint: triage-gate` and
/// `Checkpoint: fix-rounds-exhausted`.
#[test]
fn flow20_the_completion_spine_composes_with_the_halts_as_checkpoint_directives() {
    let repo = TempDir::new("spine");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let spine = start_completion(repo.path(), home.path(), "M16");
    assert!(
        spine.contains("Checkpoint: triage-gate"),
        "the triage halt composes as a structural `Checkpoint: triage-gate` directive; \
         got composed spine:\n{spine}",
    );
    assert!(
        spine.contains("Checkpoint: fix-rounds-exhausted"),
        "the fix-round halt composes as a structural `Checkpoint: fix-rounds-exhausted` \
         directive; got composed spine:\n{spine}",
    );
    // The audit/triage/fix/re-verify phase walk is present (the spine, not just the gates).
    assert!(
        spine.contains("Audit the assembled milestone") && spine.contains("Re-verify"),
        "the spine carries the audit → re-verify phase walk; got:\n{spine}",
    );
}

/// (b) The #5 gate BLOCKS on a NON-PRESENT / unsafe / **gitignored** owner-artifact — driven
/// through the REAL completion workflow. Each case exits finalize non-zero, surfaces
/// `owner-artifact.present`, and lands NO commit. Since M45 Inc 8 T2 a present-but-**merely-
/// untracked** artifact no longer blocks — finalize *stages* the recorded path
/// in-transaction, so the produced-but-unstaged case now LANDS (proven in
/// `owner_artifact_gate.rs`). The anti-vacuity teeth shift to the **gitignored** case: a file
/// on disk that git *cannot* durably stage is still not a durable recording, so the gate must
/// still fire — proving the stage path is load-bearing, not incidental.
#[test]
fn flow20_the_owner_artifact_gate_blocks_when_the_artifact_is_not_durably_staged() {
    // Three sub-cases of "not durably staged", each its own fresh repo.
    for (tag, milestone, owner_value, gitignored) in [
        // Absent: a well-shaped owned-home path whose file is never created.
        ("absent", "M16", "completions/artifacts/M16/audit.md", false),
        // Unsafe: an absolute path the gate must reject outright.
        ("unsafe", "M16", "/etc/passwd", false),
        // Gitignored: the file exists on disk under the owned home but a `.gitignore` covers
        // it, so finalize's stage cannot `git add` it — the gate's anti-vacuity teeth under
        // T2 (presence on disk ≠ durably committable).
        (
            "gitignored",
            "M16",
            "completions/artifacts/M16/audit.md",
            true,
        ),
    ] {
        let repo = TempDir::new(tag);
        let home = TempDir::new("home");
        init_repo(repo.path());
        setup(repo.path(), home.path());

        start_completion(repo.path(), home.path(), milestone);
        let addr = create_completion_record(repo.path(), home.path(), milestone);

        if gitignored {
            // Present on disk under the owned home, but covered by `.gitignore` — so it is
            // neither tracked nor stageable (finalize's `git add` skips it), defeating the
            // durable-presence gate even though it exists.
            let parent = repo
                .path()
                .join(owner_value)
                .parent()
                .expect("parent")
                .to_path_buf();
            fs::create_dir_all(&parent).expect("mk owned home");
            fs::write(repo.path().join(owner_value), "gitignored transcript\n")
                .expect("write gitignored artifact");
            fs::write(
                repo.path().join(".gitignore"),
                "completions/artifacts/M16/\n",
            )
            .expect("write .gitignore covering the owned home");
        }

        author_meta(repo.path(), home.path(), &addr, "green", owner_value);
        // A complete finding so the ONLY lever is the owner-artifact gate.
        author_finding(
            repo.path(),
            home.path(),
            &addr,
            "A finding",
            "advisory",
            "fixed",
            "file.rs:1",
        );
        let task = milestone.to_lowercase();
        fill_commit(repo.path(), home.path(), &task);

        let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
            .trim()
            .parse()
            .unwrap();
        let out = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
        let rendered = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !out.status.success(),
            "[{tag}] a not-durably-staged owner-artifact must block finalize non-zero; \
             got:\n{rendered}",
        );
        assert!(
            rendered.contains("owner-artifact.present"),
            "[{tag}] the block surfaces the `owner-artifact.present` gate; got:\n{rendered}",
        );
        let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
            .trim()
            .parse()
            .unwrap();
        assert_eq!(
            before, after,
            "[{tag}] a blocked finalize creates no commit"
        );
    }
}

/// (c) + (d) The #5 gate PASSES on an owner-artifact durably staged (present + git-tracked)
/// under `completions/artifacts/<milestone>/`: finalize lands silent (no
/// `owner-artifact.present` finding), and the artifact + the promoted completion-record +
/// the appended decisions-log commit TOGETHER (the same-transaction promotion + the
/// both-halves decisions-log append). The decisions-log triage entry rides into the
/// promoted singleton — this half appends it, proving flow 20 is a both-halves driver.
#[test]
fn flow20_the_gate_passes_on_a_staged_artifact_and_the_decisions_log_is_appended() {
    let repo = TempDir::new("pass");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let milestone = "M16";
    let task = milestone.to_lowercase();
    let artifact = "completions/artifacts/M16/audit.md";

    start_completion(repo.path(), home.path(), milestone);
    // The orchestrator's same-transaction recording: durably stage + `git add` the
    // owner-artifact BEFORE finalize, so validate sees it tracked (the M3 lesson the
    // BLOCK untracked case above proves is load-bearing) — and AFTER the mint, as the
    // task's own work (a pre-mint stage would correctly trip the M43 carryover gate).
    stage_owner_artifact(repo.path(), artifact, "the genuine audit transcript\n");
    let addr = create_completion_record(repo.path(), home.path(), milestone);
    author_meta(repo.path(), home.path(), &addr, "green", artifact);
    author_finding(
        repo.path(),
        home.path(),
        &addr,
        "A confirmed finding",
        "blocking",
        "fixed",
        "engine/src/foo.rs:42",
    );
    // The both-halves driver: a triage decision appended to the running decisions-log.
    append_decision(
        repo.path(),
        home.path(),
        "chose to fix-now the finding",
        b"Because the finding was bounded and confirmed.\n",
    );

    fill_commit(repo.path(), home.path(), &task);

    let out = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("a durably-staged owner-artifact must let finalize land; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("owner-artifact.present"),
        "a durably-staged owner-artifact must surface no owner-artifact finding; got:\n{rendered}",
    );

    // The artifact + the promoted completion-record + the appended decisions-log all land
    // in the SAME commit (the same-transaction promotion).
    let committed_files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed_files.contains(artifact),
        "the owner-artifact lands in the commit; got:\n{committed_files}",
    );
    assert!(
        committed_files.contains("completions/m16.md"),
        "the completion-record is promoted in the same commit; got:\n{committed_files}",
    );
    assert!(
        committed_files.contains("docs/decisions-log.md"),
        "the decisions-log is appended + promoted in the same commit (the both-halves \
         driver); got:\n{committed_files}",
    );

    // (d) The triage decision's reasoning rides into the promoted decisions-log.
    let log = committed(repo.path(), "docs/decisions-log.md");
    assert!(
        log.contains("Because the finding was bounded and confirmed."),
        "the triage decision is appended to the promoted decisions-log; got:\n{log}",
    );
}

/// The composed `evidence` authoring line is RUN VERBATIM (the M3 emitted-bytes contract):
/// the `findings/<id>/evidence` line the `completion` spine emits must itself author the
/// field against the real binary. `evidence` is a `string` FIELD, so the emitted verb must
/// be `set-field` (not `set-slot`) — the reconstructing `author_finding` helper would mask
/// a `set-slot`/`set-field` mismatch, so this test parses the emitted line out of the
/// composed spine, substitutes the `<slug>`/`<id>` placeholders with the real record +
/// minted item, and EXECUTES it. On the broken `set-slot` emission the binary rejects the
/// write (`write.not-present`); the emitted line must succeed.
#[test]
fn flow20_the_emitted_evidence_authoring_line_authors_the_field_verbatim() {
    let repo = TempDir::new("evidence-verbatim");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let milestone = "M16";
    let spine = start_completion(repo.path(), home.path(), milestone);

    // Pull the emitted evidence authoring line out of the composed spine verbatim.
    let emitted = spine
        .lines()
        .find(|l| l.contains("/evidence") && l.trim_start().starts_with("jigc doc "))
        .unwrap_or_else(|| {
            panic!("the spine emits a `findings/<id>/evidence` authoring line; got:\n{spine}")
        })
        .trim();

    let addr = create_completion_record(repo.path(), home.path(), milestone);
    let slug = addr
        .split(':')
        .nth(1)
        .expect("addr is `completion-record:<slug>`");
    let add = jigc_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            &format!("{addr}#findings"),
            "--title",
            "the finding",
        ],
        None,
    );
    assert_ok(&add, "`doc add-item completion-record#findings`");
    let item = String::from_utf8(add.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    // The minted item leaf id (`completion-record:<slug>#findings/<id>` → `<id>`).
    let id = item.rsplit('/').next().expect("item addr ends in `/<id>`");

    // Substitute the spine's `<slug>`/`<id>` placeholders with the real values, then split
    // the line into argv honoring the single quoted `--value "..."`.
    let concrete = emitted.replace("<slug>", slug).replace("<id>", id);
    let argv = split_emitted(&concrete);
    assert_eq!(
        argv.first().map(String::as_str),
        Some("jigc"),
        "emitted line is a `jigc ...` invocation: {concrete}"
    );
    let doc_args: Vec<&str> = argv[2..].iter().map(String::as_str).collect();
    assert_eq!(
        argv.get(1).map(String::as_str),
        Some("doc"),
        "emitted line is a `jigc doc ...` invocation: {concrete}"
    );

    let out = jigc_doc(repo.path(), home.path(), &doc_args, None);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        out.status.success(),
        "the EMITTED `findings/<id>/evidence` authoring line must author the field verbatim \
         (`evidence` is a string FIELD → `set-field`, not `set-slot`); emitted line:\n  \
         {concrete}\nran as `jigc doc {doc_args:?}` and got:\n{rendered}",
    );
}

/// Split an emitted `jigc ...` command line into argv, honoring a single `"..."`-quoted
/// token (the `--value "<...>"` the evidence line carries). Minimal — only the quoting the
/// emitted authoring lines actually use.
fn split_emitted(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;
    let mut started = false;
    for ch in line.chars() {
        match ch {
            '"' => {
                in_quote = !in_quote;
                started = true;
            }
            c if c.is_whitespace() && !in_quote => {
                if started {
                    out.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            c => {
                cur.push(c);
                started = true;
            }
        }
    }
    if started {
        out.push(cur);
    }
    out
}

/// (M17 inc-3 T4) The completion-shaped SEAL: the completion workflow's own designed
/// serial-task shape (`design/finalize.md` → Parallel hand-editing, the 2026-06-12
/// phase-1 amendment; `design/methodology-docs.md` → The engine work, item 3 amendment)
/// proven end-to-end over the REAL methodology pack:
///
///   1. the task is minted at AUDIT-START (the base pins the pre-fix HEAD);
///   2. the audit authors the completion-record but OMITS `owner-artifact:` — the
///      fresh-mint default state the M16 exemption let finalize silently void;
///   3. a fix commit moves HEAD on history DISJOINT from the task's work (a path
///      touching neither the dirty working tree nor the promote destinations
///      `completions/m16.md` / `docs/decisions-log.md` /
///      `completions/artifacts/...`);
///   4. finalize with `owner-artifact` unset exits 3 and surfaces
///      `schema-conformance.field-value-conformant` naming the field (the M40 F1
///      create skeleton pre-stamps the author-required field empty, so the
///      omission is present-but-empty) — it still blocks at CONFORMANCE (never a
///      vacuous pass, never the #5 gate's job), and no commit lands;
///   5. after `set-field meta/owner-artifact` + durably staging the artifact, finalize
///      auto-RE-PINS over the moved (disjoint) history and lands exactly ONE commit
///      promoting the record + the artifact together — the fix commit's own file does
///      not ride in it.
///
/// "Each run recorded as an owner-artifact" is no longer voidable by omission, and
/// mint-at-audit-start / finalize-after-fix-commits works without discard-and-reauthor.
#[test]
fn completion_finalize_after_fix_commits_repins_and_blocks_on_omitted_owner_artifact() {
    let repo = TempDir::new("seal");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let milestone = "M16";
    let task = milestone.to_lowercase();
    let artifact = "completions/artifacts/M16/audit.md";

    // 1. Mint at audit-start: the base pins the PRE-FIX HEAD.
    start_completion(repo.path(), home.path(), milestone);

    // 2. The audit + triage author the record — verdict, a triaged finding, the
    //    decisions-log entry — but the `owner-artifact` field stays UNSET (the fresh-mint
    //    default state).
    let addr = create_completion_record(repo.path(), home.path(), milestone);
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &[
                "set-field",
                &format!("{addr}#meta/verdict"),
                "--value",
                "green",
            ],
            None,
        ),
        "`set-field verdict`",
    );
    author_finding(
        repo.path(),
        home.path(),
        &addr,
        "A confirmed finding",
        "blocking",
        "fixed",
        "engine/src/foo.rs:42",
    );
    append_decision(
        repo.path(),
        home.path(),
        "fix-now the confirmed finding",
        b"Bounded, confirmed, in-scope.\n",
    );
    fill_commit(repo.path(), home.path(), &task);

    // 3. The fix round lands a commit: HEAD moves on history DISJOINT from the task's
    //    work (neither a dirty path nor a promote destination).
    fs::write(repo.path().join("fix.rs"), "// the audit fix\n").expect("write the fix");
    git(repo.path(), &["add", "fix.rs"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "fix: the audit finding"],
    );

    // 4. Finalize with `owner-artifact` unset: the omission blocks at CONFORMANCE —
    //    exit 3, `schema-conformance.field-value-conformant` naming the pre-stamped
    //    empty field (M40 F1), no commit.
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        out.status.code(),
        Some(3),
        "an omitted owner-artifact must block finalize with exit 3 (validation-blocked); \
         got:\n{rendered}",
    );
    assert!(
        rendered.contains("schema-conformance.field-value-conformant"),
        "the omission surfaces `schema-conformance.field-value-conformant` (conformance \
         owns the un-filled field — the M17 flip kept it a gate; the M40 F1 pre-stamp \
         makes it present-but-empty); got:\n{rendered}",
    );
    assert!(
        rendered.contains("owner-artifact"),
        "the block names the missing `owner-artifact` field; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");

    // 5. Set the field + durably stage the artifact, then finalize again: the moved
    //    (disjoint) history auto-RE-PINS and the seal lands as ONE commit.
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &[
                "set-field",
                &format!("{addr}#meta/owner-artifact"),
                "--value",
                artifact,
            ],
            None,
        ),
        "`set-field owner-artifact`",
    );
    stage_owner_artifact(repo.path(), artifact, "the genuine audit transcript\n");

    let out = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!(
            "after set-field + a durably-staged artifact, finalize must re-pin over the \
             disjoint moved history and land; got:\n{rendered}"
        ),
    );

    let sealed: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(
        sealed,
        after + 1,
        "the re-pinned seal lands exactly ONE commit"
    );
    let committed_files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed_files.contains("completions/m16.md"),
        "the completion-record is promoted in the seal commit; got:\n{committed_files}",
    );
    assert!(
        committed_files.contains(artifact),
        "the owner-artifact lands in the same seal commit; got:\n{committed_files}",
    );
    assert!(
        !committed_files.lines().any(|l| l == "fix.rs"),
        "the fix commit's file must NOT ride in the task's seal commit; got:\n{committed_files}",
    );
}

/// (e) The audit `verdict` is an AUTHORED `meta/verdict` field, never an engine opinion:
/// a `red` verdict finalizes exactly as readily as a `green` one. The engine records the
/// agent's verdict and promotes the record; it does NOT certify the audit (the
/// single-agent-spine bound — the verdict stays orchestration-level judgment). The #5
/// presence gate is about the artifact's PRESENCE, orthogonal to the verdict's value.
#[test]
fn flow20_a_red_verdict_is_recorded_not_certified() {
    let repo = TempDir::new("red");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let milestone = "M16";
    let task = milestone.to_lowercase();
    let artifact = "completions/artifacts/M16/audit.md";
    start_completion(repo.path(), home.path(), milestone);
    // Staged after the mint — the artifact is this task's own work (M43 carryover gate).
    stage_owner_artifact(repo.path(), artifact, "a red audit transcript\n");
    let addr = create_completion_record(repo.path(), home.path(), milestone);
    // A RED verdict — the deliverable did NOT hold. The engine must still land it: the
    // verdict is authored prose-judgment, not an engine pass/fail gate.
    author_meta(repo.path(), home.path(), &addr, "red", artifact);
    author_finding(
        repo.path(),
        home.path(),
        &addr,
        "An unresolved blocking finding",
        "blocking",
        "deferred",
        "engine/src/bar.rs:7",
    );
    fill_commit(repo.path(), home.path(), &task);

    let out = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!(
            "a `red` verdict must finalize exactly like a `green` one (the engine records \
             the verdict, it does not certify it); got:\n{rendered}"
        ),
    );
    // The authored `red` verdict survives into the promoted record verbatim.
    let promoted = committed(repo.path(), "completions/m16.md");
    assert!(
        promoted.contains("red"),
        "the authored `red` verdict is promoted verbatim (the verdict is authored, not an \
         engine opinion); got:\n{promoted}",
    );
}
