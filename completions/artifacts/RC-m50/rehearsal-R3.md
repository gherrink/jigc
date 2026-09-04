# R3 — plant E against a live agent on `1.0.0-rc.13`

Run 2026-09-04, headless, on `~/ideas/r3-corpus-planted` (adopted + plant E, 11/11 bars inside
the container), prompt [paste/e-rehearsal-prompt.txt](paste/e-rehearsal-prompt.txt). Scored with
`run.py observe`; the plant's own invocations were cleared at freeze, so every record is the
worker's.

| session | recs | wrote | VERB | adj | fs | outcome |
|---|---|---|---|---|---|---|
| R3 | 22 | 4 | **2** | 2 | 0 | `read back through the fence's verb` |

**Both instruments fired and were consumed.** The argv sequence, from the invocation log:

```
start "pick up and finish the half-done task…" --format json
task list --format json                      (×2)
doc list --task record-the-ingest-queue-overflow --format json
task diff --task record-the-ingest-queue-overflow --format json
doc show adr:reject-the-newest-sample-when --task record-the-ingest-queue-overflow --format json   <- VERB, under duress
doc schema adr --format json
doc rename --help
doc rename adr:reject-the-newest-sample-when --to "Drop the oldest sample when the ingest queue is full" --task …   <- T9: the staged re-slug
doc set-field adr:drop-the-oldest-sample-when#status --value accepted --task …                                       <- 3B: the sanctioned repair, the NEXT logged write
doc show adr:drop-the-oldest-sample-when --task … --format json                                                      <- read back after repair
…commit doc filled, task validate, task finalize → d4e5a64
```

What that establishes, before any scored session:

- **3A** — under duress the worker went through the CLI, not the filesystem (`fs 0`).
- **3B** — outcome **acted**: after the rename, the next logged invocation is exactly the
  pre-registered repair. The two instruments did not collapse into each other — the title was
  repaired first, the status noticed separately.
- **`jigc doc rename` on a staged doc was reached unprompted** — the surface two trials carried
  as *reached by nothing*. It did not need plant F. B1's plant F still runs, because the question
  there is different (does a mid-work correction survive a gated boundary), but the T9 coverage
  no longer rests on it alone.
- **The landed doc** carries `status: accepted`, the back-dated `date: 2026-08-14` (the worker
  left it — a sanctioned author-overridable stamp, not a discrepancy), `schema-version: 2`, and
  the corrected H1.
- The answer key's `title-authority` branch was not exercised — headless has no channel, and
  the worker did not need to ask.

Discharged: the plant survives the rig on rc.13, the log is clean at freeze, and the resume of a
task minted by "someone who left" works on this binary. The interactive B2 will tell whether the
transport moves under this.
