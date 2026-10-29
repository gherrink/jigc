# Brief — the byte table, task B8: the movers family (2026-10-19)

One builder does all of B8: the rulings commit, the rows, the cells, the numbers. A read-only review follows, then
fixers — **one per frame hole the review finds, and one last fixer for every record-only note and the numbers** (the
human's ruling of 2026-10-19). Fewer review findings is the aim: hence the self-check below, which you run and report
before you return.

## Rules

Worktree `~/projects/gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`. Start from its HEAD
(`43b555fe` — `git log -3`; a dirty tree stops you). Every commit behind the full `dev/gate` run there (in the
background, read unpiped, never `| tail`, never an exit code through a pipe), each by `git commit -F <file>` with the
trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Messages in the form of the branch's byte-table commits, ending "(the byte table, task B8, the first/second/… of its
commits)". At the end `git push origin fix/rc24-route-catalog` by name (never main, never force, never a tag). Never
`git stash`, never `rm -rf` on a variable path (`mktemp -d`, `find <dir> -mindepth 1 -delete`). Public repo: no host
path, person's name, session or agent id in a file; the human is "they" / "the human", never he/she. Do not edit
`.claude/`, `CLAUDE.md`. Kill processes only by pid. Red first wherever a test can show it. The previous release is
the rc.24 rebuild from `91834b5e` (sha256 `accf3996…ab5d`), never the `jigc` in `~/.local/bin`. A ruling's product or
doc change is a finding of the run's ledger, not a change in this lane (the form `b7ef80e6` used). If the timing test
`dev_stabilize_probe::a_hold_ends_only_after_its_seconds_is_started_once_and_is_answered_within_a_slice` fails,
re-run the gate once and report it; do not touch it.

## Read first, yourself

- `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md` §8 (B8 = `rename`, `relocate`,
  `migrate-corpus`, `config set`), and the records beside it: `byte-table-acceptance-rows.md` (§6, §7),
  `byte-table-readings.md`, `byte-table-findings.md` (the B8 leads: §14's table — `engine/src/finalize.rs` store door's
  shape arm *"nothing was written through `<destination>`"*, `render.rs`'s dry-run sentences — and §15:
  `rename.rs`'s `index::invalidate` owed `May::Invalidated` on its own doc's sentences, with a cell that builds the
  index first), `byte-table-milestone-family.md` (the numbers record's form).
- `DECISIONS.md` from the top down to `b7ef80e6`'s entry: every frame word now in force and why.
- **All four reviews and what they caught** — the self-check below is built from them:
  `~/.claude/projects/<project>/stabilization-briefs/b6a-review.md`,
  `b6b-review.md`, `b7a-review.md`, `b7b-review.md`.
- `crates/cli/tests/support/byte_cell.rs`, `crates/cli/tests/byte_table.rs`, `crates/cli/tests/byte_table/`
  (`owed.rs`: B8's four leaves). `implementation/dev-workflow.md` → Gate; `implementation/public-hygiene.md`;
  `implementation/pinning.md`. `CLAUDE.md` → *Architectural invariants* (an open question's answer is checked
  against them).

## Commit 1 — the human's three rulings of 2026-10-19

In the form `b7ef80e6` used (`DECISIONS.md` at the top; the records that state each question as open updated, the
question kept; cells whose classification or mark a ruling moves re-read on both binaries and named):

1. **Entry 18 → A.** Before removing a landed sub-task's worktree, the boundary moves its unstaged and untracked files
   that git does not ignore into `.jigc/displaced/<sub-task-id>/` and names them; what git ignores is named and removed,
   as ruled at M46 and item 6. Nothing is refused, nothing is lost. The doc change (`storage.md`'s worktree warrant,
   the non-ignored half of `team-ready-state.md`'s bound, `finalize.md:378`'s last clause, the reach of
   `sub_tasks[].discarded` and `committed.displaced`) is a ledger finding. Why: the exit rule, and the human's "Over
   take-and-name" (an exit-0 loss of a sub-agent's deliverable); it reuses the boundary's existing move-aside.
2. **Entry 20 → A.** `milestone provision` skips sub-tasks the record has settled, and the milestone design says so,
   as the re-seed does. A doc-only finding. Why: a settled sub-task has nothing to work on.
3. **Bound 2 is retired, for tasks and sub-tasks alike.** A `doc` write that would replace a part whose bytes differ
   from what is committed — an uncommitted hand edit the copy-in carried — refuses **at the write**, naming the part
   and showing the hand-edited text so the agent folds it into its prose and writes again; an explicit, recorded way
   to replace it on purpose; a hand edit in another part is absorbed as today. A product fix with a new refusal code,
   its route, and sentences in `write-commands.md` and `reconciliation.md`: a ledger finding — whose fixer first
   confirms the write knows, per part, what was committed and what was copied in. Why: `CLAUDE.md`'s invariant
   *"conflicts blocked and routed to a human, never silently merged"* — the bound accepted a breach of it; refusing at
   finalize (rc.24) is too late. Both bound-2 cells lose the mark and become plain red under a key of their own; the
   declared-bounds list loses bound 2. The human's general rule with it, their words: *"I want in general robust and
   future proofed solutions, nothing that does fail in the end and we need to come back and fix it."*

## Then the family, on the plan's pattern

Rows from the docs in a commit of their own before any cell of them runs; then the cells in parts you choose by the
leaves' shared docs (say the split); then the numbers record in the form of `byte-table-milestone-family.md` + its
`.tsv`, **including a committed whole-table print on both binaries** — every previous-release claim rests on it.
`owed.rs` loses B8's leaves. Use the plants `InPlaceOf` and `Deleted`, and the own-unit places, wherever a row names a
need they answer. Drive the B8 leads above: each becomes a cell, a key, an answered question with its sentence, or
stays a lead with the exact reason no cell can see it. `config set` meets the entry-19 ruling of 2026-10-19 (a config
change is committed on purpose, with its task or on its own; `config set` says so when it writes): read what the
docs say it prints and writes. Cells per commit by the gate you measure, said in the commit; costs labelled measured
or projected.

**Where the docs are silent or say two things:** a line only over a quoted sentence; a question becomes a §6 entry
with its *Until ruled* reading. Before calling anything a silence, search every design doc, `DECISIONS.md` and the
fix-pass record (`completions/artifacts/M55/fix-pass-rc25/`) and quote what you find; say plainly *genuine silence* /
*answered (by which sentence)* / *two docs saying two things*. Each open question carries the **robust answer** — the
end state with no known loss and no known revisit, checked against `CLAUDE.md`'s invariants, even when that means a
product fix or a doc change — never only a choice between today's readings. Do not rule; do not stop unless a cell
cannot be built at all without the answer.

## The self-check — run it on your own work before returning, and report it

Each item is a mistake a past review found. For each, say what you checked and how (file:line, a fence case, a run).

1. **A word or line wider than its sentence.** For every new frame word, line and condition: construct the case where
   it would excuse a real loss or write — a sibling's or another unit's bytes, a user's own file or worktree, two where
   the sentence allows one, a place that already stood, a refusal path — and show it red in a fence case. (Caught
   before: `May::Carried`, the re-seed's `Rebuilt`, the base-pin rule, `W1`/`W4` over any task's area, `A5`/`M5` over
   any record, `<minted>` reading only the working tree, `Allows::Registered` unchecked.)
2. **Unit-blind sets.** Every set the comparison reads is keyed on the acting unit. (Caught: `staged_docs`,
   `CopyOfAStagedDoc`, `TheWorktrees`.)
3. **A "silence" the docs answer or contest** — the search above, done for every entry. (Caught: entries 13, 14, 16,
   17, the pending-config question, entry 18's missing sentences.)
4. **Cells that cannot fail.** A plant that never reaches the command; a holding nothing checks (the inert
   `squash: false`); leads that build a state which hides the case (the index built first); a need scheduled that can
   be built now; a lead called unbuildable that can be built. For each scheduled need and each lead: why, exactly.
5. **A row missing what its door writes.** Drive each leaf once with nothing planted and compare everything it writes
   with the row's lines (caught: `join`'s index rebuild, the re-seed lines of `discard`/`finalize`, the message file).
6. **Counts.** Every count re-derived from the suite and the committed print, never copied; previous-release claims
   only from the committed print; each cost labelled measured or projected (caught: 57 for 59; the "+83 s").
7. **Every frame change re-read** on every cell it touches, on both binaries, with what moved named.
8. **Hygiene.** No host path, name, id; the human is "they"; new family files reached by `cargo fmt`
   (`tooling-tests/fmt_reach.rs`); the quote fence holds every new quote.

## Return

Compact: each commit's sha and its gate's `passed=` line; the pushed remote head; the family's numbers in five lines;
what moved on both binaries; the self-check, one line per item; then one block per new question (the sentences on each
side quoted, the robust answer and why, the reading until ruled). Anything left open, with why. Full detail goes into
the records, not the return.
