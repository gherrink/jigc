# arm-control — the run (run `canary-one`, round 1, stage `test`, attempt 1)

Reporter `arm-control-run`. Item `arm-control`, kind `trial-arm`, clause
`usable-by-agents`. Candidate: label `c1`, commit
`eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`. Trial image:
`jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`.

## Verdict

**RAN — the arm ran to its end, once, and its pass condition is met in the evidence
copied out of the container.** `run-session.sh --exec` exits 0; the script prints
`ARM 0 PASS — the channel fires and is countable`; `PROVENANCE.txt` records
`jigc-sha eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` and `exit-code 0`; the copied-out
invocation log holds ten records, every one at exit 0, one of them a
`doc show … --task …` — the pass condition's count is 1, against a bar of at least 1.
The container is gone and the source corpus is untouched.

The arm's run for the record is **one** out-dir: `<scratch>/arm-control-run.zis4p3/out-run-1`.
The score step reads that directory and no other of mine.

One finding and two leads follow. None stops the score step. The finding is the
rehearsal's ACR-2, met again in the run's own evidence: **a green arm 00 says nothing
about the round's change at `jigc doc list`** (RUN-1).

## What I asserted before driving anything

| assert | observed |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | one JSON line, `content_sha256` `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt handed me |
| the same call with `bin/previous-91834b5e011d/jigc` | `content_sha256` `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the previous release's, as handed |
| `<scratch>/bin/c1.a1` first on `PATH`, then `command -v jigc` | prints `<scratch>/bin/c1.a1/jigc`, exit 0 |

No `cargo build` was run and nothing under `target/` was driven. Shell state does not
persist between my calls, so every host command below that types a bare `jigc` sets
`PATH` in the same call. The clone is at the candidate's commit with no tracked change
(`git status --short` shows only the run's untracked `r1/` directory), so the arm's
script and the harness I drove are the candidate's.

## The names the blocks use

    C   = the repository root (this clone)
    S   = <scratch>                              the scratch root the prompt names
    W   = <scratch>/arm-control-run.zis4p3       my own directory, minted with mktemp -d
    T   = jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    ARM = $C/completions/trial-driver/arms/walk/00-positive-control.sh
    RUN = $C/completions/trial-harness/run-session.sh
    OUT = $W/out-run-1                           the arm's run, for the record

## A1 — the corpus is a fresh template corpus in its starting state

```yaml
claim: "a corpus instantiated from the template, naive, passes the template's own gate before the arm sees it"
verdict: CONFIRMED
setup:
  - none
repro:
  - "$C/completions/trial-corpus-template/instantiate.sh" "$W/walk-c1" walk-c1
  - "$C/completions/trial-corpus-template/check-corpus.sh" "$W/walk-c1"
  - git -C "$W/walk-c1" rev-parse HEAD
expect:
  exit: 0 for each
  stdout_contains: "11 passed, 0 failed"
pinned-by: "UNPINNED: one corpus of one run, not a standing fact"
```

Observed: `instantiate.sh` exit 0; `check-corpus.sh` exit 0 with eleven `PASS` lines
(7 commits · working tree clean · no jigc/adapter residue, hooks dir included · no git
remote URL · `core.hooksPath` unset · reflog carries 7 entries · on main · `README.md`
the only tracked `.md` · `node --test: 24 passed, 0 failed` · doc-code anchor symbols
present · the queue is on the live write path), one declared `SKIP` (the prose bar,
which only `--clean-prose` arms — the protocol's arm table has walk 00 on a naive
corpus), then `11 passed, 0 failed`. HEAD `e546164ce002507a2249fad2c0625f62c92fff4d`.

The corpus is new: it is not the rehearsal's, and nothing ran in it before the arm.
The protocol's arm table names walk 00's corpus `walk-rc24`; mine is named `walk-c1`.
The name reaches `package.json` and `README.md` only.

## A2 — the arm, run in the trial image through the session harness

```yaml
claim: "arm 00, driven through run-session.sh --exec in the trial image on the A1 corpus, runs to its end and prints its PASS line at exit 0"
verdict: CONFIRMED
setup:
  - the corpus $W/walk-c1 (A1)
  - the image $T, as handed (image id sha256:d37422f98361a9ca93d5e4afaf6ad117c8190acbb11ca371a6085fc268839d97, linux/arm64, env JIGC_SHA=eeffe347324f83a51d1ae83d5f254e73c3f1ea3a; its jigc answers --version with "jigc 1.0.0-rc.24", exit 0)
  - a non-empty CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING (a placeholder value — see below)
repro:
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$ARM" "$W/walk-c1" "$W/out-run-1" "$T"
expect:
  exit: 0
  stdout_contains: "ARM 0 PASS — the channel fires and is countable"
pinned-by: "UNPINNED: one arm of one run; the score is the next step's"
```

Observed — `run-session.sh` exit 0, about two seconds of wall time
(`2026-10-08T08:10:50Z` to `08:10:52Z`). Its header:

    image      : jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a (jigc 1.0.0-rc.24)
    jigc sha   : eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    model      : claude-sonnet-5
    permissions: bypassPermissions

Its stdout between the two rules, whole:

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

and the harness's own quick look after copy-out:

    invocation records : 10
    doc show --task    : 1
    doc show (any)     : 1
    §3.3 adjacent      : 2   (task diff · task validate · doc list --task)

Its stderr is the harness's standing `NOTE: bypassPermissions is the operator's chosen
default …` paragraph and nothing else.

**The credential.** `CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING` is set in this stage's
environment (I read only whether it is non-empty) and `CLAUDE_CODE_OAUTH_TOKEN` is
not. The harness refuses without one of them and writes its value into the container's
configuration; arm 00 reads neither. I ran the tool as it stands and passed the literal
`placeholder-not-a-credential` in the variable, as the rehearsal did, so no credential
entered a container that has no use for one. `JIGC_GATE_MODEL` is unset, as the
protocol requires; no model is called by a scripted arm.

## A3 — the pass condition, read from the evidence copied out

```yaml
claim: "after the container is destroyed, the out-dir holds an invocation log with at least one doc show … --task … record, and provenance naming the candidate's commit and the arm's exit 0"
verdict: CONFIRMED
setup:
  - A2 has run
repro:
  - cat "$OUT/PROVENANCE.txt"
  - grep -c '"doc","show".*"--task"' "$OUT/.jigc/logs/invocations.jsonl"
  - "project $OUT/.jigc/logs/invocations.jsonl to one [argv, exit_code, output_bytes] line per record, into $W/out-run-1.argv"
  - docker ps -a --filter ancestor="$T" --format '{{.ID}}'
  - git -C "$OUT" rev-list --count HEAD
  - git -C "$W/walk-c1" status --short ; git -C "$W/walk-c1" rev-parse HEAD
expect:
  provenance: "jigc-sha eeffe347324f83a51d1ae83d5f254e73c3f1ea3a" and "exit-code 0"
  count: at least 1
  containers: none of the image
pinned-by: "UNPINNED: one arm of one run"
```

Observed.

`PROVENANCE.txt` (its `corpus-src` line names `$W/walk-c1`):

    image        jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    image-id     sha256:d37422f98361a9ca93d5e4afaf6ad117c8190acbb11ca371a6085fc268839d97
    jigc-version jigc 1.0.0-rc.24
    jigc-sha     eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    model        claude-sonnet-5
    permissions  bypassPermissions
    session-start 2026-10-08T08:10:51Z
    exit-code    0

The pass-condition grep prints `1`. The log, as `[argv, exit_code, output_bytes]` — ten
records, every `exit_code` 0, every `error_code` null, every `finding_codes` empty,
timestamps from `2026-10-08T08:10:51Z` to `08:10:52Z` (none older than
`session-start`, so every record is this session's):

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

The chain around the script, each link read from the host after the run:

| link (protocol §2.3, item 1) | read | observed |
|---|---|---|
| copy-in | `git -C "$OUT" rev-list --count HEAD`; `git -C "$OUT" log --oneline` | 8 commits: the corpus's 7, HEAD `e546164` among them, plus `9582571 chore(jigc): install jigc workspace config` — setup's |
| the binary | `PROVENANCE.txt`; the header of A2 | `jigc-sha` is the candidate's commit; A7 adds a behavioural reading of the same image id |
| the invocation log | `$OUT/.jigc/logs/invocations.jsonl` | 10 records, sha256 `b11e64674f71d2c784340fed1ee96dce69575aea94bf79041ecbf73a37356f22` |
| copy-out | `docker ps -a --filter ancestor="$T"`; the container id the harness printed | no container of the image; the id `375b002b4a15…` is in no `docker ps -a` line |
| a countable `doc show … --task` | the grep above | 1 |
| the corpus is never written to | `git -C "$W/walk-c1" status --short`, `rev-parse HEAD`, a listing | clean tree, HEAD still `e546164c…`, no `.jigc`, no `.claude`, no `CLAUDE.md` |

The state the arm leaves, as its step 6 intends: a live task with a staged doc. Under
`$OUT/.jigc/tasks/record-that-the-ingest-queue/` eight files, among them
`docs/adr:drop-the-oldest-sample-when.md` and
`docs/commit:record-that-the-ingest-queue.md`. `git -C "$OUT" status --short` shows two
untracked paths: `.jigc/config/manifest.yaml` (see *The rehearsal's findings*, ACR-5)
and the harness's own `PROVENANCE.txt`. `$OUT/.session-transcript/` holds 0 files — a
scripted arm has no session.

## A4 — a second run cannot overwrite the evidence

The misuse shape that matters to a run step: the same command typed again.

```yaml
claim: "run-session.sh refuses an out-dir that exists, before any container is created, and the evidence is byte-identical afterwards"
verdict: CONFIRMED
setup:
  - A2 has run; a sha256 manifest of every file of $OUT outside .git and node_modules, taken before
repro:
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$ARM" "$W/walk-c1" "$W/out-run-1" "$T"
  - "the same manifest, taken after; cmp of the two"
  - docker ps -a --filter ancestor="$T" --format '{{.ID}}'
expect:
  exit: 2
  stderr: "refusing: <out-dir> already exists — pick a fresh out-dir"
  cmp: exit 0
pinned-by: "UNPINNED: an instrument's refusal, driven once"
```

Observed: exit 2, the one line on stderr, empty stdout; the two manifests are 35 lines
each and `cmp` exits 0; no container of the image exists.

## A5 — the run leaves the sequence the rehearsal recorded

This arm has no join and no merge, so there is no order to diverge and no
divergent-order re-run to make. What can be compared is the run against the rehearsal
the prompt handed me.

```yaml
claim: "the run's ten [argv, exit_code, output_bytes] records equal the ten the rehearsal's report prints for its first image run"
verdict: CONFIRMED
setup:
  - A3's projection, $W/out-run-1.argv
  - the rehearsal's report as committed to the run's directory: completions/artifacts/canary-one/r1/reports/test/arm-control-rehearse.a1.md
repro:
  - "grep -E '^    \\[\\[' <that report> | sed 's/^    //' > $W/rehearse-report.argv"
  - cmp "$W/rehearse-report.argv" "$W/out-run-1.argv"
expect:
  exit: 0
pinned-by: "UNPINNED: a comparison of two runs of one image"
```

Observed: the report yields 10 lines; `cmp` exits 0. So three runs of this script in
this image id — the rehearsal's two, on its corpus, and this one, on a different
corpus under a different name — leave the same ten records to the byte count. I read
the rehearsal's records from its report, not from its scratch directory.

**I did not run the arm a second time for the record.** The item's brief names one
run; a second out-dir would leave the score step two candidates for "the arm's run".

## A6 — the door, read on a copy of the evidence with the candidate I hashed

The arm reaches the round's one door once, `jigc doc list --task <id>`, and sends its
output to `/dev/null`. The log keeps the exit code and a byte count. This block reads
what those bytes are, on a **copy** of the out-dir — the invocation log is on in that
corpus, so a read in `$OUT` itself would append to the evidence.

```yaml
claim: "in the state the arm leaves, jigc doc list --task lists the two staged docs at exit 0 in the byte count the image's log records, jigc doc list reports an empty committed store and names the staged task, and neither touches an untracked file in the root"
verdict: CONFIRMED
setup:
  - cp -R "$OUT" "$W/door-copy" ; mkdir "$W/host-home"
  - in $W/door-copy, with HOME=$W/host-home, PATH=$S/bin/c1.a1:$PATH and a git identity of the run's (run@example.invalid) — command -v jigc prints $S/bin/c1.a1/jigc
  - printf 'one line the arm did not write\n' > notes.md      (not added)
repro:
  - jigc doc list --task record-that-the-ingest-queue
  - jigc doc list
  - shasum -a 256 notes.md ; git status --short
expect:
  exit: 0 for both reads
  staged stdout: 186 bytes
  notes.md: present, same sha256 before and after
pinned-by: "UNPINNED: one read of one state; the door's row is the review row's"
```

Observed:

    $ jigc doc list --task record-that-the-ingest-queue      -> exit 0, stdout 186 bytes, stderr 0 bytes
    id  path  state
    adr:drop-the-oldest-sample-when  docs/decisions/drop-the-oldest-sample-when.md  managed
    commit:record-that-the-ingest-queue  commit:record-that-the-ingest-queue  managed

    $ jigc doc list                                          -> exit 0
    jigc doc list — no committed docs
    (stderr) note: docs are also staged in open task record-that-the-ingest-queue — this listing is the committed store; if that task is yours, list what it stages: `jigc doc list --task record-that-the-ingest-queue`

186 bytes is the `output_bytes` the image's log records for the same argv, so the
discarded output of the arm's one read of the door has this length under the host
candidate. `notes.md` is `0407d2675910474d17cebefe6b688fe8c39caec83c698e91e900f95f3b1b3738`
before and after both reads, and `git status --short` is the same three untracked
paths before and after (`.jigc/config/manifest.yaml`, `PROVENANCE.txt`, `notes.md`).

**What this is not.** It is not the opening record's seeded claim driven: that block
names a bare repository with one empty commit and `jigc setup`, and it is the
verifier's. Here an untracked root file stood through both arms of the door in the
state the arm leaves — one observation in one state, on the host candidate.

## A7 — the image carries the round's change at its door, and the arm does not reach it (RUN-1)

```yaml
claim: "read from a linked worktree, jigc doc list prints the round's note under the candidate and in the image and not under the previous release; arm 00's one read of the door is the staged arm from the main checkout, output discarded, which that note's call is not on"
verdict: CONFIRMED
setup:
  - a probe of mine, $W/door-probe.sh (written through the shell into my scratch directory; not trial tooling): jigc setup; git worktree add -q -b door-probe-wt <a fresh temp dir>/wt; cd there; jigc doc list 2>ERR; print the exit code and the count of ERR lines starting "note: served from the main checkout at "
  - two copies of the A1 corpus, one HOME each
repro:
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$W/door-probe.sh" "$W/walk-c1" "$W/out-door-probe" "$T"
  - "host, candidate: in a corpus copy, HOME not the machine's, PATH=$S/bin/c1.a1:$PATH — bash $W/door-probe.sh"
  - "host, previous release: the same with PATH=$S/bin/previous-91834b5e011d:$PATH"
  - grep -n 'doc list\|"doc","list"' "$ARM"
  - grep -n 'fn run_list(\|run_list_staged(\|served_from_home_note(' "$C/crates/cli/src/doc.rs"
expect:
  notes: 1 in the image, 1 under the host candidate, 0 under the previous release
  exit: 0 for each doc list
pinned-by: "UNPINNED: a bound on an instrument, for triage"
```

Observed — every `doc list` at exit 0 with stdout `jigc doc list — no committed docs`:

| where | binary | `note: served from the main checkout at …` lines on stderr | stderr lines |
|---|---|---|---|
| image `$T`, through `run-session.sh --exec`, image id `sha256:d37422f9…839d97`, `exit-code 0` in its `PROVENANCE.txt` | the image's | 1 | 1 |
| host | candidate, `command -v jigc` = `$S/bin/c1.a1/jigc` | 1 | 1 |
| host | previous release, `command -v jigc` = `$S/bin/previous-91834b5e011d/jigc` | 0 | 0 |

So the image the arm ran in behaves as the candidate, and not as the previous release,
at the round's door. That matters because the labels cannot say it: the image's
`--version`, and the `binary_version` field of all ten log records, read
`1.0.0-rc.24` — the previous release's string, as the contract says a local
candidate's is.

And the arm's own evidence shows none of it:

- The script names the door on two lines (the grep: line 63, the one invocation,
  `jigc doc list --task "$TASK" >/dev/null 2>&1`; line 64, a count of log records
  that is printed and never compared). Stdout, stderr and exit status of that
  invocation are discarded. A `doc list` that failed there would change neither the
  arm's exit nor its `PASS` line — only the `exit_code` field of one log record, which
  in this run is 0.
- In `crates/cli/src/doc.rs` at the candidate's commit, `fn run_list` (line 4801)
  returns into `run_list_staged` at line 4808 whenever `--task` is given. The call
  `served_from_home_note(cwd, &jigc_home)` is at line 4975, after that return, in the
  committed arm. The grep finds that call form on three lines of the file — 4424 (in
  another function), 4469 (the helper's own body) and 4975 — and none inside
  `run_list_staged` (from line 5006).
- The run's ten records equal the rehearsal's ten (A5), and the rehearsal's report
  says the previous release leaves the same ten. **I did not re-drive that last
  comparison**: the arm under the previous release is the rehearsal's R5, not mine.

What a green `arm-control` says, then: the chain a session rides records, and the
staged read-back is countable — the positive control's own claim, in the script's
header and in protocol §2.3. It is not evidence, either way, about the changed
behaviour of `jigc doc list`.

## A8 — the authoritative reader exits 1 over this evidence (lead RUN-2)

```yaml
claim: "run.py observe over the arm's out-dir prints VERB 1 and exits 1, because a scripted arm leaves no transcript"
verdict: CONFIRMED
setup:
  - A2 has run
repro:
  - python3 "$C/completions/trial-driver/run.py" observe "$W/out-run-1"
expect:
  exit: 1
  stdout: a row with recs 10, wrote 4, VERB 1, adj 2, fs 0
pinned-by: "UNPINNED: a lead for the score step"
```

Observed — exit 1; stdout, then stderr:

    session           recs wrote  VERB  adj  fs  outcome
      provenance: ungated: jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a / eeffe347324f (pass --gate <record.json> to check it)
    out-run-1           10     4     1    2   0       read back through the fence's verb

      ! no transcript — the FILESYSTEM channel is unmeasured for this session

`run.py` sets `rc = 1` under `if o.transcript_missing:` (lines 201–204). The harness's
last line tells the reader to score with exactly this command. **For the score step:**
the arm's pass condition is the count of `doc show … --task` records in the copied-out
log (1) and the arm's exit code in `PROVENANCE.txt` (0); the reader's table agrees
with both (`VERB 1`), and its exit 1 is the missing transcript of an arm that has no
session, not the arm failing. The read wrote nothing into `$OUT`: it ran at 08:11:43Z,
and `find "$OUT" -newermt` three seconds after `PROVENANCE.txt` was written (08:10:52Z)
lists two entries, `.git` and `.git/index`, both mine (*For the score step* says how).
It wrote nothing into the repository either (`git status --short --ignored` shows the
run's `r1/` and `target/` only).

## A9 — the provenance of a scripted arm names a model and a permission mode no session used (lead RUN-3)

```yaml
claim: "PROVENANCE.txt of an --exec run carries 'model claude-sonnet-5' and 'permissions bypassPermissions', though the arm starts no assistant"
verdict: CONFIRMED
setup:
  - A2 has run
repro:
  - grep -E '^(model|permissions)' "$W/out-run-1/PROVENANCE.txt"
  - grep -n '^model\|^permissions' "$C/completions/trial-harness/run-session.sh"
expect:
  stdout: the two lines
pinned-by: "UNPINNED: a lead on trial tooling, not pursued"
```

Observed: the two lines, as A3 prints them; in the harness they are lines 257 and 258
of the one heredoc that writes the file, outside any mode branch. For this arm they
describe nothing that ran. Not pursued: I did not read what any reader of
`PROVENANCE.txt` does with the two fields.

## The rehearsal's findings, as this run met them

I was handed the rehearsal's report. I am the run step, not its reconciler: I did not
set out to re-drive its findings, and this table says what the run happened to meet.

| rehearsal's id | what this run observed | re-driven here |
|---|---|---|
| ACR-1 — the harness refuses a scripted arm with no credential in the environment | not met: the variable is set in this stage, and I passed a placeholder (A2) | no |
| ACR-2 — arm 00 cannot witness the round's change at its door | met again, with my own probe for the image and the two host binaries, and my own read of the script and of `doc.rs` — RUN-1, A7. Its "ten of ten records equal under the previous release" I did not re-drive | in part |
| ACR-3 — a walk arm run outside the container goes on in the caller's directory | not met: I ran the arm only in the container | no |
| ACR-4 — `run.py observe` exits 1 on a scripted arm's evidence | met again on the run's out-dir — RUN-2, A8 | yes |
| ACR-5 — `.jigc/config/manifest.yaml` untracked after `setup` then `config set` | met again in `$OUT`, and read against the design: **as designed, and the verb says so.** `design/project-setup.md` line 160: "`jigc config set` writes `.jigc/config/manifest.yaml` and tracks nothing". With the candidate, in the A6 copy, `jigc config set invocation-log true` exits 0 and prints "config: set `invocation-log` = `true` — written to `.jigc/config/`, uncommitted — commit it with your next commit". The file's content is the one knob (`scalar:` / `invocation-log: 'true'`). I return no finding for it | yes |

## Findings

| id | lead | title | door | clause | severity | class | count, and how it was reached | repro |
|---|---|---|---|---|---|---|---|---|
| RUN-1 | no | a green arm 00 is not evidence about the round's change at its door: the arm reads `jigc doc list` once, as the staged arm from the main checkout, with output and exit status discarded, while the changed call sits on the committed arm (the rehearsal's ACR-2, met again in the run) | jigc doc list | none broken, as I read it — it bounds what this item's green is evidence of under `usable-by-agents` | information for triage | an item counted as covering a door whose changed path it does not reach | instance, bounded for arm 00 only. In the script: 2 lines name the door (grep, lines 63 and 64), 1 of them an invocation; 1 of the run's 10 log records. In `doc.rs`: 3 lines carry the call form `served_from_home_note(` (grep: 4424, 4469, 4975), 0 of them inside `run_list_staged`; the `--task` return is line 4808 (read). Driven: 3 reads of the door from a linked worktree (image 1 note, host candidate 1, host previous release 0). Not enumerated: the other 23 walk arms — a grep finds `doc list` in 14 of the 24 scripts, none of the other 13 was read, and none is run by this item | A7 — the image carries the round's change at its door, and the arm does not reach it (RUN-1) |
| RUN-2 | yes | `run.py observe`, the reader the harness names as authoritative, exits 1 over a scripted arm's evidence while its table shows the channel fired (the rehearsal's ACR-4, met again) | `completions/trial-driver/run.py observe` — unlisted | none | information for the score step | instance, unbounded | instance, unbounded — one out-dir of mine and one read of the branch that sets the exit (`run.py` lines 201–204); the rehearsal reports the same on its own out-dir. The other two `rc = 1` sites in the file (lines 162, 200) were not read | A8 — the authoritative reader exits 1 over this evidence (lead RUN-2) |
| RUN-3 | yes | the provenance of a scripted arm records a model and a permission mode that no session used | `completions/trial-harness/run-session.sh --exec` — unlisted | none | low | instance, unbounded | instance, unbounded — 1 out-dir read; in the harness the two fields are written by 1 heredoc (lines 257, 258) outside the mode branch, so the `--shell` mode would carry them too, which I did not drive. No reader of the two fields was traced | A9 — the provenance of a scripted arm names a model and a permission mode no session used (lead RUN-3) |

No finding against `jigc` itself came out of the arm: ten invocations, ten at exit 0,
and the one read of the door printed what the staged area holds.

## What I did not do

- **No assistant session, and no spawn.** Arm 00 is scripted. Nothing here is a blind
  session, a headless turn or a sub-agent launch, and none is part of this arm. The
  genuine concurrent-spawn artifact is the orchestrator's main-session half; I did not
  run it, and nothing in this report is evidence about it. There is no fan-out, no
  join and no adapter spawn template in this unit, so there was no N-process sim to
  run either.
- **I did not score the arm for the record.** A3 reads the pass condition because a
  run that cannot show it has not run; the score is the next step's.
- **I ran the arm once.** A5 says why.
- **I did not run `verify-image.sh` or `verify-pair.sh`**, and I built no image. What I
  established about the image is its id, its `JIGC_SHA`, its `--version`, and A7.
- **I did not drive `walk.py --only 00`**, the transport the protocol's arm table
  names. The item's brief names `run-session.sh --exec`; `walk.py` builds that same
  argv (its line 58), and that is what I drove.
- **I did not drive the opening record's seeded claim.** A6 is an observation beside
  it, in another state.
- **I did not re-drive the rehearsal's ACR-1 or ACR-3**, nor its comparison of the arm
  under the previous release.
- Files of mine outside the scratch root, all in the system temp directory: the two
  linked worktrees the host probes of A7 minted with `mktemp -d` (one `wt` directory
  each, registered in the two corpus copies under `$W`), and one copy of
  `dev/stabilize-record --help`. None is in the repository. Nothing was removed.

## For the score step

The arm's run is `<scratch>/arm-control-run.zis4p3/out-run-1`. **One file of it changed
after copy-out, by my hand:** `.git/index`, which my `git -C "$OUT" status --short`
reads rewrote (git refreshes its stat cache on a status; no tracked content, ref or
object moved — HEAD is still `9582571`, and the status prints the same two untracked
paths each time). Every other file is as the harness left it: `find "$OUT" -newermt`
three seconds past `PROVENANCE.txt` lists `.git` and `.git/index` and nothing else, and
A4's two manifests of everything outside `.git` are equal. What it holds:

| file | what the score reads from it | value |
|---|---|---|
| `PROVENANCE.txt` (sha256 `4b3373c777791a06680a8fd3cb6a0196959a14e50f08d5a7995e42a65de8546c`) | `jigc-sha`, `exit-code` | `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, `0` |
| `.jigc/logs/invocations.jsonl` (sha256 `b11e64674f71d2c784340fed1ee96dce69575aea94bf79041ecbf73a37356f22`) | records whose argv is `doc show … --task …` | 1 of 10 |

Beside it under `<scratch>/arm-control-run.zis4p3/`: `run-1.stdout` and `run-1.stderr`
(the harness's two channels for the run), `observe-1.stdout` and `observe-1.stderr`,
`out-run-1.argv` (the projection), `walk-c1` (the source corpus, untouched). The other
directories there — `door-copy`, `out-door-probe`, the two `probe-corpus-*` — are
probes and are **not** the arm's run.

<!-- end of report -->
