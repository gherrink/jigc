//! The **relocation-route family**: every production site that names `jigc migrate-corpus`
//! fires in a state `jigc migrate-corpus` answers (M52 Increment 7 / T4; settle-record D4.5 +
//! D4.6, decompose order §16 — D4.1's walk, then this).
//!
//! # Why a route family needs a fence and not a grep
//!
//! Before Increment 7's walk, [`cli::relocate::relocate_freeze_exempt`] refused a frozen
//! doctype with *"relocate it through the version-gated `jigc migrate-corpus`"* — and for
//! three of the four `{location, placement}²` home moves that verb did **nothing**:
//! `0 migrated, 0 already current, 0 blocked` at exit 0, the instance still stranded. The
//! route was not wrong about *which* verb owns the act; it was pointing at a verb that could
//! not perform it. The same hole sat under the store sweep's own repair line — `validate`
//! printed *"run `jigc migrate-corpus`, then re-validate"*, and the re-validate came back red.
//!
//! D4.5 decides that route becomes **true rather than repaired**, and D4.6 widens it to the
//! family: *every* route naming the verb either fires in a state the widened walk answers, or
//! is re-routed. A claim of that shape is only worth what checks it, so:
//!
//! - **the subject is read off the source tree**, not written down — every production
//!   occurrence of the token `migrate-corpus` in `crates/cli/src` and `crates/engine/src`,
//!   comments blanked and `#[cfg(test)]` bodies excluded
//!   ([`support::rust_source`], the reader the sibling source fences share);
//! - **each occurrence must carry a stated row** naming *the state it fires in* and *its
//!   disposition*, so a new producer cannot join the family in silence;
//! - **two rows are driven, not argued**: [`the_relocate_refusal_routes_to_a_verb_that_acts`]
//!   and [`the_store_sweep_repair_line_clears_the_store`] lift the **emitted** command out of
//!   the surface the binary actually printed, split it through a real shell
//!   ([`support::shell_words`]), run it verbatim, and read the named state back cleared.
//!
//! # The one exclusion, and why it is itself fenced
//!
//! `migrate-corpus.<cause>` is the door's **finding-code namespace** (`migrate-corpus.deferred`,
//! `migrate-corpus.prose-needed`, …), never a command name — a route names the verb as
//! `jigc migrate-corpus`. Those occurrences are skipped, and
//! [`the_excluded_occurrences_are_all_finding_codes`] proves the skip cannot swallow anything
//! else: every excluded occurrence must be followed by `.` and a lowercase identifier.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support;
use crate::support::{frozen_pack, rust_source};

// ---------------------------------------------------------------------------------------------
// The registry.
// ---------------------------------------------------------------------------------------------

/// What a site that names `jigc migrate-corpus` *is*, and what was done about it.
#[derive(Clone, Copy)]
enum Disposition {
    /// A route whose **emitted bytes** an arm in this suite runs against a real corpus, reading
    /// the named state back cleared. Carries the arm's test name.
    Driven(&'static str),
    /// A route to the verb whose firing state the widened walk answers — argued from the state,
    /// not driven here (the reason says why driving it would prove nothing this suite's two
    /// driven arms do not already prove).
    Answered(&'static str),
    /// Not a command route at all: a registry key, a clap leaf path, help prose, a message
    /// clause, a fence witness — or a line that names the verb as what *cannot* fix the state.
    NotACommandRoute(&'static str),
}

/// One production site group, keyed `(file, enclosing fn)` with the number of occurrences in it.
///
/// **The key is the enclosing `fn` as [`rust_source::enclosing_fn`] reads it** — the last `fn`
/// declared textually before the site — so an occurrence inside a `const` table reports the
/// function above that table. That is a key, not a claim; each row's own prose names what the
/// site actually is, and the fence prints line numbers when a group moves.
struct RouteSite {
    file: &'static str,
    func: &'static str,
    sites: usize,
    /// The repository/corpus state in which this site's text reaches a surface.
    state: &'static str,
    disposition: Disposition,
}

/// Every production site naming the verb, with the state it fires in and its disposition.
const MIGRATE_ROUTE_SITES: &[RouteSite] = &[
    // -- the two driven rows ------------------------------------------------------------------
    RouteSite {
        file: "crates/cli/src/relocate.rs",
        func: "relocate_freeze_exempt",
        sites: 1,
        state: "`jigc relocate <frozen-ty> --from <prior home>` over a corpus whose frozen \
                doctype moved home and whose instance is still committed at the prior one — the \
                freeze-exempt path refuses and hands the act to the version-gated verb",
        disposition: Disposition::Driven("the_relocate_refusal_routes_to_a_verb_that_acts"),
    },
    RouteSite {
        file: "crates/engine/src/validate.rs",
        func: "version_currency_break",
        sites: 1,
        state: "the store sweep over a managed committed doc stamped below its doctype's \
                manifest version — `schema-conformance.schema-version-current`, whose route is \
                the repair line an operator follows",
        disposition: Disposition::Driven("the_store_sweep_repair_line_clears_the_store"),
    },
    RouteSite {
        file: "crates/cli/src/render.rs",
        func: "unmigrated_corpus_trailer",
        sites: 1,
        state: "the same sweep's closing trailer — one surface, so the trailer and the finding \
                above it must name the same command and the same promise (*then re-validate*)",
        disposition: Disposition::Driven("the_store_sweep_repair_line_clears_the_store"),
    },
    // -- routes whose firing state the widened walk answers -------------------------------------
    RouteSite {
        file: "crates/cli/src/setup.rs",
        func: "binary_mismatch_finding",
        sites: 1,
        state: "`jigc setup` over a store last written by a different jigc **and** a committed \
                corpus stale against this binary's schemas — the same below-version condition \
                the driven store-sweep row clears, reached through the install door",
        disposition: Disposition::Answered(
            "its `corpus_stale` leg is the store sweep's own predicate (`setup.rs` asks \
             `validate`'s version-currency answer), so the state is the driven row's state and \
             the verb clears it identically",
        ),
    },
    RouteSite {
        file: "crates/cli/src/ingest.rs",
        func: "near_miss_route",
        sites: 1,
        state: "`jigc ingest` over a file at a managed home that is **already registered** — \
                adoption is not the repair, and the parenthetical names the verb for the one \
                sub-case that is (a corpus left on an older schema-version)",
        disposition: Disposition::Answered(
            "a registered doc at a managed home stamped below current is exactly the driven \
             store-sweep state; the clause routes to the same verb for the same state and this \
             door adds no corpus condition of its own",
        ),
    },
    // -- `migrate-corpus`'s own re-run instructions ---------------------------------------------
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "run",
        sites: 2,
        state: "a `migrate-corpus` commit the repository's own hook rejected — the survivable \
                rejection frame's `target` and its copy-runnable `rerun` argv",
        disposition: Disposition::Answered(
            "the frame is emitted **by** the run it tells you to repeat, so the state it names \
             is one `migrate-corpus` answers by construction; the walk cannot make it false",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "future_stamp_finding",
        sites: 1,
        state: "a doc stamped **above** the current manifest version, met inside the fold",
        disposition: Disposition::Answered(
            "a `migrate-corpus` re-run instruction, emitted inside a `migrate-corpus` run once \
             the stamp is restored — the door re-answers its own refusal",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "missing_snapshot_finding",
        sites: 1,
        state: "the per-doc half: a doc carrying an out-of-band `schema-version: 0`, for which \
                no `<ty>.v0.yaml` can exist (T2)",
        disposition: Disposition::Answered(
            "a `migrate-corpus` re-run instruction behind a pack-authoring act; the run that \
             emits it is the run that repeats",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "enumeration_missing_snapshot_finding",
        sites: 1,
        state: "the enumeration half: a prior snapshot the store cannot answer for, so the \
                homes that version declared cannot be walked at all (T2)",
        disposition: Disposition::Answered(
            "the fail-closed sibling of the row above, and the one refusal the widened walk \
             itself mints — its re-run is the same run",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "unclassifiable_change_finding",
        sites: 1,
        state: "the empty-diff backstop: a structural shape change the schema-diff classifies \
                as no transform kind",
        disposition: Disposition::Answered(
            "a schema-authoring repair followed by a re-run of the emitting door",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "narrowed_cardinality_finding",
        sites: 1,
        state: "a pre-fold by-design refusal: the new schema narrows a section's cardinality \
                below what the committed corpus carries",
        disposition: Disposition::Answered(
            "a schema-authoring repair followed by a re-run of the emitting door",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "removed_field_finding",
        sites: 1,
        state: "a pre-fold by-design refusal: the new schema drops a field the committed corpus \
                carries",
        disposition: Disposition::Answered(
            "a schema-authoring repair followed by a re-run of the emitting door",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "removed_item_slot_finding",
        sites: 1,
        state: "a pre-fold by-design refusal: the new schema drops an item slot the committed \
                corpus carries",
        disposition: Disposition::Answered(
            "a schema-authoring repair followed by a re-run of the emitting door",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "halt_finding",
        sites: 7,
        state: "the fold halted on one doc — the parse, `id-from` remap, map-gap, unmodelled \
                item content, unclassified and generate/splice arms, each naming its own repair \
                before the re-run",
        disposition: Disposition::Answered(
            "seven re-run instructions inside the emitting run; none of them names a corpus \
             condition the walk decides",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "prose_needing_route",
        sites: 1,
        state: "the Framing-A handoff: the folded bytes do not gate clean until an agent \
                authors the newly required prose",
        disposition: Disposition::Answered(
            "an authoring act followed by a re-run of the emitting door — the one halt an \
             author can clear from the doc",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "destination_collision_finding",
        sites: 1,
        state: "both homes populated: a relocating instance whose destination already holds a \
                *different* document — the hazard the widened walk itself widens",
        disposition: Disposition::Answered(
            "driven as a state, with its finding and its no-clobber guarantee, by T1's \
             `a_destination_already_populated_blocks_instead_of_clobbering`; the re-run here is \
             the emitting door's own",
        ),
    },
    RouteSite {
        file: "crates/cli/src/migrate_corpus.rs",
        func: "deferred_finding",
        sites: 1,
        state: "a doc left untouched behind the run's first blocker (WIP-safety)",
        disposition: Disposition::Answered(
            "the re-run is the emitting door's, gated on clearing the blocker reported above it",
        ),
    },
    // -- not command routes ---------------------------------------------------------------------
    RouteSite {
        file: "crates/cli/src/cli.rs",
        func: "migrate_corpus_long_about",
        sites: 1,
        state: "`jigc migrate-corpus --help`",
        disposition: Disposition::NotACommandRoute(
            "the verb's own long help, naming itself in prose",
        ),
    },
    RouteSite {
        file: "crates/cli/src/cli.rs",
        func: "leaf",
        sites: 1,
        state: "no surface — the clap leaf path `Command::MigrateCorpus` resolves to",
        disposition: Disposition::NotACommandRoute("a leaf-path key, not emitted text"),
    },
    RouteSite {
        file: "crates/cli/src/cli.rs",
        func: "run_orient",
        sites: 3,
        state: "no surface — three code-side registry keys naming the leaf (`VERB_KINDS`' row, \
                `BEHALF_DOORS`' door and its commit-on-behalf argv)",
        disposition: Disposition::NotACommandRoute(
            "registry keys, not emitted text; the `enclosing fn` key is the function above the \
             tables",
        ),
    },
    RouteSite {
        file: "crates/cli/src/render.rs",
        func: "validation_store",
        sites: 1,
        state: "no surface — `STORE_EXIT_FLIPS`' `unmigrated-corpus` **witness**, a hand-built \
                copy of the production finding the exit-flip fences drive",
        disposition: Disposition::NotACommandRoute(
            "a fence witness; the live text an operator reads is the driven \
             `version_currency_break` row's",
        ),
    },
    RouteSite {
        file: "crates/cli/src/render.rs",
        func: "orientation_arm",
        sites: 1,
        state: "no surface — `ENVELOPE_ARMS`' path key for the verb's `Report` arm",
        disposition: Disposition::NotACommandRoute("a registry key, not emitted text"),
    },
    RouteSite {
        file: "crates/cli/src/doc.rs",
        func: "machine_maintained_field_guard",
        sites: 1,
        state: "`jigc doc set-field` on a machine-maintained field — the refusal's message \
                clause naming the only verb that advances the `schema-version` stamp",
        disposition: Disposition::NotACommandRoute(
            "a message clause, not the finding's route: it states who owns the field, and the \
             fact holds in every corpus state",
        ),
    },
    RouteSite {
        file: "crates/cli/src/invocation_log.rs",
        func: "<top level>",
        sites: 1,
        state: "no surface — `COMMITTING_DOORS`' verb label for the door's rejection error code",
        disposition: Disposition::NotACommandRoute("a registry label, not emitted text"),
    },
    RouteSite {
        file: "crates/engine/src/validate.rs",
        func: "ahead_route",
        sites: 1,
        state: "a committed doc stamped **above** the current manifest version — the store \
                sweep's `schema-conformance.schema-version-ahead`",
        disposition: Disposition::NotACommandRoute(
            "it names the verb as what **cannot** fix the state (*`jigc migrate-corpus` cannot \
             fix a future stamp*); the route it does give is human — upgrade jigc, or restore \
             the stamp from git history",
        ),
    },
];

// ---------------------------------------------------------------------------------------------
// The source reader.
// ---------------------------------------------------------------------------------------------

/// The cargo workspace root — the registry's `file` column is workspace-relative, because the
/// family reaches `crates/engine/src` too.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The token every route to the verb is spelled with.
const TOKEN: &str = "migrate-corpus";

/// One scanned occurrence.
struct Occurrence {
    file: String,
    func: String,
    line: usize,
    /// The byte that follows the token — the finding-code discriminator.
    tail: String,
}

/// Every production occurrence of [`TOKEN`] under the two source trees, comments blanked and
/// `#[cfg(test)]` bodies excluded ([`rust_source`]).
fn occurrences() -> Vec<Occurrence> {
    let root = workspace_root();
    let mut out = Vec::new();
    for tree in ["crates/cli/src", "crates/engine/src"] {
        for path in rust_source::rust_files(&root.join(tree)) {
            let body = fs::read_to_string(&path).expect("read a source file");
            let code = rust_source::code_and_strings(&body);
            let regions = rust_source::cfg_test_regions(&code);
            let rel = path
                .strip_prefix(&root)
                .expect("a workspace-relative path")
                .to_string_lossy()
                .replace('\\', "/");
            for (at, _) in code.match_indices(TOKEN) {
                if rust_source::is_test_domain(&path, &regions, at) {
                    continue;
                }
                out.push(Occurrence {
                    file: rel.clone(),
                    func: rust_source::enclosing_fn(&code, at)
                        .unwrap_or("<top level>")
                        .to_string(),
                    line: body[..at].lines().count(),
                    tail: code[at + TOKEN.len()..].chars().take(2).collect::<String>(),
                });
            }
        }
    }
    out
}

/// Whether an occurrence is the door's **finding-code namespace** (`migrate-corpus.<cause>`)
/// rather than the command name.
fn is_finding_code(occ: &Occurrence) -> bool {
    let mut chars = occ.tail.chars();
    chars.next() == Some('.') && chars.next().is_some_and(|c| c.is_ascii_lowercase())
}

// ---------------------------------------------------------------------------------------------
// The fence.
// ---------------------------------------------------------------------------------------------

/// **Every production site naming the verb carries a stated row.**
///
/// Read in both directions: a site the registry does not name is an unstated member of the
/// family, and a row the source no longer has is a claim about code that moved. The counts are
/// checked too, so an added route inside an already-listed producer cannot slip in under a row
/// that was written for its neighbour.
#[test]
fn every_production_site_naming_the_verb_has_a_stated_row() {
    let scanned: Vec<Occurrence> = occurrences()
        .into_iter()
        .filter(|occ| !is_finding_code(occ))
        .collect();

    let mut unstated: Vec<String> = Vec::new();
    let mut miscounted: Vec<String> = Vec::new();

    for occ in &scanned {
        let row = MIGRATE_ROUTE_SITES
            .iter()
            .find(|row| row.file == occ.file && row.func == occ.func);
        if row.is_none() {
            unstated.push(format!("  {}:{} in `{}`", occ.file, occ.line, occ.func));
        }
    }

    for row in MIGRATE_ROUTE_SITES {
        let hits: Vec<&Occurrence> = scanned
            .iter()
            .filter(|occ| occ.file == row.file && occ.func == row.func)
            .collect();
        if hits.len() != row.sites {
            miscounted.push(format!(
                "  {} in `{}`: the row says {} site(s), the source has {}{}",
                row.file,
                row.func,
                row.sites,
                hits.len(),
                if hits.is_empty() {
                    String::new()
                } else {
                    format!(
                        " (lines {})",
                        hits.iter()
                            .map(|o| o.line.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                },
            ));
        }
    }

    assert!(
        unstated.is_empty() && miscounted.is_empty(),
        "a route naming `jigc migrate-corpus` must say which state it fires in and what was \
         done about it (M52 Increment 7 / T4, settle-record D4.6) — unstated sites:\n{}\n\
         rows whose count moved:\n{}",
        if unstated.is_empty() {
            "  (none)".to_string()
        } else {
            unstated.join("\n")
        },
        if miscounted.is_empty() {
            "  (none)".to_string()
        } else {
            miscounted.join("\n")
        },
    );
}

/// **Every row says something.** A disposition with an empty reason is a row that was filled in
/// to silence the fence rather than to answer it.
#[test]
fn every_row_states_its_firing_state_and_its_disposition() {
    for row in MIGRATE_ROUTE_SITES {
        let what = format!("{} in `{}`", row.file, row.func);
        assert!(
            row.state.len() > 20,
            "{what}: the row must name the state its text fires in",
        );
        assert!(row.sites > 0, "{what}: a row covering no site is not a row");
        let reason = match row.disposition {
            Disposition::Driven(arm) => arm,
            Disposition::Answered(why) | Disposition::NotACommandRoute(why) => why,
        };
        assert!(
            !reason.trim().is_empty(),
            "{what}: the disposition must carry its reason",
        );
    }
}

/// **A `Driven` row names an arm that exists here and runs the emitted bytes.**
///
/// The registry's strongest claim is the two driven rows; nothing stops a row from *claiming*
/// to be driven, so the arm it names is looked up in this file's own source. Two distinct arms
/// is the increment's own count — a third would be welcome, and this asserts the floor rather
/// than an exact equality it would be wrong to pin.
#[test]
fn the_driven_rows_name_arms_that_exist_here() {
    let me = fs::read_to_string(
        workspace_root()
            .join("crates/cli/tests")
            .join("migrate_route_family.rs"),
    )
    .expect("read this suite's own source");

    let mut arms: Vec<&str> = MIGRATE_ROUTE_SITES
        .iter()
        .filter_map(|row| match row.disposition {
            Disposition::Driven(arm) => Some(arm),
            _ => None,
        })
        .collect();
    arms.sort_unstable();
    arms.dedup();

    for arm in &arms {
        assert!(
            me.contains(&format!("fn {arm}(")),
            "a `Driven` row names the arm `{arm}`, which this suite does not define",
        );
    }
    assert!(
        arms.len() >= 2,
        "the family's behavioural half is the two rows the increment names — \
         `relocate`'s refusal and the store sweep's repair line; got {arms:?}",
    );
}

/// **The one exclusion cannot swallow anything but a finding code.**
///
/// The scan skips `migrate-corpus.<cause>` as the door's own code namespace. That skip is only
/// safe while every skipped occurrence really is one, so each is re-read here: `.` followed by
/// a lowercase identifier, and nothing else.
#[test]
fn the_excluded_occurrences_are_all_finding_codes() {
    let excluded: Vec<Occurrence> = occurrences().into_iter().filter(is_finding_code).collect();
    assert!(
        !excluded.is_empty(),
        "the door mints `migrate-corpus.*` codes — an empty exclusion set means the reader \
         stopped seeing them, which would also mean it stopped seeing the routes",
    );
    for occ in &excluded {
        assert!(
            occ.tail.starts_with('.'),
            "{}:{}: excluded as a finding code, but the token is not followed by `.`",
            occ.file,
            occ.line,
        );
    }
}

// ---------------------------------------------------------------------------------------------
// The behavioural half: the fixture.
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-route-family-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// One driven corpus: a manufactured pack whose doctype bumps a version, a `$HOME`, and a repo
/// set up against it.
struct Corpus {
    pack: TempDir,
    home: TempDir,
    repo: TempDir,
    /// The version the committed corpus is stamped at (the pack's current version is this + 1).
    from_version: u32,
}

impl Corpus {
    fn new(
        tag: &str,
        ty: &str,
        prior: impl FnOnce(&str) -> String,
        current: impl FnOnce(&str) -> String,
    ) -> Self {
        let pack = TempDir::new(&format!("pack-{tag}"));
        let from_version = frozen_pack::bumped_pack(pack.path(), ty, prior, current);
        let home = TempDir::new(&format!("home-{tag}"));
        let repo = TempDir::new(&format!("repo-{tag}"));

        let root = repo.path();
        git(root, &["init", "-q"]);
        git(root, &["config", "user.email", "test@example.com"]);
        git(root, &["config", "user.name", "Test"]);
        fs::write(root.join("README.md"), "hello\n").expect("write file");
        git(root, &["add", "."]);
        git(root, &["commit", "-q", "-m", "initial"]);

        let corpus = Corpus {
            pack,
            home,
            repo,
            from_version,
        };
        let out = corpus.jigc(&["setup"]);
        assert!(
            out.status.success(),
            "`jigc setup` must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        corpus
    }

    fn jigc(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", self.pack.path())
            .output()
            .expect("run the jigc binary")
    }

    /// `jigc <args>`'s combined streams and whether it exited 0.
    fn run(&self, args: &[&str]) -> (String, bool) {
        let out = self.jigc(args);
        (
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            ),
            out.status.success(),
        )
    }

    /// Commit `body` at the repo-relative `path`.
    fn commit_doc(&self, path: &str, body: &str) {
        let full = self.repo.path().join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).expect("mk the doc's home");
        }
        fs::write(&full, body).expect("write the committed doc");
        git(self.repo.path(), &["add", "-A"]);
        git(self.repo.path(), &["commit", "-q", "-m", "seed the corpus"]);
    }

    fn exists(&self, path: &str) -> bool {
        self.repo.path().join(path).exists()
    }

    fn read(&self, path: &str) -> String {
        fs::read_to_string(self.repo.path().join(path))
            .unwrap_or_else(|err| panic!("read `{path}`: {err}"))
    }

    /// Run an **emitted** command line verbatim: split by a real shell
    /// ([`support::shell_words`]), `jigc` dropped from the front because the binary under test
    /// is this build's, and the streams returned.
    fn run_emitted(&self, cmd: &str) -> (String, bool) {
        let argv = support::shell_words(cmd, self.repo.path(), self.home.path());
        assert_eq!(
            argv.first().map(String::as_str),
            Some("jigc"),
            "the emitted command line leads `jigc`; got `{cmd}`",
        );
        let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
        self.run(&args)
    }
}

/// The first backticked span in `text` that names a `jigc` command — the **emitted** bytes, not
/// a reconstruction of them.
fn emitted_command(text: &str, what: &str) -> String {
    text.split('`')
        .skip(1)
        .step_by(2)
        .find(|span| span.starts_with("jigc "))
        .unwrap_or_else(|| {
            panic!("`{what}`: the surface must name a backticked `jigc …` command; got:\n{text}")
        })
        .to_string()
}

/// The shipped `adr`'s declared home line, and the shipped `changelog`'s.
const ADR_LOCATION: &str = "location: decisions/\n";
const CHANGELOG_PLACEMENT: &str = "placement: { file: CHANGELOG.md }\n";

/// Replace `needle` with `replacement` exactly once, asserting it was there.
fn swap(body: &str, needle: &str, replacement: &str) -> String {
    let out = body.replacen(needle, replacement, 1);
    assert_ne!(body, out, "the schema must carry `{}`", needle.trim_end());
    out
}

/// A conformant `adr` body with `title`, stamped at `version`.
fn adr_body(version: u32, title: &str) -> String {
    format!(
        "\
---
status: accepted
date: 2026-06-25
schema-version: {version}
---

# {title}

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
"
    )
}

/// A conformant `changelog` body, stamped at `version`.
fn changelog_body(version: u32) -> String {
    format!(
        "\
---
schema-version: {version}
---

# Changelog

## Unreleased Changes

### changed  {{#changed}}

- the staging group

## Releases

### 1.0.0  {{#1-0-0}}

<!-- fields -->
- date: 2026-06-14

#### added  {{#added}}

- the nested group
"
    )
}

// ---------------------------------------------------------------------------------------------
// The behavioural half: the two driven rows.
// ---------------------------------------------------------------------------------------------

/// **`relocate`'s frozen-doctype refusal routes to a verb that acts** (D4.5).
///
/// The freeze-exempt relocation path refuses a frozen doctype and hands the act to
/// `jigc migrate-corpus`. Through the pre-Increment-7 walk that hand-off went nowhere for this
/// exact corpus — a `placement:`→`placement:` move, the cell whose prior branch dropped every
/// prior placement home — so the operator ran the named verb and got `0 migrated, 0 already
/// current, 0 blocked` at exit 0 with the instance still committed at the old home.
///
/// So the route is **run**, not read: the emitted command is lifted out of the refusal the
/// binary printed, split by a real shell, executed verbatim, and the state the refusal named —
/// an instance stranded at the prior home of a frozen doctype — is read back cleared.
#[test]
fn the_relocate_refusal_routes_to_a_verb_that_acts() {
    let corpus = Corpus::new(
        "relocate",
        "changelog",
        |shipped| shipped.to_string(),
        |shipped| {
            swap(
                shipped,
                CHANGELOG_PLACEMENT,
                "placement: { file: HISTORY.md }\n",
            )
        },
    );
    corpus.commit_doc("CHANGELOG.md", &changelog_body(corpus.from_version));

    // The refusal, and the command it names.
    let (refusal, ok) = corpus.run(&["relocate", "changelog", "--from", "CHANGELOG.md"]);
    assert!(
        !ok,
        "a frozen doctype is refused on the freeze-exempt path; got:\n{refusal}",
    );
    assert!(
        refusal.contains("frozen doctype"),
        "the refusal says why it refuses; got:\n{refusal}",
    );
    let emitted = emitted_command(&refusal, "relocate's frozen-doctype refusal");
    assert_eq!(
        emitted, "jigc migrate-corpus",
        "the refusal hands the act to the version-gated verb",
    );

    // The state the refusal named, before: the instance is committed at the prior home and
    // nothing sits at the current one.
    assert!(corpus.exists("CHANGELOG.md"));
    assert!(!corpus.exists("HISTORY.md"));

    // Run the emitted bytes verbatim.
    let (out, ok) = corpus.run_emitted(&emitted);
    assert!(ok, "the routed command runs clean; it said:\n{out}");

    // And the state is cleared: the instance landed at the current home, prose intact, the
    // prior home vacated, and the store validates.
    assert!(
        !corpus.exists("CHANGELOG.md"),
        "the prior home is vacated; the run said:\n{out}",
    );
    let landed = corpus.read("HISTORY.md");
    assert!(
        landed.contains("- the staging group"),
        "the committed prose survives the move; landed:\n{landed}",
    );
    assert!(
        landed.contains(&format!("schema-version: {}", corpus.from_version + 1)),
        "the stamp flips to the current version; landed:\n{landed}",
    );
    let (after, after_ok) = corpus.run(&["validate"]);
    assert!(
        after_ok,
        "nothing is stranded once the routed verb has run; validate said:\n{after}",
    );
}

/// **The store sweep's repair line clears the store, re-validate included** (D4.6).
///
/// `schema-conformance.schema-version-current` routes *"run `jigc migrate-corpus` to upgrade
/// it"*, and the sweep's trailer closes *"run `jigc migrate-corpus`, then re-validate."* — a
/// promise about the **store**, not about one document. Through the pre-Increment-7 walk that
/// promise was false whenever any instance of the doctype sat at a prior home: the run cleared
/// the doc at the current home, left the stranded one untouched, and the re-validate the
/// trailer told the operator to perform came back red with a different diagnosis
/// (`schema-conformance.orphaned-instance`) and a different route.
///
/// So this corpus carries both: one `adr` at the current home stamped below (which is what
/// mints the finding) and one at the prior home the walk now reaches. The finding's **own**
/// route is lifted from the machine surface, run verbatim, and the promise is read back — the
/// whole store validates, with neither diagnosis standing.
///
/// **What this arm asserted until the M52 completion audit's fix 2, and why the change is the
/// promise getting stronger rather than weaker.** Increment 7 widened the *walk* and left the
/// *store sweep* keyed on the resolved home, so the stranded doc was still named by the wrong
/// finding — `schema-conformance.orphaned-instance`, *no resolved doctype claims this path*,
/// said of a doctype `jigc describe` still lists — and this arm pinned exactly that as the
/// pre-state. The sweep now enumerates the same recorded prior homes the walk does, so **both**
/// instances draw the diagnosis that is true of them, both carry the one route, and
/// `orphaned-instance` fires on neither. The trailer's *then re-validate* was the half that was
/// already honest; the half that was not is the report the operator reads **before** running it.
#[test]
fn the_store_sweep_repair_line_clears_the_store() {
    let corpus = Corpus::new(
        "sweep",
        "adr",
        |shipped| shipped.to_string(),
        |shipped| swap(shipped, ADR_LOCATION, "location: adrs/\n"),
    );
    let at_current = "docs/adrs/cache-sessions-in-memory.md";
    let at_prior = "docs/decisions/retire-the-nightly-job.md";
    corpus.commit_doc(
        at_current,
        &adr_body(corpus.from_version, "Cache sessions in memory"),
    );
    corpus.commit_doc(
        at_prior,
        &adr_body(corpus.from_version, "Retire the nightly job"),
    );

    // The sweep, on the machine surface: BOTH below-version docs carry the repair line — the
    // one at the current home and the one at the prior home — and neither is called an orphan.
    let out = corpus.jigc(&["validate", "--format", "json"]);
    assert!(
        !out.status.success(),
        "an unmigrated corpus flips the store exit",
    );
    let report: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("`jigc validate --format json` emits JSON");
    let findings = report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the store report carries findings; got:\n{report:#}"));
    let named: Vec<&str> = findings
        .iter()
        .filter(|f| f["code"] == "schema-conformance.schema-version-current")
        .filter_map(|f| f["key"]["target"].as_str())
        .collect();
    assert_eq!(
        named,
        vec!["adr:cache-sessions-in-memory", "adr:retire-the-nightly-job"],
        "the repair line is a promise about the STORE, so every below-version instance the \
         routed verb will land must be named by it — the one at the current home and the one \
         at the prior home the walk reaches; report:\n{report:#}",
    );
    let currency = findings
        .iter()
        .find(|f| f["code"] == "schema-conformance.schema-version-current")
        .unwrap_or_else(|| panic!("the below-version doc is named; report:\n{report:#}"));
    assert!(
        !findings
            .iter()
            .any(|f| f["code"] == "schema-conformance.orphaned-instance"),
        "and neither is called an orphan: an orphan's doctype left the composition, while \
         `adr` is composed at this very commit and only its home moved; report:\n{report:#}",
    );

    let route = currency["route"]
        .as_str()
        .unwrap_or_else(|| panic!("the finding carries a route; got:\n{currency:#}"));
    let emitted = emitted_command(route, "the version-currency repair line");
    assert_eq!(emitted, "jigc migrate-corpus");

    // The trailer on the text surface names the same command and makes the same promise.
    let (text, _) = corpus.run(&["validate"]);
    assert!(
        text.contains("run `jigc migrate-corpus`, then re-validate"),
        "the sweep's trailer promises the store, not one doc; validate said:\n{text}",
    );
    assert_eq!(
        emitted_command(&text, "the unmigrated-corpus trailer"),
        emitted,
        "the trailer and the finding name one command",
    );

    // Run the emitted bytes verbatim.
    let (migrated, ok) = corpus.run_emitted(&emitted);
    assert!(ok, "the routed command runs clean; it said:\n{migrated}");

    // The promise, kept: both instances are at the current home, and re-validate is clean.
    assert!(
        corpus.exists(at_current) && corpus.exists("docs/adrs/retire-the-nightly-job.md"),
        "the whole doctype lands at the current home; the run said:\n{migrated}",
    );
    assert!(
        !corpus.exists(at_prior),
        "the prior home is vacated; the run said:\n{migrated}",
    );
    let (after, after_ok) = corpus.run(&["validate"]);
    assert!(
        after_ok,
        "*then re-validate* is a promise about the store — it must come back clean; validate \
         said:\n{after}",
    );
    for stale in [
        "schema-conformance.schema-version-current",
        "schema-conformance.orphaned-instance",
    ] {
        assert!(
            !after.contains(stale),
            "`{stale}` must not survive the routed run; validate said:\n{after}",
        );
    }
}
