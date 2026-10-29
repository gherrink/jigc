# Brief — fixes after the review of B10 (2026-10-26)

The review of `09268a5e..1ec6f8d2` on `fix/rc24-route-catalog` is at
`~/.claude/projects/<project>/stabilization-briefs/b10-review.md`. Read it
whole, and `b10-record-brief.md` beside it (its *Rules*, which point at `b9-mint-family-brief.md` → *Rules*, hold for
you). By the human's ruling of 2026-10-19: one fixer per frame hole (grouped where holes share one word or line), one
last fixer for every record note and the numbers. **Six fixes, one fixer each, one commit each (fix 2 may take two: a
frame change before its records), in this order**; you are told which one is yours. Do only yours. The review's probes,
both binaries and its probe tree are in
`<scratch>/b10-review.mXmotS`
— reuse them, do not trust them unrun.

## Rules for every fix

Start from HEAD (`git log -3`; a dirty tree stops you), red first, the full `dev/gate` in the background read unpiped
(never two gates at once; a failed gate stops the commit), `git commit -F` with its trailers, push by name, no stash, no
`rm -rf` on a variable path, **never print an environment variable's value** (no `env`, `printenv`, `ps`/`pgrep` with
full command lines), no host path / name / id in a file, the human is "they", no edits to `.claude/` or `CLAUDE.md`, the
previous release is the rc.24 rebuild `accf3996…ab5d`. Messages end "(the byte table, task B10, after its review: the
first/second/… of its fixes)". `DECISIONS.md` newest first. **`implementation/working-principles.md` first.** Every frame
change: re-read every cell it touches and say whether any classification moved, on this build and on the previous
release. A count you change, you note in your return; fix 6 reconciles the records. Anything you leave open is named in
your return with why.

## Fix 1 — H1 and H2: the cache lines' arms

H1: a directory made at a cache's own file (`.jigc/index/edges.json`, `.jigc/state/file-state.json`, the stamped file
`Invalidated` names) is excused by the directory arm of an `Under` line (`byte_cell.rs` ~2615) and by `C1`/`C2`. H2: a
link or a directory standing at a cache file's place, replaced by a regular file, holds under *Rebuilt*/*Invalidated*
(`byte_cell.rs` ~2610) while the same link taken away is red — a link or directory where a cache file goes is the
user's (entries 23 and 25). Close both as the review proposes or better; red first with the review's probes A, B, D, E.

## Fix 2 — the human's ruling on Q1 (entry 1 (ii)), 2026-10-26 → A

**The ruling.** jigc's file↔state record (`.jigc/state/`, the hashes of what jigc last wrote for each managed doc) is a
**baseline, not a cache**: the authority on what counts as an out-of-band edit. A refusal leaves it exactly as it was.
The human's ruling of 2026-10-12 (a refusal may leave the caches rebuilt) stands for the true caches — the edge index —
and not for this record. Wrong: `design/storage.md:312` (*"The edge index and the file↔state hashes are rebuildable
from the committed docs"*; line 314 of the same section is right: *"no stamp and no rebuild path"*), and the frame's
`C2`. Whether the product is wrong is what the narrowed line shows. Why: a baseline written over a user's hand edit
makes that edit read as jigc's own to the next drift check — `CLAUDE.md`'s *out-of-band edits are detected and routed*.

In the form `b7ef80e6` used: `DECISIONS.md` at the top; rows §6 entry 1 updated (the question kept; its point (iii) —
a refusal leaves no minted work area — stated as what the findings keys already carry, with the sentence that says so,
or named as still to rule); readings/findings updated. The frame: `C2` narrowed so a refusal may not write the
file↔state record — red first with a fence case — then every cell `C2` excused re-read on both binaries (the review
counts 22 differences in `setup`'s cells here, 26 on rc.24, and 1 in `rename`'s): name what each wrote, and each cell
that turns red goes under a key that names it (extend an existing key or add one; say which and why — under entry 2's
ruling, every refusal of `setup` leaves the repository as it was).

## Fix 3 — H3: a new writing door comes with its row, and the findings fence

Hold the count's leaves to rc.24's frozen list of write leaves (a leaf not on it was not in rc.24 and cannot be
scheduled as untouched), red first with the review's `review_probe_b10_a_new_leaf_scheduled_as_untouched`. The second
half (a later change to a scheduled leaf's handler) — build it if contained (e.g. a hash of each scheduled handler's
source recorded and checked), else a record row with the exact reason. And note 7: a fence that every key in
`EXPECTED_RED` and every live key of the record has its heading in `byte-table-findings.md`, red first. `pinning.md` §7
says exactly what the fences hold, no more.

## Fix 4 — note 5: the *staged* demand and its two cells

The frame demands a commit (`must_land`, `byte_cell.rs` ~3762); a sibling demand — *what the lines changed stands
staged* — makes the two "should make" keys (`r1-migrate-corpus-no-commit-…` and the root knob's staged half) seeable.
Build it, red first, and the cells; each red under its existing key. If not contained, a record row with the exact
reason.

## Fix 5 — L1: a link on the way to a place

On both binaries a link at the sub-task's `docs` directory makes the first re-entry write `commit:add-alpha.md` and
`provenance.json` outside the repository at exit 0 (the review's D2). Add the plant kind (a link at a directory on the
way to a place a line names) so `aimed` derives a need for it, red first, and the cell at `workflow`'s re-entry; under
entry 25's ruling ("the check follows no link") it refuses — red under the entry-25 link key or a key of its own (say
which and why). Re-read every cell the new plant kind adds on both binaries; if it adds many, say how many and keep
them on demand where the gate's budget needs it. If not contained, a record row with the exact reason.

**Fix 1's left-open (1) is yours too (`c9271d9e`):** *written whole* accepts anything that stood before, so a doc written
over a user's link or directory holds by that word (today such a cell is red only because the planted marker is lost).
By entries 23 and 25 — jigc never writes over an entry of the user's — narrow it as fix 1 narrowed *rebuilt*: a file
written whole only where nothing stood or over a regular file the line allows; red first; re-read every cell that plants
a link or directory at a *written whole* place on both binaries and name every class column that moves. A commit of its
own before L1's if that is cleaner.

## Fix 6 — the review's record notes and the numbers

Notes 1–11 and leads L2, L3:
1. A regression is **a harm class on this tree that rc.24 did not show for the same cell** — read off the committed
   print's two columns, the command in record §9; the `reg.` column and every count that rests on it re-derived (the
   review finds five cells under five keys; re-derive, do not copy). The two cells the record never mentioned
   (`setup-at-the-settings-record-a-committed-file`, `relocate-a-link-to-nothing-at-the-notes-new-home`) named with
   their cause.
2. `DECISIONS.md:79`'s numbers (by a new entry, not an edit of the old) and the fence comment's "twice".
3. Previous-release words not on the committed print: cite the run they rest on or drop them.
4. Entry 1's state after fix 2 everywhere "no entry of §6 is open" is said.
5–7. As fixes 3 and 4 left them.
6. `wf-byte-table-rest` holds exactly what remains: the catalog's states as refusal cells, two plants at once (carried —
   dropping a plan item is the human's), every buildable lead not built by fixes 1–5, and what those fixes left open.
8. §27 (the spawn finding) sharpened as the review says: the probe program is jigc's own image or the override; the
   fence is an **allowlist** of what production code may launch (`git`, jigc's own image, the override while it
   exists), over every `Command::new` outside tests; the **CLI** renders (profiles ride in `cli`, the engine stays
   adapter-free); what the rendered `Spawn:` line looks like under `workflow-dialect.md`'s grammar; the five sentences
   in four docs; and L2 (which profile renders where no adapter, or a second one, is installed) as an open part of the
   finding.
9. `EXCUSES_NOTHING`'s reason exact at doors that land no commit.
10. **The merge owed:** a row (record + `decisions-pending.md`, trigger *before the stabilization run's opening*) that
    `fix/rc24-route-catalog` is merged into the loop branch `fix/rc24-tier1`, gated, pushed, and CI read for its tip,
    because the table is the run's instrument there. Do not do the merge.
11. Nothing.
L3: in `wf-byte-table-rest` if fix 3 did not close it.
Fix 2's left-opens (`9f2e6122`): the lock allowance in `C2` (`.jigc/state/file-state.json.lock` may stay) stands as the
orchestrator's reading — the lock is not the record, and deleting a lock another process may hold is the race the stable
lock exists to prevent; say so in the record. `design/reconciliation.md` routes an unreadable record to `rm <the record>`
"because the record is a rebuildable cache": record it as a finding for the run's ledger (the reason is wrong by the
ruling; deleting the baseline un-baselines every doc, so the route itself is in question — the robust answer is for the
run to decide, not this lane). Entry 1 (iii): as the human's ruling reaches you by message, or as fix 2 left it.
Fix 3's left-opens (`8acdce7a`): the rest of L3 (a change to a function a scheduled handler calls that the count did
not name) — carry it in `wf-byte-table-rest` with its reason; the reverse findings fence (every live findings heading is
listed in the record) — build it with the record's reconciliation, red first (it fails today on fix 2's new key); the
frozen list, the hashes and the count re-frozen together when the previous release changes — say so in `pinning.md` §7.
Fix 4's left-opens (`ef3ae056`): `CS5`'s *all or none* reading stated in the record as the fixer's, open to reversal when
§23's finding is ruled; the case no product can pass (a doc only the index held, migrated and staged under `--no-commit`,
read against the last commit) — a `wf-byte-table-rest` row with the frame change it needs; an uncommitted edit under
`--no-commit` (no cell plants it) — a row; the on-demand print refreshed with fix 6's print.
Fix 5's left-opens (`6735a547`, `09270673`): the six refusals at `git add` that name the wrong cause (a stale lock, the
`.gitignore` rule) where the cause is a link — leads for the route catalog in the findings; a link out of the repository
at a directory a line names whole (e.g. `.jigc/index`) — a `wf-byte-table-rest` row; the counts of commit 2 rest on a
print plus one listing correction — your fresh print settles them.
Fix 1's left-open (2): no cell plants a link or directory at `C4`'s place — carry it in `wf-byte-table-rest`, and
correct the reason the code gives for refusing such a plant (a link or directory there is the user's).

Then every count in every record re-derived from the suite and a fresh whole-table print on both binaries after fixes
1–5, committed as `bab8794e` did.

## Return

Compact: the sha(s), the gate's `passed=` line, the pushed remote head, what moved (on both binaries), counts changed,
anything left open with why. Fix 6 also: any §6 entry left open, one block each (both sides quoted, which of doc and
code is wrong, the robust answer, the reading until ruled).
