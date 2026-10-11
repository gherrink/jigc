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
//! `route_catalog/`**, declared here by `#[path]` inside [`tables!`] — `setup.rs` today.
//! They stand there, and not beside this file, because that directory also holds the
//! committed list of refusal sites: a suite file beside a directory of its own name makes
//! the directory an aggregation for the registration fence
//! (`test_target_registration.rs`), which holds every `.rs` file in it to a declaration
//! here and refuses a directory that has none. Neither this file nor a command's acts on a
//! file by its own hand: every act of a case goes through the runner, which takes its
//! operand from the printed route.
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
//! control. Every other blocking site of `setup` and `uninstall` is a row of [`OWED`],
//! under the task that owes its case.
//!
//! **The fence** (the plan's step `P2`) holds the catalog to the refusal sites of the
//! families [`CATALOGUED`] names. It scans nothing for itself — the sites are
//! [`support::refusal_sites`](crate::support::refusal_sites)' scan, which
//! `refusal_site_list.rs` holds byte-equal to the committed list, and the literals are
//! [`support::rust_source`](crate::support::rust_source)'s. Two arms:
//!
//!   * [`every_refusal_site_of_the_catalogued_families_has_a_case_a_reason_or_an_owed_row`]
//!     — a blocking site has exactly one answer: a case in its command's file, a
//!     [`Reason`] there, or a row of [`OWED`].
//!   * [`every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed`] — the list
//!     says where a finding is constructed, and a code that reaches its constructor
//!     through a `match`, a parameter or a table stands in no row of its family. So the
//!     **vocabulary** is held as well: every code-shaped literal of a catalogued prefix
//!     that production source spells is a listed site's code, is claimed by a row of
//!     [`CLAIMS`] for the `dynamic` site that mints it, or has a row of [`NO_CODE`].
//!     **A claim makes a site**: the code at the dynamic site it names owes a case like a
//!     listed one, under the id [`Claim::id`] gives it.
//!
//! and beside them [`a_row_for_a_site_that_is_gone_is_named`], for a table that outlives
//! what it is about; [`every_file_of_the_catalogs_directory_is_a_table_the_suite_reads`],
//! for a command's file that is on disk and not wired in; and
//! [`the_fence_is_not_vacuous`].
//!
//! **What the fence cannot see — declared.** A code is found where one string literal
//! spells it whole: a code built at run time is not found, nor one spelled only outside the
//! Rust source of the two crates. And **a code that is a listed site's is asked no
//! further**, so a dynamic site that mints codes other sites spell too is demanded by no
//! arm. Two such sites stand in `setup.rs`; their rows of [`CLAIMS`] are kept by reading,
//! and the table says which rows those are.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use crate::support::refusal_sites::{self, DYNAMIC, FAMILIES, Site, Source};
use crate::support::route_case::{
    self, Act, Case, Classification, ExpectedRed, Followed, Operand, Refusal, Seen,
};
use crate::support::rust_source;

/// One command's file under `route_catalog/`, as the suite reads it.
struct Table {
    /// The file, from this one's directory.
    file: &'static str,
    /// Its source, for the arm that holds a case to being data.
    source: &'static str,
    cases: &'static [Case],
    reasons: &'static [Reason],
}

/// **A command's file is wired in by its declaration inside this macro's one call, and
/// nowhere else.** The declaration is the `#[path]` line the registration fence reads
/// (`test_target_registration.rs`), and the macro makes the same line the file's row of
/// [`TABLES`] — its cases, its reasons and its source — so no arm of this suite names a
/// command's file a second time. The file exports `CASES` and `REASONS`.
macro_rules! tables {
    ($(#[path = $file:literal] mod $name:ident;)*) => {
        $(
            #[path = $file]
            mod $name;
        )*

        /// Every command's file, in the order declared.
        const TABLES: &[Table] = &[$(Table {
            file: $file,
            source: include_str!($file),
            cases: $name::CASES,
            reasons: $name::REASONS,
        }),*];
    };
}

tables! {
    #[path = "route_catalog/setup.rs"]
    mod setup;
}

use setup::{HOOK, HOOK_LINK_SITE};

/// Every case of every command's file.
fn cases() -> Vec<&'static Case> {
    TABLES.iter().flat_map(|table| table.cases).collect()
}

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
    let cases = cases();
    let followed: Vec<Result<Followed, String>> = std::thread::scope(|scope| {
        let arms: Vec<_> = cases
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
    for (case, outcome) in cases.iter().zip(&followed) {
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

/// A case names its site as the committed list of refusal sites does — or, for a code a
/// dynamic site mints, as its claim does ([`Claim::id`]) — and its own name once, across
/// every command's file. So a renamed producer, or a site that is gone, is named here and
/// a case never outlives what it is about.
#[test]
fn every_case_names_a_site_of_the_list_and_has_a_name_of_its_own() {
    let mut sites: Vec<String> = refusal_sites::scan_tree().iter().map(Site::id).collect();
    sites.extend(CLAIMS.iter().filter_map(Claim::id));
    let cases = cases();
    let mut wrong = Vec::new();
    for (at, case) in cases.iter().enumerate() {
        if !sites.iter().any(|site| site == case.site) {
            wrong.push(format!(
                "`{}` names the site `{}`, which neither the list nor a claim holds",
                case.name, case.site
            ));
        }
        if cases[..at].iter().any(|other| other.name == case.name) {
            wrong.push(format!("two cases are named `{}`", case.name));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

// ---------------------------------------------------------------------------
// The fence — its tables
// ---------------------------------------------------------------------------

/// **The code prefixes the catalog is held to** — the families of `jigc setup`
/// (`setup`, `adapter-guide`) and of `jigc uninstall`, as
/// [`refusal_sites::FAMILIES`] assigns them. A prefix joins when its command's file does.
const CATALOGUED: &[&str] = &["adapter-guide", "setup", "uninstall"];

/// Why a blocking site has no case. The three words are the plan's, and a reason is one of
/// them by construction.
#[derive(Clone, Copy, Debug)]
pub enum Why {
    /// No state the runner can plant reaches the site. The text says what stands in the
    /// way.
    NotConstructible(&'static str),
    /// The site is reached only on a platform the suite does not run on. The text names it.
    Platform(&'static str),
    /// The refusal prints no route, so there is nothing to follow — taken only where the
    /// list reads no route written at the site's call.
    NoRoutePrinted,
}

impl fmt::Display for Why {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Why::NotConstructible(text) => write!(f, "not-constructible: {text}"),
            Why::Platform(text) => write!(f, "platform: {text}"),
            Why::NoRoutePrinted => f.write_str("no-route-printed"),
        }
    }
}

/// A site, and the reason it has no case. A command's reasons stand in its file beside its
/// cases (`REASONS`): they are what a reader of that file is told instead of a case.
#[derive(Clone, Copy, Debug)]
pub struct Reason {
    /// The site's id, as a case names it.
    pub site: &'static str,
    pub why: Why,
}

/// **A code, and the `dynamic` site that mints it.** The list holds that site under no
/// family, because its code is not spelled at the call; the claim says, by a person's
/// reading, that this code is constructed there.
#[derive(Clone, Copy, Debug)]
struct Claim {
    code: &'static str,
    /// The dynamic site's id as the list holds it: `file::function::dynamic#ordinal`.
    minted_at: &'static str,
}

impl Claim {
    /// The id the claimed site goes by — what a case, a reason or an owed row names it
    /// with: `file::function::<code>#dynamic-<ordinal>`. The code stands where a listed
    /// site's does, so a case reads its code the same way; `dynamic-` keeps the id apart
    /// from every listed one, whose ordinal is a number. `None` where `minted_at` is no
    /// dynamic site's id.
    fn id(&self) -> Option<String> {
        let (function, ordinal) = self.minted_at.split_once(&format!("::{DYNAMIC}#"))?;
        Some(format!("{function}::{}#{DYNAMIC}-{ordinal}", self.code))
    }
}

const fn claim(code: &'static str, minted_at: &'static str) -> Claim {
    Claim { code, minted_at }
}

/// **The codes of the catalogued families that a dynamic site mints.**
///
/// The first five rows are demanded: no listed site carries these three codes, so
/// [`every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed`] names each until a
/// row claims it. The arm asks for one row a code; the second site of `locate_finding` was
/// found by reading it.
///
/// **The other ten are kept by reading, and no arm demands them.** Their codes are listed
/// sites' codes as well, so the vocabulary arm has nothing to ask; what the two functions
/// add is a second refusal under each code, with a route of its own. A row here that
/// outlives its site or its code is named
/// ([`a_row_for_a_site_that_is_gone_is_named`]); a code one of these functions starts to
/// mint is not.
const CLAIMS: &[Claim] = &[
    // `CarryoverBoundary::Setup.code()` — a `match` in the engine hands it over.
    claim(
        "setup.dirty-install-path",
        "crates/engine/src/finalize.rs::setup_dirty_install_finding::dynamic#1",
    ),
    // `locate_finding(code, …)`, called by `setup::run` and `setup::run_uninstall`: its
    // first site answers outside a repository, its second where `$HOME` is not set.
    claim(
        "setup.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#1",
    ),
    claim(
        "setup.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#2",
    ),
    claim(
        "uninstall.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#1",
    ),
    claim(
        "uninstall.repo-root",
        "crates/cli/src/locate.rs::locate_finding::dynamic#2",
    ),
    // By reading — `replaced_path_refusal`: the `refusal` code of each install member
    // whose writer replaces (`install_tracked_paths`), raised where the member's path, or
    // a directory on the way to it, is not a regular file's.
    claim("setup.write-bootstrap", REPLACED_PATH),
    claim("setup.version-stamp", REPLACED_PATH),
    claim("setup.init-project-layer", REPLACED_PATH),
    claim("setup.compose-marker", REPLACED_PATH),
    claim("setup.inject-allowlist", REPLACED_PATH),
    claim("setup.write-guide", REPLACED_PATH),
    // By reading — `merged_through_links`: the four codes its own body chooses among,
    // raised where a merged-into member is a link the install will not follow.
    claim("setup.inject-reference", MERGED_THROUGH),
    claim("setup.inject-allowlist", MERGED_THROUGH),
    claim("setup.secrets-gitignore", MERGED_THROUGH),
    claim("setup.init-project-layer", MERGED_THROUGH),
];

/// The two dynamic sites of `setup.rs` whose codes are listed sites' codes as well.
const REPLACED_PATH: &str = "crates/cli/src/setup.rs::replaced_path_refusal::dynamic#1";
const MERGED_THROUGH: &str = "crates/cli/src/setup.rs::merged_through_links::dynamic#1";

/// **The code-shaped literals of a catalogued prefix that are no finding's code**, each
/// with why — a file name, say. None on this tree.
const NO_CODE: &[(&str, &str)] = &[];

/// The sites one task owes a case, a reason, or both in turn.
struct Owed {
    task: &'static str,
    sites: &'static [&'static str],
}

/// **The owed list: every blocking site of a catalogued family that has neither a case nor
/// a reason yet, under the plan's task that owes it** — `P3` the sites of `jigc setup`,
/// `P4` those of `jigc uninstall`. A row goes in the commit that brings its site's case or
/// reason; a row that stays beside one is named. After `P4` the list is empty and is
/// deleted, with this type and the `owed` field of [`Rows`].
const OWED: &[Owed] = &[
    Owed {
        task: "P3",
        sites: &[
            "crates/cli/src/locate.rs::locate_finding::setup.repo-root#dynamic-1",
            "crates/cli/src/locate.rs::locate_finding::setup.repo-root#dynamic-2",
            "crates/cli/src/setup.rs::commit_install::setup.install-commit#1",
            "crates/cli/src/setup.rs::finding::setup.install-commit#1",
            "crates/cli/src/setup.rs::install::setup.guide-target#1",
            "crates/cli/src/setup.rs::install::setup.inject-allowlist#1",
            "crates/cli/src/setup.rs::install::setup.profile-incomplete#1",
            "crates/cli/src/setup.rs::install::setup.spawn-template#1",
            "crates/cli/src/setup.rs::merged_through_links::setup.init-project-layer#dynamic-1",
            "crates/cli/src/setup.rs::merged_through_links::setup.inject-allowlist#dynamic-1",
            "crates/cli/src/setup.rs::merged_through_links::setup.inject-reference#dynamic-1",
            "crates/cli/src/setup.rs::merged_through_links::setup.secrets-gitignore#dynamic-1",
            "crates/cli/src/setup.rs::replaced_path_refusal::setup.compose-marker#dynamic-1",
            "crates/cli/src/setup.rs::replaced_path_refusal::setup.init-project-layer#dynamic-1",
            "crates/cli/src/setup.rs::replaced_path_refusal::setup.inject-allowlist#dynamic-1",
            "crates/cli/src/setup.rs::replaced_path_refusal::setup.version-stamp#dynamic-1",
            "crates/cli/src/setup.rs::replaced_path_refusal::setup.write-bootstrap#dynamic-1",
            "crates/cli/src/setup.rs::replaced_path_refusal::setup.write-guide#dynamic-1",
            "crates/cli/src/setup.rs::run::setup.profile-load#1",
            "crates/cli/src/setup.rs::write_install_span::setup.compose-marker#1",
            "crates/cli/src/setup.rs::write_install_span::setup.init-project-layer#1",
            "crates/cli/src/setup.rs::write_install_span::setup.inject-allowlist#1",
            "crates/cli/src/setup.rs::write_install_span::setup.inject-allowlist#2",
            "crates/cli/src/setup.rs::write_install_span::setup.inject-allowlist#3",
            "crates/cli/src/setup.rs::write_install_span::setup.inject-deny#1",
            "crates/cli/src/setup.rs::write_install_span::setup.inject-hook#1",
            "crates/cli/src/setup.rs::write_install_span::setup.inject-reference#1",
            "crates/cli/src/setup.rs::write_install_span::setup.install-hook#1",
            "crates/cli/src/setup.rs::write_install_span::setup.install-hook#2",
            "crates/cli/src/setup.rs::write_install_span::setup.secrets-gitignore#1",
            "crates/cli/src/setup.rs::write_install_span::setup.version-stamp#1",
            "crates/cli/src/setup.rs::write_install_span::setup.write-bootstrap#1",
            "crates/cli/src/setup.rs::write_install_span::setup.write-guide#1",
            "crates/engine/src/finalize.rs::setup_dirty_install_finding::setup.dirty-install-path#dynamic-1",
            "crates/engine/src/finalize.rs::setup_unasked_install_finding::setup.unverified-install-path#1",
        ],
    },
    Owed {
        task: "P4",
        sites: &[
            "crates/cli/src/locate.rs::locate_finding::uninstall.repo-root#dynamic-1",
            "crates/cli/src/locate.rs::locate_finding::uninstall.repo-root#dynamic-2",
            "crates/cli/src/setup.rs::dirty_worktree_finding::uninstall.dirty-worktree#1",
            "crates/cli/src/setup.rs::foreign_bytes_finding::uninstall.foreign-bytes#1",
            "crates/cli/src/setup.rs::run_uninstall::uninstall.profile-load#1",
            "crates/cli/src/setup.rs::staged_prose_finding::uninstall.staged-prose#1",
            "crates/cli/src/setup.rs::uninstall::uninstall.remove-allowlist#1",
            "crates/cli/src/setup.rs::uninstall::uninstall.remove-deny#1",
            "crates/cli/src/setup.rs::uninstall::uninstall.remove-guide#1",
            "crates/cli/src/setup.rs::uninstall::uninstall.remove-hook#1",
            "crates/cli/src/setup.rs::uninstall::uninstall.remove-jigc#1",
            "crates/cli/src/setup.rs::uninstall::uninstall.remove-precommit#1",
            "crates/cli/src/setup.rs::uninstall::uninstall.unwire-reference#1",
            "crates/cli/src/setup.rs::untracked_workbench_finding::uninstall.untracked-workbench-file#1",
            "crates/cli/src/setup.rs::unverified_foreign_finding::uninstall.foreign-bytes#1",
            "crates/cli/src/setup.rs::unverified_workbench_finding::uninstall.untracked-workbench-file#1",
            "crates/cli/src/setup.rs::unverified_worktrees_finding::uninstall.dirty-worktree#1",
            "crates/cli/src/task.rs::unverified_prose_finding::uninstall.staged-prose#1",
        ],
    },
];

// ---------------------------------------------------------------------------
// The fence — what it reads, and what it says
// ---------------------------------------------------------------------------

/// What the fence reads of production source.
struct Read {
    /// The refusal sites, by the scan the committed list is held equal to.
    sites: Vec<Site>,
    /// Every code-shaped literal of a catalogued prefix that production code spells: its
    /// value, and where it stands.
    spelled: BTreeMap<String, BTreeSet<String>>,
}

/// Whether `code`'s prefix is one the catalog is held to.
fn catalogued(code: &str) -> bool {
    code.split_once('.')
        .is_some_and(|(prefix, _)| CATALOGUED.contains(&prefix))
}

/// Whether a literal's whole value has the shape of a catalogued family's code: the
/// prefix, a dot, and a name of lower-case letters, digits, `-`, `_` and `.`. Wider than
/// any code the product mints, so that a literal which only looks like one is asked about
/// ([`NO_CODE`]) rather than passed over.
fn code_shaped(value: &str) -> bool {
    catalogued(value)
        && value.split_once('.').is_some_and(|(_, name)| {
            !name.is_empty()
                && name.bytes().all(|b| {
                    b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'-' | b'_' | b'.')
                })
        })
}

fn read(sources: &[Source]) -> Read {
    let mut spelled: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for source in sources {
        let (code, regions) = refusal_sites::production_code(&source.body);
        for literal in rust_source::string_literals(&source.body) {
            if !code_shaped(&literal.value)
                || rust_source::is_test_domain(Path::new(&source.path), &regions, literal.offset)
            {
                continue;
            }
            // The reader's `enclosing_fn`: the last `fn` declared before the literal.
            let at = match rust_source::enclosing_fn(&code, literal.offset) {
                Some(function) => format!("{} (in `{function}`)", source.path),
                None => format!("{} (before any function)", source.path),
            };
            spelled.entry(literal.value).or_default().insert(at);
        }
    }
    Read {
        sites: refusal_sites::scan(sources),
        spelled,
    }
}

/// This tree's production source, read.
fn tree() -> Read {
    read(&refusal_sites::tree_sources())
}

/// The catalog's tables, as the fence is handed them.
#[derive(Default)]
struct Rows {
    /// The site of every case.
    cases: Vec<&'static str>,
    reasons: Vec<Reason>,
    /// `(site, task)`.
    owed: Vec<(&'static str, &'static str)>,
    claims: Vec<Claim>,
    /// `(literal, why it is no finding's code)`.
    no_code: Vec<(&'static str, &'static str)>,
}

/// The tables this suite holds.
fn rows() -> Rows {
    Rows {
        cases: cases().iter().map(|case| case.site).collect(),
        reasons: TABLES
            .iter()
            .flat_map(|table| table.reasons)
            .copied()
            .collect(),
        owed: OWED
            .iter()
            .flat_map(|owed| owed.sites.iter().map(|site| (*site, owed.task)))
            .collect(),
        claims: CLAIMS.to_vec(),
        no_code: NO_CODE.to_vec(),
    }
}

/// A site that owes the catalog a case.
struct Owing {
    id: String,
    /// The route's kind, as the list reads it at the site's call.
    route: &'static str,
}

/// A site refuses where its severity is written `blocking` — or is handed in, which the
/// list cannot read and the fence does not take for advisory.
fn blocks(site: &Site) -> bool {
    matches!(site.severity, "blocking" | DYNAMIC)
}

/// **Every site that owes a case, sorted by id**: the blocking listed sites of the
/// catalogued families, and each claimed code at the blocking dynamic site its claim names.
/// A claim that names no such site adds nothing here and is named by [`idle_rows`].
fn owing(read: &Read, claims: &[Claim]) -> Vec<Owing> {
    let mut out: Vec<Owing> = read
        .sites
        .iter()
        .filter(|site| catalogued(&site.code) && blocks(site))
        .map(|site| Owing {
            id: site.id(),
            route: site.route,
        })
        .collect();
    for claim in claims {
        let minted = read
            .sites
            .iter()
            .find(|site| site.id() == claim.minted_at && site.code == DYNAMIC && blocks(site));
        if let Some(site) = minted
            && let Some(id) = claim.id()
            && catalogued(claim.code)
        {
            out.push(Owing {
                id,
                route: site.route,
            });
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

/// **Every site that does not have exactly one answer** — a case, a reason or an owed row.
/// None is named; so is a case beside a reason, an owed row beside either, and
/// `no-route-printed` given to a site whose route the list reads at its call.
fn uncovered(read: &Read, rows: &Rows) -> Vec<String> {
    let mut out = Vec::new();
    for site in owing(read, &rows.claims) {
        let id = &site.id;
        let case = rows.cases.contains(&id.as_str());
        let reason = rows.reasons.iter().find(|reason| reason.site == id);
        let owed = rows.owed.iter().find(|(owed, _)| owed == id);
        match (case, reason, owed) {
            (false, None, None) => {
                out.push(format!("`{id}` has no case, no reason and no owed row"))
            }
            (true, Some(reason), _) => out.push(format!(
                "`{id}` has a case and the reason `{}` for having none — one of the two is \
                 wrong",
                reason.why
            )),
            (true, None, Some((_, task))) | (false, Some(_), Some((_, task))) => out.push(format!(
                "`{id}` has a {} and is still owed by `{task}` — the row goes in the commit \
                 that brings the answer",
                if case { "case" } else { "reason" }
            )),
            (false, Some(reason), None)
                if matches!(reason.why, Why::NoRoutePrinted)
                    && matches!(
                        site.route,
                        "prose" | "human" | "mechanical" | "informational"
                    ) =>
            {
                out.push(format!(
                    "`{id}` is given `{}`, and the list reads a `{}` route written at its \
                     call",
                    reason.why, site.route
                ))
            }
            _ => {}
        }
    }
    out
}

/// **Every code-shaped literal of a catalogued prefix that is nobody's** — the code of no
/// listed site, claimed by no row, and with no row that says why it is no finding's code.
/// A literal that has such a row *and* is a site's code or a claim's is named too.
fn unclaimed(read: &Read, rows: &Rows) -> Vec<String> {
    let mut out = Vec::new();
    for (code, at) in &read.spelled {
        let at = at.iter().map(String::as_str).collect::<Vec<_>>().join(", ");
        let listed = read.sites.iter().any(|site| site.code == *code);
        let claimed = rows.claims.iter().any(|claim| claim.code == code);
        let excused = rows.no_code.iter().any(|(literal, _)| literal == code);
        match (listed || claimed, excused) {
            (false, false) => out.push(format!(
                "`{code}` — spelled in {at} — is the code of no listed site, no row claims \
                 it for the dynamic site that mints it, and no row says why it is no \
                 finding's code"
            )),
            (true, true) => out.push(format!(
                "`{code}` has a row that says it is no finding's code, and it is {}",
                if listed {
                    "the code of a listed site"
                } else {
                    "claimed for a dynamic site"
                }
            )),
            _ => {}
        }
    }
    out
}

/// **Every row that is about nothing**: an owed row or a reason whose site owes no case —
/// gone, renamed, or never blocking — a claim whose site the list does not hold as a
/// dynamic one or whose code no production source spells, a row for a literal that is no
/// longer spelled; and a row that is there twice, or says nothing where it must say why.
fn idle_rows(read: &Read, rows: &Rows) -> Vec<String> {
    let owing = owing(read, &rows.claims);
    let owes = |id: &str| owing.iter().any(|site| site.id == id);
    let mut out = Vec::new();

    for (at, (site, task)) in rows.owed.iter().enumerate() {
        if task.is_empty() {
            out.push(format!("the owed row for `{site}` names no task"));
        }
        if !owes(site) {
            out.push(format!(
                "the owed list (`{task}`) names `{site}`, which is no blocking site of a \
                 catalogued family: it is gone, renamed, or was never one"
            ));
        } else if rows.owed[..at].iter().any(|(other, _)| other == site) {
            out.push(format!("`{site}` is on the owed list twice"));
        }
    }

    for (at, reason) in rows.reasons.iter().enumerate() {
        let site = reason.site;
        if let Why::NotConstructible("") | Why::Platform("") = reason.why {
            out.push(format!(
                "the reason `{}` for `{site}` states nothing",
                reason.why
            ));
        }
        if !owes(site) {
            out.push(format!(
                "the reason `{}` names `{site}`, which is no blocking site of a catalogued \
                 family: it is gone, renamed, or was never one",
                reason.why
            ));
        } else if rows.reasons[..at].iter().any(|other| other.site == site) {
            out.push(format!("`{site}` is given two reasons"));
        }
    }

    for (at, claim) in rows.claims.iter().enumerate() {
        let Claim { code, minted_at } = *claim;
        match read.sites.iter().find(|site| site.id() == minted_at) {
            None => out.push(format!(
                "the claim of `{code}` names `{minted_at}`, which the list does not hold"
            )),
            Some(site) if site.code != DYNAMIC => out.push(format!(
                "the claim of `{code}` names `{minted_at}`, which is no dynamic site — its \
                 code is spelled at the call"
            )),
            Some(_) => {}
        }
        if !read.spelled.contains_key(code) {
            out.push(format!(
                "the claim of `{code}` for `{minted_at}`: no production source spells that \
                 code under a catalogued prefix"
            ));
        }
        if rows.claims[..at]
            .iter()
            .any(|other| other.code == code && other.minted_at == minted_at)
        {
            out.push(format!("`{code}` is claimed twice for `{minted_at}`"));
        }
    }

    for (at, (literal, why)) in rows.no_code.iter().enumerate() {
        if why.is_empty() {
            out.push(format!(
                "the row for the literal `{literal}` does not say why it is no finding's code"
            ));
        }
        if !read.spelled.contains_key(*literal) {
            out.push(format!(
                "a row says the literal `{literal}` is no finding's code, and no production \
                 source spells it"
            ));
        } else if rows.no_code[..at].iter().any(|(other, _)| other == literal) {
            out.push(format!("the literal `{literal}` has two rows"));
        }
    }
    out
}

/// The files of `on_disk` that are no row of `wired`.
fn unwired(on_disk: &[String], wired: &[&str]) -> Vec<String> {
    on_disk
        .iter()
        .filter(|file| !wired.contains(&file.as_str()))
        .map(|file| {
            format!(
                "`{file}` is declared by no line inside `tables!`, so the catalog follows \
                 none of its cases and the fence reads none of its reasons"
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The fence — its arms
// ---------------------------------------------------------------------------

/// A production path for a fixture: under a crate's `src/`, so the reader's test-domain
/// rule leaves it to the `#[cfg(test)]` regions alone.
const FIXTURE: &str = "crates/cli/src/fixture.rs";

/// A production file in small. Of two catalogued families it holds a blocking site with a
/// route written at the call, a blocking one that prints none, an advisory one, and a code
/// that reaches its constructor through a `match`; beside them another family's site and
/// code, a literal that only looks like a code, and a code that only a test spells.
const REFUSALS: &str = r#"
use engine::finding::{Finding, Severity};

fn refuses() -> Finding {
    Finding::block("setup.spelled", "m", "re-run `jigc setup`")
}

fn routeless() -> Finding {
    Finding::blocking("uninstall.routeless", "m", None)
}

fn advises() -> Finding {
    Finding::graded(Severity::Advisory, "setup.advised", "m", None, None)
}

fn another_familys() -> Finding {
    Finding::block("task.spelled", "m", "r")
}

fn code(door: Door) -> &'static str {
    match door {
        Door::Setup => "setup.by-a-match",
        Door::Task => "task.by-a-match",
    }
}

fn minted(door: Door) -> Finding {
    Finding::block(code(door), "m", "r")
}

fn a_file_name() -> &'static str {
    "setup.rs"
}

#[cfg(test)]
mod tests {
    #[test]
    fn asserts() {
        assert_eq!(minted(Door::Setup).code, "setup.only-a-test-spells-it");
    }
}
"#;

const SPELLED: &str = "crates/cli/src/fixture.rs::refuses::setup.spelled#1";
const ROUTELESS: &str = "crates/cli/src/fixture.rs::routeless::uninstall.routeless#1";
const ADVISED: &str = "crates/cli/src/fixture.rs::advises::setup.advised#1";
const MINTED: &str = "crates/cli/src/fixture.rs::minted::dynamic#1";
const CLAIMED: &str = "crates/cli/src/fixture.rs::minted::setup.by-a-match#dynamic-1";
const GONE: &str = "crates/cli/src/fixture.rs::removed::setup.removed#1";

fn fixture() -> Read {
    read(&[Source::new(FIXTURE, REFUSALS)])
}

/// The fixture's one claim.
const BY_A_MATCH: Claim = claim("setup.by-a-match", MINTED);

/// The fixture's tables with nothing wrong: a case, a reason and an owed row, one site
/// each; the claim; and the literal that is no code.
fn in_order() -> Rows {
    Rows {
        cases: vec![SPELLED],
        reasons: vec![Reason {
            site: ROUTELESS,
            why: Why::NoRoutePrinted,
        }],
        owed: vec![(CLAIMED, "P3")],
        claims: vec![BY_A_MATCH],
        no_code: vec![("setup.rs", "a file's name")],
    }
}

/// `said` is one line for each of `expected`, in order, and each line holds its two
/// pieces: what it is about, and what it says of it.
fn says(said: &[String], expected: &[(&str, &str)]) {
    assert_eq!(said.len(), expected.len(), "{said:#?}");
    for (line, (about, what)) in said.iter().zip(expected) {
        assert!(
            line.contains(about) && line.contains(what),
            "a line about `{about}` that says `{what}`: {said:#?}",
        );
    }
}

/// **Every blocking site of the catalogued families has a case, a reason or an owed row —
/// exactly one.** On a fixture: the advisory site and the other family's owe nothing, a
/// claim makes the code at its dynamic site owe like a listed one, and two answers for one
/// site are named as none is. Then this tree, against the tables this suite holds.
#[test]
fn every_refusal_site_of_the_catalogued_families_has_a_case_a_reason_or_an_owed_row() {
    let fixture = fixture();
    says(
        &uncovered(&fixture, &Rows::default()),
        &[(SPELLED, "no case"), (ROUTELESS, "no case")],
    );
    says(
        &uncovered(
            &fixture,
            &Rows {
                claims: vec![BY_A_MATCH],
                ..Rows::default()
            },
        ),
        &[
            (CLAIMED, "no case"),
            (SPELLED, "no case"),
            (ROUTELESS, "no case"),
        ],
    );
    says(&uncovered(&fixture, &in_order()), &[]);
    for why in [
        Why::NotConstructible("the profile is compiled in"),
        Why::Platform("a filesystem without links"),
    ] {
        let reasons = vec![Reason {
            site: ROUTELESS,
            why,
        }];
        says(
            &uncovered(
                &fixture,
                &Rows {
                    reasons,
                    ..in_order()
                },
            ),
            &[],
        );
    }

    says(
        &uncovered(
            &fixture,
            &Rows {
                owed: vec![(SPELLED, "P3"), (ROUTELESS, "P4"), (CLAIMED, "P3")],
                ..in_order()
            },
        ),
        &[
            (SPELLED, "a case and is still owed by `P3`"),
            (ROUTELESS, "a reason and is still owed by `P4`"),
        ],
    );
    let both = Reason {
        site: SPELLED,
        why: Why::NotConstructible("nothing reaches it"),
    };
    says(
        &uncovered(
            &fixture,
            &Rows {
                reasons: vec![both, in_order().reasons[0]],
                ..in_order()
            },
        ),
        &[(
            SPELLED,
            "a case and the reason `not-constructible: nothing reaches it`",
        )],
    );
    // `no-route-printed` is taken from the site that prints none, and from no other.
    let routeless = |site| Reason {
        site,
        why: Why::NoRoutePrinted,
    };
    says(
        &uncovered(
            &fixture,
            &Rows {
                cases: vec![],
                reasons: vec![routeless(SPELLED), routeless(ROUTELESS)],
                ..in_order()
            },
        ),
        &[(SPELLED, "a `prose` route written at its call")],
    );

    let read = tree();
    let wrong = uncovered(&read, &rows());
    assert!(
        wrong.is_empty(),
        "the route catalog does not answer for {} of the {} blocking refusal sites of the \
         families {CATALOGUED:?}:\n\n  {}\n\na site is answered by a case in its command's \
         file under `route_catalog/`, by a reason there, or — until the task that owes it \
         lands — by a row of `OWED`; a code that a dynamic site mints is a site once \
         `CLAIMS` holds it.",
        wrong.len(),
        owing(&read, CLAIMS).len(),
        wrong.join("\n  "),
    );
}

/// **The vocabulary is held, not only the call sites.** On a fixture: the code a `match`
/// hands to a constructor is named until a row claims it, a literal that only looks like a
/// code until a row says so, and neither another family's code nor a test's is asked about.
/// Then this tree.
#[test]
fn every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed() {
    let fixture = fixture();
    assert_eq!(
        fixture.spelled.keys().collect::<Vec<_>>(),
        [
            "setup.advised",
            "setup.by-a-match",
            "setup.rs",
            "setup.spelled",
            "uninstall.routeless",
        ],
        "every code-shaped literal of a catalogued prefix in production code, and no other",
    );
    says(
        &unclaimed(&fixture, &Rows::default()),
        &[
            ("`setup.by-a-match`", "fixture.rs (in `code`)"),
            ("`setup.rs`", "fixture.rs (in `a_file_name`)"),
        ],
    );
    says(&unclaimed(&fixture, &in_order()), &[]);
    says(
        &unclaimed(
            &fixture,
            &Rows {
                no_code: vec![
                    ("setup.by-a-match", "it is one"),
                    ("setup.rs", "a file's name"),
                    ("setup.spelled", "it is one"),
                ],
                ..in_order()
            },
        ),
        &[
            ("`setup.by-a-match`", "claimed for a dynamic site"),
            ("`setup.spelled`", "the code of a listed site"),
        ],
    );

    let wrong = unclaimed(&tree(), &rows());
    assert!(
        wrong.is_empty(),
        "production source spells {} code(s) of the families {CATALOGUED:?} that the list \
         of refusal sites does not hold and nothing answers for:\n\n  {}\n\nread where each \
         is handed to a constructor. A `dynamic` site of \
         `crates/cli/tests/route_catalog/sites.tsv` mints it: a row of `CLAIMS` names that \
         site, and the code there then owes a case. It is no finding's code: a row of \
         `NO_CODE` says why.",
        wrong.len(),
        wrong.join("\n  "),
    );
}

/// **A row for a site that is gone is named** — and so is every other row that is about
/// nothing: one for a site that never owed a case, a claim for a site that is not dynamic
/// or a code nothing spells, a row that is there twice, a row that does not say why. On a
/// fixture, kind by kind; then this tree's tables.
#[test]
fn a_row_for_a_site_that_is_gone_is_named() {
    let fixture = fixture();
    says(&idle_rows(&fixture, &in_order()), &[]);

    says(
        &idle_rows(
            &fixture,
            &Rows {
                owed: vec![
                    (CLAIMED, "P3"),
                    (GONE, "P3"),
                    (ADVISED, "P3"),
                    (CLAIMED, "P4"),
                    (ROUTELESS, ""),
                ],
                ..in_order()
            },
        ),
        &[
            (GONE, "the owed list (`P3`) names"),
            (ADVISED, "no blocking site of a catalogued family"),
            (CLAIMED, "on the owed list twice"),
            (ROUTELESS, "names no task"),
        ],
    );

    let reason = |site, why| Reason { site, why };
    says(
        &idle_rows(
            &fixture,
            &Rows {
                reasons: vec![
                    reason(ROUTELESS, Why::NoRoutePrinted),
                    reason(GONE, Why::Platform("another")),
                    reason(ROUTELESS, Why::NotConstructible("")),
                ],
                ..in_order()
            },
        ),
        &[
            (GONE, "the reason `platform: another` names"),
            (ROUTELESS, "states nothing"),
            (ROUTELESS, "is given two reasons"),
        ],
    );

    // A claim that names no dynamic site makes no site, so the row owed under it is idle
    // with it.
    says(
        &idle_rows(
            &fixture,
            &Rows {
                claims: vec![claim("setup.by-a-match", GONE)],
                ..in_order()
            },
        ),
        &[
            (CLAIMED, "the owed list (`P3`) names"),
            (GONE, "which the list does not hold"),
        ],
    );
    says(
        &idle_rows(
            &fixture,
            &Rows {
                claims: vec![
                    BY_A_MATCH,
                    claim("setup.by-a-match", SPELLED),
                    claim("setup.nothing-spells-it", MINTED),
                    BY_A_MATCH,
                ],
                ..in_order()
            },
        ),
        &[
            (SPELLED, "is no dynamic site"),
            ("`setup.nothing-spells-it`", "no production source spells"),
            ("`setup.by-a-match`", "is claimed twice"),
        ],
    );

    says(
        &idle_rows(
            &fixture,
            &Rows {
                no_code: vec![
                    ("setup.rs", ""),
                    ("setup.md", "a file's name"),
                    ("setup.rs", "a file's name"),
                ],
                ..in_order()
            },
        ),
        &[
            ("`setup.rs`", "does not say why"),
            ("`setup.md`", "no production source spells it"),
            ("`setup.rs`", "has two rows"),
        ],
    );

    let wrong = idle_rows(&tree(), &rows());
    assert!(
        wrong.is_empty(),
        "the route catalog's tables hold {} row(s) that are about nothing:\n\n  {}\n\na row \
         goes with what it was about. A site that was renamed keeps its row under the new \
         id, which `crates/cli/tests/route_catalog/sites.tsv` holds.",
        wrong.len(),
        wrong.join("\n  "),
    );
}

/// **A command's file that is on disk is one the suite reads.** The registration fence
/// holds every `.rs` file under `route_catalog/` to a `#[path]` line in this file; this
/// arm holds it to a line *inside* `tables!`, where its cases are followed and its reasons
/// are read. A declaration beside the macro satisfies the first and is named here.
#[test]
fn every_file_of_the_catalogs_directory_is_a_table_the_suite_reads() {
    let wired: Vec<&str> = TABLES.iter().map(|table| table.file).collect();
    let planted = [wired[0].to_string(), "route_catalog/beside.rs".to_string()];
    says(
        &unwired(&planted, &wired),
        &[("`route_catalog/beside.rs`", "no line inside `tables!`")],
    );

    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let on_disk: Vec<String> = rust_source::rust_files(&tests.join("route_catalog"))
        .iter()
        .map(|path| {
            path.strip_prefix(&tests)
                .expect("a walked file is under the suite home")
                .components()
                .map(|part| part.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        })
        .collect();
    let wrong = unwired(&on_disk, &wired);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **The fence is not vacuous.** Each of its arms is green over an empty set, so each set
/// is shown not to be empty on this tree, and the fence is run once with its tables taken
/// away:
///
///   * every catalogued prefix is a family of the list and the prefix of a listed site;
///   * the vocabulary's reader sees what the scan sees — the code of every listed site of a
///     catalogued family that is spelled at its call is among the literals, in that file;
///   * handed no case, no reason and no owed row, the fence names every site that owes
///     one, listed and claimed, and there are both;
///   * handed no claim, it names every code no listed site carries.
#[test]
fn the_fence_is_not_vacuous() {
    let read = tree();
    for prefix in CATALOGUED {
        assert!(
            FAMILIES.iter().any(|(family, _)| family == prefix),
            "`{prefix}` is no row of the list's family table",
        );
        assert!(
            read.sites
                .iter()
                .any(|site| site.code.split_once('.').map(|(family, _)| family) == Some(*prefix)),
            "the list holds no site of the family `{prefix}`",
        );
    }

    for site in &read.sites {
        if catalogued(&site.code) && site.written == "literal" {
            assert!(
                read.spelled
                    .get(&site.code)
                    .is_some_and(|at| at.iter().any(|at| at.starts_with(&site.file))),
                "`{}` spells its code at the call, and the vocabulary's reader did not find \
                 that literal in `{}`",
                site.id(),
                site.file,
            );
        }
    }

    let bare = Rows {
        claims: CLAIMS.to_vec(),
        ..Rows::default()
    };
    let owing = owing(&read, CLAIMS);
    let claimed = owing
        .iter()
        .filter(|site| site.id.contains(&format!("#{DYNAMIC}-")))
        .count();
    assert!(
        claimed > 0 && claimed < owing.len(),
        "{} sites owe a case, {claimed} of them claimed: both kinds are there",
        owing.len(),
    );
    let named = uncovered(&read, &bare);
    assert_eq!(
        named.len(),
        owing.len(),
        "handed no case, no reason and no owed row, the fence names every site that owes \
         one: {named:#?}",
    );

    let unlisted: BTreeSet<&str> = CLAIMS
        .iter()
        .map(|claim| claim.code)
        .filter(|code| !read.sites.iter().any(|site| site.code == *code))
        .collect();
    let named = unclaimed(
        &read,
        &Rows {
            no_code: NO_CODE.to_vec(),
            ..Rows::default()
        },
    );
    assert!(
        !unlisted.is_empty(),
        "no claim is for a code the list lacks"
    );
    assert_eq!(
        named.len(),
        unlisted.len(),
        "handed no claim, the fence names every code no listed site carries: {named:#?}",
    );
    for code in unlisted {
        assert!(
            named.iter().any(|line| line.contains(&format!("`{code}`"))),
            "`{code}` is not named: {named:#?}",
        );
    }
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
/// The source half reads the frame, this file and every command's. An [`Act`] carries an
/// [`Operand`] or a span's index and nothing else; an operand is a span's index or the one
/// marked form; a [`Case`] holds no function, so it cannot act around the runner; and
/// neither this file nor a command's names a filesystem or a process API at all — what the
/// existing test did by hand (`fs::remove_file` on a path it spelled) has no place to be
/// written.
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
    let files = std::iter::once(("route_catalog.rs", include_str!("route_catalog.rs")))
        .chain(TABLES.iter().map(|table| (table.file, table.source)));
    for (file, source) in files {
        let code = rust_source::code_only(source);
        for by_hand in ["fs::", "std::os", "std::process", "Command", "symlink"] {
            assert!(
                !code.contains(by_hand),
                "`{file}` names `{by_hand}`: a case's every act goes through the runner, \
                 which takes its operand from the printed route",
            );
        }
    }

    let planted = Case {
        name: "planted-the-links-own-path-by-a-constant",
        acts: &[Act::Remove(Operand::Unprinted(HOOK)), Act::Run(1)],
        ..setup::CASES[0]
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
