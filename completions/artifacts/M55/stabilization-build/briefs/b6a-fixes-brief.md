# Brief — fixes after the review of B6a (2026-10-18)

The review of `56839f95..f4c12fa3` on `fix/rc24-route-catalog` (the human's seven rulings `b7ef80e6`, the plant
`Plant::InPlaceOf` and 22 cells `f4c12fa3`) is at
`~/.claude/projects/<project>/stabilization-briefs/b6a-review.md`.
Read it whole, and the original brief `b6-doc-family-brief.md` beside it for the rules this work is under.
Four fixes, **one fixer each, one commit each, in this order**; you are told which one is yours. Do only yours.

## Rules for every fix

Worktree `~/projects/gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`. Start from its
current HEAD (an earlier fixer may have committed; `git log -3` first, and the tree must be clean — if not, stop
and return). Red first where a test can show it. Then the full `dev/gate` in that worktree (in the background,
read unpiped, never `| tail`, never an exit code through a pipe), `git commit -F <file>` with the trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Message in the form of the branch's byte-table commits, ending "(the byte table, task B6, after its review: …)".
Then `git push origin fix/rc24-route-catalog` by name (never main, never force, never a tag). Never `git stash`,
never `rm -rf` on a variable path. Public repo: no host path, person's name, session or agent id in a file. Do not
edit `.claude/agents/`, `.claude/settings.json` or `CLAUDE.md`. Kill processes only by pid. Records you touch
(`DECISIONS.md` is newest first; the `reshape/byte-table-*.md` records) state what your fix changed; the numbers
of the family are reconciled by fix 4, so a count you change, you note in your return.

## Fix 1 — F1: entry 13 is no silence

`design/surface-contract.md:167` and `design/assistant-adapter.md:64` exempt only an *untracked* file on an unborn
HEAD; a staged one is tracked and refused. List the cell `setup-head-unborn-an-edit-to-the-root-ignore` as refused
with `setup.dirty-install-path`, drop its silence mark, retire the key
`r1-setup-refuses-a-staged-root-ignore-on-an-unborn-head-where-an-untracked-one-rides` and its `EXPECTED_RED`
row; rows-record §6 entry 13 rewritten as answered by those two sentences (not deleted — the record keeps what was
asked), readings §12 and findings §7 to match. `crates/cli/guides/QUICKSTART.md:148` ("Every file is untracked
there") is recorded as a doc-only finding, not edited.

## Fix 2 — F4 and F5: the in-place plant means what it says

F4: `InPlaceOf(File)` at a tracked path keeps the index entry, so git sees an edit while the frame counts kind U
(untracked). Make the plant truly what its kind says — remove the index entry (`git rm --cached` semantics)
before planting, so a U need is answered by an untracked file — unless the review's other reading is the plan's
(read the plan's §3 kinds); say which and why. Re-read every cell that uses it under the corrected meaning.
F5: one assertion per exclusion (`Foreign`, `FileForADir`) that the frame refuses them; the fault cells match a
message distinctive to this plant ("a plant goes in the place of …"), not one `Plant::Edit` also prints.

## Fix 3 — F2: a plant for a file that was deleted, and the discard finding's whole shape (the human's ruling, A)

The human ruled (2026-10-18, A): the frame gains **a plant for a file that was simply deleted** (removed from the
working tree, outside jigc, nothing in its place; say whether the index entry stays — the plain `rm` a user does —
and keep that one meaning), red first, fenced as the frame's other kinds are, built so the doc, milestone and mover
families (B6–B9) use it as it stands. Then the cell the review reproduced: in `fresh`, milestone created, task
added, `docs/milestone-records/<record>.md` deleted, `task discard <task>` — today it exits 0 and settles nothing
while HEAD's record still names the sub-task. It joins the finding
`r1-task-discard-settles-no-record-and-discards-a-sub-task-whose-record-is-not-on-disk` (whose evidence gains the
mechanism the review names: `milestone.rs:2761`, `:2776-2793`, `:2668`, one caller `:2665`). Any other cell of the
task and install families where a deleted file is a need the rows name: give it its cell, or list why not. Record
the ruling in `DECISIONS.md` (top), in the form `b7ef80e6` used for the in-place plant's ruling.

## Fix 4 — F3, F6, F7, F8, F9, and the numbers reconciled

F3: `task-finalize-a-file-in-the-place-of-the-workbench` never reaches `T2` (finalize refuses `finalize.no-task`)
— make it a scheduled row with that reason, as `S2 × K` and `S9 × K` are, or state plainly that it does not reach
the line; prefer the scheduled row. F6: the 17 → 13 explanation names entry 11's hook cell too; "four cells'
listed outcome" is five; "the five corrections" lists six. F7: ruling 1 recorded in the human's words ("a path"),
with the scope note separate if one is needed. F8: `DECISIONS.md` says the doc halves follow `228f2eda` only for
the doc-only entries 1 and 9, and says plainly that 3, 10, 11 and 12 fold into their product keys because the
rulings call each one finding. F9: `byte-table-install-family.md` §3 gets a dated pointer to the rulings. Also: fix 2 left one observation open — with the corrected in-place plant at the stamp's place, `task finalize`
prints both "deleted .jigc/version" and a `setup.version-stamp` advisory saying it "left it untouched and out of this
commit". Every entry a fixer leaves open gets a row: drive it on this build (and on the previous release), and record
it in the findings record — as a finding under its own key if the two printed lines say contradictory things about
one path (the surface contract's laws decide), or as an answered question with the sentence that answers it. Then the
family's numbers in every record that states them — after fixes 1 to 3 — red in the gate under which keys, cells,
on demand; and what is unverified on the previous release said as such.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, what changed in counts, anything left open
with why.
