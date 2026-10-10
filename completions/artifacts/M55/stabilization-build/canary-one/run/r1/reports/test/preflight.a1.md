# Preflight — run `canary-one`, round 1, stage `test`, attempt 1

Status: **ready**. Every environment assert holds. This call covered one step, the
environment asserts, and nothing else: no binary was built, no gate was run, no image
was built, and no deterministic check was run or read — none was asked for, so none is
reported, green, red or void.

- Candidate: label `c1`, commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`
- Branch checked out: `fix/canary-one`
- Scratch root: `<scratch>` (the root the prompt names; passed to the record script as `--scratch`)

Every git command below was run as `git -C <repository root> …`; the output is copied
as printed.

## 1. The candidate is `HEAD`

```
$ git rev-parse HEAD
eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
```

Equal to the candidate's sha, character for character. Holds.

```
$ git branch --show-current
fix/canary-one
```

## 2. The tree holds what the prompt says to expect, and no more

Expected: untracked files under `completions/artifacts/canary-one/` and nothing else.

```
$ git status --porcelain --untracked-files=all
?? completions/artifacts/canary-one/r1/reports/test/attempt.a1.md
```

One line, untracked, under `completions/artifacts/canary-one/`. No tracked file is
modified, nothing is staged. Holds.

```
$ git status
On branch fix/canary-one
Untracked files:
  (use "git add <file>..." to include in what will be committed)
	completions/artifacts/canary-one/r1/

nothing added to commit but untracked files present (use "git add" to track)
```

## 3. Every tool the listed step needs answers

The one step listed needs git, and the hand-over of this report needs the record
script, whose `--help` names what it needs: python3, git, bash (for
`dev/hygiene-scan`), gitleaks and a denylist. No other tool was looked for: no gate
tool, no container runtime and no `gh`, because no step that needs one is listed.

```
$ git --version
git version 2.54.0 (Apple Git-157)

$ python3 --version
Python 3.9.6

$ bash --version
GNU bash, version 3.2.57(1)-release (arm64-apple-darwin25)
Copyright (C) 2007 Free Software Foundation, Inc.

$ gitleaks version
8.30.1
```

The denylist — its presence only, its content never read:

```
JIGC_DENYLIST_FILE is unset
~/.config/jigc/denylist readable-and-non-empty exit=0
```

The record script, its scanner script, the scanner's configuration and the run's
opening record (each by `test`, the exit status printed):

```
dev/hygiene-scan executable exit=0
dev/stabilize-record executable exit=0
.gitleaks.toml present exit=0
opening.md present exit=0
```

(`opening.md` is `completions/artifacts/canary-one/opening.md`.)

The record script answers:

```
$ dev/stabilize-record --help
```

exit 0, its usage printed (about 87 KB; not copied here).

```
$ dev/stabilize-record state --run canary-one
```

exit 0, one JSON document. The fields of it this step reads, copied from that output:

- `"run": "canary-one"`, `"opened": true`, `"not_ready": []`
- `"previous": "1.0.0-rc.24"`, `"previous-commit": "91834b5e011de2c36e2be2b79e96c0b9f60a803c"`
- `"round": 1`, and for round 1: `"scope": false`, `"triage": false`, `"record": false`, `"test_reports": 1`, `"test_attempt": 1`, `"test_unrecorded": 1`
- `"candidate": {"round": null, "commit": null, "current": false}`
- `"next": "test"`, `"stop": null`
- `"position": {"test": {"round": 1, "attempt": 2}, "fix": {"refused": "not-tested", "round": 1}}`

These are reported as read. This role judges none of them.

Holds: every tool named answered with exit 0.

## 4. The scratch root exists and is writable

```
scratch root is a directory exit=0
scratch root is writable exit=0
<scratch>/preflight.1muuiV
mktemp exit=0
```

The third line is what `mktemp -d <scratch>/preflight.XXXXXX` printed: a directory was
minted under the scratch root. It is empty and is left as it stands. Holds.

## What this call did not do

- No binary: the candidate's and the previous release's are built by `dev/stabilize-step`.
- No gate, no regression set: held checks of the same tool.
- No trial image, no `ci-ok` read, no packaged-tarball install: not listed.
- No tracked file edited, nothing staged, nothing committed, no branch switched, nothing installed.

## Repro

```
git rev-parse HEAD
git branch --show-current
git status --porcelain --untracked-files=all
git --version
python3 --version
bash --version
gitleaks version
dev/stabilize-record --help
dev/stabilize-record state --run canary-one
```

<!-- end of report -->
