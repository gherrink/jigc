# canary-one · round 1 · `test` — verify-real, `r1-finalize-may-commit-redirected-log-record` (attempt 1)

One finding, handed over by triage with the grade *unclear*: the cross-cutting pass's `Lead L1` — prose, no repro block. Door `jigc task finalize`, clause `no-lost-files`.

## The verdict

**REFUTED — `does-not-reproduce`.** `jigc task finalize` did not carry the redirected log record into its commit, in either shape triage asked for. It commits the index and stages nothing of the working tree on the user's behalf, which is what its design section says of it.

- **Tracked-file shape (the reporter's cell D).** The record `jigc doc list` appended to `TRACKED.md` through the link stayed an unstaged edit. The finalize commit is `A greeting.txt` and nothing else; `TRACKED.md` at `HEAD` is the three lines that were committed by hand; no blob at `HEAD` holds a log record. The door named the file twice as left out, before the commit and after it.
- **Dangling-link shape (the reporter's cell C).** The root file the record minted, `minted-by-a-read.md`, stayed untracked. It is in no tree of the finalize commit, and the door named it as left out, twice.

The lead's premise is the part that does not hold: it reads `jigc task finalize` as *a door that stages the working tree on the user's behalf*. It is not one. [design/finalize.md](../../../../../../design/finalize.md) → *Dirty-tree policy*: *the commit set is the git index, not a sweep of the working tree* … *No `git add --all`. Untracked files and unstaged tracked edits are left out of the commit*; and → *5. Stage*: *this phase never sweeps bytes it does not own*. What was driven agrees with both sentences.

**Regression fact: not established, and not owed** — it goes with a confirmed verdict only. The previous release's binary was hashed and nothing was driven on it.

**Contested: no.** The finding does not argue that the index-honouring commit model is wrong.

**The clause's scope was not reached.** Whether a link planted at `.jigc/logs/invocations.jsonl` is *a healthy repository used as documented* or a *deliberately planted state* under the first clause's sharpening (`DECISIONS.md`, the entry of 2026-10-04, sharpening 1) is the question of the row that owns the write through the link. This finding falls before it: the commit does not hold the bytes.

## What was driven, and its identity

- **Candidate:** `<scratch>/bin/c1.a1/jigc`, label `c1`, commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`. `dev/stabilize-step hash … --file bin/c1.a1/jigc` printed `content_sha256` `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gave. The binary's directory was first on `PATH` in every call, and `command -v jigc` printed that path, exit 0.
- **Previous release:** `<scratch>/bin/previous-91834b5e011d/jigc`, the same call printed `content_sha256` `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gave. **Hashed, not driven.**
- **Rigs:** three, each `dev/jigc-rig fresh --binary <the candidate's path>`, each exit 0 read off the assignment and not off an `eval`, each under one directory of my own minted with `mktemp -d` under the scratch root. `HOME` was the rig's in every call. None is the reporter's.
- **Nothing built, nothing edited, nothing staged, nothing committed** in the repository.

**What I changed against the reporter's block.** `Repro F1` stops at `jigc doc list`; the lead has no block. I kept F1's setup as written and added the smallest task that reaches the door: `jigc start --workflow single-task "<intent>"`, one new file staged **by name**, the commit doc's `type` and `summary`, then `jigc task finalize <task>`. In the tracked-file shape the commit of `TRACKED.md` comes before the log is moved out, so that the hook's own `validate` record of that commit is moved out with the rest.

**Instance, unbounded.** I drove one committing door, `jigc task finalize` on the ordinary commit model, with two targets for the link, both in the repository's root. I enumerated no consumer of the mechanism: not the amend arm, not the doc-only arm, not the milestone doors, not `jigc setup`'s install commit, not `jigc rename`.

## How it went, step by step

Every exit status below was read bare, one `echo "rc=$?"` after the command, no pipe.

### Shape 1 — the link's target is a tracked file

Setup, all exit 0: `jigc config set invocation-log true` · `git add -A .jigc` · `git commit -q -m "switch the log on"` · write `TRACKED.md`, three lines · `git add -A .` · `git commit -q -m "a tracked file"` · move `.jigc/logs/invocations.jsonl` out of the repository (it held two `validate` records, one per commit, from the installed hook) · `ln -s ../../TRACKED.md .jigc/logs/invocations.jsonl`.

The precondition, which is the reporter's cell D and reproduced as written:

```text
$ git status --porcelain            (prints nothing, exit 0)
$ jigc doc list
jigc doc list — no committed docs   (exit 0)
$ git status --porcelain
 M TRACKED.md
$ git diff --stat
 TRACKED.md | 1 +
```

Then the task: `jigc start --workflow single-task "add a greeting file"` (exit 0, `task minted: add-a-greeting-file`) · write `greeting.txt` · `git add -- greeting.txt` (exit 0) · `jigc doc set-field commit:add-a-greeting-file#type --value feat --task add-a-greeting-file` (exit 0) · `jigc doc set-slot commit:add-a-greeting-file#summary --from-file - --task add-a-greeting-file` (exit 0). Before the door, `git status --porcelain` read ` M TRACKED.md` and `A  greeting.txt`, and `TRACKED.md` held four records after its three lines (`doc list`, `start`, `set-field`, `set-slot`).

The door:

```text
$ jigc task finalize add-a-greeting-file
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    TRACKED.md
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  at: task:add-a-greeting-file
  route: if the change is user-facing, record it — `jigc start --workflow record-change "<what changed>"`; if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized 2bced68 — feat: add a greeting file
  added greeting.txt
  1 file committed
  left-out (unstaged/untracked — git add to include):
    TRACKED.md
rc(finalize)=0
```

The commit, read:

```text
$ git rev-list --count <HEAD before>..HEAD
1
$ git show --name-status --format='%H %s' HEAD
2bced686acdefd36480ea54042055f40b572fc51 feat: add a greeting file

A	greeting.txt
$ git diff --exit-code HEAD~1 HEAD -- TRACKED.md      (prints nothing)
rc(diff)=0
$ git show HEAD:TRACKED.md
line one
line two
line three
$ git grep -c '"argv"' HEAD                           (prints nothing)
rc(git grep)=1
$ command grep -c '"argv"' TRACKED.md                 (the control: the working-tree file does hold them)
6
$ git status --porcelain
 M TRACKED.md
```

Six records stand in the working-tree file after the door — the four above, the hook's `validate`, and the door's own — and none of them is in a git object the commit reaches.

### Shape 2 — the link dangles, and the record mints an untracked root file

A second fresh rig. Setup, all exit 0: `jigc config set invocation-log true` · `git add -A .jigc` · `git commit -q -m "switch the log on"` · move the log out · `ln -s ../../minted-by-a-read.md .jigc/logs/invocations.jsonl`.

The precondition, the reporter's cell C, reproduced as written: `git status --porcelain` printed nothing; `jigc doc list` printed `jigc doc list — no committed docs`, exit 0; `git status --porcelain` then printed `?? minted-by-a-read.md`, and the file held the one `doc list` record.

The same task, the same four commands, each exit 0. Before the door: `A  greeting.txt` and `?? minted-by-a-read.md`.

```text
$ jigc task finalize add-a-greeting-file
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    minted-by-a-read.md
…
finalized a071f6a — feat: add a greeting file
  added greeting.txt
  1 file committed
  left-out (unstaged/untracked — git add to include):
    minted-by-a-read.md
rc(finalize)=0
$ git show --name-status --format='%H %s' HEAD
a071f6ae4c346b79de27a4842a7ad4abc6bfbf62 feat: add a greeting file

A	greeting.txt
$ git cat-file -e HEAD:minted-by-a-read.md
fatal: path 'minted-by-a-read.md' exists on disk, but not in 'HEAD'
rc(cat-file -e minted)=128
$ git grep -c '"argv"' HEAD                           (prints nothing)
rc(git grep)=1
$ command grep -c '"argv"' minted-by-a-read.md        (the control)
6
$ git status --porcelain
?? minted-by-a-read.md
```

`git ls-tree -r --name-only HEAD` listed twelve paths, `greeting.txt` among them and `minted-by-a-read.md` not.

### The control — the read is not blind, and the line reaches a commit only through a stage the user makes

A third fresh rig, shape 1's setup unchanged, and one command different: after `jigc start`, `git add -A` in place of `git add -- greeting.txt` — the user's own sweep, typed by the user. Before the door: `MM TRACKED.md`, `A  greeting.txt`.

```text
finalized 2fd991e — feat: add a greeting file
  modified TRACKED.md
  added greeting.txt
  2 files committed
  left-out (unstaged/untracked — git add to include):
    TRACKED.md
rc(finalize)=0
$ git diff --exit-code HEAD~1 HEAD -- TRACKED.md
@@ -1,3 +1,5 @@
 line one
 line two
 line three
+{"timestamp":"2026-10-08T09:06:19Z","argv":["doc","list"],…}
+{"timestamp":"2026-10-08T09:06:19Z","argv":["start","--workflow","single-task","add a greeting file"],…}
rc(diff)=1
$ git grep -c '"argv"' HEAD
HEAD:TRACKED.md:2
rc(git grep)=0
```

So the same reads that found nothing in shapes 1 and 2 find the line when it is there. And what this cell shows about the finding is its boundary, no more: the two records in that commit are the two that stood in the file when **the user's `git add -A`** ran; the door staged neither, and it named `TRACKED.md` as `modified` in the committed set. The later records stayed out. The index is, in the design's words, *the declared touch-set, expressed by `git add`* — the bytes are uninvited because of the write through the link, which is the reporter's `F1` and that row's to weigh, and they are in the commit because of a git command the user ran. Filed under *left open* below, for that row.

## Repro L1-verify

The test-in-waiting, in the pipeline's schema ([implementation/pinning.md](../../../../../../implementation/pinning.md) → §3).

```yaml
claim: "with invocation-log on and .jigc/logs/invocations.jsonl a link into the working tree, `jigc task finalize` commits the redirected log record"
verdict: REFUTED
basis: does-not-reproduce
binary: "candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, content_sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"
setup:
  - fixture: fresh                      # dev/jigc-rig fresh --binary <binary>; HOME is the rig's
  - ["jigc", "config", "set", "invocation-log", "true"]
  - ["git", "add", "-A", ".jigc"]
  - ["git", "commit", "-q", "-m", "switch the log on"]
  - "write TRACKED.md: three lines"
  - ["git", "add", "-A", "."]
  - ["git", "commit", "-q", "-m", "a tracked file"]
  - "move .jigc/logs/invocations.jsonl out of the repository"   # the two commits' pre-commit hook logged its own `jigc validate`
  - ["mkdir", "-p", ".jigc/logs"]
  - ["ln", "-s", "../../TRACKED.md", ".jigc/logs/invocations.jsonl"]
  - ["jigc", "doc", "list"]             # exit 0; `git status --porcelain` is now " M TRACKED.md"
  - ["jigc", "start", "--workflow", "single-task", "add a greeting file"]   # mints <task>
  - "write greeting.txt: one line"
  - ["git", "add", "--", "greeting.txt"]
  - ["jigc", "doc", "set-field", "commit:<task>#type", "--value", "feat", "--task", "<task>"]
  - ["jigc", "doc", "set-slot", "commit:<task>#summary", "--from-file", "-", "--task", "<task>"]   # stdin: one line
repro:
  - ["jigc", "task", "finalize", "<task>"]
  - ["git", "show", "--name-status", "--format=", "HEAD"]
  - ["git", "diff", "--exit-code", "HEAD~1", "HEAD", "--", "TRACKED.md"]
  - ["git", "grep", "-c", "\"argv\"", "HEAD"]
  - ["git", "status", "--porcelain"]
expect:
  exit: 0                               # of finalize
  stdout_contains:
    - "added greeting.txt"
    - "1 file committed"
    - "left-out (unstaged/untracked — git add to include):\n    TRACKED.md"
  committed_paths: ["greeting.txt"]     # `A greeting.txt`, and no other line
  diff_tracked_exit: 0                  # TRACKED.md is the same blob in HEAD and HEAD~1
  git_grep_exit: 1                      # no blob at HEAD holds a log record
  status_after: " M TRACKED.md"
  control: "the working-tree TRACKED.md holds six `\"argv\"` lines — assert it, so that a log that was never redirected cannot pass this test"
variant:                                # the dangling link — an untracked root file minted
  setup_instead:
    - "no TRACKED.md and no second commit"
    - ["ln", "-s", "../../minted-by-a-read.md", ".jigc/logs/invocations.jsonl"]   # the target is absent
  expect:
    exit: 0
    stdout_contains:
      - "1 file committed"
      - "left-out (unstaged/untracked — git add to include):\n    minted-by-a-read.md"
    committed_paths: ["greeting.txt"]
    cat_file_e_minted_exit: 128         # git cat-file -e HEAD:minted-by-a-read.md
    git_grep_exit: 1
    status_after: "?? minted-by-a-read.md"
pinned-by: "UNPINNED: no suite plants a link at the log's path, so nothing holds this fact with the link in place"
```

**Pinnable as it stands: yes.** Every step is an argv or a one-line file write, `fresh` is a state the shared fixture builder has, the task id is the one `jigc start` prints, and each assertion is an exit status, an exact `git` output or a substring of the door's stdout. Two things for whoever converts it: the record lines carry a timestamp and a duration, so the test counts `"argv"` lines and never compares a record whole; and the `control` line is the assertion that keeps the test from passing on a rig where the link was never followed.

**How `pinned-by` was derived** — from the suites, not from the diff. Under `crates/cli/tests` and `tooling-tests`, 38 files name the log (`invocations.jsonl`, `invocation-log` or `invocation_log`); 7 of the 38 also call `symlink(` or spell `ln -s`; I read the 13 lines those calls stand on and none plants its link at `.jigc/logs/invocations.jsonl` — the nearest is `uninstall_workbench_subject.rs:629`, a link at `.jigc/state/link`. The general fact under this one **is** pinned, without the link: `finalize_manifest::normal_finalize_leaves_unstaged_stray_uncommitted` (`crates/cli/tests/finalize_manifest.rs:207`) asserts, on the built binary, that an unstaged stray is not among the committed paths, stays untracked, and is named in the landed left-out list — read, not taken from its name. It covers an untracked stray; whether a suite asserts the same of an unstaged edit to a **tracked** file I did not establish.

## Left open — not pursued

1. **For the row that owns the write through the link (the reporter's `F1`), a fact about its reach.** A stage the user makes between the redirected write and the door — `git add -A`, or `git add TRACKED.md` — puts the records then standing in the file into the task's commit, at exit 0, the path named `modified` in the manifest. Driven once, as the control above (tracked-file shape, `git add -A`); the dangling shape under the same sweep was not driven. It is not this finding: no jigc command staged those bytes.
2. **A link whose target is a path the door stages itself.** *5. Stage* of the design section lists what finalize stages as its own: every promoted doc, the recorded owner-artifact paths, the `.jigc/version` stamp where it is jigc's to write, and on a first commit the config layer (`.jigc/config`, `.jigc/.gitignore`). A link at the log's path pointing at one of those — a file under `.jigc/config/`, a promote destination — was **not driven and not read**. Door: `jigc task finalize`. Clause: `no-lost-files`. Whether it is inside the clause's scope or a planted state is not mine to say.
3. **The other committing doors** — the amend arm, the doc-only arm, the milestone boundary and its record-only doors, `jigc setup`'s install commit, `jigc rename`, `migrate-corpus`. None was driven with the link in place. The design section itself says some of them commit without the carryover question.

## Not examined

- The previous release. Hashed, nothing driven on it: the regression fact goes with a confirmed verdict.
- Any layout but a single checkout: no linked worktree, no fan-out.
- Any other workflow than `single-task`, and no promoted doc in the task (no ADR, no changelog entry was created).
- Whether the write through the link is itself a defect, and whether it is in the clause's scope: that is the reporter's `F1`, another row, which I neither graded nor re-verified beyond reproducing its two cells as this finding's precondition.

<!-- end of report -->
