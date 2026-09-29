//! M16 Increment 4 / T1 — the three methodology running-doc schemas
//! (`roadmap` / `deferral-ledger` / `decisions-log`), proven through the real
//! `jigc` binary AND asserted at their EXACT new shape (`design/methodology-docs.md`
//! → The four doctypes).
//!
//! These are pure pack data — zero `crates/*/src` change. The singleton flag was
//! proven by the `runlog` FIXTURE (inc-2 T3); this file proves the three REAL
//! shipped YAML files, so the assertions exercise the exact shapes the planning
//! workflow authors, not a fixture stand-in. (M38: the three relocated from their
//! own `location:` subdirs to literal `docs/*.md` `placement` homes.)
//!
//! Two layers of proof:
//!
//!   (a) **Through the real binary.** `JIGC_PACK_DIR=<methodology> jigc setup` then
//!       `jigc describe --format json` loads every methodology schema through the
//!       cascade (`CascadeDefs::all_schemas`) — a malformed YAML or an undeclared
//!       field type would make `describe` exit non-zero. The three doctypes appear
//!       in the projection, so the binary genuinely parsed and resolved them.
//!
//!   (b) **The exact new shape.** Each shipped YAML's bytes are loaded through the
//!       engine's public `load_schema` (engine-native types only — the same loader
//!       the binary runs, fed no pack field-types, which the methodology pack does
//!       not declare). Each carries `singleton: true`, its OWN literal
//!       `placement.file` home, the single repeatable section, and the declared
//!       leaves (the `kind` enum members, the `date` `set: on-create` fields).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use engine::schema::{FieldType, Leaf, Schema, SectionBody, load_schema};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-methodology-running-doctypes-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Read a shipped methodology schema YAML's exact bytes from the on-disk pack tree.
fn methodology_schema_bytes(file: &str) -> Vec<u8> {
    fs::read(methodology_pack_tree().join("schemas").join(file))
        .unwrap_or_else(|e| panic!("read methodology schema {file}: {e}"))
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
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
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

/// Locate the single repeatable section of `schema` by id, returning its
/// `id-from` source field and the item block's leaves.
fn repeatable_block<'a>(schema: &'a Schema, section_id: &str) -> (&'a str, &'a [Leaf]) {
    let section = schema
        .sections
        .iter()
        .find(|s| s.id == section_id)
        .unwrap_or_else(|| panic!("{} has a `{section_id}` section", schema.ty));
    match &section.body {
        SectionBody::Repeatable { repeatable } => {
            (repeatable.id_from.as_str(), repeatable.block.as_slice())
        }
        SectionBody::Simple { .. } => {
            panic!("{}'s `{section_id}` section must be repeatable", schema.ty)
        }
    }
}

/// The leaf field with `id` inside a repeatable block, or `None` if it is a slot /
/// absent.
fn block_field<'a>(block: &'a [Leaf], id: &str) -> Option<&'a engine::schema::Field> {
    block.iter().find_map(|leaf| match leaf {
        Leaf::Field(f) if f.id == id => Some(f.as_ref()),
        _ => None,
    })
}

/// `true` when the block carries a prose slot leaf named `id`.
fn block_has_slot(block: &[Leaf], id: &str) -> bool {
    block
        .iter()
        .any(|leaf| matches!(leaf, Leaf::Slot { id: i, .. } if i == id))
}

#[test]
fn the_three_running_doctypes_load_through_the_binary_describe_projection() {
    // (a) The real binary loads all three schemas as part of the methodology pack.
    // `jigc describe` reads every doctype schema through the cascade — a malformed
    // YAML or an undeclared field type would exit non-zero. The three appear in the
    // JSON projection's doctype list, proving the binary parsed + resolved them.
    let repo = TempDir::new("describe");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["describe", "--format", "json"],
    );
    assert!(
        out.status.success(),
        "`jigc describe` over the methodology pack must load every schema and exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("describe --format json emits valid JSON");
    let doctype_ids: Vec<&str> = json["definitions"]
        .as_array()
        .expect("definitions is an array")
        .iter()
        .filter(|d| d["kind"] == "doctype")
        .filter_map(|d| d["id"].as_str())
        .collect();
    for ty in ["roadmap", "deferral-ledger", "decisions-log"] {
        assert!(
            doctype_ids.contains(&ty),
            "the `{ty}` doctype must surface in describe (the binary loaded its schema); \
             got doctype ids: {doctype_ids:?}",
        );
    }
}

#[test]
fn fresh_doc_create_mints_the_title_cased_h1_for_each_running_singleton() {
    // M40 inc-5 T1 (A4.3): the three running singletons declare `display-title`
    // (the `vision`/`changelog` precedent), so a fresh mint's H1 reads title-cased
    // (`# Roadmap` / `# Decisions Log` / `# Deferral Ledger`), never the lowercase
    // slug. Mint-only: `display-title` has no validate consumer, so committed
    // lowercase-H1 corpora stay conformant. Proven over the real binary — the
    // asserted bytes are the exact staged file `doc create` emitted.
    //
    // **The decoy premise is revised, not dropped (M48).** This test used to pass a
    // deliberately mismatched `--title` and assert the schema knob won anyway — which was
    // the *silent* half of that fact, and is exactly the write acking success over a title
    // it dropped. Since M48 a divergent `--title` on a singleton is **refused**
    // (`write.title-ignored`; `design/write-commands.md` → The four-way write), so the same
    // fact is pinned harder: the mint carries the schema's H1 under the schema's own title,
    // and the decoy is rejected rather than swallowed.
    let repo = TempDir::new("display-title");
    init_repo(repo.path());
    let home = TempDir::new("display-title-home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    // The `planning` workflow gates creation of all three running singletons.
    let start = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "mint the running docs"],
    );
    assert!(
        start.status.success(),
        "`jigc start --workflow planning` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );
    let task = "mint-the-running-docs";

    for (ty, display) in [
        ("roadmap", "Roadmap"),
        ("decisions-log", "Decisions Log"),
        ("deferral-ledger", "Deferral Ledger"),
    ] {
        // The decoy is refused, not swallowed — nothing is staged by it.
        let decoy = run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["doc", "create", ty, "--title", "Decoy Title"],
        );
        assert!(
            !decoy.status.success(),
            "`jigc doc create {ty} --title \"Decoy Title\"` must be refused — the title is \
             the schema's, not the author's; stdout:\n{}",
            String::from_utf8_lossy(&decoy.stdout),
        );
        assert!(
            String::from_utf8_lossy(&decoy.stderr).contains("write.title-ignored"),
            "the refusal names the dropped title; got:\n{}",
            String::from_utf8_lossy(&decoy.stderr),
        );

        let out = run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["doc", "create", ty, "--title", display],
        );
        assert!(
            out.status.success(),
            "`jigc doc create {ty}` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let staged_path = repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join(task)
            .join("docs")
            .join(format!("{ty}:{ty}.md"));
        let staged = fs::read_to_string(&staged_path)
            .unwrap_or_else(|e| panic!("read the staged {ty} mint at {staged_path:?}: {e}"));
        assert!(
            staged.lines().any(|l| l == format!("# {display}")),
            "a fresh `{ty}` mint carries the title-cased H1 `# {display}`; got:\n{staged}",
        );
        assert!(
            staged.lines().all(|l| l != format!("# {ty}")),
            "a fresh `{ty}` mint must not carry the lowercase slug H1 `# {ty}`; got:\n{staged}",
        );
    }
}

#[test]
fn roadmap_schema_is_a_singleton_with_a_milestones_repeatable_and_two_slots() {
    let schema = load_schema(&methodology_schema_bytes("roadmap.yaml"))
        .expect("roadmap.yaml loads engine-native");
    assert_eq!(schema.ty, "roadmap");
    assert!(schema.singleton, "roadmap is a running singleton");
    assert_eq!(
        schema.location, None,
        "a placement doctype sets no `location` — the home is the literal `placement.file`",
    );
    assert_eq!(
        schema.placement.as_ref().map(|p| p.file.as_str()),
        Some("docs/roadmap.md"),
        "roadmap is managed directly at the repo-root literal `docs/roadmap.md`",
    );

    let (id_from, block) = repeatable_block(&schema, "milestones");
    assert_eq!(
        id_from, "title",
        "the milestones entry is slugged from `title`"
    );
    assert!(
        block_field(block, "title").is_some(),
        "the milestones entry carries a `title` id-source field",
    );
    // The decomposition (increments-as-prose) and the `proves` differentiator are
    // prose slots — the one-level bound (no structured sub-items).
    assert!(
        block_has_slot(block, "proves"),
        "the milestones entry carries a `proves` slot",
    );
    assert!(
        block_has_slot(block, "decomposition"),
        "the milestones entry carries a `decomposition` slot (increments-as-prose)",
    );
}

#[test]
fn deferral_ledger_schema_is_a_singleton_with_a_kind_enum_and_an_on_create_date() {
    let schema = load_schema(&methodology_schema_bytes("deferral-ledger.yaml"))
        .expect("deferral-ledger.yaml loads engine-native");
    assert_eq!(schema.ty, "deferral-ledger");
    assert!(schema.singleton, "deferral-ledger is a running singleton");
    assert_eq!(
        schema.location, None,
        "a placement doctype sets no `location` — the home is the literal `placement.file`",
    );
    assert_eq!(
        schema.placement.as_ref().map(|p| p.file.as_str()),
        Some("docs/deferral-ledger.md"),
        "deferral-ledger is managed directly at the repo-root literal `docs/deferral-ledger.md`",
    );

    let (id_from, block) = repeatable_block(&schema, "entries");
    assert_eq!(id_from, "title");

    // `kind` is an enum over exactly the Decision/Idea members (deferred decision vs
    // parked idea) — the declared leaf the done-criterion names. M41 F4 renamed the
    // two-letter `D`/`I` members to the spelled-out `Decision`/`Idea` (the first
    // methodology v1→v2 migration; deferral-ledger schema-version 2).
    let kind = block_field(block, "kind").expect("the entry carries a `kind` field");
    assert_eq!(kind.ty, FieldType::Enum, "`kind` is an enum");
    assert_eq!(
        kind.of.as_deref(),
        Some(["Decision", "Idea"].map(String::from).as_slice()),
        "`kind` enumerates exactly the Decision/Idea members",
    );

    // `trigger` is a plain string (a milestone work-unit, NOT a managed ref).
    let trigger = block_field(block, "trigger").expect("the entry carries a `trigger` field");
    assert_eq!(trigger.ty, FieldType::String, "`trigger` is a plain string");
    assert!(trigger.to.is_none(), "`trigger` is no managed ref");

    // `date` is CLI-derived on create — the declared `set: on-create` leaf.
    let date = block_field(block, "date").expect("the entry carries a `date` field");
    assert_eq!(date.ty, FieldType::Date, "`date` is a date field");
    assert_eq!(
        date.set.as_deref(),
        Some("on-create"),
        "`date` is CLI-derived on create",
    );

    // The deferral itself is a prose `body` slot.
    assert!(
        block_has_slot(block, "body"),
        "the entry carries a `body` slot",
    );
}

#[test]
fn decisions_log_schema_is_a_singleton_with_an_on_create_date_and_a_why_slot() {
    let schema = load_schema(&methodology_schema_bytes("decisions-log.yaml"))
        .expect("decisions-log.yaml loads engine-native");
    assert_eq!(schema.ty, "decisions-log");
    assert!(schema.singleton, "decisions-log is a running singleton");
    assert_eq!(
        schema.location, None,
        "a placement doctype sets no `location` — the home is the literal `placement.file`",
    );
    assert_eq!(
        schema.placement.as_ref().map(|p| p.file.as_str()),
        Some("docs/decisions-log.md"),
        "decisions-log is managed directly at the repo-root literal `docs/decisions-log.md`",
    );

    let (id_from, block) = repeatable_block(&schema, "entries");
    assert_eq!(id_from, "title");

    // `date` set: on-create — the declared leaf.
    let date = block_field(block, "date").expect("the entry carries a `date` field");
    assert_eq!(date.ty, FieldType::Date, "`date` is a date field");
    assert_eq!(
        date.set.as_deref(),
        Some("on-create"),
        "`date` is CLI-derived on create",
    );

    // The rationale is a prose `why` slot.
    assert!(
        block_has_slot(block, "why"),
        "the entry carries a `why` slot",
    );
}

#[test]
fn the_three_singletons_each_own_a_distinct_placement_file() {
    // Review finding B-1, carried to placement (M38): each persisted singleton owns
    // its OWN exact home, never a shared one — under placement the home is the literal
    // `placement.file`, and exact-path ownership (M38 inc-1/2) keys identity on that
    // literal path. The three placement files must be pairwise-distinct.
    let files: Vec<String> = ["roadmap.yaml", "deferral-ledger.yaml", "decisions-log.yaml"]
        .into_iter()
        .map(|file| {
            let schema = load_schema(&methodology_schema_bytes(file))
                .unwrap_or_else(|e| panic!("{file} loads: {e}"));
            assert!(
                schema.location.is_none(),
                "{file} is a placement doctype — it sets no `location`",
            );
            schema
                .placement
                .unwrap_or_else(|| panic!("{file} declares a placement"))
                .file
        })
        .collect();
    let mut sorted = files.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        files.len(),
        "the three running singletons own pairwise-distinct placement files; got: {files:?}",
    );
}
