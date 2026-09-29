# Evidence — the two scored channels for every arm

Archived because it previously existed on one machine only. One directory per arm:

> **Raw transcripts removed before publication (2026-09-28).** Every `transcript.jsonl`, `result.txt` and `subagents/*.jsonl` below is gone from this repository except `B1/transcript.jsonl`, which `completions/trial-driver/test_observe.py` reads as a named fixture ([implementation/public-hygiene.md](../../../../implementation/public-hygiene.md) → rule 3). `invocations.jsonl` and `PROVENANCE.txt` remain in every directory, so VERB and VERB-ADJACENT still re-score from here; the FILESYSTEM channel of the other arms needs the transcripts, which are in the private archive of the pre-publication history.

- `invocations.jsonl` — the product surface. VERB and VERB-ADJACENT come from here, and it has a
  pinned record shape.
- `transcript.jsonl` — the CLI transcript. The FILESYSTEM channel comes from here and is a
  **heuristic**, returned as evidence for review rather than as a number.
- `subagents/*.jsonl` — **new in this trial's reader.** Present for `B2`, `B4-h` and `R3`, which
  delegated. Four transcripts across three sessions; **all walked, zero managed reads.** Before
  the I-1 fix the reader opened only the largest `.jsonl` per session, so these four would have
  gone unread and every one of those cells would have scored clean **without having been looked
  at**. They score clean *because they were*.
- `PROVENANCE.txt` — image, sha, model, permission mode, corpus, session start, exit code.
  `run.py observe --gate` now compares its `jigc-sha` against the round's gate record and refuses
  evidence from another binary rather than scoring it silently (I-2).
- `result.txt` — the headless `stream.jsonl`, where one exists. **A halt exits 0 at
  `subtype: success`**, so this is the only place a halt is visible.

`walk-record-rc14.md` is the walk, one block per arm **including the ones that did not run**.
`walk-arm17-empty-id-axis.txt` and `walk-arm22-orientation.txt` are the two arms the record
argues from directly.

## Re-scoring

```sh
python3 completions/trial-driver/run.py observe --archive          # the reader's control, first
python3 completions/trial-driver/run.py observe --gate completions/artifacts/RC-rc14/gate-rc14.json ~/out/RC14-B2 …
```

**Note the shape difference from a live out-dir:** these directories are flattened — the
`.session-transcript/projects/-work/` tree is not preserved — so `run.py observe <this dir>` does
**not** find either channel through `_find`. That wart is recorded in
[pre-trial-findings.md](../pre-trial-findings.md) PT-3 rather than left as an implied capability.
