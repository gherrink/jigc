# verify-real — `r1-p3-lowercase-address-other-doors-not-driven` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-lowercase-address-other-doors-not-driven`
- **door:** unlisted — the doors that take a `<type>:<slug>` address: the `#fragment` reads, the `--task` arm, the write verbs, `jigc rename`, `jigc unmanage`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `confirmed`
- **regression:** `false` — the same block, on fresh rigs, with the previous release's binary gives the same exits and, at every refusing step, the same bytes. Comparable: every door exists there.
- **basis:** on this case-insensitive volume `jigc doc set-slot` and `jigc doc set-field` accept `research:upper` over `docs/research/UPPER.md` at exit 0 and promise the copy is *re-promoted at finalize*; `jigc task finalize` then refuses `finalize.commit-rejected`, tells the reader to *fix the hook's complaint* where no hook spoke, and the re-run it prints refuses byte-identically — a refusal whose route does not work as printed. The `#fragment` reads, the `--task` read, `jigc rename` and `jigc unmanage` break no clause.
- **contested:** `false` — the finding argues no decision wrong, and the decisions I read rule the observed refusal a defect, not an intent.
- **class:** `instance, unbounded`. I drove four of the address doors and the one path door named; I enumerated no consumer set. The registry a fixer would derive the axis from is named under *What I drove*.

`<W>` is my own scratch directory, `<scratch>/verify-p3.InjZAw`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` stands for a rig's repository root where a binary printed it as an absolute path.

## What breaks the clause, and what does not

The finding is five doors under one sentence. They do not share a verdict, so I give each its own line before any table.

| door | what it does with the lower-cased address | the clause |
|---|---|---|
| `jigc doc show research:upper#<fragment>` | exit 0, serves the upper-cased file's slice; writes nothing | untouched — no refusal, no route |
| `jigc doc show research:upper --task <id>` | before a write: exit 1 `store.not-staged`, and its printed route runs at exit 0. After a write: exit 0, serves the staged copy | untouched — the one route works as printed |
| `jigc doc set-slot` · `jigc doc set-field` | exit 0; the upper-cased file is copied in under the identity `research:upper`; the tree outside `.jigc/` and the git index do not move | **broken, at the landing** — below |
| `jigc rename research:upper --to …` | exit 1 `rename.commit-rejected`, rolled back; its route is conditional (*resolve the cause above, then re-run*) and the re-run lands once the cause git names is resolved | untouched, on my reading — below |
| `jigc unmanage docs/research/upper.md` | exit 0; a truthful no-op where nothing is recorded, and a drop of the record where a write had made one; the file is never touched | untouched |

**The break.** A write verb acknowledges the write and says where it will end: *copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize*. `jigc task validate` then exits 0. `jigc task finalize` exits 1:

```
`git commit` was rejected (no commit was made):
On branch main
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
	modified:   docs/research/UPPER.md

no changes added to commit (use "git add" and/or "git commit -a")

task probe-task is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/probe-task/docs/`, and anything you had `git add`-ed is still in git's index. Fix the hook's complaint, then re-run `jigc task finalize probe-task`.
```

The re-run, as printed, exits 1 with the same bytes. No hook complained: the relayed text is git's own *no changes added to commit*, and the only hook in the rig is the warn-only `pre-commit` that `jigc setup` installs, which always exits 0. git's own hint leads nowhere either — the refusal restores the file, so there is no modification left to `git add` (driven: `git add docs/research/UPPER.md` exits 0, the index is unchanged, the re-run is byte-identical). That is the second half of the clause's instrument — *every refusal's route works as printed* (DECISIONS.md → *2026-10-04 — The exit rule, revised*, sharpening 2).

**A reader who assigns that refusal to `jigc task finalize`, a door this finding does not list, would grade the finding's own doors `breaks-no-clause` and file the refusal as a new row.** I did not, because the write verbs' acknowledgement names finalize as where the write ends, a staged write has no other observable outcome, and the refusal is reached only through the doors this finding names. The fact is stated here so that nobody has to find it in a table.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | that binary's path, exit 0 — asserted inside the drive script before every rig's first command, which stops the run if it differs |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: 1.0.0-rc.24. No build ran; nothing under `target/` was driven. Every rig was built with `dev/jigc-rig fresh --binary <that path>` and `SCRATCH=<W>`. git is 2.54.0 (Apple Git-157).

## The volume

**Case-insensitive.** Probed: a file written as `<W>/CaseProbe` answers `test -e <W>/caseprobe`; `git config --get core.ignorecase` prints `true` in every rig, set by `git init`. A case-sensitive volume was not driven.

## What I drove

The reporter left no repro block — the finding is item 5 of another verifier's *left open* list, one sentence. So the block is reconstructed: the smallest setup that reaches each named door with a lower-cased address over an upper-cased file. **What I changed against the sibling finding's plant:** its `docs/research/UPPER.md` was `# Upper\n`, which every door here refuses as unparseable before it does anything of its own; mine is a conformant `research` doc, as triage's instruction allows (*made conformant where the door needs it*):

```
---
date: 2026-10-08
schema-version: 1
---

# Upper

## Question

What does a lower-cased address reach?

## Findings

Nothing yet.

## Sources

None.
```

One script, run once per binary, builds every rig fresh and runs every step bare: exit status captured directly, stdout and stderr to separate files, and around each step the tree hashed file by file (everything outside `.git`, so `.jigc/` included), `git ls-files -s`, `git status --short` and `HEAD`. A route that is run *as printed* is lifted out of the captured stderr by its backticks and handed to `sh -c` unchanged.

| rig | plant | chain |
|---|---|---|
| W-C | `UPPER.md`, added and committed | the reads · `jigc start --workflow do-research … --slug probe-task` · the `--task` read and its route · `set-slot` · `set-field` · the commit doc's two leaves · `task validate` · `task finalize` · its printed re-run · `doc list` · `validate` · `unmanage docs/research/upper.md` twice |
| W-U | `UPPER.md`, untracked | the chain of W-C up to `validate` |
| W-K | as W-C, plus a code file staged after the mint | the chain of W-C up to `validate` |
| N-C · N-U | committed · untracked | `jigc rename research:upper --to "Lower Name"` · its printed re-run · the `--format json` arm |
| U-C | committed | `jigc unmanage` with `docs/research/upper.md`, `research:upper`, `docs/research/UPPER.md` |
| K-C | committed | `set-slot`, then `jigc task discard probe-task --force` |
| F-C · N-C2 | committed | the refused finalize / the refused rename, then the `git mv` that `jigc ingest` prints, a commit of it, then the refusal's own re-run |
| G-C | committed | the refused finalize, then git's own hint (`git add docs/research/UPPER.md`), then the re-run |
| D-C · D-U · DK-C | controls: `lower.md` and `research:lower`, committed · untracked · committed with a staged code file | the chain of W-C |

Candidate: 13 rigs. Previous release: 12 (every one but DK-C). Before the scripted runs I used three exploratory rigs on the candidate to learn the doors' argv and to take a conformant `research` body from the `refs-post-hoc` state; and the script's first pass on the candidate lifted the footer's `jigc start` where it should have lifted the route line's command, so I corrected the lifter and ran every rig again from nothing. Nothing in this report rests on the exploratory rigs or on that first pass.

**Doors not driven.** The registry of address-taking doors is `DOCTYPE_DOORS` filtered to `DoctypeArg::Address` (`crates/cli/src/cli.rs`; fenced by `crates/cli/tests/address_slug_head_axis.rs`) — ten rows as I read them: `rename`, `doc add-item`, `doc remove-item`, `doc retitle-item`, `doc rename`, `doc set-field`, `doc set-slot`, `doc show`, `task bind`, `milestone add-from-spec`. I drove four. That reading is where a fixer starts, not a count I derived.

### The reads — rig W-C, candidate

| step | argv | exit | what it printed | tree and index |
|---|---|---|---|---|
| r1 | `jigc doc show research:upper` | 0 | the doc's render: the planted bytes and one closing blank line | unchanged |
| r2 | `jigc doc show research:upper#question` | 0 | `What does a lower-cased address reach?` | unchanged |
| r3 | `jigc doc show research:upper#meta/date` | 0 | `2026-10-08` | unchanged |
| r4 | `jigc doc show research:upper#findings --format json` | 0 | `"Nothing yet."` | unchanged |
| t1 | `jigc doc show research:upper --task probe-task` | 1 | `store.not-staged` — *only its committed copy exists*; route `jigc doc show research:upper` | unchanged |
| t2 | `jigc doc show research:upper` — the route of t1, as printed | 0 | the render of r1 | unchanged |
| t3 | `jigc doc show research:upper#findings --task probe-task` (after w1) | 0 | the prose w1 wrote | unchanged |
| t4 | `jigc doc show research:upper#meta/date --task probe-task` (after w2) | 0 | `2026-10-07` | unchanged |

W-U gives the same exits and the same output: the read doors do not ask whether git holds the file.

### The write verbs and their landing — rigs W-C and W-U, candidate

| step | argv | exit | what it printed | tree and index |
|---|---|---|---|---|
| w1 | `jigc doc set-slot research:upper#findings --from-file <prose> --task probe-task` | 0 | `set slot research:upper#findings (51 chars) (copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)` and `advisory · file-state.baseline-adopt — baseline adopted: `docs/research/upper.md`` | `.jigc/` only: the staged copy `tasks/probe-task/docs/research:upper.md`, `provenance.json`, `roles.json`, and `state/file-state.json` with the one key `docs/research/upper.md`. `docs/`, the index and `HEAD` unchanged |
| w2 | `jigc doc set-field research:upper#meta/date --value 2026-10-07 --task probe-task` | 0 | `set research:upper#meta/date = 2026-10-07` | the staged copy only |
| v1 | `jigc task validate probe-task` | 0 | two advisories: `file-state.staged-copy` at `docs/research/upper.md`, `file-state.baseline-adopt` at `docs/research/UPPER.md` | the index cache `.jigc/index/edges.json` created; nothing else |
| f1 | `jigc task finalize probe-task` | 1 | the refusal quoted above (W-C); in W-U git's relayed text is *Untracked files: docs/ … nothing added to commit but untracked files present*, the frame and the route the same | **unchanged** — `UPPER.md` holds its planted bytes, the index its planted entry (W-C) or none (W-U), no commit |
| f2 | `jigc task finalize probe-task` — the re-run f1 prints | 1 | stderr byte-identical to f1 (`cmp`, exit 0) | unchanged |
| s1 | `jigc doc list` | 0 | `research:UPPER  docs/research/UPPER.md  unregistered` | unchanged |
| s2 | `jigc validate` | 1 | `schema-conformance.unadopted-instance` at `docs/research/UPPER.md` — *its name is not a doc id, so no `<type>:<slug>` address reaches it* | unchanged |

`--format json` (rig G-C) carries the same refusal as one finding: `"code": "finalize.commit-rejected"`, `"target": "task:probe-task"`, the same message, the same route.

**No byte is lost on this path.** In both arms the planted file's hash is the same before w1 and after f2; in W-U, where no git object holds it, that is the fact that matters for the first clause.

**How dead the dead end is** — three ways out, each driven on the candidate, none printed by f1:

- *git's own hint* (rig G-C): `git add docs/research/UPPER.md` exits 0 and changes nothing; the re-run is byte-identical to f1.
- *the repair `jigc ingest` prints for this file* (rig F-C): `git -C <repo> mv docs/research/UPPER.md docs/research/upper.md` exits 0 and stages the rename; after a commit of it the re-run exits **3** `finalize.base-mismatch` — *the moved history overlaps the task's work on `docs/research/upper.md`* — whose route is *resolve the overlap … or discard the task with `jigc task discard probe-task --force`*.
- *discard* (rig K-C): `jigc task discard probe-task --force` exits 0 and drops the edit.

So I found no sequence that lands the edit the write verb accepted.

### `jigc rename` — rigs N-C, N-U, N-C2, candidate

| step | argv | exit | what it printed | tree and index |
|---|---|---|---|---|
| n1 | `jigc rename research:upper --to "Lower Name"` | 1 | below | `.jigc/index/edges.json` (the cache) created; `docs/`, the index and `HEAD` unchanged |
| n2 | `jigc rename research:upper --to 'Lower Name'` — the re-run n1 prints | 1 | byte-identical to n1 | unchanged |

```
`git mv docs/research/upper.md docs/research/lower-name.md` failed: fatal: not under version control, source=docs/research/upper.md, destination=docs/research/lower-name.md

nothing was committed — the rename was rolled back, so `research:upper` still holds its original identity and every referrer still points at it. Resolve the cause above, then re-run `jigc rename research:upper --to 'Lower Name'`.
```

The untracked arm (N-U) prints the same bytes. The `--format json` arm carries it as `rename.commit-rejected` at `research:upper`. In rig N-C2, after the `git mv` that `jigc ingest` prints and a commit of it, the same re-run exits 0: `renamed research:upper -> research:lower-name (docs/research/upper.md -> docs/research/lower-name.md), repointed 0 referrer(s)`.

### `jigc unmanage` — rigs U-C and W-C, candidate

| step | argv | exit | what it printed | tree and index |
|---|---|---|---|---|
| U-C u1 | `jigc unmanage docs/research/upper.md` | 0 | `no-op: docs/research/upper.md is not managed (nothing to drop)` | the cache `edges.json` created; nothing else |
| U-C u2 | `jigc unmanage research:upper` | 0 | `no-op: research:upper is not managed (nothing to drop)` | unchanged |
| U-C u3 | `jigc unmanage docs/research/UPPER.md` | 0 | `no-op: docs/research/UPPER.md is not managed (nothing to drop)` | unchanged |
| W-C u1 (after w1) | `jigc unmanage docs/research/upper.md` | 0 | `unmanaged docs/research/upper.md (research:upper) — dropped its file-state baseline + forward edges; the file is left on disk. …` | `file-state.json` goes from the one key `docs/research/upper.md` to none; `UPPER.md` untouched |
| W-C u2 | the same again | 0 | the no-op line | unchanged |

`jigc unmanage` takes a path, not an address; the lower-cased **path** is what I handed it.

### The controls — rigs D-C, D-U, DK-C

With `docs/research/lower.md` and `research:lower` the chain of W-C exits 0 at every step but t1: `task finalize` prints `finalized <sha> — docs: record the probe finding`, `promoted docs/research/lower.md`, `1 file committed`; the index entry carries the new blob; `validate` exits 0; `unmanage docs/research/lower.md` drops the record; `jigc rename research:lower --to "Lower Name"` exits 0 and moves the file. So every red above is the spelling's, and each chain is sound where the address and the file agree.

## What triage asked, answered

- **The `#fragment` reads.** Served at exit 0; nothing written.
- **The `--task` read.** One refusal before a write, whose route works; the staged copy served after.
- **`set-slot`, `set-field`.** Accepted at exit 0. They write under `.jigc/` only. On the candidate the first of them also records a file-state baseline keyed at `docs/research/upper.md` — a spelling no directory entry and no index entry has.
- **`jigc rename`.** Refused at exit 1 from inside `git mv`; rolled back whole.
- **`jigc unmanage`.** Exit 0; never touches the file.
- **The tree and the index before and after each.** In the tables. Along the lower-cased chain no second directory entry ever appeared under `docs/research`, the index never gained or lost an entry, and — outside the staged-file arm under *Left open*, item 1 — `UPPER.md` never changed a byte.
- **Both binaries.** Yes — *Step 4*.

## Step 2 — is it what the finding says?

The finding says only that these doors were not driven; what it asks is whether the sibling's defect has members here. **It has one that reaches the clause.** I tried the ways such a result is usually wrong: the rigs and the scratch directory are mine and new; no status was read through a pipe; nothing was cut before it was compared; the binary is the candidate by hash and by `command -v`, checked inside the script; the control separates the spelling from the chain; and the one mistake my own script made (the lifter) was found by reading the lifted route, and the run repeated.

**No settled decision intends the refusal; two rule it a defect.**

- design/finalize.md → *Empty commit*: a task with nothing to commit *is told so here rather than reaching git's own "nothing to commit" and having it routed as a hook's rejection* (M55 completion audit, CR3). Rig W-C is git's own *no changes added to commit*, routed as a hook's rejection.
- DECISIONS.md → *2026-09-17 — M52 Increment 3 / T5*: a non-hook refusal framed with *Fix the hook's complaint* is *a law-1 lie and an unfollowable route*. The same entry declares the bound this case falls through: exit 1 is also git's refusal to record an empty commit, *which an exit code cannot separate from a hook and each door already discriminates before the seam*. Here the door's discriminator did not catch it. Why not is inference, not something I drove: the door counts the promoted doc as a change, and the git text shows the bytes arriving in the worktree under `UPPER.md` with nothing staged.
- design/validation.md → the `ingest.unaddressable-identity` row, and DECISIONS.md → *2026-09-15 — M51 Increment 9 / T4*: a file at a managed home whose name is not a doc id is one *no `<type>:<slug>` address reaches*. On this volume four address doors reach it. I cite this as the rule the behaviour sits against, and rest the verdict on the route, not on it.
- design/command-output-contract.md → *The first-touch copy-in note*: the *re-promoted at finalize* sentence is a stated part of the write verbs' acknowledgement — which is why the landing is these doors' business.
- DECISIONS.md, the M50 workbench entry (*a case-sensitive predicate is a hole on the majority filesystem*): it concerns another predicate, and shows that this volume is treated as an ordinary one and not as a planted state. I found no declared bound for case-insensitive volumes in `design/`, `DECISIONS.md` or `implementation/decisions-pending.md`.

**`jigc rename`.** I read `jigc rename --help` and design/surface-contract.md's code table (`rename.commit-rejected`); I found no rule the refusal violates. Its sentence *`research:upper` still holds its original identity* is said of a file `jigc doc list` calls `unregistered` — wording.

## Step 3 — does it break the clause, inside its scope?

**Yes — at the write verbs' landing, and nowhere else.** The clause is `working-product`; its instrument has two halves.

- *No command that works on rc.24 in a supported layout stops working.* Untouched: every exit here is the previous release's too.
- *Every refusal's route works as printed.* The refusal of `jigc task finalize probe-task` prints one route: fix the hook's complaint, then re-run. There is no hook complaint to fix, and the re-run refuses with the same bytes. The route does not work as printed.

**Why `jigc rename`'s refusal is on the other side of that line.** It too says *resolve the cause, then re-run*, and its bare re-run is also byte-identical. But its cause is true and is named — git's own `not under version control, source=docs/research/upper.md` — and resolving it by the move `jigc ingest` prints for this very file makes the printed re-run land (rig N-C2). A conditional route whose condition is real and satisfiable works as printed; one whose condition names a complaint nobody made does not. A stricter reader could hold that rename's refusal never names the spelling as the cause and grade it with finalize's; the driven facts are above for that reading.

**Scope.** The layout is ordinary: one hand-written, conformant file at a doctype's home, on the platform's default volume, no git configuration of mine. The address is the one the `store.malformed-slug` refusal of `research:UPPER` describes when it says *use lowercase letters*. Nothing here is a race or a planted state, and the run declares no bound.

**The clause it does not touch.** `no-lost-files`, on the path the verdict rests on: no byte outside `.jigc/` was written or removed at any exit status, in the committed arm or the untracked one. The staged-file arm is another matter and is *Left open*, item 1.

## Step 4 — the regression fact

The same script, fresh rigs, the previous release's binary by its absolute path, its directory first on `PATH`.

| step | candidate | previous release |
|---|---|---|
| the reads r1–r4, W-C · W-U | 0 · 0 | 0 · 0 |
| t1 `--task` read · its route | 1 · 0 | 1 · 0 |
| w1 `set-slot` · w2 `set-field` | 0 · 0 | 0 · 0 |
| v1 `task validate` | 0 | 0 |
| f1 `task finalize`, W-C · W-U | 1 · 1 | 1 · 1 |
| f2 the printed re-run | 1 · 1, same bytes as f1 | 1 · 1, same bytes as f1 |
| n1 `rename`, N-C · N-U · n2 | 1 · 1 · 1 | 1 · 1 · 1 |
| `unmanage`, U-C u1–u3 | 0 · 0 · 0 | 0 · 0 · 0 |
| controls D-C, D-U: finalize · rename | 0 · 0 | 0 · 0 |

All 351 captured exit, stdout and stderr files of the eleven shared rigs were compared with the rig's root replaced by one token; the comparer reported the files that differ, so it can tell a difference. **The stderr of f1 in W-C and in W-U and of n1 in N-C is byte-identical between the binaries with no replacement at all** (`cmp`, exit 0). The differences, all of them:

- the candidate's `set-slot` adds the `file-state.baseline-adopt` advisory and writes that record at copy-in; the previous release writes none there (in the control too — it is the candidate's copy-in record, not the spelling's);
- so W-C's `unmanage docs/research/upper.md` after the write drops a record on the candidate and is the no-op on the previous release;
- the control's `task validate` and `finalize` carry that advisory on the previous release and not on the candidate;
- commit ids.

Red on both: `regression: false`.

## Coverage

The finding's sentence is about what a verifier drove, not about what the suites test; it makes no coverage claim. For the block's `pinned-by` I looked in the suites, not in a diff. Twelve suite files under `crates/cli/tests` name `finalize.commit-rejected` or the hook sentence, one names `rename.commit-rejected`; `address_slug_head_axis.rs` plants `docs/research/OddName.md` and addresses it as `research:OddName`, never in lower case. Of the seven files under `crates/cli/tests` and `tooling-tests` that carry a case-folding term (`ignorecase`, `case-insensitive`, `case-fold`, `to_uppercase` and their variants), none is among those thirteen. A suite that reaches the cell by planting two spellings as plain literals is outside that search: this is *none found*, not *none exists*.

## Repro V-1

```yaml
claim: "on a case-insensitive volume `jigc doc set-slot research:upper#findings` over a conformant docs/research/UPPER.md exits 0 saying the copy is re-promoted at finalize; `jigc task finalize` then exits 1 `finalize.commit-rejected` under the hook frame with no hook involved, and the re-run it prints refuses byte-identically; the planted file and the index are unchanged throughout"
verdict: CONFIRMED
regression: false          # the previous release gives the same exits and the same refusal bytes
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d
precondition: "the volume is case-insensitive — a file written as `CaseProbe` answers a stat of `caseprobe`"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = a conformant research doc: front matter `date: 2026-10-08`, `schema-version: 1`; `# Upper`; `## Question`, `## Findings`, `## Sources`, each with one line of prose"
  - "arm committed: git add docs/research/UPPER.md, git commit · arm untracked: add nothing"
  - ["jigc", "start", "--workflow", "do-research", "probe the address", "--slug", "probe-task"]
repro:
  - ["jigc", "doc", "show", "research:upper#question"]
  - ["jigc", "doc", "show", "research:upper", "--task", "probe-task"]
  - ["jigc", "doc", "show", "research:upper"]                                   # the route of step 2, as printed
  - ["jigc", "doc", "set-slot", "research:upper#findings", "--from-file", "<prose>", "--task", "probe-task"]
  - ["jigc", "doc", "set-field", "research:upper#meta/date", "--value", "2026-10-07", "--task", "probe-task"]
  - ["jigc", "doc", "set-field", "commit:probe-task#header/type", "--value", "docs", "--task", "probe-task"]
  - ["jigc", "doc", "set-slot", "commit:probe-task#summary", "--from-file", "<summary>", "--task", "probe-task"]
  - ["jigc", "task", "validate", "probe-task"]
  - ["jigc", "task", "finalize", "probe-task"]
  - ["jigc", "task", "finalize", "probe-task"]                                  # the re-run step 9 prints
expect:
  - exit: 0
    stdout: "What does a lower-cased address reach?\n"
  - exit: 1
    stderr_contains: ["blocking · store.not-staged — `research:upper` is not staged in this task", "route: `jigc doc show research:upper`"]
  - exit: 0
  - exit: 0
    stdout_contains: ["set slot research:upper#findings", "copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize"]
  - exit: 0
    stdout_contains: ["set research:upper#meta/date = 2026-10-07"]
  - exit: 0
  - exit: 0
  - exit: 0
  - exit: 1
    stdout: ""          # committed arm; the untracked arm prints the left-out manifest naming docs/research/UPPER.md
    stderr_contains: ["`git commit` was rejected (no commit was made):", "Fix the hook's complaint, then re-run `jigc task finalize probe-task`."]
  - exit: 1
    stderr: "byte-identical to step 9"
  tree: "docs/research holds the one entry UPPER.md with its planted bytes after every step; `git ls-files -s` and HEAD are what they were after the setup; steps 1 to 10 write under .jigc/ only"
control: "with docs/research/lower.md and research:lower the same ten steps exit 0 · 1 · 0 · 0 · 0 · 0 · 0 · 0 · 0, step 9 printing `promoted docs/research/lower.md`, and there is no step 10"
also-driven: "`jigc rename research:upper --to 'Lower Name'` exits 1 `rename.commit-rejected` from `git mv`, rolled back; `jigc unmanage docs/research/upper.md` exits 0 — both graded breaks-no-clause above"
observed: "<W>/jigc-rig-fresh-P3Fdwk (W-C), -qkPPXi (W-U), -M5s2CZ (W-K), -TvAQu2 (N-C), -M79Hzw (N-U), -Z5AG8m (U-C), -F1i2wc (D-C), -nnfn4m (D-U), -MOrZLz (K-C), -YPoA0e (F-C), -uSMw7z (N-C2), -sk1bIr (G-C), -gLVAoe (DK-C); previous release <W>/jigc-rig-fresh-zWjxnC (W-C), -EjibRf (W-U), -Lpy9or (W-K), -dfFWqY (N-C), -mTYft0 (N-U), -7wlN47 (U-C), -izovsN (D-C), -91nThD (D-U), -qy4QMU (K-C), -FjZNJG (F-C), -U8mQyK (N-C2), -nbQTq3 (G-C)"
pinned-by: "UNPINNED: found this round; no suite naming either refusal carries a case-folding cell"
```

**Pinnable as it stands: no.** One reason, about the block and not about the fact: its expectations hold only on a case-insensitive volume, and the CI runner's is case-sensitive, so a test made from it needs the precondition as a probe that selects the arm — and what the candidate answers on a case-sensitive volume is undriven and is not in the block. Nothing else stands in the way: every asserted string is relative to the repository, so no rig path has to be replaced. With the probe it fits beside `commit_rejected_axis`, whose lifted-re-run shape it mirrors.

## Left open — hit on the way, not pursued

1. **With a file staged after the mint, the same finalize exits 0 and commits without the doc.** Rig W-K, both binaries, same bytes but the commit id: `finalized <sha> — docs: record the probe finding`, `added probe.rs`, `1 file committed`, `left-out (unstaged/untracked — git add to include): docs/research/UPPER.md`, and no `promoted` line. The commit holds `probe.rs` alone; the edit made through `set-slot` and `set-field` sits in the worktree as ` M docs/research/UPPER.md`; the task's working area is gone; `file-state.json` holds two keys for the one file, `docs/research/UPPER.md` at the old hash and `docs/research/upper.md` at the new; and `jigc validate` then exits 1 with `blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/research/UPPER.md` differs from the recorded state`, routing the reader to *review the out-of-band edit* — of an edit jigc wrote. The control (rig DK-C, candidate) commits both files and prints `promoted docs/research/lower.md`. Door `jigc task finalize`. No byte is lost — the edit is on disk and the left-out line names the file — so I do not claim the first clause; which clause it reaches, if any, is a grade I was not sent to give. The untracked arm of this cell is undriven.
2. **After the repair `jigc ingest` prints, the task that holds the write cannot land either.** Rig F-C, both binaries: `finalize.base-mismatch`, exit 3, whose route is *resolve the overlap* with no command, or discard.
3. **The candidate's copy-in records a file-state baseline under a spelling no file has.** `docs/research/upper.md`, written by `set-slot` at exit 0 and announced as `file-state.baseline-adopt`; it survives `jigc task discard --force` (rig K-C) and is dropped by `jigc unmanage docs/research/upper.md`. The previous release writes no record at copy-in. It lives under the gitignored `.jigc/state/`.
4. **`jigc task validate` reports two advisories under two spellings for one file** — `file-state.staged-copy` at `docs/research/upper.md` and `file-state.baseline-adopt` at `docs/research/UPPER.md` — and exits 0 ahead of a finalize that cannot land.
5. **`jigc validate` and `jigc ingest` say of this file that no `<type>:<slug>` address reaches it; on this volume `doc show`, `set-slot`, `set-field` and `rename` do.** The sibling report's item 2, now with the write doors behind it.
6. **`jigc rename` refuses only from inside `git mv`,** after its own work and its rollback, and its sentence credits `research:upper` with an *original identity*. Wording and ordering, as far as I looked.
7. **`jigc unmanage research:upper`** — an address where the door takes a path — exits 0 as a no-op that echoes the operand.
8. **Six address doors of the registry were not driven** with a lower-cased address over an upper-cased file: `doc add-item`, `doc remove-item`, `doc retitle-item`, `doc rename`, `task bind`, `milestone add-from-spec`. `research` has no repeatable section, so the three item verbs need another doctype.
9. **A case-sensitive volume was not driven**, on either binary.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else. I built nothing, edited nothing, staged nothing and committed nothing in it. Everything I wrote besides this report is under `<W>`: the drive scripts, the captured output, and the rigs. The only commits made are inside those rigs — the install commit `jigc setup` makes in each, mine for each committed plant and for the move in rigs F-C and N-C2, and the ones `jigc` itself landed in the control rigs, in W-K and in N-C2.

<!-- end of report -->
