# Axis 2 source pass — M53 partial re-review

**Result: no new TIER‑1 lead found.** M53 closes axis‑2 TIER‑1 `(2, DEFECT A)` in source. Two lower-tier M52 rows remain open: DEFECT B and DEFECT C.

## M52 §A row dispositions

- **`(2, DEFECT A)` — CLOSED.** `InProgress::ALL` now has ten members, placing `UncommittedCherryPick` after every marker-backed operation and before `UnmergedIndex` ([repo.rs:258](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:258)). Its predicate recognizes `MERGE_MSG` while excluding `MERGE_HEAD`, `CHERRY_PICK_HEAD`, `REVERT_HEAD`, `SQUASH_MSG`, `rebase-merge`, and `rebase-apply` ([repo.rs:291](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:291)). The member names `git commit` as the conclusion and plain `git reset` as the preserving abandon route ([repo.rs:387](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:387), [repo.rs:420](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:420)). Both door and seam guards consume this same operation-first probe ([cli.rs:599](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:599), [repo.rs:835](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:835), [repo.rs:924](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:924)). The clean, range, conflicted, and resolved `-n` variants all map to it in the derived fixture axis ([git_state.rs:243](/Users/maurice/projects/gherrink-jigc/crates/cli/tests/support/git_state.rs:243)).

- **`(2, DEFECT C)` — NOT CLOSED.** See Claim 2.

- **`(2, DEFECT B)` — NOT CLOSED.** See Claim 1.

## M51 axis‑2 row dispositions

- **`codex-1` — CLOSED.** `displace_foreign_squatter` calls `verify(Move)` immediately before `git rm --cached` ([relocate.rs:790](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:790)); the later `git mv` independently re-probes ([relocate.rs:238](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:238)).
- **D1 — CLOSED.** `posture()` emits the operation before detached/unborn HEAD ([repo.rs:924](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:924)), preventing rebase/bisect masking.
- **D2 — CLOSED.** `rebase-apply/applying` identifies `git am`, while bare `rebase-apply` identifies rebase ([repo.rs:291](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:291)); their nouns and abort commands are distinct ([repo.rs:330](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:330), [repo.rs:420](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:420)).
- **D3 — CLOSED, including M52’s formerly open clean-`-n` cell.** Marker-backed cherry-pick/revert remain explicit members, and M53 adds the marker-negative `UncommittedCherryPick` predicate cited above.
- **D3b — CLOSED.** A Git refusal is distinguished from a hook rejection using `GIT_HOOK_EXIT`, producing “failed,” not “rejected” ([task.rs:7030](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7030)).

## Claims

1. **M52 DEFECT B remains: a conflicted `git merge --squash` is classified as a fully staged squash merge rather than an unmerged-index conflict.**

   - Evidence: `SquashMerge` detects `SQUASH_MSG && !MERGE_HEAD` without testing for unmerged entries ([repo.rs:291](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:291)); it precedes `UnmergedIndex` in `ALL` ([repo.rs:272](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:272)) and renders “is staged and not committed” ([repo.rs:348](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:348)). The fixture registry still models only the clean squash variant—`SQUASH_MSG`, zero unmerged entries ([git_state.rs:337](/Users/maurice/projects/gherrink-jigc/crates/cli/tests/support/git_state.rs:337)).
   - Proposed reproduction: create conflicting branches; run `git merge --squash <branch>` and confirm `SQUASH_MSG` plus non-empty `git ls-files -u`; invoke `jigc milestone create probe`. Expected wrong behavior: refusal names “a squash merge is staged and not committed” instead of the extant conflict.
   - Confidence: **high**.

2. **M52 DEFECT C remains: a posture race detected at the commit seam bypasses the committing door’s survivable frame.**

   - Evidence: seam posture errors are `BlockedFinding`s ([repo.rs:846](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:846)); `already_typed` deliberately exempts every `BlockedFinding` from `CommitFailed` wrapping ([task.rs:5288](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5288)). Consequently `surface_commit_rejection` cannot attach the door’s state-truth clause, copy-runnable argv, or committing-door error identity, which it adds only for `CommitRejected`/`CommitFailed` ([task.rs:5388](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5388)).
   - Proposed reproduction: use a Git shim to start `git bisect` after finalize staging but before `git commit`; run `jigc task finalize <id>`. Expected wrong behavior: only `repo.operation-in-progress` with “re-run this command,” omitting the task-intact clause, concrete `jigc task finalize <id>` rerun, and door error identity.
   - Confidence: **high**.

## Completeness/consistency read

`BEHALF_DOORS` is bijective with the Clap leaves ([cli.rs:4711](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:4711)); its acting rows are 10 commit-on-behalf leaves plus `relocate` and `config set` as movers ([cli.rs:2003](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2003)). `COMMITTING_DOORS ⊆ CommitsOnBehalf` is asserted, with `setup` separately asserted as the sole no-hook addition and sole unborn exemption ([cli.rs:4939](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:4939)).

Relevant production acts:

- ordinary `git commit`: `verify(Commit)` immediately precedes it ([task.rs:7010](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7010));
- setup commit: unborn-only verification immediately precedes execution ([setup.rs:2409](/Users/maurice/projects/gherrink-jigc/crates/cli/src/setup.rs:2409));
- `git merge --ff-only`: live verification immediately precedes it ([task.rs:7219](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7219));
- `git mv`: verification immediately precedes it ([relocate.rs:238](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:238));
- `git rm --cached`: verification immediately precedes it ([relocate.rs:790](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:790));
- no production HEAD-changing `git switch` or `git checkout` was found; `checkout-index` is not such an act.

`DedicatedWorktree` remains unforgeable through private fields/private `add`, and `SeamSubject::dedicated` requires its live handle ([task.rs:7358](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7358), [repo.rs:789](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:789)). I found no `Neither` leaf reaching these commit/move seams and no setup unborn-exemption leak.

## Bounds

Read-only source pass only; I drove no binary and wrote nothing. `GIT_DIR` redirection remains explicitly out of scope. Detection remains bounded to Git 2.54.0’s observed disk contract. The schema/manifest/snapshot diff from M53’s base to `HEAD` is empty: **no schema-hash, schema-version, contract-version, or envelope-key movement observed**.