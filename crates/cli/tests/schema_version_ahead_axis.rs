//! **The above-current stamp is detected and blocked over the whole versioned-doctype
//! axis** — the confidence-audit sibling-hunt item 1 (class: freeze-stamp;
//! `completions/artifacts/M45/sibling-hunt.md`), fixed complete over its class's axis
//! per the M45 contract ([`implementation/dev-workflow.md`] → *a fix is complete over
//! its class's axis, never its repro*).
//!
//! The statement this suite pins: for **every** versioned doctype the composite
//! `[dev ▸ methodology]` registry ships, a committed instance stamped **above** its
//! doctype's manifest schema-version is
//!
//! - **reported** by `jigc validate` with one
//!   `schema-conformance.schema-version-ahead` break (exit non-zero — the doc was
//!   written to a schema this binary does not know, the same untrustworthy-sweep
//!   criterion as the below-version flip), and
//! - **blocked** by `jigc migrate-corpus` with one
//!   `migrate-corpus.schema-version-ahead` refusal — never silently reported
//!   `already-current` (the permanent migrate-skip the sibling-hunt found).
//!
//! **Enumeration comes from the registry, never a hand list**
//! ([`implementation/pinning.md`] §1): the doctypes are walked out of the
//! engine-loaded schemas of the composite pack-set — and since both shipped
//! manifests declare their doctype set **exactly equal** to their shipped schema set
//! (the pack-load freeze assert enforces it), the schema listing *is* the versioned
//! set. A doctype added later is swept by construction; one with no committed home
//! is named in [`HOMELESS`] with its reason and asserted against the computed set,
//! never silently dropped.
//!
//! [`implementation/dev-workflow.md`]: ../../../implementation/dev-workflow.md
//! [`implementation/pinning.md`]: ../../../implementation/pinning.md

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::Schema;
use std::collections::{BTreeMap, BTreeSet};
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
            "jigc-version-ahead-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path — the
/// real tree-sitter subprocess the `jigc validate` pre-flight resolves.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Make `root` a real git repo with identity, then run `jigc setup` over it.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The production composition, built the CWD-free way (`pinning.md` §1):
/// `[dev ▸ methodology]`, dev highest-precedence.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every doctype the composite registry ships, loaded through the **CLI** schema
/// loader against the doctype's own **origin** pack.
fn loaded_schemas(pack: &dyn PackSource) -> BTreeMap<String, Schema> {
    pack.list(PackResourceKind::Schemas)
        .iter()
        .map(|id| {
            let bytes = pack
                .read(PackResourceKind::Schemas, id)
                .unwrap_or_else(|e| panic!("read the `{id}` schema: {e}"));
            let origin = pack.origin_pack(PackResourceKind::Schemas, id);
            let schema = load_pack_schema(origin, &bytes)
                .unwrap_or_else(|e| panic!("load the `{id}` schema: {e}"));
            (schema.ty.clone(), schema)
        })
        .collect()
}

/// A stamp far above every shipped manifest version (all are ≤ 2 today).
const AHEAD_STAMP: u32 = 9999;

/// The slug every located planted instance is filed under.
const SLUG: &str = "ahead-probe";

/// Versioned doctypes with **no committed home** — the detectors can never meet an
/// instance of them, so they are excluded *by name and reason*, never by a filter a
/// reader has to trust.
const HOMELESS: &[(&str, &str)] = &[(
    "commit",
    "transient: its sink is the git commit message — no committed instance exists \
     for either detector to meet.",
)];

/// The repo-relative committed home a planted instance of `schema` lands at: a
/// placement doctype's literal `placement.file`, else the doctype's `location:`
/// under the default `docs-root` (`docs/` — the stock `jigc setup` cascade this
/// test's repo runs under). If home resolution ever drifts from this, the planted
/// file stops being a managed instance and the per-doctype assertion below goes
/// red — loud, never silently green.
fn home(schema: &Schema) -> Option<String> {
    if let Some(placement) = &schema.placement {
        return Some(placement.file.clone());
    }
    schema
        .location
        .as_ref()
        .map(|loc| format!("docs/{}/{SLUG}.md", loc.trim_end_matches('/')))
}

/// The planted instance: a real front-matter stamp above current, plus a minimal
/// body. Whether the body parses under the doctype's current schema is deliberately
/// irrelevant — the ahead break must fire on **both** the parse-success and the
/// parse-failure arm, so the sweep is arm-agnostic.
fn body(ty: &str) -> String {
    format!("---\nschema-version: {AHEAD_STAMP}\n---\n\n# Ahead probe {ty}\n\nProse.\n")
}

/// **The axis: every versioned doctype × the above-current stamp.**
///
/// One repo, one planted above-current instance per persisted versioned doctype
/// (registry-derived), one `jigc validate` and one `jigc migrate-corpus --dry-run`
/// over the lot — asserting the ahead break fires and the migrate skip does not,
/// per doctype.
#[test]
fn every_versioned_doctype_detects_and_blocks_an_above_current_stamp() {
    let pack = composite();
    let schemas = loaded_schemas(&pack);

    // Nothing is dropped silently: the homeless partition is stated with reasons.
    let homeless: BTreeSet<&str> = schemas
        .values()
        .filter(|s| s.location.is_none() && s.placement.is_none())
        .map(|s| s.ty.as_str())
        .collect();
    assert_eq!(
        homeless,
        HOMELESS.iter().map(|(t, _)| *t).collect::<BTreeSet<&str>>(),
        "every versioned doctype with no committed home is named in HOMELESS with its reason"
    );

    // doctype → its planted committed home (repo-relative), and the URI identity the
    // store sweep addresses its findings at.
    let planted: BTreeMap<&str, (String, String)> = schemas
        .values()
        .filter_map(|s| {
            home(s).map(|rel| {
                let identity = if s.placement.is_some() {
                    format!("{ty}:{ty}", ty = s.ty)
                } else {
                    format!("{ty}:{SLUG}", ty = s.ty)
                };
                (s.ty.as_str(), (rel, identity))
            })
        })
        .collect();
    assert_eq!(
        planted.len() + homeless.len(),
        schemas.len(),
        "the partition is total: every versioned doctype is planted or named homeless"
    );

    let repo = TempDir::new("axis");
    let home_dir = TempDir::new("home");
    setup_repo(repo.path(), home_dir.path());
    for (ty, (rel, _)) in &planted {
        let path = repo.path().join(rel);
        fs::create_dir_all(path.parent().expect("a home has a parent"))
            .expect("create the home dir");
        fs::write(&path, body(ty)).expect("plant the above-current instance");
    }
    git(repo.path(), &["add", "."]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "plant above-current stamped docs"],
    );

    // Arm 1 — `jigc validate`: one ahead break per planted doctype, exit non-zero.
    let out = jigc(
        repo.path(),
        home_dir.path(),
        &["validate", "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an above-current corpus flips `jigc validate`'s exit non-zero; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    let report: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("`jigc validate --format json` emits JSON");
    let ahead_addresses: Vec<&str> = report["findings"]
        .as_array()
        .expect("the report carries a findings array")
        .iter()
        .filter(|f| f["code"] == "schema-conformance.schema-version-ahead")
        .map(|f| {
            f["location"]["address"]
                .as_str()
                .expect("an ahead break is addressed at its doc")
        })
        .collect();
    assert_eq!(
        ahead_addresses.len(),
        planted.len(),
        "exactly one ahead break per planted doctype; stdout:\n{stdout}",
    );
    for (ty, (_, identity)) in &planted {
        assert!(
            ahead_addresses.contains(&identity.as_str()),
            "`{ty}` must surface an ahead break addressed at `{identity}`; \
             got {ahead_addresses:?}",
        );
    }

    // Arm 2 — `jigc migrate-corpus`: never a silent already-current; every planted doc
    // blocks with its own stable code, exit non-zero.
    let out = jigc(
        repo.path(),
        home_dir.path(),
        &["migrate-corpus", "--dry-run"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an above-current corpus blocks the migration (exit non-zero); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    for (ty, (rel, _)) in &planted {
        assert!(
            !stdout.contains(&format!("current    {rel}")),
            "`{ty}` at `{rel}` must NOT report `already current`; stdout:\n{stdout}",
        );
        assert!(
            stdout.contains(&format!("blocked    {rel}")),
            "`{ty}` at `{rel}` must be blocked; stdout:\n{stdout}",
        );
    }
    assert_eq!(
        stdout
            .matches("migrate-corpus.schema-version-ahead")
            .count(),
        planted.len(),
        "one ahead refusal per planted doctype; stdout:\n{stdout}",
    );
}
