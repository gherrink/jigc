# Title-anchoring fix — Opus re-test

**Written 2026-06-23.** The verdict's named next move — *close the prose blind spot,
then re-test the capable model* — implemented and measured. The fix
(`title-names-symbol`, commit `33caa87`) makes the engine **block** when an arch-doc
component heading carries a symbol-shaped token that is not the anchored symbol; it
rides the existing `doc-code` finding path, so `jigc validate`, the finalize floor, and
the blocking pre-commit hook all enforce it with no hook change. Implementation +
rationale: [VERDICT.md](VERDICT.md) → follow-up; DECISIONS.md 2026-06-23.

## Result: the fix works

Opus arm-A, the prose surface that lost pre-fix (objective oracle — dead symbol names
in committed component titles, the surface the fix governs):

| | rep1 | rep2 | mean | blind judge |
|---|--:|--:|--:|---|
| **pre-fix** (no title check) | 5 | 1 | **3.0** | 0/2 consistent |
| **post-fix** (title check) | 1 | 0 | **0.5** | objective-canonical (see note) |

- **Stale prose fell 3.0 → 0.5** — rep2 stayed clean across all 8 edits; the title
  check **blocked every edit** and drove the agent to fix the heading, not just the
  anchor. Dangling anchors stayed 0 throughout (unchanged — that surface was already clean).
- **The one residual (rep1, edit 8: `UIDoc`)** exposed a blind spot in the first
  heuristic: `is_compound_identifier` keyed only on a lower→upper camelCase hump, which
  **`UIDoc`** (an acronym-prefixed identifier, `UID`+`oc`) has none of, so the stale
  heading slipped through. Closed in the same commit by also matching the
  acronym-then-word shape (`[A-Z]{2,}[a-z]` — `UIDoc`, `HTTPServer`, `IOError`), with a
  unit test and an **e2e check**: the exact `UIDoc` stale-title case now blocks
  (`doc-code.title-names-symbol`). So the current binary would score the rep1 residual
  clean too — proven by test + e2e rather than by a further paid run.

## Honest bounds on this re-test

- **The measured re-test (stale 0.5) ran with the v1 heuristic** (camel-hump only), so
  it carries the now-closed `UIDoc` residual. A clean confirmatory run on the corrected
  (v2) binary — to *measure* ~0.0 rather than infer it — is **owed but auth-blocked**:
  the OAuth token expired mid-run on the v2 attempt (HTTP 401 from edit 2 onward; only
  edit 1 of each rep ran), so that run was discarded
  (`runs/opus-A-titlefix-v2-401-killed/`, not used). The token has a short, unstable
  lifetime; rather than burn another ~$100 Opus matrix against it, the residual closure
  is established by unit test + e2e. Re-run when auth is stable for the measured 0.0.
- **The blind judge was confounded this round** and is *not* used for the post-fix
  number. It flagged each component's lowercase `{#slug}` anchor (e.g.
  `{#commenttagparser}` on the renamed `### TagBlockParser`) as a "stale symbol." Those
  slugs are **stable opaque IDs that correctly never rename** (a jigc invariant), not
  code citations — a judge instrument error, not doc drift. The objective oracle
  (`measure.py` + the `doc-code` probe) is canonical here, per the pre-registration's
  "objective first." (Pre-fix judge 0/2 stands — those were real stale *titles*.)
- **Cost.** Opus arm-A with the title check is expensive — the block→recovery loop runs
  the agent hard ($1–10/edit). The fix raises Opus's correctness; it does not lower its
  cost. (Sonnet, the regime where jigc strictly wins, was already clean pre-fix and is
  unaffected.)

## What it changes about the verdict

The original verdict: jigc strictly wins for Sonnet (long-horizon, the move/path-drift
class) but **loses for Opus** because the enforced surface (anchors) was narrower than
the instructed one (prose). This fix **closes that gap**: with titles enforced, Opus's
prose drift collapses (3.0 → 0.5, rep2 clean), removing the specific reason jigc ended
dirtier than a static instruction for the capable model. The enforced surface now covers
the heading the capable model was leaving stale. The broader bound is unchanged: jigc's
value is the *salience-independent enforcement of a checkable surface* — extend the
checkable surface (here, titles) and the win extends with it.
