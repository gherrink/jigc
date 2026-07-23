//! The **enumeration seam** — `crates/cli/src/lib.rs` exposes the pack registry and
//! the clap tree to `tests/*.rs`, so every M45 sweep reads its members from a real
//! registry rather than a hand list ([pinning.md](../../../implementation/pinning.md)
//! §1: *enumeration comes from the registries, never a hand list*; §2: the exit-code
//! table's third consumer).
//!
//! This suite is the seam's own fence: it asserts the three consumers are reachable
//! and that what they enumerate is the real composed pack-set — never `make_pack()`,
//! which resolves against the **process CWD** and is a hazard under parallel tests.

use clap::CommandFactory;
use cli::pack::{CompositePack, EmbeddedPack};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

/// The production composition, built the **CWD-free** way: `[dev ▸ methodology]`,
/// dev highest-precedence (`design/multi-pack.md` → Embedded second pack).
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

fn ids(pack: &dyn PackSource, kind: PackResourceKind) -> Vec<String> {
    pack.list(kind)
        .iter()
        .map(ResourceId::to_string)
        .collect::<Vec<_>>()
}

fn is_sorted_and_deduped(ids: &[String]) -> bool {
    ids.windows(2).all(|w| w[0] < w[1])
}

/// The registry consumer (§1): the composite enumerates every workflow and doctype
/// both embedded packs ship, sorted + deduped, with no test-side list.
#[test]
fn composite_registry_enumerates_workflows_and_doctypes() {
    let pack = composite();

    let workflows = ids(&pack, PackResourceKind::Workflows);
    assert_eq!(
        workflows.len(),
        33,
        "the composed pack-set ships 33 workflows: {workflows:?}"
    );
    assert!(
        is_sorted_and_deduped(&workflows),
        "list() is sorted + deduped: {workflows:?}"
    );

    let doctypes = ids(&pack, PackResourceKind::Schemas);
    assert_eq!(
        doctypes.len(),
        15,
        "16 shipped schemas dedup to 15 — `commit` ships in both packs: {doctypes:?}"
    );
    assert!(
        is_sorted_and_deduped(&doctypes),
        "list() is sorted + deduped: {doctypes:?}"
    );

    // The set is exactly the union of the two constituents' own `list()`s — the
    // property that makes "a new pack joins the sweep with zero test edits" true.
    let mut union: Vec<String> = ids(&EmbeddedPack::new(), PackResourceKind::Schemas)
        .into_iter()
        .chain(ids(&EmbeddedPack::methodology(), PackResourceKind::Schemas))
        .collect();
    union.sort();
    union.dedup();
    assert_eq!(doctypes, union, "the composite doctype set is the union");
}

/// The cross-pack collision the composite resolves silently: `commit` ships in
/// **both** packs (`design/multi-pack.md` → Collision resolution), so `owner_count`
/// reports two owners while `list()` reports one id.
#[test]
fn commit_is_a_live_two_owner_collision() {
    let pack = composite();
    let commit = ResourceId::from("commit".to_string());
    assert_eq!(
        pack.owner_count(PackResourceKind::Schemas, &commit),
        2,
        "`commit` is owned by dev *and* methodology"
    );
}

/// One walked node: its space-joined path from the root (`doc show`, `task
/// finalize`, …), whether it is a leaf, and whether it carries `--format`.
struct Node {
    path: String,
    leaf: bool,
    carries_format: bool,
}

/// Every node of the clap tree. Recursive, so a new nesting level joins with no
/// test edit. clap's own auto-generated `help` verb is skipped — it is not a jigc
/// verb and prints no jigc output surface.
fn walk(cmd: &clap::Command, prefix: &str, out: &mut Vec<Node>) {
    for sub in cmd.get_subcommands().filter(|s| s.get_name() != "help") {
        let path = if prefix.is_empty() {
            sub.get_name().to_string()
        } else {
            format!("{prefix} {}", sub.get_name())
        };
        out.push(Node {
            path: path.clone(),
            leaf: sub.get_subcommands().next().is_none(),
            carries_format: sub.get_arguments().any(|a| a.get_id() == "format"),
        });
        walk(sub, &path, out);
    }
}

/// The `CommandFactory` consumer (§2): the verb tree enumerates from the built clap
/// `Command` with zero hand-listed names, and **every leaf carries `--format`** —
/// the premise the machine-output sweep rides on (`--format` is a global arg, so
/// every verb is in scope by construction).
#[test]
fn clap_tree_enumerates_and_every_leaf_carries_format() {
    let mut root = <cli::cli::Cli as CommandFactory>::command();
    // Propagate global args (and clap's own `help` subcommand) exactly as a real
    // parse does, so `--format` is observable where a caller would pass it.
    root.build();

    let mut nodes = Vec::new();
    walk(&root, "", &mut nodes);

    assert!(!nodes.is_empty(), "the walk reaches the verb tree");

    let mut paths: Vec<&str> = nodes.iter().map(|n| n.path.as_str()).collect();
    let count = paths.len();
    paths.sort_unstable();
    paths.dedup();
    assert_eq!(paths.len(), count, "the walk is duplicate-free");

    let missing: Vec<&str> = nodes
        .iter()
        .filter(|n| n.leaf && !n.carries_format)
        .map(|n| n.path.as_str())
        .collect();
    assert!(
        missing.is_empty(),
        "every leaf subcommand carries `--format`: {missing:?}"
    );
}

/// The exit-code consumer (§2): the two named codes are readable through the lib,
/// so the taxonomy suite can assert against them instead of bare literals.
#[test]
fn exit_code_constants_are_readable_through_the_lib() {
    assert_eq!(cli::task::EXIT_VALIDATION_BLOCKED, 3);
    assert_eq!(cli::task::EXIT_REVIEW_PENDING, 4);
}
