# Fix-pass plan — family: worktree registrations (the repository-wide `git worktree prune`)

Row: **L-22** (trial, tier 1 by the predicate, upheld by the adversarial re-drive).
Planned against the tree whose product code equals the published `jigc 1.0.0-rc.24`; every "driven"
below is the installed registry binary (`jigc --version` → `jigc 1.0.0-rc.24`) on a fresh
`dev/jigc-rig fresh` rig, host git 2.54.0. Nothing in the repository was edited.

## 1 · The contract

> **A jigc door removes a git worktree registration only at a path it created under
> `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>` or its own `.combine-*` — so every
> other registration of the repository (live, `prunable` or locked) is byte-identical under
> `.git/worktrees/` after any run of `milestone provision`, `milestone finalize`,
> `milestone discard` or `uninstall`; a run that refuses leaves `.git/worktrees/` exactly as it
> found it; and `uninstall` says it dropped registrations only when it dropped one of its own.**

Where it comes from:

- `design/storage.md` → Repository layout, the `worktrees/<sub-task-id>/` row: *"No door removes
  such a path silently: each probes it first and then either refuses what it cannot prove is
  disposable … or names every byte it is about to destroy"* — the doors' declared reach is that
  path set; the prune reaches every registration, probes nothing, names nothing.
- `DECISIONS.md` → M31 planning (2026-06-20/21): the prune is declared as *"drops admin records for
  dirs a crash deleted"* — jigc's own crashed-run records — and the teardown prune as a *"harmless
  no-op"*. A foreign record was never in its declared subject.
- `DECISIONS.md` → 2026-09-23, the confirmation pass: *"It prunes now … and says so: a destroying
  door narrating what it changed in the repository is law 1."* `uninstall`'s sentence attributes
  the prune to *"the fan-out worktrees `.jigc/` held"*.

No design doc, guide or `--help` states the repository-wide prune. The fix restores a reach the
product already states; it adds no capability.

## 2 · What was driven for this plan (beyond the verification file)

| cell | today (rc.24) | consequence for the plan |
|---|---|---|
| foreign prunable + `milestone provision` | exit 0, one `provisioned …` line, `.git/worktrees/y` gone | the row, re-confirmed |
| foreign prunable only, no fan-out + `uninstall` | exit 0, record gone, ack prints *"pruned git's worktree registrations for the fan-out worktrees `.jigc/` held"* | the misattribution, re-confirmed |
| own sub-task worktree, directory deleted, `provision` re-run | exit 0, record pruned, fresh worktree added | the self-heal the fix must keep (control) |
| own sub-task worktree whose `.git` file is gone, directory holds content, `provision` | **exit 1 `milestone.leftover-holds-work`, "nothing was removed" — and the record is gone** | a *refusing* provision mutates `.git/worktrees/`; a second refusing-run cell beside the verifier's refusing finalize |
| same, `provision --force` | narrated, cleared, re-added live | must keep working with the keyed removal placed *after* the clear |
| same state, `discard --force` | the A2 warning (`validation failed … .git does not exist`) with remedy *"run `git -C … worktree prune`, then `git -C … worktree remove --force <path>`"*, then the bare prune drops the record | the remedy names the repository-wide prune; after it the second command fails (`is not a working tree`) |
| same state, landed `finalize` | exit 0, **no warning** — the combine worktree's prune dropped the record before the teardown looked | after the fix this cell reaches the A2 warning (record still registered, removal fails); exit code unchanged |
| **repository moved with `mv`** after a provision with staged sub-task code, then `provision` | exit 1 `milestone.leftover-holds-work`, "nothing was removed" — **and both of jigc's own registrations (they name the old path) are gone** | the mirror the verifier named: the bare prune also destroys jigc's own records, through a refusing run. Control with plain git: without the prune, `git worktree repair <new path>` exits 0 and the staged file is still staged (`A  s.txt`) |
| own sub-task worktree with a commit made inside it, directory deleted, `provision` | exit 0, silent; `fsck --unreachable` hits for that commit 0 → 1 | an **own-set** sibling the keyed fix does not close — fork `worktree-1` |
| same, `discard` | exit 0, `workbench removed`, silent; 0 → 1 | same |
| own sub-task worktree **locked**, directory deleted, `provision` | exit 0, `provisioned 2 worktree(s)` — one of the two is not on disk | a pre-existing false success; see row P below |
| plain git: `worktree remove --force <path>` on a registration whose directory is absent | exit 0, that one record gone, a prunable sibling stays | the keyed primitive (matches the verifier, 2.54.0 and 2.47.3) |
| plain git: same, directory **present** but its `.git` file missing | **exit 128** `validation failed, cannot remove working tree` | the keyed removal must run **after** the leftover is cleared |
| plain git: same, registration locked | exit 128 `cannot remove a locked working tree` | same outcome as the bare prune (which skips locked) |
| plain git: `worktree add` at a new path while a stale record with the same basename exists | exit 0, admin dir suffixed (`alpha1`) | a stale old-home record never blocks a provision |

## 3 · The row

### L-22 · sites

| # | site | door |
|---|---|---|
| 1 | `crates/cli/src/milestone.rs` · `provision_worktrees` — the bare prune ahead of the registered-set read | `milestone provision` (landing and refusing) |
| 2 | `crates/cli/src/milestone.rs` · `remove_worktrees` — the trailing unconditional prune | `milestone finalize` (both landed arms), `milestone discard` |
| 3, 4 | `crates/cli/src/task.rs` · `DedicatedWorktree::add` and `Drop for DedicatedWorktree` | `milestone finalize` — `chain_commit`, `commit_combined_tree_with_hooks`, `checkout_tree_worktree` (the boundary gate; reached by refusing runs) |
| 5 | `crates/cli/src/setup.rs` · `prune_worktree_admin` (called from `uninstall`), and the ack line in `crates/cli/src/render.rs` keyed on `UninstallSummary::pruned_worktrees` | `uninstall` |
| 6 | `crates/cli/src/milestone.rs` · `remove_worktrees` — the A2 warning's remedy span `git_at(jigc_home, "worktree prune")` | a printed route, at `finalize`/`discard` |

### Narrow fix — behaviour

One rule, applied at every site: **no `git worktree prune` at all; a registration is removed with
`git worktree remove --force <path>`, and only for a path the door itself owns.** No new finding
code, no new flag, no new store, no new registry.

- **Site 1 — provision.** The prune goes. A sub-task path counts as *reuse* only when it is
  registered here **and** its `.git` link is on disk (git's own `prunable` test is exactly "the
  gitdir file points to a non-existent location"). A registered-but-stale own path is treated as
  what today's prune turned it into — not reused — so phase 1 probes whatever stands there and
  refuses under the existing `milestone.leftover-holds-work` with nothing moved. In phase 2, after
  the existing leftover clear and before the existing `git worktree add`, the stale own record is
  removed by path. Failure of that removal or of the add surfaces through the existing
  `milestone.provision-failed` block and its existing `provision_route`.
  Net: the owned crash-recovery self-heal is unchanged; a refused provision no longer touches
  `.git/worktrees/`; no registration outside this milestone's sub-task paths is ever touched.
- **Site 2 — teardown.** The trailing prune goes; nothing replaces it. The loop already removes
  every registered own path by path, and that same command succeeds on a registered own path whose
  directory is already gone (driven). A never-provisioned discard therefore runs no git mutation.
- **Sites 3, 4 — the combine worktree.** Both prunes go. `add` names a path unique per process and
  instant, so no stale record can sit at it; `Drop` already removes its own path by path, which
  also succeeds when the directory is gone. A refusing finalize then nets zero change to
  `.git/worktrees/`.
- **Site 5 — uninstall.** After `.jigc/` is removed, every registration whose recorded path lies
  under `<jigc_home>/.jigc/` is removed by path, best-effort exactly as the prune was.
  `pruned_worktrees` is true iff at least one such removal succeeded, so the existing ack sentence
  — unchanged — is printed only when it is true. The field stays text-only.
- **Site 6 — the remedy.** The warning keeps its shape and its aimed `git -C <home> worktree remove
  --force <path>` span; the `worktree prune` span goes, and the sentence says to deal with what
  git's quoted error names first. Reason it is in the pass: after the fix the warning is reachable
  in one more cell (landed finalize over a registered-but-unlinked sub-task path), and a remedy
  that sends the reader to the repository-wide prune walks them into the row's harm by their own
  hand. It is a separate, strike-able commit.

**Row P — one cell that changes exit code, stated rather than assumed.** A jigc sub-task worktree
that someone `git worktree lock`ed and whose directory is then deleted: today `provision` exits 0
and prints `provisioned N worktree(s)` although that worktree is not on disk (driven). With the fix
the stale record is not reused, the keyed removal and the add both fail on the lock, and the door
answers `milestone.provision-failed` carrying git's own sentence (*"missing but locked worktree; use
… 'unlock' …"*). This is a previously-exit-0 command that now refuses — but what it previously
reported was false, no jigc flow or guide produces the state, and the block uses a code and route
the door already has. It is listed here and in the next-step drives, not raised as a fork; if the
orchestrator reads ruling 5 by the letter, the behaviour-preserving alternative is "when the keyed
removal fails and the directory is absent, keep today's reuse", at the price of code that returns a
path it knows is not there.

### Class — closed

- A **foreign** prunable registration at all four doors, landing and refusing, both finalize commit
  arms, the never-provisioned finalize and discard — closed by the key.
- **jigc's own registrations under another path** — a moved repository, or the same repository
  seen at another mount (the container mirror): no longer destroyed, so `git worktree repair`
  still works. Closed by the same key (they are not under the current `jigc_home`).
- **A refusing run mutating `.git/worktrees/`** — the refusing finalize (verifier) and the refusing
  provision (driven here). Closed: provision's removal moves into phase 2; the combine worktree
  touches only its own path.
- **`uninstall`'s misattributed sentence** — closed: printed iff an owned registration was dropped.
- **The remedy that recommends the repository-wide prune** — closed (site 6).
- **The class as a fence** — a source scan that no production line runs or recommends a bare
  `worktree prune` (§4, test 9), so a sixth site cannot land unseen.

### Class — left (each with its reason; homes in §9)

- jigc's **own** stale registration (directory already gone) still has its HEAD, reflog and index
  dropped — **fork `worktree-1`**.
- Stale own records at a **previous home path** linger as `prunable` (git's own gc prunes them at
  three months): jigc cannot prove a record outside `jigc_home` is its own, which is the whole fix.
- A stale `.combine-*` record whose directory was hand-deleted after a crashed finalize lingers the
  same way; `uninstall` takes it. A crashed finalize's **live** `.combine-*` worktree is cleaned by
  no milestone door — pre-existing, not a prune matter.
- The leftover refusal's route does not name `git worktree repair` for a moved repository, though
  after this fix it works — new route text, out of a fix pass.
- `milestone.dirty-worktree`'s per-path line *"registered here, so the teardown removes it and this
  content is destroyed"* is false for a registered-but-unlinked directory (driven: the removal
  fails, the content stays) — pre-existing surface, tier 3.
- The verifier's *not covered*: an in-progress operation or `config.worktree` in the foreign
  worktree (both live in the admin directory the fix no longer touches — covered by construction,
  not by a cell).

## 4 · Red tests (written first)

New suite **`crates/cli/tests/worktree_registration_reach.rs`**, registered in exactly one group
root: `crates/cli/tests/groups/g_milestone.rs`. Fixture on the mold of
`leftover_operation_in_progress.rs` (a repo, a milestone, two sub-tasks, the real binary). The
foreign plant is the verifier's: worktree `y` detached, holding a commit only its HEAD reaches and
a staged-only file; worktree `z` on a branch; both directories moved away **after** any provision
the cell needs. Every cell carries a before-control (`git worktree list --porcelain` names both
`prunable`; the admin directories exist; the commit is reachable) and asserts on what git left
(the admin directory's entries and the bytes of `HEAD` and `index`, and reachability), never on
jigc's words alone.

| # | test | asserts | red today |
|---|---|---|---|
| 1 | `provision_leaves_a_foreign_prunable_registration` | first provision and re-run: exit 0, both admin dirs byte-identical, commit reachable; directories moved back → `git status` exit 0 in both | yes |
| 2 | `finalize_leaves_a_foreign_prunable_registration_on_both_commit_arms` | the squash combine arm and the per-sub-task chain arm: exit 0, the boundary lands, records intact, no `.combine-*` registered or on disk | yes |
| 3 | `discard_leaves_a_foreign_prunable_registration` | provisioned and never-provisioned: exit 0, records intact | yes |
| 4 | `uninstall_drops_only_the_registrations_it_took_and_says_so_only_then` | (a) fan-out worktrees + foreign: own records gone, foreign intact, ack line present, `--format json` carries no new key; (b) foreign only: foreign intact, **ack line absent** | yes (both) |
| 5 | `a_refusing_run_leaves_git_worktree_admin_as_it_found_it` | never-provisioned `finalize` (exit 3 `milestone.zero-contribution`); `provision` refused by `milestone.leftover-holds-work` over a registered-but-unlinked own path that holds a file — in both, the `.git/worktrees/` listing is identical before and after, own and foreign; refused `discard` as the green control | yes (first two) |
| 6 | `provision_still_heals_its_own_stale_registration` | directory deleted → re-run exit 0, live at the base pin, no own `prunable` entry, the foreign sibling intact; `.git` file removed + empty dir → cleared and re-added; + content → refuses, then `--force` narrates, re-adds live | control (green today except the foreign sibling) |
| 7 | `a_moved_repository_keeps_the_registrations_repair_needs` | provision, stage code in a sub-task worktree, `mv` the repo; `provision` there refuses as today **and the records survive**; `git worktree repair <paths>` exit 0, the file still staged; `provision` exit 0 (reuse); `finalize` lands that file | yes |
| 8 | `the_leaked_worktree_remedy_names_one_registration_and_works` | locked sub-task worktree and registered-but-unlinked sub-task path, at `finalize` and `discard --force`: exit code unchanged, stderr carries the aimed `worktree remove --force <path>` and **no** `worktree prune`; after the obvious repair (`git worktree unlock` / moving the directory aside) the printed span, pasted verbatim, exits 0 and the record is gone; nothing is narrated as lost that is still on disk | yes |
| 9 | `no_production_source_runs_or_recommends_a_bare_worktree_prune` | over `crates/cli/src` and `crates/engine/src`, comment lines and `mod tests` dropped (`support::rust_source`): no argv carrying `"worktree"` + `"prune"`, no `git_at(…, "worktree prune")` | yes (five argv sites, one route) |
| 10 | `every_worktree_door_leaves_foreign_registrations` | the **door axis**, read code-side: `cli::milestone::WORKTREE_DOORS` × {detached-with-commit, on-a-branch, locked-and-absent}, so a fifth worktree door lands in the cell matrix rather than being remembered | yes |
| 11 | `provision_over_a_locked_directoryless_own_registration_says_so` (row P) | exit non-zero `milestone.provision-failed`, git's lock sentence quoted, route is the provision re-run; after `git worktree unlock` the re-run exits 0 | yes (exits 0 today) |

Existing tests that move:

- `crates/cli/tests/milestone.rs` · `milestone_finalize_warns_on_a_leaked_worktree_but_still_succeeds`
  — its `stderr.contains(" worktree prune")` assertion flips to the keyed remedy (site 6).
- `crates/cli/tests/git_span_aim.rs` · `GIT_SPAN_SITES`' `remove_worktrees` row and the census
  comment above it (two spans → one; 18 → 17), and `crates/cli/tests/repo_relative_paths.rs` · the
  `remove_worktrees` disposition's reason text. Prose in a fixture table, no assertion moves.
- `crates/cli/tests/cwd_verb_subject.rs` (uninstall), `text_json_parity_axis.rs`
  (`pruned_worktrees`), `milestone_boundary_gate.rs` (no leaked `.combine-*`) stay green unchanged —
  the fixer confirms rather than assumes.

**The test that should have caught it:** `cwd_verb_subject.rs`' uninstall cell asserts *"git's
worktree admin must be clean after the teardown"* and *"the live linked worktree must survive the
prune"* over a fixture whose only foreign worktree is **live** — a state no prune can touch — so a
repository-wide prune and an owned-set removal pass it identically. Every other foreign planting in
the suites (`flow48_acceptance.rs`, `leftover_operation_in_progress.rs`, `repo_posture.rs`,
`worktree_jigc_home.rs`) is live too, and M31 recorded the teardown prune as a *"harmless no-op"*.

## 5 · Next-step drives (the fixer runs these on the built binary)

1. **Foreign worktree comes back.** After each of the four doors over a prunable foreign worktree,
   put its directory back: `git -C <it> status` exit 0 and `git worktree repair` exit 0.
2. **Moved repository.** `mv` a provisioned repo holding staged sub-task code; `milestone provision`
   refuses; then `git worktree repair <each path>`, `milestone provision` (reuse), `milestone
   finalize` — the staged code lands.
3. **The remedy, pasted as printed** — locked cell (after `git worktree unlock`) and unlinked cell
   (after moving the directory aside), from a cwd outside the repository.
4. **Landed finalize over a registered-but-unlinked sub-task path** — the cell that newly reaches
   the warning: exit 0, the directory and its bytes still on disk, the ack and the loss narration
   claim nothing that did not happen, the record still registered.
5. **Row P** — `milestone.provision-failed` over a locked, directory-less own registration; follow
   what it prints (`git worktree unlock`, then the re-run) to exit 0.
6. **After `uninstall`** over fan-out worktrees plus a foreign prunable one: `jigc setup`,
   `milestone create`/`add-task` with the same sub-task ids, `milestone provision` — exit 0, no
   stale-name collision; the foreign worktree, returned, still answers `git status`.
7. **A stale own record that outlives a warned teardown** (`discard --force` over an unlinked
   path): a later milestone with the same sub-task id — `provision` refuses over the leftover
   directory, and after it is cleared provisions cleanly.
8. **`jigc milestone execute`** after a refused provision that left a stale own record — its
   advisories (they classify the path, not the registered set) are byte-identical to today's.
9. **A hand-deleted crashed `.combine-*` record** (plant one, delete its directory): a later
   `milestone finalize` lands regardless, and `uninstall` drops the record.
10. **The suite under `dev/runner-faithful`** (stock git on the runner image, older than the host's)
    — the keyed removal over an absent directory was checked on 2.54.0 and 2.47.3 only.
11. **The trial's container cell (O-1)** re-driven on the new stamp: the host worktree answers
    `git status` at exit 0 after `milestone provision` ran in the container.

## 6 · What must not move

Checked against the sites; **nothing pinned moves**:

- Frozen schemas and both `schema-manifest.yaml` files — no pack file is touched.
- Pinned `--format json` keys and `contract-version`s — `pruned_worktrees` stays text-only (the
  `ENVELOPE_ARMS` four keys and `removed`'s seven flags are untouched); provision, finalize and
  discard envelopes are untouched.
- Compose goldens (`crates/cli/tests/goldens`) — no workflow, pack or adapter text is touched; no
  golden contains the leaked-worktree warning or the uninstall ack's prune line.
- `STORE_EXIT_FLIPS`, `COMMITTING_DOORS`, `VERB_KINDS`, `WORKTREE_DOORS`, every `DestroyingDoor`
  disposition and code set — unmoved. No finding code is added, removed or re-keyed.
- Exit codes — unmoved except row P's one cell (0 → the existing `milestone.provision-failed`).
- M55's own behaviours (the findings doctypes, the doc-only finalize step) — not in reach of any
  site.

What does move, all unpinned text: the leaked-worktree warning's remedy sentence (one test
assertion); the uninstall ack's prune line is absent in the cell where it was false; a landed
finalize over a registered-but-unlinked sub-task path now prints the existing A2 warning.

## 7 · Docs

| home | change |
|---|---|
| `design/team-ready-state.md` → `jigc milestone discard <id>` → *Abandon refuses on a dirty worktree* (the home of the destroying-door rule) | gains the registration-reach rule as one sentence: a door removes a worktree registration only at a path it created under `.jigc/worktrees/`, by path, and runs no repository-wide prune; stale records elsewhere are git's to prune |
| `design/storage.md` → Repository layout, the `worktrees/<sub-task-id>/` row | a pointer clause to that rule (it already defers the door rule to team-ready-state.md); the sentence *"No door removes such a path silently …"* is true of the registrations too after the fix |
| `crates/cli/src/render.rs` — the uninstall ack line | text unchanged; false today in the foreign-only cell, true after |
| rustdoc at the sites — `provision_worktrees`, `remove_worktrees` (incl. the A2 remedy paragraph), `DedicatedWorktree`, `UninstallSummary::pruned_worktrees`, `prune_worktree_admin` and the comment above its call | each states the prune today; revised in the commit that removes it |
| `DECISIONS.md` | one dated entry for the fix: the M31 *"harmless … prune no-op"* and the 2026-09-23 *"prunes now … and says so"* are corrected with the driven datum (foreign records, the moved repository, the refusing runs) |
| `implementation/decisions-pending.md` → the 1.x ledger | the left-open rows of §9, each with its trigger |
| `--help` (`milestone provision`/`finalize`/`discard`, `uninstall`) | no sentence is false today or after — none mentions registrations; unchanged |
| `design/finalize.md` (the throwaway worktree, *"torn down on both exits"*), `design/surface-contract.md` (`git worktree prune` as an example of an operand-less span) | true before and after; unchanged |

## 8 · Fork

**`worktree-1` — jigc's own stale registration.** When a milestone door drops a registration of
jigc's own whose directory is already gone, the record's HEAD, reflog and index go with it. Driven
on rc.24: a commit made inside a sub-task worktree whose directory was then deleted is unreachable
after `milestone provision` and after `milestone discard`, exit 0, nothing printed. The keyed fix
leaves this exactly as it is — it is inside the owned set.

- **Narrow:** keep taking it (the idempotent re-provision M31 declared) and **name it** — one
  warning per record dropped, giving the path and the commit its HEAD held. Nothing newly refuses.
- **Robust:** `provision` and un-forced `discard` **refuse** over a stale own registration that
  still anchors something, under their existing codes with `--force` the existing consent; the
  landed finalize names it.
- **Recommendation: narrow.** The working files were destroyed before jigc arrived, so a refusal
  has no route that restores anything except `--force` (git cannot repair a worktree that has no
  directory); the sha keeps the one durable survivor a `git branch` away; and the robust arm needs
  a new probe leg over git's admin directory to see a staged index, which is a mechanism.
- If neither is taken, the cell stays silent as M31 declared it and is recorded tier 2 for 1.x
  beside the external-writer class — its precondition is a non-jigc deletion.

No other fork. The rulings, the code and the driven cells settle the rest; row P is declared above.

## 9 · Commits

1. `fix(milestone): the fan-out teardown and the combine worktree remove only the registrations they created` — sites 2, 3, 4; tests 2, 3, 5 (finalize cell). Closes L-22 at `milestone finalize` (landing, refusing) and `milestone discard`.
2. `fix(milestone): provision clears its own stale registration by path, after the refusal has had its say` — site 1; tests 1, 5 (provision cell), 6, 7, 11. Closes L-22 at `milestone provision` and the moved-repository mirror.
3. `fix(uninstall): the teardown drops the registrations of the worktrees it removed, and says so only when it did` — site 5; test 4. Closes L-22 at `uninstall` and the misattribution.
4. `fix(milestone): the leaked-worktree remedy names the one registration, never a repository-wide prune` — site 6; test 8 and the three moved pins. Strike-able without reopening 1–3.
5. `test(milestone): fence the worktree-registration reach as a class, and state it where the door rule lives` — tests 9, 10; the two design sentences (the rule is wholly true only once 1–3 have landed).
6. *(fork `worktree-1`, as recommended)* `fix(milestone): a dropped stale fan-out registration is named with the commit it held` — its own red test over provision, discard and the landed finalize, narrated ⇔ taken.
7. The pass's record commit (shared across families): the `DECISIONS.md` entry, the ledger rows, the row annotations in the two rc.24 records.

## 10 · Left open

| what | tier | home |
|---|---|---|
| own stale registration's HEAD / reflog / index dropped unnamed | fork `worktree-1`; tier 2 for 1.x if neither arm is taken | `implementation/decisions-pending.md` → the 1.x ledger |
| stale own records at a previous home path, and hand-deleted crashed `.combine-*` records, linger as `prunable` until git's own gc | tier 3 (bookkeeping residue, no loss) | same |
| a crashed finalize's live `.combine-*` worktree is cleaned by no milestone door | tier 3, pre-existing | same |
| the leftover refusal's route does not name `git worktree repair` for a moved repository | tier 3 (a route that could be better; new text) | same |
| `milestone.dirty-worktree`'s *"the teardown removes it and this content is destroyed"* over a registered-but-unlinked directory | tier 3, pre-existing, driven | same |
| the external-writer race class | tier 2 for 1.x by ruling 4 — not this family | as ruled |
| O-9 (`repo.head-detached` sentence from inside a detached foreign worktree) | tier 3, not this family | `completions/artifacts/RC-rc24/README.md` → O-9 |
