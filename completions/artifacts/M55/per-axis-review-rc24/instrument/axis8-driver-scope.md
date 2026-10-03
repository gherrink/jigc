# Row 8 · composed surfaces — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. This row is **numbered axis 6
(composed surfaces), scoped** to M55's sub-task composition (S2) and its four workflows. Baseline:
**rc.19**.

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it. Expected here, not a
  finding — row 10 owns it. Composed text is not a commit and carries no trailer.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim; a new finding is keyed `(R8, <id>)`
  — **not** `(8, …)`, which is numbered axis 8's key space (adopter docs & help).
- Under `.jigc/` use `command grep` with a before-control.

## READ FIRST

- **The baseline:** `completions/artifacts/M53/per-axis-review-rc19/README.md` — *Layer D — axis 6*
  (seven rows: `(6, D-1)` … `(6, D-4)`, M51 `A6-1` … `A6-3`, M52's open leads), §A's `(2, N-1)` =
  `(6, A6-R1)`, §C's `(6, L-1)` … `(6, L-7)`, §D's observations — and
  `completions/artifacts/M53/per-axis-review-rc19/axis-6.md` for the repro blocks. The rc.20 run did
  **not** re-drive axis 6, though it closed `(2, N-1)` = `(6, A6-R1)` from the axis-2 side
  (`completions/artifacts/M53/per-axis-review-rc20/README.md` → Layer A) and its usability batch
  touched `(6, D-1)`.
- The contract: `design/workflow-dialect.md` → *Emitted format* (the sub-task omission and trailer)
  and → *On-disk definition format*; `design/findings-channel.md` §2 (the four workflows), §6 (the
  S2 row and its two declared bounds), §10's S2 rows, and Open question 2;
  `design/command-catalog.md`; `design/introspection.md` (the `describe` projection, `suppressed`);
  `design/surface-contract.md` → *The suppression fence*; `design/write-commands.md` → *Task
  origination* and → *Sub-agent re-entry*; `completions/artifacts/M51/acceptance-design.md` Part 2,
  the axis-6 row.
- What changed: `completions/artifacts/M55/VERDICT.md` — E1, E2, *Project structural-op deltas
  applied on a fresh compose only*, scenarios 3, 11, 12, 14–17, *Not run* (the ~40 unprefixed
  *finalize* mentions), *Declared bounds*; DECISIONS.md → *M55 Increment 3 planning* and →
  *Increment 3 / T2*.

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| callers of `render::composed` | `crates/cli/src/cli.rs`, `migrate.rs`, `milestone.rs`, `task.rs` | 8 call sites over **5 verbs**: `start` (compose · named compose · named with no intent · resume) · `workflow` (re-entry / preview) · `migrate` · `milestone execute` · `task amend` |
| workflows shipped | `crates/cli/packs/{dev,methodology}/workflows/` | 18 + 21 = 39 |
| the `suppressed.door` (verb-routed) set | grep `^  door:` in those files | 15 members |
| steps shipped | `crates/cli/packs/{dev,methodology}/steps/` | 31 + 44 |
| steps carrying `{{ cli.finalize-task }}` directly | grep the step trees | 4: dev `finalize`, dev `amend-message`, methodology `finalize`, methodology `finalize-doc-only` — **the omission set is derived and wider** (wrappers by inclusion; commit-doc authors under `squash: true`). Derive it from the composed output, per workflow, and state the set you observed |
| `OFF_CATALOG_VERBS` | `crates/cli/src/orient.rs` | 2: `planning` · `ingest-existing` |
| router-visible vs hidden among the four findings workflows | their front-matter | visible: `report-inconsistency` (`selectable: true`, a `when:` hint) · hidden: `report-jigc-feedback`, `triage-jigc-feedback`, `triage-inconsistency` (`selectable: false`, `suppressed {reason, expires: never}`, no `door`) |

**Doors:** `start` · `workflow` · `migrate` · `milestone execute` · `task amend` · `describe` ·
`doc show` · `doc list` · `task validate`.

## CELL SET

**The numbered axis's five cells stay** — the step text names a verb that answers · the `resume:`
line's real states · orientation's states · the catalog one-liner matches the verb's behaviour · the
off-catalog reason is stated — driven here **over the four new workflows and the sub-task
compositions**, not re-swept over all 39.

**For this run:**

1. **Sub-task composition, every workflow** — for each `creates-task: true` workflow of both packs
   that can be a sub-task's mint workflow, composed for a fan-out sub-task (its `Spawn:` line run
   verbatim from the provisioned worktree) under `finalize.fan-out.squash` = `true` **and** `false`:
   no `Run:` line naming `jigc task finalize`; no line telling the agent to author, render or approve
   a commit doc where no boundary reads one; the sub-task trailer present and naming `jigc milestone
   finalize <m>`; and the composed text names **no** refused per-task door anywhere (count the
   occurrences; quote any hit with its step). The control: the same workflow composed top-level
   keeps those steps.
2. **The verbs the surviving text names answer** — for one code-carrying and one docs-only sub-task
   under each squash value, follow the composed text to the join: `milestone join` then `milestone
   finalize` land, in both completion orders.
3. **Project-layer interference** — a project step that wraps a member of the omission set · a
   whole-file shadow of a member's body · `jigc config replace-step` / `insert-step` / `remove-step`
   touching one — at a fresh compose, on resume (`jigc start --task`), at sub-agent re-entry
   (`jigc workflow <W> --task <sub>`) and at `milestone execute`.
4. **The knob flipped between compose and join** (`true` → `false`): the join's routed block, and
   what re-composing through the `Spawn:` line then shows.
5. **The four findings workflows** — bare `jigc start` orientation and the router's catalog list
   `report-inconsistency` and none of the three hidden ones; each hidden one composes by name
   (`jigc start --workflow <id> "<intent>"`) and previews (`jigc workflow <id> --preview`);
   `jigc describe` (and `--format json`) carries each one's `suppressed` reason; the `when:` hint and
   the one-liners say what the workflow does (file one doc; land it alone).
6. **The triage intent form** — the step's stated instruction against what an address-shaped intent
   actually mints (a control with plain words beside it).
7. **Each composed text's commit promise against the commit it gets** — a report/triage task
   top-level (path-scoped) vs the same workflow as a sub-task (the join's commit): the `what's-left:`
   line and the step text say which, truthfully, in both.
8. **THE DECLARED BOUND — re-confirm, expected to hold as declared.** Under `squash: true`,
   `jigc task validate <sub>` exits **3** on the omitted commit doc (`commit:<sub>`'s unfilled
   required leaves) while the composed text never asks for it and the join lands without it
   (`completions/artifacts/M55/VERDICT.md` → *Declared bounds*; scenario 17). Drive it, record the
   exit, the codes, the route it prints and **where that route leads**; then the same under
   `squash: false` for a docs-only and a code-carrying sub-task. State **HOLDS AS DECLARED** or what
   differs. It is a bound, not a new finding — unless what you drive is *wider* than the sentence
   that declares it, in which case file the difference, not the bound.

## BASELINE ROWS TO RE-DRIVE

Keys quoted from `completions/artifacts/M53/per-axis-review-rc19/README.md`:

| key | tier there | verdict there | note |
|---|---|---|---|
| `(6, D-1)` (M52) — orientation reports a live task's findings without the repository posture | 2 | STILL-OPEN(1.x, expected) | rc.20's usability batch changed this surface (*a live task's findings carry the repository posture finalize refuses under*) and no axis-6 run has driven it since — CLOSED (argv) or STILL-OPEN (datum) |
| `(6, D-2)` (M52) — `fix-task` is composable by name and the composed walk's forbidden door is the only one that works | 2 | STILL-OPEN(1.x, expected) | re-drive |
| `(6, D-3)` (M52) — the orientation `Preview:` footer states the pre-M52 rule | 3 | STILL-OPEN(1.x, expected) | re-drive |
| `(6, D-4)` (M52) — a legitimately-empty `milestone execute` walk does not state its empty case | 3 | STILL-OPEN(1.x, expected) | re-drive |
| `(2, N-1)` = `(6, A6-R1)` (rc.19) — the hook announces a rename on a commit containing none | 3 | CLOSED on rc.20 (axis 2) | one drive, to carry the closure onto this axis's record |
| M51 `A6-1` · `A6-2` · `A6-3` | — | still CLOSED | `A6-2` over the **re-derived** `suppressed.door` set (15 now — `amend` joined) × the three argv forms; `A6-1` and `A6-3` where a cell above passes through them |
| `(6, L-1)` … `(6, L-7)` | lead | open | re-disposition each: `L-1` (non-UTF-8 path) is unbuildable on this filesystem — restate the reason, do not retry it blindly; `L-2` … `L-5` are driven-and-declared — one re-drive each; `L-6`, `L-7` declared not driven — say whether this run reached them |

There is **no tier-1 row** on numbered axis 6.

**Do not conflate two neighbouring rows of other axes with cell 8.** `(2, A2-2)` =
`(5, DEFECT 1 · rc.20)` (tier 2, rc.20) is also `jigc task validate` in a fan-out worktree, but its
datum is the **posture** (`repo.head-detached` at the committing door while the previews say clean).
Cell 8's datum is the **commit doc**. If you see `repo.head-detached`, you are looking at the rc.20
row, which this row does not re-drive; record that you saw it and move on.

## RIG STATES

- **`fresh`** — the base for every fan-out: `jigc milestone create "<title>"` → `jigc milestone
  add-task <m> "<intent>" --workflow <W>` per sub-task → `jigc milestone provision <m>` →
  `jigc milestone execute <m>` (the `Spawn:` lines) → each sub-task from its worktree → `join` →
  `finalize`. Set the knob with `jigc config set finalize.fan-out.squash <bool>` before provisioning.
- **`committed-singletons`** — orientation and the catalog over a populated store; `migrate-*` and
  planning sub-tasks.
- **`refs-post-hoc`** — a live top-level task for the `resume:` cells.
- **`--git-state bisect`** on `fresh` or `committed-singletons` — `(6, D-1)`'s posture.
- **Non-rig fixtures, named at their cells:** project steps and workflow shadows are hand-written
  files under `.jigc/config/steps/` and `.jigc/config/workflows/` — the tracked project layer, the
  one stated exception to *never write into `.jigc/`* — or `jigc config` verbs where one exists.

## ENVIRONMENT NOTES

- **Run each `Spawn:` line verbatim, as printed, from the cwd it names** — that is the surface under
  test. Do not launch real sub-agents; you are the N-process sim, and you say so. M55's genuine
  concurrent spawn is on the record (`completions/artifacts/M55/genuine-spawn/README.md`) and is not
  re-run here.
- A sub-task worktree lives under `.jigc/worktrees/<sub>`; it is a real git worktree of the rig
  repository, **not** this repository. Check `git rev-parse --show-toplevel` before any `git`
  command you type by hand in one.
- Composed text carries the task id and absolute `cd` targets by design; when quoting it in a repro
  block, elide the host prefix up to the rig root.
- The compose goldens (`crates/cli/tests/goldens/compose/`) are the debug suite's pins, not a
  driven row; reading one to learn what a composition *should* say is fine, citing one as a drive is
  not.
