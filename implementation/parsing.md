# Parsing & serialization

How the CLI reads, edits, and re-reads the canonical Markdown that [storage.md](../design/storage.md) specifies — the **mechanism**, not the syntax. The on-disk *format* (front-matter, schema-fixed headings, `key: value` field groups, `{#id}` anchors, "files are truth") is design and lives in [storage.md](../design/storage.md); the *schema* the parse is driven by is [document-type-schema.md](../design/document-type-schema.md); the write verbs and reconciliation rules it serves are [write-commands.md](../design/write-commands.md). This doc owns the read pipeline, the surgical-edit/write pipeline, conformance diagnostics, and the round-trip guarantees. The library context (`pulldown-cmark`) is in [language-runtime.md](language-runtime.md). For the *why*, see [DECISIONS.md](../DECISIONS.md).

The single risk this doc retires: **lossless, diff-clean, schema-driven editing of the source-of-truth files.** The strategy throughout is **offset-splice, never re-stringify, re-parse to validate** — every AST stringifier reformats, so the diff-clean path is splicing edits into the original byte buffer.

## The parse model

Raw bytes become the schema's section/slot/field/item structure through **a full CommonMark block parse, mapped onto the schema** — not a line scanner. A line scanner misreads a `## …` line *inside a fenced code block* as a heading, and slot prose is arbitrary LLM/human Markdown that will contain such cases; a real block parse identifies block boundaries correctly and hands us trustworthy byte ranges.

- **Library: `pulldown-cmark`** — pure-Rust (no C-dep cross-compile friction; see [language-runtime.md](language-runtime.md)), event stream with **byte-offset** ranges, ideal for offset-based edits. Rejected: `comrak` (line/col-only spans, documented inline imprecision); `tree-sitter-markdown` (C dependency, grammar targets highlighting not CommonMark semantics) — kept only as a fallback if span precision ever proves insufficient (worth a spike before any switch).
- **Mapping:** heading text → schema section; the span between a heading and its trailing field group → the slot's content span; the trailing bullet group → fields; `###` + `{#id}` → items.
- **Parsing respects the determinism boundary, literally.** The CLI parses and owns **block structure** (north of the line — which section, where it begins/ends, field positions, item anchors) and treats **slot prose as an opaque byte span** it never interprets (south of the line). The future "mentions in prose" ref-check ([document-type-schema.md](../design/document-type-schema.md) open question) is a separate, deferred validation layer that may scan *within* a slot span — it does not make the content non-opaque to the parser.

## Front-matter

Front-matter is parsed as **our own flat `key: value` field block — not a general YAML document**:

- **"One field syntax, two locations" forbids full YAML.** Section-trailing field groups can't be an arbitrary YAML document, so both locations must share one flat parser; front-matter is that parser's block at the top, between `---`.
- **Fields are schema-typed, so YAML's dynamic typing fights us** (the Norway problem `no`→false, bare dates→date objects, `1.0`→float). We read each value as a **string**; the schema adjudicates the type.
- **Diff-clean:** a general YAML parse→serialize reformats (quote style, key order, flow vs. block); flat lines splice surgically, one per field.

**Mechanism:** pulldown-cmark's metadata-block extension *recognizes and bounds* the `---` block (it's just the first event — consistent with "the parser owns block boundaries"); our field-line parser reads the contents (self-stripping the fence is the trivial fallback if the extension's edge behavior is inconvenient — not load-bearing). **List-valued fields** (a forward relation with cardinality > 1) use **inline flow** — `relates-to: [adr:a, adr:b]` — preserving one line per field; the *many* side is usually the derived inverse or a repeatable section, so high-card lists rarely live here.

**Scope.** This parser handles *document-instance* front-matter (and section-body field groups). Workflow and step *definition* front-matter is config-family YAML, parsed by the config parser ([overrides.md](../design/overrides.md)) — workflow defs need nested keys (e.g. `fan-out.over` / `fan-out.run`). Two parsers, two families: managed-document instances are flat-field; config-family files are YAML.

## Field-group delineation

A section body is `[slot prose][trailing field group]`, and slot content is opaque — so the field group is **declared by a sentinel the CLI writes**, never inferred from bullet-key content. (Front-matter stays bare because its `---` fences already frame it: the **field grammar is one** (`key: value`); each location carries the structural frame that disambiguates — fences vs. sentinel + bullets.)

- **Body/item fields are a trailing *bullet list* (`- key: value`) preceded by an HTML-comment sentinel** on its own line, blank-line separated, in schema order:
  ```markdown
  Slot prose ends here.

  <!-- fields -->
  - status: accepted
  - priority: high
  ```
  The parser identifies the field group by sentinel + following List, **never** by matching bullet content against declared field names. Slot prose that legitimately ends with `- status: TBD` has no sentinel and is unambiguously prose — the silent prose-to-field reclassification (the pathological case the old "schema-keys-match" rule allowed) is gone.
- **The sentinel is a reserved structural marker.** `<!-- fields -->` may appear **only** at the canonical trailing position of a section / item body (after slot prose, preceded by a blank line, immediately followed by a bullet list). Anywhere else — inside slot prose, without a following list, with a following list whose keys don't match declared fields — is a conformance error. Slot prose may not include `<!-- fields -->` as decorative content.
- **Sentinel is emitted only when fields exist.** A section / item with no declared body/item fields has no sentinel. Visual cost is bounded — sentinels appear only where the schema warrants them.
- **One rule, both places:** a repeatable item's body is also `[prose][<!-- fields -->][bullet list]`, bounded by the next `###` / section end.
- **Robust by the determinism boundary:** the **CLI writes canonically** (sentinel after all slot content, blank-line separated, schema order) — unambiguous by construction. Reconciling a human edit reads only the sentinel-following List against the schema's declared keys — match → fields, mismatch → precise conformance error (orphaned sentinel / unknown key / required field absent). Per strict-MVP scope ([reconciliation.md](../design/reconciliation.md) → Auto-repair scope), the error names the missing sentinel or key; no auto-restore.
- **Field values** are the **literal text after `key:`**, trimmed, adjudicated by the schema type (not interpreted as Markdown). A `code-anchor` value carries presentational backticks: **stripped on read, re-added on write** (keeps diffs readable without polluting the stored value).

## Slot heading-depth ceiling

Slot prose may contain free-form Markdown — paragraphs, lists, code blocks, emphasis, inline links, thematic breaks — **with one exception**: no headings at the CLI's structural depths.

- **Forbidden in slot prose:** ATX headings at `##` (section depth) and `###` (repeatable-item depth); Setext underline-style headings at any depth (`===` and `---` underlines — Setext is unusual in agent prose anyway).
- **Allowed in slot prose:** ATX headings at `####` and deeper, for slot-internal structure.

Why: `##` and `###` are unambiguously CLI-owned structural markers — section starts and repeatable-item starts. Letting slot prose contain them re-introduces the "is this a new section, or just prose with a heading?" content-sniff that the determinism boundary exists to prevent. A `## Decision` line authored inside a slot would otherwise either silently end the slot and start a new section that happens to match a schema name, or force the parser to compare heading text against schema sections (content-sniffing — exactly what the sentinel rule above eliminates for fields).

The rule has **two enforcement sites**:

1. **`set-slot` (write-time)** — scans the agent's content; on a violation, rejects with a precise conformance error pointing at the offending line: *"heading at schema-reserved depth `##` (or `###`) in slot prose at line N; use `####` or rephrase."* The agent retries with non-conflicting prose.
2. **The parser (read-time)** — on encountering a forbidden heading inside an already-located slot span (e.g. on an OOB-edit re-parse), surfaces the same conformance error. Per strict-MVP scope ([reconciliation.md](../design/reconciliation.md) → Auto-repair scope), no auto-rewrite.

The rule never bites the common case (most slot prose has no headings); it bites cleanly when it does. **The deeper principle**: the parser identifies structure by markers the CLI controls (sentinels, schema-fixed `##`/`###` headings, `{#id}` anchors), never by sniffing content against the schema.

## `{#id}` anchors

The only instance-minted in-body identity. We use **pulldown-cmark's heading-attributes extension** (`ENABLE_HEADING_ATTRIBUTES`): with it on, the parser *consumes* `{#id}` and hands us the clean heading text (the item's mutable **title** / id-source) and `id` (the **frozen id**) separately, on one event with spans. With it off, `{#id}` is literal text we'd have to re-scan — no reason to.

- **Only repeatable *items* carry `{#id}`.** Section headings are schema-fixed; the **doc** takes its identity from the **path** ([storage.md](../design/storage.md): "identity is the path"). A retitled H1 changes the title, never the frozen filename-id.
- **Missing, malformed, or duplicate `{#id}` → conformance error.** A new `###` item with no anchor is conformant content missing its wiring, but MVP does not auto-repair — the conformance error names exactly what's missing and the human (or agent via a follow-up `add-item`) supplies the anchor. Auto-mint from the recorded id-source is post-MVP, paired with the broader `jigc import` for entirely-new untracked files ([reconciliation.md](../design/reconciliation.md) → Auto-repair scope and MVP scope vs post-MVP). CLI owns ids, human owns content; silently reconstructing identity would be exactly the "the CLI just does things" behavior the determinism boundary exists to prevent.
- **Uniqueness is enforced per repeatable section** — duplicate `{#id}` = conformance error.
- **Boundary:** parsing reads `{#id}` as an opaque frozen token and maps it to the address fragment. The **slug-generation rules** (case/charset normalization, collision-suffix form) are *minting mechanics*, owned by [structural-grammar.md](../design/structural-grammar.md) — **not** decided here.

## The write pipeline

Two modes; which verb uses which is the spine:

| verb | mode |
|---|---|
| `set-slot` (section present), `set-field` (field present), `remove-item` | **surgical splice** — locate target span, replace/delete |
| `reorder` | **splice as relocation** — move whole item spans |
| `create`, `add-item`, `set-field` (field absent), `set-slot` (section absent) | **generation** — emit new canonical bytes, then insert |

**The load-bearing guarantee:** we **splice to edit existing content and generate only new content** — never regenerate-and-replace what's already there. An edit touches only the target's byte span, so a human's conformant-but-differently-spaced file is left byte-for-byte intact everywhere except the one thing changed. The canonical writer's fixed formatting applies *only* to bytes that didn't exist before (a new file, a new item block, an inserted field line). No churn, ever — the concrete cash-out of "files are truth."

**Absent structural homes generate at their schema-ordered position.** An *optional* section ([document-type-schema.md](../design/document-type-schema.md)) may be absent from the file, so a `set-slot` into it — or an `add-item` into an absent optional repeatable section — has no span to splice; it **materializes the section's structural home first**, then writes the leaf. Same generate-and-insert path the absent-`set-field` case already uses, lifted from leaves to sections. The **insertion point is deterministic from schema document-order** ([storage.md](../design/storage.md): schema order = physical order): the writer knows the full schema section list and which sections are present, and inserts the generated `##` heading so the present sections stay in schema order (after the nearest preceding present section, before the nearest following one). Generation scope is the **structural home only** — the heading plus the leaf being written; a field group and its sentinel, or sibling slots, materialize when *their* writes land, incrementally. Required-but-still-absent fields remain a `finalize` integrity concern, not forced at this write. Validate-after-write (below) covers the inserted region like any other edit.

**The canonical writer (one component)** is the inverse of the parser: schema (+ id-source, field values, slot prose) → canonical Markdown. It serves six callers — `create`, `add-item`, `set-field`-when-absent (one bullet inserted in schema order), `set-slot`-when-section-absent (the section's structural home generated at its schema-ordered position), the `--template` blank-instance view, and a **string sink** (rendering the `commit` doc into the git commit message at finalize, rather than to a file — see [write-commands.md](../design/write-commands.md), [CLAUDE.md](../CLAUDE.md) MVP scope). Its invariant, **golden-tested**: *parser/writer symmetry* — the writer emits only parser-accepted forms, and `parse → write` is **idempotent** on canonical content. So generated and conformant-human content converge on one form, and the first CLI touch of a hand-written file splices cleanly without reformatting.

**Validate-after-write — the local safety gate.** After every splice or generation, **re-parse the result and assert** before persisting: (a) it still parses against the schema, (b) *only the intended target changed*, (c) for `set-field`, the new value passes its type. On any anomaly, **abort the write** — never persist a file we can't re-parse. This is the *write-time local adjudication* of [write-commands.md](../design/write-commands.md), distinct from the `finalize` validation engine ([validation.md](../design/validation.md)), which owns cross-doc/ref integrity.

**Mechanics:**

- **MVP = one edit per command** (read working copy → parse → locate one target → splice → validate-after → write); no offset-invalidation problem. The **reverse-order batch** trick (apply edits bottom-up so earlier edits don't shift later offsets) is needed only later, when the deferred fillable form compiles multiple edits against a single parse.
- **`reorder` relocates, never renumbers** — gather each item's existing byte span, reassemble the region in the new order, splice the region; item bytes move intact (the diff reads as a move).
- **Atomic on disk** — write temp + rename.
- **Writes land in the task working area** (`.jigc/tasks/<id>/`, [storage.md](../design/storage.md)); an existing doc is copied in (base-pinned) on first touch; the committed file is untouched until `finalize`.

## Conformance diagnostics

When a human edits a file out-of-band, the re-parse must turn "doesn't match the schema" into precise, actionable errors.

- **Conformance falls out of the parser — no second pass.** The schema-mapping above *is* the conformance check: a successful mapping = conformant; each point it can't proceed = a located diagnostic. The parser returns *either* a parsed instance *or* a set of diagnostics.
- **Diagnostics are located** — address + source line/col, free from the offsets we already track.
- **Collect-all with resync, not fail-fast** — collect every diagnostic in one pass; after a *structural* mismatch, resynchronize to the next recognizable schema heading to limit cascades, and mark the **primary** error. (Some cascades are unavoidable when structure is badly broken.)
- **One finding shape, two producers.** Conformance diagnostics use the same `finding` shape as validation ([validation.md](../design/validation.md) — referenced, not redefined), so all problems surface through one channel. But the layers differ: **conformance is binary and intrinsic** (parses as its type, or doesn't) and is the **pre-gate** below the engine; **validation is graded and cascade-tunable**. Conformance-blocking is *not* a severity knob — a malformed file is malformed regardless of project policy.

| outcome | cases |
|---|---|
| **blocks** | missing/renamed/reordered required section heading; missing, duplicate, or malformed `{#id}` on a repeatable item; **ATX heading at `##` or `###` depth inside slot prose, or any Setext heading inside slot prose**; **`<!-- fields -->` sentinel out of canonical position** (in slot prose, without a following bullet list, or with a following list whose keys don't match declared fields); **field group without a `<!-- fields -->` sentinel**; field value malformed for its type / unknown field key (with a cheap "did you mean") / required field absent; broken front-matter |
| **auto-handled** | *none in MVP* — mint-on-import from the recorded id-source is post-MVP ([reconciliation.md](../design/reconciliation.md) → Auto-repair scope) |
| **not a conformance concern** | slot prose (opaque — the determinism boundary); an empty *required* slot (a `finalize` integrity check, not a parse error) |

## Round-trip guarantees

The diff-clean contract, in two clauses:

1. **Idempotent on canonical content** — read → write with no logical change = byte-identical.
2. **Surgical on edits** — read → change one target → write = only that target's bytes differ.

Pure byte-identity can't hold for *non-canonical* input, so precisely: **idempotent on canonical content; minimally and *locally* canonicalizing on first touch of non-canonical content.** Governing principle: canonicalize only where the diff is minimal and localized (file start/end); anything **line-spanning is preserved, never globally rewritten.**

| concern | rule |
|---|---|
| **Line endings** | preserve the file's existing EOL; match it locally at edit sites; never globally normalize; new files default to **LF** |
| **BOM** | tolerate on read (skip — it would break front-matter detection); **never emit**; stripped on first write |
| **Trailing newline** | ensure exactly one on write; tolerate either on read |
| **Internal whitespace / prose** | never reformat — no reflow, no blank-line normalization in unedited regions |
| **Encoding** | UTF-8 only; non-UTF-8 → conformance error (no transcoding) |

- **Drift hash:** the `file ↔ CLI-state` check is a **raw-byte** hash; first-touch canonicalizations re-baseline it, and subsequent no-op reads/writes don't drift. (Hash mechanism's home is [storage.md](../design/storage.md) / [validation.md](../design/validation.md).)
- **The contract is tested, not asserted:** golden round-trip tests (generate→parse→generate idempotent) plus a **property/fuzz test** — parse arbitrary conformant docs, no-op write → assert byte-identical (modulo declared canonicalizations); single-field edit → assert only that field's bytes changed.

- **Empty-slot canonical form (the M26 byte-stability fix).** A repeatable item that carries **both a slot and a field group** has two encodings of "the slot is unfilled" that must render identically, or clause 1 breaks: a freshly-minted item holds `slot: None` (`add_item` mints `### …  {#id}\n\n<!-- fields -->`, one blank line), but a re-parse of those bytes yields `slot: Some("")` (the parser populates a present-but-empty single-slot as `Some("")`). The writer canonicalizes **both to the one-blank form** — an empty present-slot renders identically to an absent slot (`render_item_at` treats `slot.trim().is_empty()` as slotless), so `render(parse(x)) == x` holds for the minted-but-unfilled intermediate. *Why it mattered:* without this, the per-leaf `doc author --from` batch (mint, then set a field on the same buffer) produced `### …  {#id}\n\n\n\n<!-- fields -->` (three blanks) on reparse — a latent clause-1 violation, masked in practice only because a required slot must be filled before commit and `ensure_single_trailing_newline` collapses the trailing-item case. Genuinely-slotless items (a schema with no slot leaf, e.g. a changelog release) keep `slot: None` and are untouched — the fix moves only items that reparse to `Some("")`. ([DECISIONS.md](../DECISIONS.md) → 2026-06-18 fork C4; found M22, scheduled M26 Increment 1.)

## Open questions

- **Span precision under stress** — pulldown-cmark inline/nested-span accuracy on real document shapes (and front-matter, which CommonMark treats as an extension) warrants an empirical spike before heavy reliance; `tree-sitter-markdown` is the fallback.
- **Multi-slot sub-label syntax** — the rendering that delimits multiple slots within one section ([storage.md](../design/storage.md) open question) determines where the parser splits slot spans; tracked there.
- **Mentions-in-prose scanning** — the deferred lighter ref-check *within* a slot span ([document-type-schema.md](../design/document-type-schema.md) open question).
