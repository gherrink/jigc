# Adding a new doctype — the registration checklist

**Consult this whenever a doctype is added or reshaped; update it whenever a registration surface changes.** Derived from the 2026-07-10 doctype × capability audit ([completions/artifacts/RC-adoption/capability-matrix.md](../completions/artifacts/RC-adoption/capability-matrix.md) — the full matrix + file:line evidence for every surface): the trials kept finding per-doctype holes one at a time (`idea`'s invisible required `trigger`, no migrate path for any methodology doctype, lowercase singleton H1s) because a doctype's obligations were never listed in one place. This is that list.

Items marked **⚠ rc.4** describe today's behavior that the rc.4 wave changes ([decisions-pending.md](decisions-pending.md) → the rc.4 wave) — re-verify this doc after M40 ships.

## Schema file (`<pack>/schemas/<ty>.yaml`)

- [ ] `type`, plus **both** `description:` and `usage:` — `describe` weaves both; a schema with neither is dropped from the menu.
- [ ] **Exactly one home:** `location: <dir>/` (docs-root applies) · `placement: {file: …}` + `singleton: true` (literal path, bypasses docs-root) · neither = transient — then you must own a finalize sink (the `commit` precedent).
- [ ] `id-from:`; and `display-title:` for any singleton whose H1 must not read as the lowercase type id (`# roadmap` is the shipped counterexample — ⚠ rc.4 fixes the three affected singletons, but the obligation stays yours for new ones).
- [ ] **Single-word section ids** — the multi-word title-case reparse defect; every schema routes around it except changelog's deliberately-fixed one.
- [ ] **For every header field, decide the mint story:** give it `default:` or `set:` (date/on-create, schema-version, on-transition) — or accept that it is **invisible in the create skeleton and blocking at finalize** (the `idea.trigger` failure class; worst case `dogfood-record`, 14 invisible fields). If unavoidable, the author step MUST name the field and instruct the `set-field`. ⚠ rc.4 (settled — [DECISIONS.md](../DECISIONS.md) → 2026-07-10 M40 Settle): **both halves ship** — the generalized fillable skeleton (every author-required header field pre-stamped as an empty `key:` line, via the shared engine `is_author_required` predicate) **and** the `jigc doc schema` read surface; the decide-the-mint-story discipline stays.
- [ ] **Repeatables:** per-item `id-from`; know that **zero items validates clean** — nothing enforces "at least one" anywhere. If ≥1 item is semantically required, say so in the author step. ⚠ rc.4 adds zero-items advisories (adopt-time + store scope), not a gate.
- [ ] **Refs:** only to doctypes inside the composed schema universe; forward cardinality `0..*`/`0..1` — never required (create-deadlock guard; the spec `derived-from` lesson); declare `inverse:`; inverse-cardinality is a store advisory only. Cross-pack targets → a plain `string` field (the `trigger`/`task-id` precedent).
- [ ] Pack field types (`code-anchor`) need a `config/field-types.yaml` entry binding the adjudicator.
- [ ] `owned-location` fields: the value must be a tracked path under `completions/artifacts/<milestone>/`, and the field is author-required (the mint-story item above applies).

## Freeze / versioning

- [ ] **Frozen (dev) pack:** add the manifest entry (`type` + `schema-version` + `schema-hash` via `engine::manifest::schema_hash`) — pack-load asserts set-equality, so an unlisted shipped schema fails loudly; add `schema-snapshots/<ty>.v<k>.yaml`. Any later shape **or home** change = version bump + versioned corpus migration (location is inside the hash — the M38 changelog precedent).
- [ ] **Methodology pack:** today no stamp, no freeze assert, no migrate-corpus arm — **a shape change strands committed instances silently.** ⚠ rc.4 (the versioning fork is **settled — build**, [DECISIONS.md](../DECISIONS.md) → 2026-07-10 M40 Settle): the methodology pack ships its **own** `schema-manifest.yaml` + `schema-snapshots/`, its persisted doctypes stamp-injected + freeze-asserted + migrate-corpus-owned ([corpus-migration.md](../design/corpus-migration.md) → The freeze-exempt sibling). A new methodology doctype then needs a manifest entry + snapshot **exactly like the frozen pack** (the bullet above applies verbatim); home changes version-gate, and `jigc relocate <ty> --from <prior>` covers only genuinely manifest-less doctypes.

## Authoring surface

- [ ] `steps/author-<ty>.yaml` enumerating **every** author-required field and slot (`dogfood-record`'s step is the gold standard; adr's inline step is the anti-pattern — ⚠ rc.4 fixes adr). Add a `{{cli.create-<ty>}}` macro in `config/commands.yaml` if steps reference it.
- [ ] A driving workflow with `allows-create: [{type: <ty>, as: <role>}]` — `doc create` is gate-blocked otherwise; set `when:`/`selectable:` for router reachability.
- [ ] **Foreign instances plausible in the wild? Ship `migrate-<ty>.yaml` + `author-migration-<ty>.yaml`** (a full `doc author --from` payload template). The workflow *name* is the entire registration (`migrate.rs` resolves `migrate-<doctype>` by convention) — there is no other switch to flip, and no error until a user hits the missing workflow.
- [ ] **Migration templates are mechanical decision trees** (⚠ rc.4): the template carries every decision pre-made — the **historic-date TRANSCRIBE/OMIT rule** (migration mode suppresses `set: on-create`, so an untranscribed date silently goes dateless) plus any per-doctype mapping decisions (the deferral-ledger `[D,I]` kind mapping, vision's omit-`grounded-in`) — so the agent **transcribes, never derives** ([DECISIONS.md](../DECISIONS.md) → 2026-07-10 M40 Settle, decision 5).
- [ ] **Machine-maintained doctype** (the `milestone-record` precedent): all leaves `set:`-bearing, **no** author step, engine write arms + CLI drivers; a compound field projection needs a `doc.rs` JSON special-case (the `base` `{sha,short}` precedent).

## Free of charge — no registration, but verify

`describe` · `doc show` (**unless** nested repeatables — write-only through JSON today, the changelog gap; ⚠ rc.4 pins nested groups into the JSON item object — or compound fields) · `rename` (location doctypes; placement singletons are retitle-only — the rule's design home is now [write-commands.md](../design/write-commands.md) → Placement singletons; ⚠ rc.4 adds the milestone-record committed-`status` guard exception) · `ingest` adoptability · docs-root relocation · `validate` conformance. ⚠ rc.4: **`jigc doc schema`** joins this verify list.

## Tests (what the complete doctypes ship)

- [ ] Engine render/round-trip golden (byte-stability, the `write.rs` test style).
- [ ] A `crates/cli/tests/flow*.rs` acceptance driving the **real binary** through the doctype's workflow (the M38 lesson: core-level tests below the shipped verb's assembly gate mask integration defects).
- [ ] Ingest-adoptability for a placement home.
- [ ] A **mint → author → finalize round-trip** — the test that would have caught the invisible-required-field class before any trial did.
