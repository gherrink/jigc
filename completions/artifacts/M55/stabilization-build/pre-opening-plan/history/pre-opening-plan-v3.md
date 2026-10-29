# Build plan v3: everything owed before the first stabilization run opens

Read-only planning, based on `fix/rc24-tier1` @ `7c34a222` (clean tree; CI run `40767224323` green).

This revises v2 (`pre-opening-plan-v2.md`), working in:
- its review (`pre-opening-plan-v2-review.md`: 3 blocking, 16 should-fix, 13 notes, 3 forks);
- the human's rulings of 2026-10-28 on that review's three forks (F-A → B, F-B → A, F-C → A).

v1's review and the review of `f99658b8` stay worked in as v2 had them (§0.5). Every count below was taken at `7c34a222` unless it says otherwise.

## 0. What changed since v2

### 0.1 The human's rulings of 2026-10-28 on the review's forks (D0 records all three)

**F-A → B. Ruling B's two open siblings go to the opening ledger and are fixed by the run.**
- The two keys:
  - `r1-config-set-writes-through-a-link-at-the-manifests-own-place` (`byte-table-findings.md` §30);
  - `r1-uninstall-half-tears-down-over-a-hook-link-it-cannot-read` (`route-catalog-findings.md` §12).
- *Why:* the workflow exists so that known findings are fixed inside the run. Fixing everything before the opening is the ad-hoc loop it replaced.
- *Consequence for this plan:* only tools that a task before the opening, or one of the run's instruments, needs are kept. The rest move to `tooling.md` → Candidates, each with its trigger (S9, S14).

**F-B → A. J2 reviews everything the `test` stage consists of, over `cbb3d736..HEAD`, with nothing left out.**
- The parts that had reviews of their own count as reviewed and are not read again. These are the byte table, the route catalog, the link-on-the-way fix and the CI fixes. The plan points at each one's review record.
- J2 is re-priced (§4).

**F-C → A. A route works as printed only if its text names every act, and every operand, that following it needs.**
- A route that leaves an act or an operand to be inferred is a finding of its own class. The run fixes it, in the product's text.
- The catalog then checks that each act a case performs is one the route asks for. This is a check of acts, not of exact wording.
- This discharges the question carried in `wf-route-catalog-rest`. P-F1 is built on it.
- P0 and P-F1 count the routes affected. If the count is far beyond expectation, there is a human stop (H0) before the C-tasks, with the number.

### 0.2 Facts corrected or added

- **The `CLAUDE.md` runner sentence changed in `26474b80`**, not in `63ce9ca5` (`git log -- CLAUDE.md`).
- **J2's base is `cbb3d736`**, the tree the last re-review of the `test` half read (`603018ac^..cbb3d736`, `DECISIONS.md` → *2026-10-07 — The `test` half … re-reviewed*).
  - Over that range: 247 commits and 413 files.
  - Over `dev/ .claude/ tooling-tests/ implementation/stabilization-workflow.md completions/trial-harness/ .github/`: 60 files, +78,721 / −3,618 lines.
  - The source files among them: the harness source +5,537, record script +4,795, canary tool +2,581, step tool +2,441, regression set +813, probe tool +793, guard and fence and guard probe +546, generator +234, hygiene scan +78, settings +21, doc +438, definitions about +250.
  - About 60 k of the lines are suites.
- **CI's legs against their bounds** on run `40767224323`:

  | Leg | Took | Bound | Share |
  |---|---|---|---|
  | `g_tooling, rest` | 629 s | 11 min | 95 % |
  | `g_tooling, simulation` | 642 s | 12 min | 89 % |
  | `g_migration` | 504 s | 10 min | 84 % |
  | `g_tooling, record` | 518 s | 14 min | 62 % |

- **Routes that print no operand.** In the pilot, 32 of the 183 cases that hold did so with an operand the case had to supply (*holds, operand implied*), out of 74 cases whose route printed no operand (`route-catalog-pilot.md` §6, after its correction of 2026-10-11). That rate is the "expectation" against which H0 is judged.
- **`link_way::WayOut` is a public enum** of the product's library (`crates/cli/src/lib.rs`: `pub mod link_way`; `link_way.rs:66`). Its route clauses are rendered by a private `fn clause`.
- **No recorded code review of the route catalog was found.** A grep of `DECISIONS.md` headings and bodies finds the plan's review (`reshape/plan-review.md`) and the pilot's record, but no review of the catalog's code. The byte table's families B6–B10 have recorded reviews (the entries "… after its review", 2026-10-18 to 2026-10-27). The link fix has four (2026-10-28). See H-J0.
- **The simulation has 92 `#[test]` arms** at HEAD (`tooling-tests/stabilize_simulation.rs`). That is T11's upper bound; the builder counts first.
- **Rows due before the opening, or due before a stage runs with real agents, that v2 left out:**
  - `wf-neutralized-return` (`decisions-pending.md:123`);
  - `wf-route-cases-same-commit` (`:151`; its trigger, the join, already happened);
  - row `:101`: the gate's stale "(probe · fmt · clippy · build · test)" in 8 definitions, three of them launched by a run;
  - `wf-o2` (`:114`).
- **Two rows to annotate:**
  - `wf-o5` (`:110`) is superseded by the human's ruling of 2026-10-16 that the run and every canary act run in `bypassPermissions` (`stabilization-workflow.md:679`).
  - `wf-return-size` (`:169`): its opening condition (*"far more than a hundred untriaged rows"*) will fire. The triage's return is a digest since `K2`, which covers it for triage; the row stays for the other roles.
- **The canary and the opening are run in the human's own session** (one run, one session). This build session prepares them, and stops before them.

### 0.3 The v2 review, finding by finding

| Finding | What v3 does with it |
|---|---|
| **B1** — J2's wrong base | J2 reads `cbb3d736..HEAD`. Its first step, **J2-0**, puts every file of the range in exactly one place: a J2 area, or a part counted reviewed (with its review record named). Re-priced in §4. |
| **B2** — `wf-neutralized-return` is in no task | **T-NR**: a runtime-probe case that sends an instruction-shaped text through a structured return, a ledger cell and a hash-held field. The main session invokes it once (a probe works on no run). A builder then records either a declared bound or an ending cell. Fork F-NR offers the declared-bound-only alternative. |
| **B3** — an unruled question inside a discharged row | Ruled by the human: F-C → A. D0 records the ruling and annotates the row. P-F1 is built on the ruling. The row is discharged only at T10b, after P-Z. |
| **S1** — CI bounds at 84–95 % | U0 re-bounds or splits every leg at or above 80 %: `g_tooling rest`, `g_tooling simulation`, `g_migration`. **A rule held through the build:** a P task runs its part in `dev/runner-faithful` and re-bounds it in the same commit if the proxy passes half its bound; for every other lane, after each push the main session reads `dev/ci-runs` (U3), and a leg past 80 % of its bound is re-bounded as that lane's next task. |
| **S2** — "a case per code" held by no fence | C16a's first commit adds `every_code_a_dynamic_site_mints_has_its_case_or_a_ruled_reason`. It names owed codes until P-Z, and refuses them from P-Z on. Once every family is in `CATALOGUED` (P-Z), the vocabulary arm holds the uncatalogued codes PC found. |
| **S3** — `NotBuilt` can survive P-Z | P-F0, or the C-task that owns the site, writes every `NotBuilt` case. P-Z's arm refuses `Why::NotBuilt`. |
| **S4** — J2 leaves out parts; it overrides a ruling | Ruled by the human: F-B → A. J2's areas cover the guard, the held checks, the canary tool, the probe tool, the generator, settings, the trial image and the doc. J2-0's partition proves nothing is left out. |
| **S5** — (b) is inert in this run; T8's wording | D0 records, and H1 tells the human, that (b)'s last clause has no effect until the `fix` half's repair, just as (c) has none. T8's prompt reads *"every site of your part, for causes beyond the catalog's; a (site, cause) the catalog has is not hunted again"*, held by an arm. |
| **S6** — RB1 should fail loud | In round ≥ 2, a confirmed filing without `earlier` is `regression_unknown`. Arm added. |
| **S7** — parts over a moving list | A part's key is its file plus its first site id. T1 adds a state check: in a round, the `route-hunt` items of every family in scope partition its parts, and `not_ready` names an uncovered part. The re-cut of parts in round 2 is noted on `wf-hunt-cause-to-case`. |
| **S8** — rows whose triggers have fired | Folded in: the repeated rule of `wf-route-cases-same-commit` (D0 for the two docs; M-defs for the two definitions); row 101 (M-defs, all 8 copies); `wf-o2` (M-defs); the build's README (T10b). |
| **S9** — U5 does not measure what is claimed | By F-A → B the claim is dropped, and the census moves to Candidates with its trigger: *the run's fix of the manifest-place key, as its class count*. The overlap with P is gone with it. |
| **S10** — the price does not reconcile | §4 shows every step of the arithmetic, labels what is measured and what is derived, and turns running hours into calendar days explicitly. |
| **S11** — no gate-queue mechanism, no priority | Every builder and fixer stops before its gate and returns `ready-for-gate`. The main session keeps one queue and tells one agent at a time to run its gate (the precedent is the 2026-10-27 CI brief). Priority is P > T > K > U, by ruling (a). The cost of that priority is stated in §4. |
| **S12** — phases undefined | Phases 0 to 4 are defined in §1. Every task pushes its branch at its end. The re-price happens after PC and after C2. |
| **S13** — P-F1 reverses a frame rule silently | Ruled by the human: F-C → A, a check of acts, not of wording. P-F1 pairs each act of the frame with the product's own act vocabulary: an exhaustive `match` on `WayOut` for the link acts, so a new variant fails to compile until it is mapped, and an act verb read by span for the others. It never asserts the rest of the sentence. The plan's line *"Must not: assert a route's wording"* stands. |
| **S14** — U3–U6 not owed before the opening | Applied, in the spirit of F-A. Kept: U0 (CI bounds), U1 (posture, as U2's one home), U2 (the drift check, fork 7 A), U3 `dev/ci-runs` (S1's rule needs it after every push), and U4, the pending-row trigger scan (the opening's section 0 needs it; the review's *Tools*). Moved to Candidates with triggers: the rig's link plant (with loops and links through a file, per S14), the byte-table record tool, the raw-write census, the docker arm of the runner-faithful suite. U merges at M-U0 and M-U, and nothing waits on H2: the gitleaks entry is G1, on the loop branch. |
| **S15** — H1's place | The ruling is recorded in whichever P commit follows the answer. That P builder writes the `DECISIONS.md` entry, an exception to Phase 0's ownership of rulings, stated in §1. Nothing waits on H1 before P-Z. |
| **S16** — H3's option C; who drives | H3 says that C leaves no measured hunt rate, so the opening could not price its `route-hunt` items (reshape plan §9, line 494). B is the floor. The canary and the opening run in the human's own session. |
| **N1** — U1's docker arm, wrong commit | That arm moves to Candidates. Its text there names the facts: red at `0bd8a5b5` and at `22f06d31`, green at `63ce9ca5`. |
| **N2** — `CLAUDE.md` changed in `26474b80` | Corrected (§0.2). |
| **N3** — B1 needs a negative control | P1 limits the exclusion to the bodies of `impl Finding` in `crates/engine/src/finding.rs`. Arm and mutant: a `Finding { … }` literal outside `impl Finding` is still a site. |
| **N4** — the byte-table print is committed | Noted in that tool's candidate row: F3 A is buildable when the tool is built. |
| **N5** — harvested tools missing from the triage | D0's candidates list adds each with its question and its trigger, built or declined: `n4-measure.py`, `enclosing.py`, `callcount.py`, `link-cells-repro.sh`, the `readforms-all`, `jsonreject` and `dryforecast` modes, the `MINTED_AT` counter (built into P0's projection), the unreviewed-range lister, the share-of-bound reading (built into U3), and the pending-row trigger scan (built: U4). D0's brief carries each one's *Tools* text from its report. |
| **N6** — two leads with no key | D0 keys them as leads in `route-catalog-findings.md` §12: `migrate.source-untrackable` against the link code, and the read-form advisory against the pinned contracts. The main session hands over their text; where no repro exists, the key says *not driven*. |
| **N7** — T11's gates | Up to 10 commits (92 arms at about ten a commit); the builder's count comes first. |
| **N8** — undeclared overlaps | `ci.yml` and `ci_matrix_fence.rs` are written by U and by any task that re-bounds a leg (S1). They are declared, and resolved by hand on different rows. U5 and the `pinning.md` §7 overlap went with the moves to Candidates. |
| **N9** — H1 and H-r as one question each | Each is put as one question: *"accept these as worded, or name the ones you challenge"*. |
| **N10** — (e) as recorded | D0 records the fact: the human allowed a fourth review cycle for the link fix. That it is a data point on the bound of three, worth putting at the first stop, is marked as the orchestrator's note. |
| **N11** — `wf-o5` and `wf-return-size` | D0 strikes `wf-o5` as superseded and annotates `wf-return-size` (§0.2). |
| **N12** — the drift tell | Told when U2 lands, with the next stop, not with H2. |
| **N13** — four worktrees, cold builds, disk | `dev/clean-litter` runs at the end of Phase 3. The main session checks free disk before each lane's first gate. |

### 0.4 What became of v2's forks

| v2 fork | Now |
|---|---|
| F1 — P run as one serial lane | Kept, A. |
| F2, F3, F5 | Moot: their tools moved to Candidates. |
| F4 — new reasons batched at phase ends | Kept, A. |
| F6 — build `dev/ci-runs` | Settled: built (U3), because S1's rule needs it. |

### 0.5 v1's review and the review of `f99658b8`, as v2 worked them in (unchanged unless 0.3 says so)

- **B1:** P1 commit 1.
- **B2:** RB1 now; `wf-regression-base` for the rest.
- **B3:** parts are data.
- **B4:** overtaken (git 2.56).
- **B5:** superseded by F-B → A.
- **S1, S2:** re-priced; S2 widened to S1 above.
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
| **Phase 0 / join** | the loop checkout `fix/rc24-tier1` | `DECISIONS.md` and `decisions-pending.md` entries for rulings (one exception: a ruling given during P is recorded by the P commit that follows it); `stabilization-workflow.md`; `tooling.md` → Candidates; the build's README; G1's `.gitleaks.toml`; the main session's own edits to the definitions (M-defs) |
| **U** | `work/rc24-tools` | `ci.yml` and `ci_matrix_fence.rs` (shared, by declared row; S1), `dev/runner-faithful`, the new `dev/ci-runs` and `dev/pending-due`, each with its suite and its `tooling.md` row; J2a's fixes |
| **K** | `fix/rc24-trial-image-layer` | `completions/trial-harness/` (K7) |
| **P** | `fix/rc24-route-catalog-rest` | `crates/cli/tests/route_catalog*`, `support/route_case.rs`, `support/refusal_sites.rs`, `refusal_site_list.rs`, the projection, `pinning.md` §6's body, `route-catalog-findings.md`, `route-catalog-pilot.md` |
| **T** | `fix/rc24-route-hunter` | the record script, the step tool, the probe tool, the harness (source and generated file), the simulation and its runtime fixture, the harness fence, the canary tool; the main session's edits to the definitions this lane's harness is held to, made in this worktree in lane order |

**Rules:**
- **The main session only orchestrates.** Its own edits are limited to `.claude/agents/` and `CLAUDE.md`, each behind its own gate, and a sentence repeated across several definitions is changed in every copy or in none (lesson 25).
- **Builders and fixers:** Opus `build-executor` and `build-fixer`, each with a self-check.
- **Reviews:** every task is followed by a read-only review and its fixers, at most three cycles; a fourth is the human's (§3). A review reads only commits, so it may overlap the next task in its lane; its fixes are that lane's next writer.
- **Git steps** (worktrees, merges with `--no-ff`, pushes by name) are run by Sonnet `build-git`. **Nothing touches `main`.**
- **The gate queue (S11):**
  - a builder or fixer stops before its gate and returns `ready-for-gate` with its tree's status;
  - the main session runs one queue and tells one agent at a time to run the full `dev/gate` (`--private-target` in a worktree), in the background, unpiped;
  - `dev/runner-faithful` runs and docker builds take the same queue;
  - priority is P > T > K > U (ruling a), and within a lane, first come first served.
- **Pushes:** every task pushes its branch at its end (S12). After each push, the main session reads `dev/ci-runs` (from U3 on; `gh run view` before that) and applies S1's 80 % rule.
- **Declared overlaps:**
  - `DECISIONS.md`: resolved by `dev/merge-logs`;
  - `tooling.md`, `tooling-tests/groups/g_tooling.rs`, `ci.yml`, `ci_matrix_fence.rs`: different rows, resolved by hand;
  - `pinning.md` §6: its heading is frozen;
  - the generated harness: resolve the source, then regenerate.

**Phases (S12):**
- **Phase 0:** D0, D0-def, M-defs, PC. On the loop checkout, serial.
- **Phase 1, foundations:**
  - U0 (then M-U0), U1–U4 (then M-U);
  - K7;
  - P0 → (H0?) → P1 → (H1) → P-F0 → P-F1 (with its H0 check);
  - T0, RB1;
  - J2a (the review of the frozen areas) and its fixes;
  - merges: M-P0, then loop → T and loop → P.
- **Phase 2, the catalog and the hunter:**
  - P: C1–C8 (2a), C9–C15 (2b), C16a–C18 (2c), each sub-phase ending at an H-r batch;
  - T: T1, T2, O-def, T8, M3, K7b, K6, T9, T11, T12, O5, T-NR, then M-T;
  - then T10a and J2b with J2r.
- **Phase 3, close and join:** P-NC2, P-Z, M-P, J1, T10b, G1 (any time after H2), the T-NR probe run and its record, `dev/clean-litter`, H3.
- **Phase 4, in the human's own session:** the canary (if ruled), with J3 back in the build session for anything it changes; then the opening.

**Merges** (all `--no-ff`, by `build-git`):
1. **Fork** U, K, P and T from Phase 0's tip.
2. **M-U0:** U → loop, after U0. Then **loop → P**, before C1.
3. **M-P0:** P → loop, after P1. Then **loop → T**, before T1.
4. **M-U:** U → loop, after U4 (and after J2a's fixes, if they come in time; otherwise a second U → loop merge).
5. **M3:** K → T, before K7b.
6. **M-T:** T → loop, after T-NR.
7. **M-P:** P → loop, after P-Z.

## 2. Tasks

The **gates** line of each task counts its build commit(s), plus the fix gates its review is expected to need.

### Phase 0

**D0 — The records.** Builder; a document task, one or two commits. The brief carries every ruling word for word.
- **`DECISIONS.md`:**
  - the rulings (a)–(e) of 2026-10-27 and 2026-10-28;
  - F-A → B, F-B → A, F-C → A;
  - the facts the human is told: (c) has no effect in round 1, and neither (c) nor (b)'s last clause does anything until the `fix` half's repair (B2 and S5);
  - (e) as a fact; the "data point at the first stop" as the orchestrator's note (N10);
  - the bracketed H1 correction on the 2026-10-27 *not comparable* entry;
  - N3 (part 4 of the staged canary ran);
  - O6 closed;
  - forks 1, 3, 5 and 7 settled, 6 moot.
- **`decisions-pending.md`:**
  - `wf-route-catalog-rest`: due **before the opening**; its carried question ruled (F-C → A); what it finds goes to the opening ledger;
  - `wf-route-walk`: annotated with (b), shape A;
  - new `wf-regression-base`: RB2, RB3, the round-2 scenarios and the derivation for the fix audit; due with the `fix` half's repair;
  - new `wf-hunt-cause-to-case`, with the round-2 re-cut of parts (S7); due with the `fix` half's repair;
  - new `wf-hunt-count-view`: due at the first stop;
  - `wf-test-review-real-run`: annotated with J2's scope (F-B → A);
  - `wf-whole-canary-after-parts`: annotated;
  - the first-stop row: annotated with (e), as the orchestrator's note;
  - `wf-o5` struck as superseded; `wf-return-size` annotated (N11).
- **Docs:**
  - `stabilization-workflow.md` lines 140, 275 and 706;
  - the repeated rule of `wf-route-cases-same-commit`, one sentence each in `dev-workflow.md` and `increment-workflow.md`.
- **Keys:** N6's two leads, in `route-catalog-findings.md` §12.
- **`tooling.md` → Candidates:**

  | Candidate | Triage |
  |---|---|
  | The rig's link plant, with loops and links through a file (asked for by five agents, and by the third re-review) | Trigger: the first stop, beside `wf-pre-driver-kit`, or the run's first fix of a link key |
  | The byte-table record tool (the print is committed: F3 A is buildable) | Trigger: the first fix that moves a byte-table cell |
  | The raw-write census | Trigger: the run's fix of `r1-config-set-writes-through-a-link-at-the-manifests-own-place`, as its class count |
  | The runner-faithful docker arm (red at `0bd8a5b5` and `22f06d31`, green at `63ce9ca5`) | Trigger: the next change to the image's posture |
  | The unreviewed-range lister | Trigger: the `fix` half's re-review |
  | `n4-measure.py`, `enclosing.py`, `callcount.py`, `link-cells-repro.sh`, and the `readforms-all`, `jsonreject` and `dryforecast` modes | Each with its question, and built or declined |
  | The `MINTED_AT` counter | Built: P0 |
  | CI's share of bound and git version | Built: U3 |
  | The pending-row trigger scan | Built: U4 |
  | v1's review: the scheduled-to-sites join | Declined: the projection answers it |
  | v1's review: gitleaks over a PR range | Declined: `vet --range` and CI's `hygiene` job already do it |

- **Done when:** `doc_link_fence` and `tooling_index` are green, and the full gate is green.
- **Gates:** 1–2 + 1.

**D0-def — Main-session edit to `stabilize-opening.md`.** The fact sheet's section 0 (what is owed before the opening) gains two things:
- **the tasks held back in build plans**: each with its plan, its id and its trigger;
- **the rows of `decisions-pending.md` due before the opening**, read first by running `dev/pending-due` (U4) and then judged.

Until U4 lands, the sentence names the tool as the first read. The edit binds only at the opening.
- **Gates:** 1.

**M-defs — Main-session edits to repeated sentences.** Each one changes every copy, and each is behind its own gate:
1. Row 101: the gate's stale "(probe · fmt · clippy · build · test)" corrected in all 8 definitions.
2. `wf-o2`: `milestone-code-reviewer.md` says the hash is asked of the tool and returned on a pass that drives nothing.
3. `wf-route-cases-same-commit`: one sentence each in `build-executor.md` and `increment-validator.md`.

Before a sentence is removed, the fences are searched for it.
- **Gates:** 3.

**PC — Count what the dynamic sites mint.** A read-only agent; no gate. For each of the 17 dynamic sites left after B1: every code it mints, under catalogued and uncatalogued prefixes, each with its source line.
- **Output:** the cut of the dynamic C-tasks, and the re-price after C2. The count is an agent's reading; S2's arm and P-Z's vocabulary arm make it held.

### Lane U

**U0 — CI's legs re-bounded.**
- **What:** `g_migration` split into parts (`route_catalog` · `byte_table` · the rest, which skips exactly those two). `g_tooling`'s `rest` and `simulation` parts are measured again and re-bounded, or split further. Each part's bound is max(10, 2 × its four-CPU proxy), measured in `dev/runner-faithful`. This follows the precedent of `72ca156a`.
- **Red first:** the fence's arms for the new rows.
- **Done when:** a pushed CI run shows every leg under 80 % of its bound.
- **Gates:** 1 + 1.
- **Then:** M-U0.

**U1 — `dev/runner-faithful` declares its posture in one place.**
- **What:** `--posture` prints the system config the image asserts (`safe.directory=*` and nothing else), and the image's build-time assertion reads the same declaration.
- **Arms** (a new `tooling-tests/dev_runner_faithful.rs`): `--posture` prints the declared posture; the Dockerfile's assertion is the declaration's; a usage error exits 2 before docker is asked.
- **Writes also:** its `tooling.md` row's *Held by* cell.
- **Gates:** 1 + 1.

**U2 — The CI check for runner drift** (fork 7 A).
- **What:** one step that prints `git --version` and `git config --system --list --show-origin`, and holds the config to `dev/runner-faithful --posture`. The version is printed and not held: the PPA's newest moves weekly.
- **Red first:** the fence's arm.
- **Told** with the next stop: the check turns CI red when GitHub changes the runner's git config (N12).
- **Gates:** 1 + 1.

**U3 — `dev/ci-runs`.**
- **What:** for a branch's last N runs, per job: conclusion, seconds, the job's bound read from `ci.yml`, the share of the bound, the runner image, and the git version from the job's log. `--compare A B` names the jobs that differ in conclusion or git.
- **Suite:** a stub `gh` on `PATH` serving fixtures, so no network is needed. The tool never prints an environment value.
- **Gates:** 1 + 1.

**U4 — `dev/pending-due`.**
- **What:** it lists each row of `decisions-pending.md` whose trigger (including a bracketed re-trigger) names the opening, a stage with real agents, or a join, and that is not struck. It prints each row's key, its trigger text and its line.
- **Suite:** a fixture file of rows (struck, re-triggered, and a trigger in prose).
- **What it is:** a list for an agent to judge, not a verdict.
- **Gates:** 1 + 1.
- **Then:** M-U.

### Lane K

**K7 — The trial image's base layer**, as in the reshape plan §5. Its two docker builds (about 25 minutes each) go through the queue.
- **Gates:** 1 + 1.
- **Then:** M3, K → T.

### Lane P — serial (F1 A)

The lane writes no product source. A red case is listed with a key and is never fixed here.

**P0 — The projection.**
- **What:** one generated, committed file, held equal by the suite. It holds:
  - a vocabulary header: `Classification` and `Why`'s four words;
  - per family: its doors, its parts (B3, keyed by file plus first site id), and every site, with claimed, `MINTED_AT` and `IN_PART` sites in the family's rows;
  - per site, its answer: each case with its cause; or the reason (word, text, `ruled`); or `scheduled`;
  - `without_case` and `not_followed`, with their `Why`;
  - **per dynamic site, the codes `MINTED_AT` and `CLAIMS` give it** (the review's counter);
  - the residue;
  - **for F-C:** per site, whether its route prints an operand span, and per case, whether it is *holds, operand implied* or uses the marked form.
- **Also:** the named-binary arm takes `JIGC_ROUTE_CATALOG_FAMILY`.
- **Red first:** v2's six arms, plus `a_dynamic_sites_minted_codes_are_its_rows` and `a_parts_key_survives_a_site_added_to_another_file`.
- **The H0 check:** the commit reports the count of *operand implied* and marked-form cases across the catalog, beside the pilot's 32 of 183. More than twice the pilot's rate is H0, before P-F1.
- **Gates:** 1 + 1.

**P1 — B1, then the fence.** Two commits.
- **Commit 1:** the scan leaves out the bodies of `impl Finding` in `crates/engine/src/finding.rs` only (N3). `sites.tsv` and `scheduled.tsv` are regenerated: 142 rows.
  - *Red first:* `a_constructor_of_finding_is_no_refusal_site`, with the negative control `a_finding_literal_outside_impl_finding_is_still_a_site` and its mutant.
- **Commit 2:** `Reason` gains `ruled: Option<heading>`.
  - A heading that is missing from `DECISIONS.md` is red.
  - `ruled: None` is named as owed, and refused only from P-Z.
  - The `not-counted` class is refused.
  - `pinning.md` §6's body is updated.
- **Then:** M-P0. **H1** follows the review.
- **Gates:** 2 + 1.

**P-F0 — Classify the reason texts, build the shared frame pieces, and write the `NotBuilt` cases.**
- **Classification:** about 164 texts, each as *plantable with piece X*, *not constructible* (the reason stands), or *not built*. The table is committed.
- **Pieces:** each built red first with the case that needs it. The first is the per-command environment for "no git on `PATH`" at `setup` (that site only).
- **`NotBuilt`:** every `NotBuilt` case is written here, or named to the C-task whose family owns it (S3).
- **Stop rule:** a piece over about 3 hours of wall time, or one that needs product source, halts.
- **Gates:** 2–4 + 1.

**P-F1 — The act check** (F-C → A).
- **What:**
  - Every act the frame performs names what it is in the product's own vocabulary. The link acts (`ReplaceWithACopy`, `MoveDirectoryIn`, `ReplaceWithDirectory`, `PutTheFileItself`) are paired with `link_way::WayOut` by an exhaustive `match`, so a new variant fails to compile until it is mapped.
  - Each act finds its verb and its operand in the printed route by span, and asserts nothing else of the sentence.
  - A case whose act or operand the route leaves to inference is red, as *followed, holds, act implied* or as *followed, holds, operand implied* (`is_red` changes). Each is listed under a key per route, minted in `route-catalog-findings.md`, for the run's ledger.
  - The §12 key `r1-catalog-link-acts-never-check-the-route-asks-for-them` is struck, with this commit.
- **The order inside the task:** before anything is listed, the first commit reports the count of cases whose act or operand is implied. Far beyond expectation (more than twice the pilot's rate) is **H0**, before the C-tasks.
- **Red first:** the §12 repro (changing `WayOut`'s choice turns a case red); an implied operand is red; an act whose verb the route lacks is red; the exhaustive `match` (a stub variant fails to compile, shown in the commit message). Every case is re-run, and every classification that moves is listed.
- **Gates:** 1–2 + 1.
- **Then:** loop → P (U0's split).

**C1–C15 — The static families**, cut as in v1, 125 sites. **C16a, C16b, C17, C18 — the dynamic families**, re-cut from PC's count: a case for every code; no task over about 35 codes; `compose`'s 27 a task of its own.
- **Commits:** about 10 sites or codes each, each behind its gate.
- **C16a's first commit** adds S2's arm.
- **Done when:**
  - the family's rows are gone from `scheduled.tsv`;
  - the strict expected-red list holds, with any implied act or operand listed red under its key;
  - the projection is regenerated;
  - one `dev/runner-faithful` run of the `route_catalog` part is green, with its line and its proxy seconds in the commit, and the part re-bounded or split in the same commit if the proxy passes half its bound (S1);
  - the named-binary arm, filtered to the family, has run on rc.24, with its table in the record;
  - any new site-level reason carries `ruled: None` and is named in the task's return;
  - the family's `NotBuilt` cases are written.
- **Stop rules:** a site over about an hour stops the task; a missing frame piece is built within P-F0's stop rule, or the task halts.
- **The re-price** happens after C2.
- **Gates:** 25–29 commits + 10–19 fix gates; 19 Linux runs.

**P-NC2 — Sweep the texts the C-tasks wrote**, by P-F0's rule.
- **Gates:** 1 + 1.

**P-Z — Close the catalog.**
- **What:**
  - `Scheduled`, `SCHEDULED`, `THE_COUNT` and its arm, and `scheduled.tsv` are removed;
  - the fence answers a site only with a case or a ruled reason: `ruled: None` and `Why::NotBuilt` are refused (S3), and S2's arm refuses any owed code;
  - every family is in `CATALOGUED`, so the vocabulary arm holds every code;
  - the named-binary arm runs whole on rc.24;
  - `route-catalog-pilot.md` gets §14's numbers, including the F-C class counts.
- **Gates:** 1 + 1.
- **Then:** M-P.

### Lane T — serial, in its own worktree

**T0 — The *Tools* channel.** `proposalPrompt` and `crossModelPrompt` carry the *Tools* paragraph. The triage writer takes an optional `tools` text, rendered as the report's last section.
- **Gates:** 1 + 1.

**RB1 — The regression base, the record script's half.**
- **What:**
  - A filing may carry `earlier`: newest first, one entry per earlier candidate, each with its hash equal to that round's recorded `binary`.
  - The list must be complete, or the fact is unknown.
  - Grade `regression` = the previous release `true`, or any earlier candidate `true`.
  - **In round ≥ 2, a confirmed filing without `earlier` is `regression_unknown`** (S6).
- **Red first:** v2's arms, plus `in_round_two_a_confirmed_filing_without_earlier_is_unknown`.
- **Fixtures:** written through the script's writers; if the state refuses them, the builder halts.
- **Gates:** 1 + 1.

*Waits for loop → T.*

**T1 — An item names `family` and optionally `part`**, keyed by file plus first site id, held to the projection read at the round's candidate commit. **The partition check (S7):** in a round, the `route-hunt` items of every family in scope partition its parts, and `not_ready` names an uncovered part.
- **Red first:** v2's arms, plus `a_family_in_scope_whose_parts_are_not_all_hunted_is_not_ready`.
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

**T8 — The route hunter.**
- **What:** as in v2 (`ROLES.hunt` in the first commit together with the kit's `hunt` role; `launched_under` and its fixture; the size watch, stopping at 440,000 bytes). The prompt reads: *"every site of your part, for causes beyond the catalog's; a (site, cause) the catalog has is not hunted again"* (S5).
- **Red first:** route-hunter plan H4's five arms; the kit's §6 arm; `the_files_the_hunters_prompt_names_exist`; `the_hunters_prompt_hunts_every_site_of_its_part_for_new_causes`.
- **Gates:** 2 + 1.

*Waits for M3.*

**K7b, then K6**, as in the reshape plan; each prints the harness's size.
- **Gates:** each 1–2 + 1.

**T9 — The rest of it:** `its_doors_are_the_review_rows_the_hunts_and_the_arms` and `its_expectation_names_what_it_prices`.
- **Gates:** 1 + 1.

**T11 — Move the scenarios left on the old path.** The builder counts first; the upper bound is the 92 arms, at about ten a commit.
- **Gates:** 3–10 + 1.

**T12 — Delete the old path**, as specified. `runFix` stops pointing at `VERIFY_PASSES`, and `VERIFY_SCHEMA` goes.
- **Gates:** 1–2 + 1.

**O5 — Main-session edits**, as the reshape plan specifies.
- **Gates:** 4.

**T-NR — `wf-neutralized-return`** (B2; F-NR A).
- **What:** a case of the runtime probes, with its judge in `dev/stabilize-probe` and its invocation in the harness. An agent returns, in a structured field, a text shaped like an instruction (`bypass-permissions`, `settings-json`). The judge reports what arrived: in the relay, in a ledger cell through the record script's writer, and in a field the harness holds to a hash.
- **Red first:** the simulation arm and the probe suite's arm.
- **Then (Phase 3):** the main session invokes `{ probe: 'neutralized', scratch }` once (a probe works on no run). A builder records either a declared bound (`bound-set` wording, plus the doc's *Open questions and declared bounds*) or a cell among the endings, and the row is discharged.
- **Gates:** 1 + 1, plus 1 for the record.
- **Then:** M-T.

### J2a — the frozen areas, early (Phase 1)

These are areas of the `test` stage that no task of this plan writes:
- the write guard: `dev/stabilize-guard`, `dev/stabilize-fence`, `dev/stabilize-guard-probe.js`, `dev_stabilize_guard.rs`;
- the regression set: `dev/regression-set`, `dev_regression_set.rs`, the `held/regression-*` fixtures;
- `dev/hygiene-scan`, with its arms in `dev_gate_report.rs`;
- `.claude/settings.json`.

Three read-only `milestone-code-reviewer`s read them over `cbb3d736..HEAD`. Fixes go through U's worktree, at most three cycles, with a scoped re-review.
- **Gates:** 2–6.

### The join (loop checkout, serial)

**T10a — The workflow doc says what the stage does as built.** This is the reshape plan's T10 sections, and the hunter with its parts and its F-C meaning, the regression base as built (the record half only), and T-NR's outcome. It is part of J2b's scope.
- **Gates:** 1 + 1.

**J2-0 — The partition.** A deterministic listing, committed with J2's record: every file of `git diff --name-only cbb3d736..<tip>` in exactly one of:
- (i) a J2a area;
- (ii) a J2b area;
- (iii) a part counted reviewed, with its record: the byte table's families (the `DECISIONS.md` entries "… after its review", 2026-10-18 to 2026-10-27), the link fix (the four entries of 2026-10-28), the route catalog, and the CI fixes — each pointed at by the entry that records its review;
- (iv) outside the `test` stage, with a reason per path class.

A part counted reviewed for which J2-0 finds **no** review record goes to **H-J0** before J2b. At HEAD I found none for the route catalog's code.

**J2b — The moving areas** (after M-T and T10a). Nine read-only `milestone-code-reviewer`s in parallel, over `cbb3d736..HEAD`:
1. the harness source, the generator and the generated file, with `dev_stabilize_harness.rs`;
2. the record script and its suite;
3. the step tool, its suite and the held fixtures, with `dev/gate` as a held check;
4. the definitions;
5. the simulation, the runtime fixture, the harness fence, the trace and the golden;
6. the canary tool, its suite and the golden;
7. the probe tool and its suite;
8. the trial image (`completions/trial-harness/`, with K7);
9. the workflow doc, with the stage's pending rows and `tooling.md` rows.

Then one pass across all of them. Fixes are serial on the loop branch, one commit per area per cycle. **J2r** is the scoped re-review of the fix diff, at most three cycles.
- **Done when:** every finding is fixed or ruled, and `wf-test-review-real-run` is discharged with the record.
- **Gates:** 10–25 + 2–10.

**J1** — as specified, plus anything of T8's arm it did not take.
- **Gates:** 1 + 1.

**T10b — Rows and the record.**
- **Discharged:** `wf-route-walk`, `wf-route-catalog-rest` (with the F-C ruling) and the reshape plan's set. **Not:** `wf-regression-base`, `wf-hunt-cause-to-case`, `wf-hunt-count-view`.
- **Also:** `completions/artifacts/M55/stabilization-build/README.md` and the lessons list (S8).
- **Gates:** 1.

**G1 — The `.gitleaks.toml` entry** (after H2, if A). One entry, exact file and exact line shape.
- **Done when:** `gitleaks git --log-opts "origin/main..HEAD"` exits 0.
- **Gates:** 1.

**Phase 3's end:** `dev/clean-litter`; every branch pushed; CI read; **H3**.

**Phase 4 — in the human's own session.**
- **The canary (if ruled).** The build session prepares the canary's clone with T9's setup and hands it over. The human runs the canary in their own session. Anything it changes in a harness, a script or a definition comes back to the build session as **J3**, a scoped re-review, before the opening (0–4 gates).
- **The opening.** Run by the human's session as its main session: the `stabilize-opening` fact sheet; **H4** and the opening's other questions, one at a time; the `CLAUDE.md` edit per H4's answer; the opening commit behind the gate; the push.
- **This build session stops after Phase 3** (and after J3, if needed).

## 3. Human stops, in order

Each stop is one question at a time, with its recommendation.

| # | When | The question | Recommended | Told with it |
|---|---|---|---|---|
| **H0** (conditional) | After P0, or P-F1's first count, before the C-tasks | The count of routes that leave an act or an operand to inference, against the pilot's 32 of 183 | Carry on, every one keyed for the run (F-C → A stands) | — |
| **H1** | After P1's review | The nine standing site-level reasons: *"accept as worded, or name the ones you challenge"* | Accept, except any the reviewer flags | (b)'s last clause and (c) do nothing until the `fix` half's repair |
| **H2** | Phase 1, any time | The gitleaks hit at `85661425`, `DECISIONS.md:99` | A: one `.gitleaks.toml` entry (B, a fingerprint, drifts; C, a rewrite, is not viable). Only C would have to be before the opening. | — |
| **H-r1…r3** (conditional) | At the ends of 2a, 2b and 2c | New site-level reasons, as one question each | — | — |
| **H-cycle** (conditional) | When any task reaches a fourth review cycle | Allow a fourth cycle? | — | The precedent of 2026-10-28 |
| **H-J0** (conditional) | J2-0, before J2b | A part the human counted reviewed has no review record (at HEAD, the route catalog's code) | A: put it in a J2b area | — |
| **H3** | End of Phase 3 | `wf-whole-canary-after-parts` | A: run what no part showed, including the measured hunt (190–330 min, $60–105). B: the measured hunt only — the floor, since C leaves no hunt rate for the opening to price its `route-hunt` items. C: none. | — |
| **H4** | In the human's session, after the fact sheet | The release being prepared | A: `1.0.0` is prepared (ruling 1) and the call to publish stays the human's at the close (ruling 3); one clause in `CLAUDE.md:7`. B: the next rc. | — |

When U2 lands, its tell goes with the next stop (N12).

## 4. Price, with the arithmetic

**Units of work in P** (derived): 125 static sites, plus the dynamic codes. Those are 73 in `MINTED_AT`, plus an estimated 20–60 of uncatalogued prefixes, so 93–133. Total **218–258, about 220–260**. PC fixes the number.

**Wall time per unit:**
- *Measured:* the 2026-10-12 families did 34 sites from 01:10 to 08:54. That is 464 minutes, or **13.6 minutes a site**, gates included, with no per-task reviews.
- *Derived:* the pilot clocked 3.9 minutes a site (110 minutes over 28 sites), and its clocked minutes were 29.3–39.5 % of wall time. So 3.9 / 0.395 = **9.9** to 3.9 / 0.293 = **13.3** minutes a site.
- **Used: 10–14 minutes a unit.**

**Lane P:**
- **The C-tasks:** 220 × 10 = 2,200 min = 37 h, up to 260 × 14 = 3,640 min = 61 h.
- **Their fixes:** 19 tasks × 0.5–1 fix commit × about 55 minutes (30 of work, 25 of gate) = 9–17 h.
- **C-tasks in all: 46–78 h.**
- **The rest of P:** P0 4–6, P1 4–6, P-F0 8–16, P-F1 4–8, P-NC2 3–5, P-Z 4–6 = **27–47 h** (estimates).
- **P = 73–125 h.**

**Lane T** (estimates per task, review fixes included):

| Task | Hours |
|---|---|
| T0 | 3–5 |
| RB1 | 5–8 |
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
| T-NR | 3–5 |
| **Total** | **59–98, about 60–100** |

**After T, before the join:**
- **T10a:** 4–8 h.
- **J2b:**
  - the reviews, nine plus one pass in parallel: 4–8 h;
  - the fixes: 10–25 commits × 55–85 minutes = 9–35 h;
  - J2r: 1–2 cycles × (3–5 h of review + 3–10 h of fixes) = 6–30 h;
  - **J2b = 19–73 h, about 20–70.**
- **The T chain** = T + T10a + J2b = 60 + 4 + 20 = **84 h**, up to 100 + 8 + 70 = **178 h**.

**The tail**, after both chains: J1 3–5, T10b 2–3, the T-NR probe and its record 1–2, the canary 0–6 (190–330 minutes if A), J3 0–6, the opening 4–8 = **10–30 h**.

**Phase 0:** 4–6 h.

**Critical path** = Phase 0 + max(P, the T chain) + the tail:
- low: 4 + max(73, 84) + 10 = **98 h**;
- high: 6 + max(125, 178) + 30 = **214 h**.

**The T chain is the critical path** (it now ends in J2b, which F-B → A widened). P runs beside it and is the longest single lane at its low end. Priority P > T by ruling (a) costs the T chain the gate waits; that cost is inside the high end.

**Gates:**

| Lane | Gates |
|---|---|
| Phase 0 | 5–7 |
| U | 5–10 |
| J2a | 2–6 |
| K | 1–2 |
| G1 | 1 |
| P | 49–65 (33–40 builds + 16–25 fixes) |
| T | 34–46 |
| T10a, J2b, J2r, J1, T10b, J3, the opening | 18–48 |
| **Total** | **115–185** |

At 20–30 minutes a gate (recent gates took 19–30 minutes), that is 38–92 hours of gate time.
- During the parallel window, P, T, U and J2a ask for about 95–135 gates, which is 32–68 hours inside an 84–178-hour window, so the queue is busy about 40 % of the time.
- Contention therefore adds waiting, not a second bottleneck.
- Beside the gates: 19–22 Linux runs (12–15 minutes each), two K7 image builds (25 minutes each), and one whole rc.24 arm (7–17 minutes).

**Calendar:**
- *Measured pace:* 2026-10-28 ran from 09:32 to 21:11, about 12 hours with the human present; 2026-10-11 and 2026-10-12 ran about 20 hours a day.
- 98 h / 20 h a day = **5 days**; 214 h / 12 h a day = **18 days**.
- Add 1–3 days for the human stops (H1, H2, H3, and H4 in the human's session, plus the conditional ones).
- **6–21 days, so 1–3 weeks.** Weeks, not days. The tail risks are beyond-bound review cycles (the link fix took four cycles in a day), J2 findings that need the human, and PC at its high end.
- **Re-priced** after PC and after C2.

## 5. Forks

**Settled:**
- by the human: F-A → B, F-B → A, F-C → A;
- by rulings or the reviews: forks 1 (A, as a field), 2 (tighten first), 3 (A), 4 (ruling c), 5 (B, with a row), 6 (moot), 7 (A); B1 (A); `ci.yml` edits (by the precedent of `72ca156a`); the queue's priority (ruling a);
- v2's F2, F3 and F5: moot, moved to Candidates; F6: built (U3).

**Engineering forks.** The main session takes these with the recommendation unless it chooses to put them:

| # | Fork | A (robust, recommended) | B |
|---|---|---|---|
| **F1** | How lane P runs | One serial lane | Two sub-lanes by family file |
| **F4** | Pacing of new reasons | Batched at the ends of 2a, 2b and 2c; `ruled: None` refused from P-Z on | A halt per C-task |
| **F-NR** | `wf-neutralized-return` | Build the probe case and run it once, then record a bound or a cell | Declare a bound now, from the canary's two observations, without a drive |
| **F-J2a** | When the frozen areas are reviewed | Early (J2a), with J2b's pass across all areas still reading them in context | Everything at J2b, which adds about 3–6 hours to the critical path |

The human's questions are H0 to H4 (§3).

## 6. Not established

- **The dynamic sites' code counts** (PC establishes them).
- **Whether the state takes a round-2 record written through the writers** (RB1's fixtures).
- **How refusing `UPDATE_GOLDENS` under CI is done for the goldens** (P0 must check).
- **Whether every C-task can satisfy F-C's act check** without a product word exposed. Today `WayOut` is public and its clause is private. A family whose acts have no public vocabulary halts rather than spell its wording.
- **Whether the route catalog's code and the CI fixes have review records** (J2-0; H-J0).
- **How fast the legs grow after U0.** S1's 80 % rule watches it.
