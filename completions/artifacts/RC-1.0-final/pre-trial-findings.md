# Pre-trial findings

Things the operator hit **while building the instrument**, recorded here so they are never
confused with trial results. A finding found by an operator writing a walk arm is not a blind
session's finding, and folding the two together would inflate the trial's yield with work the
trial did not do.

Nothing here is adjudicated against [protocol.md](protocol.md) §1. That happens when the trial
runs, with a repro block, under §7's conversion obligation.

---

## PT-A · A blocking refusal with no route, whose sibling on the same verb has one

**Status: candidate finding. Not fixed, deliberately** — fixing product surface while building
the instrument would both be scope creep and quietly remove a finding the trial could
legitimately produce.

`jigc rename` refuses two different ways, and only one of them routes.

```console
$ jigc rename adr:no-such-doc --to "Something else"          # exit 1
no managed doc `adr:no-such-doc` to rename (expected at docs/decisions/no-such-doc.md)
  route: check the id (or run `jigc describe` for the doctype surface)

$ jigc rename adr:cap-distinct-series --to "Drop the oldest sample on overflow"   # exit 1
cannot rename to `adr:drop-the-oldest-sample` — a different doc already exists at
docs/decisions/drop-the-oldest-sample.md
```

The second carries **no route line**. Measured on `1.0.0-rc.12`, and in the invocation log both
refusals record `error_code: null` and `finding_codes: []` — so the occupancy refusal is
invisible to the finding channel, invisible to the error-code registry, and carries no route.

**Why the pairing matters and a lone complaint would not.** M43's route floor binds *findings*,
and this is an anyhow error rather than a `Finding`, so the floor does not formally reach it.
But the sibling failure **on the same verb, one argument different** does carry a route. That is
the incomplete-sweep shape M45 and M46 both keep finding: a rule applied at one site and not at
its neighbour.

**What a reader can and cannot conclude.** The user is not stranded — the message names the
occupied path, which is enough to act on. So this is a surface finding, not a dead end, and
under §1 it would ship recorded rather than block. It is written down now because the trial
should not have to rediscover it, and because whether the route floor *should* extend past
`Finding` to the error-code registry is a design question this trial can inform but not settle.

**Reached by:** `arms/walk/08-identity-refusals.sh`, which drives both refusals side by side and
records the comparison rather than asserting a verdict.

---

## PT-B · Task workbench state has no read verb

**Status: observation, and it is the discoverability lens again.**

Both R1 rehearsal workers, unprompted, read task bookkeeping straight off the filesystem:
`roles.json`, `base.json`, `intent`, `workflow`, `provenance.json`, `staged-snapshot.json`.

There is no read verb for any of it. `jigc doc show --task` serves the staged *documents*;
nothing serves the task's own state. So this is a **capability gap** (§1's last row) rather than
a channel violation, and [protocol.md](protocol.md) §3.4 settlement 4 exists so the trial does
not score it as the latter.

Recorded here because it was observed **before** the trial, by workers who were not being asked
about it, which is a cleaner observation than the same thing surfacing inside a scored arm.
