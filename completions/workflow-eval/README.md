# Workflow-eval — the behavioral test layer for jigc workflows

**The standing answer to a question the rest of the test suite never asks.** `cargo
test` proves the CLI *composes* a workflow correctly — deterministic golden output.
It says nothing about whether a real coding agent, handed a real task, actually
**selects**, **engages**, and **completes** the right workflow. The
[differentiator pilot](../artifacts/differentiator-pilot-study1/) was the first
behavioral test of the workflow layer, and it failed all three:

- **mis-selection** — when nudged to engage, the weaker model picked `quick-fix` 6/6
  (the workflow that, pre-Phase-2, omitted the doc↔code check a documented rename
  needs);
- **0/16 engagement** — on a bare task the agent ran *no* `jigc` command, editing
  files directly;
- **fumbled completion** — runs that engaged still bypassed `finalize`.

"The CLI composes it correctly" ≠ "an agent uses it correctly." This harness
generalizes the pilot's one-off container rig into a **re-runnable, per-workflow**
eval that reports those exact rates, so the gap can be tracked as the workflow layer
(descriptions, the Phase-2 floor, future routing) changes — and it is the measurement
substrate for the long-horizon many-edit study.

## What it measures

| metric | question | source |
|---|---|---|
| **engage-rate** | did the agent invoke `jigc` at all? | tool-call stream |
| **select-rate** | which workflow did `jigc start --workflow X` pick (vs expected)? | tool-call stream |
| **complete-rate** | did it reach `jigc task finalize`? | tool-call stream |
| **outcome** | clean / drift / blocked? | `jigc validate` capture (the Phase-2 floor — the precise oracle) or an old-symbol grep |

The behavioral three need the **tool-call stream** — so the runner captures
`claude -p --output-format stream-json --verbose`, not the pilot's `--output-format
json` (which kept only the final result, which is why the pilot's select/engage
numbers came from an ad-hoc probe rather than the matrix).

## The pieces

| file | role |
|---|---|
| `run-eval.sh` | one isolated container run (generalized from the pilot's `run-rep.sh`): `run-eval.sh <image> <model> <prompt-file> <out-dir>` → captures `transcript.jsonl` + outcome artifacts |
| `analyze.py` | the core: one transcript → the behavioral record (engage / select / complete / outcome). Pure stdlib; tolerates stream-json, a JSON array, or a result-only object |
| `eval.py` | aggregate a dir of runs → the four rates + a markdown report |
| `selftest.sh` | verify `analyze.py` + `eval.py` over committed fixtures — **zero API spend**, CI-safe |
| `fixtures/` | synthetic stream-json transcripts (engaged-and-finalized · bypassed-to-quick-fix · no-engagement) that pin the analysis |

## Run it

**Verify the analysis layer (no Docker, no API):**

```sh
./selftest.sh
```

**A live eval of one workflow** (needs Docker + a twin image + API spend):

1. Build a twin image with `jigc` installed and a managed doc whose `code-anchor`
   the task will touch — follow the pilot
   [RUNBOOK](../artifacts/differentiator-pilot-study1/RUNBOOK.md) §1–3 (the container
   gotchas — `USER node`, `--permission-mode bypassPermissions`, prompt via
   `-e PILOT_PROMPT` — are already baked into `run-eval.sh`).
2. Write the task prompt (a neutral ticket, no doc/workflow hints — let the *agent's*
   behavior determine the metrics) to `prompt.txt`.
3. Run N reps and aggregate:

```sh
for r in $(seq 1 8); do
  ./run-eval.sh my-jigc-twin claude-sonnet-4-6 prompt.txt runs/rep$r
done
python3 eval.py runs --expected-workflow single-task --md runs/report.md
```

`eval.py` prints the JSON summary and (with `--md`) the per-run table. For a non-jigc
arm (no `jigc validate`), pass `--old-symbol <PreRenameName> --doc <rel/path/to/doc>`
to score the outcome by grep instead.

## Honest boundaries

- **The analysis is validated; a live run is not exercised here.** `analyze.py`'s
  stream-json parsing is pinned against **synthetic** fixtures matching the documented
  event schema (`type: assistant` with `message.content[]` `tool_use` blocks; a final
  `type: result` carrying `num_turns` / `total_cost_usd` — the latter verified against
  the pilot's real result-only transcripts). It is defensive (scans every `tool_use`
  block; tolerates content under `message` or top-level; word-bounds `jigc`), but the
  **first live run should spot-check one `transcript.jsonl`** against the parsed record
  before trusting a matrix — the one thing fixtures can't certify is the exact shape a
  given `claude` CLI version emits.
- **It measures behavior, not correctness of prose.** Same blind spot the pilot named:
  a run can be `clean` by the anchor oracle while a doc's *prose* drifted from the code
  it still correctly cites.
- **Cost is real.** Each rep is a full agent session. Scale reps to the question;
  `selftest.sh` is the free way to keep the harness honest between live runs.

See [`DECISIONS.md`](../../DECISIONS.md) (2026-06-22 → Phase 3) and the pilot's
[`HANDOVER-next-session.md`](../artifacts/differentiator-pilot-study1/HANDOVER-next-session.md).
