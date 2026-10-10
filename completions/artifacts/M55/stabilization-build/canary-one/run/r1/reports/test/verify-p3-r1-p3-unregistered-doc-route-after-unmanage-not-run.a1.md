# verify-real — `r1-p3-unregistered-doc-route-after-unmanage-not-run`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: item 3
of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-orphaned-doc-route-after-strand-not-run.a1.md`
— after the unmanage arm of that report's Repro VR-ROUTE-1, `jigc validate` prints
`file-state.unregistered-doc` with a route its reporter did not run. Door: `jigc validate`.
Clause it is said to break: `working-product`. Triage's grade: *unclear*. The finding carries no
repro block of its own; the state it starts from is the handed report's block.

## Verdict in one paragraph

**`refuted` — basis `does-not-reproduce`.** The state is real and I reached it from nothing on
both binaries: after the strand and `jigc unmanage docs/research/context-loss.md`,
`jigc validate` exits 0 carrying `file-state.unregistered-doc` for that path, routed
`jigc migrate <the doc's absolute path> --as research`. **What does not reproduce is a route
that fails.** Pasted to a shell exactly as printed, the command exits 0 and mints a migration
task; the workflow it composes, followed as composed — one `jigc doc author` batch, the plain
`jigc task finalize` (which holds at exit 4, as its own text announces), then the same finalize
with `--approve` — ends at exit 0 with one commit. After it the advisory is gone, `jigc doc list`
shows `research:context-loss` managed at `notes/research/context-loss.md`, `jigc doc show` of it
exits 0, the worktree is clean, and the doc's bytes are the ones it began with. The same holds
with the route pasted from a subdirectory of the repository. **The candidate and the previous
release agree on every step of both cells, byte for byte** once the rig's directory, the
binary's path, the two commit ids and my cell label are normalised — the two diffs are empty.

## The binary, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`,
  label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24). **It was driven**, because triage asked for both binaries.
- In every cell the driven binary's directory went first on `PATH`, and the driver stops unless
  `command -v jigc` prints that binary's path, unless the rig — built with
  `dev/jigc-rig refs-post-hoc --binary <that path>` — reports the same path as `$JIGC`, and
  unless the rig's repository lies under my own scratch directory. No cell stopped.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## What was driven

Five rigs of the state `refs-post-hoc`, each fresh, minted under
`<scratch>/verify-p3-unreg.PxlBWo/runs/`: one exploratory rig on the candidate, driven step by
step to learn where the route goes, and then four cells driven by one script start to end —
`cand.route`, `prev.route` (the route pasted from the repository root) and `cand.subdir`,
`prev.subdir` (the route pasted from `<repo>/docs`). Every command whose exit status is read ran
bare, standard output and standard error each to a file of its own; nothing was read through a
pipe. Nothing of the reporter's scratch root was used.

**What I changed from the block as handed: nothing in the setup.** The setup is Repro
VR-ROUTE-1's, then its unmanage arm:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"   # exit 0
    jigc config set docs-root notes                                            # exit 0, moves nothing
    git update-index --force-remove -- "bad<byte 0xFF>name.txt"                # exit 0
    jigc validate                                                              # exit 0, file-state.orphaned-doc
    jigc unmanage docs/research/context-loss.md                                # exit 0
    jigc validate                                                              # exit 0, file-state.unregistered-doc

The finding names no command for the route, so what follows it is mine to spell, and it is
spelled from what the binary printed: the route's backticked span is lifted out of the
`jigc validate` output of that same rig and handed to `bash -c` unedited; the task id is read off
the `task minted:` line; the author payload is the one the composed text prints, its `<…>`
values filled with the foreign source's own title, date and three slot texts; the two finalize
commands are the ones the composed text and then the hold print.

### The state the route starts from (both binaries, identical text)

| step | exit | what it printed |
|---|---|---|
| `jigc unmanage docs/research/context-loss.md` | 0 | `unmanaged docs/research/context-loss.md — dropped its file-state baseline; the file is left on disk` |
| `jigc validate` | 0 | two advisories; the second is the finding's (verbatim below) |
| `git status --porcelain` | 0 | `?? .jigc/config/manifest.yaml` — the knob, uncommitted |
| `jigc doc list` | 0 | four rows; `research:context-loss` has none |
| `jigc doc show research:context-loss` | 1 | `blocking · store.not-found — could not read research:context-loss at notes/research/context-loss.md` |
| `git ls-files -- docs/research notes/research` | 0 | `docs/research/context-loss.md` |

The advisory, verbatim but for the rig's own directory, the same on both binaries:

    advisory · file-state.unregistered-doc — committed doc `docs/research/context-loss.md` looks managed (it sits under a `research`-style directory) but was never adopted — a basename coincidence or an un-ingested foreign doc, not a tracked strand
      at: docs/research/context-loss.md
      route: adopt it with `jigc migrate <repo>/docs/research/context-loss.md --as research`, or ignore it if it is not meant to be managed

`<repo>` stands for the rig repository's absolute path, which the binary prints in full. This
reading, taken before the route runs, is also the control: with nothing done the advisory
stands at exit 0.

### The route, as printed, to where it ends

One row per step; a cell holds one value because it is the value in all four cells.

| # | the command | exit | what stood afterwards |
|---|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/context-loss.md --as research` — the printed span, through `bash -c` | **0** | `task minted: migrate-research-docs-research-context-loss-b7b20cdb5b4d`; the composed `migrate-research` workflow, the foreign source inside it, target `notes/research/<slug>.md`; `jigc task list` shows 2 active tasks; `git status` unchanged |
| 2 | `jigc doc author research --from-file - --task <task>` with the composed payload filled in | **0** | prints `research:context-loss`; `jigc doc list research --task <task>` shows it at `notes/research/context-loss.md`, `managed` |
| 3 | `jigc task validate <task>` | 0 | one advisory, `file-state.staged-copy`, route *no action needed* |
| 4 | `jigc task finalize <task>` | **4** | `migration review required — nothing committed. Re-run jigc task finalize <task> --approve to write the canonical doc, DELETE the foreign original docs/research/context-loss.md, and commit.`; the fidelity scan reads `(none)` twice and the two halves of the diff are the same eighteen lines; `git rev-parse HEAD` unchanged, the doc still at `docs/research/` |
| 5 | `jigc task finalize <task> --approve` | **0** | `finalized <sha7> — docs(research): adopt docs/research/context-loss.md as a managed research` / `added .jigc/config/manifest.yaml` / `deleted docs/research/context-loss.md` / `promoted notes/research/context-loss.md` / `3 files committed` |

Step 4's exit 4 is not a failure of the route: the composed text of step 1 says, before the
agent reaches it, *a plain finalize commits NOTHING — it renders the foreign source against the
canonical rewrite and holds (exit 4)*, and `design/auto-migration.md` → *The review gate
(fidelity)* owns it. The command the hold prints is step 5.

Where it ended, read back in all four cells:

| reading | exit | value |
|---|---|---|
| `jigc validate` | 0 | one advisory (`schema-conformance.repeatable-populated`, there from the start); **no `file-state.*` row at all** |
| `jigc doc list` | 0 | **five rows**; `research:context-loss  notes/research/context-loss.md  managed` |
| `jigc doc show research:context-loss` | **0** | the doc, its three sections |
| `git status --porcelain` | 0 | empty |
| `git show --stat HEAD` | 0 | `.jigc/config/manifest.yaml | 2 ++` and `{docs => notes}/research/context-loss.md | 0` — a rename with no changed line |
| `git ls-files -- docs/research notes/research` | 0 | `notes/research/context-loss.md` |
| `git hash-object notes/research/context-loss.md` | 0 | `3507022f2299fc2746a813610ea9732edd8d58ff` — the value `git hash-object docs/research/context-loss.md` printed at the first step |
| `git cat-file -t HEAD~1:docs/research/context-loss.md` | 0 | `blob` — the retired original is in history |
| `jigc task list` | 0 | 1 active task, the rig's own `ground-the-vision-in-research` |
| `jigc config get docs-root` | 0 | `docs-root = notes  (project)` |

**No byte lost in any cell.** One commit landed per cell, at step 5 and nowhere else:
`git rev-parse HEAD` printed the same commit at the first step and after the hold.

**The subdirectory cells.** The printed operand is absolute so that the route resolves from any
directory (`crates/cli/src/orphan.rs`, `unregistered_route`; `jigc migrate --help`: *resolved
against your current directory … it may be absolute, and must land inside the repository*).
Pasted from `<repo>/docs`, step 1 exits 0 with the same task id, and every later row is the same.

## Is it what the finding says?

The finding says a route was printed and nobody ran it. Run, it works. Read against the design
that owns it:

- `design/validation.md` → *Orphan detection*, *M40 two-tier route*: a path with no
  `FileStateRecord` is the unregistered tier, routed `jigc migrate <path> --as <doctype>` *or
  ignore* where the `migrate-<doctype>` workflow ships. `migrate-research` ships; the route
  names a verb that acts.
- `design/auto-migration.md` → *The `jigc migrate` verb*, *The review gate*, *Retire-the-foreign-
  original*: mint, author through the write verbs, hold at a plain finalize, `--approve` writes
  the canonical doc, retires the original and commits. Each step did that.
- The commit also carries `.jigc/config/manifest.yaml`. That is the migration stage's own fixed
  path set — `crates/cli/src/task.rs`, `stage_migration`: *jigc's git-tracked config layer* —
  and the receipt names it.

Nothing here argues that a settled decision is wrong, so nothing is contested.

## Does it break `working-product`, inside that clause's scope?

No, and for the plain reason that nothing fails — this is `does-not-reproduce`, not
`breaks-no-clause`.

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*: the
  second clause is *a working product others can use and rely on*; its instrument, in the
  second sharpening, is *no command that works on rc.24 in a supported layout stops working, and
  every refusal's route works as printed*. The run's opening names it `working-product`.
- **First half — nothing stopped working.** `norm/route.diff` and `norm/subdir.diff` — each the
  candidate's whole log against the previous release's, 508 lines a side, after the
  normalisation named above — are empty. Same exits, same standard output, same standard error.
- **Second half — the route works as printed.** `file-state.unregistered-doc` is an advisory at
  exit 0, not a refusal, so the sentence does not strictly reach it; held to the sentence anyway,
  the printed command exits 0 unedited and the workflow it opens ends with the advisory cleared.

## The regression fact

**Not owed, and so not returned**: step 4 of the brief runs with `confirmed` only. What was
established on the way stands as evidence all the same — both cells were driven on the previous
release, and it is green there exactly as on the candidate.

## Class

**Driven: one arm of the producer, at one doc — `instance, unbounded`.** The advisory is built
at one place, the `else` branch at `crates/cli/src/cli.rs:1684-1711`, and its route at
`orphan::unregistered_route`, which has two arms — *migratable* (driven here) and the
ignore-or-human fallback for a doctype with no `migrate-<doctype>` workflow (not driven; it
names no command). The diagnosis has two wordings, a `location:` home (driven) and a
`placement:` home (not driven). One doctype (`research`), one doc, one value of the knob, text
output only. I did not enumerate the doctypes the migratable arm reaches.

## Coverage

The finding's claim is *not run* about one reporter's pass, not *nothing tests this*; what
follows is for the block's `pinned-by` only. Checked against the suites, not a diff:
`crates/cli/tests` (511 `.rs` files, read recursively) and `tooling-tests` (22) were searched
for the advisory's code; 3 files name it — `orphan_detection.rs`, `placement_override.rs`,
`unclaimed_file_family.rs`. `orphan_detection.rs`,
`validate_routes_never_adopted_basename_coincidence_to_migrate_or_ignore`, asserts the route's
text, absolute operand included, and stops there. In those 3 files `"migrate"` appears as an
argv once, `placement_override.rs:179`, a helper that migrates a fresh foreign file by a
relative path and is not fed from the advisory. `route_followability.rs` has no line naming
`migrate`. So: the route's text is held; that the emitted command runs and its workflow ends
with the advisory cleared is held by no test I found. A suite that harvests routes without
naming the code would not be found by this search.

## Repro VR-UNREG-1

```yaml
claim: "after a docs-root strand and `jigc unmanage`, the route file-state.unregistered-doc prints — `jigc migrate <abs path> --as research` — fails or was never shown to work, breaking the clause working-product"
verdict: "REFUTED — does-not-reproduce. The printed command exits 0 unedited; its workflow ends at exit 0 with the advisory cleared and the doc managed at the resolved root, on the candidate and on the previous release alike."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
setup:                      # the handed block's state, then its unmanage arm; one fresh rig per cell
  - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <the binary>
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
  - ["jigc", "config", "set", "docs-root", "notes"]                                             # exit 0, moves nothing
  - ["git", "update-index", "--force-remove", "--", "bad<byte 0xFF>name.txt"]                   # exit 0
  - ["jigc", "unmanage", "docs/research/context-loss.md"]                                       # exit 0
setup-equivalent:           # NOT driven by this verifier; the handed report drove it up to the unmanage arm
  - fixture: refs-post-hoc
  - "write .jigc/config/manifest.yaml = `scalar:\n  docs-root: notes\n`"
  - ["jigc", "unmanage", "docs/research/context-loss.md"]
precondition:
  repro:
    - ["jigc", "validate"]
    - ["jigc", "doc", "show", "research:context-loss"]
  expect:
    - exit: 0
      stdout_contains: "advisory · file-state.unregistered-doc — committed doc `docs/research/context-loss.md` looks managed"
      stdout_contains_2: "route: adopt it with `jigc migrate <repo>/docs/research/context-loss.md --as research`, or ignore it"
    - exit: 1
      stderr_contains: "store.not-found"
repro:                      # <route> is the backticked span of the precondition's route line, split by a real shell, run unedited
  - "<route>"                                                         # cwd: the repository root; and, a second cell, <repo>/docs
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]   # stdin: payload below
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "validate"]
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "show", "research:context-loss"]
  - ["git", "status", "--porcelain"]
  - ["git", "ls-files", "--", "docs/research", "notes/research"]
payload: |
  title: "Context Loss"
  sections:
    - id: meta
      set:
        date: "<the date line of the source>"
    - id: question
      set:
        question: |-
          <<How does a coding agent lose the context it was given?>>
    - id: findings
      set:
        findings: |-
          <<Static rules files go stale and are read once, not just in time.>>
    - id: sources
      set:
        sources: |-
          <<The adoption trial records, 2026.>>
expect:
  - exit: 0
    stdout_contains: "task minted: migrate-research-"                 # <task> is the rest of this line
  - exit: 0
    stdout: "research:context-loss"
  - exit: 4
    stdout_contains: "migration review required — nothing committed."
  - exit: 0
    stdout_contains: "promoted notes/research/context-loss.md"
    stdout_contains_2: "deleted docs/research/context-loss.md"
  - exit: 0
    stdout_lacks: "file-state."
  - exit: 0
    stdout_contains: "research:context-loss  notes/research/context-loss.md  managed"
  - exit: 0
    stdout_contains: "# Context Loss"
  - exit: 0
    stdout: ""
  - exit: 0
    stdout: "notes/research/context-loss.md"
invariant: "`git hash-object` of the doc is the same before the setup at docs/research/ and after the last step at notes/research/; HEAD moves once, at the --approve step, and `git cat-file -t HEAD~1:docs/research/context-loss.md` prints blob"
observed: "<scratch>/verify-p3-unreg.PxlBWo — cand.route.log, prev.route.log, cand.subdir.log, prev.subdir.log; cand.explore.log for the step-by-step first pass; norm/ holds the normalised logs and the two empty diffs"
pinned-by: "UNPINNED: no standing test runs the emitted migrate command of this advisory and reads the store back — see Coverage"
```

**Pinnable as it stands: on Unix, yes; portably, through `setup-equivalent`, which I did not
drive.** The repro is plain argv over a state `trial_corpus.rs` already builds, one stdin
payload, and a route lifted from printed output the way `migrate_route_family.rs` lifts its own.
The handed `setup` needs a file name passed as raw bytes, and it holds in place, as a step, the
door landing its knob over a doc it left behind — another finding's defect — so a test should
take the manifest-written strand instead; that variant reaches the same orphan advisory by the
handed report's account, and whoever pins this must drive its unmanage-then-migrate half once
before trusting it.

## Left open

1. **The advisory's diagnosis is untrue of this doc.** It says the doc *was never adopted — a
   basename coincidence or an un-ingested foreign doc, not a tracked strand*, one command after
   `jigc unmanage` dropped a doc that had been adopted and stranded. The tier discriminates on
   `FileStateRecord` membership by design, and after `unmanage` there is none; the wording has
   no arm for *dropped a moment ago*. Same on both binaries; the route under it works. Not
   graded.
2. **`jigc migrate --help` sends an already-conformant file elsewhere** — *An already-conformant
   file needs no rewrite: adopt it with `jigc ingest` instead* — while this advisory routes a
   conformant, formerly managed doc at the rewrite door, whose end is a re-authoring and a
   destructive approval. `jigc ingest` from this state was not driven. Same on both binaries.
3. **The migration commit lands the uncommitted knob with it** (`added
   .jigc/config/manifest.yaml`), while the composed workflow's own text says *untracked files
   are left out of the commit*. The staging is the documented one (`stage_migration`) and the
   receipt names the file; the sentence in the composed text is the general finalize sentence.
   Same on both binaries. Not graded.
4. **The rig's live task after the route.** `ground-the-vision-in-research` holds a staged edge
   to `research:context-loss`; whether its finalize behaves once the doc sits at
   `notes/research/` was not driven.
5. **A title that slugs differently**, the route pasted from a directory outside the repository,
   a second paste of the route while its task is live, and `jigc task discard` of the minted
   task — none driven.
6. **The other arms of the same producer**: the `placement:` wording, the ignore-or-human
   fallback, and `--format json` of every step.

## Bounds — what this verification did not do

- One doc, one doctype, one value of the knob (`notes`); text output; one macOS host.
- The author payload is mine: faithful to the source, title unchanged, date transcribed. The
  route's end depends on an agent writing a payload the write verbs accept; I showed that one
  such payload exists and lands, not that every agent finds it.
- `setup-equivalent` was not driven here.
- I read the handed report, the run's opening, the exit rule's entry, `design/validation.md` →
  *Orphan detection*, `design/auto-migration.md` → *The mechanism*, and
  `implementation/pinning.md` → §3. I read no other finding's report and no other verifier's.

## Where the evidence is

- `<scratch>/verify-p3-unreg.PxlBWo/scripts/` — `lib.sh` (the step runner and the setup),
  `drive.sh` (the four cells), `payload.yaml`, and `drive0.sh`, `explore1.sh` … `explore3.sh`
  (the first pass).
- `<scratch>/verify-p3-unreg.PxlBWo/*.log` — every command, its exit status, its standard output
  and standard error, in order, one file per cell.
- `<scratch>/verify-p3-unreg.PxlBWo/runs/<cell>/` — one file per stream per step; the rigs are
  beside them.
- `<scratch>/verify-p3-unreg.PxlBWo/norm/` — the normalised logs, `route.diff` and
  `subdir.diff`.

<!-- end of report -->
