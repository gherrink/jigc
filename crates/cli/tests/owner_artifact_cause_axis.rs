//! M47 Increment 4 / T2 — the **`owned_location_violation` cause axis**, swept over both
//! surfaces the #5 owner-artifact gate now speaks from: the `jigc task validate` *preview*
//! and the post-stage *committing* gate (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1;
//! → 2026-07-23 M45 Settle, Decision 6, **narrowed to its own stated grounds, not
//! overturned**).
//!
//! M45 Decision 6 moved the gate out of `validate_task` because it asserts the artifact is
//! durably **tracked** — a state phase-5 staging can satisfy, so a pre-stage `tracked` check
//! false-positives a finalize that lands at exit 0. That rationale is sound and is preserved
//! here. But it reaches **one of seven** causes: only the untracked one consults `tracked`
//! (`crates/engine/src/validate.rs` → `owned_location_violation`). The other six —
//! empty · absolute · `..` · not-under-home · names-no-file · symlink-escape — are
//! *staging-independent*: no stage can change their verdict, so previewing them cannot
//! false-positive. The trial's two live blocks were causes 4 and 5.
//!
//! So the axis is `7 causes × {validate, post-stage}`, and this suite iterates it rather
//! than pinning the two reported repros ([pinning.md](../../../implementation/pinning.md)
//! §1 — the fix is complete over its class's axis). Every assertion runs on the **emitted
//! bytes / exit code** of the real binary (`CARGO_BIN_EXE_jigc`) over a throwaway `git init`
//! repo: for the six previewing causes the `owner-artifact.present` finding **objects** are
//! compared byte-for-byte between `task validate --format json` and `task finalize
//! --format json`, so a divergence in code, key target, message, location or route fails
//! here — never a hand-rebuilt equivalent of either.
//!
//! **Cause 7 (untracked) stays post-stage** and is proven so *structurally*: the preview
//! runs the gate under a constant-true `tracked` predicate, so the untracked branch cannot
//! be reached from that door. Its row asserts the split live — `task validate` exits 0 with
//! **no** `owner-artifact.present` finding over a present-but-gitignored artifact, while
//! `task finalize` over that same state exits 3 with the block.
//!
//! **Cause 1 (empty) is pre-empted at the committing door, and the row says so.** An empty
//! `owned-location` value is *also* a `schema-conformance.field-value-conformant` violation,
//! and finalize's phase-2 abort-chain blocks on it before the stage phase runs — so the
//! post-stage gate never speaks for that state through the binary. The row therefore records
//! `schema-conformance.field-value-conformant` as the finalize door's emitter (still
//! blocking, still exit 3, still no commit) rather than being quietly dropped from the axis.
//!
//! The proof rides the same **FIXTURE** `completion-record` doctype `owner_artifact_gate.rs`
//! uses — an engine-native `owned-location` field on a test-pack doctype, never methodology
//! content — listed in the in-repo `.jigc/config/packs.yaml` so it UNIONs with the embedded
//! base pack (the flow-18 listed-pack mechanism).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// `EXIT_VALIDATION_BLOCKED` — the task-scope blocking-report exit
/// (`design/command-output-contract.md` → the exit-code table).
const EXIT_BLOCKED: i32 = 3;

/// The gate's finding code, on both surfaces.
const GATE_CODE: &str = "owner-artifact.present";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-owner-cause-axis-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning stdout.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`. `JIGC_PACK_DIR` is scrubbed so the
/// embedded base pack is the one in the union, whatever the developer's shell carries.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR");
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

/// The task id every row mints (slugified from the fixed intent).
const TASK: &str = "record-the-completion";

/// Start the `complete` task, author a completion-record whose `owner-artifact` names
/// `value`, fill its body slot, and fill the commit doc — so the owned-location value is the
/// only lever a row turns.
fn author_completion(repo: &Path, home: &Path, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "complete", "record the completion"],
        ),
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
                value,
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

    for (addr, value) in [
        (format!("commit:{TASK}#type"), "chore"),
        (format!("commit:{TASK}#scope"), "completion"),
    ] {
        assert_ok(
            &jigc_doc(repo, home, &["set-field", &addr, "--value", value], None),
            &format!("set-field {addr}"),
        );
    }
    for (addr, prose) in [
        (
            format!("commit:{TASK}#summary"),
            &b"record the completion\n"[..],
        ),
        (
            format!("commit:{TASK}#body"),
            &b"A milestone completion.\n"[..],
        ),
    ] {
        assert_ok(
            &jigc_doc(
                repo,
                home,
                &["set-slot", &addr, "--from-file", "-"],
                Some(prose),
            ),
            &format!("set-slot {addr}"),
        );
    }
}

/// Parse a findings envelope (`--format json`) into its `findings` array.
fn findings_of(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let envelope: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "{what} must emit a JSON findings envelope ({e}); stdout:\n{stdout}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{what}'s envelope carries a `findings` array; got {envelope}"))
        .clone()
}

/// The `owner-artifact.present` findings inside an envelope, in emitted order.
fn gate_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    findings_of(out, what)
        .into_iter()
        .filter(|f| f["code"] == GATE_CODE)
        .collect()
}

/// One row of the cause axis.
struct Cause {
    /// A short tag, used for the temp-dir name and failure messages.
    tag: &'static str,
    /// The `owned-location` value authored into the completion-record.
    value: &'static str,
    /// Repo furniture written *before* the task is minted (never through jigc — jigc has no
    /// verb that plants a symlink or a `.gitignore`).
    plant: fn(&Path),
    /// A distinguishing fragment of the violation reason the gate renders.
    reason: &'static str,
    /// Whether `jigc task validate` previews the gate for this cause. False for cause 7
    /// only: the preview runs under a constant-true `tracked` predicate, so the untracked
    /// branch is structurally unreachable there (M45 Decision 6, preserved).
    previews: bool,
    /// The code the **committing** door surfaces for this state. `owner-artifact.present`
    /// for six of seven; cause 1 is pre-empted by phase 2's conformance abort (see the
    /// module docs), so its committing-door emitter is that earlier code.
    finalize_code: &'static str,
}

fn no_plant(_repo: &Path) {}

/// All seven `owned_location_violation` causes, in the source order the engine adjudicates
/// them (`crates/engine/src/validate.rs` → `owned_location_violation`).
const CAUSES: &[Cause] = &[
    Cause {
        tag: "empty",
        // A whitespace-only value: `--value ""` is refused by the opaque-scalar floor, so
        // this is the reachable spelling of the empty path through the real write verb.
        value: " ",
        plant: no_plant,
        reason: "the path is empty",
        previews: true,
        finalize_code: "schema-conformance.field-value-conformant",
    },
    Cause {
        tag: "absolute",
        value: "/tmp/jigc-owner-cause-axis-absolute.md",
        plant: no_plant,
        reason: "is absolute, not a repo-relative path",
        previews: true,
        finalize_code: GATE_CODE,
    },
    Cause {
        tag: "parent-dir",
        value: "completions/artifacts/M16/../../../escape.md",
        plant: no_plant,
        reason: "contains a `..` component",
        previews: true,
        finalize_code: GATE_CODE,
    },
    Cause {
        tag: "not-under-home",
        value: "docs/audit.md",
        plant: |repo| {
            fs::create_dir_all(repo.join("docs")).expect("mk docs/");
            fs::write(repo.join("docs/audit.md"), "the audit, misplaced\n").expect("write audit");
        },
        reason: "is not under the owned artifact home",
        previews: true,
        finalize_code: GATE_CODE,
    },
    Cause {
        tag: "names-no-file",
        value: "completions/artifacts/M16/missing.md",
        plant: no_plant,
        reason: "names no file under the repository",
        previews: true,
        finalize_code: GATE_CODE,
    },
    Cause {
        tag: "symlink-escape",
        value: "completions/artifacts/M16/escape.md",
        plant: |repo| {
            fs::create_dir_all(repo.join("outside")).expect("mk outside/");
            fs::write(repo.join("outside/secret.md"), "out of the owned home\n").expect("write");
            fs::create_dir_all(repo.join("completions/artifacts/M16")).expect("mk owned home");
            std::os::unix::fs::symlink(
                repo.join("outside/secret.md"),
                repo.join("completions/artifacts/M16/escape.md"),
            )
            .expect("make escaping symlink");
        },
        reason: "resolves outside the owned artifact home (symlink escape)",
        previews: true,
        finalize_code: GATE_CODE,
    },
    Cause {
        tag: "untracked",
        value: "completions/artifacts/M16/audit.md",
        // Present + safe + under the home, but **gitignored** — the one untracked shape
        // finalize's in-transaction stage cannot repair (`git add` skips it), so the
        // post-stage gate still speaks. Committed before the mint, so nothing is carried.
        plant: |repo| {
            fs::create_dir_all(repo.join("completions/artifacts/M16")).expect("mk owned home");
            fs::write(
                repo.join("completions/artifacts/M16/audit.md"),
                "the genuine audit transcript\n",
            )
            .expect("write artifact");
            fs::write(repo.join(".gitignore"), "completions/artifacts/\n").expect("write ignore");
            git(repo, &["add", ".gitignore"]);
            git(repo, &["commit", "-q", "-m", "ignore the artifact home"]);
        },
        reason: "is present but untracked",
        previews: false,
        finalize_code: GATE_CODE,
    },
];

/// The axis: all seven `owned_location_violation` causes × both surfaces, through the real
/// binary. Causes 1–6 are staging-independent and fire **identically** at the `task
/// validate` preview and the post-stage gate — same code, same key target, byte-identical
/// finding object where both doors emit it. Cause 7 fires at the post-stage gate **only**.
#[test]
fn every_owned_location_cause_previews_except_the_untracked_one() {
    for cause in CAUSES {
        let repo = TempDir::new(cause.tag);
        let home = TempDir::new(&format!("{}-home", cause.tag));
        init_repo(repo.path());
        let pack = TempDir::new(&format!("{}-pack", cause.tag));
        seed_fixture_pack(pack.path());
        list_fixture_pack(repo.path(), pack.path());
        (cause.plant)(repo.path());

        author_completion(repo.path(), home.path(), cause.value);

        // --- Surface 1: the `jigc task validate` preview.
        let validate = jigc(
            repo.path(),
            home.path(),
            &["task", "validate", TASK, "--format", "json"],
        );
        let previewed = gate_findings(&validate, &format!("`task validate` [{}]", cause.tag));

        if cause.previews {
            assert_eq!(
                previewed.len(),
                1,
                "[{}] the staging-independent cause must preview exactly once at \
                 `task validate`, got {previewed:?}",
                cause.tag,
            );
            let finding = &previewed[0];
            assert_eq!(
                finding["severity"], "blocking",
                "[{}] the previewed cause blocks at validate, got {finding}",
                cause.tag,
            );
            assert_eq!(
                finding["key"]["target"], "completion-record:m16-completion#meta/owner-artifact",
                "[{}] the previewed finding keys at the located field leaf, got {finding}",
                cause.tag,
            );
            assert!(
                finding["message"]
                    .as_str()
                    .expect("a string message")
                    .contains(cause.reason),
                "[{}] the previewed message names the cause `{}`, got {finding}",
                cause.tag,
                cause.reason,
            );
            assert_eq!(
                validate.status.code(),
                Some(EXIT_BLOCKED),
                "[{}] a blocking preview exits {EXIT_BLOCKED}; stdout:\n{}\nstderr:\n{}",
                cause.tag,
                String::from_utf8_lossy(&validate.stdout),
                String::from_utf8_lossy(&validate.stderr),
            );
        } else {
            assert!(
                previewed.is_empty(),
                "[{}] the untracked cause is structurally unreachable at the preview \
                 (constant-true `tracked`), got {previewed:?}",
                cause.tag,
            );
            assert_eq!(
                validate.status.code(),
                Some(0),
                "[{}] `task validate` over the untracked state exits 0 — the split M45 \
                 Decision 6 established, preserved; stdout:\n{}\nstderr:\n{}",
                cause.tag,
                String::from_utf8_lossy(&validate.stdout),
                String::from_utf8_lossy(&validate.stderr),
            );
        }

        // --- Surface 2: the post-stage gate, reached through a real `task finalize`.
        let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
        let finalize = jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", TASK, "--format", "json"],
        );
        assert_eq!(
            finalize.status.code(),
            Some(EXIT_BLOCKED),
            "[{}] the committing door blocks at {EXIT_BLOCKED}; stdout:\n{}\nstderr:\n{}",
            cause.tag,
            String::from_utf8_lossy(&finalize.stdout),
            String::from_utf8_lossy(&finalize.stderr),
        );
        assert_eq!(
            git(repo.path(), &["rev-parse", "HEAD"]),
            head_before,
            "[{}] a blocked finalize creates no commit",
            cause.tag,
        );
        let blocked = findings_of(&finalize, &format!("`task finalize` [{}]", cause.tag));
        assert!(
            blocked
                .iter()
                .any(|f| f["code"] == cause.finalize_code && f["severity"] == "blocking"),
            "[{}] the committing door surfaces `{}`, got {blocked:?}",
            cause.tag,
            cause.finalize_code,
        );

        // Where both doors emit the gate, they emit the SAME finding — not merely the same
        // code. A divergence in key target, message, location or route fails here.
        if cause.previews && cause.finalize_code == GATE_CODE {
            let post_stage: Vec<serde_json::Value> = blocked
                .into_iter()
                .filter(|f| f["code"] == GATE_CODE)
                .collect();
            assert_eq!(
                previewed, post_stage,
                "[{}] the preview and the post-stage gate must emit byte-identical \
                 `{GATE_CODE}` findings",
                cause.tag,
            );
        }
    }
}

/// `jigc task finalize --help`'s emitted bytes, whitespace-collapsed — clap wraps at the
/// terminal width, so a phrase assertion must be wrap-insensitive. `--help` needs no repo,
/// no `$HOME` and no pack.
fn finalize_help() -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["task", "finalize", "--help"])
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc task finalize --help` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// M47 Increment 4 — **`--dry-run`'s long help states the scope of its forecast, and the
/// scope is the truth** (`design/surface-contract.md` → law 1, nothing lies).
///
/// T3 rewrote that help to promise the flag *"never forecasts a green the finalize would
/// refuse"*. The clause is unconditional and false: the dry-run branch sits **ahead of**
/// phase 5, so none of the six previewable `owned_location_violation` causes T2 made
/// blocking reaches it — over each of them the forecast returns exit 0 with a manifest for a
/// state the committing run refuses at [`EXIT_BLOCKED`]. Only the report-borne block (cause
/// 1, pre-empted by `schema-conformance.field-value-conformant` inside `plan_finalize`,
/// which the dry-run branch *does* run) and the carryover gate T3 itself moved into the
/// branch are genuinely forecast.
///
/// So this drives the **counterexample axis first** — all seven causes through the real
/// binary, forecast vs commit, the divergence recorded per row — and only then asserts the
/// emitted help bytes: the flat promise is gone, the carryover axis it *does* refuse on is
/// named with its exit, and the gates it does not run are named as the real finalize's to
/// decide. (The help enumerates rather than generalizes — *"it forecasts what it can decide
/// read-only"* would be the same lie again, since these six causes **are** read-only
/// decidable; this door just does not gate on them.) Pinning the sentence without the axis
/// is a claim about prose; pinning the axis without the sentence would leave the prose free
/// to re-widen.
///
/// **Amended at M51 Inc 5 (T5 / EC-20).** The forecast's envelope gained `findings`, and
/// that set is `task validate`'s — so the six previewable causes are now **reported** by
/// the forecast even though it still refuses on none of them. The falsification the arm
/// exists for is untouched and is what the rows below still drive: **exit 0 forecast, exit
/// 3 commit**, over each identical state. What moved is the silence: the row now asserts
/// the gate code is carried exactly where the cause previews (`Cause::previews`), so the
/// forecast can neither go quiet again nor start speaking for the one cause — untracked —
/// that only the post-stage door can see. The help's `jigc task validate` clause went with
/// it: it named a preview the forecast now carries itself, which is the law-1 lie one
/// rider over.
#[test]
fn the_dry_run_forecast_reports_the_gate_borne_causes_without_refusing_and_its_help_says_so() {
    for cause in CAUSES {
        let repo = TempDir::new(&format!("dry-{}", cause.tag));
        let home = TempDir::new(&format!("dry-{}-home", cause.tag));
        init_repo(repo.path());
        let pack = TempDir::new(&format!("dry-{}-pack", cause.tag));
        seed_fixture_pack(pack.path());
        list_fixture_pack(repo.path(), pack.path());
        (cause.plant)(repo.path());

        author_completion(repo.path(), home.path(), cause.value);

        // Cause 1 alone blocks inside `plan_finalize` (the module docs' pre-emption), which
        // the dry-run branch runs — so it is the one cause the forecast genuinely refuses.
        let report_borne = cause.finalize_code != GATE_CODE;

        let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
        let forecast = jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", TASK, "--dry-run", "--format", "json"],
        );
        let forecast_stdout = String::from_utf8_lossy(&forecast.stdout).to_string();

        if report_borne {
            assert_eq!(
                forecast.status.code(),
                Some(EXIT_BLOCKED),
                "[{}] the report-borne cause blocks inside `plan_finalize`, which the \
                 forecast runs; stdout:\n{forecast_stdout}\nstderr:\n{}",
                cause.tag,
                String::from_utf8_lossy(&forecast.stderr),
            );
        } else {
            assert_eq!(
                forecast.status.code(),
                Some(0),
                "[{}] the gate-borne cause is not forecast — the dry-run branch sits ahead \
                 of the phase-5 gate; stdout:\n{forecast_stdout}\nstderr:\n{}",
                cause.tag,
                String::from_utf8_lossy(&forecast.stderr),
            );
            assert_eq!(
                forecast_stdout.contains(GATE_CODE),
                cause.previews,
                "[{}] the forecast reports `{GATE_CODE}` exactly where the cause previews \
                 (M51 Inc 5 / T5 — the forecast's findings are `task validate`'s, and the \
                 untracked cause is the one the preview cannot reach); \
                 got:\n{forecast_stdout}",
                cause.tag,
            );
            assert!(
                forecast_stdout.contains("\"dry_run\": true"),
                "[{}] the forecast is a manifest document; got:\n{forecast_stdout}",
                cause.tag,
            );

            // …and the committing run over that identical state refuses. This IS the
            // falsification of the flat promise: exit 0 forecast, exit 3 commit.
            let finalize = jigc(
                repo.path(),
                home.path(),
                &["task", "finalize", TASK, "--format", "json"],
            );
            assert_eq!(
                finalize.status.code(),
                Some(EXIT_BLOCKED),
                "[{}] the committing door refuses the state the forecast greened; \
                 stdout:\n{}\nstderr:\n{}",
                cause.tag,
                String::from_utf8_lossy(&finalize.stdout),
                String::from_utf8_lossy(&finalize.stderr),
            );
        }

        assert_eq!(
            git(repo.path(), &["rev-parse", "HEAD"]),
            head_before,
            "[{}] neither the forecast nor the refused finalize moves HEAD",
            cause.tag,
        );
    }

    // The emitted help must not carry the flat promise the axis above falsifies…
    let help = finalize_help();
    assert!(
        !help.contains("never forecasts a green the finalize would refuse"),
        "`task finalize --help` must not promise a forecast the phase-5 gates falsify; \
         got:\n{help}"
    );
    // …it names the axis it does forecast, with the exit and the declaration…
    for scoped in [
        "an undeclared carry-over is reported (exit 3) instead of the manifest",
        "`--carry-staged` to forecast the carry",
    ] {
        assert!(
            help.contains(scoped),
            "`task finalize --help` must state the carryover axis it forecasts (`{scoped}`); \
             got:\n{help}"
        );
    }
    // …and names the commit-time gates it does not run, as the real finalize's to decide.
    for deferred in [
        "`owner-artifact`",
        "staging",
        "promotion",
        "the commit hook",
    ] {
        assert!(
            help.contains(deferred),
            "`task finalize --help` must name `{deferred}` among the gates only the real \
             finalize decides; got:\n{help}"
        );
    }
    // …and it no longer sends the reader to `task validate` for a preview it now carries
    // itself (M51 Inc 5 / T5 — the clause the rider falsified).
    assert!(
        !help.contains("previews the owner-artifact causes this does not"),
        "`task finalize --help` must not send the reader elsewhere for findings its own \
         envelope now carries; got:\n{help}"
    );
    assert!(
        help.contains("decided only by the real finalize"),
        "`task finalize --help` must say where the un-forecast gates are decided; got:\n{help}"
    );
}
