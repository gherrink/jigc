# The byte table's pilot — the questions of reading, and the cells each one decides

*Begun 2026-10-13 on the branch `fix/rc24-route-catalog`, with the task that brought the table its first cells (`B2d` of [the plan](byte-table-plan.md)). In the form of [the route catalog's list](route-catalog-doubts.md): the words, what a committed doc answers, what stays open, and the cells that turn on it. It decides nothing. A cell that turns on an open question follows the reading under which more is red, and says so.*

Two lists feed it: the plan's four questions ([byte-table-plan.md](byte-table-plan.md) → §7) and the six places where a design doc is silent or says two things ([byte-table-acceptance-rows.md](byte-table-acceptance-rows.md) → §6). **Of the twelve cells of the acceptance, none turns on an open question** — each is named below under the question it comes nearest, with what it showed on this tree's binary on 2026-10-13.

## 1. *Bytes no git object holds* — how far *holds* reaches (the plan's question 1) — open

- *The words.* The exit rule's first clause: *"no jigc command at exit 0 destroys bytes no git object holds"* ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-04 — The exit rule, revised*).
- *The readings.* (a) reachable — from a ref, a registered checkout's `HEAD`, an index, the newest stash entry; (b) that, and what any reflog still names; (c) in the object store, reachable or not.
- *What the frame does.* It follows (a), and a verdict says where it would be another under (c): the bytes are in the store and nothing reaches them. (b) is not carried apart from (c).
- *The cells.* None of the twelve: no verdict hung on it on this tree's binary or on the previous release's. The cell of the user's own worktree moved away is red on the previous release by the worktree's registration, whichever reading holds. The cells the plan expects to turn on it — a staged change, plant **S** — are the task `B4`'s.

## 2. A refusal that left the tree changed (the plan's question 2; the record's entry 1) — ruled for the caches, open beside them

- *Ruled* on 2026-10-12: a refusal may rebuild a cache ([DECISIONS.md](../../../../../DECISIONS.md) → *A refusal may rebuild a cache*; the record's §4a, lines `C1` to `C3`).
- *What the cells showed.* The three refusals of `jigc task finalize` each left the edge index rebuilt and nothing else: one file under `.jigc/index/` in the two cells of the hand edit, three in the cell of the link. The two refusals of `jigc milestone finalize` left `merged/` rebuilt — four differences each, under the line whose own sentence says so (`M1`). The six refusals of `jigc setup` and `jigc uninstall` changed nothing at all.
- *Open, and met by no cell.* **No refusal of the twelve wrote under `.jigc/state/`**, so the point the ruling leaves open — the file↔state hashes are called rebuildable and have no rebuild — is not met. **No refusal left a task area or a milestone area behind**, so nothing here is red until ruled. And whether a cache a refusal wrote is true is not held by the frame at all.

## 3. `jigc setup` refuses in two ways (the record's entry 2) — open

- *The cells.* None turns on it: the two cells of `setup` refuse before the first write — *nothing was installed and no install commit was made* — and the frame finds no difference. The cells it decides are a rejecting hook at the install's commit, plant **H**, in the family's own task.

## 4. `jigc uninstall`, git's index, and a commit (the record's entry 5) — open

- *The cells.* None turns on it on this tree: all four cells of `uninstall` refuse, and remove nothing. On the previous release three of them act, and are *lost* by the plant's rule before any line is asked. The cells it decides are the family's base cell and every planted cell in which `uninstall` acts.

## 5. Where `setup` keeps the record of a failed run · the spawn launch-template · the rendered message's temporary file (the record's entries 3, 4 and 6) — open, and met by no cell

- No cell wrote a file no line covers, on either binary's refusals or on this tree's one acting cell. **Nothing was left under the sandbox's `tmp/`** by any of the twelve — every child of a cell has it as `TMPDIR`, and the table reads it — so the rendered message's temporary file, wherever it stood, was gone by the time the command ended.

## 6. A hand edit that rides jigc's commit, and an index entry changed at a path the command does not name (the plan's questions 3 and 4) — answered per command, open at three

- Nothing of the twelve bears on the three the plan leaves open (`migrate-corpus` at a path it touches, `relocate`, and what was then unread of `milestone finalize`, which the record's §4 has since read).
