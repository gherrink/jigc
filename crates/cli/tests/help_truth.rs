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
