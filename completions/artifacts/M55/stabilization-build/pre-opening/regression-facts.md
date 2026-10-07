# Facts for the design of the scripted regression set

*Returned 2026-10-06 by a read-only exploration at tree tip `a5a576a7` (four files uncommitted at the time). Nothing was built or run; counts are grep or awk censuses. Transcribed by the orchestrator. Paths are repo-relative.*

**The instrument's definition (the human's):** no command that works on the previous release in a supported layout stops working, and every refusal's route works as printed. Ruled: a deterministic script, run in full on every candidate; the candidate is a local build identified by its commit, compared against the previous release's binary built from its recorded commit.

## 1. Registries — routes are not enumerable
- `VERB_KINDS` 48 leaves, 12 Read (`crates/cli/src/cli.rs:2014`); `BEHALF_DOORS` 48, of which 10 commit and 2 move, each with a runnable argv (`cli.rs:2184`).
- `COMMITTING_DOORS` 11 (`invocation_log.rs:196`); `ERROR_CODE_REGISTRY` 12 (`:275`); `DESTROYING_DOORS` 6 (`milestone.rs:4634`); `AMBUSH_CONTRACTS` 7 (`pack.rs:989`).
- `EXIT_CODES` 5 (`task.rs:131`); `STORE_EXIT_FLIPS` 7 (`render.rs:1092`); `ENVELOPE_ARMS` 66 rows over 49 paths (`render.rs:7046`).
- Route-bearing: only `CONFORMANCE_ROUTE_CODES` 5 (`engine/src/validate.rs:3421`) × `BOUNDARY_DOORS` 3 (`render.rs:1482`). Otherwise a route is a `Route{text,kind}` built at its producer and flat on the wire (`engine/src/finding.rs:717-742`).
- No finding-code registry; `CHECK_INVENTORY` (31) covers probe checks only.

## 2. End-to-end drivers
- **Flow suites:** 55 files, 283 tests. The binary is compile-time `env!("CARGO_BIN_EXE_jigc")` (`support/trial_corpus.rs:458,504`); no env override. They link the tip's `cli::`/`engine::`, so cannot take a second binary without a seam.
- **`trial_corpus.rs`:** `State::ALL` (`:176`) = fresh, committed-singletons, migrated, refs-post-hoc, chatty-hooks, vendored.
- **`dev/jigc-rig`:** the same six plus `bare`, 17 `--git-state` overlays, and `--binary <path>` (`:101,1041`; present at rc.24 too). Usable against both binaries today.
- **`dev/runner-faithful`:** `post_install` (`:264-325`) runs version, setup, probe-override, control-finalize, drift-finalize on whatever `jigc` is on PATH; Linux container only.
- **`trial-driver/walk.py`:** 24 arms through `run-session.sh --exec` on a Docker image `--tag` (`:143`); two-image capable; 9 arms assert rc-specific expectations.
- **`run.py observe`:** reads agent-session logs; not a comparer. **`trial-corpus-template`:** a pre-jigc TypeScript repo, binary-independent.
- **`trial-harness/verify-pair.sh`:** the one existing old-vs-new differential; each probe declares both sides (`:12-21`); no probe set past rc.13→rc.14.

## 3. Route-as-printed exists only piecemeal
- `route_followability.rs:1-43` runs emitted argv for three families plus the 5×3 axis.
- `support/mod.rs:142` `shell_words` is the shared real-`sh` splitter (15 suites).
- `path_arg_occurrence_axis.rs:736` holds one of about 38 separately named run-the-span functions.
- Nothing sweeps every refusal. The fixers drove by hand on `target/release/jigc` in rig roots (`fixer-reports-round4.md:50,360`; six `routes_driven` sections).
- `pinning.md:34-54` gives the YAML repro block and rejects a script runner (reason to be confirmed).

## 4. What newly refuses against rc.24
`DECISIONS.md:490`: 7 clauses, 9 conditions, about 17 door × shape cells, plus `uninstall` refusing less — a `created` doc over a git-held home (three doors); a contested and a worktree-staged boundary destination; a migration source edited after the mint; `setup` where git cannot answer and over four link shapes; `discard`, `uninstall`, `provision` under `showUntrackedFiles=no`; `milestone finalize` in a moved repository; `migrate-corpus` over a non-regular destination. Prose only, round 4 only: rounds 1–3 carry nine more *Now refuses* clauses (`DECISIONS.md:562-578`). No machine-readable list.

## 5. Contracts a script could compare on
Exit codes 0–4 (`command-output-contract.md:473-485`); stream rule: parse stdout, if empty parse stderr (`:452-458`); finding key `{code,target}` (`:201`); 59 arms Pinned, 7 Unpinned (`describe`, `milestone list-tasks`, five milestone write acks). Both commits carry 66 arms and 48 verbs; 0 of 718 compose goldens moved since rc.24.

## 6. The previous binary
Annotated tag `jigc-v1.0.0-rc.24` (`e3a52c58`) peels to `91834b5e` (2026-10-03), 75 commits behind the tip. `build-image.sh` and `runner-faithful --commit` build Linux only. No committed script builds a host binary from a commit; only prose in `.claude/agents/stabilize-preflight.md:20-22` (`git archive`, release, `--locked`, own target).

## 7. Size
48 verbs × 7 states = 336 cells; 61 success arms × 6 = 366. 66 hand-written recipes already drive every arm, on three bases (`format_json_success_axis.rs:436`). Routes: 164 production constructor sites (`Route::mechanical(` 84, `human(` 74, `informational(` 6) — a census that misses `From<String>` routes, anyhow-string spans, and one site serving many codes.

## 8. Hazards
- Both binaries print `1.0.0-rc.24`: identity must be the sha or the hash.
- States are built by the binary under test, so two binaries may diverge before the compared command.
- Most newly-refusing cells need git config or a layout no named state builds.
- 74 human routes are prose, with placeholders and hand steps.
- Nondeterminism: `on-create` dates (`doc.rs:2830`), commit shas in envelopes, absolute paths in `git -C` spans (`support/route_spans.rs`).
- Release binaries omit the debug route fences.
- No TTY or agent is needed (*inferred*).

## Shapes the explorer named
- **Rig-matrix differential:** reuses `jigc-rig --binary`, registry argv, the exit and `{code,target}` contracts, and an allow-list to be written.
- **Recipe port:** reuses the 66 `ENVELOPE_ARMS` recipes and `shell_words`, behind a binary-path seam.
- **Declared-probe pair:** reuses `verify-pair.sh`'s both-sides probe form and the fixers' `routes_driven` lines.

## Follow-up facts (same exploration, 2026-10-06)

**The registered-argv matrix is not viable as a happy-path check.** Only 12 of the 48 `BEHALF_DOORS` rows carry an argv (the 10 committing and 2 moving doors); the 36 `Neither` rows, every read verb included, carry none (`cli.rs:2146-2148`). Its stated purpose is "every other argument present and well-formed, so the posture is the only thing the door can fault on" (`cli.rs:2102-2105`). `posture_door_axis.rs:236-300` runs them over 12 × 17 git states; proceeding cells assert only that no posture code appears, never exit 0. A registered-argv × state matrix would mostly compare two refusals.

**The recipes are the only real success paths, and they are Rust code bound to one compiled binary.** `drive: fn(&TrialCorpus) -> Output` closures (`format_json_success_axis.rs:436`), not data. Three bases — `Fresh`, `CommittedAdr`, `CommittedSpec` (`:113-123`) — built once and `copy_state`'d per recipe (`:132-181`); each closure then drives the remaining setup and the measured call itself. All 48 verbs have at least one success arm. Of the 66 recipes: 61 success, 3 adjudicated (`task finalize` blocked and review hold, `milestone finalize` blocked), 2 generic rejects. No per-refusal recipe set. Three things block running against a binary by path or on a state another binary built: the compile-time `env!("CARGO_BIN_EXE_jigc")` (`support/trial_corpus.rs:458,504`); setup and the measured invocation sharing one closure and one binary; the file linking the tip's `cli::`/`engine::` for its proofs (`:59-70`). Setup otherwise uses only the binary, `corpus.git` and `fs` writes.

**The rig has none of the ordinary-configuration or layout states the closing condition names.** Its 17 `--git-state` overlays are all un-concluded git operations or HEAD shapes (`dev/jigc-rig:62-83`): merge, squash-merge, rebase-merge, rebase-apply, am, cherry-pick, sequencer, dangling-sequencer, uncommitted-pick, -range, -conflicted, -resolved, revert, unmerged-index, bisect, detached, unborn. None is line-ending conversion, `status.showUntrackedFiles`, a symlinked `CLAUDE.md`, a linked worktree, a worktree of a bare repository, `--separate-git-dir`, a submodule, an old git, or a moved repository — all nine lack an overlay and are hand-built inside individual suites.

**`implementation/pinning.md:54`:** "**No script-runner harness:** blocks are converted to Rust tests by hand (they're small), not executed as YAML — a bash/YAML runner is a second test framework to maintain and a platform-fragility source; the block is a *specification for a test*, not a test format." Its subject is how a repro block becomes a standing test; its two reasons (a second framework, platform fragility) would apply to a bash differential as well (*inferred*).
