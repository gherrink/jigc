# The stabilization workflow — the settled shape and its build

## Context

The gate before 1.0.0 on `jigc 1.0.0-rc.24` ran as an ad-hoc loop — test, fix, re-test — and did
not converge: six blocking rows became 57 commits on `fix/rc24-tier1`, the audit of fix round 3
returned 54 findings, and round 4 plus the last fix (36 commits after `5f5b273a`) are unaudited.
The loop lived in one orchestrator's head; its parked description is `ideas/gate-loop-workflow.md`.

The human wants it as a real, reusable workflow, built from what exists, before any further
testing round: **it takes a build and a closing condition, finds and fixes until the condition
holds, records everything else, and ends with a version ready to release.** It runs largely
without the main session, which comes in only for decisions — as milestone planning and the
milestone build do. This run on rc.24 is its first; in the first run it stops after every round
so the workflow can be fine-tuned.

Everything under *The rulings* was decided by the human on 2026-10-05, one question at a time
(design workflow: discuss → converge → write). The plan was then read by two independent
reviewers — the project's `design-reviewer` and a cross-model pass (Codex) — and each finding was
checked against the files before it was worked in. **Four of the repairs adjust a ruling; they
are listed first and are the human's to confirm with this plan.** Approving it is the "write it".

## A fact the review surfaced — true today, whatever is decided

**CI is red on the loop branch's tip.** `fix/rc24-tier1` failed `test (g_migrate)` and `ci-ok` at
`ed783637`, `822e2edf` and `357ab425`; the last green run was `5f5b273a` (checked with
`gh run list`). The failing test named in the latest log is the test of round 4's fix `f0b1120a`
(`setup` refuses where git cannot answer). Every fixer reported a green local gate: the local
gate is macOS, CI is the first Linux run, and the pass's record does not mention the red. It
enters this run's ledger at the opening as a regression of the pass.

## Four adjustments to rulings after the review — to confirm

- **(i) Ruling 2 — a Linux check per candidate.** "Nothing through GitHub" becomes "nothing that
  needs the human's merge". Branches are pushed anyway and CI runs on every push, so the
  candidate's `ci-ok` result is **read** for its sha and belongs to the checks that run on every
  candidate; where it cannot be read, the gate runs under `dev/runner-faithful`. Without it the
  loop closes on macOS and the pull request's required check is red.
- **(ii) Ruling 7 — the invariant is by path, not by directory.** "The loop tip is an audited
  candidate apart from commits confined to the run's record directory" would be tripped by the
  first fine-tuning commit to the harness. It becomes: **paths the published crates carry, and
  their tests, move only through an audited round's merge; every other path** (the run's records,
  `.claude/`, `dev/`, `implementation/`, the logs) **commits directly on the loop branch behind
  the gate.** A git step asserts it by path class. Round 1's audit is scoped to the product paths.
- **(iii) Ruling 3 — checks before the irreversible merge, not only after it.** Once the pull
  request is merged and release-plz has put the version on the release PR, and before the human
  merges that PR, agents run on its head: the packaged-tarball install
  (`dev/runner-faithful --commit <sha> tarball`), a comparison of the packaged file set with the
  tested commit (expected differences: both crates' versions, the `=` engine pin, `Cargo.lock`,
  the changelogs), and **one upgrade arm from the previous release to the new version** — which
  no same-version local candidate can exercise, because the store's version advisory and the
  guide's replacement fire only when the version differs. After the publish only the registry
  install proof remains. It is **one release of two crates**: `jigc-engine` is versioned on its
  own and its `pub` surface moved in this pass.
- **(iv) Ruling 7 (a) — where the `fix` stage's records go.** The round branch is checked out
  during `fix`, so that stage's records ride the round branch, inside the run directory, and
  reach the loop branch when the round lands. When a round is dropped, a git step carries exactly
  the commits confined to the run directory over to the loop branch. The `test` stage's records
  land on the loop branch directly, as ruled. A dropped round still keeps its record.

**One dissent, relayed and not worked in.** The cross-model reviewer holds that scoping by door
is not safe without a full run of every model-driven instrument before closing. The human ruled
that out, for the reason that a repeated hunt over unchanged code finds new things by sampling.
What is worked in instead: the door derivation is stated and reviewed and defaults to *inside*
where it cannot be bounded (ruling 10), and at closing the record shows, per clause, how many
commits behind the tip its last evidence is — so the human can order `scope: everything` at the
last stop with that number in front of them.

## The rulings

1. **General, and named "stabilization".** The closing condition is the workflow's input; the
   outcome is a version ready to release (1.0.0 now; a 1.1.0 or 1.1.0-rc.1 later). Prose, a
   harness script and agents — not a jigc pack workflow; that migration is later.
2. **Candidates are local.** A candidate is a **commit** on the loop branch, labelled `c1`, `c2`,
   … in the run's record and identified by its sha. It is built locally from `git archive <sha>`.
   No version bump, no tag, nothing that needs the human's merge inside the loop; pushes and
   reading CI are fine (adjustment i). Every local candidate prints the previous version string,
   so evidence is keyed by sha and by the binary's hash — and **the candidate's binary is first on
   `PATH` in every driving agent, with the hash of `command -v jigc` asserted**, because a printed
   route run as printed otherwise resolves to the installed previous release.
3. **One release at the end.** The loop closes locally; then one pull request, the human's merge,
   the checks on the release PR's head (adjustment iii), the version set and the merge by the
   human, the publish, and the registry install proof.
4. **What ends the loop.** The test set is fixed before round 1. A finding inside it that breaks
   the condition, and a regression the run's own fixes introduced, are fixed without asking. A
   verified finding that breaks the condition but was found **outside** the test set goes to the
   human at the end of the round as a list — per item: admit to this run · declare a bound with
   its reach · record for a later release. **"Closed" = every clause's instrument has a green row
   on record, and every verified finding that breaks a clause inside its scope is either fixed or
   carries the human's ruling.** For now the human rules; the step is written as replaceable by
   an agent later. Every finding gets a ledger row.
5. **Grading, made tight after the review.**
   - The triage agent grades **every** finding, not only the ones a reporter called severe; an
     independent agent verifies everything graded *breaks* or *unclear* (verify-real).
   - **"Inside the test set" is decided by the script**, not by an agent: the finding's door is in
     the round's committed door list.
   - **"A regression of the run"** is a fact: the repro is green on the previous release's binary
     and red on the candidate. Preflight supplies both binaries.
   - **A grade of "out of scope" must cite a row of the opening record's declared-bounds list;**
     otherwise the finding goes to the human. A declared bound is always the human's ruling, and
     once ruled it is pinned as a test or recorded as explicitly unpinned.
   - Every entry a fixer, auditor or advocate **leaves open** becomes a ledger row and is triaged
     like any finding — never queued for a fixer on its own. A finding found again inherits its
     row's disposition.
   - A contested fix, or one that needs a new mechanism, is a halt with a `robust-advocate`'s
     case, the proposal driven through the next step before the human sees it.
6. **Bounds.** Inside a round at most **3** fix → audit cycles; then the round halts to the
   human unlanded, with the ledger. Across rounds: the first run stops after each; later runs
   default to **3 rounds**, written in the opening record. Only the human raises either, per run.
   At a bound the exits are: continue · drop the round · land a part — and a part is re-cut as its
   own branch tip, gated and audited once more as exactly that diff before it lands.
7. **Branches.** A round is its own branch; it is pushed by name and lands on the loop branch
   (`--no-ff`) only when the audit of its fix diff is green. Product paths move only through such
   a merge; everything else commits directly behind the gate (adjustment ii); records as in
   adjustment (iv). *This run:* loop branch `fix/rc24-tier1`, rounds `fix/rc24-tier1-r<N>` — both
   pass `dev/branch-name` today, so no rule or tooling changes. *Afterwards:* a branch type of its
   own, `stabilize/<slug>/main` and `stabilize/<slug>/r<N>`, applied as the last commit on the
   loop branch before its pull request (`dev/branch-name`, `branch_name_fence.rs`, `CLAUDE.md` →
   Branches, `release.md`, the fenced *Never* paragraph in every agent definition). The harness
   mints every branch name in one function, so that switch is one small change.
8. **An agent may land a round branch.** One sentence in `implementation/release.md` → *What
   agents may not do* now; the fenced paragraph in the agent definitions is rewritten with the
   branch type in 7.
9. **One script, two stages, each a fresh invocation.** `test` (preflight → instruments →
   triage → verify-real) and `fix` (fixers → audit of the fix diff, ≤ 3 cycles → land). Every
   point where the human may rule sits between two invocations. Each invocation reads committed
   state, never a previous run's memory; resume by run id is only for a killed run. When `test`
   leaves nothing to rule on, the orchestrator starts `fix` at once. The stages may be combined
   later.
10. **What a round tests: scoped by door, shrinking.** Round 1's test set is what the candidate
    changed against the previously released version; each later round tests only what the
    previous round's fixes affect. The scope is resolved **each round from the current code and
    docs** by its own read-only step, which writes the doors it **included and the doors it
    excluded, each with its derivation** (changed symbols → their consumers → the registry the
    doors come from); where the consumers of a change cannot be bounded, the whole registry is
    inside. The fixer's and the fix-diff auditor's own door lists are inputs to that step, not its
    result. The scope is also a parameter of `test`: the derived delta (default) · everything · a
    commit range or named doors; a scope the human widens is that round's test set. **Hunt once
    per change, check always:** the model-driven instruments (review rows, audit, trial arms) run
    only over what changed; the deterministic checks run in full on every candidate — the full
    `dev/gate` (into which every confirmed and refuted repro is pinned), CI's `ci-ok` for the
    candidate's sha, the packaged-tarball install, and the scripted regression set. The round
    record reports the item count and the blocker count.
11. **One binary per candidate.** Preflight builds one release binary (read-only, its hash in the
    round record) and one trial image from the candidate's commit, and provides the previous
    release's binary for ruling 5. Every agent asserts the hash before it drives anything.
    Independence is what agents do not share: own scratch root, own rigs, no sight of another's
    report. (The debug-only route fences are not in a release binary; the gate is what runs them.)
12. **Reports go into the repository; none is left behind.** Every reporter of a stage writes its
    report into the run's directory in the working tree through one script, which decides the path
    (with an attempt id, so a re-run never collides with a stale file), refuses anything outside
    the run directory, replaces host paths with the public placeholders, and **stops hard on the
    public-hygiene scan** — a report is public on the next push, and the install-line fence reads
    every markdown file, untracked ones included. The read-only agents stay read-only for code.
    The stage's serial record step requires exactly one report per agent the script launched — a
    missing or extra one halts — then gates, commits and pushes.
13. **The ledger.** One markdown table per run, one row per finding: key · doctype
    (`jigc-feedback` or `inconsistency`) · round · source · door · clause broken · grade · graded
    by · disposition · repro pointer. Keys follow the grammar of
    `completions/artifacts/M55/seed-ledger.md`, so each open row later files as one jigc doc.
    Neither agents nor the harness write it by hand: a script appends rows from the stage's
    structured results and checks one row per finding id.
14. **A run opens with a human-led step** in the main session, as milestone planning does. It
    produces the committed opening record: the closing condition (by reference), the candidate and
    the previous release, the release being prepared, the test set per instrument, the
    declared-bounds list, the bounds of 6, the stop mode, the default scope.
15. **Performance levers.** Taken now: **1** (`dev/gate` stops on a red lint or build step; a
    fast test tier first, the second tier its complement by construction; the fast tier as a
    stand-alone pre-check), **6** (timings in the gate log), **9** (scoped test runs with a
    timeout) — one tooling task; **4** and **7** into the fixer brief; **8** is how `fix`
    partitions (one fixer per area, one finding per commit). **The standing rule is unchanged: the
    full gate precedes every commit**, until the human rules otherwise at a stop. Looked at again
    at the first stop with this run's own numbers: levers 2 / 3 — the human's idea is a script
    that determines which part of the gate a commit needs, with the complete gate at the end —
    and 5, 10, 11. Lever 12 is a row for the 1.x fix pass (M57).
16. **The two missing instruments.** The scripted regression set and the port rehearsal do not
    exist. Round 1's `test` starts with what does; meanwhile both are designed with the human,
    one at a time, and built by subagents. They then run on the next candidate over the whole
    delta since the previous release — the one stated exception to 10. **Closing is computed, not
    remembered:** the run keeps one committed table with a row per clause — instrument, the last
    commit it ran on, its scope, and green / red / **void** (not reached, could not run) — and the
    script's "close" is a pure function of that table and the ledger. A void or missing row
    forbids closing.
17. **The main session only orchestrates.** Reading long reports, building, checking and writing
    go to subagents that return a compact summary and write the full result to a file. The agent
    that builds the workflow reads the governing docs itself; its brief points at them and at the
    recorded decisions and restates nothing.

## Rules already in force that the workflow carries by reference

It points at these and restates none: the exit rule (`DECISIONS.md` 2026-10-04) · a blocker is a
post-review fix, never a new milestone · verify-real, a repro block for every verdict, pinned or
declared unpinned (`milestone-completion-workflow.md`, `pinning.md`) · the fix lane: finding not
boundary, the class with how its count was derived, one finding per commit, write-set partition
(completion workflow → Fix) · round 4's brief: red first, the smallest correct change that closes
the class, a new stored file / registry / flag / finding code is a halt, ask git, must-not-refuse
cells, every printed route run as printed, docs in the same commit (`DECISIONS.md` 2026-10-05) ·
every change future-proofed · the full unscoped gate before every commit, no path exempt, read
unpiped (`dev-workflow.md` → Gate) · a record commit opens and closes every round, push at every
phase boundary, records sanitized (`public-hygiene.md`) · the branch model and what agents may not
do (`CLAUDE.md` → Branches, `release.md`) · forks to the human one at a time, plain text, lettered
options · trial tooling reused from its four homes · the port rehearsal is never a requirements
source · the close: fold-back, verdict addendum, row annotations, `dev/clean-litter`.

## The build

**Files.**

| File | Purpose |
|---|---|
| `implementation/stabilization-workflow.md` (new) | the workflow's one home: what it is, the opening, a round's `test`, triage and the human's list (marked replaceable), a round's `fix`, bounds and exits, branches (no names spelled), the ledger, the close, orchestration — each section pointing at the doc that owns the rule |
| `.claude/workflows/stabilize.js` (new) | the two-stage harness, on `milestone-build.js`'s patterns |
| `dev/stabilize-record` (new, working name; subcommands for a report, a ledger append, the two one-to-one checks) + its test | the one way a report or a ledger row reaches the run's directory (rulings 12, 13) |
| `.claude/agents/*.md` (new, only where the builder shows no existing agent fits; candidates: a preflight/state/scope agent, a finding verifier, a triage agent, a recorder — `build-executor` is tried first for the recorder) | each carries the fenced *Never* paragraph |
| `.claude/agents/build-git.md`, `milestone-e2e-tester.md`, `milestone-code-reviewer.md`, `robust-advocate.md`, `build-fixer.md`, `build-executor.md`, `increment-validator.md` (edit) | **contract changes, stated as such and recorded:** a report written through the script; the provided binary driven after its hash is asserted; one area's findings, one commit each — every one written so it is **inert under `milestone-build`** (it binds only when the prompt supplies a run, a binary or an area). And one correction of a stale fact: the gate takes 590–612 s against a 600 s ceiling and is run in the background, not "about 5 minutes in the foreground" |
| a source-scan fence over the script + its group root; `doc_link_fence.rs`, `merge_logs_fence.rs` (edit) | every agent type resolves; every git step on Sonnet; round-branch names minted in one function; the sync step's log list in parity; the new doc listed as live |
| `dev/gate`, `crates/cli/tests/dev_gate_report.rs`, `.config/nextest.toml`, `implementation/dev-workflow.md` → Gate, `implementation/machine-setup.md` | levers 1, 6, 9 |
| `DECISIONS.md`, `implementation/decisions-pending.md`, `ideas/gate-loop-workflow.md`, `VISION.md` (its index line), `CLAUDE.md`, `implementation/release.md`, `completions/artifacts/M55/fix-pass-rc25/README.md` | the records, ruling 8's sentence, and a dated bracket on each place that still states the superseded order (re-audit → next candidate → instruments on it → the call) |

**The harness.** Args `{ stage, run, scratch, scope?, rulings?, raise?, stopAfter?, selfTest?, model? }`.
The round number and the cycle number are derived from committed state, never passed. A missing or
malformed `stage`, `run` or `scratch` is refused before any agent. Runtime limits it is built
inside: no shell, filesystem, clock or mid-run input in the script; a resume needs identical args;
at most 8 agents at once on this machine; the per-axis review script hardcodes its scratch root
and binary, so the harness carries those as parameters instead of calling it.

- **`test`:** git state (`build-git`, Sonnet; the path-class assert; untracked files confined to
  the run's report directory are expected) → read the opening record, ledger, clause table and
  round → environment asserts; the candidate's binary and image
  (`completions/trial-harness/build-image.sh <sha>`, `verify-image.sh`), the previous release's
  binary, the candidate first on `PATH` → the deterministic checks (gate, CI result for the sha,
  packaged-tarball install, the regression script once it exists) → the scope step: included and
  excluded doors with derivations, the resolved test set → the instruments in parallel under the
  cap (review rows: source pass ∥ driver, then reconciler; trial arms: rehearse, run, score; the
  audit over an unaudited range of product paths: `milestone-code-reviewer` per area, one
  cross-cutting, `milestone-e2e-tester`; the rehearsal once it exists) → triage of every finding →
  verify-real on *breaks* and *unclear* → per fork a `robust-advocate`, then the proposal driven
  as printed → the record step (one report per agent checked, the triage record, ledger rows and
  clause rows through the script, gate, commit) → the ledger read back, one row per finding id
  asserted, the loop branch pushed. Returns
  `{status:'triaged', round, candidate, counts, human_list, forks, next:'fix'|'rule'|'close'}`,
  `next` computed from the clause table and the ledger.
- **`fix`:** git state → the human's rulings recorded → the round branch opened → fixers, one
  area at a time, serial in the main tree, one finding per commit, each report carrying
  `class, count_derivation, doors_affected, left_open, halt` → the round branch pushed → the
  round-tip binary built → the audit of the fix diff in parallel → triage → verify-real → record
  on the round branch → again from the fixers while blockers remain, at most 3 cycles → land
  (`--no-ff`; the merged tree equals the round's outside the run directory; push). Returns
  `{status:'landed', candidate, cycles, counts, doors_for_retest}`; at the third red cycle
  `{status:'halted', phase:'bound'}` unlanded; on a fixer's new-mechanism halt `phase:'fork'` with
  the advocate's case.

**One run's files** under `completions/artifacts/<run>/`: `opening.md` · `ledger.md` ·
`clauses.md` · `r<N>/scope.md` · `r<N>/triage.md` · `r<N>/round.md` ·
`r<N>/reports/test/…` · `r<N>/reports/fix/c<k>/…`.

**Build order** — single-concern `build-executor` tasks on the loop branch (Opus; git steps on
Sonnet), each behind the full `dev/gate`, each reading the governing docs itself:

1. **Gate tooling.** Done when `dev_gate_report.rs` holds: a red clippy means no test argv; argv
   = fmt, clippy, build, tier 1, tier 2 (`not` tier 1's filter, the same variable), doctests; the
   pre-check prints neither of the two lines `milestone-build.js` reads; totals unchanged for a
   green log; step seconds and per-test timings in the log; the nextest-absent path unchanged.
   Measured before landing: the baseline gate (wall, `passed=`, the set of test names), the tier
   derivation (candidate rule: a suite that spawns no child process, derived at run time, never
   hand-listed), the fast tier's wall cold and warm. **It reports back unlanded if the fast tier
   is over about 2 minutes.**
2. **The record script** and its test: refuses a path outside the run directory, replaces host
   paths, stops on the hygiene scan, appends ledger rows, runs the two one-to-one checks.
3. **Agents:** the new definitions and the edits; `release_pipeline_fence` arm q and
   `planning_gate_home` green.
4. **Harness:** the script and its source-scan fence; `selfTest` truth-tables the close function.
5. **Doc:** the workflow doc and the pointer in `CLAUDE.md`; `doc_link_fence` green. It lands
   after 3 and 4 because every path it names must resolve.
6. **Records:** `DECISIONS.md` (this session's rulings and the four adjustments, dated, each with
   its one line of why; the changed instrument for the trial — a local build in place of the
   registry install, with the proof that moves to the release PR's head named; that a workflow
   de-parks into `implementation/`, not `design/`); `decisions-pending.md` — graduated: the
   gate-loop row and the performance-lever row; new, each with its trigger: branch type
   `stabilize/` (before this run's pull request), the script-derived per-commit gate and levers
   5 / 10 / 11 (the first stop), lever 12 (M57), the handing of ruling 4's list to an agent (when
   the human says); the idea file's de-parking and the `VISION.md` index line; ruling 8's
   sentence; the brackets on the superseded order.

A `milestone-code-reviewer` reads the build's own diff before the harness is used. That diff is
tooling and is not part of any candidate's audit.

## Verification — nothing here touches the real loop branch's history

1. `selfTest: true` runs zero agents: key derivation, branch names, the one-row-per-finding and
   one-report-per-agent checks over synthetic input, the gate-reading patterns, and the close
   function over its truth table (a void row, a missing row, an unruled list item, an open in-set
   blocker each forbid "close").
2. Refusals: empty args, a bad stage, a bad run slug; `test` before an opening record returns
   `halted:state`; `fix` before a triage record is refused; a dirty tree returns `halted:git`.
3. **A canary run** in a `mktemp` clone with a bare remote: one door, one planted finding, driven
   through `test` → the human's ruling (scripted) → `fix` → land, and a second round that is
   dropped — so every git step, the record step, the path-class assert, the landing and the
   carry-over of a dropped round's records have run once where a mistake costs nothing. Pushes on
   the real branch cannot be force-undone, which is why this comes first. *(Its cost is real: the
   canary's fix runs a full gate.)*
4. The gate: one full `dev/gate` green on the built tree with the baseline's `passed=` count plus
   the new tests, and the pre-check's wall as measured.

## After the build — this run, not part of this plan's approval

- **The opening of this run, with the human:** loop branch `fix/rc24-tier1`; previous release
  `1.0.0-rc.24`; the closing condition = the exit rule of 2026-10-04; candidate `c1` = the tip
  once the build has landed. Round 1's test set: the audit over the product paths of
  `5f5b273a..c1`; the review rows and trial arms the pass touched — **the review instrument holds
  briefs for M54 and M55 only, so a subagent drafts the row scopes for the pass's delta from the
  code's registries and the human approves the rows**; the trial's inconsistency arm with a
  rehearsed, unambiguous occasion. The declared-bounds list is started from the exit rule's own
  (races against a non-jigc writer, deliberately planted states). The ledger opens with the
  fix-pass ledger's closing groups (a), (b), (c) — 29 rows — and **the red CI run**, and takes the
  rest in by reference.
- **To settle at the opening:** which of the twenty fixer decisions never put to the human are
  brought **before** round 1's hunts — the reviewer's point is that reversing one changes code the
  hunts would then have read for nothing.
- **Round 1's `test` stage**; in parallel the design of the regression set and of the port
  rehearsal with the human.
- **At round 1's stop:** the human's list, one item at a time, and the fine-tuning of the workflow.
- **At the close:** the pass's fold-back (`completions/artifacts/M55/fix-pass-rc25/README.md` →
  *What is owed next*, step 3), the `stabilize/` branch type, `origin/main` merged in, the gate,
  the push, the pull request. Stop. The human merges; the checks of adjustment (iii) run on the
  release PR's head; the human sets the version and merges it.

## What the two reviews found, and what became of each

*Design reviewer (same model): 4 blocking, 5 should-fix, 3 notes. Cross-model (Codex): 6
blocking, 3 should-fix, 1 note. Checked against the files before being worked in.*

| Finding | Disposition |
|---|---|
| No Linux leg; the tip is red in CI now (confirmed with `gh run list`) | accepted — adjustment (i), the fact above |
| Triage can shrink the human's list unseen (an "out of scope" grade; severity as a pre-filter; no decider for "inside"; `left_open` with no route) | accepted — ruling 5 rewritten |
| The tip invariant contradicts tuning the harness on the loop branch | accepted — adjustment (ii) |
| The publish-once checks sit after the irreversible act; no upgrade path with a same-version candidate; a bare `jigc` in a printed route resolves to the installed release | accepted — adjustment (iii), ruling 2's `PATH` assert |
| The local build is not the registry-mode trial the record names as the instrument | accepted in part — the packaged-tarball install runs on every candidate; the changed instrument and the proof that moves are recorded. The human ruled the local close knowing this |
| "Cannot close before each instrument ran" is stated, not enforced; no status for "could not run" | accepted — ruling 16's clause table and computed close |
| "Closed" could be read to exclude a known blocker outside the run's scope | accepted — ruling 4's wording |
| The verification cannot catch a broken harness | accepted — the canary run |
| The `fix` stage's mechanics are undefined (records while the round branch is out, tree equality, partial landing, cycle count, stale reports) | accepted — adjustment (iv), rulings 6 and 12, the harness section |
| Reports become gate inputs and public on push; the ledger has no writer | accepted — rulings 12 and 13 |
| The superseded order still stands in three places | accepted — build step 6 |
| "Full gate in round 1" implies a later exemption | accepted — ruling 15's wording |
| "One published version" is two crates | accepted — adjustment (iii) |
| The workflow's own build is mixed into the candidate's audit range | accepted — audit by product path; the build reviewed on its own |
| The agent edits are contract changes and must be inert under the milestone build | accepted — the files table |
| The ledger's columns do not follow the seed ledger's | accepted — a doctype column; the wording corrected to "keys follow its grammar" |
| The round-branch name is illegal and its push will fail CI (Codex) | **rejected** — `dev/branch-name fix/rc24-tier1-r1` exits 0 (the design reviewer ran it); the `stabilize/` names are used only after their rule lands, in the same commit as the name function |
| Scoping by door needs a full run of every hunt before closing (Codex) | **not worked in** — it reverses a ruling; relayed as the dissent above |
| `branch_name_fence.rs` listed as a build edit though this run changes no branch rule | accepted — removed from the build; it moves with the branch type |

*Not checked by either reviewer, and so unverified:* whether a harness-launched agent can hold a
blind trial session past the 600 s tool ceiling; how the runtime enforces its cap of eight; the
zero-token replay of a resumed run. The canary run is where the first two show.
