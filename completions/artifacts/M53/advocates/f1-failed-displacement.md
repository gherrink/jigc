# Robust-case brief — fork `(3, A3-2)`, the failed displacement

Every behavioural claim below is marked **DRIVEN** (run on `~/.local/bin/jigc` → `jigc 1.0.0-rc.16`, rigs from `dev/jigc-rig`, two-step eval, `mktemp -d` roots) or **READ** (source at HEAD `978577ec`). No edits, no cargo. Where I drove something that weakens my own case, it is in §1, first.

---

## 1 · What I concede up front, driven

**(a) The pre-commit arm is gold-plating, and I am withdrawing it from my own recommendation.** I drove the racer M52 named. A *succeeding* `pre-commit` hook that writes into the task area during the commit, with `.jigc/displaced` a regular file:

```
hook:  printf 'HOOK-WROTE-DURING-COMMIT\n' > .jigc/tasks/tidy-the-readme/hook-note.txt ; exit 0
$ jigc task finalize tidy-the-readme            EXIT=0
stderr: note: could not open .jigc/displaced/tidy-the-readme to move
          .jigc/tasks/tidy-the-readme/hook-note.txt aside: Not a directory (os error 20)
after:  find .jigc/tasks  ->  .jigc/tasks      # the hook's byte is gone, exit 0
```
**DRIVEN.** The entry did not exist when phase 1 or 2 ran, so a pre-commit probe **cannot see it**, and a pre-commit *displacement* cannot move it. The conditional removal is the only thing that closes this cell. A probe closes a strict subset of what the removal closes, at the price of new mechanism.

Worse, the *move-before-commit* variant is actively regressive: a hook-rejected `milestone finalize` today says (**DRIVEN**, A3-1 baseline §6) *"milestone:axis-three-probe is intact — nothing was committed … every provisioned sub-task worktree still holds its staged code"*, with both plants surviving. Displacing before the commit makes that survivable-frame state-truth clause false — a law-1 lie minted at a door that is already correct. **The robust arm I argue contains no pre-commit gate.**

**(b) The lying-state skeleton is a pre-existing *shipped* condition, not one the fix invents.** M52's mint-unwind already produces it deliberately, at exit 1, with a code and a route (**DRIVEN**):

```
$ jigc milestone add-task axis-three-probe "first sub"   # planting, rejecting pre-commit hook
EXIT=1  blocking · milestone.foreign-bytes — `.jigc/tasks/first-sub` holds bytes jigc did not write …
disk:   .jigc/tasks/first-sub/hook-wrote-this.txt   (only member)
$ jigc task list        EXIT=0   "1 active task(s)   first-sub"
$ jigc start            EXIT=0   "Active task: first-sub / workflow: none recorded"
$ jigc task validate first-sub    EXIT=0   "no findings — the task validates clean"
$ jigc start --task first-sub     EXIT=1   could not read the base pin for task `first-sub` … (code-less)
$ jigc task finalize first-sub    EXIT=1   could not read the base pin at "/private/var/folders/…" (code-less, HOST PATH)
```
So `task list`/`start`/`task validate` lying over a leftover area, and the two code-less dead ends, are **today's behaviour on today's binary**. Neither arm creates that class. The cheap arm's bound is joining a shipped condition, which is a real and honest cost reduction.

**(c) The residual has a correct shipped exit.** On the exact Option-A residual (landed `task finalize`, only the un-moved foreign entry standing) — **DRIVEN**:

```
$ jigc task discard tidy-the-readme    EXIT=1  blocking · task-discard.foreign-bytes — … 1 path(s) jigc did not write
                                       route: move what you need out … then re-run … or `--force`
$ jigc task discard tidy-the-readme --force   EXIT=0  warning: … not recoverable.  discarded task tidy-the-readme
$ jigc task list                       "no active tasks"
```
The state is clearable by a shipped door, with a shipped finding and a shipped route.

**(d) The milestone-area residual creates *zero* lying state.** With `.jigc/milestones/<id>/` left standing holding only a foreign byte after a landed boundary, **all six** milestone doors refuse at exit 1 with the shipped `milestone.terminal`, and the task surfaces are silent (**DRIVEN**):

| door | exit | says |
|---|---|---|
| `milestone list-tasks` · `finalize` · `join` · `discard` · `provision` · `add-task` | **1** each | `blocking · milestone.terminal — milestone …  is joined … has no workbench`, route = `jigc doc show milestone-record:<id>` |
| `task list` | 0 | *no active tasks* |
| `start` | 0 | no active task |

So for row A3-1's area, leaving-in-place is **complete as-is**. No third leg is owed there. That is a concession that removes a third of the robust arm's claimed scope.

---

## 2 · The hole the cheap cut leaves — one cell, and it is tier 1 by the charter's own definition

Everything above narrows the fork to a single `(door, cell)`: **a sub-task area left standing after a landed `milestone finalize`, met by `jigc task discard <sub-id>`.**

**Today, with no leftover (the ordinary path) — DRIVEN:**
```
$ jigc task discard first-sub            EXIT=1  blocking · finalize.no-task — no task `first-sub`
$ jigc task discard first-sub --force    EXIT=1  (same)   git log: unchanged
```

**After either arm's fix, with the residual standing — DRIVEN, twice:**

```
# (i) the residual holds the un-moved foreign byte → --force is the route jigc itself printed
$ jigc task discard first-sub            EXIT=1  task-discard.foreign-bytes  route: "… or --force …"
$ jigc task discard first-sub --force    EXIT=0
  discarded task first-sub
  record commit: e8a1d65 — this sub-task's milestone record, settled to `discarded` and committed on its own
  git log:  e8a1d65 chore(milestone): discard task:first-sub on milestone:axis-three-probe
            b67cbc4 Finalize milestone axis-three-probe (2 sub-tasks)
  record:   status: joined (header) ; - status: discarded   ← the sub-task whose work landed in b67cbc4

# (ii) WORSE — no --force needed. Follow only the FIRST half of jigc's own route
#      ("move what you need out of .jigc/tasks/first-sub/, then re-run jigc task discard"):
$ jigc task discard first-sub            EXIT=0
  discarded task first-sub
  record commit: f73c5e5 …   record: - status: discarded, under a joined milestone
```

That is: **a committing door, exit 0, a permanently committed false record** — the class M42 built `milestone discard` to prevent and M49 Increment 2 T3 built `settle_discarded_sub_task` to close — reached by following the route the cheap arm's own advisory prints. The charter's tier 1 is *"exit-0 loss or repository harm through a committing or destroying door"*; `task discard` is both. The next per-axis review drives axis 3 (`DESTROYING_DOORS`) and axis 5, and `task discard` is a row in the first.

**The codebase already states the rule and states the premise the fix breaks (READ, `crates/engine/src/milestone.rs:1551-1555`, `discard_sub_task_item`'s doc-comment):**

> *"The flip is **unconditional on the item's current value** … this arm is reachable only while the sub-task has a live working area, and a `joined` item's area no longer exists (both terminals are written at `finalize`, whose teardown removes the workbench, after which the CLI door refuses at task resolution). **A guard here would be code no state can reach.**"*

The fix's whole content is *the teardown stops being unconditional*. The moment either arm lands, that paragraph is false in the binary, and its consequence is the commit above. **This is the M46 razor's exact shape — a rule stated in one place, violated in another, demonstrable by driving the binary.** The cheap arm's price is not "a lying local state"; it is **writing that paragraph down as a declared bound that ships a path to a committed lie.**

---

## 3 · The one-way-door tell

**The pin.** `1.0.0` is being called on the next re-review. Three things here cross it and cannot be repaired cheaply afterwards:

- **The milestone landed envelope cannot carry a finding at all.** **DRIVEN:** `jigc milestone finalize --format json` on a landed boundary returns top-level keys `['committed']` — **no `findings`**. **READ:** `ENVELOPE_ARMS` pins `["milestone","finalize"] Landed` as `ArmShape::Object(&["committed"])`, `ArmStatus::Pinned`, whose own `origin` note says *"only one of them carries `findings`"*. The additive-key window closed at M48; adding it is a 2.0 act under M51 Inc 5's written policy. By contrast `task finalize` landed **does** carry `findings: []` (**DRIVEN**, keys `['committed','findings','schema_version']`).
  **Consequence for the cheap arm:** at `milestone finalize` — the door where the harmful residual lives — "say it loudly" reduces to a `note:` on **stderr only**, machine-invisible forever. The cheap arm's safety rests on *telling*, and at the door that matters the product has contractually given up the ability to tell a machine. The robust arm does not depend on being heard: it makes the residual safe at the door the residual reaches.
- **The `discard_sub_task_item` premise.** Revising it into a bound is a documented decision that a later wave must re-litigate against a pinned read contract; building the guard retires it.
- **The port.** Both doors run daily once this repo migrates (the charter says so: *"Two of the four rows are in the doors a port exercises daily"*). The residual's trigger — an occupied or unwritable `.jigc/displaced/<id>` — is *sticky*: `.jigc/displaced/` is never cleaned by jigc (`design/storage.md:125`), so one collision plus a `docs`-shaped occupant reproduces the partial cell on every later boundary (baseline §4.1, **DRIVEN**).

---

## 4 · The two rationalizations, refuted

- ***"No live case to test."*** Refuted by construction: every cell above was driven today, on the release binary, in throwaway rigs, including the one that does not exist yet (the post-fix residual, simulated exactly — jigc's files absent, only the un-moved entry standing). The harmful cell needed no plant fancier than `mkdir -p .jigc/tasks/first-sub`.
- ***"Premature generality."*** The guard is not general. It is `item_is_settled(item)` — a **shipped** engine predicate (`crates/engine/src/milestone.rs:1663`) already asked at three loci (`reseed_sub_task_areas`, `settled_sub_task_ids`, `flip_record_status_to_joined`) — asked at the fourth locus whose doc-comment says it was skipped only because no state could reach it. That is the definition of *a condition on a guard that already exists*.

---

## 5 · Pricing against THE BOUNDARY — what each arm mints that does not exist today

| | **Cheap arm** (conditional removal + note/finding + declared bound) | **Robust arm** (same + the `task discard` terminal condition) |
|---|---|---|
| **new finding code** | 1 — `task finalize` and `milestone finalize` both carry `codes: &[]` (READ, `milestone.rs:3245`/`:3263`), so the boundary's *"unless the row's door already carries none"* clause is satisfied | **same 1** — the terminal condition reuses shipped `milestone.terminal` (`engine::milestone::terminal_milestone_finding`), **zero additional** |
| **new registry** | none | none |
| **registry rows** | 1 row in `cli::render::FINALIZE_FAMILY` (derived-membership fence, `finalize_family_registry.rs` arm 1 reddens if you skip it) | same 1 |
| **`DESTROYING_DOORS` movement** | none — `codes` is declared *"door-scoped **blocking** … refuses with"*, and a kept-bytes advisory is not a refusal, so `codes: &[]` and its ⇔ stay true (READ `milestone.rs:3113-3118`) | same; the `task discard` row's `codes` gains `milestone.terminal`? **No** — it is not a refusal *over a path it destroys*; it refuses over the record. Row unchanged. State that in the plan. |
| **`GATE_COVERAGE` rows** | **0** — phase 7 is not a gate | **0** |
| **envelope keys** | **0 new keys.** `task finalize` landed already carries `findings` (DRIVEN); `committed.displaced` already present-always. **But:** `milestone finalize` landed has no `findings` key and the arm is `Pinned` — the residual is stderr-only there. **This is an unfixable-before-2.0 asymmetry the cheap arm's safety depends on.** | same 0 new keys, and the asymmetry **no longer load-bearing**, because the residual's harm is closed rather than announced |
| **new gate / phase** | none | none |
| **signature changes** | `displace_foreign_area` → returns the unmoved set (or the door recomputes `foreign_area_paths` minus `moved`); `post_commit`'s removal → `engine::state::unwind_area` | same |
| **doc homes to revise** | `design/storage.md:291` · `design/finalize.md:157` **and `:159`** (its *"the next finalize reusing the slot"* clause is **driven false** — re-finalize exits 1 on the missing base pin) · `task.rs:5839-5843` · `post_commit`'s doc (`task.rs:5942`) · `finalize_displacement.rs` module doc · `command-output-contract.md`'s `finalize.*` sub-table (row-fenced) · **`engine/milestone.rs:1551-1555` rewritten into a declared bound that names the committed-lie path** | the same list, **except** the last one is rewritten to state the guard, not the bound |
| **tests** | `finalize_displacement.rs` (+ move axis) · `milestone_boundary_displacement.rs` (+ move axis) · `flow53` arm 3 | same + one `task discard` × terminal-item cell |
| **goldens** | none (no pack text moves) | none |
| **schema-hash** | zero | zero |

**Both arms are inside the boundary.** That is the honest headline: the robust arm is **not** a mechanism-adding arm. Its entire delta over the cheap arm is one `if item_is_settled(item)` in a function whose doc-comment already names the guard and says why it was omitted.

### Sibling cells each arm creates or leaves

| cell | cheap | robust |
|---|---|---|
| `task discard <sub>` on a **joined/discarded** item, with or without `--force` — commits a false record at exit 0 | **created, declared** | **closed** |
| `task list` / `start` / `task validate` report a finalized task as active | left (pre-existing, §1b) | left (same) |
| `start --task` / `task finalize` on a residual: code-less, route-less, host-absolute path, exit 1 | left (pre-existing) | left (pre-existing) — **name it as a carried tier-2 row**, not a fix |
| `narrate_displacement`'s count (`moved.len()`, not the complement) — "held 1 entry" when it held 2 | fixed by both (it is the same seam) | fixed |
| the milestone-area residual (post-A3-1) | **safe, driven §1d** | safe |
| `.jigc/milestones/<id>/merged/**` foreign bytes (A3-1 baseline §5 rows 3/4) | out of scope for both | out of scope for both |
| `.jigc/displaced/<id>` collides between a milestone id and a task id | provenance-ambiguity only, no loss (A3-1 §4, driven) | same |

---

## 6 · The identity — recommendation

**One code, minted once, used at both doors: `finalize.foreign-bytes`.**

- **Why one, not two.** Both doors run the same phase-7 seam (`post_commit` / `cleanup_subtask_areas`), both carry `codes: &[]`, and the class is identical. Two codes under two warrants for one class is the law-1 wobble M52 Increment 9 spent an increment removing.
- **Why the `finalize.*` family and not `task-finalize.*` / `milestone.foreign-bytes`.** `task.rs:826-847` forecloses reusing a sibling door's `*.foreign-bytes` by name; `milestone.foreign-bytes`' shipped text is scoped to *"the working area **this call minted**"*, which is false at a boundary, so reuse would require rewording a code three other producers share. The `finalize.*` family is a **registry that already exists** with a derived-membership fence, and `task discard` already emits `finalize.no-task` at a task door (**DRIVEN**) — cross-namespace at a task-shaped door is shipped precedent.
- **Law-1 consequence.** The code is a key; the sentence must say what actually happened — *kept, not taken*: `.jigc/tasks/<id>` still holds N path(s) jigc could not move aside to `.jigc/displaced/<id>` (<reason>); `.jigc/` is gitignored, so nothing else has a copy — and the route must name `jigc task discard <id>` (the door that clears it) rather than "delete the rest", which the mint-unwind route says and which is the weaker of the two shipped routes.
- **Severity/exit.** Advisory at exit 0 on the landed arm. Turning a landed commit non-zero contradicts `command-output-contract.md`'s landed-arm-is-exit-0 taxonomy, and the commit is truth. At `milestone finalize` the finding cannot ride the envelope (§3) — declare that asymmetry in the plan, in writing.
- **For the terminal guard:** reuse `milestone.terminal` verbatim, target `milestone:<id>`, existing route. No new identity.

---

## 7 · The robust arm's minimal honest shape (≤10 lines)

1. `displace_foreign_area` returns the complement it could **not** move alongside what it moved.
2. `narrate_displacement` names both sets and counts the complement, never `moved.len()`.
3. `post_commit` and `cleanup_subtask_areas` replace `remove_dir_all` with `engine::state::unwind_area(area, kind)` — jigc's own members go, `remove_dir` refuses a non-empty area, `AreaUnwind::Foreign` is the typed "left standing".
4. On `Foreign`, emit one advisory `finalize.foreign-bytes` per area, located at the area, routed to `jigc task discard <id>`; carried on `findings` at `task finalize`, stderr-only at `milestone finalize` (declared).
5. `engine::milestone::discard_sub_task_item` asks the shipped `item_is_settled(item)` **before** the splice and refuses with `milestone.terminal` — the guard its own doc-comment names.
6. That doc-comment is rewritten to state the guard and why the old premise died.
7. `storage.md:291`, `finalize.md:157`/`:159`, `task.rs:5839-5843`, `post_commit`'s doc and `finalize_displacement.rs`'s module doc are revised, with `:159`'s falsified clause struck with its datum.
8. Acceptance iterates the 18-cell axis below through the real binary, plus the `task discard` × settled-item cell.
9. No pre-commit gate, no `GATE_COVERAGE` row, no new registry, no schema movement.
10. `flow53` arm 3 and the two displacement suites gain the move axis.

### The full axis — `{all move, some move, none move} × {removal ok, removal fails} × {3 doors}`

*"removal ok/fails"* = the post-displacement `unwind_area` succeeding vs returning `Err` (a genuine I/O fault on a jigc-written member).

| move outcome | door | removal ok → intended outcome | removal fails (Err) → intended outcome |
|---|---|---|---|
| **all move** | `task finalize` | area gone; `displaced` names each pair; no finding; exit 0 | area partly standing; `finalize.foreign-bytes`? **no** — nothing foreign is there. Advisory `note:` naming the fault; exit 0 |
| **all move** | `ms finalize` sub-areas | each area gone; union on `committed.displaced`; exit 0 | per-area `note:`; exit 0 |
| **all move** | `ms finalize` milestone-area (post-A3-1) | area gone; pairs under `.jigc/displaced/<milestone-id>/`; exit 0 | `note:`; exit 0 |
| **some move** | `task finalize` | **moved pairs named AND unmoved paths named** (count = complement, not `moved.len()`); area stands with the unmoved set; `finalize.foreign-bytes` advisory + `task discard` route; exit 0 | same, plus the fault `note:` |
| **some move** | `ms finalize` sub-areas | same per area; union on `displaced`; finding **stderr-only** (declared) | same + fault `note:` |
| **some move** | `ms finalize` milestone-area | same; **no** lying state — every milestone door refuses `milestone.terminal` (driven §1d) | same + fault `note:` |
| **none move** | `task finalize` | area stands whole-complement; both sets named; advisory + route; exit 0; **`task discard` clears it** | same + fault `note:` |
| **none move** | `ms finalize` sub-areas | area stands; **`task discard <sub>` now refuses `milestone.terminal`** instead of committing a false record | same + fault `note:` |
| **none move** | `ms finalize` milestone-area | area stands; all six milestone doors already refuse; exit 0 | same + fault `note:` |

*(The 18th dimension the charter's 2×2 lacks and the baseline found — **some move** — is the row that carries the law-1 lie today, and it is reachable with no permission games: occupy `.jigc/displaced/<id>/docs` with a file.)*

### Bounds the robust arm still declares

1. `task list` / `start` / `task validate` report a residual area as an active task, and `start --task` / `task finalize` dead-end code-lessly with a host-absolute path. **Pre-existing and shipped** (§1b) — joined, not created; carried as a tier-2 ledger row with its trigger.
2. At `milestone finalize` the residual is **stderr-only**: the landed arm is `Pinned` at `Object(&["committed"])` and the additive-key window closed at M48. Machine-invisible until 2.0.
3. `.jigc/milestones/<id>/merged/**` remains unwalked by declaration; A3-1's `materialize` cell (exit-3 loss) is outside both arms.
4. `free_displacement_path`'s unbounded `loop` (READ, not driven) is untouched.
5. An undecodable filename still fails its rename — now **kept** rather than taken, which strictly improves the M52 declared bound at `task.rs:5847`.

---

**Verdict: robust-now** — but the robust arm is *one shipped predicate asked at one shipped seam*, not a pre-commit gate. Drop the pre-commit probe (it cannot see the hook racer, and its move-before-commit variant falsifies the survivable frame). Keep the conditional removal — which is the cheap arm, and is right. What must not be deferred is the single cell the conditional removal **creates**: `jigc task discard <sub-id>` over a settled item, which I drove landing a permanently committed `discarded` flip on a `joined` sub-task at **exit 0, without `--force`**, by following jigc's own printed route — tier 1 by the charter's own definition, through a door the next re-review drives, guarded by `item_is_settled` which the engine already ships and whose omission its own doc-comment justifies with a premise this fix destroys. Declaring that as a bound means writing into the engine that jigc has a route to a false committed record; building it costs one `if` and no new mechanism.
