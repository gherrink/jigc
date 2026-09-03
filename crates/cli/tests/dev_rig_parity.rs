//! The parity fence over `dev/jigc-rig` — the bash front door onto the fixture
//! builder the Rust suites already use.
//!
//! `crates/cli/tests/support/trial_corpus.rs` builds named corpus states by driving
//! the real binary, and `dev/jigc-rig` builds the *same* named states for a shell
//! probe. Two builders of one state set is exactly the shape this repo calls a second
//! source of truth, so the set is fenced: the rig's accepted states and
//! [`State::ALL`] must be the **same set**, and a state added to one and not the other
//! reddens the gate rather than being discovered by a probe that silently cannot
//! build it.
//!
//! The fence compares *sets*, not construction. Construction parity is a property of
//! the rig's own code (it drives the same verbs in the same order) and is not
//! mechanically checkable from here — asserting it would mean building six corpora
//! inside the gate, which is the ~38 s of serial subprocess time pinning.md §4
//! already refuses to pay twice.
//!
//! The second arm fences the rule the rig exists to retire. A teardown of the shape
//! `rm -rf $V/$D` is refused by a static scan *before it runs* — so it prompts a human
//! and parks a delegated agent, leaving no transcript. Every rig root comes from
//! `mktemp -d`, so there is nothing to tear down; this arm makes that a property of
//! `dev/` rather than a comment inside one script.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support;
use support::trial_corpus::State;

/// The repo root — two levels up from `crates/cli`.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/cli has a repo root two levels up")
        .to_path_buf()
}

fn dev_dir() -> PathBuf {
    repo_root().join("dev")
}

/// The states `dev/jigc-rig` accepts, read from the rig itself.
///
/// `--list-states` is asked rather than the script parsed: the rig validates that
/// every listed state has a builder before printing, so a state declared without a
/// construction path exits non-zero here instead of listing as buildable.
fn rig_states() -> BTreeSet<String> {
    let rig = dev_dir().join("jigc-rig");
    assert!(
        rig.exists(),
        "dev/jigc-rig must exist — it is the shell front door onto the fixture \
         builder this fence pairs with ({})",
        rig.display(),
    );
    let out = Command::new(&rig)
        .arg("--list-states")
        .current_dir(repo_root())
        .output()
        .expect("spawn dev/jigc-rig --list-states");
    assert!(
        out.status.success(),
        "dev/jigc-rig --list-states failed ({}):\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 --list-states stdout")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

#[test]
fn the_rig_builds_exactly_the_states_the_fixture_builder_declares() {
    let declared: BTreeSet<String> = State::ALL.iter().map(|s| s.name().to_string()).collect();
    let accepted = rig_states();

    let missing_from_rig: Vec<&String> = declared.difference(&accepted).collect();
    let missing_from_rust: Vec<&String> = accepted.difference(&declared).collect();

    assert!(
        missing_from_rig.is_empty() && missing_from_rust.is_empty(),
        "`dev/jigc-rig` and `State::ALL` must name the SAME set of corpus states — \
         one builder for the Rust suites, one for a shell probe, one set.\n  \
         declared in trial_corpus.rs but not buildable by the rig: {missing_from_rig:?}\n  \
         accepted by the rig but not declared in trial_corpus.rs: {missing_from_rust:?}",
    );
    assert!(
        !declared.is_empty(),
        "the comparison is only a fence while the declared set is non-empty",
    );
}

/// Every file under `dev/`, recursively.
fn dev_files() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in
            std::fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        {
            let entry = entry.expect("read a dev/ entry");
            let path = entry.path();
            if entry.file_type().expect("stat a dev/ entry").is_dir() {
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&dev_dir(), &mut out);
    out.sort();
    out
}

/// Does this line carry a *recursive* removal whose subject is a shell variable?
///
/// Two shapes, both refused before they run by the static scan the rule exists for:
/// `rm` with a recursive flag, and `find … -delete` / `find … -exec rm`. The subject
/// is "variable" when any token after the verb interpolates (`$X`, `${X}`, `"$X/y"`).
fn recursive_removal_of_a_variable_path(line: &str) -> bool {
    let code = match line.split_once('#') {
        // A `#` inside a quoted string is not a comment, so only strip a comment that
        // starts the line or follows whitespace — the conservative read keeps the
        // scan from being talked out of a real command by a trailing quote.
        Some((before, _)) if !before.contains('\'') && !before.contains('"') => before,
        _ => line,
    };
    let tokens: Vec<&str> = code.split_whitespace().collect();
    for (i, token) in tokens.iter().enumerate() {
        let rest = &tokens[i + 1..];
        let interpolates = rest.iter().any(|t| t.contains('$'));
        if !interpolates {
            continue;
        }
        if *token == "rm" || token.ends_with("/rm") {
            let recursive = rest.iter().take_while(|t| t.starts_with('-')).any(|t| {
                *t == "--recursive"
                    || (t.starts_with('-')
                        && !t.starts_with("--")
                        && (t.contains('r') || t.contains('R')))
            });
            if recursive {
                return true;
            }
        }
        if *token == "find" && rest.iter().any(|t| *t == "-delete" || *t == "-exec") {
            return true;
        }
    }
    false
}

#[test]
fn no_dev_script_removes_a_variable_path_recursively() {
    let mut offenders = Vec::new();
    for path in dev_files() {
        let Ok(body) = std::fs::read_to_string(&path) else {
            continue; // a binary file carries no shell command
        };
        for (n, line) in body.lines().enumerate() {
            if recursive_removal_of_a_variable_path(line) {
                offenders.push(format!("{}:{}: {}", path.display(), n + 1, line.trim()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "a dev/ script must never remove a variable path recursively — the shape is \
         refused by a static scan BEFORE it runs, so it prompts a human and parks a \
         delegated agent with no transcript. Every root these tools mint comes from \
         `mktemp -d`, so there is nothing to remove.\n  {}",
        offenders.join("\n  "),
    );
}

/// The scanner has to be able to say no, or the arm above is decorative.
#[test]
fn the_recursive_removal_scanner_discriminates() {
    for bad in [
        "rm -rf $RIG",
        "rm -rf \"$RIG/repo\"",
        "    rm -fr \"${RIG}\"",
        "/bin/rm -R \"$D\"",
        "rm --recursive \"$D\"",
        "find \"$D\" -mindepth 1 -delete",
        "find \"$D\" -type f -exec rm {} +",
    ] {
        assert!(
            recursive_removal_of_a_variable_path(bad),
            "must be refused: {bad}",
        );
    }
    for ok in [
        "rm -f -- \"$PACK/config/schema-manifest.yaml\"",
        "rm -rf /tmp/a-literal-path",
        "mv \"$tmp\" \"$manifest\"",
        "# rm -rf $X would be refused before it runs",
        "find \"$D\" -type f",
        "printf 'rm -rf %s\\n' \"$D\"",
    ] {
        assert!(
            !recursive_removal_of_a_variable_path(ok),
            "must be allowed: {ok}",
        );
    }
}
