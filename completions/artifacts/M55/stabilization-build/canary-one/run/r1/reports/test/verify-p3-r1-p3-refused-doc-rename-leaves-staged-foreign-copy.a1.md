# verify-real — `r1-p3-refused-doc-rename-leaves-staged-foreign-copy` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-refused-doc-rename-leaves-staged-foreign-copy`
- **door:** `jigc doc rename`
- **clause it is said to break:** `no-lost-files` (triage's grade: *unclear*)
- **verdict:** `refuted` — as a blocker. The behaviour is real and reproduces on both binaries; it breaks no clause inside the clause's scope.
- **basis:** `breaks-no-clause` — the refused `jigc doc rename … --task` (exit 1) does write the foreign file's bytes into the task's working area as `research:upper`, but no command at exit 0 destroys a byte or commits anything: the staged bytes are the blob `HEAD` already holds, the worktree file and `HEAD` are unchanged after every door driven, `jigc task finalize` (with and without `--approve`) refuses at exit 3, and `jigc task discard` refuses without `--force`.
- **regression:** not a field of a `refuted` verdict. The fact, since triage asked for both binaries: the previous release does the same, its stdout and stderr byte-identical at every step compared (`cmp` exit 0).
- **contested:** `false` — the finding argues no decision wrong.
- **class:** `instance, unbounded`. I enumerated no consumer set. What was driven is under *What I drove*.

`<scratch>` is the scratch root the prompt names; `<W>` is my own directory under it, `<scratch>/verify-p3-staged.dERrA5`, minted with `mktemp -d`. `<repo>` is the repository of whichever rig a row names; `<task>` is the id the rig's `jigc migrate` minted (`migrate-research-docs-research-upper-192580f14883` in every rig that ran one).

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path, exit 0, in every shell that drove anything — the candidate's directory for the candidate's rigs, the previous release's for its rig |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: `jigc 1.0.0-rc.24`. No build ran; nothing under `target/` was driven.

## What I drove

Four fresh rigs of my own, each built with `dev/jigc-rig <state> --binary <that binary's path>` under `SCRATCH=<W>`. Every exit status was read bare; stdout and stderr went to separate files under `<W>`; the printed rename route was lifted out of the emitted stderr and run through `sh -c`, never retyped. The task's working area was listed — every file, with its size and a content hash — immediately before and immediately after the refused rename, and the two listings were compared with `diff`.

| rig | binary | state | what was driven |
|---|---|---|---|
| C | candidate | `fresh` + committed `docs/research/UPPER.md` = `# Upper\n` | the adoption report's Repro V-1 steps 1 to 4, then `jigc doc list research --task`, `jigc task finalize`, and five doors more (below) |
| P | previous | the same | V-1 steps 1 to 4, `jigc doc list research --task`, `jigc task finalize` |
| D | candidate | the same | `jigc migrate`, then the rename as the task's FIRST doc verb — no refused `doc author` before it |
| E | candidate | `refs-post-hoc` (an ordinary committed, managed `research:context-loss`, and a live task that does not stage it) | a control: the same refused re-slug over a doc that IS a committed managed doc |

The volume folds case (a probe in `<W>`: a file made as `CaseProbe` answers to `caseprobe`). The `doc author` payload is the adoption report's, its three slots filled and `title:` set as the row names.

### 1. The block as handed — rig C (candidate), and rig P (previous release) beside it

| # | argv | exit (C) | exit (P) | the task's `docs/` after it |
|---|---|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/UPPER.md --as research` | 0 | 0 | `commit:<task>.md`, `provenance.json` (one entry, the commit doc, `created`) |
| 2 | `jigc doc author research --from-file - --task <task>`, `title: "Upper"` | 1 — `write.non-reparseable` | 1 | **no new file**; `provenance.json` now also records `research:upper` as `edited-from-base` (on the candidate with a `copied-in` witness naming git blob `4adf6825…`), and a `roles.json` binds `research` to `research:upper` |
| — | `jigc doc list research --task <task>` | 0 | 0 | stdout: *no `research` docs staged in task `<task>`* |
| 3 | the same, `title: "Upper notes"` | 1 — `write.identity-change`, routed at `jigc doc rename research:upper --to 'Upper notes' --task <task>` | 1 | unchanged from step 2 (the two listings identical) |
| 4 | that route, as printed | **1** — `write.identity-change`: *rename rejected: `research:upper` is committed …* | **1** | **one file more: `docs/research:upper.md`, 8 bytes** — `diff` of the listings before and after shows that one added line and nothing else; `cmp` against `<repo>/docs/research/UPPER.md` exits 0 |
| 5 | `jigc doc list research --task <task>` | 0 | 0 | stdout: `research:upper  docs/research/upper.md  managed` |
| 6 | `jigc task finalize <task>` | **3** | **3** | three `blocking · conformance.section-missing` rows — `## question`, `## findings`, `## sources` — each naming `docs/research/upper.md`; the task tree is unchanged by it |

After step 6, in both rigs: `HEAD` is the commit it was before step 1, `git status --short --untracked-files=all` is empty, `docs/research/UPPER.md` still reads `# Upper`. `git ls-files --error-unmatch docs/research/upper.md` exits 1 — git has no entry under that spelling, which is the *path git has no file at* of the finding.

Candidate against previous release: `cmp` exits 0 on stdout and on stderr of steps 2, 3, 4, 5 and 6. The one difference in the working area is the candidate's `copied-in` witness inside `provenance.json`, which the previous release does not write.

### 2. The same task, five doors further — rig C, candidate only

Driven to answer step 3 of the brief: can the stranded copy be lost, or be committed, at exit 0?

| argv | exit | what it printed, and what moved |
|---|---|---|
| `jigc task finalize <task> --approve` | 3 | the same three `conformance.section-missing` rows; `HEAD` unchanged, status empty |
| `jigc doc show research:upper --task <task>` | 1 | `blocking · store.unparseable` — the staged copy *does not parse: required section heading `## question` is missing* |
| `jigc task validate <task>` | 3 | the three blocking rows; an advisory `file-state.staged-copy` for `docs/research/upper.md`; and the advisory `schema-conformance.unadopted-instance` for `docs/research/UPPER.md` — the same file under both readings in one output |
| `jigc doc author research … --task <task>`, `title: "Upper"`, again | 1 | `write.non-reparseable`, as at step 2 |
| `jigc task discard <task>` | 1 | `blocking · task-discard.staged-prose` — *stages 2 doc(s) that no commit has a copy of — discarding it would destroy them: `commit:<task>`, `research:upper`* |
| `jigc task discard <task> --force` | 0 | *discarded task … dropped staged edits to: `commit:<task>` (transient), `research:upper`* |

After the forced discard: `.jigc/tasks` is empty, `HEAD` unchanged, status empty, `docs/research/UPPER.md` = `# Upper`, and `jigc doc list` prints `research:UPPER  docs/research/UPPER.md  unregistered` — the state before step 1. The only bytes the forced discard removed are a copy of the blob `HEAD` holds and the task's unfilled commit doc.

### 3. Is the copy the rename's own? — rig D, candidate

`jigc migrate` (0), then `jigc doc rename research:upper --to 'Upper notes' --task <task>` as the task's first doc verb: exit 1, the same `write.identity-change`. The listings around it differ in three lines: `provenance.json` grew (the `research:upper` entry, `edited-from-base`, and its `copied-in` witness), `docs/research:upper.md` appeared (8 bytes, the foreign file's hash), and `roles.json` appeared. `jigc doc list research --task <task>` then prints the `managed` row, exit 0.

So the staged file is written by the refused rename itself; no refused `doc author` is needed before it. In rig C the refused `doc author` of step 2 had already written the provenance entry and the role binding — that is item 1 of the adoption report's *Left open*, another finding's ground — and the rename added the file.

### 4. The control — rig E, candidate

Over an ordinary committed managed doc the live task does not stage: `jigc doc rename research:context-loss --to 'Other title' --task <task>` exits 1 with the same `write.identity-change` sentence, and the listings around it show the same shape — `provenance.json` grew by an `edited-from-base` entry with its witness, and `docs/research:context-loss.md` appeared, byte-identical to the committed file (`cmp` exit 0). `git status` empty, `HEAD` unchanged; `jigc task validate` reports the copy under the advisory `file-state.staged-copy` — *this task's in-flight version of the doc … no action needed*.

So a refused re-slug that copies the committed doc in first is what this door does for every committed doc it has not yet touched; it is not peculiar to the foreign file. What is peculiar to the instance is **which bytes** are copied: a foreign, never-adopted file, taken for a committed managed `research:upper` because `docs/research/upper.md` and `docs/research/UPPER.md` are one file on this volume.

## Step 2 — is it what the finding says?

Yes, in each of its three parts, on both binaries:

1. *After the refused rename the task's working area holds `research:upper.md` (the foreign bytes).* Observed, and the before-listing shows it was not there (section 1, step 4; section 3).
2. *`jigc doc list research --task <task>` prints `research:upper  docs/research/upper.md  managed`.* Observed, exit 0; the same call one step earlier printed *no `research` docs staged*.
3. *`jigc task finalize` blocks on three `conformance.section-missing` rows naming a path git has no file at.* Observed, exit 3.

I tried the ways such a result is usually wrong. The rigs and the scratch directory are mine and new. No exit status was read through a pipe and no decisive output was cut. Each binary was the one named, by hash and by `command -v`. The copy was not left by an earlier step and misattributed: the listing immediately before the rename lacks the file, and rig D reaches it with no earlier write at all. None of this survived as a refutation of the observation.

Against the design that owns the door — design/write-commands.md → *`jigc doc rename` — the in-task title change, split on committed-store identity (M48)*:

- **The copy-in beside a refused re-slug is the door's ordinary posture.** The section's second arm is *a committed doc copied in — retitle-only*, its discriminator is *the persisted provenance, not a fresh path probe*, and the refusal's own sentence says *a same-slug retitle of the staged copy is supported*. The suite `crates/cli/tests/doc_rename_in_task.rs` (`a_committed_doc_is_retitle_only_and_a_reslug_routes_at_jigc_rename`) asserts *the refused re-slug leaves the staged copy where it was*. The control (section 4) shows the same on an ordinary doc. I did not find a sentence that says a refused re-slug must copy nothing in when it is the task's first touch, nor one that says it may; the suite's assertion is over a copy an earlier, successful retitle had staged.
- **Copying a foreign file in as a committed managed doc is not intended.** The same section names the case and forbids it: the *in-location-squatter exception* exists so that a foreign file at a doctype's home is not read *as a committed managed identity*, and a route that then *blocks on a doc it cannot find* is the outcome it was written to prevent. Here the exception does not hold, because the source's path and the canonical destination differ only in case. That is the mechanism the adoption report's verdict already carries (`r1-adoption-route-for-non-doc-id-name-not-run`, its section 4); this finding is one more consequence of it, at one more door.

So the basis is not `intended`: the defect is real. It is also not `does-not-reproduce`.

## Step 3 — does it break the clause, inside its scope?

The clause is DECISIONS.md → *2026-10-04 — The exit rule, revised*, first clause, in the scope its first sharpening gives it: *in a healthy repository used as documented … no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for. Where jigc cannot tell (git fails, the index is unreadable) it refuses before writing.*

Term by term, against what was driven:

- **A command at exit 0.** The command that writes the copy exits **1**. The exit-0 commands in the block are `jigc doc list research --task` (it reads; the task tree is identical before and after it) and, at the end, `jigc task discard --force` (the user's stated consent, and see the next term).
- **Destroys bytes no git object holds.** Nothing is destroyed by the rename, the listing or the finalize: the worktree file, the index and `HEAD` are unchanged after each. The staged copy is byte-identical to the worktree file, whose blob (`4adf6825…`) is in `HEAD`'s tree — the candidate's own witness records that blob id. The forced discard removes that copy and an unfilled commit doc; the unforced one refuses.
- **Commits content the user did not ask for.** Nothing is committed. `jigc task finalize` exits 3 with and without `--approve`, and `HEAD` does not move.
- **Where jigc cannot tell, it refuses before writing.** That sentence is about a git that cannot answer. Git answers here; this is not that cell.

What the instance is, then: a refusal at exit 1 that leaves the task's gitignored working area holding a copy of committed bytes under a wrong identity, after which one read door reports that identity as `managed` at exit 0 and every door that could land it refuses loudly. That is a wrong statement on a read surface and a stranded task, not a lost file and not an incorrect write to the repository. It breaks no term of the first clause inside its scope, on the candidate or on the previous release.

Whether the same instance is material under another clause — the `managed` row is printed by the round's one door, `jigc doc list`, and the task has no way forward but the forced discard — is not the question I was handed, and I grade nothing else.

## Step 4 — the regression fact

Not a field of this verdict. Recorded as a fact because triage asked for both binaries: rig P, the previous release, a fresh rig, the same block — exits 0, 1, 1, 1, 0, 3 at steps 1 to 6, the candidate's exits, and `cmp` exit 0 on every captured pair of steps 2 to 6. Comparable: every door of the block exists on the previous release.

## Step 5 — coverage

The finding makes no coverage claim. For the `pinned-by` lines below, from the suites and not from any diff: `crates/cli/tests/doc_rename_in_task.rs` is the suite that holds the committed re-slug refusal, and its one test of it stages the doc through a successful retitle first; I found no test in it that makes the refused re-slug the task's first touch and reads the working area afterwards, and none whose doc is a foreign file at a case-differing name. I read that one suite's test list and the test named above, not every suite that spells `"rename"` (42 files do).

## Repro P-1

```yaml
claim: "on a volume that folds case, a refused `jigc doc rename research:upper --to 'Upper notes' --task <task>` in a migrate task over a committed docs/research/UPPER.md loses a file or writes something incorrect to the repository"
verdict: REFUTED
basis: breaks-no-clause          # the staged copy IS written, at exit 1; nothing is destroyed and nothing is committed
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exits, the same bytes
requires: "a volume on which `UPPER.md` and `upper.md` are one file"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: a research note"]
repro:
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]                   # <task> = the id on stdout's `task minted: ` line
  - ["jigc", "doc", "rename", "research:upper", "--to", "Upper notes", "--task", "<task>"]
  - ["jigc", "doc", "list", "research", "--task", "<task>"]
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "task", "discard", "<task>"]
  - ["jigc", "task", "discard", "<task>", "--force"]
expect:
  - exit: 0
    stdout_contains: "task minted: "
  - exit: 1
    stderr_contains: "blocking · write.identity-change — rename rejected: `research:upper` is committed"
    files: ".jigc/tasks/<task>/docs/research:upper.md exists afterwards and is byte-identical to docs/research/UPPER.md"   # the defect, as it stands today
  - exit: 0
    stdout: "id  path  state\nresearch:upper  docs/research/upper.md  managed\n"                 # the defect, as it stands today
  - exit: 3
    stderr_contains: "blocking · conformance.section-missing — `docs/research/upper.md`: required section heading `## question` is missing"
  - exit: 3
  - exit: 1
    stderr_contains: "blocking · task-discard.staged-prose"
  - exit: 0
  tree: "after EVERY step: `git rev-parse HEAD` is the setup's commit, `git status --short --untracked-files=all` is empty, docs/research/UPPER.md = `# Upper\n`; after the last, `.jigc/tasks` holds no task and `jigc doc list` prints `research:UPPER  docs/research/UPPER.md  unregistered`"
where: "<W>/jigc-rig-fresh-0ez5Dz (candidate, rename first, labels D1 and D2); <W>/jigc-rig-fresh-lmw1xp (candidate, the block as handed with the two refused authors before the rename, labels C1 to C13); <W>/jigc-rig-fresh-Jk8swl (previous, labels P1 to P6)"
pinned-by: "UNPINNED: no suite drives a refused in-task re-slug over a foreign file whose name differs from the slug by case"
```

**Pinnable as it stands: no.** The `tree:` line is the fact worth holding — no door of the block moves `HEAD`, the index or the worktree — but two things stop the block from becoming a test as written. Its second and third expectations describe today's defect, and the fix of the adoption finding's mechanism will change them (to what is the fixer's and the design's to say), so a test would pin the `tree:` line and the two finalize refusals and leave those two open. And the whole block needs a volume that folds case, which the CI runner's is not expected to be (not driven there — a guess, flagged as one): it needs an arm that states its skip out loud where `CaseProbe` and `caseprobe` are two files. I drove the seven steps in this order across rigs C and D, not in one rig: rig D ran the first three; rig C ran all of them with the two refused `doc author` calls between the first and the second.

## Repro P-2 — the control

```yaml
claim: "a refused in-task re-slug of an ordinary committed doc, as the task's first touch of it, copies the committed body in, byte for byte, and changes nothing outside the task's working area"
verdict: CONFIRMED          # as a fact about the door; it is not the finding and carries no grade
binary: candidate c1 (sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc); not driven on 1.0.0-rc.24
setup:
  - fixture: refs-post-hoc          # <task> = the live task the state leaves; it stages `vision:vision` and no `research` doc
repro:
  - ["jigc", "doc", "rename", "research:context-loss", "--to", "Other title", "--task", "<task>"]
  - ["jigc", "doc", "list", "research", "--task", "<task>"]
expect:
  - exit: 1
    stdout: ""
    stderr_contains: "blocking · write.identity-change — rename rejected: `research:context-loss` is committed"
    files: ".jigc/tasks/<task>/docs/research:context-loss.md exists afterwards, byte-identical to docs/research/context-loss.md; docs/provenance.json records `research:context-loss` as `edited-from-base`"
  - exit: 0
    stdout: "id  path  state\nresearch:context-loss  docs/research/context-loss.md  managed\n"
  tree: "`git status --short --untracked-files=all` empty and `HEAD` unchanged"
where: "<W>/jigc-rig-refs-post-hoc-lSctPt (candidate, labels E0 to E3)"
pinned-by: "UNPINNED: doc_rename_in_task's committed re-slug test stages the doc through a successful retitle first, so the first-touch copy-in beside a refusal is asserted by no test I read"
```

**Pinnable as it stands: yes** — on the `refs-post-hoc` fixture, with `<task>` read back from the fixture; nothing in it depends on the volume. Whether a refusal that is the task's first touch SHOULD leave the copy-in is a question the design section does not answer in a sentence I found; the block states what the door does.

## Left open — hit on the way, not pursued

1. **`jigc doc list research --task <task>` reports a foreign, non-conformant file as `managed` at a path git has no entry for**, at exit 0, after the refused rename (rigs C, D and P). It is the listing half of this finding; I graded it against `no-lost-files` only.
2. **`jigc task validate` describes one file under two readings in one output** (rig C): `file-state.staged-copy` for `docs/research/upper.md` — *no action needed* — beside `schema-conformance.unadopted-instance` for `docs/research/UPPER.md`, routed at the `jigc migrate` whose task is the one being validated.
3. **`jigc task discard` (unforced) says the task stages `research:upper` as a doc *that no commit has a copy of*** (rig C). The staged bytes are the blob at `HEAD`. Not checked: what the same call says before the rename, or in the control rig over an untouched copy-in.
4. **A refused in-task re-slug that is the task's first touch of a committed doc leaves a copy-in, a provenance entry and — where the workflow has a role for the type — a role binding** (rigs D and E; Repro P-2). Harmless in the control. Whether a refusal should stage anything is the design's to say.
5. **A foreign file at a case-differing name that DOES conform to its schema was not driven.** There the copy the refused rename stages would parse, and what `jigc task finalize` does with it is unknown to me. This is the cell in which the mechanism could reach the first clause; I did not go looking for it.
6. **A volume that does not fold case was not driven**, on either binary.
7. **Rigs D and E, and the five further doors of section 2, were driven on the candidate only.**

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else, before and after. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my four rigs under `<W>`: the commits the rig's own construction makes, and one of mine in each of rigs C, P and D (the planted research note). No `jigc task finalize` I drove committed anything.

<!-- end of report -->
