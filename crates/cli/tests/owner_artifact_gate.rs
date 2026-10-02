//! M16 Increment 3 / T2 — the intrinsic **#5 owner-artifact presence gate** proven
//! end-to-end through the real `jigc` binary (`design/methodology-docs.md` → The engine
//! work, item 3; `milestone-completion-workflow.md` → the spawn-class third audit
//! artifact). The gate is engine-emitted inside `validate_task`, so a `task finalize`
//! (which gates on exactly what `validate` reports — `plan_finalize` phase 2) BLOCKS when
//! the named owner-artifact is absent/unsafe/untracked and LANDS when it is durably staged
//! under `completions/artifacts/<milestone>/`.
//!
//! The proof rides a **FIXTURE** `completion-record` doctype carrying an engine-native
//! `owned-location` `owner-artifact` field — a test pack, never methodology-pack content
//! (the real completion-record lands in increment 5). The fixture pack is **listed** in
//! the in-repo `.jigc/config/packs.yaml` so it UNIONs with the embedded base pack (the
//! flow-18 mechanism): the base supplies `commit`/knobs/defaults, the fixture supplies its
//! `completion-record` doctype + a `creates-task: true` workflow whose create-gate admits
//! it. Every assertion runs on the EMITTED bytes / exit code of the real binary
//! (`CARGO_BIN_EXE_jigc`) over a throwaway `git init` repo.
//!
//! The gate reads the field string + the file's presence + git tracked-status — never the
//! artifact's bytes (the determinism boundary: presence, not content). The honest bound:
//! it proves an artifact is present at the named owned path, NOT that the genuine audit
//! happened (the orchestrator writes both the field and the file).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-owner-artifact-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output. NO
/// `JIGC_PACK_DIR`: the embedded base pack stays in the union, and the listed fixture pack
/// (named in `packs.yaml`) composes alongside it (the flow-18 listed-pack seam).
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

/// Seed the fixture pack: a `completion-record` doctype whose `meta` header carries the
/// engine-native `owned-location` `owner-artifact` field (the #5 gate target) plus a body
/// slot; and a `creates-task: true` workflow `complete` whose create-gate admits it. The
/// fixture references no `{{cli.X}}` command, so an empty catalog satisfies the read.
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
         usage: proving the #5 owner-artifact presence gate through the binary.\n\
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

/// Record the fixture pack in the in-repo project layer's `packs.yaml`, UNIONing it with
/// the embedded base pack (the flow-18 listed-pack mechanism).
fn list_fixture_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the fixture pack");
}

/// Fill the commit doc's author-required header + prose so a finalize over it validates
/// clean (the `commit` doctype comes from the base pack in the union) — leaving the
/// owner-artifact gate as the only lever the absent/staged cases turn.
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

/// Start a `complete` task for `intent`, author a completion-record naming `owner_artifact`
/// (set on the `meta` header) and fill its body slot, then fill the commit doc.
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

    // The owner-artifact owned-location path (set on the meta header).
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
    // The required body slot.
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

/// (BLOCKS) A completion-record naming an owner-artifact that is **absent** (no file
/// staged under the owned home) makes `task finalize` exit non-zero with the engine-emitted
/// `owner-artifact.present` block, and lands NO commit.
#[test]
fn finalize_blocks_on_an_absent_owner_artifact() {
    let repo = TempDir::new("absent");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    let task = "record-the-completion";
    author_completion(
        repo.path(),
        home.path(),
        "record the completion",
        task,
        // A well-shaped owned-home path whose file is never created → absent.
        "completions/artifacts/M16/audit.md",
    );

    // (M47 Inc 4 / T2 — Decision 1, narrowing M45 Decision 6 to its own stated grounds)
    // The absent-artifact state is the **names-no-file** cause, which never consults
    // `tracked` — so no stage can change its verdict and `jigc task validate` PREVIEWS it
    // rather than reporting a green the committing door refuses. The relocation Decision 6
    // made still holds for cause 7 (untracked), which stays post-stage; the whole seven-cause
    // axis is swept in `owner_artifact_cause_axis.rs`.
    let validate = jigc(repo.path(), home.path(), &["task", "validate", task]);
    assert_eq!(
        validate.status.code(),
        Some(3),
        "`task validate` previews the absent-artifact block at exit 3; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr),
    );
    assert!(
        String::from_utf8_lossy(&validate.stdout).contains("owner-artifact.present"),
        "the preview names the gate it previews; stdout:\n{}",
        String::from_utf8_lossy(&validate.stdout),
    );

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    // The gate blocks AFTER the stage phase, so the shared rollback must run — snapshot
    // the pre-finalize index it must return to (confidence-audit minor item 4: the
    // post-stage gate-block failure point was block-asserted but never rollback-asserted).
    let index_before = git(repo.path(), &["ls-files", "--stage"]);
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "an absent owner-artifact must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("owner-artifact.present"),
        "the block surfaces the owner-artifact.present check; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
    // The rollback assertion: the index is byte-identical to its pre-finalize state —
    // the post-stage gate block runs the same shared rollback as a hook rejection, so
    // nothing jigc's own stage contributed (promotion, config layer, version stamp) may
    // stay staged.
    assert_eq!(
        git(repo.path(), &["ls-files", "--stage"]),
        index_before,
        "a post-stage gate block must leave the index byte-identical to its \
         pre-finalize state",
    );
}

/// (LANDS — the stage arm) A completion-record naming an owner-artifact that is **present on
/// disk but never `git add`ed** by the agent (produced, not staged) finalizes clean: phase-5
/// **stages** the recorded path in-transaction, so the post-stage gate passes and the commit
/// carries the artifact + the promoted completion-record together (`design/finalize.md` → 5.
/// Stage: finalize stages the recorded paths). Red before T2: the pre-stage gate blocked on
/// the untracked artifact (nothing staged it).
#[test]
fn finalize_stages_a_produced_but_unstaged_owner_artifact_and_lands() {
    let repo = TempDir::new("produced");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    let artifact = "completions/artifacts/M16/audit.md";
    let task = "record-the-completion";
    author_completion(
        repo.path(),
        home.path(),
        "record the completion",
        task,
        artifact,
    );

    // The agent PRODUCES the artifact on disk but never `git add`s it — the case finalize's
    // in-transaction staging exists to close. Written after the mint (its own work).
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), "the genuine audit transcript\n")
        .expect("write owner-artifact");
    // Sanity: it is genuinely untracked (never staged) going in.
    let others = git(repo.path(), &["ls-files", "--others", "--", artifact]);
    assert!(
        others.contains("audit.md"),
        "the artifact must be untracked before finalize (the produced-but-unstaged case); got:\n{others}",
    );

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("finalize must STAGE the produced artifact and land; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("owner-artifact.present"),
        "the staged-in-transaction artifact must surface no owner-artifact finding; got:\n{rendered}",
    );

    // The artifact and the promoted completion-record are committed together.
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.contains(artifact),
        "the produced owner-artifact lands in the commit (finalize staged it); got:\n{committed}",
    );
    assert!(
        committed.contains("completions/m16-completion.md"),
        "the completion-record is promoted in the same commit; got:\n{committed}",
    );
}

/// (BLOCKS) A completion-record naming an owner-artifact that is **present but
/// gitignored** (so it is in neither git's index nor the untracked-non-ignored set) must
/// make `task finalize` exit non-zero with the `owner-artifact.present` block: a gitignored
/// file is not durably committable, so it cannot satisfy the durable-presence gate even
/// though it exists on disk.
#[test]
fn finalize_blocks_on_a_gitignored_owner_artifact() {
    let repo = TempDir::new("gitignored");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    // The owner-artifact is present on disk under the owned home, but a `.gitignore`
    // pattern covers it — so it is neither tracked nor stageable (`git add` would skip
    // it), defeating the gate's durable-presence guarantee if it were treated as tracked.
    let artifact = "completions/artifacts/M16/audit.md";
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), "the genuine audit transcript\n")
        .expect("write owner-artifact");
    fs::write(
        repo.path().join(".gitignore"),
        "completions/artifacts/M16/\n",
    )
    .expect("write .gitignore covering the owned home");

    let task = "record-the-completion";
    author_completion(
        repo.path(),
        home.path(),
        "record the completion",
        task,
        artifact,
    );

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
        "a gitignored owner-artifact must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("owner-artifact.present"),
        "the block surfaces the owner-artifact.present check; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
}

/// (LANDS) A completion-record naming an owner-artifact **durably staged** under
/// `completions/artifacts/<milestone>/` (present + git-tracked) finalizes clean: the gate
/// is silent, the commit lands, and the artifact + the promoted completion-record are
/// committed together (the same-transaction promotion).
#[test]
fn finalize_lands_on_a_staged_owner_artifact() {
    let repo = TempDir::new("staged");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    let artifact = "completions/artifacts/M16/audit.md";
    let task = "record-the-completion";
    author_completion(
        repo.path(),
        home.path(),
        "record the completion",
        task,
        artifact,
    );

    // The owner-artifact is durably staged under the owned home and git-tracked (the
    // orchestrator's same-transaction recording — proven via `git add`, made explicit
    // so validate sees it tracked before the commit lands). Staged AFTER the mint, as
    // the task's own work (a pre-mint stage would correctly trip the M43 carryover
    // gate).
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), "the genuine audit transcript\n")
        .expect("write owner-artifact");
    git(repo.path(), &["add", artifact]);

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("a staged + tracked owner-artifact must let finalize land; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("owner-artifact.present"),
        "a durably-staged owner-artifact must surface no owner-artifact finding; got:\n{rendered}",
    );

    // The artifact and the promoted completion-record are committed together.
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.contains(artifact),
        "the owner-artifact lands in the commit; got:\n{committed}",
    );
    assert!(
        committed.contains("completions/m16-completion.md"),
        "the completion-record is promoted in the same commit; got:\n{committed}",
    );
}
