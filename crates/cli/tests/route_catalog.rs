//! **The route catalog** — one case per refusal site and cause: the state that provokes the
//! refusal, the refusing command, the route lifted from the printed bytes and followed, and
//! what must hold afterwards (`DECISIONS.md` → 2026-10-10, ruling 3; the plan's step `P1`,
//! `completions/artifacts/M55/stabilization-build/reshape/plan.md`).
//!
//! The exit rule says *every refusal's route works as printed*, and until this suite
//! nothing ran a `setup` route at all. The frame is
//! [`support::route_case`](crate::support::route_case), which says what a case is and what
//! the runner does; this file is the **expected-red list** ([`EXPECTED_RED`]) and the arms
//! that hold the frame to its rules. **A command's cases are a file of their own under
//! `route_catalog/`**, declared here by `#[path]` — `setup.rs` today. They stand there, and
//! not beside this file, because that directory also holds the committed list of refusal
//! sites: a suite file beside a directory of its own name makes the directory an
//! aggregation for the registration fence (`test_target_registration.rs`), which holds
//! every `.rs` file in it to a declaration here and refuses a directory that has none.
//! Neither this file nor a command's acts on a file by its own hand: every act of a case
//! goes through the runner, which takes its operand from the printed route.
//!
//! **A red case is a finding, not a failure to hide.** A case that comes out red is listed
//! in [`EXPECTED_RED`] with the key of the finding it reproduces and the classification it
//! is red with, and the list is strict three ways
//! ([`the_catalog_is_followed_and_the_expected_red_list_is_exact`]): a listed case that is
//! no longer red fails, a red case that is not listed fails, and a listed case whose
//! classification moved fails. So the gate is green over a known defect, the defect is on
//! record under its key, and the fix that turns the case removes the entry in the same
//! commit — nothing here is fixed, and nothing here asserts a route's wording.
//!
//! **Before a stabilization run is opened, a key certifies itself**: it is the key the
//! small canary gave the finding
//! (`completions/artifacts/M55/stabilization-build/canary-one/findings.md`), and no ledger
//! holds it yet. The tie becomes a real one when a run's opening seeds its rows.
//!
//! **What the catalog holds today** is the frame's first proof: the one site whose cause an
//! existing test planted and passed over, in the cause that destroys a file and in its
//! control. The other sites of `setup` and `uninstall`, and the fence that names a site
//! without a case, are the tasks after this one.

use std::path::Path;

use crate::support::refusal_sites::{self, Site};
use crate::support::route_case::{
    self, Act, Case, Classification, ExpectedRed, Followed, Operand, Refusal, Seen,
};
use crate::support::rust_source;

#[path = "route_catalog/setup.rs"]
mod setup;

use setup::{CASES, HOOK, HOOK_LINK_SITE};

/// The cases that are red, each under the key of the finding it reproduces. The one key
/// here is the first row of §1, *The two blockers*, of
/// `completions/artifacts/M55/stabilization-build/canary-one/findings.md`.
const EXPECTED_RED: &[ExpectedRed] = &[ExpectedRed {
    case: "setup-hook-linked-to-a-script-outside",
    key: "r1-setup-install-hook-route-other-causes-unexamined",
    classification: Classification::WitnessChanged,
}];

fn binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_jigc"))
}

// ---------------------------------------------------------------------------
// The catalog
// ---------------------------------------------------------------------------

/// **Every case is followed on the binary this suite was built with, and the expected-red
/// list is exactly the cases that came out red.** One arm, every difference: a case that
/// cannot be followed as written, a red case no entry lists, a listed case that is not
/// red, and a listed case whose classification moved are all named before it fails.
#[test]
fn the_catalog_is_followed_and_the_expected_red_list_is_exact() {
    let followed: Vec<Result<Followed, String>> = std::thread::scope(|scope| {
        let arms: Vec<_> = CASES
            .iter()
            .map(|case| scope.spawn(move || route_case::follow(binary(), case)))
            .collect();
        arms.into_iter()
            .map(|arm| arm.join().expect("a case's runner panicked"))
            .collect()
    });

    let mut wrong = Vec::new();
    let mut results = Vec::new();
    let mut seen = Vec::new();
    for (case, outcome) in CASES.iter().zip(&followed) {
        match outcome {
            Ok(followed) => {
                results.push((case.name, followed.classification));
                seen.push(format!(
                    "`{}` ({}) — {}\n{}",
                    case.name, case.cause, followed.classification, followed.detail
                ));
            }
            Err(fault) => wrong.push(format!(
                "`{}` cannot be followed as written: {fault}",
                case.name
            )),
        }
    }
    wrong.extend(route_case::differences(EXPECTED_RED, &results));
    assert!(
        wrong.is_empty(),
        "the route catalog and its expected-red list disagree:\n\n  {}\n\nwhat each case \
         did:\n\n{}",
        wrong.join("\n  "),
        seen.join("\n\n"),
    );
}

/// A case names its site as the committed list of refusal sites does, and its own name
/// once — so a renamed producer, or a site that is gone, is named here and a case never
/// outlives what it is about.
#[test]
fn every_case_names_a_site_of_the_list_and_has_a_name_of_its_own() {
    let sites: Vec<String> = refusal_sites::scan_tree().iter().map(Site::id).collect();
    let mut wrong = Vec::new();
    for (at, case) in CASES.iter().enumerate() {
        if !sites.iter().any(|site| site == case.site) {
            wrong.push(format!(
                "`{}` names the site `{}`, which the list does not hold",
                case.name, case.site
            ));
        }
        if CASES[..at].iter().any(|other| other.name == case.name) {
            wrong.push(format!("two cases are named `{}`", case.name));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

// ---------------------------------------------------------------------------
// The frame's rules
// ---------------------------------------------------------------------------

/// The body of the item `opening` opens in `code` — between its brace and the matching
/// one.
fn body_of<'a>(code: &'a str, opening: &str) -> &'a str {
    let start = code
        .find(opening)
        .unwrap_or_else(|| panic!("the frame declares `{opening}`"))
        + opening.len();
    let mut depth = 1;
    for (at, byte) in code[start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return &code[start..start + at];
        }
    }
    panic!("`{opening}` is never closed");
}

/// The variants of an enum body, as written.
fn variants(body: &str) -> Vec<String> {
    body.split(',')
        .map(|variant| variant.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|variant| !variant.is_empty())
        .collect()
}

/// **No act takes a path, and the marked form is refused where a path is printed.**
///
/// The source half reads the frame and this file. An [`Act`] carries an [`Operand`] or a
/// span's index and nothing else; an operand is a span's index or the one marked form; a
/// [`Case`] holds no function, so it cannot act around the runner; and this file names no
/// filesystem and no process API at all — what the existing test did by hand
/// (`fs::remove_file` on a path it spelled) has no place to be written.
///
/// The planted half is that test's step, as a case: the first case's state, with the link's
/// own path given by a constant. The printed route holds a path, so the runner refuses the
/// case before it does anything — with that refusal switched off it comes out *followed,
/// holds, operand implied*, which is the green the existing test has.
#[test]
fn no_act_takes_a_path_and_the_marked_form_is_refused_where_a_path_is_printed() {
    let frame = rust_source::code_only(include_str!("support/route_case.rs"));
    for variant in variants(body_of(&frame, "pub enum Act {")) {
        assert!(
            variant.ends_with("(Operand)") || variant.ends_with("(usize)"),
            "the act `{variant}` takes something that is neither an operand nor a span's index",
        );
    }
    assert_eq!(
        variants(body_of(&frame, "pub enum Operand {")),
        ["Span(usize)", "Unprinted(&'static str)"],
        "an operand is a span of the printed route by index, or the one marked form",
    );
    let case = body_of(&frame, "pub struct Case {");
    for callable in ["fn(", "Fn", "dyn ", "Box<"] {
        assert!(
            !case.contains(callable),
            "a case is data: `{callable}` in `Case` lets one act around the runner",
        );
    }
    let suite = rust_source::code_only(concat!(
        include_str!("route_catalog.rs"),
        include_str!("route_catalog/setup.rs"),
    ));
    for by_hand in ["fs::", "std::os", "std::process", "Command", "symlink"] {
        assert!(
            !suite.contains(by_hand),
            "the catalog names `{by_hand}`: a case's every act goes through the runner, \
             which takes its operand from the printed route",
        );
    }

    let planted = Case {
        name: "planted-the-links-own-path-by-a-constant",
        acts: &[Act::Remove(Operand::Unprinted(HOOK)), Act::Run(1)],
        ..CASES[0]
    };
    match route_case::follow(binary(), &planted) {
        Err(fault) => assert!(
            fault.contains("the marked form") && fault.contains("span 0"),
            "the refusal names the marked form and the path span that forbids it: {fault}",
        ),
        Ok(followed) => panic!(
            "a case that names the link's own path by a constant was followed, where the \
             printed route names a path: {}\n{}",
            followed.classification, followed.detail
        ),
    }
}

/// **An act is not done outside the sandbox.** The runner does what a route says with a
/// path the product printed, so it must not follow one out of the case's own scratch root —
/// a route that named `/dev/null` as the link to remove is on the small canary's list. A
/// shell stands in for the binary here (the frame names none of its own) and prints a
/// refusal whose route names a path outside the sandbox twice, once absolute and once by
/// climbing out: neither is touched, and the case says why.
#[test]
fn an_act_is_not_done_outside_the_sandbox() {
    let planted = Case {
        name: "planted-a-route-that-names-paths-outside-the-sandbox",
        site: HOOK_LINK_SITE,
        cause: "a refusal whose route names `/dev/null` and a path above the sandbox",
        state: &[],
        refusing: &[
            "-c",
            "printf '%s\\n' 'blocking · setup.install-hook — refused' \
             '  route: remove `/dev/null` and `../../elsewhere`'; exit 1",
        ],
        acts: &[Act::Remove(Operand::Span(0)), Act::Remove(Operand::Span(1))],
        holds: &[],
        witnesses: &[],
    };
    let followed = route_case::follow(Path::new("/bin/sh"), &planted)
        .expect("the planted case is one the runner follows");
    assert_eq!(
        (
            followed.classification,
            followed.detail.matches("not done").count()
        ),
        (Classification::AnotherFailure, 2),
        "both acts are refused, and the case is red for it:\n{}",
        followed.detail,
    );
}

/// A span is a path when it names an entry that is there, or is written as one; a flag, a
/// command line and a bare word are not. The route of the hook's write failure prints no
/// path at all — the marked form may stand there.
#[test]
fn a_span_is_a_path_when_it_names_an_entry_or_is_written_as_one() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let route = "remove `Cargo.toml` or `tests/no-such-file.rs`, not `no-such-entry`, then \
                 re-run `jigc setup` — `--force` changes nothing, nor does `/`, and this \
                 `backtick is not closed";
    let spans = route_case::spans(route);
    assert_eq!(
        spans,
        [
            "Cargo.toml",
            "tests/no-such-file.rs",
            "no-such-entry",
            "jigc setup",
            "--force",
            "/",
        ],
        "the spans are the pieces between paired backticks, in order",
    );
    assert_eq!(
        route_case::path_spans(&spans, here),
        [0, 1, 5],
        "an entry that is there, a word written as a path, an absolute path — and no other",
    );
    let writability = "ensure the repo's git hooks directory is writable, then re-run `jigc setup`";
    assert!(
        route_case::path_spans(&route_case::spans(writability), here).is_empty(),
        "a route that prints no path leaves the marked form standing",
    );
}

/// The route is the one of the finding that carries the case's code — not the first route
/// printed, and not the next finding's where its own finding prints none.
#[test]
fn a_route_is_lifted_from_the_finding_that_carries_the_code_and_from_no_other() {
    let said = "\
advisory · setup.other — something else
  route: do the other thing
blocking · setup.routeless — refused, and no way out is printed
blocking (gates at finalize) · setup.wanted — refused over `a/b`
  at: a/b
  route: remove `a/b`, then re-run `jigc setup`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";
    let route = |code| route_case::refusal_in(said, code).map(|refusal: Refusal| refusal.route);
    assert_eq!(
        route("setup.wanted"),
        Some(Some("remove `a/b`, then re-run `jigc setup`".to_string())),
    );
    assert_eq!(
        route("setup.other"),
        Some(Some("do the other thing".to_string()))
    );
    assert_eq!(
        route("setup.routeless"),
        Some(None),
        "a finding that prints no route has none — the next finding's is not its route",
    );
    assert_eq!(route("setup.absent"), None);
    assert_eq!(route("setup"), None, "a code is matched whole");
}

/// The seven classifications, and the order they are read in: nothing is *followed* that
/// was not provoked or printed no route; a changed witness outranks a repeated refusal,
/// which outranks any other failure; and the marked form holds only as *operand implied*.
#[test]
fn the_seven_classifications_in_their_order() {
    let provoked = Seen {
        provoked: true,
        ..Seen::default()
    };
    let followed = Seen {
        route_printed: true,
        ..provoked
    };
    let everything = Seen {
        witness_changed: true,
        same_refusal: true,
        failed: true,
        implied: true,
        ..followed
    };
    let table = [
        (
            Seen {
                provoked: false,
                ..everything
            },
            Classification::NotProvoked,
            "not provoked",
        ),
        (
            Seen {
                route_printed: false,
                ..everything
            },
            Classification::NoRoutePrinted,
            "provoked, no route printed",
        ),
        (
            everything,
            Classification::WitnessChanged,
            "followed, a witness changed",
        ),
        (
            Seen {
                witness_changed: false,
                ..everything
            },
            Classification::SameRefusalAgain,
            "followed, the same refusal again",
        ),
        (
            Seen {
                failed: true,
                implied: true,
                ..followed
            },
            Classification::AnotherFailure,
            "followed, another failure",
        ),
        (
            Seen {
                implied: true,
                ..followed
            },
            Classification::HoldsOperandImplied,
            "followed, holds, operand implied",
        ),
        (followed, Classification::Holds, "followed, holds"),
    ];
    for (seen, classification, words) in table {
        assert_eq!(route_case::classify(&seen), classification, "{seen:?}");
        assert_eq!(classification.as_str(), words);
        assert_eq!(
            classification.is_red(),
            !words.starts_with("followed, holds"),
            "`{words}`: only a route that holds is green",
        );
    }
}

/// The expected-red list is strict three ways — and says every difference at once.
#[test]
fn the_expected_red_list_is_strict_three_ways() {
    let entry = |case, classification| ExpectedRed {
        case,
        key: "r1-a-key",
        classification,
    };
    let listed = [entry("destroys", Classification::WitnessChanged)];
    let exact = [
        ("destroys", Classification::WitnessChanged),
        ("works", Classification::Holds),
        ("works-by-hand", Classification::HoldsOperandImplied),
    ];
    assert_eq!(
        route_case::differences(&listed, &exact),
        Vec::<String>::new(),
        "a red case listed with its classification, and two green ones: no difference",
    );

    let one = |expected: &[ExpectedRed], results: &[(&str, Classification)], says: &str| {
        let differences = route_case::differences(expected, results);
        assert_eq!(differences.len(), 1, "{says}: {differences:?}");
        assert!(differences[0].contains(says), "{says}: {differences:?}");
    };
    one(
        &listed,
        &[("destroys", Classification::Holds)],
        "is not red",
    );
    one(
        &[],
        &[("destroys", Classification::WitnessChanged)],
        "no entry lists it",
    );
    one(
        &listed,
        &[("destroys", Classification::SameRefusalAgain)],
        "its classification moved",
    );
    one(&listed, &[], "names a case that was not followed");
    one(
        &[listed[0], listed[0]],
        &[("destroys", Classification::WitnessChanged)],
        "is listed twice",
    );
    one(
        &[entry("works", Classification::Holds)],
        &[("works", Classification::Holds)],
        "which is not red",
    );

    let all = route_case::differences(
        &[
            entry("fixed", Classification::WitnessChanged),
            entry("moved", Classification::WitnessChanged),
        ],
        &[
            ("fixed", Classification::Holds),
            ("moved", Classification::NotProvoked),
            ("new", Classification::AnotherFailure),
        ],
    );
    assert_eq!(all.len(), 3, "every difference is reported: {all:?}");
}
