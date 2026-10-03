//! **The coding agent's co-author trailer** — every commit jigc makes while running under the
//! coding agent its adapter profile names carries that agent's `Co-Authored-By:` trailer,
//! exactly once, and no commit a human makes by hand carries it
//! (`design/assistant-adapter.md` → The co-author trailer; `design/finalize.md` →
//! Commit-doc rendering).
//!
//! The trailer is **profile-declared and session-keyed**: the shipped Claude Code profile
//! declares `co-author: {name: Claude, email: noreply@anthropic.com, when-env: CLAUDECODE}`,
//! and the one hook-capable commit seam appends it only when `CLAUDECODE` is set and non-empty
//! in jigc's own environment — the variable Claude Code sets for the processes its agent runs.
//! A trailer claiming an agent co-authored a commit a human made alone would be a law-1 lie
//! (`design/surface-contract.md`), so the absent-variable cell is as load-bearing as the
//! present one.
//!
//! **The door set is read from the code, not restated:** the sweeps iterate
//! [`COMMITTING_DOORS`] through the shared committing-door fixtures, so a door added to the
//! axis is covered here the moment it has a fixture there. `jigc setup`'s install commit — the
//! axis's one exclusion, `--no-verify` by recorded design — is driven on its own.
//!
//! **The count is git's, not ours:** every assertion reads the landed commit through
//! `%(trailers:key=Co-Authored-By,valueonly)`, git's own trailer parser, so a trailer jigc
//! appended outside the message's trailer block — where git would not read it as one — fails
//! here rather than passing on a substring.
//!
//! **The test environment is neutral by construction:** `.cargo/config.toml` forces
//! `CLAUDECODE` empty for every process cargo runs, so a suite run from inside a Claude Code
//! session and one run on a CI runner see the same binary behaviour; each cell below sets or
//! removes the variable itself.

use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use cli::invocation_log::COMMITTING_DOORS;

use crate::support::committing_doors::{
    TempDir, adr_body, base_repo, drive, fill_commit_doc, git, jigc_ok, remove_hook, seed_task,
    stage_subtask_doc,
};

/// The trailer value the shipped profile declares.
const CLAUDE: &str = "Claude <noreply@anthropic.com>";

/// The environment variable the shipped profile keys the trailer on.
const AGENT_ENV: &str = "CLAUDECODE";

/// Who is running jigc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Session {
    /// Under Claude Code — `CLAUDECODE=1`, as Claude Code sets it.
    Agent,
    /// A human at a terminal — the variable absent.
    Human,
}

/// Run `jigc <args>` in `repo` as `session`, optionally against an on-disk adapter profile
/// directory (the `JIGC_ADAPTERS_DIR` seam).
fn run(
    repo: &Path,
    home: &Path,
    args: &[&str],
    session: Session,
    adapters: Option<&Path>,
) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    match session {
        Session::Agent => command.env(AGENT_ENV, "1"),
        Session::Human => command.env_remove(AGENT_ENV),
    };
    match adapters {
        Some(dir) => command.env("JIGC_ADAPTERS_DIR", dir),
        None => command.env_remove("JIGC_ADAPTERS_DIR"),
    };
    command.output().expect("spawn the jigc binary")
}

fn run_ok(repo: &Path, home: &Path, args: &[&str], session: Session, adapters: Option<&Path>) {
    let out = run(repo, home, args, session, adapters);
    assert!(
        out.status.success(),
        "`jigc {}` ({session:?}) must exit 0; stdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

fn head(repo: &Path) -> String {
    git(repo, &["rev-parse", "HEAD"]).trim().to_owned()
}

/// Every commit in `from..HEAD`, oldest first.
fn landed_since(repo: &Path, from: &str) -> Vec<String> {
    git(repo, &["rev-list", "--reverse", &format!("{from}..HEAD")])
        .lines()
        .map(str::to_owned)
        .collect()
}

/// The `Co-Authored-By` trailer values git itself reads off `sha`'s message.
fn co_authors(repo: &Path, sha: &str) -> Vec<String> {
    git(
        repo,
        &[
            "show",
            "-s",
            "--format=%(trailers:key=Co-Authored-By,valueonly)",
            sha,
        ],
    )
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .map(str::to_owned)
    .collect()
}

fn message(repo: &Path, sha: &str) -> String {
    git(repo, &["show", "-s", "--format=%B", sha])
}

/// `sha` carries the agent trailer exactly once, as a git trailer, and its subject did not
/// move (the trailer never touches the Conventional-Commits header release-plz reads).
fn assert_carries_once(repo: &Path, sha: &str, what: &str) {
    let found = co_authors(repo, sha);
    assert_eq!(
        found
            .iter()
            .filter(|value| value.as_str() == CLAUDE)
            .count(),
        1,
        "{what}: commit {sha} must carry `Co-Authored-By: {CLAUDE}` exactly once, as a git \
         trailer; git read {found:?} off:\n{}",
        message(repo, sha),
    );
    let raw = message(repo, sha);
    assert_eq!(
        raw.matches(&format!("Co-Authored-By: {CLAUDE}")).count(),
        1,
        "{what}: the trailer line appears once in the raw message:\n{raw}",
    );
    assert!(
        !raw.lines()
            .next()
            .unwrap_or_default()
            .contains("Co-Authored-By"),
        "{what}: the subject line is untouched:\n{raw}",
    );
}

fn assert_carries_none(repo: &Path, sha: &str, what: &str) {
    let raw = message(repo, sha);
    assert!(
        !raw.contains("Co-Authored-By"),
        "{what}: commit {sha} must carry no co-author trailer:\n{raw}",
    );
}

/// **(1)** Every committing door, run under the agent, lands commits that each carry the
/// trailer exactly once — and **(2)** the same doors run by a human land none.
#[test]
fn every_committing_door_carries_the_agent_trailer_only_under_the_agent() {
    for session in [Session::Agent, Session::Human] {
        for door in COMMITTING_DOORS {
            let case = drive(door.verb);
            let repo = case.repo.path();
            let home = case.home.path();
            // The fixture installs a rejecting hook for the rejection axis; this suite wants
            // the commit to land.
            remove_hook(repo);
            let before = head(repo);
            let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();
            run_ok(repo, home, &driven, session, None);

            let landed = landed_since(repo, &before);
            assert!(
                !landed.is_empty(),
                "[{}] the door must land at least one commit",
                door.verb,
            );
            for sha in &landed {
                let what = format!("[{} / {session:?}]", door.verb);
                match session {
                    Session::Agent => assert_carries_once(repo, sha, &what),
                    Session::Human => assert_carries_none(repo, sha, &what),
                }
            }
        }
    }
}

/// `jigc setup`'s install commit — the axis's one exclusion — follows the same rule.
#[test]
fn the_install_commit_carries_the_trailer_only_under_the_agent() {
    for session in [Session::Agent, Session::Human] {
        let repo = TempDir::new("co-author-setup");
        let home = TempDir::new("co-author-setup-home");
        git(repo.path(), &["init", "-q", "-b", "main", "."]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("README.md"), "hello\n").expect("write README.md");
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-q", "-m", "initial"]);
        let before = head(repo.path());

        run_ok(repo.path(), home.path(), &["setup"], session, None);

        let landed = landed_since(repo.path(), &before);
        assert_eq!(landed.len(), 1, "[{session:?}] one install commit");
        match session {
            Session::Agent => assert_carries_once(repo.path(), &landed[0], "install commit"),
            Session::Human => assert_carries_none(repo.path(), &landed[0], "install commit"),
        }
    }
}

/// A commit doc whose own `#trailers` already name the agent — the composed step tells an
/// agent to record a co-author there — lands the trailer once, not twice. The identity is the
/// email: `Claude Opus <noreply@anthropic.com>` is the same co-author as `Claude <…>`.
#[test]
fn a_commit_doc_that_already_names_the_agent_is_not_doubled() {
    for value in [CLAUDE, "Claude Opus <noreply@anthropic.com>"] {
        let (repo, home) = base_repo("co-author-doc-trailer", None);
        let task = seed_task(repo.path(), home.path(), "record the trailer");
        let item = add_trailer(repo.path(), home.path(), &task, "Co-Authored-By");
        jigc_ok(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#trailers/{item}/value"),
                "--value",
                value,
            ],
            "`jigc doc set-field` on the trailer value",
        );
        fs::write(repo.path().join("code.txt"), "work\n").expect("write code.txt");
        git(repo.path(), &["add", "code.txt"]);

        run_ok(
            repo.path(),
            home.path(),
            &["task", "finalize", &task],
            Session::Agent,
            None,
        );

        let found = co_authors(repo.path(), "HEAD");
        assert_eq!(
            found,
            vec![value.to_owned()],
            "the doc's own trailer stands alone — jigc adds no second one for the same \
             co-author; message:\n{}",
            message(repo.path(), "HEAD"),
        );
    }
}

/// Add one `#trailers` item titled `key` to `task`'s commit doc, returning the item id the
/// binary printed.
fn add_trailer(repo: &Path, home: &Path, task: &str, key: &str) -> String {
    let out = run(
        repo,
        home,
        &[
            "doc",
            "add-item",
            &format!("commit:{task}#trailers"),
            "--title",
            key,
            "--format",
            "json",
        ],
        Session::Human,
        None,
    );
    assert!(
        out.status.success(),
        "`jigc doc add-item` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let ack: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("the add-item ack is JSON");
    ack["target"]["item"]
        .as_str()
        .unwrap_or_else(|| panic!("the add-item ack names the minted item; got {ack}"))
        .to_owned()
}

/// The amend arm re-authors the message from scratch, so the trailer is re-derived — and
/// **preserved** when a human repairs an agent's commit: the tree the agent co-authored did
/// not move, so its co-author claim is still true. An agent amending its own commit lands it
/// once, never twice.
#[test]
fn amend_preserves_the_trailer_without_duplicating_it() {
    for amender in [Session::Human, Session::Agent] {
        let (repo, home) = base_repo("co-author-amend", None);
        let task = seed_task(repo.path(), home.path(), "land the work");
        fs::write(repo.path().join("code.txt"), "work\n").expect("write code.txt");
        git(repo.path(), &["add", "code.txt"]);
        run_ok(
            repo.path(),
            home.path(),
            &["task", "finalize", &task],
            Session::Agent,
            None,
        );
        assert_carries_once(repo.path(), "HEAD", "the agent's original commit");
        let tree = git(repo.path(), &["rev-parse", "HEAD^{tree}"]);

        run_ok(
            repo.path(),
            home.path(),
            &["task", "amend", "repair the message"],
            amender,
            None,
        );
        let amend = "repair-the-message";
        fill_commit_doc(repo.path(), home.path(), amend);
        run_ok(
            repo.path(),
            home.path(),
            &["task", "finalize", amend],
            amender,
            None,
        );

        assert_eq!(
            git(repo.path(), &["rev-parse", "HEAD^{tree}"]),
            tree,
            "[{amender:?}] the premise: an amend moves no tree",
        );
        assert_carries_once(
            repo.path(),
            "HEAD",
            &format!("the amended commit ({amender:?} amender)"),
        );
    }
}

/// A human amending a commit **no agent** co-authored lands no trailer — preservation reads
/// what `HEAD` carries, it never invents a claim.
#[test]
fn a_human_amend_of_a_human_commit_lands_no_trailer() {
    let (repo, home) = base_repo("co-author-amend-human", None);
    let task = seed_task(repo.path(), home.path(), "land the work");
    fs::write(repo.path().join("code.txt"), "work\n").expect("write code.txt");
    git(repo.path(), &["add", "code.txt"]);
    run_ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        Session::Human,
        None,
    );
    run_ok(
        repo.path(),
        home.path(),
        &["task", "amend", "repair the message"],
        Session::Human,
        None,
    );
    fill_commit_doc(repo.path(), home.path(), "repair-the-message");
    run_ok(
        repo.path(),
        home.path(),
        &["task", "finalize", "repair-the-message"],
        Session::Human,
        None,
    );
    assert_carries_none(repo.path(), "HEAD", "a human's amend of a human's commit");
}

/// The fan-out join with **code-carrying** sub-tasks under `squash: false`: the per-sub-task
/// commit and the boundary aggregate each carry the trailer once; under `squash: true` the one
/// combined commit does.
#[test]
fn milestone_finalize_with_code_carries_the_trailer_on_every_commit_it_lands() {
    for squash in ["false", "true"] {
        let (repo, home) = base_repo(&format!("co-author-fan-out-{squash}"), Some(squash));
        let repo = repo.path();
        let home = home.path();
        jigc_ok(
            repo,
            home,
            &["milestone", "create", "Cache rework"],
            "`jigc milestone create`",
        );
        jigc_ok(
            repo,
            home,
            &["milestone", "add-task", "cache-rework", "Area low"],
            "`jigc milestone add-task`",
        );
        stage_subtask_doc(
            repo,
            "area-low",
            "adr:low-policy",
            &adr_body("Low policy", None),
        );
        stage_subtask_doc(
            repo,
            "area-low",
            "commit:area-low",
            "---\ntype: feat\n---\n\n# area-low\n\n## Summary\n\nrework the low cache path\n\n\
             ## Body\n\n\n\n## Trailers\n",
        );
        jigc_ok(
            repo,
            home,
            &["milestone", "provision", "cache-rework"],
            "`jigc milestone provision`",
        );
        let worktree = repo.join(".jigc").join("worktrees").join("area-low");
        fs::create_dir_all(worktree.join("src")).expect("mk the worktree's src/");
        fs::write(worktree.join("src").join("low.rs"), "pub fn low() {}\n")
            .expect("write the worktree's code");
        git(&worktree, &["add", "src/low.rs"]);
        let before = head(repo);

        run_ok(
            repo,
            home,
            &["milestone", "finalize", "cache-rework"],
            Session::Agent,
            None,
        );

        let landed = landed_since(repo, &before);
        let expected = if squash == "false" { 2 } else { 1 };
        assert_eq!(
            landed.len(),
            expected,
            "[squash: {squash}] the premise: the boundary's commit count",
        );
        for sha in &landed {
            assert_carries_once(
                repo,
                sha,
                &format!("[milestone finalize, squash: {squash}]"),
            );
        }
    }
}

/// The fixture report workflow of `tests/doc_only_finalize.rs`: code-less, granting `idea`,
/// composing the shipped `step:finalize-doc-only`.
const REPORT_WORKFLOW: &str = "\
---
when: file one finding as a doc while other work is open in the checkout
description: A fixture code-less report workflow — one idea doc, committed path-scoped.
usage: the fixture cell for the doc-only finalize, reached by name.
creates-task: true
selectable: false
suppressed:
  reason: fixture-only — the doc-only finalize cell, reached by name
  expires: never
allows-create:
  - { type: idea, as: idea }
---
{{ include: step:author-commit }}
{{ include: step:finalize-doc-only }}
";

/// The M55 **path-scoped doc-only commit** carries the trailer — beside a staged code change
/// it leaves out, so the cell is the doc-only arm and not the ordinary one.
#[test]
fn the_doc_only_path_scoped_commit_carries_the_trailer() {
    let (repo, home) = base_repo("co-author-doc-only", None);
    let repo = repo.path();
    let home = home.path();
    let workflows = repo.join(".jigc").join("config").join("workflows");
    fs::create_dir_all(&workflows).expect("mk the project workflows dir");
    fs::write(workflows.join("file-report.yaml"), REPORT_WORKFLOW).expect("write the workflow");
    git(repo, &["add", "--", ".jigc/config/workflows"]);
    git(repo, &["commit", "-q", "-m", "chore: the fixture workflow"]);

    jigc_ok(
        repo,
        home,
        &["start", "--workflow", "file-report", "file a finding"],
        "`jigc start`",
    );
    let task = "file-a-finding";
    let out = run(
        repo,
        home,
        &[
            "doc",
            "create",
            "idea",
            "--title",
            "Reports Land",
            "--task",
            task,
        ],
        Session::Human,
        None,
    );
    assert!(out.status.success(), "`jigc doc create idea` exits 0");
    let address = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    jigc_ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("{address}#trigger"),
            "--value",
            "a report comes back",
            "--task",
            task,
        ],
        "`jigc doc set-field` on the idea",
    );
    let set_slot = |addr: &str, prose: &str| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        let mut child = command
            .args(["doc", "set-slot", addr, "--from-file", "-", "--task", task])
            .current_dir(repo)
            .env("HOME", home)
            .env_remove("JIGC_PACK_DIR")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn jigc");
        crate::support::child_stdin::feed(&mut child, prose.as_bytes());
        let out = child.wait_with_output().expect("wait for jigc");
        assert!(
            out.status.success(),
            "`jigc doc set-slot {addr}` exits 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    };
    set_slot(
        &format!("{address}#description"),
        "One finding, filed while other work is open.\n",
    );
    fill_commit_doc(repo, home, task);
    // A staged code change the doc-only arm leaves out.
    fs::write(repo.join("other.txt"), "someone else's work\n").expect("write other.txt");
    git(repo, &["add", "other.txt"]);

    run_ok(
        repo,
        home,
        &["task", "finalize", task],
        Session::Agent,
        None,
    );

    let files = git(repo, &["show", "--name-only", "--pretty=format:", "HEAD"]);
    assert!(
        !files.contains("other.txt"),
        "the premise: the path-scoped commit left the staged code out; HEAD holds:\n{files}",
    );
    assert_carries_once(repo, "HEAD", "the doc-only path-scoped commit");
}

/// A profile that declares no `co-author` adds nothing, whoever runs it — the adopter on an
/// assistant whose profile has no trailer to give is unaffected.
#[test]
fn a_profile_without_the_key_adds_nothing() {
    let profile = include_str!("../adapters/claude-code.yaml");
    let start = profile
        .find("\n# The coding agent's co-author trailer")
        .expect("the shipped profile carries the co-author block's comment");
    let end = start
        + profile[start..]
            .find("\n  when-env: CLAUDECODE\n")
            .expect("the co-author block ends at its when-env line")
        + "\n  when-env: CLAUDECODE\n".len();
    let stripped = format!("{}\n{}", &profile[..start], &profile[end..]);
    assert!(
        !stripped.contains("co-author"),
        "the premise: the stripped profile declares no co-author:\n{stripped}",
    );
    let adapters = TempDir::new("co-author-no-key-adapters");
    fs::write(adapters.path().join("claude-code.yaml"), stripped).expect("write the profile");

    let (repo, home) = base_repo("co-author-no-key", None);
    let task = seed_task(repo.path(), home.path(), "land the work");
    fs::write(repo.path().join("code.txt"), "work\n").expect("write code.txt");
    git(repo.path(), &["add", "code.txt"]);
    run_ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        Session::Agent,
        Some(adapters.path()),
    );
    assert_carries_none(repo.path(), "HEAD", "a profile without the key");
}
