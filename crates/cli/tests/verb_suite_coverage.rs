//! **Every leaf verb is NAMED by at least one integration suite** — the mechanical
//! half of *a coverage classification is a claim about the code*
//! ([pinning.md](../../../implementation/pinning.md) → Classifying coverage).
//!
//! # What this proves, and what it does not
//!
//! **It proves:** for every leaf verb the clap tree enumerates, some suite under
//! `crates/cli/tests/` mentions that verb's argv path — so no verb ships with
//! *nothing at all* pointed at it, and the verb → suite map below is the lookup
//! table that answers "where would I even look?".
//!
//! **It does not prove the verb's behaviour is fenced.** "Named by a suite" is a
//! strictly weaker property than "covered": a suite that drives `jigc rename` once
//! to set up some other assertion names it exactly as loudly as one that sweeps its
//! whole reject surface. This test catches a verb with **zero** coverage; it says
//! nothing about **shallow** coverage, and nothing about which *behaviours* of a
//! named verb are reached. Read a green here as "the map is non-empty everywhere",
//! never as "the surface is fenced" — treating it as the latter would be exactly
//! the overclaim this fence was built in response to.
//!
//! # Why it exists
//!
//! Three times in two days a coverage classification was made from the artifact in
//! front of the classifier rather than from the code: `rename` was classified
//! "untouched" from a *file-level* diff (true of `rename.rs`, false of the verb —
//! its behaviour moved in `cli.rs`), and then a set of surfaces was twice asserted
//! "reached by no probe" — once by the orchestrator, once by an independent
//! reviewer — without anyone grepping the suites. Checking took one command and
//! dissolved most of the list. The judgment half of that lesson is the rule in
//! `pinning.md`; this is the half a machine can hold.
//!
//! # How "named" is decided
//!
//! A suite names verb path `p` when either
//!
//!   * its source contains `p`'s segments as a **contiguous run of quoted argv
//!     tokens** — `&["doc", "set-field", …]`, the shape every suite drives the
//!     binary through (runs are separated only by whitespace, `,` and `&`, so
//!     literals in unrelated statements cannot fuse into a false match); or
//!   * its text contains `jigc <p joined by spaces>` — the prose/assert-message
//!     form, which is how a suite that drives through a verb-prefixing helper names
//!     its verb (`config_replace_remove_step.rs` passes `&["replace-step", …]` to a
//!     `run_config` that supplies `config`, so the argv rule alone would call it a
//!     hole; the reject arm below keeps the loosened rule honest).
//!
//! Both are *textual* — the honest bound restated: this reads the suites, it does
//! not run them.

use clap::CommandFactory;
use cli::cli::Cli;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The verb-tree members deliberately outside the sweep, each with its reason —
/// a stated exclusion, never a silent skip.
const EXCLUDED_VERBS: &[(&str, &str)] = &[(
    "help",
    "clap's auto-generated builtin — not a jigc verb, and no suite should name it",
)];

/// The suite files deliberately outside the *counting* set, each with its reason.
///
/// **This list is what makes the assertion falsifiable at all.**
/// `format_json_success_axis.rs` holds a hand-written success recipe for every leaf
/// verb, kept exhaustive by a bijection against this same clap tree — so counting it
/// would make "every verb is named" true **by construction**, green forever, unable
/// to fail. Excluding it turns the property into something with content: *some suite
/// named this verb on purpose*, not merely because an axis sweep was forced to
/// enumerate it. A verb that lands and reddens here is one whose only mention in the
/// whole tree is a recipe the bijection demanded.
const EXCLUDED_SUITES: &[(&str, &str)] = &[
    (
        "format_json_success_axis",
        "one hand-written recipe per leaf verb, held exhaustive by a bijection with \
         this same clap tree — counting it makes the assertion unfalsifiable",
    ),
    (
        "verb_suite_coverage",
        "this fence's own prose names verbs as evidence; a fence must not be its own \
         evidence",
    ),
];

/// Every **leaf** verb's argv path, walked from the clap `Command` tree — the real
/// enumeration seam, so a subcommand added anywhere auto-joins this sweep. The
/// `leaf_verb_paths()` idiom of `machine_output.rs` / `format_json_success_axis.rs`,
/// kept local as those suites keep it, with the skip lifted into [`EXCLUDED_VERBS`].
fn leaf_verb_paths() -> Vec<Vec<String>> {
    fn walk(cmd: &clap::Command, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
        let mut had_child = false;
        for sub in cmd.get_subcommands() {
            if EXCLUDED_VERBS.iter().any(|(v, _)| *v == sub.get_name()) {
                continue;
            }
            had_child = true;
            let mut child = prefix.clone();
            child.push(sub.get_name().to_string());
            walk(sub, child, out);
        }
        if !had_child && !prefix.is_empty() {
            out.push(prefix);
        }
    }
    let mut out = Vec::new();
    walk(&Cli::command(), Vec::new(), &mut out);
    out.sort();
    out
}

fn tests_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

/// One counted suite file, read and scanned once.
struct Suite {
    stem: String,
    src: String,
    runs: Vec<Vec<String>>,
}

/// The counted suites: `tests/*.rs` minus [`EXCLUDED_SUITES`], each scanned once.
fn counted_suites() -> Vec<Suite> {
    let mut v: Vec<Suite> = fs::read_dir(tests_dir())
        .expect("tests/ is readable")
        .filter_map(|e| {
            let p = e.expect("dir entry").path();
            let stem = p.file_name()?.to_str()?.strip_suffix(".rs")?.to_string();
            if !p.is_file() || EXCLUDED_SUITES.iter().any(|(s, _)| *s == stem) {
                return None;
            }
            let src = fs::read_to_string(&p).expect("suite is readable");
            let runs = literal_runs(&src);
            Some(Suite { stem, src, runs })
        })
        .collect();
    v.sort_by(|a, b| a.stem.cmp(&b.stem));
    v
}

/// Maximal runs of adjacent string literals — literals separated **only** by
/// whitespace, `,` or `&`, which is the argv-list shape (`&["doc", "show"]`) and
/// nothing else. Any other character (a `(`, a `]`, an identifier) ends the run, so
/// two literals in unrelated statements never fuse into a false match. Line comments
/// are transparent: a comment between two array elements does not split them.
fn literal_runs(src: &str) -> Vec<Vec<String>> {
    let b: Vec<char> = src.chars().collect();
    let mut runs: Vec<Vec<String>> = Vec::new();
    let mut cur: Vec<String> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            '"' => {
                let mut lit = String::new();
                let mut j = i + 1;
                while j < b.len() && b[j] != '"' {
                    if b[j] == '\\' {
                        j += 1;
                        if j < b.len() {
                            lit.push(b[j]);
                        }
                    } else {
                        lit.push(b[j]);
                    }
                    j += 1;
                }
                cur.push(lit);
                i = j + 1;
            }
            '/' if i + 1 < b.len() && b[i + 1] == '/' => {
                while i < b.len() && b[i] != '\n' {
                    i += 1;
                }
            }
            ' ' | '\t' | '\n' | '\r' | ',' | '&' => i += 1,
            _ => {
                if !cur.is_empty() {
                    runs.push(std::mem::take(&mut cur));
                }
                i += 1;
            }
        }
    }
    if !cur.is_empty() {
        runs.push(cur);
    }
    runs
}

/// Whether one suite names `path` — the two-form rule stated in the module doc.
fn names_verb(src: &str, runs: &[Vec<String>], path: &[String]) -> bool {
    let contiguous = runs
        .iter()
        .any(|run| run.windows(path.len()).any(|w| w == path));
    contiguous || src.contains(&format!("jigc {}", path.join(" ")))
}

/// The suites naming `path`, over the counted set.
fn suites_naming<'a>(suites: &'a [Suite], path: &[String]) -> Vec<&'a str> {
    suites
        .iter()
        .filter(|s| names_verb(&s.src, &s.runs, path))
        .map(|s| s.stem.as_str())
        .collect()
}

/// verb path → the suites naming it. The lookup table this whole file exists to
/// produce.
fn coverage_map(suites: &[Suite]) -> BTreeMap<String, Vec<String>> {
    leaf_verb_paths()
        .into_iter()
        .map(|path| {
            let naming = suites_naming(suites, &path)
                .into_iter()
                .map(str::to_string)
                .collect();
            (path.join(" "), naming)
        })
        .collect()
}

/// Render the map, capping each verb's list at `cap` suites (`None` = uncapped).
fn render_map(map: &BTreeMap<String, Vec<String>>, cap: Option<usize>) -> String {
    let mut s = String::new();
    for (verb, suites) in map {
        let shown: Vec<&str> = match cap {
            Some(n) => suites.iter().take(n).map(String::as_str).collect(),
            None => suites.iter().map(String::as_str).collect(),
        };
        let more = suites.len().saturating_sub(shown.len());
        let tail = if more > 0 {
            format!(" (+{more} more)")
        } else {
            String::new()
        };
        let list = if shown.is_empty() {
            "— NOTHING —".to_string()
        } else {
            shown.join(", ")
        };
        s.push_str(&format!("  {:24} {:3}  {list}{tail}\n", verb, suites.len()));
    }
    s
}

/// **The fence.** Every leaf verb the clap tree enumerates is named by at least one
/// counted suite — see the module doc for what "named" means and, more importantly,
/// for what a green here does **not** license anyone to claim.
///
/// On failure the message *is* the lookup table, because the recurring failure was
/// people not knowing where to look. To read the map deliberately without breaking
/// anything, run the ignored companion below:
///
/// ```text
/// cargo test -p cli verb_suite_coverage -- --ignored --nocapture
/// ```
///
/// An `#[ignore]`d printer rather than a forced failure: it is a normal, repeatable
/// command that costs the gate nothing, and it can print the map **uncapped** where
/// an assertion message must stay readable.
#[test]
fn every_leaf_verb_is_named_by_at_least_one_integration_suite() {
    let suites = counted_suites();
    let map = coverage_map(&suites);

    assert!(
        map.len() >= 46,
        "the clap tree must still enumerate the whole verb surface (>= 46 leaf verbs); \
         got {}. A shrinking axis is how a sweep silently stops sweeping.\n{}",
        map.len(),
        render_map(&map, Some(4)),
    );

    let holes: Vec<&String> = map
        .iter()
        .filter(|(_, suites)| suites.is_empty())
        .map(|(verb, _)| verb)
        .collect();

    assert!(
        holes.is_empty(),
        "these leaf verbs are named by NO integration suite: {holes:?}\n\n\
         A verb with no suite naming it has no coverage to classify — write one, or, \
         if it is genuinely exempt, add it to `EXCLUDED_VERBS` with its reason.\n\n\
         The whole verb -> naming-suites map (`cargo test -p cli verb_suite_coverage \
         -- --ignored --nocapture` prints it uncapped):\n{}",
        render_map(&map, Some(4)),
    );

    // **Negative control.** The rule above is deliberately loose (a prose mention
    // counts), so it must still be capable of saying *no* — otherwise a green here
    // would only prove the matcher answers yes to everything.
    for fake in [
        vec!["definitely-not-a-verb".to_string()],
        vec!["doc".to_string(), "definitely-not-a-verb".to_string()],
    ] {
        let matched = suites_naming(&suites, &fake);
        assert!(
            matched.is_empty(),
            "the naming rule matched a verb that does not exist ({fake:?}), claimed by \
             {} of {} suites ({:?}…) — it is answering yes to everything, so the sweep \
             above proves nothing",
            matched.len(),
            suites.len(),
            &matched[..matched.len().min(5)],
        );
    }
}

/// The lookup table, printed on demand and **uncapped** — the deliberate way to
/// answer "which suites name this verb?" before classifying anything as uncovered.
///
/// Ignored by default: it asserts nothing, so it is a report, not a fence.
#[test]
#[ignore = "a report, not a fence: prints the verb -> naming-suites map on demand"]
fn print_the_verb_to_naming_suite_map() {
    let map = coverage_map(&counted_suites());
    println!(
        "\nverb -> integration suites naming it \
         ({} leaf verbs; excluded suites: {})\n\n{}\n\
         Reminder: \"named by\" is not \"fenced by\" — see this suite's module doc.",
        map.len(),
        EXCLUDED_SUITES
            .iter()
            .map(|(s, _)| *s)
            .collect::<Vec<_>>()
            .join(", "),
        render_map(&map, None),
    );
}
