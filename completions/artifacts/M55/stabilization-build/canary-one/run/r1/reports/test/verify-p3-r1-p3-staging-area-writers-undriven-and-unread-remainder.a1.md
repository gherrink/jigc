# verify-real — `r1-p3-staging-area-writers-undriven-and-unread-remainder` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p3-r1-p3-staging-area-writers-undriven-and-unread-remainder`. One finding, handed
over: ledger key `r1-p3-staging-area-writers-undriven-and-unread-remainder`, door `jigc doc list`,
the clause it is said to break `working-product`, triage's grade *unclear*. It has no block of its
own: its source is the paragraph *The bound of this enumeration* under *The enumeration*, and items
5 and 6 of *Left open*, in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-staging-area-writers-not-enumerated.a1.md`.
That report was read because the prompt hands it over as the finding; no other report was read, and
nothing of triage's reasoning beyond the grade and the re-drive it asks for.

## Verdict

**`refuted` — basis `does-not-reproduce`.**

The finding is the remainder of a hypothesis: that among the things the source report read and did
not drive, or did not read — `jigc migrate`'s authoring path, a re-opened (amend) task, the spawned
`git` processes — there is a writer that leaves, under ordinary use, a link or an entry its owner
cannot read in a task's staging area (`.jigc/tasks/<id>/docs/`), the state in which
`jigc doc list --task <id>` exits 1 with a raw OS error. Driven and read, it does not hold:

- **`jigc migrate`'s authoring path, driven in ten groups** (umask 022 and 077; a plain source, a
  read-only source, a source sitting at the doctype's home, a link as the source, and the create
  door over a managed committed home that is plain, mode 0444, and a tracked link): every staged
  entry is a regular file its owner can read, after every step, and `jigc doc list --task` exits 0
  over every area that exists.
- **The amend task, driven in three groups** (022, 077, and discard-then-mint-again), and **a
  sub-task re-entered from its own worktree** in two (022, 077): the same.
- **The spawned `git` call sites, each read: 64 in production code, 25 in test modules** (the 89 the
  source report counted and did not read). None of the 64 takes a path under `.jigc/tasks/` as an
  operand, and the nine that write a working tree write tracked content only, into a tree jigc made
  for the purpose or at a doc's committed home. Driven against that reading: the entries of a second,
  open task's area were read — inode, shape, mode, size, mtime, ctime — around every git-spawning
  door driven, 27 times, and were unchanged every time; the one deliberate jigc write into that area
  showed as a change, so the instrument sees a write.
- **The same block on a fresh rig of the previous release gives the same picture**: the same shapes
  and modes in every inventory, and the same exit status in every cell but one group's, which is
  about a planted link at a committed home and not about a staging area (item 2 of *Left open*).

**Two parts of what triage asked for were not driven, and the verdict does not cover them: a root
caller, and Linux.** Neither is available with the binaries this verifier was handed. They are item
1 of *Left open*, with what was read in their place.

`contested: false` — the finding argues against no settled decision.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`. Both drivers put the handed binary's directory first on `PATH` and
  stop (exit 90) unless `command -v jigc` prints the binary they were handed; every run passed it.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  refs-post-hoc`, stdout captured alone, the construction log to its own file, exit 0 each time.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and
  after, `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0, arm64), git 2.54.0 (Apple Git-157), as a non-root user.
Nothing was driven on Linux, and nothing as root** — both handed binaries are Mach-O arm64
executables, and `sudo -n true` answers *a password is required*.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p3.NursTz` (written `<W>`). Under it: an exploration rig (`<W>/explore/`,
candidate only, used to learn the verbs' spellings; one fact is read from it and says so), a first
candidate run (`<W>/c.runs/`, 167 cells, superseded when the driver gained three groups; kept, not
used) with a rig built for the previous release and never driven (`<W>/p.rig/`), and the two pairs
every number here is read from, each binary on a fresh rig of its own:

- `<W>/c2.runs/` (candidate) and `<W>/p2.runs/` (previous release) — driver `<W>/tools/drive.sh`,
  242 cells each (235 `jigc` invocations and 7 plain `git` ones a user would type);
- `<W>/c2w.runs/` and `<W>/p2w.runs/` — driver `<W>/tools/drive2.sh`, 24 cells each, run afterwards
  on the same two rigs: the sub-task doors driven with the sub-task's worktree as the directory.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. Each cell runs in a subshell under the umask the cell names, stdin from
`/dev/null` or a file, stdout and stderr to files of their own made before the umask applies, the
exit status read directly and never through a pipe. An *inventory* is `find <area>/docs -mindepth 1`
handed to `stat -f '%HT|%Sp|%z|%N'`, which reads an entry's shape **without following a link**; an
entry counts as anomalous when it is not a regular file or its owner's read bit is off. A *watch
read* is the same walk over the rig's own live task (`ground-the-vision-in-research`, three
entries), with the inode, mtime and ctime of every entry added, compared byte for byte with the
read taken before anything was driven.

**A deviation, stated.** The two drivers were written from the shell, into `<W>/tools/`; they are
scratch files in no repository. The file tool wrote this report and nothing else.

## 1 · `jigc migrate`'s authoring path

What the path is, as driven: `jigc migrate <path> --as adr` mints a task and provisions its commit
doc; the foreign source is staged at the task's root (`.jigc/tasks/<id>/source`, not under `docs/`);
`jigc doc author adr --from-file … --task <id>` creates the record; `jigc task finalize <id>` holds
(exit 4) and `--approve` lands. Each group below walks: migrate · author · a second author with a
refused leaf (exit 1, `write.malformed-value` — the in-place rollback write) · `doc set-slot` on the
staged adr · `doc list adr --task` (the route the workflow prints) and `doc show --task` ·
`task validate` · `task finalize` (the hold) · `jigc start --task <id>` (the resume) ·
`task finalize --approve`. An inventory follows every writing step; the door — plain and
`--format json` — follows the mint, the author, the hold, the resume and the approve.

| group | umask | the source, and the home the title slugs to | staged entries, at every inventory | `jigc doc list --task …` |
|---|---|---|---|---|
| MA | 022 | a plain tracked file outside the home | 2 after the mint, 3 after the author; regular, `-rw-r--r--` | exit 0 while the area exists |
| MB | 077 | the same shape, another title | 2, then 3; regular, `-rw-------` | exit 0 |
| MC | 022 | a foreign file **at** the adr home, under the slug its title mints | 2, then 3; regular, `-rw-r--r--` | exit 0 |
| MD | 022 | the same, mode 0444 | 2, then 3; regular, **`-rw-r--r--`** — the source's mode is not carried (provenance, read: `created`) | exit 0, also after the approve failed (below) |
| ME | 022 | a tracked link at the adr home | none: `jigc migrate` exits 1, `migrate.source-untrackable`, no task minted, no area | — |
| MF | 022 | a tracked link outside the home | none: the same refusal | — |
| MG | 077 | a plain file, mode 0444 | 2, then 3; regular, `-rw-------` | exit 0 |
| MH | 022 | a plain source, authored under a title a **managed** committed adr already holds — the create door copies the committed doc in | 2, then 3; regular, `-rw-r--r--` | exit 0 |
| MI | 022 | the same, the managed home mode 0444 | 2, then 3; regular, **`-rw-r--r--`** — the home's mode is not carried (provenance, read: `edited-from-base`) | exit 0, also after the approve failed |
| MJ | 022 | the same, the managed home a tracked link to a real tracked file | 2, then 3; **regular**, `-rw-r--r--` — the link is read through and not carried (provenance, read: `edited-from-base`) | exit 0 |

**That the rollback write was reached, in all eight groups that mint:** around the refused author
the staged adr keeps its inode, its shape, its mode and its size, and its mtime moves by one second
(the driver sleeps a second before the cell) — the file was rewritten in place by a command that
exited 1. The printed route `jigc doc list adr --task <id>` exits 0 in all eight.

**The approve.** Exit 0 in MA, MB, MC, MG and MH; the task's area is gone afterwards, and the door
then answers `finalize.no-task`, exit 1 — the task no longer exists. Exit 1 in MD and MI, where the
destination is mode 0444 (item 3 of *Left open*); the area stays, three regular entries, and the
door exits 0 over it. In MJ the candidate refuses with exit 3 at the hold and at the approve (item
2 of *Left open*); the area stays, three regular entries, and the door exits 0 over it.

## 2 · The amend task, and a task that is re-entered

`jigc task amend "<intent>"` mints a task holding one doc — an empty `commit` doc pinned to `HEAD`.

| group | umask | what jigc was asked to do | staged entries, at every inventory | `jigc doc list --task …` |
|---|---|---|---|---|
| AA | 022 | `task amend` · `doc set-field …#type` · `doc set-slot …#summary` · `doc set-slot …#body` · `doc add-item …#trailers` · `doc set-field …/value` · a copy-in asked of a committed doc (`doc set-slot research:context-loss#question`, **exit 1**, `finalize.amend-staged-doc`) · `doc create adr` (**exit 1**, `create.gate-blocked`) · `task validate` · `jigc start --task` (the resume) · `task finalize` (exit 0, `amended <sha> → <sha>`) | 2 (the commit doc and the manifest) at each of the four inventories before the landing; regular, `-rw-r--r--`; the two refused writes stage nothing | exit 0 at each of those four points, plain and json; `finalize.no-task`, exit 1, once the amend has landed |
| AB | 077 | the same, another intent | 2; regular, `-rw-------` | the same |
| AC | 022 | `task amend` · `task discard <id> --force` · `task amend` again under the same intent | 2, then no area, then 2 again; regular, `-rw-r--r--` | exit 0 |

An amend task can hold no doc but its commit doc on either binary: both refusals above are the same
exit on the previous release.

**Re-entry from the sub-task's own worktree** (`<W>/c2w.runs/`; the directory of every cell from
the fourth on is `.jigc/worktrees/<sub-task>`):

| group | umask | what jigc was asked to do | staged entries | `jigc doc list --task …` |
|---|---|---|---|---|
| WA | 022 | `milestone create` · `add-task` · `provision` · then, from the worktree: `jigc workflow sub-task --task <id>` (the re-entry) · `doc author adr` · `doc set-slot research:context-loss#question` (a copy-in) · `doc rename adr:… --to …` (the in-area rename) · the re-entry again | 2, 3, 4, 4, 4; regular, `-rw-r--r--` | exit 0 from the worktree and from the main checkout, plain and json; the two listings byte-identical in each format |
| WB | 077 | the same | 2, 3, 4, 4, 4; regular, `-rw-------` | the same |

All 24 cells exit 0. The worktree has no `.jigc/tasks` of its own: the writes land in the main
checkout's area, which is the one inventoried.

## 3 · The spawned `git` call sites, read as possible writers into a staging area

**Method, and its counts** (all lines at eeffe347). `command grep -rn 'Command::new' crates/cli/src
crates/engine/src` gives 97 lines: the 89 `Command::new("git")` sites the source report counted, 2
comments, 5 sites in test modules (`sh` twice, `mkfifo` three times) and
`crates/cli/src/invoke.rs:218`, the probe's self-exec. Each of the 89 was classified by whether it
sits above its file's `#[cfg(test)] mod`, and each production one read with its arguments:

- **64 production sites, 25 in test modules** — `task.rs` 33 of 41 · `milestone.rs` 14 of 15 ·
  `setup.rs` 6 of 8 · `repo.rs` 4 of 5 · `migrate_corpus.rs` 4 of 4 · `combine.rs` 2 of 3 ·
  `trackable.rs` 1 of 4 · `orphan.rs` 0 of 1 · `relocate.rs` 0 of 4 · `start.rs` 0 of 4 (those
  last files reach git through `task.rs`'s wrappers). The engine crate spawns nothing.
- **Sixteen of the 64 take their arguments from their callers** (`git_run`, `git_capture`,
  `git_index` and `commit_through_seam` of `task.rs`; `git_in`, `git_worktree` and `git_rev_parse`
  of `milestone.rs`; `git_output` and `git_capture_untrimmed` of `setup.rs`; the four of
  `migrate_corpus.rs`; `combine.rs`'s pair; `trackable.rs`'s `git_output`), so reading the sites is
  not enough. The second sweep closes that: every string literal in production code that is one of
  git's own command words (`git --list-cmds=main,others,nohelpers`, 164 words) was listed — 38
  distinct words — and every one that can write a working tree was read at each of its sites.

**What that leaves.** The git commands production code runs that write files outside `.git/`:

| site | the command | where it writes |
|---|---|---|
| `crates/cli/src/task.rs:3428` | `checkout-index -a --prefix=<dir>/` | a `ScratchTree`, which is a fresh directory under the system temp directory (`task.rs:10166`) |
| `crates/cli/src/task.rs:9708`, `crates/cli/src/milestone.rs:3381` | `worktree add --detach <path> <base>` | `.jigc/worktrees/<name>` — the tracked files of `<base>` |
| `crates/cli/src/task.rs:9640`, `:9664` | `read-tree --reset -u <tree>` | run in a dedicated worktree under `.jigc/worktrees/` |
| `crates/cli/src/task.rs:9399` | `apply --index` | run in that dedicated worktree; the patch is a sub-task worktree's own staged diff |
| `crates/cli/src/task.rs:9540` | `merge --ff-only <commit>` | the live checkout — the tracked paths the boundary's commit changes |
| `crates/cli/src/task.rs:7487` | `restore --source=HEAD --worktree -- <destination>` | a doc's committed home, on a finalize's rollback |
| `crates/cli/src/relocate.rs:299` | `mv <old> <new>` | a doc's committed home |

The rest write no working tree: `restore --staged` (four sites), `rm --cached`, `update-index`,
`add -- <paths>`, `read-tree` and `write-tree` and `apply --cached` on a scratch index,
`hash-object`, `commit`, `worktree remove`, and the readers. `stash`, `switch`, `checkout`, `init`
and `submodule` appear only in the text of printed routes (`crates/engine/src/finding.rs:995` to
`:1045`, `crates/cli/src/milestone.rs:8169`, `crates/cli/src/render.rs:2402`) and are never run.

**Why none of them reaches `.jigc/tasks/<id>/docs/`.**

- Git materializes tracked content, and jigc tracks nothing there: `.jigc/.gitignore` — written by
  `jigc setup` from `crates/cli/src/gitignore.rs:38` — ignores `tasks/` whole, and every one of
  jigc's own `git add` calls is `add -- <paths>` with no `-f`, `-A` or `--all` (ten sites, each
  read; the only `--force` literals in production are jigc's own flag in printed routes and
  `worktree remove --force`; the one `--all` is a `rev-list` argument). So no commit jigc builds
  holds a path under a task's area, and no checkout of one writes there.
- A doc's committed home cannot be under `.jigc/`: `jigc config set docs-root
  .jigc/tasks/<id>/docs` exits 1 with `config.workbench-root` (driven once, on the candidate, in the
  exploration rig).
- A worktree's files live under `.jigc/worktrees/`, and a worktree has no task area of its own
  (groups WA and WB above).
- `jigc rename` — the door that runs `git mv` and commits — refuses while any task is open:
  `rename.in-flight`, exit 1 (cell GW.4). With an area in existence that git call is never reached.
- `git commit` runs the installed `pre-commit` hook, which runs `jigc validate --format json` and
  `git diff --cached`; the sweep was driven on its own (GW.1) with the watched area unchanged.

**One way remains, and it is a planted state:** a path under `.jigc/tasks/<id>/docs/` that somebody
force-added to git (`git add -f`) is tracked content, and a checkout would write it. No jigc verb
does that. Read, not driven.

**Driven against the reading — the watch.** With the rig's live task open throughout, its three
entries were read around every git-spawning door:

| the door | what git does in it | watch reads, all *unchanged* |
|---|---|---|
| `jigc task finalize <id> --approve` — eight migrations, five of which land | `add`, `commit` in the live checkout, with the hook | 16 (before and after each) |
| `jigc task finalize <id>` — two amends | `commit --amend`, with the hook | 4 |
| `jigc milestone provision` | `worktree add` twice | 1 |
| `jigc milestone join` | — | 1 |
| `jigc milestone finalize` — a fan-out that **lands** (exit 0: two docs, a copied-in edit and two code files staged in two worktrees, six files committed) | a dedicated worktree, `read-tree --reset -u`, the commit with the hook, `merge --ff-only`, `worktree remove` | 1 |
| `jigc validate` · `jigc setup` (a re-run) · `jigc migrate-corpus` · `jigc rename` (refused) | readers; `setup` re-installs | 4 |

27 reads, 27 unchanged — the same inode, mode, size, mtime and ctime of every entry — on each
binary. The control: `jigc doc set-slot vision:vision#thesis … --task <the watched task>` exits 0
and the next read reports *changed*, a new inode at the same name.

## The totals, and the same block on the previous release

**Candidate, `<W>/c2.runs/`:** 242 cells — 200 exit 0, 33 exit 1, 2 exit 3, 7 exit 4. Every
non-zero one is accounted for: 14 are the door asked about a task that has just landed
(`finalize.no-task`); 8 are the refused author (the rollback cell); 7 exit-4 are the migration's
review hold; 4 are the amend task's two refusals, twice; 2 are `migrate` refusing a link as its
source; 2 are the approve over a mode-0444 home; 2 exit-3 are group MJ; and one each are
`jigc workflow sub-task --task` asked from the main checkout and not the worktree, `jigc validate`
over the two never-adopted files this verifier planted at the adr home, and `jigc rename` with a
task open.

- **Inventories: 78** (77 of one task's area and the closing one over every area). 10 are of an
  area a landing or a discard has removed. In the other 68: **195 entry lines, 153 `Regular
  File|-rw-r--r--` and 42 `Regular File|-rw-------`, and nothing else; 0 anomalous.** The closing
  inventory: 14 entries in 5 areas. `<W>/c2w.runs/` adds 10 inventories, 34 entry lines, 17 and 17,
  0 anomalous.
- **The door: 112 invocations** (56 plain, 56 json): 98 exit 0 — every one over an area that
  exists — and the 14 above. `<W>/c2w.runs/` adds 8, all exit 0.

**Previous release, `<W>/p2.runs/` and `<W>/p2w.runs/`,** the same drivers on a rig built with
`--binary <scratch>/bin/previous-91834b5e011d/jigc`. Not the regression step — the verdict is not
`confirmed`, and no `regression` field is returned — but what the clause's wording compares
against.

- `diff` of the two logs (the rig's path, the inodes and the timestamps normalised) differs in two
  things only. **The size of `provenance.json`** wherever a copy-in happened — the candidate's
  manifest also records what was copied in; the same shape and mode. **Group MJ**: over the tracked
  link this verifier put at a managed home, the previous release holds (exit 4) and its approve
  lands (exit 0), where the candidate refuses twice (exit 3) — so the previous release's area is
  gone at the group's end, its last two door cells answer `finalize.no-task`, and its closing
  inventory holds 11 entries in 4 areas. Exits there: 199 · 35 · 0 · 8.
- Every other cell: the same exit status. Every inventory but MJ's last: the same entries, shapes
  and modes. 27 watch reads unchanged and the control changed, as on the candidate.
- The door's streams: of the 224 of the first driver (112 invocations, stdout and stderr each),
  220 are byte-identical between the binaries, the rig's path normalised; the 4 that differ are
  MJ's last two cells, where the task exists on one binary and has landed on the other. All 16 of
  the second driver are byte-identical.

## Does it break the clause, inside its scope

The clause's instrument, in the closing condition's words (DECISIONS.md, 2026-10-04, *The exit
rule, revised*, sharpening 2): *no command that works on rc.24 in a supported layout stops working,
and every refusal's route works as printed.* The finding needs a state for the door to fail in.
Through the paths triage named and this verifier could drive, that state was not reached on either
binary: the door exits 0 over every area that exists, in 106 invocations on the candidate, and its
streams are the previous release's wherever the two binaries stand in the same state. There is
nothing here for the clause to be broken by.

**Against the design that owns the behaviour.** `design/doc-read-surface.md` → the `--task` arm of
`jigc doc list`: *a staged working copy is jigc-written by construction, never a foreign squatter*.
The migration path, the amend task and the worktree re-entry bear that sentence out for the shape
of what jigc writes, and the reading of the git sites bears out its other half — that nothing jigc
spawns writes there in jigc's place.

## Scope of what was verified

**Enumerated: the process-spawn sites of production code — 65** (64 `git`, and the probe's
self-exec), by the two textual sweeps and the reading described in section 3. The bound is the
source report's own: a spawn reached through a spelling neither sweep matches would be missed —
`Command::new` is the only spelling searched for a spawn, and a git command word assembled at run
time and not written as a literal would escape the second sweep.

**Driven: the instance.** Migration into `adr` only, of the twelve doctypes `jigc migrate` lists;
the dev pack's `amend` workflow; one fan-out that lands and two sub-tasks re-entered from their
worktrees; umask 022 and 077; two binaries; one platform; a non-root caller. For every other
doctype's migration, and for the readers of a staged doc other than `jigc doc list --task`, this
report is `instance, unbounded`.

## Repro M-1

```yaml
claim: "a writer the source report left undriven or unread — jigc migrate's authoring path, an amend task, a spawned git process — leaves, under ordinary use, a link or an owner-unreadable entry in a task's staging area, so that `jigc doc list --task <id>` exits 1"
verdict: REFUTED   # basis does-not-reproduce
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same inventories and the same exit status in every cell outside group MJ; 236 of 240 listing streams byte-identical, the other 4 being MJ's (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2 arm64, git 2.54.0, a non-root caller; NOT driven on Linux, NOT driven as root"
setup:
  - fixture: refs-post-hoc          # its live task is the watched area: ground-the-vision-in-research
  - umask: "022, and the block again under 077 with another source and title"
  - write: "legacy/old-decision.md — a title line and two paragraphs, no front matter"
  - ["git", "add", "legacy/old-decision.md"]
  - ["git", "commit", "-q", "-m", "docs: a legacy decision note"]
repro:
  - ["jigc", "migrate", "legacy/old-decision.md", "--as", "adr"]            # prints `task minted: <task>`
  - ["jigc", "doc", "author", "adr", "--from-file", "<payload: title Use plain files; context, decision, consequences slots>", "--task", "<task>"]
  - ["jigc", "doc", "author", "adr", "--from-file", "<the same payload plus `status: not-a-status`>", "--task", "<task>"]
  - ["jigc", "doc", "set-slot", "adr:use-plain-files#context", "--from-file", "-", "--task", "<task>"]   # stdin: one line of prose
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "start", "--task", "<task>"]
  - ["jigc", "doc", "list", "--task", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "task", "amend", "reword the last commit"]                     # mints reword-the-last-commit
  - ["jigc", "doc", "set-field", "commit:reword-the-last-commit#type", "--value", "docs", "--task", "reword-the-last-commit"]
  - ["jigc", "doc", "set-slot", "commit:reword-the-last-commit#summary", "--from-file", "-", "--task", "reword-the-last-commit"]
  - ["jigc", "doc", "set-slot", "research:context-loss#question", "--from-file", "-", "--task", "reword-the-last-commit"]
  - ["jigc", "doc", "list", "--task", "reword-the-last-commit"]
  - ["jigc", "task", "finalize", "reword-the-last-commit"]
expect:
  - exit: 0
    stdout_contains: "task minted: migrate-adr-legacy-old-decision-"
  - exit: 0
    stdout: "adr:use-plain-files\n"
  - exit: 1
    stderr_contains: "blocking · write.malformed-value"
    tree: "the staged adr:use-plain-files.md keeps its inode, is rewritten in place, and is a regular file with the mode it had"
  - exit: 0
  - exit: 4
    stdout_contains: "migration review required — nothing committed"
  - exit: 0
  - exit: 0
    stdout: "id  path  state\nadr:use-plain-files  docs/decisions/use-plain-files.md  managed\ncommit:<task>  commit:<task>  managed\n"
    stderr: ""
  - exit: 0
    stdout_contains: "promoted docs/decisions/use-plain-files.md"
    tree: "the task's area is gone; the watched task's three entries keep inode, mode, size, mtime and ctime"
  - exit: 0
    stdout_contains: "task minted: reword-the-last-commit"
  - exit: 0
  - exit: 0
  - exit: 1
    stderr_contains: "blocking · finalize.amend-staged-doc"
    tree: "nothing is staged: the area still holds the commit doc and provenance.json only"
  - exit: 0
    stdout: "id  path  state\ncommit:reword-the-last-commit  commit:reword-the-last-commit  managed\n"
    stderr: ""
  - exit: 0
    stdout_contains: "amended "
    tree: "the watched task's three entries keep inode, mode, size, mtime and ctime"
  - tree: "after every step, each entry of the task's .jigc/tasks/<task>/docs/ — read without following a link — is a regular file with its owner's read bit set: mode 0644 under umask 022, 0600 under 077"
variants:   # each in a task of its own; the staged entries are regular and owner-readable, and `doc list --task` exits 0 over every area that exists, in every one
  - "the source a foreign file at the adr home, under the slug its title mints — plain, and mode 0444: the staged adr is `created`, mode 0644"
  - "the source a tracked link, at the adr home or outside it: `jigc migrate` exits 1, `migrate.source-untrackable`; no task, no area"
  - "the source mode 0444, under umask 077: the staged entries are mode 0600"
  - "a title a managed committed adr already holds — the home plain, mode 0444, and a tracked link to a real tracked file: the create door copies in (`edited-from-base`); the staged copy is a regular file, mode 0644, in all three"
  - "`jigc task amend`, `jigc task discard <id> --force`, `jigc task amend` again under the same intent: a fresh area of two regular entries"
  - "a sub-task re-entered from its worktree (`jigc workflow sub-task --task <id>`, directory .jigc/worktrees/<id>), then `doc author adr`, a copy-in and `doc rename` from there — under 022 and 077: 4 regular entries in the MAIN checkout's area, none in the worktree; the listing from the worktree and from the main checkout byte-identical"
  - "a fan-out that lands — `milestone create`, `add-task` twice, `provision`, a doc in each sub-task, a file staged in each worktree, `join`, `milestone finalize` (exit 0): the watched task's entries unchanged after each of the three"
control: "the rig's own live task before anything is done: 3 regular entries, `doc list --task` exit 0; and, for the watch, `jigc doc set-slot vision:vision#thesis --from-file - --task ground-the-vision-in-research` — exit 0, and the next read of the watched area reports a new inode"
observed: "<W>/c2.runs/log with <W>/c2.runs/cells/, and <W>/c2w.runs/; the previous release: <W>/p2.runs/ and <W>/p2w.runs/"
pinned-by: "UNPINNED: found this round. Searched: no file under crates/cli/tests, crates/engine/tests or tooling-tests names a umask (`command grep -rl umask`, no hit); 14 files there read `symlink_metadata` or `.mode()`, none on a line naming `tasks`; `task_amend.rs` and the `migrate_*.rs` suites exist and were not read line by line; no suite was run by this verifier"
```

**Pinnable as it stands: partly.** The fixture is a named state of the shared builder and every
step is an argv, so the umask-022 spine converts by hand to a test whose assertion is the shape and
mode of each entry of the task's `docs/` and the identity of the watched task's entries. Four
conditions: it needs a Unix target (a mode, a link, an inode); the migration's task id carries a
suffix the test must read from the mint's output and not hard-code; the 077 half needs the test to
run the binary under a umask of its own, which an argv alone cannot say; and the comparison with
the previous release is not a suite's to hold — a suite drives one binary.

## Left open

Not pursued; each is for triage like any finding.

1. **A root caller and Linux were asked for and not driven.** Both handed binaries are Mach-O
   arm64; there is no root on this machine without a password. A container image tagged with the
   candidate's commit (`jigc-gate:eeffe347…`) and one of the previous release
   (`jigc-gate:registry-1.0.0-rc.24`) exist on this machine; this verifier's `BINARY:` line hands
   it no trial image, so neither was driven. What was read in their place, and is not evidence of
   the kind a drive is: no production code reads a uid, and the only platform-conditional
   production code is the probe's own image (`crates/cli/src/invoke.rs:114`) and the file-identity
   check of a committed home's rewrite (`crates/engine/src/store.rs:291`) — neither is a writer
   into a staging area. Read and not driven either: a root caller reads past every mode bit, so it
   narrows the state for itself; but an area written **by root under umask 077** holds entries a
   later non-root caller cannot read, which is the state the door fails in — a mixed-caller
   sequence, outside anything this report drove.
2. **Over a tracked link at a managed committed home, the candidate's finalize refuses where the
   previous release's landed** (group MJ; the link is this verifier's plant: the committed adr
   moved aside with `git mv`, a link put in its place, both committed). Candidate: `jigc task
   finalize <id>` and `--approve` both exit 3, `store.home-not-regular-file`, nothing committed,
   the area intact. Previous release: exit 4 (the hold), then exit 0. The refusal prints a route —
   put a regular file at the home, re-run the finalize — and that route was not driven.
3. **The approve over a mode-0444 destination fails with a raw OS error** (groups MD and MI):
   `jigc task finalize <id> --approve` exits 1, `could not promote "<abs>/.jigc/tasks/<id>/docs/…"
   to "<abs>/docs/decisions/….md": Permission denied (os error 13)` — no code, two absolute paths
   of the machine, and a closing sentence that says the task is intact and to re-run the same
   command. Byte-identical on the previous release in MD (the rig's path normalised). The mode is
   this verifier's `chmod`; the re-run was not driven.
4. **The store sweep's `unadopted-instance` route prints the machine's absolute path**: `jigc
   migrate <abs repo>/docs/decisions/<file> --as adr`, seen in `jigc validate` and among the
   advisories of a finalize, on both binaries. One observation; nothing further was driven.
5. **A path under `.jigc/tasks/<id>/docs/` that is tracked in git** (force-added past
   `.jigc/.gitignore`) is the one state in which a git process jigc spawns would write into a
   staging area. Read, not driven; no jigc verb creates it.
6. **Not driven:** migration into any doctype but `adr`; an amend over a commit jigc did not make;
   the M55 doc-only finalize workflows; `jigc relocate` and a `jigc rename` that runs (it refuses
   while a task is open, so no area exists when its `git mv` does).

<!-- end of report -->
