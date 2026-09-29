//! M17 Increment 5 / T1 — the REAL methodology `dogfood-record` schema
//! (`design/measurement.md` → The dogfood-record doctype). One record per measured
//! jigc run: NOT a singleton (`id-from: title`, create-fresh per run), a `meta`
//! header carrying the run identity (`case` enum + `binary-sha`), the eight ORGANIC
//! fact ints, the two SEEDED instrument-check ints (seeds validate the instrument,
//! never the thesis), the judged `verdict` enum, and the engine-native
//! `owned-location` `owner-artifact` — plus ONE prose slot, `judgment` (the verdict
//! is authored judgment over the counts, never computed).
//!
//! Driven through the REAL binary against the REAL shipped YAML — the planning
//! spike's ten-field variant is superseded; this exercises the post-review shape.
//! Five proofs:
//!
//!   (a) **Loads through the binary.** `JIGC_PACK_DIR=<methodology> jigc describe`
//!       surfaces `dogfood-record` in the JSON doctype projection.
//!
//!   (b) **Malformed values write-block.** A non-integer fact value and an
//!       out-of-enum `case` member make `doc set-field` exit non-zero — form is
//!       engine-validated at the write boundary (the typed-but-transcribed bound).
//!
//!   (c) **A missing fact field / an omitted owner-artifact BLOCKS finalize** at
//!       `field-value-conformant` (the M40 F1 create skeleton pre-stamps every
//!       author-required meta field empty, so an un-set field is present-but-empty)
//!       — the record cannot land half-transcribed.
//!
//!   (d) **A fully-authored record with a durably-staged artifact finalizes**,
//!       promoting the record to `dogfood/` (its OWN location dir).
//!
//!   (e) **Unshared location.** A repo holding a committed `completion-record` AND
//!       a committed `dogfood-record` validates clean — the two schemas declare
//!       DISJOINT location dirs (`docs/completions/` vs `dogfood/`), so the reconcile
//!       sweep (which parses every file in a location dir against EVERY schema
//!       declaring that dir) never cross-blocks them (the design-binding engine
//!       fact in measurement.md).
//!
//! The authoring path rides `JIGC_PACK_DIR=<methodology>` (the REAL schemas +
//! `commit` doctype + knobs load) UNIONed with a tiny **workflow-only** fixture pack
//! (named in the in-repo `packs.yaml`) supplying `creates-task: true` host workflows
//! whose create-gates admit the records — the `record-dogfood` workflow itself is
//! T2, not yet built. The schemas under test are the REAL ones; only the host
//! workflows are fixtures (the `methodology_completion_record.rs` precedent).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use engine::schema::load_schema;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-dogfood-record-{tag}-{}-{:?}",
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
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Read a shipped methodology schema's exact bytes from the pack tree.
fn schema_bytes(file: &str) -> Vec<u8> {
    fs::read(methodology_pack_tree().join("schemas").join(file))
        .unwrap_or_else(|e| panic!("read {file}: {e}"))
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
/// capturing output. The methodology pack is the base (its REAL `dogfood-record` +
/// `completion-record` + `commit` + knobs load); the listed fixture pack unions the
/// host workflows on top.
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

/// Both streams of an invocation, rendered for assertion messages.
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Seed the host fixture pack: two `creates-task: true` workflows whose create-gates
/// admit the (REAL, methodology-supplied) `dogfood-record` and `completion-record`.
/// They reference no `{{cli.X}}` command, so an empty catalog satisfies the read.
/// NO schema here — the schemas under test are the real ones.
fn seed_host_pack(pack: &Path) {
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk host pack subdir");
    }
    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    // Since M51 Increment 8 T2 the stated-at fence keys on the constituents that
    // SHIP STEPS, so this host pack owes the four ambush-class statements — the
    // tasks its workflows mint are finalized through `jigc task finalize`.
    crate::support::seed_ambush_class_declarer(pack);
    fs::write(
        workflows.join("host-dogfood.yaml"),
        "---\n\
         when: record a measured dogfood run\n\
         description: A host workflow that creates a dogfood-record.\n\
         usage: proving the real dogfood-record schema through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: dogfood-record, as: record}]\n\
         ---\n\
         {{ include: step:transcribe }}\n",
    )
    .expect("seed host-dogfood workflow");
    fs::write(
        workflows.join("host-completion.yaml"),
        "---\n\
         when: record a milestone completion\n\
         description: A host workflow that creates a completion-record.\n\
         usage: landing a committed completion-record beside the dogfood-record.\n\
         creates-task: true\n\
         allows-create: [{type: completion-record, as: record}]\n\
         ---\n\
         {{ include: step:transcribe }}\n",
    )
    .expect("seed host-completion workflow");
    fs::write(
        steps.join("transcribe.yaml"),
        "Record the run for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed transcribe step");
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

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &jigc(repo, home, &["doc", "set-field", addr, "--value", value]),
        &format!("set-field {addr}"),
    );
}

/// Fill the commit doc (methodology-pack `commit` doctype) so finalize over it
/// validates clean — leaving the dogfood-record conformance as the only lever.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "chore");
    set_field(repo, home, &format!("commit:{task}#scope"), "dogfood");
    assert_ok(
        &jigc_doc_stdin(
            repo,
            home,
            &[
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                "-",
            ],
            b"record the run\n",
        ),
        "set-slot commit summary",
    );
    assert_ok(
        &jigc_doc_stdin(
            repo,
            home,
            &[
                "set-slot",
                &format!("commit:{task}#body"),
                "--from-file",
                "-",
            ],
            b"A measured dogfood run.\n",
        ),
        "set-slot commit body",
    );
}

/// Start a host-workflow task and create a doc of `doctype`, returning the emitted
/// doc address (captured from `create`'s stdout, run verbatim downstream).
fn start_and_create(
    repo: &Path,
    home: &Path,
    workflow: &str,
    intent: &str,
    doctype: &str,
    title: &str,
) -> String {
    assert_ok(
        &jigc(repo, home, &["start", "--workflow", workflow, intent]),
        &format!("`jigc start --workflow {workflow}`"),
    );
    let create = jigc(repo, home, &["doc", "create", doctype, "--title", title]);
    assert_ok(&create, &format!("`doc create {doctype}`"));
    String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// The ten fact-int fields (eight organic + two seeded instrument checks) with
/// protocol-plausible values.
const FACT_INTS: [(&str, &str); 10] = [
    ("adapter-writes", "12"),
    ("oob-edits", "1"),
    ("drift-caught", "0"),
    ("validate-blocks", "3"),
    ("halts-expected", "1"),
    ("halts-unplanned", "0"),
    ("fix-rounds", "2"),
    ("audit-findings", "4"),
    ("seeded-oob", "1"),
    ("seeded-blocks", "1"),
];

/// Author the dogfood-record at `addr` completely — every meta field, the staged
/// owner-artifact, and the judgment slot — EXCEPT any field named in `skip`.
fn author_record(repo: &Path, home: &Path, addr: &str, skip: &[&str]) {
    let mut fields: Vec<(&str, &str)> = vec![("case", "pilot"), ("binary-sha", "0123abcd")];
    fields.extend_from_slice(&FACT_INTS);
    fields.push(("verdict", "green"));
    for (id, value) in fields {
        if skip.contains(&id) {
            continue;
        }
        set_field(repo, home, &format!("{addr}#meta/{id}"), value);
    }
    if !skip.contains(&"owner-artifact") {
        // The engine-native owned artifact home is `completions/artifacts/<run>/`
        // (a flat engine-wide constant, independent of the doctype's own location dir).
        let artifact = "completions/artifacts/pilot-run/capture.md";
        fs::create_dir_all(repo.join("completions/artifacts/pilot-run")).expect("mk owned home");
        fs::write(
            repo.join(artifact),
            "transcript + raw hook log + tally + manifest\n",
        )
        .expect("write owner-artifact");
        git(repo, &["add", artifact]);
        set_field(repo, home, &format!("{addr}#meta/owner-artifact"), artifact);
    }
    assert_ok(
        &jigc_doc_stdin(
            repo,
            home,
            &["set-slot", &format!("{addr}#judgment"), "--from-file", "-"],
            b"The loop held; the seeds registered once each. This run did not test ingest.\n",
        ),
        "set-slot judgment",
    );
}

/// (a) The real binary loads the `dogfood-record` schema as part of the methodology
/// pack — `jigc describe` surfaces it in the JSON doctype projection.
#[test]
fn the_dogfood_record_schema_loads_through_the_binary_describe_projection() {
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
        doctype_ids.contains(&"dogfood-record"),
        "the `dogfood-record` doctype must surface in describe (the binary loaded its \
         schema); got doctype ids: {doctype_ids:?}",
    );
}

/// (b) A malformed int fact value and an out-of-enum `case` member make
/// `doc set-field` exit non-zero — form is engine-validated at the write boundary.
#[test]
fn a_malformed_int_and_an_invalid_case_enum_write_block() {
    let repo = TempDir::new("writeblock");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("host");
    seed_host_pack(pack.path());
    list_host_pack(repo.path(), pack.path());

    let addr = start_and_create(
        repo.path(),
        home.path(),
        "host-dogfood",
        "record a measured dogfood run",
        "dogfood-record",
        "Pilot Run",
    );

    // A fact int rejects a non-integer value at the write boundary.
    let bad_int = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("{addr}#meta/oob-edits"),
            "--value",
            "several",
        ],
    );
    let rendered = streams(&bad_int);
    assert!(
        !bad_int.status.success(),
        "a non-integer fact value must write-block; got:\n{rendered}",
    );
    assert!(
        rendered.contains("not an integer"),
        "the write-block names the int violation; got:\n{rendered}",
    );

    // The `case` enum rejects an out-of-member value at the write boundary.
    let bad_enum = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("{addr}#meta/case"),
            "--value",
            "production",
        ],
    );
    let rendered = streams(&bad_enum);
    assert!(
        !bad_enum.status.success(),
        "an out-of-enum `case` value must write-block; got:\n{rendered}",
    );
    assert!(
        rendered.contains("not a member"),
        "the write-block names the enum violation; got:\n{rendered}",
    );

    // The same addresses accept conformant values — the blocks above were the
    // values' fault, not the addresses'.
    set_field(
        repo.path(),
        home.path(),
        &format!("{addr}#meta/oob-edits"),
        "1",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("{addr}#meta/case"),
        "pilot",
    );
}

/// (c) A record missing one transcribed fact field makes `task finalize` exit
/// non-zero — it cannot land half-transcribed. Since M40 F1 the create skeleton
/// pre-stamps every author-required meta field as an empty `key:` line, so the
/// un-transcribed fact is **present-but-empty** and blocks at
/// `field-value-conformant` (an empty int is non-conformant), no longer at
/// `required-field-present` — the gate holds either way.
#[test]
fn finalize_blocks_a_missing_fact_field() {
    let repo = TempDir::new("missingfact");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("host");
    seed_host_pack(pack.path());
    list_host_pack(repo.path(), pack.path());

    let task = "record-a-measured-dogfood-run";
    let addr = start_and_create(
        repo.path(),
        home.path(),
        "host-dogfood",
        "record a measured dogfood run",
        "dogfood-record",
        "Pilot Run",
    );
    // Everything authored EXCEPT the `oob-edits` organic fact.
    author_record(repo.path(), home.path(), &addr, &["oob-edits"]);
    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = streams(&out);
    assert!(
        !out.status.success(),
        "a missing fact field must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("field-value-conformant"),
        "the pre-stamped empty fact blocks at field-value-conformant; got:\n{rendered}",
    );
    assert!(
        rendered.contains("oob-edits"),
        "the block names the un-transcribed `oob-edits` fact field; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
}

/// (c, cont.) A record whose `owner-artifact` field is never set makes `task
/// finalize` exit non-zero — the raw capture cannot be silently omitted. The M40 F1
/// skeleton pre-stamps the field empty, so the omission blocks at
/// `field-value-conformant` (an empty owned-location is non-conformant), no longer
/// at `required-field-present` — the gate holds either way.
#[test]
fn finalize_blocks_an_omitted_owner_artifact() {
    let repo = TempDir::new("noartifact");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("host");
    seed_host_pack(pack.path());
    list_host_pack(repo.path(), pack.path());

    let task = "record-a-measured-dogfood-run";
    let addr = start_and_create(
        repo.path(),
        home.path(),
        "host-dogfood",
        "record a measured dogfood run",
        "dogfood-record",
        "Pilot Run",
    );
    author_record(repo.path(), home.path(), &addr, &["owner-artifact"]);
    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = streams(&out);
    assert!(
        !out.status.success(),
        "an omitted owner-artifact must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("field-value-conformant"),
        "the pre-stamped empty owner-artifact blocks at field-value-conformant; got:\n{rendered}",
    );
    assert!(
        rendered.contains("owner-artifact"),
        "the block names the omitted `owner-artifact` field; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
}

/// (d) A fully-authored record with a durably-staged owner-artifact finalizes,
/// promoting the record to its OWN `dogfood/` location dir.
#[test]
fn a_fully_authored_record_finalizes_promoting_to_dogfood() {
    let repo = TempDir::new("promote");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("host");
    seed_host_pack(pack.path());
    list_host_pack(repo.path(), pack.path());

    let task = "record-a-measured-dogfood-run";
    let addr = start_and_create(
        repo.path(),
        home.path(),
        "host-dogfood",
        "record a measured dogfood run",
        "dogfood-record",
        "Pilot Run",
    );
    author_record(repo.path(), home.path(), &addr, &[]);
    fill_commit(repo.path(), home.path(), task);

    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(&fin, "a complete dogfood-record must finalize clean");

    let promoted = repo.path().join("dogfood").join("pilot-run.md");
    assert!(
        promoted.is_file(),
        "finalize promotes the record to dogfood/pilot-run.md (its own location dir)",
    );
    // The promoted record is committed (the transactional boundary), not just on disk.
    let tracked = git(repo.path(), &["ls-files", "dogfood/pilot-run.md"]);
    assert!(
        !tracked.trim().is_empty(),
        "the promoted record is git-committed by finalize",
    );
}

/// (e) A repo holding a committed `completion-record` AND a committed
/// `dogfood-record` validates clean — the two schemas declare DISJOINT location
/// dirs, so the reconcile sweep never cross-blocks them.
#[test]
fn a_committed_completion_record_and_dogfood_record_coexist_validating_clean() {
    // The unshared-location fact itself, asserted at the schema level: the shipped
    // YAMLs declare different location dirs.
    let dogfood = load_schema(&schema_bytes("dogfood-record.yaml"))
        .expect("dogfood-record.yaml loads engine-native");
    let completion = load_schema(&schema_bytes("completion-record.yaml"))
        .expect("completion-record.yaml loads engine-native");
    assert_eq!(
        dogfood.location.as_deref(),
        Some("dogfood/"),
        "dogfood-record gets its OWN location subdir",
    );
    assert_ne!(
        dogfood.location, completion.location,
        "dogfood-record must NOT share completion-record's location dir — the reconcile \
         sweep parses every file in a location dir against every schema declaring it, \
         so a shared dir cross-blocks both",
    );

    let repo = TempDir::new("coexist");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let pack = TempDir::new("host");
    seed_host_pack(pack.path());
    list_host_pack(repo.path(), pack.path());

    // Task 1: author + finalize a completion-record (promoted to docs/completions/).
    let task1 = "record-a-milestone-completion";
    let addr1 = start_and_create(
        repo.path(),
        home.path(),
        "host-completion",
        "record a milestone completion",
        "completion-record",
        "M16",
    );
    let artifact = "completions/artifacts/M16/audit.md";
    fs::create_dir_all(repo.path().join("completions/artifacts/M16")).expect("mk owned home");
    fs::write(repo.path().join(artifact), "the genuine audit transcript\n")
        .expect("write owner-artifact");
    git(repo.path(), &["add", artifact]);
    set_field(
        repo.path(),
        home.path(),
        &format!("{addr1}#meta/verdict"),
        "green",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("{addr1}#meta/owner-artifact"),
        artifact,
    );
    fill_commit(repo.path(), home.path(), task1);
    let fin1 = jigc(repo.path(), home.path(), &["task", "finalize", task1]);
    assert_ok(&fin1, "the completion-record finalize");

    // Task 2: author + finalize a dogfood-record (promoted to dogfood/) — its
    // preflight sweep already parses the committed docs/completions/ dir.
    let task2 = "record-a-measured-dogfood-run";
    let addr2 = start_and_create(
        repo.path(),
        home.path(),
        "host-dogfood",
        "record a measured dogfood run",
        "dogfood-record",
        "Pilot Run",
    );
    author_record(repo.path(), home.path(), &addr2, &[]);
    fill_commit(repo.path(), home.path(), task2);
    let fin2 = jigc(repo.path(), home.path(), &["task", "finalize", task2]);
    assert_ok(
        &fin2,
        "the dogfood-record finalize over a repo already holding a committed completion-record",
    );
    assert!(
        repo.path().join("completions").join("m16.md").is_file(),
        "the committed completion-record sits in docs/completions/",
    );
    assert!(
        repo.path().join("dogfood").join("pilot-run.md").is_file(),
        "the committed dogfood-record sits in dogfood/",
    );

    // Task 3: with BOTH records committed, a fresh task's validate sweep parses
    // both location dirs and reports clean.
    let task3 = "verify-the-record-store";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "host-dogfood",
                "verify the record store",
            ],
        ),
        "`jigc start` for the validating task",
    );
    fill_commit(repo.path(), home.path(), task3);
    let validate = jigc(repo.path(), home.path(), &["task", "validate", task3]);
    assert_ok(
        &validate,
        "a repo holding a committed completion-record AND a committed dogfood-record \
         validates clean (the unshared-location design holding through the binary)",
    );
}
