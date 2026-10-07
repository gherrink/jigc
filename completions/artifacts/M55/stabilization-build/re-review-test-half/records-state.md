# Re-review — the record and the state machine of a stabilization run (`dev/stabilize-record`, `dev/stabilize-step`, `603018ac^..cbb3d736`)

Read-only re-review of committed state at `cbb3d736`: `dev/stabilize-record` (3637 lines), `dev/stabilize-step`
(828 lines), their suites under `tooling-tests/` (`dev_stabilize_record.rs`, `dev_stabilize_step.rs`,
`stabilize_simulation.rs` where it drives the two, `placed_executable.rs` skimmed), the sections of
`implementation/stabilization-workflow.md` that say what the two scripts do, and
`.claude/workflows/stabilize.js` only where what it hands the scripts decides a finding. Judged against
`completions/artifacts/M55/stabilization-build/state-machine.md` (the first review of this part, red),
`repair-plan.md` beside it (changes C0, C1, C3, C4, C5), and the `DECISIONS.md` entries of 2026-10-05 and
2026-10-06 that name the workflow.

**Verdict.** The repair of the `test` half is real: of the first review's thirteen findings ten are closed by
a change of structure, and for nine of the ten a mutant that takes the fix out turns a named test red
(eighteen mutants, all killed — the table is below). **Fit for a canary run of `test`.** **Not yet fit for the real run**: one new
finding puts unscanned text on a public remote (R1), and seven leave the run at a wrong `next`, or at a
refusal whose way out is not the one it names (R2–R8). None of the eight needs the `fix` stage.

Severity counts: **1 HIGH · 7 MEDIUM · 8 LOW.**

## How the repros were driven

Everything ran in throwaway roots minted with `mktemp -d` under one scratch root; nothing ran in the
repository's working tree, nothing was pushed to its remote, and the full gate did not run.

- **Light rigs** (`newrig <label>`): a small git repository with a bare remote of its own, holding the
  committed bytes of `dev/stabilize-record`, `dev/stabilize-step`, `dev/hygiene-scan`, `dev/merge-logs` and
  `.gitleaks.toml` (each from `git show cbb3d736:<path>`), one opened run `run1` on the branch `fix/run1`,
  a synthetic denylist and a stub `gitleaks` that keeps its exit contract. Used for the state machine.
- **Clones** (`newclone <label>`): `git clone` of the repository at `cbb3d736` with a bare remote of its
  own (its `origin` removed), the loop branch `fix/run1`, `run1` opened, committed and pushed. Used for
  every git act and for every kill.
- **Kills.** `killat.py` runs the record script unmodified (`runpy`) and sends itself SIGKILL just before
  the Nth call of `os.replace`, `os.link`, or the `os.unlink` of the journal; `killstep.py` does the same
  to the step tool just before a named child command. So a kill lands between two system calls of the
  committed code, never inside a copy of it.
- **Mutants.** A second clone, with a private `CARGO_TARGET_DIR`, built the `g_tooling` test target once
  (`cargo test -p jigc --test g_tooling --no-run`; the four suites were green on it unmodified: 118
  passed). Each mutant is one text replacement in that clone's `dev/` script, the named tests are run from
  the built binary, and the file is restored with `git checkout`.
- The helpers are the appendix, verbatim; every repro starts with `. ./lib.sh` and one `newrig` or
  `newclone`. `sr` is the record script, `ss` the step tool, `st f…` prints fields of `state`; `C1`…`C3`
  are forty `1`…`3`, `F1`/`F2` forty `a`/`b`; `stage1` stands a clone at the point where a `test` stage's
  record batch (seven calls, two checks) is composed and not yet applied.

Line numbers: **S** = `dev/stabilize-record`, **T** = `dev/stabilize-step`, **H** =
`.claude/workflows/stabilize.js`, **U** = `tooling-tests/dev_stabilize_record.rs`, **V** =
`tooling-tests/stabilize_simulation.rs`, **D** = `implementation/stabilization-workflow.md`, all at `cbb3d736`.

---

## Findings

### R1 — HIGH — a file at a report's path that the record script never wrote is the run's "pending write": it is committed and pushed unscanned

**Where.** S:3477-3487 (`pending`: `once` is every file **on disk** whose name is a report's, and each
round's `scope.md` — read from the directory, not from anything the script wrote) · T:355-374 (`admitted`:
an untracked path in `once` is no dirty tree) · T:631-651 (`record`: every pending path is added by name
and committed) · S:3265-3294 (`check-reports` knows a report by its file name). Against S:13-17 and
S:822-846 (*every write goes through one function*) and D:108 (*every report reaches the repository
through the record script and no other way*).

**Repro (run; clone).**

```
. ./lib.sh; newclone r1; stage1
TEXT=$(printf '# review\n\nran in %s/probe and met zzqprivatetermzz\n\n<!-- end of report -->\n' "$RIG/home")
echo "$TEXT" | sr report --run run1 --round 1 --stage test --reporter review-rename --attempt 1; echo "exit=$?"   # refused hygiene, 11
echo "$TEXT" > "$R/$RD/r1/reports/test/review-setup.a1.md"     # a shell redirect, over the report the script wrote
gs | line status refused                                        # the step that opens a stage
applyb >/dev/null; recordb | line status; ss push --branch fix/run1 | line status
henv git -C "$RIG/remote.git" show fix/run1:$RD/r1/reports/test/review-setup.a1.md | sed -n 3p
```

*Expected:* the changed file is a dirty tree (T:136-143: *a file of a batch changed after it was written …
is a tree no step starts from*), or the commit step scans what it adds.
*Got:* the script refuses the text (`hygiene`, exit 11); then `git-state` → `ready`, `record` → `recorded`,
`push` → `ready`, and the bare remote's branch holds line 3 — `ran in <scratch>/…/home/probe and met
zzqprivatetermzz` — a host path and the denylisted term, the text the script had refused one line earlier.

**Class, and its bound.** Files admitted by their **name**, not by what the script wrote. Derived by reading
`pending()`: its `once` list has two kinds of member — a report-named file under `r<N>/reports/test` or
`r<N>/reports/fix/c<K>`, and `r<N>/scope.md` — and both are admitted the same way. Driven: a report the
script wrote, overwritten afterwards (above), and a file at a launched reporter's path that the script
never wrote (same outcome). Not driven: `scope.md`. **Not** in the class: the files of an applied batch,
which are held to the sha256 the batch wrote them with (S:3489-3499, `altered`) — the mechanism exists and
is applied to the tables only.

**Reach.** A reporter has a shell. The moment it is most likely to write the file itself is the moment the
script refuses its text (`hygiene`, `host-path`, `fence`, `truncated`). In a canary's throwaway clone this
is harmless; on the real run the push is public and is not undone. CI's scan is hard, and runs after the
push.

**Smallest change.** The script keeps the sha256 of every report and scope it writes (a counted fact, or a
list beside the journal) and `pending` names only those, the rest as `altered`; `git-state` and `record`
then refuse a changed report as they refuse a changed table. Or: the commit step hands every pending path
to the scan before `git add`.

---

### R2 — MEDIUM — an applied batch that outlives its commit: every later record step is refused, the step that opens a stage says nothing is pending, and one of the two ways out the refusal names reverts committed tables

**Where.** T:650-657 (`add` → `commit` → `settle`, three steps) · T:444-448 (`live`: a batch none of whose
files the tree still holds *is nothing pending* — and is not settled) · S:3422-3423 (`apply`: refused
`pending`, naming `dev/stabilize-step record` and `discard`) · S:3503-3526 (`discard` writes every file's
text **before the batch**, committed or not) · T:634-635 (`no-batch`, which does name `settle`).

**Way in 1 — a kill between the commit and its settling (run; clone).**

```
. ./lib.sh; newclone r2a; stage1; applyb >/dev/null
ssk "stabilize-record settle" record --branch fix/run1 --run-dir $RD --gate "$GATE" --calls 7 --checks 2   # killed after the commit
gs | line status pending                 # what the next invocation reads
B='[{"argv":["ledger-set","--run","run1"],"stdin":"[{\"key\":\"f-1\",\"disposition\":\"later\"}]"},{"argv":["check-ledger","--run","run1","--","f-1"]}]'
echo "$B" | sr apply --run run1 --round 1 --subject "docs(record): the next record"; echo "exit=$?"
sr discard --run run1 >/dev/null; porcelain; st next rounds.0.facts.candidate
```

*Got:* `git-state` → `ready`, `pending = null`, so the next invocation works a whole new attempt; its
record's `apply` is refused `pending` (exit 18); and after the `discard` that refusal names: `D ledger.md`,
`D r1/gate.md`, `D r1/results.md`, `D r1/round.md`, `D r1/triage.md`, `M clauses.md`, `M run.md`,
`next = "test"`, the round's candidate `null` — the committed round reads as never tested, and the tree is
dirty.

**Way in 2 — no kill at all: a batch that changes nothing (run; clone).** The human's ruling on a finding,
recorded, and then the same ruling sent again. The harness checks only that the key is a row (H:711-715),
and the advice of every halt is *the same arguments*.

```
. ./lib.sh; newclone r2b; stage1; applyb >/dev/null; recordb >/dev/null
B='[{"argv":["ledger-set","--run","run1"],"stdin":"[{\"key\":\"f-1\",\"disposition\":\"later\",\"detail\":\"next release\"}]"},{"argv":["check-ledger","--run","run1","--","f-1"]}]'
rule() { echo "$B" | sr apply --run run1 --round 1 --subject "docs(record): run1 r1 — the human's rulings" >/dev/null; echo "apply exit=$?"
         ss record --branch fix/run1 --run-dir $RD --gate "$GATE" --calls 2 --checks 1 | line status refused halt.recommendation; }
rule          # the ruling, recorded
rule          # the same ruling, sent again
gs | line status pending; sr pending --run run1 | line batch.calls
echo "$B" | sr apply --run run1 --round 1 --subject "docs(record): any later record"; echo "exit=$?"
```

*Got:* the second `apply` is taken (exit 0); `record` halts `no-batch` — *the batch's files are in a commit
already* — and the journal keeps the batch; `git-state` → `ready`, `pending = null`; every later `apply` of
the run, whatever it records, is refused `pending`.

*Expected:* a batch whose every file is in a commit is forgotten by whoever sees that — `git-state` already
computes it — or `apply` refuses a batch that changes nothing.
*What follows instead:* each later stage does its whole work — builds, instruments, triage — and halts at
its record; after two such attempts the stage is the human's (`attempts-spent`), and the grant leads to the
same halt. The one command that ends it, `dev/stabilize-record settle`, is named by `no-batch` only — on
way 2, at the halt that created the state, and never on way 1. The harness's sentence after a record halt
(H:1898) recommends `discard`.

**Class.** States in which the journal's batch and git disagree. Enumerated from the three steps of
T:650-657 and the two predicates that read them (T:446 `live`, T:634 `paths`): batch and nothing committed
(pending — handled); batch and a staged index (R3); batch and its commit (this, way 1); a batch that never
differed from `HEAD` (this, way 2). **Smallest change:** `git-state` settles a batch that is not `live`;
`apply` refuses a batch whose staged texts are what the files already hold; `discard` is refused while no
file of the batch differs from `HEAD` — that needs git, so it belongs in the step tool.

---

### R3 — MEDIUM — a commit step whose `git commit` fails, or that is killed after `git add`, leaves a staged tree that every act refuses as dirty — and no command of either script leaves it

**Where.** T:650-651 (`git add -- <paths>`, then `git commit -q -m`, under the ambient git configuration) ·
T:368 (`admitted` takes `?? ` and ` M ` lines only: `A  ` and `M  ` are *other*) · T:372-373 (`dirty`,
*the human reconciles it*).

**Repro (run; clone).**

```
. ./lib.sh; newclone r3; stage1; applyb >/dev/null
printf '#!/bin/sh\necho "hook: refused" >&2\nexit 1\n' > "$R/.git/hooks/pre-commit"; chmod 755 "$R/.git/hooks/pre-commit"   # any reason a commit fails
recordb | line status refused halt.evidence
mv "$R/.git/hooks/pre-commit" "$R/.git/hooks/pre-commit.off"     # the cause is dealt with
recordb | line status refused                                    # the commit step, asked for again
gs | line status refused halt.recommendation                     # and a stage invoked again
porcelain | head -3
```

*Got:* `halted, git` (`git commit … exited 1`); then, the cause gone, the commit step again → `dirty`; a
stage invoked again → `dirty`, *the human reconciles it*; the index holds `M  clauses.md`, `A  ledger.md`, ….
The same state with `ssk "commit -q -m" record …` in place of the hook — a kill between the two commands
(run). `dev/stabilize-record discard` then leaves `MM` and `AD` lines (run). *Expected:* the commit step is
repeatable — staged lines of the batch's own files are admitted, or the act commits by pathspec with no
separate `add`. Meanwhile `state` reads the applied tables and answers as if the record stood.

**Reach.** One failed commit. The step commits under whatever the machine's git configuration says; on the
machine this was reviewed on `commit.gpgsign` is on, so a signing agent that does not answer is such a
failure. The step suite runs with hooks off and a fixed identity (`dev_stabilize_step.rs:361`), and no arm
of any suite fails a commit. **Class:** the lines `admitted` takes, held against the states the act itself
can leave — an instance, bounded to the one gap between the act's two commands.

---

### R4 — MEDIUM — the temporaries of a write killed before its journal make the tree dirty for the step that opens every stage, whose refusal does not name the command that removes them

**Where.** S:1564-1581 (temporaries are written before the journal names them) · S:1538-1541 (only a
writer, or `recover`, removes them) · S:3462-3500 (`pending` settles nothing) · T:426-427 (`git-state`
runs before any writer of a stage) · T:372-373.

**Repro (run; clone).**

```
. ./lib.sh; newclone r4; stage1
srk replace:1 apply --run run1 --round 1 --subject "docs(record): r1" < "$RIG/batch.json"     # killed before the journal is written
gs | line status refused halt.recommendation
sr recover --run run1; gs | line status
printf '# r\n\n<!-- end of report -->\n' | srk link:1 report --run run1 --round 1 --stage test --reporter late --attempt 1   # a reporter, killed
sr check-reports --run run1 --round 1 --stage test --attempt 1 -- attempt scope review-setup; echo "exit=$?"
gs | line status refused
```

*Got:* `git-state` → `dirty`, naming eight `.stabilize-record.*.tmp`, *the human reconciles it*; after
`recover` → `ready`. And for the killed reporter: `check-reports` → `ok: false`, `extra:
[".stabilize-record.….tmp"]`, exit 20, and `git-state` → `dirty`.
*Expected:* what the header says of this state (S:731-733: *leaves no file of its batch, and its
temporaries are removed by the next writer*) holds for a stage — or the refusal names `recover`, as the
refusal of an interrupted write does (T:351). True of the script; but no stage reaches a writer, because
its first act is the read that refuses. H:1898 tells the orchestrator to invoke the stage again, which
halts the same way. **Class:** SIGKILL only (SIGTERM unwinds, S:3609-3610, and the `finally` of `flush`
removes them). Every writer has the window and `apply` the widest; one mechanism, bounded by reading
`flush`.

---

### R5 — MEDIUM — an item whose clause the census does not name judges nothing: its void run is ignored, and the run closes over an instrument that never ran to its end

**Where.** S:2342 (`item-set`: the clause is held to the slug grammar, and to nothing else) · S:2697-2701
(`judged` walks the census and takes the items whose clause **is** one) · S:2650-2663 (`owed_by`: what the
opening owes asks the test set nothing). The clause is typed twice at an opening — `run-set` `clauses`, and
every row of `item-set` — and nothing holds the two against each other.

**Repro (run; light rig) — one letter is missing in the second item's clause.**

```
. ./lib.sh; newrig r5; opening '"stop":"at-the-bound","rounds":3' no-lost-files >/dev/null
items '[{"item":"gate","kind":"check","clause":"no-lost-files","runs":"every-candidate","brief":"b"},
        {"item":"review-setup","kind":"review-row","clause":"no-lost-file","runs":"in-scope","doors":["jigc setup"],"brief":"b"}]'; echo "exit=$?"
st not_ready next
scope 1 "jigc setup" >/dev/null
results 1 $C1 '[{"item":"gate","outcome":"green"},{"item":"review-setup","outcome":"void","reason":"its reviewer died"}]' >/dev/null
facts 1 "{\"candidate\":\"$C1\"}" >/dev/null; st next forbids_close
tail -1 "$R/$RD/clauses.md"
```

*Expected:* a run that is not ready (an item judges no clause of the condition), or a refusal of the row.
*Got:* `item-set` exit 0; `not_ready = []`, `next = "test"`; and after the round: `next = "close"`,
`forbids_close = []`, the clause table's one row naming `gate` alone. A stage runs the item — selection
does not read its clause (S:2968-2977) — records it void, and nothing asks about it again. It is the first
review's `F1` outcome (*an instrument that never ran to its end*) by another door.
**Class:** the four cells of an item's row that the derivation reads — clause, runs, doors, registries —
against what holds each; enumerated from `judged` and `selected_by`. Clause: unheld (this). Runs, doors,
registries: replaceable after results exist (R6). **Smallest change:** `not_ready` names an item whose
clause the census lacks; `item-set` refuses one once the census is written.

---

### R6 — MEDIUM — `item-set` replaces an item whole after it has results: a void item stops judging, and `retest` becomes `close`

**Where.** S:2355-2357 (a held item's row is replaced) · S:771-777 (*anywhere … what such a write CAN do
is add work or record a ruling*) · S:154-156 (*nothing removes an item of the test set (item-set)*) · D:293
(`item-set` is the documented way an instrument built later takes its row, in mid-run).

**Repro (run; light rig) — a brief is corrected, and the door's text gains one space.**

```
. ./lib.sh; newrig r6; opening '"stop":"at-the-bound","rounds":3' clause-a >/dev/null
items '[{"item":"gate","kind":"check","clause":"clause-a","runs":"every-candidate","brief":"b"},
        {"item":"review-setup","kind":"review-row","clause":"clause-a","runs":"in-scope","doors":["jigc setup"],"brief":"b"}]' >/dev/null
scope 1 "jigc setup" >/dev/null
results 1 $C1 '[{"item":"gate","outcome":"green"},{"item":"review-setup","outcome":"void","reason":"its reviewer died"}]' >/dev/null
facts 1 "{\"candidate\":\"$C1\"}" >/dev/null; st next forbids_close
items '[{"item":"review-setup","kind":"review-row","clause":"clause-a","runs":"in-scope","doors":["jigc  setup"],"brief":"a corrected brief"}]'; echo "exit=$?"
st next forbids_close clauses.0.status
```

*Got:* before: `next = "retest"`, `forbids_close = [clause-a void]`. After the one `item-set` (exit 0):
`next = "close"`, `forbids_close = []`, `green` — no round's scope selects the item any more, and *an item
no scope selected owes nothing*. The same with its clause replaced (run). Correcting a brief means
restating every cell of the row. **Reach:** a direct call; the harness composes no `item-set`.
**Smallest change:** an item that has a result keeps its clause, runs, doors and registries; `item-set`
then takes its brief, or refuses.

---

### R7 — MEDIUM — a record that is committed and not pushed is said by nothing: the next step is read from it, and the next round begins on a tip the remote does not have

**Where.** T:420-448 (`git-state` holds the pushed loop branch to *not ahead of the local one* and reports
`head` and `loop_head`, never the remote's) · H:2368, H:2191, H:2137 (a failed push is a halt whose only
record is the return) · D:100 (*the loop branch is checked out, clean, and pushed*) — a precondition no step
asserts.

**Repro (run; clone).**

```
. ./lib.sh; newclone r7; stage1; applyb >/dev/null; recordb | line status
mv "$RIG/remote.git" "$RIG/remote.away"; ss push --branch fix/run1 | line status refused     # the remote does not answer
mv "$RIG/remote.away" "$RIG/remote.git"                                                       # the cause is dealt with
henv git -C "$R" rev-list --count origin/fix/run1..fix/run1
gs | line status pending; st next position.test begins
```

*Got:* the push halts; with the remote back, the local branch is `1` commit ahead of `origin`'s, and
`git-state` → `ready`, `pending = null`; `next = "close"`, `position.test = {round 2, attempt 1}`,
`begins = 2`. Invoked again as the halt advises, `test` begins round 2 — on a candidate CI has no run for —
or, with a finding open, is refused `round-open`, and nothing pushes round 1's record until some later
record step does. The repair plan's reconcile check (*unpushed means push owed*) is under C2, which is not
built; this window is the `test` stage's own. **Smallest change:** `git-state` reports the remote's head,
and a stage whose loop branch is ahead pushes first, or says so.

---

### R8 — MEDIUM — a record in a round that has no gate on record cannot be committed under a red candidate

**Where.** S:3389-3392 (`gate-check`: no `gate.md` for the round → `known = []` → nothing may be red) ·
H:2160 (a ruling about the run is recorded for `state.round`, the latest round — begun and untested when
the ruling is `again: 'test'`) · H:1350 (`--round` is left out for round 0). The suite pins the mechanism
as intended for a batch with no round (V:1711-1714).

**Repro (run; clone) — round 1's `test` halted twice, the human grants one more attempt, the candidate's gate is red.**

```
. ./lib.sh; newclone r8
opening '"stop":"at-the-bound","rounds":3' clause-a >/dev/null
items '[{"item":"gate","kind":"check","clause":"clause-a","runs":"every-candidate","brief":"b"}]' >/dev/null
henv git -C "$R" add $RD; henv git -C "$R" commit -q -m "docs(record): the opening's facts"; henv git -C "$R" push -q origin fix/run1
CAND=$(henv git -C "$R" rev-parse HEAD)
for a in 1 2; do ss begin --run run1 --round 1 --stage test --attempt $a --reporter attempt --commit $CAND --scratch "$RIG/tmp" >/dev/null; done
st next human_stages.0.why position.test
echo '[{"argv":["round-set","--run","run1","--round","1"],"stdin":"{\"again\":\"test\"}"}]' | sr apply --run run1 --round 1 --subject "docs(record): the human's rulings" >/dev/null
RED=$(gatefile cand fail "jigc::g_cli a::test_red_on_the_candidate")     # the record's gate shows what the candidate's shows
ss record --branch fix/run1 --run-dir $RD --gate "$RED" --calls 1 --checks 0 | line refused gate.known gate.new
sr gate-set --run run1 --round 1 --commit $CAND --summary "$RED" >/dev/null; echo "a direct gate-set: exit=$?"
ss record --branch fix/run1 --run-dir $RD --gate "$RED" --calls 1 --checks 0 | line refused
```

*Got:* `gate-red`, `known = []`, `new` = the candidate's own red: the grant cannot be recorded, so the
stage stays refused. A direct `gate-set` for the round is taken (exit 0) and is no way out — `r1/gate.md`
is then an untracked file that is neither written-once nor a file of the batch, and the commit step
refuses `dirty`. The way out is a batch composed by hand that holds `gate-set` beside the grant.
**Reach:** a red candidate — the ruling this check implements exists because one is expected — and a round
whose first `test` halted twice, which is first-run teething; or any ruling recorded before round 1.
**Smallest change:** a batch whose round has no gate on record is held to the gate of the candidate the
run tested last, or carries the candidate's gate itself.

---

### R9 — LOW — 4(a), verified: while the opening is not done the record script takes not round 1's first files but the whole run

`begins` is computed *as if the opening were done* (S:3236); a fix cycle is taken while the position says
`not-ready` (S:2424); a bound is written at any time while `not_ready` is not empty (S:2481).

```
. ./lib.sh; newrig r9; st next begins
ss begin --run run1 --round 1 --stage test --attempt 1 --reporter attempt --commit $C1 --scratch "$RIG/tmp" | line status refused
ITEMS='[{"item":"gate","kind":"check","clause":"clause-a","runs":"every-candidate","brief":"b"},{"item":"review-setup","kind":"review-row","clause":"clause-a","runs":"in-scope","doors":["jigc setup"],"brief":"b"}]'
GREEN='[{"item":"gate","outcome":"green"},{"item":"review-setup","outcome":"green"}]'
{ report 1 test stray 1 && scope 1 "jigc setup" && items "$ITEMS" && results 1 $C1 "$GREEN" && facts 1 "{\"candidate\":\"$C1\"}" && facts 1 '{"cycles":1}' \
  && scope 2 "jigc setup" && results 2 $C2 "$GREEN" && facts 2 "{\"candidate\":\"$C2\"}" && echo '{"rounds":9}' | sr run-set --run run1; } >/dev/null; echo "every call: exit=$?"
st next round fix_rounds
opening '"stop":"every-round"' clause-a >/dev/null; st next stop
newrig r9b >/dev/null; report 1 test stray 1 >/dev/null; report 1 test stray 2 >/dev/null     # the smaller case: two stray reports
opening '"stop":"at-the-bound","rounds":3' clause-a >/dev/null; items "$ITEMS" >/dev/null; st next human_stages.0.why position.test
```

*Got:* `begin` → `halted, position` (the tool refuses, as built). Every direct call exit 0; then
`next = "not-ready"`, `round = 2`, `fix_rounds = 1`; and with the opening finished by one call:
`stop = {every-round, round 2, then close}`. The smaller case: `next = "rule"`, the `test` stage
`attempts-spent` before it ever ran.

**Grade.** No stage reaches it: the position refuses, and `begin` refuses again from the state. From a
direct call it is wider than D:460 says (*a report or a scope for round 1*): two tested rounds, a fix cycle,
a bound, and the stop after round 1 passed without a go — all before the opening is done. The realistic
cost is the smaller case, on the safe side; and a stray `scope-set`, which fixes round 1's scope — written
once, and not taken back by `discard`.

### R10 — LOW for `test` as it stands, a lead for the `fix` half — 4(b), verified: after a dropped round nothing but an agent decides that no hunt runs, and the state does not say that the round was dropped where a stage reads its work from

```
. ./lib.sh; newrig r10; opening '"stop":"at-the-bound","rounds":3' clause-a >/dev/null
items '[{"item":"gate","kind":"check","clause":"clause-a","runs":"every-candidate","brief":"b"},{"item":"review-setup","kind":"review-row","clause":"clause-a","runs":"in-scope","doors":["jigc setup"],"brief":"b"}]' >/dev/null
scope 1 "jigc setup" >/dev/null; finding f-1 "jigc setup" >/dev/null
results 1 $C1 '[{"item":"gate","outcome":"green"},{"item":"review-setup","outcome":"green"}]' >/dev/null
triage 1 '[{"key":"f-1","grade":"breaks","verdict":"confirmed","regression":false}]' >/dev/null; facts 1 "{\"candidate\":\"$C1\"}" >/dev/null
facts 1 '{"outcome":"dropped"}' >/dev/null
st next position.test begins fix_rounds rounds.0.facts.outcome
scope 2 "jigc setup" >/dev/null; echo "a scope that reaches the hunt: exit=$?"; st items.1.selected
```

*Got:* `next = "test"`, `position.test = {round 2, attempt 1}` — the same object as after a landing —
`fix_rounds = 1`; the drop is in `rounds[0].facts.outcome` and in no other key of the document. A scope for
round 2 that reaches the hunt is taken, and the hunt is `selected`. With an **empty** scope the state is
right (run: only the every-candidate item is selected, the hunt's round-1 green stands, the finding is a
fixer's again, the clause is green over rounds 1 and 2). What writes that scope is the scope step, and the
harness hands it round 1's candidate as its base and *the door lists of round 1's fixers* as inputs
(H:1194-1196, H:2256), with no word that those fixes never landed. Unreachable today: only the `fix` stage
writes `outcome`.

### R11 — LOW — at a clause that is the human's, the position hands a `test` stage round R+1

S:3032-3033; pinned as intended by the suite's table (U:11069: `("rule", None, Some(2))`).

```
. ./lib.sh; newrig r11; opening '"stop":"at-the-bound","rounds":3' clause-a >/dev/null
items '[{"item":"gate","kind":"check","clause":"clause-a","runs":"every-candidate","brief":"b"},{"item":"review-setup","kind":"review-row","clause":"clause-a","runs":"in-scope","doors":["jigc setup"],"brief":"b"}]' >/dev/null
scope 1 "jigc setup" >/dev/null
results 1 $C1 '[{"item":"gate","outcome":"green"},{"item":"review-setup","outcome":"void","reason":"died"}]' >/dev/null
facts 1 "{\"candidate\":\"$C1\"}" >/dev/null
results 1 $C1 '[{"item":"review-setup","outcome":"void","reason":"died again"}]' >/dev/null      # the one re-run that is automatic
st next human_clauses position.test begins
scope 2 "jigc setup" >/dev/null; results 2 $C1 '[{"item":"gate","outcome":"green"},{"item":"review-setup","outcome":"void","reason":"died in a new round"}]' >/dev/null
facts 2 "{\"candidate\":\"$C1\"}" >/dev/null; st next human_clauses fix_rounds
```

*Got:* `next = "rule"`, `human_clauses = [clause-a]`, and `position.test = {round 2, attempt 1}`,
`begins = 2`; after round 2: `next = "retest"`, `human_clauses = []`, `fix_rounds = 0`. The harness starts
`test` on the position alone (H:2214-2215), so a `test` sent in answer to `rule` begins a round, the item's
count of runs starts again, and a round with no fix stage counts against no bound. It needs an orchestrator
that answers `rule` with `test`; the header's sentence that every loop has a counter (S:779-800) is true of
what `next` asks for, not of what the position takes.

### R12 — LOW — two inputs the script takes, or refuses in the wrong class

```
. ./lib.sh; newrig r12; opening '"stop":"at-the-bound","rounds":3' clause-a >/dev/null
items '[{"item":"gate","kind":"check","clause":"clause-a","runs":"every-candidate","brief":"b"}]' >/dev/null; scope 1 "jigc setup" >/dev/null
results 1 $C1 '[{"item":"gate","outcome":"void","reason":"-"}]'; echo "exit=$?"
echo '{"key":"f-1","doctype":"jigc-feedback","round":12345678,"source":"s","door":"d","clause":"c","repro":"r"}' | sr ledger-add --run run1 >/dev/null; echo "ledger-add exit=$?"
sr state --run run1 >/dev/null; echo "state exit=$?"
```

*Got:* the void whose reason is `-` is refused as `corrupt`, naming `results.md` (S:2772 reads the cell the
call itself staged) — nothing is written, and inside a record's batch the stage halts on a table that is
whole. The ledger row with a round of eight digits is written (S:1778 asks only `>= 0`), and `state` then
refuses the ledger as corrupt (S:3159: six digits) until a hand edits the file. Instances, unbounded: found
by reading where a writer's check and the reader's check of one cell differ; not swept.

### R13 — LOW — the subject of a record commit is neither scanned nor freed of host paths

S:3411-3412 holds `--subject` to one line.

```
. ./lib.sh; newclone r13; stage1
sr apply --run run1 --round 1 --subject "docs(record): zzqprivatetermzz in $RIG/home" < "$RIG/batch.json" >/dev/null; echo "apply exit=$?"
recordb | line status; henv git -C "$R" log -1 --format=%s
```

*Got:* the commit's subject is that text. The harness composes subjects from the run's slug and fixed
words (H:1421), so this is a direct caller's.

### R14 — LOW — the script's writer × state table is a census of names; what a writer does is still written by hand

`WRITERS` and `RUN_TAKES` are read by no code but the census at S:3591, and `ROUND_TAKES` decides three of
its five words (S:2417-2426): `run-set` works from `RUN_ONCE` and `RUN_BOUNDS`, `gate-set` calls `untested`
by hand. So a writer **cannot** be added without a row — every call faults, exit 23, held by U:11535-11565
— and a writer **can** be added with a row its code does not honour. What holds behaviour is the suite's
own table (`HERE`/`NEXT` × seventeen `PLACES`, each cell written by hand, its rows held to the script's
lists): mutants `m05`, `m10`, `m11` and `m13` each turn
`every_writer_and_every_fact_is_taken_only_where_the_state_asks_for_it` red. Closed as a class by the two
together, not by the script's table.

### R15 — LOW — what the header and the doc claim and the code does not do

- S:154-156 *nothing removes an item of the test set* — R6. S:771-777 *anywhere: … add work or record a
  ruling* — R6 turns `retest` into `close`.
- S:13-17, D:108, D:287 *nobody writes any of it by hand … the script … stops hard on the scan* — R1.
- D:215 *what a stage wrote through the record script and no record step committed is pending, never a
  dirty tree* — R4 (temporaries) and R3 (the commit step's own index).
- D:213, H:1898 *invoke the stage again … commits a batch that is pending* — not what happens in R2, R3, R4.
- D:460 — R9: wider than stated. T:181 *tooling-tests/dev_stabilize_step.rs drives it* — the `record` and
  `begin` acts are driven by `stabilize_simulation.rs` and by one composed command of the step suite; no
  test of the step suite is theirs.
- S:459-461 *close is reachable only from a tested candidate* — still false for a fix with no counted
  cycle (the first review's `F2`), which S:801-804 now says in its own words.

### R16 — LOW — the suites: what no arm drives

- **No kill of the commit step, no failed commit, no batch that changes nothing, no changed report.** The
  one "killed write" a step is shown is a journal typed by the test (V:1783-1808). R1 to R4 are each a
  state none of the 118 tests stands up.
- **The first review's `S1` and `S2` stand**: the cell *every blocker fixed, every item ruled, one found
  again* still expects `close` for a `fixed` row under a round with no cycle (U:5838-5842), and the oracle
  for `candidate.current` is still the implementation's predicate respelled (U:7239-7242). Mutant `m18`
  (the candidate is not current while a row says `fixed`) turns all three truth-table tests red: the suite
  requires the `F2` close.
- `the_humans_go_is_a_recorded_fact_taken_only_for_the_stop_it_answers` stays green when a go is made to
  stand for ever (`m08`); `a_stop_is_lifted_by_its_go_and_by_no_other_write` is what holds it.

---
## The first review's findings, and where each stands

*Held by* names the test that goes red when the fix is taken out, and the mutant that showed it (the
mutants are the table further down). "Driven" is the first review's own repro, rebuilt from its block
against `cbb3d736`.

| | The first review | Now | What closes it | Held by |
|---|---|---|---|---|
| `F1` HIGH | a later round turns a hunt green that never ran to its end | **closed**, by structure (C1) | a result per item and round; the clause derived (S:2674-2742); an item is owed by the latest tested round whose scope selects it. Driven: `retest`, never `close`; the re-run is attempt 2 inside round 1 | `an_instrument_that_never_ran_to_its_end_is_not_turned_green_by_a_later_round_that_ran_the_others` and five more (`m01`); `the_clause_table_is_rendered…` (`m02`) |
| `F2` HIGH | a fix on the ledger with no counted cycle leaves the pre-fix candidate current | **not closed** — declared as the `fix` half's (S:801-804; C2 is not built). Driven: `next = "close"`, `current: true`, `fix_rounds: 0` | — | the suite still **requires** it: `m18` turns the three truth-table tests red (`S1`, `S2` below) |
| `F3` HIGH | `triage` has no bound | **closed** (C3) | `triage-set` counts, in the round's record, every finding it leaves unsettled; two in a row, then `human_list` with `…-after-retry`; one more by `reverify`. Driven | `a_finding_the_triage_cannot_settle_is_tried_once_more_and_then_it_is_the_humans`, the simulation's `an_unverified_finding_is_finished…`, and the truth tables (`m03`) |
| `F4` MEDIUM | `triage-set` refuses after writing one of two files | **closed** (C4) | one scanned, journaled batch. Driven: a scanner that cannot run leaves every file as it was; a kill before each of the four `os.replace` calls and before the journal's removal reads as *nothing written* or as `interrupted`, and `recover` finishes it | `a_write_killed_between_its_files_is_never_read_as_a_finished_one` (`m04`) |
| `F5` MEDIUM | `alone` is taken in any state and launders `not-scoped` | **closed**, by removal | `alone` is no fact (refused, exit 7); a re-run is a result of the round that selected the item, taken only while `next` is `retest` and the item is due (S:2223-2224). Driven | `a_rerun_is_an_attempt_inside_its_round…`, `every_writer_and_every_fact…` (`m13`) |
| `F6` MEDIUM | a deleted clause row and a deleted ledger row answer `close` | **closed** for both; two tables changed consistently are a declared bound (D:459; driven: the row and its census entry deleted together → `close`) | the clause table is a view held to the derivation (S:2823-2835); the run's record names every finding (S:2129-2136). Driven: exit 13 for each, naming what is gone | `what_is_deleted_is_missed_and_named` (`m06`), `the_clause_table_is_rendered…` (`m15`) |
| `F7` MEDIUM | a stop is lifted by three writes that are not the go | **closed** for all five sites — but see R9 for a run whose opening is not done | a round-naming write is taken for a begun round or the one the state hands out (S:2041-2048); the stop mode is written once; a bound is raised at its own stop. Driven: all five refused | `a_stop_is_lifted_by_its_go_and_by_no_other_write`, `every_writer_and_every_fact…` (`m05`, `m11`) |
| `F8` LOW | `retest → retest` | **closed** | an item's runs in its round are counted from the results; two, then `human_clauses`; one more by `granted`. Driven | `a_rerun_is_an_attempt…`, the truth tables (`m07`) |
| `F9` LOW | a go given once silences every later stop of its round | **closed** | the go holds the count of fix cycles it was given at (S:1938-1941). Driven | `a_stop_is_lifted_by_its_go…` (`m08`) — **not** by the test named for the go |
| `F10` LOW | `admitted` is not inherited across a fix that did not hold | **unchanged**, recorded as a declared bound (D:473); the safe side | — | not re-driven |
| `F11` LOW | three inputs end in a traceback | **closed** for the three (exit 7, 7, 13), and a last catch gives any other one line and exit 23 | S:1666-1678, S:3159, S:3626-3633. Driven | `no_input_ends_in_a_traceback` (green; not mutated). New instances of a write the reader then refuses: R12 |
| `F12` LOW | re-opened is cleared by the same sha in another length | **closed** | the fix is matched by prefix (S:2599-2603). Driven: still re-opened | the truth tables (`m09`) |
| `F13` LOW | the header claims what the code does not do | **partly**: the batch claim is true now, the stop mode is written once, and *WHAT THIS SCRIPT CANNOT KNOW* is stated (S:806-820). *Close only from a tested candidate* still stands beside `F2`; new claims that do not hold are R15 | — | — |
| `S1` | the suite certifies `F2` | **not closed** (U:5838-5842) | — | `m18` |
| `S2` | the oracle for `current` is the predicate respelled | **not closed** (U:7239-7242) | — | — |
| `S3` | fixture topology: no row rewritten by a later round | **closed** for `F1`'s shape (a fixture of its own, two hunts and a check on one clause); `F5`'s end state went with `alone` | — | `m01` |
| lead | a red check gets no ledger row | **built**: the call that records a red result files `red-<item>` (driven: `next = "triage"`, then `retest` once refuted) | S:2227-2243 | `a_red_run_is_filed_as_a_finding…` (green; `m09` does not touch it) |
| lead | a re-run that halts before its record reads as a plain `test` | **closed** by structure: the position carries `rerun`, and the attempt is counted in the round that selected the item | S:3211-3223 | `a_rerun_is_an_attempt…` |
| crash rows | cycle record, drop, part | the `fix` half's — not examined | | |

**Count: of F1–F13, ten closed (F1, F3, F4, F5, F6, F7, F8, F9, F11, F12), one partly (F13), two not (F2 —
deferred to the `fix` half and still required by the suite; F10 — recorded as a bound). None is worse.**

## The repair plan's changes that cover this part

**C0 — git acts as commands: structural, for the acts the `test` stage uses.** Each of `git-state`, `state`,
`begin`, `check-reports`, `push` and `record` is one command that prints one hashed line, and each was
driven here against real git. The record step's first half is still three commands an agent runs in order
(write the batch file, `apply`, the gate; H:1345-1352), which is fine as long as `apply` is the only one
that writes. The re-cut of a part is still prose (the `fix` half). What C0 did **not** bring with it is
repeatability: an act that stops between two of its own git commands (R3), or between its commit and the
record script (R2), leaves a state the act itself then refuses.

**C1 — results per item, the clause table derived: structural.** The table has no writer; `state` and every
writer whose table the derivation reads refuse a table that differs. The class the first review bounded —
*one row per clause, rewritten by whichever items a round ran* — is gone. Two inputs of the derivation are
left that a caller can still bend: an item's row after it has results (R6), and an item's clause against
the census (R5). The scopes, the results (counted in the round's record) and the round's facts are held.

**C3 — counters with an exit to the human: structural, for what `next` asks for.** Triage passes per finding,
runs per item and round, attempts per stage above the last that reached its record, the cycle bound as a
fact at which `next` is `stop`, a go per stop. Each was driven to its exit and one grant further, and each
is held by a test (`m03`, `m07`, `m12`, `m17`, `m08`). Two things it does not count: a round with no fix
stage (R11 — reachable only by an invocation `next` did not ask for), and a `fix` record that counts no
cycle (declared, S:801-804).

**C4 — the atomic, journaled record step: structural between its own files; a patch around git.** What a
fresh invocation reads after a kill at each point, driven in a clone on a `test` stage's record of seven
calls and seven files (`os.replace` #1 is the journal, #2–#8 the files, #9 the journal again):

| The record step stops… | The journal holds | `dev/stabilize-step git-state` | `dev/stabilize-record state` | What finishes it |
|---|---|---|---|---|
| in the scan (the scanner cannot run, or the call is killed there) | nothing | `ready` | the round is not tested: `test`, the next attempt | nothing to finish |
| after its temporaries, before the journal (#1) | nothing; eight temporaries | **`dirty`, no command named** | the state before the batch, exit 0 | `recover`, or any writer — which no stage reaches (**R4**) |
| journal written, nothing placed (#2) · between two files (#4, #7) · every file placed (#9) | the placing and the batch | `record`, naming `recover` | `interrupted` (17), naming `recover` | `recover`: the batch is applied and pending; the same batch again is refused `pending` |
| `apply` returned | the batch | `ready`, `pending` = the batch | the applied tables | the next invocation gates and commits it |
| commit step, a gate newly red | the batch | `ready`, `pending` | the applied tables | the commit step again with another gate (driven), or `discard` (driven: the tables as before, reports and scope kept, the attempt uncounted) |
| commit step, between `git add` and `git commit` — killed, or the commit fails | the batch | **`dirty`, *the human reconciles it*** | the applied tables — it answers as if recorded | nothing of the tooling (**R3**) |
| commit step, between `git commit` and `settle` | the batch, now stale | **`ready`, `pending: null`** | the committed tables | `settle`, by hand; until then every `apply` is refused `pending`, and the `discard` its refusal names reverts committed tables (**R2**) |
| after `settle`, the push fails | nothing | **`ready`** — nothing says the branch is ahead | the committed tables | `dev/stabilize-step push`, by hand (**R7**) |

So the plan's sentence for C4 — *a red gate leaves a pending batch that the next invocation commits or
discards* — holds, and so does the journal. The four rows in bold are the record step's own, and none is
the `fix` half's.

**C5 — the writer × state table: held, by two tables together (R14).** The script's own table is a census
that makes a writer or a fact without a row a fault at every call (driven by the suite's three planted
additions). It decides almost nothing: the behaviour is in each subcommand. What holds the behaviour is
the suite's hand-written table — every writer and every fact at seventeen places, its rows held to the
script's own lists — and four mutants that take one writer's check out each turn it red. A writer cannot
be added without a row; a writer can be added with a wrong row in both tables, and then it is as held as
its author's cells. Two of its cells are R9 (`the opening: nothing written`) and R11 (`the human's
clause`), written as intended behaviour.

## The two points the builders named

- **4(a)** — R9. Verified, and wider than recorded: not round 1's first files but any round's, a fix cycle
  and a bound, while the opening is not done. No stage reaches it. LOW.
- **4(b)** — R10. A `test` after a dropped round launches no hunt **only if the scope step writes a scope
  that reaches none**; the record script takes whatever it writes. The state says that the round was
  dropped in one place, the dropped round's own facts — not in the position a stage reads its work from —
  and the harness hands the scope step the dropped fixes' door lists as inputs. Unreachable while only the
  `fix` stage writes `outcome`. LOW for `test`; a lead the `fix` half's repair should take.

## What the `test` stage's correctness rests on that the doc lists as still defective

1. **The path rule** (D:414) is asserted by `git-state` at the start of every `test` invocation
   (T:432-442). Its listed defects bear on `test` as they stand: any merge on the loop branch after the
   opening that is not a round's by its subject — a merge of `main` among them — halts every later `test`.
2. **Every round after the first.** A round is over, and its candidate no longer current, only by facts
   the `fix` stage writes (`cycles`, `outcome`). Until that stage is fit, a fix that reaches the loop
   branch any other way and is recorded as `fixed` is the first review's `F2`: the state answers `close`
   on the pre-fix candidate. So `test` is sound for **round 1 and what follows it without a fix** — the
   triage that finishes it, a re-run, the rulings — and no further.
3. **The reconcile the repair plan puts under C2** (*unpushed means push owed*) is what R7 lacks, and the
   window is the `test` stage's own record step.
4. Not a dependency: `fixed_in`, `land`, the part and the drop are read by `test` only through (2).

## Constructed and found correct

- **Every repro of the first review but `F2`'s** now answers as that review expected (the table above).
- **The journal** between its own files, at every point (the table above), for `triage-set` and for `apply`.
- **A red gate in a record step**: nothing committed, the batch kept; a second gate that is clean commits
  exactly the pending paths under the batch's subject, and the tree is clean after.
- **The attempt begun on record**: `begin` refuses a run whose opening is not done, and an attempt the
  position does not hand out; two markers with no record make the stage the human's.
- **A red check**: filed as `red-<item>`, ungraded; `triage`; after a refutation the clause is still red
  and its re-run is asked.
- **A go**: taken only at its stop; it stands until one more fix cycle is counted, and then the round
  stops again.
- **The suites, unmodified, on a clone of the tip**: 118 passed, 0 failed (`dev_stabilize_record` 67,
  `dev_stabilize_step` 27, `stabilize_simulation` 19, `placed_executable` 5), outside the gate, in a
  private target directory.

## The mutants

Each is one replacement in the clone's script; *red* is what the named tests then say. All eighteen are
killed.

| | What it takes out | Red |
|---|---|---|
| `m01` | an item is owed only by the latest tested round (`F1`) | 6: the dedicated test, `a_clauses_evidence…`, `the_humans_go…`, the three truth tables (the run was stopped once these were red) |
| `m02` | a selected item with no run is green, not void (`F1`) | 1: `the_clause_table_is_rendered…` |
| `m03` | a finding's triage passes are never spent (`F3`) | 5, one of them the simulation's |
| `m04` | a two-file call is not journaled (`F4`) | 1 of the whole record suite |
| `m05` | a write is taken for any round (`F5`, `F7`) | 2 |
| `m06` | the ledger is not held to its census (`F6`) | 1 |
| `m07` | an item's re-run is never spent (`F8`) | 4 |
| `m08` | a go stands for ever (`F9`) | 1 of the whole record suite and the simulation: `a_stop_is_lifted_by_its_go…` |
| `m09` | the same fix is matched by its text (`F12`) | 2 |
| `m10` | `gate-set` no longer asks the state (C5) | 1 |
| `m11` | the stop mode is rewritable (C5, `F7`) | 3 |
| `m12` | a stage's attempts are never spent (C3) | 2 |
| `m13` | a re-run is taken unasked (C1, `F5`) | 2 |
| `m14` | a second batch is applied over a pending one (C4) | 1 |
| `m15` | a clause table that differs is read (C1, `F6`) | 1 |
| `m16` | `git-state` admits any tree (C0, C4) | 3 of the step suite |
| `m17` | the cycle bound is no stop (C3) | 3 |
| `m18` | *probe:* the candidate is not current while a row says `fixed` | 3 — the truth tables **require** the `F2` close |

`m01`, `m04` and `m08` ran against whole suites; the others against the tests named for the mechanism and
the three truth tables, so *red* is a floor for them, never the full list.

## Not examined

- The `fix` half: `open-round`, `find-round`, `land`, `round-commits`, `carry`, the cycle record, the drop
  and the part — read, not driven, beyond what the tables above say. `sync-main` likewise.
- The harness beyond the lines cited; the agent definitions; `dev/gate`; `dev/regression-set`. Whether an
  agent runs a step's one command and relays its line whole is the canary's.
- No real `gitleaks` and no real denylist: every scan here met a stub and a synthetic list. R1 does not
  depend on either.
- Concurrency: two writers at once, a reader during a write, a report beside a batch — the lock was read,
  and the suite's arms were green; nothing was raced here.
- `placed_executable.rs` was read and ran green on macOS, where by its own account it cannot go red; it
  was not run in the Linux container.
- The simulation's golden trace was not adjudicated. The first review's nextest mark was not looked into.
- `scope.md` as a member of R1's class; a sweep for R12's class; `F10`.
- Leads, not pursued: a round's `gate.md` and its `candidate` are each written once and are not held to
  one commit; which reporters an attempt launched is in the journal's `checks` and in no committed table
  once the batch is settled.

## Appendix — the helpers, verbatim

Save the three files in one directory, `cd` there, and set `SRC` to the repository's root and `S` to a
scratch root of your own (`S=$(mktemp -d <scratch>/review.XXXXXX)`).

**`lib.sh`**

```sh
# The rigs of this review. Source it with two variables set:
#   SRC  the repository's root (a checkout that holds cbb3d736)
#   S    a scratch root of your own, minted with mktemp -d
: "${SRC:?the repository root}" "${S:?a scratch root}"
TIP=cbb3d73663ab05bd6981f44bfd2defd209de3913
C1=1111111111111111111111111111111111111111
C2=2222222222222222222222222222222222222222
C3=3333333333333333333333333333333333333333
F1=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
F2=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
PREV=9999999999999999999999999999999999999999
henv() { # hermetic environment for one command
  env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE -u STUB_GITLEAKS \
    PATH="$RIG/bin:$PATH" HOME="$RIG/home" TMPDIR="$RIG/tmp/" JIGC_DENYLIST_FILE="$RIG/denylist" \
    PYTHONPYCACHEPREFIX="$RIG/tmp/pycache" GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
    GIT_AUTHOR_NAME=rig GIT_AUTHOR_EMAIL=rig@example.invalid GIT_COMMITTER_NAME=rig GIT_COMMITTER_EMAIL=rig@example.invalid \
    ${STUB:+STUB_GITLEAKS=$STUB} "$@"
}
mkbase() { # the non-repo parts of a rig: home, tmp, bin with a stub gitleaks, a denylist
  mkdir -p "$RIG/home" "$RIG/tmp" "$RIG/bin"
  cat > "$RIG/bin/gitleaks" <<'STUBEOF'
#!/bin/sh
code=1; report=; repo=
while [ $# -gt 0 ]; do
  case "$1" in
    --exit-code) code=$2; shift ;;
    --report-path) report=$2; shift ;;
    --config|--report-format|--log-opts) shift ;;
    -*|git) ;;
    *) repo=$1 ;;
  esac
  shift
done
case "${STUB_GITLEAKS:-scan}" in
  crash) echo "stub gitleaks: could not load its config" >&2; exit 1 ;;
  kill-parent) kill -9 "$PPID"; exit 0 ;;
esac
if [ -n "$report" ]; then printf '[]' >"$report"; fi
exit 0
STUBEOF
  chmod 755 "$RIG/bin/gitleaks"
  printf '# a private term\n\nzzqprivatetermzz\n' > "$RIG/denylist"
}
newrig() { # light rig: the committed bytes of the tools in a small repository with a bare remote; run1 opened
  RIG=$(mktemp -d "$S/rig.$1.XXXXXX") || return 1
  R="$RIG/work"; mkbase
  mkdir -p "$R/dev" "$R/completions/artifacts/run1"
  for f in dev/stabilize-record dev/stabilize-step dev/hygiene-scan dev/merge-logs .gitleaks.toml; do
    git -C "$SRC" show "$TIP:$f" > "$R/$f" || return 1
  done
  chmod 755 "$R"/dev/*
  printf '# the opening record\n' > "$R/completions/artifacts/run1/opening.md"
  henv git init -q --bare "$RIG/remote.git"
  henv git -C "$R" init -q -b fix/run1
  henv git -C "$R" add -A && henv git -C "$R" commit -q -m "the opening"
  henv git -C "$R" remote add origin "$RIG/remote.git"
  henv git -C "$R" push -q origin fix/run1
  echo "$RIG" > "$S/cur.$1"
  echo "rig $1: <scratch>/$(basename "$RIG")"
}
userig() { RIG=$(cat "$S/cur.$1"); R="$RIG/work"; }
sr() { henv "$R/dev/stabilize-record" "$@"; }
ss() { henv "$R/dev/stabilize-step" "$@"; }
st() { # selected fields of state
  sr state --run run1 | python3 -c '
import json,sys
d=json.load(sys.stdin)
for f in sys.argv[1:]:
    v=d
    for p in f.split("."):
        v=v[int(p)] if isinstance(v,list) else v.get(p) if isinstance(v,dict) else None
    print("%s = %s" % (f, json.dumps(v)))' "$@"
}
opening() { # run-set with the usual facts plus extra JSON members: opening '"stop":"at-the-bound","rounds":3' 'clause-a,clause-b'
  local cl; cl=$(python3 -c 'import json,sys; print(json.dumps(sys.argv[1].split(",")))' "${2:-clause-a}")
  echo "{$1,\"previous\":\"1.0.0-rc.24\",\"previous-commit\":\"$PREV\",\"scope\":\"delta\",\"clauses\":$cl}" | sr run-set --run run1
}
items() { echo "$1" | sr item-set --run run1; }
scope() { # scope N 'door1;door2' 'excl1;excl2'
  python3 -c '
import json,sys
mk=lambda s:[{"door":d,"registry":"reg-"+d.split()[-1],"derivation":"derived"} for d in s.split(";") if d]
print(json.dumps({"included":mk(sys.argv[1]),"excluded":mk(sys.argv[2] if len(sys.argv)>2 else "")}))' "$2" "${3:-}" | sr scope-set --run run1 --round "$1"
}
finding() { # finding KEY DOOR [ROUND] [CLAUSE]
  echo "{\"key\":\"$1\",\"doctype\":\"jigc-feedback\",\"round\":${3:-1},\"source\":\"review\",\"door\":\"$2\",\"clause\":\"${4:-clause-a}\",\"repro\":\"r\"}" | sr ledger-add --run run1
}
triage() { echo "$2" | sr triage-set --run run1 --round "$1"; }
lset() { echo "$1" | sr ledger-set --run run1; }
facts() { echo "$2" | sr round-set --run run1 --round "$1"; }
results() { echo "$3" | sr result-set --run run1 --round "$1" --commit "$2"; }
report() { # report ROUND STAGE REPORTER ATTEMPT [CYCLE]
  printf '# a report\n\n<!-- end of report -->\n' | sr report --run run1 --round "$1" --stage "$2" --reporter "$3" --attempt "$4" ${5:+--cycle $5}
}
srk() { # srk KILL_AT argv... : the record script, killed before the Nth call of an os function
  local at=$1; shift; KILL_AT=$at henv env KILL_AT=$at python3 "$S/killat.py" "$R/dev/stabilize-record" "$@"
}
sums() { ( cd "$R/completions/artifacts/run1" && find . -type f ! -name opening.md | sort | while read f; do printf '%s %s\n' "$(shasum -a 256 < "$f" | cut -c1-10)" "$f"; done ); }
# --- the full clone -----------------------------------------------------------------
newclone() { # newclone LABEL: a clone of the repository at the reviewed tip, with a bare remote of its own, on a loop branch fix/run1 with run1 opened, committed and pushed
  RIG=$(mktemp -d "$S/cl.$1.XXXXXX") || return 1
  R="$RIG/work"; mkbase
  git clone -q --bare "$SRC" "$RIG/remote.git" && git -C "$RIG/remote.git" remote remove origin
  henv git clone -q "$RIG/remote.git" "$R"
  henv git -C "$R" switch -q --detach "$TIP" && henv git -C "$R" switch -q -c fix/run1
  mkdir -p "$R/completions/artifacts/run1"; printf '# the opening record\n' > "$R/completions/artifacts/run1/opening.md"
  henv git -C "$R" add completions/artifacts/run1 && henv git -C "$R" commit -q -m "docs(record): run1 opened" && henv git -C "$R" push -q origin fix/run1
  echo "$RIG" > "$S/cur.$1"; echo "clone $1: <scratch>/$(basename "$RIG")"
}
RD=completions/artifacts/run1
gs() { ss git-state --stage "${1:-test}" --loop fix/run1 --rounds fix/run1-r --run-dir $RD --product crates; }
ssk() { local b=$1; shift; henv env KILL_BEFORE="$b" python3 "$S/killstep.py" "$R/dev/stabilize-step" "$@"; }
line() { python3 -c '
import json,sys
d=json.loads(sys.stdin.read() or "{}")
for f in sys.argv[1:]:
    v=d
    for p in f.split("."):
        v=v.get(p) if isinstance(v,dict) else None
    print("  %s = %s" % (f, json.dumps(v)))' "$@"; }
gatefile() { # gatefile NAME pass | gatefile NAME fail "test a::b" ...
  local f="$RIG/$1.gate.txt" v=$2; shift 2
  { echo "gate: mode   full, keep-going"
    if [ "$v" = pass ]; then echo "tests   passed=10 failed=0  (over 3 test binaries)"; echo "GATE: PASS"
    else echo "failing tests:"; for t in "$@"; do echo "  $t"; done; echo "tests   passed=9 failed=$#  (over 3 test binaries)"; echo "GATE: FAIL (step: test)"; fi; } > "$f"
  echo "$f"
}
stage1() { # in a clone: the opening done and pushed; attempt 1 of round 1 begun; a scope; two reports; the batch of the stage's record written to $RIG/batch.json
  opening '"stop":"at-the-bound","rounds":3' clause-a >/dev/null
  items '[{"item":"gate","kind":"check","clause":"clause-a","runs":"every-candidate","brief":"b"},{"item":"review-setup","kind":"review-row","clause":"clause-a","runs":"in-scope","doors":["jigc setup"],"brief":"b"}]' >/dev/null
  henv git -C "$R" add $RD && henv git -C "$R" commit -q -m "docs(record): run1, the opening's facts" && henv git -C "$R" push -q origin fix/run1
  CAND=$(henv git -C "$R" rev-parse HEAD)
  ss begin --run run1 --round 1 --stage test --attempt 1 --reporter attempt --commit $CAND --scratch "$RIG/tmp" >/dev/null
  scope 1 "jigc setup" >/dev/null; report 1 test scope 1 >/dev/null; report 1 test review-setup 1 >/dev/null
  GATE=$(gatefile cand pass)
  python3 - "$CAND" "$GATE" > "$RIG/batch.json" <<'PY'
import json,sys
c,g=sys.argv[1:]
J=json.dumps
print(J([
 {"argv":["check-reports","--run","run1","--round","1","--stage","test","--attempt","1","--","attempt","scope","review-setup"]},
 {"argv":["gate-set","--run","run1","--round","1","--commit",c,"--summary",g]},
 {"argv":["result-set","--run","run1","--round","1","--commit",c],"stdin":J([{"item":"gate","outcome":"green"},{"item":"review-setup","outcome":"green"}])},
 {"argv":["ledger-add","--run","run1"],"stdin":J([{"key":"f-1","doctype":"jigc-feedback","round":1,"source":"review-setup","door":"jigc setup","clause":"clause-a","repro":"r"}])},
 {"argv":["triage-set","--run","run1","--round","1"],"stdin":J([{"key":"f-1","grade":"breaks","verdict":"refuted"}])},
 {"argv":["round-set","--run","run1","--round","1"],"stdin":J({"candidate":c,"binary":"b"*64})},
 {"argv":["check-ledger","--run","run1","--","f-1"]},
]))
PY
}
applyb() { sr apply --run run1 --round 1 --subject "docs(record): run1 r1 — the test stage" < "$RIG/batch.json"; }
recordb() { ss record --branch fix/run1 --run-dir $RD --gate "${1:-$GATE}" --calls 7 --checks 2; }
porcelain() { henv git -C "$R" status --porcelain --untracked-files=all | sed 's/^/   /'; }
```

**`killat.py`**

```python
# Runs dev/stabilize-record (argv[1]) unmodified, and SIGKILLs the process just BEFORE the
# Nth call of one os function: KILL_AT=replace:N | link:N | unlink-journal:1
import os, runpy, signal, sys
what, _, nth = os.environ["KILL_AT"].partition(":")
nth = int(nth); seen = {"n": 0}
def die():
    sys.stderr.write("killat: SIGKILL before %s #%d\n" % (what, nth)); sys.stderr.flush()
    os.kill(os.getpid(), signal.SIGKILL)
def wrap(real, counts):
    def inner(*a, **k):
        if counts(*a):
            seen["n"] += 1
            if seen["n"] == nth: die()
        return real(*a, **k)
    return inner
if what == "replace": os.replace = wrap(os.replace, lambda *a: True)
elif what == "link": os.link = wrap(os.link, lambda *a: True)
elif what == "unlink-journal": os.unlink = wrap(os.unlink, lambda *a: str(a[0]).endswith(".pending.json"))
script = sys.argv[1]; sys.argv = sys.argv[1:]
runpy.run_path(script, run_name="__main__")
```

**`killstep.py`**

```python
# Runs dev/stabilize-step (argv[1]) unmodified and SIGKILLs the process just BEFORE the
# first child command whose argv contains every word of KILL_BEFORE (space-separated).
import os, runpy, signal, subprocess, sys
words = os.environ["KILL_BEFORE"].split()
real = subprocess.run
def run(argv, *a, **k):
    flat = " ".join(argv)
    if all(w in flat for w in words):
        sys.stderr.write("killstep: SIGKILL before `%s`\n" % " ".join(os.path.basename(x) if os.path.isabs(x) else x for x in argv[:3])); sys.stderr.flush()
        os.kill(os.getpid(), signal.SIGKILL)
    return real(argv, *a, **k)
subprocess.run = run
script = sys.argv[1]; sys.argv = sys.argv[1:]
runpy.run_path(script, run_name="__main__")
```
