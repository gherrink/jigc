//! M47 Increment 6, T4 — **no write lands at an undeclared address** (the field-leaf
//! column of the undeclared-address table).
//!
//! The class: *a write addressed at a leaf the schema does not declare*. The section-leaf
//! cell has always been guarded — `set_field_validated` resolves the declared field first
//! and rejects an undeclared key with `write.unknown-field` before any bytes move — and so
//! has the `set-field --unset` sibling on items (`unset_item_field_validated`). The
//! **item-leaf** `--value` cells were not: `set_item_field_or_insert` /
//! `set_nested_item_field_or_insert` short-circuited the schema lookup on `None` and
//! **wrote the bullet anyway**, exit 0 with a positive ack, after which every read of the
//! doc is `store.unparseable` with a *human* route the adapter rule forbids following —
//! only `jigc task discard` (the whole task) recovers (`baseline.md` §3a, N3;
//! `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9). The batch `doc author` inherited
//! the same hole through the shared `apply_field_target` seam.
//!
//! N3's aftermath also broke a **declared exemption**: `conformance.*` findings are
//! route-exempt because *"the located message is the repair — no CLI verb repairs a
//! hand-broken byte"*, a rationale that assumed out-of-band bytes. N3 made them arrive
//! through jigc's **own** write verb at exit 0, producing a blocking, route-less finding on
//! the tool's own success path. Guarding the write **restores the exemption's original
//! scope** rather than overturning it (`design/validation.md` → the `conformance.*` route
//! exemption).
//!
//! The column is enumerated as **data**, one row per cell, so a write verb added later is
//! covered by adding a row rather than by remembering this file exists. Every row asserts
//! the same four things through the real binary:
//!
//!   1. the write **blocks non-zero**;
//!   2. the emitted `--format json` finding carries `write.unknown-field` — the *same*
//!      code (and route) the `--unset` sibling already emits, so no new contract member is
//!      minted — at a **located target**: the finding's stable `(code, target)` key names
//!      the full write address, never the degenerate `null`
//!      (`design/command-output-contract.md` → The stable finding key);
//!   3. the staged bytes are **unchanged** — present-and-identical, or absent-and-still-
//!      absent for the batch cells, whose whole-doc rollback removes the file it created;
//!   4. the containing section is **still readable** through `jigc doc show` — the
//!      unreadable-doc aftermath is what made this a data-loss class rather than a
//!      papercut. (For the batch cells the doc never existed, so readability is proven the
//!      only way it can be: the rejected batch left the doc **authorable**, and the
//!      corrected payload lands and reads back.)
//!
//! **Why the offending leaf is last in every batch payload.** A `set:` map is applied in
//! sorted key order and a *following* leaf's own re-parse would catch the corruption
//! incidentally — a green that proves nothing about the guard. Each payload therefore
//! places the undeclared key alone on the **final** item, so the reject can only be the
//! guard's own. (Live at the pre-fix HEAD: both batch cells exit 0 and leave the doc
//! unreadable.)
//!
//! **The slot-leaf column of the table lands at T5** — named here so the split is tracked
//! to a landing, never left to "a later task".

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-undeclared-addr-{tag}-{}-{:?}",
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

/// Recursively copy `from` into `to` (both directories).
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create dest dir");
    for entry in fs::read_dir(from).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            copy_tree(&src, &dst);
        } else {
            fs::copy(&src, &dst).expect("copy file");
        }
    }
}

/// The fixture `changelog` schema — a two-level repeatable (so both an item leaf and a
/// **nested** item leaf have a home) carrying a `summary` slot and an **optional** `link`
/// field (the declared key the corrected batch payload writes instead), plus a
/// non-repeatable `overview` section so the already-guarded **section**-leaf control cell
/// has a target.
const CHANGELOG_SCHEMA: &str = "\
type: changelog
id-from: title
sections:
  - id: overview
    slot: { hint: \"What this changelog covers.\", optional: true }
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: link, type: string, optional: true }
        - { id: summary, slot: { hint: \"One-line release summary.\" } }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";

/// Build a throwaway pack: the embedded dev pack tree copied to a temp dir, plus the
/// fixture `changelog` schema and a `log-change` workflow whose create-gate admits it.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
    copy_tree(&dev_pack, pack.path());
    // The fixture ships a deliberately divergent `changelog` shape, so it is NOT the
    // frozen dev pack — drop the copied freeze manifest (the pack-load gate would
    // otherwise block the un-bumped shape change). A manifest-less pack is unchecked.
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the copied freeze manifest");

    fs::write(
        pack.path().join("schemas").join("changelog.yaml"),
        CHANGELOG_SCHEMA,
    )
    .expect("write changelog schema");

    fs::write(
        pack.path().join("workflows").join("log-change.yaml"),
        "\
---
when: record a release's changes in the changelog
description: Author the changelog for a release.
usage: a release's changes need recording in the changelog.
creates-task: true
allows-create: [{type: changelog, as: changelog}]
---
{{ include: step:finalize }}
",
    )
    .expect("write log-change workflow");

    pack
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
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

/// The task id `jigc start "log the release"` mints.
const TASK_ID: &str = "log-the-release";

/// The doc address every row writes at — the fixture's title `Changelog` slugs to
/// `changelog`, deterministically, whether the doc is minted by `doc create` (the
/// per-leaf rows) or by the batch payload's own `title:` (the `doc author` rows).
const ADDR: &str = "changelog:changelog";

/// The held provisioning: the temp guards must stay alive (dropping them removes the
/// repo), so they are returned to the caller.
struct Fixture {
    repo: TempDir,
    home: TempDir,
    pack: TempDir,
}

impl Fixture {
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
        use std::io::Write;
        use std::process::Stdio;
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", self.pack.path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if stdin.is_some() {
            command.stdin(Stdio::piped());
        }
        let mut child = command.spawn().expect("spawn the jigc binary");
        if let Some(bytes) = stdin {
            child
                .stdin
                .take()
                .expect("stdin piped")
                .write_all(bytes)
                .expect("write stdin");
        }
        child.wait_with_output().expect("wait for jigc")
    }

    /// The staged working copy of the fixture changelog. Read as `Option<String>` —
    /// **absent** is a state a rejected write must preserve exactly as faithfully as a
    /// byte string is (the batch verb's rollback removes the file it created).
    fn staged(&self) -> Option<String> {
        fs::read_to_string(
            self.repo
                .path()
                .join(".jigc")
                .join("tasks")
                .join(TASK_ID)
                .join("docs")
                .join("changelog:changelog.md"),
        )
        .ok()
    }
}

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying stderr.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// Bring a repo to a live `log-change` task. With `with_doc`, mint the changelog plus
/// release `1-3-0` and its nested `Added` change-group (the per-leaf rows write into an
/// existing doc); without it, leave the task doc-less (the batch rows mint their own).
fn provision(with_doc: bool) -> Fixture {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());

    let fx = Fixture { repo, home, pack };
    ok_stdout(fx.run(&["setup"], None), "jigc setup");
    ok_stdout(
        fx.run(
            &["start", "--workflow", "log-change", "log the release"],
            None,
        ),
        "jigc start --workflow log-change",
    );
    if !with_doc {
        return fx;
    }

    let created = ok_stdout(
        fx.run(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    assert_eq!(
        created, ADDR,
        "the fixture title mints the expected address"
    );
    let release = ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("{ADDR}#releases"),
                "--title",
                "1-3-0",
            ],
            None,
        ),
        "add-item release",
    );
    ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
            ],
            None,
        ),
        "add-item nested change-group",
    );
    fx
}

/// The batch payload whose **last** leaf writes an undeclared field on a **top-level**
/// release item, and its corrected twin (the declared optional `link`).
const AUTHOR_TOP_LEVEL: &str = "\
title: Changelog
sections:
  - id: releases
    items:
      - title: 1-3-0
        set:
          summary: \"<<A release.>>\"
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: \"<<- A note.>>\"
      - title: 2-0-0
        set:
          bogus: x
";
const AUTHOR_TOP_LEVEL_FIXED: &str = "\
title: Changelog
sections:
  - id: releases
    items:
      - title: 1-3-0
        set:
          summary: \"<<A release.>>\"
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: \"<<- A note.>>\"
      - title: 2-0-0
        set:
          link: https://example.com/2-0-0
";

/// The batch payload whose **last** leaf writes an undeclared field on a **nested**
/// change-group item, and its corrected twin (the declared `notes` slot).
const AUTHOR_NESTED: &str = "\
title: Changelog
sections:
  - id: releases
    items:
      - title: 1-3-0
        set:
          summary: \"<<A release.>>\"
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: \"<<- A note.>>\"
              - title: Fixed
                set:
                  bogus: x
";
const AUTHOR_NESTED_FIXED: &str = "\
title: Changelog
sections:
  - id: releases
    items:
      - title: 1-3-0
        set:
          summary: \"<<A release.>>\"
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: \"<<- A note.>>\"
              - title: Fixed
                set:
                  notes: \"<<- A fix.>>\"
";

/// How a row addresses its undeclared leaf.
enum Write {
    /// The per-leaf verb, over a doc the fixture already minted: the `set-field` address
    /// tail (appended to [`ADDR`]) and the value.
    SetField {
        fragment: &'static str,
        value: &'static str,
    },
    /// The batch verb, over a doc-less task: the whole-doc payload carrying the undeclared
    /// leaf **last**, plus the corrected payload the reject must leave landable.
    Author {
        payload: &'static str,
        corrected: &'static str,
    },
}

/// One cell of the undeclared-address table's **field-leaf** column.
struct Row {
    /// What the cell is, for the assertion messages.
    what: &'static str,
    /// The write under test.
    write: Write,
    /// The address the finding's stable `(code, target)` key must name.
    target: &'static str,
    /// The containing section that must still read back through `jigc doc show`.
    show_section: &'static str,
}

/// **The column.** Two `set-field` item-leaf cells, two `doc author` item-leaf cells, and
/// the section-leaf control that was already correct — it stays in the table so the
/// already-guarded cell keeps a live witness rather than being assumed.
const ROWS: &[Row] = &[
    Row {
        what: "set-field at an undeclared field of a top-level item",
        write: Write::SetField {
            fragment: "#releases/1-3-0/bogus",
            value: "x",
        },
        target: "changelog:changelog#releases/1-3-0/bogus",
        show_section: "#releases",
    },
    Row {
        what: "set-field at an undeclared field of a nested item",
        write: Write::SetField {
            fragment: "#releases/1-3-0/changes/added/bogus",
            value: "x",
        },
        target: "changelog:changelog#releases/1-3-0/changes/added/bogus",
        show_section: "#releases",
    },
    Row {
        what: "set-field at an undeclared field of a section (the already-guarded control)",
        write: Write::SetField {
            fragment: "#overview/bogus",
            value: "x",
        },
        target: "changelog:changelog#overview/bogus",
        show_section: "#overview",
    },
    Row {
        what: "doc author at an undeclared field of a top-level item",
        write: Write::Author {
            payload: AUTHOR_TOP_LEVEL,
            corrected: AUTHOR_TOP_LEVEL_FIXED,
        },
        target: "changelog:changelog#releases/2-0-0/bogus",
        show_section: "#releases",
    },
    Row {
        what: "doc author at an undeclared field of a nested item",
        write: Write::Author {
            payload: AUTHOR_NESTED,
            corrected: AUTHOR_NESTED_FIXED,
        },
        target: "changelog:changelog#releases/1-3-0/changes/fixed/bogus",
        show_section: "#releases",
    },
];

#[test]
fn no_write_lands_at_an_undeclared_field_leaf() {
    // Every row is adjudicated, then the whole column is reported at once — a per-row
    // panic would hide the rest of the table behind the first broken cell.
    let mut broken: Vec<String> = Vec::new();

    for row in ROWS {
        let batch = matches!(row.write, Write::Author { .. });
        let fx = provision(!batch);
        let before = fx.staged();

        let out = match &row.write {
            Write::SetField { fragment, value } => fx.run(
                &[
                    "doc",
                    "set-field",
                    &format!("{ADDR}{fragment}"),
                    "--value",
                    value,
                    "--format",
                    "json",
                ],
                None,
            ),
            Write::Author { payload, .. } => fx.run(
                &[
                    "doc",
                    "author",
                    "changelog",
                    "--from-file",
                    "-",
                    "--format",
                    "json",
                ],
                Some(payload.as_bytes()),
            ),
        };

        // 1. The write blocks non-zero.
        if out.status.success() {
            broken.push(format!(
                "  {}: exited 0 (the undeclared address was written); stdout:\n{}",
                row.what,
                String::from_utf8_lossy(&out.stdout),
            ));
        } else {
            // 2. The code + the located target.
            let stderr = String::from_utf8_lossy(&out.stderr);
            let report: serde_json::Value = serde_json::from_str(stderr.trim())
                .unwrap_or_else(|e| panic!("`{}` stderr is JSON: {e}; got:\n{stderr}", row.what));
            let finding = &report["findings"][0];
            let code = finding["code"].as_str().unwrap_or("<absent>");
            if code != "write.unknown-field" {
                broken.push(format!(
                    "  {}: expected `write.unknown-field`, got `{code}`",
                    row.what,
                ));
            }
            let target = finding["key"]["target"].as_str().unwrap_or("<absent>");
            if target != row.target {
                broken.push(format!(
                    "  {}: expected the located target `{}`, got `{target}`",
                    row.what, row.target,
                ));
            }
        }

        // 3. The staged bytes are unchanged — including "still absent". A row whose bytes
        // *did* move has already wrecked its own doc, so its readability cell is
        // meaningless: record it and move to the next cell rather than panic the column.
        let after = fx.staged();
        if after != before {
            broken.push(format!(
                "  {}: persisted a write — the staged state moved from {:?} to {:?}",
                row.what, before, after,
            ));
            continue;
        }

        // 4. The containing section still reads back. The batch cells never had a doc, so
        // readability is proven the only way it can be: the reject left the doc
        // **authorable**, and the corrected payload lands and reads back.
        if let Write::Author { corrected, .. } = &row.write {
            ok_stdout(
                fx.run(
                    &[
                        "doc",
                        "author",
                        "changelog",
                        "--from-file",
                        "-",
                        "--format",
                        "json",
                    ],
                    Some(corrected.as_bytes()),
                ),
                "the corrected batch payload",
            );
        }
        let shown = ok_stdout(
            fx.run(
                &[
                    "doc",
                    "show",
                    &format!("{ADDR}{}", row.show_section),
                    "--task",
                    TASK_ID,
                ],
                None,
            ),
            &format!("`doc show` after `{}`", row.what),
        );
        if row.show_section == "#releases" {
            assert!(
                shown.contains("1-3-0"),
                "`{}`: the containing section reads back with its live items; got:\n{shown}",
                row.what,
            );
        }
    }

    assert!(
        broken.is_empty(),
        "{} defect(s) across {} cells of the undeclared-address field-leaf column:\n{}",
        broken.len(),
        ROWS.len(),
        broken.join("\n"),
    );
}
