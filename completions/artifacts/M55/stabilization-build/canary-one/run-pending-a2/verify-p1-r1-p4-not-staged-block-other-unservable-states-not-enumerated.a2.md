# verify-real — `r1-p4-not-staged-block-other-unservable-states-not-enumerated` (run canary-one, round 1, stage test, attempt 2)

One finding, handed over: door `jigc doc show`, clause `working-product`, triage's grade
*unclear*, its repro the passage *What was driven, and what was not* of
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-not-staged-route-task-less-read-refuses.a1.md`
(that return's left-open item 3). I read that one report, because the prompt handed it
over as the finding's block, and no other finding's or verifier's report; of triage I
read the rows that carry this key and nothing else.

**What the finding says.** The staged read's refusal `store.not-staged` takes its
*a committed copy exists* arm on the bare existence of a file at the doc's home. The
sibling row drove one state behind that arm with no servable doc (a foreign file at a
`location` doctype's home). This row says there are others, names three, and says nobody
enumerated them: a managed doc whose committed file no longer parses · a placement
doctype's foreign file at the repository root · a home holding a file the process
cannot read (that last one owned by another row).

## Verdict

**confirmed** — `regression: false` (red on the previous release as well, the same
exits, messages and routes).

**Basis, one line:** all three states the finding names reach the first arm of
`store.not-staged`, whose mechanical route `jigc doc show <addr>` *"— the task-less read
serves the committed copy"*, run as printed, exits 1 and serves nothing —
`store.unparseable` over a hand-edited managed doc and over a foreign root
`CHANGELOG.md`, `store.not-found` over an unreadable file — on both binaries.

**What it is not.** No state is a dead end and no byte is written, moved or lost:
`git status --porcelain` is empty after both reads in the two committed states. The
second refusal is in each case the designed one for the task-less read. The cost is the
same as in the sibling row: a route that does not do what it prints, and one extra hop.

**One producer, one predicate.** This row and the sibling row
(`r1-p3-not-staged-route-task-less-read-refuses`) are the same refusal, raised by the
same function on the same test, in different states. Whether they are one row or two is
not mine; that they share one cause is a fact a fixer needs.

`contested: false` — the finding argues against no settled decision (the one decision
that touches the predicate is named under Step 2).

## The binaries, asserted before anything was driven

| | path | `content_sha256` printed by `dev/stabilize-step hash` |
|---|---|---|
| candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1) | `<scratch>/bin/c1.a2/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` |
| previous release (1.0.0-rc.24) | `<scratch>/bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` |

Both are the hashes the prompt handed over. With the candidate's directory first on
`PATH`, `command -v jigc` printed `<scratch>/bin/c1.a2/jigc`. Every driven command went
through one helper (`run.sh`) that puts the driven binary's directory first on `PATH` and
refuses to run unless `command -v jigc` is that binary; every rig was built with
`dev/jigc-rig --binary <that path> …`. Nothing was built and nothing under `target/` was
driven. The previous release answers `jigc --version` with `jigc 1.0.0-rc.24`; the
candidate was not asked, because it prints the same string.

## Step 1 — driven from nothing

Six fresh rigs of my own, each with `SCRATCH` under my own directory
`<scratch>/vp1a2-notstaged-states.CtELce`:

| rig | binary | built with | state driven |
|---|---|---|---|
| `candA` | candidate | `refs-post-hoc` | A — a managed doc that no longer parses; and the placement control |
| `candB` | candidate | `fresh --start single-task "tidy the readme"` | B — a foreign `CHANGELOG.md` at the root |
| `candC` | candidate | `refs-post-hoc` | C — an unreadable file at a home |
| `prevA` · `prevB` · `prevC` | previous release | the same three | the same three |

Each exit status was read bare: stdout and stderr went to two files, `$?` was read on the
next line, no pipe anywhere. Commits inside a rig were made by plain `git` under a
throwaway identity passed with `git -c`, never by jigc.

**What I changed from the handed block:** nothing, because the handed passage is a list
of three states and no commands. The commands below are the smallest setup that reaches
each named state through the same door, with the same two reads the sibling block runs.

Each refusal closes with the footer line `— jigc · run "jigc start" for orientation; all
writes through "jigc".` (its two commands in backticks), left out below.

### Controls first — the same refusal, where its route delivers and where it has its other arm (rig `candA`, clean tree)

```
$ jigc doc show research:context-loss --task ground-the-vision-in-research    exit 1
blocking · store.not-staged — `research:context-loss` is not staged in this task — only its committed copy exists
  at: research:context-loss
  route: `jigc doc show research:context-loss` — the task-less read serves the committed copy

$ jigc doc show research:context-loss                                         exit 0
(stdout: the committed doc, 255 bytes — front matter, `# Context Loss`, `## Question`, `## Findings`, `## Sources`; stderr empty)

$ jigc doc show research:nope --task ground-the-vision-in-research            exit 1
blocking · store.not-staged — `research:nope` is not staged in this task and has no committed copy — nothing to read yet
  at: research:nope
  route: create or author the doc in this task first — a staged copy exists only after a write
```

### State A — a managed doc whose committed file no longer parses (rig `candA`)

The edit a person makes by hand: the heading `## Question` of the committed, stamped
`docs/research/context-loss.md` reworded to `## The open question`, added and committed
with git. `git status --porcelain` empty before the reads. The stamp (`schema-version: 1`)
is untouched, so `jigc doc list` (exit 0) still prints the row
`research:context-loss  docs/research/context-loss.md  managed`.

```
$ jigc doc show research:context-loss --task ground-the-vision-in-research    exit 1
(stdout empty; stderr:)
blocking · store.not-staged — `research:context-loss` is not staged in this task — only its committed copy exists
  at: research:context-loss
  route: `jigc doc show research:context-loss` — the task-less read serves the committed copy

$ jigc doc show research:context-loss                                         exit 1
(stdout empty; stderr:)
blocking · store.unparseable — `research:context-loss` at `docs/research/context-loss.md` does not parse: section heading "The open question" does not match required section `question`
  at: research:context-loss
  route: fix the committed file so it conforms to its schema
```

`git status --porcelain` after: empty.

### State B — a placement doctype's foreign file at the repository root (rig `candB`)

Before the plant, as a control on the same rig, the same address takes the other arm:

```
$ jigc doc show changelog:changelog --task tidy-the-readme                    exit 1
blocking · store.not-staged — `changelog:changelog` is not staged in this task and has no committed copy — nothing to read yet
  at: changelog:changelog
  route: create or author the doc in this task first — a staged copy exists only after a write
```

The plant: an ordinary changelog no jigc wrote (`# Changelog`, one sentence, `## 0.2.0`
with one bullet, `## 0.1.0` with one bullet) at `CHANGELOG.md`, added and committed with
git; `git status --porcelain` empty. `jigc doc list` (exit 0) then prints
`changelog:changelog  CHANGELOG.md  unregistered` — the listing hands a reader the
address.

```
$ jigc doc show changelog:changelog --task tidy-the-readme                    exit 1
(stdout empty; stderr:)
blocking · store.not-staged — `changelog:changelog` is not staged in this task — only its committed copy exists
  at: changelog:changelog
  route: `jigc doc show changelog:changelog` — the task-less read serves the committed copy

$ jigc doc show changelog:changelog                                           exit 1
(stdout empty; stderr:)
blocking · store.unparseable — `changelog:changelog` at `CHANGELOG.md` does not parse: section heading "0.2.0" does not match required section `unreleased-changes`
  at: changelog:changelog
  route: adopt — run `jigc ingest` to route it, or `jigc migrate <scratch>/vp1a2-notstaged-states.CtELce/rigs-candB/jigc-rig-fresh-H2X1Yq/repo/CHANGELOG.md --as changelog` to rewrite it into the managed `changelog` shape; it is a foreign file, not an unmigrated managed doc
```

`git status --porcelain` after: empty.

**The placement control** (rig `candA`, where `CHANGELOG.md` is a managed doc the open
task does not stage): `jigc doc show changelog:changelog --task ground-the-vision-in-research`
exits 1 with the same refusal and the same route, and `jigc doc show changelog:changelog`
exits 0 with the doc (317 bytes) on stdout. So on a placement doctype too the route
delivers behind a managed doc and does not behind a foreign file.

**Is B a dead end — from the binary, candidate only:** `jigc ingest` exits 0 and prints
`needs-reconcile CHANGELOG.md → changelog` with the same `jigc migrate … --as changelog`
route; that command, run as printed, exits 0 with
`task minted: migrate-changelog-changelog-b83309faa8b0`. I stopped there. It is not a
dead end.

### State C — a home holding a file the process cannot read (rig `candC`)

Another row owns this state (the sibling report says so, and so does the finding). I
drove it once per binary because the finding names it among the states *not enumerated*,
and I rest no part of the verdict on it. Plant: `docs/research/locked.md` holding `# x`,
mode 000, not added to git; uid 501, not root. The mode was put back to 644 afterwards.

```
$ jigc doc show research:locked --task ground-the-vision-in-research          exit 1
blocking · store.not-staged — `research:locked` is not staged in this task — only its committed copy exists
  at: research:locked
  route: `jigc doc show research:locked` — the task-less read serves the committed copy

$ jigc doc show research:locked                                               exit 1
blocking · store.not-found — could not read `research:locked` at `docs/research/locked.md`: Permission denied (os error 13)
  at: research:locked
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:locked --task <task-id>` (find the task id with `jigc task list`)
```

## Step 2 — is it what the finding says

Yes, for each of the three named states: the refusal's first arm fires, and the read it
routes to refuses.

**Against the design that owns it.** `design/doc-read-surface.md` gives the staged arm's
miss two arms, *routed on the real state*: *a committed sibling exists → the route names
the task-less read (`jigc doc show <addr>`); nothing exists anywhere → nothing to read
yet*. The same document gives the task-less read its own answers: a foreign file at a
home *routes to adoption* (so that *`doc list` says unregistered, `doc show` says adopt
it, `validate` says foreign — adopt it*, with the foreign root `CHANGELOG.md` named as
the archetypal case), and a corrupted managed doc gets *fix the committed file so it
conforms to its schema*. So the **second** refusal is designed in A and in B, and nothing
about it is a defect.

What the design does not give is the **first** refusal's route in these states:

- **B.** No committed copy of `changelog:changelog` exists; a file that is no managed
  doc does. The refusal says *only its committed copy exists* and that the task-less read
  *serves the committed copy*; the binary contradicts both one command later. On the same
  rig, before the plant, the same address took the *nothing to read yet* arm.
- **A.** The honest nuance, stated so nobody reads B's weight into A: here a committed
  copy **does** exist, so the refusal's message is true and the design's first arm is
  literally the one that applies. What is false is the route's own tail — *the task-less
  read serves the committed copy* — and what fails is the route: exit 1, stdout empty.
- **C.** A file is there and cannot be read; neither sentence can be known true, and the
  two refusals route at each other.

**The producer — read, not driven.** `crates/engine/src/store.rs`, `not_staged_block`,
takes the first arm on `canonical_path(…).is_some_and(|p| p.is_file())`: the existence of
a file at the home, with no test of what the file is or whether the task-less read can
serve it. `canonical_path` returns the placement file for a placement doctype, which is
why B reaches it with any slug-bearing address the door accepts.

**The decision that touches the predicate.** `DECISIONS.md` → *2026-07-16 — M43 Inc 5 T1:
the staged-source arm in `store.rs` — three elaboration pins*, pin (1): *the not-staged
block is `store.not-staged`, discriminated on `canonical_path(...).is_file()` — a
committed sibling exists → a `Route::mechanical([…])` task-less read*. I tried to refute
the finding with it as `intended` and could not: the pin names the test and says what the
test stands for — *a committed sibling exists* — and rules nothing about a file that is
no committed sibling or a sibling that cannot be served. `design/surface-contract.md`,
written later, names exactly this gap and classes it: *a route offered on a wider domain
must be gated on that domain, not on the case that motivated it … A mechanical route that
hard-rejects, or repairs the wrong thing, is law 2 failing one level down.* A fix that
narrows the predicate changes that pin's letter and keeps its stated meaning; the finding
does not argue the pin wrong, so there is no fork here.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, from `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, sharpening 2:
*no command that works on rc.24 in a supported layout stops working, and every refusal's
route works as printed.* The first half is untouched: the two binaries answer alike. The
second half is the question triage sent.

**The reading I tried to refute it with — `breaks-no-clause`.** A route *works* if
following it takes the reader to the repair. In A the next refusal names the true cause
and the human repair; in B it names adoption, and adoption was driven to a minted task at
exit 0. Nobody is stranded.

**Why it does not survive, from the route's own text.** The route does not only print a
command; it prints what the command will do — *the task-less read serves the committed
copy*. Run as printed, in each of the three states, the command exits 1 with empty stdout
and serves nothing. No definition of *works* has to be supplied to see that: the route
fails the outcome it states. That the following refusal is a good one is a fact about the
following refusal's route. The repository's own measure agrees, read from its sources and
not from another report: `crates/cli/tests/route_followability.rs` *runs the emitted
backticked argv verbatim … asserting exit 0*, and the surface contract classes *a
mechanical route that hard-rejects* as a route failure, with no exception for one that
fails into a second, better refusal.

**Inside the scope.**

- **B is a supported layout by the design's own word:** a stock brownfield repository
  with a foreign root `CHANGELOG.md` after `jigc setup` is *the archetypal case* of
  `design/doc-read-surface.md` → `jigc doc list` (flow 43). One open task of any kind and
  that file are all it takes; the tree was clean.
- **A is an ordinary state:** a managed doc edited by hand through git is what the
  product says it expects (*out-of-band edits are detected and routed*); one reworded
  heading, committed, clean tree.
- **C** is a planted state and another row's; I do not argue its scope.

So the finding breaks the second half of `working-product` inside its scope in B and in
A, and I could not refute it. **What I am not saying:** how much it weighs. It strands
nobody, it is on the previous release too, and A is the milder of the two. The clause
has no term for weight, and what follows from a confirmed non-regression is not mine.

## Step 4 — the regression fact

The same blocks on fresh rigs built with and driven by the previous release's binary
(`<scratch>/bin/previous-91834b5e011d/jigc`, hash above):

| state | first read (`--task`) | the route, as printed | against the candidate |
|---|---|---|---|
| A (`prevA`) | exit 1 `store.not-staged`, the task-less route | exit 1 `store.unparseable`, *fix the committed file…* | identical text |
| B (`prevB`) | exit 1 `store.not-staged`, the task-less route (and before the plant: exit 1 *nothing to read yet*) | exit 1 `store.unparseable`, the adoption route | identical but for the rig's root inside the printed `jigc migrate` path |
| C (`prevC`) | exit 1 `store.not-staged`, the task-less route | exit 1 `store.not-found`, *Permission denied (os error 13)* | identical text |
| controls (`prevA`) | `research:context-loss` exit 1 → exit 0; `research:nope` exit 1 *nothing to read yet*; `changelog:changelog` exit 1 → exit 0 | - | the served stdout of `changelog:changelog` is byte-identical (`cmp`) |

Red there as on the candidate: **`regression: false`**. The door existed and every block
ran whole — comparable. (On `prevA` the heading edit was made with `perl -pi -e` and three
plain git steps instead of the one `sh -c` line used on `candA`; the resulting file has
the same three headings, read back before the reads.)

## Step 5 — coverage

The finding carries no claim that nothing tests this, so there is none to verify. For my
own blocks I searched, and say how: every file under `crates/` and `tooling-tests/` that
names `store.not-staged`, `not_staged` or `not-staged` — nine suite files
(`doc_list`, `doc_show_staged`, `flow49_acceptance`, `flow52_acceptance`,
`migrate_locus_axis`, `no_such_task_route`, `spec_read_back_arms`, `staged_read_miss_arm`,
`work_unit_unknown_envelope`), three source files of the CLI and the engine's `store.rs`.
I read the two that assert the first arm's route:
`crates/engine/src/store.rs` → `staged_read_of_an_absent_instance_blocks_on_the_real_state`
and `crates/cli/tests/doc_show_staged.rs` →
`committed_json_carries_no_marker_and_not_staged_routes_task_less`. Both build a
committed, conforming doc and assert the route's **text**; neither runs the route. Of the
nine suites, two name `unparseable` at all, and I read both places: one is an unparseable
**staged** copy (`doc_show_staged.rs`), the other a `doc list` row. The narrow claim the
search supports: **no suite drives the route of `store.not-staged` over a home whose file
the task-less read cannot serve.** I did not read the other seven suites end to end.

## The repro blocks

### Repro P4-NS-B — the route of `store.not-staged` over a foreign file at a placement doctype's home

```yaml
claim: "with a foreign CHANGELOG.md at the repository root and one open task, the route `store.not-staged` prints for changelog:changelog — the task-less read, said to serve the committed copy — refuses when run as printed"
verdict: CONFIRMED          # regression: false — the same on the previous release
setup:
  - fixture: fresh
  - ["jigc", "start", "--workflow", "single-task", "tidy the readme"]      # mints tidy-the-readme
  - ["sh", "-c", "printf '# Changelog\\n\\nAll notable changes to this project.\\n\\n## 0.2.0\\n\\n- faster startup\\n\\n## 0.1.0\\n\\n- first release\\n' > CHANGELOG.md"]
  - ["git", "add", "CHANGELOG.md"]
  - ["git", "commit", "-q", "-m", "docs: the changelog this project always had"]
repro:
  - ["jigc", "doc", "show", "changelog:changelog", "--task", "tidy-the-readme"]
  - ["jigc", "doc", "show", "changelog:changelog"]                          # the first step's route, as printed
expect:                              # as observed today, on both binaries
  - exit: 1
    stdout: ""
    stderr_contains:
      - "store.not-staged"
      - "only its committed copy exists"
      - "route: `jigc doc show changelog:changelog` — the task-less read serves the committed copy"
  - exit: 1
    stdout: ""
    stderr_contains:
      - "store.unparseable"
      - "does not match required section `unreleased-changes`"
      - "it is a foreign file, not an unmigrated managed doc"
control:
  - before the plant, same rig: ["jigc", "doc", "show", "changelog:changelog", "--task", "tidy-the-readme"]   # exit 1, "has no committed copy — nothing to read yet"
  - fixture refs-post-hoc, a managed CHANGELOG.md the task does not stage: the same two reads   # exit 1 the same refusal and route, then exit 0 with the doc on stdout
test-in-waiting: "provoke the refusal over the plant, read the backticked argv out of its route, run those bytes verbatim, assert exit 0 — the mold of crates/cli/tests/route_followability.rs. Red today on both binaries. It asserts nothing about which route the fix prints."
also-on-previous-release: "yes — the same exits, messages and routes"
pinned-by: "UNPINNED: no suite drives the route of store.not-staged over a home the task-less read cannot serve (searched by name as Step 5 says)"
```

### Repro P4-NS-A — the same route over a managed doc that no longer parses

```yaml
claim: "with a committed managed doc whose required heading was reworded by hand, the route `store.not-staged` prints — the task-less read, said to serve the committed copy — refuses when run as printed"
verdict: CONFIRMED          # regression: false — the same on the previous release
setup:
  - fixture: refs-post-hoc          # live task: ground-the-vision-in-research; committed: research:context-loss
  - ["perl", "-pi", "-e", "s/^## Question$/## The open question/", "docs/research/context-loss.md"]
  - ["git", "add", "docs/research/context-loss.md"]
  - ["git", "commit", "-q", "-m", "docs: reword a heading by hand"]
repro:
  - ["jigc", "doc", "show", "research:context-loss", "--task", "ground-the-vision-in-research"]
  - ["jigc", "doc", "show", "research:context-loss"]                        # the first step's route, as printed
expect:                              # as observed today, on both binaries
  - exit: 1
    stdout: ""
    stderr_contains:
      - "store.not-staged"
      - "route: `jigc doc show research:context-loss` — the task-less read serves the committed copy"
  - exit: 1
    stdout: ""
    stderr_contains:
      - "store.unparseable"
      - "section heading \"The open question\" does not match required section `question`"
      - "route: fix the committed file so it conforms to its schema"
control:
  - before the edit, same rig, the same two reads        # exit 1 the same refusal and route, then exit 0 with the doc on stdout
  - ["jigc", "doc", "show", "research:nope", "--task", "ground-the-vision-in-research"]   # exit 1, "has no committed copy — nothing to read yet"
test-in-waiting: "as for P4-NS-B: run the route's emitted argv verbatim and assert exit 0. Red today on both binaries."
also-on-previous-release: "yes — the same exits, messages and routes"
pinned-by: "UNPINNED: as for P4-NS-B"
```

**Pinnable as they stand: yes, both.** Each fixture is a named state of
`crates/cli/tests/support/trial_corpus.rs` plus argv steps; every assertion is an exit
status or a substring that carries no host path (the one printed host path, the
`jigc migrate` operand of B's second refusal, is outside every assertion). Two things a
converter must supply, neither of them a change to the block: a commit identity for the
two plain `git commit` steps, and the task id `tidy-the-readme`, which is what the mint
derived from that intent on both binaries. State C is left without a block here: it is
another row's, and a mode-000 plant does not hold under a root-run suite.

## What was driven, and what was not

**Driven:** the three states the finding names, each on both binaries, each on a fresh
rig; two doctypes (`research`, a multi-instance `location` doctype; `changelog`, a
placement singleton); two fixtures (`refs-post-hoc`, `fresh` with one minted task); the
controls on both binaries; the adoption chain of B to its minted task on the candidate
alone.

**The class: `instance, unbounded`.** I did not enumerate the states that satisfy the
predicate without a servable doc behind it, and I derived no count. The predicate was
read in one function; that reading suggests further states I did **not** drive and assert
nothing about — an entry at a home that is a link to a file elsewhere, a file that is not
valid text, a stamped doc at an older schema version, a `#fragment` address over any of
the above. Nor did I enumerate which other refusals hand over a read that can refuse. The
axis is the fixer's to derive.

**Not driven:** `--format json` on either read; a `#fragment` address; a second open
task; a linked worktree; any doctype but `research` and `changelog`; the adoption chain
on the previous release.

## Left open — seen on the way, not pursued

Nothing new. The one thing I saw that is not this finding's — in state C the two
refusals route at each other (`store.not-staged` at the task-less read, `store.not-found`
back at `jigc doc show research:locked --task <task-id>`) — is the unreadable-entry
state the sibling report says has a row of its own, so I file nothing for it.

## Where the evidence is

Under `<scratch>/vp1a2-notstaged-states.CtELce`: `mkrig.sh` and `run.sh` (the two
helpers); `<label>.env` (each rig's assignments) and `<label>.rig.log` for the six labels
`candA`, `candB`, `candC`, `prevA`, `prevB`, `prevC`; `<label>.<n>.out` and
`<label>.<n>.err`, one pair per driven command; `foreign-changelog.md` (B's plant,
`cmp`-equal to the file planted on `candB`); `fresh-start.print.sh` (the rig's own
construction of B's fixture, from `--print-only`). The rigs are under `rigs-<label>`
there. The repository was read and not written: branch `fix/canary-one` at `126a8531`,
its status the three untracked files under
`completions/artifacts/canary-one/r1/reports/test/` it had when I started
(`attempt.a2.md`, `preflight.a2.md`, `triage-p1.a2.md`).

<!-- end of report -->
