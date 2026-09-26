//! **`jigc task amend` — the amend commit model, over the settle's refusal table as a set**
//! (F-10; `completions/artifacts/M53/f10-amend-settle.md`, shape **(D)**: *re-author, never
//! parse*).
//!
//! # The claim this suite is the acceptance of
//!
//! *An agent that landed a finalize with a wrong commit message repairs it through jigc,
//! authoring the new message as a commit doc jigc renders — never by typing
//! `<type>(<scope>): <summary>` itself — and the repair cannot fold unrelated staged work
//! into the rewritten commit.*
//!
//! # The set it iterates, and why the set is written down here
//!
//! [`AMEND_TABLE`] is the settle's **Refusals** table, row for row, each row carrying the
//! code it answers with, the exit it takes, which of the two `--format json` arms it rides,
//! and a closure that builds its own fixture and drives the door. The sweep is
//! `rows × {agent text, --format json}`.
//!
//! **It is a manufactured set and says so.** No code-side registry enumerates *the ways an
//! amend can be refused*: three of the rows are `GitState` members (a registry), one is a
//! shape of `HEAD` (a property of a commit), one is a property of the index, one of a hook's
//! exit code and one of a caller's typing. A registry that spanned them would be a registry
//! of *this feature's refusals*, which is this table — so the honest form is the table, said
//! to be manufactured, rather than a derivation dressed up as one
//! (`implementation/pinning.md` §4; the M49 flow-50 arm-1 precedent).
//!
//! What **is** read from the code rather than restated: each row's `--format json` arm is
//! cross-checked against [`cli::render::ENVELOPE_OWED_CODES`], so a row cannot *declare*
//! the flattened arm for a code the contract obliges to the findings envelope — the
//! classification is the binary's, and the row only says which one it expects to see.
//!
//! # And the refusals act on nothing
//!
//! Every refusing row asserts, after both surfaces have answered, that `HEAD` is the commit
//! it was and that `HEAD`'s **tree** is the tree it was. The driven hazard the arm exists to
//! close is `git commit --amend` folding the index in silently at exit 0, so a cell that
//! only checked the exit code would pass over exactly the damage.

use std::path::Path;
use std::process::Output;

use crate::support::git_state::{self, GitState, GitStateRepo};
use crate::support::trial_corpus::{State, TrialCorpus};

use cli::render::ENVELOPE_OWED_CODES;
use serde_json::Value;

// ───────────────────────────── the table ─────────────────────────────

/// Which `--format json` arm a row's refusal rides.
///
/// The two are the shapes `design/command-output-contract.md` → *The two reject arms*
/// declares, and which one a door takes is not this suite's choice: a code the contract
/// lists under a target form is obliged to the envelope through
/// [`ENVELOPE_OWED_CODES`], and every non-member is flattened. The row states its
/// expectation and [`arm_matches_the_registry`] checks the expectation against that set.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Arm {
    /// The findings envelope — `{"findings": [{…, "key": {"code", "target"}}]}`.
    Envelope,
    /// The flattened `{"error": "<the whole finding rendering>"}`.
    Flattened,
}

/// The door a row is adjudicated at — the settle's two.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Door {
    /// `jigc task amend` — the mint door. `BEHALF_DOORS`' `Neither`: it commits nothing.
    Mint,
    /// `jigc task finalize <id>` over an amend task — the second commit model.
    Finalize,
}

impl Door {
    fn label(self) -> &'static str {
        match self {
            Door::Mint => "jigc task amend",
            Door::Finalize => "jigc task finalize (amend arm)",
        }
    }
}

/// Where a row's **identity** is readable on the agent-text surface.
///
/// The distinction is shipped, not incidental, and a sweep that assumed the first shape
/// everywhere would report the second as a defect.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TextIdentity {
    /// The house finding line — `blocking · <code> — <message>`, then the locus and route.
    Named,
    /// The **survivable hook-rejection frame**: git's own stderr verbatim, this door's
    /// state-truth clause, and the exact line to re-run. The frame deliberately prints no
    /// dotted code — a reader whose commit a hook just refused needs the hook's complaint
    /// and the way back, not jigc's log vocabulary — so the identity is readable on the
    /// `--format json` finding and in the invocation log, which is where
    /// `crates/cli/tests/commit_rejected_axis.rs` reads it per `COMMITTING_DOORS` row
    /// (`design/surface-contract.md` → The mirror is enforced: that log read is load-bearing
    /// because `Outcome::error`'s membership check is a `debug_assert!`).
    Framed,
}

/// One row of the settle's refusal table.
struct Row {
    /// The settle row, as its table names it.
    id: &'static str,
    door: Door,
    /// The finding code the row's answer carries.
    code: &'static str,
    /// The exit the row's answer takes.
    exit: i32,
    arm: Arm,
    /// Where the code is readable on the text surface.
    text: TextIdentity,
    /// A fragment **only this row's producer emits** — the discriminator, and the field this
    /// table would be worthless without.
    ///
    /// Driven while writing this suite: with the amend arm's moved-`HEAD` guard disabled the
    /// `finalize.base-mismatch` row went **green**, because the engine's *ordinary* base-pin
    /// constructor answers on the same code at the same exit — so the row proved the code
    /// existed and nothing about which producer raised it, the masking shape this repo's
    /// dev-workflow names. Every row now carries the words its own producer chose, so a
    /// sibling constructor cannot satisfy it.
    says: &'static str,
    /// Build this row's fixture and drive its door on **both** surfaces, then assert the
    /// refusal moved nothing. One fixture per row rather than one per surface: a refusal
    /// that mutates nothing can be asked twice, and a row whose first surface *did* mutate
    /// reddens on its own no-movement assertion.
    answer: fn() -> Answered,
}

/// What one row's door said, on both surfaces.
struct Answered {
    text: Driven,
    json: Driven,
}

/// One invocation's exit and streams, as a refusal is judged on them.
struct Driven {
    exit: i32,
    stdout: String,
    stderr: String,
}

impl Driven {
    /// Both streams, for a `contains` assertion: a refusal's rendering rides **stderr** and
    /// its envelope rides **stdout**, and which one a row uses is not what these rows are
    /// about (`design/command-output-contract.md` → Stream discipline has its own suite).
    fn both(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }
}

/// **The settle's Refusals table.** Order is the settle's.
const AMEND_TABLE: &[Row] = &[
    // | repository posture (any `InProgress` member, detached, unborn) | transfers unchanged
    // at both doors | — driven at the door that acts: `jigc task amend` is `Neither` and
    // mints under every posture exactly as `jigc start` does, while the finalize arm is
    // commit-on-behalf and refuses. Four `GitState` members, chosen to span the family's
    // three detection shapes (a marker file · a marker directory · a detached HEAD).
    //
    // **`posture_door_axis` does not cover this arm, and is not owed a cell** (corrected at
    // the F-10 review's LOW-6, which found this comment claiming it *"now includes this arm
    // through `BEHALF_DOORS`"* — false: that suite filters the registry to acting rows,
    // `task amend` is `Neither`, and its `task finalize` row is driven over an **ordinary**
    // task). The reason no second sweep is owed is that the posture family's subject is the
    // **door**, never the task: `cli::cli::finalize_posture_refusal` takes a cwd and a format
    // and cannot see which commit model a task carries, and the pre-commit re-probe is
    // `SeamSubject::verify` inside `git_commit_capture`, which `git_commit_amend` funnels
    // through like every other commit site. So all seventeen states × this door are already
    // swept there, once, for both arms — and these four rows drive the arm's own claim, that
    // a refusal leaves `HEAD` *and its tree* where they were.
    Row {
        id: "posture / merge",
        door: Door::Finalize,
        code: "repo.operation-in-progress",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "a merge is in progress",
        answer: || posture_at_finalize(GitState::Merge),
    },
    Row {
        id: "posture / bisect",
        door: Door::Finalize,
        code: "repo.operation-in-progress",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "a bisect is in progress",
        answer: || posture_at_finalize(GitState::Bisect),
    },
    Row {
        id: "posture / uncommitted-pick",
        door: Door::Finalize,
        code: "repo.operation-in-progress",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "an uncommitted cherry-pick is in progress",
        answer: || posture_at_finalize(GitState::UncommittedPick),
    },
    Row {
        id: "posture / detached",
        door: Door::Finalize,
        code: "repo.head-detached",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "HEAD is detached",
        answer: || posture_at_finalize(GitState::Detached),
    },
    // | HEAD is a **root** commit or a **merge** commit | refuse at `task amend`:
    // `amend.head-shape` | — plus the unborn cell, which the shipped guard answers on the
    // same code because `--amend` needs a commit to rewrite before it needs a single parent.
    Row {
        id: "amend.head-shape / root",
        door: Door::Mint,
        code: "amend.head-shape",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "a root commit — it has no parent",
        answer: root_head_at_mint,
    },
    Row {
        id: "amend.head-shape / merge",
        door: Door::Mint,
        code: "amend.head-shape",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "a merge commit — it has 2 parents",
        answer: merge_head_at_mint,
    },
    Row {
        id: "amend.head-shape / unborn",
        door: Door::Mint,
        code: "amend.head-shape",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "this repository has no commit yet",
        answer: unborn_head_at_mint,
    },
    // | **a non-empty index at finalize** | refuse, blocking: `finalize.amend-index-dirty` |
    // — the data-loss cell. Its own test below asserts the tree of the commit it was about
    // to rewrite, and that `jigc task validate` previews it.
    Row {
        id: "finalize.amend-index-dirty",
        door: Door::Finalize,
        code: "finalize.amend-index-dirty",
        exit: 3,
        arm: Arm::Envelope,
        text: TextIdentity::Named,
        says: "an amend rewrites `HEAD` from the index",
        answer: dirty_index_at_finalize,
    },
    // | **HEAD moved between `amend` and `finalize`** | refuse: `finalize.base-mismatch` |
    Row {
        id: "finalize.base-mismatch",
        door: Door::Finalize,
        code: "finalize.base-mismatch",
        exit: 3,
        arm: Arm::Envelope,
        text: TextIdentity::Named,
        says: "no longer the commit this amend was minted against",
        answer: moved_head_at_finalize,
    },
    // The survivable hook-rejection frame's own identity for this arm. `HEAD` byte-identical
    // is the frame's state-truth clause here, and the no-movement assertion every row makes
    // is exactly that claim — which is why this arm has the cheapest clause in the registry.
    Row {
        id: "finalize.amend-rejected",
        door: Door::Finalize,
        code: "finalize.amend-rejected",
        exit: 1,
        arm: Arm::Envelope,
        text: TextIdentity::Framed,
        says: "`HEAD` is unchanged",
        answer: rejecting_hook_at_finalize,
    },
    // | a promoted doc in HEAD's tree | — the settle called this row *"message-only by
    // construction"*, and the F-10 review's HIGH-1 drove it false for a doc **this task
    // stages**: the promote phase runs on every arm, so an amend's finalize wrote the doc
    // into the worktree, committed none of it, and baselined file-state to the bytes no
    // commit carries. The arm now refuses instead — and the fixture plants the staged doc
    // directly, because the live route to it (the copy-on-first-touch seam) is closed by
    // the write-door half of the same fix, which is the point of having both.
    Row {
        id: "finalize.amend-staged-doc",
        door: Door::Finalize,
        code: "finalize.amend-staged-doc",
        exit: 3,
        arm: Arm::Envelope,
        text: TextIdentity::Named,
        says: "this task's commit model is an amend, which changes no tree",
        answer: staged_managed_doc_at_finalize,
    },
    // Not a settle row, and here because the settle's shape (D) hands this door a *prose*
    // intent: the id slugs from what the caller typed, so a title that slugs to nothing has
    // to refuse rather than fall back to `amend-<sha7>` and name the task after something
    // else (the degenerate-title axis; `3c4f4c7a`'s third fix).
    Row {
        id: "write.unslugable-title",
        door: Door::Mint,
        code: "write.unslugable-title",
        exit: 1,
        arm: Arm::Flattened,
        text: TextIdentity::Named,
        says: "this title slugs to nothing",
        answer: unslugable_intent_at_mint,
    },
];

// ───────────────────────────── the sweep ─────────────────────────────

/// **The fence.** Every row of the settle's refusal table answers with its own code, at its
/// own exit, on both surfaces — and moves nothing.
#[test]
fn every_refusal_row_answers_with_its_code_on_both_surfaces() {
    for row in AMEND_TABLE {
        let door = row.door.label();
        let answered = (row.answer)();
        for (label, driven) in [
            ("agent text", &answered.text),
            ("--format json", &answered.json),
        ] {
            assert_eq!(
                driven.exit, row.exit,
                "[{}] `{door}` on {label} must exit {}; got {}\n--- stdout ---\n{}\n\
                 --- stderr ---\n{}",
                row.id, row.exit, driven.exit, driven.stdout, driven.stderr,
            );
        }
        // The identity: always on the `--format json` arm, and on the text arm only where
        // the row's rendering is the house finding line rather than the survivable frame.
        assert!(
            answered.json.both().contains(row.code),
            "[{}] `{door}` on --format json must name `{}`\n--- stdout ---\n{}\n\
             --- stderr ---\n{}",
            row.id,
            row.code,
            answered.json.stdout,
            answered.json.stderr,
        );
        // The discriminator: the words this row's own producer chose, on both surfaces.
        // Without it a row keyed on `(code, exit)` alone is satisfied by any sibling
        // constructor of the same code — driven, and the reason this field exists.
        let printed = answered.text.both();
        for (label, said) in [
            ("agent text", &printed),
            ("--format json", &answered.json.both()),
        ] {
            assert!(
                said.contains(row.says),
                "[{}] `{door}` on {label} must carry this row's OWN producer's words \
                 (`{}`) — the code alone does not tell one constructor from another:\n{said}",
                row.id,
                row.says,
            );
        }
        match row.text {
            TextIdentity::Named => assert!(
                printed.contains(row.code),
                "[{}] `{door}` on agent text must name `{}`:\n{printed}",
                row.id,
                row.code,
            ),
            TextIdentity::Framed => assert!(
                !printed.contains(row.code),
                "[{}] `{door}`'s text surface is the survivable frame, which states the \
                 hook's complaint and the way back and leaves the log vocabulary to the log \
                 — a dotted code appearing here means the frame stopped being the rendering \
                 and this row's disposition needs re-deciding, not relaxing:\n{printed}",
                row.id,
            ),
        }
        assert_arm(row, &answered.json);
    }
}

/// **A mint-door refusal takes the operational exit, never the gate's.** `jigc task amend`
/// adjudicates before anything is authored, so there is no validation report for it to gate
/// — exit **3** from that door would promise a driver a report it never emitted. The gate's
/// exit belongs to the finalize arm, where a report exists.
#[test]
fn a_mint_door_refusal_takes_the_operational_exit() {
    for row in AMEND_TABLE {
        if row.door == Door::Mint {
            assert_eq!(
                row.exit,
                1,
                "[{}] `{}` refuses before it mints, so it has no report to gate on",
                row.id,
                row.door.label(),
            );
        }
    }
}

/// The `--format json` arm a row declares must be the one the code's own obligation gives
/// it: a member of [`ENVELOPE_OWED_CODES`] rides the findings envelope wherever it is
/// raised, and every non-member is flattened. So a row cannot declare its way out of the
/// contract's `(code, target)` promise, and this suite cannot pin a flattening the contract
/// forbids.
#[test]
fn arm_matches_the_registry() {
    for row in AMEND_TABLE {
        if ENVELOPE_OWED_CODES.contains(&row.code) {
            assert_eq!(
                row.arm,
                Arm::Envelope,
                "[{}] `{}` is an `ENVELOPE_OWED_CODES` member, so its refusal is obliged to \
                 the findings envelope whichever constructor the door reached for",
                row.id,
                row.code,
            );
        }
    }
}

/// The one JSON document a refusal emitted, from whichever stream carries it.
///
/// **Which stream is the refusal's kind, not the row's**, and the suite asks rather than
/// assumes: a *blocked* door renders its validation report on **stdout** (it is the verb's
/// answer), while a *reject* — the door acted and git refused — rides **stderr**, the
/// envelope beside the agent-text frame (`design/command-output-contract.md` → Stream
/// discipline, whose own suite pins the rule). What this asserts is that exactly one of the
/// two carries exactly one document.
fn one_document(row: &Row, json: &Driven) -> Value {
    let stream = if json.stdout.trim().is_empty() {
        &json.stderr
    } else {
        &json.stdout
    };
    serde_json::from_str(stream.trim()).unwrap_or_else(|err| {
        panic!(
            "[{}] the `--format json` arm must emit exactly one json document ({err})\n\
             --- stdout ---\n{}\n--- stderr ---\n{}",
            row.id, json.stdout, json.stderr,
        )
    })
}

/// Assert a row's `--format json` shape, per its declared arm.
fn assert_arm(row: &Row, json: &Driven) {
    let doc = one_document(row, json);
    match row.arm {
        Arm::Envelope => {
            let findings = doc["findings"].as_array().unwrap_or_else(|| {
                panic!("[{}] the envelope carries `findings`:\n{doc:#}", row.id)
            });
            assert!(
                findings.iter().any(|finding| {
                    finding["key"]["code"] == row.code && !finding["key"]["target"].is_null()
                }),
                "[{}] one finding must carry the row's code on a NON-NULL `(code, target)` \
                 key — a key that cannot discriminate is what the contract's sweep closed:\n{doc:#}",
                row.id,
            );
        }
        Arm::Flattened => {
            let error = doc["error"].as_str().unwrap_or_else(|| {
                panic!("[{}] the flattened arm carries `error`:\n{doc:#}", row.id)
            });
            assert!(
                error.contains(row.code),
                "[{}] the flattened `error` string must carry the code and its route — that \
                 is the whole of what a driver gets here:\n{error}",
                row.id,
            );
        }
    }
}

// ───────────────────────── the rows' fixtures ─────────────────────────

/// `jigc` against a corpus, on both surfaces, over one fixture.
fn both_formats(run: impl Fn(&[&str]) -> Output) -> Answered {
    Answered {
        text: driven(run(&[])),
        json: driven(run(&["--format", "json"])),
    }
}

fn driven(out: Output) -> Driven {
    Driven {
        exit: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// The commit at `HEAD` and the tree it carries — the pair every refusing row asserts is
/// unmoved. The **tree** is the load-bearing half: the hazard is `git commit --amend`
/// folding the index in, which moves the tree at exit 0.
fn head_of(corpus: &TrialCorpus) -> (String, String) {
    (
        corpus.git(&["rev-parse", "HEAD"]),
        corpus.git(&["rev-parse", "HEAD^{tree}"]),
    )
}

fn assert_unmoved(corpus: &TrialCorpus, before: &(String, String), row: &str) {
    let after = head_of(corpus);
    assert_eq!(
        &after, before,
        "[{row}] the refusal must leave `HEAD` and its TREE exactly as they were",
    );
}

/// Mint an amend task and return the id **the binary printed**, read off the composed
/// `--format json` envelope rather than re-slugged in test code.
fn mint_amend(corpus: &TrialCorpus, intent: &str) -> String {
    let stdout = corpus.jigc_ok(&["task", "amend", intent, "--format", "json"]);
    let doc: Value = serde_json::from_str(&stdout).expect("the compose envelope is one document");
    doc["task"]
        .as_str()
        .expect("the composed envelope names the minted task")
        .to_owned()
}

/// Author the provisioned `commit` doc to the point its finalize reaches the commit phase.
fn author_message(corpus: &TrialCorpus, task: &str, summary: &str) {
    corpus.set_field(&format!("commit:{task}#type"), task, "chore");
    corpus.set_slot(&format!("commit:{task}#summary"), task, summary);
}

/// A fresh corpus with an amend task minted and its message authored — the shape four rows
/// and every control below share.
fn amend_ready(intent: &str, summary: &str) -> (TrialCorpus, String) {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = mint_amend(&corpus, intent);
    author_message(&corpus, &task, summary);
    (corpus, task)
}

/// **Posture at the finalize arm.** The amend is minted and authored *first*, on a clean
/// repository — the posture is entered last, which is the overlay's own rule (it refuses a
/// dirty worktree, and a door that has stopped answering cannot be built against).
///
/// The mint is asserted to have **succeeded** under no posture and then the state entered:
/// `jigc task amend` is `BEHALF_DOORS`' `Neither`, so it adjudicates no member of the family
/// — the settle's *"transfers unchanged at both doors"*, which for a mint door means the
/// shipped rule that a non-acting door does not refuse a repository posture.
fn posture_at_finalize(state: GitState) -> Answered {
    let (corpus, task) = amend_ready("repair the install message", "the authored subject");
    git_state::overlay(&corpus, state).expect("the posture overlay enters last, over a clean tree");
    let before = head_of(&corpus);
    let answered = both_formats(|format| {
        let mut argv = vec!["task", "finalize", task.as_str()];
        argv.extend_from_slice(format);
        corpus.jigc(&argv)
    });
    assert_unmoved(&corpus, &before, state.name());
    // The refusal concluded nothing: the markers git wrote are still the ones it wrote.
    git_state::assert_state(&corpus.repo(), &corpus.home(), state);
    answered
}

/// **A root commit at the mint door.** `jigc setup` *births* `HEAD` on an unborn
/// repository, so a repo that has never been committed to and is then set up has exactly
/// one commit — and that commit is the root. It is refused although `git commit --amend`
/// would take it: the route hands the act back with the command that performs it.
fn root_head_at_mint() -> Answered {
    let fixture = GitStateRepo::build(GitState::Unborn);
    let repo = fixture.repo();
    let home = fixture.home();
    let setup = run_jigc(&repo, &home, &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` births HEAD on an unborn repository:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    assert_eq!(
        git(&repo, &["rev-list", "--count", "HEAD"]),
        "1",
        "the fixture's whole point is that HEAD is the root commit",
    );
    both_formats(|format| {
        let mut argv = vec!["task", "amend", "repair the first commit"];
        argv.extend_from_slice(format);
        run_jigc(&repo, &home, &argv)
    })
}

/// **A merge commit at the mint door.** Built by driving a real `git merge --no-ff`, so the
/// two-parent `HEAD` is git's, not a fixture's belief about one.
fn merge_head_at_mint() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    let trunk = corpus.git(&["rev-parse", "--abbrev-ref", "HEAD"]);
    corpus.git(&["checkout", "-b", "side"]);
    std::fs::write(corpus.repo().join("side.txt"), "side\n").expect("write the side file");
    corpus.git(&["add", "side.txt"]);
    corpus.git(&["commit", "--no-verify", "-m", "the side commit"]);
    corpus.git(&["checkout", &trunk]);
    std::fs::write(corpus.repo().join("trunk.txt"), "trunk\n").expect("write the trunk file");
    corpus.git(&["add", "trunk.txt"]);
    corpus.git(&["commit", "--no-verify", "-m", "the trunk commit"]);
    corpus.git(&[
        "merge",
        "--no-ff",
        "--no-verify",
        "-m",
        "a merge commit",
        "side",
    ]);
    assert_eq!(
        corpus
            .git(&["rev-list", "--parents", "-1", "HEAD"])
            .split_whitespace()
            .count(),
        3,
        "the fixture's whole point is a two-parent HEAD (the sha plus its two parents)",
    );
    let before = head_of(&corpus);
    let answered = both_formats(|format| {
        let mut argv = vec!["task", "amend", "repair the merge message"];
        argv.extend_from_slice(format);
        corpus.jigc(&argv)
    });
    assert_unmoved(&corpus, &before, "amend.head-shape / merge");
    assert!(
        !corpus
            .repo()
            .join(".jigc/tasks")
            .join("repair-the-merge-message")
            .exists(),
        "the shape guard is adjudicated BEFORE anything mints — a refused amend strands no \
         task directory",
    );
    answered
}

/// **An unborn `HEAD` at the mint door.** The only shape it can take is a repository that
/// has never been set up, because setup itself births `HEAD` — so the fixture carries a
/// hand-made project cascade layer and no install.
fn unborn_head_at_mint() -> Answered {
    let fixture = GitStateRepo::build(GitState::Unborn);
    let repo = fixture.repo();
    let home = fixture.home();
    both_formats(|format| {
        let mut argv = vec!["task", "amend", "repair a commit that does not exist"];
        argv.extend_from_slice(format);
        run_jigc(&repo, &home, &argv)
    })
}

/// **A non-empty index at the finalize arm** — the data-loss cell. Driven on the baseline:
/// `git commit --amend` folds the whole index into the rewritten commit, silently, at exit 0.
fn dirty_index_at_finalize() -> Answered {
    let (corpus, task) = amend_ready("repair the install message", "the authored subject");
    std::fs::write(
        corpus.repo().join("unrelated.txt"),
        "not this commit's work\n",
    )
    .expect("plant the unrelated file");
    corpus.git(&["add", "unrelated.txt"]);
    let before = head_of(&corpus);
    let answered = both_formats(|format| {
        let mut argv = vec!["task", "finalize", task.as_str()];
        argv.extend_from_slice(format);
        corpus.jigc(&argv)
    });
    assert_unmoved(&corpus, &before, "finalize.amend-index-dirty");
    assert!(
        answered.text.both().contains("unrelated.txt"),
        "the refusal names the staged path it is about — one finding per path, keyed at the \
         path:\n{}",
        answered.text.both(),
    );
    answered
}

/// **`HEAD` moved between the mint and the finalize.** An ordinary task's base is the
/// history its work sits *on*; an amend's pin is the commit it *rewrites*, so once `HEAD`
/// has moved the message would land on a different commit than the one whose subject line
/// the agent read.
fn moved_head_at_finalize() -> Answered {
    let (corpus, task) = amend_ready("repair the install message", "the authored subject");
    std::fs::write(
        corpus.repo().join("later.txt"),
        "a commit that landed after\n",
    )
    .expect("write the later file");
    corpus.git(&["add", "later.txt"]);
    corpus.git(&["commit", "--no-verify", "-m", "an unrelated commit"]);
    let before = head_of(&corpus);
    let answered = both_formats(|format| {
        let mut argv = vec!["task", "finalize", task.as_str()];
        argv.extend_from_slice(format);
        corpus.jigc(&argv)
    });
    assert_unmoved(&corpus, &before, "finalize.base-mismatch");
    answered
}

/// **A rejecting `pre-commit` hook at the finalize arm.** The frame's state-truth clause
/// here is *`HEAD` is unchanged*, and it is the cheapest in the registry because
/// `git commit --amend` is atomic with respect to `HEAD`: nothing was promoted, nothing was
/// staged, and the commit is byte-identical after the refusal.
fn rejecting_hook_at_finalize() -> Answered {
    let (corpus, task) = amend_ready("repair the install message", "the authored subject");
    install_rejecting_hook(&corpus.repo());
    let before = head_of(&corpus);
    let answered = both_formats(|format| {
        let mut argv = vec!["task", "finalize", task.as_str()];
        argv.extend_from_slice(format);
        corpus.jigc(&argv)
    });
    assert_unmoved(&corpus, &before, "finalize.amend-rejected");
    let text = answered.text.both();
    assert!(
        text.contains("`HEAD` is unchanged"),
        "the frame states THIS arm's truth, not the ordinary arm's:\n{text}",
    );
    assert!(
        text.contains(&format!("jigc task finalize {task}")),
        "the frame carries the re-run line, verbatim:\n{text}",
    );
    answered
}

/// **A managed doc in the amend task's staged set, at the finalize arm** — the F-10
/// review's HIGH-1, on its backstop path.
///
/// The doc is **planted** into the working area rather than written through a verb, and the
/// plant is the honest fixture rather than a shortcut: the write door now refuses the
/// copy-on-first-touch that used to put it there ([`DOC_WRITE_LEAVES`] sweeps that half), so
/// the only way to reach the finalize gate is to arrive at the area by some other path —
/// which is exactly the case the finalize gate exists for. The planted name is a **member
/// shape** (`<type>:<slug>.md` is what `engine::state::staged_doc_id` admits), so this is not
/// the foreign-bytes cell and no destroying-door guard answers first.
fn staged_managed_doc_at_finalize() -> Answered {
    let corpus = TrialCorpus::build(State::Vendored);
    let task = mint_amend(&corpus, "repair the arch-doc commit message");
    author_message(&corpus, &task, "the authored subject");
    plant_staged_doc(&corpus, &task);
    let before = head_of(&corpus);
    let answered = both_formats(|format| {
        let mut argv = vec!["task", "finalize", task.as_str()];
        argv.extend_from_slice(format);
        corpus.jigc(&argv)
    });
    assert_unmoved(&corpus, &before, "finalize.amend-staged-doc");
    assert!(
        answered.text.both().contains(SPEC_HOME),
        "the refusal names the canonical home that would have been left diverged:\n{}",
        answered.text.both(),
    );
    // **Never made, not rolled back.** The gate sits ahead of `plan_finalize`, so the
    // promote phase is not reached at all — the distinction matters, because the shipped
    // rejected-amend path *does* roll a promotion back and a test satisfied by a clean
    // worktree alone could not tell the two apart.
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        "",
        "the refused finalize wrote nothing into the worktree",
    );
    answered
}

/// **A title that slugs to nothing.** The id is slugged from the caller's own words, so a
/// title carrying no ASCII letter or digit cannot name a task — and must not quietly become
/// `amend-<sha7>`, which would name it after something the caller never typed.
fn unslugable_intent_at_mint() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    let before = head_of(&corpus);
    let answered = both_formats(|format| {
        let mut argv = vec!["task", "amend", "日本語のみ"];
        argv.extend_from_slice(format);
        corpus.jigc(&argv)
    });
    assert_unmoved(&corpus, &before, "write.unslugable-title");
    answered
}

// ───────────────────────── the happy path ─────────────────────────

/// **The claim's positive half.** The amend lands: a new sha, the superseded one on
/// `committed.amended`, the tree byte-identical, the message **rendered from the doc** —
/// trailer item included — and the working area gone.
#[test]
fn the_amend_lands_a_new_message_over_an_untouched_tree() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = mint_amend(&corpus, "repair the install commit message");
    let (superseded, tree) = head_of(&corpus);
    let short = corpus.git(&["rev-parse", "--short", &superseded]);

    author_message(&corpus, &task, "install the jigc workspace configuration");
    corpus.set_field(&format!("commit:{task}#scope"), &task, "jigc");
    // A trailer item, so the landed message is proven to be the DOC's render and not git's
    // own `--amend --no-edit` carry-over of the message that was there.
    let item = corpus.add_item(&format!("commit:{task}#trailers"), "Refs", &task);
    corpus.set_field(&format!("{item}/value"), &task, "F-10");

    let landed = corpus.jigc_ok(&["task", "finalize", &task, "--format", "json"]);
    let doc: Value = serde_json::from_str(&landed).expect("the landed envelope is one document");
    let committed = &doc["committed"];

    let (new_sha, new_tree) = head_of(&corpus);
    assert_ne!(
        new_sha, superseded,
        "`git commit --amend` mints a new sha every time — driven even with nothing staged",
    );
    assert_eq!(
        new_tree, tree,
        "the arm's whole contract: the committed TREE does not move",
    );
    assert_eq!(
        committed["amended"].as_str(),
        Some(short.as_str()),
        "`committed.amended` is the superseded commit, abbreviated to the same length \
         `hash` carries — two shas in one object, one spelling:\n{doc:#}",
    );
    assert_eq!(
        committed["hash"].as_str(),
        Some(corpus.git(&["rev-parse", "--short", "HEAD"]).as_str()),
        "`committed.hash` is the commit that now stands:\n{doc:#}",
    );

    let message = corpus.git(&["log", "-1", "--pretty=format:%B"]);
    assert!(
        message.starts_with("chore(jigc): install the jigc workspace configuration"),
        "the subject is the DOC's render — `<type>(<scope>): <summary>`, composed by the \
         CLI and never typed by the agent:\n{message}",
    );
    assert!(
        message.contains("Refs: F-10"),
        "the trailer item the doc carries reaches the message:\n{message}",
    );

    // The area goes, as every task's does — so a second repair is a fresh mint against the
    // commit that now stands, never a resumption of this one.
    assert!(
        !corpus.repo().join(".jigc/tasks").join(&task).exists(),
        "the amend task's working area is torn down at its finalize",
    );
    let list = corpus.jigc_ok(&["task", "list"]);
    assert!(
        list.contains("no active tasks"),
        "nothing of the amend task survives its landing:\n{list}",
    );
}

/// **Amend twice.** Each amend is a fresh mint against the `HEAD` that now stands, and the
/// commit count never moves: the second rewrite replaces the first's message, it does not
/// stack a commit on it.
#[test]
fn amending_twice_rewrites_the_new_head_and_adds_no_commit() {
    let corpus = TrialCorpus::build(State::Fresh);
    let commits = corpus.git(&["rev-list", "--count", "HEAD"]);

    let first = mint_amend(&corpus, "the first repair");
    author_message(&corpus, &first, "the first re-authoring");
    corpus.jigc_ok(&["task", "finalize", &first]);
    let after_first = corpus.git(&["rev-parse", "HEAD"]);

    let second = mint_amend(&corpus, "the second repair");
    assert_ne!(first, second, "each amend is its own task");
    author_message(&corpus, &second, "the second re-authoring");
    let landed = corpus.jigc_ok(&["task", "finalize", &second, "--format", "json"]);
    let doc: Value = serde_json::from_str(&landed).expect("one document");

    assert_eq!(
        doc["committed"]["amended"].as_str(),
        Some(corpus.git(&["rev-parse", "--short", &after_first]).as_str()),
        "the second amend pins the commit the FIRST one left, not the one before it:\n{doc:#}",
    );
    assert_eq!(
        corpus.git(&["log", "-1", "--pretty=format:%s"]),
        "chore: the second re-authoring",
    );
    assert_eq!(
        corpus.git(&["rev-list", "--count", "HEAD"]),
        commits,
        "a rewrite is not an addition",
    );
}

// ───────────────────────────── the controls ─────────────────────────────

/// **The promoted-doc control.** A managed doc committed in `HEAD`'s tree stays valid across
/// an amend, because the tree is untouched and jigc's file-state baseline is
/// **content**-keyed: a message-only rewrite is genuinely invisible to it.
///
/// This is the control for the settle's *"content amend is not built"* row: the reason a
/// message-only amend is safe over a promoted doc is exactly the reason a *content* amend
/// would not be, so the row is a fact about this arm and not a convenience.
#[test]
fn an_amend_over_a_promoted_doc_leaves_the_store_valid() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let baseline = std::fs::read_to_string(corpus.repo().join(".jigc/state/file-state.json"))
        .expect("the corpus has a file-state baseline");
    let before_validate = corpus.jigc(&["validate"]);
    let tree = corpus.git(&["rev-parse", "HEAD^{tree}"]);

    let task = mint_amend(&corpus, "repair the release message");
    author_message(&corpus, &task, "record the decision properly");
    corpus.jigc_ok(&["task", "finalize", &task]);

    assert_eq!(
        corpus.git(&["rev-parse", "HEAD^{tree}"]),
        tree,
        "the committed tree — every promoted doc in it — is byte-identical",
    );
    assert_eq!(
        std::fs::read_to_string(corpus.repo().join(".jigc/state/file-state.json")).ok(),
        Some(baseline),
        "the content-keyed baseline cannot see a message-only rewrite, so it does not move",
    );
    let after_validate = corpus.jigc(&["validate"]);
    assert_eq!(
        after_validate.status.code(),
        before_validate.status.code(),
        "the store's verdict is the verdict it had:\n{}",
        String::from_utf8_lossy(&after_validate.stderr),
    );
    assert_eq!(
        String::from_utf8_lossy(&after_validate.stdout),
        String::from_utf8_lossy(&before_validate.stdout),
        "and it says the same thing about the same store",
    );
}

/// **The fan-out-worktree control.** A sub-task worktree's `HEAD` is detached at the
/// milestone base, and the commit arm refuses a detached `HEAD` — so the amend model cannot
/// land a rewrite into a commit that would belong to no branch. Driven in a **real linked
/// worktree**, because that is the shape the fan-out produces and `git switch --detach` in
/// the main checkout is a different repository layout (the `posture / detached` row above
/// covers that one).
#[test]
fn an_amend_inside_a_detached_linked_worktree_refuses_at_the_commit_arm() {
    let corpus = TrialCorpus::build(State::Fresh);
    let worktree = corpus.repo().join(".jigc/worktrees/probe");
    corpus.git(&[
        "worktree",
        "add",
        "--detach",
        worktree.to_str().expect("utf-8 worktree path"),
        "HEAD",
    ]);
    let head = git(&worktree, &["rev-parse", "HEAD"]);
    assert_eq!(
        git(&worktree, &["symbolic-ref", "--quiet", "HEAD"]),
        "",
        "the fan-out's worktree HEAD is detached",
    );

    // The mint is `Neither`, so it composes here — and the pin it writes is the HEAD the
    // caller is standing on (the C2-09 rule: `task finalize` commits where you stand).
    let minted = run_jigc_in(
        &worktree,
        &corpus.home(),
        &["task", "amend", "repair it here", "--format", "json"],
    );
    assert!(
        minted.status.success(),
        "the mint door commits nothing, so it adjudicates no posture:\n{}",
        String::from_utf8_lossy(&minted.stderr),
    );
    let composed: Value = serde_json::from_slice(&minted.stdout).expect("one document");
    let task = composed["task"]
        .as_str()
        .expect("the envelope names the task");

    // **Nothing is authored, deliberately.** The posture family is asked *at the door*,
    // before the task's content is read at all (`cli::gate_coverage::Invocation`), so this
    // cell reaches the refusal it is about without first driving a slot write through a
    // second working directory — and a cell that had to author first could not tell a
    // posture refusal from a conformance one.
    let refused = run_jigc_in(&worktree, &corpus.home(), &["task", "finalize", task]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert_eq!(refused.status.code(), Some(1), "the posture exit:\n{text}");
    assert!(
        text.contains("repo.head-detached"),
        "the commit arm refuses a detached HEAD:\n{text}",
    );
    assert_eq!(
        git(&worktree, &["rev-parse", "HEAD"]),
        head,
        "and it rewrote nothing",
    );
}

/// **The mint ack names the checkout whose `HEAD` it pinned, when that is not the
/// workbench's** (the F-10 review's LOW-7 — the settle row driven *not real*).
///
/// From an **attached** linked worktree the amend pins that worktree's `HEAD` (the C2-09 rule:
/// `task finalize` commits where you stand) while the task roster, the `.jigc/` workbench and
/// every door the reader reached this one through belong to the main checkout. Shipped, the
/// mint ack said nothing: the `committed / would commit in the linked worktree at …` line
/// existed only on the forecast and the landed ack, after the fact.
///
/// The control is the ordinary cell: from the main checkout the sentence is always true and
/// never news, so it renders no bytes — the omitting-context rule every line beside it obeys.
#[test]
fn the_mint_ack_names_the_pinned_checkout_when_it_is_not_the_workbenchs() {
    let corpus = TrialCorpus::build(State::Fresh);
    let worktree = corpus
        .repo()
        .parent()
        .expect("a parent dir")
        .join("side-wt");
    corpus.git(&[
        "worktree",
        "add",
        "-b",
        "side",
        worktree.to_str().expect("utf-8 worktree path"),
        "HEAD",
    ]);

    let from_worktree = run_jigc_in(&worktree, &corpus.home(), &["task", "amend", "repair here"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&from_worktree.stdout),
        String::from_utf8_lossy(&from_worktree.stderr),
    );
    assert!(
        from_worktree.status.success(),
        "the mint door commits nothing, so it mints here:\n{text}",
    );
    assert!(
        text.contains("that is the `HEAD` of the linked worktree at")
            && text.contains("on branch `side`")
            && text.contains("not of the main checkout jigc's workbench binds to"),
        "the mint ack names the checkout it pinned, and the branch that checkout is on:\n{text}",
    );

    // The control: the same door from the main checkout says nothing about a checkout.
    let from_main = corpus.jigc_ok(&["task", "amend", "repair at home"]);
    assert!(
        from_main.contains("amending: ") && !from_main.contains("linked worktree"),
        "…and stays silent where the pin IS the workbench's, rather than printing a line \
         that is always true and never news:\n{from_main}",
    );
}

// ─────────────────── the two gates the arm owes its preview ───────────────────

/// **`jigc task validate` previews the index gate the amend arm's `finalize` enforces** —
/// the `carryover` member of [`cli::gate_coverage::Tier::Previewed`], whose *check* is the
/// commit model's: `finalize.carried-staged` on the ordinary one,
/// `finalize.amend-index-dirty` on the amend model.
///
/// Same check, same severity, same exit at all three doors, which is the whole of what
/// `Door::Previewed` promises — and until F-10's acceptance the preview read **clean** over
/// a state its own committing door refuses at exit 3, the class M52 Increment 3 closed for
/// the posture member.
#[test]
fn task_validate_previews_the_amend_index_gate_at_the_finalize_exit() {
    let (corpus, task) = amend_ready("repair the install message", "the authored subject");
    std::fs::write(
        corpus.repo().join("unrelated.txt"),
        "not this commit's work\n",
    )
    .expect("plant the unrelated file");
    corpus.git(&["add", "unrelated.txt"]);

    for (label, argv) in [
        (
            "jigc task validate",
            vec!["task", "validate", task.as_str()],
        ),
        (
            "jigc task finalize --dry-run",
            vec!["task", "finalize", task.as_str(), "--dry-run"],
        ),
        (
            "jigc task finalize",
            vec!["task", "finalize", task.as_str()],
        ),
    ] {
        let out = corpus.jigc(&argv);
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert_eq!(
            out.status.code(),
            Some(3),
            "`{label}` must take the finalize gate's own exit:\n{text}",
        );
        assert!(
            text.contains("finalize.amend-index-dirty"),
            "`{label}` must raise the arm's index gate:\n{text}",
        );
    }
}

/// **The contrast cell.** An *ordinary* task still previews `finalize.carried-staged` — the
/// same member, the other check. Without this the fix above could have replaced the
/// carryover preview rather than discriminated on the commit model, and the sweep of the
/// amend rows would have been green over it.
#[test]
fn an_ordinary_task_still_previews_the_carryover_gate() {
    let corpus = TrialCorpus::build(State::Fresh);
    std::fs::write(
        corpus.repo().join("carried.txt"),
        "staged before the task existed\n",
    )
    .expect("plant the carried file");
    corpus.git(&["add", "carried.txt"]);
    let task = corpus.start_workflow("single-task", "an ordinary task");
    let out = corpus.jigc(&["task", "validate", &task]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        text.contains("finalize.carried-staged"),
        "the ordinary commit model's index gate is unchanged:\n{text}",
    );
}

/// **The forecast says what the amend would do to the commit graph.** `would commit` is a
/// law-1 lie on an arm that adds nothing, and the fact a reader needs — *which* commit loses
/// its message — was on no surface. The assertion runs over the **emitted** line.
#[test]
fn the_forecast_names_the_commit_it_would_rewrite() {
    let corpus = TrialCorpus::build(State::Fresh);
    let before = corpus.git(&["log", "-1", "--pretty=format:%s"]);
    let short = corpus.git(&["rev-parse", "--short", "HEAD"]);
    let task = mint_amend(&corpus, "repair the install message");
    author_message(&corpus, &task, "the subject it would carry");

    let text = corpus.jigc_ok(&["task", "finalize", &task, "--dry-run"]);
    assert!(
        text.contains(&format!(
            "would rewrite {short} \"{before}\" → \"chore: the subject it would carry\""
        )),
        "the forecast names the commit, the message it carries now, and the message it \
         would carry:\n{text}",
    );
    assert!(
        !text.contains("would commit"),
        "nothing is added on this arm, so the ordinary headline must not appear:\n{text}",
    );
    assert!(
        text.contains("the tree and the author are unchanged"),
        "and the forecast states the fact the empty manifest below it would otherwise \
         leave a reader to infer:\n{text}",
    );

    // The envelope's `subject` is the message jigc will hand git — one shape across both
    // commit models, which is why the arm needs no second `ENVELOPE_ARMS` row.
    let json = corpus.jigc_ok(&["task", "finalize", &task, "--dry-run", "--format", "json"]);
    let doc: Value = serde_json::from_str(&json).expect("one document");
    assert_eq!(
        doc["subject"].as_str(),
        Some("chore: the subject it would carry"),
        "the forecast's envelope key is the NEW subject on both models:\n{doc:#}",
    );
    assert_eq!(
        corpus.git(&["log", "-1", "--pretty=format:%s"]),
        before,
        "a forecast commits nothing",
    );
}

// ───────── the refusal's locus is the id that was never minted (F-10 review, LOW-5) ─────────

/// **`amend.head-shape` keys its target on the id the mint *would* have taken** — both cells
/// of the axis that decides that id (the F-10 review's LOW-5).
///
/// The axis is `{intent supplied, intent absent}`, which is the whole of what the door branches
/// on when it names the task: a supplied intent slugs like every other work-unit id, an absent
/// one falls back to the commit itself (`amend-<sha7>`). Shipped, the locus was the **fallback
/// in both cells**, so a refused `jigc task amend "root probe"` addressed
/// `work-unit:amend-b219d05` — a work unit no invocation would ever have created — while
/// `head_shape_refusal`'s own doc-comment claimed it named the id the mint would have taken.
///
/// One fixture, driven twice: the refusal mints nothing, so the same root-`HEAD` repository
/// answers both cells.
#[test]
fn the_head_shape_refusal_names_the_id_the_mint_would_have_taken() {
    let fixture = GitStateRepo::build(GitState::Unborn);
    let repo = fixture.repo();
    let home = fixture.home();
    // `jigc setup` births `HEAD` on an unborn repository, so the one commit it leaves is the
    // root commit the shape gate refuses.
    let setup = run_jigc(&repo, &home, &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` births HEAD:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    let short = git(&repo, &["rev-parse", "--short", "HEAD"]);

    for (cell, argv, expected) in [
        (
            "an intent supplied",
            vec!["task", "amend", "repair the first commit"],
            "at: work-unit:repair-the-first-commit".to_string(),
        ),
        (
            "no intent",
            vec!["task", "amend"],
            format!("at: work-unit:amend-{short}"),
        ),
    ] {
        let out = run_jigc(&repo, &home, &argv);
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            text.contains("amend.head-shape") && text.contains(&expected),
            "[{cell}] the refusal names the id the mint would have taken — `{expected}`:\n{text}",
        );
        assert!(
            !repo
                .join(".jigc/tasks")
                .join("repair-the-first-commit")
                .exists()
                && !repo
                    .join(".jigc/tasks")
                    .join(format!("amend-{short}"))
                    .exists(),
            "[{cell}] …and that id is exactly what does NOT exist: the refusal mints nothing",
        );
    }
}

// ───────── the arm's sentences are the arm's (F-10 review, MEDIUM-3 / LOW-8) ─────────

/// **No surface of the amend arm tells the agent to do what the arm refuses** — and the two
/// that describe the rewrite say what it rewrites (the F-10 review's MEDIUM-3 and LOW-8).
///
/// **The set: every left-out consumer that is live on this arm**, which is 2 of the 3
/// (`cli::render::left_out_lines`' callers — the `--dry-run` forecast, the landing run's
/// pre-commit advisory, and the landed residual, which is inert here because the arm supplies
/// an empty manifest by construction). Both live ones are driven.
///
/// Driven at `48d1d529`: over a dirty worktree both printed *"left-out (unstaged/untracked —
/// **git add** to include)"*, and following that instruction makes the same finalize refuse at
/// exit 3 with `finalize.amend-index-dirty` — a guidance clause the same binary refuses, which
/// is the route floor's defect one tier down. The landing run's stem also said *"about to
/// commit the index"*, on an arm that commits no tree change and has already refused unless
/// that index is empty.
///
/// And the rewrite's own sentence: `git commit --amend` preserves the author and author date
/// and **resets the committer identity and date**, so *"only its message was rewritten"* was
/// false on both the forecast and the landed ack.
#[test]
fn the_amend_arms_sentences_describe_the_amend() {
    let (corpus, task) = amend_ready("repair the message", "the re-authored subject");
    // A dirty worktree with nothing staged — the state the arm lands in, and the one the
    // left-out surfaces describe. The index stays empty: a staged path would refuse instead.
    std::fs::write(corpus.repo().join("README.md"), "edited after the commit\n")
        .expect("dirty an existing tracked file");
    std::fs::write(corpus.repo().join("untracked.txt"), "wip\n").expect("write an untracked file");
    let author_before = corpus.git(&["log", "-1", "--format=%an <%ae> %aI"]);

    let forecast = corpus.jigc_ok(&["task", "finalize", &task, "--dry-run"]);
    let landing = corpus.jigc_ok(&["task", "finalize", &task]);

    for (surface, text) in [("--dry-run", &forecast), ("the landing run", &landing)] {
        assert!(
            !text.contains("git add to include"),
            "[{surface}] must not name `git add`: this arm refuses any staged path \
             (`finalize.amend-index-dirty`), so that is a route the same binary refuses:\n{text}",
        );
        assert!(
            text.contains("an amend commits no tree change, so none of it can join"),
            "[{surface}] says why the left-out set cannot join instead:\n{text}",
        );
        assert!(
            text.contains("untracked.txt") && text.contains("README.md"),
            "[{surface}] still NAMES the set — the fix is the guidance, not the \
             disclosure:\n{text}",
        );
    }
    assert!(
        !landing.contains("about to commit the index"),
        "the landing run's stem is the model's too — this arm commits no tree change and has \
         already refused unless the index is empty:\n{landing}",
    );
    assert!(
        landing.contains("about to rewrite HEAD's message"),
        "…and says what it is about to do instead:\n{landing}",
    );

    // LOW-8: the committer rewrite, on both surfaces, and driven rather than asserted.
    for (surface, text) in [("--dry-run", &forecast), ("the landed ack", &landing)] {
        assert!(
            text.contains("the tree and the author are unchanged")
                && text.contains("the committer becomes you, now"),
            "[{surface}] `git commit --amend` resets the committer identity and date — on a \
             shared repository that silently re-attributes someone else's commit, so the \
             surface says so:\n{text}",
        );
        assert!(
            !text.contains("only its message"),
            "[{surface}] and it no longer claims the message is the only thing that moved:\n{text}",
        );
    }
    assert_eq!(
        corpus.git(&["log", "-1", "--format=%an <%ae> %aI"]),
        author_before,
        "the sentence is true in the half it claims preservation for: the author and the \
         author date survive the amend byte-for-byte",
    );
}

// ───────── every composing door names the commit (F-10 review, MEDIUM-2) ─────────

/// **Every door that composes an amend task carries the `amending:` block** — the mint and
/// both re-compose doors (the F-10 review's MEDIUM-2).
///
/// **The set it iterates is the composing axis, not a site list.** A `Composition` is built
/// at three seams and the review named two of them; driving found the third to be a different
/// class. `cli::start::compose_core` mints a *fresh* task and can therefore never hold an
/// amend marker (`jigc start --workflow amend` and `jigc workflow amend --preview` are both
/// refused `workflow.verb-routed`), so the live set is *the doors that compose an **existing**
/// area* — one spine, [`compose_task_workflow`], reached by `jigc start --task <id>` and by
/// `jigc workflow <W> --task <id>` — plus the mint door itself. All three are driven here.
///
/// **Why it is not cosmetic.** The block is the settle's *only* mitigation for the cell it
/// deliberately refuses to refuse: jigc cannot tell whether `HEAD` is a milestone boundary, a
/// record-only bookkeeping commit or a foreign commit, so instead of a discriminator it shows
/// the subject line before the instructions. Driven at `48d1d529`, the resume path — the one
/// five consecutive trials show an agent taking when context is lost — dropped it while the
/// composed step still said *"read HEAD's subject line in the ack above"*.
#[test]
fn every_composing_door_names_the_commit_the_amend_repairs() {
    let corpus = TrialCorpus::build(State::Fresh);
    let short = corpus.git(&["rev-parse", "--short", "HEAD"]);
    let subject = corpus.git(&["log", "-1", "--format=%s"]);
    let expected = format!("amending: {short} {subject:?}");

    // The mint, on its text arm — the id read off the header the binary prints.
    let minted = corpus.jigc_ok(&["task", "amend", "repair the install message"]);
    let task = minted
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("the mint ack names the task it minted")
        .trim()
        .to_owned();

    for (door, argv) in [
        ("jigc task amend", None),
        (
            "jigc start --task <id>",
            Some(vec!["start", "--task", task.as_str()]),
        ),
        (
            "jigc workflow amend --task <id>",
            Some(vec!["workflow", "amend", "--task", task.as_str()]),
        ),
    ] {
        let text = match argv {
            None => minted.clone(),
            Some(argv) => corpus.jigc_ok(&argv),
        };
        assert!(
            text.contains(&expected),
            "[{door}] names the commit it is repairing and the subject it is about to \
             replace — `{expected}`:\n{text}",
        );
        assert!(
            text.contains("already been pushed"),
            "[{door}] carries the pushed-history advisory, which rides the same block:\n{text}",
        );
        // And the step's own pointer is to a command, never to an ack this surface may not
        // have: the `--format json` arm carries the step text and no block at all (declared
        // out — `tests/text_json_parity_axis.rs`, the `jigc task amend` census row).
        assert!(
            !text.contains("in the ack above"),
            "[{door}] the composed step points at a command a reader can run, not at a \
             surface that may not be there:\n{text}",
        );
    }

    let json = corpus.jigc_ok(&["start", "--task", &task, "--format", "json"]);
    let doc: Value = serde_json::from_str(&json).expect("the composed envelope is one document");
    assert!(
        !doc["text"]
            .as_str()
            .expect("the pinned `{task, text}` arm carries the composed text")
            .contains("in the ack above"),
        "the pinned envelope's `text` points at no ack either — the block is presentation \
         and declared out of this arm:\n{doc:#}",
    );
}

// ───────── the write doors: an amend task carries one doc (F-10 review, HIGH-1) ─────────

/// The committed **non-singleton** [`State::Vendored`] carries, and its canonical home.
///
/// Non-singleton on purpose: `jigc doc rename` refuses a singleton outright
/// (`write.identity-change` — its `# H1` is the schema's), so a sweep run against
/// `vision`/`changelog` would report that leaf as *covered* while the address never reached
/// the seam under test.
const SPEC_ADDRESS: &str = "spec:padding";
const SPEC_HOME: &str = "docs/specs/padding.md";

/// What a `jigc doc` write leaf answers when it is aimed at a **managed** doc from inside an
/// amend task.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WriteVerdict {
    /// The F-10 refusal: the leaf reaches the copy-on-first-touch seam, and the seam refuses
    /// before `copy_in` — so the *"re-promoted at finalize"* ack is never printed and
    /// nothing is staged.
    AmendRefusal,
    /// Already closed, by a **different shipped gate**, before this fix existed: the leaf
    /// mints through the create-gate and the `amend` workflow declares `allows-create: []`.
    /// Carried as a row rather than dropped from the sweep, because a leaf missing from the
    /// table is indistinguishable from a leaf nobody thought about — and because the review
    /// counted these two inside the copy-on-write class, which driving refutes.
    CreateGate,
}

impl WriteVerdict {
    fn code(self) -> &'static str {
        match self {
            WriteVerdict::AmendRefusal => "finalize.amend-staged-doc",
            WriteVerdict::CreateGate => "create.gate-blocked",
        }
    }
}

/// One `(leaf, arm)` **occurrence** of a `jigc doc` write verb aimed at a managed doc.
///
/// Keyed by occurrence rather than by leaf because `set-field` reaches the seam twice — its
/// set arm and its `--unset` arm are two call sites of one leaf — which is the
/// `PATH_ARG_OCCURRENCES` shape, one registry over.
struct DocWrite {
    /// The clap leaf, spelled as [`cli::cli::VERB_KINDS`] spells it. The sweep is fenced ⇔
    /// against that registry, so a ninth `doc` write leaf reddens here until it is answered.
    leaf: &'static str,
    /// What distinguishes this row from its sibling arm of the same leaf.
    arm: &'static str,
    /// The argv after `jigc`, minus `--task <id>`, which the sweep appends.
    argv: &'static [&'static str],
    /// The payload a `--from-file -` row reads from stdin.
    stdin: Option<&'static str>,
    verdict: WriteVerdict,
}

/// A valid `doc author` payload for the committed spec — valid **deliberately**, so the row
/// proves the create-gate answers rather than the payload parser, which runs first.
const AUTHOR_PAYLOAD: &str = "title: Padding\nsections:\n  - id: goal\n    set:\n      goal: |\n        <<a goal authored through the batch verb>>\n";

/// Every `jigc doc` write leaf, aimed at the committed `spec:padding`.
const DOC_WRITE_LEAVES: &[DocWrite] = &[
    DocWrite {
        leaf: "set-field",
        arm: "set",
        argv: &[
            "doc",
            "set-field",
            "spec:padding#criteria/pads-the-input/maps-to-test",
            "--value",
            "tests/pad_test.rs#pads_the_input",
        ],
        stdin: None,
        verdict: WriteVerdict::AmendRefusal,
    },
    DocWrite {
        leaf: "set-field",
        arm: "--unset",
        argv: &[
            "doc",
            "set-field",
            "spec:padding#criteria/pads-the-input/maps-to-test",
            "--unset",
        ],
        stdin: None,
        verdict: WriteVerdict::AmendRefusal,
    },
    DocWrite {
        leaf: "set-slot",
        arm: "--from-file -",
        argv: &["doc", "set-slot", "spec:padding#goal", "--from-file", "-"],
        stdin: Some("a goal the amend task tried to rewrite\n"),
        verdict: WriteVerdict::AmendRefusal,
    },
    DocWrite {
        leaf: "add-item",
        arm: "--title",
        argv: &[
            "doc",
            "add-item",
            "spec:padding#criteria",
            "--title",
            "Another criterion",
        ],
        stdin: None,
        verdict: WriteVerdict::AmendRefusal,
    },
    DocWrite {
        leaf: "remove-item",
        arm: "the item address",
        argv: &["doc", "remove-item", "spec:padding#criteria/pads-the-input"],
        stdin: None,
        verdict: WriteVerdict::AmendRefusal,
    },
    DocWrite {
        leaf: "retitle-item",
        arm: "--title",
        argv: &[
            "doc",
            "retitle-item",
            "spec:padding#criteria/pads-the-input",
            "--title",
            "Pads it",
        ],
        stdin: None,
        verdict: WriteVerdict::AmendRefusal,
    },
    DocWrite {
        // The same-slug arm: a re-slug of a committed identity is refused on its own code,
        // but **after** the copy-in — driven on the baseline, a *refused* re-slug still
        // staged the doc and the amend then promoted it. So the seam guard has to fire
        // first, and this row is the one that proves it does.
        leaf: "rename",
        arm: "--to (same slug, retitle in place)",
        argv: &["doc", "rename", "spec:padding", "--to", "padding"],
        stdin: None,
        verdict: WriteVerdict::AmendRefusal,
    },
    DocWrite {
        leaf: "create",
        arm: "--title",
        argv: &["doc", "create", "spec", "--title", "Another spec"],
        stdin: None,
        verdict: WriteVerdict::CreateGate,
    },
    DocWrite {
        leaf: "author",
        arm: "--from-file -",
        argv: &["doc", "author", "spec", "--from-file", "-"],
        stdin: Some(AUTHOR_PAYLOAD),
        verdict: WriteVerdict::CreateGate,
    },
];

/// **The table is total over the clap tree.** Membership is `cli::cli::VERB_KINDS` filtered
/// to `doc` × `Write`, read from the registry rather than written down here, so a ninth
/// write leaf cannot ship without a row saying what it does inside an amend task.
#[test]
fn the_sweep_covers_every_doc_write_leaf() {
    let declared: std::collections::BTreeSet<&str> =
        DOC_WRITE_LEAVES.iter().map(|row| row.leaf).collect();
    let registry: std::collections::BTreeSet<&str> = cli::cli::VERB_KINDS
        .iter()
        .filter(|(path, kind)| path.first() == Some(&"doc") && *kind == cli::cli::VerbKind::Write)
        .filter_map(|(path, _)| path.get(1).copied())
        .collect();
    assert_eq!(
        declared, registry,
        "`DOC_WRITE_LEAVES` must cover exactly the `doc` leaves `cli::cli::VERB_KINDS` \
         classifies as `Write` — a leaf missing here is a leaf nobody decided about",
    );
}

/// **No `jigc doc` write verb stages a managed doc into an amend task** — the F-10 review's
/// HIGH-1 at its write half, over the whole leaf axis.
///
/// The seam it closes is one site: `cli::doc`'s `read_or_copy_in` is the only production
/// caller of `engine::state::copy_in`, and the six leaves that can stage a *committed* doc
/// all come through it. The other two mint through the create-gate, which the `amend`
/// workflow's `allows-create: []` already refuses — driven, and recorded as its own verdict
/// rather than folded into the six.
///
/// Each cell asserts what a refusal has to be worth: the exit, the code, **and** that
/// nothing was staged — because the defect this closes was a write that acked
/// *"re-promoted at finalize"* at exit 0 and staged the doc anyway.
#[test]
fn no_doc_write_verb_stages_a_managed_doc_into_an_amend_task() {
    let corpus = TrialCorpus::build(State::Vendored);
    let task = mint_amend(&corpus, "repair the arch-doc commit message");
    author_message(&corpus, &task, "the authored subject");
    let staged_before = staged_doc_names(&corpus, &task);

    for row in DOC_WRITE_LEAVES {
        let cell = format!("jigc {} ({})", row.argv.join(" "), row.arm);
        let mut argv: Vec<&str> = row.argv.to_vec();
        argv.extend_from_slice(&["--task", task.as_str()]);
        let out = match row.stdin {
            Some(payload) => corpus.jigc_stdin(&argv, payload),
            None => corpus.jigc(&argv),
        };
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert_eq!(
            out.status.code(),
            Some(1),
            "[{cell}] a write-door refusal takes the operational exit:\n{text}",
        );
        assert!(
            text.contains(row.verdict.code()),
            "[{cell}] must refuse with `{}`:\n{text}",
            row.verdict.code(),
        );
        if row.verdict == WriteVerdict::AmendRefusal {
            assert!(
                text.contains(SPEC_HOME),
                "[{cell}] the refusal names the home the doc would have been promoted to:\n{text}",
            );
            assert!(
                text.contains("jigc start"),
                "[{cell}] and routes at the task that CAN land it — this amend task keeps \
                 its own job:\n{text}",
            );
        }
        assert!(
            !text.contains("copied in for update"),
            "[{cell}] the copy-in ack is the lie the seam guard exists to stop printing:\n{text}",
        );
        assert_eq!(
            staged_doc_names(&corpus, &task),
            staged_before,
            "[{cell}] a refused write stages nothing — the area still holds only the \
             transient commit doc",
        );
    }

    // And the task is still a good amend task: the refusals cost it nothing, so its own
    // job still lands.
    let landed = corpus.jigc_ok(&["task", "finalize", &task]);
    assert!(
        landed.contains("the tree and the author are unchanged"),
        "the amend still lands after every refused doc write:\n{landed}",
    );
}

/// **The control: the amend task's own transient `commit` doc takes every write it needs.**
///
/// The guard keys on `engine::finalize::promote_destination` — *does this doc promote?* — so
/// a fix that had keyed on *"is this a `doc` write verb?"* instead would have broken the arm
/// it exists to protect, and every refusal above would still have been green.
///
/// Five of the eight leaves, and the three absences are stated rather than skipped:
/// `create`/`author` mint through the gate the workflow closes (the task's commit doc is
/// provisioned by the mint, not created by the agent), and `rename` would re-slug the commit
/// doc away from the task id its finalize looks it up by — neither is part of authoring a
/// message.
#[test]
fn the_amend_tasks_own_commit_doc_takes_every_write_it_needs() {
    let corpus = TrialCorpus::build(State::Vendored);
    let task = mint_amend(&corpus, "repair the arch-doc commit message");

    corpus.set_field(&format!("commit:{task}#type"), &task, "fix"); // set-field
    corpus.set_slot(
        &format!("commit:{task}#summary"),
        &task,
        "the repaired subject",
    ); // set-slot
    let keep = corpus.add_item(&format!("commit:{task}#trailers"), "Refs", &task); // add-item
    corpus.set_field(&format!("{keep}/value"), &task, "F-10");
    let drop = corpus.add_item(&format!("commit:{task}#trailers"), "Temp", &task);
    corpus.set_field(&format!("{drop}/value"), &task, "scratch");
    corpus.jigc_ok(&[
        "doc",
        "retitle-item",
        &drop,
        "--title",
        "Temporary",
        "--task",
        &task,
    ]); // retitle-item
    // The **same** address: the `{#id}` anchor is frozen by the retitle, so the item the
    // removal names is the one the `add-item` minted, not one re-slugged from the new
    // heading (`design/write-commands.md` → the retitle-without-reslug invariant).
    corpus.jigc_ok(&["doc", "remove-item", &drop, "--task", &task]); // remove-item

    let landed = corpus.jigc_ok(&["task", "finalize", &task]);
    assert!(
        landed.contains("the tree and the author are unchanged"),
        "the amend lands over its own doc:\n{landed}",
    );
    let message = corpus.git(&["log", "-1", "--pretty=format:%B"]);
    assert!(
        message.starts_with("fix: the repaired subject") && message.contains("Refs: F-10"),
        "every accepted write reached the rendered message:\n{message}",
    );
    assert!(
        !message.contains("Temporary"),
        "including the removal:\n{message}",
    );
}

/// **All three doors answer the staged-doc gate identically, and the forecast stops naming
/// a promotion it will not make.**
///
/// The `--dry-run` line read `promoted VISION.md` on the defect — a forecast of a write that
/// would land in the worktree and in no commit — so the fix is only complete when that line
/// is gone, and the preview promise (*same check, same severity, same exit*) is what makes
/// the three doors one answer.
#[test]
fn every_gate_door_previews_the_staged_doc_refusal_and_promises_no_promotion() {
    let corpus = TrialCorpus::build(State::Vendored);
    let task = mint_amend(&corpus, "repair the arch-doc commit message");
    author_message(&corpus, &task, "the authored subject");
    plant_staged_doc(&corpus, &task);

    let baseline = std::fs::read_to_string(corpus.repo().join(".jigc/state/file-state.json"))
        .expect("the corpus has a file-state baseline");
    let before_validate = corpus.jigc(&["validate"]);

    for (label, argv) in [
        (
            "jigc task validate",
            vec!["task", "validate", task.as_str()],
        ),
        (
            "jigc task finalize --dry-run",
            vec!["task", "finalize", task.as_str(), "--dry-run"],
        ),
        (
            "jigc task finalize",
            vec!["task", "finalize", task.as_str()],
        ),
    ] {
        let out = corpus.jigc(&argv);
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert_eq!(
            out.status.code(),
            Some(3),
            "`{label}` must take the finalize gate's own exit:\n{text}",
        );
        assert!(
            text.contains("finalize.amend-staged-doc"),
            "`{label}` must raise the arm's staged-doc gate:\n{text}",
        );
        assert!(
            !text.contains("promoted "),
            "`{label}` must not forecast a promotion this arm commits nowhere:\n{text}",
        );
    }

    // The promotion was **never made**: no worktree write, no baseline advance, and the
    // store's own verdict is the verdict it had. The last one is the false green the
    // defect produced — `jigc validate` read exit 0 over bytes no commit carried.
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        "",
        "the refused finalize wrote nothing into the worktree",
    );
    assert_eq!(
        std::fs::read_to_string(corpus.repo().join(".jigc/state/file-state.json")).ok(),
        Some(baseline),
        "and baselined nothing — the defect advanced file-state onto its own uncommitted \
         promotion, which is what made `jigc validate` green over it",
    );
    let after_validate = corpus.jigc(&["validate"]);
    assert_eq!(
        after_validate.status.code(),
        before_validate.status.code(),
        "the store's verdict is unchanged",
    );
    assert_eq!(
        String::from_utf8_lossy(&after_validate.stdout),
        String::from_utf8_lossy(&before_validate.stdout),
        "and it says the same thing about the same store",
    );
}

/// Plant the committed spec into `task`'s staged-docs area, byte-for-byte — the state the
/// finalize gate is the backstop for, reachable by no `jigc doc` verb since the write half
/// of this fix landed.
fn plant_staged_doc(corpus: &TrialCorpus, task: &str) {
    let docs = corpus.repo().join(".jigc/tasks").join(task).join("docs");
    std::fs::create_dir_all(&docs).expect("the staged-docs dir");
    std::fs::copy(
        corpus.repo().join(SPEC_HOME),
        docs.join(format!("{SPEC_ADDRESS}.md")),
    )
    .expect("plant the staged managed doc");
}

/// The `<type>:<slug>.md` entries of `task`'s staged-docs area, sorted — *what this task
/// staged*, as the gate's own subject reads it.
fn staged_doc_names(corpus: &TrialCorpus, task: &str) -> Vec<String> {
    let docs = corpus.repo().join(".jigc/tasks").join(task).join("docs");
    let mut names: Vec<String> = std::fs::read_dir(&docs)
        .expect("the staged-docs dir")
        .map(|entry| {
            entry
                .expect("a dir entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.contains(':'))
        .collect();
    names.sort();
    names
}

// ───────────────────────────── plumbing ─────────────────────────────

/// A `pre-commit` hook that refuses everything, with its own words on stderr.
fn install_rejecting_hook(repo: &Path) {
    let hook = repo.join(".git/hooks/pre-commit");
    std::fs::create_dir_all(hook.parent().expect("the hooks dir")).expect("mk the hooks dir");
    std::fs::write(&hook, "#!/bin/sh\necho 'the hook refuses' >&2\nexit 1\n")
        .expect("write the rejecting hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755))
            .expect("make the hook executable");
    }
}

/// `jigc` in a repository this suite built outside a [`TrialCorpus`].
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    run_jigc_in(repo, home, args)
}

/// `jigc` with an explicit cwd — the worktree control's shape.
fn run_jigc_in(cwd: &Path, home: &Path, args: &[&str]) -> Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run jigc")
}

/// `git` in a directory this suite built outside a [`TrialCorpus`], trimmed.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}
