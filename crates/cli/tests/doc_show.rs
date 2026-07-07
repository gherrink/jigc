//! M39 Increment 1 / T2 — `jigc doc show <ref>`, the committed-doc **read surface**,
//! proven end-to-end over the real `jigc` binary under the `[dev ▸ methodology]`
//! composition (`design/team-ready-state.md` → The read surface; `design/
//! introspection.md` → "Reading *filled* prose stays `jigc doc show`").
//!
//! The load-bearing contract this pins is the **`--format json` shape** — a 1.0 stable
//! contract, a one-way door pinned NOW by its golden: a whole-doc object
//! `{ type, slug, fields, sections }` where a slot section serializes to its prose
//! string and a repeatable section to its item array, a `#section` slice returns the
//! item array, and an `#section/<id>` slice the item object. The milestone-record (Inc 3)
//! is the contract's later witness; here `vision` (whole-doc, slot sections) + `prd`
//! (a repeatable `requirements` section) are the witnesses.
//!
//! Everything is asserted on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`) — the json goldens are matched verbatim, the block envelope of
//! a bad ref is driven through the real exit code + stderr. No external test crates.

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
            "jigc-doc-show-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// naming the methodology pack in `packs.yaml` (the `[dev ▸ methodology]` composition:
/// both the dev `prd` and the methodology `vision` doctypes resolve).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
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

/// The trimmed stdout of a successful invocation.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

/// Set one prose slot through the binary (stdin), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the auto-provisioned commit doc so a code-less task finalizes.
fn fill_commit(repo: &Path, home: &Path, task: &str, scope: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), scope);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"A managed doc.\n",
    );
}

/// Author + commit the `vision` singleton (no grounding — `grounded-in` is `0..*`), so
/// `VISION.md` is a committed whole-doc read target with three filled slot sections.
fn commit_vision(repo: &Path, home: &Path) {
    let task = "form-the-project-vision";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "form-vision",
                "form the project vision",
            ],
            None,
        ),
        "`jigc start --workflow form-vision`",
    );
    let create = jigc(
        repo,
        home,
        &[
            "doc", "create", "vision", "--title", "Vision", "--task", task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create vision`");
    assert_eq!(stdout_of(&create).trim(), "vision:vision");

    set_slot(
        repo,
        home,
        "vision:vision#thesis",
        b"A deterministic context compiler for coding agents.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#invariants",
        b"The CLI owns every structural write; the LLM writes only prose.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#open-questions",
        b"How far can one methodology pack compose.\n",
    );
    fill_commit(repo, home, task, "vision");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (form-vision) — the committed VISION.md",
    );
}

/// The `prd` batch-author payload: `vision`/`context` slot sections + a repeatable
/// `requirements` section carrying two items (each a `title` field + a `statement`
/// slot). The `<<…>>` markers tag the slot prose.
const PRD_PAYLOAD: &str = r#"title: "Habit tracker"
sections:
  - id: vision
    set:
      vision: "<<A tracker that turns intentions into daily streaks.>>"
  - id: requirements
    items:
      - title: "Log a habit in one tap"
        set:
          statement: "<<Logging a habit takes a single tap from the home screen.>>"
      - title: "Show the current streak"
        set:
          statement: "<<The current streak is shown front and center.>>"
  - id: context
    set:
      context: "<<Built for solo users who abandon heavyweight planners.>>"
"#;

/// Author + commit a `prd` with two `requirements` items, so `docs/prds/habit-tracker.md`
/// is a committed read target with a repeatable section.
fn commit_prd(repo: &Path, home: &Path) {
    let task = "build-a-habit-tracker";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "project-setup",
                "build a habit tracker",
            ],
            None,
        ),
        "`jigc start --workflow project-setup`",
    );
    let authored = jigc(
        repo,
        home,
        &["doc", "author", "prd", "--from-file", "-", "--task", task],
        Some(PRD_PAYLOAD.as_bytes()),
    );
    assert_ok(&authored, "`jigc doc author prd`");
    assert_eq!(stdout_of(&authored).trim(), "prd:habit-tracker");
    fill_commit(repo, home, task, "prd");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (project-setup) — the committed prd",
    );
}

// ---- the pinned `--format json` goldens (the 1.0 contract, byte-verbatim) ----

const VISION_JSON: &str = r#"{
  "fields": {},
  "sections": {
    "invariants": "The CLI owns every structural write; the LLM writes only prose.",
    "open-questions": "How far can one methodology pack compose.",
    "thesis": "A deterministic context compiler for coding agents."
  },
  "slug": "vision",
  "type": "vision"
}"#;

const PRD_JSON: &str = r#"{
  "fields": {
    "schema-version": "1"
  },
  "sections": {
    "context": "Built for solo users who abandon heavyweight planners.",
    "requirements": [
      {
        "statement": "Logging a habit takes a single tap from the home screen.",
        "title": "Log a habit in one tap"
      },
      {
        "statement": "The current streak is shown front and center.",
        "title": "Show the current streak"
      }
    ],
    "vision": "A tracker that turns intentions into daily streaks."
  },
  "slug": "habit-tracker",
  "type": "prd"
}"#;

const REQUIREMENTS_JSON: &str = r#"[
  {
    "statement": "Logging a habit takes a single tap from the home screen.",
    "title": "Log a habit in one tap"
  },
  {
    "statement": "The current streak is shown front and center.",
    "title": "Show the current streak"
  }
]"#;

const ONE_REQUIREMENT_JSON: &str = r#"{
  "statement": "Logging a habit takes a single tap from the home screen.",
  "title": "Log a habit in one tap"
}"#;

/// A committed `vision` round-trips through `jigc doc show` (plain carries every section)
/// and its `--format json` matches the pinned whole-doc golden; a committed `prd` shows
/// its whole-doc + repeatable-section slices under the pinned json shape; a bad ref exits
/// non-zero with the routed block envelope.
#[test]
fn doc_show_serves_the_committed_read_surface() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_vision(repo.path(), home.path());
    commit_prd(repo.path(), home.path());

    // (1) Plain whole-doc `vision` carries every section (the byte-exact committed view).
    let plain = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision"],
        None,
    );
    assert_ok(&plain, "`jigc doc show vision:vision`");
    let plain = stdout_of(&plain);
    assert!(
        plain.contains("# Vision"),
        "the display-title H1; got:\n{plain}"
    );
    for needle in [
        "A deterministic context compiler for coding agents.",
        "The CLI owns every structural write; the LLM writes only prose.",
        "How far can one methodology pack compose.",
    ] {
        assert!(
            plain.contains(needle),
            "plain vision must carry `{needle}`; got:\n{plain}"
        );
    }

    // (2) `vision --format json` matches the pinned whole-doc golden verbatim.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "vision:vision", "--format", "json"],
        None,
    );
    assert_ok(&json, "`jigc doc show vision:vision --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        VISION_JSON,
        "the vision whole-doc json is the pinned 1.0 shape",
    );

    // (3) `prd --format json` — the mixed shape: slot sections → prose, a repeatable
    //     section → its item array.
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "prd:habit-tracker", "--format", "json"],
        None,
    );
    assert_ok(&json, "`jigc doc show prd:habit-tracker --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        PRD_JSON,
        "the prd whole-doc json is the pinned 1.0 shape (slots + item array)",
    );

    // (4) A `#requirements` slice returns the item ARRAY under json.
    let json = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "prd:habit-tracker#requirements",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&json, "`jigc doc show prd:...#requirements --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        REQUIREMENTS_JSON,
        "a `#section` slice over a repeatable returns the item array",
    );

    // (5) A `#requirements/<id>` slice returns the item — plain (the rendered item) and
    //     json (the item object).
    let item_addr = "prd:habit-tracker#requirements/log-a-habit-in-one-tap";
    let plain = jigc(repo.path(), home.path(), &["doc", "show", item_addr], None);
    assert_ok(&plain, "`jigc doc show <item>`");
    let plain = stdout_of(&plain);
    assert!(
        plain.contains("Log a habit in one tap")
            && plain.contains("Logging a habit takes a single tap from the home screen."),
        "the plain item slice carries the item's title + statement; got:\n{plain}",
    );
    let json = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", item_addr, "--format", "json"],
        None,
    );
    assert_ok(&json, "`jigc doc show <item> --format json`");
    assert_eq!(
        stdout_of(&json).trim_end(),
        ONE_REQUIREMENT_JSON,
        "an `#section/<id>` slice returns the single item object",
    );

    // (6) A bad ref exits non-zero with the routed block envelope.
    let bad = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "prd:does-not-exist"],
        None,
    );
    assert!(
        !bad.status.success(),
        "a missing doc must exit non-zero; stdout:\n{}\nstderr:\n{}",
        stdout_of(&bad),
        String::from_utf8_lossy(&bad.stderr),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("route:") && stderr.contains("does-not-exist"),
        "the block envelope names the bad ref + its route; got:\n{stderr}",
    );
}
