# The pre-opening fixes — what was built, what was left open (for the run's ledger at its opening)

*Kept by the orchestrator from the fixers' returns, on branch `fix/rc24-pre-opening`; committed 2026-10-07 with the record that carries it. `<scratch>` is the session's scratch root, which is not copied. Where this log and the repository disagree the repository wins: each such place carries a note in bold square brackets, dated 2026-10-07, and the README beside this file lists them.*

## Area S — setup and uninstall (fixer returned 2026-10-06)
| Ledger item | Commit | Gate | Notes |
|---|---|---|---|
| 3 — the log refusal says the order | `cea9755a` | 4762/0 | one producer; both printed commands driven on a release build under a spaced root |
| 1 — `setup` completes after a deleted install file | `4f680249` | 4766/0 | class had two readers, the brief named one (`record_failed_install` the second); both variants driven — a second run had completed only where the restored bytes equalled `HEAD`'s |
| 18 — a link to an ignored file is followed | `aac460ea` | 4767/0 | still refused: outside the checkout, nowhere, a directory, inside `.git/` |
| 17 — own code where git cannot answer | `90fee322` | 4768/0 | **new code `setup.unverified-install-path`**; registered: engine constant, `git_span_aim` census row, `design/validation.md` row |
| 19 — the hook's end marker | `dafafb95` | 4772/0 | reach inside what was named; driven with the real rc.24 binary (before: the upgrade left two blocks) |
| 21 — permission entries | **halted, nothing committed** | — | loss reproduced on release and on rc.24: the user's own allow, deny **and `SessionStart`** entries go — three removals, not one. Red test: `<scratch>/pre-opening/fx/i21-red-test.patch`. Fork for the human: A one committed record trusted everywhere (a second clone with a private settings file can still lose its own entry) · B the record is consulted only where git tracks the settings file, otherwise leave and name · C add a per-clone record (two files). Also open: what `setup` writes over an install that predates the record. |

**Doors changed:** `jigc setup`, `jigc uninstall`, and the installed pre-commit hook's text.

**Left open by the fixer — each a ledger row at the opening:**
1. The pending row's heading for the eleven fixes still says "none built". **[2026-10-07, read against the repository: no longer so — `d53a7fce` corrected the heading on this branch. This line becomes no ledger row.]**
2. A foreign hook that is only `#!/bin/sh` + `exit 0` is removed together with jigc's block.
3. An edited hook without the end marker is still wrapped by `setup`.
4. `uninstall` removes a link whose target held only jigc's section.
5. The commit-time backstop's message for a staged-then-deleted file (`AD`/`MD`) says "pre-run bytes".

## Second fixer — the boundary, a doc's home, two routes (returned 2026-10-06)
| Ledger item | Commit | Gate | Notes |
|---|---|---|---|
| 12 — the record refusal names the exit that works | `30eb446c` | 4774/0 | one producer (`record_conflict_block`, `LastWrite`) through seven call sites; the route prints `git -C <home> status --porcelain -- <record>` and, where that is empty and no commit changed the record, `jigc unmanage <record>` then the same command; driven on a release build under a spaced root with `eol=crlf`; the cause untouched |
| 6 — `milestone finalize` refuses over a discarded sub-task's unstaged/untracked files | `855fef44` | 4778/0 | exit: `git -C <worktree> stash --include-untracked`, driven as printed; must-not cells: only ignored output, a landed sub-task's leftovers, a clean worktree, a `cp -R` copy; `status.showUntrackedFiles=no` changes no cell (an untracked file still refuses under it); `milestone.unlanded-work` now named in MIGRATING and worked-examples flow 33 |
| the four declared bounds, in their owning docs | `d53a7fce` | 4778/0 | item 22 → `design/finalize.md` (4. Promote); item 23 → `design/reconciliation.md` (What a task copied in); the invocation log → `design/measurement.md` item 7; item 16 → `design/validation.md`; the pending heading corrected; the three halts noted on their lines and in a `DECISIONS.md` entry |

**Halted to the human, nothing committed for them:**
- **Item 15.** After the deletion is committed, `jigc task finalize` exits 3 on `finalize.base-mismatch`: the task predates that commit, which touches the doc's path. What does land: commit the deletion, discard the task, start again. So the ruled exit "commit the deletion first, then finalize" is not true as worded.
- **Items 7 + 8.** The reach is larger than about five: **fourteen commands answer the shape under six codes** — `finalize.promote-clobber` (`task finalize`, `milestone finalize`, `rename`, `relocate`, both `config set` roots) · `write.already-present` (a rename destination) · `create.already-exists` · `reconciliation.conflict-block` (the five record commands) · `milestone.record-exists` · `migrate-corpus.destination-collision`. Pinned beside them: `AMBUSH_CONTRACTS`' row at `task finalize` and two help texts. Open: whether the promoting commands and the destination cases move too; the name (the fixer proposes `store.home-not-regular-file`).
- **Item 9.** Blocked on that code. Also: `migrate-corpus`'s fold stops at its first blocked doc, so "the rest still migrates" is a second change.

**Doors changed by this fixer:** `milestone add-task`, `add-from-spec`, `milestone discard`, a sub-task's `task discard` (route text only); `milestone finalize` (the new refusal and its help text).

## Rulings on the halts (the human, 2026-10-06)
- **Item 21 — option B; his words: "Take B as recommend - the record is used only where git tracks the settings file".** `setup` records which permission entries (allow, deny, `SessionStart`) it added, in one small committed file; **the record is consulted only where git tracks the settings file**, so the record and the file it describes travel together; where the settings file is ignored or untracked, `uninstall` leaves the entries in place and names them; an install with no record is treated the same way. **Over** one committed record trusted everywhere (a second clone with a private settings file could still lose its own entry) and a second, per-clone record (two files, two mechanisms). A fixer builds it next (red test: `<scratch>/pre-opening/fx/i21-red-test.patch`).
- **Item 15 — option A; his words: "Agree on your suggestion A".** The earlier ruling's exit ("commit the deletion first, then finalize") is not true as worded: driven, finalize then exits 3 on `finalize.base-mismatch` because the task predates that commit. **Now ruled: say what is true, in both places** — the refusal's route says *commit the deletion, discard this task, start it again*; and `jigc unmanage`, when the user confirms a deletion, says to commit that deletion before starting a task that reuses the name. Text and tests only; no change to any check. **Over** making "commit, then finalize" work through an exception in the base-mismatch check, and over leaving the refusal as built with the case listed as an accepted exception. To be built in the second tree after item 21's fixer.
- **Items 7 + 8 — option A; his words: "Agree on A".** At its real reach: **all fourteen commands that meet the shape answer the one new code** — today under six codes: `finalize.promote-clobber` (`task finalize`, `milestone finalize`, `rename`, `relocate`, both `config set` roots), `reconciliation.conflict-block` (the five milestone-record commands), `milestone.record-exists` (`milestone create`), `write.already-present` (a rename's destination), `create.already-exists`, `migrate-corpus.destination-collision`. The six borrowed codes go back to their original meanings; every site goes through **one shared constructor**, and **a test iterates the commands**, so a further command cannot answer something else; the pinned contract row at `task finalize` gains a sibling for the symlink case; the two help texts change. The two collision cases at the milestone boundary keep `finalize.promote-clobber` (the earlier ruling). Name: the fixer proposed `store.home-not-regular-file`; the human did not rule the name — the fixer settles it in the family's grammar and reports it. **Over** moving the code only where the borrowed code's remedy is wrong for a link (a judgement per command; two commands answering one shape differently) and over withdrawing the ruling. Each of the fourteen is an intended change against rc.24 for the regression set's list.
- **Item 9 — option A; his words: "Yes agreed".** `jigc migrate-corpus` (in place) meeting a managed doc whose file is a symlink **blocks like every other block the command has today**: the run stops at that doc, names it under the new "not an ordinary file" code, and leaves the link untouched. The earlier option's sentence "the rest of the corpus still migrates" was the orchestrator's and described a behaviour the command has for no block; **continuing past blocked docs is recorded for the 1.x fix pass (M57)** as a change to the command as a whole. **Over** blocking that doc and carrying on with the rest (it would change how a migration proceeds for every kind of block; exit status and a half-done migration unexamined).

**Queue in the second tree after item 21's fixer, one fixer, one commit each, in this order:** item 15 (two route texts and their tests) → items 7 + 8 (the one code at fourteen commands, shared constructor, iterating test) → item 9 (`migrate-corpus` blocks a symlinked doc under that code; the pinning test flips; `design/finalize.md`'s declared bound for it goes) → a pending row under M57 for continuing past blocked docs. Then merge `fix/rc24-pre-opening` into `fix/rc24-tier1`. **[2026-10-07: the queue ran as far as items 7 + 8. Item 9 halted (below), so its step is not built, and the M57 row is not written — it rested on the premise that proved false. The merge waits for the ruling on item 9.]**

## Main tree — the tooling suite's Linux failures (fixer returned 2026-10-06)
`3129ab91`, 4788/0 **[2026-10-07: this commit and `473ddd1b` are on `fix/rc24-tier1`, not on the branch this log is committed on; `tooling-tests/placed_executable.rs` exists there only]**. Cause: a test copied a script with `fs::copy` and executed it; a child forked by another thread during the copy inherits the write handle, and Linux refuses to execute a file with any open writer (macOS does not check). Reproduced in `dev/runner-faithful --commit 473ddd1b --cpus 4`: 8 of 67 failed. Class: twelve sites in six tooling suites, all converted to `tooling-tests/placed_executable.rs` (a child `cp` does the write); held by a deterministic control on Linux, a timing arm, and a source fence over `tooling-tests/`. After: `g_tooling` four runs on Linux, 280 passed each. **[2026-10-07, read against the repository: `3129ab91`'s message records *three* runs of `g_tooling` on Linux after the change, 280 passed in each; *four* is the number of Linux runs — the one before the change included — on which the product's test named next was red.]** The product's failing CI test, untouched: `setup_install_pathspec_guard::where_git_cannot_answer_setup_refuses_before_its_first_write` (`g_migrate`).

**Left open — ledger rows at the opening:**
1. The product suites have the same write-then-execute pattern, unconverted: 96 executable-bit grants, plus 3 in `support/`.
2. `support::rust_source::code_only` misreads everything after a `'"'` char literal; 27 files contain one.

## Second tree — item 21, the record of the settings entries `setup` added (fixer returned 2026-10-06)
`1c7d391c` on `fix/rc24-pre-opening`, 4789/0. `fix(uninstall): the teardown removes the settings entries setup recorded, and leaves an identical entry that was already there`.

**Record:** `.jigc/settings-entries.json` — `{format: 1, file, added: {allow, deny, hooks[{event, command}]}}`, 760 bytes in the driven case; the eleventh install member (`install_tracked_paths`: replacing writer, `Refuses`, `OwnContent::SettingsRecord`), rows in the fences of `setup_install_pathspec_guard.rs`, `replacing_writers_never_follow.rs`, `setup.rs`. Its write failure reuses the code `setup.inject-allowlist`; no code, flag or JSON key is new.

**Cells:** only jigc's entries, settings tracked → all removed · one character off → kept · added after `setup` → kept; of an identical twin one occurrence goes · tracked→ignored and ignored→tracked → nothing removed, all named; a `setup` re-run after tracked→ignored deletes the stale record in its install commit · `--force` → removes every identical entry, the user's included.

**Upgrade:** a later `setup` claims nothing for an older install; it records only entries it adds itself (reading git history would be inference). So an install made by rc.24 and torn down by the candidate leaves and names its settings entries unless `--force`.

**Class:** five removals in three files; the three settings removals converted; the `CLAUDE.md` section and the `pre-commit` block stay outside (their content names jigc).

**Driven (release build, root with a space):** the tracked round trip; ignored then `jigc uninstall --force` as printed; the no-record upgrade and teardown; the stale-record removal note.

**Decided beyond the brief:** `setup` requires both files tracked, deletes a stale record, restores an uncommitted-deleted record from `HEAD`. One item-18 cell flips (which one: in the commit's body).

**Left open — ledger rows at the opening:**
1. Left entries are named on stderr only, not in `--format json`.
2. The record's write failure reuses `setup.inject-allowlist` — the human's to reverse.
3. A settings file that leaves git and returns with no `setup` between is trusted again.
4. An entry removed and re-added by hand stays claimed.
5. A failure between the merge and the record under-claims.
6. Layouts where the home is no work tree should now leave and name where rc.24 removed — not driven.
7. A plain `uninstall` with no install now warns about the user's own identical entries.

**Doors:** `jigc setup`, `jigc uninstall`, `jigc uninstall --force`.

## Second tree — items 15 and 7+8 fixed, item 9 halted (fixer returned 2026-10-07)

**Item 15 — `a2ccdac0`, 4790/0.** `fix(finalize)`: the refusal over a deleted committed doc names the exit that replaces it — commit the deletion, drop the task, start again — and `jigc unmanage` says to commit the deletion first. Class (sentences naming "commit the deletion" as an exit): none in the product, the ruled exit had never been built. Of the held arm's five cells, a task's doc under `HEAD` is converted; four are out, with reasons in the commit. Driven as printed on a release build, root with a space, to the landed commit; control `finalize.base-mismatch`. Doors: `task finalize` (route text), `unmanage` (ack text).

**Items 7+8 — `23dfe3e8`, 4795/0.** `fix(store)`: one code, `store.home-not-regular-file`, through one constructor, with a suite that iterates the commands. It sits beside `store.not-found` as the store's invariant; the predicate is true of a link, a directory and a special file alike. Derivation: the 20 production callers of `home_entry`/`foreign_home_entry` — 10 refuse, 10 out with reasons — then each command driven. **Fifteen commands, not fourteen:** `doc author` is the create-only gate's second door; the others are `task finalize`, `milestone finalize`, `rename`, `relocate`, `config set docs-root`/`placement-root`, `doc rename`, `doc create`, `milestone add-task`/`add-from-spec`/`discard`/`create`, sub-task `task discard`, `migrate-corpus` (relocation destination). Also moved, declared in the contract: the finding's key goes to the entry's path at six commands; the JSON arm goes from flattened to envelope at seven. Driven on release: `task finalize` (+`--dry-run`), `rename` destination, `doc rename`, `milestone create`, `add-task`, `doc create`, `config set docs-root`.

**Item 9 — HALTED, nothing committed.** The ruling's premise (*`migrate-corpus` stops at its first blocked doc*) is false: only a halt of the transform's fold stops a run; every per-doc refusal — pre-fold, and this code's own relocation arm — lets the other docs migrate (`crates/cli/tests/corpus_migration.rs`, *1 migrated, 1 blocked*; checked by the orchestrator, lines 457 and 1474). Fork for the human: (a) a per-doc refusal like its siblings — no new mechanism, no M57 row (the fixer's recommendation); (b) stop the run at that doc — new behaviour, and the one code's two arms would then differ. **Ruling (the human, 2026-10-07): option (a), the fixer's recommendation** — a symlinked doc is refused per doc, like its siblings; the other docs migrate, the doc is listed as blocked, the run exits non-zero. No new mechanism, no M57 row.

**Item 9 — built, by the commit that carries this line (its message carries the gate's totals).** `fix(migrate)`: `migrate-corpus` refuses a doc whose home is not a regular file — that one doc — under `store.home-not-regular-file`, keyed at its home; the others migrate and commit; exit non-zero; `--dry-run` says the same. One probe, at the one seam that queues a doc for the fold; a doc the run only reads (already current) is not asked. Class (this command's touches of a doc's home in the worktree): nine — two write, one probes, one lists names, five follow a link; of the five, the candidate read (it fed the write) and the recovery audit's read (it fed the commit) are converted, three only classify and are out. **The recovery arm committed a link:** over an uncommitted link to current bytes it exited 0 with `1 recovered from an earlier run` and the user's link committed in the doc's place — closed. Driven on a release build, root with a space: the dry run, the applying run, `--format json`, the route to the landed migration, the current-doc cell, the recovery cell. Doors: `migrate-corpus` (a status that was 0 is 1 over a linked doc it would write; the recovery arm no longer stages a link).

**Left open by this fix — ledger rows at the opening:**
1. Behind an uncommitted link to current bytes, over a `HEAD` still below the current version, the run reports the doc `already current` at exit 0 — whether it should refuse is the human's.
2. The recovery audit's *is the relocated source still there* follows a link: a dangling link at a relocated doc's old home reads as *moved* — not driven.
3. A dangling link or a directory at a folder home is enumerated by name, skipped as unreadable and named nowhere — as before; read off the walk, not driven.
4. A doc the run does not write is not asked what its home's entry is — the fixer's choice, the human's to reverse.

**Left open — ledger rows at the opening:**
1. The item-15 exit is not driven at the milestone boundary or for a migration task's doc.
2. `AMBUSH_CONTRACTS`' new row is `Exempt` — the fixer's choice, the human's to reverse.
3. Pending row 4's note ("the fold stops at its first blocked doc") is true of fold halts only. **[2026-10-07: corrected by the record commit that carries this log — the row now says so in a dated note. This line becomes no ledger row.]**
4. `config set`, `relocate` and `migrate-corpus` print the finding without an `at:` line, as before.

**For the lessons:** the fixer's first two gates were red from stale test binaries built under a target path containing `..`, not from the code — a private target directory is named by a normalized path.
