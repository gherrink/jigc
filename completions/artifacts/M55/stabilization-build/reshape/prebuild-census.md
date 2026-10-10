# Prebuild census — what the canary's agents spent their turns on, and what of it could be built once

Read-only count over the small canary's agent transcripts (`completions/artifacts/M55/stabilization-build/canary-one/`), the harness (`.claude/workflows/stabilize.js`, `dev/stabilize-step`), the review record (`completions/artifacts/M55/per-axis-review-rc24/`) and the test tree. Nothing was built, driven or edited. Sibling counts of the same day, not repeated here: `completions/artifacts/M55/stabilization-build/reshape/repro-census.md` and `route-registry-facts.md`.

## How the count was made

- **Source.** The transcripts stand in the user-level Claude projects directory, under the entry of the canary's clone: one session file and, per workflow invocation, a directory of agent transcripts with a `.meta.json` each (agent type, label) and a `journal.jsonl` (started / result / failed). 100 agents for the first invocation, 324 for the second (25 returned, 299 lines without a result of which 297 are the dead launches). This is how `data/agents-attempt-*.tsv` and `data/agents-by-role.json` were derived; the totals here reproduce them: **2,881 model turns, 3,651 tool calls, 4,592,512 output tokens, 328,976,616 cache-read tokens** for the first invocation.
- **A turn** is one assistant message id with its `usage` block (output, cache-read, cache-write). A turn's tokens are split evenly over its tool calls, and a call's share evenly over its categories.
- **Classification** is a script over every call. A `Bash` command is split quote-aware into simple commands, here-document bodies are taken out and classified by where they are written (a `.sh`/`.py` is a driver script; later calls that run that script inherit what its body does), and each simple command gets a category from its head word and the paths it names. Read / Write / Edit by path. A call that both builds a state and drives a binary counts half each.
- **Categories.** (a) orientation — agent definitions, workflow and design docs, `DECISIONS.md`, the harness's and the tools' source and `--help`, the clone's git state; (a\*) the run's own record — opening, ledger, clauses, scope, other agents' reports; (b) product source and tests; (c) building a state — rig, `git init`, `mktemp`, plants, git config; (s) writing a driver script (split out of c/d because it is where typing goes); (d) driving the candidate; (e) driving the previous release; (o) reading back its own evidence files; (f) waiting; (g) builds and gates of its own; (h) the report, the record tool's writing calls, the structured return; (i) hashes, host probes, other.
- **Checked by eye.** About 250 calls while the script was being written (three rounds, each fixing a systematic error: pipes inside quoted grep patterns, `git status` inside a rig counted as the clone, a product `--help` file counted as tooling). On the final version 100 random verifier `Bash` calls were read: **92 right, 4 wrong, 4 right in part**; the errors are c↔o confusions with no dominant direction.
- **Three biases, stated once.** (1) A turn's output tokens include the reasoning that *precedes* its call, so a turn that thinks about a result and then issues an incidental `grep` charges the thinking to (a). This inflates (a) for triage badly (four turns of 40–57 k tokens) and for verifiers somewhat. The second output table below removes it: it counts what was *typed into calls* and leaves the rest as reasoning. (2) Cache-read by category says in which turns the context was re-read, not what filled it; the third table attributes it by source instead, and is a model (see there). (3) Agent-minutes are spans between tool results; they include generation and execution and cannot be split.

## 1. Where the turns go

### 1a. Totals per role (first invocation unless marked)

| role | agents | turns | calls | output tokens | cache-read | agent-min | fixed prefix × turns, share of cache-read |
|---|---:|---:|---:|---:|---:|---:|---:|
| reporter, source review | 2 | 124 | 151 | 135,827 | 19.4 M | 30.1 | 20% |
| reporter, driving (row driver, reconciler, three for the scripted arm) | 5 | 203 | 257 | 274,612 | 24.9 M | 67.7 | 38% |
| triage | 4 | 118 | 159 | 605,081 | 20.2 M | 103.0 | 26% |
| triage, second invocation | 1 | 32 | 40 | 98,105 | 3.8 M | 14.5 | 26% |
| verifier | 61 | 2,095 | 2,672 | 3,059,834 | 218.1 M | 641.1 | 30% |
| verifier, second invocation (returned) | 19 | 615 | 790 | 991,054 | 61.8 M | 199.6 | 31% |
| scope | 1 | 34 | 43 | 54,400 | 3.3 M | 10.2 | 31% |
| preflight | 2 | 27 | 58 | 19,874 | 1.2 M | 26.0 | 67% |
| advocate | 1 | 123 | 139 | 195,388 | 28.1 M | 83.7 | 14% |
| independent drive | 1 | 69 | 84 | 99,964 | 11.5 M | 54.6 | 29% |
| record executor | 1 | 3 | 3 | 118,460 | 0.3 M | 13.4 | — |
| tool steps | 22 | 85 | 85 | 29,072 | 2.0 M | 73.1 | — |

Verifiers are 66.6% of the stage's output tokens, triage 13.2%, the seven reporters 9.0%, the fork's two agents 6.5%, the record executor 2.6%.

### 1b. Share of model turns by category

| role | a | a\* | b | c | s | d | e | o | f | g | h | i |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| reporter, source review (2) | 16% | 2% | 41% | 8% | 4% | 8% | 1% | 11% | 0 | 0 | 7% | 2% |
| reporter, driving (5) | 22% | 6% | 10% | 13% | 4% | 18% | 4% | 8% | 1% | 3% | 9% | 2% |
| triage (4) | 47% | 29% | 0 | 1% | 0 | 0 | 0 | 12% | 0 | 0 | 11% | 0 |
| triage, second (1) | 24% | 48% | 0 | 0 | 0 | 0 | 0 | 22% | 0 | 0 | 7% | 0 |
| verifier (61) | 26% | 5% | 16% | 13% | 3% | 13% | 3% | 10% | 0 | 0 | 8% | 2% |
| verifier, second (19) | 31% | 5% | 16% | 11% | 2% | 10% | 2% | 12% | 0 | 0 | 10% | 2% |
| scope (1) | 34% | 5% | 35% | 0 | 0 | 0 | 0 | 6% | 0 | 0 | 19% | 1% |
| preflight (2) | 23% | 4% | 0 | 6% | 0 | 1% | 0 | 25% | 11% | 4% | 19% | 6% |
| advocate (1) | 13% | 2% | 15% | 17% | 2% | 12% | 2% | 29% | 0 | 3% | 3% | 3% |
| independent drive (1) | 7% | 4% | 5% | 14% | 6% | 13% | 3% | 30% | 7% | 4% | 7% | 1% |

Cache-read by category follows the turn shares within three points for every role but one cell (verifier h: 8% of turns, 12–15% of cache-read — the report is written last, on the largest context). Agent-minutes differ where a call is long: preflight f = 83% of its minutes (21.6 of 26.0 waiting for the trial image); independent drive f = 46% (25 minutes waiting for its own gates); advocate o = 35% (mostly polling its own gate's log); verifier h = 23% (148 of 641 agent-minutes), c = 16%, d = 13%, e = 4%.

Sub-kinds of the verifiers' (a), both invocations, calls: tooling help and source 312 · design docs 219 · `DECISIONS.md` 205 · `implementation/` docs 189 · the clone's git state 79 · other records under `completions/` 21.

### 1c. Output tokens: typed into calls, and the rest

Estimated from the characters of each call's input at 2.61 characters per token (the upper decile of 127 turns whose input exceeded 20,000 characters — turns with least reasoning; the median is 2.46). k tokens, both invocations.

| role | output | typed: report, record calls, return (h) | typed: scripts, state, drives (c, s, d, e) | typed: reads and probes (a, a\*, b, o, i, f, g) | not typed — reasoning |
|---|---:|---:|---:|---:|---:|
| verifier (80) | 4,051 | 1,269 (31%) | 526 (13%) | 363 (9%) | 1,908 (47%) |
| triage (5) | 703 | 280 (40%) | 1 | 26 (4%) | 424 (60%) |
| reporters (7) | 411 | 132 (32%) | 71 (17%) | 39 (9%) | 189 (46%) |
| scope | 54 | 33 (60%) | 0 | 7 | 20 (37%) |
| advocate | 195 | 46 (23%) | 33 (17%) | 19 | 97 (50%) |
| independent drive | 100 | 19 (19%) | 20 (20%) | 9 | 52 (52%) |
| record executor | 118 | 100 (84%) | 0 | 0 | 19 |

- **A verifier's report.** 80 final reports hold 2,016,004 characters (median 24,380); 403,559 of them (20%) stand inside code fences. **31 of the 80 wrote the whole report more than once** (21 of 61, 10 of 19): 911,167 characters of superseded writes — of the 32 repeated writes, 28 followed further commands (more checks, then the file again whole) and 4 followed nothing. The record tool refused none: 80 `report` calls, 0 errors.
- **Triage types every entry twice.** Each graded entry stands in the report (final writes 233,172 characters over the four passes, plus 103,787 superseded — two passes wrote the report twice) and again in the structured return (273,876 characters; 34, 56, 73 and 209 entries).
- **The record executor re-typed its prompt.** Its prompt is 234,529 characters; the file it wrote, 231,765 characters, is a **verbatim substring of that prompt**. 118,460 output tokens and 13.4 minutes, in sequence, for a copy.

### 1d. Cache-read by what put the tokens into the context (a model)

Exact part: the fixed prefix — what the first turn of an agent reads and writes before any call (system prompt, tool definitions, the project and user instruction files, the agent definition, the prompt) — times the agent's turns. For a verifier that prefix is 31.3–31.4 k tokens. Modelled part: every later turn re-reads everything that entered before it; each tool result is weighted by its bytes ÷ 3.3 and each turn's own output by its tokens, times the turns that follow, and scaled per agent to the measured total (median scale 1.16, so the byte assumption is 16% short).

| what entered the context | all 100 agents | verifiers (61) |
|---|---:|---:|
| the fixed prefix — **exact** | 92.9 M (28%) | 64.2 M (29%) |
| its own earlier output (reasoning, commands, the report) | 82.9 M (25%) | 53.5 M (25%) |
| product source and tests | 31.6 M (10%) | 18.4 M (8%) |
| another agent's report — for a verifier, the finding's source | 27.8 M (8%) | 17.4 M (8%) |
| its own evidence read back | 14.4 M (4%) | 8.4 M (4%) |
| candidate drives' output | 13.4 M (4%) | 10.6 M (5%) |
| `DECISIONS.md` | 13.1 M (4%) | 10.6 M (5%) |
| design docs | 10.7 M (3%) | 10.3 M (5%) |
| the record tool's help and report rules | 9.9 M (3%) | 4.3 M (2%) |
| the rig's help | 5.8 M (2%) | 5.1 M (2%) |
| previous-release drives' output | 3.0 M (1%) | 1.4 M (1%) |

The project instruction file is 50,027 characters of that prefix in every agent (read from the transcript's own instruction attachment); the system prompt with the agent definition is 13,904. At 3.3 to 4.0 characters per token that file is 12.5–15.2 k of the 31.4 k tokens, hence **36–44 M cache-read tokens** over the stage's 2,881 turns — an estimate on an exact base.

### 1e. Repetition

**Files read by how many agents** (Read tool and `sed`/`cat`/`grep` naming the path; 105 agents that returned over both invocations; bytes are tool-result bytes, split across the files one call names). 306 distinct files; 129 of them read by exactly one agent.

| # | file | agents | calls | bytes returned | roles |
|---:|---|---:|---:|---:|---|
| 1 | the run's `opening.md` | 94 | 105 | 302,546 | verifier 80 (whole), triage 5, reporters 6, advocate, scope, preflight |
| 2 | `DECISIONS.md` | 91 | 359 | 1,527,881 | verifier 80, triage 5, reporters 4, advocate, scope |
| 3 | `implementation/pinning.md` | 84 | 182 | 544,880 | verifier 75, reporters 7, triage 2 |
| 4 | `implementation/decisions-pending.md` | 41 | 71 | 265,513 | verifier 35, triage 3 |
| 5 | `design/validation.md` | 37 | 74 | 222,870 | verifier 37 |
| 6 | `implementation/stabilization-workflow.md` | 36 | 83 | 316,405 | verifier 30, triage 2, reporters 2 |
| 7 | `crates/cli/src/task.rs` | 35 | 76 | 288,361 | verifier 32 |
| 8 | `design/surface-contract.md` | 33 | 65 | 309,724 | verifier 32 |
| 9 | `design/storage.md` | 30 | 48 | 106,085 | verifier 28 |
| 10 | `design/doc-read-surface.md` | 26 | 46 | 252,044 | verifier 23, reporters 3 |
| 11 | `design/finalize.md` | 24 | 66 | 378,792 | verifier 23 |
| 12 | `crates/cli/src/doc.rs` | 24 | 64 | 293,565 | verifier 16, reporters 7 |
| 13 | `crates/cli/src/setup.rs` | 19 | 58 | 298,413 | verifier 17 |
| 14 | `design/write-commands.md` | 18 | 36 | 115,024 | verifier 18 |
| 15 | `crates/cli/src/milestone.rs` | 17 | 27 | 97,739 | verifier 15 |
| 16 | `crates/cli/src/cli.rs` | 17 | 47 | 132,867 | verifier 13, reporters 3, scope |
| 17 | `crates/engine/src/store.rs` | 17 | 31 | 74,987 | verifier 15 |
| 18 | `crates/cli/src/render.rs` | 16 | 28 | 77,900 | verifier 9, reporters 5 |
| 19 | `crates/cli/src/orphan.rs` | 16 | 37 | 116,870 | verifier 12, reporters 3 |
| 20 | `design/reconciliation.md` | 16 | 29 | 59,359 | verifier 16 |
| 21 | `design/command-output-contract.md` | 16 | 25 | 76,667 | verifier 16 |
| 22 | `design/project-setup.md` | 15 | 23 | 79,042 | verifier 13 |
| 23 | the run's `clauses.md` | 15 | 15 | 22,911 | verifier 11, triage 3 |
| 24 | `design/assistant-adapter.md` | 13 | 28 | 103,046 | verifier 12 |
| 25 | the run's `ledger.md` | 12 | 14 | 25,568 | verifier 10, triage 2 |

Not in this table because they are a tool's output rather than a file: `dev/stabilize-record --help` (97 agents, 196 calls that fetch it or grep it for the report rules — host paths, placeholders, fences, the last line) and `dev/jigc-rig --help` / `--list-states` (81 agents, 148 calls). By area: `completions/` 3.86 MB over 99 files (other agents' reports mostly), design docs 1.95 MB over 21, `crates/cli/src` 1.84 MB over 39, `DECISIONS.md` 1.53 MB, `implementation/` 1.18 MB over 10, `crates/cli/tests` 0.60 MB over 86.

Two things the table shows. Only three files are read by nearly everyone, and none of the three is product: the opening, the exit rule's text looked up in `DECISIONS.md` (76 agents grep for it; the run's own `clauses.md` is read by 15), and the repro-block format in `pinning.md`. Below them the reading is per finding: the most-read source file is read by 35 agents in 76 different slices.

**Facts re-derived** (agents · calls · turns that consist of nothing else, among the 80 verifiers):

| fact | agents (verifiers) | calls | verifier turns of nothing else |
|---|---:|---:|---:|
| the record tool's help / report rules | 97 (80) | 196 | 82 turns in 60 verifiers |
| the opening | 95 (80) | 107 | — |
| the hash of a binary (`dev/stabilize-step hash`) | 89 (80) | 184 | 23 |
| the run's state (`dev/stabilize-record state`) | 87 (70) | 97 | — |
| the repro-block format (`pinning.md`) | 85 (75) | 183 | 75 turns in 50 |
| the rig's help / state list | 81 (77) | 148 | 52 turns in 43 |
| the exit rule's text | 76 (69) | 80 | 22 |
| host facts (`id`, `sw_vers`, `uname`, `git --version`, `sudo -n`, docker) | 74 (64) | 106 | — |
| the clone's state (status, HEAD, branch) | 67 (55) | 119 | — |
| the workflow doc | 36 (30) | 84 | — |
| the slug rule | 22 (17) | 51 | — |
| the harness's source | 8 (1; triage 5) | 24 | — |
| the door registries in `cli.rs` | 5 (1) | 12 | — |

Taken together: **turns whose every call only establishes one of the tooling facts above** — verifiers, first invocation: **283 of 2,095 turns (13.5%), in 60 of 61; 26.8 M cache-read in those turns (exact), 225 k output tokens (an upper bound — bias 1), 48.4 agent-minutes.** Both invocations: 371 turns in 79 of 80, 35.3 M, 298 k, 63.6. The seven reporters: 36 turns, 4.5 M, 29 k, 6.4. Before its first state or drive a verifier spends a median of 6.5 turns of a median 32.5 (400 k of 4,050 k output tokens, 24.7 M of 279.9 M cache-read, over the 80).

**States built** (counted over commands and driver scripts only, never over a report's prose). 74 of 80 verifiers and 78 of 89 driving-role agents call the rig; all 89 use `mktemp -d`; 22 also `git init` by hand (17 verifiers). Rig state named in a verifier's rig-bearing commands and scripts: `fresh` 49 · `refs-post-hoc` 41 · `bare` 33 (an upper bound — the word is also plain English) · `committed-singletons` 4 · `vendored` 4 · `migrated` 2 · `chatty-hooks` 0 · no git-state member at all. On a rig command line literally: `refs-post-hoc` 37 verifiers, `fresh` 25, `bare` 6; 28 pass the state through a variable. 77 of 80 verifiers build a rig for each of the two binaries. Plants, agents (verifiers): an out-of-band edit under a doc home 34 (30) · a symlink 32 (28) · a `chmod 000` 26 (23) · odd or non-UTF-8 file names 22 (20) · a planted `pre-commit` hook 22 (18) · a `git mv` of a managed doc 20 (19) · tampering under `.jigc/` 18 (16) · a linked worktree 14 (5) · `rm` of a managed file 9 (9) · a read-only directory 9 (7) · `core.hooksPath` 8 (8) · a submodule 7 (3) · a bare repository 2 (1). Around them: a `HOME` swap 54 (46), git identity masking 46 (41), a tree byte-snapshot before and after 45 (36).

Clustered by *the same rig states plus the same plants*: **62 distinct clusters for 80 verifiers; 52 of them hold one verifier.** The top 15:

| verifiers | rig states | plants |
|---:|---|---|
| 7 | `fresh` | an out-of-band edit under a doc home |
| 4 | `refs-post-hoc` | odd file names |
| 3 | `bare` + `fresh` + `refs-post-hoc` | odd file names, `git mv`, `.jigc/` tampering |
| 2 | `fresh` | a symlink |
| 2 | `bare` + `fresh` | a symlink |
| 2 | `bare` + `fresh` + `refs-post-hoc` | odd file names, an out-of-band edit, `.jigc/` tampering |
| 2 | `refs-post-hoc` | `chmod 000`, a symlink, a linked worktree |
| 2 | `refs-post-hoc` | `chmod 000`, a symlink |
| 2 | `refs-post-hoc` | a symlink, `git mv` |
| 2 | `refs-post-hoc` | an out-of-band edit |
| 1 each | five more, each its own | — |

A rig-bearing verifier turn takes a median of 11 seconds end to end (232 such turns, 69 agent-minutes in all, generation included). So whole states repeat little — 28 of 80 verifiers share a cluster with another — and are cheap to build; their ingredients repeat. The clustering sees the kind of a plant, not its operand: two verifiers in one cluster may have planted different files.

**Driver scripts.** 57 of 80 verifiers (40 of 61) write at least one: 195 script writes, 465,274 characters (377,510 in the first invocation). The recurring shape, read off the samples: make a rig for a named state with a named binary, run a list of commands as cells, keep each cell's stdout, stderr and exit status, take a tree manifest around it, do the same on the previous release, compare.

**The previous release.** Every one of the 80 verifiers' commands names its binary; 71 have at least one call classified (e), a median of 2 such calls; 206 (e) calls over 79 agents. Verbs in a command or script that also names the previous release (an upper bound for *run against it*), agents: `doc list` 24 · `doc show` 22 · `task finalize` 21 · `validate` 16 · `doc set-slot` 16 · `start` 15 · `doc set-field` 14 · `setup` 11 · `migrate` 10 · `doc author` 9 · `config set` 9 · `task list` 9 · `doc create` 8 · `task validate` 8 · `ingest` 8. Cost over the 80: 45 k tokens typed, 37 agent-minutes, 7.5 M cache-read in those turns.

**Builds and gates of an agent's own.** Verifiers and reporters: none. The advocate started `dev/gate` four times on its spike (one killed) and the independent drive twice, plus a release build and a test build: the fork's 138 minutes hold six gate runs.

**A scripted arm run by three agents.** The trial arm of the canary is a committed shell script run in the image and scored by `completions/trial-driver/run.py observe`. Rehearse, run and score were three agents: 94 turns, 102,391 output tokens, 9.5 M cache-read, 23.8 minutes — beside the review row's chain, so no wall time of this stage.

## 2. What each instrument checks, and what of it a test could hold

The exit rule (`DECISIONS.md` → *2026-10-04 — The exit rule, revised*): no loss and no wrong write, in a healthy repository used as documented · a working product (nothing that works on the previous release stops working; every refusal's route works as printed) · usable by agents · the planned migration will work. One instrument each: the review rows and the audit · the regression set · the blind trial arms · a port rehearsal.

How this section was read: the rows' registries from `per-axis-review-rc24/instrument/README.md`; the suites by grepping `crates/cli/tests/` and `tooling-tests/` (468 + 21 files) for each registry's name and reading the header of the nearest twelve. A suite named here iterates the registry; that it covers every cell the row drove was **not** checked.

**The first clause's matrix — does a door × state matrix exist that snapshots every byte before and after?**

- `read_verb_acts_nothing.rs` — the 12 `Read` leaves of `VERB_KINDS` × **one** revealing state, whole tree including `.jigc/` byte-identical afterwards. The only suite whose oracle is the whole tree over a registry of doors.
- `posture_door_axis.rs` — the 12 acting rows of `BEHALF_DOORS` (10 commit, 2 move) × the 17 members of `GitState::ALL` = 204 cells, 22 fixtures, ~24 s. Its oracle is the refusal's identity and *nothing concluded* (markers, HEAD, commit count over all refs) — not the tree's bytes.
- Per-door byte oracles exist, each with its own snapshot function (four suites define one; `tests/support/` has none): `uninstall_workbench_subject.rs` (29 tests), `copy_in_baseline.rs` (22), `record_door_baseline.rs` (10), `replacing_writers_never_follow.rs` (14), `mint_doors.rs`, `git_status_own_flags.rs`, `linked_worktree_doc_home.rs` (16) — the last five the rc.24 fix pass's.
- **Missing:** the 36 `Write` leaves crossed with anything under a tree-and-object-store oracle (*every byte gone is held by a git object; every committed path was asked for*). `support/leaf_argv.rs` holds one clap-satisfying argv per leaf, which resolves nothing; a runnable argv exists for the 12 acting rows only. Size: 36 leaves × 7 states (the six of `State::ALL` and `bare`) = 252 cells; × the nine ordinary configurations = 2,268; the 17 git states are crossed for 12 doors today, 36 × 17 = 612 if for all.
- **What such a matrix leaves out:** the planted state. Of the six tier-1 rows of the rc.24 review, five were a door at a state no registry holds — an untracked file at an install path on an unborn HEAD, an edit made after a task's first write with no baseline, a join suffix minted onto an id on disk, a dangling link at a minted home, files under `.jigc/logs` at uninstall. The canary's verifiers planted along the same axes (§1e).

| instrument or row | looks for | a deterministic test holds it | could be a table over existing registries | needs a model |
|---|---|---|---|---|
| row 1 — setup, hook, install, release | the install commit's paths and what stands there before; the hook; the package | `setup.rs`, `setup_install_pathspec_guard.rs`, `replacing_writers_never_follow.rs`, `uninstall_workbench_subject.rs`, `package_contents.rs` | install paths 10 × occupancy (absent, tracked, untracked, link, dangling link, directory) × HEAD (born, unborn, detached) ≈ 180; no registry of `setup.*` / `uninstall.*` codes (20 / 13 by grep) — the decided pilot | the release pipeline and the published page |
| row 2 — probe, invocation log | which doors probe; the log's rows and keys | `invocation_log.rs`, `commit_rejected_axis.rs`, `count_fences.rs` | `COMMITTING_DOORS` 11 × `ERROR_CODE_REGISTRY` 12; probe doors have no registry (3 call sites → 6 doors) | little |
| row 3 — store exit codes, reconciliation | exit flips; out-of-band edits detected, never overwritten | `exit_flip_count_record.rs`, `home_vacated.rs`, `orphaned_instance.rs`, `unclaimed_file_family.rs`, `copy_in_baseline.rs`, `record_door_baseline.rs` | `STORE_EXIT_FLIPS` 7 × `STORE_FAMILIES` 7; write doors × (baseline present, absent) × (edit before, after first write) | whether a refusal's wording is true |
| row 4 — pack load, freeze, migration | a frozen schema cannot change unversioned; migration | `freeze_enforcement.rs`, `migrate_route_family.rs`, `migrate_source_rules.rs` | `SchemaChangeKind::ALL` × loci = 54; manifests 6 + 13 — the row's 419 driven rows were this table by hand | little |
| row 5 — finalize, transaction | what a commit takes, what a rollback restores | `rollback_population_registry.rs`, `rejection_frame_outcome.rs`, `commit_seam_posture.rs`, `finalize_family_registry.rs`, `posture_door_axis.rs` | 14 doors × `CommitModel` 3 × `ManifestKind` 7; `ROLLBACK_POPULATIONS` 11 | the narration's truth |
| row 6 — write surface | create gates, minted homes, slugs | `write_miss_shape_axis.rs`, `changelog_write_touch.rs`, `mint_doors.rs`, `work_unit_id_axis.rs`, `home_shape_one_code.rs` | 8 `doc` write leaves, `MINT_DOORS` 6, `DOCTYPE_DOORS` 16, `SLUG_DOORS` 6 × home occupancy (6) — where three of the six tier-1 rows sat | no |
| row 7 — pinned read contracts | the JSON key sets | `format_json_success_axis.rs`, `text_json_parity_axis.rs`, `store_key_envelope_axis.rs` | `ENVELOPE_ARMS` 66 — held | no (four tier-3 rows, none higher) |
| row 8 — composed surfaces | what a workflow's text tells an agent to do | `compose_goldens.rs` (over `State::ALL`), `verb_routed_compose.rs` | 39 workflows × 15 suppressed doors: structure is held | **yes** — whether the composed prose is followable |
| row 9 — adopter docs and help | a sentence in a guide or a help text is true | `help_truth.rs` (25 tests), the README, install-line and older-guide supports | 48 `--help` leaves: presence and shape only | **yes** — sentence against behaviour |
| row 10 — co-author trailer | every commit jigc makes carries it once | `agent_co_author.rs` (8 tests) | 12 doors × the listed cells — the row's 237 driven rows | no |
| the audit of the changed paths | defects and class siblings in the diff | the gate | no | **yes** — it reads code for what no test names |
| trial arms, scripted | a walk's doors behave | 24 scripts under `completions/trial-driver/arms/walk/`, driven by `walk.py`, scored by `run.py observe`; the harness's own tests | they are tables already — and were run by three agents (§1e) | no |
| trial arms, blind | an agent with no instructions gets through | `run-session.sh`, `run.py observe` score a finished session | no | **yes**, by definition |
| regression set, part 1 | the previous release's tests on the candidate's binary | `dev/regression-set` (4,393 baseline tests; 22 differences, all listed) | — | no |
| regression set, part 2 | success paths in the ordinary configurations, both binaries | **not built** — 0 of 9 fixtures | 61 paths × 9 configurations × 2 binaries = 1,098 runs | no |
| every refusal's route works as printed | the route, followed literally | suites for 43 to 117 of ~175 codes; none for `setup.*`, one for `uninstall.*` | decided: the pilot of 39 sites; ~205 blocking sites with a route in all | for the 187 of 241 routes that are prose |
| port rehearsal | this repository migrates on a copy | **not built** | not established | not established |

The nine ordinary configurations of part 2, built by hand in the canary — agents (verifiers): a linked worktree 14 (5) · a moved repository 9 (7) · a separate git directory 8 (2) · a submodule 7 (3) · line-ending conversion 4 (2) · a worktree of a bare repository 3 (1) · a symlinked instruction file 3 (1) · hidden untracked files 2 (0) · an older git 0.

## 3. The preparation and the record

Order as run (`data/timeline.tsv`), with what keys each.

| step | min | keyed by | a second stage on the same commit |
|---|---:|---:|---|
| state reads, the attempt's marker | 0.5 | — | redone |
| first preflight (environment asserts) | 1.2 | — | redone |
| scope step, beside it | 10.2 | the round | reused inside a round |
| the candidate's binary | 28 s | label and attempt (`c1.a1`), not the commit | **rebuilt** (second invocation: 30 s, the same hash) |
| the previous release's binary | 26 s | its commit | reused under the same scratch root |
| the candidate's gate | 23.3 | label and attempt; one per round on record | reused inside a round |
| the regression set | 22.3 | item, label and attempt | reused inside a round |
| second preflight — the trial image | 24.7 | the image's tag is the commit | not established — the canary built it once |
| **before the first instrument** | **73.5** | | |
| the record's batch, typed | 13.4 | — | — |
| the record's gate | 22.6 | stage, round, attempt, number | runs on every record commit |
| commit, push, state | 1.4 | — | — |

- **A held job is found by its name under one scratch root** (`dev/stabilize-step` → `hold-start`: a name that exists is answered, never restarted). Only the previous release's name carries a commit. Nothing is keyed across scratch roots, so a new run on the same commit starts every job again.
- **In sequence though nothing in the data orders them:** the gate (23.3), the regression set (22.3) and the image (24.7) ran one after another — 70.3 of the 73.5 minutes. The workflow doc rules the regression set *after the candidate's gate and never beside it*; the image build, which runs in Docker on an archive of the commit, waits for both only because the second preflight comes after the held checks. The scope step did run beside the first three.
- **The regression set**, from its own verdict file (`run/r1/checks/regression-set.a1.json`): unpack and build the previous release 54 s, baseline 599 s, build the candidate 33 s, candidate run 631 s. **653 of 1,319 seconds depend on the previous release's commit alone.** The candidate is built a second time here (32 s) and a third time, in release, in the image.
- **The trial image**, from its build log as the preflight read it: the compile step ran 1,284.6 s. Toolchain sync 0–225.6 s (the base image is not at the pinned toolchain's components) · crates.io index and downloads 225.6–1,256 s, with network timeouts · compiling every dependency and both crates 1,256–1,284.6 s (the two jigc crates from 1,265.6). **1,256 of 1,285 seconds are the same for every candidate with the same toolchain file and lockfile**; the Dockerfile copies the source before the build, so no layer holds them. The runtime stage (base, packages, the agent CLI at a pinned version, config) is candidate-independent too; whether its layers came from cache in this build was not read. Verification took 20 s.
- **The record's gate** runs the whole gate on a commit that adds the run's files under `completions/`. 151 of the 489 suite files name a path under `completions/`; which of them read a run's directory was not established.

## 4. Candidates to build once — ranked by output tokens the canary's stage would not have spent, then by minutes

Every figure is the first invocation's unless marked; "exact" means read from `usage` blocks or a tool's own timing.

| # | what | output tokens | cache-read | minutes | rests on |
|---:|---|---|---|---|---|
| 1 | **A kit of the facts every verifier and reporter looks up** — the two binaries with their hashes, the run's and the clone's state, the record tool's report rules, the rig's states and `--binary`, the repro-block format, the clauses — printed by one call or handed in the prompt | ≤ 254 k (verifiers 225 k, reporters 29 k) — upper bound, bias 1 | 31.3 M in those turns, exact | 54.8 agent-min | 319 turns that do nothing else (283 of the verifiers' 2,095); the content still enters the context once |
| 2 | **Triage's entries typed once** — the report rendered from the return, or the return read from the report | ~129 k (336,959 characters of reports incl. two superseded; at 3.5 chars/token 96 k) | not measurable | not separable (h is 36 of triage's 103 agent-min) | character counts of the Write and return inputs |
| 3 | **The record's batch composed by a tool** from what the harness already holds | 118 k, exact | 0.3 M | **13.4 wall**, exact | the file is a verbatim substring of the executor's prompt |
| 4 | **A driver kit** — twin rigs for a named state, cells with captured streams and exit status, a tree manifest around each, the same on the previous release, the comparison | ≤ 145 k typed (377,510 characters of scripts by 40 of 61) | 7.2 M in the script-writing turns | 37 agent-min in those turns | script characters; how much of a script is the finding's own was not separated; 36 of 80 verifiers take a tree manifest somewhere |
| 5 | **Evidence by reference in a report** — fenced output pasted by the record tool from a file | ≤ 114 k (297,892 fenced characters in 61 final reports) | follows | not measurable | fenced share 20%; not all fenced text is a file's content |
| 6 | **The scripted arm as a held check** — run and scored by the tool | 102 k, exact | 9.5 M, exact | 23.8 agent-min, 0 wall here | three agents' totals; they also probed beyond the script |
| 7 | **A base layer for the trial image** — toolchain and fetched dependencies, keyed by the toolchain file and the lockfile | ~12 k (the waiting preflight) | 1.2 M | **~21 wall** of 24.7 | the build log's timestamps; that day's network was slow |
| 8 | **The regression set's baseline kept per previous release** | 0 | 0 | **10.9 wall** of 22.3 | the tool's own `seconds` |
| 9 | **A stage-agent instruction file in place of the project one** | 0 | 36–44 M (of the prefix's exact 92.9 M) | not measurable | 50,027 characters × 2,881 turns; whether an agent can be launched without the project file is not established |
| 10 | **The finding handed as its own excerpt**, not as a pointer to the parent's whole report | not measurable | ≤ 17.4 M (model) | not measurable | 102 whole-report reads, 1.43 MB, by 61 verifiers; the excerpt's size was not measured |
| 11 | **The ordinary-configuration fixtures** (part 2 of the regression set, already owed) | not measurable | not measurable | not measurable | 2 to 14 agents built each by hand (§2) |
| 12 | **The door × state byte matrix for the write leaves** (252 cells on base states) | not measurable from this data | — | — | no canary finding was at a base state; the tier-1 history is at planted states |

Counted and found **not** worth a build: a cache of built states (62 clusters for 80 verifiers, the largest of 7; a rig turn takes 11 s); a pack of product source (129 of 306 files read by one agent, the top file in 76 different slices); the hash calls (23 turns, 11 k output tokens).

Counted, large, and not a build: 31 of 80 verifiers re-wrote their whole report (602,688 superseded characters in the first invocation, ~231 k output tokens) — a matter of how the report is revised; 47% of a verifier's output is reasoning; the fork's six gate runs.

## Not established

- How much of a fact-only turn's output is reasoning about the finding rather than the lookup — the 254 k of candidate 1 is a ceiling.
- Whether fewer or shorter turns shorten the stage's wall time: agents ran up to eight at once, so agent-minutes do not convert.
- Tokens of a call's input: estimated from characters at 2.61 per token; tool-result tokens at 3.3 bytes per token, 16% short on the median.
- Whether a stage agent can be started without the project instruction file, and what it would then lack.
- Whether the image's dependency download is as slow on another day: the 1,031 seconds include network timeouts.
- Whether the gate, the regression set and the image build can run beside each other on one machine without lengthening each; whether the record's gate may be answered from the candidate's.
- Whether the suites named in §2 cover the cells their rows drove; the review's 132 to 419 driven rows per row were not matched against test cases.
- What the real run's instruments will cost: the canary had one door, one review row and one scripted arm; no blind arm, no audit and no port rehearsal ran.
- The classification's error: 8 of 100 sampled calls wrong or partly wrong; the roles other than verifier were sampled (about 95 calls) but not scored.
