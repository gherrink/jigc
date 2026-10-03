# Row 3 · store exit codes / reconciliation — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. Source of the row: M54's S18
axis *store exit codes*; M55's S16 axis *L1, L2 and the `unmanage` route*. **A new brief** — it has
no numbered-axis file of its own, but three rows of numbered axis 7 sit on its surface and are
carried here so they are not dropped (see *Baseline rows*).

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it. Expected here, not a
  finding — row 10 owns it. A commit **you** make with plain `git commit` (the teammate's, the
  hand edit) carries none; that difference is not a datum of this row.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim; a new finding is keyed `(R3, <id>)`.
- Under `.jigc/` use `command grep` with a before-control — the file-state record and the edge index
  live there, and the harness `grep` says *not found* whether or not the bytes exist.

## READ FIRST

- **The carried baseline rows:** `completions/artifacts/M52/per-axis-review/README.md` — the
  *Axis 7 · freeze & migration* comparison table, §A's `(7, A7-F1)` and `(7, A7-F2)`, §D's axis-7
  row — and `completions/artifacts/M52/per-axis-review/axis-7.md` for their repro blocks. No M53
  partial run re-drove axis 7.
- The contract: `design/reconciliation.md` (the state machine; *Conflict — block at file level*;
  *Rename detection*; *Detection timing*); `design/validation.md` → *Exit semantics* (the paragraph
  naming `render::STORE_EXIT_FLIPS`), → *Completing the envelope*, → *The M45 registrations*;
  `design/storage.md` → *Derived caches* (which refs count); `design/findings-channel.md` §6 (L1, L2)
  and §10 (the L1 absorb · L1 store-scope arm · L2 downgrade rows);
  `design/command-output-contract.md` → *The store sweep's envelope* and → *The exit-code taxonomy*.
- What changed: `completions/artifacts/M55/VERDICT.md` — CR2, scenarios 18–19, *`jigc unmanage`
  claimed a file was left on disk*, and **F21**; DECISIONS.md → *M55 completion triage, CR2*.

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs` | 7 members: `probe-unreliable` · `oob-rename` · `unmigrated-corpus` · `ahead-corpus` · `orphaned-instance` · `home-vacated` · `foreign-squatter`. `oob-rename`'s predicate asks the code **and** the blocking severity |
| `STORE_FAMILIES` | `crates/engine/src/validate.rs` | 7 families |
| `EXIT_CODES` | `crates/cli/src/task.rs` | 5 classes |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs` | 4 |
| the two dangling-baseline producers | `crates/engine/src/file_state.rs` | `rename_dangling_baseline_finding` (a branch carries the path) and `rename_orphaned_baseline_finding` (none does) — one code, two routes |

**Doors:** `jigc validate` (store scope) · the task-scope gates L1 reaches — `jigc task validate`,
`jigc task finalize`, `jigc milestone finalize` · `jigc unmanage` · `jigc ingest` and `jigc doc list`
where they answer the same store state · the installed pre-commit hook's own store sweep.

## CELL SET

1. **Exit flips** — each `STORE_EXIT_FLIPS` member present → a non-zero exit with that member's own
   trailer, on the text arm and under `--format json`; content findings present with **no** member
   → exit 0. **Scope:** `oob-rename` (both arms) and `probe-unreliable` are this run's subject and
   must be driven; drive the other five where a rig reaches them in a few commands, and list any
   you do not as NOT DRIVEN with its reason (numbered axis 7 drove them on rc.16).
2. **L1 at store scope** — a recorded doc whose on-disk bytes differ from its file-state record and
   equal `HEAD`'s blob, reached three ways (a `git pull` from a second clone · a merge · a hand edit
   committed with plain `git commit`) × {conformant · non-conformant}: code, severity, route, exit.
3. **L1 at the task gate** — the same doc then touched by a task minted *after* the change (the base
   pin already has it) vs a change made *during* the task vs a pinned edit that fails conformance vs
   the milestone-record door; at `task validate`, `task finalize --dry-run` and `task finalize`.
   After the absorbing finalize: the committed doc carries both the teammate's line and the task's.
4. **L2** — a recorded doc absent from the worktree with no history at `HEAD`:
   while a **local** branch carries the path · only a **remote-tracking** ref does · only a **tag**
   does · **nothing** does (`git reset --hard` past the creating commit · a rebase · an amend · a doc
   ingested and deleted before it was ever committed) — and the control, absent **with** history
   (`git rm` + commit). At store scope and at a task's gate: the same code, severity, route and
   `(code, target)` key at both.
5. **Every emitted route, run verbatim** — the switch-back route; `jigc unmanage <path>`; after it,
   does the row clear at both scopes?
6. **`jigc unmanage`** — over a managed doc whose file is present · whose file is absent · a path
   jigc never managed · a second run · a symlinked path; the ack sentence about the file on disk
   against `ls`; index edges and baseline before and after (`command grep`, with a before-control).
7. **`(code, target)` parity** — every finding above on the text arm and the JSON arm.
8. **F21 — the declared open gap, to be RE-CONFIRMED, expected STILL-OPEN.** In a rig: record a
   project structural-op delta carrying a broken include (`jigc config insert-step …`), commit it,
   then `jigc validate --format json` (does it report it?) against a door that composes
   (`jigc task amend`, or `jigc start --workflow …`) — and the control: the same broken include in a
   whole-file shadow. State **CLOSED (argv)** or **STILL-OPEN (datum)**; STILL-OPEN is the expected
   answer and is a measurement, not a new finding. Then bound it: a cycle, and a bad ref inside an
   inserted native step.

## BASELINE ROWS TO RE-DRIVE

Keys quoted from `completions/artifacts/M52/per-axis-review/README.md`:

| key | tier there | what it said on rc.16 |
|---|---|---|
| `(7, A7-F1)` | 3 | the exit-flip trailer asserts a commit and a `git mv` that never happened |
| `(7, A7-F2)` (= the still-open half of M51 `(7, D-4)`) | 3 | `ingest` says *no action needed* about the files `validate` blocks on |
| `(7, C-2)` (M51) | — | the orphan territory bound misses a root-placement orphan — **a declared residual on a written trigger; STILL-OPEN is expected, do not re-file it** |
| §D axis-7 lead: `probe-unreliable`, the `STORE_EXIT_FLIPS` member no axis-7 door had driven | lead | drive it (cell 1) and close the lead or say why not |

There is **no tier-1 row** on numbered axis 7. `(7, A7-F3)` and the freeze/migration leads are row
4's; `store.no-such-leaf` is row 7's.

## RIG STATES

- **`committed-singletons`** — managed docs at placement homes; the base for most exit-flip members
  and for L1 over a placement doctype.
- **`fresh`** — land a location-doctype doc (an `inconsistency` through `report-inconsistency`, or an
  ADR) on a branch, for L1 and L2 over a location doctype.
- **`refs-post-hoc`** — a **live** task holding a staged edit of a committed doc: the DRIFTED +
  TOUCHED arm with the change made during the task.
- **`vendored`** — `probe-unreliable` (drift the anchor, point the override at a non-executable file).
- **`migrated`**, **`--pack-from-dev --repin`** — reach for these only for the exit-flip members that
  need a stale or ahead stamp.
- **Non-rig fixtures, named at their cells:** the *teammate* is a second clone of the rig repository
  in a `mktemp -d` (`git clone "$REPO" "$T"`), committing with plain git and pulled into the rig; the
  remote-tracking-only and tag-only cells delete the local branch after pushing it to that clone or
  tagging it. `crates/cli/tests/support/branch_and_pull.rs` and
  `crates/cli/tests/file_state_history_gate.rs` show each construction — reproduce them with the
  binary and plain git, never by writing into `.jigc/`.

## ENVIRONMENT NOTES

- **Ambient git config leaks into a throwaway clone.** The rig isolates `$HOME`; a second clone you
  make inherits that isolation only if you make it *after* the `eval`. Make it after.
- A plain `git commit` in a rig runs the pre-commit hook `jigc setup` installed. That is a door of
  this row (the hook's store sweep) — record what it prints, do not bypass it with `--no-verify`
  unless the cell is about the state and not the hook, and say so when you do.
- Row 2 owns *why* a probe did not run; here only its exit, trailer and key are graded.
- Exit 1 and exit 3 are different classes (`design/command-output-contract.md` → *The exit-code
  taxonomy*): read each exit bare, never through a pipe.
