# verify-real — `r1-p4-staging-area-writers-root-and-linux-not-driven` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-staging-area-writers-root-and-linux-not-driven`. One finding, handed
over: ledger key `r1-p4-staging-area-writers-root-and-linux-not-driven`, door `jigc doc list`, the
clause it is said to break `working-product`, triage's grade *unclear*. Its source is item 1 of
*Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-staging-area-writers-undriven-and-unread-remainder.a1.md`.
That report was read because the prompt hands it over as the finding. No other verifier's report
was read, and nothing of triage's reasoning beyond the grade.

## Status: HALTED — no verdict

**This finding is neither confirmed nor refuted. It stays unverified.**

The finding consists of two cells, and neither can be driven with what this verifier was handed:

- **Linux.** Both handed binaries are Mach-O arm64 executables. The `BINARY:` line names the
  candidate and the previous release and no trial image, so there is no Linux binary whose hash
  this verifier could assert.
- **A root caller.** `sudo -n true` answers `sudo: a password is required`, exit 1. The caller of
  every command below is uid 501.

Neither can be reconstructed on this machine either: no state built by a non-root caller on macOS
is a root caller or a Linux kernel. What *could* be reconstructed is one third of the finding — the
reader's half of its mixed-caller sequence — and that was driven, on both binaries (section 2). It
is evidence about that third and nothing more, and it is not offered as a verdict.

The brief was to refute the finding. A refutation was not reached, and not for want of trying: the
part that reproduces on this platform shows no difference between the two binaries, but the two
cells that are the finding's whole content were never reached, and *a finding that could not be
driven is not refuted*. Reading the code in place of a drive was done (section 3) and is labelled
as reading; it also turned up a reason not to lean on it.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the handed binary's directory first on `PATH`, `command -v jigc` printed that binary's
  path. The driver checks it twice — before the rig is built and after the rig's assignments are
  applied — and stops with exit 90 otherwise; both runs passed both checks.
- `file` on each: `Mach-O 64-bit executable arm64`.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  refs-post-hoc`, stdout captured alone, the construction log to a file of its own, exit 0 each
  time.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0, arm64), git 2.54.0 (Apple Git-157), uid 501. Nothing was
driven on Linux, and nothing as root.**

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p1-writers-rootlinux-a2.6CQevR` (written `<W>`). Under it: an exploration rig
(`<W>/explore/`, candidate only, used to read one verb's help; nothing is reported from it), the
driver (`<W>/tools/drive.sh`), and one run per binary, each on a fresh rig of its own:
`<W>/c.run/` (candidate) and `<W>/p.run/` (previous release) — 21 cells each.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. Each cell runs in a subshell under the umask the cell names, stdin from a
file, stdout and stderr to files of their own, the exit status read directly and never through a
pipe. An *inventory* is `stat -f '%HT|%Sp|uid=%u|%z|%N'` over the task's directory, its `docs/`,
and every entry `find <area>/docs -mindepth 1` returns — an entry's shape read without following a
link.

**A deviation, stated.** The driver was written from the shell into `<W>/tools/`; it is a scratch
file in no repository. The file tool wrote this report and nothing else.

## 1 · Re-driven from nothing: what a jigc writer leaves follows the caller's umask, on both binaries

The part of the source report's ground this finding stands on, driven again on fresh rigs.

| cell | umask | command | exit, both binaries | the area afterwards, both binaries |
|---|---|---|---|---|
| control | — | the rig's live task, untouched | — | task dir and `docs/` `drwxr-xr-x`; 3 regular entries, `-rw-r--r--` |
| 03 | 022 | `jigc doc set-slot vision:vision#thesis --from-file - --task <live>` | 0 | 3 regular entries, `-rw-r--r--` |
| 06 | 077 | the same writer, other prose | 0 | the rewritten `vision:vision.md` is `-rw-------`; the two entries it did not write keep `-rw-r--r--` |
| 09 | 077 | `jigc task amend "reword the last commit"` | 0 | a new area |
| 10, 11 | 077 | `jigc doc set-field …#type --value docs` · `jigc doc set-slot …#summary --from-file -` | 0, 0 | task dir `drwx------`, `docs/` `drwx------`, 2 regular entries `-rw-------` |

After each, `jigc doc list --task <task>` and the same with `--format json`: exit 0, all ten
invocations, on each binary. Every entry in every inventory is owned by uid 501.

**What this establishes, and all it establishes:** the mode of what a jigc writer leaves is the
caller's umask applied — to the entries *and to the directories* — and the candidate does what the
previous release does. So a caller under umask 077 leaves an area that only its own uid can enter.
Whether a *root* caller does the same was not driven.

## 2 · The reader's half of the mixed-caller sequence, reconstructed

The source's sentence: *an area written by root under umask 077 holds entries a later non-root
caller cannot read, which is the state the door fails in.* The writer's half needs root. The
reader's half is a caller meeting entries it may not read, and that was built on the area cell 09
to 11 left — **with mode bits standing in for another owner. That substitution is this verifier's,
and it is the whole of what was changed:** the entries are still uid 501's, made unreadable with
`chmod 000`, where the finding's state has them owned by uid 0 with mode 0600 or 0700. To the
reading process both are a refused open; nothing else about a root-owned area is reproduced
(what git makes of a repository with two owners, for one).

| cell | the plant | which caller sequence it stands for | `jigc doc list --task …`, plain and json |
|---|---|---|---|
| R1 | every entry of `docs/` mode 000 | another uid **wrote into** an area the caller owns | exit 1 · stdout empty · stderr `reading the staged doc at "<abs>/.jigc/tasks/reword-the-last-commit/docs/commit:reword-the-last-commit.md": Permission denied (os error 13)` |
| R2 | `docs/` mode 000 | — (between the two) | exit 1 · stdout empty · stderr ``listing the docs staged in task `reword-the-last-commit`: Permission denied (os error 13)`` |
| R3 | the task's directory mode 000 | another uid **minted** the task under umask 077 | exit 1 · `blocking · finalize.no-task` — *a directory carrying no base pin, so it is a leftover and not a work unit*, with a route to keep what is needed and delete the rest by hand |
| restored | modes put back | — | exit 0, the listing of cell 12 byte for byte |

**Candidate against previous release, over all 21 cells:** the same exit status in every cell
(15 exit 0, 6 exit 1 on each). Of the 42 streams, 41 are byte-identical with the rig's path
normalised; the one that differs is cell 09's stdout, in the abbreviated commit sha the amend
names — two rigs, two histories. The two logs differ in one thing: the size of the live task's
`provenance.json` (305 bytes on the candidate, 115 on the previous release; the same shape and
mode).

**What this establishes:** where the caller cannot read a staged area, the door fails, and it
fails the same way on the candidate as on 1.0.0-rc.24. **What it does not:** that a root caller
produces that area; that either binary behaves so on Linux; that the mixed-caller layout is, or is
not, a supported one — the closing condition's second clause says *a supported layout* and no
document read here says which side of it two callers of one checkout fall on.

## 3 · Read, not driven — and why the reading is not leaned on

All at the two commits, with `git grep` over `crates/cli/src` and `crates/engine/src`.

- **No production code reads a uid**, on either commit: `getuid|geteuid|\.uid\(\)|\.gid\(\)|is_root|SUDO_`
  matches nothing at eeffe347 and nothing at 91834b5e.
- **Platform-conditional production code at the candidate: two sites** — `own_image`
  (`crates/cli/src/invoke.rs:114` and `:120`, the probe's self-exec; the same on the previous
  release) and `same_file` (`crates/engine/src/store.rs:291` and `:299`, the identity check of a
  committed home's rewrite; **new in the candidate**). Every other `cfg(unix)` line sits below its
  file's `#[cfg(test)]`. Neither site writes into a task's staging area.
- **The candidate holds OS-level file code the previous release does not:** `open_no_follow`
  (`crates/cli/src/regular_file.rs:182`, `O_NOFOLLOW | O_NONBLOCK` through `libc`) and
  `copy_regular` (`crates/cli/src/task.rs:5811`, which sets the destination's permission bits from
  the source's), where the previous release called `std::fs::copy` (`task.rs:5132` there). Their
  one production caller is the promote out of a staging area to a committed home
  (`task.rs:5777`), not a write into one.

**Why this is not a refutation.** It is a text search, bounded by the spellings searched for. And
it shows the opposite of what would make reading sufficient: the delta between the two binaries
*does* contain code whose behaviour is the kernel's to decide — open flags, permission bits, file
identity — beside the standard library's own per-platform file routines, which no search of this
repository reads. That it sits one step outside the staging area's writers is a statement about
where this verifier looked. On Linux it is a question for a drive.

## Does it break the clause, inside its scope

**Not answered.** The clause's instrument, in the closing condition's words (DECISIONS.md,
2026-10-04, *The exit rule, revised*, sharpening 2): *no command that works on rc.24 in a supported
layout stops working, and every refusal's route works as printed.* To say that the finding breaks
it, or that it does not, needs the door observed on both binaries under the finding's conditions.
Under the one condition that could be reconstructed the two binaries agree (section 2). Under the
two that could not — Linux, which the project's CI runs on and is therefore inside any reading of
*supported*, and a root caller — nothing was observed.

`contested`: not applicable to a halt; the finding argues against no settled decision. The design
that owns the door, `design/doc-read-surface.md` → the `--task` arm of `jigc doc list`: *a staged
working copy is jigc-written by construction, never a foreign squatter* — which says who writes
the area and is silent on who may read it.

## Scope of what was driven

`instance, unbounded`. Three writers (`doc set-slot`, `doc set-field`, `task amend`), two umasks,
two binaries, one platform, one uid, and three planted states. No consumer of the staging area was
enumerated by this verifier.

## Repro H-1 — what was driven; the test-in-waiting for the part that runs here

```yaml
claim: "on Linux, or with root as the caller, a jigc writer leaves in a task's staging area an entry its later reader cannot read, so that `jigc doc list --task <id>` exits 1 where it exits 0 on the previous release"
verdict: NONE   # halted — the two conditions of the claim were not reached; what follows is the part that runs on macOS as a non-root caller
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status in all 21 cells; 41 of 42 streams byte-identical, the other differing in a commit sha (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2 arm64, git 2.54.0, uid 501; NOT driven on Linux, NOT driven as root"
setup:
  - fixture: refs-post-hoc          # its live task is ground-the-vision-in-research
  - env: {GIT_CONFIG_GLOBAL: /dev/null, GIT_CONFIG_NOSYSTEM: "1"}
repro:
  - ["jigc", "doc", "set-slot", "vision:vision#thesis", "--from-file", "-", "--task", "ground-the-vision-in-research"]   # umask 077; stdin: one line of prose
  - ["jigc", "task", "amend", "reword the last commit"]                                                                    # umask 077
  - ["jigc", "doc", "set-field", "commit:reword-the-last-commit#type", "--value", "docs", "--task", "reword-the-last-commit"]   # umask 077
  - ["jigc", "doc", "set-slot", "commit:reword-the-last-commit#summary", "--from-file", "-", "--task", "reword-the-last-commit"]  # umask 077
  - ["jigc", "doc", "list", "--task", "reword-the-last-commit"]
  - ["chmod", "000", ".jigc/tasks/reword-the-last-commit/docs/commit:reword-the-last-commit.md", ".jigc/tasks/reword-the-last-commit/docs/provenance.json"]
  - ["jigc", "doc", "list", "--task", "reword-the-last-commit"]
  - ["chmod", "600", ".jigc/tasks/reword-the-last-commit/docs/commit:reword-the-last-commit.md", ".jigc/tasks/reword-the-last-commit/docs/provenance.json"]
  - ["chmod", "000", ".jigc/tasks/reword-the-last-commit"]
  - ["jigc", "doc", "list", "--task", "reword-the-last-commit"]
  - ["chmod", "700", ".jigc/tasks/reword-the-last-commit"]
  - ["jigc", "doc", "list", "--task", "reword-the-last-commit"]
expect:
  - exit: 0
    tree: "the rewritten .jigc/tasks/ground-the-vision-in-research/docs/vision:vision.md is a regular file, mode 0600; the two entries beside it keep mode 0644"
  - exit: 0
    stdout_contains: "task minted: reword-the-last-commit"
  - exit: 0
  - exit: 0
    tree: ".jigc/tasks/reword-the-last-commit and its docs/ are mode 0700; docs/ holds two regular files, mode 0600"
  - exit: 0
    stdout: "id  path  state\ncommit:reword-the-last-commit  commit:reword-the-last-commit  managed\n"
    stderr: ""
  - exit: 0
  - exit: 1
    stdout: ""
    stderr_contains: "Permission denied (os error 13)"
  - exit: 0
  - exit: 0
  - exit: 1
    stderr_contains: "blocking · finalize.no-task"
  - exit: 0
  - exit: 0
    stdout: "id  path  state\ncommit:reword-the-last-commit  commit:reword-the-last-commit  managed\n"
    stderr: ""
variants:
  - "`chmod 000 .jigc/tasks/reword-the-last-commit/docs`: exit 1, stderr ``listing the docs staged in task `reword-the-last-commit`: Permission denied (os error 13)``"
  - "the first writer under umask 022: the rewritten entry is mode 0644"
not-driven:
  - "the block with uid 0 as the caller of steps 1 to 4 and another uid as the caller of the listing — no chmod then: the modes are the writer's own"
  - "the block on Linux, by one caller and by two"
control: "the rig's live task before anything is done: 3 regular entries, mode 0644, `jigc doc list --task` exit 0; and the last step — the modes put back, the listing of step 5 again"
observed: "<W>/c.run/log with <W>/c.run/cells/; the previous release: <W>/p.run/"
pinned-by: "UNPINNED: no suite was searched or run by this verifier"
```

**Pinnable as it stands: no.** As a test of *this finding* it pins nothing: the claim's two
conditions are in `not-driven`. As a pin of what was observed it would hold three raw refusals in
place — two OS errors with an absolute path and no code, and a `finalize.no-task` that calls a live
task a leftover — which is the behaviour items 1 and 2 of *Left open* put to triage, not a contract
to freeze. The umask-077 half also needs the test to run the binary under a umask of its own, which
an argv cannot say.

## The halt

- **Root cause.** The finding's two conditions — a Linux kernel, a root caller — are outside what
  this verifier was handed and outside what a non-root caller on macOS can reconstruct. Both
  binaries of the `BINARY:` line are Mach-O arm64; the line hands no trial image; `sudo` wants a
  password.
- **Evidence.** `file <scratch>/bin/c1.a2/jigc <scratch>/bin/previous-91834b5e011d/jigc` →
  `Mach-O 64-bit executable arm64`, twice. `sudo -n true` → `sudo: a password is required`, exit 1.
  `id -u` → `501`. `uname -sm` → `Darwin arm64`. The `BINARY:` line of the prompt: a candidate and
  a previous release, each a path and a sha256, and nothing else.
- **Tree state.** The clone is on `fix/canary-one` at 126a85311a54e73e9f9798f3d034d24f14e3ea7c,
  the round's record commit; its parent is the candidate's commit,
  eeffe347324f83a51d1ae83d5f254e73c3f1ea3a. `git status --porcelain`, when last read before this
  report was handed over: thirteen lines, every one an untracked file under
  `completions/artifacts/canary-one/r1/reports/test/`, none of them this verifier's. Nothing was
  edited, staged or committed by this verifier; no commit of its own landed.
- **Recommendation.** Three ways out, the human's to choose between:
  1. **Hand a verifier something that runs on Linux and has root.** Images tagged with the
     candidate's commit (`jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`) and with the previous
     release (`jigc-gate:registry-1.0.0-rc.24`) are listed by `docker image ls` on this machine;
     a container's default caller is uid 0, so one image pair would reach both conditions and the
     two-caller sequence. They were not driven here: the prompt names no image and gives no digest
     to assert, and a binary this verifier was not handed is not the candidate's evidence. The
     block above is ready to run in them as it stands, less the `chmod` lines.
  2. **Rule the mixed-caller layout** — root and another uid working one checkout — a declared
     bound, with its reach. Section 2 is what that ruling would stand on: the door refuses there,
     and refuses identically on 1.0.0-rc.24. That would settle one third of the finding and leave
     Linux.
  3. **Stop minting this row.** This finding is the second generation of *not driven on Linux, not
     driven as root* — each verifier that cannot reach those two conditions leaves the gap open,
     the gap becomes a row, and the row goes to a verifier that cannot reach them either. Until a
     verifier's `BINARY:` line carries an image, that class of row cannot be closed by this role.

## Left open

Not pursued; each is for triage like any finding.

1. **Over a task directory the caller cannot enter, `jigc doc list --task <id>` says the task is a
   leftover and routes to deleting it by hand** (cell R3; the plant is this verifier's `chmod 000`
   on `.jigc/tasks/<id>`). Exit 1, `blocking · finalize.no-task`: *a directory carrying no base
   pin, so it is a leftover and not a work unit — either jigc never minted a task there, or a
   teardown stopped partway*; route: *Keep anything you need from `.jigc/tasks/<id>` and delete the
   rest by hand*. The directory is a live task with its base pin in it; jigc could not read it. The
   same text on the previous release. Whether the route's advice would be followed to a loss was
   not driven.
2. **Over staged entries, or a `docs/` directory, the caller cannot read, the door exits 1 with a
   raw OS error** — no finding code, no route, and for an entry the machine's absolute path (cells
   R1 and R2). The same bytes on the previous release. The ledger's census holds a key that reads
   as this subject (`r1-doc-list-staged-arm-unreadable-entry`); its row and report were not read,
   so whether these two cells are inside it is triage's to say.
3. **The writer's half as root, and everything on Linux** — this finding itself, still open.

<!-- end of report -->
