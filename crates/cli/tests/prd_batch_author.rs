//! M25 Increment 5, T4 — multi-requirement prd via `jigc doc author --from-file-file` round-trips
//! (Proves P3, the doctype-general batch `items:` path on the new repeatable `prd`
//! shape; `design/auto-migration.md` → prd; `implementation/roadmap.md` → M25
//! Increment 5). Drives the **built** `jigc` binary against a throwaway `git init` temp
//! repo over the shipped dev pack:
//!
//!   `jigc setup`
//!     → `jigc start --workflow project-setup "<idea>"`   (the prd create-gate)
//!     → `jigc doc author prd --from-file - --task <id>`        (ONE declarative payload:
//!         the `vision`/`context` fixed slots + a `requirements` section carrying
//!         ≥2 `items`, each a `title` + a `statement: "<<…>>"` slot)
//!     → fill the auto-provisioned commit doc
//!     → `jigc task finalize <id>`                         (promote into `docs/prds/`)
//!
//! and asserts the promoted `docs/prds/<slug>.md`:
//!   (a) round-trips **byte-stable** — `render(instance_from_source(on_disk)) == on_disk`
//!       through the shipped prd schema, the byte-stability the deliverable names; and
//!   (b) each requirement's `statement` re-reads **verbatim** — by re-parsing the
//!       committed bytes with `engine::parse::parse_sections` and comparing
//!       `item.slot.slice(source)` per item id (the `item_authoring_acceptance.rs`
//!       pattern; `store::read_slice` only slices single-hop `#unit` fragments and
//!       rejects a `#unit/item/leaf` fragment, so the per-item byte contract is the
//!       re-parse, not a `read_slice` on an item leaf).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-prd-batch-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
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

/// The shipped `prd` schema (no pack `code-anchor` type — `requirements` drops the
/// code-anchor that `spec.criteria` carries), loaded from the embedded pack source so the
/// byte-stable round-trip asserts against exactly the bytes that ship.
fn prd_schema() -> engine::schema::Schema {
    const PRD_YAML: &[u8] = include_bytes!("../pack/schemas/prd.yaml");
    engine::schema::load_schema(PRD_YAML).expect("shipped prd schema loads")
}

/// The declarative whole-doc payload: the `vision`/`context` fixed slots + a
/// `requirements` section carrying TWO `items`, each a `title` + a `statement` slot —
/// the multi-requirement batch shape T4 proves (`AuthorPayload`/`PayloadItem` form).
const PAYLOAD: &str = r#"title: "Habit tracker"
sections:
  - id: vision
    set:
      vision: "<<A tracker that turns intentions into daily streaks.>>"
  - id: requirements
    items:
      - title: "Log a habit in one tap"
        set:
          statement: "<<Logging a habit takes a single tap from the home screen.>>"
      - title: "Show the current streak"
        set:
          statement: "<<The current streak is shown front and center.>>"
  - id: context
    set:
      context: "<<Built for solo users who abandon heavyweight planners.>>"
"#;

#[test]
fn multi_requirement_prd_via_doc_author_from_round_trips() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &["setup"], None),
        "jigc setup",
    );

    // Mint the project-setup task — it carries the prd create-gate
    // (`allows-create: [{type: prd, as: brief}]`).
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "project-setup",
                "build a habit tracker",
            ],
            None,
        ),
        "jigc start --workflow project-setup",
    );
    let task = "build-a-habit-tracker";

    // Author the WHOLE prd from one declarative payload (≥2 requirement items).
    let authored = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &["doc", "author", "prd", "--from-file", "-", "--task", task],
            Some(PAYLOAD.as_bytes()),
        ),
        "jigc doc author prd --from-file -",
    );
    assert_eq!(
        authored, "prd:habit-tracker",
        "the batch verb prints the minted per-title-slug address",
    );

    // Fill the auto-provisioned commit doc so the code-less task finalizes (the
    // promoted prd is the only diff — the empty-commit guard is satisfied).
    let set_field = |addr: &str, value: &str| {
        ok_stdout(
            run_jigc(
                repo.path(),
                home.path(),
                &["doc", "set-field", addr, "--value", value],
                None,
            ),
            "jigc doc set-field",
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        ok_stdout(
            run_jigc(
                repo.path(),
                home.path(),
                &["doc", "set-slot", addr, "--from-file", "-"],
                Some(prose),
            ),
            "jigc doc set-slot",
        );
    };
    set_field(&format!("commit:{task}#type"), "docs");
    set_field(&format!("commit:{task}#scope"), "prd");
    set_slot(
        &format!("commit:{task}#summary"),
        b"capture the habit tracker prd\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Develop the new-project idea into its first managed prd.\n",
    );

    // Finalize — promote the batch-authored prd into `docs/prds/`.
    ok_stdout(
        run_jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "jigc task finalize",
    );

    // The promoted prd is committed at its canonical path.
    let on_disk = fs::read_to_string(
        repo.path()
            .join("docs")
            .join("prds")
            .join("habit-tracker.md"),
    )
    .expect("the batch-authored prd is promoted to docs/prds/habit-tracker.md");

    // (a) The promoted prd round-trips byte-stable: render(instance_from_source(x)) == x.
    let schema = prd_schema();
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed prd re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the multi-requirement batch-authored prd is byte-stable across parse -> render:\n{on_disk}",
    );

    // (b) Each requirement's `statement` re-reads verbatim — re-parse the committed bytes
    // and resolve each item's single-slot body by its frozen id (NOT a `read_slice` on an
    // item-leaf fragment, which `store::read_slice` rejects as unsliceable).
    let doc = engine::parse::parse_sections(&schema, &on_disk)
        .expect("the committed repeatable prd re-parses");
    let reqs = doc
        .sections
        .iter()
        .find(|s| s.id == "requirements")
        .expect("requirements section present");
    assert_eq!(
        reqs.items.len(),
        2,
        "exactly the two batch-authored requirements re-parse; got:\n{on_disk}"
    );
    for (id, expected) in [
        (
            "log-a-habit-in-one-tap",
            "Logging a habit takes a single tap from the home screen.",
        ),
        (
            "show-the-current-streak",
            "The current streak is shown front and center.",
        ),
    ] {
        let item = reqs
            .items
            .iter()
            .find(|i| i.id == id)
            .unwrap_or_else(|| panic!("requirement {id} present; staged:\n{on_disk}"));
        let statement = item
            .slot
            .as_ref()
            .map(|sp| sp.slice(&on_disk))
            .unwrap_or("");
        assert_eq!(
            statement.trim(),
            expected,
            "`#requirements/{id}/statement` re-reads byte-for-byte",
        );
    }
}
