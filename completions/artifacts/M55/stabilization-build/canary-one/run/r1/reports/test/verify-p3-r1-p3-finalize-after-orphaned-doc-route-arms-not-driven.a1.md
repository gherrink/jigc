# verify-real — `r1-p3-finalize-after-orphaned-doc-route-arms-not-driven`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier:
items 4 and 5 under *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-orphaned-doc-route-after-strand-not-run.a1.md`
— after each arm of that report's Repro VR-ROUTE-1, nobody finalized the rig's live task, and
nobody showed that `file-state.un-baselined` clears after the hand move. Door:
`jigc task finalize`. Clause it is said to break: `working-product`. Triage's grade: *unclear*.
The finding carries no repro block of its own; the arms are VR-ROUTE-1's, and what follows them
is spelled here.

## Verdict in one paragraph

**`refuted` — basis `does-not-reproduce`.** I reached the strand from nothing on both binaries,
ran each of the three arms on its own fresh rig, and finalized the rig's live task after it.
**After the re-point arm and after the hand-move arm `jigc task finalize` exits 0** and lands one
commit holding what the task and the index held: the task's `VISION.md`, the pending
`.jigc/config/manifest.yaml`, and — after the move — the rename the user staged with `git mv`.
**`file-state.un-baselined` is gone after the move arm's finalize**; it is still printed after
the commit doc is authored and after `jigc task validate`, so it is the landed finalize that
clears it, as the advisory's route says. **After the unmanage arm `jigc task finalize` refuses at
exit 3** with `schema-conformance.ref-resolves`, and commits nothing: the live task holds a
staged reference to the doc the arm just dropped from the store. That refusal is the
forward-reference gate working as designed, and its route lands the task — *drop the field*,
then finalize, exit 0. **The candidate and the previous release agree on every cell, byte for
byte** once the rig's directory name, the binary's path and the commit ids are normalised, so
no command that works on the previous release has stopped working here. Four things seen on
the way are not this finding and are left open.

## The binary, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` —
  the hash the prompt gives for the candidate (commit
  `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` —
  the previous release's (1.0.0-rc.24). **It was driven**, because triage asked for both
  binaries and because the clause is a comparison with it.
- In every cell the driven binary's directory went first on `PATH`, and the driver stops
  unless `command -v jigc` prints that binary's path (checked before the rig is built and again
  after its assignments are applied), unless the rig — built with
  `dev/jigc-rig refs-post-hoc --binary <that path>` — reports the same path as `$JIGC`, unless
  the rig's repository lies under my own scratch directory, and unless the working directory
  is that repository and git's top level is it. No cell stopped.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## What was driven

Twenty-two cells, each on its own fresh rig of the state `refs-post-hoc`, minted under
`<scratch>/verify-p3-finalize-arms.5Mru7u/runs/`. Every command whose exit status is read ran
bare, its standard output and standard error each to a file of its own; nothing was read
through a pipe. Nothing of any other verifier's scratch root was used. One further rig, on the
candidate, was used to learn the task's shape (read verbs and `--help` only); it is evidence
of nothing.

The rig's live task is `ground-the-vision-in-research`, minted from the workflow
`form-vision`. Its one write is the edge `vision:vision#meta/grounded-in =
[research:context-loss]`, staged over the committed `VISION.md`. Its commit doc is empty, so
`jigc task finalize` cannot land it as the rig leaves it.

**The setup is the handed block's, unchanged:**

    blob=$(git hash-object -w <a file holding "x\n">)
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"   # exit 0
    jigc config set docs-root notes                                            # exit 0
    git update-index --force-remove -- "bad<byte 0xFF>name.txt"                # exit 0

**What I added, because the finding names no commands.** After the arm, the task's commit doc
is authored with the four writes the rig's own `finalize` helper uses, then the task is
validated and finalized:

    jigc doc set-field commit:<task>#type  --value docs   --task <task>     # exit 0
    jigc doc set-field commit:<task>#scope --value vision --task <task>     # exit 0
    jigc doc set-slot  commit:<task>#summary --from-file - --task <task>    # exit 0, "ground the vision in research"
    jigc doc set-slot  commit:<task>#body    --from-file - --task <task>    # exit 0
    jigc task validate <task>
    jigc task finalize <task>

The cells:

| cell | setup | arm | binaries |
|---|---|---|---|
| baseline | none — the rig as built | none | both |
| none | handed | none | both |
| repoint | handed | `jigc config set docs-root docs` | both |
| unmanage | handed | `jigc unmanage docs/research/context-loss.md` | both |
| move | handed | `mkdir -p notes/research`, `git mv docs/research/context-loss.md notes/research/context-loss.md` | both |
| move-fine | handed | the move arm, with `jigc validate` run after the commit doc is authored and again after `jigc task validate` | both |
| unmanage-drop-field | handed | the unmanage arm, the refused finalize, then one arm of the refusal's route | both |
| unmanage-create-target | handed | the unmanage arm, the refused finalize, then another arm of that route | both |
| repoint, move, unmanage-drop-field | equivalent — `.jigc/config/manifest.yaml` written by hand, no index entry planted | as above | both |

### The state every stranded cell starts from (both binaries, identical text)

It is the handed report's: `jigc validate` exits 0 with `file-state.orphaned-doc` for
`docs/research/context-loss.md`; `jigc doc list` exits 0 with four rows, none of them
`research:context-loss`; `jigc doc show research:context-loss` exits 1 with `store.not-found`;
`jigc config get docs-root` prints `docs-root = notes  (project)`; `git status --porcelain`
shows only the untracked `.jigc/config/manifest.yaml`, whose bytes are
`scalar:\n  docs-root: notes\n`.

### What the finalize did after each arm

`C` and `P` hold one value in every cell below, so one value is written.

| cell | `jigc task validate` | `jigc task finalize` | commits before → after | `git show --name-status HEAD` | `git status --porcelain` after |
|---|---|---|---|---|---|
| baseline | exit 0 | **exit 0**, `promoted VISION.md`, `1 file committed` | 6 → 7 | `M VISION.md` | empty |
| none | exit 3 | **exit 3**, `schema-conformance.ref-resolves` | 6 → 6 | unchanged | `?? .jigc/config/manifest.yaml` |
| **repoint** | exit 0 | **exit 0**, `added .jigc/config/manifest.yaml`, `promoted VISION.md`, `2 files committed` | 6 → 7 | `A .jigc/config/manifest.yaml`, `M VISION.md` | empty |
| **unmanage** | exit 3 | **exit 3**, `schema-conformance.ref-resolves`, standard output empty | 6 → 6 | unchanged | `?? .jigc/config/manifest.yaml` |
| **move** | exit 0 | **exit 0**, `added .jigc/config/manifest.yaml`, `promoted VISION.md`, `deleted docs/research/context-loss.md`, `added notes/research/context-loss.md`, `4 files committed` | 6 → 7 | `A .jigc/config/manifest.yaml`, `M VISION.md`, `R100 docs/research/context-loss.md notes/research/context-loss.md` | empty |

What stands after the landed finalize:

- **repoint.** `jigc task list` prints `no active tasks`. `jigc doc list` has five rows,
  `research:context-loss  docs/research/context-loss.md  managed` among them.
  `jigc doc show research:context-loss` exits 0. `jigc validate` exits 0 with one advisory,
  the rig's own `schema-conformance.repeatable-populated`; no `file-state.*` row.
  `jigc config get docs-root` prints `docs-root = docs  (project)`. Line 2 of `VISION.md` is
  `grounded-in: [research:context-loss]`. The committed manifest is
  `scalar:\n  docs-root: docs\n`.
- **move.** `jigc task list` prints `no active tasks`. `jigc doc list` has five rows, the doc
  at `notes/research/context-loss.md`. `jigc doc show research:context-loss` exits 0.
  `jigc validate` exits 0 with the same single advisory — **no `file-state.un-baselined`, no
  `file-state.orphaned-doc`**. `jigc config get docs-root` prints `docs-root = notes  (project)`.
  Line 2 of `VISION.md` is `grounded-in: [research:context-loss]`.
- **unmanage.** Nothing landed. `jigc task list` still lists the task; `HEAD` is the commit
  it was; `VISION.md` holds no `grounded-in`; the manifest is still untracked.

**No byte of the doc moved or was lost in any cell.** `git hash-object` of the doc printed
`3507022f2299fc2746a813610ea9732edd8d58ff` at all 92 readings across the 22 cells, at
whichever of the two paths the doc then sat.

### Does `file-state.un-baselined` clear after the move arm — the cell `move-fine`

| after | `jigc validate` |
|---|---|
| the move arm | exit 0; `advisory · file-state.un-baselined — committed doc notes/research/context-loss.md is not yet baselined in the file-state record`, route *no action needed — the doc is baselined on its next author or finalize* |
| the four writes to the commit doc | exit 0; the same advisory, still there |
| `jigc task validate` (exit 0, which itself prints `file-state.baseline-adopt — baseline adopted: notes/research/context-loss.md`) | exit 0; the same advisory, **still there** |
| `jigc task finalize` (exit 0, printing the same `file-state.baseline-adopt` line) | exit 0; **the advisory is gone** |

So the route's *finalize* half is true of this doc, and the sweep's own announcement is not
what clears it: the landed finalize is. That is the rule
`design/reconciliation.md` → *Hash re-baselining* states — *a sweep's adoption is in memory
until a finalize lands*. The route's *author* half was not driven: the task does not hold this
doc, and the four writes went to the commit doc.

### The unmanage arm's refusal, and its route

The refusal, verbatim, on standard error, the same on both binaries:

    blocking · schema-conformance.ref-resolves — forward-ref integrity — `vision:vision#grounded-in` target `research:context-loss` resolves in neither the committed store nor this task's working area; resolution: fix the reference to an existing target, create the target in this task, or drop the `grounded-in` field
      at: vision:vision#grounded-in/research:context-loss
      route: fix the reference, create the target in this task, or drop the field

The route is prose with three arms. Two were run, each on a fresh rig per binary:

| route arm | as run | exit | then `jigc task finalize` | what landed |
|---|---|---|---|---|
| *drop the field* | `jigc doc set-field vision:vision#grounded-in --unset --task <task>` | 0, `unset vision:vision#grounded-in` | **exit 0**, `added .jigc/config/manifest.yaml`, `1 file committed` | `A .jigc/config/manifest.yaml`; commits 6 → 7; tree clean; no active task |
| *create the target in this task* | `jigc doc create research --title 'Context Loss' --task <task>` | **1**, `blocking · create.gate-blocked — the workflow does not allow jigc doc create research in-task; allowed doctypes: [vision]` | exit 3, the same refusal | nothing; commits 6 → 6 |

After *drop the field*, `jigc validate` exits 0 and still carries
`file-state.unregistered-doc` for `docs/research/context-loss.md` — the state the unmanage arm
leaves by design — and `jigc doc list` has four rows. The second arm is first under *Left
open*.

### The same arms from a strand that needs no planted entry

Six more cells write the knob straight into the project manifest — the same 27 bytes
`jigc config set` leaves there — and plant nothing in the index. From the stranded state on,
each of *repoint*, *move* and *unmanage-drop-field* is identical, line for line, to its cell
under the handed setup, on the candidate; and the candidate's and the previous release's are
identical to each other. So the block below can be pinned without a raw byte.

## Is it what the finding says?

The finding says the finalize after each arm was never driven. Driven, nothing misbehaves.
Read against the design that owns each thing seen:

- **What a landed finalize commits.** `design/finalize.md` → *5. Stage*: *the commit set is
  therefore the git index — the agent's staged code plus jigc's just-staged docs/config —
  committed whole*. The move arm's `git mv` staged a rename, and it rode in the commit; the
  ack names it line by line. The pending manifest is the write `jigc config set` acknowledged
  with *uncommitted — commit it with your next commit* (`design/overrides.md` → *Every
  authoring ack says the write is uncommitted*), and `design/finalize.md`'s doc-only passage
  says an ordinary finalize commits a pending `.jigc/config` delta beside the doc.
- **The unmanage arm's refusal.** The forward-reference gate at finalize is the product's
  first acceptance path — it passes when the target exists and blocks when it dangles
  (`CLAUDE.md` → *MVP scope*, point (d); `design/validation.md`, the per-task gate's
  `ref-resolves`). The arm's stated purpose is to drop the doc from the store; a task that
  references the dropped doc then has a dangling reference, and the gate says so before
  anything is written. Nothing is committed and nothing is lost.
- **The baseline.** `design/validation.md` → *UNKNOWN (un-baselined) ≠ clean* and
  `design/reconciliation.md` → *Hash re-baselining*: the landed finalize writes the swept
  record. It did.

Nothing here argues that a settled decision is wrong, so nothing is contested.

## Does it break `working-product`, inside that clause's scope?

No — and because nothing fails that should work, this is `does-not-reproduce`, not
`breaks-no-clause`.

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  second clause is *a working product others can use and rely on*; its instrument, in the
  second sharpening, is *no command that works on rc.24 in a supported layout stops working,
  and every refusal's route works as printed*. The run's opening names it `working-product`.
- **First half — nothing stopped working.** For each of the eleven cell kinds the candidate's
  log and the previous release's were compared whole after replacing the rig's directory name,
  the binary's path, the cell's label and the ids of the commits: every one of the eleven
  diffs is empty. Same exits, same standard output, same standard error, same commits by
  path and status, same tree afterwards.
- **Second half — the one refusal's route.** `jigc task finalize` refuses in the unmanage
  arm. Its route's *drop the field* arm exits 0 and the finalize that follows exits 0. Its
  *create the target in this task* arm is refused by this workflow's create-gate, with a
  route of its own. I read a three-way prose route as working where an arm of it lands the
  task; whether one unreachable arm is a row is triage's, and it is left open below — on both
  binaries alike, so it is no regression either way.

## The regression fact

**Not owed, and so not returned**: step 4 runs with `confirmed` only. What was established on
the way stands as evidence all the same — every cell was driven on the previous release too,
and it behaves there exactly as on the candidate.

## Class

**Driven: one doc, one task, three arms — `instance, unbounded`.** One doctype (`research`,
a `location:` home), one doc, one value of the knob, one live task whose only staged write is
a reference to the stranded doc, one order (the arm, then the commit doc, then finalize), text
output only. I enumerated no consumers of any mechanism.

## Coverage

The finding is of the form *this was not driven*. Checked against the suites, not a diff:
`crates/cli/tests` (511 `.rs` files, read recursively) and `tooling-tests` (22) were searched
for the string `orphaned-doc`; 6 files name it. Four of the 6 hold no occurrence of the word
`finalize` at all (`orphan_detection.rs`, `orphaned_instance.rs`, `doc_show_relocated.rs`,
`unclaimed_file_family.rs`). The other two run a finalize in tests about something else —
`placement_override.rs`, on a resolved placement home, and `flow52_acceptance.rs`, on a
doctype that leaves the resolved set; I read the test names around each hit, not every test
body. For the baseline half, 7 files name `un-baselined`; `copy_in_baseline.rs` asserts the
route's *author* half (*the author write baselined the doc*), and I found no assertion in the
7 that the advisory is absent after a landed finalize over a moved doc. The engine crate's
in-module tests were not enumerated. So: no standing test found that finalizes a task after
an arm of the `location:` strand's route — the block below is that test in waiting.

## Repro VR-FIN-1

```yaml
claim: "after an arm of file-state.orphaned-doc's route (re-point, unmanage, hand move), finalizing the live task misbehaves, or file-state.un-baselined does not clear after the move — breaking the clause working-product"
verdict: "REFUTED — does-not-reproduce. Finalize exits 0 after the re-point and the move and commits what the task and the index held; the un-baselined advisory is gone after the move's finalize; after unmanage it refuses at exit 3 by the forward-ref gate, and the route's drop-the-field arm lands it. Candidate and previous release are identical."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
setup:                      # the handed block's state; one fresh rig per arm
  - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <the binary>; <task> = ground-the-vision-in-research
  - "blob=$(git hash-object -w <a file holding 'x\n'>)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
  - ["jigc", "config", "set", "docs-root", "notes"]                                             # exit 0, moves nothing
  - ["git", "update-index", "--force-remove", "--", "bad<byte 0xFF>name.txt"]                   # exit 0
setup-equivalent:           # driven on both binaries: the same outcomes from the stranded state on, no raw byte
  - fixture: refs-post-hoc
  - "write .jigc/config/manifest.yaml = `scalar:\n  docs-root: notes\n`"
author-commit-doc:          # run after the arm, before the finalize; each exits 0
  - ["jigc", "doc", "set-field", "commit:<task>#type", "--value", "docs", "--task", "<task>"]
  - ["jigc", "doc", "set-field", "commit:<task>#scope", "--value", "vision", "--task", "<task>"]
  - ["jigc", "doc", "set-slot", "commit:<task>#summary", "--from-file", "-", "--task", "<task>"]   # stdin: ground the vision in research
  - ["jigc", "doc", "set-slot", "commit:<task>#body", "--from-file", "-", "--task", "<task>"]      # stdin: one line
arms:
  - arm: "re-point docs-root to cover it"
    repro:
      - ["jigc", "config", "set", "docs-root", "docs"]
      - author-commit-doc
      - ["jigc", "task", "finalize", "<task>"]
      - ["git", "show", "--name-status", "--format=", "HEAD"]
      - ["git", "status", "--porcelain", "--untracked-files=all"]
      - ["jigc", "validate"]
    expect:
      - exit: 0
      - exit: 0
      - exit: 0
        stdout_contains: ["added .jigc/config/manifest.yaml", "promoted VISION.md", "2 files committed"]
      - exit: 0
        stdout: "A\t.jigc/config/manifest.yaml\nM\tVISION.md"
      - exit: 0
        stdout: ""
      - exit: 0
        stdout_lacks: "file-state."
  - arm: "the hand move under the new root"
    repro:
      - ["mkdir", "-p", "notes/research"]
      - ["git", "mv", "docs/research/context-loss.md", "notes/research/context-loss.md"]
      - ["jigc", "validate"]
      - author-commit-doc
      - ["jigc", "task", "validate", "<task>"]
      - ["jigc", "validate"]
      - ["jigc", "task", "finalize", "<task>"]
      - ["git", "show", "--name-status", "--format=", "HEAD"]
      - ["git", "status", "--porcelain", "--untracked-files=all"]
      - ["jigc", "validate"]
      - ["jigc", "doc", "show", "research:context-loss"]
    expect:
      - exit: 0
      - exit: 0
      - exit: 0
        stdout_contains: "advisory · file-state.un-baselined — committed doc `notes/research/context-loss.md`"
      - exit: 0
      - exit: 0
        stdout_contains: "file-state.baseline-adopt"
      - exit: 0
        stdout_contains: "file-state.un-baselined"          # a sweep's adoption is in memory
      - exit: 0
        stdout_contains: ["added .jigc/config/manifest.yaml", "promoted VISION.md", "deleted docs/research/context-loss.md", "added notes/research/context-loss.md", "4 files committed"]
      - exit: 0
        stdout: "A\t.jigc/config/manifest.yaml\nM\tVISION.md\nR100\tdocs/research/context-loss.md\tnotes/research/context-loss.md"
      - exit: 0
        stdout: ""
      - exit: 0
        stdout_lacks: ["file-state.un-baselined", "file-state.orphaned-doc"]
      - exit: 0
        stdout_contains: "# Context Loss"
  - arm: "drop it with jigc unmanage — the gate refuses, and its route lands the task"
    repro:
      - ["jigc", "unmanage", "docs/research/context-loss.md"]
      - author-commit-doc
      - ["jigc", "task", "finalize", "<task>"]
      - ["git", "rev-list", "--count", "HEAD"]
      - ["jigc", "doc", "set-field", "vision:vision#grounded-in", "--unset", "--task", "<task>"]
      - ["jigc", "task", "finalize", "<task>"]
      - ["git", "show", "--name-status", "--format=", "HEAD"]
      - ["git", "status", "--porcelain", "--untracked-files=all"]
    expect:
      - exit: 0
      - exit: 0
      - exit: 3
        stdout: ""
        stderr_contains: "blocking · schema-conformance.ref-resolves — forward-ref integrity — `vision:vision#grounded-in` target `research:context-loss`"
      - exit: 0
        stdout: "6"                                           # nothing landed
      - exit: 0
        stdout_contains: "unset vision:vision#grounded-in"
      - exit: 0
        stdout_contains: ["added .jigc/config/manifest.yaml", "1 file committed"]
      - exit: 0
        stdout: "A\t.jigc/config/manifest.yaml"
      - exit: 0
        stdout: ""
invariant: "`git hash-object` of the doc is 3507022f2299fc2746a813610ea9732edd8d58ff at every step of every arm, at whichever path it sits"
observed: "<scratch>/verify-p3-finalize-arms.5Mru7u — cand.<cell>.log and prev.<cell>.log for the cells baseline, none, repoint, unmanage, move, move-fine, unmanage-drop-field, unmanage-create-target; candm.<cell>.log and prevm.<cell>.log for the setup-equivalent cells repoint, move, unmanage-drop-field; norm/ holds the normalised logs and the diffs"
pinned-by: "UNPINNED: no standing test found that finalizes a task after an arm of the location strand's route — see Coverage"
```

**Pinnable as it stands: yes, through `setup-equivalent`.** The three arms are plain argv over
a state `trial_corpus.rs` already builds, and every `expect` states behaviour this verdict
wants to stay true. Through the handed `setup` it is pinnable on Unix only — the name must be
passed as raw bytes — and it would also hold in place, as a step, the door landing its knob
over a doc it left behind, which is another finding's subject; a test should take the
equivalent setup. One line of the move arm pins a design rule rather than this finding — the
advisory still standing after `jigc task validate` — and a test may drop it.

## Left open

1. **The `ref-resolves` refusal's arm *create the target in this task* cannot be followed in
   this task.** `jigc doc create research --title 'Context Loss' --task <task>` exits 1 with
   `create.gate-blocked — … allowed doctypes: [vision]`, because `form-vision` grants only
   `vision`; the finalize that follows refuses as before. The refusal prints the same three
   arms whatever the workflow grants. Both binaries, identical. The *drop the field* arm
   works. If triage reads *every refusal's route works as printed* as every arm of a prose
   route, this is a row of its own, at the door `jigc task finalize`; I did not grade it.
2. **`jigc unmanage` drops a doc an open task's staged reference targets, and says nothing of
   the task.** Its ack is `unmanaged docs/research/context-loss.md — dropped its file-state
   baseline; the file is left on disk`; the task's next `jigc task validate` and
   `jigc task finalize` then exit 3. Another door (`jigc unmanage`); both binaries; not
   pursued.
3. **In the strand with no arm run, the finalize refusal does not name the strand.** The
   control cell `none`: `jigc task finalize` exits 3 with the same `ref-resolves` text —
   *resolves in neither the committed store nor this task's working area* — over a target that
   is committed and stranded, where `jigc doc show` on the same doc names the re-point and the
   repair. Both binaries; nothing is committed. Not this finding, which is about the state
   after an arm.
4. **`design/finalize.md` → *5. Stage* says the project config layer is staged *on a first
   commit*;** the binary staged the pending `.jigc/config/manifest.yaml` into a seventh
   commit, as that document's rollback table (*every finalize*) and its doc-only passage
   describe. A sentence against its own document, not a behaviour I am calling wrong.
5. **Not driven:** the *author* half of the un-baselined route on the moved doc (a write to
   that doc itself); a hand move with plain `mv`, unstaged; finalize after the handed report's
   fourth cell (re-point back, then set the new root again); the route's *fix the reference*
   arm; `file-state.unregistered-doc`'s own route followed by a finalize; `--format json` of
   any cell; the placement arm of the advisory.

## Bounds — what this verification did not do

- One doc, one doctype, one value of the knob (`notes`), one way back (`docs`), one task, one
  workflow; text output.
- One order: the arm, then the four writes to the commit doc, then the finalize. The commit
  doc authored before the strand was not driven.
- The cells `none`, `unmanage`, `move-fine` and `unmanage-create-target` ran under the handed
  setup only.
- The live task's only staged doc is `VISION.md`. A task that holds the stranded doc itself
  was not built.
- I read the handed report, the exit rule's entry, the run's opening, `design/validation.md`
  → the store-scope file-state passage and *Orphan detection*, `design/storage.md` →
  `docs-root`, `design/finalize.md` → *5. Stage*, `design/reconciliation.md` → *Baseline
  adoption* and *Hash re-baselining*, `design/overrides.md` → the authoring ack. I read no
  other finding's report and no other verifier's.

## Where the evidence is

- `<scratch>/verify-p3-finalize-arms.5Mru7u/scripts/drive.sh` — the driver of the first five
  cells; `drive2.sh` — the same with the finer move cell and the two route arms; `drive3.sh`
  — its variant for the setup-equivalent. `norm.py`, `tail.py`, `pick.py` — the normaliser
  and two readers.
- `<scratch>/verify-p3-finalize-arms.5Mru7u/cand.*.log`, `prev.*.log`, `candm.*.log`,
  `prevm.*.log` — every command, its exit status, its standard output and standard error, in
  order, one file per cell.
- `<scratch>/verify-p3-finalize-arms.5Mru7u/runs/<cell>/` — one file per stream per step; the
  rigs themselves are beside them.
- `<scratch>/verify-p3-finalize-arms.5Mru7u/norm/` — the normalised logs, the eleven
  candidate-against-previous diffs (all empty) and the three handed-against-equivalent diffs
  (all empty).
- `<scratch>/verify-p3-finalize-arms.5Mru7u/explore/` — the exploratory rig and the rig's
  printed construction; evidence of nothing.

<!-- end of report -->
