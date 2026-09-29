//! M49 Increment 9 / T6 — **the forcing function: the planning task cannot finalize
//! on an unfilled gate** (`design/methodology-docs.md` → The pack form —
//! presence-conformance, never content-scoring).
//!
//! T5 shipped `planning-record`'s *shape* and said so plainly: a doctype no workflow
//! can create writes nothing. This suite is the shipped door — `jigc start --workflow
//! planning` → the composed create → author the gates → `jigc task finalize` — driven
//! end to end over the real binary (`CARGO_BIN_EXE_jigc`) against the on-disk
//! methodology pack, because the block is the doctype's entire purpose and a block
//! proven at the engine is not the block a planning session meets.
//!
//! Three claims:
//!
//!   1. **The door opens, and the composed step is what opens it.** The `planning`
//!      workflow's `allows-create` admits `planning-record`, and the create is driven
//!      from the **emitted bytes** of the composed step — the `Run:` line extracted
//!      from `jigc start --workflow planning`'s own stdout, run verbatim with only its
//!      `<TITLE>` agent placeholder filled. A test that rebuilt the command in test
//!      code could pass while the line an agent copies is broken.
//!   2. **The forcing function fires over the WHOLE gate axis, not one reported
//!      gate.** The gate set is read from the shipped schema through the binary
//!      (`jigc doc schema planning-record --format json`) — this file hand-lists no
//!      gate id — and for **each** gate in turn a record carrying every *other* gate
//!      BLOCKS `jigc task finalize` with `schema-conformance.required-slot-present`
//!      naming that gate, with no commit created. A fifteenth gate is swept the day it
//!      ships. One held-out gate would have proven the mechanism; it would not have
//!      proven that *every* gate is a fence, which is the claim the doctype makes.
//!   3. **Fill it and the identical finalize lands the record.** The same
//!      `jigc task finalize <id>` that blocked exits 0 once the held-out gate is
//!      filled, and `planning-records/<slug>.md` is committed carrying all fourteen
//!      gates' prose — so the block is a gate, never a dead end.
//!
//! What is deliberately NOT asserted: anything about the *content* of a gate slot.
//! jigc checks a slot is filled and never reads or grades the judgment inside it (the
//! A-3 bound, `design/methodology-docs.md` → the pack form). Every gate here is
//! answered with the same trivial prose, and that is a complete answer by contract.

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
            "jigc-plangate-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Run `git` in `repo`, asserting success, returning stdout.
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

/// A real git repo with one commit (composition mints against HEAD).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` over the on-disk methodology pack, optionally piping `stdin`.
fn jigc_in(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
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

/// Run `jigc <args>` with no stdin.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    jigc_in(repo, home, args, None)
}

/// Assert an invocation succeeded, surfacing both streams on failure.
fn assert_ok(out: &Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A fresh repo + home with the methodology pack installed.
fn setup() -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());
    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"]),
        "`JIGC_PACK_DIR=<methodology> jigc setup`",
    );
    (repo, home)
}

/// The `task minted: <id>` line the compose front-end emits.
fn minted_task(composed: &str) -> String {
    composed
        .lines()
        .find_map(|line| line.trim().strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("the composed output names the minted task; got:\n{composed}"))
        .trim()
        .to_string()
}

/// **The gate axis, read from the shipped schema through the binary** — every
/// `kind: slot` section of the pinned `doc schema … --format json` projection, in
/// schema order. Nothing in this file hand-lists a gate.
fn gate_axis(repo: &Path, home: &Path) -> Vec<String> {
    let raw = assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "schema", "planning-record", "--format", "json"],
        ),
        "`jigc doc schema planning-record --format json`",
    );
    let projection: serde_json::Value =
        serde_json::from_str(&raw).expect("the pinned schema projection parses as json");
    let gates: Vec<String> = projection["sections"]
        .as_array()
        .expect("`sections` is an array")
        .iter()
        .filter(|section| section["kind"] == "slot")
        .map(|section| {
            section["id"]
                .as_str()
                .expect("a section advertises its id")
                .to_string()
        })
        .collect();
    assert!(
        gates.len() > 1,
        "the gate axis must hold more than one gate for the sweep to mean anything; got {gates:?}",
    );
    gates
}

/// The composed create line for `planning-record`, taken from the **emitted bytes** of
/// the composed `planning` workflow and made runnable: the `Run: ` / backtick
/// decoration stripped and the single `<TITLE>` agent placeholder filled with
/// `milestone`. Everything else — verb, doctype, `--task` binding — is the pack's own.
fn emitted_create(composed: &str, milestone: &str) -> Vec<String> {
    let line = composed
        .lines()
        .map(str::trim)
        .find(|line| line.contains("jigc doc create planning-record"))
        .unwrap_or_else(|| {
            panic!(
                "the composed planning workflow emits a `planning-record` create; got:\n{composed}"
            )
        });
    let command = line
        .strip_prefix("Run: `")
        .and_then(|rest| rest.strip_suffix('`'))
        .unwrap_or(line);
    let argv: Vec<String> = command
        .split_whitespace()
        .map(|token| {
            if token == "<TITLE>" {
                milestone.to_string()
            } else {
                token.to_string()
            }
        })
        .collect();
    assert_eq!(
        argv.first().map(String::as_str),
        Some("jigc"),
        "the emitted create is a `jigc` invocation; got `{command}`",
    );
    assert!(
        !argv.iter().any(|token| token.starts_with('<')),
        "every placeholder in the emitted create is filled before it runs; got {argv:?}",
    );
    argv[1..].to_vec()
}

/// Fill the transient commit doc so the ONLY thing finalize can block on is a gate.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    for (leaf, value) in [("type", "docs"), ("scope", "planning")] {
        assert_ok(
            &jigc(
                repo,
                home,
                &[
                    "doc",
                    "set-field",
                    &format!("commit:{task}#{leaf}"),
                    "--value",
                    value,
                    "--task",
                    task,
                ],
            ),
            &format!("`doc set-field commit#{leaf}`"),
        );
    }
    for (leaf, prose) in [
        ("summary", "record the plan-time gates"),
        ("body", "A planning pass over the gate record.\n"),
    ] {
        assert_ok(
            &jigc_in(
                repo,
                home,
                &[
                    "doc",
                    "set-slot",
                    &format!("commit:{task}#{leaf}"),
                    "--from-file",
                    "-",
                    "--task",
                    task,
                ],
                Some(prose.as_bytes()),
            ),
            &format!("`doc set-slot commit#{leaf}`"),
        );
    }
}

/// The prose a gate is answered with. Deliberately uniform and content-free: jigc
/// enforces presence and never grades the judgment (the A-3 bound), so this is a
/// complete answer by contract.
fn gate_prose(gate: &str) -> String {
    format!("Evidence discharging the {gate} gate.\n")
}

/// Fill one gate slot on the staged record.
fn fill_gate(repo: &Path, home: &Path, task: &str, slug: &str, gate: &str) {
    assert_ok(
        &jigc_in(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("planning-record:{slug}#{gate}"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            Some(gate_prose(gate).as_bytes()),
        ),
        &format!("`doc set-slot planning-record:{slug}#{gate}`"),
    );
}

/// Open a planning task for `milestone`, create its gate record **through the composed
/// step's emitted create line**, author every gate except `held_out`, and fill the
/// commit doc. Returns `(task, slug)` — ready for `jigc task finalize`.
fn stage_record(
    repo: &Path,
    home: &Path,
    milestone: &str,
    gates: &[String],
    held_out: &str,
) -> (String, String) {
    let composed = assert_ok(
        &jigc(repo, home, &["start", "--workflow", "planning", milestone]),
        &format!("`jigc start --workflow planning {milestone}`"),
    );
    let task = minted_task(&composed);

    let argv = emitted_create(&composed, milestone);
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    let ack = assert_ok(
        &jigc(repo, home, &argv),
        "the composed `planning-record` create, run verbatim",
    );
    let slug = ack
        .trim()
        .strip_prefix("planning-record:")
        .unwrap_or_else(|| {
            panic!("`doc create planning-record` acks `planning-record:<slug>`; got `{ack}`")
        })
        .trim()
        .to_string();

    for gate in gates.iter().filter(|gate| gate.as_str() != held_out) {
        fill_gate(repo, home, &task, &slug, gate);
    }
    fill_commit(repo, home, &task);
    (task, slug)
}

// ---------------------------------------------------------------------------
// Claim 1 — the door opens, and the composed step is what opens it.
// ---------------------------------------------------------------------------

#[test]
fn the_planning_workflow_creates_the_gate_record_through_its_own_composed_line() {
    let (repo, home) = setup();
    let composed = assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "planning", "M-Door"],
        ),
        "`jigc start --workflow planning M-Door`",
    );
    let task = minted_task(&composed);

    // The gate is named on the composed surface, not learned by tripping the refusal.
    let gate_line = composed
        .lines()
        .find(|line| line.trim_start().starts_with("create-gates:"))
        .unwrap_or_else(|| panic!("the composed task names its create-gates; got:\n{composed}"));
    assert!(
        gate_line.contains("planning-record"),
        "the planning task's create-gates admit `planning-record`; got `{gate_line}`",
    );

    // And the create the step emits runs verbatim.
    let argv = emitted_create(&composed, "M-Door");
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    let ack = assert_ok(
        &jigc(repo.path(), home.path(), &argv),
        "the composed `planning-record` create, run verbatim",
    );
    assert!(
        ack.trim().starts_with("planning-record:"),
        "the create acks the minted identity; got `{ack}`",
    );

    // The staged read-back the step names serves what was just created.
    let slug = ack.trim().strip_prefix("planning-record:").unwrap().trim();
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "show",
                &format!("planning-record:{slug}"),
                "--task",
                &task,
            ],
        ),
        "`jigc doc show planning-record:<slug> --task <id>` — the step's named read-back",
    );
}

// ---------------------------------------------------------------------------
// Claim 2 — the forcing function fires over the whole gate axis.
// ---------------------------------------------------------------------------

#[test]
fn every_gate_is_a_fence_finalize_blocks_on_each_one_held_out() {
    let (repo, home) = setup();
    let gates = gate_axis(repo.path(), home.path());

    for (nth, held_out) in gates.iter().enumerate() {
        let milestone = format!("M-Hold-{nth}");
        let (task, slug) = stage_record(repo.path(), home.path(), &milestone, &gates, held_out);

        let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
            .trim()
            .parse()
            .expect("a commit count");
        let out = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
        let rendered = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !out.status.success(),
            "an unfilled `{held_out}` gate must block finalize non-zero; got:\n{rendered}",
        );
        assert!(
            rendered.contains("schema-conformance.required-slot-present"),
            "the `{held_out}` block is the required-slot-present conformance finding; \
             got:\n{rendered}",
        );
        assert!(
            rendered.contains(&format!("planning-record:{slug}#{held_out}")),
            "the block names the unfilled `{held_out}` gate at its own address; got:\n{rendered}",
        );
        let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
            .trim()
            .parse()
            .expect("a commit count");
        assert_eq!(
            before, after,
            "a blocked planning finalize creates no commit (gate `{held_out}`)",
        );

        // Leave no active task behind for the next iteration.
        assert_ok(
            &jigc(
                repo.path(),
                home.path(),
                &["task", "discard", &task, "--force"],
            ),
            &format!("`jigc task discard {task} --force`"),
        );
    }
}

// ---------------------------------------------------------------------------
// Claim 3 — fill it and the identical finalize lands the record.
// ---------------------------------------------------------------------------

#[test]
fn filling_the_held_out_gate_lets_the_same_finalize_land_the_record() {
    let (repo, home) = setup();
    let gates = gate_axis(repo.path(), home.path());
    let held_out = gates.last().expect("a gate axis").clone();

    let (task, slug) = stage_record(repo.path(), home.path(), "M-Land", &gates, &held_out);
    let blocked = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
    assert!(
        !blocked.status.success(),
        "the record blocks before the held-out gate is filled; got:\n{}{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );

    fill_gate(repo.path(), home.path(), &task, &slug, &held_out);

    // The IDENTICAL invocation — nothing about the finalize changed, only the gate.
    let landed = jigc(repo.path(), home.path(), &["task", "finalize", &task]);
    assert!(
        landed.status.success(),
        "the same finalize lands once every gate is filled; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&landed.stdout),
        String::from_utf8_lossy(&landed.stderr),
    );

    let committed = git(
        repo.path(),
        &["show", &format!("HEAD:planning-records/{slug}.md")],
    );
    let missing: Vec<&String> = gates
        .iter()
        .filter(|gate| !committed.contains(gate_prose(gate).trim()))
        .collect();
    assert!(
        missing.is_empty(),
        "the committed record carries every gate's authored prose; missing {missing:?} \
         in:\n{committed}",
    );
}
