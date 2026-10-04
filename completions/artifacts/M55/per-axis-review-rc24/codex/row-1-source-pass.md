<!-- The unseeded Codex source pass for ROW 1 (setup · the hook · install · release), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

Source pass: `jigc-v1.0.0-rc.24` (`91834b5e`). Read-only; no binary driving and no writes.

## CLAIMS

1. No grounded completeness defect found.

I found no source-grounded missing door, undispositioned install path, unrecorded post-write error return, unsafe teardown classification, package/allowlist mismatch, or release-path bypass. Accordingly, there is no numbered defect lead with a reproducible wrong behavior to report.

## M54 finding dispositions

1. Hygiene denylist failure: fix still carried. Patterns are compiled and failures propagated by `dev/hygiene-scan:49-66`; all scanning paths check command status at `dev/hygiene-scan:104-151`. CI invokes it at `.github/workflows/ci.yml:143-153`.

2. Floating/unverified release executables: fix still carried. Every workflow `uses:` is a full SHA, including `.github/workflows/release.yml:47,53,68,90,115,135`; the exhaustive workflow fence and vetted table are at `crates/cli/tests/workflow_action_runtime_fence.rs:47-107,129-199`. Both release jobs install release-plz through the verified script at `.github/workflows/release.yml:58-63,131-132`; the recorded digest check is in `dev/install-release-plz:5-39`.

3. Silent probe failures / unnamed program: fix still carried, although the probe portion is outside this row’s seam. The embedded probe reports each failure at `crates/cli/src/doc_code_probe/mod.rs:613-653`, and the caller includes the attempted program at `crates/cli/src/task.rs:1657-1675`.

4. Failed release-PR publication recovery: documentation fix and its deliberately unpinned bound remain. `implementation/release.md:172-187` records that later pushes cannot retry the publish and gives the rerun/manual recovery paths. No source test simulates the remote deployment state.

5. Published engine diagnostic exposure: fix still carried, outside this row’s principal seam. `SaveDegrades` and `save_degrades` remain public but explicitly doc-hidden and unstable at `crates/engine/src/state.rs:1184-1225`.

## M54 “Not run” dispositions

- Genuine assistant Task-tool spawn: still a declared bound and outside this row; the source continues to provide only the process simulation.
- Linux replaced-binary-while-running `/proc/self/exe`: still not covered here. Normal relocated and symlinked self-exec are fenced at `crates/cli/tests/cargo_install_probe.rs:1-24,251-340`.
- CI runtime: now measured as recorded, but the cold runtime at this exact tree remains unmeasured; `completions/artifacts/M54/ci-runtime.md` records the runs, while `.github/workflows/ci.yml:19-28` fences the matrix and timeouts.
- Real release pipeline/App/yank: remains a remote-state bound. The local source fences configuration, not the human acts: `.github/workflows/release.yml:9-19,100-156` and `crates/cli/tests/release_pipeline_fence.rs:604-690`.
- Four-CPU `dev/runner-faithful`: still a declared non-run variant; the script’s platform/container behavior remains at `dev/runner-faithful:1-260`.

The older declared bounds have advanced as recorded: M54’s six fixes are now shipped; the credentialed workflow has run for the subsequent releases; the generated crate README closes the broken-link bound (`crates/cli/Cargo.toml:11-27`, `crates/cli/tests/package_contents.rs:189-229`). Finding 4 remains intentionally unpinned.

## Consistent row census

- Setup’s committed class is derived from one registry: seven unconditional paths plus conditional root `.gitignore`, guide, and trackable hook at `crates/cli/src/setup.rs:1749-1781`. Every possible member is dispositioned through the same enumeration at `:1784-1841`; the ten-member fence is `crates/cli/tests/setup_install_pathspec_guard.rs:1080-1131`.

- Dirty-path refusal occurs before writes at `crates/cli/src/setup.rs:1262-1361`; malformed settings are parsed before writes at `:1363-1383`.

- Every `write_install_span` error crosses `record_failed_install` at `crates/cli/src/setup.rs:1385-1408`. The footprint hashes only newly dirtied candidates and stages them at `:2194-2240`. `commit_install` records its settled owned set before its refusal, staging, posture, and commit exits at `:2524-2548`; later successful/no-op paths retain only soft members left uncommitted at `:2598-2647,2701-2703`. Post-failure user edits re-arm both worktree and index checks at `:2110-2155`.

- The install commit signs its own fixed message only under the adapter’s non-empty environment predicate at `crates/cli/src/setup.rs:2681-2690`; profile validation and activation are at `crates/cli/src/adapter.rs:80-147,244-258`.

- `BEHALF_DOORS` carries `setup` as committing with exactly the unborn-HEAD exemption, and `uninstall` separately as non-committing, at `crates/cli/src/cli.rs:2144-2162`; its named fence is at `:5213-5243`.

- The hook embeds the resolved running executable, quoted for spaces, at `crates/cli/src/setup.rs:516-556,1647-1669`; real hooks-directory resolution honors `core.hooksPath` and worktrees at `:921-940`. Rename blocking uses both staged operands at `:620-658`. Teardown distinguishes standalone, wrapped, and foreign bytes at `:823-918`.

- Package publication is allowlisted to `src`, `packs`, `adapters`, `guides`, licenses, and the generated README at `crates/cli/Cargo.toml:11-27`; equality in both directions is checked at `crates/cli/tests/package_contents.rs:29-42,93-162`.

- The install line is identical in `crates/cli/guides/QUICKSTART.md:21`, `README.md:19`, and generated `crates/cli/README.md:20`. Its caret semantics and carrier census are fenced at `crates/cli/tests/install_line.rs:34-132`.

- Release publication requires a main push, a missing published version, the `release` environment approval, and `release_always = false`: `.github/workflows/release.yml:25-27,83-112,142-156`; `release-plz.toml:1-25`. CI’s publish command is dry-run only at `.github/workflows/ci.yml:15-17,30-34`.

## Bounds

No tests or commands were run, per the source-pass brief. Remote crates.io state, GitHub environment protection, actual Trusted Publishing, action-tag provenance, and runtime timing were not independently reverified.

I found no schema-boundary violation: the trailer changes no schema or pinned JSON key; the methodology manifest retains the M55 `jigc-feedback` and `inconsistency` rows at schema-version 1, with no other hash movement attributable to this range. M55’s open report-only F21 remains outside this row’s setup/hook/install/release completeness question.