//! M41 Increment 1, T1 — slot-prose fidelity across the shipped author-migration
//! templates (V1: the folding-YAML silent-corruption class).
//!
//! Drives the SHIPPED `adr` (dev pack) + `research` (methodology pack) migrate-guidance
//! payload skeletons through the built `jigc` binary — `migrate` composes the guidance,
//! the skeleton is extracted verbatim (the extract-the-skeleton coupling), and a
//! **multi-paragraph + bulleted** body is substituted into every `<<…>>` slot before it
//! is piped to `jigc doc author`. The committed slot prose must keep its paragraph break
//! and its per-bullet line breaks (`\n` intact).
//!
//! COUPLING (the red obligation): the substitution is shape-agnostic — it injects the
//! same multi-line body regardless of the template's YAML wrapping. Against a
//! **flow-scalar** template (`key: "<<…>>"`) YAML folds the bullets onto one line and
//! collapses the blank line — the committed prose loses the body verbatim (RED). Against
//! the **block-scalar** flip (`key: |-` then an indented `<<…>>`) the newlines survive
//! (GREEN). A hand-authored block-scalar payload would pass without the template fix and
//! is not valid proof — this test's payload IS the shipped skeleton, filled.

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
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The on-disk methodology pack home a `packs.yaml` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Extract the `doc author <doctype> --from-file -` heredoc payload skeleton from the
/// composed migrate guidance (between the `<<'EOF'` opener and the standalone `EOF`
/// terminator) — the agent-facing artifact the LLM fills + pipes, byte-for-byte.
fn extract_author_skeleton(composed: &str, doctype: &str) -> String {
    let opener = format!("doc author {doctype} --from-file");
    let mut lines = composed.lines();
    for line in lines.by_ref() {
        if line.contains(&opener) && line.contains("<<'EOF'") {
            break;
        }
    }
    let mut body = String::new();
    for line in lines {
        if line == "EOF" {
            return body;
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

#[test]
fn adr_slot_prose_survives_multi_paragraph_and_bullets() {
    let repo = TempDir::new("adr");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A committed foreign ADR at the off-canonical `docs/adr/` path.
    let rel = "docs/adr/0001-fidelity.md";
    fs::create_dir_all(repo.path().join("docs").join("adr")).expect("create docs/adr/");
    fs::write(
        repo.path().join(rel),
        "# 1. A decision\n\n## Status\n\nAccepted\n\n## Context\n\nForces.\n\n\
## Decision\n\nWe chose.\n\n## Consequences\n\nTradeoffs.\n",
    )
    .expect("write foreign adr");
    git(repo.path(), &["add", rel]);
    git(repo.path(), &["commit", "-q", "-m", "track foreign adr"]);

    ok_stdout(
        run_jigc(repo.path(), home.path(), Some(&pack), &["setup"], None),
        "jigc setup",
    );
    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            Some(&pack),
            &["migrate", rel, "--as", "adr"],
            None,
        ),
        "jigc migrate --as adr",
    );

    let skeleton = extract_author_skeleton(&composed, "adr");
    let payload = fill_payload(
        &skeleton,
        &[
            (
                "\"<the decision, as a short noun phrase>\"",
                "Fidelity Probe Decision",
            ),
            ("\"<proposed | accepted | superseded>\"", "accepted"),
        ],
    );

    let task = engine::slug::slugify("migrate-adr-docs-adr-0001-fidelity");
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            Some(&pack),
            &["doc", "author", "adr", "--from-file", "-", "--task", &task],
            Some(payload.as_bytes()),
        ),
        "jigc doc author adr",
    );

    let approve = run_jigc(
        repo.path(),
        home.path(),
        Some(&pack),
        &["task", "finalize", &task, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "finalize --approve on the filled adr migration must land; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );

    let committed = fs::read_to_string(
        repo.path()
            .join("docs")
            .join("decisions")
            .join("fidelity-probe-decision.md"),
    )
    .expect("the committed adr is on disk");
    assert!(
        committed.contains(BODY),
        "the committed adr slot prose must keep its paragraph break + per-bullet line \
         breaks verbatim (a folding flow scalar collapses them); committed:\n{committed}",
    );
}

#[test]
fn research_slot_prose_survives_multi_paragraph_and_bullets() {
    let repo = TempDir::new("research");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // Compose the methodology pack over the embedded dev base.
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");

    let rel = "old-notes.md";
    fs::write(
        repo.path().join(rel),
        "# Spreadsheet pain notes\n\nUsers re-import hourly.\n",
    )
    .expect("write foreign research note");
    git(repo.path(), &["add", rel]);
    git(repo.path(), &["commit", "-q", "-m", "track foreign notes"]);

    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            None,
            &["migrate", rel, "--as", "research"],
            None,
        ),
        "jigc migrate --as research",
    );

    let skeleton = extract_author_skeleton(&composed, "research");
    let payload = fill_payload(
        &skeleton,
        &[
            (
                "\"<the investigation, as a short noun phrase>\"",
                "Fidelity Probe Research",
            ),
            ("\"<historical-date>\"", "2020-01-01"),
        ],
    );

    let task = "migrate-research-old-notes";
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            None,
            &[
                "doc",
                "author",
                "research",
                "--from-file",
                "-",
                "--task",
                task,
            ],
            Some(payload.as_bytes()),
        ),
        "jigc doc author research",
    );

    let approve = run_jigc(
        repo.path(),
        home.path(),
        None,
        &["task", "finalize", task, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "finalize --approve on the filled research migration must land; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );

    let committed = fs::read_to_string(
        repo.path()
            .join("research")
            .join("fidelity-probe-research.md"),
    )
    .expect("the committed research doc is on disk");
    assert!(
        committed.contains(BODY),
        "the committed research slot prose must keep its paragraph break + per-bullet line \
         breaks verbatim (a folding flow scalar collapses them); committed:\n{committed}",
    );
}
