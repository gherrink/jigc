# Verification — `r1-p3-amend-refusal-route-spells-c-quoted-name-as-path`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, one verdict.

- **Door:** `jigc task finalize`, the amend arm.
- **Clause said to be broken:** `working-product`.
- **Triage's grade:** unclear.
- **Verdict: confirmed.** `regression: false` — the previous release is red in the same way.
- **Basis, one line:** under git's default `core.quotePath`, the amend refusal's printed
  `git -C <repo> restore --staged -- '"caf\303\251.txt"'` exits 1 when run as printed, the
  index is unchanged and the re-run finalize refuses again with the same route — a refusal
  whose route does not work as printed; identical on `1.0.0-rc.24`.

## What the finding is, and what I was asked to drive

The finding is item 4 under `Left open` of
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-git-capture-other-callers-not-enumerated.a1.md`:
the amend refusal prints an unstage route that spells git's C-quoted rendering of a name as if
it were the path. That reporter saw it for an index entry holding byte 0xFF, on a filesystem
that cannot hold that name, and did not run the route. It carries no repro block.

Triage asked for the form a user reaches without plumbing: a staged file with a non-ASCII name
**that the filesystem holds**, at the amend arm, the printed `restore --staged` run as printed,
on both binaries. That is what I drove. I reconstructed the setup from the reporter's cell
heading (*rig `fresh`, then `jigc task amend`*); nothing else of that report was used.

## The binaries

Asserted before anything was driven, by the call the prompt spells:

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` |
|---|---|---|
| candidate (commit `eeffe347`, label c1) | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` |
| previous release (`1.0.0-rc.24`) | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` |

Both equal the hashes the prompt handed me. With the candidate's directory first on `PATH`,
`command -v jigc` printed the candidate's path. Every rig was built with
`dev/jigc-rig fresh --binary <that absolute path>`, and each run checked that the rig's `$JIGC`
is the handed binary before it drove anything. Nothing was built; nothing under `target/` ran.
Host: one macOS machine, git 2.54.0 (Apple Git-157).

## Repro VR-AMEND-ROUTE-1

The block, in the pipeline's shape. Its `expect` is what the clause demands; the candidate
and the previous release both fail the two marked assertions today.

```yaml
claim: "the amend arm's index refusal prints an unstage route that runs as printed when the staged name is non-ASCII"
verdict: CONFIRMED-BROKEN   # the claim as stated is false on both binaries
setup:
  - fixture: fresh                       # dev/jigc-rig fresh --binary <binary>; default git config, core.quotePath unset
  - ["jigc", "task", "amend", "repair message"]          # exit 0; task id `repair-message`
  - write: { path: "café.txt", bytes: "hello\n" }        # name bytes 63 61 66 c3 a9 2e 74 78 74
  - ["git", "add", "--", "café.txt"]                     # exit 0
repro:
  - ["jigc", "task", "finalize", "repair-message"]       # step A
  - run-as-printed: "the backticked `git -C <repo> restore --staged -- …` span of step A's route"   # step B, through `sh`
  - ["git", "diff", "--cached", "--name-only", "-z", "HEAD"]   # step C
  - ["jigc", "task", "finalize", "repair-message"]       # step D
expect:
  A: { exit: 3, stderr_contains: "finalize.amend-index-dirty", head: unchanged }
  B: { exit: 0 }                         # RED today: exit 1, `error: pathspec '"caf\303\251.txt"' did not match any file(s) known to git`
  C: { stdout: "" }                      # RED today: `café.txt` is still staged
  D: { stderr_not_contains: "finalize.amend-index-dirty" }   # RED today: the same finding, the same route
  worktree: "café.txt still holds `hello\n`"   # holds today, and must keep holding
control:
  - same block with the name `cafe.txt`: B exits 0, C is empty, D no longer names the index
pinned-by: "UNPINNED: no fix has landed; this block is the fix's red test"
```

### What each step printed — candidate

Host paths are shown as `<repo>`; the route was run from a file holding the span byte for byte.

Step A, exit 3, standard output empty, standard error:

```text
blocking · finalize.amend-index-dirty — `"caf\303\251.txt"` is staged, and an amend rewrites `HEAD` from the index — finalizing now would fold it into the commit whose message this task is repairing, a change that commit never carried
  at: "caf\303\251.txt"
  route: unstage it (`git -C <repo> restore --staged -- '"caf\303\251.txt"'`) and re-run the finalize — this arm takes no `--carry-staged`, because an amend that carried anything would change a tree it promised not to touch
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

Step B, the span run as printed:

```text
error: pathspec '"caf\303\251.txt"' did not match any file(s) known to git
exit 1
```

Step C printed the name `café.txt` (bytes `63 61 66 c3 a9 2e 74 78 74 00`) — still staged.
Step D exited 3 with the same finding and the same route. `HEAD` did not move at any step,
and `café.txt` stood in the worktree throughout: nothing is lost here, and nothing is written.

### The four cells

| cell | binary | name | `core.quotePath` | A exit | route as printed, exit | index after the route | D |
|---|---|---|---|---|---|---|---|
| the finding | candidate | `café.txt` | unset (default) | 3 | **1** | `café.txt` still staged | 3, `finalize.amend-index-dirty` again |
| the finding | previous release | `café.txt` | unset (default) | 3 | **1** | `café.txt` still staged | 3, `finalize.amend-index-dirty` again |
| control | candidate | `cafe.txt` | unset (default) | 3 | 0 | empty | 3, the commit doc's own two findings only |
| bound | candidate | `café.txt` | `false`, set in the repo | 3 | 0 | empty | 3, the commit doc's own two findings only |

In the control and the bound cell, step D's exit 3 is the unauthored commit doc
(`schema-conformance.field-value-conformant`, `schema-conformance.required-slot-present`): the
index gate has let go, which is the route's whole job. I did not author the doc to a landed
amend — the route's end state is the empty index, and that is what the block asserts.

Each cell is a fresh rig. The previous-release cell's output is the candidate's, line for line,
apart from the rig's path and its commit ids.

## Trying to refute it

- **Stale state, another binary.** Fresh rig per cell, `$JIGC` checked against the handed path,
  hashes asserted. Does not explain it.
- **Read through a pipe.** Every exit status above was read bare, from the command itself.
- **An artifact of how I ran the route.** The control and the bound cell run their printed
  span through the same extraction and the same `sh`, and both exit 0. The only variable in
  the red cells is the spelling of the name inside the span.
- **Ambient git configuration.** `git config --get core.quotePath` in each rig printed nothing
  and exited 1: the default. The rig's home is its own. The bound cell shows the mechanism
  from the other side: with `core.quotePath=false` git prints the name raw, the route reads
  `restore --staged -- 'café.txt'`, and it works.
- **A name the user could not have.** The file was created with an ordinary write and staged
  with an ordinary `git add`; the filesystem holds it; no plumbing was used.
- **Intended by a settled decision.** I found none, and found the opposite. The design section
  that owns a printed command span — `design/surface-contract.md` → *The route fence (law 2)*,
  *The quoting half* — names `finalize`'s `git restore --staged` among the engine-minted routes
  the quoting rule exists for, says its axis suite asserts *that the command spans a door
  prints can be run*, and, in its declared bound for a control byte, rules that *an escaped
  route names a different file*. That is this case exactly: the span is correctly
  single-quoted for the shell, and what it quotes is git's display escape, not the path.
  `DECISIONS.md` line 12536 records the same fact for another reader — git display-quotes a
  non-ASCII name, and *the plain form would name a path the repo does not contain* — and
  chose `-z` there. The one entry that states a C-quote residual (`DECISIONS.md`, the entry of
  2026-09-23, *the confirmation pass*) is about the pre-commit rename backstop's script and
  says nothing of this route. The declared bound in the surface contract covers how a control
  byte **renders**; it does not cover a route that names other bytes.

By reading, not as evidence: the producer is `amend_index_findings` (`crates/cli/src/task.rs`,
line 3026), which takes the lines of `git diff --cached --name-only HEAD` — no `-z` — as
paths and hands each to `amend_index_dirty_finding` (line 246), which quotes it with
`shell_token` and puts it in the route, the message and the `at:` locus.

## Does it break the clause, inside its scope?

The clause is `working-product`, the closing condition's second: *a working product others can
rely on*. Its sharpening in `DECISIONS.md` → *2026-10-04 — The exit rule, revised* reads: *no
command that works on rc.24 in a supported layout stops working, and every refusal's route
works as printed.*

- **The second half is broken.** This is a refusal, it prints a route, and the route does not
  work as printed. The refusal leaves the user with no working instruction: the re-run prints
  the same route again.
- **Inside the scope.** The state is a healthy repository under git's default configuration, a
  file with one accented letter in its name, staged by `git add`, and a verb the product
  advertises. Nothing is planted and no race is involved; no declared bound names it.
- **The first half is not broken**, and I say so because the two must not be read as one: no
  command that works on `1.0.0-rc.24` stops working here. The previous release fails the same
  way. Whether a route that was already dead on the previous release blocks the call under
  the clause's second half is the clause's letter as I read it; it is triage's and the
  human's to weigh, not mine.

## The regression fact

Same block, fresh rig, the previous release's binary by its absolute path: step A exit 3 with
the same C-quoted route, step B exit 1 with the same git error, step C `café.txt` still
staged, step D exit 3 with the same finding. **Red there and red on the candidate:
`regression: false`.** Comparable — the door exists on the previous release and the block ran
on it unchanged.

## Coverage

The finding makes no coverage claim, so none was verified. I did not search the suites for a
test of this cell and state nothing about one.

## Class

**Instance, unbounded.** I drove one door (`jigc task finalize`, amend arm), one name, one
host. I did not enumerate the other places that read a git name listing without `-z` and put
the line into a route, and I give no count.

## Pinnable

**Yes, as it stands.** The block needs the `fresh` fixture, one file whose name holds one
two-byte UTF-8 character, and a shell to run the extracted span — the shape
`path_arg_occurrence_axis` already uses for its cells, by the surface contract's own account.
One condition: the test's filesystem must hold the name, which APFS and the CI runner's
filesystem do. Assertions B, C and D are red until a fix lands, so it is the fix's red test.

## Left open

Not pursued, each to be triaged like any finding:

1. **The preview door shares the producer.** The source comment on `amend_index_findings` says
   it is asked at the committing door and inside `TaskArea::preview_gates`. Read, not driven:
   the preview would print the same route.
2. **The message and the `at:` locus carry the C-quoted spelling too** — `"caf\303\251.txt"`,
   a path the repository does not hold, where the surface contract's law 1 asks for a printed
   path that is repo-real. The `--format json` envelope and its `(code, target)` key were not
   driven.
3. **The reporter's original form — an index entry whose name the filesystem cannot hold
   (byte 0xFF)** — was not driven. Its route was never run by anyone.
4. **Names git quotes whatever `core.quotePath` says** — a double quote, a backslash, a
   control byte. Under `core.quotePath=false` the bound cell is green for `café.txt`; for
   these it would, by reading, still be the escaped spelling. Not driven.
5. **Other routes built from a git name listing read without `-z`.** Not enumerated.

## Bounds — what this verification did not do

- One macOS host, one git (2.54.0). No Linux run.
- Agent-format output only; no `--format json`, no `--format human`.
- No amend was carried through to a rewritten `HEAD` in any cell.
- Nothing in the repository was edited, staged or committed; the only file written is this
  report, under the scratch root, handed over through `dev/stabilize-record`.

<!-- end of report -->
