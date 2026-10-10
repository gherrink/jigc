# verify-real — `r1-p3-migrate-case-variant-path-says-git-holds-no-copy` (run canary-one, round 1, attempt 1)

- **Key:** `r1-p3-migrate-case-variant-path-says-git-holds-no-copy`
- **Door:** `jigc migrate`
- **Clause it is said to break:** `working-product`
- **Triage's grade:** breaks
- **Verdict: CONFIRMED.** **Regression: false** — comparable: the door exists on the previous release and gives the same exits and, with the rig's root replaced, the same bytes.
- **Basis, one line:** on a case-folding volume `jigc migrate docs/research/upper.md --as research` reads the committed `docs/research/UPPER.md`, refuses `migrate.source-untracked` saying git holds no copy of it, and the route it prints — `git add`, then the same `jigc migrate` — exits 0 changing nothing and then refuses byte-identically; the matching spelling in the same rig exits 0 and mints the task.
- **Contested:** false. The finding does not argue against a settled decision, and I found none that intends this.
- **Class:** instance, unbounded. I drove one door with one doctype (`--as research`) and one case-variant spelling; I did not enumerate the consumers of the trackedness predicate.

## The binaries

| | path, under the scratch root | sha256 asserted by `dev/stabilize-step hash` (`content_sha256`) |
|---|---|---|
| candidate c1, commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` |
| previous release, `1.0.0-rc.24` | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` |

Both hashes are the ones the prompt gave. With the candidate's directory first on `PATH`, `command -v jigc` printed the candidate's path, exit 0. Nothing was built and nothing under `target/` was driven. Every rig was built with `dev/jigc-rig fresh --binary <that path>`, its stdout alone captured, its exit read before the eval. The previous release was driven by its absolute path, never through `PATH`.

## The volume

In my scratch directory `<W>` a file written as `CaseProbe` answered a `stat` of `caseprobe` (exit 0, size 1) while the directory listing holds the one entry `CaseProbe`. The volume folds case. `git init` on it set `core.ignorecase` to `true` in every rig, which is git's own ordinary answer to such a volume; I set no git config by hand. git is 2.54.0.

## What I drove

Four fresh rigs of my own, none of the reporter's. Each exit status was read bare: the command redirected to two files, `$?` read on the next word, no pipe.

### Rig C — candidate, the block (`jigc-rig-fresh-XsEarB`)

Setup: `docs/research/UPPER.md` written as `# Upper` and a newline (8 bytes), `git add -- docs/research/UPPER.md` (exit 0), `git commit` (exit 0). `git ls-files -s -- docs` then holds the one row `100644 4adf6825757801289bb2746e4bc84de90470bee5 0 docs/research/UPPER.md`; `git status --short` is empty.

| # | command | exit | stdout | stderr |
|---|---|---|---|---|
| 1 | `jigc migrate docs/research/upper.md --as research` | 1 | 0 bytes | the refusal below, re-run spelled relative |
| 2 | `jigc migrate <repo>/docs/research/upper.md --as research` | 1 | 0 bytes | the refusal below, re-run spelled absolute |
| 3 | `git -C <repo> add -- docs/research/upper.md` — the route, as printed | 0 | 0 bytes | 0 bytes |
| 4 | `jigc migrate docs/research/upper.md --as research` — the re-run step 1 prints | 1 | 0 bytes | `cmp` with step 1's stderr: identical |
| 5 | `jigc migrate <repo>/docs/research/upper.md --as research` — the re-run step 2 prints | 1 | 0 bytes | `cmp` with step 2's stderr: identical |

The refusal of step 1, whole, with the rig's root written `<repo>`:

```
blocking · migrate.source-untracked — `docs/research/upper.md` is in neither this repository's index nor its HEAD — git holds no copy of it, and `jigc task finalize --approve` retires the source it migrates, so the file would be deleted from the worktree with nothing to recover it from and no deletion in the commit to say so
  at: docs/research/upper.md
  route: stage it with `git -C <repo> add -- docs/research/upper.md`, then re-run `jigc migrate docs/research/upper.md --as research` — the index is enough, the source need not be committed first
```

Step 2's refusal differs from it in one place: the re-run reads `jigc migrate <repo>/docs/research/upper.md --as research`.

After step 3: `git ls-files -s -- docs` compared with its copy from before the step — identical; `git status --short` empty; `git diff --cached --name-status` empty. After step 5: a checksum of every file of the repository outside `.git`, compared with the same list taken before step 1 — identical; `docs/research` holds the one entry `UPPER.md`; `HEAD` is the commit of the setup; `.jigc/tasks` does not exist, so no task was minted.

**What git itself answers in that rig, about the two spellings:**

| question | lowercase spelling | on-disk spelling |
|---|---|---|
| `git ls-files --error-unmatch -- <path>` | exit 1, *did not match any file(s) known to git* | exit 0, prints `docs/research/UPPER.md` |
| `git cat-file -e HEAD:<path>` | exit 128, *exists on disk, but not in 'HEAD'* | exit 0 |
| `test -f <repo>/<path>` | exit 0 | exit 0 |

So the sentence the refusal prints is true of the string and false of the file: the file the door read is in the index and in `HEAD`, under the name the directory holds.

**Control N — is this any path git does not know, or this one?** `jigc migrate docs/research/nosuch.md --as research` in the same rig: exit 1, stdout 0 bytes, stderr ``could not read the foreign `research` source at `docs/research/nosuch.md` `` with a route to check the path. A different refusal. The lowercase spelling therefore passed the door's readability leg — the door reached the committed file through the fold — and only the trackedness leg answered about the string.

**Control M — the matching spelling, same rig, same file, last of all.** `jigc migrate docs/research/UPPER.md --as research`: exit 0, stderr 0 bytes, stdout 127 lines opening `task minted: migrate-research-docs-research-upper-192580f14883`; the task's `source-path` holds `docs/research/UPPER.md`. The one difference between step 1 and this is the case of five letters.

### Rigs D — candidate, the control with a lowercase file (`jigc-rig-fresh-5p50We`, `jigc-rig-fresh-dRQiIG`)

`docs/research/lower.md` committed the same way, one rig for each spelling so that neither run meets the other's task.

| rig | command | exit | stdout line 1 | stderr |
|---|---|---|---|---|
| `-5p50We` | `jigc migrate docs/research/lower.md --as research` | 0 | `task minted: migrate-research-docs-research-lower-a20f600efd2e` | 0 bytes |
| `-dRQiIG` | `jigc migrate <repo>/docs/research/lower.md --as research` | 0 | `task minted: migrate-research-docs-research-lower-a20f600efd2e` | 0 bytes |

### Rig E — the previous release, the block (`jigc-rig-fresh-5O6UpM`)

The same setup and the same five steps, the binary by its absolute path (`$JIGC` checked equal to it before the first step; `--version` prints `jigc 1.0.0-rc.24`).

| # | command | exit | stdout | stderr |
|---|---|---|---|---|
| 1 | `<previous> migrate docs/research/upper.md --as research` | 1 | 0 bytes | `migrate.source-untracked` |
| 2 | `<previous> migrate <repo>/docs/research/upper.md --as research` | 1 | 0 bytes | `migrate.source-untracked` |
| 3 | `git -C <repo> add -- docs/research/upper.md` — as printed | 0 | 0 bytes | 0 bytes |
| 4 | the re-run step 1 prints | 1 | 0 bytes | identical to step 1 |
| 5 | the re-run step 2 prints | 1 | 0 bytes | identical to step 2 |

Index and tree compared before and after as in rig C: both identical. With each rig's root replaced by one token, the stderr of step 1 here and in rig C are byte-identical (`cmp` exit 0), and so are step 2's. Control M on this rig: `<previous> migrate docs/research/UPPER.md --as research` exits 0 and mints `migrate-research-docs-research-upper-192580f14883`.

## What triage asked, answered

- *A committed `docs/research/UPPER.md`, `jigc migrate` handed the lowercase spelling typed by hand, with no `jigc doc show` in front.* Driven so: no `jigc doc show` ran in any rig of mine. The refusal does not depend on that verb's route having been followed.
- *Relative and absolute.* Both refuse, exit 1, with the same code and the same `at:`; they differ only in how the re-run is spelled back.
- *The refusal's `git add` and re-run as printed.* The `git add` exits 0, prints nothing and leaves the index as it was; each re-run refuses with the bytes of its first run.
- *The control with matching case.* Exit 0 and a minted task, in the same rig (control M) and in fresh rigs, relative and absolute (rigs D).
- *Both binaries.* The same on both.

## Step 2 — is it what the finding says?

Yes. I tried four ways to make it not so, and none held.

1. **Stale state.** Fresh rigs, the file committed by me, the index read before and after. Nothing was inherited.
2. **A status read through a pipe, or an output cut short.** No command whose status I report was piped; the refusals are quoted whole from the files they were redirected to.
3. **Another binary.** The hash was asserted through the step tool, the candidate was first on `PATH`, and the rig's `$JIGC` is that path.
4. **Behaviour a settled decision intends.** The owner is [design/auto-migration.md](../../../../../design/auto-migration.md) → *Trackedness precondition (M51)*: the door asks *does git hold a copy — the index or `HEAD`*; *a source git holds no copy of is refused with `migrate.source-untracked` and a `Human` route naming `git add <path>`, after which the identical `jigc migrate` succeeds*. Here git holds a copy of the file the door read, in both places, so the refusal is outside the case the design gives it; and the identical `jigc migrate` does not succeed after the `git add`, so the route's own promise is not kept. [design/validation.md](../../../../../design/validation.md) registers the code for *an in-repo source git holds no copy of*, the same condition. I searched `DECISIONS.md`, `design/`, `implementation/decisions-pending.md` and the guides for a ruling on a case-variant source: none. The one entry that speaks of this at all points the other way — the M50 record of the workbench predicate, *a case-sensitive predicate is a hole on the majority filesystem*. The run declares no bound (`bounds` is empty in the run's state).

## Step 3 — does it break the clause, inside its scope?

The clause is the exit rule's second, and its measure is in [DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-04 — The exit rule, revised*, sharpening 2: *no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed*.

- **The first half is not broken.** Nothing that worked on the previous release stopped working: rig E is red where rig C is.
- **The second half is.** `migrate.source-untracked` is a refusal; its route is two commands; run as printed, the first exits 0 and stages nothing and the second is the refusal again, for ever. The route does not work as printed, and no other route is offered on that surface.
- **The layout.** A default macOS volume, git's own default configuration for it, a healthy repository, one committed file. Nothing was planted: the only unusual act is the case of a typed path, which every other tool on that volume accepts — and which this door itself accepts, as far as reading the file.

Two facts the reader of this verdict should have beside it, neither of which is mine to weigh. The round's scope names one door, `jigc doc list`; `jigc migrate` is in the run's state as excluded, *reached_but_excluded*. And the behaviour is not the candidate's doing.

## Step 4 — the regression fact

`regression: false`. The block ran on the previous release in a fresh rig (rig E) and is red there in the same way: exits 1 · 1 · 0 · 1 · 1, the same refusal bytes after the root is replaced. Comparable — the door, the code and the route all exist on `1.0.0-rc.24`.

## Coverage

The finding's block says no suite carries this cell. Derived from the suites, not from the diff: six suite files under `crates/cli/tests` name `migrate.source-untracked` (`git_span_aim`, `migrate_source_rules`, `flow52_acceptance`, `pre_dispatch_faults`, `retire_sink_validation`, `path_arg_occurrence_axis`); `tooling-tests` has none. A search of those six for any case-folding vocabulary or an upper-casing call (`ignorecase`, case-insensitive, case-fold, `to_uppercase`, `to_ascii_uppercase`) finds nothing. The nearest test is `migrate_source_rules::an_untracked_in_repo_source_is_refused_and_the_printed_git_add_makes_it_admissible`, which holds the route for a source whose typed spelling is the directory's. The bound of that derivation: it is a search for vocabulary over the suites that name the code, so a cell spelled with a literal mixed-case path and none of those words would escape it; I read no such literal in the two tests of `migrate_source_rules` that the search for *untracked* and *spelling* names.

## Repro P3-1

```yaml
claim: "on a case-insensitive volume, with docs/research/UPPER.md committed, `jigc migrate docs/research/upper.md --as research` (relative or absolute) reads the file, refuses `migrate.source-untracked` saying git holds no copy of it, and prints a route — `git add`, then the same migrate — whose first command exits 0 staging nothing and whose second refuses byte-identically"
verdict: CONFIRMED
regression: false          # the previous release: the same exits, the same bytes with the root replaced
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d
precondition: "the volume folds case — a file written as `CaseProbe` answers a stat of `caseprobe`"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - ["git", "-C", "<repo>", "add", "--", "docs/research/UPPER.md"]
  - ["git", "-C", "<repo>", "commit", "-q", "-m", "docs: a hand-written research note"]
repro:
  - ["jigc", "migrate", "docs/research/upper.md", "--as", "research"]            # 1
  - ["jigc", "migrate", "<repo>/docs/research/upper.md", "--as", "research"]     # 2
  - ["git", "-C", "<repo>", "add", "--", "docs/research/upper.md"]               # 3, the route as printed
  - ["jigc", "migrate", "docs/research/upper.md", "--as", "research"]            # 4, the re-run 1 prints
  - ["jigc", "migrate", "<repo>/docs/research/upper.md", "--as", "research"]     # 5, the re-run 2 prints
expect:
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · migrate.source-untracked — `docs/research/upper.md` is in neither this repository's index nor its HEAD — git holds no copy of it", "at: docs/research/upper.md", "stage it with `git -C <repo> add -- docs/research/upper.md`, then re-run `jigc migrate docs/research/upper.md --as research`"]
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · migrate.source-untracked", "then re-run `jigc migrate <repo>/docs/research/upper.md --as research`"]
  - exit: 0
    stdout: ""
    stderr: ""
    assert: "`git ls-files -s -- docs` is the one row for docs/research/UPPER.md it was before the step; `git status --short` is empty"
  - exit: 1
    stderr: "byte-identical to step 1"
  - exit: 1
    stderr: "byte-identical to step 2"
  tree: "every file outside .git has the checksum it had before step 1; docs/research holds the one entry UPPER.md; .jigc/tasks does not exist"
controls:
  - "same rig, `jigc migrate docs/research/UPPER.md --as research`: exit 0, stderr empty, `task minted: migrate-research-docs-research-upper-<hash>`, source-path = docs/research/UPPER.md"
  - "same rig, `jigc migrate docs/research/nosuch.md --as research`: exit 1, `could not read the foreign `research` source` — a different refusal, so the lowercase spelling was read"
  - "fresh rigs, docs/research/lower.md committed, typed as it is, relative and absolute: exit 0, a task minted"
observed: "candidate <W>/jigc-rig-fresh-XsEarB (the block and the two same-rig controls), -5p50We and -dRQiIG (the lowercase control); previous release <W>/jigc-rig-fresh-5O6UpM"
pinned-by: "UNPINNED: found this round; none of the six suites naming migrate.source-untracked carries a case-variant spelling"
```

**Pinnable as it stands: no**, for three reasons, none of them about the fact.

1. The expectations hold only where the volume folds case, and the CI runner's does not. A test made from the block needs the precondition as a probe that selects the arm. What either binary answers on a case-sensitive volume I did not drive; from control N one would expect the *could not read* refusal, and that is an expectation, not a measurement.
2. The absolute arm's operand and both routes carry the rig's root, so the assertions compare after replacing it, as the comparison between rigs C and E did here.
3. As written the block asserts the defect. The test-in-waiting is its last two expectations turned round — *the printed route, run as printed, ends in a migrate that exits 0, or the refusal prints a route that does* — and which of those the fix chooses is the fixer's and the design's, not this block's.

## Not driven

The untracked arm of the same spelling (another finding's block); a source staged and never committed; a volume that does not fold case; `core.ignorecase` set to `false` by hand; any doctype but `research`; a case variant in a directory component rather than the file name; any other door that asks the same trackedness question.

## Left open — hit on the way, not pursued

Nothing. I met no second defect and no other printed route that fails.

## Tree state

The repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, `git status --short` showing the one untracked directory `completions/artifacts/canary-one/r1/` it showed when I started. I built nothing, edited nothing, staged nothing and committed nothing there. Everything I wrote is under my own scratch directory, `<W>` = `vp3-migrate-case.VjJhgh` under the scratch root the prompt names, and this report.

<!-- end of report -->
