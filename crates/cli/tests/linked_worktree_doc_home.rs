//! **A linked worktree the user made commits code only** — the linked-worktree doc guard
//! (the rc.24 fix pass, the linked-worktree sibling of `(R3, F7)`; `design/storage.md` →
//! CLI and git; `crate::render::CodeOnlyCheckout`).
//!
//! `jigc task finalize` reads the doc store, the reconciler's baseline and the clobber guard
//! at the **main** checkout and promotes into the checkout the command was typed in. From a
//! `git worktree add` those are two directories, and driven on `1.0.0-rc.24` every one of
//! these landed at exit 0:
//!
//! * **C1** — a hand edit to a copied-in doc in the linked worktree, gone from every blob;
//! * **C3** — an untracked file at a created doc's home there, overwritten;
//! * **C4** — the first doc task on a branch whose doc differs from the main checkout's,
//!   silently reverting the branch's committed wording;
//! * **C2** — the next task there blocked on an "external edit" nobody made.
//!
//! The design already says *"only code (never a managed doc) ever rides a worktree"*. The
//! guard enforces it at the earliest door: a doc that **promotes** is refused at every
//! `jigc doc` write leaf, and — for a doc staged some other way — at `jigc task finalize`,
//! its `--dry-run`, `jigc task validate` and bare `jigc start`, under one key.
//!
//! **What each arm iterates.** The class has three axes and the arms walk them rather than
//! the reported instance: the **write leaves** are `cli::doc::doc_write_verbs()` (derived
//! from the clap tree, so a new leaf with no cell here fails the suite); the **doc kinds**
//! are *location*, *placement* and *transient*, the three answers
//! `engine::finalize::promote_destination` can give; and the **checkouts** are the main
//! one, a linked worktree the user made, and one of jigc's own fan-out worktrees — plus the
//! **must-not-refuse** layouts where `.git` is a file and no second checkout exists (a
//! worktree of a bare repository, `--separate-git-dir`, a submodule), each walked to a
//! landed doc under one line-ending conversion setting. The
//! doc-only mint axis is the set of shipped workflows that compose `step:finalize-doc-only`,
//! read off the pack directories.
//!
//! **Every refusal's route is run as printed** — the spans are lifted out of the emitted
//! bytes and executed through a real shell, never re-typed
//! (`implementation/increment-workflow.md` → Validation hardening).

use crate::support::trial_corpus::{State, TrialCorpus};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The guard's finding code — one identity at every door.
const CODE: &str = "finalize.linked-worktree-doc";

/// The committed ADR's slug and home, and the symbol it cites.
const ADR: &str = "adr:single-node-cache";
const ADR_HOME: &str = "docs/decisions/single-node-cache.md";
const CITED: &str = "src/lib.rs#cache_get";

/// A set-up corpus with the committed singleton set, one committed ADR citing real code,
/// and a linked worktree the **user** made (`git worktree add -b feature`) beside it.
struct Rig {
    corpus: TrialCorpus,
    wt: PathBuf,
}

impl Rig {
    fn new() -> Self {
        let corpus = TrialCorpus::build(State::CommittedSingletons);
        let repo = corpus.repo();

        // A committed location-doctype doc that cites code — through the real door, so the
        // file-state baseline, the edge index and the ADR's bytes are what an adopter has.
        let task = corpus.start_workflow("single-task", "choose a cache");
        fs::create_dir_all(repo.join("src")).expect("mk src");
        fs::write(
            repo.join("src").join("lib.rs"),
            "pub fn cache_get() -> u32 { 1 }\n",
        )
        .expect("write the cited code");
        corpus.git(&["add", "src/lib.rs"]);
        corpus.jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Single node cache",
            "--task",
            &task,
        ]);
        for slot in ["context", "decision", "consequences"] {
            corpus.set_slot(&format!("{ADR}#{slot}"), &task, "Prose.");
        }
        corpus.set_field(&format!("{ADR}#cites-code"), &task, CITED);
        corpus.finalize(&task, "cache", "choose a cache", false);
        assert!(
            repo.join(ADR_HOME).is_file(),
            "the fixture must commit the ADR at {ADR_HOME}",
        );

        let wt = repo.parent().expect("the corpus root").join("wt");
        corpus.git(&[
            "worktree",
            "add",
            "-q",
            "-b",
            "feature",
            wt.to_str().expect("a UTF-8 worktree path"),
        ]);
        Rig { corpus, wt }
    }

    /// A repository with one commit and **no jigc install** — the source the layout cells
    /// clone from, each of which runs `jigc setup` itself. It has no linked worktree, so
    /// [`Self::wt`] names nothing.
    fn never_adopted() -> Self {
        Rig {
            corpus: TrialCorpus::build_never_adopted(),
            wt: PathBuf::new(),
        }
    }

    fn main(&self) -> PathBuf {
        self.corpus.repo()
    }

    /// The main checkout as jigc prints it — canonicalized, which on macOS carries the
    /// `/private` prefix the temp dir's own spelling lacks.
    fn main_printed(&self) -> String {
        printed(&self.main())
    }

    fn run(&self, cwd: &Path, args: &[&str]) -> Output {
        self.corpus.jigc_stdin_from(cwd, args, "")
    }

    fn run_stdin(&self, cwd: &Path, args: &[&str], stdin: &str) -> Output {
        self.corpus.jigc_stdin_from(cwd, args, stdin)
    }

    fn ok(&self, cwd: &Path, args: &[&str]) -> String {
        let out = self.run(cwd, args);
        assert_ok(&out, &format!("`jigc {}` from {cwd:?}", args.join(" ")));
        text(&out.stdout)
    }

    /// Mint `workflow` from `cwd` and return the id the binary printed.
    fn mint(&self, cwd: &Path, workflow: &str, intent: &str) -> String {
        minted(&self.ok(cwd, &["start", "--workflow", workflow, intent]))
    }

    /// The doc ids `task` stages, read off the working area itself.
    fn staged(&self, task: &str) -> Vec<String> {
        let docs = self
            .main()
            .join(".jigc")
            .join("tasks")
            .join(task)
            .join("docs");
        let mut ids: Vec<String> = fs::read_dir(&docs)
            .map(|entries| {
                entries
                    .flatten()
                    .filter_map(|entry| {
                        entry
                            .file_name()
                            .to_str()
                            .and_then(|name| name.strip_suffix(".md"))
                            .map(str::to_owned)
                    })
                    .collect()
            })
            .unwrap_or_default();
        ids.sort();
        ids
    }

    /// Author the task's transient commit doc from `cwd` — the one doc a code-only checkout
    /// still writes.
    fn fill_commit(&self, cwd: &Path, task: &str, summary: &str) {
        for (field, value) in [("type", "feat"), ("scope", "core")] {
            self.ok(
                cwd,
                &[
                    "doc",
                    "set-field",
                    &format!("commit:{task}#{field}"),
                    "--value",
                    value,
                    "--task",
                    task,
                ],
            );
        }
        for (slot, prose) in [("summary", summary), ("body", "Body prose.")] {
            let out = self.run_stdin(
                cwd,
                &[
                    "doc",
                    "set-slot",
                    &format!("commit:{task}#{slot}"),
                    "--from-file",
                    "-",
                    "--task",
                    task,
                ],
                prose,
            );
            assert_ok(&out, &format!("authoring commit:{task}#{slot}"));
        }
    }

    fn git(&self, cwd: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(cwd)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} in {cwd:?} failed: {}",
            text(&out.stderr),
        );
        text(&out.stdout).trim().to_string()
    }

    /// Run `script` through a real shell in `cwd`, with `jigc` on `PATH` resolving to the
    /// binary under test and `$HOME` the corpus's — so an emitted command line runs as the
    /// bytes it was printed as.
    fn sh(&self, cwd: &Path, script: &str) -> Output {
        let bin = self.main().parent().expect("the corpus root").join("bin");
        fs::create_dir_all(&bin).expect("mk the bin dir");
        let link = bin.join("jigc");
        if !link.exists() {
            std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_jigc"), &link)
                .expect("link the binary under test as `jigc`");
        }
        let path = format!(
            "{}:{}",
            bin.display(),
            std::env::var("PATH").unwrap_or_default()
        );
        Command::new("sh")
            .arg("-c")
            .arg(script)
            .current_dir(cwd)
            .env("PATH", path)
            .env("HOME", self.corpus.home())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run sh")
    }
}

fn printed(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn both(out: &Output) -> String {
    format!("{}{}", text(&out.stdout), text(&out.stderr))
}

fn assert_ok(out: &Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status,
        text(&out.stdout),
        text(&out.stderr),
    );
}

/// The id a `task minted: <id>` header names.
fn minted(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("no `task minted:` header in:\n{stdout}"))
        .trim()
        .to_string()
}

/// `(code, target, route)` for every finding a `--format json` refusal carries — read off
/// whichever stream holds the document, since a doc-write block's is stderr and a blocked
/// finalize's is stdout.
fn findings(out: &Output) -> Vec<(String, String, String)> {
    let value: serde_json::Value = [&out.stdout, &out.stderr]
        .into_iter()
        .find_map(|stream| serde_json::from_slice(stream).ok())
        .unwrap_or_else(|| panic!("neither stream is a JSON document:\n{}", both(out)));
    let list = value
        .get("findings")
        .or_else(|| value.get("tasks"))
        .unwrap_or_else(|| panic!("the document carries no findings: {value}"));
    let rows: Vec<&serde_json::Value> = match list.as_array() {
        Some(rows) if value.get("findings").is_some() => rows.iter().collect(),
        // The orientation view nests each task's findings under its row.
        Some(tasks) => tasks
            .iter()
            .filter_map(|task| task.get("findings").and_then(|f| f.as_array()))
            .flatten()
            .collect(),
        None => Vec::new(),
    };
    rows.into_iter()
        .map(|row| {
            let field = |name: &str| row[name].as_str().unwrap_or_default().to_string();
            (
                field("code"),
                row["key"]["target"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                field("route"),
            )
        })
        .collect()
}

/// The guard's findings alone, as `(target, route)`.
fn guard(out: &Output) -> Vec<(String, String)> {
    findings(out)
        .into_iter()
        .filter(|(code, _, _)| code == CODE)
        .map(|(_, target, route)| (target, route))
        .collect()
}

/// Every backticked span of `route`, in order — the bytes a reader would paste.
fn spans(route: &str) -> Vec<String> {
    route
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

/// The one span of `route` matching `pick`, panicking with the route when it has none.
fn span(route: &str, what: &str, pick: impl Fn(&str) -> bool) -> String {
    spans(route)
        .into_iter()
        .find(|s| pick(s))
        .unwrap_or_else(|| panic!("the route names no {what}:\n{route}"))
}

/// One write-leaf cell: the kind of doc it aims at, its argv and stdin, and the home the
/// refusal must key at.
struct Cell {
    kind: &'static str,
    args: Vec<&'static str>,
    stdin: &'static str,
    home: &'static str,
}

const AUTHORED_ADR: &str = "\
title: \"Authored decision\"
sections:
  - id: context
    set:
      context: |-
        <<Forces.>>
  - id: decision
    set:
      decision: |-
        <<Decided.>>
  - id: consequences
    set:
      consequences: |-
        <<Effects.>>
";

/// The write-leaf × doc-kind table. Keyed by the **clap leaf name**; a leaf
/// `doc_write_verbs()` reports that has no entry here fails the arm below.
fn cells(leaf: &str) -> Vec<Cell> {
    let cell = |kind, args: &[&'static str], stdin, home| Cell {
        kind,
        args: args.to_vec(),
        stdin,
        home,
    };
    match leaf {
        "set-field" => vec![
            cell(
                "location",
                &[
                    "set-field",
                    "adr:single-node-cache#status/status",
                    "--value",
                    "accepted",
                ],
                "",
                ADR_HOME,
            ),
            // The leaf's second arm: `--unset` is its own function in `cli::doc`.
            cell(
                "location, --unset",
                &["set-field", "adr:single-node-cache#cites-code", "--unset"],
                "",
                ADR_HOME,
            ),
        ],
        "set-slot" => vec![
            cell(
                "placement",
                &["set-slot", "vision:vision#thesis", "--from-file", "-"],
                "A new thesis.\n",
                "VISION.md",
            ),
            cell(
                "location",
                &[
                    "set-slot",
                    "adr:single-node-cache#context",
                    "--from-file",
                    "-",
                ],
                "New forces.\n",
                ADR_HOME,
            ),
        ],
        "add-item" => vec![cell(
            "placement",
            &[
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "added",
            ],
            "",
            "CHANGELOG.md",
        )],
        "retitle-item" => vec![cell(
            "placement",
            &[
                "retitle-item",
                "roadmap:roadmap#milestones/m-alpha",
                "--title",
                "M-Beta",
            ],
            "",
            "docs/roadmap.md",
        )],
        "remove-item" => vec![cell(
            "placement",
            &["remove-item", "roadmap:roadmap#milestones/m-alpha"],
            "",
            "docs/roadmap.md",
        )],
        "rename" => vec![cell(
            "location",
            &["rename", "adr:single-node-cache", "--to", "Two node cache"],
            "",
            ADR_HOME,
        )],
        "create" => vec![
            cell(
                "location",
                &["create", "adr", "--title", "Fresh decision"],
                "",
                "docs/decisions/fresh-decision.md",
            ),
            cell(
                "placement",
                &["create", "changelog", "--title", "Changelog"],
                "",
                "CHANGELOG.md",
            ),
        ],
        "author" => vec![cell(
            "location",
            &["author", "adr", "--from-file", "-"],
            AUTHORED_ADR,
            "docs/decisions/authored-decision.md",
        )],
        other => panic!(
            "`jigc doc {other}` is a write leaf with no cell in this suite — a new leaf must \
             be given one (and must pass the linked-worktree doc guard) before it ships",
        ),
    }
}

// ---------------------------------------------------------------------------------------
// The write door — every leaf, every promoting doc kind, from a linked worktree.
// ---------------------------------------------------------------------------------------

/// Every `jigc doc` write leaf refuses a doc that promotes, from a linked worktree the user
/// made: one finding, the guard's code, keyed at the home the doc would have promoted to,
/// with nothing staged and nothing read into the task. The same argv from the main
/// checkout raises no such finding — the predicate is the checkout, not the verb.
#[test]
fn every_doc_write_leaf_refuses_a_promoting_doc_from_a_linked_worktree() {
    let rig = Rig::new();
    let task = rig.mint(&rig.wt, "single-task", "work in the worktree");
    let control = rig.mint(&rig.main(), "single-task", "work in the main checkout");
    let untouched = rig.staged(&task);
    assert_eq!(
        untouched,
        [format!("commit:{task}")],
        "a fresh task stages its transient commit doc and nothing else",
    );

    let leaves = cli::doc::doc_write_verbs();
    assert!(
        leaves.len() >= 8,
        "the write family is derived from the clap tree; got {leaves:?}",
    );
    for leaf in &leaves {
        for cell in cells(leaf) {
            let what = format!("`jigc doc {}` ({})", cell.args.join(" "), cell.kind);
            let argv = |task: &'_ str, json: bool| {
                let mut argv = vec!["doc".to_string()];
                argv.extend(cell.args.iter().map(|arg| arg.to_string()));
                argv.extend(["--task".to_string(), task.to_string()]);
                if json {
                    argv.extend(["--format".to_string(), "json".to_string()]);
                }
                argv
            };

            // From the linked worktree: refused, keyed at the doc's home.
            let argv_wt = argv(&task, true);
            let refs: Vec<&str> = argv_wt.iter().map(String::as_str).collect();
            let out = rig.run_stdin(&rig.wt, &refs, cell.stdin);
            assert_eq!(
                out.status.code(),
                Some(1),
                "{what} from a linked worktree must be refused at the write door's exit; \
                 got:\n{}",
                both(&out),
            );
            let all = findings(&out);
            assert_eq!(
                all.iter()
                    .map(|(code, target, _)| (code.as_str(), target.as_str()))
                    .collect::<Vec<_>>(),
                [(CODE, cell.home)],
                "{what}: exactly one finding, the guard's, keyed at the home the doc \
                 promotes to",
            );
            let route = &all[0].2;
            assert!(
                route.contains(&format!("`cd {}`", rig.main_printed())),
                "{what}: the route opens with the main checkout to stand in; got:\n{route}",
            );
            assert!(
                !route.contains("--task"),
                "{what}: the route never sends this task to the main checkout — its base \
                 pin is this branch's, and that door refuses it when the branches' docs \
                 differ; got:\n{route}",
            );
            assert_eq!(
                rig.staged(&task),
                untouched,
                "{what}: a refused write stages nothing and copies nothing in",
            );

            // From the main checkout: whatever this argv does there, it is not this.
            let argv_main = argv(&control, false);
            let refs: Vec<&str> = argv_main.iter().map(String::as_str).collect();
            let seen = both(&rig.run_stdin(&rig.main(), &refs, cell.stdin));
            assert!(
                !seen.contains(CODE),
                "{what} from the main checkout must never raise the guard; got:\n{seen}",
            );
        }
    }

    // The transient kind: the task's own commit doc promotes nowhere, so the two leaves it
    // takes still write from the linked worktree — which is what keeps a code-only task
    // working here.
    rig.fill_commit(&rig.wt, &task, "work in the worktree");
}

/// The exemption and its edge, in jigc's **own** fan-out worktree: a milestone sub-task
/// writes a promoting doc from its worktree — its boundary is the milestone's, whose whole
/// subject is the main checkout — while an *ordinary* task minted in that same directory is
/// refused, because its `jigc task finalize` would commit there.
#[test]
fn a_fan_out_sub_task_writes_its_docs_and_an_ordinary_task_there_does_not() {
    let rig = Rig::new();
    let main = rig.main();
    rig.ok(&main, &["milestone", "create", "Cache rework"]);
    rig.ok(
        &main,
        &["milestone", "add-task", "cache-rework", "Area one"],
    );
    rig.ok(&main, &["milestone", "provision", "cache-rework"]);
    let fan = main.join(".jigc").join("worktrees").join("area-one");
    assert!(fan.join(".git").is_file(), "the fan-out worktree is cut");
    rig.ok(&fan, &["workflow", "sub-task", "--task", "area-one"]);

    let write = |task: &str| {
        rig.run_stdin(
            &fan,
            &[
                "doc",
                "set-slot",
                "vision:vision#thesis",
                "--from-file",
                "-",
                "--task",
                task,
            ],
            "A sub-agent's thesis.\n",
        )
    };
    let sub = write("area-one");
    assert_ok(&sub, "a sub-task's doc write from its own fan-out worktree");
    assert!(
        rig.staged("area-one")
            .contains(&"vision:vision".to_string()),
        "the sub-task staged the doc into its own area",
    );

    let ordinary = rig.mint(
        &fan,
        "single-task",
        "an ordinary task in a fan-out worktree",
    );
    let refused = write(&ordinary);
    assert_eq!(refused.status.code(), Some(1), "got:\n{}", both(&refused));
    assert!(
        both(&refused).contains(CODE),
        "an ordinary task commits in the checkout it stands in, a fan-out worktree \
         included; got:\n{}",
        both(&refused),
    );

    // …and the sub-agent's read surfaces do not move: no note on a read from here.
    let read = rig.run(&fan, &["doc", "show", "vision:vision"]);
    assert_ok(&read, "`jigc doc show` from a fan-out worktree");
    assert!(
        !text(&read.stderr).contains("served from the main checkout"),
        "jigc's own fan-out is the declared design; its reads carry no note. stderr:\n{}",
        text(&read.stderr),
    );
}

// ---------------------------------------------------------------------------------------
// Code-only tasks keep working — the C2-09 decision, unmoved.
// ---------------------------------------------------------------------------------------

/// A task that writes no managed doc lands from the linked worktree exactly as before: its
/// commit advances the worktree's branch, carries the code and nothing else, and no managed
/// path in either checkout moves. Two tasks in a row, so the second cannot be blocked on a
/// baseline the first poisoned (**C2**).
#[test]
fn a_code_only_task_lands_from_a_linked_worktree() {
    let rig = Rig::new();
    let main_head = rig.git(&rig.main(), &["rev-parse", "HEAD"]);
    for (workflow, intent, file) in [
        ("single-task", "first code change", "one.txt"),
        ("quick-fix", "second code change", "two.txt"),
    ] {
        let task = rig.mint(&rig.wt, workflow, intent);
        fs::write(rig.wt.join(file), "code\n").expect("write code");
        rig.git(&rig.wt, &["add", file]);
        rig.fill_commit(&rig.wt, &task, intent);

        for door in [
            vec!["task", "validate", task.as_str()],
            vec!["task", "finalize", task.as_str(), "--dry-run"],
            vec!["task", "finalize", task.as_str()],
        ] {
            let out = rig.run(&rig.wt, &door);
            let seen = both(&out);
            assert_ok(&out, &format!("`jigc {}` ({workflow})", door.join(" ")));
            assert!(
                !seen.contains(CODE) && !seen.contains("reconciliation.conflict-block"),
                "a code-only task raises neither the guard nor a conflict nobody made; \
                 got:\n{seen}",
            );
        }
        assert_eq!(
            rig.git(&rig.wt, &["show", "--format=", "--name-only", "HEAD"]),
            file,
            "the commit carries the task's code and nothing else",
        );
    }
    assert_eq!(
        rig.git(&rig.main(), &["rev-parse", "HEAD"]),
        main_head,
        "the main checkout's branch did not move",
    );
    for checkout in [rig.main(), rig.wt.clone()] {
        assert_eq!(
            rig.git(&checkout, &["status", "--porcelain"]),
            "",
            "no managed path was touched in {checkout:?}",
        );
    }
    // The main checkout's own sweep still reads its store as clean — the shared baseline
    // was not rewritten with the linked worktree's bytes.
    let sweep = rig.run(&rig.main(), &["validate"]);
    assert_ok(&sweep, "`jigc validate` from the main checkout");
    assert!(
        !both(&sweep).contains("reconciliation."),
        "the store reconciles clean; got:\n{}",
        both(&sweep),
    );
}

// ---------------------------------------------------------------------------------------
// The four driven loss cells, as regression arms.
// ---------------------------------------------------------------------------------------

/// **C1, C3, C4** — the bytes the rc.24 door destroyed are the bytes this one leaves alone:
/// the write that would have opened each cell is refused, and the code-only finalize that
/// follows writes no managed path. Tested with the `(R3, F7)` copy-in baseline in the tree:
/// that fix guards the checkout the store binds to, this one keeps a task from promoting
/// into any other.
#[test]
fn the_linked_worktree_keeps_its_hand_edit_its_untracked_file_and_its_committed_wording() {
    let rig = Rig::new();

    // C4's precondition: the branch carries committed wording the main checkout lacks.
    let vision = rig.wt.join("VISION.md");
    let branch_wording = fs::read_to_string(&vision)
        .expect("read the worktree's vision")
        .replace(
            "A deterministic CLI assembles",
            "BRANCH-WORDING-7731 a deterministic CLI assembles",
        );
    assert!(branch_wording.contains("BRANCH-WORDING-7731"));
    fs::write(&vision, &branch_wording).expect("reword the branch's vision");
    rig.git(
        &rig.wt,
        &["commit", "-qam", "reword the thesis on the branch"],
    );

    let task = rig.mint(&rig.wt, "single-task", "sharpen the open questions");

    // C1: an uncommitted hand edit to a managed doc in the linked worktree.
    let hand_edited =
        format!("{branch_wording}\nA hand line written out of band HAND-MARK-9911.\n");
    fs::write(&vision, &hand_edited).expect("hand-edit the worktree's vision");
    // C3: an untracked file at the home a created ADR would promote to.
    let squatter = rig
        .wt
        .join("docs")
        .join("decisions")
        .join("fresh-decision.md");
    let notes = "# My notes\n\nUNTRACKED-NOTES-5150 never committed.\n";
    fs::write(&squatter, notes).expect("write the untracked notes");

    for (cell, args, stdin) in [
        (
            "C1/C4 — the copy-in",
            vec![
                "doc",
                "set-slot",
                "vision:vision#open-questions",
                "--from-file",
                "-",
            ],
            "Which domains earn a pack, and when.\n",
        ),
        (
            "C3 — the create",
            vec!["doc", "create", "adr", "--title", "Fresh decision"],
            "",
        ),
    ] {
        let mut argv = args.clone();
        argv.extend(["--task", task.as_str()]);
        let out = rig.run_stdin(&rig.wt, &argv, stdin);
        assert!(
            out.status.code() == Some(1) && both(&out).contains(CODE),
            "{cell} must be refused by the guard; got:\n{}",
            both(&out),
        );
    }

    // The task still lands its code, and finalize writes no managed path while doing it.
    fs::write(rig.wt.join("code.txt"), "code\n").expect("write code");
    rig.git(&rig.wt, &["add", "code.txt"]);
    rig.fill_commit(&rig.wt, &task, "a code change");
    assert_ok(
        &rig.run(&rig.wt, &["task", "finalize", &task]),
        "the code-only finalize",
    );

    assert_eq!(
        fs::read_to_string(&vision).expect("read the vision back"),
        hand_edited,
        "C1: the hand edit is still in the linked worktree, byte for byte",
    );
    assert_eq!(
        fs::read_to_string(&squatter).expect("read the notes back"),
        notes,
        "C3: the untracked file is still there, byte for byte",
    );
    assert!(
        rig.git(&rig.wt, &["show", "HEAD:VISION.md"])
            .contains("BRANCH-WORDING-7731"),
        "C4: the branch's committed wording is still what the branch's HEAD carries",
    );
    assert_eq!(
        rig.git(&rig.wt, &["show", "--format=", "--name-only", "HEAD"]),
        "code.txt",
        "the commit touched no managed doc",
    );
}

// ---------------------------------------------------------------------------------------
// The backstop — docs staged some other way, at the four doors that would promote them.
// ---------------------------------------------------------------------------------------

/// The four backstop doors, each as its argv and the exit a blocked run of it takes.
fn backstop_doors(task: &str) -> Vec<(&'static str, Vec<String>, i32)> {
    let argv = |args: &[&str]| args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
    vec![
        ("task validate", argv(&["task", "validate", task]), 3),
        (
            "task finalize --dry-run",
            argv(&["task", "finalize", task, "--dry-run"]),
            3,
        ),
        ("task finalize", argv(&["task", "finalize", task]), 3),
        // Orientation reports and never refuses: the finding rides the task's row.
        ("start (orientation)", argv(&["start"]), 0),
    ]
}

/// A doc staged from the main checkout and finalized from the linked worktree is refused at
/// every door that would have promoted it — the same `(code, target)` at all four — and
/// nothing is written. The route it prints, run as printed, lands the doc on the main
/// checkout's branch.
#[test]
fn the_backstop_refuses_a_staged_doc_at_all_four_doors_and_its_route_lands() {
    let rig = Rig::new();
    let main = rig.main();
    let task = rig.mint(&main, "single-task", "sharpen the thesis");
    assert_ok(
        &rig.run_stdin(
            &main,
            &[
                "doc",
                "set-slot",
                "vision:vision#thesis",
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            "STAGED-FROM-MAIN-4410 a sharper thesis.\n",
        ),
        "staging the doc from the main checkout",
    );
    rig.fill_commit(&main, &task, "sharpen the thesis");
    let feature_head = rig.git(&rig.wt, &["rev-parse", "HEAD"]);

    let mut routes = Vec::new();
    for (door, argv, exit) in backstop_doors(&task) {
        let mut argv: Vec<&str> = argv.iter().map(String::as_str).collect();
        argv.extend(["--format", "json"]);
        let out = rig.run(&rig.wt, &argv);
        assert_eq!(
            out.status.code(),
            Some(exit),
            "`jigc {door}` from the linked worktree; got:\n{}",
            both(&out),
        );
        let raised = guard(&out);
        assert_eq!(
            raised
                .iter()
                .map(|(target, _)| target.as_str())
                .collect::<Vec<_>>(),
            ["VISION.md"],
            "`jigc {door}`: the guard, once, keyed at the staged doc's home; got:\n{}",
            both(&out),
        );
        routes.push(raised[0].1.clone());
    }
    assert!(
        routes.windows(2).all(|pair| pair[0] == pair[1]),
        "one condition, one route, at every door; got:\n{routes:#?}",
    );
    // The text surface says it too, on the row of the task it is about.
    let orientation = rig.ok(&rig.wt, &["start"]);
    assert!(
        orientation.contains(CODE) && orientation.contains("at: VISION.md"),
        "bare `jigc start` names the finding in text; got:\n{orientation}",
    );
    assert_eq!(
        rig.git(&rig.wt, &["rev-parse", "HEAD"]),
        feature_head,
        "nothing committed on the worktree's branch",
    );
    assert_eq!(
        rig.git(&rig.wt, &["status", "--porcelain"]),
        "",
        "nothing written into the linked worktree",
    );

    // The route, as printed: stand in the main checkout and finalize the task there.
    let route = &routes[0];
    let cd = span(route, "`cd`", |s| s.starts_with("cd "));
    let finalize = span(route, "finalize", |s| s.starts_with("jigc task finalize "));
    assert_eq!(cd, format!("cd {}", rig.main_printed()));
    let landed = rig.sh(&rig.wt, &format!("{cd} && {finalize}"));
    assert_ok(&landed, &format!("the route `{cd}` then `{finalize}`"));
    assert!(
        rig.git(&main, &["show", "HEAD:VISION.md"])
            .contains("STAGED-FROM-MAIN-4410"),
        "the doc landed on the main checkout's branch",
    );
    assert_eq!(rig.git(&rig.wt, &["rev-parse", "HEAD"]), feature_head);
}

/// The backstop's other arm: when the main checkout's own finalize would refuse the task's
/// base pin, the route does **not** send the task there. A task minted on a branch whose
/// doc the main checkout lacks, with that doc staged, is routed at reading the staged copy
/// back, re-authoring from the main checkout and discarding — every step of which runs.
#[test]
fn the_backstop_never_routes_a_task_at_a_main_checkout_that_would_refuse_its_pin() {
    let rig = Rig::new();
    let main = rig.main();
    // The branch moves the doc; the main checkout does not have that commit.
    let vision = rig.wt.join("VISION.md");
    let reworded = fs::read_to_string(&vision)
        .expect("read the worktree's vision")
        .replace(
            "A deterministic CLI",
            "BRANCH-ONLY-2207 a deterministic CLI",
        );
    fs::write(&vision, reworded).expect("reword on the branch");
    rig.git(&rig.wt, &["commit", "-qam", "reword on the branch"]);

    // Minted here (pinned to the branch), staged from the main checkout — the shape a task
    // an older binary staged from this worktree is in.
    let task = rig.mint(&rig.wt, "single-task", "sharpen the thesis");
    assert_ok(
        &rig.run_stdin(
            &main,
            &[
                "doc",
                "set-slot",
                "vision:vision#thesis",
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            "A sharper thesis.\n",
        ),
        "staging the doc from the main checkout",
    );

    let out = rig.run(&rig.wt, &["task", "finalize", &task, "--format", "json"]);
    assert_eq!(out.status.code(), Some(3), "got:\n{}", both(&out));
    let raised = guard(&out);
    assert_eq!(raised.len(), 1, "got:\n{}", both(&out));
    let route = &raised[0].1;
    assert!(
        !spans(route)
            .iter()
            .any(|s| s.starts_with("jigc task finalize")),
        "the route must not send this task to a door that refuses its pin; got:\n{route}",
    );
    // …and the door it declines to name really does refuse, on the pin.
    let from_main = rig.run(&main, &["task", "finalize", &task]);
    assert!(
        both(&from_main).contains("finalize.base-mismatch"),
        "the main checkout's finalize refuses this task's base pin — which is why the \
         route does not go there; got:\n{}",
        both(&from_main),
    );

    // The route that is printed, as printed.
    let show = span(route, "staged read", |s| s.starts_with("jigc doc show "));
    let cd = span(route, "`cd`", |s| s.starts_with("cd "));
    let start = span(route, "start", |s| s.starts_with("jigc start "));
    let discard = span(route, "discard", |s| s.starts_with("jigc task discard "));
    let read_back = rig.sh(&rig.wt, &show);
    assert_ok(&read_back, &format!("`{show}`"));
    assert!(text(&read_back.stdout).contains("A sharper thesis."));
    assert_ok(
        &rig.sh(
            &rig.wt,
            &format!(
                "{cd} && {}",
                start.replace("<intent>", "sharpen the thesis again")
            ),
        ),
        &format!("`{cd}` then `{start}`"),
    );
    assert_ok(&rig.sh(&rig.wt, &discard), &format!("`{discard}`"));
    assert!(
        rig.git(&rig.wt, &["show", "HEAD:VISION.md"])
            .contains("BRANCH-ONLY-2207"),
        "the branch's committed wording is untouched",
    );
}

/// **The backstop never routes a task at a main checkout whose own staged work its finalize
/// would commit** (the completion audit's F4). The carryover gate compares against the
/// snapshot taken where the task was minted, so a task minted in the linked worktree is one
/// the main checkout's gate is blind for: with the user's own `wip.txt` staged there before
/// any task existed, the route as first printed — *"finalize this task from the main
/// checkout … commits its docs"* — exited 0 with `added wip.txt`.
///
/// So the route does not name that finalize. It names what is in the way and the exit that
/// always runs, and every step is run as printed: the re-authored task's own finalize
/// answers `finalize.carried-staged` for the file, its route unstages it, and the commit
/// that lands carries the doc alone — the file still on disk, in no commit.
///
/// **And the cell it must not over-refuse:** a task whose snapshot *covers* the staged file
/// (minted in the main checkout, after the file was staged) is still routed at that
/// checkout's finalize, because that door's own carryover gate answers there.
#[test]
fn the_backstop_never_routes_a_task_at_a_main_checkout_whose_staged_work_it_would_commit() {
    let rig = Rig::new();
    let main = rig.main();
    const WIP: &str = "the user's own work in progress\n";
    fs::write(main.join("wip.txt"), WIP).expect("write the user's own file");
    rig.git(&main, &["add", "wip.txt"]);
    let head = rig.git(&main, &["rev-parse", "HEAD"]);
    let stage = |task: &str| {
        assert_ok(
            &rig.run_stdin(
                &main,
                &[
                    "doc",
                    "set-slot",
                    "vision:vision#thesis",
                    "--from-file",
                    "-",
                    "--task",
                    task,
                ],
                "NOT-SWEPT-7731 a sharper thesis.\n",
            ),
            "staging the doc from the main checkout",
        );
    };
    let carried = |out: &Output| -> Vec<(String, String)> {
        findings(out)
            .into_iter()
            .filter(|(code, _, _)| code == "finalize.carried-staged")
            .map(|(_, target, route)| (target, route))
            .collect()
    };

    // Minted in the linked worktree: its pre-task snapshot is that index's — empty.
    let task = rig.mint(&rig.wt, "single-task", "sharpen the thesis");
    stage(&task);
    let out = rig.run(&rig.wt, &["task", "finalize", &task, "--format", "json"]);
    assert_eq!(out.status.code(), Some(3), "got:\n{}", both(&out));
    let raised = guard(&out);
    assert_eq!(raised.len(), 1, "got:\n{}", both(&out));
    let route = raised[0].1.clone();
    assert!(
        !spans(&route)
            .iter()
            .any(|s| s.starts_with("jigc task finalize")),
        "the route must not send this task to a finalize that would commit the main \
         checkout's own staged work; got:\n{route}",
    );
    let listing = span(&route, "staged listing", |s| {
        s.starts_with("git ") && s.ends_with(" diff --cached --name-only")
    });
    let listed = rig.sh(&rig.wt, &listing);
    assert_ok(&listed, &format!("`{listing}`"));
    assert_eq!(
        text(&listed.stdout).trim(),
        "wip.txt",
        "the route names what is in the way",
    );

    // Must not over-refuse: a task minted HERE, after the file was staged, has it in its
    // snapshot — so the route is that checkout's finalize, and that door's own gate answers.
    let covered = rig.mint(&main, "single-task", "a task that saw the file staged");
    stage(&covered);
    rig.fill_commit(&main, &covered, "a task that saw the file staged");
    let out = rig.run(&rig.wt, &["task", "finalize", &covered, "--format", "json"]);
    let direct = guard(&out)
        .pop()
        .unwrap_or_else(|| panic!("the backstop still refuses from here; got:\n{}", both(&out)))
        .1;
    let cd = span(&direct, "`cd`", |s| s.starts_with("cd "));
    let finalize = span(&direct, "finalize", |s| {
        s.starts_with("jigc task finalize ")
    });
    let answered = rig.sh(&rig.wt, &format!("{cd} && {finalize} --format json"));
    assert_eq!(answered.status.code(), Some(3), "got:\n{}", both(&answered));
    assert_eq!(
        carried(&answered)
            .iter()
            .map(|(target, _)| target.as_str())
            .collect::<Vec<_>>(),
        ["wip.txt"],
        "the main checkout's own carryover gate answers for a file the snapshot covers; \
         got:\n{}",
        both(&answered),
    );
    assert_ok(
        &rig.run(&main, &["task", "discard", &covered, "--force"]),
        "discarding the covered task",
    );

    // The printed route, as printed: read back, re-author in a task started there, discard.
    let show = span(&route, "staged read", |s| s.starts_with("jigc doc show "));
    let cd = span(&route, "`cd`", |s| s.starts_with("cd "));
    let start = span(&route, "start", |s| s.starts_with("jigc start "));
    let discard = span(&route, "discard", |s| s.starts_with("jigc task discard "));
    let read_back = rig.sh(&rig.wt, &show);
    assert_ok(&read_back, &format!("`{show}`"));
    assert!(text(&read_back.stdout).contains("NOT-SWEPT-7731"));
    assert_ok(
        &rig.sh(
            &rig.wt,
            &format!(
                "{cd} && {}",
                start.replace("<intent>", "sharpen the thesis again")
            ),
        ),
        &format!("`{cd}` then `{start}`"),
    );
    assert_ok(&rig.sh(&rig.wt, &discard), &format!("`{discard}`"));
    let again = rig.mint(&main, "single-task", "sharpen the thesis again");
    stage(&again);
    rig.fill_commit(&main, &again, "sharpen the thesis");

    // That task's own finalize answers for the file — the carryover gate, with its route.
    let gated = rig.run(&main, &["task", "finalize", &again, "--format", "json"]);
    assert_eq!(gated.status.code(), Some(3), "got:\n{}", both(&gated));
    let answer = carried(&gated);
    assert_eq!(
        answer
            .iter()
            .map(|(target, _)| target.as_str())
            .collect::<Vec<_>>(),
        ["wip.txt"],
        "got:\n{}",
        both(&gated),
    );
    assert_eq!(
        rig.git(&main, &["rev-parse", "HEAD"]),
        head,
        "nothing landed"
    );
    let unstage = span(&answer[0].1, "unstage", |s| {
        s.starts_with("git ") && s.contains(" restore --staged ")
    });
    assert_ok(&rig.sh(&rig.wt, &unstage), &format!("`{unstage}`"));
    assert_ok(
        &rig.run(&main, &["task", "finalize", &again]),
        "the re-authored task's finalize, once the file is unstaged",
    );
    assert_eq!(
        rig.git(&main, &["show", "--format=", "--name-only", "HEAD"]),
        "VISION.md",
        "the commit carries the doc and nothing the task did not stage",
    );
    assert!(
        rig.git(&main, &["show", "HEAD:VISION.md"])
            .contains("NOT-SWEPT-7731")
    );
    assert_eq!(
        fs::read_to_string(main.join("wip.txt")).expect("the user's file is still on disk"),
        WIP,
    );
    assert_eq!(
        rig.git(&main, &["log", "--all", "--format=%H", "--", "wip.txt"]),
        "",
        "the user's file is in no commit",
    );
}

// ---------------------------------------------------------------------------------------
// The write door's route, run as printed.
// ---------------------------------------------------------------------------------------

/// The docs-only route and the mixed one: the refusal sends the doc to a task started from
/// the main checkout, and says this task stays usable for code. Both halves run — the doc
/// lands from the main checkout, the code from the worktree, as two commits.
#[test]
fn the_write_door_route_lands_the_doc_from_the_main_checkout_and_the_code_from_here() {
    let rig = Rig::new();
    let main = rig.main();
    let task = rig.mint(&rig.wt, "single-task", "a mixed change");
    let refused = rig.run_stdin(
        &rig.wt,
        &[
            "doc",
            "set-slot",
            "vision:vision#thesis",
            "--from-file",
            "-",
            "--task",
            &task,
            "--format",
            "json",
        ],
        "A thesis.\n",
    );
    let route = guard(&refused)
        .pop()
        .unwrap_or_else(|| panic!("the write is refused; got:\n{}", both(&refused)))
        .1;
    assert!(
        !route.contains("git ") || !route.contains("stash"),
        "with nothing staged in this checkout the route is the short one; got:\n{route}",
    );

    // Half one, as printed: stand in the main checkout and start the doc's task there.
    let cd = span(&route, "`cd`", |s| s.starts_with("cd "));
    let start = span(&route, "start", |s| s.starts_with("jigc start "));
    assert_ok(
        &rig.sh(
            &rig.wt,
            &format!(
                "{cd} && {}",
                start.replace("<intent>", "sharpen the thesis")
            ),
        ),
        &format!("`{cd}` then `{start}`"),
    );
    // The front door presents the workflows; a doc task minted there writes and lands.
    let doc_task = rig.mint(&main, "single-task", "sharpen the thesis");
    assert_ok(
        &rig.run_stdin(
            &main,
            &[
                "doc",
                "set-slot",
                "vision:vision#thesis",
                "--from-file",
                "-",
                "--task",
                &doc_task,
            ],
            "FROM-MAIN-3318 a sharper thesis.\n",
        ),
        "the doc write from the main checkout",
    );
    rig.fill_commit(&main, &doc_task, "sharpen the thesis");
    assert_ok(
        &rig.run(&main, &["task", "finalize", &doc_task]),
        "the doc task's finalize from the main checkout",
    );
    assert!(
        rig.git(&main, &["show", "HEAD:VISION.md"])
            .contains("FROM-MAIN-3318")
    );

    // Half two, as printed: this task commits its code on this branch.
    let finalize = span(&route, "finalize", |s| s.starts_with("jigc task finalize "));
    fs::write(rig.wt.join("code.txt"), "code\n").expect("write code");
    rig.git(&rig.wt, &["add", "code.txt"]);
    rig.fill_commit(&rig.wt, &task, "the code half");
    assert_ok(&rig.sh(&rig.wt, &finalize), &format!("`{finalize}`"));
    assert_eq!(
        rig.git(&rig.wt, &["show", "--format=", "--name-only", "HEAD"]),
        "code.txt",
    );
}

/// **The dangling-anchor cell.** Code staged in the linked worktree renames a symbol a
/// committed ADR cites, so the finalize floor blocks `doc-code.symbol-exists` — and the
/// repair it routes at is a doc write this checkout refuses. That refusal names the one
/// exit there is: take the staged change to the main checkout and land it there with its
/// repair, as one commit. Run as printed, on a branch that is level with the main checkout
/// and on one that is a commit ahead of it.
#[test]
fn the_dangling_anchor_exit_lands_the_code_and_its_repair_from_the_main_checkout() {
    for ahead in [false, true] {
        let arm = if ahead {
            "branch ahead"
        } else {
            "branches level"
        };
        let rig = Rig::new();
        let main = rig.main();
        if ahead {
            fs::write(rig.wt.join("side.txt"), "side\n").expect("write a branch-only file");
            rig.git(&rig.wt, &["add", "side.txt"]);
            rig.git(&rig.wt, &["commit", "-qm", "a branch-only commit"]);
        }
        let task = rig.mint(&rig.wt, "single-task", "rename the getter");
        fs::write(
            rig.wt.join("src").join("lib.rs"),
            "pub fn cache_fetch() -> u32 { 1 }\n",
        )
        .expect("rename the cited symbol");
        rig.git(&rig.wt, &["add", "src/lib.rs"]);
        rig.fill_commit(&rig.wt, &task, "rename the getter");

        let floor = rig.run(&rig.wt, &["task", "finalize", &task]);
        assert!(
            floor.status.code() == Some(3) && both(&floor).contains("doc-code.symbol-exists"),
            "{arm}: the rename dangles the committed citation; got:\n{}",
            both(&floor),
        );

        let repair = [
            "doc",
            "set-field",
            "adr:single-node-cache#cites-code",
            "--value",
            "src/lib.rs#cache_fetch",
        ];
        let mut argv = repair.to_vec();
        argv.extend(["--task", task.as_str(), "--format", "json"]);
        let refused = rig.run(&rig.wt, &argv);
        let route = guard(&refused)
            .pop()
            .unwrap_or_else(|| {
                panic!(
                    "{arm}: the repair is refused here; got:\n{}",
                    both(&refused)
                )
            })
            .1;

        // The exit, lifted span by span out of the printed route.
        let stash = span(&route, "stash", |s| {
            s.starts_with("git ") && s.ends_with(" stash")
        });
        let cd = span(&route, "`cd`", |s| s.starts_with("cd "));
        let merge = span(&route, "merge", |s| {
            s.starts_with("git ") && s.contains(" merge ")
        });
        let start = span(&route, "mint", |s| s.starts_with("jigc start --workflow "));
        let pop = span(&route, "pop", |s| {
            s.starts_with("git ") && s.ends_with(" stash pop --index")
        });
        let discard = span(&route, "discard", |s| s.starts_with("jigc task discard "));
        let start = start.replace("<intent>", "rename the getter from main");

        let moved = rig.sh(
            &rig.wt,
            &format!("set -e; {stash}; {cd}; {merge}; {start}; {pop}"),
        );
        assert_ok(
            &moved,
            &format!("{arm}: the route up to the restored change"),
        );
        let there = minted(&text(&moved.stdout));

        // "make this write and finalize there" — the refused argv, from the main checkout.
        let mut argv = repair.to_vec();
        argv.extend(["--task", there.as_str()]);
        assert_ok(
            &rig.run(&main, &argv),
            &format!("{arm}: the repair from the main checkout"),
        );
        rig.fill_commit(&main, &there, "rename the getter");
        let landed = rig.run(&main, &["task", "finalize", &there]);
        assert_ok(
            &landed,
            &format!("{arm}: the finalize from the main checkout"),
        );
        assert_ok(&rig.sh(&main, &discard), &format!("{arm}: `{discard}`"));

        let paths = rig.git(&main, &["show", "--format=", "--name-only", "HEAD"]);
        assert_eq!(
            paths.lines().collect::<Vec<_>>(),
            [ADR_HOME, "src/lib.rs"],
            "{arm}: ONE commit carries the rename and the repaired citation",
        );
        assert!(
            rig.git(&main, &["show", &format!("HEAD:{ADR_HOME}")])
                .contains("src/lib.rs#cache_fetch")
        );
        for checkout in [main.clone(), rig.wt.clone()] {
            assert_eq!(
                rig.git(&checkout, &["status", "--porcelain"]),
                "",
                "{arm}: nothing is left staged or dirty in {checkout:?}",
            );
        }
        assert!(
            rig.ok(&main, &["task", "list"]).contains("no active tasks"),
            "{arm}: no task is left behind",
        );
    }
}

// ---------------------------------------------------------------------------------------
// The mint doors that cannot land.
// ---------------------------------------------------------------------------------------

/// The shipped workflows on the doc-only commit model — read off the pack directories, so
/// a fifth one joins this axis the day it ships.
fn doc_only_workflows() -> Vec<String> {
    let mut ids = Vec::new();
    for pack in [cli::pack_path!(dev), cli::pack_path!(methodology)] {
        let dir = Path::new(pack).join("workflows");
        for entry in fs::read_dir(&dir)
            .expect("read a pack's workflows")
            .flatten()
        {
            let path = entry.path();
            let body = fs::read_to_string(&path).unwrap_or_default();
            if body.contains("finalize-doc-only") {
                ids.push(
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .expect("a workflow file stem")
                        .to_string(),
                );
            }
        }
    }
    ids.sort();
    ids
}

/// A workflow whose finalize commits its docs and nothing else is refused **before it
/// mints** from a linked worktree — no arm of that task could land — and the route's own
/// command mints it from the main checkout. `single-task` and its ordinary siblings, whose
/// product may be code alone, are never refused at the mint.
#[test]
fn a_doc_only_workflow_is_refused_at_the_mint_and_an_ordinary_one_never_is() {
    let rig = Rig::new();
    let doc_only = doc_only_workflows();
    assert!(
        doc_only.len() >= 4,
        "the doc-only commit model ships at least four workflows; found {doc_only:?}",
    );
    for workflow in &doc_only {
        // Triage workflows take an existing finding; the mint is refused before any of
        // that is read, which is the point.
        let out = rig.run(
            &rig.wt,
            &[
                "start",
                "--workflow",
                workflow,
                "a finding",
                "--format",
                "json",
            ],
        );
        assert_eq!(
            out.status.code(),
            Some(1),
            "`{workflow}`; got:\n{}",
            both(&out)
        );
        let raised = guard(&out);
        assert_eq!(
            raised
                .iter()
                .map(|(target, _)| target.as_str())
                .collect::<Vec<_>>(),
            [printed(&rig.wt)],
            "`{workflow}`: the guard, keyed at the checkout the mint would have committed \
             in; got:\n{}",
            both(&out),
        );
        assert!(
            rig.ok(&rig.wt, &["task", "list"])
                .contains("no active tasks"),
            "`{workflow}`: a refused mint leaves no task behind",
        );
        let route = &raised[0].1;
        let cd = span(route, "`cd`", |s| s.starts_with("cd "));
        let start = span(route, "start", |s| s.starts_with("jigc start --workflow "));
        assert_eq!(cd, format!("cd {}", rig.main_printed()), "got:\n{route}");
        assert!(start.contains(workflow.as_str()), "got:\n{route}");
    }
    // One of them, as printed, to the mint: the report workflow needs nothing but an intent.
    let report = doc_only
        .iter()
        .find(|id| id.starts_with("report-"))
        .expect("a report workflow");
    let out = rig.run(
        &rig.wt,
        &["start", "--workflow", report, "x", "--format", "json"],
    );
    let route = guard(&out).pop().expect("the refusal").1;
    let cd = span(&route, "`cd`", |s| s.starts_with("cd "));
    let start = span(&route, "start", |s| s.starts_with("jigc start --workflow "))
        .replace("<intent>", "a finding from the main checkout");
    let there = rig.sh(&rig.wt, &format!("{cd} && {start}"));
    assert_ok(&there, &format!("`{cd}` then `{start}`"));
    let task = minted(&text(&there.stdout));
    assert_ok(
        &rig.run(&rig.main(), &["task", "discard", &task, "--force"]),
        "tidying the task the route minted",
    );

    for workflow in ["single-task", "quick-fix", "record-change"] {
        let out = rig.run(
            &rig.wt,
            &["start", "--workflow", workflow, &format!("try {workflow}")],
        );
        assert_ok(
            &out,
            &format!("minting `{workflow}` from a linked worktree"),
        );
        assert!(
            text(&out.stdout).contains("task minted: "),
            "`{workflow}` mints here — its first promoting write is what refuses",
        );
    }
}

/// `jigc migrate` from a linked worktree refuses before the mint. Driven on `1.0.0-rc.24`
/// it minted under the worktree's own `.jigc/tasks/`, where no other door looks; now nothing
/// is minted anywhere, and the route's command — the main checkout's copy of the source,
/// spelled absolute — mints there once the branch's file has reached that checkout.
#[test]
fn migrate_from_a_linked_worktree_refuses_before_the_mint() {
    let rig = Rig::new();
    let main = rig.main();
    fs::write(
        rig.wt.join("NOTES.md"),
        "# Notes\n\nA decision somebody wrote down by hand, long enough to migrate.\n",
    )
    .expect("write the foreign source");
    rig.git(&rig.wt, &["add", "NOTES.md"]);
    rig.git(&rig.wt, &["commit", "-qm", "add the notes"]);

    let out = rig.run(
        &rig.wt,
        &["migrate", "NOTES.md", "--as", "adr", "--format", "json"],
    );
    assert_eq!(out.status.code(), Some(1), "got:\n{}", both(&out));
    let raised = guard(&out);
    assert_eq!(
        raised
            .iter()
            .map(|(target, _)| target.as_str())
            .collect::<Vec<_>>(),
        [printed(&rig.wt)],
        "the guard, keyed at the checkout; got:\n{}",
        both(&out),
    );
    for checkout in [&rig.wt, &main] {
        let tasks = checkout.join(".jigc").join("tasks");
        let minted_any = fs::read_dir(&tasks)
            .map(|entries| entries.flatten().count())
            .unwrap_or(0);
        assert_eq!(minted_any, 0, "nothing minted under {tasks:?}");
    }

    // Argument faults keep their rank: an unknown doctype is wrong from every checkout.
    let unknown = rig.run(&rig.wt, &["migrate", "NOTES.md", "--as", "no-such-doctype"]);
    assert!(
        !both(&unknown).contains(CODE),
        "an unknown doctype answers as that; got:\n{}",
        both(&unknown),
    );

    // The route, as printed — after the step its own tail names: the branch's file has to
    // reach the main checkout first.
    let route = &raised[0].1;
    let cd = span(route, "`cd`", |s| s.starts_with("cd "));
    let migrate = span(route, "migrate", |s| s.starts_with("jigc migrate "));
    assert!(route.contains("merge the branch"), "got:\n{route}");
    rig.git(&main, &["merge", "-q", "feature"]);
    let there = rig.sh(&rig.wt, &format!("{cd} && {migrate}"));
    assert_ok(&there, &format!("`{cd}` then `{migrate}`"));
    let task = minted(&text(&there.stdout));
    assert!(
        rig.ok(&main, &["task", "list"]).contains(&task),
        "the migration task is on the one roster every door reads",
    );
}

// ---------------------------------------------------------------------------------------
// Stated where it binds, and silent where it does not.
// ---------------------------------------------------------------------------------------

/// The mint, the resume and bare `jigc start` each say this checkout commits code only —
/// in text, and only from the linked worktree. From the main checkout none of the three
/// moves a byte, and from neither does a pinned JSON surface gain a key.
#[test]
fn the_mint_the_resume_and_orientation_state_the_checkout_only_where_it_binds() {
    let rig = Rig::new();
    let main = rig.main();
    let lead = "checkout: the linked worktree at";
    let rule = "commits here, and code only";

    let mint_wt = rig.ok(
        &rig.wt,
        &["start", "--workflow", "single-task", "from the worktree"],
    );
    let mint_main = rig.ok(
        &main,
        &[
            "start",
            "--workflow",
            "single-task",
            "from the main checkout",
        ],
    );
    let (task_wt, task_main) = (minted(&mint_wt), minted(&mint_main));
    let surfaces = [
        ("the mint", mint_wt, mint_main),
        (
            "the resume",
            rig.ok(&rig.wt, &["start", "--task", &task_wt]),
            rig.ok(&main, &["start", "--task", &task_main]),
        ),
        (
            "bare `jigc start`",
            rig.ok(&rig.wt, &["start"]),
            rig.ok(&main, &["start"]),
        ),
    ];
    for (surface, from_wt, from_main) in &surfaces {
        assert!(
            from_wt.contains(lead)
                && from_wt.contains(rule)
                && from_wt.contains("on branch `feature`")
                && from_wt.contains(&format!("`cd {}`", rig.main_printed())),
            "{surface} from the linked worktree states the checkout, its branch, the rule \
             and where docs are written; got:\n{from_wt}",
        );
        assert!(
            !from_main.contains("checkout:"),
            "{surface} from the main checkout says nothing new; got:\n{from_main}",
        );
    }
    // A resume of the worktree's task typed in the MAIN checkout stands in the main
    // checkout, so it states nothing either: the predicate is where the door is typed.
    assert!(
        !rig.run(&main, &["start", "--task", &task_wt])
            .stdout
            .windows(lead.len())
            .any(|window| window == lead.as_bytes())
    );

    // The pinned machine surfaces carry no trace of it.
    for argv in [
        vec!["start", "--task", task_wt.as_str(), "--format", "json"],
        vec!["start", "--format", "json"],
    ] {
        let keys = |cwd: &Path| -> Vec<String> {
            let out = rig.run(cwd, &argv);
            assert_ok(&out, &format!("`jigc {}`", argv.join(" ")));
            assert!(
                !text(&out.stdout).contains("checkout:"),
                "the statement is text-only; got:\n{}",
                text(&out.stdout),
            );
            let value: serde_json::Value =
                serde_json::from_slice(&out.stdout).expect("a JSON document");
            let mut keys: Vec<String> = value
                .as_object()
                .expect("an object")
                .keys()
                .cloned()
                .collect();
            keys.sort();
            keys
        };
        assert_eq!(
            keys(&rig.wt),
            keys(&main),
            "`jigc {}`: the same key set from both checkouts",
            argv.join(" "),
        );
    }
}

/// The committed-store reads keep serving the main checkout and say so in one stderr line
/// from the linked worktree — with stdout byte-identical to the same read typed in the main
/// checkout, in both formats.
#[test]
fn the_store_reads_say_which_checkout_answered_and_stdout_does_not_move() {
    let rig = Rig::new();
    let main = rig.main();
    let note = "served from the main checkout";
    for read in [
        vec!["doc", "show", "vision:vision"],
        vec!["doc", "show", "vision:vision", "--format", "json"],
        vec!["doc", "list"],
        vec!["doc", "list", "--format", "json"],
        vec!["validate"],
        vec!["validate", "--format", "json"],
    ] {
        let what = format!("`jigc {}`", read.join(" "));
        let (from_wt, from_main) = (rig.run(&rig.wt, &read), rig.run(&main, &read));
        assert_ok(&from_wt, &what);
        assert_ok(&from_main, &what);
        assert_eq!(
            text(&from_wt.stdout),
            text(&from_main.stdout),
            "{what}: stdout is the pinned read, identical from both checkouts",
        );
        let err = text(&from_wt.stderr);
        assert!(
            err.contains(note) && err.contains(&rig.main_printed()),
            "{what} from the linked worktree names the checkout that answered; stderr:\n{err}",
        );
        assert_eq!(
            err.matches(note).count(),
            1,
            "{what}: one line, once; stderr:\n{err}",
        );
        assert!(
            !text(&from_main.stderr).contains(note),
            "{what} from the main checkout says nothing new; stderr:\n{}",
            text(&from_main.stderr),
        );
    }
}

/// The changelog-gate advisory's route, from a code-only checkout: every verb it names runs
/// from where it says to run it. Unpromoted, the entry is a commit of its own from the main
/// checkout; promoted to `blocking`, the change itself has to land there, and the route
/// names that exit and the gate's own lowering — never an in-task write this checkout
/// refuses.
#[test]
fn the_changelog_advisory_routes_at_the_main_checkout_from_a_code_only_checkout() {
    let rig = Rig::new();
    let main = rig.main();
    let gate = "changelog-recording.gate-granted-unused";
    let task = rig.mint(&rig.wt, "single-task", "a user-facing change");
    fs::write(rig.wt.join("code.txt"), "code\n").expect("write code");
    rig.git(&rig.wt, &["add", "code.txt"]);
    rig.fill_commit(&rig.wt, &task, "a user-facing change");

    let route_of = |out: &Output| -> String {
        findings(out)
            .into_iter()
            .find(|(code, _, _)| code == gate)
            .unwrap_or_else(|| panic!("the advisory is raised; got:\n{}", both(out)))
            .2
    };
    let advisory = rig.run(&rig.wt, &["task", "validate", &task, "--format", "json"]);
    assert_ok(&advisory, "`jigc task validate`");
    let route = route_of(&advisory);
    assert!(
        !route.contains("jigc doc "),
        "no in-task doc write is offered from a checkout that refuses it; got:\n{route}",
    );
    let cd = span(&route, "`cd`", |s| s.starts_with("cd "));
    let start = span(&route, "start", |s| {
        s.starts_with("jigc start --workflow record-change")
    })
    .replace("<what changed>", "note the change");
    let there = rig.sh(&rig.wt, &format!("{cd} && {start}"));
    assert_ok(&there, &format!("`{cd}` then `{start}`"));
    assert_ok(
        &rig.run(
            &main,
            &["task", "discard", &minted(&text(&there.stdout)), "--force"],
        ),
        "tidying the task the route minted",
    );

    // Promoted to blocking: the route names the relocation and the gate's own exit.
    rig.ok(
        &main,
        &[
            "config",
            "set",
            &format!("validation.{gate}.severity"),
            "blocking",
        ],
    );
    let blocked = rig.run(&rig.wt, &["task", "validate", &task, "--format", "json"]);
    assert_eq!(blocked.status.code(), Some(3), "got:\n{}", both(&blocked));
    let route = route_of(&blocked);
    assert!(
        !route.contains("jigc doc "),
        "promoted, the route still offers no write this checkout refuses; got:\n{route}",
    );
    for (what, pick) in [
        ("stash", " stash"),
        ("mint", "jigc start --workflow single-task"),
        ("pop", "stash pop --index"),
        ("discard", "jigc task discard"),
    ] {
        assert!(
            spans(&route).iter().any(|s| s.contains(pick)),
            "the promoted route names the {what} step; got:\n{route}",
        );
    }
    let lower = span(&route, "config set", |s| s.starts_with("jigc config set "));
    assert_ok(&rig.sh(&rig.wt, &lower), &format!("`{lower}`"));
    assert_ok(
        &rig.run(&rig.wt, &["task", "finalize", &task]),
        "the finalize, once the gate is lowered as the route says",
    );
}

/// One repository layout in which `.git` is a **file** and yet no second checkout exists —
/// the directory jigc resolves as its home (`dirname(git-common-dir)`) is not a work tree.
struct HomelessLayout {
    name: &'static str,
    /// The one checkout the layout has — where every command below is typed.
    checkout: PathBuf,
    /// The line-ending conversion this cell runs under — the audit's other axis, walked on
    /// the diagonal: the predicate reads no file byte, so one setting per layout is what
    /// shows it stays out of the way of each.
    conversion: Conversion,
}

#[derive(Clone, Copy, Debug)]
enum Conversion {
    None,
    AutocrlfTrue,
    AutocrlfInput,
    /// An uncommitted `.gitattributes` carrying `* text=auto`.
    TextAuto,
}

/// Build every layout of the class under `root`, each from `source` (a repository with one
/// commit and no jigc install), and give each the committer identity the source carries
/// locally — a clone copies no local config.
fn homeless_layouts(rig: &Rig, root: &Path, source: &Path) -> Vec<HomelessLayout> {
    let branch = rig.git(source, &["branch", "--show-current"]);
    let utf8 = |path: &Path| path.to_str().expect("a UTF-8 path").to_string();
    let source = utf8(source);
    let mut layouts = Vec::new();

    // A bare repository behind a `gitdir:` pointer, its one worktree beside it.
    let pointer = root.join("pointer-to-bare");
    fs::create_dir_all(&pointer).expect("mk the pointer layout");
    rig.git(
        root,
        &[
            "clone",
            "-q",
            "--bare",
            &source,
            &utf8(&pointer.join(".bare")),
        ],
    );
    fs::write(pointer.join(".git"), "gitdir: ./.bare\n").expect("write the pointer");
    rig.git(
        &pointer,
        &[
            "worktree",
            "add",
            "-q",
            &utf8(&pointer.join("main")),
            &branch,
        ],
    );
    layouts.push(HomelessLayout {
        name: "a bare repository behind a `gitdir:` pointer",
        checkout: pointer.join("main"),
        conversion: Conversion::None,
    });

    // A sibling bare repository and a worktree of it.
    let sibling = root.join("sibling-bare");
    fs::create_dir_all(&sibling).expect("mk the sibling layout");
    let bare = sibling.join("proj.git");
    rig.git(root, &["clone", "-q", "--bare", &source, &utf8(&bare)]);
    rig.git(
        &bare,
        &[
            "worktree",
            "add",
            "-q",
            &utf8(&sibling.join("main")),
            &branch,
        ],
    );
    layouts.push(HomelessLayout {
        name: "a worktree of a sibling bare repository",
        checkout: sibling.join("main"),
        conversion: Conversion::AutocrlfTrue,
    });

    // `--separate-git-dir`: one checkout whose git dir lives elsewhere.
    let separate = root.join("separate-git-dir");
    fs::create_dir_all(&separate).expect("mk the separate-git-dir layout");
    rig.git(
        root,
        &[
            "clone",
            "-q",
            "--separate-git-dir",
            &utf8(&separate.join("gitdir")),
            &source,
            &utf8(&separate.join("work")),
        ],
    );
    layouts.push(HomelessLayout {
        name: "`--separate-git-dir`",
        checkout: separate.join("work"),
        conversion: Conversion::AutocrlfInput,
    });

    // A submodule: its git dir is under the superproject's `.git/modules/`.
    let superproject = root.join("super");
    fs::create_dir_all(&superproject).expect("mk the superproject");
    rig.git(&superproject, &["init", "-q"]);
    rig.git(
        &superproject,
        &[
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "--quiet",
            "add",
            &source,
            "sub",
        ],
    );
    layouts.push(HomelessLayout {
        name: "a submodule",
        checkout: superproject.join("sub"),
        conversion: Conversion::TextAuto,
    });

    for layout in &layouts {
        assert!(
            layout.checkout.join(".git").is_file(),
            "{}: the fixture must be a checkout whose `.git` is a file",
            layout.name,
        );
        rig.git(
            &layout.checkout,
            &["config", "user.email", "trial@example.com"],
        );
        rig.git(&layout.checkout, &["config", "user.name", "Trial Corpus"]);
        match layout.conversion {
            Conversion::None => {}
            Conversion::AutocrlfTrue => {
                rig.git(&layout.checkout, &["config", "core.autocrlf", "true"]);
            }
            Conversion::AutocrlfInput => {
                rig.git(&layout.checkout, &["config", "core.autocrlf", "input"]);
            }
            Conversion::TextAuto => {
                fs::write(layout.checkout.join(".gitattributes"), "* text=auto\n")
                    .expect("write .gitattributes");
            }
        }
    }
    layouts
}

/// **MUST NOT REFUSE — every layout where `.git` is a file and no second checkout exists**
/// (the rc.24 fix pass, completion audit F1). The guard's predicate is *the doc store's home
/// is itself a checkout, distinct from the standing one*, and it is asked of git. A worktree
/// of a bare repository, a `--separate-git-dir` checkout and a submodule all keep `.git` as
/// a file while the directory jigc resolves as its home is no work tree at all — so there is
/// no main checkout to commit a doc from, nothing to route at, and `1.0.0-rc.24` landed a
/// created doc in each. Driven on the build that shipped the guard, every one refused the
/// create and printed a `cd` into a directory where no jigc door runs.
///
/// The must-refuse cell beside these is every other arm of this suite: a linked worktree the
/// user made beside a real main checkout.
///
/// Each cell walks the whole of what rc.24 did there — setup, a doc-only mint, the create,
/// the finalize — to the doc in `HEAD`, and asserts the guard said nothing on the way: no
/// `checkout:` block, no finding, no served-from note. `jigc setup` exits 1 in two of the
/// layouts on rc.24 too (`setup.install-hook`, the hooks dir asked of a directory that is
/// no repository) and leaves the project layer behind, so its exit is not the cell's claim.
#[test]
fn a_layout_whose_doc_home_is_no_checkout_lands_a_doc_as_it_did_before_the_guard() {
    let rig = Rig::never_adopted();
    let source = rig.main();
    let root = source.parent().expect("the corpus root").to_path_buf();
    for layout in homeless_layouts(&rig, &root, &source) {
        let name = format!("{} ({:?})", layout.name, layout.conversion);
        let (name, here) = (name.as_str(), layout.checkout.as_path());
        let setup = rig.run(here, &["setup"]);
        assert!(
            !both(&setup).contains(CODE),
            "{name}: `jigc setup` never raises the guard; got:\n{}",
            both(&setup),
        );

        let mint = rig.run(
            here,
            &["start", "--workflow", "record-decision", "record the cache"],
        );
        assert_ok(&mint, &format!("{name}: the doc-only mint"));
        assert!(
            !both(&mint).contains("checkout:") && !both(&mint).contains(CODE),
            "{name}: the mint states no code-only checkout — there is no other one; got:\n{}",
            both(&mint),
        );
        let task = minted(&text(&mint.stdout));

        assert_ok(
            &rig.run(
                here,
                &["doc", "create", "adr", "--title", "Cache", "--task", &task],
            ),
            &format!("{name}: `jigc doc create adr`"),
        );
        for slot in ["context", "decision", "consequences"] {
            assert_ok(
                &rig.run_stdin(
                    here,
                    &[
                        "doc",
                        "set-slot",
                        &format!("adr:cache#{slot}"),
                        "--from-file",
                        "-",
                        "--task",
                        &task,
                    ],
                    "Prose.\n",
                ),
                &format!("{name}: authoring adr:cache#{slot}"),
            );
        }
        rig.fill_commit(here, &task, "record the cache");

        let orientation = rig.run(here, &["start"]);
        assert!(
            !both(&orientation).contains("checkout:") && !both(&orientation).contains(CODE),
            "{name}: bare `jigc start` states no code-only checkout; got:\n{}",
            both(&orientation),
        );
        let listed = rig.run(here, &["doc", "list"]);
        assert!(
            !text(&listed.stderr).contains("served from"),
            "{name}: a store read prints no served-from note; got:\n{}",
            both(&listed),
        );

        let landed = rig.run(here, &["task", "finalize", &task]);
        assert_ok(&landed, &format!("{name}: `jigc task finalize`"));
        assert!(
            rig.git(here, &["show", "--format=", "--name-only", "HEAD"])
                .lines()
                .any(|path| path == "docs/decisions/cache.md"),
            "{name}: the created doc is in HEAD, as on rc.24",
        );
    }
}
