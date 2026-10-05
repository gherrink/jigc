# Fix-pass plan — install and teardown

Family: `jigc setup` on an unborn `HEAD` — `(R1, F1)` — and `jigc uninstall`'s workbench guard — `(R9, F5)`.
Both doors live in `crates/cli/src/setup.rs`. Planned against HEAD = published `jigc 1.0.0-rc.24`; every
"driven" below was run on the installed `jigc 1.0.0-rc.24` in `dev/jigc-rig` throwaway roots. Nothing in the
repository was edited.

## 1. The contract

> **Neither install door exits 0 having destroyed a byte that no git object or index entry holds and that
> jigc cannot vouch for as its own: without `--force` it refuses before the first write or removal, naming
> every such path with a route that clears it; with `--force` it names what the consent was spent on.**

Testable form: for every path class each door writes or removes, plant a marked byte no object holds, run the
door without `--force` → exit ≠ 0, the path is in the finding, the mark is still on disk; follow the printed
route verbatim → the re-run exits 0; run with `--force` → exit 0 and the path is named on the door's output.

Design sentences it restores:

- `design/validation.md`, the `setup.dirty-install-path` row: *"Forgetting a new install path now costs a
  loud false alarm `--force` clears, never a silent sweep"* and *"the refusal installs nothing — which is the
  only way its own sentence (every path listed above still has its pre-run bytes there) can be true at a path
  the install rewrites whole."*
- `design/project-setup.md` → Teardown / cleanup (G5): *"All four remove nothing, name what they found, and
  route at the honest exits"*; and `design/worked-examples.md` flow 52 arm 2: *"No destroying door destroys
  bytes it does not name, and none takes a path it never looked at."*

## 2. Per row

### (R1, F1) — `setup` on an unborn `HEAD`

**Site.** `setup.rs::dirty_against_head` selects `--untracked-files=no` when `is_fresh_repo`, so on an unborn
`HEAD` no untracked path can reach `InstallSubject::probe`, `dirty_install_refusals`,
`forced_install_path_finding` or `record_failed_install`. The destroying writers are
`adapter::write_bootstrap_file`, the `.gitkeep` write in `adapter::init_project_layer`,
`setup::write_version_stamp` and `setup::write_compose_marker`. The route is worded in
`engine/src/finalize.rs::setup_dirty_install_finding`.

**Narrow fix (behaviour).** The unborn exemption stops being a property of the *query* and becomes a property
of the *member*:

- The pre-write question is asked with untracked paths included on an unborn `HEAD` exactly as on a born one.
- An untracked path on an unborn `HEAD` is exempt **only** at a member whose writer preserves the bytes it
  finds — `CLAUDE.md`, the settings file, `.jigc/.gitignore`, the root `.gitignore`, a foreign `pre-commit`.
  That is the declared exemption, unchanged, and it keeps the on-ramp.
- At a member whose writer replaces what it finds — `.jigc/AGENT.md`, `.jigc/config/.gitkeep`,
  `.jigc/config/packs.yaml` — and at the two oracle members when the oracle says *not jigc's*
  (`.jigc/version`, the guide), the door refuses with the **existing** `setup.dirty-install-path`, before the
  first write, exactly as it does one commit later. No new code.
- The per-member fact joins the existing disposition table (`InstallPathDisposition`, fenced by
  `the_install_path_class_is_dispositioned_member_by_member`). The default stays *refuse, born or unborn*, so
  a forgotten new member is loud. This is a registry row plus a guard condition — the two things the fix-pass
  rule allows.
- `--force` on an unborn `HEAD` now carries the real set, so the existing advisory
  `setup.forced-install-path` names each replaced path.
- S22: because `before` / `now` in `record_failed_install` and the footprint subtraction now see untracked
  paths on an unborn `HEAD`, a failed first run there records and stages what it wrote, an unstaged-but-
  unedited footprint still completes on a plain re-run, and an edit after the unstage re-arms the guard.
- **The route.** Today's route says *commit or stash … `git stash -u`*. Driven: on an unborn `HEAD`
  `git stash -u` and `git stash push -u -- <path>` both exit 1 with *"You do not have the initial commit
  yet"*. So the refusal's route on an unborn `HEAD` must not name a stash; it names an act that lands in one
  run (move the file out of the install path, or commit it) and then `--force`. The message's *"`HEAD` is
  untouched"* is also said of a repository that has no `HEAD`. The producer stays
  `setup_dirty_install_finding` (the `AmbushContract` row names that site).

**Class closed.** All four non-preserving members; the orphan-branch form (unborn `HEAD` in a born
repository); an untracked symlink at such a path on an unborn `HEAD` (refused by the gate, target untouched);
`--force` naming nothing on an unborn `HEAD`; S22's re-arm once unstaged; S22's record-and-stage on a
mid-span failure on an unborn `HEAD`.

**Class left.** (a) Untracked bytes at a merged-into member riding the first commit under `--no-verify` — the
declared exemption, no byte lost. (b) The born-`HEAD` symlink sibling found in planning — fork `install-2`.
(c) `(R1, F2)` — `--force` over a dirty hook names nothing — tier 2/3, already recorded.

**Red tests first** (`crates/cli/tests/setup_install_pathspec_guard.rs`, already registered in
`groups/g_migrate.rs`):

- `an_untracked_whole_rewrite_path_on_an_unborn_head_refuses_with_the_bytes_intact` — one unborn repository
  per member (`.jigc/AGENT.md` prose, `.jigc/config/.gitkeep` with content, `.jigc/version` prose,
  `.jigc/config/packs.yaml` with a comment): exit 1, `setup.dirty-install-path` names the path, bytes
  byte-identical, `git rev-list --count --all` still 0, and nothing else was written (`CLAUDE.md` and
  `.claude/` absent).
- `an_orphan_branch_with_an_emptied_index_is_the_same_cell`.
- `an_untracked_symlink_at_a_whole_rewrite_path_on_an_unborn_head_refuses_and_its_target_is_untouched`.
- `force_on_an_unborn_head_names_the_whole_rewrite_path_it_replaced` — exit 0,
  `setup.forced-install-path` names `.jigc/AGENT.md` and does not name an untracked `CLAUDE.md` beside it.
- `the_unborn_exemption_still_covers_every_merged_into_member` — the on-ramp control: untracked `CLAUDE.md`,
  root `.gitignore`, `.claude/settings.json` and `.jigc/.gitignore` together → exit 0, no finding, every mark
  in `HEAD`. Green before and after; it is what stops a fixer widening the refusal.
- `the_unborn_refusals_route_followed_verbatim_lands_the_install` — the route names no `git stash`; the
  first-named act, done verbatim, makes the re-run exit 0 **in one run**, with an untracked `CLAUDE.md` also
  present (the two-step trap in §3).
- `the_install_path_class_is_dispositioned_member_by_member` — extended: every member states whether its
  writer preserves or replaces.

In `crates/cli/tests/setup_failed_first_run.rs` (same group):

- `on_an_unborn_head_an_edit_after_an_unstaged_failed_run_re_arms_the_guard` — no git identity → exit 1
  `setup.install-commit`; set identity; `git reset -q`; append to `.jigc/AGENT.md`, comment in `packs.yaml`
  → exit 1 naming exactly those two, bytes intact.
- `on_an_unborn_head_an_unstaged_failed_run_with_no_edit_completes_on_a_plain_rerun` — the control: exit 0,
  no `--force`.
- `on_an_unborn_head_a_mid_span_failure_leaves_a_repo_a_plain_rerun_completes` — arm (a) of the suite on an
  unborn repository.

*Should have caught it:* `an_untracked_install_path_on_an_unborn_head_is_the_stated_exemption` plants only
`CLAUDE.md`; `a_whole_rewrite_install_path_refuses_with_the_users_bytes_intact` and
`a_comment_in_packs_yaml_refuses_rather_than_round_tripping_away` run only on a born repository;
`setup_failed_first_run::an_edit_after_the_failure_re_arms_the_guard_over_exactly_that_path` builds on a
seed commit and never unstages.

### (R9, F5) — `uninstall`'s workbench guard

**Site.** `setup.rs::workbench_paths` skips every directory named in `gitignore::ENTRIES`;
`setup.rs::workbench_foreign_areas` then covers only directory children of `tasks/` and `milestones/` plus
`displaced/`; `task.rs::foreign_areas` drops a non-directory (`Ok(_) => continue`). `logs/`, `state/`,
`index/` and the leaf positions under `tasks/` and `milestones/` are in no guard's subject, and
`pending_teardown` reads the same derivations, so `--force` narrates nothing there either.

**Narrow fix (behaviour).** The existing foreign-byte guard (the second of four) answers for every `ENTRIES`
subtree no other guard owns:

- Any leaf under `.jigc/logs/`, `.jigc/state/` or `.jigc/index/` that is not one of jigc's own named files
  there, and any non-directory child of `.jigc/tasks/` or `.jigc/milestones/`, blocks with the **existing**
  `uninstall.foreign-bytes`. Its message (*paths jigc did not write — the tree is gitignored*) and its route
  (*move out, or delete, then re-run; or `--force`*) are true of these paths as worded.
- **Not** `uninstall.untracked-workbench-file`: its route is *`git add -- <path>` is enough*, and driven,
  `git add -- .jigc/logs/notes.txt` exits 1 (*"paths are ignored by one of your .gitignore files"*).
- jigc's own files are a short stated row on the existing writer-registry mold
  (`engine::state::TASK_AREA_FILES` / `MILESTONE_AREA_FILES`): `state/file-state.json` and its lock,
  `state/setup-install-footprint`, `index/edges.json` and its lock, `logs/invocations.jsonl`. The members are
  the writers' own declarations, not copies.
- An `ENTRIES` directory with no stated owner defaults to *every leaf inside it is foreign*, so an eighth
  prefix fails closed rather than open.
- The narration reads the same derivation, so `--force` names what it took from these directories.
- The caches (`state/`, `index/` own files, the footprint, the locks) go with the tree unnamed —
  `design/storage.md` already declares them rebuildable. The help and G5 say so instead of stating a
  universal.
- The invocation log is fork `install-1`.

**Class closed.** `logs/**`, `state/**`, `index/**`; leaf children of `tasks/` and `milestones/`; the
tracked-then-edited leg inside those directories (the subject is the path, not the index); the `--force`
narration; the fail-open derivation for a future prefix (by the default above and by the test that iterates
`ENTRIES`).

**Class left.** The writer registry inverted over the whole tree with a source-scan fence (new mechanism —
1.x); `(R9, F3)` unless `install-1` takes the refuse arm; `decisions-pending.md` (I), the footprint
subtraction at `uninstall`, still owed at M57.

**Red tests first** (`crates/cli/tests/uninstall_workbench_subject.rs`, registered in
`groups/g_milestone.rs`):

- `a_file_inside_every_transient_prefix_blocks_the_teardown` — iterates `cli::gitignore::ENTRIES`, one
  install per prefix, a file planted **inside** the directory: exit ≠ 0, the path named, bytes intact,
  install intact, the code one of `UNINSTALL_DOOR.codes`; for `logs/`, `state/`, `index/` and the two leaf
  positions specifically `uninstall.foreign-bytes`.
- `a_nested_file_and_a_symlink_inside_a_cache_directory_block_too`.
- `a_tracked_then_edited_file_inside_a_cache_directory_blocks`.
- `force_names_what_it_took_from_the_cache_directories`.
- `the_foreign_refusals_route_followed_verbatim_clears_it` — move out → re-run exits 0; the route carries no
  `git add`.
- `jigcs_own_cache_files_never_draw_the_foreign_code` — the own-row completeness control: after real use
  (validate, a task started and discarded, a failed first setup that left the footprint, the log on),
  no `uninstall.foreign-bytes` finding names a path under `state/`, `index/` or `logs/`. This is the M52
  `renames.json` lesson as a driven cell.
- the log cell, per `install-1`.

In `crates/cli/tests/help_truth.rs` (`groups/g_flow.rs`): `uninstall_help_names_every_guard_the_door_runs` —
`uninstall --help` names each code in `UNINSTALL_DOOR.codes` and the `--force` option's help does not count
*three*.

*Should have caught it:* `a_plain_file_at_every_transient_prefix_name_blocks_the_teardown` plants a file
**at** each name, never inside; `flow53_acceptance::every_destroying_door_answers_for_the_bytes_it_did_not_write`
drives `uninstall` over one subject.

## 3. The other party's next step

Driven today, each changes what the fix must print:

1. **Unborn refusal → stash.** `git stash -u` exits 1 on an unborn `HEAD`. The current route's first resolving
   act is dead there (it already is for the staged cell that refuses today).
2. **Unborn refusal → commit only the named path.** Driven with an untracked `CLAUDE.md` beside
   `.jigc/AGENT.md`: after committing the one path the repository is born, and the re-run refuses again over
   `CLAUDE.md` under the born rule. Following *that* route (`git stash -u`) lands the install, and then
   `git stash pop` fails — *"CLAUDE.md already exists, no checkout"* — leaving the stash. So the unborn route
   should lead with the one-step exit (move the file out of the install path), and the test asserts one run.
3. **Symlink refusal → remove the link.** Driven on the member that refuses today (`.jigc/.gitignore`):
   removing the link and committing the removal lands on the re-run; removing it without committing draws
   `setup.dirty-install-path` at the commit backstop with the install written and staged. The route must say
   *commit*.
4. **`uninstall.foreign-bytes` → move out.** Driven on `displaced/`: the route lands on the re-run.
5. **Log on → a refused uninstall is itself logged** (3 → 4 records, driven), and a teardown that succeeds
   re-creates `.jigc/logs/invocations.jsonl` with its own record (driven: one-record residue; the second run
   acts, the third is the no-op). Under the refuse arm of `install-1` that residue would make the second run
   refuse over the first run's own record.

The fixer runs all of §`next_step_drives` on the built binary before the fix is believed.

## 4. What must not move

Checked: no frozen schema hash or manifest, no pinned `--format json` key (`ENVELOPE_ARMS`: `setup`'s seven,
`uninstall`'s four and `removed`'s seven flags), no `contract-version`, no compose golden, no
`STORE_EXIT_FLIPS`, `COMMITTING_DOORS` or `VERB_KINDS` row, no finding code added or removed
(`UNINSTALL_DOOR.codes` stays four; `DESTROYING_DOORS` stays six), `cli::SETUP_UNBORN_EXEMPTION` (the
*posture* exemption — `setup` still mints the first commit on an unborn `HEAD`), the co-author trailer on the
install commit, the `task-discard.foreign-bytes` / `milestone.foreign-bytes` subjects (they pass their own
area lists), and M54 S22's ten arms.

What does move, none of it frozen: exit 0 → 1 at the cells the rows name; the internal
`InstallPathDisposition` table and its fence; the engine function's parameter list; and the guide body digest
`jigc-body-blake3`, once, in the pass's single guide batch.

## 5. Docs

Same commit as the rule: `design/validation.md` (the row's unborn sentence); `design/project-setup.md` →
Idempotency (the S22 sentence, true on an unborn `HEAD` after the fix) and → G5 (c)/(d) (*"the two subtrees
(c) structurally excludes"* is a miscount); `design/worked-examples.md` flow 50 arm 6 (*"a prefix added …
narrows the subject automatically"*); `design/surface-contract.md` → the second member; `jigc setup --help`
and `jigc uninstall --help` including the `--force` option's *"all three guards"*; the doc-comments on
`dirty_against_head`, `InstallPathDisposition`, `workbench_paths`, `uninstall`.

**Guides are the exception.** `QUICKSTART.md` (*"staged, unstaged or untracked — it stops"*) and
`MIGRATING.md` gate 4 need the unborn carve-out, and `QUICKSTART.md`'s teardown sentence needs the cache
rule. Both guides are embedded in the installed skill, so any guide byte moves the body digest; M53's rule is
one guide batch per pass. These sentences ride that one batch, shared with the other families.

## 6. Forks

- **`install-1` — the invocation log.** Take and name, or refuse.
- **`install-2` — the born-`HEAD` symlink sibling found in planning.** Hold to the verified boundary, or
  apply the door's own existing symlink rule to the three sibling writers.

Detail in the structured result. Everything else is settled by the rulings, the declared exemption and the
code's own precedents.

## 7. Commits

1. `fix(setup): the dirty-install refusal names a route that works with no commit yet`
2. `fix(setup): an untracked install path jigc would replace refuses on an unborn HEAD too` — closes (R1, F1)
3. `fix(setup): the whole-rewrite writers refuse a symlink instead of writing through it` — only if
   `install-2` takes the robust arm
4. `fix(uninstall): a file jigc did not write inside a cache, log or work-unit root blocks the teardown` —
   closes (R9, F5)'s planted members
5. per `install-1`: `fix(uninstall): the invocation log is named when the teardown takes it`, **or**
   `fix(uninstall): a teardown does not re-create the workbench to log itself` then
   `fix(uninstall): the invocation log blocks the teardown` — closes (R9, F5)'s log member
6. the pass's one guide batch (shared) — `docs(guides): …`
7. the pass's record (shared logs; the orchestrator's single branch)

## 8. Left open

See `left_open` in the structured result: the declared unborn face, `(R1, F2)`, `(R9, F3)`,
`decisions-pending.md` (I), the external-writer race (ruling 4), the inverted registry, and three surface
defects observed in planning.
