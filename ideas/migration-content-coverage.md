# migration content-coverage — a deterministic source-claim coverage scan at the --approve gate

**Status: parked 2026-07-15.** From the lacon trial's v1-readiness report ([trial-record](../completions/artifacts/RC-lacon/trial-record.md) → Report 2, #4); second-trial evidence — the RC-adoption migration half (F6) already found the fidelity display "cried wolf / not verified." Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

The scary failure mode of an LLM-authored migration is silently dropping a normative sentence — and the automated part of the `--approve` gate checks almost nothing against it. Verified on rc.6 ([findings-verification](../completions/artifacts/RC-lacon/findings-verification.md) → A9): the "fidelity heuristic" is a **dotted-numeric version-token scan only** — on any doctype without version strings (every ADR in the trial) it checks literally nothing and prints a reassuring "(none)". The human reviewer is the real gate, and "a tired user clicking --approve on doc #27 won't" catch a dropped clause. V1's template-induced corruption (rc.4 trial) was this class from the template side; this is the rewrite side.

## The shape

A deterministic **string-presence coverage scan** — CLI-side, inside the boundary (no judgment, no scoring): extract checkable units from the *source* (bullets, enum-like tokens, sentences above a length floor, numbers/identifiers) and report which don't appear (normalized) in the rewrite — as **display at the gate**, feeding no structural decision (the recorded Framing-A bound: the diff is display-only, never a second structural authority — a coverage *report* raises the human's attention without becoming an authority). False positives are fine (legitimately-dropped boilerplate shows up; the human skims past); false reassurance is what's being removed. Engages the DECISIONS C4 record — the revision keeps its rationale (no CLI fuzzy judgment) while replacing the near-vacuous version-scan.

## Trigger

The next migration wave (bulk onboarding, or jigc's own post-1.0 self-migration — ~40 docs of normative design prose is exactly the corpus where a dropped sentence is expensive).
