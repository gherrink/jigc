# Preflight — run `canary-one`, round 1, stage `test`, attempt 2

**Status: ready.** Every environment assert this call covers holds. This call covered
one step — the environment asserts — and nothing else: no binary was built, no gate
was run, no image was built, and no deterministic check was asked for, so this report
holds no green, red or void entry and no image.

- Candidate: label `c1`, commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`.
- The candidate is **not** `HEAD` in this call: it is the commit round 1 tested, and
  the round's record commit lies on top of it. No step of this call reads the working
  tree, and none was listed beside the candidate.
- All commands below were run from the repository root, each bare, its exit status
  read directly.

## The environment asserts

### 1. The candidate is a commit the checked-out branch holds

```
$ git rev-parse HEAD
126a85311a54e73e9f9798f3d034d24f14e3ea7c

$ git branch --show-current
fix/canary-one

$ git merge-base --is-ancestor eeffe347324f83a51d1ae83d5f254e73c3f1ea3a HEAD; echo "exit=$?"
exit=0
```

Holds: the ancestry test exits 0. What lies between the candidate and `HEAD`:

```
$ git log --format='%H %s' eeffe347324f83a51d1ae83d5f254e73c3f1ea3a^..HEAD
126a85311a54e73e9f9798f3d034d24f14e3ea7c docs(record): canary-one r1 - the record of the test stage
eeffe347324f83a51d1ae83d5f254e73c3f1ea3a docs(record): canary-one - the small canary's synthetic opening
```

One commit on top of the candidate, a record commit of this run's round 1.

The run's own record names the same commit as the candidate
(`dev/stabilize-record state --run canary-one`, read with a JSON parser, not by eye):

```
/rounds/0/facts/candidate = eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
/candidate/commit = eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
```

### 2. The tree shows what the prompt says to expect and no more

```
$ git status --porcelain
?? completions/artifacts/canary-one/r1/reports/test/attempt.a2.md
```

Holds: one untracked file, under `completions/artifacts/canary-one/`, which is the one
place the prompt allows untracked files. No tracked file is modified, nothing is
staged. That file was not read: it is another reporter's.

### 3. The tools the listed steps need answer

The one thing this call does beyond the asserts is hand this report to
`dev/stabilize-record`, which names its needs as python3, git, bash, gitleaks and a
denylist.

```
$ python3 --version; echo "python3 exit=$?"
Python 3.9.6
python3 exit=0

$ git --version; echo "git exit=$?"
git version 2.54.0 (Apple Git-157)
git exit=0

$ bash --version; echo "bash exit=$?"
GNU bash, version 3.2.57(1)-release (arm64-apple-darwin25)
Copyright (C) 2007 Free Software Foundation, Inc.
bash exit=0

$ command -v gitleaks; echo "command -v exit=$?"; gitleaks version; echo "gitleaks version exit=$?"
/opt/homebrew/bin/gitleaks
command -v exit=0
8.30.1
gitleaks version exit=0
```

The private denylist, asked about by readability only and never printed
(`JIGC_DENYLIST_FILE` first, else the default location the script's help names):

```
JIGC_DENYLIST_FILE is unset
default denylist readable exit=0
```

`dev/stabilize-record --help` exited 0 and printed its usage;
`dev/stabilize-record state --run canary-one` exited 0 and printed one JSON document
with `"run": "canary-one", "opened": true`.

Not asserted, because no listed step needs them: the gate's tools, a container
runtime, `gh`.

### 4. The scratch root exists and is writable

```
$ test -d <scratch root>; echo "is-dir exit=$?"; test -w <scratch root>; echo "writable exit=$?"
is-dir exit=0
writable exit=0
```

The scratch root is the one the prompt names. This call minted no scratch directory of
its own; the one file it wrote is this report, at the path the prompt's `REPORT:` line
names.

## What this call did not do

- No binary, no gate, no scripted check: the harness holds those through
  `dev/stabilize-step`.
- No trial image: not listed.
- No CI read, no packaged-tarball install: not listed.
- No tracked file edited, nothing staged, nothing committed, no branch switched,
  nothing installed, no configuration changed.

## Tree state at return

- Branch: `fix/canary-one`
- `HEAD`: `126a85311a54e73e9f9798f3d034d24f14e3ea7c`
- Status before this report was handed over: the one untracked file shown in assert 2.

<!-- end of report -->
