//! M47 Increment 6, T1 — **the write-verb × miss-shape axis, one row per cell.**
//!
//! The class: *which miss a write reject claims to be*. An **item-id miss** — the
//! addressed item was never minted — is not a shape question: the schema names the
//! *shape*, never the corpus's real item ids, so `jigc doc schema <doctype>` is a dead
//! end there. Before this suite the diagnosis was decided per call site, and four of the
//! eight cells got it wrong (`set-field --value`, `set-field --unset` at an undeclared
//! field, `retitle-item`, and nested `add-item` under an absent parent), while the two
//! already-correct cells (`set-slot`, `remove-item`) had no axis-level fence keeping the
//! rest with them (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8 — sweep the axis,
//! not the instance; `baseline.md` §4e).
//!
//! The axis is enumerated as **data**, one row per cell, so a write verb added later is
//! covered by adding a row rather than by remembering this file exists:
//!
//!   * the **item-id misses** → `write.not-present`
//!     (`set-slot` · `remove-item` · `set-field --value` · `set-field --unset` at a
//!     declared field · `set-field --unset` at an *undeclared* field — the item miss
//!     outranks the field question — · `retitle-item` · nested `add-item` under an
//!     absent parent, the eighth cell the baseline census under-counted · and, added
//!     with the route column, `retitle-item` at an absent **nested** item under a real
//!     parent, the deepest strip);
//!   * the **undeclared-section** miss → `write.unknown-section`;
//!   * a **genuine declared-shape defect** (`add-item` into a non-repeatable section) →
//!     `write.wrong-shape`, which the flip deliberately leaves standing.
//!
//! Every cell asserts the same three things through the real binary: the write **blocks
//! non-zero**, the emitted `--format json` finding carries the expected `code`, and the
//! **staged bytes are byte-identical** to before the call (a rejected write persists
//! nothing).
//!
//! **T2 adds the route column** — the same axis, one hop further: naming the miss
//! correctly is worthless if the route the agent is handed is still the schema dead end.
//! Every item-id-miss cell must emit
//! `jigc doc show <type>:<slug>#<top-section> --task <id>` carrying the **real** slug and
//! the **resolved** task id (never the `<address>` / `<task-id>` placeholders of the
//! engine's defensive fallback), and the emitted argv is **run verbatim** — it must exit 0
//! and print the section's live item ids, so the recovery is followable and not merely
//! well-worded (`design/surface-contract.md` law 2: nothing hides;
//! `design/validation.md` → the `write.*` route split). A **nested** cell strips to the
//! top *showable* section — the M44 N2 pin, re-asserted here at the four verbs the CLI
//! enrichment did not reach (`set-field --value` / `--unset` · `retitle-item` ·
//! `add-item`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-miss-shape-axis-{tag}-{}-{:?}",
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

/// The fixture `changelog` schema — a two-level repeatable (so a nested `add-item`
/// under an absent parent has a home), carrying a `summary` slot and an **optional**
/// `link` field (the only unset-eligible shape: a required or defaulted field is
/// refused by the eligibility guard before the item is ever adjudicated), plus a
/// **non-repeatable** `overview` section so the genuine shape question has a target.
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

/// The held provisioning: the temp guards must stay alive (dropping them removes the
/// repo), so they are returned to the caller.
struct Fixture {
    repo: TempDir,
    home: TempDir,
    pack: TempDir,
    slug: String,
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

    /// The staged working copy of the fixture changelog — the bytes a rejected write
    /// must leave untouched.
    fn staged(&self) -> PathBuf {
        self.repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join(TASK_ID)
            .join("docs")
            .join(format!("changelog:{}.md", self.slug))
    }

    /// Run a route's emitted command **verbatim** (split on whitespace, the leading
    /// `jigc` dropped), returning its trimmed stdout — the followability proof.
    fn run_route(&self, cmd: &str, what: &str) -> String {
        let mut parts = cmd.split_whitespace();
        assert_eq!(
            parts.next(),
            Some("jigc"),
            "`{what}`: the route leads `jigc`"
        );
        let args: Vec<&str> = parts.collect();
        ok_stdout(self.run(&args, None), what)
    }
}

/// The task id `jigc start "log the release"` mints — the id the enriched route must
/// carry (never the `<task-id>` placeholder).
const TASK_ID: &str = "log-the-release";

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked<'a>(route: &'a str, what: &str) -> &'a str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("`{what}`: the route carries a backticked command; got: {route}"))
}

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying stderr.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// Provision a `changelog` task carrying release `1-3-0` with a nested `Added`
/// change-group, returning the live fixture.
fn provision() -> Fixture {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());

    let mut fx = Fixture {
        repo,
        home,
        pack,
        slug: String::new(),
    };

    ok_stdout(fx.run(&["setup"], None), "jigc setup");
    ok_stdout(
        fx.run(
            &["start", "--workflow", "log-change", "log the release"],
            None,
        ),
        "jigc start --workflow log-change",
    );

    let created = ok_stdout(
        fx.run(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    fx.slug = created
        .strip_prefix("changelog:")
        .expect("created address is changelog:<slug>")
        .to_owned();

    let release = ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("changelog:{}#releases", fx.slug),
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

/// One cell of the write-verb × miss-shape matrix.
struct Cell {
    /// What the cell is, for the assertion messages.
    what: &'static str,
    /// The `jigc` argv after the binary, with `{addr}` standing for the doc address.
    args: &'static [&'static str],
    /// Optional stdin payload (the `--from-file -` cells).
    stdin: Option<&'static [u8]>,
    /// The finding `code` the reject must carry.
    code: &'static str,
    /// For an **item-id miss**: the top showable section the emitted route must show
    /// (`jigc doc show <type>:<slug>#<section> --task <id>`, run verbatim here). `None`
    /// for the two non-item-miss cells, whose route stays the schema's — they *are*
    /// questions the schema answers.
    route_section: Option<&'static str>,
}

/// **The axis.** Six item-id misses, the undeclared-section miss, and the one genuine
/// declared-shape defect the flip leaves standing.
const CELLS: &[Cell] = &[
    Cell {
        what: "set-slot at a nonexistent item",
        args: &[
            "doc",
            "set-slot",
            "{addr}#releases/9-9-9/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        what: "remove-item at a nonexistent item",
        args: &["doc", "remove-item", "{addr}#releases/9-9-9"],
        stdin: None,
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        what: "set-field --value at a nonexistent item",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/9-9-9/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, declared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/link", "--unset"],
        stdin: None,
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, undeclared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/bogus", "--unset"],
        stdin: None,
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        what: "retitle-item at a nonexistent item",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/9-9-9",
            "--title",
            "9.9.9",
        ],
        stdin: None,
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        what: "nested add-item under an absent parent item",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        // The **deep** strip at one of the newly-enriched verbs: a real parent release
        // (`1-3-0`), an absent nested change-group — the route must still strip past two
        // hops to the top *showable* section (the N2 pin, re-asserted at `retitle-item`).
        what: "retitle-item at a nonexistent nested item under a real parent",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/changes/no-such-group",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route_section: Some("releases"),
    },
    Cell {
        what: "add-item into an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route_section: None,
    },
    Cell {
        what: "add-item into a non-repeatable section (the genuine shape question)",
        args: &["doc", "add-item", "{addr}#overview", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route_section: None,
    },
];

#[test]
fn every_write_miss_names_its_own_miss_and_routes_the_recovery() {
    let fx = provision();
    let addr = format!("changelog:{}", fx.slug);
    let staged = fx.staged();
    let before = fs::read_to_string(&staged).expect("read the staged changelog");
    // Every cell is adjudicated, then the whole axis is reported at once — a per-cell
    // panic would hide the rest of the matrix behind the first broken row.
    let mut broken: Vec<String> = Vec::new();

    for cell in CELLS {
        let args: Vec<String> = cell
            .args
            .iter()
            .map(|arg| arg.replace("{addr}", &addr))
            .chain(["--format".to_owned(), "json".to_owned()])
            .collect();
        let out = fx.run(
            &args.iter().map(String::as_str).collect::<Vec<&str>>(),
            cell.stdin,
        );

        assert!(
            !out.status.success(),
            "`{}` must block (non-zero exit); stdout:\n{}\nstderr:\n{}",
            cell.what,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        let report: serde_json::Value = serde_json::from_str(stderr.trim())
            .unwrap_or_else(|e| panic!("`{}` stderr is JSON: {e}; got:\n{stderr}", cell.what));
        let got = report["findings"][0]["code"].as_str().unwrap_or("<absent>");
        if got != cell.code {
            broken.push(format!(
                "  {}: expected `{}`, got `{}`",
                cell.what, cell.code, got
            ));
        }

        // **The route column.** An item-id miss must hand back the containing section's
        // `doc show`, with the doc's real slug and the resolved task id substituted — the
        // engine's `<address>` / `<task-id>` fallback reaching an agent is the dead end
        // this axis exists to close.
        if let Some(section) = cell.route_section {
            let route = report["findings"][0]["route"]
                .as_str()
                .unwrap_or_else(|| panic!("`{}` carries a route; got:\n{stderr}", cell.what));
            let cmd = backticked(route, cell.what);
            let expected = format!("jigc doc show {addr}#{section} --task {TASK_ID}");
            if cmd != expected {
                broken.push(format!(
                    "  {}: expected route `{}`, got `{}`",
                    cell.what, expected, cmd
                ));
            } else {
                // Followability: the emitted argv, run **verbatim**, must exit 0 and print
                // the section's live item ids — the address the agent should have used.
                let shown = fx.run_route(cmd, cell.what);
                assert!(
                    shown.contains("1-3-0"),
                    "`{}`: the emitted route reveals the section's live item ids; got:\n{shown}",
                    cell.what,
                );
            }
        }

        let after = fs::read_to_string(&staged).expect("read the staged changelog");
        assert_eq!(
            after, before,
            "`{}` persisted nothing — the staged bytes are unchanged",
            cell.what,
        );
    }

    assert!(
        broken.is_empty(),
        "{} defect(s) across {} cells (the miss named, or the route handed back):\n{}",
        broken.len(),
        CELLS.len(),
        broken.join("\n"),
    );
}
