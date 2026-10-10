# arm-control — the rehearsal (run `canary-one`, round 1, stage `test`, attempt 1)

Reporter `arm-control-rehearse`. Item `arm-control`, kind `trial-arm`, clause
`usable-by-agents`. Candidate: label `c1`, commit
`eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`. Trial image:
`jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`.

## Verdict

**REHEARSED — the occasion fires.** The image answers, the arm's script starts inside
it through `completions/trial-harness/run-session.sh --exec`, and it runs to
`ARM 0 PASS` at exit 0 on a corpus instantiated from
`completions/trial-corpus-template/`. The arm's pass condition is met in the evidence
copied out after the container was destroyed (one `doc show … --task` record), and the
round's one door, `jigc doc list`, is reached once (`doc list --task <id>`, exit 0).
Two rehearsals in the image and one drive of the same script with the hash-asserted
candidate on the host leave the same ten `(argv, exit_code, output_bytes)` records.

This is a rehearsal on a rehearsal corpus. Its out-dirs are in my own scratch directory
and are **not** the arm's run: the `run` step drives its own corpus into its own
out-dir.

Two findings and three leads follow. Neither finding stops the run step. One of them
bounds what a green arm says about this round: **arm 00 cannot witness the round's
change at its door** (ACR-2).

## What I asserted before driving anything

| assert | observed |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | one JSON line, `content_sha256` `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt handed me |
| the same call with `bin/previous-91834b5e011d/jigc` | `content_sha256` `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the previous release's, as handed |
| `<scratch>/bin/c1.a1` first on `PATH`, then `command -v jigc` | prints `<scratch>/bin/c1.a1/jigc`, exit 0 |

No `cargo build` was run and nothing under `target/` was driven. Shell state does not
persist between my calls, so every host command below that types a bare `jigc` sets
`PATH` on its own line.

## The names the blocks use

    C = the repository root (this clone)
    S = <scratch>                                  the scratch root the prompt names
    W = <scratch>/arm-control-rehearse.grSDC7      my own directory, minted with mktemp -d
    T = jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    ARM = $C/completions/trial-driver/arms/walk/00-positive-control.sh
    RUN = $C/completions/trial-harness/run-session.sh

`$W/walk-rehearse` is the corpus: `$C/completions/trial-corpus-template/instantiate.sh
"$W/walk-rehearse" walk-rehearse` (exit 0, naive — no `--clean-prose`, as the protocol's
arm table has walk 00), then `check-corpus.sh "$W/walk-rehearse"` — exit 0,
`11 passed, 0 failed`, one declared SKIP (the prose bar, which only `--clean-prose`
arms), HEAD `20970bb9cfb20abed0f6cbb5a6091c668e1b39f0`. `bash -n "$ARM"` exits 0. The
clone's tree is at the candidate's commit with no tracked change, so the script and the
harness I drove are the candidate's.

## R1 — the image answers

```yaml
claim: "the trial image exists, records the candidate's commit, and its jigc answers"
verdict: CONFIRMED
setup:
  - none (the image is the one the prompt hands; I built nothing)
repro:
  - docker image inspect "$T" --format 'id={{.Id}} os={{.Os}}/{{.Architecture}}'
  - docker image inspect "$T" --format '{{range .Config.Env}}{{println .}}{{end}}'
  - docker run --rm --entrypoint /usr/local/bin/jigc "$T" --version
  - docker run --rm --entrypoint sh "$T" -c 'cat /usr/local/share/jigc-image/layout; cat /usr/local/share/jigc-image/source'
expect:
  exit: 0 for each
pinned-by: "UNPINNED: a rehearsal of one image, not a standing fact"
```

Observed:

    id=sha256:d37422f98361a9ca93d5e4afaf6ad117c8190acbb11ca371a6085fc268839d97 os=linux/arm64
    JIGC_SHA=eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    jigc 1.0.0-rc.24
    single-binary
    sha eeffe347324f83a51d1ae83d5f254e73c3f1ea3a

The entrypoint is `/usr/local/bin/session-entry` (`chown` of `/work`, then `gosu node`),
the working directory `/work`. The version string is the previous release's, as the
contract says a local candidate's is — so the image's identity rests here on two labels
(`JIGC_SHA`, the `source` record). R5 adds a behavioural reading.

**Not done:** I did not run `verify-image.sh`. Its seven checks are the second
preflight's, and its check 4 buys two live model turns. What I established about the
image is the five lines above, R2 and R5.

## R2 — the script starts in the image and the occasion fires

```yaml
claim: "arm 00, driven through run-session.sh --exec in the trial image on a template corpus, starts and meets its pass condition in the copied-out evidence"
verdict: CONFIRMED
setup:
  - the corpus $W/walk-rehearse (above)
  - a non-empty CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING in the environment (R4 says why; I passed a placeholder value, not a credential)
repro:
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$ARM" "$W/walk-rehearse" "$W/out-rehearse-1" "$T"
  - cat "$W/out-rehearse-1/PROVENANCE.txt"
  - cat "$W/out-rehearse-1/.jigc/logs/invocations.jsonl"
expect:
  exit: 0
  stdout_contains: "ARM 0 PASS — the channel fires and is countable"
  log: at least one record whose argv is doc show … --task …
pinned-by: "UNPINNED: a rehearsal; the arm's run and its score are the next two steps"
```

Observed — `run-session.sh` exit 0; its stdout between the two rules, whole:

    === 0 · adopt the corpus with the binary under test
    jigc 1.0.0-rc.24

    === 1 · mint the task
    task: record-that-the-ingest-queue

    === 2 · author a managed doc through the write verbs
    created: adr:drop-the-oldest-sample-when
    slots authored

    === 3 · THE CHANNEL — read the staged copy back
    ---
    status: proposed
    date: 2026-10-08
    schema-version: 2
    ---

    # Drop the oldest sample when the ingest queue overflows

    ## Context

    Prose for context, authored by the control arm.


    === 4 · the pass condition, checked in-container
    doc show --task records: 1

    === 5 · the VERB-ADJACENT channel also records (protocol §3.3)
    adjacent records: 2

    === 6 · leave the task open
    jigc task list — 1 active task(s)

      record-that-the-ingest-queue  [record-decision]  record that the ingest queue drops the oldest sample when it overflows
    — jigc · run `jigc start` for orientation; all writes through `jigc`.

    ARM 0 PASS — the channel fires and is countable

and the harness's own summary after copy-out:

    invocation records : 10
    doc show --task    : 1
    doc show (any)     : 1
    §3.3 adjacent      : 2   (task diff · task validate · doc list --task)

`PROVENANCE.txt` in the out-dir: `image-id sha256:d37422f9…839d97`, `jigc-version jigc
1.0.0-rc.24`, `jigc-sha eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, `session-start
2026-10-08T08:03:31Z`, `exit-code 0`.

The log copied out, as `[argv, exit_code, output_bytes]` — ten records, every one exit 0:

    [["--version"], 0, 17]
    [["start", "--workflow", "record-decision", "record that the ingest queue drops the oldest sample when it overflows"], 0, 8039]
    [["doc", "create", "adr", "--title", "Drop the oldest sample when the ingest queue overflows", "--task", "record-that-the-ingest-queue"], 0, 32]
    [["doc", "set-slot", "adr:drop-the-oldest-sample-when#context", "--from-file", "-", "--task", "record-that-the-ingest-queue"], 0, 60]
    [["doc", "set-slot", "adr:drop-the-oldest-sample-when#decision", "--from-file", "-", "--task", "record-that-the-ingest-queue"], 0, 61]
    [["doc", "set-slot", "adr:drop-the-oldest-sample-when#consequences", "--from-file", "-", "--task", "record-that-the-ingest-queue"], 0, 65]
    [["doc", "show", "adr:drop-the-oldest-sample-when", "--task", "record-that-the-ingest-queue"], 0, 328]
    [["task", "diff", "record-that-the-ingest-queue"], 0, 516]
    [["doc", "list", "--task", "record-that-the-ingest-queue"], 0, 186]
    [["task", "list"], 0, 232]

The chain around the script held as well: `docker ps -a` shows no container of the
image afterwards; the source corpus is untouched (clean tree, HEAD still `20970bb9…`,
no `.jigc`, no `.claude`); the out-dir carries `.git` with the corpus history plus
setup's one commit (`chore(jigc): install jigc workspace config`), `.jigc/`,
`.claude/`, `CLAUDE.md`, `PROVENANCE.txt`, and an empty `.session-transcript/` (0
files — a scripted arm has no session, R7).

## R3 — run again, and with the candidate I hashed: one sequence

This arm has no join and no merge, so there is no order to diverge; the repeat is for
stability and for tying the image to the binary whose hash I asserted.

```yaml
claim: "a second rehearsal in the image, and the same script run with the hash-asserted candidate on the host, leave the same ten (argv, exit_code, output_bytes) records as R2"
verdict: CONFIRMED
setup:
  - R2's corpus, a fresh out-dir; and a copy of the corpus on the host, with a HOME that is not the machine's
repro:
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$ARM" "$W/walk-rehearse" "$W/out-rehearse-2" "$T"
  - mkdir "$W/host-home" && cp -R "$W/walk-rehearse" "$W/host-corpus"
  - cd "$W/host-corpus" && env HOME="$W/host-home" PATH="$S/bin/c1.a1:$PATH" GIT_AUTHOR_NAME=rehearsal GIT_AUTHOR_EMAIL=rehearsal@example.invalid GIT_COMMITTER_NAME=rehearsal GIT_COMMITTER_EMAIL=rehearsal@example.invalid bash -c 'command -v jigc; exec bash "$1"' _ "$ARM"
  - "for each of out-rehearse-1, out-rehearse-2, host-corpus: project .jigc/logs/invocations.jsonl to one [argv, exit_code, output_bytes] line per record, into $W/<name>.argv"
  - cmp "$W/out-rehearse-1.argv" "$W/out-rehearse-2.argv"
  - cmp "$W/out-rehearse-1.argv" "$W/host-corpus.argv"
expect:
  exit: 0 for both cmp
pinned-by: "UNPINNED: a rehearsal"
```

Observed: the second image run exits 0 with `ARM 0 PASS`, `doc show --task records: 1`,
`adjacent records: 2`, `invocation records : 10`; the host run prints `command -v jigc`
as `<scratch>/bin/c1.a1/jigc`, exits 0 with `ARM 0 PASS`; both `cmp` exit 0; each
projection is 10 lines. The host run's one stderr line is R6's subject.

**The door, read with the candidate on the host** (the arm sends this output to
`/dev/null`). In `$W/host-corpus`, same `HOME` and `PATH`:

    $ jigc doc list --task record-that-the-ingest-queue      -> exit 0, stderr empty
    id  path  state
    adr:drop-the-oldest-sample-when  docs/decisions/drop-the-oldest-sample-when.md  managed
    commit:record-that-the-ingest-queue  commit:record-that-the-ingest-queue  managed

    $ jigc doc list                                          -> exit 0
    jigc doc list — no committed docs
    (stderr) note: docs are also staged in open task record-that-the-ingest-queue — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list --task record-that-the-ingest-queue`

The staged listing is 186 bytes, the figure the image's log records for the same argv.

## R4 — with no credential in the environment the harness refuses the scripted arm (ACR-1)

```yaml
claim: "run-session.sh --exec refuses, before any container exists, when neither token variable is set — although arm 00 uses no credential"
verdict: CONFIRMED
setup:
  - R2's corpus; an out-dir that does not exist
repro:
  - env -u CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING -u CLAUDE_CODE_OAUTH_TOKEN "$RUN" --exec "$ARM" "$W/walk-rehearse" "$W/out-notoken" "$T"
expect:
  exit: 2
  stderr: "refusing: no token in CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING"
pinned-by: "UNPINNED: an instrument's precondition, reported for the run step"
```

Observed: exit 2, the one line above, no out-dir, no container. With the variable set to
the literal `placeholder-not-a-credential` the same command is R2 and passes — so the
arm needs a non-empty variable and no credential. **In this stage's environment
`CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING` is set** (I read only whether it is non-empty), so
the run step is not stopped by this. Run as the tool stands, with that variable as it
is, the harness writes its value into the container's configuration for the container's
life (the script's own comment at its `--env-file` line declares it); the placeholder
form in R2 keeps it out of a container that never reads it.

## R5 — the arm does not tell the candidate from the previous release; a linked-worktree read of the door does (ACR-2)

```yaml
claim: "arm 00's ten records are identical under the candidate and under the previous release, so its pass says nothing about the round's change at jigc doc list; that change is reachable, and the image carries it"
verdict: CONFIRMED
setup:
  - copies of R2's corpus, one HOME each; the two host binaries the prompt hands; the image
repro:
  - mkdir "$W/prev-home" && cp -R "$W/walk-rehearse" "$W/prev-corpus"
  - cd "$W/prev-corpus" && env HOME="$W/prev-home" PATH="$S/bin/previous-91834b5e011d:$PATH" GIT_AUTHOR_NAME=rehearsal GIT_AUTHOR_EMAIL=rehearsal@example.invalid GIT_COMMITTER_NAME=rehearsal GIT_COMMITTER_EMAIL=rehearsal@example.invalid bash -c 'command -v jigc; exec bash "$1"' _ "$ARM"
  - diff "$W/host-corpus.argv" "$W/prev-corpus.argv"
  - "the probe $W/pair-probe.sh, in a corpus copy's root: jigc setup; git worktree add -q -b pair-probe-wt <a fresh dir>/wt; cd there; jigc doc list 2>ERR; count lines of ERR starting 'note: served from the main checkout at '"
  - "host, candidate:  cd a corpus copy && PATH=$S/bin/c1.a1:$PATH (HOME and git identity as above) bash $W/pair-probe.sh"
  - "host, previous:   the same with PATH=$S/bin/previous-91834b5e011d:$PATH"
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$W/pair-probe.sh" "$W/walk-rehearse" "$W/out-pair-probe" "$T"
expect:
  diff: exit 0 (no difference between candidate and previous over the arm)
  probe: 1 note under the candidate, 0 under the previous release, 1 in the image
pinned-by: "UNPINNED: a bound on an instrument, for triage"
```

Observed:

- The previous release runs the arm to `ARM 0 PASS`, exit 0, and `diff` of the two
  projections exits 0: **10 of 10 records agree in argv, exit code and output bytes.**
- The probe, every arm at exit 0 with stdout `jigc doc list — no committed docs`:

  | where | binary | `note: served from the main checkout at …` lines on stderr | stderr bytes |
  |---|---|---|---|
  | host | candidate (`dded1fac…`) | 1 | 404 |
  | host | previous release (`accf3996…`) | 0 | 0 |
  | image `$T`, through `run-session.sh --exec`, cwd `/work` | the image's | 1 | 180 |

  (The byte counts differ between host and image by the lengths of the two paths the
  note names.)

So the image behaves as the candidate and not as the previous release at the round's
door — a reading that does not rest on a label. And the arm itself reaches none of it:

- `completions/trial-driver/arms/walk/00-positive-control.sh` names the door on two
  lines (`grep -n 'doc list\|"doc","list"'`: line 63, the one invocation, `jigc doc list
  --task "$TASK" >/dev/null 2>&1`; line 64, a count that is printed and never asserted).
  Its stdout, stderr and exit status are discarded: a `doc list` that failed would
  change neither the arm's exit nor its `PASS` line — only the `exit_code` field of one
  log record.
- `fn run_list` (`crates/cli/src/doc.rs:4801`) returns into `run_list_staged` at line
  4808 whenever `--task` is given. The round's direct change at this door, the call
  `served_from_home_note(cwd, &jigc_home)`, is at line 4975, the last statement of the
  committed arm — past that return. The arm's one invocation is the staged arm, from the
  main checkout.

What that leaves a green `arm-control` saying about this round: the chain records and
the staged read-back is countable — the positive control's own claim, which the
script's header states — and nothing about the changed behaviour of `jigc doc list`.

## R6 — outside the container the script runs where it stands (lead ACR-3)

```yaml
claim: "arm 00 run where /work does not exist prints one cd error and goes on in the caller's directory, where it runs jigc setup and mints a task"
verdict: CONFIRMED
setup:
  - a host with no /work (ls -d /work: No such file or directory); cwd = $W/host-corpus
repro:
  - R3's host command
expect:
  stderr: "…/00-positive-control.sh: line 15: cd: /work: No such file or directory"
  exit: 0, with ARM 0 PASS — the corpus adopted is the caller's cwd
pinned-by: "UNPINNED: a lead on trial tooling, not pursued"
```

Observed as expected: line 15 is a bare `cd /work` under `set -uo pipefail` — no `-e`.
I used that on purpose, from inside a scratch copy; typed from this repository's root
the same command would run `jigc setup` here. Not pursued beyond the grep in the
findings table.

## R7 — the authoritative reader exits 1 on a scripted arm's evidence (lead ACR-4)

```yaml
claim: "run.py observe over arm 00's out-dir prints VERB 1 and exits 1, because a scripted arm leaves no transcript"
verdict: CONFIRMED
setup:
  - R2's out-dir
repro:
  - python3 "$C/completions/trial-driver/run.py" observe "$W/out-rehearse-1"
expect:
  exit: 1
pinned-by: "UNPINNED: a lead for the score step"
```

Observed, stdout then stderr:

    session           recs wrote  VERB  adj  fs  outcome
      provenance: ungated: jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a / eeffe347324f (pass --gate <record.json> to check it)
    out-rehearse-1      10     4     1    2   0       read back through the fence's verb

      ! no transcript — the FILESYSTEM channel is unmeasured for this session

The exit is the transcript branch's (`run.py`, the `o.transcript_missing` arm sets
`rc = 1`). For the score step: the arm's pass condition is the count of `doc show …
--task` records in the copied-out log (here 1) and the arm's exit code in
`PROVENANCE.txt` (here 0) — a reader's exit 1 on this evidence is not the arm failing.

## R8 — the copied-out corpus has one untracked file of jigc's (lead ACR-5)

```yaml
claim: "after the arm, git status in the corpus shows .jigc/config/manifest.yaml untracked"
verdict: CONFIRMED
setup:
  - R2's and R3's out-dirs, R3's host corpus, R5's previous-release corpus
repro:
  - git -C "$W/out-rehearse-1" status --short      (and the three others)
expect:
  stdout_contains: "?? .jigc/config/manifest.yaml"
pinned-by: "UNPINNED: a lead, not pursued"
```

Observed in all four: `?? .jigc/config/manifest.yaml` (the two out-dirs also show the
harness's own `PROVENANCE.txt`). The arm's step 0 is `jigc setup`, which commits, then
`jigc config set invocation-log true`. The same under the previous release, so it is
not this round's. I did not read what the design says a `config set` after `setup`
should leave, and I do not say this is a defect.

## Findings

| id | lead | title | door | clause | severity | class | count, and how it was reached | repro |
|---|---|---|---|---|---|---|---|---|
| ACR-1 | no | the session harness refuses a scripted arm when no credential is in the environment, though the item's brief says the arm needs none | `completions/trial-harness/run-session.sh --exec` — unlisted | none (an instrument's precondition: where the variable is unset the arm is void for a reason that says nothing about jigc) | low — the variable is set in this stage's environment | an instrument demanding a credential it does not use | `grep -n 'refusing: no token' completions/trial-harness/*.sh` — 2 hits: `run-session.sh:97`, which precedes the mode branch and so binds all four modes, and `verify-image.sh:61`. Driven: 1 (`run-session.sh --exec`). The other site is a grep hit, not driven | R4 — with no credential in the environment the harness refuses the scripted arm (ACR-1) |
| ACR-2 | no | arm 00 cannot witness the round's change at its door: it reaches `jigc doc list` once, as the staged arm, with output and exit status discarded, and leaves identical records under the candidate and the previous release | jigc doc list | none broken, as I read it — it bounds what a green arm is evidence of under `usable-by-agents` | information for triage | an item counted as covering a door whose changed path it does not reach | instance, bounded for arm 00 only: 1 invocation of the door in the script (grep, line 63) and 1 of 10 log records; 10 of 10 records equal between the two binaries (`diff`, exit 0); the changed call at `doc.rs:4975` sits past the `--task` return at `doc.rs:4808` (read). The other 23 walk arms were not read for this | R5 — the arm does not tell the candidate from the previous release; a linked-worktree read of the door does (ACR-2) |
| ACR-3 | yes | a walk arm run outside the container goes on in the caller's directory after a failed `cd /work` | `completions/trial-driver/arms/walk/00-positive-control.sh` — unlisted | none | low | an unguarded `cd` with no `-e` | 24 of the 24 files under `completions/trial-driver/arms/walk/*.sh` have a bare `cd /work` line and no `set -e` form (a per-file `grep -cE '^cd /work *$'` and `grep -cE '^set -[a-z]*e'`); `arms/adopt.sh` has the bare `cd` and does have `set -e`. Driven: 1 (arm 00). The rest is the grep | R6 — outside the container the script runs where it stands (lead ACR-3) |
| ACR-4 | yes | `run.py observe` exits 1 on a scripted arm's evidence while its table shows the channel fired | `completions/trial-driver/run.py observe` — unlisted | none | information for the score step | instance, unbounded | instance, unbounded — one out-dir, one read of the branch that sets the exit | R7 — the authoritative reader exits 1 on a scripted arm's evidence (lead ACR-4) |
| ACR-5 | yes | `.jigc/config/manifest.yaml` is untracked in the corpus after `jigc setup` then `jigc config set invocation-log true` | `jigc config set` — unlisted | none claimed; not read against the design | unknown | instance, unbounded | instance, unbounded — four corpora, two binaries, one command sequence; same under the previous release | R8 — the copied-out corpus has one untracked file of jigc's (lead ACR-5) |

## What I did not do

- **No assistant session, and no spawn.** Arm 00 is scripted: nothing here is a blind
  session, a headless turn or a sub-agent launch, and none is part of this arm. Nothing
  in this report is evidence about a genuine concurrent spawn.
- **I did not run the arm for the record**, and I did not score it. The two out-dirs
  under `$W` are rehearsals.
- **I did not run `verify-image.sh` or `verify-pair.sh`**, and I built no image.
- **I did not drive `walk.py --only 00`**, the transport the protocol's arm table
  names; the item's brief names `run-session.sh --exec`, which is what `walk.py` calls,
  and that is what I drove.
- One probe script, `$W/pair-probe.sh`, is mine and is not trial tooling. I wrote it
  with the file tool into my own scratch directory; it is in no commit and nowhere in
  the repository.
- One file of mine is outside the scratch root: the image's environment lines, written
  to the system temp directory by my first inspect call. It holds the image's `PATH`
  and `JIGC_SHA` lines and nothing else.

## For the run step

The command, with a corpus and an out-dir of its own (the out-dir must not exist; the
corpus is never written to):

    $C/completions/trial-corpus-template/instantiate.sh <corpus> <name>
    $C/completions/trial-corpus-template/check-corpus.sh <corpus>
    $C/completions/trial-harness/run-session.sh --exec $C/completions/trial-driver/arms/walk/00-positive-control.sh <corpus> <out-dir> jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a

What a rehearsed run looks like: `run-session.sh` exit 0; `ARM 0 PASS — the channel
fires and is countable`; `PROVENANCE.txt` with `jigc-sha
eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` and `exit-code 0`; ten records in
`<out-dir>/.jigc/logs/invocations.jsonl`, the sequence in R2; a few seconds of wall
time.

<!-- end of report -->
