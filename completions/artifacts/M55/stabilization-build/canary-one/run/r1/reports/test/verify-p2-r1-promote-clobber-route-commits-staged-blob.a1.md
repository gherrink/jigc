# canary-one, round 1, test stage - verify-real of `r1-promote-clobber-route-commits-staged-blob` (attempt 1)

Verdict: **refuted** - basis `intended`. Not contested.

Everything the finding describes reproduces, on a fresh rig, exactly as its reporter saw it: the refusal,
the route, the commit of two files, and one path under both `added` and `left-out`. Set against the
design section that owns each of the three, each is what that section says the door does. No byte is
lost - the staged blob is in the route's commit under the object id it was staged with - and what is
committed is what was `git add`-ed after the task was minted. Nothing here breaks `no-lost-files`.

## What was handed

- Ledger key `r1-promote-clobber-route-commits-staged-blob`. Door: `jigc task finalize`. Clause:
  `no-lost-files`. Triage's grade: unclear.
- Its source: `completions/artifacts/canary-one/r1/reports/test/verify-p1-r1-scope-other-registries-unread-two-changed.a1.md`,
  *Drive C* and *Left open*, item 2. It has no block of its own; Drive C names its steps in prose.
- What triage asked: Drive C on both binaries; the refusal's sentence about the file git holds set
  against what the route's finalize commits and against the summary's two lists; both reconciled with
  `design/finalize.md` -> 4. Promote, *A home git holds is occupied*, and -> *Dirty-tree policy*.

That one report was read because the prompt hands it as the finding's repro. No other finding's report
and no other verifier's was read.

## The binaries

    $ dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc
    {"act": "hash", "status": "hashed", "path": "bin/c1.a1/jigc", "bytes": 15795744, "content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc", ...}      exit 0
    $ dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc
    {"act": "hash", "status": "hashed", "path": "bin/previous-91834b5e011d/jigc", "bytes": 15240064, "content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d", ...}  exit 0

Both equal the hashes of the `BINARY:` line. With the candidate's directory first on `PATH`,
`command -v jigc` printed the candidate's path; for the drive on the previous release its directory
went first instead and `command -v jigc` printed that path - checked again at the top of every call.
Each rig was built with `dev/jigc-rig fresh --binary <that path>`, stdout only captured, exit 0, under
a directory of my own minted with `mktemp -d` inside the scratch root. Nothing under `target/` was
driven and nothing was built.

## The clause, and its scope

`DECISIONS.md` -> *2026-10-04 - The exit rule, revised*, the first sharpening: *In a healthy repository
used as documented ... no jigc command at exit 0 destroys bytes no git object holds or commits content
the user did not ask for. Where jigc cannot tell ... it refuses before writing. Races against a
non-jigc writer inside a millisecond window, and deliberately planted states, are declared bounds.*
The clause is in the run's closing condition (`opening.md`, `no-lost-files`).

So the finding breaks it only if the route's finalize, at exit 0, destroys bytes no git object holds,
or commits content the user did not ask for.

## What changed against the reporter's setup

One thing. Drive C ran in the main checkout of a rig that already carried a linked worktree and one
finalized task. This drive starts from a `fresh` rig with neither: the smallest setup that reaches the
same door. Every command from the task's mint on is Drive C's own.

## The drive, candidate

Rig `fresh`, every command from `<rig>/repo`.

| step | command | exit | what it did |
|---|---|---|---|
| 1 | `jigc start --workflow record-decision "second decision"` | 0 | `task minted: second-decision`; the composed text says, of finalize: *Finalize commits only the staged set plus the docs it manages; unstaged edits and untracked files are left out ... Anything still staged from BEFORE this task was minted makes finalize refuse too* |
| 2 | `jigc doc create adr --title "Second decision" --task second-decision` | 0 | `adr:second-decision` |
| 3 | three `jigc doc set-slot adr:second-decision#<context, decision, consequences> --from-file - --task second-decision`; `jigc doc set-field commit:second-decision#type --value docs --task second-decision`; `jigc doc set-slot commit:second-decision#summary --from-file - --task second-decision` | 0 each | staged; `git status --porcelain` empty |
| 4 | the plant: one line written to `docs/decisions/second-decision.md`; `git add docs/decisions/second-decision.md`; the file removed from disk | 0 each | `git status --porcelain`: `AD docs/decisions/second-decision.md`; `git ls-files -s`: `100644 7d4653b72407c212b5f49f3600912c53eeffaf8e 0` |
| 5 | `jigc task finalize second-decision` | **3** | stdout empty; stderr the refusal below; `git log` unchanged (2 commits), status and index entry unchanged |

The refusal, whole, stderr (the checkout's path shortened to `<rig>/repo`):

    blocking · finalize.promote-clobber — `docs/decisions/second-decision.md` is missing from the worktree but not from git — a file is staged there in git's index, in no commit — and this task's doc `adr:second-decision` was minted as a new doc, not as an edit of that file: promoting it would replace the file git holds under its own path, so it is not promoted there
      at: docs/decisions/second-decision.md
      route: nothing was committed and this task's staged docs are intact: give this task's doc an id whose home is free (`jigc doc rename adr:second-decision --to "<title>" --task second-decision`), then re-run `jigc task finalize second-decision`. The file git holds at `docs/decisions/second-decision.md` is left exactly as it is: `git -C <rig>/repo checkout -- docs/decisions/second-decision.md` brings it back into the worktree, and that is no commit

The route, as printed:

| step | command | exit | what it did |
|---|---|---|---|
| 6 | `jigc doc rename adr:second-decision --to "Second decision revised" --task second-decision` | 0 | `adr:second-decision-revised (renamed ...)`; status still `AD` |
| 7 | `jigc task finalize second-decision --dry-run` | 0 | `would commit — docs: record the second decision`, `added docs/decisions/second-decision.md`, `promoted docs/decisions/second-decision-revised.md`, and `left-out (unstaged/untracked — git add to include): docs/decisions/second-decision.md`; nothing committed |
| 8 | the same with `--format json` | 0 | `manifest`: `{kind: added, path: docs/decisions/second-decision.md}`, `{kind: promoted, path: docs/decisions/second-decision-revised.md}`; `left_out`: `{kind: deleted, path: docs/decisions/second-decision.md}` |
| 9 | `jigc task finalize second-decision` | **0** | first `finalize — about to commit the index; leaving out:` with the same left-out path; then `finalized 397a0be — docs: record the second decision`, `promoted docs/decisions/second-decision-revised.md`, `added docs/decisions/second-decision.md`, `2 files committed`, and the left-out list again with that path |
| 10 | `git show --name-status --format='%h %s' HEAD` | 0 | `A docs/decisions/second-decision-revised.md`, `A docs/decisions/second-decision.md` |
| 11 | `git rev-parse HEAD:docs/decisions/second-decision.md`; `git cat-file -p` of it | 0 | `7d4653b72407c212b5f49f3600912c53eeffaf8e` - the id step 4 staged; the planted line, byte for byte |
| 12 | `git ls-files -s docs/decisions/`; `git status --porcelain` | 0 | the index entry at the path is still `7d4653b...`; status ` D docs/decisions/second-decision.md` |
| 13 | the route's second command as printed, `git -C <rig>/repo checkout -- docs/decisions/second-decision.md` | 0 | the file is on disk with the planted line; status empty; no commit made |

Steps 7 and 8 are not in Drive C; they commit nothing and were added to read what the door says it is
about to do before it does it.

So the reporter's observation holds in every part: exit 3, the route works as printed, its finalize
commits the staged blob beside the renamed doc, and the summary names one path under `added` and under
`left-out`.

## The drive, previous release

A fresh rig of its own, steps 1 to 5 as above, the same planted line (the same blob id, `7d4653b...`).

| step | command | exit | what it did |
|---|---|---|---|
| 1-4 | as above | 0 each | status `AD docs/decisions/second-decision.md` |
| 5 | `jigc task finalize second-decision` | **0** | `finalized 0656a78 — docs: record the second decision`, `promoted docs/decisions/second-decision.md`, `1 file committed` - no refusal, no left-out list |
| 6 | `git show --name-status HEAD`; `git rev-parse HEAD:docs/decisions/second-decision.md`; `git ls-files -s` | 0 | one path, `A docs/decisions/second-decision.md`, and both `HEAD` and the index hold the ADR's blob `31a7ec9...` there |
| 7 | `git cat-file -t 7d4653b...`; a count of that id in `git ls-files -s` and in `git ls-tree -r HEAD` | 0; 0 and 0 | the planted blob is still an object, and nothing points at it: no index entry, no commit |

On `1.0.0-rc.24` the door exits 0 and the doc replaces the staged blob under its own path. That is the
state `design/finalize.md` -> 4. Promote names as the reason for the guard (*the staged blob, which no
commit then had*), and the regression set's list holds the difference as intended
(`completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv`, the row of
`version_stamp_rollback::a_hook_rejected_finalize_restores_a_staged_blob_at_a_promotion_destination`:
*`task finalize` 0 -> 3*). The candidate's refusal is that ruling, built. The route does not exist on
the previous release, so nothing of it is comparable there.

## Is it what the finding says - the three statements, each against its owner

**1. The route's finalize commits the staged blob.** Owner: `design/finalize.md` -> *Dirty-tree policy*
and -> 5. Stage; `DECISIONS.md` -> *2026-06-20 - M30 planning: finalize change-set scoping*.
*The commit set is the git index, not a sweep of the working tree ... the agent `git add`s its own code
edits as it works; at `finalize` the CLI stages its own managed artifacts into that same index ... then
commits the whole index as one logical change*; and *committed whole (no curated pathspec ...)*. The
blob was put in the index by a `git add` made **after** the task was minted, so it is not carryover
(the carryover gate's subject is what *was staged before the task existed*) and it is the index's
declared touch-set. The task's own composed text says the same at step 1. Intended.

**2. One path under `added` and under `left-out`.** Owner: `design/finalize.md` -> *Dirty-tree policy*,
*Surfaced, not prevented*: *A staged-then-further-modified path appears in **both** lists - its staged
bytes commit, its later worktree delta is left out.* The staged side of the path is an add and its
worktree side is a deletion made after the stage; the commit took the first and left the second, and
the JSON forecast says so in its kinds (`added`, and `deleted` under `left_out`). Intended.

**3. The refusal's sentence, *"is left exactly as it is ... and that is no commit"*.** Owner:
`design/finalize.md` -> 4. Promote, the paragraph after the declared bound: ***Its route makes no
commit** - one made before the boundary moves `HEAD` off the unit's base. A doc under an id its author
chooses takes another id inside the unit (the in-task rename above) and **lands beside what git
holds**. ... Either way the route names the aimed `git checkout` that brings the file back into the
worktree, and leaves what git holds exactly as it is.* Read with its owner, the sentence makes two
statements and both hold as driven:

- *left exactly as it is* - of what the refusal did, and of what the route does to the file's bytes.
  After step 5 the index entry is unchanged; after step 9 it is still `7d4653b...`, and `HEAD` holds
  the same id at the same path. The harm the guard exists for is the opposite one, the blob *replaced*
  and in no commit, which is what the previous release did above.
- *that is no commit* - of the `git checkout` it has just named: `that` is the command in the same
  clause. Step 13: exit 0, the file back on disk, `git log` unchanged. It is the design's *the route
  makes no commit* **before the boundary**; the boundary itself, the route's `jigc task finalize`, is a
  commit by its nature, and the design says of it that the doc *lands beside what git holds*.

The sentence does not say that the staged file stays out of the task's commit, and the door says three
times that it will not: the dry-run forecast (step 7, `added docs/decisions/second-decision.md`), the
print ahead of the commit (step 9, *about to commit the index*) and the landed manifest. Intended.

## Does it break the clause, inside its scope

No, on either arm of the first sharpening.

- **No bytes are destroyed.** The blob staged at step 4 is in commit `397a0be` and in the index under
  the id it was staged with; the task's doc is in the same commit under the id the rename gave it; the
  worktree deletion the user made is still the user's, unstaged, and named. Before the route the blob
  was in no commit; after it, it is in one.
- **No content is committed that the user did not ask for.** The one path in the commit that is not
  the task's doc is a path the user `git add`-ed during the task, which under the agent-stage contract
  is the ask; a reader who wants it out of this commit unstages it, and a file staged before the mint
  is refused under its own code.
- The state is also a planted one - a file staged and then taken off the disk by hand - which the
  sharpening names a declared bound. The verdict does not rest on that: the design answers this state
  on purpose, and the answer was driven.

The finding does not argue that the index-as-commit-set decision or the route's shape is wrong - its
reporter wrote that it *may be exactly as intended* and left the reconciliation open - so `contested`
is false. No regression fact is owed to a refuted verdict; the previous release was driven because
triage asked for both binaries, and its result is above.

## What was driven, and what was not

**Instance, unbounded.** No consumer set was enumerated. Driven: `jigc task finalize` over a `created`
ADR whose home the index holds a blob at and the disk does not, the blob staged after the mint; the
route's rename and re-run in the printed order, with the worktree still missing the file; the route's
checkout afterwards. Not driven: the checkout run before the re-run; the same state under `HEAD` (a
committed doc deleted from the worktree); a blob staged before the mint; a doc under a fixed identity;
`jigc milestone finalize`; `--format json` at the landed finalize. No claim is made about any of them.

No coverage claim is in the finding and none is made here.

## Repro block

The test-in-waiting. **Pinnable as it stands**: a `fresh` fixture, argv steps, and three explicit file
steps (write, `git add`, remove). The previous-release drive is not part of it - a standing test holds
one binary.

```yaml
claim: "the route of finalize.promote-clobber over a staged blob breaks no-lost-files: its finalize loses the blob or commits content nobody asked for"
verdict: REFUTED          # intended
setup:
  - fixture: fresh
  - ["jigc", "start", "--workflow", "record-decision", "second decision"]
  - ["jigc", "doc", "create", "adr", "--title", "Second decision", "--task", "second-decision"]
  - "the ADR's three required slots (context, decision, consequences), the commit doc's type (docs) and summary"   # exit 0 each
  - write: { at: "docs/decisions/second-decision.md", bytes: "PLANTED\n" }
  - ["git", "add", "docs/decisions/second-decision.md"]
  - capture: { staged_blob: ["git", "rev-parse", ":docs/decisions/second-decision.md"] }
  - remove: "docs/decisions/second-decision.md"        # git status: AD
repro:
  - ["jigc", "task", "finalize", "second-decision"]
expect:
  exit: 3
  stdout: ""
  stderr_contains:
    - "blocking · finalize.promote-clobber"
    - "a file is staged there in git's index, in no commit"
    - "jigc doc rename adr:second-decision --to \"<title>\" --task second-decision"
    - "checkout -- docs/decisions/second-decision.md"
  after: "HEAD did not move; the index entry at docs/decisions/second-decision.md is <staged_blob>"
then:                     # the route, as printed
  - ["jigc", "doc", "rename", "adr:second-decision", "--to", "Second decision revised", "--task", "second-decision"]   # exit 0
  - ["jigc", "task", "finalize", "second-decision", "--dry-run", "--format", "json"]
  # exit 0; manifest == [{added, docs/decisions/second-decision.md}, {promoted, docs/decisions/second-decision-revised.md}]
  #         left_out == [{deleted, docs/decisions/second-decision.md}]
  - ["jigc", "task", "finalize", "second-decision"]
  # exit 0; "2 files committed"; the path under `added` and under `left-out`
  - ["git", "show", "--name-status", "--format=", "HEAD"]
  # exactly: A docs/decisions/second-decision-revised.md, A docs/decisions/second-decision.md
  - ["git", "rev-parse", "HEAD:docs/decisions/second-decision.md"]      # == <staged_blob>: every byte kept
  - ["git", "rev-parse", ":docs/decisions/second-decision.md"]          # == <staged_blob>: left as it was
  - ["git", "checkout", "--", "docs/decisions/second-decision.md"]      # exit 0; the file is back; HEAD did not move
pinned-by: "UNPINNED: not established by this verification - no suite's assertions were read"
```

On the previous release the `repro` line exits 0, promotes over the path and leaves `<staged_blob>` in
no commit and no index entry (above): the block's refusal does not exist there.

## Left open

Not pursued. For triage, like any finding.

1. **In the text render the left-out list carries no kind, and its heading advises `git add`.** Seen at
   steps 7 and 9: the path is listed under `left-out (unstaged/untracked — git add to include)` with
   nothing saying that what was left out is a *deletion*, while the `--format json` forecast of the
   same state says `kind: deleted`. Read in text alone, one path is `added` and, two lines on, something
   a `git add` would include. What that `git add` would do here - stage the deletion of the file the
   commit just added - was not driven, and whether the text render owes the kind was not read against
   `design/command-output-contract.md` or `design/surface-contract.md`. Only this one instance was seen.

## The tree

Branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows only
the untracked run directory `completions/artifacts/canary-one/r1/`, as it did before the first drive.
Nothing was built, edited, staged or committed. Two rigs, one per binary, both under one directory of
my own inside the scratch root.

<!-- end of report -->
