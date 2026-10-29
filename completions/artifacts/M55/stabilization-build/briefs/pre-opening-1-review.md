# Review of `f99658b8` (parent `495991e3`): `not-comparable`, the stale line 706, and the ruling's order

Read-only review. Line numbers are at `f99658b8`. Nothing in the repository was edited. Tests ran in a detached worktree under a scratch root, which has been removed:

- `cargo test -p jigc --test g_tooling -- not_comparable regression unknown`: 49 passed, 0 failed. This includes the two new `dev_stabilize_record` arms and the new fence.
- `… -- a_return_is_held_to_its_keys_its_binaries_and_its_commit_before_it_is_taken self_test`: 3 passed. This is the simulation arm plus the harness self-test.
- `dev/stabilize-harness --stdout` compared with `cmp` against `.claude/workflows/stabilize.js`: identical.

**Verdict.** The one meaning is carried end to end. No reader is left that takes `"not-comparable"` as truthy, as `false`, or as unknown. The doc's line 706 is true, claim by claim. The routing works as declared. What does not hold is the reasoning the `DECISIONS.md` entry gives for that route: it is true for round 1 only (H1).

---

## Holes

### H1 — The route is argued from "worked nowhere before", which is false after round 1. A break the run's own fixes cause at a new door goes to the human, not to a fixer.

**Where.** `DECISIONS.md:9`, *(1), the form*, says:
- *"ruling 4 fixes a regression unasked wherever it was found because it breaks what worked before; a door the previous release lacks worked nowhere before"*, and
- *"by ruling 10 round 1's test set is the delta against the previous release, so a new door is inside it by construction"*.

**The rulings this is checked against.** Ruling 4 (`DECISIONS.md:9497`) fixes *"a regression the run's own fixes introduced"* without asking, wherever it was found. Ruling 5 measures *"a regression of the run"* against **the previous release's binary only**.

**What the code does.**
- `route_of` (`dev/stabilize-record:5265-5310`) sends a verified finding to a fixer regardless of its door only when `grade == "regression"`.
- That grade is given only for `regression is True` (`:4660`, `:6670`).
- `not-comparable`, like `false`, falls to `("fix","inside") if inside else ("human","outside")`.
- The harness builds only the candidate and the previous release (`buildBoth`, `.claude/workflow-sources/stabilize.js:3871-3877`). No earlier candidate of the run is ever driven.

**The case, constructed.** A later release adds a verb `X` that the previous release lacks. Candidate `c1` handles `X` correctly. In round *n*, a fixer fixes finding F at door A. The audit of that fix diff, or round *n*+1's test, then finds that the fix broke `X`, and `X` is not in that round's committed door list.
- The verifier can only answer `regression: "not-comparable"`, because `X` does not exist on the previous release.
- `inside` is false, so the finding is routed `("human","outside")`.
- The same break at an old door would be green on the previous release and red on the candidate, so `true` → `("fix","regression")`.

This is exactly ruling 4's *"a regression the run's own fixes introduced"*. `X` did work before — on `c1`. The second argument in the entry (*"inside by construction"*) holds only for round 1, because ruling 10 shrinks the test set in every later round.

**How big the class is.** From reading `route_of`, `graded_by_the_row`, `read_state:6670` and `buildBoth`: every confirmed finding whose previous-release fact is not `true`, but which an earlier candidate of the run passed, is routed by its door. That is two values:
- `not-comparable`: a new door;
- `false`: an old door that was red on the previous release, fixed on `c1`, and broken again by a fix of the run — a re-broken fix.

These two values meet two places: the `test` stage in rounds ≥2, and the audit of the fix diff (`runFix` → `triagePasses`, `stabilize.js:5354`). The second is unreachable today, because `fix` is in `NOT_FIT` (`:1354`).

**Not introduced by this commit.** Before it, the same finding went through as `false` with the same route. What the commit adds is a recorded justification that closes the question as settled.

**Consequence.** Nothing is lost: the finding reaches the human's list, the stricter route. The cost is an extra ruling per item, plus a ruling at odds with ruling 4's text.

**Smallest correct fix.** Correct the two sentences in `DECISIONS.md:9`:
- limit *"inside by construction"* to round 1;
- say that a break the run's own fixes cause, where the previous release is not green, cannot be detected by ruling 5's fact, and is routed by its door.

Then record the gap as a `decisions-pending.md` row (Q1 below). Detecting it needs a second comparison binary, the earlier candidate of the run. That is a new mechanism, so it is the human's to decide.

---

## Notes

**N1 — Line 706 is true and incomplete about `VERIFY_PASSES`.** The line (`implementation/stabilization-workflow.md:706`) says `VERIFY_PASSES = 3` *"remains on the path of a run without an order"*. That is true for `test` (`stabilize.js:5144`, and `finishTriage`, called at `:4867` only after `continuing()` returns `[]`, i.e. without a queue). But `runFix` calls `triagePasses` (`:5354`) and `finishTriage` (`:5442`) with no `hasQueue` guard. So when `fix` is lifted from `NOT_FIT`, a run with an order also takes the `VERIFY_PASSES` loop. The plan's `T12` (`reshape/plan.md:221-223`) owes that repointing. Fix: add *"and on the `fix` stage's path of every run, unreachable while `fix` is refused (`T12`)"*.

**N2 — Two sibling lines of the same doc still describe only the old loop.**
- `:140`: step 6, *"Triage and verify-real, alternating"*.
- `:275`: *"So triage and verification alternate — as many times as the harness's `VERIFY_PASSES` says"*.

Both are unqualified. A run with a queue (`:301`, `:706`) triages once and verifies in waves. Found with `command grep -n -i 'alternat\|VERIFY_PASSES\|not built yet'` over the doc: these 2 lines plus 706. This is an instance found by that grep; the doc was not swept for every queue-blind sentence. Natural owner: `wf-test-review-real-run`.

**N3 — The mixed-type enum is new to the harness's schemas.** `VERIFY_SCHEMA.regression` is `{ enum: [true, false, 'not-comparable'] }` with no `type` (`stabilize.js:2406`). `command grep -n 'enum:'` finds it is the only schema property without a `type` and the only mixed-type enum. Its red/green evidence comes from the simulation's own validator (`tooling-tests/fixtures/stabilize-runtime.mjs:713-716`). The doc itself says (`:509`) that a stand-in is not the runtime. It is valid JSON Schema; whether the Workflow runtime's structured return accepts it has not been observed. Reported as a lead.

**N4 — The word "door" in step 4 is not this run's door vocabulary.** In the run, a door is a command (`jigc doc show`). `git diff jigc-v1.0.0-rc.24 f99658b8 -- crates/cli/src/cli.rs` adds no subcommand: `uninstall` is already present at rc.24. So, read literally, `not-comparable` cannot occur in this run. The ambiguous case is a block that uses a flag or behaviour that is new on an old command: the verifier must choose between `false` and `not-comparable`. Both route the same way, so only the recorded fact differs. Optional fix: one clause in step 4, *"the door, or a step of the block, the previous release does not have"*.

**N5 — The fence is presence-only.** `stabilize_harness_fence::a_door_the_previous_release_does_not_have_is_not_comparable_in_the_definition_the_record_and_the_harness` checks that strings are present in three files. Behaviour is held by the two record arms and the simulation arm, so this fence is a cross-home spelling check, which is acceptable as such.

**N6 — Left alone correctly.** Historical records (canary reports with `regression: false`) are untouched. That is right: they are records, not readers.

**N7 — Every reader of `regression` was traced; none is truthy or treats the value as `false` or unknown.**
- Count: `command grep -n '"regression"'` over `dev/stabilize-record` gives 24 hits. Separately, `regression` over the harness source has 4 code readers (`regressionOf` 1545, `triageRecord` 2188, the schema 2406, and the assignment at 4247), and `dev/stabilize-step` reads only the `regression_unknown` count.
- The old truthy reader at `read_state:6670` (`if said_["regression"]`) is now `is True`. That was the one place where a string would have graded `regression`.
- `graded_by_the_row:4660` compares with `== "yes"`.
- `regression_unknown:6810` tests `is None`.
- The view's dict (`:5899`) has a `NOT_COMPARABLE` key.
- The reverse cell map at `:6493` is total over `REGRESSION_CELLS`.
- `sealed:5588` and `verdict:4810` use `a_regression_fact`.
- The queue path's `VERDICT_SCHEMA` carries no `regression`.
- `1` and `0` are refused (`isinstance(1, bool)` is False).

**N8 — Hygiene and pronouns.** The added lines contain no host path, name or address (`command grep` over `+` lines: 0 hits), and use no he/his/him (0 hits). Gendered references to the human do remain in lines this commit did not touch, which `e5e323fc` aimed to remove everywhere:
- `stabilize.js:797`, `:1900`;
- `dev/stabilize-record:1245`, `:5285-5286`;
- `dev/stabilize-step:478`, `:590`.

That list comes from `git grep -w -E 'he|his|him'` filtered on *human / his ruling / he reads* over those three scripts and the verifier definition. It is an instance, not a bounded class.

**N9 — Item 4 of the brief is met.**
- `DECISIONS.md` records ruling A, its order and the why given in the brief. The form section is argued and is not attributed to the human.
- `decisions-pending.md`: `wf-regression-not-comparable` is struck as DISCHARGED; `wf-route-walk` and `wf-whole-canary-after-parts` are annotated with the order and not struck; `wf-test-review-real-run` is new.
- The new row's claim that *"the last re-review on record found the `test` half fit for a canary run, not yet for the real run"* matches `DECISIONS.md:8426`. No later heading re-reviews the `test` half for a real run; the 2026-10-12/13 reviews are scoped to the parts.
- It says nothing beyond what was ruled, except H1's sentences.

## Line 706, claim by claim (all true at `f99658b8`)

| Claim | Where it was confirmed |
|---|---|
| `run-set`, `budget`, `hand-on`, `order`, and the *THE QUEUE* header | `dev/stabilize-record:815-847`, `:2562-2591` |
| `hasQueue`, `triageOnce`, `verifyWaves`, `verifyThenRecord` | `stabilize.js:1691`, `:4271`, `:4320`, `:4416`; the `test` branch at `:5072` |
| The *"verify phase"* and *"filed when it returns"* entries | headings at `DECISIONS.md:5171` and `:7238` |
| The route hunter is not built | no agent file matching `route` or `hunt`; no `hunter` or `route-walk` in the harness |
| `VERIFY_PASSES = 3` and `triagePasses` | `:778`, `:4158`, `:5144` (see N1) |
| The route catalog is merged | `wf-route-catalog-merge` DISCHARGED as `e771c46d` |

## Questions for the human

**Q1 (from H1).** By ruling 4, a break the run's own fixes cause is fixed without asking. When that break is at a door the previous release lacks (`not-comparable`), or at one that was already red there (`false`), ruling 5's fact cannot see it, and it is routed by its door: to the human when it is outside the round's test set.

Recommended robust end state: a confirmed finding whose previous-release fact is not `true` is also driven on the candidate the round started from. If it is green there, it is graded `regression` and fixed unasked. This needs a third binary per verifier: a new mechanism, so it is the human's.

The alternative is to accept the human-list route as declared and correct the `DECISIONS.md` reasoning only. Either way, the reasoning at `DECISIONS.md:9` should be corrected.

<!-- end of report -->
