//! **Repro block (pinning.md §3) — the M41 e2e finding-emission order witness, made
//! standing** (the confidence-audit back-sweep, triage item c2 — provenance beyond the
//! parent module's RC-alpha3 ledger: this pins an **M41 audit** witness). The M41
//! completion audit drove it one-off: `completions/artifacts/M41/VERDICT.md` → Fork 1:
//! "a `0..*` two-dangling-ref sweep emits two distinctly-keyed findings, byte-identical
//! across divergent source orders"; → E2e: "the two-divergent-order determinism check".
//! Standing coverage asserts the emitted sets/keys in a **single** source order
//! (`duplicate_field_finding_keys.rs`, flow42's two-dangling-ref arm); nothing drives
//! the store sweep over two orders, so a refactor swapping the sweep's sorted
//! enumeration (`engine::index::committed_instances`' `out.sort()`) for raw `read_dir`
//! iteration would ship driver-visible finding-order nondeterminism — a direct break of
//! the finding-key contract's declared order stability — with every existing test green.
//!
//! ```yaml
//! claim: "the validate store sweep's findings are byte-identical across divergent source orders"
//! verdict: CONFIRMED (M41 e2e audit — one-off; standing as of this test)
//! setup:
//!   - two fresh repos; the SAME three ADRs — each carrying a `0..*` supersedes with
//!     two dangling targets — committed one-per-commit in divergent source orders
//! repro:
//!   - ["jigc", "validate", "--format", "json"]
//! expect:
//!   stdout: the full findings report byte-identical across the two repos
//!   keys: six `schema-conformance.ref-resolves` findings, distinctly keyed per target
//! pinned-by: pinned_facts::finding_emission_order::store_sweep_findings_are_byte_identical_across_divergent_source_orders
//! ```
//!
//! Verified catchable: locally muting `committed_instances`' `out.sort()` lets tmpfs
//! creation order through into the findings array and reddens the byte-compare
//! (mutate → catch → restore; never committed).

use std::collections::BTreeSet;
use std::fs;

use crate::support::trial_corpus::{State, TrialCorpus};

/// A conformant ADR body whose `0..*` `supersedes` carries two dangling targets —
/// the M41 witness's fan-out shape (one keyed finding per dangling target).
fn adr_with_two_dangling_refs(slug: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-25\nschema-version: 2\n\
         supersedes: [adr:ghost-{slug}-one, adr:ghost-{slug}-two]\n---\n\n\
         # {slug}\n\n## Context\n\nForces.\n\n## Options\n\nWeighed.\n\n\
         ## Decision\n\nDo it.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Seed a fresh corpus with the three dangling-ref ADRs in `order` (one commit per
/// doc, so creation *and* history order diverge while the final trees are identical),
/// then return the store sweep's full JSON report bytes.
fn seed_and_sweep(order: &[&str]) -> String {
    let corpus = TrialCorpus::build(State::Fresh);
    let decisions = corpus.repo().join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");
    for slug in order {
        fs::write(
            decisions.join(format!("{slug}.md")),
            adr_with_two_dangling_refs(slug),
        )
        .expect("write adr");
        corpus.git(&["add", "."]);
        corpus.git(&["commit", "-q", "-m", "seed adr"]);
    }

    let sweep = corpus.jigc(&["validate", "--format", "json"]);
    assert!(
        sweep.status.success(),
        "store-scope `jigc validate` is report-only (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&sweep.stdout),
        String::from_utf8_lossy(&sweep.stderr),
    );
    let stdout = String::from_utf8(sweep.stdout).expect("utf-8 report");

    // Non-vacuity: the sweep genuinely fanned six distinctly-keyed ref-resolves
    // findings (two per doc — the M41 per-dangling-target key claim).
    let report: serde_json::Value = serde_json::from_str(&stdout).expect("the report parses");
    let keys: BTreeSet<String> = report["findings"]
        .as_array()
        .expect("`findings` is an array")
        .iter()
        .filter(|f| f["code"] == "schema-conformance.ref-resolves")
        .map(|f| {
            f["key"]["target"]
                .as_str()
                .expect("a keyed target")
                .to_string()
        })
        .collect();
    assert_eq!(
        keys.len(),
        6,
        "the three two-dangling-ref ADRs fan six distinctly-keyed findings; got:\n{stdout}",
    );

    stdout
}

/// **The pin.** The identical dangling-ref corpus, seeded in two divergent source
/// orders, sweeps to a byte-identical `--format json` report.
#[test]
fn store_sweep_findings_are_byte_identical_across_divergent_source_orders() {
    let order_a = ["alpha", "beta", "gamma"];
    let order_b = ["gamma", "alpha", "beta"];
    assert_ne!(order_a, order_b, "the source orders must diverge");

    let report_a = seed_and_sweep(&order_a);
    let report_b = seed_and_sweep(&order_b);

    assert_eq!(
        report_a, report_b,
        "the store sweep's findings report must be byte-identical across divergent \
         source orders",
    );
}
