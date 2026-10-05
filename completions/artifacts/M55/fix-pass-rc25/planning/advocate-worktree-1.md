# Robust-case advocacy for fork worktree-1

## robust_case

VERDICT: robust-now. The narrow arm rests on one warrant, and I drove it false.

All cells below ran on the installed `jigc 1.0.0-rc.24`, fresh `dev/jigc-rig fresh` rigs, git 2.54.0. Scripts are in `<scratch>/adv-worktree1/` (`lib.sh`, `a.sh`, `b.sh`, `c.sh`, `e.sh`, `f.sh`). Nothing in the repository was edited.

**1. The planner's warrant is false: the record is restorable, and what it restores is the sub-task's deliverable.**

The plan says a refusal "protects nothing the reader can get back except the commit", because "git cannot repair a worktree that has no directory". Spike (`b.sh recover`):

- Setup: provisioned sub-task worktree, `alpha.rs` staged, never committed, directory then moved away. The working files were not brought back.
- Recovery, no jigc door run: `mkdir` the path, write the one-line `.git` file pointing at `.git/worktrees/add-alpha-file`, then `git restore .`.
- Result: `git status` reads `A  alpha.rs`, `jigc milestone provision` exits 0 and reuses it, `jigc milestone finalize` prints `added alpha.rs … add-alpha-file: 1 code file`, and `git show HEAD:alpha.rs` returns the staged bytes.

So the surviving index is the path-bearing copy of the sub-agent's staged code. That is the product's sanctioned carrier:

- `crates/cli/src/milestone.rs`, the zero-contribution route, tells the reader to "`git add` their work inside their worktrees".
- `design/team-ready-state.md` → *Abandon refuses on a dirty worktree* calls the worktree "the *sole copy* of a sub-agent's staged code".

**2. The product's own rule already assigns these doors the refusing arm.**

Same section of `team-ready-state.md`:

- "every door that removes a worktree-shaped path under `.jigc/worktrees/` probes that path first and answers for what it removes".
- "**Silence is the one arm no door may take**".
- "The three doors standing where **no commit has carried those bytes** refuse — `provision` … `discard` … `uninstall`".
- "The subject is the path, and the question is *'can I prove this path holds nothing precious?'*".

In code, `PROVISION_DOOR` and `DISCARD_DOOR` are `Disposition::Refuse { consent: "--force" }`. `Disposition::Narrate` is documented as the arm "for a door whose removal cannot be refused without firing on the ordinary success path" and "No member holds this disposition today".

The narrow arm gives two Refuse doors a Narrate arm for one subject. Neither Narrate warrant is available: a stale own record anchoring work is not the ordinary success path, and no commit carried those bytes.

**3. The precedent is the same loss, at the same probe, under the same fix-pass boundary.**

`DECISIONS.md` → 2026-09-22, *M53 post-review fix, sibling: the leftover classifier asks whether the worktree is concluded*:

- The loss was `.git/worktrees/<sub>/` going with a discard, "that checkout's reflog died and a commit reachable only from its detached HEAD dangled until `git gc`".
- It was graded HIGH and closed by refusal under each door's existing code, with `--force` the one consent, as a second leg on `probe_leftover`.
- The entry above it records that the fixer had carried the bound "on a warrant driven false" and "the human overturned the carry".

The narrow arm proposes the same carry.

**4. VISION and the exit-code contract.**

- `VISION.md` line 51: out-of-band edits are "detected and routed", "discard is explicit, never silent. No silent data loss". An out-of-band deletion of a worktree directory is that case. Honest scope: the sentence is written about managed docs; the worktree rule's home is `team-ready-state.md`.
- `design/command-output-contract.md` → exit semantics calls the exit vocabulary "the **first** contract a pipeline binds". `milestone provision` is emitted as a `Run:` step ahead of the fan-out, so its reader is an orchestrating agent that proceeds on exit 0 into fresh, empty worktrees. A stderr warning there is not consent.

**5. Class, not instance — and the fork as worded is itself too narrow.**

The class is "a jigc door drops one of its own registrations whose HEAD or index anchors work no ref reaches". The directory being absent is one member. I drove the live member, which needs no external deletion at all: a sub-agent commits inside its provisioned worktree, tree clean.

| door | exit | what is said | commit after |
|---|---|---|---|
| `milestone discard` (un-forced) | 0 | nothing | unreachable |
| `milestone finalize` | 0 | `add-alpha-file: nothing staged` | unreachable, file not in HEAD |
| `uninstall` (un-forced) | 0 | nothing about it | unreachable |

I found no record of this in the rc.24 review directory or `decisions-pending.md` (a grep, not a full read). Neither arm as worded closes it. The class-closing form is one anchor leg on the shared probe, which all three refusing doors inherit — the 2026-09-22 shape ("the leg lands once and all three inherit it").

**6. Robust fails safe; narrow fails as a loss.**

`DECISIONS.md` → 2026-09-16 (M51 audit) states the inverted default: forgetting a case "costs a loud false alarm `--force` clears, never a loss". A defect in a refusing guard is an annoyance; a defect or declared bound in a narrate-and-take arm is an exit-0 loss, which is the tier-1 predicate.

**Robust scope, minimal-correct:**

- One leg on `probe_leftover` (`crates/cli/src/milestone.rs`): an own registration whose HEAD is off the base pin, or whose index differs from its HEAD, is a hold — whether the directory is present or absent.
- Refused under the existing `milestone.leftover-holds-work`, `milestone.dirty-worktree` and `uninstall.dirty-worktree`, `--force` the consent.
- The per-path line carries the restore recipe, as the operation leg's line carries its abandon command.
- Under `--force`, and at the landed finalize, `PendingLoss` names the commit and the staged paths. That is the narrow arm's whole deliverable, so robust contains it.
- An empty stale record (HEAD at the pin, index equal to HEAD) still self-heals silently at exit 0, so M31's idempotent re-provision survives exactly where it is harmless.

**Where the planner is right:** unstaged and untracked bytes in a deleted directory are gone under either arm, the precondition for the stale cell is a non-jigc deletion, and the commit stays a `git branch` away until gc.

## what_the_narrow_option_leaves

All driven on rc.24. The keyed fix leaves each of these exactly as it is, and the narrow arm adds only a stderr line naming the path and the HEAD sha.

**(a) Stale own record, staged-only work — the ordinary fan-out shape (HEAD equals the base pin).**

- State: sub-agent ran `git add alpha.rs` in `.jigc/worktrees/add-alpha-file`; the directory is then removed without `git worktree remove`.
- `jigc milestone provision <id>`: exit 0, one `provisioned 2 worktree(s)…` line, stderr empty. The record's index is gone, the staged blob goes from 0 to 1 `fsck --unreachable` hits (pathless), and a fresh empty worktree stands in its place.
- `jigc milestone discard <id>` un-forced: exit 0, `workbench removed`, same loss.
- What the narrow warning would print here is the base pin — a commit every branch already reaches. The sub-task's deliverable, the only thing lost, is unnamed. The plan declares this "a stated bound".
- This is the half-truth the code's own comments name (RC-m50 N9 in `crates/cli/src/milestone.rs`: a door that names one of the two things it would destroy).
- The zero-contribution route sends the reader to `jigc milestone provision` saying "an existing one is reused" — the command that destroys the record.

**(b) Stale own record at the landed finalize.**

- State: alpha holds a commit and staged work, directory gone; beta is live and staged.
- `jigc milestone finalize`: exit 0, `sub-tasks: add-alpha-file: nothing staged, no worktree provisioned · add-beta-file: 1 code file`.
- The boundary lands without alpha's contribution, the record is destroyed, and both the commit and the staged blob become unreachable.
- "no worktree provisioned" is false: one was, and its record held staged code. The plan does not list this cell's post-fix behaviour for an absent directory.

**(c) Stale own record holding a commit.** The planner's cells, re-driven at provision: exit 0, silent, commit 0 → 1 unreachable. Narrow names the sha; the reflog and index still go.

**(d) The live sibling, closed by neither arm as worded.** A commit inside a live provisioned worktree, clean tree: `discard`, `finalize` and `uninstall` all exit 0 and leave it unreachable (table in the robust case). After the narrow fix the door warns when someone else deleted the directory and stays silent when jigc deletes it itself.

**Would the narrow fix survive the re-review? I do not think so.**

- Cell (a) is an exit-0 loss through two destroying doors, sitting in the pass's own new code (the keyed removal and the commit-6 narration). By the exit rule (`DECISIONS.md` → 2026-09-21) that is another fix pass and another partial re-review.
- Commit 6's own test claims "narrated ⇔ taken". The index is taken and not narrated, so the fence asserts a property the code breaks.
- The tier-2 argument (precondition is a non-jigc deletion) is the one the adversarial re-drive already refused for L-22: `completions/artifacts/RC-rc24/tier1-verification-L-22.md` §5 says the against-list "lowers likelihood and magnitude, not the predicate", and §6 lists "directory deleted by hand / by an agent's `rm -r`" as an equivalent trigger. Holding the foreign row tier 1 and its own-set mirror tier 2 needs a distinction I could not find. Here the lost work is jigc's own sub-agent's.
- The bound's warrant ("git cannot repair it") is driven false by the recovery spike. A reviewer who runs the same three commands has the 2026-09-22 finding again: a bound carried on a false warrant.
- Cell (d) is outside the new code, so it triggers a post-review fix either way. The narrow narration makes the asymmetry easier to see.

**Not driven:** the stale cell at `uninstall`; gc permanence for the own-set (taken from the L-22 gc cell by equivalence); git older than 2.54.0.

## what_the_robust_option_really_costs

**What both honest arms pay.**

- The HEAD read is not new. `git worktree list --porcelain` already prints `HEAD <sha>` and `prunable …` for the stale record (driven); `registered_worktrees` runs that command today and keeps only the `worktree` lines.
- The narrow arm needs the same read to print "the commit its HEAD held". So the HEAD leg does not separate the arms.

**What robust adds.**

1. **The index leg — the real cost, and it is new probe code.**
   - Locate the admin directory by matching `<git-common-dir>/worktrees/*/gitdir` against `<path>/.git`, then run `git --git-dir=<admin> diff --cached --quiet <HEAD>`. Spiked: prints `A alpha.rs`, exit 1.
   - Roughly one function. jigc already resolves the common dir and already reads files inside per-worktree admin directories (`crates/cli/src/repo.rs`: `worktree_git_dir`, `operation_in_progress`, `git_dir.join("HEAD")`).
   - What is new is locating the admin directory when no checkout exists to ask.
   - A HEAD-only robust arm would skip this and miss cell (a), the ordinary shape, so it is not the minimal-correct version.

2. **A stale arm in `LeftoverHold`.**
   - A new `LeftoverShape` or verdict member, with a match arm in each of `hold_line`, `held_here` and `because`.
   - Suites that iterate `LEFTOVER_VERDICTS` stop compiling until the member is handled, by design.
   - `probe_leftover` needs the base pin (or a pin-free test, "no ref contains this HEAD") threaded through three callers. `uninstall` spans milestones, so the pin-free form is likely the cheaper one there.

3. **Exit codes move in one cell family.**
   - `provision` and un-forced `discard` (and `uninstall`, if the leg sits in the shared probe) go from 0 to their existing blocking codes when an own record anchors a commit or staged paths.
   - The empty stale record stays exit 0.
   - The plan already accepts one such flip (row P) on the ground that what exit 0 reported was false. The same holds here: `provisioned 2 worktree(s)` over a destroyed deliverable.

4. **New refusal prose.**
   - A per-path line and a restore recipe carrying an absolute admin path and git spans.
   - Each span is a new row in the `git_span_aim.rs` census and must pass the route fence.

5. **Cells.** Roughly three refusing doors × {commit, staged, both, empty control} × {un-forced, `--force`}, plus the landed-finalize naming cell. More with the live sibling.

6. **The live sibling widens blast radius.** A sub-agent that commits instead of staging would now meet a refusal at `discard` and `uninstall`. That is off the product's stated script (the route says `git add`), so it should not train `--force`, but it is a behaviour change on a live path and deserves its own cell and its own ruling.

**What does not move:** no finding code, no flag, no JSON key, no contract version, no schema or manifest, no pack text. The planner reports no golden carries these messages; I did not re-check that.

**What I did not do:** build the leg in Rust, run the gate, or drive the recovery on the runner's older git. The recipe I spiked writes a `.git` file by hand; a refusal that prints it must be driven by pasting it verbatim, per the plan's own remedy test.

**Risk.** A refusing guard is new code under the same re-review and can be wrong. Its failure mode is a false refusal that `--force` clears.

## would_it_be_new_mechanism

**No by the boundary as the human wrote it; borderline by the M53 wording. It needs an explicit ruling.**

Against the enumerated boundary (no new capability, knob, store or doctype; no frozen schema, pinned JSON key or contract-version move), it adds none:

- It is a guard condition under existing codes, with the existing single consent, inside the existing shared probe.
- No store is written, no flag added, no key or version moved.

Supporting precedent, both in `DECISIONS.md`:

- 2026-09-21, M53's line: "registry rows and guard conditions only".
- 2026-09-22: a second leg on `probe_leftover`, refusing under each door's shipped code, was admitted inside M53 on the human's call.
- 2026-10-02: the human's revised line says "Guards inside existing CLI doors are fine regardless". That was written for M55, a milestone, not for this pass.

Where it is borderline, stated plainly:

- The 2026-09-22 leg was recorded as "No new code and no second probe" because it reused `adjudicated_breach`.
- The index leg here is new probe code: a new read of git's admin directory, located without a checkout. The planner is right about that.
- The live-sibling half is cleaner: `git rev-parse HEAD` in the checkout compared with a pin or a ref-containment test, with no new kind of read.

The same leg is needed by any arm that names the staged index honestly. The narrow arm avoids it only by declaring the index loss a bound. So the question for the human is whether the read is admitted at all, not which arm carries it. If it is ruled out, the consistent outcome under M53's rule ("a fix that needs new mechanism is a halt to the human") is a halt on this cell, not a narration that names half the loss.

## verdict_for_the_human

You are choosing whether `milestone provision` and `milestone discard` may destroy a sub-task's still-restorable staged code and commits at exit 0 with a stderr line that names at most a sha, or must refuse under their existing codes with `--force` as consent, at the price of one new read of git's admin directory and an exit-code flip in that cell. Driven on rc.24, the record alone restores the staged work through to a landed finalize, and the same doors silently drop a commit made in a live worktree, so the narrow arm ships a known exit-0 loss into the re-review and neither arm as worded closes the class.

