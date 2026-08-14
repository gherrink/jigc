//! M45 Increment 9, T1 — the exit-code taxonomy suite
//! (`design/command-output-contract.md` → The exit-code taxonomy;
//! `implementation/pinning.md` → §2, one suite per pinned statement).
//!
//! The `0/1/2/3/4` vocabulary is the outermost layer of the machine contract — a
//! driver reads the exit code before a byte of output. This suite provokes each
//! outcome class through the real binary, across the verb families, and asserts the
//! severity→exit mapping **against the one minted table constant**
//! ([`cli::task::EXIT_CODES`], read via [`cli::task::exit_code_for`]) — never a hand
//! literal, so a code that moved in the table without moving here is unrepresentable:
//!
//!   - **success (0)** — a clean store-scope `jigc validate`;
//!   - **usage (2)** — an unknown subcommand, rejected by clap before jigc reads
//!     `--format` (the declared clap carve-out — its own arm, plain text, exit 2);
//!   - **blocking finding at a task-scope gate (3)** — `jigc task validate` over a task
//!     whose commit doc is unfilled;
//!   - **store-scope validate exit-flip (1)** — `jigc validate` over an unmigrated (v0)
//!     managed ADR corpus, the third store-scope exit-flipping exception;
//!   - **migration review hold (4)** — a migration `jigc task finalize` without
//!     `--approve`.
//!
//! Drives the built `jigc` binary against throwaway `git init` temp repos over the
//! shipped dev pack (`JIGC_PACK_DIR`), a self-cleaning `TempDir` per repo.

use cli::task::{ExitClass, exit_code_for};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-exitcodes-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` (byte-identical to
/// the binary-embedded pack, so setting it is harmless and pins the pack this suite reads).
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with identity + one commit + the `.jigc/config/` layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = dev pack`,
/// optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", dev_pack())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// Assert `out` exits 0, surfacing both streams on failure; return trimmed stdout.
fn ok_stdout(out: Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The heart of the suite: assert `out`'s exit code is the taxonomy table's code for
/// `class` — read from [`EXIT_CODES`] via [`exit_code_for`], never a hand literal.
fn assert_exit_class(out: &Output, class: ExitClass, what: &str) {
    let expected = exit_code_for(class);
    assert_eq!(
        out.status.code(),
        Some(i32::from(expected)),
        "{what} must exit {expected} ({class:?}) per the EXIT_CODES table; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 0 — success: a clean store-scope `jigc validate` over a freshly-set-up repo.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn clean_store_validate_exits_success() {
    let repo = TempDir::new("success");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(repo.path(), home.path(), &["setup"], None),
        "jigc setup",
    );

    let out = jigc(repo.path(), home.path(), &["validate"], None);
    assert_exit_class(
        &out,
        ExitClass::Success,
        "a clean store-scope `jigc validate`",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 2 — usage: an unknown subcommand, rejected by clap (the declared carve-out).
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn unknown_subcommand_exits_usage() {
    let repo = TempDir::new("usage");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["definitely-not-a-real-verb"],
        None,
    );
    assert_exit_class(&out, ExitClass::Usage, "an unknown subcommand (clap usage)");
    assert!(
        out.stdout.is_empty(),
        "a usage error prints to stderr, never the JSON funnel on stdout; got:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 3 — blocking findings at a task-scope gate: `jigc task validate` over an unfilled
//     commit doc (its author-required fields empty → a blocking schema-conformance
//     finding). The task gate, not the store sweep.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn blocked_task_validate_exits_task_gate_blocked() {
    let repo = TempDir::new("taskgate");
    let home = TempDir::new("home");
    init_repo(repo.path());

    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "add a widget"],
            None,
        ),
        "jigc start",
    );

    // The freshly-provisioned commit doc has empty author-required fields — a blocking
    // finding at the task gate. No authoring, so the block is the only outcome.
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "add-a-widget"],
        None,
    );
    assert_exit_class(
        &out,
        ExitClass::TaskGateBlocked,
        "a blocking `jigc task validate`",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 1 — store-scope validate exit-flip: `jigc validate` over an unmigrated (v0) managed
//     ADR corpus flips the store sweep's exit to 1 (schema-conformance.schema-version-
//     current — the third store-scope exit-flipping exception). Maps to ExitClass::Error.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn store_validate_exit_flip_exits_error() {
    let repo = TempDir::new("flip");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(repo.path(), home.path(), &["setup"], None),
        "jigc setup",
    );

    // Commit an unstamped (v0) ADR at its canonical managed home — a managed corpus the
    // store sweep detects as unmigrated.
    let dir = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("single-node-cache.md"), UNSTAMPED_ADR).expect("write v0 adr");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed v0 adr"]);

    let out = jigc(repo.path(), home.path(), &["validate"], None);
    assert_exit_class(
        &out,
        ExitClass::Error,
        "a store-scope `jigc validate` over an unmigrated corpus (the exit-flip)",
    );
}

/// A conformant `adr` body under the pack schema, with **no** `schema-version` stamp —
/// the unmigrated v0 state the store sweep flags + exit-flips.
const UNSTAMPED_ADR: &str = "\
---
status: accepted
date: 2026-06-25
---

# Single-node cache

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

// ─────────────────────────────────────────────────────────────────────────────
// 4 — migration review hold: a migration `jigc task finalize` without `--approve`
//     renders the fidelity diff and stops (exit 4). Reaches the review gate only over a
//     conformant staged migration (an unconformant one would exit 3 first).
// ─────────────────────────────────────────────────────────────────────────────

/// The off-router migration task id `jigc migrate HISTORY.md --as changelog` mints — the
/// empty intent slugs the `migrate-<doctype>-<slug(path)>` fallback (mirrors
/// `migrate_review_gate.rs`).
const MIGRATION_TASK: &str = "migrate-changelog-history-3268e06b69e1";

const FOREIGN_CHANGELOG: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

#[test]
fn migration_finalize_without_approve_exits_review_hold() {
    let repo = TempDir::new("review");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(repo.path(), home.path(), &["setup"], None),
        "jigc setup",
    );

    fs::write(repo.path().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write foreign");
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["migrate", "HISTORY.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "doc create changelog",
    );

    // Author the single release oldest-to-newest with a historical date + one change
    // group, so the staged changelog is conformant and finalize reaches the review gate.
    let release = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "0.1.0",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "add-item release",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{release}/date"),
                "--value",
                "2021-03-09",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "set-field date",
    );
    let group = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
                "--task",
                MIGRATION_TASK,
            ],
            None,
        ),
        "add-item change-group",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
                "--task",
                MIGRATION_TASK,
            ],
            Some(b"First public release.\n"),
        ),
        "set-slot notes",
    );
    make_commit_conformant(repo.path(), home.path(), MIGRATION_TASK);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", MIGRATION_TASK],
        None,
    );
    assert_exit_class(
        &out,
        ExitClass::MigrationReview,
        "a migration finalize without --approve",
    );
    // The hold committed nothing — HEAD is unmoved, the foreign original byte-intact.
    assert!(
        repo.path().join("HISTORY.md").exists(),
        "a review hold adopts nothing and retires nothing",
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "a review hold writes no canonical doc",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 1 — the one-way door: **a rejected write exits 1, never 3** (the exit-code
//     table's rejected-write clause — `design/command-output-contract.md` → The
//     exit-code taxonomy, code 3: "never a rejected write"; Decision 6 of the M45
//     Settle). This is a PIN, not a behavior fix: every prior assert over a
//     rejected write checked `!success` (or `≠0, ≠2`), which a silent flip from
//     1 to 3 would satisfy. The arms below assert the **literal 1** — not
//     `exit_code_for`, so a flip of `EXIT_ERROR` itself is caught too — and the
//     axis is the whole `doc` write-verb family, derived from the clap tree so a
//     new write verb reddens the sweep instead of dodging it.
// ─────────────────────────────────────────────────────────────────────────────

/// The `doc` write-verb family — the partition itself lives production-side
/// (`cli::doc::doc_write_verbs`, the clap-tree-derived leaf set minus the declared
/// read verbs `show`/`schema`/`list`), because the M48 read-back fence reads the
/// same split at pack-load. This sweep consumes it rather than keeping a second
/// copy of the read set beside it: a new `doc` verb must join either the reject
/// cases here or the read set there.
use cli::doc::{DOC_READ_VERBS, doc_write_verbs};

/// One rejected-write case: the verb it exercises, the argv of a write the binary
/// must refuse, optional stdin, and the finding code the refusal carries (asserted
/// so an arm cannot rot into an accidental operational error and still pass).
struct RejectedWrite {
    verb: &'static str,
    args: &'static [&'static str],
    stdin: Option<&'static [u8]>,
    finding: &'static str,
}

/// The adr `doc author` payload whose slot prose carries a schema-reserved-depth
/// heading — parses as a payload, then refuses on the write path.
const AUTHOR_DEPTH_PAYLOAD: &[u8] = b"\
title: Rejected adr
sections:
  - id: context
    set:
      context: |
        <<## a heading at reserved depth>>
";

/// One genuinely refused write per `doc` write verb, over one minted `single-task`
/// task (`add-a-widget`): the gated create, the four `write.*` reject classes, and
/// the batch verb refusing through the same write path.
const REJECTED_WRITES: &[RejectedWrite] = &[
    RejectedWrite {
        verb: "create",
        args: &[
            "doc",
            "create",
            "spec",
            "--title",
            "A spec",
            "--task",
            "add-a-widget",
        ],
        stdin: None,
        finding: "create.gate-blocked",
    },
    RejectedWrite {
        verb: "add-item",
        args: &[
            "doc",
            "add-item",
            "commit:add-a-widget#summary",
            "--title",
            "Nope",
            "--task",
            "add-a-widget",
        ],
        stdin: None,
        finding: "write.wrong-shape",
    },
    RejectedWrite {
        verb: "remove-item",
        args: &[
            "doc",
            "remove-item",
            "commit:add-a-widget#summary/nope",
            "--task",
            "add-a-widget",
        ],
        stdin: None,
        finding: "write.not-present",
    },
    RejectedWrite {
        verb: "retitle-item",
        args: &[
            "doc",
            "retitle-item",
            // The **repeatable** `trailers` section, not the slot-only `summary`: an item
            // address under a non-repeatable section is a genuine *shape* question (the
            // `add-item` arm above pins that), while a not-yet-minted id under a real
            // repeatable is the item-id miss this arm is here to refuse.
            "commit:add-a-widget#trailers/nope",
            "--title",
            "New",
            "--task",
            "add-a-widget",
        ],
        stdin: None,
        // An item-id miss is not a shape question — the addressed item was never minted,
        // and the schema names shapes, never the corpus's live item ids (M47 — the
        // write-verb × miss-shape axis; `write_miss_shape_axis.rs`).
        finding: "write.not-present",
    },
    RejectedWrite {
        verb: "set-field",
        args: &[
            "doc",
            "set-field",
            "commit:add-a-widget#type",
            "--value",
            "not-a-type",
            "--task",
            "add-a-widget",
        ],
        stdin: None,
        finding: "write.malformed-value",
    },
    RejectedWrite {
        verb: "set-slot",
        args: &[
            "doc",
            "set-slot",
            "commit:add-a-widget#summary",
            "--from-file",
            "-",
            "--task",
            "add-a-widget",
        ],
        stdin: Some(b"## a heading at reserved depth\n"),
        finding: "write.slot-heading-depth",
    },
    RejectedWrite {
        verb: "author",
        args: &[
            "doc",
            "author",
            "adr",
            "--from-file",
            "-",
            "--task",
            "add-a-widget",
        ],
        stdin: Some(AUTHOR_DEPTH_PAYLOAD),
        finding: "write.slot-heading-depth",
    },
    RejectedWrite {
        verb: "rename",
        // The transient sink: a `commit` doc's slug IS the task id its finalize
        // renders the message for, so there is no author-owned title to move
        // (M48 — `design/write-commands.md` → `jigc doc rename`).
        args: &[
            "doc",
            "rename",
            "commit:add-a-widget",
            "--to",
            "Something Else",
            "--task",
            "add-a-widget",
        ],
        stdin: None,
        finding: "write.identity-change",
    },
];

#[test]
fn rejected_writes_exit_error_exactly_never_task_gate() {
    // The pin's anchor: the one-way door is the number 1 itself, so the table
    // constant is asserted as the **literal** — a flip of `EXIT_ERROR` (and with
    // it every disciplined `Outcome { code: EXIT_ERROR }` site) to 3 fails here.
    assert_eq!(
        cli::task::EXIT_ERROR,
        1,
        "the one-way door: a rejected write exits 1, never 3 — `EXIT_ERROR` is \
         pinned to the literal (design/command-output-contract.md → The exit-code \
         taxonomy)",
    );
    assert_eq!(
        exit_code_for(ExitClass::Error),
        1,
        "the table row for ExitClass::Error is the same literal (statement == constant)",
    );

    // The axis fence: the case list covers exactly the clap-derived write family.
    let mut family = doc_write_verbs();
    family.sort_unstable();
    let mut swept: Vec<String> = REJECTED_WRITES.iter().map(|c| c.verb.to_string()).collect();
    swept.sort_unstable();
    assert_eq!(
        swept, family,
        "the rejected-write sweep covers exactly the `doc` write-verb family \
         (clap tree minus the declared read verbs {DOC_READ_VERBS:?}) — a new \
         write verb must join the sweep or the read set",
    );

    // One repo, one minted task; every arm refuses against the same working area.
    let repo = TempDir::new("reject");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "add a widget"],
            None,
        ),
        "jigc start",
    );

    for case in REJECTED_WRITES {
        let out = jigc(repo.path(), home.path(), case.args, case.stdin);
        let stderr = String::from_utf8_lossy(&out.stderr);
        // The right reason: each arm is a genuine refused write (the named
        // finding), never an accidental operational error that also exits 1.
        assert!(
            stderr.contains(case.finding),
            "`doc {}` must refuse as `{}`; stderr:\n{stderr}",
            case.verb,
            case.finding,
        );
        // The one-way door, asserted as the literal — `assert_refused`-style
        // `!success` (or `≠0, ≠2`) would accept a silent flip to 3; this cannot.
        assert_eq!(
            out.status.code(),
            Some(1),
            "a rejected `doc {}` write must exit 1 exactly — never 3, the \
             task-gate code (the exit-code table's one-way door); stderr:\n{stderr}",
            case.verb,
        );
    }
}

/// Fill every author-required field/slot of `task`'s provisioned commit doc so a
/// finalize over it validates clean (the review gate sits behind the validation gate).
fn make_commit_conformant(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        ok_stdout(
            jigc(
                repo,
                home,
                &["doc", "set-field", addr, "--value", value, "--task", task],
                None,
            ),
            "set-field commit",
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        ok_stdout(
            jigc(
                repo,
                home,
                &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
                Some(prose),
            ),
            "set-slot commit",
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "changelog");
    set_slot(
        &format!("commit:{task}#summary"),
        b"adopt the migrated changelog\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Migrate the foreign HISTORY.md into managed shape.\n",
    );
}
