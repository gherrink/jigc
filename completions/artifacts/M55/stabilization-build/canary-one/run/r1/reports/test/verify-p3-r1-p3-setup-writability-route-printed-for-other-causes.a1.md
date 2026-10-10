# verify-real — `r1-p3-setup-writability-route-printed-for-other-causes` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc setup`. Clause it is said to break:
`working-product`. Triage's grade: *breaks*. Its block: `Repro VR-2 - the writability route
where the hooks directory is already writable`, in the report of
`verify-p2-r1-setup-install-hook-route-other-causes-unexamined`, attempt 1. Triage asked for
that block and its variants on fresh `bare` rigs with both binaries, and for one cause its
reporter graded neither way.

## Verdict in one paragraph

**`confirmed`, `regression: false`, `contested: false`.** I was sent to refute it and could
not. On a fresh rig per cause and per binary, four causes draw
*route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`* while the
path git names as the hooks directory **is already writable before the first run**:
`core.hooksPath=/dev/null`, `core.hooksPath` naming a regular file, a read-only hook file in a
writable hooks directory, and a hook holding one byte that is not UTF-8. In each, the re-run
exits 1 with stderr byte-identical to the first run, and `jigc setup` exits 0 only after an act
the route does not name. The previous release does the same, byte for byte, so nothing
regressed. **The fifth cause is not a member:** where `core.hooksPath` names a directory that
does not exist under a read-only parent, the route's condition is false at the refusal, and the
re-run exits 0 once the condition holds. A control and a positive control stand beside them: an
untouched rig exits 0, and a hooks directory of mode 555 is refused with the same route, which
then works as printed.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Both were the plain call the prompt spells, made before the first command was driven. The
driver puts the binary's directory first on `PATH` and stops unless `command -v jigc` prints
that binary's absolute path; every transcript's first line carries what it printed. Nothing was
built and nothing under `target/` was driven.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-p3.wervc8`. **One fresh rig per cause and per binary — eight for the
  candidate, eight for the previous release** — each built by
  `dev/jigc-rig bare --binary <path>` with `SCRATCH=<W>`, stdout captured alone and evaluated in
  a second step. None is the reporter's. A ninth candidate rig (`uD18tv`) was built to read the
  rig's output and never driven.
- Environment of every run: `HOME` the rig's own empty directory, `GIT_CONFIG_NOSYSTEM=1`,
  `GIT_CONFIG_GLOBAL=/dev/null`, the identity the rig sets in the repository's own config. Git
  2.54.0 (Apple Git-157), macOS, an unprivileged user (uid 501), so a mode of 555 binds.
- Every exit status was read bare: `jigc setup > out 2> err`, then `$?`.
- **The candidate was called as the route prints it, a bare `jigc` resolved through `PATH`. The
  previous release was called by its absolute path.**
- **The route's condition** was measured, not assumed: before the first run, after it, and
  after the route's act, the driver asks git for the hooks path
  (`git rev-parse --path-format=absolute --git-path hooks`, the call the binary itself makes)
  and records whether that path exists, is a directory, and is writable.
- **"The route as printed"** was taken to mean: `chmod u+w` on that path, then `jigc setup`
  again. The positive control shows that this is the act the route means — there it turns exit 1
  into exit 0.
- Where the route did not end at exit 0 I then did what clears the cause, to see whether the
  door recovers at all. Those steps are marked *not the route*.
- The driver is `<W>/drive.sh`; its transcripts are `<W>/cand.part1.txt`, `<W>/cand.part2.txt`
  and `<W>/prev.txt`; each rig holds `ev/setup<n>.out`, `ev/setup<n>.err` and
  `ev/setup<n>.porcelain`.

**What I changed against the block as written: nothing in its steps.** The block's `setup`,
`repro` and `expect` ran as they stand. I added the measurement of the route's condition at
three points, the two controls, and two forms of the fifth cause.

## What was observed

### The controls

| cause | candidate | previous release |
|---|---|---|
| untouched `bare` rig | exit 0, stderr empty, one install commit, porcelain empty (`Er7Q5e`) | the same (`GVoA3p`) |
| the default hooks directory at mode 555 | hooks path not writable; exit 1, the route below with *Permission denied (os error 13)*; `chmod u+w` on it; re-run **exit 0**, one install commit, porcelain empty (`sNCgte`) | the same (`yMjfZI`) |

### The four causes of the block — the condition holds, and the re-run fails

In every row the hooks path git names is **writable before run 1**, after run 1 and after the
route's act. Run 1 and run 2 are each exit 1 with stdout empty, and `cmp` of their stderr exits
0. The candidate's and the previous release's stderr are byte-identical too, run 1 against run 1
and run 2 against run 2 (`cmp` exit 0, eight comparisons).

| cause | the hooks path git names | run 1 | the route as printed | run 2 | what clears it — *not the route* | run 3 |
|---|---|---|---|---|---|---|
| `git config core.hooksPath /dev/null` | `/dev/null`: exists, no directory, writable | exit 1, *File exists (os error 17)* | `chmod u+w` exit 0; nothing changed | **exit 1**, the same bytes | `git config --unset core.hooksPath` | exit 0 |
| `core.hooksPath` = a regular file outside the repository, mode 644 | that file: exists, no directory, writable | exit 1, *File exists (os error 17)* | `chmod u+w` exit 0; nothing changed | **exit 1**, the same bytes | `git config --unset core.hooksPath` | exit 0 |
| a 29-byte `pre-commit` of the user's at mode 555, the hooks directory at mode 755 | `.git/hooks`: a directory, writable | exit 1, *Permission denied (os error 13)* | `chmod u+w` on the directory exit 0; nothing changed | **exit 1**, the same bytes; the hook byte-identical | `chmod u+w` on the hook **file** | exit 0, the user's line kept |
| a 31-byte `pre-commit` of the user's holding one byte `0xFF`, mode 755 | `.git/hooks`: a directory, writable | exit 1, *stream did not contain valid UTF-8* | `chmod u+w` on the directory exit 0; nothing changed | **exit 1**, the same bytes; the hook byte-identical | rewrite the hook as UTF-8 | exit 0, the user's line kept |

Rigs, each `<W>/jigc-rig-bare-<suffix>`: candidate `YW36Td`, `nF5Gi8`, `t2Be8o`, `D4JuQu`;
previous release `hcILls`, `jRCxwt`, `B7p04K`, `bYP6vv`.

stderr of the first cause, run 1 and run 2, both binaries:

```text
blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: File exists (os error 17)
  route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

In every exit-1 row the first run leaves the install's files written and uncommitted — nine
porcelain lines on the candidate, eight on the previous release — and the run that finally
exits 0 commits them: one install commit, porcelain empty. That a failed first run is completed
by a plain re-run is the design's (`design/project-setup.md`, *It holds after a failed run
too*), and is not this finding's subject.

### The fifth cause — `core.hooksPath` naming a missing directory under a read-only parent

Setup: `<rig>/outside/ro-parent` at mode 555, `core.hooksPath` its child `hooks`, which does not
exist. Driven in two forms, each on its own rig, both binaries, the same result.

| form | run 1 | what was done | the re-run |
|---|---|---|---|
| A (`DQ1g5t`, `jzoi3A`) | hooks path does **not exist**; exit 1, *Permission denied (os error 13)*, the same route | the route read literally: `chmod u+w` on the hooks path exits 1, *No such file or directory*; `mkdir` of it exits 1, *Permission denied*; the condition is still false | exit 1, the same bytes |
| A, continued | - | `chmod u+w` on the **parent**, nothing else | **exit 0** — the binary creates the directory and writes the hook there |
| B (`RysJ6x`, `G6p7kW`) | as above | the route's condition met exactly: the hooks directory created and writable, the parent back at mode 555 | **exit 0** |

**Per triage's two questions: the condition is not already true, and the re-run exits 0 once it
is.** That is the opposite of the four causes above, so this cause does not belong to the
finding. What remains is a matter of wording and not of a failing route: the sentence names the
directory, and the act a reader must take is on its parent, because the directory can be neither
created nor made writable until the parent is.

## Triage's two questions, per cause

| cause | is the route's condition already true at the refusal | does the re-run exit 0 once it is |
|---|---|---|
| the hooks directory itself read-only (the positive control) | no | **yes** |
| `core.hooksPath=/dev/null` | **yes** | **no** — exit 1, the same bytes |
| `core.hooksPath` a regular file | **yes** | **no** — exit 1, the same bytes |
| a read-only hook file in a writable hooks directory | **yes** | **no** — exit 1, the same bytes |
| a hook with one byte that is not UTF-8 | **yes** | **no** — exit 1, the same bytes |
| `core.hooksPath` a missing directory under a read-only parent | no | **yes** |

The same on both binaries, row for row.

## The attempts to refute it

- **Stale state, another binary, a pipe, a cut.** Each cause ran on a rig that did not exist a
  moment before, with the hash asserted and `command -v jigc` checked in the same process; every
  exit was read bare and every stderr was kept whole in a file and compared with `cmp`.
- **The route's act was the wrong act.** The positive control uses the same act and ends at exit
  0, on both binaries. In the four causes the act changes nothing because nothing was missing:
  the path was writable before the first run.
- **A settled decision intends it.** I read the section that owns the behaviour,
  `design/assistant-adapter.md`: the install discipline (*it must resolve the real hooks dir
  (honoring `core.hooksPath` …)*) and the paragraph *A file the install replaces is written as a
  regular file at exactly its path, never through a link*. The second names this neighbourhood
  and leaves it out of its rule: "**Not this rule's subject:** a directory or special file at a
  merged-into path, which the writer fails on loudly as before, and a `core.hooksPath` that
  itself points outside the repository". *Fails on loudly* is what the binary does; no sentence
  there, and none I found in `DECISIONS.md`, `implementation/decisions-pending.md` or `design/`
  by `command grep` for the route's words, for `setup.install-hook` and for `core.hooksPath`
  beside `/dev/null`, says a writability route is the right route for a cause that is not
  writability. The sentence was introduced as "the established `setup.*` finding+route idiom"
  (`DECISIONS.md`, the M19 increment plan, T2/T3): one route for any input-output failure of the
  hook's install. That is a description of how it was built, not a ruling that it is right for
  every cause. The nearest precedent runs the other way: a malformed committed
  `.claude/settings.json` once drew *ensure `.claude/settings.json` is writable* for a parse
  error (`DECISIONS.md`, the M54 entry that drove the two wedges), and
  `design/project-setup.md` now says that case is "refused with the parser's own error and a
  route to fix and commit it".
- **It breaks no clause inside the clause's scope.** Below.

## Does it break the clause, inside its scope

`DECISIONS.md` → *2026-10-04 — The exit rule, revised*, the second clause's measure: "no command
that works on rc.24 in a supported layout stops working, and every refusal's route works as
printed".

- **The first half is intact.** `jigc setup` does not work in these four states on rc.24
  either; nothing that worked there stopped working.
- **The second half is broken, to the letter.** A refusal prints a route; the route's condition
  holds already; the re-run it names exits 1 with the same bytes; and the door recovers only by
  an act no surface names. `design/surface-contract.md`, law 2: "Every affordance that is the
  designated recovery for a state is named by the surfaces that produce that state".
- **Scope.** The layout is a single plain checkout: no worktree, no bare store, no submodule, no
  separate git directory. What varies is the user's own hook arrangement — a knob the design
  says the install honours (`core.hooksPath`), and two ordinary properties of a file the install
  is designed to preserve (its mode, its encoding). The *deliberately planted states* bound is
  the first clause's; the run's opening record declares no bound; and I found no decision that
  puts a user's hook arrangement outside the second clause. A persistent
  `core.hooksPath=/dev/null` is how a repository is told to run no hooks; this repository's own
  tooling tests configure one that way (`tooling-tests/dev_stabilize_step.rs`, a rig no
  `jigc setup` runs in).

**Why not `contested`.** The finding does not argue that a settled decision is wrong, and no fix
needs one overturned: it is one route string serving several failure points.

## The regression fact

**`regression: false`.** The same block and the same four variants, on fresh rigs, with the
previous release called by its absolute path: the same exits, and stderr byte-identical to the
candidate's in both runs of every cause. Red there and red here. The block is comparable — the
door, the code and the route's sentence all exist on the previous release.

## Coverage — the block's `UNPINNED`, verified from the suites

The block I was handed says no test asserts the route's text and no suite plants any of the
four causes. From `command grep` over `crates/cli/tests` and `tooling-tests`:

- *hooks directory is writable* occurs in **no** test: its only two occurrences in `crates`,
  `tooling-tests` and `dev` are its producers, `crates/cli/src/setup.rs` lines 2395 (`setup`)
  and 5050 (`uninstall`).
- `setup.install-hook` is named by four suites: `setup_failed_first_run` and
  `setup_install_pathspec_guard` (a hooks **directory** at mode 555, and layouts with no work
  tree), `replacing_writers_never_follow` (the hook as a link), and a doc-comment of
  `linked_worktree_doc_home`. None plants one of the four causes.
- Every `core.hooksPath` a suite writes into a repository's config where `jigc setup` then runs
  names a directory (`.githooks`, `my-hooks`, `ro-hooks`, `shared-hooks/hooks`, `nested/hooks`,
  or an absolute directory). `core.hooksPath=/dev/null` appears in five suites as a `-c` flag on
  a git call of the test's own, and once in a config, in the tooling rig named above.
- The two places a test applies mode 555 near a hook apply it to the hooks directory. No hit for
  *valid UTF-8* or *stream did not contain* concerns a hook.

So the claim holds as far as those searches reach; a suite that reaches one of these states by
another spelling would have been missed.

## Repro VP3-1 — the writability route where the hooks path is already writable

```yaml
claim: "`jigc setup` prints `ensure the repo's git hooks directory is writable` for failures that are not about that directory's writability; the path git names as the hooks directory is writable before the first run, and the re-run fails with the same bytes"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "the same exits and the same stderr, byte for byte (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, called by its absolute path)"
regression: false
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare                              # dev/jigc-rig bare: one commit, no `jigc setup`
  - ["git", "config", "core.hooksPath", "/dev/null"]
repro:
  - assert: "the path `git rev-parse --path-format=absolute --git-path hooks` prints is writable"   # the route's condition, true before run 1
  - ["jigc", "setup"]                          # run 1
  - assert: "that path is still writable"
  - ["jigc", "setup"]                          # run 2, the route as printed
expect:
  exit: 1                                      # both runs
  stdout: ""                                   # both runs
  stderr_contains:
    - "blocking · setup.install-hook"
    - "cannot install the `pre-commit` hook into the repo's hooks dir: File exists (os error 17)"
    - "route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`"
  stderr_run_2: "byte-identical to run 1"
  then: "`git config --unset core.hooksPath`, which the route does not name; `jigc setup` exits 0, one install commit, porcelain empty"
variants:
  - cause: "core.hooksPath = a regular file outside the repository, mode 644"
    expect: "the same, `File exists (os error 17)`; cleared by unsetting the knob"
  - cause: "an existing .git/hooks/pre-commit of the user's ('#!/bin/sh\\necho user-hook-ran\\n'), mode 555, the hooks directory mode 755"
    expect: "exit 1 twice, `Permission denied (os error 13)`, the hook byte-identical; cleared by `chmod u+w` on the hook file; then exit 0 with the user's line kept"
  - cause: "an existing .git/hooks/pre-commit of the user's holding one byte 0xFF, mode 755"
    expect: "exit 1 twice, `stream did not contain valid UTF-8`, the hook byte-identical; cleared by rewriting it as UTF-8; then exit 0"
controls:
  - cause: "no cause planted"
    expect: "exit 0, one install commit, porcelain empty"
  - cause: "the hooks directory itself mode 555"
    expect: "the hooks path is NOT writable; exit 1 `Permission denied (os error 13)` with the same route; `chmod u+w` on it; re-run exit 0 — the route is right here"
  - cause: "core.hooksPath = a missing directory under a parent of mode 555"
    expect: "the hooks path does not exist; exit 1 `Permission denied (os error 13)` with the same route; once the directory exists and is writable, or its parent is writable, the re-run exits 0 — the route's condition was false, and meeting it works"
green-when-fixed: "in each of the four causes, run 1's route no longer says `hooks directory is writable` while the path git names is writable; it names an act, and after that act alone `jigc setup` exits 0 with one install commit and an empty porcelain. The two refusing controls keep the writability route"
observed: "candidate <W>/jigc-rig-bare-YW36Td, nF5Gi8, t2Be8o, D4JuQu, controls Er7Q5e, sNCgte, DQ1g5t, RysJ6x; previous release hcILls, jRCxwt, B7p04K, bYP6vv, controls GVoA3p, yMjfZI, jzoi3A, G6p7kW; <W>/cand.part1.txt, cand.part2.txt, prev.txt"
pinned-by: "UNPINNED: no test asserts this route's text and no suite plants any of the four causes; setup_failed_first_run and setup_install_pathspec_guard pin the code under a read-only hooks directory, the control where the route is right"
```

**Pinnable as it stands: yes.** Every step is argv or one file-system act, and the expectations
are exits, substrings, a byte comparison and one writability check; no value depends on the
machine. As a statement of what the binary does today it converts to a test unchanged. **As the
fix's red test it rests on `green-when-fixed`,** which is written without assuming what the new
route says: its first half (the writability sentence is gone where the path is writable) is
mechanical; its second half (the named act leads to exit 0) needs the fixer's wording for each
cause before it can be asserted. That wording is the fixer's against law 2, not a ruling.

## The class

**`instance, unbounded`.** What I drove: six states on two binaries — the four causes, the
positive control, and the missing directory in two forms — plus an untouched control. Read, not
enumerated as a class: the three sites that build `setup.install-hook`
(`crates/cli/src/setup.rs` lines 2381, 2393 and 3144, by `command grep`), of which this finding
is the second, and the five points of `install_precommit_hook` whose error it wraps — the hooks
path's resolution, `create_dir_all`, the read of an existing hook, the write, and the change of
mode. Three of the five were reached here: `create_dir_all` (`/dev/null`, a regular file, the
missing directory), the read (the byte that is not UTF-8), the write (a read-only hook, a
read-only directory). **Not driven:** a failing resolution of the hooks path; a failing change
of mode; a hook that is a directory, unreadable, or a compiled program; `core.hooksPath` naming
a file or a missing directory **inside** the repository; `--force`; the route as
`--format json` renders it; any run without the two git-config masks.

## Left open — noticed, not pursued

- `jigc uninstall` prints the same sentence, ending *then re-run `jigc uninstall`*
  (`crates/cli/src/setup.rs`, line 5050). Whether it is printed there for causes that are not
  writability was not driven.
- For a missing hooks directory under a read-only parent the route works, and its sentence names
  the directory where the act is on the parent. A wording matter; not graded as a break here.
- A hook that is a compiled program would reach the same read as the byte that is not UTF-8.
  Not driven.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports live
in. No commit, no stage, no build. With my file tool I wrote this report and nothing else; the
driver and the evidence under `<W>` were written by the shell, inside the scratch root, and two
dumps of `--help` text went to the system temp directory.

<!-- end of report -->
