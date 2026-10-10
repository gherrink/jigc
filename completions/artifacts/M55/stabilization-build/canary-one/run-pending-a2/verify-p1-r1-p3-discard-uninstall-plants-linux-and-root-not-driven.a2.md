# verify-real — `r1-p3-discard-uninstall-plants-linux-and-root-not-driven` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p3-discard-uninstall-plants-linux-and-root-not-driven`. One finding, handed over:
ledger key `r1-p3-discard-uninstall-plants-linux-and-root-not-driven`, door `jigc task discard` and
`jigc uninstall`, the clause it is said to break `no-lost-files`, triage's grade *unclear*. It has no
repro block of its own: it is item 6 under `Left open` of
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-staged-doc-ids-other-callers-not-driven.a1.md`
(*Linux, and a root caller, were not driven*), and triage asks for the plants of that report's
`Repro V-2` to be driven at both doors on Linux and as root. That one report was read, as the
finding's source; no other report, no other verifier's work, and nothing of triage's reasoning
beyond the grade and the line above.

## Status: HALTED — no verdict

**The finding is still unverified. It is neither confirmed nor refuted.** What it asks for — a
Linux drive and a root drive — could not be made from what this verifier was handed, and a verdict
reached without that drive would be a guess.

- **Root cause.** The `BINARY:` line hands two binaries and no image. Both binaries are macOS
  executables, so neither runs on Linux; and this machine gives a delegated run no root caller. The
  two platforms the finding names are exactly the two this verifier cannot reach. Triage's own line
  says a binary or an image has to be handed over for this drive; none was.
- **What was done instead, and what it is worth.** The block was re-driven from nothing on the
  platform that is reachable — macOS, a non-root caller — to establish that it runs as written on
  the candidate, and to leave the expectations a Linux or root drive can be held against. Ten cells,
  ten fresh rigs; every one agrees with the source report. **That is a baseline, not an answer**: it
  is the same platform the source report drove, and it says nothing about Linux or about root.

## The evidence for the halt

Each command run bare, its exit status read directly.

| question | command | exit | output |
|---|---|---|---|
| what the candidate is | `file <scratch>/bin/c1.a2/jigc` | 0 | `Mach-O 64-bit executable arm64` |
| what the previous release is | `file <scratch>/bin/previous-91834b5e011d/jigc` | 0 | `Mach-O 64-bit executable arm64` |
| the platform | `uname -s -r -m` | 0 | `Darwin 25.6.0 arm64` (macOS 26.6.2, git 2.54.0 Apple Git-157) |
| the caller | `id -u` | 0 | `501` |
| is a root caller reachable | `sudo -n true` | **1** | `sudo: a password is required` |

The `BINARY:` line of this verifier's prompt, in full as to what it hands over: the candidate at
`<scratch>/bin/c1.a2/jigc` (commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, label c1) and the
previous release at `<scratch>/bin/previous-91834b5e011d/jigc` (1.0.0-rc.24). No Linux binary, no
trial image, no image name and no image hash.

**Why nothing was substituted.** A container runtime is installed on this machine, and its local
image list holds one tagged `jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` and one tagged
`jigc-gate:registry-1.0.0-rc.24`. Neither was driven and neither was opened. The contract is that a
verifier drives the binary it is handed, under a hash the harness gave it; a binary inside an image
this verifier found by itself carries no such hash, and using it would be a workaround of exactly
the check a halt exists for. Building a Linux binary is likewise not this role's: no `cargo build`
was run and nothing under `target/` was driven. The two image tags are named here only so that
whoever hands one over knows they exist; that either is the candidate's tree is **not** something
this report establishes.

**Why the scope argument was not used to reach a verdict.** The first clause's scope (DECISIONS.md,
2026-10-04, *The exit rule, revised*, sharpening 1) names *deliberately planted states* as declared
bounds, and both plants are made by hand. That might have carried a `breaks-no-clause` without a
drive. It was not taken, for two reasons: the run's opening record says *No bound is declared*, and
that entry itself records that the declared bounds *are not yet a list with reach*; and a Linux
host, and a root caller of the kind a container gives by default, are ordinary use, so what the two
doors do there is a fact to drive, not one to argue. The source report made the same choice: its
verdict *does not rest on whether ordinary use can reach the plants*.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives.
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's. It was asserted and **not driven**: the regression fact belongs to a `confirmed`
  verdict, and there is none.
- The candidate's directory went first on `PATH` in every driver call, and each call stops unless
  `command -v jigc` prints `<scratch>/bin/c1.a2/jigc`. It did, every time.
- Every rig: `SCRATCH=<W>/rigs dev/jigc-rig --binary <scratch>/bin/c1.a2/jigc refs-post-hoc`, stdout
  captured alone, the construction log to its own file, exit status 0 each time.

## The baseline — macOS, a non-root caller, the candidate only

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p1-linux-root-a2.Plr98I` (written `<W>`). One fresh rig per cell, ten cells, ten
rigs under `<W>/rigs/`; nothing was torn down. Environment of every invocation: `HOME` the rig's
own, `GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, the working directory the rig's
repository, stdin from `/dev/null`, stdout and stderr each to its own file. The measurement
(`<W>/tools/cell.sh`, `<W>/tools/snap.py`) is taken before and after the one invocation: one line
per entry under `.jigc/` — kind, mode, size and the first 16 hex digits of its sha256, or the link's
target, or `UNREADABLE` — then `git status --porcelain --untracked-files=all --ignored`, then `HEAD`.

Task `ground-the-vision-in-research` (written `T`). The plants, each alone, in `.jigc/tasks/T/docs/`:

- **dangling** — `ln -s /nonexistent/x.md research:dangling.md`
- **locked** — a 30-byte regular file `research:locked.md`, then `chmod 000`
- **readable** (a control of this verifier's) — the same 30-byte file, left at mode 644

### The doors without the consent

| cell | plant | invocation | exit | stdout | stderr | `.jigc/`, porcelain, `HEAD` |
|---|---|---|---|---|---|---|
| cand-dangling-discard | dangling | `jigc task discard T` | **1** | empty | `blocking · task-discard.foreign-bytes`, naming `.jigc/tasks/T/docs/research:dangling.md`, with a route (695 bytes) | identical, the link standing |
| cand-dangling-uninstall | dangling | `jigc uninstall` | **1** | empty | `blocking · uninstall.foreign-bytes`, naming the same path, with a route (615 bytes) | identical |
| cand-locked-discard | locked | `jigc task discard T` | **1** | empty | `blocking · task-discard.staged-prose` — *stages 3 doc(s)*: `commit:T, research:locked, vision:vision`, with a route (604 bytes) | identical, the file standing at mode 000 |
| cand-locked-uninstall | locked | `jigc uninstall` | **1** | empty | `blocking · uninstall.staged-prose` — 3 staged docs for 1 open task, `research:locked` among them, with a route (696 bytes) | identical |
| cand-readable-discard | readable | `jigc task discard T` | **1** | empty | byte-identical to cand-locked-discard (`cmp`, exit 0) | identical |
| cand-readable-uninstall | readable | `jigc uninstall` | **1** | empty | byte-identical to cand-locked-uninstall (`cmp`, exit 0) | identical |

Six of six fail closed and change nothing. The byte counts are the source report's, cell for cell.

### The doors with the consent

| cell | plant | invocation | exit | stdout | stderr | `.jigc/` after |
|---|---|---|---|---|---|---|
| cand-dangling-discard-force-json | dangling | `jigc task discard T --force --format json` | 0 | `"commit": null, "dropped": [], "findings": [], "op": "task-discard", "task": "T"` (123 bytes) | a warning naming the link as work not in git (270 bytes) | 24 entries before, 13 after: the task area gone, nothing else |
| cand-dangling-uninstall-force | dangling | `jigc uninstall --force` | 0 | the removal ledger, seven lines (417 bytes) | the working-area warning naming the link; no staged-docs warning; *Anything you had staged in one is named above* | absent |
| cand-locked-discard-force | locked | `jigc task discard T --force` | 0 | `discarded task T — dropped staged edits to: commit:T (transient), research:locked, vision:vision` (155 bytes) | empty | 24 before, 13 after |
| cand-locked-uninstall-force | locked | `jigc uninstall --force` | 0 | the same ledger (417 bytes) | *discards the staged docs of 1 open task(s)*: `T: commit:T, research:locked, vision:vision`; then the tracked files, jigc's own state, the work unit | absent |

`HEAD` is unchanged in all ten cells; no commit is made. The forced `uninstall` stderr is three
bytes longer here than in the source report in both cells: it prints the rig's absolute path once,
and this verifier's directory name is three characters longer than the source's.

### What the baseline does and does not say about the two platforms

- **The readable control is the nearest thing to a root caller that could be driven, and it is not
  one.** What separates root from uid 501 at the mode-000 plant is that root can read the bytes. A
  mode-644 file of the same name, read by uid 501, gets the same answer at both bare doors, byte
  for byte, as the mode-000 file. So on this platform neither door's answer depends on whether the
  planted file can be read — which is consistent with the source report's expectation that the
  variant *holds for root as well*. It is evidence about one difference between the two callers,
  and no other: who owns the repository, what git says of a repository whose owner is not the
  caller, and anything else a root caller changes were not reached.
- **Nothing here bears on Linux.** The file system's treatment of `:` in a name, of a dangling link,
  and of case are all platform facts; the cells above measured them on one platform.

## Tree state

- The clone: branch `fix/canary-one`, `HEAD` 126a85311a54e73e9f9798f3d034d24f14e3ea7c. The
  candidate's commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a is its parent line; the commit above it
  is the round's record commit, not this verifier's.
- `git status --porcelain` before and after this verifier's work: three untracked files under
  `completions/artifacts/canary-one/r1/reports/test/` (`attempt.a2.md`, `preflight.a2.md`,
  `triage-p1.a2.md`), none of them this verifier's. Nothing was edited, staged or committed.
- Everything this verifier made is under `<W>` and in this report's own file.
- The run's state reads this finding as `unverified 1` before this attempt. This is the second
  attempt, and it leaves the finding without a verdict a second time.

## Recommendation

1. **Hand a verifier a Linux build of the candidate with its hash, or the trial image with its
   identity, on the `BINARY:` line** — and the previous release's Linux counterpart beside it, since
   a `confirmed` verdict owes the regression fact. Inside a container a root caller is the default,
   so one image answers both halves of the finding: drive the block below once as root and once as
   an unprivileged user of the same container.
2. **Or put it to the human as what it is — still unverified.** This is the second attempt without
   a verdict; the choice between granting one more attempt with an image handed over, and ruling the
   row as it stands, is the human's.
3. Not this verifier's to do, and not done: rebuild, or drive an image found on the machine.

## Repro V-3 — the block in waiting, undriven on the two platforms it is for

```yaml
claim: "on Linux, and for a root caller, with a dangling link or a mode-000 file named as a staged doc alone in a live task's staging area, `jigc task discard <id>` and `jigc uninstall` destroy the task area or the plant at exit 0"
verdict: NONE   # halted: no Linux binary or image was handed over, and no root caller is reachable
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc   # a macOS arm64 executable
driven-on: "macOS 26.6.2 (Darwin 25.6.0, arm64), git 2.54.0, uid 501 — the baseline only"
not-driven-on: ["Linux, any caller", "root, any platform"]
setup:
  - fixture: refs-post-hoc                  # live task ground-the-vision-in-research staging commit:<id> and vision:vision
  - ["ln", "-s", "/nonexistent/x.md", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md"]
repro:                                      # each from a fresh fixture: a door that succeeds destroys its subject
  - ["jigc", "task", "discard", "ground-the-vision-in-research"]
  - ["jigc", "uninstall"]
expect:                                     # the baseline's answers, as the hypothesis for Linux and for root
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · task-discard.foreign-bytes", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md", "jigc task discard ground-the-vision-in-research --force"]
    tree: "every entry under .jigc/ identical before and after (kind, mode, size, link target, content); porcelain and HEAD unchanged"
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · uninstall.foreign-bytes", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md", "jigc uninstall --force"]
    tree: "the same"
variants:        # each alone, in place of the `ln`
  - "a 30-byte research:locked.md, then chmod 000 -> bare doors: exit 1, `task-discard.staged-prose` / `uninstall.staged-prose` naming three ids, `research:locked` among them; tree unchanged. AS ROOT the file is readable: the baseline's mode-644 control gives the same bytes on stderr, which is the hypothesis, not a root drive"
  - "the forced forms (`--force`) -> exit 0 and the destruction the consent names; under the link the task door's JSON ack carries `dropped: []` and the forced uninstall prints no staged-docs warning — the source report's open row, seen again here"
confirmed-if: "on Linux or as root, a bare door exits 0, or any entry under .jigc/ differs after an exit-1 refusal"
refuted-if: "all four bare cells (two plants, two doors) exit 1 with the tree unchanged, on Linux as an unprivileged user AND as root"
observed: "<W>/runs/cand-*/ (argv, stdout, stderr, exit, snap.before, snap.after, rig.env, rig.log)"
pinned-by: "UNPINNED: halted before a verdict. No suite was run by this verifier, and no coverage claim is made."
```

**Pinnable as it stands: no.** It carries no verdict, so there is nothing yet to pin. Once driven on
Linux its two bare-door expectations convert by hand as the source report's do — a named state of
the shared builder, one filesystem call, one argv — and CI's Linux legs would then hold the Linux
half without further work. The root half is not pinnable in the suites as they run: a test cannot
assume a root caller, and the mode-000 variant means something different for one.

## Scope of what was verified

**Instance, unbounded — and for the finding itself, nothing.** Driven: two doors, bare and forced,
two plants and one control, on the candidate, on one platform, as one unprivileged caller. No
consumer of the mechanism was enumerated by this verifier; the source report's count of call sites
is its own and was not re-derived.

## Left open

Not pursued; each is for triage like any finding.

1. **This finding.** Linux and a root caller are still not driven at either door.
2. **One refusal of the four lacks the closing line the other three carry.** `task-discard.staged-prose`
   ends at its route; `task-discard.foreign-bytes`, `uninstall.foreign-bytes` and
   `uninstall.staged-prose` each end with `— jigc · run \`jigc start\` for orientation; all writes
   through \`jigc\`.` Seen in cand-locked-discard against the three others; not compared with the
   previous release, and not checked against the design that owns the footer.
3. **Seen again, already a row of the source report (its `Left open`, item 3), not a new one:** under
   the dangling link the forced task door's ack names no staged doc (`"dropped": []`) and the forced
   `uninstall` prints no staged-docs warning beside *Anything you had staged in one is named above*.

<!-- end of report -->
