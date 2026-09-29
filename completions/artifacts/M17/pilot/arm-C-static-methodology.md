# Arm C — static-methodology frozen content

**Recorded artifact, curated once before any arm runs** (measurement.md → The comparison protocol).

Arm C isolates *methodology content* from *jigc's dynamic composition + write channel*. It is
galey's existing `CLAUDE.md` (= arm B, the product baseline) **plus** the block below appended
verbatim — the methodology's `dev-task` guidance hand-frozen as static prose, stripped of every
jigc command mechanic (`{{ cli.* }}` placeholders, `jigc` invocations, the CLI write path).

- **Arm B** = galey `CLAUDE.md`, untouched.
- **Arm C** = galey `CLAUDE.md` + the block below.
- **Delta B→C** = exactly this block (the methodology, written down) — so an A-beats-C result
  attributes to *jigc*, an A-beats-B-but-not-C result attributes to *written-down methodology*.

Source: the `dev-task` workflow's steps (`packs/methodology/steps/{scope,implement,gate,finalize}.yaml`),
flattened to static instructions. Frozen at jigc `main @ 743242d`.

---

## Development methodology (per-task discipline)

Apply this discipline to the task in this session. It is the team's standard per-task
discipline, written down here for you to follow directly.

1. **Scope before touching code.** Restate the task's intent in your own words, then name the
   single observable done-criterion — a test, a command that exits cleanly, or a behaviour you
   can point at. If your restatement reveals a different problem than the intent asked for, stop
   and ask before proceeding: a clarifying question costs less than solving the wrong problem.

2. **Implement test-first.** Write the failing test first and confirm it fails for the right
   reason — the assertion you care about, not an incidental compile error standing in for it.
   Only then write the minimal implementation that makes it pass. Refactor while green, touching
   only what this task needs.

3. **Run the project's own gate.** Run this project's test, lint, and build gate — the commands
   it already uses to prove a change is sound — and confirm every one passes before you finish.
   A green gate is what separates a finished change from one that merely compiles in your head.

4. **Land exactly one logical commit.** Write a Conventional-Commits message: a type, a scope
   (the area this change touches), a concise subject line, and a body explaining why the change
   is needed. Stage and commit only the files this task required.
