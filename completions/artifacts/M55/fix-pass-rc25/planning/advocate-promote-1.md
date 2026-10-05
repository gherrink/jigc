# Robust-case advocacy for fork promote-1

## robust_case

POSITION. The robust option is the narrow option plus one thing: the copy-in records the hash of the bytes it copied when the key is absent. It is a superset, not an alternative (the pin compare stays as the backstop), so the question is only whether that extra record is worth a new writer.

1. The plan's own contract needs it. <scratch>/fix-rc25/plan-promote.md §1 says a committing door "never replaces bytes there that the base pin's commit does not hold — a hand edit, an untracked or foreign occupant". Its assertable form allows three cases: (a) absent, (b) equal to the pin blob, (c) "the content a recorded baseline says the task started from". With no baseline and no pin blob, nothing produces (c) except a record made at the copy-in. The narrow option states the contract and then exempts the member it names ("no blob at the pin ... → today's behaviour").

2. That member is an exit-0 loss today, in any checkout, not only a clone. Driven on the installed jigc 1.0.0-rc.24 (source identical to the tag), rig `fresh` with a real file-state record:
- an untracked conformant ADR at `docs/decisions/draft-queue.md`;
- `jigc doc set-slot ... --task` copies it in and records no key;
- a hand line is added to the file;
- `task validate` exits 0, `task finalize` exits 0 with "promoted docs/decisions/draft-queue.md";
- the hand line is then in the worktree 0 times, HEAD 0, any git blob 0, files under .jigc 0.
This is F7's mechanism with one input changed (tracked to untracked). It needs no race and no timing window.

3. The robust path is provable now on the existing classifier. Same rig, with a baseline recorded right after the copy-in (stand-in: the shipped baselining door `jigc ingest`, which hashes the same on-disk bytes):
- untracked case: `task finalize` exits 3 with the existing `reconciliation.conflict-block`; hand line still on disk;
- clone, hand edit after the copy-in: exit 3, hand line survives;
- clone, hand edit before the copy-in: exit 0, both sides at HEAD.
No classifier arm changed. The DRIFTED + TOUCHED and IN_SYNC arms in crates/engine/src/file_state.rs (`reconcile_committed`) already do the work once the key exists.

4. It makes three printed or stated sentences true instead of leaving them false.
- design/reconciliation.md → Detection timing: a write through the CLI "re-probe[s] before staging", and → Baseline adoption: on first encounter the "hash is computed and recorded". The code says the opposite at crates/engine/src/state.rs, `create` step 4: "drift is finalize-preflight's concern, not copy-in's".
- The route of `file-state.un-baselined`: "no action needed — the doc is baselined on its next author or finalize". In the run in point 2, `jigc validate` printed exactly this for the doc that was then overwritten. That is a law-1 lie (design/surface-contract.md → Law 1, "behaviour claims match") on the loss path itself.

5. The narrow option blocks a non-conflict and shows it a route whose words are false there. Driven with a baseline present (arm A, which the narrow option extends to every first task in every clone): the block reads "an external edit and this task's staged writes both changed it ... or revert the external edit on disk to keep them".
- Hand edit before the first write: `git checkout -- <doc>` removes the hand line from the worktree, then finalize exits 0 and the hand line is at HEAD. The edit the user just reverted is committed anyway.
- Hand edit after the first write: the same message and the same route, and the hand line is gone everywhere.
One message, opposite outcomes, and the surface cannot say which applies. design/reconciliation.md → The principle says a clean external edit is "honored — not merely tolerated"; an edit made before the task touched the doc is that case.

6. Stability across 1.0.0. The planner defers the copy-in baseline to 1.x. The narrow option therefore pins "block both orders" in a test, in storage.md and against MIGRATING item 8 at 1.0.0, for a clone state the planner already expects to redesign. Whether 1.x would then let the merge order land again is a design choice I cannot predict; it is a risk, not a certainty.

CLASS, NOT INSTANCE. The verifier's stated contract (completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R3-F7.md §7) is that no-silent-discard must hold "independently of whether the gitignored cache happens to hold a key". The narrow option makes it depend instead on whether git happens to hold a blob at the pin.

Spike scripts: <scratch>/advocate-promote1/ (lib.sh, u1.sh, c.sh).

## what_the_narrow_option_leaves

1. An exit-0 loss through `task finalize`, driven on rc.24 and untouched by the narrow condition.
- State: a managed doc at its home with no blob at the task's base pin (untracked, or `git add`-ed but uncommitted) and no file-state key. No clone is needed; a checkout with a populated record has no key for a new file.
- Door: any of the six `doc` write verbs through `read_or_copy_in` (crates/cli/src/doc.rs), which copies in on `committed.is_file()`, not on git tracking; then `task finalize`.
- Lost: every hand edit made to that file after the copy-in. The file is untracked, so no git object holds it; jigc keeps no displaced copy.
- Surfaces on the way: `file-state.un-baselined` ("baselined on its next author or finalize"), then `file-state.baseline-adopt` ("no action needed") at the path being overwritten.
- `milestone finalize` shares the arm; I did not drive that door for this member.

2. Where a re-review would find it. The member sits in the `None =>` arm the fix edits, on the else-branch the fix writes. The exit rule (implementation/decisions-pending.md, "The exit rule") sends a tier-1 inside a fix pass's own new code to another fix pass and partial re-review. The fix for it can only be a record of what was copied, because there is no pin blob to compare — that is the robust option, built one cycle later.

3. The tier is the open point. The planner files this member under "ruling 4" (external-writer class, tier 2). I could not read the ruling's text. The K-1 record's class sentence is "an untracked file at a home is treated as committed, and no door is atomic against a non-jigc writer", so the planner's reading is plausible. Against it: this member needs no race, and it meets the tier-1 predicate the same way F7 does.

4. A block on a non-conflict, in every clone's first task. A hand edit made before the task's first write now exits 3 at both committing doors although the staged copy already carries it. The route's literal action ("revert the external edit") lands the edit anyway (driven, see robust case point 5).

5. Two false surfaces stay false: Detection timing's write row and the `un-baselined` route.

6. A new comparison with its own false-fire class. The UNKNOWN arm starts comparing worktree bytes with a git blob. The planner already found autocrlf; eol attributes and smudge/clean filters are the same family. `git cat-file --filters` should cover them; I did not drive that. The failure direction is a false block, never a loss.

One correction to the planner's account, against my side: "arm B stays" under the robust option is not true of the pinned test as written. crates/cli/tests/reconciliation_baseline_contrast.rs drops the key after the `set_slot` copy-in, so the backstop blocks it under both options. Arm B survives only if the test is restaged to lose the baseline before the first touch (the clone shape).

## what_the_robust_option_really_costs

It costs everything the narrow option costs (the pin compare, the `--filters` read, the migration carve-out), plus the following.

BUILD
- Two copy-in sites, not one: `read_or_copy_in` in crates/cli/src/doc.rs (six verbs) and `create` step 4 in crates/engine/src/state.rs (`doc create` / `doc author` under a plain entry). The doc-comment calling the first "the only production caller of copy_in" is wrong.
- At each: if the key is absent and the bytes pass the existing gate (`committed_path_recordable`), record the hash of the bytes just read and save through the existing `FileStateRecord::save` (base-relative merge plus save lock).
- My estimate: one more commit, tens of lines, eight to ten tests. I built none of it.

CELLS THE NEW WRITER BRINGS (each needs a test)
1. Hash the raw bytes copied, never the canonicalized staged copy; otherwise every BOM or trailing-newline doc reads as drifted.
2. The key must be the one the sweep reads (the resolved placement home, the location path); a mismatch is a silent no-op.
3. Save-lock timeout at a write door (10 s budget, worst measured wait 1.37 s): record before staging and fail closed, or skip and rely on the backstop. Skipping leaves the untracked member open in that sub-cell.
4. Fan-out: N sub-agents' first writes now contend on one `state/file-state.json`. I found no sub-agent-time writer of that file today. The merge and lock were built for concurrent writers, but this is a new population.
5. A non-conformant doc must not be baselined.
6. Surfacing: finalize no longer prints `file-state.baseline-adopt` for that doc (seen in the stand-in run). "Every absorb surfaces" then wants the adoption stated at the write door. The write-ack already has `findings[]`, so no key moves, but ten suites name `baseline-adopt` and I did not check how many pin it at finalize.
7. Store scope: after a copy-in, a later hand edit in a clone reads as drift at `jigc validate` and the pre-commit hook, as in any checkout with a baseline, instead of the `un-baselined` advisory.
8. The record is per path, not per task. Two tasks on one doc, where the first copy-in left no record (an old binary, or `jigc unmanage` mid-task) and a hand edit falls between the two copy-ins: the second task records the edited bytes, the first task reads IN_SYNC and promotes over the edit at exit 0. The narrow option blocks that cell. It is the one place the robust option is worse on bytes.
9. Tasks copied in by an older binary get only the backstop, so they behave exactly as under the narrow option.

DOCS
- design/reconciliation.md: "Persistence of the shifted baseline" (durable only at a landed finalize) and Hash re-baselining gain a write-door site.
- design/storage.md: Concurrent writers gains the writer; "What none of this buys" narrows to a baseline lost before the first touch.

WHAT IT DOES NOT BUY
- The linked-worktree sibling (promote-2).
- The verifier's stronger design: a task-area record plus a compare at the promote. That one is a working-area format change and belongs to 1.x.
- A merge order that lands when a baseline is present (arm A is unchanged).
- Any key lost after the copy-in; those fall to the backstop and block both orders.

## would_it_be_new_mechanism

Borderline: no by the letter of the boundary as given to me, yes in the sense the fix-pass rule was written to catch.

By the letter: no new store, format, knob, doctype, finding code, schema hash, pinned JSON key or contract version. It reuses `FileStateRecord::{load, record, save}`, `committed_path_recordable` and the existing classifier arms, and it is the behaviour two design sentences already state (reconciliation.md → Detection timing and → Baseline adoption).

By M53's boundary text (implementation/decisions-pending.md, "The boundary"): a fix-pass change is "a row in a registry that already exists, or a condition on a guard that already exists". A new persistence site on eight write doors is neither. It also revises a pinned rule (the M17 "persisted only by a landed finalize" sentence, though that rule's ground is read verbs staying pure readers, and a write verb is not one). The same text says such a fix "is a halt, not a build decision: it goes to the human with the mechanism named and its sibling cells enumerated, because a new mechanism is where the next tier-1 row comes from". This fork is that halt; the nine cells are listed under costs, and cell 8 is a real one.

The narrow option fits the boundary cleanly: one condition on an existing guard.

## verdict_for_the_human

You are choosing whether a hand edit to a doc with no blob at the base pin, lost at exit 0 in the arm this fix edits (driven on rc.24), counts as tier 1 or falls under your tier-2 external-writer ruling, whose text I could not read. If tier 1, robust-now is the minimal-correct cut, because only a record made at the copy-in can close it and the narrow fix would meet it again at the re-review; if tier 2, the narrow cut is correct for this pass, at the price of a clone's first task blocking a hand edit its staged copy already carries, and the copy-in baseline, a new writer on eight write doors with nine cells, belongs to 1.x.

