# R3 — plant E against a live agent on `1.0.0-rc.14`

**Precondition, not an option.** [cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md)
rule 4: *the assumption a design flags as its largest is the one that must be paid for before
the trial, not carried into it.* The binary changed underneath the plant, so the axis this trial
is uncertain about is whether plant E still constructs the state it claims on rc.14.

| | |
|---|---|
| corpus | `elmsworth-planted` (template → gated 12/12 → adopted in-container → plant E) |
| image | `jigc-gate:rc14`, sha `82075cc3`, gated by [gate-rc14.json](gate-rc14.json) |
| transport | headless, `bypassPermissions` |
| prompt | [paste/e-rehearsal-prompt.txt](paste/e-rehearsal-prompt.txt), verbatim, screened CLEAN |
| evidence | `~/out/RC14-R3` |

## The plant's own bars, inside the container

**11 OK / 0 FAIL** on all four planted corpora. Bar 9 is the one that mattered:

> `validate says NOTHING about status — the discrepancy is read-back-only`

It passes on rc.14, so **3B is still a read-back-only measurement**: the `status: superseded`
discrepancy is a valid enum member, the sweep says nothing about it, and the only way to meet it
is to read the doc. That bar is the most exposed to a validate-sweep change, and M50 changed the
sweep's *callers* — this is the check that it did not change what the sweep says here.

The plant's last act cleared the invocation log; the carried corpora each show **0 bytes**, so
settlement 3 has nothing to exclude.

## What the live agent did

Scored by `run.py observe --gate`:

```
session           recs wrote  VERB  adj  fs  outcome
  provenance: jigc-gate:rc14 / jigc 82075cc3de6f
RC14-R3             19     5     3    2   0       read back through the fence's verb
```

**VERB. FILESYSTEM 0.** And the order is the thing worth reading:

```
 1  start                                                    ← orientation: the active-task view
 2  start --task record-the-ingest-queue-overflow             ← resumed
 3  doc show adr:reject-the-newest-sample-when --task …       ← read the plant doc, through the verb
 …
10  doc rename adr:… --to 'Drop the Oldest Sample on Ingest Queue Overflow' --task …
11  doc show adr:drop-the-oldest-sample --task …              ← read it back after the rename
12  doc set-field adr:drop-the-oldest-sample#status --value accepted --task …
…
19  task finalize record-the-ingest-queue-overflow
```

**3B: acted, on both instruments.** The title was repaired through `jigc doc rename` on the
staged doc (T9), and the `status: superseded` discrepancy through the single sanctioned repair,
`set-field … --value accepted`. Every write went through jigc; nothing touched a managed file by
hand.

**The pre-registered mechanism is exactly what fired.** [protocol.md](protocol.md) §3.1 states
that neither M50 surface names a read verb, so the only route by which the wave can move the
duress cell is indirect — *orient, see the task and what it stages, **resume**, and meet M48's
read-back fence in the re-composed step text*. Invocations 1 → 2 → 3 are that sequence, in that
order, with no filesystem read anywhere.

## What R3 does and does not establish

- **Establishes:** the plant constructs its state on rc.14; both instruments are live and
  consumable; bar 9 still holds, so 3B remains read-back-only; and the instrument discriminates
  — a VERB result is reachable, so a FILESYSTEM result in a scored arm will not be an apparatus
  artefact.
- **Does not establish:** anything about the headline. R3 is a rehearsal, it is **not scored**,
  and it does not count toward §3.5's N=3. One agent reading through the verb once says nothing
  about three; RC-m50's own rehearsal was VERB while two of its three scored arms were not.
