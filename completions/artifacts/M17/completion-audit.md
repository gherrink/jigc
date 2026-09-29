# M17 completion audit — the spine terminus

**2026-06-13.** Independent, adversarial, read-only review (milestone-code-reviewer) of the
milestone-completion phase. M17's increments 1–5 (capture apparatus + three binary soundness fixes)
were audited and remediated in a prior session (5 findings fixed); this audit covers the
session that added `decided-task` (the self-hosting dogfood target, promoted to main) + the
measurement runs + the verdict — diff `f9c4b0a..00e2ab3`.

## Verdict: PASS — no blocking findings

The auditor independently confirmed:

- **Feature sound.** `decided-task` is well-formed against its siblings; `selectable: false` genuinely
  keeps it off the router (the filter at `crates/cli/src/start.rs` is load-bearing; `selectable`
  defaults `true`, so opting out is explicit). The extended off-router guard is **non-vacuous**
  (`assert_eq!(ids, ["dev-task"])` on the real binary's JSON + the `knobs.yaml` enum). The compose
  test asserts real behavior (spine order, the create-gate admitting `decisions-log`, the
  no-roadmap/ledger negative). Gate green (test · clippy -D · fmt).
- **Scope honest.** Only 4 code/pack files + docs; no drive-by edits, no engine changes;
  decisions-pending.md:77 correctly resolved; exported evidence byte-identical to the shipped files.
- **Measurement integrity holds.** Manifest hashes match the exported files; the drift-caught split
  (raw 2 → organic 1 + seeded 1) and oob split (raw 1 → organic 0 + seeded 1) are defensible against
  the raw hook log; nothing in the record is unsupported by the capture.
- **Verdict honest.** No residual overclaim after the Codex revision; leads with "did not demonstrate
  the comparative thesis," labels the self-hosting run an uncontrolled existence proof, concedes "the
  differentiator engaged is true by construction." Grounded against the pilot results.

## Advisories (no fix required — all deferred/contested, already named)

1. **defer** — `case: pilot` is a stand-in (the `dogfood-record` enum predates self-hosting); a
   one-line `self-hosting` enum member is owed. Already named in the record + verdict.
2. **defer** — `decided-task` runs `implement`/`gate` vacuously for a pure decision-only task. The
   run's own completion audit flagged this advisory/deferred; the workflow targets "one code change
   that also decides," so it's correctly scoped.
3. **contest/ignore** — the dogfood-record and the inner completion-record carry different
   `owner-artifact` paths (two different runs' artifacts); correct as recorded.

No triage/fix rounds were required. M17 closes the roadmap spine.
