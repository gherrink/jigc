# spec criterion status — per-criterion satisfied/proof, the traceability field

**Status: parked 2026-07-10; evidence went conclusive 2026-07-12.** From the RC adoption trial's doctype-gap feedback, finding D4 ([trial-record](../completions/artifacts/RC-adoption/trial-record.md) → Doctype gaps). **Frozen-v1 gated:** `spec` is in the frozen set, so this is a schema-version bump + a versioned corpus migration when it lands — correctly priced, not free. Indexed from [VISION.md](../VISION.md) → Open questions.

**2026-07-12 update — the implementation-half trial made this the top-ranked structural gap** ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md) → the observer's final report). Two facts, both conclusive: (a) `maps-to-test` has been written **zero times ever** — across 11 specs, 153 criteria, ~1400 logged invocations, two real spec implementations that each wrote real tests, and the workflow purpose-built for it ("no longer underused — a field that does not exist in practice"; the *prompting* half is pack YAML and is chartered with the rc.6 wave, but the field only closes the loop if proof-shaped status exists to point at); (b) **three implemented specs now assert their bugs in present tense while validate reports clean** — "the anchor gate checks symbols, not claims; a spec is nothing but claims," so `spec` is precisely the doctype the anchor gate cannot protect, and it rots fastest. The observer's frame widens this idea from a field to a **lifecycle**: a status/satisfied-by that makes "this spec's work is done" a fact in the store, closure reachable at finalize. Sibling: [spec-open-decisions](spec-open-decisions.md) (the same schema-version bump should carry both).

## The gap

The trial's source `v1.0-REQUIREMENTS.md` carried a traceability table mapping all 30 requirements to the phase that delivered them. Dropping the checkbox state was *correct given the schema* — spec has no per-criterion status — but the information was real, and it now survives only as prose in a roadmap decomposition slot. "This criterion is satisfied, and here is the proof" is a field, not a doctype.

## The shape

`spec` criteria already carry an optional `maps-to-test` — one short step away. The candidate: an optional per-criterion `status` (or `satisfied-by`) field whose value is proof-shaped (a test anchor, a commit, a milestone-record ref) rather than a bare checkbox — keeping detect-and-route intact (a claimed-satisfied criterion whose proof anchor vanishes goes stale loudly, the doc↔code pattern). Optionality matters twice: authoring specs must not owe status, and the field must not break the create path (the invisible-required-field lesson, capability-matrix gap 1).

## Trigger

The doctype-completeness milestone *if* a driving flow earns it there (a milestone-completion flow reading criteria back is the natural driver); otherwise the first post-1.0 frozen-set version wave, so the migration cost amortizes with other v-bumps.
