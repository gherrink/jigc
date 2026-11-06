# The route catalog's reason texts, classified

*The plan before the stabilization run's opening, task `P-F0`, its first commit ([detail.md](../pre-opening-plan/detail.md) → *Lane P*, P-F0). Read at `c69a2f94` (branch `fix/rc24-route-catalog-rest`) by the task's builder. A reading of the catalog and of the product's code, row by row with the evidence it rests on — no case was run to make it, so every row whose class differs from the word the catalog gives it carries what was not driven.*

The table is [route-catalog-reason-classes.tsv](route-catalog-reason-classes.tsv), one row a line of the projection. Its three scripts are in [route-catalog-reason-classes/](route-catalog-reason-classes/): `extract.py` lists the texts as the source writes them, `classes.py` holds the classification, and `table.py` joins the two to the projection and prints the table. Run from the repository's root at `c69a2f94`, `python3 -I completions/artifacts/M55/stabilization-build/reshape/route-catalog-reason-classes/table.py .` prints the committed file byte for byte.

## What was counted

**The plan's "about 164" is the count of what the source writes.** A reason text is the argument of a `Why::NotConstructible(…)` or a `Why::NotBuilt(…)` in the nine command files under `crates/cli/tests/route_catalog/`. There are **164** of them at `c69a2f94`: 157 `NotConstructible`, 7 `NotBuilt` (`grep -c` over the nine files, `route_catalog.rs` itself left out, since every `Why` written there is a fixture or a `match` arm). They hold **153 distinct texts**. Eleven texts are written more than once, because a `const` repeats one text, and one `NotConstructible` sits inside the macro `the_step_before_answers!`, which two reasons expand.

**Those texts stand on 201 lines of the projection**: 9 `reason` lines, 114 `without-case` lines and 78 `not-followed` lines whose word is `not-constructible` or `not-built`. A `const` or a macro repeats one text over several sites, so the lines outnumber the texts. The text *a write that fails on an open file is a full disk or a failing one* is one text on thirteen lines. The two `platform` reasons and the word `no-route-printed` are out of the task, as the spec has it. The table has a row for every one of the 201 lines, and no other row; the classification is made per text, and a row whose site would change the class says so.

## The rule

This is the rule `P-NC2` applies to the texts the C-tasks write.

- **not constructible** — no state the runner could plant reaches the cause, or the way cannot be followed by any act under the catalog's rules: a race inside one command, a fault of the disk or the operating system, a constant compiled in, an earlier guard that answers the same state first under its own code, a cause followed at another site, or a way that is no way distinct from one followed. **The reason stands.**
- **plantable with piece X** — the state can be planted, or the way followed, once the frame has the piece the row names. No piece changes product source.
- **not built** — the frame plants the state and follows the way today: the case is not written. The row names who writes it.

**Two columns ride beside the class.**
- **F-C** says whether the route leaves an act (`act`) or an operand (`operand`) to infer, by the human's ruling F-C → A of 2026-10-28 ([detail.md](../pre-opening-plan/detail.md) → 0.1): such a route is a finding of its own class, in the product's text, and its case belongs to `P-F1`'s left-to-infer classes.
- **second class or doubt** says where a text could fall in two classes, and what was not driven.

## The split

| | lines | texts | declarations |
|---|---:|---:|---:|
| not constructible | 125 | 84 | 89 |
| plantable with piece | 48 | 43 | 47 |
| not built | 28 | 26 | 28 |
| **all** | **201** | **153** | **164** |

**F-C:** 8 lines leave an act to infer, 1 an operand, 4 are in doubt between the two readings; the other 188 leave nothing to infer.

### The pieces

| piece | lines | owner |
|---|---:|---|
| `rerun-with-a-value-of-the-readers-own` — the refusing command again with one argument a value of the reader's own (`Own`) | 9 | P-F0 |
| `P-F1`'s left-to-infer class — the route names no act, or no object, for the way | 9 | P-F1 |
| `operand-from-the-listing` — an act's object taken from the refusal's listing, by index, where the route says *those paths* or *the paths above* | 6 | P-F0 |
| `write-a-file-of-the-readers-own` — bytes the reader writes: a change, a doc's prose | 4 | P-F0 |
| `route-from-a-report-row` — a route read off `jigc migrate-corpus`'s `blocked <code>:` row | 3 | P-F0 |
| `stash-and-commit-in-a-worktree` — a stash, and a commit inside a linked worktree | 3 | P-F0 |
| `rerun-beside-another-ways-line` — a re-run where the route prints a `jigc` line for another way | 3 | a ruling: it changes the human's rule of 2026-10-12 |
| `shell-line` — a printed recipe joined by a shell's operators | 3 | a ruling: it changes the catalog's own rule (`Act::RunListed`) |
| `env-for-the-refusing-command` — an environment the refusing command has and the route's commands do not | 2 | P-F0, **the first piece** |
| `move-to-a-printed-path`, `move-the-repository`, `merge`, `drop-a-line`, `move-head`, `remove-a-directory` | 1 each | P-F0 |

**The first piece is the spec's own.** `setup.unverified-install-path`'s *git cannot be run* (T91) and `uninstall.untracked-workbench-file`'s *`git` cannot be run* (T147) are reached with no `git` on `PATH`. A case's `Var`s are the environment of the refusing command and of every command its route runs (`Case::env`), so no act can put `git` back before the route's re-run. One clause of both texts does not hold: *its own `git` children would not start*. The runner's own children run under the mask as it stands, never under `Case::env` (`route_case.rs`, `Sandbox::tested` beside `Sandbox::git`).

### Not built, and who writes it

| owner | lines | sites |
|---|---:|---|
| P-F0 | 3 | `jigc uninstall`: `uninstall.dirty-worktree` (T131, T135), `uninstall.staged-prose` (T141) — the family is whole and has no C-task |
| C1 | 8 | `doc read`: `store.home-not-regular-file` (T9, T12, T13), `store.link-on-the-way` (T18, T20, T22, T23, T24) |
| C4 | 1 | `config.repoint-failed` (T4) |
| C6 | 2 | `write.title-ignored` (T25, T26) |
| C7 | 3 | `create.already-exists` (T25, T28), `write.already-present` at `relocate::refuse` (T127) |
| C8 | 1 | `milestone.provision-failed` (T35) |
| C9 | 4 | `milestone.area-io` (T38, T40, T41 twice) |
| C13 | 4 | `finalize.provenance-io` (T113), `finalize.promote-clobber` (T119, T120, T121) |
| C14 | 2 | `finalize.nothing-staged` (T99), `finalize.stage-failed` (T100) |

A row is named to the C-task that takes its site's file, in its family, in the plan's table (C1–C15). The C-tasks' own scope is the scheduled list, so a row named here adds to it.

**The seven `NotBuilt` texts the catalog writes today** (T12, T13, T18, T20, T22, T23, T24, all in `doc_read.rs`) are all still *not built*, except T24. T24 is a way whose route, followed as printed, meets the install's dirty gate, so its case comes out *followed, another failure* unless the frame learns to follow the second route. Its second class says so.

## What the reading found — four keys

Each is a heading of [route-catalog-findings.md](route-catalog-findings.md) → §14, with its rows.

- **`r1-catalog-reasons-call-a-plantable-state-not-constructible`** — 18 lines. Each calls *not constructible* a state the frame plants today:
  - a permission bit, a declared bound whose cases the catalog pins elsewhere (T4, T35, T38, T40, T41, T100, T113, T135);
  - a pack of the reader's own, a ground the catalog words `NotBuilt` in three texts (T12, T13, T20) and `NotConstructible` in six (T25, T28, T119, T120, T121, T127);
  - a committed spec, whose text says *the state was not built* (T9);
  - a state step in a sub-task's worktree, `Step::JigcAt`, which landed after the text (T141).
- **`r1-catalog-reasons-stale-against-the-frame-they-stand-in`** — 3 lines whose ground the frame has moved past:
  - T26 rests on the re-run rule before the human's ruling of 2026-10-12;
  - T99 says a way is not followed that the three `nothing_staged!` cases follow;
  - T131 rests on the frame running no line of the message, which `Act::RunListed` has done since the milestone family.
- **`r1-catalog-migrate-corpus-reasons-miss-the-shipped-changelogs-relocation`** — T15 and T128 say no shipped schema has a home move pending. The changelog moved its home at its schema-version 2 (M38), and the byte table plants a version-1 changelog at the prior home (`a_stale_changelog!`).
- **`r1-catalog-reasons-hold-routes-that-leave-an-act-or-operand-to-infer`** — 14 lines whose reason is, by F-C → A, a route that leaves an act or an operand to infer, filed as a reason and not as a finding (T46, T67, T68, T95, T96, T108, T109, T122, T130; T3, T7, T42, T145 in doubt; T114 beside it).

## What this does not establish

- **No row was driven for this table.** The rows that call a state plantable (the 28 *not built* and the pieces' states) rest on what the frame plants and on the product's code as read. Every one of them that has a doubt says so: chiefly whether an earlier step refuses over the same planted bit first. Their cases are where that is learned.
- **A *not constructible* row that agrees with its text takes the text's own evidence** — *driven*, or the code it names — and was checked against the code where the text names no evidence. It is not a second derivation of each cause count.
- **Which pieces `P-F0` builds** is for its later commits and the main session; the table names the owner of each, not an order.
