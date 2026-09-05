//! M47 Increment 7 / T4 — **the success-path half of stream discipline, swept over
//! the whole verb axis** ([command-output-contract.md](../../../design/command-output-contract.md)
//! → Stream discipline; `implementation/pinning.md` §2).
//!
//! The claim: **every leaf verb honours `--format json` on its SUCCESS path** — exit
//! 0, stdout parses as *exactly one* JSON document, stderr carries *no* JSON document.
//! Its sibling `machine_output.rs` sweeps the same axis on the **reject** surface
//! (fixture-free, where no verb can succeed) and pins one real success arm; this suite
//! is the missing half — it drives **all 46** leaf verbs to a genuine success.
//!
//! **Why the sweep, and not the structural fence.** `adapter.rs`'s
//! `format_is_a_global_arg` asserts that `--format` *is* a global clap arg, which is a
//! necessary condition and nothing more: it stays green if every dispatch arm parses
//! the flag and then ignores it. That is not hypothetical — the M47 baseline found
//! exactly one arm doing precisely that (`TaskCommand::Diff` dropped `format` on the
//! floor and printed plain text at exit 0), and the structural fence never saw it.
//! Behaviour is the only thing that can fence behaviour.
//!
//! **The enumeration is from the clap tree; the recipes are hand-written; the
//! BIJECTION is what stops the recipes being an authoritative hand list.** Leaf verbs
//! walk out of `Cli::command()` (the `leaf_verb_paths()` idiom `machine_output.rs`
//! established), and [`recipes`] must cover that set **exactly** — no missing member,
//! no stale extra. A verb added anywhere in the tree therefore reddens this suite
//! until someone drives it to a real success, and a verb deleted reddens it until the
//! dead recipe goes. The `>= 46` floor sits beneath the bijection so a *simultaneous*
//! delete-and-drop stays visible rather than shrinking both sides in silence.
//!
//! **Every recipe has a shipped precedent** — the driving sequence is lifted from the
//! verb's own suite (`flow9_seam.rs` / `flow10_acceptance.rs` for the milestone arc,
//! `config_*.rs` for the cascade-authoring verbs, `implement_from_spec.rs` for the
//! bind, `flow40_acceptance.rs` for the freeze-exempt relocation, `trial_corpus.rs`
//! for setup/author/finalize) rather than invented here.
//!
//! **Three base corpora, copied per recipe.** Most verbs run on a plain `jigc setup`
//! repo; a read/rename/unmanage verb needs a *committed* doc, and `task bind` /
//! `milestone add-from-spec` need a *committed spec with criteria*. Each base is built
//! once and [`TrialCorpus::copy_state`]'d per recipe, so a mutating verb never sees a
//! sibling's leftovers.

use crate::support;

use clap::CommandFactory;
use cli::cli::Cli;
use cli::task::EXIT_SUCCESS;
use serde_json::Value;
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

// ──────────────────────────────── the registry ────────────────────────────────

/// One leaf verb's success recipe: the argv path it answers to, the base corpus it
/// needs, and the driving sequence that ends in the `--format json` invocation whose
/// streams are asserted.
struct Recipe {
    /// The leaf verb's argv path — matched against the clap tree by the bijection.
    path: &'static [&'static str],
    base: Base,
    /// Drive the corpus to the point the verb succeeds, then return the raw [`Output`]
    /// of the verb itself under `--format json`.
    drive: fn(&TrialCorpus) -> Output,
}

/// **The per-verb success-invocation registry.** Hand-written by necessity — a success
/// needs real state no walker can invent — and held honest by the bijection against the
/// clap tree, which is what makes it a *covering* set rather than an authoritative list.
fn recipes() -> Vec<Recipe> {
    vec![
        // ── top-level ───────────────────────────────────────────────────────────
        Recipe {
            path: &["start"],
            base: Base::Fresh,
            // Bare `start` orients (read-only) — the one success a fresh repo has.
            drive: |c| json(c, &["start"]),
        },
        Recipe {
            path: &["workflow"],
            base: Base::Fresh,
            // `--preview` composes without minting; `--task` needs a milestone sub-task.
            drive: |c| json(c, &["workflow", "single-task", "--preview"]),
        },
        Recipe {
            path: &["setup"],
            base: Base::Fresh,
            // Idempotent — the base was built by `setup`, so this is its clean re-run.
            drive: |c| json(c, &["setup"]),
        },
        Recipe {
            path: &["uninstall"],
            base: Base::Fresh,
            drive: |c| json(c, &["uninstall"]),
        },
        Recipe {
            path: &["upgrade"],
            base: Base::Fresh,
            drive: |c| json(c, &["upgrade"]),
        },
        Recipe {
            path: &["ingest"],
            base: Base::Fresh,
            drive: |c| json(c, &["ingest"]),
        },
        Recipe {
            path: &["migrate"],
            base: Base::Fresh,
            drive: |c| {
                commit_a_foreign_document(c);
                json(c, &["migrate", "docs/direction.md", "--as", "vision"])
            },
        },
        Recipe {
            path: &["migrate-corpus"],
            base: Base::Fresh,
            drive: |c| json(c, &["migrate-corpus", "--dry-run"]),
        },
        Recipe {
            path: &["unmanage"],
            base: Base::CommittedAdr,
            drive: |c| json(c, &["unmanage", "docs/decisions/cache-strategy.md"]),
        },
        Recipe {
            path: &["rename"],
            base: Base::CommittedAdr,
            drive: |c| json(c, &["rename", "adr:cache-strategy", "--to", "Cache policy"]),
        },
        Recipe {
            path: &["relocate"],
            base: Base::Fresh,
            drive: |c| {
                strand_a_freeze_exempt_note(c);
                json(c, &["relocate", "note", "--from", "docs/legacy-notes/"])
            },
        },
        Recipe {
            path: &["describe"],
            base: Base::Fresh,
            drive: |c| json(c, &["describe"]),
        },
        Recipe {
            path: &["validate"],
            base: Base::Fresh,
            drive: |c| json(c, &["validate"]),
        },
        // ── doc ─────────────────────────────────────────────────────────────────
        Recipe {
            path: &["doc", "create"],
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
            base: Base::Fresh,
            drive: |c| {
                let task = live_spec_task(c);
                let item = add_criterion(c, &task);
                json(c, &["doc", "remove-item", &item, "--task", &task])
            },
        },
        Recipe {
            path: &["doc", "retitle-item"],
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
            path: &["doc", "set-slot"],
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
            base: Base::CommittedAdr,
            drive: |c| json(c, &["doc", "show", "adr:cache-strategy"]),
        },
        Recipe {
            path: &["doc", "schema"],
            base: Base::Fresh,
            drive: |c| json(c, &["doc", "schema", "adr"]),
        },
        Recipe {
            path: &["doc", "list"],
            base: Base::CommittedAdr,
            drive: |c| json(c, &["doc", "list"]),
        },
        // ── task ────────────────────────────────────────────────────────────────
        Recipe {
            path: &["task", "list"],
            base: Base::Fresh,
            drive: |c| {
                live_adr_task(c);
                json(c, &["task", "list"])
            },
        },
        Recipe {
            path: &["task", "diff"],
            base: Base::Fresh,
            drive: |c| {
                let task = live_adr_task(c);
                json(c, &["task", "diff", &task])
            },
        },
        Recipe {
            path: &["task", "validate"],
            base: Base::Fresh,
            drive: |c| {
                // A clean preview needs every required slot filled, or the gate blocks
                // (exit 3) and this is no longer a success-path arm.
                let task = live_adr_task(c);
                fill_adr(c, &task);
                author_commit_doc(c, &task, "cache", "harden the cache");
                json(c, &["task", "validate", &task])
            },
        },
        Recipe {
            path: &["task", "discard"],
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
            base: Base::Fresh,
            drive: |c| {
                let task = live_adr_task(c);
                fill_adr(c, &task);
                author_commit_doc(c, &task, "cache", "harden the cache");
                json(c, &["task", "finalize", &task])
            },
        },
        Recipe {
            path: &["task", "bind"],
            base: Base::CommittedSpec,
            drive: |c| {
                let task = c.start_workflow("implement-from-spec", "enforce the rate limit");
                json(c, &["task", "bind", "spec", "spec:rate-limiting", &task])
            },
        },
        // ── config ──────────────────────────────────────────────────────────────
        Recipe {
            path: &["config", "get"],
            base: Base::Fresh,
            // The read rung over an untouched knob — a `setup` repo resolves every
            // declared knob, so no state has to be built first (`config_read.rs`).
            drive: |c| json(c, &["config", "get", "docs-root"]),
        },
        Recipe {
            path: &["config", "list"],
            base: Base::Fresh,
            drive: |c| json(c, &["config", "list"]),
        },
        Recipe {
            path: &["config", "set"],
            base: Base::Fresh,
            drive: |c| json(c, &["config", "set", "invocation-log", "true"]),
        },
        Recipe {
            path: &["config", "insert-step"],
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
            base: Base::Fresh,
            drive: |c| json(c, &["config", "fork", "workflow:single-task#implement"]),
        },
        // ── milestone ───────────────────────────────────────────────────────────
        Recipe {
            path: &["milestone", "create"],
            base: Base::Fresh,
            drive: |c| json(c, &["milestone", "create", "Cache rework"]),
        },
        Recipe {
            path: &["milestone", "add-task"],
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
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "list-tasks", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "provision"],
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "provision", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "execute"],
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "execute", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "join"],
            base: Base::Fresh,
            drive: |c| {
                milestone_with_populated_subtask(c);
                json(c, &["milestone", "join", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "finalize"],
            base: Base::Fresh,
            drive: |c| {
                milestone_with_populated_subtask(c);
                json(c, &["milestone", "finalize", "cache-rework"])
            },
        },
        Recipe {
            path: &["milestone", "discard"],
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "discard", "cache-rework"])
            },
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

// ───────────────────────────────── the sweep ─────────────────────────────────

/// **The bijection.** The hand-written [`recipes`] set and the clap tree's leaf verbs
/// are the *same* set — no verb unswept, no recipe stale — with a `>= 46` floor beneath
/// it so a simultaneous delete on both sides stays visible instead of silently
/// shrinking the axis.
///
/// This is what demotes the registry from an authoritative hand list to a covering one:
/// a verb added anywhere in the tree reddens here until it is driven to a real success.
#[test]
fn the_success_recipes_are_a_bijection_with_the_clap_leaf_verbs() {
    let mut from_clap = leaf_verb_paths();
    from_clap.sort();
    let mut from_recipes: Vec<Vec<String>> = recipes()
        .iter()
        .map(|r| r.path.iter().map(|s| s.to_string()).collect())
        .collect();
    from_recipes.sort();

    assert!(
        from_clap.len() >= 46,
        "the clap tree must still enumerate the whole verb surface (>= 46 leaf verbs); \
         got {}: {from_clap:?}",
        from_clap.len(),
    );

    let missing: Vec<_> = from_clap
        .iter()
        .filter(|v| !from_recipes.contains(v))
        .collect();
    let stale: Vec<_> = from_recipes
        .iter()
        .filter(|v| !from_clap.contains(v))
        .collect();
    assert!(
        missing.is_empty() && stale.is_empty(),
        "the success-invocation registry must be a BIJECTION with the clap leaf verbs.\n\
         verbs with no recipe (drive them to a real success): {missing:?}\n\
         recipes naming no verb (delete them): {stale:?}",
    );
    assert_eq!(
        from_recipes.len(),
        from_clap.len(),
        "one recipe per leaf verb — a duplicated path would hide an unswept verb",
    );
}

/// **The success-path sweep.** Every leaf verb, driven to a genuine success in a
/// corpus that makes it succeed, honours `--format json`:
///
///   * **exit 0** — the recipe really did drive a success (a blocked or errored verb
///     would be sweeping the *reject* surface `machine_output.rs` already owns);
///   * **stdout parses as exactly one JSON document** — the adjudication envelope, and
///     nothing else on that stream (the failure this suite exists to catch is a verb
///     printing plain text, or nothing at all, at exit 0);
///   * **stderr carries no JSON document** — the agent-text side channel never becomes
///     a second envelope a driver might parse.
#[test]
fn every_leaf_verb_honours_format_json_on_its_success_path() {
    let bases = Bases::build();

    for recipe in recipes() {
        let label = recipe.path.join(" ");
        let corpus = bases.corpus(recipe.base);
        let out = (recipe.drive)(&corpus);

        let code = out.status.code().unwrap_or(-1);
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

        assert_eq!(
            code, EXIT_SUCCESS as i32,
            "`jigc --format json {label}` must be driven to a SUCCESS by its recipe \
             (this suite sweeps the success path); got exit {code}\n\
             stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert!(
            is_one_json_doc(&stdout),
            "`jigc --format json {label}` must write EXACTLY ONE JSON document to stdout \
             on its success path — plain text, a mixed stream, or silence all fail here \
             (the `task diff` defect M47 Inc 7 fixed was exactly this).\nstdout:\n{stdout}",
        );
        assert!(
            !is_one_json_doc(&stderr),
            "`jigc --format json {label}` must leave stderr free of a JSON document — the \
             agent-text side channel is never a second envelope.\nstderr:\n{stderr}",
        );
    }
}
