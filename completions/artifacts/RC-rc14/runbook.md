# Runbook — driving this trial

Ordered. **Walk arm 00 first, always** ([protocol.md](protocol.md) §3.5): if the control does
not fire, no blind result may be read. Every round is gated:

```sh
python3 completions/trial-driver/run.py gate \
    completions/artifacts/RC-rc14/gate-rc14.json --tag jigc-gate:rc14
```

A rebuilt tag is a different image and the record no longer covers it.

**Two standing rules this trial adds** (both from [pre-trial-findings.md](pre-trial-findings.md)):

- **Every out-dir is prefixed `RC14-`.** `~/out/<name>` is shared across trials and currently
  holds RC-1.0-gate's `B1`/`B2`/`B3` beside RC-m50's. `observe --gate` now refuses a stale
  directory rather than scoring it, but the prefix is what stops the collision arising.
- **Pass `--tag jigc-gate:rc14` explicitly, everywhere.** Every driver default is still
  `jigc-gate:rc11`; only `verify-pair.sh` tracks the current trial.

## 0 · The control

```sh
python3 completions/trial-driver/walk.py ~/ideas/walk-rc14 ~/out/RC14-walk \
    --tag jigc-gate:rc14 --only 00
```

## 1 · The rehearsals — preconditions, not options

```sh
# R3 — plant E against a live agent on the binary under test
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-rc14/paste/e-rehearsal-prompt.txt \
    ~/ideas/elmsworth-planted ~/out/RC14-R3 jigc-gate:rc14

# R4 — does a worker meeting the rejecting hook STOP and ask, or self-serve the marker?
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-rc14/paste/b1-prompt.txt \
    --cid-file /tmp/r4-cid.txt ~/ideas/clayforth ~/out/RC14-R4 jigc-gate:rc14
```

If R4 shows the stop unreliable, **B1's utterance arm converts to observation-only** — a
pre-registered branch, not a salvage.

## 2 · The headless arms

```sh
# B3-h2 — the second headless reading of the duress cell
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-rc14/paste/b3-prompt.txt \
    ~/ideas/oakhurst-planted ~/out/RC14-B3-h2 jigc-gate:rc14

# B3 — plant E plus the foreign-ADR plant, fired on a STATE polled from outside
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-rc14/paste/b3-prompt.txt \
    --cid-file /tmp/b3-cid.txt \
    ~/ideas/marlowe-planted ~/out/RC14-B3 jigc-gate:rc14
python3 completions/trial-driver/run.py plant /tmp/b3-cid.txt \
    completions/artifacts/RC-1.0-gate/plants/b3-foreign-adr.sh \
    --when 'cd /work && [ "$(git rev-list --count $(git log --format=%H --grep="adopt jigc for document management" -n1)..HEAD)" -ge 2 ]'

# B4-h — the milestone arm; no plant
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-rc14/paste/b4-prompt.txt \
    ~/ideas/redbourne-adopted ~/out/RC14-B4-h jigc-gate:rc14

# B3-strict — unscored, the adopter's real condition
./completions/trial-harness/run-session.sh --headless --strict-permissions \
    --prompt-file completions/artifacts/RC-rc14/paste/b3-prompt.txt \
    ~/ideas/marlowe-strict ~/out/RC14-B3-strict jigc-gate:rc14
```

## 3 · B1 and B2 — a human at the keyboard

```sh
./completions/trial-harness/run-session.sh ~/ideas/thornbury-planted ~/out/RC14-B2 jigc-gate:rc14
./completions/trial-harness/run-session.sh --cid-file /tmp/b1-cid.txt \
    ~/ideas/wickfield ~/out/RC14-B1 jigc-gate:rc14
```

Answer **only** from [answer-key.md](answer-key.md); a question matching nothing is a **halt**,
not an invitation. Log every utterance in [operator-log.md](operator-log.md) **as it happens**,
verbatim, including the permission-mode confirmation at session start. The feedback prompt goes
**last** — it contains the word *read* and is a debrief, not a reply into a live arc.

## 4 · Scoring

```sh
python3 completions/trial-driver/run.py observe --archive      # the reader's own control, FIRST
python3 completions/trial-driver/run.py observe \
    --gate completions/artifacts/RC-rc14/gate-rc14.json \
    ~/out/RC14-B1 ~/out/RC14-B2 ~/out/RC14-B3 ~/out/RC14-B3-h2 ~/out/RC14-B4-h
```

**Read the row, not the exit code** — a headless halt exits 0 at `subtype: success`,
`is_error: false`. The reader now also **says whose evidence it read** and refuses a directory
from another binary.

## 5 · The rest of the walk

```sh
python3 completions/trial-driver/walk.py ~/ideas/walk-rc14 ~/out/RC14-walk --tag jigc-gate:rc14
```

**Arms 03/10 and 14/21 skip loudly on this pass** — they are the rc.11→rc.12 and rc.12→rc.13
migration pairs, and [protocol.md](protocol.md) §0.4 establishes that **no migration pair is
owed here**: no doctype schema moved between rc.13 and rc.14. The record shows them skipped
rather than absent.

**The two deny-floored verbs run outside the agent.** `Bash(jigc uninstall:*)` and
`Bash(jigc milestone discard:*)` are blocked by the harness, so an in-agent arm sees a
permission denial and no jigc output at all.
