# Codex review — Pass 1: Consistency & terminology drift

**Date:** 2026-05-28  
**Reviewer:** OpenAI Codex CLI (cross-model second opinion)  
**Files reviewed:** all 14 (VISION + CLAUDE + DECISIONS + 9 design/ + 3 implementation/)  
**Lens:** Find places where the doc set disagrees with itself — direct contradictions, vocabulary drift, duplicated facts that violate the project's own "cross-reference, never restate" rule.  
**Format:** each finding has severity, two or more location citations with exact quotes, why-it-matters, and a one-line proposed fix.

---

**[SEV: high] `CLAUDE.md` still claims the repo contains only `VISION.md` and has no chosen runtime, contradicting the actual design/implementation doc set.**
- Location A: `CLAUDE.md:5-8` — "Pre-implementation. The repository currently contains **only `VISION.md`** — the design thesis. There is no code, build system, test suite, or chosen language/runtime yet."
- Location B: `implementation/language-runtime.md:7-11` — "**The engine and the CLI are written in Rust.**"
- Why it matters: The assistant entry file gives false working-state guidance before every design session.
- Proposed fix: Replace `CLAUDE.md` project-state paragraph with “pre-implementation; design lives in `VISION.md` + `design/`, implementation decisions in `implementation/`; Rust is chosen.”

**[SEV: high] The docs disagree on ID minting sites: structural grammar says exactly two, write commands add task IDs as a third.**
- Location A: `design/structural-grammar.md:66-71` — "Runtime minting happens at **exactly two sites**: creating a container, and adding an item to a repeatable unit."
- Location B: `design/write-commands.md:49` — "The id is a **slug from the intent** (frozen, collision-suffixed — a third minting site alongside doc-creation and item-add)"
- Why it matters: ID minting is a core deterministic operation; “exactly two” vs “third” breaks the model boundary.
- Proposed fix: Update structural grammar to include task IDs as a third minted identity class, or explicitly define task IDs as outside the managed-artifact minting model.

**[SEV: high] `tool start` task-creation semantics conflict across current docs.**
- Location A: `design/write-commands.md:48-50` — "`tool start \"<intent>\"` ... No task is minted yet — this is task-less orientation." / "`tool start --workflow <X> \"<intent>\"` — mints the task" / "bare **`tool start`** — orients"
- Location B: `CLAUDE.md:66` — "`tool start` (the state-aware entry — mint task, resolve the default workflow, orient) is built"
- Why it matters: The front door is the one command the agent must know; ambiguity here breaks bootstrap behavior.
- Proposed fix: Make `CLAUDE.md` match `write-commands.md`: bare start orients; `start "<intent>"` routes/defaults; task minting happens only once a workflow is selected.

**[SEV: high] VISION still describes a human-confirmed `--dry-run` proposal flow after write-commands decided validate/finalize with autonomous finalize.**
- Location A: `VISION.md:115-116` — "LLM ... proposes to the human (CLI shows a placement + integrity preview, `--dry-run` style) → on confirmation, writes via CLI"
- Location B: `design/write-commands.md:58-69` — "There is no separate proposal object" / "`finalize` ≡ `validate` + commit" / "`finalize` **defaults to autonomous**"
- Why it matters: This is a user-facing lifecycle contract, not just terminology.
- Proposed fix: Rewrite VISION’s “Propose / write loop” to say writes stage immediately, `diff`/`validate` preview, and `finalize` commits with optional confirm-gate.

**[SEV: high] The workflow model says both “workflow is not one file” and “workflow is one file.”**
- Location A: `VISION.md:57-58` — "A workflow is not one file — it is reusable steps assembled by the CLI"
- Location B: `design/workflow-dialect.md:72-75` — "**A workflow** = a file: front-matter for workflow metadata ... the **body is the ordered `{{include}}`s**"
- Why it matters: This confuses whether the workflow definition itself is a file or only the composed workflow is assembled from step files.
- Proposed fix: Change VISION to “A workflow is not a monolithic prompt; its definition is an include-list file assembled from reusable steps.”

**[SEV: high] MVP probe count conflicts: top-level says two engine-native probes, validation lists three under “MVP ships.”**
- Location A: `CLAUDE.md:71` — "Validation is in the MVP: the framework + the **two engine-native probes** (`workflow ↔ references`, `file ↔ CLI-state`) **plus the intrinsic `finalize` integrity gate**"
- Location B: `design/validation.md:66-69` — "The MVP ships: ... **`workflow-refs`** ... **`file-state`** ... **`override-default`**"
- Why it matters: MVP scope is currently ambiguous about whether upgrade reconciliation is implemented or only specified.
- Proposed fix: In `validation.md`, move `override-default` out of “MVP ships” and label it “engine-native at contract level; implementation post-MVP,” or update `CLAUDE.md` to three probes.

**[SEV: med] The determinism boundary table is duplicated but not identical between `VISION.md` and `CLAUDE.md`.**
- Location A: `VISION.md:96-103` — "`Workflow composition and placeholder resolution`" / "`Placement of every write`" / "`Validation results`"
- Location B: `CLAUDE.md:39-45` — "`Workflow composition + placeholder resolution`" / "`Placement of every write; validation results`"
- Why it matters: The project explicitly calls this a hard contract, and the user specifically asked whether the duplicate tables match; they do not.
- Proposed fix: Keep the canonical table only in `VISION.md`; replace the `CLAUDE.md` table with a link, or copy it byte-for-byte.

**[SEV: med] Address examples omit required fragment structure from the formal grammar.**
- Location A: `design/structural-grammar.md:53-60` — "`type:name # unit / item / leaf`" and "After `#` — the fragment: `/`-separated `unit / item / leaf`."
- Location B: `design/write-commands.md:98-99` — "`tool doc set-field commit:add-rate-limiter#type`" / "`tool doc set-slot  commit:add-rate-limiter#summary`"
- Why it matters: Commands and parsers need one canonical address grammar; examples currently imply undeclared leaf-only shorthand.
- Proposed fix: Either define a leaf-only shorthand for unique doc-level leaves, or rewrite examples as full `#section/leaf` addresses.

**[SEV: med] The MVP is declared spec-less, but the `single-task` workflow examples still read `task.spec`.**
- Location A: `CLAUDE.md:65` — "The MVP `single-task` does **not** read a SPEC (no `task.spec`, no `spec` schema ships)"
- Location B: `design/workflow-dialect.md:137-141` — "`# workflow: single-task-execution` ... `Read the spec for this task:` ... `{{ task.spec#criteria }}`"
- Why it matters: The example uses the locked MVP workflow name while showing a non-MVP data root.
- Proposed fix: Rename the example to “post-MVP spec-driven workflow” or replace `task.spec#criteria` with `task.intent`.

**[SEV: med] Slot/author marker syntax drifts between `<<...>>` and `{{ author: ... }}`.**
- Location A: `VISION.md:126` — "`<<author: addr>>` points the agent at a doc slot to fill through the write path"
- Location B: `design/workflow-dialect.md:142-145` — "`Run: {{ cli.set-commit-summary }}`" / "`{{ author: commit.summary }}`"
- Why it matters: `{{...}}` is reserved for CLI-resolved placeholders; using it for an agent-authored directive violates the slot/placeholder boundary.
- Proposed fix: Change workflow examples to `<<author: commit:add-rate-limiter#summary>>` or define `author` as emitted syntax outside placeholder grammar.

**[SEV: med] Repeatable item id-source is both a field and the heading.**
- Location A: `design/structural-grammar.md:45-48` — "A repeatable unit must designate one leaf as its **id-source** ... in the document dialect, a `field` — never agent-authored prose"
- Location B: `design/storage.md:62-63` — "**id-source = the heading** ... an item's title is its `###` heading (its frozen id is the `{#id}`)"
- Why it matters: The parser/schema must know whether the item title heading is a rendered field or prose structure.
- Proposed fix: State that repeatable item headings are the rendering of the id-source `string` field, or relax the id-source rule to allow schema-owned headings.

**[SEV: med] “Front-matter” means flat field block for documents but YAML config for workflow definitions.**
- Location A: `implementation/parsing.md:17-20` — "Front-matter is parsed as **our own flat `key: value` field block — not a general YAML document**"
- Location B: `design/workflow-dialect.md:72-75` — "A step and a workflow are each **one file**, reusing the same Markdown + front-matter pattern as document instances" / "the YAML **front-matter** is the step's config"
- Why it matters: The same storage term names two incompatible parsers.
- Proposed fix: Rename document instance front-matter to “header field block” and reserve “YAML front-matter” for definition/config files.

**[SEV: med] The emitted workflow visual grammar is described as settled in VISION but open in workflow-dialect.**
- Location A: `VISION.md:63` — "The emitted workflow is structured markdown that makes unmistakably clear which lines are \"run this exact command,\" which are \"author this slot,\" and which are \"reason about X.\""
- Location B: `design/workflow-dialect.md:168-170` — "**Emitted-format micro-syntax** — the concrete visual grammar that marks \"run this exact command\" vs \"author this doc slot\" vs \"reason about X\" ... not yet decided."
- Why it matters: The design alternates between contract and open question for the same emitted format.
- Proposed fix: Mark VISION’s emitted-format sentence as intent and link to the open question, or settle the micro-syntax in `workflow-dialect.md`.

**[SEV: med] `DECISIONS.md` contains an unmarked superseded finalize rule for inverse cardinality.**
- Location A: `DECISIONS.md:22` — "implies a rebuildable edge index, inverse-cardinality enforced at `finalize`."
- Location B: `design/validation.md:60` — "Completeness obligations (inverse/minimum-cardinality ... ) ... are **advisory by default and hard-enforced only at `store`/milestone scope**, never the per-task gate"
- Why it matters: A reader scanning decisions sees an old “enforced at finalize” rule without a supersession marker.
- Proposed fix: Amend the old decision line with “superseded 2026-05-25: per-task finalize does not enforce inverse/min-cardinality.”

**[SEV: low] Validation probe/check names drift between prose labels and implementation IDs.**
- Location A: `VISION.md:122` — "`workflow ↔ references` and `file ↔ CLI-state`"
- Location B: `design/validation.md:66-69` — "**`workflow-refs`**" / "**`file-state`**" / "**`override-default`**"
- Why it matters: This creates avoidable confusion between probe IDs, human labels, and validation targets.
- Proposed fix: Define canonical probe IDs once in `validation.md` and use those IDs everywhere, with human labels only as display names.

**[SEV: low] The “cross-reference, never restate” rule is violated by restating the engine/CLI/pack split in multiple homes.**
- Location A: `CLAUDE.md:23-24` — "**Cross-reference, never restate** — if two docs state the same fact, one is wrong."
- Location B: `VISION.md:27-29` — "**Engine + CLI, kept separate.** ..." / "**Engine vs domain pack.** ..."
- Location C: `CLAUDE.md:54` — "**Engine vs CLI vs domain pack are separate.** The *engine* ... The *CLI* ... A *domain pack* ..."
- Why it matters: The repeated invariant has already drifted in adjacent docs, proving the convention’s risk.
- Proposed fix: Keep the conceptual split in `VISION.md`; make `CLAUDE.md` link to it and list only repo-specific working implications.
