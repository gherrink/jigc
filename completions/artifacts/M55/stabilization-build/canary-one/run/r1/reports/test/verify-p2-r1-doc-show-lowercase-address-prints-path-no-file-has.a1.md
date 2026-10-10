# verify-real — `r1-doc-show-lowercase-address-prints-path-no-file-has` (run canary-one, round 1, attempt 1)

- **key:** `r1-doc-show-lowercase-address-prints-path-no-file-has`
- **door:** `jigc doc show`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `confirmed`
- **regression:** `false` — the same block, on fresh rigs, with the previous release's binary gives the same exits and the same bytes (rig path normalised). Comparable: the door and both routes exist there.
- **basis:** on this case-insensitive volume `jigc doc show research:upper` opens `docs/research/UPPER.md` and refuses under the spelling `docs/research/upper.md`; the `jigc migrate` command its route prints, run as printed, dead-ends — `migrate.source-untracked`, whose printed `git add` exits 0 and stages nothing and whose printed re-run refuses byte-identically — so one of the two commands the refusal's route prints does not work as printed. The other, `jigc ingest`, does.
- **contested:** `false` — the finding argues no decision wrong, and I found none that intends this.
- **class:** `instance, unbounded`. What I drove is listed below; I enumerated no consumer set of the address-to-path mechanism.

`<W>` is my own scratch directory, `<scratch>/verify-p2.RmywLB`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` stands for the rig's repository root wherever the binary printed it as an absolute path; the binary prints that absolute path, I shortened it here and nowhere else.

## What breaks the clause, and what does not

The finding's own sentence has two halves and I separate them, because only one of them reaches the clause.

- **The printed path, by itself, breaks no clause.** `docs/research/upper.md` is the canonical home of the address that was typed; the volume resolves it to the one file there. A message that spells a path differently from its directory entry is a wording matter.
- **The route built from that path is what breaks it.** The refusal's route carries the same lower-cased spelling into a command line, and that command line cannot do what its sentence says. That is the second half of the clause's instrument — *every refusal's route works as printed* (DECISIONS.md → *2026-10-04 — The exit rule, revised*, sharpening 2).

A reader who discounts the second point — because the route's first command, `jigc ingest`, works and leads onward under the true spelling — would grade this `breaks-no-clause`. I did not, for the reasons under *Step 3*; the fact is stated here so that nobody has to find it in a table.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path, exit 0 — checked in every shell that drove anything, the candidate's for rigs A, B, C, D, G and the previous release's for rigs E and F |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: 1.0.0-rc.24. No build ran; nothing under `target/` was driven. Every rig was built with `dev/jigc-rig fresh --binary <that path>` and `SCRATCH=<W>`. git is 2.54.0 (Apple Git-157).

## The volume

**Case-insensitive.** Probed, not inferred: a file written as `<W>/CaseProbe` answers `test -e <W>/caseprobe`; the volume is APFS without the case-sensitive personality; and `git config --get core.ignorecase` prints `true` in the rigs, set by `git init` itself. A case-sensitive volume was not driven.

## What I drove

Seven fresh rigs, none of them the reporter's. Every exit status read bare; stdout and stderr captured to separate files under `<W>`; each printed command was lifted out of the captured stderr by its backticks and run as it stood, so no spelling in it is mine. The tree was hashed file by file (everything outside `.git`) before and after each step that could write.

| rig | binary | plant | chain |
|---|---|---|---|
| A | candidate | `docs/research/UPPER.md` = `# Upper\n`, untracked | `doc list` · `doc show research:upper` · route 1 `jigc ingest` · then ingest's own printed `jigc migrate` to the mint |
| B | candidate | the same, untracked | `doc show research:upper` · route 2 `jigc migrate` · its printed `git add` · its printed re-run |
| C | candidate | the same, added and committed | the chain of B, then `jigc ingest` |
| D | candidate | control: `docs/research/lower.md` = `# Lower\n`, untracked | the chain of B with `research:lower` |
| E | previous release | as A and B, untracked | the chain of B, then `jigc ingest` |
| F | previous release | as C, committed | the chain of B |
| G | candidate | as A, untracked | tree hashed around `doc show` alone; the json arm; `validate`; two neighbouring addresses |

### Rig B — candidate, untracked: the block

| step | argv | exit | what it printed | tree after |
|---|---|---|---|---|
| b1 | `jigc doc show research:upper` | 1 | stdout empty; stderr below | unchanged (rig G: hash list identical before and after) |
| b2 | `jigc migrate <repo>/docs/research/upper.md --as research` — route 2 of b1, as printed | 1 | stdout empty; stderr below | unchanged |
| b3 | `git -C <repo> add -- docs/research/upper.md` — the route of b2, as printed | 0 | nothing | index unchanged: no `docs/` entry; `git status --short` still `?? docs/research/UPPER.md` |
| b4 | `jigc migrate <repo>/docs/research/upper.md --as research` — the re-run b2 prints | 1 | stderr byte-identical to b2 (`cmp`, exit 0) | unchanged |

b1, stderr:

```
blocking · store.unparseable — `research:upper` at `docs/research/upper.md` does not parse: required section heading `## question` is missing
  at: research:upper
  route: adopt — run `jigc ingest` to route it, or `jigc migrate <repo>/docs/research/upper.md --as research` to rewrite it into the managed `research` shape; it is a foreign file, not an unmigrated managed doc
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

b2 and b4, stderr:

```
blocking · migrate.source-untracked — `docs/research/upper.md` is in neither this repository's index nor its HEAD — git holds no copy of it, and `jigc task finalize --approve` retires the source it migrates, so the file would be deleted from the worktree with nothing to recover it from and no deletion in the commit to say so
  at: docs/research/upper.md
  route: stage it with `git -C <repo> add -- docs/research/upper.md`, then re-run `jigc migrate <repo>/docs/research/upper.md --as research` — the index is enough, the source need not be committed first
```

The directory held one entry throughout, `UPPER.md`, 8 bytes.

### Rig C — candidate, committed

The plant was added and committed under its own spelling (`git ls-files -s docs` → `docs/research/UPPER.md`). Then the chain of B: exits 1 · 1 · 0 · 1, the same four texts, the re-run byte-identical to the first refusal, the index entry and the tree unchanged, no commit made. Here the second refusal's sentence is also false of the file its path opens: *git holds no copy of it* is said of a file that is in the index and in `HEAD` as `docs/research/UPPER.md`.

### Route 1 — `jigc ingest` (rigs A and C)

Exit 0, stderr empty. It names the file by its true spelling and prints a command under that spelling:

```
needs-reconcile docs/research/UPPER.md → research
  blocking · conformance.section-missing — required section heading `## question` is missing
  at: docs/research/UPPER.md
  route: `jigc migrate <repo>/docs/research/UPPER.md --as research` — it opens the `migrate-research` workflow, which rewrites the file to conformant shape and adopts it at finalize
```

Its only write is `.jigc/index/edges.json` and that file's lock — the gitignored index cache, holding `"edges": []` and no line naming either spelling.

In rig A I followed that printed command, as printed, as far as it runs without text of mine: `jigc migrate <repo>/docs/research/UPPER.md --as research` exits 1 `migrate.source-untracked` under the true spelling; its printed `git add -- docs/research/UPPER.md` exits 0 and stages the file (`A  docs/research/UPPER.md`); its printed re-run exits 0 and mints `migrate-research-docs-research-upper-<hash>`, writing eight files under `.jigc/tasks/<id>/` and nothing outside `.jigc/`. I stopped there: the next command is `jigc doc author` with a title its caller writes, which is no longer a route run as printed.

### Rig D — the control

`docs/research/lower.md`, untracked, and `research:lower`: exits 1 · 1 · 0 · **0**. The same two refusals; the printed `git add` stages the file (`A  docs/research/lower.md`); the printed re-run mints a task. So the dead end in B and C is the spelling's, and the chain itself is sound where the address and the file agree.

### Rig G — neighbours

| label | argv | exit | what it printed |
|---|---|---|---|
| show-json | `jigc doc show research:upper --format json` | 1 | stdout empty; a findings envelope on stderr with `"code": "store.unparseable"`, `"target": "research:upper"`, the same message and the same route |
| show-upper | `jigc doc show research:UPPER` | 1 | `store.malformed-slug`, route `jigc doc list` … *use lowercase letters, digits, and single hyphens* |
| show-absent | `jigc doc show research:nothere` | 1 | `store.not-found` — *could not read `research:nothere` at `docs/research/nothere.md`: No such file or directory* |
| validate | `jigc validate` | 1 | `schema-conformance.unadopted-instance` at `docs/research/UPPER.md` — *its name is not a doc id, so no `<type>:<slug>` address reaches it* — with the adoption route under the true spelling |

## What triage asked, answered

- **What each command writes, and where.** `doc show research:upper`: nothing. `jigc migrate <repo>/docs/research/upper.md --as research`: nothing, in either state. The printed `git add -- docs/research/upper.md`: nothing — the index is unchanged, untracked stays untracked and the committed entry keeps its blob. The printed re-run: nothing. `jigc ingest`: the index cache only.
- **Is a file created, replaced or left under a second spelling?** No. In no rig did a second directory entry appear, did `UPPER.md` change a byte, or did the index gain or lose an entry, along the lower-cased chain. The lower-cased spelling exists only in printed text.
- **Is the volume case-insensitive?** Yes (above).
- **Both binaries?** Yes (below).

## Step 2 — is it what the finding says?

**It reproduces, whole.** The lower-cased address reaches the upper-cased file — `doc show` gets far enough to parse it and complain about a missing `## question`, where an address nothing answers gives `store.not-found` (rig G) — and the refusal names `docs/research/upper.md`, a spelling no directory entry and no git object has. I tried the ways such a repro is usually wrong: the rigs and the scratch directory are mine and new; no status was read through a pipe; nothing was cut before it was compared; the binary is the candidate by hash and by `command -v`.

**No settled decision intends it.** I read the section that owns the refusal and the ones that own what its route leads to:

- design/doc-read-surface.md → `jigc doc list`, the bullet *`doc show`'s block on an unregistered instance routes to adoption*, and DECISIONS.md → *2026-07-14 — M42 Inc-8 T7*: the split to the adoption route is intended, and so is the route's text being the one `validate` emits. Neither says anything about an address whose spelling differs from the file's.
- design/auto-migration.md → *Trackedness precondition (M51)* states the contract of the second refusal in prose: a source git holds no copy of is refused with `migrate.source-untracked` and a route naming `git add <path>`, *after which the identical `jigc migrate` succeeds*. DECISIONS.md → *2026-09-14 — M51 Increment 1 planning*, T2, says the same of it: *the route must therefore be sufficient*. In rigs B and C the printed `git add`, run verbatim, is followed by the identical `jigc migrate` refusing again.
- DECISIONS.md → *2026-09-16 — M51 complete* records the same route dead-ending once before, on a path with a space (`git add` exit 128, the re-run exit 2, *nothing repaired*), and treats that as a defect of a class — an emitted command line whose operand is a path. So a dead end at this route has been ruled a defect, not a bound.
- design/surface-contract.md → Law 1: *every printed path is repo-real or a typed identity*. I cite it as the rule the spelling sits next to and rest nothing on it.
- DECISIONS.md, the M50 workbench entry: *the workbench predicate must fold case … a case-sensitive predicate is a hole on the majority filesystem*. It concerns another predicate; it shows that this volume is treated as an ordinary one, not as a planted state.

## Step 3 — does it break the clause, inside its scope?

**Yes — on the route, not on the message.** The clause is `working-product`; its instrument has two halves.

- *No command that works on rc.24 in a supported layout stops working.* Untouched: nothing here worked on the previous release.
- *Every refusal's route works as printed.* The refusal of `jigc doc show research:upper` prints one route with two commands. `jigc ingest` works as printed. `jigc migrate <repo>/docs/research/upper.md --as research`, whose sentence is *to rewrite it into the managed `research` shape*, does not: it refuses, the single route of that refusal is a `git add` that exits 0 having staged nothing plus a re-run that refuses with the same bytes, and nothing on either screen names the spelling as the cause. That is the sense in which this record has used the words before — a route that dead-ends on the door it names.

Why I do not let the working `jigc ingest` cancel it: the clause says *every* route, the route line offers the two commands as alternatives of equal standing and gives the second the concrete operand, and the dead end is silent — exit 0 from `git add`, then the same refusal — so a reader has nothing that sends them back to the first command.

**Scope.** The layout is ordinary: one hand-written file at a doctype's home, on the default volume of the platform, no git configuration of mine. The address is one no surface prints; it is what the `store.malformed-slug` refusal of `research:UPPER` describes when it says *use lowercase letters*. Nothing here is a race or a planted state, and the run declares no bound.

**The clause it does not touch.** `no-lost-files`: no byte was written or removed anywhere along the lower-cased chain, at any exit status.

## Step 4 — the regression fact

Rigs E (untracked) and F (committed), fresh, built and driven with the previous release's binary by its absolute path, its directory first on `PATH`.

| step | candidate (B · C) | previous release (E · F) |
|---|---|---|
| `doc show research:upper` | 1 · 1 | 1 · 1 |
| route 2, `jigc migrate …/upper.md --as research` | 1 · 1 | 1 · 1 |
| its printed `git add` | 0 · 0, nothing staged | 0 · 0, nothing staged |
| its printed re-run | 1 · 1, same bytes | 1 · 1, same bytes |
| route 1, `jigc ingest` | 0 · 0 | 0 (E) |

With the rig's root replaced by one token, the stderr of each of the three refusing steps is byte-identical between the binaries in both states, and so is `ingest`'s stdout; the comparer was checked against a pair that differs (rig B against rig D) and said so. Red on both: `regression: false`.

## Coverage

The finding makes no coverage claim. For the block's `pinned-by` I looked in the suites, not in a diff: the suite files under `crates/cli/tests` and `tooling-tests` that name `migrate.source-untracked` — **6** (`flow52_acceptance.rs`, `git_span_aim.rs`, `migrate_source_rules.rs`, `path_arg_occurrence_axis.rs`, `pre_dispatch_faults.rs`, `retire_sink_validation.rs`) — and those that name `store.unparseable` — **5**; in neither set does any line carry a case-folding term (`ignorecase`, `case-insensitive`, `to_uppercase`, `to_lowercase`, their ASCII forms, `case-fold`). A suite that reaches the cell by planting two spellings as plain literals is outside that search, so this is *none found*, not *none exists*.

## Repro V-1

```yaml
claim: "on a case-insensitive volume `jigc doc show research:upper` reaches docs/research/UPPER.md and refuses `store.unparseable` under the spelling docs/research/upper.md; the `jigc migrate` command its route prints dead-ends — `migrate.source-untracked`, whose printed `git add` exits 0 staging nothing and whose printed re-run refuses byte-identically; the route's other command, `jigc ingest`, works"
verdict: CONFIRMED
regression: false          # the previous release gives the same exits and bytes
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d
precondition: "the volume is case-insensitive — a file written as `CaseProbe` answers a stat of `caseprobe`"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - "arm untracked: add nothing · arm committed: git add docs/research/UPPER.md, git commit"
repro:
  - ["jigc", "doc", "show", "research:upper"]
  - ["jigc", "migrate", "<repo>/docs/research/upper.md", "--as", "research"]   # route 2 of step 1, as printed
  - ["git", "-C", "<repo>", "add", "--", "docs/research/upper.md"]              # the route of step 2, as printed
  - ["jigc", "migrate", "<repo>/docs/research/upper.md", "--as", "research"]   # the re-run step 2 prints
  - ["jigc", "ingest"]                                                          # route 1 of step 1
expect:
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · store.unparseable — `research:upper` at `docs/research/upper.md` does not parse", "run `jigc ingest` to route it, or `jigc migrate <repo>/docs/research/upper.md --as research`"]
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · migrate.source-untracked — `docs/research/upper.md` is in neither this repository's index nor its HEAD", "stage it with `git -C <repo> add -- docs/research/upper.md`, then re-run `jigc migrate <repo>/docs/research/upper.md --as research`"]
  - exit: 0
    stdout: ""
    stderr: ""
    assert: "`git ls-files -s` is what it was before the step"
  - exit: 1
    stderr: "byte-identical to step 2"
  - exit: 0
    stderr: ""
    stdout_contains: ["needs-reconcile docs/research/UPPER.md → research", "route: `jigc migrate <repo>/docs/research/UPPER.md --as research`"]
  tree: "docs/research holds the one entry UPPER.md, its 8 bytes unchanged, after every step; steps 1 to 4 write no file at all"
control: "with docs/research/lower.md and research:lower the same four steps exit 1 · 1 · 0 · 0 and step 4 mints a task"
observed: "<W>/jigc-rig-fresh-64hWhw (B), -ttK54s (C), -yM5SDz (A), -3y06RF (D), -OU7JUf (G); previous release <W>/jigc-rig-fresh-7tz6Ge (E), -ZKnfkS (F)"
pinned-by: "UNPINNED: found this round; no suite naming either refusal carries a case-folding cell"
```

**Pinnable as it stands: no.** Two reasons, both about the block and neither about the fact. Its expectations hold only on a case-insensitive volume, and the CI runner's is case-sensitive, so a test made from it needs the precondition as a probe that selects the arm — what the candidate answers on a case-sensitive volume is undriven and is not in the block. And the route's operand is an absolute path under the rig's root, so the assertions compare after replacing that root, as the table above did. With those two it fits beside `migrate_source_rules`'s verbatim-route arm, whose shape it mirrors.

## Left open — hit on the way, not pursued

1. **`migrate.source-untracked` says *git holds no copy of it* of a committed file.** Rig C: the source is in the index and in `HEAD` as `docs/research/UPPER.md`; the door, handed the other spelling, answers that git holds no copy and routes to a `git add` that changes nothing. Door `jigc migrate`. It is the evidence of this verdict's second step and also a statement of that door's own; whether it is reachable by a hand-typed relative path, without `doc show` in front, I did not drive.
2. **`jigc validate` says of this file that no `<type>:<slug>` address reaches it; on this volume one does.** Rig G: the sentence and `doc show research:upper` parsing the file, in the same repository. Wording, as far as I looked.
3. **Where a migration of `docs/research/UPPER.md` lands is undriven, and one cell of it is worth a look.** The composed workflow says each instance is *a managed file at `docs/research/<slug>.md`, its `<slug>` minted from `title`*, and finalize retires the source. With a title that slugs to `upper`, the destination and the source to retire are one file on this volume. I stopped at the mint and claim nothing about what finalize does there.
4. **A case-sensitive volume was not driven**, on either binary.
5. **Other doors that take a `<type>:<slug>` address were not driven with a lower-cased address over an upper-cased file** — the `#fragment` reads, the `--task` arm, the write verbs, `jigc rename`, `jigc unmanage`.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my rigs under `<W>`: the install commit `jigc setup` makes in each, and one of mine in each of rigs C and F for the *committed* arm. Rigs A and D each hold one minted, unfinalized task.

<!-- end of report -->
