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
///
/// The view opens with the frontend's `task minted:` header (M42) — this compose mints
/// `add-rate-limiter` (`workflow-dialect.md` → The `task minted:` header).
const NO_FILL_SINGLE_TASK_GOLDEN: &str = "\
task minted: add-rate-limiter

Reason about the change. The intent is:
add rate limiter

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged.

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task add-rate-limiter`

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects). Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address:

jigc doc set-slot adr:<slug>#context --from-file - --task add-rate-limiter
jigc doc set-slot adr:<slug>#decision --from-file - --task add-rate-limiter
jigc doc set-slot adr:<slug>#consequences --from-file - --task add-rate-limiter

The `options` slot is optional — fill it only when alternatives were genuinely
weighed; omit it when the call was obvious:

jigc doc set-slot adr:<slug>#options --from-file - --task add-rate-limiter

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

If the change is user-facing — a feature, a fix, or a behaviour a user would
notice — record it on the changelog. Create-or-update the singleton first:

Run: `jigc doc create changelog --title Changelog --task add-rate-limiter`

If a changelog is ALREADY committed, the CLI does not mint a fresh one — it copies
the committed body in as your edit base, so what you author below APPENDS to the
entries already there and any slot you set overwrites. Author only what THIS change
adds; an entry the committed changelog already holds would double.

Then author the entry itself. The target schema and its batch payload, both
generated from the resolved `changelog` schema, follow — one call places the whole
entry over the staged buffer:

The `changelog` schema — the managed singleton at `CHANGELOG.md`.

- `meta` (front-matter fields):
    - `schema-version`: int — CLI-stamped (schema-version) unless authored
- `unreleased-changes`: repeatable items, one per `category`:
    - `category`: enum, one of: added | changed | deprecated | removed | fixed | security — the item's id-source (authored as the item's `title:` payload key)
    - `notes`: prose slot — One bullet per change in this category.
- `releases`: repeatable items, one per `title`:
    - `title`: string — the item's id-source (authored as the item's `title:` payload key)
    - `date`: date — CLI-stamped (on-create) unless authored
    - `link`: string — optional
    - `changes`: repeatable items, one per `category`:
        - `category`: enum, one of: added | changed | deprecated | removed | fixed | security — the item's id-source (authored as the item's `title:` payload key)
        - `notes`: prose slot — One bullet per change in this category.

Author the whole document in ONE `jigc doc author` batch payload — fill each `<…>` value. The `<<…>>` wrapping on slot prose is REQUIRED literal syntax: keep the `<<`/`>>` markers and replace only the text between them (an inline field takes a bare value — wrapping one is rejected). Inside slot prose, the reserved depths differ by slot — headings must sit at `####` or deeper in `unreleased-changes.notes`, at `#####` or deeper in `releases.changes.notes`. Setext headings are rejected. An entry marked `# optional` may be omitted entirely. A repeatable's demonstrated item entry is a template — repeat one `- title:` entry per item. Multi-line slot prose wraps WHOLE inside one `<<…>>` pair within its block scalar — a two-line slot is authored as

<slot-id>: |-
  <<The first line of the prose
  and its second line, inside the SAME pair.>>

Pipe the payload on stdin:

jigc doc author changelog --from-file - --task add-rate-limiter <<'EOF'
title: Changelog
sections:
  - id: unreleased-changes
    items:
      - title: \"<added | changed | deprecated | removed | fixed | security>\"
        set:
          notes: |-
            <<One bullet per change in this category.>>
  - id: releases
    items:
      - title: \"<the title>\"
        set:
          link: \"<the link>\" # optional — omit if unused
        sections:
          - id: changes
            items:
              - title: \"<added | changed | deprecated | removed | fixed | security>\"
                set:
                  notes: |-
                    <<One bullet per change in this category.>>
EOF

Author the entry under `unreleased-changes` — the staging area — and OMIT the
`releases` entry: work records a staged change, it does not cut a release (cutting
one is its own deliberate pass, `jigc start --workflow record-change`). The item
`title` is the CATEGORY, one of the enum members the schema above lists, and the
category is the group's id — so several bullets in the same category are ONE item
with all its bullets merged into that item's `notes`.

If your decision supersedes an earlier one, set `supersedes` on the ADR; the
superseded decision then appears below for reference, so your consequences can
explain what changes (nothing appears if it supersedes none).

When done, set the required Conventional-Commits type — your editorial call on
what this change does. The subject renders as `<type>(<scope>): <summary>`, so
write the summary without a type or scope prefix of its own — the `type` field
already carries it. Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address. Set the type, then stage the summary prose:

Run: `jigc doc set-field commit:add-rate-limiter#type --value <COMMIT_TYPE> --task add-rate-limiter`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter`
<<author: commit:add-rate-limiter#summary>>

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:add-rate-limiter#scope --value <area> --task add-rate-limiter
jigc doc set-slot commit:add-rate-limiter#body --from-file - --task add-rate-limiter

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:add-rate-limiter#trailers --title Co-Authored-By --task add-rate-limiter
jigc doc set-field commit:add-rate-limiter#trailers/<id>/value --value \"Name <email>\" --task add-rate-limiter

Validate and commit the task as one logical commit. Finalize commits only the
staged set plus the docs it manages; unstaged edits and untracked files are left
out, and with nothing staged over a dirty tree it refuses. Anything still staged
from BEFORE this task was minted makes finalize refuse too (one blocking finding
per carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate add-rate-limiter` — it
previews part of what finalize gates on (this task's content findings, the
carryover gate, and the owner-artifact causes that need no staging), without
committing anything; the staged set, promotion and the commit itself are decided
at finalize.

Run: `jigc task finalize add-rate-limiter`
resume: `jigc start --task add-rate-limiter`   — re-composes this workflow if context is lost
what's-left: `jigc task validate add-rate-limiter`   — previews part of the finalize gate: this task's content findings, the carryover gate, and the owner-artifact causes that need no staging; the staged set, promotion and the commit surface at finalize
task scope: `jigc doc` writes default to the single active task; `--task add-rate-limiter` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; once a sibling task commits a path this one also touches, resuming or finalizing here blocks and names the overlapping paths
create-gates: adr, changelog
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

#[test]
fn fill_prose_with_inline_braces_stays_inert_and_composes_verbatim() {
    // Regression (M17 inc-4 validation): agent-authored fill prose carrying a
    // mid-line `{{…}}` token (e.g. guidance *about* template syntax) is accepted
    // at write time — the write-time guard rejects only nested `{{fill:}}` — so it
    // must stay **inert** at compose: emitted verbatim with exit 0, never a
    // front-door error bricking every later `jigc start`. The inline data-value
    // scanner resolves what resolves and leaves the rest; blocking stays with the
    // lone-line classes. Proven on emitted bytes through the binary.
    let repo = TempDir::new("inline-braces");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let prose = "Note: template syntax like {{version}} appears in our docs; leave it as-is.";
    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#extra-guidance",
        &format!("{prose}\n"),
    );
    assert!(
        out.status.success(),
        "the mid-line `{{{{…}}}}` fill is accepted at write time; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    let compose = run_start(repo.path(), home.path(), &["fill probe task"]);
    let stdout = String::from_utf8(compose.stdout).expect("utf-8 stdout");
    assert!(
        compose.status.success(),
        "a compose over fill prose with an unresolvable mid-line `{{{{…}}}}` must exit 0; got {:?}\nstderr:\n{}",
        compose.status,
        String::from_utf8_lossy(&compose.stderr),
    );
    assert!(
        stdout.contains(prose),
        "the fill prose must compose verbatim, the inert token included; got:\n{stdout}",
    );
}

#[test]
fn fill_prose_with_resolvable_token_stays_verbatim_while_pack_prose_substitutes() {
    // The determinism boundary applied to inline data-values (DECISIONS 2026-06-12):
    // inline `{{<path>}}` substitution exists FOR pack/step-authored composed prose
    // (the M17 inc-4 `--task {{task.id}}` stamping). Agent/project-authored slot-fill
    // prose is the LLM/human's prose — the CLI never rewrites it, so a *resolvable*
    // token in fill content composes verbatim too (not only the unresolvable one the
    // sibling test pins). Proven on emitted bytes through the binary, both halves.
    let repo = TempDir::new("resolvable-fill");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Half 1 — fill-authored prose is never rewritten, resolvable token included.
    let prose = "House rule: restate {{task.intent}} before you begin.";
    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#extra-guidance",
        &format!("{prose}\n"),
    );
    assert!(
        out.status.success(),
        "the fill is accepted at write time; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    let compose = run_start(repo.path(), home.path(), &["fill probe task"]);
    let stdout = String::from_utf8(compose.stdout).expect("utf-8 stdout");
    assert!(
        compose.status.success(),
        "compose over the filled point must exit 0; got {:?}\nstderr:\n{}",
        compose.status,
        String::from_utf8_lossy(&compose.stderr),
    );
    assert!(
        stdout.contains(prose),
        "fill-authored prose is never rewritten: the resolvable `{{{{task.intent}}}}` \
         must compose verbatim; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("restate fill probe task"),
        "the CLI must not substitute the task's intent into fill-authored prose; got:\n{stdout}",
    );
    // Non-vacuity: the same token resolves where the PACK wrote it (the `locate`
    // step's lone-line `{{task.intent}}`), so resolution itself is intact.
    assert!(
        stdout.contains("fill probe task"),
        "the pack-authored `{{{{task.intent}}}}` line must still resolve; got:\n{stdout}",
    );

    // Half 2 — the inc-4 stamping feature is pinned non-vacuously: a pack-authored
    // step line carrying a mid-line `{{task.id}}` (`author-arch-doc`'s literal
    // `--task {{task.id}}` lines) still substitutes the minted task id.
    let repo2 = TempDir::new("pack-stamp");
    init_repo(repo2.path());
    let stamped = run_start(
        repo2.path(),
        home.path(),
        &[
            "--workflow",
            "architecture-documentation",
            "document the cache layer",
        ],
    );
    let stdout = String::from_utf8(stamped.stdout).expect("utf-8 stdout");
    assert!(
        stamped.status.success(),
        "`jigc start --workflow architecture-documentation` must exit 0; got {:?}\nstderr:\n{}",
        stamped.status,
        String::from_utf8_lossy(&stamped.stderr),
    );
    assert!(
        stdout.contains("--task document-the-cache-layer"),
        "the pack-authored mid-line `{{{{task.id}}}}` must substitute the minted id; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("{{task.id}}"),
        "no literal `{{{{task.id}}}}` may survive in pack-authored composed prose; got:\n{stdout}",
    );
}
