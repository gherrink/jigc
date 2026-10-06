# The repair of the stabilization build — the planner's design

*Returned 2026-10-06 by a read-only planning agent that read both review reports (`state-machine.md`, `harness-agents-gate.md`, beside this file) and the code at `f6918aaa`. Transcribed by the orchestrator, unedited in substance. Nothing was run; the facts about moving the test suites are relayed from an earlier spike, not re-verified. No reviewer claim was refuted. Finding ids: `F…` and `S…` are the state-machine report's, `H…`, `M…`, `L…` the harness report's.*

## 1. Structural changes

- **C0 — every git act and the record act is one command** (added by the planner). A `dev/stabilize-step` prints one JSON line; a prompt becomes "run this, relay it". The ten git prompts are prose procedures today, which is why no simulation could hold git.
- **C1 — item results, derived clauses.** Result rows per item per round; `clauses.md` is rendered, and `state` refuses a view that differs. Corrected against the orchestrator's reading: not "never ran" but "selected by a round's derived scope, with no green run to its end" — an in-scope item no scope selected owes nothing (ruling 10). The clause census is written once at the opening. This closes re-run-alone and the deleted clause row; the deleted ledger row remains.
- **C2 — acts bracketed, reconciled first.** The round record gains `fixes` (finding, full sha, cycle, audited) and `acts` (intent, done). One landing predicate in `state`, untriaged included; "landed" is derived from git. Removes H1, H3, H4, H5, M1, M12 and finding F2, plus H2 and M8d. M4 belongs to C4.
- **C3 — counters with an exit:** triage passes per finding, attempts per item and stage, the cycle bound as a fact, a go per stop.
- **C4 — one atomic, journaled `apply`.** A red gate leaves a pending batch that the next invocation commits or discards.
- **C5 — a writer × state table**, property-tested over `commands.choices`, `ROUND_FACTS`, `RUN_FACTS`.
- **C6 — nothing decisive lives only in a return** (added by the planner): forks, `raise`, an exit's kept/left set, doors for retest.

Reconcile checks:
- the branch is a recorded one;
- the tree is clean or exactly a journaled batch;
- origin is an ancestor, and unpushed means push owed;
- each open intent names its resume step;
- the audited tree equals the round tip on product paths;
- every first-parent product commit since the opening is a recorded merge, by sha.

## 2. Findings → disposition

| Disposition | Findings |
|---|---|
| C0 + simulation | M10 |
| C1 | F1, F5, F6 (clause), F8, suite S3; leads: clause-less retest, halted re-run |
| C2 | H1–H5, M1, M12, M8d, F2, F12, suite S1–S2, L2, L6 (detected only), crash rows |
| C3 | F3, F9 |
| C4 | F4, M2, M4 |
| C5 | F5, F7 |
| C6 | M3, L5 |
| Own small fix | F6 (ledger count), F11, F13, M5, M6, M7, M9, M11, L1, L3, L4, L7, L9, L10, L11; leads: red check without a ledger row, door cross-check |
| Last task | M8a–c, L12 |
| Record, not now | F10 (safe side) · L8 (definitions shared with `milestone-build`) · Leaky (a pass) · leads: regex size, spliced strings, mis-keying (need the first run) |

## 3. Missing truth-table states

1. **F1:** three items on one clause; r1 `review-setup` void; fix, `cycles:1`; `scope-set 2` excludes it; r2 green → `retest`, never `close`.
2. **F2:** `ledger-set` f-1 `fixed` with no cycle, f-2 `later` → `fix`, audit owed, `candidate.current: false`. The suite's cell at line 4801 flips.
3. **F3:** two finishing attempts record `unclear` → `triage`, then `rule` (`unverified-after-retry`).
4. **H1:** `cycles:1`; an unaudited fix row in cycle 2; f-3 `later` → `fix`, `land: false`.
5. **H2:** an audit finding `breaks` with no verdict → `triage`, `land: false`.
6. **H3:** a `cut` intent with left=[B] and no done → `position.fix.resume: cut`; f-B routed `fix`; a later drop reopens both.
7. **H5/M1:** a `land` or `drop` intent without done → `resume`; `test` refused `act-pending`, never `round-over`.
8. **H4** (simulation): the finishing triage's preflight receives `git rev-parse <round branch>`.
9. **F5:** `round-set {alone}` while `next` is not `retest` for that clause → refused.
10. **F7:** stopped: `round-set --round R+1`, `report --round R+1`, `run-set {stop}` each refused.
11. **F4:** the scanner dies between the two files → both unchanged.

## 4. Tasks

| # | Task | Done when | Not touched | Size |
|---|---|---|---|---|
| 0 | Move the 15 suites to a top-level home outside `crates/` and `dev/` | `passed=` equals baseline; `cargo package --list` and the publish dry run unchanged; the registration fence derives both homes | test bodies | L |
| 1 | C0, behaviour-preserving | each step driven once in a clone with a bare remote | what a step checks | M |
| 2 | Simulation | test → ruling → fix → land → dropped round under 30 s, golden label trace | harness logic | M–L |
| 3 | C4, F11 | state 11 red, then green | harness | S–M |
| 4 | C1, red check → finding | states 1, 9 | harness | L |
| 5 | C2, script half | states 2, 4–7 | harness | M |
| 6 | C3, C5 | states 3, 10 | harness | M |
| 7 | C2, harness half | killing after every label of the trace converges, never `landed` unpushed | record script | L |
| 8 | C6, reporter deaths (M5, M9) | simulated | record script | M |
| 9 | Land a part, as ruled | §5 | — | S–M |
| 10 | Fence arms → simulation assertions; headers | M10's mutant red | — | S |
| 11 | Gate M11 · definitions M6, M7, L7 | — | — | S |
| 12 | Path-class by recorded sha, L12, product list | sync, then one stage | — | S–M |
| 13 | Re-review; canary (the harness report's list, plus kill-per-act and a red record gate) | — | — | — |

- **Order:** 0→1→2 serial; 3→6 serial (one file); 7→9 serial, after 5 and 6; 10–11 parallel; M6 after 7; 12 last.
- **Manifest:** `crates/cli/Cargo.toml` changes once, in task 0 (one `[[test]]`, "12"→"13"). It never changes for a new suite, only for a new target. *Inferred:* cargo drops a target whose root is not packaged; task 0 proves it.
- **Determinism:** agents are a label → function table, and an unknown label fails. `parallel` runs in order. Git identity and dates are fixed, hooks off. `dev/gate` and `gitleaks` are stubbed. One repo per test, one `node` process per invocation.
- **No `node`:** fail when `CI` is set; locally pass, and `dev/gate` prints a NOTE — nextest hides a passing test's stderr, so today's skip is silent. *Inferred:* hosted runners ship node; `ci.yml` installs none.

## 5. Land a part

- **A — repair the re-cut.** Record the cut and its repoint rows before the act; `currentCut` takes only recorded cuts. Cost: a branch, new shas, a second audit, six interrupt scenarios. Disposes of H2, H3, M1's part site.
- **B — remove.** The exit is refused; the kept fixes are redone next round, costing one `test` stage and one cycle. Disposes of the same findings, and deletes `repointPatches`, `-partK` and outcome `part`.
- **C — part by revert** (added by the planner). The left-out fixes are reverted on the round branch, audited as exactly that diff, and landed by the shared predicate. No re-cut, no repoint. It departs from "re-cut as its own branch tip".

## 6. What the planner would cut

- **The re-run as a round.** Make it an attempt in the round that selected the item; `alone`, `spent` and `not-scoped` go. *Inferred* that "one more test round" is not literal.
- Prose git procedures and per-row payload hashing.
- `landed-before` as a status; the unread fields (L1).

## 7. Qualifications to reviewer claims

- **H4:** `stabilize-preflight.md:16,36` asserts `HEAD` is the candidate, so the real outcome is a halt. Fixing M6 first would unmask it.
- **H3 is worse:** the carry step requires the part's branch not to exist, so a halted cut cannot be retried. A drop on a part leaves both rows `fixed` — a false close.
- **M8a:** reach is a stage invoked after the close.
- **F7.1–2:** direct calls only; the position refuses the harness first.

## 8. Decisions the planner returned as the human's

1. **Land a part:** A, B or C. The planner would take C, else B.
2. **A record commit under a red candidate gate.** Today the round is unrecordable. Options: record commits run the tooling tier only (a directory after task 0), or the full gate stays and a red candidate halts unrecorded. This touches ruling 15.
3. **An unverified finding after its retry:** the three dispositions only, or also "verify again".
4. **`.cargo/` and `rust-toolchain.toml` in the product list:** add them (they shape the binary), or leave them out and record why.
