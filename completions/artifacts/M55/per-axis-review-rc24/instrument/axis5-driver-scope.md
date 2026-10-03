# Row 5 · finalize / transaction — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. This row is **numbered axis 4
(transaction / rollback), scoped** to M55's doc-only commit. Baseline: **rc.16**.

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it. Expected here, not a
  finding — row 10 owns it. **It matters on this row in one way:** the seam now rewrites the rendered
  message *file* to sign it before `git commit` reads it, so when a cell compares a message, a
  message file or `git show --format=%B` across two runs, hold the variable constant — and for the
  hook-rejected cells, run the cell both with it set and with it cleared and say whether anything
  but the trailer differs.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim; a new finding is keyed `(R5, <id>)`
  — **not** `(5, …)`, which is numbered axis 5's key space (pinned contracts).
- Under `.jigc/` use `command grep` with a before-control.

## READ FIRST

- **The baseline:** `completions/artifacts/M52/per-axis-review/README.md` — the *Axis 4 ·
  transaction / rollback* comparison table (6 M51 rows), §A's `(4, DEFECT 1)`, `(4, DEFECT 2)`,
  `(4, DEFECT 3)`, §D's four axis-4 leads — and `completions/artifacts/M52/per-axis-review/axis-4.md`
  for the repro blocks and the racing-hook fixtures. No M53 partial run re-drove axis 4.
- The contract: `design/finalize.md` — *The seven phases*, *Dirty-tree policy* (→ *Surfaced, not
  prevented*), *Rollback discipline*, *The amend arm*, **The doc-only arm**, *Commit-doc rendering*;
  `design/findings-channel.md` §3 (whole) and §10's first row; `design/team-ready-state.md` → *The
  commit model* (the path-scoped mechanism it reuses); `design/command-output-contract.md` → *The M55
  additive kind* (`left-staged`); `completions/artifacts/M51/acceptance-design.md` Part 2, the axis-4
  row; `completions/artifacts/M53/f10-amend-settle.md` (the second commit model, for contrast).
- What changed: `completions/artifacts/M55/VERDICT.md` — CR1, CR3, E1, *Project structural-op deltas
  applied on a fresh compose only*, scenarios 1, 5–8, 15; DECISIONS.md → *M55 Increment 1 / T4*.

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | 11 rows over 9 verbs — **the doc-only commit is not a 12th row**; it rides `jigc task finalize` |
| `CommitModel` | `crates/cli/src/render.rs` | 3 cases: `Index` · `Amend` · `DocOnly` |
| `StagePolicy` | `crates/cli/src/task.rs` | read the count; `DocOnly` is the M55 member |
| `ManifestKind::ALL` | `crates/cli/src/render.rs` | 7: `promoted` · `modified` · `deleted` · `added` · `untracked` · `carried-over` · `left-staged` |
| `GATE_COVERAGE` | `crates/cli/src/gate_coverage.rs` | 13 members, each with a spelling per commit model where it declares one |
| `ROLLBACK_POPULATIONS` / `ROLLBACK_DOORS` | `crates/cli/src/rollback.rs` | 11 populations; read the doors |
| workflows composing `step:finalize-doc-only` | `crates/cli/packs/methodology/workflows/` | 4: `report-jigc-feedback` · `report-inconsistency` · `triage-jigc-feedback` · `triage-inconsistency` |

**Doors:** `jigc task finalize` (its `--dry-run` too) on the doc-only model — the subject; `jigc task
validate` as its preview; and, for the contrast every cell needs, the same door on the `Index` model
(a `park-idea` or `dev-task` task) and, where a cell says so, on the `Amend` model.

## CELL SET

1. **The three staging states × the two shapes** (flow B) — a code task open beside the doc-only
   task with its files {unstaged throughout · staged **before** the doc-only task started · staged
   **after** it started}, for a **report** (create) and for a **triage** (edit a committed finding):
   the commit holds the doc alone (`git show --stat HEAD`), `git diff --cached --name-status` is
   byte-identical before and after, the code task then finalizes its own files. Each state also with
   `--carry-staged` (inert).
2. **A pending `.jigc/config` delta** staged or unstaged beside the task — not in the doc-only commit.
3. **Preview == door** — for every state in 1–2: `jigc task validate`, `jigc task finalize --dry-run`
   and the real finalize agree on the path set, the left-out set and the findings.
4. **The left-out narration** — every staged path the commit left out appears once, as `left-staged`,
   on the text arm and in `--format json`'s `left_out[]`; an **unstaged** edit and an **untracked**
   file beside it carry their own kinds; nothing the commit included is listed.
5. **The omitting-context control** — a code-less task on the ordinary model (`park-idea`): state 2
   refuses with `finalize.carried-staged`, state 3 commits the staged paths — unchanged behaviour.
6. **Failure points on this arm** — a rejecting `pre-commit` hook, a rejecting `commit-msg` hook:
   `HEAD`, `HEAD^{tree}`, the index and the staged doc as they were; the promoted doc's destination
   rolled back; the task still live; a re-run after removing the hook lands once. An **unchanged
   recording** (a triage writing the filed bytes back): refused before git runs, and under which
   code. A hook that edits the promoted doc during the commit (the racing-hook fixture of the
   baseline): what survives, and is a rollback-conflict finding emitted?
7. **Owner-artifacts** — a doc-only task with a recorded owner-artifact {tracked and changed ·
   untracked · unchanged}; a path whose name carries pathspec metacharacters (`*`, `[`, a leading
   `:`): only the named file is committed.
8. **How the step is reached** — through a wrapping project step · with its body shadowed · swapped
   in or out by `jigc config replace-step` / `insert-step` / `remove-step` — at a fresh compose, on
   `jigc start --task <id>` resume, and at finalize: the composed promise and the commit model agree.
9. **A fan-out sub-task** of a doc-only workflow — never doc-only: the composed `what's-left:` line,
   and what the join commits when the sub-task's worktree has a staged code path.
10. **The index-gate member's spelling** on each of the three models, at `task validate`, `--dry-run`
    and the composed `what's-left:` line.

## BASELINE ROWS TO RE-DRIVE

Keys quoted from `completions/artifacts/M52/per-axis-review/README.md`:

| key | tier there | what it said on rc.16 | note |
|---|---|---|---|
| `(4, DEFECT 1)` | 2 | `finalize.stage-failed`'s route is not copy-runnable | M53's usability batch set out to close it; no axis-4 run has driven it since — CLOSED (argv) or STILL-OPEN (datum) |
| `(4, DEFECT 2)` | 3 | `config.repoint-failed` renders an absolute host path | expected STILL-OPEN(1.x) unless a later arc swept it — say which |
| `(4, DEFECT 3)` (= the still-open half of M51 `(4, C4)`) | 3 | a hook-rejected `milestone create` leaves its `.jigc/.gitignore` amend on disk, unacknowledged | expected STILL-OPEN(1.x) |
| §D axis-4 leads | lead | CL-4's stale-base ordering leg · cell D in full (stage failure × worktree concurrently edited) · a genuine concurrent process racing a pre-image · `milestone provision`'s post-`ensure` failure point | re-disposition each: now driven, or still open with the reason; none is promotable on a source read |

There is **no tier-1 row** on numbered axis 4. The five M51 rows axis 4 CLOSED on rc.16 (`C1`, `C2`,
`C3`, `DEFECT 1`, `DEFECT 2`) are re-driven **on the doc-only arm** by cell 6 — that is the row's
question: does the rollback discipline they proved on the `Index` model hold on the third model?

## RIG STATES

- **`fresh`** — the base for every flow-B cell. Mint the code task with `--start dev-task "<intent>"`
  (or through the binary) and the doc-only task with `jigc start --workflow report-inconsistency
  "<what>"`; the triage shape needs a finding landed first.
- **`committed-singletons`** — the `Index`-model contrast over committed docs; the racing-hook cells
  over a placement doc.
- **`chatty-hooks`** — a non-blocking hook's stream on a doc-only commit (it replaces jigc's own hook).
- **Non-rig fixtures, named at their cells:** rejecting and racing hooks behind `core.hooksPath` in a
  `mktemp -d`; the fan-out is `jigc milestone create` → `add-task … --workflow <W>` → `provision` →
  work in `.jigc/worktrees/<sub>` → `join` → `finalize`. Project steps and workflow shadows for cell 8
  are hand-written files under `.jigc/config/` — the tracked project layer, the one stated exception
  to *never write into `.jigc/`* — or `jigc config` verbs where one exists; say which at the cell.

## ENVIRONMENT NOTES

- **Record the index, not just `HEAD`.** This arm's whole claim is about what stays staged; every
  cell records `git diff --cached --name-status` before and after, read bare.
- The pre-commit hook `jigc setup` installed runs on these commits and may print a doc↔code advisory
  or the staged-elsewhere stem; on this arm its wording is model-keyed — grade it against
  `design/finalize.md` → *The doc-only arm*.
- A deterministic in-transaction racer other than a hook does not exist in the fixture library
  (the baseline's own open lead). Do not simulate one; keep that lead open with its reason.
- `jigc task finalize --force` does not exist (clap exit 2) — not a cell.
