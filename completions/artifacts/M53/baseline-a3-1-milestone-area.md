Ledger fragment — row `(3, A3-1)` HIGH. Binary `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.16`; repo HEAD `155054cc`; **0 files under `crates/` changed between the review's build sha `e519e4eb` and HEAD** (`git log --oneline e519e4eb..HEAD --name-only | grep -c '^crates/'` → `0`), so the installed release is code-identical to HEAD. Every fixture built by driving the binary via `dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc` (two-step eval) + `milestone create/add-task/provision`; only the plants were written by hand. No edits, no `cargo`.

**Method note that invalidated my first three controls, recorded so the next driver does not repeat it:** in this harness `grep` is a shell function wrapping `ugrep --ignore-files`, which **honours `.gitignore`** — so every `grep -r … .jigc/…` returns "not found" whether or not the bytes exist. All loss claims below use `command grep -rlE` with a **before-control** that finds the plant.

---

## 1 · The cell, driven

**A3-1 reproduces exactly as filed.** DRIVEN.

```
setup: rig committed-singletons; milestone create "axis three probe"; add-task ×2; provision
       (cd .jigc/worktrees/first-sub  && printf 'one\n' > one.txt && git add one.txt)
       (cd .jigc/worktrees/second-sub && printf 'two\n' > two.txt && git add two.txt)
       printf 'MILESTONE-AREA-PRECIOUS\n' > .jigc/milestones/axis-three-probe/mnotes.txt
       mkdir -p .jigc/milestones/axis-three-probe/sub
       printf 'DEEP-PRECIOUS\n' > .jigc/milestones/axis-three-probe/sub/deep.txt

$ git status --porcelain --ignored .jigc/milestones/axis-three-probe/mnotes.txt
!! .jigc/milestones/                       # gitignored whole — nothing else has a copy
$ command grep -rlE 'MILESTONE-AREA-PRECIOUS|DEEP-PRECIOUS' .        # BEFORE-control
./.jigc/milestones/axis-three-probe/sub/deep.txt
./.jigc/milestones/axis-three-probe/mnotes.txt

$ jigc milestone finalize axis-three-probe
exit=0
stdout: finalized d17df0a — Finalize milestone axis-three-probe (2 sub-tasks)
          modified docs/milestone-records/axis-three-probe.md
          added one.txt
          added two.txt
          3 files committed
          sub-tasks: first-sub: 1 code file · second-sub: 1 code file
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
stderr: 0 bytes                            # measured: wc -c < err == 0

$ command grep -rlE 'MILESTONE-AREA-PRECIOUS|DEEP-PRECIOUS' .    ; rc=1   (nothing on disk)
$ git grep -l 'MILESTONE-AREA-PRECIOUS' $(git rev-list --all)    ; (no object carries it)
$ find .jigc/milestones   ->  .jigc/milestones                   # the whole area is gone
$ find .jigc/displaced    ->  No such file or directory
```

Same plant, `--format json` (separate rig, exit 0):

```json
"committed": { "commits":[{"hash":"e6e595c","paths":["docs/milestone-records/axis-three-probe.md","one.txt","two.txt"],
                "subject":"Finalize milestone axis-three-probe (2 sub-tasks)"}],
               "displaced": [], "files":3, "hook_output":"", … }
```

**`squash: false` is the same defect, driven separately** (knob set + committed *before* `milestone create`, else the base pin mismatches): exit 0, three commits (`feat: land first-sub` · `feat: land second-sub` · the aggregate), `displaced` carried **only** the sub-task pair `.jigc/tasks/second-sub/tnotes.txt → .jigc/displaced/second-sub/tnotes.txt`, `mnotes.txt` + `sub/deep.txt` gone, `find .jigc/milestones` → `.jigc/milestones`. DRIVEN.

## 2 · The three sibling doors, identical plant

| door | exit | identity | what it says about `…/axis-three-probe/mnotes.txt` |
|---|---|---|---|
| `jigc milestone discard axis-three-probe` (clean worktrees) | 1 | `milestone.foreign-bytes` | names the path; route = move/delete then re-run, or `--force` |
| `jigc uninstall` (clean worktrees) | 1 | `uninstall.foreign-bytes` | names the same path; route names `--force` |
| `jigc milestone discard … --force` | 0 | — | `warning: removing the working area … discards work that is not in git:` + `mnotes.txt`, `sub`, + `not recoverable` |
| **`jigc milestone finalize axis-three-probe`** | **0** | **none** | **nothing, on either stream** |

(With the review's *staged-worktree* plant the first two refuse on `milestone.dirty-worktree`/`uninstall.dirty-worktree` first — a precedence detail, not a disagreement; the review's transcript reached the `foreign-bytes` arm. Both arms DRIVEN.)

## 3 · Code map (READ, each verified at HEAD)

| thing | file:line | fact |
|---|---|---|
| the removal | `/Users/maurice/projects/gherrink-jigc/crates/cli/src/task.rs:5981` | `if let Err(err) = std::fs::remove_dir_all(cleanup_dir)` inside `fn post_commit` (`:5956`), phase 7 |
| the displacement it skips | `crates/cli/src/task.rs:5971-5979` | `if let Some((task_id, sink)) = displace { *sink = displace_foreign_area(repo_root, jigc_root, cleanup_dir, state::WorkArea::Task, task_id); narrate_displacement(sink); }` — **`WorkArea::Task` is hard-coded here** |
| the executor param | `crates/cli/src/task.rs:3845-3857` | `try_execute_finalize_plan(… cleanup_dir: &Path, … displace: Option<(&str, &mut Vec<render::Displaced>)>)` |
| milestone call site, `squash:false` | `crates/cli/src/milestone.rs:5081` … `:5095-5098` | `cleanup_dir = &dir` (the milestone area) and `displace = None`, with the comment *"the executor's `cleanup_dir` is the MILESTONE area, a different registry row … this door must not answer for a subject it was not given"* |
| milestone call site, `squash:true` | `crates/cli/src/milestone.rs:5214` … `:5225-5228` | byte-identical `None` + the same comment |
| the shipped primitive | `crates/cli/src/task.rs:5849-5897` | `pub(crate) fn displace_foreign_area(repo_root, jigc_root, area: &Path, kind: state::WorkArea, unit_id: &str) -> Vec<render::Displaced>`; destination `jigc_root.join(relocate::WORKBENCH_SUBDIR).join(unit_id)` = `.jigc/displaced/<unit_id>/`; per-entry `free_displacement_path` no-clobber (`:5902`); narration `narrate_displacement` (`:5924`) |
| its two callers today | `crates/cli/src/task.rs:5972` (task post-commit) · `crates/cli/src/milestone.rs:5528` (`cleanup_subtask_areas`, `WorkArea::Task`, `&sub_id`) | **no caller passes `WorkArea::Milestone`** |
| writer registries | `crates/engine/src/state.rs:129` `TASK_AREA_FILES` (13) · `crates/engine/src/state.rs:168` **`MILESTONE_AREA_FILES`** (5: `BASE_PIN_FILE`, `STAGED_SNAPSHOT_FILE`, `milestone::MERGED_AREA`, `milestone::RECORD_COMMIT_MSG_FILE`, `milestone::TASKS_FILE`) | `pub`, production, not test-only; `WorkArea::{Task,Milestone}` at `:178-192`; count-fenced through the **union** by `/Users/maurice/projects/gherrink-jigc/crates/cli/tests/task_area_writer_registry.rs:227-235` |
| the complement probe | `crates/engine/src/state.rs:262-301` `pub fn foreign_area_paths(area, kind)` | already `kind`-generic; **top-level entries only** (plus a one-level `docs/` walk for `WorkArea::Task`); fail-closed on unreadable; a foreign directory is returned whole |
| the safe non-recursive sibling | `crates/engine/src/state.rs:385` `unwind_area(area, kind)` | already used with `WorkArea::Milestone` by `unwind_mint` (`crates/cli/src/milestone.rs:1063`) |
| the `displaced` key | `crates/cli/src/render.rs:4836-4848` (`MilestoneLanded::displaced`) · `:1925-1936` (`Landed::displaced`) · `Displaced{from,to}` `:4846` | doc-comment scopes it to *"a **sub-task's** working area"*; "present always, `[]` on the ordinary path", sorted by `from`; populated at `crates/cli/src/milestone.rs:5111-5118` (`cleanup_subtask_areas` → `displaced`) and handed to `milestone_landed_summary` (`:6550-6557`) |
| envelope pin | `crates/cli/src/render.rs:6721-6731` | `milestone finalize` **Landed** arm `ArmShape::Object(&["committed"])`, `ArmStatus::Pinned` — `displaced` is *inside* `committed`, so widening its **values** moves no pinned key |
| the door rows | `crates/cli/src/milestone.rs:3263` `FINALIZE_DOOR{verb:"jigc milestone finalize", disposition: Disposition::Displace, codes: &[]}` · `:3245` `TASK_FINALIZE_DOOR{…Displace, codes:&[]}` · `:3275` `DESTROYING_DOORS: [&DestroyingDoor; 6]` | `Disposition::Displace` = *"**Keep it**: move it aside with its relative path preserved, and name where it went"* (`:3082`); the registry doc-comment's rule *"Every door **answers for what it removes**"* is at `:3097` |

## 4 · The halt question — **no new mechanism is needed** for the charter's prescribed fix

Everything the charter names already exists and is already parameterized. Concretely:

| would the fix need… | answer | evidence |
|---|---|---|
| a new registry row | **No.** `MILESTONE_AREA_FILES` exists (`crates/engine/src/state.rs:168`), is `pub`, production, and is already the subject `milestone discard` uses | DRIVEN: `discard --force` over a *post-join* area named exactly `mnotes.txt` and **not** `merged/` (below) |
| a new primitive | **No.** `displace_foreign_area` already takes `kind: state::WorkArea` and `unit_id: &str` (`task.rs:5849-5854`) | READ |
| a new disposition / finding code | **No.** `FINALIZE_DOOR` is already `Disposition::Displace` with `codes: &[]`; the charter's fix is that door doing what its row says | READ |
| a new envelope key | **No.** `committed.displaced` is declared, pinned as a value inside `Object(&["committed"])`, and "present always" | READ + DRIVEN (`displaced: []` observed) |
| a new destination, or a collision risk | **No new destination.** `.jigc/displaced/<milestone-id>/` is what the primitive computes from `unit_id`. **A milestone id and a task id CAN collide** — driven: `jigc milestone create "shared name"` + `jigc start --workflow quick-fix "shared name"` both succeeded, yielding `.jigc/milestones/shared-name` **and** `.jigc/tasks/shared-name` — so both would park under one `.jigc/displaced/shared-name/`. This is **not loss**: `free_displacement_path` suffixes, driven across two runs of one id (`notes.txt` = `FIRST`, `notes.txt.2` = `SECOND`, both intact). It is a provenance-ambiguity the fixer should state, not a mechanism | DRIVEN |
| a signature change | **Yes — one, on an existing parameter.** `post_commit`'s `displace: Option<(&str, &mut Vec<Displaced>)>` (`task.rs:5956-5963`) and `try_execute_finalize_plan`'s identical param (`task.rs:3856`) must carry the **kind** too (e.g. `Option<(state::WorkArea, &str, &mut Vec<…>)>`), because `WorkArea::Task` is hard-coded at `task.rs:5976`. Then `milestone.rs:5098` and `:5228` pass `Some((WorkArea::Milestone, milestone_id, &mut sink))` instead of `None`. That is a parameter widening inside a shipped seam, not a new mechanism | READ |
| ordering work | **No.** The correct point is exactly where the task door already does it — inside `post_commit`, after the commit, before `remove_dir_all(cleanup_dir)` | DRIVEN (§6) |

**Two facts that bound the fix's completeness, both driven — neither blocks it, both belong in the fixer's brief:**

- **`merged/` is jigc's wholesale and is never walked.** `MILESTONE_AREA_FILES` carries `MERGED_AREA` as a tree member and `foreign_area_paths` does not descend into it (`crates/engine/src/state.rs:275-297`; the rule is stated at `/Users/maurice/projects/gherrink-jigc/design/team-ready-state.md:92`). So the charter's fix **keeps** `mnotes.txt` and `sub/` and still **destroys** anything under `.jigc/milestones/<id>/merged/`. DRIVEN both ways (§5 rows 3 and 4).
- **The complement probe is already correct over the real landing shape.** Over a post-join area (`base.json`, `merged/`, `record-commit-msg.txt`, `staged-snapshot.json`, `tasks.json` + one plant), `milestone discard --force` narrated exactly one path — the plant — so feeding the same probe to the finalize door yields `displaced: []` on the ordinary boundary and does not move jigc's own staging tree. DRIVEN.

## 5 · The class's sibling cells — every production remover of a `.jigc/` working area

Test-module sites (`#[cfg(test)]` boundaries: `task.rs:7314`, `milestone.rs:6829`, `setup.rs:3900`, `engine/milestone.rs:2517`, `engine/state.rs:2102`, `combine.rs:299`) are excluded; the `let _ = remove_dir_all(&self.0)` swarm is all `TempDir::drop` in tests.

| # | door · argv driven | site | area removed | complement answered? | verdict |
|---|---|---|---|---|---|
| 1 | **`jigc milestone finalize <id>`** (`squash:true` and `squash:false`) | `task.rs:5981` via `milestone.rs:5098`/`:5228` (`displace: None`) | `.jigc/milestones/<id>/` | **no** — silent, exit 0, `displaced: []` | **UNSAFE** (the row) |
| 2 | same argv, sub-task areas | `milestone.rs:5538` (`SubtaskComplement::Displace`) | `.jigc/tasks/<sub>/` | yes — moved + narrated on stderr + on `displaced` | **SAFE** |
| 3 | same argv, **foreign byte inside `merged/`** | subsumed by row 1 | `…/merged/foreign-top.txt` | no | **UNSAFE** — and *not fixed by the charter's fix* (tree member) |
| 4 | **`jigc milestone finalize` that does NOT land (exit 3)** | `crates/engine/src/milestone.rs:1890-1893` `materialize`'s `remove_dir_all(merged/docs)` | `.jigc/milestones/<id>/merged/docs/` | no | **UNSAFE — new cell, in no review row** (below) |
| 5 | `jigc task finalize <id>` | `task.rs:5981` with `displace: Some(...)` | `.jigc/tasks/<id>/` | yes — `deep` + `notes.txt` both moved, both on `displaced` and on stderr | **SAFE** |
| 6 | `jigc task discard <id>` | `task.rs:618` | `.jigc/tasks/<id>/` | yes — `task-discard.foreign-bytes` exit 1 naming `deep` and `notes.txt` | **SAFE** |
| 7 | `jigc milestone discard <id>` | `milestone.rs:4465` `remove_milestone_area` + `milestone.rs:5538` | both areas | yes — `milestone.foreign-bytes` exit 1 / `--force` narrates each path | **SAFE** |
| 8 | `jigc uninstall` | `setup.rs:2785` | `.jigc/` whole | yes — `uninstall.foreign-bytes` exit 1 naming the milestone-area plant | **SAFE** |
| 9 | `jigc milestone provision <id>` (re-provision over a milestone-area plant) | `milestone.rs:2920-2925` (worktree paths only) | `.jigc/worktrees/<sub>` | n/a — **does not touch the milestone area**; plant survived, exit 0 | **SAFE** |
| 10 | `jigc milestone join <id>` | — | nothing | join is read-only: plants in `merged/` and `merged/docs/` **survived** a second `join` at exit 0 | **SAFE** |
| 11 | mint rollback at `milestone create` / `add-task` (rejecting hook) | `engine/state.rs:385-454` `unwind_area` → registry row + non-recursive `remove_dir` | the just-minted area | by construction (removes only the registry row) | **SAFE (READ)** — not driven here |
| 12 | `DedicatedWorktree::add` stale-leftover clear (`task.rs:6974`), `TempIndex`/`CombineIndex` (`combine.rs:284`, `task.rs:7042`) | — | `.jigc/worktrees/.combine-<pid>-<nanos>`, `$TMPDIR/jigc-combine-*` | n/a — process-unique names jigc mints | **SAFE (READ)** |

**Row 4 transcript — the new cell (DRIVEN).** A foreign byte inside the milestone area's `merged/docs/` dies on a path that **commits nothing**:

```
setup: milestone rig; (in first-sub's worktree) workflow sub-task --task first-sub;
       jigc doc create adr --title "an undecided thing" --task first-sub    # unfilled slots
       mkdir -p .jigc/milestones/axis-three-probe/merged/docs
       printf 'IN-MERGED-DOCS\n' > …/merged/docs/foreign.txt
       printf 'IN-MERGED-TOP\n'  > …/merged/foreign-top.txt
$ command grep -rlE 'IN-MERGED-DOCS|IN-MERGED-TOP' .       # BEFORE-control: both present
$ jigc milestone finalize axis-three-probe
exit=3   (blocking · schema-conformance.required-slot-present — nothing committed)
$ command grep -rlE 'IN-MERGED-DOCS|IN-MERGED-TOP' .
./.jigc/milestones/axis-three-probe/merged/foreign-top.txt      # …/merged/docs/foreign.txt is GONE
$ find .jigc/milestones/axis-three-probe -maxdepth 2            # the area itself survives
  …/merged/docs   …/merged/foreign-top.txt   …/base.json   …/tasks.json   …
```

Producer: `crates/engine/src/milestone.rs:1890-1893`, *"The parent staging `docs/` — a clean rebuild on each materialize"*, an unconditional `remove_dir_all(docs_dir)` with no complement probe in front of it. It is the same class one layer in, it fires at **exit 3** (so the landed-boundary warrant is not even available), and it is outside both the charter's prescribed fix and `MILESTONE_AREA_FILES`' walk rule.

**Row 3 vs. the fix, stated plainly:** after the charter's fix, `.jigc/milestones/<id>/merged/**` remains a silent-loss region at both exits. Whether that is acceptable is the *declared bound* at `design/team-ready-state.md:92` (*"the join's staging area, jigc's wholesale, never walked"*) — but that bound was written for a tree jigc rebuilds, not for a tree a hook or an operator can write into between a blocked finalize and the next one. This is the sibling cell to hand the human with the fix, not a scope decision I make.

## 6 · Ordering — displacement + removal run **after** the commit, at both doors

READ: phase 7 (`crates/cli/src/task.rs:4106-4118` calls `post_commit` only on the `Ok` arm of the commit, after `mark_commit_failure` returns early at `:4104`); the milestone door's `cleanup_subtask_areas` is inside `Ok(hook_output) => {` (`crates/cli/src/milestone.rs:5100-5118` and `:5229-…`).

DRIVEN corroboration with a rejecting `.git/hooks/pre-commit` (`exit 1`):

```
plants: .jigc/milestones/axis-three-probe/hp.txt   and   .jigc/tasks/first-sub/hp.txt
$ jigc milestone finalize axis-three-probe
exit=1
stderr: `git commit` was rejected (no commit was made): …
        milestone:axis-three-probe is intact — nothing was committed, the merged docs were
        rolled back, and every provisioned sub-task worktree still holds its staged code.
$ command grep -rlE 'HOOK-REJECT-PLANT|SUBTASK-HOOK-PLANT' .
./.jigc/tasks/first-sub/hp.txt
./.jigc/milestones/axis-three-probe/hp.txt          # both survive => removal is POST-commit
```

Same for the blocked arms: exit 3 (required slot) and exit 1 (`finalize.render-io`) both left the milestone area and its plant on disk. So a fix placed inside `post_commit` inherits the correct interval and cannot strand a retry.

## 7 · Existing pins and doc homes the fix must keep true

| artifact | what it iterates / states |
|---|---|
| `/Users/maurice/projects/gherrink-jigc/crates/cli/tests/milestone_boundary_displacement.rs` | M52 Inc 4/T4. Iterates the **landed-arm axis** (`squash:true` × `squash:false`) × output channel, plants `NOTES.md` + `analysis/perf.txt` in **every sub-task area** (`SUBS = ["area-low","area-zed"]`); the omitting cell asserts `displaced: []` and an empty `.jigc/displaced`. **The string `milestones` appears nowhere in the suite** — A3-3: its empty-key assertion is true of a tree it never looks at, so A3-1 is green under it. |
| `/Users/maurice/projects/gherrink-jigc/crates/cli/tests/flow53_acceptance.rs` arm 3 | Iterates `DESTROYING_DOORS` through `Disposition`. `displace_plan_for` (`:1535-1553`) gives `jigc milestone finalize` `unit: "area-low"`, and the arm binds `area = repo/.jigc/tasks/<plan.unit>` (`:1605`) — **one area per door**, so the boundary's own area is outside the arm. `plan_for`/`displace_plan_for` both `panic!` on an unplanned door, so the registry stays the subject. |
| `/Users/maurice/projects/gherrink-jigc/crates/cli/tests/finalize_displacement.rs` | The task door's three cells (`…survive_the_landed_finalize_and_the_envelope_names_each_move`, `…names_the_same_moves_on_stderr`, `a_task_with_no_foreign_byte_displaces_nothing_and_says_nothing`). |
| `/Users/maurice/projects/gherrink-jigc/crates/cli/tests/task_area_writer_registry.rs:227-235` | Counts production `<dir>.join(<name>)` sites against the **union** of `TASK_AREA_FILES ∪ TASK_DOCS_FILES ∪ MILESTONE_AREA_FILES` — a member on the wrong row is invisible to it (its own doc-comment says so at `engine/state.rs:113-122`). |
| `/Users/maurice/projects/gherrink-jigc/design/team-ready-state.md:84-98` | The two populations; *"The complement is the subject of every door that destroys a working area … what no door may do is take it silently"*; `:92` carries the `merged/`-never-walked rule. |
| `/Users/maurice/projects/gherrink-jigc/design/storage.md:291` | *"The teardown keeps what jigc did not write (M52)"* — written for `.jigc/tasks/<task-id>/` only; `:125`/`:131` declare `.jigc/displaced/` as the one gitignored subdir nothing can reconstruct. |
| `/Users/maurice/projects/gherrink-jigc/design/finalize.md:157` | Phase-7 bullet — *"Keep what jigc did not write, then remove `.jigc/tasks/<task-id>/`"*; the milestone area is not named. |
| `/Users/maurice/projects/gherrink-jigc/design/command-output-contract.md:193` (milestone arm), `:521-527` (the `displaced` declaration) | `displaced` = `[{from,to}]`, repo-relative, sorted by `from`, **present always**; the unit is the complement's **entry**, a foreign directory rides one pair. Widening the union satisfies this text unchanged; the two render doc-comments (`render.rs:4836`, `:1933`) scope it to a *sub-task's* area and would become false. |
| `/Users/maurice/projects/gherrink-jigc/crates/cli/src/milestone.rs:3097` | `DESTROYING_DOORS`' rule — *"Every door answers for what it removes … a door that destroys what it never named is the law-1 half-truth"*. This is the sentence the binary violates. |

**Claim ledger:** §1, §2, §5 rows 1–10, §4's collision and no-clobber rows, and §6's corroboration are **DRIVEN**. §3, §5 rows 11–12, §4's signature row, and §7 are **READ**.
