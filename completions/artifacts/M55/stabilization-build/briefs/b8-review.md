# Review: the byte table, task B8 — the movers family (`43b555fe..29774121`)

Reviewer: independent and read-only. Worktree `gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`.
Commits: `ab0babc1` (the three rulings), `2d9f5130` (four rows, five words, `Homes`), `a8d53b89` (`rename`,
`migrate-corpus` cells), `ca80554d` (`relocate`, `config set` cells), `29774121` (numbers record, whole-table print).
No product source changes in the range (the last one is `0ad081cd`).

## Verdict

- **Counts are right**, every one I re-derived (§ Counts). **615 needs is right** (suite, not copied).
- **The rulings commit is faithful** to the three rulings. The two bound-2 cells are plain red under their own keys,
  and `DECLARED_BOUNDS = [1, 3, 4]` is fenced. One gap: the ruling's own door has no cell, and the two cells will
  *fault*, not go green, on the fixed product (H6).
- **The new frame words are wider than their sentences in four places, each shown by a probe that comes out
  *holds*:**
  - `RL3`/`CS3` let **any doc of the store** be parked into the gitignored workbench. That includes the doc the
    re-point should move, and an unrelated committed decision (H1).
  - The same lines park a **managed** occupant, where `design/reconciliation.md` says it blocks (H2).
  - *migrated* lets a migration drop prose bullets, user sub-headings, and the `supersedes` edge (H3).
  - *repointed* rewrites a ref to a different doc that shares the prefix, and prose mentions (H4).
- **Fixture-topology masking.** `config set placement-root` **parks a file the user committed at the new home, at
  exit 0, on both binaries**. That is entry 23's red case. No cell sets `placement-root`, so the family's config-set
  entry-23 cell holds only because `docs-root` refuses every squatter. `CS3`'s reason in `EXCUSES_NOTHING` is false
  today (H5).
- **Entries 21 to 23.**
  - 21 is genuine.
  - 22 is genuine but under-quoted.
  - 23 is mis-framed as "two docs say two things". `design/reconciliation.md` answers it on purpose for exactly
    `relocate` and `config set`, as "the settled foreign-only bound". It is still a question for the human, because
    the robust answer reverses a settled design. Its robust answer is incomplete for a root knob.
- **Key attributions.**
  - One key judges "the code" wrong where two design docs decide the behaviour deliberately: `relocate`'s exit 0 on
    a blocked doc (R1).
  - Three new keys repeat open fix-pass rows without citing them, and one is the residual of a ruled fix-pass item
    (R4).
- **The twelve new keys each reproduce on both binaries**, as described.
- **Hygiene is clean.** `cargo fmt` reaches `movers.rs`, and `owed.rs` lost exactly the four B8 leaves.
- **The gate failure can race.** `git commit` (Apple Git 2.54) spawns a *detached* `git maintenance run --auto`,
  which writes and deletes transient files in `.git/objects` after the commit returns. `copy_tree` reads that
  directory with `read_dir` + `fs::copy` (§ Gate).

## What I ran

- **Binaries.**
  - This tree: the worktree's `target/debug/jigc`, sha256 `a71a816e…`. It is current, because no product source
    changed since `0ad081cd`.
  - rc.24: the rebuild in my scratch `rc24/jigc`. `shasum -a 256` gives `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`,
    which matches.
- **Frame probes.** I extracted a copy of `29774121` with `git archive` into my scratch. I appended two test modules
  to that copy's `byte_table.rs`, and ran them with
  `cargo test -p jigc --test g_migration review_probe` (private target). The repository was not touched.
- **Rigs.** `dev/jigc-rig --binary <bin> <state>`, a two-step eval, capturing stdout only. Probes:
  - D1: the edge-index lead.
  - D2: `config set placement-root`.
  - D3: the `rename` destination.
  - `keys.sh`: the twelve keys.
- **Counting.** `awk` over the committed `.tsv`, and a python parse of `EXPECTED_RED`.
- **Other checks.**
  - `cargo fmt --check`, with rc=0 in the worktree.
  - A deliberate misformat in `movers.rs` in the scratch copy made `cargo fmt --check` exit 1 there, which proves
    the reach.

## Frame holes

### H1. High. `RL3`/`CS3` (`MovedAsideTo(".jigc/displaced")` at `Where::Path(A_DOC)`) excuse parking any doc of the store, not only a destination's occupant — a mover that parks the very doc it should move, or an unrelated committed doc, **holds**

**Location.**
- `movers.rs:211` (`RL3`) and `movers.rs:380` (`CS3`). Both have the place `<a-doc>`: any `<slug>.md` at any
  doctype's home, before or after the command (`byte_cell.rs` `is_at_for`, the `A_DOC` arm).
- `byte_cell.rs:2021-2027` is the `(May::MovedAsideTo(dir), Some(left), None)` arm. Any doc gone from its place is
  covered when its bytes arrive under `dir`.
- `byte_cell.rs:1930-1948` is the arrival arm, `.is_none_or(...)`, which covers "gone" as well as "something else
  stands there".
- `written_by_a_line` covers `(May::MovedAsideTo(_), None)`, so the index removal is excused too.

**Against.** The row's own sentence: *"a foreign file at a destination: moved aside"* (RL3). §7 says *"**its place
left** where something else stands there now, a destination a move landed on"*. The fourth commit's `DECISIONS`
entry says the same. The code's arm does not require that anything stands there.

**Driven** (frame probe, scratch copy of `29774121`):
- `config set docs-root notes` takes `docs/decisions/one-way.md` out of the tree, writes its bytes to
  `.jigc/displaced/one-way.md`, removes it from the index, and sets the knob → **holds**. The honest move also holds.
- `relocate note --from docs/legacy-notes/` moves the note correctly, **and** parks an unrelated committed
  `docs/decisions/one-way.md` into `.jigc/displaced/` with its index entry removed → **holds**.

**Why it is a loss the frame hides.** Both are exit-0 states where the user's next `git commit` deletes a committed
managed doc from the repository. That is the harm entry 23 itself names ("takes it out of the repository at the
user's next commit"). The fence `a_moved_doc_lands_at_its_doctypes_home` (`byte_table.rs:3776`) has no exclusion
case for either.

**Bound.** By enumeration, `grep MovedAsideTo( crates/cli/tests/byte_table/*.rs` finds 6 lines. The 2 at a one-file
`<a-doc>` place (`RL3`, `CS3`) are the class. The other 4 are `Under(...)` working-area lines (`T6`, `M6`×2,
`M6, the worktrees moved aside`).

**Fix.**
- For a one-file place, cover only a destination: the path must hold, after the command, an entry that arrived by
  `IntoItsHome`.
- Drop the `Some, None` arm and the "gone" arrival for such lines.
- Add the two exclusion cases to the fence.

### H2. High. `RL3`/`CS3` park a *managed* occupant, where the design doc blocks it; the record files this under "what the docs do not say"

**Location.**
- Rows record §4c, *What the docs do not say*: *"The docs block a managed one and park a foreign one; the frame
  tells them apart by nothing … A managed doc parked there is not seen."*
- `arrived_home` (`byte_cell.rs:2199-2216`) asks only that the occupant's bytes stand under `.jigc/displaced/`.

**The docs say it.**
- `design/reconciliation.md:196`: *"A *managed* instance already at the destination … the relocation **surfaces the
  collision as a block** (never a clobber)"*.
- `:199`: *"Classification is by **baseline**: a managed instance is recorded in the `file-state` map … a foreign
  file is not"*.
- `DECISIONS.md` 2026-07-07 (M39 T5, line 19168).

So the frame line is knowingly wider than its own owner doc. It is also buildable: the before-manifest holds
`.jigc/state/file-state.json`.

**Scenario.** A `relocate` regression parks a baselined note found at `docs/notes/<name>.md`, stages the incoming
note over it, and exits 0. That holds today, and the next commit deletes a managed doc.

**Bound.** The same 2 lines. Instance by trace (`arrived_home` reads no baseline). Not built as a probe, because the
frame reads no `.jigc/state` for these lines whatever stands there.

**Fix.** Read the occupant's baseline from the before-manifest. A baselined occupant is not excused. Add a cell
(e.g. `config set docs-root` over a decision a task landed at `notes/decisions/one-way.md`).

### H3. High. *migrated* (`May::Migrated`) is wider than *"The prose is unchanged"*: a migration that drops prose bullets, user sub-headings, or the `supersedes` edge **holds**

**Location.**
- `prose()` (`byte_cell.rs:2568-2600`) drops every `- <lowercase-key>:` line anywhere in the body, not just field
  leaves (`:2586`).
- `worded()` (`:2265-2270`) drops every line starting `#` and every blank line.
- Front matter is skipped whole: only the stamp is read (`stamp()`).

**Driven** (frame probe). A v1 decision has `supersedes: [adr:old]` and, inside the Context slot, a sub-heading
`### A caveat of the author's` and a bullet `- risk: we may lose data`:
- the honest migration → holds;
- the migration with the bullet and the sub-heading deleted → **holds**;
- the migration with `supersedes` dropped and `status: accepted` → `superseded` → **holds**.

**Why.**
- A `- key: value` bullet and a `###` heading inside a slot are the LLM's prose, which the determinism boundary
  makes the user's.
- A dropped `supersedes` is a dropped cross-reference: a structural loss the CLI owns.
- The fence `a_migrated_doc_keeps_its_prose` (`byte_table.rs:3993`) tests only plain prose lines.

**Bound.** 2 lines use the word (`MG1`, `MG1, from a prior home`), found by
`grep May::Migrated byte_table/*.rs`. Every `migrate-corpus` cell's commit and file are judged by it.

**Fix.**
- Prose = every line outside the front matter and outside the schema's own field/heading lines, as the schema
  declares them, not as a pattern guesses.
- Front-matter field values other than the stamp stand as they stood, unless a snapshot diff says the field moved.

### H4. Medium. *repointed* (`May::Repointed`) matches the old identity as a substring on any line: a ref to a *different* doc that shares the prefix, and a prose mention, both rewritten and **held**

**Location.** `repointed()` at `byte_cell.rs:2132-2170`: `line.contains(identity)` and `line.replace(identity, fresh)`
(`:2162`).

**Against.** `RN2`'s own quote: *"the CLI rewrites **structured ref-fields in the managed store** and nothing else"*
across *"the **three persisted ref-fields**"*.

**Driven** (frame probe). `rename adr:one-way --to "The one way"`, with a third decision holding
`supersedes: [adr:one-way-two]` and a prose line `See adr:one-way in passing.`:
- the ref rewritten to `adr:the-one-way-two`, which is a dangling ref to a doc that does not exist → **holds**;
- the prose line rewritten → **holds**.

The fence's README case (`byte_table.rs:3639`) only covers a file of no doctype's home.

**Bound.** 1 line (`RN2`). Latent: the product rewrites structured fields only (`rename.rs`), so no cell is red. A
regression would be hidden.

**Fix.** Match the identity as a whole ref token (bounded by `[`, `,`, `]`, whitespace or end of line), and only on
the three ref-field lines (front matter `supersedes:`/`cites:`/`derived-from:` or their `- field:` leaves).

### H5. Medium. No `config set placement-root` cell, and the placement-root re-point parks a file the user committed at the new home at exit 0 — entry 23's red case — on both binaries. The config-set entry-23 cell and `CS3`'s "excuses nothing" are artifacts of the `docs-root` fixture

**Location.**
- Every `config set` cell sets `docs-root` (`movers.rs:1531`, `CONFIG_SET_CELLS`).
- Family record §4 *"`config set placement-root`: … held by a fence of the frame, not a cell"*, and *"`CS3` excuses
  nothing on this tree — `config set` refuses every squatter"*.
- `EXCUSES_NOTHING` `CS3` → `NOT_UNTIL_A_RE_POINT_PARKS_A_SQUATTER` (`byte_table.rs:1944`, `:2341`: *"on this tree it
  refuses every squatter"*).
- Readings §22 and §3: *"`config set docs-root` refuses, as it refuses every squatter"*.

**Driven, D2** (`committed-singletons`, both binaries, identical):

| at `notes/roadmap.md` | result |
|---|---|
| nothing | exit 0, `R docs/roadmap.md -> notes/roadmap.md` |
| **a file the user committed** | **exit 0**: `notes/roadmap.md → .jigc/displaced/roadmap.md (a foreign file at the new home, parked out of the way)`, status `D docs/roadmap.md`, `M notes/roadmap.md` |
| untracked | exit 0, parked |

**Consequence.**
- The red case of entry 23 exists at `config set`, unseen by any cell.
- `CS3` would excuse today.
- The builder's own §25 notes the asymmetry ("one mover, two answers by knob") but built no cell.
- Fix-pass `CPL-8` (a placement-root rollback leaving its squatter parked) lives exactly here, and is unguarded.

**Fix.** Add `placement-root` cells in `fresh` (the leads commit a roadmap; `fresh` composes methodology). At least:
the base cell, `CS3 × U`, `CS3 × T` marked entry 23, and `CS2` at the roadmap. Then re-derive `EXCUSES_NOTHING`.

### H6. Medium. Bound 2's ruling has no cell at its door, and the two cells that held it will fault, not go green, on the fixed product

**Location.**
- The cells keep their lead-ins, where the write to the hand-edited part is a **lead** (`task.rs` `FINALIZE_CELLS`,
  the leads after `A_HAND_EDIT`; `milestone.rs` `FINALIZE_A_HAND_EDIT`).
- `Lead::Step` failing gives *"the lead N of the cell was not done"*, a fault (`byte_cell.rs:4365-4375`).
- The `ab0babc1` DECISIONS entry admits it: *"On a fixed product their lead-in is the write that refuses, so they
  stop building: the fixer's acceptance moves the case to a cell whose command is that write."*

**Why it is a hole.**
- The doc family has no cell whose command is a `doc` write over an uncommitted hand edit of the same part. Its
  `W3 × E` cells are all *"the doc deleted before the first write"* (`doc.rs:625…2236`).
- So nothing in the table holds *"refuses at the write"* now.
- The red the table shows will turn into a fault, not a green — which is the shape the human's rule of that day
  (*"nothing that does fail in the end and we need to come back and fix it"*) asks against.

**Buildable now.** A `doc set-slot` cell in `committed-singletons` over `VISION.md` with a line added to the same
slot before the first write, listed as refusing. It is red today as *not the listed outcome*, under the bound-2 key.

**Bound.** 2 cells (`grep` of the two keys in `EXPECTED_RED`: 1 cell each).

### H7. Low–medium. `rename`'s destination has no cell: no line's place is the new home, so `aimed` cannot name the need; a regression that clobbers a squatter there would not be held

**Location.** `RN1` is `Where::Path(THE_DOC)`, the old path only (`movers.rs:115`). The arrival at the new path is read
inside `retitle()`, which is no place, so no need `RN1 × U/L/K` at the destination exists. RN7 names *"a slug another
doc holds"*.

**Driven, D3.** `rename adr:one-way --to "Single way"` with `docs/decisions/single-way.md` as:

| squatter | this tree | rc.24 |
|---|---|---|
| a file of the user's | exit 1, no commit | exit 1, no commit |
| a link to nothing | `store.home-not-regular-file` | refuses with no code |
| a directory | `store.home-not-regular-file` | refuses with no code |

So these cells would be live on this tree, and differ by binary.

**Bound.** An instance: one door. The mechanism (a destination that is no place) is shared by any word read away from
its place: `Retitled` (1 line). `IntoItsHome`'s destination is covered by `RL3`/`CS3`.

### H8. Low. *it committed* (`When::ItCommitted`) lets a refusal that committed one doc also rewrite the doc it reported blocked

**Location.** `byte_cell.rs:1872`. `MG1` holds on such a refusal over every `<a-doc>`.

**Driven** (frame probe). Two v1 decisions. One is migrated and committed. The other is migrated in the working tree
and left uncommitted. Exit 1, listed refusing → **"refuses, nothing changed"**.

**Against.** *"It is a refusal of **one doc**: … the doc joins `blocked`"*: a blocked doc is not written.

**Bound.** 4 lines carry it (`MG1`, `MG1, from a prior home`, `MG2`, `MG3`, via `holds_on_a_refusal`). No cell
exercises it.

### H9. Low. *its knob set* (`May::ItsKeySet`) does not check the value, and treats the `scalar:` line as free

**Location.** `its_knob_set` (`byte_cell.rs:2299-2322`) asks only that *a* line names the knob.

**Driven** (frame probe). `config set finalize.fan-out.squash false` writing `'true'` → **holds**. A `docs-root`
written wrongly would also move every doc to the wrong root consistently, which holds too (`CS2` reads `homes_after`
off the written manifest).

**Bound.** 1 line (`CS1`).

### H10. Low. `M6, the worktrees moved aside` lets one sub-task's leftover land under a *sibling's* place

**Location.** `MovedAsideTo(".jigc/displaced/<sub-task>")` (`milestone.rs:305`) with `under_for` (`byte_cell.rs:1785`).
It is declared: *"What it does not ask"*.

**Driven** (frame probe). With a record naming `s1` and `s2`, `s1`'s `notes.txt` arrives at
`.jigc/displaced/s2/notes.txt` → **holds**.

The fence `a_file_moved_aside_lands_under_its_units_place` tests only `s2` *not* named by the record, so it cannot
see this. No byte is lost, but the ruling's `<sub-task-id>` names the wrong unit, and the narrowing is buildable
(the source's `<sub-task>` component must equal the target's).

### H11. Low. *retitled* does not check the slug the title gives

**Driven** (frame probe). The rename lands at `docs/decisions/zzz.md` for `--to "The one way"` → **holds**. No loss.
The id is wrong, and `RN2` follows it consistently.

## Record notes

- **R1 (high for triage). `r1-relocate-refuses-a-doc-and-exits-0` says "which is wrong: the code", but two design docs
  decide exit 0 on purpose:**
  - `design/command-output-contract.md:175`: *"This **`blocked` is not an exit rule** — `relocate` surfaces it and
    stays exit 0 — which is exactly the distinction `migrate-corpus`'s same-named key makes the other way"*.
  - `design/validation.md:734`: *"**`jigc relocate` takes the identical shape and for the identical reason** … Its
    exit is therefore **unchanged** — stated here rather than silently re-graded"*.
  - The family's own `RL5` quotes the first.

  The key's "held against" quotes `migrate-corpus`'s loud rule. This is a §6 entry (docs saying two things:
  `finalize.md:148` *"lands a regular file or refuses"* against the two above), or at least a key that names those
  docs as wrong. It also decides entry 23's relocate listing (`refuses(1, …)`): with the documented exit 0, a
  product that correctly *blocks* the committed squatter still comes out *not the listed outcome*.
- **R2. Entry 23 is mis-framed and under-quoted.**
  - `design/reconciliation.md` → *Relocation collisions* scopes itself to *"the freeze-exempt relocation floor + `jigc
    config set docs-root`"*.
  - It decides committed-but-unmanaged → park, as *"the settled **foreign-only** bound"*, classified *"by
    baseline"* (`:197`, `:199`).
  - `design/corpus-migration.md`'s block is `migrate-corpus`'s walk.

  So no door has two answers: the question is answered by the doc and judged against it, which is legitimate under
  the human's rule and still theirs to rule, since it reverses a settled design. Missing quotes: `reconciliation.md:196`
  and `:199`, `DECISIONS.md` 2026-07-07 (M39 T5, lines 19168 and 19292), and `validation.md:734`.

  The robust answer, *"block that doc, as `migrate-corpus` does"*, is incomplete for a root knob. Blocking one doc
  of a re-point while the knob lands strands that doc at a home the cascade no longer resolves. For `config set` the
  sensible end state is to refuse the re-point whole, with the knob unchanged.
- **R3. Entry 22 is under-quoted on the "leave it" side.**
  - `design/finalize.md:190` makes it the door's state-truth clause: *"`migrate-corpus` leaves the migrated bytes
    written and staged (and the corpus recorded unmigrated)"*.
  - `DECISIONS.md:11829` (G-47) builds a second clause on it.

  The entry's *Searched* line cites finalize's *Rollback discipline* only. The bound's origin, *"not in this
  increment's approved scope"*, is `DECISIONS.md:16545`; the entry has it as "M47's build", without the line.
- **R4. New keys that repeat fix-pass rows without citing them** (derivation: `grep -i 'config set|relocate|migrate-corpus'`
  over `fix-pass-rc25/findings-ledger.md`; every hit read):

  | B8 key | fix-pass row it repeats |
  |---|---|
  | `r1-config-set-refuses-and-leaves-the-directories-its-re-point-made` | `(R5, F9)` *"a rolled-back `config set docs-root` leaves the new root's empty directories behind — OPEN — port"*, and `CPL-9 (d)` |
  | `r1-config-set-re-points-past-a-link-to-nothing-or-a-directory-at-a-doc-it-strands` | `5f5b273a`-6 *"`config set docs-root`'s loop silently skips a dangling link … the link stays at the old home — OPEN"* |
  | `r1-config-set-writes-the-knob-through-a-link-at-the-manifest` | `5f5b273a`-7 *"config-family writers under `.jigc/config/**` (`config set`'s manifest …) are still plain `fs::write` — OPEN"* |
  | `r1-migrate-corpus-reads-past-a-link-…` | the residual of the human's ruling of 2026-10-07 on item 9 (`design/finalize.md:153`, "closed") and of `5f5b273a`-3 / `CPL-9` |
  | `r1-relocate-refuses-a-doc-and-exits-0` | sits beside `5f5b273a`-4 (the link-source refusal, *"open to overturn"*) |

  The record's sentence *"the directories it leaves are this tree's alone"* is true of this trigger only, since
  `(R5, F9)` was found on rc.24.
- **R5. Stale headers.**
  - Rows record §6 header: *"entries 4, 7, 21 and 22 are open"*. It omits 23.
  - The readings intro ends at *"§20 and §21 … whose cells come with the family's"*, with no entry 23 and no five
    marked cells.
- **R6. Wrong as stated.** These claims hold for `docs-root` only, and `placement-root` parks (H5):
  - family record §4, *"`CS3` excuses nothing … refuses every squatter"*;
  - family record §3 and readings §22, *"`config set docs-root` refuses"*;
  - the `29774121` DECISIONS line *"`config set placement-root` parks a squatter where `docs-root` refuses one"*,
    which is right but drew no cell.
- **R7. Wrong heading.** Rows record §4c lists *"whether a relocate destination's occupant is foreign or managed"*
  under *"What the docs do not say"*. The docs say it (H2).
- **R8. Inexact wording.** The `2d9f5130` DECISIONS line says *"The lines that moved an area's complement aside read
  as they did"*. The widened arrival (`byte_cell.rs:1941-1946`, `is_none_or(!same)`) is shared by all six
  `MovedAsideTo` lines. The effect on `T6`/`M6` is bounded, since the rewrite at the source place is still red.
- **R9. A misleading helper.** `movers.rs:25-34`'s `line()` sets `When::ItCommitted` on every line of all four rows,
  under the doc *"holds where the command acted, whenever it did"*. It is harmless, because `in_force` gates
  `on_refusal: false` on `acted` first. But `ItCommitted` belongs to `migrate-corpus`'s refusal lines, and this
  hides that.
- **R10. A "no doc says" that was not searched.** §24 says *"whether a migration should baseline a hand-committed doc
  no doc says"*. `design/reconciliation.md:40` (*"Absent-hash is not drift … adopts the current on-disk content as the
  baseline"*) answers the general rule.
- **R11. An inconsistency in entry 21's robust answer.** §23 rejects *staging* the knob beside the moves because a
  path-limited commit can take one half. Entry 21 picks *staging* for `--no-commit`'s relocation halves, which have
  the same exposure. The answer is defensible, since the flag is the asking, but should say so.

## Open questions — verdicts

| entry | genuine? | robust answer | says doc vs code? |
|---|---|---|---|
| 21 `--no-commit` staged? | **Yes, two docs say two things.** `write-commands.md:160` *"leaves the changes staged"* against `corpus-migration.md:258` *"exactly as the verb leaves it today"*, the guide `MIGRATING.md:28`, and M42's T3. | Stage the touched paths: no loss, and it avoids the untracked-plus-deleted half state. Consistent with the invariants; see R11. | Yes: `corpus-migration.md`, the guide and the product are wrong. |
| 22 hook rejection leaves migration staged | **Yes.** It is a builder's scope bound (`DECISIONS.md:16545`), never on the human's list. Under-quoted (R3). | Roll back, per "Writes are transactional". Sensible; the re-run's recovery is kept for a killed run. | Yes: the bound in `corpus-migration.md` (and `finalize.md:190`, unnamed), and the code. |
| 23 committed squatter: park or block | **A question for the human, but not "two docs, two things".** `reconciliation.md` decides it for these doors as a settled bound (R2). | Block the committed squatter: right for `relocate`. For a root knob it must be "refuse the re-point whole". It is entangled with `relocate`'s documented exit 0 (R1). | Names `reconciliation.md` and `relocate`; should add `config set placement-root` (H5) and the exit-rule docs. |

## The builder's self-check, verified

| item | verdict |
|---|---|
| `migrated_into` against `HEAD` at the prior home | Correct as built: the commit path reads `head_held(from)`, and the fence's committed case is *git's state moved*. But the word itself is too wide (H3). |
| A migration carries a planted line | Correct: the `IntoItsHome`/`Migrated` plant arms at `byte_cell.rs:2750-2770`, and the fence `--no-commit` case holds. |
| A directory parked whole | `in_a_parked_place` (`:2743`) is correct for a squatter, and inherits `RL3`'s breadth (H1). |
| The squatter place | `aimed`'s `parks` is correct, but `RL3`/`CS3`'s place is every doc of the store (H1), and a managed occupant goes unseen (H2). |
| Entry 23's listing | `refuses(1, <path>)`: the words are a path that a parking ack also prints, so only the exit code discriminates. That conflicts with `relocate`'s documented exit 0 (R1). |
| Sets keyed on the acting unit | `A_DOC` is unit-blind where a destination was meant (H1). `<sub-task>` in `displaced/` is not keyed on the source sub-task (H10). |
| The edge-index lead | **Genuine.** D1: on both binaries, `task validate` exits 3 with `edges.json` built, it is still there after `task discard --force`, and `rename` takes it away. |
| `MG2` on a task-landed decision | Fine. `MG2` is in `EXCUSED_IN` for `fresh`. |
| `CS3` excusing nothing; no `placement-root` cell | **Not fine** (H5). |
| Every leaf driven with nothing planted | Base cells hold on both binaries. My rigs saw no write outside the rows. |

## Counts re-derived

All match the records.
- **Needs: 615 for 26 rows; the movers family 121.** These are from the suite (a probe summing `needs(row)`). By
  hand:
  - `rename` 29 = 8 + `RN1` 8 + `RN2` 8 + `RN3` 3 + `RN4` 1 + `RN5` 1;
  - `relocate` 23;
  - `migrate-corpus` 37;
  - `config set` 32.

  Ten are scheduled (`scheduled-cells.tsv`, non-comment lines). "615, not 616" is right.
- **The print** (657 rows):
  - the family: 127 cells; 41 red on rc.24 and 37 here;
  - the four old-only reds are `rename`'s link and directory cells (*not the listed outcome*);
  - the whole table: 241 red on rc.24 and 153 here.
- **Cells.**
  - The fence gives `(657, 501, 156)`.
  - The on-demand `.tsv` has 157 tab lines, which is 156 plus the header. B8 added 24 of them (10 hold, 14 refuse).
- **`EXPECTED_RED`** (python parse): 38 entries, 36 keys, 139 cells, all unique. The movers family has 37 cells under
  12 keys (12 + 2 + 2 + 1 + 4 + 1 + 1 + 2 + 3 + 2 + 2 + 5).
- **Seconds** (print, this tree):

  | leaf | cells | mean | gate sum |
  |---|---:|---:|---:|
  | `rename` | 31 | 1.63 | 44.7 |
  | `migrate-corpus` | 39 | 2.22 | 79.1 |
  | `relocate` | 23 | 1.56 | 30.8 |
  | `config set` | 34 | 1.54 | 46.7 |
  | all | 127 | 1.77 | 201.3 (0.7–4.3) |

  The other 530 cells average 2.58. The arm seconds of the three gates (120.1 → 138.4) I could not verify; they are
  labelled a cross-run difference.
- **Marks: 5**, as the readings table has them.
- **Per-commit lists.** 600 → 657 cells; 119/28 → 139/36 red cells/keys; 3 → 5 marks.

## The twelve new keys — reproduced on both binaries

Run as the findings print them (`keys.sh`):

| key | this tree | rc.24 |
|---|---|---|
| K1 `migrate-corpus` reads past a link | exit 0, `0 migrated, 0 already current, 0 blocked`, HEAD still v1 | same |
| K2 commits an uncommitted edit | exit 0, the line in HEAD | same |
| K3 replaces a staged change | exit 0, 0 commits and the index hold it | same |
| K4 rejected hook | exit 1, `M  docs/decisions/one-way.md` | same |
| K5 `relocate` blocked, exit 0 | exit 0, `1 blocked`, `store.home-not-regular-file` | exit 0, `No such file or directory` |
| K6 parks a committed file | exit 0, `D …legacy…; M docs/notes/…`, parked | same |
| K7 comment dropped | exit 0, comment gone | same |
| K8 non-knob manifest replaced | exit 0, line gone | same |
| K9 written through a link | exit 0, `mine.txt` = `scalar:` | same |
| K10 dangling link at the decision | exit 0, knob set, no `notes/` | same |
| K11 refusal leaves directories | exit 1, `notes notes/decisions` left, link stays | exit 0, the link moved |
| K12 squatter refused | exit 1, `fatal: destination exists` | same |

The three keys held by no cell are justified:
- the §23 pair: a manifest records what changed, never what should have, and the other is words;
- `rename`'s *"check the id you typed"*: words.

The §23 repro also reproduced on both binaries in D2: `?? .jigc/config/manifest.yaml` beside the staged moves.

## Rulings commit (`ab0babc1`) — faithful

- **Entry 18.** The `DECISIONS` text has *"moves its unstaged and untracked files that git does not ignore into
  `.jigc/displaced/<sub-task-id>/` and names them; what git ignores is named and removed"*. The doc change is filed
  as a ledger finding, folded into the product key.
- **Entry 20.** Recorded as a doc-only key.
- **Bound 2.** Recorded with the human's words. `DECLARED_BOUNDS = [1, 3, 4]` is at `byte_table.rs:9378`, and the
  fence case `cell("retired", &[Mark::Bound(2)])` is at `:9482`. Both cells are unmarked and red under their own
  keys: `EXPECTED_RED`, 1 cell each.
- **Gap: H6.**

## Hygiene

- No host path, person's name, session or agent id was added. A grep of every `+` line and of the commit messages
  found none.
- The human is "they"/"their" in all new text. Pre-existing "him/his" in `decisions-pending.md` lines that B8 edited
  (`(D) The declared bounds`) predates B8 and is not B8's.
- `owed.rs` lost exactly `rename`, `relocate`, `migrate-corpus` and `config set`. `CELLS_OWED` is empty.
- `cargo fmt` reaches `movers.rs` (scratch misformat → exit 1).

## Gate: `record_door_baseline::an_untouched_record_with_no_baseline_still_serves_every_door` — can it race? Yes

**The mechanism.**
- The test builds one base corpus with `live_milestone()`, ending in `git commit`s made by `jigc` and by the
  helper. It then calls `base.copy_state()` 5 doors × N times.
- `copy_tree` (`trial_corpus.rs:1091-1120`) does `read_dir` and then `fs::copy` per entry. It panics on any error.
- `git commit` here (Apple Git 2.54) runs `git maintenance run --auto --quiet --detach`. `GIT_TRACE` shows
  `run_command: git maintenance run --auto --quiet --detach`.
- That detached child runs on after `commit` returns, and writes and deletes transient files under `.git/objects`.
  Polling `.git/objects` for 50 ms after each of 200 commits saw `bitmap-ref-tips_*` temp files on 3 occasions,
  present for hundreds of polls.
- A `read_dir` that lists such a file, followed by `fs::copy` after the child deleted it, panics with ENOENT. A copy
  of a half-written file corrupts the copy instead.
- Passing alone fits a timing race.
- `git_init` (`trial_corpus.rs:1018-1025`) sets no `maintenance.auto false` or `gc.auto 0`.

**The exposure.** Every `copy_state()` caller: 81 calls in 18 files, the byte table's `copy_state` per cell among
them (`byte_cell.rs:3963`). It is not B8's code.

**Fix.** Set `maintenance.auto=false` (and `gc.autoDetach=false`) in `git_init` and the rig.

## Repros

```zsh
# Frame probes — appended to a scratch copy of 29774121's crates/cli/tests/byte_table.rs, then
#   CARGO_TARGET_DIR=<scratch>/target cargo test -p jigc --test g_migration review_probe -- --nocapture
# Results: every case below printed `holds` (H8: `refuses, nothing changed`); suite needs total=615 movers=121 rows=26.
# H1a config set parks the doc it should move:
#   before = with_committed([("docs/decisions/one-way.md", DEC)])
#   after  = before − docs/decisions/one-way.md + .jigc/displaced/one-way.md (same bytes) − its index entry
#            + .jigc/config/manifest.yaml "scalar:\n  docs-root: notes\n";  argv config set docs-root notes; row CONFIG_SET
# H1b relocate parks an unrelated decision: the fence's note-pack corpus + a committed docs/decisions/one-way.md;
#   after = git_mv(legacy note → docs/notes/…) and one-way.md moved to .jigc/displaced/, index entry removed; row RELOCATE
# H3 migration: V1 with "supersedes: [adr:old]" and, in Context, "### A caveat of the author's" + "- risk: we may lose data";
#   after (committed) = V2 with the bullet and heading deleted → holds; with supersedes dropped + status superseded → holds
# H4 rename adr:one-way --to "The one way": third doc "supersedes: [adr:one-way-two]" rewritten to adr:the-one-way-two → holds;
#   its prose "See adr:one-way in passing." rewritten → holds
# H8 two v1 decisions; one migrated+committed, the other migrated in the tree only; code 1, Listed::Refuses → refuses, nothing changed
# H9 config set finalize.fan-out.squash false writing 'true' → holds
# H10 record m names s1,s2; .jigc/worktrees/s1/notes.txt (plant) → .jigc/displaced/s2/notes.txt; row milestone::FINALIZE → holds
# H11 rename --to "The one way" landing at docs/decisions/zzz.md → holds
```

```zsh
# H5 / D2 — config set placement-root over a file the user committed at the new home (both binaries: exit 0, parked)
W=<worktree>; rig=$($W/dev/jigc-rig --binary "$BIN" committed-singletons) || exit 1; eval "$rig"; cd "$REPO"
mkdir -p notes && echo "the user's own notes" > notes/roadmap.md && git add notes && git commit -qm "notes of the user's"
$JIGC config set placement-root notes; echo "exit=$?"   # exit=0 · notes/roadmap.md → .jigc/displaced/roadmap.md
git status --porcelain                                  # D  docs/roadmap.md · M  notes/roadmap.md · ?? .jigc/config/manifest.yaml
```

```zsh
# H7 / D3 — rename onto an occupied destination (both binaries refuse; codes differ for a link/dir)
rig=$($W/dev/jigc-rig --binary "$BIN" fresh) || exit 1; eval "$rig"; cd "$REPO"
# commit docs/decisions/one-way.md (v2 decision) + docs/architecture/gateway.md (cites adr:one-way) as movers.rs does
echo "the user's own notes" > docs/decisions/single-way.md      # or: ln -s nowhere …  /  mkdir … && file inside
$JIGC rename adr:one-way --to "Single way"; echo "exit=$?"    # 1; link/dir: store.home-not-regular-file here, no code on rc.24
```

```zsh
# D1 — the edge-index lead is genuine (both binaries)
# … commit the decision and its citer as above, then:
$JIGC start --workflow single-task "build the index"; $JIGC task validate build-the-index   # exit 3
ls .jigc/index      # edges.json
$JIGC task discard build-the-index --force; ls .jigc/index   # edges.json
$JIGC rename adr:one-way --to "Single way"; ls .jigc/index    # edges.json gone (invalidated)
```

```zsh
# Gate race — a commit leaves a detached maintenance writing .git/objects
D=$(mktemp -d "$SCRATCH/gctrace.XXXXXX"); cd "$D" && git init -q r && cd r && git config user.email a@example.com && git config user.name a
echo x > f && git add f && GIT_TRACE=1 git commit -qm c > ../trace.txt 2>&1; command grep maintenance ../trace.txt
# … run_command: git maintenance run --auto --quiet --detach
# python poll of .git/objects for 50 ms after each of 200 commits: {'bitmap-ref-tips_…': 96, …: 113, …: 562}
```

## Not checked

- No `dev/gate` and no full table run. The frame probes ran in a scratch copy, the cells were not re-run, and H2 is
  traced only.
- The three gates' arm seconds.
- Linux, and whether stock git on the runner detaches maintenance the same way.

<!-- end of report -->
