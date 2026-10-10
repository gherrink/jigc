# verify-real — `r1-p3-task-finalize-refuses-staged-non-utf8-text-file`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: the
third cell of the block under the heading *Observed, and not to be pinned green* in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-git-capture-other-callers-not-enumerated.a1.md`
— *`jigc task finalize` refuses a task that stages a text file holding a byte that is not
UTF-8*. Door: `jigc task finalize`. Clause it is said to break: `working-product`. Triage's
grade: *unclear*. Triage asked for that cell on both binaries, for the doc-only, amend and
migration arms with the same staged file, and for whether any refusal there carries a route.

## Verdict in one paragraph

**`refuted` — as a blocker, with the basis `breaks-no-clause`. It is not `does-not-reproduce`:
the refusal is real, it is a defect, and it reproduces byte for byte on both binaries.** A task
that stages a 13-byte text file holding one byte 0xE9 cannot be finalized on the ordinary arm:
exit 1, one line on standard error — `` `git` produced non-UTF-8 output: invalid utf-8 sequence
of 1 bytes from index 130 `` — no finding code, no `at:`, no route, `HEAD` unmoved, the working
area standing, the file intact. The migration arm refuses the same way through another helper.
The doc-only arm lands at exit 0 and leaves the file staged. The amend arm refuses at exit 3 as
it refuses the ASCII control, by design, with a route that works as printed. **The previous
release does exactly the same in all 18 cells** — the same exits, the same refusal bytes, the
same landed file lists. The second clause's instrument is *no command that works on rc.24 in a
supported layout stops working, and every refusal's route works as printed*: no command stopped
working, and the one route any of these refusals prints works. So the finding breaks no clause
inside its scope, and stays a row of the ledger as a defect both releases carry.

**What that verdict rests on, said so that nobody has to infer it.** It reads the second clause
by the instrument the entry of 2026-10-04 gives it. It does not say the door *works* in this
state — it does not. Whether a door that has never run over a staged Latin-1 text file makes
the product not *a working product others can rely on* in a sense wider than that instrument is
not a question this verdict answers, and not one a verifier settles.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`,
  label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's. **It was driven**, through the same driver and the same cells: triage
  asked for both binaries.
- The candidate's directory went first on `PATH` and `command -v jigc` printed the candidate's
  path. The driver repeats that check in every cell, before the rig is built and again after
  the rig's assignments are evaluated, with the binary it was handed, and stops if it fails;
  for the previous release's cells that binary's directory is the one put first, inside that
  cell's process only. Every rig was built with `dev/jigc-rig fresh --binary <that binary>`,
  and the driver stops unless the rig's `$JIGC` is that path.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host. Each rig's `HOME` is
  the rig's own, so no global git configuration of the machine's reaches a cell.

## What was driven

18 cells per binary, each on a fresh rig of its own under `<scratch>/verify-p3.YO9B3b/runs/`,
none of them the reporter's; every command whose exit status is read ran bare, its standard
output and standard error each to a file of its own. Two follow-up steps (the amend refusal's
printed route, then the finalize again) were run in the two `amend.latin1` cells afterwards.

**The plant.** A real file `legacy.txt`, written with `printf`, then `git add legacy.txt`
(exit 0 in every cell that stages it) — nothing planted by plumbing:

| state | bytes of `legacy.txt` | valid UTF-8 | what `git diff --cached` prints for it |
|---|---|---|---|
| none | no file | — | nothing |
| ascii | `cafe au lait`, newline | yes | a text hunk |
| utf8 | `caf`, 0xC3 0xA9, ` au lait`, newline | yes | a text hunk |
| **latin1** | `caf`, **0xE9**, ` au lait`, newline — the handed cell | **no** | a text hunk holding the raw byte, at offset 130 of 140 |
| nul | `caf`, 0xE9, 0x00, ` au lait`, newline | no | `Binary files … differ` |
| latin1-untracked | the latin1 bytes, **not** added | no | nothing |
| latin1-attr | the latin1 bytes, and a staged `.gitattributes` holding `legacy.txt -diff` | no | `Binary files … differ` |

Read back in every cell: the file's hex, whether the file and the captured `git diff --cached`
are valid UTF-8, the file's blob id before and after the door.

**The arms.**

- **ord** — the handed cell. Rig `fresh --start single-task "probe the finalize door"`; the
  plant; `jigc doc set-field commit:<task>#type --value chore --task <task>`; `jigc doc set-slot
  commit:<task>#summary --from-file - --task <task>`; `jigc task validate <task>`;
  **`jigc task finalize <task>`**; on a refusal the same again with `--format json` and with
  `--format human`.
- **hc** — the file is already in `HEAD`, committed by a plain `git commit` before the task is
  minted; the task changes its second line, which is ASCII, and stages that. States `ascii`
  and `latin1` (two lines each).
- **doc** — rig `fresh --start report-jigc-feedback "…"`; one `jigc-feedback` doc created and
  its three required fields and its `description` set; the plant; the commit doc; the door.
- **amend** — rig as `ord`; a staged `code.txt` finalized first (exit 0); `jigc task amend
  "reword the probe commit"` (exit 0); the plant; the amend task's commit doc; the door.
- **mig** — rig `fresh`; a foreign `docs/direction.md` committed by plain git; `jigc migrate
  docs/direction.md --as vision` (exit 0); `jigc doc author vision` with the three sections
  (exit 0); the plant; the commit doc; `jigc task finalize <task>`, then
  **`jigc task finalize <task> --approve`**.

**What I changed from the block as handed.** Three things, none of them the claim's cell.
(1) The block's `control` reads *without `legacy.txt`: exit 0 and the commit lands*. Driven as
written — a `fresh` rig, a minted task, a filled commit doc, nothing staged — that is exit 3,
`finalize.empty-commit`, on both binaries (cell `ord.none`); the reporter's own table shows its
cells also staged a `code.txt` the block does not mention. My control is `ord.ascii`: the same
file name, the same length class, ASCII bytes. (2) The block spells the field address
`#header/type`; the composed workflow prints `commit:<task>#type`, and that is what was run.
(3) Every state other than `latin1`, and the arms `hc`, `doc`, `amend` and `mig`, are mine or
triage's: the block has one cell.

### `jigc task finalize`, ordinary arm

| state | exit, both binaries | `HEAD` | working area | what it said |
|---|---|---|---|---|
| none | 3 | same | standing | `finalize.empty-commit`, with a route |
| ascii | 0 | moved; commit holds `legacy.txt` | gone | `finalized <sha> — chore: probe the finalize door` |
| utf8 | 0 | moved; commit holds `legacy.txt` | gone | the same |
| **latin1** | **1** | **same** | **standing** | standard output empty; standard error, whole: `` `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 130 `` |
| nul | 0 | moved; commit holds `legacy.txt` | gone | `finalized …` |
| latin1-untracked | 3 | same | standing | `finalize.nothing-staged`, with a route |
| latin1-attr | 0 | moved; commit holds `.gitattributes`, `legacy.txt` | gone | `finalized …` |
| **hc**, ascii | 0 | moved; commit holds `legacy.txt` | gone | `finalized …` |
| **hc, latin1** | **1** | **same** | **standing** | the same line, `from index 121` |

In the two refusing cells: `git status --short` afterwards is `A  legacy.txt` (`M  legacy.txt`
in `hc`), the file's blob id is unchanged, and nothing was written. With `--format json` the
door exits 1 with standard output empty and standard error
`{"error": "`git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 130"}`;
with `--format human` it prints the agent arm's line. **`jigc task validate <task>` exits 0 in
both refusing cells** and names nothing about the file.

**The refusal is the staged diff's text, and nothing about the file's name or git's state.**
The index the line names is the offset of the byte 0xE9 in that cell's own `git diff --cached`
output — 130 of 140 bytes in `ord.latin1`, 121 of 151 in `hc.latin1`, both computed from the
capture the driver took before the door ran. Where git prints `Binary files … differ` for the
same bytes (`nul`, `latin1-attr`) the door lands, and where the file is not staged the door
gives its ordinary finding. `hc.latin1` shows that the byte need not be in a line the task
changed: it stands in a context line of the hunk.

### `jigc task finalize`, doc-only arm

| state | exit, both binaries | `HEAD` | the index afterwards |
|---|---|---|---|
| none | 0 | moved; commit holds `docs/jigc-feedback/legacy-file-refused.md` | empty |
| ascii | 0 | moved; the same one file | `legacy.txt`, still staged |
| **latin1** | **0** | **moved; the same one file** | **`legacy.txt`, still staged** |

The landed manifest names the file under `left-out` before the commit and after it, in the
`latin1` cell as in the `ascii` one. This arm does not refuse.

### `jigc task finalize`, amend arm

| state | `task validate` exit | finalize exit, both binaries | `HEAD` | what it said |
|---|---|---|---|---|
| none | 0 | 0 | rewritten; tree holds `code.txt` | `amended <sha> → <sha>` |
| ascii | 3 | 3 | same | `blocking · finalize.amend-index-dirty` naming `legacy.txt`, with a route |
| **latin1** | **3** | **3** | **same** | **the same finding, the same route** |

The refusal is the control's, and the amend verb's help states it: *the amend refuses over a
non-empty index*. **Its route was run as printed**, in both `latin1` cells:
`git -C <the rig's repo> restore --staged -- legacy.txt` exits 0, `git status --short` then
reads `?? legacy.txt`, and `jigc task finalize reword-the-probe-commit` exits 0 with
`amended <sha> → <sha>`, the file named under `left-out`, its blob id unchanged. The route
contains no `jigc` word, so nothing in it could resolve to another binary.

### `jigc task finalize`, migration arm

| state | plain finalize exit | `--approve` exit, both binaries | `HEAD` | what it said |
|---|---|---|---|---|
| none | 4 | 0 | moved; commit holds `VISION.md`, `docs/direction.md` | the review hold, then `finalized …` |
| ascii | 4 | 0 | moved; commit holds `VISION.md`, `docs/direction.md`, `legacy.txt` | the same |
| **latin1** | **1** | **1** | **same** | standard error, whole: `` `git diff` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 130 `` |

The working area stands, `legacy.txt` is still staged and its blob id is unchanged. The line's
context string is not the ordinary arm's (`` `git diff` `` against `` `git` ``): it is another
capture helper's, over `git diff <base>`, and it decodes the same way. `jigc task validate`
exits 0 here too. Note that in this arm the refusal comes at the plain finalize already, so the
review hold a migration is meant to show first (exit 4) is never reached.

### Does any refusal there carry a route?

| arm, state `latin1` | exit | finding line with a code | `at:` | route | the route, run as printed |
|---|---|---|---|---|---|
| ordinary (`ord`, `hc`) | 1 | no | no | **no** | nothing to run |
| doc-only | 0 | — it lands | | | |
| amend | 3 | yes, `finalize.amend-index-dirty` | yes | **yes** | exit 0, and the finalize then lands |
| migration, plain and `--approve` | 1 | no | no | **no** | nothing to run |

The same on both binaries.

### Candidate against previous release

Exit status at every step, whether `HEAD` moved, the landed commit's file list, the status
afterwards and the refusal's surface agree in all 18 cells: the two facts files, with the
binary's label and the commit ids taken out, are identical line for line (331 lines each).
Of the 284 stream files compared between the two binaries, with rig paths normalised, 18
differ, each in one line and each only in an abbreviated commit id.

## Is it what the finding says?

**Yes, as to the behaviour.** The finding says: exit 1, `` `git` produced non-UTF-8 output ``,
on both binaries, over one byte 0xE9 in a 13-byte file, default git configuration, nothing
planted by plumbing; it loses nothing; the doc-only, amend and migration arms were not driven.
Every part of that held on a rig of my own. What the re-drive adds: the migration arm refuses
too, the doc-only arm does not, the amend arm's refusal is its designed one, and a file already
in `HEAD` blocks a task that edits a different line of it.

**Tried, as the ways the finding could be wrong, and not found.** *Stale state:* every cell is
a rig minted for it. *Read through a pipe or cut:* every exit is read bare and every stream is
a whole file. *Another binary:* both hashes asserted, `command -v jigc` held per cell. *A
planted state:* the plant is `printf` and `git add`. *An artefact of the file being new:* `hc`.
*A refusal for another reason:* the offset in the line is the byte's offset in that cell's
staged diff, and the cells where git prints no text for the same bytes land.

**Read against the design that owns it — it is a defect, and no settled decision intends it.**

- `design/finalize.md` → *Dirty-tree policy*: *the commit set is the git index*; the agent
  stages its code and *the CLI commits the whole index as one logical change*. Nothing there
  bounds what the staged bytes may be. The staged diff is read at this door as a presence
  signal only — *is there anything to commit* — so the decode decides nothing the door needs.
- `implementation/parsing.md`, the byte-stability table: *Encoding — UTF-8 only; non-UTF-8 →
  conformance error (no transcoding)*. That row is about a **managed document** the engine
  parses; `legacy.txt` is the user's code, which jigc never parses.
- `design/command-output-contract.md` → *The two reject arms* and the exit table: *an
  operational error carries the single key `error` … exit 1 — and nothing else*, exit 1 being
  *operational error … a git/IO failure*. **The shape of this refusal is that declared arm**,
  which carries no code and no route by contract; `design/surface-contract.md` → *The route
  fence* widens the route floor to *blocking validation/gate findings* and leaves *prose-only
  error text* untouched. What is not declared anywhere is that this state should be an
  operational error at all: git exited 0 and said nothing wrong.
- The same contract's third clause says `jigc task validate` *previews the gate up to the
  commit*; an operational error is not a gate finding, so its exit 0 here contradicts no
  sentence of it — and still tells the caller nothing.

The finding does not argue that a decision is wrong, and I found none it would have to argue
against. Nothing here is contested.

## Does it break `working-product`, inside that clause's scope?

No. **Basis: `breaks-no-clause`.**

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  second clause is *a working product others can use and rely on*, and its instrument, by the
  second sharpening, is *no command that works on rc.24 in a supported layout stops working,
  and every refusal's route works as printed*; *a finding blocks the 1.0.0 call only if it
  breaks a clause inside its scope; everything else is recorded with its tier*. The run's
  opening names the clause `working-product` and its instrument *the regression set, and here
  the gate*; `implementation/stabilization-workflow.md` → *The regression set* states it as
  *no command that works on the previous release stops working*.
- **No command that works on the previous release stops working.** In the handed cell, and in
  the two other refusing cells, the previous release exits 1 with the same bytes. In the cells
  where the previous release lands or gives its designed refusal, the candidate does the same.
- **Every refusal's route works as printed.** One refusal among these prints a route — the
  amend arm's — and it works as printed, on both binaries. The ordinary and migration refusals
  print none, in the arm the output contract declares route-less; there is no printed route
  that fails.
- This holds without deciding whether a repository that holds a Latin-1 text file is *a
  supported layout*: I took it to be one, which is the reading least favourable to the binary.

**What this verdict does not say.** That the door works: it does not, and an agent in this
state gets a line that names neither the file, nor the byte's place in any file, nor a way
out. That nothing should be done: the row stays, as a defect. And it does not bound the class.

## The regression fact

**Not owed by this verdict**: step 4 runs with `confirmed` only. Both binaries were driven
through every cell because triage asked for it, and what that showed is under *Candidate
against previous release*: no cell is green on the previous release and red on the candidate.
The door and every arm of it exist on the previous release, so the cells are comparable.

## Class

**`instance, unbounded`.** I drove one door, `jigc task finalize`, on its four arms, with one
file and one invalid byte, and one further state of the ordinary arm (`hc`). I enumerated
nothing. Two helpers were seen to refuse — the one behind the ordinary arm's line and the one
behind the migration arm's — and I counted neither's callers nor any other helper that decodes
a diff's text. The report I was handed carries an enumeration of one of them; I did not
re-derive it and give no count of my own.

## Coverage

The finding makes no coverage claim, and none is verified here.

## Repro VR-NU-1

The cells that state facts this verdict wants to stay true: beside a staged text file that is
not UTF-8, the doc-only arm lands and leaves the file staged, and the amend arm gives its
designed refusal with a route that works.

```yaml
claim: "jigc task finalize refuses a task that stages a text file holding a byte that is not UTF-8 — exit 1, no route — breaking working-product"
verdict: "REFUTED as a blocker — breaks-no-clause. The refusal reproduces on the ordinary and migration arms and is a defect; the previous release gives the same exits and the same bytes, and the one route any arm prints works as printed."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; the previous release, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, gives the same exits"
plant:                      # a real file, no plumbing; the bytes are content, so no filesystem limit applies
  - "write legacy.txt = the bytes `caf`, 0xE9, ` au lait`, newline"
  - ["git", "add", "legacy.txt"]                         # exit 0
doors:
  - door: "jigc task finalize, doc-only arm"
    setup:
      - fixture: fresh                  # dev/jigc-rig fresh --binary <binary>
      - ["jigc", "start", "--workflow", "report-jigc-feedback", "the finalize door refuses a legacy file"]
      - ["jigc", "doc", "create", "jigc-feedback", "--title", "Legacy file refused", "--task", "<task>"]
      - "set-slot jigc-feedback:legacy-file-refused#description; set-field #meta/kind = bug, #meta/found-in = trial:verify, #meta/jigc-version = 1.0.0-rc.24"
      - plant
      - "fill the commit doc: set-field commit:<task>#type = docs; set-slot commit:<task>#summary"
    repro:
      - ["jigc", "task", "finalize", "<task>"]
    expect:
      exit: 0
      stdout_contains: ["finalized ", "left-out", "legacy.txt"]
      head_commit_files: ["docs/jigc-feedback/legacy-file-refused.md"]
      index_after: ["legacy.txt"]
    control: "with legacy.txt = `cafe au lait`: the same exit, the same commit, the same index"
  - door: "jigc task finalize, amend arm"
    setup:
      - fixture: fresh
      - ["jigc", "start", "--workflow", "single-task", "probe the finalize door"]
      - "write code.txt; git add code.txt; fill the commit doc; jigc task finalize <task>   # exit 0"
      - ["jigc", "task", "amend", "reword the probe commit"]
      - plant
      - "fill the amend task's commit doc: set-field commit:<amend-task>#type = chore; set-slot commit:<amend-task>#summary"
    repro:
      - ["jigc", "task", "finalize", "<amend-task>"]
    expect:
      exit: 3
      stderr_contains: ["finalize.amend-index-dirty", "restore --staged -- legacy.txt"]
      head: "unmoved"
    then:                               # the printed route, run as printed
      - ["git", "-C", "<repo>", "restore", "--staged", "--", "legacy.txt"]    # exit 0
      - ["jigc", "task", "finalize", "<amend-task>"]                           # exit 0, `amended <sha> → <sha>`
    control: "with legacy.txt = `cafe au lait`: the same exit 3, the same finding, the same route"
observed: "<scratch>/verify-p3.YO9B3b — cand.log and prev.log (every command, its exit, stdout, stderr), cand.facts and prev.facts, runs/<binary>/<arm>.<state>/"
pinned-by: "UNPINNED: a verifier writes no test — the two cells convert as they stand"
```

**Pinnable as it stands: yes, these two cells.** The plant is a file's content written as raw
bytes and one `git add`; it needs no name the filesystem might refuse and no git
configuration, so it converts on every platform the suites run on.

### Observed, and not to be pinned green

Three cells hold the defect as observed; pinned green they would hold it in place. Each is a
fix's red test in waiting, with its `expect` inverted, if a fix is scheduled.

```yaml
- door: "jigc task finalize, ordinary arm"          # the handed cell
  setup:
    - fixture: fresh
    - ["jigc", "start", "--workflow", "single-task", "probe the finalize door"]
    - "write legacy.txt = the bytes `caf`, 0xE9, ` au lait`, newline; git add legacy.txt"
    - "fill the commit doc: set-field commit:<task>#type = chore; set-slot commit:<task>#summary"
  repro:
    - ["jigc", "task", "finalize", "<task>"]
  observed: { exit: 1, stdout: "", stderr: "`git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 130", head: "unmoved", working_area: "standing" }
  control: "with legacy.txt = `cafe au lait`: exit 0 and the commit holds legacy.txt. NOT `without legacy.txt`, which is exit 3 finalize.empty-commit"
- door: "jigc task finalize, ordinary arm, the file already in HEAD"
  setup:
    - fixture: fresh
    - "write legacy.txt = `caf`, 0xE9, ` au lait`, newline, `line two`, newline; git add; git commit --no-verify"
    - ["jigc", "start", "--workflow", "single-task", "probe the finalize door"]
    - "rewrite the second line as `line TWO`; git add legacy.txt; fill the commit doc"
  repro:
    - ["jigc", "task", "finalize", "<task>"]
  observed: { exit: 1, stderr_contains: "`git` produced non-UTF-8 output", head: "unmoved" }
  control: "the same with `cafe au lait` as the first line: exit 0"
- door: "jigc task finalize --approve, migration arm"
  setup:
    - fixture: fresh
    - "write docs/direction.md (a foreign doc); git add; git commit --no-verify"
    - ["jigc", "migrate", "docs/direction.md", "--as", "vision"]
    - "jigc doc author vision — thesis, invariants, open-questions"
    - "write legacy.txt = the bytes `caf`, 0xE9, ` au lait`, newline; git add legacy.txt"
    - "fill the commit doc: set-field commit:<task>#type = docs; set-slot commit:<task>#summary"
  repro:
    - ["jigc", "task", "finalize", "<task>"]                # observed exit 1; the control's is 4, the review hold
    - ["jigc", "task", "finalize", "<task>", "--approve"]
  observed: { exit: 1, stdout: "", stderr: "`git diff` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 130", head: "unmoved" }
  control: "with legacy.txt = `cafe au lait`: exit 4, then exit 0 under --approve"
```

## Left open

1. **`jigc task validate` exits 0 over a task whose finalize then exits 1** — in `ord.latin1`
   and `mig.latin1`, both binaries. The preview is documented as covering the gate's findings
   and the repository posture, so this contradicts no sentence I found; it was not pursued.
2. **A migration finalize refuses at its plain call, before the review hold** (`mig.latin1`,
   exit 1 where the control's plain call is exit 4), through a capture helper that is not the
   ordinary arm's. Its other callers were not enumerated and no other door of it was driven.
3. **`jigc task finalize --approve` of a migration commits a staged file that is not the
   migration's** — `mig.ascii`: the landed commit holds `VISION.md`, `docs/direction.md` and
   `legacy.txt`; both binaries. `design/finalize.md` → *Dirty-tree policy* speaks of a
   migration task's *fixed … narrowing* of the stage. I did not read that policy through or
   decide whether this is what it intends; it is another clause's question
   (`no-lost-files`: *commits content the user did not ask for*) and was not graded.
4. **The handed block's control does not hold as written** — *without `legacy.txt`: exit 0 and
   the commit lands* is exit 3, `finalize.empty-commit`, on both binaries. That is a fault of
   the block, not of the binary; whoever converts the block should take the control from this
   report.
5. **The ways through that I saw are not routes the binary prints.** The door lands when git
   treats the file as binary — a NUL in it, or a `-diff` attribute for it (`ord.nul`,
   `ord.latin1-attr`). Recorded as what the cells showed, not as a recommendation: neither was
   judged as a route, and a user is told of neither.
6. **Other encodings and other shapes of the same state** — UTF-16 text, an invalid byte in a
   deleted or renamed file, a file under a `working-tree-encoding` attribute, the same file in
   a sub-task's worktree at `jigc milestone finalize`. Not driven.

## Bounds — what this verification did not do

- One macOS host, one git. No Linux cell.
- One file, one invalid byte, one position in the file; one corpus per arm; one value per
  argument. `--format json` and `--format human` were driven in the ordinary arm's refusing
  cell only.
- The doc-only arm was driven through one of the four workflows that compose it
  (`report-jigc-feedback`); the migration arm through one doctype (`vision`).
- The amend arm was driven with the file staged beside the commit being amended. An amend of a
  commit that itself holds such a file was not built.
- The site behind each refusal is identified from the driven cells — the offset, and the cells
  where git prints no text — and from reading the two helpers' context strings in the source
  at the candidate's commit; nothing was instrumented.
- It read `DECISIONS.md`'s entries of 2026-10-04 (*The exit rule, revised*), 2026-10-05 (*The
  stabilization workflow, as ruled*) and 2026-10-06 (*The regression set's first part*),
  `implementation/stabilization-workflow.md` → *The regression set*, `design/finalize.md` →
  *Dirty-tree policy*, `design/command-output-contract.md` → *The two reject arms* and the
  exit table, `design/surface-contract.md` → *The three laws* and *The route fence*, and
  `implementation/parsing.md`'s byte-stability table. It read the one report it was handed,
  and no other finding's or verifier's.

## Where the evidence is

- `<scratch>/verify-p3.YO9B3b/scripts/drive.sh` — one cell: rig, plant, door, read-back.
  `matrix.sh` — the 18 cells, in order, for one binary.
- `<scratch>/verify-p3.YO9B3b/cand.log`, `prev.log` — every command, its exit status, its
  standard output and standard error, in order. `cand.facts`, `prev.facts` — one line per
  fact; `cand.facts.norm`, `prev.facts.norm` — the same without the label and the commit ids,
  the two files that compare equal.
- `<scratch>/verify-p3.YO9B3b/runs/cand/`, `runs/prev/` — one directory per cell: a file per
  stream per step, the staged diff as captured before the door (`plant.diff-cached`), and the
  rig itself. In the two `amend.latin1` cells, steps 11 and 12 are the printed route and the
  finalize after it.
- `<scratch>/verify-p3.YO9B3b/explore/`, `try.log`, `try.facts`, `runs/try/` — the exploration
  that fixed each arm's command line and the driver's trial run, candidate only; nothing in
  the tables is read from them.

<!-- end of report -->
