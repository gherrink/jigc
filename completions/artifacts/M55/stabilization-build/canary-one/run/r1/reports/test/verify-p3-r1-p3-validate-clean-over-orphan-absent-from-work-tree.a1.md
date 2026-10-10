# verify-real — `r1-p3-validate-clean-over-orphan-absent-from-work-tree`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed over by triage with the
grade *unclear*: door `jigc validate`, clause said to be broken `working-product`. It has no
block of its own: it is the third bullet under *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-validate-unreadable-orphan-variant-not-driven.a1.md`
— *the absent-file sibling, hit on the way, driven once, not pursued* — and triage asked for
that report's `Repro RC-9u` setup with the orphan then removed from the work tree and not
staged, `jigc validate` in both formats on both binaries, set against the comment on
`carries_stamp` and `design/validation.md`. Of that report I read what the prompt handed me, as
this finding's source; I read no other finding's report and no other verifier's.

## Verdict

**`refuted` — basis `intended`.** The behaviour reproduces exactly as the bullet says, on the
candidate and, byte for byte, on the previous release. It is the behaviour a settled decision
intends, and it is the control's own printed route working as printed.

- **What reproduces.** Over a store holding a stamped committed file no doctype claims,
  `jigc validate` exits 1 with a blocking `schema-conformance.orphaned-instance`. Remove that
  one file from the work tree and stage nothing (`git status --porcelain` reads
  ` D docs/zzz/orphan.md`; the index and `HEAD` still hold it) and the same command exits 0 and
  prints *no findings — the committed store validates clean*, with nothing on stderr;
  `--format json` exits 0 with `"findings": []`, `"blocking_probes": []`, `"report_only": true`.
  Put the file back and the exit 1 is back, its output `cmp`-identical to the control.
- **Why it is intended — the state is the end of the finding's own route.** The control prints,
  as the third exit of its route: *take it out of jigc's world: `jigc unmanage docs/zzz/orphan.md`,
  then delete the file or its `schema-version:` stamp (`unmanage` drops the baseline and leaves
  the bytes, so the stamp alone keeps this finding alive)*. That exit was settled, with its
  reason, in `DECISIONS.md` → *2026-09-15 — M51 Increment 8 / T3: `schema-conformance.orphaned-instance`,
  and the route that says what its second exit leaves behind*: `unmanage` alone *does not clear
  the finding … so the exit ships naming the act that finishes it: delete the file or its
  `schema-version:` stamp*. `design/validation.md` → the `schema-conformance.orphaned-instance`
  row carries the same route (*and then delete the file or its stamp … a route that, followed
  exactly, changes nothing is M46's PT-1 defect*). I drove that route as printed, on a fresh
  root: `jigc unmanage docs/zzz/orphan.md` exits 0 (*no-op: … is not managed (nothing to
  drop)*) and `jigc validate` still exits 1, identical to the control; then **delete the stamp**,
  unstaged → exit 0; file restored → exit 1; then **delete the file**, unstaged → exit 0, stdout
  `cmp`-identical to this finding's repro. The state the finding describes and the state the
  route leaves are one state, and the green is the route clearing its finding. The route names
  no `git add` and no commit, and its other finishing act — deleting the stamp — is a work-tree
  edit that clears the finding unstaged in the same way.
- **The comment on `carries_stamp` states the posture, and it is as old as the finding code.**
  `crates/cli/src/orphan.rs`: *A file git tracks but the worktree no longer holds reads as
  **unstamped** — the sweep is a statement about bytes it could read, never an assertion built
  on an absent file.* `git log -L` over the function puts that sentence in the commit that
  introduced the code (`00d7198b`, 2026-09-15), the commit of the decision cited above. The same
  posture is stated for the sibling predicate in the same file (`vacated_homes`: *a path that
  git tracks but the worktree no longer holds reads as vacated, because the census reads the
  worktree … the same answer every other consumer of the census gets*), and
  `design/validation.md` → the `orphaned-instance` row and the `home-vacated` row both define
  their subject through `engine::index::committed_instances`, *the census every other
  committed-instance consumer reads*.
- **The clause, which the grade *unclear* sent me to answer.** `working-product` is the second
  clause of `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, and that entry gives it one
  instrument: *no command that works on rc.24 in a supported layout stops working, and every
  refusal's route works as printed*. Neither limb is broken, and the second is positively met.
  First limb: control, repro and restored control are byte-identical between the candidate and
  the previous release, twelve files of twelve. Second limb: the refusal in play is the
  control's, and its route works as printed — this finding's state is what following it
  produces. A change that made the sweep block over the absent file would be the change that
  breaks that limb: *delete the file* would then leave the finding standing, the PT-1 shape the
  decision exists to prevent. So were the basis `intended` not accepted, the verdict would
  still be `refuted` as a blocker, on `breaks-no-clause`.
- **`contested`: false.** The bullet does not argue that the decision is wrong; it says the
  comment *names and intends* the case and that no design section it read carries the posture.
- **Regression fact.** Not owed with `refuted`. The previous release was driven because triage
  asked for both binaries and because the instrument's first limb cannot be answered without
  it: the same on both, byte-identical.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (label c1, commit
  eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- The candidate's directory went first on `PATH`; `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`, exit 0, and it was printed again at the head of each drive on the
  candidate. The previous release was driven by its absolute path in every call.
- No `cargo build`, nothing under `target/`. The clone: `git status --porcelain` read
  `?? completions/artifacts/canary-one/r1/`, `HEAD` eeffe347324f83a51d1ae83d5f254e73c3f1ea3a,
  branch `fix/canary-one`, before and after; nothing was edited, staged or committed there.

## How it was driven

Host: macOS 26.6.2, git 2.54.0 (Apple Git-157), uid 501. One directory of my own under the
scratch root, `<scratch>/verify-absent.fibb2O` (written `<W>`), and under it three fresh roots,
each from `mktemp -d`: `<W>/cand.GJmkyr` (the block, candidate), `<W>/cand-route.xMbtam` (the
control's route, candidate) and `<W>/prev.LH10Ns` (both, previous release). Nothing of the
reporter's or of an earlier verifier's was reused, and nothing was torn down.

Environment of every invocation: `HOME=<root>/home` (empty before `setup`),
`GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four
`GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables. Every jigc call: stdin from `/dev/null`, stdout
and stderr to their own files, the exit status read directly — no pipe in front of any status.

**What I changed from the block as written.** The setup is `Repro RC-9u`'s, command for
command, built by hand as that block has it (`git init`, an empty commit, `jigc setup`, the
orphan committed with `--no-verify`) rather than with `dev/jigc-rig`, so that the setup is the
block's to the letter. Its `chmod 000` step is replaced by the step triage names — the file
removed from the work tree, nothing staged — and its `cat` plant-control by three of this
state's own: the path does not exist, the index still lists it, `HEAD` still holds its blob.

### Candidate, the block (`<W>/cand.GJmkyr`)

| step | exit | observed |
|---|---|---|
| `git init -q repo` · `git -C repo commit -q --allow-empty -m base` | 0 · 0 | - |
| `jigc setup` | 0 | - |
| write `docs/zzz/orphan.md`, `git add`, `git commit -q --no-verify -m "an orphan"` | 0 · 0 | porcelain empty afterwards |
| **control:** `jigc validate` | **1** | stdout 1191 bytes, stderr 0 bytes; opens `blocking · schema-conformance.orphaned-instance — committed doc ` + the path; its route is quoted in the verdict |
| control, `--format json` | 1 | 1243 bytes; `"blocking_probes": ["schema-conformance"]`, one finding keyed at `docs/zzz/orphan.md`, `"report_only": false` |
| `rm docs/zzz/orphan.md` (one literal path) | 0 | - |
| plant-controls: `test -e docs/zzz/orphan.md` · `git ls-files -- docs/zzz/orphan.md` · `git cat-file -e HEAD:docs/zzz/orphan.md` · `git diff --cached --quiet` | 1 · 0 · 0 · 0 | absent on disk; listed by the index; held by `HEAD`; nothing staged. Porcelain ` D docs/zzz/orphan.md` |
| **repro:** `jigc validate` | **0** | stdout 125 bytes, stderr 0 bytes — below |
| repro, `--format json` | **0** | 112 bytes, stderr 0 bytes — below |
| porcelain after the two calls | - | ` D docs/zzz/orphan.md`, unchanged — the verb wrote nothing tracked |
| `git checkout -- docs/zzz/orphan.md` | 0 | porcelain empty again |
| **control, file restored:** `jigc validate` | **1** | 1191 bytes; `cmp` against the first control: identical |
| control, file restored, `--format json` | 1 | 1243 bytes; `cmp` against the first control: identical |

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

### Attempts to show the finding wrong as an observation, each of which failed

All on the candidate, in the same root, after the block above.

| attempt | what was done | result |
|---|---|---|
| stale state | the file restored with `git checkout`, then `jigc validate` | exit 1; `cmp`-identical to the control, text and JSON — the absence is the cause, and the store did not drift |
| the plant did not take | `test -e` on the path; the index and `HEAD` asked | absent on disk, present in both — the state is the one the finding names, not a staged or committed deletion |
| the empty directory left behind | the file removed and `docs/zzz` removed with `rmdir` | exit 0; stdout `cmp`-identical to the repro |
| moved, not deleted | the file moved to a path outside the repository | exit 0; stdout `cmp`-identical to the repro |
| read through a pipe, or cut | every status read bare; outputs sized with `wc -c` and compared with `cmp` | 1191 / 125 bytes for the two states |
| another binary | `command -v jigc` at the head of each drive | the candidate |

Two contrasts, driven to place the state rather than to refute it:

- **The deletion staged** (`git rm -q`, porcelain `D  docs/zzz/orphan.md`, the index no longer
  lists the path): exit 0, stdout `cmp`-identical to the repro. Unstaged again and restored:
  exit 1, identical to the control.
- **A second stamped orphan committed beside the first**, `docs/zzz/second.md`. Both on disk:
  exit 1, two findings. The first removed, unstaged: exit 1, **one** finding,
  `docs/zzz/second.md`; `orphan.md` is named nowhere on stdout and stderr is empty. The answer
  is per file, and it is the answer the route promises for a file that was deleted.

### Candidate, the control's route as printed (`<W>/cand-route.xMbtam`)

The same setup on a fresh root, then the third exit of the route the control prints.

| step | exit | observed |
|---|---|---|
| control: `jigc validate` | 1 | as above |
| `jigc unmanage docs/zzz/orphan.md` | 0 | `no-op: docs/zzz/orphan.md is not managed (nothing to drop)`; stderr empty; porcelain empty |
| `jigc validate` | 1 | `cmp`-identical to the control — `unmanage` alone does not clear it, as the route says |
| *delete its stamp*: the file rewritten without its front matter, unstaged (porcelain ` M`) | - | - |
| `jigc validate` | **0** | *no findings — the committed store validates clean*; `cmp`-identical to the block's repro |
| `git checkout -- docs/zzz/orphan.md`, `jigc validate` | 1 | the finding is back |
| *delete the file*: `rm docs/zzz/orphan.md`, unstaged (porcelain ` D`) | 0 | - |
| `jigc validate` | **0** | `cmp`-identical to the block's repro — **this is the finding's state** |
| the deletion committed (`git commit -q --no-verify -am …`), `jigc validate` | 0 | `cmp`-identical to the line above |

On this setup `unmanage` is a no-op — the orphan was committed by hand and has no baseline —
so the state the route leaves and the state triage described differ in nothing.

### Previous release (`<W>/prev.LH10Ns`, driven by absolute path)

The same block on a fresh root, every jigc call `<scratch>/bin/previous-91834b5e011d/jigc`:
`setup` 0; control `jigc validate` **exit 1** and `--format json` exit 1; the file removed,
`test -e` exit 1, the index still listing it, porcelain ` D docs/zzz/orphan.md`, nothing
staged; repro `jigc validate` **exit 0** and `--format json` **exit 0**; file restored,
`jigc validate` **exit 1** and `--format json` exit 1.

`cmp` of each of the twelve output files against the candidate's — stdout and stderr of the
control, the repro and the restored control, text and JSON: **all twelve identical** (1191 / 0
/ 1243 / 0 bytes for the controls, 125 / 0 / 112 / 0 for the repro).

The route, in the same root afterwards: `unmanage` exit 0; `validate` exit 1; stamp deleted
unstaged, `validate` exit 0; file deleted unstaged, `validate` exit 0. The four stdout files
`cmp`-identical to the candidate's.

## Is it what the finding says, against the design that owns it

- **The observation: yes.** Exit 0 and the clean line, in both formats, with the file absent
  from the work tree and still in the index and in `HEAD`.
- **A defect: no.** Three places say what the sweep does with such a file, and they agree.
  The decision of 2026-09-15 settles *delete the file* as an act that finishes the route. The
  design row carries that route. The source comment, written in the decision's commit, says how
  the reader behaves: an absent file reads as unstamped.
- **What the design prose does not say in so many words.** `design/validation.md` → the
  `orphaned-instance` row states three legs — inside a declared home, stamped, unclaimed — and
  does not itself say *the stamp is read from the work tree's bytes*. The bullet is right about
  that. The row's route presupposes it (a deleted stamp or a deleted file clears the finding),
  and the comment says it; the sentence is missing from the doc, not the decision. Under
  *Left open*, as a documentation gap and not as this finding.
- **The sibling that does block, for contrast.** At a doctype's **exact declared home** an
  uncommitted removal is a blocking `schema-conformance.home-vacated`
  (`crates/cli/src/orphan.rs`, the test `an_uncommitted_removal_reads_the_blob_head_still_carries`;
  `design/validation.md` → that row). `docs/zzz/orphan.md` is at no doctype's declared home —
  that is what makes it an orphan — so that rule does not reach it, and nothing I read says it
  should. Not driven; cited from the source and the design row.

## The class

`instance, unbounded`. Driven: one state (one tracked, stamped, unclaimed file inside a managed
home, absent from the work tree, nothing staged), one door (`jigc validate`, store scope, text
and `--format json`), one finding code, both binaries, one platform. I enumerated no consumers.
By a text search of `crates/cli/src`, `carries_stamp(` has one production call site (inside
`orphaned_instances`) and `orphaned_instances(` has two — the store sweep in `cli.rs` and the
listing in `doc.rs` — and I drove the first only. That is a search of two names, not a
derivation of the class.

## Coverage

The bullet makes no coverage claim. For the block's `pinned-by`, a scan of names, not a reading
of every test: fifteen files under `crates/cli/src`, `crates/cli/tests`, `crates/engine/src`
and `tooling-tests` name the finding code. Their work-tree removals of a single file are of
`CHANGELOG.md` at a declared home (the vacated-home sibling, in `orphan.rs` and
`home_vacated.rs`), a git hook, a symlink and a snapshot; their `git rm` calls are staged and
committed. None removes an unclaimed stamped file from the work tree and asks the sweep. So
the sentence of the comment on `carries_stamp` is pinned by nothing I found, and neither is the
*delete the file* exit of the route.

## Repro RC-9a — the block, in the pipeline's schema

```yaml
claim: "over a store holding a stamped committed file no doctype claims, with that file removed from the work tree and nothing staged, `jigc validate` exits 0 and prints `no findings — the committed store validates clean`, in text and in JSON — and that is a defect breaking `working-product`"
verdict: "REFUTED (intended) — the exit 0 REPRODUCES on both binaries, and it is the finding's own route, `delete the file`, working as printed"
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — control, repro and restored control byte-identical, twelve files of twelve"
decision: "DECISIONS.md -> 2026-09-15 M51 Increment 8 / T3; design/validation.md -> the schema-conformance.orphaned-instance row, its route; crates/cli/src/orphan.rs -> the comment on carries_stamp"
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
  - control: ["jigc", "validate"]                       # exit 1; stdout opens `blocking · schema-conformance.orphaned-instance — committed doc `, 1191 bytes; stderr empty; the route names `delete the file or its schema-version: stamp`
  - "remove docs/zzz/orphan.md from the work tree; stage nothing"
  - plant-control: ["git", "ls-files", "--", "docs/zzz/orphan.md"]   # prints the path: the index still holds it — else the cell is a staged deletion, another state
  - plant-control: ["git", "status", "--porcelain"]                  # ` D docs/zzz/orphan.md`
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
  - "git checkout -- docs/zzz/orphan.md -> `jigc validate` exit 1 and `--format json` exit 1, each stdout identical to the control"
  - "the route as printed, fresh root: `jigc unmanage docs/zzz/orphan.md` exit 0 (a no-op here) -> `jigc validate` still exit 1; the stamp deleted, unstaged -> exit 0; restored -> exit 1; the file deleted, unstaged -> exit 0, identical to the repro"
  - "a second stamped orphan beside it, on disk -> exit 1 with ONE finding, the file that is there"
observed: "<W>/cand.GJmkyr/out, <W>/cand-route.xMbtam/out and <W>/prev.LH10Ns/out — control, repro, restored (each .out, .json and their .err); on the candidate also f-nodir.out, b-staged.out, g-moved.out, c-both.json, c-one.json, c-one.out; in the route roots unmanage.out, after-unmanage.out, stamp-gone.out, file-gone.out, file-gone-committed.out"
pinned-by: "UNPINNED: found this round; no suite removes an unclaimed stamped file from the work tree and asks the sweep (a scan of names, above)"
```

**Pinnable as it stands: yes.** Every step is a plain argv or a one-line file write, nothing
depends on the caller's uid or on the platform, and the `expect` is the behaviour the decision
intends, so a standing test over it fences a settled fact and not an open question. The most
useful form is the second control rather than the bare repro: the route followed as printed —
`unmanage`, still exit 1; then *delete the file*, exit 0 — which pins the route's promise and
the comment's sentence in one arm. The setup is spelled as explicit steps; I did not check
which fixture state of the builder would replace them.

## Left open

- **`design/validation.md` does not say where the stamp is read from.** The `orphaned-instance`
  row never states that the stamp leg reads the work tree's bytes, though its route depends on
  it and the source comment says it. A documentation gap; not driven as a finding.
- **The green line's word *committed*.** With the deletion unstaged, `HEAD` and the index still
  hold the stamped file, and the line printed is *the committed store validates clean*. Whether
  that wording is a law-1 matter under `design/surface-contract.md` was not examined; it is a
  claim about a sentence, not about the exit, and not this finding's.
- **The partial answer.** With one orphan on disk beside one that is absent, the sweep names
  the one on disk only. The same mechanism and, by the same decision, the same intent; noted,
  not pursued.
- **`jigc doc list` under this state.** The second production consumer of the same enumerator.
  Another door; not driven.
- **Whether the basis is `intended` or `breaks-no-clause`.** I read the decision of 2026-09-15
  as settling it. It names the act — *delete the file* — and not the words *before any commit*;
  the comment written with it supplies those. If that reading is not accepted, the verdict is
  still `refuted` as a blocker, on `breaks-no-clause`, by the twelve `cmp`s and the route.

## Bounds — what this verification did not do

- One platform, one git, one uid. Nothing on Linux, no sparse checkout, no `skip-worktree` or
  `assume-unchanged` bit, no linked worktree.
- No other consumer of the stamp reader or of the committed listing, no other finding code of
  the sweep, no other door. `jigc task finalize`, the installed pre-commit hook and every other
  gate were not driven under this state.
- The route's first two exits (`jigc ingest`, re-adding a pack) were not followed; only the
  third, which is the one that produces this state.
- The vacated-home sibling is cited from the source and the design row, not driven.
- Working files of mine outside the scratch root: six small text files of search output under
  the system temp directory, none of them evidence; every output the verdict rests on is under
  `<W>`.

<!-- end of report -->
