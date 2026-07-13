//! M36 inc-2 / T2 — docs-root orphan detection, end-to-end through the built `jigc`
//! binary (no `JIGC_PACK_DIR`; the pack is the embedded dev pack).
//!
//! Two faces of the same coverage hole (`design/validation.md` → Orphan detection;
//! `design/storage.md` → docs-root):
//!
//! - **`jigc validate`** raises a store-scope `file-state.orphaned-doc` advisory (exit 0)
//!   for a committed managed doc stranded outside the resolved doctype roots after a
//!   `docs-root` re-point — while a `README.md`, an ordinary prose file under the
//!   docs-root parent, and a *live* doc at the *current* resolved root never flag (the
//!   bare-`.md`-match false-positive the location-basename predicate rules out).
//! - **`jigc config set docs-root`** *detect+routes+moves* (M39 inc-5 T3): when the re-point
//!   would strand committed docs under the prior resolved root, it relocates each to the new
//!   root (file-state re-keyed) and surfaces the move — the set still lands. (Auto-move is safe
//!   because the prior home is recorded — the old resolved root.)
//!
//! The advisory is **two-tier** since M40 (`design/validation.md` → M40 two-tier route):
//! emission never discriminates on `FileStateRecord` membership (a fresh clone still fires),
//! but the finding does — a **registered** path is a genuine strand and keeps the stranded →
//! `jigc unmanage`/relocate route (`file-state.orphaned-doc`); a **never-adopted** basename
//! coincidence (the adoption trial's `research/notes.md`) is the looks-managed-but-unregistered
//! advisory (`file-state.unregistered-doc`), routed migrate-or-ignore when the
//! `migrate-<doctype>` workflow ships, else ignore-or-human — never `jigc unmanage`, a proven
//! no-op loop on a never-registered doc.
//!
//! The `jigc` path comes from `CARGO_BIN_EXE_jigc`; the temp repo is a real `git init`;
//! the `doc-code` probe (the `validate` pre-flight requires it) is the real binary built
//! from the pack and selected via `JIGC_DOC_CODE_PROBE` (the `validate_command.rs` idiom).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-orphan-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the literal directory a
/// `.jigc/config/packs.yaml` entry names (the `flow_design_altitude.rs` harness shape).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path.
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
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

/// Run `jigc <args>` with `cwd = repo` and the real doc-code probe selected via
/// `JIGC_DOC_CODE_PROBE`, capturing output.
fn jigc(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// A conformant, hand-written `adr` body (no `cites-code`, so it needs no code file).
fn adr(title: &str) -> String {
    format!(
        "---\n\
         status: accepted\n\
         date: 2026-06-13\n\
         ---\n\
         \n\
         # {title}\n\
         \n\
         ## Context\n\
         Forces.\n\
         \n\
         ## Decision\n\
         Decided.\n\
         \n\
         ## Consequences\n\
         Effects.\n"
    )
}

/// Seed a real git repo with the `.jigc/config/` project layer and:
///  - a committed `adr` at the DEFAULT resolved root `docs/decisions/cache.md`;
///  - a committed `adr` at `archive/decisions/live.md` (the *future* resolved root);
///  - a `README.md` at the repo root + an ordinary prose file `docs/guide.md`.
///
/// After a re-point to `docs-root = archive`, `docs/decisions/cache.md` is stranded
/// (orphan), while `archive/decisions/live.md` sits at the current root and the two prose
/// files never look like managed docs.
fn seed(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    fs::write(repo.join("README.md"), "# The project\n").expect("write readme");
    fs::create_dir_all(repo.join("docs")).expect("mk docs");
    fs::write(repo.join("docs/guide.md"), "# A guide\n\nProse.\n").expect("write guide");

    fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
    fs::write(
        repo.join("docs/decisions/cache.md"),
        adr("The cache decision"),
    )
    .expect("write orphan adr");

    fs::create_dir_all(repo.join("archive/decisions")).expect("mk archive decisions");
    fs::write(
        repo.join("archive/decisions/live.md"),
        adr("The live decision"),
    )
    .expect("write live adr");

    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // The project layer — the locate-preamble's `require_project_layer` gate.
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// The orphaned-doc message lines from a rendered store report (the `file-state.orphaned-doc`
/// finding lines, excluding the trailing `  route:` continuation).
fn orphan_lines(stdout: &str) -> Vec<&str> {
    stdout
        .lines()
        .filter(|l| l.contains("file-state.orphaned-doc"))
        .collect()
}

/// `jigc validate` after a `docs-root` re-point: the committed doc left at the old root —
/// **registered** in the file-state record (it was adopted before the out-of-band re-point;
/// the M40 two-tier's genuine-strand arm) — surfaces `file-state.orphaned-doc` (exit 0)
/// with the existing stranded → `jigc unmanage`/relocate route, while README.md, ordinary
/// prose, and a live doc at the current root never flag.
#[test]
fn validate_flags_orphaned_doc_after_docs_root_repoint_only() {
    let repo = TempDir::new("validate");
    seed(repo.path());

    // Re-point docs-root docs/ → archive/ by writing the manifest **directly**, NOT via
    // `jigc config set docs-root` — which now detect+routes+MOVES the stranded doc (T3),
    // leaving no orphan for `validate` to find. A manifest re-pointed out-of-band (a human
    // edit, or a commit that landed on another machine before its docs were relocated) still
    // strands the committed `docs/decisions/cache.md` at the old root, which the store-scope
    // sweep must detect regardless of how the strand arose.
    fs::write(
        repo.path().join(".jigc/config/manifest.yaml"),
        "scalar:\n  docs-root: archive\n",
    )
    .expect("re-point docs-root via a direct manifest edit");

    // Two-tier discriminator (M40): the record carries the stranded doc's key, so this is
    // a REGISTERED strand — the tier that keeps the stranded route. (The never-adopted
    // sibling arm is `validate_routes_never_adopted_basename_coincidence_to_ignore_or_human`
    // below.) The hash value is irrelevant to the tier — membership is the discriminator.
    let state_dir = repo.path().join(".jigc").join("state");
    fs::create_dir_all(&state_dir).expect("mk state dir");
    fs::write(
        state_dir.join("file-state.json"),
        "{\n  \"hashes\": {\n    \"docs/decisions/cache.md\": \"deadbeef\"\n  }\n}\n",
    )
    .expect("seed the file-state record with the stranded doc's key");

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    // The orphan advisory is report-only and flips nothing. The exit here is non-zero for an
    // unrelated, honest reason: the hand-committed fixture doc is unstamped, so the corpus is
    // *unmigrated* — the M42 third exit-flipping exception (`design/validation.md` → Exit
    // semantics). Pinned explicitly so the flip is never mistaken for the advisory gating.
    assert!(
        stdout.contains("schema-conformance.schema-version-current"),
        "the v0-era fixture corpus is unmigrated — the reason the exit flips; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    let lines = orphan_lines(&stdout);
    assert!(
        lines.iter().any(|l| l.contains("docs/decisions/cache.md")),
        "the doc stranded at the old root must surface a file-state.orphaned-doc finding; \
         stdout:\n{stdout}",
    );
    assert!(
        !lines.iter().any(|l| l.contains("README.md")),
        "README.md (root, no location-named parent) must never flag as an orphan; \
         orphan lines:\n{lines:?}",
    );
    assert!(
        !lines.iter().any(|l| l.contains("docs/guide.md")),
        "ordinary prose under the docs-root parent must never flag as an orphan; \
         orphan lines:\n{lines:?}",
    );
    assert!(
        !lines
            .iter()
            .any(|l| l.contains("archive/decisions/live.md")),
        "a live doc under the CURRENT resolved root must never flag as an orphan; \
         orphan lines:\n{lines:?}",
    );

    // The registered tier keeps the stranded → unmanage/relocate route, and never the
    // unregistered sibling's code — a recorded path is a genuine strand, not a coincidence.
    assert!(
        stdout.contains("jigc unmanage"),
        "a REGISTERED strand keeps the stranded → `jigc unmanage`/relocate route; \
         stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("file-state.unregistered-doc"),
        "a file-state-recorded strand must not fire the unregistered tier; stdout:\n{stdout}",
    );
}

/// (M40 inc-3 T5; route flipped by M40 inc-7 T2) The two-tier route's **unregistered**
/// arm (`design/validation.md` → M40 two-tier route): a never-adopted committed file
/// under a doctype-basename dir — the adoption trial's `research/notes.md` shape, here
/// the methodology `research` doctype composed `[dev ▸ methodology]` via `packs.yaml` —
/// is a basename coincidence, not a tracked strand. It must still FIRE (emission is
/// never gated on file-state membership; a fresh clone always lands in this arm), but
/// as the looks-managed-but-unregistered advisory, and because `migrate-research` now
/// SHIPS (the M40 methodology migrate workflows — this very file was its motivating
/// example), the route names the real adoption verb `jigc migrate <path> --as research`
/// — never `jigc unmanage` (a proven no-op loop on a never-registered doc) and never
/// the ignore-or-human fallback that dead-ended here before the workflow existed.
/// Exit stays 0 (report-only, un-keyed).
#[test]
fn validate_routes_never_adopted_basename_coincidence_to_migrate_or_ignore() {
    let repo = TempDir::new("unregistered");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "# The project\n").expect("write readme");
    fs::create_dir_all(repo.path().join("old/research")).expect("mk old/research");
    fs::write(
        repo.path().join("old/research/notes.md"),
        "# Loose notes\n\nNever adopted through any jigc verb.\n",
    )
    .expect("write the never-adopted foreign file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    // The project layer + the [dev ▸ methodology] composition (the research doctype).
    fs::create_dir_all(repo.path().join(".jigc/config")).expect("create project layer");
    fs::write(
        repo.path().join(".jigc/config/packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
    // NO file-state record — the never-adopted arm (also the fresh-clone shape).

    let out = jigc(repo.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "`jigc validate` with only an unregistered advisory must exit 0 (report-only); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    assert!(
        stdout
            .lines()
            .any(|l| l.contains("file-state.unregistered-doc")
                && l.contains("old/research/notes.md")),
        "a never-adopted basename coincidence must fire the looks-managed-but-unregistered \
         advisory naming the path; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("`jigc migrate old/research/notes.md --as research`"),
        "with `migrate-research` shipped, the route must name the real adoption verb \
         on the flagged path; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("route it to a human"),
        "the ignore-or-human fallback is the workflow-less arm — it must not fire once \
         `migrate-research` ships; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("jigc unmanage"),
        "`unmanage` on a never-registered doc is a proven no-op loop — the stranded route \
         is wrong for this arm; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("file-state.orphaned-doc"),
        "the registered-strand code must not fire for a never-adopted file; stdout:\n{stdout}",
    );
}

/// (M39 inc-5 T3) `jigc config set docs-root <new>` over a repo with a committed doc at the
/// prior resolved root **detect+routes+moves** it — the surface-and-move resolution that
/// replaces the M36 warn-then-strand. Because a `docs-root` re-point has a **recorded prior
/// home** (the old resolved root, deterministic from the current cascade), auto-move is safe:
/// the stranded `docs/decisions/cache.md` is surfaced on stderr AND relocated to the new root
/// `archive/decisions/cache.md`, its file-state entry re-keyed (old forgotten / new recorded).
/// The set still lands (exit 0); a doc already at the new root is untouched (Prove #2).
#[test]
fn config_set_docs_root_surfaces_and_moves_the_stranded_doc() {
    let repo = TempDir::new("configset");
    seed(repo.path());

    // Seed a file-state record carrying the OLD path key, so the re-key's *forget-old* arm is
    // genuinely exercised (not only the record-new arm). The hash value is irrelevant — a pure
    // relocation preserves bytes and re-keys the path.
    let state_dir = repo.path().join(".jigc").join("state");
    fs::create_dir_all(&state_dir).expect("mk state dir");
    fs::write(
        state_dir.join("file-state.json"),
        "{\n  \"hashes\": {\n    \"docs/decisions/cache.md\": \"deadbeef\"\n  }\n}\n",
    )
    .expect("seed the file-state record with the old path key");

    let out = jigc(repo.path(), &["config", "set", "docs-root", "archive"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "the detect+route+move must not fail the write — `config set` still lands (exit 0); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // The stranded doc is physically relocated to the new resolved root; the old path is gone.
    assert!(
        !repo.path().join("docs/decisions/cache.md").exists(),
        "the stranded doc must be moved off the old root; stderr:\n{stderr}",
    );
    assert!(
        repo.path().join("archive/decisions/cache.md").exists(),
        "the stranded doc must land at the new resolved root; stderr:\n{stderr}",
    );

    // A doc already under the new root is not a strand and is untouched.
    assert!(
        repo.path().join("archive/decisions/live.md").exists(),
        "a doc already at the new root must not be disturbed by the re-point",
    );

    // File-state is re-keyed: the old key forgotten, the new key recorded.
    let record = fs::read_to_string(state_dir.join("file-state.json")).expect("read file-state");
    assert!(
        !record.contains("docs/decisions/cache.md"),
        "the old file-state key must be forgotten after the move; record:\n{record}",
    );
    assert!(
        record.contains("archive/decisions/cache.md"),
        "the new file-state key must be recorded after the move; record:\n{record}",
    );

    // The relocation is surfaced + routed — it names the old→new move it performed.
    assert!(
        stderr.contains("docs/decisions/cache.md") && stderr.contains("archive/decisions/cache.md"),
        "the surface must name the old→new relocation it performed; stderr:\n{stderr}",
    );
    // A doc already under the new root was not moved, so it is not named.
    assert!(
        !stderr.contains("archive/decisions/live.md"),
        "a doc already under the new root is not relocated by the re-point; stderr:\n{stderr}",
    );
}
