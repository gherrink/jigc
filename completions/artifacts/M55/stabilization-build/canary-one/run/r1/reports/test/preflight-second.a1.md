# Preflight (second) — run `canary-one`, round 1, stage `test`, attempt 1

Reporter `preflight-second`. Status: **ready**.

- Candidate: label `c1`, commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, the tip of `fix/canary-one`, checked out.
- Steps this call covered, in order: (1) the environment asserts, (2) the trial image, built and verified from the candidate's commit. Nothing else was asked and nothing else was done: no binary built, no gate run, no deterministic check run, no CI read.
- Every fact below is copied from a command's output. Commands were run on 2026-10-08 from the repository root of the clone that holds the candidate.

## 1. The environment asserts

All held. Each as the command and what it printed.

### 1.1 The candidate is `HEAD`

```
$ git rev-parse HEAD
eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
$ git branch --show-current
fix/canary-one
```

Equal to the candidate's sha the prompt names. Read again after the image was verified: the same sha, the same branch.

### 1.2 The tree holds what the prompt says to expect and no more

Expected: untracked files under `completions/artifacts/canary-one/` and nothing else.

```
$ git status --porcelain
?? completions/artifacts/canary-one/r1/
$ git status --porcelain --untracked-files=all
?? completions/artifacts/canary-one/r1/reports/test/attempt.a1.md
?? completions/artifacts/canary-one/r1/reports/test/preflight.a1.md
?? completions/artifacts/canary-one/r1/reports/test/scope.a1.md
?? completions/artifacts/canary-one/r1/scope.md
```

Four untracked files, all under `completions/artifacts/canary-one/`; no modified, staged or deleted tracked file. The same four lines were printed again after the image was verified — this call changed nothing in the tree. None of those files was read by this reporter.

### 1.3 The scratch root exists and is writable

```
$ test -d <scratch root> && test -w <scratch root> && echo "scratch root: exists, writable"
scratch root: exists, writable
```

The one directory of this reporter was minted there with `mktemp -d`: `<scratch>/preflight-second.zKD1lS`. It holds the two logs and their exit codes and timestamps (section 2), and it is left as it stands.

### 1.4 Every tool the listed steps need answers

The listed steps need: the container runtime (the image), `git` and `bash` (the trial harness), the `claude` CLI on the host and a session auth variable (the harness's `verify-image.sh`, check 4 and its refusal line), and `python3`, `gitleaks` and a denylist (the record script, to take this report). `gh` is not needed by any listed step and was not looked for.

```
$ docker version --format 'client {{.Client.Version}} server {{.Server.Version}}'
client 29.7.0 server 29.5.2
$ docker context show
colima
$ docker info --format 'os={{.OperatingSystem}} arch={{.Architecture}} cpus={{.NCPU}} mem={{.MemTotal}} name={{.Name}}'
os=Ubuntu 24.04.4 LTS arch=aarch64 cpus=8 mem=16732606464 name=colima
$ gitleaks version
8.30.1
$ python3 --version
Python 3.9.6
$ claude --version
2.1.290 (Claude Code)
```

The denylist and the auth variable were asserted present without printing either:

```
denylist: readable, non-empty (source: default)
token: CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING is set
```

("source: default" is the record script's default location, used because `JIGC_DENYLIST_FILE` is unset.) `dev/stabilize-record --help` and `dev/stabilize-record state --run canary-one` both answered at exit 0; the state reads `"opened": true`, round 1, `"scope": true`.

### 1.5 One fact about the harness that the build depends on — stated, not repaired

`completions/trial-harness/build-image.sh` reads its source repository from `JIGC_REPO`, and where that is unset it falls back to a path fixed in the script (line 40) — a checkout that is **not** this clone. `JIGC_REPO` was unset in this shell, and that fallback checkout does not hold the candidate:

```
$ git -C <the script's fallback repository> cat-file -t eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
fatal: git cat-file: could not get object info
exit=128
```

So `build-image.sh <sha>` typed bare would have stopped at its own `git rev-parse` before any image existed. The image below was built with `JIGC_REPO` set, for that one command, to the clone that holds the candidate — the script's own knob, the script itself unmodified, no configuration changed. Whoever builds a later image of this run from this clone needs the same variable. This is a fact for the harness's owner to read; it is not a finding about the candidate.

## 2. The trial image

**Tag `jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` — built, and verified: 7 passed, 0 failed.** A trial arm can run on it.

### 2.1 The build

```
$ JIGC_REPO=<this clone> completions/trial-harness/build-image.sh eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
```

Exit code **0**, captured directly (output redirected to a file, no pipe). Started 2026-10-08T07:37:39Z, ended 2026-10-08T07:59:10Z. The log is 229 lines, `build-image.log` in this reporter's scratch directory, sha256 `bace7322715be476587434da5a2646204694f3e0fba2e9cc380ce27592ce931f`.

What the script printed before building:

```
building jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
  sha     : eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
  stamped : 1.0.0-rc.24
  subject : docs(record): canary-one - the small canary's synthetic opening
```

Its landmarks, by log line:

```
181:#11 1265.6    Compiling jigc-engine v0.1.0-rc.2 (/src/crates/engine)
183:#11 1269.8    Compiling jigc v1.0.0-rc.24 (/src/crates/cli)
184:#11 1284.6     Finished `release` profile [optimized] target(s) in 17m 41s
223:#21 exporting manifest list sha256:d37422f98361a9ca93d5e4afaf6ad117c8190acbb11ca371a6085fc268839d97 done
224:#21 naming to docker.io/library/jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a done
228:built jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a — verifying the binary reports the stamp its source carries
229:jigc 1.0.0-rc.24
```

The script's own stamp assertion (the built binary's `--version` against the version the commit's `crates/cli/Cargo.toml` carries) held: `jigc 1.0.0-rc.24` on both sides. The candidate prints the previous release's version string, as the run's record says it will; the image is identified by its sha, never by that string.

The only warning lines in the log are four identical ones from cargo's registry fetch inside the build container, at 403.6 s, each followed by a successful retry:

```
#11 403.6 warning: spurious network error (3 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
```

The log has no line carrying `error` other than those four. Most of the 17m 41s was that slow download, not compilation.

### 2.2 What the image is

```
$ docker image inspect --format 'id={{.Id}} created={{.Created}} os={{.Os}}/{{.Architecture}}' jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
id=sha256:d37422f98361a9ca93d5e4afaf6ad117c8190acbb11ca371a6085fc268839d97 created=2026-10-08T09:59:09.488828108+02:00 os=linux/arm64
```

And what it records about itself, read from inside it (one `docker run --rm --entrypoint sh <tag> -c '…'` printing the layout record, the source record, the executables list, `jigc --version` and `sha256sum` of the binary):

```
layout: single-binary
source: sha eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
executables: jigc
jigc 1.0.0-rc.24
6d9e1ef9ec1626346566273cfa339a79a9e83ceff4a508c7b40ae6b5a6a48041  /usr/local/bin/jigc
```

The source record names the candidate's full sha. The sha256 above is the hash of the **Linux arm64 binary inside the image**, built by the harness's Dockerfile from a `git archive` of the commit; it is not the candidate's host binary and is not offered as one — no binary is this reporter's to build or return.

### 2.3 The verification

```
$ completions/trial-harness/verify-image.sh jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a 1.0.0-rc.24
```

Exit code **0**, captured directly. Started 2026-10-08T07:59:37Z, ended 2026-10-08T07:59:57Z. The whole output (`verify-image.log`, sha256 `6a4cde0060ab4a732e851e37eccb3907508f6ba1720a80174706dfd3a6685304`):

```
== 1. the binary under test reports the version its tree carries
  PASS  jigc 1.0.0-rc.24
== 2. the doc-code probe runs, in the layout the image records (a dead probe reads as a validation family finding nothing)
  PASS  single-binary: no separate doc-code, and jigc's own probe child answers
== 3. no host INSTRUCTION files are present (the seeded onboarding state is not one)
  PASS  no CLAUDE.md, no memory, no skills, no agents
== 4. the discriminating probe flips (must be YES on the host, NO in here)
  PASS  host YES / container NO — host instructions are absent
== 5. a corpus round-trips with its git history intact
  PASS  history descends from 0e5ade43bada7745593d69a3f7863ed303734eb9 -> ce9f28c3b93e57d3231e06c983508519f6276bdd, and jigc setup landed inside the container
== 6. the workspace is trusted, so jigc's allowlist is honoured — and a transcript survives
  PASS  workspace trusted; jigc's allowlist is honoured
  PASS  a session transcript is recoverable (1 jsonl)

== 7 passed, 0 failed
```

Seven assertions in six numbered sections, none failed, none skipped. The expected version passed as the second argument, `1.0.0-rc.24`, is the stamp `build-image.sh` printed for this sha.

### 2.4 Bounds of what this establishes

- The verification is the harness's seven checks and nothing more. Its own README declares that check 2 proves the probe child loads and answers, not that it parses a real anchor.
- The two commit ids in check 5 belong to the throwaway corpus the script mints and removes; they are not commits of this repository.
- The harness scripts mint their own temporaries (the build context, the env file, the round-trip corpus) with `mktemp` in the system temporary directory and remove them in their own traps. Those are the tool's, run as it stands; this reporter's own writes are the one scratch directory named in 1.3 and this report.
- Check 4 ran one host `claude -p` call and one in-container call, and check 6 one more in-container call, each on the auth variable asserted in 1.4.

## 3. Return

| field | value |
|---|---|
| status | ready |
| image.tag | `jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` |
| image.verified | true |
| image.failed | none |
| checks | none — no deterministic check was listed for this call, so none is reported, green, red or void |
| halt | none |

Tree state at return: branch `fix/canary-one`, `HEAD` `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, the four untracked files of 1.2 and this report once the script places it. Nothing staged, nothing committed, no tracked file edited.

<!-- end of report -->
