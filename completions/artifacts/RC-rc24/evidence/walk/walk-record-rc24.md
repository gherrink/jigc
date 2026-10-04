# Operator walk — recorded per arm

One block per arm. An arm that did not run says so, with the command that
would run it: a narrative walk loses a chartered probe silently, a table
cannot.

| arm | ran | exit | evidence |
|---|---|---|---|
| `00-positive-control.sh` | this pass | 0 | `00-positive-control/` |
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
| `14-migrate-author-on-rc12.sh` | **NOT RUN** | — | — |
| `15-placement-root.sh` | **NOT RUN** | — | — |
| `16-pinned-contracts.sh` | **NOT RUN** | — | — |
| `17-empty-id-axis.sh` | **NOT RUN** | — | — |
| `18-surface-batch.sh` | **NOT RUN** | — | — |
| `19-planning-record.sh` | **NOT RUN** | — | — |
| `20-project-pack-composition.sh` | **NOT RUN** | — | — |
| `21-migrate-continue-on-rc13.sh` | **NOT RUN** | — | — |
| `22-m50-orientation-and-carried.sh` | **NOT RUN** | — | — |
| `23-m50-audit-findings.sh` | **NOT RUN** | — | — |

## `00-positive-control.sh`

exit **0**

```
image      : jigc-gate:registry-1.0.0-rc.24 (jigc 1.0.0-rc.24)
jigc sha   : unknown
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : ~/ideas/walk-rc24
out        : ~/out/RC24-walk/00-positive-control
container  : d8e20fd5bc06cd558ed69eb59214eadfe2f67da6d73875a6dd34f1f2dad16c83
  (a mid-stream plant runs with: docker exec -it -u node d8e20fd5bc06cd558ed69eb59214eadfe2f67da6d73875a6dd34f1f2dad16c83 bash -l)

running ~/projects/gherrink-jigc/completions/trial-driver/arms/walk/00-positive-control.sh in the container
-------------------------------------------------------------

=== 0 · adopt the corpus with the binary under test
jigc 1.0.0-rc.24

=== 1 · mint the task
task: record-that-the-ingest-queue

=== 2 · author a managed doc through the write verbs
created: adr:drop-the-oldest-sample-when
slots authored

=== 3 · THE CHANNEL — read the staged copy back
---
status: proposed
date: 2026-10-03
schema-version: 2
---

# Drop the oldest sample when the ingest queue overflows

## Context

Prose for context, authored by the control arm.


=== 4 · the pass condition, checked in-container
doc show --task records: 1

=== 5 · the VERB-ADJACENT channel also records (protocol §3.3)
adjacent records: 2

=== 6 · leave the task open
jigc task list — 1 active task(s)

  record-that-the-ingest-queue  [record-decision]  record that the ingest queue drops the oldest sample when it overflows
— jigc · run `jigc start` for orientation; all writes through `jigc`.

ARM 0 PASS — the channel fires and is countable
-------------------------------------------------------------
evidence in ~/out/RC24-walk/00-positive-control (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 10
  doc show --task    : 1
  doc show (any)     : 1
  §3.3 adjacent      : 2   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "~/out/RC24-walk/00-positive-control"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 10 records · `doc show … --task` **1** · §3.3 adjacent 2

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

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 14
```

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

## `22-m50-orientation-and-carried.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 22
```

## `23-m50-audit-findings.sh`

**NOT RUN.** To run it:

```sh
python3 completions/trial-driver/walk.py <corpus> <out> --only 23
```

