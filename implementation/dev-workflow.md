# Dev workflow (TDD)

The loop for **building jigc** — how a single implementation task is taken from "not started" to "committed," test-first. This is distinct from the **design-collaboration loop** in [CLAUDE.md](../CLAUDE.md) → How we work together: that one governs *decisions* (discuss → converge → write a design doc); this one governs *code tasks*. It runs **per task within an increment** ([roadmap.md](roadmap.md)); tasks are cut when an increment is picked up. This is the **inner** loop: the [increment workflow](increment-workflow.md) is the outer one that cuts an increment's tasks, runs each through the loop below, then independently validates the increment and fixes until clean.

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

   **A green gate does not prove a proptest-guarded invariant holds** (see *A proptest is not a gate*, below). If your change touches behavior an existing property test guards, **pin the invariant with a deterministic case in the same commit** — and if you mean to *retire* an invariant, reconcile its proptest explicitly and state a rationale you have actually checked.
6. **Commit** — one logical change, conventional message. One task = one focused concern = one commit.

## Why this shape

Two framing notes, not extra steps:

- **One task, one concern.** The unit of work is a single focused thing — the smallest change with its own observable done-criteria. Keeping tasks small is what keeps the loop honest (a vague "make it work" task has no red step).
- **The loop is jigc in miniature.** Scope ≈ `jigc start` / compose, Gate ≈ `jigc task validate`, Commit ≈ `jigc task finalize` (one task → one commit, [finalize.md](../design/finalize.md)). Running it by hand is deliberate dogfooding — the friction we hit informs the real `single-task` workflow we're building.
- **One agent when orchestrated.** Run as a workflow, a single agent carries this whole per-task loop (scope → commit) and yields the one commit — the [increment workflow](increment-workflow.md) → Orchestration spawns one such agent per task.
- **A recorded rationale is a claim, not a fact** (M41, 2026-07-11 — [DECISIONS.md](../DECISIONS.md)). A justification written into `DECISIONS.md`, a doc-comment, or a commit message is **load-bearing**: the next reader — human or agent — trusts it and *stops questioning the choice*. So **never record a rationale you have not actually checked.** M41 produced **two false ones in a single wave**: an increment retired `slugify`'s idempotency invariant on the claim that *"the fixed-point set was exactly the valid-slug set within the caps"* (demonstrably false — and it is precisely *why* the un-reconciled proptest went unnoticed for two increments), and another wrote a doc-comment asserting a slug-only finding key was chosen *"so two dangling targets yield distinct keys"* (backwards — the full `<type>:<slug>` is strictly **more** distinct). Both passed a green gate; both survived an independent same-model review; the second fell only to a cross-model pass. A confident falsehood is worse than an admitted gap, because it **disables the next reader's skepticism**. If you cannot check it, write what you actually know and mark the rest open — an honest *"unverified"* is cheap.
- **A proptest is not a gate** (M41, 2026-07-11 — [DECISIONS.md](../DECISIONS.md)). `cargo test` runs the property/fuzz suites over **random** inputs, so a green gate means *"no counterexample was sampled this run,"* not *"the invariant holds."* An invariant guarded **only** by a proptest is therefore not gate-enforced: a change can break it, pass its own increment's gate, and surface runs later (or in the wild) once a run happens to sample the counterexample — which is exactly what happened to `slugify`'s idempotency in M41 (an edge-stopword change broke "a slug is a fixed point of normalization," passed its increment green, and was caught two increments later when the shrink seed persisted and made the failure deterministic). Two obligations follow: **(1)** a load-bearing invariant gets a **deterministic** case alongside its proptest — the proptest explores, the point test *enforces*; **(2)** an invariant is never retired by silence — if a change means to drop one, say so, reconcile the test in the same commit, and check the rationale (M41's break shipped a rationale that was demonstrably false, which is what let the un-reconciled proptest stand). Randomized coverage is a *finder*, not a *fence*.
