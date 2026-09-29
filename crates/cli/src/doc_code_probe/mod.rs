//! The dev pack's `doc-code` probe — bundled in the `jigc` **bin** (never the `cli` lib)
//! and run by self-exec: `main`'s first statement hands `jigc __probe doc-code --build
//! <version>` to [`run`] ([module-layout.md] → Probe boundary, the bundled probe; M54 S4).
//! It binds to the locked JSON wire contract ([validation.md] → The wire contract).
//!
//! It keeps its wire types **independently declared** — it imports neither `engine` nor
//! `cli` — so the any-language wire contract stays proven: it **re-declares** the
//! request / response / snapshot / finding serde shapes as its own structs matching the
//! locked JSON form (`engine::probe`). The engine's `response_findings_deserialize_into_the_one_finding_shape`
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

/// The wire-contract schema version — must match the engine's `result::SCHEMA_VERSION`
/// (3 since M50 Increment 5, which bumped the one integer every result envelope carries).
const SCHEMA_VERSION: u32 = 3;

/// The **anchor grammar**, in the one spelling every surface that names the `code-anchor`
/// type states it (M50 Increment 12 / T4). A shared `const` is refused here and that is
/// deliberate: the probe's wire types stay independent of `engine` (M54 S4), so the
/// token is re-declared and **fenced by a source scan** —
/// `crates/cli/tests/code_anchor_grammar_sites.rs` is total in both directions, so this
/// spelling cannot drift from the pack's without reddening.
const ANCHOR_GRAMMAR: &str = "<repo-relative-path>[#<symbol>]";

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
/// `anchors`, `working_tree_root`, `root_kind`).
#[derive(Debug, Deserialize)]
struct EffectiveStateSnapshot {
    anchors: Vec<TargetAnchor>,
    working_tree_root: PathBuf,
    root_kind: RootKind,
}

/// **Which** tree `working_tree_root` is — the re-declared projection of
/// `engine::probe::RootKind`. The probe cannot tell a materialized index from a working
/// tree by looking at it, so the engine says which it handed over, and every finding names
/// the root it actually read ([validation.md] → The finding must name the root it read, and
/// route to `git add`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum RootKind {
    /// The materialized git index — the tree `finalize` commits (task scope).
    StagedIndex,
    /// The on-disk working tree (the task-less store sweep).
    WorkingTree,
}

impl RootKind {
    /// The root's name, as a finding's message says it.
    fn tree(self) -> &'static str {
        match self {
            RootKind::StagedIndex => "the staged index",
            RootKind::WorkingTree => "the working tree",
        }
    }
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
    /// One blocking `doc-code.<check_id>` finding for an anchor whose **file** is absent
    /// **from the root the engine handed over** — named, never assumed: at task scope that
    /// root is the materialized index (a file on disk but never `git add`ed is absent
    /// there), at store scope the working tree. Addressed at the target the engine
    /// enumerated; `probe` / `check` split from the dotted code.
    fn dangling_file(anchor: &TargetAnchor, root: RootKind, file: &str) -> Self {
        Self::dangling(
            anchor,
            root,
            format!(
                "anchor `{}` resolves to no file (`{file}` is absent from {})",
                anchor.anchor_value,
                root.tree(),
            ),
        )
    }

    /// One blocking `doc-code.<check_id>` finding for a value that **is not an anchor** —
    /// the miss is the grammar, not an absence (RC-m50 F-2).
    ///
    /// `dangling_file` reads the whole literal as a filename, which is right when the literal
    /// *is* one. It is not when the worker typed another tool's address convention — an
    /// editor's `file:line`, a compiler's `file:line:col` — over a path that exists:
    /// *"`src/pad.ts:5` is absent from the staged index"* is then true of a reading nobody meant
    /// and false about the file actually cited, so a worker cannot tell a wrong line number from
    /// an unsupported scheme (surface-contract law 1, the same class as `dangling_symbol`'s
    /// split one branch over).
    ///
    /// The split is made on **evidence the probe already holds**, never a guess at the shape:
    /// the value takes this sentence only when a `:`-cut prefix of it *names a file in the root
    /// the finding names* ([`grammar_miss`]). Where no prefix does, the absence claim is true of
    /// every reading and the shipped sentence stands.
    ///
    /// **The verdict does not move**: a value that is not an anchor still resolves to nothing,
    /// so this is blocking, keyed and located exactly as [`dangling_file`] is
    /// ([`Finding::dangling`] builds both). And it carries **no staging clause**: the cited file
    /// is present in the root read, so `git add` would be a repair for a state the probe did not
    /// observe.
    ///
    /// [`dangling_file`]: Finding::dangling_file
    fn not_an_anchor(anchor: &TargetAnchor, root: RootKind, prefix: &str, suffix: &str) -> Self {
        let mut finding = Self::dangling(
            anchor,
            root,
            format!(
                "anchor `{}` does not match the anchor grammar `{ANCHOR_GRAMMAR}` — `{prefix}` \
                 is a file in {} and the trailing `{suffix}` is not part of an anchor",
                anchor.anchor_value,
                root.tree(),
            ),
        );
        // The route is the edit that makes the value an anchor, followed by both legal shapes.
        // The file-only arm states what it does **not** buy, because a recommendation of the
        // quietest path owes its bound ([validation.md] → The file-only fallback).
        finding.route = Some(format!(
            "drop the trailing `{suffix}` — an anchor is `{prefix}` alone (which buys the file's \
             presence, not that a test exists) or `{prefix}#<symbol>` naming a unit the file \
             declares"
        ));
        finding
    }

    /// One blocking `doc-code.<check_id>` finding for an anchor whose file is present but
    /// whose `#symbol` resolves to no named item (at any nesting) in the root read.
    ///
    /// **The diagnosis states the comparison the probe made** (M46 inc-7; the M47
    /// `title-names-symbol` repair one check over, [validation.md] → The anchor grammar +
    /// resolution). Non-resolution has two causes and they need different sentences:
    ///
    /// - the name is **not in the bytes at all** — the shipped absence sentence is true, and
    ///   the shipped rename/restore repairs are the right ones;
    /// - the name **is in the bytes** but declares nothing — the closure-framework case
    ///   (PHP/Pest, TS/vitest, `node:test` all register a test as a string argument to a call,
    ///   reported by three parties). Here *"`X` is absent from `f`"* is a claim the probe's own
    ///   parsed bytes contradict, and every one of the shipped repairs is a repair of a change
    ///   that never happened — an agent that follows them corrupts a correct citation.
    ///
    /// The **verdict does not move**: `validation.md`'s sanction is that a name living only in
    /// a string or a comment must never false-resolve, so both cells stay blocking, keyed and
    /// located identically ([`Finding::dangling`] builds both). Only the sentence and the
    /// repairs split — on evidence the probe already holds, about the very root it names.
    fn dangling_symbol(
        anchor: &TargetAnchor,
        root: RootKind,
        file: &str,
        symbol: &str,
        text_present: bool,
    ) -> Self {
        if !text_present {
            return Self::dangling(
                anchor,
                root,
                format!(
                    "anchor `{}` resolves to no symbol (`{symbol}` is absent from `{file}` in {})",
                    anchor.anchor_value,
                    root.tree(),
                ),
            );
        }
        let mut finding = Self::dangling(
            anchor,
            root,
            format!(
                "anchor `{}` resolves to no symbol (the text `{symbol}` occurs in `{file}` in \
                 {}, but declares nothing there — a name inside a string, a comment or a \
                 framework registration is not a declared unit)",
                anchor.anchor_value,
                root.tree(),
            ),
        );
        // The route leads with the repair that **works** in this cell — the bare-path degrade
        // `validation.md` declares — and states what it buys, because a recommendation of the
        // quietest path owes its bound: a file-only anchor is checked at the coarser
        // file-existence predicate, so under `criterion-maps-to-test` it never asserts that a
        // test exists. The rename/restore repairs are dropped (the cited name never moved).
        // The staging clause survives **only** on the staged index, and only as a possibility:
        // the probe reads the index and cannot see the working tree, so a declaration by that
        // name written on disk and never `git add`ed is invisible to it.
        let fallback = format!(
            "cite `{file}` alone (drop `#{symbol}`) — a file-only anchor is accepted and buys \
             the file's presence, not that a test exists; or cite a unit the file declares (a \
             function, class or method)"
        );
        finding.route = Some(match root {
            RootKind::StagedIndex => format!(
                "{fallback}; if a declaration by that name is on disk but unstaged, `git add` \
                 it — finalize adjudicates the staged index, not the working tree"
            ),
            RootKind::WorkingTree => fallback,
        });
        finding
    }

    /// One blocking `doc-code.criterion-maps-to-test` finding for an anchor whose `#symbol`
    /// resolves to a real named item that is **not** a `#[test]` fn — the is-a-test
    /// predicate failed (T3).
    fn not_a_test(anchor: &TargetAnchor, root: RootKind, file: &str, symbol: &str) -> Self {
        let mut finding = Self::dangling(
            anchor,
            root,
            format!(
                "anchor `{}` maps to no test (`{symbol}` in `{file}` is not a `#[test]` fn)",
                anchor.anchor_value,
            ),
        );
        // A resolved-but-not-a-test symbol has its own repair: point the criterion at a
        // real test, not the symbol-restore route the floor's dangling cases carry.
        finding.route =
            Some("point the criterion at a real test, or correct the cited symbol".to_string());
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

    /// One **advisory** `doc-code.is-a-test-unverifiable` finding for a `criterion-maps-to-test`
    /// anchor whose `#symbol` **resolved** under a non-Rust grammar — the symbol exists, but the
    /// is-a-test predicate is verifiable only for Rust (`#[test]` has no portable cross-language
    /// signature), so the test half could not be checked (HD3, `ideas/multi-language-doc-code.md`
    /// → is-a-test confidence tiers). Distinct from [`Finding::unsupported_language`]: that one is
    /// the genuine *no-grammar* fork (nothing resolved); this one must **not** claim "no grammar",
    /// because the grammar resolved the symbol — the message + route say so. Advisory,
    /// un-keyed/informational (no inventory row), carrying the informational "no action needed"
    /// route the floor mandates for an uncheckable-by-design outcome.
    fn is_a_test_unverifiable(anchor: &TargetAnchor, file: &str, symbol: &str) -> Self {
        Self {
            severity: Severity::Advisory,
            probe: "doc-code".to_string(),
            check: "is-a-test-unverifiable".to_string(),
            code: "doc-code.is-a-test-unverifiable".to_string(),
            message: format!(
                "anchor `{}` — `{symbol}` resolves in `{file}`, but is-a-test is unverifiable \
                 for this language (only Rust `#[test]` is checkable)",
                anchor.anchor_value,
            ),
            location: Some(Location {
                address: anchor.address.clone(),
                line: 1,
                col: 1,
            }),
            route: Some(
                "no action needed — the symbol resolves; the is-a-test predicate is uncheckable \
                 by design for this language (only Rust `#[test]` is verifiable)"
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
    /// states the ways out. This turns a blocked `finalize` into a *productively* blocked
    /// one (the weaker model can act on it), uniform across the task surface, the blast
    /// radius, and the store sweep (`not_a_test` overrides it with its own repair).
    ///
    /// **The route is root-aware** ([validation.md] → The finding must name the root it
    /// read, and route to `git add`). At task scope the adjudicated root is the **staged
    /// index**, so the dominant dangle is code that is written and *correct* but never `git
    /// add`ed — for which "update the citation … or revert the change" commands **both of
    /// the wrong repairs** and corrupts a correct citation. So the staged-index route leads
    /// with `git add` and keeps the citation/restore repairs behind it; the working-tree
    /// route (the store sweep, which has no index) is unchanged — there, `git add` would
    /// fix nothing.
    fn dangling(anchor: &TargetAnchor, root: RootKind, message: String) -> Self {
        let restore = "update the citation to match the renamed/moved code, or restore the \
                       cited symbol (e.g. revert the change)";
        let route = match root {
            // Present-but-unstaged is not observable from the index tree alone, so the route
            // states the case rather than adjudicating it.
            RootKind::StagedIndex => format!(
                "if the cited code is on disk but unstaged, `git add` it — finalize adjudicates \
                 the staged index, not the working tree; otherwise {restore}"
            ),
            RootKind::WorkingTree => restore.to_string(),
        };
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
            route: Some(route),
        }
    }
}

/// Whether the symbol's **text** occurs in the bytes the probe parsed — the evidence that
/// splits *"the name is not there"* from *"the name is there and declares nothing"*
/// ([`Finding::dangling_symbol`]). It is a read of bytes already in hand: no git, no build, no
/// wall-clock, so the probe stays inside its determinism contract and the resulting sentence
/// is a fact about the root the finding names.
///
/// **The reading is a raw substring, deliberately.** Under a word-boundary reading a name that
/// occurs only inside a longer identifier (`burst` inside `burst_rejected`) would take the
/// *absence* sentence, which the bytes contradict — the exact lie this split exists to end.
/// Raw `contains` keeps every sentence true: "the text occurs, and declares nothing" holds for
/// a substring occurrence too, and the route it carries (cite the file alone, or cite a unit
/// the file declares) is the right repair there as well.
fn symbol_text_present(src: &str, symbol: &str) -> bool {
    src.contains(symbol)
}

/// Whether the value's file portion is another convention's address over a path that exists
/// — the evidence that splits *"this file is absent"* from *"this is not an anchor"*
/// ([`Finding::not_an_anchor`]). Returns the `(prefix, suffix)` the sentence names.
///
/// The reading is: the **first `:`-cut prefix that names a file** in the root the engine handed
/// over. First-that-is-a-file rather than first-colon, because a path may legitimately carry a
/// `:` — a file called `a:b.ts` cited as `a:b.ts:5` must be named whole. And a *file*, not any
/// entry, because the sentence says "is a file in <root>": a directory prefix would make it
/// false, so that cell keeps the absence sentence instead.
///
/// Like every other split in this probe it reads only what it already has — the root's own
/// directory entries — so no git, build, network or wall-clock enters, and the sentence it
/// produces is a fact about the root the finding names.
fn grammar_miss<'a>(root: &std::path::Path, file: &'a str) -> Option<(&'a str, &'a str)> {
    file.match_indices(':')
        .map(|(at, _)| file.split_at(at))
        .find(|(prefix, _)| root.join(prefix).is_file())
}

/// Split an anchor value into its file portion (before the first `#`) and an optional
/// `#symbol` (a bare path has no `#`, so the symbol is `None`).
fn split_anchor(anchor_value: &str) -> (&str, Option<&str>) {
    match anchor_value.split_once('#') {
        Some((file, symbol)) => (file, Some(symbol)),
        None => (anchor_value, None),
    }
}

/// Resolve every anchor against the code tree the engine handed over (`working_tree_root`,
/// of kind `root_kind`): the file before any `#` must exist; a `#symbol` on a present `.rs`
/// file must additionally resolve to a named item (at any nesting) via tree-sitter. Each
/// unresolvable anchor emits one blocking finding **naming the root it read**.
fn check_anchors(snapshot: &EffectiveStateSnapshot) -> Vec<Finding> {
    let root = snapshot.root_kind;
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
            if path
                .symlink_metadata()
                .is_ok_and(|m| m.file_type().is_symlink())
            {
                return Some(Finding::symlink_anchor(anchor, file));
            }
            if !path.exists() {
                // The literal names nothing in the root read — but *which* miss it is depends
                // on whether a `:`-cut prefix of it does: another tool's `file:line` over a
                // real path is a grammar miss, and reporting it as an absence is true of a
                // reading nobody meant (RC-m50 F-2).
                return Some(match grammar_miss(&snapshot.working_tree_root, file) {
                    Some((prefix, suffix)) => Finding::not_an_anchor(anchor, root, prefix, suffix),
                    None => Finding::dangling_file(anchor, root, file),
                });
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
            // Vue is the one grammar whose resolution is not a plain `symbol_exists` over the
            // raw source: a `.vue` file is an HTML-ish SFC envelope, so its symbols resolve via
            // a union of the extracted `<script>` symbols and a **filename-component unit** (the
            // idiomatic `AppLayout.vue` ⇒ `AppLayout` component name). Every other grammar
            // resolves the raw source directly.
            let resolved = if grammar == resolve::Grammar::Vue {
                let file_stem = std::path::Path::new(file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                resolve::symbol_exists_vue(&src, symbol, file_stem)
            } else {
                resolve::symbol_exists(&src, symbol, grammar)
            };
            if !resolved {
                // The symbol resolves to no declared unit — the floor of every `#symbol` check,
                // including `criterion-maps-to-test` (whose predicate is symbol existence +
                // is-a-test). The diagnosis splits on whether the name is in the bytes the probe
                // just parsed; the verdict does not.
                return Some(Finding::dangling_symbol(
                    anchor,
                    root,
                    file,
                    symbol,
                    symbol_text_present(&src, symbol),
                ));
            }
            // The symbol resolves. `criterion-maps-to-test` additionally requires the
            // is-a-test predicate (a `#[test]` fn); `symbol-exists` is satisfied here. The
            // predicate is **Rust-only** (`#[test]` has no portable cross-language
            // signature). On a Rust file the predicate runs; on a **non-Rust** file the
            // is-a-test half is unverifiable, so a resolved `criterion-maps-to-test` symbol
            // emits one `is-a-test-unverifiable` advisory (the symbol passed, the test-half
            // could not be checked — fork F4 truth table), never a wrong `not_a_test` block.
            // This is a **distinct** advisory from `unsupported-language`: the grammar resolved
            // the symbol, so claiming "no grammar" here is a law-1 lie (fork 9); the advisory
            // names the real reason instead. `symbol-exists` is unaffected — its predicate is
            // symbol existence, already met.
            if anchor.check_id == "criterion-maps-to-test" {
                if grammar == resolve::Grammar::Rust {
                    if !resolve::test_fn_exists_in_rust(&src, symbol) {
                        return Some(Finding::not_a_test(anchor, root, file, symbol));
                    }
                } else {
                    return Some(Finding::is_a_test_unverifiable(anchor, file, symbol));
                }
            }
            None
        })
        .collect()
}

/// The probe's entry: read the request from stdin, the snapshot it names, and print the
/// response on stdout. `jigc`'s `main` calls it for the self-exec argv.
pub fn run() -> ExitCode {
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
            root_kind: RootKind::WorkingTree,
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
            root_kind: RootKind::WorkingTree,
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
            root_kind: RootKind::WorkingTree,
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
            root_kind: RootKind::WorkingTree,
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
            root_kind: RootKind::WorkingTree,
        };
        assert_eq!(check_anchors(&snapshot).len(), 1);
    }

    #[test]
    fn a_dangling_finding_names_the_root_it_read_and_routes_by_root_kind() {
        // The same absent file, adjudicated over the two roots the engine hands over
        // ([validation.md] → The finding must name the root it read, and route to `git add`).
        // Under the **staged index** (task scope) the dominant cause is code on disk but
        // never `git add`ed, so the message names the index and the route leads with `git
        // add`; under the **working tree** (the task-less store sweep) `git add` would fix
        // nothing, so the message names the working tree and the citation/restore route
        // stands. Neither generation may claim the other's root.
        let dangle = |root_kind: RootKind| {
            let root = temp_root();
            let snapshot = EffectiveStateSnapshot {
                anchors: vec![anchor("src/feature.rs#feature")],
                working_tree_root: root,
                root_kind,
            };
            let mut findings = check_anchors(&snapshot);
            assert_eq!(findings.len(), 1, "exactly one finding, got {findings:?}");
            findings.remove(0)
        };

        let index = dangle(RootKind::StagedIndex);
        assert!(
            index.message.contains("is absent from the staged index")
                && !index.message.contains("working tree"),
            "the task-scope finding names the index it read, never the working tree it did \
             not: {}",
            index.message,
        );
        assert!(
            index
                .route
                .as_deref()
                .is_some_and(|r| r.contains("`git add`")),
            "the staged-index route names `git add` — the repair for a present-but-unstaged \
             file: {:?}",
            index.route,
        );

        let tree = dangle(RootKind::WorkingTree);
        assert!(
            tree.message.contains("is absent from the working tree")
                && !tree.message.contains("staged index"),
            "the store-scope finding names the working tree it read: {}",
            tree.message,
        );
        assert!(
            tree.route.as_deref().is_some_and(|r| {
                r.starts_with("update the citation to match the renamed/moved code")
                    && !r.contains("git add")
            }),
            "the working-tree route keeps the citation/restore repair — `git add` fixes nothing \
             there: {:?}",
            tree.route,
        );
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
    fn non_rust_maps_to_test_present_symbol_names_is_a_test_unverifiable_not_no_grammar() {
        // fork 9 / law-1: on a `criterion-maps-to-test` anchor whose `#symbol` RESOLVES under a
        // non-Rust grammar, is-a-test is unverifiable — but the grammar EXISTS and resolved the
        // symbol, so the advisory must NOT lie "no grammar for <file>"; it must name the real
        // reason (is-a-test is Rust-only) and state that the symbol resolved. The message here is
        // the distinct is-a-test-unverifiable advisory, never the `unsupported-language`
        // no-grammar text (which is the genuine :366 fork, untouched).
        let findings = check_one_with_check(
            "limit.test.ts",
            TS_TEST,
            "rateLimitTest",
            "criterion-maps-to-test",
        );
        assert_eq!(findings.len(), 1, "exactly one finding, got {findings:?}");
        assert_eq!(findings[0].severity, Severity::Advisory);
        let msg = &findings[0].message;
        assert!(
            !msg.contains("no grammar"),
            "the advisory must not claim no grammar — the grammar resolved the symbol: {msg}"
        );
        assert!(
            msg.contains("is-a-test") && msg.contains("resolves"),
            "the advisory must name the is-a-test-unverifiable reason and that the symbol \
             resolved: {msg}"
        );
        assert!(
            !findings[0]
                .route
                .as_deref()
                .unwrap_or_default()
                .contains("no shipped grammar"),
            "the route must not claim no shipped grammar either: {:?}",
            findings[0].route
        );
    }

    #[test]
    fn non_rust_maps_to_test_present_symbol_advises_never_blocks() {
        // symbol PRESENT under a non-Rust grammar: exactly one `is-a-test-unverifiable` advisory
        // (the is-a-test half is Rust-only — the symbol resolved, so NOT the `unsupported-language`
        // no-grammar fork), NO `criterion-maps-to-test` block.
        for (file, src, symbol) in [
            ("limit.test.ts", TS_TEST, "rateLimitTest"),
            ("test_limit.py", PY_TEST, "test_rate_limit"),
        ] {
            let findings = check_one_with_check(file, src, symbol, "criterion-maps-to-test");
            assert_eq!(findings.len(), 1, "{file}: exactly one finding");
            assert_eq!(findings[0].severity, Severity::Advisory, "{file}: advisory");
            assert_eq!(findings[0].check, "is-a-test-unverifiable", "{file}");
            assert_eq!(
                findings[0].code, "doc-code.is-a-test-unverifiable",
                "{file}"
            );
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

    // A real Vue single-file component: an idiomatic `<script setup>` composable, a plain
    // `<script>` with an `export default { name: ... }`, and template/style/SFC-macro names
    // that must NOT resolve as script symbols.
    const APP_LAYOUT_VUE: &str = "\
<template>
  <div class=\"layout-class\">{{ templateOnlyName }}</div>
</template>

<script>
export default {
  name: 'AppLayout',
};

function plainScriptFn() {
  return 1;
}
</script>

<script setup>
defineProps(['title']);

function useCounter() {
  return 0;
}

const setupConst = 1;
</script>

<style scoped>
.layout-class {
  color: red;
}
</style>
";

    #[test]
    fn vue_fabricated_symbol_blocks_not_advises() {
        // The flagship red→green: a fabricated Vue symbol must BLOCK ("cannot claim code that
        // doesn't exist"), not finalize clean as an `unsupported-language` advisory (the pre-M41
        // behavior — `.vue` mapped to no grammar).
        let findings = check_one("AppLayout.vue", APP_LAYOUT_VUE, "DoesNotExist");
        assert_eq!(findings.len(), 1, "exactly one finding, got {findings:?}");
        assert_eq!(findings[0].severity, Severity::Blocking);
        assert_eq!(findings[0].check, "symbol-exists");
        assert_eq!(findings[0].code, "doc-code.symbol-exists");
    }

    #[test]
    fn vue_filename_component_and_script_symbols_resolve() {
        // AppLayout.vue#AppLayout resolves via the filename-component addressable unit (the
        // component name lives only as a string in `name:`, never as a citable script symbol);
        // #useCounter resolves as a `<script setup>` composable; #plainScriptFn resolves from the
        // plain `<script>` block (both blocks extracted).
        for symbol in ["AppLayout", "useCounter", "plainScriptFn"] {
            assert!(
                check_one("AppLayout.vue", APP_LAYOUT_VUE, symbol).is_empty(),
                "expected `{symbol}` to resolve (no finding)"
            );
        }
    }

    #[test]
    fn vue_template_style_macro_names_block() {
        // A name living only in `<template>`, a scoped-`<style>` class, an SFC compiler macro
        // (`defineProps`), or a macro argument (`title`) is NOT an addressable unit — each blocks.
        for symbol in ["templateOnlyName", "layout-class", "defineProps", "title"] {
            let findings = check_one("AppLayout.vue", APP_LAYOUT_VUE, symbol);
            assert_eq!(findings.len(), 1, "{symbol}: expected one blocking finding");
            assert_eq!(findings[0].severity, Severity::Blocking, "{symbol}");
            assert_eq!(findings[0].check, "symbol-exists", "{symbol}");
        }
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

    // ----- M46 inc-7 / T1: the non-resolution diagnosis, over its whole axis -----

    /// The **`RootKind` axis, read off the enum** — every root the engine can hand the probe.
    /// [`root_kind_axis_is_exhaustive`] is the fence: a third variant makes that test fail to
    /// compile until it is named, and naming it is what puts it in this array.
    const ROOT_KINDS: [RootKind; 2] = [RootKind::StagedIndex, RootKind::WorkingTree];

    /// The **check-id axis, hand-enumerated — with the reason it must be.** A check id is
    /// *opaque wire data the schema supplies* (`field.check ?? field_type.check`, resolved
    /// engine-side and handed over as a string); the probe holds no code-side registry of check
    /// ids, so there is nothing to derive from and dressing two literals as a derived set would
    /// be a fence that fences nothing. They are listed, and the reason is stated rather than
    /// implied. Both must be covered because the emitting seam is **check-id-blind**: a
    /// `symbol-exists` anchor over the same file and symbol emits byte-identical bytes.
    const CHECK_IDS: [&str; 2] = ["symbol-exists", "criterion-maps-to-test"];

    #[test]
    fn root_kind_axis_is_exhaustive() {
        // The fence for [`ROOT_KINDS`]: this `match` is exhaustive over the enum, so a new
        // `RootKind` variant reddens here until it is named — and the cells below iterate the
        // array, so naming it is what makes the axis cover it.
        for kind in ROOT_KINDS {
            match kind {
                RootKind::StagedIndex | RootKind::WorkingTree => {}
            }
        }
    }

    /// A vitest file whose only test is registered by a **closure** — the shape three
    /// frameworks share (PHP/Pest, TS/vitest, `node:test`): the test's name lives in a string
    /// argument, so the name is present in the file's text while declaring no unit.
    const CLOSURE_TEST_TS: &str = "\
import { it, expect } from \"vitest\";

it('rejects a burst beyond the cap', () => {
    expect(true).toBe(true);
});
";

    /// The closure fixture's file name, as every cell cites it.
    const CLOSURE_FILE: &str = "rate_limit.test.ts";

    /// Drive one axis cell — the closure fixture, one `<file>#<symbol>` anchor, an explicit
    /// `check_id` and an explicit `RootKind` — and return its single finding.
    fn check_cell(symbol: &str, check_id: &str, root_kind: RootKind) -> Finding {
        let root = temp_root();
        std::fs::write(root.join(CLOSURE_FILE), CLOSURE_TEST_TS).unwrap();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor_with_check(
                &format!("{CLOSURE_FILE}#{symbol}"),
                check_id,
            )],
            working_tree_root: root,
            root_kind,
        };
        let mut findings = check_anchors(&snapshot);
        assert_eq!(findings.len(), 1, "exactly one finding, got {findings:?}");
        findings.remove(0)
    }

    /// The verdict half, asserted **identically in every cell**: the split is a diagnosis
    /// change, never a verdict change ([validation.md] → the anchor grammar's sanction that a
    /// name living only in a string or comment must not resolve keeps this blocking).
    fn assert_verdict_unmoved(finding: &Finding, check_id: &str) {
        assert_eq!(finding.severity, Severity::Blocking, "severity is unmoved");
        assert_eq!(finding.probe, "doc-code");
        assert_eq!(finding.check, check_id, "the check id is unmoved");
        assert_eq!(finding.code, format!("doc-code.{check_id}"), "code unmoved");
        let location = finding.location.as_ref().expect("a located finding");
        assert_eq!(location.address, "specs/s.md#criteria/c");
        assert_eq!((location.line, location.col), (1, 1));
    }

    #[test]
    fn non_resolving_symbol_states_the_comparison_over_check_id_root_and_text_presence() {
        // The axis: check-id × RootKind × {text-present, text-absent}. The text-present cells
        // are the repair (today they assert an absence the parsed bytes contradict); the
        // text-absent cells are the shipped sentence and repairs, which must not move.
        for check_id in CHECK_IDS {
            for root in ROOT_KINDS {
                let read = root.tree();
                let unread = match root {
                    RootKind::StagedIndex => RootKind::WorkingTree.tree(),
                    RootKind::WorkingTree => RootKind::StagedIndex.tree(),
                };
                let cell = format!("[{check_id} × {read}]");

                // --- text-ABSENT: the name is in neither the AST nor the bytes.
                let absent = check_cell("totally_absent_name", check_id, root);
                assert_verdict_unmoved(&absent, check_id);
                assert!(
                    absent
                        .message
                        .contains("`totally_absent_name` is absent from"),
                    "{cell} a genuinely absent name keeps the shipped absence sentence: {}",
                    absent.message,
                );
                assert!(
                    absent.message.contains(read) && !absent.message.contains(unread),
                    "{cell} the finding names the root it read, never the other: {}",
                    absent.message,
                );
                let absent_route = absent.route.as_deref().unwrap_or_default();
                assert!(
                    absent_route.contains("update the citation to match the renamed/moved code"),
                    "{cell} the shipped repairs stand where the code really did move: \
                     {absent_route}",
                );
                assert_eq!(
                    absent_route.contains("`git add`"),
                    root == RootKind::StagedIndex,
                    "{cell} the staging clause rides the staged index only: {absent_route}",
                );

                // --- text-PRESENT: the name is in the bytes the probe parsed, as a closure
                // registration — so the finding may not claim it is absent from that root.
                let present = check_cell("rejects a burst beyond the cap", check_id, root);
                assert_verdict_unmoved(&present, check_id);
                assert!(
                    !present.message.contains("is absent from"),
                    "{cell} the probe read the text in the very root it names — it must not \
                     assert an absence it did not observe: {}",
                    present.message,
                );
                assert!(
                    present.message.contains("`rejects a burst beyond the cap`")
                        && present.message.contains(&format!("`{CLOSURE_FILE}`")),
                    "{cell} the message names both sides of the comparison: {}",
                    present.message,
                );
                assert!(
                    present.message.contains(read) && !present.message.contains(unread),
                    "{cell} the finding names the root it read, never the other: {}",
                    present.message,
                );
                let present_route = present.route.as_deref().unwrap_or_default();
                assert!(
                    present_route.starts_with(&format!("cite `{CLOSURE_FILE}` alone")),
                    "{cell} the route leads with the file-only fallback — the repair that \
                     works here: {present_route}",
                );
                assert!(
                    present_route.contains("the file's presence, not that a test exists"),
                    "{cell} the recommended fallback states what it does NOT buy: \
                     {present_route}",
                );
                assert!(
                    !present_route.contains("update the citation to match the renamed/moved code")
                        && !present_route.contains("restore the cited symbol"),
                    "{cell} the two rename/restore repairs are inapplicable in this cell — the \
                     cited name never moved: {present_route}",
                );
                assert_eq!(
                    present_route.contains("`git add`"),
                    root == RootKind::StagedIndex,
                    "{cell} the staging clause survives only where the probe's evidence leaves \
                     it possible — it cannot see the working tree: {present_route}",
                );
            }
        }
    }

    // ----- M50 inc-12 / T5: the `file:line` miss names the grammar (RC-m50 F-2) -----

    /// The trial's fixture, rebuilt: a TypeScript module declaring exactly one unit. `#pad`
    /// resolves against it; `:5` is another convention's `file:line`; `#pad.method` names a
    /// unit it does not declare.
    const PAD_TS: &str = "\
export function pad(n: number): string {
    return String(n).padStart(2, \"0\");
}
";

    /// Run one cell over a root holding `src/pad.ts` (and the `src/` directory that contains
    /// it, which is what makes the directory-prefix negative cell reachable).
    fn check_pad(anchor_value: &str, check_id: &str, root_kind: RootKind) -> Vec<Finding> {
        let root = temp_root();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src").join("pad.ts"), PAD_TS).unwrap();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor_with_check(anchor_value, check_id)],
            working_tree_root: root,
            root_kind,
        };
        check_anchors(&snapshot)
    }

    /// Exactly one finding, returned.
    fn only(findings: Vec<Finding>) -> Finding {
        let mut findings = findings;
        assert_eq!(findings.len(), 1, "exactly one finding, got {findings:?}");
        findings.remove(0)
    }

    /// The **suffix axis** — the non-anchor suffixes a worker actually types after a real
    /// path, hand-enumerated *with the reason it must be*: these are other tools' address
    /// conventions (an editor's `file:line`, a compiler's `file:line:col`), which live in no
    /// registry this probe or the engine holds — there is nothing to derive from. What makes
    /// the set total is the **predicate**, not the list: any suffix at all is caught, because
    /// the split is on whether a `:`-cut prefix names a file in the root read. The third
    /// member carries a trailing `#symbol` too, the cell where a grammar miss and a symbol
    /// citation arrive together.
    const NON_ANCHOR_SUFFIXES: [&str; 3] = [":5", ":5:12", ":5#pad"];

    #[test]
    fn a_file_line_value_names_the_grammar_over_check_id_root_and_suffix() {
        // The axis: suffix × check-id × RootKind. The producer is check-id-blind and
        // root-aware, so every cell is driven rather than one standing in for the rest.
        for suffix in NON_ANCHOR_SUFFIXES {
            for check_id in CHECK_IDS {
                for root in ROOT_KINDS {
                    let read = root.tree();
                    let unread = match root {
                        RootKind::StagedIndex => RootKind::WorkingTree.tree(),
                        RootKind::WorkingTree => RootKind::StagedIndex.tree(),
                    };
                    let value = format!("src/pad.ts{suffix}");
                    // The suffix the sentence names is the non-anchor tail of the FILE
                    // portion — everything after the path, up to any `#symbol`.
                    let tail = suffix.split('#').next().unwrap();
                    let cell = format!("[{value} × {check_id} × {read}]");
                    let finding = only(check_pad(&value, check_id, root));

                    // The verdict does not move: a value that is not an anchor still blocks,
                    // under the same check id, code and location.
                    assert_verdict_unmoved(&finding, check_id);

                    assert!(
                        !finding.message.contains("is absent from"),
                        "{cell} `src/pad.ts` is a file in the root the probe read — the miss \
                         must not be reported as an absence: {}",
                        finding.message,
                    );
                    assert!(
                        finding.message.contains(ANCHOR_GRAMMAR),
                        "{cell} the message must name the grammar the value failed to match: \
                         {}",
                        finding.message,
                    );
                    assert!(
                        finding.message.contains("`src/pad.ts`")
                            && finding.message.contains(&format!("`{tail}`")),
                        "{cell} the message names both sides of the comparison — the path that \
                         resolved and the suffix that did not: {}",
                        finding.message,
                    );
                    assert!(
                        finding.message.contains(read) && !finding.message.contains(unread),
                        "{cell} the finding names the root it read, never the other: {}",
                        finding.message,
                    );

                    let route = finding.route.as_deref().unwrap_or_default();
                    assert!(
                        route.starts_with(&format!("drop the trailing `{tail}`")),
                        "{cell} the route leads with the edit that makes the value an anchor: \
                         {route}",
                    );
                    assert!(
                        route.contains("`src/pad.ts#<symbol>`"),
                        "{cell} …and offers the symbol form, which is what the worker wanted: \
                         {route}",
                    );
                    assert!(
                        route.contains("not that a test exists"),
                        "{cell} the file-only arm it recommends states what it does not buy: \
                         {route}",
                    );
                    assert!(
                        !route.contains("`git add`"),
                        "{cell} nothing is unstaged here — the cited file is present in the \
                         root read, so the staging clause would be a repair for a state the \
                         probe did not observe: {route}",
                    );
                }
            }
        }
    }

    #[test]
    fn the_prefix_named_is_the_first_one_that_is_a_file_not_the_first_colon() {
        // A path may legitimately carry a `:`. The split is evidence — the first `:`-cut
        // prefix that NAMES A FILE in the root read — not a guess at the first separator, so
        // a file called `a:b.ts` is named whole.
        let root = temp_root();
        std::fs::write(root.join("a:b.ts"), PAD_TS).unwrap();
        let snapshot = EffectiveStateSnapshot {
            anchors: vec![anchor("a:b.ts:5")],
            working_tree_root: root,
            root_kind: RootKind::WorkingTree,
        };
        let finding = only(check_anchors(&snapshot));
        assert!(
            finding.message.contains("`a:b.ts` is a file"),
            "the prefix that resolved is the one named: {}",
            finding.message,
        );
        assert!(
            finding
                .route
                .as_deref()
                .unwrap_or_default()
                .contains("drop the trailing `:5`"),
            "…and the suffix dropped is what follows it: {:?}",
            finding.route,
        );
    }

    #[test]
    fn a_value_whose_prefix_names_nothing_keeps_the_shipped_absence_sentence() {
        // The negative half of the split, over the same axis the positive one runs. Where no
        // `:`-cut prefix names a file, the absence claim is TRUE of every reading of the
        // value, so the shipped sentence and its shipped repairs stand — and the grammar is
        // not named, because a grammar miss is not what the probe observed.
        for (value, why) in [
            ("src/ghost.ts:5", "no prefix of it names a file"),
            ("src/ghost.ts", "the shipped bare-path miss, untouched"),
            (
                "src:5",
                "`src` is a directory, not a file — the sentence would be false",
            ),
        ] {
            for check_id in CHECK_IDS {
                for root in ROOT_KINDS {
                    let finding = only(check_pad(value, check_id, root));
                    assert_verdict_unmoved(&finding, check_id);
                    assert!(
                        finding.message.contains(&format!(
                            "anchor `{value}` resolves to no file (`{value}` is absent from {})",
                            root.tree()
                        )),
                        "[{value}] {why}: the shipped absence sentence stands: {}",
                        finding.message,
                    );
                    assert!(
                        !finding.message.contains(ANCHOR_GRAMMAR),
                        "[{value}] {why}: the grammar is named only where a grammar miss was \
                         observed: {}",
                        finding.message,
                    );
                }
            }
        }
    }

    #[test]
    fn the_symbol_side_of_the_grammar_is_untouched() {
        // The sibling producer keeps its shipped wording: `#pad` resolves clean, and a
        // `#symbol` the file does not declare takes `dangling_symbol`'s sentence — which
        // already states the comparison it made (M46 inc-7) and must not gain grammar text,
        // because the value it was handed DOES parse as an anchor.
        for check_id in CHECK_IDS {
            let clean = check_pad("src/pad.ts#pad", "symbol-exists", RootKind::WorkingTree);
            assert!(clean.is_empty(), "`#pad` resolves: {clean:?}");
            let _ = check_id;
        }
        let finding = only(check_pad(
            "src/pad.ts#pad.method",
            "symbol-exists",
            RootKind::StagedIndex,
        ));
        assert_eq!(
            finding.message,
            "anchor `src/pad.ts#pad.method` resolves to no symbol (`pad.method` is absent from \
             `src/pad.ts` in the staged index)",
            "the symbol-side sentence is unmoved, byte-for-byte",
        );
        assert!(
            !finding.message.contains(ANCHOR_GRAMMAR),
            "a value that parses as an anchor is not a grammar miss: {}",
            finding.message,
        );
    }
}
