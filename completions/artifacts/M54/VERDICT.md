# M54 — publishable: completion verdict

**Verdict: COMPLETE — built, audited, five code-review findings triaged fix-now and fixed in six
commits, e2e 15 of 15 scenarios green, re-verified 4205 passed / 0 failed.** Written 2026-10-01, after
the last fix commit (`8f79ad32`) and the fold-back that recorded it (`10ef46c1`). The audit's raw output
is the source of this record: the milestone-build harness's completion stage returned both audits as
structured data (`audit.code_review`: five ranked findings with severity, location, confidence and
evidence, plus a summary; `audit.e2e`: `overall_pass`, a summary and fifteen scenarios). What follows
condenses that output and claims nothing beyond it; triage dispositions come from git.

| | |
|---|---|
| Planned | 2026-09-28 Settle, S1–S22 ([DECISIONS.md](../../../DECISIONS.md) → *M54 settled*) · [planning-gate-record](planning-gate-record.md), 24 rows |
| Built | twelve increments in S6's order; the build closed at `562352d7` (the fold-back reading *built, not audited*), gate **4196 / 0** |
| Published | inside the build, before the audit (gate-record **O7**): `jigc 1.0.0-rc.22` and `jigc-engine 0.1.0-rc.1` on crates.io, 2026-10-01 ([publish-proof](publish-proof.md)) |
| Audited | at `562352d7`, against `jigc 1.0.0-rc.22`. Code review: **2 MEDIUM · 3 LOW**, no HIGH. E2E: **15 of 15** scenarios passed, `overall_pass: true` |
| Triaged | all five **fix-now**; none deferred, none contested. A robust-advocate at triage found a residual of finding 2, fixed as its own commit on the human's approval |
| Fixed | `9ce1b2c4` · `17c05b8c` + residual `44e09fcc` · `ffcdcc85` · `3d182800` · `8f79ad32` |
| Re-verified | `dev/gate` after the last fix: **4205 passed / 0 failed** (build close 4196; the difference is the fixes' own tests) |

## What the milestone claimed, and whether it is true of what shipped

*jigc installs from crates.io with one static line and the `doc-code` probe inside the one binary; the
pipeline that published `jigc 1.0.0-rc.22` and `jigc-engine 0.1.0-rc.1` was rehearsed dry before it ran
for real; and a green push on the public runner reports in ≤ 15 min.*

The code reviewer's verdict, condensed from its summary: **the deliverable holds.** The packs and guides
moved inside the crate with no content change apart from one schema-manifest comment line. The probe
runs inside `jigc` by self-exec, with the intercept as `main`'s first statement, and the old `build.rs`,
sidecar workspace and extract step are gone. The failed-first-setup wedge is closed through one write-span
seam plus a settings-merge pre-check. Both crates were renamed, published and proved from the registry,
and CI is split into a fenced matrix. The reviewer rebuilt locally, drove the `__probe` intercept with
hostile argv and stdin (no panics; every refusal carries a reason), and ran the `g_config` group at
237 / 0. **Its headline concern sat in the new tooling, not the product** (finding 1), the second in the
release workflow's supply chain (finding 2); the rest were low-severity gaps in diagnostics, recovery
documentation and API surface.

The e2e auditor's verdict: **PASS**, all fifteen scenarios through the real binary at `562352d7`
(`jigc 1.0.0-rc.22`). The third leg of the claim — the ≤ 15 min green push — is a runner fact the e2e
did not re-measure; it is measured in [ci-runtime](ci-runtime.md) (below, *Not run*).

## The code review — five findings, five fixed

Ranked as the reviewer ranked them. Each entry gives the reviewer's severity, location and confidence,
its evidence condensed, the repro it drove where it drove one, and the triage disposition.

### 1 · MEDIUM — `dev/hygiene-scan` read a malformed denylist pattern as a clean scan

- **Location:** `dev/hygiene-scan:84, :108, :113, :115`; callers: CI's hygiene job `denylist` step and
  `dev/gate:262`. **Confidence:** high (driven).
- **Evidence:** `grep -E -f "$pats"` exits 2 when any pattern is invalid and matches nothing; the script
  ran under `set -u` only, and none of the four pattern-match sites checked the status (commit messages
  :84, added lines :108, `git grep` :113, `ls-files` :115 — enumerated by `grep -n -- '-f "$pats"'`,
  4 of 4 unguarded). The empty hits file read as 0, the *clean* branch. CI's denylist comes from the
  `JIGC_DENYLIST` secret, which no one reviews, so one typo there disabled the whole denylist while CI
  stayed green, and `dev/gate` printed `gate: hygiene  denylist clean`. A second gap, **reasoned from
  git's defaults, not driven:** the added-lines scan ran `git log -p` without `--diff-merges`/`-m`, so a
  line introduced only in a merge commit (a conflict resolution) was never scanned.
- **Repro (driven at HEAD):** a denylist holding only `Settle` → `dev/hygiene-scan <list> HEAD~3..HEAD`
  reports 37 hits, exit 1. Append one line `[unclosed` → `grep: brackets ([ ]) not balanced` twice, then
  `hygiene: denylist clean (2 pattern(s))`, **exit 0**. Expected: exit 2, a setup error.
- **Disposition: fix-now → `9ce1b2c4`** `fix(dev): hygiene-scan treats a malformed denylist as a setup
  error and scans merge resolutions`. Each pattern is compiled up front and a bad one exits 2, named by
  its line number and never by its text; all four sites check their status; the added-lines scan passes
  `--remerge-diff`. Pinned in `crates/cli/tests/dev_gate_report.rs` (a malformed list is *could not run*
  in `dev/gate` and exit 2 from the scan, history and `--tree`; a merge-only line is reported by commit,
  path and line).

### 2 · MEDIUM — the release workflow ran third-party actions at floating tags inside the credentialed jobs

- **Location:** `.github/workflows/release.yml:51-68, :116-122`;
  `crates/cli/tests/workflow_action_runtime_fence.rs:43-62`. **Confidence:** high on the configuration;
  the impact is a supply-chain exposure, not a driven exploit.
- **Evidence:** `release` (environment `release`, `id-token: write`, `contents: write`) ran
  `release-plz/action@v0.5` and `actions/checkout@v5`; `release-pr` passed the App's private key to
  `actions/create-github-app-token@v3` in no environment, so no approval gate applied. Every `uses:` was a
  mutable tag — 4 distinct actions, 7 `uses:` lines in `release.yml`. The composite `release-plz/action`
  pulls further floating refs. The runtime fence pinned *tag names* to vet the Node runtime, not commit
  SHAs, and release.md pinned only the release-plz *binary* version. A moved or compromised tag would run
  with the right to publish a permanent crates.io version; the human's approval gates *when* the job
  runs, not *what code* it runs. No decision accepting tag pins was on the record.
- **Disposition: fix-now → `17c05b8c`** `ci: pin every workflow action to a vetted commit SHA, not a
  floating tag` — every `uses:` in `release.yml` and `ci.yml` names a 40-hex commit with its tag as a
  trailing comment; `workflow_action_runtime_fence` vets `(action, sha, tag, runs.using)` per row.
- **Residual, found at triage by a robust-advocate → `44e09fcc`** `ci(release): install release-plz
  against a recorded SHA-256, not through release-plz/action`. The SHA pin left the composite fetching
  `cargo-binstall` at `releases/latest` and release-plz through it, neither checked, in both credentialed
  jobs. Both now install release-plz `0.3.169` with `dev/install-release-plz` against a recorded SHA-256,
  the composite is dropped, and the Known-gaps entry that had named binstall `1.23.0` is corrected —
  closed on the human's approval ([DECISIONS.md](../../../DECISIONS.md) → *release-plz is installed
  against a recorded SHA-256*). Pinned by `release_pipeline_fence` arms (m), (o), (p), which run the
  `run:` bodies verbatim against a stub release-plz.

### 3 · LOW — the probe's stderr channel was empty for its own five failure arms, and *could not start* named no program

- **Location:** `crates/cli/src/doc_code_probe/mod.rs:613-643`; `crates/cli/src/task.rs:1657-1675`
  (`CouldNotStart` built from `io::Error::to_string`). **Confidence:** high (one half driven, the other
  read).
- **Evidence:** S4's deliverable reads *a probe that cannot start, or runs at another version, says so in
  the finding*; both are wired. But `doc_code_probe::run()` had 5 `return ExitCode::FAILURE` arms (stdin
  read, request parse, snapshot read, snapshot parse, response serialize) and zero `eprint` calls, so the
  bounded stderr the invoker captures was always empty and the finding read *exited non-zero (exit-code 1)
  with no usable output*. Bounded to the 5 arms plus the one `CouldNotStart` site (`task.rs:1657`, the only
  production spawn). The e2e auditor observed the same thing independently and noted it is **not new in
  M54**: `ce3d86bf`'s standalone probe had the same 5 silent returns.
- **Repro (driven):** `echo garbage | jigc __probe doc-code --build 1.0.0-rc.22` → exit 1, nothing
  printed. On `dev/jigc-rig vendored` with `JIGC_DOC_CODE_PROBE` pointing at a mode-644 file,
  `jigc validate` reports ``probe `doc-code` could not start: Permission denied (os error 13)``, which
  names no file; the pre-flight it replaced named the path.
- **Disposition: fix-now → `ffcdcc85`** `fix(probe): a probe that fails says why, and could-not-start
  names the program` — each arm writes one `doc-code probe: <reason>` line on stderr (stdout empty, exit
  1, wire contract unchanged), and `CouldNotStart` carries the program. Completes S4. Pinned in
  `crates/cli/tests/doc_code_probe.rs`, with `probe_boundary_fences`' proxy re-aimed at the reason line.

### 4 · LOW — a release-PR publish that fails or is rejected is retried by no later push, and Known gaps did not say so

- **Location:** `release-plz.toml:7` (`release_always = false`); `.github/workflows/release.yml:84-122`;
  `implementation/release.md:85, :172-180`. **Confidence:** medium (reasoned from the configuration and
  release.md's own statement; not driven).
- **Evidence:** release.md:85 says approving a prompt on a commit that is not a release-PR merge
  publishes nothing. So if the deployment on the merge push is rejected, expires, or fails mid-upload,
  every later push has `check` report `missing=true` and raises a prompt whose approval publishes nothing;
  the version stays unpublished until someone re-runs the original merge-commit run inside GitHub's
  re-run window, or publishes by hand. Known gaps covered only a failed *tag* step after the upload.
- **Disposition: fix-now → `3d182800`** `docs(release): record that an unpublished release-PR merge is
  retried by no later push` — the gap and its recovery (reject the newer prompts, `gh run rerun <run-id>
  --failed` on the merge commit's run, approve again) in release.md → Known gaps; past the 30-day window
  the paths are named as undriven. Docs only: no repro was driven and none is pinned, because the
  finding is a record gap whose trigger is a remote deployment state.

### 5 · LOW — the test-only `engine::state::save_degrades` / `SaveDegrades` became public API of the published `jigc-engine`

- **Location:** `crates/engine/src/state.rs:1184-1225`. **Confidence:** high.
- **Evidence:** a thread-local tally whose own doc says *nothing in the product reads it*; its only
  consumer outside `state.rs`/`file_state.rs` is `crates/cli/tests/file_state_concurrency.rs`, which
  instruments the open save-lock flake. Shipped in `jigc-engine 0.1.0-rc.1`, a permanent version, where
  a later removal is an API break on a crate whose compatibility promise is still owed. 1 type + 1 fn.
- **Disposition: fix-now → `8f79ad32`** `fix(engine): hide the save-degrade test diagnostic from
  jigc-engine's API docs` — both items `#[doc(hidden)]` with a doc line declaring no stability promise;
  visibility unchanged, no cargo feature. Pinned by
  `state::tests::the_save_degrade_diagnostic_is_doc_hidden`. The items stay `pub` in `0.1.0-rc.1`
  itself, which a published version cannot change.

## The e2e half — 15 scenarios, 15 green

| # | Scenario | Result | What was driven |
|---|---|---|---|
| 1 | Inc 2 · the self-exec probe runs in production with no override and no sibling; `task finalize` blocks on a drifted anchor | pass | an isolated binary copy with no `doc-code` beside it, on a vendored rig: `validate` exit 0 with `doc-code.symbol-exists` after the drift; `task finalize` exit 3 naming it *in the staged index*, HEAD unchanged; setup wrote nothing beside the binary |
| 2 | Inc 2 · the `__probe` intercept refuses skew and malformed argv, and stays hidden from clap | pass | `--build 0.0.0-skew` exit 1 naming both builds; four malformed argv shapes exit 1 naming the expected form; `--help` carries no `__probe`; `--format json __probe` is clap's exit 2 (first-word only, as designed). Raised the LOW that became finding 3 |
| 3 | Inc 2 · *could not start* and a skewed build are reported at every door | pass | three stubs (mode 644 · skewed · absent) at `task finalize` (exit 3, one `pack-probe-integrity.probe-failure`, no `symbol-exists`, HEAD unmoved), `validate`, orientation, `milestone finalize` and the pre-commit hook |
| 4 | Inc 2 · a probe spawn adds zero invocation-log records | pass | one record per `validate`, none for the probe child, at the CLI and the hook door |
| 5 | Inc 2 · symlinked, PATH-resolved and relocated installs reach the probe; a stale sibling `doc-code` is ignored | pass | all three reach `symbol-exists`; a fake sibling returning no findings is not consulted; an empty override is unset; a 2 MB stderr flood is bounded (4174 chars, 0.63 s) |
| 6 | Inc 2 · the retired layout is gone | pass | no `probes/`, no `build.rs`, no `extract-probe` in source or binary; all 10 tree-sitter deps pinned with `=` |
| 7 | Inc 3 · the guides moved into the crate and are installed by setup; zero schema-hash movement | pass | both guides carried in the installed skill; one comment line changed per pack across the move, no schema-hash line; both packs embedded at `1.0.0-rc.22` |
| 8 | Inc 4 · a failed first setup (read-only `core.hooksPath`) does not wedge the repository; a user edit after it still re-arms the dirty guard | pass | plain re-run after the fix exit 0 with the hook committed; the control with a user edit refuses `setup.dirty-install-path`; a gpg-failing commit step recovers the same way |
| 9 | Inc 4 · a malformed committed `.claude/settings.json` is refused before anything is written | pass | trailing comma, non-object root, non-array `allow`, `permissions: 5`, `hooks: 3`, a non-array event — each refused with an empty status, each re-run exit 0 after the fix, the user's entry kept |
| 10 | Inc 5 · the packages are `jigc` and `jigc-engine` with separate versions; the store stamp is `jigc`'s | pass | `cargo metadata`, `--version`, `.jigc/version`; an rc.21 store read by rc.22 reports `store-version.binary-mismatch` and restamps on `setup` |
| 11 | Inc 6 · the package allowlist and the publish dry-run for both crates | pass | `cargo package --list` carries src, both packs, the adapter, both guides, licenses and README, no `tests/`; `cargo publish --workspace --dry-run` exit 0 for both |
| 12 | Inc 7 · `dev/runner-faithful tarball` | pass | every step ok on `linux/aarch64`, exit 0 |
| 13 | Inc 11 · a crates.io install works on macOS and in the runner-faithful container; unpinned resolves nothing | pass | the install line installs `1.0.0-rc.22` (only `jigc` in `bin/`); the drift finalize exit 3, the restored one exit 0; unpinned exit 101; `dev/runner-faithful registry` exit 0; `dev/unpublished-versions` `missing=false`; both `0.0.0` yanked, neither rc |
| 14 | Fan-out N-process sim · the rendered `Spawn:` lines run verbatim; the by-task-id join is byte-identical across completion orders | pass | three sub-tasks with a forced slug collision, completion order forward and reversed, dates pinned: tree `d0b00b2e` and HEAD `c1e9a0e` both times, suffixes assigned identically |
| 15 | Not driven: the Linux replaced-binary `/proc/self/exe` case, CI runtime, the release pipeline | pass (recorded as gaps, not failures) | only crates.io state was read and `dev/unpublished-versions` run |

Two observations the auditor recorded alongside the passes, neither a failure: scenario 7 notes the
crates.io README's link breakage as **already recorded** (`f36f6bcc`) and not re-raised; scenario 14 notes
that reversing the *add-task* order changes only the milestone record, whose task items follow insertion
order — pre-existing design, not M54.

## Not run — what the e2e stated it did not drive

- **The genuine concurrent assistant Task-tool spawn.** Scenario 14 is an N-process binary sim with the
  rendered `Spawn:` lines executed verbatim; the real spawn is the orchestrator's main-session half (the
  M51 bound, unchanged).
- **The Linux replaced-binary-while-running case** (`probe_self_image.rs`'s `/proc/self/exe` arm). The
  container runs show self-exec works on Linux in the normal case only.
- **The ≤ 15 min green-push claim and the before/after CI runtime** — GitHub-runner facts, not binary
  behaviour. **Since measured, outside the audit,** in [ci-runtime](ci-runtime.md): the final after run
  `36843531774`, every job on four CPUs, slowest job `test (g_flow)` at **430 s** against the 900 s bound.
  That run was warm; the cold bound is the provisional run's 448 s at Increment 8's tip, the cold figure
  at this tree is unmeasured, and the red-push half is held by the fenced workflow shape, not by a run.
- **The release pipeline, release-plz, the rehearsal, the App and the yank** — the human's acts and remote
  state ([release.md](../../../implementation/release.md) → *What agents may not do*). The auditor only
  read crates.io (yanks confirmed) and ran `dev/unpublished-versions`.
- **The 4-CPU-limited `dev/runner-faithful` variant** (scenario 12 ran unlimited).

## Declared bounds, carried in writing

- **O7: this audit ran over a version already published.** Its six fixes are on `main` and in no
  published version; they ship in the next release candidate, the one carrying M55
  ([planning-gate-record](planning-gate-record.md) → O7). `jigc-engine 0.1.0-rc.1` keeps the save-degrade
  items undecorated, and `1.0.0-rc.22`'s probe keeps its silent failure arms.
- **The reshaped `release` job has not run for real.** `44e09fcc` changed both credentialed jobs; its first
  real run is M55's publish, human-approved. The remaining supply-chain bound is trust on first use of a
  mutable release asset, not a publisher signature (release.md → Known gaps).
- **`1.0.0-rc.22` is published, not installed on this host**, whose binary is still the rc M53 shipped.
- **The crates.io README breaks four of its six link targets.** The fix is the human's fork, owed before
  the next release PR is merged ([decisions-pending.md](../../../implementation/decisions-pending.md) →
  *Before the next release PR is merged*).
- **The partial re-review over S18's axes is owed** on the release candidate that carries M54 and M55,
  beside the blind trial and before the call ([DECISIONS.md](../../../DECISIONS.md) → *the road to the
  1.0.0 call*).
- **Finding 4 is unpinned**: a record gap whose trigger is a remote deployment state; no repro was
  driven, so none is pinned.

## What is next

The switch to a branch-per-milestone model → **M55**, the findings channel → the release candidate
carrying both, with M54's audit fixes in it → the blind agent trial and the partial re-review on that
candidate → **the 1.0.0 call, which is the human's.**
