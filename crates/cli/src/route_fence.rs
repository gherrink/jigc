//! The CLI seam of the M43 route fence (law 2): a `Route::mechanical` whose argv does not
//! parse against the real CLI **cannot be constructed**
//! ([surface-contract.md](../../../design/surface-contract.md) → The route fence).
//!
//! The engine is clap-blind by layering, so the parse check lives here and is **installed**
//! into the engine constructor's hook at process start ([`install`], called from `main` so
//! every flow test through the binary carries the fence live) — the injected-validator shape,
//! chosen over a CLI-only construction helper because engine-side producers (`file_state`,
//! `validate`) mint routes too and must pass the same fence. Enforcement is debug-posture
//! (the seam-assert class): it rides the suite, never a release-build panic — the engine
//! consults the hook under `debug_assertions` only.

use clap::Parser;

/// The `<PLACEHOLDER>`-class dummy-substitution table — the **single declared table beside
/// the validator** (surface-contract.md → The route fence, pinned mechanics). A mechanical
/// route's argv may carry a placeholder the agent fills (`<task-id>`); for the parse check,
/// each whole-argv placeholder substitutes to a representative dummy value (the composed
/// route text keeps the placeholder verbatim — substitution is validation-only). A
/// placeholder **not** in this table fails the fence: the table is the declared set of legal
/// route placeholders, grown consciously as producers migrate onto the fence.
const DUMMY_SUBSTITUTIONS: &[(&str, &str)] = &[
    ("<task-id>", "dummy-task-id"),
    // The gate schema-conformance routes (engine `validate.rs` → `conformance_route`):
    // `<address>` is the finding's own `key.target`; `<value>` the field value to set.
    ("<address>", "adr:pick-a-db#context"),
    ("<value>", "dummy-value"),
    // The write-reject routes (engine `write.rs` → `write_route`): the doctype whose
    // resolved schema `jigc doc schema` projects.
    ("<doctype>", "adr"),
];

/// Install the parse fence into the engine's `Route::mechanical` constructor hook.
/// Idempotent (the engine keeps the first install); called from `main` and from this
/// module's tests.
pub fn install() {
    engine::finding::install_mechanical_argv_validator(validate_mechanical_argv);
}

/// Whether `argv` is a copy-runnable `jigc` command line: it must lead with `jigc` (clap
/// ignores argv\[0\], but the composed route text does not — a route missing the binary name
/// is broken even if the rest parses) and parse via `Cli::try_parse_from`, with
/// `<PLACEHOLDER>`-class args substituted from [`DUMMY_SUBSTITUTIONS`].
fn validate_mechanical_argv(argv: &[String]) -> Result<(), String> {
    if argv.first().map(String::as_str) != Some("jigc") {
        return Err("a mechanical route's argv must lead with `jigc`".to_owned());
    }
    let mut substituted: Vec<&str> = Vec::with_capacity(argv.len());
    for arg in argv {
        if arg.starts_with('<') && arg.ends_with('>') {
            let dummy = DUMMY_SUBSTITUTIONS
                .iter()
                .find(|(placeholder, _)| *placeholder == arg)
                .map(|(_, dummy)| *dummy)
                .ok_or_else(|| {
                    format!(
                        "placeholder `{arg}` is not in the dummy-substitution table \
                         (crates/cli/src/route_fence.rs → DUMMY_SUBSTITUTIONS)"
                    )
                })?;
            substituted.push(dummy);
        } else {
            substituted.push(arg);
        }
    }
    crate::cli::Cli::try_parse_from(&substituted)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::finding::Route;

    /// **The fence itself** (the increment's Proves): a `Mechanical` route built from an argv
    /// that does not parse against the real CLI fires the debug assert at construction —
    /// through the engine constructor, with the validator installed exactly as `main`
    /// installs it.
    #[test]
    #[should_panic(expected = "must parse against the real CLI")]
    fn mechanical_route_from_a_non_parsing_argv_fires_the_fence() {
        install();
        let _ = Route::mechanical(["jigc", "frobnicate"], "");
    }

    /// A placeholder outside the declared table is a fence failure, not a pass-through —
    /// the table is the single declared set of legal route placeholders.
    #[test]
    #[should_panic(expected = "dummy-substitution table")]
    fn mechanical_route_with_an_undeclared_placeholder_fires_the_fence() {
        install();
        let _ = Route::mechanical(["jigc", "task", "finalize", "<something-undeclared>"], "");
    }

    /// The happy path: a parsing argv carrying a declared `<PLACEHOLDER>`-class arg
    /// constructs fine, and the substitution is **validation-only** — the composed route
    /// text keeps the placeholder verbatim.
    #[test]
    fn mechanical_route_with_a_declared_placeholder_passes_verbatim() {
        install();
        let route = Route::mechanical(["jigc", "task", "finalize", "<task-id>"], "");
        assert_eq!(route.as_str(), "`jigc task finalize <task-id>`");
    }

    /// The validator's own contract, exercised directly: a real verb line parses; a route
    /// whose argv omits the leading `jigc` is rejected even though clap alone would ignore
    /// argv[0] and accept it (the composed text is what the agent runs, and it would lack
    /// the binary name).
    #[test]
    fn validator_accepts_a_real_verb_line_and_requires_the_leading_jigc() {
        let ok: Vec<String> = ["jigc", "start"].map(String::from).into();
        assert_eq!(validate_mechanical_argv(&ok), Ok(()));

        let headless: Vec<String> = ["doc", "show", "adr:pick-a-db"].map(String::from).into();
        let err = validate_mechanical_argv(&headless).expect_err("headless argv must fail");
        assert!(err.contains("must lead with `jigc`"), "got: {err}");
    }
}
