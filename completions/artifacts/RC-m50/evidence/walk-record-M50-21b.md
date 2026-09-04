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
| `14-migrate-author-on-rc12.sh` | **NOT RUN** | — | — |
| `15-placement-root.sh` | **NOT RUN** | — | — |
| `16-pinned-contracts.sh` | **NOT RUN** | — | — |
| `17-empty-id-axis.sh` | **NOT RUN** | — | — |
| `18-surface-batch.sh` | **NOT RUN** | — | — |
| `19-planning-record.sh` | **NOT RUN** | — | — |
| `20-project-pack-composition.sh` | **NOT RUN** | — | — |
| `21-migrate-continue-on-rc13.sh` | this pass | 1 | `21-migrate-continue-on-rc13/` |

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

exit **1**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50-mig-c
out        : /Users/maurice/out/M50-21b/21-migrate-continue-on-rc13
container  : 535a0ab19fbf53f5e3592a4bd42c14b54876f04eb44ad18303eace33d9e1c4cb
  (a mid-stream plant runs with: docker exec -it -u node 535a0ab19fbf53f5e3592a4bd42c14b54876f04eb44ad18303eace33d9e1c4cb bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/21-migrate-continue-on-rc13.sh in the container
-------------------------------------------------------------

=== 0 · we are the NEW binary, over a corpus the OLD one authored

$ jigc --version
jigc 1.0.0-rc.13
[exit 0]

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
  OK    this half really is running rc.13
  OK    the first half really ran rc.12
  OK    the corpus arrived at the first half's HEAD
  OK    …with the milestone-record and completion-record it committed
  OK    the tree arrived clean, with no rig evidence carried in
  OK    …and specifically no PROVENANCE.txt from the first half
  OK    the OLD stamps are the ones the first half measured (file)

=== 1 · (a) validate flips: the two stale stamps and the binary mismatch are announced

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
blocking · schema-conformance.schema-version-current — `docs/completions/m1.md`: field `schema-version` is schema-version 1, below the current schema-version 2
  at: completion-record:m1
  route: migrate — `docs/completions/m1.md` is a managed doc below the current schema-version 2; run `jigc migrate-corpus` to upgrade it
blocking · schema-conformance.schema-version-current — `docs/milestone-records/bound-the-store.md`: field `schema-version` is schema-version 2, below the current schema-version 3
  at: milestone-record:bound-the-store
  route: migrate — `docs/milestone-records/bound-the-store.md` is a managed doc below the current schema-version 3; run `jigc migrate-corpus` to upgrade it
advisory · store-version.binary-mismatch — store last written by jigc 1.0.0-rc.12; you are running 1.0.0-rc.13 — and this store's committed docs are stale against 1.0.0-rc.13's schemas: re-stamping alone would clear this advisory while the corpus stayed stale
  route: run `jigc migrate-corpus` to upgrade the committed docs, then re-run `jigc setup` to re-stamp the store at 1.0.0-rc.13 (or align the running jigc back to 1.0.0-rc.12)
the committed corpus is below its schema-version — every other finding above was adjudicated against a schema those docs were never written to, so the sweep exits non-zero; run `jigc migrate-corpus`, then re-validate.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    rc.12 validated this corpus at exit 0 (baseline) …
  OK    … and rc.13 exits NON-ZERO over the same bytes
  OK    the code that flipped it is schema-conformance.schema-version-current
  OK    …on the milestone-record
  OK    …on the completion-record
  OK    …each routed at jigc migrate-corpus
  OK    the store says it was last written by the older binary (store-version.binary-mismatch)
  OK    …and names both versions
  OK    no adr / research / changelog is flagged — the frozen dev set did not move

=== 1b · the WRONG order, on a throwaway copy: setup FIRST, then validate — recorded, not asserted

$ jigc setup
jigc setup — adapter installed

jigc is now wired into this project; setup installed:
  - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
  - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
  - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
  - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
    local to this checkout — git cannot track this path, so the hook is not in the install commit; a clone gets no drift backstop until `jigc setup` runs there
  - jigc guides → .claude/skills/jigc/SKILL.md   (jigc's own quickstart + migration notes, stamped with this build)
  - install commit → 25f72e8   (setup's install files are committed on their own, off your first feature commit)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc validate
blocking · schema-conformance.schema-version-current — `docs/completions/m1.md`: field `schema-version` is schema-version 1, below the current schema-version 2
  at: completion-record:m1
  route: migrate — `docs/completions/m1.md` is a managed doc below the current schema-version 2; run `jigc migrate-corpus` to upgrade it
blocking · schema-conformance.schema-version-current — `docs/milestone-records/bound-the-store.md`: field `schema-version` is schema-version 2, below the current schema-version 3
  at: milestone-record:bound-the-store
  route: migrate — `docs/milestone-records/bound-the-store.md` is a managed doc below the current schema-version 3; run `jigc migrate-corpus` to upgrade it
the committed corpus is below its schema-version — every other finding above was adjudicated against a schema those docs were never written to, so the sweep exits non-zero; run `jigc migrate-corpus`, then re-validate.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
setup-first copy: validate exits 1 afterwards
  OK    setup-first does not paper over the stale stamps (validate still non-zero on the copy)

=== 2 · (b) migrate-corpus: lists both, lands STAMP-ONLY, and validate returns to 0

$ jigc migrate-corpus --dry-run
corpus migration (dry run — nothing written): 2 would migrate, 4 already current, 0 blocked, 1 left unfilled
  would migrate docs/completions/m1.md
  would migrate docs/milestone-records/bound-the-store.md
  current    CHANGELOG.md
  current    docs/decisions/cap-distinct-series.md
  current    docs/decisions/drop-the-oldest-sample.md
  current    docs/research/how-other-buffers-shed-load.md
  unfilled   docs/milestone-records/bound-the-store.md#tasks/workflow
    migrate-corpus.set-field-unfilled: `docs/milestone-records/bound-the-store.md` migrated with `tasks`'s **item** field `workflow` left unfilled — the field declares `set: on-transition` and no `default:`, so the migration placed no `workflow` in any item of `tasks` and invented none; their absence conforms
    route: no action needed — `workflow` is machine-maintained (`set: on-transition`): jigc derives its value and no `jigc doc` write may set it
nothing was written — re-run without `--dry-run` to apply the migration
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the dry run names the milestone-record
  OK    the dry run names the completion-record
  OK    …and nothing from the frozen dev set is a would-migrate line
  OK    the v3 leaf the migration cannot invent (`workflow`, set: on-transition) is NAMED as left unfilled …
  OK    … with a route that says no action is needed
  OK    a dry run moves nothing

$ jigc migrate-corpus
corpus migration: 2 migrated, 4 already current, 0 blocked, 1 left unfilled
  migrated   docs/completions/m1.md
  migrated   docs/milestone-records/bound-the-store.md
  current    CHANGELOG.md
  current    docs/decisions/cap-distinct-series.md
  current    docs/decisions/drop-the-oldest-sample.md
  current    docs/research/how-other-buffers-shed-load.md
  unfilled   docs/milestone-records/bound-the-store.md#tasks/workflow
    migrate-corpus.set-field-unfilled: `docs/milestone-records/bound-the-store.md` migrated with `tasks`'s **item** field `workflow` left unfilled — the field declares `set: on-transition` and no `default:`, so the migration placed no `workflow` in any item of `tasks` and invented none; their absence conforms
    route: no action needed — `workflow` is machine-maintained (`set: on-transition`): jigc derives its value and no `jigc doc` write may set it
committed d0e6025 — only the migrated paths were staged
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
d0e6025 chore(jigc): migrate the managed corpus to the current schema versions
c7e811d Finalize milestone bound-the-store (1 sub-task)
c50e030 : cap distinct series
?? .upgrade-baseline
migrate-corpus committed on its own: 1
 docs/completions/m1.md                    | 2 +-
 docs/milestone-records/bound-the-store.md | 2 +-
 2 files changed, 2 insertions(+), 2 deletions(-)
diff --git a/docs/completions/m1.md b/docs/completions/m1.md
index 405dc43..d668118 100644
--- a/docs/completions/m1.md
+++ b/docs/completions/m1.md
@@ -4 +4 @@ owner-artifact: completions/artifacts/M1/VERDICT.md
-schema-version: 1
+schema-version: 2
diff --git a/docs/milestone-records/bound-the-store.md b/docs/milestone-records/bound-the-store.md
index 485ae07..7fba8b7 100644
--- a/docs/milestone-records/bound-the-store.md
+++ b/docs/milestone-records/bound-the-store.md
@@ -4 +4 @@ status: joined
-schema-version: 2
+schema-version: 3
  OK    migrate-corpus exits 0
  OK    exactly the two stale docs changed
  OK    the migration is STAMP-ONLY: every changed line is a schema-version line
  OK    the milestone-record file is stamped 3 (was 2)
  OK    the completion-record file is stamped 2 (was 1)
  OK    doc show --format json reports 3 on the milestone-record
  OK    doc show --format json reports 2 on the completion-record
  OK    the record's authored content survived (status joined, severity blocking)

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · store-version.binary-mismatch — store last written by jigc 1.0.0-rc.12; you are running 1.0.0-rc.13 — align versions or re-run `jigc setup`
  route: align the running jigc to 1.0.0-rc.12, or re-run `jigc setup` to re-stamp the store at 1.0.0-rc.13
1 finding(s) — report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    validate returns to exit 0 after the migration
  OK    …and schema-version-current is gone

=== 3 · (c) the widened enum: severity HIGH now lands (EnumWidened)

$ jigc doc set-field completion-record:m2#findings/stray-finding/severity --value HIGH --task m2
set completion-record:m2#findings/stray-finding/severity = HIGH
[exit 0]
  OK    rc.12 refused HIGH at exit 1 (baseline) …
  OK    … and rc.13 lands it at exit 0
  OK    the staged read carries HIGH

$ jigc doc set-field completion-record:m2#findings/stray-finding/severity --value CRITICAL --task m2
blocking · schema-conformance.field-value-conformant — write rejected: "CRITICAL" is not a member of enum "severity" (allowed: blocking, advisory, HIGH, MEDIUM, LOW)
  at: completion-record:m2#findings/stray-finding/severity
  route: `jigc doc set-field completion-record:m2#findings/stray-finding/severity --value <value>` to correct the value
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    a non-member is still refused — it is a wider enum, not an open string

$ jigc task finalize m2
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
    completions/artifacts/M2/VERDICT.md
advisory · file-state.staged-copy — staged copy of `docs/completions/m2.md` — this task's in-flight version of the doc
  at: docs/completions/m2.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized 34b8523 — docs: record the M2 completion
  modified .jigc/version
  added completions/artifacts/M2/VERDICT.md
  promoted docs/completions/m2.md
  3 files committed
  left-out (unstaged/untracked — git add to include):
    .upgrade-baseline
[exit 0]
  OK    the HIGH-graded record lands
  OK    …stamped 2 at birth

=== 4 · (d) the 1799a2d tightening: the boundary now gates the sub-task's commit doc

$ jigc config get finalize.fan-out.squash
finalize.fan-out.squash = false  (project)
[exit 0]
  OK    squash=false travelled with the corpus (committed manifest)

$ jigc milestone create bound the store again
minted milestone:bound-the-store-again (shared base 34b8523)
record: docs/milestone-records/bound-the-store-again.md   — the committed record this milestone's state lives in
record commit: c8d009f   — the record on its own; anything else you had staged stayed staged
next: `jigc milestone add-task bound-the-store-again "<intent>"`   — add the milestone's first sub-task
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc milestone add-task bound-the-store-again prune on overflow
added task:prune-on-overflow to milestone:bound-the-store-again
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc milestone provision bound-the-store-again
provisioned 1 worktree(s) for milestone:bound-the-store-again at base 34b8523 (prune-on-overflow)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
launch line from `jigc milestone execute`: cd .jigc/worktrees/prune-on-overflow && jigc workflow sub-task --task prune-on-overflow
[launch line exit 0]

$ jigc doc list --task prune-on-overflow
id  path  state
commit:prune-on-overflow  commit:prune-on-overflow  managed
[exit 0]

$ jigc task validate prune-on-overflow
blocking · schema-conformance.field-value-conformant — `commit:prune-on-overflow`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  at: commit:prune-on-overflow#header/type
  route: `jigc doc set-field commit:prune-on-overflow#header/type --task prune-on-overflow --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:prune-on-overflow`: required slot in section `summary` is empty
  at: commit:prune-on-overflow#summary · line 9
  route: `jigc doc set-slot commit:prune-on-overflow#summary --task prune-on-overflow --from-file -` to fill the empty slot
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 3]

$ jigc milestone finalize bound-the-store-again   (type AND summary unset)
`commit:prune-on-overflow`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  at: commit:prune-on-overflow#header/type
  route: `jigc doc set-field commit:prune-on-overflow#header/type --task prune-on-overflow --value <value>` to correct the value
`commit:prune-on-overflow`: required slot in section `summary` is empty
  at: commit:prune-on-overflow#summary · line 9
  route: `jigc doc set-slot commit:prune-on-overflow#summary --task prune-on-overflow --from-file -` to fill the empty slot
[exit 3]
  OK    rc.12 LANDED this shape (baseline landed=1) …
  OK    … and rc.13 BLOCKS it (exit non-zero; measured 3)
  OK    the block names the transient commit doc
  OK    …and names the unset `type`
  OK    …and the unset `summary`
  OK    …with a route carrying --task prune-on-overflow
  OK    it commits NOTHING
  OK    …and no `: …` subject entered the history
  FAIL  the block lines carry a severity · code prefix, as the task door's do

$ jigc milestone finalize bound-the-store-again --format json   (same state — the machine surface)
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "field-value-conformant",
      "code": "schema-conformance.field-value-conformant",
      "key": {
        "code": "schema-conformance.field-value-conformant",
        "target": "commit:prune-on-overflow#header/type"
      },
      "message": "`commit:prune-on-overflow`: field `type` in section `header`: \"\" is not a member of enum \"type\" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)",
      "location": {
        "address": "commit:prune-on-overflow#header/type",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc set-field commit:prune-on-overflow#header/type --task prune-on-overflow --value <value>` to correct the value"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "required-slot-present",
      "code": "schema-conformance.required-slot-present",
      "key": {
        "code": "schema-conformance.required-slot-present",
        "target": "commit:prune-on-overflow#summary"
      },
      "message": "`commit:prune-on-overflow`: required slot in section `summary` is empty",
      "location": {
        "address": "commit:prune-on-overflow#summary",
        "line": 9,
        "col": 1
      },
      "route": "`jigc doc set-slot commit:prune-on-overflow#summary --task prune-on-overflow --from-file -` to fill the empty slot"
    }
  ]
}
[exit 3]
  OK    the JSON envelope is valid JSON and still blocks
  OK    …and its findings carry a code (the key the text surface dropped)
  OK    …and still nothing committed after the JSON drive

=== 4 · the route for type, run VERBATIM from inside the worktree
route as printed (with <value> → feat): jigc doc set-field commit:prune-on-overflow#header/type --task prune-on-overflow --value feat
set commit:prune-on-overflow#header/type = feat
[route exit 0]
  OK    the printed route runs as printed

$ jigc milestone finalize bound-the-store-again   (type set, summary still unset)
`commit:prune-on-overflow`: required slot in section `summary` is empty
  at: commit:prune-on-overflow#summary · line 9
  route: `jigc doc set-slot commit:prune-on-overflow#summary --task prune-on-overflow --from-file -` to fill the empty slot
[exit 3]
  OK    still blocked on the second leaf
  OK    …which is summary, not type
  OK    …and still nothing committed
set slot commit:prune-on-overflow#summary (18 chars)
[set-slot exit 0]

$ jigc milestone finalize bound-the-store-again   (both leaves filled)
finalized cf47674 — Finalize milestone bound-the-store-again (1 sub-task)
  modified docs/milestone-records/bound-the-store-again.md
  added src/prune.ts
  2 files committed
  sub-tasks: prune-on-overflow: 1 doc, 1 code file
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
cf47674 Finalize milestone bound-the-store-again (1 sub-task)
50c69b6 feat: prune on overflow
a2cfd22 chore(milestone): record task:prune-on-overflow on milestone:bound-the-store-again
  OK    the boundary lands once both leaves are filled
  OK    one conventional commit per sub-task, with the authored subject
  OK    …carrying the sub-task's code
  OK    the new milestone-record is stamped 3 at birth
  OK    …and records the sub-task's workflow (the v3 leaf)

=== 5 · (e) setup AFTER the migration installs the guide stamped with this build

$ jigc setup
jigc setup — adapter installed

jigc is now wired into this project; setup installed:
  - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
  - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
  - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
  - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
    local to this checkout — git cannot track this path, so the hook is not in the install commit; a clone gets no drift backstop until `jigc setup` runs there
  - jigc guides → .claude/skills/jigc/SKILL.md   (jigc's own quickstart + migration notes, stamped with this build)
  - install commit → 456e425   (setup's install files are committed on their own, off your first feature commit)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the guide artifact is installed
  OK    …and is stamped with this build
  OK    …and the version stamp now names rc.13

$ jigc validate
no findings — the committed store validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    validate is exit 0 after setup
  OK    …and the binary mismatch is gone

=== 6 · (f) what the OLD binary wrote reads back through the new one

$ jigc doc show adr:cap-distinct-series
---
status: proposed
date: 2026-09-04
schema-version: 2
---

# Cap distinct series at a ceiling

## Context

Authored on rc.12, before the migration.

## Options



## Decision

Authored on rc.12, before the migration.

## Consequences

Authored on rc.12, before the migration.

[exit 0]
  OK    an adr the OLD binary wrote reads back
  OK    …and is managed in the index read
  OK    the migrated milestone-record is managed
  OK    the migrated completion-record is managed
  OK    the old changelog's nested change-group reads back

$ jigc doc show completion-record:m1 --format json
{
  "fields": {
    "owner-artifact": "completions/artifacts/M1/VERDICT.md",
    "schema-version": "2",
    "verdict": "green"
  },
  "item-count": 1,
  "schema-version": 2,
  "sections": {
    "findings": [
      {
        "detail": "",
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
  OK    the migrated completion-record's finding is intact (severity blocking, disposition fixed)
  OK    the working tree ends clean (baseline aside)

=== SUMMARY
  the migration path: a corpus rc.12 wrote — two stale stamps, a HIGH refusal and
  a type-less sub-task commit the old boundary landed — validated, migrated
  stamp-only, continued and re-gated on rc.13, every flip asserted against the
  first half's MEASURED baseline rather than against memory.
ARM 21 FAIL
(the arm exited 1 — that is data, not necessarily failure)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-21b/21-migrate-continue-on-rc13 (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 142
  doc show --task    : 1
  doc show (any)     : 12
  §3.3 adjacent      : 5   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-21b/21-migrate-continue-on-rc13"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 142 records · `doc show … --task` **1** · §3.3 adjacent 5

