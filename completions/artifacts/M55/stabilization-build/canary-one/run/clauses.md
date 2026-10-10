# canary-one — the closing condition, clause by clause

One row per clause of the closing condition, RENDERED by `dev/stabilize-record` from the results of the test set's items (each round's `results.md`) and written by no caller, never by hand: the items that judge the clause and where each one's run stands, the commit and the round of the latest of those runs, and `green`, `red` or `void` — not reached, could not run, or no longer current. A void row forbids closing; a row that differs from what the results give, or is missing, is refused the next time the state is read.

| clause | instrument | last commit | scope | status |
|---|---|---|---|---|
| `no-lost-files` | `row-doc-list` — green (round 1, attempt 1)<br>`cross-cutting` — green (round 1, attempt 1) | `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` | round 1 | green |
| `working-product` | `gate` — green (round 1, attempt 1)<br>`regression-set` — green (round 1, attempt 1) | `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` | round 1 | green |
| `usable-by-agents` | `arm-control` — green (round 1, attempt 1) | `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` | round 1 | green |
| `migration-works` | - | - | - | void |
