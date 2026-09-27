# Adversarial review — `13a8afbb..a34f910a` (the last pre-1.0.0 batch)

Debug binary built bare from HEAD (`jigc 1.0.0-rc.20`), five `dev/jigc-rig` corpora, every row driven.

---

## Findings

### MEDIUM 1 — the tier-2 route lands the ordinary task and dead-ends the sub-task the worktree exists for; the refusal's route is a `cd` to the caller's own cwd

**Location:** `crates/cli/src/repo.rs:755-771` (the new `HeadDetached ∧ provisioned ∧ Here` arm) · `crates/cli/src/start.rs:2541-2549` (`blanket_base_pin_refusal`'s provisioned arm)

**Datum (driven, rig `committed-singletons`, milestone `second-probe`, worktree `.jigc/worktrees/alpha-work`):**

```
BEFORE-CONTROL (detached worktree):  jigc workflow sub-task --task alpha-work   -> rc 0
follow the new route:                git switch -c own-branch                   -> rc 0
                                     jigc task finalize ordinary-in-wt          -> rc 0, 53cee5d lands
AFTER, same cwd:
  jigc workflow sub-task --task alpha-work -> rc 1
  jigc start --task alpha-work             -> rc 1
  both: "task `alpha-work` is pinned to base 09fb87a but you're on 53cee5d — … run this from
         that worktree — `cd /private/var/…/repo/.jigc/worktrees/alpha-work`, then re-run"
  cwd=/private/var/…/repo/.jigc/worktrees/alpha-work        <-- byte-identical to the cd target
```

The `cd` is the reader's own directory: a route that, followed exactly, changes nothing. `start.rs:2490`'s own doc-comment states the rule this violates — *"Idempotent is not useful: a route whose first clause is a no-op in the state the reader is in teaches that jigc's routes are approximate."* That arm was written for a reader in the **shared checkout**; the new route is the first jigc-sanctioned path that puts the reader **inside** the worktree with HEAD off the base (pre-fix the ordinary finalize was refused at the seam, so no commit landed and HEAD never moved).

**Not loss, driven:** `git switch --detach 09fb87a` restores the arc (`workflow sub-task` rc 0); `own-branch` survives worktree teardown and `53cee5d` stays reachable; `milestone finalize` from main still lands and folds the sub-task's code. The new route is genuinely lossless and better than the brief's suggested text.

**Class bound, and how it was derived:** the cwd-blind `cd` is **one producer** reached from **two doors** — `command grep 'cd {' crates/cli/src` → 2 hits in `start.rs` (2545 the provisioned arm; 2553 the unprovisioned arm, where the worktree does not exist so cwd cannot equal it) and the four adapter/compose hits are the spawn template, not a refusal route; `command grep blanket_base_pin_refusal` → 2 call sites (`start.rs:2628` `start --task`, `start.rs:2754` `workflow … --task`), both driven above. The *reachability* half is the one new route arm.

**Smallest correction:** in `blanket_base_pin_refusal`'s provisioned arm, compare the canonicalized worktree against the caller's cwd and, when equal, name the act that resolves it (`git switch --detach <pinned.short>`) instead of the no-op `cd`. The standing cell (`validate_previews_posture.rs:487-508`) stops one step short of this — it runs the switch and never re-runs the sub-task door.

---

### MEDIUM 2 — `4d0cfea5`'s own completeness claim is falsified by the binary: a third out-of-set code, and the fence hand-lists two cells with "no third" as a stated judgment

**Location:** `crates/cli/src/task.rs:461-480` (the `--dry-run` help) · `crates/cli/tests/dry_run_findings_equal_set.rs` (`the_forecast_names_every_gate_it_refuses_on_that_the_preview_does_not_report`, the *"Why these two and no third"* paragraph)

The help now promises *"It refuses … on **every gate decided before the transaction** — which is more than `jigc task validate` reports: the repository posture, this task's validation findings, the **base pin**, the empty-commit guard, and the carryover gate"*. The commit's derivation rests on: *"every other pre-transaction gate is … a **door**-level refusal both surfaces share (the repository posture, **the sub-task boundary**)"*. Driven, the two surfaces do **not** share the sub-task boundary:

```
rig committed-singletons, milestone second-probe, sub-task alpha-work, from the MAIN checkout:
  jigc task validate alpha-work            -> rc 3, only schema-conformance.required-slot-present
                                              (finalize.milestone-sub-task NEVER printed)
  jigc task finalize alpha-work --dry-run  -> rc 3, blocking · finalize.milestone-sub-task
  jigc task finalize --help | grep -c "milestone-sub-task" -> 0
```

`GATE_COVERAGE` (`crates/cli/src/gate_coverage.rs:206-345`, 13 rows read in full) carries no row for it either — so by the fix's own membership authority it is a third out-of-set member, exactly the *"same incomplete sweep one member over"* the commit message set out to avoid.

**Bound:** derived by reading all 13 `GATE_COVERAGE` rows against the `--dry-run` refusal set driven at both doors. I did **not** enumerate every pre-transaction refusal producer, so read this as *one confirmed additional member, count unbounded* — not "exactly three".

**Smallest correction:** name `finalize.milestone-sub-task` in the help's out-of-set list and add its cell; better, derive `out_of_set_cell`'s subject from `GATE_COVERAGE`'s complement rather than hand-listing two with a prose completeness leg.

---

### LOW 1 — "run verbatim" is an overclaim in the test comment and the commit message; the emitted placeholder is a shell parse error if actually pasted

**Location:** `crates/cli/tests/validate_previews_posture.rs:487-497`; `7081de80`'s body ("Run verbatim from the checkout that printed it: rc 0")

The test substitutes `<new-branch>` → `fan-out-ordinary` before running. Driven literally: `git switch -c <new-branch>` → zsh `parse error near ';'` (the `<` is a redirect). The `<…>` convention is pre-existing (`git switch <branch>`, `--value <value>`), so this is the *wording*, not a new class. **Correction:** say "with the placeholder substituted" in both places.

---

### LOW 2 — `9b2d0655`'s locus fix lands only inside a flattened `{"error"}` blob on the JSON arm

```
jigc --format json task amend "repair the merge"   (merge HEAD)
-> rc 1, {"error": "blocking · amend.head-shape — … \n  at: task:repair-the-merge\n  route: …"}
```

`code`/`target`/`route` are not keyed. Pre-existing to this range (F-10's `finding_to_err` mint path), and M50's audit declined the analogous envelope fix on measured grounds — recorded so the locus repair is not mistaken for a wire-level one. **Instance, unbounded** (I did not enumerate `finding_to_err` callers).

---

## Verified non-findings (each driven, so triage does not re-open them)

- **`task:<never-minted-id>` is honest.** `jigc task validate repair-the-merge` and `jigc doc list --task repair-the-merge` both answer `finalize.no-task` **at `task:repair-the-merge`** — jigc's own shipped locus for a task that does not exist. The new spelling matches precedent; the bare-`task` locus on `write.unslugable-title` at the same door is defensible (no id exists to name).
- **The narrowing to one door holds.** From inside a detached provisioned worktree: `milestone create` rc 0 (committed on main), `task discard <sub> --force` rc 0 (record commit on main), `migrate-corpus --dry-run` rc 0, `setup` rc 0, `rename` reached its own address refusal — none tripped `repo.head-detached`. `milestone finalize` from the worktree reached `milestone.zero-contribution`, and on a clean arc landed (`01c7130`, sub-task's `gamma.txt` folded). Dispatch guard keeping the classifier's subject is load-bearing, not harmless-and-incidental.
- **`PostureSubject::live()` has exactly two call sites** — `repo.rs:1053` (`SeamSubject::live`) and `cli.rs:744` (`finalize_posture_subject`), both applied to `discover_repo_root(cwd)`. `Dedicated` still has no constructor outside `posture_subject`; `commit_seam_posture::no_public_constructor_of_the_dedicated_variant_takes_a_path_or_a_boolean` green.
- **The `sub_task` verdict's source is safe.** `milestone::is_sub_task` → `jigc_home_or_repo(cwd)` → `engine::milestone::owning_milestone` — the same shared-workbench cache `posture_subject`'s `Dedicated` leg already requires, so the two cannot disagree; an unreadable cache falls to *ordinary*, the **stricter** answer. Driven: orientation in the worktree gave `milestone=fan-out-probe` for the sub-task and exempted it, `milestone=None` for the ordinary task and refused it.
- **Both arms, four surfaces, byte-identical, pre-transaction.** Ordinary and amend tasks in the worktree: `task validate` / `--dry-run` / real `finalize` all rc 1 with identical bytes; `start --format json` `tasks[].findings[0].code = repo.head-detached`; HEAD unmoved, task area standing, sub-task row silent at the same instant. Guard sits after `TaskArea::resolve` and before `task.finalize` (`task.rs:1601-1624`) — no write precedes it.
- **Inert off the worktree path.** Attached main: validate rc 0, finalize lands. Detached main: generic `re-attach HEAD with \`git switch <branch>\`` preserved.
- **The `also` route is one-door.** `task amend "   "` gains the `amend-<sha7>` clause; `milestone create "   "` control does not; the named exit runs (`jigc task amend` → `amend-41abe1b`).

## Sanity on the pre-1.0 tree

| check | result |
|---|---|
| schemas · both manifests · both snapshot trees, `5d9fd714..a34f910a` | **empty** |
| goldens, `13a8afbb..a34f910a` | **empty** |
| golden count | **646** |
| `count_fences::` 14 · `posture_member_inventory::` 4 · `gate_coverage_fence::` 4 · `help_truth::` 23 · `work_unit_id_axis::` 5 · `validate_previews_posture::` 5 · `posture_door_axis::` 7 · `commit_seam_posture` 20 | **all rc=0, 0 failed** (run bare, one filter each) |

## Closure table

| row | claim | verdict |
|---|---|---|
| `(2, A2-2)` = `(5, DEFECT 1 · rc.20)` | preview takes the seam's subject; route runs in a fan-out worktree; class = one door | **CLOSED** for the reported cell, both arms, all four surfaces, pre-transaction, inert elsewhere — **MEDIUM 1** is a new dead end the closing route creates one step downstream |
| `(2, A2-1)` | `--dry-run` help names every gate it refuses on | **CLOSED for two of three** — `finalize.milestone-sub-task` unnamed and unfenced (**MEDIUM 2**) |
| `(3, F-C)` | `write.unslugable-title` names `task amend`'s `amend-<sha7>` exit, one producer | **CLOSED**, registry-derived cell, control clean |
| `(5, DEFECT 2 · rc.20)` | `amend.head-shape` locus takes a declared spelling | **CLOSED**, consistent with `finalize.no-task`'s own locus (**LOW 2** on the JSON arm is pre-existing) |
| `(2, A2-3)` = `(3, F-D)` | the settle row struck with its datum | **CLOSED**; the strike's datum reproduces (`{task, text}`, no hex run) |

## Verdict

**HOLDS WITH FINDINGS.** No tier-1 row: zero data loss, zero corruption, zero regression driven; the freeze boundary and the 646-golden fixed point are intact; all eight named fences green. The tier-2 fix's derived class is correct and driven at five sibling doors, and its route is runnable and lossless where the shipped one exited 128. Two MEDIUMs both sit on the batch's own completeness discipline rather than on its behaviour — a route the new happy path newly reaches that points at the reader's own cwd (1 producer, 2 doors, recoverable), and a third out-of-set code the "every gate" enumeration and its hand-listed fence both miss. Neither blocks a 1.0.0 call on integrity grounds; both are the complete-fix lens landing on this batch, which the human should weigh against the exit rule's third clause.
