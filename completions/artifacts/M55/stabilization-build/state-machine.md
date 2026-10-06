# Review — the state machine of a stabilization run (`dev/stabilize-record`, `357ab425..f6918aaa`)

Read-only review of committed state at `f6918aaa`: `dev/stabilize-record` (2220 lines), its suite
`crates/cli/tests/dev_stabilize_record.rs` (7761 lines), and `.claude/workflows/stabilize.js` only where
the script's contract with it decides a finding. Judged against `DECISIONS.md` → *2026-10-05 — The
stabilization workflow, as ruled* and the seven build entries above it.

**Verdict: red.** Three ways to a wrong `close` or an unbounded step were constructed with the script's
own writers; two of them are reachable through the harness as committed.

## How the repros were driven

Everything ran in throwaway roots minted with `mktemp -d` under this directory; nothing ran in the
repository's working tree, no gate ran, the suite was not run.

- `build-review/mkrig <label>` mints a root holding the committed bytes of `dev/stabilize-record`,
  `dev/hygiene-scan` and `.gitleaks.toml` (each from `git show f6918aaa:<path>`), one opened run
  `run1`, a synthetic denylist, a stub `gitleaks` that keeps its exit contract, and two wrappers:
  `./sr <argv>` (the script, hermetic env) and `./st [field…]` (selected fields of `state`).
- `build-review/lib.sh` holds one-line helpers; each is exactly one call of the script:
  `opening X` = `run-set` with `{X, previous, previous-commit, scope: delta}` ·
  `clause C S [SHA]` = `clause-set --clause C --instrument … --scope s --status S [--commit SHA]` ·
  `scope N INC [EXC]` = `scope-set --round N` · `finding K DOOR [ROUND]` = `ledger-add` ·
  `triage N JSON` = `triage-set --round N` · `lset JSON` = `ledger-set` · `facts N JSON` =
  `round-set --round N` · `items JSON` = `item-set`. `C1`…`C5` are forty `1`…`5`; `F1`/`F2` forty
  `a`/`b`.
- Every repro starts with `cd build-review && . ./lib.sh && newrig <label>`.

Line numbers are `git show f6918aaa:dev/stabilize-record` (S), `…:.claude/workflows/stabilize.js` (H),
`…:crates/cli/tests/dev_stabilize_record.rs` (T).

---

## Findings

### F1 — HIGH — false close: a hunt that never ran to its end is turned green by a later round that ran other items of the same clause

**Where.** S:1388-1423 (`clause-set` replaces the one row a clause has) · S:1811-1863 (`close` reads that
row) · H:564-573 (`clauseRows`: a row per clause *from the units this round ran*) · H:1959, 2034-2046
(units = the items selected this round). Precedence S:1854-1861 (`fix` before `retest`) is what makes it
the common order.

**Repro (run; rig `rig.e2.*`).**

```
opening '"stop":"at-the-bound","rounds":3'; clause no-lost-files void
items '[{"item":"gate","kind":"check","clause":"no-lost-files","runs":"every-candidate","brief":"b"},
        {"item":"review-setup","kind":"review-row","clause":"no-lost-files","runs":"in-scope","doors":["jigc setup"],"brief":"b"},
        {"item":"review-show","kind":"review-row","clause":"no-lost-files","runs":"in-scope","doors":["jigc doc show"],"brief":"b"}]'
scope 1 '["jigc setup","jigc doc show"]'; finding f-1 "jigc doc show"
triage 1 '[{"key":"f-1","grade":"breaks","verdict":"confirmed","regression":false}]'
# round 1: review-setup did not run to its end -> the harness writes void (clauseRows, run under node:
#   r1 -> status void, instrument "review-setup, review-show, gate")
./sr clause-set --run run1 --clause no-lost-files --instrument "gate, review-setup, review-show" --scope "round 1" --status void --commit $C1
facts 1 '{"candidate":"'$C1'"}'                      # next = fix   (retry: due, but fix comes first)
lset '[{"key":"f-1","disposition":"fixed","detail":"'$F1'"}]'; facts 1 '{"cycles":1}'   # next = test
scope 2 '["jigc doc show"]' '["jigc setup"]'         # the fix's door only: selected = gate, review-show
# round 2 as clauseRows computes it (run under node: r2 -> status green, instrument "review-show, gate")
./sr clause-set --run run1 --clause no-lost-files --instrument "gate, review-show" --scope "round 2" --status green --commit $C2
facts 2 '{"candidate":"'$C2'"}'; ./st next forbids_close clauses
```

*Expected:* closing forbidden — `review-setup` has no run to its end on any candidate (ruling 16: *a void
or missing row forbids closing*; cell 3: that instrument is run again once).
*Got:* `next = "close"`, `forbids_close = []`, the clause `green`, `round 2`, `behind 0`, `stale None`,
`retry None`.

**Class, and its bound.** One row per clause, rewritten by whichever subset of the clause's items a round
ran. A check is not needed: two hunts suffice (above, `review-show` alone erases `review-setup`'s void).
Members: every clause judged by two or more items of which at least one is `in-scope` — derived from the
mechanism (`clauseRows` groups by `unit.clause`; `clause-set` holds one row), not from a census of a real
test set, because no run is opened. The exit rule's clauses each have many items (review rows, trial arms,
audit areas), so this is the dominant shape, not a corner. The build entry's own *Not built* bullet
discloses half of it (*a hunt's earlier green is no longer on record … the check decides*); the erased
**void** is not disclosed.

**Smallest change.** Evidence per item, the clause's status derived (worst over its items, each with its
own commit and staleness) — the entry already names it (*a table per item would hold both*). A stopgap
that needs no new table: `clauseRows` is handed the state's rows and writes `void` when the standing row
is not green and not every item that judges the clause ran this round.

---

### F2 — HIGH — false close: a fix on the ledger with no counted cycle leaves the pre-fix candidate "current"

**Where.** S:1744-1749 (`fixed_in`: cycles ≥ 1 or an outcome — the ledger is not asked) · S:1831
(`current`) · S:1697-1698 (`fixed` → recorded). Harness path: H:2182 (`audited = !fixerForks.length && …`)
· H:2221 (`facts: audited ? {cycles} : null`, **patches written either way**) · H:2367 (*the human rules;
the next `fix` carries the ruling*) · H:356-362 (any key may be ruled `later`/`bound`) · H:2315 (no
blocker left → returns `nextOf(state)`).

**Repro (run; rig `rig.e1.*`).**

```
opening '"stop":"at-the-bound","rounds":3'; clause clause-a void; clause clause-b void
items '[… gate (check, clause-a, every-candidate) …, … review-setup (clause-b, in-scope, "jigc setup") …]'
scope 1 '["jigc setup"]'; finding f-1 "jigc setup"; finding f-2 "jigc setup"
triage 1 '[{"key":"f-1","grade":"breaks","verdict":"confirmed","regression":false},
           {"key":"f-2","grade":"breaks","verdict":"confirmed","regression":false}]'
clause clause-a green $C1; clause clause-b green $C1; facts 1 '{"candidate":"'$C1'","binary":"<64 b>"}'   # next = fix
# cycle 1: the fixer of f-1 commits F1; the fixer of f-2 halts on a new mechanism (a fork).
# H:2221 records the patch and NO cycle:
lset '[{"key":"f-1","disposition":"fixed","detail":"'$F1'"}]'          # next = fix, blockers [f-2]
# the human rules the fork "record for a later release":
lset '[{"key":"f-2","disposition":"later","detail":"needs a new mechanism"}]'
./st next stop candidate fix_rounds forbids_close position
```

*Expected:* never `close` — commit `F1` is on the round's branch, audited by nobody, landed nowhere, and
the candidate `C1` was tested before it (decision 1: *close is reachable only from a tested candidate*).
*Got:* `next = "close"`, `candidate.current = true`, `fix_rounds = 0`, `forbids_close = []`,
`position.fix = {round 1, cycle 1, land: false}`.

In the mode that stops after every round the same state answers `stop` with `then: close`; the go can
only be recorded on the loop branch (H:1879), where the round's records are not, so the run sits between
a loop branch that says `fix` and a round branch that says `close`, and the `fix` stage returns `cycle`
with neither exit named (the exits are printed only at a `bound` halt, H:2246).

**Also reached without a fork** — the same state for as long as a cycle's record is half written: the
record's order is `ledger-set` (dispositions) → `triage-set` → `round-set {cycles}` (H:1273-1277), and
each is its own call, so a kill or a refusal after the first leaves it (F4 drives one).

**Class, and its bound.** "A fix stage is on record" is read from `round.md` alone. Writers of a `fixed`
disposition, enumerated by `grep -n "disposition: 'fixed'\|repointPatches\|reopenPatches" stabilize.js`:
the cycle record (H:2349 → 2221, counted only when audited — so not when a fork stands beside a fix),
the part's repoint (H:605, 2271, audited), and any direct `ledger-set`, which is unbounded.

**The suite pins it as correct.** T:4801-4806, *every blocker fixed, every item ruled, one found again*:
`in-fixed` (T:4426, `fixed: 0f34d8f0`) under `TESTED_ONCE` → `next: "close"`. T:5045-5051 (*a void clause
row, every finding fixed*) asks the clause table on the same state and expects `retest`.

**Smallest change.** The round's record says a fix exists from the first `fixed` patch on — a fact
written in the same step, before the dispositions — and `fixed_in` reads it; a cycle that is recorded and
not audited then reads as *land owed after an audit*, never as a current candidate. Deriving it from the
ledger instead (a `fixed` row whose latest triage is in the latest round and was not found with that
fix) misses rows no round triaged.

---

### F3 — HIGH (a loop no bound stops) — `triage` has no bound and no way to the human

**Where.** S:1705-1708 (`ungraded`/`unverified` → `triage`) · S:1850 · S:1964-1970 (the position hands
the finishing stage a new attempt every time). Decision 7 of the newest entry took the old exit away (*an
unverified finding … is not handed to the human*). H:2298 stops a second pass only inside one invocation.

**Repro (run; rig `rig.e6.*`).**

```
opening '"stop":"at-the-bound","rounds":1'; clause clause-a void; items '[gate …]'
scope 1 '["jigc setup"]'; finding f-1 "jigc setup"; triage 1 '[{"key":"f-1","grade":"unclear"}]'
clause clause-a green $C1; facts 1 '{"candidate":"'$C1'"}'; ./st next position
# three finishing invocations whose verifier could not drive the finding: a report, the same entry again
for a in 2 3 4; do printf '# verifier halted\n\n<!-- end of report -->\n' | ./sr report --run run1 --round 1 --stage test --reporter verify-f-1 --attempt $a
  triage 1 '[{"key":"f-1","grade":"unclear"}]'; ./st next position; done
```

*Expected:* after the stage that finishes a triage has run and left the same finding without a verdict,
a stop or the human's list. *Got:* `next = "triage"` every time, `position.test = {round 1, attempt 3 →
4 → 5, triage: true}`, `stop = null`; nothing in the document counts the laps. Each lap is a build and a
record gate.

**Class.** The steps with no counter: `triage` (this), and `test` on a round whose test stage never
reaches its record (S:1848, attempt + 1 forever — but that one returns a halt with its reason each time).
Enumerated by reading `next_of`: of nine values, `fix` is bounded by the cycle and round bounds, `retest`
by `spent` (see F8 for its hole), `stop`/`rule`/`unsettled`/`not-ready`/`close` hand over; `triage` and
`test`-again are the two without a bound.

**Smallest change.** A finding a finishing invocation recorded again without a verdict is the human's
(`rule`, why `unverified-after-retry`): the round's record counts the finishing passes, or the triage row
carries the attempt.

---

### F4 — MEDIUM — `triage-set` refuses with "nothing was written" after writing the triage record, and `state` then reads the stale grade without a word

**Where.** S:1656-1657 (two `publish` calls, each with its own scan) · S:845-846 (the refusal's text) ·
S:457 (*all of a batch is written, or none of it*) · S:2011-2032 (`state` holds a triage row to its doors,
never to the ledger's grade).

**Repro (run; rig `rig.e14.*`).** A finding refuted in round 1 (`recorded: not-a-break`, `next = close`);
the scanner answers once and then cannot run (a stub that exits 1 from its second call on):

```
echo '[{"key":"f-3","grade":"breaks","verdict":"confirmed","regression":true}]' | ./sr triage-set --run run1 --round 1
```

*Expected:* exit 12 and both files as they were, or both written. *Got:* `refused did-not-run: … so
nothing was written`, exit 12 — and `r1/triage.md` holds `| f-3 | inside | breaks | confirmed | yes |
open |` while `ledger.md` still says `refuted`; `state` exits 0 with `next = "close"`,
`forbids_close = []`. The same picture with a kill instead of a refusal, inside a fix cycle's record
(rig `rig.e13.*`): dispositions written, the audit's confirmed regression in `triage.md` only, cycles
not yet written → `close`.

**Class.** Multi-file writes inside one call: `grep -c "publish(" ` over the subcommands gives one call
with two (`triage_set`); every other writer has one. Multi-call records (the harness's, H:1264-1279) are
not atomic by design; their windows are listed under *Crash consistency* below.

**Smallest change.** Fence and scan both texts before writing either, and let `state` refuse (`corrupt`,
*repeat triage-set*) a latest triage row whose grade the ledger does not carry unless the row's grader is
`human`.

---

### F5 — MEDIUM — `alone` is taken in any state, and rows written by a re-run launder `not-scoped`: after a landing the run closes with no scope ever derived from the fixes

**Where.** S:1544-1545 (`alone`: a slug, nothing else asked — `go` and `granted` are held to the state at
S:1563-1564, `alone` is not) · S:1752-1775 (`stale_of` looks only at fix rounds *since the row's own
round*) · H:1951-1954 (`clause` accepted whenever `position.test` is not refused; nothing asks that
`next` be `retest`).

**Repro (run; rig `rig.e3.*`).** Round 1 tested and landed (`cycles: 1`); `next = test`,
`clause-a: check-always`, `clause-b: not-scoped`. Then the `test` stage is invoked with `clause`:

```
scope 2 '["jigc setup"]' '["jigc rename"]'; clause clause-a green $C2; facts 2 '{"candidate":"'$C2'","alone":"clause-a"}'   # next = retest [clause-b]
scope 3 '["jigc setup"]' '["jigc rename"]'; clause clause-b green $C3; facts 3 '{"candidate":"'$C3'","alone":"clause-b"}'
./st next forbids_close
```

*Expected:* `not-scoped` until a round resolves a scope of its own after the landing (header S:339-343:
*a round that ran one clause's instrument alone names the round before's doors again: nobody has said
which doors those fixes reach*). *Got:* `next = "close"`, `forbids_close = []`. `review-rename`, whose door
round 1 excluded, never ran, and nothing asks whether the fix reached it.

**Reach.** One wrong argument on a `test` step; the harness does not refuse it. **Class:** the facts of a
round record that record a decision — `go`, `granted`, `alone`, `outcome` (read off `ROUND_FACTS`,
S:618): two are held to the state, two are not. **Smallest change:** take `alone` only for a clause the
state lists as due, or count a fix round that no whole round followed against every row written after it.

---

### F6 — MEDIUM — a row that is gone is not missed: a deleted clause row and a deleted ledger row answer `close`

**Where.** S:978-1004 (a damaged row is refused; a missing one cannot be) · S:1731 (the only census: at
least one clause row) · S:2150-2158 (`check-ledger` checks the keys it is handed).

**Repro (run; rig `rig.he.*`).** Two clauses (`clause-b` red), one ungraded finding: `next = triage`,
two entries in `forbids_close`. Delete the `clause-b` line from `clauses.md` and the `f-1` line from
`ledger.md` by hand: `next = "close"`, `forbids_close = []`, `check-ledger` → `ok: true`, exit 0.
(Control: a note typed under the table is refused, exit 13.)

**Class.** Every table is its own census — `clauses.md`, `ledger.md`, `test-set.md`, `bounds.md`. A
triaged finding's deletion *is* caught (S:2014-2015); an untriaged one's and a clause's are not. Cell 4 of
the entry *The record script reads the run back* accepts this for clauses knowingly; for the ledger no
entry does. **Smallest change:** the opening's facts name the clauses, and a round's record the ledger's
row count; `state` refuses a table that holds fewer.

---

### F7 — MEDIUM — a stop is lifted by three writes that are not the go

**Where.** S:1867, 1869 (the stop is a function of the latest round and the mode) · S:1577-1596
(`stop` is rewritable; only `rounds` is held) · any writer that takes `--round`.

**Repros (run; rigs `rig.e4.*`, `rig.e4b.*`, `rig.e5.*`).**
1. Stopped after round 1 (`every-round`, `position.test = refused: stopped`). `{"go":true}` for round 2
   is refused (7). `echo '{"base":"abcdef1"}' | ./sr round-set --run run1 --round 2` → exit 0, and
   `next = "test"`, `stop = null`, `position.test = {round 2, attempt 1}`.
2. Same with one report: `./sr report --run run1 --round 2 --stage test --reporter scope --attempt 1` —
   a run that stood at `stop → close` now answers `test`.
3. Stopped at a spent bound (`round-bound`, `fix` refused). `echo '{"stop":"every-round"}' | ./sr run-set
   --run run1` → exit 0; `next = "fix"`, `position.fix = {round 2, cycle 1}`: the bound is not applied in
   that mode, so it is gone without being raised. And `run-set {"rounds": 50}` is taken at any time,
   before any stop.

**Class, enumerated.** Writers with `--round` (S:2181-2192): `report`, `scope-set`, `round-set`,
`triage-set` — the first three create `r<N+1>/`, the fourth refuses without a scope; plus `run-set stop`
and `run-set rounds`: five sites. The entry says a go is held to the state *so that a go written early
cannot lift a stop nobody has seen*; these lift it unseen. **Smallest change:** a writer refuses a round
above the one the position names; the stop mode is written once.

---

### F8 — LOW (script level; the harness as built does not reach it) — `retest → retest`: a re-run counts as spent only if the clause's row was rewritten with the re-run's commit

**Where.** S:1798 (`spent` = the row's commit names a round whose record says `alone`) · S:1416-1421
(`void` with or without a commit; `--status` alone keeps the old one).

**Repro (run; rig `rig.loop.*`).** Clause void on round 1 → `retest`. Three re-runs, each
`clause-set --status void` (no `--commit`) and `round-set {candidate, alone: clause-b}`: `next = "retest"`
after each, `retry: due`, `fix_rounds 0`, `stop null`. A re-run counts against no bound, so nothing ends
it. The harness always passes `--commit` (H:1276), which is why this is LOW. **Lead, not run:** the same
loop when the orchestrator answers `retest` with a `test` that carries no `clause` — a whole round whose
delta is empty selects no hunt, the row is untouched, `retest` again. **Smallest change:** `spent` when
any round from the row's on says `alone: <clause>` without `granted`.

### F9 — LOW — a go given once silences every later stop of that round

Rig `rig.e8.*`: `every-round`; round 1 over → stop → `go`; the human then admits a finding
(`ledger-set admitted`), it is fixed, `cycles: 1`. *Got:* `next = "test"`, `stop = null` — the round's
fixes landed in a run that stops after every round, and nobody is asked (S:1867 reads one flag per
round). Instance; the only fact with this shape is `go`.

### F10 — LOW — `admitted` is not inherited across a fix that did not hold

Rig `rig.e7.*`: an outside finding, admitted, fixed, found again by the cycle's audit → `human (outside)`,
`next = "rule"`: the disposition cell holds one value, `fixed` replaced `admitted`, and re-opening routes
by the grade (S:1697-1720). The human admits the same finding once per failed cycle. The entry says
inheritance *stays as it is for the human's own rulings*. Safe side; instance.

### F11 — LOW — three inputs end in a traceback (exit 1), not in a one-line refusal

Rig `rig.h2.*` and `rig.h1.*`; nothing is written in any. (a) `ledger-add` with 200 000 `[`:
`RecursionError` (S:1019-1023 catches `ValueError` only). (b) a lone surrogate in any text field
(`"a \ud800 b"`): `UnicodeEncodeError` at the scan's copy (S:890-891). (c) `state` over a ledger whose
round cell a hand turned into `²`: `ValueError` at S:2055 (`isdigit()` at S:2050 admits it). Instances,
unbounded: `main` catches `Refusal` and nothing else (S:2211-2216).

### F12 — LOW — re-opened is cleared by the same commit written in another length

Rig `rig.again.*`, last step: a finding re-opened with `fixed: <F2, 40 hex>`; `ledger-set fixed` with
`<F2, 12 hex>` → `recorded (fixed)`. The fix is matched by the cell's text (S:1688), and 7 to 40 hex are
taken (S:1094). The harness sends full shas; a part's repoint is the acknowledged sibling.

### F13 — LOW — what the header claims and the code does not do

- S:457 *all of a batch is written, or none of it* and S:845 *so nothing was written* — F4.
- S:316-317 *close is reachable only from a tested candidate* — F2, F5.
- S:186-195, 110-112: `go`, `granted` and a bound's row are called the human's; **the header never says
  the script cannot know its caller**. The entries do (*the script cannot know that a human ruled*; *a
  disposition is not tied to who gave it*); `grep -n -i "cannot know\|caller\|who gave"` over the script
  finds no such sentence. What it can guarantee is shape, vocabulary, written-once, and — for `go` and
  `granted` only — that the state asks. `admitted`/`bound`/`later` are taken from any caller, at any
  time, on any row (an ungraded one included: disposition first, S:1699), any number of times, and
  `ledger-set open` takes a ruling back.
- S:208-213: the stop mode is not said to be rewritable; it is (F7).

### The suite (item 9)

- **S1 — it certifies F2.** T:4801-4806: a `fixed` row under a round with no cycle is expected to
  `close`.
- **S2 — the oracle for `current` is the implementation's predicate, respelled.** T:5855-5861 computes
  the expected `candidate.current` and the `not-tested` entry as *the latest record's text contains
  neither `cycles` nor `outcome`* — `fixed_in` again. A mutant of the predicate goes red; the predicate's
  adequacy cannot.
- **S3 — fixture topology.** `drive_round` (T:5715-5795) writes every clause row once, before any round;
  gives a clause at most one hunt and one check; plants every finding in round 1. So no cell holds a row
  rewritten by a later round (F1), and the one cell where a check and a hunt judge a clause (T:5367-5376)
  is a green row gone stale. The `not-scoped` cell (T:5398-5408) is the middle of F5; its end state is no
  cell.
- **Leaky.** Not found in the suite. Every spawn pipes all three streams (`Rig::command`, T:410-418, used
  by `.spawn()` at T:422, 1119, 2114, 2650, 2660, 4068) or uses `.output()` (T:370, 1922), and every child
  is waited for or killed; the script's own children get `DEVNULL`/capture pipes (S:876). So no descendant
  of a test can hold that test's stdout or stderr, and the mark has to come from outside the test's own
  process tree — which fits the builder's unconfirmed reading (a sibling started in the same instant) and
  nothing else I could see. Not confirmed.

---

## Crash consistency (item 7), as read and partly driven

| Write | Interrupted between | Next `state` | Silent? |
|---|---|---|---|
| `triage-set`: triage.md → ledger.md | its two files | routes by the stale ledger grade; `close` possible | **yes** (F4, driven) |
| test record: rows → triage → clause rows → `candidate` last (H:1273-1277) | anywhere | round not tested → `test`, next attempt | safe: the candidate is the last write |
| cycle record: dispositions → triage → `cycles` | after the dispositions | F2's state; `close` possible | **yes** in `state`; the harness's clean-tree assert is the only net (read, not driven) |
| drop: reopen patches → `outcome` | between | round open, blockers back → `fix` | safe |
| part: record → `outcome: part` → land | before the land | on the part's branch: over → `test` | as below |

**A record whose merge never happened:** on the round's branch `next` is `test` (or `stop`), exactly as
after the landing; `position.fix.land` is `true` in both, so nothing in the document tells them apart.
The harness converges anyway — a `test` stage asserts the loop branch (H:1929), where the records are
not, so `next` there is `fix`, and that `fix` lands (H:2309-2313). **A merge whose record was never
written** cannot come from the harness (it lands only on `land: true`, read after the cycle's record); by
hand it is F2's state on the loop branch.

## Constructed and found correct

- **Precedence**, driven (rig `rig.pairs.*`): not-ready first; an unfinished triage before the human's
  list before a fixer, all three open at once; `test` after a fix stage before all of them; the clause
  table asked only with nothing open on a current candidate; a spent retry before a clause no item judges
  before a due one (read; the suite has the cells).
- **A dropped round** (rig `rig.drop.*`): counts toward the bound, with or without a cycle; the `test` it
  forces selects only every-candidate items under an empty scope, so no hunt re-runs; a hunt's green from
  before the drop counts once that round has a scope; drop → test → fix → drop stops at the bound
  (`round-bound`, both stages' positions right).
- **Evidence age**: `behind` and `stale` after a landing, a drop, a re-run and a granted re-run are as the
  header says in every rig above.
- **`go` and `granted`**: early, for the wrong round and for the wrong clause each refused (7); a grant
  earns exactly one more re-run, then `rule` again (rig `rig.spent.*`).
- **A finding found again** (rig `rig.again.*`): under `open`, `fixed`, `admitted`, `bound`, `later`,
  `no-action`, outside, and re-opened then fixed again then found again — each routed as the rulings say
  (but F10, F12).
- **Hostile cells**: a door with `|`, with a line break, with padding — matched identically across
  ledger, scope and test set; a hand-typed note, a wrong status, a second row are refused (13).
- **A round's test stage is "tested" by its last write**, so a half-written test record never reads as a
  tested round.

## Not examined

- The suite was not run and no mutant was re-applied: another agent holds the tree and the gate.
- The harness beyond the lines cited; the agent definitions; `dev/gate`.
- The writers' older half (`public`, `hygiene_scan`, the lock under contention, the real gitleaks),
  beyond what F4 needed.
- `check-reports`; the identifier axis (the suite's `HOSTILE_IDS` was read, not re-driven).
- Whether a red deterministic check gets a ledger row (if not, a red gate has no way to a fixer — a lead
  for the harness's reviewer).
- A `retest` round that halts before its record answers `test`, with nothing saying it was a re-run
  (lead, not driven as a stage).
