# M53 handover — start here, then plan the fix pass, then build it

Written 2026-09-21 at the end of the session that planned, built, audited and re-reviewed M52 and then
chartered M53 with the human. **Nothing below needs reconstructing from a transcript**: every decision,
correction, drive and halt is committed, and this file only points.

## State you're inheriting

| | |
|---|---|
| HEAD | `cf3b382b` (the M53 charter + the exit rule), pushed, tree clean — 103 commits since M52's planning base `c59ec3d7` |
| Gate | **3798 passed / 0 failed**, fmt + clippy clean, measured bare via `dev/gate` at the close-out commit `a3eb026b` |
| Binary | **`1.0.0-rc.16`** installed at `~/.local/bin/jigc` (sha256 `0ddd1ee6…`), built from `a3eb026b` **after** M52's audit fixes; `target/` is warm |
| The 1.0.0 call | **Not taken on rc.16.** The per-axis review re-run found four tier-1 rows, all inside mechanism M52 itself minted ([DECISIONS.md](../../../DECISIONS.md) → 2026-09-21 the exit rule) |
| M52 | **Complete**: built, audited (7 findings, 7 fixed axis-complete), rc.16 stamped after the fixes — [VERDICT](../M52/VERDICT.md); the review re-run persisted at [M52/per-axis-review/](../M52/per-axis-review/README.md) |
| M53 | **Chartered, not planned** — [decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.17 fix pass (M53)*: the four tier-1 rows, **no new mechanism**, acceptance = axes 2 · 3 · 5 re-driven; the 23 tier-2/3 rows triaged to 1.x |

## What changed in how we work — read before planning

**The exit rule is written and is the human's own gate** ([decisions-pending.md](../../../implementation/decisions-pending.md)
→ *The rc.17 fix pass (M53)* → *The exit rule*; graduated to [DECISIONS.md](../../../DECISIONS.md)
2026-09-21): *the 1.0.0 call is taken when a partial re-review of the fix pass's affected axes finds no
tier-1 row; tier-2/3 findings never block and go to the ledger for 1.x; a finding inside a fix pass's own
new code triggers another fix pass and partial re-review, never a full wave.* The human's reading of six
waves: *fixing usability led to new functionality with errors*, and v1 must be a usable product. **So M53
is a fix pass, not a wave.** Do not run the full planning fan-out over new mechanism: the four rows carry
their fix shapes in the charter already (a registry row · a condition on a guard · one posture member · the
guard a doc-comment already names). Planning is a short Scope (drive the four cells on the installed rc.16
to confirm them at HEAD), a Settle that mostly confirms those shapes — with a `robust-advocate` only where
a fix shape is genuinely forked — and a decomposition of **four increments plus a close**. **A fix that
needs new mechanism to be complete is a halt to the human**, with the mechanism named and its sibling
cells enumerated, because new mechanism is where the next tier-1 row came from every time.

**After the call comes the port** (direction, not chartered): this repository onto 1.0.0, running docs
first, `CLAUDE.md` shrunk to invariants + how-we-work + `jigc start`, and a methodology `findings-ledger`
doctype + `report-finding` workflow as the port's first act (the 23 rows are its seed). Not this session's
work; do not start it before the call.

## What you're running

**First `/milestone-plan M53`, then `milestone-build` for M53**, both from the main session, planning
human-led. Build with the harness defaults, re-passed on every resume:
`Workflow({ name: 'milestone-build', args: { milestone: 'M53', base: '<HEAD after planning>', model: 'opus' } })`.
Then the completion audit → findings reported to the human before fixing → fixers one at a time → the
VERDICT under `completions/artifacts/M53/` → the bump to `1.0.0-rc.17` + install **after** the fixes →
the goldens checked to move by the version string only → **the partial re-review**: axes 2, 3 and 5 only,
with M52's re-pointed instrument ([M52/per-axis-review/instrument/](../M52/per-axis-review/instrument/README.md)),
persisted under `completions/artifacts/M53/per-axis-review/`, compared row by row against M52's §A for
those axes. Then stop — the 1.0.0 call is the human's.

## The rules that bit this session — do not re-learn them

1. **A killed build harness may leave its loop registered as running, and then no resume is possible.**
   The M52 run's shell was stopped mid-increment-9; `TaskStop` reported it killed but the loop never
   exited, and every resume was refused (*"would run two copies against the same journal"*). The
   documented fallback worked: finish the interrupted increment's remaining tasks with `build-executor`
   subagents outside the harness, run `increment-validator`, fix, re-validate, then a **fresh** run with
   `skipThrough: <that increment>` (no `resumeFromRunId`). Before any of that, classify the dirty tree
   the killed executor left: it had passed its gate, so it was committed as *recovered, not authored*.
2. **A halt's resume note goes into the run's script snapshot, keyed on the increment, and a plain
   resume replays the cached halt** (zero tokens, 11 ms — that is the tell). The build script's own
   header rules 1–5 are exact; both M52 halts (§18, §19) were resumed that way.
3. **The Codex source passes run `codex exec -s read-only` and must never be told to write a file.**
   The first re-run attempt prepended *"write your pass to the path the harness names"*; the sandbox
   refused every write and the captured final messages were one-paragraph summaries. Re-run with
   *"your entire report is your final message"*. The lesson is written into the instrument's README.
4. **The reconcile phase starts the moment its driver returns, which can be before Codex finishes** —
   the instrument now has an `until [ -s <file> ]` wait guard in the reconciler prompt. Keep it.
5. **Drive with `dev/jigc-rig <state> --binary <path>`**, two-step eval, `cd "$REPO"`, never `rm -rf` a
   variable path; the rig drives the debug binary by default — pass `--binary ~/.local/bin/jigc` for the
   release posture. The rig gained a git-state builder for the nine `InProgress` states at M52
   Increment 2 (see `dev/jigc-rig --help`).
6. **The gate runs bare and in the background**; `dev/gate --private-target` when a concurrent gate holds
   the shared `target/` lock. One pre-existing probe race reddens `doc_code_gate::…probe_present…` on a
   full run when a parallel group relinks the shared probe binary (recorded by the increment-9 validator
   as advisory A2); a re-run is green. Worth a standing fix, not in M53's scope.
7. **Fixers one at a time**; brief each with the finding, never with the finding's boundary — every M52
   fixer found a larger class (5 homes for 1 · 12 sites for 1 · 9 doors for 2 · 5 doors for 3 · 10
   refusals for 6 · 3 states for 5).
8. **Zero schema-hash movement, zero `schema-version` bumps, zero corpora** — M52 held it across the whole
   wave and the negative fence (`git diff <base>..HEAD --stat -- <both packs' schemas, both manifests,
   every schema-snapshots/>` empty) is asserted at the close.
9. **The bump happens after the completion audit's fixes, not before**; the version-stamp confirmation
   has caught it unshipped in six consecutive waves. `foldback_truth.rs` pins the newest `**M<nn> —`
   span of CLAUDE.md: the close increment re-aims it to `**M53 —` in the *built, not audited* direction,
   the completion close inverts it back and requires the VERDICT citation.
10. **An agent's report is a lead, not a measurement.** Both mid-build halts and the Increment 9
    validator's two blocking findings were contradictions inside the record's own text, found by driving.

## Where everything is

| | |
|---|---|
| The M53 charter (four rows with fix shapes, the exit rule, the 1.x triage, the port direction) | [decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.17 fix pass (M53)* |
| The evidence base | [M52/per-axis-review/](../M52/per-axis-review/README.md) — README (the row-by-row comparison against M51's 39, §A's 27 findings by tier, coverage), `axis-1.md` … `axis-8.md`, `codex/`, `instrument/` (re-pointed, with the two lessons) |
| M52's audit and its fixes | [M52/VERDICT.md](../M52/VERDICT.md) + `M52/audit/`; fix commits `79e54c75` · `6d95756c` · `fe8f29c4` · `c96137e4` · `b9ab6a70` · `1b036264` |
| M52's planning artifacts (the mold, scaled down for a fix pass) | [M52/](../M52/) — settle-record (D1–D13, §1–§19), baseline-ledger + 7 companions, gap-findings + 4, advocates/, acceptance-design, planning-gate-record, design-review + codex-design-review |
| The build harness and its resume rules | `.claude/workflows/milestone-build.js` (header block) |
| The why, dated | `DECISIONS.md` → the 2026-09-17/21 M52 entries and the 2026-09-21 exit-rule entry |

## The prompt for the next session

> Read `completions/artifacts/M53/handover.md` first, then `implementation/decisions-pending.md` →
> *The rc.17 fix pass (M53)* (the exit rule, the four rows with their fix shapes, the boundary), then
> `completions/artifacts/M52/per-axis-review/README.md` §A Tier 1 (the four rows' repro blocks).
>
> Then plan M53 with `/milestone-plan M53` as a **fix pass, not a wave**: the baseline drives the four
> cells on the installed rc.16 to confirm them at HEAD; no gap fan-out over new mechanism; a Settle that
> confirms the charter's fix shapes, with a robust-advocate only where a shape is genuinely forked; halt to
> me if any fix needs new mechanism, with its sibling cells enumerated; fill the gate-record; four
> increments plus a close, the gate-record's rows diffed against the roadmap's Grouped scope. Zero
> schema-hash movement.
>
> Then build M53 with the milestone-build workflow: `args: { milestone: 'M53', base: '<HEAD after
> planning>', model: 'opus' }`, re-passed on every resume. Report the audit's findings before fixing them;
> fixers one at a time; the VERDICT under `completions/artifacts/M53/`; only after the audit's fixes the
> bump to `1.0.0-rc.17`, the install, and the goldens checked to move by the version string only.
>
> Then the **partial** re-review — axes 2, 3 and 5 only — on the installed rc.17 with the instrument at
> `completions/artifacts/M52/per-axis-review/instrument/` re-pointed, persisted under
> `completions/artifacts/M53/per-axis-review/`, compared row by row against M52's §A for those axes, and
> stop — the 1.0.0 call is mine.
