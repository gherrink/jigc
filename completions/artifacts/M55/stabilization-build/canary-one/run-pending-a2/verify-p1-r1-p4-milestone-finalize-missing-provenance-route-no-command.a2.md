# verify-real — `r1-p4-milestone-finalize-missing-provenance-route-no-command` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-milestone-finalize-missing-provenance-route-no-command`. One finding, handed over:
ledger key `r1-p4-milestone-finalize-missing-provenance-route-no-command`, door `jigc milestone finalize`,
the clause it is said to break `working-product`, triage's grade *unclear*. Its repro is item 5 of
*Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-staged-doc-ids-orientation-and-milestone-callers.a1.md`,
which the prompt handed over as this finding's source; it was read for that item, for the plant
and for the setup of its `Repro V-3`. No other report and nothing of triage's reasoning was read.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`. The observation reproduces exactly; the route
is thin, and that is a real defect that stays a row of the ledger; it breaks neither half of the
second clause.**

- **It reproduces, byte for byte.** With a mode-000 file `research:locked.md` alone beside the
  staged doc of sub-task `area-low`, `jigc milestone finalize cache-rework` exits **1**, stdout
  empty, stderr 225 bytes, three lines: a blocking `join.missing-provenance` finding whose message
  is *staged doc research:locked in sub-task area-low of milestone cache-rework has no recorded
  provenance*; `at: research:locked`; `route: re-stage the doc so its provenance is recorded`. The
  route names no command. Nothing is removed, moved or committed; the one change is a derived
  cache added (`.jigc/index/edges.json`).
- **The mode is not what the door answers.** The same file at mode 644 gives the same 225 bytes
  (`cmp`). The door asks whether the staging area holds a `docs/*.md` with no entry in the area's
  `provenance.json`; it does not read the file's bytes. The finding is about a hand-placed file in
  a staging area, of any mode.
- **A route that names no command is a kind the design defines.** `design/validation.md` → *The
  route floor*: a repair route is *mechanical … or a non-mechanical human one … a real fix jigc
  cannot execute*; `crates/engine/src/finding.rs`, `impl From<String> for Route`, makes this
  producer's string a `Human` route. So *names no command* is, taken alone, intended.
- **What is left of the finding is whether the sentence can be followed — and it can, with one
  step it does not say.** Driven: the acts a reader reaches for first fail or change nothing
  (`jigc doc create research --title Locked --task area-low` exits 1, `create.gate-blocked`; over
  jigc's own doc whose manifest was removed, `jigc doc create` and `jigc doc set-slot` exit 0 and
  record no provenance, and the refusal stands). The act that works is taking the unrecorded body
  out of the area: after that `jigc milestone finalize` gives the no-plant control's answer byte
  for byte, and where the body was jigc's own, `jigc doc create` then mints it again **with** its
  manifest. The route says *re-stage* and not *take it out first*. That is the defect.
- **It does not break the clause.** The second clause's instrument (DECISIONS.md, 2026-10-04, *The
  exit rule, revised*, sharpening 2): *no command that works on rc.24 in a supported layout stops
  working, and every refusal's route works as printed*. No command stopped working: all 22 paired
  invocations are byte-identical on the previous release, exit status, stdout and stderr. And no
  printed command fails: the route that leads here — `jigc milestone finalize cache-rework`,
  printed by the `milestone.staged-prose` refusal with the words *which refuses, with its own
  route* — runs as printed and refuses with its own route; this refusal's own route prints no
  command at all.
- **The state is one the design already records as answered this way.** `design/worked-examples.md`
  → flow 53, *Bounds*: at the two displacing doors a hand-placed `docs/*.md` *is answered by the gate
  and not by the teardown — … `join.missing-provenance` at the milestone boundary … No byte dies and
  nothing is silent*; DECISIONS.md, 2026-09-20, *M52 Increment 11 / T1*, records the cell as *recorded
  rather than narrowed away*. That entry's own plant, `not-an-identity.md`, was driven here too: the
  same code, the same route, 225 bytes.

`contested: false` — the finding does not argue that a settled decision is wrong.

**This verdict rests on a reading, and the reading is stated so that it can be overruled.** *Works
as printed* is read here as: what a refusal prints to be run, run as printed, does what it says.
The other reading — every refusal must name a way out that works without a step the reader has to
find alone — is argued below under *The strongest reading against this verdict*; under it this row
would be the clause's subject. Which reading holds is not a verifier's to settle.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed one line with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (1.0.0-rc.24).
- The binary's directory went first on `PATH` in every driver call, and each call stops unless
  `command -v jigc` prints the binary it was handed. The construction log of every rig holds that
  line: nine times `<scratch>/bin/c1.a2/jigc`, nine times `<scratch>/bin/previous-91834b5e011d/jigc`.
- No `cargo build`, nothing under `target/`. Every rig:
  `SCRATCH=<W>/rigs dev/jigc-rig --binary <binary> refs-post-hoc`, stdout captured alone, the
  construction log to its own file, exit status 0 each time.
- The clone: `HEAD` 126a8531 on `fix/canary-one`; `git diff --stat eeffe347 HEAD` over `crates`,
  `design`, `dev`, `implementation` and `DECISIONS.md` is empty, so the source read is the candidate's.
  `git status --porcelain` shows untracked files under
  `completions/artifacts/canary-one/r1/reports/test/` and nothing else, before and after. Nothing was
  edited, staged or committed.

## What was reconstructed, and what was changed

Item 5 is one line of a table and one sentence; it has no block of its own. The setup is that of
the source report's `Repro V-3`, as written: the fixture `refs-post-hoc`, then four commands with
the binary under test, each exit status 0 —
`jigc milestone create "Cache rework"`, `jigc milestone add-task cache-rework "Area low"`,
`jigc milestone add-task cache-rework "Area zed"`, `jigc doc create adr --title "Low policy" --task area-low`.
That leaves sub-task `area-low` staging `adr:low-policy` (with `docs/provenance.json` naming it
`created`) and `area-zed` staging nothing. No worktree is provisioned.

- **The plant**, alone, in `.jigc/tasks/area-low/docs/`: a regular file `research:locked.md`, then
  `chmod 000`. **Changed:** the source report says 29 bytes and does not give them; this plant is
  30 bytes (one line of text). The door does not read them.
- **Changed:** the source report reached `jigc milestone finalize cache-rework` as a route of the
  refused `jigc milestone discard cache-rework`, in the rig that refusal had left as it was. Here
  the door is driven directly on a fresh rig (two cells), and once more after the refused
  discard (one cell); the three answers are the same 225 bytes.
- Nothing else was written into `.jigc/` by hand except, in the variants, the plant named there or
  the removal named there.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user.** Nothing
was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p1-ms-finalize-provenance.8tzhIX` (written `<W>`). **18 rigs, nine per binary**,
each from the rig tool's own `mktemp -d` under `<W>/rigs/`. Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, the working directory the rig's repository, stdin from `/dev/null`, stdout
and stderr to their own files, the exit status read directly and never through a pipe. Around every
invocation a snapshot (`<W>/tools/snap.sh`): one line per entry under `.jigc/` — kind, mode, size
and the first 16 hex digits of its sha256, the link's target, or `UNREADABLE` for the mode-000 file
— then `git status --porcelain --untracked-files=all --ignored`, `HEAD`, `git rev-list --count HEAD`
and `git worktree list --porcelain`. Per-invocation files under `<W>/runs/<cell>/`:
`<tag>.stdout`, `.stderr`, `.exit`, `.argv`, `.snap.before`, `.snap.after`, `.snap.diff`, and the
cell's `build.log` and `rig`.

## What was driven — the candidate

In every row below stdout is empty unless given, and *tree* is the snapshot's difference.

### The door, under the plant and its controls

| cell | plant in `area-low/docs/` | exit | stderr | tree |
|---|---|---|---|---|
| cand-none | none | 3 | 1122 bytes: one advisory `file-state.staged-copy`, three blocking `schema-conformance.required-slot-present` over `adr:low-policy`, each with a `jigc doc set-slot …` route | added: `.jigc/index/edges.json`, `.jigc/milestones/cache-rework/merged/docs/adr:low-policy.md`, an empty `.jigc/worktrees/` |
| cand-locked-a | `research:locked.md`, mode 000 | **1** | 225 bytes: `join.missing-provenance` over `research:locked`, the route *re-stage the doc so its provenance is recorded* | added: `.jigc/index/edges.json`; nothing else |
| cand-locked-b | the same | 1 | the same 225 bytes (`cmp`) | the same |
| cand-locked-c | the same, after a refused `jigc milestone discard cache-rework` | 1 | the same 225 bytes (`cmp`) | the same |
| cand-readable | `research:locked.md`, mode 644 | 1 | the same 225 bytes (`cmp`) | the same |
| cand-adr-locked | `adr:locked.md`, mode 000 | 1 | 215 bytes: the same finding over `adr:locked`, the same route | the same |
| cand-notid | `not-an-identity.md`, mode 644 — the plant of flow 53's recorded cell | 1 | 225 bytes: the same finding over `not-an-identity`, the same route | the same |
| cand-noprov, cand-noprov-b | none; `provenance.json` of `area-low` removed by hand | 1 | 223 bytes: the same finding over `adr:low-policy`, the same route; the two cells byte-identical | the same |

The refusal is stable: a second `jigc milestone finalize cache-rework` in cand-locked-a prints the
same bytes and leaves the tree identical. With `--format json` the exit is 1, stdout is empty and
stderr is one JSON object with the single member `error`, whose value is the three text lines
joined with line breaks (244 bytes).

In no refusing cell is anything under `.jigc/tasks/` changed, and `HEAD`, the commit count, the
porcelain of the tracked files and the worktree list do not move. The plant stands, at its mode.

### The route that leads here, as printed

cand-locked-c, `jigc milestone discard cache-rework`: exit 1, 678 bytes, tree identical — a blocking
`milestone.staged-prose` finding listing `area-low: adr:low-policy, research:locked`, and in its
route: *or land the milestone with `jigc milestone finalize cache-rework` (which refuses, with its
own route, while the milestone has nothing to land or a required slot is empty)*. Run as printed,
that command is the 225-byte refusal above. It refuses, with its own route; the reason it refuses
for is not one of the two the sentence lists.

### This refusal's own route, followed

The route is one sentence and names no command, so each row is one reading of it, in the rig of
the cell named, after that cell's refusal.

| cell | the act | exit | what it printed | tree | `jigc milestone finalize cache-rework` afterwards |
|---|---|---|---|---|---|
| cand-locked-a | `jigc doc create research --title Locked --task area-low` | **1** | stderr 428 bytes: blocking `create.gate-blocked` — *the workflow does not allow `jigc doc create research` in-task; allowed doctypes: [adr]* — with a route at a task minted from a workflow that grants the doctype | identical | not run again in this cell; the state is that of the refusal |
| cand-locked-b | the plant removed by hand (`rm`, one file) | 0 | - | the plant gone | **exit 3, 1122 bytes, byte-identical to cand-none** (`cmp`) |
| cand-adr-locked | `jigc doc create adr --title Locked --task area-low` | **1** | stderr 549 bytes: blocking `write.identity-change` — *this task's `decision` is already `adr:low-policy`, and this call would mint `adr:locked` instead* — routed at `jigc doc rename` | identical | exit 1, the same 215 bytes |
| cand-noprov | `jigc doc create adr --title "Low policy" --task area-low` | **0** | stdout: `adr:low-policy (already existed — copied in for update)` | **identical — no manifest written** | exit 1, the same 223 bytes |
| cand-noprov | then `jigc doc set-slot "adr:low-policy#context" --task area-low --from-file <a 20-byte file>` | **0** | stdout: `set slot adr:low-policy#context (20 chars)` | the body rewritten (134 to 153 bytes); **no manifest written** | exit 1, the same 223 bytes |
| cand-noprov-b | the body moved out of the area by hand (`mv`), then `jigc doc create adr --title "Low policy" --task area-low` | 0 | stdout: `adr:low-policy` | the body and `provenance.json` (52 bytes) written | **exit 3, 1122 bytes, byte-identical to cand-none** (`cmp`) |

So the sentence has one reading that works, in the handed instance and in the population the
producer's own comment names (*a real fault, since every staging primitive records the bit beside
the body*): the unrecorded body leaves the area, and what is wanted of it is staged through jigc.
For the handed plant the second half is not this sub-task's to do — its workflow grants `adr`
alone, and the `create.gate-blocked` route says where a `research` doc is staged. Every reading
that leaves the body where it is fails: jigc refuses it, or accepts it at exit 0 and records
nothing.

## Against the claim, and against the design that owns the behaviour

**The claim.** *`jigc milestone finalize <id>` under the mode-000 plant exits 1 with
`join.missing-provenance`, whose route — re-stage the doc so its provenance is recorded — names no
command. It is a route the staged-prose refusal prints. Identical on the previous release.* Every
part of it holds as written.

**The design.**

- *The route's kind.* `design/validation.md` → *The route floor*: *Every finding carries a route —
  two kinds, never `null`. A repair route names an action — mechanical (`run jigc …`) or a
  non-mechanical human one … a real fix jigc cannot execute.* `design/command-output-contract.md`,
  the `route` field, says the same. `design/surface-contract.md` → *The route fence*: *`From<String>`
  keeps an un-migrated producer compiling as `Human`*. The producer,
  `missing_provenance_finding` in `crates/engine/src/milestone.rs`, hands a string. A route with no
  command is this kind, and no fence asks more of it: P6, route-followability, *reads an argv,
  which only a mechanical route has* (`design/validation.md`, the same section).
- *The refusal over a hand-placed file.* `design/storage.md` → *The by-task-id join*, rule 2: staged
  docs are *classified by provenance*, a bit *recorded at stage time* in `<area>/docs/provenance.json`.
  The producer's comment: *Blocking, routed to re-stage; never defaulted to a provenance the clash
  rule would then mis-decide on.* `design/worked-examples.md` → flow 53, *Bounds*, and DECISIONS.md,
  2026-09-20, *M52 Increment 11 / T1*, record that a hand-placed `docs/*.md` is answered at this door
  under this code, call the route *followable*, and keep the cell *recorded rather than narrowed
  away*: *No byte dies and nothing is silent, but two readers of one file answer under different
  identities.* The refusal, its code and its failing closed are settled.
- *What no decision rules.* This verifier found no dated decision on what the route's sentence owes
  a reader whose file was never staged by jigc. The design calls the route followable; driven, it
  is followable only by the step it leaves out. So the basis is `breaks-no-clause`, not `intended`:
  the kind of the route is intended, its wording in this state is a defect nobody ruled on.

**The clause.** The run's opening record names it: *`working-product` — a working product others
can rely on. Instrument: the regression set, and here the gate.* DECISIONS.md, 2026-10-04,
sharpening 2: *no command that works on rc.24 in a supported layout stops working, and every
refusal's route works as printed.* Read against the cells:

- *No command that works on rc.24 … stops working.* The door answers the same on both binaries in
  every cell, and so does every act of the route table.
- *Every refusal's route works as printed.* Two refusals are in play. The staged-prose refusal
  prints a command and says it may refuse with its own route; it does exactly that. The
  missing-provenance refusal prints a sentence and no command; nothing it prints fails when run,
  because nothing it prints can be run. The ruling that shaped the instrument says as much of its
  own reach (DECISIONS.md, 2026-10-06, *Ahead of the run's opening*, item 4, *The regression set's
  shape*): *a refusal's route that is a sentence for a human can be run by neither part; printed
  commands are run wherever a cell ends in a refusal, and the rest stays with the review rows.*
- *The state.* It is reached here by a hand write into a task's staging area, or by removing a
  file jigc wrote there. No jigc command was found to leave it, and none was looked for (*Left
  open*, item 6). The verdict does not rest on the state being planted: the opening record declares
  no bound.

**The strongest reading against this verdict, stated so that triage can weigh it.** Read by its
purpose, *every refusal's route works* asks that a refusal name a way out. Here an agent that does
what the sentence says in the plainest way gets a second refusal (`create.gate-blocked`) whose
route, followed, stages a `research` doc in another task and leaves the stray file standing — so
the milestone still cannot land, and no surface ever says *remove the file*. Where the body is
jigc's own, the plainest act succeeds at exit 0, says *copied in for update*, and changes nothing
the door reads. The sibling door's route for the same file does name the act: `design/validation.md`
→ *The route floor* gives the task door's `unknown-type` route as *restore the doctype's cascade
entry or remove/re-type the stray staged file*. On that reading this is a refusal whose route does
not work, at a door that lands a milestone, and it is the clause's subject whatever the release it
first shipped in. Against it: the wording of the sharpening is *as printed*; the ruling on the
instrument sets sentences apart from printed commands; the door fails closed, loses nothing, and
gives the control's answer the moment the file is gone; and the state needs a hand in `.jigc/`.

## The same cells on the previous release

The verdict is not `confirmed`, so no `regression` field is returned. Every cell and every act was
run on the previous release for comparison — nine rigs, per-invocation files under
`<W>/runs/prev-*/`.

- **22 paired invocations, 22 byte-identical**: the exit status, stdout and stderr of each (`cmp`,
  nothing normalised — none of these outputs carries a path or a hash). The pairs: the door's first
  answer in each of the nine cells; its repeat and its JSON form in locked-a; the refused discard
  in locked-c; the door after the removal in locked-b; the four `jigc doc create` acts and the one
  `jigc doc set-slot`; and the door after each of the four acts that was followed by one.
- One invocation has no twin: `jigc doc create --help` on the candidate, read for the verb's flags.

## Scope of what was verified

**Instance, unbounded.** Driven: one door, `jigc milestone finalize <id>`, text and JSON, without
provisioned worktrees; five states of one sub-task's staging area (the handed plant, the same file
readable, an `adr`-named plant, a name that is no staged identity, a removed manifest) and the
no-plant control; six readings of the route; one platform; two binaries.

What was counted: `grep -rn "missing_provenance_finding\|missing-provenance" crates/engine/src crates/cli/src`
gives **one producer and one call site** (`crates/engine/src/milestone.rs`, in the join's
classification). The doors that run the join were **not** enumerated, and no door but
`jigc milestone finalize` was driven. The plants are not bounded either: a link, a directory, a
fifo, an unreadable `docs/` directory, a manifest that is present and malformed, a manifest naming
a doc the area does not hold, and a plant in a second sub-task were not driven at this door.

## Repro V-4

```yaml
claim: "under a mode-000 file named as a staged doc in a sub-task's staging area, `jigc milestone finalize <id>` exits 1 with `join.missing-provenance`, whose route names no command — and that breaks `working-product`"
verdict: REFUTED   # basis breaks-no-clause: the observation holds byte for byte; the route is the Human kind the route floor defines, no printed command fails, and the previous release answers identically. The route's wording is a real defect: it does not say that the unrecorded body has to leave the area first
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "22 paired invocations byte-identical — exit status, stdout, stderr (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc
  - ["jigc", "milestone", "create", "Cache rework"]
  - ["jigc", "milestone", "add-task", "cache-rework", "Area low"]
  - ["jigc", "milestone", "add-task", "cache-rework", "Area zed"]
  - ["jigc", "doc", "create", "adr", "--title", "Low policy", "--task", "area-low"]
  - write: { path: ".jigc/tasks/area-low/docs/research:locked.md", bytes: "any", mode: "000" }
repro:                                      # in order, in one fixture
  - ["jigc", "milestone", "finalize", "cache-rework"]
  - ["jigc", "doc", "create", "research", "--title", "Locked", "--task", "area-low"]
  - remove: ".jigc/tasks/area-low/docs/research:locked.md"
  - ["jigc", "milestone", "finalize", "cache-rework"]
expect:
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · join.missing-provenance", "staged doc `research:locked` in sub-task `area-low` of milestone `cache-rework` has no recorded provenance", "route: re-stage the doc so its provenance is recorded"]
    tree: "nothing under .jigc/tasks/ changed, the plant standing at mode 000; HEAD, the commit count, the porcelain of tracked files and the worktree list unchanged; at most .jigc/index/edges.json added"
  - exit: 1
    stderr_contains: ["blocking · create.gate-blocked", "allowed doctypes: [adr]"]   # TODAY's answer to the route's plainest reading
    tree: "identical"
  - exit: 0                                 # the hand removal
  - exit: 3
    stderr_contains: ["schema-conformance.required-slot-present", "adr:low-policy#context"]
    stderr_lacks: ["join.missing-provenance"]
    stderr_equals_control: true             # byte-identical to the same argv with no plant
variants:        # each alone, in place of the write
  - "the same file at mode 644                 -> expectation 1 unchanged, byte for byte"
  - "adr:locked.md, mode 000                  -> exit 1, the finding over `adr:locked`; `jigc doc create adr --title Locked --task area-low` exits 1 with `write.identity-change`"
  - "not-an-identity.md, mode 644             -> exit 1, the finding over `not-an-identity`, the same route"
  - "no write; .jigc/tasks/area-low/docs/provenance.json removed -> exit 1, the finding over `adr:low-policy`; `jigc doc create adr --title \"Low policy\" --task area-low` exits 0 with `already existed — copied in for update` and writes no manifest; `jigc doc set-slot` exits 0 and writes none; the refusal stands. With the body moved out first, the same create exits 0, writes body and manifest, and finalize gives the control's answer"
control: "no plant -> exit 3, 1122 bytes: one advisory and three `required-slot-present` findings over adr:low-policy"
observed: "<W>/runs/cand-*/ (stdout, stderr, exit, argv, snap.before, snap.after, snap.diff, build.log); the previous release: <W>/runs/prev-*/"
pinned-by: "UNPINNED: the near miss is flow53_acceptance::the_displacing_doors_keep_every_byte_they_cannot_commit, whose assertions were read (not run): over `docs/not-an-identity.md` at this door it asserts a failing status and that the output holds `join.missing-provenance`, `not-an-identity` and the word `route`, then removes the file and asserts that the boundary lands. It does not assert the exit status, the route's text, a plant named as a staged identity, the mode, an unchanged tree after the refusal, or any act of the route. `grep -rn 're-stage the doc so its provenance' crates tooling-tests` finds the producer and no test."
```

**Pinnable as it stands: expectations 1 and 4 yes; expectation 2 only as a statement of today's
behaviour.** The fixture is a named state of the shared builder plus four argv steps and one file
write, so the block converts by hand. Conditions: a Unix target for the mode; the door does not
read the file's bytes, so the answer is expected to hold for a root caller too — not driven.
Expectation 1 with its `tree:` line and expectation 4 are the half worth a standing test: the
refusal changes nothing, and the door answers as if the file had never been there once it is gone.
Expectation 2 and the fourth variant pin answers a repair of the route's wording, or of what
`jigc doc create` does over an unrecorded body, would change on purpose; whoever converts them
should write them as that repair's red test. The comparison with the previous release is not a
suite's to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **The route does not say that the unrecorded body has to leave the area.** *re-stage the doc so
   its provenance is recorded* is followable only by removing or moving the file first; every
   reading that leaves it in place fails. The sibling route at the task door names the removal
   (`design/validation.md` → *The route floor*, the `unknown-type` block). `design/worked-examples.md`
   → flow 53, *Bounds*, calls this route *followable*. Identical on the previous release.
2. **`jigc doc create` over a staged body that has no provenance entry exits 0, says
   `already existed — copied in for update`, and records nothing**; `jigc doc set-slot` on the same
   doc exits 0, rewrites the body and records nothing either. The milestone door then refuses as
   before. Reached here by removing `provenance.json` by hand. Identical on the previous release.
3. **The recorded exit status is not the driven one.** DECISIONS.md, 2026-09-20, *M52 Increment 11 /
   T1*, says the milestone boundary's join answers `join.missing-provenance` *at exit 3*; driven
   with that entry's own plant (`docs/not-an-identity.md`) the exit is 1, on both binaries.
4. **The staged-prose route's description of the door it routes at is short by one reason.** It
   says `jigc milestone finalize <id>` *refuses, with its own route, while the milestone has nothing
   to land or a required slot is empty*; here it refuses for a third reason.
5. **With `--format json` the refusal is one object with the single member `error` on stderr, and
   stdout is empty** — the finding's code, target and route are not fields. Not compared against
   `design/command-output-contract.md` by this verifier.
6. **Reach was not driven.** Whether any jigc verb, an interrupted write, or ordinary use can leave
   a staged body without its provenance entry was not tested. Every state here was made by hand.
7. **The message calls a file that jigc never staged a *staged doc*** — the two-identities cell the
   design records. Not pursued beyond the cells above.
8. **Other doors that run the join, other plants at this door, a provisioned milestone, Linux and
   a root caller were not driven.**

<!-- end of report -->
