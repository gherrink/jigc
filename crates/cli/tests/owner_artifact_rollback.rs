//! M45 Increment 8 / T2 — the **third rollback axis** for owner-artifact staging, proven
//! end-to-end through the real `jigc` binary (`design/finalize.md` → Rollback discipline,
//! the owner-artifact row; 5. Stage). Since T2, finalize **stages** each recorded
//! owner-artifact in-transaction — so jigc's own `git add` can *overwrite* a user's
//! pre-staged index entry. The rollback must therefore capture each owner-artifact path's
//! **pre-finalize index entry** before staging and restore *that* on a stage/commit failure —
//! NOT "un-stage what jigc staged", which loses the user's pre-staged blob.
//!
//! The proof: a user pre-stages blob `A` at the owner-artifact path (exempt from the M43
//! carryover gate — the natural authoring order), then edits the file to `B` on disk, and a
//! rejecting `.git/hooks/pre-commit` aborts the commit. After rollback the **staged blob is
//! byte-identical to the pre-finalize `A`** (`git show :<path>` == `A`) — not dropped to
//! HEAD, not jigc's overwrite `B`. Rides the same FIXTURE `completion-record` pack + real
//! binary (`CARGO_BIN_EXE_jigc`) mold as `owner_artifact_gate.rs`.

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
            "jigc-owner-rollback-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning raw stdout as a String.
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

/// Seed the fixture pack (identical to `owner_artifact_gate.rs`): a `completion-record`
/// doctype whose `meta` header carries the engine-native `owned-location` `owner-artifact`
/// field, plus a `creates-task: true` `complete` workflow whose create-gate admits it.
fn seed_fixture_pack(pack: &Path) {
    let schemas = pack.join("schemas");
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&schemas, &workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk fixture pack subdir");
    }

    fs::write(
        schemas.join("completion-record.yaml"),
        "type: completion-record\n\
         location: completions/\n\
         id-from: title\n\
         description: A fixture completion record carrying an owner-artifact owned location.\n\
         usage: the test substrate for the #5 owner-artifact presence gate.\n\
         sections:\n\
         \x20 - id: meta\n\
         \x20   header: true\n\
         \x20   fields:\n\
         \x20     - { id: owner-artifact, type: owned-location }\n\
         \x20 - id: body\n\
         \x20   slot: {}\n",
    )
    .expect("seed completion-record schema");

    fs::write(
        workflows.join("complete.yaml"),
        "---\n\
         when: record a milestone completion\n\
         description: A fixture workflow that creates a completion-record.\n\
         usage: proving the owner-artifact staging rollback through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: completion-record, as: record}]\n\
         ---\n\
         {{ include: step:audit }}\n",
    )
    .expect("seed complete workflow");
    fs::write(
        steps.join("audit.yaml"),
        "Record the milestone completion for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed audit step");

    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    // A fixture pack that ships steps owes the four ambush-class statements since M51
    // Increment 8 T2: the stated-at fence's structural tier keys on the constituents
    // that SHIP STEPS and checks each in isolation.
    crate::support::seed_ambush_class_declarer(pack);
}

/// Record the fixture pack in the in-repo project layer's `packs.yaml` (UNIONs with the base).
fn list_fixture_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the fixture pack");
}

/// Fill the commit doc so a finalize over it validates clean — leaving the owner-artifact
/// staging + rollback as the only lever the test turns.
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
    set_slot(
        &format!("commit:{task}#summary"),
        b"record the completion\n",
    );
    set_slot(&format!("commit:{task}#body"), b"A milestone completion.\n");
}

/// Start a `complete` task, author a completion-record naming `owner_artifact`, fill the commit.
fn author_completion(repo: &Path, home: &Path, intent: &str, task: &str, owner_artifact: &str) {
    assert_ok(
        &jigc(repo, home, &["start", "--workflow", "complete", intent]),
        "`jigc start --workflow complete`",
    );
    let create = jigc_doc(
        repo,
        home,
        &["create", "completion-record", "--title", "M16 completion"],
        None,
    );
    assert_ok(&create, "`doc create completion-record`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();

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
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-slot", &format!("{addr}#body"), "--from-file", "-"],
            Some(b"The audit landed green.\n"),
        ),
        "`set-slot body`",
    );
    fill_commit(repo, home, task);
}

/// Install a `.git/hooks/pre-commit` that always rejects (exit 1) so `git commit` aborts —
/// the correction-signal path finalize rolls back from.
fn install_rejecting_pre_commit(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        "#!/bin/sh\necho 'rejected by test hook' 1>&2\nexit 1\n",
    )
    .expect("write pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).expect("chmod hook");
    }
}

/// (M45 Inc 8 T2 — the third rollback axis) A user **pre-stages blob `A`** at the
/// owner-artifact path (exempt from the carryover gate, so no `finalize.carried-staged`),
/// then **edits the file to `B`** on disk; a rejecting `pre-commit` aborts the commit. After
/// rollback the **staged index entry is byte-identical to the pre-finalize `A`** — not
/// dropped to HEAD, not jigc's overwrite `B` — and HEAD is unchanged. Red without the
/// capture-and-restore axis: jigc's stage overwrote the index with `B`, so `A` is lost.
#[cfg(unix)]
#[test]
fn a_hook_rejected_finalize_restores_the_pre_staged_blob() {
    let repo = TempDir::new("blobA");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    let artifact = "completions/artifacts/M16/audit.md";
    let blob_a = "audit transcript — version A\n";
    let blob_b = "audit transcript — version B (edited on disk)\n";

    // The user PRE-STAGES blob A (before the recording task is minted — the natural order the
    // carryover exemption exists for).
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), blob_a).expect("write blob A");
    git(repo.path(), &["add", artifact]);

    let task = "record-the-completion";
    author_completion(
        repo.path(),
        home.path(),
        "record the completion",
        task,
        artifact,
    );

    // The user then EDITS the file to B on disk (worktree = B, index still = A). jigc's
    // phase-5 `git add` will stage B over A — the exact overwrite the third axis must undo.
    fs::write(repo.path().join(artifact), blob_b).expect("edit to blob B");

    // Capture the pre-finalize staged blob so the assertion compares bytes, not our literal.
    let staged_before = git(repo.path(), &["show", &format!(":{artifact}")]);
    assert_eq!(
        staged_before, blob_a,
        "precondition: the staged blob is A going in"
    );

    install_rejecting_pre_commit(repo.path());

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a rejecting pre-commit must make finalize exit non-zero; got:\n{rendered}",
    );

    // No commit landed.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a hook-rejected finalize creates no commit");

    // The staged index entry is byte-identical to the pre-finalize A — the third axis restored
    // it. NOT B (jigc's overwrite), NOT dropped-to-HEAD (the wrong "un-stage" primitive would
    // leave the path absent from the index, since it is not at HEAD).
    let staged_after = git(repo.path(), &["show", &format!(":{artifact}")]);
    assert_eq!(
        staged_after, blob_a,
        "after rollback `git show :{artifact}` must equal the pre-finalize A, got:\n{staged_after}",
    );
    assert_ne!(
        staged_after, blob_b,
        "the staged blob must NOT be jigc's overwrite B",
    );

    // The worktree file is untouched by the rollback — it stays the user's on-disk B.
    let worktree = fs::read_to_string(repo.path().join(artifact)).expect("read worktree file");
    assert_eq!(
        worktree, blob_b,
        "the rollback restores only the index; the worktree stays the user's B",
    );
}
