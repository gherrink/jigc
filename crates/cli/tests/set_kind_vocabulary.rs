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

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::pack_locator::{self, Tree};

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
    Path::new(cli::pack_path!(dev)).to_path_buf()
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
type Pins = BTreeMap<String, (u32, String)>;

/// Every pinned entity of a manifest — each doctype under its own name, plus the
/// `slug-rule` — as `(version, hash)`. The **entity** is the unit the successor rule is
/// written over (the methodology manifest's header: *a SCHEMA-SHAPE
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

/// One pack's manifest compared across the two sides: where it was read at `HEAD` and
/// where it stands in the working tree. The two differ exactly when an uncommitted
/// change moved the pack.
#[derive(Debug, PartialEq, Eq)]
struct Compared {
    head: String,
    working: String,
}

/// An entity whose `schema-hash` moved against `HEAD` while its version stood still.
#[derive(Debug, PartialEq, Eq)]
struct QuietRepin {
    pack: String,
    entity: String,
    version: u32,
    head: String,
    working: String,
}

/// The no-quiet-re-pin comparison over one repo, every pack paired **by identity**.
#[derive(Debug, Default, PartialEq, Eq)]
struct RepinReport {
    /// Every pack located on both sides: `pack-id → where it was read`.
    compared: BTreeMap<String, Compared>,
    /// Packs located in the working tree only — nothing at `HEAD` to compare, stated.
    absent_at_head: Vec<String>,
    /// Packs located at `HEAD` only — the removal is that change's own subject, stated.
    absent_in_working: Vec<String>,
    /// The violations: the arm is green iff this is empty.
    repins: Vec<QuietRepin>,
}

/// Every pack's manifest in `tree`, by pack identity (M54 Increment 3, S16). A pack id
/// claimed by two manifests is a fence error, never a pick.
fn located(repo: &Path, tree: Tree<'_>) -> BTreeMap<String, String> {
    pack_locator::locate(repo, tree).unwrap_or_else(|duplicates| {
        panic!(
            "a pack id is claimed by more than one manifest in {tree:?} of {} — a fence \
             error, never a pick: {duplicates:?}",
            repo.display(),
        )
    })
}

/// The no-quiet-re-pin comparison: each pack's manifest as committed at `HEAD` against
/// the same pack's manifest as it stands in the working tree, **both located by pack
/// identity**. Neither path is hard-coded, so an uncommitted move — the manifest at A
/// in `HEAD` and at B on disk — compares A@HEAD with B@worktree instead of failing to
/// read one side (the pre-S16 `HEAD:<rel>` form).
fn quiet_repins(repo: &Path) -> RepinReport {
    let at_head = located(repo, Tree::Rev("HEAD"));
    let in_working = located(repo, Tree::Working);
    let mut report = RepinReport {
        absent_at_head: in_working
            .keys()
            .filter(|pack| !at_head.contains_key(*pack))
            .cloned()
            .collect(),
        ..RepinReport::default()
    };
    for (pack, head) in at_head {
        let Some(working) = in_working.get(&pack) else {
            report.absent_in_working.push(pack);
            continue;
        };
        let read = |tree: Tree<'_>, path: &str| {
            let body = pack_locator::read(repo, tree, path)
                .unwrap_or_else(|| panic!("{path} was located in {tree:?} and must read there"));
            pinned_entities(&body, path)
        };
        let head_pins = read(Tree::Rev("HEAD"), &head);
        let working_pins = read(Tree::Working, working);
        assert!(!head_pins.is_empty(), "{head} must declare hashes at HEAD");
        for (entity, (head_version, head_hash)) in &head_pins {
            let Some((version, hash)) = working_pins.get(entity) else {
                continue; // an entity that left the manifest is that change's own subject
            };
            if hash != head_hash && version == head_version {
                report.repins.push(QuietRepin {
                    pack: pack.clone(),
                    entity: entity.clone(),
                    version: *head_version,
                    head: head.clone(),
                    working: working.clone(),
                });
            }
        }
        report.compared.insert(
            pack,
            Compared {
                head,
                working: working.clone(),
            },
        );
    }
    report
}

/// The pack root a located manifest path sits under — the manifest's own path with
/// `config/schema-manifest.yaml` taken off, so a root is never named as a literal.
fn pack_root(manifest: &str) -> &str {
    manifest
        .strip_suffix(pack_locator::MANIFEST)
        .unwrap_or_else(|| panic!("{manifest} is a located pack manifest"))
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
///
/// **Both sides are located by pack identity** (M54 Increment 3, S16), never by a
/// literal path: the working tree says where each manifest is, `HEAD` says where it was,
/// so a gate run before a pack move is committed still compares every pack.
#[test]
fn no_schema_hash_moved_and_the_pinned_projection_is_byte_identical() {
    let report = quiet_repins(&repo_root());
    for pack in ["dev", "methodology"] {
        assert!(
            report.compared.contains_key(pack),
            "the `{pack}` pack's manifest must be located by identity both at HEAD and in \
             the working tree, or this arm compares nothing for it; got: {report:#?}",
        );
    }
    assert!(
        report.repins.is_empty(),
        "a `schema-hash` moved against HEAD while its version stood still — that is a \
         quiet manifest re-pin, a one-way door, and a halt: a shape change moves the hash \
         only together with the version that declares it; got: {report:#?}",
    );

    let repo = TempDir::new("projection-repo");
    let home = TempDir::new("projection-home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!(
            "packs:\n  - {}\n",
            repo_root()
                .join(pack_root(&report.compared["methodology"].working))
                .display()
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

/// Where the throwaway arm's pack sits at `HEAD`, and where the uncommitted move puts it
/// — the same move the M54 Increment 3 pack move makes.
const MOVED_FROM: &str = "crates/cli/pack";
const MOVED_TO: &str = "crates/cli/packs/dev";

/// A minimal freeze manifest pinning `adr` at `version` under `hash`.
fn adr_manifest(version: u32, hash: &str) -> String {
    format!("doctypes:\n  - type: adr\n    schema-version: {version}\n    schema-hash: {hash}\n")
}

/// Write a pack's manifest and the sibling `defaults.yaml` carrying its identity.
fn write_pack(repo: &Path, root: &str, pack: &str, manifest: &str) {
    let root = repo.join(root);
    fs::create_dir_all(root.join("config")).expect("create the pack's config dir");
    fs::write(root.join(pack_locator::MANIFEST), manifest).expect("write the manifest");
    fs::write(
        root.join(pack_locator::DEFAULTS),
        format!("pack-id: {pack}\n"),
    )
    .expect("write the pack's defaults");
}

/// A throwaway repo whose `HEAD` commits the `dev` pack's manifest at [`MOVED_FROM`]
/// and whose working tree carries it at [`MOVED_TO`] only — the move not committed,
/// which is what a gate run before the move's commit sees. `working` is the manifest
/// as it stands after the move.
fn moved_in_working_tree(tag: &str, working: &str) -> TempDir {
    let repo = TempDir::new(tag);
    init_repo(repo.path());
    write_pack(
        repo.path(),
        MOVED_FROM,
        "dev",
        &adr_manifest(2, "a".repeat(64).as_str()),
    );
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "the pack at its old home"]);
    fs::remove_dir_all(repo.path().join(MOVED_FROM)).expect("move the pack away");
    write_pack(repo.path(), MOVED_TO, "dev", working);
    repo
}

/// **S16 — the arm reads the working tree.** A manifest committed at A and moved to B
/// in the working tree only is compared A@HEAD against B@worktree. The pre-S16 form —
/// `git show HEAD:<rel>` beside a working-tree read of the same literal `rel` — could
/// read only one side of that move and failed whichever path it named.
#[test]
fn an_uncommitted_move_compares_the_old_home_at_head_with_the_new_home_on_disk() {
    let repo = moved_in_working_tree("moved-clean", &adr_manifest(2, "a".repeat(64).as_str()));
    // A second pack exists in the working tree only: nothing at HEAD to compare.
    write_pack(
        repo.path(),
        "packs/fresh",
        "fresh",
        &adr_manifest(1, &"c".repeat(64)),
    );

    assert_eq!(
        quiet_repins(repo.path()),
        RepinReport {
            compared: BTreeMap::from([(
                "dev".to_string(),
                Compared {
                    head: format!("{MOVED_FROM}/{}", pack_locator::MANIFEST),
                    working: format!("{MOVED_TO}/{}", pack_locator::MANIFEST),
                },
            )]),
            absent_at_head: vec!["fresh".to_string()],
            absent_in_working: Vec::new(),
            repins: Vec::new(),
        },
        "the moved pack is paired by identity across its two homes and is clean; the \
         pack new in the working tree is stated absent at HEAD",
    );
}

/// The move does not blind the arm: a hash re-pinned at an unchanged version, inside
/// the uncommitted move, is still flagged — and a hash moved together with its version
/// is still clean.
#[test]
fn a_hash_repinned_at_an_unchanged_version_across_an_uncommitted_move_is_flagged() {
    let repo = moved_in_working_tree("moved-repin", &adr_manifest(2, "b".repeat(64).as_str()));
    assert_eq!(
        quiet_repins(repo.path()).repins,
        vec![QuietRepin {
            pack: "dev".to_string(),
            entity: "adr".to_string(),
            version: 2,
            head: format!("{MOVED_FROM}/{}", pack_locator::MANIFEST),
            working: format!("{MOVED_TO}/{}", pack_locator::MANIFEST),
        }],
        "a quiet re-pin across a move must be flagged, naming both homes",
    );

    let declared = moved_in_working_tree("moved-bump", &adr_manifest(3, "b".repeat(64).as_str()));
    assert!(
        quiet_repins(declared.path()).repins.is_empty(),
        "a hash that moves together with its version is a declared bump, not a re-pin",
    );
}
