# Brief — fixes after the review of B7 part A (2026-10-18)

The review of `a81af2bb..cabe3aff` on `fix/rc24-route-catalog` is at
`~/.claude/projects/<project>/stabilization-briefs/b7a-review.md`. Read it
whole, and `b7-milestone-family-brief.md` beside it (its *Rules* hold for you, and its part A says what was asked).
Five fixes, **one fixer each, one commit each, in this order**; you are told which one is yours. Do only yours.

## Rules for every fix

`b7-milestone-family-brief.md` → *Rules (both parts)*: worktree, start from HEAD (`git log -3`; a dirty tree stops
you), red first, the full `dev/gate` in the background read unpiped, `git commit -F` with its trailers, push by name,
no stash, no `rm -rf` on a variable path, no host path / name / id in a file, no edits to `.claude/` or `CLAUDE.md`,
the previous release is the rc.24 rebuild `accf3996…ab5d`. Messages end "(the byte table, task B7, after part A's
review: the first/second/… of its fixes)". `DECISIONS.md` newest first. Every frame change: re-read the cells it
touches and say whether any classification moved, on this build and on the previous release. A count you change, you
note in your return; fix 5 reconciles the records. Anything you leave open is named in your return with why.

## Fix 1 — F2: three sets of the comparison that ignore the acting unit

`staged_docs` (`byte_cell.rs:1690-1694`: merged docs only, whenever any milestone has them — the loss loop then never
sees the acting task's staged docs); `CopyOfAStagedDoc` (`T1`, `M2`, `:979`, `:1014`: a sibling's staged doc promoted
is accepted); `Holding::TheWorktrees` (`:1433`: any non-main checkout, a user's own included). Key the staged set on
the acting unit; run the loss loop over the union of task docs and merged docs; hold worktrees to
`.jigc/worktrees/<sub-task>` (read the docs for the worktree path — never hard-code what a knob resolves). A fence case
for each exclusion, red first (the review's example: a joined milestone in the repo, `task finalize` drops its own doc
→ must be *lost*).

## Fix 2 — F5, F6, F7: how a line names its unit

F5: `A5` (`task.rs:224`) and `M5` (`milestone.rs:114`) reach any milestone's record because a place matches whole
components only (`byte_cell.rs:824-830`); give the vocabulary the narrowest grounded way to say "this unit's record"
(the review: a `<unit>` component with a fixed suffix, and a word for "the record whose item names this unit"; a
missing record reaches nothing). F6: "the single task that stands" counts a task only where its base pin stands
(`state.rs:1725-1737`, `storage.md:294`). F7: the sub-task list is read from where the docs say it is (the committed
record, resolved through `docs-root` — never a hard-coded path); if the working tree is what the product reads, say
which sentence says so. Fence cases red first.

## Fix 3 — F4, F9, F8: the `squash: false` spelling checks its chain

F4: `M3` and `M4` are both `Allows::Commit(Holding::TheWorktrees)`, so a product ignoring the knob stays green. Give
`M4` a holding that checks what its sentence says — one commit per code-carrying sub-task, in task-id order, each
holding only its own worktree's code, then the aggregate — red first with a fence case (a single squashed commit under
`squash: false` → not the listed outcome). F9: needs keyed by spelling as well as line, so a spelling's cell never
answers the plain spelling's need (the doc-only link-to-nothing cell and `T1`); the cells owed by that become owed or
built. F8: `T9`'s owner-artifacts (`finalize.md:329`) — unreachable with the shipped packs: a scheduled row with that
reason or a lead in the findings record.

## Fix 4 — F1 and F3: entries 16 and 17 are answered

F1: the docs say finalize invalidates the stamp and the next read rebuilds (`finalize.md:210,214`, `storage.md:328,
354,356`); every door removes the file (`index.rs:176-188` via `post_commit` `task.rs:7984`, and `rename.rs:990`).
Give the edge-index file alone a "written or taken away" word grounded in those sentences (not `.jigc/state`, which has
no rebuild path, `storage.md:314`); entry 16 rewritten as answered (keep the question asked), the key
`r1-milestone-finalize-under-squash-false-takes-the-edge-index-away-where-its-line-rebuilds-it` retired and its cells'
marks dropped; the record's false premise ("`task finalize` and `squash: true` write the file again") corrected; the
existing `T5`/`M10` cells checked with the index built first (their leads never built it). F3: entry 17 answered by
`command-output-contract.md:542-546`; `finalize.md:242` ("IndexHonoring over the now-clean index") is a doc-only
`inconsistency` key in findings §1. The review's lead (a change staged before `milestone create` meets the carryover
gate) goes into the findings record's leads for part B.

## Fix 5 — F10, F11, F12, and the numbers reconciled

F10: the two reachable fault paths never seen red (`byte_table.rs:598-602`, `byte_cell.rs:2122-2128`) — make each
red once, on its own. F11: ruling 1's "names what stands there" — record which cells must hold the fixed refusal's
wording once the product is fixed, so the fixer of that finding has them (a lead in the findings record; cells keep
today's codes). F12: `cargo fmt --check` never reaches the family files because their modules are declared inside a
macro (`byte_table.rs:153-162`) — make the gate's fmt step reach every file of the suite (declare the modules where
rustfmt sees them, or the narrowest equivalent), format them, and a fence that a module declared inside a macro is
refused or reached; this predates B7 — record it as a finding of the gate itself in `DECISIONS.md`. Then every count
in every record, re-derived from the suite after fixes 1–4.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, what moved (on both binaries), counts changed,
anything left open with why.
