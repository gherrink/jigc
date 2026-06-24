# Per-sequence summaries (the reproducibility trail)

One `sequence.json` per study sequence (the per-edit timeline: dangling-ref count,
cost, commits, blocked/recovery, control-edge + oracle-agreement flags). The aggregate
across these is the committed [`../matrix.json`](../matrix.json) / [`../matrix-table.md`](../matrix-table.md);
these are the per-rep detail one level below it. The raw agent transcripts (≈5 GB) were
throwaway and are not retained — the objective oracle data here + the aggregate are the
durable record (mirrors the doc-code study's `runs/sequences/` precedent).

**Filename tags:** `<arm>-<model>-rep<n>.json`, where the model id suffix is
`-6-` = `claude-sonnet-4-6` (Sonnet, R=3, all arms) and `-8-` = `claude-opus-4-8`
(Opus, R=2, arms A + C550 only). `cert-A-sonnet.json` is the live certification run;
`A-opus-rep1.401-discarded.*` is the OAuth-401-truncated sequence discarded + re-run per
protocol (kept for the honesty trail; the clean re-run is `A-8-rep1.json`).
