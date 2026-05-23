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

1. **Front-matter** — the document's *header block* fields (status, dates, doc-level refs), as `key: value`. Doc-level refs here keep the graph scannable without parsing the body.
2. **Body** — schema-fixed section headings; under each, the slot prose plus (if any) a trailing `key: value` field group. Same field syntax as front-matter — one rule everywhere.
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
- maps-to-test: `test/rate_limit_spec.rb#burst`

### Burst allowance  {#burst-allowance}
A short burst above the limit is tolerated for 2s.
```

### Identity, order, fields, slots, items

- **Identity is the path** — `specs/auth-flow.md` → type `spec`, id `auth-flow`. No redundant id in the file.
- **id-source = the heading** — a doc's title is its H1 (its frozen id is the filename); an item's title is its `###` heading (its frozen id is the `{#id}`). The title is mutable; the id is frozen at creation, so `spec:auth-flow#criteria/rate-limit` survives a retitle.
- **Order = physical order.** Reordering is moving a block — a clean diff move, never a renumber.
- **One field syntax** (`key: value`); header block → front-matter, section/item block → trailing group.
- **One prose slot per section preferred.** Multi-slot sections are allowed but render each slot under a schema-fixed sub-label (matched like a heading — no new marker), which nudges schemas toward one-purpose sections.
- **Items live inside their parent file** (the criteria-in-doc decision); no per-item files.

## Repository layout & state location

Committed source of truth and derived/transient state live apart:

```
specs/  decisions/  …       # committed .md docs — the only source of truth
.tool/                      # gitignored — all derived/transient state
  tasks/<task-id>/          #   per-task working area (staging)
  index/                    #   edge index (rebuildable cache)
  state/                    #   file↔CLI-state hashes (rebuildable)
```

### The per-task working area (staging)

A task's staging area is a **gitignored scratch dir of working copies**, in the same Markdown format. It realizes the locked concurrency primitive directly: fan-out sub-agents each write to their own `tasks/<task-id>/` subdir (isolation by directory), and the CLI merges at the join **by task-id order** ([structural-grammar.md](structural-grammar.md#repetition), [VISION.md](../VISION.md) → Parallelism). This is *not* git worktrees — git's text-merge is not our deterministic by-task-id join, and we'd implement the join over worktrees anyway.

A task is **pinned to its base** (the commit it started against). Operating a task whose base ≠ the current checkout is detected and routed (*"started on `<base>`; you're on `<other>` — switch back or discard"*), the same divergence philosophy as out-of-band detection. *Rebasing* a task onto a new base is deferred (parallels three-way merge).

Only `finalize` produces committed changes; the working area itself is never committed (it persists on disk across sessions, so work resumes, but is lost on a clean/clone — acceptable for staging).

### Derived caches

The edge index and the file↔state hashes are **rebuildable from the committed docs**, so they are gitignored, never committed: committing them would churn diffs *and* reintroduce the dual-source-of-truth we eliminated with derived inverses. Each cache is **stamped with the HEAD (or a doc-set fingerprint) it was built against**; a branch switch, pull, or rebase changes the docs underneath it, so on a stamp mismatch the cache rebuilds. (The hash baseline's first-run edge case is noted in [write-commands.md](write-commands.md) → reconciliation.)

## CLI and git

**The CLI orchestrates; git executes the VCS mechanics.** The CLI never reimplements branch / worktree / merge — it shells out to `git`. A *workflow* may trigger git deterministically (e.g. `milestone-execution` opening a worktree) exactly as it triggers any other structural command; git is deterministic, so the CLI core still makes no LLM calls.

**git merges code; the CLI merges managed docs — never the reverse.** Letting `git merge` reconcile two edits to the same `.md` would *blind text-merge* the structured, anchored format — corrupting `{#id}` anchors and section structure and reintroducing non-determinism. So:

- **Overlapping writes to the same managed doc → always the CLI's deterministic by-task-id join** (one checkout). This is the single merge primitive; git gains no second one.
- **Disjoint coarse work → may run in separate worktrees + `git merge`** — no shared managed doc for git to mangle; it is ordinary VCS. Since fan-out is planned and bounded, the workflow knows the partition is disjoint.

A fresh worktree **rebuilds** the derived caches (per the stamp rule) and **seeds only** genuinely-local, non-derivable config (a `local` cascade layer, if present) — it copies no derived state and no task working areas.

## Open questions

- **Milestone worktree orchestration** — how `milestone-execution` partitions and recombines worktree-isolated tasks is workflow-dialect territory ([structural-grammar.md](structural-grammar.md#open-questions)); storage only needs to *accommodate* it, which it does.
- **Multi-slot sub-label syntax** — the exact rendering of the schema-fixed sub-labels that delimit multiple slots within one section.
- **Local config layer** — whether a gitignored `local` cascade layer exists, and exactly what a fresh worktree must seed.
