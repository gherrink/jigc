//! M38 Increment 6 / T1 — the **composite `[dev ▸ methodology]` placement-layout
//! acceptance** (worked-examples flow 39; `design/storage.md` → Placement). The one
//! genuinely-missing RC-trial-layout proof: a SINGLE throwaway repo, composed the way an
//! RC trial actually composes (the on-disk methodology pack listed in `packs.yaml` OVER
//! the embedded dev base — the `flow_form_vision.rs` harness shape, NOT a
//! methodology-alone `JIGC_PACK_DIR`), that finalizes a `vision` (a methodology placement
//! doctype homed at the repo-root literal `VISION.md`) AND authors+finalizes a `changelog`
//! (a dev placement doctype homed at the repo-root literal `CHANGELOG.md`) side by side,
//! proving the whole placement convention holds end-to-end at the composed surface.
//!
//! No existing test asserts BOTH root `VISION.md` and root `CHANGELOG.md` in ONE repo —
//! the changelog-relocation + foreign-root-adoption arms are proven in flow25/flow26 and
//! the managed-at-root vision in flow_form_vision/flow_design_altitude; this file is the
//! composite that puts the two placement doctypes in the same tree, adds a `docs/*.md`
//! methodology singleton (`roadmap`), and pins the sibling root non-doctype files
//! (`README.md`, `CLAUDE.md`) as unmanaged.
//!
//! The RC-trial layout the one repo proves (every fact on the EMITTED bytes / census of
//! the real `jigc` binary over `[dev ▸ methodology]`):
//!   - the `vision` singleton lands at the literal root `VISION.md` (H1 `# Vision`), NOT a
//!     `docs/vision/vision.md` one-file-folder;
//!   - the `changelog` singleton lands at the literal root `CHANGELOG.md` (H1
//!     `# Changelog`), NOT a `docs/changelog/changelog.md` one-file-folder;
//!   - a methodology singleton (`roadmap`) lands at `docs/roadmap.md` (the `docs/*.md`
//!     placement layer);
//!   - `jigc ingest` (the census surface) reports all three managed docs `adopted` and the
//!     sibling root `README.md` / `CLAUDE.md` `unmanaged` (a literal placement home owns
//!     one path — a sibling root `.md` is not swept in);
//!   - an out-of-band nonconformant edit to EACH root home is detected + routed
//!     (`needs-reconcile`) by the same census, while the unmanaged siblings stay untouched.
//!
//! No external test crates.

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
            "jigc-flow39-placement-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the literal directory a
/// `.jigc/config/packs.yaml` entry names.
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer, then
/// record the methodology pack in `packs.yaml` (listed over the embedded dev base: the
/// `[dev ▸ methodology]` composition). Seeds two sibling root non-doctype files
/// (`README.md`, `CLAUDE.md`) that must stay unmanaged.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "# Readme\n\nhello\n").expect("write README.md");
    fs::write(repo.join("CLAUDE.md"), "# Claude\n\nagent rules\n").expect("write CLAUDE.md");
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

/// The trimmed stdout of a successful `jigc` invocation.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert_ok(&out, what);
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim()
        .to_string()
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
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

/// Set one header field through the binary, asserting success.
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

/// Fill the provisioned commit doc's four levers so a finalize validates clean — robust to
/// whichever `commit` doctype wins under the composition (`scope`/`body` are optional; both
/// accept the value).
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
        b"A placement-layout record.\n",
    );
}

/// The committed bytes of `path` at HEAD (panics if the path is not committed).
fn committed(repo: &Path, path: &str) -> String {
    let out = Command::new("git")
        .args(["show", &format!("HEAD:{path}")])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        out.status.success(),
        "{path} must be committed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 committed bytes")
}

/// Whether `path` exists at HEAD (a one-file-folder mirror must NOT).
fn committed_exists(repo: &Path, path: &str) -> bool {
    Command::new("git")
        .args(["show", &format!("HEAD:{path}")])
        .current_dir(repo)
        .output()
        .expect("git show")
        .status
        .success()
}

/// The `HEAD` commit count.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// The `jigc ingest` census row (one line) naming `path`, or a panic listing the report.
fn ingest_row(repo: &Path, home: &Path, path: &str) -> String {
    let report = ok_stdout(jigc(repo, home, &["ingest"], None), "jigc ingest");
    report
        .lines()
        .find(|l| l.contains(path))
        .unwrap_or_else(|| panic!("the census must report a row for `{path}`; got:\n{report}"))
        .to_string()
}

/// Finalize the `form-vision` flow so the `vision` singleton is managed at the repo-root
/// literal `VISION.md`. Returns nothing — the committed home is asserted by the caller.
fn finalize_vision(repo: &Path, home: &Path) {
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
    let task = "form-the-project-vision";
    let addr = ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc", "create", "vision", "--title", "Vision", "--task", task,
            ],
            None,
        ),
        "`jigc doc create vision`",
    );
    assert_eq!(
        addr, "vision:vision",
        "the singleton mints at the fixed slug"
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#thesis"),
        b"A context compiler for coding agents.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#invariants"),
        b"The CLI owns structure; the LLM owns prose.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#open-questions"),
        b"When does a public pack platform earn its keep?\n",
    );
    fill_commit(repo, home, task, "vision");
    let before = head_count(repo);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (form-vision)",
    );
    assert_eq!(
        head_count(repo),
        before + 1,
        "form-vision finalize lands exactly ONE commit"
    );
}

/// Author + finalize the `record-change` flow so the `changelog` singleton is managed at
/// the repo-root literal `CHANGELOG.md`.
fn finalize_changelog(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "record-change",
                "cut the first release",
            ],
            None,
        ),
        "`jigc start --workflow record-change`",
    );
    let task = "cut-the-first-release";
    let created = ok_stdout(
        jigc(
            repo,
            home,
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "`jigc doc create changelog`",
    );
    assert_eq!(
        created, "changelog:changelog",
        "the changelog singleton cold-mints at the fixed slug",
    );
    // The version-title slugger drops dots; drive the EMITTED address verbatim downstream.
    let release = ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.0.0",
            ],
            None,
        ),
        "`jigc doc add-item` release 1.0.0",
    );
    let group = ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "added",
            ],
            None,
        ),
        "`jigc doc add-item` change-group added",
    );
    set_slot(
        repo,
        home,
        &format!("{group}/notes"),
        b"- OAuth device-code flow\n",
    );
    fill_commit(repo, home, task, "changelog");
    let before = head_count(repo);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (record-change)",
    );
    assert_eq!(
        head_count(repo),
        before + 1,
        "record-change finalize lands exactly ONE commit"
    );
}

/// Finalize the `planning` flow's `roadmap` singleton so a methodology singleton is managed
/// at `docs/roadmap.md` (the `docs/*.md` placement layer).
fn finalize_roadmap(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "planning",
                "plan the first milestone",
            ],
            None,
        ),
        "`jigc start --workflow planning`",
    );
    let task = "plan-the-first-milestone";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc", "create", "roadmap", "--title", "Roadmap", "--task", task,
            ],
            None,
        ),
        "`jigc doc create roadmap`",
    );
    let item = ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc",
                "add-item",
                "roadmap:roadmap#milestones",
                "--title",
                "M-One",
                "--task",
                task,
            ],
            None,
        ),
        "`jigc doc add-item` milestone M-One",
    );
    set_slot(
        repo,
        home,
        &format!("{item}/proves"),
        b"M-One proves the docs/*.md placement layer.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{item}/decomposition"),
        b"Inc 1: the first increment, as prose.\n",
    );
    fill_commit(repo, home, task, "planning");
    let before = head_count(repo);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (planning)",
    );
    assert_eq!(
        head_count(repo),
        before + 1,
        "planning finalize lands exactly ONE commit"
    );
}

/// The composite RC-trial-layout proof: root `VISION.md` + root `CHANGELOG.md` + a
/// `docs/*.md` methodology singleton coexist in ONE `[dev ▸ methodology]` repo, the census
/// reports the three managed and the two sibling root files unmanaged, and an OOB
/// nonconformant edit to each root home is detected + routed.
#[test]
fn rc_trial_layout_places_vision_and_changelog_at_root() {
    let repo = TempDir::new("layout");
    let home = TempDir::new("home");
    let repo = repo.path();
    let home = home.path();
    init_repo(repo);

    // ── Land the three managed docs, each in its own finalize. ──
    finalize_vision(repo, home);
    finalize_changelog(repo, home);
    finalize_roadmap(repo, home);

    // ── The vision is managed at the literal root `VISION.md` (H1 `# Vision`), NOT a
    //    one-file-folder mirror. ──
    let vision = committed(repo, "VISION.md");
    assert!(
        vision.lines().any(|l| l.trim() == "# Vision"),
        "the managed vision's H1 reads `# Vision` (display-title knob); got:\n{vision}",
    );
    assert!(
        vision.contains("A context compiler for coding agents."),
        "the promoted vision carries the authored thesis; got:\n{vision}",
    );
    assert!(
        !committed_exists(repo, "vision/vision.md")
            && !committed_exists(repo, "docs/vision/vision.md"),
        "the vision lives ONLY at the literal `VISION.md` — no `docs/vision/vision.md` one-file-folder",
    );

    // ── The changelog is managed at the literal root `CHANGELOG.md` (H1 `# Changelog`),
    //    NOT a one-file-folder mirror. ──
    let changelog = committed(repo, "CHANGELOG.md");
    assert!(
        changelog.lines().any(|l| l.trim() == "# Changelog"),
        "the managed changelog's H1 reads `# Changelog` (display-title knob); got:\n{changelog}",
    );
    assert!(
        changelog.contains("OAuth device-code flow"),
        "the promoted changelog carries the authored release note; got:\n{changelog}",
    );
    assert!(
        !committed_exists(repo, "changelog/changelog.md")
            && !committed_exists(repo, "docs/changelog/changelog.md"),
        "the changelog lives ONLY at the literal `CHANGELOG.md` — no `docs/changelog/changelog.md` one-file-folder",
    );

    // ── The methodology singleton lands at `docs/roadmap.md` (the `docs/*.md` layer). ──
    let roadmap = committed(repo, "docs/roadmap.md");
    assert!(
        roadmap.contains("M-One proves the docs/*.md placement layer."),
        "the roadmap singleton is managed at `docs/roadmap.md`; got:\n{roadmap}",
    );
    assert!(
        !committed_exists(repo, "docs/roadmap/roadmap.md")
            && !committed_exists(repo, "roadmap/roadmap.md"),
        "the roadmap lives ONLY at the literal `docs/roadmap.md` — no one-file-folder mirror",
    );

    // ── Census: the three managed docs are `adopted`; the sibling root files unmanaged. ──
    for (path, kind) in [
        ("VISION.md", "root vision"),
        ("CHANGELOG.md", "root changelog"),
        ("docs/roadmap.md", "docs/ roadmap"),
    ] {
        let row = ingest_row(repo, home, path);
        assert!(
            row.contains("adopted")
                && !row.contains("unmanaged")
                && !row.contains("needs-reconcile"),
            "the {kind} managed doc `{path}` ingests as adopted (not unmanaged / needs-reconcile); row:\n{row}",
        );
    }
    // The sibling root files stay unmanaged — collapsed into the root `./` per-directory
    // count (V9); never itemized as an adopted / needs-reconcile (managed) row.
    let census = ok_stdout(jigc(repo, home, &["ingest"], None), "jigc ingest");
    assert!(
        census.contains("unmanaged ./ —"),
        "the sibling root files collapse into an unmanaged `./` count (a literal placement home owns one path, not a dir-glob); got:\n{census}",
    );
    for path in ["README.md", "CLAUDE.md"] {
        assert!(
            !census.contains(path),
            "the sibling root `{path}` stays unmanaged (collapsed, never swept into management); got:\n{census}",
        );
    }

    // ── OOB detect + route: a nonconformant human edit to EACH root home (drop a required
    //    section heading) is detected + routed `needs-reconcile` by the same census. ──
    let vision_drift = vision.replace("## Thesis", "## Thesisz");
    assert_ne!(
        vision_drift, vision,
        "the OOB vision edit must actually change a heading"
    );
    fs::write(repo.join("VISION.md"), &vision_drift).expect("write OOB-edited VISION.md");

    let changelog_drift = changelog.replace("## Releases", "## Releasesz");
    assert_ne!(
        changelog_drift, changelog,
        "the OOB changelog edit must actually change a heading"
    );
    fs::write(repo.join("CHANGELOG.md"), &changelog_drift).expect("write OOB-edited CHANGELOG.md");

    git(repo, &["add", "VISION.md", "CHANGELOG.md"]);
    git(
        repo,
        &[
            "commit",
            "-q",
            "-m",
            "human edits the root docs out of band",
        ],
    );

    for (path, kind) in [
        ("VISION.md", "root vision"),
        ("CHANGELOG.md", "root changelog"),
    ] {
        let row = ingest_row(repo, home, path);
        assert!(
            row.contains("needs-reconcile"),
            "the OOB nonconformant edit to the managed {kind} `{path}` is detected + routed \
             (needs-reconcile); row:\n{row}",
        );
    }
    // The unmanaged siblings are still unmanaged (a drifted managed doc's re-route does not
    // sweep an unrelated root `.md` into management) — still collapsed into the `./` count.
    let census = ok_stdout(jigc(repo, home, &["ingest"], None), "jigc ingest");
    assert!(
        census.contains("unmanaged ./ —"),
        "the sibling root files stay collapsed into an unmanaged `./` count after the OOB drift; got:\n{census}",
    );
    for path in ["README.md", "CLAUDE.md"] {
        assert!(
            !census.contains(path),
            "the sibling root `{path}` stays unmanaged after the OOB drift; got:\n{census}",
        );
    }
}
