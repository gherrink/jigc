# M43 — the rc.7 wave (the surface contract): completion verdict

**Status: COMPLETE.** Built (9 increments), audited, **3 audit findings fixed** (1 surfaced as a declared-bound observation, below), re-verified at HEAD `172aea4`: **2137 passed / 0 failed**, `clippy -D warnings` clean, `fmt --check` clean, tree clean.

Planning: [DECISIONS.md](../../../DECISIONS.md) → 2026-07-16 (the Settle — twelve decisions, every fork robust, each argued by an independent robust-advocate) · [planning-gate-record.md](planning-gate-record.md) · [roadmap.md](../../../implementation/roadmap.md) → Milestone 43. Design of record: [surface-contract.md](../../../design/surface-contract.md).
Acceptance: [worked-examples.md](../../../design/worked-examples.md) → flow 44 · `crates/cli/tests/flow44_acceptance.rs`.

---

## The claim, and whether it holds

**M43's claim: *everything jigc prints is a contracted surface — nothing lies, nothing hides, nothing ambushes — each law fenced where the surface is generated.*** The e2e audit drove all nine flow-44 done-picture arms plus six hardening probes through the real binary, **all pass**: the generated migrate template names all five adr sections (the "fixed four-part" lie unrepresentable) · `decided-task` routes from the catalog on its decision axis and the orientation footer stops claiming `start` routes · the carryover gate refuses pre-mint staged adds *and deletions* at every minting door, `--carry-staged` lands them labeled, byte-identical across divergent staging orders · the same-path migration reviews at exit 4 and `--approve` lands `M`-not-`D+A`, the hold logging `migrate.review-pending` · the staged read round-trips at slice depth with one marker key, the transient commit doc readable, the stale read routed · the resume re-feeds the foreign source · the enum members render generated from `Field.of` · `doc create` acks created-vs-existed and copies in over a committed slug · the empty store says so · the three pack-load fences block loudly on mutated filesystem packs and `describe` narrates each hidden workflow's reason · `schema-ref-resolves` blocks a dangling `{{schema:}}` at compose and reports it at the sweep · the wrong-id doors converge on `task list`, the ghost `discard-write` verb is repaired · A14 prints repo-real paths with the transient file-state skip · `doc schema` contract-version 3 carries concrete write addresses · **the N-process fan-out sim + verbatim spawn-template execution + by-task-id join determinism hold** (the M39 honest bound's sim half re-proven on the M43 surface; the live-spawn half stays owed to the rc.7 trial's designed probe).

## Audit findings — 3 fixed, 1 a declared-bound observation

| # | severity | finding | disposition |
|---|---|---|---|
| 1 | **MEDIUM** | `schema-conformance.unknown-type` shipped blocking with `route: None` — the wave's own seam assert panics the debug binary on `task validate --format json` over a staged instance of an undefined doctype (reproduced live); release builds emit the route-less block the wave claims unrepresentable | `1e5cc04` — routed via the Inc-1 declared route map (`jigc describe` check + restore-or-retype); **not** exempted (a recovery exists) |
| 2 | LOW | Three latent route-less blocking producers (`structural-target.*`, `slot-fill-target.*`, `overrides.project-step-missing`) escape the seam only because they render via Display today | `4113ccd` — **all three routed** (grammar/recovery named), none exempted; the target-presence companion argued into `is_declared_singleton` per-family with rationale, `command-output-contract.md`'s outside-the-envelope bullet revised (decision, not accident) |
| 3 | LOW | `AMBUSH_CLASS_CODES` documented `finalize.left-out` as an engine finding code no producer mints (its real surface is the M42 print pair) — a law-1 wobble in the fence's own doc | `172aea4` — the const is the ambush-class **constraint-identifier** set; the split (3 minted codes + 1 declared print-surface contract) stated honestly in the const and [surface-contract.md](../../../design/surface-contract.md); no new Finding (the M42 print-over-refuse settle untouched) |
| 4 | LOW | The error-code registry collision test checks the 34-row keyed inventory, not the un-keyed gate families the two members actually neighbour — a future un-keyed `finalize.commit-rejected` finding would collide undetected | **Declared-bound observation, not fixed** (the auditor's own framing: matches [surface-contract.md](../../../design/surface-contract.md)'s letter). Tightening requires enumerating un-keyed finding codes — no registry exists and a grep-census is the barred pattern. Carried below as the honest bound; re-open if a finding-code registry ever exists. |

## Honest bounds

- The seam fences (route parse, floor presence, key discrimination) are `debug_assert`-posture — enforced by the suite, not the release binary (the recorded per-class posture; finding 1 is the case study of why the suite must exercise every producer).
- The pack-load fences cover the shipped embedded packs; project-layer workflow deltas are outside them (declared).
- The carryover gate fails **open** on a missing snapshot (pre-M43 in-flight tasks; observed live in the audit — the declared bound, honest).
- The error-code collision test's inventory bound (finding 4 above).
- The milestone fan-out unit's **live agent spawn** stays owed to the rc.7 verification trial's designed probe (human call 2026-07-16) — the sim + verbatim-template halves are proven here.

## Re-verification

Full gate at `172aea4` (the last fix commit): `cargo build` · `cargo test` → **2137 / 0** · `cargo clippy --all-targets -- -D warnings` · `cargo fmt --check` — all clean, tree clean. Each of the three fixes landed red-first through the dev-workflow with the auditor's reproduction re-driven through the real binary.
