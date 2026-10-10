# verify-real — `r1-validate-unreadable-orphan-variant-not-driven`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed over by triage with the
grade *unclear*: door `jigc validate`, clause said to be broken `working-product`. It has no
block of its own: it is the variant line of `Repro RC-9`
(`completions/artifacts/canary-one/r1/reports/test/row-doc-list-reconciler.a1.md`), which the
first verifier of the parent finding listed under *Left open* as not driven, with the setup of
its `Repro RC-9v` through the control
(`completions/artifacts/canary-one/r1/reports/test/verify-p1-r1-validate-non-utf8-path-drops-blocking-orphan.a1.md`).
Of those two reports I read what the prompt handed me — RC-8 (whose setup RC-9 cites) and RC-9
of the first, and the whole of the second, which is this finding's source — and no other
finding's report and no other verifier's.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** It is **not** `does-not-reproduce`: the
variant is real and reproduces exactly as the reporter's one line says, on the candidate and,
byte for byte, on the previous release. It stays a row of the ledger.

- **What reproduces.** Over a store holding a stamped committed file no doctype claims,
  `jigc validate` exits 1 with a blocking `schema-conformance.orphaned-instance`. Take read
  permission off that one file (`chmod 000 docs/zzz/orphan.md`), as a caller that is not root,
  and the same command exits 0 and prints *no findings — the committed store validates clean*,
  with nothing on stderr; `--format json` exits 0 with `"findings": []`,
  `"blocking_probes": []`, `"report_only": true`. Restore the mode and the exit 1 is back, its
  output `cmp`-identical to the control.
- **Why it breaks no clause.** The run's closing condition is the exit rule of 2026-10-04
  (`DECISIONS.md` → *The exit rule, revised*), and `working-product` is its second clause,
  *a working product others can rely on*. That entry gives the clause one instrument: *no
  command that works on rc.24 in a supported layout stops working, and every refusal's route
  works as printed*. Neither limb is touched. The first: the control, the repro and the
  restored control are byte-identical between the candidate and the previous release, in text
  and in JSON, on stdout and on stderr — nothing stopped working. The second: the failing state
  prints no refusal, so no printed route fails; the fault is that a refusal is absent.
- **What the verdict does not rest on.** Not on the state being planted, and not on the layout
  being unsupported. The opening record declares no bound, and no design section I read says
  whether a tracked file the caller cannot read, inside a managed home, is a supported layout.
- **The reading is mine, and it is the one thing that could turn this.** I read the clause by
  its instrument's two limbs, because the entry gives each clause exactly one instrument and
  says a finding blocks only inside the clause's scope. If *rely on* is read to reach a false
  green that the previous release prints too, this row re-grades. Under *Left open*.
- **Not `intended`.** No settled decision intends an exit-0 clean here. The one statement of a
  posture is a source comment on the stamp reader (`crates/cli/src/orphan.rs`, `carries_stamp`):
  *a file git tracks but the worktree no longer holds reads as unstamped — the sweep is a
  statement about bytes it could read, never an assertion built on an absent file*. It names
  the absent file, and the unreadable one falls through the same `.ok()`. But a comment is not
  a ruling, and the comment at the sweep's call site says the opposite about this consumer: the
  finding is blocking *so the sweep refuses the exit-0 green rather than reporting that it
  found nothing in files it has no schema to read* (`crates/cli/src/cli.rs`, the
  orphaned-instance arm). `design/validation.md` → the `schema-conformance.orphaned-instance`
  row states the three legs of the condition and no best-effort posture for a file that cannot
  be read; a text search of `design/`, `implementation/` and `DECISIONS.md` for that posture
  found nothing.
- **`contested`: false.** The finding does not argue that a settled decision is wrong.
- **Regression fact.** Not owed with `refuted`. The previous release was driven because triage
  asked for both binaries and because the instrument's first limb cannot be answered without
  it: red there too, byte-identical.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (label c1, commit
  eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- The candidate's directory went first on `PATH`; `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`, exit 0, and it was printed again at the head of each drive.
- No `cargo build`, nothing under `target/`. The clone: `git status --porcelain` read
  `?? completions/artifacts/canary-one/r1/`, `HEAD` eeffe347324f83a51d1ae83d5f254e73c3f1ea3a,
  branch `fix/canary-one`; nothing was edited, staged or committed there.
- Both binaries print `jigc 1.0.0-rc.24` for `--version`; every line here is keyed by the hash.

## How it was driven

Host: macOS 26.6.2, git 2.54.0 (Apple Git-157). The caller's uid is 501 — not root, which the
variant needs: a mode of 000 stops no read by root. One directory of my own under the scratch
root, `<scratch>/verify-unreadable.iCeRM2` (written `<W>`), and under it one fresh root per
binary, each from `mktemp -d`: `<W>/cand.wNVwgQ` and `<W>/prev.5C0gBB`. Nothing of the
reporter's or of the first verifier's was reused, and nothing was torn down.

Environment of every invocation: `HOME=<root>/home` (empty before `setup`),
`GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four
`GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables. Every jigc call: stdin from `/dev/null`, stdout
and stderr to their own files, the exit status read directly — no pipe in front of any status.

**What I changed from the block as written: nothing in the setup, and one step added.** The
repository is built by hand, command for command as `Repro RC-9v` has it (`git init`, an empty
commit, `jigc setup`, the orphan committed with `--no-verify`), not with `dev/jigc-rig`, so
that the setup is the block's to the letter. The step added is a before-control of the plant:
after the `chmod`, `cat docs/zzz/orphan.md` must fail — otherwise an exit 0 would say nothing
about an unreadable file.

### Candidate (`<W>/cand.wNVwgQ`)

| step | exit | observed |
|---|---|---|
| `git init -q repo` · `git -C repo commit -q --allow-empty -m base` | 0 · 0 | - |
| `jigc setup` | 0 | - |
| write `docs/zzz/orphan.md`, `git add`, `git commit -q --no-verify -m "an orphan"` | 0 · 0 | porcelain empty afterwards |
| **control:** `jigc validate` | **1** | stdout 1191 bytes, stderr 0 bytes; opens `blocking · schema-conformance.orphaned-instance — committed doc ` + the path `docs/zzz/orphan.md` |
| control, `--format json` | 1 | 1243 bytes; `"blocking_probes": ["schema-conformance"]`, one finding keyed at `docs/zzz/orphan.md`, `"report_only": false` |
| `chmod 000 docs/zzz/orphan.md` | 0 | `ls -l` reads `----------`, 58 bytes |
| before-control of the plant: `cat docs/zzz/orphan.md` | 1 | `Permission denied` |
| `git status --porcelain` | 0 | ` M docs/zzz/orphan.md` — git cannot hash the file and calls it modified; the index entry is unchanged (`100644 6dc1c966…`) |
| **repro:** `jigc validate` | **0** | stdout 125 bytes, stderr 0 bytes — below |
| repro, `--format json` | **0** | 112 bytes, stderr 0 bytes — below |
| `chmod 644 docs/zzz/orphan.md` | 0 | porcelain empty again |
| **control, mode restored:** `jigc validate` | **1** | 1191 bytes; `cmp` against the first control: identical |
| control, mode restored, `--format json` | 1 | 1243 bytes; `cmp` against the first control: identical |

The repro's stdout, whole:

```text
no findings — the committed store validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

The repro's JSON, whole:

```json
{
  "blocking_probes": [],
  "findings": [],
  "report_only": true,
  "schema_version": 3,
  "scope": "store"
}
```

### Attempts to refute it, each of which failed

All on the candidate, in the same root, after the block above.

| attempt | what was done | result |
|---|---|---|
| stale state | the mode restored, then `jigc validate` | exit 1; stdout `cmp`-identical to the control, text and JSON — the mode is the cause, and the store did not drift |
| the plant did not take | `cat` of the file under mode 000 | exit 1, `Permission denied` — the caller really cannot read it |
| a dirty work tree, not an unreadable file | the file readable (mode 644) with one uncommitted line appended, so that porcelain reads ` M` exactly as under mode 000 | exit 1; stdout `cmp`-identical to the control — being modified is not the cause |
| the mode change as such | mode 400: changed, still readable | exit 1; stdout `cmp`-identical to the control |
| one mode only | mode 200: write-only, a second unreadable mode | exit 0; stdout `cmp`-identical to the repro, stderr empty |
| the whole listing failing, as in the parent finding | a second stamped orphan, `docs/zzz/second.md`, committed beside the first; then mode 000 on the first only | both readable: exit 1, two findings. One unreadable: exit 1, **one** finding, `docs/zzz/second.md`; the unreadable file is named nowhere on stdout and stderr is empty — the silence is per file, not a failed enumeration |
| read through a pipe, or cut | every status read bare; outputs sized with `wc -c` and compared with `cmp` | 1191 / 125 bytes, the sizes the earlier reports give for the two states |
| another binary | `command -v jigc` at the head of each drive | the candidate |

So the mechanism is not the parent finding's. There the committed listing comes back empty
because one capture of git's output fails. Here the listing is whole and one file's stamp
cannot be read: `carries_stamp` reads the file with `read_to_string(...).ok()`, a failed read
is `None`, and the file is filtered out as unstamped.

### Previous release (`<W>/prev.5C0gBB`, driven by absolute path)

The same block on a fresh root, every jigc call `<scratch>/bin/previous-91834b5e011d/jigc`:
`setup` 0; control `jigc validate` **exit 1** and `--format json` exit 1; `chmod 000` 0 and
`cat` exit 1, `Permission denied`; repro `jigc validate` **exit 0** and `--format json`
**exit 0**; mode restored, `jigc validate` **exit 1** and `--format json` exit 1.

`cmp` of each of the twelve output files against the candidate's — stdout and stderr of the
control, the repro and the restored control, text and JSON: **all twelve identical** (1191 / 0
/ 1243 / 0 bytes for the controls, 125 / 0 / 112 / 0 for the repro).

## Does it break the clause, inside the clause's scope

This is the question an *unclear* grade sends a verifier to answer, and the answer is no, on
the reading stated in the verdict.

- **The clause is in the closing condition.** `completions/artifacts/canary-one/opening.md`
  names `working-product` as the second clause of the entry of 2026-10-04, with the regression
  set and, in this run, the gate as its instrument.
- **The instrument's first limb — nothing that works on rc.24 stops working.** Answered by the
  twelve `cmp`s: the candidate does here exactly what rc.24 does. Whatever this behaviour is, the
  round's change did not make it.
- **The instrument's second limb — every refusal's route works as printed.** The failing state
  prints no refusal and no route. The control's route (`jigc ingest`, or
  `jigc unmanage docs/zzz/orphan.md`) is not what the finding is about and was not followed.
- **No other clause was asked of me, and I grade none.** One fact that bears on the first
  clause is stated because I observed it: `jigc validate` wrote nothing in any cell — porcelain
  was empty after the mode was restored.

## The class

`instance, unbounded`. Driven: one state (one tracked, stamped, unclaimed file inside a managed
home, unreadable by its mode), one door (`jigc validate`, store scope, text and
`--format json`), one finding code (`schema-conformance.orphaned-instance`), both binaries, one
platform. I enumerated no consumers. `orphaned_instances(` has two production call sites by a
text search of `crates/cli/src` — the store sweep in `cli.rs` and the listing in `doc.rs` —
and I drove the first only; that count is a text search of one function's name, not a
derivation of the class, and the other readers of a committed file's bytes in the sweep were
not looked at.

## The coverage claim

The parent block says `UNPINNED: found this round`. Checked by a text scan of the suites, not
by reading every test: fifteen files under `crates/cli/src`, `crates/cli/tests`,
`crates/engine/src` and `tooling-tests` name `orphaned-instance`, `orphaned_instance` or
`ORPHANED_INSTANCE`. Exactly one of them sets a file mode at all,
`crates/cli/tests/flow52_acceptance.rs`, and what it changes the mode of is a git hook. The
suite that owns the finding, `crates/cli/tests/orphaned_instance.rs`, has eight tests and none
removes a permission; the unit tests beside `carries_stamp` in `crates/cli/src/orphan.rs`
commit readable files only. So nothing I found plants an unreadable orphan under this finding,
and nothing pins the absent-file sentence of the source comment for this sweep either. A scan
of names is what this is; the claim stands as far as it reaches.

## Repro RC-9u — the block, in the pipeline's schema

```yaml
claim: "over a store holding a stamped committed file no doctype claims, `jigc validate` exits 1 with a blocking `schema-conformance.orphaned-instance`; with read permission taken off that one file it exits 0 and prints `no findings — the committed store validates clean`, in text and in JSON"
verdict: "REFUTED as a blocker (breaks-no-clause) — the behaviour itself REPRODUCES, on both binaries"
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — control, repro and restored control byte-identical, twelve files of twelve"
requires: "a Unix host and a caller that is not root (mode 000 stops no read by root)"
env:
  HOME: "<root>/home"
  GIT_CONFIG_GLOBAL: "/dev/null"
  GIT_CONFIG_NOSYSTEM: "1"
setup:
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]                                   # in repo; exit 0
  - "write docs/zzz/orphan.md = `---\nschema-version: 1\n---\n# An orphan nobody claims\n\nbody\n`"
  - ["git", "add", "docs/zzz/orphan.md"]
  - ["git", "commit", "-q", "--no-verify", "-m", "an orphan"]
  - control: ["jigc", "validate"]                       # exit 1; stdout opens `blocking · schema-conformance.orphaned-instance — committed doc `, 1191 bytes; stderr empty
  - ["chmod", "000", "docs/zzz/orphan.md"]              # exit 0
  - plant-control: ["cat", "docs/zzz/orphan.md"]        # exit 1, `Permission denied` — else the cell is vacuous
repro:
  - ["jigc", "validate"]
  - ["jigc", "validate", "--format", "json"]
expect:
  - exit: 0
    stdout: "no findings — the committed store validates clean\n— jigc · run `jigc start` for orientation; all writes through `jigc`.\n"
    stderr: ""
  - exit: 0
    stdout_json: { "blocking_probes": [], "findings": [], "report_only": true, "schema_version": 3, "scope": "store" }
    stderr: ""
controls:
  - "chmod 644 docs/zzz/orphan.md -> `jigc validate` exit 1 and `--format json` exit 1, each stdout identical to the control"
  - "mode 644 with one uncommitted line appended (porcelain ` M`, as under mode 000) -> exit 1, stdout identical to the control"
  - "mode 400 -> exit 1, identical to the control; mode 200 -> exit 0, identical to the repro"
  - "a second stamped orphan beside it, readable -> exit 1 with ONE finding, the readable file; the unreadable one is named nowhere"
observed: "<W>/cand.wNVwgQ/out and <W>/prev.5C0gBB/out — control.out, control.json, repro.out, repro.json, restored.out, restored.json (each with its .err); on the candidate also a200.out, b400.out, c-edit.out, d-both.json, d-one000.json, d-one000.out, e-absent.out, final.out"
pinned-by: "UNPINNED: the `expect` above is the behaviour as it stands, which no ruling has made intended"
```

**Pinnable as it stands: no — for two reasons, neither of them mechanical difficulty.**

1. **The `expect` is the behaviour in question.** A standing test over this block would fence
   the false green in. No ruling says an exit-0 clean is the intended answer when a tracked
   file in a managed home cannot be read, so there is nothing yet for a pin to hold. Once the
   row is ruled the block is ready either way: fixed, it is the fix's red test with `expect`
   turned to a non-zero exit that names the file; bounded, it pins the bound as written.
2. **The cell is vacuous as root.** Unlike the parent block, every argument here is a plain
   string, so the block is constructible as written. But a mode of 000 stops nothing for uid 0,
   so a test needs the `plant-control` step as a guard and must skip, announced, where the read
   succeeds — the bound `DECISIONS.md` already records for the tree's other `chmod 000` cells.

## Left open

- **The reading of the `working-product` clause.** Read here by its instrument's two limbs.
  Whether *rely on* reaches a false green that the previous release prints too is the human's
  to say. If it does, this row is not `refuted`.
- **Whether a tracked file the caller cannot read is a supported layout.** Ruled nowhere I
  read, and under no declared bound of this run. The verdict above does not use it.
- **The absent-file sibling — hit on the way, driven once, not pursued.** With the orphan moved
  out of the work tree (porcelain ` D docs/zzz/orphan.md`), `jigc validate` on the candidate
  exits 0 with stdout `cmp`-identical to the repro; put back, exit 1 again. This is the case
  the source comment on `carries_stamp` names and intends. It was not driven on the previous
  release, and no design section I read carries that comment's posture.
- **The partial answer.** With a readable orphan beside the unreadable one the sweep exits 1
  and names the readable file only; a reader who clears that one finding is then told the store
  is clean. Same mechanism, noted, not pursued.
- **`jigc doc list` under this state.** The reporter's RC-8 records the same variant there
  (*no committed docs* at exit 0). It is another door and was not driven here.

## Bounds — what this verification did not do

- One platform, one git, one uid. Nothing on Linux, nothing as root, no ACL or ownership
  variant of *unreadable*, no unreadable parent directory.
- No other consumer of the stamp reader or of the committed listing, no other finding code of
  the sweep, no other door.
- The route the control's finding prints was not followed; the finding makes no claim about it.
- `jigc task finalize`, the installed pre-commit hook and every other gate were not driven
  under this state.
- One file of mine outside the scratch root: a list of fifteen file names from the coverage
  scan, written under the system temp directory and removed by its literal path.

<!-- end of report -->
