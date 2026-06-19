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
//! extensionless-script case, a pure file-bytes read (determinism intact). An un-grammared
//! file with no `sh` shebang keeps the M10 silent-skip (the `unsupported-language` advisory
//! is a later increment).

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

/// The finding severity — kebab-case, matching `engine::finding::Severity`.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Severity {
    Blocking,
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
        Self::dangling(
            anchor,
            format!(
                "anchor `{}` maps to no test (`{symbol}` in `{file}` is not a `#[test]` fn)",
                anchor.anchor_value,
            ),
        )
    }

    /// The common blocking-finding shape: `doc-code.<check_id>` keyed on the target.
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
            route: None,
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
            if !path.exists() {
                return Some(Finding::dangling_file(anchor, file));
            }
            let symbol = symbol?;
            let src = std::fs::read_to_string(&path).ok()?;
            // The grammar is chosen by extension first (it always wins); for a file whose
            // extension maps to no grammar, a `#!…sh` shebang dispatches to bash — the
            // dominant extensionless-script case, a pure file-bytes read (no wall-clock /
            // network / build — determinism intact). An un-grammared file with no `sh`
            // shebang keeps the M10 silent-skip (the `unsupported-language` advisory is a
            // later increment).
            let grammar =
                resolve::grammar_for(&path).or_else(|| resolve::grammar_for_shebang(&src))?;
            if !resolve::symbol_exists(&src, symbol, grammar) {
                // The symbol is absent — the floor of every `#symbol` check, including
                // `criterion-maps-to-test` (whose predicate is symbol existence + is-a-test).
                return Some(Finding::dangling_symbol(anchor, file, symbol));
            }
            // The symbol resolves. `criterion-maps-to-test` additionally requires the
            // is-a-test predicate (a `#[test]` fn); `symbol-exists` is satisfied here. The
            // predicate is **Rust-only** (`#[test]` has no portable cross-language
            // signature), so on a non-Rust file the resolved symbol passes — the
            // `unsupported-language` advisory for the unverified test-half is a later
            // increment, never a wrong `not_a_test` block.
            if grammar == resolve::Grammar::Rust
                && anchor.check_id == "criterion-maps-to-test"
                && !resolve::test_fn_exists_in_rust(&src, symbol)
            {
                return Some(Finding::not_a_test(anchor, file, symbol));
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
        TargetAnchor {
            address: "specs/s.md#criteria/c".to_string(),
            anchor_value: value.to_string(),
            check_id: "symbol-exists".to_string(),
        }
    }

    /// Write `file` (relative) with `contents` under a fresh root and run `check_anchors`
    /// over a one-anchor snapshot citing `<file>#<symbol>`.
    fn check_one(file: &str, contents: &str, symbol: &str) -> Vec<Finding> {
        let root = temp_root();
        std::fs::write(root.join(file), contents).unwrap();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor(&format!("{file}#{symbol}"))],
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
    fn un_grammared_file_without_sh_shebang_silent_skips() {
        // An un-grammared extension with no `sh` shebang keeps the M10 silent-skip: the file
        // exists, the grammar dispatch yields nothing, so the `#symbol` is not checked — no
        // finding (the behavior Inc 3 supersedes with the `unsupported-language` advisory).
        let src = "#!/usr/bin/perl\nsub thing { }\n";
        assert!(check_one("script.pl", src, "anything").is_empty());
        // A plain text file with no shebang at all, likewise.
        assert!(check_one("notes.txt", "thing lives here\n", "thing").is_empty());
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
}
