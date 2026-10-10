# verify-real — `r1-p3-doc-show-lowercase-address-case-sensitive-volume` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-doc-show-lowercase-address-case-sensitive-volume`
- **door:** `jigc doc show`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `refuted`
- **basis:** `does-not-reproduce` — on a case-sensitive volume the lowercase address does not reach the upper-cased file: `jigc doc show research:upper` over `docs/research/UPPER.md` exits 1 with `store.not-found`, which is true of the directory and is the refusal the design gives an address nothing answers; no route there carries a lower-cased operand, so the dead end of the lowercase-address report's Repro V-1 has nothing to start from. Both binaries, both arms, byte-identical.
- **regression:** not stated — it goes with `confirmed` only. The fact triage asked for is below all the same: the previous release answers every step with the same exit and the same bytes.
- **contested:** `false` — the finding argues no decision wrong.
- **class:** `instance, unbounded`. What I drove is listed below; I enumerated no consumer set of the address-to-path mechanism.

`<W>` is my own scratch directory, `<scratch>/verify-p3-cs.RssRAG`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` stands for a rig's repository root wherever a binary printed it as an absolute path; I shortened it here and compared bytes only after the same replacement.

## What was handed to me, and what I answer

The finding is item 4 of another report's *Left open* list: *a case-sensitive volume was not driven, on either binary*. It asserts no behaviour. Triage's question is what that report's Repro V-1 does on a volume that does not fold case, on both binaries, and whether that breaks `working-product`.

The answer has three parts, and I keep them apart.

- **The dead end of V-1 is absent on this volume.** It needs the lowercase address to open the upper-cased file. Here it does not.
- **What the candidate does instead is the intended refusal**, and the commands that refusal prints work as printed.
- **Nothing differs between the binaries**, so neither half of the clause's instrument is touched.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path in every rig, and a string comparison with the expected path exited 0 in rigs C, D, E, F and H; in rig A it was read by eye |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: 1.0.0-rc.24. No build ran; nothing under `target/` was driven. Every rig was built with `dev/jigc-rig fresh --binary <that path>`. git is 2.54.0 (Apple Git-157).

## The volume

**Case-sensitive, probed and not inferred.** A sparse disk image, 200 MB, file system `Case-sensitive APFS`, created with `hdiutil create` as `<W>/cs.sparseimage` and attached at `<W>/mnt`.

| probe | exit | reading |
|---|---|---|
| write `<W>/mnt/CaseProbe`, then `test -e <W>/mnt/caseprobe` | 1 | the image does not fold case |
| `test -e <W>/mnt/CaseProbe` | 0 | the file is there under its own spelling |
| the same pair on `<W>` itself, the host volume | 0 | the control: the host volume folds case |
| `diskutil info <W>/mnt` | 0 | `File System Personality: Case-sensitive APFS` |
| `git config --get core.ignorecase` in each rig on the image | 1, prints nothing | `git init` set no folding there; in the host-volume rig it prints `true` |

The image is macOS's case-sensitive APFS. A Linux file system was not driven.

## What I drove

Six fresh rigs, none of them another agent's. Every exit status was read bare; stdout and stderr went to separate files under `<W>/out/<rig>/`. The tree was hashed file by file (everything outside `.git`) and the index and `HEAD` were read before and after the steps that could write.

| rig | binary | volume | plant | address |
|---|---|---|---|---|
| A | candidate | case-sensitive | `docs/research/UPPER.md` = `# Upper\n`, untracked | `research:upper` |
| C | candidate | case-sensitive | the same, added and committed | `research:upper` |
| D | candidate | case-sensitive | control: `docs/research/lower.md`, untracked | `research:lower` |
| E | previous release | case-sensitive | as A | `research:upper` |
| F | previous release | case-sensitive | as C | `research:upper` |
| H | candidate | host, case-folding | as A — the positive control | `research:upper` |

**What I changed from V-1 as written.** Nothing in its setup or its five argvs. On this volume its steps 2 to 4 are no longer commands a route printed — the refusal of step 1 prints other ones — so I ran them as V-1 spells them, and say so. I added `jigc doc list` before step 1 and `jigc task list`, the one literal command the new refusal prints, after it.

### Rigs A and C — candidate, case-sensitive

| step | argv | exit A · C | what it printed | tree, index, HEAD after |
|---|---|---|---|---|
| 0 | `jigc doc list` | 0 · 0 | stdout below; stderr empty | unchanged |
| 1 | `jigc doc show research:upper` | 1 · 1 | stdout empty; stderr below | unchanged |
| 1r | `jigc task list` — printed by step 1's route | 0 · 0 | `jigc task list — no active tasks` | unchanged |
| 2 | `jigc migrate <repo>/docs/research/upper.md --as research` — V-1's step 2, typed, printed by nothing here | 1 · 1 | stdout empty; stderr below | unchanged |
| 3 | `git -C <repo> add -- docs/research/upper.md` — V-1's step 3, typed | 128 · 128 | `fatal: pathspec 'docs/research/upper.md' did not match any files` | index identical (`cmp`, exit 0) |
| 4 | the re-run of step 2 | 1 · 1 | stderr byte-identical to step 2 (`cmp`, exit 0) | unchanged |
| 5 | `jigc ingest` | 0 · 0 | names `docs/research/UPPER.md`, prints a `jigc migrate` under that spelling; stderr empty | writes `.jigc/index/edges.json` and its lock, nothing else |

Step 0, stdout:

```
id  path  state
research:UPPER  docs/research/UPPER.md  unregistered
```

Step 1, stderr:

```
blocking · store.not-found — could not read `research:upper` at `docs/research/upper.md`: No such file or directory (os error 2)
  at: research:upper
  route: create the referenced doc, or fix the reference to an existing one; a doc staged in an open task is not committed yet — read it with `jigc doc show research:upper --task <task-id>` (find the task id with `jigc task list`)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

Steps 2 and 4, stderr:

```
could not read the foreign `research` source at `<repo>/docs/research/upper.md`
  route: check the path, then re-run `jigc migrate <path> --as research` with a readable file
```

Step 5, the row that matters:

```
needs-reconcile docs/research/UPPER.md → research
  blocking · conformance.section-missing — required section heading `## question` is missing
  at: docs/research/UPPER.md
  route: `jigc migrate <repo>/docs/research/UPPER.md --as research` — it opens the `migrate-research` workflow, which rewrites the file to conformant shape and adopts it at finalize
```

The directory held one entry throughout, `UPPER.md`, 8 bytes. In rig C the committed entry `docs/research/UPPER.md` kept its blob and no commit was made.

In rig A, three neighbours, all exit 1 and all without a write: `jigc doc show research:upper --format json` gives a findings envelope on stderr with `"code": "store.not-found"`, the same message and the same route; `jigc doc show research:nothere` gives the same refusal with the other slug, word for word; `jigc doc show research:UPPER` gives `store.malformed-slug`. `jigc validate` exits 1 with `schema-conformance.unadopted-instance` at `docs/research/UPPER.md` — *its name is not a doc id, so no `<type>:<slug>` address reaches it* — and the adoption route under the true spelling. On this volume that sentence is true.

### Rig D — the control on the same volume

`docs/research/lower.md`, untracked, and `research:lower`: exits 1 · 1 · 0 · **0** for V-1's steps 1 to 4. Step 1 is `store.unparseable` with the route *adopt — run `jigc ingest` … or `jigc migrate <repo>/docs/research/lower.md --as research`*; step 2 is `migrate.source-untracked` with its `git add` route; the `git add` stages the file (`A  docs/research/lower.md`); the re-run mints `migrate-research-docs-research-lower-<hash>` and writes eight files under `.jigc/tasks/<id>/`. The argv I typed for steps 2 to 4 is the one each refusal printed, character for character after the root is replaced. So the chain is sound on this volume where the address and the file agree, and my plant and my driver reach the door V-1 reaches.

### Rig H — the positive control on the host volume

The same driver, the same plant, the candidate, on the volume that folds case: exits 1 · 1 · 0 · 1. Step 1 is `store.unparseable` at `docs/research/upper.md`, step 2 `migrate.source-untracked`, the printed `git add` exits 0 with the index unchanged, the re-run refuses with the same bytes. That is V-1, reproduced. So the difference between rig A and V-1 is the volume and nothing in how I drove it.

## Step 2 — is it what the finding says?

**The finding says nothing but that this was undriven; V-1's claim does not hold here.** I tried to make it hold: the same plant, the same argvs, a driver that reproduces V-1 on the host volume (rig H), a committed arm as well as an untracked one, and both binaries. On the case-sensitive volume the lowercase address opens no file, `doc show` never parses `UPPER.md`, and the refusal that carried the lower-cased `jigc migrate` operand is not the one printed.

**What is printed is what the design says to print.** design/doc-read-surface.md, the read-side block list: an address nothing answers is a read-side block with a non-zero exit and a route, and *the `store.not-found` route names the staged read — `jigc doc show <addr> --task <task-id>`*. The refusal's message is true of the directory: no entry `upper.md` exists. `doc list` and `validate` name the file by its one spelling and `ingest` prints the working adoption command under it; the split to adoption is the same document's bullet *`doc show`'s block on an unregistered instance routes to adoption*, which rig D shows firing where the address does reach the file.

## Step 3 — does it break the clause, inside its scope?

**No.** The clause is `working-product`; its instrument has two halves (DECISIONS.md → *2026-10-04 — The exit rule, revised*, sharpening 2).

- *No command that works on rc.24 in a supported layout stops working.* Every step of the block gives the same exit and the same bytes on both binaries (below). Untouched.
- *Every refusal's route works as printed.* The route of step 1 prints two commands. `jigc task list` exits 0 and answers the question the route sends it — no task is open, so there is no staged copy to read. `jigc doc show research:upper --task <task-id>` carries a placeholder and is offered for the case of an open task; with none open there is no id to put in it, and I did not mint one. Its other two clauses are prose. Steps 2 to 4 are not commands any route printed on this volume, so their refusing is no route failing; the `could not read` refusal is also true, and its route is prose with a `<path>` placeholder.

**Scope.** The layout is ordinary and inside the clause's scope: one hand-written file at a doctype's home on a case-sensitive volume, no git configuration of mine. Nothing was planted to force a state.

**The clause it does not touch either.** `no-lost-files`: steps 1 to 4 wrote no file, staged nothing and made no commit, in any rig on the image; `ingest` wrote the gitignored index cache only.

## The regression fact triage asked for

It is not a field of this return, which carries `refuted`. As a measurement: rigs E and F, fresh, built and driven with the previous release's binary by its absolute path, its directory first on `PATH`.

| step | candidate (A · C) | previous release (E · F) |
|---|---|---|
| `doc list` | 0 · 0 | 0 · 0 |
| `doc show research:upper` | 1 · 1 | 1 · 1 |
| `task list` | 0 · 0 | 0 · 0 |
| `jigc migrate …/upper.md --as research` | 1 · 1 | 1 · 1 |
| `git add -- docs/research/upper.md` | 128 · 128 | 128 · 128 |
| the re-run | 1 · 1 | 1 · 1 |
| `jigc ingest` | 0 · 0 | 0 · 0 |

With each rig's root replaced by one token, stdout and stderr of all seven steps are byte-identical between the binaries, A against E and C against F — 28 comparisons, `cmp` exit 0 each. The comparer was checked against a pair that differs (rig A's step-1 stderr against rig H's) and exited 1.

## Coverage

The finding makes no coverage claim. For the block's `pinned-by` I looked in the suites, not in a diff: **17** suite files under `crates/cli/tests` and `tooling-tests` name `store.not-found`; in none of them does a line carry a case-folding term about a volume (`ignorecase`, `case-insensitive`, `case-sensitive`, `case-fold`) — the two hits of the wider search, in `count_fences.rs` and `fixed_identity_axis.rs`, are string helpers. The term search does find hits elsewhere in the suites (5 files), so it was able to. A suite that reaches the cell by planting two spellings as plain literals is outside that search: *none found*, not *none exists*.

## Repro V-1-cs

```yaml
claim: "on a case-sensitive volume `jigc doc show research:upper` over docs/research/UPPER.md does not reach the file: it refuses `store.not-found` at docs/research/upper.md, prints no adoption route and no lower-cased `jigc migrate`, and writes nothing; `jigc ingest` and `jigc validate` name the file by its one spelling"
verdict: REFUTED            # as a break of working-product: does-not-reproduce on this volume
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — same exits, same bytes
precondition: "the volume is case-sensitive — a file written as `CaseProbe` does NOT answer a stat of `caseprobe`"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - "arm untracked: add nothing · arm committed: git add docs/research/UPPER.md, git commit"
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "show", "research:upper"]
  - ["jigc", "task", "list"]                                                    # printed by step 2's route
  - ["jigc", "migrate", "<repo>/docs/research/upper.md", "--as", "research"]   # V-1's step 2; no route prints it here
  - ["git", "-C", "<repo>", "add", "--", "docs/research/upper.md"]              # V-1's step 3
  - ["jigc", "ingest"]
expect:
  - exit: 0
    stderr: ""
    stdout_contains: ["research:UPPER  docs/research/UPPER.md  unregistered"]
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · store.not-found — could not read `research:upper` at `docs/research/upper.md`", "read it with `jigc doc show research:upper --task <task-id>` (find the task id with `jigc task list`)"]
    stderr_absent: ["store.unparseable", "jigc migrate", "jigc ingest"]
  - exit: 0
    stdout_contains: ["jigc task list — no active tasks"]
  - exit: 1
    stdout: ""
    stderr_contains: ["could not read the foreign `research` source at `<repo>/docs/research/upper.md`"]
    stderr_absent: ["migrate.source-untracked"]
  - exit: 128
    stderr_contains: ["pathspec 'docs/research/upper.md' did not match any files"]
    assert: "`git ls-files -s` is what it was before the step"
  - exit: 0
    stderr: ""
    stdout_contains: ["needs-reconcile docs/research/UPPER.md → research", "route: `jigc migrate <repo>/docs/research/UPPER.md --as research`"]
  tree: "docs/research holds the one entry UPPER.md, its 8 bytes unchanged, after every step; steps 1 to 5 write no file at all"
control: "on the same volume, docs/research/lower.md and research:lower: `doc show` refuses `store.unparseable` with the adoption route, and that route's `jigc migrate`, its printed `git add` and its printed re-run exit 1 · 0 · 0, the re-run minting a task"
observed: "inside <W>/cs.sparseimage: jigc-rig-fresh-8Gsmyp (A), -QOUHJM (C), -iWBj70 (D); previous release -6YQjRR (E), -Ge3oH5 (F). On the host volume: <W>/host/jigc-rig-fresh-xcmJQ0 (H). Captured output: <W>/out/<rig>/"
pinned-by: "UNPINNED: found this round; no suite naming `store.not-found` carries a case-mismatch cell"
```

**Pinnable as it stands: no.** Two reasons, both about the block. Its expectations hold only on a case-sensitive volume, and the gate also runs on the case-folding volume of a development machine, where step 2 answers `store.unparseable` instead — so a test made from it needs the precondition as a probe that selects the arm, with the lowercase-address report's V-1 as the other arm. And the operand of steps 4 and 5 is an absolute path under the rig's root, so the assertions compare after replacing that root. With those two it is one test with two arms.

## Left open — hit on the way, not pursued

1. **`jigc doc list` prints `research:UPPER` in its `id` column, and no door takes that as an address.** Rigs A, C, E, F and H: the row is `research:UPPER  docs/research/UPPER.md  unregistered`; `jigc doc show research:UPPER` refuses `store.malformed-slug`, and that refusal's route is *`jigc doc list` lists the committed docs and the identity each one carries*; `jigc validate` says of the same file that its name is not a doc id. Same on both volumes and both binaries. Door `jigc doc list`. I read no design text that settles whether the `id` of an unregistered row whose file name is not a slug is meant to be printed in that form.
2. **The cell only a case-sensitive volume can hold was not driven:** `UPPER.md` and `upper.md` side by side at one home — what `doc list`, `doc show research:upper`, `ingest` and a migration of either do there.
3. **A repository whose `core.ignorecase` disagrees with its volume was not driven** — a clone made on a case-folding volume and used on a case-sensitive one, or the reverse.
4. **A Linux file system was not driven.** The volume here is macOS's case-sensitive APFS; the CI runner's is another one.
5. **`jigc doc show research:upper --task <task-id>`, the placeholder command of the `store.not-found` route, was not driven with an open task.**
6. **Other doors that take a `<type>:<slug>` address were not driven on this volume** — the `#fragment` reads, the write verbs, `jigc rename`, `jigc unmanage`.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my rigs: the install commit `jigc setup` makes in each, and one of mine in each of rigs C and F for the *committed* arm. Rig D holds one minted, unfinalized task.

The disk image was detached after the last drive (`hdiutil detach <W>/mnt`, exit 0, *ejected*); `<W>/mnt` is an empty directory again and the five rigs on it live inside `<W>/cs.sparseimage`, which `hdiutil attach -nobrowse -mountpoint <W>/mnt <W>/cs.sparseimage` brings back. One stray file of mine, a search result written to the system temp directory by a mistyped redirect, was removed by its literal path in the next command.

<!-- end of report -->
