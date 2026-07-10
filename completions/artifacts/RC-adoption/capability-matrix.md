# Doctype × capability completeness matrix (1.0.0-rc.3 audit, 2026-07-10)

Commissioned off the adoption trial: the project kept finding per-doctype holes one trial at a time. Read-only audit; the capability axis is **derived from code registration surfaces**, and all 15 doctypes were exercised empirically in a throwaway repo (create → author → finalize through the real binary). Planning input for the rc.4 wave (systemic gaps 1–6) and the doctype-completeness milestone (the checklist).

## The capability axis (from code)

| # | Capability | Registration/dispatch surface |
|---|---|---|
| C1 | Create route (create-gate) | workflow `allows-create` → `state::create_gated`; reject in `crates/cli/src/doc.rs` |
| C2 | Create-skeleton completeness | seed = `on_create_doc_fields` `doc.rs:856` (only `default:`, date+`set:on-create`, `set:schema-version`); render emits only seeded fields `write.rs:139-145`; finalize requires every no-default/no-set/non-optional/non-pack field (`is_author_required` `validate.rs:1599-1620`). **Invisible-required = required − seeded** |
| C3 | Author step names full payload | `crates/cli/pack/steps/author-*.yaml`, `packs/methodology/steps/author-*.yaml` |
| C4 | `doc author --from` batch | `crates/cli/src/author.rs:243` (whole-doc YAML incl. nested items) |
| C5 | `migrate-<doctype>` workflow | pure name convention `migrate.rs:64-66`; hard error if absent |
| C6 | Ingest adoptability | `engine/src/ingest.rs:127-174` — location-prefix or placement-exact-path; transient never |
| C7 | `doc show` plain + JSON | `engine/src/store.rs:101-195`; JSON shaping `doc.rs:1155-1418` |
| C8 | `describe` entry | `engine/src/introspect.rs:95-183`; skip iff neither description nor usage |
| C9 | `jigc rename` | `rename.rs:522-533`; transient refused; placement reslug trips collision guard :152-154 (retitle-only survives) |
| C10 | Relocation | docs-root detect+route+move `config.rs:204-283` + `orphan.rs:178-203` (location doctypes); `jigc relocate` freeze-exempt `relocate.rs:136-157` |
| C11 | Version gate + corpus migration | stamp iff persisted ∧ manifest-listed `pack.rs:55-60`; freeze assert `pack.rs:178-206`; `migrate_corpus.rs:148-150` skips unlisted |
| C12 | Validation-probe participation | refs blocking `validate.rs:266-269`; code-anchors `doc-code`; `owned-location` gate `validate.rs:1125-1229`; inverse-card store-advisory `validate.rs:461-476` |
| C13 | Empty-required-repeatable exposure | **none exists**: `check_repeatable` iterates parsed items only, `validate.rs:1288-1295` |
| C14 | Finalize sink/promotion | placement / location / transient-skip `engine/src/finalize.rs:601-665`; commit msg render :206-213; milestone-record machine arms `engine/src/milestone.rs:674,731,776` |
| C15 | Item verbs | add/remove/set-field/set-slot only; **no retitle/re-id/reorder** — `write.rs:2250-2253` references a `reorder` verb never built |

## The matrix

OK = works · GAP = hole · N-A = by design · superscripts = footnotes

| doctype | C1 | C2 skeleton | C3 author step | C4 | C5 migrate | C6 | C7 show | C8 | C9 rename | C10 | C11 version | C12 probes | C13 empty-rep | C14 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| commit | N-A¹ | OK² | OK | N-A | N-A | N-A | N-A³ | OK | N-A | N-A | OK⁴ | ref | opt | OK (git msg) |
| adr | OK | OK | **GAP⁵** | OK | OK | OK | OK | OK | OK | OK | OK v2 | ref+anchor | N-A | OK |
| spec | OK | OK | OK | OK | OK | OK | OK | OK | OK | OK | OK v1 | ref+anchor+inv | GAP⁶ | OK |
| prd | OK | OK | OK | OK | OK | OK | OK | OK | OK | OK | OK v1 | inv-target⁷ | GAP (silent) | OK |
| arch-doc | OK | OK | OK | OK | OK | OK | OK | OK | OK | OK | OK v1 | ref+anchor+tns | GAP (silent) | OK |
| changelog | OK | OK | OK | OK | OK | OK | **GAP⁸** | OK | GAP⁹ | N-A | OK v2 | — | GAP¹⁰ | OK |
| vision | OK | OK | OK | OK | **GAP¹¹** | OK | OK | OK | GAP⁹ | OK | **GAP¹²** | ref | N-A | OK |
| research | OK | OK | OK | OK | **GAP¹¹** | OK | OK | OK | OK | OK | **GAP¹²** | ref-target | N-A | OK |
| idea | OK | **GAP¹³** | OK | OK | **GAP¹¹** | OK | OK | OK | OK | OK | **GAP¹²** | — | N-A | OK |
| roadmap | OK | OK¹⁴ | OK | OK | **GAP¹¹** | OK | OK | OK | GAP⁹ | OK | **GAP¹²** | — | GAP (silent) | OK |
| decisions-log | OK | OK¹⁴ | OK | OK | **GAP¹¹** | OK | OK | OK | GAP⁹ | OK | **GAP¹²** | — | GAP¹⁰ | OK |
| deferral-ledger | OK | OK¹⁴ | OK | OK | **GAP¹¹** | OK | OK | OK | GAP⁹ | OK | **GAP¹²** | — | GAP¹⁰ | OK |
| completion-record | OK | **GAP¹³** (×2) | OK | OK | **GAP¹¹** | OK | OK | OK | OK | OK | **GAP¹²** | owned-loc | GAP (silent) | OK + gate |
| dogfood-record | OK | **GAP¹³** (×14) | OK | OK | **GAP¹¹** | OK | OK | OK | OK | OK | **GAP¹²** | owned-loc | N-A | OK + gate |
| milestone-record | N-A¹⁵ | N-A | N-A | N-A | N-A | OK¹⁶ | OK | OK | GAP¹⁷ | OK¹⁶ | **GAP¹²** | — | ∅ valid | OK |

¹ workflow-provisioned at task mint (`start.rs:239`), never `doc create`d. ² via `fillable_form` (`start.rs:225-243`): every header field pre-stamped as an empty `key:` line — the only mint path with no invisible fields. ³ `store.transient-type` block. ⁴ manifest-listed but stamp never injected (transient excluded). ⁵ non-migration ADR authoring inline in `implement.yaml:29-33`, does not enumerate required `context`/`decision`/`consequences`; no standalone `author-adr.yaml`. ⁶ zero-criteria spec finalizes clean; blocks only downstream at `milestone add-from-spec` (`milestone.no-criteria`). ⁷ `inverse-card "1..*"` → every spec-less PRD carries a store advisory. ⁸ nested repeatables unreadable in JSON: whole-doc omits release→`changes` (`doc.rs:1322`); nested fragment degrades to the enclosing release (`doc.rs:1277-1284`); plain item slice drops fields+nested groups; write/show address-grammar asymmetry (write `#releases/<id>/changes/<gid>`, show wants `#releases/<id>/<gid>`, write-style errors `store.no-such-item`). ⁹ placement-singleton reslug blocked only by the collision guard with a misleading message ("a different doc already exists at CHANGELOG.md" — it's the *same* file, `rename.rs:152-154`); retitle-only works. ¹⁰ verified: empty CHANGELOG / decisions-log / deferral-ledger all promote+commit with zero items. ¹¹ `jigc migrate --as <ty>` hard-errors — no migrate-*.yaml for any methodology doctype. ¹² no schema-version stamp, no freeze assert (absent manifest → `Ok(())`, `pack.rs:186-188`), migrate-corpus skips, store conformance findings un-routed (`validate.rs:706-708`) → **a methodology schema-shape change strands committed instances silently**. ¹³ invisible author-required fields, verified: `idea.trigger`; completion-record `verdict`+`owner-artifact`; dogfood-record all 14 (skeleton = empty `---\n---`). Recovery: `set-field` inserts an absent front-matter line (`write.rs:4805` → :2538). ¹⁴ singleton ignores `--title`; H1 mints as lowercase type id (`# roadmap`, committed) — `vision`/`changelog` got `display-title`, the other three placement singletons didn't; post-hoc fix only via retitle-rename. ¹⁵ gate-blocked in every workflow; machine-materialized by the milestone verbs — correct by design. ¹⁶ verified incl. docs-root move mid-active-milestone (cache re-derives). ¹⁷ renameable as an ordinary location doctype, but its slug **is** the work-unit id joining `.jigc/milestones/<id>`; only the mid-fan-out marker guards it (`rename.rs:140-145`) — a between-milestones rename desyncs record↔work-unit identity.

## Ranked gap list

**Systemic:**

1. **Invisible author-required header fields at mint** (C2) — hits idea/completion-record/dogfood-record today and every future doctype with a plain required header field. Fix precedent in-tree: `fillable_form` + `insert_front_matter_field`.
2. **No migrate-* for the 9 methodology doctypes** (C5) — zero code to close; workflow name is the registration. Foreign vision/roadmap/decisions docs are exactly what adoption meets.
3. **Methodology schemas unversioned; shape changes strand corpora silently** (C11) — deliberate M33 bound, but 9 freeze-exempt doctypes now have committed corpora in the wild (project-alpha-2.0: 33 instances; project-kb; this repo). One-way-door lens applies — every 1.0 adopter deepens the debt (recoverable via the M34 v0→v1 stamp-migration precedent, but the detector/transform machinery doesn't exist for them).
4. **Empty required repeatables pass silently everywhere** (C13) — no min-items concept; only spec's emptiness ever surfaces, downstream. A `min-items`/non-empty schema knob (advisory) closes the class.
5. **No item retitle/re-id/reorder** (C15) — anchors mint once; remove+add re-mints the id and orphans inbound addresses; docs have a full rename+repoint transaction, items have nothing.
6. **Nested-repeatable read surface** (C7, changelog today, structural for any 2-level doctype) — `doc author` writes what `doc show` can't read back; breaks the M39 "reads and writes" symmetry for one shipped doctype; JSON contract adjacency (pinned at 1.0) makes *whether nested joins the contract* itself a pre-1.0 call.

**One-doctype / polish:** 7. adr author step under-specified (⁵). 8. misleading placement-reslug error (⁹). 9. `# roadmap`/`# decisions-log`/`# deferral-ledger` lowercase H1s — missing `display-title` (¹⁴). 10. milestone-record rename unguarded between milestones (¹⁷). 11. prd's perpetual fresh-repo advisory (⁷ — by design, noise).

## Draft "adding a new doctype" checklist

Derived from what the complete doctypes (spec, arch-doc, vision, dogfood-record) actually ship. **Graduated 2026-07-10 → [implementation/doctype-authoring.md](../../../implementation/doctype-authoring.md) — that is the living copy (maintained, rc.4-annotated); this section stays as the frozen audit snapshot.**

**Schema file** (`<pack>/schemas/<ty>.yaml`):
- [ ] `type`, `description:` **and** `usage:` (describe weaves both; absent both → dropped from the menu).
- [ ] Exactly one home: `location: <dir>/` (docs-root applies) · `placement: {file: …}` + `singleton: true` (literal path, bypasses docs-root) · neither = transient (then you must own a finalize sink — the commit precedent).
- [ ] `id-from:`; `display-title:` for any singleton whose H1 must not read as the lowercase type id (gap 9's lesson).
- [ ] **Single-word section ids** (multi-word title-case reparse defect — every schema routes around it except changelog's deliberately-fixed one).
- [ ] For every header field, decide the mint story: `default:` or `set:` (date/on-create, schema-version, on-transition) **or accept it is invisible in the skeleton and blocking at finalize** (gap 1) — if unavoidable, the author step MUST name it and instruct the `set-field`.
- [ ] Repeatables: per-item `id-from`; zero items validates clean (gap 4) — if ≥1 item is semantically required, nothing enforces it today; note it in the author step.
- [ ] Refs: only to doctypes inside the composed schema universe; forward card `0..*`/`0..1` (never required — create-deadlock guard, the spec `derived-from` lesson); declare `inverse:`; inverse-card is store-advisory only. Cross-pack targets → plain `string` (the `trigger`/`task-id` precedent).
- [ ] Pack field types (`code-anchor`) need a `config/field-types.yaml` entry binding the adjudicator.
- [ ] `owned-location` fields: tracked path under `completions/artifacts/<milestone>/`; author-required (gap 1 applies).

**Freeze/versioning:**
- [ ] Frozen pack: manifest entry (`type` + `schema-version` + `schema-hash` via `engine::manifest::schema_hash`) — pack-load asserts set-equality; add `schema-snapshots/<ty>.v<k>.yaml`; any later shape/home change = version bump + corpus migration.
- [ ] Methodology/freeze-exempt: no stamp, no gate — record the exposure (gap 3); home changes via `jigc relocate <ty> --from <prior>`.

**Authoring surface:**
- [ ] `steps/author-<ty>.yaml` enumerating **every** author-required field and slot (dogfood-record is the gold standard; adr's inline step the anti-pattern) — plus a `{{cli.create-<ty>}}` macro in `config/commands.yaml` if steps reference it.
- [ ] A driving workflow with `allows-create: [{type: <ty>, as: <role>}]` — `doc create` is gate-blocked otherwise; set `when:`/`selectable:` for router reachability.
- [ ] Foreign instances plausible in the wild? Ship `migrate-<ty>.yaml` + `author-migration-<ty>.yaml` (a full `doc author --from` payload) — the workflow *name* is the entire registration (gap 2).
- [ ] Machine-maintained doctype: all leaves `set:`-bearing, **no** author step, engine write arms + CLI drivers (milestone-record precedent); compound field projections need a `doc.rs` JSON special-case (the `base` precedent).

**Free-of-charge (no registration, verify only):** describe, doc show (unless nested repeatables — gap 6 — or compound fields), rename (location doctypes; placement = retitle-only), ingest, docs-root relocation, validate conformance.

**Tests:** engine render/round-trip golden (byte-stability); a `crates/cli/tests/flow*.rs` acceptance through the real binary; ingest-adoptability for a placement home; and — per this audit — a **mint→finalize round-trip** that would have caught gap 1.
