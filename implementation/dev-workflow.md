# Dev workflow (TDD)

The loop for **building jigc** — how a single implementation task is taken from "not started" to "committed," test-first. This is distinct from the **design-collaboration loop** in [CLAUDE.md](../CLAUDE.md) → How we work together: that one governs *decisions* (discuss → converge → write a design doc); this one governs *code tasks*. It runs **per task within an increment** ([roadmap.md](roadmap.md)); tasks are cut when an increment is picked up.

The loop is shaped as named, ordered steps on purpose: each maps onto a future jigc `step:` and the whole onto a future `workflow:dev-task`. We run it by hand now; when jigc can compose, it promotes into the workflow dialect almost verbatim ([workflow-dialect.md](../design/workflow-dialect.md)).

## The loop (one task)

1. **Scope** — restate the task in one line, and its **done-criteria as an observable check**: a test that passes, a command that exits clean. Read the design-doc section the task implements (the [roadmap](roadmap.md) increment names it). If the restatement reveals a different problem than the task assumed, stop and check (understand before acting; make success verifiable).
2. **Red** — write the failing test(s) that encode the done-criteria **first**. Run them; confirm they fail, and **for the right reason** (not a compile error standing in for the assertion). Parser / serialization work is **golden + property/fuzz** tests — the #1-risk discipline ([parsing.md](parsing.md) → Round-trip guarantees) lives here, not as an afterthought.
3. **Green** — the **minimal** implementation that makes the test pass. No extra scope, no speculative generality (keep it minimal).
4. **Refactor** — tidy while green, touching **only what this task touches**. Pre-existing mess is not this task's to fix (stay in scope).
5. **Gate** — the verification gate; all four pass or the task isn't done:
   - `cargo fmt --check`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo test`
   - `cargo build`
6. **Commit** — one logical change, conventional message. One task = one focused concern = one commit.

## Why this shape

Two framing notes, not extra steps:

- **One task, one concern.** The unit of work is a single focused thing — the smallest change with its own observable done-criteria. Keeping tasks small is what keeps the loop honest (a vague "make it work" task has no red step).
- **The loop is jigc in miniature.** Scope ≈ `jigc start` / compose, Gate ≈ `jigc task validate`, Commit ≈ `jigc task finalize` (one task → one commit, [finalize.md](../design/finalize.md)). Running it by hand is deliberate dogfooding — the friction we hit informs the real `single-task` workflow we're building.
