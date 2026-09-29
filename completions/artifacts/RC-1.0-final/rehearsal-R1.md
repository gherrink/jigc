# R1 — plant E against a live agent

Run 2026-08-28 on `jigc-gate:rc12` (`JIGC_SHA 5ff85eaa…`), model `claude-sonnet-5`, headless,
`bypassPermissions`. Owed by [protocol.md](protocol.md) §2.3, which owes it to
[cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md) rule 4:

> **Rehearse on the axis you are uncertain about.** `cue-cards.md` named its own untested
> assumption in writing and carried it into the trial anyway. **The untested axis is the one
> that failed.**

Plant E had never met a live agent. This is that payment. **It found two instrument defects,
neither of them findable by reading, and both would have corrupted the trial's headline.**

## The verdict

**Plant E works.** Two runs, two workers, both instruments fired and were consumed in both.

| | R1 | R1b |
|---|---|---|
| corpus | `r1-corpus` | `r1b-corpus` (re-planted after the log fix) |
| the title contradiction | `doc rename` → `adr:drop-the-oldest-sample-when` | `doc rename` → `adr:drop-the-oldest-sample` |
| the `status` discrepancy | `set-field …#status --value accepted` | `set-field …#status/status --value accepted` |
| outcome under §3.3 | **acted** | **acted** |
| landed | one clean finalize | one clean finalize |

Both workers **found the contradiction unprompted** and both named it in their own words. R1's
report: *"an open `record-decision` task had a drafted ADR whose slug/title contradicted its own
body … The status field was also set to `superseded` with nothing for it to supersede."* R1
additionally spawned a subagent to check `src/ingest.ts` and confirmed the committed source
agrees with the ADR's *content*, not its title — the naming authority doing its job.

**All four of §5E's falsifiers held:**

| falsifier | outcome |
|---|---|
| the worker discards the task | did not happen in either run — the expensive prose worked |
| the worker keeps the wrong title | did not happen; both renamed |
| the task will not resume | did not happen; the planted task was the first thing both reached |
| the plant does not survive the rig | closed separately — asserted **inside** a fresh container before any prompt |

## Defect 1 — the plant was writing into the channel it is scored on

`.jigc/logs/invocations.jsonl` lives **inside the corpus**. Plant E drives the real binary, and
its end-state bar reads the doc back five times, so all of it landed in the measurement channel,
stamped before the session began.

Measured on R1's raw evidence:

```
observe reported : records 46   authoring 13   VERB 6   adjacent 4
the worker did   : records 26   authoring  6   VERB 2   adjacent 2
the plant added  :                             VERB 4   adjacent 2
```

**A 3× inflation of the headline, pointing the flattering way** — and both plant-E arms carry
this trial's headline. Two fixes, in [protocol.md](protocol.md) §3.4 settlement 3: the reader
splits on a new `session-start` stamp in `PROVENANCE.txt`, and the plant clears the log as its
last act (a worker that read the log would otherwise **watch itself being planted**).

Pinned by `test_observe.py::ThePlantIsNotTheWorker`, whose first case preserves the defect.

## Defect 2 — a document read and a bookkeeping read were one number

R1 and R1b are the same prompt against the same plant, and they diverge on the axis 3A measures:

| run | managed-document reads | workbench reads | the invariant |
|---|---|---|---|
| R1  | **2** — the staged `.md` itself | 5 | **broke** for documents |
| R1b | **0** — document taken via `doc show --task` | 6 | **held** |

Both are `filesystem = 6` under one count. Now reported split, per §3.4 settlement 4. Pinned by
`test_observe.py::ADocumentReadIsNotABookkeepingRead`.

**And the workbench reads are themselves a finding.** Both workers reached for `roles.json`,
`base.json`, `intent`, `workflow`, `provenance.json` and `staged-snapshot.json` — task state
**no read verb exposes**. That is §1's capability-gap row, not a channel violation, and it is
the sixth consecutive trial-shaped observation of the discoverability lens: *the capability
exists or it does not, and either way no composed surface names it.*

## What this does NOT establish

- **n = 2, one transport, one model, one prompt, one day.** Both workers were thorough. The
  trial must not read "plant E produces a repair" as a rate; it establishes that the plant
  **produces the state, survives the rig, and that both its instruments are reachable and
  independent** — a rename preserves the planted `status`, verified live, so a worker can fix
  the title and still miss the discrepancy.
- **Neither run is a scored trial result.** Both are rehearsals on a rehearsal corpus.
- **Headless only.** B2 runs interactive, and this says nothing about that transport beyond
  what [increment-0.md](../../trial-driver/increment-0.md) already licenses.
- **`bypassPermissions` was on**, which makes filesystem reads cheaper than an adopter finds
  them — protocol §9's declared directional confound, and it bears directly on defect 2's
  numbers.
