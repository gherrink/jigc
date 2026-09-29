# M32 — genuine-spawn artifact (the §24 orchestrator half)

The [milestone-completion-workflow](../../../implementation/milestone-completion-workflow.md) §24
hollow-spawn gate: a spawn-class milestone is not shippable on the automated
**N-process sim** alone — the orchestrator must drive the **real assistant
Task-tool spawn** against the same fixture and assert the committed tree matches
the automated golden **byte-for-byte**, witnessing the blackboard invariant. The
M32 e2e tester (headless) explicitly flagged it ran only the sim. This artifact
is the main-session orchestrator run that closes the gate.

**Disposition (human-gated, 2026-06-21):** run the genuine spawn — chosen over
accepting the sim, even though M32 changed **zero** launch/spawn/adapter/template
code (`git diff b5cbba4..HEAD -- crates/cli/src/adapter.rs crates/engine/src/compose.rs
crates/cli/pack/steps crates/cli/pack/workflows` is empty; M32's delta is the
CLI-internal `provision_on_first_entry` only). Belt-and-suspenders.

## Method

Binary: `target/debug/jigc` sha256 `47055ff9d2d1…` (built at HEAD `35b0606`).
Two **real concurrent sub-agents** spawned via the assistant Agent tool, each
handed the rendered Spawn directive verbatim and an exact-content fixture, told
to coordinate ONLY through `jigc` + `git add` in its own worktree and never read
the sibling area, and to STOP before finalize (the orchestrator owns the commit
boundary).

Fixture (`finalize.fan-out.squash: false`, milestone `Cache rework`, sub-tasks
added in NON-id order: `Area zed` then `Area low`):

| sub-task | re-entry directive | commit | code |
|---|---|---|---|
| `area-low` | `cd .jigc/worktrees/area-low && jigc workflow sub-task --task area-low` | `feat` · "rework the low cache path" / "Reworks the low cache path body." | `src/low.rs` |
| `area-zed` | `cd .jigc/worktrees/area-zed && jigc workflow sub-task --task area-zed` | `feat` · "rework the zed cache path" / "Reworks the zed cache path body." | `src/zed.rs` |

## Results

**Both genuine sub-agents** (run concurrently, separate Task-tool processes):
- re-entry `jigc workflow sub-task --task <sub>` → **exit 0**
- provisioning assertion `test -f .jigc/tasks/<sub>/docs/commit:<sub>.md` → **PROVISIONED**
  (in jigc_home / main checkout, not the worktree — the M31 WF3 split). On the
  pre-M32 code this doc is absent and the next step exits 1.
- `jigc doc set-field commit:<sub>#type --value feat` → **exit 0** (the old bug
  exited 1 "no staged instance — provision it first")
- `set-slot` summary + body → exit 0; `git add src/<x>.rs` in own worktree → staged
- **Blackboard invariant witnessed:** each agent confirmed it used only `jigc` +
  `git add` in its own worktree and never read the sibling worktree/task area.

**Orchestrator finalize** (`jigc milestone finalize cache-rework`, exit 0):
- 3 commits, oldest-first: `feat: rework the low cache path` · `feat: rework the
  zed cache path` · `Finalize milestone cache-rework (2 sub-tasks)` — **id-ordered**
  (low before zed, NOT add order), each per-sub-task commit carrying its authored body.
- tree files: `src/low.rs`, `src/zed.rs` (both sub-agents' disjoint code combined).

**The §24 byte-for-byte assertion** — genuine 2-agent spawn vs. a sequential
N-process replay of the identical fixture, same binary:

```
genuine-spawn tree = 579c315da3d4557af06ddb93ffb71b8c4dc7e7b3
sequential-sim tree = 579c315da3d4557af06ddb93ffb71b8c4dc7e7b3
MATCH ✅  (commit-message sequence also byte-identical)
```

The real assistant Task-tool spawn reaches the CLI and produces **byte-identical
committed output** to the automated sim. Gate closed — no hollow-spawn.
