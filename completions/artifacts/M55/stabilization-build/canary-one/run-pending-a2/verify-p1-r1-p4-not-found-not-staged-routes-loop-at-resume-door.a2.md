# verify-real — `r1-p4-not-found-not-staged-routes-loop-at-resume-door` (run canary-one, round 1, stage test, attempt 2)

One finding, handed over by triage with the grade *unclear*: door `jigc start`, clause `working-product`,
repro block *Repro V-3c* of the report
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-unreadable-entry-undriven-platform-shapes-and-doors.a1.md`
(its *Left open*, item 6). Of that report I used the block, the item and the section that states how its
author read the clause; I read no other finding's report and no other verifier's.

## Verdict

**`confirmed`. `regression: false`. `contested: false`.**

Basis, in one line: over a committed managed doc its owner may not read, `jigc start --task` refuses under
`store.not-found` and routes to the staged read, the staged read refuses under `store.not-staged` and routes
to the task-less read, and the task-less read is the first refusal again — no printed alternative leaves the
pair, in an ordinary layout, with no declared bound that excludes the state; byte-identical on `1.0.0-rc.24`.

What is fact and what is reading, kept apart:

- **Fact, driven on both binaries in rigs of my own:** the three exits, the three refusals, the loop, the
  control without the plant (0 · 1 · 0), the control with the doc truly gone (the pair ends, no loop), and
  that the two binaries answer every cell with the same bytes.
- **Reading, mine, and the one thing a ruling could overturn:** that *every refusal's route works as
  printed* is broken by a route whose printed command exits 1 and whose own sentence is false in the state
  the refusal was printed in. *Step 3* gives the three readings I tried it against and why only the weakest
  one clears it. If the human declares a bound for a managed doc made unreadable on purpose, this row is out
  of scope by that bound — the run's opening declares none, and that ruling is not mine.

## The binaries, asserted before anything was driven

By the one call the prompt spells, each printing one line of JSON:

| binary | path under the scratch root | `content_sha256` printed | the hash handed to me |
|---|---|---|---|
| candidate `c1`, commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` | `bin/c1.a2/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | the same |
| previous release `1.0.0-rc.24` | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | the same |

In every driving script the binary's directory was put first on `PATH` and `command -v jigc` compared with
the binary's path before a rig was built; a mismatch exits 90 and drives nothing (it fired once, on a slip of
mine — *Slips*). Every rig was built with `dev/jigc-rig refs-post-hoc --binary <that path>`. No build was
run, nothing under `target/` was driven.

Platform: macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), uid 501 — **not root**. `HOME` is the rig's own
(`$RIG_HOME`, set by the rig's assignments); stdin of every cell is `/dev/null`. I did not set
`GIT_CONFIG_GLOBAL` or `GIT_CONFIG_NOSYSTEM` as the reporter did; the cells are the same bytes as the
reporter's block expects, so nothing turned on it.

## Step 1 — driven from nothing

`<W>` is `<scratch>/verify-p4.kMBKHQ`, minted with `mktemp -d` under the scratch root. Eight rigs, each
minted for one arm and used for nothing else: plant, control, *missing* and *create*, on each binary. In each
rig `docs/research/context-loss.md` is tracked (`git ls-files --error-unmatch` exit 0), 254 bytes, mode 644,
and `$RIG_TASK` is `ground-the-vision-in-research`, a live task of workflow `form-vision` whose composed text
cites `research:context-loss#findings`.

The plant is one call, `chmod 000 docs/research/context-loss.md`; a plain `cat` of the file then exits 1, so
the plant bites for this user. Each exit status is the child's own, written to a file by the driver
(`"$@" >out 2>err </dev/null; rc=$?`), never read through a pipe.

### The block as written, and the routes it prints, run as printed

| cell | argv | candidate, plant | candidate, no plant | `1.0.0-rc.24`, plant | `1.0.0-rc.24`, no plant |
|---|---|---|---|---|---|
| c1 | `jigc start --task ground-the-vision-in-research` | **1** `store.not-found` | 0, the workflow (7184 bytes) | **1**, same bytes | 0, same bytes |
| c2 | `jigc task list` (the route's *find the task id*) | 0, names the task | 0 | 0 | 0 |
| c3 | `jigc doc show research:context-loss#findings --task ground-the-vision-in-research` (c1's route) | **1** `store.not-staged` | 1, the same bytes | **1** | 1 |
| c4 | `jigc doc show research:context-loss#findings` (c3's route) | **1** `store.not-found`, **byte-identical to c1's stderr** | 0, the slice (65 bytes) | **1** | 0 |
| c5 | `jigc doc show research:context-loss --task ground-the-vision-in-research` (the whole-doc form) | 1 `store.not-staged` | 1, the same bytes | 1 | 1 |
| c6 | `jigc doc show research:context-loss` (c5's route) | **1** `store.not-found` | 0, the doc (255 bytes) | **1** | 0 |

Stdout is empty in every cell that exits 1. `HEAD` did not move in any rig; after the plant arm the file is
still mode 000 and 254 bytes.

The three refusals of the loop, whole, as the candidate printed them on stderr (c1, c3, c4):

```text
blocking · store.not-found — could not read `research:context-loss#findings` at `docs/research/context-loss.md`: Permission denied (os error 13)
  at: research:context-loss#findings
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:context-loss#findings --task <task-id>` (find the task id with `jigc task list`)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

```text
blocking · store.not-staged — `research:context-loss#findings` is not staged in this task — only its committed copy exists
  at: research:context-loss#findings
  route: `jigc doc show research:context-loss#findings` — the task-less read serves the committed copy
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

```text
blocking · store.not-found — could not read `research:context-loss#findings` at `docs/research/context-loss.md`: Permission denied (os error 13)
  at: research:context-loss#findings
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:context-loss#findings --task <task-id>` (find the task id with `jigc task list`)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

### Two arms I added, to bound what the loop needs — each in a rig of its own, both binaries

**The doc truly gone** (`rm -- docs/research/context-loss.md` in place of the `chmod`): the pair does **not**
loop.

| argv | exit | what it says |
|---|---|---|
| `jigc start --task ground-the-vision-in-research` | 1 | `store.not-found — could not read … No such file or directory (os error 2)`, the same route |
| `jigc doc show research:context-loss#findings --task ground-the-vision-in-research` | 1 | `store.not-staged — … is not staged in this task and has no committed copy — nothing to read yet`; route: `create or author the doc in this task first — a staged copy exists only after a write` |
| `jigc doc show research:context-loss#findings` | 1 | `store.not-found`, as the first |

Here the second refusal tells the truth and ends the chain at *create it*. So the loop needs exactly a file
that **is there and cannot be read**: the second refusal's question is whether the home is a file, the first
refusal's is whether it can be read, and the two disagree for that one state.

**The route's first alternative, *create the referenced doc*** (plant, then
`jigc doc create research --title "Context Loss" --task ground-the-vision-in-research`, then the resume
again): exit 1, `create.gate-blocked — the workflow does not allow jigc doc create research in-task; allowed
doctypes: [vision]`; the resume afterwards is the first refusal, the same bytes. The route's second
alternative, *fix the reference to an existing one*, names no command, and the reference already names an
existing doc. So in this task **none of the three alternatives the first refusal prints leaves the refusal**.

All four arms are byte-identical between the two binaries: 30 cells, stdout, stderr and exit each compared
with `cmp`, 90 comparisons, 0 differing.

## Step 2 — is it what the finding says

The finding says: *`store.not-found` and `store.not-staged` route to each other over a doc that exists and
cannot be read, at the resume door `jigc start --task`, for a tracked managed doc.* Observed: exactly that.
What I tried, to show it wrong:

- **Stale state.** Every rig is new, built for one arm, on the binary named; nothing of the reporter's was
  used. It reproduces.
- **A pipe or a cut.** No exit was read through a pipe; every stdout and stderr was read whole from a file.
- **Another binary.** Both hashes asserted first; `command -v jigc` checked inside every driver run; the
  rig's `$JIGC` and the `jigc` on `PATH` are the same file in every arm.
- **A wrong control.** Without the plant the same three commands exit 0 · 1 · 0, and the 1 is the same
  `store.not-staged` whose route then **does** deliver. So the middle refusal is not the defect; what it
  promises is.
- **A settled decision that intends it.** I found none, and the design text goes the other way.
  `design/doc-read-surface.md` → *What it reads*: an absent staged instance blocks `store.not-staged`,
  **"routed on the real state"** — a committed sibling exists → the route names the task-less read. The real
  state here is a committed sibling that cannot be read, and the route is the one for a sibling that can.
  `design/surface-contract.md` → *The three laws*, law 1: every claim a surface makes is generated from the
  thing it describes or asserted against it — *the task-less read serves the committed copy* is asserted
  against the entry's existence only (`crates/engine/src/store.rs:605-606`, `is_file()`; the file is the
  candidate's — `git diff --quiet <candidate> HEAD -- crates/` exits 0). `design/validation.md` → *The rc.24
  fix pass registration — the home that is not a regular file* settles a neighbouring state (a home whose
  entry is a link, a directory or a special file, at the doors that write) and says a read is not that
  code's; a regular file nobody may read is not that state, and no cell drew that code.
- **The finding claiming more than it shows.** It does not: it claims the loop at one door for one doc, and
  that is what was driven.

So: real, as stated, and not intended.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause in the run's closing condition (`completions/artifacts/canary-one/opening.md`, by reference to
`DECISIONS.md` → *2026-10-04 — The exit rule, revised*): *a working product others can use and rely on*; its
instrument, **"no command that works on rc.24 in a supported layout stops working, and every refusal's route
works as printed"**; and the rule, *a finding blocks only if it breaks a clause inside its scope*.

**The first half is not broken.** Every cell is the same on `1.0.0-rc.24`. Nothing that worked there stopped
working.

**The second half is, on every reading but the weakest.** Three readings, each tried:

1. *A route works as printed when its printed command, typed, does what the route says it will.* This is how
   the repository uses the phrase wherever it measures it: `DECISIONS.md` → the audit entry of 2026-10-04,
   *What held* — "every route those refusals print ran as printed to a landed end state"; and the two
   standing fences that entry and M52's name, whose own words are *"the preserving exit must run as printed"*
   and *"the emitted `jigc task discard` must exit 0"*. Under it both routes fail: c3 is the command c1
   printed and exits 1; c4 is the command c3 printed, beside the sentence *the task-less read serves the
   committed copy*, and exits 1.
2. *A route need only work in the state its own refusal diagnoses, not survive a second, independent fault*
   — the reading the report I was handed stated for this pair. It clears c3's route at most, and it does
   **not** clear c1's: the state `store.not-found` diagnoses here, in its own first line, is *Permission
   denied* on that file. There is no second fault. Its route answers a different state — a doc that does not
   exist, or one staged and not yet committed — and of its three alternatives one is refused in this task
   (`create.gate-blocked`, driven), one names a reference that is already right, and one is c3.
3. *A route works as printed when the CLI accepts the command.* Each printed command parses and runs its
   verb. Only this reading clears the pair, and it would clear any route that parses — which is the parse
   fence's property (`design/surface-contract.md` → *The route fence*), not the exit rule's measure.

**The scope.** *In a supported layout*: the rig is a plain repository, one checkout, nothing of the nine
configurations and layouts the regression set's second part enumerates
(`implementation/decisions-pending.md` → *The regression set, part 2*); a file mode is not a layout. *A
planted state*: the first clause's sharpening makes deliberately planted states **declared bounds, written
down with their reach** — for the first clause, and a bound is the human's ruling
(`implementation/stabilization-workflow.md` → *Triage, the human's list and the forks*: a grade of *out of
scope* must cite a declared bound on the run's list). This run's opening says *No bound is declared*. I
excuse nothing as planted, because there is nothing on the list to cite.

**What I did not lean on.** That the refusal is otherwise sound — exit 1, nothing written, `HEAD` unmoved,
and the OS error and the path named in the message, from which a person can read the remedy — is true and is
not the clause's measure. Nor is severity: how much this row weighs is not a verifier's to say.

**Verdict on the clause: `working-product` is broken by its second half, inside its scope as the run declares
it.** Not by its first.

## Step 4 — the regression fact

The same block, on rigs of their own, with the previous release's binary by its absolute path
(`<scratch>/bin/previous-91834b5e011d/jigc`, sha256 `accf3996…ab5d`): c1 **1**, c3 **1**, c4 **1**, the same
bytes as the candidate's in every cell, plant and control. Red there and red on the candidate:
**`regression: false`**. The block is comparable — the door, both codes and both routes exist on
`1.0.0-rc.24`.

## Step 5 — the coverage statement

The finding claims nothing about coverage (its block says `UNPINNED: not searched`). For my own block's
`pinned-by` I derived it from the suites, not from a diff:

- `.rs` files under `crates/` and `tooling-tests/` naming `not-staged` or `NOT_STAGED`: **13** (9 suites,
  4 sources).
- `.rs` files there that set a file mode or name a permission error: **63**.
- In both lists: **4** — `crates/cli/tests/flow49_acceptance.rs` and `flow52_acceptance.rs`, whose mode
  calls are `0o755` on a hook, and two source files. **No suite makes a committed doc unreadable and then
  reads it by either arm.**
- The readable case **is** pinned: `staged_read_miss_arm::the_task_read_miss_names_the_staged_copy_it_looked_in`
  (registered in `crates/cli/tests/groups/g_doc.rs`) asserts the `--task` read blocks and the task-less read
  of the same address succeeds.

The bound of that derivation: it is a text search for two vocabularies, so a suite that reached the state by
other words would not be in it. `UNPINNED`, with that said.

## The repro block

### Repro V-4 — the two routes at the resume door, over a committed doc its owner may not read

```yaml
claim: "over a committed managed doc nobody may read, `jigc start --task` refuses under store.not-found, its route's command refuses under store.not-staged, and that refusal's route is the first refusal again — breaking `working-product` (every refusal's route works as printed); ledger key r1-p4-not-found-not-staged-routes-loop-at-resume-door"
verdict: CONFIRMED          # regression: false — the same bytes on the previous release
binary: "candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — byte-identical in every cell"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), uid 501; Linux NOT driven; root NOT driven"
setup:
  - fixture: refs-post-hoc                  # $RIG_TASK cites research:context-loss#findings
  - ["chmod", "000", "docs/research/context-loss.md"]
repro:
  - ["jigc", "start", "--task", "<RIG_TASK>"]
  - ["jigc", "doc", "show", "research:context-loss#findings", "--task", "<RIG_TASK>"]
  - ["jigc", "doc", "show", "research:context-loss#findings"]
expect:                                     # as observed today — the defect's present shape
  - { exit: 1, stdout: "", stderr_contains: "store.not-found — could not read `research:context-loss#findings` at `docs/research/context-loss.md`: Permission denied (os error 13)", route_contains: "read it with `jigc doc show research:context-loss#findings --task <task-id>`" }
  - { exit: 1, stdout: "", stderr_contains: "store.not-staged — `research:context-loss#findings` is not staged in this task — only its committed copy exists", route_contains: "`jigc doc show research:context-loss#findings` — the task-less read serves the committed copy" }
  - { exit: 1, stdout: "", stderr: "byte-identical to the first cell's" }
after: "HEAD unchanged; docs/research/context-loss.md still mode 000, 254 bytes"
control-no-plant: "0 (the workflow composes) · 1 store.not-staged, the same bytes as above · 0, the slice"
control-doc-removed: "1 store.not-found (os error 2) · 1 store.not-staged `… has no committed copy — nothing to read yet`, route `create or author the doc in this task first` · 1 store.not-found — the pair ends, no loop"
the-route's-other-alternative: "after the first cell, `jigc doc create research --title \"Context Loss\" --task <RIG_TASK>` exits 1, create.gate-blocked (allowed doctypes: [vision])"
driven-as: "one rig per arm per binary, eight rigs; the three cells in the order given, in one rig, with `jigc task list` (exit 0) between the first and the second and the whole-doc pair after the third"
class: "instance, unbounded"
observed: "<W>/runs/{cand,prev}-{plant,control,missing,create}/c<n>.{argv,rc,stdout,stderr}"
pinned-by: "UNPINNED: no suite reads an unreadable committed doc by either arm (derived as in Step 5; a text search, with that bound)"
```

**Pinnable as it stands: yes, as a characterization** — a named fixture state, one `chmod`, three literal
argv, three exits and one line each. What a converter needs to know:

1. **It pins the defect's present shape.** As the fix's red test the expectations turn over, and how they
   turn is the fixer's. One assertion holds whatever the fix says: *the command a refusal's route prints,
   run in the state the refusal was printed in, does not print that refusal's predecessor again* — here, if
   the second cell's route names a command, that command exits 0, or the route does not name it.
2. **Unix only, and not for root.** A mode-000 file refuses nobody who may read any file, a container that
   runs as root included; the test has to skip or fail loudly there, never pass.
3. **The half that makes it a non-regression is a comparison of two binaries**, which no test of one binary
   holds.

## What was driven, and what was not

- **Driven, on both binaries:** the block as written; `jigc task list` as the first route names it; the
  whole-doc forms of the two reads; the same six cells with no plant; the three cells with the doc removed;
  the first route's *create* alternative in the task the rig mints.
- **Not driven:** Linux. Root. Any other doc, doctype or workflow than the rig's. A placement doctype's
  home. The `--format json` arm of any cell. `jigc doc create research` from a task whose workflow grants
  it. Any other way a file can be there and unreadable. The other doors of the handed report's matrix — they
  are another row's.
- **The class: `instance, unbounded`.** I enumerated no consumer. Two source lines are where the two answers
  come from and are a pointer for whoever derives the axis, not a count: `crates/engine/src/store.rs:397-413`
  (every read error of the committed arm is answered `store.not-found` with the one route) and `:605-606`
  (the staged arm's *committed sibling* is `is_file()`).
- **Nothing was fixed, graded or decided.**

## Left open — seen on the way, not pursued

1. **The ledger may hold a second row for this mechanism.** The tree's status showed the file name of
   another verifier's report whose key names *the not-staged route* and *mode 000*. I read nothing of it;
   whether the two rows are one is triage's.
2. **`jigc doc create research` over a home that holds an unreadable file**, from a task whose workflow
   grants the doctype — the first route's *create the referenced doc*, where it is allowed. Not driven; what
   the create and a later finalize do there is unknown to me.
3. **The first refusal's code.** `store.not-found` is printed for a doc that is found and cannot be read;
   the message's own text says which it is, the code and the route do not. It is this finding's mechanism
   seen from the code's side, noted so that it is not lost if the row is closed by a route change alone.
4. **Linux and root** — driven by nobody here; a Linux build of the candidate with its hash would be needed.

## Slips of mine, said plainly

- Before `<W>` was minted I redirected the rig tool's `--help` into a file in the system temp directory. It
  is help text, outside the scratch root, and nothing reads it.
- A first loop over the two extra arms passed an empty binary path (a word-splitting habit this shell does
  not have). The driver's `PATH` check answered `PATH-CHECK-FAILED` and exit 90 four times, before any rig
  was built: nothing was driven by another binary, and the four calls are counted nowhere above. The arms
  were then run by four calls spelled out.
- Two driver scripts and two list files were written through the shell into `<W>/tools/` and `<W>/runs/`;
  the one file my file tool wrote is this report.

## Where the evidence is

`<W>` = `<scratch>/verify-p4.kMBKHQ`. The rigs: `<W>/rigs/jigc-rig-refs-post-hoc-*` (eight). Per arm,
`<W>/runs/<cand|prev>-<plant|control|missing|create>/` holds `c<n>.{argv,rc,stdout,stderr}`,
`head.before`, `head.after`, `porc.after`, and for the first two arms `rig.assignments`, `doc.before.ls`,
`doc.planted.ls`, `doc.after.ls`. The drivers: `<W>/tools/drive.sh`, `<W>/tools/drive2.sh`. The coverage
lists: `<W>/runs/A.txt`, `<W>/runs/B.txt`. Nothing was torn down.

<!-- end of report -->
