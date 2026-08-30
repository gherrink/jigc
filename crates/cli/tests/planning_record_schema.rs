//! M49 Increment 9 / T5 — the shipped `planning-record` doctype: **fourteen gates,
//! fourteen required slots** (`design/methodology-docs.md` → The planning
//! gate-record; `completions/artifacts/M49/settle-record.md` → D9).
//!
//! The doctype exists for exactly one behaviour: an unanswered planning gate must
//! **block**. That is `required-slot-present` over one required slot per gate — and
//! D9's first draft (a repeatable `gates` section whose names are data) was reverted
//! at the design review precisely because a repeatable has no such fence, so a record
//! carrying 3 of the 14 gates would validate clean.
//!
//! **The subject is derived, never listed.** All three arms read the fourteen gates
//! out of `design/methodology-docs.md`'s gate table — the settled single home — so a
//! gate added, renamed or reworded there reddens here until the schema follows.
//! Nothing in this file hand-lists a gate id or a hint.
//!
//! Three arms, each driving the real binary against the real shipped methodology
//! pack:
//!
//!   (a) **The shape** — `jigc doc schema planning-record --format json` projects
//!       fourteen `kind: "slot"` sections, in table order, each **required** (the
//!       projection's `optional` marker is skip-on-false, so a required slot carries
//!       no such key) and each naming its own `set-slot` write address.
//!
//!   (b) **The hints are the table cells, verbatim** — driven through the surface
//!       that actually renders a `hint:`, the `{{schema:<doctype>}}` projection
//!       (`design/surface-contract.md` → The schema projection): a project step
//!       shadow carries a lone `{{ schema:planning-record }}`, `jigc workflow
//!       planning --preview` composes it, and the **emitted text** must carry each
//!       gate's *What it requires recorded* cell byte-for-byte. `doc schema` renders
//!       no hint on either format, so asserting the hints there would assert nothing;
//!       reading the YAML instead of the emitted bytes would be the masking test.
//!
//!   (c) **Free at the freeze** — the new doctype owes **no** `schema-snapshots/`
//!       entry (there is no prior version to diff against), and a corpus holding a
//!       committed `planning-record` beside the other methodology records migrates
//!       `0 blocked` under `jigc migrate-corpus`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The doctype under test.
const TYPE: &str = "planning-record";

/// The single home of the gate set (`settle-record.md` → D9).
const GATE_TABLE_DOC: &str = "design/methodology-docs.md";

/// The heading whose table carries one row per planning gate.
const GATE_TABLE_HEADING: &str = "## The planning gate-record";

/// The gate table's first column header (the gate id).
const GATE_COLUMN: &str = "Gate";

/// The gate table column whose cell moves verbatim into the slot `hint:`
/// (`design/methodology-docs.md`:77 — *"those sentences move verbatim into the
/// schema's `hint:`; nothing is redesigned"*).
const HINT_COLUMN: &str = "What it requires recorded";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-planning-record-{tag}-{}-{:?}",
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

/// The workspace root — the checked-in design doc and pack tree are both read from
/// it, so every arm runs against the shipped artifacts rather than a fixture copy.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

/// The on-disk methodology pack home.
fn methodology_pack_tree() -> PathBuf {
    repo_root().join("packs").join("methodology")
}

/// Split a markdown table row into its trimmed cells.
fn cells(row: &str) -> Vec<&str> {
    row.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(str::trim)
        .collect()
}

/// The fourteen gates, in table order, as `(gate-id, what-it-requires-recorded)`.
/// Derived from the settled single home; the schema is measured against this, never
/// the other way round.
fn gates() -> Vec<(String, String)> {
    let text = fs::read_to_string(repo_root().join(GATE_TABLE_DOC))
        .unwrap_or_else(|e| panic!("{GATE_TABLE_DOC} is readable: {e}"));
    let mut lines = text
        .lines()
        .skip_while(|l| !l.starts_with(GATE_TABLE_HEADING));
    assert!(
        lines.next().is_some(),
        "{GATE_TABLE_DOC} has no `{GATE_TABLE_HEADING}` section — the gate set's home moved"
    );
    let table: Vec<&str> = lines
        .map(str::trim)
        .skip_while(|l| !l.starts_with('|'))
        .take_while(|l| l.starts_with('|'))
        .collect();
    assert!(
        table.len() > 2,
        "{GATE_TABLE_DOC}: `{GATE_TABLE_HEADING}` carries no gate rows"
    );

    let header = cells(table[0]);
    let gate_at = header
        .iter()
        .position(|c| *c == GATE_COLUMN)
        .unwrap_or_else(|| {
            panic!("{GATE_TABLE_DOC}: the gate table has no `{GATE_COLUMN}` column")
        });
    let hint_at = header
        .iter()
        .position(|c| *c == HINT_COLUMN)
        .unwrap_or_else(|| {
            panic!("{GATE_TABLE_DOC}: the gate table has no `{HINT_COLUMN}` column")
        });

    let gates: Vec<(String, String)> = table[2..]
        .iter()
        .map(|row| {
            let cells = cells(row);
            assert_eq!(
                cells.len(),
                header.len(),
                "{GATE_TABLE_DOC}: gate row has {} cells, header has {}: {row}",
                cells.len(),
                header.len()
            );
            (
                cells[gate_at].trim_matches('`').to_string(),
                cells[hint_at].to_string(),
            )
        })
        .collect();
    assert_eq!(
        gates.len(),
        14,
        "the settled gate set is fourteen (D9); {GATE_TABLE_DOC} now lists {} — if a gate was \
         genuinely added or retired, `{TYPE}` owes a schema-version bump in the same motion",
        gates.len()
    );
    gates
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

/// Run `jigc <args>` in `repo` against the REAL shipped methodology pack.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
        .output()
        .expect("run the jigc binary")
}

/// Both streams of an invocation, rendered for assertion messages.
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Assert a `jigc` invocation succeeded, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; streams:\n{}",
        streams(out)
    );
}

/// A committed git repo carrying the `.jigc/config/` project layer.
fn seed_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create the project layer");
}

/// (a) The shape: fourteen sections, each a REQUIRED slot named by the gate table,
/// in table order, each carrying its own `set-slot` write address.
#[test]
fn doc_schema_json_projects_one_required_slot_per_gate() {
    let gates = gates();
    let repo = TempDir::new("shape");
    let home = TempDir::new("shape-home");
    seed_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", TYPE, "--format", "json"],
    );
    assert_ok(&out, "`jigc doc schema planning-record --format json`");
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("the pinned projection is valid JSON");

    assert_eq!(
        json["type"],
        TYPE,
        "the projection is the planning-record's; got:\n{}",
        streams(&out)
    );
    let sections = json["sections"]
        .as_array()
        .expect("the projection carries a `sections` array");
    let projected: Vec<&str> = sections
        .iter()
        .map(|s| s["id"].as_str().expect("each section names its id"))
        .collect();
    let expected: Vec<&str> = gates.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(
        projected, expected,
        "the projected sections must be the fourteen gates, in the table's order"
    );

    for section in sections {
        let id = section["id"].as_str().expect("section id");
        assert_eq!(
            section["kind"], "slot",
            "gate `{id}` must be a prose slot section — a repeatable has no \
             `required-slot-present` fence, which is the whole reason the doctype exists"
        );
        assert!(
            section.get("optional").is_none(),
            "gate `{id}` must be REQUIRED: the projection's `optional` marker is skip-on-false, \
             so its presence means the gate does not block finalize"
        );
        assert_eq!(
            section["set-slot"],
            serde_json::Value::String(format!("{TYPE}:<slug>#{id}")),
            "gate `{id}` must name its own write address"
        );
    }
}

/// (b) Every gate's `hint:` is its table cell **verbatim**, proven on the emitted
/// bytes of the surface that renders a hint — the `{{schema:<doctype>}}` projection.
#[test]
fn the_composed_schema_projection_carries_every_gate_cell_verbatim() {
    let gates = gates();
    let repo = TempDir::new("hints");
    let home = TempDir::new("hints-home");
    seed_repo(repo.path());

    // A project-layer step shadow carrying a lone projection placeholder — the
    // `schema_projection.rs` precedent, so no fixture pack and no pack-load fence.
    let steps = repo.path().join(".jigc").join("config").join("steps");
    fs::create_dir_all(&steps).expect("create the project steps dir");
    fs::write(
        steps.join("plan-scope.yaml"),
        format!("Record the plan-time gates:\n\n{{{{ schema:{TYPE} }}}}\n"),
    )
    .expect("write the project step shadow");

    let out = jigc(
        repo.path(),
        home.path(),
        &["workflow", "planning", "--preview"],
    );
    assert_ok(&out, "`jigc workflow planning --preview`");
    let composed = String::from_utf8(out.stdout).expect("utf-8 composed output");

    let mut missing = Vec::new();
    for (id, cell) in &gates {
        let line = format!("- `{id}`: prose slot — {cell}");
        if !composed.contains(&line) {
            missing.push(id.clone());
        }
    }
    assert!(
        missing.is_empty(),
        "these gates' hints are not their `{GATE_TABLE_DOC}` cell verbatim: {missing:?}\n\
         composed:\n{composed}"
    );
}

/// (c) Free at the freeze: the new doctype owes no prior-schema snapshot, and a
/// corpus holding a committed `planning-record` migrates `0 blocked`.
#[test]
fn the_new_doctype_owes_no_snapshot_and_blocks_no_migration() {
    let snapshots = methodology_pack_tree().join("schema-snapshots");
    let strays: Vec<String> = fs::read_dir(&snapshots)
        .expect("the methodology snapshot store is readable")
        .map(|e| e.expect("a snapshot dir entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .filter(|n| n.starts_with(&format!("{TYPE}.v")))
        .collect();
    assert!(
        strays.is_empty(),
        "`{TYPE}` ships at schema-version 1, so it has no prior shape to diff against and \
         owes NO snapshot; found {strays:?}"
    );

    let repo = TempDir::new("migrate");
    let home = TempDir::new("migrate-home");
    seed_repo(repo.path());

    // A committed record at the shipped stamp, rendered in the canonical shape the
    // engine writes (front-matter stamp, `# <title>`, one `## <Gate Heading>` per
    // gate) — the corpus `migrate-corpus` sweeps.
    let gates = gates();
    let mut doc = String::from("---\nschema-version: 1\n---\n\n# M49\n");
    for (id, _) in &gates {
        let heading = id
            .split('-')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(first) => first.to_ascii_uppercase().to_string() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        doc.push_str(&format!("\n## {heading}\n\nDriven: see the record.\n"));
    }
    let home_dir = repo.path().join("planning-records");
    fs::create_dir_all(&home_dir).expect("create the planning-records home");
    fs::write(home_dir.join("m49.md"), &doc).expect("write the record");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "the planning record"]);

    let out = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    assert_ok(&out, "`jigc migrate-corpus`");
    let text = streams(&out);
    assert!(
        text.contains("0 migrated, 1 already current, 0 blocked"),
        "the committed `{TYPE}` must be swept as a MANAGED doc at the shipped stamp — already \
         current, nothing blocked (an unrecognised doctype would leave it uncounted, which is \
         how this arm tells a shipped doctype from a foreign file); got:\n{text}"
    );
    assert!(
        text.contains("planning-records/m49.md"),
        "the sweep must name the record it swept; got:\n{text}"
    );
}
