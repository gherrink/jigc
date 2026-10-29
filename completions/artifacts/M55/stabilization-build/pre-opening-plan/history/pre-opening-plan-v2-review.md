# Review of `pre-opening-plan-v2.md`: everything owed before the first stabilization run opens

*Adversarial and read-only. 2026-10-28, against `fix/rc24-tier1` @ `7c34a222`, with a clean tree. Every finding below was checked against the files it cites. I ran no gate and no docker. What I did run: `gh run view 40767224323` (job durations); a join of `scheduled.tsv` to `sites.tsv`; a parse of `MINTED_AT`; counts of `Why::` entries per catalog file; `git rev-list` and `git diff --shortstat` over the range J2 would review; and greps over `decisions-pending.md` for every row whose trigger is the opening, a real-agent stage, or a join that has already happened.*

**Verdict: not ready to show as is.** The fold-in of v1's review is honest, and every count I re-derived holds. Three things block:
- J2's diff base leaves the whole second repair of the `test` half unreviewed.
- One row due before the opening, in the "records falsely" class, is in no task.
- An unruled human question rides inside a pending row that the plan discharges, and P-F1 would decide that question by construction.

Beyond those: CI's bounds are already at 84–95 % on four legs, a ruled rule ("a case per code") is held by no fence, and the price does not add up. **Counts:** 3 blocking · 16 should-fix · 13 notes · 3 forks for the human.

## The cited facts, re-derived

| Plan's claim (§0.1) | Re-derived | Holds |
|---|---|---|
| 145 scheduled = 144 `unchanged` + 1 `not-counted` (`finding.rs::graded::dynamic#1`) | `scheduled.tsv`: 144 + 1, that row | yes |
| 125 static + 20 dynamic; 3 dynamic are `Finding`'s constructors | 125 / 20; `finding.rs::block`, `::blocking`, `::graded` (`scheduled.tsv:100-102`) | yes |
| 142 after B1 | 145 − 3 | yes |
| C1–C15 = 125 sites | v1's table sums to 125; the per-family-file join at HEAD matches it | yes |
| 73 `MINTED_AT` codes over 11 sites (27, 10, 10, 7, 3×5, 2×2) | 27 `WORKFLOW_REFS_BLOCK`, 10 `BLOCKING_WRITE`, 10 `CORPUS_BLOCKED`, 7 `STORE_BLOCK`, 3 each for five sites, 2 each for two | yes |
| 157 `NotConstructible(`, 7 `NotBuilt(`, 2 `Platform(`, 9 site-level `Reason {` | the family files: 157 / 7 / 2; 9 `Reason {` (config 1, setup 4, task_finalize 2, unassigned 2) | yes. Two of the nine reach `NotConstructible` through `the_step_before_answers!`, not by spelling it. |
| `Why` has four words | `NotConstructible`, `NotBuilt`, `Platform`, `NoRoutePrinted` (`route_catalog.rs:1549-1561`) | yes |
| the `base` round fact | `dev/stabilize-record:2527` (`ROUND_FACTS`), help at `:718` | yes |
| `g_migration` 8 min 24 s on run `40767224323` | 20:56:08–21:04:32, `success`; the run is green | yes |
| harness 389,986 bytes against 458,752 | `wc -c .claude/workflows/stabilize.js`; `BOUND` at `dev_stabilize_harness.rs:63` | yes |
| the scratch scripts (83 and 27 lines) | `link.rOG8BO/record-print.py` 83; `rawwrites.py` 27, in two copies | yes |
| the eight keys | four in `byte-table-findings.md` §30 and four in `route-catalog-findings.md` §12 | yes |
| "the `CLAUDE.md` sentence was corrected … in `63ce9ca5`" | it changed in **`26474b80`** (`git log -- CLAUDE.md`) | no (N2) |

---

## Blocking

### B1. J2 reviews from the wrong base, so the entire second repair of the `test` half is never reviewed for a real run. `wf-test-review-real-run` would be discharged falsely.

**Where.** v2 §2, J2: *"Each reads the diff from `6e78975b` to HEAD over its area."* This was inherited from v1's review B5 (*"first commit since, `6e78975b`"*).

**What the record says.**
- The last re-review of the `test` half read **`603018ac^..cbb3d736`**: `DECISIONS.md:8555`, recorded in `a7d15955`, 2026-10-07 02:42. Its verdict was *fit for a canary run, not yet for the real run* (`:8553`).
- `6e78975b` is not the first commit after that. It is the 2026-10-08 01:14 **merge** that joined the two halves of the second repair (`DECISIONS.md:8039`).
- `git rev-list --count cbb3d736..6e78975b` = **47 commits**, and `git diff --shortstat cbb3d736 6e78975b -- dev/ .claude/ tooling-tests/` = **35 files, +27,050 / −2,265**. That includes:
  - the harness, +1,955;
  - `dev/stabilize-step`, +1,859;
  - `dev/stabilize-record`, +1,167;
  - `dev/stabilize-canary`, +895;
  - `dev/stabilize-probe`, +763, new;
  - `dev/regression-set`, +263;
  - `dev/hygiene-scan`, +52.

  These are `a99b7639`, `4853016e`, `7db87417`, `7a04ab97`, `99f2607a`, `a182c92e`, `083d4745`, `afcde397`, `778b91b6`, `106c5179`, `6d30208f`, `3ad1e20f` and `5dea0cba`, among others.
- `git diff 6e78975b HEAD` takes the merge's tree as its base, so every one of those lines is outside J2.

**Why it blocks.**
- The row J2 discharges says *"no record since satisfies the condition"* (`decisions-pending.md:156`).
- The doc's condition is the re-review of **the repair** (`stabilization-workflow.md:13`).
- The second repair had only reviews scoped to the three dangers (`378227f3`, and R0 of 2026-10-12, *"only the code the parts run"*).
- So J2's done-condition can be met while the repair it exists to certify is unread. That is a false record in the exact class the 2026-10-07 scale ruling repairs before a run.

**Fix.**
- Base J2 on **`cbb3d736`**, the tree the last re-review read; the record commit `a7d15955` adds only docs.
- Re-price J2. The diff grows by about 27 k lines over the same areas, so expect more than 4–8 + 0–3 gates.

### B2. `wf-neutralized-return` is due before the run's opening, belongs to the "records falsely" class, and is in no task.

**Where.**
- `decisions-pending.md:123`: *"Trigger: **before the run's opening** — it is of the class that records falsely, though it was not seen to."* No bracket discharges it.
- The reshape plan's §7 (`plan.md:424`): *"`wf-neutralized-return` — **left, and still due before the opening**"*.
- v1, v1's review and v2 never name it (`grep -i neutraliz` over all three: 0 hits).

**What is owed.** One drive that sends an instruction-shaped text (the canary saw `bypass-permissions` and `settings-json`) through a reporter's return into a ledger cell and into a field the harness holds to a hash. It records what arrives, and then either a declared bound or a cell among the endings.

**Fix.**
- Add a T-lane task after T12, once the one path is final. A simulation arm can carry the text, but the row asks for a real runtime return, so it may need one probe invocation by the human in their own session (one-run-one-session).
- Or bring the row to the human as a declared bound, with the reason.
- Either way, the plan must say which.

### B3. `wf-route-catalog-rest` carries an unruled human question. v2 discharges the row (T10) without asking it, and P-F1 decides it by construction.

**Where.**
- `decisions-pending.md:212` (and `DECISIONS.md:5942`): the row *"carries what the struck row leaves unruled: what *works as printed* means for a route that prints no operand, or that is prose."*
- `grep` finds no later ruling.
- v2 T10: *"Pending rows discharged: … `wf-route-catalog-rest`"*.
- v2 P-F1: the link acts *"read their act from the printed route by span … and are refused where the route does not ask for that act."*
- The link routes are prose: `link_way.rs:113-128`, *"remove the link at `X` and move the directory `Y` into its place"*.

**Why it blocks.**
- The answer decides whether the `holds, operand implied` class (green and counted apart, per E13) is a finding. So it decides what the opening ledger is seeded with, and the number the exit rule's first clause reports.
- Finishing the catalog "now" (ruling a) without it means 220–260 units of cases are classified under an unruled meaning.
- P-F1 then binds acts to prose wording, against the frame's own rule *"Must not: assert a route's wording"* (`plan.md:271`, P3/P4). That is a reversal of the frame's design, made silently.

**Fix.**
- Add a human stop, **H0**, after P0 and before P-F1. P0's projection gives the count of `operand implied` cases and of prose-only routes.
- The recommendation (robust end state, per `working-principles.md` → the behaviour that makes sense wins): a route works as printed only when the printed text names every act a case performs — its operand **and** the act. An `operand implied` route is a finding class for the ledger, never green.
- Rule it before P-F1. Then build P-F1 against the ruling, binding acts to an act word the product exposes (`WayOut`), not to a phrase the test spells.
- T10 then records the ruling with the discharge. See fork F-C.

---

## Should-fix

### S1. CI's bounds are already at 84–95 % on four legs, and the plan splits only one of them. It also drops v1's rule that re-splits mid-lane.

**Evidence.** Run `40767224323`:

| Leg | Took | Bound (`ci.yml:271-283`, `:239`) | Share |
|---|---|---|---|
| `g_tooling, rest` | 629 s | 11 min | **95 %** |
| `g_tooling, simulation` | 642 s | 12 min | **89 %** |
| `g_migration` | 504 s | 10 min | 84 % |
| `g_tooling, record` | 518 s | 14 min | 62 % |

**What the plan adds to those legs.**
- **`rest`** gets U1's suite, U4's, U6's, U3's arms, `tooling_index` rows, T2's step-tool arms, K7b and K6 (`dev_stabilize_step` runs in `rest`) and T9's canary arms.
- **`simulation`** gets T8, K7b, K6, T11 and T12.
- **`g_migration`**'s new `route_catalog` part roughly doubles in cases during P (272 s inside a gate at HEAD, `route-catalog-pilot.md:808`).

v2 drops S2's own fix, *"split the leg in the commit that takes its proxy past 5 minutes"*. A C-task now only *records* its wall seconds, and the bounds are measured again at P-Z.

**Fix.**
- U0 re-measures and re-bounds **every** part at or above about 80 %, the `g_tooling` parts included, by the header's rule.
- Every task's done-when carries the rule: *"a part whose four-CPU proxy passes half its bound is re-bounded or split in this commit"*.
- Otherwise the human's 2026-10-27 aim (*"so CI checks every commit on Linux again"*, `DECISIONS.md:118`) is lost mid-build, as it was from 2026-10-07 to 2026-10-12.

### S2. "A case per code" (fork 3 A) is held by no fence. P-Z's done-condition can be met with one case for `compose`'s 27 codes.

**Evidence.**
- `route_catalog.rs:256-262`: *"A row of [`MINTED_AT`] makes none"* (no site). `:273-277`: *"Outside the prefixes of [`CATALOGUED`] a dynamic site is one row whatever it mints."*
- P-Z's arm is *"the fence answers a site only with a case or a ruled reason"*: per **site**.
- C16a–C18's done-when is *"the family's rows are gone from `scheduled.tsv`"*.
- The uncatalogued codes rest on PC, an agent's reading, which nothing holds (*Deterministic first*).

**Fix.**
- At P-Z (or C16a), turn every `MINTED_AT` row into a per-code claim, or add an arm: *every code a dynamic site is said to mint has a case at that site, or a ruled reason per code*.
- Once every family is in `CATALOGUED`, the vocabulary arm then holds the uncatalogued codes PC finds.

### S3. P-Z can close the catalog with `Why::NotBuilt` reasons still standing.

**Evidence.**
- `NotBuilt` is defined as *"A state the runner could plant reaches it, and none is planted"* (`route_catalog.rs:1553-1556`). There are 7 today, in `doc_read.rs`.
- P-F0 classes texts as *"not built: plantable now, the case simply not written"*, but names no task that writes those cases.
- P-Z refuses `ruled: None` only.

**Fix.** P-F0 or the owning C-task writes every `NotBuilt` case, and P-Z's arm refuses `Why::NotBuilt`. Otherwise "finished now" (ruling a) is false by the catalog's own vocabulary.

### S4. J2's areas leave out parts of the `test` stage, and narrowing J2 overrides a human ruling without saying so.

**Areas left out.** Among what `cbb3d736..HEAD` changed, J2's five areas (harness · record script · step tool · definitions · simulation and fences) do not name:
- the write guard (`dev/stabilize-guard`, +336, new; `dev/stabilize-guard-probe.js`; their suite);
- `dev/hygiene-scan` (the "publish unscanned" class);
- `dev/regression-set` (a held check);
- `dev/stabilize-canary`, `dev/stabilize-probe`, and `dev/stabilize-harness` (the generator);
- `.claude/settings.json` (+21);
- `completions/trial-harness/` (K7, held by K7b);
- the workflow doc, which T10 rewrites and which every orchestrator follows. The 2026-10-07 re-review read it whole (`DECISIONS.md:8555`).

**The ruling it overrides.** The human's ruling 2 of 2026-10-12 (`DECISIONS.md:4829-4839`): *"The review of the build's whole diff stays at the join."* v2 limits J2 to the `test` stage's areas and puts product tests *"out of scope"* without citing that ruling (v2 §0.2, B5).

**Fix.**
- Partition `git diff --stat cbb3d736..HEAD` so that every file is in exactly one area; add a "held checks and guard" area and a "doc" area.
- Put the narrowing to the human (fork F-B).

### S5. Ruling (b)'s *"each plantable cause found → a catalog case"* has no effect in this run, and the plan does not say so. T8's prompt wording risks a hunter that hunts nothing.

**Why it has no effect.**
- `fix` is `NOT_FIT`.
- After the opening, every catalog edit moves through a round (`plan.md:378`).
- So round 1's hunted causes reach only the ledger. This is the same kind of fact as (c)'s, which the plan does tell the human.

**The wording risk.**
- T8's prompt says *"the catalog's cases (not hunted again)"*.
- After P-Z, every site has a case or a ruled reason. Read at site level, that sentence leaves nothing to hunt.
- The human's ruling, as handed over (POINTER 80): *"every site for new causes"*.

**Fix.**
- Tell the human (b)'s last clause is inert until the `fix` repair, next to (c)'s fact.
- Word T8's prompt as *"every site of your part, for causes beyond the catalog's; a (site, cause) the catalog has is not hunted again"*, and give it an arm.

### S6. RB1 should fail loud in round 2 and later, not answer as before.

**Evidence.** *"A run without the new field answers exactly as before"* also covers a round ≥ 2 confirmed filing that lacks `earlier`. No round 2 is reachable before the `fix` repair. So if that repair lands without RB2 (the row forgotten), round 2 silently grades without the run's base.

**Fix.**
- In round ≥ 2, a confirmed filing without `earlier` counts as `regression_unknown`.
- No reachable run changes, the additive property holds for every run that can happen, and the gap becomes visible. Add the arm.

### S7. Hunt parts are ordinals over a moving list, and nothing checks that the opening's items cover every part.

**Evidence.**
- Parts are cut *"by file, at most 12 sites each, with a file over 12 split in id order"*. A fixer adding or removing one site re-cuts a file's parts.
- T1 refuses an item whose part the projection lacks, and nothing names a part with no item.
- The items come from an agent following `stabilize-opening.md`.

**Fix.**
- Key a part by file plus its first site id, so the key is stable.
- Add a record-script check, at the opening or the scope step: the `route-hunt` items of every family in scope partition its parts.
- Note the round-2 re-cut on the `fix`-repair rows.

### S8. Rows whose triggers have already fired are left out.

**`wf-route-cases-same-commit`** (`decisions-pending.md:151`).
- Trigger: *"the join of the two branches"*, which happened at `e771c46d`.
- What is owed: one sentence each in `dev-workflow.md`, `increment-workflow.md`, `build-executor.md` and `increment-validator.md`. `grep` finds none.
- v2 edits only `build-fixer.md`, so the repeated rule lands in one copy of five.

**Row 101.**
- The gate's *"(probe · fmt · clippy · build · test)"* is still stale in 8 definitions. Three of them are launched by a run: the reviewer, the tester and the advocate.
- Trigger for those three: *"before a stage runs with real agents"*, already passed.

**`wf-o2`** (`:114`).
- The reviewer's definition never says the hash is returned on a pass that drives nothing (`milestone-code-reviewer.md:34`).
- Same trigger.

**The reshape plan's J2 writes** (`plan.md:284`).
- The build's record, `completions/artifacts/M55/stabilization-build/README.md`, which `CLAUDE.md` names as where the build stands, is updated by no v2 task.

**Fix.** Fold all four into the definitions batch, T10 and the join.

### S9. U5 does not measure what §0.1 says it measures.

**Evidence.**
- §0.1 claims *"The extent of `r1-config-set-writes-through-a-link-at-the-manifests-own-place` … is measured by U5's census."*
- U5 is *"unclassified"*: a list of about 213 raw write sites, which says nothing about which of them follow a leaf link.
- U5 also builds on *"the site scan's own reader"*, which lane P owns and edits in P1's first commit (`support/refusal_sites.rs`). That is an overlap §1 does not declare.

**Fix.**
- Either classify each workbench writer as *follows a leaf link / replaces it / not a workbench path*, so the census bounds the key; or drop the claim.
- Declare the overlap with P, or have U5 wait for M-P0.

### S10. The price does not reconcile with itself.

**Calendar against running hours.**
- 110–220 running hours at 16–20 h a day is **6–14 days**, but the calendar says **3–5 weeks**.
- The best case, *"about 2.5 weeks"*, is 280–350 h at that pace: more than the worst-case critical path.
- The gap (human latency? queue contention?) is not stated.

**Gate totals, summed from the plan's own task lines.**

| Lane | Plan says | Task lines sum to |
|---|---|---|
| P | 45–60 | 46–61 |
| T | 33–38 | 32–40 |
| Join | 9–19 | 10–21 |
| Total | about 100–135 | about 99–142 |

**C-task commits.** *"22–26 commits"* is understated. 15 static tasks need at least 15 commits, and 95–135 codes at about 10 a commit add 10–14, so at least **25–29**.

**Per-unit time.** *"Per unit: 14–22 minutes … This is measured"*: the 14 is derived, the pilot's clocked minutes divided by its 29–40 % share (`route-catalog-pilot.md:489`).

**Fix.** Show the derivation, label what is derived, and either explain the calendar's padding or cut it.

### S11. The one-gate-at-a-time queue has no mechanism, and "catalog first" implies a priority the plan never sets.

**No mechanism.**
- `dev/gate` has no machine-wide lock.
- `--private-target` removes the only lock there is (cargo's on the shared `target/`).
- Builders start their own gates.
- In the pilot, gates ran *"beside another lane's gate on a loaded machine"* (`route-catalog-pilot.md:68`). The `wf-gate-starved` row and the 50 ms probe-hold flake are this class.

**No priority.** Ruling (a) puts the catalog first, and P is the critical path, but nothing gives P's gates priority over T's and U's.

**Fix.**
- Name the mechanism: each builder stops before its gate and returns, and the main session runs one queue. Or add a lock to `dev/gate`.
- Order the queue P > T > K > U.

### S12. Phases 1 and 2 are never defined, yet the push cadence, the re-price point and the H-r batches depend on them.

**Evidence.** v2 lines 49, 100, 157, 433, 463 and 487 use the two terms, but §2 defines only Phase 0. *"Re-priced at the end of Phase 1, from PC and from the first two C-tasks"* only makes sense if Phase 1 contains C-tasks, while *"H-r … at the end of each part of Phase 2 (after C8 …)"* puts them in Phase 2.

**Fix.** Define the phases and push at the end of every task (the push-cadence rule), not at phase ends that could be days apart.

### S13. P-F1 reverses a frame rule silently (see B3).

**What happens.** Reading an act from prose asserts route wording, which `plan.md:271` forbids.

**Fix.** Gate P-F1 on fork F-C, and bind act to route through a product-side vocabulary (`WayOut`'s word) rather than test-spelled phrases.

### S14. U3–U6 are not owed before the opening.

**Evidence.**
- The human's rule of 2026-10-07 (memory `tooling-good-enough-first-run`): before a first run, repair only what can publish unscanned, record falsely or strand state.
- U3 (rig link plant) and U6 (`ci-runs`) are none of those. U4 is arguably "records falsely" (the byte table's re-counts are done by hand today). U5 is only so if classified (S9).
- U7 waits on H2, and M-U waits on U7. So an unanswered H2 holds U0–U6 off the loop branch, and the join reads a loop branch without them.

**Fix.**
- Label U3–U6 as discretionary, put them last in the gate queue, and merge U0–U2 at M-U0 so that nothing owed waits on H2.
- U3 should also plant a **loop** (and a link through a file). The third re-review asked for it (`rr4` *Tools*), and it is the state of the open key `r1-uninstall-half-tears-down-over-a-hook-link-it-cannot-read`.

### S15. H1's place contradicts itself.

**Evidence.** *"Blocks: nothing until P-Z"*, yet *"The ruling is recorded in P-F0's first commit"* (v2:268). A late answer would either block P-F0 or break that sentence.

**Fix.** Record the ruling in whichever P commit follows the answer. Recording it is the Phase 0/join lane's per §1, so state the exception.

### S16. H3's option C leaves the opening without a hunt rate it needs, and H3's options ignore who drives the canary.

**The missing rate.** `plan.md:494`: before round 1's opening writes a `route-hunt` item, the orchestrator multiplies the measured hunt rate by the command's sites, and a command too costly gets no item. With option C (no canary), no rate exists. Say so in H3, so that B is shown as the floor.

**Who drives.**
- The canary is driven by the human in their own session (one-run-one-session). So, per POINTER 76, is the opening.
- v2's *"The opening"* reads as build-session steps. Say that the build session prepares and stops there.

---

## Notes

- **N1. U1's docker arm names the wrong green commit.**
  - *"`0bd8a5b5` is red … and its child is green."* `0bd8a5b5`'s child on the branch is `22f06d31`, which has the PPA git but not the guard, so it is red on the same two cells.
  - The green commit is `63ce9ca5`. Name both commits.
- **N2. The `CLAUDE.md` sentence changed in `26474b80`, not `63ce9ca5`** (§0.1, the S7 row).
- **N3. B1 needs a negative control.**
  - Add an arm and a mutant: a `Finding { … }` literal outside `impl Finding` is still a site.
  - *"or builder"* is broader than *"Finding's own constructors"*. Limit the exclusion to the bodies of `impl Finding` in `finding.rs`.
- **N4. §6's *"whether the print the byte-table record is read off is committed"* is already answered: it is.** See `byte-table-record.md:27`, the four committed files. F3 A is buildable.
- **N5. D0's candidates triage misses several harvested tools.** `n4-measure.py` (the guard's cost measure), `enclosing.py` / `callcount.py` (`rr3` *Tools*), `link-cells-repro.sh`, and the reviewers' proposals for suite modes (`readforms-all` as a mode of `link_on_the_way_axis`, `jsonreject` for JSON purity, `dryforecast` for the axis suite) each need a line, built or declined (`tooling.md:77`).
- **N6. Two ledger leads in the handover (POINTER 81) have no key in §12 or §30:** *`migrate.source-untrackable` vs the link code* and *the read-form advisory and the pinned contracts*. A lead that lives only in memory is graded by nobody (`decisions-pending.md:86`). Key them.
- **N7. T11's gates.** *"about ten a commit"* over up to 92 scenarios means up to about 10 gates, not 3–6. The "92" has no source I could find in `DECISIONS.md`. The builder counts first, so say it is an upper bound.
- **N8. Undeclared write-set overlaps.**
  - P-Z re-measures the `g_migration` bounds, which writes `ci.yml` and `ci_matrix_fence.rs` (U's files).
  - U5 shares P's source reader (S9).
  - U4's row cites `pinning.md` §7 (P owns `pinning.md`'s §6 body).
- **N9. H1 and the H-r batches.** Put each as one question ("accept these as worded, or name the ones to challenge"), so the one-at-a-time rule holds.
- **N10. (e) as recorded.** The human ruled a fourth cycle for the link fix. *"A data point for the bound of three, put at the first stop"* is the orchestrator's proposal; D0 should record it as such.
- **N11. D0 should annotate two rows.**
  - `wf-o5` (`:110`) is superseded by the 2026-10-16 `bypassPermissions` ruling (`stabilization-workflow.md:679`), but it is not struck.
  - `wf-return-size` (`:169`) has an opening condition, *"far more than a hundred untriaged rows"*. The opening will seed far more (73 byte-table keys, the canary's leads, the fixers' 16 left-opens, the 8 keys). K2's digest return likely moots it, but say so.
- **N12. H2's "Told with it" (U2 turns CI red on drift) belongs with U2, not with the gitleaks question.**
- **N13. Four worktrees with private targets.** Expect cold first builds and disk pressure (26 GB of `debug/deps` in the precedent). Run `dev/clean-litter` at the join.

---

## Forks the human should see (one at a time, in this order)

**F-C first, because it blocks P-F1: what does *works as printed* mean for a route that prints no operand, or is prose?**
- **A, recommended:** a route works only when its printed text names every act and every operand a case performs. `holds, operand implied` is a finding class for the ledger.
- **B:** the natural act holding counts as working.

It is carried, unruled, in `wf-route-catalog-rest` (`decisions-pending.md:212`). Ask it after P0, with the count of such routes.

**F-A: the two siblings of ruling B left open — before the opening, or the run's?**
- `r1-config-set-writes-through-a-link-at-the-manifests-own-place`: a link at a workbench file's own place. `config set` writes outside the repository at exit 0, and the knob is in no commit.
- `r1-uninstall-half-tears-down-over-a-hook-link-it-cannot-read`: the hook sibling of S-1, which was fixed for `CLAUDE.md` and the settings file.

Ruling B's reason was *"the human wants nothing to come back to"* (`DECISIONS.md:61`). v2 decides "the run's" without asking.
- **A, recommended:** classify U5's census for leaf-link writers (S9), and close both classes before the opening as B's siblings. One write rule at the workbench leaf, and `hook_written_through_a_link` asking every end but `NotFound`, as S-1 does.
- **B:** they go to the run's ledger.

**F-B: J2's scope against the 2026-10-12 ruling that the whole diff is reviewed at the join.**
- **A, recommended:** J2 over `cbb3d736..HEAD`, partitioned so that every file of the `test` stage is in one area, the guard, held checks and doc included (B1, S4). The product-test diff (stream P, the byte table) is held by its per-task reviews, each named in the record, with the human's word that this replaces the whole-diff review for it.
- **B:** the whole diff, as ruled, product tests included. This adds a P area and roughly doubles J2.

Recommendations on v2's own human stops:
- **H2:** A is sound.
- **H3:** B is the floor (S16), A as recommended.
- **H4:** A is sound.
- **H1:** sound once phrased as one question (N9).
- **F1–F4** are engineering calls and stay so. **F5/F6** are discretionary (S14), and **B1** is an engineering call (with N3).

---

## Tools

- **The `MINTED_AT` counter.** It answers *how many codes each dynamic site is said to mint*. Run from `crates/cli/tests`:

  ```python
  import re; s=open('route_catalog.rs').read(); a=s.index('const MINTED_AT: &[MintedAt] = &['); b=s.index('\n];',a)
  from collections import Counter; c=Counter(m[1] for m in re.findall(r'MintedAt\s*\{\s*code:\s*"([^"]+)",\s*site:\s*(\w+)',s[a:b]))
  print(sum(c.values()), len(c), c.most_common())
  ```

  It is a candidate for P0's projection, which should print it per site.
- **The scheduled-to-sites join** (v1 review's *Tools*). Re-used unchanged; the projection supersedes it.
- **The size of an unreviewed range.** `git rev-list --count <reviewed-tip>..<base>` and `git diff --shortstat <reviewed-tip> <base> -- dev/ .claude/ tooling-tests/`. This caught B1. A `dev/` tool that, given a review's recorded range, prints what no recorded review read would answer *"is this condition met?"* deterministically. It is a candidate.
- **The CI legs' share of their bounds.** `gh run view <id> --json jobs --jq '.jobs[] | {name, s: ((.completedAt|fromdate)-(.startedAt|fromdate))}'` against `ci.yml`'s `minutes`. U6 (`dev/ci-runs`) should print *share of bound* as a column. That is what S1 needed.
- **The pending-row trigger scan.** A short Python script over `decisions-pending.md` that extracts each row's `*Trigger:*` text and flags those naming the opening, a real-agent stage, or a join already made. It found B2 and S8. It is a candidate for D0-def's fact-sheet section 0, which should run it rather than read the rows.

<!-- end of report -->
