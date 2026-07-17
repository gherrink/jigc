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
