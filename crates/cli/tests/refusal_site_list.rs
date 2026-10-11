//! The product's refusal sites as **one scan and one committed list** — the prelude of the
//! reshaped `test` stage's build (`S0` of
//! `completions/artifacts/M55/stabilization-build/reshape/plan.md`).
//!
//! The product has no list of its refusals: a finding code is a free string, minted at
//! constructor calls spread over two crates. Two things need that list — a route hunter
//! that is handed a command's sites, and a catalog of cases a fence holds to *every site
//! has a case or a stated reason* — and both read **the same file**,
//! `crates/cli/tests/route_catalog/sites.tsv`, so neither counts for itself. The scan lives in
//! [`support::refusal_sites`](crate::support::refusal_sites), which says what a site is, what
//! each column holds and what the scan cannot see.
//!
//! This suite holds the file to the scan and the scan to its claims:
//!
//!   1. [`the_committed_list_is_the_scan`] — the file is the scan, byte for byte. A producer
//!      added, moved to another function or re-coded reddens here, with the rows named.
//!      **Regenerate, never edit:** `UPDATE_GOLDENS=1 cargo test -p jigc refusal_site_list::`
//!      — the goldens' switch, refused under `CI` the way theirs is
//!      ([`support::goldens::update_mode`](crate::support::goldens::update_mode)), so a
//!      regenerated list can never green CI. The diff is part of the commit under review.
//!   2. [`a_site_inside_an_interleaved_test_module_is_not_listed_and_one_after_it_is`] — a
//!      list of *production* sites, on a tree that interleaves its test modules.
//!   3. [`a_code_behind_a_constant_is_resolved_and_one_behind_an_expression_is_dynamic`].
//!   4. [`a_site_keeps_its_id_when_lines_above_it_move`] — an id is file, function, code and
//!      ordinal; a line is in no row, so an edit above a site moves nothing in the file.
//!   5. [`a_dynamic_site_is_listed_unassigned_and_never_dropped`] — every constructor call in
//!      production code is a row, whatever its code is.
//!   6. [`on_this_tree_the_finalize_namespace_is_the_registrys`] — the cross-check against the
//!      one family that already had a scanner and a hand-kept registry.
//!
//! and what the six lean on: a refusal's severity and route are read as written, a constant
//! two files define differently resolves in its own file only, the function an id names is
//! one its file declares, the family table is total over the codes a call spells and holds
//! no row for a prefix none spells, and a site that follows a quote character literal is
//! still seen.
//!
//! **What is not pinned here.** No count: the sizes of the `setup` and `uninstall` families
//! stand in the commit that landed this and in the file itself, never in an assertion — a
//! pinned count is the hand count this list replaces.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use crate::support::refusal_sites::{
    self, CONSTRUCTORS, DYNAMIC, FAMILIES, Site, Source, UNASSIGNED,
};
use crate::support::{goldens, rust_source};

/// A production path for a fixture: under a crate's `src/`, with no `tests` component, so
/// the reader's test-domain rule leaves it to the `#[cfg(test)]` regions alone.
const FIXTURE: &str = "crates/cli/src/fixture.rs";

fn scan_one(body: &str) -> Vec<Site> {
    refusal_sites::scan(&[Source::new(FIXTURE, body)])
}

fn codes(sites: &[Site]) -> Vec<&str> {
    sites.iter().map(|s| s.code.as_str()).collect()
}

fn ids(sites: &[Site]) -> Vec<String> {
    sites.iter().map(Site::id).collect()
}

/// The one site of `sites` in `function` — a fixture names each site by the function that
/// holds it.
fn in_fn<'a>(sites: &'a [Site], function: &str) -> &'a Site {
    let found: Vec<&Site> = sites.iter().filter(|s| s.function == function).collect();
    assert_eq!(
        found.len(),
        1,
        "the fixture holds exactly one site in `{function}`; the scan listed: {:?}",
        ids(sites),
    );
    found[0]
}

/// The rows of a list, without its `#` header lines.
fn rows(text: &str) -> BTreeSet<&str> {
    text.lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect()
}

// ---------------------------------------------------------------------------
// 1 — the file is the scan
// ---------------------------------------------------------------------------

/// A scan that finds fewer sites than this has lost its reader, not its sites — the
/// precedent fence's floor (`finalize_family_registry.rs`), for the same reason: an empty
/// scan would agree with an empty file.
const SITE_FLOOR: usize = 100;

#[test]
fn the_committed_list_is_the_scan() {
    let sites = refusal_sites::scan_tree();
    assert!(
        sites.len() >= SITE_FLOOR,
        "the scan found only {} refusal sites across both crates — the source view \
         regressed, so nothing below is trustworthy",
        sites.len(),
    );
    let scanned = refusal_sites::render(&sites);
    let path = refusal_sites::list_path();

    if goldens::update_mode(
        std::env::var("UPDATE_GOLDENS").ok().as_deref(),
        std::env::var("CI").ok().as_deref(),
    ) {
        fs::create_dir_all(path.parent().expect("the list has a parent"))
            .expect("create the list's directory");
        fs::write(&path, &scanned).expect("write the list");
        return;
    }

    const REGEN: &str = "UPDATE_GOLDENS=1 cargo test -p jigc refusal_site_list::";
    let committed = fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the committed list of refusal sites is missing at {}: {e}\nRegenerate with: {REGEN}",
            path.display(),
        )
    });
    if committed == scanned {
        return;
    }

    let (in_scan, in_file) = (rows(&scanned), rows(&committed));
    let unlisted: Vec<&str> = in_scan.difference(&in_file).copied().collect();
    let gone: Vec<&str> = in_file.difference(&in_scan).copied().collect();
    panic!(
        "{} is not the scan of production source.\n\
         {} site(s) the source holds and the list does not:\n{}\n\
         {} row(s) the list holds and the source does not:\n{}\n\
         {}\
         The list is generated, never edited: look at what moved, then regenerate with\n  \
         {REGEN}\nand commit the diff with the change that caused it.",
        path.display(),
        unlisted.len(),
        unlisted.join("\n"),
        gone.len(),
        gone.join("\n"),
        if unlisted.is_empty() && gone.is_empty() {
            "The rows agree, so the header or the order moved.\n"
        } else {
            ""
        },
    );
}

// ---------------------------------------------------------------------------
// 2 — production code only, on a tree that interleaves its test modules
// ---------------------------------------------------------------------------

const INTERLEAVED: &str = r#"
use engine::finding::Finding;

fn before() -> Finding {
    Finding::block("setup.before", "m", "re-run `jigc setup`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fixture() {
        let _ = Finding::block("setup.in-a-test", "m", "r");
    }
}

fn after() -> Finding {
    Finding::block("setup.after", "m", "re-run `jigc setup`")
}
"#;

#[test]
fn a_site_inside_an_interleaved_test_module_is_not_listed_and_one_after_it_is() {
    let sites = scan_one(INTERLEAVED);
    assert_eq!(
        codes(&sites),
        ["setup.after", "setup.before"],
        "a test module that production code resumes after bounds at its own closing brace: \
         the site inside it is a fixture, the one after it is a producer",
    );

    // The same source as a file of a `tests/` tree is test code whole.
    let as_a_suite =
        refusal_sites::scan(&[Source::new("crates/cli/tests/fixture.rs", INTERLEAVED)]);
    assert!(
        as_a_suite.is_empty(),
        "a file under `tests/` holds no production site; the scan listed {:?}",
        ids(&as_a_suite),
    );
}

// ---------------------------------------------------------------------------
// 3 — a code is a literal, a resolved constant, or dynamic
// ---------------------------------------------------------------------------

const CODES: &str = r#"
const INSTALL_CODE: &str = "setup.install-thing";

fn by_literal() -> Finding {
    Finding::block("setup.spelled-here", "m", "r")
}

fn by_constant() -> Finding {
    Finding::block(INSTALL_CODE, "m", "r")
}

fn by_a_constant_of_another_file() -> Finding {
    Finding::graded(Severity::Blocking, crate::other::REMOTE_CODE, "m", None, None)
}

fn by_a_constant_nothing_defines() -> Finding {
    Finding::block(NOWHERE_CODE, "m", "r")
}

fn by_a_method(kind: Kind) -> Finding {
    Finding::block(kind.code(), "m", "r")
}

fn by_a_parameter(code: &str) -> Finding {
    Finding::block(code, "m", "r")
}

fn by_a_built_string(check: &str) -> Finding {
    Finding::block(format!("setup.{check}"), "m", "r")
}
"#;

const OTHER_FILE: &str = r#"
pub const REMOTE_CODE: &str = "uninstall.remote-thing";

#[cfg(test)]
mod tests {
    const NOWHERE_CODE: &str = "setup.only-a-test-says-so";
}
"#;

#[test]
fn a_code_behind_a_constant_is_resolved_and_one_behind_an_expression_is_dynamic() {
    let sites = refusal_sites::scan(&[
        Source::new(FIXTURE, CODES),
        Source::new("crates/cli/src/other.rs", OTHER_FILE),
    ]);

    let read = |function: &str| {
        let site = in_fn(&sites, function);
        (site.code.as_str(), site.written.as_str())
    };
    assert_eq!(read("by_literal"), ("setup.spelled-here", "literal"));
    assert_eq!(
        read("by_constant"),
        ("setup.install-thing", "INSTALL_CODE"),
        "a constant resolves to its value, and the row keeps the name it was written by",
    );
    assert_eq!(
        read("by_a_constant_of_another_file"),
        ("uninstall.remote-thing", "crate::other::REMOTE_CODE"),
        "a constant resolves across files, by the last segment of its path",
    );
    assert_eq!(
        read("by_a_constant_nothing_defines"),
        (DYNAMIC, "NOWHERE_CODE"),
        "a name no production `const` defines is not a code — a test module's is not one",
    );
    assert_eq!(read("by_a_method"), (DYNAMIC, "kind.code()"));
    assert_eq!(read("by_a_parameter"), (DYNAMIC, "code"));
    assert_eq!(
        read("by_a_built_string"),
        (DYNAMIC, r#"format!("setup.{check}")"#),
        "a code built at the call is an expression, however code-shaped its template",
    );
}

/// Two files may each hold a private constant of one name. A site reads its own file's;
/// a third file that names it, with two values to choose from, reads neither.
#[test]
fn a_constant_two_files_define_differently_resolves_in_its_own_file_only() {
    let own = |code: &str| {
        format!(
            "const CODE: &str = \"{code}\";\nfn mint() -> Finding {{\n    \
             Finding::block(CODE, \"m\", \"r\")\n}}\n"
        )
    };
    let sites = refusal_sites::scan(&[
        Source::new("crates/cli/src/a.rs", &own("setup.from-a")),
        Source::new("crates/cli/src/b.rs", &own("uninstall.from-b")),
        Source::new(
            "crates/cli/src/c.rs",
            "fn mint() -> Finding {\n    Finding::block(crate::a::CODE, \"m\", \"r\")\n}\n",
        ),
    ]);
    assert_eq!(
        codes(&sites),
        ["setup.from-a", "uninstall.from-b", DYNAMIC],
        "sorted by file: a.rs and b.rs each read their own constant, c.rs cannot choose",
    );
}

// ---------------------------------------------------------------------------
// 4 — an id holds no line
// ---------------------------------------------------------------------------

const TWICE: &str = r#"
fn refuses(first: bool) -> Finding {
    if first {
        return Finding::block("setup.twice", "the first", "r");
    }
    Finding::block("setup.twice", "the second", "s")
}

fn other() -> Finding {
    Finding::block("setup.once", "m", "r")
}
"#;

#[test]
fn a_site_keeps_its_id_when_lines_above_it_move() {
    let before = scan_one(TWICE);
    assert_eq!(
        ids(&before),
        [
            "crates/cli/src/fixture.rs::other::setup.once#1",
            "crates/cli/src/fixture.rs::refuses::setup.twice#1",
            "crates/cli/src/fixture.rs::refuses::setup.twice#2",
        ],
        "an id is file, enclosing function, code and the ordinal among the function's sites \
         of that code",
    );
    assert_eq!(
        (before[1].route_text.as_str(), before[2].route_text.as_str()),
        ("r", "s"),
        "the ordinal follows source order",
    );

    // Lines above the file's first site, and lines above a site inside its own function.
    let moved = format!(
        "// a comment that was not here\n\nfn a_function_added_above() -> usize {{\n    1\n}}\n{}",
        TWICE.replace("    if first {", "    // pushed down\n\n\n    if first {"),
    );
    assert_ne!(moved, TWICE);
    let after = scan_one(&moved);
    assert_eq!(ids(&after), ids(&before));
    assert_eq!(
        refusal_sites::render(&after),
        refusal_sites::render(&before),
        "no column of a row holds a line, so the committed list does not move either",
    );
}

/// The function of an id is the reader's `enclosing_fn` — the last `fn <name>` before the
/// site, found in text. On this tree every one of them is a function its file declares; a
/// name that is none (`for check_fn in …` would read as a function called `in`) reddens
/// here rather than standing in a committed id.
#[test]
fn on_this_tree_every_site_stands_in_a_function_its_file_declares() {
    let code_of: BTreeMap<String, rust_source::CodeOnly> = refusal_sites::tree_sources()
        .into_iter()
        .map(|source| {
            let (code, _) = refusal_sites::production_code(&source.body);
            (source.path, code)
        })
        .collect();
    for site in refusal_sites::scan_tree() {
        let code = &code_of[&site.file];
        let declared = code
            .match_indices(&format!("fn {}", site.function))
            .any(|(at, head)| code[at + head.len()..].starts_with(['(', '<']));
        assert!(
            declared,
            "`{}`: {} declares no `fn {}` — the id names a function that is not one",
            site.id(),
            site.file,
            site.function,
        );
    }
}

// ---------------------------------------------------------------------------
// 5 — every call is a row
// ---------------------------------------------------------------------------

#[test]
fn a_dynamic_site_is_listed_unassigned_and_never_dropped() {
    // In a fixture: the dynamic sites are rows, beside the assigned ones.
    let sites = scan_one(CODES);
    assert_eq!(sites.len(), 7, "one row per call: {:?}", ids(&sites));
    for function in [
        "by_a_constant_nothing_defines",
        "by_a_method",
        "by_a_parameter",
        "by_a_built_string",
    ] {
        let site = in_fn(&sites, function);
        assert_eq!((site.code.as_str(), site.command), (DYNAMIC, UNASSIGNED));
    }
    assert_eq!(in_fn(&sites, "by_literal").command, "setup");

    // On this tree: as many rows as production code holds constructor calls — counted here
    // from the reader, not taken from the scan — and the two words go together.
    let mut calls = 0usize;
    for source in refusal_sites::tree_sources() {
        let (code, regions) = refusal_sites::production_code(&source.body);
        let path = std::path::Path::new(&source.path);
        for constructor in CONSTRUCTORS {
            calls += code
                .match_indices(constructor)
                .filter(|(at, _)| !rust_source::is_test_domain(path, &regions, *at))
                .count();
        }
    }
    let tree = refusal_sites::scan_tree();
    let called = tree.iter().filter(|s| s.form != "literal").count();
    assert!(
        calls >= SITE_FLOOR,
        "only {calls} constructor calls counted"
    );
    assert_eq!(
        called, calls,
        "production code holds {calls} `Finding` constructor calls and the scan lists \
         {called} of them — a call is a row whatever its code is",
    );

    let dynamic: Vec<&Site> = tree.iter().filter(|s| s.code == DYNAMIC).collect();
    assert!(
        !dynamic.is_empty(),
        "this tree mints codes through `code()` methods and door tables; a scan that lists no \
         dynamic site dropped them",
    );
    for site in &tree {
        assert_eq!(
            site.code == DYNAMIC,
            site.command == UNASSIGNED,
            "`{}`: a dynamic site is unassigned, and only a dynamic site is",
            site.id(),
        );
    }
}

// ---------------------------------------------------------------------------
// 6 — the cross-check against the one family that had a scanner
// ---------------------------------------------------------------------------

/// The `<crate>::<module>` a production path denotes, as `FINALIZE_FAMILY` names a producer.
fn module_of(file: &str) -> String {
    let (krate, rest) = if let Some(rest) = file.strip_prefix("crates/engine/src/") {
        ("engine", rest)
    } else if let Some(rest) = file.strip_prefix("crates/cli/src/") {
        ("cli", rest)
    } else {
        panic!("`{file}` is under neither crate's `src/`")
    };
    format!(
        "{krate}::{}",
        rest.trim_end_matches(".rs").replace('/', "::")
    )
}

#[test]
fn on_this_tree_the_finalize_namespace_is_the_registrys() {
    const NAMESPACE: &str = "finalize.";

    let scanned: BTreeSet<(String, String)> = refusal_sites::scan_tree()
        .into_iter()
        .filter(|site| site.code.starts_with(NAMESPACE))
        .map(|site| (site.code.clone(), module_of(&site.file)))
        .collect();

    // The registry's members that no call spells: the committing doors' rejections and the
    // raced rollbacks' conflicts are minted by one seam each, the code arriving from the
    // door's own row. The registry's fence completes its scan from those two tables
    // (`finalize_family_registry.rs`, arm 1); here they are what is taken away, so that what
    // is left is exactly what a call spells — and those seams are rows of the list all the
    // same, dynamic and unassigned.
    let from_a_door: BTreeSet<(String, String)> = cli::invocation_log::COMMITTING_DOORS
        .iter()
        .map(|door| (door.error_code.to_string(), "cli::render".to_string()))
        .chain(
            cli::rollback::ROLLBACK_DOORS
                .iter()
                .map(|door| (door.code.to_string(), "cli::rollback".to_string())),
        )
        .collect();
    let spelled: BTreeSet<(String, String)> = cli::render::FINALIZE_FAMILY
        .iter()
        .map(|member| (member.code.to_string(), member.producer.to_string()))
        .filter(|pair| !from_a_door.contains(pair))
        .collect();

    assert!(
        spelled.len() >= 10 && spelled.len() < cli::render::FINALIZE_FAMILY.len(),
        "the registry must hold both kinds of member for this to compare anything: {} of {} \
         are spelled at a call",
        spelled.len(),
        cli::render::FINALIZE_FAMILY.len(),
    );
    assert_eq!(
        scanned, spelled,
        "the scan's `{NAMESPACE}` sites whose code is spelled at the call must be the members \
         of `cli::render::FINALIZE_FAMILY` that a call spells, code and producing module — \
         two scanners over one tree that disagree means one of them misreads it",
    );
}

// ---------------------------------------------------------------------------
// What the six lean on
// ---------------------------------------------------------------------------

const AS_WRITTEN: &str = r#"
fn a_bare_string() -> Finding {
    Finding::block("setup.a", "a, b (c", "re-run `jigc setup`, then look")
}

fn a_format(path: &str) -> Finding {
    Finding::block(
        "setup.b",
        format!("cannot write `{path}`"),
        format!(
            "ensure `{path}` is writable, \
             then re-run `jigc setup`"
        ),
    )
}

fn an_advisory_without_a_route() -> Finding {
    Finding::graded(Severity::Advisory, "setup.c", "m", None, None)
}

fn a_severity_handed_in(severity: Severity) -> Finding {
    Finding::graded(severity, "setup.d", "m", None, Some(Route::human("ask the owner")))
}

fn a_command_line(addr: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "setup.e",
        "m",
        Some(Location::at(1, 1)),
        Some(engine::finding::Route::mechanical(
            ["jigc", "doc", "show", addr],
            " to see it",
        )),
    )
}

fn a_warning_that_directs_nothing() -> Finding {
    Finding::graded(
        Severity::Warning,
        "setup.f",
        "m",
        None,
        Some(Route::informational("nothing to do")),
    )
}

fn a_route_built_elsewhere(route: String) -> Finding {
    Finding::block("setup.g", "m", route)
}

fn a_parser_diagnostic(location: Location) -> Finding {
    Finding::blocking("conformance.h", "m", location)
}

fn a_regraded_copy(path: &str) -> Finding {
    Finding {
        severity: Severity::Advisory,
        route: Some(crate::finding::Route::informational("absorbed at the next finalize")),
        ..drift(path)
    }
}

fn a_pattern_is_no_site(finding: &Finding) -> bool {
    let Finding { severity, .. } = finding;
    matches!(finding, Finding { severity: Severity::Blocking, .. }) && severity.blocks()
}
"#;

#[test]
fn a_sites_severity_and_route_are_read_as_written() {
    let sites = scan_one(AS_WRITTEN);
    let read = |function: &str| {
        let site = in_fn(&sites, function);
        (
            site.form,
            site.severity,
            site.route,
            site.route_text.clone(),
        )
    };
    let row = |form, severity, route, text: &str| (form, severity, route, text.to_string());

    assert_eq!(
        read("a_bare_string"),
        row(
            "block",
            "blocking",
            "prose",
            "re-run `jigc setup`, then look"
        ),
        "a comma or a parenthesis inside a string splits no argument",
    );
    assert_eq!(
        read("a_format"),
        row(
            "block",
            "blocking",
            "prose",
            "ensure `{path}` is writable, then re-run `jigc setup`",
        ),
        "a route's text is the literal's value — a continuation joined, a template not filled",
    );
    assert_eq!(
        read("an_advisory_without_a_route"),
        row("graded", "advisory", "none", ""),
    );
    assert_eq!(
        read("a_severity_handed_in"),
        row("graded", DYNAMIC, "human", "ask the owner"),
    );
    assert_eq!(
        read("a_command_line"),
        row(
            "graded",
            "blocking",
            "mechanical",
            "jigc doc show  to see it"
        ),
        "a command line's text is its literals in order: the operand is not at the call",
    );
    assert_eq!(
        read("a_warning_that_directs_nothing"),
        row("graded", "warning", "informational", "nothing to do"),
    );
    assert_eq!(
        read("a_route_built_elsewhere"),
        row("block", "blocking", "variable", ""),
    );
    assert_eq!(
        read("a_parser_diagnostic"),
        row("blocking", "blocking", "none", ""),
        "`Finding::blocking` takes no route",
    );

    let regraded = in_fn(&sites, "a_regraded_copy");
    assert_eq!(
        (regraded.form, regraded.severity, regraded.route),
        ("literal", "advisory", "informational"),
    );
    assert_eq!(
        (regraded.code.as_str(), regraded.written.as_str()),
        (DYNAMIC, "..drift(path)"),
        "a literal that takes its code from a base expression is a dynamic site",
    );

    assert!(
        sites.iter().all(|s| s.function != "a_pattern_is_no_site"),
        "a `Finding {{ … }}` pattern destructures a finding and constructs none: {:?}",
        ids(&sites),
    );
    assert_eq!(sites.len(), 9);
}

#[test]
#[should_panic(expected = "no row for the code prefix `brand-new`")]
fn a_code_whose_prefix_the_family_table_does_not_hold_is_refused() {
    scan_one("fn mint() -> Finding {\n    Finding::block(\"brand-new.thing\", \"m\", \"r\")\n}\n");
}

/// A row no site of this tree can be read against is a claim about a family nobody can
/// check — the families that are built through a helper (`rename`, `relocate`, `repo`) have
/// no row for that reason, and get one when a call first spells one of their codes.
#[test]
fn every_row_of_the_family_table_assigns_a_site_of_this_tree() {
    let tree = refusal_sites::scan_tree();
    let spelled: BTreeSet<&str> = tree
        .iter()
        .filter(|site| site.code != DYNAMIC)
        .map(|site| {
            site.code
                .split_once('.')
                .map_or(&*site.code, |(prefix, _)| prefix)
        })
        .collect();

    let prefixes: Vec<&str> = FAMILIES.iter().map(|(prefix, _)| *prefix).collect();
    let mut sorted = prefixes.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(prefixes, sorted, "one row a prefix, sorted by prefix");

    for (prefix, command) in FAMILIES {
        assert!(
            spelled.contains(prefix),
            "`FAMILIES` assigns the prefix `{prefix}` to `{command}`, and no production call \
             spells a `{prefix}.` code — the row assigns nothing",
        );
    }
}

/// A reading that does not model character literals takes a `'"'` for the start of a
/// string that the next real string closes — so everything between, a constructor call
/// included, reads as prose. Three production files hold one. The shared reader models
/// them (`rust_source_reader.rs` holds that); this holds that the scan reads through it.
const AFTER_A_QUOTE: &str = r#"
fn unquoted(s: &str) -> &str {
    s.trim_matches('"')
}

fn after_the_quote() -> Finding {
    Finding::block("setup.after-a-quote", "m", "r")
}

#[cfg(test)]
mod tests {
    fn a_fixture() {
        let _ = Finding::block("setup.in-a-test", "m", "r");
    }
}
"#;

#[test]
fn a_site_after_a_quote_character_literal_is_still_listed() {
    let sites = scan_one(AFTER_A_QUOTE);
    assert_eq!(codes(&sites), ["setup.after-a-quote"]);
}
