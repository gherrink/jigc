# Auto-migration (Flow-B transform path)

**Status: settled at M23 planning (2026-06-14); the locked spec the milestone decomposes against.** Auto-migration (G1) is the second of the two milestones the paired post-M21 deferral was **split** into ([DECISIONS.md](../DECISIONS.md) → 2026-06-14 split; the doctype-expansion track G2 shipped as M22). It adds the **transform** arm to existing-project ingestion: an existing project's non-conformant foreign doc is **rewritten into conformant shape and adopted**, where M9/M21's [detect-and-route](project-setup.md) ([Flow 2](project-setup.md)) only classifies + routes and never rewrites. **Changelog is the only target this milestone migrates** (it has the live Flow-B demand and M22 shipped its schema); generalizing to the location-bearing doctypes (`adr`/`spec`/`prd`/`arch-doc`) is **M24**. The north star (M24+): introducing jigc into an existing project reads everything already there — code, docs, planning files — and rewrites it into jigc structure through the CLI.

## The determinism boundary stays intact (Framing A)

The standing charter framed G1 as "research-grade — relaxes the determinism boundary, needs tolerant/fuzzy heading-mapping machinery the strict parser withholds." **That framing is superseded.** It conflated two different acts:

- **The LLM does the fuzzy mapping** — reads the foreign file and *reasons* about which content becomes which section/slot, emitting a canonical-shape rewrite. This is **prose/structural reasoning → a proposal**, already inside the boundary ([VISION.md](../VISION.md#the-determinism-boundary) → the mixed-decision clause: "the agent reasons; the CLI executes").
- **The CLI does the fuzzy mapping** — heuristic heading-match / alias tables / edit-distance. *This* would relax the boundary by making the CLI a second, non-strict structural authority. **It is not built.**

Under Framing A the CLI's strict `parse → conformance-gate` stays the **sole** structural authority: the LLM's rewrite is just another upstream proposal that must survive the existing strict gate, exactly as a `create`-gated authoring does. The [determinism-boundary table](../VISION.md#the-determinism-boundary) does **not** change. This is the load-bearing argument of the milestone: migration is *reuse of the existing author + adopt path*, not new placement machinery.

## What it proves

The Framing-A migration mechanism, end-to-end, on one real doctype:

1. An existing non-conformant foreign `CHANGELOG.md` is read by the CLI and its content surfaced to the LLM as **deterministic workflow context** (a CLI-owned source seam — not an out-of-band agent read).
2. The LLM rewrites it to canonical `changelog` shape **through the existing write verbs** (`doc create` / `add-item` / `set-slot` / `set-field` — the proven [record-change](changelog.md) spine).
3. The CLI **strict-parses + conformance-gates** the rewrite; a non-conformant rewrite is **rejected and routed**, never adopted.
4. A **human review gate** shows the fidelity diff (foreign original → canonical rewrite) and blocks on approval — because the strict parse guarantees *structure*, never *content-faithfulness* (a structurally-perfect doc with dropped or hallucinated content passes the gate). Fidelity is LLM prose, unverifiable by the CLI ([VISION.md](../VISION.md) → "never guarantees the prose"); the human is its only check.
5. On approval, finalize **adopts** the managed doc (the existing register-only adopt: parse → conformance-gate → `index.absorb_doc` → file-state hash) at its canonical home **and retires the foreign original**, in one transaction.

This flips the `changelog` doctype's [Flow-A-only honest bound](changelog.md) for changelog: an existing `CHANGELOG.md` becomes trackable. It unblocks the **Flow-B existing-project live test** against a real foreign-doc corpus.

## Does not prove (honest bounds, recorded so the completion audit can't charge them)

- **Changelog only.** `adr`/`spec`/`prd`/`arch-doc` migration is M24. The verb + workflow + review-gate + retire mechanism is built **to generalize** (target doctype is a parameter), but only changelog is exercised + accepted here.
- **No CLI fuzzy heading-mapping.** A foreign heading is mapped to a schema section by the *LLM's rewrite*, never by CLI heuristics. A rewrite that fails the strict gate is rejected, not coerced.
- **Migration fidelity is human-judged, not CLI-verified.** The strict gate adjudicates structure + field values; whether the rewrite faithfully preserved the foreign content is the review gate's (human's) call. By design.
- **Accepted content drops.** Real Keep-a-Changelog files carry surfaces the `changelog` schema has no home for — the doc-level preamble paragraph, a per-release prose summary, the `[Unreleased]` compare-link, foot-of-file reference-style link definitions. These are **dropped** in migration (mostly boilerplate); the same honest-bounds posture [changelog.md](changelog.md) takes for entries-as-prose. A release's compare URL maps to the optional `link` field; everything else above has no slot and is lost.
- **Version-slug collisions block, not resolve.** See *Slug collision* below.

## The mechanism

### The `jigc migrate` verb (the trigger)

A new verb: `jigc migrate <path> --as <doctype>`. Settled because the alternatives **cannot reach the target**: a foreign `CHANGELOG.md` at repo root classifies `Unmanaged` (no finding, no route, no hook — `crates/cli/src/render.rs` renders an Unmanaged row as "left untouched"), so extending the `needs-reconcile` arm of `ingest` is structurally unable to reach it, and `jigc start` has no source-path parameter. An explicit verb taking `path + --as <doctype>` addresses an arbitrary foreign file directly, sidestepping location-based classification. (This also makes real the `migrate` verb the existing advisory route string at `crates/engine/src/file_state.rs` already names — today vaporware.)

`jigc migrate <path> --as changelog`:
1. Reads the foreign file's bytes.
2. Mints an **off-router migration task** (mirrors `record-change`: `creates-task: true`, `selectable: false`, `allows-create: [{type: changelog, as: changelog}]`), recording the foreign source path on the task.
3. Loads the foreign content into the task as **CLI-owned context** (see the source seam) and composes the migration workflow.

### The source seam (foreign content → composed context)

The read-path placeholder grammar is closed over managed state only (`data_value.rs` heads are `task`/`store`/`milestone`/managed-doc-id; `compose.rs` includes only `step:<id>`). Migration needs the foreign file's content in the composed workflow. **Settled: a CLI-owned source seam** — the `migrate` verb stages the foreign bytes into the task (a task-scoped source artifact) and a new placeholder surfaces them into the composed migration workflow as deterministic context. Composition stays reproducible and the read stays on the determinism boundary (the CLI owns the read; the agent does not fetch the file out-of-band). The seam carries **raw foreign bytes as read-only context** — it is *input to the LLM's reasoning*, never a managed doc or an addressable slot.

### The migration workflow (the rewrite)

An off-router `migrate-changelog` workflow (the `record-change` shape) whose body: surfaces the foreign content (the source seam) → directs the LLM to author the canonical `changelog` through the existing `doc create changelog` + `add-item` (releases, nested change-groups) + `set-field` (`date`, optional `link`) + `set-slot` (`notes`) sequence → `step:migrate-finalize`. The LLM never places; every structural act is a CLI write verb. The release `date` is set from the foreign file's **historical** date (the `set: on-create` today-stamp is overwritable by a `set-field` — confirmed at planning), so historical dates survive migration.

### The review gate (fidelity)

A **net-new finalize approval gate** (not the compose-time [`checkpoint` step kind](workflow-dialect.md) — that resembles it but is a compose-time halt directive, the wrong mechanism here; and no finalize confirm-gate exists to reuse — the "opt-in confirm-gate cascade setting" referenced in older notes is unbuilt). Concretely: on a **migration task**, `jigc task finalize <id>` *without* `--approve` renders the **fidelity diff** and **exits non-zero, committing nothing**; `jigc task finalize <id> --approve` proceeds through the retire + commit + adopt. The diff compares **{the staged source-seam bytes}** (the foreign original) against **{the task working-area rendered doc}** (`.jigc/tasks/<id>/docs/changelog:changelog.md`), both read **pre-retire**; the `--approve` re-invocation re-reads both (the retire executes only within the approved run). On rejection (the human simply does not re-run with `--approve`, or `task discard`s) the foreign original is left **untouched** and nothing is adopted. This is the milestone's net-new human-in-the-loop surface and the **only** check on rewrite fidelity — the strict parse guarantees *structure*, never *content-faithfulness*.

### Retire-the-foreign-original (the first destructive write)

The foreign filename (`CHANGELOG.md`) ≠ the managed singleton slug (`changelog/changelog.md`), so the managed write lands *beside* the original; if the foreign file is left, the reconcile sweep routes it `needs-reconcile`/advisory **forever**. **Settled: a CLI-owned retire step folded into the finalize transaction** removes the foreign original — but **only after the review gate's approval**. This is jigc's **first byte-destructive operation on a repo file** (the `unmanage`/`uninstall` teardown verbs deliberately leave bytes on disk). It is scoped to migration, human-authorized through the review gate, and transactional with the managed write + adopt: `--approve` → {write canonical doc · retire foreign original · commit · adopt} as one finalize; reject → nothing changes. The agent never removes the file (that would be the boundary-forbidden direct placement); the CLI owns the deletion.

**The transaction mechanism (so the increment doesn't guess).** The retire rides the finalize plan as a new field (`FinalizePlan.retirements: Vec<PathBuf>`, `crates/engine/src/finalize.rs`), executed **inside the commit closure** (`crates/cli/src/task.rs` → `try_execute_finalize_plan`) so the deletion is staged by the existing `git add --all` and lands in the same commit as the promoted doc. The existing rollback (`rollback_promotions`) covers only promoted paths — it **must be extended to restore each retired path** (`git restore --staged --worktree <path>`) so that a commit failure *after* the retire (e.g. a rejecting pre-commit hook) leaves the foreign file intact, not deleted-with-no-commit. Atomicity = {promote + retire + commit} all-or-nothing, the same discipline promote already has.

**Path-collision guard (the in-location squatter).** A foreign file already at the canonical managed path (a non-conformant `changelog/changelog.md`, the `needs-reconcile` case [project-setup.md](project-setup.md) → Flow 2 hardening names `migrate` as the corrective for) is **not** a separate original to retire — the managed write *is* the in-place rewrite. The guard: when the resolved foreign source path **equals** the canonical managed path, **skip the retire** and adopt in place; otherwise (the common `CHANGELOG.md` ≠ `changelog/changelog.md` case) retire the distinct original. So `migrate` handles both the root foreign file and the in-location squatter with one mechanism.

### Adopt (the existing path)

Once the canonical doc is written + the foreign original retired, finalize adopts via the unchanged register-only path (parse → conformance-gate → `index.absorb_doc` → file-state hash). Confirmed at planning: the previously-Unmanaged state of the target path does not block the fresh conformant write (no `DRIFTED+TOUCHED` conflict — the managed write lands at the lowercase singleton path, distinct from the foreign sibling, which the retire step removes).

## Engine / validation work this milestone earns

1. **Enforce the enum on `id-from` fields.** Conformance today exempts the `id-from` field at every level (`crates/engine/src/validate.rs` → `check_item_leaves`), and the changelog change-group's `category` (the enum) *is* its `id-from` — so a foreign category (`Performance`, `Breaking Changes`) is currently adopted as "conformant" with a value the schema's own enum forbids. This hollows out the "adopted iff conformant" guarantee the whole migration leans on. **The fix is a *new* code path, not un-exempting `check_item_field`:** the id-from value is **not** in `item.fields` — it lives in the parsed item **heading** (`ParsedItem.title`/`.id`), so `check_item_field` (which looks the value up in `item.fields`) would wrongly emit `required-field-present`. The new path reads the id-from value off the heading and, when that field declares an `enum`, checks membership by **re-slugging** the heading value and comparing against the (slug-form) enum members — so a foreign `Fixed`/`Added` normalizes to the valid member `fixed`/`added` and **passes** (the same re-slug discipline the multi-word-section-id parser fix uses), while a genuinely foreign `Performance` → `performance` ∉ enum is **rejected**, forcing the LLM to map it. (The check thus guarantees *membership of the normalized value*, not *semantic faithfulness* — a foreign category that coincidentally slugs to a valid member is the human review gate's residue, not the enum's.) **Scope:** the shared `schema_conformance` path (the gate `migrate`'s strict-parse and `finalize` both call) — additive; an `id-from` field with a **non-`enum`** type stays exempt (every other shipped doctype's `id-from` is a `title`/`key` string — verified at planning — so none newly fails). **Build-pin:** the exact finding code (expected `schema-conformance.field-value-conformant`) and the leaf-suffix of the emitted address are pinned at the build spike against the real binary, and flow 25's RED 1 updated to match.
2. **The source seam** (above) — the CLI-owned foreign-content-into-context surface.
3. **The `migrate` verb + the retire step** — the new CLI surface + the destructive-write-in-finalize-transaction.
4. **The review-gate halt** — the foreign-vs-canonical diff + the approval block (a `checkpoint`-class halt specialized to a fidelity diff).

**Slug collision — accept-and-block (no new work).** `slugify` drops dots, so `1.2.0` and `1.20` both mint `120`; `add-item` hard-rejects the second with `write.already-present` (it does **not** suffix — `suffixed()` is the join-time assigner's, never the mint site's). **Settled: accept-and-block** — a foreign changelog whose versions collide under slugify blocks loudly and routes to the human, rather than silently fabricating a fake version id. Rare, loud, and consistent with the review-gate posture. No engine change; recorded as a bound.

**Byte-stability finding — deferred (off the critical path).** The logged empty-slot + field-group render instability ([decisions-pending.md](../implementation/decisions-pending.md)) lives only in items carrying *both* a slot and a field group. The changelog's items are slot-only (change-group `notes`) or field-only (release `date`/`link`) — never both — so changelog migration is **clear** of it. Decompose must confirm the migration author-sequence never re-parses a minted-empty slotted item (it won't — changelog has no such item). The fix stays its own focused milestone (M24, which migrates the methodology slot+field doctypes, is its natural trigger).

## Migration-quality measure (the done-bar)

The milestone's goal is the Flow-B live test, so its done-bar is a **measured migration-quality baseline** (the M17 discipline: "a capability installs" ≠ "a comparative win"):

- **Corpus:** 3–5 real public `CHANGELOG.md` files of varying conformance (clean Keep-a-Changelog, a loosely-structured one, a non-KaC-categories one, a multi-release one).
- **Metrics:** (a) **round-trip conformance** — does the rewrite parse + conformance-gate + adopt? (b) **human-acceptance of fidelity** — the review gate's verdict (did the rewrite faithfully preserve the content?); (c) **content-preservation spot-check** — no silent data loss beyond the accepted-drops bound above.

The measure is recorded alongside the run (the [measurement.md](measurement.md) posture), not engine-emitted.

## Acceptance — worked-examples flow 25

[worked-examples.md](worked-examples.md) → flow 25 is the acceptance bar: a real non-conformant foreign `CHANGELOG.md` (today `Unmanaged`) → `jigc migrate CHANGELOG.md --as changelog` mints the off-router task and surfaces the foreign content → the LLM rewrites it through the write verbs → the review gate shows the fidelity diff → **approve** → finalize writes `changelog/changelog.md`, retires the foreign original, adopts (the managed doc round-trips byte-stable; the foreign file is gone; a follow-up `ingest` reports it adoptable/adopted); plus the reds — a non-conformant rewrite (a foreign category outside the enum) **blocks** at conformance, and a **human-reject** of the diff leaves the foreign original untouched and adopts nothing.

## Open questions

- **The source-seam placeholder syntax** — the exact spelling of the foreign-content placeholder (a `{{source}}`-class read-only context surface) is an elaboration for the build; it must stay syntactically distinct from a managed-doc deref (`{{@…}}`) since it carries non-managed bytes.
- **Multi-doctype migration in one pass** (M24) — a real project has a changelog *and* ADRs *and* a PRD; whether `migrate` generalizes to a repo sweep or stays one-file-one-doctype is M24's fork.
- **Code-inferred docs** (M24+, the north star) — inferring an `arch-doc` from the codebase or ADRs from scattered notes is the destination, not this milestone.
