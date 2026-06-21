# Differentiator pilot — Study 1 (clean-baseline A/B/C/D)

Reversible twin pilot executed 2026-06-21 to clear the **value gate** the
[readiness assessment](../../../READINESS-ASSESSMENT.md) named: *does jigc beat a
static `CLAUDE.md` when a differentiator (doc↔code consistency) is actually
engaged?* Spec home: [`ideas/differentiator-pilot.md`](../../../ideas/differentiator-pilot.md);
protocol home: [`design/measurement.md`](../../../design/measurement.md).

## Read in this order

1. **[VERDICT.md](VERDICT.md)** — the ruling: **jigc did not beat static; the
   static-`CLAUDE.md` arm won.** Three diagnosed mechanisms.
2. **[ADDENDUM-jigc-enforced.md](ADDENDUM-jigc-enforced.md)** — folding the fixes back in: an always-on validation hook brings jigc to **4/4, parity with static** (was 2/4).
3. **[pre-registration.md](pre-registration.md)** — the protocol, fixed before any run
   (arms, tasks, prompts, acceptance, judging, design deviations from the handover).
3. **[results.md](results.md)** — the full 16-cell per-cell outcome table.
4. **[LESSONS.md](LESSONS.md)** — what we learned, the arm-C **dilution confound**, the reframe (jigc's value is cumulative/dilution-proof, not single-task), and concrete ways to beat/draw C.
5. **[RUNBOOK.md](RUNBOOK.md)** — how to rebuild + re-run the whole matrix later.
6. **[judge/blind_judge_verdict.md](judge/blind_judge_verdict.md)** — independent
   blind judge, cell-for-cell identical to the mechanical measure.

## Headline

4 arms (A jigc · B GSD · C static-methodology · D plain) × 2 models (Sonnet 4.6,
Opus 4.8) × 2 tasks (public-API class renames) = 16 isolated container-twin runs of
`gherrink-ui-doc @ 542b3206`. Outcome = did the run ship a doc↔code drift?

| Arm | Clean / 4 |
|---|---|
| C · static-methodology (42-line file) | **4/4** |
| E · static-methodology (157-line realistic file) | **4/4** |
| D · plain | 2/4 |
| B · GSD (dormant on bare prompts) | 2/4 |
| A · jigc | 2/4 |

Arm E was added to test the *dilution confound* (does the doc-rule lose force when
buried in a realistic many-rule file?). It did **not** — same 4/4 as C. So the
defense of jigc narrows to heavy dilution + the long-horizon many-edit regime,
both still untested. See LESSONS.md §3.

jigc tied the unguided baselines and lost to the static file, because (1) the
weaker model **bypassed the adapter**, (2) jigc's check has a **prose blind spot**,
(3) when bypassed, jigc only adds **friction** (blocks correct OOB edits). The
seeded instrument check passed — the apparatus is sound; the result is a real
negative. **Implication: do not commit a productive corpus; the value gate is not
cleared.**

## Layout

- `arms/` — the static-methodology `CLAUDE.md` (arm C) + the managed arch-doc (arm A's anchored doc, byte-identical across all arms).
- `prompts/` — the two byte-identical task prompts.
- `harness/` — Dockerfiles (4 arm images on a glibc-2.41 trixie base) + run scripts.
- `judge/` — blind judging input, the (kept-secret-until-decode) arm mapping, the blind verdict, the Codex-quota note.
- `cells/<task>/<arm>-<model>/` — raw per-run capture: `transcript.json`, `changes.diff`, `README.after.md`, `arch-doc.after.md`, `commits.txt`, `jigc-validate.after.txt`, `base.sha`.
- `MANIFEST.sha256` — content hashes of every exported file.

## Reversibility

Everything ran on throwaway `~/diff-pilot` container twins; nothing touched the
real `gherrink-ui-doc` or jigc's own repo. These exported plain files are the only
durable trace. Zero corpus lock-in.
