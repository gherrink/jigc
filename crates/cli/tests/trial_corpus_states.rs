//! M45 Increment 1 / T2 — the **trial-shaped fixture builder** and its isolation
//! fence (`implementation/pinning.md` §4).
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
