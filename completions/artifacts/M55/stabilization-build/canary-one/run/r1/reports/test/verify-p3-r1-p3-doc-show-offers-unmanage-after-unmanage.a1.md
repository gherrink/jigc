# verify-real — `r1-p3-doc-show-offers-unmanage-after-unmanage`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: item 2
under *Left open* of
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-orphaned-doc-route-after-strand-not-run.a1.md`
— after the unmanage arm of the strand route, `jigc doc show research:context-loss` still says a
re-point stranded the doc and still offers `jigc unmanage docs/research/context-loss.md`, which
by then does nothing. Door: `jigc doc show`. Clause it is said to break: `working-product`.
Triage's grade: *unclear*. The finding came with no repro block; the one below is mine.

## Verdict in one paragraph

**`refuted` as a blocker — basis `breaks-no-clause`. The defect is real and stays a row.** It
reproduces exactly as the finding states it, from nothing, on both binaries: once the stranded
doc has been dropped with `jigc unmanage`, the read refuses at exit 1 with `store.not-found`
and prints, byte for byte, the refusal it printed before the drop — three repairs, the third
being `jigc unmanage docs/research/context-loss.md`. Each of the three was then run as printed,
on a fresh rig each. **Moving the doc to the resolved home ends the refusal** (read at exit 0).
**Re-pointing the knob ends the refusal** (read at exit 0). **The printed `jigc unmanage …`
exits 0, prints `no-op: docs/research/context-loss.md is not managed (nothing to drop)`, writes
nothing, and the refusal stands unchanged.** So one of the route's three alternatives is
offered in a state where it has nothing left to do, and the route attributes its list to
`jigc validate`, which in that same state names a different repair. That is a surface defect.
It breaks no clause inside its scope: no command fails, nothing that works on the previous
release stops working — the two binaries agree on every one of twenty cells, byte for byte
after normalisation — and no reader is left without a working repair. **The reading that
decides this is stated under *Does it break the clause*; a stricter reading of one sentence
would turn the verdict, and I name it there rather than settle it.**

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` —
  the hash the prompt gives for the candidate (commit
  `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` —
  the previous release's (1.0.0-rc.24). **It was driven**, because triage asked for both.
- In every cell the driven binary's directory went first on `PATH`, and the driver stops
  unless `command -v jigc` prints that binary's path, unless the rig — built with
  `dev/jigc-rig refs-post-hoc --binary <that path>` — reports the same path as `$JIGC`, and
  unless the rig's repository lies under the cell's own run directory. No cell stopped.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## What was driven

**Twenty cells: 2 binaries × 2 setups × 5 arms, one fresh rig each**, all of the state
`refs-post-hoc`, minted under `<scratch>/verify-p3-show-unmanage.bKDMgX/runs/`. Every command
whose exit status is read ran bare, its standard output and standard error each to a file of
its own; nothing was read through a pipe. Nothing of any other agent's scratch root was used.

The two setups that reach the strand:

    # plant — Repro VR-ROUTE-1's `setup`, as written in the handed report
    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"   # exit 0
    jigc config set docs-root notes                                            # exit 0, moves nothing
    git update-index --force-remove -- "bad<byte 0xFF>name.txt"                # exit 0

    # manifest — VR-ROUTE-1's `setup-equivalent`: no planted entry
    write .jigc/config/manifest.yaml = "scalar:\n  docs-root: notes\n"

Then, in every cell, the unmanage arm of VR-ROUTE-1 and the read triage asked for:

    jigc unmanage docs/research/context-loss.md
    jigc doc show research:context-loss

**What I changed from the block as handed: nothing in the setup or the arm.** The finding names
no command for the three repairs, so they are spelled from the refusal's own text:

- *drop it with `jigc unmanage docs/research/context-loss.md`* — **not retyped**: the driver
  reads the backticked span off the refusal's standard error and runs that string through
  `sh -c`. The span it read, in all four cells of this arm, is
  `jigc unmanage docs/research/context-loss.md`.
- *re-point the knob to cover where it sits* — `jigc config set docs-root docs`; the rig's root
  before the strand is `docs-root = docs/  (pack-default)`.
- *move it to the resolved home* — the refusal's first line names that home,
  `notes/research/context-loss.md`. Driven twice, because the prose names no tool:
  `mkdir -p notes/research` then `git mv <old> <new>`; and the same with a plain `mv`.
- a fifth arm, *none*, is the control: nothing is run between the two reads.

### The state every cell reaches (identical text on both binaries, in both setups)

| step | exit | what it printed |
|---|---|---|
| `jigc config get docs-root` after the strand | 0 | `docs-root = notes  (project)` |
| `git ls-files -- docs/research/context-loss.md notes/research/context-loss.md` | 0 | `docs/research/context-loss.md` — it did not move |
| `jigc validate` | 0 | `advisory · file-state.orphaned-doc` for the doc, route *move it under the current resolved root (re-point `docs-root` to cover it) or drop it with `jigc unmanage`* |
| `jigc doc list` | 0 | four rows; `research:context-loss` has none |
| `jigc doc show research:context-loss` | 1 | the refusal below |
| **`jigc unmanage docs/research/context-loss.md`** | 0 | `unmanaged docs/research/context-loss.md — dropped its file-state baseline; the file is left on disk` |
| **`jigc doc show research:context-loss`** — the finding | **1** | **the same refusal, the same bytes** |
| `jigc validate` | 0 | the orphan advisory is gone; `advisory · file-state.unregistered-doc` for the same path in its place (verbatim below) |
| `jigc doc list` | 0 | four rows |

The refusal, verbatim, on standard error; standard output is empty:

    blocking · store.not-found — could not read `research:context-loss` at `notes/research/context-loss.md`: No such file or directory (os error 2)
      at: research:context-loss
      route: `research:context-loss` is committed at `docs/research/context-loss.md`, outside the home this read resolves — a `docs-root` / `placement-root` re-point stranded it; `jigc validate` names the repair for this store (move it to the resolved home, re-point the knob to cover where it sits, or drop it with `jigc unmanage docs/research/context-loss.md`)

`cmp` of the read's standard error **before** the unmanage arm against **after** it exits 0 on
both binaries: the drop changes nothing the read says. `cmp` of the plant setup's refusal
against the manifest setup's exits 0, and of the candidate's against the previous release's
exits 0.

What `jigc validate` says of the same doc at that moment, verbatim but for the path:

    advisory · file-state.unregistered-doc — committed doc `docs/research/context-loss.md` looks managed (it sits under a `research`-style directory) but was never adopted — a basename coincidence or an un-ingested foreign doc, not a tracked strand
      at: docs/research/context-loss.md
      route: adopt it with `jigc migrate <the doc's absolute path> --as research`, or ignore it if it is not meant to be managed

So at one moment the read says *a re-point stranded it; `jigc validate` names the repair …
drop it with `jigc unmanage`*, and `jigc validate` says *not a tracked strand* and names
`jigc migrate … --as research` or nothing. The two surfaces disagree, and the read's sentence
about what the other one names is not true of it.

### The three repairs, each run as printed — what stands afterwards

One fresh rig per row and per binary and per setup; where a cell holds one value it is the
value in all four.

| arm | the repair, as run | its exit | `jigc doc show research:context-loss` | `jigc doc list` | `jigc validate` |
|---|---|---|---|---|---|
| none (control) | nothing | - | **exit 1**, the refusal | four rows | exit 0, `file-state.unregistered-doc` |
| **unmanage** | the span read off the refusal, through `sh -c` | **0** | **exit 1, the refusal, the same bytes** (`cmp` exit 0) | four rows | exit 0, `file-state.unregistered-doc` — nothing changed |
| **repoint** | `jigc config set docs-root docs` | 0 | **exit 0**, the doc | five rows, `research:context-loss  docs/research/context-loss.md  managed` | exit 0; `file-state.un-baselined` for the doc, route *no action needed* |
| **move, `git mv`** | `mkdir -p notes/research`, `git mv` | 0, 0 | **exit 0**, the doc | five rows, the doc at `notes/research/context-loss.md`, `managed` | exit 0; `file-state.un-baselined` for the new path |
| **move, plain `mv`** | `mkdir -p notes/research`, `mv` | 0, 0 | **exit 0**, the doc | five rows, the doc at `notes/research/context-loss.md`, `managed` | exit 0; `file-state.un-baselined` for the new path, and `file-state.unregistered-doc` for the old one, which the index still lists |

What the unmanage arm printed, on standard output, standard error empty:

    no-op: docs/research/context-loss.md is not managed (nothing to drop)

**No byte moved or was lost in any cell.** `git rev-parse HEAD` prints the same commit at the
first and the last step of each of the twenty cells; `git hash-object` of the doc prints
`3507022f2299fc2746a813610ea9732edd8d58ff` at the first step and at the last in every cell, at
whichever of the two paths the doc then sits. No cell committed anything. The only non-zero
exits in the twenty logs are the reads the tables above show at exit 1.

### Candidate against the previous release

Each of the ten pairs of logs was compared whole after replacing the binary's path, the run
directory's and the rig's minted names, the cell's label, and the commit ids: `diff` exits 0
on all ten, with no differing line. Same exits, same standard output, same standard error,
same files.

## Is it what the finding says?

Yes, word for word: the read *still says a re-point stranded the doc*, it *offers
`jigc unmanage docs/research/context-loss.md`*, that command *is by then a no-op*, and *its
other two repairs still apply*. The finding is neither stale nor an artefact of the reporter's
rig.

Read against the design that owns it, it is not intended behaviour either:

- `DECISIONS.md`, the entry that begins *`doc show` over a relocated doc routes at the repair
  the store sweep gives* (`(7, A7-F3)`): the read *asks the two walks that own where a managed
  document lives in the order `jigc validate` asks them*. The route's own sentence —
  *`jigc validate` names the repair for this store* — says the same.
- `design/validation.md` → *Orphan detection* → *M40 two-tier route*: the sweep discriminates
  on `FileStateRecord` membership. A **registered** path is a strand and takes the *unmanage*
  route; an **unregistered** one takes *`jigc migrate <path> --as <doctype>` or ignore* —
  because `unmanage` on an unregistered doc is, in that section's words, *a proven no-op loop
  (`unmanage` on a never-registered doc is a clean no-op; the advisory re-emits, exit 0)*.
- The read's producer, `relocated_route` in `crates/cli/src/doc.rs` (lines 4673-4721 at the
  candidate's commit), runs the strand walk `crate::orphan::orphaned_docs` and formats the
  strand repair for whatever it returns. That walk is keyed on the path's shape and carries no
  registration, so the read gives the registered tier's repair in the unregistered tier.

So the read departs from the sweep exactly where the sweep was changed at M40 to stop offering
this command. No settled decision intends that, and the finding does not argue that one is
wrong: nothing is contested.

What bounds the defect: the same refusal, with the same third repair, also stands after the
**first**, real drop — the read is equally refused whether `unmanage` dropped something or
not. The *drop it* alternative never was a way to make the read succeed; it is the way to stop
jigc managing the doc. What is wrong after the drop is that it is offered again, and that the
route cites the sweep for it.

## Does it break `working-product`, inside that clause's scope?

**No.**

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  second clause is *a working product others can use and rely on*; its instrument, in the
  second sharpening, is *no command that works on rc.24 in a supported layout stops working,
  and every refusal's route works as printed*. The run's opening names the clause
  `working-product`.
- **First half — nothing stopped working.** Ten pairs of whole logs, no differing line.
- **Second half — the refusal's route, as printed.** `store.not-found` at exit 1 is a refusal,
  so the sentence reaches it. Its route prints one command and two prose repairs. The command
  parses, runs and exits 0. Both prose repairs end the refusal. Nothing the route names fails,
  and a reader who follows it is never without a repair that works.
- **The reading I applied, and the one I did not.** I read *works as printed* as: what the
  route prints can be run as it stands, does not fail, and leaves the store in the state the
  repair names — here *dropped*, which holds, and which the command says in so many words.
  Under the stricter reading — *every alternative a route offers must change the state it is
  offered in* — the third alternative does not, and the finding would be `confirmed` with
  `regression: false`. I did not take that reading because the clause speaks of a product that
  works and can be relied on, and here every printed thing runs and two of three alternatives
  resolve the refusal; but it is a reading of the human's sentence, and if triage holds the
  stricter one the facts above already carry the verdict.
- **Scope.** Both setups reach the strand outside the ordinary door: one by a planted index
  entry that makes `jigc config set docs-root` land its knob without moving the doc, one by
  writing the knob into the manifest by hand. The state is then reached by following jigc's own
  printed route (`jigc unmanage`). I did not rest the verdict on the layout being unsupported.

## The regression fact

**Not owed, and so not returned**: step 4 runs with `confirmed` only. What was established on
the way stands as evidence all the same: every cell was driven on the previous release, and it
behaves there exactly as on the candidate. `git diff` between the previous release's commit
and the candidate's shows no hunk naming `relocated_route` or the strand route's text.

## Class

**Driven: one arm of one producer, at one doc — `instance, unbounded`.** The route is built at
one place, `relocated_route`, arm (2); its strand walk serves two home kinds, `location:` and
`placement:`. 1 of those 2 was driven: one doctype (`research`), one doc, one value of the
knob, text output. I did not enumerate the walk's other consumers.

## Coverage

The finding makes no coverage claim; this is for the block's `pinned-by` only, read from the
suites and not from a diff. `crates/cli/tests` (511 `.rs` files, recursively) and
`tooling-tests` (22) were searched. `crates/cli/tests/doc_show_relocated.rs` holds the read's
strand route in four tests, each from a state where the sweep reports
`file-state.orphaned-doc` — the registered tier; none runs `unmanage`. 14 files name
`"unmanage"` as an argv word; 2 of them also name `store.not-found` or the route's phrase
(`fixed_identity_axis.rs`, `placement_override.rs`), and neither unmanages a stranded path and
then reads it — `placement_override.rs:882` unmanages a doc **at** its resolved home. So no
standing test reads a stranded doc after its baseline is gone. The in-module tests of both
crates were not enumerated.

## Repro VR-SHOW-1

```yaml
claim: "after the unmanage arm of a docs-root strand, `jigc doc show research:context-loss` still says a re-point stranded the doc and offers `jigc unmanage docs/research/context-loss.md`, by then a no-op — breaking the clause working-product"
verdict: "REFUTED as a blocker — breaks-no-clause. The behaviour reproduces as stated on the candidate and on the previous release alike; every printed repair runs at exit 0 and two of the three end the refusal."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
setup:                      # one fresh rig per arm; driven on both binaries
  - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <the binary>
  - "write .jigc/config/manifest.yaml = `scalar:\n  docs-root: notes\n`"
  - ["jigc", "unmanage", "docs/research/context-loss.md"]      # exit 0: "unmanaged … dropped its file-state baseline"
setup-as-handed:            # VR-ROUTE-1's own setup; driven on both binaries, same outcomes
  - fixture: refs-post-hoc
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]
  - ["jigc", "config", "set", "docs-root", "notes"]
  - ["git", "update-index", "--force-remove", "--", "bad<byte 0xFF>name.txt"]
  - ["jigc", "unmanage", "docs/research/context-loss.md"]
precondition:
  repro:
    - ["jigc", "doc", "show", "research:context-loss"]
    - ["jigc", "validate"]
  expect:
    - exit: 1
      stdout: ""
      stderr_contains: "blocking · store.not-found — could not read `research:context-loss` at `notes/research/context-loss.md`"
    - exit: 0
      stdout_lacks: "file-state.orphaned-doc"
      stdout_contains: "advisory · file-state.unregistered-doc — committed doc `docs/research/context-loss.md`"
arms:
  - arm: "move it to the resolved home"
    repro:
      - ["mkdir", "-p", "notes/research"]
      - ["git", "mv", "docs/research/context-loss.md", "notes/research/context-loss.md"]
      - ["jigc", "doc", "show", "research:context-loss"]
    expect:
      - exit: 0
      - exit: 0
      - exit: 0
        stdout_contains: "# Context Loss"
  - arm: "re-point the knob to cover where it sits"
    repro:
      - ["jigc", "config", "set", "docs-root", "docs"]
      - ["jigc", "doc", "show", "research:context-loss"]
    expect:
      - exit: 0
        stdout_contains: "config: set `docs-root` = `docs`"
      - exit: 0
        stdout_contains: "# Context Loss"
  - arm: "drop it with jigc unmanage — the command span, as the refusal prints it"
    repro:
      - ["jigc", "unmanage", "docs/research/context-loss.md"]
      - ["jigc", "doc", "show", "research:context-loss"]
    expect:
      - exit: 0
        stdout_contains: "no-op: docs/research/context-loss.md is not managed (nothing to drop)"
      - exit: 1
        stderr_contains: "store.not-found"
observed-defect:            # true of both binaries today; NOT an assertion to pin — a fix inverts it
  after: "the setup, with or without the third arm"
  repro:
    - ["jigc", "doc", "show", "research:context-loss"]
  observed:
    exit: 1
    stderr_contains: "a `docs-root` / `placement-root` re-point stranded it; `jigc validate` names the repair for this store (move it to the resolved home, re-point the knob to cover where it sits, or drop it with `jigc unmanage docs/research/context-loss.md`)"
  beside: "`jigc validate` in the same state prints `file-state.unregistered-doc` … `not a tracked strand`, route `jigc migrate <path> --as research` or ignore"
invariant: "in every arm `git rev-parse HEAD` is unchanged and `git hash-object` of the doc is 3507022f2299fc2746a813610ea9732edd8d58ff, at whichever path it sits"
observed: "<scratch>/verify-p3-show-unmanage.bKDMgX — logs/<cand|prev>.<plant|manifest>.<none|unmanage|repoint|gitmv|mv>.log; norm/ holds the normalised logs and the ten diffs"
pinned-by: "UNPINNED: no standing test reads a stranded doc after `jigc unmanage` dropped its baseline — see Coverage"
```

**Pinnable as it stands: yes, with one part held back.** `setup`, `precondition` and the three
`arms` are plain argv over a state `trial_corpus.rs` already builds, need no raw byte, and
every `expect` states behaviour that should stay true whatever becomes of the defect: the read
refuses while the doc is off its home, two repairs end the refusal, and a repeated `unmanage`
is an honest exit-0 no-op. **`observed-defect` must not be pinned**: it is the stale offer
itself, and a test asserting it would hold the defect in place. A fix's red test is its
inverse — after the drop, the read's route no longer offers `jigc unmanage`, and what it says
`jigc validate` names is what `jigc validate` names.

## Left open

1. **After the drop and either working repair, `jigc doc list` shows the doc as `managed`
   again**, with no adopt step, and `jigc validate` calls it `file-state.un-baselined`, *no
   action needed — the doc is baselined on its next author or finalize*. So `jigc unmanage`
   followed by a move or a re-point ends with the doc back in the listing. The same on both
   binaries. Read against nothing; not graded; the round's door is `jigc doc list`.
2. **`file-state.unregistered-doc` says the doc *was never adopted* of a doc that was adopted
   and then unmanaged**, and its route's `jigc migrate` argv carries the doc's absolute path.
   Another door (`jigc validate`); its route was not run.
3. **After a plain `mv`, `jigc validate` raises `file-state.unregistered-doc` for the old path
   and routes at `jigc migrate <old path> --as research`** — a file no longer on disk, still in
   the index. A state of my own arm's making (an unstaged move); the route was not run.
4. **The `placement:` home kind of the same route** (a `placement-root` strand, then
   `jigc unmanage`, then the read), and **`--format json`** of every cell. Not driven.
5. **Nothing was finalized or committed after any arm.** The rig holds a live task with a
   staged edge on the committed vision; whether a task's finalize behaves after each arm was
   not driven.

## Bounds — what this verification did not do

- One doc, one doctype, one value of the knob (`notes`), one way back (`docs`); text output;
  one macOS host.
- *Move it to the resolved home* was spelled two ways by me; the route names no tool.
- `jigc config set docs-root docs` leaves a project-layer value `docs` where the rig began
  with the pack default `docs/`. Clearing the override instead was not driven.
- The read with `--task`, and the read of a `#fragment`, were not driven.
- I read the handed report, the exit rule's entry, the run's opening, `design/validation.md`
  → *Orphan detection*, the three laws and the route fence of `design/surface-contract.md`,
  the `(7, A7-F3)` entry of `DECISIONS.md`, `implementation/pinning.md` §3, and the producer
  in `crates/cli/src/doc.rs`, `orphan.rs` and `unmanage.rs`. I read no other finding's report
  and no other verifier's.

## Where the evidence is

- `<scratch>/verify-p3-show-unmanage.bKDMgX/scripts/drive.sh` — the driver: label, binary,
  setup, arm.
- `<scratch>/verify-p3-show-unmanage.bKDMgX/logs/` — twenty logs, one per cell: every command,
  its exit status, its standard output and standard error, in order.
- `<scratch>/verify-p3-show-unmanage.bKDMgX/runs/<cell>.<minted>/` — one file per stream per
  step, and the rig itself beside them.
- `<scratch>/verify-p3-show-unmanage.bKDMgX/norm/` — the normalised logs and the ten
  candidate-against-previous diffs, all empty.

<!-- end of report -->
