# M47 rebuild — hand-off to the build session (written 2026-08-02)

**Goal state.** Re-execute the **M47 build** (the rc.10 wave — the surface-fundament wave) from the recovered decomposition. **This is a re-execution, not a re-plan**: the wave was planned, settled (fourteen forks), reviewed twice, decomposed, and fully built on a machine whose drive died with everything unpushed; the planning record was recovered **byte-exact** from the session event exports and folded back at `b2560fc`, but the build's code (76 commits, all 11 increments, rc.10, suite 2552/0) is lost. Do not re-open any settled fork. Baseline: current `main` (code identical to installed `1.0.0-rc.9`; every commit since `e07637a` is docs-only), suite **2385/0**.

**Run it as:** `/milestone-build` with `{ milestone: "M47", base: <current HEAD> }`, one increment at a time per the roadmap decomposition.

## The record of decision (read before building — in this order)

1. **[implementation/roadmap.md](../../../implementation/roadmap.md) → Milestone 47** — the eleven-increment decomposition, recovered in its **final as-of-crash state** (Increment 3's text is the *re-scoped* version — build it as written; the four post-review amendments are already folded in).
2. **[DECISIONS.md](../../../DECISIONS.md) → the four 2026-07-26 M47 entries** — the Settle (fourteen forks; the fork resolutions bind), the pre-decompose review (ten blocking findings), the cross-model review (three blockers), and the **Increment 3 mid-build halt resolution** (the fan-out abort must stop destroying sub-agent staged code; its T0 has four parts, all owed again).
3. **[implementation/decisions-pending.md](../../../implementation/decisions-pending.md) → the rc.10 wave (✅ PLANNED)** — tiers 1–3, the checkable boundary (*the wave changes what existing surfaces **say**, never what surfaces **exist***), and the G1–G3 greenfield acceptance (post-build, not this session's job). Entry 10 (the durable "was provisioned" fact) is routed there — do not build it.
4. **[recovery-report.md](recovery-report.md)** — the full reconstruction of the lost build: per-increment content, the mid-build fork answers, the completion audit's findings (4 LOW code-review · 4 non-data-loss e2e defects), and the 12 post-audit fixes that landed. **Treat its defect list as a known-fixes checklist**: where the report says the lost build fixed something post-audit, prefer folding that fix into the owning increment this time; at completion-audit time, verify every listed defect is closed.
5. **Context, not tasks:** [baseline.md](baseline.md) (the six-auditor capability ledger the Settle was corrected against) · [planning-gate-record.md](planning-gate-record.md) · [scope-brief.md](scope-brief.md) · [settle-agenda.md](settle-agenda.md) · the findings the wave fixes: [RC-alpha4/findings-verification.md](../RC-alpha4/findings-verification.md) (A/B/C/D rows, live repros re-runnable) + [original-findings-verification.md](../RC-alpha4/original-findings-verification.md) (the H/P rows the charter keys on; crosswalk in [recovered-analysis.md](../RC-alpha4/recovered-analysis.md)).

## Hard constraints (already decided — re-deciding them is the failure mode)

- **The three one-way doors are DECIDED**: `SLUG_RULE_VERSION` 2→3 (the hyphenated-compound fix; un-migratable by design) · the `schema_hash` narrowed to a **presentation projection** (erase `description`/`usage`/slot `hint`; keep the six semantics keys) **plus the same-version-re-pin fence built now** · **one error code per committing door** (revises `ERROR_CODE_REGISTRY`'s recorded "deliberately not per-verb" stance — that revision is on record, DECISIONS → the Settle).
- **The human's overruling calls stand**: N6 — the commit-doc writes move from `step:implement` to `step:finalize` (via the Decision-5 structural resolution in the halt/review entries — both packs' `migration-finalize.yaml` end in `{{ include: step:finalize }}`, so the split is the solicit-half/commit-half design of Increment 5, not a naive move); H2 — per-door error codes (above); H1 — carryover **plus** the six staging-independent owner-artifact causes get wired onto validate, prose-scoping on all **nine** promise surfaces regardless.
- **Version discipline**: the wave ships the version it is named for — close = `1.0.0-rc.10` bump + release build + install (`milestone-completion-workflow` → the 2026-07-24 rule).
- **Push policy (new, 2026-08-02)**: push `origin main` at the end of each increment-or-larger unit — not per commit, and **never end the session unpushed**.

## Known deltas the rebuild must cover beyond the roadmap text

- The **four unfixed git-path members** + **3 LOW hygiene items** (incl. `doc create --help`) that were in-flight when the machine died — recovery-report → "build state at the crash"; they are part of done this time.
- The small **confidence-audit roadmap row** commit (`docs(m47): the confidence-audit wave gets the roadmap row it shipped without`) — lost, re-add.
- Flow **47** acceptance (`crates/cli/tests/flow47_acceptance.rs`, 7 axis-iterating arms per the report) + the golden regeneration **with** the fixes, never before them (Tier 3's rule).
- A **completion VERDICT.md** never got written — this build owes one.

## Honesty rules for the rebuild

The recovery report describes the lost code; it is a *spec*, not a diff to replay. Where test-first re-derivation lands a different shape than the report describes, follow the tests and **note the divergence** in the increment's DECISIONS entry rather than forcing conformance. Per-increment/per-task DECISIONS entries are re-minted fresh (the lost ones are gone; the marker in DECISIONS.md says so). After the build + audit + rc.10 install: hand back to the human — the **G1–G3 greenfield trial** runs operator-driven in separate blind sessions (charter → decisions-pending), and **the 1.0.0 call is the human's**.
