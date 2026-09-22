# M53 completion audit — code review (Opus, read-only, driving the built debug binary at `45427083`)

## Summary

The M53 deliverable holds. I drove all five increments' headline claims through the real binary and every one behaved as claimed: a landed `milestone finalize` now parks the milestone area's own complement (incl. two levels under `merged/`) at `.jigc/displaced/<milestone-id>/` and names it on both channels; a displacement that cannot park leaves every un-moved byte on disk, exits 0, and raises one `finalize.foreign-bytes` per area with an area-keyed count; a pin-less directory is refused at all five task by-id doors and all six milestone doors with the family's own `(code, target)` and a repo-relative route; an uncommitted `cherry-pick -n` is refused at `task validate`/`task finalize` with `MERGE_MSG` and the index byte-unchanged and its `git reset` route running clean; and `milestone create/add-task/add-from-spec` + `jigc start` all refuse `write.unslugable-title` before any write. The full gate is green (3898 passed / 0 failed, clippy + fmt clean), the 634 goldens are byte-unmoved across the whole pass, and the fold-back honestly reads "built, not audited". The headline concern is one un-swept sibling of Increment 3's own class: `engine::milestone::owning_milestone` still decides milestone-ness by `is_dir()`, so a pin-less milestone directory claims a *live* task as its sub-task and wedges that task's only commit boundary — `task finalize` refuses and routes at a `milestone finalize` that answers "this is a leftover, not a work unit". Four LOWs besides, one of them a vacuous acceptance cell (flow 54's `add-from-spec` fixture carries a single, degenerate criterion, so the mid-loop unwind it claims to prove is untested — I drove the uncovered shape and the behaviour is correct).

## Findings

### [MEDIUM] A pin-less milestone directory still claims a LIVE task as its sub-task — `owning_milestone` asks `is_dir()`, not the base pin, and wedges that task's only commit boundary

**Where:** crates/engine/src/milestone.rs:994 (consumers: crates/cli/src/orient.rs:177, crates/cli/src/task.rs:981 · :2411 · :2526, crates/cli/src/repo.rs:652)  
**Confidence:** high

Increment 3's deliverable is *a directory under `.jigc/tasks/` (or `.jigc/milestones/`) that holds no base pin is a task at **no** door* (roadmap → Milestone 53 Increment 3; design/storage.md:291 as rewritten by this pass). `engine::milestone::owning_milestone` is the one production reader of `.jigc/milestones/*` that was not converted: line 994 is still `entry.file_type().ok()?.is_dir()`, followed by a plain `read_task_list` — so a residual whose `tasks.json` survives is a milestone there.

Driven on the debug binary at HEAD, via `dev/jigc-rig fresh`:
```
jigc start --workflow quick-fix "Add a thing"
mkdir -p .jigc/milestones/stray-mile
printf '{"tasks":["add-a-thing"]}' > .jigc/milestones/stray-mile/tasks.json
```
`jigc start` now renders the live task as a sub-task and routes at the milestone:
> `Run: \`jigc milestone finalize stray-mile\`  — validate + commit: this is a sub-task of milestone \`stray-mile\`, whose door is its only commit boundary — \`jigc task finalize add-a-thing\` refuses here`

`jigc task finalize add-a-thing` → exit 1, `blocking · finalize.milestone-sub-task — task \`add-a-thing\` is a sub-task of milestone \`stray-mile\` …`, routed at `jigc milestone finalize stray-mile`.

`jigc milestone finalize stray-mile` → exit 1, `blocking · milestone.unknown — milestone \`stray-mile\` does not exist: \`.jigc/milestones/stray-mile\` is a directory carrying no base pin, so it is a leftover and not a work unit`.

So the pass's own new residual answer closes the second hop while the first hop still sends the agent there: a live task with authored, staged prose has **no** commit boundary it can reach. The only exits are deleting the directory by hand (named on no surface at the first hop) or `jigc task discard <id> --force`, which I drove and which succeeds — dropping the task's staged work. `jigc rename` is *not* wedged (Increment 3 fixed `first_live_milestone`), which is what makes the omission visible as a gap rather than a policy.

Reachable without hand-planting, though I did not drive the combination: `unwind_area` removes `MILESTONE_AREA_FILES` in row order with `base.json` first and `tasks.json` last, so the pass's own *fault on a later member* cell (`milestone_boundary_displacement::boundary_areas_*_fault_on_a_later_member`) leaves exactly `tasks.json`-without-`base.json`; a sub-task area whose own unwind faults *on the pin* stays live, which is Increment 2's stated declared bound.

**How the count was derived, and it is one site:** I enumerated every production reader of the two area roots — `grep -rn 'join("milestones")' crates/cli/src crates/engine/src` (5 production hits) and `grep -rn 'join("tasks")' …` (~12) — and classified each. Fixed by this pass: `list_active_task_ids`, `first_live_milestone`, the four by-id task seams, `require_milestone_area`'s eight doors. Deliberately pin-agnostic (correct — their subject is bytes, not work-unit-ness): `discard_foreign_subject`, `setup::workbench_foreign_areas`. Declared in the ledger with triggers: `reseed_sub_task_areas`, `staged_task_prose`. `owning_milestone` is the only reader that is neither fixed nor declared — but it has **5 production consumers**, listed above, so the blast radius is wider than the one function.

**Suggested fix:** 

### [LOW] Flow 54 arm 5's `add-from-spec` cell is vacuous on its two load-bearing assertions — the fixture carries one criterion and it is the degenerate one, so the mid-loop abort-and-unwind the engine's own comment claims is reached by no test in the tree

**Where:** crates/cli/tests/flow54_acceptance.rs:2135 (DEGENERATE_SPEC) — assertions at :2426 and :2431; the claim at crates/engine/src/milestone.rs:508-516  
**Confidence:** high

`DEGENERATE_SPEC` (flow54_acceptance.rs:2135-2151) declares exactly one `### 日本語  {#degenerate-criterion}` under `## Criteria`. `add_from_spec`'s loop asks `reject_unslugable_title` on iteration 1, so `added` is still empty when it aborts — which means the arm's two strongest assertions,
```
assert_eq!(git(repo.path(), &["rev-parse", "HEAD"]), head, "…before any record commit;\n{text}");
assert_eq!(area_names(repo.path()), areas, "…and with no sub-task area left behind;\n{text}");
```
are satisfied by *nothing having been minted*, not by the unwind. They would stay green with `unwind_unrecorded_seeds` deleted.

The code the arm is supposed to cover says so in as many words (crates/engine/src/milestone.rs:508): *"The abort carries `added` out with it rather than taking `From<Finding>`'s empty set: this call may already have minted earlier criteria, and the door's mid-loop unwind is what keeps the workbench naming exactly what the record names."*

I drove the uncovered shape — a spec with `[Throttle bursts, 日本語]` — through the real binary and the behaviour is **correct**: exit 1 with `write.unslugable-title`, HEAD unmoved, `.jigc/tasks` empty, `tasks.json` `{"tasks": []}`, the committed `milestone-record` carrying no item. So this is a coverage hole, not a defect.

**How the count was derived:** `grep -rn 'degenerate-criterion' crates/cli/tests` → 1 hit (flow54_acceptance.rs:2148). `grep -rln unslugable crates/cli/tests` → 6 files (flow37_rename, flow54_acceptance, milestone_envelope_arm, slug_override_axis, start_compose, work_unit_id_axis, write_finding_keys); I read each one's fixtures and none builds a multi-criterion spec. One site, derived.

**Suggested fix:** 

### [LOW] The new uncommitted-cherry-pick abandon route emits a bare `git reset`, whose qualifier under-states what it does: a mixed reset also unstages the user's unrelated pre-staged work

**Where:** crates/cli/src/repo.rs:442 (`InProgress::UncommittedCherryPick => "git reset"`) and :468 (`abandon_qualifier`)  
**Confidence:** high

Increment 4's deliverable states the route must be "true in each of those states and destroy[] nothing" (roadmap → Milestone 53 Increment 4). The rendered route, driven at every acting door under the `uncommitted-pick` rig state, is:
> `conclude it with \`git commit\` (which uses the pick's own message, once any conflicts are resolved), or abandon it with \`git reset\` (which keeps the picked changes in your working tree, unstaged), then re-run this command`

The qualifier is true about the picked bytes and silent about the rest of the index. Driven on git 2.54.0:
```
git add mywork.txt            # the user's own pre-staged work
git cherry-pick -n side
git diff --cached --name-only # → mywork.txt, picked.txt
git reset
git status --porcelain        # → ?? mywork.txt   ?? picked.txt
```
No bytes die, but the staged set does — and the staged set is state jigc treats as load-bearing everywhere else (M43's carryover gate snapshots it at every task-minting door, and `finalize.carried-staged` blocks over it per path). A route that jigc emits and that silently discards it is worth a clause.

**Bound: instance, one member.** I read all ten `InProgress::abandon()` values at repo.rs:425-444; the other nine name operation-scoped commands (`git merge --abort`, `git rebase --abort`, `git am --abort`, `git cherry-pick --abort`, `git revert --abort`, `git cherry-pick --quit`, `git bisect reset`). The one sibling with the same shape is `UnmergedIndex`'s `git reset --merge`, which predates this pass (M52) and is not this pass's act.

**Suggested fix:** 

### [LOW] The pass adds a new host-absolute path render inside `milestone_boundary_gate`, in a module `repo_relative_paths.rs` lists as swept — and the guarded fence reads `.display()` only, so it cannot see it

**Where:** crates/cli/src/milestone.rs:6048; fence at crates/cli/tests/repo_relative_paths.rs:955 (GUARDED_SRC) and :1346  
**Confidence:** high

Increment 1's shape leg added `let shape = entry.file_type().with_context(|| format!("could not read the shape of {path:?}"))?;` at crates/cli/src/milestone.rs:6048 — `{path:?}` on an absolute `PathBuf`, so the message names the host filesystem of the machine the door ran on (design/surface-contract.md law 1).

`crates/cli/src/milestone.rs` is the **first entry of `GUARDED_SRC`** (repo_relative_paths.rs:956), the list whose claim is that the module's path text is repo-relative or a declared absolute. That fence (`a_guarded_module_renders_no_host_path_outside_a_declared_absolute`, :1346) scans for `.display()` and nothing else — its own declared bound at :1340 says the `{…:?}` half is checked per-row over `PATH_TEXT_SITES`, and `milestone_boundary_gate` is not a row there. So the new site is fenced by nothing and the gate stayed green over it.

**How the count was derived:** `grep -nE '\{[a-z_]+:\?\}' crates/cli/src/milestone.rs` → 29 hits; `git diff 5d9fd714..HEAD -- crates/cli/src/milestone.rs | grep -E '^\+.*\{[a-z_]+:\?\}'` → exactly 1, the line above. So this is **one new site joining a ~28-site pre-existing class in the same file** (two of them, :6014 and :6065, are in the same function) — reported because the pass added it and because the `GUARDED_SRC` label over-claims for this file, not because the class is new.

**Suggested fix:** 

### [LOW] `engine::state::unslugable_title_finding` is `pub` with no caller outside its own module — not production, not a test

**Where:** crates/engine/src/state.rs:1607  
**Confidence:** high

`grep -rn 'unslugable_title_finding' crates/cli/src crates/engine/src crates/cli/tests` returns exactly two hits: the `pub fn` at crates/engine/src/state.rs:1607 and its single call inside `reject_unslugable_title` at :1645. Nothing outside the module reaches it — the three production guards (`crates/cli/src/start.rs:103`, `crates/cli/src/milestone.rs:662`, `crates/engine/src/milestone.rs:386` and `:512`) all go through `reject_unslugable_title`, and no test in either crate names the producer. It is therefore a widening of the engine's public API with no consumer. Harmless, and its sibling `residual_area_note`/`residual_area_route` are genuinely cross-module, so this is the only one of the pass's five new `pub` engine items in that position.

**Suggested fix:** 
