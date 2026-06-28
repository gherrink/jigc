# M35 CLI-owned-rename cost+completeness study — RUNBOOK

Everything in **steps 0–handoff is built and verified with ZERO API cost** (jigc is a local
CLI, not an API). Only **steps 3–4 spend money and need a human** (the OAuth auth protocol) —
those are the **post-build human acceptance run**, the settled design-now / run-post-build
decision for M35 ([DECISIONS.md](../../../DECISIONS.md) → 2026-06-28 M35 planning, *Acceptance*;
[pre-registration.md](pre-registration.md) → Status). The canonical harness is `harness/` in
this directory; it is copied into a throwaway twin workspace (`~/lh-study/`, the cross-doc
study's reused container scaffold) to run.

## State at handoff (all green, no API spent)

Verified 2026-06-28 from a clean checkout. The single gate is **`harness/check-handoff.sh`**
(it runs (A) the pre-registration structural check + (B) every T1–T4 no-API
selftest/verification, and exits 0 iff all green):

```sh
cd ~/Projects/gherrink-jigc
cargo build                                       # the handoff bar needs the M35 `jigc rename` binary
bash completions/artifacts/M35/harness/check-handoff.sh
# expect: "check-handoff.sh OK — pre-registration complete + every T1-T4 verification green, zero API spent"
```

What that gate asserts, piece by piece (each independently runnable, no API):

- **T1 — fixed inputs** `harness/check-seed.sh`: `build-seed.sh` writes the 9-adr + 1-arch-doc
  seed (8 forward edges, 1 never-touched control, all `schema-version: 1` stamped); the
  arm-agnostic oracle reports **0 dangling**; a `jigc setup`+`ingest`'d copy reports
  `jigc validate` **0 findings**; and the **non-greppable construction holds** — edit 4's
  prompt (`prompts/edit-4.txt`) **withholds** the target slug `sticky-lb-affinity` (the check
  FAILS if it leaks) while `sequence.json` records the structured `arch-doc …#cites → adr:…`
  referrer at ≥2 hops.
- **T2 — the three arms** `harness/build-templates-refint.sh` (needs `JIGC=…` with the M35
  `rename` verb; `check-handoff.sh` auto-resolves `target/debug/jigc`): exactly **3 arm
  templates** (jigc/static/plain), **`docs/` byte-identical** across all three, the jigc arm
  exposes `jigc rename` + carries the **native Inc-2 backstop hook**, the static arm carries
  the real `git mv` + `sd`/`grep` **one-liner** (not a straw).
- **T3 — verb-engagement extractor** `harness/analyze.py --selftest` (in-repo copy; the
  `$HOME/lh-study` dependency is internalized): 5 transcript fixtures incl. one using
  `jigc rename` (`rename_engaged: true`) and one renaming by hand (`rename_engaged: false`) —
  proving the field distinguishes verb-use from bypass.
- **T4 — metrics + aggregation** `harness/measure-refint.py --selftest` (dangling +
  **completeness** over structured managed refs only, incl. the non-greppable-referrer case)
  and `harness/eval-sequence-refint.py --selftest` (cost/turns-per-rename headline, completeness,
  rename-engagement rate, error→recovery, the `control✗`/`oracle≠` void-tripwires, the
  pre-registered win logic, order-invariance).
- **T5 — this pre-registration** `pre-registration.md` carries the three arms, the headline
  + dangling (structured managed refs only) + completeness-on-non-greppable metrics, the
  pre-registered win, the confounds, and the void-tripwires — asserted by `check-handoff.sh`'s
  structural grep over the emitted headings.

## Step 0 — (re)build the twin workspace (no API)

```sh
S=~/Projects/gherrink-jigc/completions/artifacts/M35
# toolchain image (rebuild only if missing/stale — reuses the cross-doc study's docker scaffold):
docker build -f ~/lh-study/docker/Dockerfile.toolchain -t lh-toolchain ~/lh-study
# arm templates (the jigc arm needs the installed M35 jigc with `rename`):
JIGC=$(command -v jigc) bash "$S/harness/build-templates-refint.sh"   # prints "docs/ identical across all 3 arms"
python3 "$S/harness/measure-refint.py" --selftest
python3 "$S/harness/analyze.py"        --selftest
python3 "$S/harness/eval-sequence-refint.py" --selftest
```

## Step 1 — verify auth is LIVE immediately before any paid run (the cheap probe)

```sh
docker run --rm -v $(readlink -f ~/.local/bin/claude):/usr/local/bin/claude:ro \
  -v ~/.claude/.credentials.json:/home/node/.claude/.credentials.json:ro \
  lh-toolchain bash -c 'claude -p "say OK" --model claude-haiku-4-5-20251001 \
  --permission-mode bypassPermissions --output-format json 2>&1' | grep -oE '"is_error":[a-z]+'
# expect "is_error":false  — if not, re-login and retry.
```

## Step 2 — single live certification (≈1 sequence, small spend)

Run ONE Sonnet jigc-arm sequence and hand-check the transcript vs the parsed record before
trusting the matrix (the Phase-3 honesty boundary):

```sh
S=~/Projects/gherrink-jigc/completions/artifacts/M35
STUDY="$S" bash "$S/harness/run-sequence-refint.sh" jigc claude-sonnet-4-6 ~/lh-study/runs/cert-jigc
cat ~/lh-study/runs/cert-jigc/sequence.json    # cost_per_rename low, rename_engaged true on the rename edits
# spot-check the non-greppable edit: less ~/lh-study/runs/cert-jigc/edit4/transcript.jsonl
```

## Step 3 — the matrix (PAID — the post-build human run). Split by model for auth safety.

**Sonnet first** (carries the verdict):
```sh
S=~/Projects/gherrink-jigc/completions/artifacts/M35
MODELS=sonnet CONC=3 STUDY="$S" bash "$S/harness/run-matrix-refint.sh"
```
**Then re-login, then Opus** (the cost-tax arm; small batches so an expiry truncates one
sequence, not many):
```sh
MODELS=opus CONC=1 STUDY="$S" bash "$S/harness/run-matrix-refint.sh"
```
After each batch, integrity-check: grep transcripts for `authentication_failed` and any
`turns==1,cost==0` dead edits; discard + re-run any 401-corrupted sequence on fresh auth.

## Step 4 — aggregate + judge + verdict (the post-build human deliverable)

```sh
S=~/Projects/gherrink-jigc/completions/artifacts/M35
python3 "$S/harness/eval-sequence-refint.py" ~/lh-study/runs/rename-matrix \
  --md "$S/results.md"
```
Then write **`VERDICT.md`** ruling on the pre-registered win
([pre-registration.md](pre-registration.md) §3): **WIN** iff jigc is strictly cheaper per
rename than plain **AND** jigc completeness ≥ static — else the honest alternative (cost-win
refuted, or engagement-failure reported as routing not mechanism). **The void-tripwires gate
the verdict:** `eval-sequence-refint.py` withholds the verdict when **`control✗ > 0`** or
**`oracle≠ > 0`** (the run is void, not a result — re-run, do not massage). The verdict is
recorded under `completions/artifacts/M35/`.
