# Robust-case advocacy for fork promote-2

## robust_case

**Verdict: robust-now, in a smaller shape than the planner priced.** The planner's objection that the lead rests on one un-bounded drive no longer holds: I drove five cells on the installed `jigc 1.0.0-rc.24` (scripts in `<scratch>/adv-promote2/`, `c1.sh`–`c5.sh`). The lead is wider than recorded.

**The root cause is one split, read in the code.** From a user-made linked worktree, `task finalize` reads at one checkout and writes at another:
- Copy-in reads the main checkout: `engine::store::canonical_path(&self.jigc_home, …)` in `read_or_copy_in`, `crates/cli/src/doc.rs`.
- The reconciler and the base-pin read bind to the main checkout: `validate_task(…, &self.jigc_home, …)` and `pin_home = self.jigc_home`, in `crates/cli/src/task.rs` `validate`.
- The clobber guard binds to the main checkout: `plan_finalize(&self.dir, &self.jigc_home, …)`.
- The write lands in the linked checkout: `promote(repo_root, …)` does `fs::copy` to `repo_root.join(destination)`.

So every guard this family's plan adds or repairs is aimed at a checkout the task door does not write into.

**What I drove (all exit 0 unless stated, text format, before-controls found each mark):**

| Cell | State | Result |
|---|---|---|
| C1 | Baseline present; hand edit in the linked worktree after copy-in | The edit is gone from the worktree, every blob, `.jigc/` and `fsck`. Re-confirms the recorded lead. |
| C3 | Fresh rig; an untracked hand-written file at `docs/decisions/single-node-cache.md` in the linked worktree; the task creates that ADR | `doc create` and `task finalize` both succeed; the untracked bytes are in no git object. In the main checkout the same sequence is refused at `doc create` with `write.title-ignored`. |
| C4 | Baseline present; no uncommitted edit anywhere; the feature branch carries a committed wording change to `VISION.md` | The first task from the linked worktree commits "restate the thesis" and silently reverts the branch's committed wording. Recoverable at `HEAD~1`, so a silent revert, not byte loss. |
| C2 | Two sequential tasks in one linked worktree, no hand edit | Task 1's finalize writes the linked bytes' hash into the shared `file-state.json`. Task 2 then exits 3 with `reconciliation.conflict-block` ("an external edit…") although no external edit exists; its "revert the external edit" route has nothing to revert. The main checkout's `jigc validate` reports its own clean `VISION.md` as differing from the recorded state. |
| C2b | C2, then `jigc unmanage VISION.md` as the way out | Finalize lands and silently reverts task 1's committed prose on the branch. |
| C5 | A doc task minted in the linked worktree, finalized from the main checkout (branches level, and feature one commit ahead) | Lands on main in both arms. This is the candidate route for a refusal, and it runs today. |

**The contract the robust option restores is already written.**
- `design/storage.md` → *Code resolves against the worktree; the doc-store + `.jigc/` resolve against jigc_home*: "The committed doc-store has one canonical home (the main checkout); resolving it against a worktree would be the worktree-doc-bleed the merge invariant above forbids." A promote into a linked worktree is that bleed.
- `design/reconciliation.md` → *What reconciliation does NOT do*: "No silent discard, ever … There is no door exemption from this sentence."
- `VISION.md`: out-of-band edits are "detected and routed, never forbidden", and placement of every write is the CLI's.
- The clobber guard's own rustdoc (`plan_clobber_guard`, `crates/engine/src/finalize.rs`) exempts an edited-from-base doc because it is re-promoted "over its own canonical path". From a linked worktree the destination is not the path it was copied from, so the exemption is applied outside its stated premise.

**The door already knows.** `TaskCtx::commit_site()` computes `CommitSite::differing(jigc_home, repo_root)`, and both the forecast and the ack print "in the linked worktree at … — not in the main checkout jigc's workbench binds to", then promote anyway. Law 1 of `design/surface-contract.md` ("nothing lies") is broken twice in the driven cells: C2's false "external edit", and the main checkout's "baseline lags `HEAD`".

**This is the class, not an instance.** The M53 entries in `DECISIONS.md` swept this same `jigc_home`/`repo_root` split door by door and recorded the lesson each time:
- 2026-09-23, `setup` and `uninstall` bind `jigc_home`: "The class is not the two reported symptoms; it is every path the two doors touch."
- 2026-09-23, the cwd fixes' review: "the half-swept pair".
- 2026-09-22, the milestone boundary fix: "The class is the *subject* of the probe, not the two members reported."

The cwd census row C2-09 (`completions/artifacts/M53/cwd-census.md`) kept the commit targeting for code and never asked where the docs land.

**Minimal-correct robust scope.** I do not argue for unifying the two roots. I argue for something smaller than the planner's "new refusal with a code still to be chosen":
- When `commit_site()` is `Some`, any promotion whose destination in the landing checkout holds an entry is refused under the existing `finalize.promote-clobber`, keyed at the destination, on the existing `promotion` gate row.
- The message names the linked checkout; the route is to finalize from the main checkout (driven in C5).
- A code-only task still commits in the linked worktree, so the C2-09 decision and its pinned test are untouched.

## what_the_narrow_option_leaves

**Reachable at the new stamp, through `jigc task finalize` run in a user-made linked worktree, with all four narrow fixes landed:**

1. **Hand edit lost (C1, driven).** An uncommitted hand edit to a copied-in doc in the linked worktree is destroyed at exit 0, with the baseline present or absent. The narrow (R3, F7) fix compares the main checkout's bytes against the pin and never reads the file the promote overwrites.
2. **Untracked file lost (C3, driven).** An untracked file at a created doc's home in the linked worktree is overwritten at exit 0, with no after-the-fact edit and no baseline question. The narrow D-1/D-7 planner guard and the K-1 create probe all read the main checkout's home, which is free. Only D-7's sink re-read sits on the right root, and it refuses non-regular entries only.
3. **Committed content silently reverted (C4, driven).** The first task on a branch whose doc differs from the main checkout's reverts the branch's wording inside an unrelated commit. This is recoverable from history, so it is not byte loss. The narrow fix does not touch it, because with a baseline present the UNKNOWN arm is never reached.
4. **Shared baseline poisoned (C2, driven).** After one landed doc finalize from the linked worktree, the next task there gets a false `reconciliation.conflict-block` with a route that cannot be followed. The escape, `jigc unmanage`, leads to a silent revert today (C2b).

**Two consequences are reasoned from the plan text and the code, not driven, because the fix is not built:**
- After the narrow fix, the `unmanage` escape in item 4 becomes a permanent block. The plan's own test `unmanage_after_the_block_does_not_switch_the_guard_off` pins that.
- The `new: true` findings doctypes share the C3 hole, since their create gate reads the same main-checkout home.

**The plan's "class closed" claims are false from a linked worktree** for the task door in each row — for example D-7's "placement and location homes, both committing doors". So is the §1 contract sentence the pass will write into the design docs.

**Would the narrow fix survive the re-review? No.** C1 and C3 meet the tier-1 predicate literally (exit 0, committing door, bytes in no git object), and the planner has already scheduled the adversarial drive that will find them. C3 would be filed against rows this pass claims to close, so it sits inside the pass's own new guards. The exit rule then forces a second fix pass and a second re-review, unless the human argues the cell down to tier 2.

## what_the_robust_option_really_costs

**Build cost of the minimal shape: roughly one more commit on a pass of five.**
- **Code.** Thread the landing root into `plan_clobber_guard`, which D-1 is already re-signaturing and D-7 is already giving a shape arm. Add one message-and-route arm to `clobber_finding`, as D-1 adds its milestone arm. The predicate inputs already exist on the door: `commit_site()` and the plan's promotions.
- **Tests.** One new suite: C1, C3 and C4 as red cells on placement and location doctypes, the route followed to exit 0, and controls for a code-only linked finalize, a created doc into a free linked home, and the main checkout unchanged.
- **Docs.** One sentence each in `design/finalize.md` and `design/storage.md`, the `task finalize` help text, and a `DECISIONS.md` line.
- **Nothing pinned moves.** No finding code, no JSON key, no contract version, no schema, no `GATE_COVERAGE` row.

**Behaviour cost, stated plainly.**
- A command that exits 0 today exits 3. Any task run from a user-made linked worktree that edits an existing doc is refused at finalize and routed to the main checkout. In the default `single-task` workflow that includes any task that records a changelog entry, because `CHANGELOG.md` exists on every branch.
- What is taken away works once and then wedges or reverts (C2, C2b, C4), so it is not a working capability.
- A task carrying both staged code in the linked worktree and docs has no one-commit route. Finalizing from the main checkout lands the docs there and leaves the code staged in the linked worktree. I did not drive that mixed arm.
- `finalize.promote-clobber` sits on a finalize-only gate, so `--dry-run` forecasts the refusal but `task validate` and `jigc start` do not. Law 3 ("nothing ambushes") is served only by the help sentence.
- The planner's fuller shape, previewed on all three surfaces, costs a `GATE_COVERAGE` row that propagates to eight enumerating surfaces. That is the part closest to new mechanism, and I do not argue for it in this pass.

**What I did not verify.**
- Whether any existing suite promotes a doc from a linked worktree and expects exit 0. The one pinned linked finalize I found, in `crates/cli/tests/cwd_verb_subject.rs`, is code-only. This was a grep, not a census.
- `--format json` on any cell.
- Whether reviewers will accept reusing `finalize.promote-clobber` for an edited-from-base promote. If they do not, the fallback is one new code value, which is a spend on the key drivers read.
- All drives were on macOS against the registry binary.

**The alternative, unifying the two roots, is out of a fix pass.** `jigc_home` appears about 800 times across `crates/cli/src`, and unifying reverses storage.md's one-canonical-home rule. That is a wave.

**Cost of the narrow option instead.** A second fix pass, another stamp and release, and another re-review, when the scheduled drive finds what these cells already show.

## would_it_be_new_mechanism

**No, by the boundary's own list; yes in one honest sense.**

- **No:** it adds no capability, knob, store or doctype, and moves no frozen schema, pinned JSON key or contract version. In the minimal shape it reuses the existing code `finalize.promote-clobber`, the existing `(code, target)` key, the existing `promotion` gate row and the existing `CommitSite::differing` predicate, which the door already computes and prints. It restores a written sentence: storage.md's one canonical home for the doc store, and reconciliation.md's "no door exemption".
- **Yes, in this sense:** it is a new refusal on a door that exits 0 today, and it is keyed on a checkout relation rather than on bytes. That is the same kind of move this pass already makes four times: (R3, F7) turns the pinned silent-merge order into a block (fork `promote-1`), D-1 and D-7 add refusals, and the worktree plan's row P turns an exit 0 into `milestone.provision-failed`.
- **What would be new mechanism, and is not argued for:** a new previewed gate across `task validate`, `--dry-run` and `jigc start` (a `GATE_COVERAGE` row), a new finding code, or unifying `jigc_home` with `repo_root`.

If the human reads the boundary as "no command that exits 0 today may refuse on a new predicate", the robust option is out. The honest fallback is then a declared-bound sentence in `design/finalize.md`, with the cell recorded as a driven tier-1 candidate rather than an ungraded lead.

## verdict_for_the_human

You are choosing between shipping the stamp with a driven exit-0 loss still open at `task finalize` from a user-made linked worktree — which the scheduled re-review drive will find, including inside the rows this pass claims to close — and adding one more refusal to this pass under the existing `finalize.promote-clobber`, with a route that already runs. Robust costs roughly one extra commit and makes doc-editing tasks from a linked worktree finalize from the main checkout; narrow costs a likely second fix pass and re-review, or an explicit decision to tier a deterministic loss cell at 2.

