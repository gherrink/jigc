//! M40 Increment 4 / T2 — `jigc doc schema <doctype>`, the **third read surface**:
//! the deterministic structural projection of a doctype's RESOLVED schema, proven
//! end-to-end over the real `jigc` binary under the `[dev ▸ methodology]`
//! composition (`design/doc-read-surface.md` → Why json is a contract here;
//! `design/introspection.md` — the third-surface sentence; `DECISIONS.md`
//! 2026-07-10 → M40 Settle #3).
//!
//! The load-bearing contract this pins is the **separately-pinned, explicitly
//! versioned `--format json` shape** — `contract-version: 1`, golden-pinned at
//! ship: `{ contract-version, type, schema-version-or-null, fields, sections }`
//! with per-field `{id, type, required, author-required, default?, set?}` and
//! per-section `{id, kind, optional?, item: {fields, slots, nested}}`, the `item`
//! object **recursive** for nested repeatables. The three witnesses:
//!
//! - **dogfood-record** — a methodology doctype (manifest-less at this increment):
//!   `schema-version` null, and ALL 14 meta fields `author-required: true` (the
//!   roadmap Proves line), asserted behaviorally as well as by golden. Inc 5 (A1)
//!   flips this doctype's version non-null + injects its stamp field — that golden
//!   refresh is Inc 5's blast radius, pinned here against today's resolved shape.
//! - **changelog** — schema-version 2 + the recursive nested `item` (the M22
//!   two-level repeatable), with the loader-injected stamp field in `fields`.
//! - **adr** — ref / pack-typed / default / set field rendering.
//!
//! Everything is asserted on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`); an unknown doctype is driven through the real exit
//! code + routed stderr. No external test crates.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-doc-schema-{tag}-{}-{:?}",
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

/// Initialize a git repo with the `.jigc/config/` project layer, naming the
/// methodology pack in `packs.yaml` (the `[dev ▸ methodology]` composition: both
/// the dev `adr`/`changelog` and the methodology `dogfood-record` resolve).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run jigc")
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

// ---- the pinned `--format json` goldens (contract-version 1, byte-verbatim) ----

/// dogfood-record — methodology (manifest-less): `schema-version` null; all 14
/// meta fields author-required (no default, no set, no optional/pack exemption).
const DOGFOOD_RECORD_JSON: &str = r#"{
  "contract-version": 1,
  "type": "dogfood-record",
  "schema-version": null,
  "fields": [
    {
      "id": "case",
      "type": "enum",
      "required": true,
      "author-required": true
    },
    {
      "id": "binary-sha",
      "type": "string",
      "required": true,
      "author-required": true
    },
    {
      "id": "adapter-writes",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "oob-edits",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "drift-caught",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "validate-blocks",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "halts-expected",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "halts-unplanned",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "fix-rounds",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "audit-findings",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "seeded-oob",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "seeded-blocks",
      "type": "int",
      "required": true,
      "author-required": true
    },
    {
      "id": "verdict",
      "type": "enum",
      "required": true,
      "author-required": true
    },
    {
      "id": "owner-artifact",
      "type": "owned-location",
      "required": true,
      "author-required": true
    }
  ],
  "sections": [
    {
      "id": "judgment",
      "kind": "slot"
    }
  ]
}"#;

/// changelog — schema-version 2 (the M38 relocation bump), the loader-injected
/// stamp field in `fields`, and the RECURSIVE nested `item` (releases → changes).
const CHANGELOG_JSON: &str = r#"{
  "contract-version": 1,
  "type": "changelog",
  "schema-version": 2,
  "fields": [
    {
      "id": "schema-version",
      "type": "int",
      "required": true,
      "author-required": false,
      "set": "schema-version"
    }
  ],
  "sections": [
    {
      "id": "unreleased-changes",
      "kind": "repeatable",
      "item": {
        "fields": [
          {
            "id": "category",
            "type": "enum",
            "required": true,
            "author-required": true
          }
        ],
        "slots": [
          {
            "id": "notes"
          }
        ],
        "nested": []
      }
    },
    {
      "id": "releases",
      "kind": "repeatable",
      "item": {
        "fields": [
          {
            "id": "title",
            "type": "string",
            "required": true,
            "author-required": true
          },
          {
            "id": "date",
            "type": "date",
            "required": true,
            "author-required": false,
            "set": "on-create"
          },
          {
            "id": "link",
            "type": "string",
            "required": false,
            "author-required": false
          }
        ],
        "slots": [],
        "nested": [
          {
            "id": "changes",
            "item": {
              "fields": [
                {
                  "id": "category",
                  "type": "enum",
                  "required": true,
                  "author-required": true
                }
              ],
              "slots": [
                {
                  "id": "notes"
                }
              ],
              "nested": []
            }
          }
        ]
      }
    }
  ]
}"#;

/// adr — the field-rendering witness: a `default:` enum, a `set: on-create` date,
/// an optional `ref` (required false), a pack-declared `code-anchor` (required
/// false), the injected stamp, and an `optional:` slot section. Schema-version 2:
/// the M36 `options`-slot migration bumped adr past v1.
const ADR_JSON: &str = r#"{
  "contract-version": 1,
  "type": "adr",
  "schema-version": 2,
  "fields": [
    {
      "id": "status",
      "type": "enum",
      "required": true,
      "author-required": false,
      "default": "proposed"
    },
    {
      "id": "date",
      "type": "date",
      "required": true,
      "author-required": false,
      "set": "on-create"
    },
    {
      "id": "supersedes",
      "type": "ref",
      "required": false,
      "author-required": false
    },
    {
      "id": "cites-code",
      "type": "code-anchor",
      "required": false,
      "author-required": false
    },
    {
      "id": "schema-version",
      "type": "int",
      "required": true,
      "author-required": false,
      "set": "schema-version"
    }
  ],
  "sections": [
    {
      "id": "context",
      "kind": "slot"
    },
    {
      "id": "options",
      "kind": "slot",
      "optional": true
    },
    {
      "id": "decision",
      "kind": "slot"
    },
    {
      "id": "consequences",
      "kind": "slot"
    }
  ]
}"#;

/// The three witnesses match their pinned `--format json` goldens byte-verbatim,
/// and the dogfood-record Proves line holds behaviorally (14 fields, all
/// `author-required: true`, `schema-version` null).
#[test]
fn doc_schema_json_is_the_pinned_contract() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // (1) dogfood-record — the golden pin.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "dogfood-record", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema dogfood-record --format json`");
    let dogfood = stdout_of(&out);
    assert_eq!(
        dogfood.trim_end(),
        DOGFOOD_RECORD_JSON,
        "the dogfood-record schema json is the pinned contract-version-1 shape",
    );

    // (2) dogfood-record — the Proves line, asserted behaviorally (not just bytes):
    //     schema-version null, exactly 14 fields, every one author-required.
    let value: serde_json::Value =
        serde_json::from_str(&dogfood).expect("the emitted contract parses as json");
    assert_eq!(value["contract-version"], 1, "the contract is versioned");
    assert!(
        value["schema-version"].is_null(),
        "a manifest-less methodology doctype reports schema-version null",
    );
    let fields = value["fields"].as_array().expect("`fields` is an array");
    assert_eq!(fields.len(), 14, "all 14 dogfood-record meta fields render");
    for field in fields {
        assert_eq!(
            field["author-required"],
            serde_json::Value::Bool(true),
            "every dogfood-record meta field is author-required; `{}` is not",
            field["id"],
        );
    }

    // (3) changelog — schema-version 2 + the recursive nested `item`.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "changelog", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema changelog --format json`");
    assert_eq!(
        stdout_of(&out).trim_end(),
        CHANGELOG_JSON,
        "the changelog schema json is the pinned shape (schema-version 2, recursive item)",
    );

    // (4) adr — ref / pack-typed / default / set field rendering.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema adr --format json`");
    assert_eq!(
        stdout_of(&out).trim_end(),
        ADR_JSON,
        "the adr schema json is the pinned shape (ref/pack/default/set fields)",
    );
}

/// An unknown doctype exits non-zero with a routed error — never a panic, never
/// exit-0 silence (`design/doc-read-surface.md`: a read-side block routes like a
/// write block).
#[test]
fn doc_schema_unknown_doctype_routes_a_block() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["doc", "schema", "no-such-type"]);
    assert!(
        !out.status.success(),
        "an unknown doctype must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no-such-type"),
        "the block names the unknown doctype; got:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc describe"),
        "the block routes to the doctype catalog (`jigc describe`); got:\n{stderr}",
    );
}
