# verify-real — `r1-p3-adoption-route-title-upper-undriven-cells` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-adoption-route-title-upper-undriven-cells`
- **door:** `jigc validate` (the door that prints the adoption route); the cells in question lie further on, at `jigc migrate`, `jigc doc author`, `jigc doc rename`, `jigc rename` and `jigc task finalize`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `refuted`
- **basis:** `does-not-reproduce` — the cells the finding names were driven and hold no defect of their own: on a volume that does not fold case the adoption report's Repro V-1 is green on both binaries (the title `Upper` lands `research:upper` at `docs/research/upper.md`, and the one route the block prints there exits 0); on the folding volume the previous release gives the candidate's exits and bytes for the untracked plant under the title `Upper`, and the way out lands on it from both plants.
- **regression:** not stated — it goes with `confirmed` only. The fact itself is in the tables: the two binaries agree in every cell driven.
- **contested:** `false` — the finding argues no decision wrong.
- **class:** `instance, unbounded`. I enumerated no consumer set of the mechanism. What was driven is under *What I drove*.

**What this verdict does not touch.** The finding was two cells nobody had driven, not a defect anybody had seen. The defect on the folding volume — `docs/research/UPPER.md` authored under the title `Upper` is refused, and two routes printed next exit 1 — is the ledger row `r1-adoption-route-for-non-doc-id-name-not-run`, confirmed there with `regression: false`. I re-drove it on both binaries on the way (rigs hC and hPC) and it reproduces exactly as that report says. `refuted` here means: the undriven cells add no second break and no difference between the binaries. It is not a refutation of that row.

`<W>` is my own scratch directory, `<scratch>/verify-p3-upper.tPU9fe`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` is the repository of whichever rig a row names.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | compared against that binary's path at the top of every shell that drove anything, and the shell stops where they differ; it never did. The rig's own `$JIGC` is held to the same path. So a bare `jigc` inside a printed route resolved to the binary that printed it |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: `jigc 1.0.0-rc.24`. No build ran; nothing under `target/` was driven.

## The volume that does not fold case — what I changed to reach the cell

The machine has no such volume, and the binaries are this platform's, so a container of another platform cannot run them. I made one, inside `<W>` and nowhere else:

| step | result |
|---|---|
| `hdiutil create -size 600m -fs "Case-sensitive APFS" -volname csvol -type SPARSE <W>/cs.sparseimage` | exit 0 |
| `hdiutil attach <W>/cs.sparseimage -mountpoint <W>/cs -nobrowse` | exit 0; `diskutil info <W>/cs`: *File System Personality: Case-sensitive APFS* |
| probe on it: write `<W>/cs/CaseProbe`, then `ls <W>/cs/caseprobe` | exit 1, *No such file or directory* — two names, two files |
| the same probe beside it, on the host volume: write `<W>/CaseProbe`, then `ls <W>/caseprobe` | exit 0 — one file |
| `git config core.ignorecase` in a rig built on it | empty (git's default, false); in a rig on the host volume: `true` |

Every rig named `cs…` below was built with `SCRATCH=<W>/cs`, so its repository, its `.jigc/` workbench and its rig home all lie on that volume. The image is detached again when this report is handed over; the rigs stay inside `<W>/cs.sparseimage` and are reached by attaching it again with the second command above.

**A bound of this reconstruction, flagged as one.** This is a case-sensitive volume of this platform, with this platform's git (`git version 2.54.0 (Apple Git-157)`). It is not the runner CI uses. What it settles is the one variable the finding names — whether `UPPER.md` and `upper.md` are one file.

## What I drove

Ten fresh rigs, each `dev/jigc-rig fresh --binary <that binary's path>`. The plant is `docs/research/UPPER.md` = `# Upper\n`. Every exit status was read bare; stdout, stderr and the exit of every step were captured to separate files under `<W>/ev/<rig>/`; every printed route was lifted out of the emitted bytes and run through `sh -c`, never retyped. The payload of every `jigc doc author research --from-file - --task <task>` is the one in the adoption report's Repro V-1, with `title:` set to the title the row names.

| rig | volume | binary | plant | what was driven |
|---|---|---|---|---|
| csV | no folding | candidate | committed | Repro V-1 as written, every printed route, then the way out |
| csPV | no folding | previous | committed | the same |
| csC | no folding | candidate | committed | `validate`, `ingest`, the route, the title `Upper`, carried to the approve |
| csPC | no folding | previous | committed | the same |
| csA | no folding | candidate | untracked | the route, its staging route, the title `Upper`, carried to the approve |
| csPA | no folding | previous | untracked | the same |
| hPA | folding | previous | untracked | the route, its staging route, the title `Upper`, then the way out |
| hA | folding | candidate | untracked | the same, for the comparison |
| hPC | folding | previous | committed | Repro V-1 as written, every printed route, then the way out |
| hC | folding | candidate | committed | the same, for the comparison |

### 1. Repro V-1 as written, on the volume that does not fold case (rigs csV, csPV)

| # | argv | V-1 says (folding volume) | observed, candidate | observed, previous |
|---|---|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/UPPER.md --as research` | 0 | 0 — `task minted: …` | 0 |
| 2 | `jigc doc author research --from-file - --task <task>`, `title: "Upper"` | 1, `write.non-reparseable` | **0** — stdout `research:upper`, stderr empty | **0** |
| 3 | the same, `title: "Upper notes"` | 1, `write.identity-change` | 1, `write.identity-change` — *this task's `research` is already `research:upper`*, routed at `jigc doc rename research:upper --to 'Upper notes' --task <task>` | 1 |
| 4 | that route, as printed | 1, `write.identity-change` — *`research:upper` is committed* | **0** — `research:upper-notes (renamed to "Upper notes" from research:upper)`; `jigc doc list research --task <task>` then prints `research:upper-notes  docs/research/upper-notes.md  managed` | **0** |
| 5 | `jigc task discard <task> --force` | 0 | 0 | 0 |
| 6 | `jigc rename research:upper --to 'Upper notes'` | 1, a bare `git mv` failure | 1, `blocking · store.not-found` — *no managed doc `research:upper` to rename (expected at docs/research/upper.md)*, routed at `jigc describe` (run: exit 0) | 1 |

After step 6: `jigc doc list` prints `research:UPPER  docs/research/UPPER.md  unregistered`, `git status --short` is empty, `docs/research/UPPER.md` is still `# Upper`.

Read against the claim:

- **Step 2 is the block's red, and it is green here.** The refusal V-1 pins does not occur where the two names are two files.
- **Step 3's refusal is the same text on both volumes** (`cmp` exit 0 against rig hC's, the rig directory's name and the volume's directory normalised), and it is a correct one here: the task does hold `research:upper`. **Its route works as printed** (step 4, exit 0) — on the folding volume that same route is the one that exits 1.
- **Step 6 exits 1 on both volumes, for different reasons.** In V-1 it is the route step 4's refusal printed. Here step 4 printed no refusal, so no surface asked for step 6: it is a command aimed at an id that the discard of step 5 has just removed, and the door says so by code, with an `at:` and a route that runs. That is a refusal doing its work, not a route failing as printed.
- **The two binaries:** every captured stdout, stderr and exit of these steps in rig csV is byte-identical to rig csPV's, the rig directory's name normalised. Over the two rigs' whole runs, the way out of section 4 included, 91 captures of 93 are identical, and the other two differ in commit ids.

### 2. The same plant carried to its end under the title `Upper`, no folding (rigs csC, csPC, csA, csPA)

| step | committed plant (csC, csPC) | untracked plant (csA, csPA) |
|---|---|---|
| `jigc validate` | 1 — `schema-conformance.unadopted-instance`, routed at `jigc ingest` or `jigc migrate <repo>/docs/research/UPPER.md --as research` | 1 — the same |
| `jigc ingest` | 0 — `needs-reconcile … → research`, `conformance.section-missing`, the same `migrate` route | 0 — the same |
| `jigc doc list` | 0 — `research:UPPER  docs/research/UPPER.md  unregistered` | 0 — the same |
| the route, as printed | 0 — task minted | 1 — `migrate.source-untracked`, routed *stage it with `git -C <repo> add -- docs/research/UPPER.md`, then re-run `jigc migrate …`*; both commands lifted and run: 0 and 0 (task minted) |
| `jigc doc author …`, `title: "Upper"` | 0 — `research:upper` | 0 — `research:upper` |
| `jigc task finalize <task>` | 4 — the review hold, naming `docs/research/UPPER.md` as the file it will delete and `docs/research/upper.md` as the rewrite | 4 |
| `jigc task finalize <task> --approve` | 0 — *deleted docs/research/UPPER.md · promoted docs/research/upper.md · 2 files committed* | 0 — *promoted docs/research/upper.md · 1 file committed* |
| `jigc doc list` | 0 — `research:upper  docs/research/upper.md  managed` | 0 — the same |
| `jigc doc show research:upper` | 0 — the authored document | 0 |
| `jigc validate` | 0 — *no findings — the committed store validates clean* | 0 |
| `git status --short --untracked-files=all` | empty | empty |
| `ls docs/research` | `upper.md` alone | `upper.md` alone |
| `git log --name-status`, the adopting commit | `D docs/research/UPPER.md`, `A docs/research/upper.md` | `A docs/research/upper.md` |

So the question the adoption report left open — *there the title `Upper` would name a second path beside `UPPER.md`; what the route does then is unknown to me* — has this answer: the route lands the file under the addressable name `research:upper`, the foreign original is retired in the adopting commit, and the store validates clean. That is what the design that owns the door says of it: design/auto-migration.md → *Adopt (the existing path)* — *the managed write lands at the lowercase singleton path, distinct from the foreign sibling, which the retire step removes* — and → *Retire-the-foreign-original*.

In the committed cell the foreign bytes stay in the commit that held them. In the untracked cell the staging route put them in the object store: `git cat-file -e` on the blob id of `# Upper\n` exits 0 in rig csA after the approve.

The two binaries: csC against csPC and csA against csPA differ in two captures each — the commit ids in the approve's `finalized …` line and in `git log` — and in one line of the candidate's install commit (`.jigc/settings-entries.json`, which the previous release's install does not write). csC's two `ls` captures were taken with another flag than csPC's and are not counted. Nothing else differs.

### 3. The untracked plant under the title `Upper`, on the folding volume, previous release (rig hPA; rig hA for the comparison)

| # | argv | exit | what it printed |
|---|---|---|---|
| 1 | `jigc validate` | 1 | the `unadopted-instance` row and its `migrate` route |
| 2 | `jigc ingest` | 0 | `needs-reconcile … → research` |
| 3 | `jigc doc list` | 0 | `research:UPPER  docs/research/UPPER.md  unregistered` |
| 4 | `jigc migrate <repo>/docs/research/UPPER.md --as research` | 1 | `blocking · migrate.source-untracked`, the two-command route; `git status` after it: `?? docs/research/UPPER.md`, unchanged |
| 5 | that route's first command, as printed | 0 | - (`git status`: `A  docs/research/UPPER.md`) |
| 6 | that route's second command, as printed | 0 | `task minted: …` |
| 7 | `jigc doc author research --from-file - --task <task>`, `title: "Upper"` | **1** | `blocking · write.non-reparseable` — *write rejected: the source does not conform to the schema (required section heading `## question` is missing)* · at `research:upper#question` |

`git status` after step 7: `A  docs/research/UPPER.md`; `ls docs/research`: `UPPER.md`. Nothing was lost and nothing committed.

**This is the candidate's behaviour, byte for byte.** Rig hA (candidate) gives the same exit at each step, and every captured stdout, stderr and exit of the two rigs through step 7 is identical under `cmp`, the rig directory's name normalised. The adoption report's rig A had driven this cell on the candidate only; the previous release does the same.

### 4. The way out, on the previous release (rigs hPA and hPC; rigs hA and hC for the comparison)

The way out the adoption report found on the candidate: discard the task, run the route again, and author under another title as the new task's first write.

| # | argv | hPA — from the staged plant, after section 3's step 7 | hPC — from the committed plant, after V-1's six commands |
|---|---|---|---|
| 1 | `jigc task discard <task> --force` | 0 — *discarded task …*; `git status` still `A  docs/research/UPPER.md` | (V-1's own step 5 had discarded it: 0) |
| 2 | `jigc migrate <repo>/docs/research/UPPER.md --as research` | 0 — task minted | 0 — task minted |
| 3 | `jigc doc author …`, `title: "Upper notes"`, the task's first write | 0 — `research:upper-notes` | 0 — `research:upper-notes` |
| 4 | `jigc task finalize <task>` | 4 — the review hold | 4 |
| 5 | `jigc task finalize <task> --approve` | 0 — *promoted docs/research/upper-notes.md · 1 file committed* | 0 — *deleted docs/research/UPPER.md · promoted docs/research/upper-notes.md · 2 files committed* |
| 6 | `jigc doc list` | 0 — `research:upper-notes  docs/research/upper-notes.md  managed` | 0 — the same |
| 7 | `jigc doc show research:upper-notes` | 0 | 0 |
| 8 | `jigc validate` | 0 — *no findings* | 0 — *no findings* |
| 9 | `git status --short --untracked-files=all` | empty | empty |

**It lands on the previous release, from both plants**, as it does on the candidate (rigs hA and hC: the same exits; the captures differ only in the commit ids and in the install commit's one line named in section 2). The same commands land on the volume that does not fold case too (rigs csV and csPV, after section 1's step 6: the route 0, the author 0, the hold 4, the approve 0, then `jigc validate` 0).

Before the way out, rig hPC ran V-1's six commands as written on the previous release: exits 0, 1, 1, 1, 0, 1 with V-1's refusal texts — the adoption report's result for the previous release, re-driven. Rig hC (candidate) is identical to it in 91 of 93 captures, the two that differ being the commit ids of the way out's approve.

## Step 2 — is it what the finding says?

The finding says two cells were not driven. That was true, and it is the whole of the finding: it names no exit, no output and no file that is wrong. So the question I was sent with is whether a defect sits in those cells. Driven:

1. **No folding, both binaries** (sections 1 and 2): V-1's red is green; the adoption route lands the file under `research:upper`, from a committed and from an untracked plant; every route a refusal printed there ran at exit 0.
2. **Folding, previous release, untracked plant under `Upper`** (section 3): the refusal the candidate gives, byte for byte.
3. **Folding, previous release, the way out** (section 4): it lands, from both plants.

I tried the ways such a result is usually wrong. The rigs are mine and new, one per cell and binary. No exit status was read through a pipe; the captures are whole files. The binary of each rig is held to its path by `command -v` and by the rig's `$JIGC`, and the two binaries' hashes are the ones the prompt gives. That the `cs…` rigs really lie on a volume of the other kind is shown three ways — the mount's personality, the probe, and `core.ignorecase` — and by the outcome itself: the same binary on the same plant refuses on the host volume (hC, hPC) and lands here. That the comparison can see a difference is shown by the differences it found (the commit ids, the install commit's line).

## Step 3 — does it break the clause, inside its scope?

The clause is DECISIONS.md → *2026-10-04 — The exit rule, revised*, second clause, by its instrument: *no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed*. It is in the run's closing condition as `working-product`.

- **First half: not broken in these cells.** Nothing that works on the previous release stopped working: on either volume, from either plant, the two binaries give the same exits and the same bytes.
- **Second half: not broken in the cells this key adds.** On the volume that does not fold case, every route a refusal printed ran at exit 0 — `migrate.source-untracked`'s two commands, `write.identity-change`'s `jigc doc rename …`, `store.not-found`'s `jigc describe`. On the folding volume the routes that fail as printed are the ones the row `r1-adoption-route-for-non-doc-id-name-not-run` already holds, confirmed; the cells of this key hold no further one.

So there is nothing under this key that breaks the clause, and nothing under it that is a defect short of the clause either: the basis is `does-not-reproduce`, not `breaks-no-clause`.

## Step 4 — the regression fact

Not stated: the verdict is not `confirmed`. For whoever reads the other row: the previous release was driven in five rigs here, and in each it agrees with the candidate.

## Step 5 — coverage

The finding's coverage claim is about what a verifier drove, and sections 1 to 4 answer it. For the blocks below I read the suites, not any diff: **78** suite files under `crates/cli/tests` and `tooling-tests` spell the argv word `"migrate"`; **37** of them hold a literal lower-case `docs/<home>/<name>.md` (the control: the pattern finds what is there); **0** hold one whose name carries an upper-case letter. Three suite files mention `ignorecase` or a case-sensitive volume, and none of the three is among the 78. A suite that composes the path some other way is outside this count.

## Repro U-1 — V-1 on a volume that does not fold case

```yaml
claim: "on a volume that does not fold case, the adoption route `jigc migrate <path> --as research` for a committed docs/research/UPPER.md, authored under the title `Upper`, is refused at `jigc doc author`"
verdict: REFUTED
basis: does-not-reproduce
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exits, the same bytes but for commit ids
requires: "a volume on which `UPPER.md` and `upper.md` are two files"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: a research note"]
repro:
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]          # <task> = the id on stdout's `task minted: ` line
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin: the payload below
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "show", "research:upper"]
  - ["jigc", "validate"]
expect:
  - exit: 0
    stdout_contains: "task minted: "
  - exit: 0
    stdout: "research:upper\n"
    stderr: ""
  - exit: 4
    stdout_contains: "DELETE the foreign original `docs/research/UPPER.md`"
  - exit: 0
    stdout_contains: "  deleted docs/research/UPPER.md\n  promoted docs/research/upper.md\n  2 files committed\n"
  - exit: 0
    stdout: "id  path  state\nresearch:upper  docs/research/upper.md  managed\n"
  - exit: 0
  - exit: 0
    stdout_contains: "no findings"
  tree: "`git status --short --untracked-files=all` empty after the approve; docs/research holds `upper.md` alone; the adopting commit is `D docs/research/UPPER.md`, `A docs/research/upper.md`"
stdin: |
  title: "Upper"
  sections:
    - id: question
      set:
        question: |-
          <<What does it look like in practice?>>
    - id: findings
      set:
        findings: |-
          <<Found.>>
    - id: sources
      set:
        sources: |-
          <<First-hand notes.>>
where: "<W>/cs/jigc-rig-fresh-G4IZtp (candidate, rig csC); <W>/cs/jigc-rig-fresh-HXV4wS (previous, rig csPC); V-1's own six commands in <W>/cs/jigc-rig-fresh-dIk5wf (candidate, rig csV) and <W>/cs/jigc-rig-fresh-IMQfXf (previous, rig csPV); the untracked plant in <W>/cs/jigc-rig-fresh-JRy1qP and <W>/cs/jigc-rig-fresh-thblty; captures under <W>/ev/"
pinned-by: "UNPINNED: no suite migrates a file at a doctype home whose name differs from a slug by case, on a volume of either kind"
```

**Pinnable as it stands: no.** Its green depends on the volume. On a volume that folds case — the default of the platform this repository is developed on — the second step is the refusal the row `r1-adoption-route-for-non-doc-id-name-not-run` holds, so the block as written is red there today. A test that pins it needs the same thing that row's block needs: an arm that says its skip out loud where `CaseProbe` and `caseprobe` are one file, or a seam below the binary that takes the two paths as given. Once that row is repaired, what this block expects is one of the two answers its fixer may choose for the folding volume as well; until then it is the volume-bound half of one fact. And I drove it on a case-sensitive volume of this platform, not on the runner CI uses (a bound, flagged above).

## Repro U-2 — the way out, on the previous release and the candidate alike

```yaml
claim: "after an adoption task for docs/research/UPPER.md has been discarded, the route run again and authored under another title as the new task's first write does not land the file"
verdict: REFUTED
basis: does-not-reproduce
binary: candidate c1 (sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc); the same on 1.0.0-rc.24 (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: a research note"]
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]          # <task0>
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task0>"]     # stdin as U-1, title "Upper"; its exit is the volume's (1 where case folds, 0 where it does not) and is not asserted
repro:
  - ["jigc", "task", "discard", "<task0>", "--force"]
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]          # <task>
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin as U-1, title "Upper notes"
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "doc", "list"]
  - ["jigc", "validate"]
expect:
  - exit: 0
  - exit: 0
    stdout_contains: "task minted: "
  - exit: 0
    stdout: "research:upper-notes\n"
  - exit: 4
  - exit: 0
    stdout_contains: "  deleted docs/research/UPPER.md\n  promoted docs/research/upper-notes.md\n  2 files committed\n"
  - exit: 0
    stdout: "id  path  state\nresearch:upper-notes  docs/research/upper-notes.md  managed\n"
  - exit: 0
    stdout_contains: "no findings"
  tree: "`git status --short --untracked-files=all` empty after the approve"
where: "<W>/host/jigc-rig-fresh-VZyR72 (previous, rig hPC); <W>/host/jigc-rig-fresh-DDfbov (candidate, rig hC); from the staged plant <W>/host/jigc-rig-fresh-DL5bQY (previous, rig hPA) and <W>/host/jigc-rig-fresh-xOtHWN (candidate, rig hA); on the other volume <W>/cs/jigc-rig-fresh-dIk5wf and <W>/cs/jigc-rig-fresh-IMQfXf"
pinned-by: "UNPINNED: no suite discards an adoption task and runs the route again under another title"
```

**Pinnable as it stands: yes, with one thing to re-derive.** Nothing it asserts depends on the volume: it landed on both. What I drove differs from the block in the commands between the setup's last step and the discard — in rigs hPC, hC, csV and csPV V-1's other commands ran in between (a second author, a rename, and after the discard a refused `jigc rename`), and in rigs hPA and hA the plant was staged, not committed, so the approve's line there reads *1 file committed* with no `deleted` row. The block is the shortest form those runs imply; the exits of its seven commands are the ones I observed in each of the six rigs.

## Left open — hit on the way, not pursued

1. **`jigc task finalize --approve` prints an `unadopted-instance` advisory for the file that same call retires**, routed at `jigc migrate` — seen again here on every approve, both volumes, both binaries. It is item 4 of the adoption report's own list, not a new observation; named so that nobody takes its presence in my captures for something I examined.
2. **A source staged by `migrate.source-untracked`'s route leaves no row in the adopting commit** — seen again (rigs csA, csPA, hA, hPA: *1 file committed*, the foreign file gone from the worktree, its blob still in the object store). It is item 5 of that list; I did not examine it against `no-lost-files`.
3. **The candidate's install commit carries `.jigc/settings-entries.json` and the previous release's does not** — a difference between the binaries at `jigc setup`, outside the door I was handed. I did not look into what it is.
4. **The runner CI uses was not driven.** The non-folding cell was reached on a case-sensitive volume of this platform; another platform's filesystem and git are a cell of their own.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else, before and after. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my ten rigs under `<W>`: the install commit `jigc setup` makes in each, one of mine in each rig whose plant is committed, and the commits the driven `jigc task finalize --approve` calls made. Outside the repository I made one thing that is not a plain file: the disk image `<W>/cs.sparseimage`, attached at `<W>/cs` while I drove and detached afterwards.

<!-- end of report -->
