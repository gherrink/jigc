# The fixers' decisions, one question each — the sheet

Prepared 2026-10-06 on `fix/rc24-tier1` at `b913e602`, from the findings ledger's closing section, groups (a) and (b), the fixer reports, `DECISIONS.md` and the code.

**How far to trust it.** Nothing here was driven: no build, no test, no binary. Every *Situation* is read from the code, a named test, a commit message or a fixer's report. What rc.24 did is taken from those reports. Costs of switching are estimates and marked *unverified* where I did not read the site. No product source changed after `0f34d8f0` (`git diff --stat 0f34d8f0..HEAD` over `crates/*/src`, packs, guides and adapters is empty), so the code is what the ledger describes; **no item has been settled since the ledger was written**.

**The review rows** a reversal is matched to: 1 install members · 2 teardown · 3 worktree registrations · 4 home occupancy · 5 home shape · 6 reconciliation · 7 the linked-worktree guard.

**Items that are one decision seen more than once.** 10 and 4 (4 is one cell of 10). 4, 5 and 6 (a discarded sub-task's worktree). 7, 8 and 17 (which code names a state; 10 answered it the other way). 7 and 11 (one guard: its code, and whether it refuses at all).

## Index

| # | ledger id | plain title | reversing changes code? (row) | recommendation |
|---|---|---|---|---|
| 1 | `104a7d4b`-2 | `setup` gets stuck after one of its files was deleted | yes (1) | fix now |
| 2 | `ef9456b5`-2 | a change hidden by a git flag blocks `setup` at every install file | yes (1) | confirm |
| 3 | `4fa1c0f5`-1 | `uninstall` can ask you to clear the log twice | docs; a route string (2) | declared bound, plus one route sentence |
| 4 | `e842342e`-3 | landing refused while a discarded sub-task's worktree has staged files | yes (3) | confirm |
| 5 | `e842342e`-4 | which command asks before a discarded sub-task's worktree goes | yes if moved (3) | defer to 1.x |
| 6 | `e842342e`-5 | a discarded sub-task's unstaged files are deleted at landing | yes if refused (3) | declared bound — the human's to declare |
| 7 | `5f5b273a`-1 | the code for "this doc's home is not an ordinary file" | yes, small (5, 4) | confirm, ruled with 8 and 17 |
| 8 | `5f5b273a`-2 | the milestone-record commands use another code for the same link | yes, small (5, 6) | confirm, ruled with 7 |
| 9 | `5f5b273a`-3 | `migrate-corpus` silently replaces a symlinked doc | yes (5) | reverse — block |
| 10 | `e842342e`-1 | `milestone finalize` refuses before landing, under a new code | yes (3) | confirm |
| 11 | `5f5b273a`-4 | moving the docs folder refuses when a doc is a symlink | yes (5) | confirm |
| 12 | `4A-2` | a record command refuses after a branch switch, with a route that does nothing | yes (6) | route now, the arm to 1.x |
| 13 | `4B-3` | a mistyped finalize is sometimes told to redo the doc work | no; alternative adds a record (7) | confirm |
| 14 | `4B-4` | a sub-agent cannot file a finding until the milestone lands | no; a design change (7) | defer to 1.x |
| 15 | `4C-10` | after `unmanage`, a new doc of the same name is refused | yes (4) | confirm, as a written exception |
| 16 | `4E-12` | a repository that ignores `.jigc/` is called unsupported | docs; else (1, 2) | declared bound |
| 17 | `4E-13` | git fails: `setup` refuses even with `--force`, under the "dirty" code | yes (1) | confirm `--force`; give it its own code |
| 18 | `4E-14` | a `CLAUDE.md` link to an ignored file is refused | yes (1) | reverse |
| 19 | `4E-5` | `uninstall` leaves a hook it cannot recognise | yes (2) | confirm |
| 20 | `4F-3` | re-linking a worktree in a moved repository takes two steps | yes, small (3) | confirm |
| 21 | `4E-1` (b1) | `uninstall` removes a permission entry you already had | yes (2, 1) | fix now, after a verify-real |
| 22 | `4C-1` (b2) | content staged in the index only is replaced when a task updates the doc | yes (4) | declared bound, the guard to 1.x |
| 23 | `W-1` (b3) | an edit made before a task's first write is carried, and can be overwritten | docs; else (6) | declared bound |

**Order to ask in.** First, the items whose ruling can change code the coming round's hunts will read, grouped so that one answer informs the next:

10 → 4 → 6 → 5 → 17 → 7 → 8 → 11 → 9 → 18 → 15 → 1 → 2 → 19 → 20 → 12 → 21 → 22

Then the rest, where both the recommendation and the cheap alternative are a document or a later row:

13 → 3 → 16 → 14 → 23

---

## 1 · `104a7d4b`-2 · `setup` gets stuck after one of its files was deleted

- **Situation.** A file `setup` installs and git tracks (say `.jigc/AGENT.md`) is deleted from the working tree, the deletion not committed. `jigc setup` runs. Its check before the first write looks only at paths that exist (`install_candidates`, `crates/cli/src/setup.rs`), so it passes; the install is written; the check before the commit then refuses — exit 1, `setup.dirty-install-path` — over the file jigc just wrote. The fixer says a second run completes; the audit (E2E F9, 5), over a deleted committed link, saw it refuse again until `git commit -- <path>`. *Read, not driven.*
- **rc.24.** The same.
- **Decided.** Nothing: left as "the human's call" by the `104a7d4b` fixer.
- **Alternatives.** (a) the commit-time check skips a path that was absent before the run — a predicate arm and cells, *unverified*; (b) the pre-write check refuses over it; (c) declare it.
- **Clause.** Second: a refusal after writing, whose route points at jigc's own file. No bytes lost. Inside scope — my reading.
- **Changes code?** Yes — `setup`; row 1.
- **Recommendation.** Fix now, (a). The rule says jigc refuses *before* writing; row 1 will find this again.

## 2 · `ef9456b5`-2 · A change hidden by a git flag blocks `setup` at every install file

- **Situation.** A tracked install file (`CLAUDE.md`, `.claude/settings.json`, `.jigc/AGENT.md` …) carries git's `assume-unchanged` or `skip-worktree` flag and differs from the last commit; `git status` calls it clean. `jigc setup` asks git again without the flag and refuses before writing — exit 1, `setup.dirty-install-path`, printing the two commands that clear the flags. A flagged file with no change is not refused. Tests: `a_change_hidden_by_an_index_flag_refuses_at_every_tracked_member`, `an_index_flag_over_unchanged_bytes_is_not_a_subject`.
- **rc.24.** Exit 0: in a file `setup` replaces the hidden edit was destroyed; in one it merges into (`CLAUDE.md`) it was swept into the install commit.
- **Decided.** The fixer (`ef9456b5`) covered every install file, though the human's ruling named ignored files and said merged-into files were unchanged; kept by `676eff57`. `unseen_by_status`.
- **Alternatives.** Take merged-into files out: one filter there plus that test's expected list.
- **Clause.** First — "commits content the user did not ask for". Inside scope — my reading.
- **Changes code?** Yes — `setup`; row 1.
- **Recommendation.** Confirm. Reversing re-opens an exit-0 commit of an edit git was told to hide.

## 3 · `4fa1c0f5`-1 · With the invocation log on, `uninstall` can ask you to clear the log twice

- **Situation.** Knob `invocation-log` on. `jigc uninstall` refuses while `.jigc/logs/invocations.jsonl` exists (exit 1, `uninstall.untracked-workbench-file`: move it out or delete it). The teardown never recreates the log, but every other jigc run does — the commands two other teardown refusals route at, and the installed pre-commit hook on any `git commit`. A log moved out too early comes back and the teardown refuses once more. The help gives the order that works in one pass: knob off, `git add` the config, move the log, uninstall. The refusal's own route does not mention the knob (*read*).
- **rc.24.** The teardown took the log at exit 0, unnamed.
- **Decided.** The human ruled the block. The fixer listed four options for this bound and none was picked (`4fa1c0f5`, restated `8b214ad5`).
- **Alternatives.** (a) accept as stated; (b) report the log last — it already is checked last (*read*); (c) the route says "turn the knob off first" — one string; (d) the knob-off `config set` does not log itself — *unverified*.
- **Clause.** Second; nothing is lost.
- **Changes code?** Docs only for (a); (c) `uninstall`, row 2.
- **Recommendation.** Declared bound, with (c): the refusal should itself say the order.

## 4 · `e842342e`-3 · Landing a milestone is refused while a discarded sub-task's worktree still has staged files

- **Situation.** In a fan-out, one sub-task is dropped with `jigc task discard <sub>`; its worktree under `.jigc/worktrees/<sub>` is still there with `git add`-ed files. `jigc milestone finalize <m>` refuses before landing — exit 3, `milestone.unlanded-work`, keyed at the worktree — and prints `git -C <worktree> stash`, then the same finalize, or `jigc milestone discard`. Tests: `the_boundary_refuses_over_what_a_settled_sub_tasks_registration_holds`, `a_settled_sub_tasks_staged_path_is_kept_by_the_stash_the_refusal_prints`.
- **rc.24.** Exit 0; the staged files went with the worktree and were named afterwards as "not recoverable".
- **Decided.** The fixer (`e842342e`), inside the words of the human's ruling on worktree registrations but never put to him by name. `unlanded_work`, `crates/cli/src/milestone.rs`.
- **Alternatives.** Reverse: `staged_is_held` becomes `!checkout` there — one line plus the two tests; the files are then destroyed at exit 0 again.
- **Clause.** First. Inside scope: "discard one, land the rest" is normal use — my reading.
- **Changes code?** Yes — `milestone finalize`; row 3.
- **Recommendation.** Confirm. It is one cell of item 10; reversing ships the loss the human refused at the three sibling doors.

## 5 · `e842342e`-4 · Which command should ask before a discarded sub-task's worktree is thrown away

- **Situation.** `jigc task discard <sub>` drops the sub-task, leaves its worktree and asks nothing about what is in it (the fixer's report; *read, not driven*). The question comes later, at `jigc milestone finalize`: it refuses over staged files or unreachable commits there (item 4) and takes unstaged and untracked files without asking (item 6).
- **rc.24.** Nobody asked; finalize took everything at exit 0.
- **Decided.** Nothing. The fixer named the question (`e842342e`) and left finalize as the door that refuses.
- **Alternatives.** `task discard` answers for the worktree itself: refuses without `--force` while it holds anything. That is a new refusal at a command that exits 0, a blocking code for it (the worktree doors' codes may not be shared between doors, by a fenced rule), and a decision on what the consent then does to the worktree — *unverified*, not small.
- **Clause.** First, through item 6.
- **Changes code?** Yes if moved — `task discard`, `milestone finalize`; row 3.
- **Recommendation.** Defer to the 1.x fix pass. It is item 6 seen from the other side: decide 6 first.

## 6 · `e842342e`-5 · A discarded sub-task's unstaged and untracked files are deleted when the milestone lands

- **Situation.** As item 5, but the dropped sub-task's worktree holds edits never `git add`-ed, or new untracked files. `jigc milestone finalize` lands at exit 0, removes the worktree and names those files on stderr afterwards. *Read, not driven* (`unlanded_work`'s doc comment, bound 1).
- **rc.24.** The same.
- **Decided.** The fixer did not widen the refusal (`e842342e`), on the human's M46 ruling that a *landed* sub-task's leftover bytes — build output — are narrated, not refused. That ruling was never taken for a discarded sub-task.
- **Alternatives.** (a) refuse here too — one predicate and cells, and it fires on most "discard one, land the rest" runs; (b) move the consent to `task discard` (item 5); (c) declare it.
- **Clause.** First: bytes no git object holds, destroyed at exit 0, in normal use. Inside scope unless declared out — my reading.
- **Changes code?** Yes for (a) or (b), row 3; docs only for (c).
- **Recommendation.** Record as a declared bound beside M46's, with its reach — but only the human can declare it: it sits inside the clause's words. If he will not, (a) is the fix that is affordable now.

## 7 · `5f5b273a`-1 · Which error code names "this doc's home is not an ordinary file" outside finalize

- **Situation.** `jigc rename` of a doc whose home, or a referrer's, is a symlink; `jigc config set docs-root|placement-root` or `jigc relocate` moving such a doc; at `jigc milestone finalize`, two docs promoting to one path, or a doc promoting onto a path a sub-task worktree staged. Each refuses before writing with `finalize.promote-clobber`, keyed at the path (exit 1 at rename, 3 at the boundary); `--format json` carries the keyed findings envelope. Suites: `store_door_home_shape`, `milestone_promote_guards`.
- **rc.24.** Exit 0: wrote through the link, moved the link, or landed and lost a doc.
- **Decided.** Each fixer chose the existing code (`5f5b273a`, `f031d31e`, `048724d0`). `engine::finalize::store_home_refusal`.
- **Alternatives.** A new code for home shape — one constructor, two registry rows, docs; `combine.code-collision` or `join.same-doc-clash` for the two boundary cells.
- **Clause.** Third: a driver keys on the code. Not the first.
- **Changes code?** Yes, small — rename and the movers (row 5), the boundary (row 4).
- **Recommendation.** Confirm: one state, one code, keyed at the path. Rule it with 8 and 17, which answer the same question differently; after 1.0.0 a code cannot move.

## 8 · `5f5b273a`-2 · The milestone-record commands use a different code for the same symlink

- **Situation.** A milestone record's file has been replaced by a symlink. `jigc milestone add-task` (also `discard`, `finalize`, a sub-task's `task discard`) refuses — exit 1, `reconciliation.conflict-block`, with a message and route about the link; nothing written (`record_shape_block`, `crates/cli/src/milestone.rs`). Suite: `store_door_home_shape`. A JSON driver still gets an error string with no key at these doors (audit E2E F7, open).
- **rc.24.** A link to an identical copy: exit 0 and a commit holding the link. A link to different content: already `reconciliation.conflict-block`.
- **Decided.** The fixer's choice (`5f5b273a`): the identity those doors already gave an edited record.
- **Alternatives.** `finalize.promote-clobber`, as item 7 uses for the same shape — one constructor and cells, and it splits one door's refusal over two codes; or `milestone.record-exists`, which `milestone create` raises over a link.
- **Clause.** Third.
- **Changes code?** Yes, small — the record doors; rows 5 and 6.
- **Recommendation.** Confirm: it keeps rc.24's code for the differing-link case. Same decision as 7 at another door — three codes now name one shape, and that deserves one ruling.

## 9 · `5f5b273a`-3 · `migrate-corpus` silently replaces a symlinked doc with a regular file

- **Situation.** A managed doc's home is a symlink to a file elsewhere. `jigc migrate-corpus` writes the migrated doc as a regular file where the link was and commits that, exit 0. The link's target keeps the old, un-migrated content; nothing says a link was replaced. Test: `store_door_home_shape::the_corpus_migration_lands_a_regular_file_and_writes_through_nothing`.
- **rc.24.** The same.
- **Decided.** Left by the fixer as "the human's call" and declared in `design/finalize.md` (`5f5b273a`). `526141e6` made the *relocation* arm block under `migrate-corpus.destination-collision`; this in-place arm is unchanged.
- **Alternatives.** (a) block that doc and leave the link, as every other writer of a doc's home now does — ask `home_entry` first, flip the test; which existing code is honest is *unverified*; (b) migrate and name the replaced link; (c) keep it declared.
- **Clause.** First, "update incorrect things": a link dropped unnamed. A symlinked doc home is the edge of "ordinary" — my reading.
- **Changes code?** Yes — `migrate-corpus`; row 5.
- **Recommendation.** Reverse to (a). One rule at every door that writes a home in place; the odd one out is what the home-shape row will report.

## 10 · `e842342e`-1 · `milestone finalize` refuses before landing when a sub-task worktree holds work it would drop — under a new code

- **Situation.** A sub-agent committed inside its worktree (a commit no branch reaches), or staged files in a worktree whose directory or `.git` link is gone. `jigc milestone finalize` refuses — exit 3, nothing committed, one `milestone.unlanded-work` per path — and prints the command that keeps or lands the work (`git branch kept/<id> <sha>`, `git reset --soft <base>`, a re-link, `git stash`). This door has no `--force`. Suite: `worktree_registration_anchor`.
- **rc.24.** Landed at exit 0 without the work, dropped the registration, named it afterwards.
- **Decided.** The orchestrator, stretching the human's ruling for `provision`, `discard` and `uninstall` (refuse; `--force` consents) to this door — the ruling itself said a landed finalize "names what it drops". New code because the sibling codes' routes end at `--force` (`DECISIONS.md`, 2026-10-05, round 3).
- **Alternatives.** (a) land and name, as the ruling worded it — remove the guard; (b) keep the refusal under a sibling code, whose route this door cannot honour.
- **Clause.** First. Inside scope.
- **Changes code?** Yes — `milestone finalize`; row 3.
- **Recommendation.** Confirm. Take-and-name is the exit-0 loss the human refused at the other three doors. Owed: the code named in MIGRATING and the worked examples (`4F-4`).

## 11 · `5f5b273a`-4 · Moving the docs folder refuses when one doc is a symlink

- **Situation.** A doc's home is a symlink. `jigc config set docs-root <dir>` (or `placement-root`): exit 1, `config.repoint-failed` carrying `finalize.promote-clobber`, the whole move undone. `jigc relocate`: that doc is a blocked row. Suites: `store_door_home_shape`, `tests/relocate.rs`.
- **rc.24.** Exit 0; the link was moved as a link, a relative one then pointed nowhere, and `jigc doc list` failed for the whole store with no code.
- **Decided.** The fixer (`5f5b273a`), beyond what the orchestrator's queued entry weighed — it named `rename` only. `refuse_foreign_source`, `crates/cli/src/relocate.rs`.
- **Alternatives.** Remove that one guard; the exit-0 broken store comes back.
- **Clause.** Second, both ways: a command that exited 0 now refuses, and what it left behind was a store jigc could not list. Edge of scope, as in item 9 — my reading.
- **Changes code?** Yes — `config set`, `relocate`; row 5.
- **Recommendation.** Confirm. The rc.24 exit 0 was not a working command. It is item 7's guard; 7 asks only what it is called.

## 12 · `4A-2` · After a branch switch in a line-ending-converting checkout, a milestone-record command refuses with a route that does nothing

- **Situation.** Line-ending conversion is on (`core.autocrlf`, or `eol=crlf`). jigc has written a milestone record and remembers its hash. A branch switch and back, or a stash, makes git rewrite the file in converted form. `jigc milestone add-task` → exit 1, "edited out of band"; the printed `git checkout -- <record>` changes nothing, because git sees no edit. `jigc unmanage <record>`, then the command, works. Driven by the area A fixer.
- **rc.24.** The same.
- **Decided.** Nothing — "needs a decision" (`676eff57`'s report). By an M55 rule this door never asks git while it remembers a hash, so a *pulled* edit to the record blocks too (`reconcile_record_preflight`).
- **Alternatives.** (a) on a mismatch, ask git whether the record differs from HEAD and adopt if not — this also turns the pulled-edit block (`(R3, F4)`, tier 2) into an adoption; (b) the route names the exit that works — a string; (c) declare.
- **Clause.** Second: a refusal whose route does not work, in a configuration the first clause calls ordinary.
- **Changes code?** Yes — the record doors; row 6.
- **Recommendation.** (b) now; (a) deferred to the 1.x line-ending row, decided with `(R3, F4)`.

## 13 · `4B-3` · From a linked worktree, a mistyped finalize is sometimes told to redo the doc work

- **Situation.** A task holds a staged doc and `jigc task finalize` is typed in a linked worktree: refused, exit 3, `finalize.linked-worktree-doc`. If the main checkout's index holds staged files the task's own snapshot does not cover, the route says: read the doc back, start again from the main checkout, discard this task — not "go there and finalize". The same is printed for a task that was started in the main checkout and only finalized from the wrong directory. Test: `linked_worktree_doc_home::the_backstop_never_routes_a_task_at_a_main_checkout_whose_staged_work_it_would_commit`.
- **rc.24.** No guard; the shorter path could commit the user's own staged file with the doc.
- **Decided.** The fixer (`ac0f63b1`). `HomeLanding`, `crates/cli/src/task.rs`.
- **Alternatives.** Record at the start of a task which checkout it was made in, and route precisely — a new stored fact in the task's area; *unverified*.
- **Clause.** The route protects the first (no unasked content in a commit); the extra work touches the second.
- **Changes code?** No to confirm; the alternative adds a record (row 7).
- **Recommendation.** Confirm. The precise route belongs to 1.x, with the parked linked-worktree design.

## 14 · `4B-4` · A sub-agent in a fan-out worktree cannot file a finding until the milestone lands

- **Situation.** Inside `.jigc/worktrees/<sub>`, a sub-agent hits a jigc defect and runs `jigc start --workflow report-jigc-feedback "…"`: exit 1, `finalize.linked-worktree-doc`. The route: land the milestone (`jigc milestone finalize <m>`), then file from the main checkout. A doc the sub-task writes for its own task still lands with the milestone. Test: `linked_worktree_doc_home::a_fan_out_sub_task_writes_its_docs_and_an_ordinary_task_there_does_not`.
- **rc.24.** The task was created there, and its finalize then refused on `repo.head-detached`.
- **Decided.** The fixer (`e0f00278`) made the route true and left the capability open: "a design question for the findings channel".
- **Alternatives.** A sub-task files a finding through its own milestone's join — new design in `design/findings-channel.md`; or the orchestrator adds a report sub-task to the milestone — whether that already works mid-milestone is *unverified*.
- **Clause.** Third, usable by agents: the blind trial's fan-out arm is where it shows.
- **Changes code?** No now; a design change later (row 7).
- **Recommendation.** Defer to the 1.x fix pass. The finding is held, not lost; the route says so.

## 15 · `4C-10` · After confirming a doc's deletion with `jigc unmanage`, a new doc of the same name is refused at finalize

- **Situation.** A committed doc's file is deleted and the deletion not committed. jigc reports it missing and offers: restore it, or confirm with `jigc unmanage <path>`. The user confirms. A task creates a doc under the same id. `jigc task finalize` → exit 3, `finalize.promote-clobber`: git still holds the old doc there. Printed exits: rename the new doc (`jigc doc rename …`), or restore the old file and update it. None replaces the old doc under its id. Suite: `milestone_promote_guards`; this exact sequence as a test is *unverified*.
- **rc.24.** Exit 0; the new doc replaced the committed one.
- **Decided.** The fixer (`78e8ded1`), as the consequence of the ruling its commit records as the human's: "an identity git holds is occupied".
- **Alternatives.** (a) keep; (b) a third exit, "commit the deletion first" — a route sentence, but the door's rule is to teach no removal, and whether it then lands is *unverified*; (c) let the confirmation free the home — a stored record.
- **Clause.** Second: the one sequence jigc's own route produces that newly refuses.
- **Changes code?** Yes — both finalize doors; row 4.
- **Recommendation.** Confirm, and write it into the regression set as an accepted exception.

## 16 · `4E-12` · A repository that gitignores `.jigc/` is called unsupported instead of made to work

- **Situation.** `.gitignore` lists `.jigc/` (or `.jigc/version`, `.jigc/config/`, `.jigc/.gitignore`). `jigc setup` exits 0. `jigc task finalize` exits 3, `finalize.stage-failed`, its route now naming the ignore rule; drop the rule, re-run, it lands. `jigc uninstall` exits 1 over jigc's own ignored files; `--force` works. Ignoring only `CLAUDE.md`, `.claude/` or `.jigc/AGENT.md` works at every door. Test: `setup_install_pathspec_guard`, cell 43.
- **rc.24.** The same exits; the finalize route named a stale index lock instead.
- **Decided.** The fixer (`a330b4f9`): say it in `design/validation.md` and QUICKSTART and fix the route — "a judgement call the human may want to reverse".
- **Alternatives.** (a) make the doors agree — finalize skips ignored paths (`existing_pathspecs`), uninstall decides by content: the project's jigc config is then silently missing from every commit; (b) `setup` says at install time that this layout cannot finalize — new advisory wording, *unverified*.
- **Clause.** Second. Nothing is lost and the routes work.
- **Changes code?** Docs only to confirm; (a) finalize and uninstall, rows 1–2; (b) row 1.
- **Recommendation.** Record as a declared bound; (b) as a 1.x row.

## 17 · `4E-13` · When git itself fails, `setup` refuses even with `--force` — under the same code as "you have uncommitted changes"

- **Situation.** git cannot answer: a corrupt `.git/index`, no `git` on `PATH`, a `safe.directory` refusal. `jigc setup`, with or without `--force`: exit 1 before any write, `setup.dirty-install-path`, quoting git and printing the command that failed. Test: `setup_install_pathspec_guard`, cells 41–42.
- **rc.24.** Installed blind: an uncommitted line in `.jigc/AGENT.md` was gone at exit 0, or at exit 1 after the loss.
- **Decided.** The human ruled "refuse before the first write". The fixer (`f0b1120a`) read that as covering `--force`, and reused the code because a new one was a halt. `engine::finalize::setup_unasked_install_finding`.
- **Alternatives.** (a) `--force` installs blind — the fixer drove it: in two of three cells it then failed later with bytes already replaced; (b) a code of its own — one string in that constructor, an inventory row, docs, cells; *unverified*.
- **Clause.** First, literally: "where jigc cannot tell it refuses before writing". The code is the third.
- **Changes code?** Yes — `setup`; row 1.
- **Recommendation.** Confirm the `--force` reading; reverse the shared code. The usual remedy under that code is `--force`, which this refusal does not honour — the reason item 10 got a new code.

## 18 · `4E-14` · A `CLAUDE.md` symlink to a gitignored file is refused, though a gitignored `CLAUDE.md` file is accepted

- **Situation.** `CLAUDE.md` (or `.claude/settings.json`) is a symlink to a file inside the repository that git ignores. `jigc setup`, with or without `--force`: exit 1, `setup.inject-reference`, nothing written. A link to a tracked file (`CLAUDE.md -> AGENTS.md`) works and is committed. A regular, gitignored `CLAUDE.md` gets jigc's section at exit 0 and joins no commit. Suite: `replacing_writers_never_follow`, m1–m3.
- **rc.24.** The link was followed and the file merged into, exit 0.
- **Decided.** The fixer (`55cb29fa`), from the round's brief — a link is followed only to a file a commit can carry — "a deliberate asymmetry, the human's to reverse". `link_end`, `crates/cli/src/setup.rs`.
- **Alternatives.** Follow it and treat the target as the ignored regular file is treated: merged, in no commit — a third outcome in `link_end`, its handling and cells; *unverified*, more than a line.
- **Clause.** Second: it works on rc.24, and the first clause names a symlinked `CLAUDE.md` as ordinary. A merge loses nothing either way.
- **Changes code?** Yes — `setup`; row 1.
- **Recommendation.** Reverse. Same bytes, same outcome for the commit; the refusal is new and protects nothing.

## 19 · `4E-5` · `uninstall` leaves a pre-commit hook it cannot recognise as its own

- **Situation.** `.git/hooks/pre-commit` begins like jigc's hook but is not it byte for byte — edited inside jigc's lines, or written by another jigc build. `jigc uninstall`: exit 0, the hook untouched, a line on stderr saying so; `jigc uninstall --force` removes it. A hook with the adopter's own lines *beside* jigc's block keeps those lines and loses the block. Tests: `uninstall_workbench_subject`, a unit cell in `setup.rs`.
- **rc.24.** The whole file deleted at exit 0, the adopter's lines with it.
- **Decided.** The fixer (`528d0205`), "the human's to reverse": an edit inside jigc's lines cannot be told from another build's hook.
- **Alternatives.** (a) delete as before; (b) recognise earlier builds' hooks — a list to maintain; (c) write an end marker into the hook from now on, so the block is always cut by its markers — install and teardown, *unverified*.
- **Clause.** First: a hook is in no git object. Inside scope.
- **Changes code?** Yes — `uninstall` (row 2); (c) also `setup` (row 1).
- **Recommendation.** Confirm. Record the bound: once the hook's text changes between releases, every older install's hook is left behind. Whether it changed since rc.24 I did not establish.

## 20 · `4F-3` · In a moved repository, re-linking a sub-task worktree takes two steps

- **Situation.** The repository directory was moved or copied while a fan-out was open. A sub-task worktree's `.git` file still points at the old place, and its registration holds staged files. `jigc milestone finalize` refuses (exit 3, `milestone.unlanded-work`); the line says to move that `.git` entry aside by hand. The re-run then prints the one `printf 'gitdir: …' > <worktree>/.git` that re-links it, and after that the finalize lands. Driven by the area F fixer; suite: `worktree_registration_anchor`.
- **rc.24.** Landed at exit 0 without the staged files.
- **Decided.** The fixer (`f1b84ef1`, `7e5ba186`) kept the rule that no printed command overwrites a `.git` entry — "decision for the human".
- **Alternatives.** Print the overwrite: a route string and the control test; one step, and a printed command that destroys a `.git` which turns out to be a real repository.
- **Clause.** Second: the route works as printed, in two steps. A repository moved mid-fan-out is the edge of scope — my reading.
- **Changes code?** Yes, small — the line the four worktree doors share; row 3.
- **Recommendation.** Confirm. One extra hand step in a rare state is cheaper than a printed command that can destroy a repository.

## 21 · `4E-1` (b1) · `uninstall` removes a permission entry you already had

- **Situation.** `.claude/settings.json` already holds an entry identical to one jigc installs — the adopter's own `Bash(rm -rf:*)` under `permissions.deny`, or `Bash(jigc:*)` under `allow`. `jigc setup`, then `jigc uninstall`: exit 0, both gone. A modified file where the settings file is tracked; silent where it is untracked or ignored. Driven on the release build by the area E fixer; `remove_deny` and `remove_allowlist` (`crates/cli/src/adapter.rs`) match whole entries (*read*).
- **rc.24.** Those functions are untouched by the pass (*read, not driven there*).
- **Decided.** Nothing: found after the audit, not fixed — an exact fix needs a record of what was there first, and a new record was the round's halt.
- **Alternatives.** (a) `setup` records the entries it added — a new stored fact; the existing install record is per-clone and ignored, so a clone has none; (b) `uninstall` leaves permission entries in place and names them — no record; (c) declare.
- **Clause.** First: in an untracked settings file the entry is in no git object, and it is the adopter's own safety rule. Inside scope.
- **Changes code?** Yes — `uninstall` (row 2); `setup` for (a) (row 1).
- **Recommendation.** Fix before the candidate, after a verify-real: (b) now; (a) only with the human's go for a new record.

## 22 · `4C-1` (b2) · Content staged in git's index only is replaced when a task updates that doc

- **Situation.** A task has copied in a committed doc to update it. Different content for that doc is then staged in git's index while the working file stays as it was (`git update-index --cacheinfo`; or `git add`, then `git restore --worktree`). `jigc task finalize`: exit 0; the promote's own `git add` replaces the staged entry, and that content is in no commit (the blob should stay in git's object store until garbage collection — my reading, *not driven*). Driven by the area C fixer.
- **rc.24.** The same.
- **Decided.** Nothing: left as outside the handed class. `78e8ded1` closed the case of a newly *created* doc only.
- **Alternatives.** (a) one more question to git — does the index differ from HEAD at a promote destination — and a refusal over every promotion: a new guard on every finalize; (b) declare, with reach.
- **Clause.** First, only if staged-only content counts. Reaching the state takes deliberate git plumbing: a planted state — my reading.
- **Changes code?** Yes for (a) — both finalize doors; row 4.
- **Recommendation.** Record as a declared bound; (a) to 1.x. A new guard over every promotion is where this pass's regressions came from.

## 23 · `W-1` (b3) · A hand edit made before a task first touches a doc is carried along — and overwritten if the task writes the same part

- **Situation.** The user edits a committed doc and does not commit. A task then writes to that doc; its first write copies in the file as edited. At `jigc task finalize`, exit 0, the edit lands in the task's commit — unless the task wrote the same slot: then the task's text replaces it and the edit is in no git object. An edit made *after* the first write blocks instead. Stated in `design/reconciliation.md` → *What a task copied in* and MIGRATING item 8.
- **rc.24.** The same in a clone with no recorded baseline; with one, the edit blocked.
- **Decided.** The human ruled option A on 2026-10-05; this consequence was accepted with it, and the fixer "judged it inside the decision".
- **Alternatives.** (a) declare; (b) the first write says "uncommitted edits here are carried" — an advisory, *unverified*; (c) refuse the copy-in over an uncommitted edit — blocks a normal flow.
- **Clause.** First, on its edge: the task authored over text it could read.
- **Changes code?** Docs only for (a); (b) or (c) the write doors, row 6.
- **Recommendation.** Record as a declared bound, with reach; (b) as a 1.x row.
