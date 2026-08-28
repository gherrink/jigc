# Runbook — driving the sessions

Ordered. **Arm 00 first, always**: if the control does not fire, no blind result may be read at
all ([protocol.md](protocol.md) §3.5). A uniform null in both directions is the signature of an
apparatus that cannot discriminate, not of a finding.

## 0 · The control — DONE

```sh
python3 completions/trial-driver/walk.py ~/ideas/walk-trial ~/out/TRIAL-walk \
    --tag jigc-gate:rc12 --only 00
```

**Result: `ARM 0 PASS — the channel fires and is countable`**, 1 `doc show … --task` record
recovered from the corpus after the container was destroyed. Blind results may be read.

## 1 · B3 — headless, driven by the rig

```sh
./completions/trial-harness/run-session.sh --headless \
    --prompt-file completions/artifacts/RC-1.0-final/paste/b3-prompt.txt \
    --cid-file /tmp/b3-cid.txt \
    ~/ideas/stonecross-planted ~/out/B3 jigc-gate:rc12

# concurrently — the plant fires on a STATE polled from outside the session
python3 completions/trial-driver/run.py plant /tmp/b3-cid.txt \
    completions/artifacts/RC-1.0-gate/plants/b3-foreign-adr.sh \
    --when 'cd /work && [ "$(git rev-list --count $(git log --format=%H --grep="install jigc workspace config" -n1)..HEAD)" -ge 2 ]'
```

## 2 · B1 and B2 — **these need a human at the keyboard**

**They are interactive by design, and the design is the reason.** Neither can be driven by the
rig, and running them headless instead would be a change to a pre-registered instrument:

- **B1** carries **plant F**. Its whole mechanism is *let a plant open the door for an utterance*
  — the hook rejects the ADR-promoting finalize, the worker stops and asks, and the operator
  answers into a pause the tool created. `claude -p` has **no queued-message channel at all**, so
  headless has no door. R2 confirmed the pause is real; delivering the correction into it is the
  half that needs a person.
- **B2** carries the headline measurement (§3.2's channel under duress). The transport does not
  move under the headline, and increment 0's channel-equivalence result is n=1.

```sh
# B1 — cold start, both plants already in the corpus
./completions/trial-harness/run-session.sh ~/ideas/harbourgate ~/out/B1 jigc-gate:rc12
# B2 — design altitude, plant E already staged
./completions/trial-harness/run-session.sh ~/ideas/pinewick-planted ~/out/B2 jigc-gate:rc12
```

Paste the prompt from `paste/b1-prompt.txt` / `paste/b2-prompt.txt` **verbatim**, and nothing else.

### While a session is live

- **Answer only from [answer-key.md](answer-key.md).** A question that matches no entry is a
  **halt**, not an invitation to improvise — improvising is itself a finding about the key.
- **B1's hook question is plant F's moment.** Complete the template in
  [plant-f-correction.md](plant-f-correction.md), run its **same-slug pre-send check** (a retitle
  that derives the slug the doc already has acks *"the id is unchanged"* at exit 0 and the arm is
  silently dead), screen it, then send.
- **Log every utterance verbatim into [operator-log.md](operator-log.md) as it happens.** §8's
  contamination rule can only be audited against that log; written from recall it is not a
  purity statement.
- **The feedback prompt goes last**, after the final finalize. It contains the word *read* and
  would contaminate the measurement if delivered earlier — deliver it early and the session's
  measurement is **void**.

### B1's plant release

The hook stays rejecting until the marker exists, and the session runs on the container's copy:

```sh
docker exec -it -u node <cid> bash -lc 'touch /work/.git/docs-approved'
```

## 3 · B3-strict — unscored

A second copy of B3's corpus under `--strict-permissions`: the adopter's real condition, which no
trial has run. **Unscored and labelled so** — increment 0 measured it truncating the arc (13
records, 0 authoring writes). That is `unmeasured`, never `NEITHER`.

## 4 · Scoring

```sh
python3 completions/trial-driver/run.py observe ~/out/B1 ~/out/B2 ~/out/B3
python3 completions/trial-driver/run.py observe --archive   # the reader's own control
```

Read the row, not the exit code. **A halt looks exactly like success** — exit 0,
`subtype: success`, `is_error: false`, empty stderr — so the reader prints `HALTED` (a denial
stopped it) and `ENDED ASKING` (the product blocked it and the worker asked) separately, and
neither is visible in `$?`.

## 5 · The rest of the walk

```sh
python3 completions/trial-driver/walk.py ~/ideas/walk-trial ~/out/TRIAL-walk --tag jigc-gate:rc12
# then the upgrade pair, which needs both binaries:
python3 completions/trial-driver/walk.py ~/ideas/walk03   ~/out/TRIAL-03a --tag jigc-gate:rc11 --only 03
python3 completions/trial-driver/run.py  carry ~/out/TRIAL-03a/03-upgrade-author-on-rc11 ~/ideas/walk03b
python3 completions/trial-driver/walk.py ~/ideas/walk03b  ~/out/TRIAL-10  --tag jigc-gate:rc12 --only 10
```
