# Review — the byte table, task B6, commits 1 and 2 (`56839f95..f4c12fa3`)

Reviewer: independent, read-only. Worktree `gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`.
Commits: `b7ef80e6` (the seven rulings recorded), `f4c12fa3` (`Plant::InPlaceOf` + 22 cells).

## Verdict

The rulings are recorded faithfully in substance, and the plant is correct and usable as it stands
for B6 to B9. **One of the two new red keys is false**: silence 13 is not a silence. The design doc
already answers it, and the cell's listing contradicts both that doc and the listing of its sibling
cell. The other new key (`r1-task-discard-settles-no-record-…`) is real, but its class is wider than
its one cell shows: deleting the record file alone (the common shape) reproduces it. The rest are
count slips in the records and fences that could be tighter.

## What I ran

- `cargo nextest run -p jigc --test g_migration -E 'test(/byte_table::(the_reader_reads_what_the_comparison_reads|every_need_of_a_row_has_a_cell|a_mark_names_a_question|the_footprint_record_is_no_cache)/)'`: 4 passed.
- `cargo nextest run -p jigc --test g_migration -E 'test(/byte_table::the_cells_are_run_and_the_expected_red_list_is_exact::(fresh|bare)$/)'`: 2 passed, so the expected-red list is exact for the two states that hold all 22 new cells and the 13 cells that now hold.
- A rig probe, F2's repro below, on this worktree's debug build. Commit 2 changes no product source, so that build is the product under review.
- I read: the brief's Commit 1 and Commit 2 sections, both commits' full diffs, `byte_cell.rs` (the `Plant`/`Kind`/`aimed`/`compare`/`put` paths), the `answers`/`unmet` fences, `task.rs` T2/A5/A8/A9, `install.rs` S1 to S16 and the unborn-`HEAD` root-ignore cells, `milestone.rs` `settle_discarded_sub_task`/`recording_milestone`, `design/assistant-adapter.md:64`, `design/surface-contract.md:167`, `QUICKSTART.md:148-156`, and precedent `228f2eda`.

## Findings

### F1 — should-fix (high): silence 13 is not a silence. The design doc answers it, the cell is mislisted, and `r1-setup-refuses-a-staged-root-ignore-on-an-unborn-head-where-an-untracked-one-rides` is a false finding

- **Location.** `crates/cli/tests/byte_table/install.rs` (`SETUP_OVER_ITS_OWN`, the last cell, `setup-head-unborn-an-edit-to-the-root-ignore`: `Listed::Acts`, `Mark::Silence(13)`). `crates/cli/tests/byte_table.rs` `EXPECTED_RED` (the new key's row). `byte-table-acceptance-rows.md` §6 entry 13. `byte-table-readings.md` §12. `byte-table-findings.md` §7, the second key.
- **Claim.** Entry 13 says *"No sentence says what becomes of a file **staged** at a path it merges into on an unborn `HEAD`"*. The design doc does say it:
  - `design/assistant-adapter.md:64`: *"a path whose index *or* worktree bytes differ from `HEAD` **before the install writes anything** refuses … **One stated exemption, and it is per member:** on an unborn `HEAD` an *untracked* file at a path the install **merges into** rides the repository's first commit"*.
  - `design/surface-contract.md:167` gives the same rule, again with *untracked* in italics.
  - A file `git add`ed on an unborn `HEAD` has index bytes that differ from `HEAD`, and it is tracked, not *untracked*. So the general rule refuses it, and the exemption does not reach it. This is the record's own reading (b), and the record itself recommends "Refuse, as it does … product change: none".
- **It contradicts the table itself.** The sibling cell `setup-head-unborn-at-the-root-ignore-a-change-staged-and-not-in-the-tree` (`install.rs:918-923`: a staged root `.gitignore` on an unborn `HEAD`, with no edit) is listed `DIRTY` (`refuses(1, "setup.dirty-install-path")`), and the B5 cells TSV shows its refusal as the listed outcome. The new cell is the same staged file plus an edit, which makes it dirtier still, yet it is listed `Acts`.
- **The only loose sentence is a guide's.** `crates/cli/guides/QUICKSTART.md:148` says *"Every file is untracked there"* about a repository with no commit. That is wrong for a staged file. At most it is a doc-only inconsistency of the guide. It is not a silence of the design doc the row quotes.
- **Why it matters.** The "stricter reading" rule applies only where a doc is silent. Here it plants a red cell and a finding key whose fix direction, reading (a), would weaken a carryover-gate refusal.
- **Bound.** Instance. I enumerated the S9 unborn-`HEAD` cells (`grep '"\.gitignore"' install.rs`: 7 sites). Only this one is mislisted.
- **Smallest fix.**
  - List the cell `DIRTY` and drop `Mark::Silence(13)`; it then holds.
  - Remove the key from `EXPECTED_RED` and from findings §7.
  - Reword entry 13 as answered by the bytes rule (or delete it), and drop readings §12 and its marked-cells row.
  - Optionally record QUICKSTART's *"Every file is untracked there"* as a doc-only finding.

### F2 — should-fix: `r1-task-discard-settles-no-record-and-discards-a-sub-task-whose-record-is-not-on-disk` is real, but its class is wider than the one cell, and the record names only the directory-replaced shape

- **Location.** `crates/cli/tests/byte_table/task.rs` (`DISCARD_AT_THE_RECORD`, `task-discard-a-file-in-the-place-of-the-records-directory`). `byte-table-findings.md` §7, the first key.
- **Mechanism (traced).**
  - `crates/cli/src/milestone.rs:2761`: `let Ok(entries) = std::fs::read_dir(...) else { return Ok((None, Vec::new())) }`. A missing or non-directory record home becomes "no record names this task".
  - `:2776-2793`: a deleted record file is simply not in the listing, so the result is again `(None, [])`.
  - `:2668`: `None if unreadable.is_empty() => return Ok(None)`, and the discard proceeds as an ordinary task.
  - `HEAD` still names the sub-task. One caller (`grep -n "recording_milestone(" crates/cli/src/*.rs`: 1 call site, `:2665`).
- **Repro (ran).**

  ```
  rig=$(dev/jigc-rig --binary <worktree>/target/debug/jigc fresh) || exit; eval "$rig"
  $JIGC milestone create "Ship it"; $JIGC milestone add-task ship-it "do the thing"
  command rm docs/milestone-records/ship-it.md
  $JIGC task discard do-the-thing     # -> "discarded task do-the-thing", exit 0
  git show HEAD:docs/milestone-records/ship-it.md | grep do-the-thing   # still named
  ```

  So deleting the file alone, with the directory left in place (the common real shape), gives the same defect. The cell covers only the rarer "file in the directory's place" shape.
- **Keying.** The key's words (*"whose record is not on disk"*) are general enough. The listed refusal `refuses(1, "docs/milestone-records/ship-it.md")` is a substring check (`byte_cell.rs:1372-1375`) and is acceptable as "any refusal names the record".
- **Bound.** Class, by mechanism: every working-tree state in which the committed record file is absent at its canonical path (file deleted, directory deleted, directory replaced by a file), at the one consumer.
- **Smallest fix.** Add the deleted-file shape and the mechanism lines to the finding's evidence so the fixer fixes the class: consult `HEAD`'s record, or run the reconcile preflight, before trusting absence. A cell for "record file deleted" needs a plant kind the frame lacks (deletion), so record that as a lead.

### F3 — note: the `T2 × K` cell cannot reach `T2`, so the need is certified by a cell that never exercises its line

- **Location.** `task.rs` `FINALIZE_AT_THE_STAMP`, the last cell, `task-finalize-a-file-in-the-place-of-the-workbench`.
- **Claim.** The stamp's parent directory is `.jigc/`, which also holds the task's working area. Moving it away removes the task, so `task finalize` refuses `finalize.no-task` before any stamp write. The cell answers `T2 × K, a file in a directory's place` in `unmet`, but it tests "no task", not what `T2` does with a file in its directory's place. This is the same unreachability the scheduled list already states for `S2 × K` and `S9 × K` (*"the line's file stands at the repository's root, whose place no file can take"*).
- **Smallest fix.** Either answer the need with a scheduled row that says why it cannot be reached, or keep the cell and state in its doc comment and in findings §7 that it does not reach the line.

### F4 — note (future-proofing): `InPlaceOf(File)` at a tracked path is a whole rewrite of a tracked file (`git status`: modified), yet it counts as kind **U** ("an untracked file")

- **Location.** `byte_cell.rs:1740-1742` (`is_of` for `InPlaceOf`) and `:1991-2008` (`put`).
- **Claim.** `put` removes the file from the working tree only. The index entry stays, so the result is an edit of a tracked file. It is not an untracked one. At `T2` this nearly duplicates `task-finalize-a-hand-edit-to-the-version-stamp` (E). In B6 to B9, where every doc's home is tracked, each **U** need at a home will be answered by a whole-rewrite edit, while the kind's name says "untracked".
- **Smallest fix.** Decide which the plan means. Either say in the `InPlaceOf` doc comment and the plan that **U** in a tracked file's place is a whole rewrite, or build a true untracked file there (`git rm --cached` first) when the inner plant is `File`.

### F5 — note: the fences hold the new kind's positive path but not its exclusions, and the fault messages are not distinctive

- **Location.** `byte_table.rs:4883-4917`, fault cells. `byte_cell.rs:2002`, `2030`.
- **Exclusions not fenced.** No test asserts that `InPlaceOf(File)` aimed at `Kind::FileForADir` or `Kind::Foreign` is a fault in `answers` (`byte_table.rs:580-589`). `grep InPlaceOf byte_table.rs` finds only the stand-in and fault cells. A regression that dropped the `!matches!(kind, Kind::Foreign | Kind::FileForADir)` guard would pass.
- **Messages not distinctive.** Two of the three fault cells expect `"no regular file stands there"`, a substring that `Plant::Edit`'s fault (`byte_cell.rs:2030`) also prints.
- **Red first.** The builder never saw the three fault cells red on their own. Reasoning against the stub the builder describes (the inner plant put with nothing taken away), each would have been red:
  - `no-such-file.md`: the inner `File` put succeeds, so there is no fault.
  - `a-dir`: the inner `Dir` put faults with "something stands there already", which is the wrong message.
  - `Edit(README)`: the inner `Edit` succeeds, so there is no fault.

  So no reachable fault path is left unproven. Only the `remove_file` I/O error branch is unexercised, and nothing reaches it deterministically.
- **Smallest fix.** Match `"a plant goes in the place of"` in the two cells, and add one `answers` assertion per exclusion.

### F6 — note: count slips in the records of commit 1

- **The 17-to-13 gap is explained incompletely.** `DECISIONS.md` (the `b7ef80e6` entry, *What moved*), `byte-table-acceptance-rows.md` §6 entry 1 (the *Ruled* bullet) and `byte-table-readings.md` §10 say the projected *seventeen* came from "entry 10's three". The B5 projection counted 14 (the hook included) + 3 = 17. The actual gap, 17 to 13, is **four**: entry 10's three plus `setup-a-hook-that-rejects`, which entry 11 moved.
- **Five cells were relisted, not four.** The `DECISIONS.md` *What.* paragraph says "four cells' listed outcome". The diff of `install.rs` relists **five**: three as `DIRTY`, one as `setup.secrets-gitignore`, and one as `setup.install-commit` (the hook). The commit message's "four … and one moves" reads correctly; the `DECISIONS` sentence and *Not established*'s "The four listed refusals" do not.
- **"Five" corrections, six listed.** `DECISIONS.md`: "the five corrections the rulings name — the sentences of entries 1, 3, 9, 10 and 12, and entry 11's" lists six.
- **Fix.** Correct the three sentences.

### F7 — note: ruling 1's wording is narrowed

The brief records it as *"`setup` refuses without `--force` when the bytes at a path are not jigc's own"*. `DECISIONS.md` and the records say *"where the bytes at a path **it replaces** are not jigc's own"*. That is faithful to the question entry 10 asked (S4, replaced paths), and merge-into paths are already refused by the bytes rule on a born `HEAD`. Still, it is a narrowing of the human's words. Either use the brief's words or say why the scope is "replaces".

### F8 — note: departure from `228f2eda`'s form, not called out

`228f2eda` recorded S12's doc sentence as its own §1 key, even though a product change follows it (`r1-setup-design-stages-a-failed-install-…`). This commit instead folds the doc halves of entries 10, 11, 12 and 3 into their §6 product keys. It gives §1 keys only to the doc-only halves (entries 1 and 9), and states this as a new rule in §1's preface. That is consistent with the rulings ("one finding"). But `DECISIONS.md` presents the routing as following `228f2eda`, which it does only for the doc-only halves. Say "follows the rulings' *one finding*, and departs from S12's form" in one line.

### F9 — note: a stale dated record

`byte-table-install-family.md` §3 still describes entries 1, 3 and 9 to 12 in their pre-ruling state, e.g. *"Entry 11 — `setup`'s install commit runs no hook: no cell red, none marked"*. It is dated 2026-10-15, so this is history, but there is no pointer to the rulings. Optional: add a one-line "ruled 2026-10-15, see DECISIONS 2026-10-18".

## Checked and sound

- Every ruling is in `DECISIONS.md` at the top (newest first), with the human's quotes. Entry 11's record is corrected: the *"as it does"* sentence is struck, and the ruled bullet says so. No ruling is contradicted.
- The doc corrections are routed as findings, with no design doc touched in this lane, matching `228f2eda`.
- `NO_CACHE`: excluded from `May::Rebuilt` only (`byte_cell.rs:762-766`). No `WriteWhole` line in any row sits under `.jigc/state`, so the footprint record is red wherever it appears.
- `May::LaidOut` covers directories only, at both the file layer and the commit layer (`:781`, `:814`).
- `InPlaceOf` semantics:
  - The `place()` whitelist is exactly the entry-making plants; nested `InPlaceOf` faults.
  - A file taken away that was itself a plant is caught by `compare`'s "does not stand there" precondition (`:872-884`), so the doc comment's "never a plant" holds.
  - The removed file is absent from `before`, so the comparison never sees it.
- Usable as it is for doc homes, the milestone record and a rename source. Limits: regular files only, repository paths only, and F4.
- Of the 22 cells, the task-family listings follow `T2`/`A5`/`A8`. The setup `E` listings follow the bytes rule and `S8`, except F1's cell.
- The chosen refusal codes exist in the product (`setup.rs:4166`, `:2260`), and each is the nearest state's.
- The scheduled list drops exactly the 22 lines. The remaining five have reasons of their own.
- **Hygiene.** `git diff 56839f95..f4c12fa3` has no host path, person's name, or session or agent id. The commit messages carry the `Claude-Session` trailer the brief mandates.

## Not checked

- Anything on the previous release (`1.0.0-rc.24`): no binary of it was run. The ten "red there and not here" claims and the "does not build there" claim are unverified.
- The `committed_singletons` arm, the arm on a named binary, the full gate, and Linux.
- The per-cell seconds and the numbers in the commit messages beyond 196/160/36 (fenced) and 41 cells under fourteen keys (arithmetic only).
- That the previous release's `link to nothing` comes out *git's state moved*.
- Per-cell verdicts beyond what the green `fresh`/`bare` arms imply (that the expected-red list is exact).

<!-- end of report -->
