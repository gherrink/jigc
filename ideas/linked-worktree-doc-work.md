# linked-worktree doc work — a linked worktree as a real home for a branch's managed docs

**Status: parked 2026-10-04, for 1.x.** Parked by the human's ruling in the rc.24 fix pass ([DECISIONS.md](../DECISIONS.md) → *2026-10-04 — rc.24 fix pass, fork on doc work from a user-made linked worktree*), which took the guard now and this direction later. Indexed from [VISION.md](../VISION.md) → Open questions; the trigger row is [decisions-pending.md](../implementation/decisions-pending.md) → *M57 — the 1.x fix pass*.

> **Notation is illustrative.** Nothing below is a design: no finding code, knob, file layout or command shape named here is settled, and the open questions are listed as open.

## The gap

A task driven from a user-made linked worktree — a directory someone made with `git worktree add`, on its own branch — **commits where it stands and reads somewhere else**. The commit, the git index and the promote land in the linked worktree; the doc store, the task area and every cache resolve against the main checkout. So the reconciler compares the main checkout's copy of a doc while the promote overwrites the linked worktree's copy, and a hand edit made in the linked worktree is destroyed at exit 0, in no ref and no blob ([the rc.24 partial re-review](../completions/artifacts/M55/per-axis-review-rc24/README.md) → *Leads the re-drive raised and did not grade*; [R3-F7.md](../completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R3-F7.md) §7, class 5).

That split is by design, not by accident. It is the rule jigc's own fan-out lives by, applied to a worktree jigc did not cut.

## The direction

**The task door reads and writes the checkout it commits in.** A linked worktree becomes a real home for doc work on its branch: a task started there copies in that branch's docs, reconciles against that checkout's files, promotes into it and commits on its branch, and the branch's docs reach the trunk the way its code does.

## What it reverses

- **[storage.md](../design/storage.md) → *CLI and git*, two sentences:** *"only code (never a managed doc) ever rides a worktree"* and *"The committed doc-store has one canonical home (the main checkout)"*. Both were written for the fan-out and both are what the guard below enforces today.
- **The read half of the M31 worktree binding.** Code, index and `HEAD` resolve against the worktree; the doc store and `.jigc/` resolve against the main checkout (same section). The write half — code rides the worktree — stays; the read half is what moves.
- **It extends M53 census row C2-09** ([cwd-census.md](../completions/artifacts/M53/cwd-census.md); [DECISIONS.md](../DECISIONS.md) → *2026-09-23 — The cwd census, the verb class*) from *commits where you stand* to *reads where you commit*. That ruling settled the commit site and said so on the surface; it left the read site at the main checkout.

## Open questions

None of these is settled, and the direction is not buildable until each is.

1. **The derived caches.** File-state and the edge index are one cache per repository, keyed by path — the file-state record a bare path-to-hash map with no stamp, the edge index stamped with the one `HEAD` it was built against ([storage.md](../design/storage.md) → *Derived caches*, *Concurrent writers*). Two checkouts hold two different files at one path. Per-checkout caches, or one cache keyed by branch as well as path?
2. **The project config layer.** It is read from the main checkout, while a branch carries its own committed copy of `.jigc/config/`. Which copy governs a task that commits on the branch — and what does a config change made on the branch mean before it merges?
3. **The pre-commit hook's nested `validate`.** From a linked worktree its sweep asks about the main checkout (`(R1, F5)` in [the rc.24 partial re-review](../completions/artifacts/M55/per-axis-review-rc24/README.md)). Which checkout does the hook ask about once docs live in both?
4. **Two branches' docs meeting at `git merge`.** The rule that git never text-merges a managed doc, because a text merge corrupts anchors and section structure, is what the one canonical home buys ([storage.md](../design/storage.md) → *CLI and git*). With docs on branches, a merge brings two versions of one doc together. What reconciles them without text-merging anchors?
5. **The task roster and the base-pin rule.** Tasks live in one roster under the main checkout's `.jigc/`, and each is pinned to a base. Is the roster per checkout, and is the pin compared against the checkout the task commits in?
6. **The fan-out.** jigc's own sub-task worktrees isolate code by worktree and docs by directory, recombined by the by-task-id join. Do they keep the directory join, or does a doc home per worktree change what the join is over?
7. **Other layouts where `.git` is a file.** Submodules and `--separate-git-dir` repositories resolve the common directory differently from a linked worktree; [monorepo-submodule-support.md](monorepo-submodule-support.md) parks the submodule case. Does this direction cover them, or state them out of scope?

## What ships meanwhile

The guard the same ruling took, landing with the rc.24 fix pass: from a user-made linked worktree, a write of a doc that promotes to a committed home **refuses** and routes to the main checkout. Code-only tasks keep working there, and jigc's own fan-out sub-tasks are exempt. It has two stated prices:

- a task that changes both code and a managed doc cannot be done from a linked worktree in 1.0.0;
- code staged in a linked worktree that breaks a committed doc's anchor must be landed from the main checkout.

## Why it matters

Coding agents commonly work in worktrees — one per task or per agent is an ordinary way to run several at once ([conflict-free-parallel-writing.md](conflict-free-parallel-writing.md)). The guard makes that safe and tells the truth about it; it does not make it work. An adopter whose agents live in worktrees can do code there and nothing else.

## Trigger

**M57's planning.** The 1.x fix pass is where post-1.0 work is next planned. This is a design change that wants its own design pass and its own milestone, not a fix-pass row; M57's Settle decides when.
