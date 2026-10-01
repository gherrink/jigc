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

## The registry install in `dev/runner-faithful` (T2)

**Verdict: the install line installs `jigc 1.0.0-rc.22` from crates.io under `--locked`, and the installed binary passes every step, on `linux/arm64` and on `linux/amd64`.** It reports the version `jigc 1.0.0-rc.22`. `jigc setup` exits 0 and leaves `.jigc/AGENT.md`. With `JIGC_DOC_CODE_PROBE` unset, the control finalize exits 0, and the drift finalize exits 3 naming `doc-code.symbol-exists` with `HEAD` unchanged. Both controls install the `0.0.0` placeholder and exit 1 at `FAIL version`, so the line resolves `1.0.0-rc.22` and not `0.0.0`. Nothing failed, so `dev/runner-faithful` is unchanged. This is S13's positive half and S9's first faithful registry proof ([release.md](../../../implementation/release.md) → Verifying a publish, Installing).

### The commands

The four runs were made on 2026-10-01 in one session, in the order below, before H4. Each ran from a clean tree at `48f411d4`, whose `rust-toolchain.toml` names the image's toolchain; no source tree enters the container in this mode. Each run's output (stdout and stderr) went to a file, and its exit status was read bare. Its wall clock is `date +%s` before and after, on the host, and covers the cached image build, a fresh container, the install from crates.io and the steps. The requirement `^1.0.0-rc.1` is the install line's, as `crates/cli/guides/QUICKSTART.md` → Install states it.

```sh
dev/runner-faithful registry '^1.0.0-rc.1'                         # R1
dev/runner-faithful --platform linux/amd64 registry '^1.0.0-rc.1'  # R2
dev/runner-faithful registry '=0.0.0'                              # R3, control
dev/runner-faithful registry                                       # R4, control (unpinned)
```

| Run | Started (UTC) | Platform | Exit | Wall clock | Stops at |
|---|---|---|---|---|---|
| R1 | 07:33:44Z | `linux/aarch64` | **0** | 32 s | — (every step `ok`) |
| R2 | 07:34:24Z | `linux/x86_64` (emulated) | **0** | 90 s | — (every step `ok`) |
| R3 | 07:36:00Z | `linux/aarch64` | **1** | 3 s | `FAIL version` |
| R4 | 07:36:03Z | `linux/aarch64` | **1** | 1 s | `FAIL version` |

All four print the same header apart from the platform: `cpus 8`, `toolchain rustc 1.95.0 (59807616e 2026-04-14)` and `git version 2.43.0`. The controls are fast because the placeholder has no dependencies.

### R1 — `linux/arm64`, the host

```
runner-faithful: the toolchain is read from commit 48f411d4d2cc325d855670ce5f37a33f7a633765; no source tree enters the container
runner-faithful: building jigc-runner-faithful:1.95.0-linux-arm64 (cached layers are reused); log: /var/folders/nj/dq5nt8bj72xc0ppkm69y_ckh0000gn/T/runner-faithful.4gUF51/build.log
runner-faithful: platform  linux/aarch64
runner-faithful: cpus      8
runner-faithful: toolchain rustc 1.95.0 (59807616e 2026-04-14)
runner-faithful: git       git version 2.43.0
runner-faithful: running   the registry install acceptance
runner-faithful: ok   install: cargo install jigc --version ^1.0.0-rc.1 --locked installed jigc 1.0.0-rc.22
runner-faithful: ok   version: jigc 1.0.0-rc.22 (/home/runner/.cargo/bin/jigc)
runner-faithful: ok   setup: jigc setup exited 0; .jigc/AGENT.md present
runner-faithful: ok   probe-override: JIGC_DOC_CODE_PROBE unset
runner-faithful: ok   control-finalize: arch-doc:padding-layer anchored at src/pad.ts#pad finalized with exit 0
runner-faithful: ok   drift-finalize: a task editing arch-doc:padding-layer finalized with exit 3, naming doc-code.symbol-exists; HEAD unchanged
runner-faithful: the registry install acceptance passed
runner-faithful: exit 0
```

### R2 — `linux/amd64`, the runner's architecture

```
runner-faithful: the toolchain is read from commit 48f411d4d2cc325d855670ce5f37a33f7a633765; no source tree enters the container
runner-faithful: building jigc-runner-faithful:1.95.0-linux-amd64 (cached layers are reused); log: /var/folders/nj/dq5nt8bj72xc0ppkm69y_ckh0000gn/T/runner-faithful.n7O3wu/build.log
runner-faithful: platform  linux/x86_64
runner-faithful: cpus      8
runner-faithful: toolchain rustc 1.95.0 (59807616e 2026-04-14)
runner-faithful: git       git version 2.43.0
runner-faithful: running   the registry install acceptance
runner-faithful: ok   install: cargo install jigc --version ^1.0.0-rc.1 --locked installed jigc 1.0.0-rc.22
runner-faithful: ok   version: jigc 1.0.0-rc.22 (/home/runner/.cargo/bin/jigc)
runner-faithful: ok   setup: jigc setup exited 0; .jigc/AGENT.md present
runner-faithful: ok   probe-override: JIGC_DOC_CODE_PROBE unset
runner-faithful: ok   control-finalize: arch-doc:padding-layer anchored at src/pad.ts#pad finalized with exit 0
runner-faithful: ok   drift-finalize: a task editing arch-doc:padding-layer finalized with exit 3, naming doc-code.symbol-exists; HEAD unchanged
runner-faithful: the registry install acceptance passed
runner-faithful: exit 0
```

### R3 — the `=0.0.0` control

```
runner-faithful: the toolchain is read from commit 48f411d4d2cc325d855670ce5f37a33f7a633765; no source tree enters the container
runner-faithful: building jigc-runner-faithful:1.95.0-linux-arm64 (cached layers are reused); log: /var/folders/nj/dq5nt8bj72xc0ppkm69y_ckh0000gn/T/runner-faithful.rLjD40/build.log
runner-faithful: platform  linux/aarch64
runner-faithful: cpus      8
runner-faithful: toolchain rustc 1.95.0 (59807616e 2026-04-14)
runner-faithful: git       git version 2.43.0
runner-faithful: running   the registry install acceptance
runner-faithful: ok   install: cargo install jigc --version =0.0.0 --locked installed jigc 0.0.0
runner-faithful: FAIL version: jigc --version printed "jigc — a context compiler for coding agents.
This crate name is reserved; the 1.0 release is imminent.
See https://github.com/gherrink/jigc", expected "jigc 0.0.0"
runner-faithful: exit 1
```

### R4 — the unpinned control, and Increment 12's before-yank baseline

```
runner-faithful: the toolchain is read from commit 48f411d4d2cc325d855670ce5f37a33f7a633765; no source tree enters the container
runner-faithful: building jigc-runner-faithful:1.95.0-linux-arm64 (cached layers are reused); log: /var/folders/nj/dq5nt8bj72xc0ppkm69y_ckh0000gn/T/runner-faithful.xJYIhP/build.log
runner-faithful: platform  linux/aarch64
runner-faithful: cpus      8
runner-faithful: toolchain rustc 1.95.0 (59807616e 2026-04-14)
runner-faithful: git       git version 2.43.0
runner-faithful: running   the registry install acceptance
runner-faithful: ok   install: cargo install jigc --locked (unpinned) installed jigc 0.0.0
runner-faithful: FAIL version: jigc --version printed "jigc — a context compiler for coding agents.
This crate name is reserved; the 1.0 release is imminent.
See https://github.com/gherrink/jigc", expected "jigc 0.0.0"
runner-faithful: exit 1
```

- **Before H4, the unpinned install resolves the placeholder.** cargo never selects a prerelease unasked, so the only version it can choose is the unyanked `0.0.0`. Once H4 yanks it, Increment 12 expects the same command to resolve nothing.
- **Both controls stop at `version`, not `install`.** The placeholder installs and runs, but it prints its reservation notice instead of a version.

### The package the install built

The published crate was downloaded from `https://static.crates.io/crates/jigc/jigc-1.0.0-rc.22.crate` and unpacked on the host, and two values were read from the crates.io API (`/api/v1/crates/<name>/<version>`).

- **The download is the published crate.** Its SHA-256, `16e53fa0303ca578751029ff648c884c9d40c98add692f67b172025c41b12081`, equals the API's `checksum` for `jigc 1.0.0-rc.22`.
- **The package has a single `[[bin]]`**: `name = "jigc"`, `path = "src/main.rs"`, beside the `[lib]` `cli` (`src/lib.rs`). The API agrees, with `bin_names=["jigc"]` and `has_lib=true`. So the install puts exactly one file in `~/.cargo/bin`, and the `doc-code` probe that blocked R1's and R2's drift finalize ran as that binary's self-exec child. With `JIGC_DOC_CODE_PROBE` unset, there is no other probe it could have been.
- **The lock's engine entry is the published engine.** The packaged `Cargo.lock` carries `jigc-engine 0.1.0-rc.1` with `source = "registry+https://github.com/rust-lang/crates.io-index"` and `checksum = "413b203508eb034811d70cb7a39e885c5da6011426b67576594813d23cb9a8d2"`. That equals the API's `checksum` for `jigc-engine 0.1.0-rc.1`. The manifest's `=0.1.0-rc.1` pin names the same version.

### The published lock resolves to itself against the registry

cargo 1.95's `--locked` does not refuse an incomplete or stale lock (S9's bound). So R1 and R2 exiting 0 does not by itself show that the install built from the published lock as written. The pre-publish `lock-compare` (Increment 7) could not close this, because it needed the overlay, and the overlay bypasses the engine's entry. With the engine on crates.io, the same compare runs without an overlay. It ran on the host, in a copy of the unpacked published crate, outside any workspace, with the toolchain pinned:

```sh
cp -R jigc-1.0.0-rc.22 resolved
( cd resolved && cargo +1.95.0 metadata --format-version 1 >/dev/null )   # rc=0: "Downloaded jigc-engine v0.1.0-rc.1"
cmp jigc-1.0.0-rc.22/Cargo.lock resolved/Cargo.lock                      # rc=0
```

- **The published lock is byte-equal to what cargo resolves from it against crates.io**, with no rewrite removed. Every entry is present and current, including the engine's registry entry. So the `--locked` install built exactly the lock's entries.
- **The compare discriminates.** As a control, the `anyhow` entry was dropped from a copy of the lock and the same resolution was run. Cargo re-added `anyhow` at `1.0.104`, the index's newest, where the published lock pins `1.0.102`. The resolved lock then differed from the planted one (`cmp` rc=1).

The install log itself is not kept on success: the container is removed, and the script prints only the step lines.
