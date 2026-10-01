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

## The trial harness's registry image (T3)

**Verdict: `build-image.sh --registry 1.0.0-rc.22` builds the image from crates.io and exits 0, and `verify-image.sh` passes all seven checks on it, with check 2 on the `single-binary` arm.** The build installs `jigc 1.0.0-rc.22` and its engine `0.1.0-rc.1` from the registry. It records the layout `single-binary` and the source `registry jigc 1.0.0-rc.22`, and the binary prints the stamp `jigc 1.0.0-rc.22`. No source tree of ours enters the build context. Nothing failed, so the harness is unchanged. This is S14's positive run. The blind trial's instrument now exists for the published version ([trial-harness README](../../trial-harness/README.md) → Two ways to build an image; [release.md](../../../implementation/release.md) → Verifying a publish).

### The commands

Both runs were made on 2026-10-01, after T2 and before H4, from `completions/trial-harness/` on a clean tree at `afa899b0`. The host is macOS/arm64 under colima (Docker 29.5.2), so the image is `linux/arm64`. Each run's output (stdout and stderr) went to a file, and its exit status was read bare. `CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING` was set, so `verify-image.sh` did not refuse.

```sh
./build-image.sh --registry 1.0.0-rc.22                          # B1
./verify-image.sh jigc-gate:registry-1.0.0-rc.22 1.0.0-rc.22     # V1
```

| Run | Started (UTC) | Exit | Wall clock | Result |
|---|---|---|---|---|
| B1 | 07:56:59Z | **0** | 35 s | image `jigc-gate:registry-1.0.0-rc.22`, `sha256:04ae7455f965…`, `arm64` |
| V1 | 07:57:44Z | **0** | 18 s | `7 passed, 0 failed` |

### B1 — the build

The script's own header, printed before the build:

```
building jigc-gate:registry-1.0.0-rc.22
  source  : crates.io, jigc 1.0.0-rc.22 (cargo install --locked)
  stamped : 1.0.0-rc.22
```

The install step, from the build log (BuildKit's step `#11`, the `build` stage's `cargo install jigc --version "1.0.0-rc.22" --locked --root /stage`):

```
#11 1.529   Downloaded jigc v1.0.0-rc.22
#11 1.669   Installing jigc v1.0.0-rc.22
#11 5.784   Downloaded jigc-engine v0.1.0-rc.1
#11 31.69     Finished `release` profile [optimized] target(s) in 31.39s
#11 31.72   Installing /stage/bin/jigc
#11 31.72    Installed package `jigc v1.0.0-rc.22` (executable `jigc`)
#11 DONE 32.0s
```

The build-time `ldd` gate over every staged executable, which also prints the image's records (step `#16`):

```
#16 0.159 layout: single-binary
#16 0.159 source: registry jigc 1.0.0-rc.22
#16 0.164 jigc 1.0.0-rc.22
```

The script's stamp assertion, after the image is tagged:

```
built jigc-gate:registry-1.0.0-rc.22 — verifying the binary reports the stamp its source carries
jigc 1.0.0-rc.22
```

- **The install fetched both published crates:** `jigc v1.0.0-rc.22` and `jigc-engine v0.1.0-rc.1`. The release build took 31.39 s. The other build steps were cached layers (the base image, its packages and the pinned Claude CLI).
- **The install staged exactly one executable, `jigc`.** A read of the finished image afterwards shows `/usr/local/share/jigc-image/executables` lists only `jigc`. `layout` reads `single-binary` and `source` reads `registry jigc 1.0.0-rc.22`. The Claude CLI is `2.1.233 (Claude Code)`, and `/usr/local/bin` holds no `doc-code`.

### V1 — the verification

```
== 1. the binary under test reports the version its tree carries
  PASS  jigc 1.0.0-rc.22
== 2. the doc-code probe runs, in the layout the image records (a dead probe reads as a validation family finding nothing)
  PASS  single-binary: no separate doc-code, and jigc's own probe child answers
== 3. no host INSTRUCTION files are present (the seeded onboarding state is not one)
  PASS  no CLAUDE.md, no memory, no skills, no agents
== 4. the discriminating probe flips (must be YES on the host, NO in here)
  PASS  host YES / container NO — host instructions are absent
== 5. a corpus round-trips with its git history intact
  PASS  history descends from 910005855d6a4a82efbf6ce923e72fa7880443d5 -> 7769e09545bfc3728533cd7fb72a49477fc00699, and jigc setup landed inside the container
== 6. the workspace is trusted, so jigc's allowlist is honoured — and a transcript survives
  PASS  workspace trusted; jigc's allowlist is honoured
  PASS  a session transcript is recoverable (1 jsonl)

== 7 passed, 0 failed
```

- **Check 2 ran the `single-binary` arm, because that is the layout the image records.** The arm asserts three things. No `/usr/local/bin/doc-code` is present. `jigc __nope` exits 2, which is clap's answer to an unknown argv. `jigc __probe doc-code --build 1.0.0-rc.22`, fed an empty-anchor request, answers `"findings":[]` at exit 0. So the self-exec probe child of the published binary runs in the image.
- **Check 5 ran `jigc setup` with the published binary**, inside a container that a corpus was copied into and back out of, and the corpus's history came back intact.
- **This is the opposite of Increment 7's pre-publish run.** Then, `--registry 1.0.0-rc.22` stopped before tagging, because cargo could not find the version, and the `0.0.0` image failed checks 1, 2 and 5.

### Bounds

- **One platform:** the image is `linux/arm64`, the host's. `build-image.sh` does not take a platform. The `linux/amd64` registry install is T2's R2.
- **The image is local.** It is not pushed anywhere. A trial session rebuilds it with the same command, and crates.io never serves two builds under one version, so the rebuild installs the same crates. Its dependencies are the published lock's, which T2 showed resolves to itself.

## The README as crates.io renders it (T4)

**Verdict: crates.io breaks four of the README's six link targets.** The README carries seven relative links to six targets, and `QUICKSTART.md` is linked twice. crates.io renders each one as `https://github.com/gherrink/jigc/blob/HEAD/crates/cli/<link>`. It resolves a relative link against the package's `path_in_vcs`, which is `crates/cli`, and not against the repository root that the links are written for. The two guide links double the prefix to `crates/cli/crates/cli/guides/…`. `VISION.md` and `WHY-JIGC.md` land on `crates/cli/VISION.md` and `crates/cli/WHY-JIGC.md`. None of those paths exists, so those five rendered links answer **404**. The two license links answer **200**, because `crates/cli/LICENSE-*` exist, but they are the symlinks the package is built from. GitHub's page for each one shows the one-line target, `../../LICENSE-APACHE` or `../../LICENSE-MIT`, and not the license text. Every source link resolves from the repository root, and each root URL answers 200, so the README is right on GitHub and wrong only where crates.io re-roots it. **Neither README is edited here.** The fix touches settled ground and is the human's to choose. It is keyed at [decisions-pending.md](../../../implementation/decisions-pending.md) → *The road to 1.0.0 and the port* → *The crates.io README's relative links*, with the trigger *before the next release PR is merged*, because only a publish changes what crates.io shows ([release.md](../../../implementation/release.md) → What the package carries).

### The commands

Every value below is one line of this script's output. The script was run whole on 2026-10-01, after T3 and before H4, from the repository root (its `git` reads need the tagged commit `5dea9476`), as `bash readme-links.sh`. A second whole run, executing this fenced block as extracted from this file, gave byte-identical output. Each status is `curl`'s `%{http_code}` for a plain GET, with no redirect followed. The README endpoint is the one exception: it answers 302 to the static HTML, so it is fetched with `-L`. The source links are read from the `README.md` inside the published `.crate`, which is what crates.io rendered. Responses are saved to files first and read from there, never through a pipe whose exit status is read.

```sh
set -u
UA='jigc-publish-proof (https://github.com/gherrink/jigc)'
API=https://crates.io/api/v1/crates/jigc/1.0.0-rc.22
GH=https://github.com/gherrink/jigc/blob
D=$(mktemp -d "${TMPDIR:-/tmp}/publish-proof.XXXXXX")

echo "## L1"
curl -sS -L -A "$UA" -o "$D/readme.html" -w 'readme http=%{http_code} served-from=%{url_effective}\n' "$API/readme"
curl -sS -L -A "$UA" -o "$D/jigc.crate" "$API/download"; echo "crate rc=$?"
shasum -a 256 "$D/jigc.crate" > "$D/sum"; echo "crate sha256=$(cut -c1-64 "$D/sum")"
tar -xzf "$D/jigc.crate" -C "$D" jigc-1.0.0-rc.22/README.md jigc-1.0.0-rc.22/.cargo_vcs_info.json
jq -r '"path_in_vcs=\(.path_in_vcs) sha1=\(.git.sha1)"' "$D/jigc-1.0.0-rc.22/.cargo_vcs_info.json"
git show 5dea9476:README.md > "$D/root-readme.md"
cmp -s "$D/jigc-1.0.0-rc.22/README.md" "$D/root-readme.md"; echo "packaged README == root README at 5dea9476: cmp rc=$?"

echo "## L2"
grep -o '](\([^)]*\))' "$D/jigc-1.0.0-rc.22/README.md" > "$D/src.raw"
sed 's/^](//; s/)$//' "$D/src.raw" > "$D/src"
grep -o 'href="[^"]*"' "$D/readme.html" > "$D/href.raw"
sed 's/^href="//; s/"$//' "$D/href.raw" > "$D/href.all"
grep -v '^#' "$D/href.all" > "$D/href.ext"
grep '^#' "$D/href.all" > "$D/href.anchor"
echo "source links=$(wc -l < "$D/src" | tr -d ' ') rendered hrefs=$(wc -l < "$D/href.all" | tr -d ' ') external=$(wc -l < "$D/href.ext" | tr -d ' ') anchors=$(wc -l < "$D/href.anchor" | tr -d ' ')"
paste -d ' ' "$D/src" "$D/href.ext" > "$D/pairs"
n=0
while read -r src href; do
  n=$((n + 1))
  rendered=$(curl -sS -A "$UA" -o /dev/null -w '%{http_code}' "$href")
  git cat-file -e "5dea9476:$src" 2> /dev/null && at_root=exists || at_root=absent
  root=$(curl -sS -A "$UA" -o /dev/null -w '%{http_code}' "$GH/HEAD/$src")
  echo "$n source=$src"
  echo "  rendered=$href http=$rendered"
  echo "  control: $src at the repo root (5dea9476) $at_root; $GH/HEAD/$src http=$root"
done < "$D/pairs"

echo "## L3"
while read -r a; do
  id=${a#\#}
  echo "$a ids-in-page=$(grep -c "id=\"$id\"" "$D/readme.html")"
done < "$D/href.anchor"

echo "## L4"
for f in LICENSE-APACHE LICENSE-MIT; do
  for p in "crates/cli/$f" "$f"; do
    curl -sS -A "$UA" -o "$D/page.html" "$GH/HEAD/$p"
    grep -o '"rawLines":\["[^"]*"' "$D/page.html" > "$D/first"
    echo "$p first-line=$(sed 's/^"rawLines":\[//; s/  */ /g' "$D/first")"
  done
done
```

The rendered README is fixed by the version: crates.io never re-renders a published version. The GitHub statuses read `HEAD`, so they hold only while `main` keeps the same paths. A `crates/cli/VISION.md` added later would turn its 404 into a 200 without any publish.

### The package the README came from (L1)

```
readme http=200 served-from=https://static.crates.io/readmes/jigc/jigc-1.0.0-rc.22.html
crate rc=0
crate sha256=16e53fa0303ca578751029ff648c884c9d40c98add692f67b172025c41b12081
path_in_vcs=crates/cli sha1=5dea947687c3b23cb306ffa52395a6007c0e4541
packaged README == root README at 5dea9476: cmp rc=0
```

- **The `.crate` is the published one.** Its sha256 equals the registry's `checksum` for `1.0.0-rc.22` (`16e53fa0…`).
- **`path_in_vcs` is `crates/cli`,** which is the base crates.io resolves relative links against. `sha1` is the tagged head `5dea9476`.
- **The packaged `README.md` is byte-equal to the root `README.md` at `5dea9476`.** Cargo copied it in through `readme = "../../README.md"`, so the links crates.io rewrote are the root README's own.

### Every rendered link, beside its source and its status (L2)

```
source links=7 rendered hrefs=12 external=7 anchors=5
1 source=crates/cli/guides/QUICKSTART.md
  rendered=https://github.com/gherrink/jigc/blob/HEAD/crates/cli/crates/cli/guides/QUICKSTART.md http=404
  control: crates/cli/guides/QUICKSTART.md at the repo root (5dea9476) exists; https://github.com/gherrink/jigc/blob/HEAD/crates/cli/guides/QUICKSTART.md http=200
2 source=crates/cli/guides/QUICKSTART.md
  rendered=https://github.com/gherrink/jigc/blob/HEAD/crates/cli/crates/cli/guides/QUICKSTART.md http=404
  control: crates/cli/guides/QUICKSTART.md at the repo root (5dea9476) exists; https://github.com/gherrink/jigc/blob/HEAD/crates/cli/guides/QUICKSTART.md http=200
3 source=crates/cli/guides/MIGRATING.md
  rendered=https://github.com/gherrink/jigc/blob/HEAD/crates/cli/crates/cli/guides/MIGRATING.md http=404
  control: crates/cli/guides/MIGRATING.md at the repo root (5dea9476) exists; https://github.com/gherrink/jigc/blob/HEAD/crates/cli/guides/MIGRATING.md http=200
4 source=VISION.md
  rendered=https://github.com/gherrink/jigc/blob/HEAD/crates/cli/VISION.md http=404
  control: VISION.md at the repo root (5dea9476) exists; https://github.com/gherrink/jigc/blob/HEAD/VISION.md http=200
5 source=WHY-JIGC.md
  rendered=https://github.com/gherrink/jigc/blob/HEAD/crates/cli/WHY-JIGC.md http=404
  control: WHY-JIGC.md at the repo root (5dea9476) exists; https://github.com/gherrink/jigc/blob/HEAD/WHY-JIGC.md http=200
6 source=LICENSE-APACHE
  rendered=https://github.com/gherrink/jigc/blob/HEAD/crates/cli/LICENSE-APACHE http=200
  control: LICENSE-APACHE at the repo root (5dea9476) exists; https://github.com/gherrink/jigc/blob/HEAD/LICENSE-APACHE http=200
7 source=LICENSE-MIT
  rendered=https://github.com/gherrink/jigc/blob/HEAD/crates/cli/LICENSE-MIT http=200
  control: LICENSE-MIT at the repo root (5dea9476) exists; https://github.com/gherrink/jigc/blob/HEAD/LICENSE-MIT http=200
```

| # | README source link | Rendered `href` (under `https://github.com/gherrink/jigc/blob/HEAD/`) | HTTP | Same link from the repo root |
|---|---|---|---|---|
| 1 | `crates/cli/guides/QUICKSTART.md` | `crates/cli/crates/cli/guides/QUICKSTART.md` | **404** | 200 |
| 2 | `crates/cli/guides/QUICKSTART.md` | `crates/cli/crates/cli/guides/QUICKSTART.md` | **404** | 200 |
| 3 | `crates/cli/guides/MIGRATING.md` | `crates/cli/crates/cli/guides/MIGRATING.md` | **404** | 200 |
| 4 | `VISION.md` | `crates/cli/VISION.md` | **404** | 200 |
| 5 | `WHY-JIGC.md` | `crates/cli/WHY-JIGC.md` | **404** | 200 |
| 6 | `LICENSE-APACHE` | `crates/cli/LICENSE-APACHE` | 200 (the symlink page) | 200 |
| 7 | `LICENSE-MIT` | `crates/cli/LICENSE-MIT` | 200 (the symlink page) | 200 |

- **The pairing is by order and is checked by count.** The README has seven links, and the rendered page has seven external `href`s in the same order. Its other five `href`s are heading anchors that crates.io adds (L3).
- **The controls discriminate.** Every source path exists at `5dea9476`, and every root-relative URL answers 200. So the 404s come from the re-rooting and not from GitHub, the repository's visibility or a moved file.

### The heading anchors crates.io adds (L3)

```
#user-content-jigc ids-in-page=1
#user-content-install ids-in-page=1
#user-content-guides ids-in-page=1
#user-content-the-cli-library-is-not-an-api ids-in-page=1
#user-content-license ids-in-page=1
```

Each anchor is an `href="#user-content-…"` that crates.io adds to a heading. Each matches exactly one `id` on the same page, so none is broken. They have no source link in the README, and `curl` does not apply to them.

### The two license links land on symlink pages (L4)

```
crates/cli/LICENSE-APACHE first-line="../../LICENSE-APACHE"
LICENSE-APACHE first-line=" Apache License"
crates/cli/LICENSE-MIT first-line="../../LICENSE-MIT"
LICENSE-MIT first-line="MIT License"
```

The first line of each page's blob view (GitHub's `rawLines`): at `crates/cli/` it is the symlink's target, and at the root it is the license text. So links 6 and 7 do not 404, but a reader who follows them sees a path, not the license.
