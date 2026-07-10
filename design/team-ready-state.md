# Team-ready state — the milestone record as a managed doctype

**M39 design of record.** The pre-1.0 team-readiness slice: work-unit state a teammate (or a fresh agent session) continues from moves *out* of the gitignored workbench and becomes a committed, managed, legible doctype. Settled at M39 planning ([DECISIONS.md](../DECISIONS.md) → 2026-07-06 M39 planning: Settle + review-baked). Reads after [methodology-docs.md](methodology-docs.md) (the work-doc-doctype surface it extends) and against [storage.md](storage.md) (the layout + source-of-truth model it revises). The exhaustive fold-back into the reading-order docs is an M39 build increment (the M38 Inc-6 pattern), not restated here.

## The principle — `.jigc/` is the workbench, the repo is the record

The classification axis is **working vs record**, and it assigns each side a *home*, not just a `.gitignore` line ([ideas/team-ready-state-externalization.md](../ideas/team-ready-state-externalization.md), the settled direction):

- **The record lives OUTSIDE `.jigc/`**, at a legible committed home — reconciled, `jigc doc show`-able, migratable, uniform with every other managed artifact.
- **`.jigc/` is a pure working directory** — WIP staging, per-machine caches, worktrees. Gitignored as a matter of **identity**, not convenience.

Today (verified at HEAD `f372b2b`) the milestone's authoritative state is raw engine JSON in gitignored `.jigc/milestones/<id>/`: `base.json` (`{sha, short}` base-pin, read by `engine::milestone.rs::read_base_pin`) + `tasks.json` (`{tasks: [<sub-task-id>, …]}` ordered list, `read_task_list`), with per-task `intent`/`status` in `.jigc/tasks/<id>/`. A teammate cloning the repo cannot see or continue in-flight milestone work — the genuinely-lost-on-clone set (`index`/`state` re-derive on a stateless clone; the milestone record does not).

## What graduates, and what stays WIP — the split is field-granular

The record/working line is drawn at **field granularity**, not by directory (a directory-level "all task state is WIP" re-strands the very continuation state this milestone targets):

| State | Home | Role |
|---|---|---|
| Milestone base-SHA pin | milestone record (doctype) | ✅ **source of truth** |
| Ordered sub-task list + each sub-task's `intent` + `status` | milestone record (doctype) | ✅ **source of truth** |
| `.jigc/milestones/<id>/{base,tasks}.json` | `.jigc/` | **rebuildable cache** (re-derived from the record) |
| Sub-task working areas (`docs/` staging, provenance) | `.jigc/tasks/<id>/` | gitignored WIP |
| Fan-out merge staging (`merged/`), worktrees | `.jigc/milestones/<id>/` | gitignored WIP |
| Derived caches (`index/`, `state/`), logs | `.jigc/` | gitignored (re-derived) |

**Scope:** the **milestone** record graduates; a standalone (non-milestone) single-task's in-flight state stays WIP (its durable value is already captured in the finalize-rendered commit / decisions-log / changelog; parked for validation in [ideas/task-record-graduation.md](../ideas/task-record-graduation.md)).

The one code split this forces: `engine::milestone.rs`'s `milestone_dir` roots **both** the record and the WIP staging under one `.jigc/milestones/<id>/`. M39 splits *record-home* (committed, outside `.jigc/`) from *WIP-home* (gitignored, inside `.jigc/`); every `<jigc_root>/milestones/<id>/` consumer learns the two-home layout. The `.gitignore` string — today a hardcoded literal duplicated across **three** divergent sites (`adapter.rs`, `task.rs`, `milestone.rs`, already drifted on `worktrees/`) — collapses to one source of truth and changes meaning (ignore WIP subpaths; the record home is committed).

## The `milestone-record` doctype

Named **`milestone-record`** (mirroring `completion-record`, which deliberately avoids reusing the reserved work-unit name) — **not** `milestone`, because the work-unit already occupies the `milestone:<id>` address (`engine::milestone.rs` emits `Location::addressed("milestone:{id}")`) and `doctype-map.md:51` locks "`task`/`increment`/`milestone` are not doctypes." The doctype governs the *state a milestone emits*, not its *identity* — the `completion-record` precedent (milestone-*close* state as a doctype) exactly. Addressed `milestone-record:<slug>`, where **the slug IS the milestone work-unit id** (both are the same kebab slug), so `milestone-record:cache-rework` names the record for work-unit `milestone:cache-rework` and `doc show` can locate the matching `.jigc/milestones/cache-rework/` WIP.

- **Methodology-pack**, multi-instance. **Freeze-exempt at M39** (no `schema-manifest.yaml` entry, no version-gate); **manifest-governed since M40** — it joined the methodology pack's own manifest at schema-version 1, stamp-injected + freeze-asserted + migrate-corpus-owned ([corpus-migration.md](corpus-migration.md) → the M40 revision; [doctype-map.md](../implementation/doctype-map.md) → the freeze scope pin).
- **Home:** `location: milestone-records/` — **flat**, consistent with its sibling per-milestone record `completion-record` (`location: completions/`); under the composed `[dev ▸ methodology]` cascade the dev pack's `docs-root` applies (→ `docs/milestone-records/`), exactly as `completions/` → `docs/completions/`. It uses `location:` (composition-variant), **not** `placement:` (composition-invariant) — so unlike `VISION.md`/`CHANGELOG.md` its resolved path differs methodology-alone vs. co-composed; acceptable, the acceptance runs `[dev ▸ methodology]`.
- **Header fields:** `base` — the base-SHA pin, materialized `set: on-create`; `status` — active / joined, materialized on transition.
- **A repeatable `tasks` section**, one item per sub-task: `task-id` (the item `id-from`, a plain **string** — not a managed `ref`: a `task` is a work-unit, `ref-resolves` cannot walk it, the `deferral-ledger.trigger` precedent) + `intent` + a `status` field, both CLI-materialized.

It carries **no LLM-authored prose slot** — purely machine-maintained structured state (the "human surface" fork was settled *pure machine-maintained*, not an authored goal slot). This is the shape the managed-doctype vocabulary does not express today (every existing leaf is an LLM-filled `slot` or an author-required `field`; the only CLI-`set` is `set: on-create` on `date`). It earns **two** engine capabilities (below); everything else is assembled from proven primitives — M16 repeatable-section + `add-item`, M22 on-create materialization + the clock-free `on_create` seam, M16 `singleton`-slug discipline, M38 `location` homing.

### Engine capability 1 (write) — `set: on-transition` machine-maintained leaves

Extend the M22 CLI-side materialization seam from **on-create** to **on-transition**: a leaf tagged `set: on-transition` is (re)materialized by the CLI at a milestone state change. **The engine stays clock-free and LLM-free** — the CLI supplies the values from work-unit state it already owns, exactly as it supplies the fan-out join message ("structure the CLI owns, like git's auto-generated merge message"). The determinism boundary is intact: a *structural* write (which items exist, their machine-set fields), never prose. Two distinct write arms, designed separately (the review's F5 census):

- **`add-task` — append.** Appends one `tasks` item (`id`/`intent`/`status: active`) via the M16 `add-item` primitive. Ordered, byte-stable.
- **`join` — in-place mutate.** Rewrites the machine-set `status` field of **every already-committed `tasks` item** (active → joined) and the header `status`. This is an in-place item-leaf rewrite of N existing items — a *different* operation from append, routed through the byte-stable item-leaf splice path (the `set_item_field` splice, but writing the **committed record directly**, outside any task working area — net-new plumbing vs. the task-scoped author path). Byte-stability of the in-place rewrite is a red acceptance obligation (create → add-task ×2 → join → both items' `status` flip, doc byte-identical modulo the two flipped values).

**No-silent-overwrite discipline (the review's F3 fix).** Because every field is CLI-owned, an OOB human edit to a machine-set field cannot be *merged* — but it must not be silently *clobbered* either. Each milestone-op write runs a **reconcile preflight** (`reconcile_committed_store` over the record) and **conflict-blocks on drift** before any `set: on-transition` write, routing the human to reconcile via the CLI. The record is thus *detected + conflict-blocked*, **not** "hand-editable and merged like an authored doc" — the honest framing: it is a CLI-maintained structural record whose OOB drift is caught, not absorbed.

### Engine capability 2 (read-back) — the committed record is the engine's authority

The committed `.md` record is the **source of truth**; `.jigc/milestones/<id>/{base,tasks}.json` is demoted to a **rebuildable cache**. `engine::milestone.rs`'s operational reads (`read_base_pin`/`read_task_list`, and `join`/`add-task`) resolve through the cache, and the cache is **re-derived from the committed record when absent or stale** — so on a **fresh clone** (no `.jigc/` working state) the first milestone op parses `docs/milestone-records/<id>.md` back into `BasePin` + `TaskList` and re-seeds the cache. This is what makes the [storage.md](storage.md) invariant — "delete `.jigc`, **rebuild from the `.md`s**, nothing lost" — *genuinely* true for milestone state (F1): the record is not merely committed, it is the rebuildable operational source.

**"Continue" means resume, not WIP recovery.** A teammate on a fresh clone re-derives the milestone's shape (base, task list, intents, statuses) from the record and **resumes un-joined tasks from scratch** — the existing stateless restart-from-scratch resumption policy (the fan-out resumption model, [decisions-pending.md](../implementation/decisions-pending.md)). Joined tasks are done; active tasks re-run from their recorded `intent`. Half-done sub-task WIP (gitignored) is *not* recovered — correct and consistent with the resumption policy.

**Stale-base edge (history rewrite).** The record content survives a rebase/squash/filter as ordinary tracked content (jigc re-derives from the *current* file, never from the record's own commit history). The one genuine fragility is the base-SHA pin naming a commit a rewrite orphaned — a property of *any* sha pin (today's `base.json` included, unchanged by this move). Handled by **graceful stale-base detection → route to the human** (re-pin or error), never silent corruption. Bound: the live in-flight window is short, mid-milestone rewrite is unusual; degrade cleanly, revisit if a real case appears.

### Why this preserves, not breaks, the source-of-truth invariant

[storage.md](storage.md) states: "the committed Markdown at type locations is the **only source of truth**; every other artifact is a rebuildable cache or a transient working copy." With capability 2, the milestone record *joins* the `.md` set as a first-class member and the `.jigc` JSON becomes exactly a *rebuildable cache* — so the invariant's wording is revised in the **strengthening** direction (a new `.md` member), not relaxed. A committed *non-doctype* record (raw JSON at a visible path) would instead be a *second, non-rebuildable source of truth* — reintroducing the dual-source-of-truth the caches are gitignored to avoid. That is the decider recorded at Settle: (c) is the only option that keeps the model uniform *and* keeps the record readable through `jigc doc show` — the read surface M39 itself builds.

## The commit model — path-scoped commit at each milestone op

The record is written **and committed at each structural milestone op**, so a teammate cloning at any moment sees true current state (settled at planning). The commit is **path-scoped** (the review's F4 fix): it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP — honoring the M30/M31 staging discipline ([storage.md](storage.md) → dirty-tree policy). Arm by arm:

- **`create` / `add-task`** — a **separate** record-only path-scoped commit each.
- **`join`** — already a finalize/commit boundary (the fan-out finalize commits code + docs); the record's `status`-flip write **folds into the existing join commit** (path-added alongside the promoted docs), not a second commit.

This does **not** breach the finalize-transaction invariant, which governs the **doc-content integrity gate** (required-slot presence, forward-ref resolution) — the record carries no authored prose to gate; it is a *structural* artifact the CLI maintains, the same class as the code commit the CLI already makes at finalize.

**The finalize base-guard refinement (settled 2026-07-07, mid-build — [DECISIONS.md](../DECISIONS.md) → 2026-07-07 M39 build).** Per-op record commits advance HEAD, so the pinned milestone base no longer equals HEAD at `finalize`; and `record.base` is structurally always ≥1 commit behind HEAD (a record commit cannot store its own resulting sha), so the old `plan_milestone_finalize` preflight `base.sha == head_sha` (the M31 worktree-combine external-drift guard) can *never* pass in this model — on a fresh clone too. The guard is **refined, not removed**: proceed if `base == HEAD` (fast path), **or** if `base` is a linear ancestor of HEAD *and every commit in `base..HEAD` touches only this milestone's record path* (`docs/milestone-records/<id>.md`) — then advance `base → HEAD` and proceed; **any non-record commit in the range still blocks** (`base-mismatch`). This makes the guard catch what it was *for* — **external** drift, which would invalidate the worktree-combine — while tolerating the milestone's own record-only bookkeeping (which touches no code, so the combine base is unaffected). The **engine does no git I/O**: the CLI computes the range's linearity + record-only-ness and feeds the result to `plan_milestone_finalize`. (Owed to [finalize.md](finalize.md)/[storage.md](storage.md) at the Inc-8 reading-order fold-back.)

## The read surface — `jigc doc show` + a pinned `--format json` shape

The milestone record is the first doctype whose *primary* consumer is a **machine** read (fresh-clone continuation), so its `jigc doc show --format json` output is a **1.0 stable contract** (a one-way door — pinned now, not left to the first implementation, the review's F6 fix): whole-doc = `{ type, slug, fields: { base, status, schema-version }, sections: { tasks: [ { task-id, intent, status }, … ] } }` (`schema-version` is the M40 additive-pre-pin key — the methodology stamp, added under [doc-read-surface.md](doc-read-surface.md)'s declared evolution posture), where `base` is a **structured object** `{ sha, short }` (the compound base pin — clean lossless machine access to both SHAs, resolving the Inc-3 audit advisory that flagged the space-joined scalar; the committed `.md` still stores `base: <sha> <short>` as a scalar line, so this is json-projection-only — [DECISIONS.md](../DECISIONS.md) → 2026-07-07) and every other field is a scalar; a `#tasks` slice returns the item array; the `status` enum serializes as its lowercase string. Pinned in the doc-read-surface design; the milestone record is its conformance witness.

## Interactions

- **`jigc doc show`** reads the record like any managed doctype — the uniformity dividend; the reason (c) beats a raw-JSON island.
- **Reconciliation** ([reconciliation.md](reconciliation.md)) — an OOB edit is detected + **conflict-blocked** (not merged; §capability 1).
- **The relocation floor** (M39) — the record rode the freeze-exempt detect+route floor *at M39* (no version-gate); **since M40** it is a methodology-manifest member, so a home change version-gates like any manifest member and `jigc relocate` refuses it ([corpus-migration.md](corpus-migration.md) → the M40 revision).
- **No managed edge** — `milestone-record → task`/`roadmap-entry` are not minted (task-ids stay strings; the M16 "no per-entry managed ref until a driver earns it" bound holds).

## Deferred / flagged (logged, not forgotten)

- **Standalone task-record** — parked, validate-first ([ideas/task-record-graduation.md](../ideas/task-record-graduation.md)).
- **Planning-gate-record reopen** — if a planning workflow is (re-)encoded to *read* the record, the condition-keyed planning-gate-record deferral may fire; flagged, not triggered by M39.
- **Config-layer / per-developer `local` distribution** stays deferred (that is *config* distribution; this is *state* legibility). The record's stored shape must not pre-decide it.
- **Concurrent-OOB-during-a-fan-out** over the committed mutating record inherits the existing concurrent-OOB deferral bound ([decisions-pending.md](../implementation/decisions-pending.md)) — not a new promise.

## Acceptance (the M39 decomposition owns the increment cut)

On a `[dev ▸ methodology]` repo: `jigc milestone create` → `add-task` ×2 → the committed `docs/milestone-records/<id>.md` carries the base pin + both sub-tasks' `id`/`intent`/`status: active` as a repeatable section, each op a path-scoped record-only commit that leaves unrelated staged/untracked WIP untouched; `join` flips both items' + the header `status` to joined **byte-stable** (folded into the join commit); a **fresh clone** (delete `.jigc/`) re-derives the cache from the record and `jigc doc show milestone-record:<id> --format json` returns the pinned shape; an OOB edit to a `status` field is **conflict-blocked** at the next op (not silently overwritten); a stale base-SHA (rewritten-away pin) routes to the human, not corruption.
