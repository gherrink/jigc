//! **The pinned-envelope registry's four proofs** (M51 Increment 5 / T7 — `settle-record.md`
//! → D5 amended by §7; the table is
//! [envelope-key-census.md](../../../completions/artifacts/M51/envelope-key-census.md)),
//! hosted on top of this suite's original claim: **every leaf verb honours `--format json`
//! on its success path** (M47 Increment 7 / T4 — [command-output-contract.md](../../../design/command-output-contract.md)
//! → Stream discipline; `implementation/pinning.md` §2).
//!
//! **Why here.** [`cli::render::ENVELOPE_ARMS`] declares, for every `(leaf verb, arm)`, the
//! top-level key set its `--format json` document carries and whether a driver may build on
//! it. Eleven of its sixty rows are chosen by a **dispatch branch**, not by a result enum —
//! `jigc task finalize`'s four (landed · `--dry-run` · blocked · the exit-4 review hold)
//! among them — so the fence has to *drive the binary*: `text_json_parity_axis.rs` renders a
//! witness and therefore cannot see an arm the dispatch chooses. This is the one suite that
//! already drives every leaf to a real success through the real binary, so the registry's
//! proofs ride it rather than minting a second driving harness.
//!
//! **The four proofs**, one test each, which is what makes *"every verb × arm"* an
//! implementable completeness claim rather than a quantifier over a set that exists nowhere:
//!
//!   1. every clap leaf verb has **≥ 1** row — the `⇔` against `Cli::command()`;
//!   2. every production arm has **exactly one** row — `(path, arm)` uniqueness, plus each
//!      result enum's rows equalling that enum's own compile-fenced arm-name table;
//!   3. every row is **driven** — [`recipes`] and the registry are one set;
//!   4. the **driven** top-level key set equals the **declared** one, under the stream
//!      discipline each row's [`cli::render::ArmOutcome`] names.
//!
//! Plus the `schema_version` **partition fence**: the result contract's version integer
//! rides an arm iff that arm's root is a value the result contract versions.
//!
//! **Why the sweep, and not a structural fence.** `adapter.rs`'s `format_is_a_global_arg`
//! asserts that `--format` *is* a global clap arg, which is a necessary condition and
//! nothing more: it stays green if every dispatch arm parses the flag and then ignores it.
//! That is not hypothetical — the M47 baseline found exactly one arm doing precisely that
//! (`TaskCommand::Diff` dropped `format` on the floor and printed plain text at exit 0), and
//! the structural fence never saw it. Behaviour is the only thing that can fence behaviour.
//!
//! **The enumeration is from the clap tree and the result enums; the recipes are
//! hand-written; the two `⇔`s are what stop the recipes being an authoritative hand list.**
//! A verb added anywhere in the tree reddens proof 1 until it is declared, and a declared
//! arm nothing drives reddens proof 3.
//!
//! **Every recipe has a shipped precedent** — the driving sequence is lifted from the
//! verb's own suite (`flow9_seam.rs` / `flow10_acceptance.rs` for the milestone arc,
//! `config_*.rs` for the cascade-authoring verbs, `implement_from_spec.rs` for the bind,
//! `flow40_acceptance.rs` for the freeze-exempt relocation, `flow42_acceptance.rs` arm V5
//! for the optional-scalar `--unset`, `exit_codes.rs` → 4 for the migration review hold,
//! `trial_corpus.rs` for setup/author/finalize) rather than invented here.
//!
//! **Three base corpora, copied per recipe.** Most arms run on a plain `jigc setup` repo; a
//! read/rename/unmanage verb needs a *committed* doc, and `task bind` /
//! `milestone add-from-spec` need a *committed spec with criteria*. Each base is built once
//! and [`TrialCorpus::copy_state`]'d per recipe, so a mutating verb never sees a sibling's
//! leftovers.

use crate::support;

use clap::CommandFactory;
use cli::cli::Cli;
use cli::render::{
    AckTarget, ArmOrigin, ArmOutcome, ArmRoot, ArmShape, ArmStatus, CONFIG_ACK_ARMS, ConfigAck,
    DOC_ACK_ARMS, DocAck, ENVELOPE_ARMS, EnvelopeArm, ORIENTATION_ARMS, STORE_EXIT_FLIPS,
    TASK_ACK_ARMS, TaskAck, config_ack_arm, doc_ack_arm, orientation_arm, task_ack_arm,
};
use cli::task::EXIT_SUCCESS;
use engine::finding::Findings;
use engine::result::OrientationView;
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::process::Output;
use support::trial_corpus::{State, TrialCorpus};

/// Whether `s` parses as **exactly one** JSON document — the pinnable form of "this
/// stream carries the document" (and, negated, of "this stream carries no JSON at
/// all"). `serde_json::from_str` accepts surrounding whitespace but rejects trailing
/// non-whitespace, so a stream holding a document *plus* a plain-text side channel
/// fails, and an empty stream is correctly "no document".
fn is_one_json_doc(s: &str) -> bool {
    serde_json::from_str::<Value>(s).is_ok()
}

/// Every **leaf** verb's argv path, walked from the clap `Command` tree — the real
/// enumeration seam, so a subcommand added anywhere auto-joins this sweep. clap's own
/// auto-generated `help` subcommand is skipped (it is not a jigc verb).
///
/// The `machine_output.rs` idiom, kept local: both suites derive the same set from the
/// same tree, so neither can drift from the surface it sweeps.
fn leaf_verb_paths() -> Vec<Vec<String>> {
    fn walk(cmd: &clap::Command, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
        let mut had_child = false;
        for sub in cmd.get_subcommands() {
            if sub.get_name() == "help" {
                continue;
            }
            had_child = true;
            let mut child = prefix.clone();
            child.push(sub.get_name().to_string());
            walk(sub, child, out);
        }
        if !had_child && !prefix.is_empty() {
            out.push(prefix);
        }
    }
    let mut out = Vec::new();
    walk(&Cli::command(), Vec::new(), &mut out);
    out
}

// ─────────────────────────────── the corpora ───────────────────────────────

/// Which base corpus a recipe runs on.
#[derive(Clone, Copy, Debug)]
enum Base {
    /// `jigc setup` only — the great majority of verbs need nothing more.
    Fresh,
    /// [`Base::Fresh`] plus a **committed** `adr:cache-strategy` — what a read
    /// (`doc show` / `doc list`) or an identity op (`rename` / `unmanage`) needs.
    CommittedAdr,
    /// [`Base::Fresh`] plus a **committed** `spec:rate-limiting` carrying one
    /// criterion — what `task bind` and `milestone add-from-spec` enumerate.
    CommittedSpec,
}

/// The three built bases, held for the duration of the sweep (each recipe takes a
/// copy). They drop together with the test.
struct Bases {
    fresh: TrialCorpus,
    committed_adr: TrialCorpus,
    committed_spec: TrialCorpus,
}

impl Bases {
    fn build() -> Self {
        let fresh = TrialCorpus::build(State::Fresh);

        let committed_adr = fresh.copy_state();
        let task = committed_adr.start_workflow("single-task", "harden the cache");
        create_adr(&committed_adr, &task);
        fill_adr(&committed_adr, &task);
        committed_adr.finalize(&task, "cache", "harden the cache", false);

        let committed_spec = fresh.copy_state();
        let task = committed_spec.start_workflow("plan", "spec the rate limiter");
        create_spec(&committed_spec, &task);
        set_slot(
            &committed_spec,
            &task,
            "spec:rate-limiting#goal",
            "Cap requests per client.",
        );
        set_slot(
            &committed_spec,
            &task,
            "spec:rate-limiting#context",
            "The gateway is unprotected.",
        );
        let criterion = add_criterion(&committed_spec, &task);
        set_slot(
            &committed_spec,
            &task,
            &format!("{criterion}/statement"),
            "A client over the cap is rejected.",
        );
        committed_spec.finalize(&task, "spec", "spec the rate limiter", false);

        Bases {
            fresh,
            committed_adr,
            committed_spec,
        }
    }

    /// A **copy** of the named base — the per-recipe working corpus.
    fn corpus(&self, base: Base) -> TrialCorpus {
        match base {
            Base::Fresh => self.fresh.copy_state(),
            Base::CommittedAdr => self.committed_adr.copy_state(),
            Base::CommittedSpec => self.committed_spec.copy_state(),
        }
    }
}

// ───────────────────────────── driving helpers ─────────────────────────────

/// Run the verb under test: `jigc --format json <args…>`, raw [`Output`].
fn json(corpus: &TrialCorpus, args: &[&str]) -> Output {
    let mut argv = vec!["--format", "json"];
    argv.extend_from_slice(args);
    corpus.jigc(&argv)
}

/// The same, with `stdin` piped — the shape the prose writers (`doc set-slot`,
/// `doc author`, `config fill`) read their payload through.
fn json_stdin(corpus: &TrialCorpus, args: &[&str], stdin: &str) -> Output {
    let mut argv = vec!["--format", "json"];
    argv.extend_from_slice(args);
    corpus.jigc_stdin(&argv, stdin)
}

/// Set one prose slot (setup, not the verb under test).
fn set_slot(corpus: &TrialCorpus, task: &str, addr: &str, prose: &str) {
    corpus.jigc_stdin_ok(
        &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
        prose,
    );
}

/// Mint `adr:cache-strategy` into `task` (the `single-task` create-gate).
fn create_adr(corpus: &TrialCorpus, task: &str) {
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache strategy",
        "--task",
        task,
    ]);
}

/// Fill every required slot of `adr:cache-strategy`, so a finalize of `task` lands.
fn fill_adr(corpus: &TrialCorpus, task: &str) {
    for (section, prose) in [
        ("context", "Forces around caching."),
        ("decision", "Use a write-through cache."),
        ("consequences", "Colder reads."),
    ] {
        set_slot(
            corpus,
            task,
            &format!("adr:cache-strategy#{section}"),
            prose,
        );
    }
}

/// Mint `spec:rate-limiting` into `task` (the `plan` create-gate).
fn create_spec(corpus: &TrialCorpus, task: &str) {
    corpus.jigc_ok(&[
        "doc",
        "create",
        "spec",
        "--title",
        "Rate limiting",
        "--task",
        task,
    ]);
}

/// Add one `criteria` item and return **the address the binary emitted** — driven
/// verbatim downstream, never a test-side reconstruction of the slug rule.
fn add_criterion(corpus: &TrialCorpus, task: &str) -> String {
    corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            "spec:rate-limiting#criteria",
            "--title",
            "Limits per IP",
            "--task",
            task,
        ])
        .trim_end_matches('\n')
        .to_string()
}

/// Author `task`'s transient `commit` doc so a `task validate` is clean and a
/// `task finalize` has a subject to render (the `trial_corpus.rs` sequence, split so
/// the finalize itself can be the `--format json` invocation under test).
fn author_commit_doc(corpus: &TrialCorpus, task: &str, scope: &str, summary: &str) {
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "feat",
        "--task",
        task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        scope,
        "--task",
        task,
    ]);
    set_slot(corpus, task, &format!("commit:{task}#summary"), summary);
    set_slot(
        corpus,
        task,
        &format!("commit:{task}#body"),
        "Driven by the format-json success-axis sweep.",
    );
}

/// The address of the committed spec's **first** criterion, read back off the binary's
/// own item array (`id`) rather than re-slugged from the title — the item id is minted
/// once and frozen, so reconstructing it test-side would be a second slug rule.
fn first_criterion_address(corpus: &TrialCorpus) -> String {
    let out = json(corpus, &["doc", "show", "spec:rate-limiting#criteria"]);
    let items: Value = serde_json::from_slice(&out.stdout).expect("the item array parses");
    let id = items[0]["id"]
        .as_str()
        .expect("each item object carries its minted `id`");
    format!("spec:rate-limiting#criteria/{id}")
}

/// A live `single-task` task with `adr:cache-strategy` staged — the working area the
/// `doc` write verbs and the `task` lifecycle verbs need.
fn live_adr_task(corpus: &TrialCorpus) -> String {
    let task = corpus.start_workflow("single-task", "harden the cache");
    create_adr(corpus, &task);
    task
}

/// A live `plan` task with `spec:rate-limiting` staged — the repeatable-bearing doc
/// the item verbs address (an `adr` ships no repeatable section).
fn live_spec_task(corpus: &TrialCorpus) -> String {
    let task = corpus.start_workflow("plan", "spec the rate limiter");
    create_spec(corpus, &task);
    task
}

/// A milestone with one sub-task, minted `--workflow sub-task` (the `flow10` shape).
fn milestone_with_subtask(corpus: &TrialCorpus) {
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    corpus.jigc_ok(&[
        "milestone",
        "add-task",
        "cache-rework",
        "Warm the read cache",
        "--workflow",
        "sub-task",
    ]);
}

/// The same, with the sub-area **populated through the real write path**
/// (`doc create adr --task <sub>` + its slots) — what `join` merges and `finalize`
/// commits.
///
/// No `jigc workflow sub-task --task <sub>` re-entry step, deliberately: on a corpus
/// carrying the methodology pack, `milestone create`/`add-task` land their
/// `milestone-record` commits *after* the shared base is pinned, so the main checkout's
/// HEAD is ahead of the pin and a re-entry there is refused by design — a fanned
/// sub-agent re-enters from its **provisioned worktree**, whose HEAD *is* the pin. The
/// join reads the sub-area's staged bodies + provenance, which the `--task`-scoped
/// write verbs (the M8 front door) produce directly.
fn milestone_with_populated_subtask(corpus: &TrialCorpus) {
    milestone_with_subtask(corpus);
    let sub = "warm-the-read-cache";
    create_adr(corpus, sub);
    fill_adr(corpus, sub);
}

/// The `doc author` batch payload — the whole-instance form the verb reads, with the
/// literal `<<…>>` slot markers it requires.
const ADR_PAYLOAD: &str = "\
title: Cache strategy
sections:
  - id: context
    set:
      context: |-
        <<Forces around caching.>>
  - id: decision
    set:
      decision: |-
        <<Use a write-through cache.>>
  - id: consequences
    set:
      consequences: |-
        <<Colder reads.>>
";

/// A minimal **manifest-less** pack shipping one persisted `note` doctype — the
/// genuinely freeze-exempt fixture `jigc relocate` operates on (lifted from
/// `flow40_acceptance.rs`, arm 3), plus a committed instance stranded at a prior home.
fn strand_a_freeze_exempt_note(corpus: &TrialCorpus) {
    let repo = corpus.repo();
    let pack = repo.join(".jigc").join("note-pack");
    fs::create_dir_all(pack.join("schemas")).expect("mk the fixture pack schemas dir");
    fs::write(
        pack.join("schemas").join("note.yaml"),
        "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n",
    )
    .expect("write the note schema");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the note pack");

    let prior = "docs/legacy-notes/cache-benchmarks.md";
    fs::create_dir_all(repo.join("docs").join("legacy-notes")).expect("mk the prior home");
    fs::write(
        repo.join(prior),
        "# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n",
    )
    .expect("write the stranded note");
    corpus.git(&["add", prior]);
    corpus.git(&["commit", "-q", "-m", "strand the note at a legacy home"]);
}

/// A committed foreign document for `jigc migrate` to rewrite.
fn commit_a_foreign_document(corpus: &TrialCorpus) {
    let repo = corpus.repo();
    fs::create_dir_all(repo.join("docs")).expect("mk docs/");
    fs::write(
        repo.join("docs").join("direction.md"),
        "# Product Direction\n\nWe build a deterministic context compiler.\n\n\
         ## Principles\n\nStructure belongs to the CLI; prose belongs to the model.\n",
    )
    .expect("write the foreign source");
    corpus.git(&["add", "docs/direction.md"]);
    corpus.git(&["commit", "-q", "-m", "add the direction doc"]);
}

// ─────────────────────── the driven arms of `ENVELOPE_ARMS` ───────────────────────

/// One registry row's driving recipe: the `(path, arm)` it answers to, the base corpus it
/// needs, and the sequence that ends in the `--format json` invocation whose streams and
/// key set are asserted.
///
/// Keyed on `(path, arm)` rather than on the verb alone, because five verbs answer with
/// more than one key set and clap enumerates **syntax**, never runtime result variants
/// (`completions/artifacts/M51/settle-record.md` → §7).
struct Recipe {
    /// The leaf verb's argv path — empty for the two cross-cutting reject arms.
    path: &'static [&'static str],
    /// The [`EnvelopeArm::arm`] this recipe drives.
    arm: &'static str,
    base: Base,
    /// Drive the corpus to the point the arm is reached, then return the raw [`Output`]
    /// of the invocation itself under `--format json`.
    drive: fn(&TrialCorpus) -> Output,
}

/// **The per-arm driving registry.** Hand-written by necessity — an arm needs real state
/// no walker can invent — and held honest by the `⇔` against [`ENVELOPE_ARMS`], which is
/// what makes it a *covering* set rather than an authoritative list.
fn recipes() -> Vec<Recipe> {
    vec![
        // ── top-level ───────────────────────────────────────────────────────────
        Recipe {
            path: &["start"],
            arm: "OrientationView::UnsetProject",
            base: Base::Fresh,
            // The one state with no project cascade layer: a git repo jigc has not been
            // installed into. Reached by taking the layer back out of a built corpus,
            // rather than by minting a fourth base for one read.
            drive: |c| {
                unset_the_project(c);
                json(c, &["start"])
            },
        },
        Recipe {
            path: &["start"],
            arm: "OrientationView::Clean",
            base: Base::Fresh,
            drive: |c| json(c, &["start"]),
        },
        Recipe {
            path: &["start"],
            arm: "OrientationView::ActiveTask",
            base: Base::Fresh,
            drive: |c| {
                live_adr_task(c);
                json(c, &["start"])
            },
        },
        Recipe {
            path: &["start"],
            arm: "Composed",
            base: Base::Fresh,
            drive: |c| {
                json(
                    c,
                    &["start", "--workflow", "single-task", "harden the cache"],
                )
            },
        },
        Recipe {
            path: &["workflow"],
            arm: "Composed",
            base: Base::Fresh,
            // `--preview` composes without minting; `--task` needs a milestone sub-task.
            drive: |c| json(c, &["workflow", "single-task", "--preview"]),
        },
        Recipe {
            path: &["setup"],
            arm: "Installed",
            base: Base::Fresh,
            // Idempotent — the base was built by `setup`, so this is its clean re-run.
            drive: |c| json(c, &["setup"]),
        },
        Recipe {
            path: &["uninstall"],
            arm: "TornDown",
            base: Base::Fresh,
            drive: |c| json(c, &["uninstall"]),
        },
        Recipe {
            path: &["upgrade"],
            arm: "Swept",
            base: Base::Fresh,
            drive: |c| json(c, &["upgrade"]),
        },
        Recipe {
            path: &["ingest"],
            arm: "Triaged",
            base: Base::Fresh,
            drive: |c| json(c, &["ingest"]),
        },
        Recipe {
            path: &["migrate"],
            arm: "Composed",
            base: Base::Fresh,
            drive: |c| {
                commit_a_foreign_document(c);
                json(c, &["migrate", "docs/direction.md", "--as", "vision"])
            },
        },
        Recipe {
            path: &["migrate-corpus"],
            arm: "Report",
            base: Base::Fresh,
            drive: |c| json(c, &["migrate-corpus", "--dry-run"]),
        },
        Recipe {
            path: &["unmanage"],
            arm: "Report",
            base: Base::CommittedAdr,
            drive: |c| json(c, &["unmanage", "docs/decisions/cache-strategy.md"]),
        },
        Recipe {
            path: &["rename"],
            arm: "Report",
            base: Base::CommittedAdr,
            drive: |c| json(c, &["rename", "adr:cache-strategy", "--to", "Cache policy"]),
        },
        Recipe {
            path: &["relocate"],
            arm: "Report",
            base: Base::Fresh,
            drive: |c| {
                strand_a_freeze_exempt_note(c);
                json(c, &["relocate", "note", "--from", "docs/legacy-notes/"])
            },
        },
        Recipe {
            path: &["describe"],
            arm: "Menu",
            base: Base::Fresh,
            drive: |c| json(c, &["describe"]),
        },
        Recipe {
            path: &["validate"],
            arm: "StoreSweep",
            base: Base::Fresh,
            drive: |c| json(c, &["validate"]),
        },
        // ── doc ─────────────────────────────────────────────────────────────────
        Recipe {
            path: &["doc", "create"],
            arm: "DocAck::Created",
            base: Base::Fresh,
            drive: |c| {
                let task = c.start_workflow("single-task", "harden the cache");
                json(
                    c,
                    &[
                        "doc",
                        "create",
                        "adr",
                        "--title",
                        "Cache strategy",
                        "--task",
                        &task,
                    ],
                )
            },
        },
        Recipe {
            path: &["doc", "add-item"],
            arm: "DocAck::AddedItem",
            base: Base::Fresh,
            drive: |c| {
                let task = live_spec_task(c);
                json(
                    c,
                    &[
                        "doc",
                        "add-item",
                        "spec:rate-limiting#criteria",
                        "--title",
                        "Limits per IP",
                        "--task",
                        &task,
                    ],
                )
            },
        },
        Recipe {
            path: &["doc", "remove-item"],
            arm: "DocAck::RemovedItem",
            base: Base::Fresh,
            drive: |c| {
                let task = live_spec_task(c);
                let item = add_criterion(c, &task);
                json(c, &["doc", "remove-item", &item, "--task", &task])
            },
        },
        Recipe {
            path: &["doc", "retitle-item"],
            arm: "DocAck::RetitledItem",
            base: Base::Fresh,
            drive: |c| {
                let task = live_spec_task(c);
                let item = add_criterion(c, &task);
                json(
                    c,
                    &[
                        "doc",
                        "retitle-item",
                        &item,
                        "--title",
                        "Limits by client",
                        "--task",
                        &task,
                    ],
                )
            },
        },
        Recipe {
            path: &["doc", "rename"],
            arm: "DocAck::Renamed",
            base: Base::Fresh,
            // A doc this task minted has no committed identity, so the rename re-slugs.
            drive: |c| {
                let task = live_adr_task(c);
                json(
                    c,
                    &[
                        "doc",
                        "rename",
                        "adr:cache-strategy",
                        "--to",
                        "Cache policy",
                        "--task",
                        &task,
                    ],
                )
            },
        },
        Recipe {
            path: &["doc", "set-field"],
            arm: "DocAck::Field",
            base: Base::Fresh,
            drive: |c| {
                let task = live_adr_task(c);
                json(
                    c,
                    &[
                        "doc",
                        "set-field",
                        "adr:cache-strategy#status",
                        "--value",
                        "accepted",
                        "--task",
                        &task,
                    ],
                )
            },
        },
        Recipe {
            path: &["doc", "set-field"],
            arm: "DocAck::UnsetField",
            base: Base::Fresh,
            // `--unset` answers with its own ack shape, and only over an **optional**
            // scalar: the required `status` is refused (`write.unset-ineligible`), so the
            // arm is reached at the adr's optional `status/cites-code` leaf.
            drive: |c| {
                let task = live_adr_task(c);
                c.jigc_ok(&[
                    "doc",
                    "set-field",
                    ADR_OPTIONAL_FIELD,
                    "--value",
                    "src/lib.rs#present_symbol",
                    "--task",
                    &task,
                ]);
                json(
                    c,
                    &[
                        "doc",
                        "set-field",
                        ADR_OPTIONAL_FIELD,
                        "--unset",
                        "--task",
                        &task,
                    ],
                )
            },
        },
        Recipe {
            path: &["doc", "set-slot"],
            arm: "DocAck::Slot",
            base: Base::Fresh,
            drive: |c| {
                let task = live_adr_task(c);
                json_stdin(
                    c,
                    &[
                        "doc",
                        "set-slot",
                        "adr:cache-strategy#decision",
                        "--from-file",
                        "-",
                        "--task",
                        &task,
                    ],
                    "Use a write-through cache.\n",
                )
            },
        },
        Recipe {
            path: &["doc", "author"],
            arm: "DocAck::Authored",
            base: Base::Fresh,
            drive: |c| {
                // `author` stands in for `create` here, so this task stages nothing
                // first — the batch verb's own create is the only one on this path.
                let task = c.start_workflow("single-task", "harden the cache");
                json_stdin(
                    c,
                    &["doc", "author", "adr", "--from-file", "-", "--task", &task],
                    ADR_PAYLOAD,
                )
            },
        },
        Recipe {
            path: &["doc", "show"],
            arm: "WholeDoc::Committed",
            base: Base::CommittedAdr,
            drive: |c| json(c, &["doc", "show", "adr:cache-strategy"]),
        },
        Recipe {
            path: &["doc", "show"],
            arm: "WholeDoc::Staged",
            base: Base::Fresh,
            // The staged read (M43): the same parse/slice/render path over the task's own
            // copy, and the envelope names it with one more key.
            drive: |c| {
                let task = live_adr_task(c);
                json(c, &["doc", "show", "adr:cache-strategy", "--task", &task])
            },
        },
        Recipe {
            path: &["doc", "show"],
            arm: "FieldsGroupSlice",
            base: Base::CommittedAdr,
            drive: |c| json(c, &["doc", "show", "adr:cache-strategy#status"]),
        },
        Recipe {
            path: &["doc", "show"],
            arm: "SlotSlice",
            base: Base::CommittedAdr,
            drive: |c| json(c, &["doc", "show", "adr:cache-strategy#decision"]),
        },
        Recipe {
            path: &["doc", "show"],
            arm: "ItemArraySlice",
            base: Base::CommittedSpec,
            // A repeatable section addressed without an item hop. An `adr` ships no
            // repeatable section at all, which is why the item projections run on the
            // committed `spec` the bind/add-from-spec recipes already build.
            drive: |c| json(c, &["doc", "show", "spec:rate-limiting#criteria"]),
        },
        Recipe {
            path: &["doc", "show"],
            arm: "ItemSlice",
            base: Base::CommittedSpec,
            // The item hop is read back off the binary's own item array rather than
            // re-slugged here from the title the base authored — the id is minted and
            // frozen, so a test-side reconstruction would be a second slug rule.
            drive: |c| json(c, &["doc", "show", &first_criterion_address(c)]),
        },
        Recipe {
            path: &["doc", "show"],
            arm: "ListFieldSlice",
            base: Base::Fresh,
            // `adr.supersedes` is the dev pack's `0..*` ref: TWO targets make the stored
            // value a list, which is what projects as an array. The targets dangle on
            // purpose — a dangling ref is adjudicated at finalize, not at this door, and
            // the projection turns on the value's cardinality, never on its resolution.
            drive: |c| {
                let task = live_adr_task(c);
                c.jigc_ok(&[
                    "doc",
                    "set-field",
                    "adr:cache-strategy#supersedes",
                    "--value",
                    "[adr:older-call, adr:second-call]",
                    "--task",
                    &task,
                ]);
                json(
                    c,
                    &[
                        "doc",
                        "show",
                        "adr:cache-strategy#status/supersedes",
                        "--task",
                        &task,
                    ],
                )
            },
        },
        Recipe {
            path: &["doc", "show"],
            arm: "CompoundFieldSlice",
            base: Base::Fresh,
            // `milestone-record.base` is the one compound leaf in the pinned read
            // surface, and `milestone create` commits the record that carries it.
            drive: |c| {
                c.jigc_ok(&["milestone", "create", "Cache rework"]);
                json(
                    c,
                    &["doc", "show", "milestone-record:cache-rework#meta/base"],
                )
            },
        },
        Recipe {
            path: &["doc", "schema"],
            arm: "Projection",
            base: Base::Fresh,
            drive: |c| json(c, &["doc", "schema", "adr"]),
        },
        Recipe {
            path: &["doc", "list"],
            arm: "Index",
            base: Base::CommittedAdr,
            drive: |c| json(c, &["doc", "list"]),
        },
        // ── task ────────────────────────────────────────────────────────────────
        Recipe {
            path: &["task", "list"],
            arm: "Rows",
            base: Base::Fresh,
            // A **populated** list, so the declared row keys are asserted against a real
            // element rather than vacuously against `[]`.
            drive: |c| {
                live_adr_task(c);
                json(c, &["task", "list"])
            },
        },
        Recipe {
            path: &["task", "diff"],
            arm: "Ack",
            base: Base::Fresh,
            drive: |c| {
                let task = live_adr_task(c);
                json(c, &["task", "diff", &task])
            },
        },
        Recipe {
            path: &["task", "validate"],
            arm: "Report",
            base: Base::Fresh,
            drive: |c| {
                // A clean preview needs every required slot filled, or the gate blocks
                // (exit 3) — the same key set, but not this suite's success arm.
                let task = finalize_ready_adr_task(c);
                json(c, &["task", "validate", &task])
            },
        },
        Recipe {
            path: &["task", "amend"],
            arm: "Composed",
            base: Base::Fresh,
            // The fresh base's HEAD is `setup`'s install commit — a single-parent commit, so
            // the door's own head-shape gate passes and it composes.
            drive: |c| json(c, &["task", "amend", "repair the install commit message"]),
        },
        Recipe {
            path: &["task", "discard"],
            arm: "TaskAck::Discarded",
            base: Base::Fresh,
            drive: |c| {
                let task = live_adr_task(c);
                // A live task always stages its `commit:<id>` doc, so the success path
                // of this door runs under the consent (M50 Inc 3 / T2).
                json(c, &["task", "discard", &task, "--force"])
            },
        },
        Recipe {
            path: &["task", "finalize"],
            arm: "Landed",
            base: Base::Fresh,
            drive: |c| {
                let task = finalize_ready_adr_task(c);
                json(c, &["task", "finalize", &task])
            },
        },
        Recipe {
            path: &["task", "finalize"],
            arm: "LandedAmend",
            base: Base::Fresh,
            // The amend commit model: mint through `jigc task amend`, author the same commit
            // doc, and land it. The task id comes off the EMITTED compose envelope rather
            // than being rebuilt from the slug rule in test code.
            drive: |c| {
                let composed = json(c, &["task", "amend", "repair the install message"]);
                let stdout = String::from_utf8(composed.stdout).expect("utf-8 stdout");
                let value: Value = serde_json::from_str(&stdout)
                    .expect("the amend compose envelope is one json document");
                let task = value["task"]
                    .as_str()
                    .expect("the composed envelope names the minted task")
                    .to_owned();
                author_commit_doc(c, &task, "cache", "harden the cache");
                json(c, &["task", "finalize", &task])
            },
        },
        Recipe {
            path: &["task", "finalize"],
            arm: "Forecast",
            base: Base::Fresh,
            drive: |c| {
                let task = finalize_ready_adr_task(c);
                json(c, &["task", "finalize", &task, "--dry-run"])
            },
        },
        Recipe {
            path: &["task", "finalize"],
            arm: "Blocked",
            base: Base::Fresh,
            // The gate ran and refused: an adr whose required slots are empty.
            drive: |c| {
                let task = live_adr_task(c);
                json(c, &["task", "finalize", &task])
            },
        },
        Recipe {
            path: &["task", "finalize"],
            arm: "MigrationReviewHold",
            base: Base::Fresh,
            drive: |c| {
                let task = stage_a_conformant_migration(c);
                json(c, &["task", "finalize", &task])
            },
        },
        Recipe {
            path: &["task", "bind"],
            arm: "TaskAck::Bound",
            base: Base::CommittedSpec,
            drive: |c| {
                let task = c.start_workflow("implement-from-spec", "enforce the rate limit");
                json(c, &["task", "bind", "spec", "spec:rate-limiting", &task])
            },
        },
        // ── config ──────────────────────────────────────────────────────────────
        Recipe {
            path: &["config", "get"],
            arm: "Reading",
            base: Base::Fresh,
            // The read rung over an untouched knob — a `setup` repo resolves every
            // declared knob, so no state has to be built first (`config_read.rs`).
            drive: |c| json(c, &["config", "get", "docs-root"]),
        },
        Recipe {
            path: &["config", "list"],
            arm: "Readings",
            base: Base::Fresh,
            drive: |c| json(c, &["config", "list"]),
        },
        Recipe {
            path: &["config", "set"],
            arm: "ConfigAck::Set",
            base: Base::Fresh,
            drive: |c| json(c, &["config", "set", "invocation-log", "true"]),
        },
        Recipe {
            path: &["config", "insert-step"],
            arm: "ConfigAck::InsertStep",
            base: Base::Fresh,
            drive: |c| {
                write_native_step(c);
                json(
                    c,
                    &[
                        "config",
                        "insert-step",
                        "--workflow",
                        "single-task",
                        "--after",
                        "implement",
                        "./extra.yaml",
                    ],
                )
            },
        },
        Recipe {
            path: &["config", "replace-step"],
            arm: "ConfigAck::ReplaceStep",
            base: Base::Fresh,
            drive: |c| {
                write_native_step(c);
                json(
                    c,
                    &[
                        "config",
                        "replace-step",
                        "workflow:single-task#implement",
                        "./extra.yaml",
                    ],
                )
            },
        },
        Recipe {
            path: &["config", "remove-step"],
            arm: "ConfigAck::RemoveStep",
            base: Base::Fresh,
            drive: |c| {
                json(
                    c,
                    &["config", "remove-step", "workflow:single-task#implement"],
                )
            },
        },
        Recipe {
            path: &["config", "fill"],
            arm: "ConfigAck::Fill",
            base: Base::Fresh,
            drive: |c| {
                json_stdin(
                    c,
                    &[
                        "config",
                        "fill",
                        "step:implement#extra-guidance",
                        "--from-file",
                        "-",
                    ],
                    "House rule: name the axis before the fix.\n",
                )
            },
        },
        Recipe {
            path: &["config", "fork"],
            arm: "ConfigAck::Fork",
            base: Base::Fresh,
            drive: |c| json(c, &["config", "fork", "workflow:single-task#implement"]),
        },
        // ── milestone ───────────────────────────────────────────────────────────
        Recipe {
            path: &["milestone", "create"],
            arm: "RecordOnlyAck",
            base: Base::Fresh,
            drive: |c| json(c, &["milestone", "create", "Cache rework"]),
        },
        Recipe {
            path: &["milestone", "add-task"],
            arm: "RecordOnlyAck",
            base: Base::Fresh,
            drive: |c| {
                c.jigc_ok(&["milestone", "create", "Cache rework"]);
                json(
                    c,
                    &[
                        "milestone",
                        "add-task",
                        "cache-rework",
                        "Warm the read cache",
                    ],
                )
            },
        },
        Recipe {
            path: &["milestone", "add-from-spec"],
            arm: "RecordOnlyAck",
            base: Base::CommittedSpec,
            drive: |c| {
                c.jigc_ok(&["milestone", "create", "Cache rework"]);
                json(
                    c,
                    &[
                        "milestone",
                        "add-from-spec",
                        "cache-rework",
                        "spec:rate-limiting",
                    ],
                )
            },
        },
        Recipe {
            path: &["milestone", "list-tasks"],
            arm: "Listing",
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "list-tasks", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "provision"],
            arm: "RecordOnlyAck",
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "provision", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "execute"],
            arm: "Composed",
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "execute", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "join"],
            arm: "Report",
            base: Base::Fresh,
            drive: |c| {
                milestone_with_populated_subtask(c);
                json(c, &["milestone", "join", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "finalize"],
            arm: "Landed",
            base: Base::Fresh,
            drive: |c| {
                milestone_with_populated_subtask(c);
                json(c, &["milestone", "finalize", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "finalize"],
            arm: "Blocked",
            base: Base::Fresh,
            // A sub-task that staged nothing: the boundary's conformance gate refuses.
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "finalize", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "discard"],
            arm: "RecordOnlyAck",
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "discard", "cache-rework"])
            },
        },
        // ── the two cross-cutting reject arms ───────────────────────────────────
        Recipe {
            path: &[],
            arm: "Reject::Error",
            base: Base::Fresh,
            // An orchestration refusal travelling as a plain `anyhow` — the
            // `render::operational_error` funnel every format-bearing dispatch arm routes
            // through.
            drive: |c| {
                json(
                    c,
                    &["config", "remove-step", "workflow:single-task#not-a-step"],
                )
            },
        },
        Recipe {
            path: &[],
            arm: "Reject::Findings",
            base: Base::Fresh,
            // A blocking finding travelling as the report envelope — the funnel
            // `render::validation` serves over a one-finding `ValidationReport`.
            drive: |c| json(c, &["doc", "show", "adr:nope"]),
        },
    ]
}

/// The source file `config insert-step` / `replace-step` register as a native step
/// (its basename becomes the step id) — the `config_insert_step.rs` idiom.
fn write_native_step(corpus: &TrialCorpus) {
    fs::write(
        corpus.repo().join("extra.yaml"),
        "Run the extra project step before finalizing.\n",
    )
    .expect("write the native step source");
}

/// The adr's one **optional** scalar leaf — what `set-field --unset` needs, the required
/// `status` being refused by `write.unset-ineligible` (`flow42_acceptance.rs` arm V5).
const ADR_OPTIONAL_FIELD: &str = "adr:cache-strategy#status/cites-code";

/// Take the project cascade layer back out of a built corpus, leaving a git repo jigc has
/// never been installed into — the only state `OrientationView::UnsetProject` is reachable
/// from.
fn unset_the_project(corpus: &TrialCorpus) {
    fs::remove_dir_all(corpus.repo().join(".jigc")).expect("remove the project cascade layer");
}

/// A `single-task` task whose adr **and** transient commit doc are complete — what the
/// three finalize arms that are not the block need.
fn finalize_ready_adr_task(corpus: &TrialCorpus) -> String {
    let task = live_adr_task(corpus);
    fill_adr(corpus, &task);
    author_commit_doc(corpus, &task, "cache", "harden the cache");
    task
}

/// Drive a **conformant** `jigc migrate` task to the point its finalize reaches the
/// human review gate: the canonical doc staged and filled, the commit doc authored, and
/// no `--approve`. An unconformant one would block at exit 3 first and never reach the
/// hold (`exit_codes.rs` → 4).
///
/// Returns the migration task's id, read from the **emitted** compose envelope rather
/// than rebuilt from the `migrate-<doctype>-<slug>-<hash>` rule in test code.
fn stage_a_conformant_migration(corpus: &TrialCorpus) -> String {
    commit_a_foreign_document(corpus);
    let composed = json(corpus, &["migrate", "docs/direction.md", "--as", "vision"]);
    let stdout = String::from_utf8(composed.stdout).expect("utf-8 stdout");
    let value: Value =
        serde_json::from_str(&stdout).expect("the migrate compose envelope is one json document");
    let task = value["task"]
        .as_str()
        .expect("the compose envelope names the minted task")
        .to_string();

    // `vision` is a title-fixed singleton, so the create takes the title its schema fixes.
    corpus.jigc_ok(&[
        "doc", "create", "vision", "--title", "Vision", "--task", &task,
    ]);
    for section in ["thesis", "invariants", "open-questions"] {
        set_slot(
            corpus,
            &task,
            &format!("vision:vision#{section}"),
            "Structure belongs to the CLI; prose belongs to the model.",
        );
    }
    author_commit_doc(corpus, &task, "vision", "adopt the direction doc");
    task
}

// ───────────────────────────── the four proofs ─────────────────────────────

/// `(path, arm)` as one comparable key — `""` for the cross-cutting reject arms' empty
/// path, which no leaf owns.
fn key(path: &[&str], arm: &str) -> (String, String) {
    (path.join(" "), arm.to_string())
}

/// Every declared `Object` / `ArrayOf` key set, or `None` for the four shapes that declare
/// none — the document's own data, or a bare scalar, decides those.
fn declared_keys(shape: &ArmShape) -> Option<&'static [&'static str]> {
    match shape {
        ArmShape::Object(keys) | ArmShape::ArrayOf(keys) => Some(keys),
        ArmShape::Scalar
        | ArmShape::DataKeyed
        | ArmShape::ArrayOfDataKeyed
        | ArmShape::ArrayOfScalars => None,
    }
}

/// **Proof 1 — every clap leaf verb has at least one arm.** The registry's verb-owning
/// rows and the clap tree's leaves are the same set: no verb unrepresented, no row naming
/// a path the tree does not have. A `>= 47` floor sits beneath it so a *simultaneous*
/// delete on both sides stays visible instead of shrinking the axis in silence.
///
/// This is what keeps the registry's declared bound survivable: arm completeness is
/// bounded by driving, so a verb may be missing an *arm* — it may never be missing a *row*.
#[test]
fn every_clap_leaf_verb_has_at_least_one_envelope_arm() {
    let mut from_clap = leaf_verb_paths();
    from_clap.sort();

    assert!(
        from_clap.len() >= 47,
        "the clap tree must still enumerate the whole verb surface (>= 47 leaf verbs); \
         got {}: {from_clap:?}",
        from_clap.len(),
    );

    let rowed: BTreeSet<Vec<String>> = ENVELOPE_ARMS
        .iter()
        .filter(|arm| !arm.path.is_empty())
        .map(|arm| arm.path.iter().map(|s| s.to_string()).collect())
        .collect();

    let missing: Vec<_> = from_clap.iter().filter(|v| !rowed.contains(*v)).collect();
    let stale: Vec<_> = rowed.iter().filter(|v| !from_clap.contains(v)).collect();
    assert!(
        missing.is_empty() && stale.is_empty(),
        "every clap leaf verb owns at least one `ENVELOPE_ARMS` row, and no row names a \
         path the tree does not have.\n\
         verbs with no arm (declare what their `--format json` answers with): {missing:?}\n\
         rows naming no verb (delete them): {stale:?}",
    );
}

/// **Proof 2 — every production arm has exactly one registry row.**
///
///   * `(path, arm)` is **unique**, so "at least one" from proof 1 plus this is "exactly
///     one" for every arm the registry knows;
///   * each production result enum's rows are **exactly** that enum's own arm-name table
///     ([`DOC_ACK_ARMS`] and its three siblings) — the table a new variant cannot skip,
///     because [`doc_ack_arm`]'s match is exhaustive and its names are read out of a
///     fixed-length array;
///   * [`ConfigAck::ALL`]'s shipped **witnesses** map through [`config_ack_arm`] onto
///     exactly the `config` rows, each row's leaf verb matching the witness's own — so the
///     six config rows are fenced from both the variant side and the verb side;
///   * each of the other three enums' arm functions is **exercised**, not merely compiled:
///     one witness per enum maps through it and its answer must be a real row, so the
///     mapper that carries the compile-time exhaustiveness is a live seam;
///   * every [`ArmOrigin::Dispatch`] row carries the **stated reason** the Settle owes for
///     an arm no enum can generate.
#[test]
fn every_production_arm_has_exactly_one_registry_row() {
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
    for arm in ENVELOPE_ARMS {
        let k = key(arm.path, arm.arm);
        assert!(
            seen.insert(k.clone()),
            "`(path, arm)` must be unique — a duplicated row would let one arm's \
             declaration stand in for another's; got `{}` / `{}` twice",
            k.0,
            k.1,
        );
    }

    for (of, table) in [
        ("DocAck", &DOC_ACK_ARMS[..]),
        ("TaskAck", &TASK_ACK_ARMS[..]),
        ("ConfigAck", &CONFIG_ACK_ARMS[..]),
        ("OrientationView", &ORIENTATION_ARMS[..]),
    ] {
        let rows: BTreeSet<&str> = ENVELOPE_ARMS
            .iter()
            .filter(|arm| matches!(arm.origin, ArmOrigin::Variant { of: named } if named == of))
            .map(|arm| arm.arm)
            .collect();
        let declared: BTreeSet<&str> = table.iter().copied().collect();
        assert_eq!(
            rows, declared,
            "the `{of}` rows of `ENVELOPE_ARMS` must be exactly that enum's arm-name \
             table — a variant with no row would ship an undeclared envelope, and a row \
             with no variant would declare one the binary cannot emit",
        );
    }

    // The three enums that ship no witness table: one variant each, mapped through the
    // production arm function, so the seam whose exhaustive match is proof 2's compile-time
    // half is driven rather than only compiled.
    let mapped = [
        (
            "OrientationView",
            orientation_arm(&OrientationView::unset_project()),
        ),
        (
            "DocAck",
            doc_ack_arm(&DocAck::Authored {
                address: "adr:cache-strategy".to_string(),
                target: AckTarget {
                    doctype: "adr".to_string(),
                    slug: "cache-strategy".to_string(),
                    section: None,
                    item: None,
                    leaf: None,
                },
                findings: Findings::from(Vec::new()),
            }),
        ),
        (
            "TaskAck",
            task_ack_arm(&TaskAck::Discarded {
                task: "harden-the-cache".to_string(),
                dropped: Vec::new(),
                commit: None,
            }),
        ),
    ];
    for (of, arm) in mapped {
        assert!(
            ENVELOPE_ARMS.iter().any(|row| row.arm == arm),
            "`{of}`'s arm function answered `{arm}`, which no registry row declares",
        );
    }

    for witness in ConfigAck::ALL {
        let ack = (witness.witness)();
        let arm = config_ack_arm(&ack);
        let row = ENVELOPE_ARMS
            .iter()
            .find(|row| row.arm == arm)
            .unwrap_or_else(|| panic!("`{arm}` (the `config {}` ack) has no row", witness.verb));
        assert_eq!(
            row.path.last().copied(),
            Some(witness.verb),
            "`{arm}`'s row must sit under the verb whose ack it is (`config {}`)",
            witness.verb,
        );
    }

    for arm in ENVELOPE_ARMS {
        let label = format!("{} / {}", arm.path.join(" "), arm.arm);
        let siblings = ENVELOPE_ARMS
            .iter()
            .filter(|other| other.path == arm.path)
            .count();
        match arm.origin {
            ArmOrigin::Variant { .. } => {}
            ArmOrigin::Sole => assert_eq!(
                siblings, 1,
                "`{label}` is declared `Sole` — the verb's several run-modes move values, \
                 not keys — so its path may own no other row; it owns {siblings}",
            ),
            ArmOrigin::Dispatch(reason) => {
                assert!(
                    !reason.trim().is_empty(),
                    "the hand-enumerated row `{label}` must state WHY no enum can generate it",
                );
                assert!(
                    siblings > 1,
                    "`{label}` is declared `Dispatch` — a branch chooses among several arms \
                     — so its path must own more than one row; it owns {siblings}. A verb \
                     with one arm is `Sole`.",
                );
            }
        }
    }
}

/// **Proof 3 — every registry row is driven.** The driving [`recipes`] and
/// [`ENVELOPE_ARMS`] are the same set on `(path, arm)`: a declared arm nothing drives is a
/// claim with no witness, and a recipe naming no row drives something the contract does
/// not declare.
#[test]
fn every_registry_row_is_driven() {
    let declared: BTreeSet<(String, String)> = ENVELOPE_ARMS
        .iter()
        .map(|arm| key(arm.path, arm.arm))
        .collect();
    let driven: BTreeSet<(String, String)> = recipes()
        .iter()
        .map(|recipe| key(recipe.path, recipe.arm))
        .collect();

    let undriven: Vec<_> = declared.difference(&driven).collect();
    let undeclared: Vec<_> = driven.difference(&declared).collect();
    assert!(
        undriven.is_empty() && undeclared.is_empty(),
        "the registry and its driving recipes are one set.\n\
         rows nothing drives (write the recipe): {undriven:?}\n\
         recipes naming no row (declare the arm, or delete the recipe): {undeclared:?}",
    );
}

/// **Proof 4 — the driven key set equals the declared key set**, swept over every row
/// through the real binary, together with the stream discipline each [`ArmOutcome`] names:
///
///   * an [`ArmOutcome::Success`] arm exits **0** and writes exactly one JSON document to
///     **stdout**, with no document on stderr;
///   * an [`ArmOutcome::Adjudicated`] arm does the same at its declared non-zero exit —
///     the gate ran and reported, so the document still rides stdout;
///   * an [`ArmOutcome::Reject`] arm leaves **stdout empty** and writes exactly one
///     document to **stderr**, at a non-zero exit (the door decides which, so the row
///     names the stream and not a code — `machine_output.rs`'s discrimination predicate).
///
/// Then the shape: an `Object` row's top-level key set is **exactly** its declaration, an
/// `ArrayOf` row is a non-empty array whose every element carries exactly the declared row
/// keys, a `Scalar` row is neither object nor array, and a `DataKeyed` row is a non-empty
/// object whose keys are the document's own (the shape is pinned; the key set is not,
/// which is the row's whole point).
#[test]
fn the_driven_key_set_equals_the_declared_key_set() {
    let bases = Bases::build();
    let rows: Vec<&EnvelopeArm> = ENVELOPE_ARMS.iter().collect();

    for recipe in recipes() {
        let label = if recipe.path.is_empty() {
            recipe.arm.to_string()
        } else {
            format!("{} / {}", recipe.path.join(" "), recipe.arm)
        };
        let row = rows
            .iter()
            .find(|row| key(row.path, row.arm) == key(recipe.path, recipe.arm))
            .unwrap_or_else(|| panic!("`{label}` has no registry row (proof 3 covers this)"));

        let corpus = bases.corpus(recipe.base);
        let out = (recipe.drive)(&corpus);
        let code = out.status.code().unwrap_or(-1);
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

        // The stream discipline the row's outcome names.
        let document = match row.outcome {
            ArmOutcome::Success | ArmOutcome::Adjudicated(_) => {
                let expected = match row.outcome {
                    ArmOutcome::Adjudicated(exit) => exit,
                    _ => EXIT_SUCCESS as i32,
                };
                assert_eq!(
                    code, expected,
                    "`{label}` must be driven to its declared exit {expected}; got {code}\n\
                     stdout:\n{stdout}\nstderr:\n{stderr}",
                );
                assert!(
                    is_one_json_doc(&stdout),
                    "`{label}` must write EXACTLY ONE JSON document to stdout — plain text, \
                     a mixed stream, or silence all fail here (the `task diff` defect M47 \
                     Inc 7 fixed was exactly this).\nstdout:\n{stdout}",
                );
                assert!(
                    !is_one_json_doc(&stderr),
                    "`{label}` must leave stderr free of a JSON document — the agent-text \
                     side channel is never a second envelope.\nstderr:\n{stderr}",
                );
                stdout
            }
            ArmOutcome::Reject => {
                assert_ne!(
                    code, EXIT_SUCCESS as i32,
                    "`{label}` is a reject arm and must not exit 0\nstdout:\n{stdout}\n\
                     stderr:\n{stderr}",
                );
                assert!(
                    stdout.is_empty(),
                    "`{label}` is a reject: stdout stays empty and the document rides \
                     stderr.\nstdout:\n{stdout}",
                );
                assert!(
                    is_one_json_doc(&stderr),
                    "`{label}` must write EXACTLY ONE JSON document to stderr\n\
                     stderr:\n{stderr}",
                );
                stderr
            }
        };

        let value: Value = serde_json::from_str(&document).expect("the document parses");
        match row.shape {
            ArmShape::Object(expected) => {
                let object = value
                    .as_object()
                    .unwrap_or_else(|| panic!("`{label}` declares an object; got:\n{document}"));
                let driven: Vec<&str> = object.keys().map(String::as_str).collect();
                let mut driven_sorted = driven.clone();
                driven_sorted.sort_unstable();
                assert_eq!(
                    driven_sorted,
                    expected.to_vec(),
                    "`{label}`'s DRIVEN top-level key set must equal its DECLARED one. An \
                     undeclared key on a pinned envelope is a defect, not an addition, \
                     whichever wave mints it — and a declared key the binary does not emit \
                     is a promise nothing keeps.\ngot:\n{document}",
                );
            }
            ArmShape::ArrayOf(expected) => {
                let array = value
                    .as_array()
                    .unwrap_or_else(|| panic!("`{label}` declares an array; got:\n{document}"));
                assert!(
                    !array.is_empty(),
                    "`{label}`'s recipe must drive a POPULATED array — an empty one asserts \
                     the row keys vacuously.\ngot:\n{document}",
                );
                for element in array {
                    let object = element.as_object().unwrap_or_else(|| {
                        panic!("`{label}`'s elements are objects; got:\n{document}")
                    });
                    let mut driven: Vec<&str> = object.keys().map(String::as_str).collect();
                    driven.sort_unstable();
                    assert_eq!(
                        driven,
                        expected.to_vec(),
                        "`{label}`'s DRIVEN row keys must equal its DECLARED ones\n\
                         got:\n{document}",
                    );
                }
            }
            ArmShape::Scalar => {
                assert!(
                    !value.is_object() && !value.is_array(),
                    "`{label}` declares a bare scalar — no keys at all; got:\n{document}",
                );
            }
            ArmShape::DataKeyed => {
                let object = value.as_object().unwrap_or_else(|| {
                    panic!("`{label}` declares a data-keyed object; got:\n{document}")
                });
                assert!(
                    !object.is_empty(),
                    "`{label}`'s recipe must reach a POPULATED map, or the shape is \
                     unproven; got:\n{document}",
                );
            }
            ArmShape::ArrayOfDataKeyed => {
                let array = value.as_array().unwrap_or_else(|| {
                    panic!("`{label}` declares an array of data-keyed objects; got:\n{document}")
                });
                assert!(
                    !array.is_empty(),
                    "`{label}`'s recipe must drive a POPULATED array — an empty one proves \
                     nothing about its elements.\ngot:\n{document}",
                );
                for element in array {
                    let object = element.as_object().unwrap_or_else(|| {
                        panic!("`{label}`'s elements are objects; got:\n{document}")
                    });
                    assert!(
                        !object.is_empty(),
                        "`{label}`'s elements must be POPULATED maps, or the shape is \
                         unproven; got:\n{document}",
                    );
                }
            }
            ArmShape::ArrayOfScalars => {
                let array = value.as_array().unwrap_or_else(|| {
                    panic!("`{label}` declares an array of scalars; got:\n{document}")
                });
                assert!(
                    !array.is_empty(),
                    "`{label}`'s recipe must drive a POPULATED array — an empty one asserts \
                     the element shape vacuously.\ngot:\n{document}",
                );
                for element in array {
                    assert!(
                        !element.is_object() && !element.is_array(),
                        "`{label}`'s elements are bare scalars — no keys at all; \
                         got:\n{document}",
                    );
                }
            }
        }
    }
}

/// **The `schema_version` partition, fenced against the registry** (D5): the result
/// contract's version integer rides an arm **iff** that arm's root is a value the result
/// contract versions. Typed result values carry it; ad-hoc `json!` envelopes do not, and
/// the *"add it to ~40 envelopes"* reading is refused on the record as gold-plating.
///
/// Its teeth come from proof 4: the declared key set **is** the driven one, so the key
/// cannot appear on an ad-hoc envelope without someone re-classifying that arm's
/// [`ArmRoot`] — which means naming the production type the contract supposedly versions.
///
/// A non-object root is a second, stronger arm of the same partition: an array, a scalar
/// and a data-keyed map cannot carry a top-level contract key at all, so each must be
/// declared off the result contract. `jigc task list`'s array is the case that matters —
/// `doc-read-surface.md`'s *"a top-level key on **every** result envelope"* is false there
/// by construction, not by omission.
///
/// The hyphenated `schema-version` / `contract-version` are **not** this key: they are the
/// addressed document's own stamp and `doc schema`'s separately-versioned projection.
#[test]
fn schema_version_rides_exactly_the_result_contract_rooted_arms() {
    const KEY: &str = "schema_version";
    for arm in ENVELOPE_ARMS {
        let label = format!("{} / {}", arm.path.join(" "), arm.arm);
        let rooted = matches!(arm.root, ArmRoot::ResultContract(_));
        match declared_keys(&arm.shape) {
            Some(keys) if matches!(arm.shape, ArmShape::Object(_)) => {
                assert_eq!(
                    keys.contains(&KEY),
                    rooted,
                    "`{label}`: `{KEY}` rides an envelope IFF the result contract versions \
                     its root. Declared keys: {keys:?}",
                );
            }
            _ => assert!(
                !rooted,
                "`{label}` has a non-object root, which cannot carry a top-level contract \
                 key at all — so it must be declared off the result contract",
            ),
        }
    }
}

// ─────────────────── the registry's prose against the driven registry ───────────────────

/// One **shape claim** `crates/cli/src/render.rs` states in its own prose, and the
/// predicate over the registry that decides whether the claim is still true.
///
/// The rule is one-directional and that is what makes it usable for both halves: *if the
/// phrase is in the source, its predicate must hold*. A claim the registry has outgrown is
/// satisfied by **deleting the sentence** — which is why the retired phrasings stay in the
/// table rather than leaving it, so a later editor cannot reintroduce one; and a claim that
/// is true today is **live-fenced** by the same entry, so the next wave that moves the
/// registry reddens here instead of leaving a stale sentence behind.
struct ShapeClaim {
    /// The sentence as the source spells it, written flat: the scan strips `///` / `//!`
    /// and joins lines with a single space, so a rewrap or a `\` string continuation
    /// cannot hide the claim.
    phrase: &'static str,
    /// Whether the module states this sentence today.
    standing: Standing,
    /// What the phrase asserts, for the failure message.
    asserts: &'static str,
    /// Whether the registry still bears it out.
    holds: fn() -> bool,
}

/// Whether a [`ShapeClaim`]'s sentence is one the module states today.
enum Standing {
    /// Stated today: the scan must **find** it, and its predicate must hold. Without this
    /// direction a wording drift would make the entry silently inert — a fence that is
    /// satisfied by nothing is the shape this whole test exists to catch.
    Stated,
    /// Stated once and rewritten. Kept in the table so it cannot come back while the
    /// registry falsifies it; nothing is asserted about its presence beyond that.
    Retired,
}

/// Every [`ArmShape`] member, as one comparable kind name — an **exhaustive** match, so a
/// seventh shape cannot compile until someone says what it is and the coverage leg below
/// starts demanding a row for it.
fn shape_kind(shape: &ArmShape) -> &'static str {
    match shape {
        ArmShape::Object(_) => "Object",
        ArmShape::ArrayOf(_) => "ArrayOf",
        ArmShape::Scalar => "Scalar",
        ArmShape::DataKeyed => "DataKeyed",
        ArmShape::ArrayOfDataKeyed => "ArrayOfDataKeyed",
        ArmShape::ArrayOfScalars => "ArrayOfScalars",
    }
}

/// The shape vocabulary itself — one witness per member, which is what gives the coverage
/// leg and the member **count** something to read. Constructed rather than listed as names,
/// so it cannot drift from the enum.
const SHAPE_VOCABULARY: &[ArmShape] = &[
    ArmShape::Object(&[]),
    ArmShape::ArrayOf(&[]),
    ArmShape::Scalar,
    ArmShape::DataKeyed,
    ArmShape::ArrayOfDataKeyed,
    ArmShape::ArrayOfScalars,
];

/// How many rows carry an **array**-rooted shape.
fn array_rooted_rows() -> usize {
    ENVELOPE_ARMS
        .iter()
        .filter(|row| {
            matches!(
                row.shape,
                ArmShape::ArrayOf(_) | ArmShape::ArrayOfDataKeyed | ArmShape::ArrayOfScalars
            )
        })
        .count()
}

/// The `jigc doc show` rows, and how many of them are **not** object-rooted.
fn doc_show_rows() -> (usize, usize) {
    let rows: Vec<&EnvelopeArm> = ENVELOPE_ARMS
        .iter()
        .filter(|row| row.path == ["doc", "show"])
        .collect();
    let non_object = rows
        .iter()
        .filter(|row| !matches!(row.shape, ArmShape::Object(_)))
        .count();
    (rows.len(), non_object)
}

/// Whether a **store-scope condition** can drive a row the registry declares
/// [`ArmOutcome::Success`] to a non-zero exit — the falsifier of an unqualified
/// *"`Success` means exit 0"*. `jigc validate`'s `StoreSweep` is the row it bites:
/// [`cli::render::STORE_EXIT_FLIPS`] is exactly the membership that flips that door.
fn a_success_row_flips_its_exit_on_a_store_condition() -> bool {
    !STORE_EXIT_FLIPS.is_empty()
        && ENVELOPE_ARMS
            .iter()
            .any(|row| row.path == ["validate"] && matches!(row.outcome, ArmOutcome::Success))
}

/// **The shape claims the registry's own prose makes**, each with the predicate that
/// decides it. The **first two entries are the sentences M52 Increment 1 / T4 rewrote**,
/// both falsified by driving `jigc doc show` over a stock corpus
/// (`completions/artifacts/M52/baseline-contracts.md` §2.3: 17 addressable cells
/// collapsing to 7 distinct root shapes, against the four rows the registry then carried).
///
/// **Declared bound.** This fences the claims it enumerates, not every sentence someone
/// could write — `implementation/pinning.md` §3 refuses a prose parser by name, and a
/// predicate has to be *computable from the registry* to be worth writing. What it buys is
/// that a claim once found false cannot return, and that the replacement is not itself a
/// free-floating assertion.
const SHAPE_CLAIMS: &[ShapeClaim] = &[
    // ── the two rewritten sentences, as retired phrasings ──
    ShapeClaim {
        phrase: "`jigc task list` is the surface's one array",
        standing: Standing::Retired,
        asserts: "the whole surface answers with exactly ONE array-rooted document",
        holds: || array_rooted_rows() == 1,
    },
    ShapeClaim {
        phrase: "whole doc, field group, slot — and two of the four are not objects at all",
        standing: Standing::Retired,
        asserts: "`jigc doc show` owns FOUR arms, TWO of them non-object-rooted",
        holds: || doc_show_rows() == (4, 2),
    },
    // ── the two further retired phrasings the same drive falsified ──
    ShapeClaim {
        phrase: "Four, not one, because three arms carry no top-level key list at all",
        standing: Standing::Retired,
        asserts: "`ArmShape` has FOUR members, THREE of them carrying no key list",
        holds: || {
            SHAPE_VOCABULARY.len() == 4
                && SHAPE_VOCABULARY
                    .iter()
                    .filter(|shape| declared_keys(shape).is_none())
                    .count()
                    == 3
        },
    },
    ShapeClaim {
        phrase: "Exit 0, the document on **stdout**, no JSON on stderr.",
        standing: Standing::Retired,
        asserts: "a `Success` arm exits 0 in EVERY state, not merely in the one it is driven in",
        holds: || !a_success_row_flips_its_exit_on_a_store_condition(),
    },
    // ── the replacements, live-fenced by the same rule ──
    ShapeClaim {
        phrase: "`jigc task list` is the one array whose element keys are declarable",
        standing: Standing::Stated,
        asserts: "exactly one row declares an array with a row-key set",
        holds: || {
            ENVELOPE_ARMS
                .iter()
                .filter(|row| matches!(row.shape, ArmShape::ArrayOf(_)))
                .count()
                == 1
        },
    },
    ShapeClaim {
        phrase: "`jigc doc show` is the only verb that answers with an array of data-keyed objects",
        standing: Standing::Stated,
        asserts: "every `ArrayOfDataKeyed` row sits under `doc show`",
        holds: || {
            ENVELOPE_ARMS
                .iter()
                .filter(|row| matches!(row.shape, ArmShape::ArrayOfDataKeyed))
                .all(|row| row.path == ["doc", "show"])
        },
    },
    ShapeClaim {
        phrase: "Six members, because four of them carry no declarable top-level key list at all",
        standing: Standing::Stated,
        asserts: "`ArmShape` has SIX members, FOUR of them carrying no key list",
        holds: || {
            SHAPE_VOCABULARY.len() == 6
                && SHAPE_VOCABULARY
                    .iter()
                    .filter(|shape| declared_keys(shape).is_none())
                    .count()
                    == 4
        },
    },
    ShapeClaim {
        phrase: "Two projections of `jigc doc show` are this shape",
        standing: Standing::Stated,
        asserts: "exactly two rows declare `DataKeyed`, both under `doc show`",
        holds: || {
            let rows: Vec<&EnvelopeArm> = ENVELOPE_ARMS
                .iter()
                .filter(|row| matches!(row.shape, ArmShape::DataKeyed))
                .collect();
            rows.len() == 2 && rows.iter().all(|row| row.path == ["doc", "show"])
        },
    },
    ShapeClaim {
        phrase: "a store-scope condition drives `jigc validate`'s own `StoreSweep` arm to exit 1",
        standing: Standing::Stated,
        asserts: "some `STORE_EXIT_FLIPS` member flips a row the registry declares `Success`",
        holds: a_success_row_flips_its_exit_on_a_store_condition,
    },
];

/// `crates/cli/src/render.rs` folded to one line per statement: `///` / `//!` markers
/// stripped, a `\` string continuation's break closed, and every line joined by a single
/// space — so a claim wrapped across three lines reads as the sentence it is.
fn folded_render_source() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("render.rs");
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {} — the registry's own source: {e}", path.display()));
    let mut folded = String::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        let body = trimmed
            .strip_prefix("///")
            .or_else(|| trimmed.strip_prefix("//!"))
            .unwrap_or(trimmed)
            .trim();
        folded.push_str(body.strip_suffix('\\').unwrap_or(body).trim_end());
        folded.push(' ');
    }
    folded
}

/// **No shape claim the registry states survives the driven registry falsifying it** — and
/// every shape the registry declares is one some row actually answers with.
///
/// **Why this exists.** The registry's shapes were declared once and then the surface was
/// driven: `jigc doc show` answers with **seven** distinct root shapes over a stock corpus
/// against the **four** rows the registry carried, and two of the sentences explaining the
/// shape space were false in the direction that matters — `ArmShape::ArrayOf`'s *"`jigc
/// task list` is the surface's one array"* (falsified by an array of item objects **and**
/// an array of strings) and the four `doc show` rows' shared dispatch reason *"whole doc,
/// field group, slot — and two of the four are not objects at all"* (falsified by the
/// projection space being larger than the enumeration). A registry whose prose is wrong
/// about its own shapes is the surface-tier defect this wave exists to close, one layer in.
///
/// **Leg 1 — coverage.** A shape the registry *declares* and no row *claims* is a shape the
/// binary does not answer with: the vocabulary is not a wish list.
///
/// **Leg 2 — the claims.** [`SHAPE_CLAIMS`], under its stated bound.
#[test]
fn no_shape_claim_in_the_registry_survives_the_registry_falsifying_it() {
    let claimed: BTreeSet<&str> = ENVELOPE_ARMS
        .iter()
        .map(|row| shape_kind(&row.shape))
        .collect();
    let unclaimed: Vec<&str> = SHAPE_VOCABULARY
        .iter()
        .map(shape_kind)
        .filter(|kind| !claimed.contains(kind))
        .collect();
    assert!(
        unclaimed.is_empty(),
        "every `ArmShape` member must be the declared shape of at least one row — a shape \
         nothing answers with is a claim about the surface with no witness: {unclaimed:?}",
    );

    let source = folded_render_source();
    for claim in SHAPE_CLAIMS {
        let present = source.contains(claim.phrase);
        if present {
            assert!(
                (claim.holds)(),
                "`crates/cli/src/render.rs` still states:\n  \"{}\"\n\
                 which asserts: {}\n\
                 — and the registry falsifies it. Rewrite the sentence (and leave the old \
                 wording in `SHAPE_CLAIMS` marked `Retired`, so it cannot come back).",
                claim.phrase,
                claim.asserts,
            );
        }
        if matches!(claim.standing, Standing::Stated) {
            assert!(
                present,
                "`SHAPE_CLAIMS` marks this sentence as one the module states today:\n  \
                 \"{}\"\n\
                 and the scan cannot find it in `crates/cli/src/render.rs`. Either the \
                 wording drifted — in which case the entry is now fencing nothing — or the \
                 sentence was deleted and its entry should be marked `Retired`.",
                claim.phrase,
            );
        }
    }
}

/// The declarations' own shape, which proofs 2 and 4 read as given: each declared key set
/// is **sorted and duplicate-free** (so the comparison is against a set, not an order),
/// and an [`ArmStatus::Unpinned`] row's `still_pinned` names keys the row actually
/// declares — an unpinned envelope may keep a key another contract pins, but it may not
/// promise one it does not emit.
#[test]
fn every_declaration_is_well_formed() {
    for arm in ENVELOPE_ARMS {
        let label = format!("{} / {}", arm.path.join(" "), arm.arm);
        if let Some(keys) = declared_keys(&arm.shape) {
            let mut sorted = keys.to_vec();
            sorted.sort_unstable();
            assert_eq!(
                sorted,
                keys.to_vec(),
                "`{label}`'s declared key set must be written sorted",
            );
            let unique: BTreeSet<&str> = keys.iter().copied().collect();
            assert_eq!(
                unique.len(),
                keys.len(),
                "`{label}`'s declared key set must be duplicate-free",
            );
        }
        if let ArmStatus::Unpinned { still_pinned, .. } = arm.status {
            let declared = declared_keys(&arm.shape).unwrap_or(&[]);
            for key in still_pinned {
                assert!(
                    declared.contains(key),
                    "`{label}` names `{key}` as still pinned, but does not declare it",
                );
            }
        }
    }
}

/// **The two `jigc task discard` cells, driven apart** (M51 Increment 9 / T8) — the door
/// is `COMMITTING_DOORS`' tenth member, and until now its success line named no sha while
/// every sibling committing door printed one (`jigc setup` → *install commit*,
/// `jigc milestone create` → *record commit*, `jigc migrate-corpus` → *committed <sha>*).
///
/// A **sub-task** discard settles the milestone's committed record
/// ([`cli::milestone::settle_discarded_sub_task`], M49 Increment 2) — a record-only commit
/// that moves `HEAD` on the operator's behalf. An **ordinary** task discard commits
/// nothing. Both exit 0 and both printed the identical line, so the surface could not be
/// read to tell which happened.
///
/// Driven rather than rendered, because the fact is git's: the sha the ack names must be a
/// commit `git rev-parse` resolves **and** the `HEAD` the door actually moved to. A witness
/// test can assert the value is carried; only the binary can assert it is *true*.
///
/// The ordinary cell asserts the **negative** at the same strength: `commit` is `null` (the
/// honest absent this envelope family already uses for a sha that does not exist —
/// `setup`'s `install_commit`, `migrate-corpus`'s own `commit`), the text names no sha, and
/// `HEAD` is byte-identical to what it was before the discard.
#[test]
fn task_discard_names_the_commit_it_landed() {
    let fresh = TrialCorpus::build(State::Fresh);

    // ── the sub-task cell: a record-only commit lands, and the ack names it ──
    let sub = fresh.copy_state();
    milestone_with_subtask(&sub);
    let before = sub.git(&["rev-parse", "HEAD"]);
    let text = sub.jigc_ok(&["task", "discard", "warm-the-read-cache", "--force"]);
    let after = sub.git(&["rev-parse", "HEAD"]);
    assert_ne!(
        before, after,
        "a sub-task discard commits the settled milestone record, so HEAD moves",
    );
    let short = sub.git(&["rev-parse", "--short", "HEAD"]);
    assert!(
        text.contains(&short),
        "the sub-task discard's agent text must name the commit it landed (`{short}`) — a \
         committing door that prints no sha leaves the operator to find it in a later `git \
         log`.\ntext:\n{text}",
    );

    let sub_json = fresh.copy_state();
    milestone_with_subtask(&sub_json);
    let before = sub_json.git(&["rev-parse", "HEAD"]);
    let document = sub_json.jigc_ok(&[
        "task",
        "discard",
        "warm-the-read-cache",
        "--force",
        "--format",
        "json",
    ]);
    let value: Value = serde_json::from_str(&document).expect("the ack parses");
    let sha = value["commit"].as_str().unwrap_or_else(|| {
        panic!("the sub-task discard's envelope must carry the landed sha:\n{document}")
    });
    // `git rev-parse <sha>^{commit}` IS the resolution: `TrialCorpus::git` asserts git
    // exited 0, so a sha git cannot resolve to a commit fails right here.
    let peeled = format!("{sha}^{{commit}}");
    let resolved = sub_json.git(&["rev-parse", &peeled]);
    assert_eq!(
        resolved,
        sub_json.git(&["rev-parse", "HEAD"]),
        "`commit` must be the HEAD the door moved to, not some earlier commit",
    );
    assert_ne!(
        before,
        sub_json.git(&["rev-parse", "HEAD"]),
        "the sub-task discard moved HEAD",
    );

    // ── the ordinary cell: nothing commits, and the ack says so ──
    let plain = fresh.copy_state();
    let task = live_adr_task(&plain);
    let before = plain.git(&["rev-parse", "HEAD"]);
    let document = plain.jigc_ok(&["task", "discard", &task, "--force", "--format", "json"]);
    let value: Value = serde_json::from_str(&document).expect("the ack parses");
    assert_eq!(
        value["commit"],
        Value::Null,
        "an ordinary task discard commits nothing, so `commit` is the honest `null` — never \
         a sha, and never a key a driver has to know is missing:\n{document}",
    );
    assert_eq!(
        before,
        plain.git(&["rev-parse", "HEAD"]),
        "an ordinary task discard is workbench-local: HEAD does not move",
    );

    let plain_text = fresh.copy_state();
    let task = live_adr_task(&plain_text);
    let text = plain_text.jigc_ok(&["task", "discard", &task, "--force"]);
    assert!(
        !text.contains("record commit"),
        "an ordinary task discard landed no commit, so its text names none. (The bare word \
         `commit` is not the probe: a dropped `commit:<task>` doc is a doctype identity, not \
         a sha.)\n{text}",
    );
}
