# verify-real — `r1-doc-list-staged-arm-unreadable-entry` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p1-r1-doc-list-staged-arm-unreadable-entry`. One finding, handed over: ledger key
`r1-doc-list-staged-arm-unreadable-entry`, door `jigc doc list`, the clause it is said to break
`working-product`, triage's grade *unclear*, its block `Repro RC-2` of
`completions/artifacts/canary-one/r1/reports/test/row-doc-list-reconciler.a1.md`. That report was read
for this finding's block; no other verifier's report and nothing of triage's reasoning was read.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`. The behaviour itself reproduces, cell for cell;
it stays a row of the ledger.**

- **The instance is real.** On the candidate, in a rig of this verifier's own, `jigc doc list --task <id>`
  exits 1 with empty stdout and a raw OS error on stderr when the task's staging area holds a
  staged-doc-named entry that is a dangling link (os error 2) or a mode-000 file (os error 13); a
  directory or a named pipe of such a name is skipped at exit 0. With the dangling link there, the
  committed listing still exits 0 with its five rows and its stderr is empty: the staged-listing note,
  208 bytes in the control, is gone. Every one of the reporter's cells holds as written.
- **It breaks neither half of the second clause.** The clause's instrument, in the closing condition's
  words (DECISIONS.md, 2026-10-04, *The exit rule, revised*, sharpening 2): *no command that works on
  rc.24 in a supported layout stops working, and every refusal's route works as printed.*
  - *First half.* The same block on a fresh rig of the previous release gives the same exit status
    and byte-identical stdout and stderr in all twelve invocations (the rig's path normalised). No
    command that works there stops working here; the command that fails here fails there, in the
    same words.
  - *Second half.* The two refusals print no route. A route that is not printed cannot fail as
    printed; the clause's text asks that printed routes work, and says nothing of a refusal that
    carries none. Reading an absent route as a break of this clause is a reading the text does not
    give, and it is not this verifier's to add.
- **The state is not one ordinary use reached in what was driven or read.** See *Does ordinary use
  reach the state*, below — three attempts through jigc's own verbs under ordinary configuration,
  and the one write primitive read in source. Bounded, and stated as such: not a proof.

`contested: false` — the finding does not argue that a settled decision is wrong.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (1.0.0-rc.24).
- The binary's directory went first on `PATH` in every driver script, and each script stops unless
  `command -v jigc` prints the binary it was handed. For the candidate that printed
  `<scratch>/bin/c1.a1/jigc`.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<W> dev/jigc-rig --binary <binary> refs-post-hoc`,
  stdout captured alone, the construction log to its own file, exit status 0 each time.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and after,
  `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user (uid 501).**
Nothing was driven on Linux. The mode-000 cell depends on the caller not being root: a root caller
reads through mode 000, so that cell is expected to be green there — not driven.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-staged-arm.zNLggx` (written `<W>`). Three rigs under it, each from the rig tool's own
`mktemp -d`: one for the candidate, one for the previous release, one for the reach probes. Nothing
was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. One driver, `<W>/tools/drive.sh <rig env> <out dir>`: for each invocation,
`git status --porcelain --untracked-files=all --ignored` and a listing of every entry of the
repository outside `.git/` (kind, mode, size, mtime, path) before and after; stdin from `/dev/null`;
stdout and stderr to their own files; the exit status read directly, never through a pipe. Each
plant was made alone and removed before the next.

## What was driven — the candidate

Rig `<W>/jigc-rig-refs-post-hoc-93UktG`, task `ground-the-vision-in-research`, staging area
`.jigc/tasks/ground-the-vision-in-research/docs/` holding `commit:ground-the-vision-in-research.md`,
`vision:vision.md` and `provenance.json`. Log: `<W>/cand.log`; per-cell files under `<W>/cand.runs/`.

| label | plant | invocation | exit | stdout | stderr |
|---|---|---|---|---|---|
| T0-task | none | `jigc doc list --task ground-the-vision-in-research` | 0 | header + two rows, 134 bytes | empty |
| T0-committed | none | `jigc doc list` | 0 | header + five `managed` rows, 257 bytes | the staged-listing note, 208 bytes |
| T2-task | dangling link `research:dangling.md` | `jigc doc list --task …` | **1** | empty | ``listing the docs staged in task `ground-the-vision-in-research`: No such file or directory (os error 2)`` |
| T2-task-json | the same | `… --format json` | **1** | empty | the same sentence in an `{ "error": … }` envelope |
| T2-committed | the same | `jigc doc list` | 0 | byte-identical to T0-committed | **empty — the note is absent** |
| T2-committed-json | the same | `jigc doc list --format json` | 0 | five rows, 1132 bytes | **empty** |
| T3-task | mode-000 file `research:locked.md` | `jigc doc list --task …` | **1** | empty | `reading the staged doc at "<abs repo>/.jigc/tasks/ground-the-vision-in-research/docs/research:locked.md": Permission denied (os error 13)` |
| T3-committed | the same | `jigc doc list` | 0 | byte-identical to T0-committed | the note, as in T0-committed |
| T1-task | directory `research:a-dir.md` | `jigc doc list --task …` | 0 | byte-identical to T0-task | empty |
| T4-task | named pipe `research:pipe.md` | `jigc doc list --task …` | 0 | byte-identical to T0-task (no hang) | empty |
| Tend-task | every plant removed | `jigc doc list --task …` | 0 | byte-identical to T0-task (`cmp`) | empty |
| Tend-committed | every plant removed | `jigc doc list` | 0 | byte-identical to T0-committed (`cmp`) | the note, byte-identical |

In all twelve: the entry listing identical before and after, the porcelain unchanged. Nothing is
written by the door in any cell.

**Against the claim.** Every cell of `Repro RC-2` holds: the two exit-1 shapes with the two stderr
lines the block gives, the two skipped shapes, and the absent note on the committed listing under
the dangling link. One cell the block does not state was driven on the way: under the mode-000
plant the committed listing **keeps** its note (T3-committed) — see *Left open*.

**Against the design that owns the behaviour.**

- `design/doc-read-surface.md` → the `--task` arm of `jigc doc list`: *a staged working copy is
  jigc-written by construction, never a foreign squatter*. The design states no behaviour for an
  entry in a staging area that jigc did not write; it presupposes there is none.
- The enumerator's own contract (`crates/cli/src/task.rs`, the comment over `staged_doc_ids`, line
  1210 on): *an entry whose own shape cannot be read is an `Err`* — on purpose, so that the doors
  which destroy a task area can fail closed — and *callers for whom the list is decoration take
  `.unwrap_or_default()`*. The dangling link's exit 1 is that `Err`, surfaced by the staged arm
  (`doc.rs:5023`); the note's disappearance is the decoration caller (`staged_listing_hint`,
  `doc.rs:5142`, `is_ok_and`). Both are what the code says it does. This verifier found no dated
  decision that rules the *listing's* answer to such an entry, so the basis claimed is
  `breaks-no-clause` and not `intended`.
- The route floor (`design/surface-contract.md`, the laws' scope): it binds findings, and *prose-only
  error text is untouched*. The two refusals are I/O errors with a context sentence, not findings;
  the floor as written does not reach them.

So what remains is a real, low-reach surface defect — a raw OS error, no route, and in the mode-000
cell the machine's absolute path — in a state the design assumes away. It is not a break of a clause.

## The same block on the previous release

Run for the clause's first half, which is comparative by its own wording; the verdict is not
`confirmed`, so no `regression` field is returned. Rig `<W>/jigc-rig-refs-post-hoc-yecymV`, built with
`--binary <scratch>/bin/previous-91834b5e011d/jigc`; the same driver; log `<W>/prev.log`.

All twelve invocations: the same exit status as on the candidate. `diff` of the two whole logs, the
rig's path and the timestamps normalised, differs in four lines and only in them — the size of
`provenance.json` in the directory listings the driver prints (305 bytes on the candidate, 115 on
the previous release), which is the rig's construction and no output of the door. Every stdout and
stderr of the door is byte-identical between the two binaries.

## Does ordinary use reach the state

Triage's question. What was done to answer it, and its bound:

**Read.** A staged doc reaches `.jigc/tasks/<id>/docs/` through one primitive, `write_atomic`
(`crates/engine/src/state.rs:1328`), which its comment names as shared by `provision_doc`, `copy_in`
and `persist`: the bytes go to a fresh sibling temp file by `std::fs::write`, which is then renamed
over the path. A file made that way is a regular file whose mode is the process's umask applied to
0666; a rename replaces whatever stood at the path and does not write through a link. So this
primitive cannot leave a link, and leaves an owner-unreadable file only under a umask that masks
the owner's read bit. **Not enumerated:** `instance_path(` has 46 textual hits across the two
crates; they were counted (`grep -rn`), not each read as a writer. Two sites that are not this
primitive were seen and not followed: a `std::fs::rename` at `doc.rs:3322` and a `std::fs::copy` at
`cli/src/milestone.rs:8553`.

**Driven**, on the candidate, in a third rig (`<W>/jigc-rig-refs-post-hoc-*` named in
`<W>/rig-reach.env`; logs `<W>/reach.log`, `<W>/reach2.log`), each a first write that copies a
committed doc into the live task:

| attempt | what was done | staged entry | `jigc doc list --task …` after |
|---|---|---|---|
| P1 | `umask 077`, then `jigc doc set-slot research:context-loss#question --from-file - --task …` (exit 0) | regular file, mode 0600 | exit 0, the row listed |
| P2 | the committed `docs/decisions-log.md` made mode 0444, then `jigc doc add-item decisions-log:decisions-log#entries --title … --task …` (exit 0) | regular file, mode 0644 — the committed file's mode is not carried | exit 0, the row listed |
| P3 | `docs/roadmap.md` replaced by a tracked link to a real file and committed, then `jigc doc add-item roadmap:roadmap#milestones --title … --task …` (exit 0) | regular file, mode 0644 — the link is read through and not carried | exit 0, the row listed |

**Answer, with its bound.** In what was read and in three attempts, jigc's own verbs under ordinary
configuration (a strict umask, a read-only committed doc, a committed doc behind a link) leave only
regular, owner-readable files in a staging area. The two failing shapes were reached only by
writing into the workbench by hand. `.jigc/` is gitignored whole and the adapter rule sends every
write through jigc, so a hand-made link or a hand-locked file there is outside documented use. This
agrees with the reporter's *only jigc writes there in documented use*. It is **not** a proof that no
jigc door can produce the state: the writers were not enumerated, and three attempts are three.

## Scope of what was verified

**Instance, unbounded.** Driven: the door `jigc doc list`, both arms, plain and json, four shapes
of planted entry, on two binaries, on one platform. Not enumerated: the other callers of
`staged_doc_ids` and the other readers of a staged doc; no count of them is given here.

## Repro V-1

```yaml
claim: "in a task's staging area a staged-doc-named entry that is a dangling link or a mode-000 file makes `jigc doc list --task <id>` exit 1 with a raw OS error and no route, and the dangling link also removes the committed listing's staged note — identically on the previous release, so no clause is broken"
verdict: REFUTED   # as a blocker — basis breaks-no-clause; the behaviour itself reproduces
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status and byte-identical stdout and stderr in all twelve cells (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc                  # live task ground-the-vision-in-research staging two docs
  - control: ["jigc", "doc", "list"]        # exit 0; stderr is the staged-listing note, 208 bytes
  - ["ln", "-s", "/nonexistent/x.md", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md"]
repro:
  - ["jigc", "doc", "list", "--task", "ground-the-vision-in-research"]
  - ["jigc", "doc", "list", "--task", "ground-the-vision-in-research", "--format", "json"]
  - ["jigc", "doc", "list"]
expect:
  - exit: 1
    stdout: ""
    stderr: "listing the docs staged in task `ground-the-vision-in-research`: No such file or directory (os error 2)\n"
  - exit: 1
    stdout: ""
    stderr_json: { "error": "listing the docs staged in task `ground-the-vision-in-research`: No such file or directory (os error 2)" }
  - exit: 0
    stdout: "id  path  state\n + the five `managed` rows (changelog, decisions-log, research:context-loss, roadmap, vision) — byte-identical to the control"
    stderr: ""                              # the staged-listing note is absent
  - tree: "the repository's entries and `git status --porcelain --untracked-files=all --ignored` identical around every invocation"
variants:        # each alone, in place of the `ln`
  - "a mode-000 research:locked.md           -> `--task`: exit 1, stdout empty, `reading the staged doc at \"<abs repo>/.jigc/tasks/ground-the-vision-in-research/docs/research:locked.md\": Permission denied (os error 13)`; the committed listing: exit 0, its note PRESENT"
  - "mkdir research:a-dir.md                 -> `--task`: exit 0, the two staged rows, stderr empty (skipped)"
  - "mkfifo research:pipe.md                 -> `--task`: exit 0, the two staged rows, stderr empty (skipped; no hang)"
control: "every plant removed -> both listings byte-identical, stdout and stderr, to the ones taken before any plant"
observed: "<W>/cand.log with <W>/cand.runs/ (labels T0, T2, T3, T1, T4, Tend); the previous release: <W>/prev.log with <W>/prev.runs/"
pinned-by: "UNPINNED: found this round; no test was searched for or run by this verifier"
```

**Pinnable as it stands: yes, with two conditions and one half that no suite can hold.** The
fixture is a named state of the shared builder and every step is an argv or a single filesystem
call, so the block converts by hand to a test. The conditions: it needs a Unix target (a link, a
mode), and the mode-000 variant needs a caller that is not root — as root the read succeeds and the
cell is green. The half that is not pinnable as a suite test: *identical on the previous release* —
a suite drives one binary, and that comparison is the regression set's. And a note for whoever
converts it: the block pins what the binary does today, which a later repair of this row would
change on purpose.

## Left open

Not pursued; each is for triage like any finding.

1. **A printed route that exits 1.** Under the mode-000 plant the committed listing exits 0 and still
   prints its note, whose route is `jigc doc list --task ground-the-vision-in-research`; that command,
   run as printed, exits 1 (labels T3-committed, T3-task). The note is not a refusal, the state is
   hand-made, and the previous release does the same — but it is a printed route whose command
   fails, and the block handed over does not state this cell.
2. **The writers into a staging area were not enumerated.** The answer to *does ordinary use reach
   the state* rests on one primitive read and three attempts; the `std::fs::rename` at `doc.rs:3322`
   and the `std::fs::copy` at `cli/src/milestone.rs:8553` were seen and not followed.
3. **Linux, and a root caller, were not driven.** The mode-000 cell is expected to differ for root.
4. **The other callers of `staged_doc_ids`** (the reporter names `task discard`'s ack and the
   `uninstall` guard) were not driven under these plants.

<!-- end of report -->
