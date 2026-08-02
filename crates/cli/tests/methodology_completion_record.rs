//! M16 Increment 5 / T1 — the REAL methodology `completion-record` schema
//! (`design/methodology-docs.md` → The four doctypes + Relations; The engine work
//! item 3, the owner-artifact gate). The per-milestone close record: NOT a
//! singleton (`id-from: title`, create-fresh per milestone), a `meta` header
//! carrying the `verdict` enum + the engine-native `owned-location` `owner-artifact`
//! (the #5-gate target shipped inc-3), and a repeatable `findings` section whose
//! every entry pairs a `severity`/`disposition` enum with a plain `evidence` STRING
//! (NOT a managed ref — the methodology pack composes alone).
//!
//! Driven through the REAL binary against the REAL shipped YAML — never the inc-3
//! FIXTURE completion-record (which carried only the bare `owner-artifact` field, no
//! verdict/findings). Three proofs:
//!
//!   (a) **Loads through the binary.** `JIGC_PACK_DIR=<methodology> jigc describe`
//!       resolves every methodology schema through the cascade — a malformed YAML or
//!       an undeclared field type would exit non-zero. `completion-record` surfaces
//!       in the JSON projection's doctype list.
//!
//!   (b) **Exact shape + the owned-location bare round-trip.** The shipped YAML loads
//!       through the engine's public `load_schema` (the loader the binary runs) at
//!       its exact shape, and re-serializes with the `owner-artifact` field's type
//!       still the **bare `owned-location` spelling** (not an unresolved Pack
//!       reference). A real authored instance, promoted to disk through `finalize`,
//!       re-parses byte-stable: `render(instance_from_source(bytes)) == bytes`.
//!
//!   (c) **A half-authored findings entry BLOCKS.** A `findings` item left with an
//!       empty required field makes `task finalize` exit non-zero at
//!       `schema-conformance` — the repeatable-conformance gate fires over the REAL
//!       findings block, not a vacuous always-pass.
//!
//! The authoring path rides `JIGC_PACK_DIR=<methodology>` (so the REAL schema +
//! `commit` doctype + knobs load) UNIONed with a tiny **workflow-only** fixture pack
//! (named in the in-repo `packs.yaml`) supplying a `creates-task: true` workflow
//! whose create-gate admits `completion-record` — the `completion` workflow itself is
//! T2, not yet built, so the test supplies the minimal authoring host. The schema
//! under test is the REAL one; only the host workflow is a fixture.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use engine::schema::{FieldType, Leaf, Schema, SectionBody, load_schema};
use engine::write::{instance_from_source, render};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-completion-record-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Read the shipped `completion-record.yaml`'s exact bytes from the pack tree.
fn completion_record_bytes() -> Vec<u8> {
    fs::read(
        methodology_pack_tree()
            .join("schemas")
            .join("completion-record.yaml"),
    )
    .expect("read completion-record.yaml")
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = <methodology>`,
/// capturing output. The methodology pack is the base (its REAL `completion-record` +
/// `commit` + knobs load); the listed fixture pack unions a host workflow on top.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>` piping `stdin` (the `set-slot --from-file -` path),
/// honoring the same `JIGC_PACK_DIR` methodology base.
fn jigc_doc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .arg("doc")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
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

/// Seed the host fixture pack: a `creates-task: true` workflow `record` whose
/// create-gate admits the (REAL, methodology-supplied) `completion-record`. The
/// workflow references no `{{cli.X}}` command, so the methodology pack's catalog
/// satisfies the read. NO schema here — the schema under test is the real one.
fn seed_host_pack(pack: &Path) {
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk host pack subdir");
    }
    // The `record` workflow's catalog is read against its OWN (host) origin pack
    // (the catalog-leak discipline). It references no `{{cli.X}}`, so an empty
    // catalog satisfies the read.
    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    fs::write(
        workflows.join("record.yaml"),
        "---\n\
         when: record a milestone completion\n\
         description: A host workflow that creates a completion-record.\n\
         usage: proving the real completion-record schema through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: completion-record, as: record}]\n\
         ---\n\
         {{ include: step:audit }}\n",
    )
    .expect("seed record workflow");
    fs::write(
        steps.join("audit.yaml"),
        "Record the milestone completion for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed audit step");
}

/// Record the host fixture pack in the in-repo project layer's `packs.yaml`, UNIONing
/// it (highest-precedence) over the `JIGC_PACK_DIR` methodology base.
fn list_host_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the host pack");
}

/// Fill the commit doc (methodology-pack `commit` doctype) so finalize over it
/// validates clean — leaving the completion-record conformance as the only lever.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        assert_ok(
            &jigc(repo, home, &["doc", "set-field", addr, "--value", value]),
            &format!("set-field {addr}"),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        assert_ok(
            &jigc_doc_stdin(repo, home, &["set-slot", addr, "--from-file", "-"], prose),
            &format!("set-slot {addr}"),
        );
    };
    set_field(&format!("commit:{task}#type"), "chore");
    set_field(&format!("commit:{task}#scope"), "completion");
    set_slot(
        &format!("commit:{task}#summary"),
        b"record the completion\n",
    );
    set_slot(&format!("commit:{task}#body"), b"A milestone completion.\n");
}

/// Start a `record` task and create a `completion-record`, returning the emitted
/// doc address (captured from `create`'s stdout, run verbatim downstream).
fn start_and_create(repo: &Path, home: &Path, intent: &str) -> String {
    assert_ok(
        &jigc(repo, home, &["start", "--workflow", "record", intent]),
        "`jigc start --workflow record`",
    );
    let create = jigc(
        repo,
        home,
        &["doc", "create", "completion-record", "--title", "M16"],
    );
    assert_ok(&create, "`doc create completion-record`");
    String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// (a) The real binary loads the `completion-record` schema as part of the methodology
/// pack — `jigc describe` surfaces it in the JSON doctype projection.
#[test]
fn the_completion_record_schema_loads_through_the_binary_describe_projection() {
    let repo = TempDir::new("describe");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let setup = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&setup, "`JIGC_PACK_DIR=<methodology> jigc setup`");

    let out = jigc(repo.path(), home.path(), &["describe", "--format", "json"]);
    assert_ok(&out, "`jigc describe` over the methodology pack");
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("describe --format json emits valid JSON");
    let doctype_ids: Vec<&str> = json["definitions"]
        .as_array()
        .expect("definitions is an array")
        .iter()
        .filter(|d| d["kind"] == "doctype")
        .filter_map(|d| d["id"].as_str())
        .collect();
    assert!(
        doctype_ids.contains(&"completion-record"),
        "the `completion-record` doctype must surface in describe (the binary loaded its \
         schema); got doctype ids: {doctype_ids:?}",
    );
}

/// (b) The shipped YAML loads at its exact shape through the engine's public loader,
/// and re-serializes with the `owner-artifact` field STILL the bare `owned-location`
/// spelling (not an unresolved Pack reference).
#[test]
fn the_completion_record_schema_is_exact_shape_with_a_bare_owned_location() {
    let schema = load_schema(&completion_record_bytes())
        .expect("completion-record.yaml loads engine-native");
    assert_eq!(schema.ty, "completion-record");
    // NOT a singleton — `id-from: title`, create-fresh per milestone.
    assert!(
        !schema.singleton,
        "completion-record is per-milestone, NOT a running singleton",
    );
    assert_eq!(schema.id_from.as_deref(), Some("title"));
    assert_eq!(
        schema.location.as_deref(),
        Some("completions/"),
        "completion-record gets its OWN location subdir",
    );

    // The `meta` header carries the verdict enum + the owner-artifact owned-location.
    let meta = schema
        .sections
        .iter()
        .find(|s| s.id == "meta")
        .expect("a `meta` section");
    assert!(meta.header, "`meta` is a header section");
    let meta_fields = match &meta.body {
        SectionBody::Simple { fields, .. } => fields.as_slice(),
        SectionBody::Repeatable { .. } => panic!("`meta` must be a simple header, not repeatable"),
    };
    let verdict = meta_fields
        .iter()
        .find(|f| f.id == "verdict")
        .expect("`meta` carries a `verdict` field");
    assert_eq!(verdict.ty, FieldType::Enum, "`verdict` is an enum");
    assert_eq!(
        verdict.of.as_deref(),
        Some(["green", "red"].map(String::from).as_slice()),
        "`verdict` enumerates exactly the green/red members",
    );
    let owner = meta_fields
        .iter()
        .find(|f| f.id == "owner-artifact")
        .expect("`meta` carries an `owner-artifact` field");
    assert_eq!(
        owner.ty,
        FieldType::OwnedLocation,
        "`owner-artifact` is the engine-native owned-location (the #5-gate target)",
    );

    // The repeatable `findings`: title + severity enum + disposition enum + evidence string.
    let (id_from, block) = match &schema
        .sections
        .iter()
        .find(|s| s.id == "findings")
        .expect("a `findings` section")
        .body
    {
        SectionBody::Repeatable { repeatable } => {
            (repeatable.id_from.as_str(), repeatable.block.as_slice())
        }
        SectionBody::Simple { .. } => panic!("`findings` must be repeatable"),
    };
    assert_eq!(
        id_from, "title",
        "the findings entry is slugged from `title`"
    );
    let block_field = |id: &str| -> &engine::schema::Field {
        block
            .iter()
            .find_map(|leaf| match leaf {
                Leaf::Field(f) if f.id == id => Some(f.as_ref()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("the findings entry carries a `{id}` field"))
    };
    let disposition = block_field("disposition");
    assert_eq!(disposition.ty, FieldType::Enum, "`disposition` is an enum");
    assert_eq!(
        disposition.of.as_deref(),
        Some(
            ["fixed", "deferred", "contested"]
                .map(String::from)
                .as_slice()
        ),
        "`disposition` enumerates exactly fixed/deferred/contested",
    );
    let severity = block_field("severity");
    assert_eq!(severity.ty, FieldType::Enum, "`severity` is an enum");
    let evidence = block_field("evidence");
    assert_eq!(
        evidence.ty,
        FieldType::String,
        "`evidence` is a plain string (a methodology pack composes alone — never a managed ref)",
    );
    assert!(
        evidence.to.is_none(),
        "`evidence` is NOT a managed ref (the artifact it names lives in the dev pack)",
    );

    // The owned-location type re-serializes to the BARE spelling and survives a reload.
    let json = serde_json::to_string(&schema).expect("schema re-serializes");
    assert!(
        json.contains("\"owned-location\""),
        "`owner-artifact`'s type re-serializes to the bare `owned-location` spelling; got: {json}",
    );
    let reloaded = load_schema(json.as_bytes()).expect("re-serialized schema reloads");
    assert_eq!(
        reloaded
            .sections
            .iter()
            .find(|s| s.id == "meta")
            .and_then(|m| match &m.body {
                SectionBody::Simple { fields, .. } => fields
                    .iter()
                    .find(|f| f.id == "owner-artifact")
                    .map(|f| f.ty.clone()),
                SectionBody::Repeatable { .. } => None,
            }),
        Some(FieldType::OwnedLocation),
        "`owner-artifact` stays owned-location across a serde roundtrip",
    );
}

/// (b, cont.) A real `completion-record` instance — authored through the binary,
/// promoted to disk by `finalize` — re-parses byte-stable: `render(parse(x)) == x`.
#[test]
fn an_authored_completion_record_round_trips_byte_stable() {
    let repo = TempDir::new("roundtrip");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("host");
    seed_host_pack(pack.path());
    list_host_pack(repo.path(), pack.path());

    let task = "record-a-milestone-completion";
    let addr = start_and_create(repo.path(), home.path(), "record a milestone completion");

    // Author the meta header (verdict + a durably-staged owner-artifact) and one
    // complete findings entry. The owner-artifact is staged under its owned home so
    // the #5 gate lands (shipped inc-3) — finalize promotes the record to disk.
    let artifact = "completions/artifacts/M16/audit.md";
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), "the genuine audit transcript\n")
        .expect("write owner-artifact");
    git(repo.path(), &["add", artifact]);

    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{addr}#meta/verdict"),
                "--value",
                "green",
            ],
        ),
        "set verdict",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{addr}#meta/owner-artifact"),
                "--value",
                artifact,
            ],
        ),
        "set owner-artifact",
    );

    let add = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            &format!("{addr}#findings"),
            "--title",
            "Vacuous gate risk",
        ],
    );
    assert_ok(&add, "add-item findings");
    let item = String::from_utf8(add.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    for (leaf, value) in [
        ("severity", "blocking"),
        ("disposition", "fixed"),
        ("evidence", "owner_artifact_gate.rs"),
    ] {
        assert_ok(
            &jigc(
                repo.path(),
                home.path(),
                &[
                    "doc",
                    "set-field",
                    &format!("{item}/{leaf}"),
                    "--value",
                    value,
                ],
            ),
            &format!("set findings {leaf}"),
        );
    }

    fill_commit(repo.path(), home.path(), task);

    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(
        &fin,
        &format!(
            "a complete completion-record must finalize clean; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&fin.stdout),
            String::from_utf8_lossy(&fin.stderr),
        ),
    );

    // The promoted committed bytes re-parse byte-stable through the REAL schema:
    // render(instance_from_source(bytes)) == bytes — the byte-stability writer property.
    let promoted = repo.path().join("completions").join("m16.md");
    let bytes = fs::read_to_string(&promoted)
        .unwrap_or_else(|e| panic!("read promoted completion-record at {promoted:?}: {e}"));
    let mut schema: Schema =
        load_schema(&completion_record_bytes()).expect("completion-record schema loads");
    // Mirror the production loader's resolved shape: since M40 A1 the methodology
    // manifest freezes `completion-record`, so the promoted instance carries the
    // injected schema-version stamp.
    engine::schema::inject_schema_version_stamp(&mut schema);
    let rerendered = render(
        &schema,
        &instance_from_source(&schema, &bytes).expect("promoted completion-record parses"),
    );
    assert_eq!(
        rerendered, bytes,
        "the authored completion-record round-trips byte-stable (render(parse(x)) == x)",
    );
}

/// (c) A half-authored findings entry — an empty required field — makes `task finalize`
/// exit non-zero at `schema-conformance`. The repeatable-conformance gate FIRES over
/// the real findings block (not a vacuous always-pass), and no commit lands.
#[test]
fn finalize_blocks_a_half_authored_findings_entry() {
    let repo = TempDir::new("half");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("host");
    seed_host_pack(pack.path());
    list_host_pack(repo.path(), pack.path());

    let task = "record-a-milestone-completion";
    let addr = start_and_create(repo.path(), home.path(), "record a milestone completion");

    // A durably-staged owner-artifact (so the #5 gate is NOT the lever) + a filled
    // meta header.
    let artifact = "completions/artifacts/M16/audit.md";
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), "transcript\n").expect("write owner-artifact");
    git(repo.path(), &["add", artifact]);
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{addr}#meta/verdict"),
                "--value",
                "green",
            ],
        ),
        "set verdict",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{addr}#meta/owner-artifact"),
                "--value",
                artifact,
            ],
        ),
        "set owner-artifact",
    );

    // A findings entry left HALF-AUTHORED: severity + evidence set, but the required
    // `disposition` enum is never filled.
    let add = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            &format!("{addr}#findings"),
            "--title",
            "Half authored",
        ],
    );
    assert_ok(&add, "add-item findings");
    let item = String::from_utf8(add.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{item}/severity"),
                "--value",
                "advisory",
            ],
        ),
        "set severity",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{item}/evidence"),
                "--value",
                "somewhere",
            ],
        ),
        "set evidence",
    );
    // `disposition` deliberately left empty → schema-conformance must block.

    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a half-authored findings entry must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("schema-conformance"),
        "the block surfaces a schema-conformance finding over the findings block; got:\n{rendered}",
    );
    assert!(
        rendered.contains("disposition") && rendered.contains("findings"),
        "the block names the empty `disposition` field inside the `findings` item — the \
         repeatable-conformance gate firing over the REAL findings block, not a vacuous \
         pass or an unrelated finding; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
}
