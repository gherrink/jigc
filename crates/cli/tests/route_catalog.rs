//! **The route catalog** — a case per refusal site, cause and way out of its route: the
//! state that provokes the refusal, the refusing command, the route lifted from the printed
//! bytes and followed, and what must hold afterwards (`DECISIONS.md` → 2026-10-10, ruling 3; the plan's step `P1`,
//! `completions/artifacts/M55/stabilization-build/reshape/plan.md`).
//!
//! The exit rule says *every refusal's route works as printed*, and until this suite
//! nothing ran a `setup` route at all. The frame is
//! [`support::route_case`](crate::support::route_case), which says what a case is and what
//! the runner does; this file is the **expected-red list** ([`EXPECTED_RED`]) and the arms
//! that hold the frame to its rules. **A command's cases are a file of their own under
//! `route_catalog/`**, declared here by `#[path]` inside [`tables!`] — `setup.rs` and
//! `uninstall.rs`.
//! They stand there, and not beside this file, because that directory also holds the
//! committed list of refusal sites: a suite file beside a directory of its own name makes
//! the directory an aggregation for the registration fence
//! (`test_target_registration.rs`), which holds every `.rs` file in it to a declaration
//! here and refuses a directory that has none. Neither this file nor a command's acts on a
//! file by its own hand: every act of a case goes through the runner, which takes its
//! operand from the printed route.
//!
//! **A red case is a finding, not a failure to hide.** A case that comes out red is listed
//! in [`EXPECTED_RED`] with the key of the finding it reproduces and the classification it
//! is red with, and the list is strict three ways: a listed case that is no longer red
//! fails, a red case that is not listed fails, and a listed case whose classification moved
//! fails. So the gate is green over a known defect, the defect is on record under its key,
//! and the fix that turns the case removes the entry in the same commit — nothing here is
//! fixed, and nothing here asserts a route's wording.
//!
//! **The cases are followed by one arm a command's file** — `tables!` makes the arm with
//! the file's row, under [`FOLLOWED`], so the test a runner bounds is one command's cases
//! and not the catalog's ([`follow_the_cases_of`]). Each arm holds the list three ways over
//! its own cases and reports every difference of its command at once; a case stands in one
//! file, so the arms together hold the list across the catalog, and what is wrong with the
//! list by itself — an entry for a case no file holds, a case listed twice — is one arm's
//! that follows nothing
//! ([`every_entry_of_the_expected_red_list_names_a_case_of_the_catalog`]). That every file
//! has its arm, and every arm a file, is held against the tests the harness itself lists
//! ([`every_table_has_its_arm_and_every_arm_its_table`]).
//!
//! **Before a stabilization run is opened, a key certifies itself**: it is the key the
//! small canary gave the finding
//! (`completions/artifacts/M55/stabilization-build/canary-one/findings.md`), and no ledger
//! holds it yet. The tie becomes a real one when a run's opening seeds its rows.
//!
//! **What the catalog holds today** is every blocking site of `jigc setup`, thirty-six (the
//! plan's step `P3`): the hook's link refusal — the site whose cause an existing test
//! planted and passed over; the two refusals the install raises before its first write over
//! an entry in the way of a file it replaces or merges into; the two of a command that
//! cannot find its repository; the four over the adapter profile; the dirty gate, and the
//! refusal where git cannot answer it; the settings file's question before the first write;
//! the fourteen steps of the write span; and the two of the install commit. Four of the
//! thirty-six have a reason and no case. **And nine of the eighteen blocking sites of
//! `jigc uninstall`** (the plan's step `P4`): the two of a command that cannot find its
//! repository, the profile not loaded, and three of the four guards over what `.jigc/`
//! holds the only copy of — a worktree path with content, a byte jigc did not write, an
//! open task's staged doc — each with the refusal it raises where it could not look. The
//! other nine are rows of [`OWED`], under the task that owes their cases.
//!
//! **A site with a case says what was read to count its causes**
//! ([`Reading`], a command's `READINGS`): the functions, the count with its derivation,
//! every cause that has no case with why, and every way out of its route that no case
//! follows with why. A fence can hold sites and never causes, so the count is a hand count
//! — [`every_site_with_a_case_says_what_was_read_and_every_cause_is_answered`] holds it to
//! the cases: the causes the cases follow and the causes given a reason are together the
//! count, and no more.
//!
//! **A cause is followed every way its route offers.** A route can offer two acts and a
//! clause that turns on git — *remove the link, or point it at a regular file inside this
//! repository — committing the change where git tracks the link* — and every red case of
//! the first ten sites stood on the second act or on the clause, where one case a cause
//! had looked. So a command's cases are groups, a group a cause, and within it a case a
//! way.
//!
//! **A doubtful reading of a route's prose is marked, never decided**
//! ([`Case::doubt`](crate::support::route_case::Case::doubt)): the case follows the printed
//! words most literally, says the two readings, and is listed for the human with whatever
//! it came out as.
//!
//! **The frame grew with the cases**, and each piece has an arm that was red before it was
//! built: the state steps and file acts
//! ([`a_state_step_plants_what_it_names_and_a_file_act_does_what_the_route_says`]), the
//! marked form's refusal for every act that takes an operand
//! ([`the_marked_form_is_refused_for_every_act_that_takes_an_operand`]), the sandbox's
//! bound for each new act ([`no_file_act_is_done_outside_the_sandbox`]), and the limit a
//! child is waited for ([`a_command_that_does_not_end_is_ended_and_the_case_says_so`]) —
//! a limit that is the commands' under test and never the sandbox's own setup's
//! ([`the_handed_limit_ends_the_command_under_test_and_never_the_sandboxs_setup`]). **A
//! command that does not end is a classification of its own**, the eighth
//! ([`the_classifications_in_their_order`]): *not provoked* could not tell a command that
//! held from one that exited 0 and said nothing, and the list is strict by classification.
//! A case listed under it is waited for seconds, so a known hold is no two minutes of
//! every run.
//!
//! **The cases are followed through a pool of bounded width**
//! ([`route_case::pooled`](crate::support::route_case::pooled),
//! [`route_case::width`](crate::support::route_case::width)): no more at once than the
//! machine has cores, however many a command's file holds
//! ([`no_more_cases_run_at_once_than_the_pool_is_wide`]). Neither this file nor a command's
//! starts a thread of its own. **And every case followed in one process passes one gate of
//! that width** ([`route_case::Gate`](crate::support::route_case::Gate)), so two commands'
//! arms that run as threads of one process — `cargo test`, which is what CI and the
//! container run — are one machine's worth of chains together
//! ([`no_more_chains_run_at_once_in_one_process_than_the_machine_is_wide`]). **Across
//! processes nothing bounds them**: under nextest, which gives each test a process, two
//! commands' arms that run at once are up to twice the machine's width of chains. A bound
//! inside the suite would make one arm wait for the other inside its own eight minutes,
//! which is the limit the arms were split for; the bound that costs an arm no time is the
//! runner's (a test group of one thread in `.config/nextest.toml`), a file that is not this
//! suite's.
//!
//! **And with the cases of `jigc uninstall`, which removes**: what must hold once a route
//! is followed is that a thing is gone, that jigc's lines are out of a file and its owner's
//! still in it, and that a file inside the repository the command has no business touching
//! is what it was ([`what_is_gone_what_still_stands_and_what_was_not_touched`]).
//!
//! **A printed line that no shell can split is a red case** — the ninth classification
//! ([`a_printed_line_no_shell_can_split_is_a_red_case_and_stops_no_arm`]). The frame's
//! reader of a printed line used to end the whole arm over one.
//!
//! **And with the sites behind those**, each piece again with an arm that was red first: a
//! case stands where it says, in the environment it says
//! ([`a_case_stands_where_it_says_in_the_environment_it_says`]); the refusing command is run
//! again where a route prints no command line, and nowhere else
//! ([`the_refusing_command_is_run_again_only_where_the_route_prints_no_command_line`]); and
//! every repository of a sandbox has the sandbox's identity, one a route's own `git init`
//! makes included ([`a_repository_a_route_makes_has_the_sandboxs_identity_and_no_other`]);
//! and a file is mended with the runner's own bytes, an entry made readable or writable
//! ([`a_file_is_mended_and_an_entry_is_made_readable_or_writable`]).
//!
//! **And with the steps of the write span**: a state is made by the binary under test
//! where it is one an earlier run left — an install that stands
//! ([`a_state_step_runs_the_binary_under_test`]); and a sandbox is removed whole, whatever
//! directory its state shut ([`a_sandbox_is_removed_whole_whatever_its_state_shut`]). And
//! with the dirty gate, whose route says to move a file out of its path: the runner moves
//! it, as it moves a directory out of the way ([`a_file_is_moved_out_of_its_path`]).
//!
//! **The fence** (the plan's step `P2`) holds the catalog to the refusal sites of the
//! families [`CATALOGUED`] names. It scans nothing for itself — the sites are
//! [`support::refusal_sites`](crate::support::refusal_sites)' scan, which
//! `refusal_site_list.rs` holds byte-equal to the committed list, and the literals are
//! [`support::rust_source`](crate::support::rust_source)'s. Two arms:
//!
//!   * [`every_refusal_site_of_the_catalogued_families_has_a_case_a_reason_or_an_owed_row`]
//!     — a blocking site has exactly one answer: a case in its command's file, a
//!     [`Reason`] there, or a row of [`OWED`].
//!   * [`every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed`] — the list
//!     says where a finding is constructed, and a code that reaches its constructor
//!     through a `match`, a parameter or a table stands in no row of its family. So the
//!     **vocabulary** is held as well: every code-shaped literal of a catalogued prefix
//!     that production source spells is a listed site's code, is claimed by a row of
//!     [`CLAIMS`] for the `dynamic` site that mints it, or has a row of [`NO_CODE`].
//!     **A claim makes a site**: the code at the dynamic site it names owes a case like a
//!     listed one, under the id [`Claim::id`] gives it.
//!
//! and beside them [`a_row_for_a_site_that_is_gone_is_named`], for a table that outlives
//! what it is about; [`every_file_of_the_catalogs_directory_is_a_table_the_suite_reads`],
//! for a command's file that is on disk and not wired in; and
//! [`the_fence_is_not_vacuous`].
//!
//! **What the fence cannot see — declared.** A code is found where one string literal
//! spells it whole: a code built at run time is not found, nor one spelled only outside the
//! Rust source of the two crates. And **a code that is a listed site's is asked no
//! further**, so a dynamic site that mints codes other sites spell too is demanded by no
//! arm. Two such sites stand in `setup.rs`; their rows of [`CLAIMS`] are kept by reading,
//! and the table says which rows those are.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use crate::support::refusal_sites::{self, DYNAMIC, FAMILIES, Site, Source};
use crate::support::route_case::{
    self, Act, Case, Check, Classification, ExpectedRed, Followed, Operand, Place, Refusal, Seen,
    Step, Var,
};
use crate::support::rust_source;

/// One command's file under `route_catalog/`, as the suite reads it.
struct Table {
    /// The file, from this one's directory.
    file: &'static str,
    /// Its source, for the arm that holds a case to being data.
    source: &'static str,
    /// The cases, a group a cause.
    cases: &'static [&'static [Case]],
    reasons: &'static [Reason],
    readings: &'static [Reading],
    /// The arm that follows the file's cases, by its name under [`FOLLOWED`].
    arm: &'static str,
}

/// **A command's file is wired in by its declaration inside this macro's one call, and
/// nowhere else.** The declaration is the `#[path]` line the registration fence reads
/// (`test_target_registration.rs`), and the macro makes the same line the file's row of
/// [`TABLES`] — its cases, its reasons and its source — so no arm of this suite names a
/// command's file a second time. The file exports `CASES`, `REASONS` and `READINGS`.
///
/// **And the same line makes the arm that follows the file's cases** — a test of the
/// file's own name in the module [`FOLLOWED`] names — so a command's file cannot be wired
/// in without its arm, and the test a runner puts a limit on is one command's cases.
macro_rules! tables {
    ($(#[path = $file:literal] mod $name:ident;)*) => {
        $(
            #[path = $file]
            mod $name;
        )*

        /// Every command's file, in the order declared.
        const TABLES: &[Table] = &[$(Table {
            file: $file,
            source: include_str!($file),
            cases: $name::CASES,
            reasons: $name::REASONS,
            readings: $name::READINGS,
            arm: stringify!($name),
        }),*];

        /// **A command's cases are followed on the binary this suite was built with, and
        /// the expected-red list is exactly the ones that came out red** — one arm a
        /// command's file ([`follow_the_cases_of`]).
        mod the_cases_are_followed_and_the_expected_red_list_is_exact {
            $(
                #[test]
                fn $name() {
                    super::follow_the_cases_of($file);
                }
            )*
        }
    };
}

/// The module `tables!` puts the arms that follow the cases in, as the harness names it
/// inside this suite.
const FOLLOWED: &str = "the_cases_are_followed_and_the_expected_red_list_is_exact";

tables! {
    #[path = "route_catalog/setup.rs"]
    mod setup;
    #[path = "route_catalog/uninstall.rs"]
    mod uninstall;
}

use setup::{HOOK, HOOK_LINK_SITE};

/// Every case of every command's file.
fn cases() -> Vec<&'static Case> {
    TABLES
        .iter()
        .flat_map(|table| table.cases)
        .flat_map(|group| group.iter())
        .collect()
}

/// **The cases that are red, by the key of the finding each reproduces** — a row a key and
/// a classification, with the cases that come out so. The keys of the hook's rows are the
/// small canary's (`completions/artifacts/M55/stabilization-build/canary-one/findings.md`,
/// §1 and §3); the others are no finding of the canary's and were minted with the cases, in
/// `completions/artifacts/M55/stabilization-build/reshape/route-catalog-findings.md`.
const EXPECTED_RED: &[(&str, Classification, &[&str])] = &[
    (
        HOOK_END,
        Classification::SameRefusalAgain,
        &[
            "setup-hook-link-out-to-a-directory-pointed-at-a-committed-script",
            "setup-hook-link-out-to-a-directory-pointed-at-a-new-script",
            "setup-hook-link-out-to-a-directory-removed",
            "setup-hook-link-out-to-a-script-pointed-at-a-committed-script",
            "setup-hook-link-out-to-a-script-pointed-at-a-new-script",
            "setup-hooks-path-inside-link-out-to-a-script-pointed-at-a-committed-script",
            "setup-hooks-path-inside-link-out-to-a-script-pointed-at-a-new-script",
            "setup-hooks-path-outside-link-out-of-it-to-a-script-pointed-at-a-committed-script",
            "setup-hooks-path-outside-link-out-of-it-to-a-script-pointed-at-a-new-script",
        ],
    ),
    (
        HOOK_END,
        Classification::WitnessChanged,
        &[
            "setup-hook-link-out-to-a-script-removed",
            "setup-hooks-path-inside-link-out-to-a-script-removed",
            "setup-hooks-path-outside-link-out-of-it-to-a-script-removed",
        ],
    ),
    (
        HOOK_RELATIVE,
        Classification::SameRefusalAgain,
        &[
            "setup-hook-relative-link-out-to-a-script-pointed-at-a-committed-script",
            "setup-hook-relative-link-out-to-a-script-pointed-at-a-new-script",
            "setup-hook-write-hook-a-directory",
            "setup-hook-write-hook-unreadable",
        ],
    ),
    (
        OUTSIDE_A_WORK_TREE,
        Classification::SameRefusalAgain,
        &["setup-hook-write-in-a-worktree-of-a-bare-repository"],
    ),
    (
        HOOK_RELATIVE,
        Classification::WitnessChanged,
        &["setup-hook-relative-link-out-to-a-script-removed"],
    ),
    (
        HOOK_DEVICE,
        Classification::SameRefusalAgain,
        &[
            "setup-hook-link-to-dev-null-pointed-at-a-committed-script",
            "setup-hook-link-to-dev-null-pointed-at-a-new-script",
            "setup-hook-link-to-dev-null-removed",
        ],
    ),
    (
        NEW_SCRIPT,
        Classification::AnotherFailure,
        &[
            "setup-hook-link-to-nothing-pointed-at-a-new-script",
            "setup-hooks-path-inside-link-to-nothing-pointed-at-a-new-script",
            "setup-hooks-path-outside-link-to-nothing-pointed-at-a-new-script",
        ],
    ),
    (
        UNTRACKED_LINK,
        Classification::AnotherFailure,
        &[
            "setup-host-link-into-gits-directory-pointed-at-a-committed-file",
            "setup-host-link-into-gits-directory-pointed-at-a-new-file",
            "setup-host-link-out-of-the-repository-pointed-at-a-committed-file",
            "setup-host-link-out-of-the-repository-pointed-at-a-new-file",
            "setup-host-link-to-a-directory-pointed-at-a-committed-file",
            "setup-host-link-to-a-directory-pointed-at-a-new-file",
            "setup-host-link-to-nothing-pointed-at-a-committed-file",
            "setup-host-link-to-nothing-pointed-at-a-new-file",
            "setup-settings-link-into-gits-directory-pointed-at-a-committed-file",
            "setup-settings-link-into-gits-directory-pointed-at-a-new-file",
            "setup-settings-link-out-of-the-repository-pointed-at-a-committed-file",
            "setup-settings-link-out-of-the-repository-pointed-at-a-new-file",
            "setup-settings-link-to-a-directory-pointed-at-a-committed-file",
            "setup-settings-link-to-a-directory-pointed-at-a-new-file",
            "setup-settings-link-to-nothing-pointed-at-a-committed-file",
            "setup-settings-link-to-nothing-pointed-at-a-new-file",
        ],
    ),
    (
        NEW_FILE,
        Classification::AnotherFailure,
        &[
            "setup-host-link-into-gits-directory-committed-and-pointed-at-a-new-file",
            "setup-host-link-out-of-the-repository-committed-and-pointed-at-a-new-file",
            "setup-host-link-to-a-directory-committed-and-pointed-at-a-new-file",
            "setup-host-link-to-nothing-committed-and-pointed-at-a-new-file",
            "setup-settings-link-into-gits-directory-committed-and-pointed-at-a-new-file",
            "setup-settings-link-out-of-the-repository-committed-and-pointed-at-a-new-file",
            "setup-settings-link-to-a-directory-committed-and-pointed-at-a-new-file",
            "setup-settings-link-to-nothing-committed-and-pointed-at-a-new-file",
        ],
    ),
    (
        NO_FLOOR,
        Classification::AnotherFailure,
        &[
            "setup-seeded-ignore-link-into-gits-directory-pointed-at-a-new-file",
            "setup-seeded-ignore-link-into-gits-directory-staged-and-pointed-at-a-new-file",
            "setup-seeded-ignore-link-out-of-the-repository-pointed-at-a-new-file",
            "setup-seeded-ignore-link-out-of-the-repository-staged-and-pointed-at-a-new-file",
            "setup-seeded-ignore-link-to-a-directory-pointed-at-a-new-file",
            "setup-seeded-ignore-link-to-a-directory-staged-and-pointed-at-a-new-file",
            "setup-seeded-ignore-link-to-nothing-pointed-at-a-new-file",
            "setup-seeded-ignore-link-to-nothing-staged-and-pointed-at-a-new-file",
        ],
    ),
    (
        HELD,
        Classification::DidNotEnd,
        &[
            "setup-compose-marker-a-special-file",
            "setup-guide-a-special-file",
            "setup-hook-write-hook-a-special-file",
            "setup-reference-host-a-special-file",
        ],
    ),
    (
        HOME_UNSET,
        Classification::SameRefusalAgain,
        &["setup-home-not-set-run-as-the-route-prints"],
    ),
    (
        REINSTALL,
        Classification::SameRefusalAgain,
        &[
            "setup-profile-declares-no-reference",
            "setup-profile-malformed",
            "setup-profile-not-in-the-adapters-directory",
            "setup-profile-not-utf-8",
        ],
    ),
    (
        CLAUSE,
        Classification::SameRefusalAgain,
        &[
            "setup-guide-target-absolute",
            "setup-guide-target-climbing-out",
            "setup-guide-target-declares-a-stamp-key",
            "setup-guide-target-malformed-key",
            "setup-guide-target-without-a-path",
            "setup-launch-template-of-three-lines",
            "setup-launch-template-with-a-forbidden-marker",
            "setup-launch-template-with-other-shell",
            "setup-launch-template-without-a-backticked-span",
            "setup-launch-template-without-the-task-placeholder",
            "setup-launch-template-without-the-workflow-placeholder",
        ],
    ),
    (
        CLAUSE,
        Classification::AnotherFailure,
        &["setup-launch-template-without-the-invocation"],
    ),
    (
        PERMISSION_ROUTE,
        Classification::SameRefusalAgain,
        &[
            "setup-bootstrap-write-root-read-only",
            "setup-bootstrap-write-workbench-a-file",
            "setup-bootstrap-write-workbench-read-only",
            "setup-compose-marker-packs-file-no-mapping",
            "setup-compose-marker-packs-file-not-utf-8",
            "setup-compose-marker-packs-file-not-yaml",
            "setup-compose-marker-packs-file-read-only",
            "setup-guide-write-no-directory-assistant-directory-read-only",
            "setup-guide-write-no-file-directory-read-only",
            "setup-hook-write-hook-not-utf-8",
            "setup-hook-write-hook-read-only",
            "setup-hook-write-hooks-path-a-file",
            "setup-hook-write-hooks-path-dev-null",
            "setup-hook-write-hooks-path-empty",
            "setup-hook-write-no-hooks-directory-parent-read-only",
            "setup-project-layer-config-a-file",
            "setup-project-layer-marker-read-only",
            "setup-project-layer-no-marker-config-read-only",
            "setup-reference-host-a-directory",
            "setup-reference-host-absent-root-read-only",
            "setup-reference-host-not-utf-8",
            "setup-seeded-ignore-a-directory",
            "setup-seeded-ignore-not-utf-8",
            "setup-seeded-ignore-read-only",
            "setup-seeded-ignore-unreadable",
            "setup-settings-a-directory-made-readable",
            "setup-settings-merge-into-the-ignored-reference-host",
            "setup-settings-merge-no-directory-root-read-only",
            "setup-settings-merge-no-file-directory-read-only",
            "setup-settings-record-asked-of-the-reference-host",
            "setup-settings-record-to-remove-workbench-read-only",
            "setup-version-stamp-read-only",
            "setup-workbench-ignore-a-directory",
            "setup-workbench-ignore-not-utf-8",
            "setup-workbench-ignore-short-and-read-only",
        ],
    ),
    (
        SHUT_WORKBENCH,
        Classification::AnotherFailure,
        &[
            "setup-project-layer-no-config-workbench-read-only",
            "setup-workbench-ignore-absent-workbench-read-only",
        ],
    ),
    (
        NO_RECORD,
        Classification::AnotherFailure,
        &[
            "setup-settings-record-absent-workbench-read-only",
            "setup-settings-record-read-only",
        ],
    ),
    (
        FORCE_CANNOT_COMMIT,
        Classification::AnotherFailure,
        &["setup-change-hidden-by-skip-worktree-forced"],
    ),
    (
        THE_QUESTION_THAT_FAILED,
        Classification::AnotherFailure,
        &[
            "setup-git-refuses-the-repository-asked-as-printed-and-resolved",
            "setup-git-status-fails-asked-as-printed-and-resolved",
        ],
    ),
    (
        A_SHAPE_FOR_A_LINE,
        Classification::UnsplittableLine,
        &[
            "setup-unborn-install-path-staged-and-deleted-unstaged-as-printed",
            "setup-unborn-staged-file-at-an-install-path-unstaged-as-printed-and-moved",
        ],
    ),
    (
        HOME_UNSET,
        Classification::SameRefusalAgain,
        &["uninstall-home-not-set-run-as-the-route-prints"],
    ),
    (
        REINSTALL,
        Classification::SameRefusalAgain,
        &[
            "uninstall-profile-malformed",
            "uninstall-profile-not-in-the-adapters-directory",
            "uninstall-profile-not-utf-8",
        ],
    ),
    (
        PLACEHOLDER_LINE,
        Classification::UnsplittableLine,
        &[
            "uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed",
            "uninstall-staged-doc-finalized-as-printed",
            "uninstall-staged-doc-read-and-discarded-as-printed",
        ],
    ),
    (
        FORCE_OVER_UNREADABLE,
        Classification::AnotherFailure,
        &[
            "uninstall-staged-docs-unsearchable-forced",
            "uninstall-task-areas-unlistable-forced",
            "uninstall-workbench-unsearchable-forced",
            "uninstall-worktree-path-unreadable-forced",
            "uninstall-worktrees-root-unreadable-forced",
        ],
    ),
    (
        READ_BIT_NOT_THE_CAUSE,
        Classification::SameRefusalAgain,
        &[
            "uninstall-staged-docs-unsearchable-made-readable",
            "uninstall-task-areas-unlistable-made-readable",
            "uninstall-workbench-unsearchable-made-readable",
        ],
    ),
    (
        MOVE_ASIDE_UNREADABLE,
        Classification::SameRefusalAgain,
        &["uninstall-worktree-path-unreadable-moved-out"],
    ),
    (
        WORKTREE_REMOVE_NO_PATH,
        Classification::SameRefusalAgain,
        &["uninstall-worktrees-root-unreadable-worktrees-removed-as-printed"],
    ),
];

// The canary's keys — the route of the hook's link refusal names where the link leads, and
// not the link.
const HOOK_END: &str = "r1-setup-install-hook-route-other-causes-unexamined";
const HOOK_RELATIVE: &str = "r1-p3-setup-install-hook-undriven-producers-and-causes";
const HOOK_DEVICE: &str = "r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null";

/// The canary's key for an install from a worktree of a bare repository: the install home is
/// the folder that holds the repository, the hook's write is refused there, and the
/// writability route leads back to the refusal.
const OUTSIDE_A_WORK_TREE: &str = "r1-setup-installs-outside-work-tree";

/// The canary's key for a route that asks for a permission where the cause is none: there
/// the hooks directory's writability, here — the same defect at another site — the settings
/// file's readability, over a directory at its path; and every other step of the write
/// span, each of which words every error of its step as *ensure … is writable* over one
/// entry it names, whatever the error was.
const PERMISSION_ROUTE: &str = "r1-p3-setup-writability-route-printed-for-other-causes";

// The pilot's keys.
/// A link git does not track, pointed at a regular file inside the repository as the route
/// says: the re-run refuses it as uncommitted work.
const UNTRACKED_LINK: &str = "r1-setup-untracked-link-repointed-draws-dirty-install-path";
/// A committed link pointed at a file in no commit, the link's change committed: the
/// re-run refuses over the file. Rests on a reading — every case under it is marked.
const NEW_FILE: &str = "r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path";
/// The hook's link pointed at a script in no commit: the re-run writes and stages the
/// install, splices its block into the script, and then refuses to commit. Rests on a
/// reading — every case under it is marked.
const NEW_SCRIPT: &str =
    "r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path";
/// On an unborn `HEAD`, the root `.gitignore`'s route followed by its second way out: the
/// re-run exits 0 and git ignores nothing the secrets floor lists.
const NO_FLOOR: &str = "r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor";
/// A FIFO at an install path `jigc setup` reads before it refuses: the command does not
/// end.
const HELD: &str = "r1-setup-fifo-at-install-path-holds-the-command";

/// `$HOME` is not set: the route says to run the command from inside the repository, where
/// it was run.
const HOME_UNSET: &str = "r1-setup-home-unset-route-says-run-from-inside-the-repository";
/// A profile read from the directory `JIGC_ADAPTERS_DIR` names is refused, and the route
/// says to reinstall jigc over an embedded profile that is not the one at fault.
const REINSTALL: &str = "r1-setup-profile-route-says-reinstall-under-adapters-dir";
/// A profile's launch template or guide target is refused, and the route is the violated
/// clause again: it names no act, no file and no command.
const CLAUSE: &str = "r1-setup-profile-clause-route-names-no-act";

/// `.jigc` may not be written, a step of the write span refuses over it, and the route says
/// to make it writable and run again: the re-run refuses the files the refused run wrote as
/// work in no commit — the record that says they are the install's own is kept in `.jigc`.
const SHUT_WORKBENCH: &str = "r1-setup-workbench-made-writable-rerun-refuses-its-own-files";
/// The settings record's write is refused after the entries are merged: the re-run finds
/// them in the settings file, records none, and leaves an install without its record.
const NO_RECORD: &str = "r1-setup-settings-record-refused-rerun-installs-without-the-record";

/// The dirty gate's consent, `--force`, over a change a skip-worktree flag hides: the install
/// runs over the file and git then refuses to stage it — no install commit is made.
const FORCE_CANNOT_COMMIT: &str = "r1-setup-force-over-skip-worktree-path-cannot-commit";
/// Where git cannot answer, the route's first step is to run the question that failed. Rests
/// on a reading — every case under it is marked.
const THE_QUESTION_THAT_FAILED: &str =
    "r1-setup-unverified-route-opens-with-the-question-that-failed";
/// Where there is no commit yet, the dirty route's line to unstage a file is printed with
/// `<path>` standing where the path belongs: no shell splits it. Rests on a reading — every
/// case under it is marked.
const A_SHAPE_FOR_A_LINE: &str = "r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line";

// The pilot's keys for `jigc uninstall`.
/// A route prints, between backticks, a line with a word in angle brackets where an id the
/// refusal lists belongs: no shell splits it. Rests on a reading — every case under it is
/// marked.
const PLACEHOLDER_LINE: &str = "r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id";
/// The consent, `jigc uninstall --force`, over a directory of the workbench that may not be
/// read or searched: the removal of `.jigc/` fails part-way, under another code.
const FORCE_OVER_UNREADABLE: &str = "r1-uninstall-force-over-unreadable-workbench-fails-part-way";
/// A guard that could not look says *make sure `.jigc/…` is readable* and names the root of
/// its walk: the entry at fault is another, or what it lacks is no read bit. Rests on a
/// reading — every case under it is marked.
const READ_BIT_NOT_THE_CAUSE: &str =
    "r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable";
/// The worktree guard's route over a path nothing can be read at says to move it aside: a
/// directory that may be neither read nor written cannot be moved.
const MOVE_ASIDE_UNREADABLE: &str = "r1-uninstall-worktree-route-move-aside-over-unreadable-path";
/// Where `.jigc/worktrees/` cannot be listed, the route's second way is `git worktree
/// remove`, printed with no path. Rests on a reading — the case under it is marked.
const WORKTREE_REMOVE_NO_PATH: &str =
    "r1-uninstall-worktrees-route-prints-git-worktree-remove-without-a-path";

/// [`EXPECTED_RED`], an entry a case.
fn expected_red() -> Vec<ExpectedRed> {
    EXPECTED_RED
        .iter()
        .flat_map(|(key, classification, cases)| {
            cases.iter().map(|case| ExpectedRed {
                case,
                key,
                classification: *classification,
            })
        })
        .collect()
}

fn binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_jigc"))
}

// ---------------------------------------------------------------------------
// The catalog
// ---------------------------------------------------------------------------

/// **Follow one command's cases on the binary this suite was built with, and hold the
/// expected-red list to exactly the ones that came out red.** One arm a command's file,
/// every difference of that command at once: a case that cannot be followed as written, a
/// red case no entry lists, a listed case that is not red, and a listed case whose
/// classification moved are all named before it fails. The entries held are the ones that
/// name a case of this file; what the list holds beside the cases of any file is
/// [`every_entry_of_the_expected_red_list_names_a_case_of_the_catalog`]'s.
fn follow_the_cases_of(file: &str) {
    let table = TABLES
        .iter()
        .find(|table| table.file == file)
        .unwrap_or_else(|| panic!("`{file}` is a row of `TABLES`"));
    let cases: Vec<&Case> = table.cases.iter().flat_map(|group| group.iter()).collect();
    let expected: Vec<ExpectedRed> = expected_red()
        .into_iter()
        .filter(|entry| cases.iter().any(|case| case.name == entry.case))
        .collect();
    let followed: Vec<Result<Followed, String>> =
        route_case::pooled(route_case::width(), &cases, |case| {
            // A case listed as one whose command does not end is waited for seconds.
            let limit = route_case::limit_of(&expected, case);
            route_case::follow_within(binary(), case, limit)
        });

    let mut wrong = Vec::new();
    let mut results = Vec::new();
    let mut seen = Vec::new();
    for (case, outcome) in cases.iter().zip(&followed) {
        match outcome {
            Ok(followed) => {
                results.push((case.name, followed.classification));
                seen.push(format!(
                    "`{}` ({}) — {}\n{}{}",
                    case.name,
                    case.cause,
                    followed.classification,
                    followed.detail.trim_end(),
                    case.doubt
                        .map(|doubt| format!("\ndoubt: {doubt}"))
                        .unwrap_or_default(),
                ));
            }
            Err(fault) => wrong.push(format!(
                "`{}` cannot be followed as written: {fault}",
                case.name
            )),
        }
    }
    wrong.extend(route_case::differences(&expected, &results));
    assert!(
        wrong.is_empty(),
        "the cases of `{file}` and the expected-red list disagree:\n\n  {}\n\nwhat each \
         case did:\n\n{}",
        wrong.join("\n  "),
        seen.join("\n\n"),
    );
}

/// **What is wrong with `expected` by itself**, whatever the cases come to: an entry for a
/// case that is among no file's `names`, a case listed twice, an entry that expects a
/// classification that is not red. It is the list held against every case coming out as
/// listed, so that nothing but the list's own faults is left to differ.
fn the_lists_own_faults(expected: &[ExpectedRed], names: &[&str]) -> Vec<String> {
    let as_listed: Vec<(&str, Classification)> = names
        .iter()
        .map(|name| {
            let listed = expected
                .iter()
                .find(|entry| entry.case == *name && entry.classification.is_red());
            (
                *name,
                listed.map_or(Classification::Holds, |entry| entry.classification),
            )
        })
        .collect();
    route_case::differences(expected, &as_listed)
}

/// **Every entry of the expected-red list names a case of the catalog, once.** The arms
/// that follow the cases each hold the entries of their own command's cases, so an entry
/// for a case no command's file holds would be held by none of them: it is named here,
/// with a case listed twice and an entry that expects a classification that is not red. On
/// planted lists, kind by kind; then this suite's.
#[test]
fn every_entry_of_the_expected_red_list_names_a_case_of_the_catalog() {
    let entry = |case, classification| ExpectedRed {
        case,
        key: "r1-a-key",
        classification,
    };
    let names = ["destroys", "works"];
    says(
        &the_lists_own_faults(&[entry("destroys", Classification::WitnessChanged)], &names),
        &[],
    );
    says(
        &the_lists_own_faults(
            &[
                entry("destroys", Classification::WitnessChanged),
                entry("gone", Classification::AnotherFailure),
                entry("destroys", Classification::SameRefusalAgain),
                entry("works", Classification::Holds),
            ],
            &names,
        ),
        &[
            ("`gone`", "names a case that was not followed"),
            ("`destroys`", "is listed twice"),
            ("`works`", "which is not red"),
        ],
    );

    let names: Vec<&str> = cases().iter().map(|case| case.name).collect();
    let wrong = the_lists_own_faults(&expected_red(), &names);
    assert!(
        wrong.is_empty(),
        "the expected-red list, held against the names of the catalog's cases:\n\n  {}",
        wrong.join("\n  "),
    );
}

/// The suite's own path inside the test binary, as the harness names its tests: the module
/// the group root declares it as.
fn suite() -> &'static str {
    module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, suite)| suite)
}

/// **Every way the tables and the arms that follow their cases disagree.** `tables` is
/// `(file, arm)`, a row a command's file; `listed` is every test the harness holds; `arms`
/// is the path the arms stand under, with its last `::`. A file whose arm the harness does
/// not list is named, and so is a test under `arms` that is no file's arm.
fn unarmed(tables: &[(&str, &str)], listed: &[String], arms: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (file, arm) in tables {
        if !listed.iter().any(|test| *test == format!("{arms}{arm}")) {
            out.push(format!(
                "`{file}` has no arm: the harness lists no test `{arms}{arm}`, so nothing \
                 follows its cases"
            ));
        }
    }
    for test in listed {
        if let Some(arm) = test.strip_prefix(arms)
            && !tables.iter().any(|(_, of)| *of == arm)
        {
            out.push(format!(
                "`{test}` follows no table: no file is wired in under the name `{arm}`"
            ));
        }
    }
    out
}

/// **Every command's file has its arm, and every arm its file** — held against the tests
/// the harness itself lists, and not against what `tables!` says it made: a file wired in
/// without an arm would have its cases followed by nothing, at a green gate, and an arm
/// left behind by a file that is gone would follow nothing under a name that says it does.
/// On planted lists; then this binary's own.
#[test]
fn every_table_has_its_arm_and_every_arm_its_table() {
    let planted = [
        ("route_catalog/setup.rs", "setup"),
        ("route_catalog/uninstall.rs", "uninstall"),
    ];
    let listed = |tests: &[&str]| {
        tests
            .iter()
            .map(|test| test.to_string())
            .collect::<Vec<_>>()
    };
    says(
        &unarmed(
            &planted,
            &listed(&["s::followed::setup", "s::followed::uninstall", "s::other"]),
            "s::followed::",
        ),
        &[],
    );
    says(
        &unarmed(
            &planted,
            &listed(&["s::followed::setup", "s::followed::gone", "s::uninstall"]),
            "s::followed::",
        ),
        &[
            ("`route_catalog/uninstall.rs`", "has no arm"),
            ("`s::followed::gone`", "follows no table"),
        ],
    );

    let tables: Vec<(&str, &str)> = TABLES.iter().map(|table| (table.file, table.arm)).collect();
    let wrong = unarmed(
        &tables,
        &route_case::harness_tests(),
        &format!("{}::{FOLLOWED}::", suite()),
    );
    assert!(
        wrong.is_empty(),
        "the tables and the arms that follow their cases disagree:\n\n  {}\n\na command's \
         file is wired in by its line inside `tables!`, which makes its arm.",
        wrong.join("\n  "),
    );
}

/// A case names its site as the committed list of refusal sites does — or, for a code a
/// dynamic site mints, as its claim does ([`Claim::id`]) — and its own name once, across
/// every command's file. So a renamed producer, or a site that is gone, is named here and
/// a case never outlives what it is about.
#[test]
fn every_case_names_a_site_of_the_list_and_has_a_name_of_its_own() {
    let mut sites: Vec<String> = refusal_sites::scan_tree().iter().map(Site::id).collect();
    sites.extend(CLAIMS.iter().filter_map(Claim::id));
    let cases = cases();
    let mut wrong = Vec::new();
    for (at, case) in cases.iter().enumerate() {
        if !sites.iter().any(|site| site == case.site) {
            wrong.push(format!(
                "`{}` names the site `{}`, which neither the list nor a claim holds",
                case.name, case.site
            ));
        }
        if cases[..at].iter().any(|other| other.name == case.name) {
            wrong.push(format!("two cases are named `{}`", case.name));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

// ---------------------------------------------------------------------------
// The fence — its tables
// ---------------------------------------------------------------------------

/// **The code prefixes the catalog is held to** — the families of `jigc setup`
/// (`setup`, `adapter-guide`) and of `jigc uninstall`, as
/// [`refusal_sites::FAMILIES`] assigns them. A prefix joins when its command's file does.
const CATALOGUED: &[&str] = &["adapter-guide", "setup", "uninstall"];

/// Why a blocking site has no case. The three words are the plan's, and a reason is one of
/// them by construction.
#[derive(Clone, Copy, Debug)]
pub enum Why {
    /// No state the runner can plant reaches the site. The text says what stands in the
    /// way.
    NotConstructible(&'static str),
    /// The site is reached only on a platform the suite does not run on. The text names it.
    Platform(&'static str),
    /// The refusal prints no route, so there is nothing to follow — taken only where the
    /// list reads no route written at the site's call.
    NoRoutePrinted,
}

impl fmt::Display for Why {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Why::NotConstructible(text) => write!(f, "not-constructible: {text}"),
            Why::Platform(text) => write!(f, "platform: {text}"),
            Why::NoRoutePrinted => f.write_str("no-route-printed"),
        }
    }
}

/// A site, and the reason it has no case. A command's reasons stand in its file beside its
/// cases (`REASONS`): they are what a reader of that file is told instead of a case.
#[derive(Clone, Copy, Debug)]
pub struct Reason {
    /// The site's id, as a case names it.
    pub site: &'static str,
    pub why: Why,
}

/// **What was read to count one site's causes, and the count.** A command's file states
/// one for every site it has a case of (`READINGS`), beside the cases.
#[derive(Clone, Copy, Debug)]
pub struct Reading {
    /// The site's id, as a case names it.
    pub site: &'static str,
    /// The functions that were read, each as `file::function`, with what it was read for
    /// where that is not the refusal itself.
    pub read: &'static [&'static str],
    /// How many causes they hold — a hand count.
    pub causes: usize,
    /// The count's derivation, in words.
    pub derivation: &'static str,
    /// The causes among them that have no case, each with why.
    pub without_case: &'static [(&'static str, Why)],
    /// **The ways out of the site's route that no case follows**, each with why. A route
    /// can offer two acts and a clause that turns on git; a cause is followed every such
    /// way, and a way that cannot be followed at this site is said here.
    pub not_followed: &'static [(&'static str, Why)],
}

/// **A code, and the `dynamic` site that mints it.** The list holds that site under no
/// family, because its code is not spelled at the call; the claim says, by a person's
/// reading, that this code is constructed there.
#[derive(Clone, Copy, Debug)]
struct Claim {
    code: &'static str,
    /// The dynamic site's id as the list holds it: `file::function::dynamic#ordinal`.
    minted_at: &'static str,
}

impl Claim {
    /// The id the claimed site goes by — what a case, a reason or an owed row names it
    /// with: `file::function::<code>#dynamic-<ordinal>`. The code stands where a listed
    /// site's does, so a case reads its code the same way; `dynamic-` keeps the id apart
    /// from every listed one, whose ordinal is a number. `None` where `minted_at` is no
    /// dynamic site's id.
    fn id(&self) -> Option<String> {
        let (function, ordinal) = self.minted_at.split_once(&format!("::{DYNAMIC}#"))?;
        Some(format!("{function}::{}#{DYNAMIC}-{ordinal}", self.code))
    }
}

const fn claim(code: &'static str, minted_at: &'static str) -> Claim {
    Claim { code, minted_at }
}

/// **The codes of the catalogued families that a dynamic site mints.**
///
/// The first five rows are demanded: no listed site carries these three codes, so
/// [`every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed`] names each until a
/// row claims it. The arm asks for one row a code; the second site of `locate_finding` was
/// found by reading it.
///
/// **The other ten are kept by reading, and no arm demands them.** Their codes are listed
/// sites' codes as well, so the vocabulary arm has nothing to ask; what the two functions
/// add is a second refusal under each code, with a route of its own. A row here that
/// outlives its site or its code is named
/// ([`a_row_for_a_site_that_is_gone_is_named`]); a code one of these functions starts to
/// mint is not.
const CLAIMS: &[Claim] = &[
    // `CarryoverBoundary::Setup.code()` — a `match` in the engine hands it over.
    claim(
        "setup.dirty-install-path",
        "crates/engine/src/finalize.rs::setup_dirty_install_finding::dynamic#1",
    ),
    // `locate_finding(code, …)`, called by `setup::run` and `setup::run_uninstall`: its
    // first site answers outside a repository, its second where `$HOME` is not set.
    claim(
        "setup.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#1",
    ),
    claim(
        "setup.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#2",
    ),
    claim(
        "uninstall.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#1",
    ),
    claim(
        "uninstall.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#2",
    ),
    // By reading — `replaced_path_refusal`: the `refusal` code of each install member
    // whose writer replaces (`install_tracked_paths`), raised where the member's path, or
    // a directory on the way to it, is not a regular file's.
    claim("setup.write-bootstrap", REPLACED_PATH),
    claim("setup.version-stamp", REPLACED_PATH),
    claim("setup.init-project-layer", REPLACED_PATH),
    claim("setup.compose-marker", REPLACED_PATH),
    claim("setup.inject-allowlist", REPLACED_PATH),
    claim("setup.write-guide", REPLACED_PATH),
    // By reading — `merged_through_links`: the four codes its own body chooses among,
    // raised where a merged-into member is a link the install will not follow.
    claim("setup.inject-reference", MERGED_THROUGH),
    claim("setup.inject-allowlist", MERGED_THROUGH),
    claim("setup.secrets-gitignore", MERGED_THROUGH),
    claim("setup.init-project-layer", MERGED_THROUGH),
];

/// The two dynamic sites of `setup.rs` whose codes are listed sites' codes as well.
const REPLACED_PATH: &str = "crates/cli/src/setup.rs::replaced_path_refusal::dynamic#1";
const MERGED_THROUGH: &str = "crates/cli/src/setup.rs::merged_through_links::dynamic#1";

/// **The code-shaped literals of a catalogued prefix that are no finding's code**, each
/// with why — a file name, say. None on this tree.
const NO_CODE: &[(&str, &str)] = &[];

/// The sites one task owes a case, a reason, or both in turn.
struct Owed {
    task: &'static str,
    sites: &'static [&'static str],
}

/// **The owed list: every blocking site of a catalogued family that has neither a case nor
/// a reason yet, under the plan's task that owes it** — `P4`, the sites of `jigc uninstall`;
/// `P3` owed those of `jigc setup` and owes none. A row goes in the commit that brings its
/// site's case or reason; a row that stays beside one is named. After `P4` the list is
/// empty and is deleted, with this type and the `owed` field of [`Rows`].
const OWED: &[Owed] = &[Owed {
    task: "P4",
    sites: &[
        "crates/cli/src/setup.rs::uninstall::uninstall.remove-allowlist#1",
        "crates/cli/src/setup.rs::uninstall::uninstall.remove-deny#1",
        "crates/cli/src/setup.rs::uninstall::uninstall.remove-guide#1",
        "crates/cli/src/setup.rs::uninstall::uninstall.remove-hook#1",
        "crates/cli/src/setup.rs::uninstall::uninstall.remove-jigc#1",
        "crates/cli/src/setup.rs::uninstall::uninstall.remove-precommit#1",
        "crates/cli/src/setup.rs::uninstall::uninstall.unwire-reference#1",
        "crates/cli/src/setup.rs::untracked_workbench_finding::uninstall.untracked-workbench-file#1",
        "crates/cli/src/setup.rs::unverified_workbench_finding::uninstall.untracked-workbench-file#1",
    ],
}];

// ---------------------------------------------------------------------------
// The fence — what it reads, and what it says
// ---------------------------------------------------------------------------

/// What the fence reads of production source.
struct Read {
    /// The refusal sites, by the scan the committed list is held equal to.
    sites: Vec<Site>,
    /// Every code-shaped literal of a catalogued prefix that production code spells: its
    /// value, and where it stands.
    spelled: BTreeMap<String, BTreeSet<String>>,
}

/// Whether `code`'s prefix is one the catalog is held to.
fn catalogued(code: &str) -> bool {
    code.split_once('.')
        .is_some_and(|(prefix, _)| CATALOGUED.contains(&prefix))
}

/// Whether a literal's whole value has the shape of a catalogued family's code: the
/// prefix, a dot, and a name of lower-case letters, digits, `-`, `_` and `.`. Wider than
/// any code the product mints, so that a literal which only looks like one is asked about
/// ([`NO_CODE`]) rather than passed over.
fn code_shaped(value: &str) -> bool {
    catalogued(value)
        && value.split_once('.').is_some_and(|(_, name)| {
            !name.is_empty()
                && name.bytes().all(|b| {
                    b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'-' | b'_' | b'.')
                })
        })
}

fn read(sources: &[Source]) -> Read {
    let mut spelled: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for source in sources {
        let (code, regions) = refusal_sites::production_code(&source.body);
        for literal in rust_source::string_literals(&source.body) {
            if !code_shaped(&literal.value)
                || rust_source::is_test_domain(Path::new(&source.path), &regions, literal.offset)
            {
                continue;
            }
            // The reader's `enclosing_fn`: the last `fn` declared before the literal.
            let at = match rust_source::enclosing_fn(&code, literal.offset) {
                Some(function) => format!("{} (in `{function}`)", source.path),
                None => format!("{} (before any function)", source.path),
            };
            spelled.entry(literal.value).or_default().insert(at);
        }
    }
    Read {
        sites: refusal_sites::scan(sources),
        spelled,
    }
}

/// This tree's production source, read.
fn tree() -> Read {
    read(&refusal_sites::tree_sources())
}

/// The catalog's tables, as the fence is handed them.
#[derive(Default)]
struct Rows {
    /// The site of every case.
    cases: Vec<&'static str>,
    reasons: Vec<Reason>,
    /// `(site, task)`.
    owed: Vec<(&'static str, &'static str)>,
    claims: Vec<Claim>,
    /// `(literal, why it is no finding's code)`.
    no_code: Vec<(&'static str, &'static str)>,
}

/// The tables this suite holds.
fn rows() -> Rows {
    Rows {
        cases: cases().iter().map(|case| case.site).collect(),
        reasons: TABLES
            .iter()
            .flat_map(|table| table.reasons)
            .copied()
            .collect(),
        owed: OWED
            .iter()
            .flat_map(|owed| owed.sites.iter().map(|site| (*site, owed.task)))
            .collect(),
        claims: CLAIMS.to_vec(),
        no_code: NO_CODE.to_vec(),
    }
}

/// A site that owes the catalog a case.
struct Owing {
    id: String,
    /// The route's kind, as the list reads it at the site's call.
    route: &'static str,
}

/// A site refuses where its severity is written `blocking` — or is handed in, which the
/// list cannot read and the fence does not take for advisory.
fn blocks(site: &Site) -> bool {
    matches!(site.severity, "blocking" | DYNAMIC)
}

/// **Every site that owes a case, sorted by id**: the blocking listed sites of the
/// catalogued families, and each claimed code at the blocking dynamic site its claim names.
/// A claim that names no such site adds nothing here and is named by [`idle_rows`].
fn owing(read: &Read, claims: &[Claim]) -> Vec<Owing> {
    let mut out: Vec<Owing> = read
        .sites
        .iter()
        .filter(|site| catalogued(&site.code) && blocks(site))
        .map(|site| Owing {
            id: site.id(),
            route: site.route,
        })
        .collect();
    for claim in claims {
        let minted = read
            .sites
            .iter()
            .find(|site| site.id() == claim.minted_at && site.code == DYNAMIC && blocks(site));
        if let Some(site) = minted
            && let Some(id) = claim.id()
            && catalogued(claim.code)
        {
            out.push(Owing {
                id,
                route: site.route,
            });
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

/// **Every site that does not have exactly one answer** — a case, a reason or an owed row.
/// None is named; so is a case beside a reason, an owed row beside either, and
/// `no-route-printed` given to a site whose route the list reads at its call.
fn uncovered(read: &Read, rows: &Rows) -> Vec<String> {
    let mut out = Vec::new();
    for site in owing(read, &rows.claims) {
        let id = &site.id;
        let case = rows.cases.contains(&id.as_str());
        let reason = rows.reasons.iter().find(|reason| reason.site == id);
        let owed = rows.owed.iter().find(|(owed, _)| owed == id);
        match (case, reason, owed) {
            (false, None, None) => {
                out.push(format!("`{id}` has no case, no reason and no owed row"))
            }
            (true, Some(reason), _) => out.push(format!(
                "`{id}` has a case and the reason `{}` for having none — one of the two is \
                 wrong",
                reason.why
            )),
            (true, None, Some((_, task))) | (false, Some(_), Some((_, task))) => out.push(format!(
                "`{id}` has a {} and is still owed by `{task}` — the row goes in the commit \
                 that brings the answer",
                if case { "case" } else { "reason" }
            )),
            (false, Some(reason), None)
                if matches!(reason.why, Why::NoRoutePrinted)
                    && matches!(
                        site.route,
                        "prose" | "human" | "mechanical" | "informational"
                    ) =>
            {
                out.push(format!(
                    "`{id}` is given `{}`, and the list reads a `{}` route written at its \
                     call",
                    reason.why, site.route
                ))
            }
            _ => {}
        }
    }
    out
}

/// **Every code-shaped literal of a catalogued prefix that is nobody's** — the code of no
/// listed site, claimed by no row, and with no row that says why it is no finding's code.
/// A literal that has such a row *and* is a site's code or a claim's is named too.
fn unclaimed(read: &Read, rows: &Rows) -> Vec<String> {
    let mut out = Vec::new();
    for (code, at) in &read.spelled {
        let at = at.iter().map(String::as_str).collect::<Vec<_>>().join(", ");
        let listed = read.sites.iter().any(|site| site.code == *code);
        let claimed = rows.claims.iter().any(|claim| claim.code == code);
        let excused = rows.no_code.iter().any(|(literal, _)| literal == code);
        match (listed || claimed, excused) {
            (false, false) => out.push(format!(
                "`{code}` — spelled in {at} — is the code of no listed site, no row claims \
                 it for the dynamic site that mints it, and no row says why it is no \
                 finding's code"
            )),
            (true, true) => out.push(format!(
                "`{code}` has a row that says it is no finding's code, and it is {}",
                if listed {
                    "the code of a listed site"
                } else {
                    "claimed for a dynamic site"
                }
            )),
            _ => {}
        }
    }
    out
}

/// **Every row that is about nothing**: an owed row or a reason whose site owes no case —
/// gone, renamed, or never blocking — a claim whose site the list does not hold as a
/// dynamic one or whose code no production source spells, a row for a literal that is no
/// longer spelled; and a row that is there twice, or says nothing where it must say why.
fn idle_rows(read: &Read, rows: &Rows) -> Vec<String> {
    let owing = owing(read, &rows.claims);
    let owes = |id: &str| owing.iter().any(|site| site.id == id);
    let mut out = Vec::new();

    for (at, (site, task)) in rows.owed.iter().enumerate() {
        if task.is_empty() {
            out.push(format!("the owed row for `{site}` names no task"));
        }
        if !owes(site) {
            out.push(format!(
                "the owed list (`{task}`) names `{site}`, which is no blocking site of a \
                 catalogued family: it is gone, renamed, or was never one"
            ));
        } else if rows.owed[..at].iter().any(|(other, _)| other == site) {
            out.push(format!("`{site}` is on the owed list twice"));
        }
    }

    for (at, reason) in rows.reasons.iter().enumerate() {
        let site = reason.site;
        if let Why::NotConstructible("") | Why::Platform("") = reason.why {
            out.push(format!(
                "the reason `{}` for `{site}` states nothing",
                reason.why
            ));
        }
        if !owes(site) {
            out.push(format!(
                "the reason `{}` names `{site}`, which is no blocking site of a catalogued \
                 family: it is gone, renamed, or was never one",
                reason.why
            ));
        } else if rows.reasons[..at].iter().any(|other| other.site == site) {
            out.push(format!("`{site}` is given two reasons"));
        }
    }

    for (at, claim) in rows.claims.iter().enumerate() {
        let Claim { code, minted_at } = *claim;
        match read.sites.iter().find(|site| site.id() == minted_at) {
            None => out.push(format!(
                "the claim of `{code}` names `{minted_at}`, which the list does not hold"
            )),
            Some(site) if site.code != DYNAMIC => out.push(format!(
                "the claim of `{code}` names `{minted_at}`, which is no dynamic site — its \
                 code is spelled at the call"
            )),
            Some(_) => {}
        }
        if !read.spelled.contains_key(code) {
            out.push(format!(
                "the claim of `{code}` for `{minted_at}`: no production source spells that \
                 code under a catalogued prefix"
            ));
        }
        if rows.claims[..at]
            .iter()
            .any(|other| other.code == code && other.minted_at == minted_at)
        {
            out.push(format!("`{code}` is claimed twice for `{minted_at}`"));
        }
    }

    for (at, (literal, why)) in rows.no_code.iter().enumerate() {
        if why.is_empty() {
            out.push(format!(
                "the row for the literal `{literal}` does not say why it is no finding's code"
            ));
        }
        if !read.spelled.contains_key(*literal) {
            out.push(format!(
                "a row says the literal `{literal}` is no finding's code, and no production \
                 source spells it"
            ));
        } else if rows.no_code[..at].iter().any(|(other, _)| other == literal) {
            out.push(format!("the literal `{literal}` has two rows"));
        }
    }
    out
}

/// The files of `on_disk` that are no row of `wired`.
fn unwired(on_disk: &[String], wired: &[&str]) -> Vec<String> {
    on_disk
        .iter()
        .filter(|file| !wired.contains(&file.as_str()))
        .map(|file| {
            format!(
                "`{file}` is declared by no line inside `tables!`, so the catalog follows \
                 none of its cases and the fence reads none of its reasons"
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The fence — its arms
// ---------------------------------------------------------------------------

/// A production path for a fixture: under a crate's `src/`, so the reader's test-domain
/// rule leaves it to the `#[cfg(test)]` regions alone.
const FIXTURE: &str = "crates/cli/src/fixture.rs";

/// A production file in small. Of two catalogued families it holds a blocking site with a
/// route written at the call, a blocking one that prints none, an advisory one, and a code
/// that reaches its constructor through a `match`; beside them another family's site and
/// code, a literal that only looks like a code, and a code that only a test spells.
const REFUSALS: &str = r#"
use engine::finding::{Finding, Severity};

fn refuses() -> Finding {
    Finding::block("setup.spelled", "m", "re-run `jigc setup`")
}

fn routeless() -> Finding {
    Finding::blocking("uninstall.routeless", "m", None)
}

fn advises() -> Finding {
    Finding::graded(Severity::Advisory, "setup.advised", "m", None, None)
}

fn another_familys() -> Finding {
    Finding::block("task.spelled", "m", "r")
}

fn code(door: Door) -> &'static str {
    match door {
        Door::Setup => "setup.by-a-match",
        Door::Task => "task.by-a-match",
    }
}

fn minted(door: Door) -> Finding {
    Finding::block(code(door), "m", "r")
}

fn a_file_name() -> &'static str {
    "setup.rs"
}

#[cfg(test)]
mod tests {
    #[test]
    fn asserts() {
        assert_eq!(minted(Door::Setup).code, "setup.only-a-test-spells-it");
    }
}
"#;

const SPELLED: &str = "crates/cli/src/fixture.rs::refuses::setup.spelled#1";
const ROUTELESS: &str = "crates/cli/src/fixture.rs::routeless::uninstall.routeless#1";
const ADVISED: &str = "crates/cli/src/fixture.rs::advises::setup.advised#1";
const MINTED: &str = "crates/cli/src/fixture.rs::minted::dynamic#1";
const CLAIMED: &str = "crates/cli/src/fixture.rs::minted::setup.by-a-match#dynamic-1";
const GONE: &str = "crates/cli/src/fixture.rs::removed::setup.removed#1";

fn fixture() -> Read {
    read(&[Source::new(FIXTURE, REFUSALS)])
}

/// The fixture's one claim.
const BY_A_MATCH: Claim = claim("setup.by-a-match", MINTED);

/// The fixture's tables with nothing wrong: a case, a reason and an owed row, one site
/// each; the claim; and the literal that is no code.
fn in_order() -> Rows {
    Rows {
        cases: vec![SPELLED],
        reasons: vec![Reason {
            site: ROUTELESS,
            why: Why::NoRoutePrinted,
        }],
        owed: vec![(CLAIMED, "P3")],
        claims: vec![BY_A_MATCH],
        no_code: vec![("setup.rs", "a file's name")],
    }
}

/// `said` is one line for each of `expected`, in order, and each line holds its two
/// pieces: what it is about, and what it says of it.
fn says(said: &[String], expected: &[(&str, &str)]) {
    assert_eq!(said.len(), expected.len(), "{said:#?}");
    for (line, (about, what)) in said.iter().zip(expected) {
        assert!(
            line.contains(about) && line.contains(what),
            "a line about `{about}` that says `{what}`: {said:#?}",
        );
    }
}

/// **Every blocking site of the catalogued families has a case, a reason or an owed row —
/// exactly one.** On a fixture: the advisory site and the other family's owe nothing, a
/// claim makes the code at its dynamic site owe like a listed one, and two answers for one
/// site are named as none is. Then this tree, against the tables this suite holds.
#[test]
fn every_refusal_site_of_the_catalogued_families_has_a_case_a_reason_or_an_owed_row() {
    let fixture = fixture();
    says(
        &uncovered(&fixture, &Rows::default()),
        &[(SPELLED, "no case"), (ROUTELESS, "no case")],
    );
    says(
        &uncovered(
            &fixture,
            &Rows {
                claims: vec![BY_A_MATCH],
                ..Rows::default()
            },
        ),
        &[
            (CLAIMED, "no case"),
            (SPELLED, "no case"),
            (ROUTELESS, "no case"),
        ],
    );
    says(&uncovered(&fixture, &in_order()), &[]);
    for why in [
        Why::NotConstructible("the profile is compiled in"),
        Why::Platform("a filesystem without links"),
    ] {
        let reasons = vec![Reason {
            site: ROUTELESS,
            why,
        }];
        says(
            &uncovered(
                &fixture,
                &Rows {
                    reasons,
                    ..in_order()
                },
            ),
            &[],
        );
    }

    says(
        &uncovered(
            &fixture,
            &Rows {
                owed: vec![(SPELLED, "P3"), (ROUTELESS, "P4"), (CLAIMED, "P3")],
                ..in_order()
            },
        ),
        &[
            (SPELLED, "a case and is still owed by `P3`"),
            (ROUTELESS, "a reason and is still owed by `P4`"),
        ],
    );
    let both = Reason {
        site: SPELLED,
        why: Why::NotConstructible("nothing reaches it"),
    };
    says(
        &uncovered(
            &fixture,
            &Rows {
                reasons: vec![both, in_order().reasons[0]],
                ..in_order()
            },
        ),
        &[(
            SPELLED,
            "a case and the reason `not-constructible: nothing reaches it`",
        )],
    );
    // `no-route-printed` is taken from the site that prints none, and from no other.
    let routeless = |site| Reason {
        site,
        why: Why::NoRoutePrinted,
    };
    says(
        &uncovered(
            &fixture,
            &Rows {
                cases: vec![],
                reasons: vec![routeless(SPELLED), routeless(ROUTELESS)],
                ..in_order()
            },
        ),
        &[(SPELLED, "a `prose` route written at its call")],
    );

    let read = tree();
    let wrong = uncovered(&read, &rows());
    assert!(
        wrong.is_empty(),
        "the route catalog does not answer for {} of the {} blocking refusal sites of the \
         families {CATALOGUED:?}:\n\n  {}\n\na site is answered by a case in its command's \
         file under `route_catalog/`, by a reason there, or — until the task that owes it \
         lands — by a row of `OWED`; a code that a dynamic site mints is a site once \
         `CLAIMS` holds it.",
        wrong.len(),
        owing(&read, CLAIMS).len(),
        wrong.join("\n  "),
    );
}

/// **The vocabulary is held, not only the call sites.** On a fixture: the code a `match`
/// hands to a constructor is named until a row claims it, a literal that only looks like a
/// code until a row says so, and neither another family's code nor a test's is asked about.
/// Then this tree.
#[test]
fn every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed() {
    let fixture = fixture();
    assert_eq!(
        fixture.spelled.keys().collect::<Vec<_>>(),
        [
            "setup.advised",
            "setup.by-a-match",
            "setup.rs",
            "setup.spelled",
            "uninstall.routeless",
        ],
        "every code-shaped literal of a catalogued prefix in production code, and no other",
    );
    says(
        &unclaimed(&fixture, &Rows::default()),
        &[
            ("`setup.by-a-match`", "fixture.rs (in `code`)"),
            ("`setup.rs`", "fixture.rs (in `a_file_name`)"),
        ],
    );
    says(&unclaimed(&fixture, &in_order()), &[]);
    says(
        &unclaimed(
            &fixture,
            &Rows {
                no_code: vec![
                    ("setup.by-a-match", "it is one"),
                    ("setup.rs", "a file's name"),
                    ("setup.spelled", "it is one"),
                ],
                ..in_order()
            },
        ),
        &[
            ("`setup.by-a-match`", "claimed for a dynamic site"),
            ("`setup.spelled`", "the code of a listed site"),
        ],
    );

    let wrong = unclaimed(&tree(), &rows());
    assert!(
        wrong.is_empty(),
        "production source spells {} code(s) of the families {CATALOGUED:?} that the list \
         of refusal sites does not hold and nothing answers for:\n\n  {}\n\nread where each \
         is handed to a constructor. A `dynamic` site of \
         `crates/cli/tests/route_catalog/sites.tsv` mints it: a row of `CLAIMS` names that \
         site, and the code there then owes a case. It is no finding's code: a row of \
         `NO_CODE` says why.",
        wrong.len(),
        wrong.join("\n  "),
    );
}

/// **A row for a site that is gone is named** — and so is every other row that is about
/// nothing: one for a site that never owed a case, a claim for a site that is not dynamic
/// or a code nothing spells, a row that is there twice, a row that does not say why. On a
/// fixture, kind by kind; then this tree's tables.
#[test]
fn a_row_for_a_site_that_is_gone_is_named() {
    let fixture = fixture();
    says(&idle_rows(&fixture, &in_order()), &[]);

    says(
        &idle_rows(
            &fixture,
            &Rows {
                owed: vec![
                    (CLAIMED, "P3"),
                    (GONE, "P3"),
                    (ADVISED, "P3"),
                    (CLAIMED, "P4"),
                    (ROUTELESS, ""),
                ],
                ..in_order()
            },
        ),
        &[
            (GONE, "the owed list (`P3`) names"),
            (ADVISED, "no blocking site of a catalogued family"),
            (CLAIMED, "on the owed list twice"),
            (ROUTELESS, "names no task"),
        ],
    );

    let reason = |site, why| Reason { site, why };
    says(
        &idle_rows(
            &fixture,
            &Rows {
                reasons: vec![
                    reason(ROUTELESS, Why::NoRoutePrinted),
                    reason(GONE, Why::Platform("another")),
                    reason(ROUTELESS, Why::NotConstructible("")),
                ],
                ..in_order()
            },
        ),
        &[
            (GONE, "the reason `platform: another` names"),
            (ROUTELESS, "states nothing"),
            (ROUTELESS, "is given two reasons"),
        ],
    );

    // A claim that names no dynamic site makes no site, so the row owed under it is idle
    // with it.
    says(
        &idle_rows(
            &fixture,
            &Rows {
                claims: vec![claim("setup.by-a-match", GONE)],
                ..in_order()
            },
        ),
        &[
            (CLAIMED, "the owed list (`P3`) names"),
            (GONE, "which the list does not hold"),
        ],
    );
    says(
        &idle_rows(
            &fixture,
            &Rows {
                claims: vec![
                    BY_A_MATCH,
                    claim("setup.by-a-match", SPELLED),
                    claim("setup.nothing-spells-it", MINTED),
                    BY_A_MATCH,
                ],
                ..in_order()
            },
        ),
        &[
            (SPELLED, "is no dynamic site"),
            ("`setup.nothing-spells-it`", "no production source spells"),
            ("`setup.by-a-match`", "is claimed twice"),
        ],
    );

    says(
        &idle_rows(
            &fixture,
            &Rows {
                no_code: vec![
                    ("setup.rs", ""),
                    ("setup.md", "a file's name"),
                    ("setup.rs", "a file's name"),
                ],
                ..in_order()
            },
        ),
        &[
            ("`setup.rs`", "does not say why"),
            ("`setup.md`", "no production source spells it"),
            ("`setup.rs`", "has two rows"),
        ],
    );

    let wrong = idle_rows(&tree(), &rows());
    assert!(
        wrong.is_empty(),
        "the route catalog's tables hold {} row(s) that are about nothing:\n\n  {}\n\na row \
         goes with what it was about. A site that was renamed keeps its row under the new \
         id, which `crates/cli/tests/route_catalog/sites.tsv` holds.",
        wrong.len(),
        wrong.join("\n  "),
    );
}

/// **A command's file that is on disk is one the suite reads.** The registration fence
/// holds every `.rs` file under `route_catalog/` to a `#[path]` line in this file; this
/// arm holds it to a line *inside* `tables!`, where its cases are followed and its reasons
/// are read. A declaration beside the macro satisfies the first and is named here.
#[test]
fn every_file_of_the_catalogs_directory_is_a_table_the_suite_reads() {
    let wired: Vec<&str> = TABLES.iter().map(|table| table.file).collect();
    let planted = [wired[0].to_string(), "route_catalog/beside.rs".to_string()];
    says(
        &unwired(&planted, &wired),
        &[("`route_catalog/beside.rs`", "no line inside `tables!`")],
    );

    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let on_disk: Vec<String> = rust_source::rust_files(&tests.join("route_catalog"))
        .iter()
        .map(|path| {
            path.strip_prefix(&tests)
                .expect("a walked file is under the suite home")
                .components()
                .map(|part| part.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        })
        .collect();
    let wrong = unwired(&on_disk, &wired);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **The fence is not vacuous.** Each of its arms is green over an empty set, so each set
/// is shown not to be empty on this tree, and the fence is run once with its tables taken
/// away:
///
///   * every catalogued prefix is a family of the list and the prefix of a listed site;
///   * the vocabulary's reader sees what the scan sees — the code of every listed site of a
///     catalogued family that is spelled at its call is among the literals, in that file;
///   * handed no case, no reason and no owed row, the fence names every site that owes
///     one, listed and claimed, and there are both;
///   * handed no claim, it names every code no listed site carries.
#[test]
fn the_fence_is_not_vacuous() {
    let read = tree();
    for prefix in CATALOGUED {
        assert!(
            FAMILIES.iter().any(|(family, _)| family == prefix),
            "`{prefix}` is no row of the list's family table",
        );
        assert!(
            read.sites
                .iter()
                .any(|site| site.code.split_once('.').map(|(family, _)| family) == Some(*prefix)),
            "the list holds no site of the family `{prefix}`",
        );
    }

    for site in &read.sites {
        if catalogued(&site.code) && site.written == "literal" {
            assert!(
                read.spelled
                    .get(&site.code)
                    .is_some_and(|at| at.iter().any(|at| at.starts_with(&site.file))),
                "`{}` spells its code at the call, and the vocabulary's reader did not find \
                 that literal in `{}`",
                site.id(),
                site.file,
            );
        }
    }

    let bare = Rows {
        claims: CLAIMS.to_vec(),
        ..Rows::default()
    };
    let owing = owing(&read, CLAIMS);
    let claimed = owing
        .iter()
        .filter(|site| site.id.contains(&format!("#{DYNAMIC}-")))
        .count();
    assert!(
        claimed > 0 && claimed < owing.len(),
        "{} sites owe a case, {claimed} of them claimed: both kinds are there",
        owing.len(),
    );
    let named = uncovered(&read, &bare);
    assert_eq!(
        named.len(),
        owing.len(),
        "handed no case, no reason and no owed row, the fence names every site that owes \
         one: {named:#?}",
    );

    let unlisted: BTreeSet<&str> = CLAIMS
        .iter()
        .map(|claim| claim.code)
        .filter(|code| !read.sites.iter().any(|site| site.code == *code))
        .collect();
    let named = unclaimed(
        &read,
        &Rows {
            no_code: NO_CODE.to_vec(),
            ..Rows::default()
        },
    );
    assert!(
        !unlisted.is_empty(),
        "no claim is for a code the list lacks"
    );
    assert_eq!(
        named.len(),
        unlisted.len(),
        "handed no claim, the fence names every code no listed site carries: {named:#?}",
    );
    for code in unlisted {
        assert!(
            named.iter().any(|line| line.contains(&format!("`{code}`"))),
            "`{code}` is not named: {named:#?}",
        );
    }
}

// ---------------------------------------------------------------------------
// The frame's rules
// ---------------------------------------------------------------------------

/// The body of the item `opening` opens in `code` — between its brace and the matching
/// one.
fn body_of<'a>(code: &'a str, opening: &str) -> &'a str {
    let start = code
        .find(opening)
        .unwrap_or_else(|| panic!("the frame declares `{opening}`"))
        + opening.len();
    let mut depth = 1;
    for (at, byte) in code[start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return &code[start..start + at];
        }
    }
    panic!("`{opening}` is never closed");
}

/// The variants of an enum body, as written.
fn variants(body: &str) -> Vec<String> {
    body.split(',')
        .map(|variant| variant.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|variant| !variant.is_empty())
        .collect()
}

/// This file and every command's, each with its source — what the arms that read the
/// suite's own source read.
fn suite_files() -> Vec<(&'static str, &'static str)> {
    std::iter::once(("route_catalog.rs", include_str!("route_catalog.rs")))
        .chain(TABLES.iter().map(|table| (table.file, table.source)))
        .collect()
}

/// **No act takes a path, and the marked form is refused where a path is printed.**
///
/// The source half reads the frame, this file and every command's. An [`Act`] carries an
/// [`Operand`], a span's index or nothing at all — never a path, a word or a command of
/// the case's own; an operand is a span's index or the one
/// marked form; a [`Case`] holds no function, so it cannot act around the runner; and
/// neither this file nor a command's names a filesystem or a process API at all — what the
/// existing test did by hand (`fs::remove_file` on a path it spelled) has no place to be
/// written.
///
/// The planted half is that test's step, as a case: the first case's state, with the link's
/// own path given by a constant. The printed route holds a path, so the runner refuses the
/// case before it does anything — with that refusal switched off it comes out *followed,
/// holds, operand implied*, which is the green the existing test has.
#[test]
fn no_act_takes_a_path_and_the_marked_form_is_refused_where_a_path_is_printed() {
    let frame = rust_source::code_only(include_str!("support/route_case.rs"));
    for variant in variants(body_of(&frame, "pub enum Act {")) {
        assert!(
            variant.ends_with("(Operand)")
                || variant.ends_with("(usize)")
                || !variant.contains('('),
            "the act `{variant}` takes something that is neither an operand nor a span's index",
        );
    }
    assert_eq!(
        variants(body_of(&frame, "pub enum Operand {")),
        ["Span(usize)", "Unprinted(&'static str)"],
        "an operand is a span of the printed route by index, or the one marked form",
    );
    let case = body_of(&frame, "pub struct Case {");
    for callable in ["fn(", "Fn", "dyn ", "Box<"] {
        assert!(
            !case.contains(callable),
            "a case is data: `{callable}` in `Case` lets one act around the runner",
        );
    }
    for (file, source) in suite_files() {
        let code = rust_source::code_only(source);
        for by_hand in ["fs::", "std::os", "std::process", "Command", "symlink"] {
            assert!(
                !code.contains(by_hand),
                "`{file}` names `{by_hand}`: a case's every act goes through the runner, \
                 which takes its operand from the printed route",
            );
        }
    }

    let planted = Case {
        name: "planted-the-links-own-path-by-a-constant",
        acts: &[Act::Remove(Operand::Unprinted(HOOK)), Act::Run(1)],
        ..setup::HOOK_OUT[0]
    };
    match route_case::follow(binary(), &planted) {
        Err(fault) => assert!(
            fault.contains("the marked form") && fault.contains("span 0"),
            "the refusal names the marked form and the path span that forbids it: {fault}",
        ),
        Ok(followed) => panic!(
            "a case that names the link's own path by a constant was followed, where the \
             printed route names a path: {}\n{}",
            followed.classification, followed.detail
        ),
    }
}

/// **An act is not done outside the sandbox.** The runner does what a route says with a
/// path the product printed, so it must not follow one out of the case's own scratch root —
/// a route that named `/dev/null` as the link to remove is on the small canary's list. A
/// shell stands in for the binary here (the frame names none of its own) and prints a
/// refusal whose route names a path outside the sandbox twice, once absolute and once by
/// climbing out: neither is touched, and the case says why.
#[test]
fn an_act_is_not_done_outside_the_sandbox() {
    let planted = Case {
        name: "planted-a-route-that-names-paths-outside-the-sandbox",
        site: HOOK_LINK_SITE,
        cause: "a refusal whose route names `/dev/null` and a path above the sandbox",
        state: &[],
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: remove `/dev/null` and `../../elsewhere`'; exit 1",
        ],
        from: Place::Repo(""),
        env: &[],
        acts: &[Act::Remove(Operand::Span(0)), Act::Remove(Operand::Span(1))],
        holds: &[],
        witnesses: &[],
        untouched: &[],
        doubt: None,
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &planted)
        .expect("the planted case is one the runner follows");
    assert_eq!(
        (
            followed.classification,
            followed.detail.matches("not done").count()
        ),
        (Classification::AnotherFailure, 2),
        "both acts are refused, and the case is red for it:\n{}",
        followed.detail,
    );
}

/// A span is a path when it names an entry that is there, or is written as one; a flag, a
/// command line and a bare word are not. The route of the hook's write failure prints no
/// path at all — the marked form may stand there.
#[test]
fn a_span_is_a_path_when_it_names_an_entry_or_is_written_as_one() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let route = "remove `Cargo.toml` or `tests/no-such-file.rs`, not `no-such-entry`, then \
                 re-run `jigc setup` — `--force` changes nothing, nor does `/`, and this \
                 `backtick is not closed";
    let spans = route_case::spans(route);
    assert_eq!(
        spans,
        [
            "Cargo.toml",
            "tests/no-such-file.rs",
            "no-such-entry",
            "jigc setup",
            "--force",
            "/",
        ],
        "the spans are the pieces between paired backticks, in order",
    );
    assert_eq!(
        route_case::path_spans(&spans, here),
        [0, 1, 5],
        "an entry that is there, a word written as a path, an absolute path — and no other",
    );
    let writability = "ensure the repo's git hooks directory is writable, then re-run `jigc setup`";
    assert!(
        route_case::path_spans(&route_case::spans(writability), here).is_empty(),
        "a route that prints no path leaves the marked form standing",
    );
}

/// The route is the one of the finding that carries the case's code — not the first route
/// printed, and not the next finding's where its own finding prints none.
#[test]
fn a_route_is_lifted_from_the_finding_that_carries_the_code_and_from_no_other() {
    let said = "\
advisory · setup.other — something else
  route: do the other thing
blocking · setup.routeless — refused, and no way out is printed
blocking (gates at finalize) · setup.wanted — refused over `a/b`
  at: a/b
  route: remove `a/b`, then re-run `jigc setup`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";
    let route = |code| route_case::refusal_in(said, code).map(|refusal: Refusal| refusal.route);
    assert_eq!(
        route("setup.wanted"),
        Some(Some("remove `a/b`, then re-run `jigc setup`".to_string())),
    );
    assert_eq!(
        route("setup.other"),
        Some(Some("do the other thing".to_string()))
    );
    assert_eq!(
        route("setup.routeless"),
        Some(None),
        "a finding that prints no route has none — the next finding's is not its route",
    );
    assert_eq!(route("setup.absent"), None);
    assert_eq!(route("setup"), None, "a code is matched whole");
}

/// The nine classifications, and the order they are read in: nothing is *followed* that
/// was not provoked or printed no route; a changed witness outranks a repeated refusal,
/// which outranks any other failure; the marked form holds only as *operand implied*; a
/// command that did not end is said where it stopped the case — ahead of *not provoked*
/// for the refusing command, behind a changed witness and ahead of everything else for a
/// command of the route; and a printed line no shell can split is read behind those two
/// and ahead of the refusal that follows a step nobody could do.
#[test]
fn the_classifications_in_their_order() {
    let provoked = Seen {
        provoked: true,
        ..Seen::default()
    };
    let followed = Seen {
        route_printed: true,
        ..provoked
    };
    let everything = Seen {
        witness_changed: true,
        same_refusal: true,
        failed: true,
        implied: true,
        unsplit: true,
        ..followed
    };
    let table = [
        (
            Seen {
                provoked: false,
                did_not_end: true,
                ..everything
            },
            Classification::DidNotEnd,
            "the command did not end",
        ),
        (
            Seen {
                provoked: false,
                ..everything
            },
            Classification::NotProvoked,
            "not provoked",
        ),
        (
            Seen {
                route_printed: false,
                ..everything
            },
            Classification::NoRoutePrinted,
            "provoked, no route printed",
        ),
        (
            Seen {
                did_not_end: true,
                ..everything
            },
            Classification::WitnessChanged,
            "followed, a witness changed",
        ),
        (
            Seen {
                witness_changed: false,
                did_not_end: true,
                ..everything
            },
            Classification::DidNotEnd,
            "the command did not end",
        ),
        (
            Seen {
                witness_changed: false,
                ..everything
            },
            Classification::UnsplittableLine,
            "followed, a printed line no shell can split",
        ),
        (
            Seen {
                witness_changed: false,
                unsplit: false,
                ..everything
            },
            Classification::SameRefusalAgain,
            "followed, the same refusal again",
        ),
        (
            Seen {
                failed: true,
                implied: true,
                ..followed
            },
            Classification::AnotherFailure,
            "followed, another failure",
        ),
        (
            Seen {
                implied: true,
                ..followed
            },
            Classification::HoldsOperandImplied,
            "followed, holds, operand implied",
        ),
        (followed, Classification::Holds, "followed, holds"),
    ];
    for (seen, classification, words) in table {
        assert_eq!(route_case::classify(&seen), classification, "{seen:?}");
        assert_eq!(classification.as_str(), words);
        assert_eq!(
            classification.is_red(),
            !words.starts_with("followed, holds"),
            "`{words}`: only a route that holds is green",
        );
    }
}

/// The expected-red list is strict three ways — and says every difference at once.
#[test]
fn the_expected_red_list_is_strict_three_ways() {
    let entry = |case, classification| ExpectedRed {
        case,
        key: "r1-a-key",
        classification,
    };
    let listed = [entry("destroys", Classification::WitnessChanged)];
    let exact = [
        ("destroys", Classification::WitnessChanged),
        ("works", Classification::Holds),
        ("works-by-hand", Classification::HoldsOperandImplied),
    ];
    assert_eq!(
        route_case::differences(&listed, &exact),
        Vec::<String>::new(),
        "a red case listed with its classification, and two green ones: no difference",
    );

    let one = |expected: &[ExpectedRed], results: &[(&str, Classification)], says: &str| {
        let differences = route_case::differences(expected, results);
        assert_eq!(differences.len(), 1, "{says}: {differences:?}");
        assert!(differences[0].contains(says), "{says}: {differences:?}");
    };
    one(
        &listed,
        &[("destroys", Classification::Holds)],
        "is not red",
    );
    one(
        &[],
        &[("destroys", Classification::WitnessChanged)],
        "no entry lists it",
    );
    one(
        &listed,
        &[("destroys", Classification::SameRefusalAgain)],
        "its classification moved",
    );
    one(&listed, &[], "names a case that was not followed");
    one(
        &[listed[0], listed[0]],
        &[("destroys", Classification::WitnessChanged)],
        "is listed twice",
    );
    one(
        &[entry("works", Classification::Holds)],
        &[("works", Classification::Holds)],
        "which is not red",
    );

    let all = route_case::differences(
        &[
            entry("fixed", Classification::WitnessChanged),
            entry("moved", Classification::WitnessChanged),
        ],
        &[
            ("fixed", Classification::Holds),
            ("moved", Classification::NotProvoked),
            ("new", Classification::AnotherFailure),
        ],
    );
    assert_eq!(all.len(), 3, "every difference is reported: {all:?}");
}

// ---------------------------------------------------------------------------
// The readings
// ---------------------------------------------------------------------------

/// **Every way the readings and the cases disagree.** `cases` is `(site, cause)`, one a
/// case. A site with a case and no reading is named; so is a reading of a site that has no
/// case, a second reading of one site, one that does not say what was read or how it was
/// counted, a reason given for a cause a case follows, a reason that states nothing — and a
/// count that is not the causes the cases follow and the causes given a reason, together.
fn miscounted(cases: &[(&str, &str)], readings: &[Reading]) -> Vec<String> {
    let mut out = Vec::new();
    let sites: BTreeSet<&str> = cases.iter().map(|(site, _)| *site).collect();
    for site in &sites {
        if !readings.iter().any(|reading| reading.site == *site) {
            out.push(format!(
                "`{site}` has a case and no reading: which functions were read for its \
                 causes, and how many they hold"
            ));
        }
    }
    for (at, reading) in readings.iter().enumerate() {
        let site = reading.site;
        if readings[..at].iter().any(|other| other.site == site) {
            out.push(format!("`{site}` has two readings"));
            continue;
        }
        if !sites.contains(site) {
            out.push(format!(
                "the reading of `{site}` is of a site that has no case"
            ));
            continue;
        }
        if reading.read.is_empty() || reading.derivation.is_empty() {
            out.push(format!(
                "the reading of `{site}` does not say what was read, or how its causes were \
                 counted"
            ));
        }
        let followed: BTreeSet<&str> = cases
            .iter()
            .filter(|(of, _)| *of == site)
            .map(|(_, cause)| *cause)
            .collect();
        for (way, why) in reading.not_followed {
            if way.is_empty() || matches!(why, Why::NotConstructible("") | Why::Platform("")) {
                out.push(format!(
                    "`{site}`: a way out that is not followed — \"{way}\", `{why}` — does not \
                     say which, or why"
                ));
            }
        }
        for (cause, why) in reading.without_case {
            if followed.contains(cause) {
                out.push(format!(
                    "`{site}`: the cause \"{cause}\" has a case and the reason `{why}` for \
                     having none — one of the two is wrong"
                ));
            }
            if let Why::NotConstructible("") | Why::Platform("") = why {
                out.push(format!(
                    "`{site}`: the reason `{why}` for the cause \"{cause}\" states nothing"
                ));
            }
        }
        if followed.len() + reading.without_case.len() != reading.causes {
            out.push(format!(
                "`{site}`: the reading counts {} cause(s), and {} are answered — {} followed \
                 by a case, {} given a reason",
                reading.causes,
                followed.len() + reading.without_case.len(),
                followed.len(),
                reading.without_case.len(),
            ));
        }
    }
    out
}

/// **A site with a case says what was read to count its causes, and every cause counted is
/// answered** — followed by a case, or given a reason beside the cases. On a fixture, kind
/// by kind; then every command's file.
#[test]
fn every_site_with_a_case_says_what_was_read_and_every_cause_is_answered() {
    let reading = |site, causes, without_case| Reading {
        site,
        read: &["a/file.rs::the_callee"],
        causes,
        derivation: "one arm a cause",
        without_case,
        not_followed: &[],
    };
    // Two cases of one cause follow two ways out of its route, and are one cause.
    let cases = [
        ("first", "one"),
        ("first", "one"),
        ("first", "two"),
        ("second", "one"),
    ];
    const THREE: &[(&str, Why)] = &[("three", Why::Platform("another"))];
    let in_order = [reading("first", 3, THREE), reading("second", 1, &[])];
    says(&miscounted(&cases, &in_order), &[]);
    says(
        &miscounted(&cases, &[]),
        &[
            ("`first`", "a case and no reading"),
            ("`second`", "a case and no reading"),
        ],
    );
    says(
        &miscounted(
            &cases,
            &[
                in_order[0],
                in_order[1],
                in_order[0],
                reading("third", 0, &[]),
            ],
        ),
        &[
            ("`first`", "has two readings"),
            ("`third`", "a site that has no case"),
        ],
    );
    // A cause nobody answers for, and one counted that is not there.
    says(
        &miscounted(
            &cases,
            &[reading("first", 4, THREE), reading("second", 0, &[])],
        ),
        &[
            ("`first`", "counts 4 cause(s), and 3 are answered"),
            ("`second`", "counts 0 cause(s), and 1 are answered"),
        ],
    );
    const TWO_AGAIN: &[(&str, Why)] = &[("two", Why::NotConstructible(""))];
    says(
        &miscounted(
            &cases,
            &[
                reading("first", 3, TWO_AGAIN),
                Reading {
                    read: &[],
                    ..in_order[1]
                },
            ],
        ),
        &[
            ("`first`", "has a case and the reason"),
            ("`first`", "states nothing"),
            ("`second`", "does not say what was read"),
        ],
    );
    says(
        &miscounted(
            &cases,
            &[
                Reading {
                    not_followed: &[
                        ("the link committed", Why::NotConstructible("")),
                        ("the link pointed elsewhere", Why::Platform("another")),
                    ],
                    ..in_order[0]
                },
                in_order[1],
            ],
        ),
        &[("`first`", "\"the link committed\"")],
    );

    let mut wrong = Vec::new();
    for table in TABLES {
        let cases: Vec<(&str, &str)> = table
            .cases
            .iter()
            .flat_map(|group| group.iter())
            .map(|case| (case.site, case.cause))
            .collect();
        wrong.extend(miscounted(&cases, table.readings));
    }
    assert!(
        wrong.is_empty(),
        "the catalog's readings and its cases disagree:\n\n  {}\n\na site's reading stands in \
         its command's file beside the cases (`READINGS`). Two cases of one cause carry the \
         same `cause`, to the letter.",
        wrong.join("\n  "),
    );
}

// ---------------------------------------------------------------------------
// The frame's vocabulary, as the cases of `jigc setup` grew it
// ---------------------------------------------------------------------------

/// The planted cases of the three arms below. A shell stands in for the binary, as in
/// [`an_act_is_not_done_outside_the_sandbox`]: it refuses only where the state is as
/// planted, and the command its route prints exits 0 only where the act did what the route
/// says.
const PLANTED: Case = Case {
    name: "",
    site: HOOK_LINK_SITE,
    cause: "a refusal a shell prints over a planted state",
    state: &[],
    refusing: &[],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: &[],
    untouched: &[],
    doubt: None,
};

/// **A state step plants what it names, and a file act does what the route says** — every
/// step and act the cases of `jigc setup` added to the frame, each in a planted case that
/// comes out *followed, holds* only if the state stood and the act was done: a file, a
/// directory, a FIFO and a commit; a directory moved out of the repository with what it
/// holds; a link replaced by a real directory, what it led to untouched; a link pointed at
/// the repository's committed file and the change committed; a link pointed at a file that
/// is new and in no commit.
#[test]
fn a_state_step_plants_what_it_names_and_a_file_act_does_what_the_route_says() {
    let planted = [
        Case {
            name: "planted-a-file-a-directory-a-fifo-and-a-commit",
            state: &[
                Step::File(Place::Repo("a/file"), "body\n"),
                Step::Dir(Place::Repo("a/dir")),
                Step::Fifo(Place::Repo("a/pipe")),
                Step::Git(&["add", "a/file"]),
                Step::Git(&["commit", "-q", "-m", "a file"]),
            ],
            refusing: &[
                "-c",
                "grep -q body a/file && test ! -x a/file && test -d a/dir && test -p a/pipe \
                 && git diff --quiet HEAD -- a/file \
                 && git ls-files --error-unmatch a/file >/dev/null 2>&1 || exit 0; \
                 printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: remove the special file at `a/pipe`, then re-run \
                 `jigc -c \"test ! -e a/pipe\"`'; exit 1",
            ],
            acts: &[Act::Remove(Operand::Span(0)), Act::Run(1)],
            holds: &[
                Check::RegularFile(Place::Repo("a/file")),
                Check::Directory(Place::Repo("a/dir")),
            ],
            ..PLANTED
        },
        Case {
            name: "planted-a-directory-to-move-out-of-the-way",
            state: &[Step::File(Place::Repo("in/the-way/kept"), "kept\n")],
            refusing: &[
                "-c",
                "test -d in/the-way || exit 0; \
                 printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: move the directory at `in/the-way` out of the way, then re-run \
                 `jigc -c \"test ! -e in/the-way\"`'; exit 1",
            ],
            acts: &[Act::MoveAway(Operand::Span(0)), Act::Run(1)],
            holds: &[
                Check::Directory(Place::Moved("in/the-way")),
                Check::RegularFile(Place::Moved("in/the-way/kept")),
            ],
            ..PLANTED
        },
        Case {
            name: "planted-a-link-to-replace-with-a-directory",
            state: &[
                Step::File(Place::Outside("store/kept"), "kept\n"),
                Step::Link(Place::Repo("linked"), Place::Outside("store")),
            ],
            refusing: &[
                "-c",
                "test -L linked || exit 0; \
                 printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: replace the link at `linked` with a real directory, then re-run \
                 `jigc -c \"test -d linked && test ! -L linked && test ! -e linked/kept\"`'; \
                 exit 1",
            ],
            acts: &[Act::ReplaceWithDirectory(Operand::Span(0)), Act::Run(1)],
            holds: &[Check::Directory(Place::Repo("linked"))],
            witnesses: &["store/kept"],
            ..PLANTED
        },
        Case {
            name: "planted-a-link-to-point-at-the-committed-file",
            state: &[
                Step::File(Place::Outside("theirs"), "theirs\n"),
                Step::Link(Place::Repo("host"), Place::Outside("theirs")),
                Step::Git(&["add", "host"]),
                Step::Git(&["commit", "-q", "-m", "a link"]),
            ],
            refusing: &[
                "-c",
                "test -L host || exit 0; \
                 printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: point the link at `host` at a regular file inside this \
                 repository, commit the change, then re-run \
                 `jigc -c \"test -L host && test host -ef README.md \
                 && git diff --quiet HEAD -- host\"`'; exit 1",
            ],
            acts: &[
                Act::PointAtCommitted(Operand::Span(0)),
                Act::Commit(Operand::Span(0)),
                Act::Run(1),
            ],
            witnesses: &["theirs"],
            ..PLANTED
        },
        Case {
            name: "planted-a-link-to-point-at-a-new-file",
            state: &[
                Step::Link(Place::Repo("host"), Place::Outside("nothing")),
                Step::Git(&["add", "host"]),
                Step::Git(&["commit", "-q", "-m", "a link"]),
            ],
            refusing: &[
                "-c",
                "test -L host || exit 0; \
                 printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: point the link at `host` at a regular file inside this \
                 repository, then re-run \
                 `jigc -c \"test -L host && test -f host && ! test host -ef README.md \
                 && git ls-files --others --exclude-standard | grep -q .\"`'; exit 1",
            ],
            acts: &[Act::PointAtNew(Operand::Span(0)), Act::Run(1)],
            witnesses: &["nothing"],
            ..PLANTED
        },
    ];
    let mut wrong = Vec::new();
    for case in planted.iter().chain(&PLANTED_LATER) {
        match route_case::follow(Path::new("/bin/sh"), case) {
            Ok(followed) if followed.classification == Classification::Holds => {}
            Ok(followed) => wrong.push(format!(
                "`{}` — {}\n{}",
                case.name, followed.classification, followed.detail
            )),
            Err(fault) => wrong.push(format!("`{}` cannot be followed: {fault}", case.name)),
        }
    }
    assert!(
        wrong.is_empty(),
        "a planted case comes out *followed, holds* where its state stood and its acts were \
         done:\n\n{}",
        wrong.join("\n\n"),
    );
}

/// The planted cases of the vocabulary the hook's site and the seeded `.gitignore` added: a
/// link whose target is spelled to the letter; a link pointed at the repository's script —
/// one the runner writes, and one the case's state planted; and a path git ignores.
const PLANTED_LATER: [Case; 4] = [
    Case {
        name: "planted-a-link-spelled-relatively",
        state: &[
            Step::File(Place::Outside("theirs"), "theirs\n"),
            Step::LinkTo(Place::Repo("relative"), "../outside/theirs"),
        ],
        refusing: &[
            "-c",
            "test -L relative && test -f relative \
             && test \"$(readlink relative)\" = ../outside/theirs || exit 0; \
             printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: remove the link at `relative`, then re-run \
             `jigc -c \"test ! -L relative\"`'; exit 1",
        ],
        acts: &[Act::Remove(Operand::Span(0)), Act::Run(1)],
        witnesses: &["theirs"],
        ..PLANTED
    },
    Case {
        name: "planted-a-link-to-point-at-a-script-the-runner-writes",
        state: &[Step::Link(Place::Repo("hook"), Place::Outside("nothing"))],
        refusing: &[
            "-c",
            "test -L hook || exit 0; \
             printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: point the link at `hook` at a script inside this repository, then \
             re-run `jigc -c \"test -L hook && test -x hook && ./hook\"`'; exit 1",
        ],
        acts: &[Act::PointAtScript(Operand::Span(0)), Act::Run(1)],
        witnesses: &["nothing"],
        ..PLANTED
    },
    Case {
        name: "planted-a-link-to-point-at-the-script-the-state-planted",
        state: &[
            Step::Script(Place::Repo(route_case::SCRIPT), "#!/bin/sh\nexit 7\n"),
            Step::Link(Place::Repo("hook"), Place::Outside("nothing")),
        ],
        refusing: &[
            "-c",
            "test -L hook || exit 0; \
             printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: point the link at `hook` at a script inside this repository, then \
             re-run `jigc -c \"test -L hook && test -x hook && ! ./hook\"`'; exit 1",
        ],
        acts: &[Act::PointAtScript(Operand::Span(0)), Act::Run(1)],
        witnesses: &["nothing"],
        ..PLANTED
    },
    Case {
        name: "planted-a-path-git-ignores",
        state: &[Step::File(Place::Repo(".gitignore"), ".env\n")],
        refusing: &[
            "-c",
            "test -f .gitignore || exit 0; \
             printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: re-run `jigc -c true`'; exit 1",
        ],
        acts: &[Act::Run(0)],
        holds: &[Check::GitIgnores(".env")],
        ..PLANTED
    },
];

/// **A check that git ignores a path fails where git does not** — the other half of the
/// planted case above, so that the check is not one that always holds.
#[test]
fn a_path_git_does_not_ignore_does_not_hold() {
    let planted = Case {
        name: "planted-a-path-git-does-not-ignore",
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: re-run `jigc -c true`'; exit 1",
        ],
        acts: &[Act::Run(0)],
        holds: &[Check::GitIgnores(".env")],
        ..PLANTED
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &planted)
        .expect("the planted case is one the runner follows");
    assert!(
        followed.classification == Classification::AnotherFailure
            && followed.detail.contains("git does not ignore `.env`"),
        "{}:\n{}",
        followed.classification,
        followed.detail,
    );
}

/// Every act that takes an operand, with the marked form naming the directory the planted
/// refusals below are about.
const MARKED: [&[Act]; 11] = [
    &[Act::Remove(Operand::Unprinted("in/the-way"))],
    &[Act::MoveAway(Operand::Unprinted("in/the-way"))],
    &[Act::MoveOut(Operand::Unprinted("in/the-way"))],
    &[Act::ReplaceWithDirectory(Operand::Unprinted("in/the-way"))],
    &[Act::PointAtCommitted(Operand::Unprinted("in/the-way"))],
    &[Act::PointAtNew(Operand::Unprinted("in/the-way"))],
    &[Act::PointAtScript(Operand::Unprinted("in/the-way"))],
    &[Act::Commit(Operand::Unprinted("in/the-way"))],
    &[Act::Mend(Operand::Unprinted("in/the-way"))],
    &[Act::MakeReadable(Operand::Unprinted("in/the-way"))],
    &[Act::MakeWritable(Operand::Unprinted("in/the-way"))],
];

/// **The marked form is refused for every act that takes an operand, wherever the printed
/// route holds a path** — not for the removal alone, which was the one act the frame had
/// when the rule was written. And where the route prints none, an act in the marked form
/// leaves the case no better than *operand implied*, whichever act it is.
#[test]
fn the_marked_form_is_refused_for_every_act_that_takes_an_operand() {
    let prints_the_path = Case {
        name: "planted-the-marked-form-beside-a-printed-path",
        state: &[Step::Dir(Place::Repo("in/the-way"))],
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: move the directory at `in/the-way` out of the way'; exit 1",
        ],
        ..PLANTED
    };
    for acts in MARKED {
        match route_case::follow(
            Path::new("/bin/sh"),
            &Case {
                acts,
                ..prints_the_path
            },
        ) {
            Err(fault) => assert!(
                fault.contains("the marked form") && fault.contains("span 0"),
                "{acts:?}: the refusal names the marked form and the path span that forbids \
                 it: {fault}",
            ),
            Ok(followed) => panic!(
                "{acts:?} names its object by the marked form and was followed, where the \
                 printed route names a path: {}\n{}",
                followed.classification, followed.detail
            ),
        }
    }

    let prints_no_path = Case {
        name: "planted-the-marked-form-where-no-path-is-printed",
        state: &[Step::File(Place::Repo("in/the-way/kept"), "kept\n")],
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: move what is in the way out of it, then re-run \
             `jigc -c \"test ! -e in/the-way\"`'; exit 1",
        ],
        acts: &[Act::MoveAway(Operand::Unprinted("in/the-way")), Act::Run(0)],
        holds: &[Check::RegularFile(Place::Moved("in/the-way/kept"))],
        ..PLANTED
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &prints_no_path)
        .expect("where the route prints no path the marked form stands");
    assert_eq!(
        followed.classification,
        Classification::HoldsOperandImplied,
        "{}",
        followed.detail,
    );
}

/// **No file act is done outside the sandbox** — the rule
/// [`an_act_is_not_done_outside_the_sandbox`] holds for the removal, for each act the cases
/// of `jigc setup` added: a route that names a path above the sandbox, or `/dev/null`, has
/// none of them done there, and nothing is written for the act that would write a file.
#[test]
fn no_file_act_is_done_outside_the_sandbox() {
    let planted = Case {
        name: "planted-file-acts-on-paths-outside-the-sandbox",
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: move `../../elsewhere` and replace `/dev/null`'; exit 1",
        ],
        acts: &[
            Act::MoveAway(Operand::Span(0)),
            Act::MoveOut(Operand::Span(0)),
            Act::ReplaceWithDirectory(Operand::Span(1)),
            Act::PointAtCommitted(Operand::Span(1)),
            Act::PointAtNew(Operand::Span(1)),
            Act::PointAtScript(Operand::Span(1)),
            Act::Commit(Operand::Span(0)),
            Act::Mend(Operand::Span(1)),
            Act::MakeReadable(Operand::Span(1)),
            Act::MakeWritable(Operand::Span(1)),
        ],
        ..PLANTED
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &planted)
        .expect("the planted case is one the runner follows");
    assert_eq!(
        (
            followed.classification,
            followed
                .detail
                .matches("not done — it names nothing inside")
                .count()
        ),
        (Classification::AnotherFailure, 10),
        "all ten acts are refused, and the case is red for it:\n{}",
        followed.detail,
    );
}

/// **A command that does not end is ended, and the case says so.** The runner waits for a
/// child no longer than its limit: a product that holds — on a read of a FIFO, say, which
/// held this suite for ten minutes the first time a case planted one where `jigc setup`
/// reads before it refuses — is a red case with its reason, and never a suite that does
/// not come back. Either way the case comes out *the command did not end*, whether it was
/// the refusing command or one the route prints. That each was ended at the limit, and did not end by itself after
/// its twenty seconds, is what *did not end* says — the runner says it of a child it ended
/// and of no other — so the arm reads no clock of its own.
#[test]
fn a_command_that_does_not_end_is_ended_and_the_case_says_so() {
    let limit = std::time::Duration::from_secs(1);
    let held_at_the_door = Case {
        name: "planted-a-refusing-command-that-does-not-end",
        refusing: &["-c", "exec sleep 20"],
        ..PLANTED
    };
    let held_on_the_route = Case {
        name: "planted-a-route-whose-command-does-not-end",
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: re-run `jigc -c \"exec sleep 20\"`'; exit 1",
        ],
        acts: &[Act::Run(0)],
        ..PLANTED
    };
    for case in [held_at_the_door, held_on_the_route] {
        let followed = route_case::follow_within(Path::new("/bin/sh"), &case, limit)
            .expect("the planted case is one the runner follows");
        assert!(
            followed.classification == Classification::DidNotEnd
                && followed.detail.contains("did not end within 1s"),
            "`{}`: the command did not end, and the detail says at which limit — it came \
             out {}:\n{}",
            case.name,
            followed.classification,
            followed.detail,
        );
    }

    // A case that is listed as one whose command does not end is waited for seconds, and
    // every other case for the standing limit.
    let listed = [
        ExpectedRed {
            case: held_at_the_door.name,
            key: "r1-a-key",
            classification: Classification::DidNotEnd,
        },
        ExpectedRed {
            case: held_on_the_route.name,
            key: "r1-a-key",
            classification: Classification::AnotherFailure,
        },
    ];
    assert_eq!(
        (
            route_case::limit_of(&listed, &held_at_the_door),
            route_case::limit_of(&listed, &held_on_the_route),
            route_case::limit_of(&[], &held_at_the_door),
        ),
        (
            route_case::EXPECTED_HOLD,
            route_case::LIMIT,
            route_case::LIMIT
        ),
    );
}

/// **The limit a caller hands the runner ends the command under test, and never the
/// sandbox's own setup.** A state step here takes two seconds by construction — a git
/// alias that sleeps — under a handed limit of one: the step is waited for, under the
/// standing limit, and the refusing command behind it is still ended at the handed one.
/// As first built, the handed limit bound every child of the sandbox, its `git init`
/// included, so the arm above failed whenever a loaded machine took a second over one.
#[test]
fn the_handed_limit_ends_the_command_under_test_and_never_the_sandboxs_setup() {
    let slow_to_plant = Case {
        name: "planted-a-state-step-slower-than-the-handed-limit",
        state: &[Step::Git(&["-c", "alias.wait=!sleep 2", "wait"])],
        refusing: &["-c", "exec sleep 20"],
        ..PLANTED
    };
    let followed = route_case::follow_within(
        Path::new("/bin/sh"),
        &slow_to_plant,
        std::time::Duration::from_secs(1),
    )
    .expect("the planted case is one the runner follows");
    assert!(
        followed.classification == Classification::DidNotEnd
            && followed.detail.contains("did not end within 1s"),
        "the state was planted and the refusing command ended at the handed limit — it came \
         out {}:\n{}",
        followed.classification,
        followed.detail,
    );
}

// ---------------------------------------------------------------------------
// The pool
// ---------------------------------------------------------------------------

/// What the jobs of the arm below count, under one lock.
#[derive(Default)]
struct Counted {
    /// How many jobs have started.
    started: usize,
    /// How many are running now, and the most that ever were.
    running: usize,
    most: usize,
    /// The threads a job ran on.
    threads: Vec<std::thread::ThreadId>,
}

/// **No more cases run at once than the pool is wide** — counted, never timed.
///
/// The catalog's arm hands its cases to [`route_case::pooled`], and as first built that
/// arm started every case at once: 138 sandboxes, each a chain of child processes, on
/// whatever machine the gate had left it. So the pool is held here by a count of its own
/// jobs. Each job says that it runs, on which thread, and then **holds until the pool is
/// full behind it** — job `k` goes on once `k + width` jobs have started — so the most
/// that run at once is no accident of scheduling: under a pool of this width it is the
/// width, exactly. Three things follow and are asserted: the jobs ran on no more threads
/// than the pool is wide (a thread runs one job at a time, so that bounds what runs at
/// once whatever the scheduler does — this is the count that is red against a runner
/// that starts a thread a case); no more ran at once than the width; and what each came
/// to is handed back in the items' order.
///
/// **Its bound, declared:** a pool that ran *fewer* at once than it was asked for would
/// hold this arm and not fail it — the first job waits for a job that never starts. The
/// arm reads no clock, so it has none to end that wait with.
///
/// And the source half: neither this file nor a command's starts a thread of its own, so
/// the catalog cannot be followed around the pool.
#[test]
fn no_more_cases_run_at_once_than_the_pool_is_wide() {
    use std::sync::{Condvar, Mutex};

    const WIDE: usize = 3;
    const JOBS: usize = 4 * WIDE;
    let counted = Mutex::new(Counted::default());
    let started = Condvar::new();
    let jobs: Vec<usize> = (0..JOBS).collect();
    let came_to = route_case::pooled(WIDE, &jobs, |job| {
        let mut counted = counted.lock().expect("no job panics under the lock");
        counted.started += 1;
        counted.running += 1;
        counted.most = counted.most.max(counted.running);
        let here = std::thread::current().id();
        if !counted.threads.contains(&here) {
            counted.threads.push(here);
        }
        started.notify_all();
        let full_behind_it = (job + WIDE).min(JOBS);
        let mut counted = started
            .wait_while(counted, |counted| counted.started < full_behind_it)
            .expect("no job panics under the lock");
        counted.running -= 1;
        job * 2
    });
    let counted = counted
        .into_inner()
        .expect("no job panicked under the lock");
    assert_eq!(
        (counted.started, counted.threads.len(), counted.most),
        (JOBS, WIDE, WIDE),
        "{JOBS} jobs through a pool {WIDE} wide: {} started, on {} thread(s), {} at once at \
         the most",
        counted.started,
        counted.threads.len(),
        counted.most,
    );
    assert_eq!(
        came_to,
        jobs.iter().map(|job| job * 2).collect::<Vec<_>>(),
        "what each job came to, in the items' order",
    );

    for (file, source) in suite_files() {
        let code = rust_source::code_only(source);
        for by_hand in ["thread::scope", "spawn("] {
            assert!(
                !code.contains(by_hand),
                "`{file}` names `{by_hand}`: a case is followed in the runner's pool, which \
                 bounds how many run at once",
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The frame's vocabulary, as the sites behind the install's first refusals grew it
// ---------------------------------------------------------------------------

/// Each planted case of `planted` comes out as the classification beside it; every one that
/// does not is said, with what it did.
fn come_out(planted: &[(Case, Classification)]) {
    let mut wrong = Vec::new();
    for (case, expected) in planted {
        match route_case::follow(Path::new("/bin/sh"), case) {
            Ok(followed) if followed.classification == *expected => {}
            Ok(followed) => wrong.push(format!(
                "`{}` — {}, where `{expected}` was planted\n{}",
                case.name, followed.classification, followed.detail
            )),
            Err(fault) => wrong.push(format!("`{}` cannot be followed: {fault}", case.name)),
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n\n"));
}

/// **A case stands where it says, in the environment it says.** The refusing command and
/// every command its route has run are started in the case's own directory — which need be
/// no repository — with the case's variables taken out of the environment or set to a place
/// of the sandbox; a path the route prints is read from where the case stands. And the
/// runner's own children are not the case's: a state's `git` step still has the variable a
/// case takes away.
#[test]
fn a_case_stands_where_it_says_in_the_environment_it_says() {
    come_out(&[
        (
            Case {
                name: "planted-a-command-run-outside-the-repository",
                state: &[Step::File(Place::Outside("in-the-way"), "theirs\n")],
                refusing: &[
                    "-c",
                    "test -e in-the-way && test ! -e .git || exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: remove `in-the-way`, then re-run \
                     `jigc -c \"test ! -e in-the-way && test ! -e .git\"`'; exit 1",
                ],
                from: Place::Outside(""),
                acts: &[Act::Remove(Operand::Span(0)), Act::Run(1)],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-a-variable-taken-out-of-the-environment",
                state: &[Step::Git(&[
                    "-c",
                    "alias.home=!env | grep -q ^HOME=",
                    "home",
                ])],
                refusing: &[
                    "-c",
                    "env | grep -q ^HOME= && exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c \"! env | grep -q ^HOME=\"`'; exit 1",
                ],
                env: &[Var::Unset("HOME")],
                acts: &[Act::Run(0)],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-a-variable-set-to-a-place-of-the-sandbox",
                state: &[Step::File(Place::Outside("there/kept"), "kept\n")],
                refusing: &[
                    "-c",
                    "test -f \"$ROUTE_CASE_PLACE/kept\" || exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c \"env | grep -q ^ROUTE_CASE_PLACE=/\"`'; exit 1",
                ],
                env: &[Var::Path("ROUTE_CASE_PLACE", Place::Outside("there"))],
                acts: &[Act::Run(0)],
                ..PLANTED
            },
            Classification::Holds,
        ),
    ]);
}

/// The two acts that run the refusing command again.
const AGAIN: [Act; 2] = [Act::Rerun, Act::RerunInTheRepository];

/// **The refusing command is run again where the route prints no command line — and only
/// there.** *Run jigc from inside the repository* and *reinstall jigc* name an act and print
/// no line to run; the act that follows them is the refusing command once more, which the
/// case already stated and the runner already ran. So a re-run takes nothing from the case:
/// no span, no path, no word. It comes out no better than *operand implied*, where it
/// stands or inside the sandbox's repository; a refusal under the case's code is *the same
/// refusal again*, as for a printed command. **And it is refused wherever the printed route
/// holds a `jigc` command line** — there the command is the route's to give, and a case
/// that ran its own in its place would be the marked form's mistake over again, with a
/// command where that had a path.
#[test]
fn the_refusing_command_is_run_again_only_where_the_route_prints_no_command_line() {
    come_out(&[
        (
            Case {
                name: "planted-a-route-that-prints-no-command-line",
                state: &[Step::File(Place::Repo("in-the-way"), "theirs\n")],
                refusing: &[
                    "-c",
                    "test -e in-the-way || { : > ran-again; exit 0; }; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: remove `in-the-way`, then run jigc again'; exit 1",
                ],
                acts: &[Act::Remove(Operand::Span(0)), Act::Rerun],
                holds: &[Check::RegularFile(Place::Repo("ran-again"))],
                ..PLANTED
            },
            Classification::HoldsOperandImplied,
        ),
        (
            Case {
                name: "planted-a-route-that-says-where-to-run-it",
                refusing: &[
                    "-c",
                    "test -e .git && { : > ran-in-the-repository; exit 0; }; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: run jigc from inside the target git repository'; exit 1",
                ],
                from: Place::Outside(""),
                acts: &[Act::RerunInTheRepository],
                holds: &[Check::RegularFile(Place::Repo("ran-in-the-repository"))],
                ..PLANTED
            },
            Classification::HoldsOperandImplied,
        ),
        (
            Case {
                name: "planted-a-route-whose-act-changes-nothing",
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: reinstall jigc'; exit 1",
                ],
                acts: &[Act::Rerun],
                ..PLANTED
            },
            Classification::SameRefusalAgain,
        ),
    ]);

    let prints_the_command = Case {
        name: "planted-a-re-run-beside-a-printed-command-line",
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: mind `--force`, then re-run `jigc -c true`'; exit 1",
        ],
        ..PLANTED
    };
    for again in &AGAIN {
        match route_case::follow(
            Path::new("/bin/sh"),
            &Case {
                acts: std::slice::from_ref(again),
                ..prints_the_command
            },
        ) {
            Err(fault) => assert!(
                fault.contains("runs the refusing command again") && fault.contains("span 1"),
                "{again:?}: the refusal names the re-run and the command line that forbids \
                 it: {fault}",
            ),
            Ok(followed) => panic!(
                "{again:?} ran the case's own command where the printed route holds a command \
                 line: {}\n{}",
                followed.classification, followed.detail
            ),
        }
    }
}

/// **Every repository of a sandbox has the sandbox's identity — one a route's own
/// `git init` makes included — and no other.** The mask leaves git no identity to find
/// and none to invent, and until this arm the runner wrote one into the one repository it
/// made itself; a repository made by a command of the route had none, and on a machine
/// whose account has a name git made one up from it. The identity is in the environment of
/// every child now, as configuration git reads from there, so it is the same in every
/// repository under the sandbox and is read from no machine. A case takes it away with the
/// variable that carries it, and then there is none: git invents nothing.
#[test]
fn a_repository_a_route_makes_has_the_sandboxs_identity_and_no_other() {
    come_out(&[
        (
            Case {
                name: "planted-a-repository-made-by-the-routes-own-git-init",
                refusing: &[
                    "-c",
                    "test -e .git && exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: `git init -q` here first, then re-run \
                     `jigc -c \"git commit -q --allow-empty -m first \
                     && git log -1 --format=%an/%ae \
                     | grep -qx Route.Case/route-case@example.com\"`'; exit 1",
                ],
                from: Place::Outside(""),
                acts: &[Act::Run(0), Act::Run(1)],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-the-identity-taken-away",
                refusing: &[
                    "-c",
                    "git commit -q --allow-empty -m no-one >/dev/null 2>&1 && exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run \
                     `jigc -c \"! git commit -q --allow-empty -m no-one\"`'; exit 1",
                ],
                env: &[Var::Unset("GIT_CONFIG_COUNT")],
                acts: &[Act::Run(0)],
                ..PLANTED
            },
            Classification::Holds,
        ),
    ]);
}

/// **A file is mended with the runner's own bytes, and an entry is made readable or
/// writable** — the acts the settings file's refusal and the first write's added. *Fix it so jigc can merge into it* says what
/// must come of the file and not what to write, so the bytes are the runner's — the empty
/// JSON object every file of its own holds — and a case supplies none; where no regular
/// file stands, nothing is made. *Ensure it is readable* gives the owner the read bit of
/// the entry itself, and *ensure it is writable* the write bit — of a directory, its own
/// and of nothing in it. (A root caller reads and writes everything: the last two planted
/// cases would not be provoked for one.)
#[test]
fn a_file_is_mended_and_an_entry_is_made_readable_or_writable() {
    come_out(&[
        (
            Case {
                name: "planted-a-file-to-mend",
                state: &[Step::File(Place::Repo("conf/settings"), "broken\n")],
                refusing: &[
                    "-c",
                    "grep -q broken conf/settings || exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: fix `conf/settings` so that it can be merged into, then re-run \
                     `jigc -c \"grep -qx {} conf/settings\"`'; exit 1",
                ],
                acts: &[Act::Mend(Operand::Span(0)), Act::Run(1)],
                holds: &[Check::RegularFile(Place::Repo("conf/settings"))],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-nothing-to-mend",
                state: &[Step::Dir(Place::Repo("conf"))],
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: fix `conf/settings`, then re-run \
                     `jigc -c \"test ! -e conf/settings\"`'; exit 1",
                ],
                acts: &[Act::Mend(Operand::Span(0)), Act::Run(1)],
                ..PLANTED
            },
            Classification::AnotherFailure,
        ),
        (
            Case {
                name: "planted-a-file-to-make-readable",
                state: &[
                    Step::File(Place::Repo("conf/shut"), "kept\n"),
                    Step::Git(&["-c", "alias.shut=!chmod 000 conf/shut", "shut"]),
                ],
                refusing: &[
                    "-c",
                    "test -r conf/shut && exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: ensure `conf/shut` is readable, then re-run \
                     `jigc -c \"grep -q kept conf/shut\"`'; exit 1",
                ],
                acts: &[Act::MakeReadable(Operand::Span(0)), Act::Run(1)],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-a-directory-to-make-writable",
                state: &[
                    Step::File(Place::Repo("shut/kept"), "kept\n"),
                    Step::Git(&[
                        "-c",
                        "alias.shut=!chmod 444 shut/kept && chmod 555 shut",
                        "shut",
                    ]),
                ],
                refusing: &[
                    "-c",
                    "test -w shut && exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: ensure `shut/` is writable, then re-run \
                     `jigc -c \": > shut/new && test ! -w shut/kept\"`'; exit 1",
                ],
                acts: &[Act::MakeWritable(Operand::Span(0)), Act::Run(1)],
                holds: &[Check::RegularFile(Place::Repo("shut/new"))],
                ..PLANTED
            },
            Classification::Holds,
        ),
    ]);
}

/// **A state step runs the binary under test** ([`Step::Jigc`]). A state an earlier run of
/// the product left — an install that stands — is made by the product, and no case copies
/// the product's lines into its state to stand in for one. A shell stands in for the
/// binary: the refusal is printed only where the step's own run left its file, and the step
/// ran in the repository.
#[test]
fn a_state_step_runs_the_binary_under_test() {
    come_out(&[(
        Case {
            name: "planted-a-state-the-binary-under-test-made",
            state: &[Step::Jigc(&[
                "-c",
                "test -f README.md && echo made > made-by-the-binary",
            ])],
            refusing: &[
                "-c",
                "grep -q made made-by-the-binary 2>/dev/null || exit 0; \
                 printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: re-run `jigc -c true`'; exit 1",
            ],
            acts: &[Act::Run(0)],
            ..PLANTED
        },
        Classification::Holds,
    )]);
}

/// **A sandbox is removed whole, whatever its state shut.** A case that plants a directory
/// nothing may be taken from — and whose route does not open it again — would otherwise
/// leave its scratch root behind on every run: the root's own removal cannot empty such a
/// directory. So the runner gives every directory of the sandbox back to its owner before
/// the root goes, and the sandbox is gone by the time the case is handed back.
#[test]
fn a_sandbox_is_removed_whole_whatever_its_state_shut() {
    let planted = Case {
        name: "planted-a-directory-nothing-may-be-taken-from",
        state: &[
            Step::File(Place::Repo("shut/deeper/kept"), "kept\n"),
            Step::Git(&[
                "-c",
                "alias.shut=!chmod 555 shut/deeper && chmod 000 shut",
                "shut",
            ]),
        ],
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: re-run `jigc -c true`'; exit 1",
        ],
        acts: &[Act::Run(0)],
        ..PLANTED
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &planted)
        .expect("the planted case is one the runner follows");
    assert_eq!(
        followed.classification,
        Classification::Holds,
        "{}",
        followed.detail
    );
    assert!(
        !followed.sandbox.exists(),
        "the sandbox of a case that shut a directory is still there once the case is \
         followed — its scratch root is left behind on every run",
    );
}

/// **A file is moved out of its path** ([`Act::MoveOut`]) — what a route asks for with
/// *move the file out of …*, which says where from and never where to: the runner owns the
/// where, as for a directory moved out of the way. The file goes to the sandbox's `moved/`
/// under the path it had, with its bytes; and what is no regular file — a directory here —
/// is not moved by this act.
#[test]
fn a_file_is_moved_out_of_its_path() {
    come_out(&[
        (
            Case {
                name: "planted-a-file-to-move-out",
                state: &[Step::File(Place::Repo("in/the-way"), "kept\n")],
                refusing: &[
                    "-c",
                    "test -f in/the-way || exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: move the file out of `in/the-way`, then re-run \
                     `jigc -c \"test ! -e in/the-way && test -d in\"`'; exit 1",
                ],
                acts: &[Act::MoveOut(Operand::Span(0)), Act::Run(1)],
                holds: &[Check::RegularFile(Place::Moved("in/the-way"))],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-a-directory-where-a-file-is-to-move-out",
                state: &[Step::File(Place::Repo("in/the-way/kept"), "kept\n")],
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: move the file out of `in/the-way`, then re-run \
                     `jigc -c \"test -f in/the-way/kept\"`'; exit 1",
                ],
                acts: &[Act::MoveOut(Operand::Span(0)), Act::Run(1)],
                ..PLANTED
            },
            Classification::AnotherFailure,
        ),
    ]);
}

// ---------------------------------------------------------------------------
// The frame, as the second command's file grew it
// ---------------------------------------------------------------------------

/// **A printed line that no shell can split is a red case, and stops no arm.** A route may
/// print between backticks a line that is no command line as it stands — the unborn dirty
/// route of `jigc setup` prints `git -C <repo> rm --cached -- <path>`, and a shell reads
/// `<path>` as a redirection with nothing behind it. As first built, the frame's reader of
/// a printed line ended the whole arm over it, and every other case of the command with
/// it. Now the line is not run, the case says what the shell said of it, the acts behind
/// it are still done — and the case comes out with a classification of its own, whatever
/// the rest of its route came to.
#[test]
fn a_printed_line_no_shell_can_split_is_a_red_case_and_stops_no_arm() {
    let planted = Case {
        name: "planted-a-route-that-prints-a-shape-where-a-command-stands",
        state: &[Step::File(Place::Repo("staged"), "mine\n")],
        refusing: &[
            "-c",
            "test -e staged || exit 0; \
             printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: `git -C . rm --cached -- <path>` first, then remove `staged` and \
             re-run `jigc -c \"test ! -e staged\"`'; exit 1",
        ],
        acts: &[Act::Run(0), Act::Remove(Operand::Span(1)), Act::Run(2)],
        ..PLANTED
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &planted)
        .expect("a line no shell can split is no fault of the case as written");
    assert!(
        followed.classification == Classification::UnsplittableLine
            && followed.detail.contains("no shell splits it into words")
            && followed
                .detail
                .contains("remove `staged` (span 1): removed")
            && followed.detail.ends_with("(span 2): exit 0"),
        "the line is not run, the acts behind it are done, and the case is red for the \
         line — it came out {}:\n{}",
        followed.classification,
        followed.detail,
    );
}

/// **No more chains run at once in one process than the machine is wide** — counted, never
/// timed. One command's arm bounds its own cases by its pool; two commands' arms in one
/// process are two pools, and a harness that runs a suite's tests as threads would have
/// twice the machine's width of chains at once. So every case passes one gate of the
/// process ([`route_case::Gate`]) before its sandbox is made.
///
/// The gate is held here by a count through one of its kind: twelve jobs arrive from
/// twelve threads at a gate three wide, each says that it has arrived before it asks to be
/// let in, and a job that is inside **holds until all twelve have arrived**. Under a gate
/// that bounds, three are inside when the last arrives and nine wait at it, so the most
/// that are ever inside is three, exactly; under one that lets everybody through, the
/// first eleven are inside when the twelfth arrives. No clock is read.
///
/// And the source half: the runner follows every case inside the process's one gate, which
/// is as wide as the pool.
#[test]
fn no_more_chains_run_at_once_in_one_process_than_the_machine_is_wide() {
    use std::sync::{Condvar, Mutex};

    const WIDE: usize = 3;
    const JOBS: usize = 4 * WIDE;
    #[derive(Default)]
    struct Counted {
        arrived: usize,
        inside: usize,
        most: usize,
    }
    let gate = route_case::Gate::new(WIDE);
    let counted = Mutex::new(Counted::default());
    let arrival = Condvar::new();
    let jobs: Vec<usize> = (0..JOBS).collect();
    route_case::pooled(JOBS, &jobs, |_| {
        counted
            .lock()
            .expect("no job panics under the lock")
            .arrived += 1;
        arrival.notify_all();
        gate.admit(|| {
            let mut counted = counted.lock().expect("no job panics under the lock");
            counted.inside += 1;
            counted.most = counted.most.max(counted.inside);
            let mut counted = arrival
                .wait_while(counted, |counted| counted.arrived < JOBS)
                .expect("no job panics under the lock");
            counted.inside -= 1;
        })
    });
    let counted = counted
        .into_inner()
        .expect("no job panicked under the lock");
    assert_eq!(
        (counted.arrived, counted.most),
        (JOBS, WIDE),
        "{JOBS} jobs at a gate {WIDE} wide: {} arrived, {} inside at once at the most",
        counted.arrived,
        counted.most,
    );

    let frame = rust_source::code_only(include_str!("support/route_case.rs"));
    assert!(
        body_of(
            &frame,
            "pub fn follow_within(binary: &Path, case: &Case, under_test: Duration) -> Result<Followed, String> {"
        )
        .contains("chains().admit("),
        "the runner follows a case inside the process's one gate",
    );
    assert!(
        body_of(&frame, "fn chains() -> &'static Gate {").contains("Gate::new(width())"),
        "the process's gate is as wide as the pool: the machine's width",
    );
}

/// **A command that removes is held to what is gone, to what still stands, and to what it
/// had no business touching** — the checks and the witnesses the cases of `jigc uninstall`
/// added. *Nothing stands there* is asked without following a link, and fails over a file
/// that is still there; *the file holds these bytes* and *no file there holds them* are
/// asked of a regular file — one that is gone lacks everything and holds nothing. And a
/// file inside the repository that a case names as untouched is a witness like one
/// outside: changed by a command of the route, or taken by the refusing command itself,
/// the case comes out *a witness changed*.
#[test]
fn what_is_gone_what_still_stands_and_what_was_not_touched() {
    come_out(&[
        (
            Case {
                name: "planted-an-entry-that-is-gone",
                state: &[Step::File(Place::Repo("in/the-way"), "theirs\n")],
                refusing: &[
                    "-c",
                    "test -e in/the-way || exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: remove `in/the-way`, then re-run `jigc -c true`'; exit 1",
                ],
                acts: &[Act::Remove(Operand::Span(0)), Act::Run(1)],
                holds: &[Check::Absent(Place::Repo("in/the-way"))],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-an-entry-that-still-stands",
                state: &[Step::File(Place::Repo("in/the-way"), "theirs\n")],
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c true`'; exit 1",
                ],
                acts: &[Act::Run(0)],
                holds: &[Check::Absent(Place::Repo("in/the-way"))],
                ..PLANTED
            },
            Classification::AnotherFailure,
        ),
        (
            Case {
                name: "planted-a-file-with-their-lines-in-it-and-ours-out",
                state: &[Step::File(Place::Repo("conf"), "mine\nours\n")],
                refusing: &[
                    "-c",
                    "grep -q ours conf || exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c \"grep -v ours conf > conf.new && mv conf.new conf\"`'; \
                     exit 1",
                ],
                acts: &[Act::Run(0)],
                holds: &[
                    Check::Contains(Place::Repo("conf"), "mine\n"),
                    Check::Lacks(Place::Repo("conf"), "ours"),
                    Check::Lacks(Place::Repo("no-such-file"), "ours"),
                ],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-a-file-their-lines-are-gone-from",
                state: &[Step::File(Place::Repo("conf"), "ours\n")],
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c true`'; exit 1",
                ],
                acts: &[Act::Run(0)],
                holds: &[Check::Contains(Place::Repo("conf"), "mine\n")],
                ..PLANTED
            },
            Classification::AnotherFailure,
        ),
        (
            Case {
                name: "planted-a-file-our-lines-are-still-in",
                state: &[Step::File(Place::Repo("conf"), "mine\nours\n")],
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c true`'; exit 1",
                ],
                acts: &[Act::Run(0)],
                holds: &[Check::Lacks(Place::Repo("conf"), "ours")],
                ..PLANTED
            },
            Classification::AnotherFailure,
        ),
        (
            Case {
                name: "planted-a-file-inside-the-repository-nobody-touched",
                state: &[Step::File(Place::Repo("kept"), "theirs\n")],
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c true`'; exit 1",
                ],
                acts: &[Act::Run(0)],
                untouched: &["kept"],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-a-file-inside-the-repository-the-route-changed",
                state: &[Step::File(Place::Repo("kept"), "theirs\n")],
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c \"echo more >> kept\"`'; exit 1",
                ],
                acts: &[Act::Run(0)],
                untouched: &["kept"],
                ..PLANTED
            },
            Classification::WitnessChanged,
        ),
        (
            Case {
                name: "planted-a-file-inside-the-repository-the-refusing-command-took",
                state: &[Step::File(Place::Repo("kept"), "theirs\n")],
                refusing: &[
                    "-c",
                    "test -e kept || exit 0; rm kept; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: re-run `jigc -c true`'; exit 1",
                ],
                acts: &[Act::Run(0)],
                untouched: &["kept"],
                ..PLANTED
            },
            Classification::WitnessChanged,
        ),
    ]);
}
