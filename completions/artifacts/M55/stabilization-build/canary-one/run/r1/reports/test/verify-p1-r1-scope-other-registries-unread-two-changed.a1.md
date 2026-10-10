# canary-one, round 1, test stage - verify-real of `r1-scope-other-registries-unread-two-changed` (attempt 1)

Verdict: **refuted as a blocker** - basis `breaks-no-clause`. Not contested. The finding stays a row of the ledger.

The two registries did change between the previous release and the candidate, as the scope step says.
The tip adds one row to each. Both rows name refusals; both refusals were driven on the candidate and
every route they print was run as printed and worked. Two commands that exit 0 on the previous release
refuse on the candidate behind these rows, and each of the two is a ruling on record. Nothing driven
here breaks `working-product` inside its scope.

## What was handed

- Ledger key `r1-scope-other-registries-unread-two-changed`. Door: none named - the rows of
  `FINALIZE_FAMILY` and `ENVELOPE_OWED_CODES`, `crates/cli/src/render.rs`. Clause: `working-product`.
  Triage's grade: unclear.
- Its source: `completions/artifacts/canary-one/r1/reports/test/scope.a1.md`, *Left open*, item 2 - two
  line counts and no repro block. The scope step claims no behaviour.
- What triage asked: the rows the tip adds, and whether any, as it stands, breaks the second clause
  within its scope.

No other finding's report and no other verifier's was read.

## The binaries

    $ dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc
    {"act": "hash", "status": "hashed", "path": "bin/c1.a1/jigc", "bytes": 15795744, "content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc", ...}      exit 0
    $ dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc
    {"act": "hash", "status": "hashed", "path": "bin/previous-91834b5e011d/jigc", "bytes": 15240064, "content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d", ...}  exit 0

Both equal the hashes of the `BINARY:` line. With the candidate's directory first on `PATH`,
`command -v jigc` printed the candidate's path (exit 0); for the drives on the previous release its
directory went first instead and `command -v jigc` printed that path. Every rig was built with
`dev/jigc-rig fresh --binary <that path>`, stdout only captured, under a directory of my own minted
with `mktemp -d` inside the scratch root. Nothing under `target/` was driven and nothing was built.

## The clause, and its scope

`DECISIONS.md` -> *2026-10-04 - The exit rule, revised*. The second clause is *a working product others
can use and rely on*; its instrument, by the second sharpening: *no command that works on rc.24 in a
supported layout stops working, and every refusal's route works as printed*. *A finding blocks the
1.0.0 call only if it breaks a clause inside its scope; everything else is recorded with its tier.*
The clause is in the run's closing condition (`opening.md`, `working-product`).

## The comparison, re-driven

The scope step gave two line counts and no command. Reconstructed from nothing: the body of each
constant is every line between its `pub const <NAME>:` line and the next line that is `];`, read with
`git show <commit>:crates/cli/src/render.rs` at `91834b5e011de2c36e2be2b79e96c0b9f60a803c` (base) and
`eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` (tip).

| constant | body lines, base | body lines, tip | rows, base | rows, tip |
|---|---|---|---|---|
| `FINALIZE_FAMILY` | 142 | 153 | 20 | 21 |
| `ENVELOPE_OWED_CODES` | 4 | 10 | 4 | 5 |

The scope step's counts (144 and 155; 6 and 12) are each two higher: they count the constant's opening
and closing lines. The differences are the same, +11 and +6. So its statement reproduces.

What the tip changes, read off `diff` of the two bodies (exit 1 for each constant):

1. **`FINALIZE_FAMILY` gains one row** - `code: "finalize.linked-worktree-doc"`, `producer: "cli::task"`,
   `subject: FinalizeSubject::FilePath`. Ten lines. `git grep -c linked-worktree-doc <base> -- crates/`
   exits 1: the code is new.
2. **`FINALIZE_FAMILY`: one existing row's note is reworded** - `finalize.promote-clobber`'s
   `subject_note` gains *"or a path git still holds though nothing is on disk there"* (2 lines become 3).
   No row is added by it. `subject_note` is declared once (`render.rs:1577`) and read by no other line
   of `crates/` or `tooling-tests/`; the reworded sentence is not in the candidate binary's bytes
   (`grep -c -a`, 0, exit 1). It is a comment in a table.
3. **`ENVELOPE_OWED_CODES` gains one member** - `engine::store::HOME_NOT_REGULAR_FILE`
   (`store.home-not-regular-file`), under five comment lines. `git grep -c home-not-regular-file <base> -- crates/`
   exits 1: the code is new. Membership decides one thing, at `render::carrier`: a refusal under a member
   code takes the findings envelope under `--format json` whichever constructor its door reached for.

10 + 1 = 11 and 5 + 1 = 6. Nothing was removed from either body.

## Drive A - `finalize.linked-worktree-doc`, the row `FINALIZE_FAMILY` gains

Owner of the behaviour: `design/command-output-contract.md`, the family table's `linked-worktree-doc`
row; `design/storage.md` -> *CLI and git* (*only code (never a managed doc) ever rides a worktree*);
`DECISIONS.md` -> *2026-10-04 - rc.24 fix pass, fork on doc work from a user-made linked worktree:
refused at the first write now ... (the human's decision)*, option A, with its stated price: *a mixed
code-and-doc task cannot be done from a linked worktree in 1.0.0*.

Candidate, rig `fresh`, then `git worktree add -q -b feature <rig>/wt` (exit 0), every command below
from `<rig>/wt` unless said:

| step | command | exit | what it did |
|---|---|---|---|
| 1 | `jigc start --workflow single-task "record a decision about caching"` | 0 | minted the task; printed *a task here commits here, and code only* and the route to the main checkout |
| 2 | `jigc doc create adr --title "Cache the index" --task record-a-decision-about-caching` | 1 | `blocking · finalize.linked-worktree-doc`, `at: docs/decisions/cache-the-index.md`, a route |
| 3 | the same with `--format json` | 1 | the findings envelope, `schema_version` 3, one finding, `key` = `{code: finalize.linked-worktree-doc, target: docs/decisions/cache-the-index.md}`, the same route |
| 4 | `git status --porcelain` in the worktree and in the main checkout | 0, 0 | empty both: the refusal wrote nothing |

The route as printed: *`cd <main checkout>`, then `jigc start "<intent>"` - the front door there, which
mints a task or presents the workflows to pick one from. Task `record-a-decision-about-caching` stays
usable here for code - `jigc task finalize record-a-decision-about-caching` commits what you `git add`
on this branch - so a change to both lands as two commits.* Run as printed:

| step | command, from | exit | what it did |
|---|---|---|---|
| 5 | `jigc start "record the caching decision"`, main checkout | 0 | presented the thirteen selectable workflows and the re-run form |
| 6 | `jigc start --workflow single-task "record the caching decision"`, main checkout | 0 | `task minted: record-the-caching-decision` |
| 7 | `jigc doc create adr --title "Cache the index" --task record-the-caching-decision` | 0 | `adr:cache-the-index` |
| 8 | three `jigc doc set-slot adr:cache-the-index#<context, decision, consequences>`, `jigc doc set-field commit:...#type --value docs`, `jigc doc set-slot commit:...#summary` | 0 each | staged |
| 9 | `jigc task finalize record-the-caching-decision`, main checkout | 0 | `finalized 7bea720 - docs: record the caching decision`, `promoted docs/decisions/cache-the-index.md`, 1 file committed; tree clean |
| 10 | in the worktree: a file `cache.rs`, `git add cache.rs`, the commit doc's type and summary, `jigc task finalize record-a-decision-about-caching` | 0 | `finalized dc003e2 - feat: add the cache`, 1 file committed, *committed in the linked worktree ... on branch `feature`*; tree clean |

Both halves of the route work as printed: the doc landed on the main checkout's branch, the code on the
worktree's, two commits.

The row's second door, the mint over the doc-only commit model:

| step | command, from | exit | what it did |
|---|---|---|---|
| 11 | `jigc start --workflow report-inconsistency "two docs disagree"`, worktree | 1 | `blocking · finalize.linked-worktree-doc`, `at:` the worktree's path, no task minted (`jigc task list` unchanged) |
| 12 | its route as printed - `cd <main checkout>`, then `jigc start --workflow report-inconsistency "two docs disagree"` | 0 | `task minted: two-docs-disagree` |

Seen on the way and conforming to the contract row: `jigc start --workflow record-decision "third decision"`
from the worktree exits 0 and mints. `record-decision` does not include the doc-only finalize step
(four workflows do: `report-inconsistency`, `report-jigc-feedback`, `triage-inconsistency`,
`triage-jigc-feedback`), so by the row it is an ordinary workflow: the mint states the rule, which it
did, and the first promoting write refuses.

The previous release, a fresh rig of its own, the same steps 1 and 2 from a worktree made the same way:
`jigc start --workflow single-task ...` exit 0, `jigc doc create adr --title "Cache the index" --task ...`
**exit 0**, printing `adr:cache-the-index`. So this command ran on rc.24 and refuses on the candidate.
That is the ruling's option A, built: the difference is the decision, and the decision names its price.

## Drive B - `store.home-not-regular-file`, the member `ENVELOPE_OWED_CODES` gains

Owner of the behaviour: `DECISIONS.md` -> *2026-10-06 - One code for a managed home that is not an
ordinary file* (the human's second ruling on the fix pass's items 7 and 8: *at seven ... the
`--format json` arm moves from the flattened `{"error": ...}` to the findings envelope, because the code
joins `cli::render::ENVELOPE_OWED_CODES`*); `design/command-output-contract.md` -> *A fifth act*;
`design/finalize.md` -> 4. Promote, *one code*. That a link at a home is refused at all is the fix
pass's `(R6, D-7)`, a tier-1 row of the rc.24 gate.

Candidate, rig `fresh`, then `mkdir -p docs/milestone-records` and
`ln -s nowhere.md docs/milestone-records/ship-it.md` (a dangling link at the record's home):

| step | command | exit | what it did |
|---|---|---|---|
| 1 | `jigc milestone create "ship it" --format json` | 1 | stdout empty; stderr the findings envelope, `schema_version` 3, one finding, `key` = `{code: store.home-not-regular-file, target: docs/milestone-records/ship-it.md}`, a route |
| 2 | `jigc milestone create "ship it"` | 1 | `blocking · store.home-not-regular-file`, `at: docs/milestone-records/ship-it.md`, the same route |
| 3 | `readlink docs/milestone-records/ship-it.md`; `git status --porcelain`; `git log` | 0 | the link stands (`nowhere.md`), no `nowhere.md` was written, no commit was made, no `.jigc/milestones` |
| 4 | control - an older member of the set: `jigc doc show adr:nope --format json` | 1 | stdout empty; stderr the same envelope shape under `store.not-found` - the new member is carried as the old ones are |

The route as printed: *create this milestone under a different title, or move the link out of the
record's home, so that `docs/milestone-records/ship-it.md` is free, and re-run this create.* Both:

| step | command | exit | what it did |
|---|---|---|---|
| 5 | `jigc milestone create "ship it two" --format json` | 0 | `minted milestone:ship-it-two`, record commit `91a6a73`, one file |
| 6 | `mv docs/milestone-records/ship-it.md <rig>/moved-link`, then `jigc milestone create "ship it" --format json` | 0, 0 | `minted milestone:ship-it`, record commit `824e317`; the home is a regular file; tree clean |

One of the five record commands, in the same rig: the committed record moved to `<rig>/elsewhere/` and a
link to it left at its home (`git status`: ` T docs/milestone-records/ship-it.md`).

| step | command | exit | what it did |
|---|---|---|---|
| 7 | `jigc milestone add-task ship-it "first sub task" --format json` | 1 | stdout empty; stderr the envelope, `key` = `{code: store.home-not-regular-file, target: docs/milestone-records/ship-it.md}`; the link stands |
| 8 | its route as printed - `git -C <repo> checkout HEAD -- docs/milestone-records/ship-it.md`, then the same command again | 0, 0 | the record is a regular file again; `added task:first-sub-task to milestone:ship-it`, record commit `9b8849f`; tree clean |

The previous release, a fresh rig of its own. Control first, no link: `jigc milestone create "ship it two" --format json`
exit 0, the same ack shape the candidate prints at step 5 - the command that works there works here.
Then the same plant and step 1: **exit 0**, `minted milestone:ship-it`, record commit `dbc25aa` - and
afterwards `git ls-files -s` holds `docs/milestone-records/ship-it.md` at mode `120000`, the link stands,
and `docs/milestone-records/nowhere.md` exists untracked. rc.24 committed the link in the record's place
and wrote the record through it into a file nobody named. That exit 0 is the defect the fix pass was
ruled to close, not a command that worked; the candidate's refusal is the ruling, built.

## Drive C - the reworded note's behaviour, `finalize.promote-clobber` over a path git holds

Not an added row; driven once because the body changed there. Owner: `design/finalize.md` -> 4. Promote,
*A home git holds is occupied*; the regression set's list already holds this difference as intended
(`completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv`, the row of
`version_stamp_rollback::a_hook_rejected_finalize_restores_a_staged_blob_at_a_promotion_destination`).

Candidate, the main checkout of Drive A's rig: `jigc start --workflow record-decision "second decision"`
(0), `jigc doc create adr --title "Second decision" --task second-decision` (0), its three slots and the
commit doc's type and summary (0 each); then the plant - a file written at `docs/decisions/second-decision.md`,
`git add` of it, and the file removed from disk (`git status`: `AD`).

| step | command | exit | what it did |
|---|---|---|---|
| 1 | `jigc task finalize second-decision` | 3 | `blocking · finalize.promote-clobber`, `at: docs/decisions/second-decision.md`; nothing committed |
| 2 | its route as printed - `jigc doc rename adr:second-decision --to "Second decision revised" --task second-decision`, then `jigc task finalize second-decision` | 0, 0 | `finalized 0a1f2df`, `promoted docs/decisions/second-decision-revised.md` |
| 3 | the route's second command as printed - `git -C <repo> checkout -- docs/decisions/second-decision.md` | 0 | the file is back in the worktree with the staged bytes; no commit |

The route works as printed. What step 2's commit also carried is under *Left open*, item 2.

## Is it what the finding says, and does it break the clause

1. **The statement reproduces.** Two registries of finding codes changed; the tip adds one row to each
   and rewords one note. So the basis is not `does-not-reproduce`.
2. **No command that works on rc.24 in a supported layout stops working, by these rows.** Two commands
   that exit 0 on rc.24 refuse on the candidate behind them. `jigc doc create` of a promoting doc from a
   user-made linked worktree is refused by the human's decision of 2026-10-04, which states that price in
   its own words. `jigc milestone create` over a link at the record's home is refused by the fix pass's
   `(R6, D-7)` and carried under one code by the human's ruling of 2026-10-06; on rc.24 that exit 0
   committed a link in the record's place. Each is behaviour a settled decision intends. The plain
   `jigc milestone create` still exits 0 on both binaries with the same ack.
3. **Every refusal's route works as printed, at the doors driven.** Five refusals, seven printed routes
   or route halves, all run as printed, all exit 0, each leaving the state the route promised.
4. **The finding does not argue that either decision is wrong** - it argues nothing about behaviour. So
   `contested` is false.

A real change, on record, that breaks no clause inside its scope: `refuted` as a blocker, basis
`breaks-no-clause`. The row stays in the ledger; the registries stay on neither side of the round's
scope, which is the human's scope as set and not this verdict's to move.

## What was driven, and what was not

**Instance, unbounded.** No consumer set was enumerated. Driven on the candidate: for
`finalize.linked-worktree-doc`, the write door at `jigc doc create` (text and json) and the mint door at
`jigc start --workflow report-inconsistency`; for `store.home-not-regular-file` under the owed set,
`jigc milestone create` and `jigc milestone add-task` over a symbolic link; for the reworded note, one
finalize over a staged blob with nothing on disk. Not driven: the other `jigc doc` write leaves and
`jigc doc author`; the staged-doc positions of the first code at `task finalize`, `task validate`,
`--dry-run` and bare `jigc start`, with their two declined-landing routes; `jigc migrate` as a mint; a
detached worktree; a worktree with code already staged (the longer route); jigc's own fan-out
worktrees; `milestone add-from-spec`, `milestone discard`, a sub-task's `task discard`, the record at
`milestone finalize` and `jigc rename`'s destination; a directory or a special file at a home. No claim
is made about any of them.

No coverage claim is in the finding and none is made here. Three suites name these codes -
`linked_worktree_doc_home`, `home_shape_one_code`, `finalize_family_registry` - and what they assert was
not read, beyond that `home_shape_one_code`'s `milestone_create` drives the plant of Drive B, step 2.

## Repro block

The test-in-waiting. **Pinnable as it stands**: a `fresh` fixture, argv steps, one `git worktree add`
and one `symlink` as explicit setup steps. The source comparison above is not part of it - it reads two
commits of history, which a standing test cannot hold.

```yaml
claim: "the rows the tip adds to FINALIZE_FAMILY and ENVELOPE_OWED_CODES break working-product: a refusal under one of them prints a route that does not work"
verdict: REFUTED          # breaks-no-clause
blocks:
  - name: linked-worktree-doc, the write door and its route
    setup:
      - fixture: fresh
      - ["git", "worktree", "add", "-q", "-b", "feature", "<rig>/wt"]
      - cwd: "<rig>/wt"
      - ["jigc", "start", "--workflow", "single-task", "record a decision about caching"]
    repro:
      - ["jigc", "doc", "create", "adr", "--title", "Cache the index", "--task", "record-a-decision-about-caching", "--format", "json"]
    expect:
      exit: 1
      stderr_json: { "schema_version": 3, "findings": [ { "key": { "code": "finalize.linked-worktree-doc", "target": "docs/decisions/cache-the-index.md" } } ] }
      route_names: ["cd <main checkout>", "jigc start \"<intent>\"", "jigc task finalize record-a-decision-about-caching"]
      after: "git status --porcelain is empty in the worktree and in the main checkout"
    then:                 # the route, as printed
      - cwd: "<rig>/repo"
      - ["jigc", "start", "--workflow", "single-task", "record the caching decision"]          # exit 0
      - ["jigc", "doc", "create", "adr", "--title", "Cache the index", "--task", "record-the-caching-decision"]   # exit 0
      - "the three required slots, the commit doc's type and summary"                          # exit 0 each
      - ["jigc", "task", "finalize", "record-the-caching-decision"]                            # exit 0, promotes docs/decisions/cache-the-index.md
      - cwd: "<rig>/wt"
      - "a staged code file, the commit doc's type and summary"
      - ["jigc", "task", "finalize", "record-a-decision-about-caching"]                        # exit 0, one file, on branch feature
  - name: linked-worktree-doc, the mint door and its route
    setup:
      - fixture: fresh
      - ["git", "worktree", "add", "-q", "-b", "feature", "<rig>/wt"]
      - cwd: "<rig>/wt"
    repro:
      - ["jigc", "start", "--workflow", "report-inconsistency", "two docs disagree"]
    expect:
      exit: 1
      stderr_contains: ["blocking · finalize.linked-worktree-doc", "jigc start --workflow report-inconsistency \"<intent>\""]
      after: "jigc task list holds no task of that intent"
    then:
      - cwd: "<rig>/repo"
      - ["jigc", "start", "--workflow", "report-inconsistency", "two docs disagree"]            # exit 0, task minted
  - name: home-not-regular-file is keyed on the wire at milestone create, and both routes work
    setup:
      - fixture: fresh
      - symlink: { at: "docs/milestone-records/ship-it.md", to: "nowhere.md" }   # dangling
    repro:
      - ["jigc", "milestone", "create", "ship it", "--format", "json"]
    expect:
      exit: 1
      stdout: ""
      stderr_json: { "schema_version": 3, "findings": [ { "key": { "code": "store.home-not-regular-file", "target": "docs/milestone-records/ship-it.md" } } ] }
      after: "the link stands, docs/milestone-records/nowhere.md does not exist, HEAD did not move"
    then:
      - ["jigc", "milestone", "create", "ship it two", "--format", "json"]                      # exit 0, a record commit
      - "move the link out of docs/milestone-records/"
      - ["jigc", "milestone", "create", "ship it", "--format", "json"]                          # exit 0, the home is a regular file
  - name: home-not-regular-file is keyed on the wire at a record command, and its route works
    setup:
      - fixture: fresh
      - ["jigc", "milestone", "create", "ship it"]
      - "move docs/milestone-records/ship-it.md outside the repository and leave a link to it at its home"
    repro:
      - ["jigc", "milestone", "add-task", "ship-it", "first sub task", "--format", "json"]
    expect:
      exit: 1
      stdout: ""
      stderr_json: { "findings": [ { "key": { "code": "store.home-not-regular-file", "target": "docs/milestone-records/ship-it.md" } } ] }
    then:
      - ["git", "checkout", "HEAD", "--", "docs/milestone-records/ship-it.md"]                  # exit 0
      - ["jigc", "milestone", "add-task", "ship-it", "first sub task", "--format", "json"]     # exit 0, a record commit
pinned-by: "UNPINNED: not established by this verification - three suites name the codes and their assertions were not read"
```

On the previous release the first and third blocks' `repro` lines exit 0 (above); the block is not
comparable there as a pass or a fail, since neither code exists in that binary (`grep -c -a` of each
code over its bytes: 0, exit 1).

## Left open

Neither was pursued. Each is for triage, like any finding.

1. **The second clause's instrument holds no row for the two ruled differences driven here.** The list
   `intended-changes.tsv` has 61 lines and none names either state (`grep -c -i` of `linked.worktree`: 0;
   of `home-not-regular`, `not a regular` or `symlink`: 0). That is what its header says it should hold -
   a row is owed only for a test of rc.24's own suite that goes red, and none drives either state. The
   part of the regression set in which such a difference would surface, success paths per command on
   both binaries, is recorded as not built (`DECISIONS.md` -> *2026-10-06 - The regression set's first
   part, as a tool*). So *`jigc doc create` from a user-made linked worktree: exit 0 on rc.24, exit 1 on
   the candidate* is held by its ruling and by no row of the instrument. Whether it owes one is not this
   verdict's.
2. **The route of `finalize.promote-clobber` over a staged blob, and what its finalize commits** (Drive C,
   step 2; a planted state). The refusal says of the file git holds: *is left exactly as it is ... and
   that is no commit*. The route's own `jigc task finalize` then committed that staged blob in the
   task's commit - `added docs/decisions/second-decision.md`, *2 files committed* - and its summary lists
   the same path under `added` and under `left-out`. Finalize commits the index by design, and the blob
   was staged after the task was minted, so this may be exactly as intended; the two sentences were not
   reconciled against `design/finalize.md` here.

## The tree

Branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows only
the untracked run directory `completions/artifacts/canary-one/r1/`, as it did before the first drive.
Nothing was built, edited, staged or committed. Four rigs, two per binary, all under one directory of
my own inside the scratch root.

<!-- end of report -->
