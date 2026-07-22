# finding doctype — a defect record that self-invalidates against the code

**Status: parked 2026-07-10.** From the RC adoption trial's doctype-gap feedback, finding D1 ([trial-record](../completions/artifacts/RC-adoption/trial-record.md) → Doctype gaps). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

The trial migrated three **live security findings** (a hardcoded credential, unfilled authorization checks, a token read from env) into `idea` docs — because nothing else held them. `idea` is "one shaped-but-unscheduled direction with a re-entry trigger"; a hardcoded credential is not a direction, it's a defect. The consequence: a security hole in main and a nice-to-have-in-v2 share one doctype and three fields, severity flattened, compensated in prose.

## The shape

jigc already has the vocabulary — a `completion-record` finding carries **severity** (blocking/advisory), **disposition** (fixed/deferred/contested), and **evidence**, imprisoned inside the per-milestone completion record. A standalone `finding` doctype = those three fields + an arch-doc-style **`implemented-by`/`evidenced-by` code anchor**.

The anchor is the differentiator: `validate` already fails when an anchored symbol vanishes — point a finding at `ImportOldCommand.php#ImportOldCommand` and the moment someone rips out the credential, `validate` reports the finding stale. **A defect record that self-invalidates against the code** is something almost no tracker does, and jigc is three fields away from it (the trial's words). Detect-and-route holds: staleness is reported, the disposition judgment stays human/LLM.

Design opens at pickup: lifecycle (does a `fixed` disposition retire the doc or keep it as record?), relation to `completion-record` findings (same item shape, shared fragment?), and whether the anchor is required (the F5 anchorless-asymmetry lesson) or optional-but-advisory-when-absent.

## Trigger

The post-1.0 **doctype-completeness milestone** (the [self-migration prerequisite](../completions/artifacts/RC-adoption/self-migration-coverage.md)) — ranked there against [reference-doctype](reference-doctype.md) and [postmortem-and-runbook-doctypes](postmortem-and-runbook-doctypes.md).

**2026-07-22 addendum (the M45 discussion — a second tenant for the same shape):** the **verified-fact ledger**. M45 lands verified-facts-become-tests as convention (a hand-verified fact either names its pinning test or says "unpinned" — [milestone-completion-workflow.md](../implementation/milestone-completion-workflow.md) → Re-verify); when this repo self-migrates under jigc, that convention should become *this doctype* — a fact record whose code-anchor points at its pinning test, so a fact whose anchor dangles goes red by the same self-invalidating mechanic the defect record uses. Rank the two tenants together at pickup.
