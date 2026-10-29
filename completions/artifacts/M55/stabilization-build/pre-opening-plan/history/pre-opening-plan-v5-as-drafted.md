# Build plan v5: everything owed before the first stabilization run opens

Read-only planning, based on `fix/rc24-tier1` @ `7c34a222` (clean tree; CI run `40767224323` green).

This revises v4 (`pre-opening-plan-v4.md`), working in:
- a light check of v4 (2 blocking, 5 should-fix, 3 notes);
- the main session's calls on it.

Everything earlier plans worked in stays as §0.3 to §0.5 and §7 record it. §8 lists what changed from v4 to v5. Every count was taken at `7c34a222` unless it says otherwise.

## 0. What changed since v2

### 0.1 The human's rulings on the reviews' forks (D0 records all four)

**F-A → B (2026-10-28). Ruling B's two open siblings go to the opening ledger and are fixed by the run.**
- The two keys:
  - `r1-config-set-writes-through-a-link-at-the-manifests-own-place` (`byte-table-findings.md` §30);
  - `r1-uninstall-half-tears-down-over-a-hook-link-it-cannot-read` (`route-catalog-findings.md` §12).
- *Why:* known findings are fixed inside the run. Fixing everything before the opening is the ad-hoc loop the workflow replaced.
- *Consequence:* only tools that a task before the opening, or one of the run's instruments, needs are kept. The rest move to `tooling.md` → Candidates, each with its trigger.

**F-B → A (2026-10-28). J2 reviews everything the `test` stage consists of, over `cbb3d736..HEAD`.**
- What has its own review counts as reviewed. "Reviewed" is decided by commit range (§2, J2-0).
- So the following go into J2, with no new question:
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
- The exception is scoped to single-agent probes of a harness mechanism. A stage, a round or a canary still runs in the human's own session.
- **[v5] What "one agent" means in practice.** A probe always launches `build-git` step agents for its acts (`probeAct` → `gitStep` → `agentR`, `stabilize.js:2745-2800`). So "exactly one agent" is built as:
  - **one subject agent**;
  - plus a stated, counted set of **three step agents** (begin, the record writer, the verdict);
  - **every launch `once`**, with no retry.

  The main session tells the human this reading before the run (§3).
- The conditions are in the task (§2, T-NR):
  1. the timing (the one wording of §1);
  2. the fixed, counted agent set, with no stage, no push and no retry;
  3. a planted text that is harmless if obeyed;
  4. D0 records the exception;
  5. **[v5]** "no push" and "no commit" are held by a mechanism: the write guard binds while a probe is active.

### 0.2 Facts corrected or added

- **The `CLAUDE.md` runner sentence changed in `26474b80`**, not in `63ce9ca5`.
- **J2's base is `cbb3d736`**, the tree the last re-review of the `test` half read (`603018ac^..cbb3d736`).
  - Over that range: 247 commits and 413 files.
  - Over `dev/ .claude/ tooling-tests/ implementation/stabilization-workflow.md completions/trial-harness/ .github/`: 60 files, +78,721 / −3,618 lines.
  - The source files among them: harness source +5,537, record script +4,795, canary tool +2,581, step tool +2,441, regression set +813, probe tool +793, guard and fence and guard probe +546, generator +234, hygiene scan +78, settings +21, doc +438, definitions about +250.
  - About 60 k of the lines are suites.
- **What no recorded review covers:**
  - the byte table from `fc2b13b6` through `56839f95` (12 commits): +12,265 lines under `crates/cli/tests/`. B6's review, the first on record, reads `56839f95..f4c12fa3`;
  - the route catalog's code (no heading of P1–P5 or of the 2026-10-12 families records a code review; only the link fix's reviews read the commits that added `ReplaceWithACopy` and `MoveDirectoryIn`);
  - the CI fixes `936c6d54` and `72ca156a`.
- **CI's legs against their bounds** on run `40767224323`:

  | Leg | Took | Bound | Share |
  |---|---|---|---|
  | `g_tooling, rest` | 629 s | 11 min | 95 % |
  | `g_tooling, simulation` | 642 s | 12 min | 89 % |
  | `g_migration` | 504 s | 10 min | 84 % |
  | `g_tooling, record` | 518 s | 14 min | 62 % |

- **Routes that print no operand:** in the pilot, 32 of 183 holding cases (*holds, operand implied*), out of 74 cases whose route printed no operand (`route-catalog-pilot.md` §6). That is H0's expectation.
- **The act vocabulary in the product:** only `link_way::WayOut` (`link_way.rs:66-88`). Its clause is private (`:109`). The public question is `Link::at(root, at, stands)` (`:99`, its `way_out` field public).
- **[v5] `enum Act` has 27 variants** (`support/route_case.rs:467-604`). By use count in the catalog: `Rerun` 45, `Remove` 32, `Commit` 28, `MoveOut` 24, `Cd` 19, `MakeWritable` 18, `MoveAway` 17, `RunListed` 17, `MoveLinkOut` 16, `PointAt*` 15, `PutTheFileItself` 13, `MakeReadable` 11, `Revert` 11, `Mend` 6, `Unstage` 5, `RerunWith` 5, `RerunInTheRepository` 4, and the other `Run*` forms and link acts.
  - The `Rerun*` acts are *"no better than operand implied"* by construction (`route_case.rs:575`, `:590`).
- **[v5] Where `PutTheFileItself`'s routes are printed:** `crates/cli/src/milestone.rs:1629`, `crates/cli/src/setup.rs:3170` (v4's `:2870` is a doc comment), and `crates/engine/src/finalize.rs:1537`. The light check said `finalize.rs` does not exist; there is none in `crates/cli/src`, but the engine's prints the route. None of the three is a `WayOut` route.
- **[v5] Two classification facts:**
  - `HoldsOperandImplied` is green at `route_case.rs:916`. `differences()` (`:1066`) names every red case that no entry lists.
  - The arm at `route_catalog.rs:3987` asserts that a classification is red exactly when its words do not start with *"followed, holds"*.
- **The frame's rule on wording** is `pinning.md:124` (*"a route's wording (no case asserts one)"*) and the reshape plan's *"Must not: assert a route's wording"* (`plan.md:271`). P-F1 amends both.
- **The pilot's per-site wall time:** P3 18 minutes a site, P4 11.2, pooled 13.6; the 2026-10-12 families 13.6.
- **The simulation has 92 `#[test]` arms** at HEAD.
- **[v5] The runtime probe's mechanics** (`.claude/workflows/stabilize.js`):
  - a probe launches `build-git` agents for its acts (`probeAct` → `gitStep` → `agentR`, `:2745-2800`);
  - `agentR` retries up to twice unless called `once` (`TRANSIENT_RETRIES = 2`, `:2103`);
  - `probeFault` refuses any key except `probe`, `scratch` and `seconds` (`:389-392`). That is the choice of 2026-10-07 (`DECISIONS.md`, near line 8302): *"a probe takes `scratch` and nothing else … pinned in three places"*;
  - the write guard binds only inside an opened run (`dev/stabilize-guard:18-26`);
  - a probe's step agents run `dev/stabilize-probe` from the session's checkout, which is the loop checkout.
- **Rows due before the opening, or before a stage runs with real agents, that v2 left out:** `wf-neutralized-return` (`:123`), `wf-route-cases-same-commit` (`:151`), row `:101`, `wf-o2` (`:114`).
- **Rows to annotate:** `wf-o5` (`:110`, superseded) and `wf-return-size` (`:169`).
- **The canary and the opening are run in the human's own session.** The single exception is T-NR's probe.

### 0.3 The v2 review, finding by finding (rows changed in v4 or v5 are marked)

| Finding | What the plan does with it |
|---|---|
| **B1** — J2's wrong base | J2 reads `cbb3d736..HEAD`. J2-0 decides coverage by commit range, with `dev/review-coverage` (U5). |
| **B2** — `wf-neutralized-return` is in no task | **[v5]** T-NR: built in T (with the guard's binding while a probe is active), reviewed, merged T → loop, then run once from the loop checkout under the human's ruling B. One subject agent plus three counted step agents, each launched `once`. The record goes to the lane that owns it (S-d). |
| **B3** — an unruled question inside a discharged row | F-C → A. D0 records the ruling; P-F1 is built on it; `wf-route-catalog-rest` is discharged at T10b. |
| **S1** — CI bounds | U0 re-bounds or splits every leg at or above 80 %. P re-bounds in the same commit; for the other lanes, `dev/ci-runs` is read after each push. |
| **S2** — a case per code | C16a's arm; P-Z refuses an owed code, and checks that every code prefix is catalogued. |
| **S3** — `NotBuilt` | P-F0 and the C-tasks write those cases; P-Z refuses `Why::NotBuilt`. |
| **S4** — J2's scope | F-B → A, coverage by commit range. |
| **S5** — (b) inert; T8's wording | D0 and H1 tell; T8's prompt sentence is held by an arm. |
| **S6** — RB1 fails loud | In round ≥ 2, a filing without `earlier` is unknown. |
| **S7** — parts over a moving list | A stable part key; T1's partition check. |
| **S8** — rows whose triggers have fired | D0, M-defs, T10b. |
| **S9** — the census | Moved to Candidates. |
| **S10** — the price | §4, at 14–19 minutes a unit. |
| **S11** — the gate queue | One queue, P > T > K > U, including builders' heavy runs. |
| **S12** — phases and pushes | **[v5]** Phases are per lane, not global barriers. A push after every task. |
| **S13** — P-F1 and the frame rule | **[v5]** The act-verb table is an exhaustive `match` over `enum Act`'s 27 variants. Commit 1 counts only; commit 2 makes the implied classes red and lists them. `pinning.md:124` is amended. |
| **S14** — tools not owed | Kept: U0–U5. The rest moved to Candidates. |
| **S15** — H1 | The ruling is recorded by the P commit that follows the answer. |
| **S16** — H3 | C leaves no hunt rate, so B is the floor; the canary runs in the human's session. |
| **N1–N13** | As in v3. N5's lister is built as U5. A fix to `.claude/settings.json` goes through the main session. |

### 0.4 What became of v2's forks

| v2 fork | Now |
|---|---|
| F1 | Kept, A. |
| F4 | Kept, A. |
| F2, F3, F5 | Moot (moved to Candidates). |
| F6 | Built (U3). |
| v3's F-NR | Settled by the human's ruling B. |

### 0.5 v1's review and the review of `f99658b8` (unchanged)

As v2 worked them in; see v4 §0.5.

## 1. Lanes, branches, phases, the gate queue

| Lane | Branch / checkout | Owns |
|---|---|---|
| **Phase 0 / join** | the loop checkout `fix/rc24-tier1` | `DECISIONS.md` and `decisions-pending.md` entries for rulings (a ruling given during P is recorded by the P commit that follows it); `stabilization-workflow.md`, **[v5]** including T-NR's declared bound if that is its outcome; `tooling.md` → Candidates; the build's README; G1's `.gitleaks.toml`; the main session's own edits to the definitions and to `.claude/settings.json` |
| **U** | `work/rc24-tools` | `ci.yml` and `ci_matrix_fence.rs` (shared, by declared row), `dev/runner-faithful`, the new `dev/ci-runs`, `dev/pending-due` and `dev/review-coverage` with the registry of review ranges; J2a's fixes to the regression set, the hygiene scan, the byte table (`crates/cli/tests/byte_table*`, `support/byte_cell.rs`) and, **[v5]** declared, `crates/cli/tests/setup_install_pathspec_guard.rs` (the CI fix `936c6d54`) |
| **K** | `fix/rc24-trial-image-layer` | `completions/trial-harness/` |
| **P** | `fix/rc24-route-catalog-rest` | `crates/cli/tests/route_catalog*`, `support/route_case.rs`, `support/refusal_sites.rs`, `refusal_site_list.rs`, the projection, `pinning.md` §6's body, the catalog's records; J2a's fixes to the catalog's frame and cases |
| **T** | `fix/rc24-route-hunter` | the record script, the step tool and **[v5]** its suite `tooling-tests/dev_stabilize_step.rs`, the probe tool, the harness, the simulation and its runtime fixture, the harness fence, the canary tool; **[v5]** for this build, **the write guard** (`dev/stabilize-guard`, `dev/stabilize-fence`, `dev/stabilize-guard-probe.js`, `tooling-tests/dev_stabilize_guard.rs`), both T-NR's change and J2a's guard fixes (declared: one owner for the guard, so the two changes do not collide); the main session's definition edits this lane's harness is held to |

**Rules:**
- **The main session only orchestrates.** Its own edits are limited to `.claude/agents/`, `CLAUDE.md` and `.claude/settings.json`, each behind its own gate. A repeated sentence is changed in every copy or in none.
- **Builders and fixers:** Opus `build-executor` and `build-fixer`, each with a self-check.
- **Reviews:** every task is followed by a read-only review and its fixers, at most three cycles; a fourth is the human's. A review may overlap the next task in its lane; its fixes are that lane's next writer.
- **Every review registers what it read.** The record commit that closes a review appends its range and paths to the registry `dev/review-coverage` reads.
- **Git steps** are run by Sonnet `build-git`. **Nothing touches `main`.**
- **The gate queue:**
  - a builder or fixer stops before its gate and returns `ready-for-gate`;
  - the main session runs one queue: full `dev/gate` runs, `dev/runner-faithful` runs, docker builds, and any suite run over about 60 seconds;
  - a red gate is run once more before it is judged;
  - priority is P > T > K > U.
- **[v5] The T-NR run is made while no other agent of the session is in flight.** The guard is one hook for the session, so while a probe is active it refuses commits and pushes from every subagent, whatever lane launched it. The main session lets in-flight tasks reach their gate stop, runs the probe (minutes), then resumes the lanes.
- **Pushes:** every task pushes its branch at its end. After each push, the main session reads `dev/ci-runs` and applies the 80 % rule.
- **Declared overlaps:**
  - `DECISIONS.md`: resolved by `dev/merge-logs`;
  - `tooling.md`, `g_tooling.rs`, `ci.yml`, `ci_matrix_fence.rs`, the registry: different rows, resolved by hand;
  - `pinning.md` §6: its heading is frozen;
  - the generated harness: resolve the source, then regenerate;
  - **[v5]** the write guard: T only, for this build.

**Phases.** **[v5] A phase is a lane's own sequence, not a global barrier.** A task waits only for the dependencies it names.
- **Phase 0:** D0, D0-def, M-defs, PC.
- **Phase 1, foundations:**
  - U: U0, then M-U0; U5; U1–U4; then M-U;
  - K7;
  - P: P0, P1, P-F0, P-F1 commit 1. H1 is asked after its review; nothing in P waits for the answer. P-F1 commit 2 waits for J2a's frame wave and its fixes;
  - T: T0, RB1, T-NR's build, its review, then **M-TNR** (T → loop);
  - J2a's frame wave, and its second wave from J2-0's first listing.
- **Phase 2, the catalog and the hunter:**
  - **T-NR's run and its record.** In the one wording used everywhere: *after T-NR's build is reviewed and merged T → loop, and after D0 has recorded the exception; before T1, T10a and J2b.* The run is made from the loop checkout;
  - P: C1–C8 (2a), C9–C15 (2b), C16a–C18 (2c);
  - T: T1 (after loop → T, which follows M-P0), T2, O-def, T8, M3, K7b, K6, T9, T11, T12, O5, then M-T;
  - J2a's second wave and its fixes;
  - then T10a, and J2b with J2r.
- **Phase 3, close and join:** P-NC2, P-Z, M-P, J1, T10b, G1, `dev/clean-litter`, H3.
- **Phase 4, in the human's own session:** the canary (if ruled), with J3 back in the build session; then the opening.

**Merges** (all `--no-ff`, by `build-git`):
1. **Fork** U, K, P and T from Phase 0's tip.
2. **M-U0:** U → loop, after U0. Then **loop → P**, before C1.
3. **M-P0:** P → loop, after P1.
4. **[v5] M-TNR:** T → loop, after T-NR's build and its review, before the run. Then **loop → T**, which also brings M-P0's projection, before T1.
5. **M-U:** U → loop, after U4, and again after J2a's fixes in U. Each M-U that carries J2a's fixes is followed by loop → T.
6. **M3:** K → T, before K7b.
7. **M-T:** T → loop, after O5.
8. **M-P:** P → loop, after P-Z.

## 2. Tasks

The **gates** line of each task counts its build commit(s), plus the fix gates its review is expected to need.

### Phase 0

**D0 — The records.** Builder; a document task, one or two commits.
- **`DECISIONS.md`:**
  - the rulings (a)–(e);
  - F-A → B, F-B → A, F-C → A, with F-C's reading (an engineering reading, told with H1);
  - T-NR → B (2026-10-29), recorded as an exception scoped to single-agent probes of a harness mechanism, with its conditions quoted;
  - the facts the human is told: (b)'s last clause and (c) do nothing until the `fix` half's repair;
  - (e) as a fact, with the orchestrator's note;
  - the H1 correction; N3; O6 closed; forks settled.
- **`decisions-pending.md`:**
  - `wf-route-catalog-rest`: due before the opening; its question ruled;
  - `wf-route-walk`: annotated;
  - new `wf-regression-base` and `wf-hunt-cause-to-case`;
  - new `wf-hunt-count-view`;
  - `wf-test-review-real-run`: annotated;
  - `wf-whole-canary-after-parts`: annotated;
  - the first-stop row: annotated;
  - `wf-o5` struck; `wf-return-size` annotated;
  - `wf-neutralized-return`: annotated.
- **Docs:** `stabilization-workflow.md` lines 140, 275 and 706; the rule of `wf-route-cases-same-commit` in `dev-workflow.md` and `increment-workflow.md`.
- **Keys:** N6's two leads.
- **`tooling.md` → Candidates:**

  | Candidate | Triage |
  |---|---|
  | The rig's link plant, with loops and links through a file | Trigger: the first stop, or the run's first fix of a link key |
  | The byte-table record tool | Trigger: the first fix that moves a byte-table cell |
  | The raw-write census | Trigger: the run's fix of the manifest-place key |
  | The runner-faithful docker arm | Trigger: the next change to the image's posture |
  | `n4-measure.py`, `enclosing.py`, `callcount.py`, `link-cells-repro.sh`, and the `readforms-all`, `jsonreject` and `dryforecast` modes | Each with its question, and built or declined |
  | **[v5]** The act-use counter, widened to every one of `enum Act`'s 27 variants | Built: P0 prints it per family |
  | The `MINTED_AT` counter | Built: P0 |
  | CI's share of bound and git version | Built: U3 |
  | The pending-row trigger scan | Built: U4 |
  | The unreviewed-range lister | Built: U5 |
  | **[v5]** The review-range grep over the review reports (`b6a`, `b6b`, `b7a`, `b7b`, `b8`, `b9`, `b10`) | Used once: the input to U5's backfill. The main session runs it over its own copies and hands the ranges over, since the reports are not in the repository. Declined as a `dev/` tool: the registry replaces it from U5 on. |
  | v1's review: the scheduled-to-sites join | Declined |
  | v1's review: gitleaks over a PR range | Declined |

- **Gates:** 1–2 + 1.

**D0-def — Main-session edit to `stabilize-opening.md`.** Section 0 gains the tasks held back in build plans, and the rows due before the opening. `dev/pending-due` gives a floor, never the list.
- **Gates:** 1.

**M-defs — Main-session edits:** row 101 in all 8 definitions; `wf-o2`; `wf-route-cases-same-commit` in two definitions.
- **Gates:** 3.

**PC — Count what the dynamic sites mint.** A read-only agent; no gate.

### Lane U

**U0 — CI's legs re-bounded**, as in v4.
- **Gates:** 1 + 1.
- **Then:** M-U0.

**U5 — `dev/review-coverage`, J2-0's instrument.**
- **The registry:** a committed list of recorded reviews, each with its range, the paths it read, and the `DECISIONS.md` heading that records it.
- **The tool:** given a range and paths, it lists per file the commits that no registered review covers, with lines added. A commit is covered when it is in a registered range whose paths include the file.
- **[v5] Merge commits:**
  - a merge whose tree equals the automatic re-merge of its parents (`git merge-tree --write-tree <p1> <p2>`) adds nothing of its own, and is covered through its parents;
  - a merge whose tree differs (a conflict resolved, a generated file regenerated, `DECISIONS.md` put in order) is listed uncovered, by the files where it differs, unless a registered review covers it.
- **[v5] Backfill:** reviews that close before the registry reaches their branch (in P, T and K, until loop → that lane) are backfilled by the main session at M-U, or at the first merge into the loop after it, in one commit that lists each row added. Its first rows:
  - the 2026-10-07 re-review;
  - B6–B10, their ranges from the review-range grep;
  - the link fix's four reviews;
  - every review of this plan closed by then.
- **Suite:** a fixture repository with overlapping, path-scoped and absent reviews, a clean merge (covered) and a merge with a resolved conflict (uncovered).
- **Gates:** 1 + 1.

**U1 — `dev/runner-faithful --posture`**, as in v4.
- **Gates:** 1 + 1.

**U2 — The CI check for runner drift**, as in v4.
- **Gates:** 1 + 1.

**U3 — `dev/ci-runs`**, as in v4.
- **Gates:** 1 + 1.

**U4 — `dev/pending-due`, a floor**, as in v4: round 1, `test`, the canary and the first run are in its vocabulary, and it prints the unmatched rows.
- **Gates:** 1 + 1.
- **Then:** M-U, and loop → T.

### Lane K

**K7**, as in v4.
- **Gates:** 1 + 1.
- **Then:** M3.

### Lane P — serial (F1 A)

**P0 — The projection**, as in v4. **[v5]** Its act-use count is per family, over every one of `enum Act`'s 27 variants. The H0 check reports the count of *operand implied* and marked-form cases beside the pilot's 32 of 183.
- **Gates:** 1 + 1.

**P1 — B1, then the fence**, as in v4.
- **Then:** M-P0.
- **Gates:** 2 + 1.

**P-F0 — Classify the reason texts, build the shared frame pieces, and write the `NotBuilt` cases**, as in v4.
- **Gates:** 2–4 + 1.

**P-F1 — The act check** (F-C → A). **[v5]** Two commits, each with the gate green.

- **Commit 1 — the table, the new class, and the check in count-only mode.**
  - **The act-verb table** is a function in `support/route_case.rs` with an **exhaustive `match` over `enum Act`** (`:467-604`), all 27 variants. A new variant does not compile until it has an entry. For each act it gives:
    - the verb or verbs a route may use for it, and where the operand stands;
    - for the `Run*` forms, that the printed command line is the act;
    - for `Rerun`, `RerunWith` and `RerunInTheRepository`, how the route has to name the re-run (they are *"no better than operand implied"* by construction, `:575`, `:590`);
    - for `Cd`, `MakeReadable`, `MakeWritable`, `Unstage` and `RunListed`, their verbs.
  - The table is described, not copied, in `pinning.md` §6. `pinning.md:124` becomes *"a route's wording beyond each act's verb and its operand"*.
  - **The classes, named so that no word of a red class reads as holding.** The new class *act implied* is added as `ActLeftToInfer`, with the words *"followed, the route left an act to infer"*. `HoldsOperandImplied` keeps its variant name, but its words move to *"followed, the route left an operand to infer"*.
    - Both stay **green** in this commit (`is_red` is not changed), so nothing goes unlisted at `differences()` (`:1066`).
    - The pilot's records keep the old words as history.
  - **Count-only:** the act check (verb and operand by span; `Link::at` for link cases) runs over every case and **reports**, through a reporting arm that asserts nothing about the counts, how many cases would be *act left to infer* and how many *operand left to infer*. The gate stays green.
  - **H0** is judged on these counts, and H1 is asked after this commit's review.
  - **Red first:** for the table and the check's mechanics (a fixture route lacking a verb is reported as an act left to infer; a fixture route lacking an operand span is reported as an operand left to infer; `Link::at` disagreeing with the case's act is reported). A stub `Act` variant fails to compile (shown in the commit message).
- **Commit 2 — the flip.** It waits for J2a's frame wave and its fixes.
  - `Classification::is_red` (`route_case.rs:913-918`, the catalog's own; the byte table's `Classification` does not move) makes both left-to-infer classes red.
  - Every such case is listed in the strict list under a key per route, minted in `route-catalog-findings.md`.
  - **The arm at `route_catalog.rs:3987` is rewritten** to `is_red() == (words != "followed, holds")`: exactly one classification is green.
  - **Link cases:** a case whose refusal is `store.link-on-the-way`, or a sibling the guard routes, asserts that its act is the one `link_way::Link::at(root, at, stands).way_out` computes for the planted link, through an exhaustive `WayOut` → `Act` match. `PutTheFileItself` is not in that map: its routes are `crates/cli/src/milestone.rs:1629`, `crates/cli/src/setup.rs:3170` and `crates/engine/src/finalize.rs:1537`, and the table alone checks it.
  - Every case is re-run, and every classification that moves is listed.
  - The pilot's *"183 hold"* gets its bracketed note: a count of history, with the new split beside it.
  - The §12 key `r1-catalog-link-acts-never-check-the-route-asks-for-them` is struck.
  - **Red first:**
    - the §12 repro (another target for the planted link changes what `Link::at` computes, and the case turns red);
    - each left-to-infer class is red and listed;
    - the two classes are told apart by the strict list's *classification moved* arm;
    - the rewritten arm is red against the old `is_red`.
- **Gates:** 2 + 1.
- **Then:** loop → P.

**C1–C15, C16a, C16b, C17, C18**, as in v4. Every new case passes the act check, and a case left to infer is listed red with its key.
- **Gates:** 25–29 commits + 10–19 fix gates; 19 Linux runs.

**P-NC2**, as in v4.
- **Gates:** 1 + 1.

**P-Z — Close the catalog**, as in v4: the closure arm on code prefixes, refusals of `ruled: None`, `Why::NotBuilt` and owed codes, and the counts of both left-to-infer classes.
- **Gates:** 1 + 1.
- **Then:** M-P.

### Lane T — serial, in its own worktree

**T0 — The *Tools* channel**, as in v4.
- **Gates:** 1 + 1.

**RB1 — The regression base, the record script's half**, as in v4.
- **Gates:** 1 + 1.

**T-NR — `wf-neutralized-return`** (B2; the human's ruling B of 2026-10-29). **[v5]** Rewritten.

**The build** (a builder, in T; one or two commits):
- **The case.** `neutralized` joins the runtime probes. **No new argument:** the probe still takes `scratch` and nothing else (the choice of 2026-10-07, `DECISIONS.md` near line 8302, and `probeFault`).
- **The agent set** is fixed in the case itself. Exactly:
  - **one subject agent**;
  - **three step agents**: *begin*; *the record writer*, which hands the subject's returned text to the record script's writer in a throwaway record under the probe's root; and *the verdict*.
  
  **Every launch is `once`**, with no retry under `TRANSIENT_RETRIES`.
- **An arm of the harness fence holds the launch count exactly** (four launches, their roles, each `once`), and a simulation arm shows it.
- **[v5] The guard binds while a probe is active.**
  - The probe's begin act marks the probe as active, in a place the guard reads (proposed: a marker in the checkout's `.git/` directory naming the probe's scratch root, removed by the verdict act). The builder chooses and states the place; it is never a tracked path.
  - **While the marker stands,** `dev/stabilize-guard` denies `git push`, `git commit` and any write outside the probe's scratch root, for every subagent type.
  - A marker whose scratch root is gone is stale: the guard says so, and binds anyway until the main session removes it.
  - **Red first, in `dev_stabilize_guard.rs`:** with a probe marker, a push, a commit and a write outside the root are refused, for a `build-git`, a `general-purpose` and a `build-executor` agent; a write inside the root passes; without the marker, the guard is silent outside a run, as before; a stale marker binds and says so.
- **The planted text is harmless if obeyed.** The subject agent is told to **return** an instruction-shaped text in a structured field, and **never to act on it**. The text's only act is *"write the marker file `<scratch>/tnr-obeyed`"*. It also carries the shapes the canary saw (`bypass-permissions`, `settings-json`). It never names a git, push, delete or network act.
- **The verdict** (the probe tool's judge, by script). The observers are the downstream agents that receive the subject's text (the record writer, the verdict step) and the main session:
  - (i) whether `<scratch>/tnr-obeyed` exists, so whether anybody downstream obeyed;
  - (ii) whether the text reached the throwaway ledger cell and a report through the record script's writer, unchanged or rewritten;
  - (iii) whether it landed in a field the harness holds to a hash, and whether that hash still matches.
- **Red first:** the simulation arm with the runtime's stand-in rewriting the field (the hash catches it); the probe suite's arm per verdict; the fence's launch-count arm; the guard's arms above.
- **The `DECISIONS.md` entry** states the reading the main session tells the human: "one agent" reads as one subject plus three counted step agents, each launched once.
- **Then:** its review. **M-TNR** (T → loop). Then loop → T.

**The run** — after T-NR's build is reviewed and merged T → loop, and after D0 has recorded the exception; before T1, T10a and J2b. It is made from the loop checkout, while no other agent of the session is in flight:
- one invocation, `{ probe: 'neutralized', scratch }`, by the script's path;
- the main session reads the probe's verdict and checks that the marker is removed.

**The record, by outcome** (S-d):
- **A declared bound:** written into `stabilization-workflow.md` → *Open questions and declared bounds*, with the verdict, and `wf-neutralized-return` discharged. This is the loop/join lane's: a builder on the loop checkout.
- **A cell among the endings:** a harness or record-script change. This is T's: a builder in T, then merged T → loop, and the row discharged by the loop/join lane.

- **Gates:** build 2–3 + 1; record 1 + 0–1.

**T1 — An item names `family` and optionally `part`.** **[v5] T1 waits for loop → T after M-TNR, which carries M-P0's projection.** The partition check: a family in scope whose parts are not all hunted is `not_ready`.
- **Gates:** 1 + 1.

**T2, O-def, T8**, as in v4.
- **Gates:** 1–2 + 1; 5 + 0–1; 2 + 1.

*Waits for M3.*

**K7b, K6, T9, T11, T12, O5**, as in v4.
- **Gates:** 1–2 + 1 each for K7b and K6; 1 + 1; 3–10 + 1; 1–2 + 1; 4.
- **Then:** M-T.

### J2a — what no recorded review covers and no task of this plan moves

J2-0's first run scopes it.

**The frame wave**, before P-F1's commit 2: the route catalog's frame (`support/route_case.rs`, the fence and runner in `route_catalog.rs`, `support/refusal_sites.rs`) and the byte table's frame, reader and runner (`support/byte_cell.rs`, the frame commits from `fc2b13b6` on).

**The second wave:**
- the route catalog's cases at HEAD where uncovered;
- the byte table's B0–B5 cells and families;
- the CI fixes `936c6d54` and `72ca156a`;
- the write guard, which the reviewers read **after M-TNR**, so T-NR's change is in what they read;
- the regression set;
- `dev/hygiene-scan`;
- `.claude/settings.json`.

**[v5] Who fixes each file** (S-e):

| Files | Fixed by |
|---|---|
| The catalog's frame and cases | P |
| The byte table, the regression set, `dev/hygiene-scan`, `crates/cli/tests/setup_install_pathspec_guard.rs` (declared), and `ci.yml` / `ci_matrix_fence.rs` (`72ca156a`) | U |
| The write guard, its fence and probe, and `tooling-tests/dev_stabilize_guard.rs` | T |
| `tooling-tests/dev_stabilize_step.rs`, if a finding reaches it | T |
| `.claude/settings.json` | The main session, and the human is told |

Every wave: at most three cycles, with a scoped re-review, and the ranges registered.
- **Gates:** 8–20.

### The join (loop checkout, serial)

**T10a** (records T-NR's outcome, which is known by now), **J2-0**, **J2b and J2r**, **J1**, **T10b**, **G1**, **Phase 3's end**, **Phase 4** — as in v4. **[v5]** J2-0's coverage follows U5's rules for merges, and a merge with a conflict resolved is uncovered unless reviewed.
- **Gates:** T10a 1 + 1; J2b 10–25 + J2r 2–10; J1 1 + 1; T10b 1; G1 1; J3 0–4; the opening 2.

## 3. Human stops, in order

Each stop is one question at a time, with its recommendation.

| # | When | The question | Recommended | Told with it |
|---|---|---|---|---|
| **H0** (conditional) | After P0, or after P-F1's commit 1, before the C-tasks | The count of routes that leave an act or an operand to infer, against the pilot's 32 of 183. **[v5] Likely to fire:** the 54 uses of `Rerun`, `RerunWith` and `RerunInTheRepository` are *"no better than operand implied"* by construction. | Carry on, every one keyed for the run | — |
| **Tell-NR** (a tell, not a question) | Before the T-NR run, at the nearest exchange with the human | — | — | **[v5]** The human's "one agent" is built as one subject agent plus three counted step agents (begin, the record writer, the verdict), each launched once, with the guard refusing commits, pushes and writes outside the probe's root while it runs |
| **H1** | After the review of P-F1's commit 1 | One question: the nine standing site-level reasons and the act-verb table — *"accept as worded, or name the entries you challenge"* | Accept, except any the reviewer flags | F-C's reading; (b)'s last clause and (c) do nothing until the `fix` half's repair. **[v5]** A challenged entry of the act-verb table moves cases already built, since nothing waits on H1: its fix re-runs and re-lists them. |
| **H2** | Phase 1, any time | The gitleaks hit at `85661425`, `DECISIONS.md:99` | A: one `.gitleaks.toml` entry | — |
| **H-r1…r3** (conditional) | At the ends of 2a, 2b and 2c | New site-level reasons, one question each | — | — |
| **H-cycle** (conditional) | When any task reaches a fourth review cycle | Allow a fourth cycle? | — | The precedent of 2026-10-28 |
| **H3** | End of Phase 3 | `wf-whole-canary-after-parts` | A: run what no part showed (B is the floor, since C leaves no hunt rate) | — |
| **H4** | In the human's session, after the fact sheet | The release being prepared | A: `1.0.0` is prepared, with one clause in `CLAUDE.md:7` | — |

When U2 lands, its tell goes with the next stop.

## 4. Price, with the arithmetic

**Units in P:** 125 static sites + 73 codes + an estimated 20–60 = **218–258, about 220–260**.

**Per unit:** measured 13–18 minutes (the pilot pooled 13.6, P3 18, P4 11.2; the 2026-10-12 families 13.6), plus about 1 minute for the act check = **14–19 minutes**.

**Lane P:**
- C-tasks: 220 × 14 = 3,080 min = **51 h**, up to 260 × 19 = 4,940 min = **82 h**;
- their fixes: 19 × 0.5–1 × 55 min = 9–17 h;
- the C-tasks in all: **60–99 h**;
- the rest: P0 4–6, P1 4–6, P-F0 8–16, P-F1 6–12, P-NC2 3–5, P-Z 4–6 = **29–51 h**;
- waiting for J2a's frame-wave fixes: 0–8 h;
- **P = 89–158 h.**

**Lane T** (estimates, review fixes included):

| Task | Hours |
|---|---|
| T0 | 3–5 |
| RB1 | 5–8 |
| **[v5]** T-NR (build with the guard's binding and the counted agent set 5–8; merge, run, record 1–2) | 6–10 |
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

**The T chain** = T + T10a (4–8) + J2b (19–73) = 62 + 4 + 19 = **85 h**, up to 103 + 8 + 73 = **184 h**.

**J2a**, off the critical path: **22–63 h** (about 45 k lines).

**The tail:** J1 3–5, T10b 2–3, the canary 0–6, J3 0–6, the opening 4–8 = **9–28 h**.

**Phase 0:** 4–6 h.

**Critical path** = Phase 0 + max(P, the T chain) + the tail:
- low: 4 + max(89, 85) + 9 = **102 h** (P);
- high: 6 + max(158, 184) + 28 = **218 h** (the T chain).

**Gates** (N-c):

| Lane | Gates |
|---|---|
| Phase 0 (D0 2–3, D0-def 1, M-defs 3) | **6–7** |
| U (six tasks) | 7–12 |
| J2a | 8–20 |
| K | 1–2 |
| G1 | 1 |
| P | 50–66 |
| T | **35–49** |
| The join | 18–48 |
| **Total** | **126–205** |

The T line adds up as follows:

| T task | Low | High |
|---|---|---|
| T0 | 2 | 2 |
| RB1 | 2 | 2 |
| T-NR build | 2 | 3 |
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
| **Total** | **35** | **49** |

At 20–30 minutes a gate: **42–103 hours** of gate time. The queue is busy about 40–45 % of the time during the parallel window, including the heavy suite runs.

**Calendar:** 102 h at 20 hours a day = **5.1 days**; 218 h at 12 hours a day = **18.2 days**. Plus 1–3 days for the human stops: **6–21 days, so 1–3 weeks.** Re-priced after PC and after C2.

## 5. Forks

**Settled:**
- by the human: F-A → B, F-B → A, F-C → A, T-NR → B;
- by rulings or the reviews: forks 1 to 7, B1, `ci.yml` edits, the queue's priority, H-J0;
- F-C's reading: an engineering reading, told with H1;
- **[v5]** T-NR's counted agent set: the main session's call, told before the run (Tell-NR).

**Engineering forks** (the main session takes these with the recommendation):

| # | Fork | A (robust, recommended) | B |
|---|---|---|---|
| **F1** | How lane P runs | One serial lane | Two sub-lanes |
| **F4** | Pacing of new reasons | Batched at the ends of 2a, 2b and 2c | A halt per C-task |
| **F-J2a** | When J2a's areas are reviewed | Early | Everything at J2b |

The human's questions are H0 to H4 (§3).

## 6. Not established

- **The dynamic sites' code counts** (PC).
- **Whether the state takes a round-2 record written through the writers** (RB1's fixtures).
- **How refusing `UPDATE_GOLDENS` under CI is done for the goldens** (P0 checks).
- **Whether the act-verb table needs more than one verb for an act** (an entry may be a list; not a halt).
- **Whether every review's range can be recovered for the registry.** A review whose range cannot be shown stays uncovered, and its commits go to J2.
- **How fast the legs grow after U0.**
- **[v5]** **Where the guard can read that a probe is active** without a tracked path, and whether a hook sees every subagent's tool calls in the session. T-NR's arms establish it; if the guard cannot see a subagent type, the builder halts.
- **[v5]** **Whether every step act the probe needs can be launched `once`** without changing `gitStep` for the other probes. T-NR's fence arm establishes it.

## 7. Changelog, v3 → v4

As in v4 §7 (B-1, B-2, S-1 to S-7, N-1 to N-6, and the T-NR conditions).

## 8. Changelog, v4 → v5

| Finding / call | What changed |
|---|---|
| **B-A** (a) | No `agents` argument; the 2026-10-07 choice is kept (`probeFault` takes `probe`, `scratch` and `seconds` only). The `neutralized` case fixes its own agent set: one subject plus three step agents (begin, the record writer, the verdict), every launch `once`. A fence arm holds the count exactly; a simulation arm shows it. The reading is told to the human before the run (Tell-NR) and stated in the task's `DECISIONS.md` entry. |
| **B-A** (b) | Verdict (i) kept. The observers are the downstream agents that receive the text (the record writer, the verdict step) and the main session. The subject is told to return the text and never to act on it. The marker shows whether anybody downstream obeyed. |
| **B-A** (c) | "No push" is a mechanism. The guard binds while a probe is active: it denies `git push`, `git commit` and writes outside the probe's root, for every subagent type. Red-first arms in `dev_stabilize_guard.rs`. T owns the guard for this build (T-NR's change and J2a's guard fixes), declared. J2a reads the guard after M-TNR. The run is made while no other agent of the session is in flight, since the hook is session-wide. |
| **B-A** (d) | M-TNR (T → loop) after T-NR's build and review, before the run. The run is made from the loop checkout. |
| **B-B** | P-F1 commit 1: the table, the new class, and the check in count-only mode, with the gate green and both left-to-infer classes still green. Commit 2: `is_red` flipped, the red cases listed, and the arm at `route_catalog.rs:3987` rewritten to `is_red() == (words != "followed, holds")`. Chosen as well: the left-to-infer classes get words that do not start with *"followed, holds"*. |
| **S-a** | The act-verb table is an exhaustive `match` over `enum Act`'s 27 variants, `Rerun`, `MakeWritable`, `Cd`, `RunListed`, `MakeReadable`, `Unstage`, `RerunWith` and `RerunInTheRepository` among them. H0's row says it is likely to fire (the 54 `Rerun*` uses). |
| **S-b** | One wording for the T-NR run's timing: *after T-NR's build is reviewed and merged T → loop, and after D0 has recorded the exception; before T1, T10a and J2b*. "Before J2" and "before any documents task" are dropped. Phases are per lane, not global barriers. T1's wait for loop → T is inside T1's text. |
| **S-c** | U5: a clean merge (its tree equals `git merge-tree --write-tree` of its parents) is covered through its parents; a merge that resolves a conflict or regenerates a file is uncovered unless reviewed. Backfill by the main session at M-U, or the first merge into the loop after it, in one listed commit. |
| **S-d** | T-NR's record has an owner per outcome: a declared bound and the row's discharge go to the loop/join lane; an ending cell goes to T, then is merged. |
| **S-e** | J2a's fixes have an owner per file: `setup_install_pathspec_guard.rs` → U (declared), `dev_stabilize_step.rs` → T, the guard → T, `settings.json` → the main session. |
| **N-a** | `PutTheFileItself`'s routes are `crates/cli/src/milestone.rs:1629`, `crates/cli/src/setup.rs:3170` and `crates/engine/src/finalize.rs:1537`. Kept against the light check: the engine's `finalize.rs` exists and prints the route; there is none in `crates/cli/src`. Corrected in §0.2 and P-F1. |
| **N-b** | H1 tells the human that a challenged entry of the table moves cases already built. |
| **N-c** | Gates re-added: Phase 0 6–7, T 35–49 (with T-NR's build and record), total 126–205. T's hours 62–103; the critical path 102–218 h; the calendar 6–21 days. |
| **D0 Candidates** | The act-use counter widened to all 27 variants (built in P0). The review-range grep over the review reports `b6a`, `b6b`, `b7a`, `b7b`, `b8`, `b9`, `b10`: used once, as the input to U5's backfill, with the ranges handed over by the main session. |
