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
| `10-upgrade-continue-on-rc12.sh` | **NOT RUN** | — | — |
| `11-item-region-shipped-doctypes.sh` | **NOT RUN** | — | — |
| `12-doors-that-lie.sh` | **NOT RUN** | — | — |
| `13-freeze-every-layer.sh` | **NOT RUN** | — | — |
| `14-migrate-author-on-rc12.sh` | this pass | 0 | `14-migrate-author-on-rc12/` |
| `15-placement-root.sh` | **NOT RUN** | — | — |
| `16-pinned-contracts.sh` | **NOT RUN** | — | — |
| `17-empty-id-axis.sh` | **NOT RUN** | — | — |
| `18-surface-batch.sh` | **NOT RUN** | — | — |
| `19-planning-record.sh` | **NOT RUN** | — | — |
| `20-project-pack-composition.sh` | **NOT RUN** | — | — |
| `21-migrate-continue-on-rc13.sh` | **NOT RUN** | — | — |

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

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 10
```

## `11-item-region-shipped-doctypes.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 11
```

## `12-doors-that-lie.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 12
```

## `13-freeze-every-layer.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 13
```

## `14-migrate-author-on-rc12.sh`

exit **0**

```
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50-mig
out        : /Users/maurice/out/M50-14/14-migrate-author-on-rc12
container  : 64fe3c95ab1043cc9bc14ac6dc0507b1c762b454c1935d2a9607d7b75a4136d7
  (a mid-stream plant runs with: docker exec -it -u node 64fe3c95ab1043cc9bc14ac6dc0507b1c762b454c1935d2a9607d7b75a4136d7 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/14-migrate-author-on-rc12.sh in the container
-------------------------------------------------------------

=== 0 · adopt with the OLD binary

$ jigc --version
jigc 1.0.0-rc.12
[exit 0]
  OK    this half really is running rc.12

=== 1 · the ordinary corpus, authored THROUGH the old binary

$ jigc task finalize record-the-first-release
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` — this task's in-flight version of the doc
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized 6544346 — docs: record the first release
  promoted CHANGELOG.md
  1 file committed
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
[exit 0]

$ git log --oneline
6544346 docs: record the first release
088e8c4 docs: record the buffer research
ee3c371 docs: record Cap distinct series at a ceiling
c5ea506 docs: record Drop the oldest sample on overflow
2597cd8 chore(jigc): install jigc workspace config
23e67d1 feat: router and service wiring
71da317 feat: aligned window rollups and summaries
98ceaed feat: in-memory per-series sample store
64cf5a2 feat: bounded ingest queue and wire-line parser
a55143f feat: injectable clock and env-read config
ffa5bd4 feat: sample validation with a named offending field
5e677f6 chore: project skeleton
[exit 0]
  OK    the corpus carries adrs
  OK    …a research doc
  OK    …and a changelog at its placement home
  OK    the changelog carries the release AND the nested change-group

=== 2 · SUBJECT (2) · a completion-record graded HIGH — what rc.12 says
completion-record: completion-record:m1 (task m1)

$ jigc doc set-field completion-record:m1#findings/stray-finding/severity --value HIGH --task m1
blocking · schema-conformance.field-value-conformant — write rejected: "HIGH" is not a member of enum "severity" (allowed: blocking, advisory)
  route: `jigc doc set-field completion-record:m1#findings/stray-finding/severity --value <value>` to correct the value
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    rc.12 REFUSES severity HIGH (exit non-zero)
  OK    …at schema-conformance.field-value-conformant
  OK    …and names the enum it holds

$ jigc task finalize m1
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
    completions/artifacts/M1/VERDICT.md
advisory · file-state.staged-copy — staged copy of `docs/completions/m1.md` — this task's in-flight version of the doc
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized bb77ed6 — docs: record the M1 completion
  added completions/artifacts/M1/VERDICT.md
  promoted docs/completions/m1.md
  2 files committed
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
[exit 0]
  OK    the completion-record lands at its home with severity blocking

$ jigc doc show completion-record:m1 --format json
{
  "fields": {
    "owner-artifact": "completions/artifacts/M1/VERDICT.md",
    "schema-version": "1",
    "verdict": "green"
  },
  "item-count": 1,
  "sections": {
    "findings": [
      {
        "disposition": "fixed",
        "evidence": "audit.log:12",
        "id": "stray-finding",
        "severity": "blocking",
        "title": "Stray finding"
      }
    ]
  },
  "slug": "m1",
  "type": "completion-record"
}
[exit 0]
completion-record schema-version as rc.12 stamps it (file): 1
  OK    the completion-record is stamped schema-version 1 on rc.12

=== 3 · SUBJECT (1) · the milestone boundary over a type-less sub-task commit doc

$ jigc milestone create bound the store
minted milestone:bound-the-store (shared base bb77ed6)
record: docs/milestone-records/bound-the-store.md   — the committed record this milestone's state lives in
record commit: 431dc24   — the record on its own; anything else you had staged stayed staged
next: `jigc milestone add-task bound-the-store "<intent>"`   — add the milestone's first sub-task
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc milestone add-task bound-the-store cap distinct series
added task:cap-distinct-series to milestone:bound-the-store
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc milestone provision bound-the-store
provisioned 1 worktree(s) for milestone:bound-the-store at base bb77ed6 (cap-distinct-series)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
launch line from `jigc milestone execute`: cd .jigc/worktrees/cap-distinct-series && jigc workflow sub-task --task cap-distinct-series
[launch line exit 0]

$ jigc doc list --task cap-distinct-series
id  path  state
commit:cap-distinct-series  commit:cap-distinct-series  managed
[exit 0]
set slot commit:cap-distinct-series#summary (20 chars)
[set-slot exit 0]

$ jigc task validate cap-distinct-series
blocking · schema-conformance.field-value-conformant — `commit:cap-distinct-series`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  route: `jigc doc set-field commit:cap-distinct-series#header/type --value <value>` to correct the value
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 3]
  OK    the TASK door sees the unset type (task validate exits non-zero)

$ jigc milestone finalize bound-the-store
finalized c7e811d — Finalize milestone bound-the-store (1 sub-task)
  modified .jigc/config/manifest.yaml
  modified docs/milestone-records/bound-the-store.md
  added src/cap.ts
  3 files committed
  sub-tasks: cap-distinct-series: 1 doc, 1 code file
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
c7e811d Finalize milestone bound-the-store (1 sub-task)
c50e030 : cap distinct series
05c6c0b chore(milestone): record task:cap-distinct-series on milestone:bound-the-store
  OK    the 1799a2d baseline: rc.12 LANDS the milestone over the type-less commit doc
  OK    …with a type-less subject (`: …`) in the history
  OK    …and the sub-task's code is in it

$ jigc doc show milestone-record:bound-the-store --format json
{
  "fields": {
    "base": {
      "sha": "bb77ed6fa143baee20508122880e131259f78c2e",
      "short": "bb77ed6"
    },
    "schema-version": "2",
    "status": "joined"
  },
  "item-count": 1,
  "sections": {
    "tasks": [
      {
        "id": "cap-distinct-series",
        "intent": "cap distinct series",
        "status": "joined",
        "task-id": "cap-distinct-series"
      }
    ]
  },
  "slug": "bound-the-store",
  "type": "milestone-record"
}
[exit 0]
milestone-record schema-version as rc.12 stamps it (file): 2 (status joined)
  OK    SUBJECT (3) · the milestone-record is committed
  OK    …stamped schema-version 2 on rc.12

=== 4 · the store as rc.12 leaves it

$ jigc validate
no findings — the committed store validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    rc.12 validates its own corpus at exit 0
  OK    the working tree is clean (the baseline is the only untracked file)
  OK    no task is left open

$ cat /work/.upgrade-baseline
rc12-version=1.0.0-rc.12
rc12-adr-count=2
rc12-adr-first=cap-distinct-series
rc12-severity-high-exit=1
rc12-severity-high-refused-at-field-value-conformant=1
rc12-completion-record=completion-record:m1
rc12-completion-record-path=docs/completions/m1.md
rc12-completion-record-schema-version=1
rc12-spawn-line=cd .jigc/worktrees/cap-distinct-series && jigc workflow sub-task --task cap-distinct-series
rc12-subtask-validate-typeless-exit=3
rc12-milestone-finalize-typeless-landed=1
rc12-milestone-finalize-typeless-exit=0
rc12-milestone-finalize-typeless-subject=: cap distinct series
rc12-milestone=bound-the-store
rc12-subtask=cap-distinct-series
rc12-milestone-record-path=docs/milestone-records/bound-the-store.md
rc12-milestone-record-schema-version=2
rc12-milestone-record-status=joined
rc12-validate-exit=0
rc12-head=c7e811dbc67467e51b8626d38a90f67a54904867
rc12-commit-count=17
[exit 0]
  OK    the baseline is present

=== SUMMARY
  a corpus authored on rc.12 — two adrs, a research doc, a nested changelog, a
  completion-record at v1 (HIGH refused, blocking landed) and a milestone whose
  type-less sub-task commit doc the old boundary landed — every number in
  .upgrade-baseline for arm 21 to assert the flips against.
ARM 14 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-14/14-migrate-author-on-rc12 (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 80
  doc show --task    : 0
  doc show (any)     : 4
  §3.3 adjacent      : 3   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-14/14-migrate-author-on-rc12"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 80 records · `doc show … --task` **0** · §3.3 adjacent 3

## `15-placement-root.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 15
```

## `16-pinned-contracts.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 16
```

## `17-empty-id-axis.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 17
```

## `18-surface-batch.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 18
```

## `19-planning-record.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 19
```

## `20-project-pack-composition.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 20
```

## `21-migrate-continue-on-rc13.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 21
```

