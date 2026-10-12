# The route catalog — every case marked `doubt`, by the question it hangs on

*Written 2026-10-12 with the last commit of the task that emptied the catalog's owed list, on the branch `fix/rc24-route-catalog`. One list to rule from, one question at a time. It decides nothing. Each case below follows the words its route prints most literally — reading (a) — and is listed with what it came to; the numbers are from one run of the suite's ignored arm on the tree of the commit that carries this file (its block `doubts`, with `rc.24`'s binary beside this tree's; 467.8 s). The cases are in `crates/cli/tests/route_catalog/`, and the findings they are listed under are in [route-catalog-findings.md](route-catalog-findings.md). `rc.24` is the previous release's binary; what a difference between the two columns is, this file does not say.*

**76 cases are marked, under nine questions. One of the nine is ruled.** The pilot's record listed six questions (its §7), and the three returns after it named three more — *a product command printed without its id*, *a line the route says mints*, *a command cited for what it prints*. Read together they are nine, not twelve:

| | the question | where it was first named | cases | inside a declared bound |
|---|---|---|---|---|
| **R** | a command a route cites for what it prints — **ruled** | the pilot's fourth; the gate's third — **one question**: both ask whether a printed line that is there for its output is held to exit 0 | 30 | 0 |
| 1 | a placeholder for a value the product knows and the refusal does not list | what was left of the pilot's first | 5 | 0 |
| 2 | a placeholder for a value that is the reader's own to choose | what was left of the pilot's first, **a second question**: no refusal could list such a value, so the first question's reading (a) — *the route owes it* — has no meaning here | 13 | 0 |
| 3 | *a regular file inside this repository* — any, or a committed one | the pilot's second | 11 | 0 |
| 4 | *make sure `.jigc/…` is readable* — the entry named, or everything under it | the pilot's third | 3 | 3 |
| 5 | a clause quoted as a route, with a backticked shape in it | the pilot's fifth | 1 | 1 |
| 6 | a `git` verb printed with no operand | the pilot's sixth | 4 | 1 |
| 7 | a product command printed without the id it requires | the gate's first — **not the pilot's sixth**: the surface contract answers that one for a `git` span and says nothing of a `jigc` span | 8 | 0 |
| 8 | a line the route says *mints* | the gate's second — **its own question**: the line has a quoted placeholder, which a shell hands on, and the doubt is over what the route says the line does | 1 | 0 |

**What [surface-contract.md](../../../../../design/surface-contract.md) says, by its line.** Line 44, the span fence: every backticked `git …` / `jigc …` span *must carry only shell-safe tokens (a `<placeholder>` the reader fills is legal bare or as a quoted span)*. Line 34, P6: *a placeholder whose value is derivable from the finding's own `key.target` must be substituted before the finding is projected* — for a mechanical route, and exactly two placeholders are derivable, `<address>` and `<doctype>`. Line 71, the printed-path fence: a `git` span is admitted in three shapes, one of them *a span with **no operand**, which names a command rather than a file (`git add`, `git worktree list`, `git stash -u`)*. It has no sentence on a `jigc` span without its operands, on a command a route cites, or on what a route may say a line does.

---

## R. A command a route cites for what it prints — ruled

**The ruling — the human's, 2026-10-13, as the session that orchestrates this branch handed it to this task: a line a route cites for what it prints is *held to its words* — it passes when its output says what the route promised, whatever it exits with; the promised text is a witness of the case, so it stays strict. Not taken: held to exit 0, under which a route could never cite a command that can be refused.**

Nothing in the catalog carries the ruling out yet. The frame holds every printed command to exit 0, and the 30 cases below are marked and listed as they were. **They are to be followed again by the task after this one**, once the frame has an act that runs a cited line and holds its output to a witness; the ruling's own entry in `DECISIONS.md` is that task's too.

The routes: *run `git …` to see what git reports, resolve that, then re-run* · *`jigc task validate <task>` names where it lands* · *re-run `jigc task finalize <id>` to review the fidelity diff and `… --approve` to land it* · *or abandon the milestone with `jigc milestone discard <id>`, which answers for the same path with its own route* · *move that entry aside, and this door then prints the one line …* · *`jigc config list` shows the value in force and the layer it wins from* · *`jigc describe` lists the doctype surface*.

On this tree, held to exit 0: another failure 9 · the same refusal again 4 · holds 17. The last column says what can be told from the output the run recorded, without driving anything.

| case | on this tree's binary | on `rc.24` | the cited command's recorded output |
|---|---|---|---|
| `setup-git-refuses-the-repository-asked-as-printed-and-resolved` | another failure | not provoked | holds what was promised: git's own report — *fatal: bad config line 1 in file .git/config* · *fatal: .git/index: index file smaller than expected* — at exit 128; the printed re-run behind it exits 0 |
| `setup-git-status-fails-asked-as-printed-and-resolved` | another failure | not provoked | holds what was promised: git's own report — *fatal: bad config line 1 in file .git/config* · *fatal: .git/index: index file smaller than expected* — at exit 128; the printed re-run behind it exits 0 |
| `linked-doc-second-write-refused-the-preview-asked-where-it-lands` | the same refusal again | not provoked | not kept: `jigc task validate` exits 3 under the case's own code again, and the runner records no text of a repeated refusal |
| `linked-doc-write-refused-on-a-detached-head-the-finalize-asked-for-its-route` | another failure | not provoked | another refusal: `jigc task finalize` exits 1 under `repo.head-detached`; whether its route is the one the citing route promises was not read |
| `clobber-held-in-head-migration-doc-renamed-reviewed-and-approved-as-printed` | another failure | not provoked | holds what was promised: *migration review required — nothing committed*, then the fidelity diff, at exit 4; the printed approval behind it exits 0 |
| `clobber-held-in-the-index-migration-doc-renamed-reviewed-and-approved-as-printed` | another failure | not provoked | holds what was promised: *migration review required — nothing committed*, then the fidelity diff, at exit 4; the printed approval behind it exits 0 |
| `dirty-worktree-a-git-entry-that-leads-nowhere-moved-out-and-discarded-as-printed` | the same refusal again | the same refusal again | not kept: the same |
| `unlanded-no-directory-and-staged-paths-in-the-registration-milestone-discarded-as-printed` | another failure | not provoked | holds what was promised: `milestone.dirty-worktree` over the same path, with a route of its own, at exit 1 |
| `unlanded-no-directory-and-a-registration-git-cannot-read-milestone-discarded-as-printed` | another failure | not provoked | holds what was promised: `milestone.dirty-worktree` over the same path, with a route of its own, at exit 1 |
| `unlanded-a-file-where-the-worktree-was-moved-out-and-finalized-as-printed` | the same refusal again | not provoked | not kept: the runner records a repeated refusal as *the same code again* and not its text — whether the line that re-links the checkout is in it cannot be told from the record |
| `unlanded-a-git-entry-that-leads-nowhere-moved-out-and-finalized-as-printed` | the same refusal again | not provoked | not kept: the runner records a repeated refusal as *the same code again* and not its text — whether the line that re-links the checkout is in it cannot be told from the record |
| `unlanded-checkout-with-no-git-link-milestone-discarded-as-printed` | another failure | not provoked | holds what was promised: `milestone.dirty-worktree` over the same path, with a route of its own, at exit 1 |
| `untrackable-root-above-the-repository-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `untrackable-root-in-gits-directory-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `untrackable-root-in-gits-directory-the-placement-knob-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `untrackable-root-in-a-git-directory-of-another-name-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `untrackable-root-in-another-repository-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `workbench-root-typed-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `workbench-root-reached-by-a-hop-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-absolute-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-padded-with-whitespace-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-a-leading-colon-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-a-wildmatch-character-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-a-component-over-the-name-ceiling-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-through-a-link-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-a-file-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `unusable-root-under-the-installs-own-artifacts-config-list-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `home-shape-migration-over-a-link-link-moved-out-reviewed-and-approved-as-printed` | another failure | not provoked | holds what was promised: *migration review required — nothing committed*, then the fidelity diff, at exit 4; the printed approval behind it exits 0 |
| `rename-not-found-no-doc-of-that-id-surface-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |
| `rename-not-found-the-doc-gone-from-the-working-tree-surface-read-as-printed` | holds | holds | not kept: the runner keeps no output of a command that exits 0 |

**Twelve of these were marked before this task** (the two of `setup`, and ten of the gate and of `jigc milestone`); eighteen are this task's — fifteen of `jigc config set`, one of the store's refusal in a migration, two of `jigc rename`. **For eight of the thirteen red ones the recorded output holds what the route promised**; for the other five the record does not keep it, or holds another refusal. The seventeen that hold exited 0, and no output of theirs is kept.

---

## 1. A placeholder for a value the product knows and the refusal does not list

**The route says:** `jigc milestone discard <milestone-id> --force` over a refusal that lists a worktree's path and no milestone · `jigc doc create <doctype> --title <title> --slug <slug> --task <id>` and `jigc migrate <path> --as <doctype>` over a refusal that lists only the destination — and, in the same routes, `jigc task finalize` with no id.

- **(a)** the value comes from the refusal's own output, as every other value a route needs: the route owes it, and the case is red as it stands.
- **(b)** the reader knows it from somewhere else: then the frame, which fills a placeholder only with a value the refusal lists, has no case.

**The cases — 5, none inside a bound:** under (a) all 5 are red (*a printed line no shell can split*). Under (b) none is a case. Driven by hand with the values put in, as an earlier entry records: the uninstall route holds; of the clobber guard's task arm the printed way is refused, an unprinted one holds, and a third is refused first.

**The contract:** line 44 makes the placeholder legal; line 34 asks for `<address>` and `<doctype>` concrete where the finding's own target gives them, on a mechanical route — these are prose routes. It does not say where a reader takes any other value from.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `clobber-task-file-at-the-home-re-created-with-a-slug-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `clobber-task-file-at-the-home-occupant-migrated-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `clobber-migration-file-at-the-destination-discarded-and-minted-again-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `clobber-migration-file-at-the-destination-finalized-and-approved-as-printed` | a printed line no shell can split | a printed line no shell can split |

## 2. A placeholder for a value that is the reader's own to choose

**The route says:** `jigc doc create <type> --title <title> --slug <slug> --task <id>` · `jigc doc author <type> --from-file <payload> --task <id>` · `jigc migrate <path> --as <type> --slug <slug>` · `jigc rename <id> --to <title> --slug <other-slug>` · `jigc doc rename <id> --to <title> --slug <other-slug>` — the id, the address and the task concrete, and a bare word in angle brackets where a title, a slug or a payload file of the reader's own goes.

- **(a)** the backticked line is a command line to run as printed, like every `jigc` line a route prints: a bare `<word>` is a redirection to a shell, and the line cannot be split.
- **(b)** it is a shape the reader fills with a value of their own: then the value is the reader's, and the frame has no case.

**The cases — 13, none inside a bound:** under (a) all 13 are red (*a printed line no shell can split*). Under (b) none is a case; **driven by hand with a value put in, twelve of the thirteen lines exit 0** — every one but the migration's, which was not driven.

**The contract:** line 44 — *a `<placeholder>` the reader fills is legal bare or as a quoted span*. Read by it alone, (b). A quoted placeholder is one word to a shell and such a line is run as printed by the catalog today (`--to "<title>"`, `"<intent>"`); these are bare.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `title-ignored-committed-create-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `title-ignored-committed-create-under-a-slug-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `title-ignored-committed-author-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `title-ignored-committed-under-a-migrations-slug-discarded-and-re-migrated-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `already-exists-create-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `already-exists-create-under-a-slug-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `already-exists-author-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `home-shape-rename-destination-a-link-another-slug-line-run-as-printed` | a printed line no shell can split | not provoked |
| `home-shape-doc-rename-destination-a-link-another-slug-line-run-as-printed` | a printed line no shell can split | not provoked |
| `home-shape-create-only-create-over-a-link-line-run-as-printed` | a printed line no shell can split | not provoked |
| `home-shape-create-only-author-over-a-link-line-run-as-printed` | a printed line no shell can split | not provoked |
| `rename-destination-taken-by-a-committed-doc-another-slug-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |
| `rename-destination-taken-by-a-file-in-no-commit-another-slug-line-run-as-printed` | a printed line no shell can split | a printed line no shell can split |

## 3. *A regular file inside this repository* — any, or a committed one

**The route says:** *point it at a regular file inside this repository* · *point it at a script inside this repository* — of a symbolic link at a path `jigc setup` writes.

- **(a)** any file, or script, in the checkout — one written a moment ago and in no commit included.
- **(b)** one the repository already holds committed.

**The cases — 11, none inside a bound:** under (a) all 11 are red (*another failure*: the re-run draws `setup.dirty-install-path` over the new file). Under (b) the case beside each one points the link at a committed file and holds — eleven cases, not marked.

**The contract:** silent.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `setup-hook-link-to-nothing-pointed-at-a-new-script` | another failure | not provoked |
| `setup-hooks-path-inside-link-to-nothing-pointed-at-a-new-script` | another failure | not provoked |
| `setup-hooks-path-outside-link-to-nothing-pointed-at-a-new-script` | another failure | not provoked |
| `setup-host-link-to-nothing-committed-and-pointed-at-a-new-file` | another failure | not provoked |
| `setup-host-link-out-of-the-repository-committed-and-pointed-at-a-new-file` | another failure | not provoked |
| `setup-host-link-to-a-directory-committed-and-pointed-at-a-new-file` | another failure | holds |
| `setup-host-link-into-gits-directory-committed-and-pointed-at-a-new-file` | another failure | not provoked |
| `setup-settings-link-to-nothing-committed-and-pointed-at-a-new-file` | another failure | not provoked |
| `setup-settings-link-out-of-the-repository-committed-and-pointed-at-a-new-file` | another failure | not provoked |
| `setup-settings-link-to-a-directory-committed-and-pointed-at-a-new-file` | another failure | holds |
| `setup-settings-link-into-gits-directory-committed-and-pointed-at-a-new-file` | another failure | not provoked |

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

## 6. A `git` verb printed with no operand

**The route says:** *remove the worktrees yourself (`git worktree list`, then `git worktree remove`)* · *move the file out of the doc's home … with `git mv` if git has it staged* · *execute the sub-tasks, `git add` their work inside their worktrees*.

- **(a)** a command line to run as printed, like every `git` line a route prints: it has no operand, and does nothing or is refused.
- **(b)** the name of a git verb, to be run on the paths the route speaks of — then the route prints no line for it.

**The cases — 4, one inside a declared bound** (the first, a permission bit): under (a) all four are red (*the same refusal again*). Under (b) none has a line to follow; (b) was not driven at any of the four.

**The contract:** line 71 — *a span with no operand … names a command rather than a file (`git add`, `git worktree list`, `git stash -u`)*. Read by it, (b): the span is a name. The contract says so of the span's shape; it does not say what a route owes a reader beside a name.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `uninstall-worktrees-root-unreadable-worktrees-removed-as-printed` | the same refusal again | the same refusal again |
| `clobber-fixed-file-staged-at-the-logs-home-moved-with-git-mv-as-printed` | the same refusal again | not followed: an act runs span 1, `--slug`, which is no `jigc` or `git` command line |
| `clobber-fixed-sub-task-file-staged-at-the-logs-home-moved-with-git-mv-as-printed` | the same refusal again | not provoked |
| `zero-contribution-work-unstaged-provisioned-added-and-finalized-as-printed` | the same refusal again | the same refusal again |

## 7. A product command printed without the id it requires

**The route says:** *`git add` your changes, then re-run `jigc task finalize`* · *leave the record to the milestone verbs — `jigc milestone add-task` appends sub-tasks and `jigc milestone finalize` advances their status* · *decide which one keeps the home, then `jigc rename` the other onto a free identity or `jigc unmanage` it*.

- **(a)** a line to run as printed, which the argument parser refuses.
- **(b)** the name of the command — the one the reader has just run, or the one to run on what the reader decides for.

**The cases — 8, none inside a bound:** under (a) all 8 are red (*another failure*, exit 2). Under (b): the three of `finalize.nothing-staged` become a re-run of the reader's own, which the frame runs only where a route prints no command line; the three of the record's retitle name no act at all, and there is nothing to follow; the two of the re-point over a stale record key are left with *decide which one keeps the home*. **Those two are red for another reason too, which rests on no reading**: the refusal says two managed docs claim a home at which nothing managed stands.

**The contract:** line 71 speaks of a `git` span with no operand; of a `jigc` span it says nothing of the kind. Its rule that a derivable placeholder is printed concrete (line 34) is about a placeholder that is printed, not about an operand that is left out.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `finalize-nothing-staged-a-tracked-file-modified-added-and-finalized-as-printed` | another failure | another failure |
| `finalize-nothing-staged-a-new-file-added-and-finalized-as-printed` | another failure | another failure |
| `finalize-nothing-staged-a-tracked-file-deleted-added-and-finalized-as-printed` | another failure | another failure |
| `record-retitle-from-the-sub-task-the-milestone-verbs-run-as-printed` | another failure | another failure |
| `record-retitle-from-an-ordinary-task-the-milestone-verbs-run-as-printed` | another failure | another failure |
| `record-retitle-an-item-the-record-does-not-hold-the-milestone-verbs-run-as-printed` | another failure | another failure |
| `repoint-failed-a-stale-key-and-a-file-at-the-new-home-what-is-managed-listed-and-the-rename-run-as-printed` | another failure | another failure |
| `repoint-failed-a-stale-key-and-a-link-to-nothing-at-the-new-home-what-is-managed-listed-and-the-rename-run-as-printed` | another failure | not followed: an act names span 3, and the printed route has 2 spans |

## 8. A line the route says *mints*

**The route says:** *`jigc start "<intent>"` mints an ordinary task, whose finalize commits the promoted doc*.

- **(a)** the printed line mints: run as it stands — the quoted placeholder is one word — it is held to having minted a task.
- **(b)** it is the front door, and the reader mints through it.

**The case — 1, not inside a bound:** under (a) it is red (*another failure*): the line, run as printed, presents the workflows and mints nothing. (b) was not driven.

**The contract:** line 44 makes the quoted placeholder legal and the reader's to fill; it does not say what a line owes that a route describes.

| case | on this tree's binary | on `rc.24` |
|---|---|---|
| `finalize-amend-staged-doc-an-edit-refused-and-the-front-door-run-as-printed` | another failure | another failure |
