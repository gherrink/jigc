# M53 — the acceptance design

Two instruments, in this order. **The second is the human's gate; the first is the build's.**

1. **Flow 54** — `crates/cli/tests/flow54_acceptance.rs` (registered in a group root) + the
   [worked-examples](../../../design/worked-examples.md) chapter: five arms through the real binary, each
   **naming the kind of set it iterates** and iterating its class's axis rather than the reported repro.
2. **The partial per-axis review** — axes **2 · 3 · 5** only, on the installed `1.0.0-rc.17`, with M52's
   re-pointed instrument, compared row by row against M52's §A. **The exit rule:** no tier-1 row ⇒ the
   1.0.0 call is the human's to take. It is the completion workflow's act after the audit's fixes, **not
   an increment.**

## The arms

| arm | decision | the set it iterates — and what kind of set that is | per-cell assertion |
|---|---|---|---|
| **1** | D1 (§1, §2) | `{area root · merged/ top · merged/docs/ beside a materialized body · merged/docs/<doctype>.md · merged/docs/provenance.json · merged/<dir>/**}` × the doors over the milestone area, read from **`DESTROYING_DOORS` through its `Disposition` axis** (a code-side registry) plus `materialize` on a blocked finalize. The plant shapes are **manufactured and say so** — *where a foreign byte can sit* is a property of a fixture no registry enumerates. | A `Displace` door moves the plant to `.jigc/displaced/<milestone-id>/<relative>` and names it on stderr and on `committed.displaced`; a `Refuse` door refuses at its shipped identity without `--force` and narrates with it; a blocked finalize (exit 3) leaves the plant on disk and is **not** blocked *by* it. **Control:** an ordinary post-join area displaces nothing, `displaced` is `[]`, `.jigc/displaced` is not created. |
| **2** | D2 (§3, §6, §7, §13) | `{all move · some move · none move} × {unwind ok · fault on the pin · fault on a later member} × {task area · sub-task areas · milestone area}`, both squash arms where the door has them — **manufactured and says so** (the failure points are decided, and the move axis is a property of the parking home's state). | Every un-moved byte is on disk after the landed commit, by `command grep` with a before-control; the narration counts the **complement**; one `finalize.foreign-bytes` advisory per area left standing (on `findings` at `task finalize`, on stderr at `milestone finalize`), exit 0. **Named cells:** **D1×D2** — a foreign byte under `merged/` whose move failed survives a landed `milestone finalize` · the hook-writes-during-the-commit cell (zero move failures, the unwind still answers `Foreign`) · `milestone discard --force` still **takes** a sub-task area's foreign byte and still acks `workbench removed` (§3). **Controls (§13):** an ordinary task lifecycle, a migrate task and an ordinary post-join area each unwind to `Removed`, mint no advisory, leave nothing. |
| **3** | D3 (§9), D2.6 (§8) | `{empty dir · foreign file · foreign docs/<ty>:<slug>.md}` (manufactured) × `{plain id · sub-task of a joined milestone · sub-task of an open milestone}` × **`WORK_UNIT_ID_DOORS`** (a code-side registry whose rows carry their own argv) ∪ the enumerating doors, **derived** from `list_active_task_ids`' production callers and stated as a derivation. | No enumerating door lists a residual; every by-id door answers `(finalize.no-task, task:<id>)` with the residual sentence and a route naming the repo-relative path, on the findings envelope, **no host-absolute path on any surface**; the same-slug mint refuses `task.serial-collision` with the residual sentence; `jigc rename` **proceeds** over a task residual and over a terminal milestone-area residual; `milestone create` over a record-less milestone residual names the path. **Named cell:** a bare `mkdir .jigc/tasks/<sub>` under a joined milestone, then `jigc task discard <sub>` → refused, **no record commit**, `git log` unchanged. §8's guard: a whole leftover area of a settled sub-task refuses `milestone.terminal` at target `task:<sub>`. |
| **4** | D4 (§10) | The four new `GitState` variants × **`BEHALF_DOORS`' acting rows** (a total classification). The family's existing arms (`flow53` arm 2, `posture_door_axis`, `repo_posture`) pick the member up **by derivation** from `InProgress::ALL` / `GitState::ALL`; this arm asserts what they cannot. | Every acting door refuses `repo.operation-in-progress` with the member's noun; `.git/MERGE_MSG` (resolved per worktree) and the index are **byte-unchanged** after the refusal — covering all three damage shapes (swallow · message-only kill · index contamination); the conclude phrase carries no *"once its conflicts are resolved"* on the clean cells; the abandon argv, run verbatim, exits 0 and leaves `posture()` clean in all four states with the picked changes still in the working tree. The five shipped conclude-bearing members' rendered bytes are unchanged. |
| **5** | D5 (§11, §12) | **`engine::state::MINT_DOORS`** (a code-side registry, three prose rows + two `Exempt(reason)` rows) ∪ `milestone add-from-spec` (reaches `add_task`; stated as a derivation) × `{"" · whitespace · punctuation · non-Latin script · stopword-only}`. | Each prose door refuses `write.unslugable-title` **before any write** — no record commit, no area, `git status` clean — with a sentence that is true for a non-Latin title; outside a repository `milestone create ""` still answers the one not-in-repo answer (§12); the exempt rows mint their non-prose ids unchanged. |

**Deliberately unrepresented:** the close (mints no verb, finding or route a done-picture walk needs an arm
for — the M46 Inc 9 / M48 Inc 11 / M49 Inc 12 precedent).

## Spikes owed — a claim until the increment drives it

| increment | before building on it, drive |
|---|---|
| 1 | a foreign file left in `merged/docs/` reaches none of `plan_promotions`, the gate copy loop **as corrected (§1)**, or the merged-state conformance validation · selective clearing still removes a **stale** jigc body (a doc a sub-task has since un-staged) and the join's order-invariance suites stay green · `displace_foreign_area` over a two-level entry (`merged/docs/x.txt`) |
| 2 | the three zero-false-fire controls (§13) · which arm a finding rides on `task finalize`'s landed envelope |
| 3 | the `rename` wedge over a terminal milestone-area residual (**relayed** at planning) · that no legitimate area is ever pin-less at a moment a door can observe it, across the five `MINT_DOORS` |
| 4 | `git reset` on all four states, per worktree (a linked worktree's `MERGE_MSG` lives under `.git/worktrees/<n>/`) |
| 5 | which reject arm a finding rides at the milestone mint doors today (**relayed both ways** at planning) |

## What the re-review's drivers are told

`command grep` with a before-control under `.jigc/` (the shell's `grep` honours `.gitignore`) · D1's and
D2's declared bounds, so the axis-3 driver grades **against** them rather than re-finding them · the
ledger of what M53 minted ([settle-record.md](settle-record.md) §14), so a finding inside M53's own new
code is recognisable as one — by the exit rule it triggers another fix pass, never a wave.
