# M35 rename-cost matrix — cost + completeness

> Headline: **cost per rename** (the first study where jigc can beat plain on *effort*). Win = jigc strictly cheaper per rename than plain AND completeness ≥ static. Completeness scored over **structured managed refs only**.

| arm | model | reps | cost/rename | turns/rename | completeness | rename-engagement | err→recov | final dangling | control✗ | oracle≠ | void |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--:|
| plain | opus-4-8 | 3 | 0.2655 | 10.4286 | 0.625 | 0.0 | 0 | 3 | 0 | 0 | ⚠ low-N |
| plain | sonnet-4-6 | 3 | 0.1248 | 8.25 | 0.25 | 0.0 | 0 | 6 | 0 | 0 | ⚠ low-N |
| plain | sonnet-5 | 3 | 0.2621 | 14.375 | 1.0 | 0.0 | 0 | 0 | 0 | 0 | ⚠ low-N |
| static | sonnet-4-6 | 3 | 0.1524 | 10.4167 | 0.875 | 0.0 | 0 | 1 | 0 | 0 | ⚠ low-N |
| static | sonnet-5 | 3 | 0.215 | 10.0 | 1.0 | 0.0 | 0 | 0 | 0 | 0 | ⚠ low-N |
| jigc | opus-4-8 | 2 | 0.1934 | 4.5625 | 1.0 | 1.0 | 0 | 0 | 0 | 0 | ⚠ low-N |
| jigc | sonnet-4-6 | 3 | 0.0795 | 3.4583 | 1.0 | 1.0 | 0 | 0 | 0 | 0 | ⚠ low-N |
| jigc | sonnet-5 | 3 | 0.1286 | 4.125 | 1.0 | 1.0 | 0 | 0 | 0 | 0 | ⚠ low-N |

## Pre-registered win (per model)

- **opus-4-8: indeterminate** — missing arm(s)
- **sonnet-4-6: WIN** (cheaper than plain=True, completeness ≥ static=True)
- **sonnet-5: WIN** (cheaper than plain=True, completeness ≥ static=True)

*control✗ = control-edge violations (must be 0); oracle≠ = path-a/path-b disagreements (must be 0). Either > 0 voids the cell — an integrity tripwire, not a result.*
