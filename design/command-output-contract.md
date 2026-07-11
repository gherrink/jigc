# The command-output contract — the pinned `--format json` of composed output, write-acks, and findings

**M41 design of record.** Where the *machine-consumed output of the write/compose verbs* is pinned. This is the write/compose-side sibling of [doc-read-surface.md](doc-read-surface.md): that doc pins the **read-back** of committed content (`jigc doc show`/`schema`); this one pins what a machine driver reads back from the **acting** verbs — the composed workflow a `jigc start` emits, the acknowledgement a `jigc doc set-field`/`author` emits, and the findings envelope a block or advisory carries. `jigc` is a *context compiler for coding agents*, and in both adoption trials the agent built a driver over `--format json`; this surface is the contract that driver stands on.

Reads right after [doc-read-surface.md](doc-read-surface.md) (its read-side sibling) and after [write-commands.md](write-commands.md) (whose acks it pins) and [workflow-dialect.md](workflow-dialect.md) (whose composed output it pins). Homed as its own part-doc — **not** folded into `doc-read-surface.md` (whose charter is explicitly the *committed-doc read*, [doc-read-surface.md](doc-read-surface.md) → charter — it disowns the compose/write outputs) and **not** left implicit in `workflow-dialect.md` (which frames composed output as an *ephemeral view*, true of the view-as-workflow-state but silent on its *machine projection*).

## Why this is a contract (and why now)

The read side pinned its machine contract at 1.0 with a stated rationale ([doc-read-surface.md](doc-read-surface.md) → "a one-way door, pinned now rather than left to the first implementation"). The **write/compose** side is consumed by the *same driver* and had no such pin — composed output was `{"text": …}` (prose-in-a-box, no task id), and write-acks were ad-hoc `serde_json::json!` with an opaque `address` string. Pinning one side and not the other is an unprincipled split at the exact freeze milestone.

The load-bearing insight (M41 Fork 1, [DECISIONS.md](../DECISIONS.md) → 2026-07-11 M41 planning Settle): **the one-way door is the *posture*, not the field count.** After the 1.0 pin a declared contract evolves "only by an explicitly versioned extension"; an *undeclared* output calcifies into a *de-facto* contract with no evolution rule, so the first shipped shape silently becomes the 1.0 contract. Declaring the surface a contract with an evolution posture is the strictly-one-way, cheap-now move; the field breadth is the part that could stage. The human's call was **full pin** — pin the whole driver contract this wave, in the still-open pre-1.0 additive window, rather than pay the versioned-extension tax post-pin.

## The three surfaces this pins

### 1 · Composed output — `jigc start` / `jigc workflow` `--format json`

The composed workflow a work-minting or plain compose call emits. Today `ComposedWorkflow` carries only `text` ([compose.rs](../crates/engine/src/compose.rs) → `ComposedWorkflow`); the minted task id is in hand at compose time but never surfaced structurally, so a driver had to scrape it from prose (the rc.4 trial's `--task (\S+)` backtick-scrape blew up shell substitution).

```json
{ "task": "<task-id>" | null, "text": "<composed workflow prose>" }
```

- **`task`** — the minted task id for a work-minting workflow (`creates-task: true`); **`null`** for a non-minting compose (orient, a `creates-task: false` router selection). The id is the handle every subsequent call requires; surfacing it structurally retires the scrape.
- **`text`** — the composed workflow prose, unchanged (the agent-facing instruction stream).

### 2 · Write-acks — `jigc doc set-field` / `set-slot` / `add-item` / `remove-item` / `retitle-item` / `create` / `author`

The acknowledgement a successful managed-write emits ([render.rs](../crates/cli/src/render.rs) → `DocAck`). Today the write target is a single opaque `address` string (`spec:my-spec#criteria/item-3/title`), forcing every driver to re-parse the write-grammar address it already handed in. **And the envelope has a coverage gap:** only `set-field`/`set-slot`/`remove-item`/`retitle-item` emit a `DocAck` today — `create`, `add-item`, and `author` print a *bare address string* on success, outside the envelope entirely. The contract **brings every write verb into the envelope**, **decomposes the address into structured components**, and **carries findings-as-data**:

```json
{ "op": "set-field",
  "target": { "doctype": "spec", "slug": "my-spec",
              "section": "criteria", "item": "item-3", "leaf": "title" },
  "value": "<the written value, shaped as in doc show's fields>",
  "findings": [ … ]  // the findings envelope below; [] on a clean write
}
```

- **`op`** — the write verb (`set-field`/`set-slot`/`add-item`/`remove-item`/`retitle-item`/`create`/`author`).
- **`target`** — the decomposed write address: `doctype` + `slug` always; `section`/`item`/`leaf` present exactly when the address reaches that depth (a whole-doc `author`/`create` carries only `doctype`+`slug`; a section-item write carries `section`+`item`; a leaf write adds `leaf`). Mirrors the `doc show` `#fragment` depth ladder so read and write addresses project identically.
- **`value`** / op-specific effect keys — a **`set-field`** ack carries `value`, the written field value shaped exactly as `doc show`'s `fields` project it (scalar → string, list → array), so a write-back and a read-back read through one shape. The other verbs carry their own effect keys, **not** `value`: **`set-slot`** → `chars` (the written prose's length; the prose is read back via `doc show`, never echoed into the ack), **`remove-item`** → `removed`, **`retitle-item`** → `title`; **`create`** / **`author`** / **`add-item`** carry no `value` — their effect is the whole doc / the new item at `target`, read back via `doc show`. Every verb carries `op`, `target`, and `findings`.
- **`findings`** — the findings envelope (below); an empty array when the write raises no intrinsic advisory. **Scope is the doc's own intrinsic, authoring-stage-agnostic advisory: `surplus-sections-absent`** (an undeclared section in the staged doc is an anomaly at *any* authoring stage) — the pure-function check that takes just `(schema, staged-buffer)`, needs no index or subprocess, and is computed from the staged working-area file right after the write persists. **Completeness** advisories are deliberately excluded: `repeatable-populated` (an empty repeatable) and `required-slot-absent` are *expected* mid-authoring (a fresh `create` legitimately has empty sections), so surfacing them on every early write is noise, not signal — they stay adopt-/store-scope on `jigc validate`. The cross-doc / subprocess families (`ref-resolves`, `inverse-cardinality`, `mention`, `doc-code`) likewise stay store-scope: they answer "is the *corpus* complete," which a single mid-authoring write cannot adjudicate. **So `findings: []` means "no intrinsic single-doc advisory," not "passes full validation"** — a driver runs `jigc validate` for corpus-completeness (this bound is stated so a driver does not over-trust the empty array). *(Not a new idea: `jigc task finalize --format json` already emits success-path findings — the wave extends that pattern to `DocAck`.)*

### 3 · The findings envelope — carried by acks, blocks, and `jigc validate --format json`

One envelope shape, pinned once here, consumed everywhere a finding is emitted as data: a write-ack's `findings[]`, a blocked write's error payload, and `jigc validate --format json`'s findings list (which **inherits** this shape rather than inventing its own — reconciling the separately-deferred "stable `validate --format json` as a CI-gating contract", [decisions-pending.md](../implementation/decisions-pending.md)).

```json
{ "code": "doc-code.symbol-exists",
  "severity": "blocking" | "warning" | "advisory",
  "key": { "code": "doc-code.symbol-exists", "target": "arch-doc:overview#components/api/implemented-by" },
  "message": "<the located, human/agent-legible message>",
  "route": "<the repair direction>" | "<informational 'no action needed' value>",
  "location": { "line": <n>, "col": <n> } | null
}
```

- **`code`** — the finding code (`<probe>.<check>`), the existing stable identifier for the *kind* of finding.
- **`severity`** — `blocking` / `warning` / `advisory` (the engine `Severity`, floor `advisory` — there is no off/silent level; a finding is emitted or it is not).
- **`key`** — **the stable per-instance identity** (below): `{ code, target }`, where `target` is a stable address (a `doctype:slug#fragment` or an anchor value), **never** line/col. This is what lets a driver dedupe/track a finding across sweeps and what a future acknowledge-ledger keys on ([decisions-pending.md](../implementation/decisions-pending.md) → the deferred ledger).
- **`message`** — the located message (unchanged).
- **`route`** — the direction, **always present** (M41 Fork 2 / V15 floor rule, [validation.md](validation.md) → the advisory-route floor; never `null`). Two route *kinds*, distinguished so a driver/agent reads the right intent: a **repair route** names an action — mechanical (`run jigc …`) *or* a non-mechanical human one ("correct or drop the mention", "author a referrer" — a real fix jigc can't execute); an **informational route** ("no action needed — uncheckable by design", e.g. `unsupported-language`) marks a genuine no-op. The floor rule is "every finding routes," **not** "every route is 'no action needed'" — flattening the two would tell an agent a dangling mention needs no action (a regression). This honors [validation.md](validation.md)'s rationale (`none` was fine only where there is no *mechanical* route) by reifying that case as an explicit route value rather than a null. (Supersedes the earlier "route is optional / `none` is fine" model.)
- **`location`** — line/col when the finding has a source position, else `null`. **Advisory only** — `location` is *not* part of the stable key (it churns under edits); it is a convenience pointer for a human reader.

## The stable finding key — `{ code, target }`

Findings-as-data is a sound 1.0 contract only if a finding has a **stable per-instance identity** — otherwise a driver cannot tell "the same finding as last sweep" from "a new one," and the pin freezes an unusable key. Today a `Finding` is keyed only by its line/col `location` ([finding.rs](../crates/engine/src/finding.rs)), which moves on every edit.

The key is **`(code, target)`**:

- **`code`** — the dotted finding id (`<probe>.<check>`), stable and present on every finding. The key uses `code` (per-code granularity for dedup), *not* `(probe, check)` — that pair stays the *severity* handle, a separate concern (the two diverge only where several codes share one check).
- **`target`** — a stable address in the **URI normal-form** `<type>:<slug>[#<fragment>]` — jigc's canonical identity, which survives reorder/retitle and changes only through `jigc rename` (which repoints referrers). Explicitly **not** the filesystem-path form (`<location>/<slug>.md#…`) some findings emit today, which breaks on rename and is not the addressing grammar. `location.line/col` is never part of the key (it churns under edits).

**The wave's real work here is normalizing `target`, because two address conventions coexist today** (spike-verified against the finding constructors):

- The **URI-form families** already emit `<type>:<slug>[#field]` and pass through: **doc-code** (`doctype:slug#field`, the anchor value in the message), **inverse-cardinality** (`doctype:slug`, one-per-doc), **ref-resolves** and **mention-resolves** (both locate at the source-doc identity `edge.from` / `from` = `doctype:slug` — index-family, they do *not* touch `attribute_to_doc`), **file-state** (the doc path, per-file).
- The **re-parse families** — **schema-conformance** and **surplus-sections-absent** — emit a **filesystem path** (`<location>/<slug>.md#…`) via `attribute_to_doc`, which breaks on rename. These need a **path→URI flip**: the engine already holds `(type, slug)` at emit time in the store walks (`schema_conformance_store`, `hollow_surplus_store`) and currently discards it for `rel_key`; the flip generalizes `attribute_to_doc` to emit `<type>:<slug>#<fragment>` (the `canonical_path` forward map is already relied on).

**Fragment synthesis for collision-freedom.** `(code, target)` must be unique *per finding instance*, so any family that can emit two findings on one doc needs a discriminating fragment:
- **ref-resolves** on a `0..*` ref fans **one finding per dangling target**, all at `doctype:slug` with the same relation — so the fragment must carry **both** relation and target value: `#<relation>/<to-slug>` (relation alone collides — two dangling `supersedes` targets would key identically, and acknowledging one would silence both).
- **mention-resolves** — fragment = the mention token; if the same token dangles twice in one doc, add an occurrence discriminator, *or* declare an explicit rule that one keyed finding covers all occurrences of a token in a doc (either is fine — pick one and pin it).
- **surplus-sections-absent** — carries no fragment; synthesize a per-surplus-heading slug (`#<surplus-heading>`) so multiple surplus sections key distinctly.

Once every family qualifies to a URI `target` with a **collision-free** fragment, `(code, target)` is unique-per-instance and stable across sweeps — sound for driver dedup and the future acknowledge-ledger's key.

## Evolution posture (declared)

The command-output contract follows the **`doc show` posture**, not an inline per-message version integer (which would be noise on outputs emitted every call): **additive keys are permitted pre-1.0 only; from the 1.0 pin the shape evolves solely by an explicitly versioned extension — never a silent additive key.** The version home is this declaration plus the golden tests that pin each shape at ship — the same mechanism `doc show` uses. A post-1.0 breaking change ships as a declared, versioned extension, exactly as the read side.

This wave pins the whole driver contract (task id, decomposed target, findings-as-data, the stable key) inside the still-open pre-1.0 window — so what would otherwise be a post-pin versioned-extension chore lands additively now, on the surface the vision names as jigc's reason to exist.
