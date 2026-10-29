# Brief — the byte table, task B6: the `doc` family (2026-10-18)

Worktree `~/projects/gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`, base
`56839f95` (= `origin/fix/rc24-route-catalog`). Work only in that worktree. Every commit behind the full
`dev/gate` run there (in the background, read unpiped, never `| tail`, never an exit code through a pipe), each
by `git commit -F <file>`, with the trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

At the end `git push origin fix/rc24-route-catalog` (by name; never main, never force, never a tag). Never
`git stash`, never `rm -rf` on a variable path (`mktemp -d`, `find <dir> -mindepth 1 -delete`). Public repo: no
host path, no person's name, no session or agent id in a file. Do not edit `.claude/agents/`,
`.claude/settings.json` or `CLAUDE.md`. Kill processes only by pid.

## Read first, yourself

- `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md` — the governing plan; §8 names
  B6 (`doc_write`: the eight `doc` leaves, in two parts) and the pattern every family follows.
- Beside it: `byte-table-acceptance-rows.md` (§6, the silences), `byte-table-readings.md`,
  `byte-table-findings.md`, `byte-table-pilot.md`, `byte-table-install-family.md` (B5's record; §3 its
  questions; and its list *What B6 to B9 need that the plan does not name*, mirrored at the top of
  `DECISIONS.md`).
- `crates/cli/tests/support/byte_cell.rs` (the frame), `crates/cli/tests/byte_table/` (the suite;
  `owed.rs` lists B6's leaves: doc create, add-item, remove-item, retitle-item, rename, set-field, set-slot,
  author).
- B4's and B5's commits, as the pattern to follow in form and message style: `f2c2f3af`, `eba666f0`,
  `4c473b41` (B4) and `228f2eda`, `42a0a9e0`, `aff3b1c3`, `56839f95` (B5). Messages end with
  "(the byte table, task B6, the first/second/… of its commits)".
- `implementation/dev-workflow.md` → Gate; `implementation/public-hygiene.md`; `implementation/pinning.md`.

## Commit 1 — the human's rulings, recorded

In the form `228f2eda` used (a `DECISIONS.md` entry — the log is **newest first**, so at the top — and the
records updated where they state the question as open). Seven rulings, all the human's:

**The six questions the install family met (ruled 2026-10-15, each option A):**
1. **Entry 10** — `setup` refuses without `--force` when the bytes at a path are not jigc's own; the doc's
   sentence becomes "while its bytes are not jigc's own"; its 3 cells are one finding.
2. **Entry 1** — the product is unchanged; `design/assistant-adapter.md` → *Generated, minimal, regenerated*
   says that `setup` also lays out the workbench's cache directories; the 20 cells it marks are marked off, 17
   of them hold.
3. **Entry 12** — `setup` refuses a link at the root `.gitignore` on any HEAD; the doc lists it beside
   `.jigc/.gitignore` as never followed; its 1 cell is a finding.
4. **Entry 9** — `unmanage` never commits; the two sentences of `write-commands.md` that say otherwise are
   corrected; a doc-only inconsistency. The human: "stick with what git does".
5. **Entry 11** — `setup`'s install commit runs the user's hooks; jigc's own hook is kept from triggering
   itself; a rejection is a late failure, rolled back as the earlier ruling 4 has it; one finding. The human:
   "we should not work around the users existing setup". (The record at `byte-table-acceptance-rows.md`
   entry 11 now says the opposite — "runs no hook … as it does" — and is corrected.)
6. **Entry 3** — the footprint record is named in the doc and moves out of `.jigc/state/`, since it is not
   rebuildable; a refusal leaves none; the table checks its path; it is part of the setup-rollback finding; the
   fixer settles how the old path is read.

**And one ruled today (2026-10-18), option A:** the plant the pilot and B5 left unbuilt — **a link or a
directory in the place of a file that stands** — is built by B6, in its own commit before the doc rows, and the
needs that wait for it in the earlier families (the task family's fourteen, the install family's eight) are
filled by B6 too. Why: every later family needs it, and the doc family is the first where it carries weight (a
doc never lands on a file that stands at its destination).

What a ruling changes in the product or a doc outside the table is **a finding of the run's ledger, not a change
in this lane** — unless `228f2eda` shows that its rulings' doc corrections were made in the lane; follow what it
did, and say which.

The cells whose classification a ruling moves: re-read them under the ruling, and say in the commit what moved
(228f2eda's "no classification moves on either binary" is the form).

## Commit 2 (and as many as it needs) — the plant in the place of a file that stands

Red first. The frame (`byte_cell.rs`) gets the kind; the waiting needs of the task and install families get their
cells (where a need waits instead "for a file the never-adopted repository does not have", build the state it
needs, or say why it still cannot be built). Its fences extended as the frame's other kinds are fenced.

## Then the family, on the plan's pattern

Rows from the docs in a commit of their own before any cell of them runs, then the cells. "In two parts", as §8
says — choose the split by the leaves' shared docs and say it. Cells per commit: the plan says about ten; B5
bent that to one commit for a whole kind because a gate cost 25–30 minutes — choose by the gate you measure and
record the choice and why in the commit, as B5 did. Plants red first. A cell names its refusal, shows every class
it is red in, and carries a mark for the question it turns on, as the frame does.

Then the numbers record, a docs commit, in the form of `byte-table-install-family.md` (+ its `.tsv`): cells in
the gate and on demand, red under which keys, red on the previous release (`jigc 1.0.0-rc.24`, installed as the
release binary the earlier families used — find how they reached it), a cell's seconds inside and outside a gate,
what the family adds to a gate, and the projection updated. `owed.rs` loses B6's leaves.

## Where the docs are silent

As B4 and B5 did: a row line only over a quoted sentence; a silence becomes an entry of
`byte-table-acceptance-rows.md` §6 with its *Until ruled* default; the family is built through it, and the
entry is returned. Do not rule it yourself and do not stop for it — unless a cell cannot be built at all without
the answer; then stop and return.

## Return

Compact: each commit's sha and its gate's `passed=` line; the pushed remote head; the family's numbers in five
lines; then **one block per new silence**, each with the doc sentences that disagree or are missing, what a
robust answer would be and the reading the table used until ruled. Anything left open, with why. Write the full
detail into the numbers record, not into the return.
