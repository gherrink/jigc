# verify-real — `r1-orphaned-doc-route-after-strand-not-run`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: the
second bullet of *What stands afterwards (cells A1, A2)* and item 4 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p1-r1-non-utf8-path-other-orphan-walk-consumers.a1.md`
— a route its reporter printed and did not run. Door: `jigc validate`. Clause it is said to
break: `working-product`. Triage's grade: *unclear*.

## Verdict in one paragraph

**`refuted` — basis `does-not-reproduce`.** The state is real and I reached it from nothing, on
both binaries: after the 0xFF index entry is planted, `jigc config set docs-root notes` lands
the knob and moves nothing, and once the entry is removed `jigc validate` exits 0 carrying
`file-state.orphaned-doc` for `docs/research/context-loss.md`. **What does not reproduce is a
route that fails.** Every arm of that advisory's route, run on its own fresh rig, exits 0 and
clears the advisory: re-pointing `docs-root` back to `docs` restores the doc to the listing
and to `jigc doc show`; `jigc unmanage docs/research/context-loss.md` drops it from the store,
as the route says; and the hand move the route's first clause names lands it at the new root.
A fourth cell answers the lead's literal question — *does anything land the doc at the new
root* — with a jigc verb: re-point back, then `jigc config set docs-root notes` again, and the
door moves the doc itself. **The candidate and the previous release agree on every cell, byte
for byte** once the rig's directory name and the commit ids are normalised, so no command that
works on the previous release has stopped working here. Two things I saw on the way are not
this finding and are left open, first in the list: the route's backticked span is
`jigc unmanage` with no path, and typed exactly so it exits 2 with a usage error.

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
  unless `command -v jigc` prints that binary's path, unless the rig — built with
  `dev/jigc-rig refs-post-hoc --binary <that path>` — reports the same path as `$JIGC`, and
  unless the rig's repository lies under my own scratch directory. No cell stopped.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## What was driven

Sixteen cells, each on its own fresh rig of the state `refs-post-hoc`, minted under
`<scratch>/verify-p2-route.kaywVo/runs/`. Every command whose exit status is read ran bare,
its standard output and standard error each to a file of its own; nothing was read through a
pipe. Nothing of the reporter's scratch root was used.

The setup, identical in the fourteen cells of the handed block's state:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"   # exit 0
    jigc config set docs-root notes                                            # exit 0
    git update-index --force-remove -- "bad<byte 0xFF>name.txt"                # exit 0

**What I changed from the block as handed: nothing in the setup.** The block names no command
for the route, so the arms are mine to spell, and they are spelled from the route's own text
and from triage's instruction. The route's `re-point docs-root to cover it` is driven as
`jigc config set docs-root docs` — the rig's root before the door ran was
`docs-root = docs/  (pack-default)`, and `docs` is the value that covers where the doc sits.
The route's `jigc unmanage` is driven with the path the finding's own `at:` line gives.

### The state every cell starts from (both binaries, identical text)

| step | exit | what it printed |
|---|---|---|
| the plant | 0 | nothing; `git ls-files -z` then exits 0 and its output is not valid UTF-8 |
| `jigc config set docs-root notes` | 0 | `config: set docs-root = notes — written to .jigc/config/, uncommitted …`; standard error empty; no relocation line |
| `jigc config get docs-root` | 0 | `docs-root = notes  (project)` |
| `git ls-files -- docs/research/context-loss.md notes/research/context-loss.md` | 0 | `docs/research/context-loss.md` — it did not move |
| the removal | 0 | nothing; the listing is valid UTF-8 again; `git status --porcelain` shows only the untracked `.jigc/config/manifest.yaml` |
| `jigc validate` | 0 | two advisories; the second is the finding's (verbatim below) |
| `jigc doc list` | 0 | four rows; `research:context-loss` has none |
| `jigc doc show research:context-loss` | 1 | `blocking · store.not-found — could not read research:context-loss at notes/research/context-loss.md: No such file or directory (os error 2)` |

The advisory, verbatim, the same on both binaries:

    advisory · file-state.orphaned-doc — committed doc `docs/research/context-loss.md` sits outside the resolved doctype roots — a `docs-root` change likely stranded it (it looks managed but resolves under no doctype location)
      at: docs/research/context-loss.md
      route: move it under the current resolved root (re-point `docs-root` to cover it) or drop it with `jigc unmanage`

So the handed report's cell A1 reproduces as written. That half is not in dispute.

### The route's arms — what stands after each

Each row is one fresh rig per binary. `C` is the candidate, `P` the previous release; where a
cell holds one value it is the value on both.

| cell | the arm, as run | arm's exit | `jigc doc list` | `jigc doc show research:context-loss` | `jigc validate` |
|---|---|---|---|---|---|
| none (control) | nothing | - | exit 0, four rows | exit 1, `store.not-found` | exit 0, `file-state.orphaned-doc` still there |
| **repoint** | `jigc config set docs-root docs` | 0 | exit 0, **five rows**, `research:context-loss  docs/research/context-loss.md  managed` | **exit 0**, the doc | exit 0, **one advisory — the orphan advisory is gone** |
| **unmanage** | `jigc unmanage docs/research/context-loss.md` | 0 | exit 0, four rows | exit 1, `store.not-found` | exit 0, **the orphan advisory is gone**; `file-state.unregistered-doc` for the same path in its place |
| move | `mkdir -p notes/research` then `git mv docs/research/context-loss.md notes/research/context-loss.md` | 0, 0 | exit 0, **five rows**, the doc at `notes/research/context-loss.md` | **exit 0**, the doc | exit 0, **the orphan advisory is gone**; `file-state.un-baselined` for the new path, route *no action needed* |
| repoint-then-set | `jigc config set docs-root docs` then `jigc config set docs-root notes` | 0, 0 | exit 0, **five rows**, the doc at `notes/research/context-loss.md` | **exit 0**, the doc | exit 0, **one advisory — the orphan advisory is gone** |
| unmanage-twice | the unmanage arm, twice | 0, 0 | as *unmanage* | as *unmanage* | as *unmanage* |
| unmanage-bare | `jigc unmanage`, no argument | **2** | four rows | exit 1 | the orphan advisory still there |

What each arm printed:

- **repoint.** `config: set docs-root = docs — written to .jigc/config/, uncommitted — commit
  it with your next commit`, standard error empty. `jigc config get docs-root` then prints
  `docs-root = docs  (project)`.
- **unmanage.** `unmanaged docs/research/context-loss.md — dropped its file-state baseline;
  the file is left on disk`. The file is on disk and tracked afterwards; a second run prints
  `no-op: docs/research/context-loss.md is not managed (nothing to drop)` at exit 0.
- **repoint-then-set.** The second command's standard error carries `relocating 1 committed
  doc(s) stranded by the docs-root re-point to notes …` and
  `- docs/research/context-loss.md → notes/research/context-loss.md`; `git status --porcelain`
  then shows `R  docs/research/context-loss.md -> notes/research/context-loss.md`.
- **unmanage-bare.** `error: the following required arguments were not provided:` /
  `<PATH>` / `Usage: jigc unmanage <PATH>`, exit 2. Nothing is written.

**No byte moved or lost in any cell.** `git rev-parse HEAD` prints the same commit at the
first and the last step of each of the fourteen cells; `git hash-object` of the doc prints
`3507022f2299fc2746a813610ea9732edd8d58ff` before the plant and after the arm in every cell,
at whichever of the two paths the doc then sits. No cell committed anything.

### The same two arms from a strand that needs no planted entry (candidate only)

Two more cells reach the strand the way the standing suite does
(`crates/cli/tests/orphan_detection.rs`, `validate_flags_orphaned_doc_after_docs_root_repoint_only`):
the knob written straight into the project manifest — `.jigc/config/manifest.yaml` =
`scalar:` / `  docs-root: notes` — with no index plant at all. `jigc validate` prints the same
advisory with the same route, `jigc doc list` the same four rows, `jigc doc show` the same
exit 1. The *repoint* arm and the *unmanage* arm then end exactly as in the table above. So
the route's behaviour does not depend on how the strand came about — and the block below can
be pinned without a raw byte.

## Is it what the finding says?

The finding says a route was printed and nobody ran it. Run, it works. Read against the
design that owns it:

- `design/validation.md` → *Orphan detection* states the route as *re-point `docs-root` to
  cover it, or `jigc unmanage` it*. Both do what those words say.
- The *unmanage* arm ends with the doc outside the store — four rows, the read at exit 1 —
  and with `file-state.unregistered-doc` raised for it. That is the design's *M40 two-tier
  route*: a path with no `FileStateRecord` is the unregistered tier, and its emission is
  deliberately never gated on registration. Dropping the doc is the arm's stated purpose, so
  a read that then fails is the arm having worked, not the route having failed.
- The *move* arm ends with `file-state.un-baselined`, whose route opens *no action needed*.

Nothing here argues that a settled decision is wrong, so nothing is contested.

## Does it break `working-product`, inside that clause's scope?

No, and for the plain reason that nothing fails — this is `does-not-reproduce`, not
`breaks-no-clause`.

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  second clause is *a working product others can use and rely on*; its instrument, in the
  second sharpening, is *no command that works on rc.24 in a supported layout stops working,
  and every refusal's route works as printed*. The run's opening names it `working-product`.
- **First half — nothing stopped working.** For each of the seven cell kinds the candidate's
  log and the previous release's were compared whole after replacing the rig's directory name,
  the binary's path and the two commit ids: the one line that differs in each pair is my own
  closing label. Same exits, same standard output, same standard error, same files.
- **Second half — the route works.** `file-state.orphaned-doc` is an advisory at exit 0, not
  a refusal, so the sentence does not strictly reach it; held to the sentence anyway, each
  arm exits 0 and removes the advisory it was printed under.

## The regression fact

**Not owed, and so not returned**: step 4 runs with `confirmed` only. What was established on
the way stands as evidence all the same — every cell was driven on the previous release too,
and it is green there exactly as on the candidate.

## Class

**Driven: one arm of the producer, at one doc — `instance, unbounded`.** The advisory is built
at one place, `crates/cli/src/cli.rs:1651-1683`, with two arms: a `location:` home (the route
driven here) and a `placement:` home, whose route is a different sentence and names
`jigc unmanage <path>` with the path. 1 of those 2 was driven: one doctype (`research`), one
doc, one value of the knob, text output only. The placement arm's route was not run.

## Coverage

The finding carries a claim of the form *this was not run*. Checked against the suites, not a
diff: `crates/cli/tests` (511 `.rs` files, read recursively) and `tooling-tests` (22) were searched for the
advisory's code and for two phrases of its message; 6 files name it. Across both trees every
execution of `unmanage` as an argv was listed, and exactly one sits in one of those 6 —
`placement_override.rs:882`, on a resolved placement home. For the `location:` arm,
`orphan_detection.rs:233` asserts that the route's text contains `jigc unmanage`, and no test
in the 6 runs either arm of it and reads the store back. The engine crate has no `tests/`
directory; the in-module tests were not enumerated. So: the route's text is held, its outcome
is held by nothing — the block below is that test in waiting.

## Repro VR-ROUTE-1

```yaml
claim: "after a docs-root strand, the route file-state.orphaned-doc prints fails or was never shown to work — breaking the clause working-product"
verdict: "REFUTED — does-not-reproduce. Every arm of the route exits 0 and clears the advisory, on the candidate and on the previous release alike."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
setup:                      # the handed block's state; one fresh rig per arm
  - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <the binary>
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
  - ["jigc", "config", "set", "docs-root", "notes"]                                             # exit 0, moves nothing
  - ["git", "update-index", "--force-remove", "--", "bad<byte 0xFF>name.txt"]                   # exit 0
setup-equivalent:           # driven on the candidate: same advisory, same outcomes, no raw byte
  - fixture: refs-post-hoc
  - "write .jigc/config/manifest.yaml = `scalar:\n  docs-root: notes\n`"
precondition:
  repro:
    - ["jigc", "validate"]
    - ["jigc", "doc", "list"]
    - ["jigc", "doc", "show", "research:context-loss"]
  expect:
    - exit: 0
      stdout_contains: "advisory · file-state.orphaned-doc — committed doc `docs/research/context-loss.md` sits outside the resolved doctype roots"
    - exit: 0
      stdout_lacks: "research:context-loss"
    - exit: 1
      stderr_contains: "store.not-found"
arms:
  - arm: "re-point docs-root to cover it"
    repro:
      - ["jigc", "config", "set", "docs-root", "docs"]
      - ["jigc", "doc", "list"]
      - ["jigc", "doc", "show", "research:context-loss"]
      - ["jigc", "validate"]
    expect:
      - exit: 0
        stdout_contains: "config: set `docs-root` = `docs`"
      - exit: 0
        stdout_contains: "research:context-loss  docs/research/context-loss.md  managed"
      - exit: 0
        stdout_contains: "# Context Loss"
      - exit: 0
        stdout_lacks: "file-state.orphaned-doc"
  - arm: "drop it with jigc unmanage"
    repro:
      - ["jigc", "unmanage", "docs/research/context-loss.md"]
      - ["jigc", "doc", "list"]
      - ["jigc", "validate"]
      - ["git", "ls-files", "--", "docs/research/context-loss.md"]
    expect:
      - exit: 0
        stdout_contains: "unmanaged docs/research/context-loss.md — dropped its file-state baseline; the file is left on disk"
      - exit: 0
        stdout_lacks: "research:context-loss"
      - exit: 0
        stdout_lacks: "file-state.orphaned-doc"
        stdout_contains: "file-state.unregistered-doc"
      - exit: 0
        stdout: "docs/research/context-loss.md"          # still tracked, still on disk
  - arm: "re-point back, then set the new root again — the door moves the doc"
    repro:
      - ["jigc", "config", "set", "docs-root", "docs"]
      - ["jigc", "config", "set", "docs-root", "notes"]
      - ["git", "ls-files", "--", "docs/research/context-loss.md", "notes/research/context-loss.md"]
      - ["jigc", "doc", "show", "research:context-loss"]
      - ["jigc", "validate"]
    expect:
      - exit: 0
      - exit: 0
        stderr_contains: "docs/research/context-loss.md → notes/research/context-loss.md"
      - exit: 0
        stdout: "notes/research/context-loss.md"
      - exit: 0
        stdout_contains: "# Context Loss"
      - exit: 0
        stdout_lacks: "file-state.orphaned-doc"
invariant: "in every arm `git rev-parse HEAD` is unchanged and `git hash-object` of the doc is unchanged, at whichever path it sits"
observed: "<scratch>/verify-p2-route.kaywVo — cand.<arm>.log and prev.<arm>.log for the arms none, repoint, unmanage, move, unmanage-bare, unmanage-twice, repoint-then-set; candm.repoint.log and candm.unmanage.log for the setup-equivalent; norm/ holds the normalised pairs"
pinned-by: "UNPINNED: no standing test runs either arm of the location route and reads the store back — see Coverage"
```

**Pinnable as it stands: yes, through `setup-equivalent`.** The three arms are plain argv over
a state `trial_corpus.rs` already builds, and every `expect` states behaviour this verdict
wants to stay true. Through the handed `setup` it is pinnable on Unix only — the name must be
passed as raw bytes — and it would also hold in place, as a step, the door landing its knob
over a doc it left behind, which is the other finding's defect; a test should take the
equivalent setup and leave that step out. `setup-equivalent` was driven on the candidate only.

## Left open

1. **The route's backticked span carries no path.** The `location:` arm prints *drop it with
   `jigc unmanage`*; typed exactly so, it exits 2 with clap's usage error naming `<PATH>`, on
   both binaries. The path is one line above, on `at:`, and with it the arm works. Its
   sibling in the same producer — the `placement:` arm, `cli.rs:1669-1673` — and the
   `jigc doc show` refusal both print the command with the path. Read against
   `design/surface-contract.md`, this is a two-command prose route, the kind that document
   leaves outside the followability property, which reads an argv; I did not grade it, and it
   is no regression. If triage reads the bare span as the route *as printed*, it is a row of
   its own.
2. **After the unmanage arm, `jigc doc show research:context-loss` still says a re-point
   stranded the doc and offers `jigc unmanage docs/research/context-loss.md`** — which by
   then is the no-op the second run printed. Its other two repairs still apply. Another door
   (`jigc doc show`); the same on both binaries; not pursued.
3. **`file-state.unregistered-doc`'s own route, after the unmanage arm** —
   `jigc migrate <the doc's absolute path> --as research`, or ignore it. Not run.
4. **`file-state.un-baselined` after the hand move** — *baselined on its next author or
   finalize*. Neither was driven.
5. **Nothing was finalized or committed after any arm.** The rig holds a live task with a
   staged edge on the committed vision, and the knob sits in an untracked manifest; whether a
   task's finalize behaves after each arm was not driven.
6. **The placement arm of the same advisory**, and `--format json` of every cell.

## Bounds — what this verification did not do

- One doc, one doctype, one value of the knob (`notes`), one way back (`docs`); text output.
- `jigc config set docs-root docs` leaves a project-layer value `docs` where the rig began
  with the pack default `docs/`. Clearing the override instead was not driven.
- The setup-equivalent cells ran on the candidate only.
- The earlier report's statement that the same `config set docs-root notes`, repeated with
  the knob already at `notes`, moves nothing was not re-driven; the *repoint-then-set* cell
  goes through `docs` first.
- I read the handed report, the exit rule's entry, the run's opening, `design/validation.md`
  → *Orphan detection*, `design/storage.md` → `docs-root`, and the route passages of
  `design/surface-contract.md`. I read no other finding's report and no other verifier's.

## Where the evidence is

- `<scratch>/verify-p2-route.kaywVo/scripts/drive.sh` — the driver; `drive-manifest.sh` —
  its variant for the setup-equivalent.
- `<scratch>/verify-p2-route.kaywVo/cand.*.log`, `prev.*.log`, `candm.*.log` — every command,
  its exit status, its standard output and standard error, in order, one file per cell.
- `<scratch>/verify-p2-route.kaywVo/runs/<cell>/` — one file per stream per step; the rigs
  themselves are beside them.
- `<scratch>/verify-p2-route.kaywVo/norm/` — the normalised logs and the seven
  candidate-against-previous diffs.

<!-- end of report -->
