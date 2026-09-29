# M53 — the baseline ledger

**Posture.** Driven 2026-09-21 on the **installed release** `~/.local/bin/jigc` = `1.0.0-rc.16`
(sha256 `0ddd1ee6…`), git 2.54.0, at `HEAD = 155054cc`. `git diff e519e4eb..HEAD --stat -- crates packs
Cargo.toml Cargo.lock dev` is **empty**, so the installed binary is code-identical to HEAD and one drive
confirms both. Four `capability-auditor`s, one per charter row, each on `dev/jigc-rig` corpora; their
reports are persisted **verbatim** as the four companions below. **A fix pass, not a wave**: no gap
fan-out ran; each auditor answered the halt question (*does the charter's fix shape fit mechanism that
already exists?*) and enumerated its class's sibling cells.

| row | companion |
|---|---|
| `(3, A3-1)` | [baseline-a3-1-milestone-area.md](baseline-a3-1-milestone-area.md) |
| `(3, A3-2)` | [baseline-a3-2-failed-displacement.md](baseline-a3-2-failed-displacement.md) |
| `(2, DEFECT A)` | [baseline-a2-cherry-pick-posture.md](baseline-a2-cherry-pick-posture.md) |
| `(5, DEFECT 1)` | [baseline-a5-degenerate-mint.md](baseline-a5-degenerate-mint.md) |

**An agent's report is a lead, not a measurement.** The claims the Settle's forks turn on were
**re-driven by the orchestrator** on the same binary before entering this record (§5); everything else
below is **relayed** from the companions, each of which marks its own claims DRIVEN or READ.

**An instrument finding, recorded once so nobody repeats it.** In this harness `grep` is a shell function
wrapping `ugrep --ignore-files`, which honours `.gitignore` — and `.jigc/` is gitignored whole. So
`grep -rl <marker> .` answers *not found* under `.jigc/` whether or not the bytes exist. Two auditors hit
it independently; the M52 review's R-I repro asserts loss with exactly that shape. The verdicts stand (the
auditors re-proved every loss with `command grep` + a before-control + `git log -S` + `git fsck`), but
**the evidence shape in `M52/per-axis-review/axis-3.md` §3 R-I is unsound**, and the re-review's drivers
are told to use `command grep` with a before-control.

---

## 1 · All four cells confirmed at HEAD

| row | confirmed | the datum |
|---|---|---|
| `(3, A3-1)` | **yes**, both `squash` arms | `milestone finalize` exit 0, stderr 0 bytes, `displaced: []`, `.jigc/milestones/<id>/` gone with both plants, in no git object; the three sibling doors refuse or narrate over the identical plant |
| `(3, A3-2)` | **yes**, both doors, both cells (a file at `.jigc/displaced`; the dir read-only) | exit 0, the commit **landed**, a bare `note:` with no code and no route, `displaced: []`, the bytes in no commit, no object, no dangling blob |
| `(2, DEFECT A)` | **yes** | clean `cherry-pick -n` leaves `MERGE_MSG` (+ `AUTO_MERGE`, outside `MARKER_UNIVERSE` and not a usable discriminator); `task validate` clean; `task finalize` exit 0 commits the payload under jigc's subject; `MERGE_MSG` gone; `cherry-pick --continue` → 128 |
| `(5, DEFECT 1)` | **yes** | `milestone create ""` and `add-task <m> ""` exit 0, two record commits at `milestone:milestone` / `task:task` |

## 2 · Every class came back wider than its row

| row | the widening | consequence for the fix |
|---|---|---|
| `(3, A3-1)` | **`merged/**` stays a silent-loss region after the charter's fix** — `MILESTONE_AREA_FILES` carries `merged/` as a tree member, *jigc's wholesale, never walked* (`team-ready-state.md:92`, M52 Inc 4/T2). And a **new cell**: `engine::milestone::materialize` clears `merged/docs/` unconditionally (`engine/milestone.rs:1890`), so a foreign byte there dies on a finalize that **does not land** (exit 3). | a fork — see §4 F2 |
| `(3, A3-2)` | **(i)** the move axis has a third value — a **partial** move: one entry blocked by `EEXIST` at the parking home, the success narration says the area *"held 1 entry"* (it counts `moved.len()`), the second entry destroyed. **(ii)** the recorded *safe-by-accident* cell is only partly safe: `remove_dir_all` is not atomic, so it leaves a **skeleton** (jigc's own files gone, the unreadable entry standing). **(iii)** the same unconditional-removal shape sits at `cleanup_subtask_areas` (`milestone.rs:5538`), whose `all_gone` return both landed arms ignore. | the axis is `{all, some, none move} × {removal ok, fails} × {three areas}`, not 2×2 at two doors |
| `(2, DEFECT A)` | **three damage shapes, not one**, across the 12 acting `BEHALF_DOORS` rows: *swallow* (`task finalize`), **message-only kill** (`milestone create` / `add-task` / `discard` / `finalize` — a record-only commit consumes `MERGE_MSG`, the pick left staged, invisible until `--continue` fails), and **index contamination** (`config set docs-root` — a mover `git mv`s into the user's pending pick). Plus two byte-identical sibling states: a clean **multi-commit** `-n` range (no `sequencer/`), and a conflicted `-n` **after `git add`**. | the member must refuse at every acting door, movers included — which the family already does by construction; the fixture owes the state |
| `(5, DEFECT 1)` | **(i)** not punctuation-only: every **non-Latin-script** title (`日本語`, Cyrillic, emoji) and every **stopword-only** title (`"the"`, `"A"`) slugs to nothing. **(ii)** a **third committing door**: `milestone add-from-spec` reaches the same `engine::milestone::add_task`. **(iii)** `add-task ""` commits a record `jigc validate` itself calls *corrupt* (`intent` must not be empty). | a CLI-side guard at two doors leaves the third open; the guard belongs where membership is decided |

## 3 · The halt question, per row

| row | fits existing mechanism? | what the charter's prescription got wrong (each driven or read at the line) |
|---|---|---|
| `(3, A3-1)` | **Yes.** `MILESTONE_AREA_FILES` is `pub` and production; `displace_foreign_area` already takes `kind: WorkArea` and `unit_id`; `foreign_area_paths` is kind-generic; `FINALIZE_DOOR` is already `Disposition::Displace`; `committed.displaced` is declared and pinned as a value inside `Object(&["committed"])`. One parameter widening: `post_commit`'s `displace` tuple hard-codes `WorkArea::Task` (`task.rs:5976`) and both milestone call sites pass `None` (`milestone.rs:5098`/`:5228`) under a comment that says *"this door must not answer for a subject it was not given"*. | nothing — but see the `merged/` fork. A milestone id and a task id **can collide** at `.jigc/displaced/<id>/` (driven); no loss (`free_displacement_path` suffixes), a provenance ambiguity to state. |
| `(3, A3-2)` | **Not as written.** | **(a)** *"the door's existing `foreign-bytes` identity"* — **neither `Displace` door has one** (`codes: &[]` at `milestone.rs:3248`/`:3266`; the three registered `*.foreign-bytes` codes belong to `task discard`, `milestone discard`+mint-unwind, `uninstall`, and `task.rs:826` says *its own, never a sibling door's*). **(b)** *"refuses"* — displacement and removal run in **phase 7, after the commit landed**, so nothing can be refused. **(c)** *"leaves the area in place"* — contradicts `storage.md:291` and `finalize.md:157` (*the teardown is never skipped — a left-over area makes `task list` report an active task that finalized*), and **that premise is driven true**: `task list` → *1 active task(s)*, `start` → *Active task*, `task validate` clean, re-finalize → `finalize.empty-commit` (wrong code); for a sub-task, `task discard <sub>` then **commits a `discarded` flip onto a joined milestone's record at exit 0**. The safe primitive ships and is unused here: `engine::state::unwind_area` (registry-keyed, non-recursive, `AreaUnwind::Foreign`). **A fork — §4 F1, with an advocate.** |
| `(2, DEFECT A)` | **Yes** — a variant, a `detect` conjunction (the enum already uses two), an `EXPECTATIONS` row (the table already carries 0/2/4-marker rows), a rig generator, two doc-table rows the generation fence (`posture_member_inventory.rs`) will demand. | **(a)** the predicate as written **matches a paused `rebase-merge`** (`MERGE_MSG` + `rebase-merge/`) — it must be disjoint from `Rebase` by predicate or by position. **(b)** the route `rm .git/MERGE_MSG` is **false inside a linked worktree** (`MERGE_MSG` is per-worktree) and is not a git command; `git cherry-pick --abort` / `git merge --abort` exit 128. **`git reset`** is git-native, worktree-correct, clears `MERGE_MSG`, keeps the picked bytes (unstaged), and works on the conflicted cell; `git reset --merge` (the siblings' route) **destroys the picked bytes**. **(c)** the noun must differ from `CherryPick`'s. **(d)** a conflicted `-n` moves off `UnmergedIndex` if the member probes before it. No false positive found in 12 concluded states; GUI clients and a crashed git are an undriven bound, as the family's git-2.54 deferral already declares. |
| `(5, DEFECT 1)` | **Yes, but not the named identity.** | `work-unit.malformed-id` is the wrong piece twice: on the **derived** id it is **inert** (`mint_id("") == "milestone"` and `is_slug("milestone")` is true — which is also why `every_mint_door_produces_an_id_every_door_accepts` is green over the defect); on the **title** it lies (`"日本語" is not a valid work-unit id` — *use lowercase letters, digits, and single hyphens* — at a door whose convention is free prose). The mint doors cannot be `WORK_UNIT_ID_DOORS` rows (that registry is derived ⇔ from clap's id arguments). **The shipped identity for *this title yields no id* is `write.unslugable-title`** — already multi-producer with a per-producer sentence (the engine write path; `jigc rename`), the M13-audit precedent for this exact class. The axis to iterate is **`MINT_DOORS`**, in the suite that already iterates it. |

## 4 · The forks the baseline raises

- **F1 · `(3, A3-2)` — what a failed displacement does.** The charter's shape cannot be built as written
  (§3). Genuinely forked, cheap-vs-robust; a `robust-advocate` ran. → settle-record.
- **F2 · `(3, A3-1)` — `merged/**`.** A declared M52 bound, or a predicate to test? Cheap-vs-robust; a
  `robust-advocate` ran. → settle-record.
- **F3 · `(5, DEFECT 1)` — the identity and the seam.** `write.unslugable-title` with a third producer vs
  the charter's `work-unit.malformed-id`; the sub-task guard at `engine::milestone::add_task` (covers
  `add-from-spec`) vs the CLI doors. Not cheap-vs-robust — one arm is inert or false. → settle-record.
- **F4 · `(2, DEFECT A)` — disjointness, route, noun.** Three small decisions, none mechanism.
  → settle-record.

## 5 · What the orchestrator re-drove (the claims the forks turn on)

| claim | command | observed |
|---|---|---|
| `git reset` clears the pending message and keeps the pick | fresh repo · `git cherry-pick -n side` · `git reset -q` | pick rc 0, `MERGE_MSG` present alone; after `git reset`: rc 0, `MERGE_MSG` gone, `?? p` (the payload, unstaged) |
| the charter's predicate overlaps `Rebase` | `dev/jigc-rig committed-singletons --git-state rebase-merge` · list the git dir | `MERGE_MSG` + `rebase-merge` — none of the four negated markers |
| a left-in-place area lies | `fresh` rig · `start --workflow quick-fix` · snapshot the area · `task finalize` (exit 0, `0c5490b` landed) · restore the area · `task list` | *jigc task list — 1 active task(s) … tidy-the-readme*; re-finalize → `finalize.empty-commit` |
| neither `Displace` door carries a code | `command grep -n 'codes: &\[\]' crates/cli/src/milestone.rs` | `:3248`, `:3266` |
| the safe primitive and the registries ship | `command grep` in `engine/src/state.rs` | `MILESTONE_AREA_FILES` `:168` · `foreign_area_paths` `:262` · `unwind_area` `:385` |
| `write.unslugable-title` ships with two producers; the milestone fallbacks are unguarded | `command grep` | `rename.rs:201`, `write.rs:6300`; `slug.is_empty()` at `engine/milestone.rs:729`, `:2472`, `state.rs:1317` |
