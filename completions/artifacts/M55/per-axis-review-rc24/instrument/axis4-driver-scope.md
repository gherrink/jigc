# Row 4 · pack-load / manifest freeze · migration — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. This row is **numbered axis 7
(freeze & migration), scoped** to what M54 and M55 changed. Baseline: **rc.16**.

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it. Expected here, not a
  finding — row 10 owns it. `jigc migrate-corpus` is a committing door, so its migration commit
  carries it too.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim; a new finding is keyed `(R4, <id>)`
  — **not** `(4, …)`, which is numbered axis 4's key space (transaction / rollback).
- Under `.jigc/` use `command grep` with a before-control.

## READ FIRST

- **The baseline:** `completions/artifacts/M52/per-axis-review/README.md` — the *Axis 7 · freeze &
  migration* comparison table (6 M51 rows), §A's `(7, A7-F3)`, §D's axis-7 row (nine leads in one
  row) — and `completions/artifacts/M52/per-axis-review/axis-7.md` for the repro blocks and the
  fixtures. No M53 partial run re-drove axis 7.
- The contract: `design/corpus-migration.md` (→ *The freeze — declared and enforced*, → *The
  freeze-exempt sibling*, → *Prior-schema sourcing*, → *The classifier's holes*);
  `design/overrides.md` → *Authored metadata on a definition resolves by whole-file shadow*;
  `implementation/module-layout.md` → *The dev pack's home* and → *Pack distribution*;
  `implementation/doctype-map.md` → *The v1 freeze* and the two new doctypes' rows;
  `design/workflow-dialect.md` → *On-disk definition format* (the `allows-create` entry's closed
  keys); `design/findings-channel.md` §1.7 (registration), §9 (the hint fold), §10;
  `completions/artifacts/M51/acceptance-design.md` Part 2, the axis-7 row.
- What changed: `completions/artifacts/M54/VERDICT.md` (scenario 7), `completions/artifacts/M55/VERDICT.md`
  (scenarios 10, 21, 22; *Not run* → the absent-manifest-entry arm).

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| the embed seam | `crates/cli/src/pack_builtin.rs` | 2 embedded packs, one module |
| dev manifest entries | `crates/cli/packs/dev/config/schema-manifest.yaml` | 6 |
| methodology manifest entries | `crates/cli/packs/methodology/config/schema-manifest.yaml` | 13 (12 persisted + the transient `commit` shadow); `jigc-feedback` and `inconsistency` are the two M55 rows |
| shipped schemas | `crates/cli/packs/{dev,methodology}/schemas/` | 6 + 13 files |
| prior-shape snapshots | `crates/cli/packs/{dev,methodology}/schema-snapshots/` | 2 + 4; none for either new doctype |
| `SchemaChangeKind::ALL` × `LOCI` | `crates/engine/src/schema_diff.rs` | 18 × 3 = 54 cells |
| `RelocateRefusal::ALL` | `crates/cli/src/relocate.rs` | read the count |
| `AllowsCreate`'s keys | `crates/engine/src/compose.rs` | 3: `type`, `as`, `new` — closed |
| workflows shipped | `crates/cli/packs/{dev,methodology}/workflows/` | 18 + 21 = 39; 34 `allows-create` entries over 29 of them |

**Doors (the numbered axis's eight):** `describe` · `doc schema` · `validate` · `doc list` ·
`migrate-corpus` · `migrate` · `ingest` · `unmanage` — plus `relocate` for the new doctypes, and at
least three other leaves of your choice for the *"the freeze assert blocks at every door that loads
a pack"* cell (name which).

## CELL SET

1. **The pack move** — the two embedded packs resolve with no `JIGC_PACK_DIR` (every door above
   answers on a `fresh` rig) · a `JIGC_PACK_DIR` pack copy still resolves · a pack copy with a
   resource removed (`pack-resource-missing`, at ≥ 2 doors) · `describe` names both packs and their
   version · nothing was written outside the repository by `setup`.
2. **The two new doctypes at schema-version 1** — `doc schema jigc-feedback` and `doc schema
   inconsistency` in all three formats (the stamp version, the home, the identity) · a filed doc of
   each carries the stamp · `migrate-corpus` and `migrate-corpus --dry-run` over a corpus holding
   both (nothing to migrate; the triage keys) · `relocate jigc-feedback --from …` and `relocate
   inconsistency --from …` (frozen: refused, routed where?) · `migrate <path> --as jigc-feedback`
   and `--as inconsistency` (is there a `migrate-*` workflow? what does the door say?) · `ingest`
   over a conformant and a non-conformant hand-placed file at each home · `unmanage` of one.
3. **The freeze over the new doctypes** — a project-layer whole-file shadow that **reshapes** either
   schema (an enum member added · a field removed · the `location` changed) blocks at pack-load, at
   every door you drive it at, naming the hash mismatch · a **prose-only** shadow (`description:`,
   `usage:`, a slot `hint:`) loads clean.
4. **The closed entry keys** — a project shadow of a workflow whose `allows-create` entry carries a
   misspelt key (`nwe: true`) or a stray one: refused at load, naming the key, at `start`,
   `workflow --preview`, `describe` and `validate`.
5. **The hint rewords** — `jigc doc schema planning-record` shows the reworded `cheap-vs-robust`
   and `claim-driven` hints; a `planning-record` filed and committed by this binary validates clean;
   `migrate-corpus --dry-run` reports nothing to do for it (no version moved).
6. **The numbered axis's stamp states, on the new doctypes only** — unstamped managed · stamp ahead ·
   an unadopted foreign file at the home · an orphaned instance: what `validate`, `doc list` and
   `ingest` each say.

## BASELINE ROWS TO RE-DRIVE

Keys quoted from `completions/artifacts/M52/per-axis-review/README.md`:

| key | tier there | what it said on rc.16 | note |
|---|---|---|---|
| `(7, A7-F3)` | 2 | `doc show` refuses a relocated managed doc with a route that names none of the repair paths | M53's usability batch set out to close it (*doc show over a relocated doc routes at the repair*) and no axis-7 run has driven it since — say CLOSED (argv) or STILL-OPEN (datum), over both the `location` and the `placement` arm |
| §D axis-7 leads, the freeze/migration members | lead | the methodology pack's own manifest (no rig built a reshapeable one) · 40 of the 54 `SchemaChangeKind × LOCI` cells · `ManifestKind::ALL` (n/a with a driven reason there) · `cwd-unreadable` and `pack-resource-missing` · CX-8's third origin pack · CX-9 as behaviour · CX-13 | re-disposition each: now driven (cell), still open (reason), or not this row's |

There is **no tier-1 row** on numbered axis 7. The M51 rows it CLOSED on rc.16 (`C-1`, `D-1`, `D-2`,
`D-3`, half of `D-4`) are re-driven only where a cell above passes through them; say which. `(7,
A7-F1)`, `(7, A7-F2)`, `(7, C-2)` and the `probe-unreliable` lead are **row 3's**.

## RIG STATES

- **`fresh`** — the base: file one `jigc-feedback` (`jigc start --workflow report-jigc-feedback
  "<what>"`) and one `inconsistency` through the binary and finalize them, then drive the doors.
- **`committed-singletons`**, **`migrated`** — the existing corpus, for the "unchanged" controls and
  for `(7, A7-F3)`'s placement arm.
- **`--pack-from-dev`** — a throwaway **dev** pack copy. **It DROPS the freeze manifest**, so every
  freeze cell driven under it is vacuously green (the rc.20 record caught exactly this mis-filing).
  For a freeze cell use **`--repin`**, which keeps the manifest.
- **`--schema <type> <file>` / `--workflow <id> <file>`** — install a reshaped resource into that
  pack copy.
- **The methodology pack has no rig copy.** The rig's pack flags copy the dev pack only, so the two
  new doctypes cannot be reshaped through `JIGC_PACK_DIR`. The drivable seam is the **project
  layer**: a whole-file shadow at `.jigc/config/schemas/<type>.yaml` (cell 3) or
  `.jigc/config/workflows/<id>.yaml` (cell 4). **No verb mints one** — `jigc config fork` forks a
  *step* only — so the shadow is a hand-written file, copied from the shipped resource
  (`crates/cli/packs/methodology/schemas/<type>.yaml`) and edited. This is the **one stated
  exception** to the standing rule *never write into `.jigc/`*: `.jigc/config/` is the tracked
  project layer a human edits (`design/overrides.md` → *Authored metadata on a definition resolves by
  whole-file shadow*), not the gitignored workbench. Say so at the cell and commit it with plain git,
  as an adopter would. It closes the *methodology manifest* lead as far as a project layer can; a
  reshaped **embedded** methodology pack stays out of reach.

## ENVIRONMENT NOTES

- **NOT DRIVEN, stated rather than guessed:** the **absent-manifest-entry** arm of the freeze (it
  needs an embedded-pack rebuild — M55's own audit did not run it either); any comparison against an
  **older binary** (none is installed here and you must not build one), which is what *"the existing
  corpus is read unchanged across the range"* would need — drive the forward half only (an
  rc.24-built corpus validates clean) and say the cross-version half was not driven.
- Hash arithmetic is not yours to redo: the claim under test is behavioural (the door blocks, or
  loads), not the digest.
- A shadow that blocks pack-load blocks **every** door in that rig, including the one you would use
  to remove it. Remove the shadow file with plain `git`/`rm` on the one named file, or start a new
  rig — there is nothing to tear down.
- `ManifestKind::ALL` is the **finalize left-out** vocabulary (it gained `left-staged` at M55), not a
  schema-manifest kind; no door of this row renders it. It is row 5's.
