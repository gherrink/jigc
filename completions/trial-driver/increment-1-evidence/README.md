# Increment 1 evidence

| file | what it is |
|---|---|
| `seed-turns.txt` | the two scripted seed turns, verbatim |
| `seed-manifest.json` | the frozen fixture's manifest — id, sha, marker |
| `forkA-result.json` · `forkB-result.json` | each fork's final `result` event: its minted id and its answer |
| `cold-resume-stderr.txt` · `cold-resume-result.json` | the cold-resume repro (below) |

**The cold-resume repro is deliberate, not salvaged.** The behaviour was first seen
by accident on a failed seed attempt, and that attempt's stderr was **lost** when the
work directory was rebuilt by the re-seed. Rather than reconstruct it from recall, it
was re-derived as a probe that anyone can re-run:

```sh
echo "Say the single word OK and nothing else." > /tmp/coldprobe.txt
./completions/trial-harness/run-session.sh --headless --prompt-file /tmp/coldprobe.txt \
  --arg --resume --arg 00000000-dead-4000-8000-000000000000 \
  <corpus> /tmp/coldprobe-out jigc-gate:rc11
```

Result on `jigc-gate:rc11` (CLI 2.1.233), 2026-08-25 — **exit 1**:

```
No conversation found with session ID: 00000000-dead-4000-8000-000000000000
```

`subtype: error_during_execution`, `is_error: true`, `num_turns: 0`.
