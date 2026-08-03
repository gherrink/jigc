//! M47 Increment 5 / T1 — the **commit-doc solicitation axis**, swept over the
//! whole **dev pack registry** ([pinning.md](../../../implementation/pinning.md) §1:
//! an axis test enumerates its members from the registry, never from a hand list).
//!
//! **The class** (`DECISIONS.md` → 2026-07-26 the Settle, Decision 5; N6): the commit
//! doc is the one doc *every* task-minting workflow's finalize gate blocks on, yet
//! the solicitation for its two required leaves lived inside the **code-writing**
//! steps (`step:implement` / `step:implement-quick`) and inside `sub-task`'s own
//! `step:author-commit`. So a workflow that reaches `step:finalize` without writing
//! code — `architecture-documentation`, `plan`, `record-change`, `record-decision`,
//! and `project-setup` through `project-finalize.yaml` — composed text that named
//! every write **except** the two its gate would refuse on: a law-1 ambush
//! (`design/surface-contract.md` → law 1 / The stated-at fence). `sub-task` had the
//! opposite defect — it solicited **twice**.
//!
//! **The axis is "every dev workflow", and the expectation is a derived function,
//! never a table.** For each workflow the pack ships:
//!
//!   * `creates-task: false` → the workflow mints no task and provisions no commit
//!     doc, so it must solicit **zero** times;
//!   * a **migrate** workflow (`migrate-<doctype>` for a doctype the same pack
//!     ships, cross-checked by its `step:migration-finalize` include — the two
//!     derivations are asserted to agree, so neither can rot alone) → the mint
//!     *pre-fills* the commit doc from a deterministic template
//!     (`crates/cli/src/start.rs` → `provision_migration_commit_doc`), so it too
//!     must solicit **zero** times;
//!   * every other task-minting workflow → **exactly once** per required leaf.
//!
//! **The required leaves come from the pack's own `commit` schema**, not from this
//! file: every author-required header field and every non-optional slot section
//! (`engine::validate::is_author_required` — the shared predicate the mint, the
//! write path and `doc schema` all read). Today that resolves to `type` + `summary`;
//! a schema change moves the axis without touching the test.
//!
//! **Two arms, because counting is not following.**
//!
//!   1. **The count** — `jigc workflow <id> --preview` for every workflow (the
//!      mint-free read of exactly the text an operator would be handed), counting
//!      the *emitted* write commands per required leaf.
//!   2. **Followability** — for every workflow the count expects at one:
//!      `jigc start --workflow <id>` in a fresh repo, then the composed write
//!      commands **executed verbatim as printed** (the emitted bytes are the
//!      contract — the only substitution is the author-owned payload: the
//!      `<COMMIT_TYPE>` placeholder and the piped slot prose), then
//!      `jigc task validate <id>` must report **no finding targeting `commit:<id>`**.
//!      A composed text that names the writes but names them wrongly passes arm 1
//!      and fails here.
//!
//! Everything drives the **real binary** against the on-disk dev pack
//! (`JIGC_PACK_DIR = crates/cli/pack`), which is also the registry this suite
//! enumerates — surface and enumeration cannot drift.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use cli::pack::{FilesystemPack, load_pack_schema};
use engine::compose::load_workflow_def;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::SectionBody;
use engine::validate::is_author_required;
use serde_json::Value;

/// The step every migrate workflow includes — the structural cross-check on the
/// `migrate-<doctype>` name derivation.
const MIGRATION_FINALIZE: &str = "migration-finalize";

/// The identity `--task <id>` renders as in a mint-free preview.
const PREVIEW_TASK: &str = "your-task-id";

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-commit-solicit-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The on-disk dev pack — both the surface the binary composes and the registry
/// this suite enumerates.
fn pack_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The directory holding the built `jigc`, prepended to `PATH` so a composed
/// command line can be executed **verbatim** (it names the bare `jigc`).
fn bin_dir() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_jigc"))
        .parent()
        .expect("the built binary has a parent directory")
        .to_path_buf()
}

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

/// A real git repo with one commit and the `.jigc/config/` project layer, plus
/// `jigc setup` run, so the cascade resolves and the dev pack composes alone.
fn init_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    std::fs::write(repo.join("README.md"), "# repo\n").expect("write readme");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    std::fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    let out = jigc(repo, home, &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Run `jigc <args>` against the on-disk dev pack.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir())
        .output()
        .expect("run the jigc binary")
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

// ---------------------------------------------------------------------------
// The registry-derived axis
// ---------------------------------------------------------------------------

/// Every required `commit` leaf, read from the pack's **own** `commit` schema: an
/// author-required header field, or a non-optional slot section. The id is the
/// address tail the composed write names (`commit:<id>#<leaf>`).
fn required_commit_leaves(pack: &dyn PackSource) -> Vec<String> {
    let bytes = pack
        .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
        .expect("the pack ships a `commit` schema");
    let schema = load_pack_schema(pack, &bytes).expect("the `commit` schema loads");
    let mut leaves = Vec::new();
    for section in &schema.sections {
        let SectionBody::Simple { slot, fields } = &section.body else {
            // A repeatable section carries no required leaf: item presence is
            // never author-required (`trailers`).
            continue;
        };
        if slot.as_ref().is_some_and(|s| !s.optional) {
            leaves.push(section.id.clone());
        }
        for field in fields {
            if is_author_required(field) {
                leaves.push(field.id.clone());
            }
        }
    }
    assert!(
        !leaves.is_empty(),
        "the `commit` schema must declare at least one required leaf",
    );
    leaves
}

/// The first member of the `commit` header's `type` enum — the value the composed
/// `<COMMIT_TYPE>` placeholder stands for, read from the schema rather than typed
/// here.
fn first_commit_type(pack: &dyn PackSource) -> String {
    let bytes = pack
        .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
        .expect("the pack ships a `commit` schema");
    let schema = load_pack_schema(pack, &bytes).expect("the `commit` schema loads");
    for section in &schema.sections {
        let SectionBody::Simple { fields, .. } = &section.body else {
            continue;
        };
        for field in fields {
            if field.id == "type" {
                return field
                    .of
                    .as_ref()
                    .and_then(|members| members.first())
                    .expect("the `type` enum declares members")
                    .clone();
            }
        }
    }
    panic!("the `commit` schema declares a `type` field");
}

/// One swept workflow: its id, whether it mints a task, and whether it is a
/// migrate workflow (whose mint pre-fills the commit doc).
struct Workflow {
    id: String,
    creates_task: bool,
    migrate: bool,
}

impl Workflow {
    /// The number of times each required `commit` leaf must be solicited.
    fn expected_solicits(&self) -> usize {
        usize::from(self.creates_task && !self.migrate)
    }
}

/// Every workflow the dev pack ships, with the expectation function applied.
///
/// The migrate discrimination is derived **twice** and the two must agree: by name
/// (`migrate-<doctype>` over the pack's own schema list) and structurally (the
/// `step:migration-finalize` include). Either alone could rot silently.
fn dev_workflows(pack: &dyn PackSource) -> Vec<Workflow> {
    let doctypes: Vec<String> = pack
        .list(PackResourceKind::Schemas)
        .into_iter()
        .map(|id| id.as_str().to_string())
        .collect();
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = pack
            .read(PackResourceKind::Workflows, &id)
            .expect("workflow is readable");
        let def = load_workflow_def(&bytes).expect("workflow front-matter parses");
        let id = id.as_str().to_string();
        let by_name = doctypes.iter().any(|ty| id == format!("migrate-{ty}"));
        let by_include = def.includes.iter().any(|step| step == MIGRATION_FINALIZE);
        assert_eq!(
            by_name, by_include,
            "the two migrate derivations disagree on `{id}`: name says {by_name}, \
             the `step:{MIGRATION_FINALIZE}` include says {by_include} — one of them has rotted",
        );
        out.push(Workflow {
            id,
            creates_task: def.creates_task,
            migrate: by_name,
        });
    }
    assert!(!out.is_empty(), "the dev pack ships workflows");
    out
}

// ---------------------------------------------------------------------------
// Counting the emitted writes
// ---------------------------------------------------------------------------

/// Every emitted line that is a `jigc doc set-…` write against
/// `commit:<task>#<leaf>` — the composed surface's own bytes, with the `Run: ` /
/// backtick decoration stripped so the remainder is executable verbatim.
fn emitted_writes(text: &str, task: &str, leaf: &str) -> Vec<String> {
    let target = format!("commit:{task}#{leaf} ");
    text.lines()
        .filter(|line| line.contains(&target))
        .filter_map(|line| {
            let trimmed = line.trim();
            let command = trimmed
                .strip_prefix("Run: `")
                .and_then(|rest| rest.strip_suffix('`'))
                .unwrap_or(trimmed);
            command
                .starts_with("jigc doc set-")
                .then(|| command.to_string())
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Arm 1 — the count over every dev workflow
// ---------------------------------------------------------------------------

#[test]
fn every_dev_workflow_solicits_each_required_commit_leaf_exactly_as_its_gate_demands() {
    let pack = FilesystemPack::new(pack_dir());
    let leaves = required_commit_leaves(&pack);
    let workflows = dev_workflows(&pack);

    let home = TempDir::new("count-home");
    let repo = TempDir::new("count-repo");
    init_repo(repo.path(), home.path());

    let mut wrong = Vec::new();
    let (mut expecting_one, mut expecting_zero) = (0usize, 0usize);
    for workflow in &workflows {
        let expected = workflow.expected_solicits();
        if expected == 1 {
            expecting_one += 1;
        } else {
            expecting_zero += 1;
        }
        let out = jigc(
            repo.path(),
            home.path(),
            &["workflow", &workflow.id, "--preview"],
        );
        let text = stdout_of(&out);
        for leaf in &leaves {
            let found = emitted_writes(&text, PREVIEW_TASK, leaf).len();
            if found != expected {
                wrong.push(format!(
                    "  {} — `#{leaf}`: expected {expected}, composed {found}",
                    workflow.id,
                ));
            }
        }
    }

    // The sweep must discriminate: a partition with an empty side proves nothing.
    assert!(
        expecting_one > 0 && expecting_zero > 0,
        "the axis must have members on both sides; got {expecting_one} at one \
         and {expecting_zero} at zero",
    );
    assert!(
        wrong.is_empty(),
        "every dev workflow must solicit each required `commit` leaf exactly as many \
         times as its gate demands (once when it mints a task and does not pre-fill \
         the commit doc, zero otherwise); offenders:\n{}",
        wrong.join("\n"),
    );
}

// ---------------------------------------------------------------------------
// Arm 2 — following the composed text to a commit-clean validate
// ---------------------------------------------------------------------------

/// Execute one composed write command **verbatim**, substituting only the
/// author-owned payload: the `<COMMIT_TYPE>` placeholder and, for a slot write,
/// the prose piped to `--from-file -`.
fn run_composed(repo: &Path, home: &Path, command: &str, commit_type: &str, prose: &str) {
    let filled = command.replace("<COMMIT_TYPE>", commit_type);
    assert!(
        !filled.contains('<'),
        "an unsubstituted placeholder survives in the composed command `{filled}` — \
         it would be read as a shell redirect, and this suite would stop executing \
         the emitted bytes",
    );
    let mut path = bin_dir().into_os_string();
    if let Some(existing) = std::env::var_os("PATH") {
        path.push(":");
        path.push(existing);
    }
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&filled)
        .current_dir(repo)
        .env("HOME", home)
        .env("PATH", path)
        .env("JIGC_PACK_DIR", pack_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the composed command");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(prose.as_bytes())
        .expect("write the authored prose");
    let out = child
        .wait_with_output()
        .expect("wait for the composed command");
    assert!(
        out.status.success(),
        "the composed command `{filled}` must run as printed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

#[test]
fn following_the_composed_text_leaves_no_commit_finding() {
    let pack = FilesystemPack::new(pack_dir());
    let leaves = required_commit_leaves(&pack);
    let commit_type = first_commit_type(&pack);
    let workflows: Vec<Workflow> = dev_workflows(&pack)
        .into_iter()
        .filter(|w| w.expected_solicits() == 1)
        .collect();
    assert!(
        !workflows.is_empty(),
        "the followability arm must cover the task-minting non-migrate workflows",
    );

    let mut offenders = Vec::new();
    for workflow in &workflows {
        let home = TempDir::new(&format!("follow-home-{}", workflow.id));
        let repo = TempDir::new(&format!("follow-repo-{}", workflow.id));
        init_repo(repo.path(), home.path());

        let intent = format!("exercise {}", workflow.id);
        let out = jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", &workflow.id, &intent],
        );
        assert!(
            out.status.success(),
            "`jigc start --workflow {}` must succeed; stderr:\n{}",
            workflow.id,
            String::from_utf8_lossy(&out.stderr),
        );
        let composed = stdout_of(&out);
        let task = composed
            .lines()
            .find_map(|line| line.strip_prefix("task minted: "))
            .expect("the mint names the task id")
            .trim()
            .to_string();

        for leaf in &leaves {
            let writes = emitted_writes(&composed, &task, leaf);
            assert_eq!(
                writes.len(),
                1,
                "`{}` must compose exactly one write for `#{leaf}`; composed:\n{composed}",
                workflow.id,
            );
            run_composed(
                repo.path(),
                home.path(),
                &writes[0],
                &commit_type,
                "record what this change does\n",
            );
        }

        let validated = jigc(
            repo.path(),
            home.path(),
            &["--format", "json", "task", "validate", &task],
        );
        let envelope: Value = serde_json::from_slice(&validated.stdout)
            .expect("`task validate --format json` emits JSON");
        let commit_findings: Vec<String> = envelope["findings"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .filter_map(|finding| {
                let target = finding["key"]["target"].as_str()?;
                target
                    .starts_with(&format!("commit:{task}"))
                    .then(|| format!("{} → {target}", finding["code"].as_str().unwrap_or("?")))
            })
            .collect();
        if !commit_findings.is_empty() {
            offenders.push(format!(
                "  {} — {}",
                workflow.id,
                commit_findings.join(", ")
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "following each workflow's composed commit-doc writes verbatim must leave the \
         commit doc clean at `jigc task validate`; still finding:\n{}",
        offenders.join("\n"),
    );
}
