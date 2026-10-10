# verify-real — `r1-p4-not-staged-route-mode-000-repro-b-not-redriven` (run canary-one, round 1, stage test, attempt 2)

One finding, handed over: door `jigc doc show`, clause `working-product`, triage's grade
*unclear*, its repro *Left open, item 2* of the report
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-not-staged-route-task-less-read-refuses.a1.md`.
That report is the one I was handed and the only report I read. The block its item 2
points back to sits in another verifier's report, which I did not open. Of triage I read
the one ledger row that carries this key, for its grade.

## Verdict

**confirmed** — `regression: false` (red on the previous release as well, the two
refusals byte for byte the same).

**Basis, one line:** over a file at a doctype's home that the process cannot read
(mode 000), `jigc doc show <addr> --task <id>` refuses `store.not-staged` with the
mechanical route `jigc doc show <addr>` *"— the task-less read serves the committed
copy"*, and that command, run as printed, exits 1 `store.not-found` (*Permission
denied*) — a refusal's route that does not work as printed; the one command the second
refusal prints is the first read again.

**What it is not, so that nobody reads more into it than was driven.** No byte is
written, moved or lost by either read: `git status --porcelain` is the same before and
after, and the file keeps its mode and size. The second refusal's message names the
real cause in the operating system's own words. And the state needs a file its own
reader cannot open — I say under Step 3 why I could not use that to refute, and whose
decision it is.

`contested: false`. The finding does not argue that a settled decision is wrong, and I
found none that intends this route in this state.

## The binaries, asserted before anything was driven

| | path | `content_sha256` printed by `dev/stabilize-step hash` |
|---|---|---|
| candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1) | `<scratch>/bin/c1.a2/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` |
| previous release (1.0.0-rc.24) | `<scratch>/bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` |

Both are the hashes the prompt handed over. With the candidate's directory first on
`PATH`, `command -v jigc` printed `<scratch>/bin/c1.a2/jigc`. Every driven command went
through one helper that puts the driven binary's directory first on `PATH` and refuses
to run unless `command -v jigc` is that binary; every rig was built with
`dev/jigc-rig --binary <that path> refs-post-hoc`. Nothing was built, and nothing under
`target/` was driven. The previous release answers `jigc --version` with
`jigc 1.0.0-rc.24`; the candidate was not asked, because it prints the same string.

## Step 1 — driven from nothing

**What I was given, and what I reconstructed.** Item 2 gives no block of its own. It
gives one sentence: *the same two reads over a mode-000 file, where the second refusal
is `store.not-found` and routes back at the `--task` read*. *The same two reads* are the
two of the handed report's own block (`Repro P3-NS`), so the setup I drove is that
block's, with one step added: the plant is made unreadable. That is the smallest setup
that reaches the door the item names, and it is the only change.

Three fresh rigs of my own, each from `dev/jigc-rig --binary … refs-post-hoc` with
`SCRATCH` under my own directory `<scratch>/vp4-mode000.o9pfhg`: `cand` and `cand2` on
the candidate, `prev` on the previous release. The fixture's live task is
`ground-the-vision-in-research`; its committed store holds `research:context-loss`, and
the task stages `vision:vision` only. Each exit status was read bare: stdout and stderr
went to two files, `$?` was read on the next line, no pipe anywhere. I ran as an
ordinary user (uid 501), which matters: for root a mode-000 file is readable.

### The instance, on the candidate (rig `cand`, clean tree before the plant)

```
$ git status --porcelain                                                     exit 0  (empty)
$ printf '# x\n' > docs/research/plain.md                                    exit 0
$ chmod 000 docs/research/plain.md                                           exit 0
$ cat docs/research/plain.md                                                 exit 1
cat: docs/research/plain.md: Permission denied

$ jigc doc show research:plain --task ground-the-vision-in-research           exit 1
(stdout empty; stderr:)
blocking · store.not-staged — `research:plain` is not staged in this task — only its committed copy exists
  at: research:plain
  route: `jigc doc show research:plain` — the task-less read serves the committed copy

$ jigc doc show research:plain                                                exit 1
(stdout empty; stderr:)
blocking · store.not-found — could not read `research:plain` at `docs/research/plain.md`: Permission denied (os error 13)
  at: research:plain
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:plain --task <task-id>` (find the task id with `jigc task list`)

$ jigc task list                                                              exit 0
jigc task list — 1 active task(s)
  ground-the-vision-in-research  [form-vision]  ground the vision in research

$ jigc doc show research:plain --task ground-the-vision-in-research           exit 1
(the first refusal again, the same 295 bytes)

$ git status --porcelain                                                     exit 0
?? docs/research/plain.md
```

Each refusal closes with the footer line `— jigc · run "jigc start" for orientation; all
writes through "jigc".` (its two commands in backticks), which I leave out above. The
plant is `----------`, 4 bytes, before and after.

So: read A refuses and prints read B; read B refuses and the one command it prints is
read A, with the task id the listing it names supplies. Neither refusal prints a
command that leaves the pair.

### The controls, same rig

```
$ jigc doc show research:context-loss --task ground-the-vision-in-research    exit 1
blocking · store.not-staged — `research:context-loss` is not staged in this task — only its committed copy exists
  at: research:context-loss
  route: `jigc doc show research:context-loss` — the task-less read serves the committed copy

$ jigc doc show research:context-loss                                         exit 0
(stdout: the committed doc, whole, 255 bytes; stderr empty)

$ jigc doc show research:nope --task ground-the-vision-in-research            exit 1
blocking · store.not-staged — `research:nope` is not staged in this task and has no committed copy — nothing to read yet
  at: research:nope
  route: create or author the doc in this task first — a staged copy exists only after a write

$ jigc doc show research:nope                                                 exit 1
blocking · store.not-found — could not read `research:nope` at `docs/research/nope.md`: No such file or directory (os error 2)
  at: research:nope
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:nope --task <task-id>` (find the task id with `jigc task list`)
```

The same refusal with the same route delivers for a committed, readable doc. For an
address with nothing behind it, the first refusal takes its other arm and prints no
task-less read, so the pair does not close on itself there: the closed pair is the
unreadable file's alone among the three.

### A variant: a committed, managed doc made unreadable (rig `cand2`, candidate)

To test whether the finding lives only on a foreign plant, I took the fixture's own
committed doc — tracked by git, adopted, served at exit 0 a moment before — and removed
its read permission, nothing else:

```
$ git status --porcelain                                                     exit 0  (empty)
$ chmod 000 docs/research/context-loss.md                                    exit 0

$ jigc doc show research:context-loss --task ground-the-vision-in-research    exit 1
blocking · store.not-staged — `research:context-loss` is not staged in this task — only its committed copy exists
  at: research:context-loss
  route: `jigc doc show research:context-loss` — the task-less read serves the committed copy

$ jigc doc show research:context-loss                                         exit 1
blocking · store.not-found — could not read `research:context-loss` at `docs/research/context-loss.md`: Permission denied (os error 13)
  at: research:context-loss
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:context-loss --task <task-id>` (find the task id with `jigc task list`)

$ chmod 644 docs/research/context-loss.md                                    exit 0
$ jigc doc show research:context-loss                                         exit 0   (the doc, whole)
$ git status --porcelain                                                     exit 0  (empty)
```

Here the first refusal's message is true — a committed copy does exist — and its route
still does not deliver. The refusal is undone by restoring the mode, which no line of
either refusal says.

## Step 2 — is it what the finding says

Yes, on every point of the sentence I was handed: the same two reads; a mode-000 file;
the second refusal is `store.not-found`; and its route names the `--task` read. Observed
at exit 1 and exit 1, on both binaries.

**Against the design that owns it.** `design/doc-read-surface.md` gives the staged arm's
miss two arms, *routed on the real state*: *a committed sibling exists → the route names
the task-less read (`jigc doc show <addr>`); nothing exists anywhere → nothing to read
yet*. Of a file that exists and cannot be read the document says nothing: the word
*unreadable* does not occur in it, nor does any other name for the state. So no line of
the design intends what was observed, and none forbids it by name.

The producer shows why the first arm is taken — read, not driven:
`crates/engine/src/store.rs`, `not_staged_block`, decides *a committed sibling exists* by
`canonical_path(…).is_some_and(|p| p.is_file())`, the existence of a file at the home,
with no test that it can be served. Its route is `Route::mechanical`. The second
refusal's producer, in the same file, maps **every** error of the read to
`store.not-found` with one fixed sentence for a route, built from a `String` — a route
for a human, in the surface contract's kinds.

`design/surface-contract.md` names the first shape and classes it: *a route offered on a
wider domain must be gated on that domain, not on the case that motivated it … A
mechanical route that hard-rejects, or repairs the wrong thing, is law 2 failing one
level down.* The route here is offered on *a file exists*, was motivated by *a
committed doc can be served*, and hard-rejects.

**The same producer as the finding the handed report verified.** That report's finding
(`r1-p3-not-staged-route-task-less-read-refuses`) reaches the same function through the
same predicate over a readable file that is no managed doc. What differs here is the
state and the second hop: there the second refusal routes on to adoption and the chain
lands; here it routes back. I state this as a fact about the code, for whoever fixes
either; whether the two are one row is not mine to say.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, from `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, sharpening 2:
*no command that works on rc.24 in a supported layout stops working, and every refusal's
route works as printed.* Its first half is not touched: nothing that worked on the
previous release stopped working, the two binaries answer alike. The second half is the
question triage sent. I tried three ways to refute it.

**1. It does not reproduce.** It does, on three rigs and two binaries.

**2. It is intended.** I looked for a decision that intends a `store.not-staged` route
ending in a refusal, or `store.not-found`'s route for a file that exists, and found
none: the design's two arms (Step 2) are its whole text for the first refusal, and for
the second it says only that its route *names the staged read*, written for a doc that
is staged and not yet committed.

**3. It is real and breaks no clause — because the state is outside the scope.** This is
the strongest of the three, and I could not make it stand. A file that its own reader
cannot open is not an ordinary state: git cannot add such a file either, the same entry
of `DECISIONS.md` names *deliberately planted states* as declared bounds under its
first clause, and my plant is a `chmod`. Against that:

- That sentence scopes the **first** clause. The second clause's words carry one scope
  term, *in a supported layout*, on its first half; *every refusal's route* carries
  none. A refusal is by nature what the product says in a state that is not the
  ordinary one, so a reading that takes every such state out would leave the half
  nothing to hold.
- **A bound is the human's, and this run has none.** The workflow's rule is that *a
  grade of out of scope must cite a declared bound on the run's list; one that cannot
  goes to the human* (`implementation/stabilization-workflow.md` → *Every finding is
  graded*), and the opening record says *No bound is declared.* To refute on *this state
  is planted* I would have to declare the bound myself.
- Measured by the repository's own meaning of a route that works —
  `crates/cli/tests/route_followability.rs` runs the emitted backticked argv verbatim
  and asserts exit 0 — the first refusal's route does not work here.

So the verdict rests on one thing: **the first refusal prints one command and one
outcome for it, and run as printed the command refuses and serves nothing.** It does not
rest on the second refusal's route. That one is a sentence for a human, which the
clause's instrument does not run (`implementation/decisions-pending.md`: *a refusal's
route that is a sentence for a human can be run by neither part and stays with the
review rows*), and its staged read is offered under a condition — *a doc staged in an
open task* — that does not hold here. That it sends the reader back is what was
observed; I grade nothing on it.

**What I am not saying:** how much this should weigh, or that the state deserves no
bound. It may well be ruled one — *a file the reader cannot open* is a clean reach to
write down — and that ruling, with the regression fact below, is the human's. What I
establish is narrower: the finding is real, the clause's words are broken by it, and no
declared bound takes the state out.

## Step 4 — the regression fact

The same block on a fresh rig built with and driven by the previous release's binary
(`<scratch>/bin/previous-91834b5e011d/jigc`, hash above): `store.not-staged` at exit 1,
then `store.not-found` at exit 1, then the first refusal again; the controls at
exit 1 → exit 0 and at exit 1 *nothing to read yet*. `cmp` of the two refusals' stderr
between the binaries exits 0 for each — identical bytes. **`regression: false`.** The
door existed there and the block ran whole: comparable.

## Step 5 — coverage

The handed item makes no coverage claim, so there is none to verify. For my own block I
searched, and say how:

- every file under `crates/cli/tests`, `crates/engine` and `tooling-tests` that names
  `store.not-staged`: ten (nine suite files and `crates/engine/src/store.rs`). Two of
  them change a file mode at all (`flow49_acceptance`, `flow52_acceptance`), and in both
  it is `0o755` on a git hook;
- every file there that changes a mode: fifty-six. Seven of them also name `doc show` in
  argv form or `store.not-found`; I read each one's mode lines. Hooks and shims at
  `0o755` in five; in `flow53_acceptance` a frozen manifest; in `repo_relative_paths` a
  task's pin at `0o000`. None makes a doc instance unreadable.

The claim that supports: **no suite drives either read over a doc instance that cannot
be read.** I read the matching lines, not the suites end to end.

## The repro block

### Repro P4-NS-000 — the route of `store.not-staged` over a file that cannot be read

```yaml
claim: "over a file at a doctype's home that the process cannot read, the route `store.not-staged` prints — the task-less read, said to serve the committed copy — refuses when run as printed, and the one command that refusal prints is the first read again"
verdict: CONFIRMED          # regression: false — the same bytes on the previous release
setup:
  - fixture: refs-post-hoc          # live task: ground-the-vision-in-research; committed: research:context-loss
  - ["sh", "-c", "printf '# x\\n' > docs/research/plain.md"]        # not added to git
  - ["chmod", "000", "docs/research/plain.md"]
  - ["sh", "-c", "cat docs/research/plain.md"]                      # precondition: exit 1 — as root the plant is readable and the cell is void
repro:
  - ["jigc", "doc", "show", "research:plain", "--task", "ground-the-vision-in-research"]
  - ["jigc", "doc", "show", "research:plain"]                       # the first step's route, as printed
expect:                              # as observed today, on both binaries
  - exit: 1
    stdout: ""
    stderr_contains:
      - "store.not-staged"
      - "only its committed copy exists"
      - "route: `jigc doc show research:plain` — the task-less read serves the committed copy"
  - exit: 1
    stdout: ""
    stderr_contains:
      - "store.not-found"
      - "could not read `research:plain` at `docs/research/plain.md`: Permission denied"
      - "read it with `jigc doc show research:plain --task <task-id>`"
control:
  - ["jigc", "doc", "show", "research:context-loss", "--task", "ground-the-vision-in-research"]   # exit 1, the same refusal and route
  - ["jigc", "doc", "show", "research:context-loss"]                                              # exit 0, the committed doc on stdout
  - ["jigc", "doc", "show", "research:nope", "--task", "ground-the-vision-in-research"]           # exit 1, "has no committed copy — nothing to read yet"
variant: "chmod 000 on the fixture's own committed docs/research/context-loss.md, nothing planted: the same two refusals at exit 1 and exit 1; chmod 644 and the task-less read is exit 0 again"
test-in-waiting: "provoke the refusal over the unreadable plant, read the backticked argv out of its route, run those bytes verbatim, assert exit 0 — the mold of crates/cli/tests/route_followability.rs. Red today on both binaries. It asserts nothing about which route the fix prints."
also-on-previous-release: "yes — byte-identical stderr for both refusals"
pinned-by: "UNPINNED: no suite makes a doc instance unreadable (ten files naming store.not-staged and fifty-six that change a mode searched by name and by their mode lines, not read whole)"
```

**Pinnable as it stands: yes, with one condition the block carries.** The fixture is a
named state of `crates/cli/tests/support/trial_corpus.rs`, the plant is two argv steps,
and every assertion is an exit status or a substring with no host path in it. The
condition is the precondition step: under root a mode-000 file is read, so the cell must
assert that the plant cannot be read, or declare the platform bound the way
`flow53_acceptance` and `flow54_acceptance` already do for their `chmod` cells.

## What was driven, and what was not

**Driven:** one doctype, `research` (a multi-instance doctype homed under
`docs/research/`), one fixture, one open task, one platform (macOS, an ordinary user);
two shapes of the unreadable file — an untracked one-line plant, on both binaries, and
the fixture's own committed doc with its read permission removed, on the candidate
alone.

**The class: `instance, unbounded`.** I read two producers in one file and one
predicate. That is a reading, not an enumeration: I did not derive which other states
reach the first arm of `not_staged_block` without a doc that can be served behind it,
which other errors of the read `store.not-found` covers with the same sentence, or which
other refusals hand over a read that can refuse. The axis is the fixer's to derive.

**Not driven:** `--format json` on either read; a `#fragment` address; a second open
task; a linked worktree; a placement doctype; a file owned by another user rather than
one at mode 000; an unreadable **directory** at the home; any door but `jigc doc show`
(what the listing, the sweep or the adoption verbs say of the same file is other rows'
business, and I did not look); and the second refusal's first direction — *create the
referenced doc* — which I did not follow, so I say nothing about what a create at this
address would do with the file that is already there.

## Left open — seen on the way, not pursued

1. **`store.not-found` is the code for a file that was found.** The task-less read of a
   file that exists and cannot be read answers with the code `store.not-found` and the
   route *create the referenced doc, or fix the reference to an existing one*; the
   message beside it says *Permission denied*. A driver that keys on the code reads
   *absent*, and neither direction of the route is the repair for the error the message
   names. Seen on both binaries, in both shapes; read against no design beyond the one
   line of `design/doc-read-surface.md` quoted in Step 3; not graded.

## Where the evidence is

Under `<scratch>/vp4-mode000.o9pfhg`: `run.sh` (the one helper, written from the shell);
`cand.env`, `cand2.env`, `prev.env` (each rig's assignments) and the matching
`*.rig.log`; `<label>.<n>.out` and `<label>.<n>.err`, one pair per driven command in the
order driven (`cand.7` and `cand.8` are the two reads; `prev.6` and `prev.7` their
twins; `cand2.3` and `cand2.4` the variant). The rigs are under `rigs-cand`,
`rigs-cand2` and `rigs-prev` there. The repository was read and not written: branch
`fix/canary-one` at `126a8531`, the round's record commit on top of the candidate
`eeffe347`, its status the three untracked files under
`completions/artifacts/canary-one/r1/reports/test/` it had when I started.

<!-- end of report -->
