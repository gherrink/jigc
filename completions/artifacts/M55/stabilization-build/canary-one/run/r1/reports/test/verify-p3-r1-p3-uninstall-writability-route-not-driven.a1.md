# verify-real — `r1-p3-uninstall-writability-route-not-driven` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc uninstall`. Clause it is said to break:
`working-product`. Triage's grade: *unclear*. The finding came with no block: it is the first
bullet under *Left open — noticed, not pursued* of the report
`verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md`, which says the teardown
prints the same writability sentence under `uninstall.remove-precommit` and that nobody drove
it. Triage asked for the four causes of that report's `Repro VR-2` and the outward hook link of
its `Repro VR-1` to be driven at `jigc uninstall` after a clean `jigc setup`, on both binaries:
the refusal, its route as printed, the re-run.

## Verdict in one paragraph

**`confirmed`, `regression: false`, `contested: false`.** I was sent to show that the teardown's
route is right, and for one cause it is: a hooks directory that is read-only over the hook jigc
wrote whole is refused with *ensure the repo's git hooks directory is writable*, and the re-run
exits 0 once that is done. It does not survive the rest. **Four of the causes I was handed, and
one more the outward link leads to, draw that same route while the hooks directory is already
writable**: `core.hooksPath` naming a regular file, `core.hooksPath=/dev/null`, a hook that is
read-only and holds a line of the user's beside jigc's block, a hook holding a byte that is not
UTF-8, and a hook linked to a read-only script outside the repository. In each the route's
condition holds before the first run, the re-run exits 1 with the same refusal lines, and the
door recovers only by an act the route does not name. The first run has by then removed `.jigc/`
and the settings entries, and leaves the hook and the guide artifact standing. All of it is the
same on the previous release, the refusal lines byte for byte. **One half of the hypothesis is
refuted:** the wrong-path shape of `Repro VR-1` does not exist at this door — the refusal prints
no path at all, and a hook linked outward draws no refusal unless what it leads to is
unwritable (`Repro VU-2`, below).

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Both hash calls were the plain call the prompt spells, made before the first command was
driven. In every driver run the binary's directory went first on `PATH` and the driver stops
unless `command -v jigc` prints that binary's absolute path; it printed it in all thirty runs.
Nothing was built and nothing under `target/` was driven. The evidence is keyed by the two
hashes above, not by a version string.

## The producer, read before it was driven

`command grep -rn 'remove-precommit'` over `crates`, `tooling-tests` and `dev` has **one** hit:
`crates/cli/src/setup.rs`, line 5048, step 6 of `uninstall`. It maps **any** error of
`remove_precommit_hook` (line 1209) to one fixed route string, *ensure the repo's git hooks
directory is writable, then re-run `jigc uninstall`*, and puts the operating system's own error
after *cannot remove the `pre-commit` hook from the repo's hooks dir:*. That function fails at
four points: `resolve_hooks_dir` (git cannot be run, git exits non-zero, git prints nothing);
`read_to_string` of the hook with any error but *not found*; `remove_file`, where the hook is
jigc's whole; `fs::write`, where something of the user's is in the hook and only jigc's block
comes out. Of those four, only `remove_file` depends on the directory's mode.

Two facts of the function's shape decide what the outward link does here. It reads and writes
**through** a link (`read_to_string`, `fs::write`), and `remove_file` removes the link and not
what it leads to. And the refusal's text names no path: `display_hook_path`, whose following of
links is what `Repro VR-1` turned on, is not called on this path — its one caller in the
teardown is the *left the `pre-commit` hook in place* warning, a different surface.

Step 6 runs **after** steps 1 to 5: `.jigc/` is removed and the settings file is edited before
the hook is asked about, and step 7, the guide artifact, runs after it. So a refusal here leaves
the teardown half done.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-uninstall-hook.N7ahGv`. **One fresh rig per cause and per binary — 32 in
  all**, 16 for each binary — each built by `dev/jigc-rig bare --binary <path>` with
  `SCRATCH=<W>`: a git repository with one commit and no `jigc setup`. None is a reporter's.
- Environment of every run: `HOME` the rig's own empty directory, `GIT_CONFIG_NOSYSTEM=1`,
  `GIT_CONFIG_GLOBAL=/dev/null`, the identity the rig sets in the repository's own config. Git
  2.54.0 (Apple Git-157), macOS, an unprivileged user, so a mode of 555 binds.
- Every exit status was read bare: `jigc <verb> > out 2> err`, then `$?`, never through a pipe.
  Nothing was read through `head`.
- The driver, `<W>/drive.sh`: build the rig; where the cause needs a hook of the user's, write
  a 29-byte one before setup; run `jigc setup` and require exit 0 with an empty porcelain — **a
  clean setup, in every one of the 32 rigs**; plant the cause; run `jigc uninstall`; do what the
  printed route says; run `jigc uninstall` again; and where that is still red, do what actually
  clears the cause and run it a third time. Transcripts: `<W>/cand.<cause>.txt` and
  `<W>/prev.<cause>.txt`; each rig holds `ev/setup.*`, `ev/un1.*`, `ev/un2.*`, `ev/un3.*`.
- For the regression fact the headline block was driven once more on a fresh rig with each
  binary **called by its absolute path**, no `PATH` change (`<W>/abs.sh`; `<W>/prev.abs.txt`,
  `<W>/cand.abs.txt`).
- **What "the route as printed" was taken to mean.** Make the directory
  `git rev-parse --path-format=absolute --git-path hooks` names writable — `chmod u+w` where
  `test -w` says it is not, nothing where it already is — then re-run `jigc uninstall`. Steps
  taken after that to see whether the door recovers at all are marked *not the route*.
- **What I changed against the handed causes, and why.** They were written for `jigc setup`,
  where the cause stands before the install. After a clean setup the hook always holds jigc's
  block, so each cause was planted **after** setup: the knob set, the mode changed, the byte
  appended, the hook moved out and linked back. The candidate refuses `setup` over an outward
  link, so "an outward link after a clean setup" can only be a link made afterwards. Each
  hook-file cause was driven over both forms of hook, the one jigc writes whole (*standalone*)
  and a user's hook with jigc's block spliced in (*wrapped*), because the source takes them down
  two different ways.

## What was observed

Controls: on an untouched rig `jigc setup` then `jigc uninstall` exits 0 on both binaries, over
the standalone hook and over the wrapped one; a second `jigc uninstall` exits 0 with *nothing to
remove*.

### The exits, both binaries

Identical on the candidate and on the previous release in every row.

| cause, planted after a clean `jigc setup` | first `jigc uninstall` | the route as printed | re-run | what clears it — *not the route* | then |
|---|---|---|---|---|---|
| hooks directory mode 555, standalone hook | exit 1, *Permission denied (os error 13)* | `chmod u+w` on it | **exit 0**, hook gone | - | - |
| hooks directory mode 555, wrapped hook | exit 0 — the block is cut out in place, the user's 29 bytes restored | - | - | - | - |
| `core.hooksPath` = a regular file outside the repository, mode 644 | exit 1, *Not a directory (os error 20)* | nothing to change: the path is writable | **exit 1** | `git config --unset core.hooksPath` | exit 0 |
| `core.hooksPath=/dev/null` | exit 1, *Not a directory (os error 20)* | nothing to change: the path is writable | **exit 1** | `git config --unset core.hooksPath` | exit 0 |
| hook file mode 555, standalone, hooks directory 755 | exit 0 — the file is removed | - | - | - | - |
| hook file mode 555, wrapped, hooks directory 755 | exit 1, *Permission denied (os error 13)* | nothing to change: the directory is writable | **exit 1** | `chmod u+w` on the hook **file** | exit 0, the user's 29 bytes restored |
| one line with a byte that is not UTF-8 appended, standalone | exit 1, *stream did not contain valid UTF-8* | nothing to change | **exit 1** | cut the line out again | exit 0 |
| the same, wrapped | exit 1, *stream did not contain valid UTF-8* | nothing to change | **exit 1** | cut the line out again | exit 0 |
| hook moved outside and linked back, standalone | exit 0 — the link removed, the script outside byte-identical | - | - | - | - |
| hook moved outside and linked back, wrapped | exit 0 — the link kept, the script outside written through it: 5141 to 29 bytes, the user's lines (5157 to 29 on the previous release) | - | - | - | - |
| the same, the script outside mode 555 | exit 1, *Permission denied (os error 13)* | nothing to change: the directory is writable | **exit 1** | `chmod u+w` on the script outside | exit 0, written through the link |
| hook replaced by a link to a 31-byte script of the user's outside | exit 0 — nothing of jigc's in it, link and script untouched, no hook bullet in the summary | - | - | - | - |
| hook replaced by a link to nothing | exit 0 — link untouched, nothing created | - | - | - | - |

### The refusal, as printed

stderr of the re-run under `core.hooksPath=/dev/null`, whole, both binaries (`cmp` exit 0
between them):

```text
blocking · uninstall.remove-precommit — cannot remove the `pre-commit` hook from the repo's hooks dir: Not a directory (os error 20)
  route: ensure the repo's git hooks directory is writable, then re-run `jigc uninstall`
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

For each of the six red-twice rows: the re-run's stderr is byte-identical between the two
binaries (`cmp` exit 0; 298, 298, 300, 303, 303 and 300 bytes), and the refusal's two lines are
byte-identical between the first run and the re-run. The first run's stderr is longer only by
the warning that removing `.jigc/` also removes the tracked files under it, which stands above
the refusal and is not repeated because the tree is gone by the second run. stdout is empty at
every exit 1.

### What the first refusal leaves

Read after the first exit 1, in each of the seven rows that refuse, both binaries: `.jigc/` is
gone; the porcelain has eight lines on the candidate and seven on the previous release, which
commits one install file fewer; the hook is byte-identical to what was planted. The run that
finally exits 0 prints two bullets in each of those rows, *removed pre-commit hook* and
*removed jigc guide artifact* — the guide artifact outlives the first refusal, read directly at
`.claude/skills/jigc/SKILL.md` in the two rigs driven by absolute path. Under the two
`core.hooksPath` causes the hook jigc installed stands at `.git/hooks/pre-commit` throughout,
because the teardown looks only where git points now; it is removed once the knob is unset.

## Is it what the finding says

The finding says the teardown prints the writability sentence from the same resolution and the
same read, failing the same ways. Driven, that holds with three corrections, each a fact of this
door and none of them the reporter's:

1. **The operating system's error differs.** A `core.hooksPath` that names a file fails
   `setup` at `create_dir_all` with *File exists (os error 17)*; it fails `uninstall` at the
   read with *Not a directory (os error 20)*. The route is the same sentence.
2. **A read-only hook file is a cause only over a wrapped hook.** The standalone hook is
   removed by an act on the directory and goes at exit 0. And the read-only **directory**, the
   one cause the route names, is a cause only over a standalone hook: a wrapped one is rewritten
   in place at exit 0. So the route is right in exactly one of the two cells the sentence
   suggests.
3. **The outward link is not `Repro VR-1` here.** See `Repro VU-2`.

## What the record holds

**The design section that owns the behaviour** — `design/project-setup.md` → *Teardown / cleanup
(G5)*: `jigc uninstall` reverses every setup-created repo-local artifact, the last of them
"prune the git `pre-commit` hook"; "Pruned means jigc's own lines, and only those"; the
acceptance is "idempotency + non-destructive". `design/assistant-adapter.md` has the install
resolve the real hooks directory, "honoring `core.hooksPath`", and the teardown resolves it the
same way. Neither document names the code `uninstall.remove-precommit`, says what its route is
to say, or says anything of a hooks path that is not a directory, a hook that cannot be read as
text, or a hook file that cannot be written: `command grep` for `remove-precommit` and for
*hooks directory is writable* over `DECISIONS.md`, `implementation/` and `design/` has no hit.
So no settled decision intends a writability route for a cause that is not writability, and
none is argued against by this finding.

**`design/surface-contract.md` → The three laws.** Law 2: "Every affordance that is the
designated recovery for a state is named by the surfaces that produce that state". In the six
red-twice rows the recovery — unset the knob, make the hook file writable, take the byte out —
is named by no surface.

**The clause.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, the second clause's
measure: "no command that works on rc.24 in a supported layout stops working, and every
refusal's route works as printed". `implementation/decisions-pending.md` → *The exit rule*, of
the regression set's two parts, says where a route of this kind is held: "a refusal's route
that is a sentence for a human is run by neither; that stays with the review rows, which run
every printed route". So a route that is a sentence is inside the measure, and running it as
printed is the test.

## Does it break the clause, inside its scope

**The second measure — broken.** A refusal's route, done as printed, ends at the same code and
exit 1. In all six rows there is nothing to do, because the route's condition is already true;
and under `/dev/null` and under the regular file, the thing the route calls a directory is none.

**The scope.** The layout is one plain checkout — no worktree, no bare store, no submodule — so
the measure's *supported layout* is not in question. What varies is the user's own hook
arrangement after setup: a knob the design says is honoured, a file's mode, a file's encoding.
`-c core.hooksPath=/dev/null` is the spelling five of this repository's own suite files use to
run git with hooks off (`crates/cli/tests/home_vacated.rs` among them). The *deliberately
planted states* bound is written into the first clause's scope; I found no decision that carries
it to the second, and a refusal exists for the states that are not the happy path.

**The reach, stated so it is not taken for more.** Each cause is something a user did to their
hook or their git config after `jigc setup`; none arises from jigc's own writes. No bytes are
lost in any row: at every exit 1 the hook is byte-identical, and what the first run removed is
what the summary of a green teardown names. The first measure is untouched — nothing that
worked on the previous release stops working.

**Why `confirmed`.** *Does not reproduce:* it reproduces from nothing, each cause on its own
fresh rig, twice per binary for the headline block. *Intended:* no decision or design sentence
gives this route for these causes. *Breaks no clause:* it is the letter of the second measure in
a plain checkout. **Why not `contested`.** The finding argues against no settled decision.

## The regression fact

**`regression: false`.** The headline block, on a fresh rig, with the previous release's binary
called by its absolute path: `jigc setup` exit 0, `jigc uninstall` exit 1, the route's condition
already true, `jigc uninstall` exit 1 with the same three lines; then exit 0 once
`core.hooksPath` is unset. The candidate, the same way: the same exits and the same bytes. Red
on both, and red the same way in all six rows. The door and the code existed on the previous
release, so the block is comparable.

## Coverage — verified from the suites, for the blocks' `pinned-by`

The finding makes no coverage claim. For the blocks below, by `command grep` over `crates` and
`tooling-tests`:

- The code `uninstall.remove-precommit` is spelled in **no** test: its one hit in the tree is
  the producer.
- The sentence *hooks directory is writable* is asserted by **no** test: its two hits are the
  two producers in `crates/cli/src/setup.rs` (lines 2395 and 5050).
- Every call of `remove_precommit_hook` in the unit tests beside it
  (`remove_precommit_removes_standalone_jigc_hook_then_is_a_no_op`,
  `remove_precommit_restores_wrapped_foreign_hook`, `remove_precommit_leaves_foreign_hook_untouched`
  and their neighbours) is followed by `.expect(...)`; none drives an error out of it.
- Eight suite files that mention `uninstall` also mention a hooks path, a read-only mode or a
  byte pattern; I did not read each, and a suite that reaches one of these states by another
  spelling without naming the code would have been missed. That no test names the code or the
  sentence is the enumerated fact.

## Repro VU-1 — `jigc uninstall` with `core.hooksPath=/dev/null` set after a clean setup: the writability route where its condition already holds

```yaml
claim: "`jigc uninstall` prints `ensure the repo's git hooks directory is writable` for failures that are not about that directory's writability; the condition already holds and the re-run fails with the same refusal"
verdict: CONFIRMED
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "the same exits and the same refusal bytes (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, called by its absolute path)"
regression: false
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare                              # dev/jigc-rig bare: one commit, no `jigc setup`
  - ["jigc", "setup"]                          # exit 0, porcelain empty: a clean install
  - ["git", "config", "core.hooksPath", "/dev/null"]
repro:
  - ["jigc", "uninstall"]                      # run 1
  - assert: "the path `git rev-parse --path-format=absolute --git-path hooks` prints is writable"   # the route's condition, already true
  - ["jigc", "uninstall"]                      # run 2, the route as printed
expect:
  exit: 1                                      # both runs
  stdout: ""                                   # both runs
  stderr_contains:                             # both runs
    - "blocking · uninstall.remove-precommit"
    - "cannot remove the `pre-commit` hook from the repo's hooks dir: Not a directory (os error 20)"
    - "route: ensure the repo's git hooks directory is writable, then re-run `jigc uninstall`"
  stderr_run_2: "exactly the refusal line, the route line and the footer; run 1 carries the `.jigc/` tracked-files warning above them"
  files_after_run_1: ".jigc absent; .git/hooks/pre-commit byte-identical to what setup wrote; .claude/skills/jigc/SKILL.md present; porcelain 8 lines (7 on the previous release)"
  then: "`git config --unset core.hooksPath`, which the route does not name; `jigc uninstall` exits 0 and its summary has two bullets, the hook and the guide artifact"
green-when-fixed: "the act run 1's route names, done as it names it, is followed by a `jigc uninstall` that exits 0 — or run 1 does not refuse at all"
variants:
  - cause: "core.hooksPath = a regular file outside the repository, mode 644"
    expect: "the same, `Not a directory (os error 20)`; cleared by unsetting the knob"
  - cause: "a user's 29-byte hook written before setup (so setup splices its block in), then `chmod 555` on the hook file; hooks directory mode 755"
    expect: "exit 1 twice, `Permission denied (os error 13)`; cleared by `chmod u+w` on the hook file; then exit 0 and the hook is the user's 29 bytes again"
  - cause: "one line holding a byte that is not UTF-8 appended to the hook after setup, over the hook jigc wrote whole and over a spliced one"
    expect: "exit 1 twice, `stream did not contain valid UTF-8`, the hook byte-identical; cleared by taking the line out"
  - cause: "the spliced hook moved to a path outside the repository, mode 555, and .git/hooks/pre-commit a link to it"
    expect: "exit 1 twice, `Permission denied (os error 13)`; cleared by `chmod u+w` on the script outside; then exit 0, the script written through the link"
  - cause: "the hooks directory itself mode 555 over the hook jigc wrote whole"
    expect: "exit 1 `Permission denied (os error 13)`, then `chmod u+w` on it and exit 0 — the route is right here"
  - cause: "the hooks directory mode 555 over a spliced hook; or the hook file mode 555 over the hook jigc wrote whole"
    expect: "exit 0 at the first run — no refusal"
observed: "candidate <W>/jigc-rig-bare-xTXbdu and ovIOKT (/dev/null), vLfSZu, uazsrU, QxR2Xu, XEvNGr, Krwmo3, KDXVH6, B4KbqS, BQtsr0; previous release 45qizG and 9QaiG2 (/dev/null), L6fcNQ, MJHXXd, EK7Qjm, VkJMlu, 6jZln3, kNmsFq, GghUNm, Vdrulm; <W>/cand.*.txt, <W>/prev.*.txt"
pinned-by: "UNPINNED: no test spells the code `uninstall.remove-precommit` or asserts this route's text, and the unit tests of remove_precommit_hook drive no error arm"
```

**Pinnable as it stands:** yes, as a statement of what the binary does today — every step is
argv or one file-system act, and the expectations are exits, substrings and what is at a path;
nothing in it depends on the machine but the rig's root. **As a fix's red test it needs one
thing the block cannot give:** what the route is to say for each cause. `green-when-fixed` is
written without assuming that wording.

## Repro VU-2 — the outward hook link at `jigc uninstall`: no refusal, and no path printed

```yaml
claim: "where .git/hooks/pre-commit is a link to a script outside the repository, `jigc uninstall` refuses at uninstall.remove-precommit with a route that names the wrong path, as `jigc setup` does in Repro VR-1"
verdict: REFUTED                               # does-not-reproduce, for this shape at this door
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "the same exits (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare
  - ["jigc", "setup"]                          # exit 0, porcelain empty
  - move: ".git/hooks/pre-commit -> <rig>/outside/shared-hook"     # beside the repo, in no repository
  - symlink: ".git/hooks/pre-commit -> <abs>/outside/shared-hook"
repro:
  - ["jigc", "uninstall"]
expect:
  exit: 0
  stderr_not_contains:
    - "uninstall.remove-precommit"
    - "outside/shared-hook"
  stdout_contains:
    - "removed pre-commit hook"
  files: ".git/hooks/pre-commit no longer exists; outside/shared-hook byte-identical"
variants:
  - cause: "a user's 29-byte hook before setup, then the spliced hook moved out and linked back"
    expect: "exit 0; the link kept; outside/shared-hook is the user's 29 bytes again — jigc's block cut out through the link"
  - cause: "the hook replaced by a link to a 31-byte script of the user's outside, nothing of jigc's in it"
    expect: "exit 0; link and script untouched; the summary has no hook bullet"
  - cause: "the hook replaced by a link to nothing"
    expect: "exit 0; link untouched; nothing created where it points"
  - cause: "the spliced hook moved out, mode 555, and linked back"
    expect: "exit 1 at uninstall.remove-precommit — the shape of Repro VU-1, not of VR-1: the refusal names no path"
observed: "candidate <W>/jigc-rig-bare-bl9SCK, ZdwiU3, oyPrf9, XdikJe, Krwmo3; previous release IxLVvv, EpHb73, 8MU086, 0L4s4S, 6jZln3"
pinned-by: "UNPINNED as far as the two searches above reach: no suite found that drives `jigc uninstall` over a hook that is a link"
```

**Pinnable as it stands:** yes. It is the block of a behaviour that holds; whether the first
variant — a file outside the repository written through a link at exit 0 — is the behaviour
wanted is not this finding's question and is left open below.

## The class

**Producers: enumerated — one**, by `command grep -rn 'remove-precommit'` over `crates`,
`tooling-tests` and `dev` (`crates/cli/src/setup.rs`, line 5048), over the four failure points
of `remove_precommit_hook` read off the source. **Causes: `instance, unbounded`.** On two
binaries I drove fifteen states: two controls; seven that reach the refusal (one at
`remove_file`, four at `read_to_string`, two at `fs::write`) and six that reach none. Not
driven: the three `resolve_hooks_dir` failures; a hook that is a directory, or unreadable; a
link written relatively; `--force`; `--format json`; a linked worktree; a hook written by one
binary and taken down by the other.

## Left open — noticed, not pursued

- **The six other teardown refusals carry a route of the same shape** — `uninstall.remove-jigc`,
  `.unwire-reference`, `.remove-allowlist`, `.remove-hook`, `.remove-deny`, `.remove-guide`
  (`crates/cli/src/setup.rs`, lines 4967 to 5088): any error of the step, then *ensure `<x>` is
  writable*. Read, not driven.
- **A refusal at step 6 leaves the teardown half done at exit 1**: `.jigc/` and the settings
  entries gone, the hook and the guide artifact standing. Observed in every row that refuses, on
  both binaries; I looked no further — in particular not at what the removal of the settings
  record in step 1 means for a re-run.
- **`jigc uninstall` writes through a hook that is a link to a file outside the repository**, at
  exit 0, under *removed pre-commit hook* (`Repro VU-2`, first variant). What it wrote there is
  the user's own lines with jigc's block cut out, so I saw no bytes lost; the install side
  refuses to follow such a link, and whether the teardown should is nobody's ruling that I
  found.
- **Where `core.hooksPath` changed after setup, the teardown never looks at the hook jigc
  installed.** Seen under the two refusing values, where the hook stood at `.git/hooks/pre-commit`
  through both red runs. A knob that names another existing directory was not driven; by the
  source that is exit 0 with the hook left behind and nothing said.
- Where the standalone hook is moved out and linked back, the teardown removes the link at exit
  0 and the script outside keeps jigc's block, under *removed pre-commit hook*.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports live
in. No commit, no stage, no build. With my file tool I wrote this report and nothing else; the
four driver scripts and the evidence under `<W>` were written by the shell, inside the scratch
root, and three dumps of `--help` text went to the system temp directory.

<!-- end of report -->
