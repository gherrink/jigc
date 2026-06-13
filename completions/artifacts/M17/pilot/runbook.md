# M17 Pilot — Runbook (human-driven runs)

Companion to `pre-registration.md` (frozen) and `measurement.md` (the protocol). This is the
operator guide for the live runs. **Arm order is fixed: B → A → C** (coin-flip, recorded).

## What's already set up (by the orchestrator session)

- **Pinned binary** `jigc` sha `8d7ab262…` at `~/.local/bin/jigc.real`; `~/.local/bin/jigc` is the
  **jrun** wrapper (the Bash `PostToolUse` payload carries no exit code — verified — so jrun
  captures the real exit). `doc-code` sha `f56032c2…` on PATH.
- **Three twins** at galey baseline `4147a36`, all **A5-green** (`pnpm test`), hook-free, clean:
  - `arm-A-jigc` @ `2db6026` — `jigc setup` + pack wiring + Write|Edit capture hook; `CLAUDE.md` =
    galey **project-facts** (GSD scaffolding stripped) + jigc adapter.
  - `arm-B-control` @ `4147a36` — galey **untouched** (full GSD `CLAUDE.md`); capture hook git-excluded.
  - `arm-C-static` @ `1ad5c6a` — galey **project-facts** (GSD stripped) + frozen methodology; hook git-excluded.
- **Capture:** Write|Edit → `log-event.py` (file_op channel, all arms); jigc → jrun (jigc-event
  channel, arm A). Per-arm logs at `/tmp/jigc-dogfood/pilot/logs/arm-{A,B,C}/hook-log.jsonl`
  (outside the repos). Launch scripts `run-arm-{A,B,C}.sh` export the needed env.
- **Setup decisions recorded:** twins are pre-built (`dist/` present; vitest resolves cross-package
  imports from it — a from-scratch `pnpm -r build` races a dev-dep cycle, so `--workspace-concurrency=1`
  was used once). `pnpm lint` has pre-existing biome drift at baseline (constant across all arms,
  left as-is — A5 is `pnpm test`, not lint). No lefthook git hooks (removed for arm/jigc symmetry).

### Environment isolation (contamination control)

**Home per arm** (the launch scripts set `CLAUDE_CONFIG_DIR` for you):

- **Arm B → `~/.claude`** (the **GSD home**): GSD agents/skills/hooks active — galey's real setup,
  the incumbent. claude-mem **disabled** here for the run window.
- **Arms A, C → `/tmp/jigc-dogfood/clean-home`**: no plugins (no mem), no GSD, `bypassPermissions`
  to match arm B. Auth reuses your token; if it prompts at first launch, `claude login` once there.

What this neutralizes (the same on every arm):

- **claude-mem off** — memory injection is the worst carryover vector (cross-session knowledge of
  the task/jigc bleeding in). Off in both homes. **Verify at each arm's startup: no "recent context"
  / memory block appears.**
- **Personal global methodology moved aside** — `~/.claude/CLAUDE.md` (`@LACON @PRINCIPLES`) loaded
  into every session and `PRINCIPLES.md`'s test-first content overlaps arm C's methodology. Moved to
  `~/.claude/CLAUDE.md.dogfood-bak` for the run window (does **not** touch GSD, which lives in
  agents/skills/hooks).
- **GSD is NOT contamination** — it is arm B's content. Only B has it.

**Restore after the pilot** (re-enables mem in `~/.claude`, restores the global `CLAUDE.md`):
`bash /tmp/jigc-dogfood/RESTORE-AFTER-PILOT.sh`.

**Recorded bound:** the GSD home also has `lacon` + `repowise-augment` (general productivity tools)
that A/C lack — a conservative bias *toward* arm B. Noted, not replicated.

## The matched intent (verbatim, all arms)

> In `@galey/extension-heading`, the `heading()` factory accepts a `maxLevel` option (1–6,
> default 3). The keymap bindings (`addKeymap`) and the `parseDOM`/`toDOM` tag handling already
> honor `maxLevel`, but the accessibility keyboard contract (`aria.keyboard`) and the theme keys
> (`themeKeys`) are hardcoded to three levels. Make both honor the configured `maxLevel`, so that
> `heading({ maxLevel: 6 })` advertises keyboard shortcuts and theme keys for all six levels (and
> the default `heading()` still exposes exactly three).

**The only allowed clarification** (verbatim, if and only if the agent asks; same on every arm):

> Scope is exactly: `aria.keyboard` and `themeKeys` honoring the configured `maxLevel`. Leave the
> toolbar, the message catalog, and everything else unchanged. Use your judgment on the wording of
> the new `aria.keyboard` descriptions.

No other steering, hints, or corrections. Off-the-rails behaviour is **recorded, not corrected**.
Stop at the first landed commit / a non-pre-registered block / ~45 min · ~40 turns.

## Phase 1 — the three-arm comparison (flow 22)

Run each arm as a **fresh, isolated session** (separate `claude` launch). Do them in order
**B → A → C**.

### Each arm, the procedure

1. **Launch:** `bash /tmp/jigc-dogfood/pilot/run-arm-<X>.sh` (it `cd`s into the twin, exports env,
   starts `claude`).
2. **Mini-smoke (first, before the task):**
   - **Environment check (arm-aware):** every arm — confirm **no claude-mem "recent context" block**
     and **no `PRINCIPLES`/`LACON`** in the loaded instructions. Then: **arm B** should show **GSD
     active** (gsd skills/agents available; galey's full GSD `CLAUDE.md`) — that's expected, it's the
     GSD arm. **Arms A and C** should show **no GSD** (stripped project-facts `CLAUDE.md` + their tool
     layer). If mem/PRINCIPLES leak anywhere, or GSD is missing on B / present on A/C, stop and fix.
   - Arm A: the `SessionStart` hook runs `jigc start` on launch → the log should already hold one
     `jigc` event with **`exit` non-null**. Confirm: `tail -1 /tmp/jigc-dogfood/pilot/logs/arm-A/hook-log.jsonl`.
     If `exit` is `null` or the file is empty, STOP — the capture is misconfigured (don't burn the run).
   - Arm B/C: after the agent's first Write/Edit, confirm a `file_op` line appears in that arm's log.
3. **Issue the task:**
   - **B and C:** paste the matched intent verbatim as the first message.
   - **A:** instruct the agent to run the task through jigc's dev-task workflow — e.g. *"Run this as a
     jigc dev-task (`jigc start --workflow dev-task`) and follow the composed steps,"* then the intent
     verbatim. (Running it as one `dev-task` is the pre-registered like-for-like session shape.)
4. **Attend** for the one clarification only. Otherwise hands-off.
5. **End** at the stop condition. Record wall-clock + turn count + anything notable (drift, friction,
   where it strained) — ceremony cost is data on every arm.
6. **Capture, do not delete:** the twin's commit stays in the twin; the hook log stays in
   `logs/arm-<X>/`. Tell the orchestrator session "arm X done" — it tallies and records.

After all three: the orchestrator tallies each log, the judge (you, LLM-assist ok) scores each arm
against the rubric in `pre-registration.md`, and the verdict (prose + `green|red`) must name **≥1
non-seeded thesis observation**.

## Phase 2 — the full-methodology spine + seeds (flow 21)

The dogfood of the whole methodology on the jigc twin (`arm-A`), measured separately from the
comparison. This is the *surrounding* run: planning → increment → dev-task → completion through the
composed pack, with the matched-task dev-task inside it. **Coordinate with the orchestrator session
before starting** — it will help drive the seeds and author the record.

- **Seed 1 (OOB edit):** *after the first promoting finalize* (once a methodology doc is committed,
  e.g. the roadmap), make a conformant edit to that committed doc **outside Write/Edit** — `sed -i`
  in Bash — so it rides the `reconciliation.absorb` channel. Must register **exactly once**.
- **Seed 2 (bad-finalize):** stage an integrity violation on a later task and `jigc task finalize`
  it — must **exit 3** (validation-blocked) and the tally must count it.
- Seeds land in the record's `seeded-*` fields and **never** enter the organic facts.
- End by authoring the `dogfood-record` **through jigc in the twin**
  (`jigc start --workflow record-dogfood "<run>"`), finalize to promote it, then the orchestrator
  exports record + owner-artifact (transcript + raw log + tally output + hash manifest) to
  `completions/artifacts/M17/pilot/`.

## Phase 3 — the pilot gate (orchestrator assembles)

All four must hold (`measurement.md` → The pilot gate): facts assembled for every arm · both seeds
registered exactly once · comparison judged per the rubric with ≥1 non-seeded observation named ·
record authored-in-twin + exported. The gate validates the *substrate*; failing it means fixing the
methodology, not pushing on. At the gate, you name **P3's greenfield intent**.

## If the mini-smoke shows `exit: null` on arm A

It shouldn't (jrun is installed and verified), but if it does: jrun isn't on PATH as `jigc`, or
`JIGC_DOGFOOD_HOME`/`JIGC_DOGFOOD_LOG` weren't exported. Check `which jigc` → `~/.local/bin/jigc`
(the wrapper), and that `run-arm-A.sh` exported all three vars. jrun warns to stderr if it can't
reach `log-event.py`.
