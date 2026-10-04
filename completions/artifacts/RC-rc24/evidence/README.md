# Evidence — what is committed for each arm, and what is not

One directory per arm, plus the walk. This is the **exact** half of the evidence: the product's own
record of each session, and the one committed trace of the trailer check. Everything the record
argues from that is *not* here stayed on the machine that ran the trial, and
[../README.md](../README.md) quotes from it in short excerpts.

| directory | arm | out-dir it was copied from |
|---|---|---|
| `C/` | (c) the disagreement — one turn | `~/out/RC24-C` |
| `B/` | (b) the correction — two seeded turns | `~/out/RC24-B-frozen-work/turn01`, `turn02` |
| `A/` | (a) the fan-out — two seeded turns | `~/out/RC24-A-frozen-work/turn01`, `turn02` |
| `walk/` | walk arm 00, the positive control | `~/out/RC24-walk` |

## What is committed

- **`invocations.jsonl`** — the product surface: `.jigc/logs/invocations.jsonl`, byte for byte.
  Which door ran, when, and how it exited. VERB and VERB-ADJACENT come from here, and so does
  every ordinal the record cites. **For a seeded arm it is turn 2's log, which is cumulative**: it
  holds turn 1's records too, and its first two records are adoption's, older than any session.
  Each was read through once, record by record, before it was committed: no name, no address and
  no host path is in any of the three (89 records). The intents a worker typed are in `argv`; they
  quote the prompt's own words and the `TKT-<n>` tokens, which are this repository's
  anonymization vocabulary.
- **`PROVENANCE.txt`** (`PROVENANCE-turn01.txt`, `PROVENANCE-turn02.txt` for a seeded arm) —
  image, image id, `jigc --version`, `jigc-sha`, model, permission mode, corpus source, session
  start, exit code. `jigc-sha` is `unknown` on every one: a registry image carries no tree of ours,
  and the image id is what identifies the binary. `corpus-src` is written with `~` for the home
  directory.
- **`trailer-rows.txt`** — the output of [../tools/trailer-rows.py](../tools/trailer-rows.py) over
  the arm's out-dir: every committing-door record at exit 0 joined to the commit it made, with the
  `Co-Authored-By` lines as git parses them. `.git` is not committed, so this is the only committed
  trace of the trailer check. Both scorers re-derived every row by hand, and the helper's two
  known limits are recorded (../README.md → Tooling findings, T-9 and T-10).
- **`walk/walk-record-rc24.md`** — the walk, one block per arm **including the 23 that did not
  run**. Its host paths are rewritten to `~`; nothing else in it was changed.

## What is not committed, and why

Never, by [implementation/public-hygiene.md](../../../../implementation/public-hygiene.md) → rule 3
(a raw transcript is committed only when a test or tool reads it as a named fixture, and none
here is):

- **no transcript** — neither a main session's nor a sub-agent's;
- **no `stream.jsonl`** and no `stderr.txt`;
- **no corpus** — not the tree, not its `.git`;
- **no debrief's raw text**;
- **no rubric file** — the operator's saved `observe`, arm, trailer, raw-git and git-log output.
  Twenty-five files under the operator's rubric and log directories carry host paths with a login
  name (../README.md → Tooling findings, T-11); the record quotes what it needs from them,
  rewritten;
- **neither scorer's file**, and none of the operator's logs.

So three things the record states **cannot be re-derived from this directory alone**: the
FILESYSTEM channel and every *first read of a site* (they need the transcripts); the reflog, the
trees and the trailers themselves (they need `.git`); and each turn's final message and denial
count (they need `stream.jsonl`). Those rest on the two independent scores, which agree.

## Re-scoring what can be re-scored

```sh
python3 completions/trial-driver/run.py observe --archive          # the reader's own control, first
python3 completions/trial-driver/run.py observe --gate completions/artifacts/RC-rc24/gate-rc24.json \
    ~/out/RC24-C
```

**These directories are flattened, as RC-rc14's are:** an out-dir's `.jigc/logs/` and
`.session-transcript/` trees are not preserved, so `run.py observe <this directory>` does not find
either channel through its finder. The second command scores the live out-dir on the machine that
holds it. The ordinals and the committing-door records can be read straight off each
`invocations.jsonl` here: one JSON record per line, the line number is the ordinal, and the `inv`
helper in [../runbook.md](../runbook.md) §4 is that reader with an out-dir's path in front.
