//! End-to-end proof for the `{{fill: extra-guidance}}` extension point (Increment 4,
//! T6 — the marquee deliverable) and the `jigc config fill` verb (T5).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo over the
//! **unmodified pack**: the pack `implement` step ships the `{{fill: extra-guidance}}`
//! point (T6), so every flow here targets that pack-declared point — no project step
//! shadow seeds it (`design/worked-examples.md` → 3c; `design/overrides.md` → The
//! `{{fill:}}` placeholder). The four done-criterion flows, all proven on emitted
//! bytes / exit status through the binary:
//!
//! (a) bare `jigc start` over the unmodified pack composes `single-task` with the
//!     **unfilled** point contributing nothing — the inc-3 baseline is unchanged.
//! (b) `config fill step:implement#extra-guidance --from-file -` (content via stdin)
//!     writes `.jigc/config/fills/extra-guidance.md` (id = fill-id) + the `slot-fill`
//!     delta, and a subsequent bare `jigc start` composes the house rule into the
//!     `implement` step body.
//! (c) a `slot-fill` aimed at an undeclared fill-id makes `jigc start` fail with the
//!     blocking `slot-fill-orphan` finding (the verb also rejects it at write time).
//! (d) fill content containing a nested `{{fill:}}` is rejected at the verb's write
//!     time, and a hand-edited manifest smuggling one past the verb is blocked at
//!     compose by the `fill-survivor` gate.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-config-fill-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// Initialize a real git repo with one commit (composition mints, which reads HEAD),
/// plus a `.jigc/config/` project layer whose manifest flips `default-workflow` to
/// `single-task`. **No project `steps/` shadow** — the `{{fill: extra-guidance}}`
/// point the flows target is declared by the *unmodified pack* `implement` step (T6).
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("create project layer");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n",
    )
    .expect("write project manifest");
}

/// Run `jigc config fill <target> --from-file -` with `content` piped on stdin.
fn run_fill(repo: &Path, home: &Path, target: &str, content: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["config", "fill", target, "--from-file", "-"])
        .current_dir(repo)
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(content.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("wait for the jigc binary")
}

/// Run `jigc start <args>` with `cwd = repo` and `$HOME = home`.
fn run_start(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// The inc-3 / M3 `single-task` no-delta composed view — byte-identical to
/// `start_compose.rs`'s `NO_DELTA_SINGLE_TASK_GOLDEN`. Adding the unfilled
/// `{{fill: extra-guidance}}` point to the pack `implement` step must compose to
/// **this** exact baseline (an unfilled point → empty default; the line collapses,
/// no blank line left behind). The marquee constraint of T6.
const NO_FILL_SINGLE_TASK_GOLDEN: &str = "\
Reason about the change. The intent is:
add rate limiter

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.

Implement the change directly in the working tree. When done, stage the
commit prose:

Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -`
<<author: commit:add-rate-limiter#summary>>

If a decision is warranted, create an ADR and author its slots:

Run: `jigc doc create adr --title <TITLE>`

If your decision supersedes an earlier one, here is that decision for
reference — make your consequences explain what changes:

Validate and commit the task as one logical commit:

Run: `jigc task finalize add-rate-limiter`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

#[test]
fn unfilled_pack_point_composes_byte_identical_to_the_baseline() {
    // (a): the pack `implement` step ships `{{fill: extra-guidance}}`; with no
    // `slot-fill` delta the point resolves to the empty pack default, so a bare
    // `jigc start "<intent>"` composes `single-task` byte-identical to the inc-3
    // baseline — the unfilled point contributes nothing. Proven on emitted bytes.
    let repo = TempDir::new("baseline");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "the unfilled-point compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        stdout, NO_FILL_SINGLE_TASK_GOLDEN,
        "an unfilled `{{fill: extra-guidance}}` point must compose byte-identical to the inc-3 baseline",
    );
}

#[test]
fn fill_writes_native_file_and_delta_then_composes_into_implement() {
    // (b), flow 3c verbatim over the unmodified pack:
    // `config fill step:implement#extra-guidance --from-file -` with the house rule
    // on stdin writes `fills/extra-guidance.md` + the slot-fill delta, and a bare
    // `jigc start "<intent>"` composes the house rule into the pack `implement`
    // body — proven on emitted bytes through the binary.
    let repo = TempDir::new("fill");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let house_rule =
        "Confirm a changelog entry exists for any user-facing change before finalizing.";
    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#extra-guidance",
        &format!("{house_rule}\n"),
    );
    assert!(
        out.status.success(),
        "`jigc config fill ...` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The native fill file landed at `.jigc/config/fills/extra-guidance.md` (id = fill-id).
    let config = repo.path().join(".jigc").join("config");
    let native = config.join("fills").join("extra-guidance.md");
    assert_eq!(
        fs::read_to_string(&native).expect("the native fill file must be written"),
        format!("{house_rule}\n"),
        "the fill file must carry the stdin content verbatim",
    );

    // The slot-fill delta landed in the manifest (preserving the `scalar:` flip).
    let manifest = fs::read_to_string(config.join("manifest.yaml")).expect("manifest written");
    let doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&manifest).expect("manifest is valid YAML");
    assert_eq!(
        doc.get("scalar")
            .and_then(|s| s.get("default-workflow"))
            .and_then(serde_yaml_ng::Value::as_str),
        Some("single-task"),
        "the `scalar:` flip must survive the delta append; got:\n{manifest}",
    );
    let delta = doc
        .get("deltas")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .and_then(|s| s.first())
        .expect("one delta entry");
    assert_eq!(
        delta.get("kind").and_then(serde_yaml_ng::Value::as_str),
        Some("slot-fill"),
        "the recorded delta must be a slot-fill; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("target").and_then(serde_yaml_ng::Value::as_str),
        Some("step:implement#extra-guidance"),
        "the slot-fill target must carry `step:<id>#<fill-id>`; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("content").and_then(serde_yaml_ng::Value::as_str),
        Some("fills/extra-guidance.md"),
        "`content:` must reference the native fill file; got:\n{manifest}",
    );

    // The runnable fill lands end-to-end: a bare `jigc start "<intent>"` composes the
    // house rule into the pack `implement` body, in place of the `{{fill:}}` point —
    // proven on emitted bytes.
    let compose = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(compose.stdout).expect("utf-8 stdout");
    assert!(
        compose.status.success(),
        "the compose over the filled step must exit 0; got {:?}\nstderr:\n{}",
        compose.status,
        String::from_utf8_lossy(&compose.stderr),
    );
    assert!(
        stdout.contains(house_rule),
        "the filled house rule must compose into the implement body; got:\n{stdout}",
    );
    // The `{{fill:}}` point itself never survives into the composed output.
    assert!(
        !stdout.contains("{{fill:"),
        "no `{{{{fill:}}}}` point may survive composition; got:\n{stdout}",
    );
}

#[test]
fn orphan_target_is_rejected_at_write_time_and_writes_nothing() {
    // (c), write-time half: a fill aimed at a `{{fill:}}` point the resolved pack
    // step body does not declare is rejected non-zero with its route, writing
    // neither the native fill file nor a delta.
    let repo = TempDir::new("orphan-verb");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    let manifest_path = config.join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#nonesuch",
        "some content\n",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "an orphan fill target must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the rejection must name the absent point and carry a route; got:\n{stderr}",
    );

    assert!(
        !config.join("fills").exists(),
        "a rejected fill must write no native fill file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected fill must leave the manifest byte-unchanged",
    );
}

#[test]
fn orphan_target_blocks_compose_when_hand_edited_past_the_verb() {
    // (c), compose-time half: a hand-edited manifest with a `slot-fill` targeting a
    // `<fill-id>` no resolved pack step body declares (`nonesuch`) makes `jigc start`
    // fail with the blocking `slot-fill-orphan` finding, surfaced with its route —
    // the closed surface holds for the hand-authoring path too.
    let repo = TempDir::new("orphan-compose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n\
         deltas:\n\
         \x20 - kind: slot-fill\n\
         \x20   target: step:implement#nonesuch\n\
         \x20   content: fills/nonesuch.md\n",
    )
    .expect("write a hand-edited orphan slot-fill manifest");
    fs::create_dir_all(config.join("fills")).expect("mk fills/");
    fs::write(config.join("fills").join("nonesuch.md"), "Some guidance.\n")
        .expect("write the native fill content");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "a hand-edited orphan slot-fill must block compose non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the orphan block must name the absent point and carry a route; got:\n{stderr}",
    );
}

#[test]
fn nested_fill_in_content_is_rejected_at_write_time_and_writes_nothing() {
    // (d), write-time half: fill content that itself contains a `{{fill:}}` is
    // rejected non-zero with its route (the no-nested rule — phase 5 does not
    // re-run), writing nothing.
    let repo = TempDir::new("nested-verb");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    let manifest_path = config.join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#extra-guidance",
        "house rule\n{{fill: another}}\n",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "fill content containing `{{{{fill:}}}}` must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("route:"),
        "the rejection must carry a route; got:\n{stderr}",
    );

    assert!(
        !config.join("fills").exists(),
        "a rejected fill must write no native fill file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected fill must leave the manifest byte-unchanged",
    );
}

#[test]
fn mid_line_nested_fill_in_content_is_rejected_at_write_time_and_writes_nothing() {
    // (d), mid-line variant: a `{{fill:}}` token embedded *inside* a content line
    // (not alone on its own line) is still a nested fill — phase 5 does not re-run,
    // so it would leak the literal token into agent output. The no-nested guard is
    // token-based, so the verb rejects it non-zero with its route, writing nothing.
    let repo = TempDir::new("nested-midline-verb");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    let manifest_path = config.join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#extra-guidance",
        "house rule with {{fill: nested-thing}} embedded\n",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "fill content with a mid-line `{{{{fill:}}}}` must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("nested-thing"),
        "the rejection must name the mid-line nested fill; got:\n{stderr}",
    );

    assert!(
        !config.join("fills").exists(),
        "a rejected mid-line fill must write no native fill file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected mid-line fill must leave the manifest byte-unchanged",
    );
}

#[test]
fn mid_line_nested_fill_hand_edited_past_the_verb_never_leaks_into_output() {
    // (d), mid-line compose half: a hand-edited `fills/extra-guidance.md` carrying a
    // mid-line `{{fill:}}` (smuggled past the verb) is spliced in by phase 5; the
    // mid-line token survives — `jigc start` must fail with the blocking
    // `fill-survivor` finding, and no composed output may carry the literal token.
    let repo = TempDir::new("nested-midline-compose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n\
         deltas:\n\
         \x20 - kind: slot-fill\n\
         \x20   target: step:implement#extra-guidance\n\
         \x20   content: fills/extra-guidance.md\n",
    )
    .expect("write the slot-fill manifest");
    fs::create_dir_all(config.join("fills")).expect("mk fills/");
    fs::write(
        config.join("fills").join("extra-guidance.md"),
        "house rule with {{fill: nested-thing}} embedded\n",
    )
    .expect("write fill content smuggling a mid-line nested fill");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "a surviving mid-line `{{{{fill:}}}}` must block compose non-zero; got {:?}",
        out.status,
    );
    assert!(
        !stdout.contains("{{fill:"),
        "no composed output may leak an unresolved `{{{{fill:}}}}` token; got stdout:\n{stdout}",
    );
    assert!(
        stderr.contains("nested-thing") && stderr.contains("survives composition"),
        "the survivor block must name the surviving mid-line fill; got:\n{stderr}",
    );
}

#[test]
fn nested_fill_hand_edited_past_the_verb_is_blocked_at_compose_by_the_survivor_check() {
    // (d), compose-time half: a hand-edited `fills/extra-guidance.md` carrying a
    // nested `{{fill:}}` (smuggled past the verb's write-time guard) targets the real
    // pack point, so phase 5 splices it in and the nested `{{fill:}}` survives —
    // `jigc start` fails with the blocking `fill-survivor` finding and its route.
    let repo = TempDir::new("nested-compose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n\
         deltas:\n\
         \x20 - kind: slot-fill\n\
         \x20   target: step:implement#extra-guidance\n\
         \x20   content: fills/extra-guidance.md\n",
    )
    .expect("write the slot-fill manifest");
    fs::create_dir_all(config.join("fills")).expect("mk fills/");
    fs::write(
        config.join("fills").join("extra-guidance.md"),
        "house rule\n{{fill: another}}\n",
    )
    .expect("write fill content smuggling a nested fill");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "a surviving nested `{{{{fill:}}}}` must block compose non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("another") && stderr.contains("survives composition"),
        "the survivor block must name the surviving nested fill; got:\n{stderr}",
    );
}
