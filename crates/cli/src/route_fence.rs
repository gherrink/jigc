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
    // The T7 anyhow-embedded routes (`design/surface-contract.md` → The route fence,
    // closing paragraph): the milestone id a wrong-id reject routes through
    // (`jigc milestone list-tasks <milestone-id>`), the foreign-source path + target
    // doctype of the `jigc migrate` rejects, and the doctype positional of the
    // provision hint's `jigc doc create <type> --title 'X'` form (named `<type>`
    // after the verb's own usage line, distinct from the write-reject `<doctype>`).
    ("<milestone-id>", "dummy-milestone-id"),
    ("<path>", "CHANGELOG.md"),
    ("<type>", "adr"),
    // The task-intent positional of a `jigc start` route (the migrate byte-floor
    // advisory's from-knowledge alternative, `jigc start --workflow record-decision
    // <intent>`): a free-text intent an agent fills in with the decision it settled.
    ("<intent>", "a decision I settled"),
    // The title an agent supplies: the milestone-title positional of the `jigc milestone
    // create "<title>"` route, and the `--to` value of the `jigc doc rename` form the
    // foreclosed-`--task` tip at `jigc rename` names (`cli.rs` → `tip_rename_task_flag`).
    // It reaches the fence in the **quoted-span** form too, which until the quoting half
    // landed was not recognized as a placeholder at all — so it skipped this table and the
    // derivability verdict beside it (`milestone.rs` → the unknown-milestone route).
    ("<title>", "A Milestone Title"),
    // The knob-key positional of `jigc config get <key>` — the read rung the read-shaped
    // `config show` tip routes to (M48 inc-6 T3). A declared knob key, so the dummy is one.
    ("<key>", "docs-root"),
    // The `--workflow <workflow-id>` value of the milestone sub-task doors' re-run route
    // (`milestone.rs` → `ensure_workflow_provided`): the caller picks from the provided set
    // the rejection names, so the dummy is the doors' own default.
    ("<workflow-id>", "sub-task"),
];

/// Install the parse fence into the engine's `Route::mechanical` constructor hook.
/// Idempotent (the engine keeps the first install); called from `main` and from this
/// module's tests.
pub fn install() {
    engine::finding::install_mechanical_argv_validator(validate_mechanical_argv);
}

/// The fence asked as a **question** rather than as an assertion: `true` iff `argv` would
/// pass [`validate_mechanical_argv`]. A producer that *derives* an argv from another route's
/// (the boundary-door task-selector enrichment, `crate::doc::scope_repair_route_to_task`)
/// cannot know in advance that its result parses, and the constructor's own check is
/// debug-posture — so it asks here, in every posture, and emits the derived route only when
/// the answer is yes. The fence therefore governs a derived route in release builds too,
/// where its panic never fires.
pub(crate) fn accepts(argv: &[String]) -> bool {
    validate_mechanical_argv(argv).is_ok()
}

/// Whether `argv` is a copy-runnable `jigc` command line: it must lead with `jigc` (clap
/// ignores argv\[0\], but the composed route text does not — a route missing the binary name
/// is broken even if the rest parses), every token must be **shell-safe as emitted**
/// ([`shell_safe`]), and it must parse via `Cli::try_parse_from`, with `<PLACEHOLDER>`-class
/// args substituted from [`DUMMY_SUBSTITUTIONS`].
fn validate_mechanical_argv(argv: &[String]) -> Result<(), String> {
    if argv.first().map(String::as_str) != Some("jigc") {
        return Err("a mechanical route's argv must lead with `jigc`".to_owned());
    }
    let mut substituted: Vec<&str> = Vec::with_capacity(argv.len());
    for arg in argv {
        // A placeholder reaches the fence in two emitted forms: bare (`<task-id>`) and as a
        // **quoted span** (`"<intent>"` — the milestone `next:` lines, which quote it to show
        // the prose that replaces it is one argument). Both are the agent's to fill, and both
        // must be declared: matching only the bare form let the quoted one skip the table
        // *and* the derivability verdict beside it.
        let arg = arg
            .strip_prefix('"')
            .and_then(|rest| rest.strip_suffix('"'))
            .filter(|inner| inner.starts_with('<') && inner.ends_with('>'))
            .unwrap_or(arg);
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
            if !shell_safe(arg) {
                return Err(format!(
                    "token `{arg}` is not shell-safe as emitted — a route's text is \
                     `argv.join(\" \")`, i.e. bytes an agent pastes into a shell, so an \
                     author-owned prose token (a title, an intent) must be rendered \
                     through `crate::task::shell_token`"
                ));
            }
            substituted.push(arg);
        }
    }
    crate::cli::Cli::try_parse_from(&substituted)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

/// Whether one emitted argv token survives a real shell as **exactly itself** — the
/// quoting half of the route fence (M48 inc-2 triage), now a thin alias over the engine's
/// [`engine::finding::shell_safe`].
///
/// The parse fence above proves a route's argv is a real command; it cannot prove the
/// *text* an agent pastes splits back into that argv, because the text is
/// `argv.join(" ")` and a shell re-lexes it. A token holding author-owned prose or a
/// filesystem path is where the two diverge.
///
/// **It delegates because the engine mints routes too** (M51 completion audit): the
/// predicate is lexical and needs no clap, so it lives beside the constructor that composes
/// a route's text, where it also fences the `Route::human` and prose-tail spans this
/// CLI-side argv check structurally cannot see.
fn shell_safe(token: &str) -> bool {
    engine::finding::shell_safe(token)
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

    /// **The two tables are one membership set** (M47 Inc 6 T3, P6 route-followability):
    /// this table declares what may *appear* in a route's argv; the engine's
    /// [`engine::finding::ROUTE_PLACEHOLDERS`] declares, per token, whether the finding's
    /// own `key.target` must *fill* it. A token in one and not the other is a hole — a new
    /// placeholder that silently skips the derivability verdict, or a verdict about a token
    /// no route may carry. The fence lives here because this is the only module where both
    /// tables are visible (the engine is CLI-blind by layering), and it asserts the sets
    /// match **in order**, so the two tables read as one.
    #[test]
    fn the_dummy_table_and_the_derivability_table_declare_the_same_placeholders() {
        let declared: Vec<&str> = DUMMY_SUBSTITUTIONS
            .iter()
            .map(|(placeholder, _)| *placeholder)
            .collect();
        let derivability: Vec<&str> = engine::finding::ROUTE_PLACEHOLDERS
            .iter()
            .map(|row| row.token)
            .collect();
        assert_eq!(
            declared, derivability,
            "every legal route placeholder carries a derivability verdict, and every verdict \
             names a legal route placeholder (crates/cli/src/route_fence.rs → \
             DUMMY_SUBSTITUTIONS; engine `finding::ROUTE_PLACEHOLDERS`)",
        );
    }

    /// **The quoting half of the fence** (M48 inc-2 triage): a route that embeds an
    /// author-owned title the double-quoted (`Debug`) way cannot be constructed — those
    /// bytes, pasted into a shell, expand `$HOME` and run `$( … )`, so the route silently
    /// recovers something other than what it names.
    #[test]
    #[should_panic(expected = "is not shell-safe as emitted")]
    fn mechanical_route_with_a_double_quoted_prose_token_fires_the_fence() {
        install();
        let title = "Cache $HOME rework";
        let _ = Route::mechanical(
            [
                "jigc",
                "doc",
                "rename",
                "adr:x",
                "--to",
                &format!("{title:?}"),
            ],
            "",
        );
    }

    /// The shape the fence exists to admit: the same prose through
    /// [`crate::task::shell_token`] — POSIX single-quoted, its own `'` spliced — plus the
    /// bare tokens (ids, addresses with a `#fragment`, flags) that need no quoting at all.
    #[test]
    fn shell_safe_admits_the_bare_and_the_single_quoted_forms_only() {
        for bare in [
            "jigc",
            "doc",
            "--to",
            "adr:pick-a-db#context",
            "docs/decisions/alpha.md",
            "-",
        ] {
            assert!(shell_safe(bare), "`{bare}` is shell-inert");
        }
        for prose in [
            "Cache $HOME rework",
            "Cache $(touch PWNED) rework",
            "Cache `touch PWNED` rework",
            "Cache 'quoted' rework",
            "Adopt Redis",
            "",
        ] {
            let token = crate::task::shell_token(prose);
            assert!(
                shell_safe(&token),
                "`shell_token` output must pass the fence; got `{token}` for {prose:?}"
            );
            assert!(
                !shell_safe(&format!("{prose:?}")),
                "the double-quoted `Debug` form of {prose:?} is the defect this fence \
                 refuses — a shell expands inside it"
            );
        }
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
