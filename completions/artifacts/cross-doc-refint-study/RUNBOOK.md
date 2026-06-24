# Cross-doc ref-integrity study — RUNBOOK

Everything here is **built and verified with zero API cost**. Only steps 3–4 spend money
and need a human (the OAuth auth protocol). The canonical harness is `harness/` in this
directory; it is copied into `~/lh-study/` to run (twin workspace is throwaway).

## State at handoff (all green, no API spent)

- **Step 1** — store-wide `ref-resolves` (4th store-sweep family), engine commit `55b7a1c`;
  fires in the installed binary.
- **Oracle** `harness/measure-refint.py` — selftest OK; path-a/path-b **agree** on the live seed.
- **Hook** `harness/blocking-pre-commit-refint` — verified in-container: blocks on a dangle,
  passes clean, recovers after a ref-fix.
- **Seed** `harness/build-seed.sh` — 8 ADRs + 1 arch-doc, 7 forward edges, 1 never-touched
  control; certifies clean (`jigc validate` 0 findings, oracle 0 dangling).
- **Ladder** `harness/build-arms-refint.py` → `arms/C40|C160|C550-CLAUDE.md` (35/149/444
  lines), cross-ref rule body **byte-identical** across all three.
- **Templates** `harness/build-templates-refint.sh` — 5 arms built; `docs/` byte-identical
  across all of them.
- **Runner** `harness/run-sequence-refint.sh` + **matrix** `harness/run-matrix-refint.sh` +
  **eval** `harness/eval-sequence-refint.py` — plumbing verified no-API; the predicted
  contrast already shows (arm P ships the dangle `n_dangling=1`; arm A's hook blocks it,
  `n_dangling=0`).

## Step 0 — (re)build the twin workspace (no API)

```sh
S=~/Projects/gherrink-jigc/completions/artifacts/cross-doc-refint-study
# toolchain image (rebuild only if missing/stale):
docker build -f ~/lh-study/docker/Dockerfile.toolchain -t lh-toolchain ~/lh-study
# ladder + templates (arm A needs the installed jigc):
python3 "$S/harness/build-arms-refint.py" ~/lh-study/arms "$S/arms"
bash "$S/harness/build-templates-refint.sh"     # prints "docs/ identical across all 5 arms"
python3 "$S/harness/measure-refint.py" --selftest
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

Run ONE Sonnet arm-A sequence and hand-check the transcript vs the parsed record before
trusting the matrix (the Phase-3 honesty boundary):

```sh
S=~/Projects/gherrink-jigc/completions/artifacts/cross-doc-refint-study
STUDY="$S" bash "$S/harness/run-sequence-refint.sh" A claude-sonnet-4-6 ~/lh-study/runs/cert-A
cat ~/lh-study/runs/cert-A/sequence.json    # dangling_curve should stay ~0 (hook holds)
# spot-check one edit: less ~/lh-study/runs/cert-A/edit3/transcript.jsonl
```

## Step 3 — the matrix (paid). Split by model for auth safety.

**Sonnet first** (carries the verdict; one token window held it last study):
```sh
S=~/Projects/gherrink-jigc/completions/artifacts/cross-doc-refint-study
MODELS=sonnet CONC=3 STUDY="$S" bash "$S/harness/run-matrix-refint.sh"   # 15 sequences
```
**Then re-login, then Opus** (the cost-tax arm; small batches so an expiry truncates one
sequence, not five):
```sh
MODELS=opus CONC=1 STUDY="$S" bash "$S/harness/run-matrix-refint.sh"     # 4 sequences
```
After each batch, integrity-check: grep transcripts for `authentication_failed` and any
`turns==1,cost==0` dead edits; discard + re-run any 401-corrupted sequence on fresh auth.

## Step 4 — aggregate + judge + verdict

```sh
S=~/Projects/gherrink-jigc/completions/artifacts/cross-doc-refint-study
python3 "$S/harness/eval-sequence-refint.py" ~/lh-study/runs/refint-matrix \
  --md "$S/results.md"
# blind judge (reuse ~/lh-study/harness/judge.py); CAVEAT: give the judge the explicit
# doc-id list and ask only "does any supersedes:/cites: value name a doc NOT in this list"
# (it mis-flags lowercase {#slug} ids as symbols otherwise). Objective oracle stays canonical.
```
Then write `VERDICT.md`: capability-gap win vs salience-decay vs hypothesis-refuted (§11 of
the pre-registration). Pass/fail tripwires in the eval: **control✗ must be 0** and
**oracle≠ must be 0** (else the run is void, not a result).
