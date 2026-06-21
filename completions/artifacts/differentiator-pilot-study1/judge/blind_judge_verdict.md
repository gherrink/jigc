# Blind judge verdict (same-family, arm labels hidden)

Method: a fresh Claude context received only the 16 coded samples (S01..S16) in
`blind_input.txt` — final docs + code-rename status, no arm/method information —
and judged doc↔code consistency per sample. Decoded against `mapping.json` AFTER
judging. NOTE: the pre-registered cross-model judge (Codex) was quota-blocked;
this same-family blind judge is corroboration of the objective grep measure.

| Sample | code_rename | verdict | drift_location | (decoded) arm/model/task |
|---|---|---|---|---|
| S01 | yes | CLEAN | none | gsd / opus / task1 |
| S02 | yes | CLEAN | none | static / opus / task2 |
| S03 | yes | CLEAN | none | static / sonnet / task1 |
| S04 | yes | CLEAN | none | jigc / sonnet / task2 |
| S05 | yes | DRIFT | arch-title | jigc / opus / task1 |
| S06 | yes | DRIFT | README,arch-anchor,arch-title | plain / sonnet / task1 |
| S07 | yes | CLEAN | none | plain / opus / task2 |
| S08 | yes | CLEAN | none | static / opus / task1 |
| S09 | yes | DRIFT | README,arch-anchor,arch-title | gsd / sonnet / task2 |
| S10 | yes | DRIFT | README,arch-anchor,arch-title | jigc / sonnet / task1 |
| S11 | yes | CLEAN | none | jigc / opus / task2 |
| S12 | yes | CLEAN | none | plain / opus / task1 |
| S13 | yes | DRIFT | README,arch-anchor,arch-title | plain / sonnet / task2 |
| S14 | yes | DRIFT | README,arch-anchor,arch-title | gsd / sonnet / task1 |
| S15 | yes | CLEAN | none | static / sonnet / task2 |
| S16 | yes | CLEAN | none | gsd / opus / task2 |

SUMMARY: 10 CLEAN, 6 DRIFT of 16 — cell-for-cell identical to the mechanical grep
measure. Drift by arm: static 0/4, plain 2/4, gsd 2/4, jigc 2/4 (task1-sonnet +
task1-opus).
