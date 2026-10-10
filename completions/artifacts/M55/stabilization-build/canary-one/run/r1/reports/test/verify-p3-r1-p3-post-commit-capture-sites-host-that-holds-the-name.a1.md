# Verification — `r1-p3-post-commit-capture-sites-host-that-holds-the-name`

Stabilization run `canary-one`, round 1, stage `test`, triage pass 3. ONE finding, handed:
item 7 under `Left open` of
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-git-capture-other-callers-not-enumerated.a1.md`
— *a host whose filesystem holds the name. There `task finalize` would not stop at the index
materialisation, and the four class-C sites that run after a commit could be reached under
`core.quotePath=false`. Each propagates its error, so what is open is a door exiting non-zero
after its commit landed. Read, not driven.* Door: `jigc task finalize` and
`jigc milestone finalize`. Clause it is said to break: `no-lost-files`. Triage's grade:
*unclear*. No repro block was handed; the setup below is reconstructed, and what I changed is
said under *What I changed to reach the door*.

## Verdict in one paragraph

**`refuted` — as a blocker, with the basis `breaks-no-clause`. It is not `does-not-reproduce`:
a door exiting non-zero after its commit landed is real, and I drove it at both doors — but
not in the state the finding names, and not with anything lost.** In the state the finding
names — a tracked file whose name holds the byte 0xFF, `core.quotePath=false`, on a volume
that accepts the name so that the index materialisation passes — both doors do one of two
things, and neither is the claim: where the name is in what the door would commit (staged as an
addition, staged as a deletion) or is an unstaged edit, the door **refuses at exit 1 before
the commit**, `HEAD` unmoved, the working area and the worktree standing; where the name is
tracked and clean, the door **lands at exit 0**. The post-commit sites are reached with a
capture they cannot decode only when the name enters the commit *behind* the door's own
pre-commit reads — I reached that with a line planted in `.git/hooks/pre-commit` that stages
the name while git is making the commit. There `jigc task finalize` and
`jigc milestone finalize` both exit **1** after their commit landed. In those cells the landed
commit holds exactly what the ASCII control's commit holds, nothing is destroyed, and the exit
is not 0 — so no sentence of the clause is broken, and the state is a planted one. One part of
the finding's reading is wrong on its own terms: of the two post-commit sites of
`jigc task finalize`, one does not propagate — its error is printed as a note and the door goes
on (the amend arm reaches it and exits 0).

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`,
  label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's. **It was hashed and not driven**: step 4 runs with `confirmed` only.
- The candidate's directory went first on `PATH` and `command -v jigc` printed the candidate's
  path. Each cell's driver repeats that check and stops if it fails, builds its rig with
  `dev/jigc-rig fresh --binary <candidate>` (two-step eval, stdout only) and stops unless the
  rig's `$JIGC` is that path and the shell stands in the rig's repository. The installed
  `pre-commit` hook names its binary by absolute path, so a hook run in a rig is the candidate.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host, Darwin 25.6.0, arm64.
  Both handed binaries are Mach-O arm64 executables, so neither can be driven in a Linux
  container.

## What I changed to reach the door

**No filesystem this host can mount without privileges holds a name that is not UTF-8.** Probed
with one file named `bad\377name.txt` (the byte 0xFF), created by a shell redirect:

| volume | creating the file | what the directory then lists |
|---|---|---|
| the scratch root (APFS) | refused — *illegal byte sequence* | nothing |
| disk image, Case-sensitive APFS | refused — *illegal byte sequence* | nothing |
| disk image, ExFAT | refused — *invalid argument* | nothing |
| disk image, UDF | refused — *file name too long* | nothing |
| disk image, MS-DOS FAT32 | exit 0 | `bad%FFname.txt` |
| disk image, Case-sensitive HFS+ | exit 0 | `bad%FFname.txt` |

So the nearest thing this host offers is the HFS+ image: it **accepts** the raw name on create,
open and `lstat`, stores it percent-escaped, and lists the escaped name. That is enough to pass
what the finding says stops the door on this host — git can materialise the entry — and it is
not a volume that *holds* the name: git sees the tracked entry as present and clean, and also
sees `bad%FFname.txt` as one untracked file. Every cell on the image therefore carries that
untracked file, and the ASCII controls do not. Four more things I changed or found on the way:

1. **The image.** A 96 MB Case-sensitive HFS+ image created with `hdiutil create` inside my
   scratch directory and attached with `hdiutil attach -nobrowse -mountpoint` at a directory
   inside it; rigs were built on it with `SCRATCH=<that mountpoint>/rigs`. All five images
   were detached and their files deleted when the driving was done; `hdiutil info` then named
   none of them.
2. **`TMPDIR`.** The door materialises the index under the system temporary directory, which
   is APFS, so with the repository on the image and nothing else changed it still stops at
   `git checkout-index … Illegal byte sequence` (cell `c2-hfs-add-ff-default`). The cells
   marked *tmp* run the finalize with `TMPDIR` pointed at a directory on the image, which is
   what a host that holds the name would give.
3. **Staging.** On that volume `git add -- <the raw name>` exits 0 and stages nothing (git's
   own behaviour; probed apart from jigc). The name is staged with
   `git update-index --add -- <name>`, the ASCII control the same way. Five earlier cells
   (`c-hfs-add-*`) used `git add` and are **void** for the 0xFF name: nothing was staged.
   Five milestone cells (`m-hfs-tmp-*`) are void too — the driver read the sub-task id wrongly
   and stopped before the door.
4. **The observer.** Every read the driver makes for the tables below is
   `git -c core.quotePath=true …`, so what is recorded is C-quoted and never the configuration
   under test.

`core.quotePath=false` was set in the repository's own config, at one of two moments: *pre* —
right after the rig is built, before any jigc command — or *fin* — immediately before the
finalize. *default* leaves it unset.

## What was driven — 40 cells on the candidate, plus 10 void

Each cell is one fresh rig (`fresh`). The task cells: `jigc start --workflow single-task`, one
line appended to `README.md` and staged, the commit doc's `type` and `summary` set, then
`jigc task finalize <task>`. The milestone cells: `jigc milestone create`, `add-task`,
`provision`, `code.txt` staged in the sub-task's worktree, then `jigc milestone finalize`.
`ascii` is `badXname.txt`; `ff` is `bad\377name.txt`.

### `jigc task finalize`, the name in what the door would commit

| cell | volume | how the name is staged | quotePath | exit | `HEAD` | what it said |
|---|---|---|---|---|---|---|
| `c2-hfs-add-ascii-default` · `-fin` | image | added | unset · false | 0 · 0 | moved | *finalized … added badXname.txt, 2 files committed* |
| `c3-hfs-tmp-add-ascii-default` | image, tmp | added | unset | 0 | moved | the same |
| `c2-hfs-add-ff-default` | image | added | unset | 1 | same | `git checkout-index -a --prefix=<tmp>/…` failed: *Illegal byte sequence* |
| `c3-hfs-tmp-add-ff-default` | image, tmp | added | unset | 1 | same | `git diff --cached` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 13 |
| `c2-hfs-add-ff-fin` · `-pre` | image | added | false | 1 · 1 | same | `git status` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 19 |
| `c3-hfs-tmp-add-ff-fin` | image, tmp | added | false | 1 | same | the same `git status` line |
| `c-apfs-del-ascii-default` · `c5-apfs-del-ascii-fin` | scratch root | deletion of a committed name | unset · false | 0 · 0 | moved | *finalized … deleted badXname.txt, 2 files committed* |
| `c-apfs-del-ff-default` | scratch root | deletion of a committed name | unset | 1 | same | the `git diff --cached` line |
| `c5-apfs-del-ff-fin` | scratch root | deletion of a committed name | false | 1 | same | the `git status` line |

In every exit-1 row the standard output is empty, `HEAD` is the commit it was, the staged
entries are still staged (`M  README.md` and the name), and `.jigc/tasks/<task>` stands.
**With `TMPDIR` on the image the door does get past the index materialisation — and stops at
the next read, still before the commit.** Under `core.quotePath=false` it never gets that far:
`git status` is read first.

### `jigc task finalize`, the name tracked and not in the commit

The name is committed with plain git before the task starts, its file present on the image.

| cell | the name's state | quotePath | exit | `HEAD` | what it said |
|---|---|---|---|---|---|
| `c4-hfs-tmp-base-ascii-pre` | clean | false | 0 | moved | *finalized … modified README.md, 1 file committed* |
| `c4-hfs-tmp-base-ff-pre` | clean | false | 0 | moved | the same, plus *left-out: bad%FFname.txt* |
| `c4-hfs-tmp-base-ff-default` | clean | unset | 0 | moved | the same |
| `c4-hfs-tmp-basemod-ascii-pre` | edited, unstaged | false | 0 | moved | *left-out: badXname.txt* |
| `c4-hfs-tmp-basemod-ff-pre` | edited, unstaged | false | 1 | same | the `git status` line; area stands |
| `c4-hfs-tmp-basemod-ff-default` | edited, unstaged | unset | 0 | moved | *left-out: "bad\377name.txt"* |

**This is the finding's own wording, driven — *such a tracked file under
`core.quotePath=false`* — and the exit after the commit lands is 0.** Both post-commit sites
ran and decoded: the landed commit's own delta is `README.md`.

### `jigc task finalize`, the amend arm — `jigc task amend`, then the finalize

`HEAD` is a plain-git commit whose own delta is the name; the amend rewrites its message. This
is the one arm where a post-commit site reads a delta the door's pre-commit reads never list.

| cell | quotePath | exit | `HEAD` | standard error |
|---|---|---|---|---|
| `a1-hfs-tmp-ascii-default` · `-pre` | unset · false | 0 · 0 | rewritten | empty |
| `a1-hfs-tmp-ff-default` | unset | 0 | rewritten | empty |
| `a1-hfs-tmp-ff-pre` · `-fin` | false | **0 · 0** | rewritten | `note: post-commit file-state update failed (self-heals): `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 3` |

`task.rs:9793` (`git show --name-only … HEAD`) reached with a capture it cannot decode: **the
error is printed as a note and the door exits 0** with its ordinary *amended … → …* summary.
The other site, `task.rs:9809`, is not called on this arm. `.jigc/state/file-state.json` is not
written in the two noted cells (it is in the controls); `jigc validate` run next exits 0 and
says *the committed store validates clean* in all of them.

### `jigc milestone finalize`

| cell | where the name is | quotePath | exit | `HEAD` | what it said |
|---|---|---|---|---|---|
| `m2-hfs-tmp-ascii-default` · `-fin` | staged in the sub-task's worktree | unset · false | 0 · 0 | moved | *finalized … added badXname.txt, added code.txt, modified docs/milestone-records/m-one.md* |
| `m2-hfs-tmp-ff-default` · `-fin` · `-pre` | staged in the sub-task's worktree | unset · false · false | 1 · 1 · 1 | same | `git diff --name-status` produced non-UTF-8: invalid utf-8 sequence of 1 bytes from index 5 |
| `m3-hfs-tmp-base-ascii-pre` | tracked, clean, committed before `milestone create` | false | 0 | moved | *finalized … added code.txt, …* |
| `m3-hfs-tmp-base-ff-pre` · `-default` | tracked, clean, committed before `milestone create` | false · unset | 0 · 0 | moved | the same (and see *Left open*, item 6) |

In the three exit-1 rows the worktree stands with `A  code.txt` and the name still staged, the
milestone's area and the sub-task's area stand, and `HEAD` is unmoved. The refusal is the same
under all three configurations, so it is a read that does not depend on `core.quotePath`.

### The mechanism, reached — a planted hook line stages the name while git makes the commit

One line of the user's own is put at the top of `.git/hooks/pre-commit`, above jigc's block.
*file*: it writes the file and runs `git update-index --add -- <name>` (image only). *entry*:
it runs `git update-index --add --cacheinfo 100644,<blob>,<name>`, an index entry with no
file, which needs no volume that takes the name.

| cell | door | volume, hook | quotePath | exit | `HEAD` | what it said |
|---|---|---|---|---|---|---|
| `h1-hfs-tmp-hook-ascii-pre` | task | image, file | false | 0 | moved | *finalized … added badXname.txt, 2 files committed* |
| `h1-hfs-tmp-hook-ff-default` | task | image, file | unset | 0 | moved | *finalized … added "bad\377name.txt", 2 files committed* |
| **`h1-hfs-tmp-hook-ff-pre`** | task | image, file | false | **1** | **moved** | the note above (index 13), then `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 17 |
| `c5-apfs-hookidx-ascii-pre` | task | scratch root, entry | false | 0 | moved | *finalized … added badXname.txt* |
| `c5-apfs-hookidx-ff-default` | task | scratch root, entry | unset | 0 | moved | *finalized … added "bad\377name.txt"* |
| **`c5-apfs-hookidx-ff-pre`** | task | scratch root, entry | false | **1** | **moved** | the same two lines as `h1-hfs-tmp-hook-ff-pre` |
| `mh1-hfs-tmp-hook-ascii-pre` | milestone | image, file | false | 0 | moved | *finalized … added badXname.txt, added code.txt, …* |
| `mh1-hfs-tmp-hook-ff-default` | milestone | image, file | unset | 0 | moved | *finalized … added "bad\377name.txt", …* |
| **`mh1-hfs-tmp-hook-ff-pre`** | milestone | image, file | false | **1** | **moved** | the note (index 3), then `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 5 |

In the three bold rows the commit **landed** and the door exited 1 with an empty standard
output — no *finalized* line, no hash. What stands afterwards:

- **task** — the landed commit is `M README.md` and `A "bad\377name.txt"`, the same two
  entries as the ASCII control's commit; `.jigc/tasks/<task>` is gone, as after a success;
  `.jigc/state/file-state.json` is not written.
- **milestone** — the landed commit is the name, `code.txt` and the milestone record, the same
  three entries as the control's; `.jigc/milestones/m-one` and `.jigc/tasks/sub-work` are
  gone; **the fan-out worktree `.jigc/worktrees/sub-work` is still registered and still
  there**, holding `A  code.txt` staged, which the landed commit also holds.

By the byte offsets in the messages: the note is `task.rs:9793` (`git show --name-only`), and
the line that sets the exit is `task.rs:9809` (`git show --name-status`, offset 17 =
`M<tab>README.md<newline>A<tab>bad` then the byte) at the task door and `milestone.rs:9475`
(`git diff --name-status`, offset 5 = `A<tab>bad` then the byte) at the milestone door.
`milestone.rs:9619` was not reached: the site before it had already failed.

## Is it what the finding says?

**No, in the state it names; yes, as a mechanism, in a state it does not name.**

- *There `task finalize` would not stop at the index materialisation.* So: with the repository
  and `TMPDIR` on a volume that accepts the name, it does not. It stops one read later.
- *The class-C sites that run after a commit could be reached under `core.quotePath=false`.*
  Not through a tracked file. A name in the commit's delta is, before the commit, a staged
  entry, and the door reads the staged set first — as `git status --porcelain` (raw under
  `core.quotePath=false`) and as `git diff --cached --no-renames --name-only -z`
  (`task.rs:8256`), which is raw under every configuration — and refuses. A name that is
  tracked and outside the delta is in neither the pre-commit reads nor the post-commit ones,
  and the door lands at exit 0. The sites are reached raw only when the delta gains the name
  after those reads: a hook that stages it, or an amend of a commit jigc did not make.
- *Each propagates its error.* Three of the four do. `task.rs:9793` sits inside the post-commit
  phase, whose failures are printed and not raised.

**Read against the design that owns it.** `design/finalize.md` → *7. Post-commit
(best-effort)*: *a failure in any of these is logged, not raised — the commit is real*, and
above it *the transaction is atomic up through `git commit`: any failure before that point
rolls back to "as if `finalize` was never called"*. The pre-commit refusals and the amend arm's
note are that design, as written. The exit 1 after a landed commit is not: the design says
*after phase 6, the commit defines truth and any post-commit residue is a chore, not a
transaction issue*, and here the read that builds the landed summary fails the command. So
that part is a defect, not intended behaviour — in the planted state only.

## Does it break the clause, inside the clause's scope?

The clause, `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, first sharpening: *in a
healthy repository used as documented — which includes ordinary git configuration … — no jigc
command at exit 0 destroys bytes no git object holds or commits content the user did not ask
for. Where jigc cannot tell (git fails, the index is unreadable) it refuses before writing.
Races … and deliberately planted states are declared bounds.*

- **In the state the finding names** nothing is written after a failed read at all: the door
  refuses before the commit, or it lands at exit 0 with every read decoded. Where it could not
  decode git's output it refused before writing, which is the clause's second sentence kept.
- **In the planted-hook cells** the exit is 1, not 0; no bytes are destroyed — the task's area
  goes as it does on success, its commit doc rendered into the landed message, and at the
  milestone door the worktree is left standing rather than removed; and the landed commit holds
  what the ASCII control's commit holds, the hook's file included, which is git's own rule for
  what a hook stages. The door did not write blind either: every read before the commit
  decoded, and the read that failed is the one that reports. And the state is a hook line that
  stages a file whose name is not UTF-8, in a repository configured `core.quotePath=false` —
  by my reading a deliberately planted state, which the clause puts outside its scope. That
  reading is mine; the verdict does not rest on it alone, since the first two points hold
  inside the scope as well.

So: **a real defect that breaks no sentence of `no-lost-files`.** A command that reports
failure after it succeeded is the second clause's question (`working-product`), and I did not
grade it there.

## The regression fact

**Not owed by this verdict**: step 4 runs with `confirmed` only. The previous release's binary
was hashed and not driven.

## Class

**`instance, unbounded`.** I drove two doors — `jigc task finalize` on its ordinary arm and
its amend arm, and `jigc milestone finalize` — and reached three of the four sites the finding
points at (`task.rs:9793`, `task.rs:9809`, `milestone.rs:9475`). I enumerated nothing: not the
other readers of a landed commit's names, not the other doors that commit and then report, not
the hook kinds other than `pre-commit`.

## Coverage

The finding makes no coverage claim, and none is verified here.

## Repro VR-PC-1

The fact this verdict wants to stay true, in the form that needs no special volume: a name
that is not UTF-8 in what the door would commit makes the door refuse before the commit, under
either quoting. The committed name has no file on a host that cannot hold it, so the task
stages its deletion.

```yaml
claim: "on a host whose filesystem holds a name that is not UTF-8, jigc task finalize and jigc milestone finalize reach their post-commit name captures raw under core.quotePath=false and exit non-zero after the commit landed — breaking no-lost-files"
verdict: "REFUTED as a blocker — breaks-no-clause. With the name in the commit's delta the door refuses before the commit; with the name tracked and clean it lands at exit 0. The exit after a landed commit is real only where a hook stages the name mid-commit (Repro VR-PC-2)."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347"
setup:
  - fixture: fresh
  - shell: "blob=$(printf 'planted\\n' | git hash-object -w --stdin)"
  - shell: "git update-index --add --cacheinfo \"100644,$blob,$(printf 'bad\\377name.txt')\""
  - ["git", "commit", "-q", "-m", "base: a tracked name"]
  - ["jigc", "start", "--workflow", "single-task", "touch the readme"]     # mints task touch-the-readme
  - shell: "printf 'one more line\\n' >> README.md && git add README.md"
  - shell: "git rm -q --cached -- \"$(printf 'bad\\377name.txt')\""
  - ["jigc", "doc", "set-field", "commit:touch-the-readme#type", "--value", "chore", "--task", "touch-the-readme"]
  - shell: "jigc doc set-slot commit:touch-the-readme#summary --from-file - --task touch-the-readme   # stdin: touch the readme"
variants:                      # each variant's step is appended AFTER the last setup step, as driven
  - name: default-quoting
    last_setup_step: null
    expect_stderr_contains: "`git diff --cached` produced non-UTF-8 output"
  - name: quotepath-false
    last_setup_step: ["git", "config", "core.quotePath", "false"]
    expect_stderr_contains: "`git status` produced non-UTF-8 output"
repro:
  - ["jigc", "task", "finalize", "touch-the-readme"]
expect:
  exit: 1
  stdout: ""
  assertions:
    - "git rev-parse HEAD is the commit it was before the finalize"
    - "git -c core.quotePath=true status --porcelain still lists `M  README.md` and `D  \"bad\\377name.txt\"`"
    - ".jigc/tasks/touch-the-readme still exists"
control: "the same block with the name badXname.txt exits 0 under both variants and lands `M README.md`, `D badXname.txt`"
pinned-by: "UNPINNED: a verifier writes no test"
```

**Pinnable as it stands**, on any host: it needs only git plumbing and the `fresh` fixture.
Two limits on what it pins. It pins the refusal, which is what keeps the post-commit sites out
of reach — it does not pin that refusing is the wanted answer for such a name (*Left open*,
item 3). And the arms that carry the finding's own wording — the name **added**, and the name
tracked and clean — are not pinnable as they stand on the macOS gate: they need a volume that
accepts the name (here a mounted disk image, with `TMPDIR` on it and `git update-index --add`
in place of `git add`). On a Linux runner the filesystem holds the name and those arms would
be ordinary tests behind a platform condition; nothing here drove one.

## Repro VR-PC-2

The mechanism, where it is real. A planted state; portable, because the hook stages an index
entry with no file.

```yaml
claim: "jigc task finalize exits non-zero after its commit landed, when the landed commit's delta holds a name that is not UTF-8 and core.quotePath=false"
verdict: "REPRODUCES — only where the name enters the commit behind the door's pre-commit reads (a pre-commit hook line that stages it). Breaks no sentence of no-lost-files: the exit is 1, nothing is destroyed, and the commit holds what the ASCII control's holds."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347"
setup:
  - fixture: fresh
  - ["git", "config", "core.quotePath", "false"]
  - ["jigc", "start", "--workflow", "single-task", "touch the readme"]     # mints task touch-the-readme
  - shell: "printf 'one more line\\n' >> README.md && git add README.md"
  - plant: ".git/hooks/pre-commit gains two lines directly under its first line: `blob=$(printf 'generated\\n' | git hash-object -w --stdin)` and `git update-index --add --cacheinfo \"100644,$blob,bad<0xFF>name.txt\"` (the name spelled with the raw byte)"
  - ["jigc", "doc", "set-field", "commit:touch-the-readme#type", "--value", "chore", "--task", "touch-the-readme"]
  - shell: "jigc doc set-slot commit:touch-the-readme#summary --from-file - --task touch-the-readme   # stdin: touch the readme"
repro:
  - ["jigc", "task", "finalize", "touch-the-readme"]
expect:
  exit: 1
  stdout: ""
  stderr_contains:
    - "note: post-commit file-state update failed (self-heals): `git` produced non-UTF-8 output"
    - "`git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 17"
  assertions:
    - "git rev-parse HEAD is a NEW commit, subject `chore: touch the readme`"
    - "git -c core.quotePath=true show --name-status --format= HEAD is `M README.md` and `A \"bad\\377name.txt\"`"
    - ".jigc/tasks/touch-the-readme is gone"
control: "the same block with the name badXname.txt exits 0 and prints `finalized <sha> — chore: touch the readme … added badXname.txt`; the same block without the `git config` line exits 0 with the 0xFF name"
pinned-by: "UNPINNED: a verifier writes no test; this block states today's behaviour of a defect, so it is a fixer's red test turned round, not a fact to pin as it reads"
```

Not pinnable as it reads: it asserts the defect. It is the block a fix would start from.

## Left open

1. **`jigc task finalize` exits 1 after its commit landed** (cells `h1-hfs-tmp-hook-ff-pre`,
   `c5-apfs-hookidx-ff-pre`; *Repro VR-PC-2*): no landed summary, no hash, the task's area
   already gone. An agent that reads the exit takes the commit for not made. Not graded
   against `working-product`. What the task's area would lose or keep at that exit when it
   holds a file jigc did not write was not driven.
2. **`jigc milestone finalize` exits 1 after its boundary commit landed, and leaves the
   fan-out worktree registered and standing** while the milestone's area and the sub-task's
   area are gone (cell `mh1-hfs-tmp-hook-ff-pre`). What state the milestone record is left in,
   and which command then removes the worktree, were not driven. `milestone.rs:9619` was not
   reached.
3. **Both doors refuse, under default git configuration, any work that stages a name that is
   not UTF-8** — `git diff --cached` produced non-UTF-8 output at the task door,
   `git diff --name-status` produced non-UTF-8 at the milestone door — where the ASCII control
   lands. On this host such a file cannot exist; on a host that holds the name this is a door
   that does not run. The second clause's question; not graded. The refusal lines carry no
   finding code and no route.
4. **The amend arm writes no `file-state` record when its note fires** (cells
   `a1-hfs-tmp-ff-pre`, `-fin`). The design calls that self-healing and `jigc validate` exits 0
   afterwards; whether the record is rebuilt was not followed further.
5. **A C-quoted name is printed as a path** in the landed summary and the left-out list under
   default quoting (*added "bad\377name.txt"*), and the `file-state` record then holds no hash
   for it (`"hashes": {}` in cell `a1-hfs-tmp-ff-default`). Not pursued.
6. **`jigc milestone finalize`, at exit 0, removed a sub-task worktree holding an untracked
   file** and said so: *discarded with the fan-out worktrees (not committed, not recoverable):
   sub-work: bad%FFname.txt (never staged)* (cells `m3-hfs-tmp-base-ff-pre`, `-default`). Here
   that file is the volume's own second name for a tracked file, so no bytes went that git does
   not hold. Whether a genuinely untracked file in a live sub-task's worktree goes the same
   way, and which ruling owns that, I did not examine: it is not this finding.
7. **The left-out list after landing names one entry where the list before the commit named
   two** (cell `c4-hfs-tmp-basemod-ff-default`: before, `"bad\377name.txt"` and
   `bad%FFname.txt`; after, the first only; `git status` still lists both). Possibly an
   artefact of the volume's second name. Not pursued.
8. **A host that holds the name was not driven**, and neither was the previous release.

## Bounds — what this verification did not do

- One macOS host, one git. No volume here holds the name; the image accepts it and lists it
  escaped. I expect a volume that holds it to give the same pre-commit refusals, because they
  are reads of the index and the scratch-root cells show them with no file at all — that is
  an expectation, not a driven fact.
- The candidate only.
- One name, one byte (0xFF), at the repository's root; one sub-task; no doc created in the
  task; the doc-only finalize arm and the migration arms not driven.
- Hooks: `pre-commit` only, one planted line, placed above jigc's block.
- The image's rigs went with the image. The cells' captured outputs stand under my scratch
  directory, `<scratch>/vp3-capture.qwTN0G/cells/`, with the drivers beside them.

<!-- end of report -->
