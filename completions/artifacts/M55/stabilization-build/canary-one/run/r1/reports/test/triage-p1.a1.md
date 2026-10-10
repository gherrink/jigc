# canary-one, round 1, test stage - triage, pass 1 (reporter `triage-p1`, attempt 1)

Status: **graded**. 54 findings in, from 9 sources; **34 entries**; **20 merged**, in 13 named
merges. 33 keys are new, 1 is found again (`canary-seeded-claim`). By grade: **breaks 1 -
unclear 9 - needs-bound 1 - no-break 23 - out-of-scope 0**. Ten keys go to a verifier.

I found none of these, re-drove none of them and fixed none of them. I drove no binary, built
no rig, ran no git command against the candidate's history, edited nothing and committed
nothing. The one file I wrote is this report. Nothing below marks a finding confirmed or
refuted, says whether one is a regression, or says what is done about one: where a reporter
says "the same on the previous release", that is quoted as the reporter's sentence, and the
fact stays the verifier's.

## What I read

- The opening record, `completions/artifacts/canary-one/opening.md`, and the closing condition
  it names by reference: `DECISIONS.md`, the entry of 2026-10-04, *The exit rule, revised* -
  four clauses, the first with its scope sharpened, each with one instrument.
- The run's state, by `dev/stabilize-record state --run canary-one` (exit 0, one JSON
  document): opened; round 1 with a scope and no triage or record; **doors: 1 included
  (`jigc doc list`), 47 excluded**; **bounds: none**; the ledger: one row,
  `canary-seeded-claim`, ungraded, open.
- The read the prompt names for the source `ledger`, `dev/stabilize-record untriaged --run
  canary-one` (exit 0): `count` 1, the row `canary-seeded-claim`, `why: ungraded`.
- Every report of the stage, whole - the eleven files of attempt 1 under
  `completions/artifacts/canary-one/r1/reports/test/`: attempt, scope, preflight,
  preflight-second, row-doc-list-source, row-doc-list-driver, row-doc-list-reconciler,
  cross-cutting-review, arm-control-rehearse, arm-control-run, arm-control-score.

**The halt conditions, each checked and none met.** The prompt lists eleven reporters; the
directory holds eleven files of attempt 1 and no other; the state counts `test_reports: 11`.
Each of the eleven ends with the report's end line, so none is cut off. The state reads. The
opening names a closing condition I could read.

## How I read the clauses - what every grade below rests on

No bound is declared for this run, so the grade *out-of-scope* could cite nothing and is used
nowhere.

- **`no-lost-files`** - as the first sharpening scopes it: in a healthy repository used as
  documented, no jigc command at exit 0 destroys bytes no git object holds or commits content
  the user did not ask for; where jigc cannot tell, it refuses before writing; and
  "deliberately planted states are declared bounds, written down with their reach". A finding
  that writes something incorrect only from a planted state is, as I read it, out of that scope
  and under no declared bound: **needs-bound**, the human's.
- **`working-product`** - its instrument sentence is "no command that works on rc.24 in a
  supported layout stops working, and every refusal's route works as printed"; no scope is
  written for it beyond that. Two shapes leave me in doubt and are graded **unclear**, never
  *breaks*: a refusal whose route does not lead out (it fails as printed, loops back to where
  the reader came from, or is absent), and an exit-0 answer that says the opposite of a
  blocking verdict. A read that answers less, or from another place, at exit 0, with nothing
  written, is graded **no-break**.
- **`usable-by-agents`** - its instrument is the trial arms. A finding that bounds what a green
  arm is evidence of, and names no behaviour of the candidate, breaks no clause; it is a row,
  and it is listed under *For the human's eye* below so that the grade does not hide it.
- **`migration-works`** - no instrument exists. One lead names it (LD-2) and is unclear.

A reporter's severity was read as information and decided nothing: of the rows sent to a
verifier, entries 3, 13 and 19 were called low by their reporters, and entry 30 carries no
severity at all.

## The arithmetic

| source | findings in | their ids |
|---|---|---|
| row-doc-list-source | 7 | S1 S2 S3 S4 S5 L1 L2 |
| row-doc-list-driver | 7 | DL-1 DL-2 DL-3 L1 L2 L3 L4 |
| row-doc-list-reconciler | 16 | RC-1 ... RC-13, LD-1 LD-2 LD-3 |
| cross-cutting-review | 4 | F1 L1 L2 L3 |
| arm-control-rehearse | 5 | ACR-1 ... ACR-5 |
| arm-control-run | 3 | RUN-1 RUN-2 RUN-3 |
| arm-control-score | 5 | SCO-1 ... SCO-5 |
| scope | 6 | left open 1 ... 6 |
| ledger | 1 | canary-seeded-claim |
| **total** | **54** | |

54 in; 34 entries; 54 - 34 = 20 merged. Every one of the 54 is in exactly one entry's sources,
in the table of entries below; no finding was split.

### The merges, each named

A merge is made only where the door and the broken behaviour are the same. Thirteen entries
carry more than one finding; the findings folded away number 2+1+3+2+2+1+1+1+1+1+1+2+2 = 20.

| key | findings folded into it | why they are one |
|---|---|---|
| `r1-doc-list-unreadable-entry-fails-listing` | source S3, driver DL-1, reconciler RC-1 (3) | one door, one defect: an entry named `*.md` that cannot be read ends the whole listing at exit 1. RC-1 says so itself: "the same defect, found twice" |
| `r1-read-verb-writes-invocation-log-unnamed` | source S1, reconciler RC-3 (2) | the same write under the same knob at the same door. The driver saw the same bytes and returned no finding for them, so there is nothing of the driver's to fold |
| `r1-invocation-log-write-follows-link` | source S2, reconciler RC-4, cross-cutting F1, cross-cutting L2 (4) | one door, one site (`invocation_log.rs:638-659`): the log's opening write goes where a link under `.jigc/logs` points. S2 drove a link to a file outside the repository, F1 a dangling link and a link to a tracked file in the working tree, RC-4 all of those and `.jigc/logs` itself as a link to a directory - **which is exactly cross-cutting L2**, a lead there, read at source and not driven. L2 is folded because RC-4 returns that shape as part of its one finding; a second key would put one driven shape on two rows. What the fold must not hide: the cross-cutting reviewer wrote of that one shape "a user who relocated the log this way would want exactly that, so it may be no defect" |
| `r1-doc-list-served-from-no-checkout-home` | source S4, driver DL-2, reconciler RC-5 (3) | one door, one behaviour, three layouts: S4 drove two, DL-2 the third, RC-5 all three |
| `r1-setup-installs-outside-work-tree` | driver DL-3, source L1, reconciler RC-6 (3) | one door, `jigc setup`, in the same three layouts. DL-3 is the bare-worktree cell (exit 1 after writing, the route loops), L1 the submodule cell (exit 0, the install in the superproject's git directory, a hook in its hooks directory) and the `--separate-git-dir` sibling; RC-6 returns all of it as one finding and names both as what it reconciles. **The fold carries two facets, and both ride the row:** the looping route, and the superproject's `pre-commit` hook rewritten at exit 0 with the user's hook body kept |
| `r1-read-verb-fence-one-form-of-doc-list` | source S5, reconciler RC-7 (2) | the same reading of the same test file |
| `r1-doc-list-non-utf8-path-drops-orphan-rows` | source L2, reconciler RC-8 (2) | L2 is the lead, from source and not driven; RC-8 is that lead driven |
| `r1-served-from-note-nested-worktree-path` | driver L1, reconciler RC-10 (2) | RC-10: "driver lead L1 ... Reproduces" |
| `r1-task-read-before-setup-answers-no-task` | driver L2, reconciler RC-11 (2) | RC-11: "driver lead L2 ... Reproduces, and the two other `--task` reads driven here do the same" |
| `r1-doc-list-prints-id-doc-show-refuses` | driver L3, reconciler RC-12 (2) | RC-12: "driver lead L3 ... Reproduces; the follow-up was driven" |
| `r1-doc-list-reads-through-link-out-of-repo` | driver L4, reconciler RC-13 (2) | RC-13: "driver lead L4. Reproduces" |
| `r1-arm-control-does-not-reach-changed-path` | rehearse ACR-2, run RUN-1, score SCO-1 (3) | one statement about one door, made by each step of the arm's chain; RUN-1 and SCO-1 each name ACR-2 as what they met again |
| `r1-observe-exits-1-on-scripted-arm-evidence` | rehearse ACR-4, run RUN-2, score SCO-4 (3) | the same reader, the same exit, the same cause; SCO-4 adds a failed arm beside the passing one |

**Kept apart, though they resemble a neighbour** - each is its own key:

- RC-2 (the staged arm's two failing sites) from RC-1: other sites (`doc.rs:5023`, `:5040`),
  another arm, and a state in the workbench and not at a managed home. The reconciler returned
  two findings.
- RC-9 (`jigc validate`) from RC-8 (`jigc doc list`): one state, two doors.
- Cross-cutting L3 (`jigc uninstall`) and L1 (`jigc task finalize`) from the log-link row:
  other doors.
- ACR-1, RUN-3 and SCO-3 share the door `completions/trial-harness/run-session.sh --exec` and
  are three behaviours; ACR-3, SCO-2 and the scope's left open 4 share the arm's script and are
  three statements.
- The scope's left open 6 from the arm-control row: it says where the change lies in the
  handler, for every instrument; the arm row says what one arm does not reach.

## The entries

`R/` stands for `completions/artifacts/canary-one/r1/reports/test/`. A door is written in the
round's list's words where the reporter's door is a row of that list - the included row or an
excluded one; whether an entry is in the round's test set is the script's to compute and is
said nowhere here. "new" is a key minted here, in the ledger's key grammar, with the prefix
`r1-` for the round that found it.

| # | key | new | doctype | sources | door | clause | grade |
|---|---|---|---|---|---|---|---|
| 1 | `canary-seeded-claim` | found again | jigc-feedback | ledger: the opening's seeded row | jigc doc list | no-lost-files | **breaks** |
| 2 | `r1-doc-list-unreadable-entry-fails-listing` | new | jigc-feedback | source S3; driver DL-1; reconciler RC-1 | jigc doc list | working-product | **unclear** |
| 3 | `r1-doc-list-staged-arm-unreadable-entry` | new | jigc-feedback | reconciler RC-2 | jigc doc list | working-product | **unclear** |
| 4 | `r1-read-verb-writes-invocation-log-unnamed` | new | inconsistency | source S1; reconciler RC-3 | jigc doc list | none | no-break |
| 5 | `r1-invocation-log-write-follows-link` | new | jigc-feedback | source S2; reconciler RC-4; cross-cutting F1; cross-cutting L2 | jigc doc list | no-lost-files | **needs-bound** |
| 6 | `r1-doc-list-served-from-no-checkout-home` | new | jigc-feedback | source S4; driver DL-2; reconciler RC-5 | jigc doc list | none | no-break |
| 7 | `r1-setup-installs-outside-work-tree` | new | jigc-feedback | driver DL-3; source L1; reconciler RC-6 | jigc setup | working-product | **unclear** |
| 8 | `r1-read-verb-fence-one-form-of-doc-list` | new | jigc-feedback | source S5; reconciler RC-7 | jigc doc list | none | no-break |
| 9 | `r1-doc-list-non-utf8-path-drops-orphan-rows` | new | jigc-feedback | source L2; reconciler RC-8 | jigc doc list | none | no-break |
| 10 | `r1-validate-non-utf8-path-drops-blocking-orphan` | new | jigc-feedback | reconciler RC-9 | jigc validate | working-product | **unclear** |
| 11 | `r1-served-from-note-nested-worktree-path` | new | jigc-feedback | driver L1; reconciler RC-10 | jigc doc list | none | no-break |
| 12 | `r1-task-read-before-setup-answers-no-task` | new | jigc-feedback | driver L2; reconciler RC-11 | jigc doc list | none | no-break |
| 13 | `r1-doc-list-prints-id-doc-show-refuses` | new | jigc-feedback | driver L3; reconciler RC-12 | jigc doc list | working-product | **unclear** |
| 14 | `r1-doc-list-reads-through-link-out-of-repo` | new | jigc-feedback | driver L4; reconciler RC-13 | jigc doc list | none | no-break |
| 15 | `r1-doc-list-fsmonitor-cookie-mtime` | new | jigc-feedback | reconciler LD-1 | jigc doc list | none | no-break |
| 16 | `r1-non-utf8-path-other-orphan-walk-consumers` | new | jigc-feedback | reconciler LD-2 | the verbs behind config.rs:1512, orphan.rs:1116 and relocate.rs:708 (callers of orphan::committed_markdown) - not identified by the reporter | migration-works | **unclear** |
| 17 | `r1-doc-list-cells-driven-on-one-platform` | new | jigc-feedback | reconciler LD-3 | jigc doc list | none | no-break |
| 18 | `r1-finalize-may-commit-redirected-log-record` | new | jigc-feedback | cross-cutting L1 | jigc task finalize | no-lost-files | **unclear** |
| 19 | `r1-uninstall-log-append-through-live-link` | new | jigc-feedback | cross-cutting L3 | jigc uninstall | no-lost-files | **unclear** |
| 20 | `r1-run-session-refuses-scripted-arm-without-credential` | new | jigc-feedback | rehearse ACR-1 | completions/trial-harness/run-session.sh --exec | none | no-break |
| 21 | `r1-arm-control-does-not-reach-changed-path` | new | jigc-feedback | rehearse ACR-2; run RUN-1; score SCO-1 | jigc doc list | none | no-break |
| 22 | `r1-walk-arm-runs-in-caller-dir-outside-container` | new | jigc-feedback | rehearse ACR-3 | completions/trial-driver/arms/walk/00-positive-control.sh | none | no-break |
| 23 | `r1-observe-exits-1-on-scripted-arm-evidence` | new | jigc-feedback | rehearse ACR-4; run RUN-2; score SCO-4 | completions/trial-driver/run.py observe | none | no-break |
| 24 | `r1-config-set-leaves-manifest-untracked` | new | jigc-feedback | rehearse ACR-5 | jigc config set | none | no-break |
| 25 | `r1-provenance-names-model-no-session-used` | new | jigc-feedback | run RUN-3 | completions/trial-harness/run-session.sh --exec | none | no-break |
| 26 | `r1-control-pass-counts-failed-read` | new | jigc-feedback | score SCO-2 | completions/trial-driver/arms/walk/00-positive-control.sh | none | no-break |
| 27 | `r1-run-session-exits-0-when-arm-fails` | new | jigc-feedback | score SCO-3 | completions/trial-harness/run-session.sh --exec | none | no-break |
| 28 | `r1-arm-evidence-git-dir-touched-after-run` | new | jigc-feedback | score SCO-5 | `<scratch>/arm-control-run.zis4p3/out-run-1` | none | no-break |
| 29 | `r1-scope-registry-is-the-scope-steps-choice` | new | jigc-feedback | scope, left open 1 | no door named - the scope step's choice of registry (VERB_KINDS) | none | no-break |
| 30 | `r1-scope-other-registries-unread-two-changed` | new | jigc-feedback | scope, left open 2 | no door named - the rows of FINALIZE_FAMILY and ENVELOPE_OWED_CODES, crates/cli/src/render.rs, registries on neither side of the round's scope | working-product | **unclear** |
| 31 | `r1-scope-47-doors-reached-and-excluded` | new | jigc-feedback | scope, left open 3 | no door named - the 47 excluded doors of VERB_KINDS, each reached by the change | none | no-break |
| 32 | `r1-scope-control-arm-drives-excluded-doors` | new | jigc-feedback | scope, left open 4 | completions/trial-driver/arms/walk/00-positive-control.sh | none | no-break |
| 33 | `r1-scope-never-selected-lists-row-before-test` | new | jigc-feedback | scope, left open 5 | no door named - dev/stabilize-record state, its never_selected list | none | no-break |
| 34 | `r1-scope-change-sits-on-committed-arm-only` | new | jigc-feedback | scope, left open 6 | jigc doc list | none | no-break |

**Doors, where I wrote the list's text for a reporter's own.** Five doors, as their reporters
wrote them, carry the word "unlisted" beside a verb that is a row of the round's list, on its
excluded side: `jigc setup` (DL-3, RC-6, source L1), `jigc validate` (RC-9), `jigc task
finalize` (cross-cutting L1), `jigc uninstall` (cross-cutting L3), `jigc config set` (ACR-5).
Each is written as the list has it. None was moved to the included door, and none off the
list. The trial tooling's doors and the scope step's notes are rows of no list and are written
as their reporters have them, less the marker.

**Doctype.** `inconsistency` for the one entry that is a disagreement between texts and code
of this repository (entry 4: the read-verb rule at `cli.rs:1992-1999` and its fence's
restatement against the designed log write); `jigc-feedback` for every other - a behaviour of
the binary, of the trial tooling or of the run's own tooling (`design/findings-channel.md`,
section 1).

## Each entry - the sentence the grade rests on, and its repro block

### 1. `canary-seeded-claim` - breaks (`no-lost-files`) - found again

- **Graded from** the report its repro cell names: `completions/artifacts/canary-one/opening.md`,
  the block under the heading *The seeded finding*.
- **Why.** As claimed - "`jigc doc list` exits 0 and an untracked file in the repository's root
  is gone afterwards - bytes no git object holds, destroyed at exit 0 by a verb that only
  reads" - it is the first clause's own sentence, in a healthy repository `jigc setup` ran in.
  The opening says "Nobody has driven it for this run, and nothing here says whether it holds".
  Whether it holds is the verifier's.
- **Beside it, and not what the grade rests on:** three reporters of this stage say they drove
  that block and that the file stood - source *V1*, driver *Repro 1*, reconciler *Repro RC-0*,
  each on both binaries - and two more saw an untracked root file stand in other states
  (cross-cutting *Repro H1*, run *A6*). I mark nothing refuted on that: a verdict is a
  verifier's, on a drive of its own.
- The row keeps its key, its cells and its disposition (`open`).

### 2. `r1-doc-list-unreadable-entry-fails-listing` - unclear (`working-product`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-1*; also `R/row-doc-list-source.a1.md`,
  *Repro S3*, and `R/row-doc-list-driver.a1.md`, *Repro 2* (DL-1).
- **Why.** RC-1: one untracked entry named `*.md` that cannot be read as a file - a dangling
  link, a directory, a mode-000 file - at a located or a placement home "makes the listing exit
  1 with no rows and no route"; a named pipe hangs it; `jigc validate` over the same store
  exits 0 and does not name the entry. Nothing is written. All three reporters read *clause:
  none*, DL-1 on the ground that "the refusal prints no route, so no route fails as printed".
  That is one reading of "every refusal's route works as printed"; that a refusal with no way
  out, on what RC-1 calls "an ordinary accident", is no breach of it is not settled by the
  report. In doubt, unclear.
- The reporters' severity: medium for the verb's job, low against the closing condition.

### 3. `r1-doc-list-staged-arm-unreadable-entry` - unclear (`working-product`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-2*.
- **Why.** RC-2: `jigc doc list --task <id>` "exits 1 with a raw OS error" on a dangling link
  (`doc.rs:5023`) or a mode-000 file (`doc.rs:5040`) in the task's staging area, and with the
  dangling link planted the committed listing exits 0 without its staged note. The same shape
  as entry 2 - a refusal with no route - so the same doubt. One thing more for the verifier:
  the reporter says "the states sit inside the gitignored workbench, where only jigc writes in
  documented use". No bound is declared, and I infer none; whether that state is one the
  clause's scope reaches is part of what the report does not settle.

### 4. `r1-read-verb-writes-invocation-log-unnamed` - no-break (`none`) - inconsistency

- **Repro:** `R/row-doc-list-source.a1.md`, *Repro S1*; also `R/row-doc-list-reconciler.a1.md`,
  *Repro RC-3*.
- **Why.** S1: the write "is designed, opt-in and gitignored, it destroys nothing and commits
  nothing, and `git status --porcelain` does not move"; "what is wrong is that two texts say
  otherwise and the fence cannot see it". An append the operator switched on, to a file jigc
  ignores, is neither a destroyed byte nor content committed unasked. What stands is a
  disagreement between the read-verb rule's text, its fence, and the designed log - which is
  what the doctype says.

### 5. `r1-invocation-log-write-follows-link` - needs-bound (`no-lost-files`)

- **Repro:** `R/cross-cutting-review.a1.md`, *Repro F1* (the working-tree shapes); also
  `R/row-doc-list-reconciler.a1.md`, *Repro RC-4* (four shapes), and
  `R/row-doc-list-source.a1.md`, *Repro S2*.
- **Why.** F1: with the knob on, "a read verb exits 0 having minted an untracked, un-ignored
  file in the repository root, or having appended a JSON line to a tracked file" - content
  written where nobody asked for it, the first clause's subject. And F1 again: "a link at a
  path inside the gitignored workbench, which no jigc door creates, is a planted state - so
  this is a bound to declare ... This run declares no bound, so the row needs a bound or a
  ruling". S2 and RC-4 read the state the same way. As I read it: out of the first clause's
  scope as a deliberately planted state, and under no declared bound. That is what this grade
  is for; a bound is the human's ruling and I cite none.
- **What a bound's reach would have to say, from the reports:** four link shapes at one site
  (a link at the file to an existing file - appended; a dangling link at the file - its target
  created; `.jigc/logs` a link to a directory - the log created there; `.jigc/logs` a dangling
  link - nothing written); the site is reached by 47 of the 48 leaf verbs (S1's and F1's
  count); and two leads ride the same state at other doors and are rows of their own, both
  unclear: entries 18 and 19.
- The reporters disagree on the clause and I took the stricter reading: S2 and RC-4 say *none*
  ("an append ... so the target's bytes stand"), F1 says `no-lost-files`.

### 6. `r1-doc-list-served-from-no-checkout-home` - no-break (`none`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-5*; also `R/row-doc-list-source.a1.md`,
  *Repro S4*, and `R/row-doc-list-driver.a1.md`, *Repro 3* (DL-2).
- **Why.** RC-5: "`none` for this door - it reads, and what it reads at the wrong place it only
  reports"; exit 0, tree identical, no refusal and so no route. The round's served-from note is
  silent there by design. What it is beside: whether the three layouts (a worktree of a bare
  repository, a `--separate-git-dir` checkout, a submodule) are supported is, per the source
  pass and the driver, an open question owed to the human at
  `implementation/decisions-pending.md`, (D), trigger M57. The reconciler did not re-read that
  entry, I did not either, and the grade does not rest on it. The door that writes in those
  layouts is entry 7.
- **Not found again.** The reporters call the class "already on record" - in the fix pass's
  ledger and in decisions-pending. Neither is this run's ledger, which held one row; the key is
  new here.

### 7. `r1-setup-installs-outside-work-tree` - unclear (`working-product`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-6*; also `R/row-doc-list-driver.a1.md`,
  *Repro 4* (DL-3), and `R/row-doc-list-source.a1.md`, *Repro L1*.
- **Why.** RC-6: in two of the three layouts "`doc list`'s refusal routes at `jigc setup`,
  which exits 1; that refusal routes at 'ensure the repo's git hooks directory is writable,
  then re-run `jigc setup`', the hooks directory is writable, and the re-run exits 1 again" -
  a route that does not work as printed - and "In scope only if the layout is a supported one
  - the open question on record". The report does not settle it; the layouts' support is not
  mine to rule.
- **The second facet, which the fold must not hide.** In a submodule `jigc setup` exits 0
  having written its install into the superproject's git directory and "rewrites the
  superproject's pre-commit hook (the user's hook body is kept after the managed block)".
  Source L1 put that to the first clause as a question; RC-6 answers that no byte of the hook's
  body is lost. And DL-3 on the bare-worktree cell: the first clause's sentence "where jigc
  cannot tell (git fails ...) it refuses before writing" is touched - git failed and the writes
  had happened - at exit 1, with a `CLAUDE.md` appended to and its lines kept.

### 8. `r1-read-verb-fence-one-form-of-doc-list` - no-break (`none`)

- **Repro:** `R/row-doc-list-source.a1.md`, *Repro S5*; also `R/row-doc-list-reconciler.a1.md`,
  *Repro RC-7*.
- **Why.** S5: "no defect is hidden behind it as far as these drives reach; it is a statement
  about what the fence would catch if one arrived". A coverage statement about one test file,
  whose citations RC-7 verified "as text" and no test run.

### 9. `r1-doc-list-non-utf8-path-drops-orphan-rows` - no-break (`none`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-8*; also `R/row-doc-list-source.a1.md`,
  *Repro L2* (not driven there).
- **Why.** RC-8: "a read that reports less than is there; nothing written" - exit 0, no
  refusal, no route. The same state at the door where it turns a blocking verdict off is
  entry 10, graded unclear; the three consumers of the same enumerator nobody read are entry
  16, graded unclear.
- **Its reach, as the reporter bounds it:** built with git plumbing (one index entry) on macOS;
  an ordinary committed file with such a name, on a file system that admits one, "was not
  driven".

### 10. `r1-validate-non-utf8-path-drops-blocking-orphan` - unclear (`working-product`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-9*.
- **Why.** RC-9: the same state "turns `jigc validate` from exit 1 with a blocking
  `schema-conformance.orphaned-instance` into exit 0 and 'the committed store validates
  clean'", and "the pre-commit hook `jigc setup` installs runs this same sweep". The reporter
  reads *clause: none* - "no bytes lost, nothing committed, the previous release does the
  same" - and returns it "for its tier". A gate that answers *clean* over a store it blocks
  without that file name is an exit-0 answer others rely on; whether that breaks the second
  clause within its scope the report does not settle. In doubt, unclear.

### 11. `r1-served-from-note-nested-worktree-path` - no-break (`none`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-10* (the driver's lead L1 came with
  no block: "row D8 of the report's section D").
- **Why.** RC-10: from a worktree nested below the main checkout the round's new stderr line
  names the worktree `.trees/nested` - "a path the reader cannot resolve from where the line
  says they are standing"; stdout is byte-identical to the main checkout's, exit 0, nothing
  written, and whether the surface contract's repo-relative rule intends the spelling "was not
  established". A note's spelling; no clause.
- RC-10 marks it as "the one finding inside code the round added: the previous release prints
  no such line". That is quoted, not graded on.

### 12. `r1-task-read-before-setup-answers-no-task` - no-break (`none`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-11* (the driver's lead L2 came with
  no block).
- **Why.** RC-11: "the refusal is true as far as it goes and its route runs (exit 0, `jigc
  task list - no active tasks`)". The route works as printed and leads to a true answer; that
  nothing says the project is not set up is a row, and not a clause.

### 13. `r1-doc-list-prints-id-doc-show-refuses` - unclear (`working-product`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-12* (the driver's lead L3 came with
  no block).
- **Why.** RC-12: the listing prints `research:UPPER`; `jigc doc show research:UPPER` exits 1,
  `store.malformed-slug`, and its route is "`jigc doc list` lists the committed docs and the
  identity each one carries" - "It is a loop all the same ... Triage may read the second
  clause's 'every refusal's route works as printed' more strictly than this". The reporter
  leaves it open in so many words, and says the state is ordinary: "any untracked `README.md`
  or `My Notes.md` in a located doctype's folder".

### 14. `r1-doc-list-reads-through-link-out-of-repo` - no-break (`none`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro RC-13* (the driver's lead L4 came with
  no block).
- **Why.** RC-13: "a read; nothing written" - the outside file's heading is printed as a row's
  title, tree identical. Both reporters add that the link is a planted state and that no bound
  is declared. **The grade does not lean on that:** in an ordinary state too, a read that
  follows a link destroys nothing and commits nothing, and no clause is about what a read
  shows. So this is no-break and not needs-bound.

### 15. `r1-doc-list-fsmonitor-cookie-mtime` - no-break (`none`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro LD-1*.
- **Why.** LD-1: "a bound on the verdict's 'mtime included', not a defect" - the one mtime that
  moves is git's own daemon's cookie directory, and "`git ls-files -z` alone does the same".

### 16. `r1-non-utf8-path-other-orphan-walk-consumers` - unclear (`migration-works`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro LD-2* - a block that says NOT DRIVEN and
  names no command ("the verb each call site serves - not identified here").
- **Why.** LD-2: "if a migration's strand walk goes empty over such a repository, the fourth
  clause is the one it would touch; nothing here establishes that. Not read, not driven." It
  may; the report settles nothing.
- The door is the reporter's own words. The third call site is in `relocate.rs`; I did not turn
  that into a verb of the round's list, because the reporter did not identify one.

### 17. `r1-doc-list-cells-driven-on-one-platform` - no-break (`none`)

- **Repro:** `R/row-doc-list-reconciler.a1.md`, *Repro LD-3* (NOT DRIVEN).
- **Why.** LD-3: "every cell of this report, and of the two it reconciles, ran on one macOS
  host and one git". It names no behaviour; it bounds where entries 2, 3 and 9 were driven. It
  is carried to the verifiers of entries 2 and 3 as part of what they are asked.

### 18. `r1-finalize-may-commit-redirected-log-record` - unclear (`no-lost-files`)

- **Repro:** none. `R/cross-cutting-review.a1.md`, the heading *Lead L1*, is prose.
- **Why.** Cross-cutting L1: a door that stages the working tree on the user's behalf "could
  then commit a line nobody wrote - content the user did not ask for, in the first clause's own
  words. Not read, not driven." It came without a repro block, so it is unclear and said to
  lack one. It rides the planted state of entry 5.

### 19. `r1-uninstall-log-append-through-live-link` - unclear (`no-lost-files`)

- **Repro:** none. `R/cross-cutting-review.a1.md`, the heading *Lead L3*, is prose.
- **Why.** Cross-cutting L3: under the append-only arm "a link whose target exists is appended
  through. Read at the source, not driven." It came without a repro block, so it is unclear and
  said to lack one. The reporter names no clause; the one it may touch is the first - a write
  through a link at exit 0 - and it rides the planted state of entry 5.

### 20. `r1-run-session-refuses-scripted-arm-without-credential` - no-break (`none`)

- **Repro:** `R/arm-control-rehearse.a1.md`, the block *R4* (ACR-1).
- **Why.** ACR-1: "an instrument's precondition: where the variable is unset the arm is void
  for a reason that says nothing about jigc"; the variable is set in this stage and the arm
  ran.

### 21. `r1-arm-control-does-not-reach-changed-path` - no-break (`none`)

- **Repro:** `R/arm-control-score.a1.md`, the block *S7* (SCO-1); also
  `R/arm-control-rehearse.a1.md`, *R5* (ACR-2), and `R/arm-control-run.a1.md`, *A7* (RUN-1).
- **Why.** SCO-1: "none broken, as I read it - it bounds what this item's green is evidence of
  under `usable-by-agents`": the arm reads the door once, as `--task`, from the main checkout,
  output and exit status discarded, and both binaries leave the same record there. It names no
  behaviour of the candidate. **It is listed under *For the human's eye*.**

### 22. `r1-walk-arm-runs-in-caller-dir-outside-container` - no-break (`none`)

- **Repro:** `R/arm-control-rehearse.a1.md`, the block *R6* (ACR-3).
- **Why.** ACR-3: outside the container the arm's bare `cd /work` fails "and goes on in the
  caller's directory, where it runs `jigc setup` and mints a task". Trial tooling run outside
  its harness; no jigc door does anything but what it was told. The reporter's count is 24 of
  24 walk arms by grep, one driven.

### 23. `r1-observe-exits-1-on-scripted-arm-evidence` - no-break (`none`)

- **Repro:** `R/arm-control-score.a1.md`, the block *S6* (SCO-4); also
  `R/arm-control-rehearse.a1.md`, *R7* (ACR-4), and `R/arm-control-run.a1.md`, *A8* (RUN-2).
- **Why.** SCO-4: the exit "is the transcript branch's ... and a scripted arm has no
  transcript ... its outcome column ... separates the passing evidence from the failed one".
  Information for whoever reads a score by exit status.

### 24. `r1-config-set-leaves-manifest-untracked` - no-break (`none`)

- **Repro:** `R/arm-control-rehearse.a1.md`, the block *R8* (ACR-5).
- **Why.** ACR-5: "I do not say this is a defect". The run step, which was handed that report,
  read it against the design and wrote: "as designed, and the verb says so" -
  `design/project-setup.md`, "`jigc config set` writes `.jigc/config/manifest.yaml` and tracks
  nothing", and the verb's own ack, "written to `.jigc/config/`, uncommitted - commit it with
  your next commit". The run step returned no finding for it; the rehearsal's lead is the
  entry.

### 25. `r1-provenance-names-model-no-session-used` - no-break (`none`)

- **Repro:** `R/arm-control-run.a1.md`, the block *A9* (RUN-3).
- **Why.** RUN-3: for a scripted arm the two provenance lines "describe nothing that ran"; the
  score reads neither.

### 26. `r1-control-pass-counts-failed-read` - no-break (`none`)

- **Repro:** `R/arm-control-score.a1.md`, the block *S5* (SCO-2).
- **Why.** SCO-2: "none broken - it bounds what the instrument of `usable-by-agents` proves;
  this run's counted record did succeed (exit 0, 328 bytes matched)"; and "the third reading
  is the registered wording, not a slip of the script against it". **It is listed under *For
  the human's eye*.**

### 27. `r1-run-session-exits-0-when-arm-fails` - no-break (`none`)

- **Repro:** `R/arm-control-score.a1.md`, the block *S5* (SCO-3).
- **Why.** SCO-3: "declared in the harness (line 229) and in `driver/session.py`
  (`arm_exit_code`), and both `walk.py` and `run.py observe` read the provenance line".

### 28. `r1-arm-evidence-git-dir-touched-after-run` - no-break (`none`)

- **Repro:** `R/arm-control-score.a1.md`, the block *S1* (SCO-5).
- **Why.** SCO-5: "nothing the score reads moved" - the 35 files outside `.git` are hash-equal
  to the run step's manifests. What touched the directory "I did not establish".

### 29. `r1-scope-registry-is-the-scope-steps-choice` - no-break (`none`)

- **Repro:** no block of its own; `R/scope.a1.md`, the section *Repro*, has the commands for
  the registry's count and the state's doors.
- **Why.** Left open 1: "The registry is this step's choice ... Later rounds must reuse the
  door wording ... exactly, or the script reads them as different doors". A note for later
  rounds; it claims no behaviour of the candidate, so there is no clause it could break.

### 30. `r1-scope-other-registries-unread-two-changed` - unclear (`working-product`)

- **Repro:** none. `R/scope.a1.md`, *Left open*, item 2, gives two line counts (144 then 155;
  6 then 12) and no command for the comparison.
- **Why.** Left open 2: "Two changed: `FINALIZE_FAMILY` ... and `ENVELOPE_OWED_CODES` ... the
  change adds rows to two registries of finding codes that this round, as set, does not look
  at. Their rows were not enumerated". It came without a repro block, so it is unclear and said
  to lack one. The reporter names no clause; rows of finding codes nobody looks at would touch
  the second, if any.

### 31. `r1-scope-47-doors-reached-and-excluded` - no-break (`none`)

- **Repro:** `R/scope.a1.md`, the section *Repro* (the hunks of `main.rs`, the one
  `cli.dispatch()` call, the state's doors).
- **Why.** Left open 3: "under the run's default scope all 48 doors would have been inside. The
  set scope tests 1 of 48". The round's scope holds each of the 47 with that derivation. It
  claims no behaviour. **It is listed under *For the human's eye*.**

### 32. `r1-scope-control-arm-drives-excluded-doors` - no-break (`none`)

- **Repro:** no block of its own. What the arm drove is in three other reports' blocks: the ten
  records of `R/arm-control-rehearse.a1.md` *R2*, `R/arm-control-run.a1.md` *A3* and
  `R/arm-control-score.a1.md` *S2*.
- **Why.** Left open 4: "anything it shows at one of those is at an excluded door and is the
  human's". The arm's three reports show ten invocations, ten at exit 0, and the run's and the
  score's each close on "No finding against `jigc` itself came out of" it. A routing note with
  nothing to route.

### 33. `r1-scope-never-selected-lists-row-before-test` - no-break (`none`)

- **Repro:** `R/scope.a1.md`, the section *Repro* (`dev/stabilize-record state --run
  canary-one`).
- **Why.** Left open 5: "that list counts tested rounds only and round 1 is not tested yet".
  The state I read says the same today: `never_selected` is `["row-doc-list"]`, the item's
  `selected` is true, `uncovered` is empty.

### 34. `r1-scope-change-sits-on-committed-arm-only` - no-break (`none`)

- **Repro:** `R/scope.a1.md`, the section *Repro* (the one hunk whose context is `fn
  run_list(`).
- **Why.** Left open 6: the one line the change added to the handler "is the last statement of
  the committed-store arm; the staged arm (`--task`) and every refusal above it return before
  reaching it". A fact about where the change lies; it claims no broken behaviour.

## To verify - ten keys

Each is graded *breaks* or *unclear*. What the verifier must re-drive:

| key | what to re-drive |
|---|---|
| `canary-seeded-claim` | The opening record's block as written (git init, one empty commit, `jigc setup`, an untracked one-line `notes.md`, `jigc doc list`): the verb's exit status, and whether `notes.md` stands with the same bytes. |
| `r1-doc-list-unreadable-entry-fails-listing` | *Repro RC-1* as written (a dangling link at `docs/decisions/` in a set-up repository) and one of its variants: exit, stdout, stderr, whether any route is printed. Then the question: does a refusal that prints no route break "every refusal's route works as printed" within the clause's scope, and what does `design/doc-read-surface.md` say an instance that cannot be read does to the listing. Say which platform was driven: the reports drove macOS only (LD-3). |
| `r1-doc-list-staged-arm-unreadable-entry` | *Repro RC-2* as written (rig `refs-post-hoc`; a dangling link, then a mode-000 file, in the task's staging area): the `--task` listing's exit and stderr, and the committed listing's stderr with the dangling link there. Then: is that workbench state one that ordinary use reaches, and does the route-less exit 1 break the second clause within its scope. Platform as above. |
| `r1-setup-installs-outside-work-tree` | *Repro RC-6* as written (`jigc setup` typed in a worktree of a bare repository: exit 1 after writing, its printed route re-run exits 1 again) and its submodule variant (exit 0, the superproject's `pre-commit` hook rewritten). Whether the clause is broken within its scope turns on whether these layouts are supported: say what `implementation/decisions-pending.md`, (D), holds. |
| `r1-validate-non-utf8-path-drops-blocking-orphan` | *Repro RC-9* as written: the control (`jigc validate` exits 1 with `schema-conformance.orphaned-instance`), then the same store with one index entry whose name holds the byte 0xFF. Then: does an exit-0 "validates clean" over that store break the second clause within its scope. |
| `r1-doc-list-prints-id-doc-show-refuses` | *Repro RC-12* as written: the listing's rows for `docs/research/UPPER.md` and `has space.md`, then `jigc doc show research:UPPER`, then its route as printed. Does a route that leads back to the listing that printed the refused identity work "as printed". |
| `r1-non-utf8-path-other-orphan-walk-consumers` | Not driven by its reporter, and its block names no command. Identify the verbs behind `config.rs:1512`, `orphan.rs:1116` and `relocate.rs:708`, build *Repro RC-8*'s state and drive each; say whether a strand walk goes empty and what a migration or a relocation then does. |
| `r1-finalize-may-commit-redirected-log-record` | No repro block. Build cross-cutting *Repro F1*'s variant (knob on and committed, the log's path a link to a tracked file), run `jigc doc list` so the record lands in the tracked file, take a task to `jigc task finalize` and read the commit: is the appended line in it. The dangling-link shape too. Leave the block. |
| `r1-uninstall-log-append-through-live-link` | No repro block; read at source only (`invocation_log.rs:656-659`, the append-only arm). Knob on, the log's path a link to an existing file outside the repository, then `jigc uninstall`: its exit and the target's bytes. Leave the block. |
| `r1-scope-other-registries-unread-two-changed` | No repro block for the comparison. The bodies of `FINALIZE_FAMILY` and `ENVELOPE_OWED_CODES` (`crates/cli/src/render.rs`) at the previous release's commit and at the candidate's: the rows the tip adds, and whether any, as it stands, breaks the second clause within its scope. The scope step claims no behaviour. |

## Findings that came without a repro block

| finding | what its report has | where that left it |
|---|---|---|
| driver L1, L2, L3, L4 | "repro: none - a lead", a row of a table each | each is folded into the reconciler's finding that drove it; the entries' blocks are *Repro RC-10* to *RC-13* |
| cross-cutting L2 | prose; "Read at source, not driven" | folded into entry 5, whose blocks drive that shape (*Repro RC-4*, third variant) |
| cross-cutting L1 | prose; "Not read, not driven" | entry 18, **unclear**, said to lack one |
| cross-cutting L3 | prose; "Read at the source, not driven" | entry 19, **unclear**, said to lack one |
| scope, left open 2 | two line counts, no command | entry 30, **unclear**, said to lack one |
| scope, left open 1, 3, 5, 6 | no block of their own; the scope report's *Repro* section holds the commands the facts rest on | entries 29, 31, 33, 34, no-break |
| scope, left open 4 | no block of its own; the fact is in three other reports' blocks | entry 32, no-break |
| reconciler LD-2, LD-3; source L2 | a block that says NOT DRIVEN | LD-2 is entry 16, unclear; LD-3 is entry 17; source L2 is folded into entry 9, driven by RC-8 |

**Two places where I did not follow my definition to the letter, each said here so that nobody
has to find it.**

1. **A missing block is no entry of its own.** My definition has a missing block be "itself an
   entry against that reporter's report". The harness holds a triage's return to as many
   entries as findings handed in, less the merged, and the run's own record already owes the
   edit: `implementation/decisions-pending.md`, (I) `wf-r-m5` - "a missing block is said in the
   entry's `why` and in triage's report, and is no entry". I followed that. The table above is
   the saying of it, and the sentence in the definition still stands unedited.
2. **Five of the scope step's six notes are graded no-break without a block of their own**
   (entries 29, 31, 32, 33, 34), where the definition's sentence reads "a finding that came
   without a repro block is graded `unclear`". Each of the five states a fact about the round's
   scope or the run's record and claims no behaviour of the candidate: there is no clause it
   may break, so *unclear* - "it may" - would be untrue of it, and a verifier halts on a
   finding whose clause is not one of the closing condition's. The scope report's own words for
   the list are "Nothing here is a finding; each is something a reader of this scope should
   know". The sixth (entry 30) names a part of the change nobody looks at, with nothing to
   reproduce it by, and is unclear. If the letter is what is wanted, the five are regraded in
   one more pass; nothing is lost meanwhile, each is a row.

## For the human's eye - rows a grade must not hide

*no-break* is recorded and is on nobody's list. These rows break no clause as reported and
still say something about what this round's greens are evidence of:

- **`r1-arm-control-does-not-reach-changed-path`** (entry 21) - three reporters, independently:
  a green `arm-control` "is not evidence, either way, about the behaviour the round changed"
  at `jigc doc list`.
- **`r1-control-pass-counts-failed-read`** (entry 26) - the positive control passes with its
  one channel read failing (driven once, as a mutant); in this run the counted read succeeded.
- **`r1-scope-47-doors-reached-and-excluded`** (entry 31) and
  **`r1-scope-other-registries-unread-two-changed`** (entry 30, unclear) - the change reaches
  all 48 leaf verbs and adds rows to two finding-code registries; the round, as its scope was
  set, tests one door.
- **`r1-doc-list-served-from-no-checkout-home`** (entry 6) - no-break at this door; the
  layouts' support is the open question of decisions-pending (D), and the write door in them is
  unclear (entry 7).
- **`r1-invocation-log-write-follows-link`** (entry 5) - the one *needs-bound*: it is the
  human's as it stands.

## In the reports, returned by nobody as a finding

Not entries - the arithmetic counts what was returned - and named so that reading every report
whole leaves nothing unseen:

- **preflight-second, section 1.5:** `completions/trial-harness/build-image.sh` falls back to a
  source repository fixed in the script when `JIGC_REPO` is unset; that checkout does not hold
  the candidate, so the image was built with the variable set. "A fact for the harness's owner
  to read; it is not a finding about the candidate."
- **reconciler, section 3, the aside to its group B:** with `CLAUDE.md` linked to a file
  outside the repository, `jigc setup` refuses on the candidate (`setup.inject-reference`, exit
  1, tree identical) and "the previous release does not refuse: exit 0, an install commit, and
  the outside file grows from 17 to 56 bytes". The two binaries differ there and the
  candidate's is the safer answer; the driver's "Previous release: same" does not hold for
  that aside.
- **reconciler, section 6:** the source pass's sentence "the door's two other hard reads ...
  are guarded" does not reproduce for `doc.rs:5040`. It is the substance of entry 3.
- **reconciler, section 9:** the driver's count of 86 invocations has two operands off (122
  labels, 35 of the previous release); the total stands and nothing rests on it.
- **rehearse, block R4:** run as the tool stands, the harness writes the credential variable's
  value into the container's configuration for the container's life; the three arm steps
  passed a placeholder instead.
- **cross-cutting, the driven block:** with the knob on, a plain `git commit` leaves one record
  in the log - the installed `pre-commit` hook's own `jigc validate`. "Not a finding."

## What I did not do

- I drove no binary and built no rig; every sentence about behaviour above is a reporter's.
- I did not re-read `implementation/decisions-pending.md`, (D), which entries 6 and 7 stand
  beside, nor the fix pass's ledger rows the reporters cite; I read (I) of that file, for the
  arithmetic.
- I called none of `ledger-add`, `ledger-set` or `triage-set`: the stage's record step writes
  the rows from my return.
- The structured return's cells are plain ASCII (a hyphen where this report might have a
  dash), because the record step writes them through a file whose bytes are hashed.

<!-- end of report -->
