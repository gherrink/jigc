# Brief — fixes after the review of B7 part B, the `milestone` family (2026-10-19)

The review of `fff27d7b..ed440461` on `fix/rc24-route-catalog` is at
`~/.claude/projects/<project>/stabilization-briefs/b7b-review.md`. Read it
whole, and `b7-milestone-family-brief.md` beside it (its *Rules (both parts)* hold for you; its part B says what was
asked). Seven fixes, **one fixer each, one commit each, in this order**; you are told which one is yours. Do only
yours. The review's probe scripts are in its scratch folder named at its end — reuse them, do not trust them unrun.

## Rules for every fix

`b7-milestone-family-brief.md` → *Rules (both parts)*: worktree `~/projects/gherrink-jigc-route-catalog`,
start from HEAD (`git log -3`; a dirty tree stops you), red first, the full `dev/gate` in the background read unpiped,
`git commit -F` with its trailers, push by name, no stash, no `rm -rf` on a variable path, no host path / name / id in
a file, no edits to `.claude/` or `CLAUDE.md`, the previous release is the rc.24 rebuild `accf3996…ab5d`. Messages end
"(the byte table, task B7, after part B's review: the first/second/… of its fixes)". `DECISIONS.md` newest first.
Every frame change: re-read every cell it touches and say whether any classification moved, on this build and on the
previous release. A count you change, you note in your return; fix 7 reconciles the records. Before calling anything
a silence or answered, search every design doc and `DECISIONS.md` and quote what you find. Do not rule. Anything you
leave open is named in your return with why.

## Fix 1 — F1: `M3, the config` / `May::Carried` is wider than its sentences

Narrow the word to what its sentences say (the first commit's config) — or, if no narrow reading is grounded, drop it
— so a user's untracked file under `.jigc/config` swept into a landed boundary is not *holds*. Red first with that
fence case (the review drove it on both binaries). Make the task family and the milestone family read the sentence the
same way. The pending-config-delta question is **two docs saying two things** (the review quotes `finalize.md:166`,
`storage.md:308` against the first-commit sentences): a §6 entry of the rows record with both sides quoted, a robust
answer and the *Until ruled* reading, and an `inconsistency` key in findings §1. Correct the record that calls it
answered.

## Fix 2 — F2: the re-seed lines excuse a rewrite of an area that already stands

The re-seed lines (11 instances from 3 sites) and `MP1` are `Rebuilt` with no condition, against "A sub-task whose area
already exists is skipped untouched" / "reuses untouched". Give the frame the narrowest grounded way to say "rebuilt
only where it did not stand", red first (a re-seed rewriting a standing sub-task's staged prose or base pin → not
holds). Add the re-seed lines the `discard` and `finalize` rows lack where their own quoted sentence names them (F10).

## Fix 3 — F3: entry 18 re-quoted on both sides

Rewrite rows §6 entry 18 with every sentence the review found: for narrating `finalize.md:378` (2026-10-06), the item-6
build entry, `storage.md:300`; against, the human's words of 2026-10-04 ("Over take-and-name … an exit-0 loss of a
sub-agent's deliverable") and the human's rulings on items 10/4 and 6. Correct "predates the rule". Then say plainly
whether a ruling of the human already answers it (quote it), or it stays open; revise the robust answer if
`storage.md:300` undercuts "the same disposition". The cell's mark follows that answer.

## Fix 4 — F5: `join`'s row and the index rebuild

`join` at exit 0 rebuilds an absent `.jigc/index/edges.json`; no line covers it. A line over the sentence that allows
it (the `May::Invalidated` family of words, or the narrowest equivalent), red first with a `join` cell whose index is
absent; check the other milestone doors that read the index the same way and say which needed it.

## Fix 5 — F6: the two scheduled `M2 × E` needs are built

Build them in `committed-singletons` (the review drove it: this build lands exit 0 carrying the user's uncommitted
`VISION.md` line into the boundary commit; rc.24 refuses). Classify from the docs, red or a key as they say; no longer
scheduled.

## Fix 6 — F8, F9: `<minted>` and `Allows::Registered` narrowed

F8: `<minted>` reads only the working tree, so a record deleted from disk but still committed counts as nothing stood —
read what git holds too; narrow "one mint from two" now if it can be. F9: `Allows::Registered` checks the new
worktree's `HEAD` against the pinned base and its index. Fence cases red first.

## Fix 7 — F4, F7, F10, F11, and the numbers reconciled

F4: the family record's row/line counts (59 lines, `finalize` 19 where 16 — re-derive, do not copy). F7: the repro
block for the directories key (the review wrote one — run it as printed on both binaries); note the shared writer
`commit_record_only` (`milestone.rs:999-1012`) beside the task family's two keys, so one fix closes each pair.
F10: the leads' reasons — `git_home_claims` (plan §1.1 puts "git fails, the index is unreadable" inside the clause:
build a cell if a plant can make git fail, else a scheduled row with the exact reason); `add-from-spec` with more than
one criterion, the message file after a successful commit, the re-seed of an absent area — build each cell where it
can be built, else say why not. Also fix 2's left-opens, each a record row: the cache lines' width (snapshot, `merged/`, message file); `provision` for a settled sub-task (code skips it, the only sentence `team-ready-state.md` → *discard* narrates the old defect — a finding or a question, say which); a refusal after the re-seed leaving a rebuilt area (build the cell). Fix 3 (`0cd5f44f`) referred to the human as he/his/him in `DECISIONS.md` and four records: make every sentence this task's fixes added say "they/their/them" (or "the human"); leave older entries as they are. **The human's ruling on entry 19 (2026-10-19), to record (form of `b7ef80e6`: `DECISIONS.md` at the top, rows §6 entry 19 answered with the question kept, readings/findings that call it open updated, the two pending-delta cells re-read — the inconsistency key stays, the doc change is a finding of the run's ledger):** A — a finalize never carries a change under `.jigc/config` that nobody staged; it names it among what it left out. The human's words: *"In general we should never add unrelated files into a commit. Also a config change need to be committed. Either when it belongs into a task or individual."* So: no unrelated file ever rides a commit; a config change must reach a commit — with the task it belongs to, or on its own — and `config set` says so when it writes. How a config change is shown to belong to a task is the product fix's to design (record it as that fix's open point, not a ruling). Fix 6's left-opens (`02675f86`): `MP1` still covers a new worktree's files as any bytes — narrow it to the pinned base's tree ("populates from the pinned base's *tree*"), red first with a fence case; `<each-minted>` cannot count a spec's criteria (with the `add-from-spec` lead); carry the rc.24 excused counts `MC1` 60, `MC2` 46, `MC3` 15 into the records. F11: the "+≈83 s" labelled as a difference of two runs at different loads. Then every
count in every record, re-derived from the suite and a fresh whole-table print on both binaries after fixes 1–6.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, what moved (on both binaries), counts changed,
anything left open with why. Fix 1 and fix 3 also: each open question in one block — both sides quoted, the robust
answer, the reading until ruled.
