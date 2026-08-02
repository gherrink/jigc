//! M42 Increment 9 / T6 — the **sixth target form**: the cascade delta target, through the
//! real binary (`design/command-output-contract.md` → The sixth form — the cascade delta
//! target).
//!
//! Every `override-default.*` constructor *computes* the delta's identity string, spends it
//! on the message + the route, then calls `Finding::block`, which hard-sets `location: None`
//! — so all five projected `(code, null)`. That is the **only** cluster of the M42 sweep
//! whose collision lives **inside a single JSON document**: `jigc upgrade --format json`
//! classifies every recorded delta in one call, so two orphaned scalar-sets rode one findings
//! array with byte-identical keys and a driver deserializing it could not tell them apart.
//!
//! The arms drive the emitted envelope, never a reconstruction: `jigc upgrade --format json`
//! is run against a seeded project layer + a seeded current pack, and the assertions read the
//! `key` the emitted JSON carries.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-upgrade-finding-keys-{tag}-{}-{:?}",
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

/// Seed a repo: a `.git` marker (repo-root discovery walks to it) and a `.jigc/config/`
/// project layer carrying `manifest_text` — the recorded deltas the classifier reconciles.
fn seed_repo(root: &Path, manifest_text: &str) {
    fs::create_dir_all(root.join(".git")).expect("mk .git");
    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk .jigc/config");
    fs::write(config.join("manifest.yaml"), manifest_text).expect("write manifest");
}

/// Seed the `JIGC_PACK_DIR` pack the recorded deltas are classified against: the given
/// `(stem, body)` steps plus a `config/knobs.yaml` declaring only `default-workflow` (+ the
/// intrinsic floored surface), so any *other* recorded scalar-set key orphans.
fn seed_pack(dir: &TempDir, steps: &[(&str, &str)]) -> PathBuf {
    let pack_root = dir.path().join("pack");
    let steps_dir = pack_root.join("steps");
    fs::create_dir_all(&steps_dir).expect("mk pack/steps");
    for (stem, body) in steps {
        fs::write(steps_dir.join(format!("{stem}.yaml")), body).expect("seed step");
    }
    let config_dir = pack_root.join("config");
    fs::create_dir_all(&config_dir).expect("mk pack/config");
    let intrinsic: String = engine::knobs::INTRINSIC_CHECK_KEYS
        .iter()
        .map(|k| {
            format!(
                "{k}:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n  floor: blocking\n"
            )
        })
        .collect();
    fs::write(
        config_dir.join("knobs.yaml"),
        format!("default-workflow:\n  type: string\n  default: single-task\n{intrinsic}"),
    )
    .expect("seed knobs.yaml");
    fs::write(config_dir.join("defaults.yaml"), "pack-id: test-pack\n")
        .expect("seed defaults.yaml");
    pack_root
}

/// Run `jigc upgrade --format json` and return the findings array the emitted envelope
/// carries (the report blocks, so the run exits non-zero).
fn upgrade_findings(repo: &Path, home: &Path, pack_dir: &Path) -> Vec<serde_json::Value> {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["upgrade", "--format", "json"])
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        !out.status.success(),
        "a blocking-classifying delta must exit non-zero; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`jigc upgrade --format json` emits JSON ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The `key`s of every finding carrying `code`, in emission order.
fn keys_of(findings: &[serde_json::Value], code: &str) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"] == code)
        .map(|f| f["key"].clone())
        .collect()
}

/// **The single-document collision.** TWO orphaned scalar-sets classify in one `jigc upgrade`
/// call and ride ONE findings array; at HEAD both keys were `{"code": …, "target": null}` —
/// byte-identical inside one JSON document. Each now keys at its own `scalar:<knob-key>`.
#[test]
fn two_orphaned_scalar_sets_in_one_json_array_carry_distinct_key_targets() {
    let repo = TempDir::new("scalars");
    let home = TempDir::new("home");
    // The pack's closed knob surface declares `default-workflow` only, so both recorded keys
    // orphan (the pack dropped them).
    let pack_dir = seed_pack(&repo, &[("implement", "implement body\n")]);
    seed_repo(
        repo.path(),
        "\
scalar:
  legacy-knob: yes
  retired-knob: no
",
    );

    let findings = upgrade_findings(repo.path(), home.path(), &pack_dir);
    let keys = keys_of(&findings, "override-default.scalar-set-orphaned");

    assert_eq!(
        keys.len(),
        2,
        "both dropped keys orphan; got:\n{findings:#?}"
    );
    let targets: Vec<_> = keys.iter().map(|k| k["target"].clone()).collect();
    assert!(
        targets.contains(&serde_json::json!("scalar:legacy-knob"))
            && targets.contains(&serde_json::json!("scalar:retired-knob")),
        "each orphaned scalar-set keys at its own `scalar:<knob-key>`; got: {targets:?}"
    );
    assert_ne!(
        keys[0], keys[1],
        "two orphaned scalar-sets in ONE emitted array must not collide on one key: {keys:?}"
    );
}

/// An orphaned `slot-fill` keys at `step:<step-id>#<fill-id>`, and a content `conflict` keys
/// at the `workflow:<workflow-id>#<step-id>` its `render_target` builds — the two remaining
/// variants of the sixth form, emitted in the same array.
#[test]
fn a_slot_fill_and_a_content_conflict_key_at_their_delta_targets() {
    let repo = TempDir::new("variants");
    let home = TempDir::new("home");

    // The pack's `validate` body changed since the fork's recorded basis → `content-changed`;
    // its `locate` body no longer declares the `{{fill: hints}}` point → `slot-fill-orphaned`.
    let validate_v1 = "validate body v1\n";
    let recorded = engine::file_state::hash_bytes(validate_v1.as_bytes());
    let pack_dir = seed_pack(
        &repo,
        &[
            ("validate", "validate body v2 — changed\n"),
            ("locate", "find the code\n"),
        ],
    );
    seed_repo(
        repo.path(),
        &format!(
            "\
deltas:
  - kind: slot-fill
    target: step:locate#hints
    content: fills/house-rules.md
  - kind: tracked-fork
    target: workflow:single-task#validate
    base-version: v1
    base-hash: {recorded}
"
        ),
    );

    let findings = upgrade_findings(repo.path(), home.path(), &pack_dir);

    let slot_fill = keys_of(&findings, "override-default.slot-fill-orphaned");
    assert_eq!(
        slot_fill.len(),
        1,
        "the dropped fill point orphans; got:\n{findings:#?}"
    );
    assert_eq!(
        slot_fill[0]["target"], "step:locate#hints",
        "an orphaned slot-fill keys at its `step:<id>#<fill-id>` delta target; got: {:?}",
        slot_fill[0]
    );

    let conflict = keys_of(&findings, "override-default.content-changed");
    assert_eq!(
        conflict.len(),
        1,
        "the changed fork target conflicts; got:\n{findings:#?}"
    );
    assert_eq!(
        conflict[0]["target"], "workflow:single-task#validate",
        "a content conflict keys at its `workflow:<id>#<step-id>` delta target; got: {:?}",
        conflict[0]
    );
}
