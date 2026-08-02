//! M45 Increment 1 / T6 — the **compose-golden harness**, self-tested over a
//! throwaway golden root (`implementation/pinning.md` §1), plus the **safe
//! state-copy** the mutating golden arms need (§4).
//!
//! **No golden is generated here.** Every arm writes into its own tempdir root, and
//! the first real generation lands in Increment 11 — *after* the wave's
//! surface-changing fixes, so the goldens cannot pin a defect's output as expected
//! (pinning.md → *Sequencing, corrected*). The harness is what a suite is *built on*;
//! this file proves the harness itself before anything depends on it.
//!
//! A golden suite is only worth what its harness refuses, so each arm drives a way
//! the harness could silently pin nothing:
//!
//!   (1) **A real capture round-trips** — bare `jigc start` from `fresh`, whose
//!       orientation line embeds the absolute project-config path (the *only* thing
//!       `<REPO>` normalization exists for). The written golden carries the token and
//!       **no** absolute path, a re-check matches, and a **fresh invocation** of the
//!       same command matches too — the surface's determinism, not just the buffer's.
//!
//!   (2) **A mutated byte fails, readably** — the diff names the golden, the differing
//!       line, and the regen route.
//!
//!   (3) **A missing golden fails** — the failure mode that would make a whole sweep
//!       vacuously green.
//!
//!   (4) **Exit code and stderr are captured**, proven on a `creates-task: false`
//!       member: `jigc workflow router --preview` exits **1** with **empty stdout** and
//!       its entire refusal on stderr. A stdout-only golden would pin an empty file.
//!
//!   (5) **Each of the three channels is pinned independently** — the axis of (4):
//!       changing *only* the exit code, *only* stdout, or *only* stderr each reddens.
//!
//!   (6) **`CI` refuses a regen request** — the insta convention: a regen can never
//!       green CI. Driven over all four combinations of the two variables.
//!
//!   (7) **`copy_state` refuses a worktree-bearing corpus** — a copied
//!       `.git/worktrees/*/gitdir` holds an absolute path back into the *source*, so
//!       the copy reads and writes the original: cross-test contamination, not a clean
//!       failure. On **both** of the guard's axes (7b): the registration directory and
//!       the linked worktree's `.git` pointer file, which are separable.
//!
//!   (8) A plain copy is **provably** independent of its source, in both directions a
//!       sweep arm mutates — the working tree and the git dir.

mod support;

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Output};

use support::goldens::{Capture, GoldenKey, GoldenSuite, REPO_TOKEN, update_mode};
use support::trial_corpus::{State, TrialCorpus};

/// A throwaway golden root that removes itself on drop. Never
/// `crates/cli/tests/goldens/` — this increment generates no golden, and a harness
/// self-test that wrote into the real tree would be doing exactly that.
struct GoldenRoot(PathBuf);

impl GoldenRoot {
    fn new() -> Self {
        let mut root = std::env::temp_dir();
        root.push(format!(
            "jigc-golden-root-{}-{}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&root).expect("create the throwaway golden root");
        GoldenRoot(root)
    }

    /// A suite that **regenerates** into this root.
    fn regenerating(&self) -> GoldenSuite {
        GoldenSuite::new(&self.0, true)
    }

    /// A suite that **checks** against this root.
    fn checking(&self) -> GoldenSuite {
        GoldenSuite::new(&self.0, false)
    }
}

impl Drop for GoldenRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Run `f`, require it to panic, and return the panic message.
fn panic_message(f: impl FnOnce()) -> String {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(()) => panic!("expected a panic; the call returned normally"),
        Err(payload) => payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string panic payload>".to_string()),
    }
}

/// A synthetic [`Output`] — the only way to vary **one** captured channel while
/// holding the other two fixed (arm 5). Every other arm captures a real invocation.
fn synthetic(code: i32, stdout: &str, stderr: &str) -> Output {
    use std::os::unix::process::ExitStatusExt;
    Output {
        status: ExitStatus::from_raw(code << 8),
        stdout: stdout.as_bytes().to_vec(),
        stderr: stderr.as_bytes().to_vec(),
    }
}

/// (1) A real capture round-trips: written with the repo path normalized away,
/// matched on re-check, and matched again by a **fresh invocation** of the same
/// command.
#[test]
fn a_real_capture_round_trips_with_the_repo_path_normalized() {
    let corpus = TrialCorpus::build(State::Fresh);
    let root = GoldenRoot::new();
    let key = GoldenKey {
        pack: "dev",
        surface: "start",
        member: "orientation",
        state: State::Fresh.name(),
    };

    let first = Capture::of(&corpus.jigc(&["start"]), &corpus.repo());
    let absolute = corpus.repo().display().to_string();
    assert!(
        String::from_utf8_lossy(&corpus.jigc(&["start"]).stdout).contains(&absolute),
        "the subject must really embed the absolute repo path, or the \
         normalization arm proves nothing",
    );

    root.regenerating().check(&key, &first);

    let path = root.checking().path_of(&key);
    assert_eq!(
        path,
        root.0.join("compose/dev/start--orientation--fresh.txt"),
        "the layout is `<root>/compose/<pack>/<surface>--<member>--<state>.txt`, \
         per-member so diffs are per-surface and regen writes never contend",
    );
    let written = std::fs::read_to_string(&path).expect("the regen wrote the golden");
    assert!(
        written.contains(REPO_TOKEN),
        "the absolute project-config path must normalize to `{REPO_TOKEN}`:\n{written}",
    );
    assert!(
        !written.contains(&absolute),
        "no absolute repo path may reach a golden:\n{written}",
    );

    // The same bytes re-check clean...
    root.checking().check(&key, &first);
    // ...and so does a fresh invocation of the same command.
    let second = Capture::of(&corpus.jigc(&["start"]), &corpus.repo());
    root.checking().check(&key, &second);
}

/// (2) One mutated byte reddens, and the failure is readable: the golden's path, the
/// differing line, and the regen route.
#[test]
fn a_mutated_byte_fails_with_a_readable_diff() {
    let corpus = TrialCorpus::build(State::Fresh);
    let root = GoldenRoot::new();
    let key = GoldenKey {
        pack: "dev",
        surface: "start",
        member: "orientation",
        state: State::Fresh.name(),
    };
    let capture = Capture::of(&corpus.jigc(&["start"]), &corpus.repo());
    root.regenerating().check(&key, &capture);

    // Mutate the *golden* — a stale expectation is the real-world shape of this
    // failure (a pack edit moved the surface, the golden did not follow).
    let path = root.checking().path_of(&key);
    let stale = std::fs::read_to_string(&path)
        .expect("the regen wrote the golden")
        .replace("Available workflows:", "Available workflowz:");
    std::fs::write(&path, &stale).expect("write the stale golden");

    let message = panic_message(|| root.checking().check(&key, &capture));
    assert!(
        message.contains(&path.display().to_string()),
        "the failure must name the golden it read:\n{message}",
    );
    assert!(
        message.contains("Available workflowz:") && message.contains("Available workflows:"),
        "the failure must show BOTH sides of the differing line:\n{message}",
    );
    assert!(
        message.contains("UPDATE_GOLDENS=1"),
        "the failure must name the regen route:\n{message}",
    );

    // The other length relation: a golden **shorter** than the capture, where the
    // differing "line" is the golden's *absence* of one — a diff that printed a bare
    // empty side there would read as a blank-line change.
    std::fs::write(&path, "exit: 0\n").expect("write the truncated golden");
    let message = panic_message(|| root.checking().check(&key, &capture));
    assert!(
        message.contains("<end of output>") && message.contains("UPDATE_GOLDENS=1"),
        "a golden shorter than the capture must still diff readably:\n{message}",
    );
}

/// (3) A missing golden **fails**. A harness that treats "no file" as "nothing to
/// compare" turns a whole sweep green while pinning nothing.
#[test]
fn a_missing_golden_fails_rather_than_silently_passing() {
    let corpus = TrialCorpus::build(State::Fresh);
    let root = GoldenRoot::new();
    let key = GoldenKey {
        pack: "dev",
        surface: "start",
        member: "never-generated",
        state: State::Fresh.name(),
    };
    let capture = Capture::of(&corpus.jigc(&["start"]), &corpus.repo());

    let path = root.checking().path_of(&key);
    assert!(!path.exists(), "the arm needs a genuinely absent golden");
    let message = panic_message(|| root.checking().check(&key, &capture));
    assert!(
        message.contains(&path.display().to_string()) && message.contains("UPDATE_GOLDENS=1"),
        "the missing-golden failure must name the path and the regen route:\n{message}",
    );
}

/// (4) Exit code **and** stderr are captured — proven on a `creates-task: false`
/// member, whose whole surface is the refusal on stderr: `jigc workflow router
/// --preview` exits 1 with empty stdout. A stdout-only golden would pin an empty
/// file forever.
#[test]
fn the_exit_code_and_stderr_of_a_mint_refusing_member_are_captured() {
    let corpus = TrialCorpus::build(State::Fresh);
    let out = corpus.jigc(&["workflow", "router", "--preview"]);

    assert_eq!(out.status.code(), Some(1), "the refusal exits 1");
    assert!(
        out.stdout.is_empty(),
        "the refusal writes nothing to stdout — which is what makes the capture's \
         other two channels load-bearing",
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(!stderr.trim().is_empty(), "the refusal rides stderr");

    let root = GoldenRoot::new();
    let key = GoldenKey {
        pack: "dev",
        surface: "workflow-preview",
        member: "router",
        state: State::Fresh.name(),
    };
    let capture = Capture::of(&out, &corpus.repo());
    root.regenerating().check(&key, &capture);

    let written =
        std::fs::read_to_string(root.checking().path_of(&key)).expect("the regen wrote the golden");
    assert!(
        written.contains("exit: 1"),
        "the golden must carry the exit code:\n{written}",
    );
    assert!(
        written.contains(stderr.trim()),
        "the golden must carry the stderr surface:\n{written}",
    );
}

/// (5) The axis of (4): each of the three captured channels is pinned
/// **independently** — a golden that folded any of them away would stay green here.
#[test]
fn each_captured_channel_is_pinned_independently() {
    let root = GoldenRoot::new();
    let repo = Path::new("/nowhere/that/appears");
    let key = GoldenKey {
        pack: "dev",
        surface: "synthetic",
        member: "channels",
        state: "fresh",
    };

    let baseline = Capture::of(&synthetic(0, "out\n", "err\n"), repo);
    root.regenerating().check(&key, &baseline);
    root.checking().check(&key, &baseline);

    for (label, mutated) in [
        ("exit code", synthetic(1, "out\n", "err\n")),
        ("stdout", synthetic(0, "OUT\n", "err\n")),
        ("stderr", synthetic(0, "out\n", "ERR\n")),
    ] {
        let capture = Capture::of(&mutated, repo);
        let message = panic_message(|| root.checking().check(&key, &capture));
        assert!(
            message.contains("mismatch"),
            "a changed {label} must redden the golden; got:\n{message}",
        );
    }
}

/// (5b) A **file-render** capture ([`Capture::of_file`], the AGENT.md arm) is the
/// file's bytes alone — no invocation framing — under the same single repo-path
/// normalization, and a mutated byte reddens exactly like an invocation capture.
#[test]
fn a_file_render_capture_is_the_normalized_bytes_alone() {
    let root = GoldenRoot::new();
    let repo = Path::new("/nowhere/that/appears");
    let key = GoldenKey {
        pack: "composite",
        surface: "agent-md",
        member: "agent-md",
        state: "fresh",
    };

    let body = format!("bootstrap prose citing {}/x.md\n", repo.display());
    let baseline = Capture::of_file(&body, repo);
    root.regenerating().check(&key, &baseline);
    root.checking().check(&key, &baseline);

    let written =
        std::fs::read_to_string(root.checking().path_of(&key)).expect("the regen wrote the golden");
    assert_eq!(
        written,
        format!("bootstrap prose citing {REPO_TOKEN}/x.md\n"),
        "a file-render golden is the file's bytes with the repo path normalized — \
         no `exit:` line, no stream framing",
    );

    let mutated = Capture::of_file("bootstrap prose, mutated\n", repo);
    let message = panic_message(|| root.checking().check(&key, &mutated));
    assert!(
        message.contains("mismatch"),
        "a changed file render must redden the golden; got:\n{message}",
    );
}

/// (6) `CI` refuses a regen request, over all four combinations — a regen can never
/// green CI, and no other combination is disturbed.
#[test]
fn a_regen_request_is_refused_under_ci() {
    assert!(
        !update_mode(None, None),
        "an ordinary local run checks, never regenerates",
    );
    assert!(
        !update_mode(None, Some("true")),
        "CI alone is not a regen request",
    );
    assert!(
        update_mode(Some("1"), None),
        "`UPDATE_GOLDENS=1` locally regenerates",
    );

    let message = panic_message(|| {
        update_mode(Some("1"), Some("true"));
    });
    assert!(
        message.contains("UPDATE_GOLDENS") && message.contains("CI"),
        "the refusal must name both variables:\n{message}",
    );
}

/// (7) `copy_state` refuses a worktree-bearing corpus, naming the reason. The copy's
/// `.git/worktrees/*/gitdir` would point back at the **source**, so the copy would
/// silently read and write the original — contamination, not a clean failure.
#[test]
fn copy_state_refuses_a_worktree_bearing_corpus() {
    let corpus = TrialCorpus::build(State::Fresh);
    corpus.git(&["worktree", "add", "-q", "-b", "probe", "../probe-worktree"]);
    assert!(
        corpus.repo().join(".git/worktrees").is_dir(),
        "the arm needs a really provisioned worktree",
    );

    let message = panic_message(|| {
        let _ = corpus.copy_state();
    });
    assert!(
        message.contains("worktree"),
        "the refusal must name the worktree hazard:\n{message}",
    );
    assert!(
        message.to_lowercase().contains("absolute"),
        "the refusal must state WHY — the absolute paths back into the source:\n{message}",
    );
}

/// (7b) The guard's **second axis**: a linked worktree's `.git` **file** is refused
/// too, not only the source's `.git/worktrees/` registration. The two are separable
/// — a pruned registration, or a worktree belonging to another repo, leaves the
/// pointer file with the registration gone — and a guard that covered only the
/// registration would copy an absolute pointer into the source's git dir.
#[test]
fn copy_state_refuses_a_worktree_pointer_file_with_its_registration_gone() {
    let corpus = TrialCorpus::build(State::Fresh);
    corpus.git(&["worktree", "add", "-q", "-b", "probe", "../probe-worktree"]);
    std::fs::remove_dir_all(corpus.repo().join(".git/worktrees"))
        .expect("prune the worktree registration");

    let pointer = corpus.repo().join("../probe-worktree/.git");
    assert!(
        pointer.is_file(),
        "the arm needs the linked worktree's `.git` pointer FILE to survive",
    );

    let message = panic_message(|| {
        let _ = corpus.copy_state();
    });
    assert!(
        message.contains(".git") && message.to_lowercase().contains("absolute"),
        "the refusal must name the pointer file and its absolute-path reason:\n{message}",
    );
}

/// (8) A plain copy is provably independent of its source, in both directions a
/// golden arm mutates: the **working tree** (a minted task) and the **git dir** (a
/// commit).
#[test]
fn a_copied_state_is_independent_of_its_source() {
    let corpus = TrialCorpus::build(State::Fresh);
    let copy = corpus.copy_state();
    assert_eq!(copy.state(), corpus.state());

    // The copy carries the built state — a copy of nothing would pass the
    // independence checks vacuously.
    assert!(
        copy.repo().join(".jigc").is_dir(),
        "the copy must carry the source's `.jigc/` workbench",
    );

    // Working tree: mint a task in the copy.
    let task = copy.start_workflow("single-task", "probe the copied state");
    assert!(
        copy.repo().join(format!(".jigc/tasks/{task}")).is_dir(),
        "the mint must land in the copy",
    );
    assert!(
        !corpus.repo().join(format!(".jigc/tasks/{task}")).exists(),
        "the mint must NOT reach the source",
    );

    // Git dir: commit in the copy.
    let before = corpus.git(&["rev-list", "--count", "HEAD"]);
    copy.git(&[
        "commit",
        "--allow-empty",
        "--no-verify",
        "-q",
        "-m",
        "in the copy only",
    ]);
    assert_eq!(
        corpus.git(&["rev-list", "--count", "HEAD"]),
        before,
        "the copy's commit must not reach the source's git dir",
    );
    assert_ne!(
        copy.git(&["rev-list", "--count", "HEAD"]),
        before,
        "the copy's own history must have advanced",
    );
}
