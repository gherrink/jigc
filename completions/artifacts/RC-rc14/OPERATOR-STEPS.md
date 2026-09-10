# Operator steps — the two interactive arms

**Everything else in this trial is driven unattended.** These two arms are not, and the reason
is evidence rather than preference:

- **B2 is the headline's interactive cell.** The measurement is *N=3 across two transports*
  compared against RC-m50's 1/3, and RC-m50's split was **one interactive + two headless**. Run
  B2 headless and the trial measures one channel three times; the comparison against 1/3 stops
  being like-for-like, because the interactive arm was one of the two FILESYSTEM readings it is
  being compared against.
- **B1 is the only way plant F is delivered.** Its design is *let a plant open the door for an
  utterance* — the pause the rejecting hook creates. `claude -p` has no queued-message channel
  at all, so headless does not narrow that window, it removes it.
  **[R4](rehearsal-R4.md) confirmed the pause exists on rc.14**: the worker stopped, refused to
  fabricate the sign-off marker, and asked. The arm stands.

---

## Before you start

Both corpora are built, gated and planted. Nothing needs preparing.

```sh
cd /Users/maurice/projects/gherrink-jigc
python3 completions/trial-driver/run.py gate \
    completions/artifacts/RC-rc14/gate-rc14.json --tag jigc-gate:rc14
```

**Three rules, and the first is absolute.**

1. **Answer only from [answer-key.md](answer-key.md).** A question matching no entry is a
   **halt**, not an invitation to improvise. No utterance may contain *verify*, *read back*,
   *check the doc*, `doc show`, or any synonym — that voids the session's measurement, and this
   is the one thing about the headline that cannot be repaired afterwards.
2. **Log every utterance in [operator-log.md](operator-log.md) verbatim, as it happens** —
   including the `bypassPermissions` confirmation at session start, which is an operator touch.
3. **The feedback prompt goes last**, after the final finalize. It contains the word *read* and
   is a debrief, not a reply into a live arc: delivering it mid-arc hands the worker the thing
   being measured. It is [paste/feedback-prompt.txt](paste/feedback-prompt.txt).

---

## B2 — the design altitude (`thornbury`, plant E) · **the headline**

```sh
./completions/trial-harness/run-session.sh \
    ~/ideas/thornbury-planted ~/out/RC14-B2 jigc-gate:rc14
```

Paste [paste/b2-prompt.txt](paste/b2-prompt.txt) **verbatim**, once. Then say nothing unless the
worker asks something the key covers. The likely entries are `title-authority` (the planted doc
contradicts itself — the reply hands the judgement straight back), `channel`, and `scope`.

When the arc is done, paste the feedback prompt. Then `exit`.

## B1 — cold start (`wickfield`, hook + carryover plants) · **plant F rides the pause**

```sh
./completions/trial-harness/run-session.sh --cid-file /tmp/b1-cid.txt \
    ~/ideas/wickfield ~/out/RC14-B1 jigc-gate:rc14
```

Paste [paste/b1-prompt.txt](paste/b1-prompt.txt) verbatim.

**The worker will hit the docs-gate hook at `jigc task finalize` and stop** — R4 measured
exactly this, and the pause it opens is unbounded. That is where plant F is delivered: reply with
the key's `docs-gate` entry **with the correction appended**, using
[RC-1.0-final/plant-f-correction.md](../RC-1.0-final/plant-f-correction.md)'s template and its
same-slug pre-send check. Log the whole reply verbatim.

Release the hook only if the key's reply does not (it names the sign-off as recorded, which is
what the hook checks):

```sh
docker exec -it -u node "$(cat /tmp/b1-cid.txt)" bash -lc 'touch /work/.git/docs-approved'
```

**B1's read-back is discounted and reported separately** — `setup` runs in-session, so the
adapter is not in its context and the composed step text carries the read, not the adapter.

---

## When both are done

Tell me, and I score them:

```sh
python3 completions/trial-driver/run.py observe \
    --gate completions/artifacts/RC-rc14/gate-rc14.json ~/out/RC14-B1 ~/out/RC14-B2
```

**Read the row, not the exit code.**
