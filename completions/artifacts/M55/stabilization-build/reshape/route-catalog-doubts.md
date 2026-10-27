# The route catalog — every case marked `doubt`, by the question it hangs on

*Written 2026-10-12 with the last commit of the task that emptied the catalog's owed list, on the branch `fix/rc24-route-catalog`. One list to rule from, one question at a time. It decides nothing. Each case below follows the words its route prints most literally — reading (a) — and is listed with what it came to; the numbers of questions 1 to 8 are from one run of the suite's ignored arm on the tree of the commit that first carried this file (its block `doubts`, with `rc.24`'s binary beside this tree's; 467.8 s). The cases are in `crates/cli/tests/route_catalog/`, and the findings they are listed under are in [route-catalog-findings.md](route-catalog-findings.md). `rc.24` is the previous release's binary; what a difference between the two columns is, this file does not say.*

*Rewritten in its head and in section R on 2026-10-12, with the commit that carried the first ruling out; the numbers of that section are from one run of the same arm on that commit's tree (536.5 s over 627 cases, load average 69 at its start and 74 at its end, no gate of this branch beside it). In that run the 46 cases of questions 1 to 8 came to what the tables below say of them.*

*Rewritten in its head again, and in section 2, on 2026-10-12, with the commit that carried the second ruling out; the numbers of that section are from one run of the same arm on that commit's tree (408.4 s over 627 cases, load average 14 at its start and 23 at its end, no gate of this branch beside it). In that run the 33 cases of questions 1 and 3 to 8 came to what the tables below say of them.*

*Rewritten in its head a third time, and in sections 3 and 1, on 2026-10-12, with the commit that recorded two more rulings; the numbers of those sections are from one run of the same arm on that commit's tree (345.0 s over 627 cases, load average 14.0 at its start and 14.5 at its end, no gate of this branch beside it), and what the eleven cases' second refusal leads to was driven by hand that day. In that run the 17 cases of questions 4 to 8 came to what the tables below say of them.*

*Rewritten in its head a fourth time, and in sections 6, 7 and 8, on 2026-10-12, with the commit that carried three more rulings out; the numbers of those sections are from one run of the same arm on that commit's tree (608.7 s over 632 cases, load average 51.2 at its start and 43.9 at its end, no gate of this branch beside it). In that run the 4 cases of questions 4 and 5 came to what the tables below say of them.*

**4 cases are marked, under two questions — 4 and 5 — and neither is put to the human: every one of the four stands inside a declared bound, so no reading of either question can block.** Seven questions are ruled, and none of their cases carries a mark: R, 2, 6, 7 and 8 are carried out — a cited line held to its words, a reader's own value filled, a command's name followed with the operand its route speaks of, a re-run as the reader's own command, a line held to what its route says it does; 3 and 1 are ruled for the reading their cases already followed.

| | the question | where it was first named | cases | inside a declared bound |
|---|---|---|---|---|
| **R** | a command a route cites for what it prints — **ruled, and carried out: no case of it is marked** | the pilot's fourth; the gate's third — **one question**: both ask whether a printed line that is there for its output is held to exit 0 | 30, none marked | 0 |
| **2** | a placeholder for a value that is the reader's own to choose — **ruled, and carried out: no case of it is marked** | what was left of the pilot's first, **a second question**: no refusal could list such a value, so the first question's reading (a) — *the route owes it* — has no meaning here | 13, none marked | 0 |
| **3** | *a regular file inside this repository* — any, or a committed one — **ruled: any file in the checkout; the eleven stay red, and no case of it is marked** | the pilot's second | 11, none marked | 0 |
| **1** | a placeholder for a value the product knows and the refusal does not list — **ruled: the route owes it; the five stay red, and no case of it is marked** | what was left of the pilot's first | 5, none marked | 0 |
| **6** | a `git` verb printed with no operand — **ruled, and carried out: followed with the operand the route speaks of; no case of it is marked** | the pilot's sixth | 4, none marked | 1 |
| **7** | a product command printed without the id it requires — **ruled, and carried out: the name of a command; a re-run is the reader's own, and a named verb is followed with its operand; no case of it is marked** | the gate's first — **not the pilot's sixth**: the surface contract answers that one for a `git` span and says nothing of a `jigc` span | 13, none marked — they were 8 | 0 |
| **8** | a line the route says *mints* — **ruled, and carried out: held to the route's words; the case stays red, and is not marked** | the gate's second — **its own question**: the line has a quoted placeholder, which a shell hands on, and the doubt is over what the route says the line does | 1, not marked | 0 |
| 4 | *make sure `.jigc/…` is readable* — the entry named, or everything under it — **not put to the human: all three stand inside a declared bound** | the pilot's third | 3 | 3 |
| 5 | a clause quoted as a route, with a backticked shape in it — **not put to the human: the one case stands inside a declared bound** | the pilot's fifth | 1 | 1 |

**What [surface-contract.md](../../../../../design/surface-contract.md) says, by its line.** Line 44, the span fence: every backticked `git …` / `jigc …` span *must carry only shell-safe tokens (a `<placeholder>` the reader fills is legal bare or as a quoted span)*. Line 34, P6: *a placeholder whose value is derivable from the finding's own `key.target` must be substituted before the finding is projected* — for a mechanical route, and exactly two placeholders are derivable, `<address>` and `<doctype>`. Line 71, the printed-path fence: a `git` span is admitted in three shapes, one of them *a span with **no operand**, which names a command rather than a file (`git add`, `git worktree list`, `git stash -u`)*. It has no sentence on a `jigc` span without its operands, on a command a route cites, or on what a route may say a line does.

**Read in this order:** the seven questions that are ruled — R and 2 as carried out, 3 and 1 as ruled, then 6, 7 and 8 as carried out; then the two that are open, 4 and 5, which no reading makes blocking.

**Three of the rulings read as one rule about a value a printed line needs.** A value that is *the reader's own to choose* is named by the case, which fills it (2). A value *the product knows and the reader may never have seen* is owed by the refusal, and where the refusal does not list it the line is red as it stands (1). And an operand *the reader has just typed* — a product command printed without its id — is the reader's own command run again (7).

---

## R. A command a route cites for what it prints — ruled, and carried out

**The ruling — the human's, 2026-10-12** ([DECISIONS.md](../../../../../DECISIONS.md) → *A command a route cites for what it prints is held to its words*): a line a route cites for what it prints is *held to its words* — it passes when its output says what the route promised, whatever it exits with; the promised text is a witness of the case, so the check stays strict. Not taken: held to exit 0, under which a route could never cite a command that can itself be refused.

**Carried out on 2026-10-12.** The frame runs a cited line as printed, does not hold its exit code, and asks what the route promised of that line's own output and no other command's (`Act::RunCited`); a line that does not say it is red under a classification of its own, *a cited line does not say what the route promised*. Each of the thirty cases below now says which printed line is cited and states the promise, read off the route's own words for the case's state. A line that is not marked cited is held to exit 0 as before — in these cases too: the approval behind a review, the re-run behind git's report.

The routes: *run `git …` to see what git reports, resolve that, then re-run* · *`jigc task validate <task>` names where it lands* · *`jigc task finalize <task>` names that route* · *re-run `jigc task finalize <id>` to review the fidelity diff and `… --approve` to land it* · *or abandon the milestone with `jigc milestone discard <id>`, which answers for the same path with its own route* · *move that entry aside, and this door then prints the one line that re-links the checkout* · *move it aside and re-run for the command that brings the checkout back* · *`jigc config list` shows the value in force and the layer it wins from* · *`jigc describe` lists the doctype surface — check the id you typed against it*.

**On this tree's binary all thirty hold**: holds 25 · holds, operand implied 5 — where they were another failure 9 · the same refusal again 4 · holds 17. **On `rc.24`**: holds 17 · not provoked 12 · a cited line does not say what the route promised 1 — where they were holds 17 · not provoked 12 · the same refusal again 1.

| case | the witness — what the cited line's own output must say | before: this tree · `rc.24` | after, on this tree's binary | after, on `rc.24` |
|---|---|---|---|---|
| `setup-git-refuses-the-repository-asked-as-printed-and-resolved` | git's own report of the file the state broke: `fatal: bad config line 1 in file .git/config` | another failure · not provoked | holds, operand implied | not provoked |
| `setup-git-status-fails-asked-as-printed-and-resolved` | git's own report of the file the state broke: `fatal: .git/index: index file smaller than expected` | another failure · not provoked | holds, operand implied | not provoked |
| `linked-doc-second-write-refused-the-preview-asked-where-it-lands` | one line with the home the doc promotes to, `docs/decisions-log.md`, and *the main checkout at* that checkout's path | the same refusal again · not provoked | holds | not provoked |
| `linked-doc-write-refused-on-a-detached-head-the-finalize-asked-for-its-route` | a finding that says `HEAD is detached` and carries a route of its own naming `git switch <branch>` | another failure · not provoked | holds | not provoked |
| `clobber-held-in-head-migration-doc-renamed-reviewed-and-approved-as-printed` | the diff: `+++ canonical rewrite → docs/decisions/title.md`, the source's `- We use tabs.`, the rewrite's `+ Because.` | another failure · not provoked | holds | not provoked |
| `clobber-held-in-the-index-migration-doc-renamed-reviewed-and-approved-as-printed` | the diff: `+++ canonical rewrite → docs/decisions/title.md`, the source's `- We use tabs.`, the rewrite's `+ Because.` | another failure · not provoked | holds | not provoked |
| `dirty-worktree-a-git-entry-that-leads-nowhere-moved-out-and-discarded-as-printed` | the line that re-links: `printf 'gitdir: %s\n' ` with the registration's path and the checkout's `.git`, on one line | the same refusal again · the same refusal again | holds, operand implied | a cited line does not say what the route promised |
| `unlanded-no-directory-and-staged-paths-in-the-registration-milestone-discarded-as-printed` | a finding `at: .jigc/worktrees/do-a-thing` that carries a route of its own | another failure · not provoked | holds | not provoked |
| `unlanded-no-directory-and-a-registration-git-cannot-read-milestone-discarded-as-printed` | a finding `at: .jigc/worktrees/do-a-thing` that carries a route of its own | another failure · not provoked | holds | not provoked |
| `unlanded-a-file-where-the-worktree-was-moved-out-and-finalized-as-printed` | the recipe that brings the checkout back: `mkdir -p `, `printf 'gitdir: %s\n' ` with both paths, ` restore .`, on one line | the same refusal again · not provoked | holds, operand implied | not provoked |
| `unlanded-a-git-entry-that-leads-nowhere-moved-out-and-finalized-as-printed` | the line that re-links: `printf 'gitdir: %s\n' ` with the registration's path and the checkout's `.git`, on one line | the same refusal again · not provoked | holds, operand implied | not provoked |
| `unlanded-checkout-with-no-git-link-milestone-discarded-as-printed` | a finding `at: .jigc/worktrees/do-a-thing` that carries a route of its own | another failure · not provoked | holds | not provoked |
| `untrackable-root-above-the-repository-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `untrackable-root-in-gits-directory-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `untrackable-root-in-gits-directory-the-placement-knob-config-list-read-as-printed` | the knob's line, whole: `placement-root =   (pack-default)` | holds · holds | holds | holds |
| `untrackable-root-in-a-git-directory-of-another-name-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `untrackable-root-in-another-repository-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `workbench-root-typed-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `workbench-root-reached-by-a-hop-config-list-read-as-printed` | the knob's line, whole: `placement-root =   (pack-default)` | holds · holds | holds | holds |
| `unusable-root-absolute-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `unusable-root-padded-with-whitespace-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `unusable-root-a-leading-colon-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `unusable-root-a-wildmatch-character-config-list-read-as-printed` | the knob's line, whole: `placement-root =   (pack-default)` | holds · holds | holds | holds |
| `unusable-root-a-component-over-the-name-ceiling-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `unusable-root-through-a-link-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `unusable-root-a-file-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `unusable-root-under-the-installs-own-artifacts-config-list-read-as-printed` | the knob's line, whole: `docs-root = docs/  (pack-default)` | holds · holds | holds | holds |
| `home-shape-migration-over-a-link-link-moved-out-reviewed-and-approved-as-printed` | the diff: `+++ canonical rewrite → docs/decisions/use-tabs.md`, the note's own line with `- ` before it, the rewrite's `+ Because.` | another failure · not provoked | holds | not provoked |
| `rename-not-found-no-doc-of-that-id-surface-read-as-printed` | the line that opens the doc-types `jigc describe` lists, with `adr` on it: `The doc-types you can author. adr is ` | holds · holds | holds | holds |
| `rename-not-found-the-doc-gone-from-the-working-tree-surface-read-as-printed` | the line that opens the doc-types `jigc describe` lists, with `adr` on it: `The doc-types you can author. adr is ` | holds · holds | holds | holds |

**What the table says, and does not.**

- **No case of the thirty keeps its mark**: for each a witness could be stated that a wrong output fails — git's report names the file the state broke, a diff names the home and a line of each side, a list names one knob's value and its layer on one line, a route of the command's own stands in a finding at the same path.
- **Thirteen left the expected-red list, and six keys the findings file**: `r1-setup-unverified-route-opens-with-the-question-that-failed` · `r1-linked-doc-route-cites-a-command-that-is-itself-refused` · `r1-clobber-held-migration-route-review-step-exits-4` · `r1-worktree-doors-route-re-run-is-for-the-next-command` · `r1-unlanded-route-abandon-line-is-refused-by-the-discard-door` · `r1-home-shape-migration-route-review-step-exits-4`. No key of the six stayed: each held cases of this question alone.
- **The case that draws another refusal** — the finalize a route cites from a worktree whose `HEAD` is detached exits 1 under `repo.head-detached`. Its route is the one the citing route promised: the citing route says the task *commits nothing from here until it is on a branch* and that the finalize *names that route*, and the finalize's finding says *HEAD is detached* and routes *re-attach HEAD with `git switch <branch>`, then re-run this command*.
- **Six cases of `jigc milestone` stop at the cited line.** Read as steps, they went on to hold that the milestone was gone or the work landed; the route promises neither of the cited command, which *answers with its own route* or *prints the one line*. What comes behind it is the discard door's own route, which has its cases at its own site, or a shell recipe on the listed line, which no case runs — an earlier task drove that recipe by hand over a directory that was gone: it exits 0, and the finalize then lands the commit.
- **The one line of `rc.24` under the new word** is the discard door's re-run over a `.git` entry that leads nowhere: on that binary the refusal's line promises no line that re-links, its route is not the same, and its re-run prints none. The case's acts and its witness were written by reading this tree's refusal.
- **`jigc describe` lists doctypes, and the id that was typed names a doc.** The witness is what the route says the command lists — the doctype surface, with the id's doctype in it — and both cases hold by it. What a reader can check a doc's id against there is its first half.

---

## 2. A placeholder for a value that is the reader's own to choose — ruled, and carried out

**The ruling — the human's, 2026-10-12** ([DECISIONS.md](../../../../../DECISIONS.md) → *A printed line that carries a placeholder for a value of the reader's own is followed with the placeholder filled*): such a line *is followed with the placeholder filled* — the case names the value, the catalog puts it in, and the filled line is held to exit 0 like any other line a route prints. Not taken: *a printed line must run as it stands*, under which every such line is red, against the surface contract's line 44 (*a `<placeholder>` the reader fills is legal bare or as a quoted span*). Not taken: *such a line is no case*, under which nothing follows these routes at all.

**Carried out on 2026-10-12.** A case says which placeholder of a printed line is the reader's own, the value a reader would choose in the case's state, and why that one (`Act::RunChosen`). The frame puts the value in where the route prints that placeholder, as one word whatever it holds, runs the filled line and holds it to exit 0; the line that was run is in the record. A placeholder the case does not name is not filled — the five cases of question 1 are *a printed line no shell can split* as they were. A filled line that is refused for the value, its output quoting the value back, is the case's mistake: a fault of the case, never a red case.

The routes: *choose a distinct `--title`, or keep this one and pass `--slug <slug>` to mint beside the existing doc: `jigc doc create <type> --title <title> --slug <slug> --task <id>`* · *the id comes from `--slug …`, not the title, so pass a distinct `--slug`* — the same line · *set the payload's `title:` to a distinct title … and re-run the same `jigc doc author <type> --from-file <payload> --task <id>`* · *discard this task …, then re-migrate under a distinct slug: `jigc migrate <path> --as adr --slug <slug>`* · *give this doc an id whose home is free* / *an id nothing else answers to — re-run `jigc rename <id> --to '<the title>' --slug <other-slug>`*, and the same at `jigc doc rename`. The id, the address, the path and the task are printed concrete in each.

**On this tree's binary all thirteen hold** — where all thirteen were *a printed line no shell can split*. **On `rc.24`**: holds 9 · not provoked 4 — where they were a printed line no shell can split 9 · not provoked 4. The four are the ones at `store.home-not-regular-file`, a refusal that binary does not have.

| case | the value the case names, and why | before: this tree · `rc.24` | after, on this tree's binary | after, on `rc.24` |
|---|---|---|---|---|
| `title-ignored-committed-create-line-run-as-printed` | `<title>` = `Use Tabs!`: the title the reader typed, kept — *or keep this one*. `<slug>` = `use-tabs-again`: nothing answers to it; it mints beside `adr:use-tabs` | a printed line no shell can split · the same | holds | holds |
| `title-ignored-committed-create-under-a-slug-line-run-as-printed` | `<title>` = `Other`: the title typed — the route asks nothing of it. `<slug>` = `other`: distinct, as the route asks — other than `use-tabs` | a printed line no shell can split · the same | holds | holds |
| `title-ignored-committed-author-line-run-as-printed` | `<payload>` = a second payload file of the state, an ADR whole, its `title:` the distinct *Use spaces* | a printed line no shell can split · the same | holds | holds |
| `title-ignored-committed-under-a-migrations-slug-discarded-and-re-migrated-as-printed` | `<slug>` = `old-note`: a slug of the migration's own, distinct as the route asks — other than `use-tabs` | a printed line no shell can split · the same | holds | holds |
| `already-exists-create-line-run-as-printed` | `<title>` = `Slow start`: the title typed, kept. `<slug>` = `slow-start-again`: nothing stands at its home | a printed line no shell can split · the same | holds | holds |
| `already-exists-create-under-a-slug-line-run-as-printed` | `<title>` = `Other`: the title typed. `<slug>` = `other`: distinct, as the route asks — other than `slow-start` | a printed line no shell can split · the same | holds | holds |
| `already-exists-author-line-run-as-printed` | `<payload>` = a second payload file of the state, its `title:` the distinct *Slow start again* | a printed line no shell can split · the same | holds | holds |
| `home-shape-rename-destination-a-link-another-slug-line-run-as-printed` | `<other-slug>` = `spaces`: its home is free — other than `use-spaces`, which the title slugs to and whose home holds the link | a printed line no shell can split · not provoked | holds | not provoked |
| `home-shape-doc-rename-destination-a-link-another-slug-line-run-as-printed` | `<other-slug>` = `spaces`: the same | a printed line no shell can split · not provoked | holds | not provoked |
| `home-shape-create-only-create-over-a-link-line-run-as-printed` | `<title>` = `Slow start`: the title typed, kept. `<slug>` = `slow-start-again`: nothing stands at its home | a printed line no shell can split · not provoked | holds | not provoked |
| `home-shape-create-only-author-over-a-link-line-run-as-printed` | `<payload>` = a second payload file of the state, its `title:` the distinct *Slow start again* | a printed line no shell can split · not provoked | holds | not provoked |
| `rename-destination-taken-by-a-committed-doc-another-slug-line-run-as-printed` | `<other-slug>` = `spaces`: an id nothing else answers to — other than `use-spaces`, which the other doc holds | a printed line no shell can split · the same | holds | holds |
| `rename-destination-taken-by-a-file-in-no-commit-another-slug-line-run-as-printed` | `<other-slug>` = `spaces`: the same, over the file somebody wrote | a printed line no shell can split · the same | holds | holds |

**What the table says, and does not.**

- **No case of the thirteen keeps its mark, and all thirteen left the expected-red list** — with the three keys that held them and nothing else: `r1-distinct-identity-routes-print-a-line-with-placeholders-for-the-readers-own-choice` (9 cases) · `r1-home-shape-destination-routes-print-a-line-with-a-placeholder-for-another-slug` (2) · `r1-rename-occupied-route-prints-a-line-with-a-placeholder-for-another-slug` (2). No key of the three stayed.
- **What each case holds behind the filled line is the doc under the value that was named**: the staged ADR or finding at the id the slug or the payload's title gives it, with the title that was named as its heading; the renamed ADR at the home of the slug that was named, and gone from the home it had; the migration's recorded slug. Somebody's file at the taken home, and the other committed doc, are witnesses and are as they were.
- **The migration's line, which nobody had driven by hand, holds on both binaries**: the printed discard, then the migration again under the reader's slug.
- **A payload is a second file of the state.** The route says *set the payload's `title:` to a distinct title … and re-run the same* command; a reader edits the payload or writes another, and the command is handed a payload either way. The frame has no act that edits a file.
- **Where a route's line offers two ways with one shape** — *choose a distinct `--title`, or keep this one and pass `--slug <slug>`*, over a line that carries both placeholders — the case follows the second: the title kept, a slug named. The first way, a distinct title and no `--slug`, is the refused command with another argument and no line the route prints; it has a reason at its site, and was driven by hand there.
- **A value outside what its route says, on this tree's binary** — eight second cases, in an arm of their own (`route_catalog::a_value_outside_what_its_route_says_is_read_as_what_it_is_on_this_trees_binary`), none of them a case of the catalog:

  | the value | at | what the line did | what the frame read |
  |---|---|---|---|
  | a slug that is none, `Not A Slug` | `jigc doc create` · `jigc migrate` | exit 1, a bare line: `` `--slug "Not A Slug"` is not a valid slug `` | the case's mistake |
  | the same | `jigc rename` | exit 1, a finding: `write.malformed-slug`, the same words | the case's mistake |
  | a payload file that is not there | `jigc doc author` | exit 1: *could not read slot prose from* the path | the case's mistake |
  | the slug the route says to leave — `use-tabs`, `use-spaces` | `jigc doc create` · `jigc rename` | the case's own refusal again | the same refusal again |
  | the payload the route says to change | `jigc doc author` | the case's own refusal again | the same refusal again |
  | the slug the route says to leave, `use-tabs` | `jigc migrate` | **exit 0** — the refusal is the later `jigc doc author`'s | the line holds; the case's check of the recorded slug does not |

  **The frame cannot tell a value the route says to leave from a route that leads back to its refusal**: both are the case's own code again. A case's value is held to its route's bound by the case, which says so in its reason; and where a line takes a value its route says to leave, as the migration's does, it is the case's own check that holds the value.
- **Not established**: that a placeholder a case names is the reader's own, and that its value is one a reader would choose — stated by the case, read by one person. And `rc.24`'s nine were followed with the values chosen by reading this tree's routes; the two binaries print the same route in all nine.

---

## 3. *A regular file inside this repository* — any, or a committed one — ruled

**The ruling — the human's, 2026-10-12** ([DECISIONS.md](../../../../../DECISIONS.md) → *The route is held to any file in the checkout*): *the route is held to any file in the checkout* — one written a moment ago and in no commit included — because that is what its words say, and a new file is the most natural thing for a reader to point the link at. The eleven cases stay red and are findings of the product. Not taken: *a committed file*, under which the route is read as meaning what it does not say, and the eleven are no cases.

**What that changed.** Nothing in the frame, and no classification: the eleven cases followed the words already. They lose their mark, stay on the expected-red list under the word they had — *another failure* — and their two keys' rows in [route-catalog-findings.md](route-catalog-findings.md) say that the reading is ruled.

**The route says:** *point it at a regular file inside this repository* · *point it at a script inside this repository* — of a symbolic link at a path `jigc setup` writes. **What happens:** the link pointed at a file written a moment before (and the link's change committed, where git tracks the link), `jigc setup` again — exit 1, `setup.dirty-install-path` over the new file. The case beside each one points the link at a file the repository holds committed, and holds: eleven cases, never marked.

**What would make the route true** — an observation, and no fix: the route naming a committed file, or the re-run taking one that is in no commit.

**Where the second refusal's own route leads** — it decides how much the finding costs its reader, and makes no case green. Driven by hand on this tree's binary, 2026-10-12: each case from its own state, each way from a fresh sandbox. In all eleven the second refusal lists the new file and prints one route — *commit or stash the work at those path(s) — `git stash -u` where git does not track them yet — then re-run `jigc setup`; `jigc setup --force` is the single consent* — three ways:

| case | this tree's binary · `rc.24` | *commit* the file, then `jigc setup` | `git stash -u`, then `jigc setup` | `jigc setup --force` |
|---|---|---|---|---|
| `setup-hook-link-to-nothing-pointed-at-a-new-script` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.install-hook` again | **ends** — exit 0 |
| `setup-hooks-path-inside-link-to-nothing-pointed-at-a-new-script` | another failure · not provoked | **ends** — exit 0 | ends — exit 0, the reader's hooks directory in the stash | **ends** — exit 0 |
| `setup-hooks-path-outside-link-to-nothing-pointed-at-a-new-script` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.install-hook` again | **ends** — exit 0 |
| `setup-host-link-to-nothing-committed-and-pointed-at-a-new-file` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.inject-reference` again | **ends** — exit 0 |
| `setup-host-link-out-of-the-repository-committed-and-pointed-at-a-new-file` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.inject-reference` again | **ends** — exit 0 |
| `setup-host-link-to-a-directory-committed-and-pointed-at-a-new-file` | another failure · holds | **ends** — exit 0 | **does not end** — `setup.inject-reference` again | **ends** — exit 0 |
| `setup-host-link-into-gits-directory-committed-and-pointed-at-a-new-file` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.inject-reference` again | **ends** — exit 0 |
| `setup-settings-link-to-nothing-committed-and-pointed-at-a-new-file` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.inject-allowlist` again | **ends** — exit 0 |
| `setup-settings-link-out-of-the-repository-committed-and-pointed-at-a-new-file` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.inject-allowlist` again | **ends** — exit 0 |
| `setup-settings-link-to-a-directory-committed-and-pointed-at-a-new-file` | another failure · holds | **ends** — exit 0 | **does not end** — `setup.inject-allowlist` again | **ends** — exit 0 |
| `setup-settings-link-into-gits-directory-committed-and-pointed-at-a-new-file` | another failure · not provoked | **ends** — exit 0 | **does not end** — `setup.inject-allowlist` again | **ends** — exit 0 |

**What the table says, and does not.**

- **In all eleven the chain ends at a completed `jigc setup` by two of the three ways the second refusal names** — the new file committed, or `--force` — so the reader is refused once more than the route says, and is not left without a way.
- **By the third way, `git stash -u`, the chain does not end in ten**: the stash takes the new file away, the link leads to nothing, and the reader is back at the first refusal, whose route says again to point the link at a regular file inside this repository. In the eleventh it ends because the stash took the whole hooks directory, which was in no commit — the link is gone with it.
- **In the three cases of the hook, the commit that ends the chain may be the reader's own with the install in it.** That second refusal comes after the install's writes — *the install files are written and staged* — so a commit of the index takes nine install files along under the reader's message, and `jigc setup` then exits 0 and makes no commit of its own. With the commit held to the script's path, `jigc setup` makes the install commit. And the script the reader pointed the link at carries jigc's block either way.
- **Not established**: `rc.24`'s side of the chain — nine of the eleven are not provoked there, and the second route was followed on this tree's binary only. And each way was driven once.

---

## 1. A placeholder for a value the product knows and the refusal does not list — ruled

**The ruling — the human's, 2026-10-12** ([DECISIONS.md](../../../../../DECISIONS.md) → *The route owes every value it needs that is not the reader's own to choose*): *the route owes every value it needs that is not the reader's own to choose* — a value the product knows comes from the refusal's own output, and where the refusal does not list it the case is red as it stands. The five cases stay red and are findings of the product. Not taken: *the reader knows it from somewhere else*, under which the frame has no case, and a reader who has never seen the id is left with a line that cannot be run.

**What that changed.** Nothing in the frame, and no classification. The five lose their mark and stay on the expected-red list under the word they had, *a printed line no shell can split*; their two keys' rows in [route-catalog-findings.md](route-catalog-findings.md) say that the reading is ruled. **Two of the five lines carry a placeholder of the other kind too** — `<title>` and `<slug>`, `<different-slug>`: the reader's own. Those the case now names and the frame fills, by the ruling of question 2; the product's placeholders stay, and the line with them — so each of the five is red for a value the product owes, and for nothing else.

**The route says:** `jigc milestone discard <milestone-id> --force` over a refusal that lists a worktree's path and no milestone · `jigc doc create <doctype> --title <title> --slug <slug> --task <id>` and `jigc migrate <path> --as <doctype>` over a refusal that lists only the destination.

| case | missing from the refusal's output | where the product has it | this tree's binary · `rc.24` |
|---|---|---|---|
| `uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed` | the milestone's id, for `<milestone-id>` | the milestone's area, `.jigc/milestones/<id>/tasks.json`, names the sub-task whose worktree the refusal lists | a printed line no shell can split · the same |
| `clobber-task-file-at-the-home-re-created-with-a-slug-as-printed` | the doctype, `adr` · the task's id | the staged doc's address · the refused command's own argument | a printed line no shell can split · the same |
| `clobber-task-file-at-the-home-occupant-migrated-as-printed` | the doctype to adopt the occupant as | the head of the staged doc's address: the occupant stands at that doc's home | a printed line no shell can split · the same |
| `clobber-migration-file-at-the-destination-discarded-and-minted-again-as-printed` | the migration's id · its doctype | the refused command's own argument · the migration's workflow | a printed line no shell can split · the same |
| `clobber-migration-file-at-the-destination-finalized-and-approved-as-printed` | the migration's id | the refused command's own argument | a printed line no shell can split · the same |

**What the table says, and does not.**

- **What would make each route true** — an observation, and no fix: the refusal listing the value, or the route printing it concrete, as the other arms of the same functions do.
- **Driven by hand with the values put in**, as an earlier entry records: the uninstall route holds; of the clobber guard's task arm the printed way is refused under another code, an unprinted one holds, and a third is refused first. Nothing here was driven again for the ruling.
- **Three of the clobber routes' lines are a product command printed without its id** — `jigc task finalize`, `jigc migrate`: question 7's, and no part of what these five are red for.
- **Not established**: where the product has a value is read off the state and the constructor's arguments, by one person.

---

## 6. A `git` verb printed with no operand — ruled, and carried out

**The ruling — the human's, 2026-10-12** ([DECISIONS.md](../../../../../DECISIONS.md) → *A span that names a command without its operand is followed with the operand the route speaks of*). That such a span is a command's name and not a line to run as printed was settled — the surface contract's line 71, and the ruling on question 7; the question put was whether the catalog still follows the advice, and the human took option C: *a span that names a command without its operand is followed with the operand the route speaks of* — the case names the path, or the target, as data, the catalog runs the named command on it, and the case is held to the outcome: the refusal that was drawn is gone when the route's steps are done. Not taken: *a name is nothing to follow*, under which nothing checks whether *move the file out with `git mv`* gets a reader past the refusal.

**Carried out on 2026-10-12.** What a name is, the frame asks of the command itself and takes no case's word for: run bare, its command refuses the line for want of an operand — git's usage at exit 129, or `git add`'s *Nothing specified, nothing added*. A case hands the name its operand, a word each — a span the route prints, a value the refusal lists, or a word of the case's own — says in words which operand the route means, and where the route says the command belongs it is started there (`Act::RunNamed`). A name is run by nobody as printed.

| case | the named command, and the operand the case hands it | before: this tree · `rc.24` | after, on this tree's binary | after, on `rc.24` |
|---|---|---|---|---|
| `clobber-fixed-file-staged-at-the-logs-home-moved-with-git-mv-as-printed` | `git mv`, on the file at the doc's home — the path the route prints one clause before the verb — and a destination of the reader's own, `kept.md` | the same refusal again · not followed | holds | not followed: an act runs the command span 1 names, `--slug`, and names operands for it — but the span is a command line as it stands: run bare, its command does not refuse it for want of an operand. A line that is complete is run as printed |
| `clobber-fixed-sub-task-file-staged-at-the-logs-home-moved-with-git-mv-as-printed` | the same, in the main checkout, at the milestone's boundary | the same refusal again · not provoked | holds | not provoked |
| `zero-contribution-work-unstaged-provisioned-added-and-finalized-as-printed` | `git add`, on the file the sub-task wrote, started inside the sub-task's worktree — *`git add` their work inside their worktrees* | the same refusal again · the same refusal again | holds | holds |
| `uninstall-worktrees-root-unreadable-worktrees-removed-as-printed` — inside a declared bound | `git worktree remove`: **no operand** — the operand the route speaks of is a worktree `git worktree list` names, and that line, run as printed, names the main checkout alone | the same refusal again · the same refusal again | the same refusal again | the same refusal again |

**What the table says, and does not.**

- **Three of the four hold on this tree and left the expected-red list, with two keys that left the findings file**: `r1-clobber-fixed-route-prints-git-mv-without-a-path` (2 cases) and `r1-zero-contribution-route-prints-git-add-with-no-path-and-no-place` (1). Followed with the operand the route speaks of, the route gets its reader past the refusal: the staged file moved with its index entry, the finalize lands the doc; the work staged in its worktree, the milestone lands.
- **The fourth stays red, in its bound**: where `.jigc/worktrees/` cannot be listed there is no worktree for `git worktree remove` to be run on — the list names none — and the printed re-run refuses as before. Its key stands, and cannot block.
- **Where the product knows the operand it leaves out** — the path one clause before `git mv`; the sub-task's worktree — is a wording note: [route-catalog-findings.md](route-catalog-findings.md) → §11.
- **Not established**: which destination a reader gives `git mv` — the case's is one at the repository's root, which then rides the commit the finalize makes.

---

## 7. A product command printed without the id it requires — ruled, and carried out

**The ruling — the human's, 2026-10-12** ([DECISIONS.md](../../../../../DECISIONS.md) → *A span that names a command without its operand is followed with the operand the route speaks of*). The human took option B of two: *a `jigc` span printed without the operand it requires is the name of a command, not a line to run as printed* — as the surface contract's line 71 already reads a bare `git add`. Where the route says to *re-run* it, the act is the reader's own earlier command run again. Not taken: *a line to run as printed*, under which all eight are red at the argument parser, for an id the reader has just typed. Beside it, not a finding that blocks: where the product knows the id and prints the command without it, that is logged as a wording note. **Amended the same day by the ruling on question 6**: a route that only names verbs — *`jigc rename` the other onto a free identity or `jigc unmanage` it*, the milestone verbs — is followed with a target the case names, in place of being dropped as a route that prints no act.

**Carried out on 2026-10-12.** A `jigc` span is a name where the product's own argument parser refuses the bare line, exit 2, naming required arguments that were not provided — asked of the binary under test, by no table. **The re-run is one rule now**: the refusing command is run again wherever the route prints no line that the case passes over — a line the case runs as printed is passed over by nobody, and a name is no line. A named verb is followed with its operand as a `git` name is.

| case | what the route's names are followed with | before: this tree · `rc.24` | after, on this tree's binary | after, on `rc.24` |
|---|---|---|---|---|
| `finalize-nothing-staged-a-tracked-file-modified-added-and-finalized-as-printed` | `git add` on the changed path, `README.md`; then the reader's own finalize, run again | another failure · another failure | holds, operand implied | holds, operand implied |
| `finalize-nothing-staged-a-new-file-added-and-finalized-as-printed` | `git add` on `new.txt`; the same | another failure · another failure | holds, operand implied | holds, operand implied |
| `finalize-nothing-staged-a-tracked-file-deleted-added-and-finalized-as-printed` | `git add` on the deleted `README.md`; the same | another failure · another failure | holds, operand implied | holds, operand implied |
| `record-retitle-from-the-sub-task-a-sub-task-appended-by-the-named-verb` | `jigc milestone add-task`, on the milestone whose record it is and an intent of the reader's own | (as `…-the-milestone-verbs-run-as-printed`) another failure · another failure | holds | holds |
| `record-retitle-from-the-sub-task-the-milestone-finalized-by-the-named-verb` | `jigc milestone finalize`, on that milestone, the sub-task's work staged | — | holds | holds |
| `record-retitle-from-an-ordinary-task-a-sub-task-appended-by-the-named-verb` | as the first | another failure · another failure | holds | holds |
| `record-retitle-from-an-ordinary-task-the-milestone-finalized-by-the-named-verb` | as the second | — | holds | holds |
| `record-retitle-an-item-the-record-does-not-hold-a-sub-task-appended-by-the-named-verb` | as the first | another failure · another failure | holds | holds |
| `record-retitle-an-item-the-record-does-not-hold-the-milestone-finalized-by-the-named-verb` | as the second | — | holds | holds |
| `repoint-failed-a-stale-key-and-a-file-at-the-new-home-what-is-managed-listed-and-the-rename-run-as-printed` | `jigc doc list`, as printed; `jigc rename` on the one doc that list names, onto a title of the reader's own; the refusing command again | another failure · another failure | the same refusal again | the same refusal again |
| `repoint-failed-a-stale-key-and-a-file-at-the-new-home-what-is-managed-listed-and-the-home-unmanaged` | `jigc doc list`; `jigc unmanage` on the path the route prints; the refusing command again | — | holds, operand implied | holds, operand implied |
| `repoint-failed-a-stale-key-and-a-link-to-nothing-at-the-new-home-what-is-managed-listed-and-the-rename-run-as-printed` | as the rename above | another failure · not followed | the same refusal again | not followed: an act runs the refusing command again, and the printed route holds a command line — span 1, `jigc config set placement-root handbook` — that no act of the case runs. Where a route prints the command, the command is the route's to give: nothing was done |
| `repoint-failed-a-stale-key-and-a-link-to-nothing-at-the-new-home-what-is-managed-listed-and-the-home-unmanaged` | as the unmanage above | — | holds, operand implied | not followed: an act runs the refusing command again, and the printed route holds a command line — span 1, `jigc config set placement-root handbook` — that no act of the case runs. Where a route prints the command, the command is the route's to give: nothing was done |

**What the table says, and does not.**

- **The three of `finalize.nothing-staged` hold**: the change staged by its path, the reader's own finalize run again, one commit. Their key left the findings file. The path is the case's — the refusal lists the task and no path.
- **The record's retitle is followed by each verb its route names, a case a verb — six cases where there were three**, and all six hold; their key left the findings file. The route promises no way past the refusal — *no manual retitle applies* — and says what each verb does to the record, so each case is held to that: the sub-task appended, the status advanced. The finalize is followed from a state in which the sub-task's work is staged; in the first three cases' state it would itself be refused, with nothing to land.
- **The two re-point cases stay red, for the reason that rests on no reading**: the refusal says *two managed docs claim* a home at which nothing managed stands. **What following them showed**: *`jigc rename` the other onto a free identity* is refused — `write.identity-change` — because the one managed doc there is has an identity fixed to its type; the re-point then refuses as before, and the case is *the same refusal again*. ***Or `jigc unmanage` it*, with the path the route itself prints, goes through** — two more cases, green: the stale key is dropped, the re-point lands the log at its new home and parks what stood there. The route's second verb is a way out for a reader who takes *it* to be the entry at the home.
- **Where the product knows the id it leaves out** — the task's, the milestone's, the path one clause earlier — is a wording note: [route-catalog-findings.md](route-catalog-findings.md) → §11. Two more spans of this kind stand in routes whose cases are red for another reason — `jigc task finalize` in the clobber guard's task arm — and are noted there.
- **Not established**: that *the other* is what a case took it for. The route leaves the choice to its reader, the case says which it took and why, and a third reading was driven by hand (the findings row).

---

## 8. A line the route says *mints* — ruled, and carried out

**The ruling — the human's, 2026-10-12** ([DECISIONS.md](../../../../../DECISIONS.md) → *A line the route says mints is held to the route's words*): *the line is held to the route's words* — the intent is the reader's own and is filled by the case, and the filled line must then have minted a task. Run as printed it presents the workflows and mints nothing, so the case stays red and is a finding of the product. Not taken: *it is the front door, and the reader mints through it*, under which the route promises what its line does not do and nothing says so.

**Carried out on 2026-10-12.** The case fills `"<intent>"` with an intent it names — *record a decision* — which reaches the command as one word, the placeholder being quoted; and the filled line is held to what the route says of it, as a cited line is held to what it is cited for: its own output must hold the line the product prints when a task is minted, `task minted: record-a-decision`. The workflows' presentation holds no such line.

| case | the value, and the witness | before: this tree · `rc.24` | after, on this tree's binary | after, on `rc.24` |
|---|---|---|---|---|
| `finalize-amend-staged-doc-an-edit-refused-and-the-front-door-run-as-printed` | `<intent>` = `record a decision`; the output must say `task minted: record-a-decision` | another failure · another failure | a cited line does not say what the route promised | a cited line does not say what the route promised |

- **The case stays red, under the word for a promise that was not kept** — *a cited line does not say what the route promised* — where it was *another failure* for a check of the task's directory. Its key stands; its row says the reading is ruled, and what would make the route true: the line that does mint, printed, or the route not saying *mints*.
- **Not established**: that the mint line is the witness the route means — it is what the product prints when a task is minted, read off one run.

---

## 4. *Make sure `.jigc/…` is readable* — the entry named, or everything under it

**The route says:** *make sure `.jigc/…` is readable* — at `jigc uninstall`, where a guard could not look.

- **(a)** the entry the route names, given its owner's read bit.
- **(b)** every directory under it, readable and searchable.

**The cases — 3, all three inside a declared bound** (a permission bit changed by hand): under (a) all three are red (*the same refusal again*). (b) was not driven. Whichever way it is read, none of the three can block.

**The contract:** silent.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `uninstall-task-areas-unlistable-made-readable` | the same refusal again | the same refusal again |
| `uninstall-workbench-unsearchable-made-readable` | the same refusal again | the same refusal again |
| `uninstall-staged-docs-unsearchable-made-readable` | the same refusal again | the same refusal again |

## 5. A clause quoted as a route, with a backticked shape in it

**The route says:** *spawn template missing the required `jigc workflow … --task …` invocation* — the route is the violated clause and nothing else.

- **(a)** the backticked `jigc` line is a command to run.
- **(b)** it is the shape the template must hold, and the route prints nothing to run.

**The case — 1, inside a declared bound** (a state only `JIGC_ADAPTERS_DIR` reaches): red either way — under (b) it is one more *the same refusal again*. It cannot block.

**The contract:** silent.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `setup-launch-template-without-the-invocation` | another failure | another failure |
