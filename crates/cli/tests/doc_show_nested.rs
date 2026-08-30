//! M40 Increment 6, T3 — nested repeatable content joins the pinned `doc show
//! --format json` contract (`design/doc-read-surface.md` → Nested repeatables join
//! the pin), proven on the EMITTED bytes of the real binary over the shipped
//! changelog doctype (the only shipped nested doctype — the witness).
//!
//! The corpus is authored THROUGH the binary (flow-24 shape: cold-create the
//! singleton, `add-item` a release + two nested change-groups, finalize), and every
//! downstream `doc show` address is driven from the EMITTED `add-item` address
//! verbatim — never a reconstructed equivalent. What this pins:
//!
//!   1. the whole-doc json carries the release item WITH its `changes` array of
//!      recursive item objects (`{"category":…,"notes":…}`) — the additive key,
//!      declared-nested-block-only;
//!   2. the section-qualified write-grammar address resolves on show:
//!      `#releases/<id>/changes` → the nested ARRAY, `…/changes/<gid>` → the nested
//!      item OBJECT, `…/<gid>/<leaf>` → the leaf (slot prose / the `id-from`
//!      heading) — the two new contract depths;
//!   3. the segment-less exit-0 wrong-node degrade is dead: the deep address that
//!      previously returned the ENCLOSING release item returns the addressed node
//!      (honest error or correct answer, never wrong-node-exit-0).
//!
//! The `date` field is `set: on-create` (non-deterministic), so the goldens
//! interpolate the value read back through the already-pinned leaf slice — every
//! other byte is matched verbatim. No external test crates.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-show-nested-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both streams.
fn ok_stdout(out: Output, what: &str) -> String {
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

/// Fill the engine-native `commit` doc for `task` so finalize has a renderable message.
fn fill_commit(repo: &Path, home: &Path, pack: &Path, task: &str) {
    for (key, value) in [("type", "docs"), ("scope", "changelog")] {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "set-field",
                    &format!("commit:{task}#{key}"),
                    "--value",
                    value,
                ],
                None,
            ),
            &format!("set-field commit:{task}#{key}"),
        );
    }
    for (key, prose) in [
        ("summary", b"cut 1.0.0\n" as &[u8]),
        ("body", b"A changelog change.\n"),
    ] {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "set-slot",
                    &format!("commit:{task}#{key}"),
                    "--from-file",
                    "-",
                ],
                Some(prose),
            ),
            &format!("set-slot commit:{task}#{key}"),
        );
    }
}

/// Author one nested change-group under the release's EMITTED address, driving the
/// emitted section-qualified nested address verbatim; returns that address.
fn author_group(
    repo: &Path,
    home: &Path,
    pack: &Path,
    parent: &str,
    category: &str,
    notes: &[u8],
) -> String {
    let group = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                &format!("{parent}/changes"),
                "--title",
                category,
            ],
            None,
        ),
        &format!("add-item {parent}/changes ({category})"),
    );
    assert_eq!(
        group,
        format!("{parent}/changes/{category}"),
        "the minted nested-group address is section-qualified",
    );
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
            ],
            Some(notes),
        ),
        &format!("set-slot {group}/notes"),
    );
    group
}

/// Author + finalize a changelog with one release (`1.0.0`, a `link`, two nested
/// change-groups: `added` then `fixed`) through the real binary; returns the EMITTED
/// release item address (`changelog:changelog#releases/<id>`).
fn commit_changelog(repo: &Path, home: &Path, pack: &Path) -> String {
    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["start", "--workflow", "record-change", "cut 1.0.0"],
            None,
        ),
        "jigc start --workflow record-change",
    );
    let task = "cut-1-0-0";
    let created = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    assert_eq!(created, "changelog:changelog");

    let release = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.0.0",
            ],
            None,
        ),
        "add-item release 1.0.0",
    );
    assert!(
        release.starts_with("changelog:changelog#releases/"),
        "the release address is under #releases/; got {release}",
    );
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("{release}/link"),
                "--value",
                "https://example.com/compare/0.9.0...1.0.0",
            ],
            None,
        ),
        "set-field release link",
    );
    author_group(
        repo,
        home,
        pack,
        &release,
        "added",
        b"- OAuth device-code flow\n",
    );
    author_group(
        repo,
        home,
        pack,
        &release,
        "fixed",
        b"- session fixation on logout\n",
    );
    fill_commit(repo, home, pack, task);
    let fin = run_jigc(repo, home, pack, &["task", "finalize", task], None);
    assert!(
        fin.status.success(),
        "finalize must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&fin.stdout),
        String::from_utf8_lossy(&fin.stderr),
    );
    release
}

// ---- the pinned nested json goldens (`<DATE>` interpolated — `set: on-create`) ----
//
// The top-level `schema-version` is M49's additive key — the doc's own stamp as a json
// NUMBER (`changelog` is manifest schema-version 2), beside the untouched `fields` entry
// that stays the string (design/doc-read-surface.md → One name, two json types).

const WHOLE_DOC_JSON: &str = r#"{
  "fields": {
    "schema-version": "2"
  },
  "item-count": 1,
  "schema-version": 2,
  "sections": {
    "releases": [
      {
        "changes": [
          {
            "category": "added",
            "id": "added",
            "notes": "- OAuth device-code flow"
          },
          {
            "category": "fixed",
            "id": "fixed",
            "notes": "- session fixation on logout"
          }
        ],
        "date": "<DATE>",
        "id": "1-0-0",
        "link": "https://example.com/compare/0.9.0...1.0.0",
        "title": "1.0.0"
      }
    ],
    "unreleased-changes": []
  },
  "slug": "changelog",
  "type": "changelog"
}"#;

const CHANGES_ARRAY_JSON: &str = r#"[
  {
    "category": "added",
    "id": "added",
    "notes": "- OAuth device-code flow"
  },
  {
    "category": "fixed",
    "id": "fixed",
    "notes": "- session fixation on logout"
  }
]"#;

const CHANGE_GROUP_JSON: &str = r#"{
  "category": "added",
  "id": "added",
  "notes": "- OAuth device-code flow"
}"#;

/// `doc show <release> --format json` — the release item object, carrying the minted
/// `id` (`1-0-0`), the key that closes the contract under its own address grammar (M42).
/// A frozen id is READ here, never re-derived: it can diverge from the heading (a
/// retitle-item, the mint-time word cap, a `-2` collision suffix — and, permanently,
/// the generation-1 corpus where this same `1.0.0` heading carries the id `100`).
const RELEASE_ITEM_JSON: &str = r#"{
  "changes": [
    {
      "category": "added",
      "id": "added",
      "notes": "- OAuth device-code flow"
    },
    {
      "category": "fixed",
      "id": "fixed",
      "notes": "- session fixation on logout"
    }
  ],
  "date": "<DATE>",
  "id": "1-0-0",
  "link": "https://example.com/compare/0.9.0...1.0.0",
  "title": "1.0.0"
}"#;

/// The nested changelog corpus reads back through `doc show --format json` under the
/// pinned recursive shape: the whole-doc release item carries its `changes` array of
/// recursive objects; the section-qualified write address resolves at the two new
/// depths (nested item object, nested leaf); the deep address that previously
/// returned the ENCLOSING item with exit 0 returns the addressed leaf.
#[test]
fn doc_show_json_serves_nested_changelog_content_under_the_pinned_shape() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    let (repo, home, pack) = (repo.path(), home.path(), pack.as_path());
    let release = commit_changelog(repo, home, pack);

    // The `date` field is stamped on-create (non-deterministic) — read it back through
    // the already-pinned flat leaf slice and interpolate it into the goldens.
    let date = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "show", &format!("{release}/date")],
            None,
        ),
        "doc show <release>/date (plain leaf)",
    );
    assert!(
        !date.is_empty() && date.chars().all(|c| c.is_ascii_digit() || c == '-'),
        "the date leaf reads back as the bare date value; got {date:?}",
    );

    // (1) The whole-doc json carries the release WITH its recursive `changes` array.
    let json = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "show", "changelog:changelog", "--format", "json"],
            None,
        ),
        "doc show changelog:changelog --format json",
    );
    assert_eq!(
        json,
        WHOLE_DOC_JSON.replace("<DATE>", &date),
        "the changelog whole-doc json is the pinned recursive shape",
    );

    // (2) `#releases/<id>/changes` — the canonical write address → the nested ARRAY.
    let json = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "show",
                &format!("{release}/changes"),
                "--format",
                "json",
            ],
            None,
        ),
        "doc show <release>/changes --format json",
    );
    assert_eq!(
        json, CHANGES_ARRAY_JSON,
        "the nested-section slice is the array of recursive item objects",
    );

    // (3) `…/changes/<gid>` — the nested item OBJECT (driving the EMITTED add-item
    //     address verbatim).
    let group = format!("{release}/changes/added");
    let json = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "show", &group, "--format", "json"],
            None,
        ),
        "doc show <group> --format json",
    );
    assert_eq!(
        json, CHANGE_GROUP_JSON,
        "the nested item slice is its object"
    );

    // (4) `…/changes/<gid>/notes` — the nested LEAF. This is the address that
    //     previously degraded to the ENCLOSING release item with exit 0 (the
    //     wrong-node defect): it must now be the leaf's json string, nothing else.
    let json = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "show", &format!("{group}/notes"), "--format", "json"],
            None,
        ),
        "doc show <group>/notes --format json",
    );
    assert_eq!(
        json, r#""- OAuth device-code flow""#,
        "the nested slot leaf is its trimmed prose as a json string (never the enclosing item)",
    );

    // (5) `…/changes/<gid>/category` — the nested block's `id-from` leaf → the item's
    //     heading (its stable id-source), at nested depth.
    let json = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "show",
                &format!("{group}/category"),
                "--format",
                "json",
            ],
            None,
        ),
        "doc show <group>/category --format json",
    );
    assert_eq!(
        json, r#""added""#,
        "the nested id-from leaf is the item's heading as a json string",
    );

    // (6) The item `id` closes the contract under its own address grammar (M42): the
    //     EMITTED add-item address carries the minted id, and the item object read back
    //     at that address carries the SAME id under `id`. A driver must READ the id, never
    //     re-derive it: the mint rule is versioned (generations 2 and 3 map the dots →
    //     `1-0-0`; generation 1 stripped them → `100`, and those frozen ids stay), so a
    //     driver slugifying the title guesses at whichever generation minted the corpus.
    assert_eq!(
        release, "changelog:changelog#releases/1-0-0",
        "the emitted release address carries the minted id (read back, not re-derived)",
    );
    let json = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "show", &release, "--format", "json"],
            None,
        ),
        "doc show <release> --format json",
    );
    assert_eq!(
        json,
        RELEASE_ITEM_JSON.replace("<DATE>", &date),
        "the item object carries its minted `id`, and every nested item carries its own",
    );
}
