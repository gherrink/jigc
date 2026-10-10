# verify-real — `r1-p3-adoption-of-staged-source-no-deletion-row` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-adoption-of-staged-source-no-deletion-row`
- **door:** `jigc task finalize` (the `--approve` of a migration task)
- **clause it is said to break:** `no-lost-files` (triage's grade: *unclear*)
- **verdict:** `refuted` — as a blocker. The behaviour the finding describes is real and reproduces exactly; it is what a settled decision intends, and by the clause's own words it does not break the clause.
- **basis:** `intended` — the staged-only cell is admitted on purpose (DECISIONS.md → *2026-09-14 — M51 Increment 1 / T2*: *priced, not a defect left open*; design/auto-migration.md → *Trackedness precondition*: the index or `HEAD`), its outcome is stated in advance on the composed surface and named on the review hold, and after the approve git's object store still holds the source's bytes as an unreachable blob — so the clause's *bytes no git object holds* is not met; no ref, no reflog, not the index and not the commit holds them.
- **regression:** not established as a field (the verdict is not `confirmed`). The fact, for whoever wants it: the previous release does the same, its four captured outputs byte-identical to the candidate's once the rig's directory name and the commit's short hash are normalized.
- **contested:** `false` — the finding argues no decision wrong. What my verdict depends on is said under *What the verdict rests on, and what would overturn it*; it is one reading of five words of the clause, and I flag it rather than hide it.
- **class:** `instance, unbounded`. I enumerated no consumer set. What was driven is below.

`<W>` is my own scratch directory, `<scratch>/verify-p3-staged.eBiuLJ`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` is the repository of whichever rig a row names.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path, exit 0, in every shell that drove anything — the candidate's directory for rigs D, E and F, the previous release's for rig PA |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: `jigc 1.0.0-rc.24`. No build ran; nothing under `target/` was driven.

## What I drove

Four fresh rigs, each `dev/jigc-rig fresh --binary <that binary's path>` with `SCRATCH=<W>`. None is the reporter's. Every exit status was read bare; stdout and stderr went to separate files under `<W>`; the two commands of the refusal's route were lifted out of the emitted bytes and run through `sh -c` in rigs D and PA, never retyped.

| rig | binary | plants | what was driven |
|---|---|---|---|
| D | candidate | `docs/research/UPPER.md` = `# Upper\n` and `docs/research/has space.md` = `# Spaced\n`, both untracked — the reporter's rig D | the whole chain for each file in turn, through the staging route |
| PA | previous | the same two, untracked | the whole chain for `has space.md` |
| E | candidate | `has space.md` alone, untracked | the one-file form of the chain — the block below as written |
| F | candidate | `has space.md` alone, **committed** | the control: the same chain over a source `HEAD` holds |

The payload of every `jigc doc author research --from-file - --task <task>` is the one the composed workflow prints, its three slots filled with a short sentence each, `title:` = `Has space` (and `Upper notes` for `UPPER.md`).

### The chain, step by step (rig D, `has space.md`; rig PA and rig E gave the same exit at every step)

| # | argv | exit | what it showed |
|---|---|---|---|
| 0 | `git cat-file -e 5b1f435249164c6191179346af4744573c24c95e` — the id `git hash-object` computes for the plant, before anything | 1 | no git object holds the bytes yet |
| 1 | `jigc migrate '<repo>/docs/research/has space.md' --as research` | 1 | `blocking · migrate.source-untracked` — *git holds no copy of it, and `jigc task finalize --approve` retires the source it migrates, so the file would be deleted from the worktree with nothing to recover it from and no deletion in the commit to say so* · route: *stage it with `git -C <repo> add -- 'docs/research/has space.md'`, then re-run `jigc migrate … --as research` — the index is enough, the source need not be committed first* |
| 2 | the route's first command, as printed | 0 | `git status`: `A  "docs/research/has space.md"`; the index row is `100644 5b1f4352… 0 docs/research/has space.md`; `git cat-file -e` of that id now exits 0 |
| 3 | the route's second command, as printed | 0 | `task minted: migrate-research-docs-research-has-space-0701333d8617`, and the composed `migrate-research` workflow — whose finalize step says, before anything is authored: *A source that was only `git add`ed and never committed has no committed copy for that deletion to point at — it leaves no row in the commit and nothing to recover from — so commit the source first if you want one* |
| 4 | `jigc doc author research --from-file - --task <task>` | 0 | `research:has-space` |
| 5 | `jigc task finalize <task>` | 4 | *migration review required — nothing committed. Re-run … `--approve` to write the canonical doc, DELETE the foreign original `docs/research/has space.md`, and commit.* With `--format json` (rig E, exit 4): `"retires": ["docs/research/has space.md"]`. `git status` and `git log` unchanged. |
| 6 | `jigc task finalize <task> --approve` | 0 | *finalized d7a1066 — docs(research): adopt docs/research/has space.md as a managed research · promoted docs/research/has-space.md · 1 file committed* |

### Where the source's bytes are after step 6 — the question triage asked

Each row was read on rig D and on rig PA with the same result; rig E repeated them for the one-file form; rig D repeated them for `UPPER.md` (blob `4adf6825757801289bb2746e4bc84de90470bee5`).

| place | how it was read | result |
|---|---|---|
| the worktree | `test -e 'docs/research/has space.md'` | exit 1 — the file is gone |
| the commit | `git show --name-status --format= HEAD` | one row, `A docs/research/has-space.md`; **no row for the source** |
| the index | `git ls-files --stage -- 'docs/research/has space.md'`, and the blob id searched in the whole `git ls-files --stage` | no row; 0 hits |
| any ref's history | `git log --all --format=%H -- 'docs/research/has space.md'` | empty, exit 0 |
| any ref or reflog, by object | the blob id searched in `git rev-list --all --reflog --objects` (24 objects) | 0 hits; the one ref is `refs/heads/main`, `git stash list` is empty |
| **the object store** | `git cat-file -e <id>` · `git cat-file -t <id>` · `git cat-file -p <id>` · `git fsck --unreachable --no-reflogs` | exit 0 · `blob` · the bytes `# Spaced\n` (rig E: written to a file and `cmp` against the plant's bytes, exit 0) · `unreachable blob 5b1f435249164c6191179346af4744573c24c95e` |

So the answer has two halves, and both are facts:

- **No ref, no reflog, not the index and not the commit holds the source's bytes**, and the commit carries no deletion row. The finding's sentence is true as written, on both binaries.
- **A git object does hold them**: the loose blob the route's `git add` wrote. It is reachable from nothing. `git cat-file -p <id>` prints it to whoever has the id, `git fsck --unreachable` (or `--lost-found`) names the id to whoever has not, and git keeps such an object until a prune takes it — which nothing in the chain ran (`git count-objects -v` after the approve in rig E: 25 loose, 0 packed, `prune-packable: 0`). How long git keeps it is git's configuration and not something I drove.

### The control (rig F, candidate, the source committed first)

The same chain without steps 1 and 2: approve exit 0, *deleted docs/research/has space.md · promoted docs/research/has-space.md · 2 files committed*; the commit's rows are `D docs/research/has space.md` and `A docs/research/has-space.md`; `git log --all -- <path>` holds 2 commits; the blob is reachable from `refs/heads/main`; `git fsck --unreachable` prints nothing. This is the cell the composed step calls *recoverable from history*, and it is.

### The two binaries, side by side

Rig PA against rig D, with the rig's directory name replaced by one token and the approve's short hash by another: `cmp` exit 0 on the refusal of step 1, on the composed workflow of step 3, on the hold of step 5 (no normalization needed) and on the approve's stdout of step 6. The same exits, the same bytes, the same six rows of the table above.

## Step 2 — is it what the finding says, and is it a defect?

It is what the finding says. I tried the ways such a result is usually wrong: the rigs and the scratch directory are mine and new; no exit status was read through a pipe; nothing was cut; the binary is the candidate by hash and by `command -v`; the route was run as printed; and the plant was shown to be in no git object before the chain began (step 0), so the unreachable blob is the route's own product and not residue.

It is not a defect, because the design that owns the behaviour chose it, said so, and pinned it:

- **design/auto-migration.md → *Trackedness precondition (M51)*** — the door's fourth question is *does git hold a copy — the index or `HEAD`, a union*; a source git holds no copy of is refused with `migrate.source-untracked` and a `Human` route naming `git add <path>`, *after which the identical `jigc migrate` succeeds*. Steps 1 to 3 are that paragraph, run.
- **DECISIONS.md → *2026-09-14 — M51 Increment 1 / T2: an untracked migrate source refuses, and the Settle's own G-5 prediction is falsified*** — this exact cell, driven then on `dev/jigc-rig fresh`: *a source that was `git add`ed and never committed is admitted, and on `--approve` the worktree file goes, the commit carries no row for it, and `git log --all -- <path>` is empty*. And the ruling on it: *The staged-only cell is priced, not a defect left open: the index leg is what makes the printed `git add` sufficient, so nobody has to commit a foreign file in order to be allowed to retire it … What the wave owes such a cell is that no surface claim about it be false.* The decomposition entry of the same day says the same: *the predicate is known to git — the index, not HEAD — so a `git add`ed but uncommitted source is admitted*.
- **The pack step both packs ship** (`crates/cli/packs/dev/steps/migration-finalize.yaml` and its methodology twin) carries the sentence quoted at step 3; the composed workflow printed it on both binaries.
- **The hold names the deletion** in its text and in the `retires` key (design/auto-migration.md → *The review gate*; design/command-output-contract.md, the `retires` declaration), so the human approves the removal of a named file.
- **A suite pins it.** `crates/cli/tests/migrate_source_rules.rs` → `the_migration_finalize_step_states_what_both_admissible_cells_actually_do`, registered in `crates/cli/tests/groups/g_migrate.rs`: its second arm stages a source without committing it, migrates, approves, and asserts that the commit carries no row for it and that `git log --all -- <path>` is empty; its doc comment reads *Both cells are admissible on purpose. The staged-only one is not a defect to fix here*. Read from the suite's text, not run by me — the gate runs it.

## Step 3 — does it break the clause, inside its scope?

The clause is DECISIONS.md → *2026-10-04 — The exit rule, revised*, first sharpening: *In a healthy repository used as documented … no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for.*

- **Healthy repository, used as documented:** yes. The layout is a fresh setup and two hand-written files; the staging is the route jigc itself prints.
- **A jigc command at exit 0 that destroys bytes:** yes — the approve removes the worktree file at exit 0.
- **Bytes no git object holds:** **no.** At the moment of the removal, and afterwards, a git object holds them (the table above, last row), byte for byte.
- **Commits content the user did not ask for:** no. The commit holds the promoted doc and nothing else.

So the clause, read by its words, is not broken. The cell the clause was written against — the M51 datum, an untracked source deleted with *the bytes in no git object, the deletion named on no surface* — is the one step 1 refuses. Here the bytes are in an object, and the deletion is named on two surfaces before it happens (step 3's composed text, step 5's hold).

### What the verdict rests on, and what would overturn it

Two things, and I want neither taken for more than it is.

1. **The settled decision.** It is dated 2026-09-14; the clause is dated 2026-10-04. The later entry lists what it supersedes and this decision is not on that list, and I found no entry between the two or after that reopens the staged-only cell. So it stands as settled. I did not find — and so do not claim — that anyone weighed it against the revised rule explicitly.
2. **The reading of *git object*.** The copy that survives is the weakest kind git has: reachable from nothing, findable by `git fsck` or by an id nobody printed, and kept only until git prunes. The clause says *git object*, and that is what holds the bytes, so I read the clause as not broken. A reader who holds that the clause means *an object some ref, reflog or index reaches* would read this cell as breaking it — and would then be arguing that the M51 decision is wrong under the revised rule. The finding does not argue that, so this is `contested: false`; if the human wants that reading, it is a ruling, and this report is the evidence for it either way. The decision's own text supports neither reading on this point: it calls the cell *the same surface silence as the untracked cell, one cell over* and never mentions the blob, and the pack step's *nothing to recover from* is stronger than what I observed.

A second thing the verdict does **not** cover: a source changed in the worktree between the route's `git add` and the approve. There the index's blob and the worktree's bytes are different bytes, and the argument of the row above does not apply as it stands. I did not drive it (left open, item 5).

## Step 4 — the regression fact

Not a field of this return: the verdict is `refuted`. The comparison was made all the same and is under *The two binaries, side by side*: the previous release behaves identically, so nothing here is new on the candidate.

## Step 5 — coverage

The finding makes no claim that nothing tests the cell. From the suites: the arm named under Step 2 drives it, over a `vision` source at `docs/direction.md` rather than a `research` file, and asserts the two halves the finding states (no row, no history). No assertion I found anywhere in that suite reads the object store (`cat-file`, `fsck`, *unreachable*, *dangling*: 0 occurrences in the file), so the fact that a git object still holds the bytes is held by no test.

## Repro V-1

```yaml
claim: "a foreign source that was untracked, staged by `migrate.source-untracked`'s printed route and adopted leaves no deletion row in the adopting commit, and afterwards no ref, reflog or index entry holds its bytes — which breaks `no-lost-files`"
verdict: REFUTED
basis: intended     # the behaviour is exactly as claimed; it is the M51 T2 ruling's admitted cell, and a git object still holds the bytes
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exits, the same bytes
setup:
  - fixture: fresh
  - "write docs/research/has space.md = `# Spaced\n`, and do not add it"
  - "<blob> = the output of [git, hash-object, 'docs/research/has space.md']   # 5b1f435249164c6191179346af4744573c24c95e"
repro:
  - ["git", "cat-file", "-e", "<blob>"]
  - ["jigc", "migrate", "<repo>/docs/research/has space.md", "--as", "research"]
  - ["git", "-C", "<repo>", "add", "--", "docs/research/has space.md"]                 # the route step 2 prints, as printed
  - ["jigc", "migrate", "<repo>/docs/research/has space.md", "--as", "research"]       # <task> = the id on stdout's `task minted: ` line
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin: the payload below
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["git", "show", "--name-status", "--format=", "HEAD"]
  - ["git", "log", "--all", "--format=%H", "--", "docs/research/has space.md"]
  - ["git", "ls-files", "--stage", "--", "docs/research/has space.md"]
  - ["git", "rev-list", "--all", "--reflog", "--objects"]
  - ["git", "cat-file", "-p", "<blob>"]
  - ["git", "fsck", "--unreachable", "--no-reflogs"]
expect:
  - exit: 1                                   # no object holds the plant before the chain
  - exit: 1
    stderr_contains: "blocking · migrate.source-untracked"
  - exit: 0
  - exit: 0
    stdout_contains: "only `git add`ed and never committed has no committed copy"      # after whitespace is flattened; the step wraps
  - exit: 0
    stdout: "research:has-space\n"
  - exit: 4
    stdout_contains: "DELETE the foreign original `docs/research/has space.md`"
  - exit: 0
    stdout_contains: "  promoted docs/research/has-space.md\n  1 file committed\n"
  - exit: 0
    stdout: "A\tdocs/research/has-space.md\n"
  - exit: 0
    stdout: ""
  - exit: 0
    stdout: ""
  - exit: 0
    stdout_lacks: "<blob>"
  - exit: 0
    stdout: "# Spaced\n"
  - exit: 0
    stdout: "unreachable blob <blob>\n"
  tree: "docs/research/has space.md absent from the worktree; `git status --short --untracked-files=all` empty; `jigc doc list` = `research:has-space  docs/research/has-space.md  managed`"
stdin: |
  title: "Has space"
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
where: "<W>/jigc-rig-fresh-kAcbln (candidate, the block as written, captures under <W>/E); <W>/jigc-rig-fresh-RPbqTJ (candidate, the two-file plant, <W>/D); <W>/jigc-rig-fresh-ADvNhJ (previous, <W>/PA); the committed control in <W>/jigc-rig-fresh-DFnKhw (<W>/F)"
pinned-by: "migrate_source_rules::the_migration_finalize_step_states_what_both_admissible_cells_actually_do — for the no-row and no-history halves, over a `vision` source; UNPINNED for the last two expectations: no suite reads the object store after a staged-only retire"
```

**Pinnable as it stands: yes, in two parts.** Everything down to the `git ls-files` line is jigc's behaviour on the `fresh` fixture, with `<repo>`, `<task>` and `<blob>` read back from the run, and its substance is already held by the arm named in `pinned-by`. The last three expectations pin a fact about git as much as about jigc — that `git add` writes a blob and that nothing in jigc's finalize prunes — and whether a suite should hold that is the design's to say: it is the fact this verdict leans on, so I would pin it, but as an assertion that the object exists (`git cat-file -e`), never as the text of `git fsck`, whose wording belongs to git. In the block as I drove it (rig E) the `git log`, `git ls-files` and `git rev-list` outputs were captured to files and searched there; the `stdout` rows above state what those files held.

## Left open — hit on the way, not pursued

1. **`migrate.source-untracked` gives two reasons and its route cures one.** The refusal says the file *would be deleted from the worktree with nothing to recover it from and no deletion in the commit to say so*, then routes to `git add` with *the index is enough*. After that route the adopting commit still carries no deletion — the second reason stands exactly as it was — while the composed step, one command later, says so plainly. The M51 ruling owes this cell *that no surface claim about it be false*; whether the refusal's sentence is such a claim is not mine to grade. Both binaries, byte-identical.
2. **The exit-0 ack does not name the removal in this cell.** The approve prints *promoted … · 1 file committed*; in the committed cell (rig F) it prints *deleted docs/research/has space.md* first. In the staged-only cell the deletion is named on the hold (exit 4) and in the composed step, and on no surface of the command that performs it; the commit's subject says *adopt docs/research/has space.md*. Both binaries.
3. **The composed step says *nothing to recover from*, and the object store holds the bytes.** The statement is stronger than the fact in the direction of caution; DECISIONS.md's *honestly stated for the staged-only one* is the sentence it would be held to.
4. **`schema-conformance.unadopted-instance` calls an untracked file, and a staged-only one, a *committed file*** — on `jigc validate` over the untracked plants (rig D), and in the advisory block the approve prints (rigs D, E and PA), where the file it names is the one that same call retires. The second half is the reporter's own item 4, seen again here.
5. **Not driven: a source edited after the route's `git add` and before the approve.** The index would then hold one version and the worktree another, and what the retire does to bytes only the worktree holds I do not know. It is the nearest cell to this one in which *a git object holds the bytes* might not be true.
6. **Not driven: how long the unreachable blob lasts.** Nothing in the chain pruned it; I ran no `git gc` and read no prune configuration.
7. **The whole chain for `UPPER.md` was driven on the candidate only** (rig D); on the previous release, `has space.md` only.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else, before and after. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my four rigs under `<W>`: the install commit `jigc setup` makes in each, one of mine in rig F (the committed plant), and the commits the driven `jigc task finalize --approve` calls made.

<!-- end of report -->
