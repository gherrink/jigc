# Review of the second repair plan of the `test` half

*Independent, read-only, at `a7d15955` (scripts, harness and suites are byte-identical to `cbb3d736`). Subject: `completions/artifacts/M55/stabilization-build/re-review-test-half/plan.md`. Four things were driven in light rigs built from review A's appendix helpers, under a minted `<scratch>` root. Everything else is read from source and says so. Returned 2026-10-07 by a `design-reviewer`; the reviewer wrote no file, so this text was transcribed from its return by the orchestrating session.*

## Verdict

**Build after the named changes.** P1, P2, P4 and P3(a) are structural; P5 and P6 are patch lists under a heading, and P3(c)–(e) are small fixes. `T1`, `T3`, `T4`, `T8` can start once reworded; `T5`, `T6`, `T9`, the canary and `T11` cannot be built as cut.

## Blocking

**B1 — Fork 2, option A, makes the stage it grants unrecordable.** Plan §6 Fork 2, `T5`.
- Driven: round 1 with two halted attempts; the grant recorded in a batch that carries `gate-set` for the commit the tree stands on. `record` → `recorded`, push → `ready`, `next = test`, attempt 3.
- The granted attempt's candidate is the new tip, because the record commit moved it. Its batch opens with `gate-set` (`stabilize.js:1380`) and is refused: *"`r1/gate.md` holds the gate of … — a round tests one candidate"*, exit 6 (`dev/stabilize-record:3369-3370`).
- So the halt comes at the record, after every instrument ran. `T4`'s new rule (a round's candidate is the commit its gate ran on) makes the collision an invariant.
- `T5`'s *done when* ("ends `ruled`") passes over it.
- Fix: the commit step compares against the pre-batch gate's file directly and writes no `gate.md`; or a round's gate is replaceable while the round is untested. The *done when* must drive the granted stage through its own record.

**B2 — P2 scans at the commit; the irreversible act is the push.** Plan P1, P2, `T1`, `T3`.
- Driven: a report with the denylisted term and a host path, committed under the run directory by plain `git add` and `git commit`. `git-state` → `ready`, `pending: null`. `push` → `ready`. The bare remote holds the text and the subject.
- P1's cell "ahead by commits that touch only the run's directory → the push is made first" turns this into the first act of every invocation.
- P2's claim that "the boundary check needs no trust" is true only of commits the act itself makes.
- Fix: `vet` runs over the blobs and subjects of `origin/L..L` at every push. The owed-push cell admits nothing unvetted.

**B3 — A vet hit has no exit.** Plan `T3`.
- A report is written once, so the script will not replace it.
- The next invocation gates and asks `record` again, and gets the same refusal.
- If a hand deletes the file, the batch's stored `check-reports` result still says `ok`.
- The table P1 builds has no cell for this. Fix: design the cell, and vet at the first report check so the reporter's item goes void instead.

**B4 — P4's headline rule is not what `T6` builds.** Plan P4, `T6`.
- The rule: after an instrument is launched, only a returned-wrong hash or a failed record tool step halts.
- Still halting after the instruments:
  - every ending of triage (`stabilize.js:2342`; `T6` itself keeps "a triage that does not balance still halts");
  - a returned-wrong hash from one agent (`:2316-2317`);
  - a batch refused for one entry's text — hygiene, the fence, a cell the reader refuses — which loses the stage, and the next attempt repeats it (`:1902-1904`).
- "As ruling 11 has it" is wrong: the ruling says every agent asserts the hash, not that the stage halts. The plan's own argument for an absent hash applies word for word to a wrong one.
- Root: findings live only in returns until the batch. That is the largest member of `C6` left in `test`, and the plan parks its fix (entries reach disk through the script from the agent that made them) in `T11`, "only if a payload failed".
- Fix: state the rule with its real exceptions, or decide that design before `T6`.

**B5 — The canary cannot show what `T11` waits for, and its plants mask each other.** Plan §5.
- The ledger is seeded with settled rows "so that triage is handed only the planted ones". Triage's return and the record's payload are then small by design. This is the masking review B named for the state relay, moved one step.
- The denylist plant reaches triage through the reviewer's return, then the ledger row, then `apply`. That refuses the whole batch, which halts at the record before the rejected push and the red candidate are reached.
- Nothing plants a dead agent or two (item 9), or a record gate that is newly red and the pending batch after it. The first plan's canary list had the last.
- Part C is nearer eight invocations than five, each a full gate: the void CI item alone adds the go, a re-run, a ruling and a granted re-run (driven: reachable).
- Unstated gaps: the real remote's rules and CI on each pushed record; the real denylist; permission state of the real checkout against a clone's; rounds after the first.

**B6 — `T11` is unreviewed code on the real run.** Plan §4, "Then, in this order".
- It lands after the third review and after the canary, before the record that lifts the rule. It may hold a redesign of the payload.
- That contradicts the ruling that a half is used only once repaired and re-reviewed, and the plan's own line that the canary is worth its hours only on the code the real run will use.
- Fix: say what `T11` may hold without review (numbers). Anything else is a fourth cycle, which the three-cycle bound returns to the human.

**B7 — Three tasks are cut on runtime facts that minutes would show.** Plan §5 Part 0.
- `T6` makes fields `required`. Nobody knows what the runtime does on a violation. If it returns nothing, a missing field becomes three full re-runs of a reviewer and a count toward the breaker (`stabilize.js:1793-1811`).
- `T9`-B's slices meet a two-minute default tool timeout and a ten-minute ceiling.
- Part 0 "wants nothing", yet it measures a relay "at the real run's size" only with a seeded, opened run and Fork 4 answered.
- Fix: Part 0 gains four probes — a return missing a required field; a relay at size; a here-document payload at size, hashed; a long command held in slices. The plan states what each result changes in `T6`, `T7`, `T9`.

**B8 — Three *done when* and write-set defects.**
- `T1`: a kill before the journal is written ends at the state *before* the batch (review A's own table), never at "the state document of the uninterrupted control". Two end states are needed.
- `T1`: nothing pins that wrong-branch is refused before any write, fetch or push. Canary safety rests on that order, which was driven on the code before `T1` rewrites `git-state` to finish steps itself.
- `T9`-B: a green row's detail must be `-` (`dev/stabilize-record:2772`) and no round fact exists for a list hash (`:1021`). The task needs `REC`, and a way for the harness to know which check is scripted.
- `T6`: "a verdict without its regression fact is unverified" contradicts `finding-verifier.md:19,27`. *Not comparable* is `regression: false`, with no previous hash required. Every confirmed finding on a new door would go to the human as unverified. Fix: an orchestrator step so the verifier always returns the previous binary's hash, as `O2` does for the reviewer.

**B9 — The canary's opening has no author.** Plan §5.
- "The opening as the real run's will be drafted" needs the items, the clauses, the ledger seed and a regression item. No task or step produces them.
- The opening is the human-led step that the order of 2026-10-06 puts after the canary.

**B10 — `R5`'s class is not closed.** Plan P5, `T4`, §8.
- Driven: one letter wrong in an item's *door*. `item-set` exits 0, the item is `selected: false`, `next = close`, the clause is green. The control with the door spelled right answers `retest`.
- Source: `dev/stabilize-record:2705-2707`, `2725-2727`.
- It cannot be told from a legitimately unselected item. So it must be declared in §8, and the stage's return must name never-selected in-scope items and uncovered doors. The second is P3(c), which today forbids nothing.

## Advisory

- **P1 siblings not caught.**
  - A kill *during* a git child leaves an index lock; the class is not finite as claimed.
  - `stopAfter: 'state'` over an owed push.
  - "Ahead by anything else → the human's" turns every unpushed tuning commit into a human halt.
- **The `fix` half.** The table has four facts and no intent. `land`, `carry` and a round branch need dimensions it lacks. Key it by open intent now, or "rows of this table" becomes a second table.
- **P3(a) covers one act.** Refusal lines and `record`'s three red lists are relayed free text too.
- **P4 over-claims** closing review B's list of never-executed lines. A failed relay, a thrown call, a report nobody launched and an executor that halts have no home.
- **`T5`'s count rule** refuses every later record of a round once a tuning commit removes a test. The lead it answers found no mechanism.
- **`T8` needs two real runs**, not one. Rows must exist before the run that shows `excluded` equals them. An exclusion row has no form in the list's grammar yet.
- **`O5`** is "after part 0" in the table and "after the canary" in §5.
- **`T7`** rewrites what the harness reads after `T2`, `T5` and `T6` built on the old contract.

## The forks

1. **Not ripe.** `T9` reads as the harness driving the act; `O3` reads as the preflight holding it. If the preflight holds it, B is the gate's existing pattern plus a result read by script — smaller than M–L, and the missing option D. If the harness holds it, a detached process must outlive its agent, which is unobserved. A-versus-script is answered by `C6`.
2. **Not the human's.** A is the ruling as worded; see B1 for its mechanics.
3. **The human's.** Sound. Say that `T1`, `T4`, `T5`, `T7` add hand-written writer cells that B later rebuilds.
4. **A request, not a fork.** `root` is correctly rejected: a role agent's shell starts in the session's directory at every call. Add a headless session launched by the orchestrator as the real alternative. Results can be files under the canary root; no relay is needed. Canary *setup* run from the wrong directory writes into this working tree, which the safety argument does not cover.
5. **The human's.** Sound. One run shows one outcome, not "each way to fail".

## Buried as settled

- The canary's size and its departure from the approved one-door shape.
- What becomes of the product findings a real-delta canary makes.
- The opening drafted before the canary.
- The automatic push.
- A returned-wrong hash halting "as ruling 11 has it" — the ruling does not say so.
- Ruling 9's resume by run id for a killed run, narrowed to never (§8 declares it never used).
- Accepting §8 for a public first run.

## Not checked

No agent, runtime, gate, simulation or self-test was run; the plan's §1 reproductions were taken as reported; `runFix`, the regression tool and the definitions were read only in part; lesson 6's source was not found; process survival across agents is unknown.
