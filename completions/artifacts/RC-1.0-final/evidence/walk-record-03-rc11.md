# Operator walk — recorded per arm

One block per arm. An arm that did not run says so, with the command that
would run it: a narrative walk loses a chartered probe silently, a table
cannot.

| arm | ran | exit | evidence |
|---|---|---|---|
| `00-positive-control.sh` | **NOT RUN** | — | — |
| `01-destination-occupancy.sh` | **NOT RUN** | — | — |
| `02-destroying-doors.sh` | **NOT RUN** | — | — |
| `03-upgrade-author-on-rc11.sh` | this pass | 0 | `03-upgrade-author-on-rc11/` |
| `04-foreign-arm.sh` | **NOT RUN** | — | — |
| `05-milestone-boundary.sh` | **NOT RUN** | — | — |
| `06-pre-guard-repair.sh` | **NOT RUN** | — | — |
| `07-changelog-gate.sh` | **NOT RUN** | — | — |
| `08-identity-refusals.sh` | **NOT RUN** | — | — |
| `09-increment-8-doors.sh` | **NOT RUN** | — | — |

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

exit **0**

```
image      : jigc-gate:rc11 (jigc 1.0.0-rc.11)
jigc sha   : 9a37f0152744f0cba5f9140483e1ca1b1c453c46
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk03
out        : /Users/maurice/out/walk03a/03-upgrade-author-on-rc11
container  : 3cec8ead828c391b2ef4177fa7535e5c79322c4b6c2df8ed550be5a792fbcd54
  (a mid-stream plant runs with: docker exec -it -u node 3cec8ead828c391b2ef4177fa7535e5c79322c4b6c2df8ed550be5a792fbcd54 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/03-upgrade-author-on-rc11.sh in the container
-------------------------------------------------------------

=== 0 · adopt with the OLD binary

$ jigc --version
jigc 1.0.0-rc.11
[exit 0]
  OK    this half really is running rc.11

=== 1 · author a mixed corpus THROUGH the old binary

$ git log --oneline
a130682 chore(milestone): record task:cap-distinct-series on milestone:bound-the-store
9beecb4 chore(milestone): open record for milestone:bound-the-store
b6b7bfc docs: record the buffer research
46f4b1e docs: record Cap distinct series at a ceiling
d1571e8 docs: record Drop the oldest sample on overflow
c7e9c06 chore(jigc): install jigc workspace config
c3c48b2 feat: router and service wiring
da62240 feat: aligned window rollups and summaries
8085ba1 feat: in-memory per-series sample store
d51919b feat: bounded ingest queue and wire-line parser
387b5b4 feat: injectable clock and env-read config
3753841 feat: sample validation with a named offending field
791f0dd chore: project skeleton
[exit 0]
  OK    the corpus carries adrs
  OK    …a research doc
  OK    …and a milestone record
  OK    the working tree is clean

=== 2 · the PRE-CHANGE baseline for the two declared behaviour changes

$ jigc task validate add-a-rate-limiter
no findings — the task validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    §0.2 baseline: on rc.11 a conformant task validates CLEAN

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc
1 finding(s) — report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    §0.1 baseline: on rc.11 a foreign squatter validates at EXIT 0

$ cat /work/.upgrade-baseline
rc11-task-validate-clean=1
rc11-validate-exit=0
[exit 0]

=== SUMMARY
  a mixed corpus authored on rc.11, with both declared changes measured BEFORE.
ARM 03a PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/walk03a/03-upgrade-author-on-rc11 (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 47
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 2   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/walk03a/03-upgrade-author-on-rc11"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 47 records · `doc show … --task` **0** · §3.3 adjacent 2

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

