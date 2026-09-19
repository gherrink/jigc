//! **Compose any workflow through the real binary — whichever door composes it.**
//!
//! A sweep that asserts something about *every* workflow's composed bytes needs one
//! call that works for every member, and since M52 Increment 9 / T2 no single argv is
//! that call:
//!
//! * a workflow whose `suppressed:` block declares a `door:` is **verb-routed** — the
//!   two compose-by-name doors refuse it (`workflow.verb-routed`), because the door is
//!   what binds the input its body reads, and they bind none;
//! * a `creates-task: false` workflow mints nothing, so it has no preview — plain
//!   `jigc start --workflow <id>` composes it;
//! * everything else previews.
//!
//! Which case a member is in is read from **its own declaration**, and a verb-routed
//! member's argv is the **declared door itself**, so a sweep keeps its whole subject
//! rather than skipping the fourteen members whose bodies are exactly the ones a
//! migration or a fan-out agent reads. Narrowing a sweep to the doors that happen to
//! still answer would be the masking shape the fences exist to catch.
//!
//! **What is manufactured, and stated as such**: the door argv carries
//! `<placeholder>` operands a caller fills, and this module fills them from fixture
//! state it stands up on the corpus — a committed foreign file for `<path>`, a
//! milestone for `<milestone-id>`, a provisioned sub-task (and its worktree, which is
//! where a fanned sub-agent re-enters from) for `<task-id>`. Those three are the
//! shipped set, fenced at pack-load by `suppression_fence.rs`; a fourth placeholder
//! panics here rather than composing something else quietly.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use super::trial_corpus::TrialCorpus;

/// The fixture state a door's placeholders are filled from, stood up lazily: a door
/// that needs none costs nothing.
pub struct DoorFixtures<'a> {
    corpus: &'a TrialCorpus,
    /// The milestone id `<milestone-id>` resolves to, once created.
    milestone: Option<String>,
    /// The provisioned sub-task id `<task-id>` resolves to, and its worktree.
    sub_task: Option<(String, PathBuf)>,
    /// One committed foreign file per doctype, so `<path>` is a fresh subject each
    /// time and no two migrations contend for one source.
    foreign: BTreeMap<String, String>,
}

impl<'a> DoorFixtures<'a> {
    /// Bind the fixtures to a corpus. Nothing is written until a door needs it.
    pub fn new(corpus: &'a TrialCorpus) -> Self {
        Self {
            corpus,
            milestone: None,
            sub_task: None,
            foreign: BTreeMap::new(),
        }
    }

    /// Compose `workflow` through whichever door composes it, and return its stdout.
    pub fn compose(&mut self, workflow: &str, def: &engine::compose::WorkflowDef) -> String {
        match def.suppressed.as_ref().and_then(|s| s.door.as_deref()) {
            Some(door) => self.through_the_door(workflow, door),
            None if def.creates_task => self.corpus.jigc_ok(&["workflow", workflow, "--preview"]),
            None => self.corpus.jigc_ok(&["start", "--workflow", workflow]),
        }
    }

    /// Run a verb-routed workflow's **declared** door, its placeholders filled.
    fn through_the_door(&mut self, workflow: &str, door: &str) -> String {
        let argv = engine::compose::Suppressed::door_argv(door);
        let mut filled: Vec<String> = Vec::with_capacity(argv.len());
        // A `--as <doctype>` operand names the migration's target, which is what
        // decides the foreign source `<path>` stands for — read off the door rather
        // than guessed from the workflow id's `migrate-` prefix.
        let doctype = argv
            .iter()
            .position(|token| token == "--as")
            .and_then(|at| argv.get(at + 1))
            .cloned();
        let mut cwd = self.corpus.repo();
        for token in argv.iter().skip(1) {
            match token.as_str() {
                "<path>" => {
                    let ty = doctype.clone().unwrap_or_else(|| {
                        panic!("`{workflow}`'s door takes a <path> but names no `--as <doctype>`")
                    });
                    filled.push(self.foreign_source(&ty));
                }
                "<milestone-id>" => filled.push(self.milestone().to_owned()),
                "<task-id>" => {
                    let (sub, worktree) = self.sub_task();
                    cwd = worktree;
                    filled.push(sub);
                }
                other if other.starts_with('<') => panic!(
                    "`{workflow}`'s door carries the placeholder `{other}`, which this \
                     fixture set does not fill — add it here rather than skipping the \
                     member",
                ),
                other => filled.push(other.to_owned()),
            }
        }
        let args: Vec<&str> = filled.iter().map(String::as_str).collect();
        let out = self.run_in(&cwd, &args);
        assert!(
            out.status.success(),
            "`{door}` is `{workflow}`'s declared door and must compose it: jigc {args:?} \
             exited {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 jigc stdout")
    }

    /// A committed foreign file for `doctype` — one per doctype, minted on first ask.
    fn foreign_source(&mut self, doctype: &str) -> String {
        if let Some(path) = self.foreign.get(doctype) {
            return path.clone();
        }
        let rel = format!("foreign-{doctype}.md");
        std::fs::write(
            self.corpus.repo().join(&rel),
            format!("# Foreign {doctype}\n\n## Section\n\nSome prose.\n"),
        )
        .expect("write the foreign source");
        // Committed, not merely added: `jigc migrate` refuses a source git has never
        // recorded, and the retirement it promises needs a committed copy to point at.
        self.corpus.git(&["add", &rel]);
        self.corpus
            .git(&["commit", "-m", &format!("the foreign {doctype}")]);
        self.foreign.insert(doctype.to_owned(), rel.clone());
        rel
    }

    /// The milestone `<milestone-id>` resolves to.
    fn milestone(&mut self) -> &str {
        if self.milestone.is_none() {
            self.corpus
                .jigc_ok(&["milestone", "create", "Door fixture"]);
            self.milestone = Some("door-fixture".to_owned());
        }
        self.milestone.as_deref().expect("the milestone is created")
    }

    /// The provisioned sub-task `<task-id>` resolves to, and the worktree its door
    /// must run from — the main checkout's HEAD is ahead of the sub-task's base pin
    /// once the milestone record commits, which is by design.
    fn sub_task(&mut self) -> (String, PathBuf) {
        if self.sub_task.is_none() {
            let milestone = self.milestone().to_owned();
            self.corpus.jigc_ok(&[
                "milestone",
                "add-task",
                &milestone,
                "Fill the door placeholder",
                "--workflow",
                "sub-task",
            ]);
            self.corpus.jigc_ok(&["milestone", "provision", &milestone]);
            let sub = "fill-the-door-placeholder".to_owned();
            let worktree = self
                .corpus
                .repo()
                .join(".jigc")
                .join("worktrees")
                .join(&sub);
            self.sub_task = Some((sub, worktree));
        }
        self.sub_task.clone().expect("the sub-task is provisioned")
    }

    /// The corpus's own runner plus a working directory — the sub-task door is the
    /// one that must run somewhere other than the repo root.
    fn run_in(&self, cwd: &std::path::Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(cwd)
            .env("HOME", self.corpus.home())
            .env_remove("JIGC_PACK_DIR")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("spawn jigc")
    }
}
