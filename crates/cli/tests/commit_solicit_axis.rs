//! M47 Increment 5 — the **commit-doc solicitation axis**, swept over the whole
//! **composite registry**: every workflow **both** shipped packs carry
//! ([pinning.md](../../../implementation/pinning.md) §1: an axis test enumerates its
//! members from the registry, never from a hand list).
//!
//! **The class** (`DECISIONS.md` → 2026-07-26 the Settle, Decision 5; N6): the commit
//! doc is the one doc *every* task-minting workflow's finalize gate blocks on, yet
//! each pack solicited its two required leaves from the wrong home. In the dev pack
//! the solicit lived inside the **code-writing** steps (`step:implement` /
//! `step:implement-quick`) and inside `sub-task`'s own `step:author-commit`, so the
//! five workflows reaching `step:finalize` without writing code composed text naming
//! every write **except** the two their gate would refuse on (a law-1 ambush —
//! `design/surface-contract.md` → law 1 / The stated-at fence) while `sub-task`
//! solicited **twice**. In the methodology pack the solicit lived inside
//! `step:finalize` itself, which `step:migration-finalize` includes — so all seven
//! methodology migrate workflows solicited a commit doc their **mint already
//! pre-filled**.
//!
//! **S1 (the Settle's cross-pack rule): both packs carry the change or it lands on
//! half the surface** — which is why this suite's axis is the composite registry and
//! not either pack alone. It runs against the `[dev ▸ methodology]` pack-set a plain
//! `jigc setup` installs (no `JIGC_PACK_DIR`, no `packs.yaml`) — the set a dogfooding
//! project actually runs, and the one where per-origin-pack include resolution is
//! live: methodology's `dev-task` composes *methodology's* `step:author-commit` even
//! though dev ships a step of the same id.
//!
//! **The axis is "every shipped workflow", and the expectation is a derived function,
//! never a table.** For each workflow, in its own origin pack:
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
//! **The required leaves come from each origin pack's own `commit` schema**, not from
//! this file: every author-required header field and every non-optional slot section
//! (`engine::validate::is_author_required` — the shared predicate the mint, the write
//! path and `doc schema` all read). Today that resolves to `type` + `summary` in both
//! packs; a schema change moves the axis without touching the test.
//!
//! **Two arms, because counting is not following.**
//!
//!   1. **The count** — `jigc workflow <id> --preview` for every workflow (the
//!      mint-free read of exactly the text an operator would be handed), counting
//!      the *emitted* write commands per required leaf.
//!   2. **Followability** — for every workflow the count expects at one:
//!      `jigc start --workflow <id>` in a fresh repo, then the composed write
//!      commands **executed verbatim as printed** (the emitted bytes are the
//!      contract — the only substitution is the author-owned payload: the value
//!      placeholder, read off the emitted bytes because the two packs spell it
//!      differently (`<COMMIT_TYPE>` / `<TYPE>`), and the piped slot prose), then
//!      `jigc task validate <id>` must report **no finding targeting `commit:<id>`**.
//!      A composed text that names the writes but names them wrongly passes arm 1
//!      and fails here.
//!
//! Everything drives the **real binary** against the two **embedded** packs, which
//! are also the registry this suite enumerates — surface and enumeration cannot
//! drift.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use cli::pack::{EmbeddedPack, load_pack_schema};
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

/// A real git repo with one commit, plus a plain `jigc setup` — which writes the
/// `compose-embedded-methodology` marker, so the cascade resolves over the
/// `[dev ▸ methodology]` composite the registry below enumerates.
fn init_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    std::fs::write(repo.join("README.md"), "# repo\n").expect("write readme");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    let out = jigc(repo, home, &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Run `jigc <args>` against the **embedded** composite pack-set: `JIGC_PACK_DIR` is
/// removed, never inherited — it would swap the pack out from under the sweep and
/// silently narrow the axis back to one pack.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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

/// One swept workflow: its origin pack, its id, whether it mints a task, whether it
/// is a migrate workflow (whose mint pre-fills the commit doc), and the required
/// `commit` leaves **its own pack's** schema declares.
struct Workflow {
    pack: &'static str,
    id: String,
    creates_task: bool,
    migrate: bool,
    /// The `door:` its `suppressed:` block declares, when it is verb-routed — the one
    /// argv that composes it since M52 Increment 9 / T2, both compose-by-name doors
    /// refusing it.
    door: Option<String>,
    leaves: Vec<String>,
}

impl Workflow {
    /// The number of times each required `commit` leaf must be solicited.
    fn expected_solicits(&self) -> usize {
        usize::from(self.creates_task && !self.migrate)
    }
}

/// The two embedded packs, in the precedence order the marker installs (dev first).
/// Built the CWD-free way, like `compose_goldens`.
fn embedded_packs() -> Vec<(&'static str, EmbeddedPack)> {
    vec![
        ("dev", EmbeddedPack::new()),
        ("methodology", EmbeddedPack::methodology()),
    ]
}

/// Every workflow the **composite** ships — both packs, undivided — with the
/// expectation function applied per origin pack.
///
/// The migrate discrimination is derived **twice** and the two must agree: by name
/// (`migrate-<doctype>` over the **origin** pack's own schema list) and structurally
/// (the `step:migration-finalize` include). Either alone could rot silently.
///
/// Workflow ids do not collide across the two packs, and that premise is fenced here:
/// a collision would silently drop the loser's surface out of the axis.
fn composite_workflows() -> Vec<Workflow> {
    let mut seen: Vec<String> = Vec::new();
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        let leaves = required_commit_leaves(&pack);
        let doctypes: Vec<String> = pack
            .list(PackResourceKind::Schemas)
            .into_iter()
            .map(|id| id.as_str().to_string())
            .collect();
        for id in pack.list(PackResourceKind::Workflows) {
            let bytes = pack
                .read(PackResourceKind::Workflows, &id)
                .expect("workflow is readable");
            let def = load_workflow_def(&bytes).expect("workflow front-matter parses");
            let id = id.as_str().to_string();
            assert!(
                !seen.contains(&id),
                "workflow id `{id}` ships in more than one pack — the composite resolves \
                 it first-wins and this sweep would leave the loser's surface unswept",
            );
            seen.push(id.clone());
            let by_name = doctypes.iter().any(|ty| id == format!("migrate-{ty}"));
            let by_include = def.includes.iter().any(|step| step == MIGRATION_FINALIZE);
            assert_eq!(
                by_name, by_include,
                "the two migrate derivations disagree on `{id}`: name says {by_name}, \
                 the `step:{MIGRATION_FINALIZE}` include says {by_include} — one of them has rotted",
            );
            out.push(Workflow {
                pack: pack_name,
                id,
                creates_task: def.creates_task,
                migrate: by_name,
                door: def.suppressed.and_then(|s| s.door),
                leaves: leaves.clone(),
            });
        }
    }
    assert!(!out.is_empty(), "the composite ships workflows");
    out
}

// ---------------------------------------------------------------------------
// Counting the emitted writes
// ---------------------------------------------------------------------------

/// Every emitted line that is a `jigc doc set-…` **solicit** against
/// `commit:<task>#<leaf>` — the composed surface's own bytes, with the `Run: ` /
/// backtick decoration stripped so the remainder is executable verbatim.
///
/// A line that *opens a heredoc* is excluded (M49 Inc 11, T8): it is a
/// **demonstration** of how to feed `--from-file -` under a harness that refuses a
/// `cat … | jigc` pipeline, not a second instruction to make the write. The
/// distinction is structural, not a name — the line carries its own payload and its
/// closing `EOF`, which is why arm 2 must not try to pipe prose into it either — and
/// the defect this suite was built on (`sub-task` soliciting `#summary` twice, from
/// two different steps) is two plain solicits, both of which still count.
fn emitted_writes(text: &str, task: &str, leaf: &str) -> Vec<String> {
    let target = format!("commit:{task}#{leaf} ");
    text.lines()
        .filter(|line| line.contains(&target))
        .filter(|line| !line.trim_end().ends_with("<<'EOF'"))
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

/// The fixture state a declared `door:` needs, stood up lazily on one repo.
///
/// A verb-routed workflow composes **only** through its own door (M52 Increment 9 /
/// T2), and the door takes the very input the compose-by-name doors could not bind: a
/// foreign source for `jigc migrate`, a milestone for `jigc milestone execute`, a
/// provisioned sub-area for `jigc workflow sub-task --task`. Composing those members
/// any other way is not available, and dropping them would leave twelve migrate cells
/// asserted against empty output — a vacuous pass exactly where the axis has its
/// zero-solicit side.
#[derive(Default)]
struct Doors {
    milestone: Option<String>,
    sub_task: Option<(String, PathBuf)>,
    foreign: usize,
}

/// One composed workflow: its bytes, the task id its emitted writes address, and the
/// directory those writes must run in.
struct Composed {
    text: String,
    task: String,
    cwd: PathBuf,
}

impl Doors {
    /// Compose `workflow` through whichever door composes it. `mint` picks the
    /// non-verb-routed door: `jigc start --workflow <id> <intent>` when the caller
    /// needs a real task to write into, the mint-free preview otherwise.
    fn compose(
        &mut self,
        repo: &Path,
        home: &Path,
        workflow: &Workflow,
        intent: &str,
        mint: bool,
    ) -> Composed {
        let Some(door) = workflow.door.clone() else {
            // A `creates-task: false` workflow mints nothing, so it has no preview —
            // `jigc start --workflow <id>` composes it, which is the route the preview
            // door's own refusal names.
            let argv: Vec<String> = if !workflow.creates_task {
                vec!["start".into(), "--workflow".into(), workflow.id.clone()]
            } else if mint {
                vec![
                    "start".into(),
                    "--workflow".into(),
                    workflow.id.clone(),
                    intent.into(),
                ]
            } else {
                vec!["workflow".into(), workflow.id.clone(), "--preview".into()]
            };
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = jigc(repo, home, &args);
            assert!(
                out.status.success(),
                "`jigc {}` must succeed; stderr:\n{}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr),
            );
            let text = stdout_of(&out);
            let task = if mint && workflow.creates_task {
                minted(&text).expect("a minting compose names the task id")
            } else {
                PREVIEW_TASK.to_string()
            };
            return Composed {
                text,
                task,
                cwd: repo.to_path_buf(),
            };
        };

        let declared = engine::compose::Suppressed::door_argv(&door);
        let doctype = declared
            .iter()
            .position(|token| token == "--as")
            .and_then(|at| declared.get(at + 1))
            .cloned();
        let mut filled: Vec<String> = Vec::new();
        let mut cwd = repo.to_path_buf();
        let mut task: Option<String> = None;
        for token in declared.iter().skip(1) {
            match token.as_str() {
                "<path>" => {
                    let ty = doctype
                        .clone()
                        .expect("a `<path>` door names `--as <doctype>`");
                    filled.push(self.foreign_source(repo, &ty));
                }
                "<milestone-id>" => filled.push(self.milestone(repo, home).to_owned()),
                "<task-id>" => {
                    let (sub, worktree) = self.sub_task(repo, home, intent);
                    cwd = worktree;
                    task = Some(sub.clone());
                    filled.push(sub);
                }
                other if other.starts_with('<') => panic!(
                    "`{}`'s door carries the placeholder `{other}`, which this fixture \
                     set does not fill — add it rather than skipping the member",
                    workflow.id,
                ),
                other => filled.push(other.to_owned()),
            }
        }
        let args: Vec<&str> = filled.iter().map(String::as_str).collect();
        let out = jigc(&cwd, home, &args);
        assert!(
            out.status.success(),
            "`{door}` is `{}`'s declared door and must compose it; stderr:\n{}",
            workflow.id,
            String::from_utf8_lossy(&out.stderr),
        );
        let text = stdout_of(&out);
        // The re-entry door was handed its id; a minting door names the one it minted;
        // `jigc milestone execute` mints nothing at all, so there is no commit doc for
        // a write to address and the unbindable token stands.
        let task = task
            .or_else(|| minted(&text))
            .unwrap_or_else(|| PREVIEW_TASK.to_string());
        Composed { text, task, cwd }
    }

    /// A committed foreign source for `doctype`, fresh each call so two migrations
    /// never contend for one subject (the task id is a function of the source path).
    fn foreign_source(&mut self, repo: &Path, doctype: &str) -> String {
        self.foreign += 1;
        let rel = format!("foreign-{doctype}-{}.md", self.foreign);
        std::fs::write(
            repo.join(&rel),
            format!("# Foreign {doctype}\n\n## Section\n\nSome prose.\n"),
        )
        .expect("write the foreign source");
        // Committed, not merely written: `jigc migrate` refuses a source git has
        // never recorded.
        git(repo, &["add", &rel]);
        git(
            repo,
            &["commit", "-q", "-m", &format!("the foreign {doctype}")],
        );
        rel
    }

    fn milestone(&mut self, repo: &Path, home: &Path) -> &str {
        if self.milestone.is_none() {
            let out = jigc(repo, home, &["milestone", "create", "Door fixture"]);
            assert!(
                out.status.success(),
                "`jigc milestone create` must succeed; stderr:\n{}",
                String::from_utf8_lossy(&out.stderr),
            );
            self.milestone = Some("door-fixture".to_string());
        }
        self.milestone.as_deref().expect("the milestone is created")
    }

    /// A provisioned sub-task and its worktree — where a fanned sub-agent re-enters
    /// from, the main checkout's HEAD having moved ahead of the shared base pin when
    /// the milestone record committed.
    fn sub_task(&mut self, repo: &Path, home: &Path, intent: &str) -> (String, PathBuf) {
        if self.sub_task.is_none() {
            let milestone = self.milestone(repo, home).to_owned();
            let added = jigc(
                repo,
                home,
                &[
                    "milestone",
                    "add-task",
                    &milestone,
                    intent,
                    "--workflow",
                    "sub-task",
                ],
            );
            assert!(
                added.status.success(),
                "`jigc milestone add-task` must succeed; stderr:\n{}",
                String::from_utf8_lossy(&added.stderr),
            );
            let provisioned = jigc(repo, home, &["milestone", "provision", &milestone]);
            assert!(
                provisioned.status.success(),
                "`jigc milestone provision` must succeed; stderr:\n{}",
                String::from_utf8_lossy(&provisioned.stderr),
            );
            // The id the binary acked (`added task:<id> to milestone:<m>`), never a
            // test-side re-slug of the intent.
            let sub = stdout_of(&added)
                .lines()
                .find_map(|line| line.trim().strip_prefix("added task:"))
                .and_then(|rest| rest.split_whitespace().next())
                .expect("`milestone add-task` acks the task it added")
                .to_string();
            let worktree = repo.join(".jigc").join("worktrees").join(&sub);
            self.sub_task = Some((sub, worktree));
        }
        self.sub_task.clone().expect("the sub-task is provisioned")
    }
}

/// The task id a composition printed it minted, when it minted one.
fn minted(text: &str) -> Option<String> {
    text.lines()
        .find_map(|line| line.trim().strip_prefix("task minted: "))
        .map(|id| id.trim().to_string())
}

// ---------------------------------------------------------------------------
// Arm 1 — the count over every dev workflow
// ---------------------------------------------------------------------------

#[test]
fn every_workflow_solicits_each_required_commit_leaf_exactly_as_its_gate_demands() {
    let workflows = composite_workflows();

    let home = TempDir::new("count-home");
    let repo = TempDir::new("count-repo");
    init_repo(repo.path(), home.path());

    let mut doors = Doors::default();
    let mut wrong = Vec::new();
    let (mut expecting_one, mut expecting_zero) = (0usize, 0usize);
    let mut packs_at_one: Vec<&str> = Vec::new();
    let mut packs_at_zero: Vec<&str> = Vec::new();
    for workflow in &workflows {
        let expected = workflow.expected_solicits();
        if expected == 1 {
            expecting_one += 1;
            if !packs_at_one.contains(&workflow.pack) {
                packs_at_one.push(workflow.pack);
            }
        } else {
            expecting_zero += 1;
            if !packs_at_zero.contains(&workflow.pack) {
                packs_at_zero.push(workflow.pack);
            }
        }
        let composed = doors.compose(
            repo.path(),
            home.path(),
            workflow,
            &format!("count the {} arm", workflow.id),
            false,
        );
        for leaf in &workflow.leaves {
            let found = emitted_writes(&composed.text, &composed.task, leaf).len();
            if found != expected {
                wrong.push(format!(
                    "  [{}] {} — `#{leaf}`: expected {expected}, composed {found}",
                    workflow.pack, workflow.id,
                ));
            }
        }
    }

    // The sweep must discriminate: a partition with an empty side proves nothing —
    // and S1 demands BOTH packs sit on BOTH sides, or the axis is one pack wide.
    assert!(
        expecting_one > 0 && expecting_zero > 0,
        "the axis must have members on both sides; got {expecting_one} at one \
         and {expecting_zero} at zero",
    );
    packs_at_one.sort_unstable();
    packs_at_zero.sort_unstable();
    assert_eq!(
        packs_at_one,
        vec!["dev", "methodology"],
        "S1: both packs must contribute workflows expecting ONE solicit, or this sweep \
         only covers half the surface",
    );
    assert_eq!(
        packs_at_zero,
        vec!["dev", "methodology"],
        "S1: both packs must contribute workflows expecting ZERO solicits, or this sweep \
         only covers half the surface",
    );
    assert!(
        wrong.is_empty(),
        "every shipped workflow must solicit each required `commit` leaf exactly as many \
         times as its gate demands (once when it mints a task and does not pre-fill \
         the commit doc, zero otherwise); offenders:\n{}",
        wrong.join("\n"),
    );
}

// ---------------------------------------------------------------------------
// Arm 2 — following the composed text to a commit-clean validate
// ---------------------------------------------------------------------------

/// Substitute the **author-owned value placeholder** in an emitted field write. The
/// two packs spell it differently (`<COMMIT_TYPE>` in dev, `<TYPE>` in methodology),
/// so the token is read off the emitted bytes rather than typed here — everything
/// outside the angle brackets stays verbatim.
fn fill_value_placeholder(command: &str, value: &str) -> String {
    let (Some(open), Some(close)) = (command.find('<'), command.rfind('>')) else {
        return command.to_string();
    };
    assert!(
        open < close,
        "a composed command's placeholder brackets must nest sanely; got `{command}`",
    );
    format!("{}{value}{}", &command[..open], &command[close + 1..])
}

/// Execute one composed write command **verbatim**, substituting only the
/// author-owned payload: the value placeholder and, for a slot write, the prose
/// piped to `--from-file -`.
fn run_composed(repo: &Path, home: &Path, command: &str, commit_type: &str, prose: &str) {
    let filled = fill_value_placeholder(command, commit_type);
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
        .env_remove("JIGC_PACK_DIR")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the composed command");
    crate::support::child_stdin::feed(&mut child, prose.as_bytes());
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
    // The `type` enum is read from the composite's WINNING `commit` schema — the one
    // the write actually validates against.
    let winner = embedded_packs()
        .into_iter()
        .next()
        .expect("the composite ships a precedence winner")
        .1;
    let commit_type = first_commit_type(&winner);
    let workflows: Vec<Workflow> = composite_workflows()
        .into_iter()
        .filter(|w| w.expected_solicits() == 1)
        .collect();
    let packs: Vec<&str> = {
        let mut packs: Vec<&str> = workflows.iter().map(|w| w.pack).collect();
        packs.sort_unstable();
        packs.dedup();
        packs
    };
    assert_eq!(
        packs,
        vec!["dev", "methodology"],
        "S1: followability must be proven for BOTH packs' task-minting non-migrate \
         workflows, not one pack's",
    );

    let mut offenders = Vec::new();
    for workflow in &workflows {
        let home = TempDir::new(&format!("follow-home-{}", workflow.id));
        let repo = TempDir::new(&format!("follow-repo-{}", workflow.id));
        init_repo(repo.path(), home.path());

        let intent = format!("exercise {}", workflow.id);
        // Through whichever door composes it — and the writes are followed from the
        // directory that door composed in, which for the fan-out sub-task is its own
        // provisioned worktree.
        let mut doors = Doors::default();
        let Composed { text, task, cwd } =
            doors.compose(repo.path(), home.path(), workflow, &intent, true);

        for leaf in &workflow.leaves {
            let writes = emitted_writes(&text, &task, leaf);
            assert_eq!(
                writes.len(),
                1,
                "`{}` must compose exactly one write for `#{leaf}`; composed:\n{text}",
                workflow.id,
            );
            run_composed(
                &cwd,
                home.path(),
                &writes[0],
                &commit_type,
                "record what this change does\n",
            );
        }

        let validated = jigc(
            &cwd,
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
                "  [{}] {} — {}",
                workflow.pack,
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
