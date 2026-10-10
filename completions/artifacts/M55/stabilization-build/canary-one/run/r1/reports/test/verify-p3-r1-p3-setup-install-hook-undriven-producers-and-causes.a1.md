# verify-real — `r1-p3-setup-install-hook-undriven-producers-and-causes` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc setup`. Clause it is said to break:
`working-product`. Triage's grade: *unclear*. The finding came with no block: it is the list of
what an earlier verifier of this round left undriven under its heading *The class*. Triage asked
for `make_executable` failing, a hook that is a directory, a hook that cannot be read, an
outward link written relatively, and each refusal under `--force` and under `--format json`,
on both binaries: does the route name the cause, and does it work as printed.

## Verdict in one paragraph

**`confirmed`, `regression: true` for the instance named in `Repro VP-1`, `contested: false`.**
I was sent to show that the undriven causes are fine, and three of the four are not. **(1) An
outward link written relatively** (`.git/hooks/pre-commit -> ../../../outside/shared-hook`)
draws the candidate's link refusal, and its route says *remove the link at* and then prints the
path of the script the link leads **to** — a regular file, no link. Done as printed, the script
outside the repository is deleted and the re-run exits 1 again. The previous release has no
refusal there: `jigc setup` exits 0. The relative spelling changes nothing against the absolute
one. **(2) A hook path that is a directory, and a hook file that cannot be read** (mode 000, or
write-only), each draw *ensure the repo's git hooks directory is writable* while that directory
is writable already: the re-run exits 1 with the same bytes, and it does so too after a
recursive `chmod u+w` over the hooks directory. Those are the same on the previous release, byte
for byte. **(3) `--force` and `--format json` change no refusal and no route**: for every
cause, the refusal under `--force` is byte-identical to the plain one, and the json envelope's
`message` and `route` strings are the plain text's. **(4) `make_executable` failing was not
reached on the candidate** — said plainly below, with what I tried. **No third mechanism came
out of this:** every red cell here is one of the two the earlier report's blocks already carry
(one path resolved through the link before it is printed; one route sentence serving several
causes). What this report adds is their members and the two flags.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Both hash calls were the plain call the prompt spells, made before the first command was
driven. With the candidate's directory first on `PATH`, `command -v jigc` printed the
candidate's path. Nothing was built and nothing under `target/` was driven. Evidence is keyed by
the hashes above, never by the version string.

## What was read

- The finding as handed: its key, its door, its clause, and the heading *The class* of the
  earlier report, whose last sentences are this finding. No other finding's report, and nothing
  of triage's reasoning but the grade and the list above.
- The clause: `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, second sharpening: "no
  command that works on rc.24 in a supported layout stops working, and every refusal's route
  works as printed". The run's opening record declares no bound.
- The design section that owns the behaviour: `design/assistant-adapter.md`, the paragraph *A
  file the install replaces is written as a regular file at exactly its path, never through a
  link*. Its sentences on the hook: "The `pre-commit` hook is asked the same way where it is
  itself a link: followed to a script in the repository or in its own hooks directory, refused
  (`setup.install-hook`) where it leads anywhere else or nowhere." and "**Not this rule's
  subject:** a directory or special file at a merged-into path, which the writer fails on loudly
  as before".
- The producers, in `crates/cli/src/setup.rs`: `install_precommit_hook` (line 907),
  `display_hook_path` (960), `make_executable` (1286), the two construction sites in `install`
  (2381, 2393) and `hook_link_refusal` (3120). They are as the earlier report lists them: three
  sites, and the second is one fixed route sentence over five failure points.
- `design/command-output-contract.md` → where a driver reads the document: a reject is one
  document on stderr with stdout empty. That is what `--format json` did in every refusal here,
  so the stream is as designed.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-p3hook.NaDUGK`. **One fresh rig per cause, per mode and per binary**, each
  built by `dev/jigc-rig bare --binary <path>` with `SCRATCH=<W>`: a git repository with one
  commit and no `jigc setup`. 120 rigs in all — 61 for the candidate, 59 for the previous
  release; two of them (one per binary) stopped on a fault of my own driver before any `jigc`
  call and one was a hand-built look at the rig's output, and none of those three is evidence.
  None is a reporter's.
- Environment of every run: `HOME` the rig's own empty directory, `GIT_CONFIG_NOSYSTEM=1`,
  `GIT_CONFIG_GLOBAL=/dev/null`. Git 2.54.0 (Apple Git-157), macOS, an unprivileged user.
- Every exit status was read bare: `jigc setup <flags> > out 2> err`, then `$?`.
- The driver, `<W>/drive.sh <cand|prev> <cause> <plain|force|json|forcejson>`, puts the
  binary's directory first on `PATH` and stops unless `command -v jigc` prints that binary's
  absolute path. It builds the rig, plants one cause, runs `jigc setup`, does what the printed
  route says, runs `jigc setup` again, and where that is not exit 0 does what actually clears
  the cause and runs a third time. Transcripts: `<W>/cand.matrix.txt` and `<W>/prev.matrix.txt`
  (13 causes × 4 modes each), `<W>/control.txt`, `<W>/control2.txt`, `<W>/cand.plain1.txt`;
  each rig holds `ev/<n>.out` and `ev/<n>.err`.
- `<W>/drive_abs.sh` calls a binary **by its absolute path with no `PATH` change**, for the
  regression fact and for the generous reading of the writability route; its transcript is
  `<W>/abs.txt`.
- **What "the route as printed" was taken to mean.** For the writability sentence: make the
  directory `git rev-parse --git-path hooks` names writable (`chmod u+w`), then re-run. For the
  link sentence: remove what is at the path printed after *remove the link at*, then re-run —
  but only where that path is inside my own rig. Steps beyond that are marked *not the route*.

## What was observed

Controls: `jigc setup` and `jigc setup --format json` on an untouched `bare` rig exit 0 on both
binaries, one install commit, porcelain empty.

### The handed causes, on the candidate

Each row is four rigs — plain, `--force`, `--format json`, `--force --format json` — and the
four agree in every exit and, the rig's path aside, in every byte of the refusal.

| cause | producer, and the reason it prints | run 1 | the route as printed | run 2 | what clears it — *not the route* | run 3 |
|---|---|---|---|---|---|---|
| the hook path is an empty directory | the fixed-route site; *Is a directory (os error 21)* | exit 1 | the hooks directory was writable already; `chmod u+w` changes nothing | **exit 1**, the same bytes | `rmdir` the directory at the hook path | exit 0 |
| the hook path is a directory holding one file | the same | exit 1 | the same | **exit 1**, the same bytes | remove the file and the directory | exit 0 |
| the user's hook, mode 000 | the fixed-route site; *Permission denied (os error 13)* | exit 1 | the hooks directory was writable already | **exit 1**, the same bytes | `chmod u+rw` on the hook **file** | exit 0, the user's line kept |
| the user's hook, mode 200 (write-only) | the same | exit 1 | the same | **exit 1**, the same bytes | `chmod u+rw` on the hook file | exit 0, the user's line kept |
| the hook a relative link to an existing script outside the repository | the link refusal | exit 1, nothing installed | it names `<rig>/outside/shared-hook`: a regular file, **no link**; removed | **exit 1**, a second refusal, which names `<rig>/repo/.git/hooks/pre-commit` | remove the link the second refusal names | exit 0 |
| the hook a relative link to nothing | the link refusal | exit 1, nothing installed | it names `<rig>/repo/.git/hooks/pre-commit`, which **is** the link; removed | exit 0 | - | - |
| the hook a link to `/dev/null` (my attempt at `make_executable`, below) | the link refusal | exit 1, nothing installed | it names `/dev/null` as the link: a character device. Not acted on — outside my rig | exit 1, the same bytes | remove the link at `.git/hooks/pre-commit` | exit 0 |

In the four fixed-route rows the first run leaves the install's files written and uncommitted
(nine porcelain lines) and the third run commits them: one install commit, porcelain empty. The
link refusal comes before the first write, as its text says.

stderr of the first run, the directory (identical on both binaries):

```text
blocking · setup.install-hook — cannot install the `pre-commit` hook into the repo's hooks dir: Is a directory (os error 21)
  route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

stderr of the first run, the relative outward link (candidate):

```text
blocking · setup.install-hook — the `pre-commit` hook `<rig>/outside/shared-hook` is a symbolic link and leads out of this repository and its hooks directory, to `<rig>/outside/shared-hook` — `jigc setup` splices its block into the hook it finds, and it follows a link there only to a file of this repository. Nothing was installed and no install commit was made
  route: remove the link at `<rig>/outside/shared-hook`, or point it at a script inside this repository, then re-run `jigc setup`. `--force` does not change this: it consents to replacing a file, not to following a link
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

**The generous reading of the writability sentence does not rescue it.** On two more candidate
rigs, called by absolute path, I did `chmod -R u+w` over the hooks directory instead of the
plain `chmod u+w`: the mode-000 hook becomes mode 200 and the re-run exits 1 with the same
bytes; the directory at the hook path is untouched in kind and the re-run exits 1 with the same
bytes.

### `--force` and `--format json`, over every refusal — the earlier report's causes included

Thirteen causes on each binary: the seven above, and six the earlier report drove plain only (a
link to an outside script written absolutely, a link to nothing written absolutely, a hooks
directory of mode 555, `core.hooksPath=/dev/null`, a read-only hook file, a hook holding a byte
that is not UTF-8).

- **`--force`:** for all thirteen, on both binaries, run 1's stderr under `--force` equals the
  plain run's once the rig's path is replaced, and every exit of the sequence is the same. The
  link refusal's own sentence, *`--force` does not change this*, is true as driven. The
  writability sentence says nothing of `--force`, and `--force` changes nothing there.
- **`--format json`:** every refusal is one findings envelope on stderr, stdout empty, exit 1:
  `schema_version` 3, one finding, `severity` `blocking`, `code` `setup.install-hook`,
  `key.target` and `location` null. Its `message` and `route` strings equal the plain text's
  for all thirteen causes on the candidate and for the nine that refuse on the previous
  release. The json under `--force` is byte-identical to the json without it.
- **So the route is right or wrong in exactly the cells where it is right or wrong plain.**
  Right: a hooks directory of mode 555 (re-run exit 0 after `chmod u+w`, all four modes, both
  binaries) and a link to nothing (re-run exit 0 after removing the named path, all four modes,
  candidate). Wrong: every other refusing cause, in all four modes.

### The previous release

| cause | previous release, all four modes |
|---|---|
| directory at the hook path (empty, or holding a file) | exit 1, exit 1, then exit 0 after the clearing act — stderr equal to the candidate's |
| hook of mode 000, or 200 | the same — stderr equal to the candidate's |
| relative link to an outside script | **exit 0**, no refusal; the link still a link; the outside script 31 → 5159 bytes, written through |
| relative link to nothing | **exit 0**, no refusal; the file the link pointed at was created |
| link to `/dev/null` | exit 1 from the fixed-route site, *Operation not permitted (os error 1)*; the hooks directory writable already; re-run exit 1, the same bytes; exit 0 once the link is removed |

Called by its absolute path on three more fresh rigs (`<W>/abs.txt`): the relative outward link
exits 0 with the script grown to 5159 bytes; the directory and the mode-000 hook exit 1 twice
with the same bytes.

### `make_executable` failing — reached on the previous release only

`make_executable` reads the hook's metadata and sets its mode to 755, after the hook has been
written. For it to fail where the write succeeded, the file must be writable by the user and
not the user's to `chmod`.

- **Tried, candidate: an access-control entry denying `writesecurity` on the user's own hook.**
  Probed on a plain file first: the owner's `chmod` succeeds through that entry on this
  machine, so it plants nothing. Not driven further.
- **Tried, both binaries: the hook a link to `/dev/null`** — a file any user may write and
  only its owner may `chmod`. On the **previous release** this reaches `make_executable`: exit
  1, *Operation not permitted (os error 1)*, the writability sentence, the condition already
  true, the re-run the same bytes. On the **candidate** the link refusal answers first, before
  any write, so `make_executable` is never called; what it prints is in the table above.
- **Not constructible here:** a regular hook file owned by another user and writable through
  its group — a shared checkout. I have one unprivileged user and no way to make another
  user's file.

So for the candidate this cause is **not driven**, and I assert nothing about it from the
binary. From the source alone: its failure maps to the same fixed route sentence as the four
other failure points of that site, and it happens after the hook's bytes are written.

## Triage's two questions, per cause

| cause | does the route name the real cause | does the re-run succeed once the route's act is done |
|---|---|---|
| `make_executable` failing | candidate: **not driven**. Previous release, by a link to `/dev/null`: no | previous release: **no** |
| the hook path is a directory | **no** — it names the hooks directory's writability | **no**: the condition already holds |
| the hook cannot be read | **no** — it names the directory, and the fault is the file's mode | **no**: the condition already holds, and a recursive `chmod u+w` does not help |
| an outward link written relatively, to an existing file | the cause yes, **the path no** — it names the link's end as the link | **no**: exit 1 again, and the act it names deletes a file outside the repository |
| an outward link written relatively, to nothing | yes | yes |
| every refusal under `--force` | as plain, cause by cause | as plain |
| every refusal under `--format json` | as plain: the same `route` string | as plain |

## Does it break the clause, inside its scope

**Reproduces:** from nothing, one fresh rig per cell, on the binary whose hash the prompt
gives, exits read bare.

**Intended?** The refusals are. The design paragraph refuses a hook link that leads out of the
repository, and leaves a directory at a merged-into path to the writer, "which the writer fails
on loudly as before". What is not in any design sentence or decision I found is a route that
names the link's end as the link, or a writability route for a failure that is not about
writability: `command grep` for `install-hook` over `design/`, `implementation/` and
`DECISIONS.md` finds the design paragraph above, one unrelated row of
`design/validation.md`, and decision entries that concern the read-only hooks directory, where
the route is right.

**The clause.** The second measure, *every refusal's route works as printed*, is broken by its
letter in each red cell: a refusal, a route, the route's act done, and exit 1 again.

- **The relative outward link is inside the scope without argument.** A plain checkout; a hook
  shared by a relative link is an ordinary arrangement and the one the design paragraph speaks
  to; `jigc setup` exits 0 there on the previous release; and the candidate's route is the only
  way back to a working `setup`, and as printed it does not lead there and removes a file the
  repository does not hold.
- **The directory and the unreadable hook break the same letter, and I record their weight
  honestly:** a directory at `.git/hooks/pre-commit` and a hook of mode 000 are rare states,
  nearer to planted than to ordinary. The *deliberately planted states* bound is the first
  clause's, the run declares no bound, and the measure says *every refusal's route*; so I do
  not grade them outside it. Were the relative link not here, these two alone would be the
  closer call.

**Why not `contested`.** The finding does not argue that a settled decision is wrong, and
nothing here needs one overturned.

## The regression fact

Per instance, because the two producers differ:

- **`Repro VP-1` (the relative outward link): `regression: true`.** Previous release, called by
  its absolute path on a fresh rig: exit 0, no refusal, no route. Candidate: exit 1, the route
  as printed, exit 1. Green there, red here. What the previous release does at that exit 0 is
  write into the script outside the repository through the link — the cell the candidate's
  refusal exists to close — so the fact is that the *route* is new and wrong, not that the
  refusal should go.
- **`Repro VP-2` (the directory, the unreadable hook): `regression: false`.** Red in the same
  way on both binaries, stderr equal byte for byte.

The return's one `regression` field carries VP-1's `true`, since that is the block the return
names; VP-2's `false` is stated here and in the basis.

## Coverage — verified from the suites, for the blocks' `pinned-by`

The finding makes no coverage claim. By `command grep` over `crates/cli/tests` and
`tooling-tests`:

- The sentence *hooks directory is writable* is in no test: its only file in the tree is
  `crates/cli/src/setup.rs`.
- `replacing_writers_never_follow::a_merged_into_member_refuses_a_link_the_repository_cannot_commit`
  plants the hook as a link to an outside file and to nothing, both by an **absolute** target,
  under `setup` and `setup --force`; asserts exit 1, the code, nothing installed, the outside
  file untouched; then removes `.git/hooks/pre-commit` by a literal of its own and asserts exit
  0. It never reads the path the route prints, and it plants no relative link. The four
  assertions on *remove the link at* in that file are on other members.
- No suite makes `pre-commit` a directory (`create_dir` beside `pre-commit`: no hit), and no
  suite sets a hook's mode to 000 or 200 (the three `0o000` hits are a leftover root, the
  project layer and a pin file). `--format json` is asserted on no `setup.install-hook`
  refusal that these searches reach. A suite that reaches one of these states by another
  spelling would have been missed.

## Repro VP-1 — `jigc setup` over a hook linked relatively to an outside script: the route names the script as the link

```yaml
claim: "where .git/hooks/pre-commit is a RELATIVE link to an existing script outside the repository, `jigc setup` refuses at setup.install-hook and its route says `remove the link at` the path of the script the link leads to; done as printed, the script is deleted and the re-run exits 1 again; `--force` and `--format json` print the same route"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "exit 0, no refusal, no route, in all four modes (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, also called by its absolute path); the outside script is written through the link, 31 -> 5159 bytes"
regression: true
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare                              # dev/jigc-rig bare: one commit, no `jigc setup`; the repository is <rig>/repo
  - write: "<rig>/outside/shared-hook = '#!/bin/sh\necho shared-hook-ran\n', mode 755"   # 31 bytes, beside the repo, in no repository
  - symlink: ".git/hooks/pre-commit -> ../../../outside/shared-hook"                   # relative to .git/hooks
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
  - "argv [\"jigc\", \"setup\", \"--force\"]: the same exits, stderr equal to the plain run's"
  - "argv [\"jigc\", \"setup\", \"--format\", \"json\"] (and with --force): exit 1, stdout empty, stderr one envelope {schema_version: 3, findings: [one]}; findings[0].code = setup.install-hook, .severity = blocking, .route = the plain route string, naming <abs>/outside/shared-hook"
  - "the link written absolutely (the earlier report's block): the same in all four modes"
  - "the relative link to nothing (../../../outside/missing): exit 1, the route names <abs>/repo/.git/hooks/pre-commit, which is the link; removed; re-run exit 0, in all four modes — correct on the candidate; exit 0 on the previous release, which creates the file"
  - "the link to /dev/null: exit 1, the refusal and the route name `/dev/null` as the link; the re-run with nothing done is the same bytes; exit 0 once .git/hooks/pre-commit is removed"
observed: "<W>/cand.matrix.txt and <W>/prev.matrix.txt, the rel-out, rel-dangle, abs-out, abs-dangle and devnull-link cells (rigs 8lHgor, eubdxk, Ju7C1W, w57LJr for rel-out on the candidate; wKMw4w, gMW75c, yBpCRl, QWnS09 on the previous release); <W>/abs.txt, rig CsKcki, the previous release by absolute path"
pinned-by: "UNPINNED for the route's path and for the relative spelling. replacing_writers_never_follow::a_merged_into_member_refuses_a_link_the_repository_cannot_commit pins exit 1, the code, nothing installed and the outside file untouched for an absolute link, then removes .git/hooks/pre-commit by a literal of its own and never reads the path the route prints"
```

**Pinnable as it stands:** yes. Every step is argv or one file-system act; the expectations are
exits, substrings and what is at a path; the one machine-dependent value is the rig's absolute
path. `green-when-fixed` does not assume how the fixed route spells the link, so it serves as
the fix's red test unchanged. It is the earlier report's block with the link's spelling changed
and the two flags added; one test over both spellings would pin both.

## Repro VP-2 — the writability route over a hook path that is a directory, or a hook that cannot be read

```yaml
claim: "`jigc setup` prints `ensure the repo's git hooks directory is writable` where the hook path is a directory or the hook file cannot be read; the directory is writable already, and the re-run fails with the same bytes, under --force and --format json alike"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "the same exits and the same stderr, byte for byte, in all four modes (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, also called by its absolute path)"
regression: false
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare
  - mkdir: ".git/hooks/pre-commit"             # the hook path is an empty directory
repro:
  - ["jigc", "setup"]                          # run 1
  - assert: "the path `git rev-parse --git-path hooks` prints is writable"   # the route's condition, already true
  - ["jigc", "setup"]                          # run 2, the route as printed
expect:
  exit: 1                                      # both runs
  stdout: ""                                   # both runs
  stderr_contains:
    - "blocking · setup.install-hook"
    - "cannot install the `pre-commit` hook into the repo's hooks dir: Is a directory (os error 21)"
    - "route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`"
  stderr_run_2: "byte-identical to run 1"
  then: "`rmdir .git/hooks/pre-commit`, which the route does not name; `jigc setup` exits 0, one install commit, porcelain empty"
variants:
  - cause: "the directory at the hook path holds one file"
    expect: "the same; cleared by removing the file and the directory"
  - cause: "an existing .git/hooks/pre-commit of the user's, mode 000, hooks directory mode 755"
    expect: "exit 1 twice, `Permission denied (os error 13)`; a recursive `chmod u+w` over the hooks directory leaves it at exit 1; cleared by `chmod u+rw` on the hook file; then exit 0 with the user's line kept"
  - cause: "the same hook, mode 200"
    expect: "as mode 000"
  - cause: "argv with --force, with --format json, with both"
    expect: "the same exits; --force stderr equal to plain; json = one findings envelope on stderr, stdout empty, findings[0].route = the plain route string"
  - cause: "the hooks directory itself mode 555"
    expect: "exit 1 `Permission denied (os error 13)`, then `chmod u+w` on it and exit 0, in all four modes — the route is right here"
observed: "<W>/cand.matrix.txt and <W>/prev.matrix.txt, the hookdir, hookdir-full, unreadable, writeonly and ro-hooksdir cells (candidate rigs JCMVoe, iKDrWf, z6CYMH, r5ZDAH for the empty directory; xJ0Fkf, a9EVgo, mP8zxF, VGVFR9 for mode 000); <W>/abs.txt, rigs l3lWQ6 and ZqglAs (previous release by absolute path), t3nFaY and Z9vnqL (candidate, the recursive chmod)"
pinned-by: "UNPINNED: no test asserts this route's text, no suite makes the hook path a directory, and none plants a hook of mode 000 or 200. setup_failed_first_run pins the code under a read-only hooks directory, the variant where the route is right"
```

**Pinnable as it stands:** yes, as a statement of what the binary does today, with one
condition: the mode-000 variants bind only for an unprivileged user, so a test of them must
skip, or assert the mode binds, where it runs as root. **As a fix's red test it needs what the
block cannot give:** what the route is to say for each cause. The `then:` lines say what clears
each one.

## The class

**`instance, unbounded`.** I drove thirteen states on two binaries in four modes (104 cells,
one rig each) and enumerated nothing new: the producers of `setup.install-hook` are the three
the earlier report counted, and I confirmed that count by reading the same file, not by a
search of my own. Not driven: the first construction site (`current_exe` failing);
`resolve_hooks_dir`'s failures, which are another finding's subject; `make_executable` failing
on the candidate; a special file at the hook path (a FIFO there would block the read); any
layout but a plain checkout; any git but 2.54.0; any system but this one. The other callers of
`display_hook_path` were not enumerated.

## Left open — noticed, not pursued

- **`make_executable` failing on the candidate is still undriven.** It needs a hook file the
  user can write and cannot `chmod`: another user's, group-writable, in a shared checkout. It
  wants a machine with two users.
- **On the candidate, a hook linked to `/dev/null` is told to *remove the link at
  `/dev/null`*.** It is the VP-1 mechanism with a device node as the printed path; I did not
  act on that route. Linking a hook to `/dev/null` is a way some users switch a hook off.
- **A write-only or unreadable hook, once made readable, is spliced into and kept.** Observed at
  the third run of those cells; I looked no further.
- **Every fixed-route refusal comes after the install's files are written,** on both binaries
  and in all four modes, and the next successful run commits them. Observed in each such row.
- **`jigc uninstall` prints the same writability sentence** (`crates/cli/src/setup.rs`, line
  5050). Not driven; the earlier report names it too.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports live
in. No commit, no stage, no build. With my file tool I wrote this report and nothing else; the
two driver scripts and the evidence under `<W>` were written by the shell, inside the scratch
root, and two dumps of `--help` text went to the system temp directory.

<!-- end of report -->
