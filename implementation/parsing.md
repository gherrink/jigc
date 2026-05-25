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

## Field-group delineation

A section body is `[slot prose][trailing field group]`, and slot content is opaque — so the field group must be a **distinct block type**, located by the schema, not a line pattern.

- **Body/item fields are a trailing *bullet list* (`- key: value`).** A Markdown List is a different block than a Paragraph, so pulldown-cmark gives a clean prose/field boundary with trustworthy offsets — no content sniffing. (Front-matter stays bare because its `---` fences already frame it: the **field grammar is one** (`key: value`); each location carries the structural frame that disambiguates — fences vs. bullets.)
- **The schema is the disambiguator.** The field group is *the final List block whose item keys match this section's declared fields*; everything before it is the opaque slot span.
- **Robust by the determinism boundary:** the **CLI writes canonically** (field group after all slot content, blank-line separated, schema order) — unambiguous by construction. Reconciling a human edit matches the trailing List against the schema's keys; a clean match → fields, otherwise → fields-absent (if optional) or a precise **conformance error**. The one pathological case (prose coincidentally ending in `- somekey: val`) either parses to a valid value or routes as a conflict — never silent corruption.
- **One rule, both places:** a repeatable item's body is also `[prose][trailing field group]`, bounded by the next `###`/section end.
- **Field values** are the **literal text after `key:`**, trimmed, adjudicated by the schema type (not interpreted as Markdown). A `code-anchor` value carries presentational backticks: **stripped on read, re-added on write** (keeps diffs readable without polluting the stored value).

## `{#id}` anchors

The only instance-minted in-body identity. We use **pulldown-cmark's heading-attributes extension** (`ENABLE_HEADING_ATTRIBUTES`): with it on, the parser *consumes* `{#id}` and hands us the clean heading text (the item's mutable **title** / id-source) and `id` (the **frozen id**) separately, on one event with spans. With it off, `{#id}` is literal text we'd have to re-scan — no reason to.

- **Only repeatable *items* carry `{#id}`.** Section headings are schema-fixed; the **doc** takes its identity from the **path** ([storage.md](../design/storage.md): "identity is the path"). A retitled H1 changes the title, never the frozen filename-id.
- **Missing `{#id}` → mint-on-import; malformed/duplicate → conformance error.** A new `###` item with no anchor is conformant content missing its wiring: the CLI mints the id (slug from the heading) and writes the anchor back (a reviewable diff; CLI owns ids, human owns content). *MVP caveat ([write-commands.md](../design/write-commands.md)): MVP detects + blocks; mint-on-import lands with the full import flow.*
- **Uniqueness is enforced per repeatable section** — duplicate `{#id}` = conformance error.
- **Boundary:** parsing reads `{#id}` as an opaque frozen token and maps it to the address fragment. The **slug-generation rules** (case/charset normalization, collision-suffix form) are *minting mechanics*, owned by [structural-grammar.md](../design/structural-grammar.md) — **not** decided here.

## The write pipeline

Two modes; which verb uses which is the spine:

| verb | mode |
|---|---|
| `set-slot`, `set-field` (field present), `remove-item` | **surgical splice** — locate target span, replace/delete |
| `reorder` | **splice as relocation** — move whole item spans |
| `create`, `add-item`, `set-field` (field absent) | **generation** — emit new canonical bytes, then insert |

**The load-bearing guarantee:** we **splice to edit existing content and generate only new content** — never regenerate-and-replace what's already there. An edit touches only the target's byte span, so a human's conformant-but-differently-spaced file is left byte-for-byte intact everywhere except the one thing changed. The canonical writer's fixed formatting applies *only* to bytes that didn't exist before (a new file, a new item block, an inserted field line). No churn, ever — the concrete cash-out of "files are truth."

**The canonical writer (one component)** is the inverse of the parser: schema (+ id-source, field values, slot prose) → canonical Markdown. It serves five callers — `create`, `add-item`, `set-field`-when-absent (one bullet inserted in schema order), the `--template` blank-instance view, and a **string sink** (rendering the `commit` doc into the git commit message at finalize, rather than to a file — see [write-commands.md](../design/write-commands.md), [CLAUDE.md](../CLAUDE.md) MVP scope). Its invariant, **golden-tested**: *parser/writer symmetry* — the writer emits only parser-accepted forms, and `parse → write` is **idempotent** on canonical content. So generated and conformant-human content converge on one form, and the first CLI touch of a hand-written file splices cleanly without reformatting.

**Validate-after-write — the local safety gate.** After every splice or generation, **re-parse the result and assert** before persisting: (a) it still parses against the schema, (b) *only the intended target changed*, (c) for `set-field`, the new value passes its type. On any anomaly, **abort the write** — never persist a file we can't re-parse. This is the *write-time local adjudication* of [write-commands.md](../design/write-commands.md), distinct from the `finalize` validation engine ([validation.md](../design/validation.md)), which owns cross-doc/ref integrity.

**Mechanics:**

- **MVP = one edit per command** (read working copy → parse → locate one target → splice → validate-after → write); no offset-invalidation problem. The **reverse-order batch** trick (apply edits bottom-up so earlier edits don't shift later offsets) is needed only later, when the deferred fillable form compiles multiple edits against a single parse.
- **`reorder` relocates, never renumbers** — gather each item's existing byte span, reassemble the region in the new order, splice the region; item bytes move intact (the diff reads as a move).
- **Atomic on disk** — write temp + rename.
- **Writes land in the task working area** (`.tool/tasks/<id>/`, [storage.md](../design/storage.md)); an existing doc is copied in (base-pinned) on first touch; the committed file is untouched until `finalize`.

## Conformance diagnostics

When a human edits a file out-of-band, the re-parse must turn "doesn't match the schema" into precise, actionable errors.

- **Conformance falls out of the parser — no second pass.** The schema-mapping above *is* the conformance check: a successful mapping = conformant; each point it can't proceed = a located diagnostic. The parser returns *either* a parsed instance *or* a set of diagnostics.
- **Diagnostics are located** — address + source line/col, free from the offsets we already track.
- **Collect-all with resync, not fail-fast** — collect every diagnostic in one pass; after a *structural* mismatch, resynchronize to the next recognizable schema heading to limit cascades, and mark the **primary** error. (Some cascades are unavoidable when structure is badly broken.)
- **One finding shape, two producers.** Conformance diagnostics use the same `finding` shape as validation ([validation.md](../design/validation.md) — referenced, not redefined), so all problems surface through one channel. But the layers differ: **conformance is binary and intrinsic** (parses as its type, or doesn't) and is the **pre-gate** below the engine; **validation is graded and cascade-tunable**. Conformance-blocking is *not* a severity knob — a malformed file is malformed regardless of project policy.

| outcome | cases |
|---|---|
| **blocks** | missing/renamed/reordered required section heading; duplicate or malformed `{#id}`; field value malformed for its type / unknown field key (with a cheap "did you mean") / required field absent; broken front-matter |
| **auto-handled** | missing `{#id}` on an item → mint-on-import |
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

## Open questions

- **Span precision under stress** — pulldown-cmark inline/nested-span accuracy on real document shapes (and front-matter, which CommonMark treats as an extension) warrants an empirical spike before heavy reliance; `tree-sitter-markdown` is the fallback.
- **Multi-slot sub-label syntax** — the rendering that delimits multiple slots within one section ([storage.md](../design/storage.md) open question) determines where the parser splits slot spans; tracked there.
- **Mentions-in-prose scanning** — the deferred lighter ref-check *within* a slot span ([document-type-schema.md](../design/document-type-schema.md) open question).
