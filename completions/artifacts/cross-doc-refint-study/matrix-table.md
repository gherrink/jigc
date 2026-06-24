# Cross-doc forward-ref integrity matrix — results

> Headline: arm A (jigc hook) flat ~0 vs static curves rising (C40 ≤ C160 < C550), plain fastest = the capability-gap result.

## Final-state + behavioral rates (mean over reps)

| arm | model | reps | final dangling | edits w/ drift | tickets landed | blocked | --no-verify | control✗ | oracle≠ | cost |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| P | sonnet | 3 | 4 | 7.67 | 8.33 | 0/3 | 0/3 | 0 | 0 | $0.87 |
| C40 | sonnet | 3 | 0 | 0 | 8 | 0/3 | 0/3 | 0 | 0 | $1.22 |
| C160 | sonnet | 3 | 0 | 0 | 8 | 0/3 | 0/3 | 0 | 0 | $1.32 |
| C550 | opus | 2 | 0 | 0 | 8 | 0/2 | 0/2 | 0 | 0 | $2.67 |
| C550 | sonnet | 3 | 0 | 0 | 8 | 0/3 | 0/3 | 0 | 0 | $1.55 |
| A | opus | 2 | 0 | 0 | 8 | 2/2 | 0/2 | 0 | 0 | $7.66 |
| A | sonnet | 3 | 0 | 0 | 8 | 3/3 | 0/3 | 0 | 0 | $3.63 |

## Cumulative dangling cross-doc refs in HEAD after edit N (THE HEADLINE)

| arm | model | e1 | e2 | e3 | e4 | e5 | e6 | e7 | e8 |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| P | sonnet | 0.67 | 1 | 2 | 3 | 4 | 4 | 4 | 4 |
| C40 | sonnet | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| C160 | sonnet | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| C550 | opus | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| C550 | sonnet | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| A | opus | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| A | sonnet | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

*control✗ = control-edge violations (must be 0); oracle≠ = path-a/path-b disagreements (must be 0 — an oracle-integrity flag, not a result).*
