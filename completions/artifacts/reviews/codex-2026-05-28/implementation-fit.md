# Codex review — Pass 5: Implementation ↔ design fit

**Date:** 2026-05-28  
**Reviewer:** OpenAI Codex CLI (cross-model second opinion)  
**Files reviewed:** all of implementation/ (language-runtime, module-layout, parsing) verified against the cited design/ docs and CLAUDE.md invariants.  
**Lens:** Do implementation choices faithfully serve design commitments? Where has an implementation choice quietly foreclosed a design option?  
**Format:** six tables of (design promise, implementation lever, status [green/yellow/red], notes), followed by a list of foreclosed options with severity + proposed fix.

---

### Area 1: Offset-Splice, Never Re-Stringify

| Design promise | Implementation lever | Status | Notes |
|---|---|---|---|
| “The `.md` is the document” and humans edit through git. `design/storage.md:15` | “offset-splice, never re-stringify, re-parse to validate.” `implementation/parsing.md:5` | green | Strong alignment: the implementation lever directly serves the source-of-truth and reviewable-diff promise. |
| “The title is mutable; the id is frozen… survives a retitle.” `design/storage.md:63` | `ENABLE_HEADING_ATTRIBUTES` separates heading text from `{#id}`. `implementation/parsing.md:37-39` | green | For heading-backed repeatable items, title rename preserves the frozen anchor. |
| “Conformant, non-conflicting edit → accepted.” `design/write-commands.md:87` | Schema mapping returns parsed instance or located diagnostics. `implementation/parsing.md:71-74` | green | Clean OOB import is the normal parse path, not a separate lossy importer. |
| “Order = physical order… clean diff move.” `design/storage.md:64` | `reorder` relocates whole item spans, never renumbers. `implementation/parsing.md:51,63` | green | Good alignment; item bytes move intact. |
| “`--template` view… blank instance.” `design/document-type-schema.md:145` | Canonical writer serves `--template`. `implementation/parsing.md:56` | green | Valuable alignment: templates cannot drift from persisted canonical form. |
| “Commit message is just a doc type.” `design/write-commands.md:80`; commit sink in MVP. `CLAUDE.md:67-69` | Same canonical writer serves a “string sink.” `implementation/parsing.md:56` | green | Good: commit rendering does not become a separate formatter. |
| “Write-time local adjudication” plus finalize checks. `design/write-commands.md:60-65` | Validate-after-write reparses and aborts before persisting. `implementation/parsing.md:58` | green | Stronger than design without violating it; local parse safety is separate from finalize integrity. |
| Sections can be “required/optional.” `design/document-type-schema.md:11` | `set-slot` is surgical only; generation covers `create`, `add-item`, absent field. `implementation/parsing.md:48-52` | yellow | Missing/absent optional section insertion is not specified; the splice model needs a canonical insertion point for absent structural homes. |
| “Only per-instance structural variation is repeatable-section items.” `design/write-commands.md:14` | `remove-item` deletes a target span. `implementation/parsing.md:50` | green | Structural delete is safely scoped because instance-level section delete is forbidden. |

### Area 2: `pulldown-cmark` + Heading Attributes

| Design promise | Implementation lever | Status | Notes |
|---|---|---|---|
| “Robust parsing” pays the cost of plain Markdown. `design/storage.md:15` | Full CommonMark block parse, not line scanner. `implementation/parsing.md:9` | green | Correct lever; fenced headings in prose are not mistaken for structure. |
| Surgical edits need source spans. `implementation/language-runtime.md:19` | `pulldown-cmark` event stream with byte-offset ranges. `implementation/parsing.md:11` | yellow | Directionally right, but the doc itself still requires a stress spike for span precision. `implementation/parsing.md:104` |
| `{#id}` item anchors are the only in-body marker. `design/storage.md:27` | Heading-attributes extension consumes `{#id}` into id + clean heading. `implementation/parsing.md:37` | green | Good fit if repeatable items are always represented as headed blocks. |
| Slot prose is “South of the line.” `design/document-type-schema.md:13,25` | Parser treats slot prose as opaque byte span. `implementation/parsing.md:13` | yellow | Holds for content after boundaries are found; the parser still interprets Markdown block structure to decide where the slot ends. |
| Body fields are distinct from prose. `design/storage.md:26,65` | Final Markdown list whose keys match declared fields. `implementation/parsing.md:29-31` | yellow | Ambiguity is acknowledged: prose ending with a schema-looking bullet list can be interpreted structurally. |
| Front matter is a flat field block, not YAML. `design/storage.md:25` | Metadata-block bounds + custom field-line parser. `implementation/parsing.md:17-23` | green for doc instances | Correct for document instances; see Area 3 for workflow-definition front matter conflict. |

Where the parser can interpret prose against its will: schema-level headings at the expected level, schema-fixed multi-slot sub-labels once designed, and a final bullet list whose keys match declared fields. The implementation routes the final-list case to conflict/error rather than silent corruption, which is the right failure mode.

### Area 3: Flat Field Block + Body Bullet Field Groups

| Design promise | Implementation lever | Status | Notes |
|---|---|---|---|
| “One field grammar (`key: value`), two structural frames.” `design/storage.md:65` | Front matter uses field-line parser; body uses Markdown List + key parser. `implementation/parsing.md:19,29` | yellow | Scalar grammar is shared, but the actual recognizers are necessarily two framed parsers, not literally one grammar end-to-end. |
| Front matter is “a strict YAML subset, not arbitrary YAML.” `design/storage.md:25` | “not a general YAML document”; values read as strings. `implementation/parsing.md:17-21` | green | Good for document instances and typed fields. |
| Workflow definitions reuse Markdown + front matter. `design/workflow-dialect.md:72-75` | Flat parser “forbids full YAML.” `implementation/parsing.md:19` | red | Workflow front matter needs nested YAML-like config (`fan-out.over`, `run`). The implementation doc must scope flat parsing to document instances or define a separate workflow-definition parser. |
| Body field groups stay distinct from prose. `design/storage.md:26` | “final List block whose item keys match” schema fields. `implementation/parsing.md:30-31` | yellow | Acceptable but leaky; it creates an escape-hatch requirement for prose that genuinely ends with such a list. |
| `code-anchor` values keep readable backticks. `design/storage.md:65` | Strip on read, re-add on write. `implementation/parsing.md:33` | green | Good localized canonicalization. |

### Area 4: Two-Crate Workspace + Embedded Pack

| Design promise | Implementation lever | Status | Notes |
|---|---|---|---|
| Engine is “frontend-neutral and ships empty.” `CLAUDE.md:54` | `engine` lib has no frontend/domain content. `implementation/module-layout.md:9` | green at crate level | The Rust library boundary honors the invariant. |
| MVP deliverable includes embedded dev pack + Claude profile. `CLAUDE.md:68` | Dev pack and profiles embedded in `cli`. `implementation/module-layout.md:16,42` | yellow at artifact level | Real shipment is not empty; the product artifact bundles the first domain and assistant profile. This is acknowledged in MVP, but it weakens “ships empty” unless “engine crate” vs “tool artifact” is explicit. |
| “MCP could be another frontend.” `CLAUDE.md:54` | Later `mcp` bin over same engine. `implementation/module-layout.md:9,66` | yellow | The engine can be reused, but the embedded pack currently lives in `cli`; MCP would need its own embedded copy or a shared pack-provider artifact. |
| Pack-default ships with installed pack. `design/overrides.md:15` | Engine reads pack-default through “source abstraction.” `implementation/module-layout.md:16,18,72` | yellow | The abstraction is named, not specified; embedded vs on-disk pack resolution remains hand-wavy. |
| Adapter profile is assistant-specific and generated. `design/assistant-adapter.md:7-13,60` | Profiles embedded in `cli`; generated from profile + engine catalog. `implementation/module-layout.md:42-44` | green | Good alignment for Claude Code MVP and later installable profiles. |

### Area 5: Rust Choice vs Design Promises

| Design promise | Implementation lever | Status | Notes |
|---|---|---|---|
| Structural correctness over discipline. `CLAUDE.md:39-58` | Rust enums + exhaustive `match`. `implementation/language-runtime.md:11,20,34` | green | Strong alignment; this is exactly where Rust pays rent. |
| `tool start` must be path of least resistance. `design/bootstrap.md:9,33-38` | Rust “single-digit-ms start.” `implementation/language-runtime.md:17,34,50` | yellow | Plausible for a bare binary; not yet proven with embedded pack discovery, cascade loading, schema parse, and adapter/profile catalog work. |
| Validation is deterministic and scope-flexible. `design/validation.md:54-60` | Engine-native checks in-process; pack probes deferred subprocess. `implementation/module-layout.md:48-50` | yellow | Rust does not create the runtime problem, but subprocess probes over many anchors can dominate validate-time unless batching/caching is specified. |
| Engine never formats for a surface. `implementation/module-layout.md:33` | Engine result types are `Serialize`; CLI owns agent/human renderers. `implementation/module-layout.md:35-38` | yellow | Good boundary, but “JSON is enough” is only true if result types are stable semantic contracts; agent-text and TUI need more than generic serialization. |
| Non-interactive path is the floor. `implementation/language-runtime.md:23` | CLI defaults to agent-text; TUI additive post-MVP. `implementation/module-layout.md:36-37` | green | Good alignment; no interactive dependency in the core path. |
| Core makes no LLM calls. `CLAUDE.md:51` | Rust core framed as deterministic file/graph/text engine. `implementation/language-runtime.md:21` | green | No implementation choice contradicts this. |

### Area 6: Probe Boundary

| Design promise | Implementation lever | Status | Notes |
|---|---|---|---|
| “Fat engine, thin probes.” `design/validation.md:21-29` | `Probe` trait with engine-owned target resolution, scheduling, severity, aggregation, gate. `implementation/module-layout.md:48` | green | Strong alignment for in-process MVP probes. |
| One interface, two implementations. `design/validation.md:64-70` | In-process now; subprocess JSON-in/out later. `implementation/module-layout.md:49-50` | yellow | Directionally aligned, but not yet concrete. |
| Subprocess probes are language-neutral JSON. `design/validation.md:70` | JSON contract “designed for but not built.” `implementation/module-layout.md:50,73` | red | The claim “zero engine change” is unsupported until target/context/finding JSON, versioning, and effective-state access are sketched. |
| Probes are read-only and deterministic. `design/validation.md:13,70` | “read-only” trait plus deferred sandboxing. `implementation/module-layout.md:48-50,73` | yellow/red | In-process probes rely on discipline; subprocess probes have no enforcement seam yet. The design honestly calls this open. `design/validation.md:78` |
| Severity is engine-owned via cascade. `design/validation.md:31` | Engine owns severity-via-cascade. `implementation/module-layout.md:48` | green | Good alignment; probes do not bake policy. |
| MVP has no external pack probes. `CLAUDE.md:68,71` | Trait + in-process impls only for MVP. `implementation/module-layout.md:50` | green | Good scope control. |

## Foreclosed Options

**[SEV: high] Flat front matter silently forecloses workflow-definition YAML front matter.**
- Foreclosed option: Nested workflow config in Markdown front matter, including `fan-out`.
- By: `implementation/parsing.md` treating front matter as one flat `key: value` parser.
- Where it should be acknowledged: `implementation/parsing.md` → Front-matter; `design/workflow-dialect.md` → On-disk definition format.
- Proposed fix: Add “document instances use flat field front matter; workflow/step definitions use config-family YAML front matter parsed by the config parser.”

**[SEV: high] Embedding the dev pack only in `cli` makes the future MCP frontend duplicate or depend on CLI packaging.**
- Foreclosed option: A non-CLI frontend reusing the exact installed pack-default without re-embedding it.
- By: `implementation/module-layout.md` placing pack-default in the `cli` binary.
- Where it should be acknowledged: `implementation/module-layout.md` → The dev pack’s home.
- Proposed fix: Specify a `PackSource` provider boundary and whether embedded built-ins live in a shared resource crate, separate pack artifact, or per-frontend embedding.

**[SEV: med] Heading-backed item identity conflicts with the schema rule that repeatable id-source is a field.**
- Foreclosed option: Repeatable items whose slug source is an explicit field rather than the `###` heading.
- By: `implementation/parsing.md` treating heading text as item title/id-source.
- Where it should be acknowledged: `design/storage.md` → Identity/order/fields; `design/document-type-schema.md` → Sections and repetition; `implementation/parsing.md` → `{#id}` anchors.
- Proposed fix: Reconcile the model: either make item heading a special rendered field, or change the schema rule to allow heading/title as the repeatable id-source.

**[SEV: med] The current `Probe` trait claim forecloses zero-change subprocess probes without proving the wire contract.**
- Foreclosed option: Adding polyglot probes later with no engine changes.
- By: A Rust trait signature plus deferred JSON shape.
- Where it should be acknowledged: `implementation/module-layout.md` → Probe boundary.
- Proposed fix: Add a minimal versioned JSON contract sketch: request `{probe_id,target,effective_state_ref,config}`, response `{findings:[...]}`.

**[SEV: med] Final schema-looking prose lists become structurally reserved.**
- Foreclosed option: Slot prose ending freely with `- declared-field: value` as ordinary prose.
- By: Body field group = final list matching schema keys.
- Where it should be acknowledged: `implementation/parsing.md` → Field-group delineation; `design/storage.md` → Anatomy.
- Proposed fix: Document an escape rule, e.g. fence/blockquote such lists or require a blank/comment sentinel before prose-final lists.

**[SEV: low] Generic `Serialize` as renderer contract may underspecify future non-CLI consumers.**
- Foreclosed option: Stable frontend-independent semantic rendering without Rust type coupling.
- By: Treating JSON as automatic over engine result structs.
- Where it should be acknowledged: `implementation/module-layout.md` → Renderers.
- Proposed fix: State that public result types are versioned API schemas, not incidental `serde` dumps.
