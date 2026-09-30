# The release rehearsal — M54 Increment 10

This file records the **dry-run rehearsal** of the release pipeline, read back by run id. It is the entry gate of Increment 10 ([planning-gate-record.md](planning-gate-record.md) → row 19; S5). The rehearsal ran at H2 in the private `gherrink/jigc-release-rehearsal`, a mirror push of `main` with the same GitHub App installed. The last section records `gherrink/jigc` as it stood when the rehearsal closed. Cross-ref [release.md](../../../implementation/release.md) → Publishing (*Rehearsed dry*, *The workflow*) and Known gaps; [DECISIONS.md](../../../DECISIONS.md) → *M54 Increment 10 planning* (the basis each read was checked against) and *M54 Inc 10 T1*.

**Verdict: the rehearsal ran the whole wiring, and every read equals the planning basis.** One release PR opened under the conditional title and carried only the two generated changelogs. The check job's flag read `missing=true` on both pushes. After the merge, the environment-bound job ran release-plz with `dry_run: true`, behind the overlay step, and reached `jigc-engine` before `jigc`, naming both tags it would create. The rehearsal repository holds no tag and no release. Nothing is missing or failed, so no fix lands here and no re-run is owed.

## The commands

Every value below is one line of this script's output, cited by its id. The script was run whole on 2026-10-01 with `gh` authenticated as `gherrink`, as `bash reads.sh <repo-root>` from HEAD `b08048c4`. A second whole run, executing this fenced block as extracted from this file, gave byte-identical output. Logs are saved to files first and read from there, never through a pipe whose exit status is read. `strip` removes the logs' ANSI colour codes, and `cut -c30-` drops the runner's timestamp prefix. release-plz's own timestamp stays on the line.

```sh
set -u
ROOT=$1
R=gherrink/jigc-release-rehearsal
G=gherrink/jigc
D=$(mktemp -d "${TMPDIR:-/tmp}/rehearsal.XXXXXX")
strip() { perl -pe 's/\e\[[0-9;]*m//g'; }

echo "## R1"
for r in 36756306377 36761534078; do
  gh run view $r -R $R --json databaseId,workflowName,headBranch,headSha,event,status,conclusion,attempt --jq '"\(.databaseId) \(.workflowName) \(.headBranch) \(.headSha) \(.event) \(.status) \(.conclusion) attempt=\(.attempt)"'
done
echo "## R2"
for r in 36756306377 36761534078; do
  gh run view $r -R $R --json jobs --jq '.jobs[] | "\(.databaseId) \(.name) \(.conclusion) \(.startedAt) \(.completedAt) \((.completedAt|fromdate) - (.startedAt|fromdate))s"'
done
echo "## R3"
for j in 110027305290 110027305793 110027540425 110045040551 110045040942 110045274235; do
  gh api --allow-escape-sequences repos/$R/actions/jobs/$j/logs > "$D/$j.log"; echo "$j rc=$? lines=$(wc -l < "$D/$j.log" | tr -d ' ')"
done
echo "## R4"
command grep -hE 'Z (jigc(-engine)?@[^ ]+ (missing|present)|missing=(true|false))$' "$D/110027305290.log" "$D/110045040551.log"
echo "## R5"
for j in 110027305793 110045040942; do
  command grep -hE 'Only changelog will be updated|opened pr:|already up-to-date|release_pr_output: \{|Truncating' "$D/$j.log" | strip | cut -c30-
done
echo "## R6"
for j in 110027305793 110045040942; do
  echo "$j metadata-warnings=$(command grep -c 'cannot read package metadata' "$D/$j.log") jigc=$(command grep -c 'cannot read package metadata of jigc in' "$D/$j.log") jigc-engine=$(command grep -c 'cannot read package metadata of jigc-engine in' "$D/$j.log")"
done
echo "## R7"
gh pr view 1 -R $R --json number,title,author,state,mergedBy,mergeCommit,headRefName,additions,deletions,changedFiles,files --jq '"#\(.number) \(.title) | author=\(.author.login) state=\(.state) mergedBy=\(.mergedBy.login) merge=\(.mergeCommit.oid) head=\(.headRefName) +\(.additions) -\(.deletions) files=\(.changedFiles)", (.files[] | "  \(.path) +\(.additions) -\(.deletions)")'
gh pr list -R $R --state all --json number,title --jq '"count=\(length)", (.[] | "#\(.number) \(.title)")'
echo "## R8"
gh api repos/$R/compare/39980394b9d7fda374faa32c7a1ec741af9d82c0...5f5c9b918c171832c3305f1dfcbae15fb2df778d --jq '"ahead=\(.ahead_by) files=\([.files[].filename]|join(","))", (.commits[] | "\(.sha[0:8]) \(.commit.message|split("\n")[0])")'
echo "## R9"
for r in 36756306377 36761534078; do
  gh run view $r -R $R --json jobs --jq '.jobs[] | select(.name=="release") | "\(.databaseId): " + ([.steps[] | "\(.number) \(.name) \(.conclusion)"]|join(" · "))'
done
echo "## R10"
for j in 110027540425 110045274235; do
  echo "-- $j"
  command grep -hE '##\[group\]Run cargo_home|^.{29}  dry_run: |skipping release|due to dry|release_output: \{' "$D/$j.log" | strip | cut -c30-
done
echo "## R11"
echo "tags=$(gh api repos/$R/tags --jq length) tag-refs=$(gh api repos/$R/git/matching-refs/tags --jq length) releases=$(gh api repos/$R/releases --jq length)"
echo "## R12"
gh api repos/$R --jq '"\(.full_name) visibility=\(.visibility)"'
gh api repos/$R/environments/release --jq '"\(.name) can_admins_bypass=\(.can_admins_bypass) protection_rules=\([.protection_rules[].type]|join(",")) branch_policy=\(.deployment_branch_policy|tostring)"'
for r in 36756306377 36761534078; do echo "approvals $r: $(gh api repos/$R/actions/runs/$r/approvals)"; done
echo "## R13"
gh run list -R $R --workflow CI --limit 20 --json databaseId,event,headBranch,headSha,conclusion --jq '.[] | "\(.databaseId) \(.event) \(.headBranch) \(.headSha[0:8]) \(.conclusion)"'
echo "## R14"
for r in 36757113219 36757105242 36757106834 36756306292 36761533977; do
  gh run view $r -R $R --json jobs --jq '"\(.jobs|length) jobs: " + ([.jobs[] | select(.conclusion != "success") | "\(.name) \(.conclusion) \((.completedAt|fromdate) - (.startedAt|fromdate))s"]|join(" · "))' | sed "s/^/$r /"
done
echo "## R15"
for j in 110030050233 110030023400; do
  gh api --allow-escape-sequences repos/$R/actions/jobs/$j/logs > "$D/ci-$j.log"; echo "-- $j rc=$?"
  echo "nproc=$(awk '/##\[group\]Run nproc/{f=1} f&&/##\[endgroup\]/{getline; print $2; exit}' "$D/ci-$j.log")"
  command grep -hE '\.\.\. FAILED$|concurrently recorded keys|^.{29}test result: ' "$D/ci-$j.log" | cut -c30-
done
echo "## G1"
for r in 36753643562 36785068612; do
  gh run view $r -R $G --json databaseId,headSha,event,status --jq '"\(.databaseId) \(.headSha) \(.event) \(.status)"'
  gh run view $r -R $G --json jobs --jq '.jobs[] | "  \(.databaseId) \(.name) \(.status) \(.conclusion) \(.startedAt) \(if .status == "completed" then ((.completedAt|fromdate) - (.startedAt|fromdate)|tostring) + "s" else "-" end)"'
done
echo "## G2"
gh run view 36753643562 -R $G --json jobs --jq '.jobs[] | select(.name=="release-pr") | [.steps[] | "\(.number) \(.name) \(.conclusion)"]|join(" · ")'
gh api --allow-escape-sequences repos/$G/actions/jobs/110018265280/logs > "$D/g-110018265280.log"
command grep -hE '##\[error\]' "$D/g-110018265280.log" | cut -c30-
echo "## G3"
gh pr view 1 -R $G --json number,title,author,state,headRefName,headRefOid,baseRefName,baseRefOid,additions,deletions,changedFiles,files --jq '"#\(.number) \(.title) | author=\(.author.login) state=\(.state) head=\(.headRefName) \(.headRefOid) base=\(.baseRefName) \(.baseRefOid) +\(.additions) -\(.deletions) files=\(.changedFiles)", (.files[] | "  \(.path) +\(.additions) -\(.deletions)")'
H=$(gh pr view 1 -R $G --json headRefOid --jq .headRefOid)
gh api "repos/$G/contents/crates/cli/Cargo.toml?ref=$H" -H 'Accept: application/vnd.github.raw' > "$D/pr-cli.toml"
command grep -m1 '^version' "$D/pr-cli.toml"
for c in cli engine; do
  gh api "repos/$G/contents/crates/$c/CHANGELOG.md?ref=$H" -H 'Accept: application/vnd.github.raw' > "$D/pr-$c.md"
  command grep -m2 '^## ' "$D/pr-$c.md"
done
echo "## G4"
gh api repos/$G/environments/release --jq '"\(.name) can_admins_bypass=\(.can_admins_bypass) protection_rules=\([.protection_rules[] | .type + (if .reviewers then ":" + ([.reviewers[].reviewer.login]|join(",")) else "" end)]|join(","))"'
gh api repos/$G/actions/secrets --jq '[.secrets[].name]|join(",")'
echo "## L1"
git -C "$ROOT" diff --stat 39980394 HEAD -- .github/workflows/release.yml release-plz.toml dev/unpublished-versions Cargo.toml crates/cli/Cargo.toml crates/engine/Cargo.toml Cargo.lock; echo "diff-rc=$? lines=$(git -C "$ROOT" diff 39980394 HEAD -- .github/workflows/release.yml release-plz.toml dev/unpublished-versions Cargo.toml crates/cli/Cargo.toml crates/engine/Cargo.toml Cargo.lock | wc -l | tr -d ' ')"
echo "## G5"
for c in cli engine; do
  gh api "repos/$R/contents/crates/$c/CHANGELOG.md?ref=ca34c8b0" -H 'Accept: application/vnd.github.raw' > "$D/reh-$c.md"
  diff "$D/reh-$c.md" "$D/pr-$c.md"; echo "$c diff-rc=$?"
done
```

`pending_deployments` is not read: the agents' deny list refuses it even as a read ([release.md](../../../implementation/release.md) → *What agents may not do*). Job states come from `gh run view --json jobs` instead. GitHub keeps run logs for 90 days by default, so R3, R15 and G2 work until about 2026-12-29. The lines they print are quoted below so this record outlives the logs.

## The runs, and the wiring they ran

| Run | Workflow | Ref · head | Event | Status | Read |
|---|---|---|---|---|---|
| `36756306377` | Release | `main` · `39980394b9d7fda374faa32c7a1ec741af9d82c0` | push | completed · **success** · attempt 1 | R1 |
| `36761534078` | Release | `main` · `5f5c9b918c171832c3305f1dfcbae15fb2df778d` | push | completed · **success** · attempt 1 | R1 |

`39980394` is this repository's `main` as mirrored. `5f5c9b91` is the merge of the rehearsal's PR #1. **The rehearsal ran today's wiring.** L1 prints no diff between `39980394` and HEAD (`diff-rc=0 lines=0`) over `.github/workflows/release.yml`, `release-plz.toml`, `dev/unpublished-versions`, the three manifests and `Cargo.lock`.

### The six jobs against their `timeout-minutes`

R2, with the timeouts from `release.yml`. R3 fetched all six logs whole (`rc=0`: 139 · 2,462 · 437 · 140 · 2,462 · 439 lines).

| Job | Id | Start → end (UTC, 2026-09-30) | Wall clock | `timeout-minutes` | Used |
|---|---|---|---|---|---|
| `check` (pre-merge) | `110027305290` | 18:07:25 → 18:07:57 | 32 s | 10 | 5 % |
| `release-pr` (pre-merge) | `110027305793` | 18:07:26 → 18:14:11 | **405 s** (6 min 45 s) | 15 | 45 % |
| `release` (pre-merge) | `110027540425` | 18:08:06 → 18:08:40 | 34 s | 30 | 2 % |
| `check` (merge) | `110045040551` | 18:51:18 → 18:51:48 | 30 s | 10 | 5 % |
| `release-pr` (merge) | `110045040942` | 18:51:17 → 18:59:25 | **488 s** (8 min 8 s) | 15 | 54 % |
| `release` (merge) | `110045274235` | 18:51:55 → 18:53:43 | **108 s** | 30 | 6 % |

All six concluded `success`. The timestamps are whole seconds, so each wall clock is ±1 s. **The two `release-pr` wall clocks are the history walk:** release-plz reads every prior commit of both packages over the `0.0.0` placeholders (the release PR, below), which the bootstrap decision estimated at about 11 minutes. The merge run's walk opened no PR and still took 488 s. **The merge `release` is a dry run.** Its 108 s packages and verifies both crates, and it uploads nothing and waits on no index. The real publish's upload and index wait (`publish_timeout` 10 min per crate) are not measured here.

## The release PR

R7 and R5, over run `36756306377`'s `release-pr` job.

- **Exactly one PR.** `gh pr list --state all` prints `count=1`, `#1 chore(release): prepare release`.
- **The title is the conditional form's two-package branch.** `pr_name` adds ` <package> v<version>` only when one package releases. Two did, so the title is the bare `chore(release): prepare release`.
- **Author:** `app/gherrink-jigc-release`, the GitHub App. **State:** `MERGED` by `gherrink`, as merge commit `5f5c9b918c171832c3305f1dfcbae15fb2df778d`, from branch `release-plz-2026-09-30T18-14-02Z`.
- **It adds only the two changelogs:** `+2064 -0` over `files=2`, which are `crates/cli/CHANGELOG.md +1407 -0` and `crates/engine/CHANGELOG.md +657 -0`.
- **No version bump.** R8 compares `39980394…5f5c9b91`: `ahead=2`, the changed files are exactly `crates/cli/CHANGELOG.md,crates/engine/CHANGELOG.md`, and the commits are `ca34c8b0 chore(release): prepare release` and `5f5c9b91 Merge pull request #1 from gherrink/release-plz-2026-09-30T18-14-02Z`. No manifest and no `Cargo.lock` changed.
- **Why only the changelogs** (R5): release-plz logged, for each package,

  ```
  2026-09-30T18:08:05.137937Z  INFO jigc-engine: local version (0.1.0-rc.1) > registry version (0.0.0). Only changelog will be updated.
  2026-09-30T18:10:00.099414Z  INFO jigc: local version (1.0.0-rc.21) > registry version (0.0.0). Only changelog will be updated.
  2026-09-30T18:14:02.219273Z  WARN PR body is still longer than 65536 characters. Truncating as is.
  2026-09-30T18:14:06.331169Z  INFO opened pr: https://github.com/gherrink/jigc-release-rehearsal/pull/1
  release_pr_output: {"prs":[{"base_branch":"main","head_branch":"release-plz-2026-09-30T18-14-02Z","html_url":"https://github.com/gherrink/jigc-release-rehearsal/pull/1","number":1,"releases":[{"package_name":"jigc-engine","version":"0.1.0-rc.1"},{"package_name":"jigc","version":"1.0.0-rc.21"}]}]}
  ```

  The hand-set local versions sit above the `0.0.0` placeholders, so release-plz keeps them and writes the full history as each package's first changelog section. **This is the path O4 requires the bootstrap to take.**
- **Two benign warnings.** The PR body is truncated at GitHub's 65,536-character limit, as the line above shows. And R6 counts **2,008** `cannot read package metadata` WARN lines in each `release-pr` log: **1,373** for `jigc` in `crates/cli` and **635** for `jigc-engine` in `crates/engine`. Each is a history commit from before the package took its current name. *The planning basis called these "~2,000 `cannot read package metadata of jigc-engine`" lines. The total is right, but 635 of them name `jigc-engine` and the rest name `jigc`.*

## The check job's flag

R4, the `check` step's output in each run:

```
2026-09-30T18:07:54.5320497Z jigc@1.0.0-rc.21 missing
2026-09-30T18:07:54.5327366Z jigc-engine@0.1.0-rc.1 missing
2026-09-30T18:07:54.5328231Z missing=true
2026-09-30T18:51:46.4717223Z jigc@1.0.0-rc.21 missing
2026-09-30T18:51:46.4722691Z jigc-engine@0.1.0-rc.1 missing
2026-09-30T18:51:46.4731639Z missing=true
```

**`missing=true` in both runs**, so the environment-bound `release` job ran on both pushes.

## The dry run

R9 prints the `release` job's steps. They are the same in both runs: `1 Set up job` · `2 Checkout` · **`3 engine overlay (rehearsal)`** · `4 release-plz release` · `8 Post Checkout` · `9 Complete job`, all `success`. R10 quotes each job's log.

**Before the merge** (`110027540425`, push of `39980394`):

```
##[group]Run cargo_home="${CARGO_HOME:-$HOME/.cargo}"
  dry_run: true
2026-09-30T18:08:38.800047Z  INFO skipping release: current commit is not from a release PR
release_output: {"releases":[]}
```

`release_always = false` did what release.md says: a push that is not a release-PR merge publishes nothing, even with the flag set.

**After the merge** (`110045274235`, push of `5f5c9b91`):

```
##[group]Run cargo_home="${CARGO_HOME:-$HOME/.cargo}"
  dry_run: true
2026-09-30T18:52:54.809545Z  INFO jigc-engine 0.1.0-rc.1: due to dry, skipping the following: ["cargo registry upload", "creation of tag 'jigc-engine-v0.1.0-rc.1'", "creation of git release"]
2026-09-30T18:53:42.213499Z  INFO jigc 1.0.0-rc.21: due to dry, skipping the following: ["cargo registry upload", "creation of tag 'jigc-v1.0.0-rc.21'", "creation of git release"]
release_output: {"releases":[]}
```

- **`dry_run: true`** reached the action. The input expression is `'true'` off `gherrink/jigc`.
- **The overlay step ran** (`Run cargo_home=…`, the `[patch.crates-io]` write) before release-plz. Without it, `jigc`'s verify build cannot resolve the `=` pin on an unpublished engine.
- **Engine first, then `jigc`:** `jigc-engine 0.1.0-rc.1` at 18:52:54, and `jigc 1.0.0-rc.21` 48 s later at 18:53:42.
- **Both would-be tags are named:** `jigc-engine-v0.1.0-rc.1` and `jigc-v1.0.0-rc.21`, each with its registry upload and its GitHub release, all skipped *due to dry*.

**After the merge, `release-pr` found nothing to propose** (R5, `110045040942`): both *"Only changelog will be updated"* lines again, then `the repository is already up-to-date` and `release_pr_output: {"prs":[]}`. No second PR was opened.

**Nothing was created** (R11): `tags=0 tag-refs=0 releases=0`. **Zero tags and zero releases** are in the rehearsal repository.

## What the rehearsal did not exercise

- **The approval prompt.** R12: `gherrink/jigc-release-rehearsal visibility=private`, and its `release` environment reads `can_admins_bypass=true protection_rules=branch_policy branch_policy={"custom_branch_policies":true,"protected_branches":false}`. **There is no required reviewer.** `approvals` is `[]` for both runs. Neither `release` job waited at a prompt, and no one approved one. The prompt is live on `gherrink/jigc` (G4, below), and the approval of a job that publishes is first exercised by the real rc.22 publish. release.md → Known gaps records this.
- **The OIDC exchange** cannot run here. The trusted-publisher configuration names `gherrink/jigc`, and `--dry-run` needs no token (O5; release.md → Known gaps).
- **Concurrency at the prompt.** No job waited, so the rehearsal could not show whether a waiting job holds its group. The real repository showed it (below).
- **A registry-resolved engine.** The overlay makes `jigc`'s dry-run verify build use the local engine (the declared bound in release.md → *The workflow*).

### The rehearsal's CI, on 2-CPU private runners

The mirror also ran `ci.yml`. The rehearsal repository is private, so its runners are the 2-CPU private ones, not the public 4-CPU runner S8's `timeout-minutes` were sized for. **CI is not the release wiring the entry gate names, so its reds are recorded here and are not a halt.** R13 lists five CI runs, and R14 lists the non-`success` jobs of each (20 jobs per run):

| Run | Event · ref · head | Conclusion | Non-green jobs (wall clock) |
|---|---|---|---|
| `36756306292` | push · `main` · `39980394` | cancelled | `test (g_milestone)` cancelled 611 s · `test (g_doc)` cancelled 619 s · `test (g_flow)` cancelled 619 s |
| `36757105242` | push · `release-plz-2026-09-30T18-14-02Z` · `39980394` | failure | `test (g_milestone)` cancelled 605 s · `test (g_flow)` cancelled 620 s · **`test (g_finalize)` failure 339 s** |
| `36757106834` | push · `release-plz-2026-09-30T18-14-02Z` · `ca34c8b0` | cancelled | `test (g_doc)` cancelled 623 s · `test (g_milestone)` cancelled 622 s · `test (g_flow)` cancelled 620 s |
| `36757113219` | **pull_request (PR #1)** · `ca34c8b0` | failure | `test (g_flow)` cancelled 619 s · **`test (g_finalize)` failure 266 s** · `test (g_milestone)` cancelled 617 s · `test (g_doc)` cancelled 616 s |
| `36761533977` | push · `main` · `5f5c9b91` | cancelled | `test (g_flow)` cancelled 618 s · `test (g_doc)` cancelled 601 s |

- **The cancellations are the 10-minute `timeout-minutes`.** Each is 601–623 s, and every one is in `g_flow`, `g_milestone` or `g_doc`, the three slowest groups. On PR #1, all three timed out. `g_milestone` finished once (on `36761533977`) and `g_doc` once (on `36757105242`), each inside its 10 minutes.
- **The two `g_finalize` failures are one flake.** R15 reads `nproc=2` in both jobs' `CPU count` step, and each failed exactly one test, `file_state_concurrency::every_concurrent_record_survives_the_shared_save`:
  - `110030050233` (PR #1): `5 of 600 concurrently recorded keys were accepted and then discarded by a later save (first lost: Some("docs/survivor-w0-0090.md"))`, then `test result: FAILED. 405 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 221.79s`.
  - `110030023400`: `3 of 600 …` (first lost `docs/survivor-w3-0093.md`), then `405 passed; 1 failed; 1 ignored`, finished in 273.74 s.

  This is the 2-CPU save-lock survival flake that the human's 2026-09-30 call covers ([DECISIONS.md](../../../DECISIONS.md) → *the save-lock survival flake is instrumented, not fixed*). These runs are on `39980394` and `ca34c8b0`, which predate that instrumentation (`048bde93`), so their panic does not yet name how many saves ran unlocked.
- **Every other job was green in every run.** That includes `hygiene`, `publish-dry-run`, and `test (g_config)`, which holds `doc_link_fence`. On PR #1 they ran over the release PR's own tree (`ca34c8b0`), which carries the two generated changelogs. The link fence does not read the changelogs themselves: its root home is not walked recursively, and neither file is a declared live doc.

## The real repository at the rehearsal's close

`gherrink/jigc` also ran `release.yml` on its pushes of `39980394` and `2e7493c5`. G1–G5 were read on 2026-10-01, and this is their state then.

| Run | Head | Run status | `check` | `release-pr` | `release` |
|---|---|---|---|---|---|
| `36753643562` | `39980394b9d7fda374faa32c7a1ec741af9d82c0` | **waiting** | `110018264584` success, 21 s | `110018265280` **failure**, 6 s | `110018437350` **waiting** since 17:45:37Z |
| `36785068612` | `2e7493c513ecbfce2059181b352fc182d985fb1c` | **pending** | `110124401864` success, 23 s | `110124402496` success, 348 s | `110124548582` **pending** since 22:21:41Z |

- **Run `36753643562`'s `release-pr` failed at its first step** (G2): `1 Set up job success · 2 App token failure · 3 Checkout skipped · 4 release-plz release-pr skipped`, with `##[error][@octokit/auth-app] appId option is required`. The push came before H2 created the App's secrets. This is release.md's declared bound (*the App and its two secrets must exist before `release.yml` reaches a repository's `main`*).
- **A job waiting at the prompt holds its concurrency group.** Both `release` jobs share group `release-refs/heads/main`. The older one has waited at the approval prompt since 17:45:37Z. The newer one is `pending` behind it and did not replace it. Meanwhile `36785068612`'s `release-pr`, in its own group, ran to `success`. So a release waiting at the prompt does not hold up the release PR.
- **The real PR #1** (G3): `#1 chore(release): prepare release`, by `app/gherrink-jigc-release`, `OPEN`. Its head is `release-plz-2026-09-30T22-26-58Z` at `b75cde5b8124e2acc688f4d6a32b7d7088480715`, and its base is `main` at `2e7493c513ecbfce2059181b352fc182d985fb1c`. It is `+2066 -0` over `files=2`: `crates/cli/CHANGELOG.md +1408 -0` and `crates/engine/CHANGELOG.md +658 -0`. At its head, `crates/cli/Cargo.toml` reads `version = "1.0.0-rc.21"`. The changelogs head `## [Unreleased]`, then `## [1.0.0-rc.21](https://github.com/gherrink/jigc/compare/jigc-v0.0.0...jigc-v1.0.0-rc.21) - 2026-09-30` and `## [0.1.0-rc.1](https://github.com/gherrink/jigc/compare/jigc-engine-v0.0.0...jigc-engine-v0.1.0-rc.1) - 2026-09-30`. **This PR carries rc.21, in the same shape as the rehearsal's PR.** G5 diffs each changelog against the rehearsal PR's (`ca34c8b0`): the only differences are the compare links' repository in the version heading and one added line, `- *(engine)* the save-lock survival cells name their own save degrades`. That is `048bde93`, which touches both crates and landed after the rehearsal's `39980394`. Merging it as it stands would publish rc.21, not rc.22. The bootstrap bump (Increment 10 T2) and the regenerated PR come first, and T3 writes the human's checks for H3.
- **The environment and the secrets** (G4): `release can_admins_bypass=false protection_rules=required_reviewers:gherrink,branch_policy`, and the repository secrets are `JIGC_DENYLIST,JIGC_RELEASE_APP_ID,JIGC_RELEASE_APP_PRIVATE_KEY`. No `CARGO_REGISTRY_TOKEN` exists.
