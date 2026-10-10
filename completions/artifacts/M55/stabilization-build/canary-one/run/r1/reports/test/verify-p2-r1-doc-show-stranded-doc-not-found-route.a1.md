# verify-real — `r1-doc-show-stranded-doc-not-found-route`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding: door `jigc doc show`, the
clause it is said to break `working-product`, triage's grade *unclear*.

## Verdict in one paragraph

**REFUTED as a blocker — `breaks-no-clause`.** The behaviour is real and reproduces as the
finding states it: with a docs-root re-point that left `docs/research/context-loss.md`
where it was, `jigc doc show research:context-loss` exits 1 with `store.not-found` and the
generic route *create the referenced doc, or fix the reference to an existing one …*,
while the doc is committed at its prior home. No arm of that route reaches the doc. But
it breaks neither half of the clause inside the clause's scope. **The first half** — no
command that works on the previous release stops working — holds as a fact: the previous
release prints the same refusal, byte for byte, and fails the route's create arm with the
same sentence. **The second half** — every refusal's route works as printed — fails only
while an index entry whose name is not UTF-8 stands at the moment of the read. Remove
that entry, or reach the same stranded state without ever having it, and the same read
prints a different route, which names the doc's path and whose repair reaches the doc
(driven, exit 0). A tracked name that is not UTF-8 is none of the layouts the clause's
instrument is ruled over, it was planted by plumbing here, and in that state both
binaries already refuse the task-minting door for the same cause. **The verdict rests on
that reading of *a supported layout*, and no declared bound of this run states it** — see
Left open, item 1.

## The binary, asserted before anything was driven

| which | call | `content_sha256` printed | matches the prompt |
|---|---|---|---|
| candidate, label c1, commit `eeffe347` | `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

The candidate's directory went first on `PATH` and `command -v jigc` printed
`<scratch>/bin/c1.a1/jigc` before the first command and again in every candidate cell. In
the previous release's cells its own directory went first instead, so that a bare `jigc`
in a route it prints resolves to it, and every command was also typed by its absolute
path. Nothing was built; nothing under `target/` was driven. Every rig was built with
`dev/jigc-rig refs-post-hoc --binary <that path>`, one fresh rig per cell.

## What was read

The finding's block as the prompt hands it — `Repro VR-LD-2`, door `jigc config set
docs-root`, and `Left open` item 3 of the report that carries it — and nothing else of
that report. The clause: `completions/artifacts/canary-one/opening.md` → The closing
condition, and `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, second sharpening.
The design that owns the behaviour: `design/doc-read-surface.md` (the `store.not-found`
route), `design/validation.md` → Orphan detection, and the code those name —
`crates/cli/src/doc.rs`, `reroute_unadopted` and `relocated_route`, and
`crates/cli/src/orphan.rs`, `committed_markdown`.

## What was driven

Six rigs, each minted by `dev/jigc-rig` under `<scratch>/verify-p2-show.Xy2vgJ`. Every
exit status was read bare, the line after its command; stdout and stderr went to
separate files.

The plant, as the finding's block gives it, needing no file on disk:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"     # exit 0

| cell | binary | sequence | the read's exit and route |
|---|---|---|---|
| D2 | candidate | control read (exit 0) · plant · the block's four `repro` lines | exit 1, `store.not-found`, the **generic** route |
| P2 | previous | the same | the same — stderr byte-identical to D2 (`cmp` exit 0) |
| E | candidate | plant · `config set docs-root notes` · read · remove the entry · read again | first read as D2; second read exit 1, `store.not-found`, the **path-naming** route |
| F | candidate | plant · `config set` · read · remove the entry · the generic route's create arm, to its finalize | see *The route, run as printed* |
| G | candidate | no plant · `config set docs-root notes` · `git reset --hard` · read · plant · read | first read: the path-naming route; after the plant: the generic route again |
| PG | previous | the same as G | the same — both reads byte-identical to the candidate's |

### The block's four lines, cells D2 and P2

Identical on both binaries:

1. `jigc config set docs-root notes --format json` — exit 0, stderr empty, stdout
   `{"committed": false, "key": "docs-root", "op": "config-set", "relocated": [], "value": "notes"}`.
2. `jigc config get docs-root` — exit 0, `docs-root = notes  (project)`.
3. `git ls-files -- docs/research/context-loss.md notes/research/context-loss.md` —
   exit 0, `docs/research/context-loss.md`: the doc did not move.
4. `jigc doc show research:context-loss` — **exit 1**, stdout empty, stderr:

```text
blocking · store.not-found — could not read `research:context-loss` at `notes/research/context-loss.md`: No such file or directory (os error 2)
  at: research:context-loss
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:context-loss --task <task-id>` (find the task id with `jigc task list`)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

The control, before the plant, on both: the same read exits 0 and prints the doc.

## The route, run as printed

The route has two commands and two sentences. All four were followed; the first three on
both binaries, with the entry standing, as the finding's state has it.

| arm | what was run | candidate | previous |
|---|---|---|---|
| `jigc task list` | as printed | exit 0, one task, `ground-the-vision-in-research` | the same |
| `jigc doc show research:context-loss --task <task-id>` | with that id | exit 1, `store.not-staged`: *is not staged in this task and has no committed copy — nothing to read yet* | the same |
| *create the referenced doc* | `jigc start --workflow do-research "<intent>"`, the one workflow whose create-gate is `research` | exit 1, no task minted, one bare line on stderr: `` `git diff --cached` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 102 `` | the same line, byte-identical |
| *create the referenced doc*, in the task already open | `jigc doc create research --title "Context Loss" --task ground-the-vision-in-research` | exit 1, `create.gate-blocked`, allowed doctypes `[vision]` | not driven |
| *fix the reference to an existing one* | nothing to run: the address was typed, no doc holds a reference | — | — |

**So, with the entry standing, following the route neither reaches the doc nor mints a
second one.** After every arm, on both binaries: `git ls-files -- docs/research notes`
prints `docs/research/context-loss.md` alone, no `notes/` directory exists, and the
doc's bytes are unchanged (`git diff --stat HEAD -- docs/` empty).

**With the entry removed, the create arm mints a second one** (cell F, candidate only).
After `git update-index --force-remove` of the planted name, `jigc start --workflow
do-research` exits 0 and mints a task; `jigc doc create research --title "Context Loss"`
exits 0 and acks `research:context-loss`; the three slots and the commit doc are
written at exit 0; `jigc task validate` exits 0 with one advisory, `file-state.staged-copy`;
`jigc task finalize` exits 0 — *promoted notes/research/context-loss.md, 2 files
committed*. Afterwards `git ls-files -- docs/research notes` prints **both**
`docs/research/context-loss.md` and `notes/research/context-loss.md`; `jigc doc show
research:context-loss` exits 0 and serves the new one; `jigc doc list` lists the new one
alone; `jigc validate` exits 0 and still names the first as `file-state.orphaned-doc`.
No door on that path said that the identity already had a committed doc. The first doc's
bytes stand, in the work tree and in git.

That second case is not the route *as printed in the state it was printed in*: once the
entry is gone the read no longer prints that route (next section). It is what happens to
a reader who acts on the sentence after clearing the entry without reading again.

## Is it what the finding says, read against the design that owns it?

Yes for the behaviour; and it is not intended. The design gives a stranded doc its own
route. `crates/cli/src/doc.rs`, `relocated_route` (M53, the review row `(7, A7-F3)`):
when `store.not-found` is raised and the doc is committed outside the home the cascade
resolves, the read's route names the path and hands the reader `jigc validate`, because
*create it / fix the reference / read it with `--task`* are *three exits none of which
is the one `jigc validate` names*. It finds the strand through
`crate::orphan::orphaned_docs`, which walks `orphan::committed_markdown` — one `git
ls-files -z` read through `task::git_capture`, returning an **empty list** when that call
fails. With one name in the listing that is not UTF-8 the call fails, the walk sees no
committed markdown, `relocated_route` returns `None`, and the generic route stands. The
same empty listing is why `config set docs-root` relocated nothing one command earlier.
So the read's route is the generic one exactly when the listing cannot be decoded.

Driven, both directions:

- Cell E: the same rig, the entry removed, the same read — exit 1, `store.not-found`,
  and the route is now:

```text
  route: `research:context-loss` is committed at `docs/research/context-loss.md`, outside the home this read resolves — a `docs-root` / `placement-root` re-point stranded it; `jigc validate` names the repair for this store (move it to the resolved home, re-point the knob to cover where it sits, or drop it with `jigc unmanage docs/research/context-loss.md`)
```

- That route, run as printed (cell E, candidate): `jigc validate` exits 0 and names
  `file-state.orphaned-doc` at `docs/research/context-loss.md`; the second of its three
  repairs, `jigc config set docs-root docs`, exits 0; `jigc doc show
  research:context-loss` then exits 0 and prints the doc. The route reaches the doc that
  stands at its prior home and mints nothing.
- Cells G and PG: the entry planted onto a strand that arose without it — the read
  falls back to the generic route, byte-identical to D2 and P2.

## Can the stranded state be reached without the non-UTF-8 entry?

**Yes — and then the finding's defect is not there.** Cells G and PG, no plant at any
point, two ordinary commands:

1. `jigc config set docs-root notes --format json` — exit 0, `relocated` holds one pair,
   `docs/research/context-loss.md` → `notes/research/context-loss.md`, staged as a
   rename, `committed: false`. (`jigc doc show` exits 0 here.)
2. `git reset --hard` — exit 0. The staged rename is undone; `.jigc/config/manifest.yaml`
   is untracked, so the knob survives: `jigc config get docs-root` still prints `notes
   (project)`, and `git ls-files` prints `docs/research/context-loss.md` alone.

The doc is stranded, on both binaries, with no non-UTF-8 name anywhere. `jigc doc show
research:context-loss` exits 1 with `store.not-found` and the **path-naming** route, the
same bytes as cell E's second read (`cmp` exit 0), the same on the previous release.

So: the *stranded state* does not need the entry. The *generic route over a stranded
doc* — the finding — does, in every cell driven here: it appeared only while the entry
stood at the read, and never without it. Whether another cause can empty the same
listing was not driven (Bounds).

## Does it break `working-product`, inside that clause's scope?

The clause, as the run's opening names it: *a working product others can rely on*; its
measure, in the exit rule's second sharpening: *no command that works on rc.24 in a
supported layout stops working, and every refusal's route works as printed*.

**First half — unbroken, as a fact.** The door answers the same on the previous release:
the block's four lines give the same exits and the same output, and the fourth line's
stderr is byte-identical (cells D2 and P2). Nothing that works there stopped working.

**Second half — not met in the state as driven, and that state is outside the scope.**
In the state the finding reaches, the route does not work: its two commands run, its
create arm is refused at exit 1, and nothing it offers leads to the doc. Three facts
place the state outside *a supported layout*:

1. It exists only while an index entry whose name is not UTF-8 stands at the read. With
   the entry absent, the same stranded doc gets the route the design gives it, and that
   route works as printed to the doc (cells E, G, PG).
2. The layouts the clause's instrument was ruled over are nine, listed in
   `implementation/decisions-pending.md` → *The regression set, part 2*: line-ending
   conversion, hidden untracked files, a symlinked `CLAUDE.md`, linked worktrees, a
   worktree of a bare repository, a separate git directory, a submodule, an older git,
   a moved repository. A tracked name that is not UTF-8 is none of them. Twice before,
   the record treats such a name as a declared bound of a door rather than a layout it
   serves (`DECISIONS.md`, *a non-UTF-8 entry name is narrated, not kept*; and the
   invocation log's lossy read of a non-UTF-8 argument).
3. As built here the entry was planted by git plumbing, and this host's filesystem
   cannot hold a file of that name — the candidate's own orientation says so (`jigc
   start`: *findings: unknown — `git checkout-index …` failed … Illegal byte sequence*).
   And in that state both binaries refuse to mint any task: `jigc start --workflow
   do-research` exits 1 on *`git diff --cached` produced non-UTF-8 output*. The product
   does not carry its core loop in that state on the previous release either, so the
   refusal's route is not something a user relied on there and lost.

A real defect, then, that breaks no clause inside its scope: `breaks-no-clause`, never
`does-not-reproduce`. It stays a row of the ledger.

**What this verdict does not settle.** The run's opening declares no bound, and no text
names a non-UTF-8 tracked name as unsupported. Fact 2 is a reading of the instrument's
ruled shape, not a ruling on this layout. If the human rules that a repository tracking
such a name is a supported layout, the second half is broken as driven here and this row
is confirmed — with `regression: false`, the previous release being red on the same
block (cells P2, PG).

## The regression fact

Not owed with this verdict. The fact, since both binaries were driven: the block is red
on the previous release exactly as on the candidate — the read's refusal and the create
arm's refusal are byte-identical (`cmp` exit 0 on both pairs). Not a regression.

## Class

**instance, unbounded.** One doctype (`research`, a `location:` home), one knob
(`docs-root`), one address, one host, one git. The `placement-root` strand and the
recorded-prior-home arm of `relocated_route` were not driven. Read, not derived: `grep`
finds five call sites of `committed_markdown(` under `crates/cli/src` (`config.rs` 1,
`orphan.rs` 3, `relocate.rs` 1); which read doors share the fallback to the generic
route was not enumerated.

## Repro VR-DS-1

```yaml
claim: "on a doc a docs-root re-point left at its prior home, `jigc doc show` exits 1 store.not-found with the route *create the referenced doc …* while the doc stands — and that breaks the clause working-product"
verdict: "REFUTED as a blocker — breaks-no-clause. The behaviour reproduces on both binaries; the route's failure is confined to a state holding an index entry whose name is not UTF-8."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
plant:                      # needs no file on disk; the name is passed as raw bytes
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
unplant:
  - ["git", "update-index", "--force-remove", "--", "bad<byte 0xFF>name.txt"]                 # exit 0
cells:
  - cell: "the finding — the generic route over a doc that stands (D2, P2)"
    setup:
      - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <binary>
      - plant
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]      # exit 0, relocated: []
    repro:
      - ["jigc", "doc", "show", "research:context-loss"]
      - ["jigc", "task", "list"]
      - ["jigc", "doc", "show", "research:context-loss", "--task", "ground-the-vision-in-research"]
      - ["jigc", "start", "--workflow", "do-research", "re-create the research the read could not find"]
      - ["git", "ls-files", "--", "docs/research", "notes"]
    expect:
      - exit: 1
        stdout: ""
        stderr_contains: ["store.not-found", "route: create the referenced doc, or fix the reference to an existing one"]
      - exit: 0
        stdout_contains: "ground-the-vision-in-research  [form-vision]"
      - exit: 1
        stderr_contains: ["store.not-staged", "has no committed copy"]
      - exit: 1
        stderr: "`git diff --cached` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 102"
      - exit: 0
        stdout: "docs/research/context-loss.md"          # not reached, and no second doc
    on-previous: "identical — the first and fourth stderr compared byte for byte"
  - cell: "the control the verdict rests on — the same strand, the entry removed (E)"
    setup:
      - fixture: refs-post-hoc
      - plant
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]
      - unplant
    repro:
      - ["jigc", "doc", "show", "research:context-loss"]
      - ["jigc", "validate"]
      - ["jigc", "config", "set", "docs-root", "docs", "--format", "json"]
      - ["jigc", "doc", "show", "research:context-loss"]
    expect:
      - exit: 1
        stderr_contains: ["store.not-found", "is committed at `docs/research/context-loss.md`, outside the home this read resolves", "jigc unmanage docs/research/context-loss.md"]
      - exit: 0
        stdout_contains: ["file-state.orphaned-doc", "docs/research/context-loss.md"]
      - exit: 0
        stdout_json: { "committed": false, "key": "docs-root", "op": "config-set", "relocated": [], "value": "docs" }
      - exit: 0
        stdout_contains: "# Context Loss"
    on-previous: "not driven"
  - cell: "the strand without the entry, by two ordinary commands (G, PG)"
    setup:
      - fixture: refs-post-hoc
    repro:
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]
      - ["git", "reset", "--hard"]
      - ["jigc", "config", "get", "docs-root"]
      - ["git", "ls-files", "--", "docs/research", "notes"]
      - ["jigc", "doc", "show", "research:context-loss"]
    expect:
      - exit: 0
        stdout_json: { "committed": false, "key": "docs-root", "op": "config-set", "relocated": [ { "from": "docs/research/context-loss.md", "to": "notes/research/context-loss.md" } ], "value": "notes" }
      - exit: 0
      - exit: 0
        stdout: "docs-root = notes  (project)"
      - exit: 0
        stdout: "docs/research/context-loss.md"
      - exit: 1
        stderr_contains: ["store.not-found", "is committed at `docs/research/context-loss.md`, outside the home this read resolves"]
        stderr_not_contains: "create the referenced doc"
    then: "plant, read again: exit 1 and the generic route of the first cell, byte-identical"
    on-previous: "identical — ack and both refusals compared byte for byte"
  - cell: "the create arm once the entry is gone — a second doc (F, candidate only)"
    setup:
      - fixture: refs-post-hoc
      - plant
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]
      - unplant
      - ["jigc", "start", "--workflow", "do-research", "re-create the research the read could not find"]   # exit 0, task re-create-the-research
      - ["jigc", "doc", "create", "research", "--title", "Context Loss", "--task", "re-create-the-research"]   # exit 0, research:context-loss
      - "the three research slots and the commit doc's type, scope, summary and body, each written at exit 0"
    repro:
      - ["jigc", "task", "finalize", "re-create-the-research"]
      - ["git", "ls-files", "--", "docs/research", "notes"]
    expect:
      - exit: 0
        stdout_contains: "promoted notes/research/context-loss.md"
      - exit: 0
        stdout: "docs/research/context-loss.md\nnotes/research/context-loss.md"
    on-previous: "not driven"
observed: "<scratch>/verify-p2-show.Xy2vgJ — d2.* (cell D2 and the route's arms), p2.* (P2), e.* (E), f.* (F), g.* (G), pg.* (PG); the rigs are the six jigc-rig-refs-post-hoc-* directories beside them"
pinned-by: "UNPINNED: the first and last cells hold a defect as observed; the second and third state the facts the verdict rests on and were not checked against the suites — see Pinnable"
```

**Pinnable as it stands: no.** The second and third cells convert as they are — plain
argv, expected exits, substring assertions — and are the two facts this verdict wants to
stay true: a strand's read names the path, and its repair reaches the doc. Whether a
suite already asserts them was not checked; the finding makes no coverage claim, so none
was verified. The first and fourth cells, pinned green, would hold in place a route that
tells a reader to create a doc that exists, and a second doc of one identity: each is a
fix's red test in waiting, with its `expect` inverted, if triage schedules one. The plant
passes a name as raw bytes, so any test that uses it is Unix-only.

## Left open

1. **The scope question this verdict rests on.** Whether a repository tracking a name
   that is not UTF-8 is *a supported layout* of the second clause. The run declares no
   bound and no text rules on it. Ruled supported, the second half of the clause is
   broken as driven here and this row is confirmed, `regression: false`.
2. **`jigc start --workflow <id>` with the entry standing** exits 1 with one bare line —
   *`git diff --cached` produced non-UTF-8 output …* — no code, no `at:`, no route. Both
   binaries, byte-identical. A refusal with no route at the task-minting door.
3. **`jigc doc show <addr> --task <id>` on the stranded doc** says *has no committed
   copy* over a doc committed at its prior home (`store.not-staged`). Both binaries, with
   the entry standing; not driven without it.
4. **A second doc of one identity, minted at exit 0** (cell F). With a strand standing
   and no undecodable name, `jigc doc create research --title "Context Loss"` and its
   task's finalize commit `notes/research/context-loss.md` beside the committed
   `docs/research/context-loss.md`; no door on the way names the first, and `jigc doc
   list` then lists the new one alone. Candidate only. No byte was lost.
5. **`jigc doc list` omits the stranded doc while the entry stands** — exit 0, four rows,
   no `research:context-loss` (cell D2, candidate). This round's one included door; seen
   once, not pursued, not driven without the entry or on the previous release.
6. **`jigc start`, the orientation, prints an absolute temporary host path** inside
   *findings: unknown — `git checkout-index -a --prefix=…` failed* at exit 0 while the
   entry stands (cell D2, candidate). Not driven on the previous release.
7. **`config set docs-root` followed by `git reset --hard` strands every relocated
   doc** — the knob's file is untracked, so the reset undoes the move and not the knob
   (cells G, PG). The read then routes correctly; whether the door should say so at the
   ack was not examined.

## Bounds — what this verification did not do

- One macOS host, one git. On a filesystem that admits the name, the entry is an
  ordinary committed file; that was not driven.
- The `placement-root` strand, the recorded-prior-home arm, and any other cause of an
  empty listing (a failing `git ls-files`) were not driven.
- Of the path-naming route's three repairs, one was run — the knob re-point — on the
  candidate, in cell E. *Move it* and `jigc unmanage` were not.
- Cells E and F were not driven on the previous release.
- No coverage was verified; no suite was read.
- Nothing was built, edited, staged or committed. The clone stands at `eeffe347` on
  `fix/canary-one`, its only untracked path the run's own `r1/` directory, as before.

## Where the evidence is

`<scratch>/verify-p2-show.Xy2vgJ`: the six rigs, `rig-*.env` and `rig-*.err` (each
rig's assignments and construction log), and the cells' captured stdout and stderr —
`d2.*`, `p2.*`, `e.*`, `f.*`, `g.*`, `pg.*`.

<!-- end of report -->
