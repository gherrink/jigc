# verify-real — `r1-p4-unreadable-entry-class-linux-root-remaining-cells-not-driven` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-unreadable-entry-class-linux-root-remaining-cells-not-driven`. One finding, handed over
by triage with the grade *unclear*: door *unlisted — every door an unreadable `*.md` at a managed home
reaches*, the clause it is said to break `working-product`, and for a repro block *Left open, item 6* of
`verify-p3-r1-p3-unreadable-entry-class-door-list-traced-by-hand.a1.md` — one sentence, **"Linux and root —
still driven by nobody for this class"**, and no block.

## Status: HALTED — no verdict

- **status:** `halted`. **No verdict is returned, and none is implied by anything below.** The finding is
  neither confirmed nor refuted; it stays unverified.
- **What stopped me, in one line:** the finding *is* a platform — its cells are the class's doors **on Linux**
  and **as uid 0** — and the `BINARY:` line hands me two Mach-O arm64 executables and no trial image, in a
  session that is uid 501 on macOS with no way to uid 0. The cells can be neither driven with what I was
  handed nor reconstructed on this machine, because the platform is the cell.
- **Where:** at step 1 (*re-drive it from nothing*), after all three binary checks had passed. This is **not**
  a hash or a path failure — both hashes hold, `PATH` holds, the rig builds on the handed candidate, and the
  class's macOS cells re-drive from nothing (below).
- **What it is not.** It is not `does-not-reproduce`: nothing of the finding's was driven, so nothing failed
  to reproduce. It is not `breaks-no-clause`: whether the Linux and uid-0 cells break `working-product` is the
  question triage sent, and it needs those cells. A finding that could not be driven is not refuted.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed one line of JSON with
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (`1.0.0-rc.24`). **The previous release was hashed and never driven**: step 4 goes with
  `confirmed` only, and there is no verdict.
- With `<scratch>/bin/c1.a2` first on `PATH`, `command -v jigc` printed `<scratch>/bin/c1.a2/jigc`. The driver
  holds that again before each rig and holds the rig's `$JIGC` to the same file (it exits 97 otherwise; it
  never did). No `cargo build`; nothing under `target/` was driven; no container was started.

**What I read.** The one report my prompt handed me, whole. The run's opening, and the run's state through
`dev/stabilize-record state --run canary-one`. Of the ledger and of triage's table, the one row a search for
this finding's key returned. `DECISIONS.md` → *2026-10-04 — The exit rule, revised*. No other verifier's
report: the ledger row names a second source (`verify-p3-r1-p3-unreadable-entry-undriven-platform-shapes-and-doors`,
left open 10) which my prompt did not hand me and which I did not open.

## The finding, and why it cannot be driven from what I was handed

The finding names no behaviour. It names a set of cells nobody drove: the doors of the unreadable-entry class
— the handed report's `Repro V-3`, `Repro V-3b` and `Repro V-3c`, and the fifteen doors of the hand trace it
confirms — over the dangling link and the mode-000 file, **on Linux** and **as root**. The handed report's own
platform line is *macOS only … a user that is not root. Nothing was driven on Linux, and a mode-000 file
refuses nobody who may read any file.*

**The Linux half.** What the `BINARY:` line hands over, read with `file`:

```
<scratch>/bin/c1.a2/jigc:                 Mach-O 64-bit executable arm64
<scratch>/bin/previous-91834b5e011d/jigc: Mach-O 64-bit executable arm64
```

`<scratch>/bin/` holds `c1.a1`, `c1.a2` and `previous-91834b5e011d`, and nothing else. A Mach-O executable is
not loadable by a Linux kernel, so the handed candidate cannot be driven on Linux at all, in a container or
otherwise. A Linux build of commit eeffe347 would be a binary I built or a binary nobody handed me, and the
contract of 2026-10-05 forbids both: *you drive the binary you are handed, never one you build*.

**The root half.** `id -u` prints `501`. `sudo -n true` exits 1 with `sudo: a password is required`. There is
no uid 0 on this machine for this session, and the only other uid 0 within reach is inside a Linux container
— the Linux half again.

**Why no reconstruction reaches the same door.** Step 1 allows the smallest setup that reaches the same door.
Here the door is not what is missing — the handed report's blocks reach every door, and they run as written
(below). What is missing is the kernel and the uid the finding is about. A macOS, non-root drive of the same
argv is the cell the handed report already holds, not a reconstruction of this one.

**What is on this machine and was not touched.** `docker images` lists `jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`
(image id `d37422f98361`, made about eight hours before this report) and `jigc-gate:registry-1.0.0-rc.24`
(`cead273d9c68`, four days old). The first is tagged with the candidate's commit and looks like the trial
image the round's second preflight builds for the trial arm — a likeness I did not check. **I ran neither.**
My `BINARY:` line names no trial image, no hash of the binary inside either was handed to me, and *driven on
another binary* is one of the faults this role is sent to find in a repro — a verdict of mine resting on one
would have it.

## What was driven — the macOS baseline, on the handed candidate only

Driven so that the halt is not mistaken for an environment that cannot drive, and so that whoever runs the
Linux and uid-0 cells has this session's own numbers beside them. **It is no part of a verdict.**

One directory of this reporter's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p1-linux-root-cells.RwRMRc` (written `<W>`). Four rigs, each `dev/jigc-rig refs-post-hoc
--binary <scratch>/bin/c1.a2/jigc` with `SCRATCH=<W>/roots`, stdout captured alone, the build's status read
before the `eval`. `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a synthetic
identity, stdin from `/dev/null`; stdout and stderr each to a file, every exit status read from the bare
command. Platform: macOS on arm64, uid 501.

The plants: **N** none · **L** `ln -s nowhere-target docs/research/dangling.md` · **M**
`docs/research/locked.md` holding `# x`, then `chmod 000` (`cat` of it: exit 1, *Permission denied*) · **R**
the same file left readable, mode 644.

| step, in this order, one rig per plant | N | L | M | R |
|---|---|---|---|---|
| `jigc doc list` | 0 | 1 | 1 | 0 |
| `jigc validate` | 0 | 0 | 1 | 1 |
| `git mv docs/research/context-loss.md docs/research/renamed-loss.md` | 0 | 0 | 0 | 0 |
| `jigc validate --format json` — bytes on stdout | 1 — 2,428 | 1 — 2,428 | 1 — **0** | 1 — 3,470 |
| `git commit -m "bare rename of a managed doc"`, through the hook `jigc setup` installed | **1** | **1** | **0** | **1** |
| `jigc validate` | 1 | 1 | 1 | 1 |

- **N, L and R**: the commit is refused, `HEAD` does not move, stderr is the hook's one line — *jigc:
  out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use
  `jigc rename` instead (commit blocked).*
- **M**: the commit is made — `rename docs/research/{context-loss.md => renamed-loss.md} (100%)`, stderr
  empty; the sweep before and after it exits 1 with `validating the committed store at "<root>/repo":
  Permission denied (os error 13)` and nothing on stdout.
- **L**, `jigc doc list`: exit 1, `reading the committed doc at "<root>/repo/docs/research/dangling.md": No
  such file or directory (os error 2)`. **M**: the same sentence with `Permission denied (os error 13)`.
- **R**, `jigc doc list`: exit 0, one more row — `research:locked  docs/research/locked.md  unregistered`.
  `jigc validate`: exit 1 under `schema-conformance.unadopted-instance`, with an adoption route that I did
  not run.
- Each plant was in place afterwards with the kind, mode and size it had: the link `-> nowhere-target`, the
  file mode 000 and 4 bytes, the file mode 644 and 4 bytes.

This agrees with the handed report's `Repro V-3b` on the same binary hash, cell for cell, for N, L and M.

**What R is, and is not.** A process with uid 0 on Linux reads a mode-000 file. If that is the only thing uid
0 changes, the M plant under root is a readable foreign file at a managed home whose mode bits are 000, and
the nearest cell this machine can drive is R — where the hook's block holds. **That is an inference about a
kernel and not a drive of the cell**: R differs from the root cell in the file's mode bits, in the owner of
every file of the rig, in the kernel, and in whatever git and jigc do with a mode-000 file they can read
(adopting it, staging it, moving it). I return it as a number beside the halt and as nothing more.

## The coverage claim inside the finding — *driven by nobody*

Verified from the suites and the workflow files, at eeffe347, as far as a read goes:

- **Where read permission is taken to nothing.** `0o000` or `from_mode(0)` is spelled in three suites and one
  source file: `crates/cli/tests/leftover_probe_fail_closed.rs:971` (the directory `.jigc/worktrees`),
  `crates/cli/tests/unreadable_project_layer.rs:127` (the directory `.jigc`),
  `crates/cli/tests/repo_relative_paths.rs:632` (a task's base pin) and `crates/cli/src/task.rs:11158` (a unit
  test's sealed path). **None is a `*.md` at a doctype's home.** That is narrower than the handed report's
  enumeration (it also followed the helpers that take a mode as an argument and found six kinds of target);
  it does not contradict it.
- **Root is known to the suites as a hazard, at one place.** `crates/cli/tests/unreadable_project_layer.rs`
  asserts that its fixture really is unreadable and names the cause when it is not — *running as root?* —
  because *as root the mode change succeeds and the traversal still resolves*. No suite holds what a door of
  this class does when that happens.
- **Linux, non-root, is where CI runs every suite**: every job of `.github/workflows/ci.yml` is
  `runs-on: ubuntu-latest`, and `dev/runner-faithful` runs as `runner`, uid 1000. So whatever a suite holds
  of the dangling-link half is held on Linux by CI. **Which cells that is was not established**: I did not
  read `crates/cli/tests/home_shape_one_code.rs` against the class's door list.

So the sentence the finding rests on stands as far as it could be checked: no suite, and no report I was
handed, holds a mode-000 `*.md` at a managed home on any platform, or any door of this class as uid 0.

## The clause

`working-product` is in the run's closing condition — the opening's second clause, by reference to
`DECISIONS.md` → *2026-10-04 — The exit rule, revised*: *"We have a working product others can use and relay
on"*, instrument *"no command that works on rc.24 in a supported layout stops working, and every refusal's
route works as printed"*. The opening declares no bound. The clause being there is why this is a halt of
step 1 and not a halt over the clause. Whether a tree driven as uid 0, or a tree holding a file its user
cannot read, is *a supported layout* the rule does not say; I did not lean on it, and it is not mine to say.

## The blocks in waiting

Nothing new is needed to drive this finding: the handed report's `Repro V-3`, `Repro V-3b` and `Repro V-3c`
are the cells, in the pipeline's schema, with their macOS expectations on both binaries. The one this session
re-drove, with the readable control added, is below — **a block with no verdict**, left so that the next
drive does not start from prose.

### Repro H-1 — the baseline of the cells that were not driven

```yaml
claim: "the doors of the unreadable-entry class were driven on macOS as a non-root user only; on Linux, and as uid 0, nobody has driven them (ledger key r1-p4-unreadable-entry-class-linux-root-remaining-cells-not-driven)"
verdict: NONE               # halted — the handed binaries are Mach-O arm64 and the session is uid 501; nothing below is a verdict
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
platform: "DRIVEN: macOS arm64, uid 501. NOT DRIVEN: Linux (any uid), and uid 0 (any kernel) — the cells the finding is about"
setup:            # HOME the rig's own; GIT_CONFIG_GLOBAL=/dev/null; GIT_CONFIG_NOSYSTEM=1
  - fixture: refs-post-hoc                 # `jigc setup` has installed .git/hooks/pre-commit, naming the binary by path
  - ["sh", "-c", "printf '# x\\n' > docs/research/locked.md && chmod 000 docs/research/locked.md"]
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "validate"]
  - ["git", "mv", "docs/research/context-loss.md", "docs/research/renamed-loss.md"]
  - ["jigc", "validate", "--format", "json"]
  - ["git", "commit", "-m", "bare rename of a managed doc"]
  - ["jigc", "validate"]
expect:           # macOS, uid 501 — observed here; the Linux and uid-0 columns are blank because nobody has them
  - exit: 1
    stdout: ""
    stderr_contains: "docs/research/locked.md\": Permission denied (os error 13)"
  - exit: 1
    stdout: ""
    stderr_contains: "validating the committed store at"
  - exit: 0
  - exit: 1
    stdout: ""
    stderr_contains: "\"error\": \"validating the committed store at"
  - exit: 0                                # the commit is made: the hook reads the empty report as nothing to say
    stdout_contains: "rename docs/research/{context-loss.md => renamed-loss.md} (100%)"
    stderr: ""
  - exit: 1
    stderr_contains: "Permission denied (os error 13)"
controls:
  - "no plant: exits 0, 0, 0, 1 (2,428 bytes of envelope), 1 (`… (commit blocked).`, HEAD unchanged), 1"
  - "`ln -s nowhere-target docs/research/dangling.md` in place of the file: 1 (`No such file or directory (os error 2)`), 0, 0, 1 (2,428 bytes), 1 (commit blocked), 1"
  - "the file left readable (no chmod): 0 (a row `research:locked … unregistered`), 1 (`schema-conformance.unadopted-instance`), 0, 1 (3,470 bytes), 1 (commit blocked), 1"
to-drive-the-finding: "this block, and Repro V-3, V-3b and V-3c of verify-p3-r1-p3-unreadable-entry-class-door-list-traced-by-hand.a1.md, as written, with a Linux build of commit eeffe347 whose sha256 the BINARY line gives — once as a non-root user and once as uid 0 — and, for the regression fact, with a Linux build of 1.0.0-rc.24. As uid 0 the setup must first be checked against the state it wants: `cat docs/research/locked.md` exits 0 there, so the plant is not unreadable, and the block's first assertion is of that fact"
observed: "<W>/out/{N,L,M,R}/NN-<label>.{argv,rc,stdout,stderr,head}, with root, plant.before, plant.cat, plant.after, final.log, final.porcelain"
pinned-by: "UNPINNED: no suite takes read permission from a `*.md` at a doctype's home (three suites and one source file spell mode 000; their targets are two `.jigc` directories, a base pin and a unit test's path)"
```

**Pinnable as it stands: no — and not because of its form.** The argv, the fixture and the assertions are
literal and it converts as it is; but what it asserts is the macOS non-root column, which the handed report's
`Repro V-3b` already carries. The columns this finding is about are empty. Two things a converter must keep
once they are filled: the commit has to go through the hook the rig's own `jigc setup` wrote, with `jigc`
resolvable at the path that hook names; and a mode-000 plant must be asserted unreadable before any arm runs,
as `unreadable_project_layer.rs` does, or the uid-0 arm passes over a readable file without saying so.

## What was driven, and what was not

- **Driven:** the three binary checks; one rig state, four plants, six steps each, on the handed candidate, on
  macOS as uid 501 — 24 invocations, each exit status read bare. A read of the suites for mode-000 targets.
- **Not driven:** every cell of the finding — the class's doors on Linux as a non-root user, on Linux as uid
  0, and on macOS as uid 0. The previous release, on any platform. Any container. The adoption route
  `jigc validate` prints over the readable file. The handed report's `Repro V-3` and `Repro V-3c` (they were
  driven by their reporter on this same candidate hash; I re-drove `V-3b` only).
- **Class:** `instance, unbounded`. I enumerated nothing; the handed report's count (15 leaf verbs behind the
  16 sites, 22 beside them, two hooks) is its own.
- **Nothing was fixed, graded or decided.**

## The halt

- **root_cause:** The finding's cells are a kernel and a uid — the unreadable-entry class's doors on Linux
  and as uid 0. The `BINARY:` line hands two Mach-O arm64 executables and no trial image; the session is uid
  501 on macOS and `sudo` asks for a password. The handed binary cannot run on Linux, I may build none, and
  no setup on this machine reaches the same cell.
- **evidence:** `file <scratch>/bin/c1.a2/jigc` → `Mach-O 64-bit executable arm64` (the same for
  `<scratch>/bin/previous-91834b5e011d/jigc`); `id -u` → `501`; `sudo -n true` → exit 1, `sudo: a password is
  required`; the `BINARY:` line as handed names `candidate … (commit eeffe347…, label c1); previous release …
  (version 1.0.0-rc.24)` and nothing else. Both `dev/stabilize-step hash` calls returned the hashes the line
  gives; `command -v jigc` returned the candidate's path.
- **tree_state:** branch `fix/canary-one`, `HEAD` 126a85311a54e73e9f9798f3d034d24f14e3ea7c (*docs(record):
  canary-one r1 - the record of the test stage*, on eeffe347, the candidate). `git status --porcelain` shows
  untracked files under `completions/artifacts/canary-one/r1/reports/test/` and nothing else, and **none of
  them is mine**: the stage's `attempt.a2.md`, `preflight.a2.md` and `triage-p1.a2.md` when I started, and
  three reports of other verifiers of this attempt beside them at my last read, before this report was handed
  over. Others may have landed since; the count is the stage's and moves while its verifiers run. I built,
  edited, staged and committed nothing; no commit of mine landed.
- **recommendation:** This is the human's, one of two. **(a) Rule the platform as a bound** — the two sibling
  rows of this class at `jigc doc list` (`r1-doc-list-unreadable-entry-linux-not-driven`,
  `r1-doc-list-staged-arm-linux-and-root-not-driven`) already stand on the human's list as `needs-bound`, and
  this row asks the same question of the other doors. **(b) Grant one more attempt whose `BINARY:` line
  hands a Linux build** of commit eeffe347 with its sha256 — the trial image the second preflight verifies
  would do, and an image tagged with that commit is on this machine — together with a Linux build of
  `1.0.0-rc.24` for the regression fact, and leave to the verifier a container run once as a non-root user and
  once as uid 0. The blocks to run exist: `Repro H-1` above and the handed report's `V-3`, `V-3b`, `V-3c`.
  **And the same is likely to stop every verifier of the same shape:** five more rows in this round's
  `awaiting` list carry *linux* or *root … not driven* in their keys, and if their `BINARY:` lines are shaped
  as mine is, each is handed the same two Mach-O files.

## Left open — seen on the way, not pursued

Nothing new. The one effect seen — the hook's rename block lost under the mode-000 file — is the handed
report's *Left open* 1, reproduced here on the same binary hash and not pursued.

## Where the evidence is

`<W>` is `<scratch>/verify-p1-linux-root-cells.RwRMRc`. The driver: `<W>/tools/drive.sh` (written through the
shell). Per plant: `<W>/out/<N, L, M or R>/`. The rigs: `<W>/roots/`. Nothing was torn down, and the one file
written with the file tool is this report.

<!-- end of report -->
