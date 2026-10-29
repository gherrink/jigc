# Brief — the byte table, task B10: the rulings, the full record, `pinning.md` §7 (2026-10-26)

One builder does all of B10. A read-only review follows, then fixers — one per frame hole, one last fixer for every
record note and the numbers (the human's ruling of 2026-10-19). Fewer review findings is the aim: run the self-check
below and report it.

## Rules

`b9-mint-family-brief.md` → *Rules* (same directory as this brief), with: start from HEAD `09268a5e` (`git log -3`; a
dirty tree stops you); messages end "(the byte table, task B10, the first/second/… of its commits)". The previous
release is the rc.24 rebuild from `91834b5e` (sha256 `accf3996…ab5d`), never the `jigc` in `~/.local/bin`. **Never
print an environment variable's value** — no `env`, `printenv`, `pgrep -fl`/`ps` with full command lines or
environments; names only. **`implementation/working-principles.md` first** — neither the doc nor the code is the
oracle. A ruling's product or doc change is a finding of the run's ledger, not a change in this lane (the form
`b7ef80e6` used).

## Read first, yourself

- `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md` — §6 (the records, and *what the record
  at the stop must count*), §8 (`B10`), §9, §10.
- The records beside it: `byte-table-acceptance-rows.md` (§6 whole, §7), `byte-table-readings.md`,
  `byte-table-findings.md`, `byte-table-pilot.md`, and the five family records with their `.tsv`s
  (`install`, `doc`, `milestone`, `movers`, `mint`) — the forms you consolidate.
- `DECISIONS.md` from the top down to `b7ef80e6`'s entry.
- The reviews and fix briefs in this directory (`b6a-review.md` … `b9-review.md`, `b8-fixes-brief.md`,
  `b9-fixes-brief.md`): what reviews catch in a record — counts copied instead of re-derived, previous-release claims
  not resting on a committed print, a cause misattributed, a lead with a cost given as its reason.
- `implementation/pinning.md` (§6, the route catalog, is the model for §7), `implementation/decisions-pending.md`
  (`wf-pre-byte-matrix`, `wf-byte-table-rest`, `wf-pre-regression-part-two`), `CLAUDE.md` → *Architectural
  invariants*.

## Commit 1 — the human's three rulings of 2026-10-26

In the form `b7ef80e6` used: `DECISIONS.md` at the top; every record that states the question as open updated, the
question kept; every cell whose classification or mark a ruling moves re-read on **both** binaries and named. A frame
change a ruling needs may be its own commit before the records, red first.

1. **Entry 25 (Q-a) → A.** A sub-task's first re-entry is decided by jigc's own provenance record. An entry at the
   commit doc's place that has no provenance entry — a file, a directory, a link, whatever it is — refuses, naming the
   path; the check follows no link. Wrong: the code (it takes a user's file as the commit doc, and writes over a link);
   the doc owes the sentence (`design/team-ready-state.md` / `design/write-commands.md` → the re-entry — find where it
   belongs). Why: entry 23's principle — jigc never overwrites or adopts a file to make room for its own — and
   `CLAUDE.md`'s *conflicts blocked and routed to a human, never silently merged*. For the table: the four cells marked
   with entry 25 lose the mark and are listed as refusing, red under their key (say whether the two keys
   `r1-workflow-re-entry-takes-a-file-of-the-users-…` and `…-replaces-a-link-of-the-users-…` stay two or merge, and
   why).
2. **Entry 4 → A, with the human's rule.** `setup` writes no file for the spawn launch-template: it lives in the
   profile the binary carries (`crates/cli/adapters/claude-code.yaml`) and `setup` only checks it before any write
   (`crates/cli/src/setup.rs` ~1767). Wrong: `design/project-setup.md`'s boundary table (*"generate … the spawn
   launch-template"* → checked at install). The table needs no line: a file `setup` writes that S1–S8 do not cover stays
   red. **The human's rule, verbatim in substance:** jigc never launches a sub-agent itself. It prints the launch,
   rendered through the adapter's template, as an instruction the coding agent carries out with its own primitive
   (for Claude Code, its Task tool), so the sub-agent is visible in the coding agent's environment; the sub-agent then
   calls jigc for its instructions. *"We need to respect the coding agent's environment completely, not build our
   own."* Found on 2026-10-26: the profile's template is checked at install and **never rendered** — the `Spawn:` line
   is built in the engine (`crates/engine/src/compose.rs`, `emit_fan_out_spawns`, ~1808) as a bare
   `cd … && jigc workflow … --task …`; `design/assistant-adapter.md` §3 says the CLI renders the launch through the
   template. Record this as a finding for the run's ledger (in `byte-table-findings.md`, under a key or as a lead —
   it is no byte cell — with: the product renders the `Spawn:` line through the profile's template, one renderer, the
   engine's worktree-path rule kept; the rule as a sentence in `design/assistant-adapter.md` §3; a source fence that no
   jigc code launches an assistant program). Verify each claim of this paragraph yourself before you write it.
3. **Entry 7 → A.** No door owes the edge index's removal, and none is faulted for it: **any door that commits** may
   remove or rebuild `.jigc/index/edges.json`, never required. Why: the index is stamped with the `HEAD` it was built
   from (`crates/engine/src/index.rs`), so a read after any commit rebuilds it; entry 1's ruling (a rebuildable cache
   is never a loss). Wrong: nothing in the product; `design/storage.md` owes one sentence (the stamp covers every
   commit, so no door removes the file for correctness). For the table: a line (or a widening of an existing cache
   line) that excuses the index's removal or rebuild at every committing door at exit 0 — red first with a fence case
   that it excuses nothing at a door that does not commit and nothing but `edges.json`; name every cell it moves on
   both binaries.

## Commit 2 — §6 swept

Every entry of `byte-table-acceptance-rows.md` §6: its state read from its **body**, not its heading — ruled (by
which `DECISIONS.md` entry), answered (by which sentence), or open. Every heading says its state the way entries 14–25
do. An entry still open carries both sides quoted, which of doc and code is wrong, the robust answer and its
*Until ruled* reading — and is named in your return. (Entries 3, 6, 8, 9, 10 and 12 have headings that do not say a
state; `b7ef80e6` ruled seven install questions. Check each.) May fold into commit 1 or 3 if small; say which.

## Commit 3 — the full record

`completions/artifacts/M55/stabilization-build/reshape/byte-table-record.md` (+ its `.tsv`s), the table as a whole, in
the five family records' form, superseding none of them (each family record gains one line pointing at it). From a
**fresh whole-table print on both binaries after commit 1**, committed beside it as `5b528b33` did.

- Everything §6 of the plan says *the record at the stop must count* — over the whole table, each number naming its
  source (suite / print / on demand), re-derived, never copied from a family record. Where a family record's number
  disagrees with yours, say which is right and why.
- **The keys, as the run's ledger will take them.** One line per key: key · doors · cells red on this tree / on rc.24
  · classification · regression (red here, green on rc.24) or not · which of doc and code is wrong · its §6 entry or
  ruling · its fix-pass row if any. **Shaped for a model to work with** (the human's ruling of 2026-10-20 on large
  lists): grouped by door and then by cause, a one-line count per group at its head, no line longer than it needs —
  the run's opening reads this list, so it must be usable without reading the whole record.
- The leads still open, each with the exact reason no cell sees it (a cost is not a reason).
- The scheduled list, each row with its reason and trigger; what of it `wf-byte-table-rest` carries.
- Seconds, each labelled measured or projected: one fan-out cell and one task cell alone; the table's arms inside a
  gate; the Linux line through `dev/runner-faithful` for `byte_table::`. If a Linux run cannot be made, say why — no
  number without a measurement.
- What each task cost: not recorded by the orchestrating session — say so; do not estimate.
- What the table does not establish.

## Commit 4 — `pinning.md` §7 and the pending rows

`implementation/pinning.md` → a §7 beside §6, at §6's scale: what the byte table is, how a row comes from the docs (a
line only over a quoted sentence; the sense-check of `working-principles.md`), the classifications, expected-red keys
and marks, needs and the scheduled list, the on-demand and previous-release arms, and **how a new writing door is
added** (the fence that forces an answer). Points at the records, restates none. `decisions-pending.md`:
`wf-pre-byte-matrix` bracketed as built (by this record), `wf-byte-table-rest` narrowed to what actually remains.
`doc_link_fence` green.

## The self-check — run it on your own work before returning, and report it

`b9-mint-family-brief.md`'s twelve items, each answered for this task (frame words: commit 1's lines and fence
cases). Plus:

13. **Every number in the record re-derived** from the suite or the committed print — name the command that produced
    it; spot-check three family-record numbers against yours.
14. **Every previous-release claim** rests on the committed print of commit 3, not on an older one.
15. **Every key line** checked against `EXPECTED_RED` (no key missing, none extra) and every regression claim against
    the two prints.
16. **The keys list is usable alone**: someone given only that section can tell what is broken, where, and which of doc
    and code changes.

## Return

Compact: each commit's sha and its gate's `passed=` line; the pushed remote head; the table's numbers in five lines;
what moved on both binaries; the self-check, one line per item; every §6 entry still open, one block each (both sides
quoted, which is wrong, the robust answer, the reading until ruled). Anything left open, with why. Full detail goes
into the records, not the return.
