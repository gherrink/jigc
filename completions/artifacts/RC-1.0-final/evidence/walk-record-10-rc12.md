# Operator walk — recorded per arm

One block per arm. An arm that did not run says so, with the command that
would run it: a narrative walk loses a chartered probe silently, a table
cannot.

| arm | ran | exit | evidence |
|---|---|---|---|
| `00-positive-control.sh` | **NOT RUN** | — | — |
| `01-destination-occupancy.sh` | **NOT RUN** | — | — |
| `02-destroying-doors.sh` | **NOT RUN** | — | — |
| `03-upgrade-author-on-rc11.sh` | **NOT RUN** | — | — |
| `04-foreign-arm.sh` | **NOT RUN** | — | — |
| `05-milestone-boundary.sh` | **NOT RUN** | — | — |
| `06-pre-guard-repair.sh` | **NOT RUN** | — | — |
| `07-changelog-gate.sh` | **NOT RUN** | — | — |
| `08-identity-refusals.sh` | **NOT RUN** | — | — |
| `09-increment-8-doors.sh` | **NOT RUN** | — | — |
| `10-upgrade-continue-on-rc12.sh` | this pass | 0 | `10-upgrade-continue-on-rc12/` |

## `00-positive-control.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 00
```

## `01-destination-occupancy.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 01
```

## `02-destroying-doors.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 02
```

## `03-upgrade-author-on-rc11.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 03
```

## `04-foreign-arm.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 04
```

## `05-milestone-boundary.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 05
```

## `06-pre-guard-repair.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 06
```

## `07-changelog-gate.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 07
```

## `08-identity-refusals.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 08
```

## `09-increment-8-doors.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 09
```

## `10-upgrade-continue-on-rc12.sh`

exit **0**

```
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk03b
out        : /Users/maurice/out/walk03b/10-upgrade-continue-on-rc12
container  : 0e3e7d3d3afffc1baa062c56dabae62c53c27e5f07f3d4e2362ffa44b1332e35
  (a mid-stream plant runs with: docker exec -it -u node 0e3e7d3d3afffc1baa062c56dabae62c53c27e5f07f3d4e2362ffa44b1332e35 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/10-upgrade-continue-on-rc12.sh in the container
-------------------------------------------------------------

=== 0 · we are the NEW binary, over a corpus the OLD one authored

$ jigc --version
jigc 1.0.0-rc.12
[exit 0]

$ cat /work/.upgrade-baseline
rc11-task-validate-clean=1
rc11-validate-exit=0
[exit 0]
  OK    this half really is running rc.12
  OK    the corpus really was authored earlier
  OK    the tree arrived clean, with no rig evidence carried in
  OK    …and specifically no PROVENANCE.txt from the first half

=== 1 · the version mismatch is announced, not silent

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc
advisory · store-version.binary-mismatch — store last written by jigc 1.0.0-rc.11; you are running 1.0.0-rc.12 — align versions or re-run `jigc setup`
  route: align the running jigc to 1.0.0-rc.11, or re-run `jigc setup` to re-stamp the store at 1.0.0-rc.12
a never-adopted file sits at a managed home — jigc was never handed it, so the sweep exits non-zero rather than report a green over a document it has never seen; run `jigc ingest` to route it (each finding above carries its own route), then re-validate.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    the store says it was last written by the older binary
  OK    …and names both versions

=== 2 · BOTH declared changes flip, measured against the first half's baseline
  OK    §0.1 · the squatter validated 0 on rc.11 …
  OK    §0.1 · … and is non-zero on rc.12
  OK    §0.1 · with the code that flipped it

$ jigc task validate add-a-second-rate-limiter
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it in this task — `jigc doc create changelog --title Changelog --task add-a-second-rate-limiter`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task add-a-second-rate-limiter`; if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    §0.2 · the same shape said 'no findings' on rc.11 …
  OK    §0.2 · … and draws the changelog advisory on rc.12
  OK    §0.2 · and it is still exit 0 — an advisory, not a new gate

=== 3 · the corpus still WORKS on the new binary — reads, writes, and a commit boundary

$ jigc doc show adr:cap-distinct-series
---
status: proposed
date: 2026-08-28
schema-version: 2
---

# Cap distinct series at a ceiling

## Context

Authored on rc.11, before the upgrade.

## Options



## Decision

Authored on rc.11, before the upgrade.

## Consequences

Authored on rc.11, before the upgrade.

[exit 0]
  OK    a doc the OLD binary wrote reads back through the new one

$ jigc doc list
id  path  state
adr:cap-distinct-series  docs/decisions/cap-distinct-series.md  managed
adr:drop-the-oldest-sample  docs/decisions/drop-the-oldest-sample.md  managed
changelog:changelog  CHANGELOG.md  unregistered
milestone-record:bound-the-store  docs/milestone-records/bound-the-store.md  managed
research:how-other-buffers-shed-load  docs/research/how-other-buffers-shed-load.md  managed
note: docs are also staged in open task add-a-second-rate-limiter — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list --task add-a-second-rate-limiter`
[exit 0]
  OK    …and appears in the index read as managed

$ jigc task finalize add-a-second-rate-limiter
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it — `jigc start --workflow record-change "<what changed>"`; if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized 9a32ac6 — feat(ingest): add a second rate limiter
  modified .jigc/version
  modified src/config.ts
  2 files committed
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
[exit 0]
  OK    a task minted on the new binary lands over the old corpus
  OK    …and the code change is in that commit
  OK    …and the §0.2 advisory did NOT gate the boundary

=== 4 · setup installs the guide artifact onto a corpus that predates it

$ jigc setup
jigc setup — adapter installed

jigc is now wired into this project; setup installed:
  - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
  - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
  - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
  - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
    local to this checkout — git cannot track this path, so the hook is not in the install commit; a clone gets no drift backstop until `jigc setup` runs there
  - jigc guides → .claude/skills/jigc/SKILL.md   (jigc's own quickstart + migration notes, stamped with this build)
  - install commit → e5f3ea0   (setup's install files are committed on their own, off your first feature commit)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the guide artifact is installed
  OK    …and is stamped with this build

=== 5 · migrate-corpus over the older corpus

$ jigc migrate-corpus --dry-run
corpus migration (dry run — nothing written): 0 would migrate, 4 already current, 0 blocked, 1 not adopted
  current    docs/decisions/cap-distinct-series.md
  current    docs/decisions/drop-the-oldest-sample.md
  current    docs/milestone-records/bound-the-store.md
  current    docs/research/how-other-buffers-shed-load.md
  unadopted  CHANGELOG.md
    schema-conformance.unadopted-instance: committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version
    route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    migrate-corpus reports rather than refusing over the upgraded corpus

=== SUMMARY
  the real 1.0.0 upgrade path: a corpus authored on rc.11, read, written,
  committed and migrated on rc.12, with both declared changes asserted
  against the first half's MEASURED baseline rather than against memory.
ARM 10 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/walk03b/10-upgrade-continue-on-rc12 (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 69
  doc show --task    : 0
  doc show (any)     : 2
  §3.3 adjacent      : 5   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/walk03b/10-upgrade-continue-on-rc12"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 69 records · `doc show … --task` **0** · §3.3 adjacent 5

