# jigc CLI surface corpus — round 2a (milestone lifecycle · operator verbs · gate surfaces)

Binary: `~/.local/bin/jigc` — `jigc 1.0.0-rc.7`. All output verbatim stdout+stderr; exit code noted when non-zero.

## $ jigc milestone create --title "Cache Rework"

```
error: unexpected argument '--title' found

  tip: to pass '--title' as a value, use '-- --title'

Usage: jigc milestone create [OPTIONS] <TITLE>

For more information, try '--help'.
```

(exit code: 2)

## $ jigc milestone create "Cache Rework"

```
minted milestone:cache-rework (shared base 29647af)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone add-task cache-rework "add the LRU eviction policy"

```
added task:add-the-lru-eviction-policy to milestone:cache-rework
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone add-task cache-rework "wire cache metrics into the stats endpoint"

```
added task:wire-cache-metrics-into to milestone:cache-rework
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone list-tasks cache-rework

```
milestone:cache-rework tasks (2): add-the-lru-eviction-policy, wire-cache-metrics-into
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone provision cache-rework

```
provisioned 2 worktree(s) for milestone:cache-rework at base 29647af (add-the-lru-eviction-policy, wire-cache-metrics-into)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone execute cache-rework

```
Provision one isolated worktree per sub-task at the milestone's base pin — each
fanned sub-agent runs against its own detached checkout, keyed by task id:

Run: `jigc milestone provision <MILESTONE_ID>`

Spawn a sub-agent per sub-task and implement it. Each runs in its own isolated
working area keyed by task id.
Spawn: `cd .jigc/worktrees/add-the-lru-eviction-policy && jigc workflow sub-task --task add-the-lru-eviction-policy`
Spawn: `cd .jigc/worktrees/wire-cache-metrics-into && jigc workflow sub-task --task wire-cache-metrics-into`

All sub-tasks are complete and have been merged by task-id order. Continue with
the merged effective state.

The parent finalize is the commit boundary for the whole milestone — validate the
merged effective state and commit per the `finalize.fan-out.squash` knob:

Run: `jigc milestone finalize <MILESTONE_ID>`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ cd .jigc/worktrees/add-the-lru-eviction-policy && jigc workflow sub-task --task add-the-lru-eviction-policy

```
Reason about the change. The intent is:
add the LRU eviction policy

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged. When done, set the
required Conventional-Commits type — your editorial call on what this change
does. The subject renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own — the `type` field already
carries it. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected. Set the
type, then stage the summary prose:

Run: `jigc doc set-field commit:add-the-lru-eviction-policy#type --value <COMMIT_TYPE> --task add-the-lru-eviction-policy`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:add-the-lru-eviction-policy#summary --from-file - --task add-the-lru-eviction-policy`
<<author: commit:add-the-lru-eviction-policy#summary>>

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:add-the-lru-eviction-policy#scope --value <area> --task add-the-lru-eviction-policy
jigc doc set-slot commit:add-the-lru-eviction-policy#body --from-file - --task add-the-lru-eviction-policy

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:add-the-lru-eviction-policy#trailers --title Co-Authored-By --task add-the-lru-eviction-policy
jigc doc set-field commit:add-the-lru-eviction-policy#trailers/<id>/value --value "Name <email>" --task add-the-lru-eviction-policy

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task add-the-lru-eviction-policy`

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects):

jigc doc set-slot adr:<slug>#context --from-file - --task add-the-lru-eviction-policy
jigc doc set-slot adr:<slug>#decision --from-file - --task add-the-lru-eviction-policy
jigc doc set-slot adr:<slug>#consequences --from-file - --task add-the-lru-eviction-policy

The `options` slot is optional — fill it only when alternatives were genuinely
weighed; omit it when the call was obvious:

jigc doc set-slot adr:<slug>#options --from-file - --task add-the-lru-eviction-policy

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

Author this sub-task's own commit prose. The parent milestone's finalize is
the only commit boundary — never run git here. Set the required
Conventional-Commits type — your editorial call on what this change does. The
subject renders as `<type>(<scope>): <summary>`, so write the summary without
a type or scope prefix of its own — the `type` field already carries it. Set
the type, then stage the summary (`scope` and `body` are optional):

Run: `jigc doc set-field commit:add-the-lru-eviction-policy#type --value <COMMIT_TYPE> --task add-the-lru-eviction-policy`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:add-the-lru-eviction-policy#summary --from-file - --task add-the-lru-eviction-policy`
<<author: commit:add-the-lru-eviction-policy#summary>>
resume: `jigc start --task add-the-lru-eviction-policy`   — re-composes this workflow if context is lost
what's-left: `jigc task validate add-the-lru-eviction-policy`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task add-the-lru-eviction-policy` is the explicit override and wins when several are active
create-gates: adr
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc doc set-field commit:add-the-lru-eviction-policy#type --value feat --task add-the-lru-eviction-policy   # in the sub-task worktree

```
set commit:add-the-lru-eviction-policy#type = feat
```

## $ echo "add the LRU eviction policy to the cache" | jigc doc set-slot commit:add-the-lru-eviction-policy#summary --from-file - --task add-the-lru-eviction-policy

```
set slot commit:add-the-lru-eviction-policy#summary (41 chars)
```

## $ jigc milestone join cache-rework

```
joined milestone:cache-rework — 1 doc(s) merged
  - commit:add-the-lru-eviction-policy  (created · from add-the-lru-eviction-policy)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone finalize cache-rework   # with one of two sub-tasks incomplete

```
```

## $ git show --stat --format=medium HEAD   # the milestone finalize landing (finalize printed nothing)

```
commit b546ca88a6d75b825171dcd9d1b07b5c0c063167
Author: Cap <cap@example.com>
Date:   Fri Jul 17 18:24:59 2026 +0200

    Finalize milestone cache-rework (2 sub-tasks)
    
    - add-the-lru-eviction-policy
    - wire-cache-metrics-into

 docs/milestone-records/cache-rework.md | 6 +++---
 lru.py                                 | 3 +++
 2 files changed, 6 insertions(+), 3 deletions(-)
```

## $ jigc milestone add-from-spec rework-two spec:cache-rework-spec   # spec committed with zero criteria

```
spec `spec:cache-rework-spec` has no criteria to seed from
  route: add `criteria` items to the spec, or add sub-tasks with `jigc milestone add-task`
```

(exit code: 1)

## $ jigc milestone create "Doomed Effort"   # in a second repo, for the discard flow

```
minted milestone:doomed-effort (shared base 6bc9d52)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone add-task doomed-effort "rewrite the parser"

```
added task:rewrite-the-parser to milestone:doomed-effort
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc milestone discard doomed-effort   # uncommitted work sitting in a provisioned worktree

```
milestone:doomed-effort has uncommitted work in 1 sub-task worktree(s) — discarding it would destroy that work:
  /home/maurice/.claude/jobs/0c0984fb/tmp/r2-discard/.jigc/worktrees/rewrite-the-parser: ?? wip.txt
  route: get the work out of those worktrees first (commit, stash, or copy it), then re-run `jigc milestone discard doomed-effort` — or re-run with `--force` to abandon the milestone and destroy the uncommitted work
```

(exit code: 1)

## $ jigc milestone discard doomed-effort --force

```
discarded milestone:doomed-effort (1 sub-task(s); workbench removed)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc doc set-field adr:drop-mysql#supersedes --value adr:use-postgres --task drop-the-mysql-fallback   # supersedes already carries one ref

```
blocking · write.list-overwrite — write rejected: field "supersedes" already has 1 value(s); `set-field` replaces the whole list and would silently drop them. To set multiple values, pass them all in one call: --value "[adr:ghost-decision, adr:use-postgres]"
  route: re-run `jigc doc set-field` with the inline-list form shown in the message, carrying every value to keep
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 1)

## $ jigc task finalize drop-the-mysql-fallback   # the adr supersedes a nonexistent doc

```
blocking · schema-conformance.ref-resolves — forward-ref integrity — `adr:drop-mysql#supersedes` target `adr:ghost-decision` resolves in neither the committed store nor this task's working area; resolution: fix the reference to an existing target, create the target in this task, or drop the `supersedes` field
  route: fix the reference, create the target in this task, or drop the field
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc rename adr:use-postgres --to "Adopt PostgreSQL"   # a committed adr with one inbound ref

```
renamed adr:use-postgres -> adr:adopt-postgresql (docs/decisions/use-postgres.md -> docs/decisions/adopt-postgresql.md), repointed 1 referrer(s)
  repointed adr:drop-mysql#supersedes
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc validate   # after a conformant out-of-band prose edit to a committed adr

```
blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions/adopt-postgresql.md` differs from the recorded state
  route: review the out-of-band edit to `docs/decisions/adopt-postgresql.md` and re-author it through the owning workflow
1 finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize`.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc task validate trim-the-readme-heading   # task-scope validate while the out-of-band edit stands

```
blocking · schema-conformance.field-value-conformant — `commit:trim-the-readme-heading`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  route: `jigc doc set-field commit:trim-the-readme-heading#header/type --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:trim-the-readme-heading`: required slot in section `summary` is empty
  route: `jigc doc set-slot commit:trim-the-readme-heading#summary --from-file -` to fill the empty slot
advisory · reconciliation.absorb — external edit absorbed: `docs/decisions/adopt-postgresql.md`
  route: no action needed — the external edit was absorbed into the baseline
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc validate   # after an out-of-band structural edit (a schema section heading demoted)

```
blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions/adopt-postgresql.md` differs from the recorded state
  route: review the out-of-band edit to `docs/decisions/adopt-postgresql.md` and re-author it through the owning workflow
blocking (gates at finalize) · conformance.section-renamed — `docs/decisions/adopt-postgresql.md`: section heading "Consequences" does not match required section `decision`
blocking (gates at finalize) · conformance.section-missing — `docs/decisions/adopt-postgresql.md`: required section heading `## consequences` is missing
3 finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize`.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc task validate trim-the-readme-heading   # task-scope, the structural out-of-band edit standing

```
blocking · schema-conformance.field-value-conformant — `commit:trim-the-readme-heading`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  route: `jigc doc set-field commit:trim-the-readme-heading#header/type --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:trim-the-readme-heading`: required slot in section `summary` is empty
  route: `jigc doc set-slot commit:trim-the-readme-heading#summary --from-file -` to fill the empty slot
blocking · reconciliation.conformance-block — nonconformant edit on `docs/decisions/adopt-postgresql.md`: section heading "Consequences" does not match required section `decision`
  route: fix the file to restore conformance, or revert the edit
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc task finalize trim-the-readme-heading   # commit doc complete, nothing staged

```
blocking · finalize.empty-commit — task validated but produced no diff — nothing to finalize
  route: make a change, then re-run `jigc task finalize`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc task finalize choose-the-cache-strategy   # a foreign file appeared at the adr's promote destination

```
blocking · finalize.promote-clobber — promoting this task's doc to `docs/decisions/cache-strategy.md` would overwrite a file already there — refusing to clobber it
  route: a file already occupies `docs/decisions/cache-strategy.md`: if it is another managed doc, retitle this task's doc so it slugs differently, or re-create it with an explicit `--slug` (`jigc doc create <doctype> --title <title> --slug <slug> --task <id>`); if it is a hand-authored/foreign file, bring it under management with `jigc migrate docs/decisions/cache-strategy.md --as <doctype>` in its own task; then re-run `jigc task finalize`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 3)

## $ jigc doc retitle-item changelog:changelog#releases/1-0-0/changes/added --to fixed --task note-the-cache-change   # a change-group id minted from an enum

```
error: unexpected argument '--to' found

  tip: to pass '--to' as a value, use '-- --to'

Usage: jigc doc retitle-item --title <TITLE> <ADDR>

For more information, try '--help'.
```

(exit code: 2)

## $ jigc doc retitle-item changelog:changelog#releases/1-0-0/changes/added --title fixed --task note-the-cache-change   # a change-group id minted from an enum

```
blocking · write.identity-change — retitle-item rejected: item `releases/1-0-0/changes/added` derives its id from enum field `category` — a member change is an identity change, not a retitle
  route: run `jigc doc remove-item changelog:changelog#releases/1-0-0/changes/added` then `jigc doc add-item changelog:changelog#releases/1-0-0/changes --title "fixed"` under the target category, moving the prose in the same motion
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

(exit code: 1)

## $ jigc ingest   # mixed repo: a conformant adr at the managed home, a foreign doc beside it, plain off-home files

```
jigc ingest — 6 candidate(s) classified  (sorted — deterministic report order)

adoptable docs/decisions/keep-the-monolith.md → adr  (adopted — indexed + baselined, no file moved)
needs-reconcile docs/decisions/old-notes.md → adr
  blocking · conformance.section-missing — required section heading `## context` is missing
  route: reconcile docs/decisions/old-notes.md against the `adr` schema
unmanaged ./ — 3 file(s) parse against no schema (left untouched — fine to stay plain)
unmanaged notes/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)

What the verdicts above mean, and what to do next:
  adoptable — conformant at its managed location; adopted register-only (indexed + baselined, the file stays in place).
  needs-reconcile — parses as the named type but conflicts; fix it per the row's route, then re-run `jigc ingest`.
  unmanaged — matches no managed schema; staying a plain file is a legitimate end-state — no action needed. To bring one under management: `jigc migrate <path> --as <doctype>`.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc migrate-corpus   # every committed doc already at current schema-version

```
corpus migration: 0 migrated, 3 already current, 0 blocked
  current    docs/decisions/adopt-postgresql.md
  current    docs/decisions/cache-strategy.md
  current    docs/decisions/drop-mysql.md
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc migrate-corpus --dry-run

```
corpus migration: 0 migrated, 3 already current, 0 blocked
  current    docs/decisions/adopt-postgresql.md
  current    docs/decisions/cache-strategy.md
  current    docs/decisions/drop-mysql.md
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc upgrade   # no recorded config deltas

```
no findings — the task validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc unmanage docs/decisions/cache-strategy.md

```
unmanaged docs/decisions/cache-strategy.md (adr:cache-strategy) — dropped its file-state baseline + forward edges; the file is left on disk
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc unmanage docs/decisions/cache-strategy.md   # idempotent re-run

```
no-op: docs/decisions/cache-strategy.md is not managed (nothing to drop)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## $ jigc relocate adr --from docs/adr   # a frozen doctype

```
`adr` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`, not the freeze-exempt path
```

(exit code: 1)

## $ jigc config set not-a-knob x

```
`not-a-knob` is not a settable knob — the cascade surface is closed
  route: run `jigc start` to orient; settable knobs are declared by the pack
```

(exit code: 1)

## $ jigc config remove-step single-task   # not a workflow:<id>#<step-id> address

```
structural-op target needs a `workflow:<id>` scheme
  route: correct the target to the delta-target grammar — `workflow:<id>#<step-id>` to name a step position, or `workflow:<id>` with an `after:`/`before:` anchor — in the `jigc config <verb>` argument that passed it or the recorded `deltas:` entry that carries it
```

(exit code: 1)

## $ echo "extra guidance" | jigc config fill step:no-such-step#extras --from-file -

```
`step:no-such-step#extras` is not a `{{fill:}}` point in the resolved `no-such-step` step body
  route: name a `{{fill:<fill-id>}}` point the step body declares (run `jigc start` to see the composed step bodies), then re-run
```

(exit code: 1)

## $ jigc config set docs-root docs2

```
relocating 3 committed doc(s) stranded by the `docs-root` re-point to `docs2`:
  - docs/decisions/adopt-postgresql.md → docs2/decisions/adopt-postgresql.md
  - docs/decisions/cache-strategy.md → docs2/decisions/cache-strategy.md
  - docs/decisions/drop-mysql.md → docs2/decisions/drop-mysql.md
```

## $ jigc upgrade   # with the docs-root delta recorded

```
no findings — the task validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

## Capture notes

- Binary: `~/.local/bin/jigc` — version confirmed `jigc 1.0.0-rc.7` before capture.
- Repos (all throwaway, under `/home/maurice/.claude/jobs/0c0984fb/tmp/`): `r2-mile` (milestone lifecycle end-to-end + add-from-spec), `r2-discard` (discard refusal + `--force`), `r2-gates` (dangling-ref finalize · list-overwrite · retitle-item enum-id · reconciliation · empty-commit · promote-clobber · rename · migrate-corpus · upgrade · unmanage · relocate · config rejections), `r2-ingest` (mixed-repo ingest funnel).
- `jigc milestone create --title` is not the surface — TITLE is positional; the clap rejection is captured verbatim, followed by the positional form.
- `jigc milestone finalize cache-rework` landed with EMPTY stdout/stderr and exit 0 — the landing is evidenced by the follow-up `git show --stat` capture (commit `Finalize milestone cache-rework (2 sub-tasks)`, landing `lru.py` + the milestone record). The second sub-task (`wire-cache-metrics-into`) had done no work and did not block the finalize.
- The second-worktree spawn line was not run — one sub-task exercised end-to-end per the brief; the compose is identical modulo task id.
- "nothing staged" at finalize surfaces as `finalize.empty-commit` (no finding named `finalize.nothing-staged` was observed).
- `jigc doc retitle-item` takes `--title`, not `--to` (the `--to` clap rejection is captured, then the real enum-id refusal `write.identity-change`).
- `jigc rename` takes `--to` (captured with 1 inbound referrer repointed).
- Reconciliation captured both ways: a conformant OOB prose edit (store-scope `file-state.hash-matches`, task-scope `reconciliation.absorb` advisory) and a structural OOB edit (store-scope `conformance.section-renamed`/`section-missing`, task-scope `reconciliation.conformance-block`).
- `adr` create is gate-blocked under `decided-task` (allows `decisions-log` only) — the dangling-ref/list-overwrite tasks were minted under `single-task` instead; that gate-blocked path was hit during prep but not captured (round-1 scope).
- `jigc upgrade` prints `no findings — the task validates clean` both with zero deltas and with the recorded `docs-root` scalar delta — captured verbatim as printed.
- `jigc config set docs-root docs2` triggered the detect+route+move relocation of all 3 committed decisions (including the just-unmanaged `cache-strategy.md`) — captured verbatim.
- add-from-spec zero-criteria block WAS reachable: a `plan`-workflow task committed a criteria-less spec, then `jigc milestone add-from-spec rework-two spec:cache-rework-spec` blocked with the "no criteria to seed from" message + route (the printed text does not carry a `milestone.no-criteria` key on the agent format).
- Nothing in the brief was unreachable.
