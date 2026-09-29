# M17 Handover — next session: jigc self-hosting dogfood (flow 21) + the milestone verdict

**Written 2026-06-13** after the pilot's single-task three-arm comparison ran live and surfaced the
measurement's core structural finding. Branch `main`, tree clean, full gate green (fmt · clippy -D ·
77 suites · build). Environment **rolled back to normal** (see § State). The next session does the
**"Both"** path the human chose: run the **jigc self-hosting dogfood (flow 21)** to positively
demonstrate the differentiators, **then write the M17 verdict** incorporating both it and the
foreign-project tie.

## Read first — what the pilot established (the substance)

The single-task three-arm comparison ran live on `gherrink-galey` (intent: heading `maxLevel`
consistency). Full evidence in [`pilot/comparison-results.md`](pilot/comparison-results.md);
findings in [`DECISIONS.md`](../../../DECISIONS.md) (2026-06-13).

1. **All three arms landed a correct fix in <5 min. A ≈ C.** jigc (A) and static-methodology (C)
   both produced test-first + quality commits; GSD's *fast skill* (B) was quickest but skipped the
   test. jigc's **adapter routed the agent unprompted** (the adherence bet held); the **capture
   apparatus validated end-to-end** on real sessions.
2. **The structural finding (why A≈C):** `dev-task` has **no `allows-create`** — a lone code task
   produces only a transient `commit` and touches **none** of jigc's differentiators (managed docs,
   validation, drift, supersession). Decision-recording is **milestone-grained** (`planning`/
   `completion` only).
3. **The deeper wall (the spike caught it):** jigc's `planning`/`completion` workflows are **jigc's
   own self-hosting process** (they reason about doctypes, engine/CLI surface, the deferral ledger,
   jigc's invariants) — **not a general project methodology**. So on a foreign project jigc reduces
   to `dev-task` (≈ a static CLAUDE.md), and a **3-arm spine comparison is structurally impossible**:
   no single project fits both jigc's spine *and* GSD's (galey has GSD-native + no jigc pack; jigc
   has its pack + isn't a GSD project).
4. **Honest thesis implication:** jigc's value is **real but domain-pack-bound** — it shows at the
   spine grain *with a fitting pack*, **proven by self-hosting**, not by a foreign head-to-head.
   This survived **three "rethink the task" rounds** — it is a found result, not green-by-construction.
5. **Owed work (already logged, [`decisions-pending.md`](../../implementation/decisions-pending.md)):**
   a lightweight **increment-level workflow** with `allows-create`, for sub-milestone decision/doc
   recording — the grain gap the dogfood surfaced.

## What the next session does

### Phase 1 — jigc self-hosting dogfood (flow 21), where the spine actually fits

Run jigc's full methodology (planning → increment → completion) on **jigc's own next work**, measured
via the `dogfood-record`. This is the differentiator demonstration the foreign-project comparison
structurally couldn't give.

- **Recommended target (self-referential + real):** build the **deferred increment-level workflow**
  itself (the grain gap from finding #5) — jigc dogfoods its methodology *by building the workflow
  that would have helped*. A genuine jigc increment (pack authoring + likely an `allows-create` on an
  increment-grade workflow + engine touch), at the methodology's **native grain**. Confirm the target
  with the human at the start (they may pick a different jigc increment).
- **The run:** on a `/tmp` twin of the jigc repo (or a git worktree — never the working tree),
  `jigc setup` + the methodology pack, then plan → build → complete. The decision (e.g. the new
  workflow's shape / where `allows-create` lands) gets recorded via `author-decisions` (managed,
  validated `decisions-log`) — **the differentiator finally engaged**.
- **Seeds (measurement.md → seeded-failure obligation):** after the first promoting finalize, one OOB
  edit to a committed managed doc via `sed` (absorb channel) + one staged bad-finalize (must exit 3).
  Each registers exactly once; seeds validate the instrument, never the thesis.
- **Author the `dogfood-record` through jigc in the twin**, finalize to promote it, then export
  record + owner-artifact (transcript + raw hook log + tally output + hash manifest) to
  `completions/artifacts/M17/self-hosting/`.

### Phase 2 — the M17 verdict

Write the milestone's honest verdict (prose + the record's `green|red`), incorporating: the
foreign-project tie (jigc ≈ static at `dev-task`), the self-hosting demonstration (differentiators
engaged + measured), the structural finding (value is domain-pack-bound), and the named bounds (n=1,
unblinded judge, the lacon/repowise arm-B edge, the grain gap). **Run a Codex cross-model pass over
the verdict prose** (the meta-lesson: the green-but-meaningless blind spot applies to verdicts too).

Then the **milestone-completion audit** over the whole of M17 (the increments + the dogfood runs) —
the spine terminus. Note: P2 (lacon existing-docs) and P3 (greenfield) from the original measurement
plan are **open** — decide with the human whether the self-hosting run + the structural finding
suffice for the verdict, or whether P2/P3 still add signal given finding #3 (on foreign projects jigc
is dev-task-only without a domain pack).

## Mechanics to re-establish (the measured environment, for jigc-on-jigc)

All proven this session — just re-apply for the new twin:

1. **Binary:** `cargo build --release`; the pinned `jigc` is sha `8d7ab262…` (currently the real
   binary at `~/.local/bin/jigc`). For capture, **re-install the jrun wrapper**: move the real binary
   to `~/.local/bin/jigc.real`, copy `implementation/dogfood/jrun` to `~/.local/bin/jigc`. (Needed
   because the Claude Code Bash `PostToolUse` payload carries **no exit code** — verified; jrun
   captures the real exit. All hardened: logs-before-passthrough, argv[0]=jigc, loud-on-misconfig.)
2. **Hooks:** the twin's `.claude/settings.json` gets the `Write|Edit` → `log-event.py` capture hook
   (jrun handles the jigc channel; **no Bash hook** in jrun mode — it'd double-log). Per-run log
   **outside** the twin.
3. **Environment isolation:** dedicated clean Claude home (`CLAUDE_CONFIG_DIR`) — no mem, no global
   methodology; **but** this is jigc-on-jigc, so jigc's *own* `CLAUDE.md` (the project instructions)
   stays. Re-check at startup: no claude-mem "recent context" block. (Mem is currently **back on** in
   `~/.claude` — disable in whatever home the measured run uses, restore after.)
4. **Pack supply:** `JIGC_PACK_DIR=<jigc>/packs/methodology` (relative `packs.yaml` paths drop
   silently — a tracked deferral). **Smoke-check** the capture (`tail -1` the log → non-null exit)
   before burning the run.

## Reusable assets (don't rebuild)

- The **apparatus**: `implementation/dogfood/` (jrun + log-event.py + tally.py + README + hooks.json),
  proven end-to-end, 9 passing tests in `crates/cli/tests/dogfood_apparatus.rs`.
- The **pre-registration / runbook templates** under `pilot/` — adapt for the self-hosting run.
- The **pinned binary** (sha `8d7ab262`), already built.
- The **galey twins** at `/tmp/jigc-dogfood/pilot/` and the clean home `/tmp/jigc-dogfood/clean-home/`
  — disposable (the comparison evidence is already exported to `pilot/`); reuse the clean-home shape
  or delete.

## State at handoff

- `main` clean, gate green, 14 commits this session (`2a025a5`…`665827e`).
- Environment **restored to normal**: `~/.claude/CLAUDE.md` back, claude-mem **re-enabled**, `jigc`
  un-wrapped to the real binary, **original galey untouched** (M12 held).
- Artifacts committed under `completions/artifacts/M17/pilot/`: `pre-registration.md` (amended twice,
  pre-run), `arm-C-static-methodology.md`, `arm-AC-project-facts-CLAUDE.md`, `runbook.md`,
  `comparison-results.md` (full three-arm evidence + the structural findings).
- Findings in `DECISIONS.md` (2026-06-13) + the owed workflow in `decisions-pending.md`.

## Traps

1. **Don't re-run the foreign-project 3-arm spine comparison** — it's structurally confounded (finding
   #3). The self-hosting run is the differentiator test.
2. **The grain fit is the whole point** — self-hosting works *because* jigc's spine is jigc's process.
   Keep the dogfood target a genuine jigc increment at milestone/increment grain.
3. **Seeds validate the instrument, not the thesis** — organic facts + the managed-artifact quality
   are the evidence; planted events never enter the organic counts.
4. **The verdict must name non-green honesty** — the foreign-project tie and the domain-pack-bound
   conclusion are *the result*, not a failure to hide. Codex-pass the verdict prose.
5. **Re-disable mem + smoke-check exit-capture** before any measured session (mem is back on now).
