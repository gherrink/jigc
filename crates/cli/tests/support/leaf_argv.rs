//! **One minimal, clap-satisfying argv per leaf verb** — the shared axis input for every
//! suite that drives *all* of [`cli::cli::VERB_KINDS`] against a condition that fires
//! before any id, path or address is adjudicated.
//!
//! It landed inside `not_in_repo_axis.rs` at M49 and moved here at M52 Increment 1 / T3,
//! when the pre-dispatch fault axis needed the same table. Two copies of "the argv that
//! reaches `jigc doc set-slot`'s precondition" is two homes for one fact, and the second
//! one rots the first time a leaf gains a required argument: the suite that was edited
//! goes green and the one that was not dies at clap with an exit 2 that looks like a
//! *declared carve-out* rather than a stale table.
//!
//! **Nothing in a tail needs to resolve.** Every value is a plausible-but-absent id and
//! every required argument is present — which is the point: a condition that fires ahead
//! of resolution is told *that*, not that its doctype is unknown. A consumer that needs
//! the tails' files on disk writes them itself ([`FIXTURE_FILES`]).

/// One cell of an axis: a leaf verb's [`cli::cli::VERB_KINDS`] path, and the **tail**
/// appended after it to make an argv clap accepts.
pub type Arm = (&'static [&'static str], &'static [&'static str]);

/// The files the tails point at, as `(name, contents)`. A consumer whose fixture
/// directory is not a jigc corpus writes these so that a cell which somehow got past the
/// condition under test fails on something else, loudly, rather than on a missing file
/// that reads like the same refusal.
pub const FIXTURE_FILES: &[(&str, &str)] = &[
    ("FOREIGN.md", "# Foreign\n"),
    ("payload.yaml", "title: X\n"),
    ("step.md", "# Step\n"),
];

/// Every leaf verb, with the argv that reaches its preconditions. Each consumer fences
/// this ⇔ [`cli::cli::VERB_KINDS`] itself, so a verb added to the surface reddens in
/// every suite that sweeps the axis until someone decides what it answers.
pub const MINIMAL_ARGV: &[Arm] = &[
    // Top level.
    (&["start"], &[]),
    (&["workflow"], &["single-task", "--preview"]),
    (&["setup"], &[]),
    (&["uninstall"], &[]),
    (&["upgrade"], &[]),
    (&["ingest"], &[]),
    (&["migrate"], &["FOREIGN.md", "--as", "changelog"]),
    (&["migrate-corpus"], &[]),
    (&["unmanage"], &["FOREIGN.md"]),
    (&["rename"], &["adr:a-decision", "--to", "A Decision"]),
    (&["relocate"], &["changelog", "--from", "docs"]),
    (&["describe"], &[]),
    (&["validate"], &[]),
    // `jigc doc` — the managed-doc surface.
    (&["doc", "create"], &["adr", "--title", "A Decision"]),
    (
        &["doc", "add-item"],
        &["adr:a-decision#options", "--title", "An Option"],
    ),
    (
        &["doc", "remove-item"],
        &["adr:a-decision#options.an-option"],
    ),
    (
        &["doc", "retitle-item"],
        &["adr:a-decision#options.an-option", "--title", "Another"],
    ),
    (
        &["doc", "rename"],
        &["adr:a-decision", "--to", "Another Decision"],
    ),
    (
        &["doc", "set-field"],
        &["adr:a-decision#status", "--value", "accepted"],
    ),
    (
        &["doc", "set-slot"],
        &["adr:a-decision#context", "--from-file", "payload.yaml"],
    ),
    (&["doc", "author"], &["adr", "--from-file", "payload.yaml"]),
    (&["doc", "show"], &["adr:a-decision"]),
    (&["doc", "schema"], &["adr"]),
    (&["doc", "list"], &[]),
    // `jigc task` — the task lifecycle.
    (&["task", "list"], &[]),
    (&["task", "diff"], &["a-task"]),
    (&["task", "validate"], &["a-task"]),
    (&["task", "discard"], &["a-task"]),
    (&["task", "finalize"], &["a-task"]),
    (&["task", "bind"], &["spec", "spec:a-spec", "a-task"]),
    // `jigc config` — the cascade surface.
    (&["config", "set"], &["docs-root", "docs"]),
    (
        &["config", "insert-step"],
        &["step.md", "--workflow", "single-task", "--after", "orient"],
    ),
    (
        &["config", "replace-step"],
        &["single-task:orient", "step.md"],
    ),
    (&["config", "remove-step"], &["single-task:orient"]),
    (
        &["config", "fill"],
        &["single-task:orient", "--from-file", "payload.yaml"],
    ),
    (&["config", "fork"], &["single-task:orient"]),
    (&["config", "get"], &["docs-root"]),
    (&["config", "list"], &[]),
    // `jigc milestone` — the work-unit surface.
    (&["milestone", "create"], &["A Milestone"]),
    (
        &["milestone", "add-task"],
        &["m-a-milestone", "do the thing"],
    ),
    (
        &["milestone", "add-from-spec"],
        &["m-a-milestone", "spec:a-spec"],
    ),
    (&["milestone", "list-tasks"], &["m-a-milestone"]),
    (&["milestone", "provision"], &["m-a-milestone"]),
    (&["milestone", "execute"], &["m-a-milestone"]),
    (&["milestone", "join"], &["m-a-milestone"]),
    (&["milestone", "finalize"], &["m-a-milestone"]),
    (&["milestone", "discard"], &["m-a-milestone"]),
];

/// [`MINIMAL_ARGV`] ⇔ [`cli::cli::VERB_KINDS`]: every leaf verb the clap tree carries has
/// exactly one arm, and no arm names a path the tree no longer has. Called from each
/// consumer's own `#[test]` so the fence fails in the suite that would otherwise skip a
/// verb, not in a shared module nothing names.
pub fn assert_covers_every_leaf_verb() {
    use cli::cli::VERB_KINDS;

    for (path, _) in VERB_KINDS {
        let hits = MINIMAL_ARGV
            .iter()
            .filter(|(known, _)| known == path)
            .count();
        assert_eq!(
            hits,
            1,
            "`jigc {}` is a leaf verb and needs exactly one argv arm; got {hits}",
            path.join(" "),
        );
    }
    for (path, _) in MINIMAL_ARGV {
        assert!(
            VERB_KINDS.iter().any(|(known, _)| known == path),
            "the argv arm for `jigc {}` names a path the clap tree no longer has",
            path.join(" "),
        );
    }
    assert_eq!(
        MINIMAL_ARGV.len(),
        VERB_KINDS.len(),
        "one argv arm per leaf verb, no duplicates",
    );
}
