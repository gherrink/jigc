# V1 — the operator-scripted verification walk (run 2026-08-12, `jigc 1.0.0-rc.10`)

**Provenance:** [protocol.md](protocol.md) → The operator walk (V1), itself the execution of
[decisions-pending.md](../../../implementation/decisions-pending.md) → *Acceptance — the pre-1.0.0
trial*. Six arms, each justified by a behaviour M47 changed that no blind probe reaches. Not blind by
design — its value is entirely operator-placed plants and scripted destructive sequences.

**Binary:** `jigc 1.0.0-rc.10` throughout, except arm 6's setup half, which runs the temp-prefix
`1.0.0-rc.9` build from `d13d8ac`. Confirmed in the logs: 115 records across the three logging
corpora, `binary_version` `1.0.0-rc.10` on all of `rc10-walk` (30) and `rc10-fanout` (13), and
`rc9-legacy` (72) splitting **52 on rc.9 then 20 on rc.10** — the version transition this arm exists
to exercise, visible in the record.

**Corpora:** `~/ideas/rc10-walk` (arms 1–2) · `~/ideas/rc10-fanout` (arms 3–5) · `~/ideas/rc9-legacy`
(arm 6), all built from the trial template (protocol.md → The corpora).

## Arm results

| Arm | Result |
|---|---|
| 1 · `jigc migrate --as` end to end | **HELD** — review hold at exit 4, `--approve` the sole destructive gate |
| 2 · `migrate-corpus` N2 recovery | **HELD** — both plants placed; the recovery landed, wording flagged (V1-F3) |
| 3 · re-`provision` over a non-registered leftover | **FAILED — unrecoverable data loss at exit 0** (V1-F1) |
| 4 · teardown matrix (cheap re-check) | **HELD** — both guards refuse and name the dirty path |
| 5 · fresh clone → refusal → route → land | **HELD** — the whole chain followable end to end |
| 6 · rc.9-authored corpus continued on rc.10 | **HELD with one defect** — corpus reads and writes clean; an idempotent `rename` misreports (V1-F2) |

---

### Arm 1 — the migrate transform, end to end · HELD

A foreign `design-notes-backpressure.md` committed by hand, then `jigc migrate … --as adr`. The task
id minted as `migrate-adr-design-notes-backpressure-45176976cf01` — M44's path-hash disambiguator
present. A plain `jigc task finalize` **exited 4** and committed nothing, rendering the fidelity diff
(foreign source beside the canonical rewrite) with the route to `--approve`. The approving re-run
landed **one** commit carrying all three effects: `deleted design-notes-backpressure.md`, `promoted
docs/decisions/backpressure-drops-the-oldest-sample.md`, and the `.jigc/config/manifest.yaml` that
had been sitting uncommitted (see V1-O2). Review-before-destroy held; nothing was destroyed before
approval.

### Arm 2 — `migrate-corpus`'s N2 recovery signature · HELD (both plants placed)

**Both plants, or the arm is vacuous** — and both were placed: the committed `schema-version:` stamp
hand-downgraded 2→1 and committed under an innocuous message, plus a rejecting `pre-commit` hook. The
downgrade alone flipped `jigc validate` to **exit 1** with `schema-conformance.schema-version-current`
routing to `migrate-corpus`, so the corpus was genuinely unmigrated rather than answering
`already current`.

Under the rejecting hook, `migrate-corpus` exited 1 with the survivable frame exactly as MIGRATING.md
gate 5 promises — hook stderr verbatim, then the **door-specific** state-truth clause: *"nothing was
committed — the migrated bytes are written and staged, and the corpus is still recorded as
unmigrated"*, plus the re-run line. HEAD unmoved, the migrated path staged, the stamp bumped on disk.

The repaired re-run **landed** it: commit `52e4d30`, trailing line *"an earlier run's migration was
written but never landed; only its paths were staged"* — the N2 signature verbatim — and `validate`
returned to exit 0.

> **Measurement note, recorded because it nearly became a false finding:** a first reading showed
> `validate` at "EXIT=0" while its own trailer said the sweep exits non-zero. That was `$?` after a
> pipe reporting `tail`, not jigc. Re-measured without the pipe, `validate` exits **1**. No defect.

### Arm 3 — re-`provision` over a non-registered leftover · FAILED (V1-F1)

The arm the charter called out as *"unfenced by any suite, consistent with its being a declared
bound"*. Planted in the leftover worktree, deliberately across all three git states plus an authored
doc, so a destroyer would show as **visible loss rather than inference**:

```
 M src/store.ts        (unstaged edit)
A  src/wal.ts          (staged, git add-ed)
?? DESIGN-wal.md       (untracked — sixty lines existing nowhere else)
```

Then only the *registration* was removed (`rm -rf .git/worktrees/<id>`), leaving the checkout — the
"crashed run" state `provision --help` says it clears. The re-`provision` **destroyed all three at
exit 0**, printing the ordinary success line (`provisioned 2 worktree(s) …`) with nothing on stderr
and no mention that anything had been cleared.

What survived, checked against the object store rather than assumed: the **staged** `src/wal.ts` blob
is reachable as a dangling object (recoverable only by someone who knows to look and how); the
**unstaged** edit and the **untracked** `DESIGN-wal.md` are **gone outright**.

**The sharpest form of it is the contrast inside this same walk.** Arm 4 shows `milestone discard`
and `jigc uninstall` both *refusing* over a single untracked scratch file, each naming the exact path
and its git state. The same destruction, guarded at two doors and unguarded at a third — a fix
complete over the doors it was reported at, not over its class's axis, which is precisely what the
M45 complete-fix contract exists to prevent. M46 entry 11's rider names this path and defers it as a
declared bound; its trigger is *"an adopter reports real sub-agent work lost"*. This is that report,
measured, with the loss shown rather than inferred.

```yaml
claim: "jigc milestone provision destroys a non-registered leftover worktree's uncommitted work at exit 0, unwarned"
verdict: CONFIRMED
setup:
  - fixture: milestone with >=1 sub-task, provisioned
  - ["sh", "-c", "echo x > .jigc/worktrees/<sub>/DESIGN.md"]          # untracked
  - ["sh", "-c", "echo y >> .jigc/worktrees/<sub>/src/store.ts"]      # unstaged
  - ["sh", "-c", "cd .jigc/worktrees/<sub> && git add src/wal.ts"]    # staged
  - ["sh", "-c", "rm -rf .git/worktrees/<sub>"]                       # unregister, keep the checkout
repro:
  - ["jigc", "milestone", "provision", "<milestone>"]
expect:
  exit: 0
  observed: "success line only; DESIGN.md and the unstaged edit unrecoverable"
  wanted: "refuse like `milestone discard` / `uninstall` do, naming the dirty paths, with a --force escape hatch"
pinned-by: UNPINNED — no suite drives provision over a leftover; the fix's red test is the pin
```

### Arm 4 — the teardown matrix, cheap re-check · HELD

Over one untracked file in a live sub-task worktree:

- `jigc milestone discard` → **exit 1**, `milestone.dirty-worktree`, naming
  `…/replay-the-log-on-startup: ?? scratch.ts` with the route *get the work out … or re-run with
  `--force`*.
- `jigc uninstall` → **exit 1**, `uninstall.dirty-worktree` (the M47 completion-audit fix, first
  field run), same path named, route offering both the extraction and the `milestone discard --force`
  path.

Both guards hold and both are specific about what they are protecting. This arm confirms; it does not
sweep — `uninstall_worktree_guard.rs` and `milestone_discard.rs` own the axis.

### Arm 5 — fresh clone → refusal → route → land · HELD

A `git clone` of the fan-out corpus: the committed milestone record present, the gitignored workbench
absent. `jigc milestone finalize` **first**, before anything else:

```
EXIT=3
milestone:durable-sample-tier would land no work: no sub-task's docs promote to the store, and no
sub-task worktree holds staged code — the boundary would commit only jigc's own bookkeeping and flip
the milestone record to the terminal `joined`, after which the milestone could never be finalized again
  route: `jigc milestone provision durable-sample-tier` gives every sub-task a working area (a fresh
  clone has none; an existing one is reused); execute the sub-tasks, `git add` their work inside
  their worktrees, then re-run … — or settle the milestone as abandoned with `jigc milestone discard`
```

The refusal states the one-way consequence it is preventing and its route **names the fresh-clone
case explicitly**. Following it worked without improvisation: `provision` → `execute` emitted one
`Spawn:` line per sub-task → each line ran **verbatim** from the clone → real code staged in both
worktrees → `milestone finalize` landed one commit (`1bb091d`) carrying both sub-tasks' code and the
record update. Increment 3(b)(i), the reseed, and route-followability all confirmed in one pass.

### Arm 6 — an rc.9-authored corpus continued on rc.10 · HELD with one defect

Authored on the temp-prefix **rc.9**: five managed docs across five doctypes (`adr`, `spec`,
`arch-doc`, `changelog` at repo-root, `milestone-record`) plus a milestone with a sub-task, six
commits. Then switched to **rc.10** and continued.

**What held.** `jigc validate` exit 0 with a single `store-version.binary-mismatch` advisory carrying
its route; `doc list` reporting all five as `managed`; `doc show` on an rc.9-authored slot returning
its prose; `migrate-corpus` correctly reporting `0 migrated, 5 already current` (no schema-version
moved in M47); `milestone discard` settling the rc.9-minted record; a `quick-fix` task through
`finalize` landing clean with the project's own tests still green. The `rename` guard also fired
correctly first — *"cannot rename while task `add-a-write-ahead-log` is in flight … (a rename changes
the by-task-id join key)"*.

**What did not.** rc.9 minted `adr:sample-store-stays` from *"The sample store stays in memory"* under
the generation-2 slug rule. Renaming that doc to a title which, under rc.10's **generation-3** rule,
slugs back to the id it already has — with the H1 already carrying that title — produces this:

```
EXIT=1
`git commit` was rejected (no commit was made):
On branch main
nothing to commit, working tree clean

nothing was committed — the rename was rolled back, so `adr:durability-belongs-to-the-caller` still
holds its original identity and every referrer still points at it. Fix the hook's complaint, then
re-run `jigc rename adr:durability-belongs-to-the-caller --to 'Durability belongs to the caller'`.
```

No hook complained. The only hook installed is jigc's own warn-only backstop, which always exits 0;
`core.hooksPath` is unset. Git refused an **empty** commit, and that refusal is being dressed in the
survivable-hook-rejection frame: it asserts a rejection that did not happen, instructs the reader to
fix a complaint that does not exist, and offers a re-run that can only fail identically — a dead end,
which is the one thing the M42 route floor forbids. The rollback half of the message is true and the
store is untouched; the diagnosis and the route are not.

**This is an axis gap, not a missing feature.** `rename.rs` documents a designed *no-op reslug* path —
*"rewrite the H1 + commit, no `git mv`, no referrer repoint"* — and `flow37_rename.rs:1022`
(`placement_same_slug_retitle_succeeds`) fences it. That test renames `changelog:changelog` → `CHANGELOG`,
where the **H1 changes** (`# Changelog` → `# CHANGELOG`), so a diff exists and the commit lands. The
un-swept point of the same axis is *same slug **and** same H1* — the fully idempotent rename, where
there is no diff at all.

```yaml
claim: "an idempotent `jigc rename` (new title slugs to the current id AND the H1 already matches) reports a hook rejection that never happened, and routes to a re-run that can only fail identically"
verdict: CONFIRMED
setup:
  - fixture: any committed managed doc, no foreign hook installed
  - ["jigc", "rename", "adr:<slug>", "--to", "Durability belongs to the caller"]   # lands, H1 now matches
repro:
  - ["jigc", "rename", "adr:durability-belongs-to-the-caller", "--to", "Durability belongs to the caller"]
expect:
  exit: 1
  observed_stderr_contains: "`git commit` was rejected (no commit was made)\nOn branch main\nnothing to commit, working tree clean"
  observed_route: "Fix the hook's complaint, then re-run <the identical command>"
  wanted: "recognise the no-diff idempotent rename and say so (or exit 0 as a no-op); never claim a rejection, never route to a re-run that cannot change the outcome"
pinned-by: UNPINNED — flow37_rename::placement_same_slug_retitle_succeeds covers same-slug/different-H1 only; the fix's red test pins the same-slug/same-H1 point
```

---

## Findings

| id | severity | one line |
|---|---|---|
| **V1-F1** | **HIGH** | `milestone provision` destroys a non-registered leftover's staged, unstaged **and** untracked work at exit 0 under a success message — while `discard` and `uninstall` refuse over one untracked file |
| **V1-F2** | **MEDIUM** | An idempotent `rename` misreports git's empty-commit refusal as a hook rejection and dead-ends the route |
| **V1-F3** | **LOW** | `migrate-corpus`'s recovery headline reads `0 migrated, 1 already current` on a run that *did* land the staged bytes — the count describes the scan, the action is only in the trailing line |
| **V1-F4** | **LOW** | `doc schema` names an enum repeatable's id-source `category`; `doc author`'s payload requires it spelled `title`, and the rejection never says so |

**V1-F3 detail.** The full recovery output is `corpus migration: 0 migrated, 1 already current, 0
blocked` followed by `committed 52e4d30 — an earlier run's migration was written but never landed;
only its paths were staged`. Both lines are individually true — the scan found the on-disk bytes
already at the current version *because the interrupted run had written them* — but a reader taking
the headline count alone concludes nothing happened on a run that made a commit.

**V1-F4 detail, verified on rc.10.** `jigc doc schema changelog` projects
`category: enum [added|changed|…] (add-item: changelog:<slug>#unreleased-changes) *` — the
author-required id-source. A payload written from that projection is rejected:

```
malformed `doc author` payload: sections[0].items[0]: unknown field `category`,
expected one of `title`, `set`, `sections`
```

Spelling it `title: changed` works and renders `### changed  {#changed}`, so the capability is
present and correct — only its name is unreachable from the projection that is supposed to teach it.
The schema read is contract-pinned (contract-version 4) and is exactly the surface an agent is told
to consult, which is what makes this a law-2 case rather than a papercut.

## Observations (not findings — recorded, not adjudicated)

- **V1-O1 · `setup` leaves a tracked in-repo hook modified and uncommitted.** With `core.hooksPath`
  pointing in-repo, `setup` splices its managed block into the tracked foreign `pre-commit`
  (preserving it verbatim, block first, foreign body owning the final exit — correct), but its
  install commit does not carry the file, leaving the repo dirty on a tracked path. Found in the
  pre-trial rehearsal; pre-registered in protocol.md → environment matrix.
- **V1-O2 · `jigc config set` does not commit, and does not say so.** By design
  (`config.rs`: *"committed by the operator's next commit, never here"*), but the ack is bare
  (`config: set 'invocation-log' = 'true'`). Measured consequence in this walk: the knob set in
  `rc10-fanout` was never swept into a commit, so **the fresh clone in arm 5 inherited no config and
  produced zero invocation-log records**. In `rc10-walk` the same file landed opportunistically
  inside an unrelated ADR commit. Worth weighing against the fresh-clone-continuable claim.
- **V1-O3 · unresolved.** Arm 5's landing manifest reported `add-a-write-ahead-log: 1 doc, 1 code
  file` for a sub-task where only a code file was staged. The sub-task areas were torn down by the
  finalize before they could be inspected, so what the doc count refers to is **not established** —
  recorded as an open question, not asserted as a miscount.

## Honest bounds

- **The walk cannot see interaction defects by construction** — scripted arms exercise the sequences
  they were written for. M47's sharpest finding existed because one increment changed a state model
  another increment's prose described; that class belongs to the full sweep.
- **Arm 4 is a re-check, not a sweep** — its axis is owned by two named suites and was driven through
  this binary by the M47 e2e.
- **Arm 3's loss is real but its recoverability claim is narrow**: the staged blob's survival was
  verified against this repo's object store, immediately after the fact. Nothing surfaces that blob
  to a user, and no `gc` had run.
- **V1-F1 and V1-F2 are unfixed and unpinned by design** — no mid-trial fixes. Both blocks carry
  `UNPINNED` with the reason stated; converting them is the [conversion ledger](findings-verification.md)'s
  obligation, which gates the 1.0.0 call.
