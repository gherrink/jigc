No new TIER-1 lead found. The post-review source closes M53 `(2, DEFECT 1)`: every provisioned worktree the milestone boundary commits from is now posture-probed before any durable write. M52’s lower-tier DEFECT B and DEFECT C remain open.

## Prior-row dispositions

### M53 rc.18 §A — axis 2

- **`(2, DEFECT 1)` — CLOSED.** `milestone finalize` derives the live worktree set once, then passes that same set to the posture probe and subsequent boundary consumers ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5155), [milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5160)). `fan_out_posture_findings` invokes the shared `repo::adjudicated_breach` on every worktree and refuses all members at `BreachSite::FanOutWorktree` ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3946)). The refusal occurs before the record flip or commit construction ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:5163)). `task validate <sub>` forecasts the identical boundary finding when invoked outside the sub-task worktree ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3985), [task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:496)).

### M52 §A — axis 2

- **DEFECT A — CLOSED.** `UncommittedCherryPick` is a member of the ten-row operation registry, ordered before `UnmergedIndex` ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:291)). Its predicate recognizes marker-negative `MERGE_MSG` state ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:330)); its conclude and preserving-abandon commands are `git commit` and `git reset` ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:406), [repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:439)).
- **DEFECT B — NOT CLOSED.** See Claim 1.
- **DEFECT C — NOT CLOSED.** See Claim 2.

### M51 §A — axis 2

- **codex-1 — CLOSED.** The squatter path re-probes immediately before `git rm --cached` ([relocate.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:790)); `git mv` independently re-probes immediately before execution ([relocate.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/relocate.rs:238)).
- **D1 — CLOSED.** `adjudicated_breach` consumes the operation-first posture result and selects the first owed breach ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:780), [repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:1124)).
- **D2 — CLOSED.** `git am` and apply-backend rebase have disjoint predicates and distinct commands ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:321), [repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:446)).
- **D3 — CLOSED**, including the clean `cherry-pick -n` cell, by the `UncommittedCherryPick` member above.
- **D3b — CLOSED.** Git/hook commit failures retain typed commit-failure handling and the survivable frame ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5274), [task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5397)).

## Claims

1. **M52 DEFECT B remains: a conflicted squash merge is classified as a completed staged squash rather than as an unmerged-index conflict.**

   - Evidence: `SquashMerge` detects `SQUASH_MSG && !MERGE_HEAD` without consulting unmerged index entries ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:313)). It precedes `UnmergedIndex` in probe order ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:291)) and renders “is staged and not committed” ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:364)).
   - Reproduction: create conflicting branches; run `git merge --squash <branch>`; verify `SQUASH_MSG` and non-empty `git ls-files -u`; invoke `jigc milestone create probe`. Expected wrong behavior: `repo.operation-in-progress` says a squash merge “is staged and not committed,” although unresolved conflicts remain.
   - Confidence: **high**.

2. **M52 DEFECT C remains: a posture race raised at a commit seam bypasses the committing door’s survivable frame.**

   - Evidence: posture errors are typed `BlockedFinding`s, and `already_typed` explicitly excludes every such error from `CommitFailed` wrapping ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5297)). The state-truth clause, concrete rerun argv, and door error identity are added only by `surface_commit_rejection` for `CommitRejected` or `CommitFailed` ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5397)).
   - Reproduction: use a Git shim to start `git bisect` after finalize staging but immediately before `git commit`; run `jigc task finalize <id>`. Expected wrong behavior: only `repo.operation-in-progress` and its generic route print; the intact-task clause, exact `jigc task finalize <id>` rerun, and committing-door identity are absent.
   - Confidence: **high**.

## Completeness and consistency

`BEHALF_DOORS` remains total over the leaf classification; `milestone finalize` is explicitly commit-on-behalf with no exemption ([cli.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:1999), [cli.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/cli.rs:2248)). `COMMITTING_DOORS` carries both milestone-finalize commit models and the other hook-capable commit doors ([invocation_log.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/invocation_log.rs:171)).

Relevant production acts remain guarded:

- ordinary commit: seam verification immediately precedes commit execution;
- setup commit: the sole unborn exemption is applied immediately before its commit ([setup.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/setup.rs:2409));
- fast-forward merge: live verification immediately precedes `merge --ff-only` ([task.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:7228));
- `git mv` and `git rm --cached`: both cited re-probes in `relocate.rs`;
- no production HEAD-changing `git switch` or `git checkout` was found.

`DedicatedWorktree` remains unforgeable through the public type: its discriminant is private, and the sole constructor requires a linked-worktree `.git` file, the canonical jigc worktree location, and registered milestone ownership ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:800), [repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:846)). The setup unborn exemption does not leak.

The post-review sibling fix is also present: provision/discard/uninstall classify a clean worktree with a live operation as held ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3699)); paths render against `jigc_home` ([milestone.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3951)); fan-out posture findings carry located filesystem targets while `Here` remains the declared singleton form ([repo.rs](/Users/maurice/projects/gherrink-jigc/crates/cli/src/repo.rs:645), [finding.rs](/Users/maurice/projects/gherrink-jigc/crates/engine/src/finding.rs:249)).

Bounds: read-only source pass; no binary drives, builds, races, or filesystem writes. Ambient `GIT_DIR` redirection remains declared out. Detection is bounded by the recorded Git 2.54.0 disk behavior. The post-rc.18 fix commits modify no schema manifests or schemas: **no schema-hash, schema-version, contract-version, or envelope-key movement found**.