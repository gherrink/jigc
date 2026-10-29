# Brief — the byte table, task B9: the mint family (2026-10-20)

One builder does all of B9: the rulings commit, the rows, the cells, the numbers. A read-only review follows, then
fixers — one per frame hole the review finds, and one last fixer for every record-only note and the numbers (the
human's ruling of 2026-10-19). Fewer review findings is the aim: hence the self-check below, which you run and report
before you return.

## Rules

Worktree `~/projects/gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`. Start from its HEAD
(`002b2090` — `git log -3`; a dirty tree stops you). Every commit behind the full `dev/gate` run there (in the
background, read unpiped, never `| tail`, never an exit code through a pipe; never two gates at once), each by
`git commit -F <file>` with the trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Messages in the form of the branch's byte-table commits, ending "(the byte table, task B9, the first/second/… of its
commits)". A gate that fails stops the commit: never commit, then gate. At the end `git push origin
fix/rc24-route-catalog` by name (never main, never force, never a tag). Never `git stash`, never `rm -rf` on a variable
path (`mktemp -d`, `find <dir> -mindepth 1 -delete`). Public repo: no host path, person's name, session or agent id in a
file; the human is "they" / "the human", never he/she. Do not edit `.claude/`, `CLAUDE.md`. Kill processes only by pid.
Red first wherever a test can show it. The previous release is the rc.24 rebuild from `91834b5e` (sha256
`accf3996…ab5d`), never the `jigc` in `~/.local/bin`. A ruling's product or doc change is a finding of the run's
ledger, not a change in this lane (the form `b7ef80e6` used). If the timing test
`dev_stabilize_probe::a_hold_ends_only_after_its_seconds_is_started_once_and_is_answered_within_a_slice` fails, re-run
the gate once and report it; do not touch it.

## Read first, yourself

- `implementation/working-principles.md` — **neither the doc nor the code is the oracle**: judge what makes sense for a
  user first, then say which of doc and code (or both) is wrong. A sentence that allows what makes no sense for a user
  is a finding against the doc, never an allowance (`byte-table-plan.md`'s first rule of a line says so since
  `002b2090`).
- `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md` §8 (B9 = `start`, `workflow`, `migrate`,
  `task amend`; it empties the owed list and removes it with its type), and the records beside it:
  `byte-table-acceptance-rows.md` (§6, §7), `byte-table-readings.md`, `byte-table-findings.md` (every lead naming a B9
  door), `byte-table-movers-family.md` + its `.tsv` (the numbers record's latest form).
- `DECISIONS.md` from the top down to `b7ef80e6`'s entry: every frame word now in force and why.
- **All six reviews and what they caught** — the self-check below is built from them, in
  `~/.claude/projects/<project>/stabilization-briefs/`: `b6a-review.md`,
  `b6b-review.md`, `b7a-review.md`, `b7b-review.md`, `b8-review.md` (and `b8-fixes-brief.md` for how each hole was
  closed).
- `crates/cli/tests/support/byte_cell.rs`, `crates/cli/tests/byte_table.rs`, `crates/cli/tests/byte_table/`
  (`owed.rs`: B9's leaves). `implementation/dev-workflow.md` → Gate; `implementation/public-hygiene.md`;
  `implementation/pinning.md`. `CLAUDE.md` → *Architectural invariants* (an open question's answer is checked against
  them).

## Commit 1 — the human's two rulings of 2026-10-20

In the form `b7ef80e6` used (`DECISIONS.md` at the top; the records that state each question as open updated, the
question kept; every cell whose classification or mark a ruling moves re-read on both binaries and named). A frame
change a ruling needs may be its own commit before the records, red first.

1. **Entry 23 → stricter than its option A.** *jigc never moves or overwrites a file to make room for its own.* Any
   file at a mover's destination — committed **or untracked**, managed or not — refuses: `relocate` blocks that doc,
   moves the others, names the path to clear and exits non-zero; a root knob (`config set docs-root`,
   `config set placement-root`) refuses the whole re-point and leaves the knob unchanged, since blocking one doc while
   the knob lands strands it. jigc moves a file aside only from a place it is about to remove (entry 18's landed
   worktree), never from a destination. Where the occupant is a doc, the refusal's route offers adopting it as well as
   clearing it. **And (A):** an unmanaged doc *elsewhere* in the target folder does not block — the output names it
   with the adopt route. Wrong: `design/reconciliation.md` → *Relocation collisions* (the settled foreign-only bound
   of 2026-07-07, M39 T5), `design/storage.md` → *Placement*'s parking of a squatter, `docs-root`'s refusal in git's
   words (owed a jigc code and route), and the product at `relocate` and `config set placement-root`. Why: the
   invariant *"conflicts blocked and routed to a human, never silently merged"*; a parked file is the user's bytes at a
   path they never chose after a command that reported success. For the table: the parking lines (`RL3`/`CS3`, and
   any sibling that excuses a parked occupant) go; the three marked entry-23 cells lose their mark and are plain red
   under their key; every cell with an untracked occupant at a destination is relisted as refusing — red under a key
   that names it (extend the entry-23 key's reach or add one, say which and why).
2. **R1 → A.** *An acting sweep does all it can, then exits non-zero if any act it was asked for did not happen; a
   reporting door keeps exit 0 for what it finds.* `relocate` (and `migrate-corpus`, which already does) are acting
   sweeps; `ingest` and `jigc validate`'s store sweep are reporting doors. Wrong: `design/command-output-contract.md`'s
   `jigc relocate` paragraph (*"This `blocked` is not an exit rule"*) and `design/validation.md`'s *"Its exit is
   therefore **unchanged**"* clause for `relocate` (M52 Increment 8 / T6); the product. The key fix 8 recorded
   (`002b2090`) now carries the ruling; cells whose listed exit it moves are re-read on both binaries and named.

## Then the family, on the plan's pattern

Rows from the docs in a commit of their own before any cell of them runs; then the cells in parts you choose by the
leaves' shared docs (say the split); then the numbers record in the form of `byte-table-movers-family.md` + its `.tsv`,
**including a committed whole-table print on both binaries** — every previous-release claim rests on it. `owed.rs` and
its type go. Use the existing plants and own-unit places wherever a row names a need they answer. Drive every B9 lead in
the findings record: each becomes a cell, a key, an answered question with its sentence, or stays a lead with the
exact reason no cell can see it. Cells per commit by the gate you measure, said in the commit; costs labelled measured
or projected.

**Where the docs are silent or say two things:** a line only over a quoted sentence that makes sense for a user; a
question becomes a §6 entry with its *Until ruled* reading. Before calling anything a silence, search every design
doc, `DECISIONS.md` and the fix-pass record (`completions/artifacts/M55/fix-pass-rc25/`) and quote what you find; say
plainly *genuine silence* / *answered (by which sentence)* / *two docs saying two things*. Each open question carries
the **robust answer** — the end state with no known loss and no known revisit, checked against `CLAUDE.md`'s
invariants, even when that means a product fix or a doc change — and names which of doc and code is wrong; never only
a choice between today's readings. Do not rule; do not stop unless a cell cannot be built at all without the answer.
The open entries 4 and 7: if a B9 door meets either, say so and build its cell.

## The self-check — run it on your own work before returning, and report it

Each item is a mistake a past review found. For each, say what you checked and how (file:line, a fence case, a run).

1. **A word or line wider than its sentence.** For every new frame word, line and condition: construct the case where
   it would excuse a real loss or write — a sibling's or another unit's bytes, a user's own file or worktree, a managed
   doc, two where the sentence allows one, a place that already stood, a refusal path — and show it red in a fence
   case. (Caught before: `May::Carried`, the re-seed's `Rebuilt`, the base-pin rule, `W1`/`W4`, `A5`/`M5`,
   `<minted>` reading only the working tree, `Allows::Registered` unchecked, `RL3`/`CS3` parking any doc.)
2. **A word that compares less than its sentence names.** Every byte the sentence does not let change is compared —
   keys, headings, front matter; a ref matched by its grammar, never as a substring; a value checked, not only its
   key; a path checked with its relative part. (Caught: *migrated*, *repointed*, *retitled*, *its knob set*, *moved
   aside*.)
3. **Unit-blind sets.** Every set the comparison reads is keyed on the acting unit. (Caught: `staged_docs`,
   `CopyOfAStagedDoc`, `TheWorktrees`, *moved aside* into a sibling's place.)
4. **A "silence" the docs answer or contest** — the search above, done for every entry. (Caught: entries 13, 14, 16,
   17, the pending-config question, entry 18's missing sentences.)
5. **Cells that cannot fail.** A plant that never reaches the command; a holding nothing checks; leads that build a
   state which hides the case; a need scheduled that can be built now; a lead called unbuildable that can be built.
   For each scheduled need and each lead: why, exactly.
6. **A row missing what its door writes.** Drive each leaf once with nothing planted and compare everything it writes
   with the row's lines (caught: `join`'s index rebuild, the re-seed lines, the message file).
7. **A ruling not held at its own door.** Every ruling this table records has a cell that runs the door the ruling
   binds, so the fixed product turns it green (caught: bound 2's ruling held only at finalize).
8. **A refusal path judged as if at exit 0.** A refusal that committed, staged or moved anything is held to what it may
   leave (caught: *it committed*).
9. **Counts.** Every count re-derived from the suite and the committed print, never copied; previous-release claims only
   from the committed print; each cost labelled measured or projected.
10. **Every frame change re-read** on every cell it touches, on both binaries, with what moved named.
11. **The sense-check.** Every line you write: does what it allows make sense for a user? If not, it is a finding
    against the doc (a key or a §6 entry), never an allowance.
12. **Hygiene.** No host path, name, id; the human is "they"; new family files reached by `cargo fmt`
    (`tooling-tests/fmt_reach.rs`); the quote fence holds every new quote.

## Return

Compact: each commit's sha and its gate's `passed=` line; the pushed remote head; the family's numbers in five lines;
what moved on both binaries; the self-check, one line per item; then one block per new question (the sentences on each
side quoted, which of doc and code is wrong, the robust answer and why, the reading until ruled). Anything left open,
with why. Full detail goes into the records, not the return.
