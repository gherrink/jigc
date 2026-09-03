//! The parity fence over `dev/jigc-rig` — the bash front door onto the fixture
//! builder the Rust suites already use.
//!
//! `crates/cli/tests/support/trial_corpus.rs` builds named corpus states by driving
//! the real binary, and `dev/jigc-rig` builds the *same* named states for a shell
//! probe. Two builders of one state set is exactly the shape this repo calls a second
//! source of truth, so the set is fenced: the rig's accepted states and
//! [`State::ALL`] must be the **same set**, and a state added to one and not the other
//! reddens the gate rather than being discovered by a probe that silently cannot
//! build it. The one exception is declared, not implied — [`RIG_ONLY_STATES`] names
//! every state the rig builds that the Rust set deliberately omits, **with its
//! reason**, and an undeclared divergence in either direction still reddens.
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
//!
//! **That static arm is blind to this tool by construction, and the third arm is why.**
//! The rig does not *contain* its commands, it **writes** them: the construction is one
//! generated script text, so a scan of the bytes under `dev/` reads the generator and
//! never the generated. Two of the generator's interpolations passed a caller-supplied
//! name through raw where every sibling used `sq`, and the result was a rig that emitted
//! — and *ran* — `cp '…' "$PACK/schemas/adr"; rm -rf $PACK; echo ".yaml"`, wrote outside
//! its own root on `../../../x` with no metacharacter at all, and exited 0 doing it. The
//! static arm cannot see any of that; its allow-list even blesses
//! `printf 'rm -rf %s\n' "$D"`, which is precisely a generator of the shape. So the
//! third arm drives `--print-only` with adversarial inputs and asserts over the
//! **emitted bytes**: no adversarial name yields a script at all, and no script the rig
//! emits for legitimate input carries a recursive removal or a path escaping its root.
//! The static allowance stands *because* of this arm — a `printf` of the shape is only a
//! defect once the printed text is executed, and the printed text is now scanned.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support;
use support::trial_corpus::State;

/// The states `dev/jigc-rig` builds that [`State::ALL`] deliberately does **not**
/// declare, each with the reason it is rig-only.
///
/// The fence below narrows to exactly this list rather than dropping the same-set
/// claim: a *second* divergence still reddens, and the asymmetry is executable
/// metadata instead of a comment. `bare` is here because
/// `TrialCorpus::build_over` runs `jigc setup` for **every** state before it
/// dispatches, and every `State::ALL` consumer presupposes that — `compose_goldens`
/// sweeps each composed surface × every state, and a corpus with no `.jigc/` has no
/// composed surface to golden. Joining the Rust set would mean either ~100 goldens of
/// a corpus that cannot compose or a per-consumer exemption at each sweep, which is a
/// second way to say "not a `State::ALL` member".
const RIG_ONLY_STATES: &[(&str, &str)] = &[(
    "bare",
    "a git repo with one commit and NO `jigc setup` — the pre-adoption state from \
     which `setup` itself, the adapter install, `ingest`, `migrate-corpus` over a \
     never-adopted corpus and the leftover classifier are reachable at all",
)];

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
    let rig_only: BTreeSet<String> = RIG_ONLY_STATES
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect();

    let missing_from_rig: Vec<&String> = declared.difference(&accepted).collect();
    let undeclared_rig_only: Vec<&String> = accepted
        .difference(&declared)
        .filter(|s| !rig_only.contains(*s))
        .collect();
    // A rig-only entry whose state the rig no longer builds is a stale exemption, and
    // a stale exemption is how a narrowed claim rots back into a false one.
    let stale_exemptions: Vec<&String> = rig_only.difference(&accepted).collect();

    assert!(
        missing_from_rig.is_empty()
            && undeclared_rig_only.is_empty()
            && stale_exemptions.is_empty(),
        "`dev/jigc-rig` and `State::ALL` must name the same set of corpus states, up to \
         the rig-only states declared in `RIG_ONLY_STATES` WITH their reason — one \
         builder for the Rust suites, one for a shell probe, one set.\n  \
         declared in trial_corpus.rs but not buildable by the rig: {missing_from_rig:?}\n  \
         accepted by the rig, not declared in trial_corpus.rs, and not an \
         acknowledged rig-only state: {undeclared_rig_only:?}\n  \
         listed as rig-only but the rig does not build them: {stale_exemptions:?}\n  \
         acknowledged rig-only: {rig_only:?}",
    );
    assert!(
        !declared.is_empty(),
        "the comparison is only a fence while the declared set is non-empty",
    );
    for (name, reason) in RIG_ONLY_STATES {
        assert!(
            reason.len() > 40,
            "rig-only state `{name}` must carry a real reason, not a placeholder",
        );
    }
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

// ---------------------------------------------------------------------------
// The runtime arm: what the rig EMITS, not what it contains.
// ---------------------------------------------------------------------------

/// Adversarial `--schema` / `--workflow` names, each with the mechanism it exploits.
///
/// The first three are the three driven faces of the defect this arm exists for: the
/// emitted `cp` line carried a recursive removal, a command substitution ran during a
/// construction that then exited 0, and a `..` chain wrote outside the rig root with no
/// metacharacter at all. The rest are the same axis's other members — every way a shell
/// word can stop being one word, plus the two non-slug shapes (`/`, upper case) that
/// retarget the write without quoting having any opinion.
const ADVERSARIAL_PACK_TARGETS: &[(&str, &str)] = &[
    (
        "adr\"; rm -rf $PACK; echo \"",
        "a double quote closes the emitted path and starts a new command",
    ),
    (
        "adr$(touch \"$RIG/INJECTED-COMMAND-RAN\")",
        "a command substitution runs when the emitted `cp` line runs",
    ),
    (
        "../../../OUTSIDE-THE-ROOT",
        "`..` escapes the pack root with no metacharacter at all",
    ),
    ("adr;rm -rf /", "a `;` ends the command and starts another"),
    ("adr`id`", "backticks substitute a command"),
    ("$PACK", "a bare expansion is still an expansion"),
    ("sub/dir", "a `/` retargets the write out of schemas/"),
    (
        "adr name",
        "whitespace splits the emitted `cp` into more arguments",
    ),
    ("adr\nrm -rf /", "a newline is a command separator"),
    ("", "an empty name writes a dotfile named `.yaml`"),
    (
        "ADR",
        "not a slug: every pack resource id this repo ships is lower-case",
    ),
    (
        "-adr",
        "a leading dash reads as an option wherever the name is re-parsed",
    ),
];

/// A legitimate file to install — the rig only *copies* it under `--print-only`, so
/// any readable file proves the path shape.
fn a_readable_pack_file() -> PathBuf {
    repo_root().join("crates/cli/pack/schemas/adr.yaml")
}

/// Drive the rig, always against the freshly built binary and an absolute scratch root.
fn run_rig(args: &[&str]) -> std::process::Output {
    let scratch = support::trial_corpus::unique_root("rig-fence");
    std::fs::create_dir_all(&scratch).expect("create the fence scratch root");
    Command::new(dev_dir().join("jigc-rig"))
        .args(args)
        .arg("--binary")
        .arg(env!("CARGO_BIN_EXE_jigc"))
        .current_dir(repo_root())
        .env("SCRATCH", &scratch)
        .output()
        .expect("spawn dev/jigc-rig")
}

/// Does this emitted line reach outside the root it was given?
///
/// The rig's every path is rooted at `$RIG`, `$REPO` or `$PACK`, so a `..` component
/// anywhere in the emitted text is an escape — there is no legitimate one.
fn escapes_its_root(line: &str) -> bool {
    line.contains("/../")
        || line.ends_with("/..")
        || line.contains("/..\"")
        || line.contains("/..'")
}

#[test]
fn the_rig_refuses_a_pack_target_that_is_not_a_slug() {
    let file = a_readable_pack_file();
    let file = file.to_str().expect("utf-8 fixture path");
    for (value, mechanism) in ADVERSARIAL_PACK_TARGETS {
        for flag in ["--schema", "--workflow"] {
            let out = run_rig(&["fresh", flag, value, file, "--print-only"]);
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            assert!(
                !out.status.success(),
                "`dev/jigc-rig fresh {flag} {value:?} …` must be REFUSED ({mechanism}), \
                 but it exited {:?} and emitted:\n{stdout}",
                out.status.code(),
            );
            assert!(
                stdout.is_empty(),
                "a refused `{flag} {value:?}` must emit no script at all — a caller \
                 that ignores the status must have nothing to eval.\nstdout:\n{stdout}",
            );
            assert!(
                stderr.contains("slug"),
                "a refused `{flag} {value:?}` must say WHY (the slug rule), so the \
                 caller can fix it.\nstderr:\n{stderr}",
            );
        }
    }
}

#[test]
fn no_emitted_construction_carries_a_recursive_removal_or_escapes_its_root() {
    let file = a_readable_pack_file();
    let file = file.to_str().expect("utf-8 fixture path");

    // The adversarial names FIRST — this arm's headline claim is about them, and an
    // ordering that reached them last would let a shape assertion over legitimate
    // input fire before the claim was ever exercised. Then every state, and every
    // flag combination that reaches the pack-writing seam.
    let mut invocations: Vec<Vec<String>> = Vec::new();
    for (value, _) in ADVERSARIAL_PACK_TARGETS {
        for flag in ["--schema", "--workflow"] {
            invocations.push(vec![
                "fresh".into(),
                flag.into(),
                (*value).into(),
                file.into(),
                "--print-only".into(),
            ]);
        }
    }
    for state in rig_states() {
        invocations.push(vec![state.clone(), "--print-only".to_string()]);
    }
    invocations.push(vec![
        "fresh".into(),
        "--pack-from-dev".into(),
        "--print-only".into(),
    ]);
    invocations.push(vec![
        "fresh".into(),
        "--repin".into(),
        "--schema".into(),
        "adr".into(),
        file.into(),
        "--print-only".into(),
    ]);
    invocations.push(vec![
        "committed-singletons".into(),
        "--schema".into(),
        "adr".into(),
        file.into(),
        "--workflow".into(),
        "single-task".into(),
        file.into(),
        "--start".into(),
        "single-task".into(),
        "an intent with 'quotes' and $VARS".into(),
        "--print-only".into(),
    ]);

    let mut emitted_any = false;
    for argv in &invocations {
        let args: Vec<&str> = argv.iter().map(String::as_str).collect();
        let out = run_rig(&args);
        let script = String::from_utf8_lossy(&out.stdout).to_string();
        if !out.status.success() {
            assert!(
                script.is_empty(),
                "a failed `dev/jigc-rig {}` must emit no script:\n{script}",
                args.join(" "),
            );
            continue;
        }
        emitted_any = true;
        for (n, line) in script.lines().enumerate() {
            assert!(
                !recursive_removal_of_a_variable_path(line),
                "`dev/jigc-rig {}` EMITS a recursive removal of a variable path at \
                 line {}: {line}\nThe static scan over dev/ cannot see this — the rig \
                 writes its commands rather than containing them.",
                args.join(" "),
                n + 1,
            );
            assert!(
                !escapes_its_root(line),
                "`dev/jigc-rig {}` EMITS a path escaping its own root at line {}: \
                 {line}\nEvery rig path is rooted at $RIG/$REPO/$PACK; a `..` \
                 component is an escape, and needs no metacharacter to be one.",
                args.join(" "),
                n + 1,
            );
        }
        // The pack-install seam specifically: the destination must stay under the
        // copied pack's own resource directories.
        for line in script.lines() {
            if !line.starts_with("cp ") || !line.contains("$PACK/") {
                continue;
            }
            assert!(
                line.contains("\"$PACK\"")
                    || line.contains("\"$PACK/schemas/\"")
                    || line.contains("\"$PACK/workflows/\""),
                "the emitted pack install must write under `$PACK/schemas/` or \
                 `$PACK/workflows/` with the expansion quoted and the caller's name a \
                 quoted literal beside it — got: {line}",
            );
        }
    }
    assert!(
        emitted_any,
        "the scan is only a fence while at least one invocation emits a script",
    );
}
