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

---

## PT-C · The router's closing claim is true only under a definition the surface never states

**Status: observation, deliberately not adjudicated.** It re-opens M46 Increment 8's own audit
finding, which is why it is written carefully rather than filed as a defect.

The composed router view closes with:

> That catalog is the selectable subset. A workflow outside it is reached by name with the same
> `--workflow` form — `jigc describe --workflows` lists every workflow, hidden ones included,
> **and each hidden one carries the reason it is hidden.**

Measured against `1.0.0-rc.12`, by deriving the population from the shipped binary rather than
from the source (`arms/walk/09-increment-8-doors.sh`):

```
catalog: 12 selectable · absent: 21 · of those, WITHOUT a stated reason: 3
without a reason: increment, ingest-existing, router
```

**Those three are exactly the `creates-task: false` workflows** — which is precisely the set
M46's completion audit named when it caught this sentence's predecessor overclaiming
(*"each with the reason it sits off the catalog"* is false for them, because M43's `suppressed:`
fence binds only `selectable: false`). That finding was fixed as a law-1 scope repair, at a cost
of 18 goldens.

**Both readings, stated fairly:**

- **The sentence is fine.** The catalog is *"the available work-workflows"*. A
  `creates-task: false` workflow is not a work-workflow, so it is not *hidden* from the catalog
  — it was never a candidate for it. Under this reading "hidden" means `selectable: false`,
  every member of that set does carry its reason, and the claim holds exactly.
- **The sentence still overclaims.** A reader has just been shown a catalog and told that
  `describe --workflows` lists *"every workflow, hidden ones included"*. The natural referent of
  "hidden" is *absent from what I was just shown*. Under that reading three are hidden without a
  reason, and the repaired sentence has the same defect its predecessor had, one definition
  further back.

**What decides it is not in the output.** Nothing on either surface defines "hidden", so the
sentence's truth depends on a distinction the reader has no way to make. That is the honest
finding here — not that the claim is false, but that **it cannot be checked by the person it is
addressed to.**

Under [protocol.md](protocol.md) §1 this is at most a surface finding and would ship recorded.
It is logged now because the trial should meet it as a known measurement rather than
re-derive it, and because a blind session that trips over it will produce a *complaint*, and the
record will need this to tell a real defect from a definitional one.

**Reached by:** `arms/walk/09-increment-8-doors.sh` T6, which prints the derivation and
explicitly declines to score it.

---

## PT-D · The trial corpus has a second accidental prose↔code contradiction, and plant E sits on it

**Status: a finding about the FIXTURE, not the product.** Recorded so the sessions that follow do
not each re-derive it, and so it is never mistaken for a product result.

**Surfaced by B3**, unprompted, which found it while writing the architecture doc and reported it
rather than describing the pipeline it expected to see. **Verified independently against the
template before being believed** — an agent's report is a lead, not a measurement:

```
src/router.ts    Router.ingest → this.store.put(...)      # the queue is bypassed on the live path
src/index.ts:37  tick(): drains the queue, prunes         # nothing calls tick()
src/index.ts:45  main(): createService(), then reads store.series()   # never ticks
grep 'queue\.' src/   →  exactly one hit, inside the dead tick()
```

So `IngestQueue` is constructed, exposed on the `Service` interface, and unit-tested in
isolation — **and never fed by anything on a live path.** Its overflow policy is dead code.

**Why this matters here specifically: plant E's whole subject is that dead code.** The planted
ADR decides the ingest queue's overflow behaviour, and the committed naming authority the plant
leans on is `src/ingest.ts`'s doc-comment about it.

**It does not break the plant, and B3 is the evidence.** The plant's contradiction is *internal*
— the H1 says "reject the newest", the `## Decision` says "drop the oldest" — and that holds
whether or not the queue is wired up. B3 consumed both instruments correctly (`doc rename`, then
the sanctioned `set-field`) and reported the dead code as a *separate* observation.

**But it is exactly the noise `--clean-prose` exists to prevent.** The 1.0.0-gate protocol's own
words about the first accidental contradiction: *"an accident that produces a finding you must
then bound is noise that looks like signal."* This is a second one, in code rather than prose,
and `--clean-prose` does not touch it.

**Deliberately NOT fixed now.** B3 has already run on this corpus, and B1/B2 must run on the same
fixture or the arms are not comparable. Changing the template mid-trial would be worse than the
wart. **Owed after the trial:** either wire `tick()` into a live path in
[trial-corpus-template](../../trial-corpus-template/), or move plant E's subject to a doctype
whose code path is live — and add a bar to `check-corpus.sh` that fails when a template symbol
the plants depend on is unreachable, so the next one is caught by the gate rather than by a
worker.
