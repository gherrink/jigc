//! M40 Increment 4 / T2 — `jigc doc schema <doctype>`, the **third read surface**:
//! the deterministic structural projection of a doctype's RESOLVED schema, proven
//! end-to-end over the real `jigc` binary under the `[dev ▸ methodology]`
//! composition (`design/doc-read-surface.md` → Why json is a contract here;
//! `design/introspection.md` — the third-surface sentence; `DECISIONS.md`
//! 2026-07-10 → M40 Settle #3).
//!
//! The load-bearing contract this pins is the **separately-pinned, explicitly
//! versioned `--format json` shape** — `contract-version: 7` (the M52 bump: the
//! `identity`/`home` pair naming which addresses a doctype's instances have and where
//! they live, pooled with `base`'s compound `json-shape`; 6 was the M50 `ref` field's
//! `to:`, the target doctype it references; 5 was the M48
//! id-source's `write-key`; 4 was the M45 three settability states — an id-from leaf
//! carries `add-item` [+ `retitle-item` iff its type is string], a `set: on-create`
//! stamp carries `set-field`, a machine-maintained absolute carries none; 3 was the
//! M43 rc.7 write-address join, 2 the M41 rc.5 `of`/`section` join), golden-pinned at
//! ship: `{ contract-version, type, schema-version-or-null, identity, home, fields,
//! sections }` — `identity` = `{kind: fixed|slugged, address}` and `home` =
//! `{kind: placement|location|transient, path-or-null}`, both rendered from the ONE
//! engine primitive `Schema::projection` the `{{schema:<doctype>}}` compose seam's
//! home line reads, so the two surfaces cannot answer the same question differently —
//! with
//! per-field `{id, type, of?, to?, required, author-required, default?, set?, section?,
//! set-field? | (add-item? + retitle-item? + write-key), json-shape?}` (`of` = the enum members,
//! universal across depths; `to` = a `ref`'s target doctype, the one thing
//! `write.malformed-value`'s own route sends the author here to read;
//! `section` = the owning simple-section id, top-level fields
//! only; `set-field` = the concrete `jigc doc set-field` address of a directly settable
//! field — **absent** on a machine-maintained absolute [`set: schema-version` /
//! `on-transition`] and on a block's `id-from` leaf, which instead carries
//! `add-item` [the block address, always] + `retitle-item` [the item address, iff a
//! string id-from — an enum id-from carries `add-item` alone] + `write-key` [the
//! payload key both those verbs take, `--title` whatever the field is called];
//! `json-shape` = the object shape the CONTENT read returns where it differs from the
//! declared `type` — the milestone-record's `base` pin, the one compound leaf in the
//! pinned surface) and
//! per-section `{id, kind, optional?, set-slot?|add-item?, item: {fields, slots,
//! nested}}` —
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

use clap::Parser;
use cli::cli::Cli;
use cli::doc::ID_SOURCE_WRITE_KEY;
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
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// A repo with the same `[dev ▸ methodology]` project layer **plus a real
/// `jigc setup`** — the state a task can be minted and a doc staged in (the plain
/// `init_repo` above is enough for the read-only `doc schema` arms, which need no
/// workbench).
fn init_repo_with_setup(repo: &Path, home: &Path) {
    init_repo(repo);
    git(repo, &["config", "user.email", "doc-schema@example.com"]);
    git(repo, &["config", "user.name", "Doc Schema Suite"]);
    fs::write(repo.join("README.md"), "doc schema suite\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// A repo whose project layer composes the **embedded** methodology pack under the dev
/// pack (`compose-embedded-methodology: true`), plus a real `jigc setup` — the
/// composition the shipped binary ships, and the one this suite's identity/home arms
/// need: with **dev** primary its `knobs.yaml` wins, so `docs-root` is a declared knob.
/// Under [`init_repo`]'s methodology-primary layout it is not — methodology's whole-file
/// `knobs.yaml` shadow drops `docs-root` deliberately (a recorded divergence,
/// `packs/methodology/config/knobs.yaml`), and `jigc config set docs-root` answers
/// `config.undeclared-key` there.
fn init_repo_embedded_with_setup(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write packs.yaml composing the embedded methodology pack");
    git(repo, &["config", "user.email", "doc-schema@example.com"]);
    git(repo, &["config", "user.name", "Doc Schema Suite"]);
    fs::write(repo.join("README.md"), "doc schema suite\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
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

// ---- the pinned `--format json` goldens (contract-version 7, byte-verbatim) ----

/// dogfood-record — methodology, frozen v1 (M40 A1): `schema-version` 1 with the
/// loader-injected stamp field appended; the 14 meta fields stay author-required
/// (no default, no set, no optional/pack exemption) and each carries its
/// `set-field` write address (the stamp, `set:`-derived, carries none).
const DOGFOOD_RECORD_JSON: &str = r#"{
  "contract-version": 7,
  "type": "dogfood-record",
  "schema-version": 1,
  "identity": {
    "kind": "slugged",
    "address": "dogfood-record:<slug>"
  },
  "home": {
    "kind": "location",
    "path": "dogfood/<slug>.md"
  },
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
  "contract-version": 7,
  "type": "changelog",
  "schema-version": 2,
  "identity": {
    "kind": "fixed",
    "address": "changelog"
  },
  "home": {
    "kind": "placement",
    "path": "CHANGELOG.md"
  },
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
            "add-item": "changelog:<slug>#unreleased-changes",
            "write-key": "--title"
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
            "retitle-item": "changelog:<slug>#releases/<id>",
            "write-key": "--title"
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
                  "add-item": "changelog:<slug>#releases/<id>/changes",
                  "write-key": "--title"
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
  "contract-version": 7,
  "type": "adr",
  "schema-version": 2,
  "identity": {
    "kind": "slugged",
    "address": "adr:<slug>"
  },
  "home": {
    "kind": "location",
    "path": "decisions/<slug>.md"
  },
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
      "to": "adr",
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
        "the dogfood-record schema json is the pinned contract-version-7 shape",
    );

    // (2) dogfood-record — the Proves line, asserted behaviorally (not just bytes):
    //     schema-version 1 (frozen by the M40 A1 methodology manifest), the 14 meta
    //     fields plus the injected stamp, every meta field author-required.
    let value: serde_json::Value =
        serde_json::from_str(&dogfood).expect("the emitted contract parses as json");
    assert_eq!(value["contract-version"], 7, "the contract is versioned");
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
    let date = field_line(&changelog, "date");
    assert!(
        date.contains("(set-field: changelog:<slug>#releases/<id>/date)"),
        "a `set: on-create` stamp names its set-field address; got:\n{changelog}",
    );
    // (M48 Inc 7 / T1 — F12) Both id-source leaves mirror the pinned json's `write-key`
    // in its own key spelling — the plain arm teaches the machine arm's vocabulary —
    // and the omitting context holds in text too: a directly settable field names no
    // payload key (its verb takes `--value`/`--unset`, not one key).
    let write_key = format!("(write-key: {ID_SOURCE_WRITE_KEY})");
    assert!(
        title.contains(&write_key) && category.contains(&write_key),
        "each id-source leaf line names the payload key its value is supplied under; \
         got:\n{changelog}",
    );
    assert!(
        !date.contains("(write-key:"),
        "a directly settable field line names no payload key; got:\n{date}",
    );
}

/// (M47 inc-10 T5 — N16) The plain listing's **`* = author-required` legend is printed
/// where its markers land, and only when one lands** (`design/surface-contract.md` →
/// law 1 *nothing lies* + law 2 *nothing hides*). It used to ride the `fields (…)`
/// header, which was wrong on both faces: the marker is appended to **item leaves under
/// `sections:`** too — `spec`'s only `*` is on `criteria/<id>`'s title, two levels away
/// from the header that explained it — and four shipped doctypes carry fields with **no**
/// author-required leaf anywhere, so the header announced a convention their listing
/// never used.
///
/// The axis is the **marker's location**, and the test iterates it: markers on top-level
/// fields only, on item leaves only, on both, and on neither.
#[test]
fn doc_schema_plain_listing_legends_the_marker_where_it_lands() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    const LEGEND: &str = "* = author-required";
    let listing = |doctype: &str| {
        let out = jigc(
            repo.path(),
            home.path(),
            &["doc", "schema", doctype, "--format", "agent"],
        );
        assert_ok(&out, "`jigc doc schema <doctype> --format agent`");
        stdout_of(&out)
    };
    // A marked line, wherever it sits in the listing.
    let marked = |text: &str| -> Vec<String> {
        text.lines()
            .filter(|line| line.ends_with(" *"))
            .map(str::to_owned)
            .collect()
    };
    // The legend leads the body: it sits after the `doctype:` line and before the
    // first `fields:`/`sections:` header, so it is read before any marker is met.
    let assert_legended = |text: &str, what: &str| {
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(
            lines.get(1).copied(),
            Some(LEGEND),
            "{what}: the legend leads the listing body; got:\n{text}",
        );
        assert_eq!(
            text.matches(LEGEND).count(),
            1,
            "{what}: the legend is stated once; got:\n{text}",
        );
    };

    // (1) Markers on ITEM LEAVES ONLY — `spec` marks `criteria/<id>`'s title and
    //     nothing at top level. The old placement put the legend in a `fields:` header
    //     whose own lines carry no marker at all.
    let spec = listing("spec");
    let spec_marked = marked(&spec);
    assert_eq!(
        spec_marked.len(),
        1,
        "spec marks exactly its criteria title; got:\n{spec}",
    );
    assert!(
        spec_marked[0].starts_with("    - title:"),
        "spec's only marker is an item leaf, indented under its repeatable; got:\n{spec}",
    );
    assert_legended(&spec, "spec");

    // (2) Markers on BOTH — `commit` marks the top-level `type` field and the two
    //     `trailers` item leaves; one legend still covers the whole listing.
    let commit = listing("commit");
    let commit_marked = marked(&commit);
    assert!(
        commit_marked.iter().any(|l| l.starts_with("  - type:"))
            && commit_marked.iter().any(|l| l.starts_with("    - key:")),
        "commit marks a top-level field AND item leaves; got:\n{commit}",
    );
    assert_legended(&commit, "commit");

    // (3) Markers on TOP-LEVEL FIELDS ONLY — `dogfood-record`'s meta fields are all
    //     author-required and its one section is a slot.
    let dogfood = listing("dogfood-record");
    assert!(
        marked(&dogfood).iter().all(|l| l.starts_with("  - ")),
        "dogfood-record marks top-level fields only; got:\n{dogfood}",
    );
    assert_legended(&dogfood, "dogfood-record");

    // (4) The OMITTING CONTEXT — NO marker anywhere. `adr` has five top-level fields
    //     and four sections, none author-required, so the listing states no legend at
    //     all: a legend for a convention this doctype never uses is a lie about the
    //     listing the reader is holding. Its `fields:`/`sections:` headers stay.
    let adr = listing("adr");
    assert!(
        marked(&adr).is_empty(),
        "adr carries no author-required leaf; got:\n{adr}",
    );
    assert!(
        !adr.contains(LEGEND),
        "no marker renders, so no legend is stated; got:\n{adr}",
    );
    assert!(
        adr.contains("\nfields:\n") && adr.contains("\nsections:\n"),
        "the section headers are unconditional — only the legend is marker-gated; \
         got:\n{adr}",
    );

    // (5) The same omitting context on a doctype whose marker-less listing has a
    //     DIFFERENT shape — `vision` (methodology): the rule is the marker's absence,
    //     never one doctype's layout.
    let vision = listing("vision");
    assert!(
        marked(&vision).is_empty() && !vision.contains(LEGEND),
        "a marker-less methodology listing states no legend either; got:\n{vision}",
    );
}

/// (M48 Inc 7 / T1 — F12) An **id-source** leaf's projection names the **payload key**
/// its value is supplied under: `write-key: --title`, whatever the field is called
/// (`design/write-commands.md` → The argument convention — *the flag names the role,
/// not the field*; `design/doc-read-surface.md` → the settability states).
///
/// The reported defect is law-2 *nothing hides* on the surface an agent is told to
/// consult: `doc schema changelog` projected `category: enum […] (add-item: …)` and a
/// payload written from that projection was rejected, with nothing anywhere naming
/// `title` as the spelling that works. The address said which **verb**; nothing said
/// which **key**, and the field's own id is the misleading answer.
///
/// The axis is the **id-from leaf**, iterated over `changelog` — the only shipped
/// doctype with an **enum** id-from, and it carries one at **both** nesting depths
/// (`unreleased-changes/<id>` and `releases/<id>/changes/<id>`) beside a **string**
/// id-from (`releases/<id>`) whose field id *happens* to be `title`, hiding the gap.
/// All three carry the same key: the state is *id-source*, never the field's spelling.
///
/// Two omitting contexts, because a marker that lands everywhere says nothing:
/// **no other settability state carries it** (a `set-field` entry's payload is the
/// verb's own `--value`/`--unset` pair, an item slot's is `--from-file <path|->` —
/// neither is one key, and neither is shadowed by a field id), and a doctype with no
/// repeatable at all (`adr`) carries none anywhere. The **suppressed** context —
/// `milestone-record`, machine-maintained whole — is asserted in its own test below.
///
/// The advertised key is **fenced against the real clap tree**: both verbs that take it
/// parse with it, and the field-id spelling the projection would otherwise imply does
/// not — so the projection cannot advertise a key the binary refuses.
#[test]
fn doc_schema_id_source_names_its_write_key() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "changelog", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema changelog --format json`");
    let value: serde_json::Value =
        serde_json::from_str(&stdout_of(&out)).expect("the changelog contract parses as json");
    assert_eq!(value["contract-version"], 7, "the contract is versioned");

    // The three id-source leaves, at both nesting depths and both id-from types.
    let sections = value["sections"]
        .as_array()
        .expect("`sections` is an array");
    let section = |id: &str| {
        sections
            .iter()
            .find(|s| s["id"] == id)
            .unwrap_or_else(|| panic!("the changelog `{id}` section projects; got:\n{value}"))
    };
    let leaf = |item: &serde_json::Value, id: &str| {
        item["fields"]
            .as_array()
            .expect("item fields")
            .iter()
            .find(|f| f["id"] == id)
            .unwrap_or_else(|| panic!("the item leaf `{id}` projects; got:\n{value}"))
            .clone()
    };
    let staging = section("unreleased-changes");
    let releases = section("releases");
    let nested = releases["item"]["nested"]
        .as_array()
        .expect("`nested` is an array")
        .iter()
        .find(|n| n["id"] == "changes")
        .expect("the nested `changes` block projects")
        .clone();

    for (what, field) in [
        // The enum id-from — the reported case, at both depths.
        (
            "unreleased-changes/<id>",
            leaf(&staging["item"], "category"),
        ),
        (
            "releases/<id>/changes/<id>",
            leaf(&nested["item"], "category"),
        ),
        // The string id-from, whose field id `title` coincides with the flag — the
        // coincidence that hid the gap. Same state, same key.
        ("releases/<id>", leaf(&releases["item"], "title")),
    ] {
        let id = field["id"].as_str().expect("a field has an id");
        let block = field["add-item"].as_str().unwrap_or_else(|| {
            panic!("{what}: the id-source leaf carries `add-item`; got:\n{field}")
        });
        assert_eq!(
            field["write-key"], ID_SOURCE_WRITE_KEY,
            "{what}: the id-source leaf names the payload key its value is supplied \
             under, whatever the field is called; got:\n{field}",
        );

        // The advertised key against the real clap tree: BOTH verbs that take the
        // id-source accept it — a projection that named a key the binary refuses would
        // be the same law-1 lie one level down.
        let block = block.replace("<slug>", "changelog").replace("<id>", "x");
        let item = format!("{block}/x");
        for argv in [
            vec!["jigc", "doc", "add-item", &block, ID_SOURCE_WRITE_KEY, "v"],
            vec![
                "jigc",
                "doc",
                "retitle-item",
                &item,
                ID_SOURCE_WRITE_KEY,
                "v",
            ],
        ] {
            assert!(
                Cli::try_parse_from(&argv).is_ok(),
                "{what}: the advertised write key does not parse: `{}`",
                argv.join(" "),
            );
            // The misleading answer the projection would otherwise imply — the field's
            // own id as a flag — is refused, which is the whole reason the key exists.
            if id != ID_SOURCE_WRITE_KEY.trim_start_matches('-') {
                let field_flag = format!("--{id}");
                let mut guess = argv.clone();
                guess[4] = &field_flag;
                assert!(
                    Cli::try_parse_from(&guess).is_err(),
                    "{what}: the field-id spelling `{field_flag}` parses — the gap this \
                     key closes would not exist",
                );
            }
        }
    }

    // Omitting context (1): no OTHER settability state carries the key — a directly
    // settable field (`date`, `link`), a machine-maintained absolute (the stamp), and
    // an item slot each name their verb by address alone.
    for (what, field) in [
        ("releases/<id>/date", leaf(&releases["item"], "date")),
        ("releases/<id>/link", leaf(&releases["item"], "link")),
        (
            "meta/schema-version",
            value["fields"]
                .as_array()
                .expect("`fields` is an array")
                .iter()
                .find(|f| f["id"] == "schema-version")
                .expect("the stamp field projects")
                .clone(),
        ),
    ] {
        assert!(
            field.get("write-key").is_none(),
            "{what}: only an id-source leaf names a payload key; got:\n{field}",
        );
    }
    let slot = &staging["item"]["slots"][0];
    assert!(
        slot.get("write-key").is_none(),
        "an item slot names no payload key (its write is `--from-file <path|->`, not \
         one key); got:\n{slot}",
    );

    // Omitting context (2): a doctype with no repeatable has no id-source leaf, so the
    // key appears nowhere in its projection — inert, never an error.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema adr --format json`");
    let adr = stdout_of(&out);
    assert!(
        !adr.contains("write-key"),
        "a doctype with no repeatable advertises no payload key; got:\n{adr}",
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
    assert_eq!(value["contract-version"], 7, "the contract is versioned");
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

/// A `ref` field's **target doctype** is named on **both** arms (M50 Inc 8 / T3 —
/// `contract-version` 5 → 6; `design/doc-read-surface.md` → the pinned `doc schema`
/// shape). The projection carried an enum's legal members (`of`) since rc.5 but never
/// a ref's `to:`, so the one structural fact a ref field *has* was withheld by the
/// surface an agent is told to consult — law 2's *nothing hides*
/// (`design/surface-contract.md`).
///
/// The plain arm renders it in the vocabulary the compose seam already uses for the
/// same fact — `ref -> <to>` (`engine::compose::field_type_text`) — so the three
/// surfaces that print a ref's target do not invent three spellings. `card:` is
/// deliberately out of scope: the compose seam carries it, this projection does not.
#[test]
fn doc_schema_names_a_ref_target_in_both_arms() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // (1) json — `to` rides the ref field beside its `type`.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema adr --format json`");
    let json = stdout_of(&out);
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("the adr contract parses as json");
    let fields = value["fields"].as_array().expect("`fields` is an array");
    let supersedes = fields
        .iter()
        .find(|f| f["id"] == "supersedes")
        .expect("the adr projection carries the `supersedes` ref");
    assert_eq!(
        supersedes["to"], "adr",
        "a ref field names the doctype it references; got:\n{json}",
    );

    // (2) The omitting context — a non-ref field carries no `to` at all (absent, not
    //     null): the enum, the date, the pack-declared `code-anchor`, the stamp.
    for id in ["status", "date", "cites-code", "schema-version"] {
        let field = fields
            .iter()
            .find(|f| f["id"] == id)
            .unwrap_or_else(|| panic!("the adr projection carries `{id}`"));
        assert!(
            field.get("to").is_none(),
            "a non-ref field carries no `to` key; `{id}` got:\n{field}",
        );
    }

    // (3) plain — the same fact in the compose seam's own vocabulary.
    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "agent"],
    );
    assert_ok(&out, "`jigc doc schema adr --format agent`");
    let listing = stdout_of(&out);
    let line = field_line(&listing, "supersedes");
    assert!(
        line.contains("ref -> adr"),
        "the plain listing names a ref's target in the compose seam's vocabulary; \
         got:\n{line}",
    );
    assert!(
        field_line(&listing, "cites-code").contains("code-anchor")
            && !field_line(&listing, "cites-code").contains("->"),
        "a non-ref field's line grows no arrow; got:\n{listing}",
    );
}

/// **The loop closes: the route a wrong-typed ref write prints can answer the
/// question the rejection raised** (M50 Inc 8 / T3). `write.malformed-value` blames a
/// ref value for targeting the wrong doctype and routes at `jigc doc schema
/// <doctype>` *"to see the field's declared type"* (`engine::write::write_route`) —
/// and until this increment that read named `supersedes: ref` and stopped, so the
/// followable route could not tell the author which type was expected.
///
/// The route is **run verbatim, as emitted**: the argv is extracted from the
/// rejection's own printed route line and executed, never reconstructed here — the
/// emitted bytes are the contract (`design/surface-contract.md` → the `Route` value).
#[test]
fn doc_schema_answers_the_route_a_wrong_typed_ref_write_prints() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo_with_setup(repo.path(), home.path());

    // A task holding a staged adr — the state a wrong-typed ref write is made from.
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "record-decision",
            "decide something",
            "--format",
            "json",
        ],
    );
    assert_ok(&out, "`jigc start --workflow record-decision`");
    let composed: serde_json::Value =
        serde_json::from_str(&stdout_of(&out)).expect("composed output is json");
    let task = composed["task"]
        .as_str()
        .expect("the compose minted a task");
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Some choice",
            "--task",
            task,
        ],
    );
    assert_ok(&out, "`jigc doc create adr`");

    // The reject: a ref value naming the wrong doctype.
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "adr:some-choice#status/supersedes",
            "--value",
            "spec:other-thing",
            "--task",
            task,
        ],
    );
    assert!(
        !out.status.success(),
        "a wrong-typed ref value must be rejected; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let rejection = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        rejection.contains("write.malformed-value")
            && rejection.contains("references type \"adr\""),
        "the reject blames the value for targeting the wrong doctype; got:\n{rejection}",
    );

    // Its own route, run as emitted — the argv is read out of the printed line.
    let route = rejection
        .lines()
        .find(|line| line.trim_start().starts_with("route: `jigc "))
        .unwrap_or_else(|| panic!("the reject carries a mechanical route; got:\n{rejection}"));
    let argv_text = route
        .split('`')
        .nth(1)
        .expect("the route's argv is backtick-delimited");
    let argv: Vec<&str> = argv_text.split_whitespace().collect();
    assert_eq!(argv[0], "jigc", "the route names the jigc binary");
    let out = jigc(repo.path(), home.path(), &argv[1..]);
    assert_ok(&out, "the route the reject printed, run verbatim");
    let answer = stdout_of(&out);
    assert!(
        field_line(&answer, "supersedes").contains("ref -> adr"),
        "the route the rejection printed names the type it blamed the value for \
         missing; ran `{argv_text}`, got:\n{answer}",
    );
}

/// M52 Increment 6 / T7 — **the projection names each doctype's identity and its
/// home**, so the one surface a driver is told to read can say which addresses a
/// doctype has (settle-record → D5.4 as amended by §10; `design/doc-read-surface.md`
/// → the identity and home of a doctype).
///
/// The gap it closes: through `contract-version` 6 the projection advertised
/// `vision:<slug>#thesis` and named **no** value `<slug>` may take, so nothing on the
/// pinned surface said `vision:alpha` is not an address this doctype can have — the
/// refusal the three doors now raise had no read surface to point back at.
///
/// Four cells, one per shape the primitive distinguishes, each driven on the emitted
/// bytes of the real binary:
///
/// * **`vision`** — placement + singleton: identity `fixed` at the bare type id,
///   home `placement` at the literal **root** file, which `placement-root` never
///   re-roots (the rule's own carve-out, asserted with the knob set).
/// * **`roadmap`** — placement whose declared home carries a **leading directory
///   component**, so its path is the one the **`placement-root` knob** resolved.
/// * **`adr`** — located + `id-from`: identity `slugged` at the `<ty>:<slug>`
///   pattern, home `location` at the **`docs-root`-resolved** directory.
/// * **`commit`** — neither: home `transient`, `path` null and present (an absent key
///   would make a driver's read of it partial).
///
/// **Both root knobs are set to non-defaults** before anything is read. That is what
/// makes these home cells a test of the **cascade-resolved** schema rather than of a
/// raw `schema.location` / `placement.file` read: under the defaults every path below
/// is byte-identical to its declaration, so the assertion would pass over the exact
/// bug it exists to catch.
#[test]
fn doc_schema_names_each_doctypes_identity_and_home() {
    let repo = TempDir::new("identity-home-repo");
    let home = TempDir::new("identity-home-home");
    init_repo_embedded_with_setup(repo.path(), home.path());

    // NON-DEFAULT roots on both knobs: every home below must be the CASCADE's answer,
    // so a projection reading `schema.location` / `placement.file` raw reddens here
    // rather than shipping. Under the defaults each path equals its declaration, and
    // the assertion would pass over the bug it exists to catch.
    let out = jigc(
        repo.path(),
        home.path(),
        &["config", "set", "docs-root", "papers"],
    );
    assert_ok(&out, "`jigc config set docs-root papers`");
    let out = jigc(
        repo.path(),
        home.path(),
        &["config", "set", "placement-root", "papers"],
    );
    assert_ok(&out, "`jigc config set placement-root papers`");

    let projection = |doctype: &str| -> serde_json::Value {
        let out = jigc(
            repo.path(),
            home.path(),
            &["doc", "schema", doctype, "--format", "json"],
        );
        assert_ok(&out, "`jigc doc schema <doctype> --format json`");
        serde_json::from_str(&stdout_of(&out)).expect("the emitted contract parses as json")
    };

    let vision = projection("vision");
    assert_eq!(
        vision["contract-version"], 7,
        "the identity/home keys ride a contract-version bump, not a silent addition; \
         got:\n{vision:#}",
    );
    assert_eq!(vision["identity"]["kind"], "fixed");
    assert_eq!(
        vision["identity"]["address"], "vision",
        "a fixed-identity doctype names its ONE address, not a `<slug>` pattern; \
         got:\n{vision:#}",
    );
    assert_eq!(vision["home"]["kind"], "placement");
    assert_eq!(
        vision["home"]["path"], "VISION.md",
        "a home declared AT the repo root is never re-rooted — `placement-root` is \
         `papers` here and the ecosystem-idiomatic file stays put by derivation; \
         got:\n{vision:#}",
    );

    let roadmap = projection("roadmap");
    assert_eq!(roadmap["identity"]["kind"], "fixed");
    assert_eq!(roadmap["identity"]["address"], "roadmap");
    assert_eq!(roadmap["home"]["kind"], "placement");
    assert_eq!(
        roadmap["home"]["path"], "papers/roadmap.md",
        "a placement home carrying a LEADING DIRECTORY COMPONENT reports the path the \
         `placement-root` knob resolved — declared `docs/roadmap.md`, so a raw \
         `placement.file` read would answer that; got:\n{roadmap:#}",
    );

    let adr = projection("adr");
    assert_eq!(adr["identity"]["kind"], "slugged");
    assert_eq!(
        adr["identity"]["address"], "adr:<slug>",
        "a per-instance doctype names the pattern its addresses take; got:\n{adr:#}",
    );
    assert_eq!(adr["home"]["kind"], "location");
    assert_eq!(
        adr["home"]["path"], "papers/decisions/<slug>.md",
        "the located home is the CASCADE-RESOLVED path — `docs-root` is set to \
         `papers` above, so a raw `schema.location` read would answer \
         `decisions/<slug>.md`; got:\n{adr:#}",
    );

    let commit = projection("commit");
    assert_eq!(commit["home"]["kind"], "transient");
    assert!(
        commit["home"]
            .as_object()
            .is_some_and(|home| home.get("path").is_some_and(serde_json::Value::is_null)),
        "a transient doctype emits `path: null` — present and null, never absent, so \
         a driver's read of the key is total; got:\n{commit:#}",
    );
}

/// M52 Increment 6 / T7 — **the two pinned read surfaces agree on the type of the one
/// compound field they both carry** (baseline-contracts LD-2; settle D6.5).
///
/// Driven at HEAD before this task: `jigc doc schema milestone-record --format json`
/// reported `"type": "string"` for `base` while `jigc doc show
/// milestone-record:<slug>#meta/base --format json` returned the **object**
/// `{"sha": …, "short": …}` — so a driver type-checking the content read against the
/// type read was told the wrong shape on the one compound field the 1.0 pin carries.
///
/// The assertion is a **cross-surface equality**, not two hard-coded key lists: the
/// member set is read off the projection and compared with the member set `doc show`
/// actually emits, so the two cannot drift apart in silence.
#[test]
fn doc_schema_projects_the_compound_field_as_the_shape_doc_show_returns() {
    let repo = TempDir::new("compound-repo");
    let home = TempDir::new("compound-home");
    init_repo_embedded_with_setup(repo.path(), home.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert_ok(&out, "`jigc milestone create`");

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "milestone-record", "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema milestone-record --format json`");
    let schema: serde_json::Value =
        serde_json::from_str(&stdout_of(&out)).expect("the schema projection parses");
    let base = schema["fields"]
        .as_array()
        .expect("`fields` is an array")
        .iter()
        .find(|field| field["id"] == "base")
        .unwrap_or_else(|| panic!("the record declares a `base` field; got:\n{schema:#}"));
    let declared = base["json-shape"]
        .as_object()
        .unwrap_or_else(|| panic!("`base` names its json shape; got:\n{base:#}"));
    assert_eq!(
        declared.get("sha").and_then(serde_json::Value::as_str),
        Some("string"),
        "the compound's members carry their own json type; got:\n{base:#}",
    );
    assert_eq!(
        declared.get("short").and_then(serde_json::Value::as_str),
        Some("string"),
    );

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "milestone-record:cache-rework#meta/base",
            "--format",
            "json",
        ],
    );
    assert_ok(&out, "`jigc doc show milestone-record:…#meta/base`");
    let served: serde_json::Value =
        serde_json::from_str(&stdout_of(&out)).expect("the content read parses");
    let served = served
        .as_object()
        .unwrap_or_else(|| panic!("the compound leaf serves an object; got:\n{served:#}"));

    let mut advertised: Vec<&str> = declared.keys().map(String::as_str).collect();
    advertised.sort_unstable();
    let mut emitted: Vec<&str> = served.keys().map(String::as_str).collect();
    emitted.sort_unstable();
    assert_eq!(
        advertised, emitted,
        "the type surface's declared member set must equal the content surface's \
         emitted one — a driver type-checking `doc show` against `doc schema` reads \
         one fact, not two",
    );
}
