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

---

## Dispositions — the conversion ledger, appended 2026-08-31 (Increment 12 / T2)

**This section is APPENDED, not a rewrite.** Everything above is the dated, driven baseline as it
stood at `99231f6` and is left exactly as measured; what follows is one line per finding id saying
how the wave disposed of it. [pinning.md](../../../implementation/pinning.md) §3 and its 2026-08-18
addendum govern: a citation is verified by **reading what the test asserts**, never by the name
looking apt, so every `pinned-by:` below carries the one clause saying what its suite asserts, and
a candidate that asserts something *adjacent* takes `UNPINNED: <why>` instead of lending its name.
**No parser ships** — §3 refuses a `pinned-by:` symbol parser by name, because the machine-checkable
half (a test *name* exists) is not the load-bearing half.

**Where an id had to be minted, it is minted here and the item it names is quoted.** Tier 0 and
Tier 1 ship their own ids. Tier 2's first four bullets do (`N1`, `N2`, `D1` — the ledger writes it
`D1 (=C2)`, and the disposition keys on `D1` — and `D5`); its last two
carry none, so they take `T2-5` / `T2-6` in the order they are listed. Tier 3's table rows take
`T3-1`…`T3-7` in table order and its *Also owed* paragraph `T3-8`. Tier 4's run-on paragraph takes
`T4-1`…`T4-14`, one per `·`-separated item, in order. `T5-1`…`T5-3` are the three RESIDUES the
*Refuted / de-scoped by driving* section names — the missing `MINT_DOORS` fence under the carryover
refutation, and C7 and C8 under `milestone provision`; that section's third bullet (the depth cap)
names no residue and takes no id. They are not tier findings, but each names an owed item and all
three were discharged by this wave, so leaving them undispositioned would be the lumping failure at
a smaller scale.

**Four sections above carry no finding rows and are excluded by name**, so the exclusion is a
statement rather than an oversight: *Charter reconciliation* (verdicts on someone else's claims,
each already CONFIRMED / CORRECTED / FALSIFIED against the binary), *Open forks for the human* (the
Settle's inputs — [settle-record.md](settle-record.md) is their disposition of record), *Fired
triggers never resurfaced* (pointers into `decisions-pending.md`, which increment 12 / T3 owns), and
the three refuted verdicts in the last section, whose residues are the `T5-*` rows.

### The done-criterion, one command

```sh
v=completions/artifacts/RC-1.0-final/findings-verification.md
b=completions/artifacts/M49/baseline-ledger.md
for r in S-1 S-4 PT-A PT-C R-1 R-2 S-5 PT-D; do
  awk -v id="$r" 'index($0,"## "id" ")==1{f=1;next} f&&/^#/{f=0} f' "$v" \
    | grep -q 'pinned-by:' || echo "NO DISPOSITION: $r"
done
for r in I-1 I-2 I-3 I-4 I-5; do
  grep -E "^\| $r \|" "$v" | grep -q 'pinned-by:' || echo "NO DISPOSITION: $r"
done
for r in T0-1 T0-2 T0-3 T0-4 T0-5 T0-6 T1-1 T1-2 T1-3 T1-4 T1-5 T1-6 T1-7 \
         N1 N2 D1 D5 T2-5 T2-6 T3-1 T3-2 T3-3 T3-4 T3-5 T3-6 T3-7 T3-8 \
         T4-1 T4-2 T4-3 T4-4 T4-5 T4-6 T4-7 T4-8 T4-9 T4-10 T4-11 T4-12 T4-13 T4-14 \
         T5-1 T5-2 T5-3; do
  n=$(grep -cE "^- \*\*$r\*\* · (pinned-by|UNPINNED): [^ ]" "$b")
  [ "$n" = 1 ] || echo "NO DISPOSITION ($n rows): $r"
done
grep -n 'UNPINNED:' "$v" "$b" | grep -v 'UNPINNED: [^ ]' && echo "UNSTATED REASON"
```

It exits clean — silent, no output — iff every id above carries exactly one disposition line and
every `UNPINNED` states its reason on the same line. It is the mechanical floor only; the citations
themselves are held by §3's read-the-assertions rule, which no command can take over.

### Tier 0

- **T0-1** · pinned-by: `item_region_shape_space::{slotless,single_slot,multi_slot}_{flat,nested}_insert_update_unset` — the eighteen cells of `{slotless, single-slot, multi-slot} × {nested, ¬nested} × {insert, update, unset}`, each driven `doc create` → `add-item` → `set-field` / `--unset` through the real binary and each asserting BOTH faces of the repaired seam: the staged bytes on disk and the pinned 1.0 `doc show --format json` read-back, so the write-acks-one-value / read-returns-another contradiction cannot pass at any shape; `::the_space_is_the_whole_cartesian_product` asserts the six shape functions cover the product exactly; `::the_registry_cannot_supply_this_axis` measures, against both packs' loaded schemas, the two structural facts that justify manufacturing the space (no shipped block is multi-slot ∧ nested, and no shipped multi-slot block carries a settable field) — so if a pack ever ships one, the departure from registry-enumeration reddens where it is claimed. The reported seams are held at their own repros by `item_region_boundary::two_writes_on_a_multi_slot_item_field_leave_one_bullet` (the write seam: two ordinary `set-field` writes leave one bullet, and `::the_json_read_returns_the_value_that_was_written` cross-checks the read), `::a_multi_slot_nested_item_parses_and_reads_back` (the parse seam), `::a_repeated_declared_field_bullet_blocks_with_its_route` (the new conformance finding for the three-values-on-a-`0..1`-enum state that validated clean), `::an_unset_of_an_absent_item_field_acks_the_no_op` (the false `write.not-present`), and `::an_unanchored_deeper_heading_is_reported_not_guessed` / `::a_malformed_anchor_on_a_deeper_heading_is_reported_not_guessed` (the declared-bound arms). The causal root — `doc-read-surface.md`'s false *"no shipped doctype declares a multi-slot repeatable"* — is swept by `record_stale_reasons::the_multi_slot_claim_is_derived_from_the_shipped_schemas`, which derives the claim from the shipped schemas instead of restating it, and the rule's home by `item_region_rule_stated::parsing_md_states_the_schema_keyed_boundary_rule` + `::parsing_md_line_citations_resolve_to_the_enforcement_sites`.
- **T0-2** · pinned-by: `set_kind_vocabulary::an_unhonored_set_kind_is_refused_at_pack_load` — a mutated `JIGC_PACK_DIR` pack whose manifest-listed doctype declares `set: on-creat` makes the invocation exit non-zero naming the field, the offending value and the honored set, while `::the_honored_spelling_of_the_same_fixture_composes_clean` proves the refusal keys on the VALUE and not on the fixture's presence; `::load_schema_returns_a_typed_unknown_set_kind` asserts the engine states it as `SchemaError::UnknownSetKind` rather than a stringly message; and `::no_schema_hash_moved_and_the_pinned_projection_is_byte_identical` discharges the one-way-door obligation — no entity in either manifest has its `schema-hash` move while its co-located version stands still, and `jigc doc schema adr --format json` is byte-identical to its `HEAD` capture, so closing the vocabulary re-pinned nothing quietly.
- **T0-3** · pinned-by: `read_verb_acts_nothing::no_read_verb_acts_over_the_revealing_state` — it enumerates the `VerbKind::Read` rows out of `cli::cli::VERB_KINDS` (the registry, never a reported instance), drives each through the real binary from its own pristine copy of a fresh-clone state built to REVEAL a write (a committed `milestone-record` naming two sub-tasks whose areas are absent, one minted under a non-default `--workflow`), and asserts the whole repo tree — the gitignored `.jigc/` workbench included — is byte-identical afterwards, with one admitted carve-out (`.jigc/index/`) whose two safety properties are themselves asserted rather than assumed: the file carries the repo's current HEAD as its stamp, and a second run changes nothing. `::every_read_verb_has_an_invocation` is the enumeration fence (a thirteenth `Read` row cannot ship without an answer here); `::list_tasks_serves_a_fresh_clone_from_the_record_without_writing` and `::list_tasks_still_refuses_a_settled_milestone` hold the repaired verb's own behaviour.
- **T0-4** · pinned-by: `milestone_workflow_membership::a_bogus_workflow_is_refused_at_every_sub_task_door` — the axis is `SUB_TASK_DOORS`, taken from `MilestoneCommand`'s two `workflow: String` arms rather than from the reported repro, and each cell asserts the refusal lands BEFORE anything mints: no sub-task area is written and no record commit is made, so the permanently-unreachable state the exit 0 produced cannot be entered; `::a_valid_non_default_workflow_still_lands_at_every_sub_task_door` is the omitting context that proves the check did not close the legitimate path, and `::the_other_workflow_doors_already_refuse_an_unknown_id` asserts the axis boundary rather than leaving it implied.
- **T0-5** · pinned-by: `subtask_discard_record::discarding_a_sub_task_settles_its_record_item_in_one_record_only_commit` — `task discard` becomes the fifth record-only door and settles THAT ONE item to `discarded` while its siblings and the record header stay byte-untouched, in a single record-only commit; `::a_rejected_sub_task_discard_restores_the_pre_image_and_frames_its_rerun` holds the transaction's refused half; `::no_door_reaching_the_reseed_site_rebuilds_a_discarded_sub_task` sweeps the reseed skip over EVERY door reaching the shared reseed site, with `DOOR_ARMS` bijected against the call sites enumerated from `crates/cli/src/milestone.rs` so a door added or removed reddens instead of escaping; `::an_ordinary_task_discard_writes_no_record_and_lands_no_commit` is the omitting context (no milestone names the task → no record, no commit).
- **T0-6** · pinned-by: `freeze_enforcement::a_shape_changing_project_schema_shadow_blocks_every_door` — a `.jigc/config/schemas/<t>.yaml` that reshapes a manifest-governed doctype now refuses at pack-load at every door, while `::a_presentation_only_project_schema_shadow_composes_clean` asserts the fence keys on the schema-hash's presentation projection and not on the shadow's existence, and `::a_project_shadow_of_a_manifest_less_pack_doctype_loads_clean` keeps the cascade's own capability intact where no manifest governs the id. The second half — two doors of one binary answering differently about one file — is `schema_resolution_unified::every_production_schema_read_goes_through_the_shared_resolver`, a SOURCE-level fence asserting membership where it is decided (so the eighth shadow-blind surface reddens when it is written, not when a trial finds it), backed by six driven contradiction arms (`::ingest_and_validate_agree_about_the_document_at_the_shadowed_home`, `::create_over_a_committed_doc_at_the_shadowed_home_copies_it_in`, `::finalize_promotes_to_the_resolved_home_the_read_surfaces_use`, `::milestone_add_from_spec_reads_the_spec_at_the_resolved_home`, `::the_provisioned_commit_form_carries_the_resolved_commit_schema`, `::a_bare_address_expands_for_a_doctype_the_cascade_makes_a_singleton`). The sibling hole — a mis-keyed leaf silently erasing a whole section at exit 0 — is `schema_load_strictness::mis_keyed_repeatable_field_leaf_blocks_instead_of_erasing_the_section` + `::stray_key_on_a_slot_leaf_blocks`, driven over the manifest-less pack that is the only state where nothing else backstops the typo.

### Tier 1

- **T1-1** · pinned-by: `migrate_corpus_item_slot::zero_to_one_optional_migrates_with_the_stamp_as_its_only_byte_delta` + `::one_to_two_optional_carries_the_committed_prose_under_its_sub_label` + `::one_to_two_required_routes_at_the_author_never_at_the_source_tree` + `::two_to_three_mints_the_new_sub_label_with_the_existing_ones_untouched` — the axis is arity × requiredness, every cell driven through the SHIPPED binary over a manufactured `JIGC_PACK_DIR` pack whose reshaped doctype is re-pinned through the production loader (the act a pack author performs), and every cell asserts the BYTES the migration left rather than a reconstruction: the 1→2 optional cell asserts the committed bare prose rides verbatim under the v1 slot's `#### <Leaf-Title>` and that `render(parse(x)) == x` holds over the emitted bytes; the required cell asserts the route is the doc-authorable `migrate-corpus.prose-needed` with the bytes rolled back and the stamp unmoved — never `unclassified-change`, and never a route naming `crates/engine/src/transform.rs`, a file no adopter can edit. `::an_item_carrying_unmodelled_bytes_is_refused_never_rewritten` and `::the_unmodelled_bytes_guard_holds_at_two_to_three` hold the destroy-nothing bound; `::a_re_run_leaves_an_already_reshaped_item_byte_identical` the idempotence.
- **T1-2** · pinned-by: `migrate_corpus_item_leaf_residual::an_added_nested_repeatable_leaf_blocks_the_run` + `::a_removed_nested_repeatable_leaf_blocks_the_run` + `::a_reshaped_nested_repeatable_leaf_blocks_the_run` — the NO-SILENT-DROP half only, and each is driven ALONGSIDE a classified change, which is exactly where HEAD was silent (it migrated and restamped the doc): the delta now names itself by emitting the backstop kind explicitly instead of vanishing into a non-empty diff. UNPINNED: the KIND half — `AddedNestedRepeatable` is not built, so `roadmap` v2 still cannot recurse into a nested block; the deferral stands at its pre-existing, untouched trigger (`decisions-pending.md`, the no-nested-transform-arm entry this ledger lists under *Fired triggers never resurfaced*), and pinning current behaviour any harder would pin the absence as expected output.
- **T1-3** · pinned-by: `migrate_corpus_item_leaf_residual::a_removed_item_slot_refuses_with_a_slot_naming_route` + `::an_item_slot_tightening_routes_the_author` + `::an_item_slot_relaxation_migrates_as_a_byte_no_op` — the backstop stops being asked to carry a classifiable change: a removed item slot refuses with a message that names a SLOT (the prose it would destroy is not a field line), a tightened `optional:` routes the author, and a relaxed one folds to a byte no-op under the one shared rule. The record half — the retired fact *an added item slot has no kind and rides the residual*, stated in three homes — is `record_item_slot_kind::the_record_no_longer_states_that_an_item_slot_has_no_kind` + `::no_matrix_row_naming_an_item_slot_says_its_kind_is_none` (structural: the matrix row's *kind* cell, not a phrasing match) + `::each_replacement_is_stated_exactly_once`; the first two sweep `crates/engine/src/schema_diff.rs` alongside the two `.md` homes — the falsified-statement table names the CODE file, so a sweep stopping at the docs cannot leave `diff_item_fields`' doc-comment contradicting the design doc that cross-references it — and `::the_rows_t2_touched_say_what_t2_shipped` asserts the three matrix rows T2 touched (the nested-repeatable row, `ProseNeeding`, `OptionalRelaxed`) say what T2 actually shipped.
- **T1-4** · pinned-by: `doc_read_surface::a_doctype_nested_to_the_cap_advertises_only_addressable_addresses` + `::a_doctype_nested_one_level_deeper_is_refused_at_pack_load` — driven: at the cap every address the projection advertises parses and round-trips, and one level deeper is refused at pack-load rather than advertised-and-rejected, which is what makes the three ceilings one number (`engine::schema::MAX_NESTING_DEPTH` derived as `(MAX_FRAGMENT_HOPS − 1) / 2`). The record half is `record_nesting_cap::the_record_no_longer_states_the_retired_cap` + `::the_nesting_subject_is_never_capped_at_the_retired_number` + `::the_nesting_cap_is_never_attributed_to_the_heading_ceiling` + `::deeper_nesting_is_never_stated_as_an_existing_capability` — a fact sweep over the conjunction (subject × retired number, subject × retired reason), not a phrasing sweep, across the five live homes including the one unenumerated until 2026-08-30.
- **T1-5** · pinned-by: `author_batch_scaling::the_batch_apply_growth_ratio_stays_within_its_stated_bound` — it pins the GROWTH RATIO between two payload sizes one octave apart, not a wall-clock ceiling: a constant-factor machine speedup moves both measurements together and leaves the ratio alone, so only a change of shape moves it (measured n²·⁹ → n²·¹; 800 items 77 s → 6.7 s, 1500 never-finished → 26 s); `::the_eight_hundred_item_document_is_byte_identical_to_the_captured_golden` asserts the two new parse indexes changed no parser decision. The residual quadratic bound is now STATED, in `MIGRATING.md` — the *"stated nowhere"* half of the finding is discharged by writing it down, not by a test.
- **T1-6** · pinned-by: `add_item_slug::two_titles_that_slug_alike_both_mint_when_the_second_supplies_a_slug` — it drives the EMITTED address verbatim (the address `add-item` prints is the one the follow-up read runs, because the acked address was itself derived from `slugify(title)` and is exactly what an override must move), shows the collision is real by asserting the un-overridden second mint is refused, and reads each item back at its own address with `id` equal to the override and its own heading text intact; `::a_malformed_slug_is_rejected_before_any_bytes_move` (the staged doc byte-identical afterwards, never silently re-slugified), `::an_enum_id_from_refuses_an_override_with_a_route`, `::an_override_mints_a_nested_item_at_the_depth_cap` and `::retitle_item_over_an_override_minted_item_leaves_the_anchor_frozen` close the surrounding cells.
- **T1-7** · pinned-by: `project_pack_composition::a_listed_pack_adds_a_doctype_the_freeze_does_not_govern` + `::a_listed_pack_may_not_shadow_a_doctype_the_freeze_governs` + `::the_methodology_pack_is_not_lost_when_a_project_pack_is_listed` + `::the_composed_set_validates_clean` — driven through the real binary over a repo carrying `compose-embedded-methodology: true` AND a listed pack shipping both a new `note` doctype and a divergent `commit`: extension works, the demotion holds on the emitted `doc schema commit` bytes, and the methodology pack survives, which is the whole point of the combination. The universal — the demotion holds for EVERY governed id — is `cli::pack::tests::a_listed_pack_shadows_no_governed_doctype_but_still_extends`, which reads the governed set from the two embedded manifests (`governed_doctype_ids`, never hand-listed), stands up a house pack shadowing EVERY governed doctype plus one id no manifest governs, and asserts the shadowing pack wins NONE of the governed ids while the ungoverned one still resolves listed-first — with the shadowed listed owner still ENUMERATED, demoted last, so `--explain` can name the loss; `::the_freeze_demotion_does_not_reach_other_id_spaces` bounds the demotion to `Schemas`. `::explain_names_the_pack_that_actually_provided_each_resolved_id` + `::explain_names_the_demoted_project_doctype_as_the_loss_it_is` pin the collision-winner repair (correct at two packs, wrong at three); `::setup_wires_the_marker_over_a_projects_own_pack_list` and `freeze_enforcement::the_setup_written_marker_never_silently_drops_a_listed_pack` hold the install path. The *project pack is a fork, not a delta* half stands as a declared bound in `design/multi-pack.md`, not as a test.

### Tier 2

- **N1** · pinned-by: `write_miss_shape_axis::every_write_miss_names_its_own_miss_and_routes_the_recovery` — the undeclared-section column gains its nested arm: seven rows (the six item-addressing verbs plus the nested dual of the bare `add-item` cell) all converge on the shipped `write.unknown-section` with its schema read, where one miss previously answered under four different codes; and the key's other half is asserted as a PROPERTY OF THE MATRIX rather than a per-row column — wherever a cell's argv carries a write address, the finding's `key.target` and its `location.address` must be that address verbatim and in full — so a row added later inherits it. `::every_doc_write_verb_taking_a_section_hop_answers_an_undeclared_one` is the enumeration fence: every `doc` write verb in `VERB_KINDS` either drives an undeclared section hop in `CELLS` or states in `NO_SECTION_HOP` why it takes none, exactly one of the two. No new code was minted, because a new contract member is itself a spend.
- **N2** · pinned-by: `doc_show_staged::the_doc_stamp_rides_the_whole_doc_serve_as_a_number_and_compares_cast_free` — it asserts BOTH surfaces speak the same json type (`is_u64()` on `doc show`'s and `doc schema`'s `schema-version`) and then compares the two values directly, so the cross-surface comparison a driver automates needs no cast; it also pins the value against a doctype whose manifest version is 2, not a constant 1, and `::an_unstamped_doctype_serves_a_null_schema_version` pins the absent case as `null` rather than a fabricated zero; `doc_show::the_stamp_key_is_whole_doc_only_and_fields_stays_stringy` asserts the additive key is whole-doc only (a `#fragment` slice carries none) and that the `fields` map stays uniformly stringy — the discriminating fragment being the header's fields-only slice, the one shape that could have carried a second, divergent `schema-version` entry. No `contract-version` moved, which is the spend this shape was chosen to avoid.
- **D1** · pinned-by: `located_finding_text::a_located_finding_names_its_own_address_and_line_in_agent_text` — the agent text names the finding's OWN `location.address` and line, cross-read from the same run's `--format json` so the assertion cannot pass over a reconstruction; `::a_location_less_finding_renders_exactly_as_before` asserts the new line appears where there is a locus and nowhere else; `::every_text_render_of_a_finding_is_disposed` is the source-derived sweep — every production read of a `Finding`'s `message` in `crates/cli/src` carries a verdict and a reason, and a `Carries` verdict is checked against the source rather than believed. Together they make the route exemption's own stated rationale (*"the located message **is** the repair"*) true on the surface an agent actually reads.
- **D5** · pinned-by: `engine::finding::tests::the_route_exemption_is_an_enumeration_not_a_namespace_prefix` — an invented `conformance.*` code nobody emits is NOT exempt (the prefix predicate admitted it by spelling); `conformance.duplicate-field`, which carries a route, needs no row and takes none, so the criterion *exemption means may* is checked on the one member it applies to; and every member of `CONFORMANCE_PARSE_DIAGNOSTICS` is exempt. `::every_production_blocking_finding_is_routed_or_exempt` stays green over the NARROWED predicate, and that green is the completeness proof — an applied mutant (a new route-less blocking `conformance.*` producer) reddens it where the prefix predicate absorbed the same mutant silently. `::every_conformance_code_named_in_engine_source_has_a_producer` closes the reverse direction, which is how the two producer-less names this ledger counted were disposed.
- **T2-5** · pinned-by: `route_followability::every_gate_repair_route_runs_at_every_boundary_door` — *"Gate repair routes omit `--task <id>`"*: rows are `engine::validate::conformance_repair_codes()` (the engine's own mechanical arms, asked rather than re-listed) and columns are `cli::render::BOUNDARY_DOORS` (the code-side door registry), neither hand-listed, and a door the registry gains with no arm is a hard panic rather than a silent skip; every cell is driven IN A REPO WITH TWO OPEN TASKS — the state that made the emitted argv unrunnable — and asserts the emitted argv names the task, RUNS VERBATIM, and that re-running the door proves the finding it emitted is gone.
- **T2-6** · pinned-by: `route_followability::every_gate_repair_route_runs_at_every_boundary_door` for the half that shipped — a route DERIVED from another route (the boundary-door task-selector enrichment) cannot know in advance that its result parses, so `cli::doc` asks `route_fence::accepts` as a QUESTION in every posture and emits the derived route only when the answer is yes; the arm above then runs the emitted bytes, and running emitted bytes is a property of the output, not of the build posture. UNPINNED: the general claim stands — `Route::mechanical`'s own constructor fence is still `debug_assert!`, so a release-only route-floor breach on a NON-derived route is caught by the debug seam suites and not by the shipped binary; that is a declared bound, unchanged by this wave.

### Tier 3

- **T3-1** · UNPINNED: `roadmap` 1→2 did not ship. Its tier half needs T1-2's `AddedNestedRepeatable`, which stays deferred, and this ledger's own rule for the row — *"must be ONE bump"* — forbids landing the field and ref halves alone; the manifest still reads `roadmap: schema-version: 1`, so nothing is half-migrated. Owed, with the nested-transform trigger as its gate.
- **T3-2** · pinned-by: `milestone_record_workflow_leaf::a_fresh_milestone_record_mints_at_schema_version_3` + `::a_committed_v2_record_migrates_with_the_stamp_as_its_only_byte_delta` + `::the_migrated_record_validates_with_no_workflow_bullet_on_any_item` + `::doc_schema_projects_the_workflow_leaf_on_the_tasks_item_block` — the 2→3 bump is an `AddedItemField` for a `default`-less, machine-maintained leaf, so the migration's ONLY byte delta is the stamp and an unmigrated record still validates; the durability the bump exists for is `milestone_record_fresh_clone::an_overridden_workflow_survives_the_clone_and_re_enters` + `::an_un_overridden_sub_task_re_enters_under_the_recorded_pack_default` + `::a_pre_bump_record_with_no_workflow_leaf_falls_back_to_the_cli_default`, and `::no_home_still_says_the_record_lacks_the_minting_workflow` retires the statement the bump falsified. The store's first SECOND snapshot for one doctype is `snapshot_store_two_snapshots::the_two_milestone_record_snapshots_are_distinct_shapes_keyed_by_version` + `::a_v1_and_a_v2_stamped_record_both_migrate_to_current_in_one_pass` + `::every_version_below_current_ships_a_snapshot_and_nothing_above_it_does`.
- **T3-3** · pinned-by: `completion_record_audit_vocabulary::every_declared_severity_lands_and_a_non_member_is_still_refused` — the widened enum admits HIGH/MEDIUM/LOW while a non-member is still refused, so the widening did not become an open string; `::a_committed_v1_record_migrates_with_the_stamp_as_its_only_byte_delta` asserts the `EnumWidened` + `AddedItemSlot` pair is a byte no-op; `::the_detail_slot_takes_prose_and_never_gates_when_left_empty` and `::the_pinned_schema_projection_carries_the_optional_detail_slot` hold the prose half. Named precisely because the row is not: the *`evidence` → prose* change in this row's *change* cell did NOT ship as a reshape of `evidence`; what shipped is a NEW optional `detail` slot, which is the settled shape (D10) and the reason the bump needed no `RemovedField`.
- **T3-4** · UNPINNED: `deferral-ledger` 2→3 did not ship — the manifest still reads `schema-version: 2`, and this wave took no `deferral-ledger` bump. Owed; increment 12 / T3 is the task that carries the wave's new deferrals into the ledger, and this row is one of the entries it is charged with.
- **T3-5** · UNPINNED: `vision` 1→2 did not ship — the manifest still reads `schema-version: 1`. The row's own note records that its transform arm (`FixedSlotToRepeatable`) was driven green for the first time during the baseline, so the capability is proven and the bump is scheduling, not capability; owed to the same ledger entry as T3-4.
- **T3-6** · UNPINNED: `idea` 1→2 did not ship — the manifest still reads `schema-version: 1`, and the row's own scope (12 of 38 instances affected) is a corpus cost that belongs with the port, which the wave's settled order places AFTER 1.0.0. Owed to the same ledger entry as T3-4.
- **T3-7** · pinned-by: `add_item_slug::two_titles_that_slug_alike_both_mint_when_the_second_supplies_a_slug` — the row states *no schema change* and names identity (T1-6) as the blocker, so its disposition IS T1-6's: 890 titles collapsing to 884 slugs is now expressible, because `--slug` decouples an item's id from its title at the fourth mint door, driven on the emitted address. No `decisions-log` schema version moved, which is the row's own prediction holding.
- **T3-8** · pinned-by: `planning_record_schema::doc_schema_json_projects_one_required_slot_per_gate` + `::the_composed_schema_projection_carries_every_gate_cell_verbatim` + `::the_new_doctype_owes_no_snapshot_and_blocks_no_migration` — the new doctype is free at the freeze (one manifest entry at schema-version 1, no `schema-snapshots/` entry owed because there is no prior shape to diff), and the gate cell text moved VERBATIM into each `hint:` rather than being paraphrased. The forcing function the doctype exists for is `planning_gate_forcing::every_gate_is_a_fence_finalize_blocks_on_each_one_held_out` (the gate axis is DERIVED from the pinned `doc schema planning-record --format json` projection — every slot section it advertises — rather than from the number 14, and each member is held out in turn while the rest are filled: `finalize` blocks non-zero on `schema-conformance.required-slot-present`, the block names the unfilled gate at its own address, and HEAD does not move) + `::filling_the_held_out_gate_lets_the_same_finalize_land_the_record` (the inverse, so the fence is not a permanent block) + `::the_planning_workflow_creates_the_gate_record_through_its_own_composed_line`; the single-home rule is `planning_gate_home::the_single_home_lists_exactly_the_shipped_gates` + `::no_paragraph_outside_the_home_restates_the_gate_set`, which is what keeps the 14-vs-13 disagreement the row names from re-forming.

### Tier 4

- **T4-1** · pinned-by: `read_surface_naming::every_soliciting_step_disposes_both_read_surfaces` — the owe-set is the M48 read-back fence's OWN (`cli::pack::solicits_managed_doc_write` over `cli::pack::doc_write_command_ids`), so the sweep and the fence cannot disagree about which steps are in, and every member is disposed against each of the two surfaces on a STRUCTURAL signal the step already renders (`{{schema:<T>}}` renders the shape inline, so the step is out; an unresolved `<slug>` in the read-back address is what makes `doc list` owed) rather than a hand-maintained exclusion list; `::every_composed_workflow_names_the_read_surfaces_its_steps_owe` asserts it on the COMPOSED bytes through the real binary, and `::the_derivation_is_not_vacuous` fences the owe-set against emptying out.
- **T4-2** · pinned-by: `read_surface_naming::the_installed_guide_artifact_names_both_read_surfaces` — it reads the INSTALLED `.claude/skills/jigc/SKILL.md` out of a built fixture corpus and asserts it names `jigc doc schema` and `jigc doc list`, which is the adopter's only version-matched copy of the guides, so a read surface it omits is one an adopter meets only by accident.
- **T4-3** · pinned-by: `author_write_contract::no_shipped_step_states_the_retired_doubling_falsehood` — no shipped step in either pack still states the retired *"would double"* claim, swept over all steps rather than the five that were reported; `::the_pack_load_fence_demands_exactly_the_clause` asserts SET-EQUALITY between `cli::pack::COPY_IN_APPEND_TOKENS` and the clause the binary produces, so the fence can no longer mandate a statement the binary contradicts; `::the_composed_batch_author_note_states_the_collision_reject` asserts the true clause on the composed bytes; and `::a_colliding_payload_item_rejects_the_whole_author_over_a_committed_singleton` drives the reject through the real binary, so the words the fence demands are the behaviour the binary has.
- **T4-4** · pinned-by: `doctype_authoring_fences::every_pack_load_fence_carries_a_checklist_row` — the fence set is DERIVED from `crates/cli/src/pack.rs` on what the members ARE (a top-level `anyhow::Result<()>` function reachable from `make_pack` — pass-or-refuse, carrying no value a caller could use), never on a name pattern, with set-equality in both directions so a fence with no row and a row naming no fence both redden; `::every_fence_row_states_its_condition_and_its_discharge` asserts each row states when it FIRES and how to satisfy it, because most are conditional and six unconditional obligations would be their own law-1 lie. The *inert for adopter packs* half is where the fences now reach a listed pack and where they deliberately do not: `suppression_fence::a_stripped_methodology_workflow_blocks_through_the_listed_pack_path` and `catalog_shape_fence::a_mutated_methodology_workflow_blocks_through_the_listed_pack_path` assert they fire through it, while `suppression_fence::a_manifest_less_pack_is_unchecked` pins the manifest-less carve-out as deliberate rather than accidental. `catalog_shape_fence::out_of_scope_workflows_are_outside_the_fence` is deliberately NOT cited for that carve-out — read, it drops `usage:` from a non-selectable and a non-work workflow and asserts the load still succeeds, which is the fence's SCOPE boundary and says nothing about a manifest; the near-miss is recorded rather than borrowed.
- **T4-5** · pinned-by: `catalog_shape_fence::a_dropped_usage_on_a_selectable_work_workflow_blocks` — *"a workflow/doctype can drop off `describe` at exit 0"*: a selectable work workflow that drops its catalog metadata is refused at PACK LOAD rather than silently disappearing from the menu, and `::a_multi_line_when_blocks` / `::a_period_terminated_when_blocks` / `::an_over_cap_when_blocks` hold the shape asserts around it, with `::both_shipped_packs_pass_the_catalog_shape_asserts` proving the fence is satisfiable by what ships.
- **T4-6** · pinned-by: `describe::describe_commands_carry_the_union_of_every_declaring_pack` — it asserts SET-EQUALITY between `jigc describe --commands --format json` and the union of both packs' declared catalog entries, each attributed to the pack that declares it, with a DERIVED non-vacuity first (the two catalogs must declare at least one divergent id, or the assert could be satisfied by the precedence winner alone), and then asserts the prose menu an agent reads carries the same union while staying non-contractual prose; `::describe_names_doc_show_on_the_composite_path` holds the composite case.
- **T4-7** · pinned-by: `clap_error_kind_axis::the_table_carries_one_row_per_error_kind` + `::every_producible_kind_is_reached_by_driving_its_probe` — `ErrorKind` is `#[non_exhaustive]` and exposes no `all()`, so the completeness proof is REACHABILITY BY DRIVING rather than exhaustiveness: every row is reached by an argv that actually produces that kind, and `::the_not_producible_kinds_are_fenced_over_the_clap_tree` disposes the rest against the real tree; `::the_table_is_pinned_to_the_clap_it_was_derived_against` reddens when the dependency moves under the table, `::every_producible_kind_holds_with_the_invocation_log_on` covers the log half that still panicked, and `::no_production_code_reads_the_process_argv_as_strings` is the class the second fix generalized.
- **T4-8** · pinned-by: `unknown_doctype_axis::every_doctype_door_answers_an_unknown_doctype_the_same_way` — the door set is DERIVED from the CLI verb tree (every leaf taking a doctype id) rather than from the reported seven, every cell exits non-zero carrying a code and a route that RUNS VERBATIM at exit 0, and no cell's stderr contains a `{:?}` rendering of any engine enum, which is where the `PackResourceKind` Debug leak died; `::the_axis_drives_every_registered_doctype_door` is the enumeration fence and `::every_address_headed_door_answers_with_the_store_code` keeps the deliberate `create.unknown-doctype` / `store.unknown-type` split from being collapsed by accident.
- **T4-9** · pinned-by: `not_in_repo_axis::every_door_outside_a_repo_answers_with_the_one_text_and_route` — driven from a non-repo directory: each door emits the one precondition text with a route a user can act on (a `Human` route, since there is no argv jigc can hand a user standing outside a repo); `::only_the_shared_constructor_composes_the_precondition_text` is the source fence that keeps the three texts from re-forming at the nineteen sites; `::arms_cover_every_leaf_verb` bijects `ARMS` against `cli::cli::VERB_KINDS` — exactly one arm per leaf verb, no phantom arm, equal lengths — so the door set is the verb registry rather than the *"11+ sites"* the baseline counted.
- **T4-10** · pinned-by: `flow10_acceptance::the_three_emitted_milestone_run_lines_carry_the_resolved_id_and_run_verbatim` — all three `{ agent: milestone_id }` refs are LIFTED FROM THE COMPOSED BYTES and executed against the built binary, so the assertion is on the emitted `Run:` lines rather than on a reconstruction of them; `::the_off_verb_compose_keeps_the_milestone_id_marker_and_stays_clean` pins the marker's remaining home (three markers, zero `Spawn:` lines, no finding), which is what keeps the fix from becoming a fabricated empty id.
- **T4-11** · pinned-by: `milestone_spawn_recorded_workflow::each_spawn_span_names_and_runs_the_sub_tasks_recorded_workflow` — each emitted `Spawn:` span names the sub-task's own recorded workflow rather than the hardcoded `sub-task`, and the span is RUN, so the fan-out line is executable rather than plausible; `::an_empty_milestone_emits_no_spawn_line` is the omitting context.
- **T4-12** · pinned-by: `finalize_render_io_absent::a_serial_task_render_tells_an_absent_commit_doc_from_a_read_fault` + `::a_sub_task_render_tells_a_never_entered_sub_task_from_a_read_fault` — the `NotFound` case is discriminated from a genuine disk fault at BOTH loci, so a never-composed sub-task stops being reported as `finalize.render-io`, and the two arms are the axis (serial task, sub-task) rather than the one repro.
- **T4-13** · pinned-by: `record_stale_reasons::the_record_no_longer_states_the_reasons_head_falsifies` + `::each_replacement_is_stated_exactly_once` + `::no_line_still_calls_a_settled_question_open` + `::the_nesting_bound_is_never_restated_as_a_limit` + `::the_multi_slot_claim_is_derived_from_the_shipped_schemas` + `::the_historical_exclusions_still_hold_their_premise` — each falsified byte is named with the string it carried and asserted absent, and each replacement asserted EXACTLY ONCE, so a correction restated in two homes reddens (this repo's cross-reference rule, fenced). The sibling record fences carrying the rest of the count: `finalize_family_registry::the_registry_equals_the_production_producer_set` + `::no_live_doc_states_a_numeral_for_the_family`, `exit_flip_count_record::each_replacement_is_stated_exactly_once` + `::no_shipped_exit_code_row_or_task_source_counts_the_flips`, `record_item_slot_kind::*` and `record_nesting_cap::*`. Stated plainly: the NUMBER *20* is deliberately not fenced anywhere — `validation.md`'s settled rule is *point at the enumeration, do not re-count*, and pinning a count is the defect these fences remove.
- **T4-14** · pinned-by: `illustrative_disclaimer_scope::every_doc_level_illustrative_disclaimer_carries_the_scoping_reference` — every `design/*.md` carrying a doc-level illustrative disclaimer also carries the scoping reference, with the set derived on the CONCEPT rather than on a phrase (`structural-grammar.md`'s *"Notation **below** is illustrative"* is a member a grep for the common phrasing misses, and the local caveats on specific examples are a different object and stay out); `::the_scoping_rule_is_stated_in_exactly_one_home` keeps the wording from being restated fourteen times, which is what makes a razor's leg 1 stop refusing by which doc an item happens to cite.

### The residues named in *Refuted / de-scoped by driving*

- **T5-1** · pinned-by: `mint_doors::mint_doors_enumerates_every_production_working_area_mint` — the carryover refutation's residue (*no `MINT_DOORS` enumeration fence*, three hand-written snapshot sites): the door set is now derived from the production mint sites and asserted equal to the registry, `::the_site_detector_separates_a_call_from_a_definition` keeps the derivation from counting its own definition, and `::every_mint_door_is_driven_over_a_pre_staged_foreign_file` drives each member over the state the gate exists for, so a fourth mint door cannot ship without a snapshot.
- **T5-2** · pinned-by: `milestone_provision_handoff::execute_speaks_only_in_the_partial_state` — residue C7 (*execute narrates a half-provisioned set as ready*): `milestone execute` checks the provisioned set on disk before narrating it, and speaks ONLY when the state is partial; `::the_partial_advisory_leaves_the_json_document_alone` asserts the advisory rides the declared channel and does not corrupt the machine document, which is the stream-discipline half.
- **T5-3** · pinned-by: `milestone_provision_handoff::a_provision_that_fails_mid_walk_blocks_with_a_code_and_a_route` — residue C8 (*code-less/route-less anyhow at `milestone provision`*): the mid-walk failure now blocks with a code AND a route, which is the `finding_to_err` promotion increment 10 chose over inventing a third precedent.
