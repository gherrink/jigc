//! The pre-trial surface-polish batch A (2026-07-17,
//! `completions/artifacts/M43/surface-comprehension-review.md`) — the pack-prose
//! half, driven through the REAL binary where the rc.7 corpus captured the old
//! surface:
//!
//! - **A1 — the catalog names the decision's destination.** Both comprehension
//!   reads flagged the decided/single/dev-task `when:` tie: the discriminating
//!   axis — where a recorded decision *lands* (running decisions log vs ADR vs
//!   nowhere) — was invisible at the router tier, as was the documented-code
//!   axis (`quick-fix` named it; `dev-task` vs `single-task` did not).
//! - **A2 — the summary-no-prefix statement.** The reproduced `docs: docs:`
//!   doubled subject committed clean because no surface stated that the subject
//!   renders `<type>(<scope>): <summary>` — the summary-soliciting steps now
//!   state it above the solicit (law 3: nothing ambushes).
//! - **A3 — the heading-depth ceiling reaches the fresh flows.** The
//!   seam-generated statement rode only the `{{schema:}}` projection (migrate
//!   templates); the ordinary fresh-authoring solicits were silent. The steps
//!   now carry the rule as hand-prose, fenced here. **M45 inc-2 T6** made the
//!   ceiling address-parameterized, so a step (composed before any address is
//!   chosen) states the schema-relative *rule* and the fence moved from
//!   byte-equality to fragment presence — plus a repo-wide census that no
//!   shipped step still claims `####` is safe unconditionally.
//! - **A5 — retire mechanics.** "RETIRE the foreign original" now states the
//!   mechanism (deleted, deletion staged, lands in the approving finalize's own
//!   commit — what `retire` + `stage_migration` actually do).

use crate::support::root_walk;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-surface-polish-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` against the **embedded** packs, asserting exit 0 and
/// returning stdout.
fn jigc_ok(repo: &Path, home: &Path, args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("spawn the jigc binary");
    assert!(
        out.status.success(),
        "`jigc {args:?}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// A fresh repo + isolated HOME, `jigc setup` run.
fn setup(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(&format!("repo-{tag}"));
    let home = TempDir::new(&format!("home-{tag}"));
    init_repo(repo.path());
    jigc_ok(repo.path(), home.path(), &["setup"]);
    (repo, home)
}

/// Compose one `migrate-<doctype>` workflow through **its own declared door** —
/// `jigc migrate <path> --as <doctype>` over a freshly committed foreign source.
///
/// Since M52 Increment 9 / T2 that door is the only one that composes it: a workflow
/// whose `suppressed:` block declares a `door:` is refused at both compose-by-name
/// doors, the verb being what stages the source its body rewrites.
fn composed_migration(repo: &Path, home: &Path, doctype: &str) -> String {
    let rel = format!("foreign-{doctype}.md");
    fs::write(
        repo.join(&rel),
        format!("# Foreign {doctype}\n\n## Section\n\nSome prose.\n"),
    )
    .expect("write the foreign source");
    // Committed, not merely written: `jigc migrate` refuses a source git has never
    // recorded.
    git(repo, &["add", &rel]);
    git(repo, &["commit", "-q", "-m", "the foreign source"]);
    jigc_ok(repo, home, &["migrate", &rel, "--as", doctype])
}

/// Collapse whitespace runs to single spaces — prose in YAML steps and clap
/// help is wrapped, so a sentence assertion must be wrap-insensitive.
fn collapsed(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ---------------------------------------------------------------------------
// A1 — the catalog names the decision's destination.
// ---------------------------------------------------------------------------

/// The bare-`jigc start` catalog line of one workflow (`- <id> — <when>`).
fn catalog_line<'a>(catalog: &'a str, workflow: &str) -> &'a str {
    let marker = format!("- {workflow} — ");
    catalog
        .lines()
        .find(|line| line.trim_start().starts_with(&marker))
        .unwrap_or_else(|| panic!("the catalog must list `{workflow}`; got:\n{catalog}"))
}

#[test]
fn catalog_when_lines_name_the_decision_destination_and_documented_code_axes() {
    let (repo, home) = setup("catalog");
    let catalog = jigc_ok(repo.path(), home.path(), &["start"]);

    // decided-task: the decision lands on the running decisions log.
    let decided = catalog_line(&catalog, "decided-task");
    assert!(
        decided.contains("decisions log"),
        "`decided-task`'s when-line must name its decision destination (the \
         running decisions log); got: {decided}"
    );

    // single-task: decisions land as ADRs. (The documented-code axis lives in
    // its `usage:` — the dev pack's when-lines are pairwise-non-overlapping by
    // fence, and `quick-fix` already owns "documented code" there — asserted
    // through `describe` below.)
    let single = catalog_line(&catalog, "single-task");
    assert!(
        single.contains("ADR"),
        "`single-task`'s when-line must name its decision destination (ADRs); \
         got: {single}"
    );

    // dev-task: records none, and stays off documented code.
    let dev = catalog_line(&catalog, "dev-task");
    assert!(
        dev.contains("no decision"),
        "`dev-task`'s when-line must keep the no-decision axis; got: {dev}"
    );
    assert!(
        dev.contains("documented code"),
        "`dev-task`'s when-line must carry the documented-code axis; got: {dev}"
    );

    // The fuller story: single-task's `usage:` (projected by `describe`) names
    // the documented-code pick positively — the nearest-neighbour handoff.
    let describe = jigc_ok(repo.path(), home.path(), &["describe"]);
    let flat = collapsed(&describe);
    let single_at = flat
        .find("single-task is ")
        .expect("`describe` narrates single-task");
    // Bound the slice at the next workflow's narration (`sub-task` follows
    // `single-task` in the alphabetical walk).
    let end = flat.find("sub-task is ").unwrap_or(flat.len());
    let single_narration = &flat[single_at..end];
    assert!(
        single_narration.contains("documented code"),
        "single-task's `usage:` must carry the documented-code axis; got: {single_narration}"
    );
}

// ---------------------------------------------------------------------------
// A2 — the summary-no-prefix statement, above the solicit.
// ---------------------------------------------------------------------------

/// The subject-shape statement's load-bearing fragments: the rendered shape and
/// the no-own-prefix rule. Asserted as fragments (not one byte-string) so the
/// steps can phrase around them, while the shape claim itself stays pinned.
const SUBJECT_SHAPE: &str = "`<type>(<scope>): <summary>`";
const NO_PREFIX: &str = "without a type or scope prefix";

/// Assert `composed` states the subject shape BEFORE the summary solicit.
fn assert_no_prefix_statement_above_solicit(composed: &str, workflow: &str) {
    let flat = collapsed(composed);
    let shape_at = flat.find(SUBJECT_SHAPE).unwrap_or_else(|| {
        panic!(
            "the `{workflow}` compose must state the rendered subject shape \
             {SUBJECT_SHAPE}; got:\n{composed}"
        )
    });
    assert!(
        flat.contains(NO_PREFIX),
        "the `{workflow}` compose must state the summary is written \
         {NO_PREFIX}; got:\n{composed}"
    );
    let solicit_at = flat.find("#summary --from-file -").unwrap_or_else(|| {
        panic!("the `{workflow}` compose solicits the summary; got:\n{composed}")
    });
    assert!(
        shape_at < solicit_at,
        "the subject-shape statement must sit ABOVE the summary solicit \
         (law 3: stated before it can fail) in the `{workflow}` compose"
    );
}

#[test]
fn single_task_compose_states_the_summary_no_prefix_rule() {
    let (repo, home) = setup("noprefix-single");
    let composed = jigc_ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add a rate limiter"],
    );
    assert_no_prefix_statement_above_solicit(&composed, "single-task");
}

#[test]
fn dev_task_compose_states_the_summary_no_prefix_rule() {
    let (repo, home) = setup("noprefix-dev");
    let composed = jigc_ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "dev-task", "add request logging"],
    );
    assert_no_prefix_statement_above_solicit(&composed, "dev-task");
}

// ---------------------------------------------------------------------------
// A3 — the heading-depth ceiling reaches the fresh authoring flows.
// ---------------------------------------------------------------------------

/// The composed fresh flows state the slot heading-depth ceiling above their
/// set-slot solicits — the statement is hand-prose in the step YAML.
///
/// **Relaxed at M45 inc-2 T6**: a step's prose is composed before any address is
/// chosen, so it can only state the schema-relative *rule* — byte-equality against
/// the engine statement is no longer available (that statement now exists only per
/// address, and pinning the step to a global one is exactly what re-mints the lie).
/// The fence still bites: the rule's invariant fragments must be present, above the
/// solicit, in every flow (`design/surface-contract.md` → The stated-at fence, the
/// M45 census table).
#[test]
fn fresh_authoring_composes_state_the_heading_ceiling() {
    let statement = "the reserved heading depths are schema-relative";
    for (workflow, intent, tag) in [
        ("single-task", "add a rate limiter", "ceiling-single"),
        ("dev-task", "add request logging", "ceiling-dev"),
        ("decided-task", "switch the cache to LRU", "ceiling-decided"),
    ] {
        let (repo, home) = setup(tag);
        let composed = jigc_ok(
            repo.path(),
            home.path(),
            &["start", "--workflow", workflow, intent],
        );
        let flat = collapsed(&composed);
        let ceiling_at = flat.find(statement).unwrap_or_else(|| {
            panic!(
                "the `{workflow}` compose must state the schema-relative ceiling \
                 rule (`{statement}`); got:\n{composed}"
            )
        });
        assert!(
            flat[ceiling_at..].contains("shallowest depth free"),
            "the rule must name the recovery a rejected write prints; got:\n{composed}"
        );
        let solicit_at = flat
            .find("set-slot")
            .expect("a fresh flow solicits at least one set-slot");
        assert!(
            ceiling_at < solicit_at,
            "the ceiling statement must sit ABOVE the first set-slot solicit \
             in the `{workflow}` compose (law 3: stated before it can fail)"
        );
    }
}

// ---------------------------------------------------------------------------
// A5 — the retire mechanics, stated where the approve gate is explained.
// ---------------------------------------------------------------------------

/// A foreign Nygard-style ADR the migrate verb stages (the corpus's repoE).
const FOREIGN_ADR: &str = "\
# 3. Use Postgres

Date: 2020-04-01

## Status

Accepted

## Context

We need a relational store.

## Decision

We will use Postgres.

## Consequences

Operational overhead accepted.
";

#[test]
fn migrate_compose_states_the_retire_mechanics() {
    let (repo, home) = setup("retire");
    fs::create_dir_all(repo.path().join("docs/adr")).expect("mkdir docs/adr");
    fs::write(
        repo.path().join("docs/adr/0003-use-postgres.md"),
        FOREIGN_ADR,
    )
    .expect("write foreign adr");
    git(repo.path(), &["add", "docs/adr/0003-use-postgres.md"]);
    git(repo.path(), &["commit", "-q", "-m", "foreign adr"]);

    let composed = jigc_ok(
        repo.path(),
        home.path(),
        &["migrate", "docs/adr/0003-use-postgres.md", "--as", "adr"],
    );
    let flat = collapsed(&composed);
    assert!(
        flat.contains("RETIRE the foreign original"),
        "the migrate compose keeps the retire contract; got:\n{composed}"
    );
    // The mechanics: deleted + the deletion staged, landing in the same commit
    // (what `retire` + `stage_migration` actually do — a `git rm` in effect).
    for fragment in ["deletion staged", "same commit", "`git rm`"] {
        assert!(
            flat.contains(fragment),
            "the migrate compose must state the retire mechanics fragment \
             `{fragment}`; got:\n{composed}"
        );
    }
}

// ---------------------------------------------------------------------------
// M45 inc-2 T6 — the ceiling statement takes its address context as a fence
// input (`design/surface-contract.md` → The stated-at fence, the M45 revision).
// ---------------------------------------------------------------------------

/// The retired global sentence — the one that claimed `####` is safe at every
/// slot address. It is a law-1 lie wherever the solicited slot is a multi-slot
/// or nested-bearing item, so no shipped surface may still carry it.
const RETIRED_GLOBAL_CLAIM: &str = "headings must sit at `####` depth or deeper";

/// `roadmap`'s `milestones` items are **multi-slot at nesting depth 1**, so the
/// CLI owns `####` there (the `#### <Leaf-Title>` sub-labels) — the projection
/// must reserve it and offer `#####`. This is the live defect: the planning
/// workflow's own primary write solicited exactly this address under a sentence
/// that blessed the corrupting depth.
#[test]
fn migrate_roadmap_preview_states_the_milestone_slot_reserved_set() {
    let (repo, home) = setup("ceiling-roadmap");
    let preview = composed_migration(repo.path(), home.path(), "roadmap");
    let flat = collapsed(&preview);
    for fragment in [
        "headings must sit at `#####` depth or deeper",
        "`##`/`###`/`####` are schema-reserved",
    ] {
        assert!(
            flat.contains(fragment),
            "the `roadmap` projection must render the milestone slots' reserved \
             set (`{fragment}`); got:\n{preview}"
        );
    }
    assert!(
        !flat.contains(RETIRED_GLOBAL_CLAIM),
        "the retired global claim must not survive in the `roadmap` compose; \
         got:\n{preview}"
    );
}

/// The omitting context — a **single-slot** item doctype (`decisions-log`'s
/// `entries` at depth 1) genuinely leaves `####` free, so its projection still
/// says `####`. The parameterization must reserve more only where the schema
/// does, never blanket-deepen every doctype.
#[test]
fn migrate_decisions_log_preview_keeps_the_shallower_reserved_set() {
    let (repo, home) = setup("ceiling-log");
    let preview = composed_migration(repo.path(), home.path(), "decisions-log");
    let flat = collapsed(&preview);
    assert!(
        flat.contains(
            "headings must sit at `####` depth or deeper — `##`/`###` are schema-reserved"
        ),
        "a single-slot item doctype still offers `####`; got:\n{preview}"
    );
}

/// The repo-wide census (`surface-contract.md` → the six-consumer table): no
/// shipped pack step may carry the unconditional claim. A step's prose is
/// composed before any address is chosen, so it states the schema-relative rule
/// and defers the depths to the projection.
#[test]
fn no_shipped_pack_step_claims_a_fixed_safe_depth() {
    let step_dirs = [
        cli::pack_path!(dev, "steps"),
        cli::pack_path!(methodology, "steps"),
    ];
    let mut checked = 0usize;
    for dir in step_dirs {
        for path in root_walk::files_in(Path::new(dir), root_walk::ext("yaml")) {
            let body = collapsed(&fs::read_to_string(&path).expect("read the step"));
            assert!(
                !body.contains(RETIRED_GLOBAL_CLAIM),
                "`{}` still claims `####` is safe unconditionally — the depths \
                 are per-address; state the schema-relative rule instead",
                path.display()
            );
            checked += 1;
        }
    }
    assert!(
        checked > 30,
        "the census must reach every shipped step; saw {checked}"
    );
}
