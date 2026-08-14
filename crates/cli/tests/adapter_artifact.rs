//! The adapter's first **owned artifact** — the guides, installed by `jigc setup` as a
//! version-stamped file jigc owns and replaces (M48 Increment 10, T1;
//! `design/assistant-adapter.md` → Generated, minimal, regenerated / The adapter profile;
//! `DECISIONS.md` → 2026-08-13 the Settle, *the adapter's own instruction files*).
//!
//! VISION commits the adapter to *"a small set of skill/command files that each just call
//! the CLI"*, and the install commit carried none — an adopter had no version-matched path
//! to the guides at all (the pre-1.0.0 trial had to seed `QUICKSTART.md`/`MIGRATING.md` into
//! a sibling directory by hand). This suite drives the **real binary** over the mechanism,
//! which is the load-bearing half:
//!
//! (a) `setup` writes the **profile-declared** path, carrying a self-describing header —
//!     the `jigc-version:` stamp convention plus the `blake3` of its own body — and the
//!     recorded hash **equals** the body's;
//! (b) a second `setup` is byte-identical (the `.jigc/AGENT.md` mold: rewritten whole,
//!     no in-file idempotency markers);
//! (c) a **pristine** copy stamped at an older version is replaced and re-stamped — the
//!     *replaced on upgrade* half, observable through the stamp rather than assumed;
//! (d) the install commit **names** it, so what `setup` lists as installed is what its
//!     commit carries;
//! (e) a profile declaring **no** guide target installs nothing and exits 0 — the
//!     omitting-context arm (`implementation/increment-workflow.md` → hardening #5): the
//!     target is optional, and an assistant without one must be inert, never an error;
//! (f) the shipped bytes carry **no in-repo relative link** — every relative link in the
//!     source guides would dangle from an adopter's repo, which is the exact stranding the
//!     trial pre-registered as a finding.
//!
//! **T2 adds the other half of ownership** — jigc replaces what jigc wrote, and refuses to
//! clobber what the *user* wrote, at both doors the Settle pinned (`setup`, the writing
//! door; `jigc upgrade`, the reading one). Its arms live under their own banner below.
//!
//! No external test crates: the binary comes from `CARGO_BIN_EXE_jigc`, the temp repo is
//! built with `std::fs`, and a self-cleaning `TempDir` keeps this off the real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The Claude Code profile's declared guide target — the path this suite asserts against,
/// stated once here as the fixture's own premise (the profile is the authority; a change
/// there reddens this constant).
const GUIDE_PATH: &str = ".claude/skills/jigc/SKILL.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-guide-{tag}-{}-{:?}",
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

/// Make `root` a real git repo with a usable identity — `jigc setup` commits its own
/// install, so the fixture needs one.
fn mark_repo(root: &Path) {
    let ok = Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git init")
        .status
        .success();
    assert!(ok, "git init failed");
    for kv in [
        ["user.email", "test@example.com"],
        ["user.name", "Test"],
        ["commit.gpgsign", "false"],
    ] {
        let ok = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["config", kv[0], kv[1]])
            .output()
            .expect("run git config")
            .status
            .success();
        assert!(ok, "git config {} failed", kv[0]);
    }
}

/// Run `git -C <root> <args>` with the ambient config neutralized, returning stdout.
fn git_capture(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .unwrap_or_else(|err| panic!("run git {args:?}: {err}"));
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Run the built binary with `cwd = repo`, `$HOME = home`, and an optional
/// `JIGC_ADAPTERS_DIR` override (removed when `None`, so an ambient value never leaks in).
fn run_jigc(
    repo: &Path,
    home: &Path,
    adapters_dir: Option<&Path>,
    args: &[&str],
) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_jigc"));
    cmd.args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_ADAPTERS_DIR");
    if let Some(dir) = adapters_dir {
        cmd.env("JIGC_ADAPTERS_DIR", dir);
    }
    cmd.output().expect("run the jigc binary")
}

/// Run the built `jigc setup` — the shorthand every install arm uses.
fn run_setup(repo: &Path, home: &Path, adapters_dir: Option<&Path>) -> std::process::Output {
    run_jigc(repo, home, adapters_dir, &["setup"])
}

/// Assert `jigc setup` exited 0, printing stderr on failure.
fn assert_clean(out: &std::process::Output, label: &str) {
    assert!(
        out.status.success(),
        "{label}: `jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Split an installed artifact into (front-matter body, guide body) — the two halves the
/// stamp relates. Panics with the artifact's own bytes when the shape is wrong, so a
/// header regression reads as one failure rather than a slice panic.
fn split_artifact(text: &str) -> (&str, &str) {
    let rest = text.strip_prefix("---\n").unwrap_or_else(|| {
        panic!("the artifact must open with a `---` front matter; got:\n{text}")
    });
    let close = rest.find("\n---\n\n").unwrap_or_else(|| {
        panic!("the artifact's front matter must close with `---`; got:\n{text}")
    });
    (&rest[..close + 1], &rest[close + "\n---\n\n".len()..])
}

/// The value of a `key: value` line in the front matter.
fn header_value<'a>(front: &'a str, key: &str) -> &'a str {
    front
        .lines()
        .find_map(|line| line.strip_prefix(key))
        .map(str::trim)
        .unwrap_or_else(|| panic!("the front matter must carry `{key}`; got:\n{front}"))
}

/// (a) + (b): `setup` writes the profile-declared path with a self-describing header whose
/// recorded hash **is** its body's, and a second `setup` is byte-identical.
///
/// The hash is asserted against a hash this test computes itself, never against the
/// header's own claim about itself — a header that records some other file's digest, or a
/// generator that stamps before assembling, fails here.
#[test]
fn setup_writes_the_version_stamped_guide_artifact_idempotently() {
    let repo = TempDir::new("stamp");
    mark_repo(repo.path());
    let home = TempDir::new("stamp-home");

    assert_clean(&run_setup(repo.path(), home.path(), None), "first setup");

    let installed = fs::read_to_string(repo.path().join(GUIDE_PATH))
        .unwrap_or_else(|err| panic!("`setup` must write `{GUIDE_PATH}`: {err}"));
    let (front, body) = split_artifact(&installed);

    assert_eq!(
        header_value(front, "jigc-version:"),
        env!("CARGO_PKG_VERSION"),
        "the artifact must be stamped with the running build's version; front matter:\n{front}",
    );
    assert_eq!(
        header_value(front, "jigc-body-blake3:"),
        engine::file_state::hash_bytes(body.as_bytes()),
        "the recorded hash must equal the body's own; front matter:\n{front}",
    );
    assert!(
        !body.trim().is_empty(),
        "the artifact must carry the guides, not an empty body",
    );
    // The guides really are the content — both of them, one file (the deliberate
    // one-artifact bound: a second file is the condition the delta discipline is owed on).
    for marker in ["jigc start", "jigc task finalize", "jigc migrate-corpus"] {
        assert!(
            body.contains(marker),
            "the artifact's body must carry the shipped guides (missing `{marker}`); got:\n{body}",
        );
    }

    // (b) A second setup is byte-identical — rewritten whole, no drifting stamp.
    assert_clean(&run_setup(repo.path(), home.path(), None), "second setup");
    let again = fs::read_to_string(repo.path().join(GUIDE_PATH)).expect("still installed");
    assert_eq!(
        installed, again,
        "a second `jigc setup` must leave the artifact byte-identical",
    );
}

/// (c) The *replaced on upgrade* half, observable through the stamp: a **pristine** copy
/// stamped at an older version — its recorded hash matching its own body, so it is jigc's
/// own artifact and not a user's edit — is replaced and re-stamped at the running build.
#[test]
fn a_pristine_stale_stamped_guide_is_replaced_and_restamped() {
    let repo = TempDir::new("stale");
    mark_repo(repo.path());
    let home = TempDir::new("stale-home");

    // A pristine artifact from an older build: an old body, an old stamp, and a hash that
    // is genuinely this body's — the state a `jigc setup` after a binary upgrade meets.
    let stale_body = "the guides as jigc 0.0.1-old shipped them\n";
    let stale = format!(
        "---\nname: jigc\njigc-version: 0.0.1-old\njigc-body-blake3: {}\n---\n\n{stale_body}",
        engine::file_state::hash_bytes(stale_body.as_bytes()),
    );
    let target = repo.path().join(GUIDE_PATH);
    fs::create_dir_all(target.parent().expect("the artifact has a parent dir"))
        .expect("seed the skill dir");
    fs::write(&target, &stale).expect("seed the stale artifact");

    assert_clean(&run_setup(repo.path(), home.path(), None), "setup");

    let installed = fs::read_to_string(&target).expect("still installed");
    let (front, body) = split_artifact(&installed);
    assert_eq!(
        header_value(front, "jigc-version:"),
        env!("CARGO_PKG_VERSION"),
        "a pristine stale copy must be re-stamped at the running build; got:\n{front}",
    );
    assert_eq!(
        header_value(front, "jigc-body-blake3:"),
        engine::file_state::hash_bytes(body.as_bytes()),
        "the re-stamped hash must equal the replaced body's own; got:\n{front}",
    );
    assert!(
        !installed.contains(stale_body),
        "the stale body must be replaced, not kept; got:\n{installed}",
    );
}

/// (d) The install commit **names** the artifact — what `setup` reports as installed is
/// what its own commit carries, so a clone gets the guides that match the binary that
/// wrote them.
#[test]
fn the_install_commit_carries_the_guide_artifact() {
    let repo = TempDir::new("commit");
    mark_repo(repo.path());
    let home = TempDir::new("commit-home");
    // A born HEAD, so the install commit is the ordinary (non-first-commit) shape.
    fs::write(repo.path().join("README.md"), "hi\n").expect("seed README");
    git_capture(repo.path(), &["add", "README.md"]);
    git_capture(repo.path(), &["commit", "-q", "-m", "initial"]);

    let out = run_setup(repo.path(), home.path(), None);
    assert_clean(&out, "setup");

    let committed = git_capture(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|line| line == GUIDE_PATH),
        "the install commit must carry `{GUIDE_PATH}`; got:\n{committed}",
    );
    // Nothing of the install is left untracked behind the summary that lists it.
    let porcelain = git_capture(repo.path(), &["status", "--porcelain"]);
    assert!(
        !porcelain.contains(".claude/skills"),
        "the guide artifact must not be left untracked; `git status` says:\n{porcelain}",
    );
    // The summary names it on both surfaces the driver reads.
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains(GUIDE_PATH),
        "`jigc setup`'s summary must name the installed guide; got:\n{stdout}",
    );
}

/// (e) The omitting-context arm: a profile declaring **no** guide target installs nothing
/// at that path and exits 0. The target is optional by design — an assistant with no place
/// to put a guide must be *inert* here, never an install error — and a green pass over the
/// one composing profile would hide a required-field regression in every omitting one.
#[test]
fn a_profile_with_no_guide_target_installs_nothing_and_exits_zero() {
    let repo = TempDir::new("omit");
    mark_repo(repo.path());
    let home = TempDir::new("omit-home");

    // The shipped profile minus its `guide:` block, everything else byte-faithful — so
    // this fixture tracks the profile as it evolves rather than re-typing it.
    let shipped = include_str!("../adapters/claude-code.yaml");
    let (head, _) = shipped
        .split_once("\nguide:")
        .expect("the shipped profile declares a `guide:` block");
    let adapters = TempDir::new("omit-adapters");
    fs::write(
        adapters.path().join("claude-code.yaml"),
        format!("{head}\n"),
    )
    .expect("write the guide-less profile");

    let out = run_setup(repo.path(), home.path(), Some(adapters.path()));
    assert_clean(&out, "guide-less setup");
    assert!(
        !repo.path().join(GUIDE_PATH).exists(),
        "a profile declaring no guide target must install no guide",
    );
    assert!(
        !repo.path().join(".claude/skills").exists(),
        "a profile declaring no guide target must not create the skill tree either",
    );
    // The rest of the install is unaffected — inert means inert, not degraded.
    assert!(
        repo.path().join(".jigc/AGENT.md").exists() && repo.path().join("CLAUDE.md").exists(),
        "the guide-less install must still write the rest of the adapter",
    );
    let committed = git_capture(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        !committed.contains(".claude/skills"),
        "a guide-less install must not name a guide in its commit; got:\n{committed}",
    );
}

/// (f) The shipped bytes carry **no in-repo relative link**. Every relative link in the
/// source guides points at a file of the *jigc* repository — `design/storage.md`,
/// `implementation/roadmap.md`, the archived migration method — and dangles from an
/// adopter's tree, which is the stranding the pre-1.0.0 trial pre-registered as a finding.
/// Shipping them unresolved would author a law-1 defect in the same motion that fixes one.
///
/// The check is over the **installed bytes**, not over the generator: it is the file an
/// adopter opens that has to be link-clean.
#[test]
fn the_installed_guide_carries_no_in_repo_relative_link() {
    let repo = TempDir::new("links");
    mark_repo(repo.path());
    let home = TempDir::new("links-home");
    assert_clean(&run_setup(repo.path(), home.path(), None), "setup");

    let installed = fs::read_to_string(repo.path().join(GUIDE_PATH)).expect("installed");
    let mut rest = installed.as_str();
    let mut dangling: Vec<&str> = Vec::new();
    while let Some(at) = rest.find("](") {
        let tail = &rest[at + 2..];
        let end = tail.find(')').unwrap_or(tail.len());
        let target = &tail[..end];
        // Absolute URLs are fine from anywhere; anything else is a path into a repo the
        // adopter does not have.
        if !target.contains("://") && !target.starts_with('#') {
            dangling.push(target);
        }
        rest = &tail[end.min(tail.len())..];
    }
    assert!(
        dangling.is_empty(),
        "the shipped guide must carry no in-repo relative link; found: {dangling:?}",
    );
}

// ───────────────── T2: the artifact refuses to clobber a user-modified copy ─────────────────
//
// The ownership mechanism's second half. `setup` **replaces** what it wrote (arms (a)–(c)
// above); it must **not** replace what the user wrote — and it must say so rather than
// silently doing nothing, at **both** doors the settle pinned: the writing door (`setup`)
// and the reading door (`jigc upgrade`, a `VerbKind::Read` that reports and routes and
// never writes). Advisory + route, never blocking: blocking `setup` on a modified skill
// would be hostile, and an install that stops because one guide file was edited is worse
// than one that leaves it alone and names it.
//
// Recorded as a **declared deviation from principle #5** (every customization is a recorded
// delta against a known base version, never an untracked fork): refuse-to-clobber is an
// untracked-fork *detector* with no delta, and the delta discipline is owed when a second
// adapter-owned artifact appears (`implementation/decisions-pending.md` → No firm trigger yet).

/// The advisory both doors raise over a user-modified artifact. Stated once here as the
/// suite's premise; the producer is `cli::setup::guide_modified_finding`.
const MODIFIED_CODE: &str = "adapter-guide.user-modified";

/// Run `jigc upgrade` in `repo`.
fn run_upgrade(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut argv = vec!["upgrade"];
    argv.extend_from_slice(args);
    run_jigc(repo, home, None, &argv)
}

/// A command's stdout as a `String`.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Every file under `dir`, as `(repo-relative path, bytes)` sorted by path — the byte
/// snapshot the "writes nothing" arms compare before and after.
fn snapshot(dir: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(base: &Path, at: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        let Ok(entries) = fs::read_dir(at) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                let rel = path
                    .strip_prefix(base)
                    .expect("the walk stays under its base")
                    .display()
                    .to_string();
                out.push((rel, fs::read(&path).expect("read a snapshot file")));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

/// Install, then hand-edit the installed body so the recorded `jigc-body-blake3:` no longer
/// describes it — the shape a user who tweaked jigc's skill file leaves behind. Returns the
/// edited bytes.
fn install_then_edit(repo: &Path, home: &Path) -> String {
    assert_clean(&run_setup(repo, home, None), "first setup");
    let target = repo.join(GUIDE_PATH);
    let installed = fs::read_to_string(&target).expect("the first setup installs the guide");
    let edited = format!("{installed}\n## My own house rule\n\nAlways run the linter first.\n");
    fs::write(&target, &edited).expect("hand-edit the installed guide");
    edited
}

/// The parsed `--format json` document of a command's stdout.
fn envelope(out: &std::process::Output) -> serde_json::Value {
    let text = stdout_of(out);
    serde_json::from_str(&text)
        .unwrap_or_else(|err| panic!("`--format json` is one document ({err}); got:\n{text}"))
}

/// **The `setup` door.** A hand-edited artifact is left **byte-identical**, the install
/// still exits 0, and the agent surface carries the advisory *and its route* — the route
/// floor applies to an advisory the same as to a block, so a user who wants jigc's copy
/// back is told how. The same fact is keyed in `--format json`: the advisory rides
/// `findings[]`, and `guide_file` is `null`, because the summary's installed list must not
/// claim a file this run did not write (law 1 — the line it would print says "stamped with
/// this build", which of a user's copy is a lie).
#[test]
fn setup_refuses_to_clobber_a_user_modified_guide_and_routes_it() {
    let repo = TempDir::new("modified");
    mark_repo(repo.path());
    let home = TempDir::new("modified-home");
    let edited = install_then_edit(repo.path(), home.path());

    // (1) The agent surface.
    let out = run_setup(repo.path(), home.path(), None);
    assert_clean(&out, "setup over a user-modified guide");
    let stdout = stdout_of(&out);
    assert!(
        stdout.contains(&format!("advisory · {MODIFIED_CODE}")),
        "a user-modified guide must be reported as an advisory, never silently skipped; got:\n{stdout}",
    );
    assert!(
        stdout.contains(GUIDE_PATH),
        "the advisory must NAME the artifact jigc left alone; got:\n{stdout}",
    );
    assert!(
        stdout.contains("  route: "),
        "the advisory must carry a route — a detector with no way back is a dead end; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("  - jigc guides → "),
        "the installed list must not claim a guide this run did not write; got:\n{stdout}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join(GUIDE_PATH)).expect("still there"),
        edited,
        "`jigc setup` must leave a user-modified artifact byte-identical",
    );

    // (2) The same fact, keyed — a driver reads the advisory rather than grepping prose.
    let json = run_jigc(
        repo.path(),
        home.path(),
        None,
        &["setup", "--format", "json"],
    );
    assert_clean(&json, "setup --format json over a user-modified guide");
    let doc = envelope(&json);
    assert_eq!(
        doc.get("guide_file"),
        Some(&serde_json::Value::Null),
        "the envelope must not name a guide this run did not write; got:\n{doc:#}",
    );
    let findings = doc["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("`jigc setup`'s envelope carries `findings`:\n{doc:#}"));
    let advisory = findings
        .iter()
        .find(|f| f["code"] == MODIFIED_CODE)
        .unwrap_or_else(|| {
            panic!("the envelope withholds the `{MODIFIED_CODE}` advisory:\n{doc:#}")
        });
    assert_eq!(
        advisory["severity"], "advisory",
        "never blocking — blocking an install on an edited skill file would be hostile:\n{doc:#}",
    );
    assert!(
        advisory["route"].as_str().is_some_and(|r| !r.is_empty()),
        "the keyed advisory carries its route too:\n{doc:#}",
    );
    assert!(
        advisory["message"]
            .as_str()
            .is_some_and(|m| m.contains(GUIDE_PATH)),
        "the keyed advisory names the artifact:\n{doc:#}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join(GUIDE_PATH)).expect("still there"),
        edited,
        "the `--format json` run must leave the artifact byte-identical too",
    );
}

/// **The ownership question over its whole answer axis.** "Not jigc's" has three shapes,
/// and a check that only compares hashes would pass the first and clobber the other two:
/// a body edited under jigc's own header, a file with **no** front matter at all (a user's
/// own skill squatting at the path), and a front matter carrying every other key but not
/// jigc's digest. Each must survive `jigc setup` byte-identical, and each must be reported.
///
/// The pristine-stale arm above is this axis's fourth cell and the only one that *is*
/// jigc's — so the two tests together say exactly when the artifact is replaced.
#[test]
fn every_shape_of_a_not_jigc_artifact_survives_setup_byte_identical() {
    let seeded: [(&str, &str); 3] = [
        (
            "body-edited",
            // jigc's header shape, a digest that no longer describes the body.
            "---\nname: jigc\njigc-version: 9.9.9\njigc-body-blake3: 00\n---\n\nmy own words\n",
        ),
        ("no-front-matter", "# My jigc notes\n\nhand-written.\n"),
        (
            "no-stamp",
            "---\nname: jigc\ndescription: mine\n---\n\nhand-written.\n",
        ),
    ];

    for (tag, body) in seeded {
        let repo = TempDir::new(tag);
        mark_repo(repo.path());
        let home = TempDir::new(&format!("{tag}-home"));
        let target = repo.path().join(GUIDE_PATH);
        fs::create_dir_all(target.parent().expect("the artifact has a parent dir"))
            .expect("seed the skill dir");
        fs::write(&target, body).expect("seed the foreign artifact");

        let out = run_setup(repo.path(), home.path(), None);
        assert_clean(&out, &format!("setup over a {tag} artifact"));
        assert_eq!(
            fs::read_to_string(&target).expect("still there"),
            body,
            "{tag}: an artifact jigc cannot prove is its own must survive byte-identical",
        );
        let stdout = stdout_of(&out);
        assert!(
            stdout.contains(&format!("advisory · {MODIFIED_CODE}")),
            "{tag}: jigc must SAY it left the file alone; got:\n{stdout}",
        );
    }
}

/// **The `upgrade` door.** `jigc upgrade` is a `VerbKind::Read` and its module contract is
/// *"reads nothing it writes and writes nothing"* — so the replacement door is `setup`, and
/// upgrade's job here is to **report and route**. It names the artifact, exits 0 (an
/// advisory gates nothing), and leaves both the artifact and the whole `.jigc/config/`
/// project layer byte-identical.
#[test]
fn upgrade_reports_a_user_modified_guide_and_writes_nothing() {
    let repo = TempDir::new("upgrade-modified");
    mark_repo(repo.path());
    let home = TempDir::new("upgrade-modified-home");
    let edited = install_then_edit(repo.path(), home.path());
    let config_before = snapshot(&repo.path().join(".jigc").join("config"));
    assert!(
        !config_before.is_empty(),
        "the fixture's project layer must exist for the writes-nothing claim to mean anything",
    );

    let out = run_upgrade(repo.path(), home.path(), &[]);
    assert!(
        out.status.success(),
        "an advisory never gates: `jigc upgrade` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = stdout_of(&out);
    assert!(
        stdout.contains(&format!("advisory · {MODIFIED_CODE}")) && stdout.contains(GUIDE_PATH),
        "`jigc upgrade` must name the artifact it found modified; got:\n{stdout}",
    );
    assert!(
        stdout.contains("  route: "),
        "the advisory carries its route at this door too; got:\n{stdout}",
    );

    assert_eq!(
        fs::read_to_string(repo.path().join(GUIDE_PATH)).expect("still there"),
        edited,
        "`jigc upgrade` writes nothing — the artifact must be byte-identical",
    );
    assert_eq!(
        snapshot(&repo.path().join(".jigc").join("config")),
        config_before,
        "`jigc upgrade` is report-and-route only — the project layer must be byte-identical",
    );

    // The keyed half: the artifact upgrade checked, and the advisory as data.
    let doc = envelope(&run_upgrade(
        repo.path(),
        home.path(),
        &["--format", "json"],
    ));
    assert_eq!(
        doc.get("guide").and_then(serde_json::Value::as_str),
        Some(GUIDE_PATH),
        "the envelope names the artifact the sweep checked:\n{doc:#}",
    );
    assert!(
        doc["findings"]
            .as_array()
            .is_some_and(|f| f.iter().any(|f| f["code"] == MODIFIED_CODE)),
        "the advisory rides `findings[]`:\n{doc:#}",
    );
}

/// **The clean line names what was checked** (round-2 D4), and it widens with this fix: a
/// sweep that now also reads the adapter's owned artifact must say so, or "no findings"
/// understates its own scope. Over a pristine install the artifact is jigc's own, so the
/// clean line names it — and the envelope carries the same path beside `checked`.
#[test]
fn upgrade_over_a_pristine_guide_names_it_in_the_clean_line() {
    let repo = TempDir::new("upgrade-clean");
    mark_repo(repo.path());
    let home = TempDir::new("upgrade-clean-home");
    assert_clean(&run_setup(repo.path(), home.path(), None), "setup");

    let out = run_upgrade(repo.path(), home.path(), &[]);
    assert!(
        out.status.success(),
        "a clean sweep exits 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = stdout_of(&out);
    assert!(
        stdout.contains("no findings"),
        "a pristine artifact raises nothing; got:\n{stdout}",
    );
    assert!(
        stdout.contains(GUIDE_PATH),
        "the clean line must name the adapter artifact the sweep checked, or it understates \
         its own scope; got:\n{stdout}",
    );

    let doc = envelope(&run_upgrade(
        repo.path(),
        home.path(),
        &["--format", "json"],
    ));
    assert_eq!(
        doc.get("guide").and_then(serde_json::Value::as_str),
        Some(GUIDE_PATH),
        "the clean line's new fact is keyed too:\n{doc:#}",
    );
    assert!(
        doc.get("checked").is_some(),
        "the delta count the clean line has always named stays keyed:\n{doc:#}",
    );
}

/// **The omitting context** (`implementation/increment-workflow.md` → hardening #5). A repo
/// with a project layer but **no** installed artifact has nothing for the sweep to check,
/// and the clean line must not claim otherwise — a green pass over the composing context
/// alone would hide a check that reports on a file it never read.
#[test]
fn upgrade_with_no_installed_guide_claims_no_guide_check() {
    let repo = TempDir::new("upgrade-none");
    mark_repo(repo.path());
    let home = TempDir::new("upgrade-none-home");
    // A project layer by hand — the state before any `jigc setup` wrote an artifact.
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk .jigc/config");

    let out = run_upgrade(repo.path(), home.path(), &[]);
    assert!(
        out.status.success(),
        "an absent artifact is inert, never an error; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = stdout_of(&out);
    assert!(
        !stdout.contains(GUIDE_PATH) && !stdout.contains(MODIFIED_CODE),
        "with no artifact installed the sweep must claim no guide check; got:\n{stdout}",
    );

    let doc = envelope(&run_upgrade(
        repo.path(),
        home.path(),
        &["--format", "json"],
    ));
    assert_eq!(
        doc.get("guide"),
        Some(&serde_json::Value::Null),
        "the envelope says `null`, never a path it did not read:\n{doc:#}",
    );
}
