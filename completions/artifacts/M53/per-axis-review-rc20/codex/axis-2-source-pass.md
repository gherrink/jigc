<!-- The unseeded Codex source pass for AXIS 2, verbatim. Source read at `1.0.0-rc.20` (repo HEAD `4d3175c3`), 2026-09-27. It drove nothing. -->

## Axis 2 source pass — M53 fourth partial rerun

Source read at `cb0de4ac` (`1.0.0-rc.20`). Read-only: I did not drive the binary, run tests, or write files. “CLOSED (argv)” below preserves the rc.20 driver datum; source inspection confirms the closure still exists.

### Claims

1. **STILL-OPEN (datum, expected tier 3): a foreign repository’s worktree placed at `.jigc/worktrees/<sub-id>` is still accepted as a live sub-task worktree and can contribute staged bytes to `milestone finalize`.**

   - Evidence: `subtask_worktrees` classifies any path for which `classify_leftover` returns `OwnWorktree` as `WorktreeState::Live`, without checking that its Git common directory belongs to `jigc_home` ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3926), [milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3951)). `provisioned_worktrees` then selects every such `Live` path ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:4081)); those paths supply the staged-code signal used by the boundary ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:4175), [milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5340)).
   - Proposed reproduction: create a milestone and sub-task without provisioning it; from a second repository, create a linked worktree exactly at the expected `.jigc/worktrees/<sub-id>` path and stage a unique file there; run `jigc milestone join <milestone>` and `jigc milestone finalize <milestone>`. Expected wrong behavior: exit 0 and the unique staged bytes are attributed to the jigc sub-task and committed.
   - Confidence: high. This is rc.20 `(2, F-2)`, not a new F-10 regression.

### rc.20 row dispositions

- **CLOSED (argv):** all ordinary detached/unborn and all ten `InProgress::ALL` rows across acting doors; operation detection still precedes detached-HEAD diagnosis, including `UncommittedCherryPick` before the unmerged-index fallback ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:301), [repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:320)).
- **CLOSED (argv):** setup’s unborn exemption remains setup-only; `live_exempt` documents its single caller, while every ordinary commit uses `SeamSubject::live` ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:983), [setup.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/setup.rs:2586)).
- **CLOSED (argv):** `(2, F-1)` aimed posture routes and the repo-relative locus/absolute route split; unchanged by F-10.
- **CLOSED (argv):** rc.20 N-1 and N-2, the pre-commit-hook false rename and missing placement-family backstop, were changed in the usability batch.
- **STILL-OPEN (datum), expected 1.x:** `(2, DEFECT B)`, because `SquashMerge` remains exactly `SQUASH_MSG && !MERGE_HEAD` ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:323)).
- **STILL-OPEN (datum), expected 1.x:** `(2, DEFECT C)`, because a seam-time posture failure still exits through the raw commit error rather than reconstructing the door’s full state-truth/re-run envelope ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7662)).
- **STILL-OPEN (datum), expected tier 3:** `(2, F-2)`, claim 1.
- **STILL-OPEN (datum), expected tier 3:** N-3, because provisioning still reuses a registered exact-path worktree untouched rather than proving its HEAD equals the milestone pin ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:2947)).
- **STILL-OPEN (datum), expected tier 3:** N-4’s shared-workbench-baseline diagnosis; the usability/F-10 range did not change that model.

### M51 confirmed-row source disposition

- **D1 CLOSED:** operation-first probing prevents rebase/bisect from falling through to detached HEAD ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:301)).
- **D2 CLOSED:** apply-backend `git am` and rebase remain disjoint predicates ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:328)).
- **D3 CLOSED:** cherry-pick, revert, sequencer, uncommitted-pick, and unmerged-index states remain explicit ordered members ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:301)).
- **D3b CLOSED:** every hook-capable commit, including amend, converges on `git_commit_capture`, whose immediately preceding statement is `subject.verify(Commit)` ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7662)).
- **Prior Codex displacement claim CLOSED:** both `git mv` and displacement `git rm --cached` have an immediate `verify(Move)` ([relocate.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:238), [relocate.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:811)).

### New F-10 code and completeness

The new code introduces no axis-2 bypass:

- `task amend` is correctly `Neither`: it only mints workbench state; `task finalize` performs the commit ([cli.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2193)).
- The second finalize commit model is separately registered, raising `COMMITTING_DOORS` to 11 while leaving the commit-on-behalf leaf set at ten ([invocation_log.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/invocation_log.rs:188)).
- `git_commit_amend` supplies `--amend -F` to the same immediately re-probed seam ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7558)).
- Its dirty-index and promoting-doc gates precede planning/promotion ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:2955)); both codes have `DeclaredWhereReachable` ambush rows ([pack.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/pack.rs:1018)).
- `DedicatedWorktree` remains unforgeable outside `task.rs`: private fields, private constructor, typed `SeamSubject::dedicated` ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:8010), [repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:1006)).
- The fast-forward merge still has an immediate commit re-probe ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7871)). No production HEAD-changing `switch` or ordinary `checkout` exists.

I also traced the named M52 registries and their consumers. None adds a commit/move act or bypasses the posture seam. No schema or doctype-manifest file changed in `609da011..HEAD`; the M52 zero-schema-hash boundary is intact.

Bounds: source-only; no binary rerun, concurrency race, `GIT_DIR` redirect, submodule, `core.worktree`, or alternate-Git-version claim.