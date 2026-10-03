# Runbook — driving this trial

Ordered. Every step names what it expects and what stops the trial. Run from the repository root,
on a checkout that carries the `seed` repair. One arm at a time.

**The forks were taken before anything here was run — [forks.md](forks.md) lists each one with
how it was taken** (the file this runbook and [protocol.md](protocol.md) still call
`open-forks.md`; it was renamed when the trial's directory was committed). F1 fixed a turn file,
F5 added §9, and F6 could have stopped the trial at §4; it did not.

```sh
TAG=jigc-gate:registry-1.0.0-rc.24
TRIAL=<the trial directory>          # holds gate-rc24.json and protocol/
GATE=$TRIAL/gate-rc24.json
PASTE=$TRIAL/protocol/paste
TOOLS=$TRIAL/protocol/tools
RUN=completions/trial-driver/run.py
HARNESS=completions/trial-harness/run-session.sh
```

**Three standing rules** ([protocol.md](protocol.md) §10):

- **Every out-dir starts `~/out/RC24-`.** `~/out` already holds five trials' directories and
  `observe` scores whatever it is pointed at.
- **`--tag $TAG` on every driver command.** Every default is `jigc-gate:rc11`.
- **Never print an environment value.** The checks below print `set` or `MISSING`.

## 0 · Before anything

```sh
[ -n "${CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING:-}" ] && echo "token: set" || echo "token: MISSING"
[ -z "${JIGC_GATE_MODEL:-}" ] && echo "model: harness default" || echo "model: OVERRIDDEN — unset it"
find ~/out -maxdepth 1 -name 'RC24-*'   # expect: nothing
python3 $RUN test                    # all suites green, test_session.py's seed turn loop among them
```

A red suite stops here. `test_cascade.py` reads every `RC-*/protocol.md`; once this trial's
directory is committed it reads this one too, and [protocol.md](protocol.md) §7.1 is written to
pass it.

## 1 · The gate

```sh
python3 $RUN gate $GATE --tag $TAG
```

Expect `gated: jigc-gate:registry-1.0.0-rc.24`, `jigc 1.0.0-rc.24 (unknown)` and the CLI version.
`unknown` is the sha a registry image carries, by construction. A refusal means the tag was
rebuilt: the record no longer covers it, and it is re-verified before anything else runs.
`verify-pair.sh` is not run — it does not apply to a registry image.

## 2 · The reader's own control

```sh
python3 $RUN observe --archive
```

Expect `reproduces the 1.0.0-gate record's channel table exactly.` If the reader cannot reproduce
figures this repository settled by hand, nothing it says about a new session is believed.

## 3 · The corpora

[corpora.md](corpora.md), steps 1–5: instantiate, gate, adopt through the container, carry, check
the frozen state. At the end `~/ideas/fenwick-adopted`, `~/ideas/halloway-adopted` and
`~/ideas/calderby-adopted` each hold 9 commits and a clean tree, and `~/ideas/walk-rc24` is naive.

## 4 · The three preconditions — each stops the trial if it fails

Two reading aids, used from here on. They print; they score nothing.

```sh
# the invocation log as: ordinal · time · exit · argv · error code · finding codes
inv() { python3 -c 'import json,sys
for n,l in enumerate(open(sys.argv[1]),1):
    r=json.loads(l)
    print(n, r["timestamp"], r["exit_code"], " ".join(r["argv"])[:150],
          r.get("error_code") or "", ",".join(r.get("finding_codes") or []))
' "$1/.jigc/logs/invocations.jsonl"; }

# a headless turn's final message, and how many tool calls were denied
result() { python3 -c 'import json,sys
for l in open(sys.argv[1]):
    try: e=json.loads(l)
    except Exception: continue
    if e.get("type")=="result":
        print(e.get("result")); print("-- denials:", len(e.get("permission_denials") or []))
' "$1/stream.jsonl"; }

# the adoption commit: everything after it is the session's
adopt() { git -C "$1" log --format=%H --grep='adopt jigc for document management' -n1; }
```

### 4.1 · Walk arm 00, the positive control — FIRST

```sh
python3 completions/trial-driver/walk.py ~/ideas/walk-rc24 ~/out/RC24-walk --tag $TAG --only 00
grep -n 'ARM 0' ~/out/RC24-walk/walk-record.md
```

Expect `exit 0` and `ARM 0 PASS — the channel fires and is countable`. **If it fails, stop: no
blind session may be read against this rig.** The record lists arms 01–23 as `NOT RUN`, which is
the truth and is left as it is.

### 4.2 · The environment probe

```sh
$HARNESS --headless --prompt-file $PASTE/env-probe.txt ~/ideas/walk-rc24 ~/out/RC24-env $TAG
result ~/out/RC24-env
```

Expect two lines: `MAIN-SET` or `MAIN-UNSET`, then `SUB-SET` or `SUB-UNSET`.
[protocol.md](protocol.md) §6.3 fixes what each reading means. **`MAIN-UNSET` stops the trial and
goes to the human.** An answer in any other shape is an unread probe, not a pass: read the
transcript for the command's output before going on.

### 4.3 · The seed smoke

```sh
[ ! -e ~/out/RC24-smoke-frozen ] && [ ! -e ~/out/RC24-smoke-frozen-work ] || echo "STOP: name in use"
python3 $RUN seed ~/ideas/walk-rc24 $PASTE/seed-smoke-turns.txt ~/out/RC24-smoke-frozen \
    --tag $TAG --gate $GATE
result ~/out/RC24-smoke-frozen-work/turn02
```

Expect `seed` to print `frozen at …` with `turns   2`, and turn 2's reply to be `ALPHA BETA`. That
reply is the proof the second container resumed the first one's conversation. **If `seed` aborts,
or the reply lacks `ALPHA`, arms (a) and (b) do not run** until the driver is fixed; arm (c) does
not depend on it.

## 5 · The arms

Order: **(c), (b), (a)** — the arm that needs no `seed` first, the longest last. Before each:

```sh
python3 $RUN gate $GATE --tag $TAG
```

### 5.1 · Arm (c) — the disagreement

```sh
[ ! -e ~/out/RC24-C ] || echo "STOP: name in use"
$HARNESS --headless --prompt-file $PASTE/c-prompt.txt ~/ideas/calderby-adopted ~/out/RC24-C $TAG
```

### 5.2 · Arm (b) — the correction

```sh
[ ! -e ~/out/RC24-B-frozen ] && [ ! -e ~/out/RC24-B-frozen-work ] || echo "STOP: name in use"
python3 $RUN seed ~/ideas/halloway-adopted $PASTE/b-turns.txt ~/out/RC24-B-frozen \
    --tag $TAG --gate $GATE
```

### 5.3 · Arm (a) — the fan-out

```sh
[ ! -e ~/out/RC24-A-frozen ] && [ ! -e ~/out/RC24-A-frozen-work ] || echo "STOP: name in use"
python3 $RUN seed ~/ideas/fenwick-adopted $PASTE/a-turns.txt ~/out/RC24-A-frozen \
    --tag $TAG --gate $GATE
```

A seeded arm leaves, under `~/out/RC24-<X>-frozen-work/`: `turn01/` and `turn02/` (one out-dir per
turn — corpus, `.session-transcript/`, `stream.jsonl`, `PROVENANCE.txt`), `turn0N.prompt.txt`,
`corpus02/` and `corpus03/` (the tree each turn left, without the rig's files) and `stage02/`.
**`turn02/` is the arm's evidence**: its log, its `.git` and its main transcript are cumulative.

**If `seed` exits `seed turn N failed`**, that turn's CLI died: the arm is void (A-V / B-V), its
out-dirs are kept as they are, and a re-run takes a **new corpus and a new name**
(`RC24-A2-frozen`), with the reason in the record.

**Nothing is said to a session.** There is no reply channel and no answer key. A turn that ends on
a question is a result.

## 6 · `observe` — the read-back, one arm per command

```sh
python3 $RUN observe --gate $GATE ~/out/RC24-C
python3 $RUN observe --gate $GATE ~/out/RC24-B-frozen-work/turn01 ~/out/RC24-B-frozen-work/turn02
python3 $RUN observe --gate $GATE ~/out/RC24-A-frozen-work/turn01 ~/out/RC24-A-frozen-work/turn02
```

Each row is preceded by `provenance: jigc-gate:registry-1.0.0-rc.24 / jigc unknown`. A `REFUSED`
row is evidence from another image and is not scored.

Reading a seeded arm's two rows ([protocol.md](protocol.md) §8.2):

- `recs`, `wrote`, `VERB` and `adj` are **per turn**; the conversation's figure is their sum;
- turn 2's *"N record(s) predate this session (plant/adoption)"* is counting **turn 1's** records;
- `fs?`, `git!` and `HISTORY REWRITTEN` on turn 2 **repeat** turn 1's — read turn 2's only;
- a turn 2 that did little shows `VOID` at row 5 or 6. That is the split, not a dead rig.

**`HISTORY REWRITTEN … commit (amend)` on arm (b) is expected under a correct outcome**
([protocol.md](protocol.md) §8.3). `HALTED` and `ENDED ASKING` lines are read as written: the
first is exact, the second a heuristic.

## 7 · The rubric — the evidence for each arm's class

Write the evidence down, then assign the class ([protocol.md](protocol.md) §8.1). Read each turn's
`PROVENANCE.txt` for `exit-code` first: non-zero is void.

### 7.1 · Arm (c) → [protocol.md](protocol.md) §5.3

```sh
C=~/out/RC24-C
inv $C | grep -E ' (start|doc create|doc add-item|task finalize|task discard)( |$)'
ls $C/docs/inconsistencies/ 2>/dev/null                       # C-1: a doc here
git -C $C log --stat --format='%h %s' $(adopt $C)..HEAD        # which commit carried what
git -C $C diff --stat $(adopt $C) HEAD -- README.md package.json src/     # C-3: a site edited
grep -rnE 'in front of|long-term store' $C/src $C/README.md $C/package.json   # the claim, now
grep -rniE 'long-term|in front of|upstream|hands? on|forward' $C/docs 2>/dev/null | head -30
result $C                                                      # C-4: said, and only said
grep -c 'README.md\|package.json\|src/store.ts' $(find $C/.session-transcript -name '*.jsonl')
```

The last line is the occasion check: a transcript that never touched one of the three sites is
C-V, not C-5. For C-5's *propagated* sub-case, read what the architecture document says the
service hands on. For C-1, read the filed doc through its own sides:
reading `$C/docs/inconsistencies/` in the out-dir copy is the evidence, not a bypass.

### 7.2 · Arm (b) → [protocol.md](protocol.md) §4.3

```sh
B1=~/out/RC24-B-frozen-work/turn01; B2=~/out/RC24-B-frozen-work/turn02

# the occasion turn 1 left (B-V if any of the three is off)
git -C $B1 log --format='%h %s' $(adopt $B1)..HEAD             # expect: one commit
git -C $B1 log -1 --format=%B | grep -n 'TKT-'                  # expect: TKT-212, at HEAD
git -C $B1 grep -n 'TKT-' HEAD -- . ':!.jigc'                   # expect: nothing in the tree

# what turn 2 did
inv $B2 | grep -E ' (task amend|task finalize|task discard|start)( |$)'
git -C $B2 reflog --format='%h %gd %gs'
git -C $B2 log -1 --format=%B                                   # B-1: cites TKT-221, not TKT-212
git -C $B2 log -g --format='%h %gs%n%B' | grep -n 'TKT-'        # every message, superseded ones too
git -C $B2 rev-parse 'HEAD^{tree}'; git -C $B1 rev-parse 'HEAD^{tree}'   # B-1: the same tree
python3 $TOOLS/raw-git-acts.py $B2                              # B-3: a rewrite outside jigc
result $B1; result $B2
```

B-1 needs all of: a `task amend` record at exit 0 **after** turn 2's `session-start`, that task's
`task finalize` at exit 0, one `commit (amend)` reflog line, the message, the equal trees, and no
raw history act in turn 2's part of the transcript.

### 7.3 · Arm (a) → [protocol.md](protocol.md) §3.3

```sh
A1=~/out/RC24-A-frozen-work/turn01; A2=~/out/RC24-A-frozen-work/turn02

inv $A2 | grep -E ' (milestone|workflow sub-task|start --task|start --workflow|task finalize|task discard)( |$)'
git -C $A2 log --stat --format='%h %s' $(adopt $A2)..HEAD        # the boundary commit, and what it carried
ls $A2/docs/milestone-records/ 2>/dev/null
git -C $A2 worktree list; ls $A2/.jigc/worktrees 2>/dev/null
grep -rho '/work/\.jigc/worktrees/[a-z0-9-]*' $A1/.session-transcript $A2/.session-transcript \
    | sort | uniq -c                                             # which worktrees were worked in
find $A1/.session-transcript $A2/.session-transcript -path '*subagents*' -name '*.jsonl'
python3 $TOOLS/raw-git-acts.py $A2
result $A1; result $A2
(cd $A2 && node --test test/*.test.ts 2>&1 | tail -6)            # does the landed tree pass? node >= 22.6
```

A-1 needs `milestone provision` at exit 0, work in at least two worktrees, and
`milestone finalize <id>` at exit 0 with the three pieces' files in its commit. Where the first
`provision` came from — after `milestone execute`, after a refused `start --task`, after
`milestone --help` — is read from the ordinals just before it. A `find` line under `subagents`
whose transcript works inside a worktree is the *sub-agents* note; none is the *main agent* note.

## 8 · The trailer check → [protocol.md](protocol.md) §6

```sh
since() { awk '/^session-start/{print $2}' "$1/PROVENANCE.txt"; }

python3 $TOOLS/trailer-rows.py $C
python3 $TOOLS/trailer-rows.py $B2 --since "$(since $B1)"
python3 $TOOLS/trailer-rows.py $A2 --since "$(since $A1)"

python3 $TOOLS/raw-git-acts.py $C                               # (b) and (a) were printed in §7

# the control: jigc's install commit, made outside the agent — expect an EMPTY trailer column
for d in $C $B2 $A2; do
  git -C $d log --grep='install jigc workspace config' \
      --format='%h %s | %(trailers:key=Co-Authored-By,valueonly,separator=%x2C)'
done
```

For a seeded arm `--since` is **turn 1's** start and the out-dir is **turn 2's**, so both turns'
commits are in scope and neither adoption commit is.

Then, by hand:

1. **Every `exact`, `variant`, `NONE` and `DUPLICATE` row is checked against the raw git acts.**
   The join is by time; a raw `git commit` inside a door's window is credited to the door. A row
   whose commit a raw act made is moved to `not-jigc`.
2. **Every `variant` gets its cause**: the `doc add-item commit:<task>#trailers` record in the
   log, by ordinal, and the value the worker set.
3. **Every `NONE` is read against the probe**: which process ran the door — the main session or a
   sub-agent — and what §4.2 said about that process.
4. **A `door with NO commit matched` line is explained or it is a row**: a refused or no-op door
   at exit 0 is expected to print one; a door that says it committed and left no commit is not.
5. **`reflog` in the third column** marks a commit a later amend superseded. It is still a commit
   jigc made and is still checked.

The helper's output is saved as each arm's `trailer-rows.txt`: `.git` is not committed, so it is
the only committed trace of this check.

## 9 · The debrief — only if [open-forks.md](open-forks.md) F5 is taken

After §6–§8 are read, never before. It resumes the frozen conversation for one more turn in a new
out-dir, so the scored out-dirs are untouched:

```sh
python3 $RUN fork ~/out/RC24-B-frozen ~/out/RC24-B-frozen-work/corpus03 \
    $PASTE/debrief-prompt.txt ~/out/RC24-B-debrief --tag $TAG --gate $GATE
python3 $RUN fork ~/out/RC24-A-frozen ~/out/RC24-A-frozen-work/corpus03 \
    $PASTE/debrief-prompt.txt ~/out/RC24-A-debrief --tag $TAG --gate $GATE
result ~/out/RC24-B-debrief; result ~/out/RC24-A-debrief
```

`fork` prints `the fork did NOT inherit the seed` when the resume began cold; that debrief is void.
Arm (c) has no frozen conversation and so no debrief by this door.

## 10 · Evidence — where it is copied from, and to

Committed per arm, as `RC-rc14/evidence/` does: **`invocations.jsonl` and `PROVENANCE.txt`**, plus
this trial's `trailer-rows.txt`. **Never** a transcript, a `stream.jsonl`, a `stderr.txt`, a corpus
or a debrief's raw text — those stay on the machine that ran the trial, and the record quotes from
them.

```sh
E=$TRIAL/evidence
prov() { sed "s|$HOME|~|g" "$1"; }      # corpus-src, and the walk record's corpus / out / arm-script
                                        # lines, are absolute host paths as written

mkdir -p $E/C $E/B $E/A $E/walk
cp   $C/.jigc/logs/invocations.jsonl  $E/C/invocations.jsonl
prov $C/PROVENANCE.txt              > $E/C/PROVENANCE.txt
python3 $TOOLS/trailer-rows.py $C   > $E/C/trailer-rows.txt

cp   $B2/.jigc/logs/invocations.jsonl $E/B/invocations.jsonl          # cumulative: both turns
prov $B1/PROVENANCE.txt             > $E/B/PROVENANCE-turn01.txt
prov $B2/PROVENANCE.txt             > $E/B/PROVENANCE-turn02.txt
python3 $TOOLS/trailer-rows.py $B2 --since "$(since $B1)" > $E/B/trailer-rows.txt

cp   $A2/.jigc/logs/invocations.jsonl $E/A/invocations.jsonl
prov $A1/PROVENANCE.txt             > $E/A/PROVENANCE-turn01.txt
prov $A2/PROVENANCE.txt             > $E/A/PROVENANCE-turn02.txt
python3 $TOOLS/trailer-rows.py $A2 --since "$(since $A1)" > $E/A/trailer-rows.txt

prov ~/out/RC24-walk/walk-record.md > $E/walk/walk-record-rc24.md   # through the same rewrite:
                                        # copied with a plain `cp` it carries 5 host-path lines and
                                        # the check below says STOP (README.md → Tooling findings, T-5)
```

Then, before anything is committed:

```sh
grep -rn -e "$HOME" -e "$(id -un)" $E && echo "STOP: a host path or a login name is in the evidence"
grep -rn -iE 'token|oauth|secret' $E | head            # expect: nothing that is a value
```

An invocation log records what a worker typed as an intent. Read each one through once for a name
or an address before it is committed; the corpora were built with a neutral identity so that there
is none to quote ([corpora.md](corpora.md) §1). `dev/gate` runs the repository's own hygiene guard
on the commit.

## 11 · The record

[protocol.md](protocol.md) §9: one row per confirmed finding in the trial's `README.md` — tier,
door, repro, found-in, `pinned-by:` or `UNPINNED:` — every repro driven on a fresh corpus before it
is written. Beside the rows, the three arms' classes with their evidence, the trailer table, the
probe's two lines, and every declared change a session met, judged against §0.6.

Tier-2 and tier-3 rows do not block the call. A tier-1 row triggers a fix pass. **The 1.0.0 call
is the human's.**
