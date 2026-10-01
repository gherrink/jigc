# The publish, proved from the registry — M54 Increment 11

This file records the first publish read back from the registry and from GitHub. Its first section is the **entry gate** of Increment 11 ([planning-gate-record.md](planning-gate-record.md) → row 21; S5, S9, S13), as amended by the human's option A on 2026-10-01. The tasks after T1 append their own sections: the registry install in `dev/runner-faithful` (T2), the trial harness's registry image (T3) and the README as crates.io renders it (T4). Cross-ref [release.md](../../../implementation/release.md) → Verifying a publish, Known gaps; [DECISIONS.md](../../../DECISIONS.md) → *M54 Increment 11 planning* (the basis each read is checked against), *Increment 11's installation leg is proved by three reads* (option A) and *M54 Inc 11 T1*.

## The entry gate (row 21)

**Verdict: every entry-gate read holds, and each equals the planning basis.** crates.io lists `jigc 1.0.0-rc.22` and `jigc-engine 0.1.0-rc.1`, both unyanked. Both tags are annotated and peel to `5dea9476`, the head of release PR #1. Both GitHub releases are prereleases and not drafts. `gherrink/jigc-release-rehearsal` answers 404. Run `36821202910` concluded `success` in all three jobs, and its `release-pr` job's `App token` step concluded `success`. Nothing differs, so the halt checklist is clear and nothing was repaired.

### The commands

Every value below is one line of this script's output, cited by its id. The script was run whole on 2026-10-01 with `gh` authenticated as `gherrink`, as `bash reads.sh`. A second whole run, executing this fenced block as extracted from this file, gave byte-identical output. crates.io is read with `curl` and a `User-Agent` naming the repository, as crates.io's data-access policy asks of API callers. Responses are saved to files first and read from there, never through a pipe whose exit status is read.

```sh
set -u
G=gherrink/jigc
UA='jigc-publish-proof (https://github.com/gherrink/jigc)'
D=$(mktemp -d "${TMPDIR:-/tmp}/publish-proof.XXXXXX")

echo "## E1"
for c in jigc jigc-engine; do
  curl -sS -A "$UA" "https://crates.io/api/v1/crates/$c/versions" > "$D/$c.json"; echo "$c rc=$?"
  jq -r '.versions[] | "  \(.crate) \(.num) yanked=\(.yanked) created_at=\(.created_at)"' "$D/$c.json"
done
echo "## E2"
gh api repos/$G/git/matching-refs/tags/jigc --jq '.[] | "\(.ref) \(.object.type) \(.object.sha)"'
for t in jigc-engine-v0.1.0-rc.1 jigc-v1.0.0-rc.22; do
  S=$(gh api repos/$G/git/ref/tags/$t --jq .object.sha)
  gh api repos/$G/git/tags/$S --jq '"\(.tag) tag-object=\(.sha) peels-to=\(.object.type) \(.object.sha)"'
done
echo "## E3"
gh api repos/$G/releases --jq '.[] | "\(.tag_name) name=\(.name) prerelease=\(.prerelease) draft=\(.draft) target=\(.target_commitish) published_at=\(.published_at) author=\(.author.login)"'
echo "## E4"
gh api repos/gherrink/jigc-release-rehearsal > "$D/rehearsal.json" 2> "$D/rehearsal.err"; echo "rc=$?"
jq -c '{message, status}' "$D/rehearsal.json"
cat "$D/rehearsal.err"
echo "## E5"
gh run view 36821202910 -R $G --json databaseId,workflowName,headBranch,headSha,event,status,conclusion,attempt --jq '"\(.databaseId) \(.workflowName) \(.headBranch) \(.headSha) \(.event) \(.status) \(.conclusion) attempt=\(.attempt)"'
gh run view 36821202910 -R $G --json jobs --jq '.jobs[] | "  \(.databaseId) \(.name) \(.status) \(.conclusion)"'
gh api repos/$G/commits/6d7f6032debc586b30081373eefd772664d95174 --jq '"merge 6d7f6032 parents=\([.parents[].sha[0:8]]|join(","))"'
echo "## E6"
gh run view 36821202910 -R $G --json jobs --jq '.jobs[] | select(.name=="release-pr") | .steps[] | select(.name=="App token") | "release-pr step \(.number) \(.name) \(.status) \(.conclusion)"'
```

E1 and E3 can change after this record: H4 yanks both `0.0.0` versions, and a later release adds rows. The values below are the ones read on 2026-10-01, before H4.

### The registry lists both versions (E1)

```
jigc rc=0
  jigc 1.0.0-rc.22 yanked=false created_at=2026-10-01T06:14:50.942770Z
  jigc 0.0.0 yanked=false created_at=2026-07-25T07:05:47.711285Z
jigc-engine rc=0
  jigc-engine 0.1.0-rc.1 yanked=false created_at=2026-10-01T06:14:20.618164Z
  jigc-engine 0.0.0 yanked=false created_at=2026-09-28T11:48:58.816966Z
```

- **Both published versions are listed and unyanked:** `jigc 1.0.0-rc.22` and `jigc-engine 0.1.0-rc.1`.
- **Engine first:** `jigc-engine` was created at 06:14:20Z and `jigc` 30 s later at 06:14:50Z, the order release-plz publishes in.
- **Both `0.0.0` placeholders are still unyanked.** H4 yanks them after this increment, so this is the expected state, and it is the state T2's controls install from.

### Both tags are annotated and peel to the release head (E2)

```
refs/tags/jigc-engine-v0.1.0-rc.1 tag ef8053ed039b0c752a86d8059d3d094a4e4e5548
refs/tags/jigc-v1.0.0-rc.22 tag 15a3cbace799e5333c7929f18c4b53790bddfcd5
jigc-engine-v0.1.0-rc.1 tag-object=ef8053ed039b0c752a86d8059d3d094a4e4e5548 peels-to=commit 5dea947687c3b23cb306ffa52395a6007c0e4541
jigc-v1.0.0-rc.22 tag-object=15a3cbace799e5333c7929f18c4b53790bddfcd5 peels-to=commit 5dea947687c3b23cb306ffa52395a6007c0e4541
```

- **Exactly two `jigc*` tags exist:** `jigc-engine-v0.1.0-rc.1` and `jigc-v1.0.0-rc.22`.
- **Both are annotated:** each ref points at a tag object (`ef8053ed`, `15a3cbac`), not at a commit.
- **Both peel to `5dea9476`**, `chore(release): prepare release`, which is release PR #1's head. E5 shows that commit is the merge's second parent.

### Both releases are prereleases, not drafts (E3)

```
jigc-v1.0.0-rc.22 name=jigc-v1.0.0-rc.22 prerelease=true draft=false target=main published_at=2026-10-01T06:14:54Z author=github-actions[bot]
jigc-engine-v0.1.0-rc.1 name=jigc-engine-v0.1.0-rc.1 prerelease=true draft=false target=main published_at=2026-10-01T06:14:23Z author=github-actions[bot]
```

The repository holds exactly these two releases. Both have `prerelease=true` and `draft=false`. Each was published 3–4 s after its crate was created on the registry.

### The installation leg, by option A's three reads

The leg *the App's installations list `gherrink/jigc` only* is proved by three reads, because no credential an agent holds can list the App's installations ([DECISIONS.md](../../../DECISIONS.md) → *Increment 11's installation leg is proved by three reads, not by listing the installations*). That agents cannot read the selection is a declared bound ([release.md](../../../implementation/release.md) → Known gaps).

**(1) The rehearsal repository answers 404 (E4).**

```
rc=1
{"message":"Not Found","status":"404"}
gh: Not Found (HTTP 404)
```

The repository was deleted on 2026-10-01, after the human had uninstalled the App from it, so it cannot remain in any installation's selection.

**(2) Run `36821202910` passed its `App token` step (E5, E6).**

```
36821202910 Release main 6d7f6032debc586b30081373eefd772664d95174 push completed success attempt=1
  110236951753 check completed success
  110236952001 release-pr completed success
  110237059468 release completed success
merge 6d7f6032 parents=5974880f,5dea9476
release-pr step 2 App token completed success
```

- The run is the `Release` workflow on the push of `6d7f6032`, the merge of release PR #1 into `main`. Its parents are `5974880f` (`main` before the merge) and `5dea9476` (the PR head both tags peel to).
- **All three jobs concluded `success`:** `check`, `release-pr` and `release`. `release` is the job that published both crates.
- **The `release-pr` job's step 2, `App token`, concluded `success`.** The App could mint a token for `gherrink/jigc`, so the App is still installed there.

**(3) The human's read of the installation**, quoted verbatim from the 2026-10-01 option-A entry in [DECISIONS.md](../../../DECISIONS.md) (2026-10-01, Settings → Applications → Installed GitHub Apps → gherrink-jigc-release):

> *"Repository access: Only select repositories — Selected 1 repository: gherrink/jigc"*; permissions shown: *"Read access to metadata"*, *"Read and write access to code and pull requests"*.

### The halt checklist

| Halt if | Read | Holds |
|---|---|---|
| a version is absent | E1: `jigc 1.0.0-rc.22`, `jigc-engine 0.1.0-rc.1` | yes |
| a tag is missing, or does not peel to `5dea9476` | E2: both present, both `peels-to=commit 5dea9476…` | yes |
| a release is not a prerelease | E3: both `prerelease=true draft=false` | yes |
| the rehearsal repository answers anything but 404 | E4: `"status":"404"` | yes |
| the `App token` step did not succeed | E6: `App token completed success` | yes |
