# Migrating to jigc — field notes

Two different migrations get called "migrating to jigc." Both have now been run for real on a brownfield repo (the RC adoption trial, 2026-07-09 → 07-12); these notes distill what worked and what bites. They are working notes, not polished docs — updated as trials land.

1. **Adopting an existing project** — foreign docs (a `.planning/` tree, loose Markdown, an old spec system) become jigc-managed docs. Verb: `jigc ingest` to classify, `jigc migrate <path> --as <doctype>` per doc.
2. **Upgrading a jigc corpus across versions** — committed managed docs move to a new doctype schema-version. Verb: `jigc migrate-corpus`.

## Adopting an existing project

The full method write-up from the reference migration (220 foreign files → 41 managed docs, ~55 commits, one day) is archived at [completions/artifacts/RC-adoption/rerun-rc4/migration-method.md](completions/artifacts/RC-adoption/rerun-rc4/migration-method.md). The short version:

1. **Triage with the tool first.** `jigc ingest` classifies every candidate against the managed schemas before you decide anything. Conformant-but-misplaced docs can be adopted; everything else is a re-authoring job through the write verbs. Knowing the split up front sets the whole plan's cost.
2. **Treat every claim in the old docs as unverified.** This is the load-bearing rule, and it is why adoption is a *verification* job, not a reformatting job. Your existing docs may already be lying about the code — status claims especially ("shipped", "passes audit", "the gate works"). Check each claim against the code before carrying it over; a false claim migrated into jigc is *worse* than a deleted one, because the new system lends it credibility (managed, validated, green). Don't migrate what you haven't verified — and where a claim can't be cheaply verified, either drop it or carry it explicitly marked as unverified and put the verification question to the human.
3. **Mine history for the four things worth keeping:** design decisions whose rationale isn't recoverable from the code, still-open debt, architecture knowledge, and work claimed complete that the code doesn't support. Everything else (plan summaries, validation reports, velocity tables) is process exhaust — drop it.
4. **Put the genuine forks to the human** — not "should I proceed" but the actual decisions (delete vs mine-then-delete, is this doc still live, where does the backlog land).
5. **Draft in parallel, write strictly sequentially.** Prose drafting and code-verification parallelize; the jigc writes must be one process (each doc mints a task and a commit — parallel writers race the tool state and the git index).
6. **Author in dependency order.** Cross-referencing docs must land after their targets: ADRs before the arch-docs that cite them — finalize blocks a citation to a doc that doesn't exist.
7. **Let `jigc migrate` retire each source in the same commit as its promoted doc.** The old tree drains itself; there is never a window where content exists in neither place. Then sweep for what the migration broke *outside* the corpus (READMEs pointing at the deleted tree, a stale CLAUDE.md).
8. **Prefer block scalars (`key: |`) in author payloads** for any multi-paragraph prose (the templates demonstrate this since rc.5; double-quoted YAML folds newlines).

## Upgrading a jigc corpus across versions

Verified on the first real committed-corpus upgrade (rc.4 → rc.5, a 41-doc corpus, one doctype with an enum rename across 41 entries — byte-stable, zero prose touched, idempotent re-run):

1. **Run `jigc migrate-corpus` first, `jigc setup` second.** The ordering is load-bearing (as of rc.5): `setup` re-stamps the store version, and re-stamping *before* migrating silences the version-mismatch signal while the docs are still on the old schema — a store that claims the new version, carries unmigrated docs, and validates clean. Known hole; until the tool enforces the ordering, you must. *(Caveat honored: the rc.5 `store-version.binary-mismatch` route text doesn't mention `migrate-corpus` — don't follow it literally.)*
2. **Know that `migrate-corpus` writes immediately.** There is no `--check`/`--dry-run` yet — running it to *find out* whether the corpus needs migrating already rewrites the working tree. It's reversible via git (the docs are committed), but run it on a clean tree so the diff is the whole story.
3. **Audit the diff before committing.** The transform is deterministic and the triage (`migrated` / `already_current` / `blocked` in `--format json`) tells you exactly what moved; the line accounting should close exactly (every changed line either a `schema-version` stamp or a declared transform). `blocked` docs are first-class output — they need a human or a prose rewrite, never a workaround.
4. **The commit is yours.** `migrate-corpus` leaves the rewritten docs uncommitted for review (deliberately — a schema migration is a content change the human approves); land it as its own commit before re-stamping.

## What migration does *not* give you

- **Truthful prose.** jigc's gates keep docs naming *real code* (anchors, refs, structure); they do not detect that a paragraph's *claim* went stale. A migrated corpus is only as honest as the verification you did at step 2 above — and it starts rotting again the first time a fix lands without its doc pass.
- **A finished doc set.** Migration carries what exists. The gaps the old system had (undocumented decisions, missing specs) are still gaps — now visible ones.
