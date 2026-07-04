# Reconciliation

How the CLI handles edits to managed documents that happen **outside** its write path — humans editing files directly, `git checkout` switching branches, `git pull` bringing in upstream changes. Reconciliation is the deterministic state machine that classifies what changed and routes it: **absorb** cleanly, **conformance-block**, or **conflict-block**. Never silent.

Builds on [VISION.md](../VISION.md) principle #3 ("files are truth"; out-of-band edits are detected and reconciled, never forbidden), [write-commands.md](write-commands.md) (the OOB philosophy; staging in `.jigc/tasks/<id>/`), [validation.md](validation.md) (the `file-state` engine-native probe whose output this consumes; severity is tunable per its [Severity inventory](validation.md#severity-inventory)), [storage.md](storage.md) (`.jigc/state/` hash records, the rebuildable edge index), [finalize.md](finalize.md) (the commit phase that updates hashes), and [implementation/parsing.md](../implementation/parsing.md) (the canonical-Markdown parse the classifier calls). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## The principle

**Files are truth, so a clean external edit is honored — not merely tolerated** ([VISION.md](../VISION.md) principle #3). The CLI never forbids file edits and never silently discards them. Reconciliation is what turns "the file on disk differs from what the CLI last saw" into one of three outcomes deterministically.

## The state machine

Two axes per managed doc instance.

**Committed-state axis** — the on-disk file relative to the recorded `file-state` hash ([validation.md](validation.md)):

- **`UNKNOWN`** — no recorded hash (fresh checkout, new repo, or first-encountered doc)
- **`IN_SYNC`** — file hash matches recorded hash
- **`DRIFTED`** — file hash differs from recorded hash

**Task-state axis** — the same doc relative to the active task's working area:

- **`UNTOUCHED`** — task has not written to this doc
- **`TOUCHED`** — task has staged at least one write to this doc

The classifier reads the pair and routes:

| committed | task | classifier output |
|---|---|---|
| `UNKNOWN` | (any) | **baseline-adopt** |
| `IN_SYNC` | `UNTOUCHED` | **clean** (no-op) |
| `IN_SYNC` | `TOUCHED` | **task-only change** (proceed normally) |
| `DRIFTED` | `UNTOUCHED` | **OOB edit** — see parse classifier below |
| `DRIFTED` | `TOUCHED` | **conflict** — block at file level |

### Baseline adoption

When the CLI first encounters a managed doc with no recorded hash (`UNKNOWN`), it adopts the current on-disk content as the baseline: hash is computed and recorded; state becomes `IN_SYNC`; the agent sees "baseline adopted: `<doc>`" as an informational line. **Absent-hash is not drift** — a fresh checkout or new repo is the normal first-encounter case.

### OOB edit — the parse classifier

For `DRIFTED + UNTOUCHED`, the CLI re-parses the file against its type's schema ([document-type-schema.md](document-type-schema.md), [implementation/parsing.md](../implementation/parsing.md)):

- **Parse succeeds + schema validates** → **absorb**:
  - re-hash and record the new hash (the `IN_SYNC` baseline shifts forward),
  - update the edge index for any cross-ref changes the edit introduced,
  - surface "external edit absorbed: `<doc>`" to the agent.

  The state returns to `IN_SYNC`. The CLI now treats the on-disk content as the source of truth, which it always was.

  **Persistence of the shifted baseline (pinned at M17 planning, 2026-06-12).** Every sweep absorbs *in memory* for its own run, but the **durable** file-state is persisted **only by a landed `finalize`** — the post-sweep state written in the same motion as the landing commit's re-hash. `validate`, `start`, and read-path sweeps stay pure readers (no write side effects on a read verb). Until M17 this persistence never happened anywhere, so an absorbed edit re-fired its advisory in every later task — the verified over-count defect the M17 measurement substrate fixes ([measurement.md](measurement.md) → The capture substrate). Within-task re-fires before the next landed finalize remain by design; they collapse to one finalize-time emission, which is the counted event.

- **Parse fails OR schema fails** → **conformance-block**:
  - the state stays `DRIFTED`,
  - the agent (and any active task touching this doc) is blocked,
  - the CLI surfaces a precise conformance error — file, line, expected shape (e.g., "missing `{#id}` anchor on item heading `### Rate limit holds`" or "field `date` value `2026/13/01` not a valid date").

  Resolution is human-side: fix the file, or revert the edit. The CLI never auto-repairs in the MVP (see [Auto-repair scope](#auto-repair-scope)).

**Root / placement docs reconcile identically (M38).** A **placement** doctype ([storage.md](storage.md) → Placement) is managed at a literal home — a repo-root file (`VISION.md`, `CHANGELOG.md`) or a direct `docs/roadmap.md` — with `location: None`. Its instance is keyed at that **exact literal path** (fixed slug = type id), and the sweep visits it there, so an out-of-band edit to a managed root file runs the *same* state machine as any managed doc: a conformant edit **absorbs**, a non-conformant one **conformance-blocks / routes** (`needs-reconcile`). Ownership is **exact-path equality against `placement.file`** — every *other* root `.md` (`README.md`, `CLAUDE.md`) is outside the store and untouched (a literal home is not a dir-glob). The M38 sweep learned this placement branch (previously the `location: None` loop skipped placement docs, silently missing their drift; [DECISIONS.md](../DECISIONS.md) → 2026-07-04 M38 Increment 2).

### Conflict — block at file level

For `DRIFTED + TOUCHED`, both sides have moved: the on-disk file changed since the last recorded hash, **and** the task's working area has staged writes to the same doc. The MVP blocks at file granularity:

```
conflict on adr:rate-limit-at-the-gateway:
  external edit since 2026-05-28T14:03:00Z + task add-rate-limiter has staged changes.
  resolution:
    - jigc task discard-write adr:rate-limit-at-the-gateway   # drop the task's changes
    - revert the file on disk                                  # drop the human's edit
```

Resolution paths:

- Agent discards the task's writes for this doc (explicit op).
- Human reverts the on-disk edit; the task's writes are kept.
- **Three-way merge** of both sides is deferred — it parallels override-conflict resolution ([overrides.md](overrides.md)) and rides on the same future machinery.

**Discard is always explicit, never the default, never silent** — no data loss.

## Rename detection

Files-as-identity means a **path change is an identity change** ([storage.md](storage.md) → Identity, order, fields, slots, items). A human running `git mv decisions/rate-limit.md decisions/gateway-rate-limit.md` isn't editing the file's *content* — they're (effectively) renaming the artifact. The hash-on-known-path state machine above doesn't catch this case: it sees the old path as *missing* and the new path as *new untracked*.

So rename detection is a separate classifier, running at the same trigger points as the state machine ([Detection timing](#detection-timing)). Two signals, two outcomes:

- **Strong signal — suspected rename.** A tracked path is missing **and** an untracked path has the **same content hash** as the last recorded hash for the missing one. The CLI surfaces a **blocking** finding naming the detected rename and the resolution — routing to the owned op first, revert second:
  ```
  error: tracked managed doc adr:rate-limit (decisions/rate-limit.md) is missing
    hint: a file at decisions/gateway-rate-limit.md has the same content hash —
          likely renamed via a bare `git mv`.
    resolution:
      adopt it as a CLI-owned rename (re-points every referrer atomically):
        $ jigc rename adr:rate-limit --to "Gateway rate limit"
      or revert the out-of-band move:
        $ git mv decisions/gateway-rate-limit.md decisions/rate-limit.md
  ```
- **Weak signal — tracked path missing, no content-matching new file.** The tracked file is simply gone (deleted, accidentally removed). The CLI surfaces a conformance error: *"tracked managed doc adr:rate-limit (decisions/rate-limit.md) is missing — restore the file, or run `jigc delete adr:rate-limit` to confirm deletion (post-MVP)."* Routes to restore.

**Policy: an out-of-band move is blocked and routed to the owned op; the CLI never auto-honors an untracked rename.** A path/slug rename is an identity change, and identity changes are CLI-orchestrated through **`jigc rename`** ([write-commands.md](write-commands.md) → `jigc rename`) — the prevention happy-path that re-keys `file-state` and repoints every referrer atomically. A bare `git mv` is the *out-of-band* case: detected and routed ("adopt as `jigc rename`" / revert), **never** silently absorbed and never auto-rewritten. Same shape as the rest of strict scope ([Auto-repair scope](#auto-repair-scope)): the finding names exactly what happened; the agent runs the owned op (or reverts). `jigc delete` (deletion confirmation) stays post-MVP.

**Amended at M17 planning (2026-06-12) — "missing" means absent from disk, not absent from the walk.** The implementation inferred *missing* from a baselined path not appearing in the sweep's walked set — but the walk is a per-schema, non-recursive `<location>/*.md` glob, so a baselined path outside every walked location (the case that fired: a promoted **owner-artifact** under `completions/artifacts/<run>/`, baselined at its finalize) was declared missing on every subsequent sweep, false-blocking every later `finalize` in the repo via `reconciliation.rename`. The fix (the dogfood pre-fix set, [DECISIONS.md](../DECISIONS.md) 2026-06-12): before classifying a tracked path as missing, **check disk presence** — present-on-disk-but-outside-the-walk is *not* missing (no finding; content stays baselined as-is). Chosen over excluding `completions/artifacts/**` from the baseline, which would blind the sweep to a genuinely deleted artifact dangling behind a committed `completion-record`'s `owner-artifact` field. The strong/weak rename signals above are unchanged — they now simply fire only on genuine disk absence.

**`jigc rename` is a single atomic transaction with its own rollback boundary — *not* `finalize`'s** (M35). `finalize` promotes a task working area by **copy** (rollback = delete the copy); `jigc rename` mutates **committed paths + file-state + the edge index directly**, so it owns a distinct transaction: capture pre-image bytes of the old path + every referrer to be rewritten → rewrite each referrer's ref-field → `git mv` → stage → run the user's git hooks (never `--no-verify`) → commit; on **any** failure restore all captured bytes and reverse the `git mv` → then re-baseline `file-state` (`forget(old)` + `record(new)` + re-record each rewritten referrer) and invalidate/rebuild the index. The migrate-corpus *no-rollback / idempotent-rerun* model is **rejected** for rename: rename is **not idempotent-safe** (its re-run keys on `<old>`, which the `git mv` already destroyed, so a half-done rename can't resume — it would leave the exact dangling-ref violation rename exists to prevent). Full spec: [write-commands.md](write-commands.md) → `jigc rename`.

**Out-of-band rename — store-scope detection is the backstop, and it blocks (M35, the A+B decision).** `jigc rename` is the prevention; the store-wide sweep is the safety net under it for the case where an agent does a bare `git mv` and commits *without* the verb. Two parts:

- **A — wire the strong rename classifier into store scope, firing first.** The strong signal (missing tracked path + content-hash match) is the *only* detector that catches a **referrer-less** OOB move (no dangling ref to find), and it carries the actionable "adopt as `jigc rename`" route. It runs in the store sweep **before** the store-wide `ref-resolves` family ([validation.md](validation.md) → forward-ref integrity), so a moved-with-referrers doc is diagnosed as *a rename to adopt/revert* rather than as N independent dangling refs — the two never emit competing diagnoses for one event.
- **B — the OOB-rename finding is blocking / exit-flipping, and the pre-commit hook blocks the commit.** This is a narrow, deliberate amendment to the store-scope **report-only** stance and the **warn-only** pre-commit hook ([validation.md](validation.md) → Exit semantics; → Auto-firing the sweep). *Engaging that rationale (record-is-rebuttable):* the report-only/warn-only stance exists so the sweep never gates on **pre-existing, unrelated content rot** (the masking-trap — a commit blocked by drift it didn't cause). An OOB rename is **not** pre-existing content drift — it is a **structural-identity corruption introduced *by this commit***, so blocking it is *consistent with* that rationale, not a violation. It joins the `pack-probe-integrity.*` exit-flipping exception class (a trust/integrity event, not a tunable content opinion); general content `ref-resolves` stays report-only. The hook blocks only on the rename detected among the files **this commit** moves, never on pre-existing store drift, so the masking-trap guard is preserved.

## Detection timing

The `file-state` probe fires at six points; reconciliation runs the classifier at each:

| trigger | scope | purpose |
|---|---|---|
| **task `start`** | every managed doc the workflow will read or write | baseline-adopt unknowns, absorb clean drift, block before the task does any work |
| **read through the CLI** | the doc being read | absorbs benign drift transparently; the agent sees current truth |
| **write through the CLI** | the target doc | re-probe before staging; block if the doc has become conflicted since the task started |
| **`jigc task validate`** | every doc in the task's scope (working area + referenced docs) | full sweep — same sweep `finalize` runs |
| **`jigc task finalize` preflight** | same as validate | `finalize ≡ validate + commit` ([finalize.md](finalize.md)) |
| **`jigc validate` (ad-hoc)** | scope-flexible (doc / store) | manual check; no task context required |

Probing is cheap (one hash per doc); the classifier and the parse only run when state is `DRIFTED`.

## Hash re-baselining

The recorded `file-state` hash updates at exactly three sites — keyed to "last-known-good committed state":

- **Baseline adoption** — first encounter (`UNKNOWN`). Hash = current on-disk content's hash.
- **Absorb** — clean external edit accepted. Hash = new on-disk content's hash.
- **Commit** — `finalize` phase 7 ([finalize.md](finalize.md)). Hash = just-committed content's hash for every managed doc the commit touched.

Task writes do **not** update the committed-state hash. The working area is separate; the hash tracks "what the committed file looked like the last time we agreed with it." Until `finalize`, the task's writes live in the working area and the committed-state hash stays pinned.

## Auto-repair scope

The MVP does **not** auto-repair conformance gaps, even ones that look mechanically obvious:

- **Dropped `{#id}` anchor** — even when the heading text still matches the recorded id-source field value, the MVP blocks and asks the human to paste the anchor back. The id is frozen and recoverable from the recorded state, but silently reconstructing identity is precisely the kind of "the CLI just does things" behavior the determinism boundary is meant to prevent.
- **Field value re-formatting** (`2026/05/28` → `2026-05-28`) — block; the human picks the canonical form.
- **Trailing whitespace, line endings, CRLF vs LF** — caught at parse time; whether the parser ignores these silently or surfaces them as conformance findings is a parser-implementation question ([implementation/parsing.md](../implementation/parsing.md) → open questions).

The rule: **the conformance error names exactly what's missing; the human fixes it.** Auto-repair (anchor re-mint, whitespace normalize, date canonicalize) is a post-MVP feature that requires its own decision per category. Strict MVP keeps the boundary visible and the failure modes audible. Easier to relax later than to tighten.

## MVP scope vs post-MVP

**MVP ships:**

- Detection — the `file-state` probe ([CLAUDE.md](../CLAUDE.md) → MVP scope).
- The full state-machine classifier above.
- Baseline adoption on first encounter.
- Clean-absorb path (parse + schema-validate → re-hash + edge-index update).
- Conformance-block with precise error surfacing.
- File-level conflict-block with explicit-discard resolution.
- Hash re-baselining at the three named sites.
- **Rename detection** — strong signal (path missing + content-hash match) and weak signal (path missing alone), at task scope (blocking) and **store scope** (M35: blocking, firing before `ref-resolves`), routed to "adopt as `jigc rename`" / revert per [Rename detection](#rename-detection).

**Post-MVP (deferred):**

- **Auto-repair categories** — anchor re-mint, whitespace normalization, date canonicalization. Each is a separate per-category decision; none ships until demand is concrete.
- **Section/leaf-level conflict granularity** — partial absorption when OOB-edit and task-touch don't overlap section-by-section. Pairs with three-way merge.
- **Three-way merge for both-sides-changed conflicts** — currently deferred ([overrides.md](overrides.md); shared machinery with override-conflict resolution).
- **`jigc import` for entirely new untracked files** — adopting a brand-new `.md` file authored outside the CLI ([implementation/parsing.md](../implementation/parsing.md) → "mint-on-import lands with the full import flow"). MVP imports only edits to *managed* docs the CLI already knows about.
- **`jigc delete`** — confirms a deletion (removes the file-state record, surfaces dangling referrer refs). MVP detects OOB deletions and routes to restore; the op ships post-MVP ([write-commands.md](write-commands.md) → `jigc delete`). *(`jigc rename` shipped at M35 — the [Rename detection](#rename-detection) backstop now routes to it; see [write-commands.md](write-commands.md) → `jigc rename`.)*

## What reconciliation does NOT do

- **No silent discard, ever.** Every block surfaces; every absorb surfaces.
- **No three-way merge.** Both-sides-changed blocks, never auto-resolves.
- **No file forbidding.** The CLI never tries to prevent edits — it detects and reconciles.
- **No identity reconstruction.** A dropped `{#id}` blocks; the CLI doesn't guess the id from context, even when it could.
- **No silent rename.** A path change is an identity change ([Rename detection](#rename-detection)); the CLI never auto-rewrites referrer refs to follow a *bare* `git mv`. The owned op **`jigc rename`** (M35) does it as one explicit, atomic transaction; an out-of-band move is detected and **blocked** (store scope), routed to adopt-or-revert — never silently followed.
- **No LLM call.** The classifier is deterministic — same `(file content + recorded hash + task working area + schema)` in → same classification out ([VISION.md](../VISION.md) principle #1).

## Open questions

- **Parser-tolerant vs parser-strict for cosmetic drift** — trailing whitespace, line endings, CRLF vs LF. Which categories pass parse silently vs surface as conformance findings is parser-implementation territory ([implementation/parsing.md](../implementation/parsing.md)) and not yet decided.
- **Concurrent OOB edits during a task** — if the human edits a file while the task is writing to its working area, the write-through-CLI probe catches it (timing row 3 above), but the window *between* two writes is unprotected. The file-level conflict-block fires at the next probe; only missed if the edit is reverted before the next probe (in which case there's nothing to reconcile).
- **External edit notification surface** — "external edit absorbed: `<doc>`" is the agent-facing line; how the assistant adapter surfaces this (inline in workflow output? separate notification?) is part of the broader blocked/error-payload question ([write-commands.md](write-commands.md#open-questions)).
