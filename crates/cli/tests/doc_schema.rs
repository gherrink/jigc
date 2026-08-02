//! M40 Increment 4 / T2 — `jigc doc schema <doctype>`, the **third read surface**:
//! the deterministic structural projection of a doctype's RESOLVED schema, proven
//! end-to-end over the real `jigc` binary under the `[dev ▸ methodology]`
//! composition (`design/doc-read-surface.md` → Why json is a contract here;
//! `design/introspection.md` — the third-surface sentence; `DECISIONS.md`
//! 2026-07-10 → M40 Settle #3).
//!
//! The load-bearing contract this pins is the **separately-pinned, explicitly
//! versioned `--format json` shape** — `contract-version: 4` (the M45 bump: the
//! three settability states — an id-from leaf carries `add-item` [+ `retitle-item`
//! iff its type is string], a `set: on-create` stamp carries `set-field`, a
//! machine-maintained absolute carries none; 3 was the M43 rc.7 write-address join,
//! 2 the M41 rc.5 `of`/`section` join), golden-pinned at ship:
//! `{ contract-version, type, schema-version-or-null, fields, sections }` with
//! per-field `{id, type, of?, required, author-required, default?, set?, section?,
//! set-field? | (add-item? + retitle-item?)}` (`of` = the enum members, universal
//! across depths; `section` = the owning simple-section id, top-level fields only;
//! `set-field` = the concrete `jigc doc set-field` address of a directly settable
//! field — **absent** on a machine-maintained absolute [`set: schema-version` /
//! `on-transition`] and on a block's `id-from` leaf, which instead carries
//! `add-item` [the block address, always] + `retitle-item` [the item address, iff a
//! string id-from — an enum id-from carries `add-item` alone]) and per-section
//! `{id, kind, optional?, set-slot?|add-item?, item: {fields, slots, nested}}` —
//! a slot section carries its `set-slot` address, a repeatable its `add-item`
//! address — the `item` object **recursive** for nested repeatables, its slots
//! carrying `set-slot` and its nested blocks `add-item`. The three witnesses:
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

// ---- the pinned `--format json` goldens (contract-version 3, byte-verbatim) ----

/// dogfood-record — methodology, frozen v1 (M40 A1): `schema-version` 1 with the
/// loader-injected stamp field appended; the 14 meta fields stay author-required
/// (no default, no set, no optional/pack exemption) and each carries its
/// `set-field` write address (the stamp, `set:`-derived, carries none).
const DOGFOOD_RECORD_JSON: &str = r#"{
  "contract-version": 4,
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
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/case"
    },
    {
      "id": "binary-sha",
      "type": "string",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/binary-sha"
    },
    {
      "id": "adapter-writes",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/adapter-writes"
    },
    {
      "id": "oob-edits",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/oob-edits"
    },
    {
      "id": "drift-caught",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/drift-caught"
    },
    {
      "id": "validate-blocks",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/validate-blocks"
    },
    {
      "id": "halts-expected",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/halts-expected"
    },
    {
      "id": "halts-unplanned",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/halts-unplanned"
    },
    {
      "id": "fix-rounds",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/fix-rounds"
    },
    {
      "id": "audit-findings",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/audit-findings"
    },
    {
      "id": "seeded-oob",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/seeded-oob"
    },
    {
      "id": "seeded-blocks",
      "type": "int",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/seeded-blocks"
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
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/verdict"
    },
    {
      "id": "owner-artifact",
      "type": "owned-location",
      "required": true,
      "author-required": true,
      "section": "meta",
      "set-field": "dogfood-record:<slug>#meta/owner-artifact"
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
      "kind": "slot",
      "set-slot": "dogfood-record:<slug>#judgment"
    }
  ]
}"#;

/// changelog — schema-version 2 (the M38 relocation bump), the loader-injected
/// stamp field in `fields`, and the RECURSIVE nested `item` (releases → changes).
/// The address witnesses the three settability states (M45): an **enum** `id-from`
/// leaf (`category`, top-level and nested) carries `add-item` **alone** (a member
/// change is an identity change — no `retitle-item`); a **string** `id-from` leaf
/// (`title`) carries the `add-item` + `retitle-item` pair (the item address, one
/// hop shallower); a `set: on-create` stamp (`date`) is an author-overridable stamp
/// and carries `set-field`; the optional `link` carries `set-field`; every item
/// slot `set-slot`; and every repeatable — nested included — its section `add-item`.
const CHANGELOG_JSON: &str = r#"{
  "contract-version": 4,
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
      "add-item": "changelog:<slug>#unreleased-changes",
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
            "author-required": true,
            "add-item": "changelog:<slug>#unreleased-changes"
          }
        ],
        "slots": [
          {
            "id": "notes",
            "set-slot": "changelog:<slug>#unreleased-changes/<id>/notes"
          }
        ],
        "nested": []
      }
    },
    {
      "id": "releases",
      "kind": "repeatable",
      "add-item": "changelog:<slug>#releases",
      "item": {
        "fields": [
          {
            "id": "title",
            "type": "string",
            "required": true,
            "author-required": true,
            "add-item": "changelog:<slug>#releases",
            "retitle-item": "changelog:<slug>#releases/<id>"
          },
          {
            "id": "date",
            "type": "date",
            "required": true,
            "author-required": false,
            "set": "on-create",
            "set-field": "changelog:<slug>#releases/<id>/date"
          },
          {
            "id": "link",
            "type": "string",
            "required": false,
            "author-required": false,
            "set-field": "changelog:<slug>#releases/<id>/link"
          }
        ],
        "slots": [],
        "nested": [
          {
            "id": "changes",
            "add-item": "changelog:<slug>#releases/<id>/changes",
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
                  "author-required": true,
                  "add-item": "changelog:<slug>#releases/<id>/changes"
                }
              ],
              "slots": [
                {
                  "id": "notes",
                  "set-slot": "changelog:<slug>#releases/<id>/changes/<id>/notes"
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

/// adr — the field-rendering witness: a `default:` enum (settable — the canonical
/// `set-field` target), a `set: on-create` date (an author-overridable stamp —
/// directly settable, so it carries `set-field` since M45), an optional `ref`
/// (required false), a pack-declared `code-anchor` (required false), the injected
/// stamp (a machine-maintained absolute — address-less), and an `optional:` slot
/// section (its `set-slot` address carried like its required siblings).
/// Schema-version 2: the M36 `options`-slot migration bumped adr past v1.
const ADR_JSON: &str = r#"{
  "contract-version": 4,
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
      "section": "status",
      "set-field": "adr:<slug>#status/status"
    },
    {
      "id": "date",
      "type": "date",
      "required": true,
      "author-required": false,
      "set": "on-create",
      "section": "status",
      "set-field": "adr:<slug>#status/date"
    },
    {
      "id": "supersedes",
      "type": "ref",
      "required": false,
      "author-required": false,
      "section": "status",
      "set-field": "adr:<slug>#status/supersedes"
    },
    {
      "id": "cites-code",
      "type": "code-anchor",
      "required": false,
      "author-required": false,
      "section": "status",
      "set-field": "adr:<slug>#status/cites-code"
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
      "kind": "slot",
      "set-slot": "adr:<slug>#context"
    },
    {
      "id": "options",
      "kind": "slot",
      "optional": true,
      "set-slot": "adr:<slug>#options"
    },
    {
      "id": "decision",
      "kind": "slot",
      "set-slot": "adr:<slug>#decision"
    },
    {
      "id": "consequences",
      "kind": "slot",
      "set-slot": "adr:<slug>#consequences"
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
        "the dogfood-record schema json is the pinned contract-version-4 shape",
    );

    // (2) dogfood-record — the Proves line, asserted behaviorally (not just bytes):
    //     schema-version 1 (frozen by the M40 A1 methodology manifest), the 14 meta
    //     fields plus the injected stamp, every meta field author-required.
    let value: serde_json::Value =
        serde_json::from_str(&dogfood).expect("the emitted contract parses as json");
    assert_eq!(value["contract-version"], 4, "the contract is versioned");
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
    let adr = stdout_of(&out);
    assert_eq!(
        adr.trim_end(),
        ADR_JSON,
        "the adr schema json is the pinned shape (ref/pack/default/set fields)",
    );

    // (5) the three settability states (M45 Inc 3), asserted behaviorally on top of
    //     the byte pin: a plain settable field names its `set-field` address; a
    //     `set: on-create` stamp is author-overridable, so it ALSO names `set-field`;
    //     a machine-maintained absolute (`schema-version`) carries none.
    let value: serde_json::Value =
        serde_json::from_str(&adr).expect("the emitted adr contract parses as json");
    let fields = value["fields"].as_array().expect("`fields` is an array");
    let field = |id: &str| {
        fields
            .iter()
            .find(|f| f["id"] == id)
            .unwrap_or_else(|| panic!("the adr `{id}` field renders; got:\n{value}"))
    };
    assert_eq!(
        field("status")["set-field"],
        "adr:<slug>#status/status",
        "a settable field carries its concrete set-field address",
    );
    assert_eq!(
        field("date")["set-field"],
        "adr:<slug>#status/date",
        "a `set: on-create` stamp is author-overridable — directly settable, so it \
         carries a set-field address (the M45 three-state split)",
    );
    assert!(
        field("schema-version").get("set-field").is_none(),
        "a machine-maintained absolute (the schema-version stamp) is CLI-owned, never \
         addressed for set-field",
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

/// The plain (`agent`/`human`) listing surfaces the same write-verb addresses the
/// pinned json carries — non-contractually (only the json shape is the pin): a
/// settable field line names its `set-field` address, a slot section its
/// `set-slot`, a repeatable its `add-item`. The three states hold in text too: a
/// string `id-from` leaf names its `add-item` + `retitle-item` pair, an enum
/// `id-from` its `add-item` alone, and a machine-maintained absolute (the stamp)
/// stays address-less — inert, never an error.
#[test]
fn doc_schema_plain_listing_surfaces_write_addresses() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // adr — field + slot-section addresses; the stamp stays address-less.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "agent"],
    );
    assert_ok(&out, "`jigc doc schema adr --format agent`");
    let adr = stdout_of(&out);
    assert!(
        field_line(&adr, "status").contains("(set-field: adr:<slug>#status/status)"),
        "a settable field line names its set-field address; got:\n{adr}",
    );
    assert!(
        !field_line(&adr, "schema-version").contains("(set-field:"),
        "the CLI-stamped field line stays address-less; got:\n{adr}",
    );
    assert!(
        adr.contains("- context: slot (set-slot: adr:<slug>#context)"),
        "a slot section line names its set-slot address; got:\n{adr}",
    );

    // changelog — repeatable + nested addresses; the id-from leaf stays address-less.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "changelog", "--format", "agent"],
    );
    assert_ok(&out, "`jigc doc schema changelog --format agent`");
    let changelog = stdout_of(&out);
    assert!(
        changelog.contains("- releases: repeatable (add-item: changelog:<slug>#releases)"),
        "a repeatable section line names its add-item address; got:\n{changelog}",
    );
    assert!(
        changelog.contains("(add-item: changelog:<slug>#releases/<id>/changes)"),
        "a nested repeatable line names its add-item address; got:\n{changelog}",
    );
    // The string `id-from` leaf (`title`) names its id-source pair, never `set-field`.
    let title = field_line(&changelog, "title");
    assert!(
        !title.contains("(set-field:")
            && title.contains("(add-item: changelog:<slug>#releases)")
            && title.contains("(retitle-item: changelog:<slug>#releases/<id>)"),
        "a string id-from leaf names its add-item + retitle-item pair, not set-field; got:\n{title}",
    );
    // The enum `id-from` leaf (`category`) names `add-item` alone — no `retitle-item`
    // (a member change is an identity change).
    let category = field_line(&changelog, "category");
    assert!(
        category.contains("(add-item: changelog:<slug>#unreleased-changes)")
            && !category.contains("(retitle-item:"),
        "an enum id-from leaf names add-item alone, never retitle-item; got:\n{category}",
    );
    // The `set: on-create` stamp (`date`) is author-overridable — directly settable.
    assert!(
        field_line(&changelog, "date").contains("(set-field: changelog:<slug>#releases/<id>/date)"),
        "a `set: on-create` stamp names its set-field address; got:\n{changelog}",
    );
}

/// `milestone-record` is machine-maintained **whole** — `machine_maintained_guard`
/// refuses every `jigc doc` write to it (`design/team-ready-state.md` → The record is
/// not writable through the `jigc doc` verbs) — so its projection advertises **no**
/// write address at all, by a **doctype** exclusion rather than the per-leaf
/// `set:`-kind rule (M45 Inc 3; `design/doc-read-surface.md` → the settability
/// states). This closes the live parity gap the planner named: before M45,
/// `add-item: milestone-record:<slug>#tasks` was advertised though the write path
/// refuses it whole. The **omitting context** of the three-state rule: every
/// settable-looking leaf (the `base` `set: on-create` stamp, the `tasks` `task-id`
/// string id-from) stays address-less here, where on any other doctype it would carry
/// one — the doctype-level guard wins over the per-leaf states.
#[test]
fn doc_schema_milestone_record_advertises_no_write_address() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "milestone-record", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema milestone-record --format json`");
    let json = stdout_of(&out);
    for verb in ["set-field", "set-slot", "add-item", "retitle-item"] {
        assert!(
            !json.contains(&format!("\"{verb}\"")),
            "a milestone-record is machine-maintained whole — no `{verb}` address may \
             appear in its projection; got:\n{json}",
        );
    }
    // It still projects — the suppression drops only the write addresses, never the
    // schema shape: the contract version, and the `tasks` repeatable (address-less).
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("the milestone-record contract parses as json");
    assert_eq!(value["contract-version"], 4, "the contract is versioned");
    let section_ids: Vec<&str> = value["sections"]
        .as_array()
        .expect("`sections` is an array")
        .iter()
        .filter_map(|s| s["id"].as_str())
        .collect();
    assert!(
        section_ids.contains(&"tasks"),
        "the `tasks` repeatable still projects (address-less); got:\n{json}",
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
