# The plan before the first stabilization run opens — the builders' spec

**What this file is.** The builders' spec of everything owed before the first stabilization run opens: the plan's fifth version of 2026-10-29, made self-contained. The parts that version took over from the fourth ("as in v4") are written out here; the parts the fourth took over from earlier versions and from the [reshape plan](../reshape/plan.md) are written out too, or point at the reshape plan's own section where it is the spec. The corrections the check of the fifth version made are applied in place, and the version's changelogs and review tables are at the end, as history. Task codes stay here; the readable plan is the roadmap entry *Before the first stabilization run opens* in [`implementation/roadmap.md`](../../../../../implementation/roadmap.md), and it names no code. Where the work stands is [progress.md](progress.md); the plan's gate answers are [planning-record.md](planning-record.md).

The plan is read-only planning, based on `fix/rc24-tier1` at `7c34a222` (clean tree; CI run `40767224323` green). Every count was taken at `7c34a222` unless it says otherwise.

## The roadmap entry's steps, and the tasks behind each

This table is the one place the readable entry and this spec meet; a step of the entry covers exactly the tasks in its row. A lane's tasks inside one step run in the lane's own order; two lanes in one step set a pace, not a dependency.

| Step | Tasks |
|---|---|
| 1 — The records, and four branches | D0 · D0-def · M-defs · PC · the fork of U, K, P and T |
| 2 — Foundations, first half | P0 · P1 · M-P0 · T0 · RB1 · U0 · M-U0 · U5 · K7 · H2 asked |
| 3 — Foundations, second half | P-F0 · T-NR's build and its review · M-TNR and loop → T · U1 · U2 · U3 · U4 · M-U and loop → T · J2-0's first listing · J2a's frame wave (reviews) |
| 4 — The probe, and the act check counted | T-NR's run and its record · P-F1 commit 1 · H0 (conditional) · H1 · T1 · J2a's frame-wave fixes (in P and U) · U2's tell |
| 5 — The act check applied | P-F1 commit 2 · loop → P · T2 · J2a's second wave (reviews) |
| 6 | C1 · C2 (then the re-price) · C3 · O-def · J2a's second-wave fixes, by owner |
| 7 | C4 · C5 · C6 · T8 · M-U again with J2a's fixes in U, then loop → T |
| 8 | C7 · C8 · H-r1 · M3 · K7b |
| 9 | C9 · C10 · C11 · K6 · T9 |
| 10 | C12 · C13 · T11 |
| 11 | C14 · C15 · H-r2 · T12 · O5 · M-T |
| 12 | C16a · C16b · T10a · J2-0's partition · J2b (reviews) |
| 13 | C17 · C18 · H-r3 · J2b's fixes |
| 14 — The catalog closed | P-NC2 · P-Z · M-P · J2b's fixes and J2r |
| 15 — The join, and the end of the build | J1 · T10b · G1 · Phase 3's end · H3 |
| 16 — The canary and the opening | Phase 4: the canary (if ruled) · J3 · the opening · H4 |

H-cycle can come up at any step. Tell-NR was given on 2026-10-29, before any of this (§3).

## 0. The rulings and the facts the plan is cut on

### 0.1 The human's rulings (D0 records every one)

**The rulings of 2026-10-27 and 2026-10-28, as v2 folded them in:**

| Ruling | What the plan does with it |
|---|---|
| (a) The catalog first: finished now, its fence tightened, its frame extended | Lane P is the critical path. P1 tightens the fence. P-F0 and P-F1 extend the frame. `wf-route-catalog-rest` is due before the opening. Red cases go to the opening ledger by key, never fixed in P. |
| (b) The hunter stays on top of the catalog, shape A | P0 builds the projection. T1 and T2: the record script holds the rows and computes the count. Each plantable cause the hunter finds becomes a catalog case: the `build-fixer` definition says so, and the `fix` half's repair holds it (new row `wf-hunt-cause-to-case`). |
| (c) The regression base: every earlier candidate of the run, newest first | RB1 now (the field `earlier`). The rest goes under `wf-regression-base`, due with the `fix` half's repair. |
| (d) The link-on-the-way fix (B) | Done at `7c34a222`. Not a task. |
| (e) A fourth review cycle was allowed | Recorded as a data point for the workflow's bound of three, put to the human at the first stop. It also sets the plan's conditional stop (§3). |

**F-A → B (2026-10-28). Ruling B's two open siblings go to the opening ledger and are fixed by the run.**
- The two keys:
  - `r1-config-set-writes-through-a-link-at-the-manifests-own-place` (`byte-table-findings.md` §30);
  - `r1-uninstall-half-tears-down-over-a-hook-link-it-cannot-read` (`route-catalog-findings.md` §12).
- *Why:* the workflow exists so that known findings are fixed inside the run. Fixing everything before the opening is the ad-hoc loop it replaced.
- *Consequence:* only tools that a task before the opening, or one of the run's instruments, needs are kept. The rest move to `tooling.md` → Candidates, each with its trigger.

**F-B → A (2026-10-28). J2 reviews everything the `test` stage consists of, over `cbb3d736..HEAD`, with nothing left out.**
- What has its own review counts as reviewed and is not read again.
- *"Reviewed" is decided by commit range* (§2, J2-0): a file's lines count as reviewed only where a recorded review's range covers the commit that wrote them.
- So the following go into J2, under this ruling, with no new question:
  - the byte table's frame and its first families (B0–B5);
  - the route catalog's code;
  - the CI fixes `936c6d54` and `72ca156a`.

**F-C → A (2026-10-28). A route works as printed only if its text names every act, and every operand, that following it needs.**
- A route that leaves one to be inferred is a finding of its own class, which the run fixes in the product's text.
- The catalog checks acts, not wording. This discharges the question carried in `wf-route-catalog-rest`.
- **Reading (an engineering reading of the ruling; told to the human with H1):**
  - the frame's **act-verb table**, an exhaustive `match` over `enum Act`, is the vocabulary for every act;
  - a case checks its act's verb plus its operand, by span, and nothing else of the sentence;
  - a link case also checks that its act is the one `Link::at` computes.
- P0 and P-F1 count the routes affected; far beyond expectation is a human stop (H0) before the C-tasks.

**T-NR → B (2026-10-29). The build session runs the single T-NR runtime probe**, as a recorded exception to the one-run-one-session rule.
- The exception is scoped to probes of one harness mechanism with **one subject agent, with the harness's counted step agents**. A stage, a round or a canary still runs in the human's own session.
- **What "one agent" means in practice.** A probe always launches `build-git` step agents for its acts (`probeAct` → `gitStep` → `agentR`, `stabilize.js:2745-2800`). So "exactly one agent" is built as:
  - **one subject agent**;
  - plus a stated, counted set of **three step agents** (begin, the record writer, the verdict);
  - **every launch `once`**, with no retry.

  The human was told this reading on 2026-10-29, before T-NR's build, together with the guard's binding while the probe runs (Tell-NR, §3). If the human objects, T-NR is rebuilt before any of it lands.
- The conditions are in the task (§2, T-NR):
  1. the timing (the one wording of §1);
  2. the fixed, counted agent set, with no stage, no push and no retry;
  3. a planted text that is harmless if obeyed;
  4. D0 records the exception;
  5. "no push" and "no commit" are held by a mechanism: the write guard binds while a probe is active.

### 0.2 Facts corrected or added

- **The `CLAUDE.md` runner sentence changed in `26474b80`**, not in `63ce9ca5` (`git log -- CLAUDE.md`).
- **J2's base is `cbb3d736`**, the tree the last re-review of the `test` half read (`603018ac^..cbb3d736`, `DECISIONS.md` → *2026-10-07 — The `test` half … re-reviewed*).
  - Over that range: 247 commits and 413 files.
  - Over `dev/ .claude/ tooling-tests/ implementation/stabilization-workflow.md completions/trial-harness/ .github/`: 60 files, +78,721 / −3,618 lines.
  - The source files among them: harness source +5,537, record script +4,795, canary tool +2,581, step tool +2,441, regression set +813, probe tool +793, guard and fence and guard probe +546, generator +234, hygiene scan +78, settings +21, doc +438, definitions about +250.
  - About 60 k of the lines are suites.
- **What no recorded review covers:**
  - the byte table from `fc2b13b6` through `56839f95` (12 commits, 2026-10-12 to 2026-10-15): +12,265 lines under `crates/cli/tests/` (the frame, the sandbox's reader and runner, the acceptance cells, the pilot, task and install families). The first review on record, of B6, reads `56839f95..f4c12fa3`;
  - the route catalog's code (no heading of P1–P5 or of the 2026-10-12 families records a code review; only the link fix's reviews of 2026-10-28 read the commits that added `ReplaceWithACopy` and `MoveDirectoryIn`);
  - the CI fixes `936c6d54` and `72ca156a`.
- **CI's legs against their bounds** on run `40767224323`:

  | Leg | Took | Bound | Share |
  |---|---|---|---|
  | `g_tooling, rest` | 629 s | 11 min | 95 % |
  | `g_tooling, simulation` | 642 s | 12 min | 89 % |
  | `g_migration` | 504 s | 10 min | 84 % |
  | `g_tooling, record` | 518 s | 14 min | 62 % |

- **Routes that print no operand.** In the pilot, 32 of the 183 cases that hold did so with an operand the case had to supply (*holds, operand implied*), out of 74 cases whose route printed no operand (`route-catalog-pilot.md` §6, after its correction of 2026-10-11). That is H0's expectation.
- **The act vocabulary in the product:** only `link_way::WayOut` (`link_way.rs:66-88`). Its route clause is rendered by a private `fn clause` (`:109`). The one public question is `Link::at(root, at, stands)` (`:99`, its `way_out` field public).
- **`enum Act` has 27 variants** (`support/route_case.rs:467-604`). By use count in the catalog: `Rerun` 45, `Remove` 32, `Commit` 28, `MoveOut` 24, `Cd` 19, `MakeWritable` 18, `MoveAway` 17, `RunListed` 17, `MoveLinkOut` 16, `PointAt*` 15, `PutTheFileItself` 13, `MakeReadable` 11, `Revert` 11, `Mend` 6, `Unstage` 5, `RerunWith` 5, `RerunInTheRepository` 4, and the other `Run*` forms and link acts.
  - The `Rerun*` acts are *"no better than operand implied"* by construction (`route_case.rs:575`, `:590`).
- **Where `PutTheFileItself`'s routes are printed:** `crates/cli/src/milestone.rs:1629`, `crates/cli/src/setup.rs:3170` (v4's `:2870` is a doc comment) and `crates/engine/src/finalize.rs:1537`. There is no `finalize.rs` in `crates/cli/src`, but the engine's prints the route; a reviewer confirmed it. None of the three is a `WayOut` route.
- **Two classification facts:**
  - `HoldsOperandImplied` is green at `route_case.rs:916`. `differences()` (`:1066`) names every red case that no entry lists.
  - The arm at `route_catalog.rs:3987` asserts that a classification is red exactly when its words do not start with *"followed, holds"*.
- **The frame's rule on wording** is `pinning.md:124` (*"What it does not pin … a route's wording (no case asserts one)"*) and the reshape plan's *"Must not: assert a route's wording"* (`plan.md:271`). P-F1 amends both.
- **The pilot's per-site wall time is measured, not derived.** P3: 10,773 s over 10 sites, 18 minutes a site. P4: 12,065 s over 18 sites, 11.2 minutes a site. Together 13.6 minutes a site (`route-catalog-pilot.md:68-81`, `:489`). That equals the 2026-10-12 families: `ee4db8f1` at 01:10 to `75754e8f` at 08:54, 464 minutes over 34 sites.
- **The simulation has 92 `#[test]` arms** at HEAD. That is T11's upper bound.
- **The runtime probe's mechanics** (`.claude/workflows/stabilize.js`):
  - a probe launches `build-git` agents for its acts (`probeAct` → `gitStep` → `agentR`, `:2745-2800`);
  - `agentR` retries up to twice unless called `once` (`TRANSIENT_RETRIES = 2`, `:2103`);
  - `gitStep` (`:2138`) does not pass `once`;
  - `probeFault` refuses any key except `probe`, `scratch` and `seconds` (`:389-392`). That is the choice of 2026-10-07 (`DECISIONS.md`, near line 8302): *"a probe takes `scratch` and nothing else … pinned in three places"*;
  - the write guard binds only inside an opened run (`dev/stabilize-guard:18-26`);
  - a probe's step agents run `dev/stabilize-probe` from the session's checkout, which is the loop checkout.
- **Rows due before the opening, or before a stage runs with real agents, that v2 left out:** `wf-neutralized-return` (`decisions-pending.md:123`); `wf-route-cases-same-commit` (`:151`); row `:101` (the gate's stale "(probe · fmt · clippy · build · test)" in 8 definitions); `wf-o2` (`:114`).
- **Two rows to annotate:**
  - `wf-o5` (`:110`) is superseded by the ruling of 2026-10-16 that the run and every canary act run in `bypassPermissions` (`stabilization-workflow.md:679`).
  - `wf-return-size` (`:169`): its opening condition will fire; `K2`'s digest covers it for triage.
- **The canary and the opening are run in the human's own session** (one run, one session). The single exception is T-NR's probe.

## 1. Lanes, branches, phases, the gate queue

| Lane | Branch / checkout | Owns |
|---|---|---|
| **Phase 0 / join** | the loop checkout `fix/rc24-tier1` | `DECISIONS.md` and `decisions-pending.md` entries for rulings (a ruling given during P is recorded by the P commit that follows it); `stabilization-workflow.md`, including T-NR's declared bound if that is its outcome; `tooling.md` → Candidates; the build's README; G1's `.gitleaks.toml`; the main session's own edits to the definitions (M-defs) and to `.claude/settings.json` |
| **U** | `work/rc24-tools` | `ci.yml` and `ci_matrix_fence.rs` (shared, by declared row), `dev/runner-faithful`, the new `dev/ci-runs`, `dev/pending-due` and `dev/review-coverage` with the committed registry of review ranges, each with its suite and its `tooling.md` row; J2a's fixes to the regression set, the hygiene scan, the byte table (`crates/cli/tests/byte_table*`, `support/byte_cell.rs`) and, declared, `crates/cli/tests/setup_install_pathspec_guard.rs` (the CI fix `936c6d54`) and `tooling-tests/rig_processes.rs` (touched by `72ca156a`) |
| **K** | `fix/rc24-trial-image-layer` | `completions/trial-harness/` (K7) |
| **P** | `fix/rc24-route-catalog-rest` | `crates/cli/tests/route_catalog*`, `support/route_case.rs`, `support/refusal_sites.rs`, `refusal_site_list.rs`, the projection, `pinning.md` §6's body, the catalog's records (`route-catalog-findings.md`, `route-catalog-pilot.md`); J2a's fixes to the catalog's frame and cases |
| **T** | `fix/rc24-route-hunter` | the record script, the step tool and its suite `tooling-tests/dev_stabilize_step.rs`, the probe tool, the harness (source and generated file), the simulation and its runtime fixture, the harness fence, the canary tool; for this build, **the write guard** (`dev/stabilize-guard`, `dev/stabilize-fence`, `dev/stabilize-guard-probe.js`, `tooling-tests/dev_stabilize_guard.rs`), both T-NR's change and J2a's guard fixes (declared: one owner for the guard, so the two changes do not collide); the main session's edits to the definitions this lane's harness is held to, made in this worktree in lane order |

**Rules:**
- **The main session only orchestrates.** Its own edits are limited to `.claude/agents/`, `CLAUDE.md` and `.claude/settings.json`, each behind its own gate. A repeated sentence is changed in every copy or in none.
- **Builders and fixers:** Opus `build-executor` and `build-fixer`, each with a self-check.
- **Reviews:** every task is followed by a read-only review and its fixers, at most three cycles; a fourth is the human's (§3). A review reads only commits, so it may overlap the next task in its lane; its fixes are that lane's next writer.
- **Every review registers what it read.** The fixer's or builder's record commit that closes a review appends that review's commit range and paths to the registry `dev/review-coverage` reads. So every commit this plan makes is covered by the review of its task, and J2-0 can tell.
- **Git steps** (worktrees, merges with `--no-ff`, pushes by name) are run by Sonnet `build-git`. **Nothing touches `main`.**
- **The gate queue:**
  - a builder or fixer stops before its gate and returns `ready-for-gate`;
  - the main session runs one queue and tells one agent at a time to run the full `dev/gate` (`--private-target` in a worktree), in the background, unpiped;
  - the queue also takes `dev/runner-faithful` runs, docker builds, and any suite run a builder makes that takes over about 60 seconds (the catalog suite, the simulation, the record script's suite, the step tool's suite): such a run is requested and started by the main session like a gate;
  - a red gate is run once more before it is judged, and both results are recorded;
  - priority is P > T > K > U (ruling a).
- **The T-NR run is made while no other agent of the session is in flight.** The guard is one hook for the session, so while a probe is active it refuses commits and pushes from every subagent, whatever lane launched it. The main session lets in-flight tasks reach their gate stop, runs the probe (minutes), then resumes the lanes.
- **Breaks.** The build is run in steps, with breaks between them; nothing in it is a global barrier, so a break is made as follows.
  - *A planned break:* the human says they want to stop. The main session starts nothing new and lets the tasks in flight finish; each ends pushed, after at most its review and fix rounds. Then it tells the human that nothing is running. Worst case, the drain takes one task plus its review cycles — possibly hours. A step's end is a natural point to ask for one.
  - *An unplanned stop* (a crash, a restart): every worktree and every brief lives in a persistent place, never in a scratch folder — the worktrees are siblings of the main checkout, and the briefs are in the repository. An unplanned stop then costs only the in-flight tasks' uncommitted work: they are redone from their last pushed commit, and `progress.md` says where to resume.
  - *The probe exception:* no break while the T-NR probe runs (minutes). If it is cut, its marker stays, and the guard refuses every command except the probe's own — so no gate and no commit — until the main session removes the marker.
- **Pushes:** every task pushes its branch at its end. After each push, the main session reads `dev/ci-runs` (from U3 on) and applies the 80 % rule: a leg past 80 % of its bound is re-bounded as that lane's next task.
- **Declared overlaps:**
  - `DECISIONS.md`: resolved by `dev/merge-logs`;
  - `tooling.md`, `tooling-tests/groups/g_tooling.rs`, `ci.yml`, `ci_matrix_fence.rs`, the review-range registry: different rows, resolved by hand;
  - `pinning.md` §6: its heading is frozen;
  - the generated harness: resolve the source, then regenerate;
  - the write guard: T only, for this build.

**Phases.** A phase is a lane's own sequence, not a global barrier. A task waits only for the dependencies it names.
- **Phase 0:** D0, D0-def, M-defs, PC.
- **Phase 1, foundations:**
  - U: U0, then M-U0; U5; U1–U4; then M-U;
  - K7;
  - P: P0, P1, P-F0, P-F1 commit 1. H0 is checked at P0 and at P-F1 commit 1. H1 is asked after commit 1's review; nothing in P waits for the answer. P-F1 commit 2 waits for J2a's frame wave and its fixes;
  - T: T0, RB1, T-NR's build, its review, then M-TNR (T → loop);
  - J2a's frame wave, and its second wave from J2-0's first listing.
- **Phase 2, the catalog and the hunter:**
  - **T-NR's run and its record.** In the one wording used everywhere: *after T-NR's build is reviewed and merged T → loop, and after D0 has recorded the exception; before T1, T10a and J2b.* The run is made from the loop checkout;
  - P: C1–C8 (2a), C9–C15 (2b), C16a–C18 (2c), each sub-phase ending at an H-r batch;
  - T: T1 (after a loop → T taken after both M-P0 and M-TNR), T2, O-def, T8, M3, K7b, K6, T9, T11, T12, O5, then M-T;
  - J2a's second wave and its fixes;
  - then T10a, and J2b with J2r.
- **Phase 3, close and join:** P-NC2, P-Z, M-P, J1, T10b, G1 (any time after H2), `dev/clean-litter`, H3.
- **Phase 4, in the human's own session:** the canary (if ruled), with J3 back in the build session for anything it changes; then the opening.

**Merges** (all `--no-ff`, by `build-git`):
1. **Fork** U, K, P and T from Phase 0's tip.
2. **M-U0:** U → loop, after U0. Then **loop → P**, before C1.
3. **M-P0:** P → loop, after P1.
4. **M-TNR:** T → loop, after T-NR's build and its review, before the run. Then **loop → T**, which also brings M-P0's projection, before T1.
5. **M-U:** U → loop, after U4, and again after J2a's fixes in U. Each M-U that carries J2a's fixes is followed by loop → T, so T's simulation and fences meet those changes in a gate of their own, not first in M-T.
6. **M3:** K → T, before K7b.
7. **M-T:** T → loop, after O5.
8. **M-P:** P → loop, after P-Z.

## 2. Tasks

The **gates** line of each task counts its build commit(s), plus the fix gates its review is expected to need.

### Phase 0

**D0 — The records.** Builder; a document task, one or two commits. The brief carries every ruling word for word.
- **`DECISIONS.md`:**
  - the rulings (a)–(e) of 2026-10-27 and 2026-10-28;
  - F-A → B, F-B → A, F-C → A, with F-C's reading (the act-verb table, the verb plus its operand by span, `Link::at` for link cases) marked as the orchestrator's engineering reading, which H1 tells the human;
  - T-NR → B (2026-10-29): the build session runs the one T-NR probe, as an exception to the one-run-one-session rule, scoped to probes of one harness mechanism with *"one subject agent, with the harness's counted step agents"*. A stage, a round or a canary still runs in the human's own session. The conditions are quoted (§0.1);
  - the facts the human is told: (b)'s last clause and (c) do nothing until the `fix` half's repair;
  - (e) as a fact; the "data point at the first stop" as the orchestrator's note;
  - the bracketed H1 correction on the 2026-10-27 *not comparable* entry;
  - N3 (part 4 ran); O6 closed; forks 1, 3, 5 and 7 settled, 6 moot.
- **`decisions-pending.md`:**
  - `wf-route-catalog-rest`: due before the opening; its question ruled (F-C → A);
  - `wf-route-walk`: annotated with (b), shape A;
  - new `wf-regression-base` and `wf-hunt-cause-to-case` (with the round-2 re-cut of parts), both due with the `fix` half's repair;
  - new `wf-hunt-count-view`: due at the first stop;
  - `wf-test-review-real-run`: annotated with F-B → A and with coverage by commit range;
  - `wf-whole-canary-after-parts`: annotated;
  - the first-stop row: annotated with (e), as the orchestrator's note;
  - `wf-o5` struck; `wf-return-size` annotated;
  - `wf-neutralized-return`: annotated with the ruling B and T-NR.
- **Docs:** `stabilization-workflow.md` lines 140, 275 and 706; the repeated rule of `wf-route-cases-same-commit` in `dev-workflow.md` and `increment-workflow.md`.
- **Keys:** N6's two leads, in `route-catalog-findings.md` §12.
- **`tooling.md` → Candidates:**

  | Candidate | Triage |
  |---|---|
  | The rig's link plant, with loops and links through a file | Trigger: the first stop, beside `wf-pre-driver-kit`, or the run's first fix of a link key |
  | The byte-table record tool (the print is committed: F3 A is buildable) | Trigger: the first fix that moves a byte-table cell |
  | The raw-write census | Trigger: the run's fix of `r1-config-set-writes-through-a-link-at-the-manifests-own-place` |
  | The runner-faithful docker arm (red at `0bd8a5b5` and `22f06d31`, green at `63ce9ca5`) | Trigger: the next change to the image's posture |
  | `n4-measure.py`, `enclosing.py`, `callcount.py`, `link-cells-repro.sh`, and the `readforms-all`, `jsonreject` and `dryforecast` modes | Each with its question, and built or declined |
  | The act-use counter, widened to every one of `enum Act`'s 27 variants | Built: P0 prints it per family |
  | The `MINTED_AT` counter | Built: P0 |
  | CI's share of bound and git version | Built: U3 |
  | The pending-row trigger scan | Built: U4 |
  | The unreviewed-range lister | Built: U5 |
  | The review-range grep over the byte table's review reports (B6a, B6b, B7a, B7b, B8, B9, B10) | Used once: the input to U5's backfill. The main session runs it over its own copies and hands the ranges over, since the reports are not in the repository. Declined as a `dev/` tool: the registry replaces it from U5 on. |
  | v1's review: the scheduled-to-sites join | Declined: the projection answers it |
  | v1's review: gitleaks over a PR range | Declined: `vet --range` and CI's `hygiene` job already do it |

- **Done when:** `doc_link_fence` and `tooling_index` are green, and the full gate is green.
- **Gates:** 1–2 + 1.

**D0-def — Main-session edit to `stabilize-opening.md`.** The fact sheet's section 0 (what is owed before the opening) gains:
- the tasks held back in build plans, each with its plan, its id and its trigger;
- the rows of `decisions-pending.md` due before the opening. `dev/pending-due` (U4) gives a floor, never the list: the gatherer reads every row the tool prints as unmatched as well, and judges each.
- **Gates:** 1.

**M-defs — Main-session edits to repeated sentences**, each behind its own gate:
1. Row 101 in all 8 definitions.
2. `wf-o2`, in `milestone-code-reviewer.md`.
3. `wf-route-cases-same-commit`, in `build-executor.md` and `increment-validator.md`.

Before a sentence is removed, the fences are searched for it.
- **Gates:** 3.

**PC — Count what the dynamic sites mint.** A read-only agent; no gate. For each of the 17 dynamic sites left after B1: every code it mints, with its source line. The count is an agent's reading; S2's arm and P-Z's arms make it held. The price is re-checked on it.

### Lane U

**U0 — CI's legs re-bounded.**
- **What:** `g_migration` split into parts (`route_catalog` · `byte_table` · the rest); `g_tooling`'s `rest` and `simulation` parts measured again and re-bounded or split. Each part's bound is max(10, 2 × its four-CPU proxy), measured in `dev/runner-faithful`.
- **Red first:** the fence's arms.
- **Done when:** a pushed CI run shows every leg under 80 % of its bound.
- **Gates:** 1 + 1.
- **Then:** M-U0.

**U5 — `dev/review-coverage`, J2-0's instrument** (second in U's order, before U1–U4).
- **The registry:** a committed list of recorded reviews, each with its range (`A..B`, or `A^..B`), the paths it read, and the `DECISIONS.md` heading that records it.
- **The tool:** given a range and paths, it lists, per file, the commits of the range that touched it and that no registered review covers, with lines added. It does this by commit membership: a commit is covered when it is in a registered range whose paths include the file.
- **Merge commits:**
  - a merge whose tree equals the automatic re-merge of its parents (`git merge-tree --write-tree <p1> <p2>`) adds nothing of its own, and is covered through its parents;
  - a merge whose tree differs (a conflict resolved, a generated file regenerated, `DECISIONS.md` put in order) is listed uncovered, by the files where it differs, unless a registered review covers it.
- **Backfill:** reviews that close before the registry reaches their branch (in P, T and K, until loop → that lane) are backfilled by the main session at M-U, or at the first merge into the loop after it, in one commit that lists each row added. Its first rows:
  - the 2026-10-07 re-review (`603018ac^..cbb3d736`);
  - B6–B10, their ranges from the review-range grep (for instance B6's `56839f95..f4c12fa3`), each citing the `DECISIONS.md` entry of the fixes that followed its review;
  - the link fix's four reviews (2026-10-28, including `22f06d31` and `63ce9ca5`);
  - every review of this plan closed by then.
- **Suite:** a fixture repository with overlapping, path-scoped and absent reviews, a clean merge (covered) and a merge with a resolved conflict (uncovered).
- **Gates:** 1 + 1.

**U1 — `dev/runner-faithful` declares its posture in one place.** `--posture` prints what the image asserts; the build-time assertion reads the same declaration.
- **Arms:** `--posture` prints the posture; the assertion is the declaration's; a usage error exits 2.
- **Writes also:** its `tooling.md` row.
- **Gates:** 1 + 1.

**U2 — The CI check for runner drift** (fork 7 A). One step prints `git --version` and `git config --system --list --show-origin`, and holds the config to `--posture`. The version is printed, not held.
- **Told** with the next stop: it turns CI red when GitHub changes the runner's git config.
- **Gates:** 1 + 1.

**U3 — `dev/ci-runs`.** Per job: conclusion, seconds, the bound from `ci.yml`, the share of the bound, the image, and the git version. `--compare A B`.
- **Suite:** a stub `gh` serving fixtures. The tool never prints an environment value.
- **Gates:** 1 + 1.

**U4 — `dev/pending-due`, a floor.**
- **What:** it lists each unstruck row of `decisions-pending.md` whose trigger (including a bracketed re-trigger) names the opening, round 1, `test`, a stage with real agents, the canary, the first run, or a join. It also prints every other unstruck row with its trigger text, as *unmatched*, so a judgment covers every row. Its header and its `tooling.md` row say the matched list is a floor.
- **Suite:** a fixture of rows: struck, re-triggered, conditional, a trigger in prose, *before round 1's `test` stage*, *before the canary*.
- **Gates:** 1 + 1.
- **Then:** M-U, and loop → T.

### Lane K

**K7 — The trial image's base layer**, as in the [reshape plan](../reshape/plan.md) §5 (candidate 7): a kept base layer for one toolchain file and one lockfile, which removes about 21 of 24.7 minutes of wall time for every candidate with a trial arm after the first. Red first in the harness's own checks: a second candidate with the same lockfile builds on the kept layer; a changed lockfile or toolchain file builds a new layer; the seven checks of the image and the check of a pair still hold, and the binary in the image is still the commit's, by hash. Its two docker builds go through the queue.
- **Gates:** 1 + 1.
- **Then:** M3, K → T.

### Lane P — serial (F1 A)

The lane writes no product source. A red case is listed with a key and is never fixed here.

**P0 — The projection.**
- **What:** one generated, committed file, held equal by the suite. It holds:
  - a vocabulary header (`Classification` and `Why`'s four words);
  - per family: its doors, its parts (keyed by file plus first site id), and every site, with claimed, `MINTED_AT` and `IN_PART` sites in the family's rows;
  - per site, its answer: each case with its cause; or the reason (word, text, `ruled`); or `scheduled`;
  - `without_case` and `not_followed`, with their `Why`;
  - per dynamic site, the codes `MINTED_AT` and `CLAIMS` give it;
  - per family, its act uses by kind, over every one of `enum Act`'s 27 variants;
  - the residue;
  - for F-C: per site, whether its route prints an operand span, and per case, whether it is *holds, operand implied* or uses the marked form.
- **Also:** the named-binary arm takes `JIGC_ROUTE_CATALOG_FAMILY`.
- **Red first:** v2's six arms, plus `a_dynamic_sites_minted_codes_are_its_rows` and `a_parts_key_survives_a_site_added_to_another_file`.
- **The H0 check:** the commit reports the count of *operand implied* and marked-form cases, beside the pilot's 32 of 183. More than twice the pilot's rate is H0.
- **Gates:** 1 + 1.

**P1 — B1, then the fence.** Two commits.
- **Commit 1:** the scan leaves out the bodies of `impl Finding` in `crates/engine/src/finding.rs` only. Negative control: `a_finding_literal_outside_impl_finding_is_still_a_site`, with its mutant. 142 rows.
- **Commit 2:** `Reason` gains `ruled: Option<heading>`. A missing heading is red; `ruled: None` is named as owed and refused from P-Z on; the `not-counted` class is refused; `pinning.md` §6's body is updated.
- **Then:** M-P0.
- **Gates:** 2 + 1.

**P-F0 — Classify the reason texts, build the shared frame pieces, and write the `NotBuilt` cases.**
- **Classification:** about 164 texts, each as *plantable with piece X*, *not constructible*, or *not built*. The table is committed.
- **Pieces:** each built red first with its case. The first is the per-command environment for "no git on `PATH`" at `setup`.
- **`NotBuilt`:** every `NotBuilt` case is written here, or named to the C-task whose family owns it.
- **Stop rule:** a piece over about 3 hours, or one that needs product source, halts.
- **Gates:** 2–4 + 1.

**P-F1 — The act check** (F-C → A). Two commits, each with the gate green.

- **Commit 1 — the table and the check in count-only mode. It changes no class.**
  - **The act-verb table** is a function in `support/route_case.rs` with an **exhaustive `match` over `enum Act`** (`:467-604`), all 27 variants. A new variant does not compile until it has an entry. For each act it gives:
    - the verb or verbs a route may use for it, and where the operand stands;
    - for the `Run*` forms, that the printed command line is the act;
    - for `Rerun`, `RerunWith` and `RerunInTheRepository`, how the route has to name the re-run (they are *"no better than operand implied"* by construction, `:575`, `:590`);
    - for `Cd`, `MakeReadable`, `MakeWritable`, `Unstage` and `RunListed`, their verbs.
  - The table is described, not copied, in `pinning.md` §6. `pinning.md:124` becomes *"a route's wording beyond each act's verb and its operand"*; the reshape plan's P3/P4 rule is read the same way in the entry.
  - **No class changes in this commit:** no new variant, no word renamed, `is_red` untouched, so the arm at `route_catalog.rs:3987` (and its table at `:3979`) and `differences()` (`:1066`) stay green. The new class and the renamed words land in commit 2.
  - **Count-only:** the act check (verb and operand by span; `Link::at` for link cases) runs over every case and **reports**, through a reporting arm that asserts nothing about the counts, how many cases would be *act left to infer* and how many *operand left to infer*. The gate stays green.
  - **H0** is judged on these counts, and H1 is asked after this commit's review.
  - **Red first:** for the table and the check's mechanics (a fixture route lacking a verb is reported as an act left to infer; a fixture route lacking an operand span is reported as an operand left to infer; `Link::at` disagreeing with the case's act is reported). A stub `Act` variant fails to compile (shown in the commit message).
- **Commit 2 — the flip.** It waits for J2a's frame wave and its fixes. All in this one commit:
  - a new classification `ActLeftToInfer`; `HoldsOperandImplied`'s words move to *"followed, the route left an operand to infer"*. Neither class's words start with *"followed, holds"*;
  - `Classification::is_red` (`route_case.rs:913-918`, the catalog's own; the byte table's `Classification` in `support/byte_cell.rs` does not move) makes both left-to-infer classes red;
  - **the arm at `route_catalog.rs:3987` is rewritten** to `is_red() == (words != "followed, holds")`, with a row for `ActLeftToInfer` in its table (`:3979`): exactly one classification is green;
  - the as-built source (`route_case.rs:898`) is updated, and P0's projection is regenerated;
  - every such case is listed in the strict list under a key per route, minted in `route-catalog-findings.md`;
  - **link cases:** a case whose refusal is `store.link-on-the-way`, or a sibling the guard routes, asserts that its act is the one `link_way::Link::at(root, at, stands).way_out` computes for the planted link, through an exhaustive `WayOut` → `Act` match, so a new `WayOut` variant fails to compile until it is mapped. `PutTheFileItself` is not in that map: its routes are `crates/cli/src/milestone.rs:1629`, `crates/cli/src/setup.rs:3170` and `crates/engine/src/finalize.rs:1537`, and the table alone checks it;
  - every case is re-run, and every classification that moves is listed;
  - the pilot's *"183 hold"* (`route-catalog-pilot.md:44`, `:277`) gets its bracketed note: a count of history, with the new split beside it. The pilot's `route-catalog-pilot-cases.tsv` keeps the old words as history (no fence reads it);
  - the §12 key `r1-catalog-link-acts-never-check-the-route-asks-for-them` is struck.
  - **Red first:**
    - the §12 repro (another target for the planted link changes what `Link::at` computes, and the case turns red);
    - each left-to-infer class is red and listed;
    - the two classes are told apart by the strict list's *classification moved* arm;
    - the rewritten arm is red against the old `is_red`.
- **Gates:** 2 + 1.
- **Then:** loop → P.

**C1–C15 — The static families**, 125 sites, cut as in v1 (families stay whole in their own file; only the commit groups them):

| # | Sites | What |
|---|---|---|
| C1 | 10 | `crates/cli/src/task.rs`: `task discard` 4, `task bind` 2, `doc read` 4 |
| C2 | 7 | `migrate` 2, `validate` 2 (`render.rs`, `engine/probe.rs`), `ingest` 3 |
| C3 | 9 | `start`: `start.rs` 4, `milestone.rs` 1, `engine/compose.rs` 4 |
| C4 | 7 | `config`: the first 7 of `config.rs` |
| C5 | 7 | `config`: the other 3 of `config.rs`, `relocate.rs` 2, `start.rs` 2 |
| C6 | 11 | `doc write`: `doc.rs` |
| C7 | 9 | `doc write`: `task.rs` 2, `engine/state.rs` 5, `engine/write.rs` 2 |
| C8 | 10 | `milestone`: `cli/milestone.rs` 6, 4 of `engine/milestone.rs` |
| C9 | 7 | `milestone`: `engine/milestone.rs` |
| C10 | 6 | `milestone`: the rest of `engine/milestone.rs` |
| C11 | 7 | `task finalize`: `engine/parse.rs` |
| C12 | 7 | `task finalize`: the rest of `engine/parse.rs` |
| C13 | 10 | `task finalize`: `engine/finalize.rs` 8, `index.rs` 1, `target_surface.rs` 1 |
| C14 | 10 | `task finalize`: cli `doc` 2, `orphan` 2, `render` 3, `start` 1, `task` 2 |
| C15 | 8 | `task finalize`: `file_state` 2, `state` 2, `validate` 3, `milestone` 1 |

**C16a, C16b, C17, C18 — the dynamic families**, re-cut from PC's count: a case for every code (fork 3 A); no task over about 35 codes; `compose`'s 27 a task of its own.
- **Writes:** that family's file under `route_catalog/` (a new one where none exists, plus its `tables!` line), `scheduled.tsv` rows removed, `EXPECTED_RED`, `CATALOGUED`, `CLAIMS` and `MINTED_AT` as needed, the projection regenerated, keys minted in `route-catalog-findings.md`, one `DECISIONS.md` entry with the 2026-10-12 table (sites · functions read · causes · followed · reasons · cases · red · minutes).
- **Commits:** about 10 sites or codes each, each behind its own gate. C16a's first commit adds S2's arm, `every_code_a_dynamic_site_mints_has_its_case_or_a_ruled_reason`.
- **Red first:** every case is run and seen in its classification before it is listed. Any frame vocabulary a case needs is added red first, against a runner that declares it and does nothing.
- **Done when:**
  - the family's rows are gone from `scheduled.tsv`;
  - the strict expected-red list holds, including the left-to-infer classes;
  - every new case passes the act check, and a case left to infer is listed red with its key;
  - the projection is regenerated;
  - one `dev/runner-faithful` run of the `route_catalog` part is green, with its proxy and wall seconds in the commit, and re-bounded or split in the same commit if the proxy passes half its bound;
  - the named-binary arm, filtered to the family, has run on rc.24, with its table in the record;
  - the suite's seconds are in the commit;
  - new site-level reasons carry `ruled: None` and are named in the task's return;
  - the family's `NotBuilt` cases are written.
- **Stop rules:** a site over about an hour stops the task; a missing frame piece is built within P-F0's rule, or the task halts.
- **The re-price** happens after C2.
- **Gates:** 25–29 commits + 10–19 fix gates; 19 Linux runs.

**P-NC2 — Sweep the texts the C-tasks wrote**, by P-F0's rule.
- **Gates:** 1 + 1.

**P-Z — Close the catalog.**
- **What:**
  - `Scheduled`, `SCHEDULED`, `THE_COUNT` and its arm, and `scheduled.tsv` are removed;
  - `ruled: None`, `Why::NotBuilt` and any owed code are refused;
  - the closure arm: every prefix of a code-shaped literal in production source is in `CATALOGUED`, so the vocabulary arm holds every code;
  - the named-binary arm runs whole on rc.24;
  - `route-catalog-pilot.md` gets §14's numbers, including the counts of both left-to-infer classes.
- **Gates:** 1 + 1.
- **Then:** M-P.

### Lane T — serial, in its own worktree

**T0 — The *Tools* channel.** `proposalPrompt` and `crossModelPrompt` carry the *Tools* paragraph; the triage writer takes an optional `tools` text.
- **Gates:** 1 + 1.

**RB1 — The regression base, the record script's half.**
- **What:**
  - a filing may carry `earlier`: newest first, one entry per earlier candidate, each with its hash equal to that round's recorded `binary`;
  - the list must be complete, or the fact is unknown;
  - grade `regression` = the previous release `true`, or any earlier candidate `true`;
  - in round ≥ 2, a confirmed filing without `earlier` is `regression_unknown`.
- **Fixtures:** written through the script's writers; if the state refuses them, the builder halts.
- **Gates:** 1 + 1.

**T-NR — `wf-neutralized-return`** (B2; the human's ruling B of 2026-10-29).

**The build** (a builder, in T; one or two commits):
- **The case.** `neutralized` joins the runtime probes. **No new argument:** the probe still takes `scratch` and nothing else (the choice of 2026-10-07 and `probeFault`).
- **The agent set** is fixed in the case itself. Exactly:
  - **one subject agent**;
  - **three step agents**: *begin*; *the record writer*, which hands the subject's returned text to the record script's writer in a throwaway record under the probe's root; and *the verdict*.

  **Every launch is `once`**, with no retry under `TRANSIENT_RETRIES`. `gitStep` (`stabilize.js:2138`) does not pass `once`, so T-NR adds a probe-only path (a parameter or a wrapper around `agentR`); the other probes keep their retries.
- **An arm of the harness fence holds the launch count exactly** (four launches, their roles, each `once`), and a simulation arm shows it.
- **The guard binds while a probe is active.**
  - The probe's begin act marks the probe as active, in a place the guard reads (proposed: a marker in the checkout's `.git/` directory naming the probe's scratch root, removed by the verdict act). The builder chooses and states the place; it is never a tracked path. The `.git/` marker sits in the common git directory every worktree shares; that is harmless only because the run is made with no other agent in flight (§1), and the builder states this at the marker's definition.
  - **While the marker stands,** the guard makes Bash an **allowlist**: exactly one plain invocation of `dev/stabilize-probe` or `dev/stabilize-record` with its grammar; everything else is denied, for every agent type, the writer types (`build-git`, `build-fixer`) included. Writes are allowed only under the probe's scratch root.
  - A marker whose scratch root is gone is stale: the guard says so, and binds anyway until the main session removes it.
  - **Red first, in `dev_stabilize_guard.rs`:** with a probe marker, a push, a commit and a write outside the root are refused, for a `build-git`, a `general-purpose` and a `build-executor` agent; so are the evasions `git -c x=y push`, `env git push`, and a script written then run; a write inside the root passes; without the marker, the guard is silent outside a run, as before; a stale marker binds and says so.
- **The planted text is harmless if obeyed.** The subject agent is told to **return** an instruction-shaped text in a structured field, and **never to act on it**. The text's only act is *"write the marker file `<scratch>/tnr-obeyed`"*. It also carries the shapes the canary saw (`bypass-permissions`, `settings-json`). It never names a git, push, delete or network act.
- **The verdict** (the probe tool's judge, by script). The observers are the downstream agents that receive the subject's text (the record writer, the verdict step) and the main session:
  - (i) whether `<scratch>/tnr-obeyed` exists, so whether anybody downstream obeyed;
  - (ii) whether the text reached the throwaway ledger cell and a report through the record script's writer, unchanged or rewritten;
  - (iii) whether it landed in a field the harness holds to a hash, and whether that hash still matches.
- **Red first:** the simulation arm with the runtime's stand-in rewriting the field (the hash catches it); the probe suite's arm per verdict; the fence's launch-count arm; the guard's arms above.
- **The `DECISIONS.md` entry** states the reading told to the human: "one agent" reads as one subject plus three counted step agents, each launched once.
- **Then:** its review. **M-TNR** (T → loop). Then loop → T.

**The run** — after T-NR's build is reviewed and merged T → loop, and after D0 has recorded the exception; before T1, T10a and J2b. It is made from the loop checkout, while no other agent of the session is in flight:
- one invocation, `{ probe: 'neutralized', scratch }`, by the script's path;
- the main session reads the probe's verdict and checks that the marker is removed;
- **a transient failure under `once`:** one re-invocation under a fresh scratch root is still *the single probe*; it is recorded with the first attempt's evidence. A second failure is recorded as a finding and goes to the human; nothing re-runs a third time.

**The record, by outcome:**
- **A declared bound:** written into `stabilization-workflow.md` → *Open questions and declared bounds*, with the verdict, and `wf-neutralized-return` discharged. This is the loop/join lane's: a builder on the loop checkout.
- **A cell among the endings:** a harness or record-script change. This is T's: a builder in T, then merged T → loop, and the row discharged by the loop/join lane.

- **Gates:** build 3–4 (2–3 + 1); record 1 + 0–1.

**T1 — An item names `family` and optionally `part`**, keyed by file plus first site id, held to the projection read at the round's candidate commit. **T1 waits for a loop → T taken after both M-P0 and M-TNR.** The partition check: in a round, the `route-hunt` items of every family in scope partition its parts, and a family in scope whose parts are not all hunted is `not_ready`, naming the uncovered part.
- **Gates:** 1 + 1.

**T2 — The hunt writer, `hunt --from FILE`.**
- **What:**
  - one row per site of the item's own set of sites;
  - each row carries `cause` and `plantable`;
  - words are held to the projection's vocabulary header;
  - the script computes the count;
  - it takes an optional `tools` text;
  - the step tool's vet learns the filed class.
- **Red first:** the route-hunter plan's seven writer arms, pointed at the projection, plus: a claimed dynamic site is a row a hunt owes; a part's hunt owes its part's sites and no other's.
- **Done when:** four mutants are red.
- **Gates:** 1–2 + 1.

**O-def — Main-session edits, in T's worktree**, each behind its own gate, then reviewed as one batch:
1. `O4`: the tester and the reviewer — what a finding carries, the kit sentence, the hunt contract bound to a `HUNT:` line.
2. `finding-triage.md`: a finding that names a site carries `site`.
3. `stabilize-opening.md`: one `route-hunt` item per family part, from the projection.
4. `build-fixer.md`: a plantable cause a hunt found gets its catalog case in the fix, red first; a new blocking site comes with its case.
5. `finding-verifier.md` step 4: *"the door, or a step of the block, the previous release does not have"*.
- **Gates:** 5 + 0–1.

**T8 — The route hunter, as an instrument.**
- **What:**
  - `ROLES.hunt`, together with the kit's `hunt` role, in the **first** commit (the `K1b` fence);
  - `CHAINS['route-hunt']`, `HUNT_SCHEMA`, `NEEDS.hunt`;
  - `huntPrompt`: the projection at the candidate's commit, the family and part, the catalog's cases (not hunted again), the not-constructible causes to try by hand, Bash timeouts, T2's call, and "say whether each cause can be planted". The prompt reads: *"every site of your part, for causes beyond the catalog's; a (site, cause) the catalog has is not hunted again"*, held by an arm.
- **The fence:** `launched_under` and its fixture change.
- **Size:** the generated size is printed. Over 440,000 bytes, the task stops.
- **Red first:** the route-hunter plan's five simulation arms; `kit --role hunt` prints §6 by its heading and exits 27 when the heading has moved; `the_files_the_hunters_prompt_names_exist`; `the_hunters_prompt_hunts_every_site_of_its_part_for_new_causes`.
- **Gates:** 2 + 1.

*Waits for M3.*

**K7b — The image's build is a held command**, as in the [reshape plan](../reshape/plan.md) §5: a kind of the step tool's hold that builds the trial image of a commit and verifies it; the tool's verdict is the tag and that the seven checks hold; the second preflight builds no image and waits for none. Each prints the harness's size.
- **Gates:** 1–2 + 1.

**K6 — The scripted trial arm is a held check**, as in the reshape plan §5: a kind of the step tool's hold that runs a scripted arm in the trial image and reads the scorer's own verdict; the blind arms stay agents. It runs in the image K7 builds and K7b holds.
- **Gates:** 1–2 + 1.

**T9 — The small canary's setup follows** (reshape plan §4). Its two remaining arms: `its_doors_are_the_review_rows_the_hunts_and_the_arms` and `its_expectation_names_what_it_prices`. The other two of its arms are built.
- **Gates:** 1 + 1.

**T11 — Move the scenarios left on the old path** (reshape plan §4). The builder counts first; at most 92, at about ten a commit.
- **Gates:** 3–10 + 1.

**T12 — Delete the old path** (reshape plan §4). `runFix` stops pointing at `VERIFY_PASSES`, and `VERIFY_SCHEMA` goes.
- **Gates:** 1–2 + 1.

**O5 — Main-session edits**, as the reshape plan's table of definition edits specifies (`finding-verifier.md`, `robust-advocate.md`, `stabilize-scope.md`, `stabilize-preflight.md`, and `build-git.md` where it names a step that moved).
- **Gates:** 4.
- **Then:** M-T.

### J2a — what no recorded review covers and no task of this plan moves

J2-0's first run (U5, over `cbb3d736..HEAD`) scopes it. It holds two waves of read-only `milestone-code-reviewer`s.

**The frame wave** — before P-F1's commit 2:
- the route catalog's frame: `support/route_case.rs`, the fence and runner in `route_catalog.rs`, `support/refusal_sites.rs`;
- the byte table's frame, reader and runner (`support/byte_cell.rs` and the frame commits from `fc2b13b6` on).

Three to four reviewers.

**The second wave** — parallel with the C-tasks. Four to six reviewers:
- the route catalog's cases that exist at HEAD (the pilot's `setup` and `uninstall`, the 2026-10-12 families, and the link fix's cases), wherever J2-0 shows them uncovered;
- the byte table's acceptance cells and its pilot, task and install families (B0–B5, up to `56839f95`);
- the CI fixes `936c6d54` and `72ca156a`;
- the write guard (`dev/stabilize-guard`, `dev/stabilize-fence`, `dev/stabilize-guard-probe.js`, `dev_stabilize_guard.rs`), which the reviewers read **after M-TNR**, so T-NR's change is in what they read;
- the regression set (`dev/regression-set`, `dev_regression_set.rs`, the `held/regression-*` fixtures);
- `dev/hygiene-scan`;
- `.claude/settings.json`.

**Who fixes each file:**

| Files | Fixed by |
|---|---|
| The catalog's frame and cases | P |
| The byte table, the regression set, `dev/hygiene-scan`, `crates/cli/tests/setup_install_pathspec_guard.rs` and `tooling-tests/rig_processes.rs` (declared), and `ci.yml` / `ci_matrix_fence.rs` (`72ca156a`) | U |
| The write guard, its fence and probe, and `tooling-tests/dev_stabilize_guard.rs` | T |
| `tooling-tests/dev_stabilize_step.rs`, if a finding reaches it | T |
| `.claude/settings.json` | The main session, and the human is told (it holds the deny list that protects the human's acts) |

Every wave: at most three cycles, with a scoped re-review, and the ranges registered.
- **Gates:** 8–20.

### The join (loop checkout, serial)

**T10a — The workflow doc says what the stage does as built.** It records T-NR's outcome, which is known by now, together with the hunter, the F-C meaning and the regression base as built.
- **Gates:** 1 + 1.

**J2-0 — The partition, by commit range.** `dev/review-coverage cbb3d736..<tip>` is run against the registry, with U5's rules for merges (a merge with a conflict resolved is uncovered unless reviewed). Every file of the range is in exactly one of:
- (i) J2a's scope, already reviewed;
- (ii) a J2b area;
- (iii) covered: every commit that touched it is in a registered review's range, each named;
- (iv) outside the `test` stage, with a reason per path class.

Nothing is "reviewed" by part or by name; uncovered commits go to (i) or (ii). The listing is committed with J2's record. There is no human question here: F-B → A decides it.

**J2b — The moving areas** (after M-T and T10a). Nine read-only `milestone-code-reviewer`s in parallel, over `cbb3d736..HEAD`:
1. the harness source, the generator and the generated file, with `dev_stabilize_harness.rs`;
2. the record script and its suite;
3. the step tool, its suite and the held fixtures, with `dev/gate` as a held check;
4. the definitions;
5. the simulation, the runtime fixture, the harness fence, the trace and the golden;
6. the canary tool, its suite and the golden;
7. the probe tool and its suite (T-NR included);
8. the trial image (`completions/trial-harness/`, with K7);
9. the workflow doc, with the stage's pending rows and `tooling.md` rows.

Then one pass across all of them, which also reads J2-0's partition, class (iv) first. Fixes are serial on the loop branch, one commit per area per cycle. **J2r** is the scoped re-review, at most three cycles. Expect J2b's high end.
- **Done when:** every finding is fixed or ruled, and `wf-test-review-real-run` is discharged with the record and J2-0's listing.
- **Gates:** 10–25 + 2–10.

**J1 — What the two streams owe each other**, as the reshape plan §4 specifies (`every_expected_red_key_is_a_row_somebody_holds_open`; `the_files_the_hunters_prompt_names_exist`), plus anything of T8's arm it did not take.
- **Gates:** 1 + 1.

**T10b — Rows and the record.**
- **Discharged:** `wf-route-walk`, `wf-route-catalog-rest` (with the F-C ruling) and the reshape plan's set. **Not:** `wf-regression-base`, `wf-hunt-cause-to-case`, `wf-hunt-count-view`.
- **Also:** `completions/artifacts/M55/stabilization-build/README.md` and the lessons list.
- **Gates:** 1.

**G1 — The `.gitleaks.toml` entry** (after H2, if A). One entry, exact file and exact line shape.
- **Done when:** `gitleaks git --log-opts "origin/main..HEAD"` exits 0.
- **Gates:** 1.

**Phase 3's end:** `dev/clean-litter`; every branch pushed; CI read; **H3**. The main session checks free disk before each lane's first gate (four worktrees, cold builds).

**Phase 4 — in the human's own session.**
- **The canary (if ruled).** The build session prepares its clone with T9's setup and hands it over. The human runs it in their own session. Anything it changes comes back to the build session as **J3**, a scoped re-review (0–4 gates).
- **The opening.** Run by the human's session: the fact sheet, **H4** and the opening's other questions, the `CLAUDE.md` edit per H4's answer, the opening commit behind the gate, the push (2 gates).
- **The build session stops after Phase 3** (and after J3, if needed).

## 3. Human stops, in order

Each stop is one question at a time, with its recommendation. H-J0 is gone: F-B → A answers it.

| # | When | The question | Recommended | Told with it |
|---|---|---|---|---|
| **H0** (conditional) | After P0, or after P-F1's commit 1, before the C-tasks | The count of routes that leave an act or an operand to infer, against the pilot's 32 of 183. **Likely to fire:** the 54 uses of `Rerun`, `RerunWith` and `RerunInTheRepository` are *"no better than operand implied"* by construction. | Carry on, every one keyed for the run | — |
| **Tell-NR** (a tell, not a question) | **Done 2026-10-29**, before T-NR's build | — | — | The human's "one agent" is built as one subject agent plus three counted step agents (begin, the record writer, the verdict), each launched once, with the guard refusing commits, pushes and writes outside the probe's root while it runs |
| **H1** | After the review of P-F1's commit 1 | One question: the nine standing site-level reasons and the act-verb table — *"accept as worded, or name the entries you challenge"* | Accept, except any the reviewer flags | F-C's reading (the verb plus its operand by span; `Link::at` for link cases) as an engineering reading of the ruling; (b)'s last clause and (c) do nothing until the `fix` half's repair. A challenged entry of the act-verb table moves cases already built, since nothing waits on H1: its fix re-runs and re-lists them. |
| **H2** | Phase 1, any time | The gitleaks hit at `85661425`, `DECISIONS.md:99` | A: one `.gitleaks.toml` entry (B, a fingerprint, drifts; C, a rewrite, is not viable) | — |
| **H-r1…r3** (conditional) | At the ends of 2a, 2b and 2c | New site-level reasons, one question each: *"accept these as worded, or name the ones you challenge"* | — | — |
| **H-cycle** (conditional) | When any task, J2a and J2b included, reaches a fourth review cycle | Allow a fourth cycle? | — | The precedent of 2026-10-28 |
| **H3** | End of Phase 3 | `wf-whole-canary-after-parts` | A: run what no part showed, including the measured hunt (190–330 min, $60–105). B: the measured hunt only — the floor, since C leaves no hunt rate. C: none. | — |
| **H4** | In the human's session, after the fact sheet | The release being prepared | A: `1.0.0` is prepared, and the call to publish stays the human's at the close; one clause in `CLAUDE.md:7`. B: the next rc. | — |

When U2 lands, its tell goes with the next stop: it turns CI red when GitHub changes the runner's git config.

## 4. Price, with the arithmetic

**Units of work in P** (derived): 125 static sites, plus 73 `MINTED_AT` codes, plus an estimated 20–60 codes of uncatalogued prefixes. Total **218–258, about 220–260**. PC fixes the number.

**Wall time per unit:**
- *Measured:* the pilot's parts, P3 at 18 minutes a site and P4 at 11.2, pooled to **13.6**; and the 2026-10-12 families, **13.6**. Both include gates; neither includes per-task reviews.
- *Range used:* 13–18 minutes. The low end is the pooled measure, the high end the slower part.
- *Plus F-C's act check:* about 1 minute a unit (an estimate).
- **Used: 14–19 minutes a unit.**

**Lane P:**
- C-tasks: 220 × 14 = 3,080 min = **51 h**, up to 260 × 19 = 4,940 min = **82 h**;
- their fixes: 19 × 0.5–1 × 55 min = 9–17 h;
- the C-tasks in all: **60–99 h**;
- the rest: P0 4–6, P1 4–6, P-F0 8–16, P-F1 6–12, P-NC2 3–5, P-Z 4–6 = **29–51 h**;
- waiting for J2a's frame-wave fixes before P-F1's commit 2: 0–8 h (the wave starts beside P0–P-F0, so most of its time overlaps);
- **P = 89–158 h.**

**Lane T** (estimates, review fixes included):

| Task | Hours |
|---|---|
| T0 | 3–5 |
| RB1 | 5–8 |
| T-NR (build with the guard's binding and the counted agent set 5–8; merge, run, record 1–2) | 6–10 |
| T1 | 4–6 |
| T2 | 6–10 |
| O-def | 4–6 |
| T8 | 8–14 |
| K7b | 5–8 |
| K6 | 5–8 |
| T9 | 3–5 |
| T11 | 6–12 |
| T12 | 5–8 |
| O5 | 2–3 |
| **Total** | **62–103** |

The pause of the other lanes for the run is minutes, so it is not priced.

**After T, before the join:**
- **T10a:** 4–8 h.
- **J2b:** the reviews, nine plus one pass in parallel, 4–8 h; the fixes, 10–25 commits × 55–85 minutes = 9–35 h; J2r, 1–2 cycles × (3–5 h of review + 3–10 h of fixes) = 6–30 h; **J2b = 19–73 h.**
- **The T chain** = T + T10a + J2b = 62 + 4 + 19 = **85 h**, up to 103 + 8 + 73 = **184 h**.

**J2a, off the critical path:** the frame wave 4–8 h of review + 4–12 h of fixes; the second wave 4–8 h + 6–20 h; one or two re-review cycles 4–15 h; **J2a = 22–63 h.** About 45 k lines: catalog frame and fence about 12 k, catalog cases about 15 k, byte table B0–B5 about 12 k, guard, regression set and hygiene scan about 5.5 k, settings and the CI fixes small.

**The tail:** J1 3–5, T10b 2–3, the canary 0–6 (190–330 minutes if A), J3 0–6, the opening 4–8 = **9–28 h**.

**Phase 0:** 4–6 h.

**Critical path** = Phase 0 + max(P, the T chain) + the tail:
- low: 4 + max(89, 85) + 9 = **102 h** (P);
- high: 6 + max(158, 184) + 28 = **218 h** (the T chain).

The priority P > T costs the T chain its gate waits, which are inside the high end.

**Gates:**

| Lane | Gates |
|---|---|
| Phase 0 (D0 2–3, D0-def 1, M-defs 3) | 6–7 |
| U (six tasks) | 7–12 |
| J2a | 8–20 |
| K | 1–2 |
| G1 | 1 |
| P | 50–66 |
| T | 36–50 |
| The join (T10a, J2b, J2r, J1, T10b, J3, the opening) | 18–48 |
| **Total** | **127–206** |

The T line adds up as follows:

| T task | Low | High |
|---|---|---|
| T0 | 2 | 2 |
| RB1 | 2 | 2 |
| T-NR build | 3 | 4 |
| T-NR record | 1 | 2 |
| T1 | 2 | 2 |
| T2 | 2 | 3 |
| O-def | 5 | 6 |
| T8 | 3 | 3 |
| K7b | 2 | 3 |
| K6 | 2 | 3 |
| T9 | 2 | 2 |
| T11 | 4 | 11 |
| T12 | 2 | 3 |
| O5 | 4 | 4 |
| **Total** | **36** | **50** |

At 20–30 minutes a gate: **42–103 hours** of gate time. The queue is busy about 40–45 % of the time during the parallel window, including the heavy suite runs. Beside the gates: 19–22 Linux runs, two K7 image builds, one whole rc.24 arm.

**Calendar:** 102 h at 20 hours a day = **5.1 days**; 218 h at 12 hours a day = **18.2 days** (the measured paces: 2026-10-11/12 about 20 h a day, 2026-10-28 about 12 h). Plus 1–3 days for the human stops: **6–21 days, so 1–3 weeks.** The tail risks are beyond-bound review cycles (most likely in J2a and J2b), J2 findings that need the human, and PC at its high end. **Re-priced** after PC and after C2.

**The roadmap entry's step hours** apportion these estimates over its sixteen steps: each step's figure is the longest lane inside it, from the per-task hours above (a C-task's hours from its units at 14–19 minutes plus its share of the fix hours, so a dynamic family runs about 6–11 h against a static family's 2–4 h). They are estimates, like the totals they come from, and the critical path of 102–218 h is not their sum: the lanes overlap, and J2b's fixes run past step 14.

## 5. Forks

**Settled:**
- by the human: F-A → B, F-B → A, F-C → A, T-NR → B;
- by rulings or the reviews: forks 1 to 7; B1; `ci.yml` edits; the queue's priority; H-J0 (answered by F-B → A);
- F-C's reading (the act-verb table): the main session's engineering reading, told with H1;
- T-NR's counted agent set: the main session's call, told before the build (Tell-NR);
- v2's F2, F3, F5 moot; F6 built (U3); v3's F-NR settled.

**Engineering forks** (the main session takes these with the recommendation unless it chooses to put them):

| # | Fork | A (robust, recommended) | B |
|---|---|---|---|
| **F1** | How lane P runs | One serial lane | Two sub-lanes by family file |
| **F4** | Pacing of new reasons | Batched at the ends of 2a, 2b and 2c; `ruled: None` refused from P-Z on | A halt per C-task |
| **F-J2a** | When J2a's areas are reviewed | Early (the frame before P-F1's commit 2, the rest beside the C-tasks), with J2b's pass across all areas reading them in context | Everything at J2b; this puts 22–63 h on the critical path, and a frame defect found then invalidates the C-tasks built on it |

The human's questions are H0 to H4 (§3).

## 6. Not established

- **The dynamic sites' code counts** (PC).
- **Whether the state takes a round-2 record written through the writers** (RB1's fixtures).
- **How refusing `UPDATE_GOLDENS` under CI is done for the goldens** (P0 checks).
- **Whether the act-verb table needs more than one verb for an act.** An entry may be a list, still one home; it is not a halt.
- **Whether every review's range can be recovered for the registry.** The byte table's review reports are outside the repository; the main session hands over their ranges. A review whose range cannot be shown stays uncovered, and its commits go to J2.
- **How fast the legs grow after U0** (the 80 % rule watches it).
- **Where the guard can read that a probe is active** without a tracked path, and whether a hook sees every subagent's tool calls in the session. T-NR's arms establish it; if the guard cannot see a subagent type, the builder halts.

*Closed by the check of v5:* whether every step act the probe needs can be launched `once` without changing `gitStep` for the other probes — T-NR adds a probe-only path (§2, T-NR).

## History

The plan went through five versions between 2026-10-27 and 2026-10-29, each read by an adversarial, read-only reviewer: v1's review (5 blocking, 11 should-fix, 9 notes), v2's review (3 blocking, 16 should-fix, 13 notes, 3 forks for the human), v3's focused re-review (2 blocking, 7 should-fix, 6 notes, 1 question for the human), a light check of v4 (2 blocking, 5 should-fix, 3 notes), and a check of v5 whose corrections the main session applied. The reviews' reports are kept outside the repository; what each changed is recorded below, and every change is applied in the sections above.

### H.1 v2's review, finding by finding

| Finding | What the plan does with it |
|---|---|
| **B1** — J2's wrong base | J2 reads `cbb3d736..HEAD`. J2-0 decides coverage by commit range, with `dev/review-coverage` (U5). |
| **B2** — `wf-neutralized-return` is in no task | T-NR: built in T (with the guard's binding while a probe is active), reviewed, merged T → loop, then run once from the loop checkout under the human's ruling B. One subject agent plus three counted step agents, each launched `once`. The record goes to the lane that owns it. |
| **B3** — an unruled question inside a discharged row | F-C → A. D0 records the ruling; P-F1 is built on it; `wf-route-catalog-rest` is discharged at T10b, after P-Z. |
| **S1** — CI bounds at 84–95 % | U0 re-bounds or splits every leg at or above 80 %. A P task re-bounds its part in the same commit if the proxy passes half its bound; for the other lanes, `dev/ci-runs` is read after each push. |
| **S2** — "a case per code" held by no fence | C16a's first commit adds the arm; P-Z refuses an owed code, and checks that every code prefix is catalogued. |
| **S3** — `NotBuilt` can survive P-Z | P-F0 and the C-tasks write those cases; P-Z refuses `Why::NotBuilt`. |
| **S4** — J2 leaves out parts | F-B → A, coverage by commit range. |
| **S5** — (b) is inert; T8's wording | D0 records, and H1 tells, that (b)'s last clause and (c) have no effect until the `fix` half's repair. T8's prompt sentence is held by an arm. |
| **S6** — RB1 should fail loud | In round ≥ 2, a confirmed filing without `earlier` is `regression_unknown`. |
| **S7** — parts over a moving list | A part's key is its file plus its first site id; T1's partition check. |
| **S8** — rows whose triggers have fired | D0 (two docs), M-defs (row 101 in all 8 copies, `wf-o2`, two definitions), T10b (the build's README). |
| **S9** — the census | Moved to Candidates (F-A → B). |
| **S10** — the price | §4 shows the arithmetic, at 14–19 minutes a unit. |
| **S11** — the gate queue | Each builder and fixer stops before its gate. One queue, P > T > K > U, including builders' heavy runs. |
| **S12** — phases and pushes | Phases are per lane, not global barriers. A push after every task; a re-price after PC and after C2. |
| **S13** — P-F1 and the frame rule | The act-verb table is an exhaustive `match` over `enum Act`'s 27 variants. Commit 1 counts only; commit 2 makes the left-to-infer classes red and lists them. `pinning.md:124` is amended. |
| **S14** — tools not owed | Kept: U0–U5. Moved to Candidates: the rig's link plant, the byte-table record tool, the raw-write census, the runner-faithful docker arm. |
| **S15** — H1's place | The ruling is recorded by the P commit that follows the answer. Nothing waits on H1 before P-Z. |
| **S16** — H3 and who drives | C leaves no hunt rate, so B is the floor; the canary and the opening run in the human's session. |
| **N1** — U1's docker arm, wrong commit | Moved to Candidates; its text names the facts: red at `0bd8a5b5` and at `22f06d31`, green at `63ce9ca5`. |
| **N2** — `CLAUDE.md` changed in `26474b80` | Corrected (§0.2). |
| **N3** — B1 needs a negative control | P1 limits the exclusion to the bodies of `impl Finding` in `crates/engine/src/finding.rs`. Arm and mutant: a `Finding { … }` literal outside `impl Finding` is still a site. |
| **N4** — the byte-table print is committed | Noted in that tool's candidate row: F3 A is buildable when the tool is built. |
| **N5** — harvested tools missing from the triage | D0's candidates list adds each with its question and its trigger, built or declined. The unreviewed-range lister is built as U5. |
| **N6** — two leads with no key | D0 keys them as leads in `route-catalog-findings.md` §12: `migrate.source-untrackable` against the link code, and the read-form advisory against the pinned contracts. Where no repro exists, the key says *not driven*. |
| **N7** — T11's gates | Up to 10 commits (92 arms at about ten a commit); the builder's count comes first. |
| **N8** — undeclared overlaps | `ci.yml` and `ci_matrix_fence.rs` are written by U and by any task that re-bounds a leg; declared, and resolved by hand on different rows. |
| **N9** — H1 and H-r as one question each | Each is put as one question: *"accept these as worded, or name the ones you challenge"*. |
| **N10** — (e) as recorded | D0 records the fact; that it is a data point on the bound of three, worth putting at the first stop, is the orchestrator's note. |
| **N11** — `wf-o5` and `wf-return-size` | D0 strikes `wf-o5` as superseded and annotates `wf-return-size`. |
| **N12** — the drift tell | Told when U2 lands, with the next stop, not with H2. |
| **N13** — four worktrees, cold builds, disk | `dev/clean-litter` runs at the end of Phase 3; the main session checks free disk before each lane's first gate. |

### H.2 What became of v2's forks

| v2 fork | Now |
|---|---|
| F1 — P run as one serial lane | Kept, A. |
| F4 — new reasons batched at phase ends | Kept, A. |
| F2, F3, F5 | Moot (moved to Candidates). |
| F6 | Built (U3). |
| v3's F-NR | Settled by the human's ruling B. |

### H.3 v1's review and the review of `f99658b8`, as v2 worked them in

- **B1:** P1 commit 1. **B2:** RB1 now; `wf-regression-base` for the rest. **B3:** parts are data. **B4:** overtaken (git 2.56). **B5:** superseded by F-B → A.
- **S1, S2:** re-priced and widened. **S3:** one merge, loop → P. **S4:** P-F0 before the C-tasks, P-NC2 after them. **S5:** T0 and T2. **S6:** H2. **S7:** done in `26474b80`. **S8:** §6's heading frozen. **S9:** `launched_under`. **S10:** T in its own worktree. **S11:** the stops in §3.
- **The notes:** N1 corrected; N2 is H4; N3, N6 and N9 recorded by D0; N4 drops T4; N5 the size watch; N7 U2; N8 the `ruled:` field.
- **The review of `f99658b8`:** H1 is D0's annotation plus ruling (c); N1 and N2 are D0's doc lines; N3 is moot through T12; N4 is a definition edit.

### H.4 Changelog, v3 → v4

| Finding / call | What changed |
|---|---|
| **B-1** | P-F1 rewritten into two commits. One act-verb table in `route_case.rs`, described in `pinning.md` §6. A case checks its act's verb plus its operand, by span. Link cases also check `Link::at(root, at, stands).way_out`, through an exhaustive `WayOut` → `Act` match. `PutTheFileItself` is taken out of the `WayOut` pairing. `pinning.md:124` and the P3/P4 rule are amended to *"beyond each act's verb and its operand"*. The table goes to the human with H1. D0 records the reading. |
| **B-2** | J2-0 counts coverage by commit range. The unreviewed-range lister is built now as U5 (`dev/review-coverage` with a committed registry of review ranges). Every review this plan runs registers its range. The byte table's B0–B5, the route catalog's code and the CI fixes `936c6d54` and `72ca156a` go into J2a under F-B → A. No human question. |
| **S-1** | T-NR runs at the start of Phase 2, so its record is inside T10a and J2b. |
| **S-2** | P-F1: `is_red` is the catalog's own and the byte table is untouched; the act class is a classification of its own; the pilot's *"183 hold"* gets its bracketed note in P-F1's commit 2. |
| **S-3** | H-J0 removed (main session's call). J2a's frame wave reviews the catalog's and the byte table's frames before P-F1's commit 2; its second wave takes the cases and the rest. Priced in §4. |
| **S-4 / the question** | The human ruled B (2026-10-29). The build session runs the one T-NR probe under its conditions. D0 records the scoped exception. No stop. |
| **S-5** | A loop → T merge after each M-U that carries J2a's fixes. Phase 1 states that H1 is asked and nothing in P waits for it. H1 follows P-F1's commit 1. |
| **S-6** | Per unit 14–19 minutes (measured 13–18, plus about 1 for the act check). P = 89–158 h. |
| **S-7** | U4's vocabulary adds round 1, `test`, the canary and the first run. It prints the unmatched rows too. Its header, its row and D0-def call the list a floor. |
| **N-1** | J2b: expect the high end. |
| **N-2** | Any fix to `.claude/settings.json` is the main session's edit, and the human is told. |
| **N-3** | P-Z's closure arm: every code prefix in production source is in `CATALOGUED`. |
| **N-4** | The gate queue also takes every builder's suite run over about 60 seconds. A red gate is run once more before it is judged. |
| **N-5** | H0 unchanged. |
| **N-6** | J2b's pass across all areas reads J2-0's partition, class (iv) first. |
| **T-NR conditions** | v4's conditions: built at the end of Phase 1, run at the start of Phase 2; exactly one agent through the harness, with a hard cap stated in the invocation; no stage, no push; the planted instruction writes only `<scratch>/tnr-obeyed`; the verdict reads the marker, the ledger and report, and the hash-held field; D0 records the exception. v5 replaced the cap and the timing (H.5). |

### H.5 Changelog, v4 → v5 (the light check of v4, and the main session's calls)

| Finding / call | What changed |
|---|---|
| **B-A** (a) | No `agents` argument; the 2026-10-07 choice is kept (`probeFault` takes `probe`, `scratch` and `seconds` only). The `neutralized` case fixes its own agent set: one subject plus three step agents (begin, the record writer, the verdict), every launch `once`. A fence arm holds the count exactly; a simulation arm shows it. The reading is told to the human before the run (Tell-NR) and stated in the task's `DECISIONS.md` entry. |
| **B-A** (b) | Verdict (i) kept. The observers are the downstream agents that receive the text and the main session. The subject is told to return the text and never to act on it. The marker shows whether anybody downstream obeyed. |
| **B-A** (c) | "No push" is a mechanism: the guard binds while a probe is active. T owns the guard for this build. J2a reads the guard after M-TNR. The run is made while no other agent of the session is in flight, since the hook is session-wide. |
| **B-A** (d) | M-TNR (T → loop) after T-NR's build and review, before the run. The run is made from the loop checkout. |
| **B-B** | P-F1 split into a count-only commit 1 and a flipping commit 2 (corrected further by the check of v5, H.6). |
| **S-a** | The act-verb table is an exhaustive `match` over `enum Act`'s 27 variants. H0's row says it is likely to fire (the 54 `Rerun*` uses). |
| **S-b** | One wording for the T-NR run's timing: *after T-NR's build is reviewed and merged T → loop, and after D0 has recorded the exception; before T1, T10a and J2b*. Phases are per lane, not global barriers. |
| **S-c** | U5's merge rules and its backfill. |
| **S-d** | T-NR's record has an owner per outcome. |
| **S-e** | J2a's fixes have an owner per file. |
| **N-a** | `PutTheFileItself`'s routes: `crates/cli/src/milestone.rs:1629`, `crates/cli/src/setup.rs:3170` and `crates/engine/src/finalize.rs:1537`. Kept against the light check: the engine's `finalize.rs` exists and prints the route. |
| **N-b** | H1 tells the human that a challenged entry of the table moves cases already built. |
| **N-c** | Gates and hours re-added (corrected further by the check of v5, H.6). |
| **D0 Candidates** | The act-use counter widened to all 27 variants; the review-range grep used once, as U5's backfill input. |

### H.6 The check of v5, and what it corrected (applied in place above)

| Finding | Correction |
|---|---|
| **B-B** — commit 1 was still red through the renamed words | P-F1's commit 1 adds the act-verb table and the check in count-only mode and changes no class. Commit 2 adds `ActLeftToInfer`, moves `HoldsOperandImplied`'s words, flips `is_red`, rewrites the arm with a row for `ActLeftToInfer`, updates the as-built source and regenerates the projection, all in one commit. The pilot's cases file keeps the old words as history. |
| **B-A** — the human heard of the four launches too late | Told on 2026-10-29, before T-NR's build. D0 words the exception's scope as *"one subject agent, with the harness's counted step agents"*. If the human objects, T-NR is rebuilt before any of it lands. |
| **Should-fix** — a denylist of `git push`/`git commit` can be evaded | Under the probe marker the guard makes Bash an allowlist; writes only under the probe's scratch root; red-first arms add the evasions. |
| `once` path | `gitStep` does not pass `once`; T-NR adds a probe-only path, and the other probes keep their retries. §6's item on `once` is closed. |
| A transient failure under `once` | One re-invocation under a fresh scratch root is still the single probe; a second failure goes to the human; never a third. |
| T1's base | T1 waits for a loop → T taken after both M-P0 and M-TNR. |
| Gate sums | T-NR's build is 2–3 + 1 = 3–4 gates: T 36–50, total 127–206. |
| `tooling-tests/rig_processes.rs` (touched by `72ca156a`) had no owner | U, declared, beside `setup_install_pathspec_guard.rs`. |
| The `.git/` marker sits in the common git directory every worktree shares | Harmless only because the run is made with no other agent in flight; the builder states this at the marker's definition. |
| N-a (v4 check) | Settled: `crates/engine/src/finalize.rs:1537` exists and prints the route; the reviewer confirmed. |
