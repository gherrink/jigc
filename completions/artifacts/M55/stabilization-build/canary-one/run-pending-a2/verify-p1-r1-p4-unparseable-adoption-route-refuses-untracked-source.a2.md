# verify-real — `r1-p4-unparseable-adoption-route-refuses-untracked-source` (run canary-one, round 1, stage test, attempt 2)

One finding, handed over: door `jigc doc show`, clause `working-product`, triage's grade
*unclear*, its repro the item *Left open, item 1* of the report
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-not-staged-route-task-less-read-refuses.a1.md`.
That report is the one I was handed; I read no other verifier's report and nothing of
triage's reasoning. What triage asked to be re-driven: the adoption route over an
untracked foreign file, on both binaries, and in the git-committed shape.

## Verdict

**confirmed** — `regression: false` (red on the previous release as well, the same
refusals word for word; the door existed there and the block ran whole — comparable).

**Basis, one line:** over an untracked foreign file at a doctype's home, the adoption
route that `jigc doc show`'s `store.unparseable` refusal prints — `jigc migrate <abs path>
--as research` — exits 1 `migrate.source-untracked` when run as printed, and the route's
other command, `jigc ingest`, exits 0 and prints the same refusing command as its row's
route; on both binaries; in the git-committed shape the same route exits 0 on both.

**What it is not, said so that nobody reads more into it than was driven.** It is not a
dead end and it loses nothing. The refusal the route runs into is the designed one, it
prints a route of its own (`git -C <abs repo> add -- <rel path>`, then the same
`jigc migrate`), and that route was followed to a minted migration task at exit 0 on both
binaries. No byte was written, moved or removed by either read or by the refused
`jigc migrate`: `git status --porcelain` shows the one planted file, untracked, before
and after, and its content is unchanged. The cost to the reader is one printed command
that refuses and one extra hop.

`contested: false`. The finding does not argue that the trackedness refusal is wrong, and
neither do I: that refusal is settled and stays out of this verdict (below, Step 2).

## The binaries, asserted before anything was driven

| | path | `content_sha256` printed by `dev/stabilize-step hash` |
|---|---|---|
| candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1) | `<scratch>/bin/c1.a2/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` |
| previous release (1.0.0-rc.24) | `<scratch>/bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` |

Both are the hashes the prompt handed over, each printed by the one plain call the
prompt spells, before the first driven command. Every driven command went through one
helper (`run.sh`) that puts the driven binary's directory first on `PATH` and exits 97
without running anything unless `command -v jigc` prints that binary's path; it never
exited 97. Every rig was built with `dev/jigc-rig --binary <that path> refs-post-hoc`
through a second helper (`mkrig.sh`) holding the same check. Nothing was built and
nothing under `target/` was driven. The previous release answers `jigc --version` with
`jigc 1.0.0-rc.24`; the candidate was not asked, because it prints the same string.

## Step 1 — driven from nothing

Six fresh rigs of my own under `<scratch>/vp4-adopt.FGjfBQ`, three per binary, each from
the rig's `refs-post-hoc` state: `cand`, `cand2`, `cand3` on the candidate and `prev`,
`prev2`, `prev3` on the previous release. Each exit status was read bare: stdout and
stderr went to two files, `$?` was read on the next line of the helper, no pipe anywhere
on a command whose status I report.

The finding gives a table of six hops rather than a runnable block, so I ran its hops as
its table names them, with the plant its parent report states: `printf '# x\n' >
docs/research/plain.md`, mode 644, not added to git. What I changed: I started at the
task-less read (the finding's own door, `jigc doc show`), not at the `--task` read that
precedes it in the handed report — that first hop belongs to another row.

### The untracked shape, candidate (rig `cand`, clean tree before the plant)

```
$ git status --porcelain                                                      exit 0
(empty)
$ printf '# x\n' > docs/research/plain.md                                     exit 0
$ git status --porcelain                                                      exit 0
?? docs/research/plain.md

$ jigc doc show research:plain                                                exit 1
(stdout empty; stderr:)
blocking · store.unparseable — `research:plain` at `docs/research/plain.md` does not parse: required section heading `## question` is missing
  at: research:plain
  route: adopt — run `jigc ingest` to route it, or `jigc migrate <scratch>/vp4-adopt.FGjfBQ/rigs-cand/jigc-rig-refs-post-hoc-AeUiMe/repo/docs/research/plain.md --as research` to rewrite it into the managed `research` shape; it is a foreign file, not an unmigrated managed doc

$ jigc ingest                                                                 exit 0
(stdout, the row for the plant; the other eight rows left out:)
needs-reconcile docs/research/plain.md → research
  blocking · conformance.section-missing — required section heading `## question` is missing
  at: research:plain#question
  route: `jigc migrate <scratch>/vp4-adopt.FGjfBQ/rigs-cand/jigc-rig-refs-post-hoc-AeUiMe/repo/docs/research/plain.md --as research` — it opens the `migrate-research` workflow, which rewrites the file to conformant shape and adopts it at finalize

$ jigc migrate <scratch>/vp4-adopt.FGjfBQ/rigs-cand/jigc-rig-refs-post-hoc-AeUiMe/repo/docs/research/plain.md --as research      exit 1
(stdout empty; stderr:)
blocking · migrate.source-untracked — `docs/research/plain.md` is in neither this repository's index nor its HEAD — git holds no copy of it, and `jigc task finalize --approve` retires the source it migrates, so the file would be deleted from the worktree with nothing to recover it from and no deletion in the commit to say so
  at: docs/research/plain.md
  route: stage it with `git -C <scratch>/vp4-adopt.FGjfBQ/rigs-cand/jigc-rig-refs-post-hoc-AeUiMe/repo add -- docs/research/plain.md`, then re-run `jigc migrate <scratch>/vp4-adopt.FGjfBQ/rigs-cand/jigc-rig-refs-post-hoc-AeUiMe/repo/docs/research/plain.md --as research` — the index is enough, the source need not be committed first

$ git status --porcelain                                                      exit 0
?? docs/research/plain.md
$ cat docs/research/plain.md                                                  exit 0
# x
```

Each `jigc` refusal from `doc show` and each `jigc ingest` report closes with the footer
line `— jigc · run "jigc start" for orientation; all writes through "jigc".` (its two
commands in backticks), which I leave out above.

Then the second refusal's own route, as printed:

```
$ git -C <scratch>/vp4-adopt.FGjfBQ/rigs-cand/jigc-rig-refs-post-hoc-AeUiMe/repo add -- docs/research/plain.md      exit 0
$ jigc migrate <scratch>/vp4-adopt.FGjfBQ/rigs-cand/jigc-rig-refs-post-hoc-AeUiMe/repo/docs/research/plain.md --as research      exit 0
(stdout opens:)
task minted: migrate-research-docs-research-plain-9108c84ed0cb
(the `migrate-research` workflow composed below it; stderr empty)
$ git status --porcelain                                                      exit 0
A  docs/research/plain.md
```

I stopped at the minted task: its finalize needs authored prose and is not this
finding's door.

### The untracked shape with nothing between the refusal and its route (rig `cand3`)

To rule out that the `jigc ingest` run between the two had changed anything: a third rig,
the plant, `jigc doc show research:plain` (exit 1, `store.unparseable`, the same route),
and the printed `jigc migrate … --as research` straight after it — exit 1,
`migrate.source-untracked`, the same message and route; `git status --porcelain` after
it: `?? docs/research/plain.md`.

### The previous release — the same two blocks (rigs `prev` and `prev3`)

`jigc doc show research:plain` exit 1 `store.unparseable`; `jigc ingest` exit 0 with the
`needs-reconcile` row and its `jigc migrate` route; the printed `jigc migrate` exit 1
`migrate.source-untracked`; the printed `git -C … add --` exit 0; the same `jigc migrate`
exit 0, `task minted: migrate-research-docs-research-plain-9108c84ed0cb`; and, on
`prev3`, the direct form exit 1 → exit 1. Read against the candidate's output above, the
messages and routes are the same; the only difference is the rig's own root inside the
printed paths.

### The git-committed shape (rigs `cand2` and `prev2`)

An ordinary note at `docs/research/latency-notes.md` (`# Latency notes`, a sentence,
`## Numbers`, two bullets), added and committed with git; `git status --porcelain` empty.

```
$ jigc doc show research:latency-notes                                        exit 1
blocking · store.unparseable — `research:latency-notes` at `docs/research/latency-notes.md` does not parse: section heading "Numbers" does not match required section `question`
  at: research:latency-notes
  route: adopt — run `jigc ingest` to route it, or `jigc migrate <abs repo>/docs/research/latency-notes.md --as research` to rewrite it into the managed `research` shape; it is a foreign file, not an unmigrated managed doc

$ jigc migrate <abs repo>/docs/research/latency-notes.md --as research        exit 0
task minted: migrate-research-docs-research-latency-notes-8b452c3296cc
$ git status --porcelain                                                      exit 0
(empty)
```

The same on both binaries: exit 1 with the same refusal, then exit 0 with the same task
id. **In the git-committed shape the route works as printed, on both binaries.** The
finding lives in the untracked shape alone — and, by the second refusal's own words and
the staged step above, a file that is in the index is enough.

## Step 2 — is it what the finding says

Yes. The finding says: the adoption route `jigc migrate <abs path> --as research`,
printed by `store.unparseable` and again by the `needs-reconcile` row of `jigc ingest`,
exits 1 `migrate.source-untracked` when the foreign file is in neither git's index nor
`HEAD`, and that refusal's own route then runs at exit 0 and exit 0. Observed: exactly
that, on both binaries. The two cells its author had not driven came out as: the
previous release — the same; the git-committed shape — the route works.

**Against the design that owns it — two things, which must not be confused.**

*The refusal the route runs into is intended, and settled.* `design/auto-migration.md` →
*Trackedness precondition (M51)*: a source git holds no copy of is refused with
`migrate.source-untracked` and a `Human` route naming `git add <path>`, *after which the
identical `jigc migrate` succeeds*; `design/validation.md` registers the code as the
trackedness leg of the migrate door; `DECISIONS.md` (M51, task T2) records it as *an
untracked in-repo source refuses, its route works*. Driven: it refuses, and its route
works. Nothing about that is a defect, and it is the reason this state loses no bytes.

*The route that hands the reader to it is not covered by that decision.* What prints
`jigc migrate … --as research` here is two other producers, and each states the rule it
is held to:

- `crates/engine/src/validate.rs` → `adoption_route` (the text `jigc doc show`'s block
  carries): *It obeys the M40 two-tier rule — never command a verb that hard-errors*;
  the `jigc migrate` half is named when the pack ships the `migrate-<ty>` workflow, and
  on no other condition.
- `crates/cli/src/ingest.rs` → `near_miss_route` (the row's route, built through
  `Route::mechanical`): `jigc migrate` is offered *only when the candidate is both
  unregistered and its doctype ships a `migrate-<doctype>` workflow*, because *a route
  that cannot be run, or one that repairs the wrong thing, is the defect one level down*.
- `design/surface-contract.md` → *The route fence (law 2)*: *A route offered on a wider
  domain must be gated on that domain, not on the case that motivated it … A mechanical
  route that hard-rejects, or repairs the wrong thing, is law 2 failing one level down.*

Both gates were read, not driven: each asks whether the file is registered and whether
the doctype is migratable, and neither asks whether git holds a copy — the one question
the verb they name asks before it does anything.

**The search for a decision that intends this.** I looked for a ruling that the upstream
routes hand over a `jigc migrate` that then refuses on trackedness, and found none. How:
every line of `DECISIONS.md`, `design/`, `implementation/` and the M51 records that names
`migrate.source-untracked` or the trackedness precondition, filtered to those that also
name `jigc ingest`, `doc show`, `store.unparseable`, the near-miss or the adoption route
— three lines, none of which rules on it (one is the M51 audit's quoting defect in the
refusal's own `git add` route, one registers the codes, one is an unrelated census). The
M51 decision names the door and its fixtures; it says nothing of the routes that point
at the door. That is a search by name over those files, not a reading of them whole.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, from `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, sharpening 2:
*no command that works on rc.24 in a supported layout stops working, and every refusal's
route works as printed.* The first half is not touched — the two binaries answer alike.
The second half is the question triage sent.

**The three refutations I tried, and what became of each.**

1. *It does not reproduce* — stale state, another binary, a status read through a pipe.
   It reproduces on six rigs of my own, three per asserted binary, each status read bare,
   with and without a `jigc ingest` between the refusal and its route.
2. *It is intended.* The refusal at the migrate door is (Step 2). The route that leads
   into it is not: the design's own text for both producers forbids handing over a verb
   that rejects, and no decision I could find exempts the trackedness leg.
3. *It breaks no clause: the chain lands.* Every hop prints a route, the last one works,
   and the reader arrives. On that reading this is one redundant hop and no clause is
   broken. It does not survive, for three reasons read from the repository:
   - The clause is about **each refusal's** route, **as printed**. `store.unparseable`
     is a refusal (exit 1) and it prints two commands. One of them, run as printed,
     refuses. The other, `jigc ingest`, exits 0 — and its row prints the same refusing
     command as its one route. So no branch of this refusal's route reaches adoption
     without meeting a second refusal.
   - The repository's own meaning of a route that works is exit 0 of the emitted bytes:
     `crates/cli/tests/route_followability.rs` *runs the emitted backticked argv verbatim
     … asserting exit 0*, and `design/validation.md` → *The boundary-door selector*
     names the failure in these words: *a route can hold all three and still exit 1 at
     the door that emitted it*.
   - The surface contract classes a mechanical route that hard-rejects as a route
     failure, with no carve-out for one whose rejection is itself well routed.

**Inside the scope.** A supported layout, and an ordinary one: a set-up repository with a
note at a managed doctype's home that nobody has run `git add` on yet. Nothing is
planted beyond a file a person writes by hand; no permission, link or git configuration
is involved. It is also the state of a first adoption, where `jigc ingest` is run over a
tree that holds new files.

So it breaks the second half of `working-product`, inside its scope, and I could not
refute it.

**One reading I flag and do not settle.** Of the two producers, only `jigc doc show`'s is
a refusal in the plain sense — the verb exits 1. `jigc ingest` exits 0; the route sits
on a blocking finding row inside a report. Whether the clause's word *refusal* reaches
such a row is a reading. The verdict does not rest on it: the `doc show` producer is the
finding's door and suffices alone.

**What I am not saying:** how much it should weigh. It strands nobody, it destroys
nothing, it is on the previous release too, and the hop it adds is the one that protects
the file. The clause has no term for weight, and what follows from a confirmed
non-regression is not mine to decide.

## Step 4 — the regression fact

The same blocks on fresh rigs built with and driven by the previous release's binary
(`<scratch>/bin/previous-91834b5e011d/jigc`, hash above): red there as on the candidate,
exit for exit and message for message. **`regression: false`** — comparable, the door
and both producers existed there.

## Step 5 — coverage

The finding makes no coverage claim, so there is none to verify. For my own block I
searched by name, and say how: under `crates/cli/tests` and `tooling-tests`, six suite
files name `migrate.source-untracked` (`git_span_aim`, `migrate_source_rules`,
`flow52_acceptance`, `pre_dispatch_faults`, `retire_sink_validation`,
`path_arg_occurrence_axis`); none of the six names `store.unparseable` or
`needs-reconcile`. In the other direction, five suite files name `store.unparseable` and
twenty-one name `needs-reconcile`; none of them names `migrate.source-untracked`. I read
none of them end to end. The claim the search supports is the narrow one: **no suite
that names either producer's verdict also names the refusal its route ends in**, so
nothing drives the adoption route over a file git holds no copy of.

## The repro block

### Repro P4-AU — the adoption route over an untracked foreign file

```yaml
claim: "over an untracked foreign file at a doctype's home, the `jigc migrate <abs path> --as <doctype>` that `store.unparseable` and `jigc ingest`'s needs-reconcile row print as the adoption route refuses `migrate.source-untracked` when run as printed"
verdict: CONFIRMED          # regression: false — the same on the previous release
setup:
  - fixture: refs-post-hoc          # committed store holds research:context-loss; clean tree
  - ["sh", "-c", "printf '# x\\n' > docs/research/plain.md"]        # mode 644, NOT added to git
repro:
  - ["jigc", "doc", "show", "research:plain"]
  - ["jigc", "migrate", "<repo>/docs/research/plain.md", "--as", "research"]     # the first step's route, the bytes it printed
  - ["jigc", "ingest"]
  - ["git", "status", "--porcelain"]
expect:                              # as observed today, on both binaries
  - exit: 1
    stdout: ""
    stderr_contains:
      - "store.unparseable"
      - "route: adopt — run `jigc ingest` to route it, or `jigc migrate "
      - "/docs/research/plain.md --as research` to rewrite it into the managed `research` shape"
  - exit: 1
    stdout: ""
    stderr_contains:
      - "migrate.source-untracked"
      - "is in neither this repository's index nor its HEAD"
      - " add -- docs/research/plain.md`, then re-run `jigc migrate "
  - exit: 0
    stdout_contains:
      - "needs-reconcile docs/research/plain.md → research"
      - "/docs/research/plain.md --as research` — it opens the `migrate-research` workflow"
  - exit: 0
    stdout: "?? docs/research/plain.md\n"     # nothing written, moved or staged by any step
control:
  - ["git", "-C", "<repo>", "add", "--", "docs/research/plain.md"]                                 # exit 0 — the second refusal's own route
  - ["jigc", "migrate", "<repo>/docs/research/plain.md", "--as", "research"]          # exit 0, stdout opens "task minted: migrate-research-docs-research-plain-"
variant: "a git-committed ordinary note at docs/research/latency-notes.md, clean tree: `jigc doc show` refuses store.unparseable with the same route shape, and the printed `jigc migrate … --as research` exits 0 with a minted task — on both binaries"
test-in-waiting: "provoke store.unparseable over the untracked plant, read the backticked `jigc migrate` argv out of its route, run those bytes verbatim, assert exit 0 — the mold of crates/cli/tests/route_followability.rs — and the same over the needs-reconcile row's route. Red today on both binaries. It asserts nothing about which route the fix prints, and it must keep the control: the bytes of an untracked source are in a git object, or still on disk, at every exit 0."
also-on-previous-release: "yes — the same exits, messages and routes"
pinned-by: "UNPINNED: no suite that names store.unparseable or needs-reconcile also names migrate.source-untracked (searched by name, not read whole)"
```

**Pinnable as it stands: yes, with one substitution.** The fixture is a named state of
`crates/cli/tests/support/trial_corpus.rs`, the plant is one argv step, and every
assertion is an exit status or a substring that carries no host path. The one thing a
test must do that the block only names: `<repo>` in the second step is not typed by the
test — it is the argv lifted out of the first step's printed route, which is absolute.

## What was driven, and what was not

**Driven:** one doctype, `research` (a multi-instance doctype homed under
`docs/research/`), one fixture; two shapes of the file — an untracked one-line plant and
a git-committed ordinary note — and, by way of the second refusal's route, a staged and
uncommitted one; both binaries for every shape; the route from both producers
(`jigc doc show`'s refusal and `jigc ingest`'s row), text output only.

**The class: `instance, unbounded`.** I read two producers of the adoption route
(`adoption_route`, `near_miss_route`) and their gates; that is a reading of two
functions, not an enumeration. I did not derive which other surfaces print a
`jigc migrate` over a file they have not asked git about — the store sweep's
`schema-conformance.unadopted-instance` advisory shares `adoption_route` by its own doc
comment and was not driven — nor which of the migrate door's other legs
(`migrate.source-untrackable`) an upstream route can run into. The axis is the fixer's
to derive.

**Not driven:** `--format json` on any step; a gitignored file; a placement doctype's
foreign file at the repository root; a linked worktree; a caller directory other than
the repository root; any doctype but `research`; `jigc validate` over the same plant.

## Left open — seen on the way, not pursued

None. The first hop of the handed report (`store.not-staged` routing at the task-less
read) is another row's and was not re-driven here.

## Where the evidence is

Under `<scratch>/vp4-adopt.FGjfBQ`: `mkrig.sh`, `run.sh`, `committed.sh` and `direct.sh`
(the four helpers); `<label>.env` and `<label>.rig.log` for each of the six rigs;
`<label>.<n>.out` and `<label>.<n>.err`, one pair per driven command in the order
driven; `prev2.transcript`, `cand3.transcript` and `prev3.transcript` (three runs
whole). The rigs are under `rigs-<label>` there. The repository was read and not
written: branch `fix/canary-one` at `126a8531`, its status the three untracked report
files under `completions/artifacts/canary-one/r1/reports/test/` it had when I started.

<!-- end of report -->
