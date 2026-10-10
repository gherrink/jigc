# verify-real — `r1-p3-start-orientation-host-temp-path-findings-unknown`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding: door `jigc start`, the
clause it is said to break `working-product`, triage's grade *unclear*.

## Verdict in one paragraph

**REFUTED as a blocker — `breaks-no-clause`.** The behaviour is real and reproduces as the
finding states it: with one index entry whose name is not UTF-8 standing, bare `jigc
start` exits 0 and its active-task block reads `findings: unknown — …`, the reason being
the failed `git checkout-index -a --prefix=<absolute path under the system temp
directory>/` and git's own words, which name that same absolute path a second time. It
breaks neither half of the clause. **The first half** — no command that works on the
previous release stops working — holds as a fact: the previous release prints the same
line at the same exit, identical once the process id and the timestamp in the directory
name are normalized. **The second half** — every refusal's route works as printed — is
not engaged: the door exits 0, the line is no refusal and carries no route, and nothing
on the screen hands the reader that path to run or to open. The design that owns the
line intends its shape (`findings: unknown — <reason>` at exit 0), and the printed-path
law admits an absolute that quotes a failed invocation; no row of that law's disposition
table names this site, so the verdict rests on the clause and cites the design as
support, never the other way round. **The verdict does not depend on whether a
repository tracking such a name is a supported layout**: read either way, the candidate
answers as the previous release does and prints no route.

## The binary, asserted before anything was driven

| which | call | `content_sha256` printed | matches the prompt |
|---|---|---|---|
| candidate, label c1, commit `eeffe347` | `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Both calls ran before the first driven command. In every candidate cell the candidate's
directory went first on `PATH` and `command -v jigc` printed `<scratch>/bin/c1.a1/jigc`,
checked by the shell before the cell ran; in the previous release's cells its directory
went first and every command was also typed by its absolute path. Nothing was built and
nothing under `target/` was driven. Each rig was built with `dev/jigc-rig refs-post-hoc
--binary <that path>`, exit 0, stdout captured alone.

## What was read

The finding as the prompt hands it: item 6 under `Left open` of
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-doc-show-stranded-doc-not-found-route.a1.md`,
and the plant of that report's `Repro VR-DS-1`, which the prompt names as the state to
re-drive. That file was opened whole to reach the two; nothing in this verdict rests on
its reasoning, and its own verdict was not re-examined. The clause:
`completions/artifacts/canary-one/opening.md` → The closing condition, and `DECISIONS.md`
→ *2026-10-04 — The exit rule, revised*, second sharpening. The design that owns the
behaviour: `design/bootstrap.md` → the blocked-task state (the paragraph under shape 4);
`DECISIONS.md` → the orientation entry's paragraph *Five keys come off disk; findings do
not, and the door degrades rather than lying*; `design/surface-contract.md` → The
printed-path fence (law 1); and the code those name — `crates/cli/src/task.rs`,
`Task::materialize_index`, `git_run` and `ScratchTree::new`, and
`crates/cli/src/render.rs` line 226.

## What was driven

Two rigs, one per binary, each minted by `dev/jigc-rig` under
`<scratch>/verify-p3-start.v3FnOJ`, a directory of this verifier's own. Every exit status
was read bare, on the line after its command; stdout and stderr went to separate files.
Host: macOS, `git version 2.54.0 (Apple Git-157)`.

The plant, as the prompt's block gives it, needing no file on disk:

    blob=$(printf 'x\n' | git hash-object -w --stdin)                            # exit 0
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"     # exit 0

A before-control found it: `git ls-files -z`, exit 0, holds the raw name once.

| cell | binary | sequence | `jigc start`: exit, and the findings line |
|---|---|---|---|
| C-control | candidate | fresh rig · `jigc start` | exit 0 · `findings: 2 blocking, 1 advisory` |
| C1 | candidate | plant · `jigc start` | exit 0 · `findings: unknown — …`, the path under the host's temp directory |
| C-tmp | candidate | the same rig · `TMPDIR=<a fresh directory of mine> jigc start` | exit 0 · the same line, the path now under that directory |
| C-json | candidate | the same rig · `jigc start --format json` | exit 0 · `"findings": null` and `"findings_unavailable"`, the same reason |
| P-control | previous | fresh rig · `jigc start` | exit 0 · `findings: 2 blocking, 1 advisory` |
| P1 | previous | plant · `jigc start` | exit 0 · the same line as C1 |
| P-tmp | previous | the same rig · `TMPDIR=<a fresh directory of mine> jigc start` | exit 0 · the same line as C-tmp |
| P-json | previous | the same rig · `jigc start --format json` | exit 0 · the same two keys as C-json |

Cells C-tmp, C-json, P-tmp and P-json re-read the rig of C1 and P1; the door is a read
and the plant is the only change either rig carries (`git status --short` afterwards:
one row, `AD` on the planted name).

### The line, cell C1 (candidate)

stderr empty; stdout, the active-task block:

```text
Active task: ground-the-vision-in-research
  workflow: form-vision
  intent:   ground the vision in research
  base:     f51b579
  staged:   commit:ground-the-vision-in-research, vision:vision
  findings: unknown — `git checkout-index -a --prefix=<tmp>/jigc-index-9153-1791466836808241000/` failed: error: unable to create file <tmp>/jigc-index-9153-1791466836808241000/bad<U+FFFD>name.txt: Illegal byte sequence
```

`<tmp>` stands for this host's per-user temporary directory, printed in full on the
screen — an absolute path of the machine, twice on the one line: once as the argument
jigc handed git, once inside git's own error. Below the block the four `Run:` directives
and the workflow list follow as in the control; the three finding blocks of the control
are absent.

### The line, cell P1 (previous release)

stderr empty; stdout line 10:

```text
  findings: unknown — `git checkout-index -a --prefix=<tmp>/jigc-index-22011-1791466855098709000/` failed: error: unable to create file <tmp>/jigc-index-22011-1791466855098709000/bad<U+FFFD>name.txt: Illegal byte sequence
```

### The two binaries, compared

Each captured stdout was normalized by three substitutions — the rig's directory name,
`jigc-index-<pid>-<nanos>`, and the name of the temp directory handed in — and compared.

| pair | `cmp` of the findings line | `diff` of the whole stdout |
|---|---|---|
| C1 / P1 | exit 0 | one line differs: `base:`, the rig's own commit hash |
| C-tmp / P-tmp | exit 0 | the same one line |
| C-json / P-json (`findings_unavailable`) | exit 0 | the base's `sha` and `short`, nothing else |
| C-control / P-control | — | the same one line |

So the candidate's orientation in this state is the previous release's, to the byte,
apart from a commit hash each rig mints for itself.

### The path in the line

- **It is the system temp directory's, never the repository's.** With `TMPDIR` pointed
  at a fresh directory under my scratch root, the line names
  `<that directory>/jigc-index-<pid>-<nanos>/` on both binaries (C-tmp, P-tmp). Read:
  `ScratchTree::new` builds it from `std::env::temp_dir()`.
- **It names nothing that exists once the command has returned.** `ls -d` of the path
  C1 printed exits 1, *No such file or directory*; the same for P1's; the directory
  handed in as `TMPDIR` is empty after C-tmp and after P-tmp. The scratch tree removes
  itself on drop, on the failure path as well.
- **It differs on every invocation** — the process id and a nanosecond timestamp are in
  its name — so no two reads of one state print the same line.

## Is it what the finding says, read against the design that owns it?

Yes for the behaviour. Against the design, each of its three parts is intended or
admitted:

1. **Exit 0 and `findings: unknown — <reason>`.** `design/bootstrap.md`, under the
   blocked-task shape: orientation reports at exit 0 *because orientation is not a gate*,
   and *a task whose sweep cannot run … reads `findings: unknown — <reason>`: the
   bootstrap door degrades, and unknown is never printed as none*. The decision behind
   it (`DECISIONS.md`, the orientation entry): *a sweep that cannot run yields
   `findings: null` plus the reason*. Driven, both forms are what the door prints.
2. **The reason is the sweep's own failure.** `Task::materialize_index` checks the index
   out into a scratch tree for the `doc-code` probe; `git_run` renders a failure as
   `` `git <args>` failed: <git's stderr> ``; `render.rs` prints `unknown — {reason}`.
   On a filesystem that refuses the name, `git checkout-index` fails and that sentence
   is the reason.
3. **An absolute path on the screen.** `design/surface-contract.md` → The printed-path
   fence admits three reasons for an absolute that stays; the second is *quoting an
   invocation — a line whose subject is the command that failed … prints the argument
   jigc handed git, beside git's own words naming that same absolute; relativizing one
   half misquotes the other*. That is this line's shape exactly. The fence's own table
   (`crates/cli/tests/repo_relative_paths.rs`, the `milestone_boundary_gate` row) records
   the companion fact: a `ScratchTree` path *leaves the repository … and the helper's
   declared fallback renders those absolute, which is its honest answer rather than a
   miss*.

**What the design does not do** is dispose this site by name. The table's domain is the
worktree doors and `jigc milestone provision`; `Task::materialize_index` has no row in
it and bare `jigc start` is not among the doors its driven scan covers, beyond the
header's declared `Project config:` segment. So *intended* holds by the class rule and
by no row — which is why this verdict's basis is the clause, below.

## Does it break `working-product`, inside that clause's scope?

The clause, as the run's opening names it: *a working product others can rely on*; its
measure, in the exit rule's second sharpening: *no command that works on rc.24 in a
supported layout stops working, and every refusal's route works as printed*.

**First half — unbroken, as a fact.** `jigc start` exits 0 on the previous release in
this state and prints the same block, the findings line identical after normalization
(cells C1 and P1, C-tmp and P-tmp, C-json and P-json). Nothing that works there stopped
working, and nothing that was printed there changed.

**Second half — not engaged.** The door exits 0: it is the orientation, and it degrades
rather than refuses. The line carries no `route:`. The path inside it is an argument of
a quoted command that has already failed; no sentence tells the reader to run, open or
remove anything at that path, and the path is gone when the command returns. There is
no route here to work or to fail as printed.

**The layout question does not decide this one.** Whether a tracked name that is not
UTF-8 is a supported layout is open in this run — no bound is declared. This verdict
holds under both answers: if the layout is supported, the first half still holds
(identical on the previous release) and the second still has no route to test; if it is
not, the finding is outside the scope outright.

A real behaviour that breaks no clause inside its scope: `breaks-no-clause`, never
`does-not-reproduce`. It stays a row of the ledger — as a surface question (should a
degraded orientation name a per-invocation temp path at all, or the cause), with its
tier, for whoever triages it.

## The regression fact

Not owed with this verdict. The fact, since both binaries were driven: the block reads
the same on the previous release as on the candidate. Not a regression.

## Class

**instance, unbounded.** One corpus state (`refs-post-hoc`, one live task), one plant,
one host, one git, two forms of one door (text and `--format json`). Read, not derived:
`grep` finds one `checkout-index` call under `crates/cli/src` (`task.rs`,
`Task::materialize_index`); the callers of `materialize_index`, and the other reasons a
sweep can be *unknown*, were not enumerated.

## Repro VR-P3-1

```yaml
claim: "with a non-UTF-8 index entry standing, bare `jigc start` exits 0 and prints an absolute temporary host path inside `findings: unknown — `git checkout-index -a --prefix=…` failed` — and that breaks the clause working-product"
verdict: "REFUTED as a blocker — breaks-no-clause. The line reproduces on both binaries, identical after normalizing the pid and timestamp; the door exits 0 and the line carries no route."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
host: "macOS, git 2.54.0 — a filesystem that refuses the planted name; see Pinnable"
plant:                      # needs no file on disk; the name is passed as raw bytes
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
cells:
  - cell: "the control — the sweep runs (C-control, P-control)"
    setup:
      - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <binary>
    repro:
      - ["jigc", "start"]
    expect:
      - exit: 0
        stderr: ""
        stdout_contains: "  findings: 2 blocking, 1 advisory"
    on-previous: "identical but for the rig's base commit hash"
  - cell: "the finding — the line and the path in it (C-tmp, P-tmp)"
    setup:
      - fixture: refs-post-hoc
      - plant
      - env: { TMPDIR: "<T>, a fresh empty directory" }
    repro:
      - ["jigc", "start"]
      - ["ls", "-A", "<T>"]
    expect:
      - exit: 0
        stderr: ""
        stdout_matches: "  findings: unknown — `git checkout-index -a --prefix=<T>/jigc-index-[0-9]+-[0-9]+/` failed: error: unable to create file <T>/jigc-index-[0-9]+-[0-9]+/bad.name\\.txt: Illegal byte sequence"
        stdout_not_contains: ["findings: none", "route:"]     # no route anywhere in the active-task block
      - exit: 0
        stdout: ""                                            # the scratch tree is gone
    on-previous: "identical after normalizing <T> and jigc-index-<pid>-<nanos> (cmp exit 0 on the line)"
  - cell: "the machine form (C-json, P-json)"
    setup:
      - fixture: refs-post-hoc
      - plant
    repro:
      - ["jigc", "start", "--format", "json"]
    expect:
      - exit: 0
        stderr: ""
        stdout_json_at: { "tasks[0].findings": null }
        stdout_contains: "\"findings_unavailable\": \"`git checkout-index -a --prefix="
    on-previous: "identical after the same normalization"
observed: "<scratch>/verify-p3-start.v3FnOJ — c1.* and p1.* (control, start, tmp, json, validate; .out and .err each, .norm the normalized copy), rig-c1.* and rig-p1.* (each rig's assignments and construction log); the rigs are the two jigc-rig-refs-post-hoc-* directories beside them"
pinned-by: "UNPINNED: host-dependent as it stands — see Pinnable"
```

**Pinnable as it stands: no.** Two reasons, both about the plant and neither about the
claim. The name is passed as raw bytes, so a test using it is Unix-only. And the line
appears only where the filesystem refuses that name: here `git checkout-index` fails
with *Illegal byte sequence*; on a filesystem that admits the name the checkout
succeeds, the sweep runs and the line reads a count — which was not driven. The
control cell converts as it is. A portable pin of the two facts this verdict rests on —
*unknown* at exit 0 with a reason and no route, and the reason's path sitting under
`TMPDIR` and gone afterwards — needs a sweep failure every host reproduces; the design
names one for the first fact (a `JIGC_DOC_CODE_PROBE` override naming no file), which
does not pass through `checkout-index` and so pins nothing about the path. Whether a
suite already asserts either was not checked: the finding makes no coverage claim, so
none was verified.

## Left open

1. **`jigc task validate <id>`, the `Run:` directive the orientation prints beside the
   line, exits 1 with one bare line in this state** — `` `git checkout-index -a
   --prefix=<tmp>/jigc-index-…/` failed: error: unable to create file … Illegal byte
   sequence `` on stderr, stdout empty: no code, no `at:`, no route. Both binaries, the
   stderr identical after normalization (`cmp` exit 0). A refusal with no route, at a
   door other than this finding's; driven once on each, not pursued.
2. **The other three `Run:` directives of that block** — `start --task`, `task
   finalize`, `task discard --force` — were not driven in this state on either binary.
3. **The machine surface carries the same per-invocation path** — `findings_unavailable`
   of `jigc start --format json` holds the reason verbatim, so the string differs on
   every call for one unchanged state. Both binaries. Whether a driver keys on it was
   not examined.
4. **No row of the printed-path fence disposes this site.** `Task::materialize_index`'s
   failure reaches bare `jigc start`, `jigc start --format json` and `jigc task
   validate` with an absolute path, under the admitted reason *quoting an invocation*,
   and the disposition table in `crates/cli/tests/repo_relative_paths.rs` has no row
   for it. Read, not driven as a fence.

## Bounds — what this verification did not do

- One macOS host, one git. A filesystem that admits the planted name was not driven;
  there the sweep is expected to run and the line not to appear.
- One corpus state with one live task. A milestone sub-task, several active tasks and a
  task with no staged doc were not driven.
- The follow-up cells (the temp-directory control, the JSON form, `task validate`)
  re-read the rig of the first cell on each binary; they were not given rigs of their
  own.
- No coverage was verified; no suite was run.
- Nothing was built, edited, staged or committed. The clone stands at `eeffe347` on
  `fix/canary-one`, its only untracked path the run's own `r1/` directory, as before.

## Where the evidence is

`<scratch>/verify-p3-start.v3FnOJ`: the two rigs, `rig-c1.env`, `rig-c1.err`,
`rig-p1.env`, `rig-p1.err`, the cells' captured stdout and stderr (`c1.*`, `p1.*`), and
the two directories handed in as `TMPDIR` (`tmp-c1.*`, `tmp-p1.*`), both empty.

<!-- end of report -->
