//! The two **probe-boundary fences** M54 S4 names
//! ([module-layout.md](../../../implementation/module-layout.md) → Probe boundary, the
//! bundled probe — the intercept and wire-types bullets).
//!
//! The bundled `doc-code` probe now lives in the `jigc` bin's module tree and runs by
//! self-exec (`jigc __probe doc-code --build <version>`). Two properties the old,
//! separate-binary shape held by construction now hold only by discipline, so each gets
//! a fence:
//!
//! - **The wire types stay independently declared.** The probe imports neither `engine`
//!   nor `cli` — which is what keeps the *any-language* claim of the subprocess seam
//!   proven rather than asserted: the one shipped probe still could have been written in
//!   anything. Sharing a crate with `cli` makes the import one `use` away, so the fence
//!   reads the probe's own source, blanked of comments and strings through
//!   [`support::rust_source`](crate::support::rust_source), and names every path that
//!   reaches either crate — directly (`engine::…`, `cli::…`, `use engine;`, `extern
//!   crate`), or through the bin's root, whose private imports (`use cli::invoke`) are
//!   visible to every descendant module (`crate::invoke::…`, or a `super::` chain that
//!   climbs out of the probe's module).
//! - **A probe spawn is never logged as an agent's call.** The intercept is the first
//!   statement of `main`, ahead of the route fence, the invocation-log gate and the
//!   output tee. Behaviourally: in a repository with the invocation log on, a direct
//!   `jigc __probe doc-code --build <v>` (this build's argv, and a skewed one) adds
//!   **zero** records, while `jigc validate` over an anchored store — which spawns the
//!   probe, as its `doc-code` finding proves — adds **exactly one**, its own, and no
//!   record's argv carries `__probe`. Structurally: nothing precedes the intercept in
//!   `main`'s body. The structural half exists because an intercept moved merely *below*
//!   `enabled_logs_dir()` still returns before the record is written, so the log alone
//!   cannot see it — while the resolution it now pays on every probe spawn is exactly
//!   what "first statement" forbids.

use crate::support::rust_source::{code_only, rust_files};
use crate::support::trial_corpus::{State, TrialCorpus, VENDORED_CODE_FILE, VENDORED_CODE_SYMBOL};
use std::fs;
use std::path::{Path, PathBuf};

/// The crates the probe must not name.
const BANNED_CRATES: &[&str] = &["engine", "jigc_engine", "cli"];

/// The probe's module directory in the `jigc` bin's tree.
fn probe_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("doc_code_probe")
}

/// An identifier token and its byte offset in blanked code.
struct Ident<'a> {
    at: usize,
    text: &'a str,
}

/// Every identifier in `code`, in order. `code` is comment- and string-blanked, so a
/// banned name inside prose or a literal never appears here.
fn idents(code: &str) -> Vec<Ident<'_>> {
    let bytes = code.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            // A token glued to a digit prefix (`0x_ff`, `1_000`) is a literal, not a name.
            let glued = start > 0 && bytes[start - 1].is_ascii_digit();
            if !glued {
                out.push(Ident {
                    at: start,
                    text: &code[start..i],
                });
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Whether the next non-whitespace text after `end` is the path separator `::`.
fn followed_by_path_sep(code: &str, end: usize) -> bool {
    code[end..].trim_start().starts_with("::")
}

/// The byte ranges of every inline `mod <name> { … }` body in `code`.
fn inline_mod_regions(code: &str) -> Vec<(usize, usize)> {
    let bytes = code.as_bytes();
    let ids = idents(code);
    let mut regions = Vec::new();
    for pair in ids.windows(2) {
        if pair[0].text != "mod" {
            continue;
        }
        let after_name = pair[1].at + pair[1].text.len();
        let rest = &code[after_name..];
        let Some(open_rel) = rest.find(|c: char| !c.is_whitespace()) else {
            continue;
        };
        if bytes[after_name + open_rel] != b'{' {
            continue;
        }
        let open = after_name + open_rel;
        let mut depth = 0usize;
        for (k, &b) in bytes.iter().enumerate().skip(open) {
            match b {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        regions.push((open, k));
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    regions
}

/// How many modules below the crate root the file's own module sits: `mod.rs` is the
/// `doc_code_probe` module itself (1), `resolve.rs` its child (2).
fn file_module_depth(rel: &Path) -> usize {
    let components = rel.components().count();
    let is_mod_rs = rel.file_name().is_some_and(|n| n == "mod.rs");
    1 + components - usize::from(is_mod_rs)
}

/// Every path in one probe source file that reaches `engine` or `cli`, as
/// `<file>:<line>: <reason>`.
fn offences_in(rel: &Path, code: &str) -> Vec<String> {
    let ids = idents(code);
    let regions = inline_mod_regions(code);
    let base_depth = file_module_depth(rel);
    let line_of = |at: usize| code[..at].matches('\n').count() + 1;
    let mut out = Vec::new();
    let mut k = 0;
    while k < ids.len() {
        let id = &ids[k];
        let end = id.at + id.text.len();
        let prev = k.checked_sub(1).map(|p| ids[p].text);
        let prev2 = k.checked_sub(2).map(|p| ids[p].text);
        let at = format!("{}:{}", rel.display(), line_of(id.at));

        if BANNED_CRATES.contains(&id.text)
            && (followed_by_path_sep(code, end)
                || prev == Some("use")
                || (prev2 == Some("extern") && prev == Some("crate")))
        {
            out.push(format!("{at}: names the `{}` crate", id.text));
        } else if id.text == "crate"
            && followed_by_path_sep(code, end)
            && ids.get(k + 1).map(|n| n.text) != Some("doc_code_probe")
        {
            // `crate` is the `jigc` bin's root, whose private `use cli::…` imports every
            // descendant module can see — a path through it leaves the probe.
            out.push(format!(
                "{at}: `crate::{}` reaches the bin root, outside the probe",
                ids.get(k + 1).map_or("", |n| n.text),
            ));
        } else if id.text == "super" && followed_by_path_sep(code, end) {
            // Count the whole `super::super::…` chain, then compare it with how deep this
            // site sits below the crate root: a chain that long climbs out of the probe.
            let mut chain = 1;
            while ids.get(k + chain).is_some_and(|n| n.text == "super") {
                chain += 1;
            }
            let inline_depth = regions
                .iter()
                .filter(|(lo, hi)| id.at > *lo && id.at < *hi)
                .count();
            if chain >= base_depth + inline_depth {
                out.push(format!(
                    "{at}: a `super::` chain of {chain} climbs out of the probe module"
                ));
            }
            k += chain;
            continue;
        }
        k += 1;
    }
    out
}

/// (a) The probe's wire types are independently declared: no file in the probe's module
/// names a path into `engine` or `cli`, directly or through the bin root.
#[test]
fn the_probe_module_names_no_engine_or_cli_path() {
    let dir = probe_dir();
    let files = rust_files(&dir);
    let rels: Vec<PathBuf> = files
        .iter()
        .map(|f| {
            f.strip_prefix(&dir)
                .expect("under the probe dir")
                .to_path_buf()
        })
        .collect();
    assert!(
        rels.iter().any(|r| r == Path::new("mod.rs")),
        "the probe module's root `mod.rs` is scanned; found {rels:?} under {}",
        dir.display(),
    );

    let mut offences = Vec::new();
    let mut wire_types_seen = Vec::new();
    for (file, rel) in files.iter().zip(&rels) {
        let body = fs::read_to_string(file).expect("read a probe source file");
        let code = code_only(&body);
        // Anti-vacuous: the scan reads the file that declares the wire types, as code.
        for ty in ["ProbeRequest", "ProbeResponse", "EffectiveStateSnapshot"] {
            if code.contains(&format!("struct {ty}")) {
                wire_types_seen.push(ty);
            }
        }
        offences.extend(offences_in(rel, &code));
    }
    wire_types_seen.sort_unstable();
    assert_eq!(
        wire_types_seen,
        ["EffectiveStateSnapshot", "ProbeRequest", "ProbeResponse"],
        "the scan must read the probe's own wire-type declarations as code",
    );
    assert!(
        offences.is_empty(),
        "the bundled `doc-code` probe keeps its own wire types and imports neither \
         `engine` nor `cli` (module-layout.md → Probe boundary); offending paths:\n  {}",
        offences.join("\n  "),
    );
}

/// The scanner itself: each way a path can reach `engine` or `cli` is named, and the
/// probe-internal paths the real module uses are not.
#[test]
fn the_scanner_names_every_route_out_and_no_route_within() {
    let root = Path::new("mod.rs");
    let child = Path::new("resolve.rs");
    let cases: &[(&Path, &str, bool)] = &[
        (root, "use engine::probe::ProbeRequest;", true),
        (root, "use jigc_engine::finding::Finding;", true),
        (root, "use cli::invoke;", true),
        (root, "use engine;", true),
        (root, "use engine as e;", true),
        (root, "use { engine :: x };", true),
        (root, "extern crate cli;", true),
        (root, "fn f() { let _ = ::engine::x(); }", true),
        (root, "use crate::invoke::BUILD_ID;", true),
        (root, "use super::invoke;", true),
        (child, "use super::super::invoke;", true),
        (child, "mod t { use super::super::super::x; }", true),
        (root, "use crate::doc_code_probe::resolve;", false),
        (root, "mod resolve; mod tests { use super::*; }", false),
        (
            child,
            "use super::Finding; mod tests { use super::*; }",
            false,
        ),
        (
            root,
            "use std::path::Path; fn f(engine: u8, cli: u8) {}",
            false,
        ),
    ];
    for (rel, src, offends) in cases {
        let found = offences_in(rel, &code_only(src));
        assert_eq!(
            !found.is_empty(),
            *offends,
            "{}: `{src}` — expected offence={offends}, got {found:?}",
            rel.display(),
        );
    }
}

/// The parsed JSONL records at `.jigc/logs/invocations.jsonl` (empty when absent).
fn log_records(repo: &Path) -> Vec<serde_json::Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Whether a record's argv carries `token`.
fn argv_has(record: &serde_json::Value, token: &str) -> bool {
    record["argv"]
        .as_array()
        .is_some_and(|a| a.iter().any(|v| v.as_str() == Some(token)))
}

/// (b) In a repository with the invocation log on, a probe spawn adds zero records —
/// both a direct spawn with this build's argv and a skewed one — while `jigc validate`
/// over an anchored store, which spawns the probe, adds exactly its own.
#[test]
fn a_probe_spawn_adds_zero_invocation_log_records() {
    let corpus = TrialCorpus::build(State::Vendored);
    let repo = corpus.repo();
    corpus.jigc_ok(&["config", "set", "invocation-log", "true"]);
    let baseline = log_records(&repo).len();

    // This build's own probe argv — the emitted spelling, never a hand-built one.
    let argv = cli::invoke::doc_code_probe_args();
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    let run = corpus.jigc(&argv);
    let run_err = String::from_utf8_lossy(&run.stderr);
    assert_eq!(
        run.status.code(),
        Some(1),
        "an empty request makes the probe exit 1 — not clap's usage exit 2; stderr:\n{run_err}",
    );
    // The probe arm ran, not clap: its stderr is the probe's own one-line reason for the
    // empty request (since the M54 audit, which completes S4, a probe failure says why).
    assert!(
        run_err.starts_with("doc-code probe: cannot parse the request: ")
            && run_err.lines().count() == 1,
        "the probe arm ran, not clap: its empty-request failure is the probe's one reason \
         line; stderr:\n{run_err}",
    );

    let mut skewed: Vec<&str> = argv.clone();
    *skewed.last_mut().expect("the argv ends with the build id") = "0.0.0-skewed";
    let refused = corpus.jigc(&skewed);
    let refused_err = String::from_utf8_lossy(&refused.stderr);
    assert_eq!(
        refused.status.code(),
        Some(1),
        "a skewed probe argv is refused with exit 1; stderr:\n{refused_err}",
    );
    assert!(
        refused_err.contains("0.0.0-skewed"),
        "the refusal names the skewed build; stderr:\n{refused_err}",
    );

    assert_eq!(
        log_records(&repo).len(),
        baseline,
        "a probe spawn is jigc's own child, never an agent's call — it adds no record",
    );

    // The before-control and the indirect spawn in one: drift the anchored symbol in the
    // working tree, so the finding proves `validate` spawned the probe, and the log
    // proves it is on.
    fs::write(
        repo.join(VENDORED_CODE_FILE),
        "export function renamed(s: string): string {\n  return s;\n}\n",
    )
    .expect("drift the anchored symbol");
    let validated = corpus.jigc(&["validate"]);
    let validated_out = String::from_utf8_lossy(&validated.stdout);
    assert!(
        validated_out.contains("doc-code.symbol-exists")
            && validated_out.contains(VENDORED_CODE_SYMBOL),
        "`jigc validate` spawned the probe, which reported the drifted anchor; got:\n\
         {validated_out}\nstderr:\n{}",
        String::from_utf8_lossy(&validated.stderr),
    );

    let records = log_records(&repo);
    assert_eq!(
        records.len(),
        baseline + 1,
        "`jigc validate` adds exactly its own record, none for the probe it spawned; \
         new records: {:?}",
        &records[baseline.min(records.len())..],
    );
    assert!(
        argv_has(&records[baseline], "validate"),
        "the one new record is validate's own: {}",
        records[baseline],
    );
    let probe_records: Vec<_> = records.iter().filter(|r| argv_has(r, "__probe")).collect();
    assert!(
        probe_records.is_empty(),
        "no record's argv carries `__probe`: {probe_records:?}",
    );
}

/// (b), structurally: nothing precedes the probe intercept in `main`'s body — not the
/// route fence, not `enabled_logs_dir()`, not the output tee. An intercept moved below the
/// log gate still returns before the record is written, so the log cannot see it; this
/// can.
#[test]
fn the_probe_intercept_is_the_first_statement_of_main() {
    let main_rs = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("main.rs");
    let code = code_only(&fs::read_to_string(&main_rs).expect("read main.rs"));
    let fn_main = code
        .find("fn main()")
        .expect("main.rs declares `fn main()`");
    let open = fn_main + code[fn_main..].find('{').expect("`fn main` has a body");
    let body = &code[open + 1..];
    let intercept = body
        .find("probe_intercept()")
        .expect("`main` calls `probe_intercept()`");
    let before = &body[..intercept];
    assert!(
        !before.contains(';') && !before.contains('}'),
        "the probe intercept must be the FIRST statement of `main` — ahead of the route \
         fence, the invocation-log gate and the output tee (module-layout.md → Probe \
         boundary); code before it:\n{}",
        before.trim(),
    );
    assert_eq!(
        before.split_whitespace().collect::<Vec<_>>(),
        ["if", "let", "Some(code)", "="],
        "the first statement is the intercept's own `if let`, nothing ahead of it",
    );
}
