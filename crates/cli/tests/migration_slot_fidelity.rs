//! M41 Increment 1, T1 — slot-prose fidelity across **every** shipped
//! author-migration template (V1: the folding-YAML silent-corruption class).
//!
//! For each slot-bearing migrate-guidance template — the five dev-pack doctypes
//! (`adr` / `prd` / `spec` / `arch-doc` / `changelog`) and the seven methodology-pack
//! doctypes (`research` / `vision` / `idea` / `roadmap` / `decisions-log` /
//! `deferral-ledger`, joined at M49 Increment 9 by `completion-record`, whose findings
//! item block gained its first prose slot) — this drives the SHIPPED payload skeleton through the built
//! `jigc` binary: `migrate` composes the guidance, the heredoc skeleton is extracted
//! verbatim (the extract-the-skeleton coupling), a **multi-paragraph + bulleted** body
//! is substituted into every `<<…>>` slot, and the filled payload is piped to
//! `jigc doc author`. The **staged** canonical doc that `author` writes must keep the
//! slot prose's paragraph break and its per-bullet line breaks (`\n` intact).
//!
//! WHY THE STAGED BUFFER (not a finalize round-trip): the fold happens at YAML-parse
//! time inside `doc author` — the staged buffer is the exact, canonical bytes `author`
//! emits, so reading it observes the property at its source. It also makes the coverage
//! **uniform**: it needs none of each doctype's finalize preconditions (arch-doc's
//! in-store `cites` edge, the required-slot / enum gates, the per-doctype committed
//! home), so one parametrized arm covers every template — adding a future template is
//! one row.
//!
//! COUPLING (the red obligation): the substitution is shape-agnostic — it injects the
//! same multi-line body regardless of the template's YAML wrapping. Against a
//! **flow-scalar** template (`key: "<<…>>"`) YAML folds the bullets onto one line and
//! collapses the blank line — the staged prose loses the body verbatim (RED). Against
//! the **block-scalar** flip (`key: |-` then an indented `<<…>>`) the newlines survive
//! (GREEN). A hand-authored block-scalar payload would pass without the template fix and
//! is not valid proof — each arm's payload IS the shipped skeleton, filled.

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
            "jigc-slot-fidelity-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk (`JIGC_PACK_DIR` selects it).
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// The on-disk methodology pack home a `packs.yaml` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally `JIGC_PACK_DIR =
/// pack` and piped `stdin`.
fn run_jigc(
    repo: &Path,
    home: &Path,
    pack: Option<&Path>,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(pack) = pack {
        command.env("JIGC_PACK_DIR", pack);
    }
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both
/// streams.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
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

/// Locate the `doc author <doctype> --from-file - --task … <<'EOF'` heredoc opener in
/// the composed migrate guidance, returning `(the skeleton between the opener and the
/// standalone `EOF`, the `--task` id the opener names)` — both extracted verbatim from
/// the agent-facing artifact the LLM fills + pipes.
fn extract_author_skeleton(composed: &str, doctype: &str) -> (String, String) {
    let opener = format!("doc author {doctype} --from-file");
    let mut lines = composed.lines();
    let mut task = None;
    for line in lines.by_ref() {
        if line.contains(&opener) && line.contains("<<'EOF'") {
            task = line
                .split("--task ")
                .nth(1)
                .and_then(|rest| rest.split_whitespace().next())
                .map(str::to_owned);
            break;
        }
    }
    let task = task.unwrap_or_else(|| {
        panic!("no `--task` id on the `doc author {doctype}` opener:\n{composed}")
    });
    let mut body = String::new();
    for line in lines {
        if line == "EOF" {
            return (body, task);
        }
        body.push_str(line);
        body.push('\n');
    }
    panic!("no `doc author {doctype}` heredoc skeleton in the composed guidance:\n{composed}");
}

/// The multi-paragraph + bulleted body substituted into every slot: a lead paragraph, a
/// blank-line paragraph break, then three bullets each on their own line. Under a folding
/// flow scalar the bullets collapse to one line and the blank line to a single `\n`; the
/// block scalar keeps every break.
const BODY: &str = "This is the first mapped paragraph of migrated prose.\n\n\
- first bulleted point survives on its own line\n\
- second bulleted point survives on its own line\n\
- third bulleted point survives on its own line";

/// Substitute `BODY` into every `<<…>>` slot of the skeleton — shape-agnostically. The
/// continuation lines are indented to the column of the `<<` marker, which is exactly the
/// block-scalar content indent in the flipped templates (and folded away under a flow
/// scalar), so the SAME filled payload is intact under a block scalar and folded under a
/// flow scalar. `BODY` carries no `<<`/`>>`, so a left-to-right scan is unambiguous.
fn fill_slots(skeleton: &str) -> String {
    let mut out = String::new();
    let mut rest = skeleton;
    while let Some(open) = rest.find("<<") {
        let line_start = rest[..open].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let col = open - line_start; // ASCII prefix ⇒ byte offset == column
        let indent = " ".repeat(col);
        out.push_str(&rest[..open]);
        out.push_str("<<");
        let after = &rest[open + 2..];
        let close = after.find(">>").expect("a matching `>>` closes each slot");
        let filled: String = BODY
            .split('\n')
            .enumerate()
            .map(|(i, line)| {
                if i == 0 || line.is_empty() {
                    line.to_string()
                } else {
                    format!("{indent}{line}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        out.push_str(&filled);
        out.push_str(">>");
        rest = &after[close + 2..];
    }
    out.push_str(rest);
    out
}

/// Apply the `(placeholder, value)` field replacements, then fill every slot with `BODY`.
/// The replacements fill only the inline fields `doc author` *validates* at write time
/// (enums, dates, the `cites` ref shape) so the author call reaches the slot-render path;
/// free-text placeholders (titles) are left as-is — the skeleton drives verbatim.
fn fill_payload(skeleton: &str, fields: &[(&str, &str)]) -> String {
    let mut filled = skeleton.to_owned();
    for (from, to) in fields {
        assert!(
            filled.contains(from),
            "the shipped skeleton must carry the field placeholder `{from}`:\n{skeleton}",
        );
        filled = filled.replace(from, to);
    }
    fill_slots(&filled)
}

/// The pack a template ships in — selects how the repo is configured before `migrate`.
#[derive(Clone, Copy)]
enum Pack {
    /// Dev pack, selected by `JIGC_PACK_DIR`; `jigc setup` installs it before migrate.
    Dev,
    /// Methodology pack, composed over the embedded dev base by a `packs.yaml` entry.
    Methodology,
}

/// Recursively find the staged canonical doc whose file name is `<doctype>:*.md` under
/// the task working area (skipping the auto-provisioned `commit:*.md`).
fn find_staged_doc(dir: &Path, prefix: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_staged_doc(&path, prefix) {
                return Some(found);
            }
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str())
            && name.starts_with(prefix)
            && name.ends_with(".md")
        {
            return Some(path);
        }
    }
    None
}

/// The property, once per template: author the SHIPPED skeleton (slots filled with the
/// multi-paragraph + bulleted `BODY`) through the real binary, then assert the staged
/// canonical doc keeps every paragraph break and per-bullet line break verbatim.
fn assert_slot_prose_fold_safe(
    doctype: &str,
    pack: Pack,
    foreign_name: &str,
    foreign_content: &str,
    fields: &[(&str, &str)],
) {
    let repo = TempDir::new(doctype);
    let home = TempDir::new("home");
    init_repo(repo.path());

    let pack_dir: Option<PathBuf> = match pack {
        Pack::Dev => Some(dev_pack()),
        Pack::Methodology => {
            fs::write(
                repo.path().join(".jigc").join("config").join("packs.yaml"),
                format!("packs:\n  - {}\n", methodology_pack_tree().display()),
            )
            .expect("write packs.yaml naming the methodology pack");
            None
        }
    };
    let pack_ref = pack_dir.as_deref();

    // A committed foreign source `migrate --as <doctype>` can stage as read-only context.
    let foreign_path = repo.path().join(foreign_name);
    if let Some(parent) = foreign_path.parent() {
        fs::create_dir_all(parent).expect("create foreign source parent dir");
    }
    fs::write(&foreign_path, foreign_content).expect("write foreign source");
    git(repo.path(), &["add", foreign_name]);
    git(repo.path(), &["commit", "-q", "-m", "track foreign source"]);

    if matches!(pack, Pack::Dev) {
        ok_stdout(
            run_jigc(repo.path(), home.path(), pack_ref, &["setup"], None),
            "jigc setup",
        );
    }

    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack_ref,
            &["migrate", foreign_name, "--as", doctype],
            None,
        ),
        &format!("jigc migrate --as {doctype}"),
    );

    let (skeleton, task) = extract_author_skeleton(&composed, doctype);
    // Every `<<…>>` slot in the shipped skeleton gets `BODY`; the staged doc must carry
    // `BODY` back once per slot. Counting (not `contains`) is what makes a MULTI-slot
    // template red-capable: were a single slot a folding flow scalar, only that slot's
    // copy would be corrupted while the others still matched — a bare `contains` would
    // pass. A slot count of zero would be a vacuous assertion, so guard it.
    let slots = skeleton.matches("<<").count();
    assert!(
        slots > 0,
        "the `{doctype}` skeleton carries no `<<…>>` slot:\n{skeleton}"
    );
    let payload = fill_payload(&skeleton, fields);

    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack_ref,
            &[
                "doc",
                "author",
                doctype,
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            Some(payload.as_bytes()),
        ),
        &format!("jigc doc author {doctype}"),
    );

    let task_docs = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("docs");
    let staged = find_staged_doc(&task_docs, &format!("{doctype}:")).unwrap_or_else(|| {
        panic!("no staged `{doctype}:*.md` under the task working area {task_docs:?}")
    });
    let staged_doc = fs::read_to_string(&staged).expect("read the staged canonical doc");
    let survived = staged_doc.matches(BODY).count();
    assert_eq!(
        survived, slots,
        "every one of the {slots} `{doctype}` slot(s) must keep its paragraph break + \
         per-bullet line breaks verbatim (a folding flow scalar collapses them); only \
         {survived} survived — staged {staged:?}:\n{staged_doc}",
    );
}

/// One `#[test]` per slot-bearing author-migration template — the fold-safety table.
/// A new template is one added row.
macro_rules! fold_safe_cases {
    ($( $name:ident : $doctype:literal , $pack:expr , $foreign:literal , $content:literal ,
        [ $( ($from:literal , $to:literal) ),* $(,)? ] ; )*) => {
        $(
            #[test]
            fn $name() {
                assert_slot_prose_fold_safe(
                    $doctype,
                    $pack,
                    $foreign,
                    $content,
                    &[ $( ($from, $to) ),* ],
                );
            }
        )*
    };
}

fold_safe_cases! {
    // ---- dev pack ----
    // The generated `{{schema:adr}}` skeleton (M43) carries no inline-field
    // placeholders: `status`/`date`/`supersedes` are default/CLI-stamped/optional, so
    // they render tree-only and the skeleton is pure slots.
    adr_slot_prose_fold_safe: "adr", Pack::Dev,
        "docs/adr/0001-fidelity.md",
        "# 1. A decision\n\n## Status\n\nAccepted\n\n## Context\n\nForces.\n\n\
## Decision\n\nWe chose.\n\n## Consequences\n\nTradeoffs.\n",
        [];

    prd_slot_prose_fold_safe: "prd", Pack::Dev,
        "old-prd.md",
        "# Some PRD\n\n## Vision\n\nDo the thing.\n\n## Requirements\n\n- must A\n",
        [];

    spec_slot_prose_fold_safe: "spec", Pack::Dev,
        "old-spec.md",
        "# Some spec\n\n## Goal\n\nDeliver it.\n\n## Acceptance Criteria\n\n- does A\n",
        [];

    // The generated `{{schema:arch-doc}}` skeleton (M43) omits the optional `cites`
    // ref and the pack-typed `implemented-by` (tree-only), so it is pure slots.
    arch_doc_slot_prose_fold_safe: "arch-doc", Pack::Dev,
        "old-arch.md",
        "# The subsystem\n\n## Overview\n\nWhat it owns.\n\n## Components\n\n### A part\n\nDoes work.\n",
        [];

    // The generated `{{schema:changelog}}` skeleton (M43) authors the category as the
    // item's `title:` (the enum id-source, members offered); `date` is CLI-stamped so
    // it renders tree-only. The enum-title placeholder appears twice (the
    // `unreleased-changes` group + the release's nested `changes` group) — both must
    // hold a valid member for the write to land.
    changelog_slot_prose_fold_safe: "changelog", Pack::Dev,
        "old-changelog.md",
        "# Changelog\n\n## 1.0.0\n\n### Added\n\n- a feature\n",
        [
            ("\"<added | changed | deprecated | removed | fixed | security>\"", "added"),
        ];

    // ---- methodology pack ----
    // The generated `{{schema:<doctype>}}` skeletons (M43 T5) render CLI-stamped
    // `date` fields tree-only, so the old `<historical-date>` placeholder rows are
    // gone; the remaining fill-me fields are free-text placeholders (left as-is —
    // the skeleton drives verbatim) plus the enum members `doc author` validates.
    research_slot_prose_fold_safe: "research", Pack::Methodology,
        "old-notes.md",
        "# Spreadsheet pain notes\n\nUsers re-import hourly.\n",
        [];

    vision_slot_prose_fold_safe: "vision", Pack::Methodology,
        "old-vision.md",
        "# Vision\n\n## Thesis\n\nThe claim.\n\n## Invariants\n\n- one\n\n## Open questions\n\n- q\n",
        [];

    idea_slot_prose_fold_safe: "idea", Pack::Methodology,
        "old-idea.md",
        "# A parked idea\n\nSome shaped direction worth keeping.\n",
        [];

    roadmap_slot_prose_fold_safe: "roadmap", Pack::Methodology,
        "old-roadmap.md",
        "# Roadmap\n\n## M1 — the first milestone\n\nGoal: ship it.\n",
        [];

    decisions_log_slot_prose_fold_safe: "decisions-log", Pack::Methodology,
        "old-decisions.md",
        "# Decisions\n\n## 2020-01-01 chose X\n\nBecause reasons.\n",
        [];

    deferral_ledger_slot_prose_fold_safe: "deferral-ledger", Pack::Methodology,
        "old-deferrals.md",
        "# Deferrals\n\n## An owed thing\n\nDeferred until later.\n",
        [("\"<Decision | Idea>\"", "Decision")];

    // M49 Inc-9 T1 — `completion-record` joined this table the day its findings item
    // block gained the optional `detail` prose slot; before that it carried no `<<…>>`
    // at all and was excluded on that premise, asserted by its own arm. The foreign
    // source must STATE AN OUTCOME: the migration's refusal rung declines an
    // outcome-less document rather than fabricating a verdict.
    completion_record_slot_prose_fold_safe: "completion-record", Pack::Methodology,
        "old-close.md",
        "# M1 close\n\nMilestone M1 shipped — all criteria met and the audit passed.\n",
        [
            ("\"<green | red>\"", "green"),
            ("\"<blocking | advisory | HIGH | MEDIUM | LOW>\"", "HIGH"),
            ("\"<fixed | deferred | contested>\"", "fixed"),
        ];
}
