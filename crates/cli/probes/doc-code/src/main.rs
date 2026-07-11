//! The dev pack's `doc-code` probe executable (T1 skeleton) — a standalone program
//! that lives **outside** the engine workspace ([module-layout.md] → Probe boundary)
//! and binds to the locked JSON wire contract ([validation.md] → The wire contract).
//!
//! It cannot depend on `engine`, so it **re-declares** the request / response /
//! snapshot / finding serde shapes as its own structs matching the locked JSON form
//! (`engine::probe`). The engine's `response_findings_deserialize_into_the_one_finding_shape`
//! test already proves probe-authored JSON ingests into the engine's one `Finding`.
//!
//! ## T1 scope — wire-conformant I/O + bare-path/file-existence resolution
//!
//! 1. Read a [`ProbeRequest`] from **stdin** (JSON).
//! 2. Read the [`EffectiveStateSnapshot`] from `effective_state.snapshot_path` (JSON).
//! 3. For each anchor, resolve its **file** — the `<path>` before any `#` — against
//!    `working_tree_root`. A **missing** file yields one blocking
//!    `doc-code.<check_id>` finding (keyed on the anchor's `address`); a **present**
//!    file (or directory) yields none. A bare `<path>` (no `#symbol`) is exactly the
//!    file-existence check ([validation.md] → The anchor grammar: a bare path degrades
//!    to a file-existence check).
//! 4. Write a valid [`ProbeResponse`] JSON to **stdout** and exit `0`.
//!
//! ## T2 scope — tree-sitter `#symbol` resolution
//!
//! When an anchor carries a `#symbol`, a present `.rs` file is parsed via tree-sitter
//! (the Rust grammar, chosen by file extension) and `<symbol>` is resolved against the
//! file's named items **at any nesting** (top level, `mod` bodies, `impl` blocks). An
//! unresolvable symbol yields one blocking `doc-code.<check_id>` finding ([resolve]). An
//! absent file still blocks (file-existence subsumed — no AST to resolve against).
//!
//! ## T3 scope — the `criterion-maps-to-test` is-a-test predicate
//!
//! For a `criterion-maps-to-test` anchor the resolved `#symbol` must additionally satisfy
//! the **is-a-test predicate** — a `#[test]`-attributed `fn` at any nesting ([resolve]). A
//! symbol that resolves but is not a `#[test]` fn yields one blocking
//! `doc-code.criterion-maps-to-test` finding; a `symbol-exists` anchor is unaffected by
//! the predicate. Resolution is static parse only — no `cargo`/build/network/wall-clock —
//! so the response is a pure function of (code + anchors) and identical across runs.
//!
//! ## M27 scope — multi-language dispatch + shebang sniff
//!
//! The grammar is chosen by extension first ([`resolve::grammar_for`]) and the extension
//! **always wins**; for a file whose extension maps to no grammar, a `#!…sh` shebang on the
//! first line dispatches to bash ([`resolve::grammar_for_shebang`]) — the dominant
//! extensionless-script case, a pure file-bytes read (determinism intact). A present file
//! carrying a `#symbol` whose extension+shebang map to no shipped grammar emits a non-blocking
//! `doc-code.unsupported-language` advisory ([`Finding::unsupported_language`]) — the
//! uncheckable citation surfaced as uncheckable, replacing M10's silent skip (fork F2).

mod resolve;

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

/// The wire-contract schema version — must match the engine's `result::SCHEMA_VERSION`.
const SCHEMA_VERSION: u32 = 2;

/// The **request** envelope the engine writes to this probe's stdin — the re-declared
/// projection of `engine::probe::ProbeRequest` (the locked field order).
#[derive(Debug, Deserialize)]
struct ProbeRequest {
    #[allow(dead_code)]
    probe_id: String,
    #[allow(dead_code)]
    target: String,
    effective_state: ProbeEffectiveState,
    #[allow(dead_code)]
    #[serde(default)]
    config: serde_json::Map<String, serde_json::Value>,
    #[allow(dead_code)]
    schema_version: u32,
}

/// The `effective_state` member — the path-ref to the materialized snapshot.
#[derive(Debug, Deserialize)]
struct ProbeEffectiveState {
    snapshot_path: PathBuf,
}

/// The read-only **effective-state snapshot** the engine materialized — the re-declared
/// projection of `engine::probe::EffectiveStateSnapshot` (the pinned field order
/// `anchors` then `working_tree_root`).
#[derive(Debug, Deserialize)]
struct EffectiveStateSnapshot {
    anchors: Vec<TargetAnchor>,
    working_tree_root: PathBuf,
}

/// One `(target-address, anchor-value, check-id)` pair — the re-declared projection of
/// `engine::target_surface::TargetAnchor`.
#[derive(Debug, Deserialize)]
struct TargetAnchor {
    address: String,
    anchor_value: String,
    check_id: String,
}

/// The **response** envelope this probe writes to stdout — the re-declared projection
/// of `engine::probe::ProbeResponse` (`findings` then `schema_version`).
#[derive(Debug, Serialize)]
struct ProbeResponse {
    findings: Vec<Finding>,
    schema_version: u32,
}

/// The engine's one [`Finding`] shape, re-declared. The engine deserializes `probe` /
/// `check` with `#[serde(default)]`, but this probe emits them explicitly (split from
/// the `code`) so the ingested finding carries the `(probe, check)` handle directly.
#[derive(Debug, Serialize)]
struct Finding {
    severity: Severity,
    probe: String,
    check: String,
    code: String,
    message: String,
    location: Option<Location>,
    route: Option<String>,
}

/// The finding severity — kebab-case, matching `engine::finding::Severity`. The engine
/// already accepts `advisory` (no engine change); `unsupported-language` is the probe's
/// first advisory emitter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Severity {
    Blocking,
    Advisory,
}

/// Where a finding points — matching `engine::finding::Location` (an `address` plus a
/// 1-based source coordinate). A dangling anchor cites the doc that carries it, at the
/// doc's first coordinate (the engine owns precise line/col; the probe has only the
/// address it was handed).
#[derive(Debug, Serialize)]
struct Location {
    address: String,
    line: usize,
    col: usize,
}

impl Finding {
    /// One blocking `doc-code.<check_id>` finding for an anchor whose **file** is absent,
    /// addressed at the target the engine enumerated. `probe` / `check` split from the
    /// dotted code.
    fn dangling_file(anchor: &TargetAnchor, file: &str) -> Self {
        Self::dangling(
            anchor,
            format!(
                "anchor `{}` resolves to no file (`{file}` is absent from the working tree)",
                anchor.anchor_value,
            ),
        )
    }

    /// One blocking `doc-code.<check_id>` finding for an anchor whose file is present but
    /// whose `#symbol` resolves to no named item (at any nesting).
    fn dangling_symbol(anchor: &TargetAnchor, file: &str, symbol: &str) -> Self {
        Self::dangling(
            anchor,
            format!(
                "anchor `{}` resolves to no symbol (`{symbol}` is absent from `{file}`)",
                anchor.anchor_value,
            ),
        )
    }

    /// One blocking `doc-code.criterion-maps-to-test` finding for an anchor whose `#symbol`
    /// resolves to a real named item that is **not** a `#[test]` fn — the is-a-test
    /// predicate failed (T3).
    fn not_a_test(anchor: &TargetAnchor, file: &str, symbol: &str) -> Self {
        let mut finding = Self::dangling(
            anchor,
            format!(
                "anchor `{}` maps to no test (`{symbol}` in `{file}` is not a `#[test]` fn)",
                anchor.anchor_value,
            ),
        );
        // A resolved-but-not-a-test symbol has its own repair: point the criterion at a
        // real test, not the symbol-restore route the floor's dangling cases carry.
        finding.route = Some("point the criterion at a real test, or correct the cited symbol".to_string());
        finding
    }

    /// One **advisory** `doc-code.symlink-anchor` finding for an anchor whose file is a
    /// symlink — uncheckable under the engine's path-local change-set scoping (the citation
    /// resolves through the link target, not the named path), so surfaced as uncheckable
    /// rather than silently followed. Mirrors `unsupported-language`: distinct check-id,
    /// advisory, un-keyed/informational, carrying the informational "no action needed"
    /// route the floor mandates for an uncheckable-by-design outcome (`validation.md`
    /// → An informational route).
    fn symlink_anchor(anchor: &TargetAnchor, file: &str) -> Self {
        Self {
            severity: Severity::Advisory,
            probe: "doc-code".to_string(),
            check: "symlink-anchor".to_string(),
            code: "doc-code.symlink-anchor".to_string(),
            message: format!(
                "anchor `{}` not validated — `{file}` is a symlink (path-locality not guaranteed)",
                anchor.anchor_value,
            ),
            location: Some(Location {
                address: anchor.address.clone(),
                line: 1,
                col: 1,
            }),
            route: Some(
                "no action needed — the anchor is uncheckable by design (path-locality is \
                 not guaranteed through a symlink)"
                    .to_string(),
            ),
        }
    }

    /// One **advisory** `doc-code.unsupported-language` finding for a `#symbol` anchor on a
    /// present file whose extension/shebang maps to no shipped grammar — the citation is
    /// uncheckable, surfaced *as uncheckable* rather than hidden green (fork F2). A distinct
    /// check-id (isolated from `symbol-exists` tuning), un-keyed/informational (no inventory
    /// row, no `knobs.yaml` key — the `file-state.baseline-adopt` precedent).
    fn unsupported_language(anchor: &TargetAnchor, file: &str) -> Self {
        Self {
            severity: Severity::Advisory,
            probe: "doc-code".to_string(),
            check: "unsupported-language".to_string(),
            code: "doc-code.unsupported-language".to_string(),
            message: format!(
                "anchor `{}` not validated — no grammar for `{file}` (uncheckable citation)",
                anchor.anchor_value,
            ),
            location: Some(Location {
                address: anchor.address.clone(),
                line: 1,
                col: 1,
            }),
            route: Some(
                "no action needed — the citation is uncheckable by design (no shipped \
                 grammar for this file)"
                    .to_string(),
            ),
        }
    }

    /// The common blocking-finding shape: `doc-code.<check_id>` keyed on the target.
    ///
    /// Carries the **actionable repair route** for a dangling code citation (the
    /// symbol-exists floor — the universal `finalize` floor's block, `design/validation.md`
    /// → Scope = effective state): the finding's `location.address` already names the
    /// citing doc + field and the `message` names the anchor + symbol + file, so the route
    /// states the two ways out — update the citation to match the renamed/moved code, or
    /// restore what it cites. This turns a blocked `finalize` into a *productively* blocked
    /// one (the weaker model can act on it), uniform across the task surface, the blast
    /// radius, and the store sweep (`not_a_test` overrides it with its own repair).
    fn dangling(anchor: &TargetAnchor, message: String) -> Self {
        Self {
            severity: Severity::Blocking,
            probe: "doc-code".to_string(),
            check: anchor.check_id.clone(),
            code: format!("doc-code.{}", anchor.check_id),
            message,
            location: Some(Location {
                address: anchor.address.clone(),
                line: 1,
                col: 1,
            }),
            route: Some(
                "update the citation to match the renamed/moved code, or restore the cited \
                 symbol (e.g. revert the change)"
                    .to_string(),
            ),
        }
    }
}

/// Split an anchor value into its file portion (before the first `#`) and an optional
/// `#symbol` (a bare path has no `#`, so the symbol is `None`).
fn split_anchor(anchor_value: &str) -> (&str, Option<&str>) {
    match anchor_value.split_once('#') {
        Some((file, symbol)) => (file, Some(symbol)),
        None => (anchor_value, None),
    }
}

/// Resolve every anchor against the working tree: the file before any `#` must exist; a
/// `#symbol` on a present `.rs` file must additionally resolve to a named item (at any
/// nesting) via tree-sitter. Each unresolvable anchor emits one blocking finding.
fn check_anchors(snapshot: &EffectiveStateSnapshot) -> Vec<Finding> {
    snapshot
        .anchors
        .iter()
        .filter_map(|anchor| {
            let (file, symbol) = split_anchor(&anchor.anchor_value);
            let path = snapshot.working_tree_root.join(file);
            // A **symlink** anchor file breaks the engine's path-locality scoping (the
            // citation's resolution depends on the link *target's* bytes, not `file`'s, so a
            // change to the target in a different path would not place `file` in the
            // change-set). Rather than silently follow it — which would make the blast
            // radius miss that drift — surface it as an uncheckable citation (the
            // `unsupported-language` precedent): advisory, non-blocking, never a wrong pass.
            if path.symlink_metadata().is_ok_and(|m| m.file_type().is_symlink()) {
                return Some(Finding::symlink_anchor(anchor, file));
            }
            if !path.exists() {
                return Some(Finding::dangling_file(anchor, file));
            }
            let symbol = symbol?;
            let src = std::fs::read_to_string(&path).ok()?;
            // The grammar is chosen by extension first (it always wins); for a file whose
            // extension maps to no grammar, a `#!…sh` shebang dispatches to bash — the
            // dominant extensionless-script case, a pure file-bytes read (no wall-clock /
            // network / build — determinism intact). A present file whose extension+shebang
            // map to no grammar carries an uncheckable `#symbol` citation: it emits a
            // non-blocking `unsupported-language` advisory (replacing M10's silent skip),
            // surfacing the gap rather than hiding it green (fork F2).
            let Some(grammar) =
                resolve::grammar_for(&path).or_else(|| resolve::grammar_for_shebang(&src))
            else {
                return Some(Finding::unsupported_language(anchor, file));
            };
            if !resolve::symbol_exists(&src, symbol, grammar) {
                // The symbol is absent — the floor of every `#symbol` check, including
                // `criterion-maps-to-test` (whose predicate is symbol existence + is-a-test).
                return Some(Finding::dangling_symbol(anchor, file, symbol));
            }
            // The symbol resolves. `criterion-maps-to-test` additionally requires the
            // is-a-test predicate (a `#[test]` fn); `symbol-exists` is satisfied here. The
            // predicate is **Rust-only** (`#[test]` has no portable cross-language
            // signature). On a Rust file the predicate runs; on a **non-Rust** file the
            // is-a-test half is unverifiable, so a resolved `criterion-maps-to-test` symbol
            // emits one `unsupported-language` advisory (the symbol passed, the test-half
            // could not be checked — fork F4 truth table), never a wrong `not_a_test` block.
            // `symbol-exists` is unaffected — its predicate is symbol existence, already met.
            if anchor.check_id == "criterion-maps-to-test" {
                if grammar == resolve::Grammar::Rust {
                    if !resolve::test_fn_exists_in_rust(&src, symbol) {
                        return Some(Finding::not_a_test(anchor, file, symbol));
                    }
                } else {
                    return Some(Finding::unsupported_language(anchor, file));
                }
            }
            None
        })
        .collect()
}

fn main() -> ExitCode {
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        return ExitCode::FAILURE;
    }
    let request: ProbeRequest = match serde_json::from_str(&input) {
        Ok(request) => request,
        Err(_) => return ExitCode::FAILURE,
    };
    let snapshot_bytes = match std::fs::read(&request.effective_state.snapshot_path) {
        Ok(bytes) => bytes,
        Err(_) => return ExitCode::FAILURE,
    };
    let snapshot: EffectiveStateSnapshot = match serde_json::from_slice(&snapshot_bytes) {
        Ok(snapshot) => snapshot,
        Err(_) => return ExitCode::FAILURE,
    };

    let response = ProbeResponse {
        findings: check_anchors(&snapshot),
        schema_version: SCHEMA_VERSION,
    };
    match serde_json::to_string(&response) {
        Ok(json) => {
            print!("{json}");
            ExitCode::SUCCESS
        }
        Err(_) => ExitCode::FAILURE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A throwaway working-tree dir under the OS temp dir, unique per call.
    fn temp_root() -> PathBuf {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("doc-code-shebang-test-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn anchor(value: &str) -> TargetAnchor {
        anchor_with_check(value, "symbol-exists")
    }

    fn anchor_with_check(value: &str, check_id: &str) -> TargetAnchor {
        TargetAnchor {
            address: "specs/s.md#criteria/c".to_string(),
            anchor_value: value.to_string(),
            check_id: check_id.to_string(),
        }
    }

    /// Write `file` (relative) with `contents` under a fresh root and run `check_anchors`
    /// over a one-anchor snapshot citing `<file>#<symbol>`.
    fn check_one(file: &str, contents: &str, symbol: &str) -> Vec<Finding> {
        check_one_with_check(file, contents, symbol, "symbol-exists")
    }

    /// Like [`check_one`] but with an explicit `check_id` (so a `criterion-maps-to-test`
    /// anchor can be driven, not just the default `symbol-exists`).
    fn check_one_with_check(
        file: &str,
        contents: &str,
        symbol: &str,
        check_id: &str,
    ) -> Vec<Finding> {
        let root = temp_root();
        std::fs::write(root.join(file), contents).unwrap();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor_with_check(&format!("{file}#{symbol}"), check_id)],
            working_tree_root: root,
        };
        check_anchors(&snapshot)
    }

    // An extensionless bash script (the dominant real-world case) carrying a shebang.
    const EXTENSIONLESS_BASH: &str = "\
#!/bin/bash

deploy() {
    echo deploying
}
";

    #[test]
    fn extensionless_shebang_script_dispatches_to_bash_and_resolves() {
        // An extensionless `#!/bin/bash` script dispatches to bash via the shebang sniff and
        // its `function_definition` resolves — no finding.
        assert!(check_one("deploy", EXTENSIONLESS_BASH, "deploy").is_empty());
    }

    #[test]
    fn extensionless_env_shebang_script_dispatches_to_bash() {
        // The `#!/usr/bin/env bash` form also sniffs to bash.
        let src = "#!/usr/bin/env bash\nrun() { :; }\n";
        assert!(check_one("run-it", src, "run").is_empty());
    }

    #[test]
    fn symlink_anchor_file_advises_rather_than_following() {
        // A symlink anchor file is uncheckable under the engine's path-local change-set
        // scoping (the citation resolves through the link target, in a different path), so the
        // probe advises rather than silently following it — never a wrong pass, and the
        // blast radius's path-locality stays sound.
        let root = temp_root();
        std::fs::write(root.join("target.rs"), "pub fn real() {}\n").unwrap();
        std::os::unix::fs::symlink("target.rs", root.join("link.rs")).unwrap();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor("link.rs#real")],
            working_tree_root: root,
        };
        let findings = check_anchors(&snapshot);
        assert_eq!(findings.len(), 1, "exactly one finding, got {findings:?}");
        assert_eq!(findings[0].code, "doc-code.symlink-anchor");
        assert!(matches!(findings[0].severity, Severity::Advisory));
        // An uncheckable-by-design advisory routes the informational no-op, never null
        // (validation.md → the floor rule "every finding routes").
        assert!(
            findings[0]
                .route
                .as_deref()
                .is_some_and(|r| r.contains("no action needed")),
            "symlink-anchor advisory must carry the informational route, got {:?}",
            findings[0].route,
        );
    }

    #[test]
    fn extensionless_shebang_script_blocks_vanished_symbol() {
        // A vanished symbol in a shebang-dispatched bash script still blocks (one finding).
        assert_eq!(check_one("deploy", EXTENSIONLESS_BASH, "vanished").len(), 1);
    }

    #[test]
    fn py_file_with_bash_shebang_stays_python() {
        // Extension always wins: a `.py` file carrying a `#!/bin/bash` shebang is parsed as
        // Python, so a Python `def` resolves (a bash-only construct would not).
        let src = "#!/bin/bash\ndef handler():\n    return 1\n";
        assert!(check_one("app.py", src, "handler").is_empty());
        // And a name absent from the Python AST still blocks — proving Python (not bash) ran.
        assert_eq!(check_one("app.py", src, "vanished").len(), 1);
    }

    #[test]
    fn un_grammared_file_with_symbol_emits_unsupported_language_advisory() {
        // A present file whose extension+shebang map to no grammar, carrying a `#symbol`,
        // emits exactly one `unsupported-language` advisory — not a silent pass, not a
        // blocking `symbol-exists` (the M10 silent-skip M27 superseded, fork F2).
        // A `#!/usr/bin/perl` shebang maps to no grammar (the sniff is `sh`-only — `.pl` is the
        // surviving un-grammared exemplar now that `.yaml` activates at M29), so it takes the
        // advisory rather than the old silent-skip.
        let perl = check_one("script.pl", "#!/usr/bin/perl\nsub thing { }\n", "thing");
        assert_eq!(perl.len(), 1);
        assert_eq!(perl[0].severity, Severity::Advisory);
        assert_eq!(perl[0].check, "unsupported-language");
        // An uncheckable-by-design advisory routes the informational no-op, never null
        // (validation.md → the floor rule "every finding routes").
        assert!(
            perl[0]
                .route
                .as_deref()
                .is_some_and(|r| r.contains("no action needed")),
            "unsupported-language advisory must carry the informational route, got {:?}",
            perl[0].route,
        );
    }

    #[test]
    fn bare_path_on_un_grammared_file_emits_no_finding() {
        // A bare path (no `#symbol`) on an un-grammared file (`.pl` — off-roadmap, no shipped
        // grammar) is the file-existence check only — the present file passes, no advisory
        // (the advisory rides a `#symbol`).
        let root = temp_root();
        std::fs::write(root.join("script.pl"), "sub thing { }\n").unwrap();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor("script.pl")],
            working_tree_root: root,
        };
        assert!(check_anchors(&snapshot).is_empty());
    }

    #[test]
    fn missing_un_grammared_file_still_blocks() {
        // The file-existence floor wins: a `#symbol` anchor on an absent un-grammared file
        // (`.pl`) emits one blocking finding before any grammar/advisory dispatch.
        let root = temp_root();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor("ghost.pl#thing")],
            working_tree_root: root,
        };
        let findings = check_anchors(&snapshot);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Blocking);
    }

    #[test]
    fn missing_file_still_blocks_regardless_of_shebang() {
        // The file-existence floor is unchanged: a `#symbol` anchor on an absent file blocks
        // before any grammar/shebang dispatch.
        let root = temp_root();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor("ghost#deploy")],
            working_tree_root: root,
        };
        assert_eq!(check_anchors(&snapshot).len(), 1);
    }

    // The non-Rust `criterion-maps-to-test` truth table ([validation.md] → Multi-language
    // resolution): the symbol-existence floor blocks a vanished symbol (nothing to advise
    // about); a present symbol — whose is-a-test half is unverifiable cross-language — takes
    // the `unsupported-language` advisory. Exactly one finding either way, never both.

    // A present TypeScript test function, named like a Vitest/Jest test.
    const TS_TEST: &str = "\
import { test, expect } from \"vitest\";

test(\"limits to 100/min\", () => {
    expect(true).toBe(true);
});

export function rateLimitTest(): void {}
";

    // A present Python test function, named like a pytest test.
    const PY_TEST: &str = "\
def test_rate_limit():
    assert True
";

    #[test]
    fn non_rust_maps_to_test_present_symbol_advises_never_blocks() {
        // symbol PRESENT under a non-Rust grammar: exactly one `unsupported-language` advisory
        // (the is-a-test half is Rust-only — unverified), NO `criterion-maps-to-test` block.
        for (file, src, symbol) in [
            ("limit.test.ts", TS_TEST, "rateLimitTest"),
            ("test_limit.py", PY_TEST, "test_rate_limit"),
        ] {
            let findings = check_one_with_check(file, src, symbol, "criterion-maps-to-test");
            assert_eq!(findings.len(), 1, "{file}: exactly one finding");
            assert_eq!(findings[0].severity, Severity::Advisory, "{file}: advisory");
            assert_eq!(findings[0].check, "unsupported-language", "{file}");
            assert_eq!(findings[0].code, "doc-code.unsupported-language", "{file}");
        }
    }

    #[test]
    fn non_rust_maps_to_test_absent_symbol_blocks_never_advises() {
        // symbol ABSENT under a non-Rust grammar: exactly one blocking `criterion-maps-to-test`
        // (the symbol-existence floor — nothing to advise about a vanished symbol), NO advisory.
        for (file, src) in [("limit.test.ts", TS_TEST), ("test_limit.py", PY_TEST)] {
            let findings = check_one_with_check(file, src, "vanished", "criterion-maps-to-test");
            assert_eq!(findings.len(), 1, "{file}: exactly one finding");
            assert_eq!(findings[0].severity, Severity::Blocking, "{file}: blocking");
            assert_eq!(findings[0].check, "criterion-maps-to-test", "{file}");
            assert_eq!(
                findings[0].code, "doc-code.criterion-maps-to-test",
                "{file}"
            );
        }
    }

    #[test]
    fn non_rust_symbol_exists_present_symbol_emits_no_finding() {
        // The test-half advisory is `maps-to-test`-only: a present non-Rust `symbol-exists`
        // anchor passes silently (no `unsupported-language` advisory rides a plain symbol-exists).
        assert!(
            check_one_with_check("limit.test.ts", TS_TEST, "rateLimitTest", "symbol-exists")
                .is_empty()
        );
        assert!(
            check_one_with_check("test_limit.py", PY_TEST, "test_rate_limit", "symbol-exists")
                .is_empty()
        );
    }

    #[test]
    fn rust_not_a_test_block_still_fires() {
        // The Rust is-a-test predicate is unchanged: a resolved Rust symbol that is not a
        // `#[test]` fn still emits one blocking `criterion-maps-to-test`, never the advisory.
        let src = "fn helper() {}\n";
        let findings = check_one_with_check("lib.rs", src, "helper", "criterion-maps-to-test");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Blocking);
        assert_eq!(findings[0].check, "criterion-maps-to-test");
    }

    #[test]
    fn css_file_with_symbol_resolves_present_blocks_vanished() {
        // The M28 keystone activation: a `.css#selector` now resolves through the CSS grammar
        // (no longer the `unsupported-language` advisory) — a present selector emits no
        // finding; a vanished one BLOCKS with one `doc-code.symbol-exists`.
        let css = ".btn { color: red; }\n";
        assert!(check_one("styles.css", css, "btn").is_empty());
        let gone = check_one("styles.css", css, "btn_renamed");
        assert_eq!(gone.len(), 1);
        assert_eq!(gone[0].severity, Severity::Blocking);
        assert_eq!(gone[0].check, "symbol-exists");
        assert_eq!(gone[0].code, "doc-code.symbol-exists");
    }

    #[test]
    fn yaml_file_with_symbol_resolves_present_blocks_vanished() {
        // The M29 activation: a `.yaml#key` now resolves through the YAML grammar (no longer the
        // `unsupported-language` advisory) — a present mapping key emits no finding; a vanished
        // one BLOCKS with one `doc-code.symbol-exists`. (`compose.yaml#web` means "a YAML key
        // named `web` exists" — schema-blind, the F7 honest bound.)
        let compose = "services:\n  web:\n    image: nginx\n";
        assert!(check_one("compose.yaml", compose, "web").is_empty());
        let gone = check_one("compose.yaml", compose, "web_renamed");
        assert_eq!(gone.len(), 1);
        assert_eq!(gone[0].severity, Severity::Blocking);
        assert_eq!(gone[0].check, "symbol-exists");
        assert_eq!(gone[0].code, "doc-code.symbol-exists");
    }
}
