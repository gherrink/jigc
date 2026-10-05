# Completion audit of the rc.24 fix pass at 5f5b273a — the auditors' structured returns, verbatim

Six independent code reviewers and one end-to-end tester. Findings are the auditors' hypotheses with their own repro blocks: verify-real (reproduce as a red test) before fixing.

# AREA: cross-cutting: interactions and scope honesty of the rc.24 fix pass (origin/main..HEAD, 19 commits, read across the fixes; debug binary built from HEAD 5f5b273a in a private target, driven in twelve throwaway rigs, with the published 1.0.0-rc.24 binary as control)

Verdict: red — no exit-0 loss or repository harm was found in the interactions I drove, but the pass's record does not cover its last six fix commits (including a second new finding code), one as-built claim overstates what the code closes, one sentence in the embedded guide is false as written, and one new refusal's route strands the reader's own milestone.

## XC-1 · MEDIUM · The shared logs do not record the six round-3 fixes: DECISIONS.md still says none is built, the second new finding code has no stated reason, and eleven fixer-declared human's-call items have no pending row

- where: DECISIONS.md:5-15 (top entry, 'None is built at this entry'; item 1 'write_version_stamp is still a plain write that follows a link'); DECISIONS.md:43 (ruling 1, 'Not built at this entry'); implementation/decisions-pending.md:87 (row '(D) The landed jigc milestone finalize envelope has no key…'); crates/cli/src/milestone.rs:4180 (FINALIZE_HELD_CODE = "milestone.unlanded-work"); crates/cli/src/render.rs:5848; commits 104a7d4b ef9456b5 4fa1c0f5 8d9c3afb e842342e 5f5b273a; last record commit 614d9c99
- driven: read only
- class: Record-versus-tree gap over the round-3 range. Count derived: six SHAs grepped against three logs, 18 lookups, 0 hits. The eleven undocketed items were counted by reading the six `left_open` sections of the round-3 fixer report and counting entries labelled HUMAN'S CALL / HUMAN CALL / DECISION FOR THE HUMAN / CONFIRM (104a7d4b: 1, ef9456b5: 2, 4fa1c0f5: 1, 8d9c3afb: 1, e842342e: 3, 5f5b273a: 3); the fixer report is a lead, so that count bounds what the authors flagged, not what exists. The stale-sentence list (items 2, 5, 6) is instances, unbounded — I did not re-read every sentence of the 2026-10-04 entries against the tree.

```text
READ, with greps. (1) The last record commit (614d9c99) precedes all six round-3 fix commits; `git show --stat` of each shows none touches DECISIONS.md, decisions-pending.md or project-history.md. Grepping the three logs for each of the six short SHAs returns 0 hits in every file. (2) DECISIONS.md's top entry queues five members and states 'None is built at this entry'; all five are built at HEAD (104a7d4b, 4fa1c0f5, 8d9c3afb, e842342e, 5f5b273a), and ruling 1 of the two-rulings entry ('Not built at this entry') is built as ef9456b5. Item 1's sentence that `write_version_stamp` 'is still a plain write that follows a link' is false at HEAD (crates/cli/src/setup.rs:439-467 asks `stamp_standing` before the write). (3) Rule 2 of the pass requires a stated reason and every registration home for a new finding code. `finalize.linked-worktree-doc` has both (DECISIONS fork entry; design/validation.md:901, design/command-output-contract.md:351, design/surface-contract.md:116, crates/cli/src/pack.rs:1052). `milestone.unlanded-work` is minted at crates/cli/src/milestone.rs:4180 and stated in design/finalize.md:359, but `grep -c unlanded-work DECISIONS.md implementation/decisions-pending.md` is 0/0 — its reason (no existing door code fit; flow48/flow49 fence shared blocking identities) exists only in the fixer's report. Its design homes match its sibling `milestone.zero-contribution` (finalize.md only), so the gap is the record, not the registration. (4) Behaviour the round-3 commits changed with no ruling recorded: the index-flag class (assume-unchanged / skip-worktree) converted at merged-into install members too (grep 'assume-unchanged' and 'skip-worktree' in DECISIONS.md: 0); `config set docs-root`, `config set placement-root` and `jigc relocate` now refuse over a link source; a settled sub-task's live worktree with staged paths now refuses at `milestone finalize`; `jigc task finalize` leaves a non-stamp `.jigc/version` and raises an advisory under `setup.version-stamp`, a code that was setup-only and blocking-only. (5) A pending row is now false: decisions-pending.md:87 (a) says what a dropped registration held 'is on stderr in both formats'; e842342e removed that clause (crates/cli/src/render.rs:5585 `stale: bool`, and the contribution label at render.rs:5875-5880 prints only 'its worktree directory is gone, no code counted'), while the doc comment at render.rs:5848 still says the line states 'what git's registration of it held'. (6) DECISIONS top entry item 5 cites finalize.md Declared bounds (3) as recording rename as a door not ruled on; at HEAD bound (3) is `jigc milestone create` (design/finalize.md:124).
```

## XC-2 · MEDIUM · The (R1, F1) as-built claim overstates the fix: with git unable to answer, `jigc setup` still replaces a file holding bytes no commit holds, and nothing records the open cell

- where: DECISIONS.md:21 ((R1, F1) paragraph, 'Restores: jigc setup never replaces bytes no commit holds without refusing first'); crates/cli/src/setup.rs (the `InstallSubject::Unknown` arm, as named in the ef9456b5 fixer report); no row in implementation/decisions-pending.md
- driven: driven (HEAD debug binary; control on published 1.0.0-rc.24)
- class: instance, unbounded — one trigger of the `Unknown` arm was driven (no git on PATH). Other ways git fails to answer (an ownership refusal in a container-mounted checkout, a corrupt index) were not driven and may or may not reach the same writes.

```text
DRIVEN on the HEAD debug binary, and on published rc.24 as control (identical outcome, so pre-existing and unchanged by the pass).
Setup: `dev/jigc-rig bare`; `jigc setup` (exit 0, install committed); append one line of prose to the tracked `.jigc/AGENT.md` (uncommitted; `git status --short` shows ` M .jigc/AGENT.md`).
Argv: `PATH=<an empty directory> jigc setup`
Observed: exit 1; `blocking · setup.install-hook — cannot install the pre-commit hook into the repo's hooks dir: No such file or directory (os error 2)` / `route: ensure the repo's git hooks directory is writable, then re-run jigc setup`. Afterwards `grep -c` of the planted line in `.jigc/AGENT.md` is 0 and `git status --short` is empty: the file was rewritten to jigc's own body before the run failed, the prose is in no git object, and the one finding printed names the hook, not the loss.
The ef9456b5 fixer reported this as a decision for the human (refuse before the first write when git cannot answer / ask each replacing member its oracle / leave it). It is not an exit-0 loss, and the trigger I drove is narrow. The finding is that DECISIONS states the class as restored without the bound, and the open cell has no pending row.
```

## XC-3 · MEDIUM · The embedded guide's new sentence 'An edit you made before the task first wrote the doc is carried into the task and lands with it' is false wherever a baseline is held

- where: crates/cli/guides/MIGRATING.md:60 (item 8; added by eb18e5fe); embedded into the installed skill by `jigc setup`
- driven: driven (HEAD debug binary; control on published 1.0.0-rc.24)
- class: instance, unbounded — one false sentence found by driving the cells it describes. I did not drive every sentence the pass added to the two guides; the others I read against driven behaviour (the setup paragraphs, the uninstall-log paragraph, the linked-worktree paragraph) held, with the bound in XC-5.

```text
DRIVEN on the HEAD debug binary.
Setup: `dev/jigc-rig committed-singletons` (the file-state record holds VISION.md's key, as in any checkout where jigc has landed the doc).
Argv: `jigc start --workflow single-task "task b"` (mints task-b); append one line to VISION.md by hand, uncommitted — before the task's first write; `jigc doc set-slot vision:vision#invariants --from-file - --task task-b` (ack: 'copied in for update'; `jigc doc show vision:vision --task task-b` contains the hand line, so the copy-in did carry it); author the commit doc; `jigc task finalize task-b`.
Observed: exit 3, `blocking · reconciliation.conflict-block — conflict on VISION.md: an external edit and this task's staged writes both changed it`, route 'jigc task discard task-b --force … or revert the external edit on disk'. Nothing lands.
The same order with the hand edit made before the mint blocks identically on rc.24 (control driven), so the behaviour is pre-existing and correct per the state machine; the sentence is what is new. It is true in two cells only, both driven: no key for the doc at the copy-in (the copy-in records the edited bytes), and a committed edit with a held key (exit 0, `reconciliation.absorb`). design/reconciliation.md states the claim scoped to the no-key case; the guide dropped the scope. A reader who trusts it meets a block whose two printed exits are 'discard the task' or 'revert your edit'. Related, driven: a task that copies a doc in and is then discarded leaves its copy-in key behind (record-if-absent), which moves a fresh clone from the cell where the sentence is true into the one where it is false.
```

## XC-4 · MEDIUM · From one of jigc's own fan-out worktrees, the linked-worktree guard's route sends a doc-only mint to the main checkout, where landing it blocks the reader's own milestone on `finalize.base-mismatch`

- where: crates/cli/src/render.rs:2233-2235 (`CodeOnlyCheckout::of_mint` is `differing` alone, no fan-out exemption; `of_reader` at 2238-2246 has one); crates/cli/src/start.rs:2058-2066 (`refuse_doc_only_mint_from_a_code_only_checkout`); commit e3a6ba58
- driven: driven (HEAD debug binary; control on published 1.0.0-rc.24)
- class: The routes of `finalize.linked-worktree-doc` when the standing checkout is a jigc fan-out worktree. Derived by grep: `CodeOnlyCheckout::of_mint(` has 4 call sites (start.rs:1683, 2036, 2066; migrate.rs:342) and `of_task` covers an ordinary task's doc write leaves; every one of those routes opens with `cd <main>` plus a mint there. I drove one (the doc-only mint). The `jigc migrate` mint and an ordinary task's write-leaf refusal from a fan-out worktree are the same shape by reading and were not driven.

```text
DRIVEN on the HEAD debug binary.
Setup: `dev/jigc-rig committed-singletons`; `jigc milestone create "Cache rework"`; two `jigc milestone add-task`; `jigc milestone provision cache-rework`.
Argv 1, from `.jigc/worktrees/area-high`: `jigc start --workflow report-jigc-feedback "<intent>"`
Observed: exit 1, `blocking · finalize.linked-worktree-doc — workflow report-jigc-feedback mints a task whose only product is a managed doc, and this checkout — the linked worktree at .jigc/worktrees/area-high — commits code only …` / `route: run it from the main checkout: cd <main>, then jigc start --workflow report-jigc-feedback "<intent>"`.
Argv 2, the route as printed: `cd <main>`; `jigc start --workflow report-jigc-feedback …`; author the finding and the commit doc; `jigc task finalize <id>` → exit 0, 'finalized … promoted docs/jigc-feedback/join-route-wording.md'.
Argv 3: `jigc milestone finalize cache-rework`
Observed: exit 3, `blocking · finalize.base-mismatch — the milestone was pinned to base … but HEAD is now …, and the commits landed since move more than milestone-record bookkeeping` / route: out-of-band git only (return HEAD to the base, finalize, re-land the newer commits; or re-provision and re-apply each sub-task's staged changes).
So the refusal's route works for the command it prints and strands the milestone the reader is a sub-agent of. The base-mismatch mechanism is unchanged by the pass (no hunk in the range touches `record_only_range`), and any commit on the main branch mid-milestone trips it; what is new is a refusal that routes a sub-agent there without saying so. Control on rc.24: the same mint from the fan-out worktree succeeded and its finalize refused `repo.head-detached` with a `git switch -c` route, so the earlier state was also broken — this is a changed failure, not a regression from a working path. No bytes are lost in either. The guide's parenthesis at MIGRATING.md:51 ('the worktrees jigc milestone provision cuts are jigc's own and work the way the milestone flow describes') reads as an exemption this mint does not get.
```

## XC-5 · LOW · The stated bound on 'the teardown never starts the log' undercounts: a plain `git commit` mints the log through the pre-commit hook, and `uninstall.dirty-worktree`'s own route offers 'commit'

- where: design/measurement.md (The in-repo invocation log, item 7: 'Two of the teardown's four refusals route through another jigc verb'); crates/cli/guides/QUICKSTART.md (the uninstall-log paragraph added by 4fa1c0f5); `jigc uninstall --help` ('A log you moved out stays out')
- driven: driven (HEAD debug binary)
- class: instance, unbounded — I found the hook path by hitting it; I did not enumerate every indirect jigc invocation (the SessionStart hook is another candidate, not driven).

```text
DRIVEN on the HEAD debug binary.
Setup: an adopted rig with `jigc config set invocation-log true`; `.jigc/logs/` absent.
Argv: `jigc uninstall` (refuses, exit 1) → `.jigc/logs/` still absent, as the fix claims. Then `git add scratch.txt && git commit -m …` → `.jigc/logs/invocations.jsonl` exists with one record, argv `["validate", "--format", "json"]` — the installed pre-commit hook is a jigc invocation.
In the combined run (below, under held) this is how the log came back: the `uninstall.dirty-worktree` route says 'get the work out of those paths first (commit, stash, or copy it)', a commit was made inside a fan-out worktree, and the fourth un-forced `jigc uninstall` then refused over the log. It converges in one extra move and loses nothing; the sentence 'every other jigc verb still starts the log' is literally true. The undercount is the bound's enumeration: three of the four refusals can re-mint the log, and so can any commit the operator makes between attempts.
```

## XC-6 · LOW · Gate runtime: the pass's suites are the bulk of the test step's growth, led by worktree_registration_anchor; the step is at 537 s against the 600 s foreground ceiling

- where: crates/cli/tests/worktree_registration_anchor.rs; copy_in_baseline.rs; setup_install_pathspec_guard.rs; milestone_promote_guards.rs; record_door_baseline.rs; linked_worktree_doc_home.rs; create_only_gate.rs; .config/nextest.toml (slow-timeout 60 s, terminate-after 8)
- driven: driven (cargo nextest, private target dir)
- class: The 14 suites the pass added or grew, enumerated from `git diff --stat origin/main..HEAD -- crates/cli/tests` (new files) plus the three existing suites with the largest additions. Pre-existing suites the pass touched by a few lines were not timed.

```text
DRIVEN. A full gate log on this tree (another run's, read from its log) shows `Summary [536.634s] 4570 tests run: 4570 passed (4 slow)`, the four slow tests all pre-existing. I ran the pass's eleven new suites plus the three it grew, alone, under nextest in a private target: 194 tests, all pass, 271.7 s wall. Summed per-test time by suite: worktree_registration_anchor 14 tests / 849 s (max 114 s); copy_in_baseline 16 / 413 s (max 98 s); setup_install_pathspec_guard 36 / 409 s (max 113 s; grew from 22 cells); milestone_promote_guards 11 / 351 s; record_door_baseline 7 / 288 s (max 107 s); linked_worktree_doc_home 14 / 275 s; create_only_gate 26 / 242 s; promote_destination_shape 9 / 178 s; store_door_home_shape 6 / 168 s; uninstall_workbench_subject 23 / 166 s; worktree_registration_reach 8 / 132 s; replacing_writers_never_follow 10 / 89 s; setup_failed_first_run 13 / 46 s; reconciliation_baseline_contrast 1 / 10 s. Caveat: my run shared the machine with other auditors and a cold target, so absolute times are inflated (17 tests crossed the 60 s slow mark in my run; none of them did in the full-gate log). The ranking is the datum. Seven single tests exceeded 90 s under that load, against a terminate threshold of 480 s.
```

## XC-7 · LOW · `jigc uninstall` still prints 'pruned git's worktree registrations' after the pass removed every prune, and the test pins the old word under a message that names the new one

- where: crates/cli/src/render.rs:4660; crates/cli/tests/cwd_verb_subject.rs:1185-1187
- driven: driven (HEAD debug binary) for the printed line; read only for the test
- class: instance, unbounded — one printed line; I did not scan every printed surface for the word.

```text
DRIVEN and READ. A landed `jigc uninstall` over a repository that had fan-out worktrees prints `- pruned git's worktree registrations for the fan-out worktrees .jigc/ held`. DECISIONS.md's L-22 paragraph corrects the earlier 'It prunes now … and says so' entry and states no repository-wide prune remains (confirmed: `grep '"prune"'` over both crates' src is empty). The line is scoped to jigc's own worktrees, so it misstates the mechanism, not the reach. The pass edited the assertion's message to 'the door must say it dropped them' while the assertion still matches `"pruned git's worktree registrations"`.
```

## held

- Uninstall fixes x registration refusals x log rule, as one chain: an active milestone with a live worktree holding staged code, a stale registration holding a sub-agent's commit, a sub-task staged doc, an uncommitted config delta and the invocation log on. Four un-forced `jigc uninstall` runs, each route followed as printed (the keep-branch command, `git stash` in the worktree, `jigc task discard <sub> --force`, `git add` of the delta, moving the log out). Converged at exit 0 with the commit on `kept/area-high`, the staged code in the stash, the log outside `.jigc/`, and no foreign registration touched.
- Linked-worktree guard's sub-task exemption: `jigc milestone create`, `add-task`, `provision` and `finalize` typed from a user-made linked worktree all acted on and committed in the main checkout; a hand edit to VISION.md in the linked worktree survived the milestone finalize untouched. The exemption does not reopen the loss the guard closes.
- `jigc rename` from a user-made linked worktree with an uncommitted hand edit there: renamed and committed in the main checkout, linked worktree's bytes untouched.
- Copy-in baseline x fan-out x milestone boundary: with the file-state record removed, a sub-task's first write recorded the baseline, a later hand edit in the main checkout blocked `jigc milestone finalize` on `reconciliation.conflict-block` (exit 3), bytes intact.
- Unlanded-work refusal x record door's HEAD witness x clobber guard in a clone-like state (record removed mid-milestone): the refusal mutated nothing (clean `git status`, HEAD unmoved); the printed `reset --soft` route was run as printed and the boundary then landed both sub-tasks' code and docs, with the record flip, at exit 0.
- Setup guards x replacing writers on the upgrade path, published rc.24 install then HEAD binary `jigc setup`: tracked install (one new install commit, second run a no-op), `.claude/` ignored (exit 0), `.jigc/` and `.claude/` both ignored whole (exit 0 twice, footprint record written), then a user edit to the ignored `.jigc/AGENT.md` (refused, `setup.dirty-install-path`, bytes intact). The guide edits change the installed skill's digest and a re-run handled it cleanly in all three.
- Uninstall then setup round trip on HEAD: setup refuses over uninstall's own uncommitted edits with a route that works (commit), then installs and is idempotent.
- Setup on an unborn HEAD over an untracked symlink at `.jigc/AGENT.md`: refused under `setup.write-bootstrap`, link and target untouched, route runnable without a commit.
- Scope honesty by grep over the range's src diff: no new environment read (the three `std::env` hits are `temp_dir` in tests), no new clap argument, no new file under `.jigc/` (the footprint record gained lines in the same file; `.jigc/displaced/` predates the pass). All 18 newly `pub` items I sampled have a production caller.
- Flipped assertions in pre-existing tests (flow49_acceptance, reconciliation_baseline_contrast, milestone, milestone_record_stale_base, flow53_acceptance, setup, cwd_verb_subject, five stale-stamp fixtures): each flip follows a behaviour the pass changed on purpose; none weakens an assertion to admit a defect. The stale-stamp fixtures moved to the `jigc-version:` shape, which the stamp has had since 2026-07-03, so no older install is left unrefreshed.
- DECISIONS as-built claims checked against the tree and found true: no `git worktree prune` in production code; one `git worktree remove` site (`remove_owned_registration`); seven call sites of `reconcile_record_preflight`; the five record-rewriting doors the guide lists match those call sites.

## not_examined

- The requested report file under the scratchpad's fix-rc25/audit directory was NOT written: this session's standing instructions forbid writing report files and direct findings to the structured return, so everything is here.
- The interior of each fix (planner, sink, classifier, guard code paths) — read only where two fixes meet; each single-fix area has its own reviewer.
- The new suites' assertions line by line. I read headers and structure of record_door_baseline and linked_worktree_doc_home and the flipped hunks of existing tests; I did not audit the other nine new suites for instance-pinning.
- Linux behaviour: every drive ran on macOS; `rewrite_home`'s dev/ino check and the worktree admin reads were not exercised in a container.
- Release-profile binary: all drives used the debug build, whose route fences panic where release prints.
- `jigc migrate` and an ordinary task's doc write from a fan-out worktree (same route shape as XC-4, read not driven); `jigc task amend` from a linked worktree.
- Other triggers of setup's `Unknown` arm beyond git missing from PATH (XC-2).
- Concurrency: save-lock contention on the copy-in under a real N-agent fan-out, and every external-writer race the pass deferred.
- `dev/gate` itself was not run by me; the full-gate figure is read from another run's log on the same tree, and the hygiene scan (gitleaks, denylist) was not re-run.
- design/ sentences touched by the pass beyond those cited in findings and held — reconciliation.md, measurement.md item 7 and finalize.md's store-doors paragraph were read; storage.md, validation.md, team-ready-state.md, write-commands.md and the `--help` texts were not checked sentence by sentence.
- Published-crate API impact of the `jigc-engine` signature changes (`create_gated`, `state::create`, `setup_dirty_install_finding`) on the release pipeline's version decision.

# AREA: linked-worktree-guard (a88f71cc · e3a6ba58), reviewed on the tree at 5f5b273a

Verdict: red — the guard holds beside a normal main checkout, but its predicate also fires wherever `.git` is a file and the resolved home is not a checkout (bare-repo worktrees, submodules, `--separate-git-dir`), where it refuses doc creation that landed on rc.24 and prints a route that cannot work.

## F1 · HIGH · The guard fires where jigc_home is not a checkout: doc creation that landed on rc.24 is refused, and every printed route dead-ends

- where: crates/cli/src/render.rs:2138 (`CommitSite::differing`), consumed by `CodeOnlyCheckout::of_task/of_mint/of_reader` at render.rs:2221/2233/2238; home resolution at crates/cli/src/repo.rs:91. False record: implementation/decisions-pending.md:91; pinning test crates/cli/tests/linked_worktree_doc_home.rs:1596-1646 (commit e3a6ba58)
- driven: driven (NEW release build of this tree, and the published 1.0.0-rc.24 as the before)
- class: One predicate, three constructors. Members: every repository whose `.git` is a file and whose `dirname(git-common-dir)` is not a checkout of it. Four layouts driven (pointer-to-bare, sibling bare, submodule, --separate-git-dir); not enumerated beyond those — unbounded.

```text
Same script on both binaries (NEW = release build of this tree, OLD = published 1.0.0-rc.24), in a `dev/jigc-rig bare` root: `jigc setup`, `jigc start --workflow single-task "first decision"`, `jigc doc create adr --title "First decision" --task first-decision`, three slots, commit doc, `jigc task finalize first-decision`.

Layout B (`proj/.bare` + `proj/.git` = `gitdir: ./.bare` + `proj/main` worktree): OLD setup 0, create 0, finalize 0, HEAD carries `docs/decisions/first-decision.md`. NEW setup 0, mint 0 with a `checkout:` block, create exit 1 `finalize.linked-worktree-doc`.
Layout C (sibling `bare.git` + worktree): both binaries setup exit 1 `setup.install-hook` but leave the project layer behind, so mint exits 0. OLD create 0, finalize 0, doc in HEAD. NEW create exit 1, same code.
Layout A (git submodule `vendor/sub`): OLD setup 0, create 0, finalize 0, doc in HEAD. NEW create exit 1; the message calls the submodule "the linked worktree" and `<super>/.git/modules/vendor` "the main checkout".
Layout D (`git init --separate-git-dir`): OLD create 0, finalize 0. NEW create exit 1, same code.

Route run as printed on NEW:
- B: `cd <proj>` 0 (`git rev-parse --is-inside-work-tree` prints false); `jigc start "record it"` 0 (router, mints nothing); `jigc start --workflow record-decision "record it"` 0; `jigc doc create adr --title "Routed decision" --task record-it` 0; `jigc task finalize record-it` exit 1: "`git status --porcelain` failed: fatal: this operation must be run in a work tree".
- C: `cd <parent of bare.git>` 0; `jigc start ...` exit 1 "not inside a git repository".
- A: `cd <super>/.git/modules/vendor` 0; `jigc start ...` exit 1 "this project isn't set up".
- B, doc-only mint from the only checkout: `jigc start --workflow report-inconsistency "a finding"` exit 1, same code, same dead `cd`.

So in these layouts no door can commit a managed doc any more and no printed exit works. The M57 deferral row says the layout "answers isn't set up at every door", "the guard never speaks there" and is "pre-existing ... unusable"; that holds only until the `jigc setup` that "isn't set up" routes at has run, and the pinning test never runs it. Caveat: rc.24 was already partial there (`jigc doc list` answers "no committed docs"; an edit of the committed doc answers "no staged instance"), but create plus finalize landed.
```

## F2 · MEDIUM · a88f71cc leaves the second reader of a staged doc's committed bytes open: the blast-radius walk still blocks on an anchor the task's staged copy removed

- where: crates/engine/src/validate.rs:2126-2135 (blast set deduped by anchor address, not by doc); design/validation.md:322 ("Effective state is the working overlay over the base, never both"); crates/cli/tests/doc_code_gate.rs `an_in_task_citation_repair_clears_the_floor_under_a_role_binding_workflow` (commit a88f71cc)
- driven: driven (NEW release build; rc.24 as the before)
- class: Readers of committed doc bytes inside `schedule_doc_code`: 2 — the bound surface (fixed) and the blast walk (open). Derived by grepping `enumerate_committed_surface(` (3 call sites, one at task scope) and reading `enumerate_target_surface`. Test axis: 9 shipped workflow files grant a code-anchored doctype with `as:` (grep over the pack workflow directories), 1 driven; the repair-shape axis (update / unset / remove-item) is not iterated.

```text
Setup: `dev/jigc-rig fresh`, main checkout, committed `adr:single-node-cache` citing `src/lib.rs#cache_get`.
argv: `jigc start --workflow single-task "rename the getter"`; rename the function to `cache_fetch`, `git add src/lib.rs`; `jigc doc set-field adr:single-node-cache#status/cites-code --unset --task rename-the-getter`; `jigc task finalize rename-the-getter`.
Observed (NEW): the unset exits 0 ("copied in for update"), the staged copy carries no `cites-code` line, and finalize exits 3 with `doc-code.symbol-exists` — "anchor `src/lib.rs#cache_get` resolves to no symbol", at `adr:single-node-cache#status/cites-code` — i.e. on the committed bytes of a doc this task has staged. Same result under `quick-fix` (no role binding) and on rc.24. Deleting the function and unsetting the citation: exit 3 as well; the only way through is two commits (doc-only finalize exit 0, then the code). The printed route ("update the citation ... or restore the cited symbol") names no repair for a removed citation.
The doc sentence added by this commit is therefore false for this path. The binary test drives one cell (single-task x adr x value update).
```

## F3 · MEDIUM · The relocation route's mint collides with the task it relocates when `<intent>` is the task's own

- where: crates/cli/src/render.rs:2284-2311 (`relocate_change_steps`), printed at crates/cli/src/task.rs:421 and task.rs:4915; masked at crates/cli/tests/linked_worktree_doc_home.rs:1141 (commit e3a6ba58)
- driven: driven (NEW release build)
- class: 2 print sites of one producer (grep `relocate_change_steps(`: the write door's dangling-anchor arm and the changelog gate promoted to blocking). The collision needs only that the intent slugs to the old task's id.

```text
Setup: `committed-singletons` rig, committed ADR citing `src/lib.rs#cache_get`, `git worktree add -b feature <wt>`; in `<wt>`: `jigc start --workflow single-task "rename the getter"`, rename staged, the repair `jigc doc set-field ... --task rename-the-getter` refused (exit 1) with the long route.
Route as printed: `git -C <wt> stash` 0; `cd <main>` 0; `git -C <main> merge feature` 0; `jigc start --workflow single-task "rename the getter"` exit 1: "blocking · task.serial-collision — task `rename-the-getter` is already active". The route discards the old task only as its last step, so the most natural fill of `<intent>` refuses, and continuing with the printed `stash pop --index` restores the change before any new task exists.
The acceptance test substitutes `<intent>` with "rename the getter from main", which is why it is green (suite run at HEAD: 14/14).
Also driven: after stash, merge and pop, the SAME task typed from `<main>` takes the repair (exit 0) and `jigc task finalize rename-the-getter` exits 0 with one commit carrying the ADR and `src/lib.rs` — an exit the route does not name.
```

## F4 · MEDIUM · The backstop's "finalize it from the main checkout" route commits the main checkout's pre-task staged work at exit 0

- where: crates/cli/src/task.rs:2957-2973 (`lands_from_home` asks `decide_base_repin` only); route text in `linked_worktree_doc_finding`, task.rs:399 onward (commit e3a6ba58)
- driven: driven (NEW release build; rc.24 for the mechanism)
- class: instance, unbounded — any route that moves a worktree-minted task's finalize to another checkout; the relocation route avoids it by minting there first.

```text
Setup: `committed-singletons` rig; in `<main>` the user's own `wip.txt` is `git add`-ed before any task; `git worktree add -b feature <wt>`.
argv: in `<wt>` `jigc start --workflow single-task "sharpen"`; in `<main>` `jigc doc set-slot vision:vision#thesis --from-file - --task sharpen` (exit 0) and the commit doc; in `<wt>` `jigc task finalize sharpen` exits 3 with the guard and the route "finalize this task from the main checkout: `cd <main>`, then `jigc task finalize sharpen` commits its docs on that checkout's branch".
Route as printed: exit 0 — "promoted VISION.md / added wip.txt / 2 files committed"; `git show --name-only HEAD` lists VISION.md and wip.txt.
Control (identical, task minted in `<main>`): exit 3 `finalize.carried-staged — wip.txt was already staged before this task existed`.
The task's pre-task snapshot was taken from the worktree's index, so the carryover gate is blind in the checkout the route sends it to. No bytes are lost and the mechanism exists on rc.24 (driven, same result); what is new is a printed route into it that says only the docs are committed.
```

## F5 · LOW · Two route sentences are false in a reachable state

- where: crates/cli/src/task.rs:409 and :442, crates/cli/src/render.rs:2321-2333 ("`jigc start \"<intent>\"` starts a task there"); crates/cli/src/task.rs:411 ("Task `<id>` stays usable here for code")
- driven: driven (NEW release build)
- class: (a) 3 print sites, by grep for the span in task.rs and render.rs. (b) 1 print site, false in 2 driven states (doc already staged; detached worktree).

```text
(a) From `<main>`, `jigc start "record it"` exits 0, lists the selectable workflows and mints nothing — the CLI's own top-level help says the bare front door mints nothing. The write-door test runs that span and then mints separately with `--workflow single-task` (linked_worktree_doc_home.rs:1026-1032).
(b) Task `mixed-change` minted in `<main>` with `vision:vision#thesis` staged from there; a second write from `<wt>` is refused with "Task `mixed-change` stays usable here for code — `jigc task finalize mixed-change` commits what you `git add` on this branch"; that command from `<wt>` then refuses with the backstop (`finalize.linked-worktree-doc` at VISION.md). The same sentence prints from a detached linked worktree, where the command answers `repo.head-detached` (exit 1).
```

## held

- a88f71cc does not let a dangling committed anchor land: with a doc staged and bound, an out-of-band edit that dangles the committed citation blocks at exit 3 on both binaries (uncommitted edit: `reconciliation.conflict-block`; committed edit: `finalize.base-mismatch`).
- jigc's own fan-out is untouched: a sub-task in `.jigc/worktrees/area-one` ran `set-slot`, `create adr`, slot fills and a `set-field` citation repair at exit 0, saw no `checkout:` block on `workflow sub-task --task` or `start --task` and no served-from note, and `milestone finalize` landed three promoted docs plus the renamed code in one commit.
- Milestone doors typed in a user-made linked worktree commit in the main checkout and leave that worktree's index alone, so the sub-task exemption opens no promote into the worktree.
- A code-only task lands from a linked worktree: validate, --dry-run and finalize exit 0, and the commit on `feature` carries the one code file.
- Write leaves from the worktree: nine argvs (create x2, add-item, remove-item, retitle-item, rename, set-field, set-field --unset, set-slot) each exit 1 with the guard and leave only the commit doc in the task area; a tenth, `doc author`, exited 1 on my malformed payload before reaching the guard, so it is covered by the suite only.
- Backstop: one guard row at `task validate`, `--dry-run` and `task finalize` (exit 3 each) and at bare `jigc start` (exit 0); where the main checkout would refuse the pin, that checkout answers `finalize.base-mismatch` for the same task, confirming the arm's premise.
- No text leaks into `--format json`: mint and resume stdout keys are exactly {task, text} with no `checkout:` text; orientation keys are unchanged; read notes go to stderr only; the mint and migrate refusals are the findings envelope on stderr with stdout empty.
- `jigc migrate` from the worktree exits 1 with nothing minted.
- The new code cannot be demoted: `jigc config set validation.finalize.linked-worktree-doc.severity advisory` answers `config.undeclared-key`.
- A worktree path with a space and an apostrophe is correctly quoted in the route's `git -C` operand.
- A doc-only mint by an ordinary task inside a fan-out worktree is refused at the mint with a route that works from the main checkout.
- Registration homes match the sibling code `finalize.amend-staged-doc` (pack.rs, render.rs, task.rs, command-output-contract, finalize, surface-contract, validation), by `git grep -l` of both codes.
- Neither commit's diff stat touches a pack schema, compose golden, snapshot or AGENT.md file.
- At HEAD, `linked_worktree_doc_home::` passes 14/14 and the engine arm 1/1 (private target dir).

## not_examined

- The full `dev/gate`; only the area's two suites were run.
- Any git older than this machine's, and Linux.
- `jigc ingest`, `unmanage`, `relocate`, `migrate-corpus` and `config set docs-root` typed in a linked worktree (outside this area's diff; `jigc rename` was driven once and commits in the main checkout).
- Stash-route conflict cases (dirty main checkout, diverged branches) and the changelog-gate blocking arm end to end.
- An unparseable staged copy of a bound doc (read only: it enumerates no anchors and relies on the conformance gate).
- The served-from note on `doc show --task`, and the `doc author` leaf with a valid payload (my payload was malformed).
- The route in layout D (`--separate-git-dir`) was not run; only the refusal was driven there.
- The full report is at fix-rc25/audit/review-linked-worktree-guard.md under the session scratchpad, as the task asked; it carries no host path.

# AREA: worktree-registrations (commits 8c159622 · ebfc79fb · e842342e on fix/rc24-tier1)

Verdict: red — the three fixes hold in every dominant cell I drove (no foreign registration touched, healthy fan-out lands, every printed route I ran landed the work), but the same doors still carry one driven exit-0 loss that predates the pass (F1), one regression on git older than 2.31 (F2, emulated), three class members the fixes claim closed (F3, F4, F6), one refusal whose route names a command it does not print (F5), and a decision record that contradicts the design doc and source on who ruled the boundary refusal (F7).

## F1 · HIGH · Un-forced `milestone discard` and `uninstall` destroy a sub-agent's untracked work at exit 0 when `status.showUntrackedFiles=no` is set (pre-existing, not introduced by this pass)

- where: crates/cli/src/milestone.rs:5890-5914 (`dirty_worktrees`, argv `status --porcelain` at :5894), consumed by `probe_leftover` at :4900. Not in the diff of 8c159622 / ebfc79fb / e842342e; identical on the published rc.24.
- driven: driven (release build of this tree; published rc.24 as before-control)
- class: Derived by enumerating production `git status` argv: grep for `"status"` above each file's first `#[cfg(test)]` in crates/cli/src gives 8 probes, of which 3 pass no `--untracked-files` (milestone.rs:5894, setup.rs:4594, task.rs:3530). Only milestone.rs:5894 was examined. It is the bytes leg of `probe_leftover`, which has 3 door consumers (provision phase 1, discard, uninstall); driven at 2. Provision reaches it only for a live worktree this repository has not registered (not driven). The other two probes are unexamined, so the class beyond this probe is unbounded.

```text
SETUP: `dev/jigc-rig fresh --binary <release build of this tree>`; `jigc milestone create "Cache rework"`; `jigc milestone add-task cache-rework "Area low"`; same for "Area zed"; `jigc milestone provision cache-rework`; `git -C <repo> config status.showUntrackedFiles no`; write `untracked-work.rs` into `.jigc/worktrees/area-low` and never stage it. `git -C <worktree> status --porcelain` then prints nothing.
ARGV: `jigc milestone discard cache-rework` (no `--force`).
OBSERVED: exit 0. stderr: `warning: removing the fan-out worktree .jigc/worktrees/area-low discards work that is not in git:` / `untracked-work.rs (never staged)` / `note: the fan-out worktree is the only copy of these bytes — they are not recoverable.` stdout: `discarded milestone:cache-rework (2 sub-task(s); workbench removed)`. `.jigc/worktrees` is gone.
CONTROL 1 (same state, config unset): exit 1, `blocking · milestone.dirty-worktree`.
SECOND DOOR: `jigc uninstall` (no `--force`) on the same state with the config: exit 0, same `not recoverable` warning.
CONTROL 2 (published rc.24 binary, same state with the config): `milestone discard` exit 0, same warning — so this is not a regression.
MECHANISM: the refusal probe runs `git status --porcelain` with no `--untracked-files`, so it inherits the repository's config; the narration probe (`worktree_work`, :8526) passes `--untracked-files=all`, which is why the loss is named after the removal and never refused before it.
```

## F2 · MEDIUM · On git older than 2.31 `milestone provision` no longer re-creates a sub-task worktree whose directory is gone, and the advisory's route does nothing — a regression against rc.24 (emulated, not a real old git)

- where: crates/cli/src/milestone.rs:3222-3226 (reuse and stale read from `prunable`), :3446-3448 (the bound, stated only in rustdoc), :5345-5349 (the advisory route); commit 8c159622.
- driven: driven under emulation (PATH wrapper over git 2.54.0) on the release build and on published rc.24; no real git < 2.31 was run
- class: Instance at the one door that reads `prunable` to decide reuse. Grep for `.prunable` consumers gives provision's reuse/stale decision and `leaked_worktree_remedy` (text only). Same mechanism as the deferred M57 row for a locked registration whose directory is gone (decisions-pending.md:88), which is pre-existing on every git; this member is new with 8c159622. Other git-version assumptions were read, not driven: `git restore` in the recipe needs 2.23; `rev-parse --absolute-git-dir` and `rev-list --single-worktree` are older than that.

```text
EMULATION, stated plainly: no real old git was run. A `git` wrapper first on PATH passes every call through to git 2.54.0 and removes the `prunable` and `locked` lines from `worktree list --porcelain` only — the one difference the rustdoc at :3446 names for git < 2.31.
SETUP: rig `fresh`; milestone with two provisioned sub-tasks; delete the directory `.jigc/worktrees/area-low` (registration stays, HEAD at the base pin, nothing staged).
ARGV (wrapper on PATH): `jigc milestone provision cache-rework`.
OBSERVED: exit 0, `provisioned 2 worktree(s) for milestone:cache-rework … (area-low, area-zed)`; `ls .jigc/worktrees` shows `area-zed` only. `jigc milestone provision cache-rework --force`: same output, same directory listing. `jigc milestone execute cache-rework` then prints `advisory · milestone.worktrees-partial … sub-task area-low has none (nothing is there)` with `route: jigc milestone provision cache-rework — idempotent: it reuses every worktree that landed and adds only the missing ones` — the command that just did nothing — and still prints the `Spawn: cd …/area-low && …` line.
CONTROL (real git 2.54.0, same state): exit 0 and `area-low` is re-added.
BEFORE-CONTROL (published rc.24, same state, same wrapper): exit 0 and `area-low` is re-added, because rc.24 pruned and then listed.
No shipped doc states a minimum git version: grep for `2.31` and for a git-version requirement over README, the two guides and design/ returned nothing; the bound exists only in the rustdoc.
```

## F3 · MEDIUM · `jigc uninstall` still drops a registration jigc did not create when its recorded path is under `.jigc/worktrees/`, at exit 0, and attributes it to the fan-out — ownership is decided by location, not by creation

- where: crates/cli/src/setup.rs:4052-4066 (`drop_workbench_registrations` iterates every registration under the root); crates/cli/src/milestone.rs:3497-3508 (`is_owned_worktree_path` accepts any direct child); commit 8c159622.
- driven: driven (release build of this tree)
- class: Derived from the call sites of `remove_owned_registration` (grep: 5 — milestone.rs:3301, :7430; setup.rs:4061; task.rs:9109, :9117). Four are aimed at a path jigc builds from a sub-task id or its own `.combine-<pid>-<nanos>` name; one (setup.rs:4061) iterates every registration under the root regardless of name. So 1 of 5 sites, at 1 of 4 doors. The reach suite's foreign shapes (detached, branch, locked, each recorded outside `.jigc/`) do not include a foreign record under the root.

```text
SETUP: rig `fresh` (no milestone). `git worktree add -b mine .jigc/worktrees/mine HEAD`; append a line to `README.md` inside it (unstaged); `mv .jigc/worktrees/mine <rig>/mine-moved`. `git worktree list --porcelain` shows the record with `branch refs/heads/mine` and `prunable gitdir file points to non-existent location`.
ARGV: `jigc uninstall` (no `--force`).
OBSERVED: exit 0. stdout includes `- pruned git's worktree registrations for the fan-out worktrees .jigc/ held`. `.git/worktrees` no longer exists. `git -C <rig>/mine-moved status` → `fatal: not a git repository`. The edited file is still on disk; the checkout's HEAD, index and reflog linkage are gone, which is the harm L-22 names for a worktree moved with `mv`.
The contract text (design/team-ready-state.md, the door rule; the commit message) is: a door removes a registration only at a path it created — a sub-task's `<sub-task-id>` or a boundary's `.combine-*`. The code tests only that the path is a direct child of `.jigc/worktrees/`. The commit also claims the ack line is printed only when uninstall dropped one of its own; here it dropped none of its own.
Not a regression: rc.24's repository-wide prune took the same record (by reading; not re-driven on rc.24 for this cell). The registration leg does refuse here when the record holds staged paths or an unreached commit; with HEAD on a branch and only unstaged work it clears.
```

## F4 · MEDIUM · In a moved repository the boundary still lands, at exit 0, without staged code that one of the milestone's own sub-task registrations holds — the guard matches registrations by path and the record names the old path

- where: crates/cli/src/milestone.rs:7562-7618 (`unlanded_work`), :3780-3786 and :3621-3633 (`admin_records` lookup through `same_worktree_path`, which cannot match a recorded path whose parent no longer exists); commit e842342e.
- driven: driven (release build of this tree)
- class: Instance, bounded by mechanism: every stale-side read goes through one path-keyed lookup (`admin_records` + `same_worktree_path`), with 3 consumers by grep — `anchored_reading` (all four worktree doors), `fanout_worktree_paths` (uninstall), and `relink_command` via the anchored value. Any registration whose recorded path differs from `<jigc_home>/.jigc/worktrees/<id>` is invisible to all of them. At the three consenting doors that fails safe (the directory itself refuses as unverifiable content). At the boundary it does not. The suite's moved-repository cell drives provision only; its copied-repository cell is `cp -R`, not `mv`.

```text
SETUP: rig `fresh`; milestone with two provisioned sub-tasks; `git add` one file in each worktree (`low.txt`, `zed.txt`). `mv <rig>/repo <rig>/repo2`. In the moved repo run `git worktree repair <rig>/repo2/.jigc/worktrees/area-zed` (one worktree mended, so the boundary has something to land). `git worktree list --porcelain` shows `area-low` at the OLD path, `prunable`; the directory `.jigc/worktrees/area-low` is on disk with its `.git` link pointing at the old location.
ARGV: `jigc milestone finalize cache-rework` in the moved repo.
OBSERVED: exit 0. `finalized … (2 sub-tasks)`, `added zed.txt`, `sub-tasks: area-low: unreadable worktree at .jigc/worktrees/area-low, no code counted · area-zed: 1 code file`. The record reads `status: joined`. Afterwards `git --git-dir .git/worktrees/area-low diff --cached --name-only` still prints `low.txt`, and both the directory and the admin record are still there.
Nothing is destroyed — the teardown matches by path too, so it reaches neither. But the milestone is settled without work a registration of its own sub-task holds, which is the first half of the harm the commit names, and no `milestone.unlanded-work` is raised. Contrast: with the link merely unreadable at the SAME path the guard does refuse (F5).
The enumerated cells in design/finalize.md say `directory or .git link is gone`, which this cell is not by the letter; the rule sentence above them (`while git's registration of one of the milestone's own sub-task worktrees holds work the boundary would land without`) covers it. The landed `unreadable worktree` line predates the pass (by reading; not re-driven on rc.24). The partial repair is my construction; a sub-task that contributes a doc would land the boundary the same way (not driven).
```

## F5 · MEDIUM · The new boundary refusal says `run the command on that line` over a line that carries no command, when the checkout's `.git` entry is present but unreadable

- where: crates/cli/src/milestone.rs:4654-4662 (`relink_command` prints only when `.git` is NotFound), :7684-7720 (this door's own exits), :7735-7739 (the route sentence); commit e842342e.
- driven: driven (release build of this tree)
- class: Derived by enumerating the command-bearing arms of the refusal line (`hold_line` :4497-4531, `held_here` :4601-4628, exits :7684-7720): keep (a commit), recipe (nothing at the path), re-link (directory with no `.git`), `reset --soft` (commit, live, landed), stash (staged, live), prose for a file in the way. Cells holding staged paths with none of those: (1) a directory whose `.git` entry exists and is unreadable — driven; (2) a path whose stat fails for a reason other than absence, mapped to the Directory shape at :7598 — read only. The Unreadable-registration cell is handled separately and says there is no command. So 2 cells, 1 driven.

```text
SETUP: rig `fresh`; two provisioned sub-tasks, each with one `git add`-ed file. Overwrite `.jigc/worktrees/area-low/.git` with the bytes `garbage\n`.
ARGV: `jigc milestone finalize cache-rework`.
OBSERVED: exit 3. `blocking · milestone.unlanded-work — … .jigc/worktrees/area-low: git's registration of this path still holds 1 staged path in no commit (low.txt) — nothing else holds that work, and the boundary lands only the paths staged in a live worktree; the teardown behind a landed boundary would then drop the registration, and that work with it` / `route: run the command on that line that fits what you want kept or landed (…), then re-run jigc milestone finalize cache-rework — or abandon the milestone with jigc milestone discard cache-rework, which answers for the same path with its own route`.
The line contains no backticked command: no keep (no commit), no recipe (a directory stands there), no re-link (a `.git` entry exists), no stash (not live). The only printed exit is `milestone discard`, whose consent destroys the work. The door has no `--force`. Withholding the re-link there is deliberate and documented (it would overwrite another repository's linkage); what is missing is any route for the cell, or a route sentence that does not promise one.
CONTROL: delete the `.git` entry instead → the line prints `printf 'gitdir: %s\n' <admin> > <path>/.git`; run as printed from `/` → exit 0; `finalize` → exit 0, both files landed.
```

## F6 · MEDIUM · The keep command still exits 128 as printed when a branch differing only in case exists on a case-insensitive filesystem — the dead-end e842342e says it closed

- where: crates/cli/src/milestone.rs:3947-3993 (`kept_branch`, `branch_is_free`: byte-exact comparison against `for-each-ref`); commit e842342e.
- driven: driven (release build of this tree, macOS case-insensitive volume)
- class: One function with one predicate, so the class is every place the keep command is printed: `keep_commit_command` has 2 callers by grep (`anchored_words`, shared by the four door refusals, and `PendingAnchor::narrate_taken` after a forced drop). Driven at the boundary only. The suite's keep-name test iterates blocking branches of exact case.

```text
Machine: macOS default (case-insensitive) volume, git 2.54.0, loose refs.
SETUP A: rig `fresh`; two provisioned sub-tasks; `git -C <repo> branch Kept`; in `.jigc/worktrees/area-low` add and commit one file.
ARGV: `jigc milestone finalize cache-rework` → exit 3, prints `git -C <worktree> branch kept/area-low <sha>`. Run as printed from `/`.
OBSERVED: `fatal: cannot lock ref 'refs/heads/kept/area-low': 'refs/heads/kept' exists; cannot create 'refs/heads/kept/area-low'`, exit 128.
SETUP B: delete `Kept`, create `KEPT/area-low`; same argv; the printed command is again `… branch kept/area-low <sha>`.
OBSERVED: `fatal: a branch named 'kept/area-low' already exists`, exit 128.
These are the two failures the commit message names as fixed (`kept/<id>-2`, or `kept-<id>`), reached through a case variant. At the boundary `reset --soft` is printed beside it, so the reader has another exit. At provision, discard and uninstall the keep command is the only non-destructive route for a held commit, and the alternative is `--force`. Nothing is lost when it fails.
```

## F7 · MEDIUM · The decision log contradicts the design doc and source on who ruled the boundary refusal, still states the superseded behaviour, and has no entry for e842342e or its new finding code

- where: DECISIONS.md:12 and :48; design/team-ready-state.md:113 (`Struck the same day, on the human's second ruling over this class`); crates/cli/src/milestone.rs:7510, :4334, :3682; the commit message of e842342e (`ruled on by the human on 2026-10-04`).
- driven: read only (the envelope and exit code were driven on the release build)
- class: Instance, unbounded: I compared the three commits of this area against the 2026-10-04 entries only. Whether the other round-3 commits are likewise unrecorded was not checked.

```text
READ, at HEAD:
- DECISIONS.md:48, the human's fork ruling on own registrations, ends: `the landed jigc milestone finalize names what it drops`. At HEAD the boundary refuses before landing instead.
- DECISIONS.md:6-12 queues the pre-landing refusal as item 4 of five members taken `without a ruling of the human's on each`, `the orchestrator's call, open to overturn`, says it `departs from the fork's wording`, and says `None is built at this entry`. It is built (e842342e).
- design/team-ready-state.md and three rustdoc sites attribute the same refusal to `the human's second ruling` / `the human's ruling of 2026-10-04 on this class`. One of the two records is false.
- `grep -c "e842342e\|unlanded-work"` over DECISIONS.md, implementation/decisions-pending.md and implementation/project-history.md returns 0 in each. `git log origin/main..HEAD -- DECISIONS.md` shows three record commits, the last before the round-3 fixes. The other new code of this pass, `finalize.linked-worktree-doc`, has a ruling that admits it by name; `milestone.unlanded-work` has none, and its fixer's own report lists it first under things to rule on.
- Registration homes of the new code, by grep: the const and `FINALIZE_DOOR.codes` in milestone.rs, a rustdoc mention in render.rs, three test files, design/finalize.md, design/team-ready-state.md. It is absent from crates/cli/guides/MIGRATING.md and design/worked-examples.md, where the sibling door codes `milestone.dirty-worktree` and `milestone.leftover-holds-work` are named.
The code's stated reason (the door had no blocking code; both sibling codes route to `--force`, which this door lacks; `milestone.zero-contribution` is false whenever a sibling contributes) is coherent, and I found no existing code that fits. Exit 3 and the `--format json` envelope match the door's other pre-landing blocks (driven: `schema_version: 3`, keys `severity, probe, check, code, key{code,target}, message, location, route`).
```

## held

- L-22 reach: with three foreign worktrees registered outside `.jigc/` (one detached with a commit and a staged path then moved away, one on a branch locked then moved away, one live), the admin directory of each was byte-identical (hash of every file) after `milestone provision`, a refusing `milestone discard` (exit 1), a refusing `uninstall` (exit 1), a landed `milestone finalize` (exit 0) and `uninstall --force` (exit 0). Moved back, the first worktree answered git with its commit and its staged path intact.
- No repository-wide registry verb remains: grep for `"worktree"` argv in production code gives `add --detach` (2 sites), `list --porcelain` (1) and `remove --force <path>` (1, behind `remove_owned_registration`); no `prune`, `repair` or `gc` argv anywhere in crates/cli/src or crates/engine/src.
- Healthy fan-out is not refused: provision, both sub-agents `git add`, `milestone finalize` → exit 0, both files landed, every sub-task registration gone. Driven on the squash arm and on the chain arm (`finalize.fan-out.squash: false`).
- A sub-agent that commits in its worktree: `milestone finalize` exits 3 with `milestone.unlanded-work` in text and in `--format json`. The printed `git -C <worktree> reset --soft <base pin>`, run as printed from `/`, exits 0 and the re-run lands the committed file plus a separately staged one. Driven on the squash arm with one committing sub-agent and on the chain arm with both committing (two findings, two routes, exit 0, per-sub-task commits in id order).
- The shipped sub-task step tells the agent to `git add` and never `git commit` (crates/cli/packs/dev/steps/sub-task-commit.yaml), so the new refusal does not fire on the route the product itself prints.
- A sub-task settled by `jigc task discard` whose live worktree has a staged path: `milestone finalize` exits 3; the printed `git -C <worktree> stash` exits 0 as printed; the re-run lands the sibling (`1 sub-task`), and `git stash list` in the main checkout still holds the entry after the worktree is gone.
- The unlinked-checkout cell: with the `.git` entry deleted and a staged path in the registration, the refusal prints the one-line re-link; run as printed it exits 0 and the re-run lands both files.
- Idempotent re-provision on current git: a stale registration that holds nothing (directory deleted, HEAD at the base pin, nothing staged) is dropped and re-added at exit 0 with no refusal.
- A severity override cannot turn the new refusal into a landing: `blocked()` returns the blocked exit whatever grade the cascade assigns (read only, milestone.rs:7824-7848).
- False refusals in healthy states: I could not construct one. A worktree HEAD at the base pin short-circuits before any reachability walk; record commits advance the main branch only; a commit on a branch made inside the worktree, a stash, and a HEAD moved to an existing ref all read as reached. The documented sibling-worktree-HEAD false refusal needs a sub-agent to check out a commit only another linked worktree holds; it was reasoned, not driven.
- The two suites pass on this tree in a private target: `cargo nextest run -p jigc -E 'test(/worktree_registration_(reach|anchor)::/)'` → 22 run, 22 passed.
- Edits to pre-existing tests in the three commits re-derive assertions rather than widen them: the `flow53_acceptance` code-set assertion became per-door and exact (`["milestone.unlanded-work"]` for the milestone door, empty for the task door); the leaked-worktree remedy test now also asserts that no `worktree prune` text is printed.

## not_examined

- The report file the task asked for was not written: my standing instructions forbid writing report files and name this structured return as the only channel, so everything is here.
- No real git older than 2.31 was run (F2 is an emulation by PATH wrapper), and CI's git 2.43 was not driven; every drive used git 2.54.0 on macOS.
- A case-sensitive filesystem, Linux and Windows were not driven.
- The full `dev/gate` was not run; only the two worktree-registration suites.
- `cp -R` copy cells (a worktree whose registration lives in the source repository) were read, not driven — including that the printed `reset --soft` and keep commands there act on the source repository's shared admin directory.
- The restore recipe and keep command at provision, discard and uninstall were not re-driven by me; they are suite-pinned and the suite passed.
- Printed commands over paths containing spaces or quotes were not driven.
- The two other production `git status` probes that pass no `--untracked-files` (setup.rs:4594, task.rs:3530) and the provision door of F1 were not examined.
- A `git worktree add` killed mid-way (a locked, half-checked-out registration that provision would reuse) was reasoned only; it predates the pass.
- In a moved repository `milestone provision` refuses with `--force` as its only printed command while `git worktree repair` would restore the worktrees — the fixer's own report lists this as pre-existing; I did not drive it.
- Whether the round-3 commits outside this area are also missing from the decision log.
- The reflog bound, the landed envelope's missing key and the locked-and-gone registration are already deferred to M57 (decisions-pending.md:87-88); I did not re-drive them.

# AREA: create-promote-link — commits c0c4d88c · 3f3a724b · 9465f9b6 · 5f5b273a on fix/rc24-tier1 (HEAD 5f5b273a). Driven on a release binary built from HEAD in a private target dir, in throwaway `dev/jigc-rig` corpora; before-controls on the installed published 1.0.0-rc.24. The four area suites and the engine unit tests were re-run on HEAD and are green (milestone_promote_guards 11, promote_destination_shape 9, store_door_home_shape 6, create_only_gate 26, engine 12). Project tree left untouched. NOTE: the requested report file review-create-promote-link.md was NOT written — this session's system-level instructions forbid writing report .md files and say the script reads only this structured return, so everything is self-contained here.

Verdict: red — the link/shape half and the create gate hold as built, but the milestone boundary still loses a sub-task's doc or staged file at exit 0 in two driven cells inside the class 3f3a724b claims closed, and its new refusal prints a route that refuses for every fixed-identity doctype.

## CPL-1 · HIGH · Milestone boundary: two sub-tasks that each mint a placement singleton both promote to one path — one sub-task's doc is lost at exit 0, in no commit and on no disk

- where: crates/engine/src/finalize.rs:2081-2087 (promote_destination ignores the slug for a placement doctype) · :2106-2167 (plan_promotions, no duplicate-destination check) · :651-684 (refused_promotions asks only what is on disk at plan time) · crates/engine/src/milestone.rs:2569-2575 and :2614-2636 (the join suffixes a fixed-identity doc) · crates/cli/src/task.rs:5514-5540 (promote copies both in sequence). Commit 3f3a724b (guard added, cell left open). False doc sentence: design/storage.md:402.
- driven: driven (release binary built from HEAD 5f5b273a; before-control on installed 1.0.0-rc.24)
- class: Two promotions in one plan sharing one destination. Mechanism consumers: plan_promotions has 2 committing consumers (`grep 'refused_promotions('` -> 2 hits, finalize.rs:495 and :565). Reachable only at the milestone door: a single task cannot stage two addresses of a singleton (driven — `doc create changelog --slug other` hands back the existing `changelog:changelog`). Members = join suffix x placement doctypes: 5 shipped (`grep 'placement:' crates/cli/packs/*/schemas` -> changelog, decisions-log, deferral-ledger, roadmap, vision), granted to sub-tasks by 6 non-verb-routed workflows (`grep allows-create` -> single-task, record-change, decided-task, form-vision, planning, completion). Driven: 2 doctypes (changelog, vision), both squash modes for changelog. A custom pack with two doctypes sharing one `location:` would be a further member — not driven.

```text
DRIVEN, release binary from HEAD 5f5b273a, rig `fresh`.
Setup: `jigc milestone create "ship it"`; `jigc milestone add-task ship-it "alpha does work" --workflow single-task`; same for bravo. In each sub-task: `jigc doc create changelog --title Changelog --task <sub>` (both print `changelog:changelog`), add-item `added`, set-slot notes `- ENTRY-FROM-<sub>`, commit doc filled. Before-control: each marker is in exactly 1 staged file under .jigc/ (read with `command grep`).
argv: `jigc milestone join ship-it` -> exit 0: `changelog:changelog (created · from alpha-does-work)` / `changelog:changelog-2 (created · from bravo-does-work) ← suffixed -2 on collision`.
argv: `jigc milestone finalize ship-it` -> exit 0: `finalized 0d9c406 … promoted CHANGELOG.md … 2 files committed … sub-tasks: alpha-does-work: 1 doc, no worktree provisioned · bravo-does-work: 1 doc, no worktree provisioned`.
After: ENTRY-FROM-alpha in HEAD tree 1; ENTRY-FROM-bravo in HEAD tree 0, `git log --all -S` 0 commits, 0 files anywhere under the repo including .jigc/ (the sub-areas were torn down).
Same result with `finalize.fan-out.squash false`, and with `--workflow form-vision` (`vision:vision` + `vision:vision-2` -> one `promoted VISION.md`, bravo's vision gone).
Before-control on the installed 1.0.0-rc.24: identical outcome — pre-existing, left reachable by the pass.
Why the guard misses it: both destinations are free when the planner looks; the second copy lands on the first inside the same promote. Promotions sort by destination and the suffixed body's file name sorts first, so the unsuffixed one overwrites it.
The fixer's own left-open list names this cell as "not driven: two created instances of a placement/singleton doctype in one fan-out". design/storage.md:402 says the suffix-resolved overlay means finalize "never sees two docs competing for one path" — false for a placement doctype; the pass edited the next paragraph (:404) and left it. No decisions-pending row.
```

## CPL-2 · HIGH · Milestone boundary: a promote overwrites a file a sub-task worktree staged at the same path — exit 0, the staged file in no commit, while the manifest counts it as landed

- where: crates/engine/src/finalize.rs:661 (the guard reads the main checkout's worktree only) · crates/cli/src/milestone.rs:6597-6611 (planner runs before the worktrees' staged code is folded) · crates/cli/src/combine.rs:115 (detect_code_collision compares worktrees with each other, never with promote destinations; `grep -i promot combine.rs` -> 0 hits). Commit 3f3a724b.
- driven: driven (release binary built from HEAD; before-control on installed 1.0.0-rc.24)
- class: A promote destination that is also a path in a sub-task worktree's staged set. One mechanism; neither occupancy consumer (2 `refused_promotions(` hits) nor the one code-collision check (combine.rs:115) compares the two channels. Driven: 4 cells (created x other sub-task's file, squash true and false; created x own sub-task's file; edited-from-base x other sub-task's staged edit). The task-door analogue — an index-only occupant — is CPL-5. Otherwise instance-bounded to the milestone door; not enumerated further.

```text
DRIVEN, release binary from HEAD, rig `fresh`, default `finalize.fan-out.squash`.
Setup: milestone `ship-it`, sub-tasks alpha and bravo (default workflow), `jigc milestone provision ship-it`. In alpha's worktree: hand-write `docs/decisions/shared.md` carrying WORKTREE-VICTIM-MARKER and `git add` it. In bravo's worktree: `jigc doc create adr --title Shared --task bravo-does-work` -> `adr:shared`, three slots filled. Before-control: `git -C <alpha worktree> diff --cached --name-only` -> docs/decisions/shared.md.
argv: `jigc milestone finalize ship-it` -> exit 0: `finalized e4f99c4 … promoted docs/decisions/shared.md … 2 files committed … sub-tasks: alpha-does-work: 1 code file · bravo-does-work: 1 doc`.
After: victim marker in HEAD tree 0, `git log --all -S` 0 commits, 0 files on disk; `git worktree list` 1 (alpha's worktree removed). The blob survives only as an unreferenced object.
Variants driven:
(a) the same sub-task stages the file and creates the doc -> exit 0, same loss (`alpha-does-work: 1 doc, 1 code file`).
(b) `squash false` -> exit 0; alpha's file lands in its own commit 472a088 and the aggregate replaces it at HEAD — bytes in history, replaced unnamed.
(c) edited-from-base: committed `adr:seed`; alpha appends ALPHA-HAND-EDIT-MARKER to docs/decisions/seed.md in its worktree and stages it; bravo runs `jigc doc set-slot adr:seed#context … --task bravo-does-work` -> `milestone finalize` exit 0, alpha's edit in 0 commits and 0 files.
Before-control on installed 1.0.0-rc.24 (created variant): identical — pre-existing, left reachable.
design/finalize.md §4 states "A created doc never promotes over a file already at its destination — at both committing doors"; here the destination is occupied in the tree the boundary commits, not in the main checkout the guard reads. No declared bound found in finalize.md, storage.md or decisions-pending.md.
```

## CPL-3 · HIGH · The milestone boundary's new promote-clobber refusal prints a `jigc doc rename` that refuses for every fixed-identity doctype — a dead-end route

- where: crates/engine/src/finalize.rs:911-943 (sub_task_clobber_text) and :952-977 (sub_task_renames): no fixed-identity branch (0 hits for fixed_title/has_fixed_identity in those lines), while the shape arm has one at :705-728 and says at :997-998 that a route must not hand back a command that refuses. Commit 3f3a724b.
- driven: driven (release binary built from HEAD)
- class: File-arm route x fixed-identity doctypes: one constructor, 5 shipped placement singletons (same grep as CPL-1). Driven: 1 (vision, unsuffixed landing). The acceptance suite's doctype axis is idea / adr / inconsistency — all `location:` doctypes (`grep -i 'vision|changelog|placement|singleton|fixed' crates/cli/tests/milestone_promote_guards.rs` -> matches only the word 'suffixed'); the placement cell exists only at the task door's shape arm in promote_destination_shape.rs. That missing axis member masks both CPL-1 and this finding. The task-door file arm (ClobberedBy::Task, finalize.rs:848-862) also routes a singleton at retitle / `--slug` — pre-existing, read only.

```text
DRIVEN, release binary from HEAD, rig `fresh`.
Setup: `jigc milestone create "ship it"`; `jigc milestone add-task ship-it "alpha forms vision" --workflow form-vision`; `jigc doc create vision --title Vision --task alpha-forms-vision` (home free -> created), three slots and commit doc filled; then an untracked hand-written `VISION.md` (OCCUPANT-MARKER) appears.
argv: `jigc milestone finalize ship-it` -> exit 3: `blocking · finalize.promote-clobber — promoting sub-task alpha-forms-vision's doc vision:vision to VISION.md would overwrite a file already there`; route: `… give the doc a title that slugs to an id nothing holds (jigc doc rename vision:vision --to "<title>" --task alpha-forms-vision), then re-run jigc milestone finalize ship-it. The file at VISION.md is left exactly as it is — deal with it after the milestone has landed, not before …`.
argv (the route as printed, title filled): `jigc doc rename vision:vision --to "Vision Two" --task alpha-forms-vision` -> exit 1: `blocking · write.identity-change — rename rejected: vision is a singleton — its slug IS the type id …; route: nothing to rename`.
argv: `jigc milestone finalize ship-it` again -> exit 3, same finding. Occupant intact (marker 1).
The refusal itself is correct. The route is the only exit offered, it cannot be run, and it tells the reader not to touch the occupant first — the one act that would unblock (freeing the home, no commit) is named only by the shape arm. Breaks the pass's rule 3; design/finalize.md "Its route lands the milestone with every sub-task's work in it" is false for this doctype class.
```

## CPL-4 · MEDIUM · `jigc relocate` / `config set placement-root` park a displaced squatter by basename and overwrite an earlier parked file of that name — exit 0, sole copy gone (pre-existing; in a function this pass widened)

- where: crates/cli/src/relocate.rs:852-888 (displace_foreign_squatter: `workbench_dir.join(name)` then `fs::rename`), probe widened at :823 by 5f5b273a.
- driven: driven (release binary built from HEAD; before-control on installed 1.0.0-rc.24)
- class: instance, unbounded. One park site read (relocate.rs:859), reached by `jigc relocate` and `config set placement-root` through relocate_stranded. The two sibling parks — rollback::park (`<door>/<identity>.pre-image.<nanos>`) and task::displace_foreign_area (`<unit-id>/…`) — carry discriminators; their collision behaviour was not driven.

```text
DRIVEN, release binary from HEAD, rig `fresh --pack-from-dev` (manifest dropped, so `adr` is freeze-exempt).
Setup: committed `legacy/a.md`; `.jigc/displaced/a.md` holding EARLIER-PARKED-MARKER (an earlier displacement left where the report named it); an untracked squatter `docs/decisions/a.md` holding NEW-SQUATTER-MARKER.
argv: `jigc relocate adr --from legacy` -> exit 0: `moved legacy/a.md -> docs/decisions/a.md` / `displaced docs/decisions/a.md -> .jigc/displaced/a.md (foreign squatter → workbench)`.
After: EARLIER-PARKED-MARKER in 0 files; `.jigc/displaced/a.md` now holds the new squatter. Nothing names the overwritten copy.
Before-control on installed 1.0.0-rc.24: same loss — not introduced by the pass. 5f5b273a made dangling links displaceable through this arm and kept the park naming. Graded MEDIUM rather than HIGH only because it needs an earlier parked file still in the workbench; the harm is an exit-0 loss of a sole copy through a moving door.
```

## CPL-5 · MEDIUM · The absent-home cell is still open at both doors, and its only record is a design-doc open question — no decisions-pending trigger row

- where: crates/engine/src/finalize.rs:651-684 (worktree-only occupancy) · design/reconciliation.md:233 (stated, added by 8d9c3afb) · implementation/decisions-pending.md (no row; last touched at 614d9c99).
- driven: driven (release binary built from HEAD)
- class: A home git holds (HEAD or index) that the disk does not. Members by the doc's own enumeration: task finalize, milestone finalize (suffix landing included), milestone create over a deleted record = 3 doors; plus the index-only variant found here. Driven by me: task finalize x {HEAD-held with no baseline, index-only}. Milestone finalize and milestone create not re-driven.

```text
DRIVEN, release binary from HEAD, rig `fresh`.
(1) Committed `adr:cache-strategy` (SEED-MARKER). `.jigc/state/file-state.json` moved away (a clone's state); the doc's file removed from the worktree, uncommitted (` D`). New task: `jigc doc create adr --title "Cache Strategy"` -> `adr:cache-strategy` (created). `jigc task validate` -> exit 0. `jigc task finalize` -> exit 0: `promoted docs/decisions/cache-strategy.md`; the commit shows `3 insertions(+), 3 deletions(-)` on that path — the committed doc replaced under its own id. Its bytes remain at the parent commit.
Control with the baseline present: exit 3, `reconciliation.rename`.
(2) Index-only occupant at the task door: created `adr:fresh-one`; `git add docs/decisions/fresh-one.md` (INDEX-ONLY-OCC) then remove it from the worktree (`AD`). `jigc task finalize` -> exit 0, `promoted docs/decisions/fresh-one.md`; the staged blob is in 0 commits.
reconciliation.md:233 declares cell (1) as "not closed, because closing it is a decision about jigc unmanage", and says it was driven at three doors. But decisions-pending.md has no row for it, and `grep` finds none of the round-3 commits (5f5b273a, e842342e, 8d9c3afb, 4fa1c0f5, ef9456b5, 104a7d4b) in DECISIONS.md or decisions-pending.md. By this repo's own rule a deferral left only in prose has no trigger. Cell (2) is stated nowhere; the guard's doc-comment (finalize.rs:614-620) says a merely-staged file is seen because it is in the worktree, which an index-only one is not.
```

## CPL-6 · MEDIUM · `jigc migrate-corpus`'s relocation arm probes its destination through links and replaces a dangling link there at exit 0, unnamed — a follow-links probe in front of a replacing write, left unconverted and unpinned

- where: crates/cli/src/migrate_corpus.rs:966-977 (`std::fs::read(repo_root.join(target)).ok()` as the collision probe) and :995 (temp+rename over the destination); also :2440 `is_file()`. Commit 5f5b273a declares the in-place arm only (design/finalize.md:134).
- driven: driven (release binary built from HEAD)
- class: Follow-links probe in front of a write at a managed home, in migrate_corpus.rs: 2 sites by grep of `is_file()|exists()|fs::read` in the file's production half (:969 relocation collision probe; :2440 exists_in, read-only use). Driven: the relocation destination x {untracked dangling link, tracked dangling link}. In-place arm: the fixer's pin, not re-driven. A directory at the destination was read (rename over it fails, nothing lost), not driven.

```text
DRIVEN, release binary from HEAD, rig `fresh`.
Setup: commit a v1 changelog at its old home `docs/changelog/changelog.md` (`schema-version: 1`); plant an untracked `CHANGELOG.md -> nowhere-important.md` at the relocation destination.
argv: `jigc migrate-corpus --dry-run` -> exit 0, `1 would migrate, 0 blocked`.
argv: `jigc migrate-corpus` -> exit 0: `1 migrated, 0 already current, 0 blocked … committed df63ad9`. After: `CHANGELOG.md` is a regular file; the link is gone and named by nothing. Tracked-link variant: same, the link replaced in the commit.
The arm's own rule (comment at :952-961) is that a destination holding anything else is blocked, nothing written. A dangling link reads as nothing there, so the rule is skipped. Sibling doors disagree about the same state: `jigc relocate` parks and reports it, `jigc rename` refuses it (`write.already-present`).
The loss is the link entry (its target string), which is why this is MEDIUM. The pass's pin (`the_corpus_migration_lands_a_regular_file_and_writes_through_nothing`) covers the in-place arm only. finalize.md:134 says whether the door should refuse "is not ruled here"; there is no decisions-pending row.
```

## CPL-7 · MEDIUM · `finalize.promote-clobber` at `jigc rename` reaches a `--format json` driver as an `{"error": …}` string with no key, while the contract row says it is keyed at the home

- where: crates/cli/src/rename.rs:354-368 (foreign_home -> render::finding_error) · crates/cli/src/relocate.rs:210-225 · design/command-output-contract.md:356 · crates/engine/src/finalize.rs:1160-1169 (the stated rationale). Commit 5f5b273a.
- driven: driven (release binary built from HEAD)
- class: Store doors raising the reused code through the flattening funnel: `grep 'store_home_refusal('` in crates/cli/src -> 2 production sites (rename.rs:361, relocate.rs:215), reached by 4 doors (rename, relocate, config set docs-root, config set placement-root). Wire shape driven at 1 (rename); config set driven in text mode only.

```text
DRIVEN, release binary from HEAD: a committed `adr:cache-strategy` whose home is a committed live link.
argv: `jigc rename adr:cache-strategy --to "Cache Plan" --format json` -> exit 1; stdout empty; stderr `{ "error": "blocking · finalize.promote-clobber — docs/decisions/cache-strategy.md is a symbolic link … at: docs/decisions/… route: …" }`.
At the committing doors the same code arrives in the findings envelope with a `(code, target)` key. command-output-contract.md:356 says the second arm "is also raised outside a finalize … keyed at that home", and the constructor's doc says a driver that learnt the code "has nothing new to learn at these doors". At the wire the driver has no key to read.
At `config set docs-root` the code is flattened into the message of `config.repoint-failed` (config.rs:1402-1405 wraps the error as a Cause, where the placement-root arm's sweep-ending refusals are carried as Raised). Both printed routes there work.
The text-mode message and route are true at rename and at the move primitive (driven for a live link, a dangling link and a directory at the doc's own home; and under relocate, docs-root and placement-root).
```

## CPL-8 · LOW · A refused `config set placement-root` leaves an already-displaced squatter in the gitignored workbench, and the route's all-undone sentence does not mention it

- where: crates/cli/src/relocate.rs:418-424 (declared bound: a displaced squatter is not put back) · :749-780 (relocate_one) · crates/cli/src/config.rs:662-680 (the route text).
- driven: driven (release binary built from HEAD)
- class: instance — the one declared residual of rollback_relocations, newly reachable through the 1 new refusal (refuse_foreign_source). `--format json` narration not driven.

```text
DRIVEN, rig `committed-singletons`: `docs/roadmap.md` made a committed link; an untracked squatter `handbook/decisions-log.md` (SQUATTER-MARKER).
argv: `jigc config set placement-root handbook` -> exit 1, `config.repoint-failed` carrying the link refusal; route: `placement-root is unchanged and every doc this re-point moved is back at its prior home`.
After: git status clean, file-state byte-identical, knob unchanged, the link untouched — the managed docs are all-or-nothing. But the squatter is now at `.jigc/displaced/decisions-log.md` and absent from `handbook/`. Only the stderr narration line names the move.
The residual itself is M52's declared bound. What is new is the trigger: this pass's ForeignSource refusal on doc k after doc j displaced a squatter. Bytes survive.
```

## CPL-9 · LOW · Round-3 fix 5f5b273a has no as-built DECISIONS entry and no decisions-pending rows; `relocate` and `config set` help do not state their new refusal; the dangling-link route names no HEAD restore

- where: DECISIONS.md (the as-built entry lists ten commits and ends before round 3) · implementation/decisions-pending.md · `jigc relocate --help`, `jigc config set --help` · crates/engine/src/finalize.rs:1119-1124.
- driven: read only for (a); driven for (b), (c), (d)
- class: instance, unbounded — records and wording, not enumerated.

```text
READ and partly driven.
(a) `grep -c 5f5b273a` -> 0 in DECISIONS.md, decisions-pending.md and project-history.md. The fixer's own choices — `finalize.promote-clobber` at rename and the move primitive, `reconciliation.conflict-block` at the record doors, two commands that now refuse, migrate-corpus left as is — are flagged in the fixer report as CONFIRM / HUMAN'S CALL and recorded nowhere. DECISIONS item 5 says "None is built at this entry".
(b) Driven: `--help` mentions of link / regular file -> rename 1, relocate 0, config set 0, milestone add-task 0, milestone discard 0. The design docs were revised in the commit.
(c) Driven: `jigc rename` over an uncommitted dangling link at the doc's own home routes at "for a link, a copy of the file it points at … and commit that". A dangling link points at nothing, and where HEAD still holds the doc there is nothing to commit; the record door's route names `git checkout HEAD -- <path>` for the same state, this one does not.
(d) A refused docs-root re-point leaves empty destination directories behind (`handbook/decisions/`) — untracked, harmless.
```

## CPL-10 · LOW · Engine pub signatures moved in a published rc crate; every in-workspace caller is updated

- where: crates/engine/src/state.rs (create, create_gated, create_occupied, already_exists_finding) · crates/engine/src/finalize.rs (plan_milestone_finalize) · crates/engine/src/milestone.rs (MaterializeOutcome gains a pub field).
- driven: read only
- class: The pub-diff grep above, whole engine crate across the pass; the callers counted are the area's.

```text
READ. `git diff origin/main..HEAD -- crates/engine/src | grep '^[-+]pub '` shows the re-signed items. Callers by grep in crates/cli/src: create_gated 2 (doc.rs:4084, :4210), create_occupied 1 (doc.rs:3535), already_exists_finding 1 (doc.rs:3672), plan_milestone_finalize 1 (milestone.rs:6597), plan_finalize 1 (task.rs:3547); the ungated `state::create` has no production caller. The release build is green. jigc-engine is published at 0.1.0-rc.2, so the release PR's semver check may flag the break; DECISIONS records it for (R6, K-1) and (R3, F7) only.
```

## held

- Task door, created doc over an occupant that appears after the create: an untracked file and a `git add`-ed file both refuse at exit 3 with `finalize.promote-clobber`, occupant bytes intact; after the occupant is moved out the same finalize lands.
- Task door rollback under a rejecting pre-commit hook, with a created doc and an edited-from-base doc in one promote: exit 1, the created copy removed, the edited doc byte-identical to before, git status clean; the next finalize lands both as 0644 regular files.
- A plain create of a placement singleton over an untracked NON-conformant `VISION.md`: copied in as edited-from-base, but every write refuses (`write.non-reparseable`) and validate / finalize exit 3 — the foreign bytes cannot be overwritten that way.
- Create-only gate (`new: true`) over a dangling link at the home: exit 1 `create.already-exists`, nothing staged; the printed `--slug` route mints beside it; the link is untouched.
- A single task cannot stage two addresses of a singleton: `doc create changelog --slug other` hands back the existing `changelog:changelog`; a different title refuses `write.title-ignored`.
- `config set docs-root` over a batch whose middle doc is a committed live link: exit 1, the two docs already moved are back, git status clean, file-state byte-identical, knob unchanged, link untouched.
- `config set placement-root` over a linked placement doc: managed docs all-or-nothing, knob unchanged, file-state identical (the displaced-squatter residual is CPL-8).
- `jigc relocate` over a link source: exit-0 triage with a blocked row carrying the code and a true message and route; the regular sibling moves. The rc.24 control moved the link — the fix is real.
- `jigc rename` regression: with a `supersedes` referrer and a rejecting hook, exit 1, HEAD unchanged, worktree and file-state byte-identical; then the rename lands with `repointed 1 referrer(s)` and the referrer names the new id; the retitle-only arm rewrites the H1 in place.
- `jigc rename` over a dangling link and over a directory at the doc's own home: refused before any write with a true message; the doc renames normally once the regular file is back.
- Record doors over a record deleted from the worktree: same code-less refusals as rc.24 — no behaviour change from the switch to `rewrite_home`.
- Rollback paths that still follow links (PreImage::capture, the restore's `fs::write`): read at every site in the area (promote, rename, relocate, record doors, record-flip guard). Each sits behind a no-follow gate asked earlier in the same run, so a link reaches them only if something swaps one in mid-transaction — the external-writer class already deferred to M57. `create_new` failing before `pre.wrote()` leaves the entry untouched, so a racer's file at the record home is not deleted.
- No pack file, schema manifest or compose golden moved in the four commits (stat read); the registry counts that moved (11 -> 12 rename refusals, relocate 10 -> 11 states) are new members, not widened snapshots.
- The four area suites and the engine unit tests pass on HEAD 5f5b273a (63 CLI tests, 12 engine tests, all exit 0).

## not_examined

- The report file review-create-promote-link.md was not written (system-level instruction against report files); this return is the whole report.
- The K-1 race window itself: the FIFO harness and the 60-run toggler were not re-run — only sequential gate cells were driven.
- Milestone-boundary shape arm (link / directory at a sub-task doc's home) and its routes; the file arm's rename route for ordinary `location:` doctypes: the suite passes on HEAD, not re-driven by hand.
- Absent-home cell at `milestone finalize` and at `milestone create` over a deleted record: read from reconciliation.md, not re-driven.
- Record doors over a link (add-task, add-from-spec, discard, task discard, finalize flip) and `milestone create` over a link at the record home: code read, suite green, not driven by hand.
- `config set docs-root` silently skipping a dangling-link source (the fixer's declared cell) and `--format json` narration of a refused re-point.
- migrate-corpus in-place arm over a link, and a directory at its relocation destination.
- Symlinked parent directories of a home, special files (FIFO / device) at a home, Linux, and case-sensitive filesystems — every drive ran on macOS.
- The full `dev/gate` on HEAD; only the area's suites were run.
- The other fix commits of the pass (setup, uninstall, reconcile, worktree registration, linked-worktree guard) except where a drive crossed them.

# AREA: install-teardown — `jigc setup`, `jigc uninstall`, and the version stamp at `jigc task finalize` (commits 12398ddc · dc0d7586 · 104a7d4b · ef9456b5 · 4fa1c0f5, reviewed at HEAD 5f5b273a). Full report: <scratch>/fix-rc25/audit/review-install-teardown.md

Verdict: red — the status-blind ask added in ef9456b5 makes `jigc setup` refuse a clean, committed, unflagged install path with a false cause and a route that cannot clear it (a regression from rc.24), and three exit-0 losses that predate the pass stay reachable at `setup` and `uninstall`, one of them inside the class `(R1, F1)` is recorded as closing.

## F1 · HIGH · Regression with a dead-end route: a clean tracked install path whose committed blob holds CRLF refuses `jigc setup` as "flagged", on every run

- where: crates/cli/src/setup.rs:3005-3024 (`unseen_by_status` compares `git hash-object -- <paths>` with the index blob), claim at :2933-2938; wording crates/engine/src/finalize.rs:1635-1639, route :1698-1716; design/validation.md:676; commit ef9456b5
- driven: driven (release binary built from 5f5b273a; control on published jigc 1.0.0-rc.24)
- class: One comparison site (setup.rs:3005-3024), one gating caller (`InstallSubject::probe`, setup.rs:2566), asked of every present tracked install member — the table of 10 read at setup.rs:1949-2012, 2 driven (`CLAUDE.md`, `.claude/settings.json`). Triggers: 2 driven (`core.autocrlf=input`, `* text=auto`) plus 1 read-only sibling at setup.rs:3023 (a failing `hash-object` flags the whole tracked set, same route). The trigger set is an instance, unbounded: git's conversion modes and filters were not enumerated.

```text
Observed mechanism: `git status` reads the path clean while `git hash-object -- CLAUDE.md` returns a different blob id than the index entry (887784cb… vs fa1fe3ef…) when the blob holds CRLF and conversion is in auto mode. The code reads any mismatch as a flag; it never reads the flag.
SETUP: `bare` rig; `git config core.autocrlf false`; `printf '# Team notes\r\n\r\nUse tabs.\r\n' > CLAUDE.md`; `git add CLAUDE.md && git commit -m notes`; `git config core.autocrlf input`. `git status --porcelain` is empty.
ARGV: `jigc setup` -> exit 1, `blocking · setup.dirty-install-path`, "`CLAUDE.md`: tracked, with the change hidden by its index entry (assume-unchanged or skip-worktree)".
ROUTE AS PRINTED: `git -C <repo> update-index --no-assume-unchanged -- CLAUDE.md` -> 0; `git -C <repo> update-index --no-skip-worktree -- CLAUDE.md` -> 0; `git status --porcelain` -> empty; `git commit -am keep` -> exit 1 "nothing to commit, working tree clean"; `git stash` -> "No local changes to save"; `jigc setup` -> exit 1, the identical refusal.
CONTROLS: published rc.24 on the same repository -> exit 0, install commit made. This tree with no conversion configured -> exit 0.
SECOND TRIGGER: no `core.autocrlf`, a committed `.gitattributes` holding `* text=auto` -> exit 1 listing `CLAUDE.md` and `.claude/settings.json`.
UPGRADE: rc.24 installs into the CRLF repository (exit 0, re-run exit 0); this tree's `jigc setup` -> exit 1 over `CLAUDE.md`.
PERMANENCE: `jigc setup --force` -> exit 0, its `setup.forced-install-path` advisory says `CLAUDE.md` "carried changes in no commit" (false); the next plain `jigc setup` -> exit 1; after a second `--force`, exit 1 again.
TEST MASKING: cells 33 and 34 (crates/cli/tests/setup_install_pathspec_guard.rs:2284, :2408) plant LF-only content; grep for `autocrlf|crlf|text=auto|gitattributes` over the three setup suites, setup.rs and the two design docs returns 0 hits.
```

## F2 · HIGH · Exit-0 loss, unchanged from rc.24 and declared nowhere: when `git status` fails, `jigc setup` installs blind over uncommitted bytes

- where: crates/cli/src/setup.rs:2573-2581 (`probe` returns `InstallSubject::Unknown`), :1524 (the gate's empty `Unknown` arm), :3327-3329 (commit skipped); DECISIONS.md:23 ("`jigc setup` never replaces bytes no commit holds without refusing first"); no row in implementation/decisions-pending.md; design/validation.md:676 does not state the arm
- driven: driven (release binary from 5f5b273a and published rc.24, identical result)
- class: 3 `InstallSubject::Unknown` sites by grep over setup.rs (:1524, :2580, :3327). Triggers: 3 driven (submodule gitdir gone, unreadable index, no git on PATH); the trigger set is an instance, unbounded.

```text
SETUP A: `fresh` rig; `git submodule add` a local repository at vendor/sub, commit, then move `.git/modules/vendor/sub` out. `git status` -> 128, `git rev-parse --is-inside-work-tree` -> 0. Append a marked line to the tracked `.jigc/AGENT.md`, uncommitted.
ARGV: `jigc setup` -> exit 0, "jigc setup — adapter installed"; mark on disk 0, `git log --all -S <mark>` 0 hits. Same on rc.24.
SETUP B: same edit, `.git/index` overwritten with garbage instead (`git status` 128, `rev-parse` 0) -> `jigc setup` exit 0, mark gone, in no object; both binaries.
`--force` in setup A: `jigc setup --force --format json` -> exit 0, `findings: []` — the consent names nothing it replaced.
NO GIT ON PATH: `env PATH=<empty dir> jigc setup` -> exit 1 `setup.install-hook` ("cannot install the `pre-commit` hook into the repo's hooks dir: No such file or directory", route "ensure the repo's git hooks directory is writable"); mark gone; both binaries.
No test in the three setup suites names the arm (grep `Unknown`: 0 hits). The ef9456b5 fixer reported only the exit-1 shape, as a human decision; that deferral is in no tracked file (see F7).
```

## F3 · HIGH · Exit-0 loss, pre-existing: the guide's ownership oracle proves the body only, so an uncommitted front-matter edit is replaced by `setup` and deleted by `uninstall`

- where: crates/cli/src/setup.rs:285-295 (`recorded_body_digest`), :273-277, member declared `ExemptWhenJigcOwned` + `OwnContent::GuideDigest` at :2001-2006, oracle at :2155-2159 (ef9456b5), teardown at :3998; the installed file's own promise at :114-120 ("Edit it and it becomes yours: every later `setup` leaves it byte-identical")
- driven: driven (release binary from 5f5b273a; setup cell also on published rc.24)
- class: 4 consumers of the oracle by grep (`guide_ownership(` at setup.rs:1435, setup.rs:3998, upgrade.rs:136; `GuideDigest` at setup.rs:2155); the 2 that destroy (setup, uninstall) are both driven. The blind region is the whole front matter: both profile keys and an added key were driven.

```text
SETUP: `fresh` rig; in `.claude/skills/jigc/SKILL.md` replace the `description:` line with a marked one and add a marked `allowed-tools:` line, body untouched. `git status --porcelain` -> ` M .claude/skills/jigc/SKILL.md`.
ARGV: `jigc setup --format json` -> exit 0, `findings: []`; both marks gone from disk, `git log --all -S <mark>` 0 hits, `git status` empty. Same on rc.24.
ARGV: same edit, `jigc uninstall` -> exit 0, "- removed jigc guide artifact", file gone, mark in no object.
CONTROL (body edit): `jigc setup` -> exit 0 with `adapter-guide.user-modified`, bytes kept; `jigc uninstall` -> exit 0, file left, same advisory.
The dirty gate sees the ` M` and exempts the path because the oracle answers "jigc's own".
```

## F4 · HIGH · Exit-0 loss, pre-existing and outside the class `(R9, F5)` names: `jigc uninstall` deletes a standalone jigc `pre-commit` hook the adopter extended

- where: crates/cli/src/setup.rs:997-1002 (`classify_precommit_for_teardown`: a prefix match decides the file is wholly jigc's), removal at :1032
- driven: driven (release binary from 5f5b273a and published rc.24, identical result)
- class: instance, unbounded. Of the teardown's host-file classifiers, `unwire_reference` was read (exact match, holds); the three settings-file removals were not examined.

```text
SETUP: `fresh` rig; insert `npm run lint || exit 1   # <mark>` before the final `exit 0` of `.git/hooks/pre-commit` (untracked by nature, so in no object).
ARGV: `jigc uninstall` -> exit 0, "- removed pre-commit hook", no warning or advisory about the hook, file gone. Same on rc.24.
CONTROL: the same edit followed by `jigc setup` -> exit 0 and the line is kept — the install side treats the edited hook as foreign and wraps it, the teardown side treats it as jigc's and removes it.
```

## F5 · MEDIUM · The repository ef9456b5 makes first-class is certified at `setup` only: with jigc's paths gitignored, `task finalize` cannot land and `uninstall` refuses over jigc's own files

- where: crates/cli/src/task.rs:7063-7069 (`existing_pathspecs` filters on existence, not on ignore), :6494-6495; crates/cli/src/setup.rs:4577-4611; cells 31/32 at crates/cli/tests/setup_install_pathspec_guard.rs:2021, :2111; design/validation.md:676 ("keeps re-running `setup` at exit 0")
- driven: driven (release binary from 5f5b273a; the `.jigc/` cells also on published rc.24)
- class: 4 ignore shapes driven at finalize (3 fail), 3 doors driven (setup, task finalize, uninstall); other doors unbounded.

```text
SETUP: `bare` rig, a committed `.gitignore` holding one pattern; `jigc setup` twice; `jigc start --workflow single-task …`, fill the commit doc, stage one file.
ARGV `jigc task finalize <id>` per pattern: `.jigc/AGENT.md` -> exit 0; `.jigc/version` -> exit 3 `finalize.stage-failed`; `.jigc/config/` -> exit 3; `.jigc/` -> exit 3 ("`git add -- :(literal).jigc/config :(literal).jigc/.gitignore :(literal).jigc/version` failed: The following paths are ignored…", route "once the embedded git failure is resolved (e.g. remove a stale `.git/index.lock`)"). `setup` exited 0 both times in all four. `.jigc/` cell identical on rc.24.
ARGV `jigc uninstall` with `.jigc/` ignored and never edited -> exit 1 `uninstall.untracked-workbench-file` listing `.jigc/.gitignore`, `.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`, `.jigc/version` — all jigc-generated. Same on rc.24.
Neither is introduced by the pass; the pass's suite and design row certify this repository shape at one door.
```

## F6 · MEDIUM · A doc that is false: "whatever this door takes, it names" — `uninstall` takes jigc's own files in a milestone area at exit 0, named by nothing

- where: crates/cli/src/setup.rs:3777-3782 ("the four guards and that one narration partition every byte under `.jigc/`"); design/project-setup.md:154 (G5, (d)); commit 12398ddc
- driven: driven (release binary from 5f5b273a)
- class: jigc's own registry rows in the work-unit areas: 6 milestone rows + 15 task rows, read off crates/engine/src/state.rs:246-253 and :186-202; 4 driven (milestone area). The task-area rows were not driven.

```text
SETUP: `fresh` rig; `jigc milestone create "Ship the cache"` -> exit 0.
ARGV: `jigc uninstall` -> exit 0. Removed and neither refused over nor named: `.jigc/milestones/ship-the-cache/base.json`, `record-commit-msg.txt`, `staged-snapshot.json`, `tasks.json`. The narration lists 5 tracked files and 2 files under `state/` only.
The completeness arm (n) of crates/cli/tests/uninstall_workbench_subject.rs:774 reads `state/` and `index/` only, so the claim is pinned for those directories and not for the work-unit areas. No operator bytes are lost here; the stated rule is what fails.
```

## F7 · MEDIUM · Records: the three round-3 commits are in no log, and what they left open has no trigger row

- where: DECISIONS.md:5-15 ("None is built at this entry"); implementation/decisions-pending.md:75-91 (the M57 rows); commits 104a7d4b, ef9456b5, 4fa1c0f5
- driven: read only
- class: 6 open items counted from the `left_open` sections of the three fixer reports; not re-derived against the tree.

```text
grep for `104a7d4b|ef9456b5|4fa1c0f5` over DECISIONS.md, implementation/decisions-pending.md and implementation/project-history.md: 0 hits; `git log origin/main..HEAD` over those three files shows only bca470c1, fcd9af18, 614d9c99, all before round 3.
Open items that exist only in the fixers' reports: the `InstallSubject::Unknown` arm (F2); the index-flag class applied to merged-into members; leave-and-say-so rather than refuse at `task finalize`; the teardown-log bound (the log is cleared twice after a route through another jigc verb); a linked `.claude/skills` cannot take the install; the tracked-path-deleted wedge at the commit-time backstop.
Also unrecorded: `jigc_engine::finalize::setup_dirty_install_finding` gained a fourth argument and `SetupUnseen` is a new pub struct in the published crate.
```

## F8 · LOW · Following the unborn refusal's commit arm silently forfeits the secrets-floor `.gitignore`

- where: crates/engine/src/finalize.rs:1772-1778 (the commit arm); crates/cli/src/setup.rs:1140-1143 (`seed_secrets_gitignore` runs on an unborn HEAD only); commit dc0d7586
- driven: driven (release binary from 5f5b273a)
- class: instance, unbounded

```text
SETUP: unborn repository, untracked `.jigc/AGENT.md` (prose) and `CLAUDE.md`.
ARGV: `jigc setup` -> exit 1, route "…or commit them in one commit with `CLAUDE.md`…". `git add -- .jigc/AGENT.md CLAUDE.md && git commit -m "our notes"` -> 0. `jigc setup` -> exit 0, status empty, no root `.gitignore`.
CONTROL: plain unborn `jigc setup` -> exit 0 and the root `.gitignore` is seeded. The route does not say the commit ends the fresh-repository path.
```

## held

- First-time on-ramp: unborn repository with untracked `CLAUDE.md`, `.gitignore`, `.claude/settings.json` -> `jigc setup` exit 0, the adopter's mark in the first commit, re-run exit 0 with no second commit.
- On-ramp with `.claude/` gitignored on an unborn HEAD: three `setup` runs exit 0. Born HEAD with an ignored hand-made `.claude/settings.json`: merged into, the adopter's permit kept.
- No git identity on the first unborn run (exit 1 `setup.install-commit`), identity set, re-run -> exit 0, one commit, status empty, mark in HEAD.
- Whole `.jigc/` gitignored and never edited: three `jigc setup` runs exit 0 (the footprint record carries the provenance).
- Upgrade re-run: all six installed rig states built by published rc.24, then this tree's `jigc setup` twice -> exit 0 each, status clean.
- Ignored hand-written `.jigc/AGENT.md` on a born HEAD: refusal names it as ignored; the printed move followed; re-run exit 0; notes kept.
- Committed symlink at `.jigc/config/packs.yaml`: `jigc setup --force` still exit 1 `setup.compose-marker`, target intact; `git rm` + commit + re-run -> exit 0 with a regular file there.
- A symlinked `.claude/skills`: exit 1 on both binaries, so no regression; this tree writes nothing into the linked directory where rc.24 wrote the guide into it.
- Tried to find a new route into the blind `Unknown` arm through the added `git ls-files` call: `git ls-files -s -z` over a path beyond a symlinked directory exits 0, tracked or untracked link.
- Version stamp at `task finalize`: hand-edited prose (exit 0, advisory, bytes intact, stamp left unstaged, commit holds the code only); committed symlink whose target is stamp-shaped (exit 0, `setup.version-stamp` at advisory in the JSON envelope, target and link untouched); stamp deleted (re-created and committed, no finding).
- Uninstall and the invocation log: refusal under `uninstall.untracked-workbench-file` with the ignored-path route; log moved out; teardown exit 0; `.jigc` not re-created; second run a no-op.
- Uninstall `--force` over plants in all six transient prefixes at depth 1 and depth 2, plus the log: every one named on stderr.
- Uninstall over all six installed rig states: 5 exit 0, 1 refused with `uninstall.staged-prose` over a genuinely open task; none called jigc's own cache files foreign.
- `unwire_reference` strips only the exact section jigc appended (read at adapter.rs:1004-1042).
- `InstallMember` table against the writers: production writers enumerated by grep over adapter.rs, setup.rs and gitignore.rs; every write target is a table row with the declared writer kind.
- The area's own suites at HEAD in a private target: `setup_install_pathspec_guard::`, `replacing_writers_never_follow::`, `setup_failed_first_run::` 59 passed; `uninstall_workbench_subject::` 23 passed.

## not_examined

- The full `dev/gate` at HEAD — only the area's four suites were run.
- A FIFO at `.jigc/version` (the store sweep's plain read), and the `MigrationFixed` and fan-out finalize arms.
- `jigc uninstall` typed from inside a fan-out worktree or a subdirectory with the log knob on.
- A flagged or dirty in-worktree `pre-commit` hook under `core.hooksPath`, and `(R1, F2)`.
- The three settings-file removals at `uninstall`, and `jigc config set` as a replacing writer through a link.
- Races between a door's question and its write.
- Linux — everything was driven on macOS with git 2.54.0.
- The task-area half of F6 and any ignore shape beyond the four in F5.
- Whether F1 has further triggers (custom clean filters, `core.safecrlf`, LFS) — reasoned from the read at setup.rs:3023, not driven.

# AREA: reconcile-baseline — the (R3, F7) fix: baseline recorded at both copy-in sites + base-pin backstop (eb18e5fe), and the milestone-record door comparing against HEAD with no baseline (8d9c3afb). Reviewed at 5f5b273a on fix/rc24-tier1; driven on a private debug build of this tree beside the published 1.0.0-rc.24 as control. Full report: scratchpad fix-rc25/audit/review-reconcile-baseline.md; probe scripts in fix-rc25/audit/rb/.

Verdict: red — the two rules work on the cells their tests iterate, but a hand edit made after a task's first write is still overwritten at exit 0 and left in no git object by three plain sequences on this build (F1, F2, F4), and the new backstop refuses with no working printed exit in an eol-converting checkout where rc.24 landed (F3).

## F1 · HIGH · Another writer moves the doc's baseline between the hand edit and the holding task's finalize: exit 0, the edit in no git object (unrelated task's finalize, or `jigc ingest`)

- where: crates/engine/src/file_state.rs:929,968 (task_touched = the sweeping task's own area only) -> :629-631 absorb / :586-589 adopt re-hash the key to on-disk bytes; crates/engine/src/state.rs:1112-1124 staged_in_another_task is consulted at the copy-in only (file_state.rs:786); crates/cli/src/ingest.rs:251. Pre-existing on rc.24 in a baselined checkout; not closed by eb18e5fe, which applies 'the record is per path, not per task' to the copy-in writer alone.
- driven: driven (this tree's debug build; rc.24 as control)
- class: Every production writer of a path's file-state key other than the staging task. Count derived by `command grep -n "\.record(" crates/engine/src/file_state.rs crates/cli/src/*.rs` minus #[cfg(test)] modules: 10 lines (engine 588, 650, 789; cli migrate_corpus.rs:1012,1018 · milestone.rs:1476 · relocate.rs:297 · rename.rs:1116 · task.rs:7895,7909) plus ingest's writer behind absorbed_drift (ingest.rs:251). Only :789 asks whether another open task holds the doc. Driven at 2 movers (another task's landed finalize; `jigc ingest`); rename, migrate-corpus, relocate, the milestone join's sweep and a fan-out sibling's sub-task finalize are read only, not driven.

```text
REPRO A (this build, in a real clone and in a baselined checkout; rc.24 baselined identical). setup: rig committed-singletons; `jigc start --workflow single-task "beta pass"`; `jigc start --workflow single-task "alpha pass"`; `jigc doc set-slot vision:vision#open-questions --from-file <f> --task beta-pass` (copy-in); author beta's commit doc; append a marker paragraph under `## Thesis` in VISION.md, uncommitted. argv: `git add alpha.rs`, author alpha's commit doc, `jigc task finalize alpha-pass`; then `jigc task finalize beta-pass`. observed: alpha exit 0, prints `advisory · reconciliation.absorb — external edit absorbed: VISION.md ... no action needed`, commits alpha.rs only (git status still ` M VISION.md`); beta EXIT 0, `finalized ... promoted VISION.md`. After: marker worktree 0 · HEAD 0 · files under .jigc 0 · blobs in the object database holding it 0.
REPRO B (this build and rc.24; one task only). Same setup with only beta-pass; after the hand edit run `jigc ingest` -> exit 0, `advisory · file-state.absorbed — out-of-band edit to VISION.md absorbed — its file-state baseline now records the on-disk bytes ... no action needed`, key moved; `jigc task finalize beta-pass` -> EXIT 0, marker worktree 0 · HEAD 0 · .jigc 0 · blobs 0. Control: without the ingest the same finalize exits 3 on reconciliation.conflict-block and the marker survives.
Not in any record: tier1-verification/R3-F7.md §3 drives one unrelated landed finalize FIRST (to obtain a baseline), never between the hand edit and the holding task's finalize; implementation/decisions-pending.md:79 names only non-jigc-writer windows.
```

## F2 · HIGH · The declared residual (staged doc, no record, no blob at the pin) is reachable from one task by a plain sequence, and jigc's own route directs the hand edit it then overwrites

- where: crates/engine/src/file_state.rs:784-787 (the two Unrecorded arms) x :580-585 (backstop needs pinned(path) = Some) -> :586-607; crates/cli/src/task.rs:2624-2626. Kept cell stated at design/reconciliation.md:50 and design/storage.md:341; deferred at implementation/decisions-pending.md:79, which says the loss it leaves reachable is 'two tasks and a hand edit between their copy-ins of an untracked doc ... not driven'. (eb18e5fe)
- driven: driven (this tree's debug build)
- class: {no record} x {no blob at the pin}. No-record causes read off the backstop's own comment (file_state.rs:569-579) and the Unrecorded enum (:697-708): non-conformant at copy-in · staged elsewhere · key lost after the copy-in · copied in by an older binary = 4. No-blob causes that get past the other finalize guards = 1 (untracked): `git add`ed-uncommitted is refused by finalize.carried-staged and committed-after-mint by finalize.base-mismatch, both driven. 3 of the 4 cells driven, all exit-0 losses; the older-binary cell is read only. `jigc milestone finalize` shares the arm, not driven.

```text
REPRO (this build). setup: rig fresh; write an untracked draft docs/decisions/draft-queue.md — valid front-matter, schema-version 2, Context/Options/Decision filled, Consequences EMPTY (parses; fails required-slot-present). `jigc validate` prints `advisory · file-state.un-baselined` and `blocking · schema-conformance.required-slot-present ... route: corrupt — ... review it by hand`. `jigc start --workflow single-task "finish the queue decision"`. argv: `jigc doc set-slot adr:draft-queue#consequences --from-file <f> --task finish-the-queue-decision` -> exit 0, `copied in for update`, no baseline-adopt line, no file-state.json written; author the commit doc; add a marker line under Context in the file on disk; `jigc task finalize finish-the-queue-decision`. observed: EXIT 0; the finalize prints `advisory · reconciliation.conformance-block — unvetted file docs/decisions/draft-queue.md ... route: fix the file to restore conformance, or revert the edit — this is the one case a managed file is yours to hand-edit`, then `finalized ... promoted docs/decisions/draft-queue.md`. After: marker worktree 0 · HEAD 0 · .jigc 0 · no blob holds it. control: the same draft with Consequences filled -> copy-in prints baseline-adopt, finalize exits 3 on reconciliation.conflict-block, marker survives.
Other cells driven: untracked x `jigc unmanage` after the copy-in -> finalize exit 0, marker 0·0·0·0. untracked x staged-elsewhere (first task's copy-in, unmanage, second task's copy-in records nothing, a line added after it) -> second task's finalize exit 0, that line worktree 0 · HEAD 0, in no blob. A committed doc carrying a UTF-8 BOM also fails the gate (copy-in adopts nothing), so it is in the non-conformant arm; tracked, the pin protects it.
```

## F3 · HIGH · In an eol-converting checkout the backstop conflict-blocks a doc nobody edited, at the task door and the record door, and no printed exit works — rc.24 landed

- where: crates/cli/src/task.rs:8302-8322 (git_blob_at answers two forms: checked-out, or the raw blob when the file is exactly that); crates/engine/src/state.rs:1034-1047 (copy-in is EOL-preserving, so a slot written into a CRLF doc leaves a mixed file); crates/cli/src/milestone.rs:1540-1551,1706-1709. design/reconciliation.md:50 ('a promote or a record write lands the bytes it commits — \n endings whatever the checkout converts to') and :54 (the record route's re-run 'cannot block again'). (8d9c3afb, eb18e5fe)
- driven: driven (this tree's debug build; rc.24 as control)
- class: Working-file forms of a doc git calls unmodified = 3 (checked-out · raw blob · mixed, which an in-place jigc edit of a checked-out file leaves); git_blob_at answers 2. Consumers of the seam = 4 by `command grep -n "git_blob_at" crates/cli/src` (task.rs:2626, milestone.rs:1708, milestone.rs:8104, cli.rs:1511). Driven at 2 (task finalize; record add-task). The join gate and the store sweep's lag arm read only. core.autocrlf=true and eol=crlf driven; smudge filters not.

```text
TASK DOOR. setup: rig committed-singletons; `git clone --config core.autocrlf=true <origin> clone` (same result with `* text eol=crlf` committed). In the clone: task 1 `jigc doc set-slot vision:vision#open-questions`, `jigc task finalize` -> exit 0; VISION.md now CRLF on 15 of 17 lines, git status clean. Task 2 set-slot on the same doc, `jigc task finalize second-pass` -> exit 3 reconciliation.conflict-block (pre-existing and identical on rc.24: advance_file_state, task.rs:7906-7909, records the committed blob's hash, so the doc reads DRIFTED at once). argv: `jigc unmanage VISION.md`; `jigc task finalize second-pass`. observed: THIS BUILD exit 3, same block; RC.24 exit 0, `promoted VISION.md`. Same state reached by deleting .jigc/state/file-state.json after a task's copy-in, nobody editing (this build): exit 3; the route's second exit as printed — `git checkout -- VISION.md`, then `git checkout HEAD -- VISION.md` — each exits 0 and rewrites nothing (still 15 of 17), each re-run exits 3; the unprinted `rm VISION.md && git checkout -- VISION.md` -> finalize exit 0.
RECORD DOOR. setup: rig fresh + `* text eol=crlf` committed; `jigc milestone create "Probe ms"`; `jigc milestone add-task probe-ms "Add alpha file"`; git clone. In the clone the record is CRLF on 16 of 16 lines; `jigc milestone add-task probe-ms "Add beta file"` -> exit 0, record now CRLF on 11 of 23, git status clean; delete .jigc/state/file-state.json. argv: `jigc milestone add-task probe-ms "Add delta file"`. observed: exit 1, `reconciliation.conflict-block ... differs from what HEAD holds`, route `git -C <clone> checkout HEAD -- docs/milestone-records/probe-ms.md`; run as printed from outside the repository -> exit 0, file unchanged; re-run -> exit 1, same block. rc.24 on that state -> exit 0.
MASKED BY FIXTURE: crates/cli/tests/copy_in_baseline.rs:1522-1554 builds 'jigc's own write' as a doc landed before the attribute existed (all \n); crates/cli/tests/record_door_baseline.rs:534-583 iterates Form::{Written, CheckedOut} and runs one door once. Neither holds the mixed form a clone's first in-place write produces.
```

## F4 · HIGH · A migration task's own source is exempt from the backstop unconditionally: a hand edit to it after the mint is replaced at exit 0 (unchanged from rc.24, declared only as a kept adoption)

- where: crates/engine/src/file_state.rs:581 (`!conflict.keys(path)`), :368-370; design/reconciliation.md:50 justifies the cell by the `jigc unmanage <source>` exit. The guard does not ask whether an unmanage happened, and a foreign non-conformant source is never baselined, so the path is UNKNOWN at every finalize. (eb18e5fe)
- driven: driven (this tree's debug build; rc.24 as control)
- class: instance, unbounded — one doctype (vision), one source shape (in-location squatter), one door. The off-canonical source (retired rather than replaced) and `jigc milestone finalize` not driven.

```text
REPRO (this build and rc.24, identical). setup: rig fresh; a foreign non-conformant VISION.md committed; `jigc migrate VISION.md --as vision`; `jigc doc author vision --from-file - --task <id>` (thesis, invariants, open-questions); author the commit doc; append `## Pricing` and a marker paragraph to VISION.md on disk. argv: `jigc task finalize <id> --approve`. observed: EXIT 0, `advisory · schema-conformance.unadopted-instance`, `finalized ... promoted VISION.md`. Marker worktree 0 · HEAD 0 · .jigc 0 · blobs 0. No block fires, so the keyed route's sentence ('an edit made to the file since is replaced without appearing in the --approve fidelity diff') is never printed.
```

## F5 · MEDIUM · An unreadable file-state.json is a new refusal on every first-touch `doc` write, with no route and a message naming the committed doc instead of the cache

- where: crates/engine/src/file_state.rs:773 (`load(..)?`) -> crates/cli/src/doc.rs:6884-6892 (context "could not read committed `{addr}`"); design/reconciliation.md:48 says the failure is re-run by 'the lock's own route'. (eb18e5fe)
- driven: driven (this tree's debug build; rc.24 as control)
- class: The door's fail-closed arms = 3, read at file_state.rs:762-765 (lock not taken · unreadable record · unreadable doc). The lock arm carries a route that works (driven: 10 s, exit 1, nothing staged, retry lands and adopts). The record arm carries none. The doc arm was not driven.

```text
REPRO. setup: rig committed-singletons; one task; `printf '{ not json' > .jigc/state/file-state.json`. argv: `jigc doc set-slot vision:vision#open-questions --from-file <f> --task <id>`. observed: this build EXIT 1, whole output: "could not read committed `vision:vision#open-questions`: key must be a string at line 1 column 3"; nothing staged. rc.24: exit 0, slot set. `jigc task validate` and `jigc validate` also exit 1 on that record but name the file-state record.
```

## F6 · MEDIUM · Four sentences the pass wrote are false on this build

- where: crates/cli/guides/MIGRATING.md:60 · design/reconciliation.md:222 · design/validation.md:366 · implementation/decisions-pending.md:79 (eb18e5fe, 8d9c3afb)
- driven: read only (each sentence held against a driven repro in F1/F2/F4)
- class: instance list — four sentences found by reading the two commits' doc hunks against the driven repros; not a sweep of every doc the pass touched.

```text
(1) MIGRATING.md:60 — 'A hand edit to a doc an open task has already written is never overwritten by that task — in a fresh clone as much as anywhere': falsified by F1 (both repros), F2 (three cells) and F4; the guide is embedded in the installed skill. (2) reconciliation.md:222 — 'a hand edit to a doc a task has staged blocks jigc task finalize and jigc milestone finalize whether or not the cache was ever populated. What is left outside it is stated there': F1 is stated nowhere. (3) validation.md:366 — the file-state.un-baselined route ('baselined on its next author or finalize') 'is true since the rc.24 fix pass': in F2's repro the advisory printed for the draft and the next author write baselined nothing. (4) decisions-pending.md:79 — the residual's reachable loss described as a two-task sequence, 'not driven': driven here from one task.
```

## F7 · MEDIUM · The acceptance suites iterate the axis but skip its crossing cells — the three cells where the losses and the false block live

- where: crates/cli/tests/copy_in_baseline.rs:175-200, 1137-1217, 1358, 1522-1554; crates/cli/tests/record_door_baseline.rs:534-583 (eb18e5fe, 8d9c3afb)
- driven: read only (the tests were read and run; the masked cells are driven in F1/F2/F3)
- class: three masked cells, each located by a driven loss or false block; the suites were not otherwise audited cell by cell.

```text
copy_in_baseline.rs:175-200 crosses Absent::Untracked only with a conformant doc; :1144-1217 puts the non-conformant cell on a tracked doc in a clone shape with a break (`date: 2026/13/01`) that makes the write itself refuse. The crossing cell — untracked x non-conformant with the write succeeding — is F2, and the cell's doc-comment (:1137-1142) says the door blocks 'instead of advising fix the file and then overwriting it', which is what F2's finalize printed and did. No cell lands a second jigc writer of the key between the hand edit and the holding task's finalize (F1); :1358 covers the copy-in writer only. The eol cells hold two of the three working-file forms (F3). All listed suites are green on this tree.
```

## F8 · LOW · A hand edit reverted after the copy-in is committed anyway and announced as 'external edit absorbed'

- where: crates/engine/src/file_state.rs:617-625 (L1: on-disk bytes equal the pin => the drift predates the task). With the copy-in now recording the hand-edited bytes, a revert to the pin reads as a pull.
- driven: driven (this tree's debug build)
- class: instance, unbounded

```text
REPRO. setup: clone of committed-singletons; one task; add a marker paragraph under `## Thesis` in VISION.md; `jigc doc set-slot vision:vision#open-questions ... --task <id>` (copy-in, baseline-adopt); `git checkout -- VISION.md`. argv: `jigc task finalize <id>`. observed: exit 0, `advisory · reconciliation.absorb — external edit absorbed: VISION.md`, and the reverted paragraph is at HEAD and back in the worktree. No byte lost; same outcome on rc.24 by adoption. The surface says the opposite of what the promote did.
```

## held

- The reported (R3, F7) instance: in a real clone, first write then a hand edit -> `jigc task finalize` exit 3 reconciliation.conflict-block, the edit left on disk. Also with the copy-in made by the rc.24 binary and the finalize by this build: no key -> backstop blocks; with no hand edit it lands.
- Declared order: a hand edit made before the first write lands merged, and the write's ack carries file-state.baseline-adopt on the text surface and in findings[] under --format json. The ack's key set is identical to rc.24's (chars, copied_in, findings, op, target; target: doctype, section, slug).
- Cell 8 on a tracked doc: A copies in, `jigc unmanage`, hand edit, B copies in -> B records nothing, both finalizes exit 3.
- Save-lock timeout at the write door: lock held by a live process for 14 s -> the write exits 1 after 10 s, nothing staged, no record written, message names the lock and the retry; the retry lands and adopts.
- Fresh clone, LF checkout: first task lands with no false block; untouched docs still adopt at the sweep; a committed doc with a BOM and no final newline lands, and its second task lands with the key removed after the copy-in.
- Store scope and the hook after a copy-in: a hand edit reads file-state.hash-matches at `jigc validate` (exit 0), and a plain `git commit` of that edit through the installed pre-commit hook exits 0.
- Guards beside the residual: a `git add`ed-uncommitted doc is refused by finalize.carried-staged, a doc committed after the mint by finalize.base-mismatch.
- Record door, LF checkout, real clone: a hand note -> exit 1 and nothing written; the route run as printed from outside the repository, then the re-run -> exit 0.
- Pulled record edits through a bare hub and two clones: with no key the pulled edit is adopted and the door lands; with a held key it conflict-blocks and routes at reverting the commit. That is the declared M55 P3 bound (the review's (R3, F4)), unchanged by this pass.
- Scope rule: neither commit touches a golden, snapshot, schema manifest or pack file (git show --stat); no new finding code.
- `jigc setup` run from a subdirectory installs at the git toplevel, so the `<pin>:<path>` lookup is always toplevel-relative.
- Suites run on this tree in a private target, all green: g_finalize copy_in_baseline + reconciliation_baseline_contrast + file_state_concurrency 23 passed; pre_guard_repair_route + l1_pull_absorption + migration 20 passed; g_milestone record_door_baseline + milestone_record_* 39 passed; g_methodology flow59_branch_and_pull 3 passed; g_flow flow49_acceptance 10 passed; jigc-engine file_state 45 passed.

## not_examined

- `jigc milestone finalize` (the join door) for F1, F2 and F4; a fan-out sibling's sub-task finalize as F1's mover.
- `jigc rename`, `jigc migrate-corpus` and `jigc relocate` as movers of the key in F1.
- Record doors other than `add-task` and a sub-task `task discard`: `add-from-spec`, `milestone discard`, the finalize flip.
- Smudge/clean filters, working-tree-encoding, Windows.
- The off-canonical migration source (retired rather than replaced) and the unreadable-doc arm of the copy-in door.
- rc.24 on the task-door variant of F3 where the cache file is deleted (rc.24 was driven on the unmanage variant and on the record door).
- The external-writer race windows the fixer declared (tier 2 by ruling).
- The full gate: only the suites listed under held were run.
- The doc hunks in command-output-contract.md, write-commands.md, worked-examples.md, team-ready-state.md and findings-channel.md were not read line by line; the engine unit tests were run, not reviewed.

# E2E

Verdict: FAIL — overall_pass: false. One HIGH inside the pass's own new code, three MEDIUM, five LOW. What holds: all six tier-1 repros and (R6, K-1) now refuse or hold with the bytes intact, and every route those refusals print was run AS PRINTED to a landed end state; every class sibling the pass added refuses (B1–B5); every happy path is green on the standing states and in fresh clones (incl. a CRLF clone); the fan-out is byte-identical across divergent launch/completion orders in both squash modes; the co-author trailer is on every jigc commit exactly once; 36 of 37 compared pinned surfaces are byte-identical to rc.24 (the 37th is --help). What fails: the new linked-worktree guard (`finalize.linked-worktree-doc`, e3a6ba58) fires in every checkout whose `.git` is a file — including a worktree of a bare repository, a `--separate-git-dir` repository and a git submodule, where no main checkout exists — and its printed route dead-ends there, so no door can land a managed doc in those layouts, where rc.24 landed one at exit 0 (F1). FAN-OUT CAVEAT: the fan-out rows are an N-process simulation — I rendered the adapter's `Spawn:` lines through `jigc milestone execute` and ran each rendered command verbatim as its own concurrent process; the GENUINE concurrent assistant Task-tool spawn is the orchestrator's main-session half and I did NOT run it. REPORT FILE: the requested report.md was NOT written — the harness refused the Write ("subagents return findings as text, not report files") and I did not route around it; this structured result carries the full report. Scripts and raw outputs are in the e2e audit directory the task named (a*.sh b*.sh c*.sh d*.sh lib.sh, c5-witness-*.txt). Nothing was written, committed or pushed in the jigc repository (git status empty, HEAD 5f5b273a at the end).

Binary: Release build of branch fix/rc24-tier1 @ 5f5b273a: `CARGO_TARGET_DIR=$(mktemp -d <e2e>/target.XXXXXX) cargo build --release -p jigc` (exit 0), `<e2e>/target.*/release/jigc --version` → `jigc 1.0.0-rc.24` ("new"). Comparison binary where a row says so: the installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` ("rc.24"). Every rig: `dev/jigc-rig <state> --binary <new>` under SCRATCH=<e2e>/rigs, two-step eval, stdout only, $REPO guarded against the jigc checkout. git 2.54.0 (Apple Git-157), macOS/APFS. CLAUDECODE set unless a row says otherwise (value never printed).

## Scenarios

- A1 (R3,F7) — PASS — rig committed-singletons → `git clone` → `jigc start --workflow single-task "sharpen the open questions"` → `jigc doc set-slot vision:vision#open-questions --from-file tp.txt --task <t>` (ack now carries `file-state.baseline-adopt`; file-state.json present after the first write) → commit doc filled → one hand line added to VISION.md uncommitted. `task validate` exit 3, `task finalize --dry-run` exit 3, `task finalize --format json` exit 3: one JSON doc on stdout, `reconciliation.conflict-block` key target `VISION.md`; HEAD unmoved; marker worktree=1 before and after. Routes run: `git checkout -- VISION.md` + finalize → exit 0, task prose at HEAD; `jigc task discard <t> --force` → exit 0, hand line intact. Merge order (hand line under Thesis BEFORE the first write) → exit 0, both sides at HEAD. Also blocks (exit 3, bytes intact): file-state.json moved away after copy-in (edit after AND edit before), `jigc unmanage VISION.md` before the write and after the block, hand edit `git add`-ed, and the `milestone finalize` door (create + add-task + set-slot --task <sub> + hand edit).
- A2 (R6,D-1) — PASS — rig fresh; landed `inconsistency:stage-2`, untracked hand `stage-3.md`; milestone with alpha/bravo/charlie each `doc create inconsistency --title Stage` in its worktree. `milestone join` exit 0 (suffixes -2, -3); `milestone finalize --format json` exit 3, one JSON doc on stdout, two `finalize.promote-clobber` (targets the two homes); HEAD unmoved, both victims' shasums unchanged. Route as printed: `jigc doc rename inconsistency:stage --to "Bravo stage" --task bravo-reports-the-stage`, same for charlie, `jigc milestone finalize stage-reports` → exit 0, 4 files, victims intact. Same with `finalize.fan-out.squash=false`. Occupant shapes at an unsuffixed home (untracked conformant, foreign, staged, directory, dangling link, live link to a file outside the repo, link to /dev/null): all exit 3 `finalize.promote-clobber`, occupant/target untouched.
- A3 (R6,D-7) — PASS — link at `docs/inconsistencies/dangling-home.md` (dangling in-repo, dangling out-of-repo, /dev/null, live). Before the create: `jigc doc create inconsistency --title "Dangling home" --task <t> --format json` exit 1, stdout empty, one JSON doc on stderr, `create.already-exists`. After the create: `task finalize --dry-run` exit 3 and `task finalize --format json` exit 3 `finalize.promote-clobber`; HEAD unmoved, link still a link, nothing created through it. Also `adr` under `record-decision` (dangling link → exit 3; live link → copied in through the link, finalize exit 3, target unchanged) and a committed `VISION.md` link (exit 3, target unchanged). Routes run: `jigc doc rename … --to "Free home" --task <t>` + finalize → exit 0, `100644`; move the link out + finalize → exit 0; `doc create … --slug dangling-home-b` → lands; regular copy in the link's place + finalize → exit 0, `100644 VISION.md`.
- A4 (R6,K-1) — PASS — Sequential: home absent → `existed:false`; home present → exit 1 `create.already-exists`, nothing staged but the commit doc; file landing after the create → finalize exit 3 `finalize.promote-clobber`, victim 1→1. Window: python toggler `os.rename(park→home); os.rename(home→park)` (98 898 round-trips) with a fresh task per call: `doc create` ×60 → 50 refused / 10 fresh / 0 copied in; `doc author --format json` ×60 → 41 refused / 19 fresh / 0 copied in (no `edited-from-base` provenance, no victim marker in any staged copy). The timed one-shot plant was not driven.
- A5 (R1,F1) — PASS — Repro A: unborn repo, untracked `.jigc/AGENT.md` + `.jigc/config/packs.yaml` → `jigc setup --format json` exit 1, one JSON doc on stderr, `setup.dirty-install-path` naming both; marks still in the worktree, 0 commits. Each of the four replaced members alone (AGENT.md, version, config/.gitkeep, packs.yaml) → exit 1. Orphan-branch trigger (`git checkout --orphan` + `git rm -r --cached .` + AGENT.md) → exit 1, mark intact. On-ramp control (untracked CLAUDE.md, .claude/settings.json, root .gitignore, .jigc/.gitignore on unborn HEAD) → exit 0, all four marks in HEAD. Repro B: failed first run (`setup.install-commit`), then arm 1 (index kept + edit) exit 1; arm 2b (`git reset` + edits) exit 1, marks intact; arm 3 (plain re-run) exit 0; unstaged own bytes, no edit → exit 0. Routes run: move out → exit 0; commit the named paths → next run names untracked CLAUDE.md, commit it → exit 0; `--force` → exit 0 with advisory `setup.forced-install-path` naming both paths.
- A6 (R9,F5) — PASS — rig fresh; `notes.txt` planted under .jigc/{logs,state,index,tasks,milestones,logs/sub/deep} → `jigc uninstall --format json` exit 1, one JSON doc on stderr, `uninstall.foreign-bytes` naming all six; all six still on disk. Route: move them out, re-run → exit 0, `.jigc/` gone, second run "nothing to remove". Each prefix alone → exit 1 (worktrees → `uninstall.dirty-worktree`). `--force` → exit 0 and names the file it destroys. Log: `jigc config set invocation-log true`, committed, 5 records → `uninstall` exit 1 `uninstall.untracked-workbench-file` naming `.jigc/logs/invocations.jsonl`; route (move the log out, re-run) → exit 0, `.jigc/` absent afterwards (not re-created), moved-out log holds its records.
- A7 (L-22) — PASS — From the record's block: foreign worktree y (detached, a commit + staged-only blob + unstaged edit), z (branch `side`), `live`; `mv` y and z away (git lists both `prunable`). Doors: `milestone provision` (first and re-run) exit 0, `milestone finalize` exit 0 (3 files), `milestone discard` exit 0, `jigc uninstall` exit 0, `milestone finalize` never-provisioned exit 3 `milestone.zero-contribution`, `milestone discard` never-provisioned exit 0. After every one: `.git/worktrees/{y,z}` present, the foreign commit 0 hits in `git fsck --unreachable`, `git worktree list` still names y and z, the live worktree byte-identical; with the directories moved back `git status` exits 0 in both and y's HEAD is the foreign commit.
- B1 gitignored install path — PASS — `.jigc/AGENT.md` gitignored + hand-written, on a born HEAD, an unborn HEAD, and with the whole `.jigc/` ignored (AGENT.md + version): `jigc setup --format json` exit 1 `setup.dirty-install-path` (message says git status reports nothing there), bytes intact. Route (move the file out, re-run) → exit 0, install commit made, re-run exit 0 clean. jigc's own generated content at an ignored, untracked path → re-run exit 0. A hand line appended to that ignored installed file → exit 1 again.
- B2 symlinks at install paths and .jigc/version — PASS — Link at each of `.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml` × (born+untracked, committed, unborn): 12 of 12 `jigc setup --format json` exit 1 under `setup.write-bootstrap` / `setup.version-stamp` / `setup.init-project-layer` / `setup.compose-marker`; target sha unchanged, path still a link, no commit. `--force` → still exit 1. `.jigc` → link and `.jigc/config` → link refuse too. Routes: `git rm` the committed link + commit, re-run → exit 0 `100644`; `rm` the untracked link, re-run → exit 0; targets untouched. At `jigc task finalize` with a non-regular or non-stamp `.jigc/version` (committed link, uncommitted link, out-of-repo link, dangling link, directory, hand text): task lands exit 0, advisory `setup.version-stamp`, the entry and its target untouched and out of the commit; a genuine stale stamp `jigc-version: 1.0.0-rc.20` is still rewritten and committed. Advisory routes run (remove link / move dir / move file, then `jigc setup`) → exit 0, stamp restored.
- B3 milestone record in a clone — PASS — rig fresh, milestone + one task, `git clone`, one hand line appended to `docs/milestone-records/record-probe.md`: `jigc milestone add-task` exit 1, `milestone finalize` exit 1, `milestone discard` exit 1, and with the line `git add`-ed exit 1 — `reconciliation.conflict-block` ("differs from what HEAD holds"), hand line worktree=1, HEAD unmoved. Control without the hand line: exit 0. `milestone provision` exit 0 and leaves the line. Route as printed (`git -C <clone> checkout HEAD -- <record>`, re-run) → add-task and finalize land. JSON: one doc on stderr, `{error}` shape (see F7).
- B4 own sub-task registrations — PASS — Milestone, two provisioned sub-tasks. Stale+staged (alpha.txt `git add`-ed, directory moved away): `provision` exit 1 `milestone.leftover-holds-work`, `discard` exit 1 `milestone.dirty-worktree`, `finalize` exit 3 `milestone.unlanded-work`, `uninstall` exit 1 `uninstall.dirty-worktree`; registration present, nothing dangling. Stale+commit: same four refuse. `.git` link removed: finalize exit 3, provision exit 1. Live+commit: finalize exit 3, discard exit 1, uninstall exit 1 (provision exit 0, reuses it). Stale+empty: provision exit 0, finalize exit 0. Routes run as printed: restore recipe (`mkdir -p … && printf 'gitdir: %s\n' … > …/.git && git -C … restore .`) → finalize lands alpha.txt; re-link `printf` → lands; `git -C … branch kept/add-alpha-file <sha>` → finalize lands the rest, branch holds the commit; `git -C … reset --soft <base>` → finalize lands alpha.txt (also with an extra staged file); `provision --force` and `discard --force` name the dropped staged path and blob. Sub-task settled by `jigc task discard` with staged code: finalize exit 3 `milestone.unlanded-work`; printed `git -C … stash` → finalize exit 0, stash survives.
- B5 jigc rename over a link — PASS — Two landed ADRs (`cache`, `cache-two` superseding it). Link at the doc's own home (committed; uncommitted), at the referrer's home (committed), at the new home (dangling; live): `jigc rename adr:cache --to Caching --format json` exit 1 in all five (`finalize.promote-clobber` for own/referrer, `write.already-present` for the new home); HEAD unmoved, targets unchanged, `git status` unchanged. Control exit 0 with the referrer repointed. Routes run: regular copy in the link's place + commit, re-run → exit 0 (both own-home and referrer); `--slug caching-b` → exit 0; move the link out, re-run → exit 0, link target untouched. In-task `jigc doc rename … --task` onto a linked home → exit 1 `write.already-present` (findings envelope). `jigc milestone create` over a link at the record's home → exit 1 `milestone.record-exists`, target untouched.
- B6 user-made linked worktree — FAIL — With a main checkout present everything holds: `git worktree add -b feature <lw>`; in it `doc set-slot vision…`, `doc create adr`, `doc create changelog` → exit 1 `finalize.linked-worktree-doc` (stderr, one JSON doc, key target = the doc's home); commit-doc write exit 0; `start --workflow report-inconsistency` and `migrate notes.md --as vision` exit 1 before minting (key target = the checkout path); code-only task → `task finalize` exit 0 on branch `feature`, main checkout untouched, trailer once. Two-commit route: doc task in the main checkout lands, code task in the linked worktree lands, branch merges. Backstop: doc staged from the main cwd into the linked task, hand line in the linked VISION.md → `task validate`, `--dry-run`, `finalize` exit 3 from the linked worktree; its route (finalize from the main checkout) lands the doc, staged code and hand line stay. Dangling-anchor exit on rig `vendored` (`git mv src/pad.ts src/padding.ts` → `doc-code.symbol-exists`): route run as printed (`git -C <lw> stash`, merge, mint, `git stash pop --index`, set-field, finalize, discard) → one commit with doc + rename, store validates clean. FAILS in the layouts with no main checkout (worktree of a bare repo, --separate-git-dir, submodule): the same refusal fires and its route dead-ends — finding F1.
- C1 happy paths: standing states and fresh clones — PASS — `<e2e>/c1.sh <state> <rig|clone>` on fresh/rig, committed-singletons/rig, chatty-hooks/rig and fresh clones of fresh, committed-singletons, vendored, migrated: `jigc start` (text + one JSON doc) exit 0; single-task with staged code + `doc create adr` + changelog entry → `task validate` 0, `task finalize --format json` 0 (manifest: promoted CHANGELOG.md, promoted the ADR, added the code); `jigc task amend` → finalize (tree hash unchanged, subject replaced); doc-only `report-inconsistency` → finalize; `jigc rename adr:rate-limiter --to "Token bucket limiter"`; a second single-task copying the committed changelog in + a superseding ADR + code → finalize, both changelog entries at HEAD; `jigc validate` 0; `jigc setup` re-run with HEAD, `git status --porcelain --ignored` and every worktree byte unchanged. 37–38 PASS lines and 0 FAIL per state.
- C2 fan-out N-process sim, order invariance — PASS — `<e2e>/c5.sh <squash> <id|reverse> <rig|clone> <tag>` with fixed GIT_AUTHOR_DATE/GIT_COMMITTER_DATE set before the rig is built: `milestone create` + three `add-task` → `provision` → `milestone execute` renders three lines `Spawn: `cd <repo>/.jigc/worktrees/<sub> && jigc workflow sub-task --task <sub>``; each rendered command run verbatim with `bash -c` as its own background process (the new binary first on PATH): exit 0, 72 composed lines (120 with squash off), stderr empty. Each process stages code in its worktree; alpha and bravo each `doc create adr --title "Shared decision"` from their worktree (not refused — jigc's own worktrees are exempt). `milestone join` exit 0 (`adr:shared-decision` from alpha, `-2` from bravo). `milestone finalize --format json` exit 0, one JSON doc. Launch order id vs reverse with a 2 s delay on the opposite end: witness (tree hash + log subjects/bodies + every tracked file) shasum eb8efdfb… = eb8efdfb… (squash) and ab0c3777… = ab0c3777… (squash off), `cmp` clean, HEAD commit ids equal. Squash off lands three per-sub-task commits + the boundary. Fresh-clone variant: PASS. The genuine Task-tool spawn was NOT run.
- C3 concurrent first copy-ins in a clone — PASS — `<e2e>/c6.sh 8`: eight committed ADRs, fresh clone (no file-state baseline), milestone with eight sub-tasks, eight concurrent processes each doing two `doc set-slot adr:decision-N#… --task edit-decision-N` from its worktree + staging a file: all 16 writes exit 0 (no save-lock failure); `milestone finalize` exit 0, 17 files, all eight edits at HEAD.
- C4 migrate a foreign changelog — PASS — rig fresh, a hand keep-a-changelog `CHANGELOG.md` committed; `jigc migrate CHANGELOG.md --as changelog` exit 0; `jigc doc author changelog --from-file payload.yaml --task <t>` exit 0; `task finalize` exit 4 (review hold, fidelity diff); `task finalize --approve` exit 0 `promoted CHANGELOG.md`, canonical shape with schema-version 2, trailer once. Same in a fresh clone. (The hand-edit-after-mint cell is finding F3.)
- C5 setup / re-run / uninstall — PASS — `<e2e>/c8.sh off|on` on rig bare: `jigc setup` exit 0, install commit `chore(jigc): install jigc workspace config` with the trailer exactly once, clean tree; re-run exit 0 with HEAD, every worktree byte and the pre-commit hook unchanged; `setup --format json` re-run one JSON doc, findings []. Knob off: `uninstall` exit 0, `.jigc/` gone, second run "nothing to remove", hook absent; `git checkout -- .` + `setup` round-trips. Knob on (`config set invocation-log true`, committed): `uninstall` exit 1 `uninstall.untracked-workbench-file`; log moved out → exit 0, `.jigc/` gone and not re-created.
- C6 co-author trailer — PASS — Per-commit census in every C1/C2/C5 run: `git log -1 --format=%B <c> | grep -c '^Co-Authored-By: Claude <noreply@anthropic.com>$'` = 1 and case-insensitive `co-authored-by` count = 1 on every commit jigc made — task finalize, amend, `jigc rename`, milestone record commits, per-sub-task commits, the milestone boundary, the migration commit, the install commit. Control with the variable unset: 0 trailer lines.
- C7 pinned surfaces vs rc.24 — PASS — `<e2e>/c9.sh` on one committed-singletons rig, each surface produced by both binaries and `cmp`-ed (stdout, stderr, exit): `doc schema <type> --format json` for 16 doctypes, `describe`, `describe --workflows`, `start`, `start --format json`, `doc list --format json`, `doc show vision --format json`, `validate --format json`, and the composed output of 13 workflows (`start --workflow <w>`): 36 of 37 byte-identical. The one difference is `jigc --help` (the setup and uninstall command descriptions, expected).
- D1 CRLF checkout, first task in a clone — PASS — `git clone -c core.autocrlf=true` (VISION.md checks out with CRLF): the whole C1 path is green (36 of 37; the one FAIL line is the first `setup` rewriting CRLF checkouts of its own files — F9). `<e2e>/d2.sh crlf-keylost`: file-state.json moved away after the copy-in, no hand edit → finalize exit 0 (no false block). `crlf-hand` and `crlf-keylost-hand` → exit 3 `reconciliation.conflict-block`, hand line intact.
- D2 non-canonical committed bytes — PASS — VISION.md committed with doubled blank lines, trailing spaces and no final newline; fresh clone; first task set-slot → finalize exit 0; with file-state.json moved away after the copy-in → still exit 0.
- D3 every copy-in leaf + later hand edit, in a clone — PASS — `<e2e>/d3.sh <leaf>`: `doc set-slot`, `set-field`, `create` (copy-in of a committed adr), `author`, `add-item`, `remove-item` as the task's first touch, then a hand line → `task finalize` exit 3 `reconciliation.conflict-block`, hand line on disk, in all six. A refused in-task `doc rename` of a committed doc (exit 1 `write.identity-change`) still leaves a staged copy and a later hand edit blocks. An UNTRACKED conformant doc copied in by `doc create` + a later hand edit → exit 3. (Same with the cache deleted is F5.)
- D4 two open tasks, pre-staged state, wrong addresses — PASS — Two tasks on VISION.md in a clone: first finalize exit 0, second exit 3 `finalize.base-mismatch` (first task's prose at HEAD). A hand edit between their copy-ins → both exit 3 conflict-block, hand line intact. Pre-staged code → exit 3 `finalize.carried-staged`, `--carry-staged` lands; the managed doc itself pre-staged with a hand edit → exit 3 `finalize.carried-staged`, edit intact. `set-slot` at `vision:vision#nope` / `vision:nope#thesis` / `nope:vision#thesis` / an undeclared field / `VISION.md#thesis` → exit 1, stdout empty, one JSON doc on stderr (`write.unknown-section`, `store.fixed-identity`, `store.unknown-type`, `write.unknown-field`, `{error}`).
- D5 printed routes under a path with a space — PASS — `<e2e>/d4.sh` in a clone at `<rig>/my project.XXXX/repo`: the restore recipe, the `branch kept/…` command, the record door's `git -C '<path>' checkout HEAD -- …`, the linked-worktree `cd '<path>'` and the setup route's `git -C '<path>' rm --cached -- <path>` are printed shell-quoted and each ran as printed with exit 0 (no stray directory created).
- D6 index flags at an install path — PASS — `git update-index --assume-unchanged` / `--skip-worktree` on `.jigc/AGENT.md` and on `CLAUDE.md`, then a hand line (git status empty): `jigc setup --format json` exit 1 `setup.dirty-install-path`, message names the hidden change, route prints both `update-index --no-…` commands; hand line intact.
- D7 uninstall over symlinks inside .jigc — PASS — `.jigc/logs` → link to an outside directory: `uninstall` exit 1 `uninstall.untracked-workbench-file`; `.jigc/state/link.txt` → link to an outside file: exit 1 `uninstall.foreign-bytes`; the outside bytes untouched.
- D8 merged-into install members through a committed link — FAIL — See F2: `jigc setup` exit 0 writes through a committed `CLAUDE.md` / `.claude/settings.json` link into a file outside the repository, and creates one through a dangling link; identical on rc.24.
- D9 migration source edited after the mint — FAIL — See F3: a hand line added to a same-path migration source after `jigc migrate` is replaced at `task finalize --approve` exit 0, never shown in the review diff; identical on rc.24.
- D10 the declared no-blob cell — FAIL — See F5: untracked conformant doc copied in + cache deleted + hand edit → `task finalize` exit 0, hand line nowhere. Declared in design/reconciliation.md; reported as LOW.
- D11 --format json on every new refusal — PASS — Every new refusal printed exactly one JSON document — on stdout for an adjudication (exit 3 / 0), on stderr with stdout empty for a reject (exit 1). New codes: `finalize.linked-worktree-doc` (key target = the doc's home path at a write/backstop, the checkout path at a mint) and `milestone.unlanded-work` (key target `.jigc/worktrees/<sub>`). Reused codes with new arms: `reconciliation.conflict-block` (target the doc path), `finalize.promote-clobber` (the home path), `create.already-exists` (`<type>:<slug>`), `write.already-present` (`<type>:<slug>`), `setup.dirty-install-path` / `setup.write-bootstrap` / `setup.version-stamp` / `setup.init-project-layer` / `setup.compose-marker` / `setup.forced-install-path` (target null), `uninstall.foreign-bytes` / `uninstall.untracked-workbench-file` / `uninstall.dirty-worktree` (target null). Five doors print the `{error}` envelope with the finding as text only — F7.

## F1 · HIGH · The linked-worktree guard fires where no main checkout exists, and its printed route dead-ends (bare repo + worktree, --separate-git-dir, submodule)

- where: crates/cli/src/task.rs `linked_worktree_doc_finding` (:399) and `linked_worktree_mint_finding` (:481), raised from doc.rs:6791, migrate.rs:400, start.rs:2080, task.rs:2935 — commit e3a6ba58
- driven: driven — new binary (release, this branch) and rc.24 (installed registry build)
- class: Raise sites: 4, by `grep -n "linked_worktree_doc_finding\|linked_worktree_mint_finding" crates/cli/src/*.rs` (doc.rs:6791 the write-leaf funnel, migrate.rs:400, start.rs:2080, task.rs:2935 the backstop read at :3010 and :3309) — all share the one 'main checkout' path. Layouts: 4 driven; the set of layouts in which `.git` is a file (or jigc_home is not a work tree) was NOT enumerated — the layout axis is an instance list, unbounded.

```text
setup     rig bare ; W=<rig>/layout
  (a)     git clone --bare <repo> W/.bare ; printf 'gitdir: ./.bare\n' > W/.git ; git -C W worktree add W/main main ; cd W/main
  (b)     git clone --bare <repo> W/proj.git ; git -C W/proj.git worktree add W/main main ; cd W/main
  (c)     git init --separate-git-dir W/gitdir W/work ; one commit ; cd W/work
  (d)     git init W/super ; (one commit) ; git -c protocol.file.allow=always submodule add <repo> sub ; cd W/super/sub
argv      jigc setup ; jigc start --workflow record-decision "record the cache decision" ;
          jigc doc create adr --title Cache --task record-the-cache-decision ; (fill 3 slots + commit doc) ; jigc task finalize record-the-cache-decision
rc.24     (a) setup 0 · doc create 0 · finalize 0 "finalized … promoted docs/decisions/cache.md" (committed in the worktree)
          (b) setup 1 (setup.install-hook) · doc create 0 · finalize 0, committed
          (c) setup 1 (setup.install-hook) · doc create 0 · finalize 0, committed
          (d) setup 0 · doc create 0 · finalize 0, committed
new       all four: `jigc start` prints "checkout: the linked worktree at `…` on branch `main` — a task here commits here, and code only"
          doc create → exit 1  blocking · finalize.linked-worktree-doc — … jigc's doc store has one home, the main checkout at `<W>` [(d): `<W>/super/.git/modules`]
            route: write it from the main checkout: `cd <W>`, then `jigc start "<intent>"` starts a task there, whose finalize commits the doc on that checkout's branch …
          task finalize → exit 3 finalize.empty-commit ; HEAD unmoved
route run as printed
          (a) cd <W> ; jigc start --workflow record-decision "record another decision" → 0 ; jigc doc create adr --title Cache --task … → 0 ; fill ;
              jigc task finalize … → exit 1  "`git status --porcelain` failed: fatal: this operation must be run in a work tree"   (git -C <W> rev-parse --is-bare-repository → true)
          (d) cd <W>/super/.git/modules ; jigc start … → exit 1 "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
          (b), (c): <W> is not a git repository
Framing: these layouts were already half-working on rc.24 — the install lands outside the checkout (in (d) under <super>/.git/modules/) and in (d) rc.24's `jigc doc show adr:cache` after the landing exits 1 `store.not-found`. So the refusal may be the right answer; the route is what is false, and no door can land a managed doc in these layouts any more. Subdirectory of a normal checkout: unaffected (control, both binaries exit 0).
```

## F2 · MEDIUM · `jigc setup` still writes through a committed link at a merged-into install member — into, or creating, a file outside the repository — at exit 0

- where: crates/cli/src/setup.rs, the `Preserves` members of the install table (~:1937–2011); 104a7d4b converts the `Replaces` members only
- driven: driven — new binary and rc.24 (pre-existing, not introduced by the pass)
- class: `Preserves` members: 5, by reading the install table in setup.rs (`member(line_file, …, Preserves)`, `allowlist_file`, `.jigc/.gitignore`, root `.gitignore`, an in-worktree hook). Driven: CLAUDE.md and .claude/settings.json write through (2); a root `.gitignore` link showed no write in my cell; `.jigc/.gitignore` and the hook not driven.

```text
setup     rig bare ; printf '# shared USERMARK\n' > <rig>/shared-claude.md ; ln -s <rig>/shared-claude.md CLAUDE.md ; git add CLAUDE.md ; git commit -m "link claude"
argv      jigc setup
observed  exit 0 ; ack "bootstrap reference → CLAUDE.md" ; CLAUDE.md still a link ; git status empty ; `git show --stat HEAD` does not list CLAUDE.md ;
          <rig>/shared-claude.md 1 line → 5 lines
same      .claude/settings.json → link to <rig>/shared-settings.json (3 bytes) : exit 0, link kept, target now 952 bytes, status empty
same      CLAUDE.md → DANGLING link to <rig>/nowhere-claude.md : exit 0, link kept, the file now exists outside the repository
in-repo   CLAUDE.md → notes/claude.md (tracked): exit 0, ` M notes/claude.md` left uncommitted, not in the install commit
rc.24     identical in all cells (`<e2e>/d7.sh <cell> rc24|new`)
No byte is lost (the writer merges). A file outside the repository is modified or created with a clean status, the ack names an install the commit does not carry, and a clone gets a link with no jigc reference behind it — (R6, D-7)'s out-of-repository member at the setup door, and the symlink ruling's own reason ("the rule the same door already applies to .jigc/.gitignore") reaches it.
```

## F3 · MEDIUM · A hand edit to a migration's same-path source after `jigc migrate` is replaced at `task finalize --approve`, exit 0, never shown in the review

- where: `jigc task finalize <migration> --approve`; the base-pin backstop skips the migration task's own source (eb18e5fe; design/reconciliation.md → "The `unmanage` exit stays an exit")
- driven: driven — new binary and rc.24 (pre-existing; inside the cell the (R3, F7) fix carves out)
- class: instance, unbounded — driven on the changelog same-path migration only (rig; the plain path also in a fresh clone). The other `migrate-*` workflows and a non-same-path source were not driven for this cell.

```text
setup     rig fresh ; a hand keep-a-changelog CHANGELOG.md, committed with plain git
          jigc migrate CHANGELOG.md --as changelog                                   → exit 0, task minted
          jigc doc author changelog --from-file payload.yaml --task <t>              → exit 0 ; commit doc filled
          printf '\n- HAND-AFTER-AUTHOR line added to the foreign file.\n' >> CHANGELOG.md        (uncommitted)
argv      jigc task finalize <t> ; jigc task finalize <t> --approve
observed  hold: exit 4 ; the diff is headed "--- foreign source (staged seam)" ; `grep -c HAND-AFTER-AUTHOR` over the whole hold output → 0
          --approve: exit 0 "finalized … docs(changelog): adopt the changelog / promoted CHANGELOG.md" ; no finding names the path
after     HAND-AFTER-AUTHOR: worktree 0 ; `git log --all --oneline -S…` 0
rc.24     identical (`JIGC_OVERRIDE=<rc.24> bash <e2e>/c2b.sh hand-edit`)
The design declares this cost on one path only — after a `reconciliation.conflict-block`, on the route that offers `jigc unmanage <source>` and states the cost inline. Here the source was never baselined, so nothing blocks, no route is shown, and the reviewer approves a diff that cannot contain the line.
```

## F4 · MEDIUM · DECISIONS.md is false at HEAD: it says five queued members are not built and admits one new finding code; HEAD builds all five and carries a second new code with no stated reason

- where: DECISIONS.md, the two top 2026-10-04 entries ("…five further class members queued… None is built at this entry"; "One new finding code, `finalize.linked-worktree-doc`"); commits 104a7d4b, ef9456b5, 4fa1c0f5, 8d9c3afb, e842342e, 5f5b273a
- driven: read only (greps and git log); the behaviour the entries describe was driven on the new binary
- class: The three orchestrator-owned logs (DECISIONS.md, implementation/decisions-pending.md, implementation/project-history.md): counts derived by the greps and git logs quoted. Other design docs were not audited here (the e2e half).

```text
`git log --oneline origin/main..HEAD`: the last record commit is 614d9c99; six fix commits follow it and build the five queued members plus the gitignored-path ruling the log calls "Not built at this entry" — each driven green above (B1, B2, B3, B4, B5, A6).
`git log --oneline -3 -- DECISIONS.md implementation/project-history.md implementation/decisions-pending.md` → 614d9c99, fcd9af18, bca470c1 (nothing after the round-three fixes).
`git grep -l -F milestone.unlanded-work origin/main -- crates design` → 0 files ; at HEAD → 6 files (milestone.rs, render.rs, two test suites, design/finalize.md, design/team-ready-state.md).
`grep -c -F milestone.unlanded-work DECISIONS.md` → 0.
The code's other homes match its sibling `milestone.zero-contribution` (design/finalize.md, tests). What is missing is rule 2's stated reason for a new code and rule 4's doc-with-the-rule for the logs.
```

## F5 · LOW · The declared no-blob cell is still an exit-0 loss at `task finalize` (untracked doc copied in + cache deleted + hand edit)

- where: crates/engine/src/file_state.rs, the UNKNOWN + TOUCHED arm with no blob at the base pin; declared in design/reconciliation.md → "Three cells keep the adoption"
- driven: driven — new binary
- class: The three adoption-keeping cells the design names (no blob at the pin · the migration task's own source · any git failure): no-blob driven here, the migration source is F3, a git failure not driven.

```text
setup     rig fresh, one adr landed ; a copy of it at docs/decisions/squat.md, retitled, never `git add`ed
          jigc start --workflow single-task "touch the docs"
          jigc doc create adr --title Squat --task touch-the-docs      → exit 0 "adr:squat (already existed — copied in for update)"
          jigc doc set-slot adr:squat#decision --from-file - --task touch-the-docs ; mv .jigc/state/file-state.json <elsewhere>
          one line (HANDMARK) appended under the file's consequences slot by hand ; commit doc filled
argv      jigc task finalize touch-the-docs
observed  exit 0 "finalized … promoted docs/decisions/squat.md" ; HANDMARK: worktree 0, HEAD 0, `grep -rl` over the repo outside .git → nothing
control   the same without the cache removal → exit 3 reconciliation.conflict-block, HANDMARK intact
Three independent acts are needed and the bound is declared; reported because it is the one exit-0 loss still reachable inside the class the pass closes.
```

## F6 · LOW · The dangling-anchor exit's mint step collides when the linked task's intent is reused

- where: the `finalize.linked-worktree-doc` write-leaf route (task.rs `linked_worktree_doc_finding`)
- driven: driven — new binary
- class: instance (one route text)

```text
rig vendored ; git worktree add -b feature <lw> ; in <lw>: jigc start --workflow single-task "rename the pad module" ; git mv src/pad.ts src/padding.ts ; task finalize → exit 3 doc-code.symbol-exists ; jigc doc set-field arch-doc:padding-layer#components/pad/implemented-by --value src/padding.ts#pad --task … → exit 1 finalize.linked-worktree-doc with the stash route (discard the linked task LAST).
As printed with the same intent: `git -C <lw> stash` ; cd <repo> ; `jigc start --workflow single-task "rename the pad module"` → exit 1 `task.serial-collision — task `rename-the-pad-module` is already active` (an intent with two more words slugs to the same id).
It converges: the collision route's `jigc start --task rename-the-pad-module` resumes the task from the main checkout, then `git stash pop --index`, the set-field and `task finalize` land doc + rename in one commit (driven). With a distinct intent the route lands exactly as printed (driven, store validates clean). Also: the routes' `jigc start "<intent>"` composes the router selection on a multi-workflow store rather than minting.
```

## F7 · LOW · New refusals at five doors print the `{error}` envelope with the finding as text only — no `findings`, no `key`, no `route` field

- where: `jigc rename` (three new link arms, 5f5b273a), the milestone-record door (8d9c3afb), `milestone provision` / `milestone discard` (ebfc79fb), `milestone create` (9465f9b6)
- driven: driven — new binary; the rename shape also on rc.24
- class: instance, unbounded — five doors observed through the binary; the doors raising a rendered finding through the operational-error funnel were not enumerated from the code.

```text
jigc rename adr:cache --to Caching --format json   (link at the doc's home) → exit 1, stdout empty, stderr one document, keys ['error'] : "blocking · finalize.promote-clobber — … \n  at: …\n  route: …"
jigc milestone add-task record-probe "second thing" --format json   (hand line, clone) → exit 1, stderr keys ['error'] : "blocking · reconciliation.conflict-block — …"
jigc milestone provision / discard / create --format json → the same shape for milestone.leftover-holds-work / milestone.dirty-worktree / milestone.record-exists
rc.24, `jigc rename` onto an occupied id → the same `{error}` shape (`write.already-present`), so this is each door's existing shape and no pinned key moved; a driver cannot address the pass's new refusals at these doors by (code, target).
```

## F8 · LOW · A FIFO at a doc home hangs `jigc milestone finalize`

- where: the store read seam ahead of the planners (the fixer names `engine::target_surface::enumerate_target_surface`); declared out of the (R6, D-7) fix
- driven: driven — new binary; not compared with rc.24
- class: instance, unbounded (one door driven; the fixer reports the same seam at `task finalize` / `task validate`)

```text
rig fresh ; milestone with one sub-task that ran `jigc doc create inconsistency --title Drift --task <sub>` ; `mkfifo docs/inconsistencies/drift.md` ; `jigc milestone finalize drift-reports --format json` → no output and no exit after five minutes (process killed). The seven other occupant shapes at the same home refuse with exit 3.
```

## F9 · LOW · Riders seen while driving, all pre-existing or off the printed route

- where: various (listed in evidence)
- driven: driven — new binary; (1) and (3) also on rc.24
- class: six instances, unbounded

```text
(1) After (R6, D-7)'s rename or `--slug` route the link is still at the home: `jigc doc list inconsistency` → exit 1 with a raw OS error that prints an absolute host path (rc.24's control shows the same).
(2) `finalize.promote-clobber`'s regular-file route prints an absolute host path inside `jigc migrate <path> --as <doctype>`.
(3) `jigc task validate` exits 0 over an occupant that `finalize --dry-run` and `finalize` refuse with `finalize.promote-clobber` — link or regular file; rc.24 the same (`<e2e>/d7.sh validate-forecast`).
(4) On an orphan branch the setup refusal says "this repository has no commit yet"; `git rev-list --count --all` is 1.
(5) A committed link at `.jigc/AGENT.md` removed WITHOUT committing the removal: `jigc setup` exits 1 `setup.dirty-install-path` with the install written and staged, and a second run exits 1 again (the fixer's note says a second run completes); `git commit -- .jigc/AGENT.md` then `jigc setup` → exit 0. Reached only off the printed route, which says to commit the removal.
(6) In an autocrlf clone the first `jigc setup` rewrites CRLF checkouts of its own files (status stays clean) and reports the guide under its user-modified advisory.
```

## not_run

- A genuine concurrent assistant Task-tool spawn — the orchestrator's main-session half; my fan-out rows are an N-process simulation over the rendered Spawn commands
- The requested report.md: the harness refused the Write for a subagent report file, so the full report is this structured result only
- Linux, a case-sensitive filesystem, and any git other than 2.54.0 (Apple Git-157)
- (R6, K-1)'s timed one-shot plant measurement (only the rename toggler, 60 creates per door)
- L-22's container cell (a live worktree whose path the container cannot see) and the `git gc` permanence cell
- `jigc setup` with no git on PATH (the fixer's open InstallSubject::Unknown decision)
- The deleted-occupant cell (a home git holds and the disk does not, no baseline) — declared open in design/reconciliation.md
- A save-lock timeout at the copy-in writer; a git failure at the base-pin backstop
- The MigrationFixed finalize arm; the `jigc-feedback` doctype and the two triage workflows; `jigc ingest`
- `jigc uninstall` from inside a fan-out worktree or a subdirectory with the invocation-log knob on
- A hook or `.jigc/.gitignore` reached through a link at `jigc setup` (F2's two undriven Preserves members)
- F3's cell on the other migrate-* workflows and on a non-same-path source; F8's FIFO on rc.24
- The full gate (dev/gate) — this is the binary-driven half only; the builders' tests were neither run nor trusted

