# verify-real — `r1-p4-nothing-staged-route-git-add-fails-unreadable-file` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-nothing-staged-route-git-add-fails-unreadable-file`. One finding, handed over by
triage with the grade *unclear*: door `jigc task finalize`, the clause it is said to break `working-product`,
its repro block `Repro V-3c` of `verify-p3-r1-p3-unreadable-entry-class-door-list-traced-by-hand.a1.md`
(*Left open*, item 2). Triage asked for: `Repro V-3c` as written, on both binaries, and whether the state
needs the mode-000 plant.

## Verdict

- **verdict:** `confirmed`.
- **regression:** `false` — a fact: the block is red on the candidate **and** red on `1.0.0-rc.24`, the same
  exit statuses and, commit ids aside, the same bytes in every cell (189 of 189 files).
- **basis:** `finalize.nothing-staged` prints the route *`git add` your changes, then re-run `jigc task
  finalize`*; where the only dirty path is one `git add` refuses, the route's command exits 128 in every
  spelling driven and the re-run prints the same refusal with the same route — the second half of the
  clause's instrument (*every refusal's route works as printed*) is not met, and no declared bound of this
  run covers the state.
- **contested:** `false`. The finding argues against no settled decision; the design's own reasoning about
  this code runs the same way (*Step 2*). The reading of the clause the verdict rests on is stated under
  *Step 3* so that it can be overruled.
- **Does the state need the mode-000 plant? No.** The plant is one way into it. An untracked directory that
  holds a git repository with no commit (`git init sub`, nothing else) reaches the same three exits — 3, 128,
  3 — with no permission bit touched, on both binaries. What the state needs is that **every** dirty path
  `git status` lists is one `git add` refuses: one addable file beside the plant, and the route delivers.
- **class:** `instance, unbounded`. Driven: two causes of a refused `git add` (a file nobody may read, at two
  places; a commit-less nested repository), three spellings of the route's command, one fixture, one workflow.
  The causes for which `git add` refuses a path were not enumerated.
- **platform:** **macOS only** — macOS 26.6.2 on arm64, git 2.54.0 (Apple Git-157), a user that is not root.
  Nothing was driven on Linux; a mode-000 file refuses nobody who may read any file, so the plant's arm is not
  reachable as root. The nested-repository arm does not depend on who runs it, and was not driven as root
  either.

**What the drive found, in five lines.**

1. **`Repro V-3c` reproduces as written, on both binaries**: `jigc task finalize` exits 3 under
   `finalize.nothing-staged`; `git add scratch.bin` exits 128, *unable to index file*; the re-run exits 3 with
   the same three lines. The same with `docs/research/locked.md` in place of `scratch.bin`.
2. **Both of the block's controls hold**: a readable `scratch.txt` — 3, then `git add` 0, then finalize 0 with
   one commit; no untracked file at all — 3 under `finalize.empty-commit`, another refusal with another route.
3. **No spelling of the route's command delivers in that state**: `git add scratch.bin`, `git add -A` and
   `git add .` each exit 128 with the same three lines of git's.
4. **The route delivers as soon as one addable change exists**: with a readable `scratch.txt` beside the
   mode-000 `scratch.bin`, `git add scratch.txt` exits 0 and the finalize commits, naming `scratch.bin` as
   left out.
5. **The mode-000 plant is not needed**: an empty `git init sub` in the work tree gives 3, 128, 3 the same
   way.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (`1.0.0-rc.24`).
- Before every sequence the driven binary's directory went first on `PATH` and `command -v jigc` was held to
  `<that directory>/jigc`; the rig's `$JIGC` was held to the same file (the driver exits 97 otherwise; it never
  did). No `cargo build`; nothing under `target/` was driven. Every rig was built with
  `dev/jigc-rig refs-post-hoc --binary <that binary>`, stdout captured alone, the build's status read before
  the `eval`.
- The clone, after the drives: branch `fix/canary-one`, `HEAD` 126a8531; `git status --porcelain` lists only
  untracked report files under `completions/artifacts/canary-one/r1/reports/test/`, none of them mine.
  Nothing was built, edited, staged or committed there.

**What I read, said plainly.** The one report my prompt names, whole — it is where the block is. The run's
opening record. Of the run's state, this finding's own ledger row and the list of declared bounds (empty).
No other verifier's report and nothing of triage's file. The helper scripts are in my own scratch directory
and were written through the shell; the file tool wrote this report and nothing else.

## Step 1 — driven from nothing

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p4-nsroute.MBuPXm` (written `<W>`). Every root is under `<W>/roots`, minted by
`dev/jigc-rig` (`SCRATCH` pointed there). **One root per variant, per binary — 18 rigs**; no root of any other
reporter was read or reused.

Environment of every invocation: `HOME` the rig's own `home/`, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four `GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables, stdin
from `/dev/null` or from a file where the argv reads `-`. Around each invocation:
`git status --porcelain --untracked-files=all` before; the invocation, stdout and stderr each to a file,
**the exit status read from the bare command**; the status again, and `HEAD` with the commit count. No pipe
stands between a driven command and its exit status, and no output was cut.

**The sequence**, the block's own, in every rig: the plant; `jigc start --workflow do-research "study the
plant"`; `jigc doc set-field commit:study-the-plant#header/type --value docs --task study-the-plant`;
`jigc doc set-slot commit:study-the-plant#summary --from-file - --task study-the-plant` (stdin `record
nothing`); then `jigc task finalize study-the-plant`, the route's `git add`, `jigc task finalize
study-the-plant` again — and, added by me as a seventh cell, the same finalize with `--format json`. The three
setup cells exit 0 in all 18 rigs. Before the plant, `git status` of the rig is empty in all 18.

**Nothing in the block had to be changed.** What I added is named: the seventh cell, and five variants
beside the block's four.

| variant | the plant | the route's command, as driven | finalize | `git add` | finalize again | the same, `--format json` |
|---|---|---|---|---|---|---|
| X — the block | `printf 'x\n' > scratch.bin && chmod 000 scratch.bin`, at the repository's root | `git add scratch.bin` | **3** | **128** | **3** | 3 |
| M — the block's *the-same-with* | `printf '# x\n' > docs/research/locked.md && chmod 000 …` | `git add docs/research/locked.md` | **3** | **128** | **3** | 3 |
| R — the block's control | a readable `scratch.txt` | `git add scratch.txt` | 3 | 0 | **0** | 1 (`finalize.no-task`: the task is done) |
| N — the block's control | none | `git add -A` | 3 (`finalize.empty-commit`) | 0 | 3 (the same) | 3 |
| XA — mine | as X | `git add -A` | 3 | **128** | 3 | 3 |
| XD — mine | as X | `git add .` | 3 | **128** | 3 | 3 |
| XR — mine | as X, and a readable `scratch.txt` | `git add scratch.txt` | 3 | 0 | **0** | 1 (`finalize.no-task`) |
| G — mine | `git init -q sub` — no file, no commit, no mode change | `git add sub` | 3 | **128** | 3 | 3 |
| GA — mine | as G | `git add -A` | 3 | **128** | 3 | 3 |

**The previous release has the same exit status in every cell of this table.** Compared file by file —
exit status, stdout and stderr of the seven cells of each variant, 189 files, after replacing each root's name
and the abbreviated commit id of the two variants that commit — **189 are identical, none differs.**

**What the cells print**, candidate (and, byte for byte, the previous release):

- The first finalize under X, M, XA, XD, XR, G and GA — stdout empty, stderr:

  ```text
  blocking · finalize.nothing-staged — you staged nothing — the working tree has changes but the index is empty
    at: task:study-the-plant
    route: `git add` your changes, then re-run `jigc task finalize`
  — jigc · run `jigc start` for orientation; all writes through `jigc`.
  ```

- `git add`, under X, XA and XD — stdout empty, stderr:

  ```text
  error: open("scratch.bin"): Permission denied
  error: unable to index file 'scratch.bin'
  fatal: adding files failed
  ```

  under M the same three lines with `docs/research/locked.md`; under G and GA:

  ```text
  error: 'sub/' does not have a commit checked out
  error: unable to index file 'sub/'
  fatal: adding files failed
  ```

- The second finalize under X, M, XA, XD, G and GA: the four lines of the first, unchanged. `--format json`:
  the findings envelope on stdout, one finding, `"code": "finalize.nothing-staged"`, keyed at
  `task:study-the-plant`, its `route` the same sentence; stderr empty.
- Under R, after `git add scratch.txt`: *no findings — the task validates clean*, then `finalized <id> —
  docs: record nothing`, `added scratch.txt`, `1 file committed`.
- Under XR, the same commit, and `left-out (unstaged/untracked — git add to include): scratch.bin` printed
  before the commit and after it.
- Under N: `finalize.empty-commit — task validated but produced no diff — nothing to finalize`, routed *make a
  change, then re-run `jigc task finalize study-the-plant` — or, if the task is done with nothing to show,
  abandon it with `jigc task discard study-the-plant --force`*.

**After the sequence**, under X, M, XA, XD, G and GA: `HEAD` is where the rig left it (six commits), `git
status` lists the one untracked path it listed before, and the plant stands — `scratch.bin` mode 0 and 2
bytes, `locked.md` mode 0 and 4 bytes (`cat` of either exits 1, *Permission denied*, read before the
sequence), `sub/` a directory holding its `.git`.

## Step 2 — is it what the finding says

The finding says: `finalize.nothing-staged` routes at `git add`, and over an untracked file that cannot be
read the route does not deliver. **That is what the binary does**, and the block's exits, its two stderr
fragments, its *the-same-with* arm and both of its controls are as written.

- **Against stale state, a pipe, a cut, another binary.** Every root was minted for this report; each variant
  has a rig of its own per binary; exit statuses are read bare and outputs are whole files; both hashes were
  asserted first and `PATH` held before each sequence.
- **Against an artefact of the rig.** The rig's own live task is not what trips the refusal: the control with
  no untracked file answers `finalize.empty-commit`, so the one untracked path is what turns that refusal into
  `finalize.nothing-staged`.
- **Against the spelling of the route.** The route names no path, so the block's `git add scratch.bin` is one
  reading of it. The two others a user would type, `git add -A` and `git add .`, fail the same way.
- **Against a settled decision.** `design/finalize.md` → *Dirty-tree policy*: "**Block on nothing staged.** If
  the narrowed commit set is empty while the tree is dirty, `finalize` blocks (*"you staged nothing — `git add`
  your changes"*)", and → *Commit-doc rendering*, the paragraph *Empty commit*: the message is selected "by
  *why* it is empty: a **dirty tree with nothing staged** blocks with *"you staged nothing — `git add` your
  changes"* (the agent has work but never staged it)". So the refusal firing over a dirty tree is intended,
  and so is its route's text. **Neither sentence says what is owed where the dirty path is one `git add`
  refuses**, and in the one place the design meets a state in which `git add` cannot deliver, it withdraws
  this code: → *The doc-only arm*, "The ordinary model's recolor of `finalize.empty-commit` into
  `finalize.nothing-staged` is skipped here: that finding routes at `git add`, and no `git add` can bring a
  path into a path-scoped commit, so an empty doc-only task keeps `finalize.empty-commit`." The source says
  the same of itself (`crates/cli/src/task.rs`, the comment at the recolor, and `nothing_staged_finding`).
  **Not intended**; the finding asks for nothing the design has ruled out, so it is not contested.
- **What the code asks**, read to say why the state is what it is: the recolor fires when the plan answers
  `finalize.empty-commit` and `git_dirty_paths` — `git status --porcelain --untracked-files=all` — lists
  anything. It asks whether a path is dirty, never whether git can index it.

## Step 3 — does it break `working-product`, inside the clause's scope

The clause, `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: "We have a working product others can use
and relay on"; its instrument, "no command that works on rc.24 in a supported layout stops working, and every
refusal's route works as printed"; and the rule, "A finding blocks the 1.0.0 call only if it breaks a clause
inside its scope; everything else is recorded with its tier." The run's opening names the clause
`working-product` and declares no bound; the state's list of bounds is empty.

**First half — no command that works on rc.24 stops working.** Not broken. Every cell is the previous
release's, byte for byte. Nothing that works there stops working.

**Second half — every refusal's route works as printed.** **Broken, in the instance.** A refusal was printed;
its route was run as printed, in the state that refusal was printed in, at the door the finding names, in
three spellings; the command exits 128, and the re-run the route ends with prints the same refusal and the
same route. The user is handed `git add` by jigc and a refusal by git, in a loop no step of the route leaves.

**Inside the scope.** Three things were tried against it, and none holds:

1. **A declared bound.** *Deliberately planted states* are named as declared bounds in the rule's first
   sharpening — of the first clause, and as bounds that are "written down with their reach". This run declares
   none, and a bound is the human's to declare, never a verifier's. I did not excuse the mode-000 file as a
   plant.
2. **The plant itself.** The state does not need it. The nested-repository arm reaches the same loop with
   nothing a user could not do by typing `git init` in a subdirectory.
3. **A supported layout.** The phrase stands in the instrument's first half. Where the design has met the
   question for the second, it reads it without that qualifier: `design/validation.md`, of a layout the human
   ruled **unsupported**, says "What the layout is owed instead is what every refusal owes: no byte is lost at
   any of the three doors, and each route works as printed" — and records that `finalize.stage-failed`'s route
   was widened there to name a second cause.

*The reading this rests on, said so that it can be overruled.* *Works as printed* is asked of a route in the
state its refusal was printed in. On a narrower reading — a route is held only to the state its author had in
mind, here *the agent has work but never staged it* — the route works wherever there is addable work (R, XR),
and this finding would be `refuted`, basis `breaks-no-clause`, a real defect with a tier. Nothing in the
rule's text chooses the narrower reading, and the verifier of the report this finding grew from stated the
wider one; I kept it.

**The reach, as a fact for whoever rules on it — not a grade.** The loop needs a task whose finalize has
nothing of its own to commit (no doc that promotes, nothing staged) in a tree whose every dirty path is one
`git add` refuses. A task that has any addable change is not in it: the route delivers, and the path git
refuses is named as left out. No byte is lost or written in any cell; `HEAD` does not move. The exits a user
has are git's own message (make the path addable, remove it, ignore it), after which the same task would
answer `finalize.empty-commit` with a route of its own — that refusal was driven as the control N, in a rig
that never held a plant, and not as a continuation of X.

**Verdict on the clause: `working-product` is broken in its instrument's second half by this instance**, and
by nothing in its first. `confirmed`, `regression: false`.

## Step 4 — the regression fact

The same block, each variant on a rig of its own built with the previous release's binary by its absolute
path (`<scratch>/bin/previous-91834b5e011d/jigc`, sha256
accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d): **red there too** — 3, 128, 3 under X, M,
XA, XD, G and GA; the controls R, N and XR as on the candidate. The door and the code both exist on that
release, so the two are comparable. **`regression: false`.**

## Step 5 — the coverage, for the block's `pinned-by`

The finding's block says `UNPINNED: not searched`. Searched, in the tree as checked out — 126a8531, which
differs from the candidate's commit eeffe347 in the run's records only (`git diff --stat` between the two over
`crates`, `tooling-tests`, `design` and `dev` is empty): every file under `crates/cli/tests` and
`tooling-tests` that spells `nothing-staged`, `nothing_staged` or *staged nothing* — **38 files, 26 of them
suites and 12 compose goldens**. In the 26, every line that sets a mode (`from_mode`, `set_mode`,
`set_permissions`, a spawned `chmod`) was listed and read: 18 call sites and one helper, each making a hook, a
shim or a script executable (0o755) or a directory read-only (0o555) and restoring it. **None takes read
permission from an untracked file.** And no file under `crates/cli/tests`, `tooling-tests`, `crates/cli/src`
or `crates/engine/src` spells git's *unable to index*, *adding files failed* or *does not have a commit
checked out*. So no suite was found that runs this refusal's route in a state where `git add` refuses. Two
bounds of that search: what the 26 suites assert of the code otherwise was not read test by test, and a suite
that plants a nested repository without spelling git's refusal would not have been found by it.

## The repro block

### Repro V-P4

```yaml
claim: "`finalize.nothing-staged` routes at `git add`; where every dirty path is one `git add` refuses, the route's command fails and the re-run repeats the refusal (ledger key r1-p4-nothing-staged-route-git-add-fails-unreadable-file)"
verdict: CONFIRMED          # regression: false — the same on the previous release
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exits and, commit ids aside, the same bytes: 189 of 189 files over the nine variants"
platform: "macOS 26.6.2 arm64, git 2.54.0 (Apple Git-157), not root; Linux NOT driven"
setup:            # HOME a fresh directory; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1
  - fixture: refs-post-hoc
  - ["sh", "-c", "printf 'x\\n' > scratch.bin && chmod 000 scratch.bin"]     # at the repository's root
  - ["jigc", "start", "--workflow", "do-research", "study the plant"]
  - ["jigc", "doc", "set-field", "commit:study-the-plant#header/type", "--value", "docs", "--task", "study-the-plant"]
  - ["jigc", "doc", "set-slot", "commit:study-the-plant#summary", "--from-file", "-", "--task", "study-the-plant"]   # stdin: "record nothing"
repro:
  - ["jigc", "task", "finalize", "study-the-plant"]
  - ["git", "add", "scratch.bin"]
  - ["jigc", "task", "finalize", "study-the-plant"]
expect:           # what the binary does today — the red side
  - exit: 3
    stdout: ""
    stderr_contains: "finalize.nothing-staged — you staged nothing — the working tree has changes but the index is empty"
    stderr_contains_also: "route: `git add` your changes, then re-run `jigc task finalize`"
  - exit: 128
    stderr_contains: "error: unable to index file 'scratch.bin'"
  - exit: 3
    stderr_contains: "route: `git add` your changes, then re-run `jigc task finalize`"
  - after: "HEAD unchanged; `git status --porcelain --untracked-files=all` is `?? scratch.bin`; scratch.bin still mode 000 and 2 bytes"
the-same-with:
  - "docs/research/locked.md (mode 000) in place of scratch.bin — 3, 128, 3"
  - "`git add -A` or `git add .` in place of `git add scratch.bin` — 3, 128, 3"
  - "NO mode change: `git init -q sub` in place of the plant, then `git add sub` or `git add -A` — 3, 128 (`error: 'sub/' does not have a commit checked out`), 3"
  - "`jigc task finalize study-the-plant --format json` as the last cell — exit 3, one finding on stdout, the same code and the same route"
controls:
  - "a readable scratch.txt in place of the plant: 3, `git add scratch.txt` 0, finalize 0 — `added scratch.txt`, `1 file committed`"
  - "a readable scratch.txt BESIDE the mode-000 scratch.bin: 3, `git add scratch.txt` 0, finalize 0 — the commit, and `left-out … scratch.bin`"
  - "no untracked file at all: 3 `finalize.empty-commit`, routed at a change or at `jigc task discard study-the-plant --force`"
clause: "working-product — the instrument's second half (every refusal's route works as printed) is not met in this state; its first half is: nothing differs from the previous release"
observed: "<W>/out/{X,M,R,N,XR,XA,XD,G,GA}-{cand,prev}/"
pinned-by: "UNPINNED: of the 26 suites that name the code, none takes read permission from an untracked file; no test spells git's `unable to index`"
```

**Pinnable as it stands: yes** — a named fixture state, one filesystem call, literal argv, exit statuses and
fragments of stderr. What a converter needs to know:

1. **The `expect` list is the red side.** It asserts what the binary does now. What the green side is — a
   route that delivers, another refusal, the paths named — is the fixer's to derive and nobody's ruling yet;
   this block does not choose it.
2. **The mode-000 arm is Unix-only and not reachable as root**, a container that runs as root included. **The
   nested-repository arm has neither limit** and is the form to pin where the suite may run as root; it leans
   on git's own refusal of a repository with no commit, driven here on git 2.54.0 only.
3. **`git add`'s exit status and wording are git's.** A test should hold jigc's two cells to their bytes and
   the middle cell to *non-zero, nothing staged*.
4. **The controls are facts a fix must not break** and pin as they are.
5. The fixture is larger than the state needs: the refusal turns on one untracked path and a task with nothing
   of its own to commit. A smaller fixture was not driven.

## What was driven, and what was not

- **Driven:** the block as written, its *the-same-with* arm and both of its controls; two more spellings of
  the route's command; an addable file beside the plant; a commit-less nested repository in place of the
  plant, in two spellings; the `--format json` form of the second refusal. Each on both binaries, each in a
  rig of its own: 18 rigs, 126 invocations, 63 on each binary.
- **Not driven:** Linux, and root. Any other cause of a refused `git add` — a line-ending refusal under
  `core.safecrlf`, an index lock, a tracked file edited and then made unreadable, a nested repository that
  holds files. Any workflow but `do-research`, and any fixture but `refs-post-hoc`. A linked or a fan-out
  worktree. `jigc task finalize --dry-run` and `jigc task validate` in the state. The other caller of
  `nothing_staged` the source shows (`crates/cli/src/rename.rs`). The continuation in which the plant is
  removed and the same task is finalized again.
- **Not established:** how many states make `git add` refuse a path `git status` lists — the class is
  `instance, unbounded`.
- **Nothing was fixed, graded or decided.**

## Left open — seen on the way, not pursued

1. **The loop is reached without any permission plant** — an untracked directory holding a git repository
   with no commit (`git init sub`). It is reported here as the answer to triage's question and is part of this
   finding's block; whether it is a row of its own is triage's.
2. **The refusal names no path.** `finalize.nothing-staged` says the tree *has changes* and lists none of the
   paths it counted, so the user learns which path is in the way from git's refusal and not from jigc's. Both
   binaries.
3. **`jigc task finalize <task> --format json` over a task that is gone** prints its `finalize.no-task`
   envelope on **stderr** and exits **1**, with stdout empty — where the `finalize.nothing-staged` envelope of
   the same verb is on stdout at exit 3. Seen in the last cell of R and XR, both binaries; not held against any
   design text here.
4. **Other causes of a refused `git add`** were not driven (the list under *Not driven*).
5. **Linux and root** — driven by nobody for this finding.

## Where the evidence is

`<W>` is `<scratch>/verify-p4-nsroute.MBuPXm`. Per variant and binary:
`<W>/out/<variant>-<cand or prev>/<NN>-<label>.{argv,rc,stdout,stderr,porc.before,porc.after,head.after}`,
with `root` (the root it ran in), `rig.stderr` (the rig's construction log), `00-porc.rig`,
`00-ls.after-plant`, `00-cat.{rc,stdout,stderr}` where a file was planted, `final.log`
(`git log --name-status`) and `final.ls`. The roots: `<W>/roots/`. The tools:
`<W>/tools/{drive.sh,compare.py}`, with `rig-print.txt` (the rig's construction, printed and not run) and
`record-help.txt`. Nothing was torn down, and the one file written for the repository is this report.

<!-- end of report -->
