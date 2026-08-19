//! M42 Increment 12, T1 — the two help texts stop lying.
//!
//! `jigc start --help` described the **pre-M2** surface (*"an `<intent>` mints a
//! task and composes the default workflow"*), which the router default made false:
//! the shipped `default-workflow` is the `router` (`creates-task: false`), so a
//! bare `<intent>` mints nothing — minting rides `--workflow <X>` on a
//! `creates-task: true` `<X>`. `jigc doc create --help` advertised a
//! `--<id-source>` form that **does not exist** — the flag is always literally
//! `--title` (`design/write-commands.md` → The argument convention: "the help is
//! what is wrong, not the convention").
//!
//! M43 Increment 8, T2 (B8) — help verbs lead with the one-line common case
//! (`design/surface-contract.md` → the style guide): the `jigc doc --help`
//! subcommand table carries each verb's one-line lead, never a multi-clause
//! contract wall or a `design/` path — contract detail lives below the fold, in
//! the verb's own long help. The top-level `jigc --help` `doc` line stops
//! claiming a write-only surface (`show`/`schema`/`list` read).
//!
//! These drive the REAL binary and assert on the **emitted help bytes** an agent
//! reads — not a reconstructed equivalent.

use std::process::Command;

/// Run `jigc <args>` and return its stdout with whitespace runs collapsed (clap
/// wraps at the terminal width, so a phrase assertion must be wrap-insensitive).
/// Panics with both streams on a non-zero exit; `--help` needs no repo/pack.
fn help_stdout(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .output()
        .expect("spawn the jigc binary");
    assert!(
        out.status.success(),
        "`jigc {args:?}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    stdout.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The `about` blurb — everything clap prints before the `Usage:` line. This is
/// the sentence an agent reads first, and the one that carried the falsehood.
fn about(help: &str) -> String {
    let end = help
        .find("Usage:")
        .expect("help must carry a `Usage:` line");
    help[..end].trim().to_string()
}

#[test]
fn start_help_states_the_router_default_not_the_pre_m2_mint() {
    let help = help_stdout(&["start", "--help"]);
    let about = about(&help);

    assert!(
        !help.contains("mints a task and composes the default workflow"),
        "`start --help` must not repeat the pre-M2 claim; got:\n{help}"
    );
    for truth in ["router", "creates-task", "--workflow"] {
        assert!(
            about.contains(truth),
            "`start --help`'s about must name `{truth}` (the router default and \
             the `--workflow` minting form); got:\n{about}"
        );
    }
}

/// Run `jigc <args>` and return raw stdout (line structure preserved — the
/// subcommand-table tests parse per-line). Panics with both streams on a
/// non-zero exit; `--help` needs no repo/pack.
fn help_stdout_raw(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .output()
        .expect("spawn the jigc binary");
    assert!(
        out.status.success(),
        "`jigc {args:?}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// Parse a clap help's subcommand table into `(verb, collapsed description)`
/// rows: the lines between `Commands:` and the next blank line. An entry line is
/// two-space-indented (`  create   Mint …`); deeper-indented lines continue the
/// previous entry's description (clap wraps / verbatim comments span lines).
fn commands_table(help: &str) -> Vec<(String, String)> {
    let mut rows: Vec<(String, String)> = Vec::new();
    let mut in_table = false;
    for line in help.lines() {
        if line.trim_end() == "Commands:" {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        if line.trim().is_empty() {
            break;
        }
        let is_entry = line.starts_with("  ") && !line.starts_with("   ");
        if is_entry {
            let mut words = line.split_whitespace();
            let verb = words.next().expect("an entry line names its verb");
            rows.push((verb.to_string(), words.collect::<Vec<_>>().join(" ")));
        } else {
            let (_, desc) = rows
                .last_mut()
                .expect("a continuation line follows an entry line");
            desc.push(' ');
            desc.push_str(&line.split_whitespace().collect::<Vec<_>>().join(" "));
        }
    }
    assert!(
        !rows.is_empty(),
        "the help must carry a `Commands:` table; got:\n{help}"
    );
    rows
}

/// A one-line lead: a single short clause, not a multi-clause contract wall.
/// 100 collapsed chars is the mold — every restructured lead sits well under it,
/// every pre-B8 wall well over.
const LEAD_CAP: usize = 100;

#[test]
fn doc_subcommand_table_carries_one_line_leads_and_no_design_paths() {
    let help = help_stdout_raw(&["doc", "--help"]);
    let rows = commands_table(&help);

    for verb in [
        "create",
        "add-item",
        "remove-item",
        "retitle-item",
        "set-field",
        "set-slot",
        "author",
        "show",
        "schema",
        "list",
    ] {
        assert!(
            rows.iter().any(|(v, _)| v == verb),
            "the `doc` table must list `{verb}`; got:\n{help}"
        );
    }
    for (verb, desc) in &rows {
        assert!(
            !desc.contains("design/"),
            "the `doc` table line for `{verb}` must not carry a design-doc path \
             (below the fold, in long help); got: {desc}"
        );
        assert!(
            desc.chars().count() <= LEAD_CAP,
            "the `doc` table line for `{verb}` must be its one-line lead \
             (<= {LEAD_CAP} chars collapsed), not a contract wall; got {} chars: {desc}",
            desc.chars().count()
        );
    }
}

#[test]
fn doc_long_help_retains_contract_detail_below_the_fold() {
    // The table drops the contract walls; the verb's own long help keeps them —
    // one representative per read surface: the pinned-shape pointer.
    for verb in ["show", "schema", "list"] {
        let help = help_stdout(&["doc", verb, "--help"]);
        assert!(
            help.contains("design/doc-read-surface.md"),
            "`doc {verb} --help` long help must retain the contract pointer \
             `design/doc-read-surface.md`; got:\n{help}"
        );
    }
}

#[test]
fn top_level_doc_line_names_the_read_half() {
    let help = help_stdout_raw(&["--help"]);
    let rows = commands_table(&help);
    let (_, desc) = rows
        .iter()
        .find(|(v, _)| v == "doc")
        .expect("`jigc --help` lists `doc`");

    for half in ["Read", "write"] {
        assert!(
            desc.contains(half),
            "the top-level `doc` line must name `{half}` (the surface reads AND \
             writes since M39); got: {desc}"
        );
    }
    assert!(
        desc.chars().count() <= LEAD_CAP,
        "the top-level `doc` line must be a one-line lead; got {} chars: {desc}",
        desc.chars().count()
    );
}

/// M43 pre-trial surface polish A3 (help half), **revised at M45 inc-2 T6** —
/// `doc set-slot --help` is address-independent: clap renders help before any
/// address exists, so no target's reserved depth is knowable there. It states
/// the schema-relative *rule* and names no depth — the one carve-out the M45
/// stated-at revision takes consciously (`design/surface-contract.md` → The
/// stated-at fence, the `set-slot --help` carve-out). The assertion therefore
/// relaxes off byte-equality against the engine statement (which now exists
/// only per-address) while still fencing presence: the rule's invariant
/// fragments must be there, and no fixed depth may be blessed as safe.
#[test]
fn doc_set_slot_help_states_the_schema_relative_rule_without_a_depth() {
    let help = help_stdout(&["doc", "set-slot", "--help"]);
    for fragment in ["schema-relative", "Setext", "shallowest depth free"] {
        assert!(
            help.contains(fragment),
            "`doc set-slot --help` must state the schema-relative ceiling rule \
             (missing `{fragment}`); got:\n{help}"
        );
    }
    assert!(
        !help.contains("depth or deeper"),
        "`doc set-slot --help` knows no address, so it must bless no specific \
         depth; got:\n{help}"
    );
}

/// M43 pre-trial surface polish A4 — `doc author --help` relates the batch verb to
/// the incremental ones. **Restated at M47 Inc 11 T1**: the ordering *hazard* it used
/// to state (*"never after them — a doc already staged by `create` rejects the second
/// create"*) was refuted live (RC-alpha4 B2) — since M45 fork 2's copy-on-write a
/// `create` over the task's staged copy acks the copy-in — so the help now states the
/// relation plus what the repeat actually does. The *absence* of any rejection claim
/// is swept across help and every composed workflow in `batch_author_rerun.rs`; this
/// pins the positive half, beside the other help-text truths.
#[test]
fn doc_author_help_relates_the_batch_verb_to_the_incremental_ones() {
    let help = help_stdout(&["doc", "author", "--help"]);
    for fragment in [
        "instead of those verbs",
        "already existed — copied in for update",
    ] {
        assert!(
            help.contains(fragment),
            "`doc author --help` must state `{fragment}`; got:\n{help}"
        );
    }
}

/// M45 inc-10 T11 — `doc author --help`'s payload example shows a section's
/// **own** slot keyed by the section id (`set: {<section-id>: <<…>>}`), the form
/// the prior example never made visible (it only showed a field/slot named
/// arbitrarily within a section; findings §72). The canonical confirmed form is
/// the flow map keyed by the section's own id.
#[test]
fn doc_author_help_shows_the_section_own_slot_form() {
    let help = help_stdout(&["doc", "author", "--help"]);
    assert!(
        help.contains("set: {<section-id>: <<…>>}"),
        "`doc author --help` must show the section-own-slot payload form \
         `set: {{<section-id>: <<…>>}}`; got:\n{help}"
    );
}

/// M43 pre-trial surface polish A7 — `doc create --help` states the slug mint
/// rule at the `--title` it binds to. **M47 Inc 10 T8 (B7):** it stated the two
/// caps and nothing else, hand-typed, so it described a rule the mint does not
/// implement (the renormalization and the edge-stopword drop produce every
/// reported "except when it isn't"). It now renders the same seam-generated
/// `slug::mint_statement` the `add-item` help and the `{{schema:}}` projection do,
/// so the assertion is the whole statement, not a substring of it — all three
/// mint sites state one rule and none can drift from it.
#[test]
fn doc_create_help_states_the_slug_caps() {
    let help = help_stdout(&["doc", "create", "--help"]);
    let statement = engine::slug::mint_statement("the doc id");
    assert!(
        help.contains(&statement),
        "`doc create --help` must carry the seam-generated slug mint statement \
         (`{statement}`) at the title it binds to; got:\n{help}"
    );
}

/// M45 T9 — `add-item` mints an item's `{#id}` anchor from `--title` the same
/// way `create` mints a doc id, so its soliciting surface (the second one, after
/// the doc-level `{{schema:}}` skeleton) must state the slug mint caps too. The
/// statement is the **seam-generated** `mint_statement` output, so the numbers
/// can never drift from what the mint enforces.
#[test]
fn doc_add_item_help_states_the_slug_caps() {
    let help = help_stdout(&["doc", "add-item", "--help"]);
    let statement = engine::slug::mint_statement("the item `{#id}` anchor");
    assert!(
        help.contains(&statement),
        "`doc add-item --help` must carry the seam-generated slug mint-cap \
         statement (`{statement}`) at the `--title` it binds to; got:\n{help}"
    );
}

#[test]
fn doc_create_help_names_title_literally_and_drops_the_phantom_form() {
    let help = help_stdout(&["doc", "create", "--help"]);

    assert!(
        !help.contains("--<id-source>"),
        "`doc create --help` must not advertise the non-existent `--<id-source>` \
         form; got:\n{help}"
    );
    assert!(
        about(&help).contains("--title"),
        "`doc create --help`'s about must name `--title` literally; got:\n{help}"
    );
}

/// M47 Increment 10, T7 (N15) — **no help text names a Rust path.** Six `--task`
/// doc-comments read *"(see `Create::task`)"*, a `clap::Subcommand` variant path
/// that renders verbatim into `jigc doc <verb> --help`: an agent is pointed at a
/// symbol that exists only in this crate's source, so the pointer answers nothing
/// and the surface reads as a leaked internal.
///
/// Swept over **every leaf verb of the real clap tree**, not the six sites — the
/// enumeration is the axis (`pinning.md` §1), so a seventh leak arrives red.
#[test]
fn no_verb_help_carries_a_rust_path() {
    let mut leaked = Vec::new();
    for path in leaf_verb_paths() {
        let mut args: Vec<&str> = path.iter().map(String::as_str).collect();
        args.push("--help");
        let help = help_stdout(&args);
        for token in help.split_whitespace() {
            if token.contains("::") {
                leaked.push(format!("`jigc {} --help` prints `{token}`", path.join(" ")));
            }
        }
    }
    assert!(
        leaked.is_empty(),
        "a help text names a `::`-qualified Rust path — a symbol that exists only in \
         this crate's source, so the reader cannot follow it:\n  {}",
        leaked.join("\n  "),
    );
}

/// Every leaf verb path of the real clap tree (`jigc doc set-field` → `["doc",
/// "set-field"]`) — the `leaf_verb_paths()` idiom the machine-output sweep uses,
/// so a verb added to the tree joins this sweep with no edit here.
fn leaf_verb_paths() -> Vec<Vec<String>> {
    use clap::CommandFactory;
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
    walk(&cli::cli::Cli::command(), Vec::new(), &mut out);
    out
}

/// M47 Increment 10, T7 (B1 / P3-2) — `jigc doc author --help` states the **whole**
/// write contract over a committed doc, not the append half alone. The batch verb's
/// own grammar surface is where an agent reads what `author` does before running it;
/// at HEAD it said only *"a committed doc is fine: `author` copies it in and updates
/// it"*, which reads as "re-authoring is safe" — and a payload item the doc already
/// holds rejects the whole payload.
#[test]
fn doc_author_help_states_the_three_way_write_contract() {
    let help = help_stdout(&["doc", "author", "--help"]);
    for fact in [
        "appended",
        "in place",
        "write.already-present",
        "whole payload",
    ] {
        assert!(
            help.contains(fact),
            "`doc author --help` must state \"{fact}\" — the three-way contract over a \
             COMMITTED doc (a new item appends · an existing leaf overwrites in place · \
             a colliding item rejects the whole payload); got:\n{help}"
        );
    }
}

/// M48 Increment 2, T2 — the closed set's **fourth** member, stated at **both** minting
/// verbs before the write. A create/author over a doc the task already holds does not
/// rewrite its `# H1`, and one that mints a different id is a second document — two
/// refusals an agent otherwise meets only after the fact
/// (`design/surface-contract.md` → law 3, the ambush class;
/// `design/write-commands.md` → The four-way write over a committed doc).
///
/// The two helps are fenced by the **same** token set, which is what keeps the two
/// surfaces (a generated `long_about` on `create`, a doc comment on `author`) from
/// drifting into two different rules.
#[test]
fn both_minting_verbs_state_the_title_contract_before_the_write() {
    for verb in ["create", "author"] {
        let help = help_stdout(&["doc", verb, "--help"]);
        for fact in [
            "write.title-ignored",
            "write.identity-change",
            "jigc doc rename",
            "second document",
        ] {
            assert!(
                help.contains(fact),
                "`doc {verb} --help` must state \"{fact}\" — the title contract (a title \
                 that would be silently dropped is refused · one that mints a different \
                 identity is a second document, not a correction · both route at the \
                 in-task title change); got:\n{help}"
            );
        }
    }
}

/// M46 Increment 8, T2 (B2-3) — `jigc milestone finalize --help` stops contradicting
/// itself. Its `about` enumerated only what the join produces (*"materialize its
/// suffix-resolved **doc bodies** … and commit **them**"*), while `--carry-staged`,
/// three lines below in the SAME help output, states *"the aggregate commit is built
/// from the sub-task worktrees"* — so one screen described a docs-only boundary and a
/// code-carrying one (RC-1.0-gate findings-verification §4; the boundary really lands
/// both halves, pinned by
/// `milestone_finalize_squash_true_genuine_reentry_materializes_transient_then_lands_aggregate`,
/// which asserts the aggregate commit's tree carries two worktrees' code beside the
/// merged docs). Law 1: the `about` names the fold it performs.
///
/// The fence is the **agreement**, not the presence — the `about` and the flag below it
/// must name the same subject (`sub-task worktree`), so the one help output cannot drift
/// back into describing two different boundaries.
#[test]
fn milestone_finalize_about_names_the_code_fold_beside_the_doc_bodies() {
    let help = help_stdout(&["milestone", "finalize", "--help"]);
    let about = about(&help);

    for half in ["doc bodies", "code", "sub-task worktree"] {
        assert!(
            about.contains(half),
            "`milestone finalize --help`'s about must name `{half}` — the boundary \
             commits the join's doc bodies AND the code staged in each sub-task \
             worktree, and the about enumerated only the docs; got:\n{about}"
        );
    }
    let flag = help
        .split_once("--carry-staged")
        .expect("`milestone finalize --help` carries the `--carry-staged` flag")
        .1;
    assert!(
        flag.contains("sub-task worktree"),
        "`--carry-staged`'s help states the boundary is built from the sub-task \
         worktrees; the about above it must name that same subject, or the one help \
         output describes two boundaries again; got:\n{help}"
    );
}
