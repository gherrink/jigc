# Brief — the reshape plan's `K1b` (2026-10-27)

One builder, one commit (two if the plan's own text asks it) on `fix/rc24-tier1` in `~/projects/gherrink-jigc`,
from HEAD `f99658b8` (`git log -3`; a dirty tree stops you). **`implementation/working-principles.md` first.**

## Why now

The human ruled on 2026-10-27 that what is owed before the stabilization run's opening comes first. The reshaped `test`
stage's build plan, `completions/artifacts/M55/stabilization-build/reshape/plan.md`, held back eleven tasks "until the
parts have run" (`DECISIONS.md` → *2026-10-12 — The reshaped `test` stage is run for real in parts*, ruling 2); the
parts ran by 2026-10-18. They are now built in the plan's ruled order (line 135): `K1b` → `O4` → `T8` → `K7b` → `K6` →
`T9` → `T11` → `T12` → `O5` → `KR` → `T10`, then `J1`, `J2` with `O6`. **Yours is `K1b` only.**

## The task

`plan.md` → §5 *`K1b` — the lookup kit, the harness's half — and the report line of `KR`* (~line 315), with the table row
at ~line 128 and `K1a` (built) for what the kit is (`dev/stabilize-record kit --help`). Read the plan's §4 rules (red
first, a task carries the assertions it breaks, one full gate per commit, the harness regenerated with
`dev/stabilize-harness` and both files committed) and follow them. Verify the plan's line numbers against the code at
HEAD — they are from 2026-10-12 and the harness has moved since; where the plan and the code disagree, the code as it
stands and the ruling decide, and you say what you found.

Not yours: any edit to `.claude/agents/` (the definitions' half is `O4`/`KR`, the main session's). If `K1b` cannot land
without a definition edit, stop before the gate and return the exact text, old and new, byte for byte.

## Rules

The full `dev/gate` in the background, read unpiped — **never two gates at once** (no other gate is running; a review
that is read-only may be running). `git commit -F <file>` with the trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Message ends "(the reshaped `test` stage, task `K1b`)". `DECISIONS.md` newest first, one entry for the task as the
plan's other tasks have one (`LOG`). Push by name. No stash, no `rm -rf` on a variable path, never print an environment
variable's value, no host path / name / id in a file, the human is "they", no edits to `.claude/agents/`,
`.claude/settings.json` or `CLAUDE.md`.

## Self-check — report it

1. Every driving role's prompt names the kit as its first call — the fence arm's own count, and how you derived the set
   of driving roles (from `ROLES`, never hand-listed).
2. A run without an order: what of it changed, if anything, and the test that shows it.
3. Each plan line you found stale against the code, and what you did.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, the self-check, anything left open with why.
