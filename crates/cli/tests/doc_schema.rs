//! M40 Increment 4 / T2 — `jigc doc schema <doctype>`, the **third read surface**:
//! the deterministic structural projection of a doctype's RESOLVED schema, proven
//! end-to-end over the real `jigc` binary under the `[dev ▸ methodology]`
//! composition (`design/doc-read-surface.md` → Why json is a contract here;
//! `design/introspection.md` — the third-surface sentence; `DECISIONS.md`
//! 2026-07-10 → M40 Settle #3).
//!
//! The load-bearing contract this pins is the **separately-pinned, explicitly
//! versioned `--format json` shape** — `contract-version: 2` (the M41 rc.5 bump:
//! per-field `of` enum members + the field→section mapping joined the projection),
//! golden-pinned at ship:
//! `{ contract-version, type, schema-version-or-null, fields, sections }` with
//! per-field `{id, type, of?, required, author-required, default?, set?, section?}`
//! (`of` = the enum members, universal across depths; `section` = the owning
//! simple-section id, top-level fields only) and per-section
//! `{id, kind, optional?, item: {fields, slots, nested}}`, the `item` object
//! **recursive** for nested repeatables. The three witnesses:
//!
//! - **dogfood-record** — a methodology doctype, frozen at schema-version 1 since
//!   Inc 5 (A1, the methodology `schema-manifest.yaml`): the loader-injected stamp
//!   field joins its 14 meta fields (which stay ALL `author-required: true` — the
//!   roadmap Proves line), asserted behaviorally as well as by golden.
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

// ---- the pinned `--format json` goldens (contract-version 2, byte-verbatim) ----

/// dogfood-record — methodology, frozen v1 (M40 A1): `schema-version` 1 with the
/// loader-injected stamp field appended; the 14 meta fields stay author-required
/// (no default, no set, no optional/pack exemption).
const DOGFOOD_RECORD_JSON: &str = r#"{
  "contract-version": 2,
  "type": "dogfood-record",
  "schema-version": 1,
  "fields": [
    {
      "id": "case",
      "type": "enum",
      "of": [
        "pilot",
        "existing-docs",
        "greenfield"
      ],
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "binary-sha",
      "type": "string",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "adapter-writes",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "oob-edits",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "drift-caught",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "validate-blocks",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "halts-expected",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "halts-unplanned",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "fix-rounds",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "audit-findings",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "seeded-oob",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "seeded-blocks",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "verdict",
      "type": "enum",
      "of": [
        "green",
        "red"
      ],
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "owner-artifact",
      "type": "owned-location",
      "required": true,
      "author-required": true,
      "section": "meta"
    },
    {
      "id": "schema-version",
      "type": "int",
      "required": true,
      "author-required": false,
      "set": "schema-version",
      "section": "meta"
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
  "contract-version": 2,
  "type": "changelog",
  "schema-version": 2,
  "fields": [
    {
      "id": "schema-version",
      "type": "int",
      "required": true,
      "author-required": false,
      "set": "schema-version",
      "section": "meta"
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
            "of": [
              "added",
              "changed",
              "deprecated",
              "removed",
              "fixed",
              "security"
            ],
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
                  "of": [
                    "added",
                    "changed",
                    "deprecated",
                    "removed",
                    "fixed",
                    "security"
                  ],
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
  "contract-version": 2,
  "type": "adr",
  "schema-version": 2,
  "fields": [
    {
      "id": "status",
      "type": "enum",
      "of": [
        "proposed",
        "accepted",
        "superseded"
      ],
      "required": true,
      "author-required": false,
      "default": "proposed",
      "section": "status"
    },
    {
      "id": "date",
      "type": "date",
      "required": true,
      "author-required": false,
      "set": "on-create",
      "section": "status"
    },
    {
      "id": "supersedes",
      "type": "ref",
      "required": false,
      "author-required": false,
      "section": "status"
    },
    {
      "id": "cites-code",
      "type": "code-anchor",
      "required": false,
      "author-required": false,
      "section": "status"
    },
    {
      "id": "schema-version",
      "type": "int",
      "required": true,
      "author-required": false,
      "set": "schema-version",
      "section": "status"
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
        "the dogfood-record schema json is the pinned contract-version-2 shape",
    );

    // (2) dogfood-record — the Proves line, asserted behaviorally (not just bytes):
    //     schema-version 1 (frozen by the M40 A1 methodology manifest), the 14 meta
    //     fields plus the injected stamp, every meta field author-required.
    let value: serde_json::Value =
        serde_json::from_str(&dogfood).expect("the emitted contract parses as json");
    assert_eq!(value["contract-version"], 2, "the contract is versioned");
    assert_eq!(
        value["schema-version"], 1,
        "a manifest-frozen methodology doctype reports schema-version 1",
    );
    let fields = value["fields"].as_array().expect("`fields` is an array");
    assert_eq!(
        fields.len(),
        15,
        "the 14 dogfood-record meta fields render, plus the injected stamp",
    );
    for field in fields {
        let is_stamp = field["id"] == "schema-version";
        assert_eq!(
            field["author-required"],
            serde_json::Value::Bool(!is_stamp),
            "every dogfood-record meta field is author-required (the CLI-derived \
             stamp is not); `{}` breaks that",
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

/// The plain (`--format agent`/`human`) listing mirrors the `of` enum members so
/// an agent orienting by the non-contractual listing still sees the legal values —
/// the text mirror of the pinned json's `of` (M41 rc.5).
#[test]
fn doc_schema_plain_listing_shows_enum_members() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "agent"],
    );
    assert_ok(&out, "`jigc doc schema adr --format agent`");
    let listing = stdout_of(&out);
    assert!(
        listing.contains("status: enum") && listing.contains("[proposed|accepted|superseded]"),
        "the plain listing shows an enum field's members; got:\n{listing}",
    );
}

/// The listing line for field `id` — the `- <id>: …` line at any depth.
fn field_line<'a>(listing: &'a str, id: &str) -> &'a str {
    listing
        .lines()
        .find(|line| line.trim_start().starts_with(&format!("- {id}: ")))
        .unwrap_or_else(|| panic!("the listing has a `- {id}: …` line; got:\n{listing}"))
}

/// The plain (`agent`/`human`) listing names each **top-level** field's owning
/// simple-section — the field group an agent must address to write it (the `doc
/// author` payload is section-keyed: `sections: - id: <section-id>` / `set:`), and
/// the one thing the flat `fields:` list could not tell it. The pinned json has
/// carried `section` since contract-version 2; the listing never read it (M42 rc.6,
/// T3 — the papercut batch).
///
/// The **omitting context** is asserted too: an item field under a repeatable
/// carries its section *structurally* (it is printed indented under it), so its line
/// stays section-less — inert, not wrong. Only the json shape is the pin
/// (`design/doc-read-surface.md`); this listing is non-contractual presentation, and
/// the byte-verbatim goldens above hold it to an unchanged json key set.
#[test]
fn doc_schema_plain_listing_names_each_field_owning_section() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // (1) adr — every top-level field names its owning section, which is `status`:
    //     the very guess an agent gets wrong (most doctypes group under `meta`).
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "agent"],
    );
    assert_ok(&out, "`jigc doc schema adr --format agent`");
    let adr = stdout_of(&out);
    for id in [
        "status",
        "date",
        "supersedes",
        "cites-code",
        "schema-version",
    ] {
        let line = field_line(&adr, id);
        assert!(
            line.contains("(section: status)"),
            "the adr field `{id}` names its owning section; got:\n{line}",
        );
    }

    // (2) spec — the section is READ from the schema, not hardcoded: spec groups its
    //     fields under `meta`, so the same listing says `meta` there.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "spec", "--format", "agent"],
    );
    assert_ok(&out, "`jigc doc schema spec --format agent`");
    let spec = stdout_of(&out);
    let derived = field_line(&spec, "derived-from");
    assert!(
        derived.contains("(section: meta)"),
        "the spec field `derived-from` names its own owning section; got:\n{derived}",
    );

    // (3) changelog — the omitting context: its ONLY top-level field is the injected
    //     stamp (section `meta`); every item field under a repeatable (`category`,
    //     `title`, `date`, `link`, and the nested `category`) carries its section
    //     structurally, so no item line names one.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "changelog", "--format", "agent"],
    );
    assert_ok(&out, "`jigc doc schema changelog --format agent`");
    let changelog = stdout_of(&out);
    let stamp = field_line(&changelog, "schema-version");
    assert!(
        stamp.contains("(section: meta)"),
        "the changelog stamp field names its owning section; got:\n{stamp}",
    );
    let annotated: Vec<&str> = changelog
        .lines()
        .filter(|line| line.contains("(section:"))
        .collect();
    assert_eq!(
        annotated,
        vec![stamp],
        "only the top-level field line names a section — an item field under a \
         repeatable carries its section structurally; got:\n{changelog}",
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
