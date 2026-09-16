<!-- M51 per-axis review — axis 2 · Codex source pass, verbatim · read against `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

## Claims

1. `jigc relocate` can run `git rm --cached` on a committed destination squatter without the required posture re-probe immediately before that index mutation.

   - Evidence: `relocate_one` calls `displace_foreign_squatter` before the guarded `move_doc` at `crates/cli/src/relocate.rs:241-258`. The displacement path executes `git rm --cached --ignore-unmatch` at `crates/cli/src/relocate.rs:280-282, 308-323`. The posture re-probe exists only later inside `move_doc`, immediately before `git mv`, at `crates/cli/src/relocate.rs:65-79`. Therefore a concurrent merge/rebase/bisect beginning after the door-level probe but before line 311 permits the `git rm` to mutate the index during that operation.
   - Proposed reproduction: prepare a stranded managed document whose new destination is occupied by a tracked-but-unmanaged file; run `jigc relocate <type> --from <old-home>` while synchronizing a second process to begin a merge after command dispatch but before displacement. Expected wrong behavior: `git rm --cached <destination>` succeeds and the squatter is moved into `.jigc/displaced/` despite the newly in-progress merge; only the subsequent `git mv` re-probe refuses.
   - Confidence: high.

## Consistent findings

I read `CLAUDE.md`, all of `repo.rs`, the complete `BEHALF_DOORS` table and its completeness assertions, `COMMITTING_DOORS`, and every production consumer found for `PostureSubject`, `SeamSubject`, `move_doc`, `git_commit_capture`, and the relevant git subprocess helpers.

- `BEHALF_DOORS` is total over the clap leaf tree through the bidirectional test described at `crates/cli/src/cli.rs:4533-4557`. Its committing rows are `setup`, `migrate-corpus`, `rename`, `task discard`, `task finalize`, four milestone record doors, and `milestone finalize`; mover rows are `relocate` and `config set` (`crates/cli/src/cli.rs:1838-2099`).
- All ten `COMMITTING_DOORS` rows are asserted to resolve to commit-on-behalf rows (`crates/cli/src/invocation_log.rs:130-170`; `crates/cli/src/cli.rs:4747-4794`). `setup` is intentionally the additional no-hook committing door.
- The ordinary hook-capable `git commit` site re-probes immediately before execution: `verify(Commit)` at `crates/cli/src/task.rs:5414-5423`.
- The `setup` `--no-verify` commit re-probes immediately before execution and carries only the unborn exemption (`crates/cli/src/setup.rs:2398-2428`).
- The fan-out `git merge --ff-only` re-probes the live checkout immediately before execution (`crates/cli/src/task.rs:5614-5619`).
- The sole production `git mv` primitive re-probes immediately before execution (`crates/cli/src/relocate.rs:65-79`).
- No production `git switch` or ordinary `git checkout` act was found; `checkout-index` is unrelated to changing HEAD.
- I found no `neither` door that reaches a commit or committed-file move.
- `DedicatedWorktree` cannot be constructed outside `task.rs`: its fields and `add` constructor are private (`crates/cli/src/task.rs:5753-5783`), while the dedicated seam constructor requires a live handle (`crates/cli/src/repo.rs:432-448`).
- The unborn exemption does not leak: `live_exempt` has one production caller, `setup`; ordinary commit callers use `SeamSubject::live`.
- `GIT_DIR` redirection is explicitly declared out, not silently omitted, at `crates/cli/src/repo.rs:38-55` and reiterated at `crates/cli/src/repo.rs:463-466`.