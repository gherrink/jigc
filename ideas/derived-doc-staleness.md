# Derived-doc staleness — track what a doc was distilled from, flag when the source moves

**Status:** idea; parked (2026-07-02); unscheduled — explicitly **post-v1 / post-go-live**. Not
a gap in the spine; a candidate for the doctype-expansion track once a concrete driver earns it.

**Consumer framing superseded by M37 (2026-07-04).** The `adr`/`spec`/`prd`-consumer framing
throughout this doc (§The idea #1, §Motivating scenarios, §Constraints #1) is **superseded by the
narrowed M37 scope**: M37 shipped the `research` doctype in the **methodology pack** with a plain
`vision —grounded-in→ research` `ref` edge only ([design-altitude-doctypes.md](../design/design-altitude-doctypes.md);
[doctype-map.md](../implementation/doctype-map.md) → The relations (edges)). The frozen-v1 dev-pack
consumers (`adr`/`spec`/`prd`) are **not** what shipped — so the §Constraints #1 "touches frozen-v1
doctypes → version-gate" cost does **not** apply to the M37 build. The **hash-pinned staleness mechanic
itself (the expensive half, #2) stays parked here, trigger unchanged** — M37 built neither `derived-from`
nor the pinned-hash probe.

## The idea

Two separable pieces, deliberately parked as a pair so a future milestone can earn them
independently:

1. **Provenance edges / a `research` doctype (pack-level, the cheaper half).** Research becomes
   a managed, citable artifact; an adr/spec/prd (or slide-deck, skill, guidance doc — whatever a
   pack ships) references it through an ordinary `ref` relation. Valuable on its own with zero
   new engine machinery: the edge index already makes provenance queryable both directions
   ("what does this decision rest on?" / "who consumes this research?" — the latter is exactly
   the M35 inverse-edge enumeration).
2. **The staleness mechanic (engine-level, the expensive half).** A `derived-from` edge carries
   a **pinned content hash of the source at last acknowledgment**. When the source's current
   hash diverges from the pin, a store-scope probe reports the derived doc as *stale*. A cheap
   acknowledge verb ("reviewed; no change needed" / "refreshed") re-pins without forcing a prose
   edit. Presupposes nothing about which doctypes use it.

## Motivating scenarios (all "source updated → know where to update too")

- Research articles distilled into presentation slides — update the research, get told which
  slides cite it.
- Guidance docs distilled into skills — update the guidance, get told which skills derive from it.
- Vision-sharpening / milestone research stored **in** the project and referenced from adr/spec/
  prd — the research that justified a decision stays attached to it, and the decision surfaces
  for review when the research moves. Which doctypes participate is a **pack** question
  (methodology pack and beyond), not an engine one.

Both ends are **managed docs** — that's the scoped version. Arbitrary-file sources (the
doc↔code path+hash anchor model) were considered and set aside; if wanted later, that's a
separate weaker tier.

## Why it fits (the existing machinery)

- A `derived-from` edge is structurally a sibling of `supersedes` — stable IDs, edge index,
  rename-survival all come for free.
- The posture is already doctrine: **detect and route, never auto-author** (Non-goals; the
  reconciliation model). The CLI can *prove* "source changed since last acknowledged" (hash
  comparison — squarely inside the determinism boundary); whether the change is *material*, and
  the updated prose itself, stay with the LLM/human. The staleness flag is a routing signal,
  not a fix.
- Direct architectural rhyme: `doc-code` validation is doc↔code drift; this is **doc↔doc**
  drift — the missing sibling in the drift family.

## Constraints to respect (record now so a future planner doesn't underestimate)

- **The consuming side touches frozen-v1 doctypes.** Adding a research/`derived-from` relation
  field to adr/spec/prd is a schema-*shape* change → rides the version-gate: manifest
  `schema-version` bump + a versioned corpus migration (the M34 machinery exists; sanctioned,
  not free).
- **Store-scope, report-only.** The staleness probe reports like `validate`; it must **never**
  gate finalize — an unrelated doc going stale cannot block the current task (blocking verdicts
  live at the task/finalize gate, per the validation doctrine).
- **The new stored fact is the per-edge pin.** The genuinely new engineering is the pinned
  source hash + acknowledge verb lifecycle (where it lives, how OOB edits re-baseline it, how
  fan-out joins merge it) — not the edge itself.

## Open questions

- **Granularity:** doc-level pins are cheap but noisy (any edit to a large research doc flags
  every consumer); section-level ("slide 4 cites research §3") needs **per-section content
  hashing**, which the engine doesn't have. Deliberately undecided — roughly doubles or halves
  the engine cost.
- **False-positive fatigue:** the design risk. If the flag is noisy, it gets ignored and the
  feature is worse than nothing. The acknowledge verb + granularity choice are the two levers.
- **Acknowledge semantics:** who may re-pin, and is "reviewed, no change" distinguishable from
  "refreshed" in the record?
- **Which pack ships the `research` doctype** (methodology? a new domain pack?), and what its
  schema needs beyond title + prose + citable sections.

*Doctype mapped 2026-07-03: the pack-level `research`/provenance doctype half of this idea is indexed in [doctype-map](../implementation/doctype-map.md) → The purpose map.*
