//! The route catalog's cases for `jigc setup` — the family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.

use super::Reason;
use crate::support::route_case::{Act, Case, Check, Operand, Place, Step};

/// The hook `jigc setup` installs, from the repository root.
pub const HOOK: &str = ".git/hooks/pre-commit";

/// The refusal a `pre-commit` hook draws when it is a link that leads out of the
/// repository and its hooks directory, or to nothing.
pub const HOOK_LINK_SITE: &str = "crates/cli/src/setup.rs::hook_link_refusal::setup.install-hook#1";

/// The cases of `jigc setup`. A case follows one way out of its route: where a route offers two —
/// *remove the link, or point it at a script inside this repository* — the other way is a
/// case of its own.
pub const CASES: &[Case] = &[
    Case {
        name: "setup-hook-linked-to-a-script-outside",
        site: HOOK_LINK_SITE,
        cause: "the `pre-commit` hook is a link to an existing script outside the repository",
        state: &[
            Step::Script(Place::Outside("pre-commit"), "#!/bin/sh\necho ours\n"),
            Step::Link(Place::Repo(HOOK), Place::Outside("pre-commit")),
        ],
        refusing: &["setup"],
        acts: &[Act::Remove(Operand::Span(0)), Act::Run(1)],
        holds: &[Check::RegularFile(Place::Repo(HOOK))],
        witnesses: &["pre-commit"],
    },
    Case {
        name: "setup-hook-linked-to-nothing",
        site: HOOK_LINK_SITE,
        cause: "the `pre-commit` hook is a link to a path outside the repository that holds \
                nothing",
        state: &[Step::Link(Place::Repo(HOOK), Place::Outside("pre-commit"))],
        refusing: &["setup"],
        acts: &[Act::Remove(Operand::Span(0)), Act::Run(1)],
        holds: &[Check::RegularFile(Place::Repo(HOOK))],
        witnesses: &["pre-commit"],
    },
];

/// The sites of `jigc setup` that have no case, each with why.
pub const REASONS: &[Reason] = &[];
