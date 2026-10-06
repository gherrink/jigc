//! The **temp-mint fence** — test code takes its throwaway path names from one seam.
//!
//! ## The class this fences
//!
//! A test that mints a scratch directory needs a name no *sibling test* can produce.
//! The idiom the workspace reached for, ~268 times, was `pid + SystemTime::now()`:
//! `pid` separates processes, and the clock was assumed to separate the threads inside
//! one. **It does not.** `SystemTime::now()` reports nanosecond units but not
//! nanosecond resolution — macOS truncates it to microseconds (measured on a build
//! host: 1000 tight calls returned **88** distinct values), so two `#[test]` threads
//! reaching the mint in the same microsecond took the *same* path.
//!
//! The collision is silent where it happens (`create_dir_all` over an existing
//! directory succeeds) and surfaces as whatever the shared directory breaks first —
//! `git init` dying on `fatal: cannot copy … File exists`, or one owner's `Drop`
//! deleting the other's tree mid-test (`NotFound`). Under the documented gate
//! (`cargo test`, default threads) that aborted the run at the first failing binary,
//! so roughly 200 later binaries never executed at all: a regression anywhere past it
//! was invisible.
//!
//! ## Why a fence rather than a swept list
//!
//! The sweep that fixed the 268 sites is a *finder*; it cannot stop the 269th
//! ([dev-workflow.md](../../../implementation/dev-workflow.md) → *a grep is not a
//! fence*). A new suite copies its preamble from an existing one, and the idiom would
//! return in whichever binary happened to be written next. So the property is checked
//! where membership is decided: **any test-domain call that mints a path under the OS
//! temp dir must take its disambiguator from [`engine::tempname::unique_nanos`]**, the
//! one place that guarantees per-process uniqueness regardless of clock resolution.
//!
//! ## The domain, and why it is structural rather than a list
//!
//! *Test domain* = every file under a crate's `tests/` tree, plus each `#[cfg(test)]`
//! module of every `src/` file — located by brace-matching the module body, because
//! this workspace interleaves them with production code rather than putting them last
//! (`engine/src/validate.rs` opens one at line 969 and resumes production code after
//! it). Production `src/` code is **out of scope by construction, not by exemption**:
//! its two temp-path mints (`state::temp_sibling`, `validate::store_scratch_path`)
//! already carry their own sequence counters, having learned this lesson on the write
//! path at M45.
//!
//! Both the region search and the mint search run over the file with **comments and
//! string literals blanked out**, so a `temp_dir()` written in prose or quoted inside
//! an assertion message is not an offender, and a `{` inside a string cannot throw the
//! brace matching off. That reading — the blanking and the region search — lives in
//! [`support::rust_source`](crate::support::rust_source), shared with the schema
//! resolution fence (M49 Increment 3): the scanner is subtle enough that a second copy
//! would be a second set of bugs.
//!
//! The bundled `doc-code` probe is likewise outside by construction: its module
//! (`crates/cli/src/doc_code_probe/`, in the `jigc` bin) keeps its wire types independent
//! and **cannot depend on `engine`** (M54 S4), so it carries a local `AtomicU32` instead.

use crate::support::rust_source::{cfg_test_regions, code_only, is_test_domain};
use std::path::{Path, PathBuf};

/// The workspace root — two levels up from `crates/cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize the workspace root")
}

/// Every `.rs` file under the workspace's two member crates and under each suite home
/// outside them, recursively.
///
/// Membership is the crate tree itself, so a file added tomorrow is swept without
/// anyone remembering to list it — and the homes outside the crates are read off the
/// manifest's `[[test]]` paths ([`crate::support::test_homes`]), so the tooling suites
/// that left `crates/` on 2026-10-06 did not leave this fence.
fn workspace_sources(root: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = ["crates/engine", "crates/cli"]
        .iter()
        .map(|crate_dir| root.join(crate_dir))
        .chain(crate::support::test_homes::homes_outside_the_crates())
        .flat_map(|dir| crate::support::rust_source::rust_files(&dir))
        .collect();
    // The bundled `doc-code` probe cannot depend on `engine` — its wire types stay
    // independent of `engine` and `cli` (M54 S4) — so it cannot reach the mint.
    out.retain(|p| !p.components().any(|c| c.as_os_str() == "doc_code_probe"));
    out.sort();
    assert!(
        out.len() > 100,
        "the sweep must find the workspace sources; found only {}",
        out.len(),
    );
    out
}

/// The mint the fence requires every test-domain temp path to be named from.
const SEAM: &str = "unique_nanos";

/// A call that puts a name under the OS temp dir.
const MINT: &str = "temp_dir()";

/// Whether a `temp_dir()` occurrence actually *mints a name*, as opposed to merely
/// naming the directory.
///
/// Only two forms build a path here: `std::env::temp_dir().join(…)`, and
/// `let mut path = std::env::temp_dir();` followed by a `push`. Anything else — most
/// often `assert!(p.starts_with(std::env::temp_dir()))` — reads the temp dir without
/// creating anything in it, so it has no disambiguator to get wrong.
fn is_a_mint(code: &str, after: usize) -> bool {
    let rest = code[after..].trim_start();
    rest.starts_with(';') || rest.starts_with(".join(")
}

/// Every test-domain temp-path mint names itself from the shared seam.
///
/// The check is per **call site**, not per file: a file may hold both a compliant and
/// a stale mint, and a file-level test would pass on the compliant one. Each
/// `temp_dir()` occurrence is read together with the name expression that follows it,
/// and that window must reach the seam.
#[test]
fn every_test_domain_temp_mint_takes_the_shared_seam() {
    /// How far past `temp_dir()` the name expression can run. The longest compliant
    /// mint in the workspace is a multi-line `format!` of ~200 bytes.
    const WINDOW: usize = 400;

    /// The sweep must keep *finding* mints, or a discriminator bug would let it pass by
    /// inspecting nothing. The workspace carries ~270; this floor is well under that and
    /// only trips if the search itself has broken.
    const MINTS_FLOOR: usize = 200;

    let root = workspace_root();
    let fence = Path::new(file!())
        .file_name()
        .expect("this file has a name");
    let mut offenders = Vec::new();
    let mut inspected = 0usize;

    for path in workspace_sources(&root) {
        // This suite quotes the banned idiom on purpose, to prove the fence bites.
        if path.file_name() == Some(fence) {
            continue;
        }
        let body = std::fs::read_to_string(&path).expect("read a workspace source");
        let code = code_only(&body);
        let regions = cfg_test_regions(&code);

        for (at, _) in code.match_indices(MINT) {
            if !is_test_domain(&path, &regions, at) || !is_a_mint(&code, at + MINT.len()) {
                continue;
            }
            inspected += 1;
            let window = &code[at..code.len().min(at + WINDOW)];
            if window.contains(SEAM) {
                continue;
            }
            let rel = path.strip_prefix(&root).unwrap_or(&path).display();
            let line = code[..at].lines().count();
            let snippet: String = body[at..body.len().min(at + 90)]
                .lines()
                .take(2)
                .collect::<Vec<_>>()
                .join(" ");
            offenders.push(format!("  {rel}:{line}: {snippet}"));
        }
    }

    assert!(
        inspected >= MINTS_FLOOR,
        "the fence must actually inspect the workspace's temp mints; it saw only \
         {inspected} (floor {MINTS_FLOOR}) — the mint discriminator has drifted",
    );
    assert!(
        offenders.is_empty(),
        "every test-domain `{MINT}` mint must take its disambiguator from \
         `engine::tempname::{SEAM}` — a `pid + SystemTime::now()` name is not unique \
         within a process, because the OS clock is coarser than the call rate, and two \
         parallel `#[test]`s then build into one directory.\n\
         {} offending mint(s):\n{}",
        offenders.len(),
        offenders.join("\n"),
    );
}

/// The domain detector really discriminates — calibrated on the hardest real file.
///
/// `engine/src/validate.rs` is the shape that breaks the naive "first `#[cfg(test)]`
/// to EOF" rule: it opens an inline test module partway down and then resumes
/// production code, including the production mint `store_scratch_path`. If the
/// detector ever swallowed the rest of the file, the fence would demand the seam of
/// production code; if it found no regions at all, the fence would go silently
/// vacuous over every `src/` unit test. Both directions are asserted here.
#[test]
fn the_domain_detector_separates_production_from_inline_test_modules() {
    let path = workspace_root().join("crates/engine/src/validate.rs");
    let body = std::fs::read_to_string(&path).expect("read validate.rs");
    let code = code_only(&body);
    let regions = cfg_test_regions(&code);

    assert!(
        regions.len() >= 2,
        "validate.rs carries several inline `#[cfg(test)]` modules; found {}",
        regions.len(),
    );

    let production_mint = code
        .find("fn store_scratch_path")
        .expect("validate.rs must still carry the production scratch-path mint");
    assert!(
        !is_test_domain(&path, &regions, production_mint),
        "the production `store_scratch_path` must sit outside every test region",
    );

    let inline_test = code
        .find("mod provenance_tests")
        .expect("validate.rs must still carry its inline provenance test module");
    assert!(
        is_test_domain(&path, &regions, inline_test),
        "an inline `#[cfg(test)]` module must be inside the test domain",
    );
    assert!(
        inline_test < production_mint,
        "the calibration only bites while the inline module precedes production code",
    );
}

/// The fence is not vacuous: it really would catch the idiom it bans.
///
/// A fence that silently swept zero files, or whose window never looked at the name
/// expression, would pass forever. So the detector is run against the exact text the
/// class produces and must reject it — the lever flipped, rather than assumed.
#[test]
fn the_fence_rejects_the_banned_idiom() {
    let banned = r#"
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-example-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        path.push(unique);
"#;
    let at = banned.find(MINT).expect("the sample must mint a temp path");
    assert!(
        is_a_mint(banned, at + MINT.len()),
        "the sample must read as a mint, or the fence would skip it",
    );
    let window = &banned[at..banned.len().min(at + 400)];
    assert!(
        !window.contains(SEAM),
        "the banned idiom must not appear compliant to the fence",
    );

    // A `temp_dir()` that only names the directory is not a mint and is not policed.
    let read_only = "assert!(scratch.starts_with(std::env::temp_dir()), \"under tmp\");";
    let at = read_only.find(MINT).expect("sample contains the call");
    assert!(
        !is_a_mint(read_only, at + MINT.len()),
        "reading the temp dir without building a name must not be flagged",
    );

    let compliant = banned.replace(
        "std::time::SystemTime::now()\n                .duration_since(std::time::UNIX_EPOCH)\n                .unwrap()\n                .as_nanos()",
        "engine::tempname::unique_nanos()",
    );
    let at = compliant
        .find(MINT)
        .expect("the sample must mint a temp path");
    assert!(
        compliant[at..].contains(SEAM),
        "the repaired idiom must satisfy the fence",
    );
}
