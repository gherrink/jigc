# verify-real — `r1-p3-git-capture-commit-message-sites-not-driven`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed by triage with the
grade *unclear*: the five commit-message call sites of `task::git_capture` were not driven
with a commit message that is not UTF-8, and the clause said to be at risk is `no-lost-files`.

## Verdict in one paragraph

**Refuted as a blocker — `breaks-no-clause`.** The five sites were driven, on the candidate
and on the previous release, and the mechanism is real: four of the five were reached with a
capture that does not decode, and three of the four misbehave. None of it destroys a byte no
git object holds, and none of it commits content the user did not ask for. With the state
triage named — a `HEAD` commit whose message is not UTF-8 — `jigc task finalize`, the amend
arm's committing run and `jigc milestone finalize` all exit 0 with a correct commit, and the
one non-zero exit (the amend arm's `--dry-run`) comes with `HEAD` unmoved and nothing written.
A non-zero exit *after* `HEAD` moved does exist, and I reached it only by changing the setup
(git asked for Latin-1 log output): there the commit that landed is the right one, the exit is
1, and the previous release does the same. The defects are real and are listed under *Left
open*; this row stays a row of the ledger, and it is not a `does-not-reproduce`.

One cell sits nearest the clause and is the human's to weigh if the clause is read wider than
its sharpening: on the amend arm, run by a human, a `HEAD` message git cannot print as UTF-8
loses the profile's co-author trailer in the rewritten commit, at exit 0. I graded it outside
the clause for the reasons under *Does it break `no-lost-files`*.

## The binaries, asserted before anything was driven

| | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` |
|---|---|---|
| candidate `c1`, commit `eeffe347` | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` |
| previous release, `1.0.0-rc.24` | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` |

Both match the line I was handed. The candidate's directory went first on `PATH` and
`command -v jigc` printed the candidate's path; every driver below prints the same line again
at its top. For the previous release's cells the previous release's directory was first on
`PATH` instead, and the driver printed that path. Every rig was built with
`dev/jigc-rig fresh --binary <that path>`, one rig per cell, shared with no one.

## The enumeration — which sites, and how I counted

`grep -rn "git_capture(" crates/cli/src` at `eeffe347`, the definition excluded: **36 call
sites** in 6 files (`task.rs` 23, `milestone.rs` 8, `rename.rs` 2, `ingest.rs` 1, `orphan.rs`
1, `setup.rs` 1). Read for a format that prints a commit message (`%s`, `%B`): **5**, the five
the finding names.

| # | site | git call | which commit it reads | what an undecodable capture does |
|---|---|---|---|---|
| 1 | `task.rs:3957` | `log -1 --pretty=format:%s <pinned>` | the commit an amend task rewrites — any commit, a foreign one included | `?` — the amend arm's `--dry-run` fails |
| 2 | `task.rs:4196` | `log -1 --pretty=format:%s` | the commit this finalize has just made | `?` — fails after the commit landed |
| 3 | `task.rs:9191` | `log -1 --format=%B HEAD` | the commit an amend rewrites, outside an agent session only | `.ok()?` — swallowed; no trailer is carried |
| 4 | `milestone.rs:9474` | `log -1 --pretty=format:%s` | the aggregate commit the boundary has just made | `?` — would fail after the boundary landed |
| 5 | `milestone.rs:9616` | `show --no-patch --format=%h%n%s <sha>` | every commit the boundary has just made | `?` — fails after the boundary landed |

The step they share is `String::from_utf8(out.stdout)` in `git_capture` (`task.rs:10100`).

Not one of the five: a sixth place where a commit subject reaches that step, hit on the way to
`milestone finalize` and not pursued — `git worktree add` prints `HEAD is now at <sha>
<subject>` on its standard output, and `jigc milestone provision` captures it (*Left open*,
item 5).

## What I changed in the setup, and why

**1. `git commit-tree` with raw bytes does not make the state.** On git 2.54.0 it warns and
re-encodes: the bytes `caf<0xE9>` went in, and the stored object and `git log -1 --format=%s`
both hold `caf<0xC3 0xA9>` — valid UTF-8 (read back with a strict decoder, exit 0). A probe
run with that plant would have driven nothing and read as green. So the plant writes the
commit object itself: the header lines of an ordinary commit, then the message bytes
`caf<0xE9> latin1 subject`, a blank line, `body bytes <0xFF 0xFE> here`, stored with `git
hash-object -t commit -w` and put on `HEAD` with `git update-ref`. The control ran with every
plant: `git log -1 --format=%B` read back and handed to a strict UTF-8 decoder, which must
fail. `git fsck` exits 0 over a repository so planted (run once, on the first rig).

**2. An amend refuses a commit that changes nothing.** My first plant reused the parent's tree,
and `git commit --amend` refused it as an empty commit — an artifact of the plant, not of the
message. The plant now commits one file first and replaces that commit, tree and parent kept.

**3. The state triage named cannot reach sites 2, 4 and 5.** They read commits jigc itself has
just made, whose messages jigc renders from a Rust string. To reach them with bytes that do
not decode I used a second setup, with no planted object: the repository's git configuration
asks for Latin-1 log output (`git config i18n.logOutputEncoding ISO-8859-1`) and the task's
summary holds one letter Latin-1 can express (`add the caf<U+00E9> file`). Git then re-encodes
the subject on the way out. Whether that configuration is inside the clause's *ordinary git
configuration* is not mine to settle (*Left open*, item 10); it is the smallest setup I found
that reaches the door, and I say so rather than leave three of five sites undriven.

## What was driven

Every cell: a `fresh` rig of its own; an untracked `keep.txt` as the witness; each exit read
bare, standard output and standard error to separate files.

### The state triage named — a `HEAD` commit whose message is not UTF-8

| cell | candidate | previous release |
|---|---|---|
| **`jigc task finalize`** — `single-task`, one staged `code.txt` | `--dry-run` 0; finalize **0**. `HEAD` moved by one commit, its parent the planted one, holding `code.txt` alone. `keep.txt` stands. Site 2 read jigc's own subject. (Run twice: once over the first plant, once over the second.) | the same exits and state |
| **amend arm, agent session** (`CLAUDECODE` set), planted message carries the profile's trailer | `task amend` 0, ack `amending: <sha> (no subject line)`. `--dry-run` **1**: `` `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 3 `` — no code, no route; `HEAD` the same, task area intact. Finalize **0**: `HEAD` rewritten, tree identical, message `fix: reworded subject` plus the trailer (signed on the session's basis; site 3 not reached). | the same |
| **amend arm, human channel** (`CLAUDECODE` unset on every call) | the same exits. Finalize **0**: `HEAD` rewritten, tree identical — and the message is `fix: reworded subject` **with no trailer**. Site 3 swallowed the failed read. | the same, the trailer absent |
| control for the row above: the same commit with a UTF-8 message carrying the trailer | `--dry-run` 0, finalize 0, the trailer **carried** | not run |
| **`jigc milestone finalize`**, squashed (the default) — plant, then `create`, `add-task`, `provision`, one staged `alpha.txt` in the sub-task worktree, `join` | `create` 0, `add-task` 0, `provision` **1** (item 5 of *Left open*; its printed route, the re-run, exits 0), `join` 0, finalize **0**: `HEAD` moved by one commit holding `alpha.txt` and the record; the worktree removed; `keep.txt` stands. Sites 4 and 5 read jigc's own subjects. (Run twice.) | the same |

One more read on the candidate, in the amend cell over the first plant: `jigc task validate
<task>` exits 0 and says the task validates clean, immediately before the `--dry-run` that
exits 1.

### The second setup — git asked for Latin-1 log output, no planted object

| cell | candidate | previous release |
|---|---|---|
| **`jigc task finalize`**, summary `add the caf<U+00E9> file` | `--dry-run` 0, forecasting the subject correctly. Finalize **1**, standard error `` `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 17 `` — **after `HEAD` moved**: the commit is there, holds `code.txt` alone and the right message; the task area is gone; `keep.txt` stands. The re-run exits 1, `finalize.no-task`. Site 2. | the same |
| **`jigc milestone finalize`**, `finalize.fan-out.squash` false, the sub-task's summary as above | finalize **1**, the same bare line (index 25) — **after `HEAD` moved by two commits**, the sub-task's and the aggregate, both correct. `alpha.txt` and `keep.txt` stand; the milestone and task areas are gone; the sub-task's worktree is **still registered** under `.jigc/worktrees/`. The re-run exits 1, `milestone.terminal`. Site 5. | the same |
| **amend, human channel**, over the commit the first row made in an agent session | `task amend` 0, ack `(no subject line)`; `--dry-run` **1**, the bare line; finalize **0** — the rewritten message has **lost the trailer** the commit carried. Sites 1 and 3, with nothing planted. | not run |

Site 4 was not reached with an undecodable capture in either setup: the aggregate subject is
`Finalize milestone <slug> (<n> sub-task)`, ASCII by construction.

## Is it what the finding says?

The finding says the sites were not driven; they now are, and what it implied — that an
undecodable message at one of them could cost a file or land a wrong commit — is not what
happens. What happens, per site:

- **Site 1** — a reader fails. Exit 1, nothing written, `HEAD` unmoved. The surface is poor (a
  bare line, no finding code, no route, while `task validate` says clean and the committing
  run then succeeds), which is a matter for another clause.
- **Sites 2 and 5** — the ack fails after a correct commit. The milestone door runs its
  teardown ahead of the landed summary, and its own source comment states the consequence that
  order was chosen for: *the commit is already truth, so a summary that then fails to render
  leaves no byte behind it*. Driven, that holds. The exit is a false red, and on the
  unsquashed milestone arm a worktree is left registered.
- **Site 3** — the swallowed read drops a trailer the design says is carried
  (`design/assistant-adapter.md` → *The co-author trailer* → *The amend arm*: *a human
  repairing an agent's message keeps the agent's credit*). Real, at exit 0, on both binaries.
- **Site 4** — not reachable by a message jigc renders.

## Does it break `no-lost-files`, inside that clause's scope?

The clause's scope (`DECISIONS.md` → *2026-10-04 — The exit rule, revised*, sharpening 1): in a
healthy repository used as documented, *no jigc command at exit 0 destroys bytes no git object
holds or commits content the user did not ask for*; *where jigc cannot tell … it refuses
before writing*; deliberately planted states are declared bounds.

- **Bytes destroyed.** None, in any cell on either binary. `keep.txt` stood every time; the
  work tree after each run is the commit plus the witness; an amended commit stays reachable
  in the reflog. The task and milestone areas torn down are the ones a landed commit always
  tears down, and their content is in the commit.
- **Content committed that was not asked for.** None. Every commit that landed holds exactly
  the staged file (and, at the milestone door, the record), under the message the task
  authored.
- **Refuses before writing.** That sentence is about a write jigc cannot judge. At sites 2 and
  5 the write was judged on complete information and is correct; what fails is the read-back
  for the ack. It is a wrong exit code, not a wrong write. It is also not new: the previous
  release gives the same exit in the same cell.
- **The trailer.** The rewritten commit is the message the user authored, exactly; what is
  missing is an attribution line the adapter owns. The design's own vocabulary for a missing
  trailer is *an omission, never a false claim* (the same section, *Declared bounds*). Nothing
  unasked was committed and no byte was destroyed — the superseded commit, trailer and all, is
  in the object store. With a planted object the state is a deliberately planted one; without
  one it needs git configured for a non-UTF-8 log encoding.

So no sentence of the clause is broken in any driven cell. If the human reads *update
incorrect things* to cover a dropped attribution line, the third row of the second table is
the cell that would count, and item 2 of *Left open* carries it to triage.

## The regression fact

Not owed with this verdict. For the record, every cell run on both binaries came out the same
on both: none of this is new in the candidate.

## Class

`instance, unbounded` for the mechanism at large. Within the finding's own axis — the
commit-message sites of `git_capture` — I enumerated by the search stated above: 5 of 36 call
sites, of which 4 were reached with an undecodable capture and 1 (site 4) could not be. I did
not enumerate the capture helpers other than `git_capture`, nor the git calls whose output
carries a commit subject without a format placeholder; the one I met is item 5.

## Coverage

From the suites, not from the diff: no file under `crates/cli/tests`, `crates/engine/tests` or
`tooling-tests` names `logOutputEncoding`, `commitEncoding` or the refusal's text (`produced
non-UTF-8`) — 0 files each. `hash-object` appears in 3 suite files (prose twice over, and one
`-w` over a blob) and `commit-tree` in 2 (one a comment, one with `-m rewritten`); none writes
a commit object with a message of its own bytes. No test drives any of the five sites with a
message that does not decode.

## Repro VR-GCMSG-1

```yaml
claim: "a commit message that is not UTF-8 at one of the five commit-message sites of task::git_capture loses a file or lands a wrong commit — breaking no-lost-files"
verdict: "REFUTED as a blocker — breaks-no-clause. The undecodable capture reproduces at four of the five sites; no byte is destroyed and no unasked content is committed, on either binary."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; the previous release, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, gives the same exits in every cell run on both"
plant:                      # `git commit-tree` re-encodes the bytes; write the object
  - ["git", "add", "planted.txt"]                       # one line, `planted`
  - ["git", "commit", "-q", "-m", "placeholder"]
  - "object := `tree <HEAD^{tree}>` LF `parent <HEAD~1>` LF author line LF committer line LF LF"
  - "          then the bytes: c a f 0xE9 ` latin1 subject` LF LF `body bytes ` 0xFF 0xFE ` here` LF"
  - "          (the trailer cells add: LF, then the profile's co-author trailer line, LF)"
  - ["git", "hash-object", "-t", "commit", "-w", "<object file>"]
  - ["git", "update-ref", "HEAD", "<that sha>"]
  - control: "`git log -1 --format=%B` does NOT decode as UTF-8"
cells:
  - cell: ordinary                       # pin: a fact to stay true
    setup:
      - fixture: fresh                   # dev/jigc-rig fresh --binary <binary>
      - plant
      - ["jigc", "start", "--workflow", "single-task", "add a code file"]
      - "write code.txt and `git add` it; write keep.txt and do not add it"
      - ["jigc", "doc", "set-field", "commit:add-a-code-file#type", "--value", "feat", "--task", "add-a-code-file"]
      - ["jigc", "doc", "set-slot", "commit:add-a-code-file#summary", "--from-file", "<file: add a code file>", "--task", "add-a-code-file"]
    repro:
      - ["jigc", "task", "finalize", "add-a-code-file"]
    expect:
      exit: 0
      stdout_contains: "feat: add a code file"
      head: "one commit past the planted one; `git show --name-status --format= HEAD` is `A code.txt`"
      file_present: "keep.txt"
  - cell: amend-dry-run                  # pin the two state facts; the exit and the text are a defect (Left open, 1)
    setup:
      - fixture: fresh
      - plant
      - ["jigc", "task", "amend"]        # mints amend-<sha7>
      - ["jigc", "doc", "set-field", "commit:<task>#type", "--value", "fix", "--task", "<task>"]
      - ["jigc", "doc", "set-slot", "commit:<task>#summary", "--from-file", "<file: reworded subject>", "--task", "<task>"]
    repro:
      - ["jigc", "task", "finalize", "<task>", "--dry-run"]
    expect:
      head: "unchanged — the planted sha"
      dir_present: ".jigc/tasks/<task>"
    observed_not_pinned:
      exit: 1
      stdout: ""
      stderr_contains: "`git` produced non-UTF-8 output"
  - cell: amend                          # pin: the rewrite lands and the tree does not move
    setup: "as amend-dry-run"
    repro:
      - ["jigc", "task", "finalize", "<task>"]
    expect:
      exit: 0
      stdout_contains: "fix: reworded subject"
      head: "a new sha whose tree is the planted commit's tree; `git log -1 --format=%s` is `fix: reworded subject`"
      file_present: "keep.txt"
    observed_not_pinned: "with CLAUDECODE unset and the planted message carrying the trailer, the rewritten message carries none (Left open, 2); the same commit with a UTF-8 message carries it"
  - cell: milestone                      # pin: the boundary lands over such a history
    setup:
      - fixture: fresh
      - plant
      - ["jigc", "milestone", "create", "Batch one"]
      - ["jigc", "milestone", "add-task", "batch-one", "add alpha"]
      - ["jigc", "milestone", "provision", "batch-one"]   # observed: exit 1 (Left open, 5)
      - ["jigc", "milestone", "provision", "batch-one"]   # its printed route; exit 0
      - "in .jigc/worktrees/add-alpha: `jigc workflow sub-task --task add-alpha`, write alpha.txt, `git add` it"
      - ["jigc", "milestone", "join", "batch-one"]
    repro:
      - ["jigc", "milestone", "finalize", "batch-one"]
    expect:
      exit: 0
      stdout_contains: "Finalize milestone batch-one (1 sub-task)"
      head: "one commit past the record commits, holding alpha.txt and docs/milestone-records/batch-one.md"
      file_present: "keep.txt"
observed: "<scratch>/vr-gcmsg.KTmUIE — cand-<cell>/ and prev-<cell>/ (each command's stdout and stderr in its own file), cand-<cell>.log and prev-<cell>.log, B2/ and C/ and Cc/ (the candidate's amend cells and the UTF-8 control), E/ and F/ and cand-enc-amend/ (the second setup), and the drivers themselves: plant2.sh, ordinary.sh, amend-arm.sh, milestone.sh, enc-finalize.sh, enc-milestone.sh, enc-amend.sh, cell.sh, cell2.sh"
pinned-by: "UNPINNED: a verifier writes no test — the four cells convert as they stand"
```

**Pinnable as it stands: yes** — the `expect` of the four cells. The plant is a file of bytes
and two git plumbing calls, so it is portable. Two cautions for whoever converts it: the suite
runs every process under an empty `CLAUDECODE`, so the amend cell lands on the human channel
unless the test sets the variable; and the fields under `observed_not_pinned` are defects as
observed — pinned green they would hold the defects in place.

The second setup is not in the block: every cell of it is a defect as observed (a fix's red
test in waiting, its `expect` inverted), and whether its configuration is in scope is open.
Its argv is `enc-finalize.sh`, `enc-milestone.sh` and `enc-amend.sh` above: `git config
i18n.logOutputEncoding ISO-8859-1`, then the ordinary flow with the summary `add the
caf<U+00E9> file`.

## Left open

Hit on the way, not pursued. Each is for triage, on its own.

1. **The amend arm's `--dry-run` over a commit whose subject git cannot print as UTF-8** — exit
   1, the bare line `` `git` produced non-UTF-8 output: … ``, no finding code, no route, while
   the committing run exits 0 (both binaries) and `jigc task validate` says clean at exit 0
   (candidate, one run). `task.rs:3957`.
2. **The amend arm drops the profile's co-author trailer when `HEAD`'s message does not
   decode**, on the human channel, at exit 0, with no word about it — against
   `design/assistant-adapter.md` → *The co-author trailer* → *The amend arm*. Driven with a
   planted object on both binaries, and with nothing planted (Latin-1 log output, the commit
   jigc's own) on the candidate. `task.rs:9191`, the `.ok()?`.
3. **`jigc task finalize` exits 1 after its commit landed** when git prints the new subject in
   a non-UTF-8 encoding: the commit is correct, the task area is gone, the ack never prints,
   and the re-run answers `finalize.no-task`. `--dry-run` forecasts exit 0. Both binaries.
   `task.rs:4196`.
4. **`jigc milestone finalize`, unsquashed, exits 1 after two commits landed** in the same
   configuration, and **leaves the sub-task's worktree registered** under `.jigc/worktrees/`
   with no milestone left that owns it (the re-run answers `milestone.terminal`). What removes
   that worktree was not driven. Both binaries. `milestone.rs:9616`.
5. **`jigc milestone provision` over a base commit whose subject is not UTF-8** exits 1 with
   `milestone.provision-failed` saying *0 of 1 worktree(s) landed* while `git worktree list`
   shows the worktree registered at the base. The printed route (the re-run) exits 0 and
   reports it provisioned. A sixth place a commit subject reaches the UTF-8 step, through `git
   worktree add`'s standard output. Both binaries.
6. **`jigc task amend`'s ack prints `(no subject line)`** for a `HEAD` that has a subject line
   git cannot print as UTF-8 — the line the design puts above the instructions so the reader
   sees what is about to be replaced. Both binaries.
7. **An amend of a commit that changes nothing against its parent**: git refuses (*would make
   it empty*), and jigc's frame ends *Fix the hook's complaint* where no hook ran. Candidate
   only, one run, met through my first plant.
8. **Site 4, `milestone.rs:9474`**, was not reached with an undecodable capture. A `commit-msg`
   hook that rewrites the message is the route I can see; not driven.
9. **`i18n.commitEncoding` set to a non-UTF-8 encoding** — jigc hands git UTF-8 bytes, which
   git would then label with that encoding. Not driven.
10. **Whether a non-UTF-8 `i18n.logOutputEncoding` is inside the first clause's *ordinary git
    configuration*.** The scope's examples do not list it. Items 2, 3 and 4 need it once
    nothing is planted. The human's to say.

## Bounds — what this verification did not do

- One platform: macOS, git 2.54.0 (Apple Git-157). Whether another git re-encodes at
  `commit-tree`, or prints a `HEAD is now at` line the same way, was not established. That
  `git commit` re-encodes as `commit-tree` does is my reading of git, not something I drove.
- One undecodable shape: Latin-1 bytes with no `encoding` header in the first setup, Latin-1
  log output in the second. No other encoding, and no commit carrying an `encoding` header.
- The amend cells in the second setup, and the UTF-8 control, ran on the candidate only.
- The squashed milestone arm ran in the first setup only, the unsquashed arm in the second
  only. One sub-task each.
- The doc-only commit model and the record-only milestone doors were not driven.
- I read one other report, and only what the finding's own pointer names in it: item 8 under
  *Left open* of `verify-p2-r1-git-capture-other-callers-not-enumerated.a1.md` (one line, no
  repro block), plus that report's headings, its repro block for the form and the table rows
  of its milestone section while looking for a setup (they gave none). No other finding's
  report.
- An environment listing I ran to find which variable marks an agent session printed the
  first characters of two session variables' values into my own transcript. No value is in
  this report or in any file I wrote.

## Where the evidence is

`<scratch>/vr-gcmsg.KTmUIE`, mine alone, minted with `mktemp -d` under the scratch root the
prompt names. Rigs are the `jigc-rig-fresh-*` directories inside it; nothing was torn down.

<!-- end of report -->
