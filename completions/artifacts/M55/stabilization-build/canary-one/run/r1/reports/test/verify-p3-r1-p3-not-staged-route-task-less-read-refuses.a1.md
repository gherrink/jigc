# verify-real — `r1-p3-not-staged-route-task-less-read-refuses` (run canary-one, round 1, stage test, attempt 1)

One finding, handed over: door `jigc doc show`, clause `working-product`, triage's grade
*unclear*, its repro the block `Repro V-2b` (setup-b, repro-b, control-b) of the report
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-doc-list-unreadable-entry-undriven-shapes-and-consumers.a1.md`.
Of that report I read the one block, lines 433-453, and nothing else. I read no other
verifier's report, and of triage only the two rows that carry this key.

## Verdict

**confirmed** — `regression: false` (red on the previous release as well, the same
refusals word for word).

**Basis, one line:** over a readable file that is no managed doc at a doctype's home,
`jigc doc show <addr> --task <id>` refuses `store.not-staged` with the mechanical route
`jigc doc show <addr>` *"— the task-less read serves the committed copy"*, and that
command, run as printed, exits 1 `store.unparseable` — a refusal's route that does not
work as printed, in an ordinary brownfield state, on both binaries.

**What it is not, said so that nobody reads more into it than was driven:** it is not a
dead end. The second refusal is the designed one, it prints the adoption route, and that
route was followed to a minted adoption task at exit 0 (below, *The chain, followed*).
The cost to the reader is one false sentence, one false route and one extra hop. No
byte is written, moved or lost at any step of the two reads (`git status --porcelain`
before and after shows only the planted file).

`contested: false`. The finding does not argue that a settled decision is wrong, and I
found none that intends this route in this state.

## The binaries, asserted before anything was driven

| | path | `content_sha256` printed by `dev/stabilize-step hash` |
|---|---|---|
| candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1) | `<scratch>/bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` |
| previous release (1.0.0-rc.24) | `<scratch>/bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` |

Both are the hashes the prompt handed over. With the candidate's directory first on
`PATH`, `command -v jigc` printed `<scratch>/bin/c1.a1/jigc`. Every driven command went
through one helper that puts the driven binary's directory first on `PATH` and refuses
to run unless `command -v jigc` is that binary; every rig was built with
`dev/jigc-rig --binary <that path> refs-post-hoc`. Nothing was built, and nothing under
`target/` was driven. The previous release answers `jigc --version` with
`jigc 1.0.0-rc.24`; the candidate was not asked, because it prints the same string.

## Step 1 — driven from nothing

Four fresh rigs of my own, each from `dev/jigc-rig --binary … refs-post-hoc` with
`SCRATCH` under my own directory `<scratch>/vp3-notstaged.kIBpqL`: `cand`, `cand2`,
`cand3` on the candidate, `prev` on the previous release. The fixture's live task is
`ground-the-vision-in-research`; its committed store holds `research:context-loss`, and
the task stages `vision:vision` only. Each exit status was read bare: stdout and stderr
went to two files, `$?` was read on the next line, no pipe anywhere.

The block ran as written. One thing the block leaves implicit and I made explicit: the
plant of control-b is `printf '# x\n' > docs/research/plain.md`, mode 644, **not** added
to git — that is what the block's words *a readable non-doc (`# x`, mode 644,
research:plain)* give, and it is what I planted.

### The instance, on the candidate (rig `cand2`, clean tree before the plant)

```
$ printf '# x\n' > docs/research/plain.md                                    exit 0

$ jigc doc show research:plain --task ground-the-vision-in-research           exit 1
(stdout empty; stderr:)
blocking · store.not-staged — `research:plain` is not staged in this task — only its committed copy exists
  at: research:plain
  route: `jigc doc show research:plain` — the task-less read serves the committed copy

$ jigc doc show research:plain                                                exit 1
(stdout empty; stderr:)
blocking · store.unparseable — `research:plain` at `docs/research/plain.md` does not parse: required section heading `## question` is missing
  at: research:plain
  route: adopt — run `jigc ingest` to route it, or `jigc migrate <scratch>/vp3-notstaged.kIBpqL/rigs-cand2/jigc-rig-refs-post-hoc-ITMscM/repo/docs/research/plain.md --as research` to rewrite it into the managed `research` shape; it is a foreign file, not an unmigrated managed doc
```

Each refusal closes with the footer line `— jigc · run "jigc start" for orientation; all
writes through "jigc".` (its two commands in backticks), which I leave out above.

### The two controls, same rig

```
$ jigc doc show research:context-loss --task ground-the-vision-in-research    exit 1
blocking · store.not-staged — `research:context-loss` is not staged in this task — only its committed copy exists
  at: research:context-loss
  route: `jigc doc show research:context-loss` — the task-less read serves the committed copy

$ jigc doc show research:context-loss                                         exit 0
(stdout: the committed doc, whole — front matter `date`, `schema-version: 1`, `# Context Loss`,
 `## Question`, `## Findings`, `## Sources`; stderr empty)

$ jigc doc show research:nope --task ground-the-vision-in-research            exit 1
blocking · store.not-staged — `research:nope` is not staged in this task and has no committed copy — nothing to read yet
  at: research:nope
  route: create or author the doc in this task first — a staged copy exists only after a write
```

So the same refusal with the same route delivers for a committed managed doc and does
not deliver for the foreign file; and the refusal has a second arm, for *nothing there*,
that the foreign file does not take.

### The previous release (rig `prev`) — the same block

`store.not-staged` at exit 1 with the same message and the same route;
`jigc doc show research:plain` at exit 1 `store.unparseable` with the same message and
the same route; the two controls at exit 1 → exit 0 and at exit 1 *nothing to read yet*.
Compared line for line with the candidate's output above, the only difference is the
rig's own root inside the printed `jigc migrate` path.

### The same thing with a file a project would really have (rig `cand3`, candidate)

To test whether the finding lives only on a one-line plant: `docs/research/latency-notes.md`,
an ordinary note (`# Latency notes`, a paragraph, `## Numbers`, two bullets), added and
**committed with git**; `git status --porcelain` empty. `jigc doc list` (exit 0) prints the
row `research:latency-notes  docs/research/latency-notes.md  unregistered` — the listing
hands a reader the address. Then:

```
$ jigc doc show research:latency-notes --task ground-the-vision-in-research   exit 1
blocking · store.not-staged — `research:latency-notes` is not staged in this task — only its committed copy exists
  at: research:latency-notes
  route: `jigc doc show research:latency-notes` — the task-less read serves the committed copy

$ jigc doc show research:latency-notes                                        exit 1
blocking · store.unparseable — `research:latency-notes` at `docs/research/latency-notes.md` does not parse: section heading "Numbers" does not match required section `question`
  at: research:latency-notes
  route: adopt — run `jigc ingest` to route it, or `jigc migrate <scratch>/vp3-notstaged.kIBpqL/rigs-cand3/jigc-rig-refs-post-hoc-2Hsxf2/repo/docs/research/latency-notes.md --as research` to rewrite it into the managed `research` shape; it is a foreign file, not an unmigrated managed doc
```

The state needs nothing unreadable, nothing untracked and nothing dirty.

### The chain, followed (rig `cand`, candidate, the `# x` plant)

Driven so that *is it a dead end* is answered from the binary rather than argued:

| # | command, as the previous refusal printed it | exit | what came back |
|---|---|---|---|
| 1 | `jigc doc show research:plain --task ground-the-vision-in-research` | 1 | `store.not-staged`, route: the task-less read |
| 2 | `jigc doc show research:plain` | 1 | `store.unparseable`, route: `jigc ingest`, or `jigc migrate <abs path> --as research` |
| 3 | `jigc ingest` | 0 | nine candidates; `needs-reconcile docs/research/plain.md → research`, route: the same `jigc migrate … --as research` |
| 4 | `jigc migrate <abs path> --as research` | 1 | `migrate.source-untracked`, route: `git -C <abs repo> add -- docs/research/plain.md`, then the same `jigc migrate` |
| 5 | `git -C <abs repo> add -- docs/research/plain.md` | 0 | - |
| 6 | `jigc migrate <abs path> --as research` | 0 | `task minted: migrate-research-docs-research-plain-9108c84ed0cb`, the `migrate-research` workflow composed |

I stopped there: the task's finalize needs authored prose, and it is not this finding's
door. Hops 3 to 6 were driven on the candidate only.

## Step 2 — is it what the finding says

Yes. The finding says the route of `store.not-staged` names a task-less read that
refuses for a readable file that is no managed doc, with no unreadable entry anywhere.
Observed: exactly that, at exit 1, on both binaries, on an untracked plant and on a
git-committed ordinary note.

**Against the design that owns it.** `design/doc-read-surface.md` gives the staged arm's
miss two arms, *routed on the real state*: *a committed sibling exists → the route names
the task-less read (`jigc doc show <addr>`); nothing exists anywhere → nothing to read
yet*. The same document, under `jigc doc list`, gives the task-less read of a file that
is not adopted its own answer: *`doc show`'s block on an unregistered instance routes to
adoption*, so that *the three surfaces tell one story: `doc list` says unregistered,
`doc show` says adopt it, `validate` says foreign — adopt it*.

So the **second** refusal is the designed one, and nothing about it is a defect: the
task-less read of a foreign file is meant to refuse and to route at adoption, and it
does. What the design does not give is the **first** refusal's words in this state. The
real state is *a file that is not a managed doc*; the refusal says *only its committed
copy exists* and that the task-less read *serves the committed copy*, and the binary
contradicts both one command later. The producer shows why — read, not driven:
`crates/engine/src/store.rs`, `not_staged_block`, takes the first arm on
`canonical_path(…).is_some_and(|p| p.is_file())`, the existence of a file at the home,
with no test of what the file is.

`design/surface-contract.md` names this shape and classes it: *a route offered on a
wider domain must be gated on that domain, not on the case that motivated it … A
mechanical route that hard-rejects, or repairs the wrong thing, is law 2 failing one
level down.* The route here is `Route::mechanical`, it is offered on *a file exists* and
was motivated by *a committed doc exists*, and it hard-rejects. The message beside it is
a law-1 matter by the same document (*every claim a surface makes is generated from the
thing it describes*).

I looked for a settled decision that intends a `store.not-staged` route ending in a
further refusal and found none: the two arms above are the whole of the design's text
for this refusal, and both suites that pin the first arm
(`crates/engine/src/store.rs` → `staged_read_of_an_absent_instance_blocks_on_the_real_state`;
`crates/cli/tests/doc_show_staged.rs` → `committed_json_carries_no_marker_and_not_staged_routes_task_less`)
build a committed managed doc and assert the route's text. Neither runs the route.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, from `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, sharpening 2:
*no command that works on rc.24 in a supported layout stops working, and every refusal's
route works as printed.* Its first half is not touched: nothing that worked on the
previous release stopped working, the two binaries answer alike. The second half is the
question triage sent.

**The reading I tried to refute it with.** A route *works* if following it gets the
reader to the repair; here every hop prints a route, every route runs, and hop 6 lands.
On that reading the defect is real, is a false sentence in a refusal, and breaks no
clause — `breaks-no-clause`.

**Why that reading does not survive.** Three things, each read from the repository
rather than supplied by me:

1. The clause is about **each refusal's** route, and says *as printed*. This refusal
   prints one command and one outcome for it — *serves the committed copy*. Run as
   printed, the command refuses and serves nothing. That the next refusal is a good one
   is a fact about the next refusal's route.
2. The repository's own meaning of a followable route is exit 0 of the emitted bytes:
   `crates/cli/tests/route_followability.rs` *runs the emitted backticked argv verbatim
   … asserting exit 0*, and `design/validation.md` → *The boundary-door selector* states
   the failure as *a route can hold all three and still exit 1 at the door that emitted
   it*. By that meaning this route is not followable in this state.
3. The surface contract classes a mechanical route that hard-rejects as a route failure
   (quoted in Step 2), without a clause for one that fails gracefully.

**Inside the scope.** The state is a supported layout, not a planted one: a file at a
managed doctype's home that is not adopted is the brownfield state the design calls
*unregistered*, lists on purpose, and gives a flow (`design/doc-read-surface.md` →
`jigc doc list`, *flow 43*). `jigc doc list` printed the address I then read. One open
task and one such file are all it takes; rig `cand3` reached it with a clean tree.

So it breaks the second half of `working-product`, inside its scope, and I could not
refute it. **What I am not saying:** how much it should weigh. It strands nobody, and it
is on the previous release too. The clause has no term for weight, and what follows from
a confirmed non-regression is not mine to decide.

## Step 4 — the regression fact

The same block on a fresh rig built with and driven by the previous release's binary
(`<scratch>/bin/previous-91834b5e011d/jigc`, hash above): red there as on the candidate.
`regression: false`. The door existed there and the block ran whole — comparable.

## Step 5 — coverage

The finding's block says `pinned-by: "UNPINNED: not searched"`, which is no coverage
claim, so there is none to verify. For my own block I searched, and say how: every file
under `crates/cli/tests`, `crates/engine` and `tooling-tests` that names
`store.not-staged` — nine suite files (`staged_read_miss_arm`, `spec_read_back_arms`,
`flow49_acceptance`, `doc_list`, `flow52_acceptance`, `migrate_locus_axis`,
`doc_show_staged`, `work_unit_unknown_envelope`, `no_such_task_route`) and the engine's
unit test named in Step 2. None of the nine names `store.unparseable` at all, and the
unit test's fixture is a committed, conforming doc. I did not read all nine end to end;
the claim I make is the narrow one the search supports: **no suite that names this
refusal also names the refusal its route ends in**, so nothing drives the route over a
file that is no managed doc.

## The repro block

### Repro P3-NS — the route of `store.not-staged` over a file that is no managed doc

```yaml
claim: "over a readable file that is no managed doc at a doctype's home, the route `store.not-staged` prints — the task-less read, said to serve the committed copy — refuses when run as printed"
verdict: CONFIRMED          # regression: false — the same on the previous release
setup:
  - fixture: refs-post-hoc          # live task: ground-the-vision-in-research; committed: research:context-loss
  - ["sh", "-c", "printf '# x\\n' > docs/research/plain.md"]        # mode 644, not added to git
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
      - "store.unparseable"
      - "required section heading `## question` is missing"
      - "it is a foreign file, not an unmigrated managed doc"
control:
  - ["jigc", "doc", "show", "research:context-loss", "--task", "ground-the-vision-in-research"]   # exit 1, the same refusal and route
  - ["jigc", "doc", "show", "research:context-loss"]                                              # exit 0, the committed doc on stdout
  - ["jigc", "doc", "show", "research:nope", "--task", "ground-the-vision-in-research"]           # exit 1, "has no committed copy — nothing to read yet"
variant: "a git-committed ordinary note at docs/research/latency-notes.md, clean tree: the same two refusals (the second names `section heading \"Numbers\" does not match required section `question``)"
test-in-waiting: "provoke the refusal over the plant, read the backticked argv out of its route, run those bytes verbatim, assert exit 0 — the mold of crates/cli/tests/route_followability.rs. Red today on both binaries. It asserts nothing about which route the fix prints."
also-on-previous-release: "yes — the same exits, messages and routes"
pinned-by: "UNPINNED: no suite that names store.not-staged also names store.unparseable (nine suite files and one engine unit test searched by name, not read whole)"
```

**Pinnable as it stands: yes.** The fixture is a named state of
`crates/cli/tests/support/trial_corpus.rs`, the plant is one argv step, every assertion
is an exit status or a substring that carries no host path, and the two controls sit on
the same rig. The one printed host path — the `jigc migrate` operand in the second
refusal — is outside every assertion.

## What was driven, and what was not

**Driven:** one doctype, `research` (a multi-instance doctype homed under
`docs/research/`), one fixture, one open task; two shapes of the file — an untracked
one-line plant and a git-committed ordinary note; both binaries for the first shape, the
candidate alone for the second and for hops 3 to 6 of the chain.

**The class: `instance, unbounded`.** I read one producer (`not_staged_block`) and its
predicate, and that is a reading of one function, not an enumeration: I did not derive
which other states reach its first arm without a servable doc behind it — a managed doc
whose committed file no longer parses, a placement doctype's foreign file at the
repository root, a home that holds a file the process cannot read (the handed block's
repro-b, which is another row's) — nor which other refusals hand over a read that can
refuse. The axis is the fixer's to derive.

**Not driven:** `--format json` on either read; a `#fragment` address; a second open
task; a linked worktree; any doctype but `research`.

## Left open — seen on the way, not pursued

1. **A printed route whose command refused, in the untracked shape.** The adoption
   route `jigc migrate <abs path> --as research` — printed by `store.unparseable`, and
   again by the `needs-reconcile` row of `jigc ingest` — exits 1
   `migrate.source-untracked` when the foreign file is not in git's index or `HEAD`; its
   own route (`git -C <abs repo> add -- docs/research/plain.md`, then the same command)
   then ran at exit 0 and exit 0. Candidate only; not driven on the previous release,
   not driven in the git-committed shape, not read against its design, not graded.
2. **The handed block's repro-b** (the same two reads over a mode-000 file, where the
   second refusal is `store.not-found` and routes back at the `--task` read) was not
   re-driven: triage sent control-b, and the unreadable shape has a row of its own.

## Where the evidence is

Under `<scratch>/vp3-notstaged.kIBpqL`: `mkrig.sh` and `run.sh` (the two helpers);
`cand.env`, `cand2.env`, `cand3.env`, `prev.env` (each rig's assignments) and the
matching `*.rig.log`; `<label>.<n>.out` and `<label>.<n>.err`, one pair per driven
command in the order driven; `cand.migrate.txt` and `cand.migrate2.txt` (hops 4 and 6
whole). The rigs are under `rigs-cand`, `rigs-cand2`, `rigs-cand3` and `rigs-prev`
there. The repository was read and not written: branch `fix/canary-one` at `eeffe347`,
its status the one untracked directory `completions/artifacts/canary-one/r1/` it had
when I started.

<!-- end of report -->
