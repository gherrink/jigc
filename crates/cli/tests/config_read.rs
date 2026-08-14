//! M48 Increment 6, T1 — **the cascade surface grows its read rung**: `jigc config
//! get <key>` and `jigc config list` (`design/overrides.md` → Reading the resolved
//! cascade; `DECISIONS.md` → 2026-08-13 the Settle, F8).
//!
//! Until now `jigc config` was **write-only**: six authoring verbs and no way to ask
//! what a knob currently resolves to, or which knobs exist at all. The closed surface
//! was enumerated in exactly one place — inside `config set`'s *rejection* message —
//! so the only way to read the cascade was to fail a write against it.
//!
//! The four facts these arms drive through the real binary:
//!
//!   * **`config list` is the closed surface**, not a curated excerpt: its emitted key
//!     set is asserted **set-equal** to `engine::knobs::load_knobs`' declared set for
//!     the composed pack — never a count, never a hand list (`pinning.md` §1). A knob
//!     added to `knobs.yaml` joins the surface with no edit here; one dropped reddens.
//!   * **`config get` names the winning layer**, so a value that looks wrong is
//!     attributable: `pack-default` for an untouched knob, `project` for one an
//!     applied `scalar-set` won.
//!   * **A soft-rejected set is visible from the read side.** A `scalar-set` below its
//!     knob's `floor` is *dropped*, and resolution continues as if it were absent
//!     (`overrides.md` → Soft-rejection) — the state most likely to read as "my
//!     override did nothing". `config get` names the attempted value **and** the floor
//!     it ranked below, so the drop is legible instead of silent.
//!   * **An undeclared key is a routed rejection**, carrying `config.undeclared-key`
//!     and routing to `jigc config list` — the enumeration that used to live only in
//!     the rejection text now has a verb of its own to point at.
//!
//! Both verbs are also driven under `--format json` and asserted to emit **exactly one
//! JSON document on stdout** (`design/command-output-contract.md` → Stream discipline).

use engine::knobs::load_knobs;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-config-read-{tag}-{}-{:?}",
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

/// The knob an applied project `scalar-set` wins — a tunable (floor-less) bool.
const APPLIED_KEY: &str = "invocation-log";
/// The knob a **below-floor** project `scalar-set` targets: an intrinsic check whose
/// `floor: blocking` soft-rejects the `advisory` demotion below.
const FLOORED_KEY: &str = "validation.workflow-refs.placeholder-resolves.severity";

/// Initialize a real git repo with a `.jigc/config/` project layer carrying **one
/// applied override** (`invocation-log: true`) and **one below-floor set** (an
/// `advisory` demotion of a `floor: blocking` intrinsic check, which resolution drops).
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

    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("create project layer");
    fs::write(
        config.join("manifest.yaml"),
        format!("scalar:\n  {APPLIED_KEY}: \"true\"\n  {FLOORED_KEY}: advisory\n"),
    )
    .expect("write project manifest");
}

/// Run `jigc config <args>` with `cwd = repo` and `$HOME = home`.
fn run_config(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("config");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

fn stderr_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("utf-8 stderr")
}

/// Whether `s` parses as **exactly one** JSON document — the `machine_output.rs` /
/// `format_json_success_axis.rs` predicate (trailing non-whitespace fails, so a
/// document plus a plain-text side channel is not one document).
fn is_one_json_doc(s: &str) -> bool {
    serde_json::from_str::<Value>(s).is_ok()
}

/// The declared knob set, read from the **pack** the binary composes for this repo
/// (no `packs.yaml`, no compose marker → the embedded dev pack alone) through the
/// same `engine::knobs::load_knobs` the cascade seeds itself from. The reference set
/// for the set-equality below — a hand list here would prove nothing.
fn declared_knob_keys() -> BTreeSet<String> {
    let pack = cli::pack::EmbeddedPack::new();
    let bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
        .expect("the embedded pack ships `config/knobs`");
    let knobs = load_knobs(&bytes).expect("`config/knobs` parses");
    knobs.keys().map(str::to_owned).collect()
}

/// **The closed surface, read back whole.** `config list`'s emitted key set is the
/// declared knob set — set-equality against `load_knobs`, never a count, so a knob
/// added to `knobs.yaml` joins with no edit here and one dropped reddens.
#[test]
fn config_list_emits_exactly_the_declared_knob_set() {
    let repo = TempDir::new("list");
    let home = TempDir::new("list-home");
    init_repo(repo.path());

    let out = run_config(repo.path(), home.path(), &["list", "--format", "json"]);
    let stdout = stdout_of(&out);
    let stderr = stderr_of(&out);
    assert!(
        out.status.success(),
        "`jigc config list` must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    let doc: Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|err| panic!("stdout must be one JSON document ({err}); got:\n{stdout}"));
    let emitted: BTreeSet<String> = doc["knobs"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `knobs` array; got:\n{stdout}"))
        .iter()
        .map(|row| {
            row["key"]
                .as_str()
                .unwrap_or_else(|| panic!("each row carries a `key`; got:\n{stdout}"))
                .to_owned()
        })
        .collect();

    let declared = declared_knob_keys();
    let missing: Vec<_> = declared.difference(&emitted).collect();
    let extra: Vec<_> = emitted.difference(&declared).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "`config list` must emit EXACTLY the declared knob surface.\n\
         declared but not listed: {missing:?}\nlisted but not declared: {extra:?}",
    );
}

/// The untouched knob: `config get` prints the resolved value **and** names the layer
/// that owns it — `pack-default`, because no override layer set it.
#[test]
fn config_get_names_the_resolved_value_and_its_pack_default_layer() {
    let repo = TempDir::new("get-default");
    let home = TempDir::new("get-default-home");
    init_repo(repo.path());

    let out = run_config(repo.path(), home.path(), &["get", "docs-root"]);
    let stdout = stdout_of(&out);
    assert!(
        out.status.success(),
        "`jigc config get docs-root` must exit 0; stderr:\n{}",
        stderr_of(&out),
    );
    assert!(
        stdout.contains("docs/"),
        "`config get docs-root` must print the resolved value `docs/`; got:\n{stdout}",
    );
    assert!(
        stdout.contains("pack-default"),
        "`config get docs-root` must name the winning layer `pack-default`; got:\n{stdout}",
    );
}

/// The overridden knob: the winning layer is `project`, so a value that surprises the
/// reader is attributable to the layer that set it.
#[test]
fn config_get_names_the_project_layer_for_an_applied_override() {
    let repo = TempDir::new("get-project");
    let home = TempDir::new("get-project-home");
    init_repo(repo.path());

    let out = run_config(repo.path(), home.path(), &["get", APPLIED_KEY]);
    let stdout = stdout_of(&out);
    assert!(
        out.status.success(),
        "`jigc config get {APPLIED_KEY}` must exit 0; stderr:\n{}",
        stderr_of(&out),
    );
    assert!(
        stdout.contains("true"),
        "the applied override's resolved value `true` must print; got:\n{stdout}",
    );
    assert!(
        stdout.contains("project"),
        "an applied `scalar-set` must name the `project` layer; got:\n{stdout}",
    );
}

/// **The soft-rejection is legible from the read side.** A below-floor `scalar-set` is
/// dropped and resolution continues as if it were absent — the state that otherwise
/// reads as "my override did nothing". `config get` names the **attempted value** and
/// the **floor** it ranked below, on both surfaces.
#[test]
fn config_get_names_the_attempted_value_and_the_floor_of_a_soft_rejected_set() {
    let repo = TempDir::new("get-floored");
    let home = TempDir::new("get-floored-home");
    init_repo(repo.path());

    let text = run_config(repo.path(), home.path(), &["get", FLOORED_KEY]);
    let stdout = stdout_of(&text);
    assert!(
        text.status.success(),
        "`jigc config get {FLOORED_KEY}` must exit 0; stderr:\n{}",
        stderr_of(&text),
    );
    assert!(
        stdout.contains("advisory"),
        "the soft-rejected set's ATTEMPTED value `advisory` must print; got:\n{stdout}",
    );
    assert!(
        stdout.contains("floor"),
        "the reading must name the `floor` the set ranked below; got:\n{stdout}",
    );

    let json = run_config(
        repo.path(),
        home.path(),
        &["get", FLOORED_KEY, "--format", "json"],
    );
    let out = stdout_of(&json);
    let doc: Value = serde_json::from_str(&out)
        .unwrap_or_else(|err| panic!("stdout must be one JSON document ({err}); got:\n{out}"));
    assert_eq!(
        doc["value"].as_str(),
        Some("blocking"),
        "the dropped set leaves the pack-default value resolved; got:\n{out}",
    );
    assert_eq!(
        doc["rejected"]["attempted"].as_str(),
        Some("advisory"),
        "the envelope carries the attempted value; got:\n{out}",
    );
    assert_eq!(
        doc["rejected"]["floor"].as_str(),
        Some("blocking"),
        "the envelope carries the floor it ranked below; got:\n{out}",
    );
    assert_eq!(
        doc["rejected"]["layer"].as_str(),
        Some("project"),
        "the envelope names the layer that attempted the set; got:\n{out}",
    );
}

/// An undeclared key is a **routed rejection**: non-zero, carrying the stable
/// `config.undeclared-key` code, routing to the verb that enumerates the surface.
#[test]
fn config_get_of_an_undeclared_key_is_a_routed_rejection() {
    let repo = TempDir::new("get-undeclared");
    let home = TempDir::new("get-undeclared-home");
    init_repo(repo.path());

    let out = run_config(repo.path(), home.path(), &["get", "not-a-knob"]);
    let stdout = stdout_of(&out);
    let stderr = stderr_of(&out);
    assert!(
        !out.status.success(),
        "an undeclared key must exit non-zero; stdout:\n{stdout}",
    );
    assert!(
        stdout.trim().is_empty(),
        "a reject must leave stdout empty; got:\n{stdout}",
    );
    assert!(
        stderr.contains("config.undeclared-key"),
        "the rejection must carry its stable code; got:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc config list"),
        "the route must name `jigc config list` — the verb that enumerates the closed \
         surface; got:\n{stderr}",
    );
}

/// Stream discipline on the read rung's success path: **exactly one** JSON document on
/// stdout, none on stderr, for both verbs.
#[test]
fn both_read_verbs_emit_exactly_one_json_document_on_stdout() {
    let repo = TempDir::new("json");
    let home = TempDir::new("json-home");
    init_repo(repo.path());

    for args in [
        vec!["get", "docs-root", "--format", "json"],
        vec!["list", "--format", "json"],
    ] {
        let out = run_config(repo.path(), home.path(), &args);
        let stdout = stdout_of(&out);
        let stderr = stderr_of(&out);
        assert!(
            out.status.success(),
            "`jigc config {args:?}` must exit 0; stderr:\n{stderr}",
        );
        assert!(
            is_one_json_doc(&stdout),
            "`jigc config {args:?}` must emit exactly one JSON document on stdout; got:\n{stdout}",
        );
        assert!(
            !is_one_json_doc(&stderr),
            "`jigc config {args:?}` must not put a JSON document on stderr; got:\n{stderr}",
        );
    }
}
