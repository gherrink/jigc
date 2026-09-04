# Session findings — the running ledger

Every entry here is a **lead** until [findings-verification.md](findings-verification.md) drives it
(*an agent's report is a lead, not a measurement* — DECISIONS 2026-08-20). Arm-writer reports,
blind-session feedback and `observe` output all land here first, in arrival order, with their
source. Classification under [protocol.md](protocol.md) §1 happens at verification, never here.

| id | source | lead | status |
|---|---|---|---|
| **W-1** | walk arm 21 (agent report, driven in-container) | `jigc milestone finalize`'s **text** surface prints the sub-task transient-commit-doc findings (`1799a2d`'s new gate) as **bare lines with no `blocking · <code> —` prefix**, while `jigc task validate` prints the prefix on the identical findings and `--format json` carries `code` + `key` correctly at the milestone door. Also: the codes are `schema-conformance.field-value-conformant` for an unset `type` (`""` not in the enum) and `required-slot-present` for `summary` — not `required-field-present`. | to verify |
| **W-2** | walk arm 02 cell C (agent report, driven in-container) | `jigc uninstall` over a **file** planted at a fan-out worktree path refuses (`uninstall.dirty-worktree`, *"could not read the leftover directory … Not a directory"*) — the audit's data-loss hole is closed, bytes survive — but the refusal is the fail-closed probe-error shape: it does not enumerate the directory leftover beside the file, does not name `--force` as the consent, and its route (*git on PATH / `git worktree remove`*) does not fit a plain file. Cell A asserts all three for the same door over a directory leftover. | to verify |
| **W-3** | walk arm 21 (agent report) | `migrate-corpus` over an rc.12 `milestone-record` names the v3 `workflow` leaf as `migrate-corpus.set-field-unfilled` with route *"no action needed — machine-maintained (set: on-transition)"* — an advisory that says no action is needed. Reads as designed; recorded so B-session readers do not score it. | observation |
| **W-4** | walk arm 14/21 (agent report) | `migrate-corpus` **commits on its own** (`committed … — only the migrated paths were staged`, subject `chore(jigc): migrate the managed corpus to the current schema versions`), stamp-only diff. As designed (M42). | observation |
| **I-1** | this session, apparatus | `walk.py` lost every arm's stdout across `--only` passes — exit codes kept, bar lines gone. **Fixed** (`7be81e1`): output persisted as `ARM-OUTPUT.txt`, re-rendered on later passes, fenced by `test_walk.py`. | fixed |
| **I-2** | this session, apparatus | `~/out/<arm>` names collide with the previous trial's out-dirs; `run-session.sh` refuses (correctly) and `observe` then scores the **old** evidence at the same path without saying so — B3 and B3-strict were relaunched under `M50-*` names. The refusal is right; the silent stale read is the wart. | recorded; relaunched |
