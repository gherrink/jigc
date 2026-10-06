# Build review B — the harness, the agent definitions, the gate tooling

*Independent, read-only review (`milestone-code-reviewer`) of `357ab425..f6918aaa`, returned 2026-10-06. The reviewer does not write files outside a stabilization run, so this file is the orchestrator's transcription of its return, unedited in substance. Line numbers are at `f6918aaa`.*

**Verdict: red.** Five paths let a round land, or a fix count as landed, without the audit the rulings require; three were driven under `node` against the committed script with stub agents, the others are traced in source.

**Scope:** `.claude/workflows/stabilize.js`, `crates/cli/tests/stabilize_harness_fence.rs`, `merge_logs_fence.rs` arm (k), `.claude/agents/*`, `dev/gate`, `dev_gate_report.rs`, `.config/nextest.toml`, and the three doc files. All read from committed state.

**How evidence was obtained**
- **Driven:** the committed script run under `node` with stub agents, in `<scratch>/drive.*`, `drive2.*`, `drive3.*`. The state documents were the reviewer's, shaped as `dev/stabilize-record`'s `route_of` and `position_of` compute them.
- **nextest probe:** a scratch crate in `<scratch>/nx.EXlDS9`.
- **Gate logs:** read-only from `$TMPDIR/jigc-gate-*`; machine-local, not committed.
- **Everything else:** traced in source.

## HIGH

### H1 — Landing does not check that the audit covers the tip
- **Where:** `stabilize.js:2309-2313` (land on `at.land`), `:2182`, `:2192`, `:2221`; `dev/stabilize-record:1957`.
- **Evidence:** `land` is `cycles >= 1 and not still`. The script then merges with no step comparing the last audited commit to the round's tip.
- **Scenario A:**
  1. Cycle 1 is audited, so `cycles` is 1.
  2. In cycle 2 a fixer commits a fix for f-2 and another fixer forks on f-3.
  3. `audited` is false; the record writes f-2 `fixed` and no cycle.
  4. The human rules f-3 `later`. Nothing is open and `cycles` is 1, so the round lands with f-2's commit read by no audit.
- **Scenario B:** a fixer commits, then the stage halts before its record; a later `rulings` invocation closes the remaining rows. The commit lands unaudited and unknown to the ledger.
- **Class:** fix commits on the round branch after the last counted audit.
- **Bound:** every return between the fixer loop and the cycle's record commit was enumerated — 13 sites (`:2340`, `:2343`, `:2348`, `:2361`, `:2177`, `:2188`, `:2195`, `:2205`, `:2206`, `:2212`, `:2215`, `:2216`, `:2223`) plus the fork branch at `:2192`.
- **Smallest change:** record the audited tip as a round fact; before landing, a git step asserts `git diff --quiet <audited> <branch> -- <PRODUCT_PATHS>`, else run an audit-only cycle.

### H2 — A part lands with its own audit finding unverified
- **Where:** `stabilize.js:2280-2286`.
- **Evidence:** the landing condition is `fresh.length || state.human_list.length || forks.length`. `fresh` is taken from `state.blockers`, which is route `fix` only (`dev/stabilize-record:2104`). An unverified `breaks` is route `triage` (`:1705-1708`) and is in none of the three.
- **Driven:** the part's audit returned one finding, triage graded it `breaks`, the verifier halted. Result: `status: landed`, `next: triage`, `untriaged: [f-9]`.
- **Class:** landing decisions the script takes itself rather than from `position.fix.land`. One site; the normal path is protected because `land` requires nothing open.
- **Smallest change:** add `state.untriaged.length` to that condition.

### H3 — A part exit that halts after its cut loses the left-out set
- **Where:** `stabilize.js:2259-2278`, `:597-608`, `:627-631`.
- **Evidence:** `left`, `reopen` and `repoint` exist only in memory. They are recorded by `secondHalf`'s record, after the cut is made and pushed. Ten halt sites lie between.
- **Scenario:**
  1. The round has fixes A and B; the human passes `exit: { part: [A] }`.
  2. `part1` is cut, then the audit halts.
  3. Re-invoking with the same args: `currentCut` now picks `part1`, so `:2259` halts and prints "its fix commits are: A′".
  4. Re-invoking with `{ part: [A′] }`: `left` is empty, and `repointPatches` matches nothing because the ledger still names A.
  5. f-B stays `fixed` by a commit that never lands. If the part's audit is clean, it lands.
- **Also:** `exit: 'drop'` on `part1` reopens nothing, for the same reason.
- **Class:** values computed before a git act and recorded after it. This one; fixers' `patches` (H1 scenario B, M12); `tipSha` (H4).
- **Smallest change:** write the reopen and repoint patches and a `cut` fact in a record before or atomically with the cut; have `currentCut` accept only a recorded cut.

### H4 — A finishing triage in a running `fix` invocation verifies on a stale commit
- **Where:** `stabilize.js:2098`, `:2105` (the only writers of `tipSha`), `:2302` (its only reader).
- **Evidence (driven):** a cycle ran with round tip `999…`. The verifier halted, so the position said `triage: true`. `finishTriage` then told the preflight: "commit 000… — the tip of `fix/x-r1`, which is checked out". The second verifier refuted and the round landed.
- **Outcome:** either the preflight notices `HEAD` differs and halts, or it builds what it was told and verdicts come from the pre-fix binary.
- **Class:** bounded by grep — three hits of `tipSha`.
- **Smallest change:** set `tipSha` from each push step's `head` in `secondHalf`.

### H5 — `landed-before` reports `landed` with no checks and no push (narrow)
- **Where:** `stabilize.js:1014` (step 4 stops the step), `:2238-2242`.
- **Evidence (driven):** call sequence `git:state → state → find → land → state`, result `status: landed, landed_before: true`. `remote_head` is not required on that branch.
- **Scenario:** a land halts after its merge commit (a step-9 mismatch, where the step says "undo nothing", or a rejected push). `halt()` at `:1624` says to invoke again with the same args. Doing so returns `landed`, with step 9 never re-run and the loop branch unpushed until the next `test` record.
- **Caveat:** a step-9 mismatch after a clean merge needs prior off-script state; the skipped push needs only a network failure.
- **Smallest change:** on `landed-before`, find the merge commit, run steps 9–11 on it, and require `remote_head === head`.

## MEDIUM

**M1 — Outcome written before the act it describes.**
- `stabilize.js:2148→2156` (drop record, then carry) and `:2284→2286` (part record, then land).
- After a halt between them, the round's branch says `outcome`, so `fix` is refused `round-over` (`dev/stabilize-record:1953`; js `:2086`, `:2115`). The loop branch still says open, so `test` is refused `round-open`.
- Only manual git recovers. Two sites, counted by grep of `outcome:`.

**M2 — `recordFault` accepts absent or empty `checks`.**
- `stabilize.js:1666`. Driven: `checks: []` and no `checks` both passed.
- In fix cycles, `finishTriage`, and the triage and verify reporters of `test`, the executor's `check-reports` is the only report check.
- Change: require one `ok` line per check command the script composed.

**M3 — `contested` has no committed form.**
- `grep contested dev/stabilize-record` returns nothing; `triageRecord` at `:1819-1829` drops it.
- `test` returns `next: fix` or `close` with the fork only in `forks`. The cycle loop lands at `:2309` without looking at `forks`, while the part path does look (`:2282`).

**M4 — The record step is not atomic.**
- It writes tables, then gates, then commits. A red gate or a halt leaves tracked files modified.
- Git state step 2 (`:935`, `:947`) then halts every later invocation, while `:1624` says to invoke again.
- Corollary: a candidate whose gate is red can never have its `test` stage recorded.

**M5 — A cross-model failure can halt the stage.**
- `:1710` drops a reporter with no result or no `report`.
- If its file exists, `check_reports` counts it `extra` (`dev/stabilize-record:2131-2137`) and the stage halts at `:2017`. Triage's definition also halts on an unlisted report.
- This contradicts the ruling that the source pass is never failed by it.
- Also: a pass that reported without a `report` field has its findings dropped while the record says it ran.

**M6 — Preflight contract contradicts the finishing triage.**
- `stabilize-preflight.md:16` asserts `git rev-parse HEAD` is the candidate; `:36` halts when the tree is not the candidate.
- `finishTriage` in `test` (`stabilize.js:1942`, `:1101`) always asks for a commit that is not HEAD, because the record commit moved it.

**M7 — Reviewers get no binary.**
- `ROLES.review` (`:206`) has no `binary` label, yet `milestone-code-reviewer.md` carries the binary paragraph and points at `dev/jigc-rig`, whose default is the tree's debug build.
- Bound: three definitions carry the paragraph (grep); only `review` is not sent the line.

**M8 — Path-class assert limits (`:950`).**
- The close's sync merge, "Merge remote-tracking branch…", fails the subject rule, so no stage can run after a sync.
- The prefix `fix/<run>-r` matches unrelated branches.
- `.cargo/config.toml` and `rust-toolchain.toml` shape the candidate's binary and lie outside `PRODUCT_PATHS`.
- It is a tripwire for direct commits; it does not show a merged round was audited.

**M9 — A dead verifier, advocate or proposal driver wastes the stage.**
- The script knows the result is null (`:1768`, `:1804`) but continues to a record step that must halt on the missing report.
- The next attempt re-runs every instrument.
- Change: drop the reporter and leave the finding unverified.

**M10 — Fence holes.**
- **Arm (g):** accepts any `chainOf(…, true)` (`fence:676-681`). Mutating `:2005` to `chainOf(i.kind, true)` turns cross-model on for every review row. The self-test still passes 285/285 (driven); the Rust arm passes by reading.
- **Arm (f):** misses the shorthand `{ key, disposition }`.
- **Arm (e):** misses an edit to a paragraph's bold lead.
- **Arm (k):** its go/grant scan is three literal strings.
- **No arm at all for:** hash comparison per driving call; `launch.add` before each `roleStep`; forbidden git commands outside `landPrompt` (one of ten prompts, plus sync by parity).
- **No committed test executes `runTest` or `runFix`.** Without `node`, the 285 checks are unprotected.

**M11 — A stale timing row can redden the gate permanently.**
- Probe: a `default-filter` naming `binary_id(=probe::gone)` makes nextest exit 96, "operator didn't match any binary IDs".
- Tier 1 then fails before running, writes no rows, and the record never heals.
- `dev-workflow.md` says a stale row "costs time, never coverage"; it can also cost a red gate until `test-seconds.tsv` is deleted.

**M12 — A fixer's top-level halt discards its entries (`:2340`).**
- Commits stay on the branch; the ledger never learns `fixed`; the same findings are re-sent.

## LOW

- **L1 — Fields collected and never read.** Fixer `gate`; preflight `checks[].commit` and `evidence`, `path_check`, `version_string`; verifier `key`. Verified by grep of each field against the script's uses.
- **L2 — `halt()` always says "same args".** Wrong after rulings are recorded, after a cut, and after a drop record.
- **L3 — Header claim is false.** "stopAfter … Nothing is recorded" does not hold for fix `rulings`, `fixers` or `audit`.
- **L4 — Binary count comment is wrong.** `dev/gate:111-114` and `dev_gate_report.rs:673` say every tier prints every binary. Logs show 15, 14 and 0. The count is a max and can understate.
- **L5 — `raise` is not a recorded fact** and is not bound to a round.
- **L6 — Nothing stops two concurrent invocations.**
- **L7 — `proposal` and `crossModel` agents carry no fenced *Never* paragraph.**
- **L8 — Eight definitions still name a `probe` step.** Acknowledged in the build entry.
- **L9 — Duplicate triage keys make `launch.add` throw.**
- **L10 — The doc's "4644" was measured one test short of the commit.**
- **L11 — The previous binary's hash is compared only when `regression` is true.**
- **L12 — A run slug ending `-rN` collides with round branch names.**

## Leads (not confirmed)

- The regex size limit as the slow filter grows.
- Agent-authored strings spliced into other agents' prompts.
- Triage mis-keying a new finding as found-again.
- Triage's door text is not cross-checked against the reporter's.

## Checked and correct

**Git steps**
- Every push is `git push origin fix/<slug>…`. There is no force, rebase, pull, reset or branch deletion.
- `main` is only fetched, and merged into the loop branch.
- The land step checks parents and product-path equality.
- The carry-over checks that it is confined to the run directory.

**Human rulings**
- `bound-set`, the three dispositions, `go`, `granted` and `rounds` appear only in `rulingsRecordPrompt`.
- The record script refuses a go or a grant the state does not ask for.

**Cross-model**
- With no item named there is no preflight question, no chain step and no prompt.
- The tool is named in three functions only.
- Thirteen "every item" values are refused.
- A void pass is logged, recorded and returned.

**Hashes**
- Compared for units (`:2012`, `:2204`), verifiers (`:1773`), and advocate and proposal (`:1805`, `:1812`).
- `finishTriage` inherits the comparison through `triagePasses`.

**Reporters**
- Every `roleStep` except the record executor is preceded by `launch.add`.
- Halted agents stay on the list.

**Definitions under `milestone-build.js`**
- It contains zero `REPORT:`, `BINARY:`, `AREA:` and `RECORD STEP`.
- Its one `BRANCH:` is not a binding label.
- New definitions each state their halts.
- Triage is denied inside, real/regression and disposition.

**Gate**
- The two tiers are complementary by construction and in logs: tier 1's skipped-via-filter equals tier 2's run count (1187, 1200, 1228, 1252).
- Doctests pass 0.
- No record puts every test in tier 1.
- `--fast` prints neither line; `--quick` prints no totals.

**The 4,644 / 4,645 question**
- `dev_gate_report.rs` went from 24 to 34 tests, and 4635 + 10 = 4645.
- The 20:20 gate ran 2962 + 1682 = 4644, with nine arms.
- The commit-time gate at 20:39 ran 3458 + 1187 = 4645.
- The tier lists (3478 + 1166) were taken at about 20:26, before the tenth arm.

**Doc sentences**
- They match the code, apart from M11 and L4.

## Not examined

- `dev/stabilize-record` internals and its suite.
- `crates/cli/tests/doc_link_fence.rs`, `release_pipeline_fence.rs`, `implementation/decisions-pending.md`.
- The Rust fences were read, not executed.
- The workflow doc did not exist yet.

## What only a real invocation can verify (the canary list)

1. **Runtime accepts the script** — top-level `return`, `parallel`, schema enforcement, a JSON-stringified `args`.
2. **Relay and payload hashing with real agents** — a Sonnet agent copying a multi-kilobyte JSON line byte-exact; the heredoc write and its `shasum`.
3. **Every git step once** — in a clone with a bare remote, including log-conflict resolution on land.
4. **Kill and halt at each boundary** — merge then push (H5); record then push; drop record then carry (M1); cut then record (H3); part record then land.
5. **The three exits, each interrupted** — and `raise` passed on one invocation only.
6. **Record step under a red gate** — and the dirty tree it leaves (M4).
7. **Reporter death modes** — dead; halted with a report; wrote then died; cross-model variants (M5, M9).
8. **Unfinished triage, both stages** — including in the same invocation as a cycle (H4).
9. **Preflight on a non-HEAD commit** — M6.
10. **Concurrency cap and long trial arms** — the cap of eight; a trial arm past 600 s; reviewers' rigs contending for `target/` (M7).
11. **Close sync, then one more stage** — M8.
12. **A fork cycle followed by a `later` ruling** — H1.
