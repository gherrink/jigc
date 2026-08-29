//! M49 Increment 5, T3 — **the address-parse fault axis, one row per `ParseError` variant.**
//!
//! The class: *whose fault a malformed address reports, and where it sends the caller*.
//! At HEAD `doc.rs`'s `parse_verb_addr` flattened all six
//! [`engine::address::ParseError`] variants into one sentence — *a doc is addressed as
//! `<type>:<slug>`* — and one route, `jigc describe`. For the three **head** faults
//! (`EmptyType` · `MissingColon` · `EmptySlug`) that is true and followable: the caller
//! got `<type>:<slug>` wrong, and the doctype surface is what answers. For the three
//! **fragment** faults (`EmptyFragment` · `EmptyHop` · `TooManyHops`) the head is already
//! correct, so the sentence describes a part of the address the caller typed properly and
//! `jigc describe` — which lists doctypes, never addresses — answers nothing. `TooManyHops`
//! was the sharpest: the fault is **depth**, and no word of the message named it
//! (`DECISIONS.md` → 2026-08-29 M49 Increment 5 planning, the driven basis).
//!
//! So the axis splits 3/3 and this suite enumerates it as **data**, one row per variant:
//!
//!   * a **head** fault keeps today's `<type>:<slug>` sentence and the `jigc describe`
//!     route, unchanged where it is true;
//!   * a **fragment** fault gets the fragment grammar and routes at
//!     `jigc doc schema <type>` — the surface that, after T1, lists only addresses the
//!     write path accepts (`design/surface-contract.md` → law 1 + the route floor;
//!     `design/validation.md` → the `write.*` route split; the M47 write-verb × miss-shape
//!     precedent, which is this table's shape).
//!
//! Every row is driven through a **real `doc` verb on the real binary**, and asserts: the
//! call blocks non-zero; the message names *this* fault; the surface carries **exactly one**
//! `route:` line and it is the expected one; no fragment fault routes at `jigc describe`;
//! and — for the fragment half — the emitted route **runs verbatim at exit 0**, so it is
//! followable and not merely well-worded. Across the six rows the whole rendered surface is
//! **pairwise distinct**: six faults, six answers.
//!
//! **The table is bound to the enum, not to a remembered list.** [`variant_name`] matches
//! `ParseError` **exhaustively**, so a seventh variant cannot compile without an author
//! coming here; and every row asserts that its address really provokes the variant it
//! claims, through `engine::address::Address::parse` itself. The producer side carries the
//! same fence: `doc.rs`'s `address_parse_guidance` is an exhaustive match, so a seventh
//! variant cannot compile without a message **and** a route.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use engine::address::{Address, MAX_FRAGMENT_HOPS, ParseError};
use engine::schema::MAX_NESTING_DEPTH;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-address-parse-axis-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A real git repo with one commit plus the `.jigc/config/` project layer `jigc doc show`
/// locates before it adjudicates an address.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// The held fixture — the temp guards must outlive the runs.
struct Fixture {
    repo: TempDir,
    home: TempDir,
}

impl Fixture {
    fn new() -> Self {
        let repo = TempDir::new("repo");
        let home = TempDir::new("home");
        init_repo(repo.path());
        Fixture { repo, home }
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("spawn the jigc binary")
    }
}

/// Which half of the axis a fault sits on — *what the caller got wrong*.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Half {
    /// The `<type>:<slug>` head.
    Head,
    /// The `#…` fragment, over a head that already parsed.
    Fragment,
}

/// One cell: a variant, an address that provokes it through a real verb, the fault clause
/// the message must name, and the route the surface must carry.
struct Cell {
    variant: ParseError,
    address: &'static str,
    fault: &'static str,
    half: Half,
}

/// The axis — six rows, one per [`ParseError`] variant (the count is asserted against the
/// enum in [`the_table_covers_every_parse_error_variant`]).
fn axis() -> Vec<Cell> {
    vec![
        Cell {
            variant: ParseError::EmptyType,
            address: ":pick-a-db",
            fault: "empty type",
            half: Half::Head,
        },
        Cell {
            variant: ParseError::MissingColon,
            address: "pick-a-db",
            fault: "missing ':' between type and slug",
            half: Half::Head,
        },
        Cell {
            variant: ParseError::EmptySlug,
            address: "adr:",
            fault: "empty slug",
            half: Half::Head,
        },
        Cell {
            variant: ParseError::EmptyFragment,
            address: "adr:pick-a-db#",
            fault: "empty fragment",
            half: Half::Fragment,
        },
        Cell {
            variant: ParseError::EmptyHop,
            address: "adr:pick-a-db#status//date",
            fault: "empty fragment hop",
            half: Half::Fragment,
        },
        Cell {
            variant: ParseError::TooManyHops,
            address: "adr:pick-a-db#a/b/c/d/e/f/g",
            fault: "too many fragment hops",
            half: Half::Fragment,
        },
    ]
}

/// The enum-completeness fence: an **exhaustive** match over `ParseError`, so a seventh
/// variant cannot compile without an author coming to this suite — the test-side mirror of
/// the producer's own exhaustive mapping.
fn variant_name(err: &ParseError) -> &'static str {
    match err {
        ParseError::EmptyType => "EmptyType",
        ParseError::MissingColon => "MissingColon",
        ParseError::EmptySlug => "EmptySlug",
        ParseError::EmptyFragment => "EmptyFragment",
        ParseError::EmptyHop => "EmptyHop",
        ParseError::TooManyHops => "TooManyHops",
    }
}

/// The `route: …` lines of a rendered surface.
fn route_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("route:"))
        .map(|line| line.trim_start_matches("route:").trim().to_owned())
        .collect()
}

/// Every row's address really provokes the variant it claims — the table is bound to the
/// grammar, not to a remembered list — and the table covers the enum.
#[test]
fn the_table_covers_every_parse_error_variant() {
    let axis = axis();
    let mut seen: Vec<&'static str> = Vec::new();
    for cell in &axis {
        let err = Address::parse(cell.address).expect_err("the row's address must not parse");
        assert_eq!(
            err,
            cell.variant,
            "`{}` provokes {} — the row claims {}",
            cell.address,
            variant_name(&err),
            variant_name(&cell.variant),
        );
        seen.push(variant_name(&cell.variant));
    }
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(
        seen,
        [
            "EmptyFragment",
            "EmptyHop",
            "EmptySlug",
            "EmptyType",
            "MissingColon",
            "TooManyHops",
        ],
        "the axis must hold one row per `ParseError` variant",
    );
}

/// **The axis.** Each variant, driven through a real `doc` verb on the real binary: one
/// distinct message and exactly one route per variant, the head half keeping
/// `jigc describe` and the fragment half routing at `jigc doc schema <type>` — never at
/// `jigc describe`.
#[test]
fn each_parse_error_names_its_own_fault_and_routes_where_the_answer_is() {
    let fixture = Fixture::new();
    let mut surfaces: Vec<(&'static str, String)> = Vec::new();

    for cell in axis() {
        let name = variant_name(&cell.variant);
        let out = fixture.run(&["doc", "show", cell.address]);
        assert!(
            !out.status.success(),
            "{name}: a malformed address must block non-zero",
        );
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

        assert!(
            stderr.contains(&format!(
                "malformed address `{}`: {}",
                cell.address, cell.fault
            )),
            "{name}: the message must name this fault\n{stderr}",
        );

        let routes = route_lines(&stderr);
        assert_eq!(
            routes.len(),
            1,
            "{name}: the surface must carry exactly one route\n{stderr}",
        );
        let route = &routes[0];

        match cell.half {
            Half::Head => assert_eq!(
                route, "run `jigc describe` for the doctype surface",
                "{name}: a head fault keeps the doctype surface",
            ),
            Half::Fragment => {
                assert!(
                    !route.contains("jigc describe"),
                    "{name}: a fragment fault must not route at `jigc describe` — the head \
                     already parsed and that surface lists doctypes, not addresses\n{route}",
                );
                assert!(
                    route.contains("`jigc doc schema adr`"),
                    "{name}: a fragment fault routes at the schema surface, naming the \
                     doctype the address already carries\n{route}",
                );
                // Followable, not merely well-worded: the emitted command runs verbatim.
                let schema = fixture.run(&["doc", "schema", "adr"]);
                assert!(
                    schema.status.success(),
                    "{name}: the emitted route must run at exit 0\n{}",
                    String::from_utf8_lossy(&schema.stderr),
                );
            }
        }

        surfaces.push((name, stderr));
    }

    for (i, (name, text)) in surfaces.iter().enumerate() {
        for (other, other_text) in surfaces.iter().skip(i + 1) {
            assert_ne!(
                text, other_text,
                "{name} and {other} render the same surface — six faults, six answers",
            );
        }
    }
}

/// `TooManyHops` states the **depth budget**, not a generic "malformed": the hop ceiling and
/// the nesting depth it implies, both read from the constants T1 pinned so the sentence
/// cannot go stale the day the grammar moves.
#[test]
fn the_hop_cap_reject_names_the_depth_budget() {
    let fixture = Fixture::new();
    let out = fixture.run(&["doc", "show", "adr:pick-a-db#a/b/c/d/e/f/g"]);
    assert!(!out.status.success(), "an over-budget fragment must block");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(&format!("{MAX_FRAGMENT_HOPS} hops")),
        "the reject must name the hop budget ({MAX_FRAGMENT_HOPS})\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("{MAX_NESTING_DEPTH} nesting levels")),
        "the reject must name the depth the budget buys ({MAX_NESTING_DEPTH})\n{stderr}",
    );
}

/// The doctype hop is **user bytes**, and a mechanical route's text is `argv.join(" ")` —
/// bytes an agent pastes into a shell. A head that re-lexes as a **flag** (reachable through
/// clap's `--`) must still emit a route that runs as printed, rather than firing the route
/// fence's argv assert.
#[test]
fn a_flag_shaped_doctype_hop_still_emits_a_runnable_route() {
    let fixture = Fixture::new();
    let out = fixture.run(&["doc", "show", "--", "-x:pick-a-db#"]);
    assert!(!out.status.success(), "the address must block");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let routes = route_lines(&stderr);
    assert_eq!(routes.len(), 1, "exactly one route\n{stderr}");
    assert!(
        routes[0].contains("`jigc doc schema -- -x`"),
        "the flag-shaped hop is placed after clap's end-of-flags separator, so the emitted \
         line runs as printed\n{}",
        routes[0],
    );
}
