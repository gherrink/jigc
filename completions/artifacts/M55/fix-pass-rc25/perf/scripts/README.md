# The run-performance report's instrument — kept so the measurement can be repeated

These are the scripts that produced [../run-performance.md](../run-performance.md): where the wall clock and
the tokens of a multi-agent session went. They sit beside the report so that a later session can **measure
again** after it changes the gate or the orchestration, instead of rebuilding the instrument — standard-library
Python 3, bash, and one small C file. They were ported from the session's scratch directory with their
analysis unchanged. Against the session they were written for, every ported Python script printed what its
original printed, byte for byte, and the table scripts regenerated the report's fifteen tables; the parser's
two datasets were equal record for record up to the cutoff. Of the shell scripts, `probe_all.sh` and
`spawn_probe.sh` were re-run on one sampled test and returned the counts the report has; `measure_suite.sh`,
`spawn_unit_cost.sh` and `verb_cost.sh` were syntax-checked and not re-run.

What the port changed is only **where the inputs come from**. Every host path, the session id and the nine
workflow run ids that the originals carried as constants are now read from the environment or from an
argument, through [`perf_env.py`](perf_env.py); a script started without an input it needs says which one
and exits 2, and `--help` prints its opening comment. Two files here are not in the report's §7 list:
`perf_env.py`, which the port added, and [`probe_sample.txt`](probe_sample.txt), which `probe_all.sh` read
from the start.

## What it needs

- **A Claude Code session's transcript directory** — the directory that holds `subagents/` (one transcript
  per single subagent, and under `subagents/workflows/<run id>/` each workflow run's `journal.jsonl` and
  per-agent transcripts) and `workflows/` (one run file per workflow run), with the session's own transcript
  beside it as `<that directory>.jsonl`.
- **The `dev/gate` logs of that session, still in the system temp directory** — `jigc-gate-XXXXXX` and its
  `-steps-`, `-hygiene-` and `-gitleaks-` siblings. `gate_logs.py` looks where `dev/gate` writes: `$TMPDIR`,
  `/tmp` when it is unset. A gate run's start is its log's file **birth time**, so the logs are read in
  place; a copy does not carry it, and a reboot that clears the temp directory ends the measurement.
- **The repository**, read-only, for `git log` and `git show`.
- **Optionally, a private clone** for the per-test timing run. `measure_suite.sh` makes it itself, inside a
  scratch work directory, and builds into the clone's own `target/` — several gigabytes, and never the
  working repository's. It runs the gate's own test command, so it needs `cargo nextest`.

| variable | what it names | read by |
|---|---|---|
| `SESSION_DIR` | the session's transcript directory | `parse_transcripts.py`, `analyze_tokens.py`, `explore1.py` … `explore4.py` |
| `OUT_DIR` | where the derived datasets are written and read back — **outside the repository** | every script that reads or writes a dataset |
| `RUNS_FILE` | a JSON object naming the session's workflow runs, in run order: `{"wf_<run id>": "<short name>", …}` | every script that prints a per-run column or label |
| `CUTOFF_UTC` | the instant the measured run ended, ISO 8601; everything later is left out | every script that filters by time |
| `WORK_DIR` | the scratch directory of the fresh suite run (`clone/`, `suite/`, `shim/`) | `make_tables2.py probe`; the shell scripts take it as their argument |

`RUNS_FILE` must name **every** run that has a directory under `subagents/workflows/`; a run it omits ends a
script with a `KeyError` on that run's id. The single subagents and the orchestrating session need no entry —
they are `singles` and `orchestrator`.

## The order to run it in

`S` is this directory. Each step names what it writes into `OUT_DIR`; the report's §2 says how each duration
is derived.

**1. The datasets.**

| # | command | writes |
|---|---|---|
| 1 | `python3 $S/parse_transcripts.py` | `agents.json`, `calls.jsonl` |
| 2 | `python3 $S/gate_logs.py` | `gate_logs.json` |
| 3 | for example `git -C <repo> log --abbrev=8 --format='%h %cI %s' origin/main..<rev> > "$OUT_DIR/commits.txt"` | `commits.txt` — one line per commit of the measured branch: hash, committer date in strict ISO 8601, subject |
| 4 | `python3 $S/analyze_gates.py` | `gate_ledger.json` (needs 1 and 2) |
| 5 | `python3 $S/analyze_partition.py` | `partition_by_agent.json` (needs 4) |
| 6 | `python3 $S/analyze_tokens.py` | `tokens_by_agent.json` (needs 1) |

**2. The fresh suite run** — what the per-test tables of §3.7 and the fast-tier arithmetic rest on, because
the gate logs carry no per-test timings.

| # | command | writes |
|---|---|---|
| 7 | `bash $S/measure_suite.sh <repo> "$WORK_DIR"` | `$WORK_DIR/clone`, and the logs under `$WORK_DIR/suite` |
| 8 | `python3 $S/new_tests.py <repo> <rev>` | `new_tests.json` |
| 9 | `python3 $S/analyze_suite.py "$WORK_DIR/suite/nextest-full.log"` | `per_test_suite.json` — the dataset is named after the log's directory, and the later scripts read that name |
| 10 | `cc -O2 -o "$WORK_DIR/shim/shim" $S/shim/shim.c && cp "$WORK_DIR/shim/shim" "$WORK_DIR/shim/git"`, then `bash $S/probe_all.sh "$WORK_DIR"` | `$WORK_DIR/suite/probe-summary.txt` and one `.spawns` file per sampled test; `python3 $S/analyze_spawns.py "$WORK_DIR"/suite/*.spawns` sums them |
| 11 | `cargo build --release` in the clone, then `bash $S/spawn_unit_cost.sh "$WORK_DIR"` and `bash $S/verb_cost.sh "$WORK_DIR"` | nothing — they print |

`probe_all.sh` reads [`probe_sample.txt`](probe_sample.txt), the ten tests the report sampled; a second
argument names another sample. The compiled shim belongs in the work directory, never in this one.

**3. The analyses.** Each prints to the terminal and none reads another's output: `analyze_overview.py`,
`analyze_timeline.py`, `analyze_classes.py`, `analyze_reds.py`, `analyze_red_causes.py`,
`analyze_gate_steps.py`, `analyze_gate_wait.py`, `analyze_commits.py`, `analyze_builds.py`,
`analyze_waste.py`, `analyze_orientation.py`, `analyze_instruments.py <run id>…`,
`trial_steps.py <directory of the trial's .start/.end/.rc markers>`, and — once step 9 has run —
`analyze_fast_tier.py`. The report's §7 says which section each one feeds.

**4. The report's tables, and its assembly.** Redirect each into a file of your own, under `OUT_DIR`.

| table of the report | command |
|---|---|
| §3.1 runs | `make_tables.py runs` |
| §3.2 exclusive partition | `make_tables2.py partition` |
| §3.3 command classes | `make_tables3.py` |
| §3.4 agents · single subagents | `make_tables.py agents` · `make_tables.py singles` |
| §3.5 longest commands · longest outside the gate | `top_commands.py 30` — two tables, separated by a blank line |
| §3.6 gate growth · red gates | `make_tables2.py gate-growth` · `make_tables2.py reds` |
| §3.7 binaries · suites · tests · spawn sample | `make_tables2.py suite-binaries` · `suite-top` · `tests-top` · `probe` |
| §3.10 tokens | `make_tables2.py tokens` |
| §3.11 instruments | `make_tables.py instruments` |

`build_report.py <source>` copies a source file to stdout, replacing each line `<!--include <path>-->` by
that file, the path taken relative to the source; it refuses to emit a line that carries a host path. The
source it was run on is not kept here: it was the report's prose with fifteen such lines where the tables
stand, and the committed report is that output plus the paragraph that opens it.

`explore1.py` … `explore4.py` come before all of this, and only if the parser stops matching: they are the
shape explorations `parse_transcripts.py` was written from.

## What a script prints

The analyses and the table scripts print aggregates — counts, seconds, shares — with agent labels and test
names. Three kinds of row carry more, and none of them is a raw transcript line:

- the listings of long commands (`analyze_builds.py`, `analyze_classes.py -top`, `analyze_waste.py`) carry
  the **sanitised head of a command** as `calls.jsonl` stores it, and `top_commands.py` the first 80
  characters of the description a command was launched with — the text of the report's §3.5;
- `analyze_gates.py` and `gate_logs.py` carry the head of a gate log's first compiler error line, and
  `analyze_gates.py` the description a red gate's launching command carried;
- **`explore1.py` … `explore4.py` print short excerpts of transcript text**, and `explore3.py` up to 700
  characters of it, raw but for the home path. Their opening comments say exactly what; run them only where
  the output stays private, and commit none of it.

## What is not in the repository, and why

- **`data/`** — `agents.json`, `calls.jsonl`, the gate ledger, the per-agent and per-test datasets and the
  generated tables. `calls.jsonl` is derived from private transcripts: one record per tool call, each with a
  600-character command head. The sanitiser masks the home path, secret-shaped assignments and token-shaped
  strings; that is a filter, not a proof, so the dataset stays out. The tables it led to are in the report.
- **The transcripts and the gate logs** themselves.
- **The private clone, the suite run's logs and the compiled shim** — rebuilt by steps 7 and 10.
- **The report's source file** — see step 4.

Keep `OUT_DIR` and `WORK_DIR` outside the working tree, so that none of this can be staged by accident.

## What is still this run's

The port moved the places and the ids out; the analysis's vocabulary stayed, because changing it is changing
the analysis. To measure a different run, read these first:

- **The run short names.** The fixed columns and filters expect `review`, `trial-score`, `tier1-redrive`,
  `fix-plan`, `round1` … `round4` and `audit` — the report's §3.1 lists the nine runs they name, with their
  ids. A run given another name appears in the per-run columns, is skipped by every filter that asks for a
  fix round or the audit, and stops `make_tables.py runs`, whose row labels are keyed by these names.
- **`CUTOFF_UTC`.** The report used the instant of the round-4 push in two spellings: `2026-10-05T07:00:00Z`
  for `analyze_partition.py`, `analyze_timeline.py` and the three `make_tables*.py`, and
  `2026-10-05T07:05:00Z` for every other script.
- **`analyze_commits.py`** carries the four fix rounds' walls in minutes (`rw`), typed in from
  `analyze_timeline.py`'s output.
- **The fence-suite lists** in `analyze_red_causes.py`, `analyze_fast_tier.py` and `make_tables2.py reds`
  name the suites that reddened *this* run's gates.
- **The agent types** `build-fixer`, `build-executor` and `build-git` decide which agent owns a commit or a
  detached gate.

## Limits

The report's §2 (*Limits*) and §6 state what the measurement cannot say, and they bind every re-run: model
time is "not in a tool", not "the model computing"; the gate logs name only failing and slow tests; the
individual step durations exist only where a transcript kept the gate's output; nothing here measures what a
change *would* do. Three more belong to the instrument itself:

- **It is macOS-shaped.** `gate_logs.py` reads the file birth time, and the shell scripts call `xcrun`,
  `sysctl -n vm.loadavg` and BSD `find -perm +111`.
- **Command classes are heuristic** — a quote-aware split of each Bash command, classed by its most
  expensive simple command.
- **It follows one transcript format.** A change in how the harness records tool calls, background
  completions or workflow runs breaks the parser silently; `parse_transcripts.py` prints the count per class
  and the first words of what it could not class, and that line is the first thing to read.
