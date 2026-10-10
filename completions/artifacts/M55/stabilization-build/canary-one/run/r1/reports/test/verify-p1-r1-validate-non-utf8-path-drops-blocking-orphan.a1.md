# verify-real — `r1-validate-non-utf8-path-drops-blocking-orphan`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed over by triage with the
grade *unclear*: door `jigc validate`, clause said to be broken `working-product`, repro block
`Repro RC-9` of `completions/artifacts/canary-one/r1/reports/test/row-doc-list-reconciler.a1.md`.
No other finding's report and no other verifier's was read; of that report, only its sections on
the binary, on how rows were measured, and RC-8 (whose setup RC-9 cites) and RC-9.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** It is **not** `does-not-reproduce`: the
behaviour is real and reproduces exactly as the block says, on the candidate and, byte for byte,
on the previous release. It stays a row of the ledger.

- **What reproduces.** Over a store holding a stamped committed file no doctype claims,
  `jigc validate` exits 1 with a blocking `schema-conformance.orphaned-instance`. Add one index
  entry whose name holds the byte 0xFF and the same command exits 0 and prints
  *no findings — the committed store validates clean*, with nothing on stderr. Take the entry out
  and the exit 1 is back.
- **Why it breaks no clause.** The run's closing condition is the exit rule of 2026-10-04
  (`DECISIONS.md` → *The exit rule, revised*). Its second clause, *a working product others can
  rely on*, is given one instrument there: *no command that works on rc.24 in a supported layout
  stops working, and every refusal's route works as printed*. Neither limb is touched. Nothing
  changed against the previous release — control and repro are byte-identical on both binaries.
  And no refusal is printed in the failing case, so no printed route fails; the fault is the
  absence of one.
- **What the verdict does not rest on.** It does not rest on the state being planted or the
  layout being unsupported. No declared bound of this run names a tracked path whose name is not
  UTF-8 (the opening record declares none), and no design doc I read says whether that layout is
  supported. That question is open, and it is under *Left open*.
- **The reading is mine, and it is the one thing that could turn this.** I read the clause by
  its instrument's two limbs, because the entry gives every clause exactly one instrument and
  says a finding blocks only inside the clause's scope. If *rely on* is read to reach a false
  green that the previous release prints too, this row re-grades. Also under *Left open*.
- **Not `intended`.** No settled decision intends an exit-0 clean here. The enumerator's own
  comment calls an empty listing on a git failure *best-effort advisory*
  (`crates/cli/src/orphan.rs`, `committed_markdown`), which dates from when it fed an advisory
  only. The sweep's comment at the call site says the opposite about this consumer: the finding
  is blocking *so the sweep refuses the exit-0 green rather than reporting that it found nothing*
  (`crates/cli/src/cli.rs`, the orphaned-instance arm). `design/validation.md` → the
  `schema-conformance.orphaned-instance` row states no best-effort posture for it.
- **`contested`: false.** The finding does not argue that a settled decision is wrong.
- **Regression fact.** Not owed with `refuted`. The previous release was driven anyway, because
  the first limb of the instrument cannot be answered without it: red there too, byte-identical.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the prompt gives for the candidate (label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- The candidate's directory went first on `PATH`; `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`, exit 0.
- No `cargo build`, nothing under `target/`. The clone: `git status --porcelain` read
  `?? completions/artifacts/canary-one/r1/`, `HEAD` eeffe347324f83a51d1ae83d5f254e73c3f1ea3a,
  branch `fix/canary-one`; nothing was edited, staged or committed there.

## How it was driven

Host: macOS 26.6.2, git 2.54.0 (Apple Git-157). One directory of my own under the scratch root,
`<scratch>/verify-rc9.UOlQKe` (written `<W>`), and under it one fresh root per binary, each from
`mktemp -d`: `<W>/cand.7eTlSJ` and `<W>/prev.if6qIm`. Nothing of the reporter's was reused, and
nothing was torn down.

Environment of every invocation: `HOME=<root>/home` (empty before `setup`),
`GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four
`GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables. Every jigc call: stdin from `/dev/null`, stdout and
stderr to their own files, the exit status read directly — no pipe in front of any status.

**One change from the block as written, and what it is.** The block builds the repository by
hand (`git init`, an empty commit, `jigc setup`), and I ran exactly those commands rather than
`dev/jigc-rig`, so that the setup is the reporter's to the letter. The blob was made with
`git hash-object -w <file>` instead of the block's `printf | git hash-object -w --stdin`, to
keep a pipe out of a command whose status is read; the blob id is the same either way
(587be6b4c3f93f93c489c0111bba5596147a26cb).

### Candidate (`<W>/cand.7eTlSJ`)

| step | exit | observed |
|---|---|---|
| `git init -q repo` · `git -C repo commit -q --allow-empty -m base` | 0 · 0 | |
| `jigc setup` | 0 | |
| write `docs/zzz/orphan.md`, `git add`, `git commit -q --no-verify -m "an orphan"` | 0 · 0 | porcelain empty afterwards |
| **control:** `jigc validate` | **1** | stdout 1191 bytes, stderr 0 bytes; opens `blocking · schema-conformance.orphaned-instance — committed doc ` + the path `docs/zzz/orphan.md` |
| control, `--format json` | 1 | `"blocking_probes": ["schema-conformance"]`, one finding, `"report_only": false` |
| `git update-index --add --cacheinfo 100644,<blob>,bad<byte 0xFF>name.txt` | 0 | `git ls-files -z` holds the bytes `62 61 64 ff 6e 61 6d 65 2e 74 78 74`; porcelain: `AD "bad\377name.txt"` |
| **repro:** `jigc validate` | **0** | stdout 125 bytes, stderr 0 bytes — below |
| repro, `--format json` | 0 | `"blocking_probes": []`, `"findings": []`, `"report_only": true` |

The repro's stdout, whole:

```text
no findings — the committed store validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

### Attempts to refute it, each of which failed

| attempt | what was done | result |
|---|---|---|
| stale state | `git update-index --force-remove` of the entry, then `jigc validate` | exit 1; stdout `cmp`-identical to the control — the entry is the cause, and the store did not drift |
| the entry's absence from the working tree, not its name | the same staged-and-absent state under an ASCII name, `goodname.txt` (porcelain `AD goodname.txt`) | exit 1; stdout `cmp`-identical to the control — the byte is the cause, not the missing file |
| only a staged oddity | the 0xFF path committed (`git commit --no-verify`; porcelain then ` D "bad\377name.txt"`) | exit 0; stdout `cmp`-identical to the repro — a clone of such a history is in the same state |
| read through a pipe, or cut | every status read bare; outputs sized with `wc -c` and compared with `cmp` | 1191 / 125 bytes, as the reporter recorded for the control |
| another binary | `command -v jigc` before the drive; the installed hook's `jigc=` line names the same path | the candidate |

### Previous release (`<W>/prev.if6qIm`, driven by absolute path)

The same block on a fresh root: `setup` 0, control `jigc validate` **exit 1**, 1191 bytes; with
the entry, **exit 0**, 125 bytes, stderr empty. `cmp` against the candidate's files: control
stdout identical, repro stdout identical, repro stderr identical (both empty).

## Two facts that bound the finding's reach — driven, and narrower than the report says

1. **On this host the state exists only through git plumbing.** `touch` of a file with that name
   under `<W>` failed, exit 1, *Illegal byte sequence*: this file system refuses the name. So
   here the state is an index entry with no file behind it — made by `update-index`, or left by
   checking out a history that holds such a path. On a file system that admits the name it would
   be an ordinary tracked file; **that was not driven** (one platform).
2. **The installed pre-commit hook does not act on this finding at all.** The reporter's
   severity rests on *the pre-commit hook `jigc setup` installs runs this same sweep*. It does
   run the sweep, and it does not key on this finding: the hook's own header says it always
   exits 0 except over a staged out-of-band rename, and it warns only where `blocking_probes`
   names `doc-code`. Driven in the **control** state, where `jigc validate` exits 1:
   `git commit -q --allow-empty -m …` with the hook active exited 0, with 0 bytes on stdout and
   0 on stderr. So no commit that the hook would have stopped gets through because of this
   state; the cost is the false green of `jigc validate` itself and of whatever reads its exit.

## The class

`instance, unbounded`. Driven: one state (one index entry whose name is not UTF-8), one door
(`jigc validate`, store scope, text and `--format json`), one finding code
(`schema-conformance.orphaned-instance`), both binaries. I enumerated no consumers. The reporter's
count of the enumerator's call sites is theirs and was not re-derived here; I read only
`committed_markdown` and `task::git_capture` to see why the listing comes back empty — the
capture fails as a whole when git's output is not UTF-8, and the enumerator turns that failure
into an empty listing.

## The coverage claim

The block says `UNPINNED: found this round`. Checked by a text scan, not by reading every test:
fourteen files under `crates/cli/src`, `crates/cli/tests` and `tooling-tests` name
`orphaned-instance` or `orphaned_instance`; none of them holds `cacheinfo`, a `\xff` literal or
a bytes-built file name. `cacheinfo` appears in four files of the crate, none of them among the
fourteen. So nothing I found plants this state under this finding. A scan of names is what this
is; the claim stands as far as it reaches.

## Repro RC-9v — the block, in the pipeline's schema

```yaml
claim: "over a store holding a stamped committed file no doctype claims, `jigc validate` exits 1 with a blocking `schema-conformance.orphaned-instance`; with one more index entry whose name is not valid UTF-8 it exits 0 and prints `no findings — the committed store validates clean`"
verdict: "REFUTED as a blocker (breaks-no-clause) — the behaviour itself REPRODUCES, on both binaries"
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — control and repro byte-identical"
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
  - "write a file holding `x\n`; blob = git hash-object -w <that file>"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0; the argument is BYTES, not a string
repro:
  - ["jigc", "validate"]
  - ["jigc", "validate", "--format", "json"]
expect:
  - exit: 0
    stdout: "no findings — the committed store validates clean\n— jigc · run `jigc start` for orientation; all writes through `jigc`.\n"
    stderr: ""
  - exit: 0
    stdout_json: { "blocking_probes": [], "findings": [], "report_only": true, "scope": "store" }
controls:
  - "git update-index --force-remove of the entry -> `jigc validate` exit 1, stdout identical to the control"
  - "the same staged-and-absent entry under the ASCII name goodname.txt -> exit 1, stdout identical to the control"
  - "the 0xFF path committed with --no-verify -> exit 0, stdout identical to the repro"
observed: "<W>/cand.7eTlSJ/out and <W>/prev.if6qIm/out — control.out, repro.out, repro.json, reverse.out, ascii.out, committed.out, hook.err"
pinned-by: "UNPINNED: the `expect` above is the behaviour as it stands, which no ruling has made intended"
```

**Pinnable as it stands: no — for two reasons.**

1. **The `expect` is the defect.** A standing test over this block would fence the false green
   in. No ruling says an exit-0 clean is the intended answer when the committed set cannot be
   enumerated, so there is nothing yet for a pin to hold. Once the row is ruled, the block is
   ready either way: if it is fixed, it is the fix's red test with `expect` turned to a
   non-zero exit that names the cause; if a bound is declared, it pins the bound as written.
2. **One argument is bytes.** The index entry's name cannot be carried by a string list. A test
   has to build that argument from bytes, which is a Unix-only construction; the block marks the
   place with `<byte 0xFF>`.

## Left open

- **The reading of the `working-product` clause.** Read here by its instrument's two limbs —
  nothing that works on rc.24 stops working; every refusal's route works as printed. Whether
  *rely on* reaches a false green that the previous release prints too is the human's to say.
  If it does, this row is not `refuted`.
- **Whether a tracked path whose name is not UTF-8 is a supported layout.** Ruled nowhere I
  read, and under no declared bound of this run. The verdict above does not use it. On a file
  system that admits such a name the state needs no plumbing; that platform was not driven.
- **The block's variant, not driven:** `chmod 000 docs/zzz/orphan.md` in place of the index
  entry, which the reporter records as giving the same exit 0. It is a different mechanism (an
  unreadable file read as unstamped) and triage asked for the index entry.

## Bounds — what this verification did not do

- One platform, one git. Nothing on a file system that admits the name.
- No other consumer of the enumerator, no other finding code of the sweep, no other door.
- The route the control's finding prints was not followed; the finding makes no claim about it.
- `jigc task finalize` and every other gate were not driven under this state.

<!-- end of report -->
