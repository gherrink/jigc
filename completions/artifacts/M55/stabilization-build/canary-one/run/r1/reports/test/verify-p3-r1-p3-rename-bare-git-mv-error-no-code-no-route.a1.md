# verify-real — `r1-p3-rename-bare-git-mv-error-no-code-no-route` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-rename-bare-git-mv-error-no-code-no-route`
- **door:** `jigc rename`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `refuted`
- **basis:** `intended` — the door's stderr is not a bare `git mv` error: it is the frame DECISIONS.md → *2026-09-14 — M51 Increment 2 / T5* rules for a failure inside the rename transaction (git's cause verbatim, what survived, the door's own re-run), it carries `rename.commit-rejected` in the invocation log and as a keyed, routed finding under `--format json`, and its route ran to exit 0 once the cause git names was resolved.
- **regression:** not stated — the verdict is not `confirmed`. The fact, for whoever wants it: the previous release prints the same bytes (`cmp` exit 0).
- **contested:** `false` — the finding argues no decision wrong.
- **class:** `instance, unbounded`. I enumerated no consumer set. What was driven is under *What I drove*.

`<W>` is my own scratch directory, `<scratch>/verify-p3-rename.JAt8gv`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` is the repository of whichever rig a row names.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path in each of the three driving shells; the driver stops before the first command when it does not |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: `jigc 1.0.0-rc.24`. No build ran; nothing under `target/` was driven. git on this machine: 2.54.0 (Apple Git-157). The volume folds case (a probe in `<W>`: a file made as `CaseProbe` answers to `caseprobe`).

## What I drove

Three fresh rigs, each `dev/jigc-rig fresh --binary <that binary's path>` with `SCRATCH=<W>`. Every exit status was read bare; stdout and stderr went to separate files under `<W>`; the route was lifted out of the emitted bytes and run through `sh -c`.

| rig | binary | what was driven |
|---|---|---|
| C (`<W>/jigc-rig-fresh-74vTon`) | candidate | Repro V-1 as written, all six steps, then `jigc doc list`, the door again under `--format json`, and the door a third time unchanged |
| P (`<W>/jigc-rig-fresh-gLIcfS`) | previous | the same |
| D (`<W>/jigc-rig-fresh-xf1NMb`) | candidate | the smallest setup that reaches the door — the committed plant and no task at all — with the invocation log switched on, then a control over an absent id, then the route's two halves |

The plant is the finding's: `docs/research/UPPER.md` = `# Upper\n`, added and committed. The `doc author` payloads are the block's `stdin`, under the titles `Upper` and `Upper notes`.

### 1. Repro V-1, steps 1 to 6 (rigs C and P)

| # | argv | exit, both binaries |
|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/UPPER.md --as research` | 0 |
| 2 | `jigc doc author research --from-file - --task <task>`, title `Upper` | 1 — `write.non-reparseable` |
| 3 | the same, title `Upper notes` | 1 — `write.identity-change` |
| 4 | `jigc doc rename research:upper --to 'Upper notes' --task <task>` | 1 — `write.identity-change` |
| 5 | `jigc task discard <task> --force` | 0 — stdout *discarded task … dropped staged edits to: commit:… (transient), research:upper*, stderr empty |
| 6 | `jigc rename research:upper --to 'Upper notes'` | **1** |

Step 6's stdout is empty. Its stderr, whole — 409 bytes, three lines, the second of them empty:

```text
`git mv docs/research/upper.md docs/research/upper-notes.md` failed: fatal: not under version control, source=docs/research/upper.md, destination=docs/research/upper-notes.md

nothing was committed — the rename was rolled back, so `research:upper` still holds its original identity and every referrer still points at it. Resolve the cause above, then re-run `jigc rename research:upper --to 'Upper notes'`.
```

The report this finding came from quoted the first line and the last sentence with an ellipsis between them. What the ellipsis cut is the frame's middle: the statement of what survived.

After step 6, in both rigs: `git status --short --untracked-files=all` empty, `HEAD` where the plant's commit left it, `git ls-files docs/research` = `docs/research/UPPER.md`, the file's bytes `# Upper\n`, and `jigc doc list` (exit 0) still `research:UPPER  docs/research/UPPER.md  unregistered`.

Candidate against previous release, with the task id normalized: `cmp` exit 0 on stdout and stderr of steps 2, 3, 4 and 6, of the listing, and of the `--format json` run below. Step 6's stderr is byte-identical raw.

### 2. The same door, three more ways

| argv | rig | exit | what it gave |
|---|---|---|---|
| `jigc rename research:upper --to 'Upper notes' --format json` | C, P | 1 | stdout empty; stderr ONE findings envelope, `schema_version` 3, one finding: `severity` `blocking` · `code` `rename.commit-rejected` · `key` `{code: rename.commit-rejected, target: research:upper}` · `message` = git's line above · `location.address` `research:upper` · `route` = the frame's last paragraph above |
| the door again, unchanged | C, P | 1 | stderr byte-identical to step 6's |
| the door with no task ever minted (the plant committed, nothing else) | D | 1 | stderr byte-identical to rig C's step 6 (`cmp` exit 0) — steps 1 to 5 change nothing at this door |
| `jigc rename research:nosuch --to 'Upper notes'` (control: an id absent under every spelling) | D | 1 | `blocking · store.not-found` — *no managed doc `research:nosuch` to rename (expected at docs/research/nosuch.md)* · `at: research:nosuch` · a route at `jigc describe` |

The invocation log, rig D (`jigc config set invocation-log true`, exit 0, committed before the door so the tree is clean): the record of the failed rename reads `"exit_code":1,"finding_codes":[],"output_bytes":409,"error_code":"rename.commit-rejected"`.

### 3. The route, as printed — both halves (rig D, candidate)

The route is *Resolve the cause above, then re-run* the one command it spells. The cause above is git's: the source path `docs/research/upper.md` is not under version control.

| half | what ran | exit |
|---|---|---|
| resolve the cause | `git mv docs/research/UPPER.md docs/research/upper.md`, then `git commit -q -m …` — after it `git ls-files docs/research` = `docs/research/upper.md` | 0, 0 |
| re-run, lifted from step 6's bytes | `sh -c "jigc rename research:upper --to 'Upper notes'"` | **0** — *renamed research:upper -> research:upper-notes (docs/research/upper.md -> docs/research/upper-notes.md), repointed 0 referrer(s)* |

After it: `git status` empty, one commit `rename docs/research/upper.md -> docs/research/upper-notes.md`, the file `# Upper notes\n`.

The re-run without the first half fails identically (section 2, second row). That is a route with a condition holding its condition, not a route that fails as printed.

## Step 2 — is it what the finding says?

The finding's three statements, against the bytes:

| the finding says | driven |
|---|---|
| a bare `git mv` error | **No.** git's line is the first of three parts; the second states what survived (*nothing was committed — the rename was rolled back*), and it is true of the tree: status empty, `HEAD` unmoved, the plant's bytes unchanged. |
| no code | **On the text arm, yes — and by design. On the two machine surfaces, no.** The log record carries `error_code: rename.commit-rejected`; the `--format json` arm carries the same string as a blocking finding with a `(code, target)` key. |
| no route but itself | **There is a route, and it is the door's own re-run — which is what the design says this route is.** It ran to exit 0 behind its stated condition (section 3). |

And against the docs that own the behaviour:

- **DECISIONS.md → *2026-09-14 — M51 Increment 2 / T5: a non-hook boundary refusal keeps the frame that was built for it (N20)*.** A failure inside a committing door's transaction, after its rollback has run, is framed with three things — *the cause verbatim, what survived, and this door's own copy-runnable re-run* — and *names no cause at all*: *"Resolve the cause above" is true whatever the cause was, and git's own bytes are printed verbatim immediately above it.* The entry names *the rename transaction* among the doors marked, and *a stale `.git/index.lock` … meeting … the rename's `git mv`* as an ordinary cause. Per door it asks for *exactly one route lifted verbatim and equal to that door's own re-run* and the door's `error_code` *read back from the invocation log*. Each of those is what section 1 and section 2 show.
- **design/finalize.md → 6. Commit, the survivable frame.** git's bytes stay *verbatim and unwrapped*; jigc's sentence goes *around* them. The frame is *the whole committing family's*, parameterized on a state-truth clause, the door's own re-run argv, and *the door's own route-exempt error identity*.
- **design/surface-contract.md → The route fence** re-affirms the *hook-rejection route-exempt error identity* as an exemption from the route floor on its own rationale; **→ The error-code namespace** lists `rename.commit-rejected` for `jigc rename`, says *not every non-commit is a `Finding`*, and — since M52 — that the door identities *are also findings* on the `--format json` arm, *one name for one event*. So a text arm with no `blocking · <code>` line and no `at:` line, beside a log and a JSON arm that both name the code, is the contracted shape.
- **design/write-commands.md → `jigc rename`, step 5** is the transaction with its own rollback: *a failed rename leaves the store byte-and-record identical to before*. Driven, it does. The paragraph *Every refusal names itself and routes (M49 — PT-A)* is about the twelve states the **gate** declines, declared as `RefusalKind`; a `git mv` that fails is inside the transaction behind that gate and is not one of them.
- **The suite that pins the shape** (read, not run — I build nothing): `commit_rejected_axis::every_committing_door_keeps_its_frame_when_no_hook_spoke`, registered in `crates/cli/tests/groups/g_flow.rs`. Its `jigc rename` cell plants a stale index lock so the door's `git mv` fails, and asserts a non-zero exit, `HEAD` untouched, the state clause present and true of the repository, exactly one `then re-run` route equal to the door's own argv, the log's `error_code` equal to the door's identity, and **zero** `finding_codes` — *a non-hook commit failure is an operational error, not a Finding*.

I tried the ways a refutation like this is usually wrong. The rigs and the scratch root are mine and new. The block ran as written before anything was reconstructed, and the reconstruction gave the same bytes. No exit status was read through a pipe and no decisive output was cut: the stderr above is the whole file, by byte count. The binary is the candidate by hash and by `command -v`. And the frame was not taken on its word — its state clause was checked against the tree, and its route was run.

## Step 3 — does it break the clause, inside its scope?

The clause is DECISIONS.md → *2026-10-04 — The exit rule, revised*, second clause, by its instrument: *no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed*.

- **First half: not broken.** The two binaries agree byte for byte at this door.
- **Second half: not broken by this door's refusal.** Its route has two halves and says so; followed in order it lands (section 3). I drove one reading of *resolve the cause* — the literal one, putting the path git names under version control — and the choice of reading is the operator's, as it is for every cause this frame is printed over.

So the row does not stand as a blocker of its own. The step this row was cut from — the report's step 8, where the route printed by `jigc doc rename` exits 1 — is that route failing as printed, and it is weighed where it was reported, under the adoption row. This row asked a narrower thing: whether the shape of `jigc rename`'s own answer is a defect. It is the shape a settled decision built.

**What is real here and is not this row's claim**, said so that the refutation is not read wider than it is: in this cell the frame speaks of `research:upper` as a doc that *still holds its original identity*, while the listing holds no such doc — it holds `research:UPPER`, `unregistered`. The door's gate took the foreign file for its target because the two names are one file on this volume. That is the ground of the adoption row, not a property of the frame; it is under *Left open* as what I saw of it at this door.

## Step 4 — the regression fact

Not owed: the verdict is not `confirmed`. Triage asked for both binaries, so the fact is on the page: rig P, previous release, fresh rig, the same block — the same exits at every step and `cmp` exit 0 against the candidate's captures. Section 3 (the route's two halves), the control and the log read were driven on the candidate only.

## Step 5 — coverage

The finding makes no coverage claim. For the block below, from the suites and not from any diff: **42** suite files under `crates/cli/tests` and `tooling-tests` spell the argv word `"rename"`; **1** of them holds a literal `docs/<home>/<name>.md` whose name carries an upper-case letter — `crates/cli/tests/address_slug_head_axis.rs`, whose test over `docs/research/OddName.md` and `docs/decisions/OldDoc.md` drives `jigc doc list` and `jigc validate` and never `jigc rename` under the lower-cased id. A suite that composes such a path some other way is outside this count. So the frame's shape at this door is pinned, by the suite test named above over a different cause; this cause — a source whose tracked name differs from the slug by case — is driven at this door by no suite I found.

## Repro R-1

```yaml
claim: "`jigc rename` fails with a bare `git mv` error — no finding code, no `at:`, and no route but a re-run of the command that just failed"
verdict: REFUTED
basis: intended
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the first two commands of `repro` driven there, the same exits, the same bytes
requires: "a volume on which `UPPER.md` and `upper.md` are one file"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: a research note"]
repro:
  - ["jigc", "rename", "research:upper", "--to", "Upper notes"]
  - ["jigc", "rename", "research:upper", "--to", "Upper notes", "--format", "json"]
  - ["git", "mv", "docs/research/UPPER.md", "docs/research/upper.md"]            # the route's first half: the cause git named
  - ["git", "commit", "-q", "-m", "docs: the note's name, lower-cased"]
  - ["jigc", "rename", "research:upper", "--to", "Upper notes"]                  # the route's second half, as printed
expect:
  - exit: 1
    stdout: ""
    stderr: "`git mv docs/research/upper.md docs/research/upper-notes.md` failed: fatal: not under version control, source=docs/research/upper.md, destination=docs/research/upper-notes.md\n\nnothing was committed — the rename was rolled back, so `research:upper` still holds its original identity and every referrer still points at it. Resolve the cause above, then re-run `jigc rename research:upper --to 'Upper notes'`.\n"
    tree: "`git status --short` empty, HEAD unmoved, docs/research/UPPER.md = `# Upper\n`"
  - exit: 1
    stdout: ""
    stderr_json: { "findings": [ { "severity": "blocking", "code": "rename.commit-rejected", "key": { "code": "rename.commit-rejected", "target": "research:upper" } } ] }
  - exit: 0
  - exit: 0
  - exit: 0
    stdout_contains: "renamed research:upper -> research:upper-notes (docs/research/upper.md -> docs/research/upper-notes.md), repointed 0 referrer(s)"
    tree: "`git status --short` empty; docs/research/upper-notes.md = `# Upper notes\n`"
log: "with the `invocation-log` knob on, the first command's record carries `error_code: rename.commit-rejected` and an empty `finding_codes`"
where: "<W>/jigc-rig-fresh-74vTon (candidate, the block's first two commands after Repro V-1's steps 1 to 5); <W>/jigc-rig-fresh-gLIcfS (previous, the same); <W>/jigc-rig-fresh-xf1NMb (candidate, the block as written here without its second command, the log knob set and committed before it)"
pinned-by: "commit_rejected_axis::every_committing_door_keeps_its_frame_when_no_hook_spoke — the frame's shape at this door, over a stale index lock; UNPINNED for this cause: no suite drives the door over a source whose tracked name differs from its slug by case"
```

**Pinnable as it stands: no.** Its first two expectations hold only where the volume folds case; on a volume that does not, I expect the first command to answer `store.not-found` instead — a guess, flagged as one, not driven. The fact that does not depend on the volume — a `git mv` failing inside this door's transaction is framed, logged under `rename.commit-rejected` and routed at the door's own re-run — is the one the named suite test already asserts, and a second test of it would pin nothing new. The last three commands also pin an end state (*Left open*, 1) that nobody has ruled wanted; a test taken from this block should stop after the second command until that is ruled.

No one rig ran the block's five commands in a row, and the report says so rather than leave it to be found: rigs C and P ran its first two (behind Repro V-1's steps 1 to 5, which section 2 shows change nothing at this door), and rig D ran its first, third, fourth and fifth, with the log knob set and committed before the first.

## Left open — hit on the way, not pursued

1. **`jigc rename` moves a file jigc never adopted.** In rig D, once `docs/research/upper.md` was the tracked name, the door renamed it at exit 0, rewrote its heading and committed the move — while `jigc doc list` shows the result as `research:upper-notes … unregistered` and `jigc validate` exits 1 over it with `schema-conformance.unadopted-instance`. The door's refusal for an absent id says *no managed doc*; whether its target must be a managed one is the design's to say. Candidate only.
2. **At this door the frame's state clause names `research:upper`, and the listing holds no such doc.** The gate accepted the id because `docs/research/upper.md` answers on this volume; the control over an absent id is refused `store.not-found` before any transaction. It is the same ground as the adoption row's cell, seen from this door.
3. **The cause line does not say that the tracked name differs only in case.** git's words are relayed verbatim, as the frame is built to do; an operator has to see for themselves that `UPPER.md` is the file meant.
4. **A volume that does not fold case was not driven**, on either binary.
5. **The route's two halves, the control and the log record were driven on the candidate only.**

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my three rigs under `<W>`: the install commit `jigc setup` makes in each, the plant's commit in each, and in rig D the knob's commit, the case-only move's commit and the commit the driven `jigc rename` made. Beside the rigs, `<W>` holds my two payload files, two driver scripts and the captured outputs; one list of suite file names, made for the count in step 5, sits under the system temp directory.

<!-- end of report -->
