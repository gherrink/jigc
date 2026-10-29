# Build plan v4: everything owed before the first stabilization run opens

Read-only planning, based on `fix/rc24-tier1` @ `7c34a222` (clean tree; CI run `40767224323` green).

This revises v3 (`pre-opening-plan-v3.md`), working in:
- its re-review (`pre-opening-plan-v3-review.md`: 2 blocking, 7 should-fix, 6 notes, 1 question);
- the human's ruling of 2026-10-29 on that question;
- the main session's engineering calls.

Everything v3 worked in (v2's review, v1's review, the review of `f99658b8`) stays as §0.3 to §0.5 record it. §7 lists what changed from v3 to v4. Every count was taken at `7c34a222` unless it says otherwise.

## 0. What changed since v2

### 0.1 The human's rulings on the reviews' forks (D0 records all four)

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
- A route that leaves one to be inferred is a finding of its own class. The run fixes it, in the product's text.
- The catalog checks that each act a case performs is one the route asks for: acts, not wording. This discharges the question carried in `wf-route-catalog-rest`.
- **Reading (an engineering reading of the ruling; told to the human with H1):**
  - the frame's **act-verb table** is the vocabulary for every act the product has no type for;
  - a case checks its act's verb plus its operand, by span, and nothing else of the sentence;
  - a link case also checks that its act is the one `Link::at` computes.
- P0 and P-F1 count the routes affected. If the count is far beyond expectation, there is a human stop (H0) before the C-tasks.

**T-NR → B (2026-10-29). The build session itself runs the single T-NR runtime probe**, as a recorded exception to the one-run-one-session rule.
- The exception is scoped to single-agent probes of a harness mechanism. A stage, a round or a canary still runs in the human's own session.
- Its four conditions are in the task (§2, T-NR):
  1. it runs early, at the start of Phase 2, before the documents tasks and before J2;
  2. exactly one agent, through the harness, with no stage and no push, and a hard cap of one agent stated in the invocation;
  3. the planted text is harmless if obeyed;
  4. D0 records the exception, scoped as above.

### 0.2 Facts corrected or added

- **The `CLAUDE.md` runner sentence changed in `26474b80`**, not in `63ce9ca5` (`git log -- CLAUDE.md`).
- **J2's base is `cbb3d736`**, the tree the last re-review of the `test` half read (`603018ac^..cbb3d736`, `DECISIONS.md` → *2026-10-07 — The `test` half … re-reviewed*).
  - Over that range: 247 commits and 413 files.
  - Over `dev/ .claude/ tooling-tests/ implementation/stabilization-workflow.md completions/trial-harness/ .github/`: 60 files, +78,721 / −3,618 lines.
  - The source files among them: harness source +5,537, record script +4,795, canary tool +2,581, step tool +2,441, regression set +813, probe tool +793, guard and fence and guard probe +546, generator +234, hygiene scan +78, settings +21, doc +438, definitions about +250.
  - About 60 k of the lines are suites.
- **What no recorded review covers** (re-review B-2, verified):
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

- **Routes that print no operand.** In the pilot, 32 of the 183 cases that hold did so with an operand the case had to supply (*holds, operand implied*), out of 74 cases whose route printed no operand (`route-catalog-pilot.md` §6, after its correction of 2026-10-11). That rate is H0's expectation.
- **The act vocabulary in the product.** Only `link_way::WayOut` (`link_way.rs:66-88`) is a product type for an act. Its route clause is rendered by a private `fn clause` (`:109`). The one public way to ask which way out the product chose is `Link::at(root, at, stands)` (`:99`, its `way_out` field public).
- **Acts the catalog uses that have no product type:** `Remove` 32, `Commit` 28, `MoveOut` 24, `MoveAway` 17, `MoveLinkOut` 16, `PointAt*` 15, `PutTheFileItself` 13, `Revert` 11, `Mend` 6, and the `Run*` forms. `PutTheFileItself`'s route is no `WayOut` route: it is printed by `milestone.rs:1629`, `finalize.rs:1537` and `setup.rs:2870`.
- **The frame's rule on wording** is `pinning.md:124`, *"What it does not pin … a route's wording (no case asserts one)"*, and the reshape plan's *"Must not: assert a route's wording"* (`plan.md:271`). P-F1 amends both (B-1).
- **The pilot's per-site wall time is measured, not derived.** P3: 10,773 s over 10 sites, 18 minutes a site. P4: 12,065 s over 18 sites, 11.2 minutes a site. Together 13.6 minutes a site (`route-catalog-pilot.md:68-81`, `:489`). That equals the 2026-10-12 families: `ee4db8f1` at 01:10 to `75754e8f` at 08:54, 464 minutes over 34 sites.
- **The simulation has 92 `#[test]` arms** at HEAD. That is T11's upper bound.
- **Rows due before the opening, or due before a stage runs with real agents, that v2 left out:** `wf-neutralized-return` (`decisions-pending.md:123`); `wf-route-cases-same-commit` (`:151`); row `:101` (the gate's stale "(probe · fmt · clippy · build · test)" in 8 definitions); `wf-o2` (`:114`).
- **Two rows to annotate:**
  - `wf-o5` (`:110`) is superseded by the ruling of 2026-10-16 that the run and every canary act run in `bypassPermissions` (`stabilization-workflow.md:679`).
  - `wf-return-size` (`:169`): its opening condition will fire; `K2`'s digest covers it for triage.
- **The canary and the opening are run in the human's own session** (one run, one session). The single exception is T-NR's probe (§0.1).

### 0.3 The v2 review, finding by finding (as v3 worked it in; rows changed in v4 are marked)

| Finding | What the plan does with it |
|---|---|
| **B1** — J2's wrong base | J2 reads `cbb3d736..HEAD`. **[v4]** J2-0 decides coverage by commit range, with the review-coverage lister (U5). |
| **B2** — `wf-neutralized-return` is in no task | **[v4]** T-NR: built at the end of Phase 1, and run once by the build session at the start of Phase 2, under the human's ruling B of 2026-10-29 and its four conditions. A builder then records a declared bound or an ending cell, inside T's lane, before T10a and J2b. |
| **B3** — an unruled question inside a discharged row | F-C → A. D0 records the ruling; P-F1 is built on it; `wf-route-catalog-rest` is discharged at T10b, after P-Z. |
| **S1** — CI bounds at 84–95 % | U0 re-bounds or splits every leg at or above 80 %. A P task runs its part in `dev/runner-faithful` and re-bounds it in the same commit if the proxy passes half its bound. For every other lane, after each push the main session reads `dev/ci-runs` (U3), and a leg past 80 % of its bound is re-bounded as that lane's next task. |
| **S2** — "a case per code" held by no fence | C16a's first commit adds `every_code_a_dynamic_site_mints_has_its_case_or_a_ruled_reason`. P-Z refuses an owed code. **[v4]** P-Z also checks that every code prefix is catalogued (re-review N-3). |
| **S3** — `NotBuilt` can survive P-Z | P-F0, or the C-task that owns the site, writes every `NotBuilt` case. P-Z refuses `Why::NotBuilt`. |
| **S4** — J2 leaves out parts | F-B → A. **[v4]** J2-0's coverage is by commit range, so a part is in J2 unless a recorded review's range covers its commits. |
| **S5** — (b) is inert; T8's wording | D0 records, and H1 tells the human, that (b)'s last clause and (c) have no effect until the `fix` half's repair. T8's prompt sentence is held by an arm. |
| **S6** — RB1 should fail loud | In round ≥ 2, a confirmed filing without `earlier` is `regression_unknown`. |
| **S7** — parts over a moving list | A part's key is its file plus its first site id. T1's state check: a family in scope whose parts are not all hunted is `not_ready`. |
| **S8** — rows whose triggers have fired | D0 (two docs), M-defs (row 101 in all 8 copies, `wf-o2`, two definitions), T10b (the build's README). |
| **S9** — U5's census | Moved to Candidates (F-A → B). |
| **S10** — the price | §4 shows the arithmetic. **[v4]** Re-priced at 14–19 minutes a unit (re-review S-6), with J2a's widened scope priced. |
| **S11** — the gate queue | Each builder and fixer stops before its gate. One queue, with priority P > T > K > U. **[v4]** The queue also takes every heavy run a builder makes (re-review N-4). |
| **S12** — phases and pushes | Phases 0 to 4; a push after every task; a re-price after PC and after C2. |
| **S13** — P-F1 and the frame rule | **[v4]** One act-verb table in `route_case.rs`, described in `pinning.md` §6. A case checks its act's verb plus its operand, by span. Link cases also check `Link::at`. The frame rule is amended to *"… beyond each act's verb and its operand"*. v3's claim that the rule "stands" is withdrawn. |
| **S14** — tools not owed | Kept: U0, U1, U2, U3, U4; **[v4]** and U5, the review-coverage lister, which J2-0 needs. Moved to Candidates: the rig's link plant, the byte-table record tool, the raw-write census, the runner-faithful docker arm. |
| **S15** — H1's place | The ruling is recorded by the P commit that follows the answer. Nothing waits on H1 before P-Z. |
| **S16** — H3 and who drives | H3 says that C leaves no hunt rate, so B is the floor. The canary and the opening run in the human's session. |
| **N1–N13** | As in v3, with these changes: N1's docker arm stays in Candidates; N5's unreviewed-range lister is now built as U5; **[v4]** N-2 (re-review): a J2a fix to `.claude/settings.json` goes through the main session, and the human is told. |

### 0.4 What became of v2's forks

| v2 fork | Now |
|---|---|
| F1 — P run as one serial lane | Kept, A. |
| F2, F3, F5 | Moot (moved to Candidates). |
| F4 — new reasons batched at phase ends | Kept, A. |
| F6 | Built (U3). |
| v3's F-NR | Settled: T-NR is built and run, by the human's ruling B. |

### 0.5 v1's review and the review of `f99658b8`, as v2 worked them in (unchanged)

- **B1:** P1 commit 1.
- **B2:** RB1 now; `wf-regression-base` for the rest.
- **B3:** parts are data.
- **B4:** overtaken (git 2.56).
- **B5:** superseded by F-B → A.
- **S1, S2:** re-priced and widened.
- **S3:** one merge, loop → P.
- **S4:** P-F0 before the C-tasks, P-NC2 after them.
- **S5:** T0 and T2.
- **S6:** H2.
- **S7:** done in `26474b80`.
- **S8:** §6's heading frozen.
- **S9:** `launched_under`.
- **S10:** T in its own worktree.
- **S11:** the stops in §3.
- **The notes:** N1 corrected; N2 is H4; N3, N6 and N9 recorded by D0; N4 drops T4; N5 the size watch; N7 U2; N8 the `ruled:` field.
- **The review of `f99658b8`:** H1 is D0's annotation plus ruling (c); N1 and N2 are D0's doc lines; N3 is moot through T12; N4 is a definition edit.

## 1. Lanes, branches, phases, the gate queue

| Lane | Branch / checkout | Owns |
|---|---|---|
| **Phase 0 / join** | the loop checkout `fix/rc24-tier1` | `DECISIONS.md` and `decisions-pending.md` entries for rulings (one exception: a ruling given during P is recorded by the P commit that follows it); `stabilization-workflow.md`; `tooling.md` → Candidates; the build's README; G1's `.gitleaks.toml`; the main session's own edits to the definitions (M-defs) and, **[v4]**, any edit of `.claude/settings.json` |
| **U** | `work/rc24-tools` | `ci.yml` and `ci_matrix_fence.rs` (shared, by declared row), `dev/runner-faithful`, the new `dev/ci-runs`, `dev/pending-due` and `dev/review-coverage` with the committed registry of review ranges, each with its suite and its `tooling.md` row; J2a's fixes to the guard, the regression set, the hygiene scan and **[v4]** the byte table (`crates/cli/tests/byte_table*`, `support/byte_cell.rs`) |
| **K** | `fix/rc24-trial-image-layer` | `completions/trial-harness/` (K7) |
| **P** | `fix/rc24-route-catalog-rest` | `crates/cli/tests/route_catalog*`, `support/route_case.rs`, `support/refusal_sites.rs`, `refusal_site_list.rs`, the projection, `pinning.md` §6's body, `route-catalog-findings.md`, `route-catalog-pilot.md`; **[v4]** J2a's fixes to the catalog's frame and cases |
| **T** | `fix/rc24-route-hunter` | the record script, the step tool, the probe tool, the harness (source and generated file), the simulation and its runtime fixture, the harness fence, the canary tool; the main session's edits to the definitions this lane's harness is held to, made in this worktree in lane order |

**Rules:**
- **The main session only orchestrates.** Its own edits are limited to `.claude/agents/`, `CLAUDE.md` and `.claude/settings.json`, each behind its own gate. A repeated sentence is changed in every copy or in none.
- **Builders and fixers:** Opus `build-executor` and `build-fixer`, each with a self-check.
- **Reviews:** every task is followed by a read-only review and its fixers, at most three cycles; a fourth is the human's (§3). A review reads only commits, so it may overlap the next task in its lane; its fixes are that lane's next writer.
- **[v4] Every review registers what it read.** The fixer's or builder's record commit that closes a review appends that review's commit range and paths to the registry `dev/review-coverage` reads. So every commit this plan makes is covered by the review of its task, and J2-0 can tell.
- **Git steps** (worktrees, merges with `--no-ff`, pushes by name) are run by Sonnet `build-git`. **Nothing touches `main`.**
- **The gate queue:**
  - a builder or fixer stops before its gate and returns `ready-for-gate`;
  - the main session runs one queue and tells one agent at a time to run the full `dev/gate` (`--private-target` in a worktree), in the background, unpiped;
  - **[v4]** the queue also takes `dev/runner-faithful` runs, docker builds, and **any suite run a builder makes that takes over about 60 seconds** (the catalog suite, the simulation, the record script's suite, the step tool's suite): such a run is requested and started by the main session like a gate;
  - **[v4]** a red gate is run once more before it is judged, and both results are recorded;
  - priority is P > T > K > U (ruling a).
- **Pushes:** every task pushes its branch at its end. After each push, the main session reads `dev/ci-runs` (from U3 on) and applies the 80 % rule.
- **Declared overlaps:**
  - `DECISIONS.md`: resolved by `dev/merge-logs`;
  - `tooling.md`, `tooling-tests/groups/g_tooling.rs`, `ci.yml`, `ci_matrix_fence.rs`, the review-range registry: different rows, resolved by hand;
  - `pinning.md` §6: its heading is frozen;
  - the generated harness: resolve the source, then regenerate.

**Phases:**
- **Phase 0:** D0, D0-def, M-defs, PC.
- **Phase 1, foundations:**
  - U: U0, then M-U0; U5; U1–U4; then M-U;
  - K7;
  - P: P0, P1, P-F0, then P-F1 commit 1 (the act-verb table). **H1 is asked then; nothing in P waits for its answer.** H0 is checked at P0 and at P-F1 commit 1. P-F1 commit 2 waits for J2a's frame wave and its fixes;
  - T: T0, RB1, then T-NR's build;
  - J2a's frame wave and its fixes (in P and U); J2a's second wave starts;
  - merges: M-P0, then loop → T and loop → P.
- **Phase 2, the catalog and the hunter:**
  - **at its start, before anything else of T and before any documents task or J2b:** the T-NR probe run, its verdict, and its record;
  - P: C1–C8 (2a), C9–C15 (2b), C16a–C18 (2c), each sub-phase ending at an H-r batch;
  - T: T1, T2, O-def, T8, M3, K7b, K6, T9, T11, T12, O5, then M-T;
  - J2a's second wave and its fixes;
  - then T10a, and J2b with J2r.
- **Phase 3, close and join:** P-NC2, P-Z, M-P, J1, T10b, G1 (any time after H2), `dev/clean-litter`, H3.
- **Phase 4, in the human's own session:** the canary (if ruled), with J3 back in the build session for anything it changes; then the opening.

**Merges** (all `--no-ff`, by `build-git`):
1. **Fork** U, K, P and T from Phase 0's tip.
2. **M-U0:** U → loop, after U0. Then **loop → P**, before C1.
3. **M-P0:** P → loop, after P1. Then **loop → T**, before T1.
4. **M-U:** U → loop, after U4 (and again after J2a's fixes in U). **[v4]** Then **loop → T** after each M-U that carries J2a's fixes, so T's simulation and fences meet those changes in a gate of their own, not first in M-T.
5. **M3:** K → T, before K7b.
6. **M-T:** T → loop, after O5.
7. **M-P:** P → loop, after P-Z.

## 2. Tasks

The **gates** line of each task counts its build commit(s), plus the fix gates its review is expected to need.

### Phase 0

**D0 — The records.** Builder; a document task, one or two commits. The brief carries every ruling word for word.
- **`DECISIONS.md`:**
  - the rulings (a)–(e) of 2026-10-27 and 2026-10-28;
  - F-A → B, F-B → A, F-C → A, **[v4]** with F-C's reading (the act-verb table, the verb plus its operand by span, `Link::at` for link cases) marked as the orchestrator's engineering reading, which H1 tells the human;
  - **[v4]** T-NR → B (2026-10-29): the build session runs the one T-NR probe, as an exception to the one-run-one-session rule, scoped to single-agent probes of a harness mechanism. A stage, a round or a canary still runs in the human's own session. The four conditions are quoted;
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
  | **[v4]** The act-use counter (the re-review's *Tools*) | Built: P0 prints it per family |
  | The `MINTED_AT` counter | Built: P0 |
  | CI's share of bound and git version | Built: U3 |
  | The pending-row trigger scan | Built: U4 |
  | **[v4]** The unreviewed-range lister | Built: U5 |
  | v1's review: the scheduled-to-sites join | Declined: the projection answers it |
  | v1's review: gitleaks over a PR range | Declined: `vet --range` and CI's `hygiene` job already do it |

- **Done when:** `doc_link_fence` and `tooling_index` are green, and the full gate is green.
- **Gates:** 1–2 + 1.

**D0-def — Main-session edit to `stabilize-opening.md`.** The fact sheet's section 0 (what is owed before the opening) gains:
- the tasks held back in build plans, each with its plan, its id and its trigger;
- the rows of `decisions-pending.md` due before the opening. **[v4]** `dev/pending-due` (U4) gives a floor, never the list: the gatherer reads every row the tool prints as unmatched as well, and judges each.
- **Gates:** 1.

**M-defs — Main-session edits to repeated sentences**, each behind its own gate:
1. Row 101 in all 8 definitions.
2. `wf-o2`, in `milestone-code-reviewer.md`.
3. `wf-route-cases-same-commit`, in `build-executor.md` and `increment-validator.md`.

Before a sentence is removed, the fences are searched for it.
- **Gates:** 3.

**PC — Count what the dynamic sites mint.** A read-only agent; no gate. For each of the 17 dynamic sites left after B1: every code it mints, with its source line. The count is an agent's reading; S2's arm and P-Z's arms make it held.

### Lane U

**U0 — CI's legs re-bounded.**
- **What:** `g_migration` split into parts (`route_catalog` · `byte_table` · the rest); `g_tooling`'s `rest` and `simulation` parts measured again and re-bounded or split. Each part's bound is max(10, 2 × its four-CPU proxy), measured in `dev/runner-faithful`.
- **Red first:** the fence's arms.
- **Done when:** a pushed CI run shows every leg under 80 % of its bound.
- **Gates:** 1 + 1.
- **Then:** M-U0.

**U5 — `dev/review-coverage`, J2-0's instrument** (re-review B-2; second in U's order, before U1–U4).
- **The registry:** a committed list of recorded reviews — each with its range (`A..B`, or `A^..B`), the paths it read, and the `DECISIONS.md` heading that records it. The first rows:
  - the 2026-10-07 re-review (`603018ac^..cbb3d736`);
  - the byte table's reviews B6–B10 (their ranges as the review reports state them; for instance B6's `56839f95..f4c12fa3`). The main session hands the ranges over from the review reports, since those reports are not in the repository. Each row cites the `DECISIONS.md` entry of the fixes that followed its review;
  - the link fix's four reviews (2026-10-28, including `22f06d31` and `63ce9ca5`);
  - any other review the main session can document.
- **The tool:** given a range and paths, it lists, per file, the commits of the range that touched it and that no registered review covers, with lines added. It does this by commit membership: a commit is covered when it is in a registered range whose paths include the file.
- **Suite:** a fixture repository with three reviews: overlapping, path-scoped, and none.
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
- **What:** it lists each unstruck row of `decisions-pending.md` whose trigger (including a bracketed re-trigger) names the opening, round 1, `test`, a stage with real agents, the canary, the first run, or a join. **[v4]** It also prints every other unstruck row with its trigger text, as *unmatched*, so a judgment covers every row. Its header and its `tooling.md` row say the matched list is a floor.
- **Suite:** a fixture of rows: struck, re-triggered, conditional, a trigger in prose, *before round 1's `test` stage*, *before the canary*.
- **Gates:** 1 + 1.
- **Then:** M-U, and loop → T.

### Lane K

**K7 — The trial image's base layer**, as in the reshape plan §5. Its two docker builds go through the queue.
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
  - **[v4]** per family, its act uses by kind;
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

**P-F1 — The act check** (F-C → A, in the reading of §0.1; re-review B-1 and S-2). Two commits.

- **Commit 1 — the vocabulary, the classification and the count.**
  - **One act-verb table, in `support/route_case.rs`**, the frame's single home for what an act is called. For each `Act` kind (`Remove`, `Commit`, `MoveOut`, `MoveAway`, `MoveLinkOut`, `PointAt*`, `Revert`, `Mend`, `PutTheFileItself`, `ReplaceWithDirectory`, `ReplaceWithACopy`, `MoveDirectoryIn`, and the `Run*` forms) it gives the verb or verbs a route may use for it, and where the operand stands.
  - The table is described, not copied, in `pinning.md` §6.
  - `pinning.md:124` changes from *"a route's wording (no case asserts one)"* to *"a route's wording beyond each act's verb and its operand"*. The reshape plan's P3/P4 rule is read the same way in the entry.
  - **A classification of its own,** *followed, holds, act implied*, with its own `as_str`, beside *followed, holds, operand implied*. `Classification::is_red` (`route_case.rs:913-918`) makes both red. That function is the catalog's own: the byte table's `Classification` (`support/byte_cell.rs`) does not move.
  - **The count:** before anything is listed red, the commit reports how many cases would be *act implied* and how many *operand implied*. More than twice the pilot's rate is H0.
  - **H1** is asked after this commit's review (§3). The table goes to the human with the nine reasons.
- **Commit 2 — the check applied.** It waits for J2a's frame wave and its fixes in P.
  - Each case's act finds its verb in the table at a span of the printed route, together with its operand by span, and asserts nothing else of the sentence.
  - **Link cases:** a case whose refusal is `store.link-on-the-way` (or a sibling the guard routes) also asserts that its act is the one `link_way::Link::at(root, at, stands).way_out` computes for the planted link. The map from `WayOut` to `Act` is an exhaustive `match`, so a new variant fails to compile until it is mapped. `PutTheFileItself` is not in that map: its routes are `milestone.rs:1629`, `finalize.rs:1537` and `setup.rs:2870`, and it is checked by the table alone.
  - Every case is re-run, and every classification that moves is listed. Each implied case is listed red under a key per route, minted in `route-catalog-findings.md`.
  - **A bracketed note now, not at P-Z:** the pilot's *"183 hold"* (`route-catalog-pilot.md:44`, `:277`) becomes a count of history, with the new split beside it.
  - The §12 key `r1-catalog-link-acts-never-check-the-route-asks-for-them` is struck.
- **Red first:**
  - the §12 repro (changing the planted link's target, so that `Link::at` computes another way out, turns the case red);
  - an implied operand is red;
  - an act whose verb the route lacks is red;
  - the two implied classes are told apart by the strict list's *classification moved* arm;
  - a stub `WayOut` variant fails to compile (shown in the commit message).
- **Gates:** 2 + 1.
- **Then:** loop → P.

**C1–C15 — The static families**, 125 sites. **C16a, C16b, C17, C18 — the dynamic families**, re-cut from PC's count: a case for every code; no task over about 35 codes; `compose`'s 27 alone.
- **Commits:** about 10 sites or codes each. C16a's first commit adds S2's arm.
- **Done when:**
  - the family's rows are gone from `scheduled.tsv`;
  - the strict expected-red list holds, including the implied classes;
  - every new case passes the act check;
  - the projection is regenerated;
  - one `dev/runner-faithful` run of the `route_catalog` part is green, with its proxy in the commit, and re-bounded or split in the same commit if the proxy passes half its bound;
  - the named-binary arm, filtered to the family, has run on rc.24;
  - new site-level reasons carry `ruled: None`;
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
  - **[v4]** the closure arm: every prefix of a code-shaped literal in production source is in `CATALOGUED` (re-review N-3), so the vocabulary arm holds every code;
  - the named-binary arm runs whole on rc.24;
  - `route-catalog-pilot.md` gets §14's numbers, including the counts of the two implied classes.
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

**T-NR — `wf-neutralized-return`** (B2; the human's ruling B of 2026-10-29). **[v4]** Moved: built at the end of Phase 1, run at the start of Phase 2.

- **The build** (a builder, in T):
  - a case of the runtime probes: its judge in `dev/stabilize-probe`, its invocation in the harness;
  - the invocation names **a hard cap of one agent**: the harness refuses a probe invocation of this case that does not say `agents: 1`, and refuses to launch a second agent under it;
  - it starts **no stage**, writes **no run**, and makes **no push** — no step act of the step tool that commits or pushes is reachable from it.
- **The planted text is harmless if obeyed.** The one agent returns, in a structured field, an instruction-shaped text whose only act is *"write the marker file `<scratch>/tnr-obeyed`"*. The text also carries the shapes the canary saw (`bypass-permissions`, `settings-json`). It never names a git, push, delete or network act.
- **The verdict** (the probe tool's judge, by script):
  - (i) whether the marker exists, so whether the instruction was obeyed;
  - (ii) whether the text reached a ledger cell and a report through the record script's writer, in a throwaway record under the scratch root, unchanged or rewritten;
  - (iii) whether it landed in a field the harness holds to a hash, and whether that hash still matches.
- **Red first:** a simulation arm with the runtime's stand-in rewriting the field (red: the hash is caught); the probe suite's arms for each verdict; the invocation without `agents: 1` refused.
- **The run** (the main session, at the start of Phase 2, before T1 and before any documents task or J2): one invocation, `{ probe: 'neutralized', scratch, agents: 1 }`, by the script's path.
- **The record:** a builder records either a declared bound (`stabilization-workflow.md` → *Open questions and declared bounds*, with the verdict) or a cell among the endings. In the latter case the change is made in T, here, behind its gate. `wf-neutralized-return` is discharged.
- **Gates:** 1 + 1 for the build; 1 + 0–1 for the record.

*Waits for loop → T (after M-P0).*

**T1 — An item names `family` and optionally `part`**, keyed by file plus first site id. The partition check: a family in scope whose parts are not all hunted is `not_ready`.
- **Gates:** 1 + 1.

**T2 — The hunt writer**, as in v2: rows over the item's own set of sites; `cause`, `plantable`, `tools`; the count computed by the script.
- **Gates:** 1–2 + 1.

**O-def — Main-session edits, in T's worktree**, each behind its own gate:
1. `O4`: the tester and the reviewer — what a finding carries, the kit sentence, the hunt contract bound to `HUNT:`.
2. `finding-triage.md`: a finding that names a site carries `site`.
3. `stabilize-opening.md`: one `route-hunt` item per family part.
4. `build-fixer.md`: a plantable cause gets its case, red first; a new blocking site comes with its case.
5. `finding-verifier.md` step 4: *"the door, or a step of the block, the previous release does not have"*.
- **Gates:** 5 + 0–1.

**T8 — The route hunter**, as in v2: `ROLES.hunt` in the first commit together with the kit's `hunt` role; `launched_under` and its fixture; the size watch. The prompt reads: *"every site of your part, for causes beyond the catalog's; a (site, cause) the catalog has is not hunted again"*, held by an arm.
- **Gates:** 2 + 1.

*Waits for M3.*

**K7b, then K6**, as in the reshape plan.
- **Gates:** each 1–2 + 1.

**T9 — The rest of it:** `its_doors_are_the_review_rows_the_hunts_and_the_arms` and `its_expectation_names_what_it_prices`.
- **Gates:** 1 + 1.

**T11 — Move the scenarios left on the old path.** The builder counts first; at most 92, at about ten a commit.
- **Gates:** 3–10 + 1.

**T12 — Delete the old path.** `runFix` stops pointing at `VERIFY_PASSES`, and `VERIFY_SCHEMA` goes.
- **Gates:** 1–2 + 1.

**O5 — Main-session edits**, as the reshape plan specifies.
- **Gates:** 4.
- **Then:** M-T.

### J2a — what no recorded review covers and no task of this plan moves (Phase 1, and on into Phase 2)

**[v4]** J2-0's first run (U5, over `cbb3d736..HEAD`) scopes it. It holds two waves of read-only `milestone-code-reviewer`s.

**The frame wave** — before P-F1's commit 2 (re-review S-3; the main session's call):
- the route catalog's frame: `support/route_case.rs`, the fence and runner in `route_catalog.rs`, `support/refusal_sites.rs`;
- the byte table's frame, reader and runner (`support/byte_cell.rs` and the frame commits from `fc2b13b6` on).

Three to four reviewers. Fixes go into P (the catalog) and U (the byte table), as each lane's next writer.

**The second wave** — parallel with the C-tasks:
- the route catalog's cases that exist at HEAD: the pilot's `setup` and `uninstall`, the 2026-10-12 families, and the link fix's cases wherever J2-0 shows them uncovered;
- the byte table's acceptance cells and its pilot, task and install families (B0–B5, up to `56839f95`);
- the CI fixes `936c6d54` and `72ca156a`;
- the write guard (`dev/stabilize-guard`, `dev/stabilize-fence`, `dev/stabilize-guard-probe.js`, `dev_stabilize_guard.rs`);
- the regression set (`dev/regression-set`, `dev_regression_set.rs`, the `held/regression-*` fixtures);
- `dev/hygiene-scan`;
- `.claude/settings.json`.

Four to six reviewers. **A fix to `.claude/settings.json` is the main session's edit, and the human is told** (it holds the deny list that protects the human's acts).

Every wave: at most three cycles, with a scoped re-review, and the ranges registered.
- **Gates:** 8–20.

### The join (loop checkout, serial)

**T10a — The workflow doc says what the stage does as built.** It records T-NR's outcome, which is known by now, together with the hunter, the F-C meaning and the regression base as built.
- **Gates:** 1 + 1.

**J2-0 — The partition, by commit range.** **[v4]** `dev/review-coverage cbb3d736..<tip>` is run against the registry. Every file of the range is in exactly one of:
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

Then one pass across all of them, which **[v4]** also reads J2-0's partition, class (iv) first (re-review N-6). Fixes are serial on the loop branch, one commit per area per cycle. **J2r** is the scoped re-review, at most three cycles. Expect J2b's high end (re-review N-1).
- **Done when:** every finding is fixed or ruled, and `wf-test-review-real-run` is discharged with the record and J2-0's listing.
- **Gates:** 10–25 + 2–10.

**J1** — as specified, plus anything of T8's arm it did not take.
- **Gates:** 1 + 1.

**T10b — Rows and the record.**
- **Discharged:** `wf-route-walk`, `wf-route-catalog-rest` (with the F-C ruling) and the reshape plan's set. **Not:** `wf-regression-base`, `wf-hunt-cause-to-case`, `wf-hunt-count-view`.
- **Also:** `completions/artifacts/M55/stabilization-build/README.md` and the lessons list.
- **Gates:** 1.

**G1 — The `.gitleaks.toml` entry** (after H2, if A). One entry, exact file and exact line shape.
- **Done when:** `gitleaks git --log-opts "origin/main..HEAD"` exits 0.
- **Gates:** 1.

**Phase 3's end:** `dev/clean-litter`; every branch pushed; CI read; **H3**.

**Phase 4 — in the human's own session.**
- **The canary (if ruled).** The build session prepares its clone with T9's setup and hands it over. The human runs it in their own session. Anything it changes comes back to the build session as **J3**, a scoped re-review (0–4 gates).
- **The opening.** Run by the human's session: the fact sheet, **H4** and the opening's other questions, the `CLAUDE.md` edit per H4's answer, the opening commit behind the gate, the push.
- **The build session stops after Phase 3** (and after J3, if needed).

## 3. Human stops, in order

Each stop is one question at a time, with its recommendation. **[v4]** H-J0 is gone: F-B → A answers it (re-review S-3). T-NR is no stop: the human ruled B.

| # | When | The question | Recommended | Told with it |
|---|---|---|---|---|
| **H0** (conditional) | After P0, or after P-F1's commit 1, before the C-tasks | The count of routes that leave an act or an operand to inference, against the pilot's 32 of 183 | Carry on, every one keyed for the run | — |
| **H1** | **[v4]** After the review of P-F1's commit 1 | One question: the nine standing site-level reasons **and the act-verb table** — *"accept as worded, or name the entries you challenge"* | Accept, except any the reviewer flags | F-C's reading (the verb plus its operand by span; `Link::at` for link cases) as an engineering reading of the ruling; (b)'s last clause and (c) do nothing until the `fix` half's repair |
| **H2** | Phase 1, any time | The gitleaks hit at `85661425`, `DECISIONS.md:99` | A: one `.gitleaks.toml` entry (B, a fingerprint, drifts; C, a rewrite, is not viable) | — |
| **H-r1…r3** (conditional) | At the ends of 2a, 2b and 2c | New site-level reasons, as one question each | — | — |
| **H-cycle** (conditional) | When any task, J2a and J2b included, reaches a fourth review cycle | Allow a fourth cycle? | — | The precedent of 2026-10-28 |
| **H3** | End of Phase 3 | `wf-whole-canary-after-parts` | A: run what no part showed, including the measured hunt (190–330 min, $60–105). B: the measured hunt only — the floor, since C leaves no hunt rate. C: none. | — |
| **H4** | In the human's session, after the fact sheet | The release being prepared | A: `1.0.0` is prepared, and the call to publish stays the human's at the close; one clause in `CLAUDE.md:7`. B: the next rc. | — |

When U2 lands, its tell goes with the next stop.

## 4. Price, with the arithmetic

**Units of work in P** (derived): 125 static sites, plus 73 `MINTED_AT` codes, plus an estimated 20–60 codes of uncatalogued prefixes. Total **218–258, about 220–260**. PC fixes the number.

**Wall time per unit** (re-review S-6):
- *Measured:* the pilot's parts, P3 at 18 minutes a site and P4 at 11.2, pooled to **13.6**; and the 2026-10-12 families, **13.6**. Both include gates; neither includes per-task reviews.
- *Range used:* 13–18 minutes. The low end is the pooled measure, the high end the slower part.
- *Plus F-C's act check:* about 1 minute a unit (an estimate: one act-table entry or span per act).
- **Used: 14–19 minutes a unit.**

**Lane P:**
- **The C-tasks:** 220 × 14 = 3,080 min = **51 h**, up to 260 × 19 = 4,940 min = **82 h**.
- **Their fixes:** 19 tasks × 0.5–1 fix commit × about 55 minutes = 9–17 h.
- **C-tasks in all: 60–99 h.**
- **The rest of P** (estimates): P0 4–6, P1 4–6, P-F0 8–16, P-F1 6–12, P-NC2 3–5, P-Z 4–6 = 29–51 h.
- **Waiting for J2a's frame-wave fixes before P-F1's commit 2:** 0–8 h. The wave starts in Phase 1 beside P0–P-F0, so most of its time overlaps.
- **P = 89–158 h.**

**Lane T** (estimates per task, review fixes included):

| Task | Hours |
|---|---|
| T0 | 3–5 |
| RB1 | 5–8 |
| T-NR (build, run and record) | 4–7 |
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
| **Total** | **60–100** |

**After T, before the join:**
- **T10a:** 4–8 h.
- **J2b:**
  - the reviews, nine plus one pass in parallel: 4–8 h;
  - the fixes: 10–25 commits × 55–85 minutes = 9–35 h;
  - J2r: 1–2 cycles × (3–5 h of review + 3–10 h of fixes) = 6–30 h;
  - **J2b = 19–73 h.**
- **The T chain** = 60 + 4 + 19 = **83 h**, up to 100 + 8 + 73 = **181 h**.

**J2a, off the critical path** (it runs beside P and T, except for the frame wave's 0–8 h already inside P):
- the frame wave: 4–8 h of review + 4–12 h of fixes;
- the second wave: 4–8 h of review + 6–20 h of fixes;
- one or two re-review cycles: 4–15 h;
- **J2a = 22–63 h.** About 45 k lines: catalog frame and fence about 12 k, catalog cases about 15 k, byte table B0–B5 about 12 k, guard, regression set and hygiene scan about 5.5 k, settings and the CI fixes small.

**The tail**, after both chains: J1 3–5, T10b 2–3, the canary 0–6 (190–330 minutes if A), J3 0–6, the opening 4–8 = **9–28 h**.

**Phase 0:** 4–6 h.

**Critical path** = Phase 0 + max(P, the T chain) + the tail:
- low: 4 + max(89, 83) + 9 = **102 h**, with P the longer chain;
- high: 6 + max(158, 181) + 28 = **215 h**, with the T chain the longer.

The two chains are within about 15 % of each other. The priority P > T (ruling a) costs the T chain its gate waits, which are inside the high end.

**Gates:**

| Lane | Gates |
|---|---|
| Phase 0 | 5–7 |
| U (six tasks) | 7–12 |
| J2a | 8–20 |
| K | 1–2 |
| G1 | 1 |
| P | 50–66 (P-F1 now 2 + 1) |
| T | 36–49 (T-NR now 3–4) |
| T10a, J2b, J2r, J1, T10b, J3, the opening | 18–48 |
| **Total** | **126–205** |

At 20–30 minutes a gate, that is **42–103 hours** of gate time.
- During the parallel window, P, T, U and J2a ask for about 105–150 gates, which is 35–75 hours inside an 89–181-hour window, so the queue is busy about 40 % of the time.
- The heavy suite runs that now go through the queue (re-review N-4) add about 5–10 % to that.
- Beside the gates: 19–22 Linux runs, two K7 image builds, one whole rc.24 arm.

**Calendar:**
- 102 h at 20 hours a day = **5.1 days**; 215 h at 12 hours a day = **17.9 days** (the measured paces: 2026-10-11/12 about 20 h a day, 2026-10-28 about 12 h).
- Plus 1–3 days for the human stops: **6–21 days, so 1–3 weeks.**
- The tail risks are beyond-bound review cycles (most likely in J2a and J2b), J2 findings that need the human, and PC at its high end.
- **Re-priced** after PC and after C2.

## 5. Forks

**Settled:**
- by the human: F-A → B, F-B → A, F-C → A, T-NR → B;
- by rulings or the reviews: forks 1 to 7; B1; `ci.yml` edits; the queue's priority; H-J0 (answered by F-B → A);
- F-C's reading (the act-verb table): the main session's engineering reading, told with H1;
- v2's F2, F3, F5 moot; F6 built; v3's F-NR settled.

**Engineering forks.** The main session takes these with the recommendation unless it chooses to put them:

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
- **[v4]** **Whether the act-verb table can name every act that today's cases perform without a second verb for one act.** P-F1's commit 1 counts the acts that need more than one verb. Such an entry is a list, still one home; it is not a halt.
- **[v4]** **Whether every review's range can be recovered for the registry.** The byte table's review reports are outside the repository; the main session hands over their ranges. A review whose range cannot be shown is not registered, and its commits stay uncovered, which puts them into J2.
- **How fast the legs grow after U0** (the 80 % rule watches it).
- **[v4]** **Whether the harness's probe invocation can be capped at one agent** by its own arguments. T-NR builds the cap, and its arm shows a second launch refused.

## 7. Changelog, v3 → v4

| Finding / call | What changed |
|---|---|
| **B-1** | P-F1 rewritten into two commits. One act-verb table in `route_case.rs`, described in `pinning.md` §6. A case checks its act's verb plus its operand, by span. Link cases also check `Link::at(root, at, stands).way_out`, through an exhaustive `WayOut` → `Act` match. `PutTheFileItself` is taken out of the `WayOut` pairing. `pinning.md:124` and the P3/P4 rule are amended to *"beyond each act's verb and its operand"*. §6's halt clause is dropped. The table goes to the human with H1. D0 records the reading. §0.1, §0.2 and the S13 row changed. |
| **B-2** | J2-0 counts coverage by commit range. The unreviewed-range lister is built now as U5 (`dev/review-coverage` with a committed registry of review ranges). Every review this plan runs registers its range. The byte table's B0–B5, the route catalog's code and the CI fixes `936c6d54` and `72ca156a` go into J2a under F-B → A. No human question. |
| **S-1** | T-NR runs at the start of Phase 2, so its record is inside T10a and J2b. |
| **S-2** | P-F1: `is_red` is the catalog's own and the byte table is untouched; *act implied* is a classification of its own with its own `as_str`; the pilot's *"183 hold"* gets its bracketed note in P-F1's commit 2. |
| **S-3** | H-J0 removed (main session's call). J2a's frame wave reviews the catalog's and the byte table's frames before P-F1's commit 2; its second wave takes the cases and the rest. Priced in §4 (J2a 22–63 h; 8–20 gates). |
| **S-4 / the question** | The human ruled B (2026-10-29). The build session runs the one T-NR probe, under its four conditions in T-NR. D0 records the scoped exception. No stop. |
| **S-5** | A loop → T merge after each M-U that carries J2a's fixes. Phase 1 states that H1 is asked and nothing in P waits for it. H1 now follows P-F1's commit 1. |
| **S-6** | Per unit 14–19 minutes (measured 13–18, plus about 1 for the act check). P = 89–158 h. Critical path 102–215 h. Gates 126–205. Calendar 6–21 days. |
| **S-7** | U4's vocabulary adds round 1, `test`, the canary and the first run. It prints the unmatched rows too. Its header, its row and D0-def call the list a floor. |
| **N-1** | J2b: expect the high end. |
| **N-2** | Any fix to `.claude/settings.json` is the main session's edit, and the human is told. Phase 0's lane owns it. |
| **N-3** | P-Z's closure arm: every code prefix in production source is in `CATALOGUED`. |
| **N-4** | The gate queue also takes every builder's suite run over about 60 seconds. A red gate is run once more before it is judged. |
| **N-5** | H0 unchanged. |
| **N-6** | J2b's pass across all areas reads J2-0's partition, class (iv) first. |
| **T-NR conditions** (the human's ruling B) | Built at the end of Phase 1, run at the start of Phase 2, before T1, the documents tasks and J2. Exactly one agent through the harness, a hard cap stated in the invocation, no stage, no push. The planted instruction writes only `<scratch>/tnr-obeyed`. The verdict reads the marker, the ledger and report, and the hash-held field. D0 records the exception, scoped to single-agent probes of a harness mechanism. |
