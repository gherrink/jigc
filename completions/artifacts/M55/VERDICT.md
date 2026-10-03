# M55 — the findings channel: completion verdict

**Verdict: COMPLETE — built, audited and triaged. The audit found five defects: three from the code
review (2 MEDIUM · 1 LOW) and two from the e2e, which passed 22 of 24 scenarios. All five were fixed,
one commit each with a red test. The fixers surfaced two more defects, both fixed, and one report-only
gap, F21, which is filed open. The genuine spawn MATCHes the sim's golden. Re-verified 4376 passed /
0 failed.** Written 2026-10-03, after the last triage commit (`72f54033`). The audit's raw output is
the source of this record: the milestone-build harness's completion stage returned both passes as
structured data (`code_review`: three ranked findings with severity, location, confidence and evidence,
plus a summary; `e2e`: `overall_pass: false`, a summary and twenty-four scenarios). What follows
condenses that output and claims nothing beyond it. Triage dispositions come from git.

| | |
|---|---|
| Planned | 2026-10-02 Settle, S1–S17 with the design review's R1–R4 and the gate-record's R5 ([DECISIONS.md](../../../DECISIONS.md) → *M55 settled*) · [planning-gate-record](planning-gate-record.md), 15 rows |
| Built | eleven increments; the build closed at `542fccda` (the fold-back reading *built, not audited*), gate **4357 / 0** |
| Audited | at `542fccda`, through `target/debug/jigc` (`1.0.0-rc.22`). Code review: **2 MEDIUM · 1 LOW**, no HIGH. E2E: **22 of 24** scenarios passed, `overall_pass: false` on two defects |
| Triaged | all five confirmed findings **fix-now**; none deferred, none contested. CR2's shape was the human's: option (a) |
| Fixed | E1 `c849c8ef` · CR1 `1ff75c1c` · CR3 `00c26593` · E2 `49a0c0da` · CR2 `317c813c` |
| Surfaced by the fixers | structural deltas at every door `1306d891` · unmanage's false ack `97008b20` · F21 filed open `92f279c3` + `72f54033` |
| Genuine spawn | **MATCH**, tree `d30d6110b5b4ec4eb9fed298f4db51942503122e`, at `72f54033` ([genuine-spawn/](genuine-spawn/README.md)) |
| Re-verified | `dev/gate` at `72f54033`: **4376 passed / 0 failed** (build close 4357; the difference is the fixes' own tests) |

## What the milestone claimed, and whether it is true of what shipped

*A finding about jigc, or a disagreement between a project's own docs and code, is filed through the
real binary as one doc in a commit of its own — beside an open code task without touching its staging,
and never over an earlier finding — read back as a triageable index in one call, and moved off `open`
through a guided door; several reporters file into one store through a fan-out whose sub-tasks are
never told to run a refused door; a pull or a branch switch no longer blocks or misroutes the next task;
and the rows this repository already holds are filed, re-driven, as a seed M56 adopts.*

The code reviewer's verdict, condensed from its summary: **the deliverable mostly holds on the shipped
packs.** It ran `dev/gate --private-target` green (4357 / 0) and confirmed on a rig that the create-only
gate refuses `create.already-exists` before copy-in, that the doc-only commit leaves other staged paths
staged when a commit is rejected, that the seed (79 `jigc-feedback` + 12 `inconsistency` at the time)
keeps its own conventions, and that no schema-hash or schema-version moved apart from the two new
manifest rows. The golden moves were adjudicated in DECISIONS: 34 (28 catalog + 6 describe) at
Increment 8, and 12 for the planning hint. Nothing breaks the engine invariants: the engine diff carries
no pack content outside tests, has no LLM path, and leaves slots and placeholders untouched. **Two
defects were MEDIUM, both in reach of a project layer or an ordinary git operation rather than the
shipped packs alone** (findings CR1, CR2), and one LOW was inherited from the ordinary arm (CR3).

The e2e auditor's verdict: **every acceptance flow, A through E, passes** through the real binary at
`542fccda` in throwaway `dev/jigc-rig` repos, and the fan-out join was byte-identical across filing
orders under both `squash` settings. **Two defects set `overall_pass: false`:** a fan-out sub-task
composed from a report or triage workflow was told its staged paths stay out of the commit, which the
join contradicts (E1, new in M55), and one false line in the planning sub-task's text (E2, tier-3).

**After triage, the deliverable holds.** Every confirmed finding is fixed with a red test, the two the
fixers surfaced are fixed, F21 is report-only and filed open, and the genuine spawn closes the half the
headless e2e could not run.

## The code review — three findings, three fixed

Ranked as the reviewer ranked them. Each entry gives the reviewer's severity, location and confidence,
its evidence condensed, the repro it drove, and the triage disposition.

### CR1 · MEDIUM — the doc-only predicate read only a workflow's top-level includes

- **Location:** `crates/cli/src/task.rs:1391-1395` (`composes_doc_only_finalize`), consumed at
  `task.rs:4236-4247` (`commits_doc_only`) and `start.rs:1648/1985/3102` (`Composition.doc_only`).
  **Confidence:** high — reproduced on the audited binary.
- **Evidence:** the predicate was `def.includes.iter().any(|step| step == "finalize-doc-only")`, so it
  saw direct includes only, while the composer expands includes at every depth (`expand_step` recurses
  through `BodySegment::Include`, and the same milestone's `sub_task_omission_set`, `start.rs:3164`,
  walks the tree at any depth). A workflow reaching `step:finalize-doc-only` through a wrapping step
  composed the path-scoped promise but finalized on the ordinary model, sweeping another task's staged
  code into the report's commit — exactly what flow B state 3 exists to prevent. The reverse holds too:
  a project shadow of the step's body kept the predicate true while the text no longer said
  path-scoped. Bound by `grep -n '\.includes' crates/cli/src/*.rs`: two predicates of this
  top-level-only shape — this one and the older `composes_review_hold` (`task.rs:4216-4222`,
  `MIGRATION_FINALIZE_STEP`), which has the same depth blind spot. The shipped packs include both steps
  at the top level only, so a project-layer workflow is needed to reach it.
- **Repro (driven, rig `fresh`, dev + methodology packs):**
  1. Add `.jigc/config/steps/my-final.yaml` holding `{{ include: step:finalize-doc-only }}`.
  2. Shadow `.jigc/config/workflows/report-inconsistency.yaml` so its last include is `step:my-final`;
     commit both.
  3. `jigc start --workflow report-inconsistency "readme and code disagree"` — the text says *This
     finalize commits path-scoped … Anything else staged … stays staged*, while its `what's-left:` line
     names *the carryover gate*, the ordinary model's spelling.
  4. Fill the doc and the commit doc; `echo x > code.txt; git add code.txt`.
  5. `jigc task finalize` → exit 0, `added code.txt` and `promoted
     docs/inconsistencies/readme-and-code-disagree.md`; `git show --stat HEAD` lists both.
- **Disposition: fix-now → `1ff75c1c`** `fix(cli): the doc-only commit and the review hold read the
  composed include tree at any depth`. One walker, `start::walk_include_tree`, serves the sub-task
  omission set and both arms (`start::composes_step`) over the same cascade-resolved step source the
  compose expands, at the three compose sites and at the finalize seam; the arm keys on the step id,
  never its body. The review hold's blind spot is fixed in the same commit. **Red tests:**
  `doc_only_finalize::the_doc_only_step_reached_through_a_wrapping_step_commits_path_scoped`,
  `doc_only_finalize::a_project_shadow_of_the_doc_only_step_body_keeps_the_doc_only_commit`,
  `migration_review_hold_axis::a_hold_step_reached_through_a_wrapping_step_still_holds`.

### CR2 · MEDIUM — L2's one route asserted a branch switch whatever caused the missing baseline

- **Location:** `crates/engine/src/file_state.rs:1564-1585` (`rename_dangling_baseline_finding`),
  selected by the history predicate at `file_state.rs:1490-1494` and, new at store scope,
  `file_state.rs:1067-1072` via `cli.rs:1478`. **Confidence:** high — reproduced.
- **Evidence:** the producer fires whenever a path is recorded, absent on disk, and has no history at
  `HEAD`. Its own doc comment names three causes — `git reset --hard`, a branch switch, a rebase past the
  creating commit — but M55's route stated one as fact: *a branch switch left this baseline behind …
  switch back to that branch to work on it again*. For a reset, and for a doc ingested and then deleted
  (which before M55 store scope graded as the blocking weak-signal finding), there is no branch to switch
  back to. The advisory then persisted on every task gate, and only `jigc unmanage <path>` cleared it —
  the route M55 had removed. That is a surface-contract law-1 lie for two of the three causes the comment
  names, plus the ingest case, with no named way out. Instance found by driving those causes; whether
  other causes reach this producer is unbounded. The design (findings-channel.md §6 L2, R5) argued only
  from the branch-switch case.
- **Repro 1 (driven, rig `fresh`, one branch `main`):** land a `report-inconsistency` finalize, then
  `git reset --hard HEAD~1`; `jigc validate` prints the advisory with the switch-back route, and no other
  branch exists.
- **Repro 2 (driven):** write an uncommitted conformant doc under `docs/inconsistencies/`, `jigc ingest`
  (*adopted — indexed + baselined*), delete the file, `jigc validate` → the same advisory, branch route
  included. After a later report task landed (exit 0), the row was still printed.
- **Disposition: fix-now, shape decided by the human → `317c813c`** `fix(engine): a history-less
  baseline no branch carries routes at unmanage, not at a branch switch`. The human chose option (a):
  distinguish the causes by whether a branch tip carries the path. A CLI-supplied `OtherRefsPredicate`
  (the engine stays shell-free) asks `refs/heads/*` and `refs/remotes/*`, only after history reads empty;
  a failed read answers *carried*. Carried keeps M55's switch-back route byte for byte; not carried draws
  a new producer, `rename_orphaned_baseline_finding`, that claims no cause and offers
  `jigc unmanage <path>`. Same code, severity and `(code, target)` key at both scopes. Recorded in
  [DECISIONS.md](../../../DECISIONS.md) → *M55 completion triage, CR2*. **Red tests:**
  `file_state_history_gate::ingested_then_deleted_doc_routes_at_unmanage_and_following_it_clears`
  (runs the emitted route verbatim; the row clears at both scopes),
  `file_state_history_gate::branch_switch_routes_back_while_a_branch_carries_the_doc_and_at_unmanage_once_none_does`
  (local branch → remote-tracking only → tag only), and the engine units
  `rename_weak_signal_history_less_on_no_branch_routes_at_unmanage`,
  `rename_other_refs_is_asked_only_when_history_is_empty`,
  `store_twin_history_less_baseline_on_no_branch_is_the_orphaned_finding`. The history gate's op table
  was re-derived with a route column: reset, rebase, amend, gc, stash, rewrite and milestone-create rows
  now route at unmanage.

### CR3 · LOW — a triage that changes nothing about the doc reached git and was routed at a hook

- **Location:** `crates/cli/src/task.rs:3177-3187` (the doc-only diff signal: `staged_promotable ||
  owner-artifacts`) → `crates/cli/src/milestone.rs:1376-1400` (`git_commit_paths`). **Confidence:** high
  — reproduced; the ordinary arm does the same.
- **Evidence:** the diff signal counted a staged doc as a diff whenever its type promotes, without
  comparing bytes to the committed doc. No data is lost — the baseline hash is unchanged and another
  task's staged path stays staged — but the route names a hook that never spoke. The same no-op on the
  ordinary arm (a `single-task` re-setting a doc) gets the same route, so the gap predates M55; M55's
  triage workflows make it easy to hit by re-triaging a finding to the status it already has. Instance,
  unbounded: every door whose empty-commit guard counts a promotable staged doc as a diff shares it.
- **Repro (driven, rig `fresh`):**
  1. Land a `report-inconsistency` doc.
  2. `jigc start --workflow triage-inconsistency "triage first"`, then
     `jigc doc set-field inconsistency:first-one#status --value open --task …` — copies it in unchanged.
  3. Fill the commit doc; `jigc task finalize` → exit 1, *`git commit` was rejected … nothing added to
     commit but untracked files present*, routed *Fix the hook's complaint, then re-run*.
- **Disposition: fix-now → `00c26593`** `fix(cli): a staged doc written back unchanged is an empty
  commit, not a hook rejection`. Both arms count a promotable staged doc only when its would-be-committed
  blob differs from `HEAD`'s, and a recorded owner-artifact only when git sees it differ; such a task gets
  `finalize.empty-commit` (or `nothing-staged`) before git runs, and `--dry-run` agrees. Fails toward
  *changed* when git cannot answer. **Red tests:**
  `identical_staged_doc::a_triage_writing_the_filed_bytes_back_is_an_empty_commit_not_a_hook_rejection`,
  `identical_staged_doc::a_single_task_writing_a_committed_doc_back_unchanged_is_an_empty_commit`,
  `doc_only_finalize::an_unchanged_recording_is_an_empty_commit_and_a_recaptured_artifact_lands`.

## The e2e half — 24 scenarios, 22 green, two defects

| # | Scenario | Result | What was driven |
|---|---|---|---|
| 1 | Flow A · `report-jigc-feedback` lands its doc alone, a foreign staged path left out | pass | the emitted create line carries no flag; an unfenced `# a shell comment` refused `write.slot-heading-depth` (exit 1), the fenced one accepted; finalize exit 0, one file committed, `src/foreign.rs` still staged and named in left-out; `--dry-run` forecast the same set |
| 2 | Flow A · read surfaces carry `title` and `fields.status`; a hand-deleted status still projects `open` | pass | `doc list --format json` and `doc show` carry the title; after deleting `status:` by hand and committing, both report `open`; store `validate` one advisory `file-state.hash-matches`, exit 0 |
| 3 | Flow A · `report-inconsistency` from the catalog, three sides, hidden workflows absent | pass | bare `jigc start` lists `report-inconsistency` and none of the three hidden ones; three `add-item --slug`; finalize one file, foreign path still staged; `triage-inconsistency` composes by name |
| 4 | Flow A · `triage-jigc-feedback` moves the row to `resolved`, landing that doc alone | pass | the first `set-field` acks *copied in for update*; finalize one file; the diff adds only status, pinned-by and resolution, every hand-edited line kept |
| 5 | Flow B · the three staging states, plus state 2 with `--carry-staged`, on the shipped report workflow | pass | four runs with the pre-commit hook installed: each report finalize exit 0 with one commit holding only the finding, the staged index byte-identical before and after, the code task landing its paths afterwards |
| 6 | Flow B · a pending `.jigc/config` delta stays out of the report's commit | pass | `.jigc/config/manifest.yaml` under left-out, absent from the report commit, landed by the code task |
| 7 | Flow B · the triage edit shape across states 1–3 | pass | `--dry-run` forecast the promoted doc with the code paths left out; the real finalize one file, index identical, `status: resolved` |
| 8 | Flow B · omitting context (`park-idea`, ordinary `step:finalize`) | pass | state 1 the idea alone; state 2 exit 3 `finalize.carried-staged`, nothing committed; state 3 the idea plus both code paths — unchanged behaviour |
| 9 | Flow C · create-only refusal on both doors, before copy-in; `--slug`; re-run idempotence | pass | a re-run create inside the first task is idempotent; in a second task four attempts (`doc create` / `doc author` × same title / title + `!`) each exit 1 `create.already-exists`, never `write.title-ignored`, with nothing staged and the first finding's checksum unchanged; `--slug …-again` lands |
| 10 | Flow C · the general case under `park-idea`, and the dropped-key / misspelt-key shadows | pass | a different title onto the committed idea `write.title-ignored` exit 1, its route never names `doc rename`; a shadow without `new: true` acks *copied in for update*; a shadow with `nwe: true` refused `workflow-refs.malformed-front-matter … expected one of type, as, new` |
| 11 | Flow D · N-process fan-out sim, `Spawn:` lines verbatim, colliding ids, order-invariant join (`squash=true`) | pass | three sub-tasks, every span exit 0; each composed text has 0 `jigc task finalize`, 0 `commit:<sub>` writes and the sub-task trailer; the lower task id keeps the bare slug; one commit of 4 files; forward and reverse filing byte-identical. *The real Task-tool spawn was not run* |
| 12 | Flow D under `squash=false`, both filing orders | pass | one commit each way, byte-identical; the docs-only sub-tasks still compose author-commit, as the declared bound says |
| 13 | Flow D / Increment 9 · the seed's adoptability predicate, on the join output and on the committed seed, plus a red control | pass | ingest + `validate --format json`: the join output 0 findings; the committed seed 91 files, 0 findings, 79 + 12 managed, 58 open / 33 resolved, matching the ledger at the time; the red control (`kind: notakind`) 2 findings |
| 14 | Increment 3 · sub-task composition for `amend` / `migrate-adr` / `park-idea` / `planning` / `single-task` | pass | under `squash=true` four sub-tasks carry no finalize, approve-prose, render or commit-write line and one trailer; the planning sub-task's one `task finalize` mention is not a `Run:` line (scenario 16) |
| 15 | **DEFECT E1** · a fan-out sub-task's `what's-left:` line promises the doc-only path scope, but the join commits staged code | **fail** | below |
| 16 | **DEFECT E2** · the planning sub-task text says the per-task door gates the record | **fail** | below |
| 17 | Declared bound reproduced · `jigc task validate <sub>` blocks on the omitted commit doc under `squash=true` | pass | exit 3 on `commit:<sub>` type and summary while the text never asks for the commit doc and the join lands without it — the bound at DECISIONS.md, owed to the S2 partial re-review; not a new finding |
| 18 | Flow E L1 · a pulled edit is advisory at store scope and absorbed by the next task | pass | store `validate` exit 0 with the advisory; the next task's finalize carries both the teammate's line and its own; control: an out-of-band edit during the task blocks `reconciliation.conflict-block`; a non-conformant hand edit stays blocking |
| 19 | Flow E L2 · a branch switch is advisory with one route at both scopes; delete-with-history still blocks | pass | after `git switch main`, one advisory `reconciliation.rename` routed at the switch back, `unmanage` 0 times, the store row equal to the task row; control: `git rm` + commit on the branch blocks |
| 20 | Increment 6 · read-surface row states, `--task` rows, the ADR default projection | pass | managed / stamped-but-broken / unstamped-broken rows, `fields` never `{}`; `--task` rows show the staged status; an ADR with `status:` deleted reads `proposed` with its bytes untouched |
| 21 | Increment 7 · the new doctypes, the freeze, zero hash movement, the existing corpus unchanged | pass | `doc schema` in all three formats; a shadow adding an enum member fails pack-load on `schema-hash mismatch`; only two manifest entries added; `validate --format json` byte-identical between `1.0.0-rc.22` and the built binary. The absent-manifest-entry arm was not exercised |
| 22 | Increment 10 · the generated crate README and the `cheap-vs-robust` hint fold | pass | `dev/crate-readme --stdout` byte-identical to the committed README, all six rewritten targets exist, `cargo package --list` carries it; the planning compose names *artifact that can drift* twice |
| 23 | Observation (tier-3, instance, unbounded) · the `create.already-exists` route is a two-hop when the task's role is already bound | pass | following the route's `--slug` or a distinct `--title` meets `write.identity-change`, which routes at `jigc doc rename`. Not a claimed acceptance assertion; recorded for triage |
| 24 | Observation, older than M55 (also on `1.0.0-rc.22`) · the *docs are also staged in open task* note fires when that task stages no doc of the listed type | pass | byte-identical on `1.0.0-rc.22` and the built binary, so not an M55 regression. Instance, unbounded |

### E1 — a fan-out sub-task was promised the doc-only path scope (new in M55)

- **Evidence:** the composed `what's-left:` line of a report or triage sub-task named *the path scope
  that leaves every other staged path staged*, while the milestone join commits whatever the sub-task's
  worktree staged. [findings-channel.md](../../../design/findings-channel.md) §10 applies the doc-only
  commit model only when the task *is not a fan-out sub-task*. Cause: `start.rs:3102`, the sub-task
  compose site, set `doc_only` from `composes_doc_only_finalize(&def)` without the `owning_milestone`
  check that site already computes; `render.rs:560` fed it into `CommitModel::of`. Bound: `doc_only` is
  set at three sites and only `3102` composes a sub-task; four workflows compose `step:finalize-doc-only`.
- **Repro (driven):**
  ```sh
  rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
  jigc milestone create mixed
  jigc milestone add-task mixed "code sub" --workflow dev-task
  jigc milestone add-task mixed "report sub" --workflow report-jigc-feedback
  jigc milestone provision mixed
  (cd .jigc/worktrees/report-sub && jigc workflow report-jigc-feedback --task report-sub | grep "^what's-left")
  # → names "the path scope that leaves every other staged path staged"; code-sub names "the carryover gate"
  # in the report-sub worktree: echo x > src/z.rs; git add src/z.rs; file the finding
  jigc milestone join mixed; jigc milestone finalize mixed
  # → "added src/z.rs … report-sub: 2 docs, 1 code file": the staged path was committed
  ```
- **Disposition: fix-now → `c849c8ef`** `fix(cli): a fan-out sub-task's what's-left line never promises
  the doc-only path scope` — the shared re-compose spine asks the `owning_milestone` membership it already
  reads. **Red test:**
  `sub_task_composition::e1_a_doc_only_workflow_composed_for_a_sub_task_never_promises_the_path_scope`.
  CR1's `1ff75c1c` then moved the fan-out exclusion into one place, `task::doc_only_commit`.

### E2 — the planning sub-task's text named the refused per-task door (LOW, tier-3)

- **Evidence:** the `plan-sub` sub-task's composed text said *`jigc task finalize` BLOCKS on an
  unfilled one … and lands the record once every gate is answered*. In a sub-task that door refuses with
  `finalize.milestone-sub-task` (exit 3) and the boundary is `jigc milestone finalize`. The sentence came
  from `crates/cli/packs/methodology/steps/author-planning-record.yaml:12` and named the door literally
  rather than through `{{ cli.finalize-task }}`, so the derived omission set never saw it. Bound: a
  literal grep of the pack steps finds three hits, two of them the correct *never `jigc task finalize`*;
  about 40 unprefixed *finalize* mentions were not adjudicated, so the class is unbounded beyond the
  literal form. The roadmap's Increment 3 claim (no `Run:` line) holds; this is design §6's *every false
  line* reading.
- **Repro (driven):** a milestone with a `planning` sub-task, its `Spawn:` line run verbatim
  (`jigc workflow planning --task plan-sub`), the composed text read at line 90.
- **Disposition: fix-now → `49a0c0da`** `fix(packs): a planning sub-task is never told the refused
  per-task door blocks its gates` — the sentence names the boundary by role, true at both kinds of task;
  twelve `planning` compose goldens move by the reworded sentence. **Red tests:**
  `sub_task_composition::e2_a_planning_sub_task_states_the_gate_its_join_enforces` (driven true at the
  join) and `sub_task_composition::e2_no_pack_text_names_the_per_task_door_outside_the_never_phrasing`
  (a fence over both packs' step and workflow text, its misses declared on the test).

### The two observations (scenarios 23 and 24)

The auditor recorded both inside passing scenarios and asked for neither as a finding: the first *for
triage*, the second as older than M55. **The triage gave neither a disposition** — no commit, no
register row and no seed doc names them. Both were then fixed now, after the verdict above, one commit each:

- **O23 — scenario 23, `create.already-exists` under a bound role** → fix-now, `4aca049b`. Its route
  named a distinct `--title` / `--slug` that the one-doc-per-role rule refuses (`write.identity-change`);
  it now names the doc the task holds, the exits that end the task, and the `jigc start --workflow` the
  next doc belongs in. Red: `create_only_gate::a_bound_role_routes_an_occupied_id_at_the_next_task_not_a_distinct_identity`;
  the identity-change route under `new: true` was checked and runs, pinned by
  `create_only_gate::under_new_the_identity_change_route_runs`.
- **O24 — scenario 24, `doc list <type>`'s staged note** (pre-existing on rc.22) → fix-now, `1e7382fe`.
  It named any open task staging anything; it now reads the staged arm's own row predicate, so it names
  only tasks staging that doctype and its route never lands on an empty listing. Red:
  `doc_list::a_narrowed_listing_names_only_tasks_staging_that_doctype`.

## Surfaced by the fixers — two fixed, one filed open

### Project structural-op deltas applied on a fresh compose only (found while fixing CR1)

The project layer's `structural-op` deltas (`jigc config replace-step / insert-step / remove-step`)
applied only on a fresh compose. The resume and sub-agent re-entry, the verb-minted compose behind
`jigc task amend` and `jigc migrate`, and the finalize-time arms keyed on a composed step read the
workflow without them, so a `replace-step` of `step:finalize-doc-only` composed on one commit model and
finalized on the other, and a resumed task's text was not its minted text. **Fixed → `1306d891`** `fix(cli):
a project structural-op delta resolves one workflow at every door` — one seam,
`start::apply_project_structure`, for all four, with the compose gate validating the same post-delta
list. **Red tests** (`structural_delta_resolution::`):
`a_replace_step_off_the_doc_only_step_composes_and_commits_the_index`,
`a_replace_step_into_the_doc_only_step_composes_and_commits_path_scoped`,
`a_resumed_task_composes_its_minted_text_under_a_structural_delta`,
`an_insert_step_into_amend_composes_at_the_verb_and_on_resume`,
`a_replace_step_into_the_review_hold_holds`.

### `jigc unmanage` claimed a file was left on disk when there was none (found while fixing CR2)

After CR2, the orphaned baseline routes at `jigc unmanage <path>`, so the verb's ordinary subject became
a path with no file at it — and its ack said *the file is left on disk* unconditionally. **Fixed →
`97008b20`** `fix(cli): unmanage's ack no longer claims a file is left on disk when there is none` — the
report gains `file_absent` (off the wire; a declared exclusion in the text/JSON parity fence), true only
on a real drop of a contained path with no directory entry, read with `symlink_metadata`. Every other
case keeps its ack byte for byte. **Red test:**
`unmanage::unmanage_of_a_missing_file_does_not_claim_it_is_left_on_disk` (ingests a conformant ADR,
deletes it, runs the store sweep's emitted route verbatim, and pins the file-present ack whole).

### F21 — `jigc validate`'s workflow-ref check ignores project structural-op deltas (filed open)

Found while fixing the structural deltas. `enumerate_store_workflows` and the engine's
`workflow_refs_store` read the raw workflow definition without the project's structural-op deltas, so a
broken or circular include, or a bad ref inside an inserted native step, is never reported.
**Report-only**: it misses findings and never makes a wrong commit, and the fix needs an engine API
change. **Recorded → `92f279c3`** as register row F21 in [planning-findings.md](planning-findings.md);
**filed → `72f54033`** into the seed through the report workflow, as
[seed/jigc-feedback/validate-misses-a-workflow-ref.md](seed/jigc-feedback/validate-misses-a-workflow-ref.md),
`status: open`. Its re-drive on the debug build from `92f279c3`: a `config insert-step` of a broken
include records at exit 0; once committed, `validate --format json` exits 0 with no findings while
`jigc task amend` refuses on `workflow-refs.include-resolves`; the same include in a whole-file shadow
is reported. The seed now holds **92 docs** (80 `jigc-feedback`, 12 `inconsistency`), re-filed as one
fan-out with the other 91 byte-identical ([DECISIONS.md](../../../DECISIONS.md) → *M55 completion triage:
F21 filed into the seed*).

## The genuine spawn — MATCH

The e2e ran the fan-out as an N-process sim and said in its summary that the real concurrent spawn
from the orchestrator's main session was not run and that the sim does not stand in for it. The
orchestrator ran it at `72f54033`: three general-purpose sub-agents launched concurrently by the main
session's Agent tool, one per sub-task of flow 58's fixture, each given only its brief. Every sub-agent
command exited 0. The join merged 6 docs and suffixed the colliding slug `-2` on the higher task id, and
`milestone finalize` committed 4 files. The genuine tree, the sim tree re-derived at comparison time and
the recorded golden all read `d30d6110b5b4ec4eb9fed298f4db51942503122e` — **MATCH**. The blackboard
witness is each sub-agent's own statement (only `jigc` calls, no path outside its worktree, no direct
edit, no `git`) plus the orchestrator's pre-join check that every worktree was clean. Bound: the first
half is a self-report, and the raw transcripts are not committed. The record, the scripts and the briefs
are in [genuine-spawn/](genuine-spawn/README.md).

## Not run — what the audit stated it did not drive

- **The genuine concurrent spawn**, by the e2e (scenario 11) — run since by the orchestrator, above.
- **The absent-manifest-entry arm** of the freeze (scenario 21): it needs an embedded-pack rebuild.
- **The packaged-bytes comparison and the one-byte-drift red fence** of the crate README (scenario 22):
  cargo-test fences, not binary behaviour.
- **About 40 unprefixed *finalize* mentions in pack steps** (E2's bound): not adjudicated; the fence
  `49a0c0da` added covers the literal spelling only, its misses declared on the test.

## Declared bounds, carried in writing

- **M55 built no version.** The crate is still `1.0.0-rc.22`; `1.0.0-rc.23`, carrying M54's audit fixes
  and M55 with these fixes, is release PR #2, the human's merge and deployment approval. The crates.io
  re-read of the README that publish renders (S15) follows it.
- **The partial re-review** over the merged M54 S18 and M55 S16 axes is owed on that release candidate,
  beside the blind trial and before the call ([DECISIONS.md](../../../DECISIONS.md) → *the road to the
  1.0.0 call*).
- **`jigc task validate <sub>` blocks on the omitted commit doc under `squash=true`** (scenario 17) — the
  declared bound owed to that re-review, reproduced, not new.
- **F21 is open** and its fix needs an engine API change.
- **The two e2e observations are fixed** (scenarios 23 and 24 → `4aca049b`, `1e7382fe`; → The two observations).
- **The genuine spawn's blackboard witness rests partly on self-report.**

## What is next

M55's pull request `milestone/findings-channel/main → main`, which the human merges → release PR #2,
publishing `1.0.0-rc.23` on the human's merge and approval → the crates.io README re-read → the blind
agent trial and the partial re-review on that candidate → **the 1.0.0 call, which is the human's.**
