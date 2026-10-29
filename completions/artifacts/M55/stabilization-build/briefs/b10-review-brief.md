# Brief — the review of B10 (2026-10-26)

Read-only review of `09268a5e..1ec6f8d2` on `fix/rc24-route-catalog`, worktree
`~/projects/gherrink-jigc-route-catalog`. You did not build it. Write your report to
`~/.claude/projects/<project>/stabilization-briefs/b10-review.md`; edit nothing
in the repository, commit nothing.

The builder's brief is `b10-record-brief.md` in the same directory — read it whole, and `b9-review.md` for the form and
depth a review here takes. `implementation/working-principles.md` first: neither the doc nor the code is the oracle.

## What to check

1. **The three rulings of 2026-10-26** (Q-a / entry 25, entry 4 with the human's no-spawn rule, entry 7) recorded as the
   human gave them (the brief's commit-1 text is the ruling) — nothing narrowed, nothing widened; every cell they move
   named and re-read; the doc each makes wrong named.
2. **The new line `C4`** (the edge index at a committing door) and the frame fix `eb76c13e`: construct the case where
   each would excuse a real loss or write (a file other than `edges.json`; a door that landed no commit; a refusal
   that committed something; a directory at a file's place) and show it red, in a probe test in a scratch copy.
3. **The entry-4 finding** (§27 of the findings): re-verify on both binaries that the profile's template is never
   rendered and that jigc launches no agent process today; is the finding's fix stated so it holds the human's rule
   (jigc never launches a sub-agent; the coding agent's own primitive does, visibly)?
4. **The §6 sweep:** for each entry whose heading changed, does the body support the state the heading now claims?
   Any entry called ruled or answered that a `DECISIONS.md` entry or sentence does not actually settle?
5. **The full record** (`byte-table-record.md` + `.tsv`s): re-derive at least the headline counts (cells, red here /
   on rc.24, keys, the regression and the two lost-only-here cells) from the suite and the committed print yourself.
   Is "not a regression" for the two lost-only-here cells sound, or does the table's definition of regression hide a
   real one? Every previous-release claim on the committed print?
6. **The keys list**: usable alone (given only that section, can the run's opening tell what is broken, where, which of
   doc and code changes)? Every key in `EXPECTED_RED` present, none extra; the 22 keys no cell can see — each with an
   exact reason, not a cost?
7. **`pinning.md` §7 and the pending rows**: true to the frame as built; points rather than restates; does it say how a
   new writing door is added, and is that what the fence enforces? `wf-byte-table-rest` holds exactly what remains.
8. **Does the record say how the byte table reaches the loop branch `fix/rc24-tier1`** before the run's opening (the
   table is the run's instrument there)? If no record or pending row owes that step, say so as a note.
9. Hygiene: no host path, name, session/agent id; the human is "they".

Never print an environment variable's value. Use `mktemp -d` scratch; no `rm -rf` on a variable path. The previous
release is the rc.24 rebuild from `91834b5e` (sha256 `accf3996…ab5d`), never `~/.local/bin/jigc`.

## Report

Frame holes first (each: what it excuses, the probe that shows it, the smallest correct fix), then record notes
(numbered), then questions that need the human (only ones you confirmed genuine: both sides quoted, which of doc and
code is wrong, the robust answer). Compact return: counts of each, and the path of the report.
