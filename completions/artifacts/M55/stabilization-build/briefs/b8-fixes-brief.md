# Brief — fixes after the review of B8, the movers family (2026-10-19)

The review of `43b555fe..29774121` on `fix/rc24-route-catalog` is at
`~/.claude/projects/<project>/stabilization-briefs/b8-review.md`. Read it
whole, and `b8-movers-family-brief.md` beside it (its *Rules* hold for you). By the human's ruling of 2026-10-19:
**one fixer per frame hole (grouped where the holes share one word or line), one last fixer for every record note
and the numbers.** Eight fixes, **one fixer each, one commit each, in this order**; you are told which one is yours. Do
only yours. The review's probe tests are in its scratch copy (named in the review) — reuse them, do not trust them unrun.

## Rules for every fix

`b8-movers-family-brief.md` → *Rules*: worktree `~/projects/gherrink-jigc-route-catalog`, start from HEAD
(`git log -3`; a dirty tree stops you), red first, the full `dev/gate` in the background read unpiped, `git commit -F`
with its trailers, push by name, no stash, no `rm -rf` on a variable path, no host path / name / id in a file, the
human is "they", no edits to `.claude/` or `CLAUDE.md`, the previous release is the rc.24 rebuild `accf3996…ab5d`.
Messages end "(the byte table, task B8, after its review: the first/second/… of its fixes)". `DECISIONS.md` newest
first. **Read `implementation/working-principles.md` first** — neither the docs nor the code is the oracle: judge what
makes sense for a user, then say which of doc and code is wrong. Every frame change: re-read every cell it touches and
say whether any classification moved, on this build and on the previous release. A count you change, you note in your
return; fix 8 reconciles the records. Anything you leave open is named in your return with why.

## Fix 1 — the race in the suite's copies (the review's *Gate failure*)

`git_init` (`crates/cli/tests/support/trial_corpus.rs`) leaves git's auto-maintenance on; each commit starts a detached
`git maintenance run --auto` that writes and deletes files in `.git/objects` after the commit returns, and `copy_tree`
(`:1091-1120`) panics on a file that vanished between listing and copying. Close it at the source (auto-maintenance
off for every repo the suite makes — and gc's auto too if it can do the same), red first if a test can show it (a
fence that every repo the support builds has it off), and say whether `copy_tree` should also tolerate a vanished
file. This is the `copy_tree` flake recorded for the run's opening ledger, and the `record_door_baseline` failure of
B8: note both in `DECISIONS.md` as closed by this.

## Fix 2 — H1, H2: `RL3`/`CS3` park any doc, and a managed one

Narrow the parking lines to exactly what squats the destination, and never a doc jigc manages
(`design/reconciliation.md:196,199` says the move blocks). Red first with the review's probes (a `config set
docs-root` parking the decision it should move; `relocate` parking an unrelated committed decision; a managed occupant).

## Fix 3 — H3: *migrated* ignores keys, headings and front matter

*migrated* must hold every byte the migration's sentences do not let it change — `- key:` lines, `#` lines and front
matter included; a dropped bullet, sub-heading, `supersedes` edge or flipped `status` → not holds. Red first.

## Fix 4 — H4, H11, H7: `rename`'s words

H4: *repointed* matches the old id as a substring on any line — hold it to the ref's own grammar (a ref to
`adr:one-way-two` rewritten → not holds; a prose mention rewritten → not holds unless a sentence allows it). H11:
*retitled* checks the new slug is the one the title gives. H7: `rename`'s destination becomes a line's place, with
cells for a squatter there (link, directory, file) on both binaries. Red first for each.

## Fix 5 — H5: `config set placement-root`

Cells for `config set placement-root`, including a file the user committed at the new home (entry 23's red case,
driven by the review on both binaries); `CS3`'s "excuses nothing" reason corrected.

## Fix 6 — H6: bound 2's ruling held at its door

A doc-family cell that runs the write itself over a hand edit in the same part (the ruling of 2026-10-19: the write
refuses, naming the part), so the fixed product turns it green; the two former bound-2 cells' leads checked so they
fail on the product's answer, not on their lead.

## Fix 7 — H8, H9, H10: the narrow words

H8: *it committed* — a refusal that committed one doc may not rewrite the doc it reported blocked. H9: *its knob set*
checks the value. H10: one sub-task's leftover held to its own `.jigc/displaced/<sub-task-id>/` place, not a
sibling's. Red first for each.

## Fix 8 — R1–R11, and the numbers reconciled

Every record note of the review. R1 and R2 with the working principle: `relocate` refusing a doc at exit 0 is chosen by
`command-output-contract.md:175` and `validation.md:734` — judge whether it makes sense for a user, then either a key
that names the docs as wrong or a §6 entry; entry 23 reframed (the settled foreign-only bound, the 2026-07-07 entry,
`validation.md:734`; for a root knob the robust answer is to refuse the whole re-point, since blocking one doc strands
it). **The human's ruling on entry 21 (2026-10-19) → A**, to record in the form `b7ef80e6` used (`DECISIONS.md`, rows §6 entry 21 answered with the question kept, readings/findings updated, its cell re-read and its mark dropped): `migrate-corpus --no-commit` stages exactly the paths the migration touched — both halves of a move — and commits nothing; `design/corpus-migration.md`'s sentence, the guide's and the product are wrong, `design/write-commands.md` is right; the doc and product change is a finding of the run's ledger. **The human's ruling on entry 22 (2026-10-19) → A**, recorded the same way: a `migrate-corpus` whose commit a hook rejects rolls back — every migrated doc restored, nothing left staged, the repository as it was — and the refusal names the hook's output (to inspect, re-run with `--no-commit`, which by entry 21 stages exactly the migrated paths); the stated bound in `design/corpus-migration.md`, its echo in `design/finalize.md:190` (the door's state-truth clause, and `DECISIONS.md:11829`'s clause built on it) and the product are wrong; why: `CLAUDE.md`'s *writes are transactional*, the human's *every refusal of `setup` leaves the repository as it was*, and `rename`'s restore. R3 (still quote): entry 22 quotes `finalize.md:190` and `DECISIONS.md:11829`. R4: the three keys cite their fix-pass rows. R5–R11
as the review says. Also `byte-table-plan.md`: beside its rule that a line stands only over a quoted sentence, the sense-check of `implementation/working-principles.md` (a sentence that allows what makes no sense for a user is a finding against the doc, not an allowance). Fix 5's left-opens (`c5142e6c`): `config-set-a-file-in-the-place-of-the-decisions-directory` is listed as acting and holds while the re-point strands the committed decision — relist it as refusing (as its `placement-root` twin is) and red under `r1-config-set-re-points-past-a-link-to-nothing-or-a-directory-at-a-doc-it-strands`, red first; `placement-root`'s refusal in the operating system's words with no jigc code → a lead for the route catalog in the findings record; R6's sentences that fix 5's cells falsify. Fix 7's left-opens (`8dc9fc84`): *moved aside* does not hold the file's relative path inside its unit's place (the docs: *"with its relative paths intact"*) — narrow it, red first, if it is a contained change, else a record row; the product's `.2`/`.3` suffix on a taken name, which no doc mentions — judge it by the working principle and record a key or a §6 entry; the cache read as at exit 0 on a refusal, and a multi-commit refusal judged on its last commit — record rows with why. Then every count in every record, re-derived from the suite and a fresh whole-table print on both
binaries after fixes 1–7, committed as `29774121` did.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, what moved (on both binaries), counts changed,
anything left open with why. Fix 8 also: each open §6 entry in one block — both sides quoted, which of doc and code is
wrong, the robust answer, the reading until ruled.
