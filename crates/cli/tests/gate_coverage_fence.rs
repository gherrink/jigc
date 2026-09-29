//! **The gate-coverage fence** (M46 Inc 6 / T2 — `DECISIONS.md` → 2026-08-18 M46
//! planned, F-E: *"the exclusion list becomes **generated rather than hand-listed**
//! so the enumeration cannot go stale again"*).
//!
//! What `jigc task validate` previews — and what only `jigc task finalize` decides —
//! is stated on eight surfaces, and every one of those enumerations was written by
//! hand. `cli::gate_coverage::GATE_COVERAGE` is now the single source; this suite is
//! what makes it binding: each site declares the **tiers** it enumerates, and must
//! carry every member of them.
//!
//! **Per token, not verbatim.** The five sentences legitimately differ — the `task`
//! tip compresses the carryover gate to *"carryover"*, the catalog hint compresses
//! the whole clause to one line — and
//! [surface-contract.md](../../../design/surface-contract.md) → the M43 law-3
//! fence-depth **re-open** is exactly this lesson: verbatim equality *"bought
//! unrepresentable drift"* once one statement varied by context. So this is the M48
//! named-fact-guard shape (`cli::pack::CONSTRAINT_REQUIRED_TOKENS`), one table over.
//!
//! **Emitted bytes where a surface emits.** Four of the eight sites are read out of
//! the **real binary's** stdout rather than out of the source that produces it — the
//! composed `what's-left:` line and both packs' `finalize` step bodies through
//! `jigc start`, the `task` tip through a read-shaped guess, the `validate-task`
//! catalog hint through `jigc describe`. A fence over the source file would pass
//! while the bytes an agent reads were broken. The remaining four are documents, and
//! are read as documents — each through a **region** anchored at its own paragraph,
//! so a token living elsewhere in the file cannot satisfy it.
//!
//! **And the fence can fail.** Every site × every member it owes is driven a second
//! time with that member's token cut out of the extracted text: the checker must
//! report exactly that member. That is the axis (site × member), not one witness —
//! a fence that only ever runs green is a grep wearing a badge.

use cli::gate_coverage::{self, GateCoverage, Tier};
use cli::render::CommitModel;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-gate-coverage-{tag}-{}-{:?}",
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

/// The repository root — the home of the documents and the methodology pack.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize repo root")
}

/// Initialize a real git repo with one commit (composition reads HEAD).
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
    // The cascade's project layer — `jigc setup` would write it, and composing
    // refuses without it. Creating it directly keeps this suite off the install path.
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// A second commit, so `HEAD` is the single-parent commit `jigc task amend` requires — its
/// `amend.head-shape` gate refuses a root commit, which is what [`init_repo`] leaves.
fn second_commit(root: &Path) {
    fs::write(root.join("NOTES.md"), "notes\n").expect("write the second file");
    for args in [
        vec!["add", "NOTES.md"],
        vec!["commit", "-q", "--no-verify", "-m", "the second commit"],
    ] {
        let out = Command::new("git")
            .args(&args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

/// Run `jigc <args>` and return stdout + stderr concatenated (a tip rides stderr).
fn run(repo: &Path, home: &Path, pack: Option<&Path>, args: &[&str]) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_jigc"));
    cmd.args(args).current_dir(repo).env("HOME", home);
    match pack {
        Some(dir) => {
            cmd.env("JIGC_PACK_DIR", dir);
        }
        None => {
            cmd.env_remove("JIGC_PACK_DIR");
        }
    }
    let out = cmd.output().expect("run jigc");
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// Slice `text` from the first occurrence of `from` through the end of the first
/// occurrence of `to` after it — the region a site's statement lives in.
fn region(text: &str, from: &str, to: &str, site: &str) -> String {
    let start = text
        .find(from)
        .unwrap_or_else(|| panic!("`{site}`: region start not found: {from:?}\nin:\n{text}"));
    let rest = &text[start..];
    let end = rest
        .find(to)
        .unwrap_or_else(|| panic!("`{site}`: region end not found: {to:?}\nin:\n{rest}"));
    rest[..end + to.len()].to_owned()
}

/// One statement of the coverage split, and the tiers it enumerates.
struct Site {
    /// What a fence failure names.
    name: &'static str,
    /// The tiers this site enumerates — it owes every member of them.
    tiers: &'static [Tier],
    /// The site's text, region-sliced to its own statement.
    text: String,
    /// The **commit model** this site's statement is about — which token each member is
    /// asked for (the F-10 review's MEDIUM-3). Every site but one is the ordinary model;
    /// the composed line of a `jigc task amend` task states the same split about an arm
    /// whose index gate is its own, and must name it in the arm's words.
    model: CommitModel,
}

impl Site {
    /// The members this site owes, table order.
    fn owed(&self) -> Vec<&'static GateCoverage> {
        self.tiers
            .iter()
            .flat_map(|tier| gate_coverage::members(*tier))
            .collect()
    }
}

/// Every coverage site, its text taken from the surface that actually emits it.
///
/// Five binary runs: one dev-pack compose (which carries **two** sites — the composed
/// `what's-left:` line and the dev `finalize` step body), one `jigc task amend` mint (the
/// same composed line on the **amend** commit model), one methodology compose, one
/// read-shaped `task` guess, one `describe`.
fn sites() -> Vec<Site> {
    let root = repo_root();
    let home = TempDir::new("home");
    let dev = TempDir::new("dev");
    let meth = TempDir::new("meth");
    init_repo(dev.path());
    init_repo(meth.path());
    // `jigc task amend` refuses a **root** HEAD (`amend.head-shape`), and `init_repo` leaves
    // exactly one commit — so the amend site needs a second one before it can be composed.
    second_commit(dev.path());

    let composed = run(
        dev.path(),
        home.path(),
        None,
        &["start", "--workflow", "single-task", "add a thing"],
    );
    let amend = run(
        dev.path(),
        home.path(),
        None,
        &["task", "amend", "repair the message"],
    );
    let methodology = run(
        meth.path(),
        home.path(),
        Some(Path::new(cli::pack_path!(methodology))),
        &["start", "--workflow", "dev-task", "add a thing"],
    );
    let tip = run(dev.path(), home.path(), None, &["task", "status"]);
    let describe = run(dev.path(), home.path(), None, &["describe"]);

    let doc = |rel: &str| fs::read_to_string(root.join(rel)).expect("read repo document");

    vec![
        Site {
            name: "the composed `what's-left:` line (render.rs, generated)",
            tiers: &[Tier::Previewed, Tier::LaterSummary],
            text: region(
                &composed,
                "what's-left: `jigc task validate",
                "\n",
                "what's-left",
            ),
            model: CommitModel::Index,
        },
        Site {
            // The same generated sentence, about the **other** commit model (the F-10
            // review's MEDIUM-3): an amend task's index gate is `finalize.amend-index-dirty`
            // over the whole index with no `--carry-staged`, so naming it *"the carryover
            // gate"* named a check that does not answer for this task. One member, two
            // spellings — and this site is what makes the second one binding.
            name: "the composed `what's-left:` line of an AMEND task (render.rs, generated)",
            tiers: &[Tier::Previewed, Tier::LaterSummary],
            text: region(
                &amend,
                "what's-left: `jigc task validate",
                "\n",
                "what's-left (amend)",
            ),
            model: CommitModel::Amend,
        },
        Site {
            name: "the dev pack's `finalize` step (composed)",
            tiers: &[Tier::Previewed, Tier::LaterSummary],
            text: region(
                &composed,
                "To see what's left before committing",
                "at finalize.",
                "dev finalize step",
            ),
            model: CommitModel::Index,
        },
        Site {
            name: "the methodology pack's `finalize` step (composed)",
            tiers: &[Tier::Previewed, Tier::LaterSummary],
            text: region(
                &methodology,
                "To see what's left before committing",
                "at finalize.",
                "methodology finalize step",
            ),
            model: CommitModel::Index,
        },
        Site {
            name: "the `task` unknown-subcommand tip (cli.rs)",
            tiers: &[Tier::Previewed],
            text: region(&tip, "`jigc task validate <task-id>`", "\n", "task tip"),
            model: CommitModel::Index,
        },
        Site {
            name: "the `validate-task` catalog hint (`jigc describe`)",
            tiers: &[Tier::Previewed],
            text: region(
                &describe,
                "validate-task",
                "without committing.",
                "validate-task hint",
            ),
            model: CommitModel::Index,
        },
        Site {
            name: "QUICKSTART.md → the core loop",
            tiers: &[Tier::Previewed, Tier::LaterSummary],
            text: region(
                &doc("crates/cli/guides/QUICKSTART.md"),
                "You can preview part of what finalize will gate on",
                "not *this will commit*:",
                "QUICKSTART.md",
            ),
            model: CommitModel::Index,
        },
        Site {
            name: "design/command-output-contract.md → the exit-code taxonomy, third clause",
            tiers: &[Tier::Previewed, Tier::LaterCause],
            text: region(
                &doc("design/command-output-contract.md"),
                "**The third clause, and it is position, not scope:",
                "*nothing this side of the commit blocks it*.",
                "command-output-contract.md",
            ),
            model: CommitModel::Index,
        },
        Site {
            name: "design/finalize.md → 2. Validate",
            tiers: &[Tier::LaterCause],
            text: region(
                &doc("design/finalize.md"),
                "`finalize` has no private check path",
                "not *this will commit*.",
                "finalize.md",
            ),
            model: CommitModel::Index,
        },
    ]
}

/// **The fence.** Every site names every member of the tiers it enumerates.
///
/// Every site is checked before the verdict, so a redness reports the whole axis
/// rather than the first site that broke.
#[test]
fn every_site_names_every_member_of_the_tiers_it_enumerates() {
    let mut broken = Vec::new();
    for site in sites() {
        let missing = gate_coverage::unmet_in(&site.text, site.owed(), site.model);
        for row in missing {
            broken.push(format!(
                "{} does not name `{}` (token {:?})\n  text: {}",
                site.name,
                row.id,
                row.token(site.model),
                site.text
            ));
        }
    }
    assert!(
        broken.is_empty(),
        "coverage members missing from their sites:\n{}",
        broken.join("\n")
    );
}

/// **And the fence can fail — per site, per member.** Cutting one member's token out
/// of a site's own text must make the checker report exactly that member.
///
/// This is the axis (site × owed member), not one witness: it proves the fence is
/// load-bearing at every cell it claims to hold, so a member dropped from any single
/// enumeration reddens rather than passing on its neighbours' behalf.
#[test]
fn the_fence_reddens_when_a_site_drops_a_member() {
    let mut checked = 0usize;
    for site in sites() {
        for row in site.owed() {
            let token = row.token(site.model);
            let cut = strike(&site.text, token);
            assert_ne!(
                cut, site.text,
                "{}: token {token:?} was not present to cut",
                site.name,
            );
            let reported: Vec<&str> = gate_coverage::unmet_in(&cut, [row], site.model)
                .iter()
                .map(|r| r.id)
                .collect();
            assert_eq!(
                reported,
                vec![row.id],
                "{}: dropping `{}` must be reported",
                site.name,
                row.id
            );
            checked += 1;
        }
    }
    // The axis today is 66 cells (9 sites × the tiers each owes) — 45 before the
    // changelog gate joined [`Tier::Previewed`], 51 before the repository posture did
    // (M52 Increment 3 / T6), and 58 before the amend model's own composed line became a
    // site of its own (the F-10 review's MEDIUM-3, +8: five previewed members and three
    // later-summary ones). Each is one cell per site owing that tier. The floor guards
    // against the axis silently collapsing — a site whose region stopped resolving, or a
    // tier that lost its members, would otherwise pass as a vacuous green.
    assert!(checked >= 66, "the mutation axis ran only {checked} cells");
}

/// Remove **every** occurrence of `token` from the normalized view's perspective:
/// the text is normalized first (the comparison view), then each hit deleted, so a
/// token that wraps a line in the raw source is still struck.
fn strike(text: &str, token: &str) -> String {
    let normalized = normalized(text);
    normalized.replace(token, "")
}

/// The comparison view the checker uses, mirrored here so the mutation is applied in
/// the same space the fence measures in.
fn normalized(text: &str) -> String {
    let mut out = String::new();
    let mut pending_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        out.push(ch.to_ascii_lowercase());
    }
    out
}

/// The generated line is the source's own rendering — not a second copy of the
/// sentence living in a test.
///
/// The composed bytes an agent reads must **contain the generated fragment
/// verbatim**; if `render.rs` ever grows a hand-written sentence beside the table,
/// this is what catches it.
#[test]
fn the_composed_line_carries_the_generated_fragment_verbatim() {
    let home = TempDir::new("gen-home");
    let dev = TempDir::new("gen-dev");
    init_repo(dev.path());
    let composed = run(
        dev.path(),
        home.path(),
        None,
        &["start", "--workflow", "single-task", "add a thing"],
    );
    let line = composed
        .lines()
        .find(|line| line.starts_with("what's-left: "))
        .expect("the composed text carries a what's-left line");
    assert!(
        line.contains(&gate_coverage::whats_left_coverage(CommitModel::Index)),
        "the composed line must carry the generated fragment verbatim:\n  line: {line}\n  \
         generated: {}",
        gate_coverage::whats_left_coverage(CommitModel::Index),
    );
}

/// Every finalize-only member states **why** it cannot preview — the exclusion list
/// carries its reason with it, which is what stops it decaying into a sentence
/// someone has to remember to extend.
#[test]
fn every_finalize_only_member_states_why_it_cannot_preview() {
    let later: Vec<&GateCoverage> = gate_coverage::members(Tier::LaterSummary)
        .chain(gate_coverage::members(Tier::LaterCause))
        .collect();
    assert!(!later.is_empty(), "the exclusion list is empty");
    for row in later {
        match row.door {
            cli::gate_coverage::Door::FinalizeOnly(reason) => {
                assert!(!reason.stated().is_empty(), "`{}` states no reason", row.id)
            }
            cli::gate_coverage::Door::Previewed(_) => {
                panic!(
                    "`{}` is stated at a later tier but marked previewed",
                    row.id
                )
            }
        }
    }
}
