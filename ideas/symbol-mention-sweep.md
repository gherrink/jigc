# symbol-mention sweep — prose blast radius on a symbol removal/rename

**Status: parked 2026-07-12.** From the RC implementation-half trial, probe 2 ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md) → Probe 2). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

The anchor gate catches the *structured* half of a symbol's blast radius (`cites-code`/`implemented-by` anchors) and blocked the trial's refactor correctly. The *prose* half — seven managed docs whose paragraphs mentioned `AuditService` or asserted "the gate is dead" — was invisible: the agent found them by hand-grepping in three rounds of guessed patterns. "Green validate, lying prose" is the false-safety failure mode, and it lands on exactly the tasks the anchor gate already flags.

> The cheap fix is a symbol-mention sweep. When a task removes or renames a file or symbol, grep the managed corpus for the old name and emit advisory findings: "6 managed docs mention AuditService in prose — review." It doesn't need to understand the prose; it just needs to point.

## The shape

Detect-and-route, never rewrite (the determinism boundary holds: pointing is structural, judging materiality is the agent's). Trigger condition is already known deterministically — a task whose diff deletes/renames a file or an anchored symbol (the same signal `doc-code.symbol-exists` consumes). Sweep = substring/word match of the old name across managed-doc prose; emit one **advisory** finding per mentioning doc, route "review whether this prose still holds." Kinship: the in-prose `mention-resolves` check (M33) is the managed-mention sibling; this sweeps *unstructured* mentions, keyed off a task's code delta rather than the store.

Also the honest-cost framing from the trial, worth carrying wherever this lands: docs that describe code in the present tense make every fix a documentation project — that's the deal, and the sweep is what makes the deal payable.

## Trigger

The next validation-family wave, or the first adopter report of prose-drift-after-refactor.

**Trigger FIRED 2026-07-22** (project-alpha-3.0 trial, session 1 — [RC-alpha3/findings-verification.md](../completions/artifacts/RC-alpha3/findings-verification.md) → §3-C): a single-task inverted an anchored symbol's documented behavior (removed the throw the arch-doc calls "dead by design") with the symbol intact → **zero findings** — the third independent rediscovery of the identity-not-truth bound, and exactly this idea's failure mode ("green validate, lying prose"). The session also asked for this idea's task-scoped half almost verbatim (`jigc doc anchors <path>` / "surface the docs anchoring symbols in your staged diff" — the coupling data already exists in the target-surface enumeration). On the **M45 Settle agenda** (decisions-pending → the rc.9 wave, fork 10): de-park the sweep vs the cheaper anchoring-docs surface vs print-the-bound-only. **Re-Settled 2026-07-23 → deferred to M46** (the capability wave, entry 6): the sweep is a whole new checking surface (T3 — new capability, no bytes produced unauthored or discarded), and the three-shape fork is exactly the decision an integrity wave has no room to make well, so it stays open for the M46 Settle. Pairs naturally with [derived-doc-staleness](derived-doc-staleness.md) (doc↔doc drift) but is much cheaper — no pinned hashes, no acknowledge model needed for a per-task advisory.
