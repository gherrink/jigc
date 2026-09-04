# Runbook — driving the sessions

Ordered. **Arm 00 first, always** ([protocol.md](protocol.md) §3.5). If the control does not
fire, no blind result may be read. Every round is gated by
`python3 completions/trial-driver/run.py gate completions/artifacts/RC-m50/gate-rc13.json --tag jigc-gate:rc13`
— a rebuilt tag is a different image and the record no longer covers it.

## 0 · The control

```sh
python3 completions/trial-driver/walk.py ~/ideas/walk-m50 ~/out/M50-walk --tag jigc-gate:rc13 --only 00
```

## 1 · The headless blind arms — driven by the rig

```sh
# B3 — plant E + the foreign-ADR poller (state-triggered, from outside the session)
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-m50/paste/b3-prompt.txt --cid-file /tmp/b3-cid.txt \
    ~/ideas/ashgrove-planted ~/out/B3 jigc-gate:rc13
python3 completions/trial-driver/run.py plant /tmp/b3-cid.txt \
    completions/artifacts/RC-1.0-gate/plants/b3-foreign-adr.sh \
    --when 'cd /work && [ "$(git rev-list --count $(git log --format=%H --grep="install jigc workspace config" -n1)..HEAD)" -ge 2 ]'

# B3-h2 — the second headless reading, same shape, fresh corpus
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-m50/paste/b3-prompt.txt --cid-file /tmp/b3h2-cid.txt \
    ~/ideas/brackenmoor-planted ~/out/B3-h2 jigc-gate:rc13
python3 completions/trial-driver/run.py plant /tmp/b3h2-cid.txt \
    completions/artifacts/RC-1.0-gate/plants/b3-foreign-adr.sh --when '<same>'

# B4-h — plan the first milestone; no plant
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-m50/paste/b4-prompt.txt \
    ~/ideas/saltmarsh-adopted ~/out/B4-h jigc-gate:rc13

# B3-strict — unscored, the adopter's condition
./completions/trial-harness/run-session.sh --headless --strict-permissions \
    --prompt-file completions/artifacts/RC-m50/paste/b3-prompt.txt \
    ~/ideas/ashgrove-strict ~/out/B3-strict jigc-gate:rc13
```

## 2 · B1 and B2 — a human at the keyboard

[OPERATOR-STEPS.md](OPERATOR-STEPS.md) is the script. Answer only from
[answer-key.md](answer-key.md); a question matching nothing is a **halt**, not an invitation.
Log every utterance into [operator-log.md](operator-log.md) **as it happens**. The feedback prompt
goes **last**.

```sh
./completions/trial-harness/run-session.sh ~/ideas/quillon-planted ~/out/B2 jigc-gate:rc13
./completions/trial-harness/run-session.sh --cid-file /tmp/b1-cid.txt ~/ideas/larkspur ~/out/B1 jigc-gate:rc13
```

## 3 · Scoring

```sh
python3 completions/trial-driver/run.py observe --archive        # the reader's own control first
python3 completions/trial-driver/run.py observe ~/out/B1 ~/out/B2 ~/out/B3 ~/out/B3-h2 ~/out/B4-h
```

Read the row, not the exit code — a halt looks exactly like success.

## 4 · The rest of the walk, then the migration pair

```sh
python3 completions/trial-driver/walk.py ~/ideas/walk-m50 ~/out/M50-walk --tag jigc-gate:rc13
# the two-binary pair, three commands, carry between them:
python3 completions/trial-driver/walk.py ~/ideas/walk-m50-mig ~/out/M50-14 --tag jigc-gate:rc12 --only 14
python3 completions/trial-driver/run.py  carry ~/out/M50-14/14-migrate-author-on-rc12 ~/ideas/walk-m50-mig-b
python3 completions/trial-driver/walk.py ~/ideas/walk-m50-mig-b ~/out/M50-21 --tag jigc-gate:rc13 --only 21
```

Arms 03 and 10 **skip loudly** on this pass (they are the rc.11 → rc.12 pair); the record shows
them skipped, not absent.
