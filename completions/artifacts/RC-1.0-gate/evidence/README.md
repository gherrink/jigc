# Evidence — the 1.0.0-gate trial's primary channels, archived

**Why this exists.** Every count in [trial-record.md](../trial-record.md),
[session-findings.md](../session-findings.md) and [coverage.md](../coverage.md) derives from these
files. Until 2026-08-18 they existed **only in `~/ideas/*-out/` on one machine** — and this project
has already lost a machine mid-run ([DECISIONS.md](../../../../DECISIONS.md) → 2026-08-02, the
recovery entries). A record whose evidence lives on one disk is a record that can become an
unverifiable claim overnight.

| file | what it is |
|---|---|
| `<corpus>-invocations.jsonl` | the invocation log — §3.3's **primary channel**. Every `doc show … --task` count, every exit code, every finding code is read from here |
| `<corpus>-transcript.jsonl` | the session transcript — §3.3's **second channel**, and the only evidence for a FILESYSTEM outcome (a direct read of a managed doc) |
| `b1-carryover-909a24c.txt` | the commit that swept two planted paths into an ADR commit, captured **before** the worker rewrote it |
| `b1-invocations-at-carryover.jsonl` | the log as it stood at that moment |

**The corpora themselves are not archived** — ~7 MB of working trees whose end states are already
described in the record, and whose git history is reconstructible from the template plus the logs.
What is irreplaceable is the *conduct*, and that is entirely in these two channels.

## The four blind sessions, at a glance

| corpus | session | records | `doc show … --task` |
|---|---|---|---|
| `harborlight` | B1 · cold start, both plants | 26 | 3 |
| `pinegrove` | B2 · design altitude | 102 | 4 |
| `stonefly` | B3a · corpus accretes (plant failed to fire) | 94 | 6 |
| `rosewater` | B3b · the re-run, plant landed | 99 | 7 |
| `rc11-control` | arm 0 · the positive control | 10 | 1 |

**Read the logs, not the record, when the two disagree.** That rule is not decorative: the record
carried B1 at 5 read-backs for a day, and the log said 3 the whole time
([session-findings.md](../session-findings.md) → Corrections).
