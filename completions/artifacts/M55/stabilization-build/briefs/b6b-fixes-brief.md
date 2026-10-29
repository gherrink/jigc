# Brief — fixes after the review of B6b, the `doc` family (2026-10-18)

The review of `42a0776f..52fca6e6` on `fix/rc24-route-catalog` is at
`~/.claude/projects/<project>/stabilization-briefs/b6b-review.md`. Read it
whole, and `b6-doc-family-brief.md` + `b6b-doc-family-brief.md` beside it for the rules this work is under. Four
fixes, **one fixer each, one commit each, in this order**; you are told which one is yours. Do only yours.

## Rules for every fix

Same as `b6a-fixes-brief.md` → *Rules for every fix* (beside this file) — read that section: worktree
`~/projects/gherrink-jigc-route-catalog`, start from HEAD (`git log -3`; a dirty tree stops you), red
first, the full `dev/gate` in the background read unpiped, `git commit -F` with its trailers, push by name, no
stash, no `rm -rf` on a variable path, no host path / name / id in a file, no edits to `.claude/` or `CLAUDE.md`.
Messages end "(the byte table, task B6, after the family's review: the first/second/… of its fixes)". The previous
release is the rc.24 rebuild from `91834b5e` (sha256 `accf3996…ab5d`), not the `jigc` in `~/.local/bin`. A count
you change, you note in your return; fix 4 reconciles the records.

## Fix 1 — review finding 2: the base-pin rule must not hide a loss

`byte_cell.rs:1280-1286` (from `8fba32a5`) stops classing a staged doc deleted while its unit's `base.json` stands
as *lost*; under `T6`, `A1`, `M6`, `W4` (every line with `May::Removed` under `.jigc/tasks/*`) such a deletion now
*holds* — e.g. `task finalize t1` deleting t2's staged doc. Narrow the exemption to exactly what it was for
(`doc rename`'s re-slug inside a live task: the same unit gains the new staged doc) — the review proposes a word of
its own for `W4` and the skip only for paths that word covers; choose the narrowest grounded form and say why. Red
first: a fence case for the opposite (a staged doc of another live unit deleted, its pin standing → *lost*),
failing before the change. Re-read every cell the rule touches; say whether any classification moved.

## Fix 2 — review finding 3: "nothing was persisted" is false where it prints

`write.rs:6318-6330`: the edit leaves that refuse after the copy-in print "nothing was persisted" while the copy is
staged (and `doc author` leaves `roles.json` and provenance for a doc no longer staged, against
`write-commands.md:43` "persist only on full success"). A ledger key of its own in the findings record (surface
contract law 1, and the `author` half under `write-commands.md:43`), with the review's rig repro run as printed on
this build and on the previous release, independent of how entry 14 is ruled. Where cells already show it, they
carry the key (fix 3 re-marks the silence). No product change — this lane records.

## Fix 3 — review findings 1 and 4: entry 14 says two things, split in two

Rewrite rows record §6 entry 14 and readings §13 as **two docs saying two things**, quoting both sides as the review
lists them (`reconciliation.md:216` whole, `:47`; `auto-migration.md:82`; `finalize.md:143` (4); the human's
item-23 ruling at `DECISIONS.md:4437`; against `doc-read-surface.md:16`, `:197`; `surface-contract.md:206`), and
split it into **(a)** whether a write's copy-in takes a file that is no doc of its type (the cells of `add-item`
and `create`), and **(b)** whether a write that copied in and then refuses keeps the copy (the rest). For each: is it
still open after reading every sentence, or does one sentence (e.g. `write-commands.md:43`) answer it — say so
plainly, and only an open one keeps a silence mark. Re-mark the 24 cells: a cell red under both readings carries no
silence mark; fix the self-contradicting "other reading" text. A doc pair that disagrees is an `inconsistency`
finding of its own in the findings record.

## Fix 4 — review findings 5, 6, 7, 8, and the numbers reconciled

F5: "fifty-one hold" → the count the print gives (the review: 48). F6: red counts on the previous release say how
many are red and how many did not build (125 + 2; the install family's 73 + 2), not one sum. F7: `W1`/`W4` cover any
task's area while `write-commands.md:240` says a write cannot stage into a sibling's area — a cell with two live
tasks and a sibling's staged doc, or a scheduled row with its reason; prefer the cell. F8: create/author over a
deleted doc file cite `command-output-contract.md:356`, not "the same free home". Then every count in every record,
re-derived from the suite and the print after fixes 1–3.

Also the entries fix 2 left open — each gets a row in the findings record (a key if it is a finding, an answered
question with its sentence if not, a lead naming the family that will drive it if it is outside the doc family):
`write.target-escape`'s route, in fix 2's class by code only — drive it as printed; the 15 refusing cells
(`write.wrong-shape`, "carries no `# H1`") that leave the copy staged and say nothing about it; the other "nothing was
written" sentences in `task.rs`, `relocate.rs` and `milestone.rs` (a lead for B7/B8 unless a doc-family cell reaches
one).

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, what changed in counts, anything left open with
why. For fix 3 also: per half (a)/(b), open or answered, with the sentence.
