//! M45 Increment 1 / T2–T4 — the **trial-shaped fixture builder**, its isolation
//! fence, the three managed-corpus states, and the two repo-furniture states
//! (`implementation/pinning.md` §4).
//!
//! The builder is the shared substrate the compose-golden suite and the contract
//! property suites run over, so the substrate itself needs a fence: a fixture that
//! silently reads the *wrong pack* would regenerate a whole golden set of wrong
//! bytes, and nothing downstream would notice.
//!
//! Three arms, all over the real binary (`CARGO_BIN_EXE_jigc`):
//!
//!   (1) **Every named state builds** — the sweep entry point suites use, iterating
//!       [`State::ALL`] so a state added to the builder auto-joins here.
//!
//!   (2) **`fresh` is `jigc setup` only, and really ran it** — the `.jigc/`
//!       workbench and setup's own `.git/hooks/pre-commit` are on disk, built by
//!       driving the binary (the narrowed build rule: managed state through the
//!       binary, unmanaged furniture written directly).
//!
//!   (3) **A leaked `JIGC_PACK_DIR` never reaches a built state** — the parent test
//!       process is poisoned with a bogus pack directory and the built corpus still
//!       composes the embedded `[dev ▸ methodology]` pair. The provenance is read
//!       from a **real invocation's output** (`jigc start`'s `Pack:` line), never
//!       from the env the test itself just set.
//!
//! Then one arm per managed-corpus state, each asserting the state's **defining
//! property off real artifacts** — git's index and log, and the task working area
//! on disk — never off the builder's own bookkeeping:
//!
//!   (4) **`committed-singletons`** — vision/roadmap/decisions-log are tracked at
//!       their resolved `placement` homes and no task is left live (created *and*
//!       finalized, not merely staged).
//!
//!   (5) **`migrated`** — the foreign source was committed *before* the migration
//!       and the landing commit carries both its deletion and the managed doc's
//!       addition, so the retirement is a real destructive retire, not a tidy-up of
//!       an untracked file.
//!
//!   (6) **`refs-post-hoc`** — the `edited-from-base` provenance no finalized state
//!       can carry: a staged copy under `.jigc/tasks/<id>/docs/` holding the edge
//!       the committed bytes do not.
//!
//! Then T4's two **repo-furniture** states, both written directly (the narrowed
//! build rule: jigc has no verb that authors a foreign hook or a vendor dir):
//!
//!   (7) **`chatty-hooks`** — the hook speaks on a **successful** commit driven
//!       through a real jigc write path, and `jigc setup`'s own hook is **gone**, not
//!       wrapped: the assertion reads the real [`cli::setup::PRECOMMIT_SENTINEL`], so
//!       "replace, not append" is mechanically true rather than a comment.
//!
//!   (8) **`vendored`** — the gitignored runtime tree is on disk yet invisible to
//!       `git ls-files --cached --others --exclude-standard` (the ingest funnel's
//!       candidate walk) while the tracked code file is listed — a discrimination,
//!       not an empty walk.

mod support;

use support::trial_corpus::{State, TrialCorpus};

/// (1) Every named state in the builder constructs a real git repo.
#[test]
fn every_named_state_builds() {
    for state in State::ALL {
        let corpus = TrialCorpus::build(*state);
        assert!(
            corpus.repo().join(".git").is_dir(),
            "state `{}` must build a real git repo",
            state.name(),
        );
        assert_eq!(corpus.state(), *state);
    }
}

/// (2) `fresh` is `jigc setup` only — and the workbench plus setup's own
/// pre-commit hook prove the binary, not the test, put the state there.
#[test]
fn fresh_carries_the_workbench_and_setups_own_pre_commit_hook() {
    let corpus = TrialCorpus::build(State::Fresh);
    let repo = corpus.repo();

    assert!(
        repo.join(".jigc").is_dir(),
        "`fresh` must carry the `.jigc/` workbench setup installs",
    );
    let hook = repo.join(".git/hooks/pre-commit");
    assert!(
        hook.is_file(),
        "`fresh` must carry setup's own pre-commit hook",
    );
    let body = support::trial_corpus::read(&repo, ".git/hooks/pre-commit");
    assert!(
        body.contains("jigc"),
        "the installed pre-commit hook must be jigc's own:\n{body}",
    );
}

/// (3) The isolation fence: a poisoned parent `JIGC_PACK_DIR` does not reach the
/// built state, proven by the pack provenance a real invocation prints.
#[test]
fn a_leaked_pack_dir_never_reaches_a_built_state() {
    let mut bogus = std::env::temp_dir();
    bogus.push(format!(
        "jigc-bogus-pack-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after the epoch")
            .as_nanos(),
    ));
    std::fs::create_dir_all(&bogus).expect("create the bogus pack dir");

    // SAFETY: the Rust harness runs `#[test]` fns in parallel and the environment is
    // process-global, so this is the **sole** in-process writer of `JIGC_PACK_DIR` in
    // this binary, written once and never cleared. Poisoning the parent is the point:
    // the fence is that `TrialCorpus` removes the variable from every child it spawns,
    // which is unprovable unless the parent actually carries it.
    unsafe { std::env::set_var("JIGC_PACK_DIR", &bogus) };

    let corpus = TrialCorpus::build(State::Fresh);
    let stdout = corpus.jigc_ok(&["start"]);
    let pack_line = stdout
        .lines()
        .find(|line| line.starts_with("Pack:"))
        .unwrap_or_else(|| panic!("orientation must print a `Pack:` provenance line:\n{stdout}"));
    assert!(
        pack_line.contains("dev/") && pack_line.contains("methodology/"),
        "the built state must compose the embedded [dev ▸ methodology] pair, \
         not the poisoned JIGC_PACK_DIR: {pack_line}",
    );

    let _ = std::fs::remove_dir_all(&bogus);
}

/// (4) `committed-singletons` — the three methodology singletons are **created and
/// finalized**, so git tracks each at its resolved home and no task is left live.
/// The homes are the schemas' own: `VISION.md` and `docs/roadmap.md` /
/// `docs/decisions-log.md` are `placement` files, so they bypass `docs-root`.
#[test]
fn committed_singletons_tracks_the_three_singletons_at_their_homes() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let tracked = corpus.git(&["ls-files"]);
    for home in ["VISION.md", "docs/roadmap.md", "docs/decisions-log.md"] {
        assert!(
            tracked.lines().any(|line| line == home),
            "`committed-singletons` must track `{home}`; git ls-files:\n{tracked}",
        );
        let body = support::trial_corpus::read(&corpus.repo(), home);
        assert!(
            !body.trim().is_empty(),
            "the committed `{home}` must carry content",
        );
    }
    assert_eq!(
        corpus.live_task(),
        None,
        "every `committed-singletons` task is finalized — none stays live",
    );
    let listed = corpus.jigc_ok(&["task", "list"]);
    assert!(
        listed.contains("no active tasks"),
        "`committed-singletons` must leave no active task; got:\n{listed}",
    );
}

/// (5) `migrated` — the managed doc landed through `jigc migrate … --approve` and
/// the **foreign source is retired in the same commit**. Both halves are read off
/// git: the source was tracked before (so its deletion is a real retirement), the
/// finalize commit carries the add *and* the delete, and the path is gone from the
/// index and the working tree.
#[test]
fn migrated_retires_the_committed_foreign_source_in_the_landing_commit() {
    let corpus = TrialCorpus::build(State::Migrated);
    let foreign = support::trial_corpus::FOREIGN_VISION_PATH;

    let history = corpus.git(&["log", "--oneline", "--all", "--", foreign]);
    assert!(
        !history.trim().is_empty(),
        "the foreign source must have been committed before the migration, \
         so its retirement is a real deletion",
    );

    let landing = corpus.git(&["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        landing.lines().any(|line| line == format!("D\t{foreign}")),
        "the landing commit must retire the foreign source; got:\n{landing}",
    );
    assert!(
        landing.lines().any(|line| line == "A\tVISION.md"),
        "the landing commit must add the managed doc; got:\n{landing}",
    );

    let tracked = corpus.git(&["ls-files"]);
    assert!(
        !tracked.lines().any(|line| line == foreign),
        "the retired foreign source must be gone from the index; git ls-files:\n{tracked}",
    );
    assert!(
        !corpus.repo().join(foreign).exists(),
        "the retired foreign source must be gone from the working tree",
    );
}

/// (6) `refs-post-hoc` — an edge set by `doc set-field` on a **committed** doc
/// inside a **live** task, so the corpus carries the `edited-from-base` provenance
/// a finalized state cannot: a staged copy under `.jigc/tasks/<id>/docs/` whose
/// bytes diverge from the committed ones, carrying the edge the committed doc
/// does not.
#[test]
fn refs_post_hoc_carries_an_edited_from_base_staged_copy() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let repo = corpus.repo();
    let task = corpus
        .live_task()
        .expect("`refs-post-hoc` leaves its edge-setting task live");

    // The edge's target is committed — the edge points at real, tracked evidence.
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked
            .lines()
            .any(|line| line == "docs/research/context-loss.md"),
        "the grounding research must be committed; git ls-files:\n{tracked}",
    );

    let provenance =
        support::trial_corpus::read(&repo, &format!(".jigc/tasks/{task}/docs/provenance.json"));
    assert!(
        provenance.contains("\"vision:vision\": \"edited-from-base\""),
        "the committed vision must be copied in for editing, not created; got:\n{provenance}",
    );

    let staged =
        support::trial_corpus::read(&repo, &format!(".jigc/tasks/{task}/docs/vision:vision.md"));
    let committed = support::trial_corpus::read(&repo, "VISION.md");
    assert_ne!(
        staged, committed,
        "the staged copy must diverge from the committed bytes",
    );
    assert!(
        staged.contains("grounded-in: [research:context-loss]"),
        "the staged copy must carry the edge `set-field` wrote; got:\n{staged}",
    );
    assert!(
        !committed.contains("grounded-in"),
        "the committed doc must NOT carry the edge — it is live in the task only; got:\n{committed}",
    );
}

/// (7) `chatty-hooks` — the foreign hook speaks on a **successful** commit landed
/// through a real jigc write path (`start` → `doc create` → `task finalize`), and it
/// **replaced** setup's own hook rather than wrapping it.
///
/// The replacement half is asserted against the binary's own
/// [`cli::setup::PRECOMMIT_SENTINEL`] — the marker setup's installer keys on — so a
/// future wrap-instead-of-replace regression reddens here instead of quietly
/// restoring jigc's backstop underneath every suite that runs this state.
#[test]
fn chatty_hooks_speaks_on_a_successful_commit_and_replaced_setups_hook() {
    let corpus = TrialCorpus::build(State::ChattyHooks);
    let hook = support::trial_corpus::read(&corpus.repo(), ".git/hooks/pre-commit");
    assert!(
        !hook.contains(cli::setup::PRECOMMIT_SENTINEL),
        "`chatty-hooks` REPLACES setup's hook — its sentinel must be absent:\n{hook}",
    );
    assert!(
        hook.contains(support::trial_corpus::CHATTY_HOOK_MARKER),
        "the installed hook must be the chatty one:\n{hook}",
    );

    // A real jigc write path, landing a real commit — the hook fires on success.
    let task = corpus.start_workflow("planning", "plan the first wave");
    corpus.jigc_ok(&[
        "doc", "create", "roadmap", "--title", "Roadmap", "--task", &task,
    ]);
    let landed = corpus.finalize(&task, "planning", "mint the roadmap", false);

    assert!(
        landed.contains(support::trial_corpus::CHATTY_HOOK_MARKER),
        "the successful commit must relay the chatty hook's stdout; got:\n{landed}",
    );
    assert!(
        landed.contains("--- hook output ---"),
        "the relay must arrive in its delimited section; got:\n{landed}",
    );
    // The commit really landed — a hook that spoke on a *rejected* commit would
    // prove nothing about the success path.
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked.lines().any(|line| line == "docs/roadmap.md"),
        "the finalize must have landed its commit; git ls-files:\n{tracked}",
    );
}

/// (8) `vendored` — the gitignored runtime tree is **present on disk** and **absent
/// from** `git ls-files --cached --others --exclude-standard`, the walk the ingest
/// funnel takes its candidates from, while the tracked code file *is* listed.
///
/// Asserting the file exists is what keeps the invisibility claim from being
/// vacuously true over a directory that was never written.
#[test]
fn vendored_hides_its_runtime_tree_from_the_ingest_walk() {
    let corpus = TrialCorpus::build(State::Vendored);
    let runtime = support::trial_corpus::VENDORED_RUNTIME_FILE;
    let code = support::trial_corpus::VENDORED_CODE_FILE;

    assert!(
        corpus.repo().join(runtime).is_file(),
        "the vendored runtime file must be on disk — otherwise its invisibility is vacuous",
    );

    let walked = corpus.git(&["ls-files", "--cached", "--others", "--exclude-standard"]);
    assert!(
        !walked.lines().any(|line| line.starts_with("node_modules/")),
        "the gitignored runtime tree must be invisible to the ingest walk; got:\n{walked}",
    );
    assert!(
        walked.lines().any(|line| line == code),
        "the tracked code file must be listed by the same walk; got:\n{walked}",
    );
    let tracked = corpus.git(&["ls-files"]);
    assert!(
        tracked.lines().any(|line| line == code),
        "the code file must be committed, not merely untracked-and-visible; git ls-files:\n{tracked}",
    );
}
