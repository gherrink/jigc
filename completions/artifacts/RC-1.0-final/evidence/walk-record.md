# Operator walk — recorded per arm

One block per arm. An arm that did not run says so, with the command that
would run it: a narrative walk loses a chartered probe silently, a table
cannot.

| arm | ran | exit | evidence |
|---|---|---|---|
| `00-positive-control.sh` | earlier pass | 0 | `00-positive-control/` |
| `01-destination-occupancy.sh` | this pass | 0 | `01-destination-occupancy/` |
| `02-destroying-doors.sh` | this pass | 0 | `02-destroying-doors/` |
| `03-upgrade-author-on-rc11.sh` | this pass | 0 | `03-upgrade-author-on-rc11/` |
| `04-foreign-arm.sh` | this pass | 0 | `04-foreign-arm/` |
| `05-milestone-boundary.sh` | this pass | 0 | `05-milestone-boundary/` |
| `06-pre-guard-repair.sh` | this pass | 0 | `06-pre-guard-repair/` |
| `07-changelog-gate.sh` | this pass | 0 | `07-changelog-gate/` |
| `08-identity-refusals.sh` | this pass | 0 | `08-identity-refusals/` |
| `09-increment-8-doors.sh` | this pass | 0 | `09-increment-8-doors/` |
| `10-upgrade-continue-on-rc12.sh` | this pass | 0 | `10-upgrade-continue-on-rc12/` |

## `00-positive-control.sh`

Run in an **earlier pass**, exit **0**. Its evidence is in `00-positive-control/`; delete that directory to re-run.

invocation log: 10 records · `doc show … --task` **1** · §3.3 adjacent 2

## `01-destination-occupancy.sh`

exit **0**

```
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/01-destination-occupancy
container  : 33282d331c08815ae4cd5fd9da89a84bf66e6d240141b41a95a9f53cee7a4a17
  (a mid-stream plant runs with: docker exec -it -u node 33282d331c08815ae4cd5fd9da89a84bf66e6d240141b41a95a9f53cee7a4a17 bash -l)

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
      - install commit → e1ae854   (setup's install files are committed on their own, off your first feature commit)
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
      route: `jigc doc set-slot adr:bound-the-number-of-distinct#context --from-file -` to fill the empty slot
    blocking · schema-conformance.required-slot-present — `docs/decisions/bound-the-number-of-distinct.md`: required slot in section `decision` is empty
      route: `jigc doc set-slot adr:bound-the-number-of-distinct#decision --from-file -` to fill the empty slot
    blocking · schema-conformance.required-slot-present — `docs/decisions/bound-the-number-of-distinct.md`: required slot in section `consequences` is empty
      route: `jigc doc set-slot adr:bound-the-number-of-distinct#consequences --from-file -` to fill the empty slot
    blocking · schema-conformance.field-value-conformant — `commit:decisions-that-will-collide`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
      route: `jigc doc set-field commit:decisions-that-will-collide#header/type --value <value>` to correct the value
    blocking · schema-conformance.required-slot-present — `commit:decisions-that-will-collide`: required slot in section `summary` is empty
      route: `jigc doc set-slot commit:decisions-that-will-collide#summary --from-file -` to fill the empty slot
    — jigc · run `jigc start` for orientation; all writes through `jigc`.
--- exit: 3
--- $ jigc doc list
    jigc doc list — no committed docs
    note: docs are also staged in open task decisions-that-will-collide — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list --task decisions-that-will-collide`
--- exit: 0

=== 7 · top-level rename onto an existing identity
--- $ jigc rename adr:bound-the-number-of-distinct --to Bound the number of distinct series
    no managed doc `adr:bound-the-number-of-distinct` to rename (expected at docs/decisions/bound-the-number-of-distinct.md)
      route: check the id (or run `jigc describe` for the doctype surface)
--- exit: 1
top-level self-rename exit: 1

=== done
arm exit: 0   (0 = every step recorded an exit code and nothing silently succeeded)
-------------------------------------------------------------
evidence in /Users/maurice/out/TRIAL-walk/01-destination-occupancy (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 10
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 2   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/01-destination-occupancy"
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

exit **0**

```
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/02-destroying-doors
container  : 64a916b9ec127e08784edbf92080ee8e7a0fba388c47e6daf9e64880eabe7b77
  (a mid-stream plant runs with: docker exec -it -u node 64a916b9ec127e08784edbf92080ee8e7a0fba388c47e6daf9e64880eabe7b77 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/02-destroying-doors.sh in the container
-------------------------------------------------------------

=== 0 · adopt, and commit the ignore rules BEFORE provisioning (see the order trap)
jigc 1.0.0-rc.12

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
  route: `jigc milestone provision bound-the-store --force` — but look inside that directory first and move out anything you need; the removal is permanent
[exit 1]

$ jigc milestone discard bound-the-store
blocking · milestone.dirty-worktree — milestone:bound-the-store: 1 sub-task worktree path(s) hold content, and `jigc milestone discard` would settle the record and tear the workbench down over them:
  /work/.jigc/worktrees/cap-distinct-series: .claude, .git, .gitignore, .jigc, CLAUDE.md, README.md, build, package.json, secrets.env, src, staged.ts, test, untracked.ts — not registered here, so the teardown leaves it on disk with no milestone naming it
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
provisioned 2 worktree(s) for milestone:bound-the-store at base 930b302 (cap-distinct-series, prune-on-overflow)
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
finalized 6818f43 — Finalize milestone bound-the-store-again (1 sub-task)
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

=== SUMMARY
  doors this arm was written against : 4 (3 refusing + 1 narrate-only)
  the registry fence lives in flow49_acceptance.rs, not here
ARM 02 PASS
-------------------------------------------------------------
evidence in /Users/maurice/out/TRIAL-walk/02-destroying-doors (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 1
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/02-destroying-doors"
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/03-upgrade-author-on-rc11
container  : 17920ecc4913fd29b34da546377b15a30c2f7ba3330fbbac9f5622aee8017327
  (a mid-stream plant runs with: docker exec -it -u node 17920ecc4913fd29b34da546377b15a30c2f7ba3330fbbac9f5622aee8017327 bash -l)

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
ARM 03 SKIPPED (needs jigc-gate:rc11; got jigc 1.0.0-rc.12)
-------------------------------------------------------------
evidence in /Users/maurice/out/TRIAL-walk/03-upgrade-author-on-rc11 (corpus + .session-transcript/ + PROVENANCE.txt)
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/04-foreign-arm
container  : 042e49710bf873184ce22eee140fbf3d3fd483bc8953f565bc1db693d43e3d3b
  (a mid-stream plant runs with: docker exec -it -u node 042e49710bf873184ce22eee140fbf3d3fd483bc8953f565bc1db693d43e3d3b bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/04-foreign-arm.sh in the container
-------------------------------------------------------------

=== 0 · adopt jigc onto a repo that already has documents (the adopter's day one)
jigc 1.0.0-rc.12

=== A · the store sweep — protocol §0.1's declared change

$ jigc validate
note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
advisory · schema-conformance.unadopted-instance — committed file `docs/decisions/use-an-in-memory-buffer.md` sits at the `adr` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `adr` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate docs/decisions/use-an-in-memory-buffer.md --as adr` to rewrite it into the managed `adr` shape; it is a foreign file, not an unmigrated managed doc
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `changelog` schema version
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
  route: `jigc migrate CHANGELOG.md --as changelog` — it opens the `migrate-changelog` workflow, which rewrites the file to conformant shape and adopts it at finalize
needs-reconcile docs/decisions/use-an-in-memory-buffer.md → adr
  blocking · conformance.section-renamed — section heading "Status" does not match required section `context`
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
  route: migrate — `docs/decisions/prefer-a-bounded-buffer.md` is a managed doc below the current schema-version 2; run `jigc migrate-corpus` to upgrade it
the committed corpus is below its schema-version — every other finding above was adjudicated against a schema those docs were never written to, so the sweep exits non-zero; run `jigc migrate-corpus`, then re-validate.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 1]

$ jigc migrate-corpus
corpus migration: 1 migrated, 0 already current, 0 blocked
  migrated   docs/decisions/prefer-a-bounded-buffer.md
committed f2d9e85 — only the migrated paths were staged
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
evidence in /Users/maurice/out/TRIAL-walk/04-foreign-arm (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 19
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/04-foreign-arm"
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/05-milestone-boundary
container  : 9c861a69312a364c3097292e7b1ad780e9af7d8b1efbfc6b870910b436ccd347
  (a mid-stream plant runs with: docker exec -it -u node 9c861a69312a364c3097292e7b1ad780e9af7d8b1efbfc6b870910b436ccd347 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/05-milestone-boundary.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.12

=== 1 · OWN linkage — the ordinary success path, as the baseline
finalized 0b34385 — Finalize milestone bound-the-store (1 sub-task)
  added .jigc/config/manifest.yaml
  modified docs/milestone-records/bound-the-store.md
  added src/cap.ts
  3 files committed
  sub-tasks: cap-distinct-series: 1 code file
— jigc · run `jigc start` for orientation; all writes through `jigc`.
  OK    the sub-task's staged code lands

=== 2 · FOREIGN linkage — a cp -R of a repo whose SOURCE still exists
finalized 78a9c20 — Finalize milestone bound-the-store (1 sub-task)
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
  route: `jigc milestone provision bound-the-store` gives every sub-task a working area (a fresh clone has none; an existing one is reused); execute the sub-tasks, `git add` their work inside their worktrees, then re-run `jigc milestone finalize bound-the-store` — or settle the milestone as abandoned with `jigc milestone discard bound-the-store`
  OK    it refuses rather than landing a bookkeeping-only commit
  OK    the refusal is milestone.zero-contribution
  OK    …and says what landing anyway would cost — the terminal record
  OK    …and routes at provision, the verb that repairs it

=== 3b · NO linkage, but docs DO land — it proceeds and NAMES what it could not count
finalized 3d14c1b — Finalize milestone bound-the-store (1 sub-task)
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
    "hash": "3d14c1b",
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
evidence in /Users/maurice/out/TRIAL-walk/05-milestone-boundary (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 1
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/05-milestone-boundary"
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/06-pre-guard-repair
container  : 4d7697a93f0614abda8a83b4c2c15eb9b3e95b72db47e8e74199cd0321159e3c
  (a mid-stream plant runs with: docker exec -it -u node 4d7697a93f0614abda8a83b4c2c15eb9b3e95b72db47e8e74199cd0321159e3c bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/06-pre-guard-repair.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.12

=== 0 · land a committed spec — a doctype with a repeatable item section
spec: spec:bound-the-distinct-series-count
  OK    the spec is committed

=== 1 · a human hand-adds an item heading with no anchor — the pre-guard state

$ jigc validate
blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/specs/bound-the-distinct-series-count.md` differs from the recorded state
  route: review the out-of-band edit to `docs/specs/bound-the-distinct-series-count.md` and re-author it through the owning workflow
blocking (gates at finalize) · conformance.item-heading-unanchored — `docs/specs/bound-the-distinct-series-count.md`: `### A hand-added criterion` sits at `###`, the schema-reserved item depth here, so the parser reads it as an item boundary — and it carries no `{#id}` anchor; if that line is slot prose, demote it to `####` or deeper; if it is a new item, anchor it in place — `### A hand-added criterion  {#<id>}` — with `<id>` a lowercase-kebab slug unique among this section's items
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
evidence in /Users/maurice/out/TRIAL-walk/06-pre-guard-repair (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 15
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/06-pre-guard-repair"
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/07-changelog-gate
container  : cffb63cdadb603ec6976ae90064ce7ecf9e7bfc4522c81716cb450efabcc29c6
  (a mid-stream plant runs with: docker exec -it -u node cffb63cdadb603ec6976ae90064ce7ecf9e7bfc4522c81716cb450efabcc29c6 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/07-changelog-gate.sh in the container
-------------------------------------------------------------

=== 0 · adopt, mint a gate-granting task, and fill its commit doc
jigc 1.0.0-rc.12
task: add-a-rate-limiter

=== A · the declared change: a conformant task no longer validates silent

$ jigc task validate add-a-rate-limiter
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
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
evidence in /Users/maurice/out/TRIAL-walk/07-changelog-gate (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 23
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 8   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/07-changelog-gate"
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/08-identity-refusals
container  : 2da893f93d39fc7d7eaaa2de4092f0ce5ea246170a20bbe370f00f550e74f6b3
  (a mid-stream plant runs with: docker exec -it -u node 2da893f93d39fc7d7eaaa2de4092f0ce5ea246170a20bbe370f00f550e74f6b3 bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/08-identity-refusals.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.12

=== T10 · doc rename against a COMMITTED identity — the refusal, not exit 0
committed: adr:drop-the-oldest-sample

$ jigc doc rename adr:drop-the-oldest-sample --to Evict the oldest sample on overflow --task revise-the-overflow-policy
blocking · write.identity-change — rename rejected: `adr:drop-the-oldest-sample` is committed, so `--to "Evict the oldest sample on overflow"` would move its identity to `evict-the-oldest-sample` — a committed doc's path IS its identity, and referrers outside this task point at the old one. A same-slug retitle of the staged copy is supported; a re-slug is not
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
cannot rename to `adr:drop-the-oldest-sample` — a different doc already exists at docs/decisions/drop-the-oldest-sample.md
[exit 1]
  OK    the destination-occupancy guard refuses
  OK    it names the occupied destination
  OK    nothing was moved — the loser is intact
  OK    …and so is the incumbent

=== OCCUPANCY · the sibling refusal on the SAME verb, for comparison

$ jigc rename adr:no-such-doc --to Something else
no managed doc `adr:no-such-doc` to rename (expected at docs/decisions/no-such-doc.md)
  route: check the id (or run `jigc describe` for the doctype surface)
[exit 1]
  OK    the unknown-doc refusal on this same verb DOES carry a route
  OBSERVE  the occupancy refusal above carries NO route line:
           | cannot rename to `adr:drop-the-oldest-sample` — a different doc already exists at docs/decisions/drop-the-oldest-sample.md           -> recorded in pre-trial-findings.md as a candidate finding.
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
evidence in /Users/maurice/out/TRIAL-walk/08-identity-refusals (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 40
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/08-identity-refusals"
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/09-increment-8-doors
container  : f1946608a3b771025a71d4754708309d8232927f79bdc9abe51f4cc82b0df0fb
  (a mid-stream plant runs with: docker exec -it -u node f1946608a3b771025a71d4754708309d8232927f79bdc9abe51f4cc82b0df0fb bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/09-increment-8-doors.sh in the container
-------------------------------------------------------------
jigc 1.0.0-rc.12

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
task `cap-distinct-series` is pinned to base a8d614b but you're on 596210b — this is a sub-task of milestone `bound-the-store`, and a sub-task's work happens in its own worktree at .jigc/worktrees/cap-distinct-series, cut from that base rather than in this checkout: run `jigc milestone provision bound-the-store` — it adds a worktree that is missing and leaves one that exists untouched — then re-run this from that worktree
[exit 1]
  OK    it explains the pin rather than just refusing
  OK    …names the worktree the work belongs in
  OK    …and routes at provision — NOT at task discard, which used to exit 0 on a live record
  OK    …and does not offer discarding the sub-task

=== T6 · the router's closing text — the increment's own audit finding, re-measured

$ tail -6 /tmp/router.txt

That catalog is the selectable subset. A workflow outside it is reached by name
with the same `--workflow` form — `jigc describe --workflows` lists every
workflow, hidden ones included, and each hidden one carries the reason it is
hidden.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
[exit 0]
  OK    the closing text points at describe --workflows for the fuller set

  MEASURED · workflows absent from the router catalog, and whether each states why:
           catalog: 12 selectable · absent: 21 · of those, WITHOUT a stated reason: 3
           without a reason: increment, ingest-existing, router
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

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects). Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address:

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
address. Set the type, then stage the summary prose:

Run: `jigc doc set-field commit:your-task-id#type --value <COMMIT_TYPE> --task your-task-id`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:your-task-id#summary --from-file - --task your-task-id`
<<author: commit:your-task-id#summary>>

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
evidence in /Users/maurice/out/TRIAL-walk/09-increment-8-doors (corpus + .session-transcript/ + PROVENANCE.txt)
  invocation records : 21
  doc show --task    : 0
  doc show (any)     : 0
  §3.3 adjacent      : 0   (task diff · task validate · doc list --task)
  ^ indicative only — score with: completions/trial-driver/run.py observe "/Users/maurice/out/TRIAL-walk/09-increment-8-doors"
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
image      : jigc-gate:rc12 (jigc 1.0.0-rc.12)
jigc sha   : 314f59ecc1c32c0ccf16685b83f2797fd2e13fc2
model      : claude-sonnet-5
permissions: bypassPermissions
corpus     : /Users/maurice/ideas/walk-trial
out        : /Users/maurice/out/TRIAL-walk/10-upgrade-continue-on-rc12
container  : 4a8a3de72dd626857a40d0e30ce30c9a05777f2aa7bc90dd285141774099ebaf
  (a mid-stream plant runs with: docker exec -it -u node 4a8a3de72dd626857a40d0e30ce30c9a05777f2aa7bc90dd285141774099ebaf bash -l)

running /Users/maurice/projects/gherrink-jigc/completions/trial-driver/arms/walk/10-upgrade-continue-on-rc12.sh in the container
-------------------------------------------------------------

=== SKIPPED — no baseline from the first half in this corpus

This arm continues the corpus `03-upgrade-author-on-rc11.sh` left behind. Run:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc11 --only 03
  python3 run.py  carry <out-a>/03-upgrade-author-on-rc11 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc12 --only 10
ARM 10 SKIPPED (no .upgrade-baseline — the rc.11 half has not run)
-------------------------------------------------------------
evidence in /Users/maurice/out/TRIAL-walk/10-upgrade-continue-on-rc12 (corpus + .session-transcript/ + PROVENANCE.txt)
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

