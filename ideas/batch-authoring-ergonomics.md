# batch authoring ergonomics — the per-leaf write cost

**Status: parked 2026-07-12.** From the RC implementation-half trial, probes 3+4 ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Authoring a multi-leaf doc costs one command per leaf, and the trial paid it twice over:

- **The trailer dance** (probe 3): a commit trailer is `add-item` → read the ack's address → `set-field <addr>/value` — a round-trip that exists purely to learn an ID. "Commit authoring was five commands; it could be one (`jigc commit --type fix --scope monitoring --summary … --trailer …`)."
- **The batch verb didn't pay for itself** (probe 4): `doc author` exists for exactly this, but its payload had enough of a learning curve — *post*-M41's fold-safe templates and long-help grammar — that the agent opted out and used incremental `set-slot --from-file -` heredocs for every multi-paragraph doc.

So the batch path exists and is avoided, and the incremental path is verbose. Both are the same cost surface as [composed-context-token-budget](composed-context-token-budget.md)'s invocation-output face (N commands × N acks).

## The shape

Candidates, not settled: (a) `add-item` accepts leaf values inline (`add-item <addr> --field value=…`) so the mint+fill round-trip collapses — smallest, no new verb; (b) per-doctype convenience flags on a fill verb (the `jigc commit --type …` sketch) — biggest surface, probably wrong (per-doctype flag surfaces don't generalize); (c) invest in `doc author` adoption instead — figure out *why* agents opt out (payload construction cost vs. failure-recovery cost when one leaf is rejected) before adding anything. The M41 fold-safety work fixed correctness; this is about the payload being worth writing.

## Trigger

A third independent session avoiding `doc author`, or the composed-context/invocation-output measurement showing the per-leaf ack overhead is material.

**2026-07-15 — the trigger fired, and the cause is now located** ([lacon trial](../completions/artifacts/RC-lacon/trial-record.md), task 4; verified [findings-verification](../completions/artifacts/RC-lacon/findings-verification.md) → B15): the third session defaulted to N per-leaf calls *because that's what the workflow text models* — the fresh-authoring steps (`author-roadmap`/`author-decision`/`author-ledger`) print `add-item` + `set-slot` lines verbatim, while `doc author` is modeled **only** in the migrate-* templates. Notably the *migration* phase of the same trial used `doc author` 25 times without a stumble — the verb is fine when the template hands it over. So the cheapest candidate is now (d): **model `doc author` in the fresh-authoring workflow steps**, before any new verb surface. The single-call-finalize ask (task 1 #4: `finalize --type --summary` for quick-fix) is the same cost surface, thinnest tier.
