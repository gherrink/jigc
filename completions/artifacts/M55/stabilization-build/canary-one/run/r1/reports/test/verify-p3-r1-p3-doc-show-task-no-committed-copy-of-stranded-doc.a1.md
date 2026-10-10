# verify-real — `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding: door `jigc doc show`, the
clause it is said to break `working-product`, triage's grade *unclear*.

## Verdict in one paragraph

**CONFIRMED, `regression: false`.** With a doc stranded by two ordinary commands and no
planted index entry, `jigc doc show research:context-loss --task <task>` exits 1 with
`store.not-staged`, says the doc *has no committed copy — nothing to read yet*, and routes
at *create or author the doc in this task first*. The doc is committed, at
`docs/research/context-loss.md`, and the task-less read of the same address in the same
state says so in its own route. Three attempts to refute it failed: it reproduces on a
fresh rig; the design that owns the block says it is *routed on the real state* and
reserves this message for *nothing exists anywhere*; and the route, run as printed, does
not work — in the task the read named, both acts it names are refused at exit 1, and in a
task whose gate grants them it yields a blank doc under the identity of the committed one,
never the doc. The state is reached by `jigc config set` and `git reset --hard`, and the
design names it as an ordinary one. The previous release answers every step with the same
bytes, so this is not a regression. **The verdict rests on one reading of *works as
printed*, stated in Left open, item 1.**

## The binary, asserted before anything was driven

| which | call | `content_sha256` printed | matches the prompt |
|---|---|---|---|
| candidate, label c1, commit `eeffe347` | `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

In every candidate cell the candidate's directory went first on `PATH` and `command -v
jigc` printed `<scratch>/bin/c1.a1/jigc` before the first command. Nothing was built and
nothing under `target/` was driven. Every rig was built with `dev/jigc-rig refs-post-hoc
--binary <that path>`, one fresh rig per cell, under a directory of my own minted with
`mktemp -d` below the scratch root. The previous release was driven twice over: in four
cells with its own directory first on `PATH` (so a bare `jigc` resolves to it; `command
-v` printed its path), and in one cell with every call typed by its absolute path and
`PATH` untouched.

## What was read

The finding as the prompt hands it, and its block: the second row of the table under *The
route, run as printed* in the report that carries it, with that report's `Repro VR-DS-1`,
third cell, which triage asks to be re-driven. The clause: the run's opening → The closing
condition, and `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, second sharpening.
The design that owns the behaviour: `design/doc-read-surface.md` → the `--task` read (the
`store.not-staged` bullet), `design/validation.md` → Orphan detection, and the code those
name — `crates/engine/src/store.rs`, `read_slice_staged` and `not_staged_block`, and
`crates/cli/src/doc.rs`, `reroute_unadopted` and `relocated_route`.

## What was driven

Nine rigs. Every exit status was read bare, on the line after its command; stdout and
stderr went to separate files.

| cell | binary | sequence | what it shows |
|---|---|---|---|
| C | candidate | controls · the strand · both reads | the finding, as claimed |
| P | previous, `PATH` | the same | the same bytes |
| PA | previous, absolute path | the strand · both reads · the route's create arm | the same bytes |
| CI | candidate | the strand · the staged read · the route followed in the task the read named | every arm refused at exit 1 |
| PI | previous, `PATH` | the same | the same bytes |
| CN | candidate | the strand · a task whose gate grants `research` · the route followed there | exit 0, and a blank doc of the same identity |
| PN | previous, `PATH` | the same | the same bytes |
| CK | candidate | no strand · the same doors in a granting task | the control: the committed doc is found and copied in |
| PK | previous, `PATH` | the same | the same bytes |

### The strand, by two ordinary commands (cells C, P, PA)

1. `jigc config set docs-root notes --format json` — exit 0; `relocated` holds one pair,
   `docs/research/context-loss.md` → `notes/research/context-loss.md`; `committed: false`.
2. `git reset --hard` — exit 0. The staged rename is undone; the knob's file is untracked
   and survives.
3. `jigc config get docs-root` — exit 0, `docs-root = notes  (project)`.
4. `git ls-files -- docs/research notes` — exit 0, `docs/research/context-loss.md` alone.

No index entry was planted at any point; `git status --short` prints the untracked
`.jigc/config/manifest.yaml` and nothing else.

### The two reads of one address, in that state (cell C)

`jigc doc show research:context-loss` — **exit 1**, stdout empty, stderr:

```text
blocking · store.not-found — could not read `research:context-loss` at `notes/research/context-loss.md`: No such file or directory (os error 2)
  at: research:context-loss
  route: `research:context-loss` is committed at `docs/research/context-loss.md`, outside the home this read resolves — a `docs-root` / `placement-root` re-point stranded it; `jigc validate` names the repair for this store (move it to the resolved home, re-point the knob to cover where it sits, or drop it with `jigc unmanage docs/research/context-loss.md`)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

`jigc doc show research:context-loss --task ground-the-vision-in-research` — **exit 1**,
stdout empty, stderr:

```text
blocking · store.not-staged — `research:context-loss` is not staged in this task and has no committed copy — nothing to read yet
  at: research:context-loss
  route: create or author the doc in this task first — a staged copy exists only after a write
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

With `--format json` the same read exits 1 and carries the same `message` and `route`
under `code: store.not-staged`.

One binary, one state, one address: the first read says the doc *is committed at*
`docs/research/context-loss.md`; the second says it *has no committed copy*.

**The control, before the strand, same rig:** the task-less read exits 0 and prints the
doc; the `--task` read exits 1 with `store.not-staged` — *is not staged in this task —
only its committed copy exists*, route `jigc doc show research:context-loss`. So the
door knows how to say that a committed copy stands; the strand is what takes that
answer away.

## The route, run as printed

The route is one sentence: *create or author the doc in this task first — a staged copy
exists only after a write*. It names two acts and one task.

### In the task the read named (cells CI, PI)

| arm | what was run | exit | answer |
|---|---|---|---|
| *create* | `jigc doc create research --title "Context Loss" --task ground-the-vision-in-research` | 1 | `create.gate-blocked` — *the workflow does not allow `jigc doc create research` in-task; allowed doctypes: [vision]* |
| *author* | `jigc doc author research --from-file <payload> --task ground-the-vision-in-research` | 1 | the same block |
| *a write* | `jigc doc set-slot research:context-loss#findings --from-file <prose> --task ground-the-vision-in-research` | 1 | one bare line: *no staged instance … nothing in this task provisions it; create `research` from a task minted on a workflow that grants it* |
| the read again | `jigc doc show research:context-loss --task ground-the-vision-in-research` | 1 | the same `store.not-staged` block |

Afterwards `git ls-files -- docs/research notes` prints `docs/research/context-loss.md`
alone and `git status --short` is unchanged. **No arm of the route can be taken in the
task it names, and none reaches the doc.**

That half is not the strand's alone, and the report must say so: in the same task, a
never-written address — `jigc doc show research:never-written --task …` — prints the
same block and the same route, and its create is refused by the same gate (Left open,
item 2). What the strand adds is the next section.

### In a task whose gate grants the create (cells CN, PN)

`jigc start --workflow do-research "read the research on context loss"` — exit 0, task
`read-the-research-on-context`, create-gate `research`. Then:

1. `jigc doc show research:context-loss --task read-the-research-on-context` — exit 1,
   the same `store.not-staged` block and route as above.
2. `jigc doc create research --title "Context Loss" --task read-the-research-on-context`
   — **exit 0**, stdout `research:context-loss`.
3. `jigc doc show research:context-loss --task read-the-research-on-context` — **exit
   0**, and what it serves is a skeleton: the front matter, `# Context Loss`, and three
   empty sections.
4. `jigc doc show research:context-loss` — exit 1, still `store.not-found`, still *is
   committed at `docs/research/context-loss.md`*.

So here the route runs, and what it hands the reader is a blank second doc under the
identity of the committed one. The committed doc — question, findings, sources — stands
untouched at its prior home and was reached by nothing the route said. The task was not
finalized (Left open, item 3).

### The same doors with no strand (cells CK, PK)

In a fresh rig with the knob untouched and the same `do-research` task: the `--task` read
says *only its committed copy exists* and routes at the task-less read; `jigc doc create
research --title "Context Loss"` exits 0 and acks `research:context-loss (already existed
— copied in for update)`. That is what the door does when it can see the doc: it copies
the committed bytes in. Over the strand it sees none and mints a blank.

## The three attempts to refute it

**1. It does not reproduce — no.** Fresh scratch root, fresh rigs, no pipe on any command
whose status was read, no `head`, the handed binary by hash. The block as triage worded
it gives the claimed exit and the claimed sentence in every cell that holds the strand:
three on the candidate and four on the previous release.

**2. A settled decision intends it — no.** `design/doc-read-surface.md` states the block's
rule: *an absent staged instance blocks `store.not-staged`, routed on the real state: a
committed sibling exists → the route names the task-less read; nothing exists anywhere →
nothing to read yet, create or author it in the task first*. The message driven here is
the *nothing exists anywhere* arm, printed over a doc that exists and that the sibling
read locates by path. The code's notion of the real state is one question —
`not_staged_block` asks whether `canonical_path(…)` is a file at the **resolved** home —
and that question cannot see a strand. The task-less arm had the same blindness until
M53, when `relocated_route` was added at the verb boundary for exactly this state, on the
grounds that *create it / fix the reference / read it with `--task`* are *three exits
none of which is the one `jigc validate` names*. That repair sits in
`reroute_unadopted`, which only the committed arm calls. The design's one sentence about
the staged arm and that function — the foreign-adoption reroute *never fires on the
staged arm* — is about a `store.unparseable` staged copy, not about a strand. No entry
rules that the staged read should deny a stranded doc's committed copy, and the finding
does not argue that any decision is wrong: nothing here is contested.

**3. It breaks no clause inside the clause's scope — no, on the second half.** The
clause's measure, in the exit rule's second sharpening: *no command that works on rc.24
in a supported layout stops working, and every refusal's route works as printed*.

- *First half — unbroken, as a fact.* The previous release gives the same exits and the
  same bytes at every step (next section). Nothing that worked there stopped working.
- *Second half — broken as driven.* This is a refusal, and its route was run as printed.
  In the task it names, both acts are refused at exit 1. In a task that grants them, the
  route completes and delivers a blank doc where the reader asked for a committed one;
  the route that does reach the doc — `jigc validate`, then one of three repairs — is
  printed by the sibling read and not by this one.
- *Inside the scope.* No plant, no plumbing, no unusual git configuration: a plain
  repository, `jigc config set`, and `git reset --hard`. The design treats the resulting
  state as an ordinary one with a name and a detector of its own —
  `design/validation.md` → Orphan detection, *the `docs-root`-changed orphan*,
  `file-state.orphaned-doc` — and the product's own route text names its cause (*a
  `docs-root` / `placement-root` re-point stranded it*). None of the bounds the exit rule
  declares — a race against another writer, a deliberately planted state — describes
  it, and the run's opening declares none.

## The regression fact

**`regression: false`** — the block is red on the previous release exactly as on the
candidate. Fresh rigs, the previous release's binary, hash asserted
(`accf3996…ab5d`):

- Cell PA, every call by the binary's absolute path: the strand's four steps exit 0, 0,
  0, 0; the task-less read exits 1; the `--task` read exits 1; the route's create arm
  exits 1. The three refusals' stderr compared against the candidate's with `cmp`: exit
  0, 0 and 0.
- Cells P, PI, PN, PK against C, CI, CN, CK: every step's stdout and stderr compared
  byte for byte, 43 steps in all. They differ in one file per stranded pair — the
  abbreviated commit hash in `git reset --hard`'s own `HEAD is now at …` line — and
  nowhere else.

The door existed on the previous release and the block ran there whole: comparable, and
the same.

## Class

**instance, unbounded.** One doctype (`research`, a `location:` home), one knob
(`docs-root`), one address, one way of reaching the strand, one host, one git; the plain
and the `json` format of the one door. Not driven: a `placement-root` strand, a doc at a
recorded prior home after a schema bump (the other arm `relocated_route` answers), any
other doctype. Read, not derived: `grep` over `crates/` finds `not_staged_block(` at one
call site, `read_slice_staged` in `crates/engine/src/store.rs`; which other doors ask
*is there a committed copy* of the resolved home alone was not enumerated — the create
door's answer in cell CN suggests it is one of them.

## Repro VR-DST-1

```yaml
claim: "over a doc a docs-root re-point left at its prior home, `jigc doc show <addr> --task <id>` exits 1 store.not-staged saying the doc has no committed copy, and routes at creating it — and that breaks the clause working-product"
verdict: "CONFIRMED, regression: false — the refusal's route does not work as printed; the previous release is red on the same block, byte for byte"
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
strand:                      # two ordinary commands, nothing planted
  - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]    # exit 0, one relocated pair, committed: false
  - ["git", "reset", "--hard"]                                              # exit 0
cells:
  - cell: "the finding — the staged read denies a committed doc (C, P, PA)"
    setup:
      - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <binary>; its open task is ground-the-vision-in-research
      - strand
    repro:
      - ["git", "ls-files", "--", "docs/research", "notes"]
      - ["jigc", "doc", "show", "research:context-loss"]
      - ["jigc", "doc", "show", "research:context-loss", "--task", "ground-the-vision-in-research"]
    expect:
      - exit: 0
        stdout: "docs/research/context-loss.md"
      - exit: 1
        stdout: ""
        stderr_contains: ["store.not-found", "is committed at `docs/research/context-loss.md`, outside the home this read resolves"]
      - exit: 1
        stdout: ""
        stderr_contains: ["store.not-staged", "is not staged in this task and has no committed copy — nothing to read yet", "route: create or author the doc in this task first"]
    red-test: "the third step's stderr must not contain `has no committed copy` while the first step lists the doc"
    on-previous: "identical — all three compared byte for byte"
  - cell: "the route in the task the read named — every arm refused (CI, PI)"
    setup:
      - fixture: refs-post-hoc
      - strand
    repro:
      - ["jigc", "doc", "create", "research", "--title", "Context Loss", "--task", "ground-the-vision-in-research"]
      - ["jigc", "doc", "author", "research", "--from-file", "<payload: title Context Loss, the three research slots>", "--task", "ground-the-vision-in-research"]
      - ["jigc", "doc", "show", "research:context-loss", "--task", "ground-the-vision-in-research"]
    expect:
      - exit: 1
        stderr_contains: ["create.gate-blocked", "allowed doctypes: [vision]"]
      - exit: 1
        stderr_contains: ["create.gate-blocked", "allowed doctypes: [vision]"]
      - exit: 1
        stderr_contains: ["store.not-staged", "has no committed copy"]
    on-previous: "identical"
  - cell: "the route in a task that grants the create — a blank doc of the same identity (CN, PN)"
    setup:
      - fixture: refs-post-hoc
      - strand
      - ["jigc", "start", "--workflow", "do-research", "read the research on context loss"]    # exit 0, task read-the-research-on-context
    repro:
      - ["jigc", "doc", "show", "research:context-loss", "--task", "read-the-research-on-context"]
      - ["jigc", "doc", "create", "research", "--title", "Context Loss", "--task", "read-the-research-on-context"]
      - ["jigc", "doc", "show", "research:context-loss", "--task", "read-the-research-on-context"]
      - ["git", "ls-files", "--", "docs/research", "notes"]
    expect:
      - exit: 1
        stderr_contains: ["store.not-staged", "has no committed copy"]
      - exit: 0
        stdout: "research:context-loss"
      - exit: 0
        stdout_contains: "# Context Loss"
        stdout_not_contains: "Static rules files go stale"       # the committed doc's findings are not what is served
      - exit: 0
        stdout: "docs/research/context-loss.md"
    on-previous: "identical"
  - cell: "the control — no strand, the same doors (CK, PK)"
    setup:
      - fixture: refs-post-hoc
      - ["jigc", "start", "--workflow", "do-research", "read the research on context loss"]
    repro:
      - ["jigc", "doc", "show", "research:context-loss", "--task", "read-the-research-on-context"]
      - ["jigc", "doc", "create", "research", "--title", "Context Loss", "--task", "read-the-research-on-context"]
    expect:
      - exit: 1
        stderr_contains: ["store.not-staged", "only its committed copy exists", "route: `jigc doc show research:context-loss`"]
      - exit: 0
        stdout: "research:context-loss (already existed — copied in for update)"
    on-previous: "identical"
observed: "<scratch>/verify-p3-showtask.yAHnbn — c.* p.* pa.* (the finding), ci.* pi.* (the route in the named task), cn.* pn.* (the route in a granting task), ck.* pk.* (the control); one <cell>.log per cell; the rigs are the jigc-rig-refs-post-hoc-* directories beside them"
pinned-by: "UNPINNED: the first three cells hold a defect as observed — they are the fix's red test, with the first cell's `red-test` line as the assertion that does not depend on the fix's wording; the control cell states a fact that should stay true and was not checked against the suites"
```

**Pinnable as it stands: yes, as a red test.** Setup and argv are plain — one named
fixture, two ordinary commands, no raw bytes, nothing host-specific — and the block ran
unchanged on both binaries. Its `expect` lines are what was observed, so pinned green
they would hold the defect in place: the conversion is the first cell with its `red-test`
assertion, which names the claim and not a repair. The control cell converts as it is.
The finding makes no coverage claim, so none was verified and no suite was read for one.

## Left open

1. **The reading this verdict rests on.** *Works as printed* is read here as: the route,
   followed as it stands, brings the reader to what the refused command asked for. On
   that reading it fails twice over. On the narrowest reading — the route's commands run
   and its own sentence comes true — the granting-task cell passes (a staged copy does
   exist after the write) and only the named-task cell fails, for a cause that is not the
   strand's alone (item 2). No text of record chooses between the two. Ruled the narrow
   way, what is left of this row is a false sentence on a refusal — real, a row of the
   ledger, and `breaks-no-clause`.
2. **`store.not-staged`'s route names an act the task's gate refuses.** *Create or author
   the doc in this task first* is printed for a task whose workflow grants no create for
   the doctype; both acts then exit 1 with `create.gate-blocked`. Driven on a
   never-written address too (`research:never-written`, cells CI and PI): the same
   block, the same route, the same gate. Both binaries. Not the strand's; not pursued.
3. **`jigc doc create` over a strand stages a second doc of a committed identity at exit
   0**, where the same call with no strand acks *already existed — copied in for update*
   (cells CN and CK; both binaries). The task was not finalized here, so what its
   finalize commits was not driven by this verification.
4. **`jigc doc set-slot <addr> --task <id>` over the strand** answers with one bare line —
   no code, no `at:`, no closing line — that says *nothing in this task provisions it*
   over a doc committed at its prior home (cell CI, arm three). Both binaries. Not driven
   without the strand.

## Bounds — what this verification did not do

- One macOS host, one git.
- The strand was reached one way. A `placement-root` strand, a recorded prior home, and a
  strand left by a committed knob with the move reverted were not driven.
- The path-naming route of the task-less read was not followed here; its repair is not
  this finding's subject.
- No task was finalized; nothing was committed in any rig after the strand.
- No coverage was verified and no suite was run.
- Nothing was built, edited, staged or committed in the repository. The clone stands at
  `eeffe347` on `fix/canary-one`, its only untracked path the run's own `r1/` directory,
  as before.

## Where the evidence is

`<scratch>/verify-p3-showtask.yAHnbn`: the rigs, each cell's `<cell>.rig.env` and
`<cell>.rig.err` (the rig's assignments and its construction log), each step's captured
stdout and stderr as `<cell>.<step>.out` and `.err`, the per-cell logs, and under
`tools/` the four small scripts that ran the cells.

<!-- end of report -->
