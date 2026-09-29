# M44 planning gate-record — the rc.8 wave (the pull tier)

Filled by hand at planning (2026-07-21), the by-hand form of the pack-encoded `planning-record` presence gate. One row per scope item; every cell carries evidence or `N/A` + a one-line why. An empty/hand-wavy cell is a halt, not a pass. Grounded in the four-auditor + four-detector baseline (real binary at `3b41824`), four robust-advocate cases, and the independent pre-decompose review (2 blocking + 4 should-fix findings, all baked — [DECISIONS.md](../../../DECISIONS.md) → 2026-07-21 M44 planning: the Settle → Review).

## Milestone-level gates

| Gate | Verdict + evidence |
|------|--------------------|
| **strategic-claim-fresh** (Scope) | ✅ The claim — *make capabilities behave like gates; the model is reliable at push, unreliable at pull* — is fresh, from the rc.7 rerun (2026-07-20, [findings-verification.md](../RC-adoption/rerun-rc7/findings-verification.md)); the discoverability lens has landed four trials running and this run named the mechanism. Not read off a stale doc. |
| **cheap-vs-robust** | ✅ All four cheap-vs-robust forks resolved robust, **each argued by an independent robust-advocate with spike evidence** (task-id · adr authoring · route split · preview). The robust arm was in every case the minimal-*correct* cut (path-hash already ships as the empty-slug fallback; adr workflow is migrate-adr-minus-source, no schema change; route enrichment mirrors `store.rs:134`; preview is `compose_core`-minus-mint). The proposer never self-framed a cheap recommendation. |
| **foreclosed-by-doc** | ✅ Hunted. Two surfaced, neither blocks: the route **wire-argv** promotion is an already-tracked deferral ([decisions-pending.md](../../../implementation/decisions-pending.md):174), so M44's prose-tier route fixes don't engage it; the fidelity "advisory stays advisory" rests on a *parked idea* ([migration-content-coverage.md](../../../ideas/migration-content-coverage.md)), revisable but consciously kept out. `surface-contract.md:8`'s disown of the AGENT.md body handled at Review S3 (cross-reference, not restate). No doc forecloses a better path. |
| **census** | ✅ **No frozen-schema bump hides in the wave** (doctypes gap-detector headline — every change is CLI/adapter/pack-prose or an additive read key; no doctype `schema-hash` moves). The print-surface census was completed at M43 round-3 (2026-07-18, ~460 rows). |
| **acceptance-spiked** | ✅ The riskiest flow-45 assumptions spiked against the real binary: compose-without-mint composes byte-identical (review N3 — `compose_core` consumes only id+intent, empty roles); `doc show <section> --task <id>` reveals item ids (review N2, [doc-read-surface.md](../../../design/doc-read-surface.md):11/74/78); the empty-slug blake3 fallback exists (`start.rs:232`, auditor). The one flow that failed its spike — the path-hash surviving re-slugify — was caught at Review (B1) and its acceptance re-specified (distinct long-slug paths → distinct ids). |
| **value-flow-exercised** | ✅ Each change closes a *verified trial reach*: the AGENT.md paragraph/read-rule + preview kill the stale-source reach (D1/D2/D3/D4); the route split closes the C2 dead-end; the adr workflow + rider close the live placeholder-source loophole (C4); the task-id fix closes the C1/L1 stopper. The pull-tier value flow (agent meets a gap → the capability is now preloaded or surfaced at relevance) is traced per item below. |

## Per-scope-item gates

Columns: **PA** = prior-art-reconciled (every `design/` doc touching it agrees) · **DC** = design-complete (behaviour + finding-id/severity/scope + acceptance concrete) · **RE** = reuse-exercised (the reused path spiked, not analogized) · **CS** = check-scope-pinned (what surface a new finding fires against) · **IS** = integration-seam (the file/seam it lands at, named).

### 1 · Task-id path-hash derivation (fork 1) — Inc 1
- **PA** ✅ [storage.md](../../../design/storage.md):74/82/91 quoted into the Settle (slugify one-way door applies only to editing `slugify`; the fold in `start.rs` is outside `SLUG_RULE_VERSION`, `slug.rs:406`). No doc contradiction.
- **DC** ✅ Behaviour: append `blake3(repo-relative-path)` disambiguator, deterministic (resume-preserving) + collision-free. **B1-corrected**: hash must survive re-slugify's 5-word/50-char caps (hash-first or `slug_override` bypass, (I) at pickup). Acceptance: two distinct *long-slug* colliding paths → distinct ids; same path → same id (resume). No finding-code (it's a derivation change, not a check).
- **RE** ✅ The mechanism ships as the empty-slug fallback (`start.rs:232-233`, `file_state::hash_bytes`, blake3); auditor + reviewer traced the cap interaction.
- **CS** N/A — a derivation change, not a check that fires against a surface.
- **IS** ✅ `crates/cli/src/start.rs` `migration_task_id_source` (+ possibly `mint_task` `slug_override`, `state.rs:486`). Downstream: preserves `re_invoking_migrate…routes_to_the_resume` (migrate_adr.rs); fixes `milestone-record.yaml:53` `id-from: task-id` item collision.

### 2 · `write.not-present` route enrichment (the 6th fork) — Inc 2
- **PA** ✅ [validation.md](../../../design/validation.md):60 (the `write.*` route prior-art home — owed the split) + [surface-contract.md](../../../design/surface-contract.md):13/26/68 (law 2, placeholder-substitution fence) + [doc-read-surface.md](../../../design/doc-read-surface.md):11/74/78 (`doc show --task` reveals item ids). All agree.
- **DC** ✅ Behaviour: split `not-present` out of the shape-question arm (`write.rs:5071`); CLI-layer enrichment substitutes the real containing-section address + resolved `task.id`. The three shape-codes keep the generic `doc schema` placeholder. Acceptance: a `not-present` finding routes to a followable `doc show <type>:<slug>#<section> --task <id>` that reveals the real item ids. **N2-pinned**: strip to the top showable section, never a field-leaf.
- **RE** ✅ Precedent `store.rs:129-140` (inline CLI `Route::mechanical` with the real address) spiked; the M43 route parse-fence passes the argv (dummy table has `<address>`/`<task-id>`).
- **CS** ✅ Fires against a task-scope write (`set-slot`/`set-field`) at the ~4-5 dispatch sites (`doc.rs:794/800/808/1119/1130`).
- **IS** ✅ `crates/cli/src/doc.rs` (the `block()` enrichment seam `:3608`; task.id in scope at every dispatch site); `crates/engine/src/write.rs:5064` (arm split).

### 3 · Workflow preview (fork 3) — Inc 3
- **PA** ✅ [surface-contract.md](../../../design/surface-contract.md) law 2 (nothing hides) + [introspection.md](../../../design/introspection.md)/[doc-read-surface.md](../../../design/doc-read-surface.md) (the read-surface spine). `describe` is menu-only by M11 design (agrees — this is the fifth read surface, not a contradiction).
- **DC** ✅ Behaviour: `jigc workflow <id> --preview` composes a `creates-task: true` workflow's step text WITHOUT minting (synthetic valid slug through `build_context`, branch around `mint_in_repo`/`provision_commit_doc`). Render convention: mark it a preview; render `--task <your-task-id>` as an identity (never a fictional real id — law 1) + a mint-first banner (law 3). Acceptance: `--preview` composes step text mutation-free (no task dir written), the render carries no copy-pasteable-into-error `--task`.
- **RE** ✅ Review N3 confirmed byte-identity: `compose_core` (`start.rs:1037-1056`) consumes only id-string + intent + def + catalog/store, binds empty roles; staged-doc reads only on the resume arm. Reuse target is `compose_core`, **not** `--explain` (which returns a tree).
- **CS** N/A — a read verb, emits no finding.
- **IS** ✅ `crates/cli/src/{start.rs (compose_core), cli.rs (Command::Workflow — add --preview), render.rs (preview-safe --task)}`.

### 4 · AGENT.md machine-output paragraph (change 1) + 5 · read-rule amendment (change 2) — Inc 4 (S4: one increment)
- **PA** ✅ [assistant-adapter.md](../../../design/assistant-adapter.md):48 owns the bootstrap body; [surface-contract.md](../../../design/surface-contract.md):8 disowns it (S3: preload tier joins as a cross-reference only). [command-output-contract.md](../../../design/command-output-contract.md) §1 is the machine-contract source the paragraph must not contradict (owed: name `migrate` — item 13).
- **DC** ✅ Change 1: a 5th hand-written paragraph, **scoped to successful/validation-outcome invocations** (N4/baseline: clap parse errors don't honor `--format json`); the `.task` producer set = `start`/`workflow`/`migrate`. Change 2: derive jigc behavior from the installed binary, worded to reconcile with the existing "read source freely" clause (project source stays free; jigc *behavior* from the binary, not a checked-out jigc/pack tree). Acceptance: a generated AGENT.md carries both, and the whole-body golden re-pins once.
- **RE** ✅ Auditor: `bootstrap_file()` (`adapter.rs:464`) is four hand-written consts under one whole-body `insta` golden (`adapter.rs:1400`); a 5th paragraph is a const + one golden update. `--format` is a global arg (structurally every verb accepts it).
- **CS** N/A — bootstrap prose, not a check. Law-1 fenced by the whole-body golden + a lightweight assert that `--format` is a global arg.
- **IS** ✅ `crates/cli/src/adapter.rs` (the const + `bootstrap_file` + the golden); `design/assistant-adapter.md:48` enumeration.

### 6 · adr from-knowledge authoring workflow (fork 4) — Inc 5
- **PA** ✅ Precedents `plan.yaml`/`architecture-documentation.yaml` (from-knowledge, plain `finalize`) agree. **S1-corrected**: plain `finalize`, NOT `migration-finalize` (else the composed output lies about a review-hold + retire).
- **DC** ✅ Behaviour: a new selectable dev-pack workflow (`allows-create: [{type: adr, as: decision}]`) = new `author-adr` step (foreign-mapping/transcribe-date/supersedes-from-foreign prose removed; on-create date stamp kept) + plain `finalize`. `when:` on the decision axis (surface-contract style guide). Acceptance: `--workflow record-decision` (or chosen id) composes from-knowledge, authors an adr with a fresh on-create date, finalizes with no review-hold. No schema change (auditor + review: adr's required author-fills are `context`/`decision`/`consequences` slots).
- **RE** ✅ Reviewer traced `migration-finalize.yaml` vs `finalize.yaml`; `task.rs:876` confirms exit-4/retire is task-state-driven (no source seam → plain-finalize behavior).
- **CS** N/A — a workflow, not a check.
- **IS** ✅ `crates/cli/pack/workflows/<new>.yaml` + `crates/cli/pack/steps/author-adr.yaml`; a flow-45 arm.

### 7 · byte-floor triviality-guard rider (fork 4 rider) — Inc 5
- **PA** ✅ M23 migration-quality posture (round-trip/fidelity/preservation) + M24 (the fidelity summary is a labeled advisory, not a gate) — an advisory guard is consistent. [auto-migration.md](../../../design/auto-migration.md) Framing-A (no CLI content judgment) holds — a byte-length threshold is deterministic, no content judgment.
- **DC** ✅ **S2-pinned**: an **advisory** (not a refuse), firing on the migrate source at mint, below a small byte floor, with its own finding-code + route (author it from knowledge instead); the from-knowledge adr path (no source) naturally exempt. Acceptance: a 1-byte migrate source emits the advisory; a terse-but-valid source does not.
- **RE** ✅ Auditor: no byte-floor exists today (1-byte `p` loophole reproduces live); would live at the foreign-bytes read (`migrate.rs:~180-214`).
- **CS** ✅ Fires against the migrate source at mint (a deterministic byte count), advisory scope.
- **IS** ✅ `crates/cli/src/migrate.rs` (foreign-bytes read, before compose).

### 8 · The `{{schema:<singleton>}}`-derived stated-at fence + migrate-step statements (D5) — Inc 6
- **PA** ✅ [surface-contract.md](../../../design/surface-contract.md):51 (the stated-at fence — owed a per-soliciting-step extension) + the A-3 presence-never-content bound. **B2-corrected**: owe-set from the `{{schema:<T>}}` body signal (single source of truth), the `authors-into:` marker dropped as circular/drift-prone.
- **DC** ✅ Behaviour: pack-load asserts that any step whose body references `{{schema:<T>}}` with `T` a create-or-update singleton carries the copy-in/append constraint declaration; the 7 migrate author-steps gain the statement. Acceptance: a `{{schema:<singleton>}}`-bearing step lacking the declaration reddens pack-load (mutated shipped-tree arm, the `stated_at_fence.rs` mold).
- **RE** ✅ Doctypes + docs detectors + review confirmed the M43 fence is existence-per-code (`pack.rs:358-419`), so this is a **new fence shape** (not a reused code addition) — flagged, not analogized.
- **CS** ✅ Fires at pack-load (per shipped origin pack), presence-only (A-3).
- **IS** ✅ `crates/cli/src/pack.rs` (the sweep); the 7 `author-migration-*.yaml` steps in both packs; a mutated-pack fence arm.

### 9 · Fidelity boundary-guard (a) + 10 · report-split (b) — Inc 7
- **PA** ✅ [auto-migration.md](../../../design/auto-migration.md):91 (the scan calibration home — owed the entry) + Framing-A (advisory, no gate). Consistent — a calibration fix, not a basis change.
- **DC** ✅ (a): reject a dotted-numeric run whose preceding byte is alphanumeric/`-` (kills `project-alpha-2.0`→`2.0`) — a local edit. (b): split "package@version absent" vs "version-like token absent" — **a new extraction pass** (the scan carries zero package context), knowingly included. Advisory/no-gate unchanged. Acceptance: `project-alpha-2.0` no longer flags `2.0`; the report separates the two tiers.
- **RE** ✅ Auditor + review: `scan_version_tokens` (`render.rs:2196`) is advisory, feeds no gate; (a) LOW, (b) new-extraction — costed separately.
- **CS** ✅ Advisory review render (human/agent arm; the JSON review arm carries no scan — one-line ack of the asymmetry).
- **IS** ✅ `crates/cli/src/render.rs` (`scan_version_tokens`, `dropped_release_versions`, `migration_review`).

### 11 · Item-count projection rider — Inc 7
- **PA** ✅ [doc-read-surface.md](../../../design/doc-read-surface.md):80/109 — **N1-corrected**: `doc show`/`doc list` evolve by additive-keys-pre-1.0, an **additive key, not a contract-version bump** (the version integer is `doc schema`'s). Agrees.
- **DC** ✅ Behaviour: a count key on `doc show`/`doc list` over already-parsed structure. Acceptance: the count appears additively; existing consumers unaffected. Noted: the item *array* is already pinned, so this is a convenience/discoverability aid (included per the human's inclusive call).
- **RE** ✅ Auditor: the count is `len()` of a value already parsed (`reqs.items.len()` computed at `doc.rs:3825`).
- **CS** N/A — a read projection, no finding.
- **IS** ✅ `crates/cli/src/doc.rs` (the read projection).

### 12 · Ingest JSON summary block + 13 · command-output-contract §1 — Inc 7
- **PA** ✅ [command-output-contract.md](../../../design/command-output-contract.md) §1 owed (name `migrate` as an id-carrying producer — coupled to change 1's AGENT.md producer set). Ingest: pure render.
- **DC** ✅ Ingest summary: additive verdict-counts + per-directory block on the ingest `--format json` (`ingest.rs:89` — bump `serialize_struct` to 2). §1: a doc edit (the guarantee holds today by code-path inheritance — `migrate` composes through the id-carrying `compose_core`). Acceptance: ingest JSON carries a `summary`; §1 names three producers.
- **RE** ✅ Auditor: `IngestReport::serialize` is one `rows` field today (`ingest.rs:81-97`); additive.
- **CS** N/A — render + doc.
- **IS** ✅ `crates/cli/src/{ingest.rs, render.rs}`; `design/command-output-contract.md`.

### 14 · Flow 45 + docs fold-back — Inc 8
- **PA** ✅ [worked-examples.md](../../../design/worked-examples.md) (the flow home, flow-44 harness mold) + MIGRATING.md (owed: bulk same-dir migration parallelisable) + CLAUDE.md/VISION fold-back.
- **DC** ✅ Behaviour: `crates/cli/tests/flow45_acceptance.rs` — done-picture arms through the real binary (distinct long-slug migration ids · a followable `not-present` route · `--preview` composes mutation-free · the AGENT.md machine-output paragraph + read rule present · a from-knowledge adr with a fresh date · a `{{schema:singleton}}`-step's copy-in statement · `project-alpha-2.0` not flagged). Acceptance: the suite green through the real binary; the pack-load fence proven by a mutated-pack arm.
- **RE** ✅ The flow-44 acceptance harness (`CARGO_BIN_EXE_jigc` in throwaway `jigc setup` repos) is the proven mold.
- **CS** N/A — acceptance + prose.
- **IS** ✅ `crates/cli/tests/flow45_acceptance.rs`; `design/worked-examples.md`; MIGRATING.md; CLAUDE.md/VISION.md.

**No empty or hand-wavy cells.** The two `spike-failed` risks (B1 path-hash cap, B2 fence circularity) were caught at Review and their designs corrected before this record was finalized; every acceptance is concrete and every reuse is spiked, not analogized.
