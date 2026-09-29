<!-- M52 per-axis review (re-run) — axis 2 · caller tokens — the CODEX SOURCE PASS, verbatim. Read against the repository at commit e519e4eb, 2026-09-21. It reads; it drives nothing. Every claim below is a LEAD until the reconciler drove it — see axis-2.md for the verdict on each. -->

## M52 source-pass result

No grounded source claim survives this pass. I found no posture-registry omission, misclassified `Neither` door, forgeable dedicated-worktree exemption, leaked `setup` exemption, or unguarded production `git commit`/`mv`/`merge`/`rm` seam.

## M51 §A row dispositions

1. **`codex-1` — CLOSED.** `displace_foreign_squatter` now constructs a live `SeamSubject` and calls `verify(SeamAct::Move)` immediately before `git rm --cached`; no filesystem displacement occurs until afterward. Evidence: `crates/cli/src/relocate.rs:790-818`. The subsequent `git mv` independently re-probes at `crates/cli/src/relocate.rs:217-240`.

2. **`D1` — CLOSED.** `posture` now checks an operation before detached HEAD, preventing rebase/bisect detachment from masking the actionable operation: `crates/cli/src/repo.rs:760-783`. `SeamSubject::verify` takes the first applicable breach at `crates/cli/src/repo.rs:659-675`.

3. **`D2` — CLOSED.** `git am` and rebase are distinct typed states. `Am` detects `rebase-apply/applying`, while apply-backend rebase excludes that discriminator: `crates/cli/src/repo.rs:181-190,252-268`. Their routes are respectively `git am --abort` and `git rebase --abort`: `crates/cli/src/repo.rs:312-348`.

4. **`D3` — CLOSED.** `InProgress::ALL` contains all nine declared Git states, including `CherryPick`, `Revert`, `Sequencer`, squash merge, and marker-less unmerged index: `crates/cli/src/repo.rs:173-222,225-244`. Cherry-pick and revert detection is explicit at `crates/cli/src/repo.rs:268-272`; operation-first posture prevents any committing door from reaching its commit seam under them.

5. **`D3b` — CLOSED.** The shared hook-capable commit seam classifies only Git exit `1` as a hook rejection and renders other failures as “`git commit` failed,” preserving Git’s streams separately: `crates/cli/src/task.rs:6540-6570,6605-6634`. All ordinary production commits funnel through this seam; `setup` is the documented `--no-verify` exception.

## Claims

None. Consequently there is no source-grounded wrong-behaviour reproduction to propose.

## Completeness and consistent findings

- `BEHALF_DOORS` is fenced as a bijection with every clap leaf at `crates/cli/src/cli.rs:4711-4750`; parsed leaf identity is independently checked at `crates/cli/src/cli.rs:4835-4880`. Acting rows also owe a parseable argv reaching their own leaf: `crates/cli/src/cli.rs:4883-4937`.

- The acting set remains complete: ten hook-capable `COMMITTING_DOORS` rows at `crates/cli/src/invocation_log.rs:171-222`, plus `setup`; movers are `relocate` and `config set`. `COMMITTING_DOORS ⊆ CommitsOnBehalf` is asserted at `crates/cli/src/cli.rs:4939-4986`, while `setup` and its sole unborn exemption are named separately at `crates/cli/src/cli.rs:4989-5014`.

- Production subprocess census for the requested Git subcommands:

  - `git commit`: shared seam at `crates/cli/src/task.rs:6605-6615`, immediately preceded by `subject.verify(Commit)` at line 6609.
  - `setup`’s `git commit --no-verify`: `crates/cli/src/setup.rs:2437-2441`, immediately preceded by `live_exempt(...HeadUnborn).verify(Commit)` at lines 2409-2418.
  - `git mv`: `crates/cli/src/relocate.rs:217-240`, immediately preceded by `verify(Move)` at line 238.
  - `git rm --cached`: `crates/cli/src/relocate.rs:790-816`, immediately preceded by `verify(Move)` at line 802.
  - `git merge --ff-only`: `crates/cli/src/task.rs:6814-6819`, immediately preceded by `live.verify(Commit)` at line 6818.
  - No production ordinary `git switch` or `git checkout` site exists. `checkout-index` is an index materialization command, not a HEAD-changing checkout (`crates/cli/src/task.rs:2357`).

- `SeamSubject::dedicated` requires `&DedicatedWorktree`, whose construction and fields remain private to `task.rs`: `crates/cli/src/repo.rs:613-629`; `crates/cli/src/task.rs:6970-6988`. The door-level classifier additionally requires a real linked-worktree `.git` file, the canonical worktree path, and a registered milestone subtask: `crates/cli/src/repo.rs:485-523`.

- `setup`’s exemption is confined to `live_exempt`, whose documentation says it has one production caller: `crates/cli/src/repo.rs:597-610`. All other live seams use `SeamSubject::live`.

- `RelocateRefusal::ALL` carries all ten refusal kinds and their finding-code mapping: `crates/cli/src/relocate.rs:60-118`. It introduces no alternate move path.

- The M52 rollback registry’s `config-root-relocation` population covers the move primitive’s prior homes, destinations, file-state rekey, and index rollback under `FileCas`: `crates/cli/src/rollback.rs:365-389`. This is transaction coverage, not a posture bypass.

- `TASK_AREA_FILES`, `DESTROYING_DOORS`/`Disposition`, `ENVELOPE_OWED_CODES`, `STORE_EXIT_FLIPS` including `schema-conformance.home-vacated`, `suppressed.door`/`workflow.verb-routed`, and the shared fixed-identity predicate do not add a commit/move seam in this axis. Their consumers contain no requested Git act outside the census above.

- The prior-home store sweep is included in validation and document enumeration at `crates/cli/src/cli.rs:1332-1436` and `crates/cli/src/doc.rs:4206-4344`; it does not bypass relocation posture.

## Schema-hash boundary and bounds

I compared the six manifest `schema-hash` values at M51 commit `577a0099` with the current manifest; they are byte-identical. I see **no zero-schema-hash boundary violation**.

Declared bounds remain:

- Ambient `GIT_DIR` redirection is explicitly out of posture identity enforcement: `crates/cli/src/repo.rs:38-55,644-647`.
- Unanswerable Git probes fail open by recorded design.
- This was source-only: I did not build or drive the binary, run mutation/race reproductions, or write any file.