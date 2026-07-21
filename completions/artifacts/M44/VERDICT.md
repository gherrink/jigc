# M44 — the rc.8 wave (the pull tier): completion verdict

**Status: COMPLETE — built + audited clean, 2 advisory-surface findings fixed, re-verified 2195 / 0.** As of 2026-07-21, at HEAD `9f6a67b` (base `04d7feb`).

## The claim proved

*The capabilities behave like the gates* — every capability an agent had to *reach for* in the rc.7 rerun is now preloaded into the context it can't avoid, or surfaced at the moment of relevance. M43 made the printed surfaces truthful; M44 makes the pull as reliable as the push.

## What shipped (8 risk-first increments, 26 build commits)

1. **Task-id path-hash derivation** — a `blake3(repo-relative-source-path)` disambiguator folded into every migration task id, closing the collision/serial-reuse footgun at the root. Two long-slug sources that would collide in one directory now mint distinct, attributable, resumable ids. Outside `slugify` (no slug-rule bump, no corpus migration).
2. **`write.not-present` containing-section route** — the dead-end write reject (was routed to `doc schema`, which can't reveal a minted item id) now points at a followable `doc show <type>:<slug>#<section> --task <id>`, enriched CLI-side with the real address.
3. **`jigc workflow <id> --preview`** — a compose-minus-mint read of a work-minting workflow's step text with no task dir written; `--task <your-task-id>` rendered as an identity, `.task` null.
4. **The AGENT.md preload tier** — the machine-output-contract paragraph (every verb speaks `--format json` on a success/validation outcome; producers return the id at `.task`) + the read-rule stale-source guardrail (derive behavior from the installed binary, never a checked-out jigc/pack tree).
5. **The from-knowledge `record-decision` adr workflow** — author an adr from knowledge with a fresh on-create date via plain finalize (not `migration-finalize`) + the migrate byte-floor triviality advisory (gated to `--as adr` after audit).
6. **The `{{schema:<singleton>}}`-derived stated-at fence** — pack-load reddens for any schema-bearing singleton migrate step lacking the copy-in/append constraint statement (owe-set derived structurally, no drift-prone marker).
7. **The fidelity boundary-guard + report-split** — `scan_version_tokens` stops crying wolf on slug-glued dotted runs (`project-alpha-2.0` no longer false-alarms `2.0`; `v1.0.0` kept after audit) and splits `package@version` from bare version-like tokens.
8. **Flow 45 + the docs fold-back** — the composite done-picture through the real binary + MIGRATING/CLAUDE.md/VISION updates.

## The audit

- **e2e (milestone-e2e-tester):** all 7 flow-45 arms + every grouped rider pass end-to-end through the real binary in throwaway repos. The one order-invariant flow (the task-id path-hash) was run in two divergent migrate orders across fresh repos → byte-identical per-path ids (determinism + collision-freedom witnessed). The not-present arm re-ran its emitted route verbatim to prove followability; the preview byte-identity oracle is non-vacuous.
- **code-review (milestone-code-reviewer):** no invariant violations, no correctness bugs. Engine makes no LLM calls, composition stays deterministic, frozen schemas untouched, the route fence enforced (all new placeholders declared), the new parsers panic-safe on out-of-band input. Two advisory-surface findings only.

## Findings fixed (completion-triage autonomy — both bounded fix-now)

- **MEDIUM — byte-floor advisory misdirection (`7ee77d1`).** `trivial_source_advisory` fired for every doctype but hard-coded adr-specific advice + the `record-decision` route (which produces an ADR), so a trivial `--as changelog` migration was told to author an ADR — a surface-contract law-1 lie. Fixed by gating the advisory to `--as adr`, restoring the rider to the exact scope of the rc.7 adr placeholder-source loophole it was built for. Red-reproduced through the real binary, then green.
- **LOW — fidelity guard v-prefix over-rejection (`9f6a67b`).** The word-boundary guard rejected any dotted run preceded by an alphanumeric, so a conventional `v1.0.0` was dropped from both scan sides and never reported. Fixed with a lone-`v`/`V` version-marker exception that still rejects `project-alpha-2.0`/`dev2.0`. Display-only, no gate (Framing A).

## Honest bound carried forward

M44 ships **no spawn/fan-out deliverable** — the milestone fan-out live-spawn probe (the M39 honest bound's owed half) is explicitly the **post-M44 rc.7 verification trial**, sequenced after this build. There is no concurrent-spawn artifact in this milestone to sim or to flag as unrun.

## Next

The **rc.7 verification trial** (clean-room protocol: structural absence of this repo's checkout + unseeded probes; carrying the fan-out live-spawn probe) → the **1.0.0 call** (the human's, rc count deliberately open).
