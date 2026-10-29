# Brief — fixes after the review of B9, the mint family (2026-10-20)

The review of `002b2090..5b528b33` on `fix/rc24-route-catalog` is at
`~/.claude/projects/<project>/stabilization-briefs/b9-review.md`. Read it
whole, and `b9-mint-family-brief.md` beside it (its *Rules* and its self-check hold for you). By the human's ruling of
2026-10-19: one fixer per frame hole, one last fixer for every record note and the numbers. Six fixes, **one fixer
each, one commit each, in this order**; you are told which one is yours. Do only yours. The review's probes, rig
scripts D1–D6, both binaries and its probe tree are in
`<scratch>/b9-review.1vHyAN`
— reuse them, do not trust them unrun.

## Rules for every fix

`b9-mint-family-brief.md` → *Rules*: worktree `~/projects/gherrink-jigc-route-catalog`, start from HEAD
(`git log -3`; a dirty tree stops you), red first, the full `dev/gate` in the background read unpiped (never two
gates at once; a failed gate stops the commit), `git commit -F` with its trailers, push by name, no stash, no `rm -rf`
on a variable path, no host path / name / id in a file, the human is "they", no edits to `.claude/` or `CLAUDE.md`, the
previous release is the rc.24 rebuild `accf3996…ab5d`. Messages end "(the byte table, task B9, after its review: the
first/second/… of its fixes)". `DECISIONS.md` newest first. **`implementation/working-principles.md` first** — neither
the doc nor the code is the oracle. Every frame change: re-read every cell it touches and say whether any
classification moved, on this build and on the previous release. A count you change, you note in your return; fix 6
reconciles the records. Anything you leave open is named in your return with why.

## Fix 1 — hole 1: the rulings of 2026-10-20 held at `relocate`'s own door

R1 (*does all it can, then exits non-zero*) and entry 23 (*blocks that doc, moves the others*) need cells a product
that refuses the whole sweep cannot pass: the moved note is something the listing **requires** (e.g. list
`docs/notes/load-tests.md` as moved), and an occupant-at-a-new-home cell with a second note. Red first: show the
review's whole-sweep-refusal probe green before, red after.

## Fix 2 — hole 2: *into its home* holds what its sentences name

`May::IntoItsHome` (four lines — `grep`) must require a doc that was committed (RL1's "every committed `.md`") and a
regular file (finalize's "lands a regular file or refuses"). Red first with the review's two probes (a never-committed
`.md` swept in; a dangling link carried).

## Fix 3 — hole 3: the catalog-spelling cells judged under their spelling

`start-as-the-catalog-names-it` and `workflow-as-the-catalog-names-it` run a spelling but are judged under the plain
lines, so bare `jigc start` minting a task holds against "Never mints". Judge each under its spelling's lines; red
first with the review's probe (and drive the `workflow` case, which the review only traced).

## Fix 4 — hole 4: *provisioned* is a file or a directory, and `WF1`'s link and directory needs

Narrow *provisioned* to "a file or a directory" as its doc says (no link, no FIFO), red first; add `WF1`'s link and
directory needs as cells (the review drove a dangling link at the commit doc's place replaced by jigc's file at exit 0
on both binaries — classify it from the docs and the working principle, a key if red).

## Fix 5 — hole 5: `migrate` then `task finalize --approve` over an uncommitted edit

The lead the builder left undriven is data loss on both binaries: on a committed `HISTORY.md` with an uncommitted
edit, `migrate` then `task finalize --approve` leaves the edit in no file and no git object. Build the cell (an
authored changelog is one heredoc), red under a key of its own naming the loss, with a repro block run as printed on
both binaries. `design/auto-migration.md:76`'s test ("deletes only a file git holds a copy of", by path) and its own
warrant ("destroys only a reviewed, in-repo, **git-recoverable** file") say two things; record it in the form the
human's ruling takes — **if the ruling has not reached you by message, record it as a §6 entry with the review's
robust answer (refuse when the working bytes are in no git object, routing to commit the edit first) and its
*Until ruled* reading.**

## Fix 6 — the review's record notes 1–11, the questions, and the numbers

Every record note:
1. The regression's cause corrected everywhere it is given (findings §25, the first commit's `DECISIONS.md` entry —
   by a new entry, not an edit of the old — and the mint record §1): `5f5b273a` (2026-10-04) changed
   `displace_foreign_squatter`'s `exists()` to `home_entry() == Free` (`relocate.rs:855-860`), so a dangling link
   counts as an occupant and is parked; rc.24 already routed `placement-root` through `relocate_stranded`. Name
   `relocate`'s link-to-nothing cell as the same mechanism (red on both).
2. The moved-cell counts: 18 by the committed prints, the other 665 — re-derive, do not copy.
3. §26's repro block fixed (`mkdir -p` the plant's parent) and run as printed on both binaries.
4. Stale doc comments in `movers.rs` (≈1500-1504, 1830-1846) on `RL3`/`CS3` parking and the removed marks.
5. §26's lead reason: a cost is not a reason no cell can see it — build the cell or give the exact reason.
6. The mint record's `needs` / `excused` claims rest on blocks the commit does not carry — commit them or cite only
   what is committed.
7. `CS6`'s "a racer's bytes" corrected to what the code parks (jigc's own copy).
8. `migrate-a-directory-in-the-sources-place` must not pin today's words: the fix that adds a code turns it green.
9. The on-demand base cells that only refuse for a missing precondition (twelve here, the same pattern in B8): give
   each its precondition so it reaches the door, or record why it stays.
10. The (A) twins: add a committed-but-unmanaged doc twin beside the untracked one.
11. `MI2 × E, before`: no change (precedent) — say so in the record.
12. Fix 1's left-opens (`b04d7c43`): `migrate-corpus`, R1's other acting sweep, has no cell a whole refusal can fail —
    every cell builds one stale doc; build a two-doc cell whose listing requires the other doc's migration in the
    commit, red first if a whole refusal would pass today. Fix 1's uncommitted print is
    `<scratch>/fix1-cells.tsv`
    (739 cells) — re-derive, do not copy.
13. Fix 2's left-opens (`d428d6e6`): `May::Migrated`'s `migrated_away` requires a regular file but not one git held,
    while `migrate-corpus` "re-parses every **committed** doc" — narrow it the way fix 2 narrowed *into its home*
    (`a_doc_git_holds`), red first, if contained; else a record row. The fence message's "fifty" mint cells → 51.
    Fix 2's reading of *committed* as "git holds a copy, `HEAD` or the index" stands (a staged doc moved by `git mv`
    keeps its staged bytes; nothing of the user's is at risk) — record it as the reading, with that reason.
    Latest uncommitted print: `.../scratchpad/fix2/cells.tsv` (741 cells).
14. Fix 3's left-open (`71466f7b`): `Spelling::typed_in` infers a spelling's kind from its name (`--…`, `bare`), so a
    future typed spelling with another name escapes the fence. Make the kind explicit where each spelling is declared
    (typed by the command / a state the cell builds), the fence reading that and never the name; red first with a
    fixture of a typed spelling whose name is neither. A frame change in a commit of its own before the records.
15. Fix 4's left-opens (`931c5f60`): a sub-agent's edit to its commit doc surviving a second re-entry has no cell
    (the frame's E kind is a tracked edit; nothing under `.jigc/` is tracked) — build it if a plant can make it, else
    a record row with the exact reason; a later `doc` write following a link taken as the commit doc out of the
    repository — drive it once on both binaries and record it as a key or a lead for the doc family with its reason;
    §26's heading and the mint record's numbers re-derived. Latest uncommitted print: `.../scratchpad/fix4/cells.tsv`
    (746 cells).
16. Fix 5's left-opens (`6af09e03`): a directory at the migration source after the authoring — drive it once on both
    binaries and record it; a source deleted by hand (`Plant::Deleted`) — a cell or the reason the edit cell covers
    it. Q-b is §6 entry 24 (readings §23): if the human's ruling reaches you by message, record it in the `b7ef80e6`
    form and move the cells as it says. Latest uncommitted print: `.../scratchpad/fix5/cells.tsv` (752 cells).
The review's question Q-a (the `workflow` re-entry over a file at the commit doc's place): record it in the form the
human's ruling takes if it reaches you by message, else as a §6 entry with the review's robust answer — "first entry"
decided by jigc's own provenance record, a commit doc with no provenance entry refuses naming the path, the check
follows no link; the code is wrong and the doc owes a sentence. Then every count in every record re-derived from the
suite and a fresh whole-table print on both binaries after fixes 1–5, committed as `5b528b33` did.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, what moved (on both binaries), counts changed,
anything left open with why. Fix 6 also: each open §6 entry in one block — both sides quoted, which of doc and code is
wrong, the robust answer, the reading until ruled.
