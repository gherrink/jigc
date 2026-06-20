//! End-to-end integration test for the `jigc doc <verb> <addr>` write surface.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo with a
//! started task, exercising the MVP write loop's primitives
//! (`design/write-commands.md` → The verbs / Content handoff / Worked example):
//! `set-field <addr> --value` (inline, adjudicated) and `set-slot <addr>
//! --from-file -` (prose via stdin). A malformed `--value` blocks with the typed
//! finding message + route on stderr and a non-zero exit.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init` (`jigc start` reads
//! HEAD), and a self-cleaning `TempDir` keeps the test off the developer's repo.
//! Staged bytes are asserted by reading the working-area instance directly (the
//! sibling `jigc task diff` surface lands in a later task; the staged buffer on
//! disk *is* what `task diff` would render).

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
            "jigc-doc-write-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves, then `jigc start --workflow single-task
/// "<intent>"` to mint a task and provision its commit doc (post-flip the cascade
/// default is the `router`, so minting goes through Form D). Returns the repo + a
/// `$HOME` temp dir.
fn started_repo(intent: &str) -> (TempDir, TempDir) {
    started_repo_on("single-task", intent)
}

/// As [`started_repo`], but minting the task on an explicit `--workflow <id>` — the
/// `plan` workflow (whose gate `allows-create: [{type: spec, as: spec}]`) is what the
/// item-authoring path needs to `jigc doc create spec` (`pack/workflows/plan.yaml`).
fn started_repo_on(workflow: &str, intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
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
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", workflow, intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home)
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
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

/// Read the staged commit instance from the task working area.
fn staged_commit(repo: &Path, task: &str) -> String {
    let path = repo
        .path_join(task)
        .unwrap_or_else(|| panic!("compute staged path"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// Helper trait sugar: the staged commit-doc path for a task.
trait PathJoin {
    fn path_join(&self, task: &str) -> Option<PathBuf>;
}
impl PathJoin for Path {
    fn path_join(&self, task: &str) -> Option<PathBuf> {
        Some(
            self.join(".jigc")
                .join("tasks")
                .join(task)
                .join("docs")
                .join(format!("commit:{task}.md")),
        )
    }
}

#[test]
fn set_field_block_renders_as_json_under_format_json() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // An invalid enum value for `type` is a blocking finding. Under `--format json`
    // the block must render as a JSON finding envelope, not plain text (an agent
    // running `--format json` expects a parseable error, not prose).
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "notatype",
            "--format",
            "json",
        ],
        None,
    );
    assert!(
        !out.status.success(),
        "an invalid enum value must block (non-zero exit)"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let trimmed = stderr.trim();
    assert!(
        trimmed.starts_with('{') || trimmed.starts_with('['),
        "the block renders as JSON under --format json; got:\n{stderr}"
    );
    assert!(
        trimmed.contains("\"findings\"") || trimmed.contains("\"message\""),
        "the JSON carries the finding envelope; got:\n{stderr}"
    );
}

#[test]
fn set_field_inline_stages_the_field_value() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        None,
    );
    assert!(
        out.status.success(),
        "`set-field ... --value feat` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let staged = staged_commit(repo.path(), task);
    assert!(
        staged.contains("type: feat"),
        "the staged commit doc must carry `type: feat`; got:\n{staged}"
    );
}

#[test]
fn set_field_prints_a_success_confirmation() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // Agent-text: a terse confirmation naming the address + the value that landed,
    // so the write outcome is visible without `cat`-ing the working-area file.
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        None,
    );
    assert!(
        out.status.success(),
        "`set-field` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains(&format!("commit:{task}#type")) && stdout.contains("feat"),
        "set-field confirms the address + value on stdout; got:\n{stdout}"
    );

    // JSON: a structured ack on stdout (the house serde-object shape), parseable.
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "fix",
            "--format",
            "json",
        ],
        None,
    );
    assert!(
        out.status.success(),
        "`set-field --format json` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let ack: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("set-field json ack parses");
    assert_eq!(ack["address"], format!("commit:{task}#type"));
    assert_eq!(ack["value"], "fix");
}

#[test]
fn set_slot_prints_a_success_confirmation() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";
    let prose = b"Add a per-client rate limiter at the gateway.\n";

    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
        ],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "`set-slot` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains(&format!("commit:{task}#summary")) && stdout.contains("char"),
        "set-slot confirms the address + a char count on stdout; got:\n{stdout}"
    );

    // JSON: a structured ack on stdout.
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--format",
            "json",
        ],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "`set-slot --format json` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let ack: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("set-slot json ack parses");
    assert_eq!(ack["address"], format!("commit:{task}#summary"));
    assert!(
        ack["chars"].is_number(),
        "json ack carries a char count; got:\n{stdout}"
    );
}

#[test]
fn set_slot_from_stdin_stages_the_prose() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";
    let prose = b"Add a per-client rate limiter at the gateway.\n";

    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
        ],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "`set-slot ... --from-file -` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let staged = staged_commit(repo.path(), task);
    assert!(
        staged.contains("Add a per-client rate limiter at the gateway."),
        "the staged commit doc must carry the piped slot prose; got:\n{staged}"
    );
}

#[test]
fn malformed_field_value_blocks_with_a_routed_finding() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "not-a-member",
        ],
        None,
    );
    assert!(
        !out.status.success(),
        "a non-member enum value must exit non-zero"
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("not-a-member") || stderr.to_lowercase().contains("enum"),
        "the block must name the rejected value or the enum constraint; got:\n{stderr}"
    );
    assert!(
        stderr.contains("route:"),
        "the block must carry a route directing the agent's next action; got:\n{stderr}"
    );

    // The malformed write must NOT have mutated the staged doc (no `type: not-a-member`).
    let staged = staged_commit(repo.path(), task);
    assert!(
        !staged.contains("not-a-member"),
        "a rejected write must persist nothing; got:\n{staged}"
    );
}

/// The shipped `spec` schema, loaded from the embedded pack source tree with the
/// dev-pack field types (`code-anchor` on `maps-to-test`), so the byte-stable
/// round-trip asserts against exactly the bytes that ship.
fn spec_schema() -> engine::schema::Schema {
    const SPEC_YAML: &[u8] = include_bytes!("../pack/schemas/spec.yaml");
    // The spec's `maps-to-test` is a pack-declared `code-anchor`, so the schema only
    // loads with that type threaded in (mirrors the shipped pack's `code-anchor →
    // doc-code` decl).
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
    }];
    engine::schema::load_schema_with_types(SPEC_YAML, &types).expect("spec.yaml loads")
}

/// The staged `spec:<slug>` instance path in the task working area.
fn staged_spec(repo: &Path, task: &str, slug: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("spec:{slug}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

#[test]
fn add_item_mints_a_repeatable_item_byte_stable() {
    let (repo, home) = started_repo_on("plan", "plan the auth flow");
    let task = "plan-the-auth-flow";

    // Provision a `spec` container via the create-gate (plan allows {type: spec}).
    let created = run_doc(
        repo.path(),
        home.path(),
        &["create", "spec", "--title", "Auth flow"],
        None,
    );
    assert!(
        created.status.success(),
        "`jigc doc create spec` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr)
    );

    // Mint a repeatable item into the `criteria` section by title.
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            "spec:auth-flow#criteria",
            "--title",
            "Rate limit holds",
        ],
        None,
    );
    assert!(
        out.status.success(),
        "`jigc doc add-item` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // The minted item address prints verbatim on stdout (the next address an agent
    // addresses the item's slot/field at).
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout.trim_end_matches('\n'),
        "spec:auth-flow#criteria/rate-limit-holds",
        "the minted item address prints on stdout; got:\n{stdout}"
    );

    // The staged spec carries exactly one item with the minted heading + anchor.
    let staged = staged_spec(repo.path(), task, "auth-flow");
    assert!(
        staged.contains("### Rate limit holds  {#rate-limit-holds}"),
        "the staged spec carries the minted `### …  {{#id}}` item; got:\n{staged}"
    );

    // Byte-stable: render(parse(staged)) == staged — the splice path is the parser's
    // inverse on the minted bytes (the #1-risk round-trip, proven through the binary).
    let schema = spec_schema();
    let instance =
        engine::write::instance_from_source(&schema, &staged).expect("staged spec re-parses");
    let rerendered = engine::write::render(&schema, &instance);
    assert_eq!(
        rerendered, staged,
        "the minted item is byte-stable across parse → render",
    );
}

/// Item-leaf addressing (M13 Increment 3 / T2): `set-slot`/`set-field` on an
/// item-scoped leaf (`spec:<slug>#<section>/<item>/<leaf>`) splices **that item's**
/// leaf, disambiguating between two items that carry identically-keyed leaves —
/// through the real binary. Two criteria items A + B are minted; B's `statement`
/// slot + `maps-to-test` field are set; the assertions prove (a) B's leaves are
/// exactly the new values, (b) A's identically-keyed leaves are byte-for-byte
/// untouched, (c) the whole authored doc round-trips `render(parse(staged)) ==
/// staged` (the mint-empty byte-stability the engine seam now holds). The inverse
/// (targeting A) closes the disambiguation in both directions.
#[test]
fn set_slot_and_field_target_the_addressed_item_leaf() {
    for target_is_b in [true, false] {
        let (repo, home) = started_repo_on("plan", "plan the auth flow");
        let task = "plan-the-auth-flow";

        let created = run_doc(
            repo.path(),
            home.path(),
            &["create", "spec", "--title", "Auth flow"],
            None,
        );
        assert!(
            created.status.success(),
            "`jigc doc create spec` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&created.stderr)
        );

        // Two criteria items A + B (each carries a `statement` slot + a `maps-to-test`
        // field — identically-keyed leaves the item hop must disambiguate).
        for title in ["Criterion A", "Criterion B"] {
            let out = run_doc(
                repo.path(),
                home.path(),
                &["add-item", "spec:auth-flow#criteria", "--title", title],
                None,
            );
            assert!(
                out.status.success(),
                "`jigc doc add-item {title}` must exit 0; stderr:\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }

        // The targeted item + the untouched sibling.
        let (target, other) = if target_is_b {
            ("criterion-b", "criterion-a")
        } else {
            ("criterion-a", "criterion-b")
        };
        let statement = b"The targeted item's statement, set through the item hop.\n";
        let anchor = "`crates/x.rs#f`";

        // set-slot on the targeted item's `statement` leaf.
        let slot = run_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                &format!("spec:auth-flow#criteria/{target}/statement"),
                "--from-file",
                "-",
            ],
            Some(statement),
        );
        assert!(
            slot.status.success(),
            "`set-slot` on an item leaf must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&slot.stderr)
        );

        // set-field on the targeted item's `maps-to-test` leaf.
        let field = run_doc(
            repo.path(),
            home.path(),
            &[
                "set-field",
                &format!("spec:auth-flow#criteria/{target}/maps-to-test"),
                "--value",
                anchor,
            ],
            None,
        );
        assert!(
            field.status.success(),
            "`set-field` on an item leaf must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&field.stderr)
        );

        let staged = staged_spec(repo.path(), task, "auth-flow");

        // (a) The targeted item carries exactly the new leaf values.
        assert!(
            staged.contains("The targeted item's statement, set through the item hop."),
            "the targeted item's `statement` slot carries the new prose; got:\n{staged}"
        );
        assert!(
            staged.contains("maps-to-test: `crates/x.rs#f`"),
            "the targeted item's `maps-to-test` field carries the new value; got:\n{staged}"
        );

        // (b) The untouched sibling's identically-keyed leaves stay byte-for-byte the
        // mint-empty form (no statement prose, no maps-to-test value leaked across).
        let schema = spec_schema();
        let parsed =
            engine::write::instance_from_source(&schema, &staged).expect("staged spec re-parses");
        let crit = parsed
            .sections
            .iter()
            .find(|s| s.id == "criteria")
            .expect("criteria section");
        let other_item = crit
            .items
            .iter()
            .find(|i| i.id == other)
            .unwrap_or_else(|| panic!("sibling item {other} present"));
        assert!(
            other_item.slot.as_deref().unwrap_or("").trim().is_empty(),
            "the sibling item's `statement` slot stays empty (item hop disambiguated); got:\n{staged}"
        );
        assert!(
            !other_item.fields.iter().any(|f| f.key == "maps-to-test"),
            "the sibling item carries no `maps-to-test` (the field did not leak); got:\n{staged}"
        );

        // (c) The whole authored doc round-trips byte-for-byte (the mint-empty seam
        // the engine now holds — render(parse(staged)) == staged).
        let rerendered = engine::write::render(&schema, &parsed);
        assert_eq!(
            rerendered, staged,
            "the item-leaf-authored spec is byte-stable across parse → render",
        );
    }
}

#[test]
fn add_item_into_a_non_repeatable_section_blocks_with_a_routed_finding() {
    let (repo, home) = started_repo_on("plan", "plan the auth flow");

    let created = run_doc(
        repo.path(),
        home.path(),
        &["create", "spec", "--title", "Auth flow"],
        None,
    );
    assert!(created.status.success(), "create spec");

    // `goal` is a simple slot section, not repeatable — adding an item must block
    // with a routed finding, never panic.
    let out = run_doc(
        repo.path(),
        home.path(),
        &["add-item", "spec:auth-flow#goal", "--title", "Bogus"],
        None,
    );
    assert!(
        !out.status.success(),
        "an add-item into a non-repeatable section must exit non-zero"
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        stderr.to_lowercase().contains("repeatable") || stderr.contains("goal"),
        "the block names the wrong-shape reason; got:\n{stderr}"
    );
    assert!(
        stderr.contains("route:"),
        "the block carries a route directing the agent's next action; got:\n{stderr}"
    );
}

/// The shipped `changelog` schema (engine-native types only), so the byte-stable
/// round-trip asserts against exactly the bytes the pack ships.
fn changelog_schema() -> engine::schema::Schema {
    const CHANGELOG_YAML: &[u8] = include_bytes!("../pack/schemas/changelog.yaml");
    engine::schema::load_schema(CHANGELOG_YAML).expect("changelog.yaml loads")
}

/// The staged `changelog:changelog` singleton instance in the task working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// M24 inc-2 T2 — the item-field parity gap is closed: a `set-field` on a repeatable
/// **item** field whose value fails its declared type is rejected **at the write verb**
/// (not deferred to finalize), carrying the finding code finalize's item-field path
/// emits (`schema-conformance.field-value-conformant`). The changelog release `date`
/// (`type date`) is the one malformable item bullet field; a non-ISO value blocks, a
/// valid ISO value passes and round-trips byte-stable. Driven over the real binary so
/// the block is the emitted contract.
#[test]
fn malformed_item_field_date_blocks_at_the_set_field_verb() {
    let (repo, home) = started_repo_on("record-change", "cut the release");
    let task = "cut-the-release";

    let created = run_doc(
        repo.path(),
        home.path(),
        &["create", "changelog", "--title", "Changelog"],
        None,
    );
    assert!(
        created.status.success(),
        "`jigc doc create changelog` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr)
    );

    // Mint a release item (title = the version string). The `date` materializes
    // on-create; we then overwrite it via `set-field`. The minted item address prints
    // on stdout — the address an agent next targets the `date` leaf at.
    let added = run_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.0.0",
        ],
        None,
    );
    assert!(
        added.status.success(),
        "`jigc doc add-item …#releases` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&added.stderr)
    );
    let item_addr = String::from_utf8_lossy(&added.stdout)
        .trim_end_matches('\n')
        .to_string();
    let date_addr = format!("{item_addr}/date");

    // A non-ISO date must block at the write verb, naming finalize's item-field code.
    let blocked = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &date_addr,
            "--value",
            "June 16, 2026",
            "--format",
            "json",
        ],
        None,
    );
    assert!(
        !blocked.status.success(),
        "a non-ISO item-field date must block at the set-field verb (non-zero exit); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr);
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("stderr is JSON: {e}; got:\n{stderr}"));
    assert_eq!(
        report["findings"][0]["code"], "schema-conformance.field-value-conformant",
        "the block carries the code finalize's item-field path emits; got:\n{stderr}",
    );

    // The rejected write must persist nothing — the staged release carries no bad date.
    assert!(
        !staged_changelog(repo.path(), task).contains("June 16, 2026"),
        "a rejected item-field write must persist nothing",
    );

    // A valid ISO date passes and round-trips byte-stable.
    let ok = run_doc(
        repo.path(),
        home.path(),
        &["set-field", &date_addr, "--value", "2026-06-16"],
        None,
    );
    assert!(
        ok.status.success(),
        "a valid ISO item-field date must pass; stderr:\n{}",
        String::from_utf8_lossy(&ok.stderr)
    );

    let staged = staged_changelog(repo.path(), task);
    assert!(
        staged.contains("date: 2026-06-16"),
        "the staged release carries the new ISO date; got:\n{staged}"
    );

    let schema = changelog_schema();
    let instance =
        engine::write::instance_from_source(&schema, &staged).expect("staged changelog re-parses");
    let rerendered = engine::write::render(&schema, &instance);
    assert_eq!(
        rerendered, staged,
        "the item-field-edited changelog is byte-stable across parse → render",
    );
}
