# The route catalog's pilot — its numbers

*Written 2026-10-11 with the plan's step `P5` ([plan.md](plan.md) → Stream P), on the branch `fix/rc24-route-catalog`. A record: it counts what the pilot built and what it cost, and it **decides nothing** — not whether another family is catalogued ([decisions-pending.md](../../../../../implementation/decisions-pending.md) → `wf-route-catalog`, the human's, on these numbers), not what *works as printed* means for a route that prints no operand (§6), and not what a difference between two releases is (§8). What the mechanism is stands in [pinning.md](../../../../../implementation/pinning.md) → §6; why each piece is as it is stands in [DECISIONS.md](../../../../../DECISIONS.md), under the entries named *The route catalog's …*.*

**[2026-10-12 — the two tables committed beside this file were printed again.** [route-catalog-pilot-cases.tsv](route-catalog-pilot-cases.tsv) and [route-catalog-pilot-sites.tsv](route-catalog-pilot-sites.tsv) are the arm's blocks `cases` and `sites` as one run printed them on 2026-10-12, byte for byte, over the cases, the readings and the frame of the commit that carries this line — the run was made before that commit's last edits, which are to prose (the variable naming `rc.24`'s binary of §8; 274.1 s; no gate beside it): 415 lines of cases and 68 of sites under a header each. They hold what the arm prints now — the six cases of §7's first question as they were followed again, **and the cases and readings of a third file, the gate's** (`route_catalog/task_finalize.rs`, 62 cases over 12 sites with a reading and 2 with a reason; [DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-12 — Which families the route catalog takes in*, and the two entries above it). Every count of this record is `P5`'s, over the pilot's two commands, as counted then; where a cell of §3 quoted a line that moved, the cell is brought to the table and says so.**]**

**[2026-10-12, again — the two tables were printed with the cases of `jigc milestone`.** They are the arm's blocks `cases` and `sites` as one run printed them on 2026-10-12, byte for byte, over the cases, the readings and the reasons of the commit that carries this line (the variable naming `rc.24`'s binary of §8; 303.9 s, load average 7.0 at its start and 13.3 at its end; no gate of this branch beside it) — **and a run after the last edit to anything the arm reads, a change to how the runner is handed what a refusal printed, printed both blocks byte for byte the same** (676.6 s, beside another lane's gate, load average 38): 544 lines of cases and 78 of sites under a header each. They hold the pilot's two commands, the gate's 107 cases over sixteen sites — fourteen with a reading, two with a reason — and `jigc milestone`'s 84 over eight ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-12 — The route catalog's eight sites of `milestone`*, and the entry below it). Every count of this record is still `P5`'s, over the pilot's two commands, as counted then.**]**

**[2026-10-12, a third time — the two tables were printed with every family the ruling takes in, and §14 counts the catalog beyond the pilot.** They are the arm's blocks `cases` and `sites` as one run printed them on 2026-10-12, byte for byte, over the cases, the readings and the reasons of the commit that carries this line, made after the last edit to anything the arm reads (the variable naming `rc.24`'s binary of §8; 467.8 s, load average 8.8 at its start and 38.7 at its end; no gate of this branch beside it): 627 lines of cases and 92 of sites under a header each. Every count of §1 to §13 is still `P5`'s, over the pilot's two commands, as counted then; what stands beyond them is §14's.**]**

**[2026-10-12 — the two tables were printed with the first ruling carried out, and §15 counts the catalog after it.** They are the arm's blocks `cases` and `sites` as one run printed them on 2026-10-12, byte for byte, over the cases, the readings and the frame of the commit that carries this line, made after the last edit to anything the arm reads (the variable naming `rc.24`'s binary of §8; 536.5 s, load average 68.7 at its start and 73.7 at its end; no gate of this branch beside it): 627 lines of cases and 92 of sites under a header each — **30 lines of cases and 6 of sites differ from the tables committed before, and no other**: the thirty cases the ruling on cited commands was carried out on, and the count of listed-red cases at the six sites they stand at. §14 is as it was written, a day earlier.**]**

**[2026-10-12 — the two tables were printed with the second ruling carried out, and §16 counts the catalog after it.** They are the arm's blocks `cases` and `sites` as one run printed them on 2026-10-12, byte for byte, over the cases, the readings and the frame of the commit that carries this line, made after the last edit to anything the arm reads (the variable naming `rc.24`'s binary of §8; 408.4 s, load average 14.0 at its start and 23.4 at its end; no gate of this branch beside it): 627 lines of cases and 92 of sites under a header each — **13 lines of cases and 4 of sites differ from the tables committed before, and no other**: the thirteen cases the ruling on a placeholder for a value of the reader's own was carried out on, and the four sites that count them listed red. The table's column `operand` has a third word since that commit, `chosen`, for a case that names such a value. Every count of §1 to §15 is as counted then; what stands after this ruling is §16's.**]**

**Where a number comes from.** Three sources, and every table says which.

- **(arm)** — the suite's one ignored arm, run by hand on this tree at the commit that carries this record:

  ```text
  JIGC_ROUTE_CATALOG_BINARY=<scratch>/rc24-tree.XhX18W/target/release/jigc \
      cargo test -p jigc --test g_migration route_catalog::the_cases_are_followed_on_the_binary_a_variable_names -- --ignored --exact --nocapture
  ```

  It prints five tab-separated blocks **[2026-10-11: six — the block `bounds` joined them, §13]**. Two are committed beside this file as printed, byte for byte: `cases` is [route-catalog-pilot-cases.tsv](route-catalog-pilot-cases.tsv) (353 lines under a header) and `sites` is [route-catalog-pilot-sites.tsv](route-catalog-pilot-sites.tsv) (54 lines under a header). A count over one of them is an `awk` over its columns, named where it is used.
- **(entry)** — an entry of `DECISIONS.md`, or a commit's message, of this stream; the commit is named.
- **(orchestrator)** — the usage table of §2, handed to this task by the session that launched the builders.

**The pilot in one table** (arm, `sites` and `cases`):

| | `jigc setup` | `jigc uninstall` | together |
|---|---|---|---|
| blocking refusal sites | 36 | 18 | 54 |
| — with a case | 32 | 18 | 50 |
| — with a reason and no case | 4 | 0 | 4 |
| causes counted, by reading each site's callees | 200 | 77 | 277 |
| — followed by at least one case | 147 | 61 | 208 |
| — given a reason | 53 | 16 | 69 |
| cases | 268 | 85 | 353 |
| — red on this tree's binary | 121 | 49 | 170 |
| — that hold | 147 | 36 | 183 |

A case is a cause followed one way out of its route, so there are more cases than causes: 353 over 208, about 1.7 a cause.

---

## 1. What stands in the tree

`wc -l` at `bdaca476`, the last commit of `P4`:

| file | lines | what it is |
|---|---|---|
| `crates/cli/tests/support/route_case.rs` | 1,577 | the frame: what a case is, the runner, the nine classifications |
| `crates/cli/tests/route_catalog.rs` | 3,698 | the expected-red list, the fence, and the arms that hold the frame to its rules |
| `crates/cli/tests/route_catalog/setup.rs` | 4,142 | 268 cases, 32 readings, 4 reasons |
| `crates/cli/tests/route_catalog/uninstall.rs` | 1,899 | 85 cases, 18 readings |
| `crates/cli/tests/support/refusal_sites.rs`, `refusal_site_list.rs`, `route_catalog/sites.tsv` | 734 · 727 · 267 | `S0`: the scan, its suite, and the committed list of 261 sites |

The two files of cases are 6,041 lines for 353 cases — 17.1 lines a case, the readings and the reasons included.

## 2. What it cost — the frame's fixed part, apart from the cost per case

### The orchestrating session's numbers, as given

**Whose they are, and what they are.** These are the orchestrating session's, not this task's: for each builder it launched, the wall seconds from launch to return, and that builder's **total subagent tokens as the harness reports them — not output tokens**. The plan asked for output tokens; the harness does not give them apart. The wall seconds include every full gate the task ran (two or three each, beside another lane's gate on a loaded machine) and its Linux runs.

| task | wall seconds | subagent tokens |
|---|---|---|
| `S0` | not recorded | not recorded |
| `P1` | 6,859 | 343 k |
| `P2` | 2,644 | 306 k |
| `P3` parts one and two (first twenty-two sites' worth; read the entries for the split) | 12,094 | 655 k |
| `P3` part three | 10,773 | 560 k |
| `P3` part four | 11,554 | 645 k |
| `P4` | 12,065 | 728 k |
| `P5` | added by the orchestrating session after this task | — |

**[2026-10-11, the orchestrating session's: `P5`'s own line is 9,508 wall seconds and 581 k subagent tokens.]**

**[2026-10-11, the orchestrating session's: the label *first twenty-two sites' worth* on the row for `P3` parts one and two was that session's mistake. The entries' split is right — parts one and two end at eleven sites and 138 cases — as the paragraph under the next table found from the commits' own times.]**

### What each task added, by its entries

The entries give each part's cases and sites; the split of `P3` is theirs (entry — the catalog's running total at the end of each commit).

| task — commits | sites answered | cases added | the catalog afterwards |
|---|---|---|---|
| `S0` — `b17aa2fe` | — the list of sites | — | — |
| `P1` — `6cdf47db` | 1 | 2 | 2 cases, 1 site |
| `P2` — `3b7d220c` | — the fence | 0 | 2 cases, 1 site; 53 sites owed |
| `P3` parts one and two — `14dbd326` `d4e79d55` `b5df7e23` | 10 new, and the first site followed in full | 136 | 138 cases, 11 sites |
| `P3` part three — `9a5507aa` `4baa8b38` `0ea1c6fd` | 10 (9 with a case, 1 with a reason) | 47 | 185 cases, 20 sites, 1 reason |
| `P3` part four — `cb6d9e91` `40fd02cd` | 15 (12 with a case, 3 with a reason) | 81 | 266 cases, 32 sites, 4 reasons |
| `P4` — `c2b2af2d` `31d0fdd4` `bdaca476` | 18, and 2 more cases of `setup` | 87 | 353 cases, 50 sites, 4 reasons |

**The label *first twenty-two sites' worth* is not a count the entries give.** By the entries, parts one and two end at eleven sites and 138 cases. The commits' own times agree with the split used here to within a quarter of an hour (`git log --format='%h %ad'`): from `P2`'s commit to `b5df7e23` is 3 h 10 min against the 12,094 s given (3 h 22 min); from there to `0ea1c6fd`, 3 h 12 min against 10,773 s (3 h 0 min); from there to `40fd02cd`, 3 h 12 min against 11,554 s (3 h 13 min). The divisions below use the entries' counts.

### The divisions

| | wall seconds | tokens | cases | sites | seconds a case | tokens a case | seconds a site | tokens a site |
|---|---|---|---|---|---|---|---|---|
| **the fixed part: `P1` + `P2`** | 6,859 + 2,644 = **9,503** | 343 k + 306 k = **649 k** | 2 | 1 | — | — | — | — |
| `P3` parts one and two | 12,094 | 655 k | 136 | 10 | 12,094 / 136 = 88.9 | 655 k / 136 = 4.82 k | 12,094 / 10 = 1,209.4 | 655 k / 10 = 65.5 k |
| `P3` part three | 10,773 | 560 k | 47 | 10 | 10,773 / 47 = 229.2 | 560 k / 47 = 11.91 k | 10,773 / 10 = 1,077.3 | 560 k / 10 = 56.0 k |
| `P3` part four | 11,554 | 645 k | 81 | 15 | 11,554 / 81 = 142.6 | 645 k / 81 = 7.96 k | 11,554 / 15 = 770.3 | 645 k / 15 = 43.0 k |
| **`P3`** | 12,094 + 10,773 + 11,554 = **34,421** | 655 k + 560 k + 645 k = **1,860 k** | 264 | 35 | 34,421 / 264 = **130.4** | 1,860 k / 264 = **7.05 k** | 34,421 / 35 = **983.5** | 1,860 k / 35 = **53.1 k** |
| **`P4`** | **12,065** | **728 k** | 87 | 18 | 12,065 / 87 = **138.7** | 728 k / 87 = **8.37 k** | 12,065 / 18 = **670.3** | 728 k / 18 = **40.4 k** |
| **`P3` + `P4`** | 34,421 + 12,065 = **46,486** | 1,860 k + 728 k = **2,588 k** | 351 | 53 | 46,486 / 351 = **132.4** | 2,588 k / 351 = **7.37 k** | 46,486 / 53 = **877.1** | 2,588 k / 53 = **48.8 k** |

In minutes: 132.4 s a case is 2.2 minutes, and 877.1 s a site is 14.6 minutes, of wall time.

**What these divisions are, and what they are not.**

- **The fixed part is a floor, not the frame's whole cost.** `S0` is not recorded. And the frame went on growing inside `P3` and `P4`, with the case that needed each piece: four of their eleven commits are the frame's and answer no site — `d4e79d55` (the runner's limit), `9a5507aa` (the pool), `4baa8b38` (a case stands where it says), `c2b2af2d` (one arm a command's file) — and the others carry acts, checks and steps beside their cases. The usage table cannot split a builder's seconds between the two; the entries give minutes for some of it (§10).
- **So the cost per case is a ceiling on what a case costs once the frame is there**, and a mixed one: it holds the frame's growth, every full gate the builder waited for (one entry has three of them at over thirty minutes each, beside another lane's), the Linux runs, and the red-first runs.
- **Seconds a case and seconds a site do not move together across the parts.** Part three took the most seconds a case (229.2), over 47 / 10 = 4.7 cases a site, and two of its three commits are the frame's; parts one and two took the fewest (88.9), over 136 / 10 = 13.6 cases a site — the second of them a table of ways over causes already read — and the most seconds a site (1,209.4).
- **Nothing here is a quiet machine's number.** Every builder ran beside another lane.

## 3. Sites, causes and their derivations

One line a site (arm, `sites`). *Causes* is a hand count: the ways through the site's callees to its constructor, by reading. **How each count was derived, in words, is the last column of [route-catalog-pilot-sites.tsv](route-catalog-pilot-sites.tsv)** — the `derivation` of the site's reading, as the command's file holds it beside the functions that were read (`READINGS` in `crates/cli/tests/route_catalog/setup.rs` and `uninstall.rs`); for a site with a reason and no case that column holds the reason. *A reason* is a cause that has no case, with why; *followed* is the rest. *Ways not followed* are ways out of the site's route that no case takes, each with why in the reading. *Minutes* are the clock's where an entry gives them per site (§10); *entry* is the commit whose entry holds the site's row:

**a** `14dbd326` and `b5df7e23` · **c** `0ea1c6fd` · **d** `cb6d9e91` · **e** `40fd02cd` · **f** `31d0fdd4` · **g** `bdaca476`.

| site — `file::function::code#ordinal` | causes | a reason | followed | ways not followed | cases | red | minutes | entry |
|---|---|---|---|---|---|---|---|---|
| `setup.rs::hook_link_refusal::setup.install-hook#1` | 9 | 0 | 9 | 1 | 28 | 21 | — | a |
| `setup.rs::replaced_path_refusal::setup.write-bootstrap#dynamic-1` | 5 | 0 | 5 | 1 | 9 | 0 | — | a |
| `setup.rs::replaced_path_refusal::setup.version-stamp#dynamic-1` | 4 | 1 | 3 | 1 | 5 | 0 | — | a |
| `setup.rs::replaced_path_refusal::setup.init-project-layer#dynamic-1` | 5 | 1 | 4 | 1 | 7 | 0 | — | a |
| `setup.rs::replaced_path_refusal::setup.compose-marker#dynamic-1` | 5 | 2 | 3 | 1 | 5 | 1 | — | a |
| `setup.rs::replaced_path_refusal::setup.inject-allowlist#dynamic-1` | 4 | 1 | 3 | 1 | 5 | 0 | — | a |
| `setup.rs::replaced_path_refusal::setup.write-guide#dynamic-1` | 6 | 2 | 4 | 1 | 7 | 1 | — | a |
| `setup.rs::merged_through_links::setup.inject-reference#dynamic-1` | 5 | 0 | 5 | 1 | 26 | 12 | — | a |
| `setup.rs::merged_through_links::setup.inject-allowlist#dynamic-1` | 8 | 0 | 8 | 0 | 32 | 12 | — | a |
| `setup.rs::merged_through_links::setup.secrets-gitignore#dynamic-1` | 4 | 0 | 4 | 2 | 12 | 8 | — | a |
| `setup.rs::merged_through_links::setup.init-project-layer#dynamic-1` | 2 | 1 | 1 | 0 | 2 | 0 | — | a |
| `locate.rs::locate_finding::setup.repo-root#dynamic-1` | 3 | 1 | 2 | 0 | 4 | 0 | 6 | c |
| `locate.rs::locate_finding::setup.repo-root#dynamic-2` | 1 | 0 | 1 | 0 | 1 | 1 | 2 | c |
| `setup.rs::run::setup.profile-load#1` | 4 | 1 | 3 | 0 | 3 | 3 | 5 | c |
| `setup.rs::install::setup.profile-incomplete#1` | 1 | 0 | 1 | 0 | 1 | 1 | 2 | c |
| `setup.rs::install::setup.spawn-template#1` | 7 | 0 | 7 | 0 | 7 | 7 | 6 | c |
| `setup.rs::install::setup.guide-target#1` | 5 | 0 | 5 | 0 | 5 | 5 | 4 | c |
| `setup.rs::finding::setup.install-commit#1` | 5 | 1 | 4 | 2 | 4 | 0 | 7 | c |
| `setup.rs::install::setup.inject-allowlist#1` | 9 | 0 | 9 | 2 | 17 | 1 | 10 | c |
| `setup.rs::write_install_span::setup.compose-marker#1` | 9 | 4 | 5 | 2 | 5 | 4 | 6 | c |
| `setup.rs::write_install_span::setup.write-bootstrap#1` | 6 | 2 | 4 | 1 | 4 | 3 | — | d |
| `setup.rs::write_install_span::setup.inject-reference#1` | 7 | 2 | 5 | 1 | 5 | 4 | — | d |
| `setup.rs::write_install_span::setup.init-project-layer#1` | 13 | 5 | 8 | 1 | 8 | 8 | — | d |
| `setup.rs::write_install_span::setup.version-stamp#1` | 5 | 3 | 2 | 1 | 2 | 1 | — | d |
| `setup.rs::write_install_span::setup.secrets-gitignore#1` | 7 | 2 | 5 | 1 | 5 | 4 | — | d |
| `setup.rs::write_install_span::setup.inject-allowlist#1` | 3 | 2 | 1 | 0 | 1 | 1 | — | d |
| `setup.rs::write_install_span::setup.inject-allowlist#2` | 9 | 5 | 4 | 1 | 4 | 3 | — | d |
| `setup.rs::write_install_span::setup.inject-allowlist#3` | 7 | 4 | 3 | 1 | 3 | 3 | — | d |
| `setup.rs::write_install_span::setup.write-guide#1` | 6 | 3 | 3 | 0 | 3 | 2 | — | e |
| `setup.rs::write_install_span::setup.install-hook#2` | 15 | 4 | 11 | 1 | 11 | 10 | — | e |
| `engine/finalize.rs::setup_dirty_install_finding::setup.dirty-install-path#dynamic-1` | 16 | 3 | 13 | 2 | 33 | 2 *(was 3)* | — | e |
| `engine/finalize.rs::setup_unasked_install_finding::setup.unverified-install-path#1` | 5 | 3 | 2 | 2 | 4 | 2 | — | e |
| `setup.rs::write_install_span::setup.inject-hook#1` | — | — | — | — | 0 | 0 | — | e · the site has a reason |
| `setup.rs::write_install_span::setup.inject-deny#1` | — | — | — | — | 0 | 0 | — | e · the site has a reason |
| `setup.rs::write_install_span::setup.install-hook#1` | — | — | — | — | 0 | 0 | — | e · the site has a reason |
| `setup.rs::commit_install::setup.install-commit#1` | — | — | — | — | 0 | 0 | 3 | c · the site has a reason |
| `locate.rs::locate_finding::uninstall.repo-root#dynamic-1` | 3 | 1 | 2 | 0 | 4 | 0 | 2 | f |
| `locate.rs::locate_finding::uninstall.repo-root#dynamic-2` | 1 | 0 | 1 | 0 | 1 | 1 | 1 | f |
| `setup.rs::run_uninstall::uninstall.profile-load#1` | 4 | 1 | 3 | 0 | 3 | 3 | 2 | f |
| `setup.rs::dirty_worktree_finding::uninstall.dirty-worktree#1` | 9 | 1 | 8 | 4 | 14 | 3 | 9 | f |
| `setup.rs::unverified_worktrees_finding::uninstall.dirty-worktree#1` | 2 | 1 | 1 | 0 | 3 | 2 | 3 | f |
| `setup.rs::foreign_bytes_finding::uninstall.foreign-bytes#1` | 5 | 0 | 5 | 2 | 6 | 0 | 3 | f |
| `setup.rs::unverified_foreign_finding::uninstall.foreign-bytes#1` | 2 | 0 | 2 | 0 | 4 | 4 | 3 | f |
| `setup.rs::staged_prose_finding::uninstall.staged-prose#1` | 2 | 1 | 1 | 0 | 3 | 0 *(was 2)* | 4 | f |
| `task.rs::unverified_prose_finding::uninstall.staged-prose#1` | 3 | 2 | 1 | 0 | 2 | 2 | 3 | f |
| `setup.rs::untracked_workbench_finding::uninstall.untracked-workbench-file#1` | 6 | 1 | 5 | 2 | 10 | 0 *(was 2)* | 5 | g |
| `setup.rs::unverified_workbench_finding::uninstall.untracked-workbench-file#1` | 5 | 2 | 3 | 0 | 6 | 3 | 4 | g |
| `setup.rs::uninstall::uninstall.remove-jigc#1` | 4 | 0 | 4 | 0 | 4 | 3 | 3 | g |
| `setup.rs::uninstall::uninstall.unwire-reference#1` | 8 | 2 | 6 | 0 | 6 | 6 | 4 | g |
| `setup.rs::uninstall::uninstall.remove-allowlist#1` | 7 | 1 | 6 | 1 | 6 | 6 | 3 | g |
| `setup.rs::uninstall::uninstall.remove-hook#1` | 2 | 1 | 1 | 0 | 1 | 1 | 2 | g |
| `setup.rs::uninstall::uninstall.remove-deny#1` | 2 | 1 | 1 | 0 | 1 | 1 | 2 | g |
| `setup.rs::uninstall::uninstall.remove-precommit#1` | 11 | 1 | 10 | 0 | 10 | 9 | 5 | g |
| `setup.rs::uninstall::uninstall.remove-guide#1` | 1 | 0 | 1 | 0 | 1 | 1 | 1 | g |
| **54 sites** | **277** | **69** | **208** | **37** | **353** | **165** *(was 170)* | | |

**[2026-10-12: three cells of the column *red* and its sum are the table's as printed again** — the five cases of §7's first question that hold since they were followed with their line filled: one of `setup_dirty_install_finding`, two of `staged_prose_finding`, two of `untracked_workbench_finding`. The sentences under the table, and every other section, count the 170 of `P5`.**]**

Paths are from `crates/cli/src/`, and `engine/` is `crates/engine/src/`. **Causes a site: 277 over the 50 sites with a reading, 5.5; the least is 1 and the most 16.** Cases a site: 353 over 50, 7.1; the least is 1 and the most 33.

**The entries' own site tables were joined by the arm, not by hand.** Each entry's *cases* and *red* were true at its commit; the ten sites of the first part went from 46 cases to 110 when their ways out were followed (`b5df7e23`), and the numbers above are the tree's at `bdaca476`. The causes of every entry's table add up to the arm's: 48 and 9 (**a**), 44 (**c**), 57 (**d**), 42 (**e**), 31 (**f**), 46 (**g**) — 277.

## 4. Reasons, by word

(arm, the block `reasons`.) A reason is one of three words by construction.

| what has the reason | `not-constructible` | `platform` | `no-route-printed` |
|---|---|---|---|
| a site | 4 | 0 | 0 |
| a cause | 68 | 1 | 0 |
| a way out of a route | 37 | 0 | 0 |

No site, cause or way has the reason `no-route-printed`: every one of the 346 cases whose refusing command ended printed a route. The one `platform` cause is `git commit` refused because git cannot make up an address — the mask forbids the invention outright (entry, `0ea1c6fd`).

## 5. Red cases, by classification and by key

(arm, `cases`: `awk -F'\t'` over the sixth column, *this tree's binary*, and the fifth, *key*.)

| what the case came to on this tree's binary | `setup` | `uninstall` | together |
|---|---|---|---|
| followed, holds | 128 | 24 | 152 |
| followed, holds, operand implied | 19 | 12 | 31 |
| **holds** | **147** | **36** | **183** |
| followed, the same refusal again | 68 | 30 | 98 |
| followed, another failure | 43 | 11 | 54 |
| the command did not end | 4 | 3 | 7 |
| followed, a printed line no shell can split | 2 | 5 | 7 |
| followed, a witness changed | 4 | 0 | 4 |
| not provoked · provoked, no route printed | 0 | 0 | 0 |
| **red** | **121** | **49** | **170** |

These are the gate's own: the two arms that follow the cases hold the expected-red list to exactly these 170, by case, key and classification.

**[2026-10-11, after the first doubt was read by the surface contract (§7, question 1): the list holds 165.** Five cases that were *a printed line no shell can split* hold — one of `setup`, *holds, operand implied*, and four of `uninstall`, *holds* — and a sixth, of `setup`, is *the same refusal again* under a key of its own. So on this tree's binary: 188 hold (156, and 32 with the operand implied), 99 are the same refusal again, 1 is a printed line no shell can split, and the other rows are as above; `setup` has 120 red cases of 268 and `uninstall` 45 of 85. The keys are 27 still: the pilot's `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` holds no case, `r1-setup-unborn-dirty-route-unstaging-line-refused-behind-the-installs-writes` is new and holds 1, and `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` holds 1 of its 5. The tables of this section are `P5`'s, as counted then.**]**

**The keys — 27, over the 170 red cases.** Seven are the small canary's ([canary-one/findings.md](../canary-one/findings.md)), 78 cases; twenty were minted by the pilot ([route-catalog-findings.md](route-catalog-findings.md)), 92 cases. **No ledger holds any of them**: until a stabilization run is opened a key certifies itself.

| the canary's key | cases | as |
|---|---|---|
| `r1-p3-setup-writability-route-printed-for-other-causes` | 35 | the same refusal again |
| `r1-p3-uninstall-writability-route-not-driven` | 20 | the same refusal again 19 · another failure 1 |
| `r1-setup-install-hook-route-other-causes-unexamined` | 12 | the same refusal again 9 · a witness changed 3 |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | 5 | the same refusal again 4 · a witness changed 1 |
| `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | 3 | the same refusal again |
| `r1-p3-uninstall-force-names-nothing-undecodable-listing` | 2 | the same refusal again |
| `r1-setup-installs-outside-work-tree` | 1 | the same refusal again |

| the pilot's key | cases | as | of them `doubt` |
|---|---|---|---|
| `r1-setup-untracked-link-repointed-draws-dirty-install-path` | 16 | another failure | 0 |
| `r1-setup-profile-clause-route-names-no-act` | 12 | the same refusal again 11 · another failure 1 | 1 |
| `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | 8 | another failure | 8 |
| `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | 8 | another failure | 0 |
| `r1-setup-profile-route-says-reinstall-under-adapters-dir` | 7 | the same refusal again | 0 |
| `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | 6 | another failure | 0 |
| `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | 5 | a printed line no shell can split | 5 |
| `r1-setup-fifo-at-install-path-holds-the-command` | 4 | the command did not end | 0 |
| `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | 4 | another failure | 0 |
| `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | 3 | another failure | 3 |
| `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable` | 3 | the same refusal again | 3 |
| `r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command` | 3 | the command did not end | 0 |
| `r1-setup-home-unset-route-says-run-from-inside-the-repository` | 2 | the same refusal again | 0 |
| `r1-setup-settings-record-refused-rerun-installs-without-the-record` | 2 | another failure | 0 |
| `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` | 2 | a printed line no shell can split | 2 |
| `r1-setup-unverified-route-opens-with-the-question-that-failed` | 2 | another failure | 2 |
| `r1-setup-workbench-made-writable-rerun-refuses-its-own-files` | 2 | another failure | 0 |
| `r1-setup-force-over-skip-worktree-path-cannot-commit` | 1 | another failure | 0 |
| `r1-uninstall-worktree-route-move-aside-over-unreadable-path` | 1 | the same refusal again | 0 |
| `r1-uninstall-worktrees-route-prints-git-worktree-remove-without-a-path` | 1 | the same refusal again | 1 |

**55 of the 170 stand under the canary's two writability keys** — a step that words every error of its callee as *ensure … is writable* over one entry it names, in `jigc setup` and in `jigc uninstall` (the first two rows of the canary's table). **Three of the pilot's keys, 21 cases, are about a state only a variable reaches** — `JIGC_ADAPTERS_DIR`, or `$HOME` taken away — and whether a route owes anything there is said nowhere. **[2026-10-11: ruled. A state only `JIGC_ADAPTERS_DIR` reaches is a declared bound — two of the three keys, 19 cases; `$HOME` taken away is none — the third, 2 cases (§13).]**

## 6. Routes that print no operand

(arm, `cases`: the third column, *operand*, is `implied` where an act of the case takes an object the route names and does not print — the marked form — or runs the refusing command again because the route prints no command line.)

| | `setup` | `uninstall` | together |
|---|---|---|---|
| cases with such an act | 51 | 25 | 76 |
| — **followed, holds, operand implied**: the natural act was done, and what must hold held | 19 | 12 | **31** |
| — followed, the same refusal again | 28 | 12 | 40 |
| — followed, another failure | 2 | 0 | 2 |
| — followed, a printed line no shell can split | 1 | 0 | 1 |
| — the command did not end, so no route was reached | 1 | 1 | 2 |

So of the 74 cases in which a route was reached that prints no operand for an act, **31 hold when the natural act is done** and 43 do not. **[2026-10-11: 32 and 42 — the one case of this table that was *a printed line no shell can split* holds, operand implied, once its line is filled (§7); the operand it implies is the path of a move, as before.]** Of the 43, 40 are *the same refusal again*, by key (`awk` over columns 3, 5 and 6): 18 under the two writability keys, where the route names one entry in words — *the repo's git hooks directory*, *the repository root* — and another cause refused; 18 under the two profile keys, where the route prints no command and the refusing command was run again; and 4 under three other keys.

**Whether *followed, holds, operand implied* is *works as printed* is not decided here.** It is the human's, and the number it is decided on is this one: **31 of the 183 cases that hold, hold with an operand the case had to supply.** The frame guards the form two ways — it is refused wherever the route prints a path, or prints a `jigc` line — so none of the 31 stands in for an operand the route did print.

## 7. The cases marked `doubt` — one list to rule from

(arm, the block `doubts`.) **25 cases rest on a reading of a route's prose that nobody has given.** Each follows the printed words most literally and is listed with what it came to; the suite decides none. They are six questions:

| # | the route says | reading (a) — followed by the marked cases | reading (b) | cases | on this tree, under (a) |
|---|---|---|---|---|---|
| 1 | a backticked line with a word in angle brackets — `` `git -C <repo> rm --cached -- <path>` ``, `` `jigc task discard <task-id> --force` ``, `` `jigc milestone discard <milestone-id> --force` ``, `` `git -C <repo> add -- <path>` `` | a command to run as printed, like every backticked line | a shape to fill in from the refusal's listing — then the route prints no line a reader can run as it stands, and the frame has no case: a filled-in line would be the case's own | 7 | a printed line no shell can split, 7 |
| 2 | *point it at a regular file inside this repository* · *point it at a script inside this repository* | any file, or script, in the checkout — one in no commit included | one the repository already holds committed | 11 | another failure, 11 |
| 3 | *make sure `.jigc/…` is readable* | the entry the route names, its owner's read bit | every directory under it, readable and searchable | 3 | the same refusal again, 3 |
| 4 | *run `git …` to see what git reports, resolve that, then re-run* | the printed `git` line is a step, held to exit 0 like every printed command | it is there to show git's words; the steps are *resolve that* and the re-run | 2 | another failure, 2 |
| 5 | *spawn template missing the required `jigc workflow … --task …` invocation* | the backticked `jigc` line is a command to run | it is the shape the template must hold, and the route prints nothing to run | 1 | another failure, 1 |
| 6 | *remove the worktrees yourself (`git worktree list`, then `git worktree remove`)* | two command lines to run as printed | the names of two git verbs, the second run once for each path the first lists | 1 | the same refusal again, 1 |

**[2026-10-11 — question 1 was not put to the human: the design of record decides it.** [surface-contract.md](../../../../../design/surface-contract.md) says of every backticked `git …` / `jigc …` span that *a `<placeholder>` the reader fills is legal* — reading (b). It asks for a concrete value only where a placeholder is derivable from a finding's own target on a mechanical route, which none of these lines is; and it does not say where a reader takes any other value from ([DECISIONS.md](../../../../../DECISIONS.md) → 2026-10-11, *The route catalog's first doubt, read by the surface contract*, has its lines). So the frame fills a placeholder with a value the refusal's own output lists, by index, and the seven cases were followed again:

| case | now |
|---|---|
| `setup-unborn-staged-file-at-an-install-path-unstaged-as-printed-and-moved` | followed, holds, operand implied |
| `setup-unborn-install-path-staged-and-deleted-unstaged-as-printed` | followed, the same refusal again — the filled line is refused by git behind the install's writes; a finding, under a key of its own, and no doubt |
| `uninstall-staged-doc-read-and-discarded-as-printed` | followed, holds |
| `uninstall-staged-doc-finalized-as-printed` | followed, holds — from the state the way is offered to: the doc complete, a change staged |
| `uninstall-untracked-workbench-file-added-as-printed` | followed, holds |
| `uninstall-workbench-files-one-ignored-moved-out-and-one-added-as-printed` | followed, holds |
| `uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed` | a printed line no shell can split, **still marked** |

**What is left of question 1 is one case and another question.** `jigc milestone discard <milestone-id> --force` stands over a refusal that lists the worktree's path and no milestone, so the line cannot be filled from what was printed. The contract is silent on where the id comes from: (a) from the refusal's own output, as every other value a route needs — then the route owes it and the case is red as it stands; (b) from somewhere else the reader knows of — then the frame has no case. **So 19 cases are marked, not 25**: 1, 11, 3, 2, 1 and 1 by question; the 14 of questions 1, 2 and 4 stand on no declared bound.**]**

**What reading (b) would come to, where a case shows it.** Question 2: the case beside each marked one points the link at a committed file or script and **holds** — 11 cases. Question 4: the two `…-resolved` cases **hold, operand implied**. Question 1: filled in by hand, both `uninstall` routes hold (entry, `31d0fdd4`); no case runs a filled-in line. Questions 3 and 6 were not driven under (b). Question 5 is red either way: under (b) it is one more *the same refusal again*.

**The cases**, by question (the first column of `doubts`):

1. `setup-unborn-staged-file-at-an-install-path-unstaged-as-printed-and-moved` · `setup-unborn-install-path-staged-and-deleted-unstaged-as-printed` · `uninstall-staged-doc-read-and-discarded-as-printed` · `uninstall-staged-doc-finalized-as-printed` · `uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed` · `uninstall-untracked-workbench-file-added-as-printed` · `uninstall-workbench-files-one-ignored-moved-out-and-one-added-as-printed`
2. `setup-host-link-<end>-committed-and-pointed-at-a-new-file` and `setup-settings-link-<end>-committed-and-pointed-at-a-new-file`, for each of the four ends `to-nothing`, `out-of-the-repository`, `to-a-directory`, `into-gits-directory` (8) · `setup-hook-link-to-nothing-pointed-at-a-new-script` · `setup-hooks-path-inside-link-to-nothing-pointed-at-a-new-script` · `setup-hooks-path-outside-link-to-nothing-pointed-at-a-new-script`
3. `uninstall-task-areas-unlistable-made-readable` · `uninstall-workbench-unsearchable-made-readable` · `uninstall-staged-docs-unsearchable-made-readable`
4. `setup-git-refuses-the-repository-asked-as-printed-and-resolved` · `setup-git-status-fails-asked-as-printed-and-resolved`
5. `setup-launch-template-without-the-invocation`
6. `uninstall-worktrees-root-unreadable-worktrees-removed-as-printed`

**[2026-10-11 — which of them stand on a declared bound (§13).** Five of the 25 do, and cannot block whichever way their question is read; they stay listed, marked and red.

| # | cases | inside a bound | which |
|---|---|---|---|
| 1 | 7 | 0 | — |
| 2 | 11 | 0 | — |
| 3 | 3 | 3 | a permission bit changed by hand — `.jigc/tasks` and `.jigc` of mode 000, an open task's `docs/` of mode 600 |
| 4 | 2 | 0 | — |
| 5 | 1 | 1 | a state only `JIGC_ADAPTERS_DIR` reaches |
| 6 | 1 | 1 | a permission bit changed by hand — `.jigc/worktrees` of mode 000 |

So the readings that can still decide whether a case blocks are those of questions 1, 2 and 4: 20 cases — 14, once question 1 was read by the surface contract (the note below).**]**

The two readings of each, in the case's own words, are the last column of the block the arm prints; the routes as printed are in [route-catalog-findings.md](route-catalog-findings.md) under each key.

## 8. The same cases on the previous release's binary

**The table is [route-catalog-pilot-cases.tsv](route-catalog-pilot-cases.tsv)** — the arm's `cases` block, as printed. A line is a case: its name, its site, `printed` or `implied`, `doubt` or `-`, the key it is listed under on this tree; then for this tree's binary and for the named one **what the case came to**, **how its refusing command ended**, and **what became of its witnesses**; and whether the two binaries printed **the same route**.

**This section counts and does not judge.** Whether a row that differs is a regression, an improvement, or neither is not said by the arm, by the suite or by this record.

**[2026-10-11: six lines of the committed table are not what this tree prints any more** — the six cases of §7's first question that were followed again with their line filled. The table and the counts of this section are `P5`'s, as printed then, and are left so. On `rc.24`, from one run of the arm on the tree of the commit that carries this note (270.0 s; load average 10.1 at its start, 4.4 at its end): the two discarded-and-finalized staged-doc cases and the added workbench file are *followed, holds* there as here, under the same route; the two-file case is *a printed line no shell can split* there, under another route; and the two cases of `setup` are *not followed* there — `rc.24`'s route under that code has `git stash -u` where the case fills a line, and the frame says so. Every other line of the run is the committed table's, byte for byte.**]**

### The two binaries

| | the named binary | this tree's binary |
|---|---|---|
| what it is | `jigc 1.0.0-rc.24`, built from its release commit | the binary cargo builds for the suite |
| commit | `91834b5e011de2c36e2be2b79e96c0b9f60a803c` (the tag `jigc-v1.0.0-rc.24`) | product source as at `bdaca476` — no commit of `P5` changes it |
| how built | `git archive` of that commit into `<scratch>/rc24-tree.XhX18W/src`, then `cargo build --release` there with `CARGO_TARGET_DIR=<scratch>/rc24-tree.XhX18W/target`; the archive's `Cargo.lock`, unchanged by the build; rustc 1.95.0, the toolchain both trees name | `cargo test`'s own build: the `dev` profile, **a debug build** |
| sha256 | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | `f1ec27375d148622694ab368f47c96fa27a8fc9ff3603ce076e9689465e54e3b`, this machine's build |
| `--version` | `jigc 1.0.0-rc.24` | `jigc 1.0.0-rc.24` — the branch has not moved the version, so **the version line does not tell the two apart**; the commit and the hash do |

Between the two commits the product source differs by `29 files changed, 18477 insertions(+), 1388 deletions(-)` (`git diff --stat 91834b5e bdaca476 -- crates/cli/src crates/engine/src`), over 169 commits.

**A release build beside a debug build — what that can explain, measured.** A debug build carries the route fences, which panic where a printed route breaks one of the surface laws; a release build does not have them. So the build profile alone *could* turn a row: a route the debug binary panics over and the release binary prints. **It turns none here.** The arm was run a second time with the variable naming **this tree's own release build** (`cargo build --release -p jigc`, sha256 `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc`): all 353 lines have the same outcome on both, in three runs — and in the two that print those columns, the refusing command ending the same way, the same witnesses and, in the 346 that print one, the same route. So no difference in the table below is the build profile's. What the profile does change is seconds, and no column is one.

### How to read a line — four things the columns say that a classification does not

1. **The states are the named binary's own.** A case's state is built by the binary that is then asked: where a state starts from an install, `rc.24` made that install.
2. ***Not provoked* is not *nothing happened*.** The frame reads it where the case's own code was not printed at a non-zero exit — ahead of a changed witness, because without a refusal there is no route to follow. On a binary that has no such refusal yet, that is two different things, and the column *its refusing command* says which: **`exit 0`**, the command went on and did its work, or **`exit 1`**, another refusal answered first. And the column *its witnesses* says whether a file outside the repository, or one the command had no business touching, is still what it was.
3. **A case's acts were written by reading this tree's route.** An act names a span by its index. Where the named binary prints another route under the same code, the same indices are other words: *followed, …* there is the case's acts done to that route, and **not that route followed as somebody read it**. The last column says where that is so — `not the same`.
4. **`not followed: …` is the frame's reason, in its words**: the case could not be run on that binary, and the line is kept.

### The count

(arm, the block `pairs`, and `awk -F'\t'` over columns 6, 9, 10, 11 and 12 of the table. The arm was run on `rc.24` four times, under load averages between 5 and 47: the count by pair was the same each time, the three runs that print them agree line by line on what a case came to, on the routes and on the witnesses, and the last two printed the committed table byte for byte.)

**Of 353 lines, 194 have the same classification on both binaries and 159 do not.**

| on this tree's binary | on `rc.24` | lines | the two routes | `rc.24`'s refusing command | `rc.24`'s witnesses |
|---|---|---|---|---|---|
| **the same on both — 194** | | | | | |
| followed, the same refusal again | the same | 83 | the same 82 · not the same 1 | exit 1 | as they were |
| followed, holds | the same | 62 | the same 51 · not the same 11 | exit 1 | as they were |
| followed, holds, operand implied | the same | 27 | the same 25 · not the same 2 | exit 1 | as they were |
| followed, another failure | the same | 10 | the same 10 | exit 1 | as they were |
| the command did not end | the same | 7 | none printed | did not end | as they were |
| followed, a printed line no shell can split | the same | 5 | the same 4 · not the same 1 | exit 1 | as they were |
| **not the same — 159** | | | | | |
| followed, holds | not provoked | 77 | — | exit 0: 45 · exit 1: 32 | changed 40 · as they were 37 |
| followed, another failure | not provoked | 38 | — | exit 0: 14 · exit 1: 24 | changed 14 · as they were 24 |
| followed, the same refusal again | not provoked | 9 | — | exit 0: 9 | changed 8 · as they were 1 |
| followed, the same refusal again | not followed | 6 | — | — | — |
| followed, a witness changed | not provoked | 4 | — | exit 0: 4 | changed 4 |
| followed, holds, operand implied | not provoked | 4 | — | exit 0: 3 · exit 1: 1 | as they were |
| followed, holds | not followed | 4 | — | — | — |
| followed, another failure | followed, holds | 4 | the same 2 · not the same 2 | exit 1 | as they were |
| followed, holds | followed, another failure | 3 | not the same 3 | exit 1 | as they were |
| followed, holds | followed, the same refusal again | 3 | not the same 3 | exit 1 | as they were |
| followed, holds | the command did not end | 3 | none printed | did not end | as they were |
| followed, another failure | not followed | 2 | — | — | — |
| followed, a printed line no shell can split | followed, the same refusal again | 2 | not the same 2 | exit 1 | as they were |

Read down the columns:

- **132 lines are *not provoked* on `rc.24`**: the state that draws the case's refusal on this tree does not draw it there. In **75** the command **exited 0**, and in **44** of those a witness changed. In **57** it exited 1 under another code — `setup.dirty-install-path` 35, `setup.install-commit` 18, `setup.install-hook` 2, `uninstall.remove-allowlist` 2 (counted from the arm's last block, what the runner did, which is not committed) — and in **22** of those a witness changed.
- **A witness changed on `rc.24` in 66 lines — every one of them *not provoked*** — and on this tree's binary in **4**, the four *a witness changed*.
- **Both binaries printed a route in 199 lines: the same one in 174, another in 25.** Of the 12 lines where both printed a route and the outcome differs, the route is the same in **2**.
- **149 of the 159 are cases of `jigc setup`** (of 268) and **10 are cases of `jigc uninstall`** (of 85).

### The lines `rc.24` could not be run on — 12, each kept

| the frame's words | lines | what it means |
|---|---|---|
| `an act names span 1, and the printed route has 1 spans` | 8 | `rc.24` prints, under the case's code, a route with one backticked span; the case's second act is the re-run the tree's route prints as a second span. The six cases of the hook linked to a directory outside or to `/dev/null`, and two of the seeded `.gitignore` linked to a directory |
| `an act names span 3, and the printed route has 3 spans` · `span 4 … 3 spans` · `span 6 … 4 spans` | 3 | the same, over the dirty gate's route and the fourth guard's: `rc.24`'s has fewer spans than the act the case ends on — `setup-uncommitted-work-of-every-kind-at-once`, `setup-unborn-staged-file-at-an-install-path-forced`, `uninstall-workbench-files-one-ignored-forced` |
| `git ["rm", "-q", ".claude/settings.json", ".jigc/settings-entries.json"]: exit 128` | 1 | a state `rc.24` cannot build: the case's state takes the settings record out of an install, and `rc.24`'s install writes no such file — `setup-settings-record-absent-workbench-read-only` |

Each is a reason of the frame's: a case is this tree's reading of this tree's route, and a state step names this tree's files.

### Eight lines, as the arm printed what the runner did

Chosen to show each kind of line once; not chosen as the most important, and not judged.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `setup-hook-link-out-to-a-script-removed` | `setup.install-hook` refused; the route names the script outside as *the link*; removed, the re-run refuses again — **a witness changed**: the script is gone | **not provoked**, exit 0: the install completes and says `pre-commit hook → <sandbox>/outside/pre-commit` — the witness's bytes differ |
| `setup-hook-link-to-nothing-removed` | `setup.install-hook` refused; the route names the link; removed, the re-run installs — **holds** | **not provoked**, exit 0: the install completes, and a file is created where the link led, outside the repository |
| `setup-workbench-a-link-replaced` | `setup.write-bootstrap` refused before any write, *replace the link at `.jigc` with a real directory* — **holds** | **not provoked**, exit 1 under `setup.install-commit` — *the jigc install files were written, but `git add` was rejected* — and two files were created outside, through the link |
| `setup-host-link-out-of-the-repository-removed` | `setup.inject-reference` refused, *remove the link at `CLAUDE.md`* — **holds** | **not provoked**, exit 1 under `setup.dirty-install-path` over `CLAUDE.md`; the witnesses as they were |
| `setup-bootstrap-file-a-special-file` | `setup.write-bootstrap` refused at once, *remove the special file* — **holds** | **the command did not end** within the standing 120 s |
| `uninstall-reference-host-read-only` | the route followed, the re-run exits 0, and the settings file still holds jigc's entries — **another failure** | the same route, the same acts, the same exits — and the settings file holds none: **holds** |
| `setup-version-stamp-a-directory-moved` | route: *move the directory at `.jigc/version` out of the way* — **holds** | under the same code, another route: *ensure `.jigc/` is writable*. The case's act is *move span 0 out of the way*, so `.jigc/` was moved; the re-run refuses under another code — **another failure**. A line of the third kind above |
| `setup-settings-record-absent-workbench-read-only` | **another failure** | **not followed** — the state's `git rm` names a file `rc.24` does not write |

## 9. The suite's seconds

**No number at the catalog's present size is a quiet machine's.** Another lane's builder ran on this machine through the whole stream. Two rows were taken with no gate beside them by their builders' word — 8.7 s at 48 cases, and 26.3 s at 266, handed on and not taken again; this task's three were taken with no gate process on the machine and a load average that was not low. Each number stands with the load it was taken under, as its source gives it.

**Alone** — `cargo test -p jigc --test g_migration route_catalog::`, every arm of the suite in one process:

| cases | seconds | under | source |
|---|---|---|---|
| 48 | 8.73 · 8.70 · 8.78 | no other gate running | entry, `14dbd326` |
| 138 | 33 · 64 | other lanes' runs, load average 60 and above | entry, `b5df7e23` |
| 138 | 35.3 · 32.2 | another lane's gate, load average 35 to 90 | entry, `9a5507aa` |
| 185 | 103 · 84 | this branch's own gate and another lane's, load average 48 | entry, `0ea1c6fd` |
| 185 | 31 | that gate over, load average falling from 46 to 20 | the same |
| 217 | 38 | another builder's gate beside it | entry, `cb6d9e91` |
| 266 | 85 | under load — the entry first said *alone* and was corrected | entry, `40fd02cd`, corrected in `c2b2af2d` |
| 266 | 26.3 | *alone*: that builder's own measurement afterwards, handed on and not taken again | entry, `c2b2af2d` |
| 268 | 132 · 140 | another lane's gate, load average 43 to 53 | entry, `c2b2af2d` |
| 308 | 59 | another lane's gate beside it | entry, `31d0fdd4` |
| 353 | 134 | another lane's gate, load average 44 to 53 | entry, `bdaca476` |
| 353 | 70.9 · 91.3 · 70.6 | **no gate process on the machine** (`pgrep`), load average 6.1, 14.5 and 23.8 at the start of each run — the runs' own load among it; what else the other lane ran is not known | this task, 37 arms, one of them ignored |

**Inside a gate, per arm** — nextest, a process a test; from the gate's own log (`test-seconds`):

| cases | the arm of `setup` | the arm of `uninstall` | under | source |
|---|---|---|---|---|
| 48 | 6.7 | — | another lane's regression set | message of `14dbd326` |
| 138 | 19.5 | — | other lanes' runs | message of `b5df7e23` |
| 138 | 58.5 · 21.8 | — | the gates of `9a5507aa` and `4baa8b38`, each beside another lane's | entry, `0ea1c6fd` |
| 217 | 38.6 | — | another builder's gate | message of `cb6d9e91` |
| 266 | 60.7 | — | | message of `40fd02cd` |
| 268 | 70.7 | — | load average 60 to 74 | message of `c2b2af2d` |
| 308 | 92.4 | 12.9 | both at once; load average 55 to 64 | message of `31d0fdd4` |
| 353 | 76.1 | 48.8 | both at once; load average 40 to 48 | message of `bdaca476` |
| 353 | 88.3 | 47.9 | **one behind the other**, the group of `baef2c20`; load average 43 to 70 | message of `baef2c20` |

The gate ends a test at 480 s. The arm of the gate of the commit that carries this record is in that commit's message: a record cannot hold the numbers of the gate that runs over it.

**On Linux** — `dev/runner-faithful --commit <sha> cargo test -p jigc --test g_migration route_catalog::`, linux/aarch64, 8 CPUs, `cargo test`:

| commit | cases | the line | under | source |
|---|---|---|---|---|
| `14dbd326` | 48 | `test result: FAILED. 17 passed; 1 failed; … finished in 15.81s` — the failure an arm of the frame, repaired in `d4e79d55` | another lane's regression set | entry, `d4e79d55` |
| `14dbd326` | 48 | `test result: ok. 18 passed; 0 failed; … finished in 14.91s` | the same | the same |
| a commit of 138 cases | 138 | 142 s and 120 s — the two lines are not on record, the seconds are | a loaded host | entry, `9a5507aa` |
| `cb6d9e91` | 217 | `test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 89 filtered out; finished in 10.34s` | not said | message of `40fd02cd` |

**The Linux lines of `0ea1c6fd`, `40fd02cd` and `bdaca476` are not in the repository**: each entry says its runs *follow this commit and are in the task's report*, and a report is not a committed file. The line for the commit that carries this record is in this task's return, for the same reason.

**The ignored arm**, on `rc.24`: 286.1 s (load average 16 at its start, 6 at its end), 284.9 s (6 and 6), 507.5 s (47 and 25) and 346.4 s (25 and 11) — `finished in …` of each run. It follows 353 cases twice, and waits the standing 120 s for each of the 10 cases whose command does not end on `rc.24`. On this tree's own release build, the control of §8: 272.5 s, 281.7 s and 343.9 s (load average 23 at the start of the last).

## 10. Minutes, where a task clocked them

(entry.) Minutes are a builder's clock from a site's first probe to its cases listed. Two tasks give them per site — the column *minutes* of §3 — and the others give totals.

| part | sites | what the entry gives |
|---|---|---|
| `P3` part one — `14dbd326` | 10 | *about an hour together, a third of it the frame's growth and ten minutes of it the held run* |
| `P3` part two — `b5df7e23` | the same 10, and the hook's site in full | no minutes |
| `P3` part three — `0ea1c6fd` | 10 | **per site: 51 minutes** — 6, 2, 5, 2, 6, 4, 3, 7, 10, 6 — and *about twenty more* for the frame's growth, not in the column |
| `P3` part four, first commit — `cb6d9e91` | 8 | *one pass of about thirty-five minutes* of probing *and one of about thirty* of writing |
| `P3` part four, second commit — `40fd02cd` | 7 | *about an hour of probing and writing together — the dirty gate about twenty-five minutes of it, the hook's write ten* |
| `P4`, second commit — `31d0fdd4` | 9 | **per site: 30 minutes** — 2, 1, 2, 9, 3, 3, 3, 4, 3 — the one pass of writing, about eight minutes, spread over them |
| `P4`, third commit — `bdaca476` | 9 | **per site: 29 minutes** — 5, 4, 3, 4, 3, 2, 2, 5, 1 |

**Where a site was clocked: 110 minutes over 28 sites, 3.9 minutes a site; the least 1 and the most 10.** No site came near the hour at which the plan stops the task.

**The clocked minutes are a small part of the wall seconds.** `P4`'s eighteen sites were clocked at 59 minutes — 3,540 s of the 12,065 s the orchestrating session gives the task, 3,540 / 12,065 = 29.3 %. Part three's ten at 51 minutes and about twenty for the frame — 4,260 s of 10,773 s, 39.5 %. The rest is the gates, the red-first runs, the Linux runs and waiting; no source splits it further.

## 11. Seen by hand, and no case

By pointer; none of it is restated here.

- [route-catalog-findings.md](route-catalog-findings.md) → §4, *Seen, and no case — nothing refuses, so there is no route to follow*: `jigc uninstall` writes through a link to a file outside the repository at exit 0; and a clean teardown leaves an emptied `SessionStart` list and an empty `deny` list in a settings file that had neither.
- The same file → §1, under `r1-setup-fifo-at-install-path-holds-the-command`, *Not under this key — a FIFO at an ignore file*: `jigc setup` waits without a limit for a `git status` that does not return.
- `DECISIONS.md` → *The route catalog's first ten sites of `setup`* → *Seen, and no case*.
- The ways out of a route that no case follows, each with why: 37, in the readings (§3, §4).

**This task adds nothing to the findings file.** It followed no case on this tree's binary that the gate's arms do not follow; a line of §8's table is a line of the table and no finding of its own.

## 12. What the pilot does not establish

- **Anything about another family.** The cost per case is of two commands whose states are a repository and an install. A command whose refusals need an open task, a milestone, a migrated corpus or a second pack has states the frame has no step for yet — and here the frame grew in six of the seven commits that added sites' cases.
- **What *works as printed* means** where the route prints no operand (§6), and **which reading the 25 doubts want** (§7).
- **That a difference between the two binaries is a regression, or is not.** §8 counts.
- **A quiet machine's seconds at 353 cases** (§9); and what the suite costs on the hosted runner, which runs `cargo test` on four cores.
- **The cost of the frame apart from the cases**, beyond a floor: `S0`'s usage is not recorded, and the frame's growth inside `P3` and `P4` is in their totals.
- **Output tokens.** The table of §2 is total subagent tokens.
- **That a key is a finding.** A key certifies itself until a run's ledger holds it; the join's fence (`J1`, not built yet) is to hold keys to the two findings files.
- **That a claim is true** — that a code is minted at the dynamic site a row claims it for is a person's reading (entry, `3b7d220c`).
- **A cause nobody constructed.** 69 causes have a reason and no case; a fence holds sites, never causes.
- **A root caller**, for whom no permission state is provoked; **the advisory sites** of `jigc setup`, which exit 0; and **a second platform's classifications** beyond what the Linux lines show — that the same list is exact there.
- **`rc.24` on any other state than the ones this tree's cases build.** The table says what `rc.24` did where this tree refuses; what `rc.24` refused that this tree does not is not in it.

## 13. 2026-10-11 — which states a route must work in: the ruling by pointer, and what can block

**The ruling is the human's, and its home is [DECISIONS.md](../../../../../DECISIONS.md)** → *2026-10-11 — Which states a route must work in* — it is not restated here. In one line: three kinds of planted state are declared bounds of the route catalog — a permission bit changed by hand, a FIFO or a device node planted at a path, a state only `JIGC_ADAPTERS_DIR` reaches; a red case whose refusal is over one is recorded, stays pinned red, and does not block; and one key is admitted by name, bound or not. **The count the ruling was put on** is [route-catalog-red-by-state.md](route-catalog-red-by-state.md), committed as it was handed over.

**How a case's bound is known.** It is read off the case's state — its steps and its environment, never its acts — by one function of the frame (`route_case::standing`, `bound_of`), and marked by hand nowhere it can be read. Six cases stand on a bound **and** on a state that is none — a link to a file outside that may not be written (2), an unborn `HEAD` beside a permission (3), a `core.hooksPath` value beside one (1); each carries one word for which of the two its refusal is over, with what that was read from, and in all six it is the permission: the same state without the bit was driven by hand and refuses nothing.

**The table** (arm, the block `bounds`, as printed — run on 2026-10-11 on the tree of the commit that carries this section, the variable naming `rc.24`'s binary of §8: 301.7 s, load average 4.6 at its start and 10.6 at its end. The same run printed the blocks `cases` and `sites` byte for byte as they are committed beside this file.) A line a key: the red cases listed under it, how many of them **can block** — their refusal is over no bound, or the key is admitted — and how many stand on each bound.

| key | red cases | can block | a permission bit changed by hand | a FIFO or a device node planted | a state only `JIGC_ADAPTERS_DIR` reaches | admitted |
|---|---|---|---|---|---|---|
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | 5 | 4 | 1 | 0 | 0 | - |
| `r1-p3-setup-writability-route-printed-for-other-causes` | 35 | 18 | 17 | 0 | 0 | - |
| `r1-p3-uninstall-force-names-nothing-undecodable-listing` | 2 | 2 | 0 | 0 | 0 | - |
| `r1-p3-uninstall-writability-route-not-driven` | 20 | 10 | 10 | 0 | 0 | - |
| `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | 3 | 3 | 0 | 0 | 0 | - |
| `r1-setup-fifo-at-install-path-holds-the-command` | 4 | 0 | 0 | 4 | 0 | - |
| `r1-setup-force-over-skip-worktree-path-cannot-commit` | 1 | 1 | 0 | 0 | 0 | - |
| `r1-setup-home-unset-route-says-run-from-inside-the-repository` | 2 | 2 | 0 | 0 | 0 | - |
| `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | 3 | 3 | 0 | 0 | 0 | - |
| `r1-setup-install-hook-route-other-causes-unexamined` | 12 | 12 | 0 | 0 | 0 | - |
| `r1-setup-installs-outside-work-tree` | 1 | 1 | 0 | 0 | 0 | - |
| `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | 8 | 8 | 0 | 0 | 0 | - |
| `r1-setup-profile-clause-route-names-no-act` | 12 | 0 | 0 | 0 | 12 | - |
| `r1-setup-profile-route-says-reinstall-under-adapters-dir` | 7 | 0 | 0 | 0 | 7 | - |
| `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | 8 | 8 | 0 | 0 | 0 | - |
| `r1-setup-settings-record-refused-rerun-installs-without-the-record` | 2 | 0 | 2 | 0 | 0 | - |
| `r1-setup-unborn-dirty-route-unstaging-line-is-no-command-line` | 2 | 2 | 0 | 0 | 0 | - |
| `r1-setup-untracked-link-repointed-draws-dirty-install-path` | 16 | 16 | 0 | 0 | 0 | - |
| `r1-setup-unverified-route-opens-with-the-question-that-failed` | 2 | 2 | 0 | 0 | 0 | - |
| `r1-setup-workbench-made-writable-rerun-refuses-its-own-files` | 2 | 0 | 2 | 0 | 0 | - |
| `r1-uninstall-could-not-look-route-names-the-walks-root-to-make-readable` | 3 | 0 | 3 | 0 | 0 | - |
| `r1-uninstall-fifo-at-a-file-the-teardown-reads-holds-the-command` | 3 | 0 | 0 | 3 | 0 | - |
| `r1-uninstall-force-over-unreadable-workbench-fails-part-way` | 6 | 0 | 6 | 0 | 0 | - |
| `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | 4 | 4 | 4 | 0 | 0 | admitted |
| `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` | 5 | 5 | 0 | 0 | 0 | - |
| `r1-uninstall-worktree-route-move-aside-over-unreadable-path` | 1 | 0 | 1 | 0 | 0 | - |
| `r1-uninstall-worktrees-route-prints-git-worktree-remove-without-a-path` | 1 | 0 | 1 | 0 | 0 | - |
| **every key (27), of which 17 can block** | **170** | **101** | **47** | **7** | **19** | **1** |
| **the cases no entry lists** | **183** | **-** | **12** | **4** | **0** | **-** |

Read across:

- **17 of the 27 keys keep a case that can block, and 101 of the 170 red cases can.** Sixteen keys by their states — 97 cases — and the admitted one, with its 4.
- **73 red cases stand on a bound**: 47 on a permission bit, 7 on a FIFO, 19 on the override — the count's own three numbers, which the function reaches by reading the states. 69 of them do not block; the 4 under the admitted key do.
- **Ten keys lie wholly inside the bounds and block nothing** — 41 cases; the eleventh the count names, the admitted key, is wholly inside them and blocks by name. Three keys hold both kinds: 28 bounded cases beside 32 that can block.
- **Of the 183 cases that hold, 16 stand on a bound**: 12 on a permission bit and 4 on a FIFO. No case that holds stands on the override.

**[2026-10-11, after the first doubt was read by the surface contract (§7): three lines of the table moved, and no bound's column.** As the arm printed them on the tree of the commit that carries this note:

| key | red cases | can block | a permission bit changed by hand | a FIFO or a device node planted | a state only `JIGC_ADAPTERS_DIR` reaches | admitted |
|---|---|---|---|---|---|---|
| `r1-setup-unborn-dirty-route-unstaging-line-refused-behind-the-installs-writes` — new, where `…-unstaging-line-is-no-command-line` stood with 2 and 2 | 1 | 1 | 0 | 0 | 0 | - |
| `r1-uninstall-routes-print-lines-with-a-placeholder-for-the-id` — was 5 and 5 | 1 | 1 | 0 | 0 | 0 | - |
| **every key (27), of which 17 can block** | **165** | **96** | **47** | **7** | **19** | **1** |
| **the cases no entry lists** | **188** | **-** | **12** | **4** | **0** | **-** |

None of the seven cases stands on a bound, so the five that hold now come off the cases that could block: **96 of 165 red cases can block, under 17 of 27 keys.]**

**What a bound does not do.** It takes no case off the expected-red list, moves no classification and makes no case green: the list is what it was at §5, strict three ways, and the 73 are in it. And the human may admit a bounded case by its key at any stop of a run; the table of admitted keys is one constant of the suite (`ADMITTED`).

**What this section does not establish.** That a case that *can* block is a finding a run would grade as blocking — a key certifies itself until a run's ledger holds it (§12). That the three bounds are rows of a run's declared-bounds list: no run is open, and the opening record takes them when one is. And anything about a state the function cannot read as a second one beside a bound — a file's bytes, a directory, what a `git add` leaves: the count's §1c and §4 name such conditions beside a permission — a `.jigc/` entry the state plants by hand under eleven of the permission cases, a file that must lack an entry under one, a refusing command that carries `--force` under one — read there by a person and here by nobody.

## 14. 2026-10-12 — the catalog beyond the pilot, as it stands

**What this section is.** The human's ruling of 2026-10-11 ([DECISIONS.md](../../../../../DECISIONS.md) → *Which families the route catalog takes in*) took in, beyond the pilot's two commands, the blocking sites the fix pass touched; six entries of 2026-10-12 above that one carry them out, family by family. This section counts what stands when the last of them lands. It judges nothing: not what a difference between two binaries is, and not which reading a marked case wants — those cases are one list, [route-catalog-doubts.md](route-catalog-doubts.md).

**Where a number comes from.** One run of the ignored arm (§ *Where a number comes from*, at the head), on the tree of the commit that carries this section, the variable naming `rc.24`'s binary of §8: **467.8 s over 627 cases**, load average 8.8 at its start and 38.7 at its end, no gate of this branch beside it. Each table names the arm's block it is read from; a count over a block is a sum over its columns. [route-catalog-pilot-cases.tsv](route-catalog-pilot-cases.tsv) and [route-catalog-pilot-sites.tsv](route-catalog-pilot-sites.tsv) are the blocks `cases` and `sites` of that same run, byte for byte — 627 lines of cases and 92 of sites under a header each; the 544 and 78 lines committed before are in them unchanged.

### Sites, causes, cases — per family (block `sites`)

| family — its file under `route_catalog/` | sites with a reading | sites with a reason | causes | followed | a reason | ways not followed | cases | red |
|---|---|---|---|---|---|---|---|---|
| `task finalize` — `task_finalize.rs` | 14 | 2 | 98 | 84 | 14 | 15 | 107 | 26 |
| `milestone` — `milestone.rs` | 8 | 0 | 79 | 69 | 10 | 10 | 84 | 32 |
| `doc write` — `doc_write.rs` | 3 | 0 | 21 | 16 | 5 | 3 | 18 | 10 |
| `config` — `config.rs` | 4 | 1 | 28 | 25 | 3 | 4 | 26 | 2 |
| `start` — `start.rs` | 1 | 0 | 3 | 3 | 0 | 1 | 3 | 0 |
| `doc read` — `doc_read.rs` | 1 | 0 | 32 | 25 | 7 | 2 | 30 | 6 |
| the dynamic sites no family has — `unassigned.rs` | 2 | 2 | 4 | 4 | 0 | 2 | 6 | 3 |
| **beyond the pilot** | **33** | **5** | **265** | **226** | **39** | **37** | **274** | **79** |
| the pilot's two commands, as the same run prints them | 50 | 4 | 277 | 208 | 69 | 37 | 353 | 165 |
| **the catalog** | **83** | **9** | **542** | **434** | **108** | **74** | **627** | **244** |

`migrate` has no site with a case: its two were read and left for the port. A site of `unassigned.rs` is a code claimed at a dynamic site — two of `jigc rename`'s with cases, one of `jigc relocate`'s and one of `jigc migrate-corpus`'s with a reason.

### Reasons, by word (block `reasons`)

| what has the reason | `not-constructible` | `platform` | `no-route-printed` |
|---|---|---|---|
| a site — the catalog · beyond the pilot | 9 · 5 | 0 | 0 |
| a cause — the catalog · beyond the pilot | 106 · 38 | 2 · 1 | 0 |
| a way out of a route — the catalog · beyond the pilot | 74 · 37 | 0 | 0 |

*Beyond the pilot* is the block less §4's numbers.

### The keys minted beyond the pilot — 29, over 79 red cases (block `cases`, columns 5 and 6)

| key | red cases | family |
|---|---|---|
| `r1-finalize-nothing-staged-route-prints-finalize-without-its-id` | 3 | task finalize |
| `r1-finalize-stage-failed-route-names-a-lock-and-three-paths-and-the-docs-home-is-ignored` | 1 | task finalize |
| `r1-finalize-amend-staged-doc-route-says-the-start-line-mints-and-it-presents-the-workflows` | 1 | task finalize |
| `r1-linked-doc-route-cites-a-command-that-is-itself-refused` | 2 | task finalize |
| `r1-conformance-block-in-a-migration-minted-over-the-edit-revert-draws-conflict-block` | 1 | task finalize |
| `r1-finalize-provenance-io-route-says-disk-or-permissions-over-a-manifest-of-another-kind` | 3 | task finalize |
| `r1-task-working-area-io-route-says-disk-or-permissions-over-an-entry-of-another-kind` | 3 | task finalize |
| `r1-doc-create-refused-over-the-manifest-stages-the-body-and-the-re-run-exits-0` | 2 | task finalize |
| `r1-clobber-task-and-migration-routes-print-lines-with-placeholders-the-refusal-lists-nothing-for` | 4 | task finalize |
| `r1-clobber-fixed-route-prints-git-mv-without-a-path` | 2 | task finalize |
| `r1-clobber-held-migration-route-review-step-exits-4` | 2 | task finalize |
| `r1-clobber-route-drops-the-milestones-only-sub-task-and-the-re-run-draws-zero-contribution` | 1 | task finalize |
| `r1-clobber-staged-by-two-worktrees-rename-way-draws-code-collision` | 1 | task finalize |
| `r1-record-exists-route-says-continue-over-a-file-that-is-no-record` | 1 | milestone |
| `r1-zero-contribution-route-prints-git-add-with-no-path-and-no-place` | 1 | milestone |
| `r1-zero-contribution-route-abandon-line-is-refused-by-the-discard-doors-guards` | 3 | milestone |
| `r1-milestone-area-io-route-says-disk-or-permissions-over-an-entry-of-another-kind` | 19 | milestone |
| `r1-worktree-doors-route-re-run-is-for-the-next-command` | 3 | milestone |
| `r1-leftover-consent-over-a-link-to-nothing-draws-provision-failed` | 1 | milestone |
| `r1-unlanded-route-keeps-the-only-commit-and-the-re-run-draws-zero-contribution` | 1 | milestone |
| `r1-unlanded-route-abandon-line-is-refused-by-the-discard-door` | 3 | milestone |
| `r1-record-retitle-route-names-two-milestone-verbs-without-their-ids` | 3 | doc write |
| `r1-distinct-identity-routes-print-a-line-with-placeholders-for-the-readers-own-choice` | 9 | doc read · doc write |
| `r1-repoint-over-a-stale-record-key-says-two-managed-docs-claim-the-home` | 2 | config |
| `r1-home-shape-edited-doc-route-says-copy-what-the-link-points-at-over-a-link-to-nothing` | 1 | doc read |
| `r1-home-shape-migration-route-review-step-exits-4` | 1 | doc read |
| `r1-home-shape-destination-routes-print-a-line-with-a-placeholder-for-another-slug` | 2 | doc read |
| `r1-rename-occupied-route-prints-a-line-with-a-placeholder-for-another-slug` | 2 | unassigned |
| `r1-rename-occupied-route-calls-a-file-in-no-commit-a-doc-and-its-read-is-refused` | 1 | unassigned |

With the pilot's 27 keys the expected-red list holds **56 keys over 244 cases** (block `bounds`, its last two lines: *every key (56), of which 46 can block — 244 red cases, 175 that can block, 47 on a permission bit, 7 on a FIFO, 19 on the override, 1 admitted*; *the cases no entry lists — 383*). No red case beyond the pilot stands on a declared bound.

### The two lists

The owed list is empty and gone: the suite has no table for it and the fence no answer of that kind. **The scheduled list holds 145 sites**, every one classed `unchanged` by the count but one that the count did not class; 28 of them carry the change of a caller that was read and why it is off the path, and 3 that they are answered in part.

### The same cases on the previous release's binary — totals per family (block `cases`, columns 6, 9 and 12)

| family | lines | the same classification on both — of them over the same printed route | not provoked on `rc.24` | other |
|---|---|---|---|---|
| `task finalize` | 107 | 47 — 41 | 55 | 5 |
| `milestone` | 84 | 59 — 55 | 25 | 0 |
| `doc write` | 18 | 18 — 18 | 0 | 0 |
| `config` | 26 | 23 — 23 | 2 | 1 |
| `start` | 3 | 3 — 3 | 0 | 0 |
| `doc read` | 30 | 0 | 30 | 0 |
| the dynamic sites | 6 | 6 — 6 | 0 | 0 |
| **beyond the pilot** | **274** | **156 — 146** | **112** | **6** |
| `setup` · `uninstall` | 268 · 85 | 119 — 102 · 74 — 70 | 125 · 7 | 24 · 4 |

*Other* is a line where `rc.24` came to another classification than this tree and to more than *not provoked* — or could not be followed there, the case's act naming a span that binary's route does not have. What a line says and does not say is §8's.

### The arms' seconds

Inside a gate, one behind the other (the gate of the commit before this section's, `tests passed=5058`, its log's `test-seconds`): `start` 1.2 · `config` 9.6 · `doc_write` 10.3 · `doc_read` 12.7 · `uninstall` 27.2 · `setup` 43.5 · `milestone` 51.3 · `task_finalize` 66.8 — 222.6 s together, at 617 cases. The gate before that one, under a heavier load: `doc_write` 25.3 · `uninstall` 29.4 · `setup` 71.7 · `task_finalize` 78.1 · `milestone` 111.2. The ignored arm, which follows every case twice: 1028.6 s at 617 cases under a load average of about 100, and 467.8 s at 627 under one of 9 to 39.

### Not established

- **Anything this section's tables do not count**: a cause nobody constructed, a way with a reason, and the 145 scheduled sites.
- **The frame over a route that removes the directory its reader stands in and then prints a command without a `cd`**: the runner still starts that command in a directory that is gone and panics. No case of the seven families met it — none stands in a directory its route removes — and it was not changed.
- **The clause the fix pass added to the re-point's route** (a file parked on the way): followed by no case; driven by hand ([route-catalog-findings.md](route-catalog-findings.md) → §8).
- **That a site left for the port was rightly left**: 28 rows say which change was read and why it is off the path; that is a reading.
- **Linux**: the run follows the commit and is in the task's return.

## 15. 2026-10-12 — a cited line held to its words: the catalog after the first ruling was carried out

**What this section is.** Of the nine questions the catalog's marked cases hang on ([route-catalog-doubts.md](route-catalog-doubts.md)), the human ruled the first on 2026-10-12: a line a route cites for what it prints is held to its words and not to exit 0 ([DECISIONS.md](../../../../../DECISIONS.md) → *A command a route cites for what it prints is held to its words*). The entry of that day gave the frame the act that carries it out; the one after it followed the thirty cases again. This section counts the catalog when that second entry lands. It judges nothing: not what a difference between two binaries is, and not whether a witness is the one its route makes — that is each case's reading, stated in the case.

**Where a number comes from.** One run of the ignored arm (§ *Where a number comes from*, at the head), on the tree of the commit that carries this section, the variable naming `rc.24`'s binary of §8: **536.5 s over 627 cases**, load average 68.7 at its start and 73.7 at its end, no gate of this branch beside it. [route-catalog-pilot-cases.tsv](route-catalog-pilot-cases.tsv) and [route-catalog-pilot-sites.tsv](route-catalog-pilot-sites.tsv) are the blocks `cases` and `sites` of that run, byte for byte. *Before* is §14, and where §14 has no such count, the same sum over the table committed with it.

### The catalog's totals (blocks `cases` and `bounds`)

| | before | after |
|---|---|---|
| cases | 627 | 627 |
| red — the cases the expected-red list holds | 244 | **231** |
| keys | 56 | **50** |
| red cases that can block · keys that can block | 175 · 46 | 162 · 40 |
| red cases on a declared bound — a permission bit · a FIFO · the override | 47 · 7 · 19 | 47 · 7 · 19 |
| the cases no entry lists | 383 | 396 |
| cases marked `doubt` — of them red | 76 — 59 | 46 — 46 |
| **red cases at a refusal `rc.24` does not provoke** — listed red here, *not provoked* there | 70 | **58** |

The thirteen cases that left the list could all block: none stood on a declared bound. By family, red: `setup` 120 → 118 · `task finalize` 26 → 22 · `milestone` 32 → 26 · `doc read` 6 → 5; `uninstall` 45, `doc write` 10, `config` 2, `start` 0 and the dynamic sites' 3 as they were. Beyond the pilot's two commands 79 → 68, under 24 keys where there were 29; on them 165 → 163, under 26 where there were 27.

What the 627 came to on this tree's binary (block `cases`, column 6): holds 311 · holds, operand implied 85 · the same refusal again 129 · another failure 73 · a printed line no shell can split 18 · the command did not end 7 · a witness changed 4 — **and no case under the tenth classification**, *a cited line does not say what the route promised*: every cited line of the catalog says it on this tree.

### The thirty cases, and the six keys

Per case — the witness, and what it came to on both binaries before and after — is [route-catalog-doubts.md](route-catalog-doubts.md) → R. In sum: on this tree's binary another failure 9 · the same refusal again 4 · holds 17 became holds 25 · holds, operand implied 5; on `rc.24` holds 17 · not provoked 12 · the same refusal again 1 became holds 17 · not provoked 12 · a cited line does not say what the route promised 1. The six keys that left the findings file with their thirteen cases are named there and at the head of [route-catalog-findings.md](route-catalog-findings.md).

### Red cases at a refusal `rc.24` does not provoke — 58, under 17 keys (block `cases`, columns 5, 6 and 9)

A line of the block whose fifth column holds a key and whose ninth says *not provoked*: red on this tree, and on `rc.24` the refusing command printed no finding of the case's code at a non-zero exit — it exited 0, or another refusal answered. What such a line says and does not say is §8's.

| key | cases |
|---|---|
| `r1-setup-untracked-link-repointed-draws-dirty-install-path` | 16 |
| `r1-setup-install-hook-route-other-causes-unexamined` | 9 |
| `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | 7 |
| `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | 6 |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | 3 |
| `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | 3 |
| `r1-distinct-identity-routes-print-a-line-with-placeholders-for-the-readers-own-choice` | 2 |
| `r1-home-shape-destination-routes-print-a-line-with-a-placeholder-for-another-slug` | 2 |
| `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | 2 |
| `r1-clobber-fixed-route-prints-git-mv-without-a-path` | 1 |
| `r1-clobber-route-drops-the-milestones-only-sub-task-and-the-re-run-draws-zero-contribution` | 1 |
| `r1-clobber-staged-by-two-worktrees-rename-way-draws-code-collision` | 1 |
| `r1-home-shape-edited-doc-route-says-copy-what-the-link-points-at-over-a-link-to-nothing` | 1 |
| `r1-p3-setup-writability-route-printed-for-other-causes` | 1 |
| `r1-setup-force-over-skip-worktree-path-cannot-commit` | 1 |
| `r1-setup-settings-record-refused-rerun-installs-without-the-record` | 1 |
| `r1-unlanded-route-keeps-the-only-commit-and-the-re-run-draws-zero-contribution` | 1 |
| **17 keys** | **58** |

The twelve that are no longer among them are twelve of the thirteen that left the list: every one but the discard door's re-run, which `rc.24` refuses too.

### The same cases on the previous release's binary — what moved (block `cases`, columns 6, 9 and 12)

One line of §14's table moved, and no other: of `milestone`'s 84 lines the same classification on both is now 58 — 55 over the same printed route — where it was 59, and *other* 1 where it was 0. It is the discard door's re-run over a `.git` entry that leads nowhere, *holds, operand implied* here and *a cited line does not say what the route promised* on `rc.24`, whose route under the code is not the same. Beyond the pilot: 274 lines, 155 the same on both, 112 not provoked on `rc.24`, 7 other. The pilot's two commands as §14 has them.

### The arms' seconds

Inside a gate, one behind the other (the gate of the commit before this section's, `tests passed=5060`, its log's `test-seconds`, under a load average of 40 to 120): `start` 3.9 · `unassigned` 6.1 · `doc_write` 11.8 · `config` 13.5 · `doc_read` 21.5 · `uninstall` 25.5 · `setup` 47.3 · `milestone` 56.3 · `task_finalize` 75.0 — 260.9 s together; the arm that holds a cited line to its words, 14.0 s. The ignored arm: 536.5 s.

### Not established

- **That a witness is the promise its route makes**: a case states it, by one person's reading of the route's words; the frame holds a case to its witness and nothing holds the witness to the route.
- **A cited line whose promise no piece of text states** — what is absent from an output, an order of lines: no case asked for one.
- **The other eight questions**: 46 cases, marked and red as they were.
- **The frame over a route that removes the directory its reader stands in and then prints a command without a `cd`**: not met, not changed.
- **Linux**: the run follows the commit and is in the task's return.

## 16. 2026-10-12 — a placeholder for a value of the reader's own, filled: the catalog after the second ruling was carried out

**What this section is.** Of the nine questions the catalog's marked cases hang on ([route-catalog-doubts.md](route-catalog-doubts.md)), the human ruled a second on 2026-10-12: a printed line that carries a placeholder for a value of the reader's own is followed with the placeholder filled ([DECISIONS.md](../../../../../DECISIONS.md) → *A printed line that carries a placeholder for a value of the reader's own is followed with the placeholder filled*). The entry of that day gave the frame the act that carries it out; the one after it followed the thirteen cases again. This section counts the catalog when that second entry lands. It judges nothing: not what a difference between two binaries is, and not whether a value is the one a reader would choose — that is each case's reading, stated in the case with its reason.

**Where a number comes from.** One run of the ignored arm (§ *Where a number comes from*, at the head), on the tree of the commit that carries this section, the variable naming `rc.24`'s binary of §8: **408.4 s over 627 cases**, load average 14.0 at its start and 23.4 at its end, no gate of this branch beside it. [route-catalog-pilot-cases.tsv](route-catalog-pilot-cases.tsv) and [route-catalog-pilot-sites.tsv](route-catalog-pilot-sites.tsv) are the blocks `cases` and `sites` of that run, byte for byte. *Before* is §15.

### The catalog's totals (blocks `cases` and `bounds`)

| | before | after |
|---|---|---|
| cases | 627 | 627 |
| red — the cases the expected-red list holds | 231 | **218** |
| keys | 50 | **47** |
| red cases that can block · keys that can block | 162 · 40 | 149 · 37 |
| red cases on a declared bound — a permission bit · a FIFO · the override | 47 · 7 · 19 | 47 · 7 · 19 |
| the cases no entry lists | 396 | 409 |
| cases marked `doubt` — of them red | 46 — 46 | 33 — 33 |
| cases that name a value of the reader's own — column `operand`, `chosen` | — | 13 |
| **red cases at a refusal `rc.24` does not provoke** — listed red here, *not provoked* there | 58 | **54** |

The thirteen cases that left the list could all block: none stood on a declared bound. By family, red: `doc write` 10 → 3 · `doc read` 5 → 1 · the dynamic sites' 3 → 1; `setup` 118, `uninstall` 45, `task finalize` 22, `milestone` 26, `config` 2 and `start` 0 as they were. Beyond the pilot's two commands 68 → 55, under 21 keys where there were 24; on them 163, under 26, as it was.

What the 627 came to on this tree's binary (block `cases`, column 6): holds 324 · holds, operand implied 85 · the same refusal again 129 · another failure 73 · the command did not end 7 · a printed line no shell can split 5 · a witness changed 4 — and no case under *a cited line does not say what the route promised*. **The five lines no shell can split that are left are the five of the list's question 1**, a placeholder for a value the product knows and the refusal does not list: nothing names a value for them.

### The thirteen cases, and the three keys

Per case — the value, why that one, and what it came to on both binaries before and after — is [route-catalog-doubts.md](route-catalog-doubts.md) → 2. In sum: on this tree's binary a printed line no shell can split 13 became holds 13; on `rc.24` a printed line no shell can split 9 · not provoked 4 became holds 9 · not provoked 4. The three keys that left the findings file with their thirteen cases are named there and at the head of [route-catalog-findings.md](route-catalog-findings.md): 9 cases · 2 · 2.

### Red cases at a refusal `rc.24` does not provoke — 54, under 15 keys (block `cases`, columns 5, 6 and 9)

A line of the block whose fifth column holds a key and whose ninth says *not provoked*: red on this tree, and on `rc.24` the refusing command printed no finding of the case's code at a non-zero exit — it exited 0, or another refusal answered. What such a line says and does not say is §8's.

| key | cases |
|---|---|
| `r1-setup-untracked-link-repointed-draws-dirty-install-path` | 16 |
| `r1-setup-install-hook-route-other-causes-unexamined` | 9 |
| `r1-setup-seeded-ignore-route-followed-leaves-no-secrets-floor` | 7 |
| `r1-setup-link-pointed-at-uncommitted-file-draws-dirty-install-path` | 6 |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | 3 |
| `r1-setup-hook-link-pointed-at-uncommitted-script-draws-dirty-install-path` | 3 |
| `r1-uninstall-refusal-behind-the-workbench-rerun-leaves-the-settings-entries` | 2 |
| `r1-clobber-fixed-route-prints-git-mv-without-a-path` | 1 |
| `r1-clobber-route-drops-the-milestones-only-sub-task-and-the-re-run-draws-zero-contribution` | 1 |
| `r1-clobber-staged-by-two-worktrees-rename-way-draws-code-collision` | 1 |
| `r1-home-shape-edited-doc-route-says-copy-what-the-link-points-at-over-a-link-to-nothing` | 1 |
| `r1-p3-setup-writability-route-printed-for-other-causes` | 1 |
| `r1-setup-force-over-skip-worktree-path-cannot-commit` | 1 |
| `r1-setup-settings-record-refused-rerun-installs-without-the-record` | 1 |
| `r1-unlanded-route-keeps-the-only-commit-and-the-re-run-draws-zero-contribution` | 1 |
| **15 keys** | **54** |

The four that are no longer among them are the four of the thirteen that stand at `store.home-not-regular-file` — two under each of two keys that are gone.

### The same cases on the previous release's binary — what moved (block `cases`, columns 6, 9 and 12)

No line of §14's table moved from one of its three columns to another: nine of the thirteen were the same classification on both binaries over the same printed route and still are — *a printed line no shell can split* then, *holds* now — and four were *not provoked* on `rc.24` and still are. Beyond the pilot: 274 lines, 155 the same on both, 112 not provoked on `rc.24`, 7 other, as §15 has them. The pilot's two commands as §14 has them.

### The arms' seconds

Inside a gate, one behind the other (the gate of the commit before this section's, `tests passed=5061`, its record of seconds, under a load average of 30 to 105): `start` 3.0 · `unassigned` 5.0 · `config` 11.6 · `doc_write` 12.1 · `doc_read` 20.2 · `uninstall` 26.6 · `setup` 61.1 · `milestone` 64.7 · `task_finalize` 67.5 — 272.0 s together; the arm that fills a placeholder with the value a case names, 3.6 s. The arm of the eight second cases, new with this section's commit: about 20 s where it was run by itself. The ignored arm: 408.4 s.

### Not established

- **That a placeholder a case names is the reader's own, and that its value is one a reader would choose**: a case states both, by one person's reading of the route's words; the frame fills what it is told to and nothing holds the value to the route.
- **A value the route says to leave**: the frame reads the product's refusal of it as *the same refusal again*, as it reads a route that leads back to its refusal. Eight second cases hold what this tree's binary does with a value outside its route's words; none is a case of the catalog.
- **The first way of a route that offers two with one line** — a distinct title and no `--slug`: a reason at its site, no case.
- **The other seven questions**: 33 cases, marked and red as they were.
- **The frame over a route that removes the directory its reader stands in and then prints a command without a `cd`**: not met, not changed.
- **Linux**: the run follows the commit and is in the task's return.
