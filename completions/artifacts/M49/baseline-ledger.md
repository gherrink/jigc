# M49 — the verified baseline ledger

**Driven at HEAD `99231f6`, binary `jigc 1.0.0-rc.12`.** Six capability-auditors + four gap-detectors,
each exercising the real binary in throwaway repos. **This ledger supersedes `charter.md` wherever they
disagree** — the charter was written by the session that ran the 1.0.0 trial, which its own handover
names as "exactly the position that produces confident wrong premises."

**Scope, settled by the human 2026-08-28:** do the work now → ship 1.0.0 → then port this repo onto
jigc. The port is a *later act*; M49 makes it possible.

---

## Charter reconciliation

| claim | verdict |
|---|---|
| `doc show` 34 · `doc schema` 0 · `doc list` 0 | CONFIRMED, reproduced independently |
| A4: 12 catalog / 21 absent / 3 unreasoned | CONFIRMED (`ingest-existing`, `router`, `increment`) |
| 5 `type: ref`, all doc-level, zero per-item | CONFIRMED |
| E1 `doctype-authoring.md` names no fence | CONFIRMED (0 hits) |
| E2 / F3 stale statements | CONFIRMED; F3 has an unlisted sibling at `workflow-dialect.md:5`, and a third at `write-commands.md:5` |
| N2 int-vs-string | CONFIRMED live |
| S1 needs a new engine transform kind | CONFIRMED **by building it** |
| "13 route-less conformance.*" | 13 live producers · 15 code names · **2 with no producer at all** |
| S-1 `--from-file -` "~30x" | CORRECTED — 42x; heredoc 0x |
| the `add-task --workflow` "dead end" | CORRECTED — routed and self-recovering, *except* under a bogus workflow or a fresh clone |
| **P1 "two packs describe the same phase incompatibly"** | **FALSIFIED** — different tiers (`execute.yaml` ∈ `increment`; `implement-tasks.yaml` ∈ `milestone-execution`) |
| P2 `robust-advocate` absent from packs | CONFIRMED as fact, **but it is a recorded DECISION** (`DECISIONS.md` 2026-06-21: orchestration does not fold), not a gap |
| P3 `validate.yaml` states no independence | PARTIAL — it says "independently"; only the *who* is missing |
| "per-item refs unproven / engine can't" | REFUTED — built, written, edge-indexed, gated, and migrates as a byte no-op |
| the depth cap blocks the increment tier | REFUTED — the tier sits at depth 2, inside the real cap |

---

## Tier 0 — silent corruption and integrity. Everything else lands on top.

**T0-1 · The item-region class — one root cause, two seams, driven on the REAL `roadmap` doctype.**
An item's own leaf region is computed by stopping at the first *deeper* heading — but an item's slot
sub-labels **are** deeper headings.
- *Write seam* (`write.rs:2406` -> `:3441`): >=2 slots + one settable field. Two ordinary `set-field`
  writes yield `- status: planned` AND `- status: complete`. **Commits at exit 0**, `jigc validate`
  says "validates clean", and `doc show --format json` silently returns only one value — the pinned
  1.0 read contract drops the other. A 1-slot control replaces correctly. **No nesting required.**
- *Parse seam* (`parse.rs:974`): >=2 slots + a nested repeatable. `doc add-item` **acks exit 0 then
  bricks the doc** — `store.unparseable`, every subsequent read and write refused, four route-less
  findings for one defect, only `task discard` escapes.
- `--unset` on the corrupted item reports a **false** `write.not-present` whose route does not
  terminate. **The tool cannot undo the corruption it just created.**
- Axis: `item_own_leaf_region` has exactly 3 call sites; the class test iterates
  `{single-slot, multi-slot, slotless} x {nested, not} x {insert, update, unset}` = 18 cells,
  derived from `Schema`, not hand-listed.
- **Causal root:** `design/doc-read-surface.md:85` asserts "no shipped doctype declares a multi-slot
  repeatable." False since **M16** (`self-hosting.md:90` says so). M47 therefore pinned the *read*
  side against a synthetic fixture and never swept the *write* side.

**T0-2 · `set:` is an open string.** A typo (`on-creat`) loads clean, is echoed through the pinned
`doc schema` contract as a real deriver, and **silently exempts the field from `required-field-present`
forever**. `validate.rs:2885` exempts on `set.is_some()` regardless of value. Three honored kinds are
already named in three consts. **One-way: `set:` is inside every `schema-hash` and crosses contract-v5.**

**T0-3 · `milestone list-tasks` — classified `VerbKind::Read` — resurrects a discarded sub-task with
the WRONG workflow.** `milestone.rs:1567` -> `reseed_sub_task_areas(..., DEFAULT_SUB_TASK_WORKFLOW)`.
The registry's own predicate (`cli.rs:1348`) says a Read verb mutates nothing "nor the `.jigc/`
workbench", and classifies `milestone execute` as Write *for this exact reason*. All nine Read leaves
swept over a revealing state: single-member class, fully enumerated.

**T0-4 · `add-task --workflow <bogus>` accepted at exit 0** -> a permanently unreachable sub-task; both
re-entry doors refuse, neither route repairs.

**T0-5 · `task discard` of a sub-task leaves the committed record claiming `status: active`** — the
M42 `milestone discard` class, sibling never swept. It also feeds T0-3: the lying record is the reseed source.

**T0-6 · The project-layer schema shadow bypasses the freeze entirely.** `assert_schema_freeze` reads
the **pack**; `all_schemas` reads the **project shadow**. A `.jigc/config/schemas/adr.yaml` dropping four
sections validates clean at exit 0, while `doc schema` reports the frozen version for an unfrozen shape
and `doc create` writes a third shape. **Falsifies CLAUDE.md's freeze invariant for the project layer.**
Entangled: the shadow is currently the only mechanism that could re-home a frozen methodology doctype.

---

## Tier 1 — engine capability. The port cannot begin without these.

**T1-1 · `AddedItemSlot` is missing at EVERY arity** — including 0-slot -> 1-slot. `RemovedField` is
refused by design. So **there is no legal path from a scalar item field to item prose for any doctype,
ever.** Three of four port-critical reshapes are exactly that (`completion-record.evidence`,
`milestone-record.tasks`, `idea.trigger`).

**T1-2 · `AddedNestedRepeatable` missing, and no transform kind recurses INTO a nested block.**
If the increment tier ships without the nested arm, **`roadmap` v2 is the last version `roadmap` can
ever have** — frozen at birth, no legal route, for every adopter.

**T1-3 · The backstop cannot save either.** `schema_diff.rs:309` declares it catches only an
*otherwise-empty* diff. **Locked prior art already settled the principle the other way** —
`corpus-migration.md:188`, for `RemovedField`: "The kind must exist rather than ride the backstop,
because the backstop is a residual... the dropped leaf would be silently ignored." `schema_diff.rs:653`
blesses the opposite for slots. Building the kinds is *mandated by existing locked doc*, not a fresh choice.
Driven: a removed-nested + added-field bump lands a **permanent dead end** whose route points into the
jigc source tree — a route no adopter can follow.

**T1-4 · Three nesting ceilings ship; one is documented.** Loader admits 4 (`schema.rs:541`); `add-item`
usable to 3; **leaf writes usable to 2** (`address.rs:170`, `MAX_FRAGMENT_HOPS = 6`). The constant's own
comment models the budget as "a section hop, up to four item hops, and a leaf" — wrong: each level costs
**two** hops. `doc schema` advertises depth-3+ addresses the write path rejects, and the blocking gate's
route **is** the unrunnable command. **The increment tier at depth 2 is inside the real cap** — no
address-grammar change is needed for it.

**T1-5 · `doc author` is ~O(n^3).** 50 -> 0.08s · 400 -> 12.5s · 800 -> 98.7s · 1500 -> killed.
`add-item` stays linear. **Every one of the 12 migrate workflows uses `author`.** Bounds MIGRATING.md's
whole promise; stated nowhere.

**T1-6 · `decisions-log` cannot receive this repo's corpus.** 890 titles -> **884 slugs, 5 collision
groups**; batch refused at `write.already-present`; the printed route is not runnable in a batch;
`add-item` has no `--slug` (unlike `doc create` and `migrate`).

**T1-7 · PB-1 — a project pack cannot coexist with the embedded methodology pack.**
`compose-embedded-methodology: true` + a `packs:` list is refused. This repo's config carries the marker.
**Adding any doctype costs the entire methodology pack.** A project pack is also a *fork, not a delta*
(every reused base step must be vendored) — which the cascade invariant forbids by name.

---

## Tier 2 — pinned contracts. Breaking after the 1.0 pin.

- **N1** — a nested-section hop answers with 4 codes across 6 doors. Stable-key change.
- **N2** — `schema-version` is `2` on `doc schema`, `"2"` on `doc show`. A *type* change, not an
  additive key; `doc show` has **no version integer**, so post-pin it costs a minted v2.
- **D1 (=C2)** — the text renderer drops `location.address` AND line (`render.rs:2376`).
  **This falsifies the route exemption's own stated rationale** (`validation.md:64`, `finding.rs:255`:
  "the located message *is* the repair"). All three razor legs satisfied **today** — no rule-writing needed.
- **D5** — 13 live route-less blocking codes. The instrument D5 was refused for lacking is
  **two functions above it in the same file** (`is_declared_singleton` / `is_declared_non_unique`).
  `jigc ingest` already enriches one of these codes with a route.
- **Gate repair routes omit `--task <id>`** — unrunnable with >=2 active tasks. 3 codes x 3 doors = 9 cells.
  The fix pattern ships (`enrich_not_present_route`, `doc.rs:899`), applied to the sibling class only.
- **The route-floor assert is `debug_assert!`** — release-inactive. Every "the floor catches this" claim
  is about the debug seam suites, not the shipped binary.

---

## Tier 3 — schema. Every bump is an adopter migration after the pin.

| doctype | change | classification | note |
|---|---|---|---|
| `roadmap` 1->2 | `status` field · a `milestone-record` ref · **drop `decomposition`, add `increments`** | field/ref halves ship; tier half needs T1-1+T1-2 | **must be ONE bump.** T0-1 fires the instant a field lands while both slots remain |
| `milestone-record` 2->3 | per-task `workflow` · `increment` · widen `status` | all `AddedItemField`/`EnumWidened`; 0-slot block, safe | third version in one cycle |
| `completion-record` 1->2 | severity enum -> HIGH/MEDIUM/LOW · `evidence` -> prose | enum ships; **prose half gated on T1-1** | zero prose slots today |
| `deferral-ledger` 2->3 | discharge axis · `(Q)` member · `trigger-test` | `EnumWidened` + optional field | 1-slot block, safe |
| `vision` 1->2 | 12 real `##` sections vs 3 slots | `AddedOptionalSection` / `FixedSlotToRepeatable` (**driven green — first ever**) | |
| `idea` 1->2 | multi-paragraph `trigger` | add-a-section | 12 of 38 affected |
| `decisions-log` | **no schema change** | — | blocker is identity (T1-6) |

**Positive result:** `1 slot + nested repeatable` **works end-to-end today**. So
`roadmap.milestones = title + proves + increments` is viable — **only if `decomposition` is removed, not kept.**

**Also owed:** `planning-record` (spec'd since 2026-06-21, unbuilt; two locked homes specify **two
different shapes** and **14 vs 13 gate sets**). Free at the freeze — a new doctype moves no hash.

---

## Tier 4 — the surface wave (the charter's original scope)

`doc schema`/`doc list` named 0x in either pack · SKILL.md names `jigc doc` 0 times · the
`create.singleton-copy-in` fence **mandates a false statement** on 5 steps while 4 unfenced steps state
it correctly · all 6 pack-load fences inert for adopter packs · a workflow/doctype can drop off
`describe` at exit 0 · `describe` lists 16 commands while composition resolves the union · clap seam
covers 2 of 16 error kinds · unknown-doctype 7 doors / 5 shapes / `PackResourceKind` Debug leak ·
not-a-git-repo 3 texts / 11+ sites · `<MILESTONE_ID>` unsubstituted x3 · the fan-out spawn line
hardcodes `sub-task` · `finalize.render-io` misdiagnoses a never-composed sub-task as a disk fault ·
20 falsified doc statements across 12 files · **14 of 28 `design/` docs open with "Notation is
illustrative"**, which makes the razor's leg 1 arbitrary.

---

## Open forks for the human (do NOT decide in a plan)

1. **The razor.** Claw 2 items violate no stated rule, so M46's three-leg razor refuses them **by
   construction** — the charter saw this and shrank the scope to protect the claim. With port-enablement
   now in scope, claw 2 needs its **own** razor that can still refuse.
2. **PB-1** — the doc's rationale is "out of M21 scope" / "a deferred composition": a *scheduling*
   bound, explicitly reopenable. "Do nothing" is not neutral; it defers the port outright.
3. **The edge direction** — `team-ready-state.md:185` says `milestone-record -> roadmap-entry`; the
   charter says the reverse. Two locked artifacts, opposite directions, neither citing the other.
   The `spec.derived-from` precedent points at storing the ref on the side authored *later*.
4. **Multi-slot ∧ nested** — fence it (honest, small, but a one-way door on the canonical render form
   that must be *removed* later, post-1.0, over an adopter corpus) vs implement it now.
5. **The region fix** — anchor-sniffing (undefined over exactly the malformed corpora the
   `item-heading-unanchored` finding exists for) vs schema-derived, which `surface-contract.md:58`
   already mandates by name.
6. **N1 · N2** — both one-way, both parked *for* a deliberate call.
7. **`decisions-pending.md:146` reads "CLOSED — take neither (the human, 2026-08-18)"** and the
   2026-08-28 done-picture reverses it. Owed: an explicit **basis-has-changed supersession**, not a
   silent override. That entry also records a cost that must be weighed openly: *"a schema bump makes
   the following trial about migration rather than about the fixes."*
8. **Delegation vocabulary** — `DECISIONS.md` 2026-06-21 deliberately excludes the multi-agent planning
   fan-out from the pack. A step's *prose* naming the instrument may survive that rationale; anything
   that makes jigc dispatch or count rounds does not. Requires the original sentence quoted and re-argued.

## Fired triggers never resurfaced

`decisions-pending.md:371` (multi-slot ∧ nested — fired by S1) · `:378` (3+ pack composition — fired by
PB-1) · `:393` (mis-keyed leaf silently erases a section — fired by any project pack) · `:394` (no nested
transform arm) · `:354`/`:181` (`planning-record`) · `:146` (the frozen bump) · `methodology-docs.md:103`
(multi-level repetition — shipped at M22) · the snapshot-hash revisit trigger (`corpus-migration.md:94`)
— this wave grows the snapshot set by up to five, two of them *second* snapshots for one doctype.

## Refuted / de-scoped by driving

- The carryover standing order — **REFUTED**; the gate protects pre-mint staging (3 arms driven).
  Residue: no `MINT_DOORS` enumeration fence.
- `milestone provision` mid-phase-2 non-rollback — **REAL but LOW**; recovery holds as declared.
  Residue: C7 (execute narrates a half-provisioned set as ready) + C8 (code-less/route-less anyhow).
- The depth cap blocking the increment tier — **REFUTED**; depth 2 is inside the cap.
