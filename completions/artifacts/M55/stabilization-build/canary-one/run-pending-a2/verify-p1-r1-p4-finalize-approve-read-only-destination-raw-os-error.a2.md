# verify-real — `r1-p4-finalize-approve-read-only-destination-raw-os-error` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-finalize-approve-read-only-destination-raw-os-error`. One finding, handed
over: ledger key `r1-p4-finalize-approve-read-only-destination-raw-os-error`, door
`jigc task finalize`, the clause it is said to break `working-product`, triage's grade *unclear*.
Its source is item 3 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-staging-area-writers-undriven-and-unread-remainder.a1.md`
— a three-sentence observation with no block of its own, which says of itself *the re-run was not
driven*. That report was read because the prompt hands it over as the finding; no other report was
read, and nothing of triage's reasoning beyond the grade and the re-drive it asks for: *the approve
over a mode-0444 destination, then the re-run the closing sentence prints*.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** The observation is real and reproduces
exactly; it is not `does-not-reproduce`, and the row stays a row of the ledger.

What was driven, on the candidate, from nothing:

- **The refusal is as the finding says.** `jigc task finalize <id> --approve` over a destination
  whose mode is 0444 exits 1; stdout is empty; stderr is the line `could not promote "<abs
  source>" to "<abs destination>": Permission denied (os error 13)`, a blank line, and one closing
  sentence. Both paths in the first line are absolute paths of the machine. No code is printed on
  this arm.
- **The closing sentence is not quite what the finding says it is.** It does not say *re-run the
  same command*; it says *Resolve the cause above, then re-run* and names the command. Every state
  claim in it was read from the repository and holds: `HEAD` unchanged, `git status` unchanged, the
  destination's bytes and mode unchanged, the task's staged docs still in its area, nothing parked
  under `.jigc/displaced/`, and — in the variant that staged a file — the staged file still in
  git's index.
- **The re-run, as printed, with the cause not resolved:** exit 1, the same bytes on stderr, and
  the same state. It neither lands nor decays.
- **The re-run, as printed, after the cause is resolved** (`chmod u+w` on the destination, and
  nothing else): **exit 0**, the doc promoted and committed, the task's area gone. In all three
  groups and in the variant.
- **Under `--format json` the same refusal is a finding with a code and a route**:
  `finalize.commit-rejected`, keyed at `task:<id>`, the cause line as its `message`, the closing
  sentence as its `route`. So *no code* is true of the text arm only.
- **The previous release does the same thing, byte for byte.** The same block on a fresh rig of
  1.0.0-rc.24: the same exit status in all 28 cells, and all 56 output streams identical once the
  rig's own path and the short commit hash of the landing line are normalised.

So neither half of the clause's instrument is broken (below): no command that works on rc.24 stops
working, and the route this refusal prints works as printed. What remains is a surface defect of low
weight — two absolute host paths in a cause line, on both arms — which no clause reaches.

`contested: false`. The finding argues against no settled decision, and this report does not either.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a2/jigc`. Every driver run puts the handed binary's directory first on `PATH`
  and stops (exit 90) unless `command -v jigc` prints the binary it was handed; every run passed.
  That is what makes a printed re-run, which names a bare `jigc`, run on the binary under test.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  refs-post-hoc`, stdout captured alone into a file, the construction log into another, exit 0 each
  time (five rigs: one exploration, two for the block, two for the variant).
- The clone: `git status --porcelain` holds only the untracked reports of this stage under
  `completions/artifacts/canary-one/r1/reports/test/`, before and after; branch `fix/canary-one`.
  Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0, arm64), git 2.54.0 (Apple Git-157), as a non-root user
(uid 501). Nothing was driven on Linux, and nothing as root.** Both matter to this finding: a root
caller writes past a mode-0444 file, so the refusal is not expected there at all.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p4-ro.BSW8TP` (written `<W>`). Under it:

- `<W>/explore/` — one candidate rig, used to learn the verbs' spellings. The refusal was first
  seen there; no number in this report is read from it.
- `<W>/c.runs/` (candidate) and `<W>/p.runs/` (previous release) — the block, 28 cells each, each
  binary on a fresh rig of its own.
- `<W>/cx.runs/` and `<W>/px.runs/` — the variant, 4 cells each, each on a fresh rig again.
- `<W>/norm/` — copies of the streams with the rig's path and the short commit hash replaced, for
  the comparison between the binaries.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, umask 022. Each cell has stdin from `/dev/null` or a file, stdout and
stderr to files of their own, and its exit status read directly — never through a pipe. A *state
read* is `git rev-parse HEAD`, `git status --porcelain --untracked-files=all`,
`git diff --cached --name-status`, the destination's shape, mode, size and sha256, the entries of
the task's `docs/` read without following a link, and a listing of `.jigc/displaced/`.

**A printed re-run is run as printed.** The command is lifted out of the refusing cell's own stderr
— the span between the backticks after `re-run` — and handed to `sh -c` unchanged.

**A deviation, stated.** No driver file was written: each driver went to `bash` on stdin. The file
tool wrote this report and nothing else; the evidence files above were written by the cells'
redirections.

## What was driven

Fixture `refs-post-hoc`: a set-up repository with five committed managed docs and one live task,
`ground-the-vision-in-research`, which holds a staged edit of the committed `VISION.md`.

| group | what the destination is, and how the task holds the doc | the door |
|---|---|---|
| MD | a foreign file **at** the adr home under the slug its title mints, mode 0444 — the in-place migration; the doc is `created` | `jigc task finalize <id> --approve` |
| MI | a **managed** committed adr, mode 0444, which a second migration authors over — the create door copies it in; the doc is `edited-from-base` | `jigc task finalize <id> --approve` |
| OT | the rig's own live task, no migration: `VISION.md`, mode 0444, a doc the task copied in and edited | `jigc task finalize <id>` |
| X (variant) | OT again on a fresh rig, with a new file `git add`-ed for the task before the finalize | `jigc task finalize <id>` |

MD and MI are the two groups the source report names. OT is the smallest setup that reaches the same
write: it needs no migration and no `--approve`, which shows the refusal belongs to the promote and
not to the approval. The mode is this verifier's `chmod` in every group, as it was the source
report's.

### The candidate — `<W>/c.runs/log`

28 cells: 17 exit 0, 8 exit 1, 3 exit 4. The three exit-4 cells are the migration's review hold.
The eight exit-1 cells are the subject:

| cell | the command | exit | what the state read says afterwards |
|---|---|---|---|
| MD.4 | `jigc task finalize <task> --approve`, destination mode 0444 | **1** | identical to the read before it: `HEAD` on *docs: a legacy decision note*, status empty, index empty, destination `-r--r--r--`, 73 bytes, the same sha256; three regular entries in the task's `docs/`; nothing displaced |
| MD.5 | the re-run MD.4 prints, as printed, nothing changed | **1** | identical again; stderr byte-identical to MD.4's |
| MD.6 | MD.4 with `--format json` | **1** | — |
| MD.7 | the re-run MD.4 prints, as printed, after `chmod u+w <destination>` | **0** | `HEAD` moved to *docs(adr): adopt docs/decisions/use-plain-files.md as a managed adr*; status and index empty; destination `-rw-r--r--`, 204 bytes, the staged doc's bytes; the task's area gone |
| MI.8 | `jigc task finalize <task> --approve`, the managed home mode 0444 | **1** | identical to the read before it; the source `legacy/second.md` still present, so the retirement did not run |
| MI.9 | the re-run MI.8 prints, as printed, nothing changed | **1** | identical again; stderr byte-identical to MI.8's |
| MI.10 | the re-run MI.8 prints, as printed, after `chmod u+w <destination>` | **0** | *promoted docs/decisions/keep-records-as-text.md*, *deleted legacy/second.md*, *2 files committed*; status and index empty; the area gone |
| OT.4 | `jigc task finalize ground-the-vision-in-research`, `VISION.md` mode 0444 | **1** | identical to the read before it |
| OT.5 | the re-run OT.4 prints, as printed, nothing changed | **1** | identical again; stderr byte-identical to OT.4's |
| OT.6 | OT.4 with `--format json` | **1** | — |
| OT.7 | the re-run OT.4 prints, as printed, after `chmod u+w VISION.md` | **0** | *finalized … — docs: ground the vision in research*, *promoted VISION.md*, *1 file committed*; the area gone |

After the three landings, `jigc doc list` and `jigc validate` both exit 0 (END.1, END.2).

**MD.4's stderr, whole** (973 bytes; `<REPO>` stands for the rig's repository, an absolute path of
the machine in the real bytes; stdout is empty):

```
could not promote "<REPO>/.jigc/tasks/migrate-adr-docs-decisions-use-plain-files-d04ea3b9780c/docs/adr:use-plain-files.md" to "<REPO>/docs/decisions/use-plain-files.md": Permission denied (os error 13)

task migrate-adr-docs-decisions-use-plain-files-d04ea3b9780c is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/migrate-adr-docs-decisions-use-plain-files-d04ea3b9780c/docs/`, and anything you had `git add`-ed is still in git's index. Resolve the cause above, then re-run `jigc task finalize migrate-adr-docs-decisions-use-plain-files-d04ea3b9780c --approve`.
```

MI.8's and OT.4's are the same two lines with their own paths and task id; OT.4's printed re-run
carries no `--approve`, because that run carried none.

**MD.6's stderr** (`--format json`; stdout empty): one findings document, `schema_version` 3, one
finding — `severity` `blocking`, `code` `finalize.commit-rejected`, `key.target`
`task:<task id>`, `message` the cause line above (the two absolute paths included), `route` the
closing sentence above, whole. OT.6 is the same shape.

**The variant — `<W>/cx.runs/log`.** With `notes.txt` staged (`A  notes.txt`) before the finalize:
X.3 exits 1 and the index entry is the same blob afterwards; after `chmod u+w VISION.md` the re-run
X.3 prints exits 0 — *promoted VISION.md*, *added notes.txt*, *2 files committed*. That is the one
half of the closing sentence the three groups could not show, since they stage nothing in git.

### The same block on the previous release — `<W>/p.runs/log`, `<W>/px.runs/log`

Not the regression step: the verdict is not `confirmed` and no `regression` field is returned. It is
what the clause's wording compares against.

- The same exit status in every one of the 28 cells, and in the variant's 4.
- **All 56 streams of the block and all 8 of the variant are byte-identical between the binaries**,
  the rig's path and the seven-character commit hash of the `finalized` line normalised — the
  refusals, their closing sentences, the JSON documents and the landings.
- The two logs differ in six state lines, all for one reason: `provenance.json` is larger on the
  candidate wherever a doc was copied in (305 against 115 bytes; 336 against 135). Same shape, same
  mode; it is the difference the source report already names.

## Does it break the clause, inside its scope

The clause's instrument, in the closing condition's words (DECISIONS.md, 2026-10-04, *The exit
rule, revised*, sharpening 2): *no command that works on rc.24 in a supported layout stops working,
and every refusal's route works as printed.*

- **First half — not broken.** In this state the command does not work on rc.24 either: the same
  exit 1, the same bytes. Nothing stopped working. Whether a doc's home made read-only by hand is a
  *supported layout* is a question this report does not have to settle, and does not.
- **Second half — not broken.** The route this refusal prints is *Resolve the cause above, then
  re-run* and a command. The cause above names the file and the error. Resolved by the one act it
  implies — the owner's write bit back on that file — the printed command lands at exit 0 in every
  group, and the commit holds what the refused run would have committed. Run without resolving the
  cause it refuses again with the same words, which is what a route that opens with *resolve the
  cause* says it will do; no state is lost or changed by trying.

**Against the design that owns the behaviour.** Three parts of what the finding lists are what the
design says should happen, and one is not covered by it:

- *The refusal and the untouched state.* `design/finalize.md` → *Rollback discipline*, the row for
  phase 4: *copy/delete fails mid-way (disk full, permissions)* → capture and restore, *working
  area intact*. That is the row this state lands in, and the state reads bear it out.
- *The cause verbatim, the closing sentence, and no code on the text arm.* `design/finalize.md` →
  *6. Commit*, *The frame is the whole committing family's* and *The state-truth clause is a
  function of the rollback's OUTCOME*, which names the non-hook cell. Its renderer
  (`crates/cli/src/render.rs:2967`, `commit_failed`) says why the sentence reads as it does: *this
  arm names no cause at all — "resolve the cause above" is true whatever the cause was, and the
  cause itself is printed verbatim immediately above it*. The door's identity rides the
  `--format json` document, where it was read (`finalize.commit-rejected`), and the invocation log,
  which is off by default and was not read.
- *A raw OS error.* `design/surface-contract.md` → *The route fence*, last paragraph: error strings
  that carry command spans are in scope of the rewrite; *prose-only error text is untouched*.
- **Not covered: the two absolute paths.** `design/surface-contract.md` → *The printed-path fence
  (law 1)* gives the rule — a printed path is repo-relative, and an absolute that stays says why —
  and its *declared bound* limits the fenced class to what a named set of doors prints; this line
  is outside that set and carries no stated reason. It is a real defect of the surface, on both
  arms and on both binaries. It breaks no clause: it is no route, and nothing that worked stops
  working.

## Scope of what was verified

**`instance, unbounded`.** Driven: one write — the promote's copy over a regular file its owner
cannot write — reached through three setups at `jigc task finalize`, each with exactly one promotion
in its plan; one doctype by migration (`adr`) and one placement doc (`vision`); umask 022; two
binaries; one platform; a non-root caller. The mechanism's other consumers were not enumerated: no
count is given, and none should be read into this report.

## Repro RO-1

```yaml
claim: "`jigc task finalize <id> [--approve]` over a promote destination of mode 0444 exits 1 with a raw OS error — no code on the text arm, two absolute host paths — and the re-run its closing sentence prints does not work as printed, breaking `working-product`"
verdict: REFUTED   # basis breaks-no-clause: the refusal reproduces exactly; the printed route works once the cause it names is resolved; rc.24 is byte-identical
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status in every cell and every output stream byte-identical, the rig's path and the short commit hash normalised (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2 arm64, git 2.54.0, a non-root caller, umask 022; NOT driven on Linux, NOT driven as root"
setup:
  - fixture: refs-post-hoc          # its live task: ground-the-vision-in-research, holding a staged edit of the committed VISION.md
  - ["jigc", "doc", "set-field", "commit:ground-the-vision-in-research#header/type", "--task", "ground-the-vision-in-research", "--value", "docs"]
  - ["jigc", "doc", "set-slot", "commit:ground-the-vision-in-research#summary", "--task", "ground-the-vision-in-research", "--from-file", "-"]   # stdin: ground the vision in research
  - ["chmod", "0444", "VISION.md"]
repro:
  - ["jigc", "task", "finalize", "ground-the-vision-in-research"]
  - ["jigc", "task", "finalize", "ground-the-vision-in-research"]                       # the re-run the first step prints, lifted from its stderr; nothing changed
  - ["jigc", "task", "finalize", "ground-the-vision-in-research", "--format", "json"]
  - ["chmod", "u+w", "VISION.md"]                                                       # "Resolve the cause above"
  - ["jigc", "task", "finalize", "ground-the-vision-in-research"]                       # the same printed re-run
expect:
  - exit: 1
    stdout: ""
    stderr_contains: "Permission denied (os error 13)"
    stderr_ends_with: "Resolve the cause above, then re-run `jigc task finalize ground-the-vision-in-research`.\n"
    tree: "HEAD, `git status --porcelain`, the index, and VISION.md's bytes and mode are what they were; .jigc/tasks/ground-the-vision-in-research/docs/ still holds its three regular entries; no .jigc/displaced/"
  - exit: 1
    stderr: "<byte-identical to the first step's>"
    tree: "unchanged again"
  - exit: 1
    stdout: ""
    stderr_json: { "findings": [ { "severity": "blocking", "code": "finalize.commit-rejected", "key": { "code": "finalize.commit-rejected", "target": "task:ground-the-vision-in-research" } } ] }
  - exit: 0
  - exit: 0
    stdout_contains: "promoted VISION.md"
    stderr: ""
    tree: "HEAD is a new commit `docs: ground the vision in research`; status and index empty; VISION.md holds the staged doc's bytes, mode 0644; the task's area is gone"
variants:   # each on the candidate and on the previous release, with the same result
  - "the in-place migration: a foreign file at docs/decisions/use-plain-files.md, committed, mode 0444; `jigc migrate <it> --as adr`, `jigc doc author adr` under the title `Use plain files`, `jigc task finalize <task>` (exit 4, the hold), `--approve` (exit 1, as above, the printed re-run carrying `--approve`); after `chmod u+w` the printed re-run exits 0"
  - "a migration authored over a managed committed adr whose home is mode 0444 (the create door copies it in): `--approve` exits 1 and the foreign source is still on disk; after `chmod u+w` the printed re-run exits 0, promotes the doc and retires the source — 2 files committed"
  - "the spine with a new file `git add`-ed before the finalize: the index entry is the same blob after the refusal, and the resolved re-run commits it with the doc — 2 files committed"
control: "the same task's `jigc task finalize ground-the-vision-in-research --dry-run`, before the chmod: exit 0, a forecast; and the last step itself — the same command over the same task lands once the mode is the only thing changed"
observed: "<W>/c.runs/log with <W>/c.runs/cells/ (cells OT.1 to OT.7; the variants are MD.*, MI.* and <W>/cx.runs/); the previous release: <W>/p.runs/ and <W>/px.runs/"
pinned-by: "UNPINNED: found this round. The frame is pinned for this door over another cause — `commit_rejected_axis::every_committing_door_keeps_its_frame_when_no_hook_spoke` drives a stale `.git/index.lock`, and reads the clause against the repository — but no suite drives a promote onto a destination its owner cannot write. Searched: `could not promote` appears in no file under crates/cli/tests or tooling-tests; of the nine files there that name a read-only mode (`0o444`, `0o500`, `set_readonly` …), the two that touch a finalize use it for a hook's mode or a manifest, not a promote destination. The suites were read by search, not line by line, and none was run by this verifier"
```

**Pinnable as it stands: yes, under two conditions.** The fixture is a named state of the shared
builder and every step is an argv, `chmod` included. (1) It needs a Unix target — a mode. (2) **It
must not run as root**: a root caller writes past mode 0444, so the first step would land and the
test would redden for a reason that is not the product's; the runner-shaped container's non-root
user satisfies this, a default container user does not. The second step's argv is to be lifted from
the first step's stderr by the test, as the existing frame suite lifts its re-run, and not
hard-coded. The comparison with the previous release is not a suite's to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **Linux and a root caller were not driven.** Both handed binaries are Mach-O arm64 and this
   machine gives no root without a password. Read, not driven: as root the open succeeds and the
   refusal does not occur; on Linux as a non-root user the same open is expected to fail the same
   way.
2. **A plan of more than one promotion, with the read-only destination not the first.** The earlier
   promotions are then written before the failure and put back by the rollback, and the closing
   sentence's *nothing was committed … intact* is a claim about that rollback. Every plan driven
   here held one promotion, so that half of the sentence was not exercised.
3. **`jigc milestone finalize` over a read-only destination** promotes through the same write and
   prints its own frame. Not driven.
4. **A doc's home directory that cannot be written** (mode 0555, for a doc the task minted, where
   the write would create the file) is a neighbouring state of the same row of the rollback table.
   Not driven.
5. **Nothing says so before the finalize.** The only `--dry-run` driven ran before the mode was
   changed; whether the forecast, `jigc task validate` or the review hold says anything about a
   destination that cannot be written was not looked at. The hold (exit 4) did not, in MD.3 and
   MI.7 — both ran with the destination already read-only and printed the ordinary hold.
6. **The store sweep's `unadopted-instance` route prints the machine's absolute path** — seen again
   among the advisories of MD.7's landing, on both binaries. It is item 4 of the source report's
   own *Left open*; nothing further was driven.
7. **The code on the JSON arm is `finalize.commit-rejected`** for a failure in which no commit was
   attempted. It is the door's one identity for every cell of its frame, by the design cited above;
   noted only because a driver that keys on the word *rejected* reads a hook where there was none.

<!-- end of report -->
