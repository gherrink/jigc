//! M55 Increment 3 / T1 — **a fan-out sub-task's composed text carries no per-task finalize
//! door** (gate-record row 11; `design/findings-channel.md` §6, the S2 row;
//! `design/workflow-dialect.md` → Emitted format).
//!
//! A sub-task has no per-task commit: `jigc task finalize <sub>` refuses at exit 3
//! (`finalize.milestone-sub-task`), and `jigc milestone finalize <m>` is its only boundary. Yet
//! every workflow carrying a finalize step told a sub-task `Run: \`jigc task finalize <sub>\``,
//! and the wrappers around that step said more of the same — the migration wrapper's *"re-run
//! the same finalize with `--approve`"*, `planning-finalize`'s *"Committing this plan lands the
//! roadmap entry"*. So when the CLI composes for a sub-task it now **omits** every step of the
//! derived omission set's finalize-door clause, and the existing sub-task trailer
//! (`resume:` / `what's-left:` / `task scope:`) follows the last surviving step directly.
//!
//! **Every compose below drives the real binary and reads the emitted bytes** — the `Spawn:`
//! line `jigc milestone execute` prints is extracted and run **verbatim** through a `jigc` shim
//! on `PATH`, never rebuilt in test code — and the cases are:
//!
//! * **(a)** five sub-tasks (`amend`, `migrate-idea`, `migrate-adr`, `planning`, `park-idea`)
//!   composed through both doors, agent text and `--format json`;
//! * **(b)** the omitting context — the same workflows composed as ordinary tasks still carry
//!   every line (and `compose_goldens::` stays green with no golden edited);
//! * **(c)** the fence — for every shipped workflow, the production derivation equals an
//!   independent text scan of the raw step files, never a literal member list (P6);
//! * **(d)** the mutant — a copy of the step files in which `methodology:author-idea` gains the
//!   door reddens the fence against the unmutated derivation and greens it re-derived;
//! * **(e)** the composed-pack mutant through the binary — a project-layer step shadow carrying
//!   the door drops `author-idea` from a sub-task and doubles the `Run:` line of an ordinary task;
//! * **(f)** `jigc milestone execute`'s own text is unchanged — it composes `Spawn:` lines, not
//!   sub-task bodies.

use crate::support;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use cli::pack::EmbeddedPack;
use engine::compose::{
    CommandCatalog, StepDef, StepSource, WorkflowDef, load_command_catalog, load_step_def,
    load_workflow_def,
};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

use support::run_then_parse::stdout_json;
use support::scratch::ScratchDir;

/// The milestone every fan-out case runs under, and the id `create` mints from it.
const MILESTONE_TITLE: &str = "Fan out the doors";
const MILESTONE: &str = "fan-out-the-doors";

/// The five workflows the Proves names, composed as sub-tasks.
const SUB_TASK_WORKFLOWS: [&str; 5] = [
    "amend",
    "migrate-idea",
    "migrate-adr",
    "planning",
    "park-idea",
];

/// The migration wrapper's own sentence about the per-task door (`step:migration-finalize`,
/// both packs).
const APPROVE_PROSE: &str = "re-run the same finalize with `--approve`";

/// `step:planning-finalize`'s opening sentence — a wrapper around the per-task door.
const PLANNING_FINALIZE_PROSE: &str = "Committing this plan lands the roadmap entry";

/// The per-task door's `Run:` line for `task`, as the composer emits it.
fn finalize_run_line(task: &str) -> String {
    format!("Run: `jigc task finalize {task}`")
}

/// Run `jigc <args>` in `cwd` with `$HOME = home`, never inheriting a harness `JIGC_PACK_DIR`.
fn jigc(cwd: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert `out` exited 0 and return its stdout.
fn ok(out: &Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
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

/// A `[dev ▸ methodology]` repo (the compose marker rides the initial commit, so every
/// provisioned worktree carries it), with two foreign notes for the migrate doors committed on
/// top of it.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    fs::create_dir_all(repo.join(".jigc/config")).expect("mk project config");
    fs::write(
        repo.join(".jigc/config/packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // A second commit, so HEAD has a parent and `jigc task amend` has a commit to repair.
    fs::create_dir_all(repo.join("notes")).expect("mk notes");
    fs::write(
        repo.join("notes/cache-idea.md"),
        "# Cache idea\n\nMaybe cache the hot reads.\n",
    )
    .expect("write foreign idea");
    fs::write(
        repo.join("notes/use-redis.md"),
        "# Use redis\n\nWe decided on redis for the cache.\n",
    )
    .expect("write foreign adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "notes: the foreign docs"]);
}

/// A fresh repo + `$HOME`, each removed on drop.
struct Fixture {
    root: ScratchDir,
}

impl Fixture {
    fn new(label: &str) -> Self {
        let root = ScratchDir::new(&format!("sub-task-composition-{label}"));
        fs::create_dir_all(root.path().join("repo")).expect("mk repo");
        fs::create_dir_all(root.path().join("home")).expect("mk home");
        init_repo(&root.path().join("repo"));
        Fixture { root }
    }

    fn repo(&self) -> PathBuf {
        self.root.path().join("repo")
    }

    fn home(&self) -> PathBuf {
        self.root.path().join("home")
    }

    fn jigc_ok(&self, cwd: &Path, args: &[&str]) -> String {
        ok(
            &jigc(cwd, &self.home(), args),
            &format!("`jigc {}`", args.join(" ")),
        )
    }

    /// `jigc milestone add-task <m> "<intent>" --workflow <w>`, returning the sub-task id the
    /// door's own ack names — never re-slugged in test code.
    fn add_task(&self, intent: &str, workflow: &str) -> String {
        let ack = self.jigc_ok(
            &self.repo(),
            &[
                "milestone",
                "add-task",
                MILESTONE,
                intent,
                "--workflow",
                workflow,
            ],
        );
        ack.split_once("task:")
            .and_then(|(_, rest)| rest.split_whitespace().next())
            .unwrap_or_else(|| panic!("the add-task ack names the minted sub-task; got:\n{ack}"))
            .to_string()
    }

    /// A sub-task's own worktree, the one `jigc milestone provision` cut.
    fn worktree(&self, sub: &str) -> PathBuf {
        self.repo().join(".jigc/worktrees").join(sub)
    }

    /// The minted id an ordinary compose's `task minted:` header names.
    fn minted(view: &str) -> String {
        view.lines()
            .find_map(|line| line.strip_prefix("task minted: "))
            .unwrap_or_else(|| panic!("an ordinary compose names its mint; got:\n{view}"))
            .trim()
            .to_string()
    }
}

/// Every `` Spawn: `…` `` span of a composed view, verbatim.
fn spawn_spans(view: &str) -> Vec<String> {
    view.lines()
        .filter_map(|line| line.strip_prefix("Spawn: `"))
        .filter_map(|rest| rest.strip_suffix('`'))
        .map(str::to_owned)
        .collect()
}

/// Install a `jigc` shim on a throwaway `PATH` entry so an emitted span — which names the bare
/// command `jigc`, as an agent runs it — resolves to the binary under test.
#[cfg(unix)]
fn install_jigc_shim(dir: &Path) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let bin = dir.join("shim-bin");
    fs::create_dir_all(&bin).expect("mk the shim bin dir");
    let shim = bin.join("jigc");
    fs::write(
        &shim,
        format!("#!/bin/sh\nexec {:?} \"$@\"\n", env!("CARGO_BIN_EXE_jigc")),
    )
    .expect("write the jigc shim");
    fs::set_permissions(&shim, fs::Permissions::from_mode(0o755)).expect("chmod the jigc shim");
    bin
}

/// Run an emitted shell span verbatim through `sh -c`, with the shim first on `PATH`.
#[cfg(unix)]
fn run_span(cwd: &Path, home: &Path, shim_bin: &Path, span: &str) -> Output {
    let path = match std::env::var("PATH") {
        Ok(rest) => format!("{}:{rest}", shim_bin.display()),
        Err(_) => shim_bin.display().to_string(),
    };
    Command::new("sh")
        .arg("-c")
        .arg(span)
        .current_dir(cwd)
        .env("HOME", home)
        .env("PATH", path)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the emitted span")
}

/// The composed-output JSON contract, `{task, text}`.
#[derive(serde::Deserialize)]
struct ComposedJson {
    task: Option<String>,
    text: String,
}

/// Assert a sub-task's composed **agent text** carries none of the per-task door's lines, and
/// that its step text — exactly the `--format json` `text` of the same door — is followed
/// directly by the sub-task trailer naming the milestone's boundary, then only the gates line
/// and the footer.
fn assert_sub_task_view(view: &str, json: &ComposedJson, sub: &str, workflow: &str, door: &str) {
    let what = format!("the `{workflow}` sub-task composed through {door}");
    assert_eq!(
        json.task.as_deref(),
        Some(sub),
        "{what}: the json names the sub-task"
    );
    for text in [view, json.text.as_str()] {
        for absent in [
            finalize_run_line(sub).as_str(),
            APPROVE_PROSE,
            PLANNING_FINALIZE_PROSE,
        ] {
            assert!(
                !text.contains(absent),
                "{what} must not carry `{absent}` — a sub-task's only commit boundary is \
                 `jigc milestone finalize {MILESTONE}`; got:\n{text}",
            );
        }
        // No `Run:` line of the per-task door for any id — a prose *mention* of the door
        // (`author-planning-record`'s) is a declared bound, not a command line.
        assert!(
            !text
                .lines()
                .any(|line| line.starts_with("Run: `jigc task finalize")),
            "{what} must carry no `Run:` line of the per-task door; got:\n{text}",
        );
    }

    // The step text is the json `text`, byte for byte, and the trailer follows it directly.
    let trailer = view.strip_prefix(json.text.as_str()).unwrap_or_else(|| {
        panic!(
            "{what}: the agent text must open with exactly the composed step text (the json \
             `text`); got:\n{view}\n--- json text ---\n{}",
            json.text
        )
    });
    let lines: Vec<&str> = trailer.lines().collect();
    assert!(
        lines.len() >= 4,
        "{what}: the trailer and footer follow the step text; got:\n{trailer}"
    );
    assert!(
        lines[0].starts_with(&format!("resume: `jigc workflow {workflow} --task {sub}`")),
        "{what}: the last surviving step is followed directly by the `resume:` line; got:\n{view}",
    );
    assert!(
        lines[1].starts_with(&format!("what's-left: `jigc task validate {sub}`")),
        "{what}: `what's-left:` follows `resume:`; got:\n{trailer}",
    );
    assert!(
        lines[2].starts_with("task scope: ")
            && lines[2].contains(&format!(
                "whose `jigc milestone finalize {MILESTONE}` is its only commit boundary"
            )),
        "{what}: the `task scope:` line names the milestone's boundary; got:\n{trailer}",
    );
    let rest = &lines[3..];
    let footer = rest.last().expect("a footer");
    assert!(
        footer.starts_with("— jigc · run `jigc start` for orientation"),
        "{what}: the routing footer closes the view; got:\n{trailer}",
    );
    assert!(
        rest[..rest.len() - 1]
            .iter()
            .all(|line| line.starts_with("create-gates: ")),
        "{what}: only the `create-gates:` line sits between the trailer and the footer; \
         got:\n{trailer}",
    );
}

/// **(a) + (f)** — five sub-tasks, both doors, agent text and json.
#[cfg(unix)]
#[test]
fn a_sub_task_composes_no_per_task_finalize_door_through_either_door() {
    let fx = Fixture::new("doors");
    let repo = fx.repo();
    let home = fx.home();
    fx.jigc_ok(&repo, &["milestone", "create", MILESTONE_TITLE]);
    let subs: BTreeMap<&str, String> = SUB_TASK_WORKFLOWS
        .iter()
        .map(|w| (*w, fx.add_task(&format!("Compose the {w} sub-task"), w)))
        .collect();
    fx.jigc_ok(&repo, &["milestone", "provision", MILESTONE]);

    // (f) `jigc milestone execute` composes `milestone-execution` — `Spawn:` lines and the
    // milestone's own boundary — and no sub-task body: its text keeps every step.
    let executed = fx.jigc_ok(&repo, &["milestone", "execute", MILESTONE]);
    assert!(
        executed.contains(&format!("Run: `jigc milestone finalize {MILESTONE}`"))
            && executed
                .contains("After a clean join, the parent finalize is the one commit boundary"),
        "`jigc milestone execute` keeps its milestone-finalize step; got:\n{executed}",
    );
    let spans = spawn_spans(&executed);
    assert_eq!(
        spans.len(),
        SUB_TASK_WORKFLOWS.len(),
        "one `Spawn:` line per sub-task; got:\n{executed}"
    );

    let shim_bin = install_jigc_shim(fx.root.path());
    for (workflow, sub) in &subs {
        let worktree = fx.worktree(sub);

        // Door 1: the `Spawn:` line, run verbatim (it `cd`s into the worktree itself).
        let span = spans
            .iter()
            .find(|span| span.ends_with(&format!(" --task {sub}")))
            .unwrap_or_else(|| panic!("a `Spawn:` line for `{sub}`; got:\n{executed}"));
        assert!(
            span.contains(&format!("jigc workflow {workflow} --task {sub}")),
            "the span re-enters the recorded workflow: {span}"
        );
        let view = ok(
            &run_span(&repo, &home, &shim_bin, span),
            &format!("the span `{span}`"),
        );
        let json: ComposedJson = stdout_json(
            &jigc(
                &worktree,
                &home,
                &["--format", "json", "workflow", workflow, "--task", sub],
            ),
            &[0],
            "the re-entry door's json",
        );
        assert_sub_task_view(&view, &json, sub, workflow, "its `Spawn:` line");

        // Door 2: `jigc start --task <sub>`, from the sub-task's own worktree.
        let view = fx.jigc_ok(&worktree, &["start", "--task", sub]);
        let json: ComposedJson = stdout_json(
            &jigc(
                &worktree,
                &home,
                &["--format", "json", "start", "--task", sub],
            ),
            &[0],
            "the resume door's json",
        );
        assert_sub_task_view(&view, &json, sub, workflow, "`jigc start --task`");
    }

    // The whole-step omission, not only the `Run:` line: an `amend` sub-task's one step is
    // the door's, so its step text is empty and the trailer opens the view.
    let amend = &subs["amend"];
    let view = fx.jigc_ok(&fx.worktree(amend), &["start", "--task", amend]);
    assert!(
        view.starts_with(&format!("resume: `jigc workflow amend --task {amend}`")),
        "an all-omitted `amend` sub-task composes an empty body, then the trailer; got:\n{view}",
    );
    // And the surviving steps are really there: a migrate sub-task keeps its author step.
    let idea = &subs["migrate-idea"];
    let view = fx.jigc_ok(&fx.worktree(idea), &["start", "--task", idea]);
    assert!(
        view.contains("Migrate the foreign parked-idea note into a managed `idea`"),
        "a `migrate-idea` sub-task keeps its author step; got:\n{view}",
    );
}

/// **(b)** — the omitting context: composed as ordinary tasks, the same workflows still carry
/// the per-task door and its wrappers' prose.
#[test]
fn b_ordinary_tasks_keep_the_per_task_finalize_door() {
    let fx = Fixture::new("ordinary");
    let repo = fx.repo();

    let park = fx.jigc_ok(
        &repo,
        &["start", "--workflow", "park-idea", "Keep the cache idea"],
    );
    let planning = fx.jigc_ok(
        &repo,
        &[
            "start",
            "--workflow",
            "planning",
            "Plan the cache milestone",
        ],
    );
    let migrate_idea = fx.jigc_ok(&repo, &["migrate", "notes/cache-idea.md", "--as", "idea"]);
    let migrate_adr = fx.jigc_ok(&repo, &["migrate", "notes/use-redis.md", "--as", "adr"]);
    let amend = fx.jigc_ok(&repo, &["task", "amend"]);

    for (what, view) in [
        ("park-idea", &park),
        ("planning", &planning),
        ("migrate-idea", &migrate_idea),
        ("migrate-adr", &migrate_adr),
        ("amend", &amend),
    ] {
        let id = Fixture::minted(view);
        assert!(
            view.contains(&finalize_run_line(&id)),
            "an ordinary `{what}` task keeps `{}`; got:\n{view}",
            finalize_run_line(&id),
        );
    }
    for (what, view) in [
        ("migrate-idea", &migrate_idea),
        ("migrate-adr", &migrate_adr),
    ] {
        assert!(
            view.contains(APPROVE_PROSE),
            "an ordinary `{what}` task keeps the review-hold prose; got:\n{view}"
        );
    }
    assert!(
        planning.contains(PLANNING_FINALIZE_PROSE),
        "an ordinary `planning` task keeps `planning-finalize`; got:\n{planning}"
    );
}

/// **(e)** — the set is derived from the **composed** packs: a project-layer step shadow of
/// `author-idea` gaining the door is classified as it composes.
#[cfg(unix)]
#[test]
fn e_a_project_step_shadow_carrying_the_door_joins_the_omission() {
    let fx = Fixture::new("shadow");
    let repo = fx.repo();
    let home = fx.home();
    fx.jigc_ok(&repo, &["milestone", "create", MILESTONE_TITLE]);
    let sub = fx.add_task("Park the warm-cache idea", "park-idea");
    fx.jigc_ok(&repo, &["milestone", "provision", MILESTONE]);

    const AUTHOR_IDEA: &str = "Park the shaped direction on a fresh idea doc.";
    // Before the shadow: the sub-task keeps `author-idea` (it is not a member).
    let span = spawn_spans(&fx.jigc_ok(&repo, &["milestone", "execute", MILESTONE]))
        .pop()
        .expect("one `Spawn:` line");
    let shim_bin = install_jigc_shim(fx.root.path());
    let before = ok(&run_span(&repo, &home, &shim_bin, &span), "the span");
    assert!(
        before.contains(AUTHOR_IDEA),
        "author-idea composes before the shadow; got:\n{before}"
    );

    // The project layer forks `author-idea` and gives it the per-task door.
    fx.jigc_ok(&repo, &["config", "fork", "workflow:park-idea#author-idea"]);
    let shadow = repo.join(".jigc/config/steps/author-idea.yaml");
    let mut body = fs::read_to_string(&shadow).expect("read the forked shadow");
    body.push_str("\n{{ cli.finalize-task }}\n");
    fs::write(&shadow, body).expect("write the shadow");

    let after = ok(
        &run_span(&repo, &home, &shim_bin, &span),
        "the span over the shadow",
    );
    assert!(
        !after.contains(AUTHOR_IDEA) && !after.contains(&finalize_run_line(&sub)),
        "the shadowed `author-idea` now carries the door, so the sub-task omits it; got:\n{after}",
    );

    // The omitting context: an ordinary `park-idea` task composes the shadow — a second
    // `Run:` line, the shadow's own.
    let ordinary = fx.jigc_ok(
        &repo,
        &["start", "--workflow", "park-idea", "Keep the cold idea"],
    );
    let id = Fixture::minted(&ordinary);
    assert!(
        ordinary.contains(AUTHOR_IDEA),
        "an ordinary task composes the shadow; got:\n{ordinary}"
    );
    assert_eq!(
        ordinary.matches(&finalize_run_line(&id)).count(),
        2,
        "an ordinary task carries the shadow's door and finalize's; got:\n{ordinary}",
    );
}

// ---------------------------------------------------------------------------------------
// (c) / (d) — the fence: the production derivation against an independent raw-text scan.
// ---------------------------------------------------------------------------------------

/// The two shipped packs, in precedence order, each with its on-disk directory.
fn shipped_packs() -> Vec<(&'static str, EmbeddedPack, PathBuf)> {
    vec![
        (
            "dev",
            EmbeddedPack::new(),
            PathBuf::from(cli::pack_path!(dev)),
        ),
        (
            "methodology",
            EmbeddedPack::methodology(),
            PathBuf::from(cli::pack_path!(methodology)),
        ),
    ]
}

/// One shipped workflow, as the fence reads it: its origin pack, its definition, and that
/// pack's command catalog.
struct ShippedWorkflow {
    pack: &'static str,
    id: String,
    def: WorkflowDef,
    catalog: CommandCatalog,
}

/// Every shipped workflow, enumerated from the registry over `[dev ▸ methodology]` — the
/// two packs' workflow sets do not overlap, so each workflow's origin is the pack listing it.
fn shipped_workflows() -> Vec<ShippedWorkflow> {
    let mut out = Vec::new();
    for (name, pack, _) in shipped_packs() {
        let catalog = load_command_catalog(
            &pack
                .read(PackResourceKind::Config, &ResourceId::from("commands"))
                .expect("the pack ships a catalog"),
        )
        .expect("the catalog loads");
        for id in pack.list(PackResourceKind::Workflows) {
            let bytes = pack
                .read(PackResourceKind::Workflows, &id)
                .expect("read the workflow");
            out.push(ShippedWorkflow {
                pack: name,
                id: id.as_str().to_string(),
                def: load_workflow_def(&bytes).expect("the workflow loads"),
                catalog: catalog.clone(),
            });
        }
    }
    assert!(
        out.len() > 30,
        "the registry enumerates the shipped workflows"
    );
    out
}

/// A step source over one pack's embedded steps — what the binary composes from.
struct EmbeddedSteps<'a>(&'a EmbeddedPack);

impl StepSource for EmbeddedSteps<'_> {
    fn step(&self, id: &str) -> Option<StepDef> {
        let bytes = self
            .0
            .read(PackResourceKind::Steps, &ResourceId::from(id))
            .ok()?;
        load_step_def(id, &bytes).ok()
    }
}

/// A step source over a directory of `<id>.yaml` step files — the mutant's copy.
struct DirSteps(PathBuf);

impl StepSource for DirSteps {
    fn step(&self, id: &str) -> Option<StepDef> {
        let bytes = fs::read(self.0.join(format!("{id}.yaml"))).ok()?;
        load_step_def(id, &bytes).ok()
    }
}

/// The derived set per workflow, keyed `<pack>:<workflow>`.
type Derived = BTreeMap<String, BTreeSet<String>>;

/// The **production** derivation for every shipped workflow, each over the step source
/// `steps_of(pack)` yields.
fn derive<'a>(steps_of: impl Fn(&'static str) -> Box<dyn StepSource + 'a>) -> Derived {
    shipped_workflows()
        .into_iter()
        .map(|w| {
            let source = steps_of(w.pack);
            (
                format!("{}:{}", w.pack, w.id),
                cli::start::sub_task_omission_set(&w.def, source.as_ref(), &w.catalog),
            )
        })
        .collect()
}

/// **The independent scan** (P6): a raw-text read of `<steps_root>/<pack>/<id>.yaml`. A step is
/// a member when its file carries `{{ cli.finalize-task }}`, or includes a member over raw
/// `{{ include: step:<id> }}` lines. No engine recognizer, no catalog, no member list.
fn scan(steps_root: &Path) -> Derived {
    fn member(dir: &Path, id: &str, memo: &mut BTreeMap<String, bool>) -> bool {
        if let Some(known) = memo.get(id) {
            return *known;
        }
        let path = dir.join(format!("{id}.yaml"));
        let raw = fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "the scan reads every included step: {}: {e}",
                path.display()
            )
        });
        let mut is = raw.contains("{{ cli.finalize-task }}");
        for line in raw.lines() {
            if let Some(child) = line
                .trim()
                .strip_prefix("{{ include: step:")
                .and_then(|rest| rest.strip_suffix(" }}"))
            {
                is |= member(dir, child, memo);
            }
        }
        memo.insert(id.to_string(), is);
        is
    }
    shipped_workflows()
        .into_iter()
        .map(|w| {
            let dir = steps_root.join(w.pack);
            let mut memo = BTreeMap::new();
            for id in &w.def.includes {
                member(&dir, id, &mut memo);
            }
            let set = memo
                .into_iter()
                .filter(|(_, is)| *is)
                .map(|(id, _)| id)
                .collect();
            (format!("{}:{}", w.pack, w.id), set)
        })
        .collect()
}

/// The fence: `derived` must equal the scan of `steps_root`, workflow for workflow.
fn fence(derived: &Derived, steps_root: &Path) -> Result<(), String> {
    let scanned = scan(steps_root);
    let drift: Vec<String> = scanned
        .iter()
        .filter(|(w, set)| derived.get(*w) != Some(set))
        .map(|(w, set)| format!("{w}: scanned {set:?}, derived {:?}", derived.get(w)))
        .collect();
    if drift.is_empty() {
        Ok(())
    } else {
        Err(drift.join("\n"))
    }
}

/// The shipped step files, laid out `<root>/<pack>/<id>.yaml`, copied into `dest`.
fn copy_shipped_steps(dest: &Path) {
    for (name, _, dir) in shipped_packs() {
        let to = dest.join(name);
        fs::create_dir_all(&to).expect("mk the copy dir");
        for entry in fs::read_dir(dir.join("steps")).expect("read the shipped steps") {
            let entry = entry.expect("a dir entry");
            fs::copy(entry.path(), to.join(entry.file_name())).expect("copy a step file");
        }
    }
}

/// **(c)** — the fence is green over the shipped packs, and not vacuously.
#[test]
fn c_the_derived_omission_set_equals_a_raw_scan_of_the_shipped_steps() {
    let embedded: BTreeMap<&str, EmbeddedPack> = shipped_packs()
        .into_iter()
        .map(|(n, p, _)| (n, p))
        .collect();
    let derived = derive(|pack| Box::new(EmbeddedSteps(&embedded[pack])));

    let root = ScratchDir::new("sub-task-composition-fence");
    copy_shipped_steps(root.path());
    if let Err(drift) = fence(&derived, root.path()) {
        panic!("the derived sub-task omission set drifted from the shipped steps:\n{drift}");
    }
    // Not vacuous: the door reaches the Proves workflows through every carrier shape.
    for (workflow, member) in [
        ("dev:amend", "amend-message"),
        ("dev:migrate-adr", "migration-finalize"),
        ("methodology:migrate-idea", "migration-finalize"),
        ("methodology:planning", "planning-finalize"),
        ("methodology:park-idea", "finalize"),
        ("dev:project-setup", "project-finalize"),
    ] {
        assert!(
            derived[workflow].contains(member),
            "{workflow}'s derived set holds `{member}`; got {:?}",
            derived[workflow],
        );
    }
}

/// **(d)** — the mutant: `methodology:author-idea` gains the door. The fence is red against the
/// set derived from the unmutated packs and green against the set re-derived from the copy.
#[test]
fn d_a_step_gaining_the_door_reddens_the_fence_until_re_derived() {
    let root = ScratchDir::new("sub-task-composition-mutant");
    copy_shipped_steps(root.path());
    let author_idea = root.path().join("methodology/author-idea.yaml");
    let mut body = fs::read_to_string(&author_idea).expect("read author-idea");
    assert!(
        !body.contains("{{ cli.finalize-task }}"),
        "the mutant starts unmutated"
    );
    body.push_str("\n{{ cli.finalize-task }}\n");
    fs::write(&author_idea, body).expect("apply the mutant");

    let embedded: BTreeMap<&str, EmbeddedPack> = shipped_packs()
        .into_iter()
        .map(|(n, p, _)| (n, p))
        .collect();
    let unmutated = derive(|pack| Box::new(EmbeddedSteps(&embedded[pack])));
    let drift = fence(&unmutated, root.path()).expect_err("the fence reddens on the mutant");
    assert!(
        drift.contains("methodology:park-idea"),
        "the drift names the workflow composing the mutated step; got:\n{drift}"
    );

    let copy = root.path().to_path_buf();
    let rederived = derive(|pack| Box::new(DirSteps(copy.join(pack))));
    assert!(
        rederived["methodology:park-idea"].contains("author-idea"),
        "the re-derived set follows the mutant"
    );
    if let Err(drift) = fence(&rederived, root.path()) {
        panic!("the re-derived set must satisfy the fence over the mutant:\n{drift}");
    }
}
