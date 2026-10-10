# verify-real — `r1-setup-install-hook-route-other-causes-unexamined` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc setup`. Clause it is said to break:
`working-product`. Triage's grade: *unclear*. The finding came with no block; triage asked for
the producers of `setup.install-hook` to be enumerated and every cause reachable in an ordinary
checkout to be driven on both binaries.

## Verdict in one paragraph

**`confirmed`, `regression: true` for the instance named in `Repro VR-1`, `contested: false`.**
I was sent to show that the route is right for the other causes, and for three of them it is: a
read-only hooks directory — the default one, or one `core.hooksPath` names inside or outside the
repository — is refused with *ensure the repo's git hooks directory is writable*, and the re-run
exits 0 once that is done. It does not survive the rest. **(1) A `pre-commit` hook that is a
link to an existing script outside the repository** draws a refusal that is new on the
candidate, and its route names the wrong file: it says *remove the link at* and then prints the
path of the script the link leads **to**, which is a regular file and no link. Done as printed,
the script outside the repository is deleted, the re-run exits 1 again, and only a second
refusal names the real link. The previous release has no refusal there at all: `jigc setup`
exits 0. **(2) Four causes draw the writability route while the hooks directory is already
writable** — `core.hooksPath` naming a regular file, `core.hooksPath=/dev/null`, an existing
hook file that is read-only, and an existing hook that holds a byte that is not UTF-8. The
route's condition holds before the first run, the re-run exits 1 with the same bytes, and the
door recovers only by an act the route does not name. Those four are the same on the previous
release. Both shapes are the letter of the clause's second measure, *every refusal's route works
as printed*, in a plain checkout.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Both hash calls were the plain call the prompt spells, made before the first command was
driven. With the candidate's directory first on `PATH`, `command -v jigc` printed the
candidate's path. Nothing was built and nothing under `target/` was driven. Both binaries print
the same version string; the evidence here is keyed by the hashes above.

## The producers of `setup.install-hook`

`command grep -n '"setup.install-hook"'` over `crates/cli/src` names **three** construction
sites, all in `crates/cli/src/setup.rs`, and no other file of the tree's sources:

| # | site | when | route it prints |
|---|---|---|---|
| P1 | line 2381 | `std::env::current_exe()` fails | re-run `jigc setup` (the install resolves its own absolute path) |
| P2 | line 2393 | `install_precommit_hook` returns any error | ensure the repo's git hooks directory is writable, then re-run `jigc setup` |
| P3 | line 3144, `hook_link_refusal` | the hook is a link that leads out of the repository and its hooks directory, or to nothing — asked before the first write | remove the link at `<shown>`, or point it at a script inside this repository, then re-run `jigc setup` |

**P2 is one fixed route string over five failure points.** Read in `install_precommit_hook`
(line 907): `resolve_hooks_dir` (git cannot be run, git exits non-zero, git prints nothing),
`create_dir_all` on the hooks directory, `read_to_string` of an existing hook with any error but
*not found*, `fs::write` of the hook, `make_executable`. Whatever failed, the text after
*cannot install the `pre-commit` hook into the repo's hooks dir:* is the operating system's own
error, and the route is the sentence about writability.

**P3 prints `<shown>` from `display_hook_path`,** which canonicalizes the hook's path before
it names it (line 960). Canonicalizing a link follows it. So wherever the link leads to a file
that exists, `<shown>` is that file; only where the link dangles does canonicalizing fail and
the function fall back to the link's own path.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-hookroute.caNI4r`. **One fresh rig per cause and per binary** — 17 for the
  candidate, 16 for the previous release — each built by `dev/jigc-rig bare --binary <path>`
  with `SCRATCH=<W>`: a git repository with one commit and no `jigc setup`. None is a
  reporter's.
- Environment of every run: `HOME` the rig's own empty directory, `GIT_CONFIG_NOSYSTEM=1`,
  `GIT_CONFIG_GLOBAL=/dev/null`, the identity the rig sets in the repository's own config. Git
  2.54.0 (Apple Git-157), macOS, an unprivileged user, so a mode of 555 binds.
- Every exit status was read bare: `jigc setup > out 2> err`, then `$?`, never through a pipe.
- The driver, `<W>/drive.sh` and `<W>/drive2.sh`, puts the binary's directory first on `PATH`
  and stops unless `command -v jigc` prints that binary's absolute path; it then builds the
  rig, plants one cause, runs `jigc setup`, does what the printed route says, and runs
  `jigc setup` again. Its transcripts are `<W>/cand.part1.txt`, `cand.part2.txt`,
  `cand.part3.txt`, `prev.part1.txt` and `prev.part2.txt`; each rig holds `ev/setup<n>.out`,
  `ev/setup<n>.err` and `ev/setup<n>.porcelain`.
- For the regression fact of `Repro VR-1` the previous release was also called **by its
  absolute path** on one more fresh rig (`<W>/jigc-rig-bare-VTX014`), with no `PATH` change.
- **What "the route as printed" was taken to mean.** For P2: make the directory
  `git rev-parse --git-path hooks` names writable (`chmod u+w`), then re-run. For P3: remove
  what is at the path the route prints, then re-run. Where that did not end at exit 0 I then
  did what actually clears the cause, to see whether the door recovers at all; those steps are
  marked *not the route* below.

## What was observed

A control first: `jigc setup` on an untouched `bare` rig exits 0 on both binaries, one install
commit, porcelain empty.

### Causes where the route is right, or where there is no refusal

| cause | candidate | previous release |
|---|---|---|
| default hooks directory mode 555 | exit 1, P2 with *Permission denied (os error 13)*; `chmod u+w` on it; re-run exit 0 | the same |
| `core.hooksPath` = an existing directory outside the repository, mode 555 | exit 1, P2; `chmod u+w`; re-run exit 0 | the same |
| `core.hooksPath` = an existing directory inside the repository, mode 555 | exit 1, P2; `chmod u+w`; re-run exit 0 | the same |
| `core.hooksPath` = a path that does not exist, outside the repository | exit 0 — the directory is created and the hook written there | the same |
| `core.hooksPath` = `my-hooks`, which does not exist | exit 0 — created, the hook in the install commit | the same |
| hook a link to a tracked script, `../../scripts/pre-commit` | exit 0, the ack names `scripts/pre-commit` | the same |
| hook a link to a sibling in the same hooks directory | exit 0, the ack names the sibling | the same |
| hook a link that leads to nothing | exit 1, P3, and the route names `<repo>/.git/hooks/pre-commit`, which is the link; removed; re-run exit 0; nothing was created where the link pointed | **exit 0** — no refusal; the file the link pointed at was created |

In each exit-1 row above the first run leaves the install's files written and uncommitted (nine
porcelain lines on the candidate, eight on the previous release) and the re-run commits them:
one install commit, porcelain empty. P3 refuses before the first write, as its text says.

### (1) A hook that is a link to an existing script outside the repository — P3 names the script as the link

Setup: a 31-byte script `<rig>/outside/shared-hook` (`#!/bin/sh`, one `echo`), mode 755, beside
the repository and in no repository; `.git/hooks/pre-commit` a symbolic link to its absolute
path.

Candidate, `<W>/jigc-rig-bare-zVoSck`:

| step | exit | what |
|---|---|---|
| `jigc setup` | **1** | stdout empty; stderr below; nothing installed, porcelain empty, the outside script byte-identical (`cmp` exit 0) |
| what is at the path the route names | - | `<rig>/outside/shared-hook`: **a regular file, no link**. `.git/hooks/pre-commit`, which the route does not name, is the link |
| the route as printed, first arm: remove what is at the named path | 0 | the outside script is gone |
| `jigc setup`, the re-run | **1** | a second refusal — the link now leads to nothing — and this one names `<repo>/.git/hooks/pre-commit` |
| the second route as printed: remove what is at the named path | 0 | the link is gone |
| `jigc setup` | 0 | one install commit, porcelain empty, a hook jigc wrote at `.git/hooks/pre-commit` |

stderr of the first run (the rig's path shortened):

```text
blocking · setup.install-hook — the `pre-commit` hook `<rig>/outside/shared-hook` is a symbolic link and leads out of this repository and its hooks directory, to `<rig>/outside/shared-hook` — `jigc setup` splices its block into the hook it finds, and it follows a link there only to a file of this repository. Nothing was installed and no install commit was made
  route: remove the link at `<rig>/outside/shared-hook`, or point it at a script inside this repository, then re-run `jigc setup`. `--force` does not change this: it consents to replacing a file, not to following a link
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

The refusal gives one path twice: as the hook that *is a symbolic link*, and as where that link
leads. The first is false. The route's second arm, *point it at a script inside this
repository*, cannot be done to the named path either: a regular file points nowhere.

**The refusal itself is intended and holds.** On two more candidate rigs
(`<W>/jigc-rig-bare-MEq1tR`, `<W>/jigc-rig-bare-MbVVVe`) I did what the route means and not
what it prints — removed the link at `.git/hooks/pre-commit`; and re-pointed that link at a
committed `scripts/pre-commit` — and the re-run exits 0 both times, with the outside script
untouched. So the door is recoverable; the path the route prints is what is wrong.

Previous release, `<W>/jigc-rig-bare-VmY2gk` and, by its absolute path,
`<W>/jigc-rig-bare-VTX014`: `jigc setup` exits **0**, stderr empty, one install commit,
porcelain empty. There is no refusal and no route. The link is still a link, and the script
outside the repository grew from 31 to 5159 bytes: jigc's block written into it through the
link, the user's `echo` line kept as its last 21 bytes.

### (2) Four causes where P2's route is printed and its condition already holds

In each, the directory `git rev-parse --git-path hooks` names is writable before the first run.
The same on both binaries, byte for byte in stderr.

| cause | first `jigc setup` | the route as printed | re-run | what clears it — *not the route* | then |
|---|---|---|---|---|---|
| `core.hooksPath` = a regular file outside the repository, mode 644 | exit 1, *File exists (os error 17)* | `chmod u+w` on the path git names: it was writable already | **exit 1**, the same bytes | `git config --unset core.hooksPath` | exit 0 |
| `core.hooksPath=/dev/null` | exit 1, *File exists (os error 17)* | nothing to change: the path is writable | **exit 1**, the same bytes | `git config --unset core.hooksPath` | exit 0 |
| an existing 31-byte hook of the user's, mode 555, in a hooks directory of mode 755 | exit 1, *Permission denied (os error 13)* | `chmod u+w` on the hooks directory: it was writable already | **exit 1**, the same bytes | `chmod u+w` on the hook **file** | exit 0, the user's line kept after jigc's block |
| an existing hook of the user's holding one byte that is not UTF-8, mode 755 | exit 1, *stream did not contain valid UTF-8* | `chmod u+w` on the hooks directory: it was writable already | **exit 1**, the same bytes; the hook byte-identical | rewrite the hook as UTF-8 | exit 0 |

stderr, the second cause, both runs, both binaries:

```text
blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: File exists (os error 17)
  route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

Rigs: candidate `8qDpcm`, `Ub48OD`, `aQ7c9e`, `r31UBo`; previous release `udw9ml`, `YWomqk`,
`CHbIRg`, `AyGagA` (each `<W>/jigc-rig-bare-<suffix>`).

**One cause I grade neither way:** `core.hooksPath` naming a directory that does not exist
under a parent of mode 555. Exit 1 with *Permission denied*, on both binaries; the hooks
directory cannot be made writable because it is not there, and the re-run exits 0 once the
parent is writable. A reader told to *ensure the hooks directory is writable* may well create
it; whether the sentence names that cause is a matter of reading, and I leave it as observed.

## Triage's two questions, answered per cause

| cause | does the route name the real cause | does the re-run succeed once the route's condition is met |
|---|---|---|
| a read-only hooks directory (default, or `core.hooksPath` in or out of the repository) | yes | yes |
| `pre-commit` a link to a script of the repository or a sibling | no refusal | - |
| `pre-commit` a link to nothing | yes | yes |
| `pre-commit` a link to an existing file outside the repository | the cause yes, **the path no** — it names the link's end as the link | **no**: exit 1 again, and the act it names deletes a file outside the repository |
| `core.hooksPath` naming a missing path | no refusal — it is created | - |
| `core.hooksPath` naming a missing path under a read-only parent | arguable | arguable |
| `core.hooksPath` naming a file, or `/dev/null` | **no** | **no**: the condition already holds |
| `core.hooksPath` naming an unwritable directory | yes | yes |
| an existing hook file that is read-only | **no** — it names the directory | **no**: the condition already holds |
| an existing hook that is not UTF-8 | **no** | **no**: the condition already holds |

The last two rows are not in triage's list. They came out of the enumeration triage asked for —
they are the `fs::write` and `read_to_string` failure points of P2 — and they are states of an
ordinary checkout, so I drove them.

## What the record holds

**The design section that owns the behaviour** — `design/assistant-adapter.md`, the paragraph
*A file the install replaces is written as a regular file at exactly its path, never through a
link*. Of the merged-into members it gives the route as *remove the link, or point it at a
regular file inside this repository*, and says: "The `pre-commit` hook is asked the same way
where it is itself a link: followed to a script in the repository or in its own hooks
directory, refused (`setup.install-hook`) where it leads anywhere else or nowhere." So the
refusal of (1) is intended, and what it must name is **the link**. No sentence of the design
has it name the link's end. The same paragraph leaves "a directory or special file at a
merged-into path" to the writer, "which the writer fails on loudly as before"; it says nothing
of what that failure's route is to say, and nothing of a hooks directory that is a file.

**`design/surface-contract.md` → The three laws.** Law 1: "Every claim a surface makes is
generated from the thing it describes, or asserted against it … every printed path is repo-real
or a typed identity". The refusal of (1) claims that a regular file is a symbolic link. Law 2:
"Every affordance that is the designated recovery for a state is named by the surfaces that
produce that state". In (2) the recovery — unset the knob, make the file writable, re-encode
the hook — is named by no surface. That document's *Honest bounds* say its fences hold through
the suite and not in a release binary; they bound how the laws are enforced, not what they say.

**The clause.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, the second clause's
measure: "no command that works on rc.24 in a supported layout stops working, and every
refusal's route works as printed". The scope I drove is a single plain checkout — no worktree,
no bare store, no submodule — so the layout question the neighbouring finding turned on does not
arise here. What varies is the user's own hook arrangement, and each state is one the design
names (`core.hooksPath` is honoured by the install discipline; a hook that is a link is the
paragraph quoted above) or an ordinary property of a file (its mode, its encoding). The
repository's own suites use `-c core.hooksPath=/dev/null` as the way to run git with hooks off
(`crates/cli/tests/home_vacated.rs`, among five files). The *deliberately planted states* bound
is the first clause's, and I found no decision that puts a user's hook arrangement outside the
second.

**Nothing recorded as known or deferred.** `command grep` over `DECISIONS.md`,
`implementation/decisions-pending.md` and `design/` for the route's sentence, for a read-only
or non-UTF-8 hook and for `core.hooksPath` beside `/dev/null` finds no entry.

## Does it break the clause, inside its scope

- **The second measure — broken, twice over.** (1) A refusal's route, done as printed, ends in
  the same code at exit 1 and has removed a file the repository does not hold. (2) A refusal's
  route whose condition is already true: nothing the route says changes the answer.
- **The first measure.** In (1), `jigc setup` exits 0 on the previous release and 1 on the
  candidate. That stop is the intended one — the symlink ruling the design paragraph records —
  and I do not grade it as a break by itself. It is why the route matters: the route is the
  only way back to a working `setup` in that state, and as printed it does not lead there.

**Why `confirmed`.** *Does not reproduce:* it reproduces from nothing, each cause on its own
fresh rig. *Intended:* the refusal of (1) is intended, the path it prints is not — the design's
own words name the link; for (2) no decision says a writability route is right for a cause that
is not writability. *Breaks no clause:* both shapes are the measure's letter in a plain
checkout. **Why not `contested`.** The finding does not argue that a settled decision is wrong,
and no fix needs one overturned: (1) is one path resolved before it is printed, (2) is one
route string serving several causes.

## The regression fact

Per instance, because the two producers differ:

- **`Repro VR-1` (the link to an outside script): `regression: true`.** Previous release,
  called by its absolute path on a fresh rig: `jigc setup` exits 0, no refusal, nothing to
  follow. Candidate: exit 1, the route as printed, exit 1. Green there, red here. What the
  previous release does at that exit 0 is write into a file outside the repository through the
  link — the cell the candidate's refusal exists to close — so the fact is that the *route* is
  new and wrong, not that the refusal should go.
- **`Repro VR-2` (the four P2 causes): `regression: false`.** Red in the same way on both
  binaries, stderr byte for byte.

The return's one `regression` field carries VR-1's `true`, since that is the block the return
names; VR-2's `false` is stated here and in the basis.

## Coverage — verified from the suites, for the blocks' `pinned-by`

The finding makes no coverage claim. For the blocks below I looked at what stands, by
`command grep` over `crates/cli/tests` and `tooling-tests`:

- The sentence *hooks directory is writable* is asserted by **no** test: the only two hits in
  the tree are its two producers in `crates/cli/src/setup.rs` (lines 2395 and 5050).
- `replacing_writers_never_follow::a_merged_into_member_refuses_a_link_the_repository_cannot_commit`
  drives the hook as a link to an outside file and to nothing: it asserts exit 1, the code,
  nothing installed and the outside file untouched, on `setup` and on `setup --force`. It then
  performs the route itself — `fs::remove_file(repo.join(".git/hooks/pre-commit"))` — and
  asserts exit 0. **It never reads the path the route prints**, which is how the wrong path
  stands under a green suite.
- `setup_failed_first_run` asserts the code under a read-only hooks directory, the one cause
  whose route is right.
- No suite sets `core.hooksPath` to a file in a repository's config, plants a read-only hook
  file, or plants a hook that is not UTF-8, as far as those three searches reach (`hooksPath`
  beside `/dev/null`; `valid UTF-8`; the route's sentence). A suite that reaches one of these
  states by another spelling would have been missed.

## Repro VR-1 — `jigc setup` over a hook linked to an outside script: the route names the script as the link

```yaml
claim: "where .git/hooks/pre-commit is a link to an existing script outside the repository, `jigc setup` refuses at setup.install-hook and its route says `remove the link at` the path of the script the link leads to; done as printed, the script is deleted and the re-run exits 1 again"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "exit 0, no refusal, no route (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, called by its absolute path); the outside script is written through the link, 31 -> 5159 bytes, the user's line kept"
regression: true
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare                              # dev/jigc-rig bare: one commit, no `jigc setup`
  - write: "<rig>/outside/shared-hook = '#!/bin/sh\necho shared-hook-ran\n', mode 755"   # 31 bytes, beside the repo, in no repository
  - symlink: ".git/hooks/pre-commit -> <abs>/outside/shared-hook"
repro:
  - ["jigc", "setup"]                          # run 1
  - remove: "what is at the path run 1's route prints after `remove the link at`"
  - ["jigc", "setup"]                          # run 2, the route as printed
expect:
  run_1:
    exit: 1
    stdout: ""
    stderr_contains:
      - "blocking · setup.install-hook"
      - "the `pre-commit` hook `<abs>/outside/shared-hook` is a symbolic link"
      - "route: remove the link at `<abs>/outside/shared-hook`"
    files: "no .jigc directory; porcelain empty; outside/shared-hook byte-identical"
    the_named_path: "a regular file, not a symbolic link; .git/hooks/pre-commit is the link and is not named"
  after_the_route: "outside/shared-hook no longer exists; .git/hooks/pre-commit is still a link"
  run_2:
    exit: 1
    stderr_contains:
      - "blocking · setup.install-hook"
      - "leads to nothing"
      - "route: remove the link at `<abs>/repo/.git/hooks/pre-commit`"
green-when-fixed: "the path run 1's route prints is itself a symbolic link; removing what is at it leaves outside/shared-hook byte-identical; run 2 exits 0 with one install commit and an empty porcelain"
variants:
  - "the hook a link to nothing: exit 1, the route names .git/hooks/pre-commit, removing it and re-running exits 0 — correct on the candidate; exit 0 on the previous release, which creates the file the link points at"
  - "acting on the real link instead of the printed path (remove it, or re-point it at a committed scripts/pre-commit): re-run exit 0 on the candidate"
observed: "<W>/jigc-rig-bare-zVoSck (candidate, as printed), MEq1tR and MbVVVe (candidate, the real link), VmY2gk and VTX014 (previous release); <W>/cand.part3.txt, cand.part1.txt, prev.part1.txt"
pinned-by: "UNPINNED for the route's path. replacing_writers_never_follow::a_merged_into_member_refuses_a_link_the_repository_cannot_commit pins exit 1, the code, nothing installed and the outside file untouched, then removes .git/hooks/pre-commit by a literal of its own and never reads the path the route prints"
```

**Pinnable as it stands:** yes. Every step is argv or one file-system act, the expectations are
exits, substrings and what is at a path, and the one machine-dependent value is the absolute
path of the rig. `green-when-fixed` is written without assuming how the fixed route spells the
link, so it serves as the fix's red test unchanged.

## Repro VR-2 — the writability route where the hooks directory is already writable

```yaml
claim: "`jigc setup` prints `ensure the repo's git hooks directory is writable` for failures that are not about that directory's writability; the condition already holds and the re-run fails with the same bytes"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "the same exits and the same stderr, byte for byte (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
regression: false
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare
  - ["git", "config", "core.hooksPath", "/dev/null"]
repro:
  - ["jigc", "setup"]                          # run 1
  - assert: "the path `git rev-parse --git-path hooks` prints is writable"   # the route's condition, already true
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
  - cause: "an existing .git/hooks/pre-commit of the user's, mode 555, hooks directory mode 755"
    expect: "exit 1 twice, `Permission denied (os error 13)`; cleared by `chmod u+w` on the hook file; then exit 0 with the user's line kept"
  - cause: "an existing .git/hooks/pre-commit holding one non-UTF-8 byte, mode 755"
    expect: "exit 1 twice, `stream did not contain valid UTF-8`, the hook byte-identical; cleared by rewriting it as UTF-8"
  - cause: "the hooks directory itself mode 555 (default, or core.hooksPath in or out of the repository)"
    expect: "exit 1 `Permission denied (os error 13)`, then `chmod u+w` on it and exit 0 — the route is right here"
observed: "candidate <W>/jigc-rig-bare-Ub48OD, 8qDpcm, aQ7c9e, r31UBo, YTj2dE, KENTVo, SNbHrK; previous release YWomqk, udw9ml, CHbIRg, AyGagA, kLnEI7, 7XS45t, Twg7Kl; <W>/cand.part2.txt, prev.part2.txt"
pinned-by: "UNPINNED: no test asserts this route's text, and no suite plants any of the four causes. setup_failed_first_run pins the code under a read-only hooks directory, the variant where the route is right"
```

**Pinnable as it stands:** yes, as a statement of what the binary does today. **As a fix's red
test it needs one thing the block cannot give:** what the route is to say for each cause. That
is the fixer's wording against the design's law 2, not a ruling; the block's `then:` lines say
what clears each cause.

## The class

**Producers: enumerated — three**, by `command grep -n '"setup.install-hook"'` over
`crates/cli/src` (one file, `setup.rs`, lines 2381, 2393 and 3144), and P2's five failure points
read off `install_precommit_hook`. **Causes: `instance, unbounded`.** On two binaries I drove
fourteen states: eight that reach P2 (three at `create_dir_all`, one at `read_to_string`, four
at `fs::write`), two that reach P3 (a link to an outside script, a link to nothing), and four
that reach no refusal (a link to a tracked script, a link to a sibling, and `core.hooksPath`
naming a missing path inside and outside the repository). Not driven: P1; P2's
`resolve_hooks_dir` failures, which are the neighbouring finding's subject; `make_executable`
failing; a hook that is a directory or unreadable; an outward link written relatively;
`--force`; the route as `--format json` renders it. The other consumers of `display_hook_path`
were not enumerated.

## Left open — noticed, not pursued

- `jigc uninstall` prints the same writability sentence under `uninstall.remove-precommit`
  (`crates/cli/src/setup.rs`, line 5050), from the same hooks-directory resolution and a read
  of the hook that fails the same ways. Not driven.
- `display_hook_path` follows a link before it names a hook. Its other callers — the install's
  ack, and the teardown's *left the `pre-commit` hook in place* warning — were not driven; the
  warning tells its reader to *delete the file yourself* at whatever path that function prints.
- On the previous release a hook linked to nothing has the file it points at created at exit 0,
  and one linked to an outside script is written through. Both are closed on the candidate by
  P3; recorded here only because the regression fact above rests on that exit 0.
- `core.hooksPath` naming a missing directory under a read-only parent: graded neither way
  above.
- Every P2 refusal comes after the install's files are written, on both binaries, and the next
  successful run commits them. I observed it in each exit-1 row and looked no further.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports live
in. No commit, no stage, no build. With my file tool I wrote this report and nothing else; the
two driver scripts and the evidence under `<W>` were written by the shell, inside the scratch
root, and two dumps of `--help` text went to the system temp directory.

<!-- end of report -->
