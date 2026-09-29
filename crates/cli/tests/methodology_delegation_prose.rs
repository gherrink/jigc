//! M49 Increment 10 / T1 — the **delegation prose sweep**: every phase step of the
//! three methodology loops **names the actor and the instrument**
//! ([DECISIONS.md](../../../DECISIONS.md) → the M49 Settle, D7), applying over the
//! whole class what `crates/cli/packs/methodology/steps/plan-review.yaml` already did at
//! exactly one site (*"A reader who did NOT author the decisions…"*).
//!
//! **The rule it enforces** — [finalize.md](../../../design/finalize.md):131, written
//! on the *family* rather than on a file: *"The contract only binds if the composed
//! workflow says it."* A delegation mandated in a locked artifact but absent from the
//! composed bytes binds nobody: the agent never reads
//! `implementation/increment-workflow.md`, it reads what `jigc` prints. At HEAD before
//! this task, `step:validate` said *"independently"* and named **no actor** while
//! `increment-workflow.md:19` requires *"an agent that did not build it and cannot
//! edit or commit"*, and the **independent robust-case advocate** — mandatory in
//! **two** locked artifacts (`milestone-planning-workflow.md:33`,
//! `milestone-completion-workflow.md:29`) — appeared in **zero** pack files.
//!
//! **jigc dispatches nothing, counts nothing, scores nothing** ([DECISIONS.md] →
//! 2026-06-21, *the orchestration does not fold*). This is prose in a step body: the
//! step *states who must act and with what instrument*; the CLI neither spawns the
//! actor nor checks that it ran. The bar this suite holds is therefore a statement in
//! the composed bytes, never a mechanism.
//!
//! **Why a standing suite over a dispositioned set, and not a pack-load fence.** The
//! M43/M47 stated-at tiers derive their owe-set from a *structural* signal
//! (`{{schema:<T>}}` × singleton, a `set:`-kind, a committing door). Delegation has
//! none — `crates/cli/packs/methodology/steps/triage.yaml` carries the mandated defer fork while
//! never using the word *"defer"*, so a keyword selector misses the class's hardest
//! member. Where the owe-set is a judgment rather than a signal, this repo's idiom is
//! a standing suite whose subject is **derived** and whose every member is
//! **disposed** — `author_write_contract.rs`, `compose_goldens::EXCLUSIONS`
//! ([pinning.md](../../../implementation/pinning.md) §1).
//!
//! **The subject is derived, never hand-listed.** It is the union of the *phase*
//! steps the three methodology loop workflows include — `planning`, `increment`,
//! `completion` read from the **embedded** methodology pack at runtime — minus the
//! `author-*` solicitation steps and the `*-finalize` commit steps, which are write
//! and commit machinery rather than phases of the loop. A step added to, or dropped
//! from, any of the three moves this axis with no edit here, and
//! [`DISPOSITIONS`] must then be brought back into bijection with it or arm 1 reddens.
//!
//! **Every member is disposed** — `In` with the facts it owes, or `Out` with the
//! reason no delegation is owed (executable metadata, read by arm 1, never a silent
//! skip). The owed facts are asserted on the **composed bytes of the real binary**
//! over the `[dev ▸ methodology]` pack-set a plain `jigc setup` installs — the text an
//! agent is actually handed — not on the YAML source, which would pass while the
//! compose path dropped the step.
//!
//! **The omitting axis is swept too** (hardening #5): a fact owed by a step a given
//! workflow does **not** include must be **absent** from that workflow's compose — so
//! a sentence cannot be parked in a shared step and counted for the loop that never
//! composes it. `increment` composes neither `settle` nor `triage`, so the advocate
//! is owed there by nobody and must not appear.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cli::pack::EmbeddedPack;
use engine::compose::load_workflow_def;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

/// The three methodology loop workflows whose includes *are* the axis.
const PHASE_WORKFLOWS: [(&str, &str); 3] = [
    ("planning", "M98 the planning loop"),
    ("increment", "M98 increment 1"),
    ("completion", "M99 the completion loop"),
];

/// What a phase step owes the composed surface.
enum Disposition {
    /// A delegation a locked artifact mandates: the step must state it. Each fact is
    /// a verbatim phrase the composed bytes must carry (whitespace-flowed, because
    /// step prose is hard-wrapped and a sentence straddles the wrap column).
    In(&'static [&'static str]),
    /// No delegation is owed, with the reason — read by arm 1, never a silent skip.
    Out(&'static str),
}

use Disposition::{In, Out};

/// Every phase step, disposed. Bijective with the derived set (arm 1).
///
/// The `In` facts are quoted from the locked artifact that mandates them:
/// `implementation/increment-workflow.md` (:19, :110),
/// `implementation/milestone-planning-workflow.md` (:24, :33, :39, :67),
/// `implementation/milestone-completion-workflow.md` (:19, :29, :34).
const DISPOSITIONS: &[(&str, Disposition)] = &[
    // ---- planning ---------------------------------------------------------
    (
        "plan-scope",
        // planning:24 — the capability ledger is "established by delegated recon
        // (subagents that exercise the real binary, not the orchestrator reading
        // code)". The step named the instrument and no actor.
        In(&["delegate the baseline audit to recon agents that exercise the real binary"]),
    ),
    (
        "detect-gaps",
        // planning:67 — the gap pass is run by delegated readers in parallel, and
        // planning:33's bar is that the party asserting a claim is not the party
        // testing it.
        In(&["several independent readers rather than one"]),
    ),
    (
        "settle",
        // planning:33 + completion:29 — the cheap-vs-robust fork "is never
        // self-framed": an independent robust-case advocate argues the vision-robust
        // case at full strength before the human decides. planning:39 — write-back
        // may be delegated to writer subagents only under its two conditions.
        In(&[
            "commission an independent robust-case advocate",
            "delegated to writer subagents",
        ]),
    ),
    (
        "plan-review",
        // planning:67 (`design-reviewer`) — already satisfied at HEAD; this is the
        // site the sweep generalizes from, held here so it cannot rot back.
        In(&["A reader who did NOT author the decisions"]),
    ),
    (
        "decompose",
        Out(
            "planning:67 keeps Decompose inline with the human alongside Settle — no \
             actor is delegated and no instrument is commissioned, so there is nothing \
             for the step to name that it is not itself",
        ),
    ),
    // ---- increment --------------------------------------------------------
    (
        "plan",
        Out(
            "increment:110 names one planning agent per increment — the actor OF this \
             phase, which is the reader the composed step already addresses. It carries \
             no independence constraint and commissions no second party, so the step \
             would be naming itself",
        ),
    ),
    (
        "plan-gate",
        Out(
            "a checkpoint step whose only actor is the human, already named in its body \
             (\"surface it to the human\") — the halt is the delegation",
        ),
    ),
    (
        "execute",
        Out(
            "increment:110 names one agent per task — the actor OF this phase — and the \
             step already states its instrument (the dev workflow verbatim) and its \
             binding constraint (strictly serial over one shared working tree). What \
             remains is dispatch shape, which jigc does not own",
        ),
    ),
    (
        "execute-gate",
        Out(
            "a checkpoint step whose only actor is the human, already named in its body \
             (\"Surface it to the human\")",
        ),
    ),
    (
        "validate",
        // increment:19 + :110 — "run by an agent that did not build it and cannot edit
        // or commit (so it cannot certify its own work)". The step said
        // "independently" and named no actor at all.
        In(&["an agent that did not build it and cannot edit or commit"]),
    ),
    (
        "fix-gate",
        // increment:110 ("one agent per blocking finding") + completion:34 ("each
        // confirmed fix is its own agent … provisioned into its own worktree";
        // "Brief the fixer with the finding, never with the finding's boundary").
        // M50 replaced the serial-over-one-tree clause with the fan-out the doc had
        // named as the encode since M42 — the actor and the instrument both moved,
        // so the fact this fence holds moved with them.
        In(&[
            "Each fixer is its own agent in its own worktree",
            "Brief the fixer with the finding, never with the finding's boundary",
        ]),
    ),
    // ---- completion -------------------------------------------------------
    (
        "audit",
        // completion:19-20 — the code review is "a read-only agent that did not build
        // the milestone and cannot edit or commit", the e2e half "an agent that drives
        // the real binary ... not trusting the builders' own tests". The step said
        // "Two independent passes" and named neither actor.
        In(&["neither of them run by an agent that built the milestone"]),
    ),
    (
        "triage",
        // completion:29 — "this fork is never self-framed by the orchestrator: before
        // surfacing a `too-big -> defer` recommendation, spawn an independent
        // robust-case advocate". The class's hardest member: it carries the mandated
        // fork while never using the word "defer".
        In(&["commission an independent robust-case advocate"]),
    ),
    (
        "re-verify",
        Out(
            "completion:36 commissions no party for it — it re-runs the gate and the \
             audit slice the audit phase already commissioned, so the independence it \
             depends on is stated where those passes are commissioned (`audit`)",
        ),
    ),
];

// ---------------------------------------------------------------------------
// The derived axis
// ---------------------------------------------------------------------------

/// A step id that is loop *machinery* rather than a *phase*: the `author-*`
/// solicitations and the `*-finalize` commit steps.
fn is_machinery(step: &str) -> bool {
    step.starts_with("author-") || step == "finalize" || step.ends_with("-finalize")
}

/// The axis: every phase step of the three loops, mapped to the workflows that
/// compose it. Read from the **embedded** methodology pack at runtime.
fn phase_steps() -> BTreeMap<String, BTreeSet<&'static str>> {
    let pack = EmbeddedPack::methodology();
    let mut out: BTreeMap<String, BTreeSet<&'static str>> = BTreeMap::new();
    let mut machinery_seen = false;
    for (workflow, _) in PHASE_WORKFLOWS {
        let bytes = pack
            .read(PackResourceKind::Workflows, &ResourceId::from(workflow))
            .unwrap_or_else(|e| panic!("the methodology pack ships `{workflow}`: {e:?}"));
        let def = load_workflow_def(&bytes).unwrap_or_else(|e| panic!("`{workflow}` loads: {e:?}"));
        assert!(
            !def.includes.is_empty(),
            "`{workflow}` must compose at least one step",
        );
        for step in def.includes {
            if is_machinery(&step) {
                machinery_seen = true;
                continue;
            }
            out.entry(step).or_default().insert(workflow);
        }
    }
    assert!(
        machinery_seen,
        "the machinery filter must actually filter — if the three loops stopped \
         composing any `author-*`/`*-finalize` step, this derivation is no longer the \
         set it documents",
    );
    out
}

// ---------------------------------------------------------------------------
// Harness — the real binary over the `[dev ▸ methodology]` composite
// ---------------------------------------------------------------------------

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-delegation-prose-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `JIGC_PACK_DIR` **removed** — inheriting it would swap the
/// pack out from under the axis this suite derives.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// A repo carrying the `compose-embedded-methodology` marker via a plain `jigc setup`
/// — the pack-set a dogfooding project runs, and the one where per-origin-pack include
/// resolution is live.
fn marker_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
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
    fs::write(repo.path().join("README.md"), "# repo\n").expect("write readme");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    let setup = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "a plain `jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    (repo, home)
}

/// The composed bytes of `jigc start --workflow <id> <intent>`.
fn compose(repo: &Path, home: &Path, workflow: &str, intent: &str) -> String {
    let out = jigc(repo, home, &["start", "--workflow", workflow, intent]);
    assert!(
        out.status.success(),
        "`jigc start --workflow {workflow}` must compose + exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// The composed body with runs of whitespace collapsed. Step prose is hard-wrapped at
/// ~80 columns, so a sentence straddles a line break; the contract is the words, not
/// the wrap column. Still the **emitted** bytes — just not hostage to a reflow.
fn flowed(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn disposition(step: &str) -> Option<&'static Disposition> {
    DISPOSITIONS
        .iter()
        .find(|(id, _)| *id == step)
        .map(|(_, d)| d)
}

/// Every fact owed by the IN members, whether or not a given workflow composes them.
fn all_facts() -> Vec<(&'static str, &'static str)> {
    DISPOSITIONS
        .iter()
        .filter_map(|(id, d)| match d {
            In(facts) => Some(facts.iter().map(move |f| (*id, *f))),
            Out(_) => None,
        })
        .flatten()
        .collect()
}

// ---------------------------------------------------------------------------
// Arm 1 — the disposition is a bijection with the derived set
// ---------------------------------------------------------------------------

#[test]
fn every_derived_phase_step_is_disposed_exactly_once() {
    let derived = phase_steps();
    let disposed: BTreeSet<&str> = DISPOSITIONS.iter().map(|(id, _)| *id).collect();
    assert_eq!(
        disposed.len(),
        DISPOSITIONS.len(),
        "a step is disposed twice — the table must be a set",
    );
    let derived_ids: BTreeSet<&str> = derived.keys().map(|s| s.as_str()).collect();
    assert_eq!(
        derived_ids,
        disposed,
        "the disposition table must be in bijection with the phase steps the three \
         methodology loops compose (derived from the embedded pack at runtime). \
         Undisposed: {:?}; stale: {:?}",
        derived_ids.difference(&disposed).collect::<Vec<_>>(),
        disposed.difference(&derived_ids).collect::<Vec<_>>(),
    );

    for (id, d) in DISPOSITIONS {
        assert!(
            !is_machinery(id),
            "`{id}` is loop machinery, not a phase — it cannot be a member of this axis",
        );
        match d {
            In(facts) => assert!(
                !facts.is_empty() && facts.iter().all(|f| !f.trim().is_empty()),
                "`{id}` is disposed IN, so it must owe at least one stated fact",
            ),
            Out(reason) => assert!(
                reason.trim().len() > 40,
                "`{id}` is disposed OUT, so it must carry the reason no delegation is \
                 owed — a bare skip is the thing this table exists to forbid",
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Arm 2 — the owed facts are in the composed bytes of the real binary
// ---------------------------------------------------------------------------

#[test]
fn every_in_member_names_its_actor_and_instrument_in_the_composed_bytes() {
    let derived = phase_steps();
    let (repo, home) = marker_repo("owed");

    // Every miss is collected, never panicked on at the first: a sweep that stops at
    // member one reports one member and hides the rest of the class.
    let mut missing: Vec<String> = Vec::new();
    for (workflow, intent) in PHASE_WORKFLOWS {
        let flat = flowed(&compose(repo.path(), home.path(), workflow, intent));
        for (step, composers) in &derived {
            if !composers.contains(workflow) {
                continue;
            }
            let In(facts) = disposition(step).expect("arm 1 fences the bijection") else {
                continue;
            };
            for fact in *facts {
                if !flat.contains(fact) {
                    missing.push(format!("{workflow} ▸ step:{step} owes {fact:?}"));
                }
            }
        }
    }
    assert!(
        missing.is_empty(),
        "the composed surface must state every owed delegation — the contract only \
         binds if the composed workflow says it (design/finalize.md:131). Missing:\n{}",
        missing.join("\n"),
    );
}

// ---------------------------------------------------------------------------
// Arm 3 — the omitting axis: a fact stays inside the loop that composes its step
// ---------------------------------------------------------------------------

#[test]
fn a_delegation_fact_is_absent_from_a_loop_that_composes_no_step_owing_it() {
    let derived = phase_steps();
    let (repo, home) = marker_repo("omitting");

    for (workflow, intent) in PHASE_WORKFLOWS {
        let flat = flowed(&compose(repo.path(), home.path(), workflow, intent));
        // Facts this workflow legitimately carries, because it composes a step owing
        // them — a phrase shared by two steps (the advocate, owed by `settle` AND
        // `triage`) is expected wherever either is composed.
        let owed_here: BTreeSet<&str> = all_facts()
            .into_iter()
            .filter(|(step, _)| derived.get(*step).is_some_and(|c| c.contains(workflow)))
            .map(|(_, fact)| fact)
            .collect();
        for (step, fact) in all_facts() {
            if owed_here.contains(fact) {
                continue;
            }
            assert!(
                !flat.contains(fact),
                "`{workflow}` composes no step owing {fact:?} (owed by `step:{step}`), \
                 so it must not carry it — a delegation parked in a shared step and \
                 counted for a loop that never composes it binds nobody. Composed \
                 body:\n{flat}",
            );
        }
        assert!(
            !flat.contains("{{") && !flat.contains("}}"),
            "`{workflow}` must compose with no surviving placeholder; got:\n{flat}",
        );
    }
}
