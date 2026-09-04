# Operator walk — recorded per arm

One block per arm. An arm that did not run says so, with the command that
would run it: a narrative walk loses a chartered probe silently, a table
cannot.

| arm | ran | exit | evidence |
|---|---|---|---|
| `00-positive-control.sh` | this pass | 0 | `00-positive-control/` |
| `01-destination-occupancy.sh` | this pass | 0 | `01-destination-occupancy/` |
| `02-destroying-doors.sh` | this pass | 1 | `02-destroying-doors/` |
| `03-upgrade-author-on-rc11.sh` | this pass | 0 | `03-upgrade-author-on-rc11/` |
| `04-foreign-arm.sh` | this pass | 0 | `04-foreign-arm/` |
| `05-milestone-boundary.sh` | this pass | 0 | `05-milestone-boundary/` |
| `06-pre-guard-repair.sh` | this pass | 0 | `06-pre-guard-repair/` |
| `07-changelog-gate.sh` | this pass | 0 | `07-changelog-gate/` |
| `08-identity-refusals.sh` | this pass | 0 | `08-identity-refusals/` |
| `09-increment-8-doors.sh` | this pass | 0 | `09-increment-8-doors/` |
| `10-upgrade-continue-on-rc12.sh` | this pass | 0 | `10-upgrade-continue-on-rc12/` |
| `11-item-region-shipped-doctypes.sh` | this pass | 0 | `11-item-region-shipped-doctypes/` |
| `12-doors-that-lie.sh` | this pass | 0 | `12-doors-that-lie/` |
| `13-freeze-every-layer.sh` | this pass | 0 | `13-freeze-every-layer/` |
| `14-migrate-author-on-rc12.sh` | this pass | 0 | `14-migrate-author-on-rc12/` |
| `15-placement-root.sh` | this pass | 1 | `15-placement-root/` |
| `16-pinned-contracts.sh` | this pass | 0 | `16-pinned-contracts/` |
| `17-empty-id-axis.sh` | this pass | 1 | `17-empty-id-axis/` |
| `18-surface-batch.sh` | this pass | 1 | `18-surface-batch/` |
| `19-planning-record.sh` | this pass | 0 | `19-planning-record/` |
| `20-project-pack-composition.sh` | this pass | 1 | `20-project-pack-composition/` |
| `21-migrate-continue-on-rc13.sh` | this pass | 0 | `21-migrate-continue-on-rc13/` |

## `00-positive-control.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/00-positive-control
container  : 568e100d51339b15e77363ad0855ca73ffb4fdd21e56ab3d8e77286a84a420fa
  (a mid-stream plant runs with: docker exec -it -u node 568e100d51339b15e77363ad0855ca73ffb4fdd21e56ab3d8e77286a84a420fa bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/00-positive-control.sh in the container
-------------------------------------------------------------

=== 0 · adopt the corpus with the binary under test
jigc 1.0.0-rc.13

=== 1 · mint the task
task: record-that-the-ingest-queue

=== 2 · author a managed doc through the write verbs
created: adr:drop-the-oldest-sample-when
slots authored

=== 3 · THE CHANNEL — read the staged copy back
---
status: proposed
date: 2026-09-04
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
evidence in /Users/maurice/out/M50-walk-final/00-positive-control (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 10
  doc show --task    : 1
  doc show (any)     : 1
  §3.3 adjacent      : 2   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/00-positive-control"
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

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/01-destination-occupancy
container  : 8f6014c405eae6771e76076606eeeb906661a18a3034b46831fbb46ee1cd9c8f
  (a mid-stream plant runs with: docker exec -it -u node 8f6014c405eae6771e76076606eeeb906661a18a3034b46831fbb46ee1cd9c8f bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/01-destination-occupancy.sh in the container
-------------------------------------------------------------

=== 0 · adopt, so there is a managed store to rename inside
--- $ jigc setup
    jigc setup — adapter installed
    
    jigc is now wired into this project; setup installed:
      - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
      - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
      - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
      - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
        local to this checkout — git cannot track this path, so the hook is not in the install commit; a clone gets no drift backstop until `jigc setup` runs there
      - jigc guides → .claude/skills/jigc/SKILL.md   (jigc's own quickstart + migration notes, stamped with this build)
      - install commit → 5a7f72f   (setup's install files are committed on their own, off your first feature commit)
    — jigc · run `jigc start` for orientation; all writes through `jigc`.
--- exit: 0
--- $ jigc config set invocation-log true
    config: set `invocation-log` = `true` — written to `.jigc/config/`, uncommitted — commit it with your next commit
--- exit: 0

=== 1 · mint a task
task: decisions-that-will-collide

=== 2 · the doc this task's create-gate allows
--- $ jigc doc create adr --title Bound the number of distinct series --task decisions-that-will-collide
    adr:bound-the-number-of-distinct
--- exit: 0

=== 2b · a SECOND create of the same type in the same task
--- $ jigc doc create adr --title Cap the retention window at seven days --task decisions-that-will-collide
    blocking · write.identity-change — create rejected: this task's `decision` is already `adr:bound-the-number-of-distinct`, and this call would mint `adr:cap-the-retention-window` instead — a second document beside the first, not a correction of it
      at: adr:bound-the-number-of-distinct
      route: `jigc doc rename adr:bound-the-number-of-distinct --to 'Cap the retention window at seven days' --task decisions-that-will-collide` moves the doc this task already holds onto the title (and id) you asked for; a genuinely separate second document is its own task — finalize or discard this one first
    — jigc · run `jigc start` for orientation; all writes through `jigc`.
--- exit: 1
second-create exit: 1

=== 3 · what the task actually holds
--- $ jigc doc list --task decisions-that-will-collide
    id  path  state
    adr:bound-the-number-of-distinct  docs/decisions/bound-the-number-of-distinct.md  managed
    commit:decisions-that-will-collide  commit:decisions-that-will-collide  managed
--- exit: 0

=== 4 · rename onto an identity that is already taken — the occupancy probe
--- $ jigc doc rename adr:bound-the-number-of-distinct --to Bound the number of distinct series --task decisions-that-will-collide
    adr:bound-the-number-of-distinct (retitled "Bound the number of distinct series" — the id is unchanged)
--- exit: 0
self-rename (same identity) exit: 0

=== 5 · rename onto a DIFFERENT existing identity, if there are two docs
    only one adr exists in this task, so the two-doc collision is not reachable here.
    That is a finding about the create-gate, not about the occupancy guard:
    a one-role workflow cannot hold two docs of that type.

=== 6 · the committed home
--- $ jigc task finalize decisions-that-will-collide
    blocking · schema-conformance.required-slot-present — `docs/decisions/bound-the-number-of-distinct.md`: required slot in section `context` is empty
      at: adr:bound-the-number-of-distinct#context · line 10
      route: `jigc doc set-slot adr:bound-the-number-of-distinct#context --task decisions-that-will-collide --from-file -` to fill the empty slot
    blocking · schema-conformance.required-slot-present — `docs/decisions/bound-the-number-of-distinct.md`: required slot in section `decision` is empty
      at: adr:bound-the-number-of-distinct#decision · line 18
      route: `jigc doc set-slot adr:bound-the-number-of-distinct#decision --task decisions-that-will-collide --from-file -` to fill the empty slot
    blocking · schema-conformance.required-slot-present — `docs/decisions/bound-the-number-of-distinct.md`: required slot in section `consequences` is empty
      at: adr:bound-the-number-of-distinct#consequences · line 22
      route: `jigc doc set-slot adr:bound-the-number-of-distinct#consequences --task decisions-that-will-collide --from-file -` to fill the empty slot
    blocking · schema-conformance.field-value-conformant — `commit:decisions-that-will-collide`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
      at: commit:decisions-that-will-collide#header/type
      route: `jigc doc set-field commit:decisions-that-will-collide#header/type --task decisions-that-will-collide --value <value>` to correct the value
    blocking · schema-conformance.required-slot-present — `commit:decisions-that-will-collide`: required slot in section `summary` is empty
      at: commit:decisions-that-will-collide#summary · line 9
      route: `jigc doc set-slot commit:decisions-that-will-collide#summary --task decisions-that-will-collide --from-file -` to fill the empty slot
    — jigc · run `jigc start` for orientation; all writes through `jigc`.
--- exit: 3
--- $ jigc doc list
    jigc doc list — no committed docs
    note: docs are also staged in open task decisions-that-will-collide — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list --task decisions-that-will-collide`
--- exit: 0

=== 7 · top-level rename onto an existing identity
--- $ jigc rename adr:bound-the-number-of-distinct --to Bound the number of distinct series
    blocking · store.not-found — no managed doc `adr:bound-the-number-of-distinct` to rename (expected at docs/decisions/bound-the-number-of-distinct.md)
      at: adr:bound-the-number-of-distinct
      route: `jigc describe` lists the doctype surface — check the id you typed against it
--- exit: 1
top-level self-rename exit: 1

=== done
arm exit: 0   (0 = every step recorded an exit code and nothing silently succeeded)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/01-destination-occupancy (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 10
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 2   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/01-destination-occupancy"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 10 records · `doc show … --task` **0** · §3.3 adjacent 2

## `02-destroying-doors.sh`

exit **1**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/02-destroying-doors
container  : f984c5f919aa87522e26bcec824c59c87c4eadff7655481c82823888f6323232
  (a mid-stream plant runs with: docker exec -it -u node f984c5f919aa87522e26bcec824c59c87c4eadff7655481c82823888f6323232 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/02-destroying-doors.sh in the container
-------------------------------------------------------------

=== 0 · adopt, and commit the ignore rules BEFORE provisioning (see the order trap)
jigc 1.0.0-rc.13

=== A · THE REFUSAL CELLS — the three doors whose DestroyingDoor::code is Some

$ jigc milestone provision bound-the-store
blocking · milestone.leftover-holds-work — milestone:bound-the-store: `/work/.jigc/worktrees/cap-distinct-series` already holds 13 item(s) that `jigc milestone provision` would delete — git cannot read a repository there, so nothing can say those bytes are disposable:
  .claude
  .git
  .gitignore
  .jigc
  CLAUDE.md
  README.md
  build
  package.json
  secrets.env
  src
  staged.ts
  test
  untracked.ts
  at: /work/.jigc/worktrees/cap-distinct-series
  route: `jigc milestone provision bound-the-store --force` — but look inside that directory first and move out anything you need; the removal is permanent
[exit 1]

$ jigc milestone discard bound-the-store
blocking · milestone.dirty-worktree — milestone:bound-the-store: 1 sub-task worktree path(s) hold content, and `jigc milestone discard` would settle the record and tear the workbench down over them:
  /work/.jigc/worktrees/cap-distinct-series: .claude, .git, .gitignore, .jigc, CLAUDE.md, README.md, build, package.json, secrets.env, src, staged.ts, test, untracked.ts — not registered here, so the teardown leaves it on disk with no milestone naming it
  at: /work/.jigc/worktrees/cap-distinct-series
  route: look inside those paths and get out what you need (commit or stash what a live worktree holds), then re-run `jigc milestone discard bound-the-store` — or re-run with `--force` to abandon the milestone anyway: a registered worktree is removed with everything uncommitted in it, and a path nothing vouches for is left behind on disk for you to deal with
[exit 1]

$ jigc uninstall
blocking · uninstall.dirty-worktree — `.jigc/` holds content in 1 fan-out sub-task worktree path(s) that removing it would destroy:
  /work/.jigc/worktrees/cap-distinct-series: .claude, .git, .gitignore, .jigc, CLAUDE.md, README.md, build, package.json, secrets.env, src, staged.ts, test, untracked.ts — registered as a worktree nowhere in this repository
  route: get the work out of those worktrees first (commit, stash, or copy it), then re-run `jigc uninstall` — abandoning the milestone will not clear them: a milestone teardown removes only the worktrees this repository has registered, and none of these paths is; `jigc uninstall --force` deletes them with the install
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]

=== A · bars — the §5 standard: names the path · says what is lost · names the consent
  OK    provision refuses (exit non-zero)
  OK    provision carries milestone.leftover-holds-work
  OK    provision names the path
  OK    provision names the consent
  OK    discard refuses (exit non-zero)
  OK    discard carries milestone.dirty-worktree
  OK    discard names the path
  OK    discard names the consent
  OK    uninstall refuses (exit non-zero)
  OK    uninstall carries uninstall.dirty-worktree
  OK    uninstall names the path
  OK    uninstall names the consent

=== A · and the consent, run verbatim, WORKS — criterion (d) of the §5 standard
warning: removing the leftover directory /work/.jigc/worktrees/cap-distinct-series discards work that is not in git:
    .claude
    .git
    .gitignore
    .jigc
    CLAUDE.md
    README.md
    build
    package.json
    secrets.env
    src
    staged.ts
    test
    untracked.ts
  note: the leftover directory is the only copy of these bytes — they are not recoverable.
provisioned 2 worktree(s) for milestone:bound-the-store at base 7607371 (cap-distinct-series, prune-on-overflow)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    the leftover is gone after --force
  OK    provision --force narrates what it took
  OK    provision names it as a leftover, not a worktree
  OK    and says the bytes are unrecoverable

=== B · THE NARRATION CELLS — all four doors, including the two that were silent

=== B1 · discard --force over a LIVE worktree — and the whole --ignored axis
warning: removing the fan-out worktree /work/.jigc/worktrees/prune-on-overflow discards work that is not in git:
    build/ (ignored by git)
    secrets.env (ignored by git)
    untracked.ts (never staged)
  note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
discarded milestone:bound-the-store (2 sub-task(s); workbench removed)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    discard --force narrates
  OK    the --ignored axis is labelled
  OK    a never-staged file is labelled differently
  OK    an ignored DIRECTORY is named by its container, not its members (--ignored=matching)
  OK    M46's declared bound holds: the loss is VISIBLE, not prevented (exit 0)

=== B2 · milestone finalize — the door with NO refusal code, where loss is visible not prevented
finalized ebf7975 — Finalize milestone bound-the-store-again (1 sub-task)
  added .jigc/config/manifest.yaml
  modified docs/milestone-records/bound-the-store-again.md
  added src/cap.ts
  3 files committed
  sub-tasks: cap-distinct-series: 1 code file
  discarded with the fan-out worktrees (not committed, not recoverable):
    cap-distinct-series: build/ (ignored by git) · secrets.env (ignored by git) · untracked.ts (never staged)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
warning: removing the fan-out worktree /work/.jigc/worktrees/cap-distinct-series discards work that is not in git:
    build/ (ignored by git)
    secrets.env (ignored by git)
    untracked.ts (never staged)
  note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
  OK    finalize lands the sub-task code rather than refusing
  OK    finalize actually committed the sub-task's code
  OK    finalize narrates the ignored bytes its teardown takes

=== B3 · uninstall --force over a LIVE worktree (the other formerly-silent door)
warning: removing the fan-out worktree /work/.jigc/worktrees/cap-distinct-series discards work that is not in git:
    build/ (ignored by git)
    secrets.env (ignored by git)
    untracked.ts (never staged)
  note: the fan-out worktree is the only copy of these bytes — they are not recoverable.
jigc uninstall — repo-local install removed

  - removed .jigc/
  - unwired bootstrap reference ← CLAUDE.md
  - removed jigc allowlist permit ← .claude/settings.json
  - removed SessionStart hook ← .claude/settings.json
  - removed deny safety floor ← .claude/settings.json
  - removed pre-commit hook
  - removed jigc guide artifact
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    uninstall --force narrates

=== C · the NON-DIRECTORY leftover at the uninstall door (M49's completion audit)
total 16
drwxr-xr-x 3 node node 4096 Sep  4 18:28 .
drwxr-xr-x 5 node node 4096 Sep  4 18:28 ..
drwxr-xr-x 2 node node 4096 Sep  4 18:28 leftover-dir
-rw-r--r-- 1 node node   20 Sep  4 18:28 leftover-file

$ jigc uninstall
blocking · uninstall.dirty-worktree — cannot check `.jigc/worktrees/` for uncommitted fan-out work, so removing `.jigc/` could destroy it: could not read the leftover directory "/work/.jigc/worktrees/leftover-file": Not a directory (os error 20)
  route: make sure `git` is on PATH and the repository is readable, then re-run `jigc uninstall` — or, once you have confirmed the fan-out worktrees hold nothing you need, remove them yourself (`git worktree list`, then `git worktree remove`) and re-run
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    uninstall (no --force) refuses over the planted shapes (exit non-zero)
  OK    …carries uninstall.dirty-worktree
  OK    …names the regular FILE at the worktree path
  FAIL  …and the directory leftover beside it
  FAIL  …and names the consent
  OK    the FILE still exists afterwards
  OK    …with its bytes intact
  OK    the directory leftover still exists afterwards
  OK    .jigc/ itself was not removed

=== SUMMARY
  doors this arm was written against : 4 (3 refusing + 1 narrate-only)
  the registry fence lives in flow49_acceptance.rs, not here
  cell C (M49 audit): a regular FILE at a fan-out worktree path is a subject, not a shape
ARM 02 FAIL
(the arm exited 1 — that is data, not necessarily failure)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/02-destroying-doors (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 1
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/02-destroying-doors"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 1 records · `doc show … --task` **0** · §3.3 adjacent 0

## `03-upgrade-author-on-rc11.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/03-upgrade-author-on-rc11
container  : 47df2b3f85d5531f29100ed0f362f213322c69f651e952a113d1e32e3e90e524
  (a mid-stream plant runs with: docker exec -it -u node 47df2b3f85d5531f29100ed0f362f213322c69f651e952a113d1e32e3e90e524 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/03-upgrade-author-on-rc11.sh in the container
-------------------------------------------------------------

=== SKIPPED — this arm needs the OLD binary, and this pass is not it

The upgrade arm is the only two-binary arm, so it is driven as an explicit
sequence rather than by a normal walk pass:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc11 --only 03
  python3 run.py  carry <out-a>/03-upgrade-author-on-rc11 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc12 --only 10

`run.py carry` between the halves is load-bearing: run-session.sh writes its own
evidence into the out-dir, and handing that straight on would plant the rig's
droppings in the corpus the second half reads.
ARM 03 SKIPPED (needs jigc-gate:rc11; got jigc 1.0.0-rc.13)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/03-upgrade-author-on-rc11 (corpus + .session-transcript/ + PROVENANCE.txt)
  NO INVOCATION LOG — §3.3's primary channel is missing for this session
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

## `04-foreign-arm.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/04-foreign-arm
container  : 1fdce1fee2c785f00519fa9d417cf84e03f6f12d5131969b1c1c5f25a11f6c83
  (a mid-stream plant runs with: docker exec -it -u node 1fdce1fee2c785f00519fa9d417cf84e03f6f12d5131969b1c1c5f25a11f6c83 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/04-foreign-arm.sh in the container
-------------------------------------------------------------

=== 0 · adopt jigc onto a repo that already has documents (the adopter's day one)
jigc 1.0.0-rc.13

=== A · the store sweep — protocol §0.1's declared change

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · schema-conformance.unadopted-instance — committed file `docs/decisions/use-an-in-memory-buffer.md` sits at the `adr` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `adr` schema version
  at: docs/decisions/use-an-in-memory-buffer.md
  route: adopt — run `jigc ingest` to route it, or `jigc migrate docs/decisions/use-an-in-memory-buffer.md --as adr` to rewrite it into the managed `adr` shape; it is a foreign file, not an unmigrated managed doc
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version
  at: CHANGELOG.md
  route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc
a never-adopted file sits at a managed home — jigc was never handed it, so the sweep exits non-zero rather than report a green over a document it has never seen; run `jigc ingest` to route it (each finding above carries its own route), then re-validate.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    (a) it names the file it objects to
  OK    (a) …and the home that makes it jigc's business
  OK    (b) it says what the consequence is — why the sweep itself exits non-zero
  OK    (c) it names a route
  OK    the exit flipped — this is the declared change
  OK    the finding itself is untouched: still advisory, not blocking

=== A · the paired door — migrate-corpus stops claiming a file that is not its subject

$ jigc migrate-corpus --dry-run --format json
{
  "migrated": [],
  "already_current": [],
  "blocked": [],
  "unadopted": [
    {
      "severity": "advisory",
      "probe": "schema-conformance",
      "check": "unadopted-instance",
      "code": "schema-conformance.unadopted-instance",
      "key": {
        "code": "schema-conformance.unadopted-instance",
        "target": "CHANGELOG.md"
      },
      "message": "committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version",
      "location": {
        "address": "CHANGELOG.md",
        "line": 1,
        "col": 1
      },
      "route": "adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc"
    },
    {
      "severity": "advisory",
      "probe": "schema-conformance",
      "check": "unadopted-instance",
      "code": "schema-conformance.unadopted-instance",
      "key": {
        "code": "schema-conformance.unadopted-instance",
        "target": "docs/decisions/use-an-in-memory-buffer.md"
      },
      "message": "committed file `docs/decisions/use-an-in-memory-buffer.md` sits at the `adr` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `adr` schema version",
      "location": {
        "address": "docs/decisions/use-an-in-memory-buffer.md",
        "line": 1,
        "col": 1
      },
      "route": "adopt — run `jigc ingest` to route it, or `jigc migrate docs/decisions/use-an-in-memory-buffer.md --as adr` to rewrite it into the managed `adr` shape; it is a foreign file, not an unmigrated managed doc"
    }
  ],
  "unfilled": [],
  "commit": null,
  "hook_output": "",
  "dry_run": true
}
[exit 0]
  OK    migrate-corpus exits 0 over a brownfield repo (rc.11 exited 1 here)
  OK    the foreign files ride the DECLARED 'unadopted' key
  OK    …and are NOT in 'blocked' — emptiness of blocked IS the exit rule
  OK    both doors emit the same code from the one producer

=== A · (d) the route, run verbatim, WORKS — it routes rather than dead-ends

$ jigc ingest
jigc ingest — 5 candidate(s) classified  (sorted — deterministic report order)

needs-reconcile CHANGELOG.md → changelog
  blocking · conformance.section-renamed — section heading "[0.1.0] - 2026-01-04" does not match required section `unreleased-changes`
  at: changelog:changelog#unreleased-changes · line 3
  route: `jigc migrate CHANGELOG.md --as changelog` — it opens the `migrate-changelog` workflow, which rewrites the file to conformant shape and adopts it at finalize
needs-reconcile docs/decisions/use-an-in-memory-buffer.md → adr
  blocking · conformance.section-renamed — section heading "Status" does not match required section `context`
  at: adr:use-an-in-memory-buffer#context · line 3
  route: `jigc migrate docs/decisions/use-an-in-memory-buffer.md --as adr` — it opens the `migrate-adr` workflow, which rewrites the file to conformant shape and adopts it at finalize
unmanaged ./ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
unmanaged .claude/skills/jigc/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)

What the verdicts above mean, and what to do next:
  needs-reconcile — parses as the named type but conflicts; fix it per the row's route, then re-run `jigc ingest`.
  unmanaged — matches no managed schema; staying a plain file is a legitimate end-state — no action needed. To bring one under management: `jigc migrate <path> --as <doctype>`.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    (d) ingest routes each non-conformant file onward, naming the verb
  OK    (d) …and classifies rather than silently skipping

=== B · THE POSITIVE CONTROL: the exit must be able to go back to 0

$ jigc ingest
jigc ingest — 4 candidate(s) classified  (sorted — deterministic report order)

adoptable docs/decisions/prefer-a-bounded-buffer.md → adr  (adopted — indexed + baselined, no file moved)
unmanaged ./ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
unmanaged .claude/skills/jigc/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)

What the verdicts above mean, and what to do next:
  adoptable — conformant at its managed location; adopted register-only (indexed + baselined, the file stays in place).
  unmanaged — matches no managed schema; staying a plain file is a legitimate end-state — no action needed. To bring one under management: `jigc migrate <path> --as <doctype>`.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc validate
blocking · schema-conformance.schema-version-current — `docs/decisions/prefer-a-bounded-buffer.md`: field `schema-version` is absent; the committed doc predates the schema-version stamp (below the current schema-version 2)
  at: adr:prefer-a-bounded-buffer
  route: migrate — `docs/decisions/prefer-a-bounded-buffer.md` is a managed doc below the current schema-version 2; run `jigc migrate-corpus` to upgrade it
the committed corpus is below its schema-version — every other finding above was adjudicated against a schema those docs were never written to, so the sweep exits non-zero; run `jigc migrate-corpus`, then re-validate.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]

$ jigc migrate-corpus
corpus migration: 1 migrated, 0 already current, 0 blocked
  migrated   docs/decisions/prefer-a-bounded-buffer.md
committed 7b5822e — only the migrated paths were staged
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc validate
no findings — the committed store validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    before adoption the sweep refuses (non-zero)
  OK    the conformant foreign doc is ADOPTED register-only, no file moved
  OK    adoption hands off to the OTHER exit-flip member, not to green
  OK    …and that member is the schema-version one, with its own route
  OK    …routed at migrate-corpus, not back at ingest
  OK    …and it no longer says the file is unadopted — the discriminator moved it
  OK    AFTER the full adopter path the sweep goes GREEN — the exit is not stuck
  OK    the file was never moved by any of it
  OK    and it now carries the stamp that made it current

=== SUMMARY
  §5 standard for the declared change §0.1: (a) names it · (b) says the consequence
  · (c) names the route · (d) the route runs and the exit returns to 0
ARM 04 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/04-foreign-arm (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 19
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/04-foreign-arm"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 19 records · `doc show … --task` **0** · §3.3 adjacent 0

## `05-milestone-boundary.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/05-milestone-boundary
container  : aa7d6a8cf1723a5e3d15b515c8a8c3171200ce201478f1bd43cf8a6febbee7fa
  (a mid-stream plant runs with: docker exec -it -u node aa7d6a8cf1723a5e3d15b515c8a8c3171200ce201478f1bd43cf8a6febbee7fa bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/05-milestone-boundary.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== 1 · OWN linkage — the ordinary success path, as the baseline
finalized 28dafac — Finalize milestone bound-the-store (1 sub-task)
  added .jigc/config/manifest.yaml
  modified docs/milestone-records/bound-the-store.md
  added src/cap.ts
  3 files committed
  sub-tasks: cap-distinct-series: 1 code file
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    the sub-task's staged code lands

=== 2 · FOREIGN linkage — a cp -R of a repo whose SOURCE still exists
finalized 28dafac — Finalize milestone bound-the-store (1 sub-task)
  added .jigc/config/manifest.yaml
  modified docs/milestone-records/bound-the-store.md
  added src/cap.ts
  3 files committed
  sub-tasks: cap-distinct-series: 1 code file
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    the copy lands the sub-agent's code rather than a docs-only commit
  OK    …and does not silently report having provisioned nothing

=== 3a · NO linkage, nothing else to land — the boundary REFUSES
milestone:bound-the-store would land no work: no sub-task's docs promote to the store, and no sub-task worktree holds staged code — the boundary would commit only jigc's own bookkeeping and flip the milestone record to the terminal `joined`, after which the milestone could never be finalized again
  at: milestone:bound-the-store
  route: `jigc milestone provision bound-the-store` gives every sub-task a working area (a fresh clone has none; an existing one is reused); execute the sub-tasks, `git add` their work inside their worktrees, then re-run `jigc milestone finalize bound-the-store` — or settle the milestone as abandoned with `jigc milestone discard bound-the-store`
  OK    it refuses rather than landing a bookkeeping-only commit
  OK    the refusal is milestone.zero-contribution
  OK    …and says what landing anyway would cost — the terminal record
  OK    …and routes at provision, the verb that repairs it

=== 3b · NO linkage, but docs DO land — it proceeds and NAMES what it could not count
finalized 9461490 — Finalize milestone bound-the-store (1 sub-task)
  added .jigc/config/manifest.yaml
  promoted docs/decisions/cap-distinct-series.md
  modified docs/milestone-records/bound-the-store.md
  3 files committed
  sub-tasks: cap-distinct-series: 1 doc, unreadable worktree at .jigc/worktrees/cap-distinct-series, no code counted
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    the docs still promote
  OK    the unreadable worktree is NAMED, with its path
  OK    …and it says plainly that no code was counted

=== 3b · and the same fact reaches the machine surface, not only the prose
{
  "committed": {
    "files": 3,
    "hash": "9461490",
    "hook_output": "",
    "manifest": [
      {
        "kind": "added",
        "path": ".jigc/config/manifest.yaml"
      },
      {
        "kind": "promoted",
        "path": "docs/decisions/cap-distinct-series.md"
      },
      {
        "kind": "modified",
        "path": "docs/milestone-records/bound-the-store.md"
      }
    ],
    "sub_tasks": [
      {
        "code_files": 0,
        "discarded": [],
        "docs": 1,
        "id": "cap-distinct-series",
        "provisioned": false,
        "worktree_unreadable": true
      }
    ],
    "subject": "Finalize milestone bound-the-store (1 sub-task)"
  }
}
  OK    the --format json envelope carries worktree_unreadable
  OK    …and it is valid JSON, not prose with a key in it

=== SUMMARY
  three linkage states driven: own · foreign (cp -R) · none (mv), the last split
  on whether anything else was landing. M46's declared bound at this door —
  the loss is visible, not prevented — is cells 3a and 3b together.
ARM 05 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/05-milestone-boundary (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 1
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/05-milestone-boundary"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 1 records · `doc show … --task` **0** · §3.3 adjacent 0

## `06-pre-guard-repair.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/06-pre-guard-repair
container  : f990d28aaf76d03ab889c8fcdcc1a9ee03f2937889a9b7fa6aa4ff17e6dd8f14
  (a mid-stream plant runs with: docker exec -it -u node f990d28aaf76d03ab889c8fcdcc1a9ee03f2937889a9b7fa6aa4ff17e6dd8f14 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/06-pre-guard-repair.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== 0 · land a committed spec — a doctype with a repeatable item section
spec: spec:bound-the-distinct-series-count
  OK    the spec is committed

=== 1 · a human hand-adds an item heading with no anchor — the pre-guard state

$ jigc validate
blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/specs/bound-the-distinct-series-count.md` differs from the recorded state
  at: docs/specs/bound-the-distinct-series-count.md
  route: review the out-of-band edit to `docs/specs/bound-the-distinct-series-count.md` and re-author it through the owning workflow
blocking (gates at finalize) · conformance.item-heading-unanchored — `docs/specs/bound-the-distinct-series-count.md`: `### A hand-added criterion` sits at `###`, the schema-reserved item depth here, so the parser reads it as an item boundary — and it carries no `{#id}` anchor; if that line is slot prose, demote it to `####` or deeper; if it is a new item, anchor it in place — `### A hand-added criterion  {#<id>}` — with `<id>` a lowercase-kebab slug unique among this section's items
  at: spec:bound-the-distinct-series-count#criteria · line 21
2 finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize` / `jigc milestone finalize`.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the finding fires
  OK    it says WHY the parser cares — it reads as an item boundary
  OK    it prints the paste-able REPAIR rather than describing one
  OK    it covers the prose branch too — demote, do not anchor
  OK    …and says what a valid id looks like, so the paste can be completed
  OK    it does NOT name jigc doc add-item, the verb this state refuses
  OK    conformance.* is route-exempt by declaration, so the MESSAGE carries the repair

=== 2 · the same edit at a task door — where it actually gates
  OK    the store sweep is report-only about it
  OK    …and says which doors it DOES gate at
  OK    …naming all three of them

=== 3 · the out-of-band edit is detected, not silently absorbed (Inc 1's consequence)
  OK    the hand edit is seen as drift against the recorded baseline
  OK    …and routes at re-authoring through the owning workflow

=== DECLARED GAP — arm 2 of Increment 5 is NOT reached by this arm

`reconciliation.conflict-block`'s new migration arm — the one routing at
`jigc unmanage <source>` — needs a migration task whose source is a MANAGED doc
carrying a baseline, then an out-of-band edit to that same path. Driven here, a
`jigc migrate <path> --as adr` over a FOREIGN file does not reach it: the source
has no baseline, so the reconciliation guard never engages and the task door
reports `schema-conformance.unadopted-instance` instead.

Reaching it needs the same-path carve-out (M43) over an already-managed doc. That
is a longer setup than this arm should carry silently, so it is declared rather
than faked, and it is NOT counted as covered anywhere.

Its standing fence is the Increment 5 suite; the trial's coverage table (§6) must
record this cell as test-fenced, not trial-reached.

=== ALSO NOT REACHED — Increment 1's lock and merge

Only its observable consequence (section 3 above) is drivable. The base-relative
merge and the save-scoped lock need CONCURRENT jigc processes; the wave itself
reproduced the symptom with a 2000-round harness. Test-fenced, and §6 says so.

=== SUMMARY
  Increment 5 arm 1 driven; arm 2 declared unreached with its reason.
  Increment 1 reached only at its observable consequence, declared likewise.
ARM 06 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/06-pre-guard-repair (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 15
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/06-pre-guard-repair"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 15 records · `doc show … --task` **0** · §3.3 adjacent 0

## `07-changelog-gate.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/07-changelog-gate
container  : 0ad59bb5115495f495ded437ad20035fd96a201c814464e8b4f8ac0563684ccb
  (a mid-stream plant runs with: docker exec -it -u node 0ad59bb5115495f495ded437ad20035fd96a201c814464e8b4f8ac0563684ccb bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/07-changelog-gate.sh in the container
-------------------------------------------------------------

=== 0 · adopt, mint a gate-granting task, and fill its commit doc
jigc 1.0.0-rc.13
task: add-a-rate-limiter

=== A · the declared change: a conformant task no longer validates silent

$ jigc task validate add-a-rate-limiter
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  at: task:add-a-rate-limiter
  route: if the change is user-facing, record it in this task — `jigc doc create changelog --title Changelog --task add-a-rate-limiter`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task add-a-rate-limiter`; if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the advisory fires at jigc task validate — NEW at this door
  OK    (a) it names the workflow and the gate it granted
  OK    (b) it says the consequence — this task recorded no entry
  OK    (c) it names a route with real argv, scoped to THIS task
  OK    (c) …and names the not-user-facing branch too, so it is not a demand
  OK    exit stays 0 at the default severity
  OK    rc.11's byte-shape is gone: it does NOT say the task validates clean
  OK    it is the ONLY finding — nothing else was disturbed

=== B · the suppressing event is a WRITE-TOUCH, not the doc existing
  OK    creating the changelog and stopping does NOT suppress it

=== B · (d) now follow the printed route VERBATIM

$ jigc doc add-item changelog:changelog#unreleased-changes --title Added --task add-a-rate-limiter
changelog:changelog#unreleased-changes/added
[exit 0]
  OK    (d) the route runs and the advisory is suppressed
  OK    …and what remains is honest unfinished work, not the gate

=== C · the promoted severity — the door parity M46 shipped this for

$ jigc task validate add-a-second-rate-limiter
blocking · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  at: task:add-a-second-rate-limiter
  route: this project has promoted the gate to `blocking`, so record the entry in this task — `jigc doc create changelog --title Changelog --task add-a-second-rate-limiter`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task add-a-second-rate-limiter`; if the change is not user-facing, lower the gate back with `jigc config set validation.changelog-recording.gate-granted-unused.severity advisory`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 3]
  OK    promoted to blocking, task validate exits 3 on the state finalize refuses
  OK    …and back at the default severity the same state exits 0 again

=== SUMMARY
  §5 standard for the declared change §0.2: (a) names the gate · (b) says the
  consequence · (c) routes with live in-task argv · (d) the route suppresses it
ARM 07 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/07-changelog-gate (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 23
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 8   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/07-changelog-gate"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 23 records · `doc show … --task` **0** · §3.3 adjacent 8

## `08-identity-refusals.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/08-identity-refusals
container  : 9daa46976e38f44dbfc2223a8d4c48eba8623c4fe1a330007a85b39746603a16
  (a mid-stream plant runs with: docker exec -it -u node 9daa46976e38f44dbfc2223a8d4c48eba8623c4fe1a330007a85b39746603a16 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/08-identity-refusals.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== T10 · doc rename against a COMMITTED identity — the refusal, not exit 0
committed: adr:drop-the-oldest-sample

$ jigc doc rename adr:drop-the-oldest-sample --to Evict the oldest sample on overflow --task revise-the-overflow-policy
blocking · write.identity-change — rename rejected: `adr:drop-the-oldest-sample` is committed, so `--to "Evict the oldest sample on overflow"` would move its identity to `evict-the-oldest-sample` — a committed doc's path IS its identity, and referrers outside this task point at the old one. A same-slug retitle of the staged copy is supported; a re-slug is not
  at: adr:drop-the-oldest-sample
  route: `jigc rename adr:drop-the-oldest-sample --to 'Evict the oldest sample on overflow'` moves it for real — repointing every committed referrer in one transaction — once this task is finalized or discarded (it is a task-less, self-committing store op)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    it refuses with write.identity-change
  OK    it explains WHY a committed doc is different — the path IS the identity
  OK    it routes at the top-level, task-less verb
  OK    …and states the precondition rather than handing over an argv that would fail
  OK    the same-slug retitle is named as the thing that IS supported

=== T11 · doc rename against a SINGLETON doctype — nothing to rename at all
created: vision:vision

$ jigc doc rename vision:vision --to Product Vision --task project-vision
blocking · write.identity-change — rename rejected: `vision` is a singleton — its slug IS the type id and its `# H1` is supplied by the schema (`Vision`), so it carries no author-owned title or slug
  at: vision:vision
  route: nothing to rename: the name is part of the `vision` schema, so a genuinely wrong one is a pack change, not a write; edit the doc's prose with `jigc doc set-slot`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    the singleton refuses with write.identity-change
  OK    it says the H1 is the SCHEMA's, not the author's
  OK    its route says there is nothing to rename — a pack change, not a write
  OK    …and points at the verb that DOES edit it

=== OCCUPANCY · two committed docs, one identity — arm 01's declared bound, discharged
second committed: adr:cap-distinct-series
  OK    both docs really are committed

$ jigc rename adr:cap-distinct-series --to Drop the oldest sample on overflow
blocking · write.already-present — cannot rename to `adr:drop-the-oldest-sample` — a different doc already exists at docs/decisions/drop-the-oldest-sample.md
  at: adr:drop-the-oldest-sample
  route: give this doc an id nothing else answers to — re-run `jigc rename adr:cap-distinct-series --to 'Drop the oldest sample on overflow' --slug <other-slug>`; or, if `adr:drop-the-oldest-sample` is the doc you meant to work on, read it with `jigc doc show adr:drop-the-oldest-sample` and rename that one instead
[exit 1]
  OK    the destination-occupancy guard refuses
  OK    it names the occupied destination
  OK    nothing was moved — the loser is intact
  OK    …and so is the incumbent

=== OCCUPANCY · the sibling refusal on the SAME verb, for comparison

$ jigc rename adr:no-such-doc --to Something else
blocking · store.not-found — no managed doc `adr:no-such-doc` to rename (expected at docs/decisions/no-such-doc.md)
  at: adr:no-such-doc
  route: `jigc describe` lists the doctype surface — check the id you typed against it
[exit 1]
  OK    the unknown-doc refusal on this same verb DOES carry a route
  OBSERVE  the occupancy refusal above carries NO route line:
           | blocking · write.already-present — cannot rename to `adr:drop-the-oldest-sample` — a different doc already exists at docs/decisions/drop-the-oldest-sample.md
           |   at: adr:drop-the-oldest-sample
           |   route: give this doc an id nothing else answers to — re-run `jigc rename adr:cap-distinct-series --to 'Drop the oldest sample on overflow' --slug <other-slug>`; or, if `adr:drop-the-oldest-sample` is the doc you meant to work on, read it with `jigc doc show adr:drop-the-oldest-sample` and rename that one instead           -> recorded in pre-trial-findings.md as a candidate finding.
           -> NOT adjudicated here: this arm builds the instrument.

=== T-idempotent · a same-title rename is a no-op ack, not a dressed-up failure

$ jigc rename adr:cap-distinct-series --to Cap distinct series at a ceiling
no-op: adr:cap-distinct-series already holds the title "Cap distinct series at a ceiling" at docs/decisions/cap-distinct-series.md — nothing renamed, nothing committed
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the idempotent rename exits 0
  OK    …and says plainly that nothing moved

=== SUMMARY
  T10 committed-identity refusal · T11 singleton refusal · destination occupancy
  in the committed home · the idempotent no-op. Arm 01's declared bound is
  discharged: the two-doc collision needs two COMMITTED docs and the top-level verb.
ARM 08 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/08-identity-refusals (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 40
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/08-identity-refusals"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 40 records · `doc show … --task` **0** · §3.3 adjacent 0

## `09-increment-8-doors.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/09-increment-8-doors
container  : c64341bf7c7edb78294f942b5a4d19138b41991449d23f6ff8d44afb892fab8c
  (a mid-stream plant runs with: docker exec -it -u node c64341bf7c7edb78294f942b5a4d19138b41991449d23f6ff8d44afb892fab8c bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/09-increment-8-doors.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== T5 · jigc describe <positional> — a tip where rc.11 printed a bare clap error

$ jigc describe adr
error: unexpected argument 'adr' found

  tip: describe is the whole menu — the single-item form is not built. Narrow it by kind: `jigc describe --workflows` tours the workflows alone; `jigc describe --doctypes` the doc-types; `jigc describe --commands` the command refs. For how one workflow resolves, `jigc start --explain` is the resolution trace

Usage: jigc describe [OPTIONS]

For more information, try '--help'.
[exit 2]
  OK    it still refuses the single-item form
  OK    …but now says the form is not built
  OK    …and names the three narrowing flags
  OK    …and points at the resolution trace for one workflow

=== T2 · jigc milestone finalize --help says what the door actually does

$ jigc milestone finalize --help
The milestone commit boundary — run the by-task-id join, then land BOTH halves of the fan-out as one logical boundary with a CLI-synthesized message: the join's suffix-resolved doc bodies, materialized into the parent staging area, and the code staged in each sub-task worktree, folded in by task id. A blocking join finding (a same-doc clash, an unknown milestone) routes to stderr and commits nothing

Usage: jigc milestone finalize [OPTIONS] <MILESTONE_ID>

Arguments:
  <MILESTONE_ID>  The milestone id (the slug under `.jigc/milestones/`)

Options:
      --carry-staged     Declare the pre-milestone staged index state deliberate: proceed past the carryover refuse (`finalize.carried-staged`). The aggregate commit is built from the sub-task worktrees, so the carried entries never ride it — they stay staged across the boundary either way. Inert when nothing is carried
      --format <FORMAT>  Output format: `agent` (default, terse text for a coding agent) · `json` (machine-readable) · `human` (formatted for a terminal) [default: agent] [possible values: agent, json, human]
  -h, --help             Print help
[exit 0]
  OK    it names BOTH halves the boundary lands
  OK    …and says a blocking join finding commits nothing

=== T3 · a write verb at an unstaged address — the ProvisionRoute shapes

$ sh -c echo x | jigc doc set-slot 'adr:not-created-yet#context' --from-file - --task probe-the-write-path
no staged instance for `adr:not-created-yet#context` — provision it first (`jigc start` / `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` derives the id from the title (`X` → slug), not the task id — address writes at that title-derived id
[exit 1]
  OK    an un-created address says so, rather than writing nowhere
  OK    …and routes at the verb that provisions it
  OK    …and warns the id comes from the TITLE, not the task id — the M44 footgun

$ sh -c echo 'body text' | jigc doc set-slot 'commit:probe-the-write-path#body' --from-file - --task probe-the-write-path
set slot commit:probe-the-write-path#body (10 chars)
[exit 0]
  OK    a compose-provisioned doc accepts the write — the contrast that makes T3 legible

=== T4 · the sub-task base-pin refusal routes at provision, not at discard

$ jigc workflow sub-task --task cap-distinct-series
task `cap-distinct-series` is pinned to base 86edcce but you're on 9e94a65 — this is a sub-task of milestone `bound-the-store`, and a sub-task's work happens in its own worktree at .jigc/worktrees/cap-distinct-series, cut from that base rather than in this checkout: run `jigc milestone provision bound-the-store` — it adds a worktree that is missing and leaves one that exists untouched — then re-run this from that worktree
[exit 1]
  OK    it explains the pin rather than just refusing
  OK    …names the worktree the work belongs in
  OK    …and routes at provision — NOT at task discard, which used to exit 0 on a live record
  OK    …and does not offer discarding the sub-task

=== T6 · the router's closing text — the increment's own audit finding, re-measured

$ tail -6 /tmp/router.txt

That catalog is the selectable subset. A workflow outside it is reached by name
with the same `--workflow` form — `jigc describe --workflows` lists every
workflow, the ones the catalog leaves out included, and each of those carries
the reason it is hidden from the catalog.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the closing text points at describe --workflows for the fuller set

  MEASURED · workflows absent from the router catalog, and whether each states why:
           catalog: 12 selectable · absent: 21 · of those, WITHOUT a stated reason: 0
           without a reason: (none)
           -> recorded in pre-trial-findings.md (PT-C) with both readings.
           -> NOT adjudicated here: this arm builds the instrument.

=== T7 · step:locate-from-spec names the commit doc, not the sanctioned-optional spec write

$ jigc workflow implement-from-spec --preview
preview: workflow `implement-from-spec` — no task minted. This shows what it will ask before you commit to running it.
To run it for real: `jigc start --workflow implement-from-spec "<intent>"`   — mints the task and composes this.
Below, `--task your-task-id` marks where the minted id goes.

Implement from a committed spec. The intent is:


Pick the spec this work implements from the committed specs below and bind it by
its FULL address exactly as listed — `spec:<slug>`, the `spec:` prefix included
(a bare slug is rejected as a malformed address) — then re-run to read its
criteria:



Run: `jigc task bind spec <SPEC_ADDRESS> your-task-id`
Run: `jigc start --task your-task-id`

The bound spec's criteria — `spec:<slug>#criteria`, one item per criterion, each
carrying the `{#id}` anchor that addresses it:



Wire every criterion you satisfy back to the test that proves it. Once that test
passes, set `maps-to-test` on the criterion, addressed by the id in its `{#id}`
anchor above — never a slug you re-derive from the title (`jigc doc show
spec:<slug> --format json` prints the same ids):

jigc doc set-field spec:<slug>#criteria/<id>/maps-to-test --value <path>#<test-fn> --task your-task-id

The `#<test-fn>` names the test symbol — a named test function. A closure-based test
framework registers its tests with no named symbol, so there is nothing to name: drop
the `#<test-fn>` and give the `<path>` alone. A file-only anchor is accepted — it
resolves on the file's presence, no symbol required.

The anchor path is repo-relative and resolves against the staged index, not the working
tree: the file — and the `#<test-fn>`, when you name one — must be staged. The blocking
`doc-code.criterion-maps-to-test` check re-resolves it at finalize, so leave `maps-to-test`
off a criterion this task did not cover rather than pointing it at a test you have not
written and staged.

Wire this task's commit to the spec it implements, so the landed record links back
to what it built. `commit.implements` points at the spec you bound above — set it
by that spec's full address:

jigc doc set-field commit:your-task-id#implements --value spec:<slug> --task your-task-id

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the commit write you just made, which the committed store does
not carry yet:

jigc doc show commit:your-task-id --task your-task-id

That read names the commit doc because the commit write above is the one this step
always makes. The spec is staged by this task ONLY when you set `maps-to-test`: when
you did, the same `--task` read serves that staged copy too — put the spec's address
in place of the commit's. When you left `maps-to-test` off, this task stages nothing
of the spec, and the task-less `--format json` read above — of the COMMITTED spec —
is the one that serves it.

A spec can leave a decision open — a question its prose raises but does not settle.
When you settle one while implementing, record it before you finalize: this workflow
grants the `adr` gate, so mint a decision record through the `jigc doc create adr`
route below and author its slots. A decision that lives only in the code is one the
next reader cannot find.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged.

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task your-task-id`

The `adr` schema is the authority on what you write into it — its required slots
and fields, each field's enum members, and every address a write can take:

jigc doc schema adr

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects). Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address:

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot adr:<slug>#context --from-file - --task your-task-id <<'EOF'
<the forces that made this decision necessary>
EOF

jigc doc set-slot adr:<slug>#context --from-file - --task your-task-id
jigc doc set-slot adr:<slug>#decision --from-file - --task your-task-id
jigc doc set-slot adr:<slug>#consequences --from-file - --task your-task-id

The `options` slot is optional — fill it only when alternatives were genuinely
weighed. Its `## Options` heading renders either way; an empty optional slot is
conformant and never blocks finalize:

jigc doc set-slot adr:<slug>#options --from-file - --task your-task-id

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

`<slug>` is the slug the title minted, and this task's own index names it back —
every doc the task stages, each by the `<type>:<slug>` identity the read takes:

jigc doc list adr --task your-task-id

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show adr:<slug> --task your-task-id

When done, set the required Conventional-Commits type — your editorial call on
what this change does. The subject renders as `<type>(<scope>): <summary>`, so
write the summary without a type or scope prefix of its own — the `type` field
already carries it. Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address.

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot commit:your-task-id#summary --from-file - --task your-task-id <<'EOF'
<one line saying what changed>
EOF

Set the type, then stage the summary prose:

Run: `jigc doc set-field commit:your-task-id#type --value <COMMIT_TYPE> --task your-task-id`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:your-task-id#summary --from-file - --task your-task-id`
<<author: commit:your-task-id#summary>>

The `commit` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema commit

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:your-task-id#scope --value <area> --task your-task-id
jigc doc set-slot commit:your-task-id#body --from-file - --task your-task-id

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:your-task-id#trailers --title Co-Authored-By --task your-task-id
jigc doc set-field commit:your-task-id#trailers/<id>/value --value "Name <email>" --task your-task-id

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show commit:your-task-id --task your-task-id

Validate and commit the task as one logical commit. Finalize commits only the
staged set plus the docs it manages; unstaged edits and untracked files are left
out, and with nothing staged over a dirty tree it refuses. Anything still staged
from BEFORE this task was minted makes finalize refuse too (one blocking finding
per carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate your-task-id` — it
previews part of what finalize gates on (this task's content findings, the
carryover gate, the owner-artifact causes that need no staging, and the
granted-but-unused changelog gate), without committing anything; the staged set,
promotion and the commit itself are decided at finalize.

Run: `jigc task finalize your-task-id`
create-gates: adr   — the doc-types this task is allowed to create; any other type is refused
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the composed step text is readable without minting a task
  OK    …and its read-back names the task's commit doc

=== SUMMARY
  six of Increment 8's seven doors driven here; T1 (the hook frame) is R2's and B1's.
  T6 is reported as a measurement against the shipped binary, not as a verdict.
ARM 09 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/09-increment-8-doors (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 21
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/09-increment-8-doors"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 21 records · `doc show … --task` **0** · §3.3 adjacent 0

## `10-upgrade-continue-on-rc12.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/10-upgrade-continue-on-rc12
container  : a3eaeb735b36b4cc81fabb63e25542d8b91d6b0d0c5e156ae6613e88294da40f
  (a mid-stream plant runs with: docker exec -it -u node a3eaeb735b36b4cc81fabb63e25542d8b91d6b0d0c5e156ae6613e88294da40f bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/10-upgrade-continue-on-rc12.sh in the container
-------------------------------------------------------------

=== SKIPPED — no baseline from the first half in this corpus

This arm continues the corpus `03-upgrade-author-on-rc11.sh` left behind. Run:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc11 --only 03
  python3 run.py  carry <out-a>/03-upgrade-author-on-rc11 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc12 --only 10
ARM 10 SKIPPED (no .upgrade-baseline — the rc.11 half has not run)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/10-upgrade-continue-on-rc12 (corpus + .session-transcript/ + PROVENANCE.txt)
  NO INVOCATION LOG — §3.3's primary channel is missing for this session
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

## `11-item-region-shipped-doctypes.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/11-item-region-shipped-doctypes
container  : 328bbacc75eeabbc8b43dffb3a4d1b84e3a797cb992d7bec537bc1255a660e7d
  (a mid-stream plant runs with: docker exec -it -u node 328bbacc75eeabbc8b43dffb3a4d1b84e3a797cb992d7bec537bc1255a660e7d bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/11-item-region-shipped-doctypes.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== 0 · the set, re-derived from this binary — every shipped item block
  arch-doc#components  slots=1 nested=0 id-from=string optional-settable=[implemented-by]
  changelog#releases  slots=0 nested=1 id-from=string optional-settable=[link]
  changelog#releases/<id>/changes  slots=1 nested=0 id-from=enum optional-settable=[]
  changelog#unreleased-changes  slots=1 nested=0 id-from=enum optional-settable=[]
  commit#trailers  slots=0 nested=0 id-from=string optional-settable=[]
  completion-record#findings  slots=1 nested=0 id-from=string optional-settable=[]
  decisions-log#entries  slots=1 nested=0 id-from=string optional-settable=[]
  deferral-ledger#entries  slots=1 nested=0 id-from=string optional-settable=[]
  milestone-record#tasks  slots=0 nested=0 id-from=? optional-settable=[]
  prd#requirements  slots=1 nested=0 id-from=string optional-settable=[]
  roadmap#milestones  slots=2 nested=0 id-from=string optional-settable=[]
  spec#criteria  slots=1 nested=0 id-from=string optional-settable=[maps-to-test]
  MEASURED · 12 item blocks · multi-slot: 1 · slot∧nested: 0
  OK    the census reaches the five blocks this arm drives
  OK    …and no shipped block is slot∧nested — the audit's silent-loss face is fenced, not reachable here

=== mint · one task per create-gate the drive needs (three open tasks; every write is --task-explicit)

$ jigc start --workflow record-change record the probe release
task minted: record-the-probe-release

Record the change on the project changelog. Create-or-update the changelog
singleton — safe whether or not it already exists (an existing committed changelog
is copied in for append):

Run: `jigc doc create changelog --title Changelog --task record-the-probe-release`

The `changelog` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema changelog

Then author the change. To cut a release, mint a release item (its id is minted
from the version title you give — e.g. `1.0.0` mints `1-0-0` — and its `date` is
stamped on create). `add-item` PRINTS the new item's address; use that printed
address verbatim for every follow-up verb — never re-spell the version string:

jigc doc add-item changelog:changelog#releases --title "<version>" --task record-the-probe-release
#   → prints e.g. changelog:changelog#releases/1-0-0  (call this <release-addr>)
jigc doc set-field <release-addr>/link --value "<diff-url>" --task record-the-probe-release

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot <group-addr>/notes --from-file - --task record-the-probe-release <<'EOF'
<one bullet per change>
EOF

Under that release, add one nested change-group per category and author its notes
(one bullet per change). The category is the group's id (added / changed /
deprecated / removed / fixed / security). Again, drive the printed address verbatim:

jigc doc add-item <release-addr>/changes --title "<category>" --task record-the-probe-release
#   → prints e.g. changelog:changelog#releases/1-0-0/changes/added  (call this <group-addr>)
jigc doc set-slot <group-addr>/notes --from-file - --task record-the-probe-release

For a change that is not yet cut into a release, add the change-group under the
staged section instead:

jigc doc add-item changelog:changelog#unreleased-changes --title "<category>" --task record-the-probe-release
#   → prints e.g. changelog:changelog#unreleased-changes/added  (call this <group-addr>)
jigc doc set-slot <group-addr>/notes --from-file - --task record-the-probe-release

Or make it one call — the batch alternative to the whole sequence above (run it
instead of the create + per-entry verbs, or after them — the create it implies
over a doc this task already staged acks `already existed — copied in for
update` and changes nothing): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write, no printed-address dance. Over an already-committed changelog it copies
the committed doc in and appends — existing releases are untouched; an entry whose
title mints an id the changelog already holds is refused (`write.already-present`),
the WHOLE payload rejected and nothing staged, so edit that entry in place with the
per-entry `set-field` / `set-slot` lines above rather than re-authoring it here:

jigc doc author changelog --from-file - --task record-the-probe-release

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show changelog:changelog --task record-the-probe-release

When done, set the required Conventional-Commits type — your editorial call on
what this change does. The subject renders as `<type>(<scope>): <summary>`, so
write the summary without a type or scope prefix of its own — the `type` field
already carries it. Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address.

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot commit:record-the-probe-release#summary --from-file - --task record-the-probe-release <<'EOF'
<one line saying what changed>
EOF

Set the type, then stage the summary prose:

Run: `jigc doc set-field commit:record-the-probe-release#type --value <COMMIT_TYPE> --task record-the-probe-release`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:record-the-probe-release#summary --from-file - --task record-the-probe-release`
<<author: commit:record-the-probe-release#summary>>

The `commit` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema commit

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:record-the-probe-release#scope --value <area> --task record-the-probe-release
jigc doc set-slot commit:record-the-probe-release#body --from-file - --task record-the-probe-release

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:record-the-probe-release#trailers --title Co-Authored-By --task record-the-probe-release
jigc doc set-field commit:record-the-probe-release#trailers/<id>/value --value "Name <email>" --task record-the-probe-release

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show commit:record-the-probe-release --task record-the-probe-release

Validate and commit the task as one logical commit. Finalize commits only the
staged set plus the docs it manages; unstaged edits and untracked files are left
out, and with nothing staged over a dirty tree it refuses. Anything still staged
from BEFORE this task was minted makes finalize refuse too (one blocking finding
per carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate record-the-probe-release` — it
previews part of what finalize gates on (this task's content findings, the
carryover gate, the owner-artifact causes that need no staging, and the
granted-but-unused changelog gate), without committing anything; the staged set,
promotion and the commit itself are decided at finalize.

Run: `jigc task finalize record-the-probe-release`
resume: `jigc start --task record-the-probe-release`   — re-composes this workflow if context is lost
what's-left: `jigc task validate record-the-probe-release`   — previews part of the finalize gate: this task's content findings, the carryover gate, the owner-artifact causes that need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the commit surface at finalize
task scope: `jigc doc` writes default to the single active task; `--task record-the-probe-release` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; once a sibling task commits a path this one also touches, resuming or finalizing here blocks and names the overlapping paths
create-gates: changelog   — the doc-types this task is allowed to create; any other type is refused
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  T=record-the-probe-release
  TS=bound-the-ingest-queue  (plan — the spec create-gate)
  TR=plan-the-first-milestone  (planning — the roadmap create-gate)
  OK    three tasks minted

=== (d) · --slug decouples id from title; refused on an enum id-from; the collision route runs

$ jigc doc create changelog --title Changelog --task record-the-probe-release
changelog:changelog
[exit 0]

$ jigc doc add-item changelog:changelog#releases --title 1.0.0 --task record-the-probe-release
changelog:changelog#releases/1-0-0
[exit 0]
  REL=changelog:changelog#releases/1-0-0
  OK    the release address the binary emitted is the one this arm drives

$ jigc doc add-item changelog:changelog#releases --title 2.0.0 --slug my-slug --task record-the-probe-release
changelog:changelog#releases/my-slug
[exit 0]
  OK    --slug my-slug mints the item under my-slug, not under the title's 2-0-0

$ jigc doc add-item changelog:changelog#unreleased-changes --title added --slug my-slug --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "identity-change",
      "code": "write.identity-change",
      "key": {
        "code": "write.identity-change",
        "target": "changelog:changelog#unreleased-changes/my-slug/category"
      },
      "message": "add-item rejected: `unreleased-changes` derives its item id from enum field `category`, so the heading IS the member and the anchor equals it — `--slug my-slug` would mint a second identity beside the member, not an id",
      "location": {
        "address": "changelog:changelog#unreleased-changes/my-slug/category",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc add-item changelog:changelog#unreleased-changes --title added` mints the member itself; a name of your own belongs in the item's prose"
    }
  ]
}[exit 1]
  OK    on an enum id-from block the same --slug is refused write.identity-change
  OK    …whose route mints the member itself (the heading IS the member)

$ jigc doc add-item changelog:changelog#releases --title Two point oh --slug my-slug --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "already-present",
      "code": "write.already-present",
      "key": {
        "code": "write.already-present",
        "target": "changelog:changelog#releases"
      },
      "message": "write rejected: item \"my-slug\" in section \"releases\" is already present",
      "location": {
        "address": "changelog:changelog#releases",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc add-item changelog:changelog#releases --title 'Two point oh' --slug my-slug-2 --task record-the-probe-release` mints it beside the item already there under an id of your own — `--slug` drives the item id verbatim, so two titles that slug alike can coexist. If this is a correction of that item rather than a second entry, edit it in place with `jigc doc set-slot` / `jigc doc set-field` instead"
    }
  ]
}[exit 1]
  OK    a colliding --slug draws write.already-present
  ROUTE=jigc doc add-item changelog:changelog#releases --title 'Two point oh' --slug my-slug-2 --task record-the-probe-release
  OK    …whose route is the mechanical --slug <id>-N mint, argv-complete with --task

$ sh -c jigc doc add-item changelog:changelog#releases --title 'Two point oh' --slug my-slug-2 --task record-the-probe-release
changelog:changelog#releases/my-slug-2
[exit 0]
  OK    …and run VERBATIM it succeeds, landing my-slug-2

=== (a) · --unset of an ABSENT optional field acks the no-op — rc.12 refused it write.not-present

$ jigc doc set-field changelog:changelog#releases/1-0-0/link --unset --task record-the-probe-release --format json
{
  "already_absent": true,
  "copied_in": false,
  "findings": [],
  "op": "set-field",
  "target": {
    "doctype": "changelog",
    "item": "1-0-0",
    "leaf": "link",
    "section": "releases",
    "slug": "changelog"
  },
  "unset": true
}
[exit 0]
  OK    exit 0 with already_absent: true at changelog#releases/<id>/link

$ jigc doc set-field changelog:changelog#releases/1-0-0/link --unset --task record-the-probe-release
unset changelog:changelog#releases/1-0-0/link (already absent — nothing changed)
[exit 0]
  OK    …and the agent text says so rather than blocking

$ jigc doc create spec --title Bounded ingest --task bound-the-ingest-queue
spec:bounded-ingest
[exit 0]

$ jigc doc add-item spec:bounded-ingest#criteria --title Queue is bounded --task bound-the-ingest-queue
spec:bounded-ingest#criteria/queue-is-bounded
[exit 0]
  CRIT=spec:bounded-ingest#criteria/queue-is-bounded

$ jigc doc set-field spec:bounded-ingest#criteria/queue-is-bounded/maps-to-test --unset --task bound-the-ingest-queue --format json
{
  "already_absent": true,
  "copied_in": false,
  "findings": [],
  "op": "set-field",
  "target": {
    "doctype": "spec",
    "item": "queue-is-bounded",
    "leaf": "maps-to-test",
    "section": "criteria",
    "slug": "bounded-ingest"
  },
  "unset": true
}
[exit 0]
  OK    the same at spec#criteria/<id>/maps-to-test — the other optional settable item field

=== (c) · a heading at the depth the rejection itself names lands intact — audit fix 2
  -- face 1: the nested single-slot, changelog#releases/<id>/changes/<id>/notes

$ jigc doc add-item changelog:changelog#releases/1-0-0/changes --title added --task record-the-probe-release
changelog:changelog#releases/1-0-0/changes/added
[exit 0]

$ jigc doc set-slot changelog:changelog#releases/1-0-0/changes/added/notes --from-file /tmp/arm11.QqRchS/shallow.md --task record-the-probe-release
blocking · write.slot-heading-depth — heading at schema-reserved depth `#` in slot prose at line 3; `#####` is the shallowest depth free at this address — use it or rephrase
  at: changelog:changelog#releases/1-0-0/changes/added/notes · line 3
  route: demote the heading to the depth the message names (or deeper), or rephrase it as plain prose, then re-run the same write
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  prescribed free depth at changes/added/notes: '#####'
  OK    a '#' heading is refused write.slot-heading-depth, naming the free depth

$ jigc doc set-slot changelog:changelog#releases/1-0-0/changes/added/notes --from-file /tmp/arm11.QqRchS/notes.md --task record-the-probe-release
set slot changelog:changelog#releases/1-0-0/changes/added/notes (31 chars)
[exit 0]
  OK    prose carrying a heading at exactly that depth is accepted

$ jigc doc show changelog:changelog#releases/1-0-0/changes/added/notes --task record-the-probe-release --format json
"Intro\n\n##### Deep enough\n\nmore"
[exit 0]
  OK    …and reads back VERBATIM through the pinned read
  -- face 2: the multi-slot block, NON-LAST slot — roadmap#milestones/<id>/proves

$ jigc doc create roadmap --title Roadmap --task plan-the-first-milestone
roadmap:roadmap
[exit 0]

$ jigc doc add-item roadmap:roadmap#milestones --title M1 the probe --task plan-the-first-milestone
roadmap:roadmap#milestones/m1-the-probe
[exit 0]
  MS=roadmap:roadmap#milestones/m1-the-probe

$ jigc doc set-slot roadmap:roadmap#milestones/m1-the-probe/proves --from-file /tmp/arm11.QqRchS/shallow.md --task plan-the-first-milestone
blocking · write.slot-heading-depth — heading at schema-reserved depth `#` in slot prose at line 3; `#####` is the shallowest depth free at this address — use it or rephrase
  at: roadmap:roadmap#milestones/m1-the-probe/proves · line 3
  route: demote the heading to the depth the message names (or deeper), or rephrase it as plain prose, then re-run the same write
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  prescribed free depth at milestones/<id>/proves: '#####'
  OK    the multi-slot block names its own free depth

$ jigc doc set-slot roadmap:roadmap#milestones/m1-the-probe/proves --from-file /tmp/arm11.QqRchS/proves.md --task plan-the-first-milestone
set slot roadmap:roadmap#milestones/m1-the-probe/proves (42 chars)
[exit 0]

$ jigc doc set-slot roadmap:roadmap#milestones/m1-the-probe/decomposition --from-file /tmp/arm11.QqRchS/decomp.md --task plan-the-first-milestone
set slot roadmap:roadmap#milestones/m1-the-probe/decomposition (41 chars)
[exit 0]

$ jigc doc set-slot roadmap:roadmap#milestones/m1-the-probe/proves --from-file /tmp/arm11.QqRchS/proves2.md --task plan-the-first-milestone
set slot roadmap:roadmap#milestones/m1-the-probe/proves (43 chars)
[exit 0]
  OK    both slots accept a heading at the named depth, and the NON-LAST slot accepts an UPDATE over it

$ jigc doc show roadmap:roadmap#milestones/m1-the-probe --task plan-the-first-milestone --format json
{
  "decomposition": "Decomp intro\n\n##### Decomp heading\n\ntail",
  "id": "m1-the-probe",
  "proves": "Proves v2\n\n##### Proves heading v2\n\nafter2",
  "title": "M1 the probe"
}
[exit 0]
  OK    the item reads back with BOTH slots intact — the next '#### Decomposition' sub-label was not swallowed

$ sed -n /^## Milestones/,$p .jigc/tasks/plan-the-first-milestone/docs/roadmap:roadmap.md
## Milestones

### M1 the probe  {#m1-the-probe}

#### Proves

Proves v2

##### Proves heading v2

after2

#### Decomposition

Decomp intro

##### Decomp heading

tail
[exit 0]
  -- face 3: the writer's half — spec#criteria, set-field insert+update over heading-bearing prose

$ jigc doc set-slot spec:bounded-ingest#criteria/queue-is-bounded/statement --from-file /tmp/arm11.QqRchS/shallow.md --task bound-the-ingest-queue
blocking · write.slot-heading-depth — heading at schema-reserved depth `#` in slot prose at line 3; `####` is the shallowest depth free at this address — use it or rephrase
  at: spec:bounded-ingest#criteria/queue-is-bounded/statement · line 3
  route: demote the heading to the depth the message names (or deeper), or rephrase it as plain prose, then re-run the same write
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  prescribed free depth at criteria/<id>/statement: '####'

$ jigc doc set-slot spec:bounded-ingest#criteria/queue-is-bounded/statement --from-file /tmp/arm11.QqRchS/stmt.md --task bound-the-ingest-queue
set slot spec:bounded-ingest#criteria/queue-is-bounded/statement (45 chars)
[exit 0]

$ jigc doc set-field spec:bounded-ingest#criteria/queue-is-bounded/maps-to-test --value test/ingest.test.ts --task bound-the-ingest-queue
set spec:bounded-ingest#criteria/queue-is-bounded/maps-to-test = test/ingest.test.ts
[exit 0]

$ jigc doc set-field spec:bounded-ingest#criteria/queue-is-bounded/maps-to-test --value test/rollup.test.ts --task bound-the-ingest-queue
set spec:bounded-ingest#criteria/queue-is-bounded/maps-to-test = test/rollup.test.ts
[exit 0]
  maps-to-test bullets on disk: 1

$ sed -n /^## Criteria/,$p .jigc/tasks/bound-the-ingest-queue/docs/spec:bounded-ingest.md
## Criteria

### Queue is bounded  {#queue-is-bounded}

Given a queue

#### Then it is bounded

tail

<!-- fields -->
- maps-to-test: test/rollup.test.ts
[exit 0]
  OK    insert then update over the heading-bearing prose lands ONE bullet, not two

$ jigc doc show spec:bounded-ingest#criteria/queue-is-bounded --task bound-the-ingest-queue --format json
{
  "id": "queue-is-bounded",
  "maps-to-test": "test/rollup.test.ts",
  "statement": "Given a queue\n\n#### Then it is bounded\n\ntail",
  "title": "Queue is bounded"
}
[exit 0]
  OK    …and the read returns the SECOND value, never the stale first, with the prose intact

=== (b) · two hand-planted field bullets → task validate blocks conformance.duplicate-field — rc.12 exited 0

$ jigc task validate record-the-probe-release
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` — this task's in-flight version of the doc
  at: CHANGELOG.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    baseline: the task validates at exit 0 BEFORE the plant

$ grep -n ^- date: .jigc/tasks/record-the-probe-release/docs/changelog:changelog.md
15:- date: 2026-09-04
16:- date: 2020-01-01
29:- date: 2026-09-04
34:- date: 2026-09-04
[exit 0]

$ jigc task validate record-the-probe-release
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` — this task's in-flight version of the doc
  at: CHANGELOG.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
blocking · conformance.duplicate-field — `CHANGELOG.md`: field `date` appears more than once in this field group — a declared field carries exactly one value, and which one this is cannot be read from the schema
  at: changelog:changelog#releases/1-0-0/date · line 15
  route: delete the repeated `date:` line, keeping the one value you intend — this is the one case a managed file is yours to hand-edit: the damage was made out-of-band, so it is repaired where it happened
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 3]

$ jigc task validate record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "advisory",
      "probe": "file-state",
      "check": "staged-copy",
      "code": "file-state.staged-copy",
      "key": {
        "code": "file-state.staged-copy",
        "target": "CHANGELOG.md"
      },
      "message": "staged copy of `CHANGELOG.md` — this task's in-flight version of the doc",
      "location": {
        "address": "CHANGELOG.md",
        "line": 1,
        "col": 1
      },
      "route": "no action needed — the staged copy is validated in-task and baselined when its finalize lands"
    },
    {
      "severity": "blocking",
      "probe": "conformance",
      "check": "duplicate-field",
      "code": "conformance.duplicate-field",
      "key": {
        "code": "conformance.duplicate-field",
        "target": "changelog:changelog#releases/1-0-0/date"
      },
      "message": "`CHANGELOG.md`: field `date` appears more than once in this field group — a declared field carries exactly one value, and which one this is cannot be read from the schema",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/date",
        "line": 15,
        "col": 1
      },
      "route": "delete the repeated `date:` line, keeping the one value you intend — this is the one case a managed file is yours to hand-edit: the damage was made out-of-band, so it is repaired where it happened"
    }
  ]
}[exit 3]
  OK    task validate EXITS NON-ZERO over the duplicate — the exit flip (rc.12: exit 0)
  OK    …reporting conformance.duplicate-field as blocking, located at the item's field
  OK    …and the text names the line and the hand-edit repair

$ jigc task validate record-the-probe-release
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` — this task's in-flight version of the doc
  at: CHANGELOG.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    restoring the bytes returns the task to exit 0 — the plant, not the doc, was the fault

=== SUMMARY
  driven on the shipped blocks the census names: changelog (releases · nested changes ·
  the enum unreleased-changes), spec#criteria, roadmap#milestones. The 18-cell shape
  space is test-fenced (item_region_shape_space.rs) and was not re-driven; the slot∧nested
  face is unreachable on any shipped block, measured above.
ARM 11 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/11-item-region-shipped-doctypes (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 55
  doc show --task    : 3
  doc show (any)     : 3
  §3.3 adjacent      : 4   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/11-item-region-shipped-doctypes"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 55 records · `doc show … --task` **3** · §3.3 adjacent 4

## `12-doors-that-lie.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/12-doors-that-lie
container  : b6f5f9e839b55c1e7409705bad8d46f0ad6e38bc1493bae4132d34aec4ad9f7c
  (a mid-stream plant runs with: docker exec -it -u node b6f5f9e839b55c1e7409705bad8d46f0ad6e38bc1493bae4132d34aec4ad9f7c bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/12-doors-that-lie.sh in the container
-------------------------------------------------------------

=== 0 · adopt
jigc 1.0.0-rc.13

=== A · THE MINT DOORS — an unknown --workflow is refused BEFORE anything mints

$ jigc milestone create bound the store
minted milestone:bound-the-store (shared base d2660fd)
record: docs/milestone-records/bound-the-store.md   — the committed record this milestone's state lives in
record commit: 427f072   — the record on its own; anything else you had staged stayed staged
next: `jigc milestone add-task bound-the-store "<intent>"`   — add the milestone's first sub-task
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc milestone add-task bound-the-store cap distinct series --workflow no-such-workflow
blocking · workflow-refs.unknown-workflow — no workflow `no-such-workflow` — a sub-task's `--workflow` must name a workflow the loaded packs provide: architecture-documentation, completion, decided-task, dev-task, do-research, form-vision, implement-from-spec, increment, ingest-existing, migrate-adr, migrate-arch-doc, migrate-changelog, migrate-completion-record, migrate-decisions-log, migrate-deferral-ledger, migrate-idea, migrate-prd, migrate-research, migrate-roadmap, migrate-spec, migrate-vision, milestone-execution, park-idea, plan, planning, project-setup, quick-fix, record-change, record-decision, record-dogfood, router, single-task, sub-task
  route: `jigc milestone add-task bound-the-store "<intent>" --workflow <workflow-id>` — nothing was minted, recorded or committed
[exit 1]
  OK    add-task refuses (exit non-zero)
  OK    …carrying workflow-refs.unknown-workflow
  OK    …enumerating the loaded set (names sub-task AND dev-task)
  OK    …and says what it left behind
  OK    no working area was minted
  OK    .jigc/tasks/ is unchanged
  OK    no commit landed
  OK    the record does not name the refused sub-task
  route as printed : jigc milestone add-task bound-the-store "<intent>" --workflow <workflow-id>
  route as filled  : jigc milestone add-task bound-the-store "cap distinct series" --workflow sub-task
  OK    a route was printed
  OK    …and carries both placeholders

$ bash -c jigc milestone add-task bound-the-store "cap distinct series" --workflow sub-task
added task:cap-distinct-series to milestone:bound-the-store
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the filled route mints the sub-task

=== A · add-from-spec — the same check, and it precedes the spec read

$ jigc milestone add-from-spec bound-the-store spec:nothing --workflow no-such-workflow
blocking · workflow-refs.unknown-workflow — no workflow `no-such-workflow` — a sub-task's `--workflow` must name a workflow the loaded packs provide: architecture-documentation, completion, decided-task, dev-task, do-research, form-vision, implement-from-spec, increment, ingest-existing, migrate-adr, migrate-arch-doc, migrate-changelog, migrate-completion-record, migrate-decisions-log, migrate-deferral-ledger, migrate-idea, migrate-prd, migrate-research, migrate-roadmap, migrate-spec, migrate-vision, milestone-execution, park-idea, plan, planning, project-setup, quick-fix, record-change, record-decision, record-dogfood, router, single-task, sub-task
  route: `jigc milestone add-from-spec bound-the-store spec:nothing --workflow <workflow-id>` — nothing was minted, recorded or committed
[exit 1]
  OK    add-from-spec refuses the unknown workflow (exit non-zero)
  OK    …carrying workflow-refs.unknown-workflow
  OK    …and says nothing was minted or seeded
  OK    …while the same address under a VALID workflow reaches the spec read (store.not-found)
  OK    the workflow check precedes the spec read (no store.not-found on the unknown-workflow path)

=== B · task discard <sub> — the COMMITTED record tells the truth

$ jigc milestone add-task bound-the-store prune on overflow --workflow dev-task
added task:prune-on-overflow to milestone:bound-the-store
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc task discard cap-distinct-series
discarded task cap-distinct-series
[exit 0]
{
  "fields": {
    "base": {
      "sha": "d2660fd9c8e04bd2f307577cb3596e124a1eae83",
      "short": "d2660fd"
    },
    "schema-version": "3",
    "status": "active"
  },
  "item-count": 2,
  "schema-version": 3,
  "sections": {
    "tasks": [
      {
        "id": "cap-distinct-series",
        "intent": "cap distinct series",
        "status": "discarded",
        "task-id": "cap-distinct-series",
        "workflow": "sub-task"
      },
      {
        "id": "prune-on-overflow",
        "intent": "prune on overflow",
        "status": "active",
        "task-id": "prune-on-overflow",
        "workflow": "dev-task"
      }
    ]
  },
  "slug": "bound-the-store",
  "type": "milestone-record"
}
  record items:
    cap-distinct-series=discarded workflow=sub-task
    prune-on-overflow=active workflow=dev-task
  OK    the discarded sub-task reads discarded in the committed record
  OK    …and its sibling stays active
  OK    exactly one commit landed
  OK    …named as the discard
  OK    …and it is record-only (touches just the record file)
  OK    the working area is gone
milestone:bound-the-store tasks (1): prune-on-overflow
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    list-tasks no longer shows it active
  OK    …and still shows the live one

=== C · FRESH CLONE — list-tasks is a read; add-task is the door that re-seeds
  clone: /tmp/clone-uVvkMY/repo
  .jigc/tasks before      : ls: cannot access '.jigc/tasks': No such file or directory 

$ jigc milestone list-tasks bound-the-store
milestone:bound-the-store tasks (1): prune-on-overflow
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  .jigc/tasks after read  : ls: cannot access '.jigc/tasks': No such file or directory 
  .jigc/milestones        : ls: cannot access '.jigc/milestones': No such file or directory 

$ jigc milestone add-task bound-the-store evict cold entries
added task:evict-cold-entries to milestone:bound-the-store
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  .jigc/tasks after add   : evict-cold-entries prune-on-overflow 
  OK    the fresh clone's read answers from the record
  OK    …and writes nothing under .jigc/tasks/ or .jigc/milestones/
  OK    an operating door (add-task) DOES re-seed the absent area
  OK    …sourcing the workflow from the record (dev-task), not the pack default

=== D · THE READ-VERB CARVE-OUT — all 12 VerbKind::Read leaves, snapshot before/after
  OK    the live ordinary task exists

  verb                     exit   changed under .jigc/ (excluding logs), HEAD, index
  upgrade                  0      (nothing)
  describe                 0      (nothing)
  validate                 0      (nothing)
  doc show                 0      (nothing)
  doc schema               0      (nothing)
  doc list                 0      (nothing)
  task list                0      (nothing)
  task diff                0      (nothing)
  task validate            3      .jigc/index/edges.json .jigc/index/edges.json.lock 
  config get               0      (nothing)
  config list              0      (nothing)
  milestone list-tasks     0      (nothing)

  OK    no read verb created anything under .jigc/tasks/ or .jigc/milestones/, moved HEAD, or moved the index

=== D · the one admitted write — .jigc/index/ is a self-healing derived cache, checked not assumed
  edges.json stamp : ba2cc517530cfa3ba1d15c4f9b0352a6d50d41a8
  HEAD             : ba2cc517530cfa3ba1d15c4f9b0352a6d50d41a8
  OK    the cache is stamped with the committed HEAD it was built from
  OK    a second run of the materializing verb is a fixed point (changes nothing)

=== SUMMARY
  read verbs this arm was written against : 12 (the registry fence lives in read_verb_acts_nothing.rs)
  the per-verb change table above is an observation; only .jigc/tasks/, .jigc/milestones/, HEAD and the index are asserted
ARM 12 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/12-doors-that-lie (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 31
  doc show --task    : 0
  doc show (any)     : 3
  §3.3 adjacent      : 3   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/12-doors-that-lie"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 31 records · `doc show … --task` **0** · §3.3 adjacent 3

## `13-freeze-every-layer.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/13-freeze-every-layer
container  : 0348552c877131a083760e8d3f0e50db1fda4ba507be738c3b1ec42eb2fc34fd
  (a mid-stream plant runs with: docker exec -it -u node 0348552c877131a083760e8d3f0e50db1fda4ba507be738c3b1ec42eb2fc34fd bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/13-freeze-every-layer.sh in the container
-------------------------------------------------------------

=== 0 · adopt, and mint the task FIRST (see the order trap)
jigc 1.0.0-rc.13
  OK    the task the in-task cells need exists

=== A · THE PROJECT LAYER — a shadow that drops `options` blocks every pack-loading door

$ jigc describe
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `2d86024048d5b791e3df9da79761612b6c3ef2e2f08ed6f1ae5132a4680f34cb`, recomputed `379147c79e9d2030291d0b01f7830b17461df98d86f20f43b2fb884b17cf5c48`) — the project schema shadow /work/.jigc/config/schemas/adr.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migrat…
[exit 1]
  OK    jigc describe — blocks · names adr · names the shadow · states the rule · carries a route

$ jigc validate
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `2d86024048d5b791e3df9da79761612b6c3ef2e2f08ed6f1ae5132a4680f34cb`, recomputed `379147c79e9d2030291d0b01f7830b17461df98d86f20f43b2fb884b17cf5c48`) — the project schema shadow /work/.jigc/config/schemas/adr.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migrat…
[exit 1]
  OK    jigc validate — blocks · names adr · names the shadow · states the rule · carries a route

$ jigc doc schema adr
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `2d86024048d5b791e3df9da79761612b6c3ef2e2f08ed6f1ae5132a4680f34cb`, recomputed `379147c79e9d2030291d0b01f7830b17461df98d86f20f43b2fb884b17cf5c48`) — the project schema shadow /work/.jigc/config/schemas/adr.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migrat…
[exit 1]
  OK    jigc doc schema adr — blocks · names adr · names the shadow · states the rule · carries a route

$ jigc doc show commit:probe-the-freeze --task probe-the-freeze
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `2d86024048d5b791e3df9da79761612b6c3ef2e2f08ed6f1ae5132a4680f34cb`, recomputed `379147c79e9d2030291d0b01f7830b17461df98d86f20f43b2fb884b17cf5c48`) — the project schema shadow /work/.jigc/config/schemas/adr.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migrat…
[exit 1]
  OK    jigc doc show commit:probe-the-freeze --task probe-the-freeze — blocks · names adr · names the shadow · states the rule · carries a route

$ jigc doc create adr --title Probe --task probe-the-freeze
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `2d86024048d5b791e3df9da79761612b6c3ef2e2f08ed6f1ae5132a4680f34cb`, recomputed `379147c79e9d2030291d0b01f7830b17461df98d86f20f43b2fb884b17cf5c48`) — the project schema shadow /work/.jigc/config/schemas/adr.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migrat…
[exit 1]
  OK    jigc doc create adr --title Probe --task probe-the-freeze — blocks · names adr · names the shadow · states the rule · carries a route

$ jigc task validate probe-the-freeze
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `2d86024048d5b791e3df9da79761612b6c3ef2e2f08ed6f1ae5132a4680f34cb`, recomputed `379147c79e9d2030291d0b01f7830b17461df98d86f20f43b2fb884b17cf5c48`) — the project schema shadow /work/.jigc/config/schemas/adr.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migrat…
[exit 1]
  OK    jigc task validate probe-the-freeze — blocks · names adr · names the shadow · states the rule · carries a route

$ jigc doc list
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `2d86024048d5b791e3df9da79761612b6c3ef2e2f08ed6f1ae5132a4680f34cb`, recomputed `379147c79e9d2030291d0b01f7830b17461df98d86f20f43b2fb884b17cf5c48`) — the project schema shadow /work/.jigc/config/schemas/adr.yaml changes the shape of a frozen doctype, which the freeze forbids at every layer (`design/corpus-migrat…
[exit 1]
  OK    jigc doc list — blocks · names adr · names the shadow · states the rule · carries a route
  OK    every listed door blocks with the four-part message
  OK    no adr was created through the blocked create door
  MEASURED · jigc task list over the shadow: exit 0 — jigc task list — 1 active task(s)

=== A · the route, run VERBATIM, clears the block
  route: rm /work/.jigc/config/schemas/adr.yaml
  OK    the route is an rm of the shadow file itself, inside this repo's project config

$ bash -c rm /work/.jigc/config/schemas/adr.yaml
[exit 0]
  OK    the shadow is gone

$ jigc describe
jigc describe — a tour of what this project lets you compose and author.

The workflows you can compose here. To read a task-minting one's full step text before you commit to running it, run `jigc workflow <id> --preview`, which composes the steps without minting a task; a workflow that mints nothing has no preview, and `jigc start --workflow <id>` composes it directly without minting either. architecture-documentation is Authors living architecture documentation for one part of the system and commits it. Reach for it when a part of the system needs a durable, code-checked description so a later reader can orient without reverse-engineering it.

completion is The milestone-completion loop (audit → triage → fix → re-verify), with the human-gated triage and fix-round halts composed as structural Checkpoints, then authors the per-milestone completion-record (verdict + owner-artifact + findings) and appends the triage decisions to the running decisions-log. Reach for it when you are closing a milestone — auditing the assembled whole, triaging and fixing its findings, then recording the verdict and the genuine-audit owner-artifact — and you want the phase walk, its human gates, and the record authoring made visible. It is hidden from the router catalog: invoked by name when a milestone closes (`jigc start --workflow completion "<milestone>"`) — the completion audit follows the milestone lifecycle, not an intent the router disambiguates.

decided-task is The dev-workflow plus a decision-recording step — scope, implement test-first, run the gate, record the task's design decision on the running decisions log, then land one commit. The lightweight sub-milestone path for a task that decides something worth keeping, without milestone-planning ceremony. Reach for it when the work is one coherent change you can carry to a single commit AND it makes a design decision worth recording, and you want the test-first discipline plus a managed decision record without invoking milestone planning.

dev-task is The reduced-linear dev-workflow — scope the task, implement test-first, run the project's own gate, and land exactly one commit, all as one pass. Reach for it when the work is one coherent change you can carry from intent to a single commit, and you want the test-first discipline kept visible. If it makes a decision worth recording, pick decided-task; if it touches code a managed doc describes (a documented symbol renamed or reshaped), pick single-task, whose doc gates cover the update.

do-research is Investigate one question and record what it found — author a standalone research doc (question, findings, sources) and land it as one commit, so a later vision or decision can rest on committed evidence. Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and you want that investigation carried through the binary to one committed research record.

form-vision is Form or revise the project's vision from committed research — create the vision singleton, ground it in the research it rests on, and author its thesis, invariants, and open questions, then land it as one commit. A re-entry flow — set `grounded-in`, re-compose, then author against the grounding research now in view. Reach for it when the project's direction is worth a durable, managed anchor that later work compares against, and you want it formed from and traceable to the committed research behind it.

implement-from-spec is An implementation pass driven by a committed spec — locate against it, implement, optionally record a decision, and commit. Reach for it when a spec already exists and you are turning its agreed criteria into code.

increment is The increment-workflow's outer loop (plan → execute → validate → fix), with its three human-gated halts composed as structural Checkpoint directives. Reach for it when you are working a whole roadmap increment, not a single task, and you want the plan/execute/validate/fix phases and their halt points made visible. It is hidden from the router catalog: mints no task — an outer loop whose execute phase mints one task per planned task, and the catalog lists task-minting work-workflows only.

ingest-existing is An onboarding pass that scans an existing repo's documents and routes each to a verdict for jigc management. Reach for it when you are adopting jigc on a repo that already has docs and want to bring them under management rather than starting fresh. It is hidden from the router catalog: mints no task — an onboarding scan reached by name or through the `jigc ingest` verb, and the catalog lists task-minting work-workflows only.

migrate-adr is Migrate a foreign architectural decision record into a managed `adr` at `decisions/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant ADR (a MADR or Nygard file under `docs/adr/` or similar) and `jigc migrate <path> --as adr` is rewriting it into a managed `adr`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as adr`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-arch-doc is Migrate a foreign architecture document into a managed `arch-doc` at `architecture/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant architecture document (an arc42, a C4 model, or a README "Architecture" section under `docs/` or similar) and `jigc migrate <path> --as arch-doc` is rewriting it into a managed `arch-doc`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as arch-doc`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-changelog is Migrate a foreign `CHANGELOG.md` into the managed `changelog` singleton — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant `CHANGELOG.md` and `jigc migrate <path> --as changelog` is rewriting it into the managed changelog. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as changelog`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-completion-record is Migrate a foreign milestone-close record into a managed per-milestone `completion-record` at `completions/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs (resolving the required `owner-artifact` by the artifact ladder), and the task finalizes. Reach for it when an existing project carries a non-conformant milestone-close record (a GSD completion file, a "M3 done" write-up) and `jigc migrate <path> --as completion-record` is rewriting it into a managed completion record. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as completion-record`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-decisions-log is Migrate a foreign decisions log into the managed `decisions-log` singleton — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant decisions log (a DECISIONS.md, a running "choices we made" file) and `jigc migrate <path> --as decisions-log` is rewriting it into the managed decisions log. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as decisions-log`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-deferral-ledger is Migrate a foreign record of deferred decisions and parked ideas into the managed `deferral-ledger` singleton — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant owed-and-when record (a deferred-decisions list, a "later" backlog kept as prose) and `jigc migrate <path> --as deferral-ledger` is rewriting it into the managed deferral ledger. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as deferral-ledger`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-idea is Migrate a foreign parked-idea note into a managed `idea` at `ideas/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant parked-idea note (a someday/maybe file, a backlog entry kept as prose) and `jigc migrate <path> --as idea` is rewriting it into a managed idea. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as idea`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-prd is Migrate a foreign product requirements document into a managed `prd` at `prds/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant PRD (a free-form vision-and-requirements file under `docs/` or similar) and `jigc migrate <path> --as prd` is rewriting it into a managed `prd`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as prd`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-research is Migrate a foreign research or investigation note into a managed `research` doc at `research/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant research note (loose investigation notes, an evidence dump under `research/` or similar) and `jigc migrate <path> --as research` is rewriting it into a managed research doc. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as research`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-roadmap is Migrate a foreign roadmap or milestone plan into the managed `roadmap` singleton — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant roadmap (a milestone plan, a phase list, a GSD ROADMAP.md) and `jigc migrate <path> --as roadmap` is rewriting it into the managed roadmap. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as roadmap`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-spec is Migrate a foreign specification into a managed `spec` at `specs/` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant spec (a free-form requirements or acceptance-criteria file under `docs/specs/` or similar) and `jigc migrate <path> --as spec` is rewriting it into a managed `spec`. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as spec`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

migrate-vision is Migrate a foreign vision/charter document into the managed `vision` singleton at the repo-root `VISION.md` — the CLI stages the foreign bytes, the LLM rewrites them to canonical shape through the write verbs, and the task finalizes. Reach for it when an existing project carries a non-conformant vision document (a free-form charter/thesis/direction file) and `jigc migrate <path> --as vision` is rewriting it into the managed vision. Off-router — only reached through the migrate verb. It is hidden from the router catalog: verb-routed — reached only through `jigc migrate <path> --as vision`, which stages the foreign source bytes the author step rewrites; a router pick would compose with no staged source.

milestone-execution is A parallel execution of a planned milestone's sub-tasks, joined and committed as one boundary. Reach for it when a milestone is decomposed into independent sub-tasks you want run together and landed as a single commit. It is hidden from the router catalog: composes a degenerate walk — zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb, so a router pick could never land.

park-idea is Park one shaped-but-unscheduled direction — author a standalone idea doc (the direction and the trigger that would bring it back) and land it as one commit, so a mid-work thought has a durable home cheaper than losing it. Reach for it when a direction is worth keeping but not worth scheduling now, and you want it carried through the binary to one committed idea record with the trigger that would revisit it.

plan is A planning pass that authors the specification for upcoming work and commits it. Reach for it when the work needs its goal and acceptance criteria agreed up front, before any code is written.

planning is The milestone-planning loop (scope → detect-gaps → settle → review → decompose), with the human-gated Settle composed as a structural Checkpoint, then authors this milestone's plan-time gate-record and the three running working-docs (roadmap / deferral-ledger / decisions-log) the loop maintains. Reach for it when you are opening a milestone — scoping it against the roadmap, settling its gaps, and cutting it into increments — and you want the phase walk, its human gate, the gate-record that blocks the commit until every plan-time gate is answered, and the running-doc authoring made visible. It is hidden from the router catalog: invoked by name with the milestone in hand (`jigc start --workflow planning "<milestone>"`) — opening a milestone is a deliberate lifecycle act, not an intent the router disambiguates.

project-setup is A bootstrap pass that develops a project idea into its first product requirements document and commits it. Reach for it when you are at the very start of a brand-new project, before any spec or code, and want to turn an idea into shared requirements.

quick-fix is A small commit-only fix — locate, implement, and commit, with no decision record. Reach for it when the change is a quick, low-stakes fix that needs nothing preserved beyond the commit itself, and changes no code a managed doc describes — if it renames or removes a documented symbol, the arch-doc anchoring it needs updating too, so pick single-task instead.

record-change is Record a change on the changelog — create-or-update the singleton, author a release (or a staged change-group) with its nested category groups, and commit. Reach for it when a user-facing change needs recording on the changelog — staged now, or cut into a versioned release. It is hidden from the router catalog: reached by name for the deliberate record-a-change/cut-a-release pass (`jigc start --workflow record-change`); routine change recording already rides `single-task`'s record-changelog step, so a catalog line would duplicate it.

record-decision is A decision-recording pass that authors an ADR for a choice already made and commits it — no code changes. Reach for it when a call has been made and is worth preserving with its rationale, and there is nothing to build — just the record to lay down.

record-dogfood is The measured-dogfood recording spine — transcribes a completed measured run's organic fact counts and seeded instrument checks from the committed capture into a per-run dogfood-record, with the judged verdict and the owner-artifact holding the raw capture, then finalizes. Reach for it when a measured dogfood run has completed and its capture (transcript + raw hook log/tally + manifest) is in hand — the run's facts, instrument checks, verdict, and capture artifact need a durable record authored through the binary. It is hidden from the router catalog: invoked by name once a measured run's capture is in hand (`jigc start --workflow record-dogfood "<run>"`) — the recording follows the dogfood protocol, not an intent the router disambiguates.

router is A selection pass that presents the available work-workflows and routes an intent to the right one. Reach for it when you have an intent but are unsure which work-workflow fits, and want the catalog to choose from. It is hidden from the router catalog: mints no task — the catalog it presents lists task-minting work-workflows, and this selection pass is the front door that presents them rather than an entry on its own menu.

single-task is An end-to-end scoped change — locate, implement, optionally record a decision and a changelog entry, and commit, all as one task. Reach for it when the work is one coherent change you can hold in your head and carry from intent to commit in a single pass. Also the pick when the change touches documented code (a symbol a managed doc names, renamed or reshaped) — its doc gates cover the update, where quick-fix's commit-only path does not. It grants the changelog gate alongside the ADR one, so a user-facing change is recorded as it lands; when the change is internal, author no entry — finalize notes the unused gate as an advisory and commits anyway.

sub-task is One sub-task of a milestone — locate, implement, and author a commit into an isolated working area for a later join. Reach for it when a milestone execution needs the unit it fans out to; it is not selected directly. It is hidden from the router catalog: spawned by a milestone execution's fan-out and joined at the parent's finalize — it has no commit boundary of its own, so a router pick could never land.

The doc-types you can author. adr is A dated architectural decision record, capturing the context a choice was made in, the choice itself, and its consequences, with an optional link to the decision it supersedes. Reach for it when a choice is worth preserving with its rationale, so a later reader can recover why the call was made or supersede it on the record.

arch-doc is Living architecture documentation for one part of the system — its overview, its components and the code that implements them, and the decisions behind it. Reach for it when a part of the system is worth a durable description that stays honest against the code, so a later reader (or agent) can orient without reverse-engineering it.

changelog is A Keep-a-Changelog singleton — staged unreleased changes plus the cut releases, each grouped by category, maintained over the life of the project. Reach for it when a user-facing change lands and the project keeps a human-readable record of what changed, staged now and cut into versioned releases over time.

commit is A Conventional-Commits message — a typed, scoped header over a subject line, an optional body, and trailers — rendered into the git commit rather than persisted as a repo file. Reach for it when you need to record what a change does and why at the moment it lands, in the form git and reviewers already read.

completion-record is The per-milestone completion record — the audit verdict, the owner-artifact the genuine audit produced, and each triaged finding with its disposition. Reach for it when a milestone reaches its completion audit, so its verdict, the audit's owner-artifact, and the triaged findings need a durable per-milestone record.

decisions-log is The running log of decisions made — each with the date it was settled and the reasoning behind it. Reach for it when the planning loop reaches Settle (or completion reaches triage) and a decision is made worth preserving with its rationale, dated, on the record.

deferral-ledger is The running ledger of deferred decisions and parked ideas, each keyed to the milestone trigger that resurfaces it. Reach for it when the planning loop reaches Settle and a decision is deferred or an idea parked rather than made now, so it needs a durable owed-and-when record.

dogfood-record is The per-run measured-dogfood record — the run's case and pinned binary, the transcribed organic fact counts, the seeded instrument checks, the judged verdict, and the owner-artifact holding the raw capture. Reach for it when a measured dogfood run completes, so its transcribed facts, seeded instrument checks, verdict, and capture artifact need a durable per-run record.

idea is One shaped-but-unscheduled direction, with the trigger that would bring it back. Reach for it when a direction is worth keeping but not worth scheduling now, so it needs a durable home cheaper than losing the thought and a note of when to revisit it.

milestone-record is The per-milestone team-ready state record — the shared base-SHA pin, the ordered sub-task list, and each sub-task's intent and status, resumable from a fresh clone. Reach for it when a milestone is executing and its in-flight state (base, sub-tasks, statuses) must be committed and legible so a teammate or fresh session can continue it, not stranded in the gitignored workbench.

planning-record is One milestone's plan-time gate record — every planning gate answered with the evidence that discharges it, or an explicit N/A and why. Reach for it when a milestone is being planned, and each plan-time gate needs an answer recorded before the plan is allowed to close.

prd is A product requirements document — the vision, the requirements, and the context for a piece of work, the first managed document a fresh project develops its idea into. Reach for it when you are at the start of a new project and want to turn an idea into a shared statement of what to build and why, before any spec or code.

research is One investigation and what it found — the evidence a vision or design is formed from. Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and that record should stay addressable by what it grounds.

roadmap is The running milestone spine — each milestone with what it proves and the prose decomposition of its increments. Reach for it when the planning loop reaches Decompose and a milestone's breakdown is worth recording on the durable roadmap.

spec is A specification of what a task implements — its goal, its context, and a set of testably-phrased acceptance criteria. Reach for it when you need to pin down what a piece of work must deliver before building it, so the criteria are agreed up front and code can be checked against them.

vision is The project's charter — its thesis, the invariants it holds, and its open questions — grounded in the research it was formed from. Reach for it when the project's direction is worth a durable, managed anchor that later work compares against, formed from and traceable to the research behind it.

And the commands jigc hands you along the way. bind-spec (dev pack) Bind the spec this work implements to the task's spec role. create-adr (dev pack) Create a new ADR in the current task. create-arch-doc (dev pack) Create a new arch-doc in the current task. create-changelog (dev pack) Create-or-update the running changelog singleton in the current task. create-completion-record (methodology pack) Create the per-milestone completion-record fresh in the current task. create-dogfood-record (methodology pack) Create the per-run dogfood-record fresh in the current task. create-idea (methodology pack) Park the shaped direction as a fresh idea doc in the current task. create-ledger (methodology pack) Create-or-update the running deferral-ledger singleton in the current task. create-log (methodology pack) Create-or-update the running decisions-log singleton in the current task. create-planning-record (methodology pack) Create the per-milestone planning gate-record fresh in the current task. create-prd (dev pack) Create a new prd in the current task. create-research (methodology pack) Create the per-investigation research record fresh in the current task. create-roadmap (methodology pack) Create-or-update the running roadmap singleton in the current task. create-spec (dev pack) Create a new spec in the current task. create-vision (methodology pack) Create-or-update the running vision singleton in the current task. finalize-task (dev pack) Run validate + commit. One task → one commit. finalize-task (methodology pack) Run validate + commit. One task → one commit. milestone-finalize (dev pack) The milestone commit boundary — validate the merged join + commit per the squash knob. milestone-join (dev pack) Merge the sub-tasks' staged docs by task-id order and report — commits nothing. milestone-provision (dev pack) Provision one detached base-pin worktree per sub-task before the fan-out. recompose-task (dev pack) Re-compose the task to pick up the freshly bound slice. run-ingest (dev pack) Scan the repo, classify candidate docs, and report the triage verdicts. set-commit-body (methodology pack) Stage the commit body slot from stdin. set-commit-scope (methodology pack) Set the commit header scope before finalize. set-commit-summary (dev pack) Stage the commit summary slot from stdin. set-commit-summary (methodology pack) Stage the commit summary slot from stdin. set-commit-type (dev pack) Set the required Conventional-Commits type. set-commit-type (methodology pack) Set the commit header type before finalize. show-doc (dev pack) Read a managed doc, or an addressed slice of it — with `--task`, the staged copy of your own in-flight write. show-doc (methodology pack) Read a managed doc back — the committed copy, or with `--task` the staged copy of your own in-flight write. validate-task (dev pack) Preview part of the finalize gate — content findings, carryover, owner-artifact, the granted-but-unused changelog gate — without committing.

— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    …and the door that printed the route now opens (describe exit 0)
  OK    …as does the schema read, at the frozen shape (options present)

=== B · THE DOCUMENTED CAPABILITY — a presentation-only shadow loads clean

$ jigc describe --doctypes
jigc describe — a tour of what this project lets you compose and author.

The doc-types you can author. adr is A house-worded ADR. Reach for it when a choice is worth preserving with its rationale, so a later reader can recover why the call was made or supersede it on the record.

arch-doc is Living architecture documentation for one part of the system — its overview, its components and the code that implements them, and the decisions behind it. Reach for it when a part of the system is worth a durable description that stays honest against the code, so a later reader (or agent) can orient without reverse-engineering it.

changelog is A Keep-a-Changelog singleton — staged unreleased changes plus the cut releases, each grouped by category, maintained over the life of the project. Reach for it when a user-facing change lands and the project keeps a human-readable record of what changed, staged now and cut into versioned releases over time.

commit is A Conventional-Commits message — a typed, scoped header over a subject line, an optional body, and trailers — rendered into the git commit rather than persisted as a repo file. Reach for it when you need to record what a change does and why at the moment it lands, in the form git and reviewers already read.

completion-record is The per-milestone completion record — the audit verdict, the owner-artifact the genuine audit produced, and each triaged finding with its disposition. Reach for it when a milestone reaches its completion audit, so its verdict, the audit's owner-artifact, and the triaged findings need a durable per-milestone record.

decisions-log is The running log of decisions made — each with the date it was settled and the reasoning behind it. Reach for it when the planning loop reaches Settle (or completion reaches triage) and a decision is made worth preserving with its rationale, dated, on the record.

deferral-ledger is The running ledger of deferred decisions and parked ideas, each keyed to the milestone trigger that resurfaces it. Reach for it when the planning loop reaches Settle and a decision is deferred or an idea parked rather than made now, so it needs a durable owed-and-when record.

dogfood-record is The per-run measured-dogfood record — the run's case and pinned binary, the transcribed organic fact counts, the seeded instrument checks, the judged verdict, and the owner-artifact holding the raw capture. Reach for it when a measured dogfood run completes, so its transcribed facts, seeded instrument checks, verdict, and capture artifact need a durable per-run record.

idea is One shaped-but-unscheduled direction, with the trigger that would bring it back. Reach for it when a direction is worth keeping but not worth scheduling now, so it needs a durable home cheaper than losing the thought and a note of when to revisit it.

milestone-record is The per-milestone team-ready state record — the shared base-SHA pin, the ordered sub-task list, and each sub-task's intent and status, resumable from a fresh clone. Reach for it when a milestone is executing and its in-flight state (base, sub-tasks, statuses) must be committed and legible so a teammate or fresh session can continue it, not stranded in the gitignored workbench.

planning-record is One milestone's plan-time gate record — every planning gate answered with the evidence that discharges it, or an explicit N/A and why. Reach for it when a milestone is being planned, and each plan-time gate needs an answer recorded before the plan is allowed to close.

prd is A product requirements document — the vision, the requirements, and the context for a piece of work, the first managed document a fresh project develops its idea into. Reach for it when you are at the start of a new project and want to turn an idea into a shared statement of what to build and why, before any spec or code.

research is One investigation and what it found — the evidence a vision or design is formed from. Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and that record should stay addressable by what it grounds.

roadmap is The running milestone spine — each milestone with what it proves and the prose decomposition of its increments. Reach for it when the planning loop reaches Decompose and a milestone's breakdown is worth recording on the durable roadmap.

spec is A specification of what a task implements — its goal, its context, and a set of testably-phrased acceptance criteria. Reach for it when you need to pin down what a piece of work must deliver before building it, so the criteria are agreed up front and code can be checked against them.

vision is The project's charter — its thesis, the invariants it holds, and its open questions — grounded in the research it was formed from. Reach for it when the project's direction is worth a durable, managed anchor that later work compares against, formed from and traceable to the research behind it.

— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    describe loads (exit 0)
  OK    …and shows the reworded description
  OK    doc schema adr loads at the frozen version, options intact
  OK    validate is clean
  OK    the create-gate write goes through
  OK    …and the staged doc reads back through --task
  OK    task validate runs (exit reflects content findings, not a pack-load fault)

=== C · THE DECLARED BOUND — setup over the shape-changing shadow, scored on the four-part standard
jigc setup — adapter installed

jigc is now wired into this project; setup installed:
  - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
  - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
  - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
  - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
    local to this checkout — git cannot track this path, so the hook is not in the install commit; a clone gets no drift backstop until `jigc setup` runs there
  - jigc guides → .claude/skills/jigc/SKILL.md   (jigc's own quickstart + migration notes, stamped with this build)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

  MEASURED · setup exit: 0
  MEASURED · four-part standard at the setup door:
             names what it objects to (adr)      : no
             says the consequence (next doors block): no
             names a route                        : no
             the route runs                       : n/a (no route to run)
             score: 0/4 — the declared bound, measured; VERDICT.md declares exactly this counter-case
  OK    the bound holds AS DECLARED: setup exits 0 over the shape-changing shadow
  OK    …while the very next door blocks
  OK    the shadow is cleared again via the printed route

=== D · A MIS-KEYED LEAF INSIDE repeatable: — refused at load, at BOTH layers

=== D1 · the pack layer — a listed house pack whose `memo` block carries `patern:`

$ jigc doc schema memo
the `memo` schema is malformed: malformed schema at `memo#items/title`: unknown field `patern`, expected one of `id`, `type`, `of`, `default`, `set`, `to`, `card`, `inverse`, `inverse-card`, `check`, `optional`, `title-names-symbol`
[exit 1]
  OK    doc schema memo refuses the load (exit non-zero)
  OK    …located at <type>#<section>/<leaf>
  OK    …naming the offending key
  OK    …and the section is NOT silently erased (no exit-0 answer without it)

$ jigc describe
the `memo` schema is malformed: malformed schema at `memo#items/title`: unknown field `patern`, expected one of `id`, `type`, `of`, `default`, `set`, `to`, `card`, `inverse`, `inverse-card`, `check`, `optional`, `title-names-symbol`
[exit 1]
  OK    the menu door refuses the same pack the same way
  OK    the pack list is restored and the menu opens again

=== D2 · the project layer — the same typo in a `commit` shadow (the rc.12 repro, on the shipped block)

$ jigc doc schema commit
the project schema shadow /work/.jigc/config/schemas/commit.yaml is malformed: malformed schema at `commit#trailers/key`: unknown field `patern`, expected one of `id`, `type`, `of`, `default`, `set`, `to`, `card`, `inverse`, `inverse-card`, `check`, `optional`, `title-names-symbol`
[exit 1]
  OK    doc schema commit refuses the load (exit non-zero)
  OK    …names the shadow file
  OK    …located at commit#trailers/key
  OK    …naming the offending key
  OK    …and trailers is NOT silently erased
  OK    everything restored: describe and validate open again

=== SUMMARY
  layers driven: project shadow (A/B/C, D2) · pack file via a listed house pack (D1) — the two the resolver reads
  the setup score in C is a measurement of a declared bound, not a verdict
ARM 13 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/13-freeze-every-layer (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 22
  doc show --task    : 1
  doc show (any)     : 1
  §3.3 adjacent      : 1   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/13-freeze-every-layer"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.

/tmp/arm.sh: line 246: patern: command not found
/tmp/arm.sh: line 284: patern: command not found
```

invocation log: 22 records · `doc show … --task` **1** · §3.3 adjacent 1

## `14-migrate-author-on-rc12.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/14-migrate-author-on-rc12
container  : 18a0927a30c591b55871f3e9bbcdcf2b3689c48e14297407611d87c18a8b6395
  (a mid-stream plant runs with: docker exec -it -u node 18a0927a30c591b55871f3e9bbcdcf2b3689c48e14297407611d87c18a8b6395 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/14-migrate-author-on-rc12.sh in the container
-------------------------------------------------------------

=== SKIPPED — this arm needs the OLD binary (rc.12), and this pass is not it

The migration pair is driven as an explicit three-command sequence:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc12 --only 14
  python3 run.py  carry <out-a>/14-migrate-author-on-rc12 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc13 --only 21

`run.py carry` between the halves is load-bearing: run-session.sh writes its own
evidence into the out-dir, and handing that straight on would plant the rig's
droppings in the corpus the second half reads.
ARM 14 SKIPPED (needs jigc-gate:rc12; got jigc 1.0.0-rc.13)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/14-migrate-author-on-rc12 (corpus + .session-transcript/ + PROVENANCE.txt)
  NO INVOCATION LOG — §3.3's primary channel is missing for this session
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

## `15-placement-root.sh`

exit **1**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/15-placement-root
container  : f8fa549dd32c3d7b8d89a7b86c603a7d9f7b70e6258714ad9686f578e7aae6af
  (a mid-stream plant runs with: docker exec -it -u node f8fa549dd32c3d7b8d89a7b86c603a7d9f7b70e6258714ad9686f578e7aae6af bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/15-placement-root.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== FIXTURE · two committed placement docs — a nested home (roadmap) and a root home (vision)

$ git ls-files docs VISION.md
VISION.md
docs/decisions-log.md
docs/roadmap.md
[exit 0]
  OK    roadmap is committed at its declared nested home
  OK    vision is committed at its declared ROOT home
  OK    no task is live — the knob doors run over a settled store

$ jigc config get placement-root
placement-root =   (pack-default)
[exit 0]

=== (a) · placement-root planning — the re-point MOVES the doc it would strand

$ jigc config set placement-root planning
relocating the committed doc(s) stranded by the `placement-root` re-point to `planning` (every committed doc at a placement doctype's prior home, managed or not; each move is a staged `git mv` — commit it with your next commit):
  - docs/decisions-log.md → planning/decisions-log.md
  - docs/roadmap.md → planning/roadmap.md
config: set `placement-root` = `planning` — written to `.jigc/config/`, uncommitted — commit it with your next commit
[exit 0]

$ git status --porcelain
 M .jigc/config/manifest.yaml
R  docs/decisions-log.md -> planning/decisions-log.md
R  docs/roadmap.md -> planning/roadmap.md
[exit 0]

$ git diff --cached --name-status
R100	docs/decisions-log.md	planning/decisions-log.md
R100	docs/roadmap.md	planning/roadmap.md
[exit 0]
  OK    the knob reads back as planning, from the project layer
  OK    the re-point names the move it made, per file
  OK    the index carries the move as a rename (R), not a delete + add
  OK    the file is at the new home and gone from the old

$ cat .jigc/state/file-state.json
{
  "hashes": {
    ".jigc/config/manifest.yaml": "7007bea4c005dcc6f80ad9d6520d9a2a4274ec86d9b6b1a185b9affa49f2a901",
    "VISION.md": "2143bda9e5a8e49e30ecc1254010c00d74a3026bc1b27e2b82faec77cf85558d",
    "planning/decisions-log.md": "8d783b6fbc4e7cd3624ff067ac4c0d36ccbd8812534442a6645c20f41e9f7330",
    "planning/roadmap.md": "63b76fb1047425bb2e83b9ebe5d07f60589fb70d41ccaf8725755356a6c86960"
  }
}
[exit 0]
  OK    file-state is re-keyed to planning/roadmap.md
  OK    …and no longer keys docs/roadmap.md

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · schema-conformance.repeatable-populated — `planning/decisions-log.md`: repeatable section `entries` parses zero items — structurally empty
  at: decisions-log:decisions-log#entries
  route: populate the section, or exempt `decisions-log#entries` via the `validation.schema-conformance.repeatable-populated.exempt` knob
1 finding(s) — report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    jigc validate exits 0 with no blocking row over the moved store

$ jigc doc show roadmap --format json
{
  "fields": {
    "schema-version": "1"
  },
  "item-count": 1,
  "schema-version": 1,
  "sections": {
    "milestones": [
      {
        "decomposition": "Increment 1 — the enumeration seam.",
        "id": "m-alpha",
        "proves": "That the composed loop lands one task end to end.",
        "title": "M-Alpha"
      }
    ]
  },
  "slug": "roadmap",
  "type": "roadmap"
}
[exit 0]
  OK    jigc doc show roadmap still reads the doc at its identity

$ jigc doc list roadmap
id  path  state
roadmap:roadmap  planning/roadmap.md  managed
[exit 0]
  OK    jigc doc list names the resolved path

=== (b) · the root-declared VISION.md is NOT re-rooted
  OK    VISION.md is still at the repo root
  OK    …and the index did not touch it
  OK    …and the re-point's own output never named it

=== (c) · placement-root "" canonicalizes to . and moves the doc to the repo root

$ jigc config set placement-root 
relocating the committed doc(s) stranded by the `placement-root` re-point to `.` (every committed doc at a placement doctype's prior home, managed or not; each move is a staged `git mv` — commit it with your next commit):
  - planning/decisions-log.md → decisions-log.md
  - planning/roadmap.md → roadmap.md
config: set `placement-root` = `.` — written to `.jigc/config/`, uncommitted — commit it with your next commit
[exit 0]

$ jigc config get placement-root
placement-root = .  (project)
[exit 0]
  OK    the empty string reads back as .

$ git diff --cached --name-status
R100	planning/decisions-log.md	decisions-log.md
R100	planning/roadmap.md	roadmap.md
[exit 0]
  OK    the doc moved to the repo root as a rename
  OK    VISION.md is still exactly where it was
  OK    jigc doc show roadmap reads the root-homed doc

=== (d) · placement-root .git — THE HIGH: refused, nothing moves, the doc stays tracked

$ jigc config set placement-root .git
blocking · config.untrackable-root — `.git` cannot be the `placement-root`: `.git` is inside git's own directory — git refuses to track any path with a `.git` component (`error: invalid path`), so the bytes would survive only in history
  route: re-run with a root git can record — a path under the repository root and outside `.git/`; `jigc config list` shows the value in force and the layer it wins from
[exit 1]
  OK    it refuses (non-zero)
  OK    …with a code
  OK    …that names git's own behaviour, so the reader knows WHY
  OK    …and a route

$ git status --porcelain
[exit 0]
  OK    the working tree is untouched — no move was attempted

$ git ls-files roadmap.md
roadmap.md
[exit 0]
  OK    the doc is still tracked at its home
  OK    …and nothing landed under .git/
  OK    the knob is unchanged

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · schema-conformance.repeatable-populated — `decisions-log.md`: repeatable section `entries` parses zero items — structurally empty
  at: decisions-log:decisions-log#entries
  route: populate the section, or exempt `decisions-log#entries` via the `validation.schema-conformance.repeatable-populated.exempt` knob
1 finding(s) — report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    jigc validate is still clean (exit 0, no blocking row)

$ jigc config set placement-root .git --format json
{
  "error": "blocking · config.untrackable-root — `.git` cannot be the `placement-root`: `.git` is inside git's own directory — git refuses to track any path with a `.git` component (`error: invalid path`), so the bytes would survive only in history\n  route: re-run with a root git can record — a path under the repository root and outside `.git/`; `jigc config list` shows the value in force and the layer it wins from"
}
[exit 1]
  OK    the JSON envelope carries the same code

$ jigc config set placement-root planning
relocating the committed doc(s) stranded by the `placement-root` re-point to `planning` (every committed doc at a placement doctype's prior home, managed or not; each move is a staged `git mv` — commit it with your next commit):
  - decisions-log.md → planning/decisions-log.md
  - roadmap.md → planning/roadmap.md
config: set `placement-root` = `planning` — written to `.jigc/config/`, uncommitted — commit it with your next commit
[exit 0]
  OK    the store is back at planning/ for the strand cells

=== (e) · a hand-git-mv of a placement doc to a wrong home, committed — NOT 'validates clean'

$ git mv planning/roadmap.md notes/roadmap.md
[exit 0]

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
blocking (gates at finalize) · reconciliation.rename — tracked managed doc roadmap:roadmap (planning/roadmap.md) is missing
  at: planning/roadmap.md
  route: restore planning/roadmap.md, or confirm the deletion by dropping it from the index: `jigc unmanage planning/roadmap.md`
advisory · schema-conformance.repeatable-populated — `planning/decisions-log.md`: repeatable section `entries` parses zero items — structurally empty
  at: decisions-log:decisions-log#entries
  route: populate the section, or exempt `decisions-log#entries` via the `validation.schema-conformance.repeatable-populated.exempt` knob
advisory · file-state.unregistered-doc — committed doc `notes/roadmap.md` looks managed (it carries `roadmap`'s home filename, whose resolved home is `planning/roadmap.md`) but was never adopted — a basename coincidence or an un-ingested foreign doc, not a tracked strand
  at: notes/roadmap.md
  route: adopt it with `jigc migrate notes/roadmap.md --as roadmap`, or ignore it if it is not meant to be managed
out-of-band rename detected — a structural-identity change this commit introduced; the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    the strand is reported — the sweep does NOT say the store validates clean
  OK    …and the report carries a route
  OK    …and the wrong-home copy is named
  MEASURED · codes the hand-move draws (the charter expected file-state.orphaned-doc; an
             out-of-band git mv is ALSO a reconciliation rename, which fires first):
             reconciliation.rename
             schema-conformance.repeatable-populated
             file-state.unregistered-doc
             exit: 1
  OK    the sweep exits non-zero over an out-of-band rename (M35's structural-identity rule)

=== (e') · the knob moved, the doc did not — file-state.orphaned-doc with its route

$ sed -i s/placement-root: planning/placement-root: notes/ .jigc/config/manifest.yaml
[exit 0]

$ jigc config get placement-root
placement-root = notes  (project)
[exit 0]

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · file-state.orphaned-doc — committed doc `planning/decisions-log.md` sits outside `decisions-log`'s resolved home `notes/decisions-log.md` — a `placement-root` change likely stranded it (the home moved, the committed instance did not)
  at: planning/decisions-log.md
  route: move it to `notes/decisions-log.md` and re-run `jigc ingest`, re-point `placement-root` to cover where it sits, or drop it with `jigc unmanage planning/decisions-log.md`
advisory · file-state.orphaned-doc — committed doc `planning/roadmap.md` sits outside `roadmap`'s resolved home `notes/roadmap.md` — a `placement-root` change likely stranded it (the home moved, the committed instance did not)
  at: planning/roadmap.md
  route: move it to `notes/roadmap.md` and re-run `jigc ingest`, re-point `placement-root` to cover where it sits, or drop it with `jigc unmanage planning/roadmap.md`
2 finding(s) — report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the stranded doc is reported as file-state.orphaned-doc
  OK    …naming the doc where it sits
  OK    …and the home it should be at
  OK    …with a route naming the three exits (move+ingest · re-point · unmanage)
  OK    VISION.md draws no strand row — a root home is never re-rooted

$ jigc config set placement-root planning
config: set `placement-root` = `planning` — written to `.jigc/config/`, uncommitted — commit it with your next commit
[exit 0]

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · schema-conformance.repeatable-populated — `planning/decisions-log.md`: repeatable section `entries` parses zero items — structurally empty
  at: decisions-log:decisions-log#entries
  route: populate the section, or exempt `decisions-log#entries` via the `validation.schema-conformance.repeatable-populated.exempt` knob
1 finding(s) — report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the store is clean again once knob and docs agree

=== (d') · placement-root .jigc — the workbench: driven and RECORDED, then asked to survive uninstall

$ jigc config set placement-root .jigc
relocating the committed doc(s) stranded by the `placement-root` re-point to `.jigc` (every committed doc at a placement doctype's prior home, managed or not; each move is a staged `git mv` — commit it with your next commit):
  - planning/decisions-log.md → .jigc/decisions-log.md
  - planning/roadmap.md → .jigc/roadmap.md
config: set `placement-root` = `.jigc` — written to `.jigc/config/`, uncommitted — commit it with your next commit
[exit 0]
placement-root = .jigc  (project)

$ git status --porcelain
 M .jigc/config/manifest.yaml
R  planning/decisions-log.md -> .jigc/decisions-log.md
R  planning/roadmap.md -> .jigc/roadmap.md
[exit 0]

$ git check-ignore -v .jigc/roadmap.md
[exit 1]
  OBSERVE  placement-root .jigc: placement-root = .jigc  (project); moved: 2
  OK    (recorded, not required) the doc is tracked inside the workbench
  OK    …and readable at its identity

$ jigc uninstall
jigc uninstall — repo-local install removed

  - removed .jigc/
  - unwired bootstrap reference ← CLAUDE.md
  - removed jigc allowlist permit ← .claude/settings.json
  - removed SessionStart hook ← .claude/settings.json
  - removed deny safety floor ← .claude/settings.json
  - removed pre-commit hook
  - removed jigc guide artifact
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ git status --porcelain
 M .claude/settings.json
 D .claude/skills/jigc/SKILL.md
 D .jigc/.gitignore
 D .jigc/AGENT.md
 D .jigc/config/.gitkeep
 D .jigc/config/manifest.yaml
 D .jigc/config/packs.yaml
 D .jigc/decisions-log.md
 D .jigc/roadmap.md
 D .jigc/version
 D CLAUDE.md
?? .jigc/logs/
[exit 0]
  FAIL  a COMMITTED doc homed under the workbench survives `jigc uninstall` on disk
  FAIL  …and in the index (no staged/unstaged deletion of it)
  FAIL  …or, failing that, uninstall at least REFUSED or NAMED the committed docs it took
  OBSERVE  if the bars above FAIL: `jigc uninstall` removed committed managed docs the knob
           had just accepted, at exit 0, naming neither — the .git refusal's sibling on
           the .jigc axis (recoverable from git here; the door's narration is the finding).

=== SUMMARY
  the placement-root door over {planning, "", .git, .jigc} + the two strand shapes;
  the eight mover sites are test-fenced (crates/cli/tests/placement_override.rs), not driven.
ARM 15 FAIL
(the arm exited 1 — that is data, not necessarily failure)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/15-placement-root (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 1
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/15-placement-root"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 1 records · `doc show … --task` **0** · §3.3 adjacent 0

## `16-pinned-contracts.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/16-pinned-contracts
container  : 4e1b80fb790b2d519c4fc8fae2c195c1ac413c9681a7095e9dab632427def5e5
  (a mid-stream plant runs with: docker exec -it -u node 4e1b80fb790b2d519c4fc8fae2c195c1ac413c9681a7095e9dab632427def5e5 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/16-pinned-contracts.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== mint · one record-change task, the changelog singleton and one release

$ jigc start --workflow record-change record the probe release
task minted: record-the-probe-release

Record the change on the project changelog. Create-or-update the changelog
singleton — safe whether or not it already exists (an existing committed changelog
is copied in for append):

Run: `jigc doc create changelog --title Changelog --task record-the-probe-release`

The `changelog` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema changelog

Then author the change. To cut a release, mint a release item (its id is minted
from the version title you give — e.g. `1.0.0` mints `1-0-0` — and its `date` is
stamped on create). `add-item` PRINTS the new item's address; use that printed
address verbatim for every follow-up verb — never re-spell the version string:

jigc doc add-item changelog:changelog#releases --title "<version>" --task record-the-probe-release
#   → prints e.g. changelog:changelog#releases/1-0-0  (call this <release-addr>)
jigc doc set-field <release-addr>/link --value "<diff-url>" --task record-the-probe-release

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot <group-addr>/notes --from-file - --task record-the-probe-release <<'EOF'
<one bullet per change>
EOF

Under that release, add one nested change-group per category and author its notes
(one bullet per change). The category is the group's id (added / changed /
deprecated / removed / fixed / security). Again, drive the printed address verbatim:

jigc doc add-item <release-addr>/changes --title "<category>" --task record-the-probe-release
#   → prints e.g. changelog:changelog#releases/1-0-0/changes/added  (call this <group-addr>)
jigc doc set-slot <group-addr>/notes --from-file - --task record-the-probe-release

For a change that is not yet cut into a release, add the change-group under the
staged section instead:

jigc doc add-item changelog:changelog#unreleased-changes --title "<category>" --task record-the-probe-release
#   → prints e.g. changelog:changelog#unreleased-changes/added  (call this <group-addr>)
jigc doc set-slot <group-addr>/notes --from-file - --task record-the-probe-release

Or make it one call — the batch alternative to the whole sequence above (run it
instead of the create + per-entry verbs, or after them — the create it implies
over a doc this task already staged acks `already existed — copied in for
update` and changes nothing): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write, no printed-address dance. Over an already-committed changelog it copies
the committed doc in and appends — existing releases are untouched; an entry whose
title mints an id the changelog already holds is refused (`write.already-present`),
the WHOLE payload rejected and nothing staged, so edit that entry in place with the
per-entry `set-field` / `set-slot` lines above rather than re-authoring it here:

jigc doc author changelog --from-file - --task record-the-probe-release

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show changelog:changelog --task record-the-probe-release

When done, set the required Conventional-Commits type — your editorial call on
what this change does. The subject renders as `<type>(<scope>): <summary>`, so
write the summary without a type or scope prefix of its own — the `type` field
already carries it. Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address.

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot commit:record-the-probe-release#summary --from-file - --task record-the-probe-release <<'EOF'
<one line saying what changed>
EOF

Set the type, then stage the summary prose:

Run: `jigc doc set-field commit:record-the-probe-release#type --value <COMMIT_TYPE> --task record-the-probe-release`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:record-the-probe-release#summary --from-file - --task record-the-probe-release`
<<author: commit:record-the-probe-release#summary>>

The `commit` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema commit

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:record-the-probe-release#scope --value <area> --task record-the-probe-release
jigc doc set-slot commit:record-the-probe-release#body --from-file - --task record-the-probe-release

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:record-the-probe-release#trailers --title Co-Authored-By --task record-the-probe-release
jigc doc set-field commit:record-the-probe-release#trailers/<id>/value --value "Name <email>" --task record-the-probe-release

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show commit:record-the-probe-release --task record-the-probe-release

Validate and commit the task as one logical commit. Finalize commits only the
staged set plus the docs it manages; unstaged edits and untracked files are left
out, and with nothing staged over a dirty tree it refuses. Anything still staged
from BEFORE this task was minted makes finalize refuse too (one blocking finding
per carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate record-the-probe-release` — it
previews part of what finalize gates on (this task's content findings, the
carryover gate, the owner-artifact causes that need no staging, and the
granted-but-unused changelog gate), without committing anything; the staged set,
promotion and the commit itself are decided at finalize.

Run: `jigc task finalize record-the-probe-release`
resume: `jigc start --task record-the-probe-release`   — re-composes this workflow if context is lost
what's-left: `jigc task validate record-the-probe-release`   — previews part of the finalize gate: this task's content findings, the carryover gate, the owner-artifact causes that need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the commit surface at finalize
task scope: `jigc doc` writes default to the single active task; `--task record-the-probe-release` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; once a sibling task commits a path this one also touches, resuming or finalizing here blocks and names the overlapping paths
create-gates: changelog   — the doc-types this task is allowed to create; any other type is refused
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  T=record-the-probe-release

$ jigc doc create changelog --title Changelog --task record-the-probe-release
changelog:changelog
[exit 0]

$ jigc doc add-item changelog:changelog#releases --title 1.0.0 --task record-the-probe-release
changelog:changelog#releases/1-0-0
[exit 0]
  REL=changelog:changelog#releases/1-0-0
  OK    the release address the binary emitted is the one this arm hops from

=== N1 · one nested-section hop, ONE code, at every item-addressing write door

$ jigc doc set-slot changelog:changelog#releases/1-0-0/bogus/xyz --from-file /tmp/arm16.MX6L5N/prose.md --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "unknown-section",
      "code": "write.unknown-section",
      "key": {
        "code": "write.unknown-section",
        "target": "changelog:changelog#releases/1-0-0/bogus/xyz"
      },
      "message": "write rejected: no section \"releases/1-0-0/bogus\" declared in the schema",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/bogus/xyz",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc schema changelog` to see the declared shape, then re-run the write at a declared address"
    }
  ]
}[exit 1]
  OK    set-slot · blocks
  OK    set-slot · key.code is write.unknown-section
  OK    set-slot · key.target is the address typed, verbatim
  OK    set-slot · the route names jigc doc schema changelog

$ jigc doc set-field changelog:changelog#releases/1-0-0/bogus/xyz --value v --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "unknown-section",
      "code": "write.unknown-section",
      "key": {
        "code": "write.unknown-section",
        "target": "changelog:changelog#releases/1-0-0/bogus/xyz"
      },
      "message": "write rejected: no section \"releases/1-0-0/bogus\" declared in the schema",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/bogus/xyz",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc schema changelog` to see the declared shape, then re-run the write at a declared address"
    }
  ]
}[exit 1]
  OK    set-field--value · blocks
  OK    set-field--value · key.code is write.unknown-section
  OK    set-field--value · key.target is the address typed, verbatim
  OK    set-field--value · the route names jigc doc schema changelog

$ jigc doc set-field changelog:changelog#releases/1-0-0/bogus/xyz --unset --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "unknown-section",
      "code": "write.unknown-section",
      "key": {
        "code": "write.unknown-section",
        "target": "changelog:changelog#releases/1-0-0/bogus/xyz"
      },
      "message": "write rejected: no section \"releases/1-0-0/bogus\" declared in the schema",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/bogus/xyz",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc schema changelog` to see the declared shape, then re-run the write at a declared address"
    }
  ]
}[exit 1]
  OK    set-field--unset · blocks
  OK    set-field--unset · key.code is write.unknown-section
  OK    set-field--unset · key.target is the address typed, verbatim
  OK    set-field--unset · the route names jigc doc schema changelog

$ jigc doc remove-item changelog:changelog#releases/1-0-0/bogus/xyz --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "unknown-section",
      "code": "write.unknown-section",
      "key": {
        "code": "write.unknown-section",
        "target": "changelog:changelog#releases/1-0-0/bogus/xyz"
      },
      "message": "write rejected: no section \"releases/1-0-0/bogus\" declared in the schema",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/bogus/xyz",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc schema changelog` to see the declared shape, then re-run the write at a declared address"
    }
  ]
}[exit 1]
  OK    remove-item · blocks
  OK    remove-item · key.code is write.unknown-section
  OK    remove-item · key.target is the address typed, verbatim
  OK    remove-item · the route names jigc doc schema changelog

$ jigc doc retitle-item changelog:changelog#releases/1-0-0/bogus/xyz --title Y --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "unknown-section",
      "code": "write.unknown-section",
      "key": {
        "code": "write.unknown-section",
        "target": "changelog:changelog#releases/1-0-0/bogus/xyz"
      },
      "message": "write rejected: no section \"releases/1-0-0/bogus\" declared in the schema",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/bogus/xyz",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc schema changelog` to see the declared shape, then re-run the write at a declared address"
    }
  ]
}[exit 1]
  OK    retitle-item · blocks
  OK    retitle-item · key.code is write.unknown-section
  OK    retitle-item · key.target is the address typed, verbatim
  OK    retitle-item · the route names jigc doc schema changelog

$ jigc doc add-item changelog:changelog#releases/1-0-0/bogus --title X --task record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "blocking",
      "probe": "write",
      "check": "unknown-section",
      "code": "write.unknown-section",
      "key": {
        "code": "write.unknown-section",
        "target": "changelog:changelog#releases/1-0-0/bogus"
      },
      "message": "write rejected: no section \"releases/1-0-0/bogus\" declared in the schema",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/bogus",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc schema changelog` to see the declared shape, then re-run the write at a declared address"
    }
  ]
}[exit 1]
  OK    add-item · blocks
  OK    add-item · key.code is write.unknown-section
  OK    add-item · key.target is the address typed, verbatim
  OK    add-item · the route names jigc doc schema changelog

  MEASURED · distinct finding codes across the six doors (rc.12: 4; the contract: 1):
           1 → write.unknown-section
  OK    one code over the whole set

=== N2 · the doc's own stamp as a top-level integer — staged

$ jigc doc show changelog:changelog --task record-the-probe-release --format json
{
  "fields": {
    "schema-version": "2"
  },
  "item-count": 1,
  "schema-version": 2,
  "sections": {
    "releases": [
      {
        "changes": [],
        "date": "2026-09-04",
        "id": "1-0-0",
        "title": "1.0.0"
      }
    ],
    "unreleased-changes": []
  },
  "slug": "changelog",
  "staged": "record-the-probe-release",
  "type": "changelog"
}
[exit 0]
  OK    schema-version is a top-level INTEGER on the staged serve
  OK    …equal to the string in fields, which stays all-strings
  OK    …and the serve says which task staged it

$ jigc doc schema changelog --format json
{
  "contract-version": 5,
  "type": "changelog",
  "schema-version": 2,
  "fields": [
    {
      "id": "schema-version",
      "type": "int",
      "required": true,
      "author-required": false,
      "set": "schema-version",
      "section": "meta"
    }
  ],
  "sections": [
    {
      "id": "unreleased-changes",
      "kind": "repeatable",
      "add-item": "changelog:<slug>#unreleased-changes",
      "item": {
        "fields": [
          {
            "id": "category",
            "type": "enum",
            "of": [
              "added",
              "changed",
              "deprecated",
              "removed",
              "fixed",
              "security"
            ],
            "required": true,
            "author-required": true,
            "add-item": "changelog:<slug>#unreleased-changes",
            "write-key": "--title"
          }
        ],
        "slots": [
          {
            "id": "notes",
            "set-slot": "changelog:<slug>#unreleased-changes/<id>/notes"
          }
        ],
        "nested": []
      }
    },
    {
      "id": "releases",
      "kind": "repeatable",
      "add-item": "changelog:<slug>#releases",
      "item": {
        "fields": [
          {
            "id": "title",
            "type": "string",
            "required": true,
            "author-required": true,
            "add-item": "changelog:<slug>#releases",
            "retitle-item": "changelog:<slug>#releases/<id>",
            "write-key": "--title"
          },
          {
            "id": "date",
            "type": "date",
            "required": true,
            "author-required": false,
            "set": "on-create",
            "set-field": "changelog:<slug>#releases/<id>/date"
          },
          {
            "id": "link",
            "type": "string",
            "required": false,
            "author-required": false,
            "set-field": "changelog:<slug>#releases/<id>/link"
          }
        ],
        "slots": [],
        "nested": [
          {
            "id": "changes",
            "add-item": "changelog:<slug>#releases/<id>/changes",
            "item": {
              "fields": [
                {
                  "id": "category",
                  "type": "enum",
                  "of": [
                    "added",
                    "changed",
                    "deprecated",
                    "removed",
                    "fixed",
                    "security"
                  ],
                  "required": true,
                  "author-required": true,
                  "add-item": "changelog:<slug>#releases/<id>/changes",
                  "write-key": "--title"
                }
              ],
              "slots": [
                {
                  "id": "notes",
                  "set-slot": "changelog:<slug>#releases/<id>/changes/<id>/notes"
                }
              ],
              "nested": []
            }
          }
        ]
      }
    }
  ]
}
[exit 0]
  OK    …and equal to doc schema's number — the upgrade check a driver automates needs no cast

$ jigc doc show commit:record-the-probe-release --task record-the-probe-release --format json
{
  "fields": {
    "scope": "",
    "type": ""
  },
  "item-count": 0,
  "schema-version": null,
  "sections": {
    "body": "",
    "summary": "",
    "trailers": []
  },
  "slug": "record-the-probe-release",
  "staged": "record-the-probe-release",
  "type": "commit"
}
[exit 0]
  OK    the transient commit doc serves schema-version: null — the key present, the value null

$ jigc doc show changelog:changelog#releases --task record-the-probe-release --format json
[
  {
    "changes": [],
    "date": "2026-09-04",
    "id": "1-0-0",
    "title": "1.0.0"
  }
]
[exit 0]
  OK    a #section slice is the section's value (an item array) and gains no stamp

$ jigc doc list --task record-the-probe-release --format json
{
  "docs": [
    {
      "id": "changelog:changelog",
      "path": "CHANGELOG.md",
      "state": "managed",
      "item-count": 1
    },
    {
      "id": "commit:record-the-probe-release",
      "path": "commit:record-the-probe-release",
      "state": "managed",
      "item-count": 0
    }
  ]
}
[exit 0]
  OK    doc list carries no schema-version key anywhere

=== D1 · a located finding says WHERE — address and line — on the agent-text surface

$ grep -n bogusfield .jigc/tasks/record-the-probe-release/docs/changelog:changelog.md
15:- bogusfield: x
[exit 0]

$ jigc task validate record-the-probe-release
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` — this task's in-flight version of the doc
  at: CHANGELOG.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
blocking · conformance.unknown-field — `CHANGELOG.md`: unknown field key `bogusfield`
  at: changelog:changelog#releases/1-0-0/bogusfield · line 15
blocking · schema-conformance.field-value-conformant — `commit:record-the-probe-release`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  at: commit:record-the-probe-release#header/type
  route: `jigc doc set-field commit:record-the-probe-release#header/type --task record-the-probe-release --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:record-the-probe-release`: required slot in section `summary` is empty
  at: commit:record-the-probe-release#summary · line 9
  route: `jigc doc set-slot commit:record-the-probe-release#summary --task record-the-probe-release --from-file -` to fill the empty slot
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 3]
  OK    task validate blocks
  OK    the parse diagnostic conformance.unknown-field reaches the agent text
  OK    …carrying the address it located
  OK    …and the line it read it at

$ jigc task validate record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "advisory",
      "probe": "file-state",
      "check": "staged-copy",
      "code": "file-state.staged-copy",
      "key": {
        "code": "file-state.staged-copy",
        "target": "CHANGELOG.md"
      },
      "message": "staged copy of `CHANGELOG.md` — this task's in-flight version of the doc",
      "location": {
        "address": "CHANGELOG.md",
        "line": 1,
        "col": 1
      },
      "route": "no action needed — the staged copy is validated in-task and baselined when its finalize lands"
    },
    {
      "severity": "blocking",
      "probe": "conformance",
      "check": "unknown-field",
      "code": "conformance.unknown-field",
      "key": {
        "code": "conformance.unknown-field",
        "target": "changelog:changelog#releases/1-0-0/bogusfield"
      },
      "message": "`CHANGELOG.md`: unknown field key `bogusfield`",
      "location": {
        "address": "changelog:changelog#releases/1-0-0/bogusfield",
        "line": 15,
        "col": 1
      },
      "route": null
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "field-value-conformant",
      "code": "schema-conformance.field-value-conformant",
      "key": {
        "code": "schema-conformance.field-value-conformant",
        "target": "commit:record-the-probe-release#header/type"
      },
      "message": "`commit:record-the-probe-release`: field `type` in section `header`: \"\" is not a member of enum \"type\" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)",
      "location": {
        "address": "commit:record-the-probe-release#header/type",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc set-field commit:record-the-probe-release#header/type --task record-the-probe-release --value <value>` to correct the value"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "required-slot-present",
      "code": "schema-conformance.required-slot-present",
      "key": {
        "code": "schema-conformance.required-slot-present",
        "target": "commit:record-the-probe-release#summary"
      },
      "message": "`commit:record-the-probe-release`: required slot in section `summary` is empty",
      "location": {
        "address": "commit:record-the-probe-release#summary",
        "line": 9,
        "col": 1
      },
      "route": "`jigc doc set-slot commit:record-the-probe-release#summary --task record-the-probe-release --from-file -` to fill the empty slot"
    }
  ]
}[exit 3]
  OK    …and in the envelope that exempt finding carries route: null — the located message IS the repair

$ jigc task validate record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "advisory",
      "probe": "file-state",
      "check": "staged-copy",
      "code": "file-state.staged-copy",
      "key": {
        "code": "file-state.staged-copy",
        "target": "CHANGELOG.md"
      },
      "message": "staged copy of `CHANGELOG.md` — this task's in-flight version of the doc",
      "location": {
        "address": "CHANGELOG.md",
        "line": 1,
        "col": 1
      },
      "route": "no action needed — the staged copy is validated in-task and baselined when its finalize lands"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "field-value-conformant",
      "code": "schema-conformance.field-value-conformant",
      "key": {
        "code": "schema-conformance.field-value-conformant",
        "target": "commit:record-the-probe-release#header/type"
      },
      "message": "`commit:record-the-probe-release`: field `type` in section `header`: \"\" is not a member of enum \"type\" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)",
      "location": {
        "address": "commit:record-the-probe-release#header/type",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc set-field commit:record-the-probe-release#header/type --task record-the-probe-release --value <value>` to correct the value"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "required-slot-present",
      "code": "schema-conformance.required-slot-present",
      "key": {
        "code": "schema-conformance.required-slot-present",
        "target": "commit:record-the-probe-release#summary"
      },
      "message": "`commit:record-the-probe-release`: required slot in section `summary` is empty",
      "location": {
        "address": "commit:record-the-probe-release#summary",
        "line": 9,
        "col": 1
      },
      "route": "`jigc doc set-slot commit:record-the-probe-release#summary --task record-the-probe-release --from-file -` to fill the empty slot"
    }
  ]
}[exit 3]
  OK    restoring the bytes clears it

=== the gate route · with TWO open tasks, a required-slot-present route carries --task
  T2=second-open-task

$ jigc task list
jigc task list — 2 active task(s)

  record-the-probe-release  [record-change]  record the probe release
  second-open-task  [record-decision]  a second open task
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    two tasks are open — the single-active-task default can no longer resolve

$ jigc task validate record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "advisory",
      "probe": "file-state",
      "check": "staged-copy",
      "code": "file-state.staged-copy",
      "key": {
        "code": "file-state.staged-copy",
        "target": "CHANGELOG.md"
      },
      "message": "staged copy of `CHANGELOG.md` — this task's in-flight version of the doc",
      "location": {
        "address": "CHANGELOG.md",
        "line": 1,
        "col": 1
      },
      "route": "no action needed — the staged copy is validated in-task and baselined when its finalize lands"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "field-value-conformant",
      "code": "schema-conformance.field-value-conformant",
      "key": {
        "code": "schema-conformance.field-value-conformant",
        "target": "commit:record-the-probe-release#header/type"
      },
      "message": "`commit:record-the-probe-release`: field `type` in section `header`: \"\" is not a member of enum \"type\" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)",
      "location": {
        "address": "commit:record-the-probe-release#header/type",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc set-field commit:record-the-probe-release#header/type --task record-the-probe-release --value <value>` to correct the value"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "required-slot-present",
      "code": "schema-conformance.required-slot-present",
      "key": {
        "code": "schema-conformance.required-slot-present",
        "target": "commit:record-the-probe-release#summary"
      },
      "message": "`commit:record-the-probe-release`: required slot in section `summary` is empty",
      "location": {
        "address": "commit:record-the-probe-release#summary",
        "line": 9,
        "col": 1
      },
      "route": "`jigc doc set-slot commit:record-the-probe-release#summary --task record-the-probe-release --from-file -` to fill the empty slot"
    }
  ]
}[exit 3]
  OK    task validate on the first task carries required-slot-present on its commit summary
  ROUTE=jigc doc set-slot commit:record-the-probe-release#summary --task record-the-probe-release --from-file -
  OK    …the route is a jigc doc set-slot at that address
  OK    …and it carries --task record-the-probe-release

$ sh -c printf 'record the probe release\n' | jigc doc set-slot commit:record-the-probe-release#summary --task record-the-probe-release --from-file -
set slot commit:record-the-probe-release#summary (25 chars)
[exit 0]
  OK    run VERBATIM (prose on stdin, as its --from-file - asks) it succeeds

$ jigc task validate record-the-probe-release --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "advisory",
      "probe": "file-state",
      "check": "staged-copy",
      "code": "file-state.staged-copy",
      "key": {
        "code": "file-state.staged-copy",
        "target": "CHANGELOG.md"
      },
      "message": "staged copy of `CHANGELOG.md` — this task's in-flight version of the doc",
      "location": {
        "address": "CHANGELOG.md",
        "line": 1,
        "col": 1
      },
      "route": "no action needed — the staged copy is validated in-task and baselined when its finalize lands"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "field-value-conformant",
      "code": "schema-conformance.field-value-conformant",
      "key": {
        "code": "schema-conformance.field-value-conformant",
        "target": "commit:record-the-probe-release#header/type"
      },
      "message": "`commit:record-the-probe-release`: field `type` in section `header`: \"\" is not a member of enum \"type\" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)",
      "location": {
        "address": "commit:record-the-probe-release#header/type",
        "line": 1,
        "col": 1
      },
      "route": "`jigc doc set-field commit:record-the-probe-release#header/type --task record-the-probe-release --value <value>` to correct the value"
    }
  ]
}[exit 3]
  OK    …and the finding is gone at that target

=== N2 · committed — the same integer after finalize

$ jigc task finalize record-the-probe-release
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` — this task's in-flight version of the doc
  at: CHANGELOG.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized a458b9a — docs(changelog): record the probe release
  added .jigc/config/manifest.yaml
  promoted CHANGELOG.md
  2 files committed
[exit 0]
  OK    the finalize lands, promoting CHANGELOG.md

$ jigc doc show changelog:changelog --format json
{
  "fields": {
    "schema-version": "2"
  },
  "item-count": 1,
  "schema-version": 2,
  "sections": {
    "releases": [
      {
        "changes": [],
        "date": "2026-09-04",
        "id": "1-0-0",
        "title": "1.0.0"
      }
    ],
    "unreleased-changes": []
  },
  "slug": "changelog",
  "type": "changelog"
}
[exit 0]
  OK    the committed serve carries the integer too, and no staged key
  OK    …the same number the staged serve carried

$ jigc doc list --format json   (stdout → list2.json; stderr shown below)
{
  "docs": [
    {
      "id": "changelog:changelog",
      "path": "CHANGELOG.md",
      "state": "managed",
      "item-count": 1
    }
  ]
}
note: docs are also staged in open task second-open-task — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list --task second-open-task`
[exit 0]
  OK    the committed doc list has no schema-version key either

=== SUMMARY
  six of DOCTYPE_DOORS' Address rows driven over the nested-section miss (the write
  doors); the rest are the unknown-doctype axis, fenced elsewhere. N2 driven staged
  and committed; D1 on a planted parse diagnostic; the gate route run verbatim with
  two tasks open.
ARM 16 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/16-pinned-contracts (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 31
  doc show --task    : 3
  doc show (any)     : 4
  §3.3 adjacent      : 6   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/16-pinned-contracts"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 31 records · `doc show … --task` **3** · §3.3 adjacent 6

## `17-empty-id-axis.sh`

exit **1**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/17-empty-id-axis
container  : 324f61dc44d85e479ff202f60b9fa8b51e99f83b7344a00912c63827ebf5ec31
  (a mid-stream plant runs with: docker exec -it -u node 324f61dc44d85e479ff202f60b9fa8b51e99f83b7344a00912c63827ebf5ec31 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/17-empty-id-axis.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== FIXTURE · one live task holding a staged doc, so destruction is measurable

$ jigc task list
jigc task list — 1 active task(s)

  keep-one-task-live  [record-decision]  keep one task live
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ jigc doc list --task keep-one-task-live
id  path  state
adr:keep-the-oldest-sample  docs/decisions/keep-the-oldest-sample.md  managed
commit:keep-one-task-live  commit:keep-one-task-live  managed
[exit 0]
  OK    the live task exists
  OK    …and stages a doc

=== COLUMN · id=[no-such-task]

$ jigc task validate no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] task validate · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] task validate · the refusal carries a code AND a route

$ jigc task diff no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] task diff · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] task diff · the refusal carries a code AND a route

$ jigc task finalize no-such-task --dry-run
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] task finalize --dry-run · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] task finalize --dry-run · the refusal carries a code AND a route

$ jigc task bind decision adr:nothing no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] task bind · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] task bind · the refusal carries a code AND a route

$ jigc start --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] start --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] start --task · the refusal carries a code AND a route

$ jigc workflow sub-task --task no-such-task
no task `no-such-task` — list a milestone's sub-tasks with `jigc milestone list-tasks <milestone-id>`
[exit 1]
  OK    [nonexistent] workflow --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] workflow --task · the refusal carries a code AND a route

$ jigc doc show adr:keep-the-oldest-sample --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc show --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc show --task · the refusal carries a code AND a route

$ jigc doc list --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc list --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc list --task · the refusal carries a code AND a route

$ jigc doc create adr --title Probe --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc create --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc create --task · the refusal carries a code AND a route

$ jigc doc add-item spec:nothing#criteria --title Probe --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc add-item --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc add-item --task · the refusal carries a code AND a route

$ jigc doc remove-item spec:nothing#criteria/probe --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc remove-item --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc remove-item --task · the refusal carries a code AND a route

$ jigc doc retitle-item spec:nothing#criteria/probe --title Probe --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc retitle-item --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc retitle-item --task · the refusal carries a code AND a route

$ jigc doc rename adr:keep-the-oldest-sample --to Probe --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc rename --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc rename --task · the refusal carries a code AND a route

$ jigc doc set-field adr:keep-the-oldest-sample#header/status --value accepted --task no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc set-field --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc set-field --task · the refusal carries a code AND a route

$ sh -c echo x | jigc doc set-slot 'adr:keep-the-oldest-sample#context' --from-file - --task 'no-such-task'
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc set-slot --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc set-slot --task · the refusal carries a code AND a route

$ sh -c printf 'title: Probe\nsections: []\n' | jigc doc author adr --from-file - --task 'no-such-task'
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] doc author --task · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] doc author --task · the refusal carries a code AND a route

$ jigc milestone execute no-such-task
milestone `no-such-task` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [nonexistent] milestone execute · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] milestone execute · the refusal carries a code AND a route

$ jigc milestone provision no-such-task
milestone `no-such-task` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [nonexistent] milestone provision · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] milestone provision · the refusal carries a code AND a route

$ jigc milestone join no-such-task
blocking · milestone.unknown — milestone `no-such-task` does not exist
  at: milestone:no-such-task
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [nonexistent] milestone join · refuses: no success ack, no panic (exit 1)
  OK    [nonexistent] milestone join · the refusal carries a code AND a route

$ jigc milestone finalize no-such-task
milestone `no-such-task` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [nonexistent] milestone finalize · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] milestone finalize · the refusal carries a code AND a route

$ jigc milestone list-tasks no-such-task
milestone `no-such-task` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [nonexistent] milestone list-tasks · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] milestone list-tasks · the refusal carries a code AND a route

$ jigc milestone add-task no-such-task some intent
blocking · milestone.unknown — milestone `no-such-task` does not exist
  at: milestone:no-such-task
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [nonexistent] milestone add-task · refuses: no success ack, no panic (exit 1)
  OK    [nonexistent] milestone add-task · the refusal carries a code AND a route

$ jigc milestone add-from-spec no-such-task spec:nothing
blocking · milestone.unknown — milestone `no-such-task` does not exist
  at: milestone:no-such-task
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [nonexistent] milestone add-from-spec · refuses: no success ack, no panic (exit 1)
  OK    [nonexistent] milestone add-from-spec · the refusal carries a code AND a route

$ jigc milestone discard no-such-task
milestone `no-such-task` does not exist
  route: check the milestone id (`jigc milestone list-tasks <milestone-id>` names a live milestone's sub-tasks); nothing was discarded
[exit 1]
  OK    [nonexistent] milestone discard · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] milestone discard · the refusal carries a code AND a route

$ jigc task discard no-such-task
no task `no-such-task` — list live tasks with `jigc task list`
[exit 1]
  OK    [nonexistent] task discard · refuses: no success ack, no panic (exit 1)
  FAIL  [nonexistent] task discard · the refusal carries a code AND a route

=== AFTER the column · did the live task survive?

$ jigc task list
jigc task list — 1 active task(s)

  keep-one-task-live  [record-decision]  keep one task live
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ ls -la .jigc/tasks
total 12
drwxr-xr-x 3 node node 4096 Sep  4 18:28 .
drwxr-xr-x 6 node node 4096 Sep  4 18:28 ..
drwxr-xr-x 3 node node 4096 Sep  4 18:28 keep-one-task-live
[exit 0]
  OK    [nonexistent] the live task `keep-one-task-live` is still listed
  OK    [nonexistent] …its working area is still on disk
  OK    [nonexistent] …and its staged doc is still readable
  OBSERVE  if the three bars above FAIL in the [nonexistent] column: the last door of the column,
           `jigc task discard 'no-such-task'`, took the LIVE task's working area with it — `.jigc/tasks/`
           joined to an empty id is the tasks directory itself — and acked at exit 0.
           That is loss of staged prose no commit holds, on a non-destructive-looking path.

=== COLUMN · id=[]

$ jigc task validate 
no findings — the task validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  FAIL  [empty] task validate · refuses: no success ack, no panic (exit 0)
  FAIL  [empty] task validate · the refusal carries a code AND a route

$ jigc task diff 
could not read the base pin at "/work/.jigc/tasks/base.json": No such file or directory (os error 2)
[exit 1]
  OK    [empty] task diff · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] task diff · the refusal carries a code AND a route

$ jigc task finalize  --dry-run
could not read the base pin at "/work/.jigc/tasks/base.json": No such file or directory (os error 2)
[exit 1]
  OK    [empty] task finalize --dry-run · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] task finalize --dry-run · the refusal carries a code AND a route

$ jigc task bind decision adr:nothing 
task at "/work/.jigc/tasks/" has no recorded workflow — discard it and re-start with `jigc start`
[exit 1]
  OK    [empty] task bind · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] task bind · the refusal carries a code AND a route

$ jigc start --task 
could not read the base pin for task ``: No such file or directory (os error 2)
[exit 1]
  OK    [empty] start --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] start --task · the refusal carries a code AND a route

$ jigc workflow sub-task --task 
could not read the base pin for task ``: No such file or directory (os error 2)
[exit 1]
  OK    [empty] workflow --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] workflow --task · the refusal carries a code AND a route

$ jigc doc show adr:keep-the-oldest-sample --task 
blocking · store.not-staged — `adr:keep-the-oldest-sample` is not staged in this task and has no committed copy — nothing to read yet
  at: adr:keep-the-oldest-sample
  route: create or author the doc in this task first — a staged copy exists only after a write
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    [empty] doc show --task · refuses: no success ack, no panic (exit 1)
  OK    [empty] doc show --task · the refusal carries a code AND a route

$ jigc doc list --task 
jigc doc list — no docs staged in task 
[exit 0]
  FAIL  [empty] doc list --task · refuses: no success ack, no panic (exit 0)
  FAIL  [empty] doc list --task · the refusal carries a code AND a route

$ jigc doc create adr --title Probe --task 
the active task has no recorded workflow — discard it with `jigc task discard ` and re-start with `jigc start`
[exit 1]
  OK    [empty] doc create --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] doc create --task · the refusal carries a code AND a route

$ jigc doc add-item spec:nothing#criteria --title Probe --task 
no staged instance for `spec:nothing#criteria` — provision it first (`jigc start` / `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` derives the id from the title (`X` → slug), not the task id — address writes at that title-derived id
[exit 1]
  OK    [empty] doc add-item --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] doc add-item --task · the refusal carries a code AND a route

$ jigc doc remove-item spec:nothing#criteria/probe --task 
no staged instance for `spec:nothing#criteria/probe` — provision it first (`jigc start` / `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` derives the id from the title (`X` → slug), not the task id — address writes at that title-derived id
[exit 1]
  OK    [empty] doc remove-item --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] doc remove-item --task · the refusal carries a code AND a route

$ jigc doc retitle-item spec:nothing#criteria/probe --title Probe --task 
no staged instance for `spec:nothing#criteria/probe` — provision it first (`jigc start` / `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` derives the id from the title (`X` → slug), not the task id — address writes at that title-derived id
[exit 1]
  OK    [empty] doc retitle-item --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] doc retitle-item --task · the refusal carries a code AND a route

$ jigc doc rename adr:keep-the-oldest-sample --to Probe --task 
no staged instance for `adr:keep-the-oldest-sample` — provision it first (`jigc start` / `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` derives the id from the title (`X` → slug), not the task id — address writes at that title-derived id
[exit 1]
  OK    [empty] doc rename --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] doc rename --task · the refusal carries a code AND a route

$ jigc doc set-field adr:keep-the-oldest-sample#header/status --value accepted --task 
blocking · write.unknown-section — write rejected: no section "header" declared in the schema
  at: adr:keep-the-oldest-sample#header/status
  route: `jigc doc schema adr` to see the declared shape, then re-run the write at a declared address
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    [empty] doc set-field --task · refuses: no success ack, no panic (exit 1)
  OK    [empty] doc set-field --task · the refusal carries a code AND a route

$ sh -c echo x | jigc doc set-slot 'adr:keep-the-oldest-sample#context' --from-file - --task ''
no staged instance for `adr:keep-the-oldest-sample#context` — provision it first (`jigc start` / `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` derives the id from the title (`X` → slug), not the task id — address writes at that title-derived id
[exit 1]
  OK    [empty] doc set-slot --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] doc set-slot --task · the refusal carries a code AND a route

$ sh -c printf 'title: Probe\nsections: []\n' | jigc doc author adr --from-file - --task ''
the active task has no recorded workflow — discard it with `jigc task discard ` and re-start with `jigc start`
[exit 1]
  OK    [empty] doc author --task · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] doc author --task · the refusal carries a code AND a route

$ jigc milestone execute 
milestone `` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [empty] milestone execute · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] milestone execute · the refusal carries a code AND a route

$ jigc milestone provision 
milestone `` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [empty] milestone provision · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] milestone provision · the refusal carries a code AND a route

$ jigc milestone join 
blocking · milestone.unknown — milestone `` does not exist
  at: milestone:
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [empty] milestone join · refuses: no success ack, no panic (exit 1)
  OK    [empty] milestone join · the refusal carries a code AND a route

$ jigc milestone finalize 
milestone `` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [empty] milestone finalize · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] milestone finalize · the refusal carries a code AND a route

$ jigc milestone list-tasks 
milestone `` does not exist
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [empty] milestone list-tasks · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] milestone list-tasks · the refusal carries a code AND a route

$ jigc milestone add-task  some intent
blocking · milestone.unknown — milestone `` does not exist
  at: milestone:
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [empty] milestone add-task · refuses: no success ack, no panic (exit 1)
  OK    [empty] milestone add-task · the refusal carries a code AND a route

$ jigc milestone add-from-spec  spec:nothing
blocking · milestone.unknown — milestone `` does not exist
  at: milestone:
  route: create it first with `jigc milestone create "<title>"`
[exit 1]
  OK    [empty] milestone add-from-spec · refuses: no success ack, no panic (exit 1)
  OK    [empty] milestone add-from-spec · the refusal carries a code AND a route

$ jigc milestone discard 
milestone `` does not exist
  route: check the milestone id (`jigc milestone list-tasks <milestone-id>` names a live milestone's sub-tasks); nothing was discarded
[exit 1]
  OK    [empty] milestone discard · refuses: no success ack, no panic (exit 1)
  FAIL  [empty] milestone discard · the refusal carries a code AND a route

$ jigc task discard 
discarded task 
[exit 0]
  FAIL  [empty] task discard · refuses: no success ack, no panic (exit 0)
  FAIL  [empty] task discard · the refusal carries a code AND a route

=== AFTER the column · did the live task survive?

$ jigc task list
jigc task list — no active tasks
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]

$ ls -la .jigc/tasks
ls: cannot access '.jigc/tasks': No such file or directory
[exit 2]
  FAIL  [empty] the live task `keep-one-task-live` is still listed
  FAIL  [empty] …its working area is still on disk
  FAIL  [empty] …and its staged doc is still readable
  OBSERVE  if the three bars above FAIL in the [empty] column: the last door of the column,
           `jigc task discard ''`, took the LIVE task's working area with it — `.jigc/tasks/`
           joined to an empty id is the tasks directory itself — and acked at exit 0.
           That is loss of staged prose no commit holds, on a non-destructive-looking path.

=== THE TABLE · what M50 receives (release binary jigc 1.0.0-rc.13)
  door                      id=""                             id=no-such-task
  task validate             ACK(exit 0) NOCODE route          exit 1 NOCODE route
  task diff                 exit 1 NOCODE NOROUTE             exit 1 NOCODE route
  task finalize --dry-run   exit 1 NOCODE NOROUTE             exit 1 NOCODE route
  task bind                 exit 1 NOCODE route               exit 1 NOCODE route
  start --task              exit 1 NOCODE NOROUTE             exit 1 NOCODE route
  workflow --task           exit 1 NOCODE NOROUTE             exit 1 NOCODE route
  doc show --task           exit 1 code route                 exit 1 NOCODE route
  doc list --task           ACK(exit 0) NOCODE NOROUTE        exit 1 NOCODE route
  doc create --task         exit 1 NOCODE route               exit 1 NOCODE route
  doc add-item --task       exit 1 NOCODE route               exit 1 NOCODE route
  doc remove-item --task    exit 1 NOCODE route               exit 1 NOCODE route
  doc retitle-item --task   exit 1 NOCODE route               exit 1 NOCODE route
  doc rename --task         exit 1 NOCODE route               exit 1 NOCODE route
  doc set-field --task      exit 1 code route                 exit 1 NOCODE route
  doc set-slot --task       exit 1 NOCODE route               exit 1 NOCODE route
  doc author --task         exit 1 NOCODE route               exit 1 NOCODE route
  milestone execute         exit 1 NOCODE route               exit 1 NOCODE route
  milestone provision       exit 1 NOCODE route               exit 1 NOCODE route
  milestone join            exit 1 code route                 exit 1 code route
  milestone finalize        exit 1 NOCODE route               exit 1 NOCODE route
  milestone list-tasks      exit 1 NOCODE route               exit 1 NOCODE route
  milestone add-task        exit 1 code route                 exit 1 code route
  milestone add-from-spec   exit 1 code route                 exit 1 code route
  milestone discard         exit 1 NOCODE route               exit 1 NOCODE route
  task discard              ACK(exit 0) NOCODE NOROUTE        exit 1 NOCODE route

  cells failing the pass condition: 42 of 50
    [nonexistent] task validate: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] task diff: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] task finalize --dry-run: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] task bind: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] start --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] workflow --task: exit 1 NOCODE route — "no task `no-such-task` — list a milestone's sub-tasks with `jigc milestone list-tasks <milestone-id>`"
    [nonexistent] doc show --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc list --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc create --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc add-item --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc remove-item --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc retitle-item --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc rename --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc set-field --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc set-slot --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] doc author --task: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [nonexistent] milestone execute: exit 1 NOCODE route — "milestone `no-such-task` does not exist"
    [nonexistent] milestone provision: exit 1 NOCODE route — "milestone `no-such-task` does not exist"
    [nonexistent] milestone finalize: exit 1 NOCODE route — "milestone `no-such-task` does not exist"
    [nonexistent] milestone list-tasks: exit 1 NOCODE route — "milestone `no-such-task` does not exist"
    [nonexistent] milestone discard: exit 1 NOCODE route — "milestone `no-such-task` does not exist"
    [nonexistent] task discard: exit 1 NOCODE route — "no task `no-such-task` — list live tasks with `jigc task list`"
    [empty] task validate: ACK(exit 0) NOCODE route — "no findings — the task validates clean"
    [empty] task diff: exit 1 NOCODE NOROUTE — "could not read the base pin at "/work/.jigc/tasks/base.json": No such file or directory (os error 2)"
    [empty] task finalize --dry-run: exit 1 NOCODE NOROUTE — "could not read the base pin at "/work/.jigc/tasks/base.json": No such file or directory (os error 2)"
    [empty] task bind: exit 1 NOCODE route — "task at "/work/.jigc/tasks/" has no recorded workflow — discard it and re-start with `jigc start`"
    [empty] start --task: exit 1 NOCODE NOROUTE — "could not read the base pin for task ``: No such file or directory (os error 2)"
    [empty] workflow --task: exit 1 NOCODE NOROUTE — "could not read the base pin for task ``: No such file or directory (os error 2)"
    [empty] doc list --task: ACK(exit 0) NOCODE NOROUTE — "jigc doc list — no docs staged in task "
    [empty] doc create --task: exit 1 NOCODE route — "the active task has no recorded workflow — discard it with `jigc task discard ` and re-start with `jigc start`"
    [empty] doc add-item --task: exit 1 NOCODE route — "no staged instance for `spec:nothing#criteria` — provision it first (`jigc start` / `jigc doc create <type> --"
    [empty] doc remove-item --task: exit 1 NOCODE route — "no staged instance for `spec:nothing#criteria/probe` — provision it first (`jigc start` / `jigc doc create <ty"
    [empty] doc retitle-item --task: exit 1 NOCODE route — "no staged instance for `spec:nothing#criteria/probe` — provision it first (`jigc start` / `jigc doc create <ty"
    [empty] doc rename --task: exit 1 NOCODE route — "no staged instance for `adr:keep-the-oldest-sample` — provision it first (`jigc start` / `jigc doc create <typ"
    [empty] doc set-slot --task: exit 1 NOCODE route — "no staged instance for `adr:keep-the-oldest-sample#context` — provision it first (`jigc start` / `jigc doc cre"
    [empty] doc author --task: exit 1 NOCODE route — "the active task has no recorded workflow — discard it with `jigc task discard ` and re-start with `jigc start`"
    [empty] milestone execute: exit 1 NOCODE route — "milestone `` does not exist"
    [empty] milestone provision: exit 1 NOCODE route — "milestone `` does not exist"
    [empty] milestone finalize: exit 1 NOCODE route — "milestone `` does not exist"
    [empty] milestone list-tasks: exit 1 NOCODE route — "milestone `` does not exist"
    [empty] milestone discard: exit 1 NOCODE route — "milestone `` does not exist"
    [empty] task discard: ACK(exit 0) NOCODE NOROUTE — "discarded task "

=== SUMMARY
  25 id-taking doors derived from the binary's help × 2 columns; the table above is the
  deliverable. Expected RED on rc.13 (PT-1): the bars that FAIL are the measurement.
ARM 17 FAIL
(the arm exited 1 — that is data, not necessarily failure)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/17-empty-id-axis (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 65
  doc show --task    : 4
  doc show (any)     : 4
  §3.3 adjacent      : 8   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/17-empty-id-axis"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 65 records · `doc show … --task` **4** · §3.3 adjacent 8

## `18-surface-batch.sh`

exit **1**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/18-surface-batch
container  : 85603c31f1bea6b86448e261bcf7798f85b6374cc47fd02574717ea892054d85
  (a mid-stream plant runs with: docker exec -it -u node 85603c31f1bea6b86448e261bcf7798f85b6374cc47fd02574717ea892054d85 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/18-surface-batch.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== FIXTURE · two committed adrs and a committed vision — what the rename cells need

$ git ls-files docs/decisions VISION.md
VISION.md
docs/decisions/cap-distinct-series.md
docs/decisions/drop-the-oldest-sample.md
[exit 0]
  OK    two adrs + the vision are committed, no task is live, the tree is clean

=== (a) · unknown doctype `nosuch` — one answer per door kind, no Debug leak

$ jigc doc create nosuch --title X --task probe-the-doctype-doors
blocking · create.unknown-doctype — unknown doctype `nosuch`; known doctypes: [adr, arch-doc, changelog, commit, completion-record, decisions-log, deferral-ledger, dogfood-record, idea, milestone-record, planning-record, prd, research, roadmap, spec, vision]
  at: nosuch
  route: run `jigc describe` to see the doctypes you can author
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    jigc doc create nosuch --title X --task probe-the-doctype-doors → create.unknown-doctype + a route
  OK    …and it names the authorable set, routing at jigc describe

$ sh -c printf 'title: X\nsections: []\n' | jigc doc author nosuch --from-file - --task probe-the-doctype-doors
blocking · create.unknown-doctype — unknown doctype `nosuch`; known doctypes: [adr, arch-doc, changelog, commit, completion-record, decisions-log, deferral-ledger, dogfood-record, idea, milestone-record, planning-record, prd, research, roadmap, spec, vision]
  at: nosuch
  route: run `jigc describe` to see the doctypes you can author
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    sh -c printf 'title: X\nsections: []\n' | jigc doc author nosuch --from-file - --task probe-the-doctype-doors → create.unknown-doctype + a route

$ jigc doc show nosuch:x
blocking · store.unknown-type — unknown doctype `nosuch` for `nosuch:x`
  at: nosuch:x
  route: list the available doctypes with `jigc describe`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    jigc doc show nosuch:x → store.unknown-type + a route

$ jigc doc schema nosuch
blocking · store.unknown-type — unknown doctype `nosuch`
  at: nosuch
  route: list the available doctypes with `jigc describe`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    jigc doc schema nosuch → store.unknown-type + a route

$ jigc doc list nosuch
blocking · store.unknown-type — unknown doctype `nosuch`
  at: nosuch
  route: list the available doctypes with `jigc describe`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    jigc doc list nosuch → store.unknown-type + a route

$ jigc migrate README.md --as nosuch
blocking · store.unknown-type — unknown doctype `nosuch`; migratable doctypes: adr, arch-doc, changelog, completion-record, decisions-log, deferral-ledger, idea, prd, research, roadmap, spec, vision
  at: nosuch
  route: list the available doctypes with `jigc describe`
[exit 1]
  OK    jigc migrate README.md --as nosuch → store.unknown-type + a route
  OK    …migrate's narrower set is said in the MESSAGE, the code stays store.unknown-type

$ jigc relocate nosuch --from x/
blocking · store.unknown-type — unknown doctype `nosuch`
  at: nosuch
  route: list the available doctypes with `jigc describe`
[exit 1]
  OK    jigc relocate nosuch --from x/ → store.unknown-type + a route

$ jigc rename nosuch:x --to Y
blocking · store.unknown-type — unknown doctype `nosuch`
  at: nosuch
  route: list the available doctypes with `jigc describe`
[exit 1]
  OK    jigc rename nosuch:x --to Y → store.unknown-type + a route
  OK    no {:?} Debug leak on any of the eight doors

=== (b) · jigc from a NON-git directory — one text, one route, on both verbs

$ (cd /tmp/arm18-nogit.7rzJbX && jigc validate)
not inside a git repository (no `.git` found from /tmp/arm18-nogit.7rzJbX) — run jigc from inside the target git repository; if this project isn't one yet, `git init` here first
[exit 1]

$ (cd /tmp/arm18-nogit.7rzJbX && jigc start "x")
not inside a git repository (no `.git` found from /tmp/arm18-nogit.7rzJbX) — run jigc from inside the target git repository; if this project isn't one yet, `git init` here first
[exit 1]
  OK    validate refuses (non-zero)
  OK    start refuses (non-zero)
  OK    both name the state and the route: git init here first
  OK    …and the two verbs print the SAME text

=== (c) · write.unknown-section at four write verbs — the code + the `jigc doc schema` route
staged: adr:probe-decision

$ sh -c echo x | jigc doc set-slot 'adr:probe-decision#nosection' --from-file - --task probe-the-section-miss
blocking · write.unknown-section — write rejected: no section "nosection" declared in the schema
  at: adr:probe-decision#nosection
  route: `jigc doc schema adr` to see the declared shape, then re-run the write at a declared address
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    set-slot · write.unknown-section
  OK    set-slot · routes at jigc doc schema adr
  OK    set-slot · non-zero

$ jigc doc set-field adr:probe-decision#nosection --value x --task probe-the-section-miss
no field addressed by `adr:probe-decision#nosection`
[exit 1]
  FAIL  set-field --value · write.unknown-section
  FAIL  set-field --value · routes at jigc doc schema adr
  OK    set-field --value · non-zero

$ jigc doc set-field adr:probe-decision#nosection --unset --task probe-the-section-miss
no field addressed by `adr:probe-decision#nosection`
[exit 1]
  FAIL  set-field --unset · write.unknown-section
  FAIL  set-field --unset · routes at jigc doc schema adr
  OK    set-field --unset · non-zero

$ sh -c printf 'title: Probe decision\nsections:\n  - id: nosection\n    set:\n      context: |\n        <<x>>\n' | jigc doc author adr --from-file - --task probe-the-section-miss
blocking · write.unknown-section — write rejected: no section "nosection" declared in the schema
  at: adr:probe-decision#nosection/context
  route: `jigc doc schema adr` to see the declared shape, then re-run the write at a declared address
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OK    author payload · write.unknown-section
  OK    author payload · routes at jigc doc schema adr
  OK    author payload · non-zero

$ jigc doc set-field adr:probe-decision#nosection/status --value x --task probe-the-section-miss
blocking · write.unknown-section — write rejected: no section "nosection" declared in the schema
  at: adr:probe-decision#nosection/status
  route: `jigc doc schema adr` to see the declared shape, then re-run the write at a declared address
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]
  OBSERVE  if the two set-field cells above FAIL while this deeper form carries the code,
           the miss is the SECTION-level address shape at set-field only (M49 Increment 11's
           'four producers of the bare form' claim, re-measured on the binary).

=== (e) · a non-UTF-8 argv byte with the invocation log ON — must not panic

$ jigc doc show <the single byte 0xff>
error: invalid UTF-8 was detected in one or more arguments

Usage: jigc doc show [OPTIONS] <ADDR>

For more information, try '--help'.
[exit 2]
  OK    exit is not 101 (no panic)
  OK    …and it is a refusal, not a success
  OK    …and the invocation log recorded it (lossy argv, not a crash)

$ tail -1 .jigc/logs/invocations.jsonl
{"timestamp":"2026-09-04T18:28:45Z","argv":["doc","show","�"],"exit_code":2,"duration_ms":9,"finding_codes":[],"output_bytes":135,"binary_version":"1.0.0-rc.13","error_code":null}
[exit 0]

=== (f) · describe --commands — every entry carries its pack, in both formats

$ node -e 
const d=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));
const c=d.commands||[]; const nopack=c.filter(x=>!x.pack);
console.log("entries: "+c.length+" · without pack: "+nopack.length+" · packs: "+[...new Set(c.map(x=>x.pack))].join(","));
process.exit(c.length>16&&nopack.length===0?0:1); /tmp/arm18-cmds.X2Xvrc
entries: 31 · without pack: 0 · packs: dev,methodology
[exit 0]
  OK    JSON: > 16 entries, each with a pack key
  OK    prose renders <id> (<pack> pack)
  OK    …for BOTH packs

=== (g) · S-1: the composed step names the permitted heredoc form · S-4: rename --task names doc rename

$ grep -n from-file -\|EOF\|heredoc /tmp/composed.txt
18:Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
23:jigc doc set-slot adr:<slug>#context --from-file - --task x <<'EOF'
25:EOF
31:jigc doc set-slot adr:<slug>#context --from-file - --task x
32:jigc doc set-slot adr:<slug>#decision --from-file - --task x
33:jigc doc set-slot adr:<slug>#consequences --from-file - --task x
40:jigc doc set-slot adr:<slug>#options --from-file - --task x
50:jigc doc author adr --from-file - --task x
76:Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
81:jigc doc set-slot commit:x#summary --from-file - --task x <<'EOF'
83:EOF
89:Run: `jigc doc set-slot commit:x#summary --from-file - --task x`
101:jigc doc set-slot commit:x#body --from-file - --task x
[exit 0]
  OK    S-1 · the composed text names the heredoc form <<'EOF' next to --from-file -
  OK    S-1 · …and says in prose that stdin is the payload channel

$ jigc rename adr:drop-the-oldest-sample --to Y --task x
error: unexpected argument '--task' found

  tip: `jigc rename` is the committed-store identity refactor — task-less and self-committing, and it refuses while any task is in flight, so it takes no `--task`. The in-task title change is its sibling: `jigc doc rename <address> --to <title> --task <task-id>` retitles the doc your task has staged, and re-slugs it while its identity is still uncommitted

Usage: jigc rename --to <TO> <type:slug>

For more information, try '--help'.
[exit 2]
  OK    S-4 · the wrong-turn refuses
  OK    S-4 · …and its tip names jigc doc rename with the argv shape
  OK    S-4 · …and says WHY rename takes no --task (task-less, self-committing)

=== (d) · jigc rename — the nine refusals, reached from the store the fixture built

$ jigc rename adr:drop-the-oldest-sample --to Drop the newest sample
blocking · rename.in-flight — cannot rename while task `x` is in flight — finalize or discard it first (a rename changes the by-task-id join key)
  at: adr:drop-the-oldest-sample
  route: settle it first — `jigc task finalize x` if that work is done, `jigc task discard x` if it is not; a rename is task-less and self-committing, so it runs once neither is open
[exit 1]
  OK    in-flight (task x open) · code rename.in-flight
  OK    in-flight (task x open) · exits non-zero
  OK    …the in-flight route names BOTH exits for that task
  OK    the store is settled again: no live task, clean tree

$ jigc rename nosuch:x --to Y
blocking · store.unknown-type — unknown doctype `nosuch`
  at: nosuch
  route: list the available doctypes with `jigc describe`
[exit 1]
  OK    unknown type · code store.unknown-type
  OK    unknown type · exits non-zero

$ jigc rename commit:x --to Y
blocking · store.transient-type — `commit` is a transient doctype — it never lands as a repo file, so it has no persisted path to rename
  at: commit
  route: `jigc describe` lists the doctypes that do persist, and the identity each one carries
[exit 1]
  OK    transient commit · code store.transient-type
  OK    transient commit · exits non-zero

$ jigc rename adr:no-such-doc --to Y
blocking · store.not-found — no managed doc `adr:no-such-doc` to rename (expected at docs/decisions/no-such-doc.md)
  at: adr:no-such-doc
  route: `jigc describe` lists the doctype surface — check the id you typed against it
[exit 1]
  OK    not-found · code store.not-found
  OK    not-found · exits non-zero

$ jigc rename adr:drop-the-oldest-sample --to ???
blocking · write.unslugable-title — `--to "???"` slugs to nothing — a rename derives the new id from the title, and this one carries no slug-able content
  at: adr:drop-the-oldest-sample
  route: re-run with a title carrying at least one word character, or name the id yourself: `jigc rename adr:drop-the-oldest-sample --to <a title> --slug <new-slug>`
[exit 1]
  OK    unslugable title · code write.unslugable-title
  OK    unslugable title · exits non-zero

$ jigc rename adr:cap-distinct-series --to Drop the oldest sample
blocking · write.already-present — cannot rename to `adr:drop-the-oldest-sample` — a different doc already exists at docs/decisions/drop-the-oldest-sample.md
  at: adr:drop-the-oldest-sample
  route: give this doc an id nothing else answers to — re-run `jigc rename adr:cap-distinct-series --to 'Drop the oldest sample' --slug <other-slug>`; or, if `adr:drop-the-oldest-sample` is the doc you meant to work on, read it with `jigc doc show adr:drop-the-oldest-sample` and rename that one instead
[exit 1]
  OK    already-present · code write.already-present
  OK    already-present · exits non-zero

$ jigc rename vision:vision --to New Vision
blocking · write.identity-change — cannot reslug `vision:vision` — a placement singleton's identity is fixed to its type (the slug IS the type id `vision` and the doc lives at the literal VISION.md); only a retitle is supported
  at: vision:vision
  route: `jigc rename vision:vision --to 'New Vision' --slug vision` keeps the identity that cannot move and rewrites only the title
[exit 1]
  OK    identity-change · singleton · code write.identity-change
  OK    identity-change · singleton · exits non-zero
  OK    …routes at the --slug form that keeps the fixed identity

$ git ls-files docs/milestone-records
docs/milestone-records/bound-the-store.md
[exit 0]

$ jigc rename milestone-record:bound-the-store --to Bound
blocking · write.identity-change — cannot reslug `milestone-record:bound-the-store` — a milestone-record's slug IS its milestone work-unit id (it keys `.jigc/milestones/bound-the-store` and every milestone op), so a reslug would sever the committed record from its work unit; only a retitle is supported
  at: milestone-record:bound-the-store
  route: `jigc rename milestone-record:bound-the-store --to Bound --slug bound-the-store` keeps the identity that cannot move and rewrites only the title
[exit 1]
  OK    identity-change · milestone-record · code write.identity-change
  OK    identity-change · milestone-record · exits non-zero
  OK    …routes at the --slug form that keeps the work-unit identity

$ jigc rename adr:drop-the-oldest-sample --to Drop the newest sample
blocking · rename.dirty-tree — cannot rename with a dirty working tree — commit or stash your changes first (a rename is a deliberate standalone op that commits in place): README.md
  at: adr:drop-the-oldest-sample
  route: commit those tracked changes, or `git stash` them, then re-run the rename — a rename commits in place with no pathspec, so anything already in the index would ride its commit
[exit 1]
  OK    dirty-tree · code rename.dirty-tree
  OK    dirty-tree · exits non-zero

$ jigc rename adr:drop-the-oldest-sample --to Drop the oldest sample
no-op: adr:drop-the-oldest-sample already holds the title "Drop the oldest sample" at docs/decisions/drop-the-oldest-sample.md — nothing renamed, nothing committed
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the idempotent rename is a no-op ack at exit 0, saying nothing moved

$ jigc rename vision --to New Vision
`vision` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`
  route: run `jigc describe` for the doctype surface
[exit 1]
  OBSERVE  a bare singleton address at the top-level verb: recorded above — it carries a route
           but no finding code (compare `jigc doc show vision`, which accepts the bare form).

=== (h) · changelog-recording.gate-granted-unused at exit 0 on a single-task with no changelog (M46 carry)

$ jigc task validate tidy-the-ingest-path
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  at: task:tidy-the-ingest-path
  route: if the change is user-facing, record it in this task — `jigc doc create changelog --title Changelog --task tidy-the-ingest-path`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task tidy-the-ingest-path`; if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the advisory fires
  OK    …names the granting workflow
  OK    …carries a route naming the create + add-item pair AND the no-action exit
  OK    …at exit 0 — advisory, not a block

=== SUMMARY
  8 of DOCTYPE_DOORS' 15 · all 9 of RefusalKind::ALL (+ the no-op) · the non-git pair ·
  the four-verb section miss · non-UTF-8 argv · describe in both formats · S-1 · S-4 · the
  M46 changelog gate. Both registries are fenced in full by flow50; this arm drives a stated subset.
ARM 18 FAIL
(the arm exited 1 — that is data, not necessarily failure)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/18-surface-batch (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 83
  doc show --task    : 0
  doc show (any)     : 3
  §3.3 adjacent      : 1   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/18-surface-batch"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 83 records · `doc show … --task` **0** · §3.3 adjacent 1

## `19-planning-record.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/19-planning-record
container  : dc1b0bb81db2e59023b47b8a7c08c7a891a5afc1e50eb52b5f5de0dad99a0ad7
  (a mid-stream plant runs with: docker exec -it -u node dc1b0bb81db2e59023b47b8a7c08c7a891a5afc1e50eb52b5f5de0dad99a0ad7 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/19-planning-record.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.13

=== 1 · the gate set, read off the binary

$ jigc doc schema planning-record --format json
{
  "contract-version": 5,
  "type": "planning-record",
  "schema-version": 1,
  "fields": [
    {
      "id": "schema-version",
      "type": "int",
      "required": true,
      "author-required": false,
      "set": "schema-version",
      "section": "meta"
    }
  ],
  "sections": [
    {
      "id": "reuse-exercised",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#reuse-exercised"
    },
    {
      "id": "cheap-vs-robust",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#cheap-vs-robust"
    },
    {
      "id": "foreclosed-by-doc",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#foreclosed-by-doc"
    },
    {
      "id": "prior-art-reconciled",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#prior-art-reconciled"
    },
    {
      "id": "census",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#census"
    },
    {
      "id": "integration-seam",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#integration-seam"
    },
    {
      "id": "check-scope-pinned",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#check-scope-pinned"
    },
    {
      "id": "design-complete",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#design-complete"
    },
    {
      "id": "acceptance-spiked",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#acceptance-spiked"
    },
    {
      "id": "value-flow-exercised",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#value-flow-exercised"
    },
    {
      "id": "deliverable-reachable",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#deliverable-reachable"
    },
    {
      "id": "strategic-claim-fresh",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#strategic-claim-fresh"
    },
    {
      "id": "quote-attributed",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#quote-attributed"
    },
    {
      "id": "claim-driven",
      "kind": "slot",
      "set-slot": "planning-record:<slug>#claim-driven"
    }
  ]
}
[exit 0]
  MEASURED · 14 required gate slots: reuse-exercised cheap-vs-robust foreclosed-by-doc prior-art-reconciled census integration-seam check-scope-pinned design-complete acceptance-spiked value-flow-exercised deliverable-reachable strategic-claim-fresh quote-attributed claim-driven
  held out (the LAST one the projection lists): claim-driven
  OK    the projection yields at least one required slot
  OK    …and the count is the settled 14 (milestone-planning-workflow.md's one home)
  OK    planning-record reports schema-version 1 on doc schema
  OK    …and every gate is a required slot, none optional

=== 2 · jigc start --workflow planning — the composed output renders the schema projection
  T=plan-the-first-milestone
  MEASURED · composed planning workflow: 406 lines
           (observation, not a bar — DECISIONS 2026-08-31 says the projection roughly tripled it)

$ sed -n /^Run: `jigc doc create planning-record/,/^- `meta`/p /tmp/arm19.Zr2h9z/compose.txt
Run: `jigc doc create planning-record --title <TITLE> --task plan-the-first-milestone`

EVERY gate slot is required, and that is the whole point of the record rather than a
side effect of it: `jigc task finalize` BLOCKS on an unfilled one
(`schema-conformance.required-slot-present`, naming the gate at its own address) and
lands the record once every gate is answered. jigc checks only that a slot is
FILLED — it never reads, scores or lints the judgment you put inside it — so an
honest `N/A — <why>` is a complete answer, no phrasing is graded, and what the record
buys is that no gate goes UNANSWERED.

The schema and its batch payload follow, both generated from the resolved
`planning-record`: each gate's line states what that gate requires recorded, and the
payload places every gate over a single staged buffer.

The `planning-record` schema — each instance a managed file at `docs/planning-records/<slug>.md`, its `<slug>` minted from `title`.

- `meta` (front-matter fields):
[exit 0]
  OK    the compose minted a task
  OK    it names the create: jigc doc create planning-record --title <TITLE> --task plan-the-first-milestone
  OK    it renders the planning-record schema (the {{schema:planning-record}} projection) and its batch payload
  gates named in the composed output: 14 of 14
  OK    every gate the schema declares is named in the composed output
  OK    …and it says the finalize BLOCKS on an unfilled gate, naming the code
  OK    …and states the home: docs/planning-records/<slug>.md

=== 3 · create the record through the gate role, fill every gate but the held-out one

$ jigc doc create planning-record --title M1 the first milestone --task plan-the-first-milestone
planning-record:m1-the-first-milestone
[exit 0]
  ADDR=planning-record:m1-the-first-milestone  SLUG=m1-the-first-milestone
  OK    the create is accepted at the gate role and emits the identity
  filled 13 of 14 gates; claim-driven left empty
  OK    every gate but the held-out one accepted its prose

=== 4 · task validate and task finalize both block, NAMING the unfilled gate, and commit nothing

$ jigc task validate plan-the-first-milestone
advisory · file-state.staged-copy — staged copy of `docs/planning-records/m1-the-first-milestone.md` — this task's in-flight version of the doc
  at: docs/planning-records/m1-the-first-milestone.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
blocking · schema-conformance.required-slot-present — `docs/planning-records/m1-the-first-milestone.md`: required slot in section `claim-driven` is empty
  at: planning-record:m1-the-first-milestone#claim-driven · line 60
  route: `jigc doc set-slot planning-record:m1-the-first-milestone#claim-driven --task plan-the-first-milestone --from-file -` to fill the empty slot
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 3]

$ jigc task validate plan-the-first-milestone --format json
{
  "schema_version": 2,
  "findings": [
    {
      "severity": "advisory",
      "probe": "file-state",
      "check": "staged-copy",
      "code": "file-state.staged-copy",
      "key": {
        "code": "file-state.staged-copy",
        "target": "docs/planning-records/m1-the-first-milestone.md"
      },
      "message": "staged copy of `docs/planning-records/m1-the-first-milestone.md` — this task's in-flight version of the doc",
      "location": {
        "address": "docs/planning-records/m1-the-first-milestone.md",
        "line": 1,
        "col": 1
      },
      "route": "no action needed — the staged copy is validated in-task and baselined when its finalize lands"
    },
    {
      "severity": "blocking",
      "probe": "schema-conformance",
      "check": "required-slot-present",
      "code": "schema-conformance.required-slot-present",
      "key": {
        "code": "schema-conformance.required-slot-present",
        "target": "planning-record:m1-the-first-milestone#claim-driven"
      },
      "message": "`docs/planning-records/m1-the-first-milestone.md`: required slot in section `claim-driven` is empty",
      "location": {
        "address": "planning-record:m1-the-first-milestone#claim-driven",
        "line": 60,
        "col": 1
      },
      "route": "`jigc doc set-slot planning-record:m1-the-first-milestone#claim-driven --task plan-the-first-milestone --from-file -` to fill the empty slot"
    }
  ]
}[exit 3]
  OK    task validate exits non-zero
  OK    …with schema-conformance.required-slot-present at planning-record:m1-the-first-milestone#claim-driven
  OK    …and it is the ONLY blocking finding — every other gate is answered
  OK    …and the text names the gate
  OK    …routing at the set-slot that fills it, with --task

$ jigc task finalize plan-the-first-milestone
blocking · schema-conformance.required-slot-present — `docs/planning-records/m1-the-first-milestone.md`: required slot in section `claim-driven` is empty
  at: planning-record:m1-the-first-milestone#claim-driven · line 60
  route: `jigc doc set-slot planning-record:m1-the-first-milestone#claim-driven --task plan-the-first-milestone --from-file -` to fill the empty slot
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 3]
  OK    task finalize exits non-zero
  OK    …naming the same code and the same gate
  OK    …and commits nothing: git rev-list --count HEAD 8 -> 8
  OK    …and the record is not on disk — still staged, not promoted
  OK    …and the task is still open

=== 5 · fill the last gate; the IDENTICAL finalize lands

$ jigc doc set-slot planning-record:m1-the-first-milestone#claim-driven --from-file /tmp/arm19.Zr2h9z/answer.md --task plan-the-first-milestone
set slot planning-record:m1-the-first-milestone#claim-driven (29 chars)
[exit 0]
  OK    the held-out gate accepts its prose

$ jigc task finalize plan-the-first-milestone
advisory · file-state.staged-copy — staged copy of `docs/planning-records/m1-the-first-milestone.md` — this task's in-flight version of the doc
  at: docs/planning-records/m1-the-first-milestone.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized ec9b959 — docs(planning): record the M1 planning gates
  added .jigc/config/manifest.yaml
  promoted docs/planning-records/m1-the-first-milestone.md
  2 files committed
[exit 0]
  OK    the same finalize now lands at exit 0
  OK    …as exactly one commit: 8 -> 9
  OK    the record is at docs/planning-records/m1-the-first-milestone.md
  OK    …not under completions/

$ git show --stat --format=%s HEAD
docs(planning): record the M1 planning gates

 .jigc/config/manifest.yaml                      |  2 +
 docs/planning-records/m1-the-first-milestone.md | 61 +++++++++++++++++++++++++
 2 files changed, 63 insertions(+)
[exit 0]

$ jigc doc show planning-record:m1-the-first-milestone --format json
{
  "fields": {
    "schema-version": "1"
  },
  "item-count": 0,
  "schema-version": 1,
  "sections": {
    "acceptance-spiked": "Answered for the probe wave.",
    "census": "Answered for the probe wave.",
    "cheap-vs-robust": "Answered for the probe wave.",
    "check-scope-pinned": "Answered for the probe wave.",
    "claim-driven": "Answered for the probe wave.",
    "deliverable-reachable": "Answered for the probe wave.",
    "design-complete": "Answered for the probe wave.",
    "foreclosed-by-doc": "Answered for the probe wave.",
    "integration-seam": "Answered for the probe wave.",
    "prior-art-reconciled": "Answered for the probe wave.",
    "quote-attributed": "Answered for the probe wave.",
    "reuse-exercised": "Answered for the probe wave.",
    "strategic-claim-fresh": "Answered for the probe wave.",
    "value-flow-exercised": "Answered for the probe wave."
  },
  "slug": "m1-the-first-milestone",
  "type": "planning-record"
}
[exit 0]
  OK    doc show reads it back committed with schema-version: 1 — a top-level integer, no staged key
  OK    …every gate the schema declares holding its prose

$ head -8 docs/planning-records/m1-the-first-milestone.md
---
schema-version: 1
---

# M1 the first milestone

## Reuse Exercised

[exit 0]
  OK    …and the file carries the stamp in its front-matter

$ jigc doc list planning-record --format json
{
  "docs": [
    {
      "id": "planning-record:m1-the-first-milestone",
      "path": "docs/planning-records/m1-the-first-milestone.md",
      "state": "managed",
      "item-count": 0
    }
  ]
}
[exit 0]
  OK    doc list planning-record lists it managed at that path

$ jigc validate
no findings — the committed store validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the committed store validates clean

=== SUMMARY
  the gate set was read off jigc doc schema planning-record (14 gates), the last
  one held out; validate and finalize both named it and committed nothing; filled, the
  identical finalize landed docs/planning-records/m1-the-first-milestone.md at schema-version 1.
  composed planning workflow: 406 lines (observation).
ARM 19 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/19-planning-record (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 31
  doc show --task    : 0
  doc show (any)     : 1
  §3.3 adjacent      : 2   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/19-planning-record"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

invocation log: 31 records · `doc show … --task` **0** · §3.3 adjacent 2

## `20-project-pack-composition.sh`

exit **1**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/20-project-pack-composition
container  : 3215c2f3c8e8b5d0211c1d4d1665a5c72fd68e35a9653292574e1e6c1522d2c4
  (a mid-stream plant runs with: docker exec -it -u node 3215c2f3c8e8b5d0211c1d4d1665a5c72fd68e35a9653292574e1e6c1522d2c4 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/20-project-pack-composition.sh in the container
-------------------------------------------------------------

=== 0 · adopt — setup writes the marker into every project
jigc 1.0.0-rc.13
compose-embedded-methodology: true
  OK    setup wrote the compose marker

=== 1 · THE HOUSE PACK — one new doctype, one workflow that creates it, one step
  house pack: /tmp/housepack-Ec9Bpg
    /tmp/housepack-Ec9Bpg/config/defaults.yaml
    /tmp/housepack-Ec9Bpg/schemas/note.yaml
    /tmp/housepack-Ec9Bpg/steps/author-note.yaml
    /tmp/housepack-Ec9Bpg/workflows/record-note.yaml
compose-embedded-methodology: true
packs:
  - /tmp/housepack-Ec9Bpg

=== 1 · BOUND 1, driven — the pack composes only after it vendors config/commands.yaml

$ jigc start --workflow record-note note the cache policy
the embedded pack is missing `commands`: no pack resource of kind config with id `commands`
[exit 1]
  OK    without a vendored catalog the door refuses (exit non-zero)
  OK    …naming the missing resource
  OK    no task was minted by the refused door
  FAIL  the fault names the pack that lacks the catalog (house), not 'the embedded pack' — law 1

$ jigc workflow record-note --preview
preview: workflow `record-note` — no task minted. This shows what it will ask before you commit to running it.
To run it for real: `jigc start --workflow record-note "<intent>"`   — mints the task and composes this.
Below, `--task your-task-id` marks where the minted id goes.

Record the note. The intent is:


Create it, then set its topic and author its body:

jigc doc create note --title "<title>" --task your-task-id
jigc doc set-field note:<slug>#meta/topic --value "<topic>" --task your-task-id
jigc doc set-slot note:<slug>#body --from-file - --task your-task-id <<'NOTE'
<the note>
NOTE

Read the staged note back before finalizing — `jigc doc show note:<slug> --task your-task-id` serves the staged copy.
Then author the commit doc and finalize with `jigc task finalize your-task-id`.
create-gates: note   — the doc-types this task is allowed to create; any other type is refused
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    with an (empty) vendored catalog the same pack composes (workflow --preview)

=== A · EXTENSION — the ungoverned `note` resolves from the listed pack

$ jigc describe --doctypes
jigc describe — a tour of what this project lets you compose and author.

The doc-types you can author. adr is A dated architectural decision record, capturing the context a choice was made in, the choice itself, and its consequences, with an optional link to the decision it supersedes. Reach for it when a choice is worth preserving with its rationale, so a later reader can recover why the call was made or supersede it on the record.

arch-doc is Living architecture documentation for one part of the system — its overview, its components and the code that implements them, and the decisions behind it. Reach for it when a part of the system is worth a durable description that stays honest against the code, so a later reader (or agent) can orient without reverse-engineering it.

changelog is A Keep-a-Changelog singleton — staged unreleased changes plus the cut releases, each grouped by category, maintained over the life of the project. Reach for it when a user-facing change lands and the project keeps a human-readable record of what changed, staged now and cut into versioned releases over time.

commit is A Conventional-Commits message — a typed, scoped header over a subject line, an optional body, and trailers — rendered into the git commit rather than persisted as a repo file. Reach for it when you need to record what a change does and why at the moment it lands, in the form git and reviewers already read.

completion-record is The per-milestone completion record — the audit verdict, the owner-artifact the genuine audit produced, and each triaged finding with its disposition. Reach for it when a milestone reaches its completion audit, so its verdict, the audit's owner-artifact, and the triaged findings need a durable per-milestone record.

decisions-log is The running log of decisions made — each with the date it was settled and the reasoning behind it. Reach for it when the planning loop reaches Settle (or completion reaches triage) and a decision is made worth preserving with its rationale, dated, on the record.

deferral-ledger is The running ledger of deferred decisions and parked ideas, each keyed to the milestone trigger that resurfaces it. Reach for it when the planning loop reaches Settle and a decision is deferred or an idea parked rather than made now, so it needs a durable owed-and-when record.

dogfood-record is The per-run measured-dogfood record — the run's case and pinned binary, the transcribed organic fact counts, the seeded instrument checks, the judged verdict, and the owner-artifact holding the raw capture. Reach for it when a measured dogfood run completes, so its transcribed facts, seeded instrument checks, verdict, and capture artifact need a durable per-run record.

idea is One shaped-but-unscheduled direction, with the trigger that would bring it back. Reach for it when a direction is worth keeping but not worth scheduling now, so it needs a durable home cheaper than losing the thought and a note of when to revisit it.

milestone-record is The per-milestone team-ready state record — the shared base-SHA pin, the ordered sub-task list, and each sub-task's intent and status, resumable from a fresh clone. Reach for it when a milestone is executing and its in-flight state (base, sub-tasks, statuses) must be committed and legible so a teammate or fresh session can continue it, not stranded in the gitignored workbench.

note is A short house-local note. Reach for it when you want to jot a house-local note that jigc manages.

planning-record is One milestone's plan-time gate record — every planning gate answered with the evidence that discharges it, or an explicit N/A and why. Reach for it when a milestone is being planned, and each plan-time gate needs an answer recorded before the plan is allowed to close.

prd is A product requirements document — the vision, the requirements, and the context for a piece of work, the first managed document a fresh project develops its idea into. Reach for it when you are at the start of a new project and want to turn an idea into a shared statement of what to build and why, before any spec or code.

research is One investigation and what it found — the evidence a vision or design is formed from. Reach for it when a question needs evidence gathered and recorded before a vision or decision rests on it, and that record should stay addressable by what it grounds.

roadmap is The running milestone spine — each milestone with what it proves and the prose decomposition of its increments. Reach for it when the planning loop reaches Decompose and a milestone's breakdown is worth recording on the durable roadmap.

spec is A specification of what a task implements — its goal, its context, and a set of testably-phrased acceptance criteria. Reach for it when you need to pin down what a piece of work must deliver before building it, so the criteria are agreed up front and code can be checked against them.

vision is The project's charter — its thesis, the invariants it holds, and its open questions — grounded in the research it was formed from. Reach for it when the project's direction is worth a durable, managed anchor that later work compares against, formed from and traceable to the research behind it.

— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    describe --doctypes lists note
  OK    describe --workflows lists record-note

$ jigc doc schema note
doctype: note
* = author-required
fields:
  - topic: string (section: meta) (set-field: note:<slug>#meta/topic) *
sections:
  - body: slot (set-slot: note:<slug>#body)
[exit 0]
  OK    doc schema note projects the house schema
  OK    …and its --format json parses with the house field

=== A · the provider is named — --explain carries the house pack, its path and its content-hash

$ jigc start --workflow record-note --explain
workflow:record-note    (pack-default · house/vfs-local)
  collision: doctype:commit → won by dev/1.0.0-rc.13
  collision: config:knobs → won by dev/1.0.0-rc.13
  Pack input: house/fs-local = /tmp/housepack-Ec9Bpg  (blake3 9b6a1e6a67588b0dc35ce33ce5dda84f2ec4c3e80c7504926c69537f52b73e64)
  Pack input: dev/1.0.0-rc.13 = <embedded>  (blake3 65244b6a89b38336f43ac81fd9f51b0d74eb22b83c64067ada60e7d52eee83a5)
  Pack input: methodology/1.0.0-rc.13 = <embedded>  (blake3 b9fbe7bc56cbfbf4804bcfdc958a6167ea236d8b65fb3c02293ce4f52b897716)
  overrides applied: 1
    invocation-log = true    (project)
  includes:
    step:author-note    (pack-default)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the workflow line names house as the pack that provided record-note
  OK    a Pack input line names the house pack's resolving directory
  OK    …with a blake3 content-hash
  OK    …and both embedded packs are still in the set
  MEASURED · house labels on one --explain surface: house/fs-local house/vfs-local 
  MEASURED · describe --format json keys per definition: kind,id,prose,router_hidden

=== A · a task creates, authors, reads back and finalizes a note into the committed store

$ jigc start --workflow record-note note the cache policy
task minted: note-the-cache-policy

Record the note. The intent is:
note the cache policy

Create it, then set its topic and author its body:

jigc doc create note --title "<title>" --task note-the-cache-policy
jigc doc set-field note:<slug>#meta/topic --value "<topic>" --task note-the-cache-policy
jigc doc set-slot note:<slug>#body --from-file - --task note-the-cache-policy <<'NOTE'
<the note>
NOTE

Read the staged note back before finalizing — `jigc doc show note:<slug> --task note-the-cache-policy` serves the staged copy.
Then author the commit doc and finalize with `jigc task finalize note-the-cache-policy`.
resume: `jigc start --task note-the-cache-policy`   — re-composes this workflow if context is lost
what's-left: `jigc task validate note-the-cache-policy`   — previews part of the finalize gate: this task's content findings, the carryover gate, the owner-artifact causes that need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the commit surface at finalize
task scope: `jigc doc` writes default to the single active task; `--task note-the-cache-policy` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; once a sibling task commits a path this one also touches, resuming or finalizing here blocks and names the overlapping paths
create-gates: note   — the doc-types this task is allowed to create; any other type is refused
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  task: note-the-cache-policy
  OK    a task was minted under the house workflow

$ jigc doc create note --title Cache policy --task note-the-cache-policy
note:cache-policy
[exit 0]

$ jigc doc set-field note:cache-policy#meta/topic --value caching --task note-the-cache-policy
set note:cache-policy#meta/topic = caching
[exit 0]

$ bash -c printf 'Evict cold entries first.\n' | jigc doc set-slot note:cache-policy#body --from-file - --task note-the-cache-policy
set slot note:cache-policy#body (26 chars)
[exit 0]

$ jigc doc show note:cache-policy --task note-the-cache-policy --format json
{
  "fields": {
    "topic": "caching"
  },
  "item-count": 0,
  "schema-version": null,
  "sections": {
    "body": "Evict cold entries first."
  },
  "slug": "cache-policy",
  "staged": "note-the-cache-policy",
  "type": "note"
}
[exit 0]
  OK    the staged read-back carries the field, the slot and the staging task

$ jigc task finalize note-the-cache-policy
advisory · file-state.staged-copy — staged copy of `docs/notes/cache-policy.md` — this task's in-flight version of the doc
  at: docs/notes/cache-policy.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized 3f04ad0 — docs: record the cache policy note
  added .jigc/config/manifest.yaml
  modified .jigc/config/packs.yaml
  promoted docs/notes/cache-policy.md
  3 files committed
[exit 0]
  OK    finalize landed
  OK    …promoting the note to its house location
  OK    doc list note reports it managed
  OK    the committed read serves it
  OK    validate is clean over the composed set

=== C · the methodology pack is not lost — the point of the combination
  OK    doc schema idea resolves
  OK    doc schema milestone-record resolves

=== B · DEMOTION — the same listed pack ships an `adr` with `options` dropped

$ jigc doc schema adr
doctype: adr (schema-version 2)
fields:
  - status: enum [proposed|accepted|superseded] (section: status) (default: proposed) (set-field: adr:<slug>#status/status)
  - date: date (section: status) (set: on-create) (set-field: adr:<slug>#status/date)
  - supersedes: ref (section: status) (set-field: adr:<slug>#status/supersedes)
  - cites-code: code-anchor (section: status) (set-field: adr:<slug>#status/cites-code)
  - schema-version: int (section: status) (set: schema-version)
sections:
  - context: slot (set-slot: adr:<slug>#context)
  - options: slot (optional) (set-slot: adr:<slug>#options)
  - decision: slot (set-slot: adr:<slug>#decision)
  - consequences: slot (set-slot: adr:<slug>#consequences)
[exit 0]
  OK    doc schema adr still loads (exit 0)
  OK    …at the frozen shape: options present
  OK    …and the frozen version
  OK    …with none of the house copy's loss (supersedes still there)
  OK    describe still describes adr in the embedded pack's words
  OK    …and not in the house copy's

$ jigc start --explain
workflow:router    (pack-default · dev/v1.0.0-rc.13)
  collision: doctype:adr → won by dev/1.0.0-rc.13
  collision: doctype:commit → won by dev/1.0.0-rc.13
  collision: config:knobs → won by dev/1.0.0-rc.13
  Pack input: house/fs-local = /tmp/housepack-Ec9Bpg  (blake3 043b33095d13053abc118ff1e1f331f79508997f2581428bcee527c9cd6643e1)
  Pack input: dev/1.0.0-rc.13 = <embedded>  (blake3 65244b6a89b38336f43ac81fd9f51b0d74eb22b83c64067ada60e7d52eee83a5)
  Pack input: methodology/1.0.0-rc.13 = <embedded>  (blake3 b9fbe7bc56cbfbf4804bcfdc958a6167ea236d8b65fb3c02293ce4f52b897716)
  overrides applied: 1
    invocation-log = true    (project)
  includes:
    step:present-catalog    (pack-default)
    step:route-to-workflow    (pack-default)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    --explain names the adjudicated collision and its winner
  OK    …and still names the house pack as a composed input
  OK    validate stays clean under the demotion
  OK    the ungoverned note still resolves from the listed pack (per-id demotion, not per-pack)

=== B · BOUND 2, measured — the un-markered M14 path has no demotion
doctype: adr
fields:
  - status: enum [proposed|accepted|superseded] (section: status) (default: proposed) (set-field: adr:<slug>#status/status)
  - date: date (section: status) (set: on-create) (set-field: adr:<slug>#status/date)
sections:
  - context: slot (set-slot: adr:<slug>#context)
  - decision: slot (set-slot: adr:<slug>#decision)
  - consequences: slot (set-slot: adr:<slug>#consequences)
[exit 0]
  MEASURED · un-markered list: the house fork renders at exit 0 with options ABSENT — bound 2 as declared
  OK    marker + list restored: the frozen adr is back

=== SUMMARY
  cells: extension (note wins) · demotion (adr loses, named) · methodology retained · bound 1 driven · bound 2 measured
  the one expected FAIL is the missing-catalog fault's wording (see the header) — a driven observation
ARM 20 FAIL
(the arm exited 1 — that is data, not necessarily failure)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/20-project-pack-composition (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 41
  doc show --task    : 2
  doc show (any)     : 3
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/M50-walk-final/20-project-pack-composition"
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.

/tmp/arm.sh: line 121: commands: command not found
```

invocation log: 41 records · `doc show … --task` **2** · §3.3 adjacent 0

## `21-migrate-continue-on-rc13.sh`

exit **0**

```
image      : jigc-gate:rc13 (jigc 1.0.0-rc.13)
jigc sha   : 979bacaf31cbf513ca8afcb0bade447a88c06ba1
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-m50
out        : /Users/maurice/out/M50-walk-final/21-migrate-continue-on-rc13
container  : abad47be35db8827ad471105a8924a9f64d15be8f7732c13fe8324bcdb6799db
  (a mid-stream plant runs with: docker exec -it -u node abad47be35db8827ad471105a8924a9f64d15be8f7732c13fe8324bcdb6799db bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/21-migrate-continue-on-rc13.sh in the container
-------------------------------------------------------------

=== SKIPPED — no baseline from the first half in this corpus

This arm continues the corpus `14-migrate-author-on-rc12.sh` left behind. Run:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc12 --only 14
  python3 run.py  carry <out-a>/14-migrate-author-on-rc12 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc13 --only 21
ARM 21 SKIPPED (no .upgrade-baseline — the rc.12 half has not run)
-------------------------------------------------------------
evidence in /Users/maurice/out/M50-walk-final/21-migrate-continue-on-rc13 (corpus + .session-transcript/ + PROVENANCE.txt)
  NO INVOCATION LOG — §3.3's primary channel is missing for this session
```

stderr:

```

NOTE: bypassPermissions is the operator's chosen default (2026-08-16). It removes prompt
friction from file reads while `jigc setup` allowlists Bash(jigc:*) — i.e. from one side
of the asymmetry the headline measurement is about. Declared in protocol.md §9 as a
directional confound, not assumed harmless. `--strict-permissions` runs the adopter's
real condition instead.
```

