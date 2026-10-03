<!-- Reconciled ROW 8 file (composed surfaces), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis8` in the body means ROW 8 of this run, not numbered axis 8. -->

# rc.24 partial per-axis re-review — ROW 8 · composed surfaces — RECONCILED

> **Reconciled file.** Sections 1–8 below are the Opus driver's table, unchanged except for the
> demotions marked **DEMOTED (reconciler)** in §2 and the corrected count line under it. The
> reconciliation — every Codex claim driven, every driver defect re-driven, the tier adjudication, the
> baseline dispositions and the reconciled door list — is the **Reconciliation ledger** at the end
> (§R). The Codex source pass for this row **exists** (one claim, eleven baseline dispositions, nine
> consistency statements) and was reconciled against in full. The reconciler authored neither the
> driver file nor the code.

# The driver table — rc.24 partial per-axis re-review — ROW 8 · composed surfaces — the OPUS DRIVER

Row 8 of this run = numbered axis 6 (composed surfaces), scoped to M55's sub-task composition (S2)
and its four findings workflows, plus the baseline rows the scope names. Baseline: rc.19
(`completions/artifacts/M53/per-axis-review-rc19/`).

- **Binary:** the installed registry build `~/.local/bin/jigc`; `jigc --version` → `jigc 1.0.0-rc.24`
  (asserted before the first drive and again at the head of every script). Release posture.
- **Source read at:** `bffa6667` (branch `work/rc24-gate`; the instrument was read at `aa6666cb`).
- **Rigs:** 18 throwaway rigs, each `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit`
  with stdout only captured, then sourced under a `[ -n "$REPO" ]` guard plus a check that `$REPO`
  is a rig path. States used: `fresh` (15), `committed-singletons` (3). No teardown. No `git`
  command was typed outside a rig; nothing was written, staged or committed in the working
  repository.
- **`CLAUDECODE` is set** in this session and was held constant: every commit jigc made in a rig
  carries `Co-Authored-By: Claude <noreply@anthropic.com>`. Expected, row 10's subject, not a finding
  here.
- **I am the N-process sim.** Every `Spawn:` line was run verbatim (`sh -c "<line>"` from `/`, so
  the `cd` in the line is what lands it), sequentially, by this one process. No real sub-agent was
  launched; M55's genuine concurrent spawn is on its own record and was not re-run.
- The Codex source pass for this row (`axis8-prompt.md`, `codex/`) was **not read**.
- Paths below: `$REPO` = the rig repository, `$RIG` = its root, `<tmp>` = the host temp prefix
  elided from composed text.

**Verdict for the exit rule: NO TIER-1 ROW found on this row.** Four new findings, all proposed
tier 3. The declared bound **HOLDS AS DECLARED** at the door its sentence names, and is **wider than
its sentence** at one door (filed as `(R8, F-4)`, the difference, not the bound).

---

## 1. The door set and the registry counts, read from the code

| registry | file | count I read | the instrument's author read | datum |
|---|---|---|---|---|
| callers of `render::composed` / `render::composed_preview` | `crates/cli/src/cli.rs` (6) · `migrate.rs` (1) · `milestone.rs` (1) · `task.rs` (1) | **9 call sites** (8 `composed` + 1 `composed_preview`) over **5 verbs** | 8 call sites over 5 verbs | **differs by one.** The author's own enumeration lists nine forms (`start` ×4 · `workflow` ×2 · `migrate` · `milestone execute` · `task amend`); the "8" is the rc.19 count before `task amend` (`task.rs:626`, `run_amend`) joined. Verbs agree: 5 |
| workflows shipped | `crates/cli/packs/{dev,methodology}/workflows/` | **18 + 21 = 39** | 39 | = |
| `suppressed.door` set (`^  door:`) | same | **15** (8 dev + 7 methodology): `amend` · 12 `migrate-*` · `milestone-execution` · `sub-task` | 15 | = |
| steps shipped | `crates/cli/packs/{dev,methodology}/steps/` | **31 + 44** | 31 + 44 | = |
| steps carrying `{{ cli.finalize-task }}` directly | grep of both step trees | **4**: dev `finalize`, dev `amend-message`, methodology `finalize`, methodology `finalize-doc-only` | 4 | = |
| steps including one of those four | grep `include: step:(finalize\|finalize-doc-only\|amend-message)` | **4**: dev `migration-finalize`, dev `project-finalize`, methodology `migration-finalize`, methodology `planning-finalize` | "wrappers by inclusion" (not counted) | datum |
| steps carrying a catalog ref that writes the commit doc (`from: task.commit#…`) | grep of `set-commit-*` refs | **3**: dev `author-commit`, methodology `author-commit`, dev `amend-message` | "commit-doc authors under squash: true" (not counted) | datum |
| `OFF_CATALOG_VERBS` | `crates/cli/src/orient.rs:48` | **2**: `planning` · `ingest-existing` | 2 | = |
| the four findings workflows | front-matter | visible **1** (`report-inconsistency`: `selectable: true`, a `when:`) · hidden **3** (`selectable: false`, `suppressed:`, no `door`) | 1 / 3 | = |
| `creates-task` partition of the 39 | front-matter | **35** `true` · **4** `false` (`router`, `ingest-existing`, `increment`, `milestone-execution`) | not stated | datum |

**Door set of this row (9):** `start` · `workflow` · `migrate` · `milestone execute` · `task amend` ·
`describe` · `doc show` · `doc list` · `task validate`.

**The omission set I observed, derived from composed output (not from the list above).** For each
workflow: lines composed top-level (`jigc workflow <W> --preview`) → as a sub-task under
`squash=true` → as a sub-task under `squash=false`:

| workflow | top | sub · true | sub · false |
|---|---|---|---|
| `single-task` | 211 | 145 | 194 |
| `quick-fix` | 78 | 12 | 61 |
| `dev-task` | 95 | 31 | 76 |
| `park-idea` | 114 | 50 | 95 |
| `planning` | 425 | 342 | 387 |
| `record-decision` | 134 | 68 | 117 |
| `completion` | 280 | 216 | 261 |
| `report-inconsistency` | 127 | 60 | 105 |
| `triage-inconsistency` | 120 | 53 | 98 |
| `report-jigc-feedback` | 134 | 67 | 112 |
| `triage-jigc-feedback` | 128 | 61 | 106 |

What left, by what the text shows: under both knob values, every `Run: jigc task finalize` line and
the prose of the step carrying it (dev/methodology `finalize`, `finalize-doc-only`), and the
wrappers' own prose (`planning-finalize`'s *"Committing this plan lands…"*: 1 hit top-level, 0 in
the sub-task; the `migrate-*` wrapper's `--approve` prose: 1 hit top-level, 0 in 12 sub-tasks;
`amend-message`: the `amend` sub-task composes **zero** step lines, only the trailer). Under
`true` only, additionally the `author-commit` step (0 `commit:<sub>#` writes in the sub-task text
of every workflow but one — see `(R8, F-1)`); under `false` it stays (6–11 `commit:<sub>` mentions per text).
That is the set the design names (`design/findings-channel.md` §6) and it matches the source census
above. **One commit-doc write survives under `true` because it is a literal in step prose, not a
catalog ref** — `(R8, F-1)`.

---

## 2. The (door, cell) table

Route kind: **M**echanical / **H**uman / **I**nformational / **none**. Cells 1–8 are the scope's
*for this run* cells; C1–C5 are the numbered axis's five. Rows whose argv differs only by workflow
or door are grouped, with the count.

| # | door | cell | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `milestone add-task` | 1 | `milestone add-task sweep-all "s <W>" --workflow <W>` × **39** (every shipped workflow), two rigs | 0 ×78 | none | none | `added task:s-<W> to milestone:sweep-all` | **DEFECT `(R8, F-3)`** — the 15 verb-routed and the 4 `creates-task: false` workflows are accepted too |
| 2 | `milestone execute` | 1 | `milestone execute sweep-all`, `squash=true` and `=false` | 0 ×2 | none | M | 39 `Spawn:` lines each, stderr 0 bytes | matches contract |
| 3 | `workflow` | 1 · `squash=true` | the 39 `Spawn:` lines verbatim (`cd <abs>/.jigc/worktrees/s-<W> && jigc workflow <W> --task s-<W>`) | 0 ×39 | none | M | `^Run:.*jigc task finalize` = **0/39**; the sub-task trailer naming `` `jigc milestone finalize sweep-all` is its only commit boundary `` = **39/39**; `jigc task finalize` anywhere in the body = **2** (both the *never* phrasing, quoted in §3.1); `--approve` = 0; `task amend` = 0; `commit:<sub>#` writes = **1 workflow** (`implement-from-spec`) | matches the S2 rule; **DEFECT `(R8, F-1)`** on the one surviving write; **DEFECT `(R8, F-2)`** on the reason the *never* phrasing gives |
| 4 | `workflow` | 1 · `squash=false` | the same 39 lines in a second rig | 0 ×39 | none | M | `Run: … task finalize` = 0/39; trailer 39/39; the author step present in **22** texts (`finalize renders the commit doc` in 13) and absent from the 12 `migrate-*`, `amend`, and the four `creates-task: false` (17) | matches contract (the docs-only author is the declared bound) |
| 5 | `workflow` | 1 control | `workflow <W> --preview` × 39 in a third rig | 0 ×20 · 1 ×19 | `workflow.verb-routed` ×15; un-coded mints-nothing refusal ×4 | M | the 20 composable ones: 19 carry exactly one `Run: jigc task finalize` (`fix-task` carries none by design); `task list` unchanged | matches contract — **top-level keeps the steps** |
| 6 | `migrate` | 1 control | `migrate legacy/old.md --as adr` (tracked source) | 0 | none | M | `task minted: migrate-adr-legacy-old-…`; `Run: … task finalize` = 1, `--approve` = 1; the source body is embedded | ~~matches contract~~ **DEMOTED (reconciler): no repro block carries this row's asserted counts — not driven on the driver's record.** Re-driven by the reconciler, §R.4 `RD-6`: reproduces (the `migrate-adr` sub-task has 0 / 0) |
| 7 | `task amend` | 1 control | `task amend` | 0 | none | M | `task minted: amend-<sha>`; 101 lines, `Run: … task finalize` = 1, 8 `commit:<id>#` writes | ~~matches contract~~ **DEMOTED (reconciler): no repro block anywhere in the file — not driven on the driver's record.** Re-driven by the reconciler, §R.4 `RD-7`: reproduces, with 96 lines where the driver read 101 (the `amend` sub-task composes 0 step lines) |
| 8 | `workflow` + `task bind` + `start` + `doc set-field` + `doc show` + `milestone join` + `milestone finalize` | 1 | an `implement-from-spec` sub-task under `squash=true`, its text followed line by line | 0 ×7 | none | M | the text tells the agent to write `commit:impl-sub#implements` *"so the landed record links back to what it built"*; the join lands; `greeting-spec` in the landed message: 0, in the landed diff: 0 | **DEFECT `(R8, F-1)`** |
| 9 | `task finalize` | 1 | `task finalize code-sub` from the sub-task's worktree | **3** | `finalize.milestone-sub-task` | M (`jigc milestone finalize fan-run`) | worktree `HEAD` unmoved, `A  greet.rs` still staged | the refusal matches contract; it falsifies the step sentence — **DEFECT `(R8, F-2)`** |
| 10 | `workflow` → `doc create` · `doc set-slot` · `doc set-field` · `doc add-item` · `doc list` · `doc show` · `task validate` → `milestone join` → `milestone finalize` | 2 · `squash=true` · code first | one `dev-task` (stages `greet.rs`) and one `report-inconsistency` sub-task, each text followed | 0 … 0 (validate: 3, row 20) | none | M | `finalized … Finalize milestone fan-run (2 sub-tasks)`, 4 files, one commit | matches contract |
| 11 | same | 2 · `squash=true` · docs first | same, docs sub-task completed first | 0 … 0 | none | M | patch sha of the landed code + doc and sha of the message **identical** to row 10 | matches contract (order-invariant) |
| 12 | same | 2 · `squash=false` · code first | same, each sub-task authoring the commit doc its text now asks for | 0 … 0 (validate: 0) | none | M | two commits: `feat: add a greeting helper` (the code sub-task's own doc rendered) + the aggregate | matches contract |
| 13 | same | 2 · `squash=false` · docs first | same, reversed | 0 … 0 | none | M | patch sha and message sha identical to row 12 | matches contract |
| 14 | `config fork` · `config insert-step` ×3 · `config replace-step` ×2 | 3 (fixture) | see §3.3 | 0 ×6 | none | none | six deltas in `.jigc/config/manifest.yaml`, six native steps under `.jigc/config/steps/` | matches contract |
| 15 | `milestone execute` | 3 | `milestone execute delta-run` and `--format json`, with a project step inserted into `milestone-execution` | 0 ×2 | none | M | the inserted step's mark is in the text and in JSON `text`; keys `['task','text']`, `task: null`; 5 `Spawn:` lines | matches contract |
| 16 | `workflow` | 3 · first entry | 5 `Spawn:` lines verbatim (`dev-task`, `single-task`, `quick-fix`, `report-inconsistency`, `plan`) | 0 ×5 | none | M | the inserted carrier, the wrapper and the shadowed `finalize` body are **absent** in all five; the plain inserted step is **present**; `Run: … task finalize` = 0 | matches contract |
| 17 | `workflow` | 3 · re-entry | `workflow <W> --task <sub>` from each worktree | 0 ×5 | none | M | byte-identical to the first entry, 5/5 | matches contract |
| 18 | `start` | 3 · resume | `start --task <sub>` from each worktree, and `--format json` | 0 ×10 | none | M | byte-identical to the first entry, 5/5; JSON `text` carries the same omissions | matches contract |
| 19 | `start` | 3 control | `start --workflow <W> "top <W>"` ×5, then `start --task top-<W>` ×5 | 0 ×10 | none | M | top-level keeps the inserted carrier (`dev-task`: 2 `Run: … task finalize` lines), the wrapper and the shadow; the resume carries the same marks as the mint | matches contract |
| 20 | `workflow` · `start` · `milestone execute` | 3 · workflow whole-file shadow | a hand-written `.jigc/config/workflows/park-idea.yaml` including a project wrapper of `step:finalize`: `workflow park-idea --preview`; the `Spawn:` line; `start --task`; `--format json`; `--format human` | 0 ×6 | none | M | top-level: wrapper + shadow + 1 `Run: … task finalize`; sub-task at all four forms: plain step present, wrapper absent, 0 finalize lines, trailer present; JSON keys `['task','text']`, no trailer line inside `text` | matches contract |
| 21 | `workflow` · `milestone execute` | 3 · malformed shadow | the same file with a prose line in the workflow body | **1** ×2 | `workflow-refs.body-include-only` | H | *"workflow body line is not an `{{ include: step:<id> }}`, blank, or comment"*, `at: line 12` | matches contract (blocks loudly, at both doors) |
| 22 | `config set` → `milestone join` → `milestone finalize` | 4 | compose under `true`; then `config set finalize.fan-out.squash false`; `milestone join`; `milestone finalize` | 0 / 0 / **3** | `schema-conformance.field-value-conformant` + `schema-conformance.required-slot-present`, both at `commit:code-sub` | M ×2 | the block names only the code-carrying sub-task; `HEAD` unmoved | matches the declared bound (see O-1 on the code count) |
| 23 | `workflow` · `start` | 4 | re-compose through both `Spawn:` lines; `start --task code-sub` | 0 ×3 | none | M | lines 31 → 76 and 60 → 105; `commit:<sub>#` writes 0 → 5; *"finalize renders the commit doc"* present; `start --task` byte-identical to the re-spawn | matches the declared bound |
| 24 | `doc set-field` · `doc set-slot` → `milestone finalize` | 4 | the block's two routes run as printed, then `milestone finalize` | 0 / 0 / 0 | none | M | lands `feat: add greet` + the aggregate | matches contract |
| 25 | `start` | 5 | bare `start` and `start --format json` on `committed-singletons` | 0 ×2 | none | M | `report-inconsistency` listed once with its `when:`; the three hidden ids: 0 mentions on both arms; 13 catalog entries | matches contract |
| 26 | `start` | 5 | `start --workflow router` | 0 | none | M | the same one line; three hidden ids: 0 | matches contract |
| 27 | `start` | 5 | `start --workflow <id> "<intent>"` for `report-inconsistency`, `report-jigc-feedback`, `triage-inconsistency`, `triage-jigc-feedback` | 0 ×4 | none | M | `task minted:` on all four; each top-level text carries one `Run: jigc task finalize <id>` | matches contract for the two `*-inconsistency` ids (§3.5). **DEMOTED in part (reconciler): the two `*-jigc-feedback` mints have no repro block.** Re-driven, §R.4 `RD-27`: all four reproduce |
| 28 | `workflow` | 5 | `workflow <id> --preview` for the four | 0 ×4 | none | I | each composes; `task list` unchanged | matches contract for `triage-inconsistency` (§3.5). **DEMOTED in part (reconciler): the other three previews have no repro block.** Re-driven, §R.4 `RD-28`: all four reproduce |
| 29 | `describe` | 5 / C5 | `describe --workflows` and `--format json` | 0 ×2 | none | I | 39 = **26** hidden + **13** visible; *"It is hidden from the router catalog"* printed 26 times; the three hidden findings workflows each carry a non-empty `router_hidden` reason naming the by-name argv; `report-inconsistency`: `router_hidden: null`; the 13 visible ids equal the orientation catalog | matches contract |
| 30 | `start` | 6 | `start --workflow triage-inconsistency "inconsistency:cache-doc-drift: reconciled by the cache rewrite"` and the plain-words control | 0 ×2 | none | none | minted `inconsistencycache-doc-drift-reconciled` (colon dropped, two words fused, no warning) vs `cache-doc-drift-reconciled`; the step says *"an address-shaped intent mangles it"* | matches contract (the step states what the binary does) |
| 31 | `start` → `doc …` → `task validate` → `task finalize` | 7 / C4 · top-level report | `report-inconsistency` minted by name, a foreign path staged, the text followed | 0 … 0 | advisory `file-state.staged-copy` | M | `what's-left:` names *"the path scope that leaves every other staged path staged"*; the step says *"commits path-scoped"*; the commit has **1 file**; `foreign.txt` still `A ` | matches contract |
| 32 | `doc list` · `doc set-field` · `doc set-slot` · `task finalize` | 7 · top-level triage | the plain-words triage task followed | 0 ×4 | advisory | M | 1 file changed; `foreign.txt` left staged and named in `left-out` | matches contract |
| 33 | `workflow` → `milestone join` → `milestone finalize` | 7 · sub-task | one `report-inconsistency` and one `triage-inconsistency` sub-task; `z.rs` staged in the report worktree | 0 … 0 | none | M | both `what's-left:` lines name *"the carryover gate"* and not the path scope; 0 path-scope claims in the bodies; the join commits `z.rs` (`report-sub: 2 docs, 1 code file`) | matches contract — **E1 holds closed** |
| 34 | `task validate` | 8 · `true` · code-carrying | `task validate code-sub` and `--format json` | **3** / 3 | `schema-conformance.field-value-conformant` (`commit:code-sub#header/type`) · `schema-conformance.required-slot-present` (`commit:code-sub#summary`) | M ×2 (`jigc doc set-field …#header/type`, `jigc doc set-slot …#summary`) | keys `['findings','schema_version']`; the composed text has 0 `commit:<sub>#` writes | **HOLDS AS DECLARED** |
| 35 | `doc set-field` · `doc set-slot` · `task validate` → `milestone join` · `milestone finalize` | 8 · where the route leads | both routes run as printed, then validate, then the join | 0 / 0 / **0** / 0 / 0 | none | M | *"no findings — the task validates clean"*; the aggregate lands; the authored subject appears **0** times in the landed history | **HOLDS AS DECLARED** — the route leads to a doc no boundary reads; nothing breaks |
| 36 | `task validate` | 8 · `true` · docs-only | `task validate docs-sub` | **3** | the same two codes at `commit:docs-sub`, plus advisory `file-state.staged-copy` | M ×2 | — | **HOLDS AS DECLARED** |
| 37 | `task validate` → `milestone join` → `milestone finalize` | 8 · `false` · both kinds, commit docs unfilled | `task validate code-sub`; `task validate docs-sub`; `milestone join`; `milestone finalize` | 3 / 3 / 0 / **3** | the two codes at `commit:<sub>`; the finalize block names `commit:code-sub` only | M | after filling **only** the code sub-task's doc, `milestone finalize` lands at exit 0 with the docs-only one still unfilled | **HOLDS AS DECLARED** (code-carrying: gated and asked for; docs-only: asked for, not read) |
| 38 | `start` | 8 · the bound at orientation | bare `start` and `--format json` with five live `squash=true` sub-tasks | 0 ×2 | `schema-conformance.field-value-conformant` + `required-slot-present` per sub-task | M | `findings: 2 blocking` on every sub-task, on the text arm and in `tasks[].findings` | **DEFECT `(R8, F-4)`** — wider than the bound's sentence |
| 39 | `start` · `workflow` · `doc …` | C1 | the mechanical sweep: 7959 lines (78 sub-task texts + 4 top-level findings texts), every distinct `jigc <a> [<b>]` form driven `--help` | 0 | none | — | 70 distinct forms: **62 real** (23 distinct clap leaves + 39 `workflow <id>` forms), 7 prose fragments (`jigc itself accepts`, `jigc version in`, `jigc did`, `jigc never auto-migrates`, `jigc has adopted`, `jigc checks only`, `jigc can manage`), 1 group stem (`jigc doc` in prose) | ~~matches contract~~ **DEMOTED (reconciler): no repro block — not driven on the driver's record.** Re-driven by the reconciler, §R.4 `RD-39`: the counts reproduce exactly |
| 40 | `start` | C2 | `start --task inconsistencycache-doc-drift-reconciled` (a top-level findings task), the `resume:` line run | 0 | none | M | one `resume:` line: *"re-composes this workflow if context is lost"* | ~~matches contract~~ **DEMOTED (reconciler): no repro block — not driven on the driver's record.** Re-driven by the reconciler on the plain-words sibling task, §R.4 `RD-40`: reproduces |
| 41 | `workflow` | C2 · M51 `A6-1` | `workflow dev-task --task sub-dev-task` from its own worktree · a sibling worktree · the root · `docs/deep` | 0 / 0 / 1 / 1 | none (flattened, routed) | M | *"run it here"* · *"this checkout is not that worktree — run it from …"* · the base-pin refusal, byte-identical root vs subdir, routed at an absolute `cd` | matches contract — **still CLOSED** |
| 42 | `start` · `workflow` | C1/C5 · M51 `A6-2` | the re-derived 15-member door set × {`start --workflow <X>`, `start --workflow <X> "an intent"`, `workflow <X> --preview`} | 1 ×45 | `workflow.verb-routed` ×45 | M | `task list` byte-identical before/after; JSON arm: findings envelope keyed `workflow:<id>` | matches contract — **still CLOSED** over the three forms (but see `(R8, F-3)` for a fourth) |
| 43 | `start` · `task bind` | C1 · M51 `A6-3` | `start --workflow implement-from-spec "bind probe"` over a zero-spec corpus; `task bind spec spec:nope bind-probe --format json`; `task bind spec nosuch:thing bind-probe --format json` | 0 / 1 / 1 | `store.not-found` (`spec:nope`) · `store.unknown-type` (`nosuch`) | M | both empty-case clauses stand; keys `['findings','schema_version']`, stdout 0 bytes | matches contract — **still CLOSED** |
| 44 | `start` · `task validate` | C3 · `(6, D-1)` | mint; `git bisect start`; `start`; `start --format json`; `task validate posture-parity` | 0 / 0 / 1 | `repo.operation-in-progress` | H | orientation now reports it: text 1 hit, `findings: 3 blocking, 1 advisory`; wire `tasks[0].findings[0].code = repo.operation-in-progress` | **CLOSED on rc.24** |
| 45 | `start` · `task finalize` | C1 · `(6, D-2)` | `start --workflow fix-task "fix the thing"` → fill → `task finalize fix-the-thing` | 0 / 0 | none | none | line 25 forbids `jigc task finalize`; it lands `fix: fix the thing`, 1 file | **STILL-OPEN (expected)** |
| 46 | `start` · `describe` | C5 · `(6, D-3)` | `start` → `Preview:` footer; `start --workflow milestone-execution`; `start --workflow router` | 0 / 1 / 0 | `workflow.verb-routed` | I vs M | footer unchanged: *"a workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly"* | **STILL-OPEN (expected)**. **DEMOTED in part (reconciler): the `describe` door of this row has no argv and no repro block (§4's `(R8, F-3)` cites it as *driven at row 46*).** Re-driven, §R.4 `RD-46` |
| 47 | `milestone execute` · `milestone finalize` | C1 · `(6, D-4)` | `milestone create "Empty probe"`; `milestone execute empty-probe`; `milestone finalize empty-probe` | 0 / 0 / 3 | `milestone.zero-contribution` | M | 0 `Spawn:` lines, stderr 0 bytes, no sentence naming the empty case | **STILL-OPEN (expected)** |
| 48 | `validate` (the hook's reader) | `(2, N-1)` = `(6, A6-R1)` | cell 1: `git rm docs/roadmap.md` + commit, then an unrelated commit; cell 2: `git mv docs/roadmap.md docs/plan.md` + commit | 0 / 0 / 0 ; **1** | — | — | cell 1: the unrelated commit's stderr is empty; cell 2: *"out-of-band managed-doc rename staged in this commit … (commit blocked)"* | **CLOSED on rc.24** (carried onto this axis) |
| 49 | `describe` · `start` | C5 · `(6, L-2)` | a manufactured manifest-less pack with a `creates-task: false` workflow and no `suppressed:` | 0 ×3 | none | I | loads clean; `router_hidden: null`; orientation lists it 0 times | lead **unchanged** (declared) |
| 50 | `config set` | `(6, L-4)` | `config set invocation-log true` from inside a fan-out worktree | 0 | none | none | ack *"written to `.jigc/config/`"*; the write is in the **main** checkout's manifest; the worktree's own `.jigc/config/` holds only `packs.yaml` | lead **unchanged** (declared convention) |
| 51 | `start` | C3 · `(6, L-5)` a | `chmod 000` a live task's `docs`; `start`; `start --format json` | 0 ×2 | none | M | `findings: unknown — … at "<absolute host path>"…`; JSON `findings: null` + `findings_unavailable` carrying the same absolute | lead **unchanged** |
| 52 | `migrate` | `(6, L-5)` b | `migrate legacy/old.md --as adr` with 10 tasks live; then `start --workflow quick-fix "another one"` | 0 / 0 | none | M | migrate: 0 `also open:` lines; the next mint: *"also open: 11 other tasks were already open"* | lead **unchanged** (declared) |
| 53 | `doc show` | `(6, L-5)` c | `doc show padding` and `--format json` | 1 ×2 | none | M (`jigc describe`) | no `blocking · <code>` prefix; JSON `{"error": …}` | lead **unchanged** (declared flatten) |
| 54 | `start` · `describe` · `workflow` · `milestone execute` | `(6, L-6)` | each with `JIGC_PACK_DIR` pointed at an empty directory | 1 ×4 | `pack.resource-missing` ×4 | — | stdout 0 bytes at all four; the finding names the missing `config/defaults` / `config/knobs` and the searched layer | **reached this run** — matches contract |

**54 rows driven · 0 marked driven from a source read.** *(The driver's count. Reconciled: **50 rows
stand as driven on the driver's own repro blocks; 4 are demoted whole** — rows 6, 7, 39, 40 — **and 3
in part** — rows 27, 28, 46. None was marked from a source read; each lacked a repro block. All seven
were re-driven by the reconciler and reproduce — §R.4. One count in a standing row does not reproduce:
row 5's `0 ×20 · 1 ×19` is `0 ×21 · 1 ×18` — §R.4 `RD-5`.)*

---

## 3. Repro blocks

Every block: setup · argv · observed. `$J` is the installed binary.

### 3.1 Cell 1 — sub-task composition, all 39 workflows, both knob values (rows 1–5)

```
# rig: fresh ×2 (one per knob value); a third fresh rig for the top-level control
jigc config set finalize.fan-out.squash <true|false>          # 0
jigc milestone create "Sweep all"                              # 0
for W in <all 39 shipped workflow ids>:
  jigc milestone add-task sweep-all "s $W" --workflow $W       # 0 ×39  — including the 15 verb-routed
                                                               #          and the 4 creates-task:false ones
jigc milestone provision sweep-all                             # 0
jigc milestone execute sweep-all                               # 0 · 39 `Spawn:` lines · stderr 0 bytes
for each Spawn line:  ( cd / && sh -c "<line>" )               # 0 ×39 · stderr 0 bytes ×39
```

Per composed text, the body with the `resume:` / `what's-left:` / `task scope:` trailer lines
excluded was counted for: `^Run:.*jigc task finalize` · `task finalize` · `task amend` · `--approve`
· `commit:<sub>` · `commit doc|commit message` · `finalize renders|renders the commit`; and the
trailer for `` this task is a sub-task of milestone `sweep-all`, whose `jigc milestone finalize
sweep-all` is its only commit boundary ``.

```
squash=true    Run:task-finalize 0/39   trailer 39/39   task amend 0   --approve 0
               `task finalize` in a body: 2
                 s-fix-task:23   "Never `git commit` and never `jigc task finalize` here. The milestone's finalize"
                                 (step: methodology fix-finding)
                 s-sub-task:65   "this worktree's staged index, but never `git commit` and never `jigc task finalize`"
                                 (step: dev sub-task-commit)
               `commit:<sub>` in a body: 1 workflow
                 s-implement-from-spec:45  "jigc doc set-field commit:s-implement-from-spec#implements --value spec:<slug> --task …"
                 s-implement-from-spec:51  "jigc doc show commit:s-implement-from-spec --task s-implement-from-spec"
                                 (step: dev locate-from-spec — a literal in step prose)
               "commit doc" prose: s-implement-from-spec:53, s-migrate-arch-doc:9
                 ("the migration commit doc is auto-provisioned for you, so never `doc author commit`")
squash=false   Run:task-finalize 0/39   trailer 39/39
               author step present (6–11 `commit:<sub>` mentions) in 22 texts; absent in the 12 migrate-*,
               amend, increment, ingest-existing, milestone-execution, router
control        `jigc workflow <W> --preview` ×39:
               20 exit 0 — 19 carry exactly one `Run: jigc task finalize`, fix-task none
               15 exit 1 `workflow.verb-routed` · 4 exit 1 (mints nothing, no preview)
               `jigc task list` unchanged before/after
```

### 3.2 Cells 2 and 8 — to the join, both knob values, both completion orders (rows 9–13, 34–37)

```
# rig: fresh ×4 (rows 10–13), fresh ×2 (rows 34–35, 37)
jigc config set finalize.fan-out.squash <bool>
jigc milestone create "Fan run"
jigc milestone add-task fan-run "code sub" --workflow dev-task
jigc milestone add-task fan-run "docs sub" --workflow report-inconsistency
jigc milestone provision fan-run ; jigc milestone execute fan-run ; <both Spawn lines verbatim>   # 0 …
#   true : code-sub 31 lines, docs-sub 60 lines, 0 commit writes, trailer present
#   false: code-sub 76 lines, docs-sub 105 lines, 5 commit writes each, trailer present

# code sub-task, in $REPO/.jigc/worktrees/code-sub (git rev-parse --show-toplevel checked)
printf '…' > greet.rs ; git add greet.rs
#   false only, as the text asks:
jigc doc set-field commit:code-sub#type --value feat --task code-sub                          # 0
echo 'add a greeting helper' | jigc doc set-slot commit:code-sub#summary --from-file - --task code-sub   # 0
# docs sub-task, in $REPO/.jigc/worktrees/docs-sub
jigc doc create inconsistency --title "Cache doc drift" --task docs-sub                       # 0
jigc doc set-slot inconsistency:cache-doc-drift#description --from-file - --task docs-sub      # 0
jigc doc set-field inconsistency:cache-doc-drift#meta/kind --value code-doc --task docs-sub    # 0
jigc doc add-item inconsistency:cache-doc-drift#sides --title "src/cache.rs" --slug code --task docs-sub   # 0
jigc doc add-item inconsistency:cache-doc-drift#sides --title "docs/cache.md" --slug doc --task docs-sub   # 0
jigc doc set-slot inconsistency:cache-doc-drift#sides/<side>/says --from-file - --task docs-sub  ×2        # 0
jigc doc list inconsistency --task docs-sub      # 0 — inconsistency:cache-doc-drift … managed
jigc doc show inconsistency:cache-doc-drift --task docs-sub                                   # 0

jigc milestone join fan-run                      # 0 — "joined milestone:fan-run — 3 doc(s) merged", HEAD unmoved
jigc milestone finalize fan-run                  # 0
```

Observed, `squash=true` (rows 10, 11):

```
finalized <sha> — Finalize milestone fan-run (2 sub-tasks)
  added .jigc/config/manifest.yaml · promoted docs/inconsistencies/cache-doc-drift.md ·
  modified docs/milestone-records/fan-run.md · added greet.rs · 4 files committed
  sub-tasks: code-sub: 1 doc, 1 code file · docs-sub: 2 docs
code-first vs docs-first:  sha of `git show HEAD -- greet.rs docs/inconsistencies`  5f7709772833 = 5f7709772833
                           sha of the commit message                                76162a2a2c6e = 76162a2a2c6e
```

Observed, `squash=false` (rows 12, 13): two commits, `feat: add a greeting helper` then the
aggregate; `sub-tasks: code-sub: 1 doc, 1 code file, committed as <sha> · docs-sub: 2 docs`; patch
sha `1d21beb2cd89` and message sha `76162a2a2c6e` identical across the two orders.

**The declared bound, `squash=true` (rows 34–36):**

```
jigc task validate code-sub                      # 3
> blocking · schema-conformance.field-value-conformant — `commit:code-sub`: field `type` in section
>   `header`: "" is not a member of enum "type" (allowed: feat, fix, …)
>   at: commit:code-sub#header/type
>   route: `jigc doc set-field commit:code-sub#header/type --task code-sub --value <value>` to correct the value
> blocking · schema-conformance.required-slot-present — `commit:code-sub`: required slot in section `summary` is empty
>   at: commit:code-sub#summary · line 9
>   route: `jigc doc set-slot commit:code-sub#summary --task code-sub --from-file -` to fill the empty slot
jigc task validate code-sub --format json        # 3 · keys ['findings','schema_version'] · the same two codes
jigc task validate docs-sub                      # 3 · the same two codes at commit:docs-sub (+ advisory file-state.staged-copy)

# where the route leads — both run as printed:
jigc doc set-field commit:code-sub#header/type --task code-sub --value feat                # 0
echo 'a subject nobody reads' | jigc doc set-slot commit:code-sub#summary --task code-sub --from-file -   # 0
jigc task validate code-sub                      # 0 — "no findings — the task validates clean"
jigc milestone join fan-run ; jigc milestone finalize fan-run                               # 0 / 0
git log --format=%B | grep -c 'a subject nobody reads'                                      # 0
```

The join also lands with **both** commit docs unfilled (rows 10, 11: neither sub-task authored one).
The bound is exactly as `completions/artifacts/M55/VERDICT.md` → *Declared bounds* states it.

**`squash=false` (row 37):**

```
jigc task validate code-sub                      # 3 · the two codes at commit:code-sub
jigc task validate docs-sub                      # 3 · the two codes at commit:docs-sub
jigc milestone join fan-run                      # 0
jigc milestone finalize fan-run                  # 3 · blocks on commit:code-sub ONLY (two findings); HEAD unmoved
jigc doc set-field commit:code-sub#header/type --task code-sub --value feat                # 0
echo 'add greet' | jigc doc set-slot commit:code-sub#summary --task code-sub --from-file -  # 0
jigc milestone finalize fan-run                  # 0 · "feat: add greet" + the aggregate; commit:docs-sub never filled
```

**The refused per-task door (row 9):**

```
# in $REPO/.jigc/worktrees/code-sub, greet.rs staged
jigc task finalize code-sub                      # 3
> blocking · finalize.milestone-sub-task — task `code-sub` is a sub-task of milestone `fan-run` — the
>   parent milestone's finalize is the only commit boundary; a per-sub-task finalize would land a
>   commit outside it and strand this sub-task's work
>   route: `jigc milestone finalize fan-run` — …
git rev-parse --short HEAD   → the base pin, unmoved ;  git status --short → "A  greet.rs"
```

### 3.3 Cell 3 — project-layer interference at every compose door (rows 14–21)

```
# rig: fresh. Source step files written under $RIG/src.<x>/ (outside the repo):
#   proj-gate.yaml : "INSERT-MARK-GATE …\n\n{{ cli.finalize-task }}"          (a native step CARRYING the ref)
#   wrap-fin.yaml  : "WRAP-MARK-FIN …\n\n{{ include: step:finalize }}"        (a project WRAPPER of a member)
#   proj-note.yaml : "INSERT-MARK-PLAIN …"                                    (a non-member)
#   lit-fin.yaml   : "LIT-MARK now run `jigc task finalize {{task.id}}` to commit."   (a LITERAL, no ref)
#   exec-note.yaml : "EXEC-MARK …"
jigc config fork workflow:quick-fix#finalize                                  # 0 → .jigc/config/steps/finalize.yaml
#   the stated exception: that tracked project-layer shadow hand-edited to prepend "SHADOW-MARK-FIN …"
#   before its `{{ cli.finalize-task }}`
jigc config insert-step --workflow dev-task --before finalize <src>/proj-gate.yaml            # 0
jigc config replace-step 'workflow:single-task#finalize' <src>/wrap-fin.yaml                  # 0
jigc config insert-step --workflow report-inconsistency --before finalize-doc-only <src>/proj-note.yaml   # 0
jigc config replace-step 'workflow:plan#finalize' <src>/lit-fin.yaml                          # 0
jigc config insert-step --workflow milestone-execution --before provision-worktrees <src>/exec-note.yaml  # 0

jigc milestone create "Delta run" ; add-task ×5 (dev-task, single-task, quick-fix, report-inconsistency, plan)
jigc milestone provision delta-run
jigc milestone execute delta-run                 # 0 · EXEC-MARK ×1 · 5 Spawn lines ; --format json: keys ['task','text'], task null, EXEC-MARK ×1
```

Observed (marks counted per composed text):

```
door                                   sub           SHADOW GATE WRAP PLAIN LIT  Run:task-finalize
first entry (Spawn verbatim)           sub-dev-task     0     0    0    0    0        0
                                       sub-single-task  0     0    0    0    0        0
                                       sub-quick-fix    0     0    0    0    0        0
                                       sub-report-…     0     0    0    1    0        0
                                       sub-plan         0     0    0    0    1        0   (body names `jigc task finalize sub-plan` once — the project's own literal)
re-entry  `workflow <W> --task <sub>`  ×5  → byte-identical to the first entry, 5/5
resume    `start --task <sub>`         ×5  → byte-identical to the first entry, 5/5 ; --format json `text`: same counts
control   `start --workflow <W> "top <W>"`:
          top-dev-task     SHADOW 1 GATE 1 · 2 Run:task-finalize lines
          top-single-task  SHADOW 1 WRAP 1 · 1
          top-quick-fix    SHADOW 1        · 1
          top-report-…     PLAIN 1         · 1
          top-plan         LIT 1           · 0 Run lines (the literal is prose)
          `start --task top-<W>` ×5 → the same marks as the mint
```

Workflow whole-file shadow (row 20) — `.jigc/config/workflows/park-idea.yaml`, hand-written (the
stated exception), body `author-idea` · `author-commit` · `proj-note` · `wrap-fin`:

```
jigc workflow park-idea --preview                # 0 · PLAIN 1 · WRAP 1 · SHADOW 1 · 1 Run:task-finalize
jigc milestone create "Shadow run" ; add-task shadow-run "park sub" --workflow park-idea ; provision
jigc milestone execute shadow-run                # 0
<the Spawn line verbatim>                        # 0 · PLAIN 1 · WRAP 0 · SHADOW 0 · 0 Run:task-finalize · 0 commit writes · trailer 1
jigc start --task park-sub                       # 0 · byte-identical
jigc workflow park-idea --task park-sub --format json   # 0 · keys ['task','text'] · task park-sub · PLAIN 1 WRAP 0 · no trailer line inside `text`
jigc workflow park-idea --task park-sub --format human  # 0 · PLAIN 1 · WRAP 0 · 0 `jigc task finalize` · names `jigc milestone finalize shadow-run`
```

Malformed shadow (row 21) — the same file with one prose line in the workflow body:

```
jigc workflow park-idea --preview                # 1
jigc milestone execute shadow-run                # 1
> blocking · workflow-refs.body-include-only — workflow body line is not an `{{ include: step:<id> }}`,
>   blank, or comment: `WF-SHADOW-MARK a project line before the commit boundary.`
>   at: line 12
>   route: fix the workflow/step/catalog definition the message names …, then re-run
```

### 3.4 Cell 4 — the knob flipped between compose and join (rows 22–24)

```
# rig: fresh. squash=true; the fan-run milestone of §3.2 composed (31 / 60 lines, 0 commit writes).
# code-sub stages greet.rs; docs-sub files inconsistency:cache-doc-drift. Neither authors a commit doc.
jigc config set finalize.fan-out.squash false    # 0
jigc milestone join fan-run                      # 0 — 3 docs merged
jigc milestone finalize fan-run                  # 3
> blocking · schema-conformance.field-value-conformant — `commit:code-sub`: field `type` … at: commit:code-sub#header/type
>   route: `jigc doc set-field commit:code-sub#header/type --task code-sub --value <value>` to correct the value
> blocking · schema-conformance.required-slot-present — `commit:code-sub`: required slot in section `summary` is empty
>   route: `jigc doc set-slot commit:code-sub#summary --task code-sub --from-file -` to fill the empty slot
> advisory · file-state.staged-copy — …
<both Spawn lines verbatim, again>               # 0 ×2 · code-sub 31 → 76 lines · docs-sub 60 → 105 ·
                                                 #        5 commit writes each · "finalize renders the commit doc" ×1 · 0 Run:task-finalize
jigc start --task code-sub   (from its worktree) # 0 · byte-identical to the re-spawn
<the two routes, as printed> ; jigc milestone finalize fan-run       # 0 / 0 / 0 — "feat: add greet" + the aggregate
```

### 3.5 Cells 5, 6, 7 — the four findings workflows (rows 25–33)

```
# rig: committed-singletons
jigc start                                       # 0
>   - report-inconsistency — code and a doc, or two docs, disagree and the disagreement is worth a
>     record until it is reconciled
#   mentions: report-jigc-feedback 0 · triage-jigc-feedback 0 · triage-inconsistency 0 ; 13 catalog entries
jigc start --format json                         # 0 · keys ['header','next_steps','schema_version','state','workflows'] · same 1/0/0/0
jigc start --workflow router                     # 0 · same 1/0/0/0
jigc describe --workflows                        # 0 · "It is hidden from the router catalog" ×26
jigc describe --workflows --format json          # 0 · 39 workflow rows = 26 hidden + 13 visible
>   report-jigc-feedback  router_hidden: "invoked by name (`jigc start --workflow report-jigc-feedback "<what>"`) — jigc's own
>                         agents report what they hit about jigc, and a catalog entry would invite reports into a repository
>                         the maintainers never see; opened when a feedback interface exists"
>   triage-inconsistency  router_hidden: "invoked by name with the record named in plain words (`jigc start --workflow
>                         triage-inconsistency "<record in plain words>"`) — triage is a maintainer's act, not a catalog entry"
>   triage-jigc-feedback  router_hidden: "invoked by name with the finding named in plain words (…) — triage is a maintainer's act, …"
>   report-inconsistency  router_hidden: null
```

Top-level report, followed with a foreign path staged (row 31):

```
jigc start --workflow report-inconsistency "cache doc drift"      # 0 — task minted: cache-doc-drift
> what's-left: `jigc task validate cache-doc-drift` — previews part of the finalize gate: … the path scope
>   that leaves every other staged path staged, …
> (step text) "Land this task's docs as exactly one commit. This finalize commits path-scoped: …"
printf 'foreign\n' > foreign.txt ; git add foreign.txt
<create · set-slot · set-field · add-item ×2 · set-slot ×2 · the commit doc>                   # 0 …
jigc task validate cache-doc-drift               # 0 (advisory file-state.staged-copy)
jigc task finalize cache-doc-drift               # 0
> finalize — about to commit only this task's docs, path-scoped; leaving out:
>   left-out (…a staged path stays staged for the task it belongs to):  foreign.txt
> finalized <sha> — docs: file the cache drift · promoted docs/inconsistencies/cache-doc-drift.md · 1 file committed
git status --short → "A  foreign.txt"
```

The triage intent form (row 30) and the top-level triage (row 32):

```
jigc workflow triage-inconsistency --preview
> Name the record in the intent in plain words — its slug, without the `inconsistency:` prefix and its
> colon. The task id is minted from the intent, so an address-shaped intent mangles it; the address,
> `inconsistency:<slug>`, goes to the `doc` verbs below and never to `jigc start`.
jigc start --workflow triage-inconsistency "inconsistency:cache-doc-drift: reconciled by the cache rewrite"   # 0
> task minted: inconsistencycache-doc-drift-reconciled
jigc start --workflow triage-inconsistency "cache doc drift reconciled by the cache rewrite"                  # 0
> task minted: cache-doc-drift-reconciled

jigc doc set-field inconsistency:cache-doc-drift#meta/status --value resolved --task cache-doc-drift-reconciled   # 0
jigc doc set-slot inconsistency:cache-doc-drift#resolution --from-file - --task cache-doc-drift-reconciled        # 0
jigc task finalize cache-doc-drift-reconciled    # 0 — "docs: triage the cache drift", 1 file changed, foreign.txt left out and still staged
```

The same two workflows as sub-tasks (row 33) — `git restore --staged foreign.txt` first, in `$REPO`:

```
jigc milestone create "Mixed" ; add-task mixed "report sub" --workflow report-inconsistency ;
add-task mixed "triage sub" --workflow triage-inconsistency ; provision ; execute ; <both Spawn lines>
> what's-left (both): … this task's content findings, the carryover gate, the owner-artifact causes …
#   path-scope / "alone" / "one commit" / "stays staged" claims in either body: 0
# report-sub worktree: printf 'z\n' > z.rs ; git add z.rs ; create + fill inconsistency:second-drift
# triage-sub worktree: set-field …#meta/status --value refuted ; set-slot …#resolution
jigc milestone join mixed                        # 0 — 4 docs merged (cache-doc-drift: edited-from-base · from triage-sub)
jigc milestone finalize mixed                    # 0
> promoted docs/inconsistencies/cache-doc-drift.md · promoted docs/inconsistencies/second-drift.md ·
> modified docs/milestone-records/mixed.md · added z.rs · 4 files committed
> sub-tasks: report-sub: 2 docs, 1 code file · triage-sub: 2 docs
```

### 3.6 Baseline re-drives (rows 41–54)

```
# (6, D-3) · rig: fresh
jigc start | grep '^Preview:'
> Preview: `jigc workflow <id> --preview` — read a task-minting workflow's step text without minting a task; a
>   workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly, and mints nothing either
jigc start --workflow milestone-execution        # 1 · workflow.verb-routed
jigc start --workflow router                     # 0

# M51 A6-2 · same rig
for X in <the 15 `suppressed.door` workflows>:
  jigc start --workflow X ; jigc start --workflow X "an intent" ; jigc workflow X --preview
# 45 drives · 45 exit 1 · 45 workflow.verb-routed · `jigc task list` unchanged

# (6, D-4)
jigc milestone create "Empty probe"              # 0
jigc milestone execute empty-probe               # 0 · 0 Spawn lines · stderr 0 bytes · 30 lines, none naming the empty case
jigc milestone finalize empty-probe              # 3 · milestone.zero-contribution

# (6, D-2)
jigc start --workflow fix-task "fix the thing"   # 0
>  :25  Never `git commit` and never `jigc task finalize` here. …
jigc doc set-field commit:fix-the-thing#type --value fix --task fix-the-thing ; … set-slot …#summary ; git add fixed.txt
jigc task finalize fix-the-thing                 # 0 — finalized <sha> — fix: fix the thing · added fixed.txt · 1 file committed

# (6, D-1)
jigc start --workflow single-task "posture parity"   # 0
git bisect start                                     # 0
jigc start                                           # 0
>   findings: 3 blocking, 1 advisory
>   blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a committable state
>     route: conclude it, or abandon it with `git bisect reset`, then re-run this command
jigc start --format json                             # 0 · tasks[0].findings codes:
>   ['repo.operation-in-progress', 'schema-conformance.field-value-conformant',
>    'schema-conformance.required-slot-present', 'changelog-recording.gate-granted-unused']
jigc task validate posture-parity                    # 1 · repo.operation-in-progress

# (2, N-1) = (6, A6-R1) · rig: committed-singletons ×2
jigc validate                                    # 0
git add f1.txt ; git commit -m "unrelated 1"     # 0 · stderr 0 bytes
git rm -q docs/roadmap.md ; git commit -m "delete the roadmap out of band"    # 0 · stderr empty
git add f2.txt ; git commit -m "unrelated 2"     # 0 · stderr empty · 'rename' 0 times
# second rig:
git mv docs/roadmap.md docs/plan.md ; git commit -m "bare git mv of a placement doc"   # 1
> jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity
>   tracking; use `jigc rename` instead (commit blocked).

# M51 A6-1 · rig: the §3.3 rig (HEAD off the pin)
jigc workflow dev-task --task sub-dev-task       from its own worktree   # 0 — "…where its work happens — run it here"
                                                 from a sibling worktree # 0 — "this checkout is not that worktree — run it from this sub-task's own worktree at `.jigc/worktrees/sub-dev-task`"
                                                 from $REPO              # 1 — "task `sub-dev-task` is pinned to base <a> but you're on <b> … run this from that worktree — `cd <tmp>/…/repo/.jigc/worktrees/sub-dev-task`, then re-run this command"
                                                 from $REPO/docs/deep    # 1 — byte-identical to the root's

# M51 A6-3
jigc start --workflow implement-from-spec "bind probe"   # 0 — "(nothing is listed until a `spec` is committed; with none there is nothing to bind here …)"
jigc task bind spec spec:nope bind-probe --format json   # 1 · stdout 0 bytes · findings[0].key = {code: store.not-found, target: spec:nope}
jigc task bind spec nosuch:thing bind-probe --format json # 1 · findings[0].key = {code: store.unknown-type, target: nosuch}

# (6, L-4)   from $REPO/.jigc/worktrees/sub-quick-fix
jigc config set invocation-log true              # 0 — "written to `.jigc/config/`, uncommitted — …"
#   the worktree's own .jigc/config: packs.yaml only ; $REPO/.jigc/config/manifest.yaml carries the key

# (6, L-5) a
chmod 000 $REPO/.jigc/tasks/top-quick-fix/docs
jigc start | grep 'findings: unknown'            # "findings: unknown — enumerating the task's code-anchor surface at "<absolute host path>/.jigc/tasks/top-quick-fix": Permission denied (os error 13)"
jigc start --format json                         # findings: null ; findings_unavailable carries the same absolute path
chmod 755 …
# (6, L-5) b
jigc migrate legacy/old.md --as adr              # 0 — task minted: migrate-adr-legacy-old-<hash> ; 0 "also open:" lines
jigc start --workflow quick-fix "another one"    # 0 — "also open: 11 other tasks were already open before this call …"
# (6, L-5) c
jigc doc show padding                            # 1 — "malformed address `padding`: missing ':' between type and slug …" (no code)
jigc doc show padding --format json              # 1 — {"error": "…"}

# (6, L-6)   E=$(mktemp -d "$RIG/emptypack.XXXXXX")
JIGC_PACK_DIR=$E jigc start                      # 1 · stdout 0 bytes · blocking · pack.resource-missing — no composed pack ships `config/defaults` …
JIGC_PACK_DIR=$E jigc describe                   # 1 · … `config/knobs` …
JIGC_PACK_DIR=$E jigc workflow single-task --preview   # 1 · same
JIGC_PACK_DIR=$E jigc milestone execute delta-run      # 1 · same

# (6, L-2)   rig: fresh --workflow probe <f>   (a manifest-less pack copy; <f>: creates-task: false, no suppressed:)
jigc describe --workflows                        # 0
jigc describe --workflows --format json          # 0 · probe: router_hidden = null
jigc start | grep -c '\bprobe\b'                 # 0

# (6, L-1)   not a jigc drive: os.mkdir(b"nonutf8-\xff-dir") in the scratch root → OSError 92 Illegal byte sequence
```

---

## 4. Defects

Four, all new on this row, all **proposed tier 3**. None is an exit-0 loss or a repository harm; none
is a route dead end a stated route cannot leave.

### `(R8, F-1)` · tier 3 — a `squash=true` sub-task composed from `implement-from-spec` is still told to write and read back its commit doc, and the reason it is given is not something the binary does

**Contract contradicted.** `design/findings-channel.md` §6 (S2): *"the omission removes every false
line, not only `Run:` — … and `step:author-commit`'s 'finalize renders the commit doc', which no
boundary of a docs-only sub-task reads"*; the scope's cell 1: *"no line telling the agent to author,
render or approve a commit doc where no boundary reads one"*. `design/workflow-dialect.md` →
*Emitted format* derives the commit-doc clause from **catalog refs** (`from: task.commit#…`);
`crates/cli/packs/dev/steps/locate-from-spec.yaml:41-45` writes the commit doc through a **literal**
(`jigc doc set-field commit:{{task.id}}#implements …`), so the derivation never sees it.

```
# rig: fresh. A spec committed through the plan workflow (spec:greeting-spec, one criterion).
jigc config get finalize.fan-out.squash          # true (pack-default)
jigc milestone create "Spec run" ; jigc milestone add-task spec-run "impl sub" --workflow implement-from-spec
jigc milestone provision spec-run ; jigc milestone execute spec-run ; <the Spawn line verbatim>     # 0
>  :41  Wire this task's commit to the spec it implements, so the landed record links back
>       to what it built. `commit.implements` points at the spec you bound above — set it by that spec's full address:
>  :45  jigc doc set-field commit:impl-sub#implements --value spec:<slug> --task impl-sub
>  :51  jigc doc show commit:impl-sub --task impl-sub
>  :53  That read names the commit doc because the commit write above is the one this step always makes.
# followed, from the worktree:
jigc task bind spec spec:greeting-spec impl-sub                                      # 0
jigc start --task impl-sub                                                           # 0 (the criterion is listed)
jigc doc set-field commit:impl-sub#implements --value spec:greeting-spec --task impl-sub   # 0
jigc doc show commit:impl-sub --task impl-sub                                        # 0 — "implements: spec:greeting-spec"
git add greet_sub.rs
jigc milestone join spec-run ; jigc milestone finalize spec-run                      # 0 / 0
git log -1 --format=%B   →  "Finalize milestone spec-run (1 sub-task)\n\n- impl-sub\n\nCo-Authored-By: …"
grep -c greeting-spec <the landed message>  → 0 ;  <the landed diff>  → 0
```

**Control, and a wider datum.** The same workflow **top-level**, followed the same way:
`jigc task finalize implement-greeting-top` → exit 0, `feat: greeting top`, one file; the landed
message carries **no** mention of the spec, no tracked file mentions `implements`/`implemented-by`,
and `jigc doc show spec:greeting-spec --format json` → `fields: {'schema-version': '1'}`. So *"the
landed record links back to what it built"* is not visible in what lands at top level either. That
half predates the M54 → rc.24 range (the step file's last commit is M54's relocation of the packs into the crate, `ceb18372`) and is recorded as
the datum that makes the sub-task line false twice over; this row files the **sub-task** line.

**Tier 3, because** the surface states a consequence the binary does not produce; every verb it
names exits 0 and nothing is lost.

### `(R8, F-2)` · tier 3 (low) — two shipped steps say `jigc task finalize` in a sub-task *"lands a commit on this worktree's detached HEAD"*; the binary refuses it

**Contract contradicted.** `design/workflow-dialect.md` → *Emitted format*: *"`jigc task finalize
<sub>` refuses (`finalize.milestone-sub-task`)"*. The composed text of every `sub-task` and
`fix-task` sub-task says otherwise:

```
s-sub-task:62-66  (step: dev sub-task-commit)
  The parent milestone's finalize is the only commit boundary: `git add` your code edits so the milestone can fold
  this worktree's staged index, but never `git commit` and never `jigc task finalize` here — either lands a
  commit on this worktree's detached HEAD and strands the sub-task's work outside the milestone boundary.
s-fix-task:23-25  (step: methodology fix-finding)
  Never `git commit` and never `jigc task finalize` here. The milestone's finalize is the only commit boundary
  of a fix round: either command lands a commit on this worktree's detached HEAD and strands the fix outside that boundary.

jigc task finalize code-sub        # 3 · finalize.milestone-sub-task · worktree HEAD unmoved · "A  greet.rs" still staged   (§3.2)
jigc start   (orientation, same binary, a live sub-task):
> Run: `jigc milestone finalize delta-run` — validate + commit: this is a sub-task of milestone `delta-run`, whose
>   door is its only commit boundary — `jigc task finalize sub-dev-task` refuses here
```

Two surfaces of one binary disagree and the binary agrees with orientation. The **instruction**
(never run it) is safe, which is why M55's E2 fence classes the *never* phrasing as correct
(`completions/artifacts/M55/VERDICT.md` → E2); the **reason** attached to it is the false half. For
`git commit` the sentence is true. **Tier 3, because** a surface says something the binary does not
do, in the harmless direction. (For `fix-task` composed top-level by name the same sentence is the
STILL-OPEN `(6, D-2)`, where the door lands instead of refusing — the two rows are the two halves of
one sentence that is true in neither posture.)

### `(R8, F-3)` · tier 3 — `jigc milestone add-task --workflow` accepts every loaded workflow, so a verb-routed or mints-nothing workflow is composed off its one declared door through `jigc workflow <W> --task <sub>`

**Contract contradicted.** `jigc describe --workflows`, its opening paragraph, driven at row 46:
*"Neither reaches a workflow whose line below says it is reached only through a verb: that verb
binds what its steps read, so it is the one door that composes it, and both of these refuse it by
name."* And `design/workflow-dialect.md` → *No nested `fan-out`*: *"a sub-agent's workflow can never
itself fan out"*. M51 `A6-2` is CLOSED over three argv forms (row 42); this is a fourth form those
drives never included.

```
# rig: fresh (either knob value — both rigs of §3.1)
jigc milestone add-task sweep-all "s migrate-adr" --workflow migrate-adr                # 0
jigc milestone add-task sweep-all "s amend" --workflow amend                            # 0
jigc milestone add-task sweep-all "s milestone-execution" --workflow milestone-execution # 0
jigc milestone add-task sweep-all "s router" --workflow router                          # 0   (creates-task: false)
#   … all 39 exit 0: the 15 `suppressed.door` workflows and the 4 `creates-task: false` ones included
jigc milestone provision sweep-all ; jigc milestone execute sweep-all                   # 0 · 39 Spawn lines
cd $REPO/.jigc/worktrees/s-migrate-adr && jigc workflow migrate-adr --task s-migrate-adr   # 0
> Migrate the foreign ADR into a managed `adr`. Below is the foreign source the CLI
> staged for you (read-only context — …):
>
>                                                     <-- nothing: no source was ever bound
> The target schema and its batch payload, …
cd …/s-milestone-execution && jigc workflow milestone-execution --task s-milestone-execution   # 0
> Run: `jigc milestone provision <MILESTONE_ID>`       <-- an unfilled operand in a `Run:` line; 0 Spawn lines
> Run: `jigc milestone join <MILESTONE_ID>`
> Run: `jigc milestone finalize <MILESTONE_ID>`
cd …/s-amend && jigc workflow amend --task s-amend                                      # 0 — 4 lines: the trailer only, no step
# the three forms A6-2 covers, same rig state:
jigc start --workflow migrate-adr | jigc start --workflow migrate-adr "an intent" | jigc workflow migrate-adr --preview   # 1 ×3 · workflow.verb-routed
```

Of the 15 verb-routed workflows, 14 are composed off their declared door this way (`sub-task`'s
declared door *is* `jigc workflow sub-task --task <id>`). The compositions carry the S2 omission
correctly (0 `Run: … task finalize`, the trailer present); what is wrong is that the verb-bound data
is absent while the prose asserts it. **Tier 3, because** the surfaces (the `describe` paragraph,
the migrate step's *"the foreign source the CLI staged for you"*) say something the binary does not
do; reaching it takes an operator typing `--workflow <verb-routed id>` by hand, nothing commits and
nothing is lost. A reviewer who reads the unfilled `Run:` operand as a route dead end would put it at
tier 2; it never blocks either way. Not new in the range (`--workflow` on `add-task` is M8's); found
because cell 1 asks for *every* workflow that can be a sub-task's mint workflow, and the binary's
answer is all 39, not the 35 `creates-task: true` ones.

### `(R8, F-4)` · tier 3 — the declared bound is wider than its sentence: orientation reports the omitted commit doc as blocking findings on every `squash=true` sub-task

**The sentence** (`completions/artifacts/M55/VERDICT.md` → *Declared bounds*): *"`jigc task validate
<sub>` blocks on the omitted commit doc under `squash=true`"*. It names one door. Driven, a second
door carries the same two findings, at exit 0, on the text arm **and** on the pinned wire:

```
# rig: the §3.3 rig — squash true (pack-default), five live sub-tasks whose composed text asks for no commit doc
jigc start                                       # 0
> Active task: sub-dev-task
>   workflow: dev-task · intent: sub dev-task · base: <sha> · staged: commit:sub-dev-task
>   findings: 2 blocking
> blocking · schema-conformance.field-value-conformant — `commit:sub-dev-task`: field `type` in section `header`: "" is not a member of enum "type" …
>   route: `jigc doc set-field commit:sub-dev-task#header/type --task sub-dev-task --value <value>` to correct the value
> blocking · schema-conformance.required-slot-present — `commit:sub-dev-task`: required slot in section `summary` is empty
>   route: `jigc doc set-slot commit:sub-dev-task#summary --task sub-dev-task --from-file -` to fill the empty slot
> Run: `jigc task validate sub-dev-task` — previews part of the finalize gate: …
> Run: `jigc milestone finalize delta-run` — validate + commit: this is a sub-task of milestone `delta-run`, whose door is its only commit boundary — …
#   the same block for each of the five sub-tasks
jigc start --format json                         # 0 · state: active-task · tasks[i].milestone = "delta-run" · tasks[i].findings = the two codes
```

So the front door tells the orchestrating agent that every sub-task of a default-mode fan-out has
two blocking findings, routed at a doc the composed text never asks for and the boundary named two
lines below (`jigc milestone finalize`) does not read (§3.2: the join lands with every commit doc
unfilled). **Tier 3, because** a surface asserts blocking findings that no boundary of this task
gates on; following the routes is harmless (§3.2, row 35). Filed as the difference, per the scope;
the bound itself HOLDS AS DECLARED at `jigc task validate`.

---

## 5. Observations (driven; no stated contract contradicted, or a declared bound)

- **O-1 — the knob-flip block carries two codes, the design's sentence names one.**
  `design/findings-channel.md` §6 and §10: *"the join's existing routed block (`required-slot-present`
  at `commit:<sub>`, routed at `jigc doc set-slot`)"*. Driven (row 22): `milestone finalize` exit 3
  with `schema-conformance.field-value-conformant` at `commit:<sub>#header/type` (routed at
  `jigc doc set-field`) **and** `schema-conformance.required-slot-present` at `commit:<sub>#summary`.
  And the door that blocks is `jigc milestone finalize`; `jigc milestone join` exits 0. A precision
  datum on a design sentence, not a binary defect.
- **O-2 — a project step that names the per-task door as a literal survives in a sub-task.** Row 16:
  `replace-step workflow:plan#finalize` with a native step whose body is the prose *"now run
  `jigc task finalize {{task.id}}` to commit"* composes that line into the `plan` sub-task. This is
  the derivation as declared (*"classified by the catalog entry, not by the ref's id"*; a literal is
  no catalog ref) and the text is the project's own. Recorded so it is not re-found as a leak.
- **O-3 — unprefixed *finalize* in sub-task text.** 21 of the 39 `squash=true` sub-task bodies carry
  at least one bare *finalize* (e.g. `dev-task`: *"finalize commits only what you have staged"*,
  *"before you finalize"*). This is the VERDICT's *Not run* bound (~40 unprefixed mentions, not
  adjudicated); I did not adjudicate them either. In a sub-task each reads true of
  `jigc milestone finalize` for the two I read in full.
- **O-4 — the sub-task `what's-left:` line says `jigc task validate <sub>` *"previews part of the
  finalize gate: … this task's content findings"*.** Under `squash=true` the content findings it
  previews are the two the boundary does not gate. This is the declared bound seen from the trailer;
  noted, not filed twice.
- **O-5 — `jigc task discard <sub>` refuses `task-discard.staged-prose` over the auto-provisioned,
  never-asked-for `commit:<sub>`** (driven on a sub-task that had done nothing: *"stages 1 doc(s) that
  no commit has a copy of — discarding it would destroy them: commit:sub-quick-fix"*, exit 1, routed
  at `--force` and at `jigc milestone finalize`). The same shape exists for any untouched top-level
  task, so it is not this row's finding; recorded because a third door notices the omitted doc.
- **O-6 — `milestone finalize` sweeps an uncommitted `jigc config set`.** Every fan-out here landed
  `.jigc/config/manifest.yaml` in the aggregate commit; the `config set` ack says *"uncommitted —
  commit it with your next commit"*, so the binary does what it says. Row 5's subject.
- **O-7 — `(2, A2-2)` was not re-driven and its datum was not seen.** No `repo.head-detached` finding
  appeared at any `jigc task validate <sub>` in a fan-out worktree on this run (the sub-tasks are
  milestone-owned; `(2, A2-2)` is about an *ordinary* task minted there, which no cell here mints).

---

## 6. What I did NOT drive, stated plainly

- **The genuine concurrent spawn.** Every sub-task was run by this one process, sequentially.
- **Cell 1's top-level control at the door for 11 of the 12 `migrate-*` workflows.** `migrate … --as
  adr` and `task amend` were driven as controls; the other eleven `migrate --as <ty>` doors were not
  (their sub-task compositions were: 0 finalize lines, 0 `--approve`, trailer present, ×2 knob values).
- **Cell 2 for 37 of the 39 workflows.** Followed to a landed boundary: `dev-task` and
  `report-inconsistency` (×4 arms), `triage-inconsistency`, `implement-from-spec`. The other
  sub-task texts were composed and scanned (rows 3, 4, 39), not followed.
- **Cell 3:** `jigc config remove-step` (no drive); a delta recorded **after** a task was minted (every
  delta here preceded the mint); the verb-minted doors (`task amend`, `migrate`) and the finalize-time
  arms under a delta; the team layer; `squash=false` under deltas. A sub-task composed from the
  **main** checkout under deltas is the base-pin refusal of row 41, not a composition.
- **Cell 4:** only `true` → `false` (the direction the bound declares). `false` → `true` not driven.
- **Cell 5:** `describe`'s narration for the other 35 workflows was partitioned (26 + 13) but not
  checked against behaviour.
- **Cell 6:** the address-shaped intent on `triage-jigc-feedback` (only `triage-inconsistency`).
- **Cell 7:** `report-jigc-feedback` and `triage-jigc-feedback` were composed top-level by name and
  their `what's-left:` line read (path scope), but not followed to a commit; as sub-tasks they were
  composed (rows 3, 4) and not followed to the join.
- **`--format human`** at one door only (row 20).
- **C6 (cwd invariance)** — the rc.19 cell — was not re-swept; four postures at one door (row 41).
- **`(6, L-1)`** — unbuildable here: the filesystem refuses a non-UTF-8 name (`OSError 92`), restated,
  not retried further. **`(6, L-3)`** — a property of a test-support carve-out, not of the binary;
  the third span kind is still emitted (every `Spawn:` line, and row 41's refusal), and
  `crates/cli/tests/support/route_spans.rs` still enumerates two — read, not driven.
  **`(6, L-7)`** — a project pack listed in `packs.yaml`: not driven (needs a hand-written pack tree).
- **`(2, A2-2)`** — named by the scope as not this row's; not driven.
- **`doc show` / `doc list` beyond the task-scoped reads the composed text names** (rows 10–13, 31–33,
  53): no committed-store sweep of either verb.
- The Codex source pass for this row — deliberately not read.

---

## 7. Baseline rows: CLOSED / STILL-OPEN

| key | tier there | status on rc.24 | the drive |
|---|---|---|---|
| `(6, D-1)` — orientation reports a live task's findings without the repository posture | 2 | **CLOSED** | row 44: after `git bisect start`, `jigc start` → exit 0, `findings: 3 blocking, 1 advisory`, `blocking · repo.operation-in-progress` on the text arm and as `tasks[0].findings[0].code` on the wire; `jigc task validate posture-parity` → exit 1, the same code |
| `(6, D-2)` — `fix-task` composable by name; its forbidden door is the only one that works | 2 | **STILL-OPEN (1.x, expected)** | row 45: line 25 forbids `jigc task finalize`; `jigc task finalize fix-the-thing` → exit 0, `finalized … fix: fix the thing`, 1 file |
| `(6, D-3)` — the orientation `Preview:` footer states the pre-M52 rule | 3 | **STILL-OPEN (1.x, expected)** | row 46: footer byte-for-byte as at rc.19; `start --workflow milestone-execution` → 1 `workflow.verb-routed`; `--workflow router` → 0 |
| `(6, D-4)` — an empty `milestone execute` walk does not state its empty case | 3 | **STILL-OPEN (1.x, expected)** | row 47: exit 0, 0 `Spawn:` lines, stderr 0 bytes, no empty-case sentence; `milestone finalize` → 3 `milestone.zero-contribution` |
| `(2, N-1)` = `(6, A6-R1)` — the hook announces a rename on a commit containing none | 3 | **CLOSED** (rc.20's closure carried onto this axis) | row 48: after a committed deletion, the next unrelated commit's stderr is empty; a staged placement-doc `git mv` is **blocked** (rc=1) with the *staged in this commit* sentence |
| M51 `A6-1` | — | **still CLOSED** | row 41: own worktree / sibling worktree / root / subdirectory — three distinct, correct `resume:` or refusal texts, root ≡ subdir |
| M51 `A6-2` | — | **still CLOSED** over the re-derived **15**-member set × 3 argv forms | row 42: 45 drives, 45 exits of 1, 45 `workflow.verb-routed`, `task list` unchanged. **A fourth form is not refused — `(R8, F-3)`** |
| M51 `A6-3` | — | **still CLOSED** | row 43: both empty-case clauses; `store.not-found` / `store.unknown-type` on the findings envelope with their `(code, target)` keys |
| `(6, L-1)` non-UTF-8 path | lead | **OPEN LEAD, unbuildable** | the filesystem refuses the name (`OSError 92 Illegal byte sequence`); not retried |
| `(6, L-2)` the `suppressed:` guarantee is manifest-scoped | lead | **unchanged** (declared) | row 49 |
| `(6, L-3)` the route-span carve-out enumerates two span kinds | lead | **unchanged** | the third kind is still emitted (driven); the carve-out still lists two (read, not driven) |
| `(6, L-4)` a jigc_home-bound door acks a repo-relative path | lead | **unchanged** (declared) | row 50 |
| `(6, L-5)` absolute path in `findings_unavailable` · `migrate` silent over open work · un-coded malformed-address refusal | lead | **unchanged ×3** | rows 51, 52, 53 |
| `(6, L-6)` `pack-resource-missing` × this axis's doors | lead | **reached this run — matches contract** | row 54: four doors, exit 1, `pack.resource-missing`, stdout empty |
| `(6, L-7)` a project pack listed in `packs.yaml` | lead | **not reached** | needs a hand-written pack tree |
| M55's **declared bound** (`jigc task validate <sub>` exits 3 on the omitted commit doc under `squash=true`) | bound | **HOLDS AS DECLARED** at the door it names; **wider** at orientation → `(R8, F-4)` | rows 34–38 |
| M55 **E1** (a sub-task promised the doc-only path scope) | — | **still CLOSED** | row 33 |
| M55 **E2** (the planning sub-task named the refused door) | — | **still CLOSED** | row 3: `s-planning` carries 0 `jigc task finalize`; the `planning-finalize` wrapper prose is absent |
| M55 *structural-op deltas on a fresh compose only* | — | **still CLOSED** | rows 15–20: one resolution at first entry, re-entry, resume, `milestone execute`, JSON and human |

There is **no tier-1 row** on numbered axis 6, and this run adds none.

---

## 8. Doors covered

A leaf is listed iff it is the door of at least one driven row above.

| leaf | rows |
|---|---|
| `start` | 8, 18, 19, 20, 23, 25, 26, 27, 30, 31, 38, 39, 40, 42, 43, 44, 45, 46, 49, 51, 54 |
| `workflow` | 3, 4, 5, 8, 16, 17, 20, 21, 23, 28, 33, 39, 41, 42, 54 |
| `migrate` | 6, 52 |
| `describe` | 29, 46, 49, 54 |
| `validate` | 48 |
| `doc create` | 10–13, 31, 33 |
| `doc add-item` | 10–13, 31 |
| `doc set-field` | 8, 10–13, 24, 31, 32, 33, 35, 37, 45 |
| `doc set-slot` | 10–13, 24, 31, 32, 33, 35, 37, 45 |
| `doc show` | 8, 10–13, 53 |
| `doc list` | 10–13, 32 |
| `task validate` | 10–13, 31, 34, 35, 36, 37, 44 |
| `task amend` | 7 |
| `task finalize` | 9, 31, 32, 45 |
| `task bind` | 8, 43 |
| `config set` | 22, 50 |
| `config fork` | 14 |
| `config insert-step` | 14 |
| `config replace-step` | 14 |
| `milestone create` | 47 |
| `milestone add-task` | 1 |
| `milestone execute` | 2, 15, 20, 21, 47, 54 |
| `milestone join` | 8, 10–13, 22, 33, 35, 37 |
| `milestone finalize` | 8, 10–13, 22, 24, 33, 35, 37, 47 |

24 leaves. `task list`, `task discard`, `config get`, `milestone provision` and `milestone list-tasks`
ran as fixture or control steps inside the blocks above and are the door of no row; they are not
claimed.

---

# R. Reconciliation ledger

**How this was reconciled.** Binary: `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24`,
asserted at the head of every script. Source read at `bffa6667` (`work/rc24-gate`). **15 throwaway
rigs** (`fresh` ×12, one of them with `--workflow probe <f>`; `committed-singletons` ×3), each built
`rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit`, stdout only, then sourced under a
`[ -n "$REPO" ]` guard plus a check that `$REPO` is not the working repository and that
`git rev-parse --show-toplevel` is not it either. No teardown; no removal of any path; nothing
written, staged or committed in the working repository (`git status` clean before and after). Hand-written
files: project-layer step sources under `$RIG/src.<x>/` handed to `jigc config`, and one appended line
in a `jigc config fork` shadow under `.jigc/config/steps/` (the scope's stated exception).
`CLAUDECODE` is **set** in this session and was held constant; the trailer it causes is row 10's.

**Spawn lines.** I am the N-process sim, as the driver was. Each printed `Spawn:` line was shape-checked
against `cd <abs> && jigc workflow <W> --task <sub>` and its two commands were then run **as printed**
(the printed absolute `cd`, then the printed argv) from cwd `/`. This differs from the driver's
`sh -c "<line>"` only in that the line is parsed rather than handed opaque to a shell; a line of any
other shape would have stopped the run, and none occurred (87 distinct lines over eight rigs, 89 runs).

**The rule applied.** A claim by one that the other cannot reproduce is a lead, not a finding. Every
Codex claim below was entered as `lead(codex, …)` and driven; every driver defect was re-driven once.

**Result.** **TIER-1 ROWS ON THIS ROW: 0.** Five CONFIRMED new findings, all tier 3 — four from the
driver, one from Codex (which is the driver's own observation O-2, promoted by a repro). One Codex
disposition REFUTED. Three baseline rows CONFIRMED still open, two CLOSED, three stay closed. Seven
driver rows demoted for a missing repro block, all re-driven and reproducing. One driver count does not
reproduce (row 5).

---

## R.1 The door-set count, against the registries

Recounted from the tree at `bffa6667`; the driver's §1 table stands on every line.

| registry | driver | reconciler | |
|---|---|---|---|
| print sites of `render::composed` / `render::composed_preview` | 9 (8 + 1) over 5 verbs | `cli.rs` 5 + 1 (`composed_preview`), `migrate.rs` 1, `milestone.rs` 1, `task.rs` 1 = **9** over `start` · `workflow` · `migrate` · `milestone execute` · `task amend` | = (the instrument author's "8" is one short, as the driver says) |
| workflows shipped | 18 + 21 = 39 | **18 + 21 = 39** | = |
| `suppressed.door` members | 15 | **15**: `amend` · 12 `migrate-*` · `milestone-execution` · `sub-task` | = |
| steps shipped | 31 + 44 | **31 + 44** | = |
| steps carrying `{{ cli.finalize-task }}` | 4 | **4** (dev `finalize`, dev `amend-message`, methodology `finalize`, methodology `finalize-doc-only`) | = |
| wrappers including one | 4 | **4** (`migration-finalize` ×2, `project-finalize`, `planning-finalize`) | = |
| steps carrying a commit-doc-writing ref | 3 | **3** (`author-commit` ×2, `amend-message`) | = |
| `OFF_CATALOG_VERBS` | 2 | **2** | = |
| `creates-task: false` | 4 | **4** (`router`, `ingest-existing`, `increment`, `milestone-execution`) | = |
| findings workflows visible / hidden | 1 / 3 | **1 / 3** (three carry `suppressed:` with `expires: never`, no `door`) | = |

The row's door set (9: `start` · `workflow` · `migrate` · `milestone execute` · `task amend` ·
`describe` · `doc show` · `doc list` · `task validate`) is fully covered — `task amend` by the
reconciler's `RD-7`, since the driver's only `task amend` row is demoted.

**One count the registries falsify, in a standing row.** Row 5 reads `0 ×20 · 1 ×19` with *"`workflow.verb-routed`
×15; un-coded mints-nothing refusal ×4"*. `milestone-execution` is both verb-routed and
`creates-task: false`, so it is counted twice there. Driven (`RD-5`): **21** previews exit 0, **18**
exit 1 (15 `workflow.verb-routed` + 3 mints-nothing: `router`, `ingest-existing`, `increment`), and
**20** — not 19 — carry exactly one `Run: jigc task finalize` (`fix-task` none). The row's verdict
(top-level keeps the steps) is unaffected.

---

## R.2 The Codex source pass — every claim

### Claim 1 — `lead(codex, a project-layer step that names the per-task door as a literal survives sub-task composition)` → **CONFIRMED · `(R8, C-1)` · tier 3 · door `workflow`**

Driven exactly as the source pass proposed. This is the driver's observation **O-2** (driven there
through `replace-step`); the two passes agree on the behaviour and differed only on whether it is a
finding. Under the rule a Codex claim driven to a repro block is one.

```
# rig: fresh.  finalize.fan-out.squash = true (pack-default)
jigc milestone create "Lit run"                                         # 0
jigc milestone add-task lit-run "park sub" --workflow park-idea         # 0
jigc config fork workflow:park-idea#author-idea                         # 0
> config: forked `workflow:park-idea#author-idea` -> .jigc/config/steps/author-idea.yaml (pinned base …)
# the stated exception — one line appended to that tracked project-layer shadow, no catalog ref:
#   LIT-MARK when the idea reads right, run `jigc task finalize {{task.id}}` to commit it.
jigc workflow park-idea --preview                                       # 0 · LIT-MARK 1 · 1 `Run: … task finalize` (top-level control)
jigc milestone provision lit-run ; jigc milestone execute lit-run       # 0 / 0
<the Spawn line as printed>                                             # 0 · stderr 0 bytes
>  :47  LIT-MARK when the idea reads right, run `jigc task finalize park-sub` to commit it.
>  :50  task scope: … this task is a sub-task of milestone `lit-run`, whose `jigc milestone finalize lit-run` is its only commit boundary
#  `^Run:.*jigc task finalize` lines in the sub-task text: 0   (the pack's own finalize step is omitted)
jigc workflow park-idea --task park-sub      (from its worktree)        # 0 · byte-identical to the first entry
jigc start --task park-sub                   (from its worktree)        # 0 · byte-identical
# the door the surviving line names, from the worktree:
jigc task finalize park-sub                                             # 3
> blocking · finalize.milestone-sub-task — task `park-sub` is a sub-task of milestone `lit-run` — the parent
>   milestone's finalize is the only commit boundary; …
>   route: `jigc milestone finalize lit-run` — …
git rev-parse --short HEAD   → unmoved
```

**Tier 3, because** a composed surface tells the agent to run a door the binary refuses, three lines
above a trailer that says that door is not its boundary. **Not tier 2:** the refusal carries a route
that answers. **Not tier 1:** exit **3**, not 0, and nothing is lost — both halves missing.
**What a triager should know before filing it as a defect:** the design names the classifier as the
catalog entry (`design/workflow-dialect.md` → *Composing for a fan-out sub-task*: *"classified by the
catalog entry, not by the ref's id"*), so a literal is outside the derivation **by its declared
mechanism**, and the E2 fence's own doc comment lists *"a project-layer step shadow, which no
shipped-pack scan can see"* among what it misses (`crates/cli/tests/sub_task_composition.rs`). It is
**not** one of §6's two declared bounds and no pack-load or compose finding notices it. The text is
the project's own. So: reproduced, surface-tier, and a candidate for a written bound rather than a
code change. The source pass's own words — *"a completeness/usability gap, not a demonstrated tier-1
exit-0 loss"* — match what was driven.

### *"No source-grounded tier-1 finding was found inside the M55 code"* → **consistent with every drive on this row**

Not a claim driving can prove; nothing driven contradicts it (§R.3, the tier-1 adjudication).

### rc.19 baseline dispositions

| Codex disposition | status | datum |
|---|---|---|
| `lead(codex, (6, D-1) STILL OPEN — orientation has no repository-posture sweep)` | **REFUTED** | `RD-D1`: after `git bisect start`, `jigc start` → exit 0, `findings: 3 blocking, 1 advisory`, first line `blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a committable state`, routed at `git bisect reset`; `jigc start --format json` → `tasks[0].findings[0].code = repo.operation-in-progress`; `jigc task validate posture-parity` → exit 1, the same code. Orientation **does** carry the posture on rc.24. The driver's CLOSED stands; the source read of `orient.rs` missed the path that reaches it |
| `lead(codex, (6, D-2) STILL OPEN)` | **CONFIRMED** (baseline row, tier 2 there) | `RD-D2`: `jigc start --workflow fix-task "fix the thing"` → 0, line 25 *"Never `git commit` and never `jigc task finalize` here"*; `jigc task finalize fix-the-thing` → **0**, `finalized … fix: fix the thing`, 1 file |
| `lead(codex, (6, D-3) STILL OPEN)` | **CONFIRMED** (baseline row, tier 3) | `RD-D3`: the `Preview:` footer still says *"a workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly"*; `jigc start --workflow milestone-execution` → 1 `workflow.verb-routed` |
| `lead(codex, (6, D-4) STILL OPEN)` | **CONFIRMED** (baseline row, tier 3) | `RD-D4`: `jigc milestone execute empty-probe` → 0, 0 `Spawn:` lines, 30 lines, stderr empty, no sentence naming the empty case; `jigc milestone finalize empty-probe` → 3 `milestone.zero-contribution` |
| `lead(codex, (6, L-1) STILL OPEN — the non-UTF-8 fallback)` | **OPEN LEAD** | cannot be driven here: the filesystem refuses the name (`os.mkdir` of a `\xff` byte name → `OSError 92 Illegal byte sequence`, re-run this pass). The source leg is a read, not a drive. Unchanged from rc.19 |
| `lead(codex, (6, L-2) STILL OPEN, declared; the three hidden findings workflows carry the reason and `expires: never`)` | **CONFIRMED**, stays a declared lead | `RD-L2`: a manifest-less pack's `creates-task: false` workflow with no `suppressed:` loads clean, `router_hidden: null`, absent from orientation. And `describe --workflows --format json` carries a non-empty `router_hidden` for each of the three hidden findings workflows (`RD-C5`); `expires: never` is front-matter, read not driven |
| `lead(codex, (6, L-3) STILL OPEN — absolute `cd … && jigc workflow …` is a distinct span kind)` | **OPEN LEAD** for the carve-out half; the emitted half **CONFIRMED** | the third span kind is emitted on every `Spawn:` line (87 driven here) and in the base-pin refusal (`RD-A61`). Whether `crates/cli/tests/support/route_spans.rs` enumerates it is a property of a test-support file, not of the binary — read, not drivable |
| `lead(codex, (6, L-4) STILL OPEN, declared convention)` | **CONFIRMED**, stays a declared lead | `RD-L4`: `jigc config set invocation-log true` from a fan-out worktree → 0, ack *"written to `.jigc/config/`"*; the key is in the **main** checkout's manifest (0 → 1), the worktree's own `.jigc/config/` holds `packs.yaml` only |
| `lead(codex, (6, L-5) STILL OPEN as three declared bounds)` | **CONFIRMED** ×3, stay declared leads | `RD-L5`: a) `findings: unknown — … at "<absolute host path>/.jigc/tasks/another-one": Permission denied`, JSON `findings: null` + an absolute `findings_unavailable`; b) `jigc migrate legacy/old.md --as adr` with 6 tasks live → 0 `also open:` lines, the next `start --workflow quick-fix` → *"also open: 7 other tasks…"*; c) `jigc doc show padding` → 1, un-coded, JSON `{"error": …}` |
| `lead(codex, (6, L-6) STILL OPEN / not re-driven)` | **driven — matches contract; the lead is discharged** | `RD-L6`: `JIGC_PACK_DIR` pointed at an empty directory × `start` · `describe` · `workflow single-task --preview` · `milestone execute <m>` → exit 1 ×4, `pack.resource-missing` ×4, stdout 0 bytes ×4. The source pass's *"nothing changes"* is borne out |
| `lead(codex, (6, L-7) STILL OPEN / not driven)` | **OPEN LEAD** | not driven by either pass: the rig manufactures a pack behind `JIGC_PACK_DIR`, not a project pack listed in `packs.yaml`, and hand-writing a pack tree is outside the brief's fixture rule |

### *Consistent coverage* — nine statements and the schema boundary

Each is a claim that nothing is wrong; each was driven where a binary can answer it.

| Codex statement | status | the drive |
|---|---|---|
| all five producer families converge on `render::composed`; no bypass | **CONFIRMED** | 9 print sites recounted (§R.1). `--format json` at every producer → keys `['task','text']`: `start --workflow quick-fix "…"` (task set), `start --task <id>`, `workflow quick-fix --preview` (`task: null`), `workflow <W> --task <sub>`, `migrate legacy/old2.md --as adr`, `milestone execute <m>` (`task: null`), `task amend` (`RD-K1`). *No bypass* is a completeness claim the source pass owns; driving found none |
| fresh, minted, resumed and finalize-time reads all apply project structural deltas | **CONFIRMED** | `RD-K2`: a `replace-step` of `finalize-doc-only` and two `insert-step`s. Top-level mint and resume carry the same marks and the same `what's-left:` line; sub-task first entry, re-entry, resume and JSON are byte-identical and omit the carrier; `milestone execute` carries its inserted step in text and JSON; the verb-minted composes (`task amend`, `migrate`) carry theirs at mint and resume; and the **finalize-time arm agrees with the text** — with the doc-only step replaced, the text names the index-honoring model and `jigc task finalize` commits the staged index (`foreign.txt` lands), where the unreplaced workflow leaves it out (driver row 31) |
| the omission closes under inclusion; commit-doc authors leave only under squash | **CONFIRMED** | `RD-1`: `^Run:.*jigc task finalize` 0/39 under both knob values; `--approve` 0, `task amend` 0; `commit:<sub>` in 1 text under `true` (the `(R8, F-1)` literal), in 22 under `false`; *"finalize renders the commit doc"* in 0 / 13 |
| compose and join share the sole squash reader | **CONFIRMED** | `RD-K4`: compose under `true` (31 / 60 lines, 0 commit writes) → `config set finalize.fan-out.squash false` → `milestone join` 0 → `milestone finalize` **3** on `commit:code-sub` only → the same `Spawn:` lines now compose 76 / 105 lines with the author step → both routes run → `milestone finalize` 0, `feat: add greet` + the aggregate, `commit:docs-sub` never filled |
| the sub-task trailer names re-entry and the milestone-only boundary | **CONFIRMED** | `RD-1`: the trailer sentence in 39/39 texts under both knob values; `resume: jigc workflow <W> --task <sub>` present |
| `report-inconsistency` selectable with a `when:`; the other three hidden, door-less, name-composable, reason exposed; all four end in `finalize-doc-only` | **CONFIRMED** | `RD-C5` |
| no stale command or flag in either step tree | **CONFIRMED** within a stated bound | `RD-39`: every distinct `jigc <a> [<b>]` form in 7959 lines of composed text answers `--help` (23 leaves, 39 `workflow <id>`, 1 group stem, 7 prose fragments); every `--flag` printed on the same line as a leaf, over 11170 lines, is declared by that leaf's own `--help` (21 pairs, 13 leaves, 0 misses). **Bound:** the scan reads composed text — the 78 sub-task texts, 4 top-level findings mints and 21 top-level previews — not the step trees; a step no composition here reached, an operand's *value*, and a flag wrapped onto a following line are outside it |
| the declared `squash=false` docs-only and `task validate <sub>` bounds remain open as documented | **CONFIRMED — HOLDS AS DECLARED** | `RD-B`: `squash=true` → `jigc task validate code-sub` 3 and `docs-sub` 3 on `commit:<sub>#header/type` + `#summary`, the text asking for neither, the join landing without either; `squash=false` → a docs-only sub-task is asked for a commit doc `milestone finalize` then lands without (`RD-K4`) |
| the schema boundary: M55 adds exactly `inconsistency` and `jigc-feedback` at schema-version 1; no other hash, version or pinned `--format json` key moved; the co-author trailer does not alter the commit doctype | **CONFIRMED** for the two doctypes; the rest an **OPEN LEAD on this row** | `jigc doc schema inconsistency --format json` and `… jigc-feedback …` → `schema-version: 1`, exit 0; every rig loaded both packs clean, which is the freeze assertion passing on the installed binary; `jigc validate` → 0, clean. *No other hash moved* cannot be driven here — it needs the prior binary, and the repository carries no `rc.19` tag to diff (the tag diff `jigc-v1.0.0-rc.22` → `jigc-v1.0.0-rc.24` shows two manifest entries added and none changed — a source datum, not a drive). The pinned-key and trailer halves are the output-contract row's and row 10's subjects; not driven here beyond the `{task, text}`, orientation and findings-envelope keys above |

---

## R.3 The driver's defects — re-driven once each, and the tiers

| key | re-drive | status | tier | door | why that tier |
|---|---|---|---|---|---|
| `(R8, F-1)` — an `implement-from-spec` sub-task under `squash=true` is told to write and read back `commit:<sub>#implements` *"so the landed record links back to what it built"* | `RD-F1` | **CONFIRMED** | **3** | `workflow` | a surface states a consequence the binary does not produce; every verb it names exits 0. The source pass did not contradict it (it found no stale *command*; this is a true command with a false reason) |
| `(R8, F-2)` — two shipped steps say `jigc task finalize` in a sub-task *"lands a commit on this worktree's detached HEAD"* | `RD-F2` | **CONFIRMED** | **3** | `workflow` (the text) · `task finalize` (the door that falsifies it) | the door refuses, exit 3, `HEAD` unmoved, the staged path still staged; the instruction (*never run it*) is safe and only its stated reason is false |
| `(R8, F-3)` — `jigc milestone add-task --workflow` accepts every loaded workflow, so a verb-routed or mints-nothing one is composed off its declared door | `RD-F3` | **CONFIRMED** | **3** | `milestone add-task` (accepts) · `workflow` (composes) | the three forms M51 `A6-2` covers refuse with a reason the binary states itself — for `milestone-execution`: *"composes a degenerate walk — zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb"* — and the fourth form composes exactly that walk at exit 0. Probed for harm to the boundary (below): none |
| `(R8, F-4)` — orientation reports the omitted commit doc as two blocking findings on every `squash=true` sub-task | `RD-F4` | **CONFIRMED** | **3** | `start` | a surface asserts blocking findings that the boundary named two lines below does not gate on; following the routes is harmless. The bound itself HOLDS AS DECLARED at `task validate` |
| `(R8, C-1)` (Codex claim 1 = driver O-2) | §R.2 | **CONFIRMED** | **3** | `workflow` | see §R.2 |

The source pass contradicted **none** of the four driver defects; no source claim is refuted by them.

### The tier-1 adjudication, both directions

Tier 1 needs **both** halves: exit 0, **and** a loss or repository harm through a committing,
destroying or moving door, shown by a before-control. Each candidate on this row, pressed:

- **`(R8, F-3)`, pressed toward tier 1 — a `migrate-*` sub-task followed to the boundary.** The
  top-level `migrate … --approve` door moves a foreign source; a sub-task composed from the same
  workflow has no source bound. `RD-F3p`: a tracked `legacy/old.md`, a `migrate-adr` sub-task, the
  composed `jigc doc author adr` payload authored, `milestone join` 0, `milestone finalize` **0** —
  `promoted docs/decisions/old-adr.md`, 2 files committed; `git ls-files` before/after differs by that
  one added path; `legacy/old.md` still tracked and present; `git status` clean. **Exit 0: yes. Loss or
  harm: none.** Not tier 1. Not tier 2 either: no refusal, no dead end — the sub-task trailer names the
  boundary and it lands.
- **`(R8, F-4)` and the declared bound, pressed toward tier 1 — the routed commit-doc subject.** The
  blocking finding at `task validate <sub>` (and at orientation) routes the agent to author
  `commit:<sub>#summary`. `RD-B`: a marked subject authored through that route; **before-control** —
  the marker is in 1 file under `.jigc/`; `jigc task validate code-sub` → 0; `milestone join` 0;
  `milestone finalize` **0**; **after** — the marker is in 0 commit messages, 0 tracked files and 0
  files under `.jigc/`. So bytes an agent authored on a route a *blocking* finding printed are gone at
  exit 0 through a committing door. **Exit 0: yes. Loss: the bytes are gone, but they are not a loss
  the contract knows** — `squash: true` is declared as *"one aggregate commit"* (`design/finalize.md`,
  the fan-out render step), `commit` is a transient doctype whose only sink is a message, and
  `design/workflow-dialect.md` → *Composing for a fan-out sub-task* says in terms that this boundary
  *"synthesizes one aggregate message and reads no sub-task's commit doc"*. No managed doc, no code and
  no content any surface promises to land is dropped. **Not tier 1 — the loss half is the knob's declared meaning.** It is the sharpest datum on
  this row and it is what makes `(R8, F-4)` worth its tier 3: two doors print a *blocking* route to
  authoring something the boundary discards by design.
- **`(R8, F-1)`, pressed toward tier 1 — the `implements` relation.** `RD-F1`: before-control, the
  authored `implements: spec:greeting-spec` is in 2 workbench files; `milestone finalize` **0**; after,
  `greeting-spec` is in 0 lines of the landed message and 0 lines of the landed diff. The top-level
  control drops it the same way (`jigc task finalize` 0, message `feat: greeting top`, 0 mentions; the
  spec's projected `fields` still `{'schema-version': '1'}`). The value lands through **no** door in
  either posture, so there is no door that loses what another keeps: the sentence is false, nothing is
  taken. **Not tier 1.** Tier 3 stands.
- **`(R8, F-2)` and `(R8, C-1)`** — the door they name exits **3**. The exit-0 half is missing. Not
  tier 1.
- **Pressed the other way — is anything tier 3 here really tier 2?** A tier-2 row is a posture or
  route dead end. `(R8, F-3)`'s `Run: jigc milestone provision <MILESTONE_ID>` is an unfilled operand
  in a composed line, not a refusal with nowhere to go; `(R8, F-4)`'s routes answer; `(R8, C-1)`'s
  refusal is routed. None is a dead end. All five stay tier 3.
- **The driver's observations, checked for a hidden tier 1.** O-5 (`task discard <sub>` refuses over
  the auto-provisioned commit doc) is a refusal. O-6 (`milestone finalize` and `task finalize` sweep an
  uncommitted `jigc config` write into the commit) reproduced here (`RD-K2`: `.jigc/config/manifest.yaml`
  and three project steps landed in the task's commit) and is what the `config` ack says will happen;
  row 5's subject, not harm. O-3 (unprefixed *finalize* in sub-task text) was not adjudicated by either
  pass and stays an open lead.

---

## R.4 The reconciler's repro blocks

`$J` is the installed binary. Rig and scratch paths are `$REPO` / `$RIG` / `<tmp>`.

### `RD-1` — cell 1, all 39 workflows as sub-tasks, both knob values (re-drives rows 1–4)

```
# rig: fresh ×2
jigc config set finalize.fan-out.squash <true|false>                    # 0
jigc milestone create "Sweep all"                                       # 0
for W in <all 39 shipped ids>: jigc milestone add-task sweep-all "s $W" --workflow $W   # 0 ×39, ×2 rigs
jigc milestone provision sweep-all ; jigc milestone execute sweep-all   # 0 / 0 · 39 Spawn lines · stderr 0 bytes
<each Spawn line as printed>                                            # 0 ×39 · stderr 0 bytes ×39, ×2 rigs

squash=true    ^Run:.*jigc task finalize 0/39 · trailer 39/39 · --approve 0 · task amend 0
               `jigc task finalize` in a body: 2
                 s-fix-task:23  "Never `git commit` and never `jigc task finalize` here. The milestone's finalize"
                 s-sub-task:65  "this worktree's staged index, but never `git commit` and never `jigc task finalize`"
               `commit:s-…` in a body: 1 text — s-implement-from-spec:45, :51 ; "commit doc" prose :53, s-migrate-arch-doc:9
               "finalize renders the commit doc": 0 texts
               lines: single-task 145 · quick-fix 12 · dev-task 31 · park-idea 50 · planning 342 · report-inconsistency 60 · amend 4
squash=false   ^Run:.*jigc task finalize 0/39 · trailer 39/39 · --approve 0 · task amend 0
               `commit:s-…` in 22 texts · "finalize renders the commit doc" in 13
               lines: single-task 194 · quick-fix 61 · dev-task 76 · park-idea 95 · planning 387 · report-inconsistency 105 · amend 4
```

Every count equals the driver's §1 and §3.1.

### `RD-5` — the top-level control and M51 `A6-2` (re-drives rows 5 and 42)

```
# rig: fresh
for W in <all 39>: jigc workflow $W --preview
#   exit 0 ×21 — 20 carry exactly one `Run: jigc task finalize`, fix-task none
#   exit 1 ×18 — 15 `workflow.verb-routed` · 3 "mints no task, so there is nothing to preview" (router, ingest-existing, increment)
for X in <the 15 `suppressed.door` workflows>:
  jigc start --workflow X ; jigc start --workflow X "an intent" ; jigc workflow X --preview
#   45 drives · 45 exit 1 · 45 `workflow.verb-routed` · `jigc task list` byte-identical before/after
jigc start --workflow amend --format json                               # 1 · stderr keys ['findings','schema_version'] · key {code: workflow.verb-routed, target: workflow:amend}
```

The driver's `0 ×20 · 1 ×19` does not reproduce (§R.1). `A6-2` reproduces: 45 / 45 / 45.

### `RD-F1` — `(R8, F-1)`

```
# rig: fresh. A spec committed through the plan workflow:
jigc start --workflow plan "greeting spec"                              # 0 — task minted: greeting-spec
jigc doc create spec --title "Greeting spec" --task greeting-spec       # 0
jigc doc add-item spec:greeting-spec#criteria --title "greets by name" --task greeting-spec   # 0
<set-slot #goal · #context · #criteria/greets-by-name/statement ; the commit doc's type and summary>   # 0 …
jigc task finalize greeting-spec                                        # 0 — finalized … docs: add the greeting spec

jigc milestone create "Spec run" ; jigc milestone add-task spec-run "impl sub" --workflow implement-from-spec
jigc milestone provision spec-run ; jigc milestone execute spec-run ; <the Spawn line as printed>   # 0 …
>  :41  Wire this task's commit to the spec it implements, so the landed record links back
>       to what it built. …
>  :45  jigc doc set-field commit:impl-sub#implements --value spec:<slug> --task impl-sub
>  :51  jigc doc show commit:impl-sub --task impl-sub
#  `^Run:.*jigc task finalize` in the text: 0
# followed, from $REPO/.jigc/worktrees/impl-sub:
jigc task bind spec spec:greeting-spec impl-sub                         # 0
jigc start --task impl-sub                                              # 0 (the criterion is listed)
jigc doc set-field commit:impl-sub#implements --value spec:greeting-spec --task impl-sub   # 0
jigc doc show commit:impl-sub --task impl-sub                           # 0 — "implements: spec:greeting-spec"
git add greet_sub.rs
# before-control: `spec:greeting-spec` in 2 files of the workbench
jigc milestone join spec-run ; jigc milestone finalize spec-run         # 0 / 0
> finalized <sha> — Finalize milestone spec-run (1 sub-task) · modified docs/milestone-records/spec-run.md · added greet_sub.rs · 2 files committed
git log -1 --format=%B   →  "Finalize milestone spec-run (1 sub-task)\n\n- impl-sub\n\nCo-Authored-By: Claude <noreply@anthropic.com>"
greeting-spec in the landed message: 0 · in the landed diff: 0
jigc doc show spec:greeting-spec --format json                          # 0 — fields: {'schema-version': '1'}

# control, top-level:
jigc start --workflow implement-from-spec "implement greeting top"      # 0 — the same lines at :43 / :47, plus one `Run: jigc task finalize …`
<bind · set-field …#implements · type · summary · git add greet_top.rs>
jigc task finalize implement-greeting-top                               # 0 — finalized … feat: greeting top · 1 file committed
greeting-spec in the landed message: 0 · in the landed diff: 0
```

### `RD-F2` — `(R8, F-2)`

```
# rig: the squash=true sweep rig of RD-1
s-sub-task:62-66   "The parent milestone's finalize is the only commit boundary: `git add` your code edits so the milestone
                    can fold this worktree's staged index, but never `git commit` and never `jigc task finalize` here —
                    either lands a commit on this worktree's detached HEAD and strands the …"
s-fix-task:23-25   "Never `git commit` and never `jigc task finalize` here. The milestone's finalize is the only commit
                    boundary of a fix round: either command lands a commit on this worktree's detached HEAD and strands
                    the fix outside that boundary."
# in $REPO/.jigc/worktrees/s-sub-task (toplevel checked), probe.rs staged:
jigc task finalize s-sub-task                                           # 3
> blocking · finalize.milestone-sub-task — task `s-sub-task` is a sub-task of milestone `sweep-all` — the parent
>   milestone's finalize is the only commit boundary; a per-sub-task finalize would land a commit outside it …
>   route: `jigc milestone finalize sweep-all` — …
git rev-parse --short HEAD → unmoved · git status --short → "A  probe.rs"
# and orientation, same binary (RD-F4):
> Run: `jigc milestone finalize fan-run` — validate + commit: this is a sub-task of milestone `fan-run`, whose door is
>   its only commit boundary — `jigc task finalize code-sub` refuses here
```

### `RD-F3` and `RD-F3p` — `(R8, F-3)` and its pressure toward tier 1

```
# rig: the sweep rigs of RD-1 — all 39 `milestone add-task … --workflow <W>` exit 0, both rigs
<the s-migrate-adr Spawn line as printed>                               # 0
> Migrate the foreign ADR into a managed `adr`. Below is the foreign source the CLI
> staged for you (read-only context — …):
>
>                                                     <-- nothing
> The target schema and its batch payload, …
<the s-milestone-execution Spawn line as printed>                       # 0 · 0 Spawn lines
> :10 Run: `jigc milestone provision <MILESTONE_ID>`
> :20 Run: `jigc milestone join <MILESTONE_ID>`
> :29 Run: `jigc milestone finalize <MILESTONE_ID>`
<the s-amend Spawn line as printed>                                     # 0 — 4 lines: the trailer and footer, no step
# the three forms A6-2 covers, same rig:
jigc start --workflow <X> ; jigc start --workflow <X> "an intent" ; jigc workflow <X> --preview    X ∈ {migrate-adr, amend, milestone-execution}
#   9 drives · 9 exit 1 · 9 `workflow.verb-routed` · `jigc task list` unchanged
> blocking · workflow.verb-routed — workflow `milestone-execution` is not composed by name: composes a degenerate walk —
>   zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb, so a router pick could never land
> blocking · workflow.verb-routed — workflow `migrate-adr` is not composed by name: verb-routed — reached only through
>   `jigc migrate <path> --as adr`, which stages the foreign source bytes the author step rewrites; a router pick would
>   compose with no staged source
jigc describe --workflows   (RD-46)
> … Neither reaches a workflow whose line below says it is reached only through a verb: that verb binds what its steps
>   read, so it is the one door that composes it, and both of these refuse it by name.

# RD-F3p — rig: fresh. legacy/old.md committed (carrying LEGACY-MARK).
jigc milestone create "Mig run" ; jigc milestone add-task mig-run "mig sub" --workflow migrate-adr   # 0 / 0
jigc milestone provision mig-run ; jigc milestone execute mig-run ; <the Spawn line as printed>      # 0 …
#   LEGACY-MARK in the composed text: 0 · `^Run:` lines: 0
# in $REPO/.jigc/worktrees/mig-sub, the payload the text prints, filled:
jigc doc author adr --from-file - --task mig-sub                        # 0 — adr:old-adr
jigc milestone join mig-run ; jigc milestone finalize mig-run           # 0 / 0
> finalized <sha> — Finalize milestone mig-run (1 sub-task) · promoted docs/decisions/old-adr.md ·
>   modified docs/milestone-records/mig-run.md · 2 files committed
git ls-files before vs after: one line added (docs/decisions/old-adr.md) · legacy/old.md present · git status --short: empty
```

### `RD-F4` and `RD-B` — `(R8, F-4)`, the declared bound, and where its route leads

```
# rig: fresh.  jigc config get finalize.fan-out.squash → "true  (pack-default)"
jigc milestone create "Fan run"
jigc milestone add-task fan-run "code sub" --workflow dev-task
jigc milestone add-task fan-run "docs sub" --workflow report-inconsistency
jigc milestone provision fan-run ; jigc milestone execute fan-run ; <both Spawn lines as printed>   # 0 …
#   code-sub 31 lines · docs-sub 60 lines · 0 `commit:<sub>` mentions · trailer 1 each

jigc start                                                              # 0
> Active task: code-sub
>   workflow: dev-task · intent: code sub · base: <sha> · staged: commit:code-sub
>   findings: 2 blocking
> blocking · schema-conformance.field-value-conformant — `commit:code-sub`: field `type` in section `header`: "" is not a member of enum "type" …
>   route: `jigc doc set-field commit:code-sub#header/type --task code-sub --value <value>` to correct the value
> blocking · schema-conformance.required-slot-present — `commit:code-sub`: required slot in section `summary` is empty
>   route: `jigc doc set-slot commit:code-sub#summary --task code-sub --from-file -` to fill the empty slot
> Run: `jigc milestone finalize fan-run` — validate + commit: this is a sub-task of milestone `fan-run`, whose door is its only commit boundary — `jigc task finalize code-sub` refuses here
#   the same block for docs-sub
jigc start --format json                                                # 0 · state active-task · tasks[i].milestone = fan-run ·
                                                                        #     tasks[i].findings = the two codes, both sub-tasks

# the bound — from $REPO/.jigc/worktrees/code-sub, greet.rs staged
jigc task validate code-sub                                             # 3 · the two codes, routed as above
jigc task validate code-sub --format json                               # 3 · keys ['findings','schema_version'] ·
                                                                        #     (field-value-conformant, commit:code-sub#header/type) · (required-slot-present, commit:code-sub#summary)
jigc task validate docs-sub      (from its worktree)                    # 3 · the same two codes at commit:docs-sub
# where the route leads — both run as printed, the subject carrying a marker:
jigc doc set-field commit:code-sub#header/type --task code-sub --value feat          # 0
jigc doc set-slot commit:code-sub#summary --task code-sub --from-file -              # 0   (stdin: SUBJECT-NOBODY-READS-MARK)
jigc task validate code-sub                                             # 0 — "no findings — the task validates clean"
# docs-sub: doc create inconsistency · set-slot #description · set-field #meta/kind · add-item #sides ×2 ·
#           set-slot #sides/<side>/says ×2 · doc list inconsistency --task · doc show … --task            # 0 …
# before-control: the marker is in 1 file under .jigc/
jigc milestone join fan-run                                             # 0 — "joined milestone:fan-run — 3 doc(s) merged", HEAD unmoved
jigc milestone finalize fan-run                                         # 0
> finalized <sha> — Finalize milestone fan-run (2 sub-tasks) · promoted docs/inconsistencies/cache-doc-drift.md ·
>   modified docs/milestone-records/fan-run.md · added greet.rs · 3 files committed
>   sub-tasks: code-sub: 1 doc, 1 code file · docs-sub: 2 docs
# after: the marker in the commit messages: 0 · in tracked files: 0 · under .jigc/: 0 files
```

**The bound HOLDS AS DECLARED** at `jigc task validate <sub>`, and is wider at orientation — `(R8, F-4)`.

### `RD-K4` — the knob flipped between compose and join, and the `squash=false` bound (re-drives rows 22–24, 37)

```
# rig: fresh. squash true (pack-default); the fan-run milestone above composed (31 / 60 lines, 0 commit writes).
# code-sub stages greet.rs; docs-sub files inconsistency:cache-doc-drift. Neither authors a commit doc.
jigc config set finalize.fan-out.squash false                           # 0
jigc milestone join fan-run                                             # 0
jigc milestone finalize fan-run                                         # 3 · HEAD unmoved
> blocking · schema-conformance.field-value-conformant … at: commit:code-sub#header/type
>   route: `jigc doc set-field commit:code-sub#header/type --task code-sub --value <value>` …
> blocking · schema-conformance.required-slot-present … at: commit:code-sub#summary · line 9
>   route: `jigc doc set-slot commit:code-sub#summary --task code-sub --from-file -` …
> advisory · file-state.staged-copy — …
<both Spawn lines as printed, again>                                    # 0 ×2 · code-sub 31 → 76 lines · docs-sub 60 → 105 ·
                                                                        #        5 commit writes each · "finalize renders the commit doc" ×1 · 0 Run:task-finalize
jigc start --task code-sub        (from its worktree)                   # 0 · byte-identical to the re-spawn
jigc task validate docs-sub       (from its worktree)                   # 3 · the two codes at commit:docs-sub
<the two code-sub routes, as printed>                                   # 0 / 0
jigc milestone finalize fan-run                                         # 0 — "feat: add greet" (<sha>) + the aggregate; commit:docs-sub never filled
```

The block carries two codes where `design/findings-channel.md` §6 names one, and the blocking door is
`milestone finalize`, not `milestone join` — the driver's O-1, reproduced.

### `RD-K2` — project structural deltas at every compose door and at finalize

```
# rig: fresh. Source step files under $RIG/src.<x>/ (outside the repo):
#   proj-fin.yaml  : "PROJ-FIN-MARK …\n\n{{ cli.finalize-task }}"
#   proj-gate.yaml : "GATE-MARK …\n\n{{ cli.finalize-task }}"
#   exec-note.yaml : "EXEC-MARK …"       amend-note.yaml : "AMEND-MARK …"       mig-note.yaml : "MIG-MARK …"
jigc config replace-step 'workflow:report-inconsistency#finalize-doc-only' <src>/proj-fin.yaml   # 0
jigc config insert-step --workflow dev-task --before finalize <src>/proj-gate.yaml               # 0
jigc config insert-step --workflow milestone-execution --before provision-worktrees <src>/exec-note.yaml   # 0

jigc start --workflow report-inconsistency "delta report"               # 0 · PROJ-FIN-MARK 1 · 1 Run:task-finalize · "commits path-scoped" 0
> what's-left: … this task's content findings, the carryover gate, the owner-artifact causes …      (no path-scope clause)
jigc start --task delta-report                                          # 0 · PROJ-FIN-MARK 1 · the same what's-left line
jigc start --workflow dev-task "delta dev"                              # 0 · GATE-MARK 1 · 2 Run:task-finalize lines

jigc milestone create "Delta run" ; add-task … "sub report" --workflow report-inconsistency ; add-task … "sub dev" --workflow dev-task
jigc milestone provision delta-run
jigc milestone execute delta-run                                        # 0 · EXEC-MARK 1 · 2 Spawn lines
jigc milestone execute delta-run --format json                          # 0 · keys ['task','text'] · task null · EXEC-MARK 1
<both Spawn lines as printed>                                           # 0 ×2 · PROJ-FIN-MARK 0 · GATE-MARK 0 · 0 Run:task-finalize · trailer 1
jigc workflow <W> --task <sub> ; jigc start --task <sub>                # 0 ×4 · byte-identical to the first entry, 2/2 and 2/2
jigc start --task <sub> --format json                                   # 0 ×2 · keys ['task','text'] · 0 marks · 0 `jigc task finalize`

# the finalize-time arm, top-level task delta-report: the inconsistency doc and the commit doc authored,
# then  printf 'foreign\n' > foreign.txt ; git add foreign.txt
jigc task validate delta-report                                         # 0 (advisory file-state.staged-copy)
jigc task finalize delta-report                                         # 0
> finalized <sha> — docs: file the delta drift · added .jigc/config/manifest.yaml · added .jigc/config/steps/{exec-note,proj-fin,proj-gate}.yaml ·
>   promoted docs/inconsistencies/delta-drift.md · added foreign.txt · 6 files committed
#   the text named the index-honoring model and the commit honors the index — the two agree

# the verb-minted composes:
jigc config insert-step --workflow amend --before amend-message <src>/amend-note.yaml            # 0
jigc config insert-step --workflow migrate-adr --before migration-finalize <src>/mig-note.yaml   # 0
jigc task amend                                                         # 0 — task minted: amend-<sha> · AMEND-MARK 1 ; `jigc start --task amend-<sha>` → AMEND-MARK 1
jigc migrate legacy/old.md --as adr                                     # 0 — task minted: migrate-adr-legacy-old-<hash> · MIG-MARK 1 ; resume → MIG-MARK 1
```

Not driven under deltas: `config remove-step`; a delta recorded after the mint; the team layer.

### `RD-C5` — the four findings workflows (re-drives rows 25–30; supplies `RD-27`, `RD-28`, `RD-40`)

```
# rig: committed-singletons
jigc start ; jigc start --format json ; jigc start --workflow router    # 0 ×3
#   mentions on each of the three: report-inconsistency 1 · report-jigc-feedback 0 · triage-jigc-feedback 0 · triage-inconsistency 0
>   - report-inconsistency — code and a doc, or two docs, disagree and the disagreement is worth a record until it is reconciled
#   JSON keys ['header','next_steps','schema_version','state','workflows'] · 13 workflows
jigc describe --workflows                                               # 0 · "It is hidden from the router catalog" ×26
jigc describe --workflows --format json                                 # 0 · keys ['commands','definitions','schema_version'] · 39 = 26 hidden + 13 visible
>   report-inconsistency  router_hidden: null
>   report-jigc-feedback  router_hidden: "invoked by name (`jigc start --workflow report-jigc-feedback "<what>"`) — jigc's own agents report what they hit about jigc, …"
>   triage-inconsistency  router_hidden: "invoked by name with the record named in plain words (…) — triage is a maintainer's act, not a catalog entry"
>   triage-jigc-feedback  router_hidden: "invoked by name with the finding named in plain words (…) — triage is a maintainer's act, not a catalog entry"
# RD-28
jigc workflow <id> --preview   ×4                                       # 0 ×4 · one `Run: jigc task finalize` and the path-scoped step prose in each ·
                                                                        #        `jigc task list` byte-identical around each
# RD-27
jigc start --workflow <id> "probe words for <id>"   ×4                  # 0 ×4 · `task minted:` ×4 · one `Run: jigc task finalize <id>` in each
# cell 6
jigc start --workflow triage-inconsistency "inconsistency:cache-doc-drift: reconciled by the cache rewrite"   # 0 — task minted: inconsistencycache-doc-drift-reconciled
jigc start --workflow triage-inconsistency "cache doc drift reconciled by the cache rewrite"                  # 0 — task minted: cache-doc-drift-reconciled
>  :8  an address-shaped intent mangles it; the address, `inconsistency:<slug>`, goes to …
# RD-40
jigc start --task cache-doc-drift-reconciled                            # 0
> :116 resume: `jigc start --task cache-doc-drift-reconciled`   — re-composes this workflow if context is lost
<that resume argv, run>                                                 # 0 · byte-identical to the composition that printed it
```

### `RD-6`, `RD-7`, `RD-K1` — the verb-minted controls and the composed JSON shape

```
# rig: the committed-singletons rig of RD-C5; legacy/old.md and legacy/old2.md committed (old.md carries LEGACY-MARK)
# RD-6
jigc migrate legacy/old.md --as adr                                     # 0 — task minted: migrate-adr-legacy-old-<hash>
#   `^Run:.*jigc task finalize` 1 · `--approve` 1 · LEGACY-MARK 1 (the source body is embedded) · `also open:` 0
# RD-7
jigc task amend                                                         # 0 — task minted: amend-<sha>
#   96 lines · `^Run:.*jigc task finalize` 1 · 8 `commit:amend-<sha>#` writes · stderr 0 bytes
# RD-K1 — keys of `--format json` stdout at every composed producer
jigc start --workflow quick-fix "json probe" --format json              # 0 · ['task','text'] · task json-probe
jigc start --task json-probe --format json                              # 0 · ['task','text'] · task json-probe
jigc workflow quick-fix --preview --format json                         # 0 · ['task','text'] · task null
jigc migrate legacy/old2.md --as adr --format json                      # 0 · ['task','text'] · task migrate-adr-legacy-old2-<hash>
jigc task amend --format json                                           # 0 · ['task','text'] · task amend-<sha>
#   `jigc workflow <W> --task <sub> --format json` and `jigc milestone execute <m> --format json`: RD-K2
```

### `RD-39` — the mechanical sweep (re-drives row 39)

```
# corpus: the 78 sub-task texts of RD-1 + the 4 top-level findings mints of RD-27 — 7959 lines
# every distinct `jigc <a> [<b>]` form extracted; each driven as `jigc <a> [<b>] --help`
distinct forms 70:
  23 clap leaves, `--help` exit 0 with the matching usage line —
     config set · describe · doc add-item · doc author · doc create · doc list · doc schema · doc set-field · doc set-slot ·
     doc show · ingest · migrate · milestone add-task · milestone create · milestone execute · milestone finalize ·
     milestone join · milestone provision · start · task bind · task discard · task finalize · task validate
  39 `workflow <id>` forms, each id a shipped workflow
  1 group stem (`jigc doc`, in prose)
  7 prose fragments, exit 2 as verbs: jigc can · checks · did · has · itself · never · version
# flags — the same corpus plus the 21 top-level previews of RD-5, 11170 lines:
# every `--flag` printed on the same line after a `jigc <leaf>`, checked against that leaf's own `--help`
21 (leaf, flag) pairs over 13 leaves · 0 flags the leaf does not declare
```

### `RD-D1` … `RD-D4`, `RD-46` — the M52 rows

```
# rig: fresh
# RD-D3 / RD-46
jigc start | grep '^Preview:'
> Preview: `jigc workflow <id> --preview`   — read a task-minting workflow's step text without minting a task; a workflow
>   that mints nothing has no preview — `jigc start --workflow <id>` composes it directly, and mints nothing either
jigc start --workflow milestone-execution                               # 1 · workflow.verb-routed
jigc start --workflow router                                            # 0
jigc describe --workflows                                               # 0 — the opening paragraph carries "…and both of these refuse it by name."
# RD-D4
jigc milestone create "Empty probe"                                     # 0
jigc milestone execute empty-probe                                      # 0 · 0 Spawn lines · 30 lines · stderr 0 bytes ·
                                                                        #     "empty" appears only inside the id `empty-probe`
jigc milestone finalize empty-probe                                     # 3 · milestone.zero-contribution
# RD-D2
jigc start --workflow fix-task "fix the thing"                          # 0
>  :25  Never `git commit` and never `jigc task finalize` here. The milestone's finalize
<commit type fix · summary · git add fixed.txt>
jigc task finalize fix-the-thing                                        # 0 — finalized <sha> — fix: fix the thing · 1 file committed
# RD-D1
jigc start --workflow single-task "posture parity"                      # 0
git bisect start                                                        # 0
jigc start                                                              # 0
>   findings: 3 blocking, 1 advisory
> blocking · repo.operation-in-progress — a bisect is in progress — the repository is not in a committable state
>   route: conclude it, or abandon it with `git bisect reset`, then re-run this command
jigc start --format json                                                # 0 · tasks[0].findings codes: repo.operation-in-progress ·
                                                                        #     schema-conformance.field-value-conformant · schema-conformance.required-slot-present ·
                                                                        #     changelog-recording.gate-granted-unused
jigc task validate posture-parity                                       # 1 · repo.operation-in-progress
```

### `RD-A61`, `RD-A63`, `RD-N1`, `RD-E1` — the closed rows, carried by a drive

```
# RD-A61 · rig: the RD-K2 rig (HEAD off the pin after the task's commit)
jigc workflow dev-task --task sub-dev      from its own worktree       # 0 — "…run it here"
                                           from the sibling worktree   # 0 — "this checkout is not that worktree — run it from this sub-task's own worktree at `.jigc/worktrees/sub-dev`, where its work happens"
                                           from $REPO                  # 1 — "task `sub-dev` is pinned to base <a> but you're on <b> — … run this from that worktree — `cd <tmp>/repo/.jigc/worktrees/sub-dev`, then re-run this command"
                                           from $REPO/docs/deep        # 1 — byte-identical to the root's
# RD-A63 · rig: fresh, zero specs
jigc start --workflow implement-from-spec "bind probe"                  # 0 — "nothing is listed until a `spec` is committed; with none there is …"
jigc task bind spec spec:nope bind-probe --format json                  # 1 · stdout 0 bytes · keys ['findings','schema_version'] · key {code: store.not-found, target: spec:nope}
jigc task bind spec nosuch:thing bind-probe --format json               # 1 · key {code: store.unknown-type, target: nosuch}
# RD-N1 · rig: committed-singletons ×2
jigc validate                                                           # 0
git add f1.txt ; git commit -m "unrelated 1"                            # 0 · stderr 0 bytes
git rm -q docs/roadmap.md ; git commit -m "delete the roadmap out of band"   # 0 · stderr 0 bytes
git add f2.txt ; git commit -m "unrelated 2"                            # 0 · stderr 0 bytes · "rename" 0 times
#   second rig:
git mv docs/roadmap.md docs/plan.md ; git commit -m "bare git mv of a placement doc"   # 1
> jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).
# RD-E1 · the report sub-task of RD-K2 and of RD-F4
> what's-left: `jigc task validate sub-report` — previews part of the finalize gate: the repository posture finalize refuses
>   under, this task's content findings, the carryover gate, …
#   "path-scoped" / "path scope" / "stays staged" in the sub-task text: 0
#   (the join committing a report sub-task's staged code is the driver's row 33; not re-driven)
```

### `RD-L2`, `RD-L4`, `RD-L5`, `RD-L6`, `RD-L1` — the leads

```
# RD-L2 · rig: fresh --workflow probe <f>   (<f>: `creates-task: false`, no `suppressed:`; the pack copy drops its manifest)
jigc describe --workflows                                               # 0 — "probe is A probe workflow that mints nothing …" (no hidden sentence)
jigc describe --workflows --format json                                 # 0 · probe: router_hidden = null
jigc start                                                              # 0 · `probe` 0 times
# RD-L4 · from $REPO/.jigc/worktrees/s-quick-fix of the squash=false sweep rig
jigc config set invocation-log true                                     # 0 — "config: set `invocation-log` = `true` — written to `.jigc/config/`, uncommitted — …"
#   $REPO/.jigc/config/manifest.yaml: the key 0 → 1 · the worktree's own .jigc/config/: packs.yaml only
# RD-L5 · the RD-C5 rig
chmod 000 $REPO/.jigc/tasks/another-one/docs
jigc start                                                              # 0 — "findings: unknown — enumerating the task's code-anchor surface at "<absolute host path>/.jigc/tasks/another-one": Permission denied (os error 13)"
jigc start --format json                                                # 0 · findings: null · findings_unavailable carries the same absolute
chmod 755 …
jigc migrate legacy/old.md --as adr                                     # 0 · 0 "also open:" lines (6 tasks live)
jigc start --workflow quick-fix "another one"                           # 0 — "also open: 7 other tasks were already open before this call …"
jigc doc show padding                                                   # 1 — "malformed address `padding`: missing ':' between type and slug …" (no code)
jigc doc show padding --format json                                     # 1 — {"error": "…"}
# RD-L6 · E=$(mktemp -d "$RIG/emptypack.XXXXXX")
JIGC_PACK_DIR=$E jigc start                                             # 1 · stdout 0 bytes · blocking · pack.resource-missing — … `config/defaults` …
JIGC_PACK_DIR=$E jigc describe                                          # 1 · stdout 0 bytes · … `config/knobs` …
JIGC_PACK_DIR=$E jigc workflow single-task --preview                    # 1 · same
JIGC_PACK_DIR=$E jigc milestone execute sweep-all                       # 1 · same
# RD-L1 · not a jigc drive
os.mkdir(<a name carrying the byte \xff>) under $RIG                    → OSError 92 Illegal byte sequence
```

---

## R.5 Baseline rows — CLOSED / STILL-OPEN, reconciled

| key | tier there | driver | Codex | reconciled on rc.24 | the drive |
|---|---|---|---|---|---|
| `(6, D-1)` — orientation reports a live task's findings without the repository posture | 2 | CLOSED | STILL OPEN | **CLOSED** — the source disposition is refuted | `RD-D1` |
| `(6, D-2)` — `fix-task` composable by name; its forbidden door is the only one that works | 2 | STILL-OPEN | STILL OPEN | **STILL-OPEN (1.x, expected)** | `RD-D2` |
| `(6, D-3)` — the orientation `Preview:` footer states the pre-M52 rule | 3 | STILL-OPEN | STILL OPEN | **STILL-OPEN (1.x, expected)** | `RD-D3` |
| `(6, D-4)` — an empty `milestone execute` walk does not state its empty case | 3 | STILL-OPEN | STILL OPEN | **STILL-OPEN (1.x, expected)** | `RD-D4` |
| `(2, N-1)` = `(6, A6-R1)` — the hook announces a rename on a commit containing none | 3 | CLOSED | — | **CLOSED** | `RD-N1` |
| M51 `A6-1` | — | still CLOSED | — | **still CLOSED** | `RD-A61` |
| M51 `A6-2` | — | still CLOSED over 15 × 3 | — | **still CLOSED** over the re-derived 15 × 3 argv forms (45 / 45 / 45); a fourth form is `(R8, F-3)` | `RD-5` |
| M51 `A6-3` | — | still CLOSED | — | **still CLOSED** | `RD-A63` |
| `(6, L-1)` non-UTF-8 path | lead | OPEN, unbuildable | STILL OPEN | **OPEN LEAD** — unbuildable on this filesystem | `RD-L1` |
| `(6, L-2)` the `suppressed:` guarantee is manifest-scoped | lead | unchanged | STILL OPEN, declared | **OPEN LEAD, unchanged (declared)** | `RD-L2` |
| `(6, L-3)` the route-span carve-out enumerates two span kinds | lead | unchanged | STILL OPEN | **OPEN LEAD** — the third kind is emitted (driven); the carve-out is a test-support property (read) | `RD-1`, `RD-A61` |
| `(6, L-4)` a `jigc_home`-bound door acks a repo-relative path | lead | unchanged | STILL OPEN, declared | **OPEN LEAD, unchanged (declared convention)** | `RD-L4` |
| `(6, L-5)` three declared bounds | lead | unchanged ×3 | STILL OPEN ×3 | **OPEN LEAD, unchanged ×3 (declared)** | `RD-L5` |
| `(6, L-6)` `pack-resource-missing` × this axis's doors | lead | reached, matches contract | not re-driven | **reached on this run by both drivers — matches contract; discharged** | `RD-L6` |
| `(6, L-7)` a project pack listed in `packs.yaml` | lead | not reached | not driven | **OPEN LEAD** — driven by nobody | — |
| M55's declared bound (`jigc task validate <sub>` exits 3 on the omitted commit doc under `squash=true`) | bound | HOLDS AS DECLARED; wider at orientation | remains open as documented | **HOLDS AS DECLARED**; wider at orientation → `(R8, F-4)` | `RD-B`, `RD-F4` |
| M55's declared bound (a docs-only sub-task under `squash=false` still authors an unread doc; the knob flip meets the join's routed block) | bound | HOLDS | remains open as documented | **HOLDS AS DECLARED** | `RD-K4` |
| M55 **E1** | — | still CLOSED | — | **still CLOSED** — the text half re-driven (`RD-E1`); the join half on the driver's row 33 | `RD-E1` |
| M55 **E2** | — | still CLOSED | — | **still CLOSED** — `s-planning` carries 0 `jigc task finalize` (`RD-1`) | `RD-1` |
| M55 *structural-op deltas on a fresh compose only* | — | still CLOSED | all four reads apply them | **still CLOSED**, now also at the finalize-time arm and the verb-minted composes | `RD-K2` |

There is **no tier-1 row** on numbered axis 6 at the baseline, and this run adds none.

### New on this row

| key | origin | status | tier | door |
|---|---|---|---|---|
| `(R8, F-1)` | driver | CONFIRMED | 3 | `workflow` |
| `(R8, F-2)` | driver | CONFIRMED | 3 | `workflow` · `task finalize` |
| `(R8, F-3)` | driver | CONFIRMED | 3 | `milestone add-task` · `workflow` |
| `(R8, F-4)` | driver | CONFIRMED | 3 | `start` |
| `(R8, C-1)` | codex (= driver O-2) | CONFIRMED | 3 | `workflow` |

### Open leads carried out of this row

- `(6, L-1)` · `(6, L-3)` (the carve-out half) · `(6, L-7)` — reasons in the table above.
- `(6, L-2)` · `(6, L-4)` · `(6, L-5)` ×3 — driven, unchanged, each inside a declared bound.
- **O-3** — the unprefixed *finalize* mentions in sub-task text (21 of 39 `squash=true` bodies by the
  driver's count): adjudicated by neither pass; `completions/artifacts/M55/VERDICT.md` → *Not run*.
- **The schema boundary's *no other hash or pinned key moved*** — not drivable on this row without the
  prior binary; the freeze row's and the output-contract row's.
- **Not driven by either pass, from the driver's §6:** `config remove-step`; a delta recorded after a
  mint; the team layer; the knob flipped `false` → `true`; the address-shaped intent on
  `triage-jigc-feedback`; the two `*-jigc-feedback` workflows followed to a commit; eleven of the
  twelve `migrate --as <ty>` doors as top-level controls.

---

## R.6 Doors covered (reconciled)

VERB_KINDS spelling (`crates/cli/src/cli.rs`). A leaf is listed iff it is the door of at least one row
that carries a repro block — a standing driver row, or a reconciler re-drive (`RD-…`).

| leaf | standing driver rows | reconciler re-drives |
|---|---|---|
| `start` | 8, 18, 19, 20, 23, 25, 26, 27 (in part), 30, 31, 38, 42, 43, 44, 45, 46, 49, 51, 54 | `RD-C5`, `RD-D1`–`RD-D3`, `RD-F4`, `RD-K1`, `RD-K2`, `RD-40` |
| `workflow` | 3, 4, 5, 8, 16, 17, 20, 21, 23, 28 (in part), 33, 41, 42, 54 | `RD-1`, `RD-5`, `RD-F1`–`RD-F3`, §R.2 `(R8, C-1)`, `RD-K2`, `RD-A61` |
| `migrate` | 52 | `RD-6`, `RD-K1`, `RD-K2` |
| `describe` | 29, 49, 54 | `RD-C5`, `RD-46`, `RD-L2` |
| `validate` | 48 | `RD-N1` |
| `doc create` | 10–13, 31, 33 | `RD-F1`, `RD-B` |
| `doc add-item` | 10–13, 31 | `RD-F1`, `RD-B` |
| `doc set-field` | 8, 10–13, 24, 31, 32, 33, 35, 37, 45 | `RD-F1`, `RD-B`, `RD-K4` |
| `doc set-slot` | 10–13, 24, 31, 32, 33, 35, 37, 45 | `RD-F1`, `RD-B`, `RD-K4` |
| `doc show` | 8, 10–13, 53 | `RD-F1`, `RD-L5` |
| `doc list` | 10–13, 32 | `RD-B` |
| `doc author` | — | `RD-F3p` *(reconciler-added)* |
| `doc schema` | — | §R.2, the schema boundary *(reconciler-added)* |
| `task validate` | 10–13, 31, 34, 35, 36, 37, 44 | `RD-B`, `RD-K4`, `RD-D1` |
| `task amend` | — *(row 7 demoted)* | `RD-7`, `RD-K1`, `RD-K2` |
| `task finalize` | 9, 31, 32, 45 | `RD-F2`, §R.2 `(R8, C-1)`, `RD-D2`, `RD-K2` |
| `task bind` | 8, 43 | `RD-F1`, `RD-A63` |
| `config set` | 22, 50 | `RD-K4`, `RD-L4` |
| `config fork` | 14 | §R.2 `(R8, C-1)` |
| `config insert-step` | 14 | `RD-K2` |
| `config replace-step` | 14 | `RD-K2` |
| `milestone create` | 47 | `RD-D4` |
| `milestone add-task` | 1 | `RD-1`, `RD-F3` |
| `milestone execute` | 2, 15, 20, 21, 47, 54 | `RD-1`, `RD-K2`, `RD-D4`, `RD-L6` |
| `milestone join` | 8, 10–13, 22, 33, 35, 37 | `RD-F1`, `RD-F3p`, `RD-B`, `RD-K4` |
| `milestone finalize` | 8, 10–13, 22, 24, 33, 35, 37, 47 | `RD-F1`, `RD-F3p`, `RD-B`, `RD-K4`, `RD-D4` |

**26 leaves** — the driver's 24, every one still standing (`task amend` on the reconciler's re-drive
alone), plus `doc author` and `doc schema`, which only a reconciler drive reaches. `task list`,
`task discard`, `config get`, `milestone provision` and `milestone list-tasks` ran as fixture or
control steps and are the door of no row; they are not claimed. The row's own nine-door set is covered
in full.
