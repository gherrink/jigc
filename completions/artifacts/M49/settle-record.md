# M49 — the Settle record

**Ten decisions, taken with the human 2026-08-28**, against [baseline-ledger.md](baseline-ledger.md)
(six capability-auditors + four gap-detectors + one robust-advocate, every claim driven on
`1.0.0-rc.12` at HEAD `99231f6`). Where this record and [charter.md](charter.md) disagree, this
record governs.

---

## D1 · The wave, and the order

M49 is the **pre-1.0 completion wave**: **do the work now -> ship 1.0.0 -> then port this repo onto
jigc.** The port is a *later act*; M49 makes it possible.

*Why:* nearly all the port's schema and contract work is cheap before the 1.0 pin and expensive after
— and N2 becomes impossible without minting a contract v2. The charter's contrary reasoning ("this
repo is not self-hosted, so no cheap-now window is being spent") was **right about corpus instances
and wrong about the pin**: the window that matters is the freeze, not our corpus.

## D2 · The razor — one razor, both claws, general

1. **Demonstrated, not asserted** — a rule stated in a locked artifact and violated at HEAD, *or* a
   defect/gap driven on the binary. Both require execution, never a source read.
2. **The fix is right for any adopter** — it improves jigc independent of this repo's conventions.
   **If the only beneficiary is our layout, our naming, or our process, it is refused and we change
   instead.**
3. **Timing is named** — one-way at the pin ⇒ state the concrete post-1.0 cost; not one-way ⇒ it may
   be deferred without ceremony.
0. **On the pre-1.0 critical path** — a **one-way door at the pin**, or a **hole in a declared surface**. This is the necessity leg, and it is the one that refuses.
1. **Demonstrated, not asserted** — a rule stated in a locked artifact and violated at HEAD, *or* a defect/gap driven on the binary. Both require execution, never a source read.
2. **The fix is right for any adopter** — it improves jigc independent of this repo's conventions. **If the only beneficiary is our layout, our naming, or our process, it is refused and we change instead.**
3. **Timing is named** — one-way at the pin ⇒ state the concrete post-1.0 cost; not one-way ⇒ it may be deferred without ceremony.
4. **Falsifier (M46, kept verbatim)** — *if the razor cannot refuse, the claim is wrong.*

**Leg 0 exists because the design review caught the razor unable to refuse (2026-08-28).** The first draft had three legs, and both load-bearing refusals — the increment tier and `AddedNestedRepeatable` — were justified as *"the port does not need it."* That is **port-as-requirements-source**, the very thing this razor rejects, operating as an unwritten fourth test. Leg 0 writes the necessity test down in product terms (a door, or a hole in a declared surface) so it can be argued and checked rather than assumed.

**PB-1's override framing is withdrawn.** Re-adjudicated against the corrected legs, PB-1 **passes**: leg 0 — an adopter cannot extend a pack whose composition is pinned at 1.0 (a hole in a declared surface); leg 1 — driven; leg 2 — the record itself calls it adopter-facing. Recording it as an override *manufactured a refusal to be virtuous about* while the real refusals rode an unwritten rule. An unnecessary override is a worse record than an honest admission.

**The port is a discovery mechanism, not a requirements source.** It surfaces defects; admission is
decided on product quality. An earlier draft made leg 1 *"the port's own acts touch it"* — that makes
our migration the arbiter of the product's shape, and was rejected as backwards.

**Refusals on the record, each at a named leg:** the increment tier (**leg 0** — no door, no declared
hole; `planning-finalize.yaml` already states the gap honestly) · `AddedNestedRepeatable` (**leg 0**,
with the tier) · a delegation primitive (**leg 0** — an invariant revision nothing on the path needs) ·
the `deferral-ledger` / `vision` / `idea` corpus-shape bumps (**leg 2** — the beneficiary is our
document shape; we change instead) · a placement override argued from *our* layout (**leg 2**).

## D3 · The item tier — (A) in, (B) in, (C) out; the increment tier defers

- **(A) The item-region fix — IN, and it is a bug, not a capability.** One root cause, two seams: an
  item's own leaf region ends at the first *deeper* heading, but an item's slot sub-labels **are**
  deeper headings. Write seam (`write.rs:2406` -> `:3441`): >=2 slots + one settable field duplicates
  the field bullet, **commits at exit 0**, `jigc validate` reports "validates clean", and the pinned
  1.0 `doc show --format json` silently returns one of the two values. Parse seam (`parse.rs:974`):
  >=2 slots + nested repeatable ⇒ `doc add-item` acks exit 0 then bricks the doc. Driven on **`spec`**,
  a frozen-v1 *dev-pack* doctype, so the class is not methodology-specific. Fix keyed on the schema
  (the first deeper heading **that is not a declared slot sub-label of this item template**), never on
  byte-sniffing — `surface-contract.md:58` mandates that by name. Three call sites; the acceptance
  iterates `{single-slot, multi-slot, slotless} x {nested, not} x {insert, update, unset}` from the
  schema registry.
- **(B) `AddedItemSlot` — IN.** Not a one-way door (a kind can ship in 1.1), but this is the cheapest
  moment, and without it two named shapes are hard bricks whose only route terminates in
  `crates/engine/src/transform.rs` — a file no adopter can edit. That is not a bound on lock-in cost;
  it is the absence of one.
- **(C) `AddedNestedRepeatable` — OUT.** Nothing needs it once the tier defers.
- **The increment tier (S1) — DEFERRED, with a written trigger.** It does not block the port: the
  migration writes decomposition prose into a slot, exactly as `planning-finalize.yaml` already
  states. Fails leg 1.

*The fact that decided it:* `roadmap.milestones` is the **only** shipped 2-slot item block and has no
settable field. The corpus sits on the safe side of the boundary **by accident of shape** — and
`roadmap.status` is precisely the change that crosses it. The cheap arm does not defer the defect; its
own headline deliverable manufactures its first live instance in the frozen v1 set.

## D4 · Multi-slot ∧ nested — FIXED, not fenced (revised at the design review)

**The first draft fenced it at pack-load. The review killed that on two grounds and both hold.**
(i) The six shipped pack-load fences are scoped to constituents shipping a `schema-manifest.yaml`
(`pack.rs:284`), so the fence would be **inert for exactly the adopter packs D5 ships** — which
falsifies its own load-bearing claim that it *"prevents any corpus from existing in that shape."*
(ii) It would make three of D3(A)'s eighteen acceptance cells unconstructible through any loadable
pack, silently shrinking the axis-completeness claim that is this wave's method.

**So the shape is fixed rather than banned.** The same schema-keyed boundary D3(A) builds resolves
the parse seam too, because the two constructs at `item_level + 1` **are** distinguishable: a nested
item heading carries an `{#id}` anchor and a declared slot sub-label does not. The boundary rule
becomes *the first deeper heading that is neither a declared slot sub-label of this item template nor
an anchored nested item*.

**Declared bound, carried:** on a malformed corpus the anchor discriminator is undefined for exactly
the documents `conformance.item-heading-unanchored` exists to report. That is acceptable on the
**write** path (jigc's own writer always anchors) and produces a conformance finding rather than a
guess on the **read** path — but it must be stated, and the acceptance must include the unanchored and
malformed-anchor arms rather than only the conformant one.

**Consequence:** D4 is no longer a separate deliverable; it folds into D3(A) as the parse seam's half
of one fix, and the increment tier's later un-deferral no longer has a fence to remove first.

## D5 · PB-1 — IN, by explicit decision

`compose-embedded-methodology: true` cannot combine with a `packs:` list, so no adopter can add a
house doctype while using the methodology pack. Brings with it the **precedence model** for listed
packs relative to the two embedded ones (`multi-pack.md:103` leaves it undefined) and the
**`--explain` collision-winner defect**, correct at the two-pack bound and **latent for N>2** — live
the moment this ships, and the provenance header is a declared determinism contract.

*Recorded honestly:* this does not block **our** port (we own the methodology pack and can add
doctypes to it directly). It is an adopter-facing gap — and under the corrected razor it is
**admitted by leg 0**, not by override.

**Two things the first draft named and did not settle; both are now settled, because the review showed
each is load-bearing.**

- **Precedence.** A listed project pack composes **below** the embedded pair for any
  manifest-governed doctype: **a project pack may not shadow a doctype the freeze governs.** Without
  this, the natural reading (`[listed ▸ dev ▸ methodology]`) lets a manifest-less pack shadow a frozen
  dev doctype while `assert_schema_freeze` is skip-on-absent for it — T0-6's class one layer out,
  delivered by a wave item. The `--explain` collision-winner correction ships with it: the shipped
  line names the highest-precedence pack as winner, which is **correct at two packs and wrong at
  three**, against a header `multi-pack.md` calls a hard determinism contract.
- **Precondition — the mis-keyed-leaf guard (`decisions-pending.md` → the schema-load entry).** A
  mis-keyed leaf inside a `repeatable:` block **silently erases the whole section from every surface**,
  and the freeze is what backstops it today. PB-1 *is* that entry's stated trigger. **The schema-load
  `deny_unknown_fields` repair lands before PB-1**, or this wave ships an adopter-facing silent
  section-deletion at exit 0 — in a wave whose Tier 0 is titled *silent corruption and integrity*.

## D6 · Placement — we move; the override ships on product grounds

- **The port moves our files** to `docs/roadmap.md`, `docs/decisions-log.md`,
  `docs/deferral-ledger.md`. No schema event on our account. *We do not bend the CLI to fit the
  project; we migrate the project to fit the product.*
- **`placement.file` becomes cascade-overridable**, argued **solely** on the asymmetry: `location:`
  homes resolve through `docs-root`, `placement:` homes resolve through nothing, and `jigc relocate`
  refuses frozen doctypes — an adopter whose docs are not under `docs/` has **no path at all**.
  Composition-invariance is preserved: a project override is one fixed choice, it does not vary by
  composition (the M37 ambiguity the rationale defends against).
- **Owed:** the `storage.md` placement census (14 sites reading `location`/`placement`, several
  flagged ⚠ silent-data-loss) re-walked against an overridable home.

## D7 · Delegation — prose only

Steps **name the actor and the instrument** (*"validated by an agent that did not build it and cannot
commit"*, *"commission an independent robust-advocate brief before recommending a defer"*). jigc
dispatches nothing, counts nothing, scores nothing.

*Why this is a sweep, not a new feature:* `plan-review.yaml` **already** says *"A reader who did NOT
author the decisions…"* — the capability ships, applied at exactly one site, absent from its class.
It honours `DECISIONS.md` 2026-06-21 (*the orchestration does not fold*), which bars jigc from
**orchestrating** delegation and says nothing about a step **stating who must act**; and it applies
`finalize.md:131`'s standing rule — *"The contract only binds if the composed workflow says it."*

**A delegation primitive in the dialect — DEFERRED with a trigger.** It is an invariant revision, and
nothing in the port needs jigc to dispatch.

## D8 · The pinned contracts — N1 and N2 both taken

- **N1 — IN.** One nested-section-hop defect answers with four codes across six item-addressing doors;
  drivers key on `(code, target)`. Cheap now, breaking after. M47 refused it **for scope, not merit**.
  Authorization template: `command-output-contract.md:394`.
- **N2 — IN, taken ADDITIVELY (revised at the design review).** The first draft said *"unified to
  NUMBER, `doc schema` contract-version 5 -> 6"* and was **internally contradictory**: `doc schema`
  already emits a number, so it is the surface that does **not** change, and there is no basis for
  bumping it. The surface that would change is `doc show` — which has **no version integer at all**.
  The draft also silently broke a stated universal: `doc show`'s `fields` map is produced by the
  generic projection whose own doc-comment reads *"A scalar field serializes as its string"*
  (`doc.rs:4292`), so typing one key makes the map heterogeneous and breaks any driver iterating it.
  **Settled instead:** `fields` stays uniformly stringy, and `doc show` gains a **top-level
  `schema-version` integer** beside it. The driver gets its cast-free cross-surface comparison, no
  existing key changes type, and **no contract-version moves on either surface**. Purely additive —
  which the pre-1.0 window covers, and which is why this must land now rather than after the pin.

*Decided deliberately, per `doc-read-surface.md:177`'s own demand that a fix wave must not decide N2
by accident.*

## D9 · `planning-record` — ships, in a third shape

Phase 4.5 has no doctype; nine hand-built gate-records exist; the ledger's third-demand rule stands at
**five**. **Free at the freeze** — a new doctype moves no hash and owes no snapshot.

**Shape — required slot per gate (reverted at the design review).** The first draft settled a
repeatable `gates` section whose names were data, to make a new gate cost no schema bump. **The review
showed that deletes the reason the doctype exists.** `methodology-docs.md:79` is explicit: the forcing
function is `required-slot-present` — *an empty gate-slot **blocks `finalize`** on the planning task*.
A repeatable has no such fence: `min-items` was **rejected as a frozen trap** (`validation.md:569`),
and `repeatable-populated` fires only at **zero** items, is **advisory**, and is **store-scope only,
never a per-task gate**. So a `planning-record` carrying **3 of 14 gates, each filled, validates clean
and finalizes at exit 0** — the doctype becomes prose an agent skims, which is the state
`methodology-docs.md:52` says it exists to end.

**So: one required slot per gate**, as the design of record always said. The bump-per-new-gate cost is
real and accepted — the doctype is **new**, so the freeze is free, and a future gate is a
`ProseNeeding` bump on a doctype whose whole corpus is ours. `methodology-docs.md:77`'s commitment that
the M42 cell text *"moves verbatim into the schema's `hint:`"* therefore **stands unrevised**, and the
per-gate guidance stays readable through `jigc doc schema planning-record` — the surface an agent reads
before writing.

**The gate set is 14**, settled in `design/methodology-docs.md` as the single home.
`implementation/milestone-planning-workflow.md:63` cross-references it instead of restating — it
currently lists **13**, omitting `quote-attributed`, **under a paragraph titled "Accretion
discipline"**, while line 46 of the same file discusses that very gate.

## D10 · `completion-record` 1 -> 2

- **`severity` widened to HIGH/MEDIUM/LOW** (`EnumWidened`, byte no-op). Blocks the port today: every
  real audit uses those words and the shipped enum rejects them at exit 1.
- **Prose evidence — as a NEW optional `detail` slot, not by converting `evidence` (revised at the
  design review).** The first draft said the item block has zero slots, so this was the benign 0->1
  arity. **That was wrong**: `evidence` ships today as a `type: string` **field**, so converting it is
  `RemovedField` + `AddedItemSlot` — and `RemovedField` is **refused by design**
  (`corpus-migration.md:188`), which this record's own corrections table already states
  (*"the claim holds at 1->2 and scalar->prose"*). As drafted, a build increment would reach
  `migrate-corpus` and halt. **Settled:** `evidence` stays a scalar; an **optional `detail` slot**
  joins the item block — the genuine 0->1 arity, which (B) carries. No third engine kind, and
  `corpus-migration.md:188`'s *"no frozen doctype needs a removal"* predicate stays true.

---

## Scope — every baseline-ledger row disposed

*Added at the design review: the first draft disposed ten **forks** and never stated the wave's
**scope**, so a decompose session would have invented it. Each row below is IN, DEFERRED with a
trigger, or OUT with a ground, adjudicated against the corrected razor (leg 0 · leg 1 · leg 2).*

### Tier 0 — integrity · all IN
| row | disposition |
|---|---|
| **T0-1** the item-region class, **both seams** (write + parse), schema-keyed boundary + anchor discriminator | IN — D3(A)+D4 |
| **T0-2** `set:` is an open string; a typo silently exempts a required field and is serialized by the pinned `doc schema` | IN — a **closed `set:` vocabulary** rejected at pack-load. Leg 0: one-way (`set:` is inside every `schema-hash` and crosses contract-v5) |
| **T0-3** `milestone list-tasks` (`VerbKind::Read`) reseeds a discarded sub-task with the wrong workflow | IN — violates the registry's own stated predicate at `cli.rs:1348` |
| **T0-4** `add-task --workflow <bogus>` accepted at exit 0 ⇒ a permanently unreachable sub-task | IN |
| **T0-5** `task discard` of a sub-task leaves the committed record claiming `status: active` | IN — M42's `milestone discard` class, sibling never swept |
| **T0-6** the project-layer schema shadow bypasses `assert_schema_freeze` entirely | IN — **and a precondition of D5**, since PB-1 makes project packs real. Falsifies CLAUDE.md's freeze invariant for the project layer |

### Tier 1 — engine
| row | disposition |
|---|---|
| **T1-1** `AddedItemSlot` | IN — D3(B) |
| **T1-2** `AddedNestedRepeatable` + nested arms | **DEFERRED**, trigger written (leg 0) |
| **T1-4** three nesting ceilings; `doc schema` advertises addresses the write path rejects | IN — **reconcile the surfaces, do not raise the cap**: stop advertising unaddressable addresses, and make the ceiling one stated number. The increment tier sits at depth 2, inside the real cap |
| **T1-5** `doc author` ~O(n³) — 800 items 98s, 1500 killed | IN — every `migrate` workflow authors in one payload; bounds MIGRATING.md's shipped promise (leg 2) |
| **T1-6** `add-item` has no `--slug`; 890 titles → 884 slugs | IN — a CLI capability, no schema event, and it generalises to any adopter with colliding titles |
| **T1-7** PB-1 | IN — D5, by leg 0 |

### Tier 2 — pinned contracts · all IN
N1 (four codes → one) · N2 (additive top-level integer) · **D1** the text renderer carrying
`location.address` + line — which is what makes the route exemption's own stated rationale true ·
**D5** the route batch: narrow `is_route_exempt` from a bare prefix match to an **enumerated set with
a stated reason per member**, the shape `is_declared_singleton` already uses two functions above it ·
the gate repair routes gaining `--task <id>` (3 codes × 3 doors) · the two `conformance.*` codes with
**no producer at all**, removed or given one.

*Recorded, not fixed:* the route-floor assert is a `debug_assert!` and therefore release-inactive —
**by declared design**. Every "the floor catches this" claim is about the debug seam suites. Left as
is; noted so no acceptance leans on it.

### Tier 3 — schema
| doctype | disposition |
|---|---|
| **`completion-record` 1→2** | IN — severity → HIGH/MEDIUM/LOW (`EnumWidened`) + a new optional `detail` slot |
| **`planning-record`** (new) | IN — required slot per gate, 14 gates, free at the freeze |
| **placement override** | IN — D6, on the product asymmetry alone |
| **`milestone-record` 2→3** | IN — a per-task `workflow` leaf. Not our convenience: without it the recorded workflow is **not fresh-clone durable**, and the W-equality guard then asserts **false provenance** with full confidence (driven). Leg 0: a frozen bump, one-way at the pin |
| **`roadmap` 1→2** (`status`) | **DEFERRED with the increment tier** — leg 0: no door, no declared hole. This also removes the ordering hazard the ledger flagged |
| **`deferral-ledger` 2→3 · `vision` 1→2 · `idea` 1→2** | **OUT — leg 2.** These are our corpus's shape (`(Q)` kinds, 12 `##` sections, multi-paragraph triggers). *We migrate the project to fit the product*, not the reverse |
| **`decisions-log`** | no schema change — the blocker is identity, answered by T1-6 |

### Tier 4 — the surface wave · IN, in full
The charter's original scope, every row: `doc schema`/`doc list` named 0× in either pack and 0× in
`SKILL.md` · the heredoc form nobody names against `--from-file -` 42× · the clap seam (**2 of 16**
error kinds; `UnknownArgument` covered by one curated row) · PT-A's routeless occupancy refusal and
its sibling at `milestone provision` · C2's `write.unknown-section` universal falsified at two of three
write verbs · E1 `doctype-authoring.md` naming **none** of the M43–M48 fences **and** its transform
matrix missing the `AddedItemSlot` row while its nearest row mandates the wrong conclusion · E2/F3 and
their unlisted siblings · A4's three unreasoned catalog absences · **the `create.singleton-copy-in`
fence that mandates a false statement on five steps while four unfenced steps state it correctly** ·
`describe` listing 16 commands while composition resolves the union · the fan-out spawn line
hardcoding `sub-task` · three unsubstituted `<MILESTONE_ID>` · `finalize.render-io` misdiagnosing a
never-composed sub-task as a disk fault · the unknown-doctype axis (7 doors, 5 shapes, a
`PackResourceKind` Debug leak) · the not-a-git-repo axis (3 texts, 11+ sites) · **the 20 falsified doc
statements across 12 files** · and the *"Notation is illustrative"* header scoping, so a stated rule in
a disclaimered doc is citable and a YAML sketch is not — without which the razor's leg 1 refuses by
**which doc an item cites**.

### Ordering constraints — the wave is not freely reorderable
1. **D3(A) before any bump that puts a settable field on a ≥2-slot item block.** (`roadmap.status`
   deferring removes today's instance; the constraint stays, because it binds the next one.)
2. **T0-6 (the shadow freeze-bypass) and the mis-keyed-leaf guard before D5 (PB-1).** PB-1 makes
   project packs real and both holes are adopter-facing the moment it lands.
3. **D3(B) `AddedItemSlot` before D10**, which consumes it for the `detail` slot.
4. **T1-1/T1-2's classifier work before any Tier-3 bump** that is not a pure `EnumWidened`.

### Acceptance — one correction to how the axis is built
D3(A)'s eighteen cells are **not enumerable from the shipped schema registry**: across both packs,
`roadmap.milestones` is the only 2-slot block and `changelog.releases/changes` the only nested one, so
the registry populates **2 of 18**. The axis is a **shape space that must be manufactured** — synthetic
fixture packs built by the trial-corpus substrate — which is a deliberate departure from M45's and
M47's registry-enumeration pattern and must be written as one, or an increment will loop over the
registry and call two cells axis-complete.

*Also corrected:* the class was driven on a **modified** `spec` (the shipped `spec.criteria` has one
slot). The conclusion — the class is not methodology-specific — stands; the sentence must not read as a
shipped repro. And the write seam corrupts on the **first** write (the default bullet plus the new
one), then appends unboundedly; `doc show --format json` returns the **first, stale** value, actively
contradicting the write it just acknowledged.

## Corrections this Settle put on the record

| corrected | by what |
|---|---|
| P1 *"two packs describe the same phase incompatibly"* | **FALSIFIED** — different tiers; `execute.yaml` ∈ `increment`, `implement-tasks.yaml` ∈ `milestone-execution` |
| P2 *"`robust-advocate` absent = a law-2 gap"* | It is a **recorded decision** (2026-06-21), not a gap |
| *"the migration seam lands silently"* | **REFUTED** — three guards fire (`unclassified-change`, `prose-needed`, `fold-refused`); no false green at any arity |
| *"no legal path from scalar to item prose, ever"* | **Too strong** — a manual Framing-A path works at 0->1; the claim holds at 1->2 and scalar->prose |
| *"the depth cap blocks the increment tier"* | **REFUTED** — depth 2 is inside the real cap |
| the carryover standing order | **REFUTED** — the gate protects pre-mint staging (3 arms driven). Residue: no `MINT_DOORS` enumeration |
| `milestone provision` non-rollback | **REAL but LOW** — recovery holds as declared. Residue: the code-less anyhow + `execute` narrating a half-provisioned set as ready |
| `decisions-pending.md:146` *"CLOSED — take neither"* | **Superseded** by D1 as a **basis-has-changed reversal**, not an override |

## Carried, and to be recorded as owed

The tier's deferral · the delegation primitive · `AddedNestedRepeatable` — **each with a written
trigger**, because the multi-slot ∧ nested ledger entry fired unnoticed once already and that is the exact
failure the ledger exists to prevent.
