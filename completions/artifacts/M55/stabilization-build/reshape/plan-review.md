# Review of the reshape build plan — the independent reviewer's report, as returned

*Filed by the orchestrating session from the reviewer's return (the reviewer is read-only and writes no file). `R/` stands for the repository root. What became of each finding is the plan's disposition table.*

**Review of the reshape build plan (T: reshaped `test` stage · P: route-catalog pilot) — 2026-10-11.** Read-only; nothing driven, built or run. `R` = the repository root. Verdict: not ready for the human as written — 4 blocking, 12 should-fix. The plan's §2 holds twelve claims, not eleven.

## Blocking

- **B1 — A row that appears after the one triage has no grader, no class and no `next`.** A red item is filed as `red-<item>` by the record call itself, in the same batch as the results (`R/dev/stabilize-record:143-145, 2564`; `R/.claude/workflows/stabilize.js:3603`). It is ungraded, so `route_of` answers `triage` (`stabilize-record:3120-3121`) and `next_of` answers `triage` (`:3377`). Today the lap's triage grades it; T5 has the continuation run no triage, and T1 refuses a row without a kind. Result: a loop or a silent drop — the strand-state class. The re-run path (`position.test.rerun`, `stabilize.js:3413-3423`) has no scenario under two records and a budget either. *Settles it:* red results go to the one triage as a source (they are known before it), with a named class; one simulation arm each for a red held check and for a re-run.
- **B2 — T3 cannot be decomposed: nothing says what makes a filed verdict count.** Today a verdict counts only with its verifier's report (`stabilize.js:3026-3027, 3094`). T3/T6 count a file on disk from an invocation that died, and no test says "filed, no report → no verdict". The script cannot know its caller, so any agent can file first for any key, and written-once makes that filing win. Plan §9 admits `check_reports` was not read. *Settles it:* state the acceptance (report on disk first; the file names its reporter and attempt; scope of "once" = the round) and add those arms before T3 is briefed.
- **B3 — Follow-ups skip triage, and the plan puts that to the human as a premise, not a question.** §3 and T3 queue follow-ups with no merge and no grade. Every rate the rulings rest on (15/30, 9/18) was measured on follow-ups triage had already filtered: it passed 23–56% and merged 1 in 7 (`R/completions/artifacts/M55/stabilization-build/canary-one/README.md:99-102`). H2(a) says it is "the population the 15 of 30 was measured on"; that is false for follow-ups. This is the ruling's own gap: the correction removed replay, the only filter it had for follow-ups, and kept "triage once". Consequence: each verifier leaves about six items, roughly two of them route items, so the unreached route list that holds the close grows with every verdict; only a ruling shrinks it. *Settles it:* its own H question, with the estimated size of the first stop's list (my estimate: 60–90 at budget 30).
- **B4 — The canary of §7 cannot show what it must.** Three faults:
  - §3's order puts `route/seed` below `route/hunt`. With budget 8 and a first wave of 8, a hunt that returns five or more queued route items means the seeded plants are never launched and no follow-up overtakes.
  - Killing the session with verifiers in flight makes T2 relaunch them (no verdict, no spend). That contradicts "never more than eight verifiers in all" and the "ninth verifier: interrupt" rule; the human would stop a correct run.
  - T8 tells the hunter to skip catalogued site × cause pairs. After the join the hunter at `jigc setup` has nothing to hunt; before it, the prompt names a table that does not exist.
  - *Settles it:* a canary order with seeds first or a wave smaller than the budget; count verdicts, not launches; a door the pilot does not cover.

## Should-fix

- **S1 — H10(a) probably misclassifies its own first case.** It requires the previous binary to finish "with every witness intact". By source, rc.24 wrote jigc's block through the link into the outside script (`R/crates/cli/src/setup.rs:907-951, 3111-3116`; `reshape/route-registry-facts.md:286-287`). Not driven by me; drive rc.24 once before the sheet (lesson 13). Missing option: report both classifications and compute no "regression" word in the pilot.
- **S2 — H11(c) would un-red a confirmed defect class.** The writability route prints no operand, and the confirmed findings are "natural act done, same refusal again" (`canary-one/README.md:160-161`). Keep that outcome red; the ungated bucket is only for "natural act works".
- **S3 — The expected-red entry is (case, key) only (E9).** A listed case whose classification worsens (same refusal → witness destroyed) stays hidden. Add the expected classification to the entry. Before the opening the tie is also self-certifying: the builder mints the key into a file the fence reads (P3, J1).
- **S4 — "Route item" is narrowed to "names a refusal's code" (H3(b), T1).** About 17 free-text next steps, 3 hand-written route lines and 12 clap tips are refusals with a route and no code (`route-registry-facts.md:129-138`). They would be recorded as behaviour or lead and stop holding the close. Evidence should be the printed route line, plus the code where one exists. No T1 test holds a code to the scan's vocabulary, though H3(b) promises it.
- **S5 — P2's fence misses a real setup refusal.** `setup.dirty-install-path` is minted through a `code()` method in `R/crates/engine/src/finalize.rs:2139-2144`. It is a dynamic site outside `setup.rs`, so the "unassigned site in `setup.rs`" rule never sees it. `AMBUSH_CONTRACTS` (`R/crates/cli/src/pack.rs:1090`) names its door.
- **S6 — O3's own gate goes red.** Fence arm (k) pins the phrase "the round's triage is not finished" in `finding-verifier.md` (`R/tooling-tests/stabilize_harness_fence.rs:1323-1328`). O3 rewrites that paragraph and the fence is a builder's file. Name the co-change.
- **S7 — No step commits the record the plan is held to.** The first `DECISIONS.md` entry, the lessons, `decisions-pending.md` and `reshape/` are uncommitted. A worktree forked after S0 holds no rulings 1–3 for a builder to read. Add an orchestrator step before S0.
- **S8 — "Verified once in a run" (§2.8) is the human's question.** Today a finding found again is graded again (`stabilize-record:274-279`). The plan parks the hole under `wf-replay`, whose trigger is conditional. Give it a row of its own, due before round 2's `test`.
- **S9 — Only ranked route items hold the close (T2).** `route/under-no-repro`, and any class the human takes off the order, do not. The ruling says a route item nobody reached holds it. Say so in H4 or H6.
- **S10 — §3's order is labelled "as ruled", but the ruling fixes only "under a confirmed parent first".** The rest is E5's choice, and it contradicts the census: route under a not-a-break parent confirmed 7 of 12 (`reshape/repro-census.md:312-313`) yet ranks fourth.
- **S11 — E1 puts a non-product file inside the product gate's verdict.** `route_catalog` under `crates/` runs `dev/refusal-sites`, and E16 guards that by discipline only. It is also a third scanner beside `R/crates/cli/tests/support/rust_source.rs:12`, which warns against a second. Missing option: a Rust scan on that reader plus a committed, fence-held site list the hunter reads — no python in a product suite, and H9 falls away.
- **S12 — Two places where "nothing runs twice" is still not built, and no row says so.** The gate and held checks are named by attempt (`stabilize.js:3461, 3500`), so a kill before RECORD 1 re-runs 45 minutes. "No retry at once" is built for verifiers only (T6's *must not*).

## 1. §2, claim by claim

1. **True** (`stabilize.js:3571-3607`; `:3239-3262`). Honest, but "filed" should say uncommitted and unpushed: a lost disk loses it.
2. **True** (`stabilize.js:2716-2720`; `canary-one/evidence/then-halted.json`: 297 runtime lines, 297 `null return`, no `transient failure`). One observation; the plan says so.
3. **True; "the table is clean" is overstated** — see S5; `uninstall.staged-prose` is also minted at `R/crates/cli/src/task.rs:1329`.
4. **True** (`setup.rs:2391-2397` over `:907`). Honest.
5. **True of the suites** (`R/crates/cli/tests/support/trial_corpus.rs:458, 504`); **"no road" is overstated** — `dev/regression-set` already swaps a binary under built tests (lesson 11), as E10 concedes. Not said: the ruling's sentence becomes a table run once by hand.
6. **True** (no `opening.md` outside `canary-one/run/`). See S3.
7. **Yield claim plausible; its citation is not there.** Census §2a counts shapes, not identity between blocks. The plan then builds the mechanism anyway (E4) — a row, not a task.
8. **True.** See S8.
9. **True** (`stabilize.js:971-973, 3082`). Honest.
10. **True** (`stabilize-record:1289, 3377, 3792-3796`). Honest.
11. **Overstated.** `args.rulings` is a list, so dozens of findings are ruled by key in one invocation (`stabilize.js:914-958`). What is missing is a ruling by class.
12. **True** (`repro-census.md:298`). Honest.

## 2. H1–H11

- **H1** — the human's; (a) is sound; state the durability loss.
- **H2** — the human's; misdescribed (B3).
- **H3** — the human's in part; hides S4.
- **H4, H5** — the human's; sound.
- **H6** — the cost is overstated (claim 11) and the list size understated (B3). `wf-rule-a-class` is needed at the first stop, not "if the list is long".
- **H7** — the human's; (a) is sound. Missing option: a threshold of two silent calls in one wave. Say that other roles still retry three times.
- **H8** — the human's; already half-ruled in `wf-verify-fan-out`.
- **H9** — the human's; see S11.
- **H10** — S1. **H11** — S2.
- **Decided silently:** B3, S4, S8, S9, S10. Also E8 (the clause's first number is a hunter's unchecked word) and E16 (every catalog edit after the opening moves through a round).

## 3. Against the rulings and the scale ruling

- **Ruled, no task:** "a lead already listed is not listed again". O3 keeps undriven items in report prose while T3 makes evidence-less follow-ups rows — two things called a lead, and the stop's leads list holds only rows. Who writes one `route-hunt` item per command into the opening is also unassigned.
- **Not needed before a first supervised run — rows instead:** E4 identity, T4's run-wide rates, T2's lowered-budget arm, S0's `--codes`. T10 should annotate `wf-breaker-parallel`, not discharge it.
- **Missing from the three classes:** B1 (strand), B2 and S4 (record falsely). `wf-neutralized-return` is correctly left due before the opening.

## 4. The cut

- **Write-sets** are disjoint except `DECISIONS.md`, with one undeclared dependency: T8's prompt names the catalog's table, which is stream P's. `g_tooling.rs` is clean (S0, then J1).
- **Too large for one commit:** T2, T3, T5; cut lines should be fixed in the plan, not left to the brief.
- **Red-first:** P1's "does not compile" arm cannot be a test here; T9 and T10 name no test.
- **Edits to definitions, settings and `CLAUDE.md`** are all the orchestrator's (correct). The "What a finding carries" paragraph stands in exactly the two definitions O4 edits. Arm (d) ordering holds: definition before script.
- **Branches:** CLAUDE.md's letter says a new branch starts from `origin/main`; the two stream branches fork from `fix/rc24-tier1`. Cite the precedent (`724e4ce7`).

## 5. Stream P

- **The three numbers:** P5 leaves all three. Separate the frame's fixed cost (S0, P1, P2) from the marginal cost per case, or the number will not transfer to other families.
- **First proof:** real by source. `display_hook_path` canonicalizes through the link (`setup.rs:960-971`), the route prints that path (`:3150-3154`), and the existing test removes a hand-written path (`R/crates/cli/tests/replacing_writers_never_follow.rs:1235`). Span-by-index does take the operand from the printed bytes. The escape hatch is E11's second form: forbid it wherever the route prints a path span.
- **Gate honesty:** strict-both-ways is not enough (S3).
- **Before the opening:** nothing forbids it. The path check reads `opening..loop` (`R/dev/stabilize-step:1245-1247`; `R/implementation/stabilization-workflow.md:406`).

## 6. The estimate

- **It holds for what it prices.** The rows sum to 174–223 minutes, 50 agents and 0.93–1.09 M tokens; 52 USD per M matches.
- **Left out:**
  - the relaunches after the prescribed interruption (up to 6 more verifiers, +0.15–0.45 M, a second wave of 10–30 minutes);
  - the hunter, which is unmeasured and likely the largest term — one verifier needed 17 rigs for one site (`route-registry-facts.md:313-314`), and setup has 23 blocking sites;
  - the human's latency between invocations.
- **My range:** 190–330 minutes and 1.1–2 M tokens.

## 7. Where it is most likely wrong and does not say

B3 (the queue's arithmetic), B1, and T3's trust model (B2).

## Notes

- `next: triage` now means "verify only" — the misnaming `wf-return-names` complains of, made again.
- F12's "one module" is false in premise (S5); F15 is true but no tooling test names `cli::` today.
- A hunter for `rename`, `relocate` or `repo` gets zero listed sites, because those codes are built through helpers (`route-registry-facts.md:345-348`).

The plan reviewed: `reshape/plan.md`

---

# The second read — of the revised plan

*The same reviewer, on the plan as revised against the first review and with the pre-built pieces scheduled. Filed as returned.*

**1. B1–B4**
- **B1 — closed.** `T1`, `T2b`, `T5a` and `T6b` each carry an arm that fails on the defect, and they agree with `route_of` (`R/dev/stabilize-record:3120-3121`) and `next_of` (`:3377`) as they stand. *Note:* the `T5a` arm passes only if triage's entry carries the key `red-<item>` that `result_set` mints (`:2564`); say so. A red check that is not held has no verdict file to be its evidence.
- **B2 — closed for the acceptance (i)–(v); two holes, should-fix.**
  - The file has no home. `check_reports` counts anything in the stage's report directory without a report's name as `extra` and fails (`stabilize-record:3890-3899`); `run_file` refuses it (`:4171-4172`); `set_aside` takes only a report or a scope (`:4474-4476`), so `T3b`'s set-aside arm cannot pass. Name a home like `r<N>/checks/` in `T3a`.
  - A dead attempt's report is `other_attempts`, held by no check's hash (`:3894-3895`, `:916-919`). The filing should store the report's sha256 and the record should hold it.
- **B3 — closed.** I recounted `Q2`'s figures from `canary-one/data/entries.tsv`: 118 of 338, 62 sent on, 13% needs-bound.
- **B4 — closed; three residuals, should-fix.**
  - A seed reaches the queue only if the one triage grades it `breaks` or `unclear` (`E5`). The Linux-and-root plant may well be graded `needs-bound` and never launch; `T9`'s arm checks class, not grade.
  - "No second launch in either invocation" is false if that verifier is in flight at the kill; it should read "none after `undrivable` is filed".
  - The scripted arm brings the trial image back, and with it the composed wait of `wf-image-hold`, which prompts in a session that asks.
  - The hunt at `task discard` does have something to hunt, and a handed-on item can overtake in wave 2.

**2. The K tasks**
- **`K3` — should-fix, records-falsely class.** Today a batch is held to the hash of the text the harness composed (`R/.claude/workflows/stabilize.js:2090`; `stabilize-record:820, 4579`). Under `K3` the item statuses, round facts, launched names and keys become argv words typed by a step agent, unhashed. `green` for `void` is not fail-safe. Keep a `--sha256` over the canonical words, or echo them in the digest, with an arm.
  - State that the launched list stays the harness's; derived from disk, "missing/extra" is vacuous (`:3900-3901`).
  - A rulings record carries free text (a bound's reach), so `ruleTheRun` is not covered. Neither are a red brief-check's `RED_FIELDS` (`:1288`).
- **`K2` — should-fix.** `T5a` promises a partition by finding; `K2` hands the script only counts per source, which holds distinctness and number, not identity. Pass the handed ids. Its "129 k a triage" is the census's figure for four passes plus superseded writes (`prebuild-census.md:244`), about a quarter of that per triage.
- **`K8` — should-fix.** The guard is `K8a`'s; `K8b` only names a directory. The key lacks the tool's own bytes, the runner's version, the resolved `rustc` (not the file) and the host's git. Staleness here mostly gives a false red, so it is not blocking. No step merges the third lane into stream T before `K8b`.
- **`KR` — not closed, should-fix.** It does touch all eight copies; I verified eight carry the paragraph and six hold `Write`. But the harness's own report line still says "write your report, whole … WITH YOUR FILE TOOL" (`stabilize.js:1095`, pinned by the self-test at `:2428`), and `KR` has no harness half.
- **Collisions:** none inside the stream. `K6` consumes the image build `K7` changes in parallel, undeclared.
- **Not on record, should-fix:** `DECISIONS.md` has no `2026-10-11` heading, so the ruling every K task cites is in no file a builder can read. This is S7 again.

**3. The window — new problem, blocking**
- **(a) `T5b`'s refusal is `NOT_FIT`, which returns before any agent (`stabilize.js:2682-2685`).** All 43 simulation tests run the unmodified harness through `test`, so they are refused. `T5b`'s and `T6a`'s own simulation arms cannot run. The gate is red at `T5b`, `K8b` and `T6a`. *Settles it:* a stop after the first record that the simulation can pass, or the old path kept until `T6b`.
- **(b) The gate is red much earlier, from `T1`.** The simulation drives the real record script through the unchanged harness until `T5a`:
  - `T1` refuses a row without a kind.
  - `T2a` makes every fixture opening not-ready (`R/tooling-tests/stabilize_simulation.rs:468-500`; `R/tooling-tests/dev_stabilize_canary.rs:814`); `K1a` does the same.
  - `T2b` answers `verify`, which `KNOWN_NEXT` lacks (`stabilize.js:528`).
  - Their write-sets exclude `SIM` and `CAN`. *Settles it:* script tasks stay additive until the harness task flips, or each carries the fixtures.
  - This was true of the first cut too; I missed it.

**4. Q1–Q3**
- **`Q1` — true to the code.**
- **`Q2` — option A's cost is understated; should-fix.** Every triage pass cost 98–169 k and 14.5–33 minutes whatever it was handed (`canary-one/data/agents-attempt-1.tsv` seq 24, 38, 53, 93). Four waves is about 400–650 k and 60–130 minutes before `K2`, not "100–160 k … 20–75 minutes". §11's "no pass alone" is false.
- **`Q2` — option C records a kind falsely; should-fix.** A route-quoting item beyond N is entered as `lead`, against `T1`'s own definition, and leaves "holds the close" without `Q3` asking. Record it as `route`, off the queue, and let `Q3` govern.
- **`Q3` — true** (11 of 80; 0 of 11; 0 of 6).
- **Still silent:** `E5` (the grade still gates what triage saw) is not among "told"; `K1a` copies the clauses into the record where ruling 14 says by reference.

**5. What the revision broke**
- **Held checks named by commit (`T5b`, from my S12) defeat `retest`; should-fix, strand class.** `hold-start` answers the job that is there (`R/dev/stabilize-step:725`). A void or dead check's automatic re-run (`stabilize-record:3217`; named by the item's attempt today, `stabilize.js:3500`) gets the same dead job forever, and a starved red gate likewise. The only exit is a new scratch root. Name by commit plus the item's attempt, and give a standing job without a verdict the next name (`:2762`).
- **Key minting for a handed-on item is gone from `T3a`.** Acceptance (ii) needs `verify-<key>` to fit in 100 characters (`stabilize-record:1228`).
