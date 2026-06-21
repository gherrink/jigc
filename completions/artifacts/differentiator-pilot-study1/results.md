# Differentiator pilot — Study 1 per-cell results (6 arms)

`Rd`=old-symbol refs in README; `Aanchor`/`Atitle`/`Aprose`=old refs in the arch-doc
anchor/title/prose. Arm E (`bigstatic`)=dilution test of arm C. Arm A′ (`jigcenforced`)
=arm A + an always-on `jigc validate` Stop hook (see ADDENDUM-jigc-enforced.md).
NOTE: a bare old-name in a *valid* anchor path (file not renamed but symbol resolves)
is NOT doc↔code drift — see the addendum's measurement note; `jigc validate` is
authoritative for the anchor.

| task | arm | model | turns | cost | Rd | Aanchor | Atitle | Aprose | jigc_validate |
|---|---|---|---|---|---|---|---|---|---|
| task1 | plain | sonnet | 19 | $0.31 | 4 | 1 | 1 | 1 |  |
| task1 | plain | opus | 32 | $0.72 | 0 | 0 | 0 | 0 |  |
| task1 | static | sonnet | 33 | $0.46 | 0 | 0 | 0 | 0 |  |
| task1 | static | opus | 34 | $0.8 | 0 | 0 | 0 | 0 |  |
| task1 | bigstatic | sonnet | 30 | $0.54 | 0 | 0 | 0 | 0 |  |
| task1 | bigstatic | opus | 33 | $0.81 | 0 | 0 | 0 | 0 |  |
| task1 | gsd | sonnet | 16 | $0.24 | 4 | 1 | 1 | 1 |  |
| task1 | gsd | opus | 27 | $0.72 | 0 | 0 | 0 | 0 |  |
| task1 | jigc | sonnet | 17 | $0.29 | 4 | 1 | 1 | 1 | doc-code-BLOCK |
| task1 | jigc | opus | 44 | $1.61 | 0 | 0 | 1 | 0 | clean |
| task1 | jigcenforced | sonnet | 20 | $0.38 | 0 | 0 | 0 | 0 | OOB/other |
| task1 | jigcenforced | opus | 57 | $2.27 | 0 | 0 | 0 | 0 | clean |
| task2 | plain | sonnet | 13 | $0.16 | 2 | 1 | 1 | 1 |  |
| task2 | plain | opus | 19 | $0.51 | 0 | 0 | 0 | 0 |  |
| task2 | static | sonnet | 18 | $0.24 | 0 | 0 | 0 | 0 |  |
| task2 | static | opus | 25 | $0.7 | 0 | 0 | 0 | 0 |  |
| task2 | bigstatic | sonnet | 25 | $0.4 | 0 | 0 | 0 | 0 |  |
| task2 | bigstatic | opus | 27 | $0.62 | 0 | 0 | 0 | 0 |  |
| task2 | gsd | sonnet | 11 | $0.22 | 2 | 1 | 1 | 1 |  |
| task2 | gsd | opus | 22 | $0.76 | 0 | 0 | 0 | 0 |  |
| task2 | jigc | sonnet | 19 | $0.28 | 0 | 0 | 0 | 0 | OOB/other |
| task2 | jigc | opus | 42 | $1.64 | 0 | 0 | 0 | 0 | clean |
| task2 | jigcenforced | sonnet | 18 | $0.22 | 0 | 0 | 0 | 0 | OOB/other |
| task2 | jigcenforced | opus | 39 | $1.58 | 0 | 1 | 0 | 0 | clean |
