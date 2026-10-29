# Brief — the byte table, task B6, the family itself (2026-10-18)

You build **the `doc` family** of the byte table: the rows from the docs, the cells, the numbers record. Read
`b6-doc-family-brief.md` beside this file whole — its rules (worktree, gate, commits, trailers, push, hygiene,
silences, return) all hold, and its sections *Then the family, on the plan's pattern* and *Where the docs are
silent* are your task. Its *Commit 1* and *Commit 2* are **done — do not redo them**:

- `b7ef80e6` the seven rulings; `f4c12fa3` the plant `Plant::InPlaceOf` and 22 cells;
- after a review (`b6a-review.md` beside this file) four fixes: `75bd2e4d` (entry 13 answered),
  `77bc254d` (an in-place user's file drops its index entry; links and directories keep theirs), `9d664d76`
  (`Plant::Deleted`, a plain `rm`, counted as kind E; 13 cells), `42a0776f` (records and numbers reconciled).

Read those commits' `DECISIONS.md` entries (newest first, at the top) before anything else. Start from the
worktree's HEAD (`42a0776f` unless something moved — `git log -3`; a dirty tree stops you).

**Use both new plants where the doc family's rows name a need they answer** — a doc's home with a link, a
directory or a user's file in its place, and a doc's file deleted outside jigc, are the cases these plants were
built for B6 to have (the human's rulings of 2026-10-18).

**One addition to the numbers record.** Every claim about the previous release since 2026-10-15 rests on runs whose
output was never committed (fix 4 marked each one so). The family's numbers commit includes a committed print of
**the whole table** run on the recorded previous release — the rc.24 rebuild from `91834b5e`, sha256
`accf3996…ab5d`, not the `jigc` in `~/.local/bin` — on the arm that runs a named binary, as the 2026-10-15 print
was made; and every record that states a previous-release count states it from that print, removing the
"not established" notes it settles.

`owed.rs` loses B6's eight leaves. Return as `b6-doc-family-brief.md` → Return asks.
