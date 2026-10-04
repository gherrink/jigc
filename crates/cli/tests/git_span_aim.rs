//! **Every operator-facing `git` span names the checkout it runs in** — the M53 cwd census's
//! route class, fenced (`completions/artifacts/M53/cwd-census.md` → Census 1;
//! `design/surface-contract.md` → The printed-path fence, the *pasteable shell bytes*
//! disposition).
//!
//! Law 1 renders every printed path repo-relative, and that is right for the three surfaces a
//! reader reads and a driver keys on — the message, the `at:` locus and the `(code, target)`
//! key. It is wrong for the one span that is not read but **run**: git resolves a pathspec
//! against the *caller's* cwd, and a route is read from wherever the agent happens to be
//! standing. Driven on `1.0.0-rc.18` from `$REPO/docs/deep`:
//!
//! ```text
//! git restore --staged -- docs/deep/carried.txt     error: pathspec … did not match   rc=1
//! git add -- docs/deep/untracked.md                 fatal: pathspec … did not match   rc=128
//! git checkout -- docs/decisions-log.md                                               rc=1
//! git -C .jigc/worktrees/<id> bisect reset          fatal: cannot change to …         rc=128
//! ```
//!
//! and from a fan-out worktree the pathspec forms reach **the wrong index**, which no relative
//! spelling can fix, because `:/` means *that* worktree's top. `-C` takes a directory rather
//! than a pathspec, so the last one has no relative spelling at all.
//!
//! So `engine::finding::git_at` is the one home of the runnable spelling — `git -C <absolute
//! checkout> <subcommand> [flags] -- <repo-relative path>` — and `engine::finding::
//! unaimed_git_span` is the predicate that says whether a span obeys it. This suite fences
//! three things: **the predicate's own domain** (what counts as aimed, and what is declared
//! exempt with its reason), **the fence being live** on the `Route` constructors, and **the
//! producers**, by the source they route through, plus the key-leak invariant driven through
//! the real binary — the absolute is allowed in the route and nowhere else.

use crate::support::root_walk;
use crate::support::rust_source;
use engine::finding::{Route, git_at, unaimed_git_span};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------------
// Arm 1 — the predicate's domain
// ---------------------------------------------------------------------------------

/// Spans the predicate must **admit**, each with the shape that admits it.
///
/// Three, and only three: aimed, operand-less, or a declared non-path command
/// (`engine::finding::GIT_NON_PATH_COMMANDS`). Everything else resolves a path against the
/// reader's cwd.
#[test]
fn the_predicate_admits_aimed_operandless_and_declared_non_path_spans_only() {
    for ok in [
        // aimed — the render `git_at` produces, with and without a quoted home
        "unstage it (`git -C /repo restore --staged -- docs/a.md`)",
        "unstage it (`git -C '/a repo' restore --staged -- docs/a.md`)",
        "revert the move: `git -C /repo mv docs/a.md docs/b.md`",
        "then `git -C /repo worktree remove --force .jigc/worktrees/x`",
        "`git -C /repo show 0000000 -- CHANGELOG.md`",
        // aimed at a placeholder the reader fills — the hook body's own documented shape
        "`git -C <repo> mv <new> <old>`",
        // operand-less: the span names a command, not a file
        "`git add` your code edits before finalize",
        "run `git worktree list`, then look again",
        "commit or stash the work — `git stash -u` where git does not",
        "`git worktree list`, then `git worktree remove`",
        // declared non-path commands
        "run `git init` here first",
        "re-attach HEAD with `git switch main`, then re-run",
        "switch back with `git checkout ab12cd3`",
        "tell git who you are — `git config user.email \"you@example.com\"`",
        // not a git span at all
        "`jigc doc show adr:pick-a-db`",
        "`--force` consents past it",
        // --- M53 post-review-fix review, LOW 8 ---
        // A COMPOSITE whose `git` half is aimed: the shipped `repair_prefix` shape. Before
        // the fix the fence skipped this span entirely (its head is `mkdir`), so it was
        // admitted for the wrong reason — unchecked rather than checked and correct.
        "`mkdir -p /repo/docs/new && git -C /repo mv docs/old/a.md docs/new/a.md`",
        // The three commands declared non-path at the same review. None ships today; each
        // would have been called a defect, and a fence that panics over a correct span is a
        // fence producers route around.
        "`git commit -m \"docs/x.md\"`",
        "`git log a..b`",
        "`git show 0000000`",
    ] {
        assert_eq!(
            unaimed_git_span(ok),
            None,
            "the fence must admit: {ok}\n(aimed, operand-less, or a declared non-path command)"
        );
    }

    for bad in [
        "unstage it (`git restore --staged -- docs/a.md`)",
        "stage it with `git add -- docs/deep/untracked.md`",
        "restore it: `git checkout -- CHANGELOG.md`",
        "`git restore --source=HEAD --staged --worktree -- CHANGELOG.md`",
        "revert the move: `git mv docs/a.md docs/b.md`",
        "`git worktree remove --force .jigc/worktrees/x`",
        "`git show 0000000 -- CHANGELOG.md`",
        // `-C` present but relative — the census row that opened this class: `-C` takes a
        // directory, and a relative one exits 128 from every cwd but the repository root.
        "abandon it with `git -C .jigc/worktrees/x bisect reset`",
        // a declared non-path command turned into a path one by git's own `--`
        "restore it: `git checkout -- <path>`",
        // LOW 8: the composite's `git` half UNAIMED — the cell the shipped fence could not
        // reach, because it only ever looked at a span's first token.
        "`mkdir -p /repo/docs/new && git mv docs/old/a.md docs/new/a.md`",
        // and the same three verbs turned into path commands by `--`
        "`git log -- docs/x.md`",
        "`git show 0000000 -- CHANGELOG.md`",
    ] {
        assert!(
            unaimed_git_span(bad).is_some(),
            "the fence must refuse: {bad}\n(a path operand resolved against the reader's cwd)"
        );
    }
}

// ---------------------------------------------------------------------------------
// Arm 2 — the fence is live
// ---------------------------------------------------------------------------------

/// **The fence itself**, on the constructor every route of every kind passes through.
///
/// This is the red the class's fix was proven against: leaving one producer unconverted is
/// exactly this construction, and it panics in debug posture (the seam-assert class — it
/// rides the suite, never a release-build panic, like its two siblings on the same
/// constructor).
#[test]
#[should_panic(expected = "must name the checkout it runs in")]
fn an_unaimed_git_span_with_a_path_operand_fires_the_fence() {
    let _ = Route::human("unstage it (`git restore --staged -- docs/deep/carried.txt`)");
}

/// The same span through the one home, on all three constructors: it constructs, and the
/// bytes are the runnable spelling.
#[test]
fn the_same_span_constructs_once_it_is_rendered_through_git_at() {
    let home = Path::new("/repo");
    let text = format!(
        "unstage it (`{}`)",
        git_at(home, "restore --staged -- docs/deep/carried.txt")
    );
    assert_eq!(
        Route::human(text.clone()).as_str(),
        "unstage it (`git -C /repo restore --staged -- docs/deep/carried.txt`)",
    );
    let _ = Route::informational(text.clone());
    let _ = Route::mechanical(["jigc", "task", "list"], format!(" — {text}"));
}

// ---------------------------------------------------------------------------------
// Arm 3 — the producers, by the source they route through
// ---------------------------------------------------------------------------------

/// What a production site does with the `git` command it emits.
#[derive(Clone, Copy, PartialEq)]
enum Aim {
    /// It renders through the one home ([`AIM_HOME`]) — `git -C <absolute checkout> …`.
    Aimed,
    /// It emits a `git` span on purpose without aiming it, for the stated reason.
    DeclaredOut,
}

/// The one shared home every `Aimed` site must reach.
const AIM_HOME: &str = "git_at";

/// `(file, fn, aim, reason)` — **the class, enumerated**, with each verdict checked against
/// the source below rather than believed.
///
/// **Derived, not taken from the census.** The class was re-walked at HEAD:
/// `command grep -rn '`git ' crates/{cli,engine}/src` with comment lines dropped yields **136**
/// production lines carrying a backticked `git` span. Of those, the operator-facing commands
/// with a path operand are **13 producing functions** — the rows below — emitting **18 spans**
/// from **14** `git_at` call sites (one call site serves `finalize`'s three boundaries and
/// another `orphan`'s three removal arms; `remove_worktrees` and `untracked_workbench_finding`
/// emit two spans each). The census predicted "~15 sites"; the walk found one it does not list
/// at all — `setup::forced_install_path_finding`'s `git show HEAD -- <path>` — and refuted one
/// it does: `milestone::remove_worktrees` already emitted an **absolute** operand
/// (`canonical_home.join(…)`), so what was cwd-fragile there was its `git worktree prune`
/// neighbour, not the `remove`.
///
/// **[Corrected 2026-10-04 (the rc.24 fix pass, L-22).** That `git worktree prune` span is
/// gone — a repository-wide prune is no remedy jigc prints any more
/// (`worktree_registration_reach.rs`). The leaked-worktree warning's spans moved out of
/// `remove_worktrees` into `milestone::leaked_worktree_remedy`, which emits one
/// (`worktree remove --force <path>`) or, for a locked worktree, two (`worktree unlock
/// <path>` first), every one with a path operand and every one aimed. The producer count is
/// unchanged; the span count is 17 or 18 by arm.**]**
///
/// **[Extended 2026-10-04 (the rc.24 fix pass, the registration's own content).** Two more
/// producers, both in `milestone.rs` and both off the `Route` (they ride a refusal's
/// per-path line and a loss narration): `keep_commit_command` and `restore_recipe`. The
/// table is the count — `every_production_caller_of_the_one_home_is_a_row` holds it to the
/// source.**]** **[And a third the same day**, `unlanded_work_finding`: the milestone
/// boundary's pre-landing refusal prints two aimed spans of its own (`reset --soft`,
/// `stash`) beside the two above.**]** **[And a fourth**, `record_shape_block`: the record
/// doors' refusal over a record whose home is not a regular file prints the `HEAD`-sourced
/// restore its sibling `record_conflict_block` does (the store doors' half of `(R6, D-7)`).**]**
/// **[And a fifth**, `task::git_against_rev`: where the base-pin backstop cannot get git's
/// answer, its refusal quotes the comparison it asked, aimed, so the reader can run it.**]**
///
/// The remaining 120 lines are `bail!`/`with_context` diagnostics **quoting the invocation
/// that failed** — law 1's second declared-absolute reason, and outside a route by
/// construction (the span fence sits on `Route`, which they never build) — plus bare
/// operand-less mentions in help and pack prose (`git add` your code edits), which need no
/// aiming because they name no file.
const GIT_SPAN_SITES: &[(&str, &str, Aim, &str)] = &[
    // --- the engine's producers (handed the absolute home, never discovering one) ---
    (
        "crates/engine/src/finalize.rs",
        "held_clobber_text",
        Aim::Aimed,
        "`finalize.promote-clobber`'s held arm (the rc.24 fix pass, the completion audit's \
         CPL-5) — `git checkout HEAD -- <home>` / `git checkout -- <home>`, the restore of a \
         file git still holds at a doc's home and the worktree does not. Printed by both \
         committing doors, which run from any directory of the checkout, and aimed at the \
         checkout whose index and `HEAD` were asked",
    ),
    (
        "crates/engine/src/finalize.rs",
        "fixed_clobber_text",
        Aim::Aimed,
        "`finalize.promote-clobber`'s fixed-identity arm (the completion audit's CPL-3) — \
         `git add -- <home>`, the first of the two commands that bring the file at a \
         singleton's home under management once the mint is dropped: `jigc migrate` refuses \
         a source git has never recorded, so the stage is printed ahead of it, and both are \
         run from wherever the reader stands",
    ),
    (
        "crates/engine/src/finalize.rs",
        "staged_clobber_text",
        Aim::Aimed,
        "`finalize.promote-clobber`'s staged arm at the milestone boundary (the completion \
         audit's CPL-2) — `git stash push -- <home>`, aimed at the SUB-TASK WORKTREE that \
         staged the file, never at the main checkout the boundary runs in: the stash has to \
         take the path out of that worktree's index, and it keeps the bytes under the \
         repository's `refs/stash`, where the worktree's teardown does not reach",
    ),
    (
        "crates/engine/src/milestone.rs",
        "record_home_held_finding",
        Aim::Aimed,
        "the same restore at `jigc milestone create`, for a committed record deleted from \
         the worktree — `milestone.record-exists`'s held sibling. A record door runs from \
         any directory of the checkout, as its conflict-block siblings below do",
    ),
    (
        "crates/engine/src/finalize.rs",
        "carried_staged_finding",
        Aim::Aimed,
        "`finalize.carried-staged` at all three boundaries — the trial-ranked #1 v1 gate, and \
         the row that ranks first in the census: it fires on the ordinary finalize path and \
         the route is the only thing the agent is given",
    ),
    (
        "crates/engine/src/finalize.rs",
        "setup_dirty_install_finding",
        Aim::Aimed,
        "`setup.dirty-install-path` read on an unborn `HEAD` — `git rm --cached -- <path>`, \
         the unstage that makes *move the file out* true of a path the adopter had staged. \
         The born arm's `git stash -u` carries no operand; this one does, and the refusal is \
         printed by the first command an adopter runs, from wherever they ran it. And, on a \
         born `HEAD`, the two `git update-index --no-assume-unchanged|--no-skip-worktree -- \
         <path>` spans that let git report a change an index flag was hiding (the rc.24 fix \
         pass), each over the listed paths themselves",
    ),
    (
        "crates/engine/src/validate.rs",
        "owner_artifact_present",
        Aim::Aimed,
        "`owner-artifact.present`'s untracked cause — `git add`, which also gained the `--` \
         separator it had been missing (git's own option-parsing guard, the one \
         `trackable.rs` states in prose)",
    ),
    (
        "crates/engine/src/file_state.rs",
        "rename_strong_finding",
        Aim::Aimed,
        "`reconciliation.rename`'s revert arm — `git mv`, which takes paths and refuses a \
         pathspec outright (`fatal: bad source`), so `-C` is the only spelling that runs \
         from more than one directory",
    ),
    // --- the CLI's producers ---
    (
        "crates/cli/src/orphan.rs",
        "home_vacated_finding",
        Aim::Aimed,
        "`schema-conformance.home-vacated`'s three removal arms — `git show <sha> --`, \
         `git checkout --`, `git restore --source=HEAD --staged --worktree --`",
    ),
    (
        "crates/cli/src/task.rs",
        "amend_index_dirty_finding",
        Aim::Aimed,
        "`finalize.amend-index-dirty`'s unstage route (F-10) — `git restore --staged -- \
         <path>`, one per staged path the amend would otherwise fold into the commit it is \
         rewriting. Aimed because the operand is a repo-relative pathspec git resolves \
         against the repository, not against the caller's cwd",
    ),
    (
        "crates/cli/src/task.rs",
        "git_against_rev",
        Aim::Aimed,
        "the base-pin backstop's *git could not answer* refusal (the rc.24 fix pass's \
         completion audit) — it quotes the comparison jigc asked (`git ls-tree <pin> -- \
         <path>`, `git diff --quiet <pin> -- <path>`, `git ls-files -v -- <path>`) so the \
         reader can run it and see git's answer, and each carries a repo-relative path \
         operand, so each is aimed at the checkout it was asked in",
    ),
    (
        "crates/cli/src/migrate.rs",
        "adjudicate_source_tracked",
        Aim::Aimed,
        "`migrate.source-untracked` — the one route line carrying TWO spans with two \
         resolution bases: the `git` half is aimed and keeps the adjudicated repo-relative \
         pathspec, while the `jigc migrate` half echoes the token the operator typed, \
         because that door resolves against the caller's cwd since `4f61c80a`",
    ),
    (
        "crates/cli/src/repo.rs",
        "aim_at",
        Aim::Aimed,
        "the `-C` redirection itself, shared by `BreachSite::aim` and the leftover \
         classifier's `held_here` — the row that opened the census, and the one shape no \
         pathspec spelling can reach",
    ),
    (
        "crates/cli/src/milestone.rs",
        "record_conflict_block",
        Aim::Aimed,
        "the milestone-record conflict block's `git checkout --` restore",
    ),
    (
        "crates/cli/src/milestone.rs",
        "record_shape_block",
        Aim::Aimed,
        "the same conflict block's shape sibling — a record whose home has become a link — \
         and the same `git checkout HEAD --` restore, which puts the regular file back where \
         `HEAD` still holds the record as one (git replaces the entry, it does not write \
         through it). Aimed for the reason its sibling is: a record door runs from any \
         directory of the checkout",
    ),
    (
        "crates/cli/src/milestone.rs",
        "leaked_worktree_remedy",
        Aim::Aimed,
        "the leaked-worktree warning's remedy, printed by `remove_worktrees`. Its \
         `git worktree remove --force` operand was ALREADY absolute (the census predicted \
         otherwise); the `git worktree prune` that stood beside it was the fragile one, and \
         since the rc.24 fix pass it is not printed at all — the remedy names the one \
         registration, with `git worktree unlock <path>` ahead of it for a locked worktree",
    ),
    (
        "crates/cli/src/milestone.rs",
        "keep_commit_command",
        Aim::Aimed,
        "the command that keeps a commit only a fan-out registration's `HEAD` reaches — \
         `git branch kept/<sub-task-id> <sha>` (the rc.24 fix pass). Its operands are a ref \
         name and a sha, not paths, but the *repository* is the subject: a reader standing in \
         another checkout would branch the wrong one, so it is aimed — at the live worktree \
         where there is one, at the main checkout where the worktree's directory is gone. \
         Printed on the refusal's per-path line (via `anchored_words`) and again by \
         `PendingAnchor::narrate_taken` after a drop, where it still works",
    ),
    (
        "crates/cli/src/milestone.rs",
        "unlanded_work_finding",
        Aim::Aimed,
        "the two exits only `jigc milestone finalize`'s pre-landing refusal offers (the rc.24 \
         fix pass, `milestone.unlanded-work`), both aimed at the sub-task's live worktree: \
         `git reset --soft <base pin>`, which turns a commit made inside it into staged paths \
         the boundary lands, and `git stash`, which keeps a settled sub-task's staged paths \
         under `refs/stash`. Neither takes a path operand, but the *checkout* is the subject \
         — run from anywhere else they would move another HEAD or stash another index. Like \
         the keep command they ride the refusal's per-path line rather than the `Route`; \
         `worktree_registration_anchor.rs` runs each as printed from outside the repository",
    ),
    (
        "crates/cli/src/milestone.rs",
        "restore_recipe",
        Aim::Aimed,
        "the recipe that brings a missing fan-out checkout back from git's registration of \
         it — a COMPOSITE, `mkdir -p <abs> && printf 'gitdir: %s\\n' <abs admin> > <abs>/.git \
         && git -C <abs> restore .`. Only the last part is a `git` command and it is aimed; \
         the two before it have no `-C` and render absolute, shell-quoted operands instead, \
         so the whole line runs from any directory (`repair_prefix`'s rule, three parts \
         long). Not a `Route` — it rides the refusal's per-path line — so the fence cannot \
         see it; `worktree_registration_anchor.rs` runs it as printed from outside the \
         repository",
    ),
    (
        "crates/cli/src/ingest.rs",
        "unaddressable_identity_finding",
        Aim::Aimed,
        "`ingest.unaddressable-identity`'s `git mv` repair, at both the ingest and the \
         relocate door",
    ),
    (
        "crates/cli/src/setup.rs",
        "forced_install_path_finding",
        Aim::Aimed,
        "`setup.forced-install-path`'s `git show HEAD -- <path>` — IN NO CENSUS ROW, earned \
         by re-walking the class rather than taking the reported list",
    ),
    (
        "crates/cli/src/setup.rs",
        "untracked_workbench_finding",
        Aim::Aimed,
        "`uninstall.untracked-workbench-file`'s recovery pair — `git add -- <path>` and \
         `git checkout -- <path>`",
    ),
    (
        "crates/cli/src/setup.rs",
        "narrate_workbench_files",
        Aim::Aimed,
        "the teardown's tracked-file warning — `git checkout -- <path>`. Not a `Finding`, so \
         the `Route` fence cannot see it; it is in the class because it is a remedy an \
         operator pastes, and it reaches the same home",
    ),
    (
        "crates/cli/src/render.rs",
        "relocate_change_steps",
        Aim::Aimed,
        "the linked-worktree doc guard's one exit for a change that must share a commit with \
         a managed doc (the rc.24 fix pass) — `git stash` aimed at the linked worktree, then \
         `git merge <branch>` and `git stash pop --index` aimed at the main checkout. Three \
         spans over TWO checkouts in one route, which is exactly the shape an unaimed span \
         cannot express: the reader is standing in one of them and each command must run \
         against the one it names",
    ),
    (
        "crates/cli/src/task.rs",
        "linked_worktree_doc_finding",
        Aim::Aimed,
        "the linked-worktree doc guard's backstop, where the main checkout's index holds \
         staged entries the task's pre-task snapshot does not cover (the rc.24 fix pass, the \
         completion audit's F4) — `git diff --cached --name-only` aimed at the main checkout. \
         The reader is standing in the linked worktree, and the listing that says what a \
         finalize there would have committed has to be of the other checkout's index",
    ),
    // --- the one span in the class that is deliberately NOT a route ---
    (
        "crates/cli/src/ingest.rs",
        "repair_prefix",
        Aim::DeclaredOut,
        "`mkdir -p <dir> && ` is the relocate door's prefix to the `git mv` beside it, and \
         `mkdir` has no `-C`. It is not exempt from the RULE — it renders the ABSOLUTE \
         directory, so the two halves of one pasted line run in the same place — only from \
         the `git_at` home, which composes a `git` command",
    ),
];

/// The enclosing function of every production `git_at(` call in `file`.
fn aim_home_callers(file: &str) -> Vec<String> {
    home_callers(file, &[AIM_HOME])
}

/// The enclosing function of every production call of `home` in `file` — shared by this
/// suite's two homes (`git_at` for the `git` class, `migrate_operand` for the `jigc migrate`
/// one), because both classes ask the identical source question.
fn home_callers(file: &str, homes: &[&str]) -> Vec<String> {
    let path = workspace_root().join(file);
    let body = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {file}"));
    let code = rust_source::code_only(&body);
    let regions = rust_source::cfg_test_regions(&code);
    let mut owners: Vec<String> = Vec::new();
    for home in homes {
        owners.extend(
            code.match_indices(home)
                .filter(|(at, _)| !rust_source::is_test_domain(&path, &regions, *at))
                .filter_map(|(at, _)| rust_source::enclosing_fn(&code, at).map(str::to_string)),
        );
    }
    owners
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root is two levels above crates/cli")
        .to_path_buf()
}

/// **Every `Aimed` row reaches the one home, and every reason is written.**
///
/// The verdict is checked against the source, not believed: a row claiming `Aimed` whose
/// function holds no `git_at` call is a row that went stale, which is the whole failure mode a
/// disposition table has (`repo_relative_paths.rs`'s `PATH_TEXT_SITES` idiom, same shape).
#[test]
fn every_aimed_site_routes_through_the_one_home_and_every_row_states_why() {
    let mut offenders: Vec<String> = Vec::new();
    for (file, function, aim, reason) in GIT_SPAN_SITES {
        assert!(
            !reason.trim().is_empty(),
            "{file}::{function} carries no reason"
        );
        let callers = aim_home_callers(file);
        let reaches = callers.iter().any(|owner| owner == function);
        match aim {
            Aim::Aimed if !reaches => offenders.push(format!(
                "  {file}::{function}: disposed `Aimed`, but no production `{AIM_HOME}(` call \
                 sits in it — the `{AIM_HOME}(` callers in that file are: {callers:?}"
            )),
            Aim::DeclaredOut if reaches => offenders.push(format!(
                "  {file}::{function}: disposed `DeclaredOut`, but it calls `{AIM_HOME}(` — \
                 one of the two is wrong"
            )),
            _ => {}
        }
    }
    assert!(
        offenders.is_empty(),
        "a disposition nothing checks is a sentence, not a disposition:\n{}",
        offenders.join("\n"),
    );
}

/// **The table is the class, and the class is re-derivable.** Every production `git_at(`
/// caller in the codebase is a row here — so a producer converted later without joining the
/// table reddens, and the table cannot quietly become a subset of the code.
#[test]
fn every_production_caller_of_the_one_home_is_a_row() {
    let files: Vec<&str> = {
        let mut files: Vec<&str> = GIT_SPAN_SITES.iter().map(|(file, ..)| *file).collect();
        files.sort_unstable();
        files.dedup();
        files
    };
    // The search domain is the two crates' sources, not only the files the table names —
    // otherwise a new file would be invisible to its own fence.
    let mut unlisted: Vec<String> = Vec::new();
    for dir in ["crates/cli/src", "crates/engine/src"] {
        for path in root_walk::files_in(&workspace_root().join(dir), root_walk::ext("rs")) {
            let rel = format!("{dir}/{}", path.file_name().unwrap().to_string_lossy());
            for owner in aim_home_callers(&rel) {
                // The home's own definition and its fence live in `engine/src/finding.rs`.
                if rel == "crates/engine/src/finding.rs" {
                    continue;
                }
                let listed = GIT_SPAN_SITES
                    .iter()
                    .any(|(file, function, ..)| *file == rel && *function == owner);
                if !listed {
                    unlisted.push(format!("  {rel}::{owner}"));
                }
            }
        }
    }
    unlisted.sort();
    unlisted.dedup();
    assert!(
        unlisted.is_empty(),
        "these production sites render the runnable spelling and are in no row of \
         `GIT_SPAN_SITES` — add them (the files the table currently names: {files:?}):\n{}",
        unlisted.join("\n"),
    );
}

// ---------------------------------------------------------------------------------
// Arm 3b — the same class, one verb over: `jigc migrate <PATH>`
// ---------------------------------------------------------------------------------

/// **`jigc migrate` is the second verb in this class, and it was swept at one producer of
/// six** (M53 post-review-fix review, HIGH 2).
///
/// `4f61c80a` moved `jigc migrate <PATH>`'s base from the repository root to the caller's
/// cwd. That is right for a path the caller types and wrong for a path **jigc** prints into
/// the verb: a repo-relative store key, a promote destination, a recorded migration source.
/// Driven from `$REPO/docs/deep` on a `fresh` corpus with a foreign `CHANGELOG.md`, the store
/// sweep's own advisory emitted `` `jigc migrate CHANGELOG.md --as changelog` `` and running
/// it verbatim answered *"could not read the foreign `changelog` source at `CHANGELOG.md`"* —
/// the row the census had called *"already correct — the model for the fix"*.
///
/// The verb takes no `-C`, so there is no redirection to add: the operand itself is spelled
/// absolute, through `engine::finding::migrate_operand`.
#[test]
fn the_migrate_predicate_admits_an_absolute_or_a_reader_supplied_operand_only() {
    for ok in [
        // absolute — `migrate_at`'s render, bare and quoted
        "adopt it with `jigc migrate /repo/CHANGELOG.md --as changelog`",
        "adopt it with `jigc migrate '/a repo/CHANGELOG.md' --as changelog`",
        // reader-supplied: the reader fills it with a path rooted where they stand, which is
        // exactly where the verb roots it
        "bring it under management: `jigc migrate <path> --as <doctype>`",
        // not a migrate span at all
        "`jigc migrate-corpus`",
        "`jigc ingest` routes it",
        // no operand
        "`jigc migrate --help`",
    ] {
        assert_eq!(
            engine::finding::unbased_migrate_span(ok, None),
            None,
            "the fence must admit: {ok}"
        );
    }

    for bad in [
        "adopt it with `jigc migrate CHANGELOG.md --as changelog`",
        "adopt it with `jigc migrate docs/deep/x.md --as adr`",
        "re-mint with `jigc migrate 'decisions/my notes.md' --as adr`",
        // **A composite span is read part by part** (the confirmation pass, LOW 4). This
        // read only the span's FIRST token, so a `jigc migrate` half behind any sequencing
        // operator was skipped whole — the same blindness its two siblings in this range
        // were swept for one commit later.
        "from the repo: `cd /repo && jigc migrate rel.md --as adr`",
        "`mkdir -p /repo/docs && jigc migrate docs/x.md --as adr`",
    ] {
        assert!(
            engine::finding::unbased_migrate_span(bad, None).is_some(),
            "the fence must refuse: {bad}\n(the verb roots that operand at the READER's cwd)"
        );
    }
}

/// **The carve-out exempts one OPERAND, not the whole text** (the confirmation pass, LOW 3).
///
/// The shipped exemption asked `span.contains(echo)` of the predicate's **first** offender
/// and, on a hit, skipped the panic for every other span in the text. So a producer emitting
/// the operator's own token *and* a second span carrying a path jigc computed was never
/// checked on the second span — and `contains` is a substring test, so a computed
/// `docs/note.md` swallowed an echo of `note.md` too.
#[test]
fn the_caller_echo_exempts_its_own_operand_and_nothing_else() {
    // The echoing span alone — exempt.
    let echo_only = "stage it, then re-run `jigc migrate note.md --as adr`";
    assert_eq!(
        engine::finding::unbased_migrate_span(echo_only, Some("note.md")),
        None,
        "the operator's own token is the declared carve-out"
    );
    // The echoing span PLUS a computed relative one — the second is still an offender.
    let echo_plus_computed = "stage it, then re-run `jigc migrate note.md --as adr`; the \
                              sibling is `jigc migrate docs/other.md --as adr`";
    assert_eq!(
        engine::finding::unbased_migrate_span(echo_plus_computed, Some("note.md")).as_deref(),
        Some("jigc migrate docs/other.md --as adr"),
        "the carve-out must not exempt a span it does not name"
    );
    // Substring is not identity: a computed `docs/note.md` merely CONTAINS the echo.
    let superstring = "re-run `jigc migrate docs/note.md --as adr`";
    assert!(
        engine::finding::unbased_migrate_span(superstring, Some("note.md")).is_some(),
        "the carve-out is operand identity, not a substring of the span"
    );
}

/// The per-operand carve-out is live on the constructor, not only in the predicate.
#[test]
#[should_panic(expected = "must name a path that resolves from anywhere")]
fn a_second_computed_span_beside_the_caller_echo_fires_the_fence() {
    let _ = Route::human_echoing_caller_token(
        "note.md",
        "stage it, then re-run `jigc migrate note.md --as adr`; the sibling is \
         `jigc migrate docs/other.md --as adr`",
    );
}

/// The fence is live on the constructor, exactly like its `git` sibling.
#[test]
#[should_panic(expected = "must name a path that resolves from anywhere")]
fn an_unbased_migrate_span_fires_the_fence() {
    let _ = Route::human("adopt it with `jigc migrate CHANGELOG.md --as changelog`");
}

/// **The one declared carve-out, and it is a constructor rather than a hole.** A span
/// re-printing the token the *operator* typed is bytes-indistinguishable from a store key
/// jigc computed; only the producer knows which, so the producer says so
/// (`Route::human_echoing_caller_token`) and the exemption is one call site wide.
#[test]
fn the_caller_echo_carve_out_is_declared_at_the_constructor() {
    let text = "stage it, then re-run `jigc migrate note.md --as adr`";
    assert!(engine::finding::unbased_migrate_span(text, None).is_some());
    let route = Route::human_echoing_caller_token("note.md", text);
    assert_eq!(route.as_str(), text);
}

/// **The head-only read had a fourth member, and the finding named one** (the confirmation
/// pass, LOW 4 — the class, not the reported site).
///
/// `unaimed_git_span` and `unsafe_command_token` were widened to `split_shell_sequence` in
/// this range; the review swept `unbased_migrate_span` with them. The subject-quoting fence
/// (`fence_addressed_token_is_quoted`, the one that asserts a finding's own unsafe address is
/// rendered through `shell_token` wherever a command span names it) read the span's first
/// token too — so the one span shape that carries two paths, `cd <abs> && git …`, was skipped
/// whole. All four now ask the same question of a composite span.
#[test]
#[should_panic(expected = "names this finding's own subject")]
fn a_composite_span_is_still_asked_the_subject_quoting_question() {
    use engine::finding::{Finding, Location, Severity};
    let _ = Finding::graded(
        Severity::Advisory,
        "probe.subject-quoting",
        "the subject is named unquoted inside a composite span".to_owned(),
        Some(Location::addressed("my notes.md", 1, 1)),
        Some(Route::human(
            "run `cd /repo && git -C /repo add -- my notes.md`",
        )),
    );
}

/// What a production site does with the `jigc migrate` path operand it emits — the sibling of
/// [`GIT_SPAN_SITES`], over the sibling home. Two spellings of one home: `migrate_at` renders
/// the whole command, `migrate_operand` the `<PATH>` alone for the producer whose route is an
/// argv list rather than a command line.
const MIGRATE_OPERAND_HOME: &[&str] = &["migrate_at", "migrate_operand"];

/// `(file, fn, based, reason)`.
///
/// **Derived, not taken from the review.** The review reported *"6 production sites, 1
/// correct"* from `command grep -rn 'jigc migrate [^-c]'`. That grep reads command lines, and
/// `crates/cli/src/ingest.rs::near_miss_route` spells the verb as **separate argv tokens** to
/// `Route::mechanical`, so no line of it matches — it was invisible to the derivation that
/// found it a problem everywhere else. Re-walked at HEAD over both spellings, the class is
/// **five producers that print a path jigc computed** (the four the review named minus the
/// legend, plus `near_miss_route`) and **three that print a `<path>` the reader fills**.
const MIGRATE_SPAN_SITES: &[(&str, &str, Aim, &str)] = &[
    (
        "crates/engine/src/validate.rs",
        "adoption_route",
        Aim::Aimed,
        "`schema-conformance.unadopted-instance`'s adoption route — reached by the store \
         sweep AND by `jigc doc show`'s reroute of an unregistered instance, so one home \
         keeps the two doors telling one story. The row the review drove broken",
    ),
    (
        "crates/cli/src/orphan.rs",
        "unregistered_route",
        Aim::Aimed,
        "the unregistered tier of the two-tier orphan advisory. Its operand was interpolated \
         RAW — not even shell-quoted — so a store key with a space in it emitted two tokens \
         as well as the wrong base",
    ),
    (
        "crates/engine/src/finalize.rs",
        "clobber_finding",
        Aim::Aimed,
        "`finalize.promote-clobber`'s two task arms — the recorded migration source, and the \
         occupied promote destination a foreign file is invited to be adopted at. Its third, \
         the milestone boundary's sub-task arm, prints no `jigc migrate` span on purpose: \
         adopting the occupant before that boundary moves `HEAD` off the milestone's base",
    ),
    (
        "crates/engine/src/finalize.rs",
        "fixed_clobber_text",
        Aim::Aimed,
        "`finalize.promote-clobber`'s fixed-identity arm (the completion audit's CPL-3): the \
         adoption of the file at a singleton's home, offered for after the mint is dropped \
         — at the milestone boundary, after the milestone has landed. The same destination \
         path `clobber_finding`'s task arm prints, for a doc that arm's other exits cannot \
         move",
    ),
    (
        "crates/cli/src/ingest.rs",
        "near_miss_route",
        Aim::Aimed,
        "the near-miss adoption argv — THE PRODUCER IN NO REPORTED LIST, earned by walking \
         both spellings of the verb rather than the one a command-line grep can see",
    ),
    (
        "crates/cli/src/doc.rs",
        "distinct_identity_route",
        Aim::Aimed,
        "the distinct-identity route under a migration's recorded `--slug` (M55): the id is \
         the override, so the correction is a re-migrate of the task's recorded source under \
         a distinct slug — a path jigc recorded, run from wherever the agent stands",
    ),
    (
        "crates/cli/src/migrate.rs",
        "migrate_in_repo",
        Aim::Aimed,
        "the door's refusal from a checkout that commits code only (the rc.24 fix pass): the \
         route re-aims the migration at the MAIN checkout's copy of the adjudicated source, \
         a path jigc computed against a checkout the reader is not standing in — so it is \
         spelled absolute, and never the token the operator typed, which names this \
         worktree's file",
    ),
    (
        "crates/cli/src/migrate.rs",
        "adjudicate_source_tracked",
        Aim::DeclaredOut,
        "`migrate.source-untracked` re-prints the token the OPERATOR typed, so that a re-run \
         means re-running theirs. It is declared at the constructor \
         (`Route::human_echoing_caller_token`), not exempted by a predicate that cannot tell \
         that operand from a store key",
    ),
];

/// Every `Aimed` row reaches the one home; every `DeclaredOut` row states why it does not.
#[test]
fn every_based_migrate_site_routes_through_the_one_home_and_every_row_states_why() {
    let mut offenders: Vec<String> = Vec::new();
    for (file, function, aim, reason) in MIGRATE_SPAN_SITES {
        assert!(
            !reason.trim().is_empty(),
            "{file}::{function} carries no reason"
        );
        let callers = home_callers(file, MIGRATE_OPERAND_HOME);
        let reaches = callers.iter().any(|owner| owner == function);
        match aim {
            Aim::Aimed if !reaches => offenders.push(format!(
                "  {file}::{function}: disposed `Aimed`, but no production \
                 {MIGRATE_OPERAND_HOME:?} call sits in it — the callers in that file are: \
                 {callers:?}"
            )),
            Aim::DeclaredOut if reaches => offenders.push(format!(
                "  {file}::{function}: disposed `DeclaredOut`, but it calls one of \
                 {MIGRATE_OPERAND_HOME:?}"
            )),
            _ => {}
        }
    }
    assert!(
        offenders.is_empty(),
        "a disposition nothing checks is a sentence, not a disposition:\n{}",
        offenders.join("\n"),
    );
}

/// The table is the class: a producer converted later without joining it reddens.
#[test]
fn every_production_caller_of_the_migrate_operand_home_is_a_row() {
    let mut unlisted: Vec<String> = Vec::new();
    for dir in ["crates/cli/src", "crates/engine/src"] {
        for path in root_walk::files_in(&workspace_root().join(dir), root_walk::ext("rs")) {
            let rel = format!("{dir}/{}", path.file_name().unwrap().to_string_lossy());
            if rel == "crates/engine/src/finding.rs" {
                continue; // the home's own definition
            }
            for owner in home_callers(&rel, MIGRATE_OPERAND_HOME) {
                if !MIGRATE_SPAN_SITES
                    .iter()
                    .any(|(file, function, ..)| *file == rel && *function == owner)
                {
                    unlisted.push(format!("  {rel}::{owner}"));
                }
            }
        }
    }
    unlisted.sort();
    unlisted.dedup();
    assert!(
        unlisted.is_empty(),
        "these production sites render the based `jigc migrate` operand and are in no row of \
         `MIGRATE_SPAN_SITES`:\n{}",
        unlisted.join("\n"),
    );
}

// ---------------------------------------------------------------------------------
// Arm 4 — the key-leak invariant, driven
// ---------------------------------------------------------------------------------

/// **The absolute is allowed in the route and nowhere else** — the half of the decision that
/// is easiest to lose, and the one the rc.18 review's MEDIUM 1 already closed once.
///
/// A `(code, target)` key and an `at:` locus are what a driver dedupes on and what a reader
/// pastes back into an address; an absolute one names a *machine*, not a repository, and is
/// not portable across the two checkouts a fan-out is made of. So the split is asserted on
/// the pinned `--format json` envelope, at the census's rank-1 row (`finalize.carried-staged`)
/// and at the row that opened the census (`repo.operation-in-progress` at a fan-out worktree).
#[test]
fn the_absolute_rides_the_route_and_never_the_key_the_locus_or_the_message() {
    use crate::support::trial_corpus::{State, TrialCorpus};

    // C1-01 — the carryover gate, through the real binary.
    let corpus = TrialCorpus::build(State::Fresh);
    std::fs::write(corpus.repo().join("foreign.txt"), "not this task's work\n")
        .expect("plant a pre-mint staged file");
    corpus.git(&["add", "foreign.txt"]);
    let task = corpus.start_workflow("quick-fix", "probe the carryover gate");
    let out = corpus.jigc(&["task", "validate", &task, "--format", "json"]);
    let envelope: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("the findings envelope parses");
    let carried = envelope["findings"]
        .as_array()
        .expect("findings is an array")
        .iter()
        .find(|f| f["code"] == "finalize.carried-staged")
        .unwrap_or_else(|| panic!("the plant must fire the gate; got:\n{envelope:#}"));

    let root = corpus
        .repo()
        .canonicalize()
        .unwrap_or_else(|_| corpus.repo())
        .display()
        .to_string();
    assert_eq!(
        carried["key"]["target"], "foreign.txt",
        "the key is repo-relative"
    );
    assert_eq!(
        carried["location"]["address"], "foreign.txt",
        "the `at:` locus is repo-relative"
    );
    for (field, value) in [
        (
            "key.target",
            carried["key"]["target"].as_str().unwrap_or_default(),
        ),
        (
            "location.address",
            carried["location"]["address"].as_str().unwrap_or_default(),
        ),
        ("message", carried["message"].as_str().unwrap_or_default()),
    ] {
        assert!(
            !value.contains(&root),
            "`{field}` must carry no host path — an absolute there names a machine, not a \
             repository, and is not portable across the two checkouts a fan-out is made of; \
             got: {value}"
        );
    }
    let route = carried["route"].as_str().expect("blocking ⇒ routed");
    assert!(
        route.contains(&format!("git -C {root} restore --staged -- foreign.txt")),
        "…and the route DOES carry it, because those bytes are run rather than read; \
         got: {route}"
    );
}

/// The same split at the census's own opening row, through the posture family's shipped
/// producer rather than a hand-built equivalent.
#[test]
fn the_fan_out_posture_site_keys_repo_relative_and_routes_absolute() {
    use crate::support::git_state::{GitState, GitStateRepo};
    use cli::repo::{BreachSite, PostureMember, posture};

    let repo = GitStateRepo::build(GitState::Bisect);
    let breach = posture(&repo.repo())
        .into_iter()
        .find(|b| b.member() == PostureMember::OperationInProgress)
        .expect("a bisect is an operation in progress");
    let at = ".jigc/worktrees/area-low";
    let abs = repo.repo().join(at);
    let finding = breach.finding_at(BreachSite::FanOutWorktree { at, abs: &abs });

    assert_eq!(
        finding
            .location
            .as_ref()
            .and_then(|l| l.address.as_deref())
            .unwrap_or_default(),
        at,
        "the locus — and so the `(code, target)` key — is the repo-relative worktree",
    );
    assert!(
        !finding.message.contains(&repo.repo().display().to_string()),
        "the message carries no host path; got: {}",
        finding.message,
    );
    let route = finding.route.as_deref().expect("blocking ⇒ routed");
    assert!(
        route.contains(&format!("git -C {}", abs.display())),
        "the route names the checkout, absolutely — `-C` takes a directory, and driven, the \
         repo-relative spelling exited 128 from every cwd but the repository root; \
         got: {route}",
    );
}
