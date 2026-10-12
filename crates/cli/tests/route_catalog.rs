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
//! `route_catalog/`**, declared here by `#[path]` inside [`tables!`] — `setup.rs`,
//! `uninstall.rs` and `task_finalize.rs`.
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
//! thirty-six have a reason and no case. **And every blocking site of `jigc uninstall`,
//! eighteen** (the plan's step `P4`): the two of a command that cannot find its repository,
//! the profile not loaded, the four guards over what `.jigc/` holds the only copy of — a
//! worktree path with content, a byte jigc did not write, an open task's staged doc, a file
//! no index has — each with the refusal it raises where it could not look, and the seven
//! steps that remove. Every one of the eighteen has a case.
//!
//! **And beyond the two commands, the sites the fix pass touched** — the human's ruling of
//! 2026-10-11 (`DECISIONS.md` → [`THE_RULING_ON_FAMILIES`]): the blocking sites of every
//! other family that are new or changed since the previous release, or whose direct caller
//! changed on the path to the refusal, get their cases now, family by family, and are rows
//! of [`OWED`] until they have them; every other blocking site is a row of the scheduled
//! list ([`SCHEDULED`], `route_catalog/scheduled.tsv`), owed a case before the port begins.
//! The first of those families has its file, `task_finalize.rs` — the gate — with seven of
//! its fifteen sites so far. With them the frame grew twice, each piece red first: the
//! refusing command is run again with the flag a route prints
//! ([`the_refusing_command_is_run_again_only_where_the_route_prints_no_command_line`]),
//! and a reader goes where a route sends them, and nowhere outside the sandbox
//! ([`a_reader_goes_where_the_route_says_and_nowhere_outside_the_sandbox`]).
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
//! processes the bound is the runner's**: nextest gives each test a process, and a bound
//! inside the suite would make one arm wait for the other inside its own eight minutes,
//! which is the limit the arms were split for. So the arms stand in a test group of one
//! thread in `.config/nextest.toml`, named by their module and by no arm's own name — a
//! group waits before a test is started, which costs the test no time — and
//! [`the_arms_that_follow_cases_run_one_at_a_time_under_the_gates_runner`] holds that file
//! to it.
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
//! **And a printed line with a word in angle brackets is a shape the reader fills** — the
//! surface contract's reading, taken from the design of record and asked of nobody
//! (`DECISIONS.md` → 2026-10-11, *The route catalog's first doubt, read by the surface
//! contract*). The runner fills each placeholder with a value the refusal's own output
//! lists, by index, and runs the line
//! ([`a_printed_line_is_filled_with_what_the_refusal_lists_and_with_nothing_else`]); the
//! arms that hold the marked form and the sandbox's bound hold the new act too. A
//! placeholder the refusal lists no value for cannot be filled.
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
//! **And every case on another binary** — one ignored arm, run by hand
//! ([`the_cases_are_followed_on_the_binary_a_variable_names`]; the plan's step `P5`): it
//! follows the whole catalog on the binary a variable names, each case from the state that
//! binary built, and prints what the case came to there beside what it came to on this
//! tree's — with how the refusing command ended, what became of the witnesses and whether
//! the two printed the same route, which a classification alone does not say. It asserts
//! that it ran. What a difference between two releases is, it does not say, and neither
//! does this suite ([`the_table_follows_each_binary_with_the_state_that_binary_built`]
//! holds the table to its binaries).
//!
//! **Which red cases can block is a ruling's, and this suite does its arithmetic**
//! (`DECISIONS.md` → 2026-10-11, *Which states a route must work in*). A case's bound is
//! read off its state by the frame
//! ([`a_cases_bound_is_read_off_its_state_and_never_off_its_acts`]); the keys the human
//! admitted by name are one table ([`ADMITTED`]), held to the list
//! ([`the_keys_say_what_can_block_and_an_admitted_key_is_a_listed_one`]); and the ignored
//! arm prints, key by key, the red cases that can block and the ones that stand on each
//! bound ([`bounds`]). The expected-red list is as it was: a bound takes no entry off it.
//!
//! **The fence** (the plan's step `P2`, and since the ruling above over every family) holds
//! the catalog to the product's refusal sites. It scans nothing for itself — the sites are
//! [`support::refusal_sites`](crate::support::refusal_sites)' scan, which
//! `refusal_site_list.rs` holds byte-equal to the committed list, and the literals are
//! [`support::rust_source`](crate::support::rust_source)'s. Two arms:
//!
//!   * [`every_blocking_refusal_site_has_a_case_a_reason_an_owed_row_or_a_scheduled_row`]
//!     — a blocking site, of whatever family, has exactly one answer: a case in its
//!     command's file, a [`Reason`] there, a row of [`OWED`] or a row of the scheduled
//!     list. **So a blocking site production source gains is on no list and is named until
//!     it has its case**: the two lists are one ruling's, and the scheduled one is held to
//!     the count that ruling was put on
//!     ([`a_scheduled_site_is_one_the_count_holds_unchanged`]).
//!   * [`every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed`] — over the
//!     prefixes [`CATALOGUED`] names, which a family joins when its file does: the list
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
//! and the table says which rows those are. **Outside the prefixes of [`CATALOGUED`] a
//! dynamic site is one row whatever it mints**: which codes reach it is asked by nobody
//! until its family's prefix joins. **And the two lists are held to a count, not to the
//! ruling**: that a site was rightly left for the port — that the change of its caller is
//! off the path — is a person's reading, in the row's note; and nothing but the rule that
//! a new refusal comes with its cases keeps a new site off the scheduled list under the
//! word for a site the count did not class.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use crate::support::refusal_sites::{self, DYNAMIC, FAMILIES, Site, Source};
use crate::support::route_case::{
    self, Act, Bound, Case, Check, Classification, ExpectedRed, Filled, Followed, Operand, Over,
    Place, Refusal, Seen, Standing, Step, Var,
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
    #[path = "route_catalog/task_finalize.rs"]
    mod task_finalize;
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
        UNSTAGING_LINE_REFUSED,
        Classification::SameRefusalAgain,
        &["setup-unborn-install-path-staged-and-deleted-unstaged-as-printed"],
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
        &["uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed"],
    ),
    (
        FORCE_OVER_UNREADABLE,
        Classification::AnotherFailure,
        &[
            "uninstall-staged-docs-unsearchable-forced",
            "uninstall-task-areas-unlistable-forced",
            "uninstall-workbench-unlistable-forced",
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
    (
        UNINSTALL_PERMISSION_ROUTE,
        Classification::SameRefusalAgain,
        &[
            "uninstall-guide-directory-read-only",
            "uninstall-hook-a-directory",
            "uninstall-hook-a-link-to-a-read-only-script-outside",
            "uninstall-hook-hooks-path-a-file",
            "uninstall-hook-hooks-path-dev-null",
            "uninstall-hook-hooks-path-empty",
            "uninstall-hook-not-utf-8",
            "uninstall-hook-read-only",
            "uninstall-hook-unreadable",
            "uninstall-reference-host-a-directory",
            "uninstall-reference-host-a-link-to-a-read-only-file-outside",
            "uninstall-reference-host-not-utf-8",
            "uninstall-reference-host-unreadable",
            "uninstall-settings-a-directory",
            "uninstall-settings-not-json",
            "uninstall-settings-not-utf-8",
            "uninstall-settings-unreadable",
            "uninstall-workbench-directory-read-only",
            "uninstall-workbench-under-a-read-only-root",
        ],
    ),
    (
        UNINSTALL_PERMISSION_ROUTE,
        Classification::AnotherFailure,
        &["uninstall-forced-workbench-directory-unreadable-made-writable"],
    ),
    (
        COULD_NOT_ASK_GIT,
        Classification::SameRefusalAgain,
        &[
            "uninstall-git-index-unreadable-made-readable",
            "uninstall-git-listing-undecodable-made-readable",
        ],
    ),
    (
        LATE_REFUSAL,
        Classification::AnotherFailure,
        &[
            "uninstall-deny-floor-settings-read-only",
            "uninstall-reference-host-read-only",
            "uninstall-session-hook-settings-read-only",
            "uninstall-settings-read-only",
        ],
    ),
    (
        UNINSTALL_HELD,
        Classification::DidNotEnd,
        &[
            "uninstall-hook-a-special-file",
            "uninstall-reference-host-a-special-file",
            "uninstall-settings-a-special-file",
        ],
    ),
    (
        FINALIZE_WITHOUT_ITS_ID,
        Classification::AnotherFailure,
        &[
            "finalize-nothing-staged-a-new-file-added-and-finalized-as-printed",
            "finalize-nothing-staged-a-tracked-file-deleted-added-and-finalized-as-printed",
            "finalize-nothing-staged-a-tracked-file-modified-added-and-finalized-as-printed",
        ],
    ),
    (
        STAGE_ROUTE_NAMES_OTHER_CAUSES,
        Classification::SameRefusalAgain,
        &["finalize-stage-failed-the-docs-home-ignored-re-run-as-printed"],
    ),
    (
        START_LINE_MINTS_NOTHING,
        Classification::AnotherFailure,
        &["finalize-amend-staged-doc-an-edit-refused-and-the-front-door-run-as-printed"],
    ),
    (
        POINTER_REFUSES,
        Classification::SameRefusalAgain,
        &["linked-doc-second-write-refused-the-preview-asked-where-it-lands"],
    ),
    (
        POINTER_REFUSES,
        Classification::AnotherFailure,
        &["linked-doc-write-refused-on-a-detached-head-the-finalize-asked-for-its-route"],
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
/// Where there is no commit yet and the refusal comes behind the install's writes, the
/// dirty route's line to unstage a file — `git -C <repo> rm --cached -- <path>`, filled
/// with the path the refusal lists — is refused by git: the staged bytes differ from the
/// file the install wrote there and from `HEAD`, and git asks for `-f`. (The key the pilot
/// minted for that line read it as a command to run as printed,
/// `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line`; the surface contract
/// reads it as a shape the reader fills, and no case stands under that key any more.)
const UNSTAGING_LINE_REFUSED: &str =
    "r1-setup-unborn-dirty-route-unstaging-line-refused-behind-the-installs-writes";

// The pilot's keys for `jigc uninstall`.
/// A route prints, between backticks, a line with `<milestone-id>` where an id belongs that
/// the refusal lists nowhere: the line cannot be filled from what was printed, and as
/// printed no shell splits it. Rests on a reading — the case under it is marked. (Until
/// the surface contract was read for it, the key held four more cases, whose placeholders
/// the refusal does list a value for: filled, each holds.)
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
/// The canary's key for the teardown's hook step: *ensure the repo's git hooks directory is
/// writable* over causes that are no such thing. The same defect at the six other steps
/// that remove — each words every error of its callee as *ensure … is writable* over one
/// entry it names, whatever the error was.
const UNINSTALL_PERMISSION_ROUTE: &str = "r1-p3-uninstall-writability-route-not-driven";
/// The canary's key for the fourth guard where git's listing cannot be decoded: its route
/// says to put `git` on `PATH` and make `.jigc/` readable, and both already hold. The same
/// route over another answer git could not give.
const COULD_NOT_ASK_GIT: &str = "r1-p3-uninstall-force-names-nothing-undecodable-listing";
/// A step behind the removal of `.jigc/` refuses over a file that may not be written, and
/// its route — make it writable, run again — leads to a teardown that leaves every settings
/// entry of jigc's in place: the record that says they are jigc's went with `.jigc/`.
const LATE_REFUSAL: &str =
    "r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries";
/// A FIFO at a file the teardown reads — `CLAUDE.md`, the settings file, the hook: the
/// command does not end, with `.jigc/` already removed.
const UNINSTALL_HELD: &str = "r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command";

/// `jigc task finalize` refuses over a dirty tree with an empty index, and the route's
/// second printed line is `jigc task finalize` with no task id: run as printed it is
/// refused by the argument parser.
const FINALIZE_WITHOUT_ITS_ID: &str =
    "r1-finalize-nothing-staged-route-prints-finalize-without-its-id";

/// git refuses to stage a promoted doc whose home an ignore rule covers; the route names
/// a lock and a rule over three other paths, and the printed re-run refuses again.
const STAGE_ROUTE_NAMES_OTHER_CAUSES: &str =
    "r1-finalize-stage-failed-route-names-a-lock-and-three-paths-and-the-docs-home-is-ignored";

/// An edit refused in an amend task: the route says the printed front-door line mints an
/// ordinary task, and under the shipped default it presents the workflows and mints none.
const START_LINE_MINTS_NOTHING: &str =
    "r1-finalize-amend-staged-doc-route-says-the-start-line-mints-and-it-presents-the-workflows";

/// A route cites a command for what it prints — *names where it lands*, *names that
/// route* — and the command is itself refused.
const POINTER_REFUSES: &str = "r1-linked-doc-route-cites-a-command-that-is-itself-refused";

/// **The keys the human admitted by name** — a row a key, and the heading of the entry of
/// `DECISIONS.md` that holds the ruling. Every red case under an admitted key can block,
/// whatever its state stands on ([`bounds`]).
const ADMITTED: &[(&str, &str)] = &[(LATE_REFUSAL, THE_RULING_ON_STATES)];

/// The entry of `DECISIONS.md` that holds the ruling on which states a route must work in,
/// by its heading.
const THE_RULING_ON_STATES: &str = "2026-10-11 — Which states a route must work in: three \
    kinds of planted state are declared bounds of the route catalog, a case's bound is read \
    off its state, and one key is admitted by name (the human's ruling, option B of three)";

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

/// The configuration of the runner `dev/gate` runs the suite under, as committed.
const RUNNER: &str = include_str!("../../../.config/nextest.toml");

/// The term of a nextest filter that names every test under `arms` — a path with its last
/// `::` — and no test by its own name.
fn under(arms: &str) -> String {
    format!("test(/^{arms}/)")
}

/// **Every way a nextest configuration leaves the tests `term` names free to run beside
/// the others of their group.** `term` is one term of a nextest filter — the path the arms
/// stand under, as [`under`] writes it, or one test by its name. What holds its tests to
/// one at a time is an override of `[profile.default]` — the profile every other one
/// inherits — whose filter has that term, and whose test group is one thread wide. The
/// filter's terms may be joined by ` | ` and by nothing else: a term narrowed by an `&` is
/// not the one asked for. And the override sets nothing else — a key of the profile
/// written below the override's header is read as the override's, and the profile loses
/// it without a word.
fn unbounded(config: &str, term: &str) -> Vec<String> {
    let config: toml::Table = match config.parse() {
        Ok(config) => config,
        Err(why) => return vec![format!("the runner's configuration is no TOML: {why}")],
    };
    let naming: Vec<&toml::Value> = config
        .get("profile")
        .and_then(|profiles| profiles.get("default"))
        .and_then(|default| default.get("overrides"))
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter(|over| {
            over.get("filter")
                .and_then(toml::Value::as_str)
                .is_some_and(|filter| filter.split(" | ").any(|part| part.trim() == term))
        })
        .collect();
    if naming.is_empty() {
        return vec![format!(
            "no override of `[profile.default]` has `{term}` as a term of its filter: the \
             arms that follow cases are in no test group, and a runner that gives each \
             test a process runs them beside each other"
        )];
    }
    let mut out = Vec::new();
    for over in naming {
        for (key, _) in over.as_table().into_iter().flatten() {
            if !matches!(key.as_str(), "filter" | "test-group") {
                out.push(format!(
                    "the override that names `{term}` also sets `{key}`: a key written \
                     below an override's header is the override's, and the profile above \
                     it no longer has it"
                ));
            }
        }
        let Some(group) = over.get("test-group").and_then(toml::Value::as_str) else {
            out.push(format!(
                "the override that names `{term}` puts it in no test group"
            ));
            continue;
        };
        let wide = config
            .get("test-groups")
            .and_then(|groups| groups.get(group))
            .and_then(|group| group.get("max-threads"));
        if wide.and_then(toml::Value::as_integer) != Some(1) {
            out.push(format!(
                "`{term}` is in the test group `{group}`, which is {} — one thread wide is \
                 what runs the arms one at a time",
                wide.map_or("declared nowhere".to_string(), |wide| format!(
                    "`max-threads = {wide}`"
                )),
            ));
        }
    }
    out
}

/// **The arms that follow cases run one at a time under the gate's runner** — and an arm
/// `tables!` makes for a third command's file is in the bound the day it is made.
///
/// Inside one process every case passes one gate as wide as the machine
/// ([`no_more_chains_run_at_once_in_one_process_than_the_machine_is_wide`]). nextest gives
/// each test a process of its own, so there the gate is one an arm, and two commands' arms
/// at once are twice the machine's width of chains. What bounds them is the runner's: a
/// test group of one thread in `.config/nextest.toml`, over every test under [`FOLLOWED`]
/// — named by the module, so that no arm is in it by its own name and none can be left
/// out. On planted configurations, fault by fault; then the committed one.
///
/// **And the arm that follows every case on a named binary is in the same group**, by its
/// own name — it is one test, ignored, and follows the whole catalog twice: asked for
/// together with the others, it waits for them like one of them. The harness is asked
/// that a test of that name is there, so the term names something.
#[test]
fn the_arms_that_follow_cases_run_one_at_a_time_under_the_gates_runner() {
    let arms = under("s::followed::");
    let arms = arms.as_str();
    let config = |groups: &str, filter: &str, group: &str| {
        format!(
            "[test-groups]\n{groups}\n[profile.default]\ntest-threads = 16\n\
             [[profile.default.overrides]]\nfilter = '{filter}'\n{group}\n"
        )
    };
    let one = "one-at-a-time = { max-threads = 1 }";
    let in_it = "test-group = 'one-at-a-time'";
    says(
        &unbounded(&config(one, "test(/^s::followed::/)", in_it), arms),
        &[],
    );
    says(
        &unbounded(
            &config(one, "test(=s::other) | test(/^s::followed::/)", in_it),
            arms,
        ),
        &[],
    );
    for (groups, filter, group, said) in [
        // An arm named by itself: the next command's arm is outside the group.
        (one, "test(=s::followed::setup)", in_it, "in no test group"),
        // The path, narrowed.
        (
            one,
            "test(/^s::followed::/) & binary(g_other)",
            in_it,
            "in no test group",
        ),
        (
            one,
            "test(/^s::followed::/)",
            "",
            "puts it in no test group",
        ),
        (
            "one-at-a-time = { max-threads = 2 }",
            "test(/^s::followed::/)",
            in_it,
            "`max-threads = 2`",
        ),
        (
            "another = { max-threads = 1 }",
            "test(/^s::followed::/)",
            in_it,
            "declared nowhere",
        ),
    ] {
        says(
            &unbounded(&config(groups, filter, group), arms),
            &[("`test(/^s::followed::/)`", said)],
        );
    }
    says(
        &unbounded("[profile.default]\ntest-threads = 16\n", arms),
        &[("`test(/^s::followed::/)`", "in no test group")],
    );
    // The profile's own key, written below the override's header.
    says(
        &unbounded(
            &config(
                one,
                "test(/^s::followed::/)",
                "test-group = 'one-at-a-time'\nslow-timeout = '60s'",
            ),
            arms,
        ),
        &[("`test(/^s::followed::/)`", "also sets `slow-timeout`")],
    );
    says(
        &unbounded("[profile.default", arms),
        &[("the runner's configuration", "is no TOML")],
    );

    let on_a_named_binary = format!("{}::{ON_A_NAMED_BINARY}", suite());
    assert!(
        route_case::harness_tests().contains(&on_a_named_binary),
        "the harness lists no test `{on_a_named_binary}`: the term that puts the arm on a \
         named binary into the group names nothing",
    );
    let mut wrong = unbounded(RUNNER, &under(&format!("{}::{FOLLOWED}::", suite())));
    wrong.extend(unbounded(RUNNER, &format!("test(={on_a_named_binary})")));
    assert!(
        wrong.is_empty(),
        "`.config/nextest.toml` and the arms that follow cases:\n\n  {}",
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

/// **The code prefixes whose vocabulary the catalog is held to** — the families of
/// `jigc setup` (`setup`, `adapter-guide`) and of `jigc uninstall`, as
/// [`refusal_sites::FAMILIES`] assigns them. A prefix joins when its command's file does.
/// The sites are held family or no family
/// ([`every_blocking_refusal_site_has_a_case_a_reason_an_owed_row_or_a_scheduled_row`]);
/// this is the list for the arm beside it, which asks after every code a prefix spells.
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
    /// The id the claimed site goes by — what a case or a reason names it with: `file::function::<code>#dynamic-<ordinal>`. The code stands where a listed
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

/// The sites one task owes an answer.
struct Owed {
    task: &'static str,
    sites: &'static [&'static str],
}

/// **The owed list: the sites the ruling takes in now that have no answer yet, under the
/// task of the ruling that owes it** (`DECISIONS.md` → [`THE_RULING_ON_FAMILIES`]). A row
/// goes in the commit that brings its site's case or reason — or, for a site whose caller
/// is still to be read, its row of the scheduled list with why the caller's change is off
/// the path; a row that stays beside an answer is named. The list is emptied family by
/// family and deleted, with this type, when the last is in.
const OWED: &[Owed] = &[
    Owed {
        task: "P7 · config — a case or a reason",
        sites: &[
            "crates/cli/src/config.rs::repoint_failed::config.repoint-failed#1",
            "crates/cli/src/config.rs::run_set::config.untrackable-root#1",
            "crates/cli/src/config.rs::run_set::config.unusable-root#1",
            "crates/cli/src/config.rs::run_set::config.value-rejected#1",
            "crates/cli/src/config.rs::run_set::config.workbench-root#1",
        ],
    },
    Owed {
        task: "P7 · doc read — a case or a reason",
        sites: &["crates/engine/src/store.rs::home_shape_refusal::store.home-not-regular-file#1"],
    },
    Owed {
        task: "P7 · doc read — the caller's diff is read: a case, or a scheduled row that says why the change is off the path",
        sites: &[
            "crates/cli/src/task.rs::reject_fixed_identity_slug::store.fixed-identity#1",
            "crates/cli/src/task.rs::reject_malformed_work_unit_id::work-unit.malformed-id#1",
        ],
    },
    Owed {
        task: "P7 · doc write — a case or a reason",
        sites: &[
            "crates/cli/src/doc.rs::run_retitle_item::write.machine-maintained#1",
            "crates/cli/src/doc.rs::title_ignored_refusal::write.title-ignored#1",
        ],
    },
    Owed {
        task: "P7 · doc write — the caller's diff is read: a case, or a scheduled row that says why the change is off the path",
        sites: &[
            "crates/cli/src/doc.rs::committed_reslug_refusal::write.identity-change#1",
            "crates/cli/src/doc.rs::fixed_identity_refusal::write.identity-change#1",
            "crates/cli/src/doc.rs::fixed_title_refusal::write.title-ignored#1",
            "crates/cli/src/doc.rs::identity_divergence_refusal::write.identity-change#1",
            "crates/cli/src/doc.rs::machine_maintained_guard::write.machine-maintained#1",
            "crates/cli/src/doc.rs::occupied_destination_refusal::write.already-present#1",
            "crates/cli/src/doc.rs::retitle_id_from_refusal::write.identity-change#1",
            "crates/cli/src/task.rs::reject_slug_over_name_ceiling::write.slug-name-ceiling#1",
            "crates/engine/src/state.rs::already_exists_finding::create.already-exists#1",
            "crates/engine/src/state.rs::instance_collision_finding::create.serial-collision#1",
            "crates/engine/src/state.rs::unknown_doctype_finding::create.unknown-doctype#1",
        ],
    },
    Owed {
        task: "P7 · migrate — the caller's diff is read: a case, or a scheduled row that says why the change is off the path",
        sites: &[
            "crates/cli/src/migrate.rs::adjudicate_source_path::migrate.source-untrackable#1",
            "crates/cli/src/migrate.rs::adjudicate_source_tracked::migrate.source-untracked#1",
        ],
    },
    Owed {
        task: "P7 · milestone — a case or a reason",
        sites: &[
            "crates/cli/src/combine.rs::collision_finding::combine.code-collision#1",
            "crates/cli/src/milestone.rs::dirty_worktree_finding::milestone.dirty-worktree#1",
            "crates/cli/src/milestone.rs::leftover_finding::milestone.leftover-holds-work#1",
            "crates/cli/src/milestone.rs::provision_failed_finding::milestone.provision-failed#1",
            "crates/cli/src/milestone.rs::unlanded_work_finding::milestone.unlanded-work#1",
            "crates/cli/src/milestone.rs::zero_contribution_finding::milestone.zero-contribution#1",
            "crates/engine/src/milestone.rs::io_finding::milestone.area-io#1",
            "crates/engine/src/milestone.rs::record_home_refusal::milestone.record-exists#1",
        ],
    },
    Owed {
        task: "P7 · start — a case or a reason",
        sites: &["crates/cli/src/start.rs::reenter_in_repo::workflow-refs.workflow-mismatch#1"],
    },
    Owed {
        task: "P7 · start — the caller's diff is read: a case, or a scheduled row that says why the change is off the path",
        sites: &[
            "crates/cli/src/start.rs::read_workflow::workflow-refs.unknown-workflow#1",
            "crates/engine/src/compose.rs::workflow_refs_with_fills::workflow-refs.slot-fill-orphan#1",
        ],
    },
    Owed {
        task: "P7 · task finalize — a case or a reason",
        sites: &[
            "crates/engine/src/file_state.rs::conflict_block_finding::reconciliation.conflict-block#1",
            "crates/engine/src/file_state.rs::conformance_block_finding::reconciliation.conformance-block#1",
            "crates/engine/src/file_state.rs::drift_store_finding::file-state.hash-matches#1",
            "crates/engine/src/file_state.rs::unanswered_pin_finding::reconciliation.conflict-block#1",
            "crates/engine/src/finalize.rs::clobber_finding::finalize.promote-clobber#1",
            "crates/engine/src/finalize.rs::provenance_io_finding::finalize.provenance-io#1",
            "crates/engine/src/finalize.rs::source_path_io_finding::finalize.source-path-io#1",
            "crates/engine/src/state.rs::io_finding::task.working-area-io#1",
        ],
    },
    Owed {
        task: "P7 · unassigned — the caller's diff is read: a case, or a scheduled row that says why the change is off the path",
        sites: &[
            "crates/cli/src/migrate_corpus.rs::blocked_finding::dynamic#1",
            "crates/cli/src/relocate.rs::refuse::dynamic#1",
            "crates/cli/src/rename.rs::refuse::dynamic#1",
            "crates/engine/src/store.rs::block::dynamic#1",
        ],
    },
];

/// **A site the ruling leaves for the port**: its id, the class the count gave it
/// ([`THE_COUNT`]), and a note — `-`, or, where a direct caller of the site's function
/// changed since the previous release and the site is left out all the same, which change
/// that is and why it is off the path to the refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scheduled {
    site: &'static str,
    class: &'static str,
    note: &'static str,
}

/// **The scheduled list — the blocking sites outside the fix pass's reach, which the
/// human's ruling leaves until before the port** ([`THE_RULING_ON_FAMILIES`]): one file of
/// data, a line a site. A row leaves it in the commit that brings its site's case.
const SCHEDULED: &str = include_str!("route_catalog/scheduled.tsv");

/// The entry of `DECISIONS.md` that holds the ruling on which families the catalog takes
/// in, by its heading.
const THE_RULING_ON_FAMILIES: &str = "2026-10-12 — Which families the route catalog takes \
    in: the sites the fix pass touched now, the rest scheduled before the port — and the \
    fence holds every blocking site of every family to a case, a reason, an owed row or a \
    scheduled row (the human's ruling of 2026-10-11, the first of three options)";

/// The class the count gives a site whose constructing function is byte for byte the
/// previous release's — the one class the ruling leaves for the port.
const UNCHANGED: &str = "unchanged";

/// The class of a scheduled site the count holds no row for; its note says why.
const NOT_COUNTED: &str = "not-counted";

/// The rows of a scheduled list: every line that is no comment and not empty, as
/// `site`, `class` and `note`, tab-separated. A line of another shape is a row with what it
/// has, so that [`idle_rows`] names it.
fn scheduled(list: &'static str) -> Vec<Scheduled> {
    list.lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut columns = line.splitn(3, '\t');
            Scheduled {
                site: columns.next().unwrap_or_default(),
                class: columns.next().unwrap_or_default(),
                note: columns.next().unwrap_or_default(),
            }
        })
        .collect()
}

/// **The count the ruling was put on**, as committed beside the pilot's record: every
/// blocking site of the list, classed against the previous release's production source.
const THE_COUNT: &str = include_str!(
    "../../../completions/artifacts/M55/stabilization-build/reshape/route-catalog-sites-changed-since-rc24.md"
);

/// `site → class`, from the rows of a count's table: a line `` | `site` | family | class | … ``.
fn counted(count: &str) -> BTreeMap<&str, &str> {
    count
        .lines()
        .filter_map(|line| {
            let mut cells = line.strip_prefix("| `")?.split(" | ");
            let site = cells.next()?.strip_suffix('`')?;
            let _family = cells.next()?;
            // A site's id, and no other table's first cell.
            site.contains("::").then_some((site, cells.next()?))
        })
        .collect()
}

/// **Every row of a scheduled list that the count does not bear out.** The ruling leaves
/// for the port the sites whose function did not change; so a scheduled site the count
/// holds is one it classes [`UNCHANGED`], under that word, and a site it holds under
/// another class is the ruling's to take in now. A site the count has no row for is
/// scheduled as [`NOT_COUNTED`], with why in its note.
fn misscheduled(scheduled: &[Scheduled], counted: &BTreeMap<&str, &str>) -> Vec<String> {
    let mut out = Vec::new();
    for row in scheduled {
        let Scheduled { site, class, note } = *row;
        match counted.get(site) {
            Some(&counted) if counted != UNCHANGED => out.push(format!(
                "`{site}` is on the scheduled list, and the count classes it `{counted}`: a \
                 site of that class is one the ruling takes in now"
            )),
            Some(&counted) if class != counted => out.push(format!(
                "`{site}` is scheduled as `{class}`, and the count classes it `{counted}`"
            )),
            Some(_) => {}
            None if class != NOT_COUNTED => out.push(format!(
                "`{site}` is scheduled as `{class}`, and the count holds no row for it: a \
                 site the count did not class is scheduled as `{NOT_COUNTED}`, with why"
            )),
            None if matches!(note, "" | "-") => out.push(format!(
                "`{site}` is scheduled as `{NOT_COUNTED}` and does not say why the count \
                 has no row for it"
            )),
            None => {}
        }
    }
    out
}

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
    claims: Vec<Claim>,
    /// `(literal, why it is no finding's code)`.
    no_code: Vec<(&'static str, &'static str)>,
    /// `(site, the task that owes its answer)` — [`OWED`], a row a site.
    owed: Vec<(&'static str, &'static str)>,
    /// The sites the ruling leaves for the port ([`scheduled`]).
    scheduled: Vec<Scheduled>,
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
        claims: CLAIMS.to_vec(),
        no_code: NO_CODE.to_vec(),
        owed: OWED
            .iter()
            .flat_map(|owed| owed.sites.iter().map(|site| (*site, owed.task)))
            .collect(),
        scheduled: scheduled(SCHEDULED),
    }
}

/// Whether `id` is the id a claim gives a code minted at the dynamic site `site`.
fn claimed_at(site: &Site, id: &str) -> bool {
    CLAIMS
        .iter()
        .any(|claim| claim.minted_at == site.id() && claim.id().as_deref() == Some(id))
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

/// **Every site that owes an answer, sorted by id**: the blocking listed sites of every
/// family, and each claimed code at the blocking dynamic site its claim names. A dynamic
/// site a claim names owes nothing under its own id — it is answered through the codes
/// claimed for it; one no claim names owes like a listed site. A claim that names no such
/// site adds nothing here and is named by [`idle_rows`].
fn owing(read: &Read, claims: &[Claim]) -> Vec<Owing> {
    let claimed =
        |site: &Site| site.code == DYNAMIC && claims.iter().any(|c| c.minted_at == site.id());
    let mut out: Vec<Owing> = read
        .sites
        .iter()
        .filter(|site| blocks(site) && !claimed(site))
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

/// **Every site that does not have exactly one answer** — a case, a reason, a row of the
/// owed list or a row of the scheduled list. None is named; so is a case beside a reason,
/// a row of either list beside an answer or beside the other list's, and
/// `no-route-printed` given to a site whose route the list reads at its call.
fn uncovered(read: &Read, rows: &Rows) -> Vec<String> {
    let mut out = Vec::new();
    for site in owing(read, &rows.claims) {
        let id = &site.id;
        let case = rows.cases.contains(&id.as_str());
        let reason = rows.reasons.iter().find(|reason| reason.site == id);
        let owed = rows.owed.iter().find(|(owed, _)| owed == id);
        let scheduled = rows.scheduled.iter().any(|row| row.site == id);
        let answer = match (case, reason.is_some()) {
            (true, _) => Some("case"),
            (false, true) => Some("reason"),
            (false, false) => None,
        };
        match (case, reason) {
            (false, None) if owed.is_none() && !scheduled => out.push(format!(
                "`{id}` has no case, no reason, no owed row and no scheduled row"
            )),
            (true, Some(reason)) => out.push(format!(
                "`{id}` has a case and the reason `{}` for having none — one of the two is \
                 wrong",
                reason.why
            )),
            (false, Some(reason))
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
        match (answer, owed, scheduled) {
            (Some(answer), Some((_, task)), _) => out.push(format!(
                "`{id}` has a {answer} and is still owed by `{task}` — the row goes in the \
                 commit that brings the answer"
            )),
            (None, Some((_, task)), true) => out.push(format!(
                "`{id}` is owed by `{task}` and is on the scheduled list — a site the \
                 ruling takes in now is not one it leaves for the port"
            )),
            _ => {}
        }
        if let (Some(answer), true) = (answer, scheduled) {
            out.push(format!(
                "`{id}` has a {answer} and is still on the scheduled list — the row goes in \
                 the commit that brings the answer"
            ));
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

/// **Every row that is about nothing**: a reason whose site owes no case — gone, renamed,
/// or never blocking — a claim whose site the list does not hold as a
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
                "the owed list (`{task}`) names `{site}`, which is no blocking site that \
                 owes an answer: it is gone, renamed, was never one, or is a dynamic site \
                 answered through its claims"
            ));
        } else if rows.owed[..at].iter().any(|(other, _)| other == site) {
            out.push(format!("`{site}` is on the owed list twice"));
        }
    }

    for (at, row) in rows.scheduled.iter().enumerate() {
        let site = row.site;
        if row.class.is_empty() {
            out.push(format!("the scheduled row for `{site}` has no class"));
        }
        if !owes(site) {
            out.push(format!(
                "the scheduled list names `{site}`, which is no blocking site that owes an \
                 answer: it is gone, renamed, was never one, or is a dynamic site answered \
                 through its claims"
            ));
        } else if rows.scheduled[..at].iter().any(|other| other.site == site) {
            out.push(format!("`{site}` is on the scheduled list twice"));
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
                "the reason `{}` names `{site}`, which is no blocking site that owes an \
                 answer: it is gone, renamed, or was never one",
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
/// The fixture's site of a family no command's file is there for.
const ANOTHER: &str = "crates/cli/src/fixture.rs::another_familys::task.spelled#1";

/// A row of a scheduled list with nothing to note.
const fn left(site: &'static str) -> Scheduled {
    Scheduled {
        site,
        class: UNCHANGED,
        note: "-",
    }
}

fn fixture() -> Read {
    read(&[Source::new(FIXTURE, REFUSALS)])
}

/// The fixture's one claim.
const BY_A_MATCH: Claim = claim("setup.by-a-match", MINTED);

/// The fixture's tables with nothing wrong: a case for the site that spells its code and
/// for the one a claim makes, a reason for the third, the fourth — another family's —
/// scheduled; the claim; and the literal that is no code.
fn in_order() -> Rows {
    Rows {
        cases: vec![SPELLED, CLAIMED],
        reasons: vec![Reason {
            site: ROUTELESS,
            why: Why::NoRoutePrinted,
        }],
        claims: vec![BY_A_MATCH],
        no_code: vec![("setup.rs", "a file's name")],
        owed: Vec::new(),
        scheduled: vec![left(ANOTHER)],
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

/// **Every blocking refusal site of the product has exactly one answer**: a case in its
/// command's file, a reason there, a row of the owed list under the task of the ruling
/// that owes it, or a row of the scheduled list — the sites the ruling leaves for the port
/// (`DECISIONS.md` → [`THE_RULING_ON_FAMILIES`]). Every family, and the dynamic sites no
/// family has: one that a claim names is answered through the codes claimed for it, each a
/// site of its own; one no claim names answers under its own id.
///
/// On a fixture: the advisory site owes nothing; **a site of a family no command's file is
/// there for owes like any other**; a claim makes the code at its dynamic site owe like a
/// listed one, and without it the dynamic site owes; and two answers for one site are
/// named as none is. Then this tree, against the tables this suite holds — and this tree
/// with one blocking site more than it has, in a family nobody catalogued: **a site
/// production source gains tomorrow is on no list, and is named until it has its case**.
#[test]
fn every_blocking_refusal_site_has_a_case_a_reason_an_owed_row_or_a_scheduled_row() {
    let fixture = fixture();
    says(
        &uncovered(&fixture, &Rows::default()),
        &[
            (ANOTHER, "no case"),
            (MINTED, "no case"),
            (SPELLED, "no case"),
            (ROUTELESS, "no case"),
        ],
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
            (ANOTHER, "no case"),
            (CLAIMED, "no case"),
            (SPELLED, "no case"),
            (ROUTELESS, "no case"),
        ],
    );
    says(&uncovered(&fixture, &in_order()), &[]);
    // The scheduled list taken away: the other family's site is on no list.
    says(
        &uncovered(
            &fixture,
            &Rows {
                scheduled: Vec::new(),
                ..in_order()
            },
        ),
        &[(
            ANOTHER,
            "no case, no reason, no owed row and no scheduled row",
        )],
    );
    // An owed row answers as a scheduled one does.
    says(
        &uncovered(
            &fixture,
            &Rows {
                owed: vec![(ANOTHER, "P7")],
                scheduled: Vec::new(),
                ..in_order()
            },
        ),
        &[],
    );
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
    // A row of either list beside an answer, and a row of both.
    says(
        &uncovered(
            &fixture,
            &Rows {
                owed: vec![(SPELLED, "P7"), (ROUTELESS, "P7"), (ANOTHER, "P7")],
                ..in_order()
            },
        ),
        &[
            (ANOTHER, "is owed by `P7` and is on the scheduled list"),
            (SPELLED, "a case and is still owed by `P7`"),
            (ROUTELESS, "a reason and is still owed by `P7`"),
        ],
    );
    says(
        &uncovered(
            &fixture,
            &Rows {
                scheduled: vec![left(ANOTHER), left(CLAIMED), left(ROUTELESS)],
                ..in_order()
            },
        ),
        &[
            (CLAIMED, "a case and is still on the scheduled list"),
            (ROUTELESS, "a reason and is still on the scheduled list"),
        ],
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
                cases: vec![CLAIMED],
                reasons: vec![routeless(SPELLED), routeless(ROUTELESS)],
                ..in_order()
            },
        ),
        &[(SPELLED, "a `prose` route written at its call")],
    );

    let here = tree();
    let wrong = uncovered(&here, &rows());
    assert!(
        wrong.is_empty(),
        "the route catalog does not answer for {} of the {} blocking refusal sites of the \
         product:\n\n  {}\n\na site is answered by a case in its command's file under \
         `route_catalog/`, or by a reason there; a code that a dynamic site mints is a site \
         once `CLAIMS` holds it. **A site that is new has its case in the commit that adds \
         it**: the owed list and `route_catalog/scheduled.tsv` are the lists of one ruling \
         (`DECISIONS.md` → {THE_RULING_ON_FAMILIES}) and take no site that ruling did not \
         count.",
        wrong.len(),
        owing(&here, CLAIMS).len(),
        wrong.join("\n  "),
    );

    // Tomorrow's site: one file more, in a family no command's file is there for.
    let mut sources = refusal_sites::tree_sources();
    sources.push(Source::new(
        "crates/cli/src/planted.rs",
        "fn planted() -> Finding {\n    Finding::block(\"ingest.planted\", \"m\", \"r\")\n}\n",
    ));
    says(
        &uncovered(&read(&sources), &rows()),
        &[(
            "`crates/cli/src/planted.rs::planted::ingest.planted#1`",
            "no case, no reason, no owed row and no scheduled row",
        )],
    );
}

/// **A scheduled site is one the count holds unchanged.** The scheduled list is the rest of
/// one ruling, put on one count: a row whose site the count classes otherwise is the
/// ruling's to take in now, a row under another class than the count's is wrong, and a
/// row for a site the count has no row for says why. On planted rows against a planted
/// count; then the suite's list against the committed count — whose table is not empty and
/// holds every scheduled site but the ones that say why it does not.
#[test]
fn a_scheduled_site_is_one_the_count_holds_unchanged() {
    let count = "| site (`file::function::code#ordinal`) | family | class | why |\n\
                 |---|---|---|---|\n\
                 | `a.rs::kept::task.kept#1` | task finalize | unchanged | function byte-identical |\n\
                 | `a.rs::moved::task.moved#1` | task finalize | route-changed | route text differs |\n";
    let counted_here = counted(count);
    assert_eq!(
        counted_here.iter().collect::<Vec<_>>(),
        [
            (&"a.rs::kept::task.kept#1", &"unchanged"),
            (&"a.rs::moved::task.moved#1", &"route-changed"),
        ],
    );
    let row = |site, class, note| Scheduled { site, class, note };
    says(
        &misscheduled(
            &[
                row("a.rs::kept::task.kept#1", UNCHANGED, "-"),
                row(
                    "a.rs::handed::dynamic#1",
                    NOT_COUNTED,
                    "its severity is handed in",
                ),
            ],
            &counted_here,
        ),
        &[],
    );
    says(
        &misscheduled(
            &[
                row("a.rs::moved::task.moved#1", "route-changed", "-"),
                row("a.rs::kept::task.kept#1", "function-changed", "-"),
                row("a.rs::kept::task.kept#1", NOT_COUNTED, "it is"),
                row("a.rs::new::task.new#1", UNCHANGED, "-"),
                row("a.rs::new::task.new#1", NOT_COUNTED, "-"),
                row("a.rs::new::task.new#1", NOT_COUNTED, ""),
            ],
            &counted_here,
        ),
        &[
            ("`a.rs::moved::task.moved#1`", "the ruling takes in now"),
            (
                "`a.rs::kept::task.kept#1`",
                "the count classes it `unchanged`",
            ),
            (
                "`a.rs::kept::task.kept#1`",
                "the count classes it `unchanged`",
            ),
            ("`a.rs::new::task.new#1`", "the count holds no row for it"),
            ("`a.rs::new::task.new#1`", "does not say why"),
            ("`a.rs::new::task.new#1`", "does not say why"),
        ],
    );

    let counted = counted(THE_COUNT);
    let scheduled = scheduled(SCHEDULED);
    assert!(
        counted.len() > 200 && scheduled.len() > 100,
        "the count's table has {} rows and the scheduled list {}: neither was read",
        counted.len(),
        scheduled.len(),
    );
    let wrong = misscheduled(&scheduled, &counted);
    assert!(
        wrong.is_empty(),
        "`route_catalog/scheduled.tsv` and the count it was cut from disagree:\n\n  {}",
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
/// nothing: a reason for a site that never owed a case, a claim for a site that is not dynamic
/// or a code nothing spells, a row that is there twice, a row that does not say why. On a
/// fixture, kind by kind; then this tree's tables.
#[test]
fn a_row_for_a_site_that_is_gone_is_named() {
    let fixture = fixture();
    says(&idle_rows(&fixture, &in_order()), &[]);

    let reason = |site, why| Reason { site, why };
    says(
        &idle_rows(
            &fixture,
            &Rows {
                reasons: vec![
                    reason(ROUTELESS, Why::NoRoutePrinted),
                    reason(GONE, Why::Platform("another")),
                    reason(ADVISED, Why::Platform("another")),
                    reason(ROUTELESS, Why::NotConstructible("")),
                ],
                ..in_order()
            },
        ),
        &[
            (GONE, "the reason `platform: another` names"),
            (ADVISED, "no blocking site that owes an answer"),
            (ROUTELESS, "states nothing"),
            (ROUTELESS, "is given two reasons"),
        ],
    );

    // A claim that names no dynamic site makes no site, so a reason given under it is idle
    // with it.
    says(
        &idle_rows(
            &fixture,
            &Rows {
                cases: vec![SPELLED],
                reasons: vec![
                    in_order().reasons[0],
                    reason(CLAIMED, Why::Platform("another")),
                ],
                claims: vec![claim("setup.by-a-match", GONE)],
                ..in_order()
            },
        ),
        &[
            (CLAIMED, "the reason `platform: another` names"),
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

    // A row of the owed list, and one of the scheduled list, for a site that is gone, was
    // never blocking, or is there twice; an owed row that names no task; a scheduled row
    // that has no class.
    says(
        &idle_rows(
            &fixture,
            &Rows {
                owed: vec![
                    (GONE, "P7"),
                    (ADVISED, "P7"),
                    (ANOTHER, ""),
                    (ANOTHER, "P7"),
                ],
                scheduled: Vec::new(),
                ..in_order()
            },
        ),
        &[
            (GONE, "the owed list (`P7`) names"),
            (ADVISED, "which is no blocking site"),
            (ANOTHER, "names no task"),
            (ANOTHER, "on the owed list twice"),
        ],
    );
    says(
        &idle_rows(
            &fixture,
            &Rows {
                scheduled: vec![
                    left(ANOTHER),
                    left(GONE),
                    left(ADVISED),
                    left(ANOTHER),
                    Scheduled {
                        class: "",
                        ..left(ROUTELESS)
                    },
                ],
                ..in_order()
            },
        ),
        &[
            (GONE, "the scheduled list names"),
            (ADVISED, "which is no blocking site"),
            (ANOTHER, "on the scheduled list twice"),
            (ROUTELESS, "has no class"),
        ],
    );
    // A dynamic site a claim names is answered through its claims: a row under its own id
    // is about nothing.
    says(
        &idle_rows(
            &fixture,
            &Rows {
                scheduled: vec![left(ANOTHER), left(MINTED)],
                ..in_order()
            },
        ),
        &[(MINTED, "the scheduled list names")],
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
///   * handed no case and no reason, the fence names every site that owes
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
        "handed no case, no reason and neither list, the fence names every site that owes \
         an answer: {named:#?}",
    );
    // Every family of the list that has a blocking site is among the sites that owe: the
    // fence is over the product, and not over the commands that have a file.
    for family in read
        .sites
        .iter()
        .filter(|site| blocks(site))
        .map(|site| site.command)
        .collect::<BTreeSet<_>>()
    {
        let of_family = |id: &str| {
            read.sites
                .iter()
                .any(|site| site.command == family && (site.id() == id || claimed_at(site, id)))
        };
        assert!(
            owing.iter().any(|site| of_family(&site.id)),
            "no site of the family `{family}` owes an answer",
        );
    }

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
/// [`Operand`], a span's index, a line with the indices of the listed values that fill it
/// ([`Filled`]), or nothing at all — never a path, a word or a command of
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
                || variant.ends_with("(Filled)")
                || !variant.contains('('),
            "the act `{variant}` takes something that is neither an operand, a span's index \
             nor a line with the indices of what fills it",
        );
    }
    assert_eq!(
        variants(body_of(&frame, "pub struct Filled {")),
        ["pub line: usize", "pub with: &'static [usize]"],
        "what fills a printed line is named by index and by nothing else: the line a span \
         of the route, each value one the refusal lists",
    );
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
        over: None,
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
    over: None,
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

    // And a value filled into a printed line: a refusal that lists `/dev/null` and a path
    // above the sandbox has neither handed to the command its route prints.
    let listed_outside = Case {
        name: "planted-a-line-filled-with-paths-outside-the-sandbox",
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused over:' '  `/dev/null`' \
             '  ../../elsewhere' '  route: add them with `git -C . add -- <path>`'; exit 1",
        ],
        acts: &[
            Act::RunFilled(Filled {
                line: 0,
                with: &[0],
            }),
            Act::RunFilled(Filled {
                line: 0,
                with: &[1],
            }),
        ],
        ..PLANTED
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &listed_outside)
        .expect("the planted case is one the runner follows");
    assert_eq!(
        (
            followed.classification,
            followed
                .detail
                .matches("not done — it names nothing inside")
                .count()
        ),
        (Classification::AnotherFailure, 2),
        "neither listed path is filled into the line, and the case is red for it:\n{}",
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

/// The three acts that run the refusing command again.
const AGAIN: [Act; 3] = [Act::Rerun, Act::RerunInTheRepository, Act::RerunWith(0)];

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
        // The flag a route prints for the re-run is the re-run's last word.
        (
            Case {
                name: "planted-a-route-that-prints-a-flag-for-the-re-run",
                refusing: &[
                    "-c",
                    "test \"$0\" = --consent && { : > ran-with-the-flag; exit 0; }; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: unstage `it`, or re-run with `--consent` to declare it'; exit 1",
                ],
                acts: &[Act::RerunWith(1)],
                holds: &[Check::RegularFile(Place::Repo("ran-with-the-flag"))],
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

    // A span that is no flag is no word a re-run takes.
    match route_case::follow(
        Path::new("/bin/sh"),
        &Case {
            name: "planted-a-re-run-with-a-span-that-is-no-flag",
            refusing: &[
                "-c",
                "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: unstage `it`, or re-run with `--consent`'; exit 1",
            ],
            acts: &[Act::RerunWith(0)],
            ..PLANTED
        },
    ) {
        Err(fault) => assert!(
            fault.contains("span 0") && fault.contains("is no flag"),
            "the fault names the span and says it is no flag: {fault}",
        ),
        Ok(followed) => panic!(
            "a re-run took a span that is no flag: {}\n{}",
            followed.classification, followed.detail
        ),
    }

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

/// **A reader goes where the route says, and nowhere outside the sandbox.** A route can
/// send its reader to another directory before the next line — *run it from the main
/// checkout: `cd <there>`, then `jigc …`* — and the line then runs there, not where the
/// case stands. The directory is a span of the printed route. One that lies outside the
/// sandbox is gone to by nobody: the act is recorded as not done, and the case is red for
/// it. A span that is no `cd` line is a fault of the case.
#[test]
fn a_reader_goes_where_the_route_says_and_nowhere_outside_the_sandbox() {
    come_out(&[
        (
            Case {
                name: "planted-a-route-that-sends-its-reader-elsewhere",
                state: &[Step::Dir(Place::Outside("there"))],
                refusing: &[
                    "-c",
                    "test -e ../outside/there/ran-there && exit 0; \
                     printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     \"  route: \\`cd $(cd ../outside/there && pwd -P)\\`, then \
                     \\`jigc -c ': > ran-there'\\`\"; exit 1",
                ],
                acts: &[Act::Cd(0), Act::Run(1)],
                holds: &[Check::RegularFile(Place::Outside("there/ran-there"))],
                ..PLANTED
            },
            Classification::Holds,
        ),
        (
            Case {
                name: "planted-a-route-that-sends-its-reader-out-of-the-sandbox",
                refusing: &[
                    "-c",
                    "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                     '  route: `cd /`, then `jigc -c \"test $PWD != /\"`'; exit 1",
                ],
                acts: &[Act::Cd(0), Act::Run(1)],
                ..PLANTED
            },
            Classification::AnotherFailure,
        ),
    ]);

    match route_case::follow(
        Path::new("/bin/sh"),
        &Case {
            name: "planted-a-span-that-is-no-cd-line",
            refusing: &[
                "-c",
                "printf '%s\\n' 'blocking · setup.install-hook — refused' \
                 '  route: go `elsewhere`, then `jigc -c true`'; exit 1",
            ],
            acts: &[Act::Cd(0), Act::Run(1)],
            ..PLANTED
        },
    ) {
        Err(fault) => assert!(
            fault.contains("span 0") && fault.contains("is no `cd` line"),
            "the fault names the span and says what it is not: {fault}",
        ),
        Ok(followed) => panic!(
            "a reader went by a span that is no `cd` line: {}\n{}",
            followed.classification, followed.detail
        ),
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

/// A shell that stands in for a refusal which lists three values — beside another
/// finding's listing, a line of prose and its own route — and prints a line that holds
/// three placeholders: the line exits 0 only where each was filled with its own value, as
/// one word, and removes the file the second names.
const LISTS: &str = r#"test -e 'kept here' || exit 0
cat <<'END'
advisory · setup.other — another finding lists:
  not-this
blocking · setup.install-hook — refused over two entries:
  first: `kept here`, second
prose that is not indented lists nothing: stray
  route: remove it with `jigc -c 'rm -- "$2" && test "$1-$3" = first-second' x <id> <path> <id>`, then re-run `jigc -c "test ! -e 'kept here'"`
END
exit 1"#;

/// **A printed line is filled with what the refusal lists, and with nothing else.** A
/// route prints a line with a word in angle brackets — a shape the reader fills, by the
/// surface contract — and the value belongs to the refusal's own output: its listing. So
/// the act names the line by its span and each value by its index in what the refusal
/// lists; the runner puts each in as one word and runs the line.
///
/// *What a refusal lists*: the indented lines of its own finding that are not its route —
/// a backticked span one value, every other word one, the colon or comma that sets a word
/// off dropped; never another finding's lines, a line of prose, or the route.
///
/// *What the act may not do*: leave a placeholder unfilled or fill one that is not there,
/// name a value the refusal does not list, or stand in for a line that is a command as
/// printed. And a line with a placeholder that is run as printed is what it was before:
/// one no shell can split.
#[test]
fn a_printed_line_is_filled_with_what_the_refusal_lists_and_with_nothing_else() {
    let said = "\
advisory · setup.other — another finding lists:
  not-this
blocking · setup.install-hook — refused over two entries:
  first: `kept here`, second
prose that is not indented lists nothing: stray
  route: remove `a-span-of-the-route` and re-run
blocking · setup.later — the next finding lists:
  nor-this
";
    assert_eq!(
        route_case::listed_in(said, "setup.install-hook"),
        ["first", "kept here", "second"],
        "the listing of the finding that carries the code, and of no other",
    );
    assert_eq!(route_case::listed_in(said, "setup.other"), ["not-this"]);
    assert!(route_case::listed_in(said, "setup.absent").is_empty());

    const AS_LISTED: &[Act] = &[
        Act::RunFilled(Filled {
            line: 0,
            with: &[0, 1, 2],
        }),
        Act::Run(1),
    ];
    const OUT_OF_ORDER: &[Act] = &[
        Act::RunFilled(Filled {
            line: 0,
            with: &[2, 1, 0],
        }),
        Act::Run(1),
    ];
    const AS_PRINTED: &[Act] = &[Act::Run(0), Act::Run(1)];
    const ONE_UNFILLED: &[Act] = &[Act::RunFilled(Filled {
        line: 0,
        with: &[0, 1],
    })];
    const NOT_LISTED: &[Act] = &[Act::RunFilled(Filled {
        line: 0,
        with: &[0, 1, 7],
    })];
    const NOTHING_TO_FILL: &[Act] = &[Act::RunFilled(Filled { line: 1, with: &[] })];
    let sh = Path::new("/bin/sh");
    let planted = |acts: &'static [Act]| Case {
        name: "planted-a-line-filled-with-what-the-refusal-lists",
        state: &[Step::File(Place::Repo("kept here"), "mine\n")],
        refusing: &["-c", LISTS],
        acts,
        ..PLANTED
    };
    let followed = route_case::follow(sh, &planted(AS_LISTED))
        .expect("the planted case is one the runner follows");
    assert!(
        followed.classification == Classification::Holds
            && followed.detail.contains(
                "x first 'kept here' second` (span 0, `<id>` filled by listed value 0, \
                 `<path>` filled by listed value 1, `<id>` filled by listed value 2): exit 0"
            ),
        "each placeholder is filled with its listed value, as one word, and the line is \
         run — it came out {}:\n{}",
        followed.classification,
        followed.detail,
    );

    come_out(&[
        // The values arrive in the order the act gives them, and in no other.
        (planted(OUT_OF_ORDER), Classification::AnotherFailure),
        // Run as printed, the line is what it was before: no shell splits it.
        (planted(AS_PRINTED), Classification::UnsplittableLine),
    ]);

    for (acts, says) in [
        (ONE_UNFILLED, "holds 3 placeholders, with 2 values"),
        (NOT_LISTED, "the refusal lists 3"),
        (NOTHING_TO_FILL, "holds no placeholder"),
    ] {
        match route_case::follow(sh, &planted(acts)) {
            Err(fault) => assert!(fault.contains(says), "{says}: {fault}"),
            Ok(followed) => panic!(
                "{says}: the case was followed — {}\n{}",
                followed.classification, followed.detail
            ),
        }
    }
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

// ---------------------------------------------------------------------------
// The declared bounds
// ---------------------------------------------------------------------------

/// **A case's bound is read off its state — its steps and its environment — and never off
/// its acts; and a word about it stands only where the state stands on a bound and on a
/// state that is none.** On planted states, kind by kind: what plants each of the three
/// bounds; what is a link, a configuration or another variable and no bound; an act that
/// changes a permission, which is a route followed and no state; and the states that stand
/// on both, which the function names and does not decide. Then the word: it decides where
/// the state stands on both, is owed there, and is a fault anywhere else.
///
/// Then this suite's: every case of the catalog has a bound or none without a fault — so
/// a word is exactly where [`route_case::standing`] answers *both*. And **a bound makes no
/// case green**: neither the runner nor the arm that follows the cases reads one.
#[test]
fn a_cases_bound_is_read_off_its_state_and_never_off_its_acts() {
    const SHUT: Step = Step::Git(&["-c", "alias.shut=!chmod 444 CLAUDE.md", "shut"]);
    const UNBORN: Step = Step::Git(&["update-ref", "-d", "HEAD"]);
    const ADAPTERS: Var = Var::Path("JIGC_ADAPTERS_DIR", Place::Outside("adapters"));
    let planted = |state: &'static [Step], env: &'static [Var], acts: &'static [Act]| Case {
        state,
        env,
        acts,
        ..PLANTED
    };
    let bit = Standing::On(Bound::PermissionBit);
    let node = Standing::On(Bound::SpecialFile);
    let stands: &[(&str, Case, Standing)] = &[
        (
            "nothing planted",
            planted(&[], &[], &[]),
            Standing::Unbounded,
        ),
        (
            "a file, added and committed",
            planted(
                &[
                    Step::File(Place::Repo("CLAUDE.md"), "mine\n"),
                    Step::Git(&["add", "--all"]),
                    Step::Git(&["commit", "-q", "-m", "theirs"]),
                ],
                &[],
                &[],
            ),
            Standing::Unbounded,
        ),
        (
            "a `chmod` a git alias runs",
            planted(&[SHUT], &[], &[]),
            bit,
        ),
        (
            "a `chmod` at the end of a longer shell line",
            planted(
                &[Step::Git(&[
                    "-c",
                    "alias.plant=!cp a ../outside/a && chmod 444 ../outside/a",
                    "plant",
                ])],
                &[],
                &[],
            ),
            bit,
        ),
        (
            "a FIFO",
            planted(&[Step::Fifo(Place::Repo("CLAUDE.md"))], &[], &[]),
            node,
        ),
        (
            "a `mkfifo` in a shell line",
            planted(
                &[Step::Git(&["-c", "alias.plant=!mkfifo CLAUDE.md", "plant"])],
                &[],
                &[],
            ),
            node,
        ),
        (
            "a `mknod` in a shell line",
            planted(
                &[Step::Git(&["-c", "alias.plant=!mknod null c 1 3", "plant"])],
                &[],
                &[],
            ),
            node,
        ),
        (
            "the override set",
            planted(&[], &[ADAPTERS], &[]),
            Standing::On(Bound::AdaptersOverride),
        ),
        (
            "a permission and a FIFO: the first in the ruling's order",
            planted(&[Step::Fifo(Place::Repo("CLAUDE.md")), SHUT], &[], &[]),
            bit,
        ),
        // No bound: an act, a link, a configuration, another variable.
        (
            "an act that changes a permission — a route followed, no state",
            planted(
                &[],
                &[],
                &[
                    Act::MakeWritable(Operand::Unprinted("CLAUDE.md")),
                    Act::MakeReadable(Operand::Span(0)),
                ],
            ),
            Standing::Unbounded,
        ),
        (
            "a link whose target is `/dev/null` — a link",
            planted(&[Step::LinkTo(Place::Repo(HOOK), "/dev/null")], &[], &[]),
            Standing::Unbounded,
        ),
        (
            "a `core.hooksPath` that names `/dev/null` — a configuration",
            planted(
                &[Step::Git(&["config", "core.hooksPath", "/dev/null"])],
                &[],
                &[],
            ),
            Standing::Unbounded,
        ),
        (
            "`$HOME` taken away — another variable",
            planted(&[], &[Var::Unset("HOME")], &[]),
            Standing::Unbounded,
        ),
        (
            "the override's variable taken away — it is not set",
            planted(&[], &[Var::Unset("JIGC_ADAPTERS_DIR")], &[]),
            Standing::Unbounded,
        ),
        // On a bound and on a state that is none.
        (
            "a link and a permission",
            planted(
                &[
                    Step::Link(Place::Repo("CLAUDE.md"), Place::Outside("theirs")),
                    SHUT,
                ],
                &[],
                &[],
            ),
            Standing::Both(Bound::PermissionBit, "a symbolic link"),
        ),
        (
            "a link a shell line makes, and a permission",
            planted(
                &[Step::Git(&[
                    "-c",
                    "alias.plant=!ln -s \"$PWD/../outside/a\" a && chmod 444 ../outside/a",
                    "plant",
                ])],
                &[],
                &[],
            ),
            Standing::Both(Bound::PermissionBit, "a symbolic link"),
        ),
        (
            "an unborn `HEAD` and a permission",
            planted(&[UNBORN, SHUT], &[], &[]),
            Standing::Both(Bound::PermissionBit, "an unborn `HEAD`"),
        ),
        (
            "a `core.hooksPath` value and a permission",
            planted(
                &[
                    Step::Git(&["config", "core.hooksPath", "../outside/hooks"]),
                    SHUT,
                ],
                &[],
                &[],
            ),
            Standing::Both(Bound::PermissionBit, "a `core.hooksPath` value"),
        ),
        (
            "another variable beside the override",
            planted(&[], &[Var::Unset("HOME"), ADAPTERS], &[]),
            Standing::Both(
                Bound::AdaptersOverride,
                "a variable of the environment that is not the override",
            ),
        ),
    ];
    for (what, case, expected) in stands {
        assert_eq!(route_case::standing(case), *expected, "{what}");
    }

    const ON_BOTH: &[Step] = &[UNBORN, SHUT];
    const ON_THE_BIT: &[Step] = &[SHUT];
    const ON_NO_BOUND: &[Step] = &[UNBORN];
    let word = |state: &'static [Step], over| Case {
        over,
        ..planted(state, &[], &[])
    };
    let the_bound = Some((
        Over::TheBound,
        "planted: without the bit nothing is refused",
    ));
    let the_other = Some((Over::TheOther, "planted: the link is refused by itself"));
    assert_eq!(
        (
            route_case::bound_of(&word(ON_BOTH, the_bound)),
            route_case::bound_of(&word(ON_BOTH, the_other)),
            route_case::bound_of(&word(ON_THE_BIT, None)),
            route_case::bound_of(&word(ON_NO_BOUND, None)),
        ),
        (
            Ok(Some(Bound::PermissionBit)),
            Ok(None),
            Ok(Some(Bound::PermissionBit)),
            Ok(None),
        ),
        "where a state stands on both the word decides, and where it does not the state does",
    );
    for (state, over, says) in [
        (ON_BOTH, None, "does not say which of the two"),
        (ON_THE_BIT, the_bound, "does not stand on both"),
        (ON_NO_BOUND, the_other, "does not stand on both"),
    ] {
        match route_case::bound_of(&word(state, over)) {
            Err(fault) => assert!(fault.contains(says), "{says}: {fault}"),
            Ok(bound) => panic!("{says}: the case was answered {bound:?}"),
        }
    }

    let faults: Vec<String> = cases()
        .iter()
        .filter_map(|case| {
            route_case::bound_of(case)
                .err()
                .map(|fault| format!("`{}`: {fault}", case.name))
        })
        .collect();
    assert!(
        faults.is_empty(),
        "a word about the cause is exactly where the state stands on both:\n\n  {}",
        faults.join("\n  "),
    );

    let frame = rust_source::code_only(include_str!("support/route_case.rs"));
    let suite = rust_source::code_only(include_str!("route_catalog.rs"));
    for (what, body) in [
        (
            "the runner",
            body_of(
                &frame,
                "fn followed(binary: &Path, case: &Case, under_test: Duration) -> \
                 Result<Followed, String> {",
            ),
        ),
        (
            "the classification",
            body_of(&frame, "pub fn classify(seen: &Seen) -> Classification {"),
        ),
        (
            "the list's differences",
            body_of(
                &frame,
                "pub fn differences(expected: &[ExpectedRed], results: &[(&str, \
                 Classification)]) -> Vec<String> {",
            ),
        ),
        (
            "the arm that follows the cases",
            body_of(&suite, "fn follow_the_cases_of(file: &str) {"),
        ),
    ] {
        assert!(
            !body.contains("bound") && !body.contains("standing") && !body.contains("ADMITTED"),
            "{what} reads a bound: a bound says whether a red case can block, and makes \
             none green",
        );
    }
}

/// Every key of `admitted` that no entry of `expected` lists.
fn admitted_and_not_listed(admitted: &[(&str, &str)], expected: &[ExpectedRed]) -> Vec<String> {
    admitted
        .iter()
        .filter(|(key, _)| !expected.iter().any(|entry| entry.key == *key))
        .map(|(key, ruling)| {
            format!(
                "`{key}` is admitted by name — {ruling} — and no entry of the expected-red \
                 list is under it: an admitted key is a key of a red case"
            )
        })
        .collect()
}

/// **What can block, a line a key** — tab-separated: the key, the red cases listed under
/// it, how many of them can block, how many stand on each of the three bounds, and whether
/// the key is admitted by name. A case can block where its refusal is over no bound, or
/// where its key is admitted. Then a line of sums, and a line for the cases no entry lists
/// — the ones that hold — which says how many of those stand on each bound.
///
/// It counts. A bound takes no case off the expected-red list and changes no
/// classification; what blocks a call is a ruling's to say, and this is that ruling's
/// arithmetic over the cases.
fn bounds(cases: &[&Case], expected: &[ExpectedRed], admitted: &[(&str, &str)]) -> String {
    let bound_of = |case: &Case| {
        route_case::bound_of(case)
            .unwrap_or_else(|fault| panic!("`{}` has no bound to count: {fault}", case.name))
    };
    let per_bound = |cases: &[&Case]| -> String {
        Bound::ALL
            .iter()
            .map(|bound| {
                cases
                    .iter()
                    .filter(|case| bound_of(case) == Some(*bound))
                    .count()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\t")
    };
    let is_admitted = |key: &str| admitted.iter().any(|(named, _)| *named == key);
    let can_block = |key: &str, cases: &[&Case]| {
        if is_admitted(key) {
            cases.len()
        } else {
            cases.iter().filter(|case| bound_of(case).is_none()).count()
        }
    };

    let mut by_key: BTreeMap<&str, Vec<&Case>> = BTreeMap::new();
    let mut listed: Vec<&Case> = Vec::new();
    let mut unlisted: Vec<&Case> = Vec::new();
    for case in cases {
        match expected.iter().find(|entry| entry.case == case.name) {
            Some(entry) => {
                by_key.entry(entry.key).or_default().push(case);
                listed.push(case);
            }
            None => unlisted.push(case),
        }
    }

    let mut out = format!(
        "key\tred cases\tcan block\t{}\tadmitted\n",
        Bound::ALL.map(Bound::as_str).join("\t")
    );
    let mut blocking = (0, 0);
    for (key, of_key) in &by_key {
        let can = can_block(key, of_key);
        if can > 0 {
            blocking = (blocking.0 + 1, blocking.1 + can);
        }
        out.push_str(&format!(
            "{key}\t{}\t{can}\t{}\t{}\n",
            of_key.len(),
            per_bound(of_key),
            if is_admitted(key) { "admitted" } else { "-" },
        ));
    }
    out.push_str(&format!(
        "every key ({}), of which {} can block\t{}\t{}\t{}\t{}\n",
        by_key.len(),
        blocking.0,
        listed.len(),
        blocking.1,
        per_bound(&listed),
        by_key.keys().filter(|key| is_admitted(key)).count(),
    ));
    out.push_str(&format!(
        "the cases no entry lists\t{}\t-\t{}\t-\n",
        unlisted.len(),
        per_bound(&unlisted),
    ));
    out
}

/// **The keys say what can block, and an admitted key is a listed one.** On planted cases:
/// a key whose cases stand on a bound and on none counts the ones on none; a key wholly
/// inside a bound can block nothing — unless it is admitted by name, and then every case
/// of it can; a case that holds is counted by its bound on the last line and under no key.
/// An admitted key that no entry lists is named. Then this suite's table of admitted keys,
/// against its list; and the ignored arm prints the block.
#[test]
fn the_keys_say_what_can_block_and_an_admitted_key_is_a_listed_one() {
    const SHUT: Step = Step::Git(&["-c", "alias.shut=!chmod 444 CLAUDE.md", "shut"]);
    let planted = |name, state: &'static [Step]| Case {
        name,
        state,
        ..PLANTED
    };
    let cases = [
        planted("shut-and-red", &[SHUT]),
        planted("plain-and-red", &[]),
        planted("held", &[Step::Fifo(Place::Repo("CLAUDE.md"))]),
        planted("shut-admitted", &[SHUT]),
        planted("shut-and-holds", &[SHUT]),
        planted("plain-and-holds", &[]),
    ];
    let cases: Vec<&Case> = cases.iter().collect();
    let entry = |case, key, classification| ExpectedRed {
        case,
        key,
        classification,
    };
    let expected = [
        entry("shut-and-red", "r1-mixed", Classification::SameRefusalAgain),
        entry(
            "plain-and-red",
            "r1-mixed",
            Classification::SameRefusalAgain,
        ),
        entry("held", "r1-inside", Classification::DidNotEnd),
        entry("shut-admitted", "r1-named", Classification::AnotherFailure),
    ];
    let admitted = [("r1-named", "a ruling's heading")];
    assert_eq!(
        bounds(&cases, &expected, &admitted),
        "key\tred cases\tcan block\ta permission bit changed by hand\t\
         a FIFO or a device node planted\ta state only `JIGC_ADAPTERS_DIR` reaches\tadmitted\n\
         r1-inside\t1\t0\t0\t1\t0\t-\n\
         r1-mixed\t2\t1\t1\t0\t0\t-\n\
         r1-named\t1\t1\t1\t0\t0\tadmitted\n\
         every key (3), of which 2 can block\t4\t2\t2\t1\t0\t1\n\
         the cases no entry lists\t2\t-\t1\t0\t0\t-\n",
    );

    says(&admitted_and_not_listed(&admitted, &expected), &[]);
    says(
        &admitted_and_not_listed(&[("r1-gone", "a ruling's heading")], &expected),
        &[("`r1-gone`", "no entry of the expected-red list is under it")],
    );
    let wrong = admitted_and_not_listed(ADMITTED, &expected_red());
    assert!(
        wrong.is_empty(),
        "the keys admitted by name, held against the expected-red list:\n\n  {}",
        wrong.join("\n  "),
    );

    let suite = rust_source::code_only(include_str!("route_catalog.rs"));
    let arm = body_of(&suite, &format!("fn {ON_A_NAMED_BINARY}() {{"));
    assert!(
        arm.contains("bounds(&cases, &expected, ADMITTED)"),
        "the ignored arm prints what can block, over this suite's cases, list and \
         admitted keys",
    );
}

// ---------------------------------------------------------------------------
// The cases on a binary a variable names
// ---------------------------------------------------------------------------

/// The variable that names the binary [`the_cases_are_followed_on_the_binary_a_variable_names`]
/// follows every case on, by an absolute path.
const NAMED_BINARY: &str = "JIGC_ROUTE_CATALOG_BINARY";

/// That arm, as the harness names it inside this suite.
const ON_A_NAMED_BINARY: &str = "the_cases_are_followed_on_the_binary_a_variable_names";

/// What one case came to on one binary: a classification — or, where the frame could not
/// follow it there, why.
#[derive(Clone, Debug, PartialEq, Eq)]
enum CameTo {
    Classified(Classification),
    /// **The case could not be followed on that binary, for a reason that is the frame's**:
    /// a state one of its steps could not build there — a step that runs the binary under
    /// test and did not exit 0 — or a case that does not fit the route that binary printed
    /// — an act names a span the route does not have. The first line of what the frame
    /// said, with the machine's temporary directory written `<tmp>`.
    NotFollowed(String),
}

impl fmt::Display for CameTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameTo::Classified(classification) => write!(f, "{classification}"),
            CameTo::NotFollowed(why) => write!(f, "not followed: {why}"),
        }
    }
}

/// One case on one binary.
struct On {
    came: CameTo,
    /// The route that binary printed, with the case's sandbox written `<sandbox>` — where
    /// the case was followed and a route was printed.
    route: Option<String>,
    /// What the runner did, step by step — or all of what the frame said, where it could
    /// not follow the case.
    detail: String,
}

/// One case on two binaries.
struct Beside<'a> {
    case: &'a Case,
    /// On the binary handed in as the named one.
    there: On,
    /// On the other — this tree's, in the arm.
    here: On,
}

/// **Follow `case` on `binary` — its state built by that binary — and say what it came
/// to.** A case the frame cannot follow there is no failure of the caller's: the frame
/// says why by a fault it hands back, or by a panic of the runner's where a state's step
/// did not do what the state needs, and either is what the case came to on that binary.
fn came_to(binary: &Path, case: &Case, limit: std::time::Duration) -> On {
    let not_followed = |said: &str| {
        // The machine's temporary directory, as it resolves and as it is spelled — read
        // so that it can be written `<tmp>`; nothing is made in it.
        let roots = [
            std::env::temp_dir().canonicalize().ok(),
            Some(std::env::temp_dir()),
        ];
        let line = roots.iter().flatten().fold(
            said.lines().next().unwrap_or_default().to_string(),
            |line, root| line.replace(root.to_string_lossy().trim_end_matches('/'), "<tmp>"),
        );
        On {
            came: CameTo::NotFollowed(line),
            route: None,
            detail: said.to_string(),
        }
    };
    match std::panic::catch_unwind(|| route_case::follow_within(binary, case, limit)) {
        Ok(Ok(followed)) => On {
            came: CameTo::Classified(followed.classification),
            route: followed
                .route
                .map(|route| route.replace(&followed.sandbox.display().to_string(), "<sandbox>")),
            detail: followed.detail,
        },
        Ok(Err(fault)) => not_followed(&fault),
        Err(panic) => not_followed(
            panic
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic.downcast_ref::<&str>().copied())
                .unwrap_or("the runner stopped and said nothing"),
        ),
    }
}

/// **Every case of `cases` on two binaries, side by side** — each binary handed the case
/// whole, so the state it is asked in is the state it built. `expected` is the list the
/// `here` binary's known holds are read from; on the named binary nothing is known, and a
/// command there is waited for under the standing limit.
fn on_two_binaries<'a>(
    there: &Path,
    here: &Path,
    cases: &[&'a Case],
    expected: &[ExpectedRed],
) -> Vec<Beside<'a>> {
    route_case::pooled(route_case::width(), cases, |case| Beside {
        case,
        there: came_to(there, case, route_case::LIMIT),
        here: came_to(here, case, route_case::limit_of(expected, case)),
    })
}

/// Whether a case follows its route by an object or a command the route does not print:
/// the marked form, or the refusing command run again. Read off the acts as they print
/// themselves, so an act added to the frame later is read without a list of acts here.
fn operand_implied(case: &Case) -> bool {
    let acts = format!("{:?}", case.acts);
    acts.contains("Unprinted(") || acts.contains("Rerun")
}

/// **What became of a case's witnesses on one binary**, read off what the runner said:
/// `changed` where it named one that is not what it was, `as they were` where it named
/// none, and `-` where the case was not followed there and nothing was compared.
///
/// *Why beside the classification.* The frame reads *not provoked* ahead of a changed
/// witness: a command that exits 0 and prints no refusal has no route to follow, and the
/// classification says that and no more. On a binary that has no such refusal yet, the
/// command may have written the file a later release refuses over — and a table of
/// classifications alone would hold `not provoked` there beside `a witness changed` here,
/// as if nothing had happened to the file on the first.
fn witnesses(on: &On) -> &'static str {
    let named_one = |line: &str| line.starts_with("the witness ") && line.contains(" changed: ");
    match on.came {
        CameTo::Classified(_) if on.detail.lines().any(named_one) => "changed",
        CameTo::Classified(_) => "as they were",
        CameTo::NotFollowed(_) => "-",
    }
}

/// **How the refusing command ended on one binary**, in the runner's own words — `exit 0`,
/// `exit 1`, *did not end within … and was ended* — and `-` where the case was not followed
/// there.
///
/// *Why beside the classification.* *Not provoked* is a command that exited 0 and a command
/// that refused under another code alike: the case's own code was not printed at a
/// non-zero exit, and the frame says no more. On a binary that lacks a refusal the two are
/// different things — it went on and did the work, or another refusal answered first.
fn refusing_ended(on: &On) -> &str {
    // The runner's first line: `jigc <argv>`: <how it ended>, `<code>` printed | not printed
    let ended = || {
        let (ran, _) = on.detail.lines().next()?.rsplit_once(", `")?;
        Some(ran.rsplit_once("`: ")?.1)
    };
    match on.came {
        CameTo::Classified(_) => ended().unwrap_or("-"),
        CameTo::NotFollowed(_) => "-",
    }
}

/// **Whether the two binaries printed the same route**, byte for byte once each sandbox's
/// own root is written `<sandbox>`: `the same`, `not the same`, and `-` where one of them
/// printed none — it refused nothing, or the case was not followed there.
///
/// *Why the table says it.* A case's acts name spans by index, and were written by reading
/// the route this tree prints. Where another binary prints another route under the same
/// code, the same indices are other words: what the case came to there is those acts done
/// to that route, and not that route followed as somebody read it.
fn routes(row: &Beside) -> &'static str {
    match (&row.here.route, &row.there.route) {
        (Some(here), Some(there)) if here == there => "the same",
        (Some(_), Some(_)) => "not the same",
        _ => "-",
    }
}

/// **The table: a line a case, tab-separated** — the case, its site, whether its route
/// prints the operand of every act (`printed`) or the case names one (`implied`), whether
/// it rests on a doubtful reading, the key it is listed red under on this tree; then for
/// this tree's binary and for the named one what the case came to, how its refusing
/// command ended ([`refusing_ended`]) and what became of its witnesses ([`witnesses`]);
/// and whether the two printed the same route ([`routes`]). It says what happened on
/// each, and nothing about what a difference is.
fn table(rows: &[Beside], expected: &[ExpectedRed]) -> String {
    let mut out = String::from(
        "case\tsite\toperand\tdoubt\tkey\t\
         this tree's binary\tits refusing command\tits witnesses\t\
         the named binary\tits refusing command\tits witnesses\tthe two routes\n",
    );
    for row in rows {
        let key = expected
            .iter()
            .find(|entry| entry.case == row.case.name)
            .map_or("-", |entry| entry.key);
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{key}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            row.case.name,
            row.case.site,
            if operand_implied(row.case) {
                "implied"
            } else {
                "printed"
            },
            if row.case.doubt.is_some() {
                "doubt"
            } else {
                "-"
            },
            row.here.came,
            refusing_ended(&row.here),
            witnesses(&row.here),
            row.there.came,
            refusing_ended(&row.there),
            witnesses(&row.there),
            routes(row),
        ));
    }
    out
}

/// **How many cases came to each pair** — what a case came to on this tree's binary, what
/// it came to on the named one, and the number of cases of that pair; a line a pair, in
/// the order of the words. A count, and no word about a pair.
fn pairs(rows: &[Beside]) -> String {
    let mut counted: BTreeMap<(String, String), usize> = BTreeMap::new();
    for row in rows {
        *counted
            .entry((row.here.came.to_string(), row.there.came.to_string()))
            .or_default() += 1;
    }
    let mut out = String::from("cases\tthis tree's binary\tthe named binary\n");
    for ((here, there), cases) in counted {
        out.push_str(&format!("{cases}\t{here}\t{there}\n"));
    }
    out
}

/// A reason's word.
fn word(why: &Why) -> &'static str {
    match why {
        Why::NotConstructible(_) => "not-constructible",
        Why::Platform(_) => "platform",
        Why::NoRoutePrinted => "no-route-printed",
    }
}

/// **The catalog's own numbers, a line a site**: the command's file, the site, the causes
/// its reading counts, how many of them have a reason and no case, how many ways out of
/// its route are not followed, its cases, how many of those the expected-red list holds,
/// and how the count of causes was derived. A site with a reason and no case has no
/// reading: its line carries the reason where the derivation would stand.
fn sites(expected: &[ExpectedRed]) -> String {
    let mut out = String::from(
        "file\tsite\tcauses\twith a reason\tways not followed\tcases\tlisted red\tderivation\n",
    );
    for table in TABLES {
        let cases: Vec<&Case> = table.cases.iter().flat_map(|group| group.iter()).collect();
        for reading in table.readings {
            let of_site = || cases.iter().filter(|case| case.site == reading.site);
            let red = of_site()
                .filter(|case| expected.iter().any(|entry| entry.case == case.name))
                .count();
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{red}\t{}\n",
                table.file,
                reading.site,
                reading.causes,
                reading.without_case.len(),
                reading.not_followed.len(),
                of_site().count(),
                reading.derivation,
            ));
        }
        for reason in table.reasons {
            out.push_str(&format!(
                "{}\t{}\t-\t-\t-\t0\t0\t{}\n",
                table.file, reason.site, reason.why,
            ));
        }
    }
    out
}

/// **The reasons, by word**: how many sites, causes and ways out have a reason of each of
/// the three words, a line a kind and a word.
fn reasons() -> String {
    let mut counted: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    for table in TABLES {
        for reason in table.reasons {
            *counted.entry(("a site", word(&reason.why))).or_default() += 1;
        }
        for reading in table.readings {
            for (_, why) in reading.without_case {
                *counted.entry(("a cause", word(why))).or_default() += 1;
            }
            for (_, why) in reading.not_followed {
                *counted.entry(("a way out", word(why))).or_default() += 1;
            }
        }
    }
    let mut out = String::from("what has the reason\tword\thow many\n");
    for ((what, word), many) in counted {
        out.push_str(&format!("{what}\t{word}\t{many}\n"));
    }
    out
}

/// **The cases that rest on a doubtful reading**, a line a case: what it came to on each
/// binary, and the two readings in the case's own words.
fn doubts(rows: &[Beside]) -> String {
    let mut out = String::from("case\tthis tree's binary\tthe named binary\tthe two readings\n");
    for row in rows {
        if let Some(doubt) = row.case.doubt {
            out.push_str(&format!(
                "{}\t{}\t{}\t{doubt}\n",
                row.case.name, row.here.came, row.there.came
            ));
        }
    }
    out
}

/// **The table follows each binary with the state that binary built.** Two shells stand in
/// for two releases, and a planted case tells them apart by the name each was started
/// under: its state's step writes that name down, and its refusing command refuses only
/// where the name written is its own — with a route that holds under one shell and fails
/// under the other. So each column is what its own binary did from the state it made: a
/// table that drove one binary for both columns would print one classification twice, and
/// one that built the state with one binary and asked the other would print *not provoked*.
/// Handed the two the other way round, the columns change places.
///
/// And three things a table of two classifications would drop, each a planted case: a case
/// whose state a binary cannot build is a line that says so in the frame's words — never a
/// line that is missing, and never the end of the table; a binary that refuses nothing and
/// writes the case's witness is *not provoked* on a line that still says the witness
/// changed; and two binaries that print one route, each with its own sandbox's path in it,
/// are said to print the same route — as two that print different ones are not.
///
/// The source half: the ignored arm hands the variable's value, and this suite's binary,
/// to the function this arm holds.
#[test]
fn the_table_follows_each_binary_with_the_state_that_binary_built() {
    const WHICH: &str = "case \"$(cat built-by 2>/dev/null):$0\" in \
         /bin/sh:/bin/sh) printf '%s\\n' 'blocking · setup.install-hook — refused' \
         '  route: re-run `jigc -c true`'; exit 1;; \
         /bin/bash:/bin/bash) printf '%s\\n' 'blocking · setup.install-hook — refused' \
         '  route: re-run `jigc -c false`'; exit 1;; \
         esac";
    let answers = Case {
        name: "planted-two-binaries-that-answer-differently",
        state: &[Step::Jigc(&["-c", "echo \"$0\" > built-by"])],
        refusing: &["-c", WHICH],
        acts: &[Act::Run(0)],
        ..PLANTED
    };
    let one_builds = Case {
        name: "planted-a-state-one-binary-cannot-build",
        state: &[Step::Jigc(&["-c", "test \"$0\" = /bin/bash"])],
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: re-run `jigc -c true`'; exit 1",
        ],
        acts: &[Act::Run(0)],
        doubt: Some("planted: (a) one reading; (b) the other"),
        ..PLANTED
    };
    let writes = Case {
        name: "planted-a-binary-that-refuses-nothing-and-writes-a-witness",
        state: &[Step::File(Place::Outside("theirs"), "theirs\n")],
        refusing: &[
            "-c",
            "case \"$0\" in /bin/sh) echo ours >> ../outside/theirs; exit 0;; esac; \
             printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: re-run `jigc -c true`'; exit 1",
        ],
        acts: &[Act::Run(0)],
        witnesses: &["theirs"],
        ..PLANTED
    };
    let one_route = Case {
        name: "planted-two-binaries-that-print-one-route",
        state: &[Step::File(Place::Repo("in-the-way"), "theirs\n")],
        refusing: &[
            "-c",
            "test -e in-the-way || exit 0; \
             printf '%s\\n' 'blocking · setup.install-hook — refused' \
             \"  route: remove \\`$PWD/in-the-way\\`, then re-run \\`jigc -c true\\`\"; exit 1",
        ],
        acts: &[Act::Remove(Operand::Span(0)), Act::Run(1)],
        ..PLANTED
    };
    let (sh, bash) = (Path::new("/bin/sh"), Path::new("/bin/bash"));
    let cases = [&answers, &one_builds, &writes, &one_route];
    let holds = CameTo::Classified(Classification::Holds);
    let fails = CameTo::Classified(Classification::AnotherFailure);

    let rows = on_two_binaries(sh, bash, &cases, &[]);
    let came: Vec<(&CameTo, &CameTo)> = rows
        .iter()
        .map(|row| (&row.there.came, &row.here.came))
        .collect();
    assert_eq!(
        came[0],
        (&holds, &fails),
        "the planted case on `/bin/sh` and on `/bin/bash`, each from the state it built:\n{}\n\n{}",
        rows[0].there.detail,
        rows[0].here.detail,
    );
    assert!(
        matches!(came[1].0, CameTo::NotFollowed(why) if why.contains("as a state's step"))
            && *came[1].1 == holds,
        "a state `/bin/sh` cannot build is a line that says so, and `/bin/bash` is still \
         asked: {:?}",
        came[1],
    );
    let turned = on_two_binaries(bash, sh, &cases[..1], &[]);
    assert_eq!(
        (&turned[0].there.came, &turned[0].here.came),
        (&fails, &holds),
        "handed the two binaries the other way round, the columns change places",
    );

    let listed = [ExpectedRed {
        case: answers.name,
        key: "r1-a-key",
        classification: Classification::AnotherFailure,
    }];
    let printed = table(&rows, &listed);
    let lines: Vec<&str> = printed.lines().collect();
    assert_eq!(
        (lines.len(), lines[1]),
        (
            5,
            "planted-two-binaries-that-answer-differently\t\
             crates/cli/src/setup.rs::hook_link_refusal::setup.install-hook#1\tprinted\t-\t\
             r1-a-key\tfollowed, another failure\texit 1\tas they were\t\
             followed, holds\texit 1\tas they were\tnot the same",
        ),
        "a line a case — this tree's columns, then the named binary's, then the routes:\n\
         {printed}",
    );
    assert!(
        lines[2].starts_with("planted-a-state-one-binary-cannot-build\t")
            && lines[2].contains(
                "\tprinted\tdoubt\t-\tfollowed, holds\texit 1\tas they were\tnot followed: "
            )
            && lines[2].ends_with("\t-\t-\t-"),
        "the case that could not be followed has its line, and no word on an exit, a \
         witness or a route nobody saw:\n{printed}",
    );
    assert!(
        lines[3]
            .ends_with("\tfollowed, holds\texit 1\tas they were\tnot provoked\texit 0\tchanged\t-"),
        "a binary that refuses nothing and writes the witness is `not provoked`, and the \
         line still says that it exited 0 and that the witness changed:\n{printed}",
    );
    assert!(
        lines[4].ends_with(
            "\tfollowed, holds\texit 1\tas they were\t\
                 followed, holds\texit 1\tas they were\tthe same"
        ),
        "one route printed by two binaries, each with its own sandbox's path in it, is the \
         same route:\n{printed}",
    );
    assert_eq!(
        pairs(&rows),
        format!(
            "cases\tthis tree's binary\tthe named binary\n\
             1\tfollowed, another failure\tfollowed, holds\n\
             1\tfollowed, holds\tfollowed, holds\n\
             1\tfollowed, holds\t{}\n\
             1\tfollowed, holds\tnot provoked\n",
            rows[1].there.came
        ),
    );
    assert_eq!(doubts(&rows).lines().count(), 2, "{}", doubts(&rows));
    let said = [printed, pairs(&rows), doubts(&rows), sites(&[]), reasons()].concat();
    assert!(
        !said.to_lowercase().contains("regress"),
        "what the arm prints says what each binary did, and nothing about what a \
         difference is",
    );

    let suite = rust_source::code_only(include_str!("route_catalog.rs"));
    let arm = body_of(&suite, &format!("fn {ON_A_NAMED_BINARY}() {{"));
    assert!(
        arm.contains("std::env::var_os(NAMED_BINARY)")
            && arm.contains("on_two_binaries(named, binary(), "),
        "the ignored arm follows the cases on the binary the variable names, beside this \
         suite's own",
    );
}

/// **Every case of the catalog on the binary a variable names, beside what it came to on
/// this tree's** — a table, run by hand and by no gate:
///
/// ```text
/// JIGC_ROUTE_CATALOG_BINARY=<an absolute path> \
///     cargo test -p jigc --test g_migration route_catalog::the_cases_are_followed_on_the_binary_a_variable_names -- --ignored --exact --nocapture
/// ```
///
/// Every case is followed twice, once on each binary, **the state built by the binary that
/// is then asked** ([`the_table_follows_each_binary_with_the_state_that_binary_built`]). A
/// case the frame cannot follow on the named binary — a state it cannot build there, a
/// route with fewer spans than the case's acts name — keeps its line, which says why.
///
/// It prints six blocks, each tab-separated under a line `== <name> ==`: `cases`, the
/// table; `pairs`, how many cases came to each pair of outcomes; `sites` and `reasons`,
/// the catalog's own numbers, which need no binary; `bounds`, what can block key by key
/// ([`bounds`]), which needs none either; `doubts`, the cases that rest on a reading
/// nobody has given; and then, for a reader of the run, what the runner did in every case
/// whose two outcomes are not the same words.
///
/// **It asserts that it ran** — a binary was named, and every case has its line — and
/// nothing about what a case came to on either binary or about a pair: whether a
/// difference between two releases is anything is not this suite's to say.
#[test]
#[ignore = "a table, not a fence: every case on the binary JIGC_ROUTE_CATALOG_BINARY names"]
fn the_cases_are_followed_on_the_binary_a_variable_names() {
    let named = std::env::var_os(NAMED_BINARY).unwrap_or_else(|| {
        panic!("`{NAMED_BINARY}` names the binary to follow the cases on, and is not set")
    });
    let named = Path::new(&named);
    assert!(
        named.is_absolute() && named.is_file(),
        "`{NAMED_BINARY}` is `{}`: the absolute path of a binary — a case is started in a \
         directory of its own",
        named.display(),
    );
    let (cases, expected) = (cases(), expected_red());
    let rows = on_two_binaries(named, binary(), &cases, &expected);

    println!("== cases ==\n{}", table(&rows, &expected));
    println!("== pairs ==\n{}", pairs(&rows));
    println!("== sites ==\n{}", sites(&expected));
    println!("== reasons ==\n{}", reasons());
    println!("== bounds ==\n{}", bounds(&cases, &expected, ADMITTED));
    println!("== doubts ==\n{}", doubts(&rows));
    println!("== what the runner did, where the two outcomes are not the same words ==");
    for row in rows.iter().filter(|row| row.here.came != row.there.came) {
        println!(
            "\n`{}` ({})\n-- the named binary: {}\n{}\n-- this tree's binary: {}\n{}",
            row.case.name,
            row.case.cause,
            row.there.came,
            row.there.detail.trim_end(),
            row.here.came,
            row.here.detail.trim_end(),
        );
    }

    assert_eq!(rows.len(), cases.len(), "every case has its line");
}
