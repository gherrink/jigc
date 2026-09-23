//! M50 Increment 2 / T1 — **the address slug head, at every door that takes an address.**
//!
//! ## The class this closes
//!
//! A doc address is `<type>:<slug>`, and the slug is what names the file:
//! `<docs-root>/<location>/<slug>.md`. The address grammar splits on `:` / `#` / `/`
//! and **sanitizes nothing**, so until M50 the `<slug>` head reached a path component
//! with no door asking whether it was a slug at all. Driven at `23487ab`:
//!
//!   * `jigc doc show "research:<absolute path outside the repository>" --format json`
//!     exits **0** and serves the outside file's prose through the **1.0-pinned** JSON
//!     contract, with `slug` echoing the absolute path;
//!   * `jigc rename 'research:../../src/planted' --to "Captured Doc"` exits **0** and
//!     lands the commit `src/planted.md => docs/research/captured-doc.md` — an
//!     arbitrary in-repo file captured into the docs root;
//!   * `jigc doc add-item 'roadmap:../..#milestones' --title M --task <id>` exits **0**,
//!     minting a staged identity no door can address afterwards.
//!
//! (`DECISIONS.md` → 2026-09-05 M50 Increment 2 planning; `completions/artifacts/M50/`.)
//!
//! ## The axis is the shipped registry, not a list
//!
//! The subject is [`DOCTYPE_DOORS`] filtered to [`DoctypeArg::Address`] — the door set
//! `cli_parse::every_doctype_door_is_registered` already fences **⇔** against the real
//! clap tree, so an eleventh address-taking verb cannot ship without joining it. Ten
//! doors, reaching **four** different parsers (`doc::parse_verb_addr` over seven `doc`
//! verbs, `task bind`'s own `Address::parse`, `rename`'s hand-split `parse_addr`, and
//! `milestone add-from-spec`'s door-side parse ahead of the engine's spec read), which is
//! exactly why the door set and not the function is the acceptance: a guard at any one of
//! them covers a quarter of the class.
//!
//! **The tenth door joined late, and that is the finding this count records.** Until the
//! M50 completion audit the registry was derived from an allowlist of clap *argument
//! names*, and `milestone add-from-spec` takes its address through `spec_addr` — a name
//! nobody had listed — so the door was invisible to the derivation the Settle called the
//! strongest available. Driven: `jigc milestone add-from-spec <m> 'spec:../../../..
//! /<outside>/planted'` read the file **outside the repository**, seeded a sub-task from
//! its criteria and landed a commit naming the foreign source, at exit 0. The registry is
//! now derived from `cli::cli::ARG_TOKENS`, a **total** classification of every clap
//! argument, so an argument nobody classifies cannot ship at all.
//!
//! **A row with no cell is a hard panic, never a skip** — [`argv_for`] matches the door
//! path exhaustively and panics on an unregistered one, and the driven set is compared
//! back to the registry at the end.
//!
//! ## The three cells
//!
//! A `../..` traversal, an **absolute path that really exists** (the corpus's own
//! repository root), and a **space-carrying** non-slug. Each must block non-zero with
//! [`MALFORMED_SLUG`], name the token the caller typed, and carry exactly one route.
//!
//! ## Why the tree arms exist
//!
//! The text alone passed at `23487ab`'s exit 0, so two arms assert the **tree**: the
//! read arm plants a canary **outside the repository** and requires that no byte of it
//! reaches the pinned JSON, and the rename arm requires that no commit lands and that
//! `src/planted.md` is still where it was. (Fixture fact, driven: macOS resolves `..`
//! physically, so `docs/research/` must exist on disk or the rename is refused for an
//! unrelated reason and the arm proves nothing.)
//!
//! ## The disposition arm
//!
//! The guard newly refuses an identity that a **hand-dropped** file at a managed home can
//! still carry, so the store surfaces must not go on calling it `managed`: a row nothing
//! can address is not a registered doc. The arm drives the stamped and the unstamped
//! population and requires both to answer `unregistered` with an adoption route that
//! names a runnable repair.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

use cli::cli::{DOCTYPE_DOORS, DoctypeArg};

use crate::support;
use support::trial_corpus::{State, TrialCorpus};

/// The blocking finding a malformed address slug head earns at every door — one code for
/// the whole family, read doors and write doors alike. Spelled out rather than imported:
/// a test comparing emitted bytes against the constant that produced them proves only
/// that the constant equals itself.
const MALFORMED_SLUG: &str = "store.malformed-slug";

/// The grammar sentence the refusal's route states — the **one** shipped literal
/// (`crate::task::WORK_UNIT_ID_GRAMMAR`, fenced against `design/structural-grammar.md`
/// by `malformed_work_unit_id::the_design_doc_states_the_shipped_grammar_verbatim`).
const GRAMMAR: &str =
    "use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)";

/// The placeholder every door's argv carries in place of the address slug, so one row
/// serves all three cells instead of three hand-written argvs per door.
const SLUG_SLOT: &str = "<slug>";

/// The milestone id the `milestone add-from-spec` row names — well-formed, so the door
/// answers about the **address** and not about the id.
const MILESTONE: &str = "axis-milestone";

/// A payload file the `set-slot` row's `--from-file` reads, so that row's door answers
/// about the address and not about a missing file.
const PAYLOAD: &str = "payload.txt";

/// The runnable argv for one address door, with [`SLUG_SLOT`] standing in for the slug.
///
/// Exhaustive over the registered address doors **by panic**: a door that joins
/// [`DOCTYPE_DOORS`] with no row here fails the suite naming itself, rather than being
/// silently skipped.
fn argv_for(door: &[&str], task: &str) -> Vec<String> {
    let argv: Vec<&str> = match door {
        ["rename"] => vec!["rename", "research:<slug>", "--to", "Captured Doc"],
        ["doc", "add-item"] => vec![
            "doc",
            "add-item",
            "roadmap:<slug>#milestones",
            "--title",
            "Captured",
            "--task",
            task,
        ],
        ["doc", "remove-item"] => vec![
            "doc",
            "remove-item",
            "roadmap:<slug>#milestones/m-alpha",
            "--task",
            task,
        ],
        ["doc", "retitle-item"] => vec![
            "doc",
            "retitle-item",
            "roadmap:<slug>#milestones/m-alpha",
            "--title",
            "Captured",
            "--task",
            task,
        ],
        ["doc", "rename"] => vec![
            "doc",
            "rename",
            "research:<slug>",
            "--to",
            "Captured Doc",
            "--task",
            task,
        ],
        ["doc", "set-field"] => vec![
            "doc",
            "set-field",
            "research:<slug>#meta/date",
            "--value",
            "2026-01-01",
            "--task",
            task,
        ],
        ["doc", "set-slot"] => vec![
            "doc",
            "set-slot",
            "research:<slug>#question",
            "--from-file",
            PAYLOAD,
            "--task",
            task,
        ],
        ["doc", "show"] => vec!["doc", "show", "research:<slug>"],
        ["task", "bind"] => vec!["task", "bind", "spec", "spec:<slug>", task],
        // The milestone is real (minted by the fixture), so the cell reaches the spec
        // read — the seam whose escape served bytes from outside the repository.
        ["milestone", "add-from-spec"] => {
            vec!["milestone", "add-from-spec", MILESTONE, "spec:<slug>"]
        }
        other => panic!(
            "`jigc {}` is a registered `DoctypeArg::Address` door with no cell here — a \
             new address-taking verb owes this axis the argv that reaches its address \
             parse, or the guard's completeness is unproven for it",
            other.join(" "),
        ),
    };
    argv.into_iter().map(str::to_owned).collect()
}

/// Substitute the cell's token into a door's argv.
fn with_slug(argv: &[String], slug: &str) -> Vec<String> {
    argv.iter()
        .map(|arg| arg.replace(SLUG_SLOT, slug))
        .collect()
}

/// Both streams of one invocation — the surface a reader meets.
fn surface(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Commit whatever is in the corpus's worktree, so a planted file is a **committed**
/// file (the store surfaces adjudicate the committed store).
fn commit_all(corpus: &TrialCorpus, message: &str) {
    for args in [
        vec!["add", "-A"],
        vec![
            "-c",
            "user.email=t@example.com",
            "-c",
            "user.name=T",
            "commit",
            "-qm",
            message,
        ],
    ] {
        let out = Command::new("git")
            .args(&args)
            .current_dir(corpus.repo())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// The repository's current HEAD sha — so a refused door can be proven to have landed
/// **no commit**, not merely to have printed a refusal.
fn head_sha(repo: &Path) -> String {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()
        .expect("run git rev-parse");
    assert!(out.status.success(), "git rev-parse HEAD must succeed");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A conformant `research` doc body — the shape the read arm plants outside the
/// repository and the disposition arm plants at the managed home.
fn research_body(title: &str, question: &str) -> String {
    format!(
        "---\ndate: 2026-01-01\nschema-version: 1\n---\n\n# {title}\n\n## Question\n\n\
         {question}\n\n## Findings\n\nnothing\n\n## Sources\n\nnone\n"
    )
}

/// **The axis** — every registered `DoctypeArg::Address` door, over the whole malformed
/// slug-head cell space.
#[test]
fn every_address_door_refuses_a_malformed_slug_head() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    // `implement-from-spec` is the one shipped workflow declaring a `reads` role, so the
    // `task bind` row reaches its address parse instead of stopping at the role check.
    let task = corpus.start_workflow("implement-from-spec", "address slug head axis");
    fs::write(repo.join(PAYLOAD), "prose\n").expect("write the set-slot payload");
    // The rename row's destination home must exist on disk or `..` is refused for an
    // unrelated reason (driven; macOS resolves `..` physically).
    fs::create_dir_all(repo.join("docs").join("research")).expect("create the research home");
    // The `milestone add-from-spec` row's milestone must be REAL, or the door would answer
    // `milestone.unknown` and the cell would pass while proving nothing about the address:
    // the escape it closes is the spec **read**, which sits behind the milestone resolve.
    let minted = corpus.jigc_ok(&["milestone", "create", "Axis Milestone"]);
    assert!(
        minted.contains(MILESTONE),
        "the axis milestone must be `{MILESTONE}`; got: {minted}",
    );

    let absolute = repo.to_string_lossy().into_owned();
    let cells: Vec<String> = vec!["../..".to_string(), absolute, "Odd Name".to_string()];

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    let mut registered = 0usize;
    for (door, arg) in DOCTYPE_DOORS {
        if *arg != DoctypeArg::Address {
            continue;
        }
        registered += 1;
        let template = argv_for(door, &task);
        for slug in &cells {
            let argv = with_slug(&template, slug);
            let borrowed: Vec<&str> = argv.iter().map(String::as_str).collect();
            let shown = format!("jigc {} [{slug}]", door.join(" "));
            let out = corpus.jigc(&borrowed);
            assert_ne!(
                out.status.code(),
                Some(101),
                "{shown}: the refusal must be a finding, never a panic\n{}",
                surface(&out),
            );
            assert!(
                !out.status.success(),
                "{shown}: a slug head that is not a slug names no doc — the door must \
                 block non-zero\n{}",
                surface(&out),
            );
            let text = surface(&out);
            assert!(
                text.contains(&format!("· {MALFORMED_SLUG} — ")),
                "{shown}: must carry the `{MALFORMED_SLUG}` finding code\n{text}",
            );
            assert!(
                text.contains(&format!("{slug:?}")),
                "{shown}: must name the token the caller typed\n{text}",
            );
            assert_eq!(
                text.matches("route:").count(),
                1,
                "{shown}: must carry exactly one route\n{text}",
            );
            assert!(
                text.contains(GRAMMAR),
                "{shown}: the route must state the grammar\n{text}",
            );
        }
        driven.insert(door.to_vec());
    }

    assert_eq!(
        driven.len(),
        registered,
        "every registered address door is driven exactly once",
    );
    assert_eq!(
        registered, 10,
        "the registry ships ten `DoctypeArg::Address` doors; a change to that count is \
         a change to this class's axis and must be read, not absorbed",
    );
}

/// **Tree arm 1 — the read.** A canary planted *outside* the repository must not reach
/// the 1.0-pinned `doc show --format json` contract.
#[test]
fn the_pinned_json_serves_no_byte_from_outside_the_repository() {
    let corpus = TrialCorpus::build(State::Fresh);
    // `home()` is the corpus's `$HOME`, a sibling of `repo()` — outside the repository
    // by construction, and removed with the corpus.
    let outside = corpus.home().join("outside.md");
    let canary = "CANARY-OUTSIDE-THE-REPOSITORY";
    fs::write(&outside, research_body("Outside Doc", canary)).expect("plant the canary");
    let stem = outside.with_extension("");
    let addr = format!("research:{}", stem.to_string_lossy());

    let out = corpus.jigc(&["doc", "show", &addr, "--format", "json"]);
    let text = surface(&out);
    assert!(
        !out.status.success(),
        "an absolute path is not a slug — the read must block\n{text}",
    );
    assert!(
        !text.contains(canary),
        "no byte of a file outside the repository may reach the pinned read\n{text}",
    );
    assert!(
        outside.is_file(),
        "the arm is only a proof while the planted file is genuinely readable on disk",
    );
}

/// **Tree arm 2 — the write.** A traversal address must land no commit and move no file.
#[test]
fn a_traversal_address_captures_no_file_into_the_docs_root() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let planted = repo.join("src").join("planted.md");
    fs::create_dir_all(planted.parent().expect("src/")).expect("create src/");
    fs::write(&planted, "# Planted\n\nsource-tree prose\n").expect("plant the file");
    // Driven fixture fact: without the doctype's home on disk the traversal is refused
    // for an unrelated reason (macOS resolves `..` physically) and the arm proves nothing.
    fs::create_dir_all(repo.join("docs").join("research")).expect("create the research home");
    commit_all(&corpus, "plant a source-tree doc");
    let before = head_sha(&repo);

    let out = corpus.jigc(&[
        "rename",
        "research:../../src/planted",
        "--to",
        "Captured Doc",
    ]);
    let text = surface(&out);
    assert!(!out.status.success(), "the traversal must block\n{text}");
    assert_eq!(
        head_sha(&repo),
        before,
        "a refused rename must land no commit\n{text}",
    );
    assert!(
        planted.is_file(),
        "the planted file must still be where it was\n{text}",
    );
    assert!(
        !repo
            .join("docs")
            .join("research")
            .join("captured-doc.md")
            .exists(),
        "nothing may be captured into the docs root\n{text}",
    );
}

/// **The disposition arm** — no surface still reports `managed` for an identity every
/// door now refuses.
///
/// The population is a **hand-dropped** file at a managed home whose name is not a doc
/// id: jigc's own writer only ever names a doc `<slug>.md`, so no jigc operation produced
/// that path. Before the guard the file was addressable and `managed` was true of it;
/// after the guard it is addressable by nothing, and a `managed` row would be a law-1 lie
/// — the sharpest form of it, since the refusal's own route names `jigc doc list`.
///
/// Both halves of the population are driven — **stamped** (which the byte-keyed
/// discriminator called managed) and **unstamped** (which it already called foreign) — and
/// both must answer with the shipped `unregistered` state and an adoption route that names
/// a runnable repair. The fixture's names are deliberately **space-free**: `adoption_route`
/// emits an unquoted path token, which is a pre-existing law-2 defect this arm must not
/// trip (recorded in `DECISIONS.md`, not fixed here).
#[test]
fn no_surface_calls_an_unaddressable_identity_managed() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();
    let stamped = repo.join("docs").join("research").join("OddName.md");
    fs::create_dir_all(stamped.parent().expect("research home")).expect("create the research home");
    fs::write(&stamped, research_body("OddName", "stamped and conformant"))
        .expect("plant the stamped instance");
    let unstamped = repo.join("docs").join("decisions").join("OldDoc.md");
    fs::create_dir_all(unstamped.parent().expect("decisions home")).expect("create decisions/");
    fs::write(&unstamped, "# OldDoc\n\n## Status\n\naccepted\n").expect("plant the unstamped one");
    commit_all(&corpus, "plant two unaddressable identities");

    for (label, doctype, id) in [
        ("stamped", "research", "research:OddName"),
        ("unstamped", "adr", "adr:OldDoc"),
    ] {
        let listing = corpus.jigc_ok(&["doc", "list", doctype]);
        let row = listing
            .lines()
            .find(|line| line.starts_with(id))
            .unwrap_or_else(|| panic!("the {label} instance must be listed\n{listing}"));
        assert!(
            !row.ends_with("managed"),
            "the {label} instance carries an identity every door refuses — `managed` is a \
             law-1 lie\n{listing}",
        );
        assert!(
            row.ends_with("unregistered"),
            "the {label} instance must answer with the shipped `unregistered` state\n{listing}",
        );
    }

    // The route half: the store sweep names the same file as an adoption case, with a
    // route that names a repair that actually runs.
    let out = corpus.jigc(&["validate"]);
    let text = surface(&out);
    assert!(
        !out.status.success(),
        "a never-adopted file at a managed home flips the store sweep's exit\n{text}",
    );
    for path in ["docs/research/OddName.md", "docs/decisions/OldDoc.md"] {
        assert!(
            text.contains("schema-conformance.unadopted-instance") && text.contains(path),
            "`{path}` must draw the shipped adoption advisory\n{text}",
        );
    }
    // **A repair that runs from where the reader is standing** (M53 post-review-fix review,
    // HIGH 2): `jigc migrate <PATH>` roots its argument at the caller's cwd, so the operand
    // is absolute. Composed from the corpus's own root, not pinned.
    let expected_migrate = format!(
        "jigc migrate {} --as research",
        corpus
            .repo()
            .canonicalize()
            .unwrap_or_else(|_| corpus.repo().to_path_buf())
            .join("docs/research/OddName.md")
            .display(),
    );
    assert!(
        text.contains(&expected_migrate),
        "the adoption route must name a repair that runs — expected `{expected_migrate}`\n{text}",
    );
}
