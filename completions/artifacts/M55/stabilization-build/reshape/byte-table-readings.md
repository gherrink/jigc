# The byte table's pilot — the questions of reading, and the cells each one decides

*Begun 2026-10-13 on the branch `fix/rc24-route-catalog`, with the task that brought the table its first cells (`B2d` of [the plan](byte-table-plan.md)); rewritten the same day with the cells of the task family (`B4`), the first that turn on a question. In the form of [the route catalog's list](route-catalog-doubts.md): the words, what a committed doc answers, what stays open, and the cells that turn on it. It decides nothing. A cell that turns on an open question follows the reading under which more is red, **and carries a mark that names the question** — the last section's table, which the suite holds to the cells (`byte_table::a_mark_names_a_question_the_list_of_readings_has`). A ruling changes a mark and this list, and never a cell's construction.*

Two lists feed it: the plan's four questions ([byte-table-plan.md](byte-table-plan.md) → §7) and the eight places where a design doc is silent or says two things ([byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §6). **Of the 61 cells, five turn on an open question or stand on a declared bound**; each is named below with what it showed on this tree's binary and on the previous release's on 2026-10-13.

## 1. *Bytes no git object holds* — how far *holds* reaches (the plan's question 1) — open

- *The words.* The exit rule's first clause: *"no jigc command at exit 0 destroys bytes no git object holds"* ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-04 — The exit rule, revised*).
- *The readings.* (a) reachable — from a ref, a registered checkout's `HEAD`, an index, the newest stash entry; (b) that, and what any reflog still names; (c) in the object store, reachable or not.
- *What the frame does.* It follows (a), and a verdict says where it would be another under (c): the bytes are in the store and nothing reaches them. (b) is not carried apart from (c).
- *The cells.* **Three, and all three are the same shape**: a change the user staged at a path the command stages itself, and did not keep in the working file — at a doc a task is updating, at `.jigc/version`, and at the milestone's record. In each the command exits 0 and the staged bytes are in the index no longer. **Under (a) each is *lost*. Under (b) each is *lost* too**: a blob that stood only in an index is named by no reflog. **Under (c) none is**: `git add` wrote the blob, and it is in the store until git collects it. All three come out the same on the previous release.
- No other verdict of the 61 hangs on it, on either binary. The cell of the user's own worktree moved away is red on the previous release by the worktree's registration, whichever reading holds.

## 2. A refusal that left the tree changed (the plan's question 2; the record's entry 1) — ruled for the caches, open beside them

- *Ruled* on 2026-10-12: a refusal may rebuild a cache ([DECISIONS.md](../../../../../DECISIONS.md) → *A refusal may rebuild a cache*; the record's §4a, lines `C1` to `C3`).
- *What the cells showed.* Of the family's seventeen refusals on this tree, **eight left the edge index rebuilt and nothing else** (`C1`, three files each) — every refusal of a finalize that got as far as its validation, and the one a hook rejected; **eight changed nothing at all** — a task that is not there, at both commands; a discard refused over a staged doc, over a file jigc did not write, twice, and over a hand edit to the record; and the two cells that run a refused finalize a second time; **one left a file no line covers** — §6.
- *The ruling's second bound is held in two cells*: *a refused command run again refuses again, with the plant standing*. `task-finalize-an-untracked-file-at-the-created-docs-home-run-again` and `task-finalize-a-hand-edit-after-the-first-write-run-again` run their command once as a lead that must refuse and then as the cell's command: each refuses again with the refusal it names, the plant stands, and the second refusal changes nothing — the cache the first rebuilt is as the first left it.
- *Open, and met by no cell.* **No refusal of the 61 wrote under `.jigc/state/`**, so the point the ruling leaves open there is not met. **No refusal left a task area or a milestone area behind.** Whether a cache a refusal wrote is true is not held by the frame at all.

## 3. `jigc setup` refuses in two ways (the record's entry 2) — open, and no cell turns on it

- The two cells of `setup` refuse before the first write, and the frame finds no difference. The cells it decides are a rejecting hook at the install's commit — `S11 × H` in the fence's words — in the family's own task.

## 4. `jigc uninstall`, git's index, and a commit (the record's entry 5) — open, and no cell turns on it

- All four cells of `uninstall` refuse on this tree, and remove nothing. On the previous release three of them act, and are *lost* by the plant's rule before any line is asked. The cells it decides are the family's base cell and every planted cell in which `uninstall` acts.

## 5. Where `setup` keeps the record of a failed run · the spawn launch-template (the record's entries 3 and 4) — open, and met by no cell

## 6. The rendered message's temporary file (the record's entry 6) — open, and met by one cell

- *The cell.* `task-discard-a-hook-that-rejects`: the record commit of a sub-task's discard is rejected by the user's `pre-commit` hook; the command exits 1 and says *nothing was committed … the task's working area is intact*; and `record-commit-msg.txt` stands in that working area, where it did not before. *Refused, and changed what a refusal may not* — on this tree and on the previous release.
- *The readings* are the record's: (a) the file is a write a refusal may not make; (b) it is part of *the survivable frame on a rejecting hook*, which a doc names and none describes. The cell follows (a).
- *Its sibling shows nothing.* `task-finalize-a-hook-that-rejects` leaves no file: only the edge index, rebuilt. **Nothing was left under the sandbox's `tmp/`** by any of the 61 cells on either binary.

## 7. `jigc task discard`, and the edge index when it settles a sub-task (the record's entry 7) — open, and met by no cell

- Five cells of the gate settle a sub-task at exit 0 on this tree, and five more on demand. None wrote under `.jigc/index/`: each wrote the record and under `.jigc/state/`, and took the working area away. So the cells do not turn on the entry, and no cell carries its mark.

## 8. A change staged at a path the command stages itself, and not in the working file (the record's entry 8) — open, and met by three cells

- *The cells.* `task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file` — the declared bound's own case; `task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file`; `task-discard-a-change-staged-at-the-record-and-not-in-the-file`. Each is *lost*, and hangs on §1.
- *Under reading (a)* — the bound is the rule wherever jigc stages a path of its own — all three stand on a declared bound. *Under (b)* — the bound is one case — the first stands on it and the other two are findings. **All three are red under either**: a bound makes no cell green.
- *What a neighbour shows.* A file staged at a **created** doc's home, and not in the tree, is refused on this tree (`finalize.promote-clobber`) where the previous release replaced it at exit 0. A change staged **before** the task existed is refused on this tree (`finalize.carried-staged`, seen by hand and no cell: its position is not the plan's).

## 9. A hand edit that rides jigc's commit, and an index entry changed at a path the command does not name (the plan's questions 3 and 4) — answered per command, open at three

- Nothing of the 61 bears on the three the plan leaves open (`migrate-corpus` at a path it touches, `relocate`, and what was then unread of `milestone finalize`, which the record's §4 has since read). **What the family's cells show of the answered ones**: at `task finalize` a file staged elsewhere rides the commit, a name git quotes included, as `T4` declares; at a sub-task's `task discard` the same two stay staged and out of the record's commit, as `A6` declares.

## 10. The declared bounds the cells stand on

- **Bound (1)** — *content staged for a doc a task is updating, while the working file does not carry it, is replaced at finalize*: one cell, §8's first. The same on the previous release.
- **Bound (2)** — *an uncommitted edit made before a task first writes a doc is carried into the task, and a write by that task to the same part replaces it*: `task-finalize-a-hand-edit-before-the-first-write-to-the-same-part` — exit 0, and the line added by hand stands at no path and in no object. **The previous release does not show it**: there the finalize refuses (`reconciliation.conflict-block`, exit 3) and the line stands. The cell beside it, in which the task writes another part, carries the line into the commit on this tree, and is refused the same way on the previous release.
- The human's entries for the four bounds are still owed as one list with reach ([decisions-pending.md](../../../../../implementation/decisions-pending.md) → *The declared bounds, as one list with reach*); a mark names a bound by its number in the plan's §1.1.

## The marked cells

A line a mark of a cell, as the suite prints it. A cell with two marks has two lines.

| mark | cell |
|---|---|
| what *held* means | `task-discard-a-change-staged-at-the-record-and-not-in-the-file` |
| what *held* means | `task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file` |
| what *held* means | `task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file` |
| silence 6 | `task-discard-a-hook-that-rejects` |
| silence 8 | `task-discard-a-change-staged-at-the-record-and-not-in-the-file` |
| silence 8 | `task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file` |
| bound 1 | `task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file` |
| bound 2 | `task-finalize-a-hand-edit-before-the-first-write-to-the-same-part` |
