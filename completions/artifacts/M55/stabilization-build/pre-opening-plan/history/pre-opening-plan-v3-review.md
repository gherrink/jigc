# Focused re-review of `pre-opening-plan-v3.md`

*Read-only. 2026-10-28, against `fix/rc24-tier1` @ `7c34a222`. Each finding is checked against the files. I ran no gate.*

*What I did run:*
- *counts of `Act::` variants across the catalog;*
- *a read of `route_case.rs`'s `Classification::is_red` and its consumers;*
- *`link_way.rs`'s `WayOut` and its clause;*
- *the `DECISIONS.md` headings and the briefs directory, searched for review records;*
- *`git log` / `git diff --shortstat` over the byte table's first commits;*
- *the 2026-10-12 commit times;*
- *a re-addition of §4.*

**Verdict: nearly ready — fix the two blocking findings, then show it.**
- v2's 3 blocking and 16 should-fix findings are worked into the task text, not only the table. I spot-checked P1, P-F0, P-Z, C-done, RB1, T1, T8, U0, M-defs, D0, T10b, §1's queue and the phases.
- The three rulings are represented faithfully in intent.
- Two new problems would halt the build or record falsely.

**Counts:** 2 blocking · 7 should-fix · 6 notes · 1 question that is genuinely the human's.

## Were v2's findings worked in? (checked in the task text)

| v2 finding | In v3's task text | Holds? |
|---|---|---|
| B1 J2 base | J2-0 / J2b over `cbb3d736..HEAD` | yes — but see B-2 below on what counts as "reviewed" |
| B2 neutralized-return | T-NR | yes — but see S-1 and the human's question |
| B3 unruled question | F-C → A; D0 annotates; T10b discharges after P-Z | yes |
| S1 CI bounds | U0 re-bounds every leg ≥ 80 %, with a done-when; C-done re-bounds its part; the rule after every push | yes |
| S2 case per code | C16a's first commit; P-Z refuses an owed code | yes — see N-3 on prefixes |
| S3 `NotBuilt` | P-F0, C-done, P-Z refuses | yes |
| S5 | T8's prompt sentence plus an arm; H1 tells the human | yes |
| S6 | RB1's arm | yes |
| S7 | T1's partition check; stable part key; P0's arm | yes |
| S8 | D0 (two docs), M-defs (row 101 in all 8 copies, `wf-o2`, the two definitions), T10b (README) | yes |
| S10 | §4 re-done | adds up — see N-1 |
| S11 | §1's queue, with priority | yes — see N-4 |
| S12 | phases 0–4 defined; a push after every task | yes |
| S13 | P-F1 | not settled — see B-1 |
| S15, S16, the notes | as the §0.3 table says | yes |

---

## Blocking

### B-1. P-F1's act check has no settled meaning for the roughly 160 uses of acts outside the `WayOut` group, and §6 makes those families halt. The lane would stop at P-F1's first commit.

**What the plan says.**
- P-F1 (v3:325–327): *"Every act the frame performs names what it is in the product's own vocabulary … the link acts … paired with `link_way::WayOut` by an exhaustive `match` … Each act finds its verb and its operand in the printed route by span."*
- §6 (v3:601): *"A family whose acts have no public vocabulary halts rather than spell its wording."*
- Lane P *"writes no product source"* (v3:289).

**What the code has.** Only `WayOut` is a product act vocabulary (`link_way.rs:66-88`). Act uses across the catalog today:

| Act | Uses |
|---|---|
| `Commit` | 28 |
| `MoveOut` | 24 |
| `MoveAway` | 17 |
| `MoveLinkOut` | 16 |
| `PointAt*` | 15 |
| `Revert` | 11 |
| `Mend` | 6 |
| `Remove` | 32 |
| `PutTheFileItself` | 13 |

None of them has a product type. `PutTheFileItself`'s route is not a `WayOut` route at all: it is printed by `milestone.rs:1629`, `finalize.rs:1537` and `setup.rs:2870`. So v3:325 also wrongly pairs it with `WayOut`.

**What follows.**
- P-F1 re-runs every case, and most families would halt on the §6 rule.
- Lane P cannot write the product vocabulary that would unblock them.
- The pairing with `WayOut` checks only a mapping at compile time. At run time the case sees prose (the clause is the private `fn clause`, `link_way.rs:109`). So the §12 repro (*"changing `WayOut`'s choice turns a case red"*) depends on reading the verb from the printed text anyway.
  - The one public way to ask the product which way out it chose is `Link::at(root, at, stands)` (pub, `link_way.rs:99`). It is not named in the plan.

**Fix: settle the semantics before P-F1.**
- The frame's own **act-verb table** — one home, in `route_case.rs`, described in `pinning.md` §6 — is the act vocabulary for every act the product has no type for.
- A case asserts that the act's verb stands at a span of the printed route, together with its operand, and nothing else of the sentence.
- For link-on-the-way cases, additionally assert that the case's act equals the one `Link::at` computes for the planted link.
- Amend the frame rule from *"Must not: assert a route's wording"* to *"… beyond each act's verb and its operand"*. v3's *"The plan's line … stands"* (§0.3, S13) is not accurate as written.
- Drop §6's halt clause.
- D0 records this as the reading of F-C → A (*"checks acts, not wording"*): the act's verb is the one word checked. This is an engineering reading of the ruling; tell the human with H1. It needs no new question.

### B-2. J2-0 counts a part as reviewed if any review record exists for it. The byte table's frame and its first families have none, so J2 would pass about 12 k lines as reviewed while nobody read them.

**Evidence.**
- J2-0 (iii) (v3:442) counts *"the byte table's families (the `DECISIONS.md` entries '… after its review', 2026-10-18 to 2026-10-27)"* as reviewed.
- Every review on record is scoped to a task's diff. The first, `b6a-review.md:1`, reads **`56839f95..f4c12fa3`**.
- Everything before that range has no review record: 12 commits on the byte table from `fc2b13b6` up to and including `56839f95` (the reviewed range starts after it), **+12,265 lines** under `crates/cli/tests/` (`git diff --shortstat fc2b13b6^ 56839f95`). That is the frame, the sandbox reader and runner, the acceptance cells, and the pilot, task and install families (B0–B5, 2026-10-13 to 2026-10-15). No "… after its review" heading exists for B0–B5.
- H-J0 fires only for a part with **no** record, so it never fires for the byte table.

**Why it blocks.** This is v2's B1 again: `wf-test-review-real-run` discharged while code it counts as reviewed was never read.

**The plan's claim about the route catalog is confirmed.**
- No heading for P1–P5 or the 2026-10-12 families (`DECISIONS.md:3944–7053`) records a code review.
- The briefs directory holds reviews only for the byte table (b6–b10), the link fix and plans.
- The 2026-10-12 R0 review read only *"the code the parts run"*.
- Only the link-fix reviews of 2026-10-28 read the route-catalog commits that added `ReplaceWithACopy` and `MoveDirectoryIn`.
- The CI fixes (`936c6d54`, `72ca156a`) also have no review record.

**Fix.**
- Make J2-0 count coverage **by commit range**, not by part. A file's lines in `cbb3d736..HEAD` are "reviewed" only where a recorded review's range covers the commit that wrote them.
- That is exactly the *unreviewed-range lister* v3 parked in Candidates (trigger: the `fix` half's re-review). Build it as J2-0's instrument, in lane U.
- Then H-J0 is no longer a question (see S-3).

---

## Should-fix

### S-1. T-NR's outcome arrives after the review that certifies the stage, and after the document that reports it.

**Evidence.**
- T10a (Phase 2, v3:436) documents *"T-NR's outcome"*. The probe that produces it runs in **Phase 3** (v3:169, 419).
- If the outcome is *"a cell among the endings"*, that is a harness or record-script change landing **after** J2b discharges `wf-test-review-real-run`. The done-condition is met while unreviewed code follows it.

**Fix.** Run the probe as soon as T-NR is built (Phase 2, before M-T), so its record, bound or cell, is inside T10a and J2b.

### S-2. Making the implied classes red is honest only with three things P-F1 does not yet say.

- **Where the change is.** `is_red` (`route_case.rs:913-918`) is the catalog's own; the byte table has its own `Classification`, so nothing else moves.
- **The green cases it flips.** P-F1 re-runs every case and lists what moves — good. It must also say that the pilot's recorded *"183 hold"* (`route-catalog-pilot.md:44`, :277) becomes a count of history, with a bracketed note at the time of the change, not only at P-Z.
- **Two ways of failing kept apart.** *"followed, holds, act implied"* must be a classification separate from *operand implied*, with its own `as_str`. The strict list's *"classification moved"* arm then distinguishes them, and the opening ledger can grade the two kinds separately.

### S-3. H-J0 is not the human's question: F-B → A already answers it. The plan also does not price or schedule the review its recommended answer creates.

**Not a question.** The ruling counts as reviewed only parts *with their own reviews*. A part without one is therefore in J2.

**Not priced or scheduled.** v3's recommendation, *"A: put it in a J2b area"*:
- adds about 3 k lines of `support/route_case.rs`, the fence in `route_catalog.rs` (8.4 k) and the pre-existing families;
- and B-2 adds the byte table's 12 k.

None of this is in §4, and a J2b area over the catalog would have to wait for M-P.

**Better:** review the **frame** — `route_case.rs`, the fence, `refusal_sites.rs` and the byte table's runner — **early**, as part of J2a, before P-F1 and the C-tasks build 220–260 more units on it. A frame defect found at J2b would invalidate them. Price that addition.

### S-4. The T-NR probe is a run of the tool, which the human reserves for their own session.

**Evidence.**
- v3:419: *"the main session invokes `{ probe: 'neutralized', scratch }` once."* That is a Workflow-runtime invocation that launches an agent.
- The human's rule of 2026-10-10 (one run, one session): running or testing the tool is not launched from the build session. The runtime probes of 2026-10-07 predate that rule.

**Fix.** Either hand the probe over to the human like the canary — early, per S-1 — or ask the one question below.

### S-5. The phase and merge order has two gaps (no deadlock).

- **J2a's fixes reach the loop branch only through M-U, and nothing merges loop → T after that.** T's simulation and fences, which use the regression set's and the hygiene scan's stand-ins, meet J2a's changes for the first time in M-T's merge commit. Add a loop → T merge after M-U, or say that M-T's gate is the integration check.
- **Phase 1 writes *"P0 → (H0?) → P1 → (H1) → P-F0"*** (v3:161), which reads as P-F0 waiting on H1; §3 and the table's S15 row say nothing waits. Write it as *"H1 asked; P-F0 does not wait"*.

### S-6. The price's low end per unit is not supported.

**Re-added:** units 218–258; P = 73–125; T = 59–98; J2b = 19–73; T chain 82–179; gates 118–184; calendar 6–21 days. All of it reconciles.

**The 10 minutes a unit is not a measurement.**
- *"3.9 / 0.395 = 9.9"* divides the two parts' pooled clocked minutes by **one** part's share.
- Per part, the pilot's own wall seconds are measured: P3 10,773 s for 10 sites, 18 min a site; P4 12,065 s for 18 sites, 11.2 min a site; together **13.6 min a site** (`route-catalog-pilot.md:68-81`, :489).
- That equals the 2026-10-12 families' 13.6, which I confirmed: `ee4db8f1` at 01:10 to `75754e8f` at 08:54 = 464 min over 34 sites.

**Use 13–18,** plus the act check F-C adds to every case. P becomes about 84–150 h. The high end of the critical path is still the T chain, so the calendar moves little — but H-J0 and B-2's added review are not priced at all (S-3).

### S-7. U3 and U4: keeping them is justified, but U4 must be described as a floor.

**Why they stay.** Both serve something needed before the opening: S1's rule after every push, and the fact sheet's section 0. Both are 1 + 1 gates, last in the queue. That is consistent with F-A → B's consequence ("a task before the opening … needs").

**The problem.** U4's vocabulary (*"names the opening, a stage with real agents, or a join"*, v3:275) misses rows the sheet must still read:
- `decisions-pending.md:276`, *"before round 1's `test` stage"*;
- `:110`, *"before the canary"* (struck by D0, but the class remains);
- conditional triggers such as `:128` and `:169`.

D0-def's *"read first by running `dev/pending-due` and then judged"* would anchor the gatherer on the tool's list.

**Fix.**
- Add *round 1*, *canary* and *first run* to U4's vocabulary.
- Have the tool print the rows it did **not** match, by trigger, so that a judgment covers every row.
- Have the definition say that the list is a floor.

---

## Notes

- **N-1. The price's other figures check out.** Gates per lane, the queue's roughly 35–40 % busy share, and both calendar ends (98 h / 20 h a day ≈ 5 days; 214 h / 12 h ≈ 18 days) all reconcile. J2b's 10–25 fix commits rest on batching by area. The 2026-10-07 re-review found 36 findings over a smaller range, so expect the high end.
- **N-2. J2a's fixes may touch `.claude/settings.json`,** which holds the deny list protecting the human's acts. Route any such fix through the main session and tell the human. It is not a builder's file.
- **N-3. P-Z's *"every family is in `CATALOGUED`, so the vocabulary arm holds every code"* (v3:356) is only true if every code prefix in source belongs to a family.** Add the closure to P-Z's arm: every prefix of a code-shaped literal is in `CATALOGUED`.
- **N-4. The queue covers gates, but not the builders' own red-first suite runs.** The catalog suite (about 270 s on 8 cores) and the simulation still run beside a queued gate, which is the load that produced the 50 ms probe-hold flake and `wf-gate-starved`. Either queue suite runs over about 60 s, or re-run a red gate before judging it.
- **N-5. H0's threshold is sound.** Its basis, 32 of 183 (`route-catalog-pilot.md:275-277`, corrected 2026-10-11), is verified. Its recommendation ("carry on") is right: F-C stands, and H0 is about scale only.
- **N-6. J2-0's class (iv), *"outside the `test` stage, with a reason per path class"*, is a judgment.** Have J2b's cross-area pass read the partition itself.

---

## The one question that is genuinely the human's

**May the build session invoke the single T-NR runtime probe?** It is one agent, bounded, and launched with no run. Without this, it is handed to the human's session like the canary.
- **A, recommended:** hand it over early in Phase 2. The human runs it in their own session, so their rule stands with no exception, and its outcome lands before T10a and J2b (S-1, S-4).
- **B:** the human grants the build session this one exception.

The rest are engineering calls:
- **B-1's verb table** is the plan's reading of F-C → A; tell the human with H1.
- **H-J0** is answered by F-B → A (S-3).

---

## Tools

- **Count the catalog's act uses by kind.** It answers *which acts have no product vocabulary*. From `crates/cli/tests`:
  ```sh
  for a in Remove Run RunFilled RunChosen RunNamed RunCited MoveAway MoveOut MoveLinkOut PutTheFileItself ReplaceWithDirectory ReplaceWithACopy MoveDirectoryIn PointAtCommitted PointAtNew PointAtScript Commit Mend Revert; do printf '%s %s\n' $a "$(cat route_catalog.rs route_catalog/*.rs | command grep -c "Act::$a(")"; done
  ```
  P0's projection could print it per family.
- **Find a part's unreviewed range.** `git diff --shortstat <first commit>^ <first reviewed range's base>` showed B-2. The unreviewed-range lister (Candidates) is the tool J2-0 needs now, not at the `fix` repair.

<!-- end of report -->
