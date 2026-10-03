# M55 — genuine-spawn artifact (the orchestrator half of the spawn-class audit)

**Result: MATCH.** The committed tree of a real, concurrent Agent-tool spawn of three sub-agents equals
the automated N-process sim's golden byte for byte: tree `d30d6110b5b4ec4eb9fed298f4db51942503122e`.
Run 2026-10-03, about 17:32 UTC, at HEAD `72f54033` — the tip after every completion-triage fix.

## Why it exists

M55's deliverable includes a fan-out whose sub-tasks file findings, and the launch of each sub-agent
is the assistant's, not the CLI's. The [milestone-completion workflow](../../../../implementation/milestone-completion-workflow.md)
→ Audit makes such a milestone **spawn-class**: the e2e auditor's N-process sim proves the CLI payload
and the join, never that the real assistant primitive reaches the CLI, so the milestone is not
shippable until the orchestrator drives the real spawn against the sim's fixture and the trees match.
The M55 e2e auditor, being headless, ran only the sim (its scenario *Flow D*) and said so in its
summary. This is the half it did not run.

## What was run, and by whom

- **Who:** the orchestrator's **main session**, using its own Agent tool to launch three
  general-purpose sub-agents **concurrently** — a real spawn, not a subagent simulating one.
- **Fixture:** flow 58's, unchanged — `crates/cli/tests/flow58_several_reporters.rs`,
  `several_reporters_land_through_one_join_by_task_id_in_either_filing_order`. Milestone *File the
  findings together*, three report sub-tasks: `report-the-staged-sweep` and `report-the-sweep-again`
  (both `report-jigc-feedback`, titles differing only by a trailing `!`, so both mint one slug) and
  `report-the-eviction-disagreement` (`report-inconsistency`, three sides). The field and slot values
  are `findings_workflows.rs`' `file_jigc_feedback` / `file_inconsistency`. Nothing here is a new
  fixture ([common.sh](common.sh) says so in its header).
- **Binary:** a pinned copy of `target/debug/jigc` as `cargo build` linked it at `72f54033`
  (`jigc 1.0.0-rc.22`, sha256 `771116a4…`), reached through a shim first on `PATH`. A copy, because a
  `cargo test` relinks `target/debug/jigc` mid-run; the `cargo test` build lands the same tree
  ([golden.txt](golden.txt)).
- **Determinism:** every rig commit before the spawn runs under a fixed author and committer date, so
  the milestone's `base:` sha is the same in every rig. The docs' `date:` is the CLI's UTC day and
  cannot be pinned, which is why [after.sh](after.sh) re-derives the golden at comparison time and
  binds on that one.

## The sequence

1. [prepare-genuine.sh](prepare-genuine.sh) built a fresh rig (`dev/jigc-rig fresh`), created the
   milestone, added the three sub-tasks, ran `milestone provision` and `milestone execute`, saved the
   execute output ([execute-output.txt](execute-output.txt), the three `Spawn:` lines verbatim) and
   wrote one brief per sub-task.
2. The orchestrator launched the three sub-agents at once, each with only its brief:
   [brief-report-the-staged-sweep.md](brief-report-the-staged-sweep.md) ·
   [brief-report-the-sweep-again.md](brief-report-the-sweep-again.md) ·
   [brief-report-the-eviction-disagreement.md](brief-report-the-eviction-disagreement.md). Each runs its
   `Spawn:` line verbatim, files exactly one finding through per-leaf `jigc doc` verbs, reads it back,
   and stops — no finalize, no join, no `git`, no file read or write outside `jigc`.
3. Their reports are condensed in [subagent-reports.md](subagent-reports.md): every command exited 0
   (10 · 10 · 13 commands), each create acked the expected address, and no sub-agent saw a refusal, a
   warning or a command that differed from its brief.
4. After all three returned, [after.sh](after.sh) witnessed the worktrees, ran `milestone join` and
   `milestone finalize`, and compared trees. Its output is [after-genuine.log](after-genuine.log).
5. The golden: [sim.sh](sim.sh) runs the same fixture as an N-process sim, each `Spawn:` line through
   `sh -c` and each sub-task filed from its own worktree. [golden.txt](golden.txt) records six sim runs —
   by-id and reverse filing order × exact and heredoc slot payloads × with and without an explicit
   join — all on one tree.

## The result

```
genuine-spawn tree      = d30d6110b5b4ec4eb9fed298f4db51942503122e
sim tree (re-derived)   = d30d6110b5b4ec4eb9fed298f4db51942503122e
sim tree (golden.txt)   = d30d6110b5b4ec4eb9fed298f4db51942503122e
MATCH
```

The join merged 6 docs (3 commit docs, 3 findings) and gave the colliding slug's `-2` suffix to the
higher task id, `report-the-sweep-again`. `milestone finalize` made one commit of 4 files — the three
findings and the milestone record — and left the main checkout clean.

## The blackboard witness

Two pieces of evidence, neither of which a permutation test can supply:

- **Each sub-agent's own statement**, asked for in its brief's report section and recorded in
  [subagent-reports.md](subagent-reports.md): it reached state only through `jigc` calls, read or
  listed no path outside its worktree (the only file read was the brief), edited no file directly and
  ran no `git` command.
- **The orchestrator's pre-join check** in [after-genuine.log](after-genuine.log): each sub-task
  worktree's `git status --porcelain --untracked-files=all` was empty, and each sub-task's doc was
  listed as that task's staged doc by `jigc doc list --task`.

**Bound:** the first piece is the sub-agents' self-report, and the raw sub-agent transcripts are not
committed; [subagent-reports.md](subagent-reports.md) is the orchestrator's condensation of them. The
second piece shows no file was written outside `jigc`. Neither proves that a sub-agent never *read* a
sibling's area — that rests on the self-report.

## Redaction

The files are copied from the session's scratchpad with every absolute local path replaced: the rig
root by `<RIG>`, the scratchpad by `<scratchpad>` and the jigc checkout by `<repo>`. Nothing else
changed. The scripts are recorded as run, so they will not run until those three placeholders are put
back.
