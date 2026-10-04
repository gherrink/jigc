# Row 7 · pinned read contracts — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. This row is **numbered axis 5
(pinned contracts), scoped** to the read surface M55 extended. Baseline: **rc.20**.

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it. Expected here, not a
  finding — row 10 owns it. No key this row reads carries a commit message.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim — and note the rc.20 record's own
  keying: its two new axis-5 findings are **`(5, DEFECT 1 · rc.20)`** and **`(5, DEFECT 2 · rc.20)`**,
  distinct from the older `(5, DEFECT 1)` and `(5, DEFECT 2)`. A new finding of this run is keyed
  `(R7, <id>)`.
- Under `.jigc/` use `command grep` with a before-control.

## READ FIRST

- **The baseline:** `completions/artifacts/M53/per-axis-review-rc20/README.md` — *Layer C — axis 5*,
  §A's two axis-5 findings, §C's `(5, C-13)` and `(5, C-18)` — and
  `completions/artifacts/M53/per-axis-review-rc20/axis-5.md` for the repro blocks, the registry
  counts and the row-state fixtures (how it built an `unregistered` and an `orphaned` row).
- The contract: `design/doc-read-surface.md` (whole — *The pinned `--format json` contract*, the
  whole-doc key list, the `doc list` row table, *Evolution posture*, the paragraph opening *M55
  spends the window again*, the `--task` arm); `design/findings-channel.md` §5 and §10's
  *absent-default projection* row; `design/command-output-contract.md` → *Every other envelope* and →
  *Evolution posture*; `completions/artifacts/M51/acceptance-design.md` Part 2, the axis-5 row.
- What changed: `completions/artifacts/M55/VERDICT.md` — scenarios 2, 20, 24 and *The two
  observations* (O24).

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs` | 66 arms over 48 leaves — **unmoved since rc.20** (59 `Pinned` · 7 `Unpinned`); `doc show` 8 arms · `doc schema` 1 · `doc list` 1 |
| `VERB_KINDS` | `crates/cli/src/cli.rs` | 48 leaves |
| `DOC_READ_VERBS` | `crates/cli/src/doc.rs` | 3: `show` · `schema` · `list` |
| `WHOLE_DOC_KEYS` | `crates/cli/src/doc.rs` | 7: `type` · `slug` · `title` · `item-count` · `schema-version` · `fields` · `sections` (+ `staged` on a staged serve) |
| `DocRow` (a `doc list` row) | `crates/cli/src/doc.rs` | 6 keys: `id` · `path` · `state` · `item-count` · `title` · `fields` |
| `doc schema`'s `contract-version` | `crates/cli/src/doc.rs` → `SchemaContract` | 7 (unmoved) |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs` | 4 |

**Doors:** `jigc doc show` · `jigc doc list` · `jigc doc schema`. The rest of the registry is the
numbered axis's door set and is **not** re-driven by this row — say so in your bounds; do not sweep
all 48 leaves.

## CELL SET

1. **`doc show`, whole doc** — committed and staged (`--task`): the driven top-level key set equals
   `WHOLE_DOC_KEYS` (+ `staged`), for at least one doctype of each shape class (a per-instance
   `id-from: title` doctype — both new ones; a singleton; a placement doctype; one with a
   repeatable section). `title` is the `# H1`; a doc whose H1 was removed by hand → `null`.
2. **No `title` on a slice** — every `#fragment` arm (the six slice arms of the registry) carries
   no object to hang it on: shape unchanged against the baseline.
3. **`doc list` rows × row state** — `managed` and parses · `managed` and does not parse (a
   stale-shape or hand-broken doc) · `unregistered` · `orphaned` · a `--task` staged row — for each:
   the row's key set, and `title` / `fields` valued as `design/doc-read-surface.md`'s table says
   (`fields` is `null`, never `{}`, on every row but a managed one that parses). With and without a
   `<type>` argument.
4. **The default projection** — a defaulted header field **absent from the stored bytes**
   (hand-delete `status:` and commit): `doc show` and `doc list` both report the default, on the
   committed serve and on a staged copy; the file's bytes are untouched after the reads
   (checksum before/after); for `jigc-feedback`, `inconsistency` **and** a pre-M55 doctype with a
   defaulted field (`adr`'s `status`). And the two controls: an absent field with **no** default,
   and a present field whose value differs from the default.
5. **`doc schema`** — `contract-version` unmoved; the projection of `jigc-feedback` and
   `inconsistency` in all three formats (fields, defaults, the `sides` item block, `home`,
   `identity`); the key set equal to the registry's row.
6. **The plain and human arms** say what the JSON arm says (title shown, defaults shown or not —
   record which; the contract is the JSON, the text arm is graded against law 1 only).
7. **The read is a read** — `git status --porcelain`, the file-state record and the edge index are
   byte-identical before and after every cell above (`command grep` / checksum, with a control).
8. **Reject funnels on these three verbs** — unknown type · unknown slug · a malformed address ·
   an undeclared leaf · a declared-but-unpopulated optional leaf · `--task` naming no task: `error`
   vs the findings envelope, the `(code, target)` key, the exit.
9. **`--help`** of `doc show` and `doc list` — the key list rendered equals the driven key set.
10. **The staged-elsewhere note** on `doc list <type>` (O24's surface) — with an open task that
    stages a doc of that type, and one that stages only another type.

## BASELINE ROWS TO RE-DRIVE

Keys quoted from `completions/artifacts/M53/per-axis-review-rc20/README.md`:

| key | tier there | verdict there | re-drive here? |
|---|---|---|---|
| `(5, DEFECT 1)` (rc.17) — a mint door commits a record at a fabricated identity | **1** | CLOSED (stays closed) | **yes — the baseline's one tier-1 row is always re-driven**, at both mint doors it names |
| `(5, DEFECT 4)` (rc.17) — `doc show` blocks a declared but unpopulated optional leaf (`store.no-such-leaf`) | 3 | STILL-OPEN(1.x, expected) | **yes** — inside the subject; M55's default projection sits right beside it, so say precisely what changed and what did not |
| rc.17 `DEFECT A` — `store.unknown-type` emits a URI-shaped target at `doc show`, the bare id at `doc schema` | 3 | STILL-OPEN(1.x, expected) | **yes** — inside the subject |
| `(5, DEFECT 3)` (rc.17) — the colon-less-address bail carries no code, two producers | 3 | STILL-OPEN(1.x, expected) | **the `doc show` half only**; the `rename` half is outside the subject |
| `(5, DEFECT 2)` (rc.17) — `config remove-step` / `replace-step` refuse a step in the resolved list | 3 | STILL-OPEN(1.x, expected) | no — outside the subject (a `config` write door) |
| `(5, C1)` (rc.19) — orientation rows declare `next_steps`, a reachable composition omits it | 3 | STILL-OPEN(1.x, expected) | no — outside the subject (`start`) |
| `(5, D1)` (rc.19) — `start --explain` emits an arm `ENVELOPE_ARMS` does not carry | 3 | STILL-OPEN(1.x, expected) | no — outside the subject (`start`) |
| `(5, DEFECT 1 · rc.20)` = `(2, A2-2)` — in a fan-out worktree the previews report a clean posture the door refuses | 2 | new on rc.20 | no — outside the subject (posture; `task validate` / `start`); row 8 is told not to conflate it with M55's declared bound |
| `(5, DEFECT 2 · rc.20)` — `amend.head-shape`'s locus prints `work-unit:<id>` | 3 | new on rc.20 | no — outside the subject (`task amend`) |
| M51 `DEFECT A · B · C · D` (the hostile-cwd sweeps) | — | all still CLOSED | the three read verbs' cells only (outside any repository; cwd deleted) |
| §C `(5, C-13)`, `(5, C-18)` | lead | open (not drivable in release posture) | no — unchanged reason; restate it |
| §D axis-7 lead (M52): `store.no-such-leaf`, the 2nd `ENVELOPE_OWED_CODES` member | lead | open on rc.16 | **yes** — it is the code `(5, DEFECT 4)` rides; one drive closes or keeps both |

**The rows marked *no* are listed so the assembler does not read their absence as closure:** they
were open on rc.20, nothing in this run re-drives them, and they stay where rc.20 left them.

## RIG STATES

- **`fresh`** — land one `jigc-feedback` and one `inconsistency` (with three `sides`) through their
  report workflows; then the hand-deletions of cell 4 (a plain edit plus `git commit` — the
  out-of-band path, deliberately).
- **`committed-singletons`** — singleton / placement / repeatable / nested-repeatable shapes.
- **`refs-post-hoc`** — a live task with a staged copy of a committed doc (`$RIG_TASK`): the
  `--task` arms.
- **`migrated`** — a doc landed through `migrate … --approve`.
- **Row-state fixtures** (`unregistered`, `orphaned`, *managed and does not parse*): build them the
  way the baseline `axis-5.md` records — a plain foreign `.md` written at a managed home in the
  worktree; a doctype removed from the resolved set through a pack copy; a hand-broken managed doc
  committed with plain git. Name the construction at the cell.

## ENVIRONMENT NOTES

- Compare **key sets** and **value shapes**, not whole envelopes byte-for-byte, across rigs: paths,
  dates and slugs differ per rig by construction.
- `jq` is on this host; `jq -S 'keys'` is the instrument for a key set. Never read an exit through
  the pipe — capture the JSON to a file, read `$?`, then query the file.
- The four proof fences (`crates/cli/tests/format_json_success_axis.rs`) are `#[test]` targets on the
  **debug** build: not drivable in this posture, the same reason rc.20's `(5, C-18)` gives. Do not
  run them to "confirm" a driven row.
