# Row 6 · write surface — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. Source of the row: M55's S16
axis *write surface* — the `new: true` create-gate entry at both create doors, and the
`write.title-ignored` route. **A new brief — no baseline; first drive.** Its nearest numbered axis
is 1 (caller tokens), whose subject (the path a token becomes) is not re-opened here.

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it. Expected here, not a
  finding — row 10 owns it.
- **Keys:** a new finding is keyed `(R6, <id>)` — **not** `(6, …)`, which is numbered axis 6's key
  space (composed surfaces).
- Under `.jigc/` use `command grep` with a before-control — *nothing staged after a refusal* is a
  claim about `.jigc/tasks/<id>/docs/`, exactly where the harness `grep` lies.

## READ FIRST

- **Baseline: none — first drive.**
- The contract: `design/write-commands.md` → *The verbs* (the title pre-check), → *Instance
  provisioning*, → *The create-gate*, → *`jigc doc rename`*; `design/findings-channel.md` §4 (whole)
  and §10's `create.already-exists`, *unknown `allows-create` entry key* and `write.title-ignored`
  rows; `design/command-output-contract.md` → *The stable finding key* (the `create.*` rows — the
  doctype-scoped ones key at the bare doctype id, the instance-scoped ones at the `type:slug`
  address); `design/workflow-dialect.md` → *On-disk definition format*.
- What changed: `completions/artifacts/M55/VERDICT.md` — scenarios 9, 10, 23 and *The two
  observations* (O23); `design/worked-examples.md` → flow 57.

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| `VERB_KINDS`, the `doc` × `Write` leaves | `crates/cli/src/cli.rs` | 8: `create` · `add-item` · `remove-item` · `retitle-item` · `rename` · `set-field` · `set-slot` · `author`; the **create doors** are `create` and `author` |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs` | 16 rows |
| `SLUG_DOORS` | `crates/cli/src/cli.rs` | 6 |
| `MINT_DOORS` | `crates/engine/src/state.rs` | 6 |
| `allows-create` entries, both packs | the front-matter of `crates/cli/packs/{dev,methodology}/workflows/*.yaml` | 34 entries over 29 of 39 workflows; **2 carry `new: true`** — `report-jigc-feedback` (`jigc-feedback` as `feedback`) and `report-inconsistency` (`inconsistency` as `inconsistency`) |
| `AllowsCreate`'s keys | `crates/engine/src/compose.rs` | 3, closed: `type`, `as`, `new` |
| the `create.*` family | grep `crates/cli/src`, `crates/engine/src` (no registry) | 6 literals: `already-exists` · `empty-title` · `gate-blocked` · `serial-collision` · `singleton-copy-in` · `unknown-doctype` |
| `PayloadReject::ALL` (the `doc author` payload's refusals) | `crates/cli/src/author.rs` | read the count |

## CELL SET

The matrix is `entry × door × identity × title × role`; drive every cell of the first two axes
against the identity and title axes, and the role axis where it discriminates.

- **entry** — carries `new: true` (a `report-inconsistency` or `report-jigc-feedback` task) · does
  not (a `park-idea`, `record-decision` or `single-task` task).
- **door** — `jigc doc create <type> --title …` · `jigc doc author` (payload carrying `title:`).
- **the minted identity** — absent · **committed** at its home · an **untracked file** placed at its
  home · staged by **this same task** (the create re-run — idempotent?) · staged in **another open
  task** · already **copied in by this task through another verb** (`set-field` on the committed
  doc, then the create).
- **title** — the same title · a **different** title that slugs onto the same id (punctuation,
  case, an edge stop-word) · `--slug <distinct>` · `--slug <the occupied id>`.
- **role** — unbound · already bound to a doc this task created (the one-doc-per-role rule).

For each refused cell record: exit · code · the `(code, target)` key under `--format json` · the
route · **nothing staged** (`ls`/`command grep` under the task's `docs/`, with a before-control) ·
the committed doc's checksum unchanged · then **run the route verbatim** and record where it lands.

Also:

1. **Order of refusals** — which of `create.already-exists`, `write.title-ignored`,
   `write.identity-change`, `create.gate-blocked`, `create.serial-collision`,
   `create.empty-title` answers when two apply.
2. **The general-case route** — under an entry without the key, a different title onto an existing
   id: `write.title-ignored`, and what its route names.
3. **A singleton / placement doctype** under each entry kind (`vision`, `roadmap`, `changelog`): the
   create-or-update path planning depends on is unchanged.
4. **The fan-out join** — two sub-tasks of a `new: true` workflow minting one slug in isolation:
   the join's suffixing, in both filing orders.
5. **Project shadows of the entry** — the key dropped · the key added to a workflow that shipped
   without it · the key misspelt · a stray extra key: what loads, what refuses, and at which doors.
6. **After the refusal** — can any other `doc` write leaf of the same task still reach the existing
   doc (`set-field`, `set-slot`, `add-item` … on its address)? Record what each does; grade it
   against `design/findings-channel.md` §1.5's stated convention and §4's stated scope, which is
   what decides whether it is a defect or the declared bound.

## BASELINE ROWS TO RE-DRIVE

None. One context note, so a declared bound is not filed as a new finding: that any task may
`set-field`/`set-slot`/`remove-item` on a committed doc whatever its workflow's `allows-create`
(the edit gate is parked — `design/findings-channel.md` §1.5) is on the record as a **bound**.
Cell 6 measures where that bound sits on rc.24; it does not re-discover it.

## RIG STATES

- **`fresh`** — the base: land a first finding through each report workflow, then open the second
  task that collides with it. Also `park-idea` for the general case (land an idea first).
- **`committed-singletons`** — the singleton / placement cells.
- **`refs-post-hoc`** — a live task already holding a copy of a committed doc (the *staged in
  another open task* and *copied in by another verb* identities, on a singleton).
- **Non-rig fixtures, named at their cells:** the untracked file at a home is a plain file written
  into the worktree (`docs/…`, not `.jigc/`); the entry shadows of cell 5 are hand-written files
  under `.jigc/config/workflows/` — the tracked project layer, the one stated exception to *never
  write into `.jigc/`* — committed with plain git; the fan-out is `jigc milestone create` →
  `add-task … --workflow report-inconsistency` ×2 → `provision` → file in each worktree → `join` →
  `finalize`.

## ENVIRONMENT NOTES

- **This host's filesystem is case-insensitive and case-preserving** (APFS default). A cell about a
  title differing only in case is therefore a cell about *this* filesystem; say so, and do not
  generalise its result to a case-sensitive one — that half is NOT DRIVEN here (it is a
  `dev/runner-faithful` cell, and only if Docker answers and you have a way to run the installed
  build there; otherwise say it was not driven).
- A task id is minted from the intent. Give the address to the `doc` verbs and plain words to
  `jigc start` — an address-shaped intent mints a mangled id, which is a row-8 surface, not yours.
- Read each exit bare. A blocked write is exit 1; a task-scope gate is exit 3; clap is exit 2.
