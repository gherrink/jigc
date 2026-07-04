# On-disk storage

How managed content lives on disk: the **document file format** and the **repository layout** around it. One format serves committed documents, the staging working copy, and the future fillable form.

Builds on [structural-grammar.md](structural-grammar.md) (addressing, units, minting), [document-type-schema.md](document-type-schema.md) (slot/field/relation), and [write-commands.md](write-commands.md) (staging, `finalize`, reconciliation). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## Source of truth

**The committed Markdown at type locations is the only source of truth.** Every other artifact — the staging working area, the edge index, the file↔state hashes — is either a rebuildable cache or a transient working copy. Delete all of it, rebuild from the `.md`, and nothing committed is lost; the system degrades cleanly to *plain Markdown a human can read and edit*. This is what keeps "documents live in the repo as plain files" honest.

## The document file format

### Single-file canonical Markdown

The `.md` *is* the document — not a view rendered from a hidden model file. A dual scheme (canonical model + regenerated Markdown) would make the file humans actually edit second-class, contradicting our locked principles (plain files are the source; humans edit through git; out-of-band edits are detected and reconciled). The cost — robust parsing — is paid by the format below.

### Schema-driven parse

The CLI parses an instance **against its type's schema**, never as blind Markdown. The schema already knows every section (id, heading, order) and every leaf (which slots, which fields), so the file needs **no markers for any of that** — sections match by their schema-fixed headings, slots are the prose in their schema position, fields are the labeled values the schema tells the parser to find. The only thing the schema does not know — and the only thing the file marks — is **instance-minted identity**.

### Anatomy

Three parts, and only the first and third carry any "machine" content:

1. **Front-matter** — the document's *header block* fields (status, dates, doc-level refs, and the **schema-version stamp**), as `key: value`. A **flat field block** (a strict YAML *subset*, not an arbitrary YAML document — values are typed by the schema, not coerced by YAML); a list-valued field uses **inline flow** (`relates-to: [adr:a, adr:b]`). Doc-level refs here keep the graph scannable without parsing the body. The **schema-version stamp** (M34) is a **schema-declared, engine-valued** front-matter field on every persisted doctype recording the schema version it was authored against (the `status`/`date` model — in-schema, engine-set) — so the engine emits a `---` block even for doctypes that declare no other `header:` section ([corpus-migration.md](corpus-migration.md)).
2. **Body** — schema-fixed section headings; under each, the slot prose plus (if any) a trailing field group rendered as a **bullet list** (`- key: value`). The field *grammar* is the same as front-matter; the bullet is the body's structural frame (the counterpart to front-matter's `---` fences) that keeps fields distinct from the opaque prose around them.
3. **`{#id}` item anchors** — the *only* in-body marker, on each repeatable item.

An ADR (header-heavy, all-prose sections):

```markdown
---
status: accepted
date: 2026-05-23
supersedes: adr:single-node-cache
---

# Rate-limit at the gateway

## Context
Per-client limits were enforced ad hoc in each service.

## Decision
Centralize rate limiting at the gateway.
```

A SPEC (a repeatable `criteria` section with per-item structure and a local field):

```markdown
## Acceptance criteria

### Rate limit holds at 100/min  {#rate-limit}
The gateway rejects the 101st request in a 60s window.

<!-- fields -->
- maps-to-test: `test/rate_limit_spec.rb#burst`

### Burst allowance  {#burst-allowance}
A short burst above the limit is tolerated for 2s.
```

### Identity, order, fields, slots, items

- **Identity is the path** — `docs/specs/auth-flow.md` → type `spec`, id `auth-flow`. No redundant id in the file. An **ordinary retitle** (editing the H1) changes the title field with the id held frozen (see next bullet) — identity-stable. A **path/slug rename** *is* an **identity change** (the path *is* the identity), allowed **only** through one explicit, atomic CLI op — **`jigc rename <old-slug> --to "<New Title>"`** (top-level): re-slug from the new title, rewrite the H1, repoint every structured referrer old→new, `git mv`, one transaction ([write-commands.md](write-commands.md) → `jigc rename`). An **out-of-band** `git mv` is *not* an identity change the CLI honors — it's detected and routed ("adopt as `jigc rename`" / revert), never an untracked move ([reconciliation.md](reconciliation.md) → Rename detection). The surgical line is **not** "ids never change" but "ids change only through one owned, atomic operation"; **reference-integrity** survives the change (referrers repointed in lockstep) even though identity-stability does not — and diff-legibility *improves*, a slug tracking its title beating one frozen out of sync.
- **id-source = the title field, rendered as the heading** — the schema declares a `string` field as the item's id-source (`id-from: title`, see [document-type-schema.md](document-type-schema.md)); that field is rendered on disk as the item's `###` heading text (not as a trailing `- key: value` field), with the minted `{#id}` anchor carrying the frozen id slugged from the field's value at creation. A doc's title is its H1 (filename = frozen id); an item's title is its `###` heading (`{#id}` = frozen id). The title is mutable (editing the heading edits the field's value); the id is frozen under an **ordinary retitle**, so `spec:auth-flow#criteria/rate-limit` survives editing the heading text. *(A deliberate doc-level **re-slug** — `jigc rename`, above — does change the doc's id, atomically and with every referrer repointed; that is an identity refactor, distinct from the identity-stable retitle here. Item-level ids are not yet renameable: `jigc rename` is doc-level in v1.)*
- **Order = physical order.** Reordering is moving a block — a clean diff move, never a renumber.
- **One field grammar** (`key: value`), two structural frames — header block → a flat front-matter block between `---`; a section/item block → a trailing **bullet list** (`- key: value`) preceded by an `<!-- fields -->` sentinel on its own line (the field-group boundary marker — see [implementation/parsing.md](../implementation/parsing.md) → Field-group delineation). The sentinel makes the field group unambiguous *by marker*, not by content-matching the bullet keys against the schema — so slot prose that legitimately ends with `- status: TBD` stays prose. `code-anchor` values carry presentational backticks (stripped on read, re-added on write) so the diff stays readable without polluting the stored value.
- **One prose slot per section preferred.** Multi-slot sections are allowed but render each slot under a schema-fixed sub-label (matched like a heading — no new marker), which nudges schemas toward one-purpose sections.
- **Slot prose has a heading-depth ceiling** — no ATX headings at `##` (section) or `###` (repeatable-item) depth, and no Setext underline-style headings, inside slot prose; `####` and deeper are allowed for slot-internal structure (see [implementation/parsing.md](../implementation/parsing.md) → Slot heading-depth ceiling). This keeps the CLI's structural depths unambiguous: `##` and `###` always mean "structural marker," never "prose that happens to look like one."
- **Items live inside their parent file** (the criteria-in-doc decision); no per-item files.

## Repository layout & state location

Committed source of truth and derived/transient state live apart:

```
docs/decisions/ docs/specs/ docs/prds/ docs/architecture/ docs/changelog/   # committed .md docs at per-doctype locations under docs-root — the only source of truth
.jigc/                      # one project-state home (committed config + bootstrap; gitignored caches)
  AGENT.md                  #   committed — the managed bootstrap routing sentence (CLI-owned; the host file references it)
  version                   #   committed — the binary-provenance stamp: which jigc built the store (recommended home; see Store provenance)
  config/                   #   committed — the project cascade layer (see Config layout)
    packs.yaml              #     pack composition (e.g. compose the embedded methodology pack)
    manifest.yaml           #     scalar-set knobs + the deltas list — created on the first `jigc config` delta
    steps/<id>.yaml         #     native step files for structural-op / tracked-fork deltas — created lazily
    fills/<id>.md           #     native content for slot-fill deltas — created lazily
  .gitignore                #   ignores the derived/transient subdirs below
  tasks/<task-id>/          #   gitignored — per-task working area (staging)
  index/                    #   gitignored — edge index (rebuildable cache)
  state/                    #   gitignored — file↔CLI-state hashes (rebuildable)
  milestones/               #   gitignored — milestone working state
  worktrees/<sub-task-id>/  #   gitignored — ephemeral per-sub-agent code worktrees (provisioned + torn down per fan-out)
```

**Per-doctype locations are human-chosen, not mechanical.** Each persisted doctype declares its own `location:` (`adr → decisions/`, `spec → specs/`, `prd → prds/`, `arch-doc → architecture/`, `changelog → changelog/`) — deliberately the most readable name for that doctype (`decisions/` over `adrs/`, `architecture/` over `arch-docs/`), **not** a uniform `<type>s/` rule. These nest under the **`docs-root`** parent (default `docs/` → `docs/decisions/`, `docs/specs/`, …; see [Config layout](#config-layout)) — one tidy home rather than five dirs polluting the repo root — committed, human-reviewable plain Markdown alongside the project's own `src/`. (A pre-existing same-named dir with non-conformant content is detected and routed by `ingest`, not silently overwritten.)

The three cascade layers ([overrides.md](overrides.md)) live in three homes: the **project** layer is **committed** in-repo at `.jigc/config/` (deltas + knobs, diff-reviewed); the **team** layer is **external** (`~/.config/jigc/`, same internal layout); **pack-default** ships with the installed pack (its knob surface in `config/knobs.yaml`). So `.jigc/` is one home: its `config/` subdir + the `AGENT.md` bootstrap are committed, while `tasks/`, `index/`, `state/`, `milestones/`, and `worktrees/` are gitignored (via `.jigc/.gitignore`) and fully disposable.

### Config layout

A cascade layer's `config/` dir has one fixed shape (settled M4 planning, [overrides.md](overrides.md) → Delta representation / Authoring deltas):

- **`manifest.yaml`** — the layer's deltas: a top-level `scalar:` map (`scalar-set` knobs, inline `key: value`) and a `deltas:` list (one entry per `structural-op` / `slot-fill` / `tracked-fork`, each referencing a native file, never inline prose). Every content-bearing delta records its `base-version` + the target's `base-hash` ([overrides.md](overrides.md) → Delta representation) for the **(M5)** reconciliation.
- **`steps/<id>.yaml`** — native step files in the **same form the pack's `steps/` ships** (a step body — verbatim instruction prose with `{{…}}` placeholders, optional `---`-fenced front-matter only for a `fan-out` marker; *not* a front-mattered workflow definition). Referenced by `insert`/`replace` structural-ops and `tracked-fork`; the id is the filename basename.
- **`fills/<id>.md`** — native Markdown content for `slot-fill` deltas (the body that fills a `{{fill:<id>}}` point).
- **`packs.yaml`** — pack-composition for the project (e.g. `compose-embedded-methodology: true`, composing the embedded methodology pack as `[dev ▸ methodology]`; [multi-pack.md](multi-pack.md)). Written by `jigc setup`; the only config-dir member present before any `jigc config` delta. *(`manifest.yaml`, `steps/`, `fills/` are this fixed shape but materialize lazily — only once a delta is authored.)*

The pack-default layer adds **`config/knobs.yaml`** — the closed, typed knob *declaration* (`{key, type, of?, default}`, [overrides.md](overrides.md) → Scalar knobs); override layers carry only *values* (`scalar:` in their manifest), never re-declare the surface. The loader reads these into the `PackDefaultLayer` / `OverrideLayer` the resolver consumes.

**`docs-root`** — the managed-doc parent dir (`string`, default `docs/`), one of the dev pack's knobs. The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load (`docs/decisions/`, `docs/specs/`, …), so the doctype id, addressing (`adr:foo`), and stable-id invariants are unchanged — only the on-disk path gains the prefix. Per-project overridable (`jigc config set docs-root <path>`); `.` (or `""`, which the config-set path canonicalizes to `.`) restores the flat repo-root layout. It is an **opt-in** knob: it nests only where the resolved cascade carries the key — a pack whose knob surface omits `docs-root` (e.g. the methodology pack) stays flat. Applied uniformly at **every** schema-load surface (compose/describe, finalize-promote, doc copy-in, ingest/unmanage, milestone join) so the read and write paths always resolve the same parent.

**Changing `docs-root` can orphan docs at the old root.** When `jigc config set docs-root` re-points the parent, committed managed docs already at the *prior* resolved root no longer resolve under any doctype location — they become **orphans**, detected store-side by the `file-state.orphaned-doc` advisory ([validation.md](validation.md) → Orphan detection). To catch it at the source, `jigc config set docs-root` **warns when the change would orphan committed docs** under the prior resolved root (enumerated via `git ls-files`, matched by the location-directory-basename predicate — [validation.md](validation.md) → Orphan detection, *not* a bare `.md` match), routing the operator to move them or re-point back before the knob lands. This is a **net-new seam**: `config set`'s `run_set` has no store access today, so the warning gives the config-set path its first read of the committed doc surface.

### Placement — direct-file and root-located homes (M38)

`location:` above resolves a doctype's home as **a directory** (`<docs-root>/<location>/<slug>.md`) — right for a **multi-instance** doctype (many `docs/specs/*.md`), wrong for a **singleton**, which lands one file alone in its own folder (`docs/changelog/changelog.md`, the one-file-in-a-folder anti-pattern). The **placement convention** (M38) fixes this and puts ecosystem-idiomatic singletons where an agent and a human look for them.

**The rule (binds every doctype):**
- **Singleton, ecosystem-idiomatic** → a **managed direct file at the repo root**: `VISION.md`, `CHANGELOG.md`, `README.md`. *Managed* (reconciliation applies — an out-of-band human edit is detected and routed, never silently overwritten), not a generated mirror.
- **Singleton, project-internal** → a **direct file** (no per-type folder): `docs/roadmap.md`, `docs/deferral-ledger.md`, `docs/decisions-log.md`.
- **Multi-instance** → a **folder** under `docs/` (unchanged): `docs/specs/`, `docs/decisions/`, `docs/research/`, `docs/ideas/`.
- **Killed:** every `docs/<x>/<x>.md` one-file-folder.

**The declaration — a structured `placement:` key** (schema-level, optional, skip-when-absent so the freeze hashes of doctypes that don't use it are unchanged; the additive-`Option`-with-serde-skip precedent of `display-title`/`root-render`, so absent ⇒ hash byte-identical, only `changelog` re-hashes to v2). Declared in [document-type-schema.md](document-type-schema.md) (the schema-format home); the key:

```yaml
placement: { file: VISION.md }        # repo-root-relative literal path, case-preserved
placement: { file: docs/roadmap.md }  # a direct file under docs/ (the docs/ is in the literal)
placement: { file: CHANGELOG.md }     # repo-root idiomatic singleton
# absent → today's folder behavior (location: decisions/ → docs/decisions/<slug>.md), byte-identical
```

`file:` is a **repo-root-relative literal path** (case-preserved, bypassing the lowercase-`[a-z0-9-]` slug), so `VISION.md`/`CHANGELOG.md`/`docs/roadmap.md` are each reachable exactly as written. **`docs-root` never applies to a placement doctype** — the home is the literal, so the resolved path is *composition-invariant* (`docs/roadmap.md` is the same under methodology-alone and `[dev ▸ methodology]`, fixing the flat-vs-`docs/` ambiguity that bit M37). This is the first per-doctype opt-out from `docs-root`'s uniform prepend; it honors the anti-root-pollution intent by scoping root homes to the handful of files humans already expect there (`VISION.md`/`CHANGELOG.md`). *(No `root:` flag — the literal path already encodes root-vs-under-`docs/`.)*

**Identity stays explicit, never filename-derived; and `location` stays `None`.** A placement doctype sets **no `location`** (`location: None`, like the transient `commit`) — the home comes from `placement.file`, not `<location>/<slug>.md`. Path→identity (`docs/specs/auth-flow.md → spec:auth-flow`) assumes a lowercase slug under a location dir; a literal uppercase root file would reverse-map `VISION.md → vision:VISION`, never round-tripping the lowercase forward path. So a `placement` doctype **carries its fixed slug** (the singleton's type id — address `vision:vision`, `changelog:changelog`, unchanged) and its case-preserved `file:` independently, and every path↔identity site matches **that exact literal path is this doctype's one instance**, never a dir-glob.

**The census — every `schema.location` consumer needs a placement branch (the decompose cut-list).** Because `location` is `None` for a placement doctype, the ~10 sites that read `schema.location` **each drop it through the transient arm** (never promoted / read / reconciled) unless taught the placement case — and the four flagged **⚠** are silent data-loss / broken-guarantee, not cosmetic:
- `store::canonical_path` **⚠** — else the doc is not store-readable (every `{{@…}}` ref into `vision`/`changelog` breaks).
- `finalize::plan_promotions` (single-task **and** milestone-join) — else the authored `VISION.md`/`CHANGELOG.md` is never promoted; the join path must compute the literal destination too (no separate guard needed once it does — the retired `reject_root_render_promotions` is *dropped*, not retargeted, since a correct `plan_promotions` cannot misplace).
- `file_state::reconcile_committed_store` **⚠** — else an OOB edit to `VISION.md`/`CHANGELOG.md` is never detected/routed, silently voiding the "*managed*, not a mirror" promise.
- `file_state::committed_path_recordable` **⚠**, `::persisted_committed_path`, `::identity_of`, `::untracked_committed`; `cli::unmanage::{identity_of, under_location}` (a **second** path→identity pair — do not miss it); `cli::rename::location_dir`.
- `index::rebuild_committed` **⚠** (the **10th site the M38 census missed** — folded back post-M38) — else a placement doctype's committed **`ref` edges are never indexed**, so `vision`'s `grounded-in → research` edge is dropped and `jigc rename research:X` repoints nothing on `vision` (violating reference-integrity: rename must repoint every structured referrer in lockstep). Its edge-index siblings share the same guard: `index::mention_resolves_store` (a placement doc's prose managed-mentions go un-checked) and `index::inverse_cardinality_store` (a placement type as a ref target with an inverse-card floor is never enumerated). All three route through one `index::committed_instances` placement-aware enumerator (the sibling of `store::canonical_path`), addressing a placement doc by its fixed `<type>:<type>` singleton slug at the literal `placement.file`.
Ownership at each is **exact-path equality against `placement.file`** — `VISION.md` is the managed vision, every other root `.md` (`README.md`, `CLAUDE.md`) stays unmanaged (rebutting B-1: a literal file is not a glob). The **no-silent-data-loss guard is inherited, not rebuilt**: the general `plan_clobber_guard` already blocks any create-promote whose destination file already exists, so a hand-authored foreign `VISION.md` is detected + routed at first `finalize` — M37's bespoke root-render foreign-file guard retires as redundant.

This **retires M37's `root-render` mechanism**: the vision is *managed directly* at root `VISION.md` (one file, proper reconciliation), not a `docs/vision/vision.md` source mirrored to root ([design-altitude-doctypes.md](design-altitude-doctypes.md) §4 is superseded).

**Adoption interaction — the changelog's foreign-source path collapses onto its managed home.** [auto-migration.md](auto-migration.md) architected changelog *adoption* on managed=`changelog/changelog.md` vs foreign=root `CHANGELOG.md` (a non-canonical path the migrate verb adopts then **retires** the original). Post-M38, managed **is** root `CHANGELOG.md`, so an existing project's foreign root `CHANGELOG.md` is now at the *canonical destination* — the M24 **in-location-squatter** case (`source-path == promote-destination` → seed/adopt in place, **skip the retire**), not the retire case. M38 must verify M24's squatter path covers "adopt a foreign root `CHANGELOG.md` into the managed root `CHANGELOG.md`" and re-point [auto-migration.md](auto-migration.md) (its "foreign root CHANGELOG = Unmanaged, retire the original" lines are false post-M38). This lands in the **existing-project adoption RC trial** (every existing repo has a root `CHANGELOG.md`), so it is M38 acceptance, not a hypothetical.

**Freeze + relocation.** A doctype's `location`/`placement` is inside its frozen `schema-hash`, so relocating a **frozen** doctype trips the version gate: `changelog` moves `docs/changelog/changelog.md` → root `CHANGELOG.md` (and `# changelog` → `# Changelog`) via a **`v1→v2` migration** that moves its own committed instances — the file-move + display-title transform kinds, [corpus-migration.md](corpus-migration.md) → Relocation. Freeze-exempt doctypes (the methodology singletons) relocate by a bare schema edit — **greenfield-clean**, but changing a home **silently strands** a pre-existing committed instance at the old path (no version, no trigger, no recorded prior location). M38 ships this honestly scoped — it fixes *homes*; a general **detect+route** floor for freeze-exempt existing instances (which also closes the standing `docs-root`-repoint orphan above — same shape) is **owed at M39** ([decisions-pending.md](../implementation/decisions-pending.md)).

### Store provenance — the binary-provenance stamp (M36)

A committed record of **which `jigc` binary produced this store**, so an adopter on a different machine is told when the store was written by a different build than the one now reading it. This hardens "same resolved cascade in → same workflow out" *across machines*: two developers on divergent binaries get a surfaced advisory rather than silent behavioural drift.

**Home — a dedicated committed file under `.jigc/`, never `.jigc/state/`.** The cross-machine claim *requires the record travel with the repo*, so it must be **committed** — the gitignored `.jigc/state/` (file↔CLI-state hashes) would never reach a clone. The recommended home is a new committed **`.jigc/version`** (a one-line record, e.g. `jigc-version: <semver>`; the exact filename is an increment detail). It is **not** a new field in `.jigc/config/packs.yaml`: the cascade loader treats each config file as a closed surface and rejects unknown keys ([overrides.md](overrides.md)), so a field there risks a load-time rejection — a dedicated file is the safer shape and keeps config (cascade *input*) and provenance (a written *fact about* the store) one-file-one-purpose. The value is the engine's `CARGO_PKG_VERSION` at write time; `jigc setup` writes it and store-writing ops refresh it.

**The check — a store-scope advisory.** The store-scope `jigc validate` sweep compares the recorded stamp to the running binary's `CARGO_PKG_VERSION` and, on mismatch, emits **`store-version.binary-mismatch`** (advisory severity, store scope, report-only — an un-keyed advisory, [validation.md](validation.md) → un-keyed findings). It **never** gates a transaction or `finalize` — honoring the report-only store-scope invariant ([validation.md](validation.md) → Exit semantics). The route is informational ("store last written by jigc `<x>`; you are running `<y>`" — align versions / re-run `jigc setup`); a version delta is a *heads-up* that two builds may resolve the cascade differently, not by itself corruption.

**Three senses of "stamp" — keep them distinct.** [corpus-migration.md](corpus-migration.md) already separates two: the per-doc **schema-version stamp** (an in-doc front-matter field recording the doctype schema version a doc was authored against) and the edge-index **cache-stamp** (the rebuildable-cache HEAD-sha freshness marker, [Derived caches](#derived-caches) below). This adds a **third** — the **binary-provenance stamp**, a *committed* store-level fact recording which `jigc` built the store. It sits on a distinct axis from both: neither per-doc nor a rebuildable cache.

### The per-task working area (staging)

A task's staging area is a **gitignored scratch dir of working copies**, in the same Markdown format. It realizes the locked concurrency primitive directly: fan-out sub-agents each write their **managed docs** to their own `tasks/<task-id>/` subdir (isolation by directory), and the CLI merges at the join **by task-id order** ([structural-grammar.md](structural-grammar.md#repetition), [VISION.md](../VISION.md) → Parallelism). This managed-doc recombination is *not* git worktrees and is never git-merged — git's text-merge is not our deterministic by-task-id join. (Fan-out **code** *does* isolate by worktree, recombined by deterministic disjoint-apply — see [CLI and git](#cli-and-git) below; the **docs** always stay on this directory-isolated by-task-id join.)

A task is **pinned to its base** (the commit it started against). Operating a task whose base ≠ the current checkout is detected and routed (*"started on `<base>`; you're on `<other>` — switch back or discard"*), the same divergence philosophy as out-of-band detection. *Rebasing* a task onto a new base is deferred (parallels three-way merge).

Only `finalize` produces committed changes; the working area itself is never committed (it persists on disk across sessions, so work resumes, but is lost on a clean/clone — acceptable for staging).

**The working area stages managed docs; the git index stages the code.** This scratch dir is the *managed-doc* staging surface — its working copies promote at finalize. A task's **code** is staged on a separate surface, the **git index**: under the agent-stage contract the agent `git add`s its own code edits as it works, and per-task `finalize` commits that index together with the docs it promotes ([finalize.md](finalize.md) → Dirty-tree policy). So a per-task commit set has two stages with two owners — the agent's index (code, plus any agent-written owner-artifact) and jigc's promotion (docs + first-commit config) — and finalize unions exactly those, never sweeping unstaged/untracked working-tree WIP. (The milestone fan-out combines each sub-agent's **worktree-isolated** code — never a whole-tree sweep of the shared checkout — and joins the managed docs by task-id; see the [three combine modes](#cli-and-git) below and [finalize.md](finalize.md#fan-out-finalize) → fan-out finalize.)

### Derived caches

The edge index and the file↔state hashes are **rebuildable from the committed docs**, so they are gitignored, never committed: committing them would churn diffs *and* reintroduce the dual-source-of-truth we eliminated with derived inverses. Each cache is **stamped with the HEAD (or a doc-set fingerprint) it was built against**; a branch switch, pull, or rebase changes the docs underneath it, so on a stamp mismatch the cache rebuilds. (The hash baseline's first-run edge case is noted in [write-commands.md](write-commands.md) → reconciliation.)

### Edge index lifecycle

The edge index ([document-type-schema.md](document-type-schema.md) → Bidirectional, but the inverse is derived) is a derived map of every cross-reference edge across the committed store. Its lifecycle has **five sites**, all deterministic:

| site | action | persistence |
|---|---|---|
| **committed rebuild** | first read after a stamp mismatch (branch switch, pull, rebase) — rebuild from committed `.md`s | atomic; persists in `.jigc/index/` with the new stamp |
| **working overlay** | `validate(task)` overlays the task's pending writes on the committed index for the scope of the run ([validation.md](validation.md) → Scope = effective state) | derived per-call, never persisted |
| **OOB absorb** | reconciliation's clean-absorb path incrementally updates the committed index for the absorbed doc's edges ([reconciliation.md](reconciliation.md) → Parse classifier) | atomic, in-place |
| **fan-out join** | at `join`, sub-task working overlays merge into the parent's working overlay by task-id order ([workflow-dialect.md](workflow-dialect.md#fan-out--join)) | derived, in-memory, never persisted |
| **finalize commit** | phase 7 invalidates the stamp; next read rebuilds against the new HEAD ([finalize.md](finalize.md)) | best-effort; cache-stamp absorbs failure |

One rule keeps the model honest: **the index is never persisted with task deltas in it.** Task overlays live in-memory for the scope of a `validate` or `finalize` run; only `finalize` writes them through (indirectly, via stamp invalidation → next read rebuilds against the new HEAD). This is what makes "delete `.jigc/`'s gitignored caches, rebuild from `.md`s, nothing lost" hold for the index too — committed `.md`s + the cascade (now including committed `.jigc/config/`) are the only source of truth.

The **working-overlay derivation**: walk the task's working area, parse each `.md` per the schema, emit the same `(source, relation, target)` edges the committed-rebuild path would emit, layer them over the committed index. Forward-ref integrity (the `finalize` gate) walks the *overlaid* graph; inverse-cardinality (completeness, store-scope) walks the *committed* graph. No expensive merge — overlay is read-side only.

## CLI and git

**The CLI orchestrates; git executes the VCS mechanics.** The CLI never reimplements branch / worktree / merge — it shells out to `git`. A *workflow* may trigger git deterministically (e.g. `finalize` staging + committing) exactly as it triggers any other structural command; git is deterministic, so the CLI core still makes no LLM calls. (`milestone-execution`'s fan-out **isolates sub-agent code by worktree** — each sub-task `git add`s its own edits in its own worktree — while its **managed docs** stay directory-isolated in `tasks/<sub>/` areas recombined by the by-task-id join; the worktree decision was *reopened and revised in M31*, see the three combine modes below and [Open questions](#open-questions).)

**git merges code; the CLI merges managed docs — never the reverse.** Letting `git merge` reconcile two edits to the same `.md` would *blind text-merge* the structured, anchored format — corrupting `{#id}` anchors and section structure and reintroducing non-determinism. **This 2026-06-04 rationale stays binding** — and M31 honors it: only *code* (never a managed doc) ever rides a worktree, and even code is never `git merge`d. So **three** combine modes, not two:

- **Overlapping writes to the same managed doc → always the CLI's deterministic by-task-id join** (one checkout, resolved against jigc_home). This is the single *managed-doc* merge primitive; git gains no second one. **The `fan-out` primitive's *docs* are exactly this case**: sub-tasks write managed docs into directory-isolated `tasks/<sub>/` areas, recombined by the join — never `git merge`. The algorithm is [The by-task-id join](#the-by-task-id-join-m7) below.
- **`fan-out` *code* → worktrees + deterministic disjoint-apply with block-on-collision** (the **third combine mode**, M31). Each sub-agent runs in its **own git worktree** (own index/HEAD) and `git add`s its code there; `finalize` combines the N attributable staged code-sets by **applying them in task-id order**, blocking + routing the named colliding paths on a same-file collision. This is **neither** the doc join (it operates on code, off the by-task-id overlay) **nor** ordinary `git merge` (no three-way text-merge — it is disjoint-apply, or block). It is the worktree-isolation the locked concurrency invariant already mandates ("each sub-agent writes to an isolated working area keyed by task ID … merged by task ID"), applied to code.
- **Disjoint *code-only* coarse work with no shared managed doc → may run in separate worktrees + `git merge`** — ordinary VCS, nothing for the CLI to mangle. This is a general capability of "the CLI shells out to git," distinct from the deterministic fan-out code-combine above.

**Why the CLI owns placement — the same rationale as owning the merge.** That the CLI, not `git`, merges managed docs is one face of a single rule: **the LLM places nothing, so it can misplace nothing.** Where every write lands is computed deterministically — a doctype's `location:` prefixed by the resolved `docs-root` (above), a section/item address resolved against the schema — never chosen by the model. This is the write-side of the determinism boundary ([CLAUDE.md](../CLAUDE.md) → "The LLM writes only through the CLI"): the agent hands prose for a *named slot*, and placement, cross-ref wiring, and merge are all CLI-owned. A hand-placed file bypasses this and is simply unmanaged — outside the store, the index, and reconciliation.

**Code resolves against the worktree; the doc-store + `.jigc/` resolve against jigc_home.** During a fan-out, a sub-agent's **code, git index, and HEAD** resolve against its **worktree** (`repo_root`), while the committed **doc-store** and the shared **`.jigc/`** (config, task areas, caches) resolve against **jigc_home** — the main checkout (`dirname(git rev-parse --git-common-dir)`). The committed doc-store has one canonical home (the main checkout); resolving it against a worktree would be the worktree-doc-bleed the merge invariant above forbids. So the two seams split cleanly: worktrees carry code, jigc_home carries docs and state.

A fresh worktree **rebuilds** the derived caches (per the stamp rule) and copies nothing else: every cascade layer is already reachable — the project layer is in-repo, the team layer is external, pack-default ships with the pack — and the per-developer `local` layer is deferred ([overrides.md](overrides.md)).

## The by-task-id join (M7)

The deterministic merge primitive (edge-index lifecycle **site 4**). A milestone fans out over its task list ([structural-grammar.md](structural-grammar.md#work-units-and-runtime-identity) → work-units); the milestone pins **one shared base** at `jigc milestone create` and every sub-task inherits it, so "present at the milestone base" is a deterministic lookup against a frozen commit, never a live-filesystem race. Each sub-task stages writes in its **own isolated `tasks/<sub-task-id>/` area** (isolation by directory). At the `join` the CLI recombines those areas into the parent's working overlay. The merge is a **pure function of the set of sub-task areas** — same set in, byte-identical result out, regardless of completion order — which is the property M7 exists to prove.

> **Isolation in M7 is structural + join-checked; the write-time barrier is M8.** "Each sub-agent writes only to its own area keyed by task id" is the locked invariant, but the *write-time* mechanism that enforces it depends on the `jigc workflow W --task <id>` re-entry that scopes a write to the acting sub-task — which is **M8** (control plane). In M7 the sub-areas are separate directories and are populated as fixtures; isolation is enforced at the **join**: a staged doc that is not attributable to its own sub-area is a blocking finding. The write-time barrier (rejecting a misdirected `--task`-scoped write) lands in M8 with the real sub-agent write path.

The algorithm:

1. **Order by task id, never by anything else.** Enumerate the sub-task areas by their **sorted task id** (an ordered key, never `read_dir`/filesystem order, never completion order). Every accumulator the merge touches (staged-doc set, edge overlay, finding list) is order-keyed (a sorted map / sorted vec), so iteration order cannot leak into the output.
2. **Disjoint union of staged docs, classified by provenance.** Each sub-area's staged docs are folded into the parent overlay in that order. The classification depends on a bit recorded at stage time — **`created`** (the doc was minted in this sub-task) vs **`edited-from-base`** (the doc existed in the committed store at the milestone base and was copied in for editing). This discriminator is what makes the next two rules decidable; without it the join could not tell a clash from a coincidental slug collision.
3. **Same-doc clash → blocking, never a blind merge.** Two sub-areas staging **`edited-from-base`** writes to the **same committed-at-base slug** is a partition violation: a blocking **`join.same-doc-clash`** finding, routed to a human. The CLI does not section-merge or last-writer-win — overlap of a shared target is an error, not a merge case (a section-level union was considered and rejected; see [DECISIONS.md](../DECISIONS.md) 2026-06-04). The **mixed case** — one sub-area `created` a slug the other `edited-from-base` (same slug) — is *also* a blocking clash, not a suffix.
4. **Colliding *new* instances → deterministic suffix.** Two sub-areas each staging a **`created`** instance whose minted id slugs the same (e.g. two coincidental `adr:cache-strategy`) are *distinct docs*, not a clash. The lower-task-id instance keeps the bare slug; each higher one takes the deterministic numeric suffix (`-2`, `-3`, … in task-id order) per [structural-grammar.md](structural-grammar.md#ids-provenance-and-minting). The join then **rewrites that instance's intra-document self-references** to the suffixed id in lockstep, so the renamed doc never dangles or points at its sibling. (A ref to *another* colliding doc cannot arise here: it would be a cross-area ref, rejected by rule 5 — one area is one sub-task.)
5. **Cross-area refs → blocking, checked per sub-area.** A staged ref that resolves only inside a *sibling* sub-area is rejected — see [validation.md](validation.md) → the cross-area rule. Mechanism: the join runs the forward-ref check **once per sub-area**, resolving that area's outgoing edges against `committed ∪ that one area` (reusing the single-area `ref_resolves` walk unchanged), **never** against a merged multi-area overlay — a naïve union of all sub-areas' `docs/` would silently resolve cross-area refs by mere path-existence.

The result is the parent working overlay, already **suffix-resolved**: every colliding `created` slug has its final (possibly suffixed) id before `finalize` sees it, so `finalize`'s destination-path sort never sees two docs competing for one path. `finalize` then commits the overlay (the commit-shaping — one aggregate commit vs per-sub-task commits — is the **`finalize.fan-out.squash`** cascade knob, M8; [finalize.md](finalize.md#fan-out-finalize)). The overlay is derived in-memory for the run, never persisted with task deltas in it (the edge-index invariant above).

## Open questions

- ~~**Milestone worktree orchestration**~~ — *settled (2026-06-04), reopened + revised (M31):* **managed docs** isolate by **directory** (`tasks/<sub>/`) and recombine via the by-task-id join above ([workflow-dialect.md](workflow-dialect.md#fan-out--join)) — never git-merged; **code** isolates by **worktree** and recombines by deterministic disjoint-apply (the [third combine mode](#cli-and-git)). `milestone-execution` partitions over the milestone's task list (`{{milestone.tasks}}`).
- **Multi-slot sub-label syntax** — the exact rendering of the schema-fixed sub-labels that delimit multiple slots within one section.
- ~~**Config layout**~~ — *settled (M4 planning, 2026-06-03):* the committed config home `.jigc/config/` has a fixed shape — `manifest.yaml` (`scalar:` + `deltas:`) + `steps/<id>.yaml` + `fills/<id>.md`, with the pack-default knob *declaration* in `config/knobs.yaml`. See [Config layout](#config-layout) above; the cascade's layers and homes in [overrides.md](overrides.md).
