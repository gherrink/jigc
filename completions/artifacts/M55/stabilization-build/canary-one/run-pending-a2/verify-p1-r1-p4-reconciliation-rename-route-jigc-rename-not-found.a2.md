# verify-real — `r1-p4-reconciliation-rename-route-jigc-rename-not-found` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-reconciliation-rename-route-jigc-rename-not-found`. One finding, handed over by
triage with the grade *unclear*: door `jigc validate`; the clause it is said to break `working-product`; its
repro block item 7 of *Left open* in
`verify-p3-r1-p3-unreadable-entry-undriven-platform-shapes-and-doors.a1.md`. Triage asked for: a staged bare
`git mv` of a managed doc, then both alternatives of `reconciliation.rename` as printed, both binaries,
nothing planted.

## Verdict

- **verdict:** `confirmed`.
- **regression:** `false` — the same block on the previous release, in rigs of its own, gives the same exit
  status in all 50 paired cells and the same bytes in every product output but one commit id. The door
  existed there; the cells are comparable; the defect is not new.
- **basis:** the route `jigc validate` prints under `reconciliation.rename` holds two commands; the first,
  `jigc rename <old-id> --to "<New Title>"`, exits 1 with `store.not-found` in every state that prints it and
  however the placeholder is filled (seven cells per binary), so a refusal's route does not work as printed.
- **contested:** `false`. The finding does not argue against a decision. The verdict rests on one reading of
  the clause's second half, stated under *Step 3* with what the other reading returns.
- **class:** `instance, unbounded`. I drove one doctype (`arch-doc`, a located doctype) in one rig state. I
  enumerated no consumer of the route and no other producer of it.
- **platform:** macOS 26.6.2 on arm64, git 2.54.0 (Apple Git-157), a user that is not root. Linux was not
  driven.

**What the drive found, in six lines.**

1. With nothing planted, after `git mv docs/architecture/padding-layer.md docs/architecture/renamed-layer.md`
   (staged, not committed), `jigc validate` exits 1 and prints one blocking `reconciliation.rename` whose
   route reads: *adopt it as a CLI-owned rename (re-points every referrer atomically):
   `jigc rename arch-doc:padding-layer --to "<New Title>"`; or revert the move: `git -C <REPO> mv
   docs/architecture/renamed-layer.md docs/architecture/padding-layer.md`*.
2. **The first command refuses.** Run exactly as printed (the placeholder left in), with the title of the
   new file, with another title, and with `--slug renamed-layer`: exit 1 each time, stdout empty, the same
   322 bytes on stderr — `blocking · store.not-found — no managed doc `arch-doc:padding-layer` to rename
   (expected at docs/architecture/padding-layer.md)`. Nothing moves, nothing is committed, and the next
   `jigc validate` prints the same finding at exit 1.
3. **The second command works as printed.** Taken from the output and run from a file: exit 0, the tree is
   clean again, and `jigc validate` exits 0 with *no findings — the committed store validates clean*.
4. **The first command refuses in the neighbouring states too**: after the installed hook has blocked the
   commit; when the move is a plain `mv` that is not staged; and when the move was committed past the hook.
   The route is printed in all three, and in all three `jigc rename` answers the same 322 bytes.
5. **The command itself is sound.** With no move, `jigc rename arch-doc:padding-layer --to "Renamed Layer"`
   exits 0 and commits the rename; and after the route's second command has reverted the move, the same call
   exits 0. So the only way through the first alternative is to take the second one first.
6. **All of it is the same on `1.0.0-rc.24`.**

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (`1.0.0-rc.24`).
- With the candidate's directory first on `PATH`, `command -v jigc` printed `<scratch>/bin/c1.a2/jigc`. The
  driver puts the directory of the binary a rig belongs to first on `PATH`, repeats that check before every
  cell and exits 97 otherwise; it never did. So the route's bare `jigc`, run as printed, is that binary.
- No `cargo build`; nothing under `target/` was driven. Every rig was built with
  `dev/jigc-rig vendored --binary <that binary>`, stdout captured alone, the two-step eval.
- The installed hook holds the absolute path of the binary that ran `jigc setup` — read in the hook cell's
  rig on each side: `jigc='<scratch>/bin/c1.a2/jigc'` and `jigc='<scratch>/bin/previous-91834b5e011d/jigc'`.
- The clone after the drives: branch `fix/canary-one`, `HEAD` 126a8531; `git status --porcelain` lists only
  untracked report files of this round under `completions/artifacts/canary-one/r1/reports/test/`, none of
  them mine. Nothing was built, edited, staged or committed there.

## Step 1 — driven from nothing

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p1-rename-route.MkwsVs` (written `<W>`). **Ten scenarios, each in a fresh rig, on each
binary — twenty rigs, 50 cells a side.** One more rig on the candidate was exploration (the output's shape,
and `jigc validate --format json`) and is not counted. No root of any other reporter was read or reused, and
nothing was planted in any rig.

Environment of every cell: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a
synthetic identity in the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables, `JIGC_PACK_DIR` unset, stdin
from `/dev/null`. Each cell: `git status --porcelain --untracked-files=all` and `HEAD`, the invocation with
stdout and stderr each to a file and **the exit status read from the command itself**, then the two again.
No pipe stands between a command and its status, and no output was cut.

**The two printed commands were never retyped.** After the `jigc validate` that prints the route, a small
script cuts the two backticked commands out of that stdout into two files, and the *as printed* cells run
`sh <file>`. The first file reads `jigc rename arch-doc:padding-layer --to "<New Title>"`; the second
`git -C <REPO> mv docs/architecture/renamed-layer.md docs/architecture/padding-layer.md`.

**What I changed from the finding's block:** nothing in its path — rig `vendored`, the same `git mv`, the
same `jigc rename … --to` with a title in the placeholder. I added the literal run of the first command, three
more fillings of it, three neighbouring states, and two controls.

### The exit matrix — the same on both binaries in every cell

| scenario (one rig each) | cell | exit | what it left |
|---|---|---|---|
| every scenario | `jigc validate` before the move | 0 | *no findings — the committed store validates clean* |
| s1–s6, s8, s9 | `git mv <old> <new>` | 0 | `R  <old> -> <new>` staged |
| s1–s6, s8, s9 | `jigc validate` after the move | **1** | the finding and its route, on stdout; stderr empty |
| s0 control | `jigc rename arch-doc:padding-layer --to "Renamed Layer"`, **no move** | 0 | one commit, the rename; `jigc validate` then 0 |
| s1 | **alternative 1 exactly as printed** (`--to "<New Title>"`) | **1** | `store.not-found`; tree and `HEAD` unchanged; `jigc validate` then 1 |
| s2 | alternative 1, `--to "Renamed Layer"` (the new file's own slug) | **1** | the same bytes; then `jigc describe` 0; `jigc validate` still 1 |
| s3 | alternative 1, `--to "Padding Layer Two"` | **1** | the same bytes |
| s4 | alternative 1, `--to "Padding layer" --slug renamed-layer` | **1** | the same bytes |
| s5 | **alternative 2 exactly as printed** | 0 | tree clean, `padding-layer.md` back; `jigc validate` then **0** |
| s6 | `git commit -m …` through the installed hook | 1 | *commit blocked*, `HEAD` unchanged |
| s6 | then alternative 1, `--to "Renamed Layer"` | **1** | the same bytes |
| s7 | a plain `mv <old> <new>`, not staged; `jigc validate` | 1 | the same finding and route |
| s7 | then alternative 1, `--to "Renamed Layer"` | **1** | the same bytes |
| s8 | `git commit --no-verify` of the move; `jigc validate` | 0; 1 | the move committed; the same finding and route |
| s8 | then alternative 1, `--to "Renamed Layer"` | **1** | the same bytes |
| s9 | alternative 2 as printed, **then** `jigc rename arch-doc:padding-layer --to "Renamed Layer"` | 0; 0 | one commit, the rename (`R093`); `jigc validate` then 0 |

Tally per binary: 28 cells at exit 0, 22 at exit 1. Seven cells per binary run the first alternative; their
stderr is byte-identical across all fourteen.

### The outputs the verdict rests on, whole

`jigc validate` after the staged move — stdout, exit 1, stderr empty (`<REPO>` for the rig's repository):

    note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the same break can still gate at a task or milestone door; the trailer below says which, where a gate exists.
    advisory · file-state.un-baselined — committed doc `docs/architecture/renamed-layer.md` is not yet baselined in the file-state record
      at: docs/architecture/renamed-layer.md
      route: no action needed — the doc is baselined on its next author or finalize
    blocking (gates at finalize) · reconciliation.rename — tracked managed doc arch-doc:padding-layer (docs/architecture/padding-layer.md) is missing; docs/architecture/renamed-layer.md has the same content hash — likely renamed via `git mv`
      at: docs/architecture/padding-layer.md
      route: adopt it as a CLI-owned rename (re-points every referrer atomically): `jigc rename arch-doc:padding-layer --to "<New Title>"`; or revert the move: `git -C <REPO> mv docs/architecture/renamed-layer.md docs/architecture/padding-layer.md`
    out-of-band rename detected — a structural-identity change this commit introduced; the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
    — jigc · run `jigc start` for orientation; all writes through `jigc`.

The first alternative — stderr, exit 1, stdout empty, in all seven cells:

    blocking · store.not-found — no managed doc `arch-doc:padding-layer` to rename (expected at docs/architecture/padding-layer.md)
      at: arch-doc:padding-layer
      route: `jigc describe` lists the doctype surface — check the id you typed against it
    — jigc · run `jigc start` for orientation; all writes through `jigc`.

The hook, on the commit that stages the move — stderr, exit 1:

    jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).

## Step 2 — is it what the finding says

**Yes, in each particular.** The finding says: after a staged bare `git mv` of a managed doc `jigc validate`
prints `reconciliation.rename` with two alternatives; the second works; the first, with a title in the
placeholder, exits 1 with `store.not-found — no managed doc arch-doc:padding-layer to rename (expected at
docs/architecture/padding-layer.md)`, routed to `jigc describe`; with no plant. Each is what I observed.

The ways it could have been wrong, and what each returned:

- **Stale state.** Every rig was minted for this report; each scenario opens with a `jigc validate` that
  exits 0 clean, so the finding is caused by the move and nothing before it.
- **A pipe or a cut.** Exit statuses read from the command, outputs read whole from files.
- **Another binary.** Both hashes asserted first; the `PATH` check before every cell; the hook's own binary
  read from the hook.
- **The filling of the placeholder.** Four fillings, the literal one included. The refusal names the old
  home, not the title, and is the same bytes in all four.
- **The state.** Staged; refused by the hook; not staged; committed. The same in all four. The refusal is
  raised where `jigc rename` finds no entry at the old id's home — which is the condition the finding is
  printed under (`crates/cli/src/rename.rs`, the `HomeEntry::Free` arm; `crates/engine/src/file_state.rs`,
  `detect_rename`, which runs only when the recorded path is absent). I read that, I did not enumerate from
  it: one doctype was driven.
- **A broken verb.** The two controls: with no move, and after the revert, the same call exits 0.
- **A settled decision that intends it.** None does, and the design says the opposite.
  `design/reconciliation.md` → *Rename detection* gives the strong signal's resolution as *routing to the
  owned op first, revert second*, with `jigc rename adr:rate-limit --to "Gateway rate limit"` as the first
  command, and calls it *the actionable "adopt as `jigc rename`" route*. `design/write-commands.md` →
  `jigc rename`, step 2, has the verb validate up front that the *target exists*, and
  `design/reconciliation.md` says of the verb that a re-run *keys on `<old>`, which the `git mv` already
  destroyed*. The two statements cannot both hold in the state the finding is printed in; no entry of
  `DECISIONS.md` chooses between them. `design/surface-contract.md` → *The route fence (law 2)*: *A
  mechanical route that hard-rejects, or repairs the wrong thing, is law 2 failing one level down* — this
  route holds two commands and so is not constructed through the fenced constructor, but the rule is the
  design's own account of such a route.
- **A row already ruled.** The same defect is row `(R3, F1)` of the review of `1.0.0-rc.24`
  (`completions/artifacts/M55/per-axis-review-rc24/README.md`: *the strong-signal route's adopt exit cannot
  be run*, tier 3), and the fix pass's ledger carries it as `OPEN — port`
  (`completions/artifacts/M55/fix-pass-rc25/findings-ledger.md`). An open row is not an intention; and the
  entry that revised the exit rule says of that tier: *The review's and the trial's tier-2 and tier-3 rows
  have not been graded against the revised rule.* So no ruling stands on whether it breaks a clause. I read
  those two records for their disposition only, after my own cells were driven.

So: real, as stated, not intended.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, in `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: "We have a working product others can
use and relay on", its instrument "**no command that works on rc.24 in a supported layout stops working, and
every refusal's route works as printed**", and the rule "A finding blocks the 1.0.0 call only if it breaks a
clause inside its scope; everything else is recorded with its tier." The run's opening takes the closing
condition by reference to that entry and declares no bound.

**First half — no command that works on rc.24 stops working: not broken.** 50 paired cells, 50 the same exit
status. After replacing each rig's root, 49 pairs are byte-identical in stdout, stderr, the tree's status and
the directory listing; the one that differs is the seven-character commit id in `git commit`'s own line.

**Second half — every refusal's route works as printed: broken.**

- *It is a refusal.* `jigc validate` exits 1 on this finding and says so (*the sweep exits non-zero*); the
  finding is graded *blocking (gates at finalize)*; and the installed hook refuses the commit that stages
  the move (*commit blocked*).
- *It is a route, printed.* The line opens `route:` and holds two commands in backticks.
- *It does not work as printed.* The first command, the one the route puts first and recommends
  (*re-points every referrer atomically*), exits 1 in the very state the refusal diagnoses, with every
  filling of its one placeholder. The trailer names the same exit (*or adopt it via `jigc rename`*), and the
  hook names it alone (*use `jigc rename` instead*).
- *It is inside the scope.* A stock rig state, ordinary git configuration, no plant, no second fault: one
  `git mv`, which is the event this finding exists to detect.

*The reading this verdict rests on.* *Works as printed* is read of each command a route prints, in the state
its refusal diagnoses. That is the reading the report this finding came from stated for a neighbouring pair
of routes (a property *of a route in the state its refusal diagnoses*, and no promise about a second,
independent fault); here there is no second fault to set aside.

*The other reading, and what it returns.* If a route with two alternatives works whenever one of them does,
this route works: the revert runs as printed, clears the finding, and the refusal is no dead end. Under that
reading the verdict is `refuted`, basis `breaks-no-clause`, and the row stays in the ledger as a route that
names an exit which refuses. Which reading the clause carries is a ruling on what its second half reaches; I
took the plainer one — a printed command that cannot succeed has not worked as printed — and it is said
here so that the other is not re-derived.

**Verdict on the clause: `working-product` is broken inside its scope, by its second half.** `confirmed`.

## Step 4 — the regression fact

The same ten scenarios, in ten fresh rigs, with the previous release's binary by its absolute path
(`<scratch>/bin/previous-91834b5e011d/jigc`, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d).
The door exists there and prints the same route. **Red there, red on the candidate: `regression: false`.**
The pairing is `<W>/runs/compare.txt`: `pairs=50 same_rc=50 byte-identical=46` — of the four that differ,
three are my own argv files, which hold the path of the route file under `runs/cand` or `runs/prev`, and the
fourth is the commit id above.

## Step 5 — coverage

The finding makes no coverage claim, and I establish none. Read on the way, and no enumeration:
`crates/cli/tests/flow37_rename.rs` has one test that commits a bare `git mv`
(`store_scope_oob_rename_flips_exit_and_report_only`) and asserts the finding's code, the exit and
`report_only` — it does not run either command of the route. The block below says `UNPINNED`.

## The repro block

### Repro V-1 — the route's first command refuses

```yaml
claim: "with nothing planted, after a staged bare `git mv` of a managed doc, `jigc validate` prints `reconciliation.rename` with a route of two commands, and the first — `jigc rename <old-id> --to \"<New Title>\"` — refuses with `store.not-found` however its placeholder is filled; that breaks the closing condition's `working-product` clause (ledger key r1-p4-reconciliation-rename-route-jigc-rename-not-found)"
verdict: CONFIRMED
regression: false           # red on the previous release too — 50 paired cells, the same exit in each
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exit, stdout and stderr in every cell"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), not root; Linux NOT driven"
env: "HOME a fresh directory; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1; a synthetic identity; stdin /dev/null"
setup:
  - fixture: vendored                       # spec:padding and arch-doc:padding-layer, committed; no task in flight
  - ["git", "mv", "docs/architecture/padding-layer.md", "docs/architecture/renamed-layer.md"]
repro:
  - ["jigc", "validate"]
  - ["jigc", "rename", "arch-doc:padding-layer", "--to", "Renamed Layer"]     # the route's first command, its placeholder filled
  - ["jigc", "validate"]
observed:                                   # the red state, both binaries
  - { exit: 1, stderr: "", stdout_contains: "route: adopt it as a CLI-owned rename (re-points every referrer atomically): `jigc rename arch-doc:padding-layer --to \"<New Title>\"`; or revert the move: `git -C " }
  - { exit: 1, stdout: "", stderr_contains: "store.not-found — no managed doc `arch-doc:padding-layer` to rename (expected at docs/architecture/padding-layer.md)", after: "HEAD unchanged; the staged rename unchanged" }
  - { exit: 1, stdout_contains: "reconciliation.rename" }
expect:                                     # the test in waiting — it does not choose the fix
  - "every command the `reconciliation.rename` route prints in backticks, cut out of the output and run as printed with each placeholder filled, exits 0"
  - "and after any one of them `jigc validate` exits 0 and prints no `reconciliation.rename`"
same-refusal-with: "--to \"<New Title>\" left as printed · --to \"Padding Layer Two\" · --to \"Padding layer\" --slug renamed-layer · after the hook has blocked `git commit` · after a plain `mv` that is not staged · after `git commit --no-verify` of the move — each in a rig of its own"
control:
  - "the route's second command, as printed — `git -C <REPO> mv docs/architecture/renamed-layer.md docs/architecture/padding-layer.md`: exit 0; `jigc validate` then exits 0, `no findings — the committed store validates clean`"
  - "no move at all: `jigc rename arch-doc:padding-layer --to \"Renamed Layer\"` exits 0, one commit"
  - "the second command, then that rename: exit 0 and exit 0, one commit, `jigc validate` exits 0"
clause: "working-product — BROKEN by its second half (every refusal's route works as printed); its first half is not broken (nothing differs from the previous release)"
observed-at: "<W>/runs/{cand,prev}/<scenario>/<nn>-<cell>.{argv,rc,out,err,porc.before,porc.after,head.before,head.after,ls.after}; <W>/runs/{cand,prev}/<scenario>/route-alt{1,2}.sh; <W>/runs/{cand,prev}.log; <W>/runs/compare.txt"
pinned-by: "UNPINNED: no test runs a command of this route — not enumerated; one suite read (flow37_rename), which asserts the finding and not its route"
```

**Pinnable as it stands: yes, with three things a converter needs to know.**

1. **The assertion must not choose the fix.** Two repairs satisfy the clause — the verb adopts a move that
   has already happened, or the route stops printing a command that refuses — and they are a design choice
   between the two sentences of `design/reconciliation.md` quoted in *Step 2*. So the test cuts the commands
   out of the route it is given, rather than hard-coding `jigc rename`; the `observed` rows are today's red
   and would pin the defect if copied as expectations.
2. **The placeholder needs a filling.** `<New Title>` is a title; any title that slugs is a fair one, and the
   block's is the moved file's own.
3. **The revert holds an absolute path** — the canonical one of the repository — so it is run from the
   output, never matched as text. The whole block is one platform's; nothing in it is known to depend on
   that.

## What was driven, and what was not

- **Driven, on both binaries:** rig `vendored`, the located doctype `arch-doc`, one doc with no referrer;
  the move staged, staged and then refused by the hook, not staged, and committed past the hook; the
  route's first command in four fillings; its second command as printed; the verb with no move, and after
  the revert.
- **Not driven:** Linux. Any other doctype — a placement singleton, a fixed-identity doctype, a doc that has
  referrers. The task-scope door (`jigc task validate`, `jigc task finalize`) that prints the same finding.
  The route's second command in the not-staged and committed states. `jigc validate --format json` on the
  previous release (on the candidate it carries the same route string, in the exploration rig).
- **Not established:** that every producer of this route is this one; that no suite pins the route.
- **Slips of mine:** none that touched a cell. The helper scripts were written through the shell into
  `<W>/tools/`; the one file my file tool wrote is this report. I asked the record script for the run's
  state once and read only the head of what it printed — the list of ledger keys; I used nothing of it.
- **Nothing was fixed, graded or decided.**

## Left open — seen on the way, not pursued

1. **The second refusal's own route does not fit the state it is printed in.** `store.not-found` at
   `jigc rename` routes to *`jigc describe` lists the doctype surface — check the id you typed against it*.
   The id typed is the one the first route printed; `jigc describe` exits 0, names neither the old nor the
   new slug, and `jigc validate` afterwards is unchanged at exit 1. Both binaries.
2. **The hook names only the exit that refuses.** Its one line says *use `jigc rename` instead (commit
   blocked)* and prints no revert; in the blocked state that command exits 1 (scenario s6). Whether this is
   a second surface of this row or a row of its own is triage's.
3. **The revert for a move that was never staged — not driven.** After a plain `mv` the finding still reads
   *likely renamed via `git mv`* and prints the same `git -C <REPO> mv <new> <old>`. I did not run it there.
4. **The revert after the move was committed — not driven.** Scenario s8 prints the same route; only its
   first command was run.
5. **This row has an older name.** It is `(R3, F1)` of the review of `1.0.0-rc.24`, `OPEN — port` in the fix
   pass's ledger, and by the revised rule's own sentence never graded against a clause. The run's ledger
   holds it under this key only.
6. **Linux** — driven by nobody for this row.

## Where the evidence is

`<W>` is `<scratch>/verify-p1-rename-route.MkwsVs`. Per rig `<W>/r/<scenario>-<cand|prev>/<rig root>`; per
cell `<W>/runs/<cand|prev>/<scenario>/<nn>-<cell>.{argv,rc,out,err,porc.before,porc.after,head.before,head.after,ls.after}`,
beside them `route-alt1.sh`, `route-alt2.sh`, `repo.path` and `rig.err`. The logs: `<W>/runs/cand.log`,
`<W>/runs/prev.log`; the pairing: `<W>/runs/compare.txt`. The tools: `<W>/tools/{drive.sh,runall.sh,compare.py}`.
The exploration rig: `<W>/explore/`. Nothing was torn down.

<!-- end of report -->
