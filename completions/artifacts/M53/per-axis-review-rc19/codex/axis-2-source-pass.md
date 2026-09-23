<!-- M53 THIRD PARTIAL per-axis review — axis 2 — the unseeded Codex SOURCE pass, verbatim. Source-only: it drove nothing and wrote nothing. Captured 2026-09-23 against the rc.19 tree. -->

## Axis 2 source pass — rc.19 (`1cc5da8d`)

### Claims

1. **STILL-OPEN (datum): a directory at `.jigc/worktrees/<sub-task>` belonging to another repository is still accepted as a live sub-task worktree, allowing `milestone finalize` to commit that repository’s staged bytes as jigc work.**

   - Evidence: `provisioned_worktrees` merely filters `subtask_worktrees` by `WorktreeState::Live`, without verifying repository identity or Git common-dir ownership ([crates/cli/src/milestone.rs:4081](crates/cli/src/milestone.rs#L4081), [crates/cli/src/milestone.rs:4091](crates/cli/src/milestone.rs#L4091)). Those paths feed the boundary gate, staged-code test, and contribution accounting ([crates/cli/src/milestone.rs:5071](crates/cli/src/milestone.rs#L5071), [crates/cli/src/milestone.rs:5288](crates/cli/src/milestone.rs#L5288), [crates/cli/src/milestone.rs:5340](crates/cli/src/milestone.rs#L5340)). By contrast, `posture_subject`’s registry leg proves only that the directory name is a registered jigc sub-task; it does not prove the checkout belongs to the same repository ([crates/cli/src/repo.rs:878](crates/cli/src/repo.rs#L878), [crates/cli/src/repo.rs:908](crates/cli/src/repo.rs#L908)).
   - Proposed reproduction: create a jigc milestone/sub-task but do not provision it; use a second repository’s `git worktree add` to place an attached worktree at `.jigc/worktrees/<sub-id>`; stage a unique file there; run `jigc milestone join <id>` followed by `jigc milestone finalize <id>`. Expected wrong behavior: exit 0, the unique file appears in the jigc repository’s new commit, and the acknowledgement attributes it to the sub-task.
   - Confidence: high. This is rc.18 `(2, F-2)`, an expected tier-3 STILL-OPEN row, not a new rc.19 regression.

### Baseline-row dispositions

- `(2, DEFECT 1)` — **CLOSED (argv)**: all ten `InProgress::ALL` members remain enumerated, including `UncommittedCherryPick` and `UnmergedIndex` ([crates/cli/src/repo.rs:287](crates/cli/src/repo.rs#L287)); fan-out boundary checks consume the worktree paths before commit construction.
- `(2, DEFECT A)` / M51 `D3` — **CLOSED (argv)**: the four uncommitted-pick shapes remain represented by `UncommittedCherryPick`; its detection follows every marker-backed member and precedes the unmerged-index fallback ([crates/cli/src/repo.rs:295](crates/cli/src/repo.rs#L295), [crates/cli/src/repo.rs:340](crates/cli/src/repo.rs#L340)).
- M51 `D1` — **CLOSED (argv)**: operation detection retains precedence over detached-HEAD rendering through the ordered operation table and posture breach construction ([crates/cli/src/repo.rs:287](crates/cli/src/repo.rs#L287), [crates/cli/src/repo.rs:570](crates/cli/src/repo.rs#L570)).
- M51 `D2` — **CLOSED (argv)**: `rebase-apply/applying` is `Am`, with `git am --abort`; other `rebase-apply` is `Rebase`, with `git rebase --abort` ([crates/cli/src/repo.rs:327](crates/cli/src/repo.rs#L327), [crates/cli/src/repo.rs:456](crates/cli/src/repo.rs#L456)).
- M51 `D3b` — **CLOSED (argv)** on source: committing doors converge on `git_commit_capture`, which re-verifies before invoking Git and returns Git’s rejection bytes ([crates/cli/src/task.rs:7100](crates/cli/src/task.rs#L7100)).
- M51 `codex-1` — **CLOSED (argv)**: the displacement’s `git rm --cached` now has `SeamSubject::live(...).verify(Move)` immediately before it ([crates/cli/src/relocate.rs:790](crates/cli/src/relocate.rs#L790), [crates/cli/src/relocate.rs:802](crates/cli/src/relocate.rs#L802)); ordinary `git mv` has the same immediate re-probe ([crates/cli/src/relocate.rs:238](crates/cli/src/relocate.rs#L238)).
- `(2, F-1)` — **CLOSED (argv)**: `aim_at` now delegates to `git_at`, producing `git -C <quoted absolute> …` ([crates/cli/src/repo.rs:688](crates/cli/src/repo.rs#L688), [crates/cli/src/repo.rs:701](crates/cli/src/repo.rs#L701), [crates/engine/src/finding.rs:904](crates/engine/src/finding.rs#L904)).
- `(2, F-2)` — **STILL-OPEN (datum)**: claim 1 above; expected tier 3.
- `(2, DEFECT B)` — **STILL-OPEN (datum), expected 1.x**: `SquashMerge` is still detected solely by `SQUASH_MSG && !MERGE_HEAD` ([crates/cli/src/repo.rs:323](crates/cli/src/repo.rs#L323)).
- `(2, DEFECT C)` — **STILL-OPEN (datum), expected 1.x**: the commit seam still returns a raw operational error rather than the door’s full state-truth/re-run envelope ([crates/cli/src/task.rs:7100](crates/cli/src/task.rs#L7100)).
- rc.18 MEDIUM 1 — **CLOSED (argv)**: fan-out messages/locations retain the repo-relative `at`, while the runnable route carries the separately stored absolute path ([crates/cli/src/repo.rs:594](crates/cli/src/repo.rs#L594), [crates/cli/src/repo.rs:661](crates/cli/src/repo.rs#L661)).

### Completeness and consistency read

I read `CLAUDE.md`; all requested M51/M52/M53 §A material; the M51 prior source pass; the cwd census; VERDICT Addendum 2; the four 2026-09-23 decisions; `repo.rs`; `BEHALF_DOORS`; `COMMITTING_DOORS`; and all production Git commit/move/merge/remove sites.

- `BEHALF_DOORS` remains total by a clap-tree bijection fence ([crates/cli/src/cli.rs:4797](crates/cli/src/cli.rs#L4797)). Its acting set remains ten commit-on-behalf leaves plus two mover leaves; the 35 other leaves explicitly say `Neither` ([crates/cli/src/cli.rs:2004](crates/cli/src/cli.rs#L2004)).
- `COMMITTING_DOORS` remains the hook-capable subset; `setup` is the intentional `--no-verify` exception ([crates/cli/src/invocation_log.rs:145](crates/cli/src/invocation_log.rs#L145), [crates/cli/src/cli.rs:2018](crates/cli/src/cli.rs#L2018)).
- Relevant acts and immediate probes:
  - ordinary commits: `git_commit_capture` calls `verify(Commit)` immediately before `git commit`;
  - setup commit: `live_exempt(...HeadUnborn).verify(Commit)` immediately before its commit ([crates/cli/src/setup.rs:2555](crates/cli/src/setup.rs#L2555));
  - `git mv`: immediate `verify(Move)` ([crates/cli/src/relocate.rs:238](crates/cli/src/relocate.rs#L238));
  - displacement `git rm --cached`: immediate `verify(Move)` ([crates/cli/src/relocate.rs:802](crates/cli/src/relocate.rs#L802));
  - live fast-forward `git merge --ff-only`: immediate `live.verify(Commit)` ([crates/cli/src/task.rs:7309](crates/cli/src/task.rs#L7309)).
- No production HEAD-changing `git switch` or ordinary `git checkout` was found; `checkout-index` and `read-tree` do not change HEAD.
- No `Neither` leaf reaches these primitives.
- `DedicatedWorktree` remains unforgeable through public construction: private fields and private `add` constructor ([crates/cli/src/task.rs:7448](crates/cli/src/task.rs#L7448), [crates/cli/src/task.rs:7456](crates/cli/src/task.rs#L7456)).
- The setup unborn exemption remains confined to setup; ordinary seams use `SeamSubject::live`.
- The M52 registries named in the brief do not introduce an axis-2 bypass. `InProgress::ALL` is the posture authority; the other registries govern rollback, work-area contents, pre-dispatch precedence, destruction, envelopes, fixed identity, and store exits rather than adding commit/move acts.
- `discover_repo_root` has one implementation, and store/check-out subject choice is now explicit at callers ([crates/cli/src/repo.rs:109](crates/cli/src/repo.rs#L109)).

No schema-hash movement or schema-shape change was observed in this arc; the stated M52 zero-schema-hash boundary remains intact.

### Bounds

Source-only: I did not drive the binary or run tests. A genuine concurrent racer inside a seam’s apply window, `GIT_DIR` redirection, alternate Git versions, submodules, `core.worktree`, and the same-repository variant of `(2, F-2)` remain outside this pass.