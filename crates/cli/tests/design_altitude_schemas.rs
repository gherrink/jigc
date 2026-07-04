//! M37 Increment 3 / T1 — the two ref-free design-altitude leaves (`research` +
//! `idea`) load and compose under the **`[dev ▸ methodology]`** composition, proven
//! through the real `jigc describe` binary (`design/design-altitude-doctypes.md`
//! §1/§2; `implementation/roadmap.md` → M37 Increment 3).
//!
//! The composition is the RC-trial's actual on-ramp: the on-disk methodology pack
//! listed in `packs.yaml` OVER the embedded dev base (the `multi_pack_acceptance.rs`
//! scaffolding — `init_repo`, `write_packs_yaml`, `methodology_pack_tree`, `run`).
//! Under it, `jigc describe` enumerates the *unfiltered* union doctype set and weaves
//! each doctype's authored `description:` / `usage:` into its facts-not-advice prose
//! ("`<type>` is `<description>`. Reach for it when `<usage>`."). So the observable
//! proof that both new schemas are **real and composable** is that describe:
//!   - exits 0 (a schema that failed to parse would `bail` out of `load_schemas`), AND
//!   - narrates BOTH `research` and `idea` with their authored prose verbatim.
//!
//! Both are ref-free one-per-doc leaves (no `type: ref` field), so T1 is self-
//! contained gate-green — no dangling type-ref, no `vision` (the `grounded-in → research`
//! ref lands in T2). Asserted on the EMITTED bytes of the real binary (hardening #4),
//! so a pack file that drops or garbles either doctype's prose fails here.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/../../packs/methodology`, the temp repo
//! is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-design-altitude-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the literal
/// directory a `.jigc/config/packs.yaml` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// (the setup gate the cascade + `describe` require).
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
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Record the methodology pack in the project layer's `packs.yaml` — listed highest,
/// over the implicit embedded dev base: the `[dev ▸ methodology]` composition.
fn write_packs_yaml(repo: &Path, listed: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

#[test]
fn describe_narrates_research_and_idea_under_dev_methodology() {
    let repo = TempDir::new("describe");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "`jigc describe` over the `[dev ▸ methodology]` composition must exit 0 — a schema \
         that failed to parse would bail; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // `research` — the one-per-doc append-only record. Its authored `description:` and
    // `usage:` are woven facts-not-advice ("`<type>` is `<description>`. Reach for it
    // when `<usage>`."). Asserting both clauses proves the schema loaded, composed
    // under the union, and carries the authored prose verbatim.
    assert!(
        stdout.contains(
            "research is One investigation and what it found — the evidence a vision or design is formed from."
        ),
        "describe must narrate the `research` doctype's authored description; got:\n{stdout}",
    );
    assert!(
        stdout.contains(
            "Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and that record should stay addressable by what it grounds."
        ),
        "describe must narrate the `research` doctype's authored usage; got:\n{stdout}",
    );

    // `idea` — the one-per-doc parked shaped direction (coexists with
    // `deferral-ledger.kind=I`). Same woven-prose proof.
    assert!(
        stdout.contains(
            "idea is One shaped-but-unscheduled direction, with the trigger that would bring it back."
        ),
        "describe must narrate the `idea` doctype's authored description; got:\n{stdout}",
    );
    assert!(
        stdout.contains(
            "Reach for it when a direction is worth keeping but not worth scheduling now, so it needs a durable home cheaper than losing the thought and a note of when to revisit it."
        ),
        "describe must narrate the `idea` doctype's authored usage; got:\n{stdout}",
    );
}
