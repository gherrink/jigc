# Long-horizon many-edit study

The study the pilot ([../differentiator-pilot-study1/](../differentiator-pilot-study1/))
and three prior ties (M17–M19) named as the **one remaining shot** at jigc's empirical
superiority claim: does jigc make a coding agent *measurably more correct over many
edits* than a static `CLAUDE.md`, in the regime they all excluded — **many edits, fresh
cold agent each, evolving repo, salience-independent enforcement vs salience-dependent
instruction**?

## Read in this order

1. **[VERDICT.md](VERDICT.md)** — the ruling: *qualified yes* (first controlled
   blind-confirmed jigc-beats-static win, for Sonnet) and *self-refuting for Opus*
   (the prose blind spot, made decisive).
2. **[results.md](results.md)** — the cumulative-drift curves, the three diagnosed
   mechanisms, the probe-vs-judge reconciliation, data-integrity note.
3. **[pre-registration.md](pre-registration.md)** — the protocol, fixed before any run,
   with the build-time amendments recorded.

## What's here

- `harness/` — the reusable sequence harness (the handover's named deliverable):
  - `run-sequence.sh` — one rep: fresh cold agent per edit over an **evolving** twin.
  - `run-matrix.sh` — the full bounded-concurrency matrix.
  - `measure.py` — arm-agnostic drift oracle: drives the real `doc-code` probe over the
    doc's anchors (dangling) + dead-symbol grep over titles/prose (stale). `--selftest`.
  - `eval-sequence.py` — aggregates sequences → cumulative-drift curves + rates.
  - `judge.py` — blind cross-model judge (Codex), stdin + `--skip-git-repo-check`.
  - `analyze.py` — the workflow-eval behavioral analyzer (engage/finalize/cost), reused.
  - `build-arm-A.sh` / `build-templates.sh` — twin builders.
  - `blocking-pre-commit` — the **blocking** hook variant (1-line delta from jigc's
    shipped warn-only `precommit_hook_body`: `exit 1` on a `doc-code` finding).
  - `sequence.json` — the 8-edit manifest (rename/move/delete on real symbols).
- `arms/` — the dilution ladder (`C40`/`C160`/`C550` `CLAUDE.md`, rule byte-identical
  across all three) + the managed `arch-doc` under test.
- `prompts/` — the 8 byte-identical neutral tickets.
- `runs/sequences/` — per-sequence rollups; `runs/matrix-tables.md` — the raw aggregate.
- `judge/judge-results.json` — the blinded Codex verdicts + unblinding map.

## Reproduce

Twin source = `gherrink-ui-doc @ 542b320`. Build the templates, then run:

```sh
ROOT=~/lh-study bash harness/build-arm-A.sh        # jigc twin + managed arch-doc + hook
ROOT=~/lh-study bash harness/build-templates.sh    # C40/C160/C550/P twins (byte-identical doc)
python3 harness/measure.py --selftest              # oracle self-check, no API
bash harness/run-matrix.sh                         # the full matrix (Docker + API)
python3 harness/eval-sequence.py runs/matrix --md runs/results.md
python3 harness/judge.py runs/matrix --out runs/judge-results.json
```

Container gotchas (from the pilot RUNBOOK) are baked into `run-sequence.sh`: `USER node`,
`--permission-mode bypassPermissions`, prompt via `-e PILOT_PROMPT`, glibc ≥ 2.39 base.

## The one-line result

Sonnet: **A flat at 0 drift; plain → 5.0; every static arm ships residual move-path
drift it structurally can't see (6/9 reps); A 0/3.** Opus: **A's hook keeps anchors
clean but Opus leaves stale titles (judge 0/2) at 3× cost; static-Opus clean (2/2).**
Win is real, regime-bound, and points at one fix: **anchor the component titles.**
