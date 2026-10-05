# Where the wall clock and the tokens of the rc.24 gate run went

**These are recommendations only — nothing in this report is decided.** Several of the levers in section 4 change a standing rule and are marked *needs the human's ruling* (2, 3, 10 and 12); none of the twelve was taken in the session that measured them. The next session decides, with the human, which to take.

Measured 2026-10-05 over one orchestrating session: the blind agent trial, the partial per-axis review, the
four-round fix pass and its completion audit ("the gate before 1.0.0"). Everything below is derived from the
session's transcripts, the surviving `dev/gate` logs, and one fresh full-suite run in a private clone. The
levers in section 4 come out of the tables in section 3; section 5 lists the levers the data does **not**
support.

Times are UTC. "h:mm" unless a unit is given.

## 1. Headline numbers

| | |
|---|---|
| Session wall clock, first prompt → round-4 push | **34:06** (2026-10-03 20:53 → 2026-10-05 07:00) |
| …of which some subagent or workflow was running | 27:56 (82 %) |
| …of which the orchestrator was blocked on a question to the human | 6:01 (five questions; one of them 5:13, overnight) |
| Agent-hours (sum of all subagent walls; average concurrency 1.5) | **42:48** |
| …model time (between tool calls) | 24:29 (57 %) |
| …a full `dev/gate` running | **13:16 (31 %)** |
| …every other tool call together | 5:03 (12 %) — scoped tests 1:34, rig and route drives 1:55, all cargo builds 0:26 |
| The fix pass alone (rounds 1–4, serial, so agent-hours = wall) | **23:32** |
| …a full gate running | **12:45 (54 %)** — round 4: 8:08 of 11:55 (68 %) |
| …model time | 8:07 (34.5 %) |
| …scoped test runs | 1:34 (6.7 %) |
| …cargo builds, debug and release | 0:25 (1.8 %) |
| …rig and driving the binary | 0:15 (1.1 %) |
| Full gate runs | **91**: 62 green, 23 red, 6 killed mid-test; plus 12 `--quick` runs |
| Wall inside those gates | 13:20; **95.6 % of a full gate is the nextest step** |
| The fixer during a gate | blocked for **10:13** of the 13:06 (8:57 in a foreground poll loop, 1:16 in the gate call itself) |
| Red gates | 23 runs, **3:36**; in **15** of them every failing check was a lint or a source-scanning fence that itself runs in at most 3.2 s |
| One full gate | 451 s at the start (4,393 tests) → 601–612 s at the end (4,630 tests): past the 10-minute foreground tool ceiling |
| Tokens actually processed | 68.7 M cache writes · 1,565 M cache reads · 9.75 M output, over 6,782 API requests |
| The workflow files' "tokens" (27.4 M over nine runs) | the **sum of each agent's final context size**, not usage (§3.10) |
| Cost in input-token equivalents ("cost units", §2) | **291 M**: 54 % cache reads, 30 % cache writes, 17 % output |
| …of which full-context cache rewrites after a pause > 5 min | 37.5 M tokens rewritten = **16 %** of the run's cost; 37 % of round 4's |

The five largest time sinks:

1. **Full gate runs — 13:06 for 85 completed runs** (median 574 s), almost all of it in rounds 2–4. Per commit
   the fix pass paid 13–15 minutes of gate wall.
2. **Model time — 24:29 agent-hours**, 8:07 of it on the fix pass's critical path; the review and scoring
   instruments are 87–95 % model time.
3. **Red gates — 3:36** (a subset of 1): 23 runs, each followed by a fix that took a median of 91 s and then
   another ~10-minute run.
4. **Waiting on the human — 6:01** of wall (outside the agents' time; 5:13 of it one overnight question).
5. **Orientation — 3:43**: the time 21 fixers spent before their first edit to the source (median 11 min each,
   93 % of it model time over ~104 k tokens of files read). Then scoped test runs, 1:34–1:56.

## 2. Method, and what the timestamps can and cannot say

**Sources.** (a) The orchestrating session's transcript and every subagent transcript (134 agents: 109 in nine
workflow runs, 24 single subagents, the session itself). (b) The nine workflow run files (labels, phases,
per-agent "tokens"). (c) The 103 `dev/gate` logs of the session still in the system temp directory, each with
its step-result and hygiene sibling files. (d) The trial's `.start`/`.end`/`.rc` step markers. (e) `git log` of
the fix branch, read-only. (f) One new measurement: the branch HEAD cloned into a scratch directory, built there
into its own target directory, and the gate's own test command run with per-test status lines on; then ten
tests re-run one at a time with a small shim in place of `git` and of the `jigc` binary under test, counting
and timing every child process.

**How durations are derived.**

- A **tool call** lasts from the timestamp of its `tool_use` block to the timestamp of its `tool_result`. A
  command launched in the **background** returns at once; its real duration is launch → its completion
  notification.
- **Model time** is an agent's wall minus the union of its tool-call intervals (minus idle gaps after an
  `end_turn`). It therefore contains API latency, queueing and any rate-limit wait, hook latency, and the
  harness's own overhead between a result and the next request — it is "not in a tool", not "the model
  computing".
- A **gate run** is taken from the log file itself: creation time = start, last modification = end; the step
  exit codes from the sibling steps file; the nextest wall, test count, SLOW and failing tests from the log
  text; the launching agent by matching the start time to a transcript call (four gates detached with
  `nohup … &` were attributed to the one serial fixer alive at that moment).
- The **exclusive partition** (§3.2) assigns every second of an agent to one bucket, by priority: a full gate
  running → a foreground tool call (by class) → model. A foreground poll loop waiting on a background gate is
  therefore counted once, as gate.
- **Command classes** come from a quote-aware split of each Bash command into simple commands; a compound
  command gets its most expensive class. Classification is heuristic: `scratch scripts` are mostly route
  drives; helper functions sourced from a scratch library count as driving the binary.
- **Tokens** are read from the `usage` object of every assistant message, once per message id.
  **Cost units** weight them as input-token equivalents with the usual prompt-caching multipliers — cache
  write × 1.25, cache read × 0.1, output × 5. No price is assumed, so the unit is relative.

**Limits.**

- The gate logs carry **no per-test timings**: `.config/nextest.toml` sets `status-level = "fail"`, so a log
  names only failing tests and the tests over the 60 s slow mark. Per-suite and per-test numbers (§3.7) are
  therefore from the fresh run at the branch HEAD (4,630 tests, `Summary [615 s]` against the last gate's
  583 s — a first run on a cold file cache), not from the historic gates.
- The individual durations of fmt, clippy and build exist only where a transcript captured the gate's stdout
  (40–46 of 85 runs); the hygiene advisory and the test compile are derived from file times and cargo's own
  `Finished … in` lines; the rest is a residual.
- The ten-test spawn sample and the per-invocation costs ran while another agent's gate loaded the machine for
  part of the time; the "alone" column is indicative, the child-process shares and call counts are not affected.
- Nothing here measures what a change *would* do. Every "expected saving" in section 4 is arithmetic over
  measured durations, stated with its assumption.
- A permission prompt or a human pause inside a tool call is indistinguishable from the tool running. The
  subagents ran non-interactively, so this matters only for the orchestrator's own calls.
- "Re-deriving what a previous agent established" has no timestamp. The proxies used are time-to-first-edit,
  files re-read across agents, and rows driven more than once by design.
- The count of hook-refused commands is a text match on the refusal and is approximate.
- Two subagents that started after the round-4 push (this analysis among them) are excluded.

## 3. The tables

### 3.1 Wall clock by phase

| run | what | start → end (UTC) | wall | agents | agent-hours | model | gate | other tools | context-sum "tokens" | cost units |
|---|---|---|---|---|---|---|---|---|---|---|
| `wf_ed65e6c9-b2b` | per-axis review | 03 21:50 → 03 23:14 | 1:24 | 21 | 7:21 | 6:24 | — | 0:56 | 6.56 M | 48.3 M |
| `wf_036fedaa-d57` | trial scoring | 03 22:41 → 03 23:32 | 0:50 | 42 | 3:10 | 3:01 | — | 0:08 | 5.38 M | 21.6 M |
| `wf_ebb7be3c-6ca` | tier-1 re-drive | 03 23:16 → 03 23:33 | 0:17 | 6 | 1:02 | 0:55 | — | 0:07 | 1.23 M | 6.4 M |
| `wf_3814722e-476` | fix planning | 04 05:13 → 04 05:38 | 0:25 | 8 | 1:23 | 1:17 | — | 0:06 | 1.64 M | 8.3 M |
| `wf_0c2c857d-ea8` | fix round 1 | 04 06:13 → 04 07:40 | 1:27 | 4 | 1:27 | 0:40 | 0:39 | 0:07 | 0.94 M | 8.6 M |
| `wf_9fa626da-40d` | fix round 2 | 04 07:43 → 04 13:21 | 5:37 | 8 | 5:37 | 2:34 | 2:15 | 0:47 | 3.41 M | 41.7 M |
| `wf_08cc5afb-78f` | fix round 3 | 04 13:23 → 04 17:56 | 4:32 | 7 | 4:32 | 2:21 | 1:42 | 0:28 | 2.60 M | 28.2 M |
| `wf_9c41b2de-83b` | completion audit | 04 17:58 → 04 18:48 | 0:49 | 7 | 2:58 | 2:18 | — | 0:39 | 2.35 M | 19.2 M |
| `wf_9971585b-366` | fix round 4 | 04 19:02 → 05 06:58 | 11:55 | 6 | 11:55 | 2:31 | 8:08 | 1:15 | 3.30 M | 76.4 M |
| `—` | single subagents | 03 20:55 → 05 06:59 | — | 22 | 3:19 | 2:24 | 0:30 | 0:24 | — | 22.3 M |
| `—` | the orchestrating session | 03 20:53 → 05 07:00 | 34:06 | 1 | 6:57 | 0:55 | — | 6:01 | — | 10.0 M |

The review and the trial scoring overlapped (22:41–23:14); everything from fix planning on ran one workflow at
a time. Gaps between runs: 5:39 before fix planning (the overnight question), 0:35 before round 1, 0:14
before round 4, two minutes elsewhere.

Critical path: rounds 1–4 are strictly serial — one fixer at a time in one working tree — so their wall is the
sum of their agents' walls (23:32). In the parallel runs the slowest chain set the wall: the review's
`drive → reconcile → assemble` chain (rows 9 and 10 started 20 minutes late, see §3.11), the scoring run's
`score (0:13) → verify (0:21) → assemble (0:15)`, and the audit's e2e tester (0:49 against 0:19–0:24 for the
six reviewers).

### 3.2 Where the agents' time went (exclusive partition, agent-hours)

| bucket | review | trial-score | tier1-redrive | fix-plan | round1 | round2 | round3 | audit | round4 | singles | all subagents | share |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| model | 6:24 | 3:01 | 0:55 | 1:17 | 0:40 | 2:34 | 2:21 | 2:18 | 2:31 | 2:24 | 24:29 | 57.2% |
| gate | · | · | · | · | 0:39 | 2:15 | 1:42 | · | 8:08 | 0:30 | 13:16 | 31.0% |
| rig + driving the binary | 0:50 | 0:03 | 0:04 | 0:02 | · | 0:05 | 0:05 | 0:36 | 0:04 | 0:02 | 1:55 | 4.5% |
| scoped tests | · | · | · | · | 0:03 | 0:27 | 0:15 | · | 0:48 | · | 1:34 | 3.7% |
| cargo build | · | · | · | · | 0:01 | 0:10 | 0:03 | · | 0:09 | · | 0:26 | 1.0% |
| sleep/poll (no gate running) | 0:02 | · | · | 0:01 | · | 0:01 | · | 0:01 | 0:02 | 0:14 | 0:24 | 1.0% |
| shell reads | 0:01 | 0:03 | · | 0:01 | 0:02 | 0:00 | 0:00 | 0:01 | 0:01 | 0:01 | 0:15 | 0.6% |
| clippy/fmt | · | · | · | · | · | 0:01 | 0:00 | · | 0:04 | · | 0:07 | 0.3% |
| git | · | · | 0:02 | · | · | · | · | 0:00 | · | 0:01 | 0:05 | 0.2% |
| python (patch scripts) | 0:01 | · | · | · | · | · | · | · | · | 0:02 | 0:04 | 0.2% |
| trial tooling | · | 0:00 | · | · | · | · | · | · | · | 0:02 | 0:03 | 0.1% |
| quick gate | · | · | · | · | · | · | · | · | 0:02 | · | 0:02 | 0.1% |
| **total agent wall** | 7:21 | 3:10 | 1:02 | 1:23 | 1:27 | 5:37 | 4:32 | 2:58 | 11:55 | 3:19 | **42:48** | |

### 3.3 Wall time per command class, per run

Foreground call time; a background command counts launch → notification. Not exclusive: a foreground poll and
the background gate it waits on both appear, which is why this table's gate and poll rows together exceed
§3.2's gate row.

| class | review | trial-score | tier1-redrive | fix-plan | round1 | round2 | round3 | audit | round4 | singles | all | calls |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `dev/gate` (full) | · | · | · | · | 0:39 | 2:15 | 1:39 | · | 7:26 | 0:08 | 12:08 | 87 |
| `dev/gate --quick` | · | · | · | · | · | · | 18s | · | 0:02 | · | 0:02 | 9 |
| sleep / wait / poll loops | 0:02 | 13s | 9s | 0:01 | 0:21 | 1:28 | 1:00 | 0:01 | 6:35 | 0:22 | 9:54 | 99 |
| `cargo test` / `nextest`, scoped | 0:01 | · | · | 0:02 | 0:03 | 0:27 | 0:15 | 0:17 | 0:48 | · | 1:56 | 225 |
| `cargo build --release` | · | · | · | · | 0:01 | 0:06 | 0:02 | 0:03 | 0:11 | · | 0:26 | 82 |
| `cargo build` (debug) | · | · | · | 26s | 14s | 0:04 | 0:01 | 0:02 | 0:01 | · | 0:10 | 59 |
| `cargo clippy` | · | · | · | · | 24s | 0:01 | 10s | · | 52s | · | 0:02 | 19 |
| `cargo fmt` | · | · | · | · | 4s | 30s | 42s | · | 0:04 | · | 0:05 | 36 |
| `dev/jigc-rig` (+ what the same command drove) | 0:11 | 48s | 34s | 16s | 6s | 0:01 | 35s | 0:02 | 3s | 4s | 0:17 | 428 |
| driving the binary directly | 0:09 | 0:01 | 44s | 2s | · | 0:02 | 1s | 0:01 | 10s | 0:01 | 0:16 | 605 |
| scratch scripts (mostly route drives) | 0:29 | 0:01 | 0:02 | 0:02 | 21s | 0:06 | 0:11 | 0:33 | 0:11 | 48s | 1:40 | 619 |
| trial harness / driver | · | 43s | · | · | · | · | · | · | · | 0:02 | 0:03 | 75 |
| docker / `dev/runner-faithful` | 0:08 | · | · | · | · | 47s | · | · | 0:01 | 11s | 0:11 | 49 |
| git / gh | 24s | 25s | 0:02 | 2s | 7s | 11s | 11s | 36s | 12s | 0:01 | 0:05 | 620 |
| python (patch and analysis scripts) | 0:01 | 14s | 12s | · | · | 0:02 | 7s | · | 6s | 0:14 | 0:19 | 374 |
| file reads via the shell | 0:01 | 0:03 | 17s | 0:01 | 0:04 | 0:01 | 45s | 0:01 | 0:02 | 0:03 | 0:20 | 2934 |
| `cargo install` | 0:01 | · | · | · | · | · | · | · | · | · | 0:01 | 4 |
| other cargo | 1s | · | · | · | · | 10s | 13s | · | 14s | · | 40s | 14 |

### 3.4 Agents of the fix pass, the audit, the planning and the re-drive

`gates g/r/a` = full gates green / red / killed mid-test. `peak context` is what the workflow file reports as
the agent's "tokens".

| run | agent | start (UTC) | wall | model | gate | scoped tests | other tools | calls | commits | gates g/r/a | peak context | output | cost units |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| tier1-redrive | tier1:(R1, F1) | 03 23:16 | 0:07 | 0:06 | — | — | 0:00 | 39 | — | — | 173k | 39k | 0.7 M |
| tier1-redrive | tier1:(R3, F7) | 03 23:16 | 0:09 | 0:07 | — | — | 0:01 | 42 | — | — | 201k | 43k | 0.9 M |
| tier1-redrive | tier1:(R6, K-1) | 03 23:16 | 0:16 | 0:13 | — | — | 0:03 | 65 | — | — | 227k | 78k | 1.5 M |
| tier1-redrive | tier1:(R6, D-7) | 03 23:16 | 0:10 | 0:10 | — | — | 0:00 | 59 | — | — | 175k | 55k | 1.1 M |
| tier1-redrive | tier1:(R6, D-1) | 03 23:16 | 0:09 | 0:08 | — | — | 0:00 | 46 | — | — | 238k | 53k | 1.1 M |
| tier1-redrive | tier1:(R9, F5) | 03 23:16 | 0:08 | 0:08 | — | — | 0:00 | 54 | — | — | 212k | 50k | 1.1 M |
| fix-plan | plan:promote | 04 05:13 | 0:16 | 0:15 | — | — | 0:00 | 55 | — | — | 333k | 90k | 1.8 M |
| fix-plan | plan:install | 04 05:13 | 0:15 | 0:15 | — | — | 0:00 | 71 | — | — | 324k | 88k | 2.2 M |
| fix-plan | plan:worktree | 04 05:13 | 0:14 | 0:13 | — | — | 0:01 | 49 | — | — | 234k | 80k | 1.1 M |
| fix-plan | advocate:worktree-1 | 04 05:27 | 0:06 | 0:06 | — | — | 0:00 | 36 | — | — | 156k | 37k | 0.7 M |
| fix-plan | advocate:install-1 | 04 05:28 | 0:07 | 0:05 | — | — | 0:01 | 50 | — | — | 161k | 33k | 0.7 M |
| fix-plan | advocate:install-2 | 04 05:28 | 0:05 | 0:04 | — | — | 0:00 | 30 | — | — | 114k | 25k | 0.4 M |
| fix-plan | advocate:promote-2 | 04 05:29 | 0:08 | 0:07 | — | — | 0:00 | 34 | — | — | 159k | 43k | 0.7 M |
| fix-plan | advocate:promote-1 | 04 05:29 | 0:08 | 0:08 | — | — | 0:00 | 38 | — | — | 150k | 44k | 0.7 M |
| round1 | git:open fix/rc24-tier1 | 04 06:13 | 0:00 | 0:00 | — | — | 0:00 | 9 | 0 | 0/0/0 | 29k | 1k | 0.1 M |
| round1 | record:decisions | 04 06:13 | 0:11 | 0:03 | 0:07 | — | 0:00 | 36 | 1 | 1/0/0 | 176k | 21k | 0.8 M |
| round1 | fix:(R9,F5) uninstall | 04 06:24 | 0:39 | 0:18 | 0:15 | 0:02 | 0:03 | 102 | 1 | 1/1/0 | 375k | 116k | 4.2 M |
| round1 | fix:(R1,F1) setup | 04 07:04 | 0:36 | 0:18 | 0:15 | 0:01 | 0:01 | 76 | 1 | 1/1/0 | 360k | 120k | 3.5 M |
| round2 | record:forks 2/4/5 + parking | 04 07:43 | 0:12 | 0:04 | 0:07 | — | 0:00 | 55 | 1 | 1/0/0 | 163k | 29k | 0.9 M |
| round2 | fix:(R6,K-1) one-probe | 04 07:55 | 0:33 | 0:13 | 0:16 | 0:01 | 0:01 | 90 | 1 | 2/0/0 | 324k | 130k | 3.1 M |
| round2 | fix:(R6,D-1) milestone clobber guard | 04 08:29 | 0:36 | 0:17 | 0:16 | 0:01 | 0:01 | 112 | 1 | 2/0/0 | 398k | 120k | 4.4 M |
| round2 | fix:(R6,D-7) regular file at the home | 04 09:06 | 0:48 | 0:24 | 0:09 | 0:12 | 0:01 | 151 | 1 | 1/0/1 | 488k | 164k | 6.5 M |
| round2 | fix:(R3,F7) baseline at copy-in | 04 09:54 | 0:51 | 0:22 | 0:25 | 0:02 | 0:01 | 136 | 1 | 1/2/0 | 527k | 174k | 6.9 M |
| round2 | fix:L-22 foreign worktree prune | 04 10:45 | 0:39 | 0:18 | 0:16 | 0:02 | 0:02 | 91 | 1 | 1/1/0 | 358k | 120k | 3.7 M |
| round2 | fix:own registration holding work | 04 11:25 | 0:54 | 0:27 | 0:17 | 0:04 | 0:05 | 117 | 1 | 2/0/0 | 499k | 169k | 5.9 M |
| round2 | fix:linked-worktree doc guard | 04 12:20 | 1:00 | 0:26 | 0:26 | 0:01 | 0:06 | 195 | 2 | 2/1/0 | 648k | 204k | 10.3 M |
| round3 | record:fixes as built | 04 13:23 | 0:19 | 0:10 | 0:08 | — | 0:00 | 57 | 1 | 1/0/0 | 251k | 54k | 1.8 M |
| round3 | fix:setup replacing writers: non-regular ent | 04 13:43 | 0:54 | 0:24 | 0:26 | 0:01 | 0:01 | 129 | 1 | 2/1/0 | 442k | 155k | 6.0 M |
| round3 | fix:setup replaced member: gitignored bytes | 04 14:37 | 0:42 | 0:26 | 0:11 | 0:02 | 0:01 | 116 | 1 | 1/0/3 | 389k | 166k | 4.4 M |
| round3 | fix:uninstall: a refused run never creates t | 04 15:19 | 0:19 | 0:08 | 0:09 | 0:01 | 0:00 | 47 | 1 | 1/0/0 | 208k | 57k | 1.4 M |
| round3 | fix:milestone-record door: hand edit with no | 04 15:39 | 0:42 | 0:19 | 0:18 | 0:02 | 0:01 | 95 | 1 | 1/1/0 | 364k | 113k | 3.6 M |
| round3 | fix:milestone finalize: refuse before landin | 04 16:22 | 0:44 | 0:24 | 0:09 | 0:07 | 0:03 | 123 | 1 | 1/0/0 | 456k | 150k | 4.6 M |
| round3 | fix:rename never writes through a link | 04 17:06 | 0:49 | 0:27 | 0:18 | 0:01 | 0:02 | 129 | 1 | 1/1/0 | 491k | 162k | 6.4 M |
| audit | e2e | 04 17:58 | 0:49 | 0:30 | — | — | 0:19 | 131 | — | — | 435k | 180k | 4.5 M |
| audit | review:install-teardown | 04 17:58 | 0:23 | 0:20 | — | — | 0:02 | 80 | — | — | 377k | 110k | 2.8 M |
| audit | review:create-promote-link | 04 17:58 | 0:22 | 0:18 | — | — | 0:04 | 108 | — | — | 432k | 92k | 3.9 M |
| audit | review:reconcile-baseline | 04 17:58 | 0:24 | 0:19 | — | — | 0:04 | 77 | — | — | 287k | 109k | 2.1 M |
| audit | review:linked-worktree-guard | 04 17:58 | 0:19 | 0:15 | — | — | 0:03 | 66 | — | — | 261k | 83k | 1.7 M |
| audit | review:worktree-registrations | 04 17:58 | 0:20 | 0:18 | — | — | 0:01 | 65 | — | — | 300k | 96k | 2.1 M |
| audit | review:cross-cutting | 04 17:58 | 0:18 | 0:16 | — | — | 0:02 | 92 | — | — | 252k | 83k | 2.1 M |
| round4 | fix:A | 04 19:02 | 1:03 | 0:30 | 0:23 | 0:06 | 0:02 | 139 | 1 | 1/1/1 | 532k | 207k | 7.2 M |
| round4 | fix:B | 04 20:05 | 1:41 | 0:12 | 1:16 | 0:05 | 0:07 | 123 | 6 | 6/2/0 | 397k | 158k | 7.2 M |
| round4 | fix:C | 04 21:47 | 2:11 | 0:36 | 1:18 | 0:11 | 0:04 | 244 | 5 | 5/3/0 | 675k | 298k | 17.1 M |
| round4 | fix:D | 04 23:59 | 1:14 | 0:20 | 0:48 | 0:01 | 0:03 | 126 | 4 | 4/1/0 | 444k | 157k | 7.3 M |
| round4 | fix:E | 05 01:13 | 3:11 | 0:27 | 2:29 | 0:09 | 0:04 | 234 | 9 | 9/6/0 | 622k | 278k | 20.5 M |
| round4 | fix:F | 05 04:24 | 2:33 | 0:22 | 1:52 | 0:14 | 0:04 | 217 | 10 | 10/1/1 | 628k | 263k | 17.1 M |

Single subagents:

| single subagent | type | start (UTC) | wall | model | tools | idle | calls | peak context | cost units |
|---|---|---|---|---|---|---|---|---|---|
| Map blind-trial tooling | Explore | 03 20:55 | 0:05 | 0:05 | 0:00 | 0:00 | 73 | 296k | 1.1 M |
| Map per-axis review apparatus | Explore | 03 20:56 | 0:04 | 0:04 | 0:00 | 0:00 | 55 | 213k | 0.7 M |
| Map trailer and release surface | Explore | 03 20:56 | 0:04 | 0:04 | 0:00 | 0:00 | 54 | 221k | 0.7 M |
| Open work/rc24-gate branch | build-git | 03 21:18 | 0:00 | 0:00 | 0:00 | 0:00 | 8 | 29k | 0.1 M |
| Repair run.py seed turn loop | build-fixer | 03 21:21 | 0:11 | 0:02 | 0:08 | 0:00 | 34 | 115k | 0.5 M |
| Draft review instrument rc.24 | general-purpose | 03 21:22 | 0:26 | 0:26 | 0:00 | 0:00 | 105 | 498k | 4.1 M |
| Build and verify rc.24 image | general-purpose | 03 21:23 | 0:02 | 0:01 | 0:01 | 0:00 | 20 | 123k | 0.3 M |
| Draft RC24 trial protocol | general-purpose | 03 21:24 | 0:29 | 0:28 | 0:00 | 0:00 | 102 | 485k | 4.1 M |
| Run trial preconditions | general-purpose | 03 21:54 | 0:03 | 0:02 | 0:00 | 0:00 | 29 | 107k | 0.4 M |
| Run the three trial arms | general-purpose | 03 22:18 | 0:21 | 0:04 | 0:16 | 0:00 | 37 | 155k | 0.8 M |
| Adversarially verify trial lead L-22 | general-purpose | 03 23:33 | 0:09 | 0:08 | 0:00 | 0:00 | 53 | 171k | 1.0 M |
| Persist verification, gate, commit | general-purpose | 03 23:44 | 0:12 | 0:05 | 0:00 | 0:06 | 53 | 213k | 1.2 M |
| Push branch and open PR | build-git | 03 23:57 | 0:00 | 0:00 | 0:00 | 0:00 | 8 | 30k | 0.1 M |
| Fix stale lines in trial README | general-purpose | 03 23:58 | 0:10 | 0:02 | 0:00 | 0:07 | 28 | 137k | 0.6 M |
| Push follow-up commit to PR | build-git | 04 05:12 | 0:00 | 0:00 | 0:00 | 0:00 | 7 | 29k | 0.1 M |
| Record preparation-phase observations | general-purpose | 04 05:33 | 0:14 | 0:06 | 0:00 | 0:07 | 53 | 190k | 1.2 M |
| Push record commit to PR #13 | build-git | 04 05:48 | 0:00 | 0:00 | 0:00 | 0:00 | 7 | 29k | 0.1 M |
| Drive linked-worktree finalize routes | capability-auditor | 04 05:54 | 0:08 | 0:07 | 0:00 | 0:00 | 41 | 112k | 0.6 M |
| Design the linked-worktree doc guard | Plan | 04 06:18 | 0:14 | 0:14 | 0:00 | 0:00 | 102 | 377k | 2.1 M |
| Push fix branch for safety | build-git | 04 19:08 | 0:00 | 0:00 | 0:00 | 0:00 | 7 | 29k | 0.1 M |
| Draft the fix pass findings ledger | general-purpose | 04 19:08 | 0:19 | 0:19 | 0:00 | 0:00 | 70 | 370k | 2.5 M |
| Push round 4 commits | build-git | 05 06:59 | 0:00 | 0:00 | 0:00 | 0:00 | 8 | 30k | 0.1 M |

### 3.5 The thirty longest single commands

Twenty-eight of the thirty are a full gate or a loop waiting for one, all within seconds of ten minutes.

| # | min:s | class | run | agent | start (UTC) | what the command said it was doing |
|---|---|---|---|---|---|---|
| 1 | 10:23 | cargo-test (bg) | round2 | fix:(R6,D-7) regular file at t | 04 09:21 | Run the new suite against the fix |
| 2 | 10:18 | python (bg) | single | Run the three trial arms | 03 22:24 | Run arm (a): two seeded headless turns in the trial container |
| 3 | 10:12 | gate-full (bg) | round4 | fix:F | 05 06:09 | Run the full gate for the CPL-6 change |
| 4 | 10:10 | gate-full (bg) | round4 | fix:E | 05 02:27 | Write the link-rule docs and run the full gate |
| 5 | 10:09 | gate-full (bg) | round4 | fix:E | 05 03:14 | Note the clause in the engine doc and run the full gate for F8 |
| 6 | 10:09 | gate-full (bg) | round4 | fix:F | 05 06:35 | Run the full gate for the CPL-8 change |
| 7 | 10:09 | gate-full (bg) | round4 | fix:F | 05 05:45 | Run the full gate for the F4 change |
| 8 | 10:08 | gate-full (bg) | round4 | fix:C | 04 22:57 | Run the full gate for the CPL-1 commit |
| 9 | 10:08 | poll (bg) | round4 | fix:E | 05 03:14 | Wait for the F8 gate to report |
| 10 | 10:07 | gate-full (bg) | round4 | fix:E | 05 01:26 | Run the full gate for the F2 commit |
| 11 | 10:06 | gate-full (bg) | round4 | fix:F | 05 06:20 | Re-run the full gate for the CPL-6 change |
| 12 | 10:05 | gate-full (bg) | round4 | fix:E | 05 02:04 | Write the F4 docs and run the full gate |
| 13 | 10:03 | gate-full (bg) | round4 | fix:E | 05 02:39 | Rerun the full gate for the link rule |
| 14 | 10:03 | gate-full (bg) | round4 | fix:F | 05 05:56 | Run the full gate for the CPL-4 change |
| 15 | 10:03 | gate-full (bg) | round4 | fix:E | 05 04:12 | Run the full gate for XC-7 |
| 16 | 10:03 | gate-full (bg) | round4 | fix:E | 05 03:25 | Note cell 43 in the suite header and run the full gate for F5 |
| 17 | 10:03 | gate-full (bg) | round4 | fix:E | 05 03:51 | Run the full gate for XC-5 |
| 18 | 10:03 | gate-full (bg) | round4 | fix:F | 05 05:20 | Run the full gate for the F6 change |
| 19 | 10:02 | poll (bg) | round4 | fix:E | 05 03:51 | Wait for the XC-5 gate to report |
| 20 | 10:02 | gate-full (bg) | round4 | fix:F | 05 05:08 | Run the full gate for the F3 change |
| 21 | 10:01 | gate-full (bg) | round4 | fix:F | 05 04:56 | Run the full gate for the F2 change |
| 22 | 10:01 | gate-full (bg) | round4 | fix:F | 05 06:46 | Run the full gate for the F7 provenance correction |
| 23 | 10:00 | gate-full (bg) | round4 | fix:F | 05 05:32 | Run the full gate for the F5 change |
| 24 | 9:58 | gate-full (bg) | round4 | fix:E | 05 02:51 | Run the full gate for F6 |
| 25 | 9:58 | gate-full (bg) | round4 | fix:C | 04 22:14 | Run the full gate for the CPL-5 commit |
| 26 | 9:58 | poll | round4 | fix:F | 05 06:21 | Wait for the CPL-6 gate to report |
| 27 | 9:58 | poll | round4 | fix:E | 05 02:39 | Wait for the link-rule gate to report |
| 28 | 9:58 | poll | round4 | fix:E | 05 03:25 | Wait for the F5 gate to report |
| 29 | 9:58 | poll | round4 | fix:F | 05 06:46 | Wait for the F7 gate to report |
| 30 | 9:57 | gate-full (bg) | round4 | fix:C | 04 23:12 | Run the full gate for the CPL-2 commit |

The longest commands that are neither a gate nor a wait on one:

| # | min:s | class | run | agent | start (UTC) | what the command said it was doing |
|---|---|---|---|---|---|---|
| 1 | 10:23 | cargo-test (bg) | round2 | fix:(R6,D-7) regular file at t | 04 09:21 | Run the new suite against the fix |
| 2 | 10:18 | python (bg) | single | Run the three trial arms | 03 22:24 | Run arm (a): two seeded headless turns in the trial container |
| 3 | 5:14 | script (bg) | audit | e2e | 04 18:03 | Drive D-1 occupant shape variants at the milestone door |
| 4 | 5:08 | docker (bg) | review | drive:row1 | 03 21:53 | Run the 4-CPU tarball variant and the amd64 registry variant in the container |
| 5 | 4:58 | cargo-test (bg) | audit | review:cross-cutting | 04 18:06 | Time the pass's new and grown suites with nextest in the private target |
| 6 | 4:01 | read (bg) | round1 | fix:(R9,F5) uninstall | 04 06:40 | Check for copies of the guide sentence and pinned digest literals |
| 7 | 3:42 | cargo-test (bg) | audit | review:worktree-registrations | 04 18:08 | Run the two worktree registration suites in a private target |
| 8 | 3:33 | cargo-test | round3 | fix:milestone finalize: refuse | 04 16:43 | Run the whole milestone test group |
| 9 | 3:21 | cargo-test | round4 | fix:F | 05 06:31 | Apply the CPL-8 fix and run the config group and surface fences |
| 10 | 3:20 | cargo-fmt | round4 | fix:B | 04 20:46 | Register the new git span site and run the whole finalize group |
| 11 | 3:08 | cargo-test (bg) | audit | review:create-promote-link | 04 18:15 | Run the area's four acceptance suites and engine unit tests in the private targe |
| 12 | 2:55 | script | round3 | fix:milestone finalize: refuse | 04 16:56 | Re-drive every printed route on the final release binary |
| 13 | 2:18 | cargo-test (bg) | audit | review:reconcile-baseline | 04 18:03 | Run the area's suites in the private target directory |
| 14 | 2:06 | cargo-test (bg) | fix-plan | advocate:install-1 | 04 05:32 | Run the affected suites against the spike in the scratch clone |
| 15 | 2:05 | python (bg) | round2 | fix:(R3,F7) baseline at copy-i | 04 10:21 | Restage the baseline contrast suite as three arms |

Row 1 is a hang, not work: a new test cell opened a FIFO and blocked; `cargo test` has no per-test timeout, so
the run sat until the tool's ten-minute ceiling, and the fixer then sampled the process to find out why.

### 3.6 The gate's anatomy

**Counts.** 103 gate logs: 85 full runs that completed (62 green, 23 red), 6 full runs killed mid-test by the
fixer, 12 `--quick` runs (11 of them one fixer's pre-check). Wall: 13:06 in completed full runs, 0:11 in the
killed ones, 0:04 in quick runs.

**Steps.** Of a full gate's median 574 s:

| step | median | how it was measured |
|---|---|---|
| hygiene advisory (denylist scan + gitleaks over the unpushed commits) | 6.3 s (5.0 → 8.0 as unpushed commits accumulate) | log creation → steps-file creation |
| `cargo fmt --check` | 3 s | the gate's own `==>` lines, 40 captured |
| `cargo clippy` | 2 s (0–13) | 45 captured |
| `cargo build` | 3 s (0–9) | 46 captured |
| test compile (relink of the 15 test binaries) | 9.1 s (0.1–21) | cargo's `Finished` line |
| **`cargo nextest run`** | **546 s** | nextest's `Summary` line |
| `cargo test --doc` | 1 s | 60 captured |

Summed over the 85 runs: nextest 12:31 of 13:05 (95.6 %); everything else together 0:35.

**Growth of the test step.**

| run | full gates | tests (first → last) | gate wall, median s | nextest, median s | nextest min–max s | hygiene advisory, median s | test compile, median s |
|---|---|---|---|---|---|---|---|
| singles | 4 | 4393 → 4393 | 452 | 443 | 442–447 | 5.0 | 0.1 |
| round1 | 5 | 4393 → 4419 | 468 | 447 | 445–447 | 5.0 | 6.7 |
| round2 | 16 | 4419 → 4513 | 506 | 478 | 447–500 | 6.1 | 9.6 |
| round3 | 11 | 4513 → 4570 | 547 | 516 | 500–541 | 7.7 | 7.5 |
| round4 | 49 | 4580 → 4630 | 593 | 567 | 542–591 | 6.3 | 9.7 |

From the first green of the pass to the last: +237 tests (+5.4 %), **+136 s of nextest wall (+31 %)**. The
pool was saturated throughout (15.9 of 16 test threads busy in the fresh run), so wall ≈ summed test time ÷ 16:
an added test cost **9.2 thread-seconds against a suite average of 1.63** — the pass's tests are 5.6 × the
average. The fixers' reports ("~470 s → ~550 s") understate the growth; the logs say 443 → 583–592 s.

**Slow tests the logs name** (over 60 s, every run): `planning_gate_forcing::every_gate_is_a_fence…`
(median 128 s), `author_batch_scaling::the_eight_hundred_item_document…` (126 s),
`flow_role_binding::every_object_form_pair…` (79 s), `author_batch_scaling::the_batch_apply_growth_ratio…`
(72 s). All four predate the pass. One of the pass's tests crossed the mark once
(`record_door_baseline::a_hand_edit_to_the_record…`, 64 s).

**Red runs.**

| # | start (UTC) | run | fixer | gate s | red step(s) | failing tests (suite, own seconds) | cause class | until the rerun started |
|---|---|---|---|---|---|---|---|---|
| 1 | 04 06:45 | round1 | (R9,F5) uninstall | 475 | test | help_truth 0.03; migrate_route_family 0.12; task_area_writer_registry 0.89 | fence/registry | 91 s |
| 2 | 04 07:21 | round1 | (R1,F1) setup | 480 | test | git_span_aim 0.17; setup 1.30 | mixed | 112 s |
| 3 | 04 10:16 | round2 | (R3,F7) baseline at copy-in | 519 | fmt, test | reconciliation_baseline_contrast 2.98; flow49_acceptance 2.97; file_state 0.02 | mixed | 174 s |
| 4 | 04 10:27 | round2 | (R3,F7) baseline at copy-in | 508 | clippy | — | lint only | 20 s |
| 5 | 04 11:04 | round2 | L-22 foreign worktree prune | 506 | test | message_whitespace_fence 1.33; rollback_population_registry 0.60; repo_relative_paths 0.05 | fence/registry | 118 s |
| 6 | 04 12:58 | round2 | linked-worktree doc guard | 529 | test | dry_run_findings_equal_set 0.02; git_span_aim 0.17; git_span_aim 0.17; count_fences 3.23; flow52_acceptance 0.01 | fence/registry | 147 s |
| 7 | 04 14:05 | round3 | setup replacing writers: non | 537 | test | task_area_writer_registry 1.15 | fence/registry | 90 s |
| 8 | 04 16:00 | round3 | milestone-record door: hand  | 568 | test | milestone_record_stale_base 1.06 | behaviour | 47 s |
| 9 | 04 17:35 | round3 | rename never writes through  | 561 | test | git_span_aim 0.12 | fence/registry | 35 s |
| 10 | 04 19:44 | round4 | A | 565 | test | git_span_aim 0.11 | fence/registry | 74 s |
| 11 | 04 20:14 | round4 | B | 574 | fmt | — | lint only | 21 s |
| 12 | 04 20:36 | round4 | B | 574 | test | git_span_aim 0.15 | fence/registry | 241 s |
| 13 | 04 22:14 | round4 | C | 598 | test | git_span_aim 0.19; linked_worktree_doc_home 6.32; promote_destination_shape 2.33; version_stamp_rollback 0.81 | mixed | 311 s |
| 14 | 04 23:12 | round4 | C | 594 | test | milestone_landed_attribution 1.34 | behaviour | 113 s |
| 15 | 04 23:36 | round4 | C | 583 | test | message_whitespace_fence 1.23 | fence/registry | 50 s |
| 16 | 05 00:17 | round4 | D | 592 | test | git_span_aim 0.14; foldback_truth 0.01 | fence/registry | 47 s |
| 17 | 05 01:41 | round4 | E | 593 | test | flow48_acceptance 1.57 | behaviour | 68 s |
| 18 | 05 02:04 | round4 | E | 603 | test | task_area_writer_registry 0.98 | fence/registry | 48 s |
| 19 | 05 02:27 | round4 | E | 607 | clippy, test | repo_relative_paths 0.11 | fence + lint | 111 s |
| 20 | 05 02:51 | round4 | E | 598 | test | leftover_probe_fail_closed 0.49 | behaviour | 94 s |
| 21 | 05 03:25 | round4 | E | 596 | test | replacing_writers_never_follow 4.46 | behaviour | 195 s |
| 22 | 05 03:51 | round4 | E | 600 | test | help_truth 0.02 | fence/registry | 47 s |
| 23 | 05 06:09 | round4 | F | 612 | test | migrate_route_family 0.13 | fence/registry | 39 s |

| cause class | red gates | wall |
|---|---|---|
| a source-scanning fence or registry only (a row or a count the edit owed) | 12 | 1:52 |
| behaviour test only (another suite's assertion moved) | 5 | 0:49 |
| mixed (behaviour + fence or lint) | 3 | 0:27 |
| fmt or clippy only | 2 | 0:18 |
| fence + clippy | 1 | 0:10 |
| **all** | **23** | **3:36** |

- **No flake.** Every red was followed by an edit and then a green of the same tests.
- **Every failing test was fast.** The slowest failing test in any red gate ran 6.3 s; in 21 of 23 runs the
  slowest ran under 3.3 s. The gate nevertheless took its full 8–10 minutes to report it.
- **The same fences recur.** `git_span_aim` (a registry of every production caller of one function) reddened
  seven gates for seven different fixers; `task_area_writer_registry` three; `help_truth`,
  `migrate_route_family`, `message_whitespace_fence`, `repo_relative_paths` two each. Eleven fence suites
  account for every fence red; with the suite-registration fence they are 119 tests and 135 summed seconds
  (≈ 8 s of wall).
- **A red fmt or clippy step does not stop the gate.** Four runs had a red lint step and still ran the whole
  test step (0:37 together). One fixer noticed a clippy red by reading the log early and killed the run.
- **Fix time was short.** From a red gate's end to the rerun's start: median 91 s, total 0:38 over 22 cycles.
  A red → green cycle cost 17–25 minutes, of which the fix was one or two.
- **One fixer pre-checked.** The round-4 fixer that ran `dev/gate --quick` before each full gate (11 × 18 s)
  had 1 red in 11 completed full runs; the one that did not had 6 in 15. Its one red was a registry fence,
  which `--quick` does not run.

**Killed and superseded runs.** Six full gates were killed mid-test (0:11) and four greens were superseded
without a commit (0:34): in all ten the fixer launched the gate, then drove the printed routes on the release
binary or re-read its diff, found something to change, and gated again.

**What the fixer did while a gate ran** (85 full runs, 13:06): in a foreground poll loop 8:57 (68 %), inside
the foreground gate call 1:16 (10 %), between calls 2:20 (18 %), in another tool 0:33 (4 %, mostly driving
routes). Four polls ran into the ten-minute tool ceiling and had to be re-issued; five background polls were
shadowed by a foreground poll on the same log.

**Commits against gates.**

| round | commits | full gates (green / red / killed) | gate wall | gate wall per commit | round wall per commit |
|---|---|---|---|---|---|
| 1 | 3 | 5 (3 / 2 / 0) | 0:39 | 13.0 min | 29 min |
| 2 | 9 | 17 (12 / 4 / 1) | 2:15 | 15.1 min | 38 min |
| 3 | 7 | 14 (8 / 3 / 3) | 1:42 | 14.6 min | 39 min |
| 4 | 35 | 51 (35 / 14 / 2) | 8:08 | 14.0 min | 20 min |

### 3.7 The tests

One full run at the branch HEAD in a private clone, the gate's own command with per-test status on: 4,630
tests, `Summary [615 s]`, 9,754 summed test-seconds.

| test binary | tests | summed s | share | mean s | max s | ≈ wall alone at 16 threads | added by the fix pass (tests / s) |
|---|---|---|---|---|---|---|---|
| `jigc::g_milestone` | 383 | 1937 | 19.9% | 5.06 | 55 | 121 s | 81 / 895 |
| `jigc::g_finalize` | 528 | 1794 | 18.4% | 3.40 | 48 | 112 s | 58 / 694 |
| `jigc::g_flow` | 335 | 1437 | 14.7% | 4.29 | 82 | 89 s | 2 / 3 |
| `jigc::g_migrate` | 468 | 1235 | 12.7% | 2.64 | 41 | 77 s | 40 / 234 |
| `jigc::g_doc` | 301 | 901 | 9.2% | 2.99 | 124 | 124 s | 6 / 65 |
| `jigc::g_methodology` | 192 | 657 | 6.7% | 3.43 | 141 | 141 s | 0 / 0 |
| `jigc::g_compose` | 204 | 517 | 5.3% | 2.54 | 27 | 32 s | 1 / 1 |
| `jigc::g_config` | 239 | 503 | 5.2% | 2.11 | 34 | 33 s | 1 / 13 |
| `jigc::g_item` | 174 | 334 | 3.4% | 1.92 | 30 | 29 s | 0 / 0 |
| `jigc::g_migration` | 89 | 268 | 2.8% | 3.02 | 23 | 22 s | 0 / 0 |
| `jigc::g_solo_trial_corpus` | 14 | 79 | 0.8% | 5.71 | 17 | 17 s | 0 / 0 |
| `jigc` | 562 | 42 | 0.4% | 0.08 | 3 | 2 s | 0 / 0 |
| `jigc-engine` | 1042 | 34 | 0.4% | 0.03 | 4 | 3 s | 0 / 0 |
| `jigc::g_solo_store_sweep` | 3 | 5 | 0.1% | 1.88 | 3 | 3 s | 0 / 0 |
| `jigc::bin/jigc` | 96 | 1 | 0.0% | 0.02 | 0 | 0 s | 0 / 0 |
| **all** | 4630 | 9753 | | 2.11 | | | 189 / 1907 |

- The 1,700 unit tests are 0.8 % of the step. The twelve group binaries are the rest.
- Distribution: median 0.25 s, p90 5.9 s, p99 25 s. **215 tests of 10 s or more carry 45 %** of the summed
  time; the 2,613 tests under 0.5 s carry 2 %. 47 of 602 suites carry half.
- **The fix pass's 242 tests (5.2 %) carry 19.6 %** — mean 7.9 s against 1.8 s for the rest. Six of the ten
  heaviest suites are the pass's.

The 25 heaviest suites:

| # | suite | group | tests | summed s | share of the step | mean s | max s | fix pass |
|---|---|---|---|---|---|---|---|---|
| 1 | `milestone_promote_guards` | g_milestone | 24 | 313 | 3.2% | 13.0 | 44 | new |
| 2 | `copy_in_baseline` | g_finalize | 21 | 311 | 3.2% | 14.8 | 48 | new |
| 3 | `worktree_registration_anchor` | g_milestone | 15 | 291 | 3.0% | 19.4 | 36 | new |
| 4 | `author_batch_scaling` | g_doc | 2 | 194 | 2.0% | 97.4 | 124 |  |
| 5 | `linked_worktree_doc_home` | g_finalize | 16 | 189 | 1.9% | 11.9 | 24 | new |
| 6 | `setup_install_pathspec_guard` | g_migrate | 43 | 188 | 1.9% | 4.4 | 41 | grown |
| 7 | `planning_gate_forcing` | g_methodology | 3 | 156 | 1.6% | 52.2 | 141 |  |
| 8 | `flow53_acceptance` | g_flow | 19 | 146 | 1.5% | 7.7 | 24 |  |
| 9 | `record_door_baseline` | g_milestone | 8 | 146 | 1.5% | 18.3 | 55 | new |
| 10 | `flow_role_binding` | g_flow | 5 | 143 | 1.5% | 28.7 | 82 |  |
| 11 | `commit_rejected_axis` | g_flow | 9 | 131 | 1.4% | 14.6 | 31 |  |
| 12 | `create_only_gate` | g_doc | 26 | 114 | 1.2% | 4.4 | 37 | grown |
| 13 | `doc_only_finalize` | g_finalize | 16 | 110 | 1.1% | 6.9 | 16 |  |
| 14 | `flow54_acceptance` | g_flow | 13 | 100 | 1.0% | 7.8 | 26 |  |
| 15 | `validate_previews_posture` | g_finalize | 6 | 97 | 1.0% | 16.3 | 37 |  |
| 16 | `compose_goldens` | g_compose | 6 | 95 | 1.0% | 16.0 | 20 |  |
| 17 | `migrate_rollback` | g_migrate | 11 | 94 | 1.0% | 8.5 | 9 |  |
| 18 | `promote_destination_shape` | g_finalize | 9 | 82 | 0.8% | 9.2 | 28 | new |
| 19 | `fixed_identity_axis` | g_doc | 20 | 81 | 0.8% | 4.1 | 10 |  |
| 20 | `trial_corpus_states` | g_solo_trial_corpus | 14 | 79 | 0.8% | 5.7 | 17 |  |
| 21 | `agent_co_author` | g_finalize | 8 | 78 | 0.8% | 9.8 | 47 |  |
| 22 | `home_vacated` | g_migrate | 12 | 77 | 0.8% | 6.5 | 20 |  |
| 23 | `doc_show` | g_doc | 9 | 77 | 0.8% | 8.6 | 15 |  |
| 24 | `flow48_acceptance` | g_flow | 9 | 77 | 0.8% | 8.6 | 25 |  |
| 25 | `uninstall_workbench_subject` | g_milestone | 26 | 76 | 0.8% | 3.0 | 28 | grown |

The 20 heaviest tests:

| # | s | group | test | fix pass |
|---|---|---|---|---|
| 1 | 141 | g_methodology | `planning_gate_forcing::every_gate_is_a_fence_finalize_blocks_on_each_one_held_out` |  |
| 2 | 124 | g_doc | `author_batch_scaling::the_eight_hundred_item_document_is_byte_identical_to_the_captured_golden` |  |
| 3 | 82 | g_flow | `flow_role_binding::every_object_form_pair_acks_existed_on_recreate` |  |
| 4 | 71 | g_doc | `author_batch_scaling::the_batch_apply_growth_ratio_stays_within_its_stated_bound` |  |
| 5 | 55 | g_milestone | `record_door_baseline::a_hand_edit_to_the_record_blocks_every_record_door_that_has_no_baseline` | yes |
| 6 | 49 | g_flow | `rejection_frame_outcome::the_frame_states_what_the_rollback_actually_left` |  |
| 7 | 48 | g_finalize | `copy_in_baseline::a_hand_edit_after_the_first_write_blocks_both_committing_doors_and_survives` | yes |
| 8 | 47 | g_finalize | `agent_co_author::every_committing_door_carries_the_agent_trailer_only_under_the_agent` |  |
| 9 | 44 | g_flow | `format_json_success_axis::the_driven_key_set_equals_the_declared_key_set` |  |
| 10 | 44 | g_milestone | `milestone_promote_guards::a_home_nobody_holds_and_an_update_land_under_every_conversion_setting` | yes |
| 11 | 41 | g_migrate | `setup_install_pathspec_guard::a_change_hidden_by_an_index_flag_refuses_at_every_tracked_member` | yes |
| 12 | 37 | g_finalize | `validate_previews_posture::orientation_reports_every_posture_finalize_refuses_without_refusing` |  |
| 13 | 37 | g_finalize | `copy_in_baseline::a_hand_edit_before_the_first_write_still_lands_merged` | yes |
| 14 | 37 | g_doc | `create_only_gate::an_entry_that_is_not_a_regular_file_is_an_occupied_home` | yes |
| 15 | 36 | g_milestone | `worktree_registration_anchor::the_boundary_refuses_before_landing_without_work_a_registration_holds_chain` | yes |
| 16 | 36 | g_doc | `path_arg_occurrence_axis::every_path_arg_occurrence_answers_the_whole_escape_axis` |  |
| 17 | 35 | g_milestone | `worktree_registration_anchor::the_boundary_refuses_before_landing_without_work_a_registration_holds_squas` | yes |
| 18 | 34 | g_config | `changelog_write_touch::every_doc_write_verb_reads_as_a_write_touch` |  |
| 19 | 33 | g_flow | `flow_design_altitude::done_picture_research_to_vision_to_park_idea` |  |
| 20 | 33 | g_milestone | `worktree_registration_anchor::the_keep_command_picks_a_branch_name_git_can_create` | yes |

**Audit finding XC-6, checked.** Its per-suite sums reproduce exactly from the auditor's own nextest log
(`worktree_registration_anchor` 849 s, `copy_in_baseline` 413 s, `setup_install_pathspec_guard` 409 s, …;
194 tests, 3,614 summed s, wall 272 s). Its caveat is also right and matters: that run shared the machine with
six other auditors, and its absolute times are 2–3 × the full-gate ones (`worktree_registration_anchor`: 849 s
there, 291 s in a full run). Its ranking holds except at the top: in a full run `milestone_promote_guards` and
`copy_in_baseline` (313 and 312 s) lead `worktree_registration_anchor` (291 s). Its headline holds: the pass's
suites are the bulk of the step's growth.

**Process spawn versus assertion work.** Ten tests, from the median to the slowest, each re-run alone with a
shim in place of `git` and of the `jigc` binary:

| test | in the full run, s | alone, s | git spawned by the test | jigc invocations | git spawned inside jigc | child-process share of the test's wall | in-process, s |
|---|---|---|---|---|---|---|---|
| `file_state_history_gate::git_rm_with_history_still_blocks` | 1.3 | 0.7 | 9 | 16 | 60 | 93% | 0.1 |
| `task_amend::amending_twice_rewrites_the_new_head_and_adds_no_commit` | 3.3 | 1.3 | 10 | 9 | 131 | 97% | 0.1 |
| `copy_in_baseline::an_edit_undone_after_the_copy_in_lands_with_the_stag` | 8.3 | 3.9 | 7 | 51 | 260 | 97% | 0.1 |
| `doc_only_finalize::an_unchanged_recording_is_an_empty_commit_and_a_rec` | 12.9 | 5.5 | 24 | 64 | 368 | 97% | 0.2 |
| `worktree_registration_anchor::the_boundary_refuses_before_landing_with` | 36.4 | 15.8 | 274 | 80 | 1760 | 95% | 1.8 |
| `copy_in_baseline::a_hand_edit_after_the_first_write_blocks_both_commit` | 47.8 | 25.1 | 69 | 145 | 1531 | 95% | 1.1 |
| `milestone_promote_guards::a_home_nobody_holds_and_an_update_land_under` | 43.5 | 34.6 | 44 | 165 | 1175 | 97% | 0.7 |
| `flow_role_binding::every_object_form_pair_acks_existed_on_recreate` | 82.3 | 26.2 | 194 | 105 | 219 | 97% | 1.5 |
| `author_batch_scaling::the_eight_hundred_item_document_is_byte_identica` | 124.2 | 34.4 | 5 | 3 | 43 | 100% | 0.0 |
| `planning_gate_forcing::every_gate_is_a_fence_finalize_blocks_on_each_o` | 141.2 | 32.9 | 33 | 296 | 445 | 98% | 1.0 |

- **93–100 % of a test's wall is spent waiting on child processes**; the test's own work is 0.0–1.8 s.
- The children are overwhelmingly the product under test: in the sample, 934 `jigc` invocations took 96 % of
  the child time. About a quarter of that is the `git` processes `jigc` itself spawns — 5,992 of the sample's
  6,661 `git` processes, 2–22 per invocation (6.4 on average), 49 % of them `rev-parse` or `symbolic-ref`.
- **Repository setup is not where the time is.** `jigc setup`, `git init` and `git config` together are about
  4 % of the sample's child time. The time is in the verbs each cell drives (`milestone finalize` 0.8 s,
  `task finalize` 0.4 s, `milestone add-task` 0.3 s, `doc create` 0.2 s per call, under partial load).
- The heavy tests are **matrices**: one `#[test]` loops over doors × holdings × standings and mints a fresh
  fixture per cell, replaying 80–300 `jigc` invocations.
- A test runs **2–4 × slower inside the gate than alone** (33 s alone, 141 s in the gate, for the slowest):
  16 test threads, each driving child processes, on 10 cores — load average 35–42 during the run.
- Unit costs, CPU time (user + sys) per invocation including the loop's own fork: `git --version` 4 ms;
  `jigc --version` outside a repository 3 ms, **inside one 30 ms (debug) / 19 ms (release)**; `jigc describe`
  46 / 29 ms; `jigc validate` 86 / 61 ms. A release binary is 1.4–1.6 × faster per call, not an order of
  magnitude.

**Could a cheaper tier have caught the reds?** Using the per-test durations of the full run:

| tier | tests | ≈ wall at 16 threads | red gates it would have caught entirely |
|---|---|---|---|
| fmt + clippy only (today's `--quick`) | 0 | — | 2 of 23 |
| + the twelve fence suites | 119 | 8 s | 15 of 23 |
| + every test under 2 s | 3,532 (76 %) | 81 s | 16 of 23 |
| + every test under 5 s | 4,096 (88 %) | 190 s | 19 of 23 |
| the full step | 4,630 | 583–615 s | 23 of 23 |

### 3.8 Builds

| | calls | wall | note |
|---|---|---|---|
| `cargo build --release` | 82 | 0:26 | median 15–20 s (incremental); 77 of them the fixers' route drives (0:22) |
| `cargo build` (debug) | 59 | 0:10 | 25 under 5 s |
| in a private target directory, cold | 7 by the auditors | 0:06 of agent time | five release (23–70 s) and two debug (52–72 s) builds, concurrent; plus two installs in the review and one spike build in planning |
| `cargo install` | 4 | 0:01 | the review's install rows |

A cold debug build of the workspace in a new target directory took **25 s**, the test binaries a further
**28 s**, a cold release build **22 s** (measured in the private clone). All builds of the run together are
0:36 — 1.4 % of the agent-hours. Nothing was rebuilt needlessly at a scale worth a line: the per-fixer release
builds are incremental rebuilds after that fixer's own edits.

### 3.9 Waiting and waste

| what | measured |
|---|---|
| Foreground poll loops | 91 calls, **9:00**; 8:14 of it waiting on a gate log |
| …that hit the ten-minute tool ceiling | 4 |
| Background polls shadowed by a foreground poll on the same log | 5 of 8 |
| Commands refused by a hook before running | ≈ 47 (20 an exit code read through a pipe); each one re-issued |
| Commands that hit their own timeout | 2, plus the one hung test (10 min) |
| Byte-identical commands repeated by the same agent | 30 extra runs; 0:45 of it gates re-run after a red |
| Results too large to return inline | 48 |
| Whole-file reads of large sources | `milestone.rs` 89 reads by 19 agents (1.46 M chars returned), `setup.rs` 73 by 18, `task.rs` 71 by 27; 42 % of all tool-result characters are `Read` results |
| Time before a fixer's first source edit | **3:43 over 21 fixers**, median 11 min, 22–75 calls each, ≈ 104 k tokens of results; 93 % model time |
| …per commit, one finding per fixer (rounds 1–3) | 9.8 min |
| …per commit, one area per fixer (round 4) | 2.2 min |
| `dev/jigc-rig` | 268 commands, 0:08 in total, median 0.6 s — not a cost |
| Rows driven more than once, by design | review drive 3:52 + independent reconcile 3:05 agent-hours; six tier-1 rows re-driven a third time (1:02); then reproduced by a fixer and re-driven by an auditor |

### 3.10 Tokens

The workflow files' per-agent `tokens` equal each agent's **peak context size** (27.41 M summed over the
workflow agents, against 27.41 M summed peak contexts). They say how large the conversations grew, not what
was processed: every request re-reads the whole context, so usage is context × requests.

| run | API requests | cache writes | cache reads | output | cost units | share | full-context rewrites after a pause > 5 min | tokens rewritten | rewrites' share of the run's cost |
|---|---|---|---|---|---|---|---|---|---|
| review | 1406 | 7.00 M | 276.6 M | 2.38 M | 48.3 M | 17% | 0 | 0.00 M | 0% |
| trial-score | 992 | 4.96 M | 99.1 M | 1.09 M | 21.6 M | 7% | 0 | 0.00 M | 0% |
| tier1-redrive | 251 | 1.13 M | 34.1 M | 0.32 M | 6.4 M | 2% | 0 | 0.00 M | 0% |
| fix-plan | 268 | 1.57 M | 41.4 M | 0.44 M | 8.3 M | 3% | 0 | 0.00 M | 0% |
| round1 | 177 | 2.52 M | 41.9 M | 0.26 M | 8.6 M | 3% | 5 | 1.60 M | 23% |
| round2 | 780 | 8.95 M | 249.3 M | 1.11 M | 41.7 M | 14% | 13 | 5.69 M | 17% |
| round3 | 587 | 6.28 M | 160.0 M | 0.86 M | 28.2 M | 10% | 10 | 3.79 M | 17% |
| audit | 554 | 2.47 M | 123.5 M | 0.76 M | 19.2 M | 7% | 1 | 0.15 M | 1% |
| round4 | 975 | 26.01 M | 370.8 M | 1.36 M | 76.4 M | 26% | 43 | 22.88 M | 37% |
| singles | 674 | 4.59 M | 122.3 M | 0.87 M | 22.3 M | 8% | 5 | 0.75 M | 4% |
| orchestrator | 118 | 3.23 M | 45.7 M | 0.28 M | 10.0 M | 3% | 5 | 2.60 M | 32% |
| **all** | 6782 | 68.72 M | 1564.6 M | 9.75 M | 291.1 M | | 82 | 37.47 M | 16% |

- **Cache reads are 54 % of the cost**: 6,782 requests over contexts that reach 200–675 k tokens. A fixer
  costs 40–90 k cost units per request, rising with its context.
- **Cache rewrites are the avoidable part.** The prompt cache lives five minutes; a gate takes ten. After
  each gate wait the next request rewrites the fixer's whole context at 1.25 instead of reading it at 0.1:
  82 such rewrites, 37.5 M tokens, **16 % of the run's cost and 37 % of round 4's**. The area fixers of round 4,
  with 9–10 gates each and contexts over 600 k, paid 5–7 M rewritten tokens each.
- **Token-heavy relative to what they returned:** the review's 20 row agents (44 M cost units for 20 returns
  of 9–19 k characters — each row is driven, then independently re-driven); the scoring run's 39 verifiers
  (17 M; each loads the 57 k-character protocol); the three area fixers C, E and F (17–21 M each for 5–10
  commits).
- **Structured returns are small**: 109 results, 1.4 M characters in total (≈ 350 k tokens), the largest 43 k
  characters. They are not a cost.

### 3.11 The trial and review instruments

| trial step | wall |
|---|---|
| image build (`build-image.sh --registry`) | 0:33 |
| image verification (seven checks) | 0:18 |
| preconditions (corpus instantiate, check, adopt, carry, walk) | 3:17 for the agent |
| arm c | 1:50 |
| arm b | 1:58 |
| arm a | 10:18 |
| debrief b, debrief a | 0:36, 1:36 |
| the three arms and two debriefs, run one after another | 16:18 |
| drafting the protocol ‖ drafting the review instrument (two single agents) | 0:29 ‖ 0:26 |

| run | phase | agents | span | sum of agent walls | longest agent | max alive at once | model | tools | context-sum "tokens" | cost units |
|---|---|---|---|---|---|---|---|---|---|---|
| review | Drive | 10 | 0:47 | 3:52 | 0:28 | 8 | 3:18 | 0:34 | 3.31 M | 27.0 M |
| review | Reconcile | 10 | 0:39 | 3:05 | 0:26 | 7 | 2:43 | 0:22 | 2.65 M | 17.1 M |
| review | Assemble | 1 | 0:23 | 0:23 | 0:23 | 1 | 0:23 | 0:00 | 0.60 M | 4.2 M |
| trial-score | Verify | 39 | 0:21 | 2:33 | 0:08 | 8 | 2:25 | 0:07 | 4.51 M | 17.0 M |
| trial-score | Score | 2 | 0:13 | 0:20 | 0:13 | 2 | 0:20 | 0:00 | 0.49 M | 2.2 M |
| trial-score | Assemble | 1 | 0:15 | 0:15 | 0:15 | 1 | 0:15 | 0:00 | 0.39 M | 2.4 M |
| tier1-redrive | Verify | 6 | 0:17 | 1:02 | 0:16 | 6 | 0:55 | 0:07 | 1.23 M | 6.4 M |
| fix-plan | Plan | 3 | 0:16 | 0:46 | 0:16 | 3 | 0:44 | 0:01 | 0.89 M | 5.2 M |
| fix-plan | Advocate | 5 | 0:10 | 0:36 | 0:08 | 5 | 0:32 | 0:04 | 0.74 M | 3.2 M |
| audit | Audit | 7 | 0:49 | 2:58 | 0:49 | 7 | 2:18 | 0:39 | 2.35 M | 19.2 M |

- Both instruments are **model-bound**: 87 % of the review's agent time and 95 % of the scoring run's is
  between tool calls. Their tools — rig, drives, containers — are 0:56 and 0:08.
- **A cap of eight concurrent agents shaped both.** The review's drive rows 9 and 10 waited 20 and 21 minutes
  for a slot, so the phase spanned 0:47 against a longest agent of 0:28. The scoring run's 39 verifiers went
  through eight slots in 0:21; the longest took 0:08.
- The review's assembler read all twenty returns into one 595 k context and took 0:23; the scoring assembler
  0:15.

### 3.12 What each standing rule cost in this run

| rule | measured cost | what it bought, as far as the data shows |
|---|---|---|
| Every commit is preceded by a full `dev/gate` | 85 full runs, 13:06; at one green per commit the floor for 54 commits is ≈ 8:20 | 8 of the 23 reds carried a behaviour failure a fence tier would not have shown (for example `flow48_acceptance`, in a different test group from the setup suites that fixer was editing) |
| …including doc-only record commits | 7 gates, 0:54 | no red among them |
| One finding per commit | round 4: 35 commits by 6 fixers = 35 green gates where one per fixer would be 6: **29 gates, 4:41** | each of round 4's 14 reds pointed at exactly one finding's change |
| Fixers serial, one working tree | the fix pass's 8:07 of model time and 1:34 of scoped tests ran end to end; nothing overlapped | no merge, no conflict, no second target directory |
| Each fixer builds a release binary to drive printed routes | 77 builds, 0:22; the drives themselves 0:15 | a drive or a diff re-read found something to change in each of the ten killed or superseded gates |
| Each auditor builds its own binary in a private target | 7 cold builds, 0:06 of agent time, ≤ 72 s of wall (they ran concurrently) | isolation from the tree a fixer might touch |

## 4. Ranked recommendations

Ranked by measured time addressed. "Needs the human's ruling" marks a change to a standing rule.

**1. Make the gate tiered and fail-fast: lint → a fast tier → the rest.**
Addresses the 3:36 of red gates. Stop after a red fmt/clippy/build step (today the gate runs the test step
regardless: four runs, 0:37). Then run a first test tier — the unit tests, the fence and registry suites,
and optionally every test under ~2 s — and stop if it is red; then the remainder. A green gate runs exactly
the tests it runs today.
*Expected saving:* 15–16 of the 23 reds would have stopped within 1–2 minutes instead of 8–10: **≈ 2 h per
pass of this shape**, plus the ~16 full-context cache rewrites those waits caused (≈ 8 M tokens).
*Risk:* a red reports only the tier that failed, so a mixed red (3 of 23) takes one more short cycle. The tier
list must not rot: derive it (a suite that spawns no child process, or a recorded timing table), do not
hand-list it.
*Rule:* none changed — the full gate still precedes every commit. It does change `dev/gate`'s
run-every-step-after-a-red behaviour, which `dev_gate_report.rs` may pin.
The same tier as a stand-alone command (`dev/gate --fences`, ≈ 40–60 s with the relink) is the pre-flight the
fixers lacked: `--quick` cannot see a registry row, and that is what reddened 13 gates.

**2. Do not block the fixer on the gate: gate the commit candidate in a snapshot worktree with its own
target, and let the fixer start the next finding.**
Addresses the 10:13 a fixer sat blocked inside gates. In round 4 the non-gate work per commit was 6.5 minutes
against 9.7 minutes of gate, so nearly all of it could run under the previous commit's gate.
*Expected saving:* **≈ 3 h on a round-4-shaped round** (11:55 → about 8:30); nothing for a one-commit fixer.
*Risk:* a red arrives after the next finding was started (fix forward, or rebase the candidate); the gate and
the fixer's scoped tests compete for the same 10 cores, so both slow down; a second target directory (cheap:
25 s + 28 s cold, measured). It also closes a hazard seen ten times in this run: the tree was edited while a
gate was reading it.
*Rule:* **needs the human's ruling** — "every commit is preceded by a full gate" becomes "no commit reaches
the branch tip without a green gate on exactly its tree".

**3. Alternatively: a fast per-commit check and one full gate per fixer.**
Addresses the 29 of round 4's 35 green gates that one-gate-per-finding added (4:41). Per commit: lint, the
fast tier and the test groups the commit touched (30–120 s each alone); the full gate once when the fixer is
done, and once at the end of the round.
*Expected saving:* **≈ 3.5 h on a round-4-shaped round.** Not additive with 2 — both remove the same waiting.
*Risk:* the 8 reds with a behaviour failure would surface late and across up to ten commits; attribution
costs more than the 1–5 minutes it cost here.
*Rule:* **needs the human's ruling** — it changes "every commit is preceded by a full `dev/gate`".

**4. Keep the prompt cache warm across a wait: poll in slices under five minutes.**
Addresses 37.5 M rewritten tokens (16 % of the run's cost, 37 % of round 4's). A wait loop that returns after
at most four minutes costs one cache read per slice instead of one full rewrite per gate.
*Expected saving:* **≈ 35 M cost units, 12 % of the run**, at no wall cost. Recommendations 1–3 shrink the
same number by shrinking the waits.
*Risk:* none found; two extra short turns per gate. *Rule:* none.

**5. Take the pass's test cost back out of the suite, and keep new suites from adding it.**
Addresses the +136 s the pass added to every gate (the step is now 31 % longer than it was two days earlier)
— 19.6 % of the step in 5.2 % of the tests. The heavy tests replay 80–300 `jigc` invocations because every
matrix cell mints its fixture from nothing. Build a state once per test (or once per suite) and hand each cell
a copy; `TrialCorpus::copy_state` exists and 17 suites use it.
*Expected saving:* if the pass's 1,914 summed seconds halve, **≈ 60 s per gate (10 %)** — 1:25 over a pass of
85 gates. Unmeasured; the estimate assumes half of each cell's invocations are state-building.
*Risk:* `copy_state` refuses a worktree-bearing state, and the heaviest suites are worktree-bearing, so
they need a re-provision step after the copy; shared state invites cross-cell contamination.
*Rule:* none. A cheap guard: have the gate print the ten heaviest suites on every run, so a suite's cost is
seen when it lands rather than by an audit.

**6. Put the per-test timings in the gate log.**
Addresses nothing by itself; it is why this analysis had to re-run the suite. nextest writes a JUnit report
with every test's duration when a profile asks for one, without changing what the terminal shows. Also write
each step's elapsed seconds into the log (they are on stdout only). *Risk, rule:* none.

**7. Drive the printed routes and re-read the diff before launching the gate, not during it.**
Addresses six killed and four superseded gates: 0:45. *Expected saving:* that. *Risk, rule:* none — it is an
ordering inside the fixer's loop.

**8. Group fixes by area from the first round.**
Addresses orientation and per-commit overhead: rounds 2–3 (one finding per fixer) cost 38–39 minutes of wall
and 9.8 minutes of orientation per commit; round 4 (one area per fixer) cost 20 and 2.2.
*Expected saving:* had the 16 commits of rounds 2–3 run at round 4's rate, **up to 4.7 h** — an upper bound,
the findings differ in size.
*Risk:* contexts grow (450–675 k for an area fixer), which raises cost per request and the size of every
cache rewrite; pair it with 4. *Rule:* none.

**9. Run scoped tests through nextest with a short terminate timeout.**
Addresses one ten-minute hang (a test blocked on a FIFO under `cargo test`, which has no per-test timeout).
A scoped profile with a 30–60 s terminate turns that into a named timeout. *Expected saving:* 0:10 here.
*Risk, rule:* none.

**10. Let a doc-only commit pass on lint plus the fast tier.**
Addresses 7 full gates for record commits: 0:54. The fences that read the record files are in the fast tier.
*Expected saving:* ≈ 0:45. *Risk:* a doc that a slow test reads (a guide embedded into the installed skill)
would not be seen until the next full gate.
*Rule:* **needs the human's ruling** — it changes "no commit is gate-exempt by its path".

**11. Raise or stage the cap of eight concurrent agents for model-bound fan-outs.**
Addresses ≈ 0:20 in the review (two rows queued) and ≈ 0:13 in the scoring run (39 verifiers through eight
slots). *Expected saving:* ≈ 0:30 of wall. *Risk:* rate limits; unmeasured. *Rule:* none.

**12. Outside the scripts and the tests — the product's own start cost.**
Observed, not asked for: an invocation of `jigc` inside a repository costs about 30 ms of CPU in the debug
build before it does anything (3 ms outside one), and it spawns 2–22 `git` processes per verb, half of them
`rev-parse` or `symbolic-ref`. The suite makes tens of thousands of such invocations (an earlier measurement
counted ~25 k `jigc` and ~174 k `git` per run at 4,222 tests). Resolving the repository once per process and
loading packs lazily would shorten every gate and every user's every command.
*Expected saving:* unmeasured; on the order of 10 % of the test step if the start cost halves.
*Rule:* **needs the human's ruling** — the pack-load freeze assertion is an invariant, and this is a product
change, not a tooling one.

## 5. Where the data does not support an expected lever

- **A shared prebuilt release binary, a compiler cache, private-target reuse.** All builds of the run are
  0:36 (1.4 % of agent-hours). A cold debug build is 25 s, a cold release build 22 s. Sharing one binary among
  the seven auditors would have saved under a minute of a 50-minute audit.
- **Parallel fixers in worktrees with private targets.** The gate saturates the machine: 15.9 of 16 test
  threads busy, load average 35–42 on 10 cores, tests 2–4 × slower inside the gate than alone, and the
  auditors' concurrent scoped runs 2–3 × slower than the same suites in a full gate. Two gates at once would
  each take about twice as long. Only the model time (34.5 % of the fix pass) parallelises, and
  recommendation 2 already overlaps it inside one fixer without a second writer.
- **A shared "fresh repository" fixture.** `git init` + `jigc setup` + `git config` are ≈ 4 % of the sampled
  tests' child time. The cost is the verbs each test drives, not the repository it drives them in.
- **`dev/jigc-rig`.** 268 uses, eight minutes in total, median 0.6 s. **`dev/runner-faithful` and docker:**
  eleven minutes in total.
- **The gate's non-test steps.** Hygiene advisory, fmt, clippy, build, test compile and doctests are ≈ 25 s
  of 574 (4.4 %). Reordering or caching them moves nothing; stopping on a red one does (recommendation 1).
- **The four long-standing slow tests.** 418 summed seconds, 4.3 % of the step — about 26 s of wall if they
  vanished. They matter only as the floor: no gate can finish faster than its slowest test (141 s in the
  gate), which becomes relevant only after the step is under about three minutes.
- **Target filters on scoped test runs.** Runs with `--test <group>` and runs with only a name filter have
  the same median (16 s) and mean (25 s against 23 s).
- **More nextest threads.** The pool is already full and the cores oversubscribed. Fewer threads (10–12) might
  cost nothing or help slightly; not measured.
- **A release-profile binary under test.** 1.4–1.6 × per invocation, and an earlier measurement of the whole
  workspace at `opt-level = 1` found 6 % on the gate.
- **Tooling in the trial and the review.** Their tools are 4–13 % of their agent time. The trial's five
  operator steps ran one after another for 16 minutes; running the arms side by side would save about four.

## 6. What could not be measured, and why

- **Per-test timings of the historic gates** — the logs do not carry them (§2). The fresh run stands in.
- **The effect of any recommendation** — none was tried. In particular: thread counts, shared states in the
  heavy suites, two gates at once, a fast tier's real wall including its relink.
- **The price** — only relative cost units.
- **How much of "model time" is the model** rather than latency, queueing or rate limiting.
- **Individual fmt/clippy/build durations** for the 39–45 gates whose stdout no transcript kept.
- **How many `jigc` and `git` processes a whole gate spawns today** — sampled on ten tests only; the whole-run
  count quoted is the earlier measurement at 4,222 tests.
- **Re-derivation between agents in the semantic sense** — only its proxies (§3.9).
- **Whether the behaviour reds lay in a group the fixer's commit touched** — it would need each commit's file
  list joined to each red; not done.
- **The human's side** — the six hours are the time a question stood open, not the time the answer took.

## 7. The scripts

All under [`scripts/`](scripts/README.md) beside this report (`perf/scripts/`, whose README gives the inputs they
need and the order to run them in), standard-library Python 3 and POSIX shell; each opens with a comment saying
what it reads and prints. They print aggregates only and never echo a transcript line.

| script | what it does |
|---|---|
| `parse_transcripts.py` | every transcript → `data/agents.json`, `data/calls.jsonl` (one sanitised record per tool call, classified) |
| `gate_logs.py` | every surviving gate log and its sibling files → `data/gate_logs.json` |
| `analyze_overview.py` | per run and per agent: start, end, wall, model/tool split, tokens |
| `analyze_timeline.py` | the session's wall clock: coverage, concurrency, gaps |
| `analyze_partition.py` | the exclusive partition (§3.2) |
| `analyze_classes.py` | time per command class per run or agent; the longest commands |
| `analyze_gates.py` | the gate ledger: each log joined to its launching agent and classified |
| `analyze_reds.py`, `analyze_red_causes.py` | failing tests of red gates, red → green cycles, cause classes |
| `analyze_gate_steps.py`, `analyze_gate_wait.py` | step anatomy and growth; what the fixer did during a gate |
| `analyze_commits.py` | commits against gates; superseded greens |
| `analyze_builds.py` | builds and scoped test runs |
| `analyze_waste.py`, `analyze_orientation.py` | polls, refusals, repeats, context fill, time to first edit |
| `analyze_tokens.py` | usage per request; cache rewrites after a pause |
| `analyze_instruments.py`, `trial_steps.py` | the review, scoring and trial timings |
| `measure_suite.sh` | clone the HEAD, build in a private target, run the suite with per-test status |
| `new_tests.py`, `analyze_suite.py`, `analyze_fast_tier.py` | the pass's tests; per-binary, per-suite, per-test sums; tier coverage of the reds |
| `shim/shim.c`, `spawn_probe.sh`, `probe_all.sh`, `analyze_spawns.py` | the child-process shim and the ten-test sample |
| `spawn_unit_cost.sh`, `verb_cost.sh` | CPU per invocation of `git` and of debug and release `jigc` |
| `make_tables.py`, `make_tables2.py`, `make_tables3.py`, `top_commands.py`, `build_report.py` | the tables of this report, and its assembly |
| `explore1.py` … `explore4.py` | the shape explorations the parser was written from |
