# M55 — the two findings-channel doctypes: proposal for cross-review (2026-10-02)

> **[Superseded 2026-10-02 by [design/findings-channel.md](../../../design/findings-channel.md) — the design of record, which took this proposal's Revision 1 as its field set and was then revised at the design review (R1–R4, [settle-log.md](settle-log.md) → Review phase): among others, the doc-only finalize commits path-scoped and no longer refuses (`finalize.foreign-staged` withdrawn), and create-only lives on the create-gate entry (`new: true`), not a `doc create --new` flag. The body below is kept as written, the record of what was cross-reviewed.]**

Status: **proposal, generally agreed by the human; under cross-review** (Codex + an independent agent). Settled context: [settle-log.md](settle-log.md) S1–S8; evidence: [baseline-ledger.md](baseline-ledger.md), [gap-list.md](gap-list.md), [planning-findings.md](planning-findings.md).

## Settled constraints the proposal must respect

- **One doc per finding** (S1) — a location doctype, one instance per report, one created instance per task.
- **Two doctypes** (S2): a generic, router-visible **inconsistency** doctype (code↔doc / doc↔doc found in the adopter's own project — migration is its first use) and a **jigc-feedback** doctype (bugs, inconveniences, feedback about jigc), **hidden** (S3: `selectable: false` + `suppressed`, the `record-dogfood`/`dogfood-record` precedent; opened when a feedback web service exists).
- Filed through code-less workflows that compose a new **`step:finalize-doc-only`** (S4: finalize refuses any staged path other than the authored doc → `finalize.foreign-staged`) and create with **`doc create --new`** (S7: refuses an existing doc).
- **Append-only is a stated convention** (S5): after filing only `status` (and resolution fields) change.
- **`pinned-by` is a plain string** in `implementation/pinning.md` §3's grammar — `module::test_name` or `"UNPINNED: <why>"` (S6).
- **Mechanism line** (human): build new mechanism now when that is cheaper than reworking or being blocked later.
- **Version-gate:** a shipped methodology doctype is frozen at schema-version 1 under `crates/cli/packs/methodology/config/schema-manifest.yaml`. Adding an optional field, widening an enum, relaxing required→optional are cheap later migrations; removing a field, narrowing an enum, changing a type, tightening optional→required are refused or need authored prose. No conditional requiredness exists in the schema format.
- Engine-native field types: `enum, string, date, bool, int, ref, owned-location`. A `ref` names **one** target doctype (`to: <type>`, e.g. `vision.yaml`'s `grounded-in: ref to research`); refs on a header resolve and report precisely (per-doc), dangling refs block at finalize.

## Proposal

### `jigc-feedback` (hidden; workflow `report-jigc-feedback`) — jigc's own vocabulary is fine here

| field | type | required |
|---|---|---|
| `kind` | enum `bug · inconvenience · feedback` | yes |
| `version` | string — the jigc version it was seen on | yes |
| `status` | enum `open · fixed · declined · duplicate · refuted`, default `open` | yes |
| `date` | date, `set: on-create` | yes |
| `tier` | enum `1 · 2 · 3` — our review predicate (triage may set it) | no |
| `door` | string — the verb/surface (e.g. `task finalize`) | no |
| `pinned-by` | string, pinning.md grammar | no |
| slots | `description` (required), `repro` (optional, fenced block) | |

### `inconsistency` (router-visible; workflow `report-inconsistency`) — generic, for any adopter

| field | type | required |
|---|---|---|
| `kind` | enum `code-doc · doc-doc` | yes |
| `status` | enum `open · resolved · intended · refuted`, default `open` | yes |
| `date` | date, `set: on-create` | yes |
| `left` / `right` | strings — path or address of each side | yes |
| slots | `description` (required), `evidence` (optional) | |

### Choices made

- No "finding" in either id — "findings" already means validation output across the CLI and its pinned JSON contract (`design/command-output-contract.md`). No "drift" — jigc uses it for out-of-band file edits.
- `repro`/`tier` optional because conditional requiredness does not exist and a repro on *feedback* is wrong.
- No `trigger` field (the M53 Settle's six seed rows carry one; it would go into the description) — an optional string, cheap now or later.

### OPEN — the human's addition: "a reference so [we know] where the feedback belongs to"

The human wants each feedback item to carry a reference to where it belongs. Interpretation is open; reviewers should weigh in:
- **context** — the task / milestone / workflow / doc during which it was found (e.g. the milestone-record of M56 when the port surfaced it), or
- **subject** — the managed doc, workflow or doctype it is about.
Constraint: a `ref` targets one doctype; tasks and workflows are not docs; a typed ref to `milestone-record` validates (dangling blocks) but only where milestone records exist (this repo gets them at the port, M56). A polymorphic "ref to any doc" is new mechanism (the mechanism line above admits it if cheaper now than later). A plain string address is unvalidated. Does `inconsistency` need the same?

## What the reviewers are asked

Review this proposal adversarially against the settled constraints and the vision: fields that will need a refused migration later (one-way doors), missing fields the port (M56) or the fix pass (M57 triage) will need, enum values that are wrong or missing, required/optional choices, the id names, the reference question, the read surface ("all open feedback" costs `doc list` + N `doc show`; `doc show --format json` on a per-instance doc has no title key — F18), and contradictions with existing design docs (e.g. `completion-record`'s findings vocabulary, `deferral-ledger`, `ideas/finding-doctype.md`, `implementation/pinning.md`, `implementation/doctype-map.md`'s deliberate outs, which exclude a TODO/backlog doctype).

## Revision 1 — after the cross-review (2026-10-02)

Reviews: Codex ([review-doctypes-codex.md](review-doctypes-codex.md)) and an independent design-reviewer that drove both schemas on the rc.22 debug binary (scratch pack `rvpack.*`). The human answered the open question: the reference is **context — where it was found**, so feedback can be sorted, grouped and filtered later.

### `jigc-feedback` (hidden; `report-jigc-feedback`; home `jigc-feedback/`)

| field | type | req | change |
|---|---|---|---|
| `kind` | enum `bug · inconvenience · feedback`, each defined in `usage` | yes | members defined (circular `feedback` called out) |
| `found-in` | string, grammar `<kind>:<value>` — `milestone:<slug>` · `task:<id>` · `workflow:<id>` · `review:<label>` · `trial:<label>` | yes | **new — the human's context reference** (string, not a typed ref: seed rows and adopters have no milestone-record; a dangling ref blocks with a route a reporter can't act on — driven) |
| `about` | string, grammar `jigc <verb>` · `workflow:<id>` · `step:<id>` · `doctype:<id>` · `<finding-code>` · `guide:<file>` | no | was `door` — renamed while renaming is free, grammar widened |
| `jigc-version` | string, `<semver>[+<short-sha>]` | yes | was `version` — collides with the injected `schema-version` in `doc show` JSON |
| `status` | enum `open · resolved · declined · duplicate · refuted`, default `open` | yes | `fixed` → `resolved` (fits all three kinds; the how goes in `resolution`) |
| `tier` | enum `tier-1 · tier-2 · tier-3`, the exit rule as the scale's one home | no | numerals refused at pack-load (`expected a string`) — driven |
| `duplicate-of` | ref → `jigc-feedback`, card 0..1 | no | new — driven: resolves, lands, blocks when dangling |
| `pinned-by` | string, pinning.md §3 grammar | no | unchanged |
| `date` | date, `set: on-create` | yes | unchanged |
| slots | `description` (req) · `repro` (opt, fenced) · `resolution` (opt) | | `resolution` new — where the reason for a non-open status goes |

### `inconsistency` (router-visible; `report-inconsistency`; home `inconsistencies/`)

| field | type | req | change |
|---|---|---|---|
| `kind` | enum `code-doc · doc-doc` | yes | |
| `status` | enum `open · resolved · intended · refuted`, default `open` | yes | |
| `date` | date, `set: on-create` | yes | |
| `sides` | **repeatable section** — item title = path/address, optional `says` slot | ≥2 by convention | **replaces `left`/`right`** — the seed already has 3–5-sided inconsistencies (D1, D3, the "ten/eleven schemas" claim); two fields → a list later is a refused removal. Driven: validates clean |
| slots | `description` (req) · `evidence` (opt) · `resolution` (opt) | | `resolution` new |

### Not taken
- `trigger` — scheduling, which is what the backlog out excludes; seed rows' triggers go into the description.
- `resolved-date` (Codex) — git history carries it; add later as a cheap optional field if a consumer appears.
- `subject` required + `door` removed (Codex) — kept as optional `about`; the human's reference is context (`found-in`).
- `found-in` on `inconsistency` — `sides` already locates it; add later cheaply if needed.

### Stated in the doctypes' usage + design doc
- Why neither is a backlog (`doctype-map.md:99`): a finding records observed evidence; accepted work is promoted to the roadmap/milestone system and the finding keeps only its resolution.
- Routing: a `completion-record` finding with `disposition: deferred` that is about jigc is filed as `jigc-feedback` (its address in `evidence`).

### Open → its own fork: the read surface
"All open feedback, grouped by `found-in`" costs `doc list` + one `doc show` per item today; `doc show` JSON has no title key (F18); a hand-deleted defaulted `status` vanishes from `fields` (F10). `design/doc-read-surface.md:88-92` admits new keys only **before 1.0**; after it, only a versioned extension.
