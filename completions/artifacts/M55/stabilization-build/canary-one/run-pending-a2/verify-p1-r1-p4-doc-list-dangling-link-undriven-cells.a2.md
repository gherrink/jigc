# verify-real — `r1-p4-doc-list-dangling-link-undriven-cells` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-doc-list-dangling-link-undriven-cells`. One finding, handed over: ledger
key `r1-p4-doc-list-dangling-link-undriven-cells`, door `jigc doc list`, the clause it is said to
break `working-product`, triage's grade *unclear*. It has no block of its own: its source is item 3
of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-doc-list-tracked-dangling-link-raw-os-error.a1.md`
— the cells that report names as not driven. That report was read because the prompt hands it over
as the finding; no other report was read, and nothing of triage's reasoning beyond the grade.

The item, as handed: *Not driven: Linux; a root caller; a placement doctype's home as a dangling
link; the prior-home rows; `jigc doc show`, `jigc validate` and the copy-in over the same state; a
home that is unreadable for another reason (mode 000, a directory, a special file).*

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** Not `does-not-reproduce`: the cells are real,
most of them answer badly, and each stays a row of the ledger. **The verdict covers the cells that
were driven; two of the item's cells — Linux, and a root caller — were not driven and are not
refuted** (see *What was not driven*, and *Left open*, item 1).

- **Every cell the item names that this machine can reach was driven on the candidate**, each on a
  fresh rig: a placement home as a dangling link (two homes), the prior-home rows (a dangling link
  and a mode-000 file), `jigc doc show`, `jigc validate` and the copy-in over the dangling located
  link, and a home that is a mode-000 file, a directory and a FIFO (a located home and a placement
  home each).
- **The same block was driven on the previous release.** 230 cells on each binary: **the exit
  status is the same in all 230.** Of the 460 streams, 447 are byte-identical (the rig's path
  normalised); 11 differ only in an abbreviated commit id of the rig's own history; 2 differ in the
  wording of one refusal that exits 1 on both (the copy-in over a mode-000 doc).
- **So no command that works on the previous release stops working in any driven cell.** Where the
  listing does not answer on the candidate (exit 1, or no exit at all over a FIFO) it does not
  answer on the previous release, in the same bytes.
- **`jigc doc list` prints no route in any of its refusals here**, so no printed route of this door
  fails. The routes the sibling doors print were driven as printed; what they did is in *The printed
  routes*, and one of them is *Left open*, item 7.
- **Every state is a hand plant by a non-jigc writer** — a link, a mode change, a directory or a
  FIFO put at a managed doc's home.

`contested: false` — the finding argues against no settled decision. No `regression` field: the
verdict is not `confirmed`.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a2/jigc`, exit 0. The driver repeats the check for every rig: it puts the handed
  binary's directory first on `PATH`, stops (exit 90) unless `command -v jigc` prints the binary it
  was handed, and stops (exit 95) unless the rig was built with that same binary. Every run passed
  both.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  refs-post-hoc`, stdout captured alone, the construction log to a file of its own; all 31 rig
  builds exited 0 (16 with the candidate, 14 with the previous release, 1 for exploration).
- The clone: nothing was edited, staged or committed by this verifier. `git status --porcelain`
  lists only untracked reports under `completions/artifacts/canary-one/r1/reports/test/`, before
  and after; `HEAD` 126a8531 on `fix/canary-one`.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0, arm64), git 2.54.0 (Apple Git-157), as a non-root user
(uid 501).** Nothing was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-undriven.jrIMGm` (written `<W>`). Under it: `<W>/c/` (candidate) and `<W>/p/`
(previous release), each with `log`, `rigs/` (one fresh rig per scenario) and
`runs/<scenario>/<cell>.out` and `.err`; `<W>/tools/drive.sh` and `<W>/tools/compare.sh`;
`<W>/explore/`, one rig used to find the addresses before the driver was written. Nothing was torn
down.

Every cell: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, the
working directory the rig's repository, stdin from `/dev/null`, stdout and stderr to files of their
own, the exit status read directly — no pipe anywhere a status is read.

**Deviations, stated.**

1. The driver and the comparison are scratch files written from the shell; they are in no
   repository.
2. **Every jigc cell runs under a wall-clock alarm** — `perl -e 'alarm shift; exec @ARGV' <T> jigc
   …` — because a read of a FIFO never returns. `exec` replaces the process, so the status read is
   jigc's own; a cell the alarm ends reads **142** (128 + SIGALRM). `T` was 25 s, and 10 s in the
   FIFO scenarios. No cell outside the FIFO scenarios read 142.
3. **Two setups were wrong the first time and were redone under a new name.** `git rm` of the only
   doc under `docs/research/` removes the directory with it, so the `mkdir` and the `mkfifo` that
   followed failed. The scenarios `loc-dir` and `loc-fifo` on the candidate therefore hold a state
   in which the doc is simply absent (its removal committed); their cells are kept as an
   *absent-doc control* and are in no count below. `loc-dir2` and `loc-fifo2` recreate the
   directory first and are the ones counted.
4. The item has no block. Each scenario is the smallest setup that reaches the cell from the rig
   state `refs-post-hoc`, which commits a located doc (`research:context-loss` at
   `docs/research/context-loss.md`), two placement docs with a root home (`CHANGELOG.md`,
   `VISION.md`), one with a home under a directory (`docs/roadmap.md`), and holds one open task.

## What was driven — the candidate

The read cells of every scenario, in this order: `jigc doc list` · `--format json` · `jigc doc list
<the doctype>` · `--format json` · `jigc doc list decisions-log` (a doctype that does not hold the
entry) · `jigc doc show <the doc>` · `--format json` · `jigc validate` · `--format json` · `jigc
doc list --task <the rig's task>`. Then, where the table says so, the copy-in: `jigc doc set-slot
<a slot of the doc> --from-file <file> --task <the rig's task>`, and the two listings again.

**Controls (scenario `ctl`, the rig as built, 18 cells, all exit 0).** The listing prints its header
and five rows; `doc show` serves both docs; the sweep exits 0 with one advisory; **the copy-in
works** — `set slot research:context-loss#question … (copied in for update …)`, and the staged copy
is in the task's working area afterwards — for the located doc and for the placement one.

| scenario | what stands at the home | `jigc doc list` (both formats) | `jigc doc show` | `jigc validate` | the copy-in |
|---|---|---|---|---|---|
| `loc-dangling` — the handed finding's own state, re-driven | tracked link, index mode `120000`, target removed and the removal committed | **exit 1**, stdout empty, `reading the committed doc at "<abs repo>/docs/research/context-loss.md": No such file or directory (os error 2)` | exit 1, `store.not-found`, a route | exit 1, `reconciliation.rename` blocking, a route | exit 1, *no staged instance …*, nothing staged |
| `plc-dangling` — `CHANGELOG.md` | the same shape at a placement home | **exit 0, the `changelog` row is gone**: four rows where the live link gave five; `jigc doc list changelog` prints `jigc doc list — no committed changelog docs` (the doctype in backticks), `--format json` prints `{"docs": []}` | exit 1, `store.not-found`, a route | exit 1, `reconciliation.rename` blocking, a route | exit 1, *no staged instance …*, nothing staged |
| `plc-dangling-roadmap` — `docs/roadmap.md` | the same shape at a placement home under a directory | **exit 0, the `roadmap` row is gone** | exit 1, `store.not-found` | exit 1, `reconciliation.rename` | not driven |
| `prior-home`, the link dangling — `docs/changelog/changelog.md`, the v1 home of `changelog` | tracked link, target removed | **exit 0, the prior-home row is gone** (it is present over the regular file and over the live link: `changelog:changelog  docs/changelog/changelog.md  managed`) | exit 1, `store.not-found`; the route no longer names the prior home or `jigc migrate-corpus`, which it does over the regular file and the live link | exit 1; the `schema-version-current` row is gone, `reconciliation.rename` and `schema-conformance.home-vacated` stay | not driven |
| `prior-mode000` — the same prior home, a mode-000 regular file | `----------`, index `100644` | **exit 0, the prior-home row is gone** | exit 1, as the row above | exit 1, as the row above | not driven |
| `loc-mode000` | regular file, mode 000 (`git status` shows it modified) | **exit 1**, stdout empty, `… "<abs repo>/docs/research/context-loss.md": Permission denied (os error 13)` | exit 1, `store.not-found` … `Permission denied (os error 13)`, a route | **exit 1, stdout empty**, `validating the committed store at "<abs repo>": Permission denied (os error 13)` — no code, no route | exit 1, `could not copy … in for editing — nothing was staged: Permission denied (os error 13)` |
| `plc-mode000` — `CHANGELOG.md` | regular file, mode 000 | **exit 1**, `… "<abs repo>/CHANGELOG.md": Permission denied (os error 13)` | exit 1, `store.not-found` | exit 1, the same code-less line | exit 1, the same sentence |
| `loc-dir2` | a directory holding one tracked file | **exit 1**, `… Is a directory (os error 21)` | exit 1, `store.not-found` … `Is a directory (os error 21)` | **exit 0**, the one advisory of the control and nothing about the home | exit 1, *no staged instance …* |
| `plc-dir` — `CHANGELOG.md` | a directory holding one tracked file | **exit 1**, `… "<abs repo>/CHANGELOG.md": Is a directory (os error 21)` | exit 1, `store.not-found` | exit 1, stdout empty, `validating the committed store at "<abs repo>": Is a directory (os error 21)` | exit 1, *no staged instance …* |
| `loc-fifo2` | a FIFO (untracked — git records none); the doc's removal committed | **no exit: killed by the alarm at 10 s (142), both streams empty** | no exit, 142 | no exit, 142 | exit 1, *no staged instance …* |
| `plc-fifo` — `CHANGELOG.md` | a FIFO | **no exit, 142** | no exit, 142 | no exit, 142 | exit 1, *no staged instance …* |

Three facts hold in every row above:

- **`jigc doc list decisions-log` exits 0** with its one row — a listing narrowed to a doctype that
  does not hold the entry is not taken down, the FIFO rows included.
- **`jigc doc list --task <task>` exits 0** with the task's own rows — the staged arm does not read
  the committed home.
- **Nothing is written.** `git status --porcelain` at the end of each scenario is what it was after
  the plant (empty; the mode-000 file alone shows as modified, and is clean again once its mode is
  put back), the entry at the home is the one planted, and no copy-in that refused left a staged
  copy (the task's working area lists the same three files before and after).

**The controls around each plant.** Over a **live** link the listing, the read and the sweep answer
exactly as over the regular file: the ten read cells of `loc-dangling`'s live step are
byte-identical, both streams, to `ctl`'s, and the placement and prior-home live steps list their
row. After `chmod 644` the listing over the former mode-000 file exits 0 with all five rows (both
mode-000 scenarios). In the absent-doc control the listing exits 0 without the row.

**The dangling link reads as an absent doc at three of the four doors.** `jigc doc show`, `jigc
validate` and the copy-in over the dangling located link are **byte-identical, both streams, to the
same cells over a doc whose removal was committed** (the absent-doc control). Only `jigc doc list`
tells the two apart — it refuses the one and omits the other.

## The printed routes, driven as printed

`jigc doc list` prints **no route** in any refusal above: a search of the stderr of every committed
listing cell of the candidate for `route`, `jigc ` with a trailing space, or `blocking` finds only
the one-line staged-listing note, and that only on cells that exit 0. The sibling doors print
three, over the dangling located link (scenario `route-loc`, a fresh rig) and the dangling placement
link (`route-plc`):

| the door, and the route as printed | the command, run as printed | what followed |
|---|---|---|
| `jigc validate` → `reconciliation.rename`: *restore docs/research/context-loss.md, or confirm the deletion by dropping it from the index: `jigc unmanage docs/research/context-loss.md`* | **exit 0**: *unmanaged docs/research/context-loss.md (research:context-loss) — dropped its file-state baseline + forward edges; the file is left on disk …* | the link stands, untouched. `jigc validate` then **exits 0** with the control's one advisory. **`jigc doc list` still exits 1** with the same line, and `jigc doc show` still answers `store.not-found` |
| the same route at the placement home: `jigc unmanage CHANGELOG.md` | **exit 0** | `jigc validate` exits 0; `jigc doc list` exits 0, the `changelog` row still absent |
| `jigc doc show` → `store.not-found`: *create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:context-loss --task <task-id>` (find the task id with `jigc task list`)* | `jigc task list` **exits 0** and names the task. `jigc doc show research:context-loss --task <that id>` **exits 1**: `store.not-staged — research:context-loss is not staged in this task and has no committed copy — nothing to read yet` | the route's own condition — *a doc staged in an open task* — does not hold in this rig: no task stages the doc. See *Left open*, item 7 |
| the copy-in's refusal: *create research from a task minted on a workflow that grants it (`jigc start` lists the catalog)* | `jigc start` **exits 0** and lists the catalog | not followed further: minting a doc over an entry with no body is the design's declared bound (4), below |

## The same block on the previous release

Not the regression step — the verdict is not `confirmed` — but what the clause's instrument is
worded on. Fresh rigs built with `--binary <scratch>/bin/previous-91834b5e011d/jigc`, that binary's
directory first on `PATH`, the same driver, the same scenarios: log `<W>/p/log`.

- **230 cells on each binary** — 189 in the ten scenarios without a FIFO, 26 in the two with one, 15
  in the two route scenarios. **The exit status is the same in every one** (`<W>/tools/compare.sh`:
  `exit-diffs=0` in all three runs). Eighteen of the 230 read 142 on each binary — the same
  eighteen.
- **Streams** (stdout and stderr of each cell, the rig's path normalised to one token): 447 of 460
  byte-identical. The 13 that differ:
  - 10 are `jigc validate` outputs in the two prior-home scenarios, and 1 is `jigc start`'s
    orientation; each differs only in an abbreviated commit id of the rig's own history, and all 11
    are byte-identical once that id is normalised as well.
  - 2 are the stderr of the copy-in over a mode-000 doc, exit 1 on both. The previous release:
    `could not read committed <address>: Permission denied (os error 13)`. The candidate: `could not
    copy <address> in for editing — nothing was staged: Permission denied (os error 13)`.
- **The listing over each plant answers on the previous release exactly as on the candidate**: the
  same exit 1 with the same line over the located dangling link, the mode-000 files and the
  directories; the same exit 0 with the row missing over the placement and prior-home plants; the
  same hang over the FIFOs.

## Against the design that owns the behaviour

- `design/doc-read-surface.md` → *`jigc doc list` — the fourth read surface*: the enumerator
  *yields a placement doctype's instance on mere file existence* — so a placement home whose link
  dangles yields nothing and the row is absent, which is what was observed; a located doctype's
  instances are enumerated from its directory, and there the same link is read and refuses. The
  section settles an instance that does not **parse** (it counts 0 — *never a block: `doc list` is
  a report*); it says nothing of an entry that cannot be **read**. For the prior-home rows it
  refers to `design/validation.md`'s fifth family and *its best-effort posture*; the observed drop
  of an unreadable prior-home entry is consistent with that posture, which this verifier did not
  read further.
- `design/finalize.md` → *4. Promote managed docs* → **Declared bounds, (2)**: *Reads still follow a
  link at a home — `jigc doc show` serves a linked doc and the copy-in reads one — and a read over an
  entry it cannot read is its own seam: `jigc doc list` answers a code-less exit 1 over one
  unreadable entry, and a FIFO under a doctype's directory parks the validate phase before any
  planner runs. Neither is a promote; both are filed with the review.* The exit-1 cells and the
  sweep's FIFO cells are that sentence, driven. **(4)** of the same list: *A plain create over an
  entry with no doc body (a dangling link, a directory) still mints fresh and is refused at the
  committing door.*
- `DECISIONS.md` → *2026-10-06 — One code for a managed home that is not an ordinary file* → **Not
  this ruling's, and unchanged:** *a read through a link at a home*; and its **Left open**: *a FIFO
  at a home still parks the validate phase before any planner runs (the standing read seam, not this
  commit's)*.
- `design/surface-contract.md` → *The three laws*: law 1 holds *every printed path* to *repo-real
  or a typed identity*, law 2 holds the surface that produces a state to naming its recovery. The
  code-less refusals — `jigc doc list`'s and, over a mode-000 doc, `jigc validate`'s — print the
  machine's absolute path and no route. That is what they fall short of: the surface contract, not
  the closing condition's second clause.

So no settled decision **intends** a raw OS error, a silent omission or a hang here — the basis is
not `intended` — and none is argued against. The design records the seam as known, open and filed.

## Does it break the clause, inside its scope

The clause (`DECISIONS.md`, 2026-10-04, *The exit rule, revised*): *a working product others can
use and rely on*; its one instrument (sharpening 2): *no command that works on rc.24 in a supported
layout stops working, and every refusal's route works as printed.*

1. **No command that works on rc.24 stops working.** In all 230 cells the candidate exits as the
   previous release does. Where a cell works there (the controls, the live links, the listings
   narrowed to another doctype, the staged listings, the recoveries) it works on the candidate with
   the same bytes; where it refuses, omits or hangs there, it does the same on the candidate.
2. **Every refusal's route works as printed.** `jigc doc list`, the finding's door, prints none.
   Of the sibling doors' routes: `jigc unmanage <path>` exits 0 and clears the finding it was
   printed for; `jigc task list` and `jigc start` exit 0; the conditional read `jigc doc show
   <address> --task <id>` refuses where its stated condition does not hold. That last route is
   `store.not-found`'s own, printed in the same bytes for a doc that is simply absent and on the
   previous release; it is not this door's, and it is left open rather than ruled here.
3. **In a supported layout.** Each state was reached by a non-jigc writer: a link, a mode, a
   directory or a FIFO put at a managed doc's home by hand. `design/finalize.md` (the paragraph
   above the declared bounds) states that *jigc writes regular files and real directories and never
   a link*. Whether a jigc verb can carry a live link into a dangling one is another row's question
   and was not driven here.

The defects are real, and wider than the handed finding's one cell: one unreadable entry denies the
whole committed listing with a raw OS error (three shapes), the same entry at a placement or prior
home is dropped from the listing at exit 0, a FIFO at a home parks three read verbs, and the sweep
has a code-less exit 1 of its own. None breaks the clause inside its scope **in a driven cell**,
because the previous release answers each in the same way, no printed route of this door fails, and
every state is planted.

## What was not driven

- **Linux.** The candidate handed over is a macOS arm64 binary, the prompt hands over no trial
  image, and this role builds nothing. Not driven, and not refuted.
- **A root caller.** This verifier is uid 501 and escalates nothing. Not driven, and not refuted.
  *Read, not driven:* the mode-000 cells rest on the caller being refused the read, which a root
  caller is not; the other shapes do not rest on the caller.

## Scope of what was verified

**`instance, unbounded`.** Driven: one located doctype (`research`), two placement homes
(`CHANGELOG.md`, `docs/roadmap.md`), one prior home (`changelog`'s v1 location home); four shapes (a
tracked dangling link, a mode-000 regular file, a directory, a FIFO); four doors (`jigc doc list`
committed and staged arms, `jigc doc show`, `jigc validate`, the copy-in through `jigc doc
set-slot`); the `agent` and `json` formats; two binaries; one platform; a non-root caller. The
mechanism's consumers were not enumerated: the other readers of what the committed enumerator
yields were not counted, and no suite was run.

## Repro U-1

```yaml
claim: "the cells left undriven beside the dangling-link listing — a placement home and a prior home as a dangling link, `doc show` / `validate` / the copy-in over the same state, and a home that is a mode-000 file, a directory or a FIFO — said to break `working-product`"
verdict: REFUTED   # as a blocker — basis breaks-no-clause; every cell reproduces, on both binaries alike
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d: the same exit status in all 230 cells; 447 of 460 streams byte-identical, 11 differing in a commit id of the rig, 2 in one reworded refusal"
platform: "macOS 26.6.2 arm64, git 2.54.0, a non-root caller; not driven on Linux, not driven as root"
# Each cell group starts from a FRESH fixture. <task> is the fixture's open task.
cell-A-placement-dangling:
  setup:
    - fixture: refs-post-hoc
    - ["mkdir", "notes"]
    - ["git", "mv", "CHANGELOG.md", "notes/changelog-target.md"]
    - ["ln", "-s", "notes/changelog-target.md", "CHANGELOG.md"]
    - ["git", "add", "CHANGELOG.md", "notes/changelog-target.md"]
    - ["git", "commit", "-q", "-m", "plant: the changelog's home becomes a tracked link"]
  control:
    - ["jigc", "doc", "list", "changelog"]          # the live link: exit 0, the row `changelog:changelog  CHANGELOG.md  managed`
  setup-2:
    - ["git", "rm", "-q", "notes/changelog-target.md"]
    - ["git", "commit", "-q", "-m", "plant: the link's target is removed"]
  repro:
    - ["jigc", "doc", "list"]
    - ["jigc", "doc", "list", "changelog", "--format", "json"]
    - ["jigc", "doc", "show", "changelog:changelog"]
    - ["jigc", "validate"]
  expect:
    - { exit: 0, stdout_lacks: "changelog:changelog", stdout_contains: "vision:vision  VISION.md  managed" }
    - { exit: 0, stdout_json: { "docs": [] } }
    - { exit: 1, stdout: "", stderr_contains: "blocking · store.not-found — could not read `changelog:changelog` at `CHANGELOG.md`: No such file or directory (os error 2)" }
    - { exit: 1, stdout_contains: "reconciliation.rename — tracked managed doc changelog:changelog (CHANGELOG.md) is missing" }
cell-B-prior-home:
  setup:
    - fixture: refs-post-hoc
    - ["mkdir", "-p", "docs/changelog"]
    - ["git", "mv", "CHANGELOG.md", "docs/changelog/changelog.md"]
    - edit: "docs/changelog/changelog.md — `schema-version: 2` becomes `schema-version: 1`, the H1 `# Changelog` becomes `# changelog`"
    - ["git", "add", "-A"]
    - ["git", "commit", "-q", "-m", "plant: a v1-era changelog at its prior home"]
  control:
    - ["jigc", "doc", "list", "changelog"]          # exit 0, the row `changelog:changelog  docs/changelog/changelog.md  managed`
  setup-2:                                          # either of the two
    - ["chmod", "000", "docs/changelog/changelog.md"]                       # non-root caller only
    - "or: the home made a tracked link and its target removed, as in cell A"
  repro:
    - ["jigc", "doc", "list", "changelog"]
  expect:
    - { exit: 0, stdout: "jigc doc list — no committed `changelog` docs\n" }
cell-C-sibling-doors-over-the-located-dangling-link:
  setup:
    - fixture: refs-post-hoc
    - ["mkdir", "notes"]
    - ["git", "mv", "docs/research/context-loss.md", "notes/context-loss-target.md"]
    - ["ln", "-s", "../../notes/context-loss-target.md", "docs/research/context-loss.md"]
    - ["git", "add", "docs/research/context-loss.md", "notes/context-loss-target.md"]
    - ["git", "commit", "-q", "-m", "plant: the research doc's home becomes a tracked link"]
    - ["git", "rm", "-q", "notes/context-loss-target.md"]
    - ["git", "commit", "-q", "-m", "plant: the link's target is removed"]
  repro:
    - ["jigc", "doc", "list"]
    - ["jigc", "doc", "show", "research:context-loss"]
    - ["jigc", "validate"]
    - ["jigc", "doc", "set-slot", "research:context-loss#question", "--from-file", "<a one-line file>", "--task", "<task>"]
    - ["jigc", "unmanage", "docs/research/context-loss.md"]     # the sweep's printed route
    - ["jigc", "validate"]
    - ["jigc", "doc", "list"]
  expect:
    - { exit: 1, stdout: "", stderr: "reading the committed doc at \"<abs repo>/docs/research/context-loss.md\": No such file or directory (os error 2)\n" }
    - { exit: 1, stdout: "", stderr_contains: "blocking · store.not-found — could not read `research:context-loss` at `docs/research/context-loss.md`: No such file or directory (os error 2)" }
    - { exit: 1, stdout_contains: "route: restore docs/research/context-loss.md, or confirm the deletion by dropping it from the index: `jigc unmanage docs/research/context-loss.md`" }
    - { exit: 1, stdout: "", stderr_contains: "no staged instance for `research:context-loss#question`", tree: "the task's working area holds no research:context-loss.md" }
    - { exit: 0, stdout_contains: "unmanaged docs/research/context-loss.md (research:context-loss)" }
    - { exit: 0, stdout_lacks: "reconciliation.rename" }
    - { exit: 1, stdout: "", stderr_contains: "No such file or directory (os error 2)" }
    - tree: "`git status --porcelain` is empty throughout, and the home is still the link"
cell-D-mode-000:                                    # non-root caller only
  setup:
    - fixture: refs-post-hoc
    - ["chmod", "000", "docs/research/context-loss.md"]
  repro:
    - ["jigc", "doc", "list"]
    - ["jigc", "doc", "list", "--format", "json"]
    - ["jigc", "validate"]
    - ["jigc", "doc", "list", "decisions-log"]
  expect:
    - { exit: 1, stdout: "", stderr: "reading the committed doc at \"<abs repo>/docs/research/context-loss.md\": Permission denied (os error 13)\n" }
    - { exit: 1, stdout: "", stderr_json: { "error": "reading the committed doc at \"<abs repo>/docs/research/context-loss.md\": Permission denied (os error 13)" } }
    - { exit: 1, stdout: "", stderr: "validating the committed store at \"<abs repo>\": Permission denied (os error 13)\n" }
    - { exit: 0, stdout_contains: "decisions-log:decisions-log  docs/decisions-log.md  managed" }
  recovery:
    - ["chmod", "644", "docs/research/context-loss.md"]
    - ["jigc", "doc", "list"]                       # exit 0, five rows
cell-E-directory:
  setup:
    - fixture: refs-post-hoc
    - ["git", "rm", "-q", "docs/research/context-loss.md"]
    - ["mkdir", "-p", "docs/research/context-loss.md"]          # -p: the `git rm` took docs/research/ with it
    - ["cp", "README.md", "docs/research/context-loss.md/keep.txt"]
    - ["git", "add", "docs/research/context-loss.md/keep.txt"]
    - ["git", "commit", "-q", "-m", "plant: a directory stands at the research doc's home"]
  repro:
    - ["jigc", "doc", "list"]
    - ["jigc", "validate"]
  expect:
    - { exit: 1, stdout: "", stderr: "reading the committed doc at \"<abs repo>/docs/research/context-loss.md\": Is a directory (os error 21)\n" }
    - { exit: 0, stdout_lacks: "context-loss" }
cell-F-fifo:
  setup:
    - fixture: refs-post-hoc
    - ["git", "rm", "-q", "docs/research/context-loss.md"]
    - ["git", "commit", "-q", "-m", "plant: the research doc is removed"]
    - ["mkdir", "-p", "docs/research"]
    - ["mkfifo", "docs/research/context-loss.md"]
  repro:                                            # each under a watchdog; none exits by itself
    - ["jigc", "doc", "list"]
    - ["jigc", "doc", "show", "research:context-loss"]
    - ["jigc", "validate"]
    - ["jigc", "doc", "list", "decisions-log"]
  expect:
    - { exit: "none within 10 s — killed by the watchdog", stdout: "", stderr: "" }
    - { exit: "none within 10 s — killed by the watchdog", stdout: "", stderr: "" }
    - { exit: "none within 10 s — killed by the watchdog", stdout: "", stderr: "" }
    - { exit: 0, stdout_contains: "decisions-log:decisions-log" }
observed: "<W>/c/log with <W>/c/runs/ (candidate); <W>/p/log with <W>/p/runs/ (previous release); scenarios plc-dangling, plc-dangling-roadmap, prior-home, prior-mode000, loc-dangling, route-loc, route-plc, loc-mode000, plc-mode000, loc-dir2, plc-dir, loc-fifo2, plc-fifo, ctl"
pinned-by: "UNPINNED: found this round. A search, not an enumeration, and no suite was run (`command grep` over crates/cli/tests, crates/engine/tests and tooling-tests): the sentence `reading the committed doc at` is in none of them (one hit in the tree, the production site); the three suites under crates/cli/tests that plant a FIFO hold no `list` argv; the three that spell mode `0o000` set it on entries under `.jigc/`, none on a managed doc's home. A suite that reaches one of these cells by another spelling would not have been found by this search"
```

**Pinnable as it stands: no — in two cell groups, and for one decision.** Cells A, B (its link
arm), C and E convert by hand: the fixture is a named state of the shared builder and every step is
an argv, one link creation or one in-place edit. **Cell D and the mode-000 arm of B** rest on the
caller being refused a read, so the same test passes for an ordinary user and fails as root — it
needs a guard on the effective uid, which the block does not carry. **Cell F** has no exit to
assert: a test of it needs a watchdog that kills the child, which the block does not specify. And
**the decision**, for every cell that pins a refusal, an omission or a hang: these are seams the
design files as open, so a standing test of one turns red the day the seam is closed — it then
wants to be that fix's red test, not a pinned fact. The controls (the live link lists, reads and
sweeps as a regular file; the listing narrowed to another doctype answers; the listing answers
again once the mode is put back) are facts to hold. The comparison with the previous release is
not a suite's to hold: a suite drives one binary.

## Left open

Not pursued; each is for triage like any finding.

1. **Linux, and a root caller, are still not driven** — for this door and for every cell above. The
   two cells of the handed item that this report does not cover.
2. **A FIFO at a managed home parks `jigc doc list`, `jigc doc show` and `jigc validate`** — no
   exit within 10 s, both streams empty, at a located home and at a placement home, on both
   binaries. `design/finalize.md` (declared bound 2) and the 2026-10-06 entry name the park for the
   validate phase; neither names the listing or the read.
3. **`jigc validate` has a code-less exit 1 of its own**, with the machine's absolute path and no
   route: `validating the committed store at "<abs repo>": Permission denied (os error 13)` over a
   mode-000 doc (located and placement), and `… Is a directory (os error 21)` over a directory at a
   placement home. Both formats, both binaries. The design's sentence names this shape for `jigc doc
   list` only.
4. **A placement home or a prior home that cannot be read is dropped from the listing at exit 0** —
   `jigc doc list changelog` prints *no committed `changelog` docs* while git tracks an entry at
   that home. Two answers to the same link: refused at a located home, omitted at a placement one.
   Both binaries.
5. **After the sweep's own route the sweep and the listing disagree.** Over the dangling located
   link, `jigc unmanage docs/research/context-loss.md` exits 0, `jigc validate` then exits 0 with
   nothing about that home, and `jigc doc list` still exits 1 for the whole store. Both binaries.
6. **The same disagreement without any route:** over a directory at a located home `jigc validate`
   exits 0 with nothing about it, while `jigc doc list` exits 1. Both binaries.
7. **`store.not-found`'s route over an unreadable home.** It offers *create the referenced doc, or
   fix the reference*, and a staged read that answers `store.not-staged` when no task stages the
   doc; it names neither the entry that stands at the home nor what would make it readable, for a
   dangling link, a mode-000 file and a directory alike. Whether that is a route that *works as
   printed* is a reading of the instrument this verifier did not make. Both binaries, the same
   bytes.
8. **The sweep's closing line over these states** — *out-of-band rename detected … (revert the `git
   mv` or adopt it via `jigc rename`)* — is printed where nothing was renamed, and names a verb
   without its arguments. Seen, not driven.
9. **One refusal's wording moved between the releases**: the copy-in over a mode-000 doc (quoted
   above), exit 1 on both. Recorded as a difference; nothing was judged of it.
10. **Not driven:** a socket or a device node at a home; an untracked link (every link above is
    committed); `VISION.md`, the home of the doc the rig's task already holds staged; the copy-in
    over a live link, and over the roadmap and prior-home plants; `--format human`; a prior home
    that is a placement file (`deferral-ledger`'s) — only `changelog`'s location prior home was
    driven; following the copy-in's route into a create over the planted entry (the design's
    declared bound 4).

<!-- end of report -->
