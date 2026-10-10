# arm-control — the score (run `canary-one`, round 1, stage `test`, attempt 1)

Reporter `arm-control-score`. Item `arm-control`, kind `trial-arm`, clause
`usable-by-agents`. Candidate: label `c1`, commit
`eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`. Trial image:
`jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`.

## Verdict

**SCORED — PASS. The positive control fired, and its pass condition is met in the
evidence copied out of the container.** The arm's own exit code, as the harness
recorded it in `PROVENANCE.txt`, is 0. The copied-out invocation log holds ten
records, one of them a `jigc doc show … --task …` — a count of 1 against a bar of at
least 1, by the arm's own grep and by a structured read of each record's argv. That
record exited 0 and its byte count is the byte count the hash-asserted candidate
prints for the same argv on a copy of the evidence. No void condition holds: the
session ran, the log exists and is readable, every record is this session's, and
authoring happened before the read.

The outcome for the item, as I read it: **green**. Writing the item's result is the
stage's record step, not mine.

What that green is evidence of is bounded twice, and both bounds are returned as
findings:

- **It is not evidence about the round's change at `jigc doc list`** (SCO-1 — the
  rehearsal's ACR-2 and the run's RUN-1, re-driven here with reads of my own).
- **The pass condition counts a record, not a read that succeeded** (SCO-2, new): the
  same script with its one channel read pointed at a doc that does not exist exits 0
  with `ARM 0 PASS`. In *this* run's evidence the counted record did succeed — I read
  its exit code and matched its bytes — so the score above does not rest on that gap.

Three leads follow them (SCO-3, SCO-4, SCO-5).

## What I asserted before driving anything

| assert | observed |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | one JSON line, `content_sha256` `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt handed me |
| the same call with `bin/previous-91834b5e011d/jigc` | `content_sha256` `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the previous release's, as handed |
| `<scratch>/bin/c1.a1` first on `PATH`, then `command -v jigc` | prints `<scratch>/bin/c1.a1/jigc`, exit 0 |

No `cargo build` was run and nothing under `target/` was driven. Shell state does not
persist between my calls, so every host command that types a bare `jigc` sets `PATH`
in the same call. The clone is at the candidate's commit (`git rev-parse HEAD` prints
`eeffe347…`), and `git status --short` shows only the run's untracked `r1/` directory,
before and after my work.

## The names the blocks use

    C    = the repository root (this clone)
    S    = <scratch>                               the scratch root the prompt names
    W    = <scratch>/arm-control-score.qo9Edo      my own directory, minted with mktemp -d
    R    = <scratch>/arm-control-run.zis4p3        the run step's directory
    OUT  = $R/out-run-1                            the arm's run — the evidence I score
    T    = jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a
    ARM  = $C/completions/trial-driver/arms/walk/00-positive-control.sh
    RUN  = $C/completions/trial-harness/run-session.sh
    TASK = record-that-the-ingest-queue
    ADDR = adr:drop-the-oldest-sample-when

The run step's report names `OUT` as the one directory of the arm's run. I read it and
wrote nothing into it: every `jigc` call and every reader below ran on a copy under
`$W`, and every `git` read of `OUT` ran with `GIT_OPTIONAL_LOCKS=0`, which does not
refresh the index.

## The pass condition, and where it is written

Three places state it, and they agree:

- the item's brief: "Score: its pass condition, from the evidence copied out of the
  container";
- the arm's header (`$ARM`, lines 9–10): "at least one `jigc doc show … --task …`
  record in the corpus's invocation log, recoverable from $OUT after the container is
  destroyed";
- `completions/artifacts/RC-rc24/protocol.md` section 2.3, item 1: the control "proves
  the chain a blind session rides: copy-in, the binary, the invocation log, copy-out,
  a countable `doc show … --task`. If it does not fire, no blind result may be read."

And the exit code that says whether the arm itself passed is **not** the harness's:
`run-session.sh` exits 0 when the arm fails (its line 229: "the arm exited N — that
is data, not necessarily failure"), and `completions/trial-driver/driver/session.py`
(`arm_exit_code`, line 441) reads the arm's own code from the `exit-code` line of
`PROVENANCE.txt`. `walk.py`, the transport the protocol's arm table names, stops on
that value for arm 00 (its `got["rc"] != 0` branch). So the score reads two things:
the count in the log, and `exit-code` in the provenance. S5 shows both can fail.

## S1 — the evidence I score is the evidence the run step left

```yaml
claim: "every file of OUT outside .git has the sha256 the run step's manifests record, and nothing in OUT is newer than the provenance but .git and .git/index"
verdict: CONFIRMED
setup:
  - none (OUT as the run step left it)
repro:
  - (cd "$OUT" && find . -type f -not -path './.git/*' -not -path './node_modules/*' -print0 | sort -z | xargs -0 shasum -a 256) > "$W/out-run-1.manifest.score-before"
  - sort "$R/out-run-1.manifest.before" > "$W/run-before.sorted" ; sort "$W/out-run-1.manifest.score-before" > "$W/score-before.sorted"
  - cmp "$W/run-before.sorted" "$W/score-before.sorted"
  - find "$OUT" -newer "$OUT/PROVENANCE.txt"
  - "the same manifest again after all my work, into $W/out-run-1.manifest.score-after; cmp with score-before"
expect:
  cmp: exit 0, both times
  find: .git and .git/index, nothing else
pinned-by: "UNPINNED: one evidence directory of one run"
```

Observed: 35 files in each manifest; the sorted `cmp` exits 0 (unsorted, the two
differ only in where one line, `./package.json`, sorts — a difference of sort locale
between my listing and the run step's); my before/after `cmp` exits 0. The two files the score reads:

    b11e64674f71d2c784340fed1ee96dce69575aea94bf79041ecbf73a37356f22  ./.jigc/logs/invocations.jsonl
    4b3373c777791a06680a8fd3cb6a0196959a14e50f08d5a7995e42a65de8546c  ./PROVENANCE.txt

Both are the hashes the run step's report gives under *For the score step*. `find
-newer` lists `.git` and `.git/index` and nothing else, as that report says; the
index is dated 10:11:03 local, and the `.git` directory itself 10:16:24 local — later
than any read that report describes (SCO-5).

## S2 — the pass condition, counted from the copied-out log

```yaml
claim: "the invocation log in OUT holds at least one doc show … --task … record, and that record is a read that succeeded"
verdict: CONFIRMED
setup:
  - S1
repro:
  - grep -c '"doc","show".*"--task"' "$OUT/.jigc/logs/invocations.jsonl"
  - "a structured read, $W/readlog.py: json-load each line; count records whose argv[0:2] is [doc, show] and whose argv holds --task; print timestamp, exit_code, output_bytes, error_code, finding_codes, binary_version for each"
  - grep -E '^(exit-code|session-start|jigc-sha|image-id)' "$OUT/PROVENANCE.txt"
expect:
  count: at least 1, by both reads
  exit-code: 0
pinned-by: "UNPINNED: the score of one arm of one run"
```

Observed. The grep — the arm's own, line 56 of the script — prints `1`. The
structured read prints 10 records, 1 of them the channel, 0 with a non-zero exit:

    n  timestamp             exit  bytes  argv
    1  2026-10-08T08:10:51Z  0     17     --version
    2  2026-10-08T08:10:51Z  0     8039   start --workflow record-decision "record that the ingest queue drops the oldest sample when it overflows"
    3  2026-10-08T08:10:51Z  0     32     doc create adr --title "Drop the oldest sample when the ingest queue overflows" --task TASK
    4  2026-10-08T08:10:51Z  0     60     doc set-slot ADDR#context --from-file - --task TASK
    5  2026-10-08T08:10:51Z  0     61     doc set-slot ADDR#decision --from-file - --task TASK
    6  2026-10-08T08:10:51Z  0     65     doc set-slot ADDR#consequences --from-file - --task TASK
    7  2026-10-08T08:10:51Z  0     328    doc show ADDR --task TASK                  <- the channel
    8  2026-10-08T08:10:51Z  0     516    task diff TASK
    9  2026-10-08T08:10:52Z  0     186    doc list --task TASK                       <- the round's door
    10 2026-10-08T08:10:52Z  0     232    task list

Every record: `error_code` null, `finding_codes` empty, `binary_version`
`1.0.0-rc.24`. `PROVENANCE.txt`: `exit-code 0`, `session-start 2026-10-08T08:10:51Z`,
`jigc-sha eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, `image-id
sha256:d37422f98361a9ca93d5e4afaf6ad117c8190acbb11ca371a6085fc268839d97`. No record is
older than `session-start`, so none predates the session; four authoring writes
(records 3–6) precede the read, so an occasion to read back existed.

My projection of the log to `[argv, exit_code, output_bytes]` is byte-equal to the
run step's (`cmp "$W/out-run-1.argv" "$R/out-run-1.argv"`, exit 0).

## S3 — the counted record is a real read: the candidate I hashed prints the same bytes

The log keeps a byte count and no content, and the arm shows only twelve lines of the
read. This block ties record 7 — and the three reads after it — to what the binary
prints.

```yaml
claim: "on a copy of OUT, the hash-asserted candidate answers the arm's four read verbs at exit 0 in exactly the byte counts the image's log records"
verdict: CONFIRMED
setup:
  - cp -Rp "$OUT" "$W/replay-copy" ; mkdir "$W/replay-home"
  - in $W/replay-copy, HOME=$W/replay-home, PATH=$S/bin/c1.a1:$PATH, a git identity of mine (score@example.invalid) — command -v jigc prints $S/bin/c1.a1/jigc
repro:
  - jigc doc show "$ADDR" --task "$TASK" > "$W/replay.show.out"
  - jigc task diff "$TASK" > "$W/replay.diff.out"
  - jigc doc list --task "$TASK" > "$W/replay.list.out"
  - jigc task list > "$W/replay.tasklist.out"
  - wc -c "$W"/replay.*.out
expect:
  exit: 0 for each
  bytes: 328, 516, 186, 232
pinned-by: "UNPINNED: one replay on one state"
```

Observed: four exits 0, four empty stderr files, and `wc -c` prints 328, 516, 186 and
232 — records 7, 8, 9 and 10. The copy's own log gained four records with those argv,
exit codes and byte counts. The 328 bytes are the staged decision record with its
CLI-owned front matter (`status: proposed`, `date: 2026-10-08`, `schema-version: 2`),
the full title, the three authored sections and the empty optional `## Options`
section — "state the author never typed", which is the read the script's comment at
its step 3 says the channel serves.

## S4 — the chain's other four links

```yaml
claim: "copy-in, the binary, and copy-out hold for OUT: the corpus history arrived, the image is the handed one and behaves as the candidate, and no container of it is left"
verdict: CONFIRMED
setup:
  - S1; my own fresh corpus for the image probe — $C/completions/trial-corpus-template/instantiate.sh "$W/walk-score" walk-score, then check-corpus.sh "$W/walk-score" (exit 0, "11 passed, 0 failed", HEAD 40d9233653b93c26e786a4e2caa3878deabe4602)
  - a probe of mine, $W/identity-probe.sh (not trial tooling) — jigc setup; git worktree add -q -b identity-probe-wt <a fresh dir>/wt; cd there; jigc doc list 2>ERR; print the exit code and the count of ERR lines starting "note: served from the main checkout at "
repro:
  - GIT_OPTIONAL_LOCKS=0 git -C "$OUT" log --format='%h %s' ; GIT_OPTIONAL_LOCKS=0 git -C "$OUT" merge-base --is-ancestor e546164ce002507a2249fad2c0625f62c92fff4d HEAD
  - docker image inspect "$T" --format 'id={{.Id}} os={{.Os}}/{{.Architecture}}'
  - docker ps -a --filter ancestor="$T" --format '{{.ID}} {{.Status}}'
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$W/identity-probe.sh" "$W/walk-score" "$W/out-identity-probe" "$T"
  - "host, candidate: in a copy of the corpus, HOME not the machine's, PATH=$S/bin/c1.a1:$PATH — bash $W/identity-probe.sh"
  - "host, previous release: the same with PATH=$S/bin/previous-91834b5e011d:$PATH"
expect:
  history: 8 commits, the source corpus's HEAD an ancestor
  image id: the one PROVENANCE.txt records
  containers: none
  notes: 1 in the image, 1 under the host candidate, 0 under the previous release
pinned-by: "UNPINNED: one image and one evidence directory"
```

Observed, link by link:

| link (protocol 2.3, item 1) | read | observed |
|---|---|---|
| copy-in | `git log` of `OUT`; the source corpus `$R/walk-c1` | 8 commits: the template's 7 (`162882d` … `e546164`) and `9582571 chore(jigc): install jigc workspace config` on top; `merge-base --is-ancestor` exits 0; the source is at `e546164c…` with 7 commits |
| the binary, by label | `PROVENANCE.txt` against the prompt | `jigc-sha` is the candidate's commit; `image-id` is `sha256:d37422f9…839d97`, which is what `docker image inspect` prints for the handed tag now (`linux/arm64`) |
| the binary, by behaviour | the worktree probe | below |
| the invocation log | S2 | 10 records, readable, all this session's |
| copy-out | `docker ps -a --filter ancestor=…` | no line: no container of the image exists, after the run's session and after my four |
| a countable `doc show … --task` | S2, S3 | 1, exit 0, 328 bytes |

The probe, every `doc list` at exit 0 with stdout `jigc doc list — no committed docs`:

| where | binary | `note: served from the main checkout at …` lines on stderr | stderr lines |
|---|---|---|---|
| image `$T` through `run-session.sh --exec` (its `PROVENANCE.txt`: the same `image-id`, `exit-code 0`) | the image's | 1 | 1 |
| host | candidate, `command -v jigc` = `$S/bin/c1.a1/jigc` | 1 | 1 |
| host | previous release, `command -v jigc` = `$S/bin/previous-91834b5e011d/jigc` | 0 | 0 |

The labels alone could not say which binary ran: the image's `--version` and the
`binary_version` of all ten records read `1.0.0-rc.24`, the previous release's string,
as the contract says a local candidate's is. The probe says it: the image the arm ran
in behaves as the candidate at the round's door, and not as the previous release.

**Not checked against a gate record.** `run.py observe` reports the provenance as
`ungated` (S6); its `--gate <record.json>` check needs the isolation record of the
image, which this prompt does not hand me. What stands in for it here is the two rows
of the table above.

## S5 — the score can fail, and where it cannot (SCO-2, lead SCO-3)

A pass condition that no evidence could fail would make S2 empty. Three sessions in
the image, on one corpus, through the harness, differing in line 49 of the script
only (`jigc doc show "$ADDR" --task "$TASK" | head -12`).

```yaml
claim: "with the channel read removed the arm fails and the provenance says so, while run-session.sh still exits 0; with the channel read pointed at a doc that does not exist the read exits 1 and the arm passes"
verdict: CONFIRMED
setup:
  - the corpus $W/walk-score (S4); it is never written to (no .jigc in it after the four sessions)
  - sed '49d' "$ARM" > "$W/mutant-a-no-read.sh"
  - sed '49s/"\$ADDR"/"adr:no-such-doc"/' "$ARM" > "$W/mutant-b-failed-read.sh"
  - diff of each against $ARM shows line 49 and nothing else
repro:
  - env -u CLAUDE_CODE_OAUTH_TOKEN CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING=placeholder-not-a-credential "$RUN" --exec "$ARM" "$W/walk-score" "$W/out-control" "$T"
  - the same with "$W/mutant-a-no-read.sh" and "$W/out-mutant-a"
  - the same with "$W/mutant-b-failed-read.sh" and "$W/out-mutant-b"
  - "for each out-dir: grep exit-code PROVENANCE.txt; grep -c '\"doc\",\"show\".*\"--task\"' .jigc/logs/invocations.jsonl; the [argv, exit_code, output_bytes] projection; diff against the control's"
expect:
  control: exit-code 0, count 1, PASS
  mutant a: exit-code 1, count 0, FAIL
  mutant b: the read exits 1 — and the arm's verdict is the finding
pinned-by: "UNPINNED: a bound on an instrument, for triage"
```

Observed:

| session | `run-session.sh` exit | the script's last line | `exit-code` in `PROVENANCE.txt` | grep count | records | the difference from the control's projection |
|---|---|---|---|---|---|---|
| control (the script as it stands) | 0 | `ARM 0 PASS — the channel fires and is countable` | 0 | 1 | 10 | none — and byte-equal to `OUT`'s (`cmp`, exit 0) |
| mutant a (no read) | 0 | `ARM 0 FAIL — fix the instrument before reading any blind session` | 1 | 0 | 9 | record 7 absent |
| mutant b (a read that fails) | 0 | `ARM 0 PASS — the channel fires and is countable` | 0 | 1 | 10 | record 7 is `[["doc", "show", "adr:no-such-doc", "--task", TASK], 1, 320]`, `finding_codes` `["store.not-staged"]` |

Three readings:

- **The score discriminates on presence.** Without the read, the count is 0 and the
  arm's own exit code is 1. So `OUT`'s count of 1 and `exit-code 0` are not what any
  session would have left.
- **`run-session.sh`'s exit status carries none of it** (SCO-3): it is 0 in all three
  rows, the failed one included. The run step's verdict opens with "`run-session.sh
  --exec` exits 0"; that fact is true and says nothing about the arm. The provenance
  line and the count are what say it, and the run step's report gives both.
- **The score does not discriminate on success** (SCO-2). In mutant b the one read of
  the channel exits 1 with a refusal, 320 bytes of it, and the arm prints `PASS` at
  exit 0: line 56 counts lines matching a pattern, with no term for `exit_code`, and
  line 49's status is the status of a pipeline into `head` that nothing reads (the
  script sets `pipefail` and not `-e`). The reader agrees with the arm: `run.py
  observe` grades mutant b `VERB 1`, `read back through the fence's verb` (S6).

The third reading is the registered wording, not a slip of the script against it. The
arm's header and the protocol both ask for a *record*, and the reader's own source
says so and carries the other number beside it: `driver/observe.py`, at the
`verb_succeeded` field, "`verb` counts what §3.3 registers … which is appearances,
not successes … the choice is the protocol's, not the reader's". So I return it as a
bound on what a green control proves, with one driven instance. **It does not touch
this score**: S2 reads record 7's exit code (0) and S3 matches its 328 bytes.

## S6 — the registered reader's row for the evidence (lead SCO-4)

```yaml
claim: "run.py observe over a copy of OUT grades it 'read back through the fence's verb' with VERB 1, and exits 1 for the missing transcript; over the failed mutant it prints a void row and exits 1 too"
verdict: CONFIRMED
setup:
  - mkdir "$W/observe-copy" && cp -Rp "$OUT" "$W/observe-copy/out-run-1"
  - S5's two mutant out-dirs
repro:
  - python3 "$C/completions/trial-driver/run.py" observe "$W/observe-copy/out-run-1"
  - python3 "$C/completions/trial-driver/run.py" observe "$W/out-mutant-b" "$W/out-mutant-a"
expect:
  row: recs 10, wrote 4, VERB 1, adj 2, fs 0
  exit: 1
pinned-by: "UNPINNED: one read of one evidence directory"
```

Observed — first call, exit 1; stdout, then stderr:

    session           recs wrote  VERB  adj  fs  outcome
      provenance: ungated: jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a / eeffe347324f (pass --gate <record.json> to check it)
    out-run-1           10     4     1    2   0       read back through the fence's verb

      ! no transcript — the FILESYSTEM channel is unmeasured for this session

Second call, exit 1, the same stderr line once per session:

    out-mutant-b        10     4     1    2   0       read back through the fence's verb
    out-mutant-a         9     4     0    2   0  VOID apparatus — the session did not run

`read back through the fence's verb` is row 7 of the protocol's registered outcome
table (section 7.1), kind `score`; the table is walked top-down and the first match
wins, so the evidence does not reach row 10 (`unmeasured — no transcript`, kind
`void`). The row agrees with S2 in every cell: 10 records, 4 authoring writes, 1 read
through the verb, 2 adjacent reads (`task diff`, `doc list --task`).

The exit status is 1 for the passing control, for mutant b and for the failed mutant
a alike — it is the transcript branch's (`run.py`, `if o.transcript_missing:` sets
`rc = 1`), and a scripted arm has no transcript (`OUT/.session-transcript/` holds 0
files). So the reader's exit status is not the score of a scripted arm; its outcome
column is, and that column separates the passing evidence from the failed one. This
is the rehearsal's ACR-4 and the run's RUN-2, met again; the mutant rows are what I
add.

## S7 — the round's door, as this evidence reaches it (SCO-1)

The item's door list is one door, `jigc doc list`. The evidence reaches it once:
record 9, `doc list --task TASK`, exit 0, 186 bytes.

```yaml
claim: "the arm's one read of the door leaves the same record under the candidate and under the previous release, and the round's change at the door is on a path that read does not take"
verdict: CONFIRMED
setup:
  - S3's copy and outputs; cp -Rp "$OUT" "$W/prev-copy" ; mkdir "$W/prev-home"
  - in $W/prev-copy, HOME=$W/prev-home, PATH=$S/bin/previous-91834b5e011d:$PATH — command -v jigc prints $S/bin/previous-91834b5e011d/jigc
repro:
  - jigc doc list --task "$TASK" > "$W/prev.list.out" 2> "$W/prev.list.err"
  - jigc doc list > "$W/prev.committed.out" 2> "$W/prev.committed.err"
  - "in $W/replay-copy under the candidate: jigc doc list > $W/replay.committed.out 2> $W/replay.committed.err"
  - cmp "$W/prev.list.out" "$W/replay.list.out"
  - cmp "$W/prev.committed.out" "$W/replay.committed.out" ; cmp "$W/prev.committed.err" "$W/replay.committed.err"
  - grep -n 'doc list\|"doc","list"' "$ARM"
  - grep -n 'fn run_list(\|run_list_staged(\|served_from_home_note(' "$C/crates/cli/src/doc.rs"
expect:
  exit: 0 for every read and every cmp
pinned-by: "UNPINNED: a bound on an instrument, for triage"
```

Observed:

- Both binaries answer `doc list --task TASK` at exit 0 with the same 186 bytes and an
  empty stderr (`cmp` exits 0): the two staged docs, `adr:drop-the-oldest-sample-when`
  and `commit:record-that-the-ingest-queue`, each `managed`. So record 9 —
  `[argv, 0, 186]` — is what either binary would have left.
- From the main checkout the committed read is the same under both as well: stdout
  `jigc doc list — no committed docs` (36 bytes) and one stderr note of 206 bytes
  naming the open task and the staged listing; both `cmp` exit 0.
- The two binaries differ at this door only from a linked worktree (S4's table: 1
  note against 0). The arm never leaves the main checkout.
- The script names the door on two lines (the grep: 63 and 64). Line 63 is the one
  invocation, `jigc doc list --task "$TASK" >/dev/null 2>&1`; line 64 prints a count
  and compares nothing. Its output and its exit status are discarded, so the door
  failing there would change one `exit_code` in the log and neither the arm's exit
  nor its `PASS` line.
- In `crates/cli/src/doc.rs` at the candidate's commit: `fn run_list` is line 4801 and
  returns into `run_list_staged` at line 4808 whenever `--task` is given; the call
  `served_from_home_note(cwd, &jigc_home)` is at line 4975, after that return. The
  grep finds the call form on lines 4424, 4469 (inside the helper, defined at 4467)
  and 4975, and none after line 5006, where `run_list_staged` begins.

So the item's green says this about its door: `jigc doc list --task` ran once in the
image at exit 0 and printed the staged listing's byte count. It is not evidence,
either way, about the behaviour the round changed.

## The handed reports' claims, as the score met them

I was handed the rehearsal's and the run's reports. For a trial arm I score; this
table says which of their claims the score depends on and re-drove, and which it did
not touch.

| their claim | does the score rest on it | re-driven here | what I observed |
|---|---|---|---|
| run A2/A3: the arm ran to `ARM 0 PASS`, `exit-code 0`, ten records, one `doc show … --task` | yes | yes — from the evidence, S1, S2 | as reported; the two file hashes and the ten-record projection are equal to the run step's |
| run A3: copy-in 8 commits, no container, the source corpus untouched | yes | yes for the history and the containers (S4); for the source corpus I read its HEAD and commit count only | as reported |
| run A4: a second run cannot overwrite the evidence | no | no | the evidence is unchanged since the run step's manifests (S1), which is the consequence the score needs |
| run A5 / rehearsal R3: every run of the script in this image leaves the same ten records | no | in part — my control session (S5) is a fourth image run and a third corpus, and is byte-equal to `OUT`'s projection | equal |
| run A6: the door's 186 bytes under the host candidate | yes, for SCO-1 | yes (S3) | 186 |
| run A7 / rehearsal R5: 1 note in the image, 1 under the candidate, 0 under the previous release | yes, for the binary link | yes, with a probe and a corpus of my own (S4) | 1, 1, 0 |
| rehearsal R5: the arm's ten records are equal under the previous release | yes, for SCO-1 | in part — I compared the door's two reads and the channel read on copies of `OUT` under both binaries (S7; `doc show` is byte-equal too, 328); I did not re-run the whole arm under the previous release | equal where compared |
| rehearsal ACR-4 / run RUN-2: `run.py observe` exits 1 on a scripted arm | yes, for how the score is read | yes (S6) | exit 1, row 7 |
| rehearsal ACR-1: the harness refuses with no token variable | no | no | the variable is set in this stage (I read only whether it is non-empty) and I passed the same placeholder the two steps did |
| rehearsal ACR-3: the script goes on after a failed `cd /work` outside the container | no | no | I ran the script only in the container; this host has no `/work` |
| rehearsal ACR-5: `.jigc/config/manifest.yaml` untracked | no | no | the file is in `OUT`'s manifest; the run step read it against the design and returned no finding |
| run RUN-3: the provenance names a model and a permission mode no session used | no | read only | `OUT/PROVENANCE.txt` carries `model claude-sonnet-5` and `permissions bypassPermissions`; the score reads neither line |

## Findings

| id | lead | title | door | clause | severity | class | count, and how it was reached | repro |
|---|---|---|---|---|---|---|---|---|
| SCO-1 | no | a green `arm-control` is not evidence about the round's change at its door: the evidence reaches `jigc doc list` once, as the staged read from the main checkout, and both binaries leave the same record there (the rehearsal's ACR-2 and the run's RUN-1, re-driven) | jigc doc list | none broken, as I read it — it bounds what this item's green is evidence of under `usable-by-agents` | information for triage | an item counted as covering a door whose changed path it does not reach | instance, bounded for arm 00 only. In the evidence: 1 of 10 records names the door (the structured read). In the script: 2 lines name it (grep, 63 and 64), 1 an invocation. In `doc.rs`: 3 lines carry the call form `served_from_home_note(` (grep: 4424, 4469, 4975), 0 inside `run_list_staged`; the `--task` return is line 4808 (read). Driven: 4 reads of the door on copies of the evidence (2 binaries, staged and committed, equal pairwise) and 3 from a linked worktree (1, 1, 0 notes). Not enumerated: the other 23 walk arms — `doc list` appears in 14 of the 24 scripts by grep; none of the other 13 was read and none is run by this item | S7 — the round's door, as this evidence reaches it (SCO-1) |
| SCO-2 | no | the positive control's pass condition counts a `doc show … --task` record whether or not the read succeeded: with the read pointed at a doc that does not exist it exits 1 and the arm prints `ARM 0 PASS` at exit 0 | `completions/trial-driver/arms/walk/00-positive-control.sh` — unlisted | none broken — it bounds what the instrument of `usable-by-agents` proves; this run's counted record did succeed (exit 0, 328 bytes matched) | information for triage | a pass condition a failed invocation satisfies | instance, bounded for arm 00's pass condition: 1 pass-condition line in the script (56, a `grep -c` with no `exit_code` term) and 1 channel invocation (49, a pipeline whose status nothing reads) — both read; 1 mutant driven in the image against 1 control. The reader counts the same way by declared choice (`driver/observe.py`, the `verb_succeeded` field's comment, read; `verb_ok` is computed at line 624) and graded the mutant `VERB 1`. Not enumerated: the pass lines of the other 23 walk arms, and whether any caller reads `verb_succeeded` | S5 — the score can fail, and where it cannot (SCO-2, lead SCO-3) |
| SCO-3 | yes | `run-session.sh --exec` exits 0 when the arm fails; only the `exit-code` line of `PROVENANCE.txt` carries the arm's verdict, so a report that leads with the harness's exit has said nothing about the arm | `completions/trial-harness/run-session.sh --exec` — unlisted | none | low — declared in the harness (line 229) and in `driver/session.py` (`arm_exit_code`), and both `walk.py` and `run.py observe` read the provenance line | instance, unbounded | instance, unbounded — 3 sessions driven (exit 0 each; provenance 0, 1, 0); by grep `RUN_RC` is on 4 lines of the harness: initialised at 215, set from the container's exit at 227, echoed at 229, written to the provenance at 261, and on no `exit` line. The other three modes of the harness were not driven | S5 — the score can fail, and where it cannot (SCO-2, lead SCO-3) |
| SCO-4 | yes | `run.py observe` exits 1 over a scripted arm's evidence whether the arm passed or failed; only its outcome column separates the two (the rehearsal's ACR-4 and the run's RUN-2, met again, with a failed arm beside it) | `completions/trial-driver/run.py observe` — unlisted | none | information for whoever reads the score by exit status | instance, unbounded | instance, unbounded — 3 out-dirs read (a copy of the evidence, 2 mutants), exit 1 each; 1 branch read (`if o.transcript_missing:`). The file's other `rc = 1` sites were not read | S6 — the registered reader's row for the evidence (lead SCO-4) |
| SCO-5 | yes | the evidence directory's `.git` was touched after the run step's last described read: the directory is dated 10:16:24 local, its newest entry (`index`) 10:11:03 | `<scratch>/arm-control-run.zis4p3/out-run-1` — unlisted | none | low — nothing the score reads moved | instance, unbounded | instance, unbounded — 1 directory; `find -newer PROVENANCE.txt` lists 2 entries (`.git`, `.git/index`); the 35 files outside `.git` are hash-equal to the run step's two manifests; HEAD is `9582571a…` with 8 commits. I did not establish what touched the directory: a `git status` that takes and drops the index lock would leave exactly this, and that is a guess | S1 — the evidence I score is the evidence the run step left |

No finding against `jigc` itself came out of the score: ten invocations in the
evidence, ten at exit 0, and the four reads I replayed print the bytes the log
counted.

## What I did not do

- **No assistant session, and no spawn.** Arm 00 is scripted: nothing here is a blind
  session, a headless turn or a sub-agent launch, and none is part of this arm. The
  genuine concurrent-spawn artifact is the orchestrator's main-session half; I did
  not run it, and nothing in this report is evidence about it. This unit has no
  fan-out, no join and no adapter spawn template, so there was no N-process sim to
  run and no template to render either.
- **No divergent-order re-run.** The arm has no join and no merge, so there is no
  order to diverge. What I compared instead is one more unchanged run against the
  evidence (S5's control: equal).
- **I did not score the arm against a gate record** (S4, S6): `observe` ran ungated.
- **I did not run `verify-image.sh` or `verify-pair.sh`**, and I built no image.
- **I did not re-run the whole arm under the previous release**, nor the rehearsal's
  ACR-1 and ACR-3.
- **I did not drive the opening record's seeded claim.** It is the verifier's.
- **I did not write the item's result.** The outcome I read is green; the record step
  writes it.
- **I read no report but the two this prompt hands me.**
- Files of mine: everything is under `$W` — the copies of the evidence, the fresh
  corpus `walk-score`, five out-dirs of image sessions that are **not** the arm's run
  (`out-control`, `out-mutant-a`, `out-mutant-b`, `out-identity-probe`, and
  `observe-copy`, a copy), the two mutant scripts and the probe script (written
  through the shell; none is trial tooling and none is in the repository), two small
  Python readers, and the captured outputs. Outside the scratch root: two empty-or-
  one-line stderr captures the probe minted with `mktemp` in the system temp
  directory on the host. Nothing was removed.

<!-- end of report -->
