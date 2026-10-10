# verify-real — `r1-p3-migration-destination-is-source-on-case-folding-volume` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-migration-destination-is-source-on-case-folding-volume`
- **door:** `jigc task finalize`
- **clause it is said to break:** `no-lost-files` (triage's grade: *unclear*)
- **verdict:** `refuted` — as a blocker on the clause named. The defect underneath is real and is described below.
- **basis:** `breaks-no-clause` — of `no-lost-files`, the clause this row names: the destination and the source ARE one file on this volume and finalize does treat them as two paths, but `jigc task finalize <id> --approve` then refuses at exit 3 (`finalize.stage-failed`), makes no commit, and destroys no byte — the original is in `HEAD`, and where the rewrite differs from it the original is also parked on disk under a path the refusal prints. No command along the chain that exits 0 removes or commits anything. That the migration cannot land at all, and that the refusal's route does not lead out, is a question for another clause and is in *Left open*, ungraded by me.
- **regression:** not established as a field — it goes with `confirmed` only. The fact triage asked for is in the report all the same: the previous release behaves the same, exit for exit.
- **contested:** `false` — the finding argues no decision wrong, and I found none that intends this.
- **class:** `instance, unbounded`. What I drove is listed below; I enumerated no consumer set of the source-equals-destination comparison.

`<W>` is my own scratch directory, `<scratch>/verify-p3.jYuK1K`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` stands for a rig's repository root wherever the binary printed it as an absolute path. `<id>` is the task the migration mints, `migrate-research-docs-research-upper-192580f14883` in every rig whose plant is `UPPER.md`.

## The answer to what triage asked

> is the file there afterwards, and what does the commit hold

- **The file is there afterwards**, in every arm, on both binaries: `docs/research` holds the one entry `UPPER.md` after `--approve`.
- **There is no commit.** `--approve` exits 3 and `HEAD` is the commit it was. The index entry `docs/research/UPPER.md` keeps its blob.
- **What the file holds depends on the rewrite.** Where the authored rewrite is byte-identical to the source, the file is byte-identical to what it was and `git status --short` is empty. Where the rewrite differs, the file is left holding the **rewrite**, uncommitted (` M docs/research/UPPER.md`), a second refusal says so (`finalize.rollback-conflict`), and the pre-image — the original, byte for byte — is parked at `.jigc/displaced/finalize/docs/research/upper.md.pre-image.<n>`.
- **Both binaries:** the same exits and the same end states (below).

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path — checked in every shell that drove anything, and a mismatch exits the shell before the first command |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: 1.0.0-rc.24. No build ran; nothing under `target/` was driven. Every rig was built with `dev/jigc-rig fresh --binary <that path>` and `SCRATCH=<W>`. git is 2.54.0 (Apple Git-157).

## The volume

**Case-insensitive.** Probed: a file written as `<W>/CaseProbe` answers `test -e <W>/caseprobe` with 0; the volume's personality is plain APFS; `git config --get core.ignorecase` prints `true` in the rigs, set by `git init` itself. A case-sensitive volume was not driven.

## The finding had no repro block; what I reconstructed

The finding is one sentence of another report's *Left open* list and says of itself that nothing was driven. I reconstructed the smallest chain that reaches the door, from what triage asked:

1. **A conformant plant.** In a control rig (rig 0, candidate) I let jigc write a `research` doc itself — `jigc start --workflow do-research`, `doc create research --title Upper`, three `set-slot`s, the commit doc, `jigc task finalize`, exit 0 — and took the committed bytes of `docs/research/upper.md`: 216 bytes, sha256 `b398c7c9…1eec0d`. Those bytes conform by construction.
2. **The plant.** In each fresh rig: those bytes written as `docs/research/UPPER.md`, `git add`, `git commit`.
3. **The way in is a route jigc prints.** `jigc validate` exits 1 over that repository with `schema-conformance.unadopted-instance` and the route *adopt — run `jigc ingest` to route it, or `jigc migrate <repo>/docs/research/UPPER.md --as research` to rewrite it into the managed `research` shape*. I ran that `jigc migrate` command as printed. So the chain is not a hand-made one: it starts at a printed command.
4. **Author under the title Upper** with the batch payload the composed workflow prints (`jigc doc author research --from-file - --task <id>`), the commit doc's four writes, then `jigc task finalize <id>` and `jigc task finalize <id> --approve`.

Every exit status was read bare. stdout and stderr went to separate files under `<W>`. For the driver-run rigs the tree outside `.git` and `.jigc` was hashed file by file, with `HEAD`, `git ls-files -s` and `git status --short`, after the plant, after the authoring, after the hold and after `--approve`.

## What I drove

| rig | binary | plant | rewrite | `--approve` | commit | file afterwards |
|---|---|---|---|---|---|---|
| A | candidate | `UPPER.md`, committed | identical to the source | **3** `finalize.stage-failed` | none | `UPPER.md`, the original bytes; status clean |
| B | candidate | `UPPER.md`, committed | differs in one slot | **3** `finalize.stage-failed` + `finalize.rollback-conflict` | none | `UPPER.md`, the rewrite, ` M`; original parked and in `HEAD` |
| C (control) | candidate | `upper.md`, committed | differs in one slot | **0** | `M docs/research/upper.md`, one file | `upper.md`, the rewrite |
| D (added) | candidate | `UPPER.md`, committed, then one uncommitted line appended | differs | **3**, both refusals | none | `UPPER.md`, the rewrite, ` M`; the uncommitted original parked |
| E (added) | candidate | `UPPER.md`, staged and never committed | differs | **3**, both refusals | none | `UPPER.md`, the rewrite, `AM`; original parked and its blob still in the object store |
| P1 | previous release | as A | identical | **3** `finalize.stage-failed` | none | as A |
| P2 | previous release | as B | differs | **3**, both refusals | none | as B |

Rigs D and E are mine, beyond the state triage named (*a committed file*): they are the two neighbouring states in which the source's bytes are not in a commit, and I drove them because a refutation resting on *git holds it* would otherwise say nothing about them. They are evidence for the refutation, not a second finding.

### The chain, rig A (candidate), step by step

| step | argv | exit | what it printed, what it wrote |
|---|---|---|---|
| a1 | `jigc doc list` | 0 | `research:UPPER  docs/research/UPPER.md  unregistered` |
| a2 | `jigc ingest` | 0 | `needs-reconcile docs/research/UPPER.md → research` · `ingest.unaddressable-identity` · route: `git -C <repo> mv docs/research/UPPER.md docs/research/upper.md`, then re-run `jigc ingest` |
| a3 | `jigc validate` | 1 | `schema-conformance.unadopted-instance`, with the two-command route quoted above |
| a4 | `jigc migrate <repo>/docs/research/UPPER.md --as research` — the route of a3, as printed | 0 | mints `<id>`; composes `migrate-research`; the recorded `source-path` is `docs/research/UPPER.md` |
| a5 | `jigc doc author research --from-file - --task <id>`, title `Upper` | 0 | `research:upper`, and `advisory · file-state.baseline-adopt — baseline adopted: docs/research/upper.md`; the task's provenance records `research:upper` as `edited-from-base` |
| a6 | the commit doc: two `set-field`, two `set-slot` | 0 each | - |
| a7 | `jigc task validate <id>` | 0 | two advisories: `file-state.staged-copy` at `docs/research/upper.md`, `file-state.baseline-adopt` at `docs/research/UPPER.md` |
| a8 | `jigc task finalize <id>` | 4 | the review hold, below; nothing written outside `.jigc/` |
| a9 | `jigc task finalize <id> --approve` | **3** | stdout empty; stderr below; `HEAD` unchanged, index unchanged, `UPPER.md` byte-identical to the plant (`cmp`, 0), `git status --short` empty |
| a10 | `jigc task finalize <id> --approve` — the re-run a9's route prints | **3** | stderr byte-identical to a9 (`cmp`, 0); same end state |

a8, the first line of stdout — the hold names two spellings of the one file, one to write and one to delete:

```
migration review required — nothing committed. Re-run `jigc task finalize <id> --approve` to write the canonical doc, DELETE the foreign original `docs/research/UPPER.md`, and commit.
```

and the fidelity diff's second header is `+++ canonical rewrite → docs/research/upper.md`.

a9 and a10, stderr (candidate):

```
blocking · finalize.stage-failed — jigc could not stage its own changes — no commit was made and the promotions were rolled back: `git add -- :(literal)docs/research/upper.md :(literal)docs/research/UPPER.md :(literal).jigc/config :(literal).jigc/.gitignore :(literal).jigc/version` failed: fatal: pathspec ':(literal)docs/research/upper.md' did not match any files
  at: task:<id>
  route: `jigc task finalize <id> --approve` once the embedded git failure is resolved — a stale `.git/index.lock` removed, or the `.gitignore` rule dropped that covers `.jigc/config`, `.jigc/.gitignore` or `.jigc/version`: jigc commits those three with the work, and git will not add a path it ignores — the task survives intact, so the same re-run lands the commit
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

The `git add` it quotes names both spellings as two pathspecs. That is the finding's sentence, shown by the binary: the promote destination is `docs/research/upper.md`, the retirement is `docs/research/UPPER.md`, and they are one directory entry.

### Rig B (candidate) — the rewrite differs

The same chain by the driver, with the `findings` slot authored as a different sentence (the rewrite: 213 bytes, sha256 `a00d6c7d…6cd178`). Exits: validate 1 · migrate 0 · author 0 · hold 4 · `--approve` **3**. stderr of `--approve` is the `finalize.stage-failed` text above, followed by:

```
blocking · finalize.rollback-conflict — `docs/research/upper.md` changed while this finalize was running, so the rollback did not restore it: the bytes on disk are not the ones jigc wrote
  at: docs/research/upper.md
  route: nothing was committed and both versions are on disk: the file as it now stands at `docs/research/upper.md`, and this finalize's pre-image at `.jigc/displaced/finalize/docs/research/upper.md.pre-image.<n>`. Compare them, keep what you want, and delete the parked copy
```

The snapshot before `--approve` against the one after differs in exactly two lines: `git status --short` gains ` M docs/research/UPPER.md`, and that file's hash goes from the original's to the rewrite's. `HEAD` and the index are unchanged. Then, checked by `cmp`, each exit 0:

- the parked pre-image against the original 216 bytes;
- the worktree file against the task's staged rewrite, `.jigc/tasks/<id>/docs/research:upper.md`;
- `HEAD:docs/research/UPPER.md` against the original.

So after the refusal the original exists twice — a git object and a parked file — and the rewrite twice — the worktree and the task's working area.

The re-run the first refusal's route prints, `jigc task finalize <id> --approve`: exit 3 again, now `reconciliation.conflict-block` at `docs/research/UPPER.md` (*this path is the source the task is migrating, and it no longer holds what the migration recorded when it was minted*). Nothing changes on disk; no second parked file appears. A plain `jigc task finalize <id>` after it: exit 3, the same text.

### Rig C (candidate) — the control

The same bytes planted as `docs/research/upper.md`, the same chain, the differing rewrite: `jigc validate` exits 0, `--approve` exits **0**, `finalized … promoted docs/research/upper.md · 1 file committed`, and the commit is `M docs/research/upper.md` — the in-place rewrite design/auto-migration.md → *Honest bounds* describes for a same-path migration (*one commit, `M <path>` never `D`+`A`, nothing retired*). So the chain is sound where the two spellings agree, and what rigs A and B show is the spelling's.

### Rigs D and E (candidate) — the source's bytes are not in a commit

- **D.** After the commit, one line appended to `UPPER.md` and left uncommitted — bytes no git object holds. `jigc migrate` exits 0 and stages those bytes into the task. `--approve`: exit 3, both refusals, no commit. The parked pre-image is byte-identical to the uncommitted original (`cmp`, 0) and holds the appended line; the worktree file holds the rewrite. I then ran the first command of the next refusal's route, `jigc task discard <id> --force`: exit 0, *dropped staged edits*; the worktree file is unchanged by it and the parked pre-image is still there, still byte-identical to the uncommitted original.
- **E.** The plant `git add`ed and never committed. `--approve`: exit 3, both refusals, no commit; status `AM`; the index entry keeps the original's blob, `git cat-file -e` on it exits 0, and the parked pre-image is byte-identical to the original.

In neither is a byte destroyed, at any exit status.

### The previous release — rigs P1 and P2

Built and driven with the previous release's binary by its absolute path, its directory first on `PATH`.

| step | candidate (A · B) | previous release (P1 · P2) |
|---|---|---|
| `jigc validate` | 1 · 1, the route naming `jigc migrate <repo>/docs/research/UPPER.md --as research` | 1 · 1, the same route |
| `jigc migrate …/UPPER.md --as research` | 0 · 0 | 0 · 0 |
| `jigc doc author research … --task <id>` | 0 · 0 | 0 · 0 |
| `jigc task finalize <id>` | 4 · 4 | 4 · 4 |
| `jigc task finalize <id> --approve` | **3 · 3** | **3 · 3** |
| refusals | `finalize.stage-failed` · the same plus `finalize.rollback-conflict` | the same codes, the same `git add` line and the same pathspec error |
| commit | none · none | none · none |
| file afterwards | original, clean · rewrite, ` M`, original parked | original, clean (snapshot after the plant and after `--approve` identical) · rewrite, ` M`, original parked |

Two differences, neither in an exit or an end state: the previous release's author acknowledgement carries no `file-state.baseline-adopt` advisory, and its `finalize.stage-failed` route is the shorter sentence (*once the embedded git failure is resolved (e.g. remove a stale `.git/index.lock`)*).

## Step 2 — is it what the finding says?

**Its premise reproduces; the loss it wondered about does not.**

- *The destination and the source to retire are one file on this volume* — yes, and the binary shows it treating them as two: the hold promises to write one spelling and delete the other, and the quoted `git add` carries both as pathspecs.
- *What finalize does there* — it refuses. I tried the ways a refutation of a loss is usually wrong: the rigs are mine and new; no status was read through a pipe; the end state was read from the directory, the index, `HEAD` and a file-by-file hash, not from the refusal's own sentence; the rewrite was made to differ so that *restored* and *left as written* could be told apart (in rig A they cannot); the source was put outside any commit twice (D, E); and the binary is the candidate by hash and by `command -v`.

**No settled decision intends the state it ends in, and none forbids it as a loss.** The sections that own the behaviour:

- design/auto-migration.md → *Path-collision guard*: when the resolved foreign source path **equals** a promote destination the retire is skipped, by a *normalized path comparison*, so that *the migration never deletes the doc it just wrote*. On this volume the comparison answers *not equal* for two spellings of one file, so the retire is planned. The section says nothing about a volume that folds case.
- design/auto-migration.md → *Honest bounds*, the M43 same-path carve-out: the in-place rewrite is what a same-path migration is meant to get. Rig C gets it; rigs A and B do not.
- design/finalize.md → *Rollback discipline*: *nothing jigc wrote inside a transaction survives that transaction's failure except where surviving it is the only way to avoid destroying someone else's bytes, and every such survival is named.* In rig B jigc's rewrite survives the failed transaction in the worktree and the survival is named — but no third party wrote anything; the two writers the compare-and-swap sees are this one finalize's own promote and its own retire. That is a statement about the rollback's sentence, and it is in *Left open*.
- DECISIONS.md, the M50 workbench entry: *the workbench predicate must fold case … a case-sensitive predicate is a hole on the majority filesystem*. Another predicate; it shows this volume is treated as an ordinary one, and the run declares no bound.

## Step 3 — does it break the clause, inside its scope?

**No.** The clause is `no-lost-files`, and its scope is the first sharpening of DECISIONS.md → *2026-10-04 — The exit rule, revised*: *in a healthy repository used as documented … no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for.* Held against that, term by term:

- **At exit 0.** The commands of the chain that exit 0 are `migrate`, `doc author`, the four commit-doc writes and `task validate`; each writes only under `.jigc/`. The command that touches the file, `--approve`, exits 3.
- **Destroys bytes no git object holds.** Nothing is destroyed. In A the file is restored; in B, D and E the pre-image is parked and its path printed. In D, where the bytes were in no git object, they are still on disk after the refusal and after the `discard --force` the next route names.
- **Commits content the user did not ask for.** No commit is made.

**What a reader could hold against this grade, stated so nobody has to find it.** The clause's own words are *we do not … write / update incorrect things*, and in rig B a tracked file is left rewritten in the worktree by a command that refused. I do not count that as a break of this clause, for two reasons the scope gives: the sharpening keys the clause to exit 0 and to a commit, and here there is neither; and the refusal names the state and both copies. Whoever reads the clause's first sentence without its sharpening would grade rig B the other way, and that is a reading for the human, not for me.

**What is real, and is not this clause's.** A migration of this file cannot land on this volume by the route `jigc validate` prints, and the refusal it ends in routes to a re-run that refuses again. That is the kind of fact `working-product`'s second half is about — *every refusal's route works as printed*. This row was not sent to me under that clause and I do not grade it there; it is the first item of *Left open*.

## Coverage

The finding makes no coverage claim. For the block's `pinned-by` I looked in the suites, not in a diff. Suite files under `crates/cli/tests` and `tooling-tests` carrying a case-folding term (`ignorecase`, `case-insensitive`, `case_insensitive`, `case-fold`, `casefold`, `eq_ignore_ascii_case`, `to_uppercase`, `to_ascii_uppercase`): **7**. Of those, two name `migrate` at all (`planning_record_schema.rs`, `sub_task_composition.rs`), each with a single case-term line. No suite file names `UPPER.md` or `Upper.md`. Suites naming `finalize.stage-failed`: 3; `finalize.rollback-conflict`: 5. A suite that reaches the cell by planting two spellings under other literals is outside that search, so this is *none found*, not *none exists*.

## Repro V-1

```yaml
claim: "on a case-insensitive volume, a committed conformant docs/research/UPPER.md migrated with `jigc migrate … --as research`, authored under the title Upper and approved at finalize is lost, or lands in a commit that deletes it"
verdict: REFUTED            # breaks-no-clause: the approve refuses, exit 3, no commit, no byte destroyed
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d   # the same exits and end states
precondition: "the volume is case-insensitive — a file written as `CaseProbe` answers a stat of `caseprobe`"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = a conformant research doc: the bytes `jigc task finalize` commits for `doc create research --title Upper` with its three slots set"
  - ["git", "add", "docs/research/UPPER.md"]
  - ["git", "commit", "-m", "docs: a hand-written research note"]
repro:
  - ["jigc", "validate"]
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]      # the route of step 1, as printed
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<id>"]   # stdin: title "Upper", the three slots; arm `differ` changes one slot's prose
  - "the commit doc: set-field type, set-field scope, set-slot summary, set-slot body"
  - ["jigc", "task", "finalize", "<id>"]
  - ["jigc", "task", "finalize", "<id>", "--approve"]
expect:
  - exit: 1
    stdout_contains: ["schema-conformance.unadopted-instance", "jigc migrate <repo>/docs/research/UPPER.md --as research"]
  - exit: 0
    stdout_contains: ["task minted: migrate-research-docs-research-upper-"]
  - exit: 0
    stdout_contains: ["research:upper"]
  - exit: 0
  - exit: 4
    stdout_contains: ["migration review required — nothing committed", "DELETE the foreign original `docs/research/UPPER.md`", "canonical rewrite → docs/research/upper.md"]
  - exit: 3
    stdout: ""
    stderr_contains: ["blocking · finalize.stage-failed", "`git add -- :(literal)docs/research/upper.md :(literal)docs/research/UPPER.md", "pathspec ':(literal)docs/research/upper.md' did not match any files"]
  after:
    - "HEAD is the commit it was before step 6; `git ls-files -s docs` is the one entry docs/research/UPPER.md with the plant's blob"
    - "docs/research holds the one entry UPPER.md"
    - "arm `same`: UPPER.md is byte-identical to the plant and `git status --short` is empty"
    - "arm `differ`: stderr also carries `blocking · finalize.rollback-conflict`; UPPER.md holds the rewrite (` M`); the one file under .jigc/displaced/finalize/docs/research/ is byte-identical to the plant; `git show HEAD:docs/research/UPPER.md` is byte-identical to the plant"
control: "the same bytes planted as docs/research/upper.md: step 1 exits 0, step 6 exits 0, and the commit is `M docs/research/upper.md`, one file"
observed: "<W>/jigc-rig-fresh-M7WbPI (A), -4qUGnf (B), -fVsQRq (C), -jVmFIq (D), -jkNujq (E); previous release <W>/jigc-rig-fresh-kwnzzv (P1), -uY8f38 (P2); the plant's bytes from <W>/jigc-rig-fresh-kFTZty (rig 0)"
pinned-by: "UNPINNED: found this round; no suite found that plants two spellings of one doc home"
```

**Pinnable as it stands: no.** Three reasons. Its expectations hold only on a case-insensitive volume and the CI runner's is case-sensitive, so a test made from it needs the precondition as a probe that selects the arm; what either binary answers on a case-sensitive volume is undriven and is not in the block. The route's operand is an absolute path under the rig's root, so the assertions compare after replacing that root. And step 6's `exit: 3` is the behaviour found, not a behaviour anyone ruled: a repair of the dead end (*Left open*, 1) would change that line. What is worth holding across such a repair is the `after:` list's substance — no commit that deletes the doc, and the original's bytes still reachable — and a test made from this block should assert that and leave the exit to the fix.

## Left open — hit on the way, not pursued

1. **The migration cannot land, and the refusal's route does not lead out.** Door `jigc task finalize`. `finalize.stage-failed` prints *`jigc task finalize <id> --approve` once the embedded git failure is resolved — a stale `.git/index.lock` removed, or the `.gitignore` rule dropped … the same re-run lands the commit*. Neither named cause is present; the re-run, as printed, exits 3 again — byte-identically in rig A, with `reconciliation.conflict-block` in rigs B and D. The way in is itself a printed route: `jigc validate`'s `jigc migrate <repo>/docs/research/UPPER.md --as research`. Both binaries. A `working-product` question (*every refusal's route works as printed*); I graded nothing under that clause.
2. **`finalize.rollback-conflict` says the file *changed while this finalize was running*; nothing but this finalize wrote it.** Rigs B, D, E, P2. The first refusal's sentence, *the promotions were rolled back*, and the second's, *the rollback did not restore it*, stand in one stderr. The tracked file is left holding the rewrite, uncommitted, by a command that refused.
3. **`reconciliation.conflict-block` on the re-run names *an edit made to the file since* the mint; the edit is the first attempt's own write.** Rigs B and D. Its second route — *undo the edit made since the mint and run this finalize again, which lands the rewrite as authored* — I did not drive; by rig A, where the file holds the recorded source, that finalize is the one that exits 3. Its first route's first command, `jigc task discard <id> --force`, exits 0 and leaves the rewrite in the worktree and the parked pre-image in place (rig D); the `jigc migrate` that follows it in the route I did not drive.
4. **The review hold promises to write the canonical doc and DELETE the foreign original, naming two spellings of one file.** Rig A, step a8. Wording; what a human consents to there is not what can happen.
5. **`jigc ingest`'s route for this file — `git -C <repo> mv docs/research/UPPER.md docs/research/upper.md`, then re-run `jigc ingest` — was not driven.** It is the other command of the route in item 1 and may be the one that works.
6. **Undriven states of the same mechanism:** a case-sensitive volume; a repository whose index carries both spellings as two paths (a clone from a case-sensitive origin), the one state in which the `git add` of the lower-cased literal could match; a doctype other than `research`; the milestone commit boundary, which promotes and retires too.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my rigs under `<W>`: the install commit `jigc setup` makes in each; one of mine for the plant in each of rigs A, B, C, D, P1 and P2; jigc's own in rig 0 (the control doc) and rig C (the in-place migration). Rigs A, B, E, P1 and P2 each hold one minted, unfinalized migration task; rig D's was discarded. Besides the rigs, `<W>` holds the driver script I ran the rigs with and the captured output of every step.

<!-- end of report -->
