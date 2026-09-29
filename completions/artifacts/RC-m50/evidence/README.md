# Evidence — the scored channels of every session, archived

Each directory holds the two channels `run.py observe` scores — `invocations.jsonl` (the
product's own log, exact) and `transcript.jsonl` (the CLI transcript, the FILESYSTEM heuristic's
source) — plus `PROVENANCE.txt` (image, sha, `session-start`) and `result.txt` (the worker's
closing text). Archived because the machine loss of 2026-08-02 took a trial's analysis with it:
what exists on one machine only does not exist.

> **Raw transcripts removed before publication (2026-09-28).** Every `transcript.jsonl` and `result.txt` below is gone from this repository, and so is everything under `subagents/` except `B2/subagents/*.jsonl`, which `completions/trial-driver/test_observe.py` reads as a named fixture ([implementation/public-hygiene.md](../../../../implementation/public-hygiene.md) → rule 3). `invocations.jsonl` and `PROVENANCE.txt` remain, so VERB and VERB-ADJACENT still re-score from here; the FILESYSTEM channel needs the transcripts, which are in the private archive of the pre-publication history.

| dir | arm | scored |
|---|---|---|
| `R3/` | plant E rehearsal, headless | rehearsal |
| `R4/` | plant F's stop rehearsal, headless | rehearsal |
| `B3/` | corpus accretes, headless, plant E + foreign ADR | **yes** |
| `B3-h2/` | second headless reading | **yes** |
| `B3-strict/` | `--strict-permissions` | unscored, labelled |
| `B4-h/` | plan the first milestone, first run — halted at the Settle gate | void (no authoring occasion) |
| `B4-s-turn01/` | the seeded re-run's first turn — the whole arc | **yes** (read-back series) |
| `B4-s-turn02/` | its second turn — the `scope` reply; nothing left to do | void |
| `walk-record-*.md` | the final walk pass, the migration pair's two halves | — |

B1 and B2 (interactive) are added when they run. Re-score any directory with
`python3 completions/trial-driver/run.py observe <dir>` — `observe` reads exactly these files.
