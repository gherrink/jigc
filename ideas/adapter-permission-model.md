# Adapter permission model — context-aware permission writes at setup

**Status: parked 2026-07-06, unscheduled.** From RC greenfield trial 1, findings A2+A3 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions. **Rebuts (in part) the M36 deny-floor decision — engage its rationale at pickup, not just its conclusion.**

## The gap

Setup writes a fixed permission surface into `.claude/settings.json` regardless of the host's actual permission posture:

- Under a global `bypassPermissions` setting, the allow entries are dead weight — permissions require context and are obsolete when the user has opted out of prompting entirely. Setup should detect that and skip the write (or say why it wrote anyway).
- The deny floor may be **too restrictive** — it can block something a given project legitimately needs (`curl`/`wget` deny bites any project whose workflow fetches; the trial raised the concern from live use). The M36 rationale was a safety default for running in *someone else's* repo — the counter is that a context-blind floor in someone else's repo is also a context-blind *policy* imposition.

## The shape

- Detect the host posture at setup (bypass set → skip allow-writes, note it in the summary).
- Make the deny floor a **visible, knobbed choice** at setup rather than an unconditional write — e.g. on-by-default with `--no-deny-floor`, and named in the setup summary either way (it already lists what it installed).
- Keep the merge discipline (never clobber foreign entries) — that part is settled and right.

## Trigger

The adoption trial or the first external adopter hitting a floor-blocked legitimate command; or the multi-assistant adapter milestone (a second profile forces the permission model to generalize anyway).
