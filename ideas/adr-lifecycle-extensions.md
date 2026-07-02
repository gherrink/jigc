# ADR lifecycle extensions — dated annotations + the edit-accepted immutability probe

**Status: parked 2026-07-02, unscheduled.** From the KB/research comparison (conscious rejects at v1, recorded in [DECISIONS.md](../DECISIONS.md) → 2026-07-02 harvest record). Indexed from [VISION.md](../VISION.md) → Open questions.

## Two separable halves

**1. Dated annotations (the light-update path).** The KB's `supersede-never-edit` allows appending a dated annotation instead of a full superseding record. Buildable with existing machinery: a repeatable `annotations` section (`date` set on-create + `note` slot) on the `adr` schema — a T2 version-bump when taken. Rejected at v1: no driver — the supersede path is built and proven, and a light update lives in a superseding small ADR or the consequences prose.

**2. The immutability probe (the enforcement half).** "Never edit an accepted ADR silently" is today prose discipline only; nothing detects a task editing an `accepted` ADR's slots without superseding it. A store/finalize-scope probe (advisory or blocking by severity knob) would make the norm structural — detection over declaration, the house pattern. Fits the validation-family machinery; needs a driver to earn its severity call.

## Trigger

The first real corpus shows edit-in-place of accepted ADRs (the probe's driver), or a workflow genuinely needs the light-annotation grain (the schema's driver) — whichever arrives first takes its half only.
