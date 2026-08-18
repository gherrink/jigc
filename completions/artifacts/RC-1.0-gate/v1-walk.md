# The operator walk — arms 0–4, driven 2026-08-18

Every arm run against `jigc-gate:rc11` in the isolated rig, verbatim output captured as it ran.
`verify-pair.sh` was run first, as [protocol.md](protocol.md) §5 arm 2 requires: **both binaries run,
the old one is the pre-M48 tree, so the upgrade arm is not vacuous.**

**Exit codes below are measured unpiped.** Two of my first measurements were `head`'s status rather
than jigc's; they were re-measured before anything was recorded.

---

## Arm 0 · the positive control — **PASS**

Run 2026-08-16 in the rig, recorded in [pre-trial-findings.md](pre-trial-findings.md). The VERB
channel records, greps, and survives copy-out, so §3.4's *"control did not fire"* row is closed.

---

## Arm 1 · the declared breaking change — **PASS on all three doors**

The one deliberate regression an adopter meets. Judged against the standard §5 arm 1 fixes in
advance: (a) names the path · (b) says what would be lost · (c) names the consent · (d) the consent
works.

Plant: a **non-registered** leftover at `.jigc/worktrees/cap-distinct-series` holding staged,
unstaged and untracked work (a provisioned worktree, then de-registered — the shape a `cp -R` or `mv`
of a repo leaves behind).

```console
$ jigc milestone provision bound-the-store          # exit 1
blocking · milestone.leftover-holds-work — …`/work/.jigc/worktrees/cap-distinct-series` already holds
  11 item(s) that `jigc milestone provision` would delete — git cannot read a repository there, so
  nothing can say those bytes are disposable:
  .claude / .git / .gitignore / .jigc / CLAUDE.md / DESIGN-notes.md / README.md …
  route: `jigc milestone provision bound-the-store --force` — but look inside that directory first
         and move out anything you need; the removal is permanent

$ jigc milestone discard bound-the-store            # exit 1
blocking · milestone.dirty-worktree — …: not registered here, so the teardown leaves it on disk with
  no milestone naming it
  route: … get out what you need … or re-run with `--force`

$ jigc uninstall                                     # exit 1
blocking · uninstall.dirty-worktree — … registered as a worktree nowhere in this repository
  route: … `jigc uninstall --force` deletes them with the install
```

All four criteria met at every door. `--force` was then run verbatim and **destroyed the work as
consented** (exit 0). An **empty** directory at a real sub-task path is **reused, not refused**
(exit 0) — the idempotent case still works.

**Judgment recorded, per §1 row 2: the refusal reads as protection, not obstruction.** Each door
names the exact path, enumerates what it holds, explains *why* it cannot decide the bytes are
disposable, and offers the consent with a warning attached.

---

## Arm 2 · the rc.10 → rc.11 upgrade — **PASS**

A corpus authored end-to-end on **rc.10** (adr · changelog at repo root · milestone-record; 12
commits), then continued on **rc.11**. Covered by nothing else in the suite — the fixture builder
constructs every state with the *current* binary.

```console
$ jigc validate                       # exit 0
advisory · store-version.binary-mismatch — store last written by jigc 1.0.0-rc.10; you are running
  1.0.0-rc.11 — align versions or re-run `jigc setup`

$ jigc setup                          # exit 0
  - jigc guides → .claude/skills/jigc/SKILL.md   (stamped with this build)

$ jigc doc show adr:drop-the-oldest-sample     # exit 0 — rc.11 reads what rc.10 wrote
$ jigc migrate-corpus                          # exit 0 — 0 migrated, 3 already current, 0 blocked
$ jigc task finalize <t>                       # exit 0 — promoted, 1 file committed
```

**The guide artifact installs onto a corpus that predates it**, which §5 arm 2 predicted.
`migrate-corpus` correctly reports *already current* — no schema-version moved between rc.10 and
rc.11, so there is nothing to migrate and it says so rather than inventing work.

### The rename took three attempts, and each refusal was right

```console
$ jigc rename adr:drop-the-oldest-sample --to "Shed the oldest sample on overflow"
exit 1  cannot rename while task `cap-the-number-of-distinct` is in flight — finalize or discard it
        first (a rename changes the by-task-id join key)
exit 1  cannot rename while milestone `bound-the-store` is in flight — …
exit 0  renamed adr:drop-the-oldest-sample -> adr:shed-the-oldest-sample
        (docs/decisions/drop-the-oldest-sample.md -> …/shed-the-oldest-sample.md), repointed 0 referrer(s)
```

Both refusals name the blocker, state the reason, and give a route that **works when followed** — I
followed each in turn (`task discard`, then `milestone discard`) and the third attempt landed, moved
the file, updated the store, and left `validate` at exit 0. Recorded as a **route-followability pass**,
which is what §5 asks arms to demonstrate.

---

## Arm 3 · the guide artifact's clobber refusal — **PASS on both branches**

```console
# guide edited by hand, then:
$ jigc setup            # exit 0
advisory · adapter-guide.user-modified — `.claude/skills/jigc/SKILL.md` no longer carries the bytes
  jigc wrote, so jigc left it untouched rather than clobber your edits …
  route: keep your copy … or delete it and re-run `jigc setup` to reinstall jigc's own copy
```

Not a silent overwrite and not a blocked setup — exactly the middle the protocol asked for. Then the
teardown, tested on **both** branches because "takes it back out **while it is jigc's**" is a claim
about ownership, not about deletion:

| guide state | `jigc uninstall` | result |
|---|---|---|
| jigc's own bytes | exit 0 | `- removed jigc guide artifact` |
| user-edited | exit 0 | **kept**, with `adapter-guide.user-modified`: *"it is yours now, not part of jigc's install"* |

---

## Arm 4 · the verbs a blind session may never reach — **PASS**

```console
$ jigc config get docs-root      docs-root = docs/  (pack-default)
$ jigc config list               default-workflow = router  (pack-default) …
$ jigc describe --workflows      (renders the workflow tour)

$ jigc doc read
error: unrecognized subcommand 'read'
  tip: reading a managed doc is its own verb — `jigc doc list` enumerates the managed docs …;
       `jigc doc show <address>` prints one committed doc … (--task <task-id> to read a task's staged copy)
```

The read-shaped near-miss is answered by M48's read-intent rule: a **read** verb, not a write verb,
and clap's contradicting suggestion is gone.

---

## What the walk did not find

No data loss, no corruption, no dead end, and no regression against rc.10 on any arm driven here.
Every refusal encountered named its path, its reason and a route, and every route followed verbatim
worked.

---

## Arm 4 · addendum, 2026-08-18 — the two probes the first pass missed

The coverage derivation caught these: §5 arm 4 charters four probes and `v1-walk.md` recorded two.
Run now, so the arm is complete rather than reported complete.

### 4a · `doc rename` on a **committed** identity — expect the refusal, not exit 0

```console
$ jigc doc rename adr:drop-the-oldest-sample --to "Shed the oldest sample" --task <t>
exit 1
blocking · write.identity-change — rename rejected: `adr:drop-the-oldest-sample` is committed, so
  `--to "Shed the oldest sample"` would move its identity to `shed-the-oldest-sample` — a committed
  doc's path IS its identity, and referrers outside this task point at the old one. A same-slug
  retitle of the staged copy is supported; a re-slug is not
  route: `jigc rename adr:drop-the-oldest-sample --to 'Shed the oldest sample'` moves it for real —
         repointing every committed referrer in one transaction — once this task is finalized or
         discarded (it is a task-less, self-committing store op)
```

**PASS.** The committed-store split M48 shipped: it refuses, says why identity and path are the same
thing, and routes to the verb that does it atomically. Note the first attempt of this probe was run
with **no task open** and got `no active task — start one with jigc start` (exit 1) — a precondition
refusal, not this one. The distinction is why the probe is run in-task.

### 4b · a typo'd read-shaped `config` verb

```console
$ jigc config git docs-root
exit 2
error: unrecognized subcommand 'git'
  tip: some similar subcommands exist: 'list', 'get'
```

**PASS.** Both suggestions are read verbs, so the tip does not push a read intent at a write verb.

### What this addendum says about the walk

Arm 4 was reported complete when it was half-run. The gap was found by a derivation against the
protocol's own charter, not by the walk noticing — which is the argument for deriving §6's table
**before** the record is written rather than after.
