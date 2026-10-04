<!-- The unseeded Codex source pass for ROW 5 (finalize / transaction), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

# ROW 5 · finalize / transaction — source pass

## CLAIMS

No new source-grounded completeness defect found in the M54/M55/trailer scope. In particular, I found no tier-1 path by which the doc-only arm commits an unrelated path, loses an exit-0 artifact, bypasses the hook-capable seam, or restores repository state unconditionally. Therefore there is no new wrong-behaviour reproduction to list.

## Baseline dispositions

1. **`(4, DEFECT 1)` — CLOSED.** `finalize.stage-failed` now constructs a mechanical re-run containing the task id and preserves `--approve` and `--carry-staged`; the debug route fence can therefore parse the actual argv ([task.rs:6579](crates/cli/src/task.rs:6579), [task.rs:6590](crates/cli/src/task.rs:6590)).  
   Former repro: induce a stage failure, then paste the route; rc.16 omitted `<ID>` and exited 2. Current source constructs `jigc task finalize <id> [flags]`. Confidence: high.

2. **`(4, DEFECT 2)` — STILL OPEN, tier 3.** `config.repoint-failed` still embeds the producer’s rendered `anyhow` chain directly as `cause` in its finding message; only the finding location is guaranteed repo-relative ([config.rs:599](crates/cli/src/config.rs:599), [config.rs:659](crates/cli/src/config.rs:659), [config.rs:671](crates/cli/src/config.rs:671)).  
   Repro: make `.jigc/config/manifest.yaml` unwritable, then run `jigc config set docs-root documentation --format json`; expect the blocking finding’s `message` to retain an absolute host path while `location.address` is repo-relative. Confidence: high.

3. **`(4, DEFECT 3)` / the still-open half of M51 C4 — STILL OPEN, tier 3.** `milestone create` calls `gitignore::ensure` before reading HEAD, minting the area, and attempting its record commit, but the returned `Ensured` value is only emitted on success; no `.gitignore` pre-image participates in the record-commit rollback ([milestone.rs:724](crates/cli/src/milestone.rs:724), [milestone.rs:733](crates/cli/src/milestone.rs:733), [milestone.rs:735](crates/cli/src/milestone.rs:735)).  
   Repro: trim `.jigc/.gitignore`, install a rejecting pre-commit hook, and run `jigc milestone create 'Amend probe'`; expect exit 1 with the union amendment left on disk and unnamed. Confidence: high.

## M52 §D open leads

1. **Stale-base ordering — CLOSED by source.** `milestone provision` reads and guards the recorded base before `gitignore::ensure`; task-list resolution also precedes it ([milestone.rs:2864](crates/cli/src/milestone.rs:2864), [milestone.rs:2870](crates/cli/src/milestone.rs:2870), [milestone.rs:2895](crates/cli/src/milestone.rs:2895)).

2. **Stage failure × concurrent worktree edit — STILL OPEN as a dynamic bound.** Source shows every relevant finalize pre-image is captured before promotion, retirement, ignore amendment, staging, and commit, and the common error arm restores them ([task.rs:4827](crates/cli/src/task.rs:4827), [task.rs:4841](crates/cli/src/task.rs:4841), [task.rs:4990](crates/cli/src/task.rs:4990)). It cannot establish scheduler behaviour for a real concurrent editor before the hook window.

3. **Genuine concurrent process racing a pre-image — STILL OPEN as a dynamic bound.** The restore primitive remains compare-and-swap and the transaction gathers conflicts, but a source pass cannot substitute for the scheduler/racer drive. No new unconditional restore was found.

4. **`milestone provision` post-ensure failure — STILL OPEN.** `gitignore::ensure` is followed by fallible `provision_worktrees`; acknowledgement is assembled only afterward ([milestone.rs:2895](crates/cli/src/milestone.rs:2895), [milestone.rs:2897](crates/cli/src/milestone.rs:2897), [milestone.rs:2904](crates/cli/src/milestone.rs:2904)).  
   Proposed drive: begin with a trimmed ignore file, force `git worktree add` to fail after the amendment, and check whether the surviving amendment is unnamed. This is historical surface residue, not a new M55 tier-1 loss path.

## Consistent completeness reads

- The arm predicate is centralized in `doc_only_commit`: it excludes fan-out sub-tasks and walks the entire cascade-resolved include tree ([task.rs:1385](crates/cli/src/task.rs:1385), [task.rs:1406](crates/cli/src/task.rs:1406)). The walker follows nested includes and `with_composing_step_source` applies project structural deltas before the read ([start.rs:3265](crates/cli/src/start.rs:3265), [start.rs:3312](crates/cli/src/start.rs:3312), [start.rs:3331](crates/cli/src/start.rs:3331)). This closes CR1, E1, project shadows, and replace/insert/remove-step divergence.

- Finalize computes `doc_only` once and uses it for the diff signal, carryover exemption, forecast/model, and `StagePolicy::DocOnly` dispatch ([task.rs:3163](crates/cli/src/task.rs:3163), [task.rs:3231](crates/cli/src/task.rs:3231), [task.rs:3314](crates/cli/src/task.rs:3314), [task.rs:3542](crates/cli/src/task.rs:3542), [task.rs:3558](crates/cli/src/task.rs:3558)). `--carry-staged` therefore has no membership effect on this arm.

- CR3 is closed: promotions and owner-artifacts count only when their bytes differ from `HEAD`; unrelated staged code and config deltas cannot make a doc-only task non-empty ([task.rs:3191](crates/cli/src/task.rs:3191), [task.rs:3223](crates/cli/src/task.rs:3223), [task.rs:3231](crates/cli/src/task.rs:3231)).

- `stage_doc_only` builds only promoted destinations plus stageable recorded owner-artifacts, sorts/deduplicates them, and literalizes every `git add` pathspec ([task.rs:6082](crates/cli/src/task.rs:6082)). `git_commit_paths` independently rejects an empty list and literalizes every commit pathspec before entering the shared seam ([milestone.rs:1372](crates/cli/src/milestone.rs:1372), [milestone.rs:1383](crates/cli/src/milestone.rs:1383), [milestone.rs:1392](crates/cli/src/milestone.rs:1392)).

- Left-out classification is one entry per porcelain path; any nonblank index column becomes `left-staged` on the doc-only model ([task.rs:8693](crates/cli/src/task.rs:8693), [task.rs:8733](crates/cli/src/task.rs:8733)). `ManifestKind::ALL` contains seven members, including `LeftStaged` ([render.rs:2302](crates/cli/src/render.rs:2302), [render.rs:2318](crates/cli/src/render.rs:2318)).

- `CommitModel::{Index, Amend, DocOnly}` is exhaustive, and `CommitModel::of(amended, doc_only)` gives amend precedence ([render.rs:2418](crates/cli/src/render.rs:2418), [render.rs:2440](crates/cli/src/render.rs:2440)). `GATE_COVERAGE` has 13 rows and gives its index-gate member distinct amend/doc-only spellings ([gate_coverage.rs:193](crates/cli/src/gate_coverage.rs:193), [gate_coverage.rs:235](crates/cli/src/gate_coverage.rs:235), [gate_coverage.rs:302](crates/cli/src/gate_coverage.rs:302)).

- `COMMITTING_DOORS` remains 11 rows; doc-only correctly rides the existing `jigc task finalize` identity rather than minting a leaf ([invocation_log.rs:188](crates/cli/src/invocation_log.rs:188)).

- The trailer did not create a commit bypass: `CommitMessage` makes the seam own `-F`/`-m`; file messages are signed before `git commit` reads them, and all models still pass through `commit_through_seam` ([task.rs:8026](crates/cli/src/task.rs:8026), [task.rs:8147](crates/cli/src/task.rs:8147), [task.rs:8156](crates/cli/src/task.rs:8156), [task.rs:8166](crates/cli/src/task.rs:8166)).

## Schema boundary and bounds

No schema-hash-boundary violation found. The range adds exactly the `inconsistency` and `jigc-feedback` manifest rows, both at schema-version 1; existing hashes do not move. The rc.24 planning-record edit is presentation/slot-hint wording and leaves its manifest hash unchanged. The trailer changes neither schema nor pinned JSON keys.

Read-only source pass: I read the named historical records, M54/M55 verdicts and bounds, current designs, registries, symbol consumers, commit seams, pre-image/restore sites, and four `gitignore::ensure` production callers. I did not build, test, drive the binary, create directories, or write files. M55’s open F21 remains report-only and outside this committing-loss conclusion.