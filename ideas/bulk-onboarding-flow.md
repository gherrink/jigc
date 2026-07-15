# bulk onboarding — bring-a-repo-under-management as a first-class flow

**Status: parked 2026-07-15.** From the lacon trial's v1-readiness report ([trial-record](../completions/artifacts/RC-lacon/trial-record.md) → Report 2) — "onboarding an existing repo is simultaneously the roughest path in jigc and the most common first contact with it." Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Onboarding today is the sum of N single migrations: the lacon trial's first hour was ~26 sequential `migrate` tasks, each human-gated (`finalize --approve`), each minting its own commit, plus a manual ~40-link repair phase afterward. Every identity edge (slug drift, task-id guessing, the same-path clobber dance) concentrated there — the steady-state single-task loop was clean by contrast on the very same corpus. The path most likely to form a new user's impression has the least affordance.

## The shape

Not one feature — a flow over existing primitives: (a) **a batch plan** — `ingest` already classifies the corpus; let it emit an ordered migration worklist (doctype-grouped, same order the trial hand-derived) instead of N independent route lines; (b) **identity preservation by default** — `--slug`-on-migrate (the fix-shaped half, chartered separately) applied batch-wide so filenames don't drift and the link sweep mostly vanishes; (c) **batched human gates** — one review/approve session over the staged set (or doctype-grouped approvals) instead of 26 gate round-trips; whether that violates the one-task-one-commit boundary is the design question — a migration *batch* may legitimately be one work unit with per-doc sub-records (the fan-out/join shape, or the milestone-record precedent); (d) the inbound-reference report (doc-search layer 2) run once over the whole plan. Cross-refs: the re-opened one-source→many-doctypes fork (F11, [decisions-pending](../implementation/decisions-pending.md)), [slug-minting-ergonomics](slug-minting-ergonomics.md), [doc-search](doc-search.md), [MIGRATING.md](../MIGRATING.md) (whose field notes this flow would mechanize).

## Trigger

The next existing-repo adopter — or the doctype-completeness milestone landing (a `reference` doctype multiplies the migration volume per repo, e.g. jigc's own ~40-file self-migration, which makes batch affordances pay for themselves).
