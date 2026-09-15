//! M51 Increment 5, T1 — the release-versioning policy, and the two rules the posture
//! homes never carried.
//!
//! `design/command-output-contract.md` → *Evolution posture (declared)* and its read-side
//! sibling both governed **one contract each**, and both stopped at the same line: they
//! say additive keys are permitted **pre-1.0** and that the shape then evolves by a
//! versioned extension. Three things neither said, and each of them is load-bearing at a
//! 1.0 pin:
//!
//!   * **what a binary version promises.** Nothing in the tree stated jigc's own
//!     versioning — what exists governs *contracts* — so `1.0.0` carried a pin and no
//!     rule about what the next release may do to it (D6; the gap recorded at G-29).
//!   * **whether a key may be *removed* before the pin.** Both homes authorize an
//!     *addition*; the policy above governs *inside 1.x*; between them sat the removal
//!     case, unwritten — and it is the sentence that authorizes this increment's four
//!     deletions (the §7/A7 amendment). A removal executed before its authorizing rule is
//!     written is the undeclared act this wave exists to close.
//!   * **when a constant is a fact and when it is waste.** Two constants ship *declared*
//!     with their reasons (`committed: false`, `findings: []`), four ship undeclared — and
//!     with no stated rule the two look like the same thing (D5's constant rule).
//!
//! Doc-content assertions by nature — the deliverable **is** the prose, exactly as
//! `crates/cli/tests/exit_flip_count_record.rs` fences its own sweep's prose. Each clause
//! is asserted **exactly once and inside the named section**: a policy restated elsewhere
//! is the cross-reference rot this repo's one-file-one-purpose rule exists to prevent, and
//! a policy stated outside its declared home is one a reader has no path to.

use std::fs;
use std::path::{Path, PathBuf};

const DOC: &str = "design/command-output-contract.md";
const HEADING: &str = "Evolution posture (declared)";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

/// The `## <heading>` section body — up to the next `## ` heading or EOF.
fn posture_section() -> String {
    let path = repo_root().join(DOC);
    let body = fs::read_to_string(&path).unwrap_or_else(|_| panic!("{DOC} must exist: {path:?}"));
    let marker = format!("## {HEADING}\n");
    let start = body
        .find(&marker)
        .unwrap_or_else(|| panic!("{DOC} must carry a `## {HEADING}` section"));
    let rest = &body[start + marker.len()..];
    match rest.find("\n## ") {
        Some(end) => rest[..end].to_string(),
        None => rest.to_string(),
    }
}

#[track_caller]
fn stated_once(clause: &str, what: &str) {
    let section = posture_section();
    let hits = section.matches(clause).count();
    assert_eq!(
        hits, 1,
        "{DOC} → {HEADING} must state {what} exactly once, and carried it {hits} time(s):\n  \
         {clause:?}"
    );
}

/// D6 — the policy, and the subject it is stated over: not this doc's three shapes but
/// every machine surface the binary emits.
#[test]
fn the_posture_section_states_the_inside_1x_policy_over_every_json_surface() {
    stated_once(
        "every `--format json` surface the binary emits",
        "the widened subject — the policy governs the binary's output, not this doc's \
         three shapes alone",
    );
    stated_once(
        "Inside `1.x`, nothing pinned is removed or reshaped",
        "the inside-1.x policy",
    );
    stated_once(
        "contract-versions and schema-versions only increase",
        "the monotonic-integer half of the policy",
    );
    stated_once(
        "A removal or a reshape is `2.0`",
        "the major-version half of the policy",
    );
}

/// §7/A7 — the rule that authorizes a removal taken *before* the pin. Without it, this
/// increment's four deletions are an unwritten exception to a written policy.
#[test]
fn the_posture_section_states_the_pre_pin_removal_rule() {
    stated_once(
        "while the window is open, an undeclared key may be removed",
        "the pre-pin removal rule",
    );
    stated_once(
        "what the 1.0 pin freezes is the declared set",
        "the reason the pre-pin removal rule exists — an undeclared key would otherwise \
         be blessed by omission",
    );
}

/// D5 — the line between the four undeclared constants and the two declared ones, with
/// both declared constants named so the rule has its worked instances.
#[test]
fn the_posture_section_states_the_constant_rule_and_names_both_declared_constants() {
    stated_once(
        "a constant ships iff it is declared as one and its reason is written down",
        "the constant rule",
    );
    stated_once(
        "`committed: false` on all six `ConfigAck`s",
        "the first declared constant, named at the rule",
    );
    stated_once(
        "`findings: []` on `jigc task diff` / `jigc task bind` / `jigc task discard`",
        "the second declared constant, named at the rule",
    );
}

/// M51 Increment 6 — the third pre-pin case. Addition is authorized by both posture
/// homes and removal by the rule above; a door that already emits a shape and emits a
/// **different** one was authorized by neither, and Increment 6 took exactly that act at
/// twenty-five verbs. The rule and its bound are asserted here rather than left to the
/// per-increment record, because a policy stated only in `DECISIONS.md` is one this
/// contract's readers have no path to.
#[test]
fn the_posture_section_states_the_pre_pin_reshape_rule_and_its_bound() {
    stated_once(
        "a reshape of a door's declared arm is admissible only while the window is open",
        "the pre-pin reshape rule",
    );
    stated_once(
        "after the pin the identical repair is a `2.0` act",
        "the bound the reshape rule carries — stated, not left as an implication",
    );
}
