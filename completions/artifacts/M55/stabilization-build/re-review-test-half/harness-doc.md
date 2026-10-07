# Re-review B — the harness, the agents, the gate, the regression tool, and the workflow doc

*Independent, read-only re-review (`milestone-code-reviewer`) of the repair of the `test` half, `603018ac^..cbb3d736` on `fix/rc24-tier1`, returned 2026-10-07. The first review of this part is [harness-agents-gate.md](../harness-agents-gate.md); its finding ids (`H…`, `M…`, `L…`) are used below as it uses them, and this report's own are prefixed `R-`. Line numbers are at `cbb3d736`. A second reviewer read the record script and the state machine at the same time; neither saw the other's report.*

**Verdict: fit for a canary run of `test` — shaped as the last section says, not as one planted finding. Not yet fit for the real run after it.** Nothing found lets a `test` stage record something false: every defect below fails to the safe side — a halt, an attempt counted, a finding left with the human. What they cost is the stage: two of them (`R-H1`, `R-H2`) are the likeliest ways the first real stage ends with hours of instruments run and nothing recorded, one (`R-H2`) was introduced by the repair of `M7`, and the canary as the record describes it would show neither. One driven defect is a bug in a path no test executes (`R-M4`). The workflow doc says of its own list that nothing on it is the `test` stage's; one member of the list's last class is (`R-M1`).

**Scope:** `.claude/workflows/stabilize.js`; the definitions it launches (`stabilize-preflight`, `stabilize-scope`, `finding-triage`, `finding-verifier`, `build-git`, `build-executor`, `milestone-code-reviewer`, `milestone-e2e-tester`, `robust-advocate`); `dev/gate`; `dev/regression-set` and its record; `dev/stabilize-step` as far as the harness's steps go through it; `tooling-tests/stabilize_simulation.rs`, `stabilize_harness_fence.rs`, `dev_gate_report.rs`, `dev_regression_set.rs` and `fixtures/stabilize-runtime.mjs`; and `implementation/stabilization-workflow.md`, whole. All read from committed state.

## How evidence was obtained

- **Run, as committed:** the four suites — `cargo test -p jigc --test g_tooling -- stabilize_simulation:: stabilize_harness_fence:: dev_gate_report:: dev_regression_set::` — **101 passed, 0 failed** (19 · 16 · 42 · 24), exit 0. The harness's self-test under the committed stand-in: `self-test-passed`, 373 checks.
- **Driven:** the committed harness, unmodified, under the committed runtime stand-in (`tooling-tests/fixtures/stabilize-runtime.mjs`), in rigs built as the simulation builds its own (*The rig*, below) — real `dev/stabilize-record`, real `dev/stabilize-step`, real git with a bare remote, the real gitleaks. Every scenario is a JSON file handed to the stand-in; each repro block gives it whole. One scenario needed a copy of the stand-in with one line changed (`R-H2`), because the committed one cannot script it; two used a mutated copy of the harness (the table of the first review's findings, `M10` and the `fix` refusal).
- **Measured:** which lines of the harness the simulation executes — `NODE_V8_COVERAGE` over the simulation suite, block coverage merged over the 78 invocations its 19 tests make (*The simulation*, below).
- **Driven without cargo:** `dev/gate` under a stand-in `cargo` (eleven mode × red combinations); `dev/regression-set check-list` and `verdict` over evidence written for the purpose. **Not run:** the regression tool's real run, the whole gate, any agent.
- **Everything else:** traced in source, and said to be.

### The rig

`<scratch>/rig-<label>.<minted>/` holds `repo/` (a git repository with `dev/stabilize-step`, `dev/stabilize-record`, `dev/merge-logs`, `dev/hygiene-scan` and `.gitleaks.toml` copied from the tree under review, `Cargo.toml`, `crates/a.txt`, the two logs), `origin.git` (bare, the remote), `scratch/` (the invocation's `scratch`), a denylist of one term, a home and a temp directory of its own. Every child runs with `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL=/dev/null`, a fixed identity and fixed dates. The run `rc24` is opened on `fix/rc24` as *Opening a run* has it — `run-set` (`stop: every-round`, `previous: 1.0.0-rc.24`, `scope: delta`, the clauses), `item-set`, optionally `ledger-add`, one commit, pushed — and is ready when `state` prints `"not_ready": []` and `"next": "test"`. An invocation is `node tooling-tests/fixtures/stabilize-runtime.mjs .claude/workflows/stabilize.js <scenario.json>` from `repo/`. **Control:** one `audit-area` item, a reviewer that finds nothing → `status: triaged`, twelve agents from `git:state` to `git:state:test-3`, origin at the local head, tree clean.

In the blocks below `ONE_DOOR` is `{"included": [{"door": "jigc doc set", "registry": "verbs", "derivation": "scripted"}], "excluded": [], "uncovered": [], "reached_but_excluded": [], "left_open": []}`, `NOTHING` is `{"status": "reported", "findings": [], "summary": "Nothing found."}`, and a triage return is `{"status": "graded", "entries": […], "to_verify": [], "counts": {"findings_in": […], "entries": N, "merged": 0}}`.

## HIGH

*Neither can put something false on record. Each can cost the real run its stage.*

### R-H1 — The state document is relayed whole through a Sonnet agent's return, three times a stage; at the real run's size that is 75–150 KB, and the canary as planned relays 3 KB

- **Where:** `dev/stabilize-step:451-466` (the `state` act prints the whole document inside its one line, `json.dumps` with its default `\uXXXX` escapes, `:788`); `stabilize.js:1854-1869` (`readState`: the relayed line is held to its hash, asked for again twice, then the stage halts), called at `:2209`, `:2271`, `:2369`; `stabilize.js:2123-2128` with `:275` (`nextOf` returns the whole state for every `next` but `fix`, `rule`, `close`).
- **Evidence (measured, on a rig whose ledger and scope were written through the record script):**

  | ledger rows | doors in the round's scope | the ONE line a git step must relay | `\u` escapes in it | double quotes in it |
  |---|---|---|---|---|
  | 1 | 2 | 3,172 bytes | 12 | 376 |
  | 20 | 40 | 25,317 bytes | 227 | 2,714 |
  | 60 | 150 | 74,949 bytes | 793 | 7,638 |
  | 120 | 300 | 142,899 bytes | 1,573 | 14,358 |

  The line goes into the agent's structured return as the string `line`, so every one of those quotes and backslashes is escaped once more on the way; one character off and the hash does not hold. The doors rows carry their derivations, the ledger rows their repro cells; the dashes and arrows this project writes everywhere are what the `\u` escapes are.
- **Why the real run is at the large end.** The fix pass this run measures left a findings ledger of several hundred rows (`completions/artifacts/M55/fix-pass-rc25/findings-ledger.md`, 173 KB); round 1's scope is the whole delta since the previous release, and a scope that cannot be bounded puts the whole registry inside, both sides listed.
- **What follows a relay that does not hash:** `readState` asks twice more; then `halt('state')`. At the second read that is after the preflight's two builds and the candidate's gate, with the attempt counted; two such attempts and the stage is the human's (`attempts-spent`).
- **The same document goes back to the orchestrator.** Driven: a clean stage over a ledger of 60 rows returned 38,602 bytes, 37,686 of them `state`, because `next` was `stop` — as it is after every round of the first run. The doc says the main session only orchestrates and that long reports are read by subagents.
- **Class:** what the harness reads through an agent's hands. Bounded by the acts of `dev/stabilize-step`: twelve acts; only `state` carries a document that grows with the run — `record` carries the pending paths and the gate's red list (a few KB), the others a handful of fields. The record step's payload (`batch.json`, written by the executor from its prompt by here-document and held to its hash, `:1316-1322`) grows the same way with the number of findings and is the second member; it was not measured.
- **Fixture-topology masking:** the simulation's stand-in relays every line perfectly — `readStep`'s `lost` and `readState`'s *again* loop are executed by no test (coverage, below) — and the canary the record plans is one planted finding. Neither can show this.
- **Smallest change:** the `state` act prints what the harness reads and no more — the position, `next`, the items, the doors without their derivations, the untriaged rows, the keys of the rest — and leaves the document in the file it already writes (`<scratch>/state/<tag>.json`); `nextOf` returns that file's path instead of the document. Short of that: the canary's run is opened with a ledger and a scope of the real run's size.

```yaml
claim: "the line `dev/stabilize-step state` prints grows with the ledger and the scope, to ~75 KB at 60 rows and 150 doors"
verdict: CONFIRMED
setup:
  - rig: opened, one audit-area item and 12 review rows, `ledger-add` of 60 rows (door, clause and repro cells of one line each)
  - ["dev/stabilize-record", "scope-set", "--run", "rc24", "--round", "1", "--scratch", "<scratch>"]   # stdin: 150 included doors, each with a one-line derivation
repro:
  - ["dev/stabilize-step", "state", "--run", "rc24", "--scratch", "<scratch>", "--tag", "x"]
expect:
  exit: 0
  stdout_bytes: 74949        # one line; 793 occurrences of `\u`
pinned-by: "UNPINNED: no test holds the size of a relayed line, and no test relays one that fails its hash"
```

### R-H2 — One field left out of one return halts the stage after every instrument has run; the reviewer on a source pass is the likeliest to leave it out, and the repair of `M7` is what made it one

- **Where:** `stabilize.js:626-628` (`hashMismatch`: a role that `drives`, did not halt, and returned any `asserted_sha256` but the binary's — `undefined` included), `:2314-2317` (compared after `runUnits` returned, so after every chain of every item); `:322-334` (`ROLES.review` is `drives: true` since the repair); `:941-953` (`UNIT_SCHEMA` requires `status` and `findings`, nothing else). And the verifier's two: `:2026-2033` with `:2180`/`:2343` — a `confirmed` verdict without `ran_on.previous`, or without `regression`, is a *fault*, and a fault halts the stage (`halt('binary', …)`); `VERIFY_SCHEMA` (`:994-1012`) requires neither.
- **Why a reviewer leaves it out.** `milestone-code-reviewer.md` binds the assertion to driving: *"Before the first command you drive: `shasum -a 256 <path>` prints the hash the line gives … The hash you asserted goes into your return."* The source pass of a review row, the audit of an area and the cross-cutting pass read source; a reviewer that drove nothing asserted nothing. The first review's `M7` was that reviewers got no binary; the repair sends them the line **and** holds them to the hash like a driver.
- **Driven, three ways:**
  1. An `audit-area` reviewer returns its findings and its report and no `asserted_sha256` → `status: halted`, `phase: binary`: *"a driving agent did not assert the candidate's binary (…): area-a-review asserted null — nothing it drove is evidence about this candidate"*. Nothing recorded; `position.test` is `{round: 1, attempt: 2}`.
  2. A verifier returns `confirmed` with its hashes and no `regression` → `status: halted`, `phase: binary`: *"the verifier of `k-one` confirmed it and did not say whether it is a regression"*.
  3. (Traced, `:2026`) a verifier returns `confirmed` for a door that did not exist on the previous release — its definition's own *not comparable* case — and gives no `ran_on.previous`: the same halt.
- **What it costs:** the stage's instruments are run, their reports are on disk, and nothing is recorded; the next attempt runs all of them again; a second such attempt and the stage is the human's.
- **The rule the header states** — *a returned hash that is not the binary's … halts the stage* — is about a driver that drove something else. An absent field is a different ending, and it is in neither table of endings.
- **Class:** fields a stage halts on that no schema requires. Enumerated from every `halt(` and `faults.push(` the `test` stage can reach against the `required` lists: `asserted_sha256` (units, advocate `:2085`, independent drive `:2093`), `ran_on.previous` and `regression` (verifier). Three fields, five sites.
- **Simulation:** the stand-in always returns a hash where the prompt carries a binary line (`stabilize-runtime.mjs:288`) and always returns `ran_on.previous` (`:289`), so neither omission can be scripted; `:2031`, `:2085` and `:2093` are executed by no test.
- **Smallest change:** a schema per role that requires what the stage halts on, so the runtime asks again; and an item whose reporter returned no hash is *void* with that reason — the cell every other failure of a chain's step already has — rather than a halt of the stage.

```yaml
claim: "a reviewer that returns no `asserted_sha256` halts the stage after its instruments ran"
verdict: CONFIRMED
setup:
  - rig: opened, clauses [audit-clean], items [{item: area-a, kind: audit-area, clause: audit-clean, runs: every-candidate}]
  - stand-in: a copy of stabilize-runtime.mjs whose line 288 opens `if ((binary || spelled) && !(scenario.noHash || []).includes(label))`
repro:
  - scenario: {args: {stage: test, run: rc24, scratch: "<scratch>"}, scope: ONE_DOOR, agents: {"area-a:review": NOTHING}, noHash: ["area-a:review"]}
expect:
  result: {status: halted, halted: {phase: binary}}
  agents: [git:state, git:state:test-1, git:begin, preflight, scope, git:state:test-2, area-a:review]
  state_after: {next: test, position: {test: {round: 1, attempt: 2}}}
pinned-by: "UNPINNED: the committed stand-in cannot script a return without the hash"
---
claim: "a verifier that confirms and leaves out `regression` halts the stage"
verdict: CONFIRMED
repro:
  - scenario: {scope: ONE_DOOR, agents: {"area-a:review": {one finding a-1}, "triage:p1": {one entry k-one, grade breaks}, "verify:k-one": {status: verified, key: k-one, verdict: confirmed, basis: "reproduces as written", repro: "the block", pinnable: true, left_open: []}}}
expect:
  result: {status: halted, halted: {phase: binary, reason: "the verifier of `k-one` confirmed it and did not say whether it is a regression"}}
pinned-by: "UNPINNED"
```

## MEDIUM

### R-M1 — A record that is committed and not pushed: the harness's advice is refused, nothing pushes it, and every later read answers as if it were pushed

- **Where:** `stabilize.js:2367-2368` (the halt), `:1835` (its last sentence — *invoke the stage again with the same args*), `dev/stabilize-step:317-325` and `:429-430` (`git-state` refuses a remote that is *ahead*; a local branch that is ahead passes). The same shape at `:1918`, `:2137`, `:2191`.
- **Driven:** origin refuses the push (a `pre-receive` hook). Invocation 1: `halted`, `phase: push` — *"the record is committed on fix/rc24 (…) and the branch was not pushed: push-rejected"*. The hook is removed — the cause is dealt with. Invocation 2, the same args: `git:state` answers ready, the state says the round is tested, and the return is `refused` (`stopped`), *"Nothing was run beyond the two reads"*. Local head `92806779`, origin `5e5cfb44`: not pushed, and nothing said so.
- **What pushes it, eventually:** the next record step of any kind — a go, a ruling, a finishing triage, a re-run. Where `next` is `fix`, nothing does: that stage refuses to start.
- **Why it matters here:** the doc's precondition for a stage is a loop branch that is *pushed*, because the check that reads CI needs a run for the candidate's commit — and nothing asserts it. And the doc says of its list of what is still defective: *"Nothing on it is the `test` stage's own."* The list's fifth item is *records around a halted git step*; this is that class, in `test`.
- **Class:** a halt after a commit the stage made and before its push. Bounded by grep of `was not pushed`: eight sites — four the `test` half reaches (`:1918`, `:2137`, `:2191`, `:2368`) and four in `runFix` (`:2474`, `:2481`, `:2497`, `:2545`). None of the first four is executed by a test (coverage).
- **Smallest change:** `git-state` reports a loop branch that is ahead of its remote as a push that is owed, and the stage's first act is that push; the halt's sentence says so.

```yaml
claim: "after a record commit whose push was rejected, the same invocation again is refused and pushes nothing"
verdict: CONFIRMED
setup:
  - rig: opened, one audit-area item
  - origin.git/hooks/pre-receive: "#!/bin/sh\nexit 1"   # executable
repro:
  - scenario: {args: {stage: test, run: rc24, scratch: "<scratch>"}, scope: ONE_DOOR, agents: {"area-a:review": NOTHING}}   # -> halted, phase push
  - remove: origin.git/hooks/pre-receive
  - scenario: the same                                                                                                       # -> refused (stopped)
expect:
  second_result: {status: refused, next: stop}
  second_agents: [git:state, git:state:test-1]
  git: "rev-parse fix/rc24 != ls-remote origin fix/rc24"
pinned-by: "UNPINNED"
```

### R-M2 — Two reporters that die halt the stage, where the table says each of them is a void; no test has a call after the breaker trips

- **Where:** `stabilize.js:1784-1811` (the breaker: two *different* labels exhausted, and every later call returns nothing — the git steps among them), `:635-637` (a report check that returned nothing is a fault), `:2039-2040`, `:2322-2323`.
- **Driven:** one `audit-area` reviewer, two findings graded *breaks*. *One* verifier dies: the stage reaches its record, `next: triage`, `unverified: [k-one — its verifier returned nothing]` — the cell as the table has it. *Both* die: `git:check-reports:p1` is never spawned, `halted`, `phase: triage`, *"the report check could not be read. The rate-limit breaker is tripped (verify:k-one, verify:k-two)"*; the ledger holds no row; the same again, and `next` is `rule` with the stage in `human_stages`, and a third invocation is `refused` (`attempts-spent`).
- **What is undecided:** whether two agents that died for a reason of their own — a repro neither could hold, a context that overflowed — are a rate limit. The breaker cannot tell, and the two tables of endings (the harness header's, the doc's) state each row without it. The doc names the breaker once, under *Halt and resume*.
- **Simulation:** one scenario kills two agents — the first preflight and the scope step, where the stage halts anyway. `:1789`, the line a call takes once the breaker is tripped, is executed by no test.
- **Smallest change:** the report check and the record step are not subject to the breaker — what was established is recorded, and the stage returns with what it could not launch as void or unverified.

```yaml
claim: "two dead verifiers halt the stage before anything is recorded"
verdict: CONFIRMED
setup: [ {rig: "opened, one audit-area item"} ]
repro:
  - scenario: {scope: ONE_DOOR, agents: {"area-a:review": {findings a-1, a-2}, "triage:p1": {entries k-one, k-two, grade breaks}, "verify:k-one": {…}, "verify:k-two": {…}}, endings: {"verify:k-one": dies, "verify:k-two": dies}}
expect:
  result: {status: halted, halted: {phase: triage, reason: "the report check could not be read"}}
  last_agents: [triage:p1, verify:k-one ×3, verify:k-two ×3]      # no git:check-reports:p1
  state_after: {next: test, position: {test: {round: 1, attempt: 2}}, ledger: []}
pinned-by: "UNPINNED"
```

### R-M3 — The reporters a stage stands on are first checked after every instrument has run; and the second preflight's missing report is a halt neither table has

- **Where:** `stabilize.js:2314` (`runUnits`), then `:2321-2325` (the first report check, and the halt on `ATTEMPT`, the first preflight, the scope step, and the second preflight where it provided).
- **Driven:** a trial arm; the *first* preflight returns `ready` and wrote no report → `preflight-second`, `arm-a:rehearse`, `arm-a:run`, `arm-a:score` all run, then `halted`, `phase: reports`. The same with the *second* preflight returning a verified image and no report: the three steps of the arm run on that image, then the same halt — *"a reporter this stage stands on returned and left no report: preflight-second"*.
- **Against the tables:** both give the second preflight one row — *no result, or an image that is not verified → goes on*. The ending *a result with no report* is a halt in the code, and is in neither.
- **Simulation:** the first preflight's two endings; the second preflight's `dies` only; the scope step's `dies` only.
- **Smallest change:** one `check-reports` after the preflight and the scope step, before anything is launched on what they established.

```yaml
claim: "a first preflight that left no report is found out only after the round's instruments ran"
verdict: CONFIRMED
setup: [ {rig: "opened, clauses [adoptable], items [{item: arm-a, kind: trial-arm, clause: adoptable, runs: every-candidate}]"} ]
repro:
  - scenario: {scope: ONE_DOOR, agents: {"arm-a:rehearse": NOTHING, "arm-a:run": NOTHING, "arm-a:score": NOTHING}, endings: {preflight: unreported}}
expect:
  result: {status: halted, halted: {phase: reports}}
  agents: [git:state, git:state:test-1, git:begin, preflight, scope, git:state:test-2, preflight-second, arm-a:rehearse, arm-a:run, arm-a:score, git:check-reports]
pinned-by: "UNPINNED"   # with `endings: {preflight-second: unreported}` the trace and the halt are the same, naming preflight-second
```

### R-M4 — A second triage pass that keys what was left open to a finding the first pass verified halts the stage at its record; and no test runs a second pass at all

- **Where:** `stabilize.js:1999-2003` — an entry under a key the stage already holds is merged over the earlier one, `Object.assign(held, e, { new: held.new })`, and the earlier pass's `verdict`, `regression`, `basis` and `fork` stay on it.
- **Driven:** pass 1 grades `k-one` *breaks*; its verifier confirms it and leaves one thing open. Pass 2 is handed that, keys it `k-one` — *found again*, as `finding-triage.md` tells it to for the same door and the same behaviour — and grades it `no-break`. The stage composes `{key: k-one, grade: no-break, verdict: confirmed}`, and the record's batch is refused: *"`triage-set`: entry "k-one": a verdict is the verifier's, and the verifier is sent breaks and unclear — not no-break — nothing of the batch was written"*. `halted`, `phase: record`, after everything ran; the ledger holds nothing.
- **The neighbouring case (traced):** pass 2 grades it `unclear`, a verifier is sent, and that verifier dies → the entry keeps pass 1's `confirmed` beside the new grade and is recorded so, while the return lists it under `unverified`.
- **A control:** a second pass that mints a *new* key works — `triage:p2`, `verify:k-two`, `git:check-reports:p2`, the record, two rows.
- **No committed test executes a second pass.** Every scripted return has `left_open: []`: `leftOpenSource` is never called, and `:2001`, `:2008` (the pass past `VERIFY_PASSES`), `:2055-2056`, `:2086`, `:2094` and `:2337` are executed by no test. This is ruling 5's *what an agent leaves open is triaged like any finding* — the reason triage and verification alternate — and it is a path the stand-in could script.
- **Class:** `instance, unbounded` for the merge — the other state a merged entry could carry was not enumerated; the coverage gap is bounded by the list of lines above.
- **Smallest change:** a re-graded entry drops what the earlier pass established for it; and a scenario with a verifier that leaves something open, in both keyings.

```yaml
claim: "a left-open entry keyed in pass 2 to a finding pass 1 confirmed, and graded no-break, halts the stage at its record"
verdict: CONFIRMED
setup: [ {rig: "opened, one audit-area item"} ]
repro:
  - scenario:
      scope: ONE_DOOR
      agents:
        "area-a:review": {status: reported, findings: [{id: a-1, door: "jigc doc set", clause: audit-clean, …}]}
        "triage:p1": {entries: [{key: k-one, new: true, grade: breaks, …}], counts: {findings_in: [{reporter: area-a-review, count: 1}], entries: 1, merged: 0}}
        "verify:k-one": {status: verified, key: k-one, verdict: confirmed, regression: false, left_open: ["the same door, another symptom"], …}
        "triage:p2": {entries: [{key: k-one, new: false, grade: no-break, …}], counts: {findings_in: [{reporter: verify-p1-k-one, count: 1}], entries: 1, merged: 0}}
expect:
  result: {status: halted, halted: {phase: record}}
  reason_contains: "`triage-set`): entry \"k-one\": a verdict is the verifier's"
  state_after: {ledger: []}
pinned-by: "UNPINNED"
```

### R-M5 — The triage definition asks for an entry the harness's arithmetic refuses

- **Where:** `finding-triage.md:37` — *"A finding that came without a repro block is graded `unclear` and said to lack one … and the missing block is itself an entry against that reporter's report."* `stabilize.js:1991-1992` — the entries plus the named merges must equal the findings handed in, or the stage halts: *"a finding in no entry and in no named merge is lost"*.
- **Driven:** one finding in; triage returns it graded `unclear` and one more entry for the block its report lacks → `halted`, `phase: triage`: *"was handed 1 finding(s) and accounts for 1 in, 2 entries and 0 merged"*.
- **Simulation:** `:1992` is executed by no test — the one scenario that touches the arithmetic balances it and trips the duplicate key behind it.
- **A smaller mismatch in the same definition:** its return names `new` *or* `found_again` for an entry; the schema has one boolean, `new`, and requires it (`:964-967`).
- **Smallest change:** one sentence of the definition — the missing block is said in the entry's `why` and in triage's report, and is no entry.

```yaml
claim: "a triage that returns the entry its definition asks for a missing repro block halts the stage"
verdict: CONFIRMED
repro:
  - scenario: {scope: ONE_DOOR, agents: {"area-a:review": {findings: [a-1]}, "triage:p1": {entries: [{key: k-one, grade: unclear, …}, {key: k-one-has-no-repro-block, grade: no-break, …}], counts: {findings_in: [{reporter: area-a-review, count: 1}], entries: 2, merged: 0}}}}
expect: {result: {status: halted, halted: {phase: triage}}}
pinned-by: "UNPINNED"
```

### R-M6 — The regression tool is green over a list no commit holds, over two commits that are one, and over rows nobody audited

- **Where:** `dev/regression-set:721-742` (`act_run`: the list is the file `--list` names; the two commits are each resolved and never compared), `:271-308` (a pointer is held to *exist* — a commit in the range, a heading in a file).
- **The list.** The doc: *"The list of intended changes is a committed file that the tool reads."* The tool reads whatever file it is handed; `DECISIONS.md` → *The regression set's first part*, items 4 and 13, says so (*"Nothing holds the list to a commit"*). **The first green on record was read from a file that was in no commit**: the run named the candidate `d423f09b`, the list entered the tree at `cbb3d736`, and the record says it itself — *"untracked in the tree when the run read it"*. Driven: `git cat-file -e d423f09b:<the list>` exits 128; `check-list` over a file in a scratch directory holding one row nobody ruled, pointing at an arbitrary commit of the range, answers `listed`, exit 0.
- **Inside a stage** the list is the candidate's by one fact: `git-state` holds the tree to the run's pending writes when the stage starts. Nothing holds it between that step and the check, and nothing compares the `list.sha256` the tool prints with the candidate's blob.
- **The two commits.** `check-list --previous S --candidate S` over an empty list answers `listed`; `run` resolves the two the same way. Two builds of one commit differ in bytes wherever a build embeds its path, so `hold_binaries` (`:396-401`) does not refuse them, and the verdict over them is green with no difference. The preflight composes this call from a brief (`R-M8`).
- **Who adds a row.** *"Never fixed by adding a row nobody ruled"* is held by nothing. The list lies outside the product paths, so a row commits directly on the loop branch, or rides a fixer's commit on a round branch — where the audit of the fix diff is *over the product paths* and does not read it.
- **Class:** what a green verdict rests on that the tool does not hold. Enumerated from `act_run`'s inputs: the list's file, the two commits, the scratch directory (held), and the rows' pointers (held to existence). Three unheld.
- **Smallest change:** `--list` is a path *in the candidate's commit*, read with `git show` (the stricter rule item 4 names and did not choose); `previous == candidate` is a refusal; and the round's record keeps the `list.sha256` a green was given over.

```yaml
claim: "the first green on record was given over a list the candidate's commit does not hold"
verdict: CONFIRMED
repro:
  - ["git", "cat-file", "-e", "d423f09bf1bdc98fa2169134e607a8c0b203919f:completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv"]
expect: {exit: 128}
---
claim: "a list in no commit, with a row nobody ruled, is read as a list; and so is an empty one over two commits that are one"
verdict: CONFIRMED
setup:
  - file: "<scratch>/uncommitted.tsv"   # jigc::g_flow <TAB> some_suite::a_test_a_fixer_broke <TAB> it changed <TAB> commit:<the 20th commit of 91834b5e..d423f09b>
  - file: "<scratch>/empty.tsv"         # no row
repro:
  - ["dev/regression-set", "check-list", "--previous", "91834b5e011de2c36e2be2b79e96c0b9f60a803c", "--candidate", "d423f09bf1bdc98fa2169134e607a8c0b203919f", "--list", "<scratch>/uncommitted.tsv"]
  - ["dev/regression-set", "check-list", "--previous", "d423f09bf1bdc98fa2169134e607a8c0b203919f", "--candidate", "d423f09bf1bdc98fa2169134e607a8c0b203919f", "--list", "<scratch>/empty.tsv"]
expect: {exit: [0, 0], stdout_json: {status: listed}}
pinned-by: "UNPINNED: dev_regression_set.rs has no arm for either"
```

### R-M7 — Up to one test in a hundred drops out of the comparison with the verdict green, and no list says which may; eleven do today, one of which reaches the binary

- **Where:** `dev/regression-set:194` (`BASELINE_TOLERANCE`), `:404-421`, `:484-487`.
- **Driven:** evidence of 200 tests in which two fail on their own binary *and* on the candidate's → `green`, exit 0, `excluded: 2`, `differences: 0`. With three → `void`, `baseline`. On the previous release's 4,393 tests the tolerance is 43.
- **What that admits:** a test that fails twice on the baseline for a reason of the machine — the first run was made at a load average between forty-five and eighty — is excluded, and a regression in it is not seen. The tool names it under `excluded`; the verdict does not change; a green reaches the round's record as one word (`R-M8`), so the names reach nobody.
- **The eleven.** All fail for one reason — the previous release is built from an archive, and each asks git about its own tree or history — and the record names each. One, `set_kind_vocabulary::no_schema_hash_moved_and_the_pinned_projection_is_byte_identical`, reaches the binary: it passed on both binaries in the measurement, which built from a linked worktree. The workflow doc's bullet (*"An archive is no repository … excluded, named"*) gives neither the number nor that one of them is coverage lost.
- **A cheap way to keep them.** The archive was chosen over a linked worktree because a worktree is a registration in this repository's `.git` (item 8). A *clone* is neither: `git clone --no-checkout <repo> previous` and a detached checkout of the commit, under the run's own directory, is a repository with the history, builds nothing uncommitted, and registers nothing here. Not driven — whether all eleven then pass is one run's answer.
- **Smallest change:** the exclusions a run may have are rows — in the same list, with a reason — and an exclusion that is on no row is void.

```yaml
claim: "a test that fails on its own binary and on the candidate's is excluded, and the verdict is green"
verdict: CONFIRMED
setup:
  - evidence/run.json: {previous: {commit: <91834b5e…>, sha256: "a"×64}, candidate: {commit: <d423f09b…>, sha256: "b"×64}, baked_path: "/x/target-previous/debug/jigc", named_by: 12, at_baked_path: {baseline_before: "a"×64, baseline_after: "a"×64, candidate_before: "b"×64, candidate_after: "b"×64}, exits: {}, seconds: {}, failed: null}
  - evidence/baseline-junit.xml: 200 testcases of suite jigc::g_flow, s::t007 and s::t008 with <failure/>
  - evidence/candidate-junit.xml: the same
repro:
  - ["dev/regression-set", "verdict", "--evidence", "<scratch>/evidence", "--list", "<scratch>/empty.tsv"]
expect: {exit: 0, stdout_json: {status: green, counts: {tests: 200, excluded: 2, differences: 0}}}
pinned-by: "dev_regression_set::a_baseline_is_compared_against_up_to_one_failure_in_a_hundred   # pins the tolerance as built"
```

### R-M8 — A check's verdict is the preflight's reading of a brief; the regression tool's proof is read by no script, and no brief for a check of thirty-five minutes exists

- **Where:** `stabilize.js:1177` (the checks are handed as `<item>: <brief>`), `:2351-2357` (the harness takes `status` and holds `commit` to the candidate's), `:776-782` (a green is recorded as `{item, outcome: green}`); `stabilize-preflight.md:30` (*"The scripted regression set, once your prompt names one"* — the whole of what the definition says of it).
- **What decides:** the doc gives the mapping — *exit 0 is green, 1 is red, 10 to 16 are void, a refusal is void too* — and the preflight applies it. The tool's one line carries the two commits it was run with, the list's hash, the four hashes that prove which binary each run drove, and the excluded tests; for a green none of it reaches the harness or a table. *Which binary ran is proven, not assumed* holds inside the tool and stops at the agent.
- **How the preflight would call it:** from the item's brief, which the opening writes. No run has the item; the doc describes what the brief must say (in the background, its line in a file, waited for in slices, after the gate) and gives none. The definition's one pattern for a long command is the gate's. The prompt's one bound is for CI (*at most 60 minutes*).
- **Does the contract allow it:** yes — nothing bounds a check's time — and nothing in it tells the agent how. The first preflight's turn is then two release builds, the candidate's gate (twelve to thirteen minutes alone, sixteen to twenty-one on a shared machine, as `DECISIONS.md` → *The gate, repaired* measured it), up to sixty minutes of CI and this check, one after another, in one agent.
- **The time the doc gives is the rehearsal's.** *"It measures about twenty-five minutes on a machine other agents use."* The record's run, on a machine other agents were using, took 2,045 s — thirty-four minutes; 1,501 s was the rehearsal.
- **One sentence that holds only half the time:** *"the two commits and the scratch root are in the preflight's prompt already."* The previous release's commit is in the prompt of a call that builds (`:1170-1173`); a check the round's scope selects goes to the second preflight (`:2291`, `build: false`), whose prompt does not carry it.
- **Smallest change:** `dev/stabilize-step` gains the act — it runs the tool with the run's own facts and relays its line — and the harness holds `previous.commit`, `candidate.commit` and `status` itself; or, short of that, the item's brief is written and committed before the canary, and the canary runs it.

## LOW

- **R-L1 — `stopAfter` over a pending batch runs a gate, commits and pushes.** `stabilize.js:2206-2208` finishes a pending record before it reads `stopAfter` (`:2246`). Driven: a record step whose gate is newly red leaves its batch applied; the next invocation, `stopAfter: 'state'`, returns `recorded` after `record:pending`, `git:record:pending`, `git:push:pending` — the local head and origin both moved. The doc and the header: *"No record step runs and nothing is committed."*
- **R-L2 — The candidate's gate runs on a tree that holds the run's own pending files.** The attempt's marker is written before the preflight (`:2247`, then `:2263`), and the scope step writes the round's scope and its report beside the preflight while the gate runs (`:2265`). What such a file turns red is on record as the candidate's red, and is then accepted of every record commit of the round. The record script screens its writes for the one fence that reads every markdown file; a second such fence owes it a row (the doc's own declared bound). Traced, not driven.
- **R-L3 — The previous release's binary is known by a version string every candidate prints, at a path taken from the return.** `:2110-2111` hold it to `--version` — and *every local candidate prints the previous release's version*; its path is not compared with the one the prompt named (`:1172`), though the candidate's is (`:2106`); its hash is compared with nothing — not even with the candidate's, which the regression tool does refuse — and is on no record. A regression fact measured against the wrong binary routes a regression to the human instead of a fixer: the safe side.
- **R-L4 — A door of the round that no item reaches is known from an agent's return and from nothing else.** `uncovered` and `reached_but_excluded` are the scope step's (`:2303-2304`, `:2389`); the doc says both are *in this return and in the scope step's report* and in no table. The first is computable from what the state already holds — the included doors against each item's doors and registries, which the harness computes per item at `:1205-1208`. A second attempt, whose scope *stands*, need not return either.
- **R-L5 — A step that is one command, never run twice, is wrapped in a retry that tells its agent to look around and go on.** `STEP_RULES` (`:1086`): *"nothing else: no command before it or after it … never run the command a second time"*; the retry note (`:1794`) appended to the same prompt: *"look at what that attempt left — `git status --porcelain`, `git log -1` … and go on from it."* Two steps are not idempotent: `begin` (a second run is refused, the stage halts, the attempt is counted — declared) and `record` (a second run after a commit is `no-batch`, the stage halts with the commit unpushed — `R-M1`).
- **R-L6 — The self-test passes a harness that runs the cross-model pass on every review row.** The first review's mutant, re-driven: `chain: chainOf(i.kind, true)` → `self-test-passed`, 373 checks. What catches it now is the simulation — the same mutant under the stand-in is fatal, *"no agent is scripted for the label `row-a:crossmodel`"*. Both need `node`.
- **R-L7 — `dev/gate --quick` prints `GATE: PASS`.** Driven. The suite pins it (`dev_gate_report.rs:1022`), and both scripts that read a gate's output demand the totals line beside it. An agent told to wait *until that file holds the gate's verdict line* (`:1174`, `:1327`) reads a gate's word on a pre-check; the commit step then refuses the file.
- **R-L8 — Nothing in an invocation names the repository.** Every step runs *from the repository's root* of whatever session launched the workflow. A canary is aimed at its clone by the session's working directory and by nothing else; a canary run whose slug is a branch of the real repository, launched from the wrong directory, is a real run.
- **R-L9 — Three definitions a run launches still describe the gate as running a `probe` step** (`milestone-code-reviewer`, `milestone-e2e-tester`, `robust-advocate`). The first review's `L8`, unchanged and declared.
- **R-L10 — The doc names three of the six rulings about the run** where it says which invocation of `fix` is not refused: *"(`go`, `rerun`, `rounds`)"* (doc line 13); the harness takes `go`, `rerun`, `rounds`, `cycles`, `reverify`, `again` (`:459`).

## The first review's findings, each with its disposition

*Closed* means: the defect is gone, and something committed goes red if the repair is reverted. *Read* means the reverting test was read and run green, not reverted; *driven* means a mutant or the scenario was run for this report.

| | Finding | Disposition | What closes it · what holds it |
|---|---|---|---|
| H1 | landing does not check that the audit covers the tip | **not closed — the `fix` half's** | unreachable: `stage: 'fix'` is refused before any agent (driven: zero agents, with no arguments, with `exit: 'drop'`, with a ruling on a finding, with `stopAfter`, with `args` as a JSON string) |
| H2 | a part lands with its audit finding unverified | **not closed — `fix`** | as H1 |
| H3 | a part exit that halts after its cut loses the left-out set | **not closed — `fix`** | ruled to be rebuilt as a revert |
| H4 | a finishing triage in a running `fix` verifies on a stale commit | **not closed — `fix`** | the doc says that lap halts |
| H5 | `landed-before` reports landed with no checks and no push | **not closed — `fix`**; its class has a member in `test` | `R-M1` |
| M1 | outcome written before the act | **not closed — `fix`** | |
| M2 | `recordFault` accepts absent or empty checks | **closed** | `:1886-1895`; held by the self-test's checks of `recordFault` (run by fence arm (i)). The simulation never reaches those lines — the step tool refuses first (`no-batch`) |
| M3 | `contested` has no committed form | **closed for `test`** | the fork is a cell of the round's triage record; simulation (read). A fixer's fork: `fix` |
| M4 | the record step is not atomic | **closed** | one batch, pending, `finishRecord`; simulation (read; re-driven for `R-L1`) — with `R-L1` as its side effect |
| M5 | a cross-model failure can halt the stage | **closed** | driven with the tool absent: the pass is void for the pass, the item green, `cross-model-void: [row-a]` on record; simulation, five endings (read) |
| M6 | the preflight contract contradicts the finishing triage | **closed for `test`** | the definition takes a commit `HEAD` holds; simulation (read) |
| M7 | reviewers get no binary | **closed — and worse in one respect** | `ROLES.review` binds on the line and `drives`; fence arm (d). The reviewer that drives nothing now halts the stage if it returns no hash: `R-H2` |
| M8 | path-class assert limits | **not closed** | declared; the repair's last task |
| M9 | a dead verifier, advocate or driver wastes the stage | **closed for one dead agent; not for two** | driven both ways: `R-M2` |
| M10 | fence holes; no test executes `runTest` or `runFix` | **closed for `runTest`** | the simulation executes it (coverage below); arm (g)'s mutant is fatal there (driven). Arms (e), (f), (k) not re-probed. `runFix`: one line of 263 is executed |
| M11 | a stale timing row can redden the gate for good | **closed** | the probe before any tier, `dev/gate:711-733`; `dev_gate_report` (read, 42 green) |
| M12 | a fixer's top-level halt discards its entries | **not closed — `fix`** | |
| L1 | fields collected and never read | **closed for `test`** | `path_check`, a check's `commit`, the verifier's `key` are read; the fixer's `gate` is `fix`'s |
| L2 | `halt()` always says *the same args* | **closed in part** | a record step's halt has its own sentence; a push halt after a record still says it, and it is wrong there (`R-M1`) |
| L3 | the header's claim about `stopAfter` | **closed** | the header says what stays on disk; `R-L1` is what it still gets wrong |
| L4 | the binary-count comment | **closed** | `dev/gate:141-153` |
| L5 | `raise` is not a recorded fact | **closed in part** | `cycles` is a ruling about the run; the `fix` stage's own `raise` stands beside it (declared) |
| L6 | nothing stops two concurrent invocations | **closed for `test`, by reading** | `begin` refuses an attempt the state no longer hands out; not driven |
| L7 | no *Never* paragraph in the two prompt-only roles | **closed** | `NEVER`, fence arm (o) |
| L8 | definitions still name a `probe` step | **not closed** | `R-L9`, declared |
| L9 | duplicate triage keys make `launch.add` throw | **closed** | `:1997-1998`; simulation (read) |
| L10 | the doc's 4644 | **closed** | `dev-workflow.md` dates it to its tree |
| L11 | the previous binary's hash compared only when `regression` is true | **closed** | `:2026`, with every `confirmed` — which is also `R-H2`'s third case |
| L12 | a run slug ending `-rN` | **not closed** | declared. The first run's slug does not end so |

**In numbers:** 29 findings — **16 closed** (four of them for the `test` stage only; one, `M7`, with a new defect), **3 closed in part** (`M9`, `L2`, `L5`), **10 not closed** — seven the `fix` half's and unreachable behind its refusal, three declared (`M8`, `L8`, `L12`). **One is worse:** `M7`. Of the four leads: the filter's size is answered (a refusal of the tiers, for whatever reason, is "no record"); agent-written text spliced into other agents' prompts, a new finding keyed as found again, and the door text triage returns against the reporter's are leads still.

## The reporter × ending table

*What the stage does for each reporter and each way it can end, as the code has it. **S** — a simulation scenario scripts the cell; **D** — driven for this report; **—** — traced only. A cell in bold is one neither the harness header's table nor the doc's states.*

| Reporter | no result (dies) | halts | result, no report | wrote and died | a return that is wrong |
|---|---|---|---|---|---|
| the attempt's marker (`begin`) | halt, transient; counted if the command ran — | refusal → halt — | — | — | a relayed line that fails its hash → halt — |
| first preflight | halt **S** | halt — | halt, **after the instruments** **S D** | halt — | wrong candidate, path, previous version → halt — |
| scope step | halt **S** | halt — | halt, **after the instruments** — | halt — | `written` and no doors in the state → halt — |
| second preflight | its arms and checks void, goes on **S** | the same — | **halt, after its arms ran** **D** | void, goes on — | image not verified → void — |
| a chain's step | item void **S** | item void **S**; a finding it returned with its report is triaged — | item void, its findings dropped **S** | item void — | **no hash → halt of the stage** **D**; another hash → halt **S** |
| cross-model pass | pass void **S** | pass void **S** | pass void **S** | pass void **S** | tool absent → not launched, pass void **S D** |
| triage | halt **S** | halt **S** | halt **S** | halt — | counts that do not balance → halt **D**; two entries, one key → halt **S** |
| verifier | unverified **S D** | unverified **S** | unverified **S** | unverified **S** | another key → unverified **S**; a hash → halt **S**; **`confirmed` with no `regression` → halt** **D** |
| advocate · independent drive | no fork, unverified **S** | the same — | the same **S** | the same **S** | a hash → halt — |
| the record's executor | halt, the batch may be pending — | halt — | n/a | n/a | says *gated*, applied nothing → `no-batch`, halt **S** |
| **any two of them, by different labels** | **the breaker: every later call returns nothing, the stage halts at its next step** **D** | | | | |

- **Every cell is decided — the code does one thing in each.** Four are decided in the code and stated in neither table: a result with no report from the second preflight (halt); an absent hash; a confirmed verdict without its regression fact; and the second death of an invocation. Two are decided late: the first preflight's and the scope step's missing report (`R-M3`).
- **An agent that returns twice** has no cell: the runtime hands one return. Its two shadows are a *retry* that finds the dead try's report (the record script refuses the second with `exists`, which the definitions make a halt: the item is void, and the dead try's report is committed and counts for nothing) and a *reporter name asked for twice* (`launch.add` throws; `unitNames` and the duplicate-key check stand before the two places it could happen).
- **The cross-model tool, absent.** With no item named: no step asks for it, no prompt names it (the tool's name is one line of the harness, `:1289`, reached only through the two functions the opt-in guards), and no definition does (`command grep -il` over `.claude/agents/`: none). With an item named and the tool absent: the preflight answers *void*, the pass is not launched, the item's own passes run, and the round's record says `cross-model-void`. Driven both ways. The workflow runs for someone who does not have it.

## The simulation — what it executes, and what only a real invocation shows

**What it executes.** The committed script, unmodified, as the body of an async function in a `vm` context; 78 invocations over 19 tests. Block coverage, merged over all of them:

| Part of the harness | code lines | fully run | partly | never |
|---|---|---|---|---|
| `runTest` (`:2204-2393`) | 153 | 121 | 32 | 0 |
| agent plumbing, the record step, triage, forks, `finishTriage`, `ruleTheRun` (`:1777-2202`) | 328 | 239 | 75 | 14 |
| `runFix` (`:2396-2692`) | 263 | 0 | 1 | 262 |

**What of the `test` half it never takes** — each a branch, read off the coverage:

- **a relay that fails:** `readStep`'s `lost` (`:753-764`) and `readState`'s *again* loop (`:1858-1868`) — the stand-in relays every line byte for byte;
- **a call after the breaker tripped** (`:1789`), and an agent call that throws (`:1802`);
- **a second triage pass:** `leftOpenSource` is never called; `:2001`, `:2008`, `:2055-2056`, `:2086`, `:2094`, `:2337` (`R-M4`);
- **a push that fails after a commit:** `:1918`, `:2137`, `:2191`, `:2368` (`R-M1`); and a state that cannot be read back after one (`:1920`, `:2139`, `:2193`, `:2370`);
- **a report nobody launched:** `:638`, and with it `:2323` and `:2040` — the halt ruling 12 rests on is executed by no stage of the simulation (the check itself is the record suite's);
- **a record whose executor halts** (`:1903-1904`), and every harness-side line of `recordFault` behind the tool's own refusal (`:1887-1893`);
- **a preflight that returns the wrong thing:** `:2106`, `:2110`, `:2111`, `:2113`;
- **a triage whose counts do not balance** (`:1992`) or whose key is no slug (`:1994`); a verifier without its regression fact (`:2031`); an advocate's or an independent drive's hash (`:2085`, `:2093`);
- **a second round:** `:2264` — no test stage ever has a round before it;
- **an unknown kind, a name that cannot be made, a cross-model item that is no item, `stopAfter` at `instruments` and at `triage`:** `:2282`, `:2313`, `:2243`, `:2332`, `:2344`.

**What it cannot show, by construction** — the stand-in's own header says most of it: `parallel` is serial; a return is held to its schema by a validator of the simulation's own; an agent dies only where scripted, on every try, never half-way; nothing is resumed; the gate, gitleaks, the builds and the trial image are stand-ins. Three things it does not say: a scripted reporter **always returns the hash** (so `R-H2` cannot be scripted); every relayed line is **small** (`R-H1`); and **a label that is to die must also be scripted**, or the run is fatal for a reason of the stand-in's.

**What the canary must show that nothing else can.** The builders' list — how an agent really ends and what the runtime hands back; `begin` with a real agent and its retry; the preflight building a commit that is not `HEAD`; a cross-model pass with its real tool; a reviewer asserting the binary's hash; the breaker tripping where the stage could still record; an orchestrator bringing a fork from the human's list — stands. **Missing from it:**

1. **A state document of the real run's size, relayed** — the run opened with the ledger and the scope the first run will have, not one planted finding (`R-H1`); and the record step's payload at that size, written by here-document and held to its hash.
2. **A reviewer on a pass that drives nothing** — the source pass of a review row, the audit of an area — and whether its return carries the hash (`R-H2`); and a verifier's `confirmed` on a door the previous release does not have.
3. **A `check` item run from its brief by a real preflight** — the regression set above all: the brief does not exist, the run is thirty-five minutes, and the mapping of its exit status is the agent's (`R-M8`); and the whole of the first preflight's turn in one agent.
4. **A verifier that leaves something open**, so that a second triage pass runs — a path no test has executed (`R-M4`).
5. **A push that fails after the record's commit**, and what the orchestrator then does (`R-M1`).
6. **What the gate sees of the run's own files** — the marker and the scope in the tree while the candidate's gate runs (`R-L2`).
7. **A report the record script refuses for its text** — a host path, a hygiene hit, the fenced line — with the real scanners, and whether an agent repairs the text and converges.
8. **The size of what a stage returns to the main session** (`R-H1`).
9. **Whether a subagent's first `dev/stabilize-step`, `dev/stabilize-record` and `shasum` each run without a permission prompt** under the runtime's mode — `.claude/settings.json` holds a deny list and no allow list.
10. **Where the canary runs** — that the session's working directory is the clone (`R-L8`).

**What a clone with a bare remote cannot show at all:** the check that reads CI for the candidate's commit. Its first run is the real run's.

## The regression tool — the four questions

| Can it report green when… | Answer | Evidence |
|---|---|---|
| …the old binary ran | **No, as far as a hash at the path can say.** The file at the path the tests compiled in is hashed before and after each run; anything but the candidate's around the second run is `void`, `swap`. A test that reaches the binary by another path would not be seen; twelve of fifteen test binaries hold the path, and none was found that reaches it otherwise. | `:424-434`, `:706-718`; suite: `a_binary_that_was_put_back_is_void_and_never_green`, `no_run_goes_through_cargos_build` (read, green) |
| …the two binaries are the same | **Byte for byte: no** (`void`, `swap`). **Built from one commit: nothing refuses it** — the two commits are never compared, and two builds of one commit need not be the same bytes. | `:396-401`, `:721-724`; `R-M6`, driven with `check-list` |
| …the list is not the committed one | **Yes.** The first green on record was. | `R-M6` |
| …tests were silently dropped | **Not between the two runs** — a second run that did not run exactly the baseline's tests is `void`, `incomplete`. **Yes, at the baseline** — up to one in a hundred, named in the line and changing no verdict. | `:471-475`; `R-M7`, driven |

- **Is the exclusion of the eleven named loudly enough?** In the record: yes — each test, what it asks git for, and that one reaches the binary. In the tool's line: by name, under `excluded`, with a count. In the workflow doc: one bullet without the number and without the loss. In a round's record: not at all — a green is one word (`R-M8`).
- **A cheap way not to lose them:** a clone in place of the archive (`R-M7`).
- **How the preflight would call it, and whether its contract allows thirty-five minutes:** `R-M8`.
- **Not driven:** the real run; the stand-in for cargo in the tool's suite was read, not re-derived.

## `dev/gate` — the three questions

Driven under a stand-in `cargo` (nextest absent, so the suite is `cargo test`), `JIGC_GATE_HYGIENE=off`:

| Run | exit | mode line | verdict line | totals line | steps |
|---|---|---|---|---|---|
| `--quick`, green | 0 | `quick` | `GATE: PASS` | none | fmt, clippy, build |
| `--fast`, green | 0 | `fast` | `PRE-CHECK: PASS …` | none | fmt, clippy, build |
| plain, green | 0 | `full` | `GATE: PASS` | one | + test |
| `--keep-going`, green | 0 | `full, keep-going` | `GATE: PASS` | one | + test-build, test |
| plain, red fmt | 1 | `full` | `GATE: FAIL (step: fmt)` | one | no test |
| `--keep-going`, red fmt | 1 | `full, keep-going` | `GATE: FAIL (step: fmt)` | one | test-build and test ran |
| `--keep-going`, red clippy, one red test | 1 | `full, keep-going` | `GATE: FAIL (step: clippy test)` | one | all ran |
| `--keep-going`, red build | 1 | `full, keep-going` | `GATE: FAIL (step: build)` | one | **no test** |
| `--keep-going`, tests do not build | 1 | `full, keep-going` | `GATE: FAIL (step: test-build)` | one | **no test** |
| `--keep-going --fast` · `--quick --keep-going` | 2 | — | — | — | refused |

- **Can `--keep-going` report green over a red step?** No: every step's status is captured bare and any that is not 0 is in the verdict and the exit status (`dev/gate:575-601`, `:919-946`). The nextest path (`run_whole_suite`, `:773-786`) was read, not driven.
- **Can a pre-check be mistaken for a gate by what reads its output?** By a script, no: `dev/stabilize-record` takes a file that holds one verdict line *and* one totals line, and a red one only with the keep-going mode line (`:3308-3334`); `--fast` prints neither line, `--quick` the verdict alone. By an agent told to wait for *the gate's verdict line*: `--quick`'s is a gate's (`R-L7`).
- **Is a red build still a stop?** Yes, in every mode, and so is a red build of the tests under `--keep-going`.
- **A lead, the record script's:** the comparison of a record's gate with the candidate's holds names, never counts. Driven: the candidate's gate on record with 4,778 passed and two red tests; a record's gate with 1,199 passed and one of those two red → `ok: true`, `new: []`. A gate cut short reads as nothing newly red. What could cut a `--keep-going` run short and still let it print its summary was not found.

## The workflow doc — `implementation/stabilization-workflow.md`, read whole

**In one paragraph.** It is an unusually candid doc: it says what is built and not fit, what was read off the source and never observed, and it keeps a long list for the canary. Where it states what the `test` stage does, the code does it — with the exceptions below. Its list of what is still defective is **honest about the `fix` half and incomplete about `test`**: it closes with *"Nothing on it is the `test` stage's own"*, and one member of its own last class is. No contradiction was found with `dev-workflow.md` → Gate (the record commit's rule is stated once there and pointed at), with `release.md` → *What agents may not do* (the `test` stage pushes one branch, by name, and the step tool takes no name without a prefix), with `CLAUDE.md` → Branches (the three names the harness mints pass `dev/branch-name`: exit 0 for the loop, a round and a part; `stabilize/…` exits 1, as the doc says), or with the rulings as the doc cites them.

### Sentences that are false, or that lack what the code does

| Doc line | The sentence | What the code does | Evidence |
|---|---|---|---|
| 417 | *"What is left on this list is the `fix` half's … Nothing on it is the `test` stage's own, and nothing on it is the gate's."* | A `test` stage that commits its record and cannot push halts with the advice the list calls wrong for `fix`; nothing pushes the commit, and the state reads as recorded. | `R-M1`, driven |
| 314 | *"The harness's message says to send the same invocation again once the cause is dealt with. That holds for a halt before the stage changed anything, and for a halt of a record step."* | It is also what the message says after a record's commit whose push failed — where the same invocation is refused and does nothing. The next sentence sends *a halt in the middle of a git act* to the human; this halt is named nowhere. | `stabilize.js:2368`, `:1835`; `R-M1` |
| 100 | *"The loop branch is checked out, clean, and **pushed**"* | Checked out and clean are asserted. Pushed is not: the step refuses a remote that is ahead, and lets a local branch that is ahead pass. | `dev/stabilize-step:317-325`; `R-M1`, second invocation |
| 131 | *"`stopAfter` — return after a named step, for tuning. No record step runs and nothing is committed"* | Over a pending batch the invocation runs the full gate, commits and pushes, and returns `recorded`. | `R-L1`, driven |
| 113–124 | the table *How an agent can end*, and *"A hash that is not the binary's is none of these: it halts the stage."* | Four more endings halt the stage and are in no row: a hash that is **not returned**; a `confirmed` verdict without its regression fact, or without the previous binary's hash; a second preflight that returned and left no report; and the second agent of an invocation to die, whatever its row says. | `R-H2`, `R-M3`, `R-M2`, each driven |
| 108 | *"The reports, checked — after the instruments, after every triage pass, and once more inside the record's batch."* | True — and the first of those checks is where the first preflight's and the scope step's reports are first looked for. *"Halts, naming it"* in the table is a halt after every instrument ran. | `R-M3`, driven |
| 133 | *"**What comes back.** `status: 'triaged'` with the round, the candidate and its binary's hash, the counts, `human_list`, `forks`, `unverified`, what forbids closing, and `next`."* | And, for every `next` but `fix`, `rule` and `close`, the whole state document — 38 KB over a ledger of sixty rows. | `R-H1`, measured |
| 106 | *"runs **the candidate's own full gate, once**"* | On the working tree, which by then holds the attempt's marker and may hold the round's scope and its report. | `R-L2` |
| 172, 177 | *"Triage grades every finding … every entry an agent *left open*, included"* · *"triage and verification alternate — as many times as the harness's `VERIFY_PASSES` says"* | The code does; no test has run a second pass, and one keying of what was left open halts the stage at its record. | `R-M4`, driven |
| 343 | *"an agent of every kind that dies, halts, writes and dies, or returns with no report — … the scope step and both preflights"* | Every kind in at least one ending, not every ending of every kind: the scope step and the second preflight only die. | `stabilize_simulation.rs:2552`, `:2596`, `:2710` |
| 353 | *"What it does not establish"* | Lacks three things that are no property of a stand-in and could be scripted: no reporter ever leaves anything open; no relay ever fails; no step is asked for after the breaker tripped. | coverage, above |
| 375 | *"The list of intended changes is a committed file that the tool reads and no agent does."* | The tool reads the file it is handed; the first green on record read one that was in no commit. `DECISIONS.md` says so (item 13: *"Nothing holds the list to a commit"*); the doc does not. | `R-M6`, driven |
| 382 | *"An archive is no repository. The previous release's tests that ask git about their own source tree fail on their own binary and are excluded, named."* | Eleven, of which one reaches the binary; and the tolerance that admits them admits forty-three. | `R-M7` |
| 386 | *"It measures about twenty-five minutes on a machine other agents use"* | The record's run, on such a machine: 2,045 s. Twenty-five minutes was the rehearsal. | the record's *The numbers* |
| 386 | *"the two commits and the scratch root are in the preflight's prompt already"* | In the first preflight's. A check the scope selects goes to the second, whose prompt names no previous release. | `stabilize.js:1170-1173`, `:2291` |
| 13 | *"the one that only records what the human ruled about the run (`go`, `rerun`, `rounds`)"* | Six: `go`, `rerun`, `rounds`, `cycles`, `reverify`, `again`. | `stabilize.js:459` |

### What an orchestrator who has read only this doc would get wrong

1. **After a halt at `push`** they would send the same invocation again, as the harness tells them, read `refused`, and take the round's record for pushed.
2. **They would open the canary with one planted finding** — the doc's own description of it — and learn nothing about a relay at the first run's size.
3. **They would write the regression set's item with a brief of their own**, with nothing to copy it from, and learn in the real run whether a preflight can hold it.
4. **They would use `stopAfter: 'state'` to look at a run** that has a batch pending, and commit and push it.
5. **They would expect a second dead verifier to be one more unverified finding**, and get a halted stage and a counted attempt.
6. **They would read the stage's return whole** — it is the thing the doc tells them to read `next` from — and take the state document into the main session with it.

What they would get *right* from the doc alone, checked against the code: that `fix` refuses and what it says; that a ruling on a finding goes to `test` with `rulings` and nothing else; that a run's go costs a record step; that `recorded` means a pending batch was committed and nothing else done; that a cross-model pass needs naming, item by item, and its tool is needed by nobody otherwise; that a refusal carries `next`.

## Leads — noticed, not pursued

- **A report written for an attempt nobody began counts as an attempt and is committed with the next record.** Driven once: a report under a reporter of its own, attempt 1, written through the record script before any stage → the stage ran as attempt 2 and reached its record, with that file among the reports it committed. Ruling 12's *a file nobody launched stops the stage* holds within an attempt and not across them (the doc declares the second half: a report of an attempt that reached no record is held against nothing). The record script's.
- **The gate comparison holds names, never counts** (`dev/gate`, above). The record script's.
- **A commit made on the loop branch while a stage runs** is under the stage's record commit, and the round's `candidate` fact names the commit the stage began on; the `record` act holds the branch, not the head. Traced.
- **A human's scope on a second attempt is not applied**: the round's scope stands, the scope step answers `stands`, and the return says so in one word. The doc says a scope is written once.
- **Agent-written text in other agents' prompts** — a finding's title and door into triage's, a verdict's basis into the advocate's, a proposal into its independent driver's. None reaches a shell: every record call is an argument list or a document on stdin, and the names that do reach one are slugs and a validated path.
- **`resumeFromRunId` after a kill** replays every step whose prompt is the same — a `git-state` and a `state` line from before the kill among them. Not examined.

## Checked and correct

**The refusal of `fix`**
- Asked before any agent: zero agents launched in five driven shapes. An invocation that carries only rulings about the run passes it and reaches the two reads.
- With `NOT_FIT` emptied the self-test still passes (369 checks): lifting the refusal is the one edit the header says it is. Fence arm (m) ties the edit to a `DECISIONS.md` heading (read).

**The git acts of `test`**
- One branch is pushed, by name, by one act; no act takes a name without a prefix. The settings' deny list binds what an agent types, not what the step tool runs — what holds the tool is its own grammar and the suite that reads every git call of it.
- The record's commit adds the run's pending paths by name and nothing else (`dev/stabilize-step:631-651`).

**The cross-model pass**
- Named by one line of the harness; absent from every definition; void for the pass and nothing else when its tool does not answer (driven).

**The record step**
- A gate newly red leaves the batch applied; the next invocation gates, commits and pushes it and does nothing else (driven as the setup of `R-L1`).
- A file with one of a gate's two lines is refused by both readers.

**The definitions against the schemas**
- Preflight, scope, verifier, advocate, executor and `build-git`: every field the harness reads is named in the definition's return, or by the schema where the definition is silent (`status`, a finding's `id` and `title`). `version_string` is gone from both sides.
- The contracts bind on labelled lines the build harness does not send; fence arms (d) and (e) hold them (run green).

**`dev/gate`**
- `--keep-going` is green only when every step is; a red build stops it; `--keep-going` beside a pre-check is refused (driven).

**The regression tool**
- A swap that did not hold, a second run that is not the baseline's tests, and byte-identical binaries are each void (read; the suite's 24 run green).

## Not examined

- `dev/stabilize-record` beyond `gate_read`, `gate_set` and `gate_check`; its suite; `dev/stabilize-step`'s suite. The other reviewer's.
- The `fix` stage's body (`runFix`), beyond its entry and the refusal before it.
- The regression tool's real run, and the stand-in for cargo in its suite.
- `dev/gate`'s nextest path, its timing record and its probe, beyond reading; no gate was run.
- Fence arms (e), (f) and (k) against the first review's mutants.
- The sections of the doc on *Close* past its first three steps, and the `decisions-pending.md` rows it points at.
- What the Workflow runtime does with any of it: no agent was launched.
- Block coverage was merged by overwriting each function's ranges in the order V8 lists them; a line reported as *never run* was spot-checked against the scenarios, not proven.

## What only a real invocation can verify

*The canary list, in order of what a failure would cost the real run. Items 1–5 are this report's; the rest are the builders' and the first review's, kept.*

1. **The state relay and the record's payload at the first run's size** (`R-H1`).
2. **A reviewer that drives nothing, and the hash** (`R-H2`).
3. **The first preflight's whole turn** — two builds, the gate, a check that reads CI, the regression set from a brief (`R-M8`); the CI check only against the real remote.
4. **A verifier that leaves something open** (`R-M4`), and **two agents that die** on a road where the stage could record (`R-M2`).
5. **A push that fails after the record** (`R-M1`); **a report refused for its text**; **the permission mode**; **the directory the canary runs in**.
6. That the runtime accepts the script; that a schema's `required` is enforced, and what the script is handed when a return lacks one.
7. `begin` with a real agent, and its retry; the preflight on a commit that is not `HEAD`.
8. A cross-model pass with its real tool.
9. How an agent really ends; whether one the runtime gave up on writes its report afterwards.
10. An orchestrator bringing a fork from the human's list, with the return lost.
11. The cap on concurrent agents; a trial arm past the ten-minute ceiling; the preflight's status assert beside the scope step's writes.
12. A kill at each boundary, and the replay of a resumed run.
