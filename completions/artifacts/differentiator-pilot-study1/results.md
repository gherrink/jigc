# Differentiator pilot — Study 1 per-cell results (5 arms)

Outcome columns: `Rd` = old-symbol refs left in README; `Aanchor`/`Atitle`/`Aprose`
= old-symbol refs left in the arch-doc's anchor / `### title` / prose;
`drift` = any nonzero. Arm E (`bigstatic`) added post-hoc to test the dilution
confound: the SAME doc-rule as arm C, embedded verbatim in a realistic 157-line
CLAUDE.md (1 of 14 sections, ~23%, vs arm C's single-purpose 42-line / 100%).

| task | arm | model | turns | cost | Rd | Aanchor | Atitle | Aprose | drift |
|---|---|---|---|---|---|---|---|---|---|
| task1 | plain | sonnet | 19 | $0.31 | 4 | 1 | 1 | 1 | **DRIFT** |
| task1 | plain | opus | 32 | $0.72 | 0 | 0 | 0 | 0 | clean |
| task1 | static | sonnet | 33 | $0.46 | 0 | 0 | 0 | 0 | clean |
| task1 | static | opus | 34 | $0.8 | 0 | 0 | 0 | 0 | clean |
| task1 | bigstatic | sonnet | 30 | $0.54 | 0 | 0 | 0 | 0 | clean |
| task1 | bigstatic | opus | 33 | $0.81 | 0 | 0 | 0 | 0 | clean |
| task1 | gsd | sonnet | 16 | $0.24 | 4 | 1 | 1 | 1 | **DRIFT** |
| task1 | gsd | opus | 27 | $0.72 | 0 | 0 | 0 | 0 | clean |
| task1 | jigc | sonnet | 17 | $0.29 | 4 | 1 | 1 | 1 | **DRIFT** |
| task1 | jigc | opus | 44 | $1.61 | 0 | 0 | 1 | 0 | **DRIFT** |
| task2 | plain | sonnet | 13 | $0.16 | 2 | 1 | 1 | 1 | **DRIFT** |
| task2 | plain | opus | 19 | $0.51 | 0 | 0 | 0 | 0 | clean |
| task2 | static | sonnet | 18 | $0.24 | 0 | 0 | 0 | 0 | clean |
| task2 | static | opus | 25 | $0.7 | 0 | 0 | 0 | 0 | clean |
| task2 | bigstatic | sonnet | 25 | $0.4 | 0 | 0 | 0 | 0 | clean |
| task2 | bigstatic | opus | 27 | $0.62 | 0 | 0 | 0 | 0 | clean |
| task2 | gsd | sonnet | 11 | $0.22 | 2 | 1 | 1 | 1 | **DRIFT** |
| task2 | gsd | opus | 22 | $0.76 | 0 | 0 | 0 | 0 | clean |
| task2 | jigc | sonnet | 19 | $0.28 | 0 | 0 | 0 | 0 | clean |
| task2 | jigc | opus | 42 | $1.64 | 0 | 0 | 0 | 0 | clean |
