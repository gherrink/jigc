//! M45 Increment 8 / T1 — the **natural pre-staged authoring order** of a milestone
//! completion lands through the real `jigc` binary (`DECISIONS.md` 2026-07-23 M45 Settle,
//! Decision 5 — owner-artifact both arms; `design/finalize.md` → 5. Stage).
//!
//! The natural order an orchestrator follows is: produce the audit artifact under
//! `completions/artifacts/<M>/`, `git add` it, THEN mint the recording task, author the
//! completion-record naming that path, and `finalize`. Before this increment that order was
//! a **closed route cycle**: the pre-mint `git add` tripped the M43 carryover gate
//! (`finalize.carried-staged`), whose route says to `git restore --staged` — which then
//! trips the owner-artifact presence gate, whose route says to fix the path/field (both
//! already correct). Neither route ever named `git add`.
//!
//! T1 breaks the cycle from the plumbing side: the recorded owner-artifact path is
//! **exempt** from the carryover gate (an owner-artifact staged before the recording task
//! is the task's own subject, never a foreign carry-over), and the presence gate's
//! untracked-cause route now names `git add <path>`. This suite proves the pre-staged order
//! LANDS at exit 0 with the artifact + the promoted record both in HEAD and no
//! `finalize.carried-staged` block. It rides the same FIXTURE `completion-record` pack the
//! `owner_artifact_gate.rs` suite uses (a test pack UNIONed via `.jigc/config/packs.yaml`,
//! never methodology-pack content).

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
            "jigc-owner-natural-{tag}-{}-{:?}",
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

/// Seed the fixture pack: a `completion-record` doctype whose `meta` header carries the
/// engine-native `owned-location` `owner-artifact` field (the #5 gate target) plus a body
/// slot; and a `creates-task: true` workflow `complete` whose create-gate admits it.
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
         usage: proving the pre-staged natural authoring order through the binary.\n\
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
/// clean (the `commit` doctype comes from the base pack in the union).
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

/// Start a `complete` task, author a completion-record naming `owner_artifact`, fill its
/// body slot, and fill the commit doc.
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

/// The HEAD commit count.
fn rev_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap()
}

/// (LANDS) The natural pre-staged authoring order: produce the artifact, `git add` it
/// BEFORE minting the recording task, author the completion-record naming it, and
/// `finalize`. It lands at exit 0 — the recorded owner-artifact is exempt from the carryover
/// gate (no `finalize.carried-staged`), the presence gate is satisfied (present + tracked),
/// and the artifact + the promoted completion-record are committed together.
///
/// RED before T1: the pre-mint `git add` trips `finalize.carried-staged` — the closed route
/// cycle `owner_artifact_gate.rs` documents at its landing case.
#[test]
fn finalize_lands_on_the_pre_staged_natural_order() {
    let repo = TempDir::new("natural");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("pack");
    seed_fixture_pack(pack.path());
    list_fixture_pack(repo.path(), pack.path());

    // Produce the artifact and stage it FIRST — the natural order. A pre-mint stage is
    // exactly what the M43 carryover gate refuses; the owner-artifact exemption is what
    // lets this order through.
    let artifact = "completions/artifacts/M16/audit.md";
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), "the genuine audit transcript\n")
        .expect("write owner-artifact");
    git(repo.path(), &["add", artifact]);

    let task = "record-the-completion";
    author_completion(
        repo.path(),
        home.path(),
        "record the completion",
        task,
        artifact,
    );

    let before = rev_count(repo.path());
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("the pre-staged natural order must let finalize land; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("finalize.carried-staged"),
        "the recorded owner-artifact must be exempt from the carryover gate; got:\n{rendered}",
    );
    assert!(
        !rendered.contains("owner-artifact.present"),
        "a present + tracked owner-artifact must surface no presence finding; got:\n{rendered}",
    );

    let after = rev_count(repo.path());
    assert_eq!(
        after,
        before + 1,
        "the natural order lands exactly one commit"
    );

    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.contains(artifact),
        "the pre-staged owner-artifact lands in the commit; got:\n{committed}",
    );
    assert!(
        committed.contains("completions/m16-completion.md"),
        "the completion-record is promoted in the same commit; got:\n{committed}",
    );
}
