//! M49 Increment 2 / T1 — **`set:` becomes a closed vocabulary**, refused at schema
//! load (`implementation/roadmap.md` → M49 Increment 2, bullet 1;
//! `completions/artifacts/M49/settle-record.md` → T0-2).
//!
//! `Field::set` is a free `Option<String>` on the wire, and until now nothing ever
//! looked at its *value* on the load path. Three spellings are honored — `on-create`,
//! `on-transition`, `schema-version` — and a fourth was a silent no-op with teeth:
//!
//!   * `engine::validate::is_author_required` exempts a field on `set.is_some()`
//!     alone, so `set: on-creat` **permanently exempts the field from
//!     `required-field-present`** — the doctype author's stated obligation deleted by
//!     a typo, at exit 0;
//!   * the pinned `jigc doc schema --format json` contract
//!     serializes the typo back as a real deriver (`"set": "on-creat"`,
//!     `"author-required": false`), so a driver reading the contract is told the CLI
//!     fills a field nothing fills;
//!   * and the value sits **inside** every `schema-hash`, so the mistake freezes.
//!
//! Driven at `73b6bd6` before the fix: a `JIGC_PACK_DIR` dev-pack copy whose `adr`
//! `date` field read `set: on-creat` (hash re-pinned) ran `jigc doc schema adr
//! --format json` at **exit 0**, emitting `"set": "on-creat"` with
//! `"author-required": false`.
//!
//! The three arms below are the done-criterion of the task:
//!
//!  1. **The real binary refuses it.** A mutated filesystem pack whose extra
//!     manifest-listed doctype declares `set: on-creat` makes the invocation exit
//!     non-zero, naming the field, the offending value and the honored set — and its
//!     control twin (the same fixture spelled `on-create`) composes clean, so the
//!     refusal keys on the *value*, never on the fixture's presence.
//!  2. **The engine states it as a typed error.** `load_schema` returns
//!     `SchemaError::UnknownSetKind`, not a stringly-typed message.
//!  3. **Nothing was re-pinned quietly.** `set:` lives inside
//!     `engine::manifest::schema_hash` and crosses the pinned `doc schema` contract, so
//!     closing the vocabulary is a one-way door if it moves either: no entity in either
//!     manifest may have its `schema-hash` move against `HEAD` **while its co-located
//!     version stands still** (the shipped successor rule, stated in both manifest
//!     headers), and `jigc doc schema adr --format json` must be byte-identical to its
//!     `HEAD` capture. This arm is a **red-step obligation**, not a pin: a serialization
//!     that moves a hash behind no version is a manifest re-pin — a one-way door and a
//!     halt, never a quiet bump.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-set-kind-{tag}-{}-{:?}",
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

/// The crate root (`crates/cli`).
fn crate_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// The repository root — the home of both freeze manifests.
fn repo_root() -> PathBuf {
    crate_dir().join("..").join("..")
}

/// The embedded dev pack tree — the faithful source every on-disk copy mirrors.
fn dev_pack_tree() -> PathBuf {
    crate_dir().join("pack")
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// The fixture doctype's YAML, with `<SET>` standing in for the `set:` spelling under
/// test. An *extra* doctype rather than a mutated shipped one: the shipped set is
/// hash-frozen, and the point of arm 3 is that no shipped hash moves.
fn probe_schema(set: &str) -> String {
    format!(
        "type: probe-note\n\
         location: probe-notes/\n\
         id-from: title\n\
         description: A throwaway doctype minted by the set-kind vocabulary suite.\n\
         usage: never — this doctype exists only inside a mutated filesystem pack copy.\n\
         \n\
         sections:\n\
         \x20 - id: meta\n\
         \x20   header: true\n\
         \x20   fields:\n\
         \x20     - {{ id: noted, type: date, set: {set} }}\n\
         \x20 - id: body\n\
         \x20   slot: {{ hint: \"The note.\" }}\n"
    )
}

/// A dev-pack copy carrying the fixture doctype spelled `set: <set>`, listed in the
/// copy's own freeze manifest under `hash` (a dummy unless the caller pins it).
fn probe_pack(tag: &str, set: &str, hash: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&dev_pack_tree(), dir.path());
    fs::write(
        dir.path().join("schemas").join("probe-note.yaml"),
        probe_schema(set),
    )
    .expect("write the fixture schema");
    let manifest_path = dir.path().join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let entry = format!("  - type: probe-note\n    schema-version: 1\n    schema-hash: {hash}\n");
    fs::write(&manifest_path, format!("{manifest}{entry}"))
        .expect("write the manifest with the fixture entry");
    dir
}

/// A dummy `schema-hash` — a well-formed digest that matches nothing.
const DUMMY_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home` and an optional `JIGC_PACK_DIR`.
fn jigc(repo: &Path, home: &Path, pack: Option<&Path>, args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_jigc"));
    cmd.args(args).current_dir(repo).env("HOME", home);
    match pack {
        Some(pack) => cmd.env("JIGC_PACK_DIR", pack),
        None => cmd.env_remove("JIGC_PACK_DIR"),
    };
    cmd.output().expect("spawn the jigc binary")
}

/// **Arm 1 — the real binary refuses an unhonored `set:` spelling.** The emitted exit
/// code and stderr are the contract: non-zero, naming the field, the offending value
/// and the honored set, so the doctype author can fix the typo from the message alone.
#[test]
fn an_unhonored_set_kind_is_refused_at_pack_load() {
    let repo = TempDir::new("reject-repo");
    let home = TempDir::new("reject-home");
    let pack = probe_pack("reject", "on-creat", DUMMY_HASH);
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        Some(pack.path()),
        &["doc", "schema", "adr"],
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an unhonored `set:` value must make the invocation exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    for needle in [
        "noted",
        "on-creat",
        "on-create",
        "on-transition",
        "schema-version",
    ] {
        assert!(
            stderr.contains(needle),
            "the refusal must name `{needle}` (the field, the offending value, the honored set); got:\n{stderr}",
        );
    }
}

/// The control twin: the identical fixture spelled `on-create` composes clean, so
/// arm 1's refusal keys on the **value** and not on the fixture doctype's presence.
///
/// Its manifest hash is not a pinned constant — it is read back out of the freeze
/// gate's own mismatch message and re-pinned, so the fixture stays correct without
/// carrying a second copy of a hash the engine already computes.
#[test]
fn the_honored_spelling_of_the_same_fixture_composes_clean() {
    let repo = TempDir::new("control-repo");
    let home = TempDir::new("control-home");
    let pack = probe_pack("control", "on-create", DUMMY_HASH);
    init_repo(repo.path());

    // First pass: the dummy hash trips the freeze gate, which names the recomputed one.
    let out = jigc(
        repo.path(),
        home.path(),
        Some(pack.path()),
        &["doc", "schema", "adr"],
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        !out.status.success(),
        "the dummy-hash pass must trip the freeze gate; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    let recomputed = stderr
        .split("recomputed `")
        .nth(1)
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| {
            panic!("the freeze mismatch must name the recomputed hash; got:\n{stderr}")
        })
        .to_owned();
    let manifest_path = pack.path().join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the fixture manifest");
    fs::write(&manifest_path, manifest.replace(DUMMY_HASH, &recomputed))
        .expect("re-pin the fixture entry");

    // Second pass: a correctly pinned fixture carrying an honored `set:` loads clean.
    let out = jigc(
        repo.path(),
        home.path(),
        Some(pack.path()),
        &["doc", "schema", "probe-note", "--format", "json"],
    );
    assert!(
        out.status.success(),
        "the honored spelling must load clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"set\": \"on-create\""),
        "the honored spelling still projects as a deriver; got:\n{stdout}",
    );
}

/// **Arm 2 — the engine states it as a typed error**, at `load_schema`, so *every*
/// schema-loading door refuses (the pack, a versioned snapshot, a project shadow),
/// not only the pack-load sweep that arm 1 drives.
#[test]
fn load_schema_returns_a_typed_unknown_set_kind() {
    use engine::schema::{SchemaError, load_schema};

    let yaml = probe_schema("on-creat");
    let err = load_schema(yaml.as_bytes()).expect_err("an unhonored `set:` must not load");
    match &err {
        SchemaError::UnknownSetKind { field, set } => {
            assert_eq!(field, "noted", "the error names the offending field");
            assert_eq!(set, "on-creat", "the error names the offending value");
        }
        other => panic!("expected a typed UnknownSetKind, got: {other:?}"),
    }
    let rendered = err.to_string();
    for needle in [
        "noted",
        "on-creat",
        "on-create",
        "on-transition",
        "schema-version",
    ] {
        assert!(
            rendered.contains(needle),
            "the rendered error must name `{needle}`; got: {rendered}",
        );
    }

    // The honored three load, so the vocabulary is closed rather than narrowed.
    for honored in ["on-create", "on-transition", "schema-version"] {
        let yaml = probe_schema(honored);
        load_schema(yaml.as_bytes())
            .unwrap_or_else(|err| panic!("`set: {honored}` must load; got: {err}"));
    }
}

/// A manifest's pinned entities, keyed by entity name: `name -> (version, hash)`.
type Pins = std::collections::BTreeMap<String, (u32, String)>;

/// Every pinned entity of a manifest — each doctype under its own name, plus the
/// `slug-rule` — as `(version, hash)`. The **entity** is the unit the successor rule is
/// written over (`packs/methodology/config/schema-manifest.yaml` header: *a SCHEMA-SHAPE
/// change moves a doctype's `schema-hash` only together with its `schema-version`*), so
/// the comparison below is per entity rather than over a flat list of hash lines.
fn pinned_entities(body: &str, rel: &str) -> Pins {
    let manifest: engine::manifest::Manifest = serde_yaml_ng::from_str(body)
        .unwrap_or_else(|e| panic!("{rel} deserializes as a freeze manifest: {e}"));
    let mut out = Pins::new();
    if let Some(rule) = manifest.slug_rule {
        out.insert("slug-rule".to_string(), (rule.version, rule.hash));
    }
    for entry in manifest.doctypes {
        out.insert(entry.ty, (entry.schema_version, entry.schema_hash));
    }
    out
}

/// The pinned entities of `rel` as committed at `HEAD` and as they stand in the working
/// tree — the two sides of the no-quiet-re-pin comparison.
fn head_and_working(rel: &str) -> (Pins, Pins) {
    let out = Command::new("git")
        .args(["show", &format!("HEAD:{rel}")])
        .current_dir(repo_root())
        .output()
        .expect("run git show");
    assert!(
        out.status.success(),
        "git show HEAD:{rel} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let head = String::from_utf8(out.stdout).expect("utf-8 manifest");
    let working =
        fs::read_to_string(repo_root().join(rel)).expect("read the working-tree manifest");
    (pinned_entities(&head, rel), pinned_entities(&working, rel))
}

/// **Arm 3 — the red-step obligation: nothing was re-pinned *quietly*.** Closing the
/// vocabulary must leave `Field`'s serialization untouched, because `set:` sits inside
/// `engine::manifest::schema_hash` and inside the pinned `doc schema --format json`
/// contract. A hash that moves with **no version behind it** is a manifest re-pin — a
/// one-way door and a halt.
///
/// The predicate is the shipped **successor rule**, not `hashes are equal`: a hash moves
/// only together with its co-located version (both manifests' headers state it; M48
/// Increment 11 fences it in CI over the pushed range). The flat form this arm first
/// carried could not tell a silent re-pin from a **declared, versioned bump**, so it
/// reddened on the first legitimate one — M49 Increment 9's `completion-record` 1→2 —
/// where the honest verdict is *this moved, and it said so*. Increment 2's own claim is
/// unweakened: a serialization change moves a hash while every version stands still, and
/// that is exactly what still fails here.
#[test]
fn no_schema_hash_moved_and_the_pinned_projection_is_byte_identical() {
    for rel in [
        "crates/cli/pack/config/schema-manifest.yaml",
        "packs/methodology/config/schema-manifest.yaml",
    ] {
        let (head, working) = head_and_working(rel);
        assert!(!head.is_empty(), "{rel} must declare hashes at HEAD");
        for (entity, (head_version, head_hash)) in &head {
            let Some((version, hash)) = working.get(entity) else {
                continue; // an entity that left the manifest is that change's own subject
            };
            assert!(
                hash == head_hash || version != head_version,
                "`{entity}`'s `schema-hash` in {rel} moved against HEAD while its version \
                 stood still at {head_version} — that is a quiet manifest re-pin, a \
                 one-way door, and a halt: a shape change moves the hash only together \
                 with the version that declares it",
            );
        }
    }

    let repo = TempDir::new("projection-repo");
    let home = TempDir::new("projection-home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!(
            "packs:\n  - {}\n",
            repo_root().join("packs").join("methodology").display()
        ),
    )
    .expect("write packs.yaml naming the methodology pack");

    let out = jigc(
        repo.path(),
        home.path(),
        None,
        &["doc", "schema", "adr", "--format", "json"],
    );
    assert!(
        out.status.success(),
        "`doc schema adr --format json` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let captured = fs::read_to_string(
        crate_dir()
            .join("tests")
            .join("fixtures")
            .join("set-kind-vocabulary")
            .join("doc-schema-adr.json"),
    )
    .expect("read the HEAD capture");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        captured,
        "the pinned `doc schema adr --format json` projection moved against its HEAD \
         capture — closing the `set:` vocabulary must not touch what the contract emits",
    );
}
