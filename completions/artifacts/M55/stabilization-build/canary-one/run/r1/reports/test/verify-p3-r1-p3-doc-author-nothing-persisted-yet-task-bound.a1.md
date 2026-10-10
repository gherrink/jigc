# verify-real — `r1-p3-doc-author-nothing-persisted-yet-task-bound` (run canary-one, round 1, attempt 1)

- **key:** `r1-p3-doc-author-nothing-persisted-yet-task-bound`
- **door:** `jigc doc author`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `refuted` — as a blocker. The facts the finding states reproduce exactly; they break no clause.
- **basis:** `breaks-no-clause` — a refused `jigc doc author` does leave two task-area records at exit 1 (the role binding and a provenance row) under a route that opens *nothing was persisted*, on both binaries alike; the surviving binding is the settled design of DECISIONS.md → *2026-08-15 — M48 complete*, pinned in both releases, and the refusal's one printed command exits 0 and removes it — so the sentence is wider than the fact by two records, and no route fails on that account.
- **regression:** not stated (it goes with `confirmed` only). The fact triage asked for is in *Both binaries*, below: the two agree in every exit and every printed byte at this door.
- **contested:** `false` — the finding argues no decision wrong; it asks what the design says, and the design has said it.
- **class:** `instance, unbounded`. I enumerated no consumer set. What was driven is one doctype (`research`), one workflow (`migrate-research`), three refusal codes and three layouts, listed under *What I drove*.

`<W>` is my own scratch directory, `<scratch>/verify-p3-nothing-persisted.SEuPoQ`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` is the repository of whichever rig a row names; `<task>` is the id on the `task minted: ` line of that rig's `jigc migrate`.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path, exit 0, in every shell that drove anything; each arm compares the two strings and exits 99 on a difference, and none did |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: `jigc 1.0.0-rc.24`. No build ran; nothing under `target/` was driven. Every rig is `dev/jigc-rig fresh --binary <that binary's path>` with `SCRATCH=<W>`, its stdout captured alone and evaluated in a second step.

## What I drove

Fourteen fresh rigs: one built only to read the rig's assignments, thirteen driven. Every exit status was read bare, stdout and stderr captured to separate files under `<W>`. Before and after each refused write the arm hashed every file under `<repo>/.jigc` (`find .jigc -type f`, sorted, `shasum -a 256`), listed its directories, copied the task areas aside, and recorded `git status --short --untracked-files=all` and `HEAD`.

The conforming payload is the reporter's (Repro V-1's `stdin`), its `title:` set as the row names. The non-conforming payload is the same grammar with one undeclared section, `- id: bogus` / `set: {bogus: <<x>>}`.

| rig | binary | layout committed at `docs/research/` | what was driven |
|---|---|---|---|
| C1, C2 | candidate | `UPPER.md` = `# Upper` | V-1 steps 1 to 3, the working area before and after (C1 hashes only; C2 with the contents) |
| P2 | previous | the same | as C2 |
| C3, P3 | candidate, previous | the same | step 1, step 2, then the refusal's own printed command, then the task started over |
| CA, PA | candidate, previous | `has space.md` = `# Spaced` | the non-conforming payload under `Has space`, then the conforming one under **another** title, to adoption |
| CA2 | candidate | the same | the non-conforming payload, then the same write repaired, to adoption |
| CB, PB | candidate, previous | `has space.md` and `upper.md` = `# Upper` (lower case, exactly) | `has space.md` migrated and authored under `Upper`, then under `Has space`, then that refusal's printed route |
| C4, P4 | candidate, previous | the same | the same refusal, then its own printed command, then the task started over, to adoption |
| X1 | candidate | `has space.md` | exploration only: which payload defects a batch refuses at a leaf |

A probe in `<W>` (a file made as `CaseProbe` answers to `caseprobe`) says this volume folds case. A volume that does not was **not** driven.

## Step 1 — the block, re-driven: V-1 steps 1 to 3

Rig C2 (candidate) and rig P2 (previous release), fresh, the same exits at each step.

| # | argv | exit | what it printed |
|---|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/UPPER.md --as research` | 0 | `task minted: <task>` |
| - | `jigc doc list research --task <task>` | 0 | *no `research` docs staged in task `<task>`* |
| 2 | `jigc doc author research --from-file - --task <task>`, `title: "Upper"` | **1** | stderr: `blocking · write.non-reparseable — write rejected: the source does not conform to the schema (required section heading `## question` is missing)` · `at: research:upper#question` · `route: nothing was persisted — revise the payload so the result still conforms (the message names the break), then re-run the same write; if the staged source itself is what no longer parses, `jigc task discard <task-id> --force` and start the task over` |
| - | `jigc doc list research --task <task>` | 0 | *no `research` docs staged in task `<task>`* — as before |
| 3 | the same argv, `title: "Upper notes"` | **1** | stderr: `blocking · write.identity-change — author rejected: this task's `research` is already `research:upper`, and this call would mint `research:upper-notes` instead` · route: `jigc doc rename research:upper --to 'Upper notes' --task <task>` |

### The working area, before and after the refused step 2

`<repo>/.jigc/tasks/<task>/` — the whole of what changed under `.jigc`, on the candidate (rig C2; rig C1 gave the same hashes):

| file | before step 2 | after step 2 (exit 1) |
|---|---|---|
| `roles.json` | absent | `{"roles": {"research": "research:upper"}}` |
| `docs/provenance.json` | `{"docs": {"commit:<task>": "created"}}` | the same, plus `"research:upper": "edited-from-base"` under `docs`, plus a `copied-in` member holding `research:upper`'s two hashes (`bytes`, `git`) |
| `docs/` listing | `commit:<task>.md`, `provenance.json` | the same two — **no `research:upper.md`** |
| `.jigc/state/file-state.json.lock` | absent | present, 0 bytes |
| every other file under `.jigc` | - | hash unchanged |
| `git status --short`, `HEAD` | empty, the plant's commit | unchanged |

Step 3 (the second refusal) changed nothing: every hash under `.jigc` is the same before and after it.

On the previous release (rig P2): `roles.json` appears with the same bytes (`cmp` exit 0 against the candidate's); `docs/provenance.json` gains the same `"research:upper": "edited-from-base"` row and no `copied-in` member; no lock file appears.

**So the finding's two facts are as stated.** The route opens *nothing was persisted*; two records of the task area were written by the call that printed it; and the task is bound afterwards — the next author under another title is refused on the strength of that binding.

## Step 2 — is it what the finding says, and what does the design say of it?

I tried the ways a result like this is usually wrong. The rigs and the scratch directory are mine and new, and the cell was driven on three fresh candidate rigs (C1, C2, C3) with the same result. No exit status was read through a pipe. The decisive outputs are whole files. The binary is the candidate by hash and by `command -v`. None of that removed it.

What did answer it is the record. The question the finding leaves — *whether that binding is what the sentence disclaims is the design's to say* — has been asked before, and settled:

- **DECISIONS.md → *2026-08-15 — M48 complete: the audit's four findings all confirmed and fixed*** names this exact mechanism as the wave's HIGH finding: *a `doc author` failing mid-payload rolled back the staged file but not the role binding*. It was fixed **by a state-derived probe** of whether the bound document is there, not by unbinding on rollback; and the entry adds that *a binding naming a **committed** doc is a genuine incumbent whose refusal is honest*. The fix's commit, `d9af1b8d`, gives the reason the binding stays: the probe *holds however the orphan arose … Unbinding would close one producer and leave the guard's premise unchecked.*
- The code says the same where the binding is read: `crates/engine/src/state.rs`, `bound_instance_present` — *a leaf failure rolls the staged `.md` back but not the binding (the rollback is `CreatedDoc::rollback`'s, which owns the file and not the record)*; a binding is live when its document is in either of two homes, the task's staged copy or the committed home a create would copy in, and stale otherwise.
- Two suite arms pin it, and both are in the previous release's tree (`git grep` at `91834b5e`): `write_title_divergence::a_binding_whose_doc_is_gone_is_not_an_incumbent_at_any_minting_verb` asserts that the failed author **leaves the role bound**, that the bound doc is not staged, and that a retry under another title lands and re-points the role; `write_title_divergence::a_binding_whose_doc_is_committed_is_still_an_incumbent` asserts that over a committed doc the binding stays, the divergent mint is refused `write.identity-change`, and the printed `jigc doc rename` answers with a route at `jigc rename`. I read both bodies, not their names.
- The per-axis review of M52 met the state itself — *a rolled-back `jigc doc author` discards the created instance's bytes but keeps its provenance entry and its role binding* — and filed it as an observation, not a defect (completions/artifacts/M52/per-axis-review/axis-4.md, §7), reading design/write-commands.md's *rejected whole (atomic), never half-applied* as a promise about the payload's leaves. `crates/cli/src/rollback.rs` → `ROLLBACK_POPULATIONS`, row `created-doc-staged-write`, states its subject the same way: *a staged instance … restored to what the create found*.

So the **binding** is intended, and I cite it as such. What no decision covers is the **sentence**: `crates/engine/src/write.rs`, `write_route`, prints *nothing was persisted* for `write.non-reparseable` at every write verb, and at this one door — the batch's create arm, where `create_gated` binds the role and records provenance before the first leaf is applied — it is wider than what happened by those two records. That residue is real. It is the only part of the finding that is a defect at all.

### Triage's question — does the binding come of the refusal or of the case fold?

Neither alone. Two separate things were conflated in the question, and the arms separate them.

**The record comes of the refusal — of any refusal at a leaf, on any volume.** Rig CA (candidate) and PA (previous), `has space.md` only, nothing that depends on case:

| # | argv | exit | after |
|---|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/has space.md --as research` | 0 | `roles.json` absent |
| 2 | `jigc doc author …`, `title: "Has space"`, the undeclared section | 1 — `write.unknown-section`, routed at `jigc doc schema research` | `roles.json` = `{"roles": {"research": "research:has-space"}}`; `provenance.json` gains `"research:has-space": "created"`; no `research:has-space.md` staged |
| 3 | `jigc doc author …`, `title: "Spaced notes"`, conforming | **0** — `research:spaced-notes` | `roles.json` re-pointed at `research:spaced-notes`; the row `"research:has-space": "created"` still in `provenance.json` |
| 4, 5 | `jigc task finalize <task>`, then `--approve` | 4, 0 | *deleted `docs/research/has space.md`, promoted `docs/research/spaced-notes.md`, 2 files committed* |
| 6, 7 | `jigc doc list`, `jigc validate` | 0, 0 | `research:spaced-notes … managed`; *no findings* |

Rig CA2: step 3 as the same write repaired (`title: "Has space"`, conforming) — exit 0, `research:has-space`, adopted, `jigc validate` exit 0. The candidate's and the previous release's `roles.json` and `provenance.json` are byte-identical at each of the three snapshots (`cmp` exit 0).

**Whether the record binds comes of the home, not of the case.** A binding is honoured when a body sits at the bound identity's canonical home. The case fold is one way to put a body there — `docs/research/upper.md` *is* `UPPER.md` on this volume. An exact-case file is another. Rig CB (candidate) and PB (previous), `has space.md` beside a lower-case `upper.md`:

| # | argv | exit | what happened |
|---|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/has space.md --as research` | 0 | task minted |
| 2 | `jigc doc author …`, `title: "Upper"` | 1 | `write.non-reparseable`, the same message and the same route as V-1 step 2, byte for byte; `roles.json` = `research → research:upper`; `provenance.json` gains `research:upper: edited-from-base`; nothing staged |
| 3 | `jigc doc author …`, `title: "Has space"` | 1 | `write.identity-change` — *this task's `research` is already `research:upper`*, routed at `jigc doc rename research:upper --to 'Has space' --task <task>` |
| 4 | that route, lifted from the bytes and run through `sh -c` | 1 | `write.identity-change` — *rename rejected: `research:upper` is committed*, routed at `jigc rename research:upper --to 'Has space'` once the task is out of flight |

So the bound state of V-1 step 3 needs no case fold. It needs a body at the home of the title that was authored — and that same body is what makes the write `write.non-reparseable` in the first place, because the create copies it in. In the batch's create arm the two arrive together: I found no way to reach this refusal code there without also reaching a binding that holds.

## Step 3 — does it break the clause, inside its scope?

The clause is DECISIONS.md → *2026-10-04 — The exit rule, revised*, second clause, by its instrument: *no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed*.

- **First half: not broken.** Nothing that works on the previous release stopped working at this door. See *Both binaries*.
- **Second half, the route this refusal prints.** It spells one command, `jigc task discard <task-id> --force`, with one declared placeholder. Lifted from the emitted bytes, the placeholder filled with the task's id, run through `sh -c`:

  | rig | layout | the printed command | then |
  |---|---|---|---|
  | C3, P3 | `UPPER.md` (the finding's cell) | exit **0** — *discarded task `<task>` — dropped staged edits to: `commit:<task>` (transient)*; `.jigc/tasks` empty, so `roles.json` and the provenance row are gone | `jigc migrate` again 0 (`roles.json` absent, `provenance.json` back to its one row); `jigc doc author` under `Upper notes` as the task's first write **0** — `research:upper-notes`; finalize 4; approve 0 — *deleted `docs/research/UPPER.md`, promoted `docs/research/upper-notes.md`*; `jigc validate` 0, *no findings* |
  | C4, P4 | `has space.md` beside `upper.md` | exit **0**, the same line; `.jigc/tasks` empty | `jigc migrate` again 0; `jigc doc author` under `Has space` **0**; finalize 4; approve 0; `jigc doc list`: `research:has-space … managed`, `research:upper … unregistered`; `upper.md` still `# Upper` |

  The command works as printed, on both binaries, and it removes exactly what the sentence left unsaid. The route's other arm — *revise the payload … then re-run the same write* — is prose, conditioned by the route itself on the payload being the broken thing; where it is (rigs CA, CA2) the re-run lands, under the same title and under another.
- **What the over-wide sentence costs, exactly.** In the two layouts where the binding holds, a reader who takes the first arm and revises the **title** is refused (`write.identity-change`), and that refusal's route exits 1 as printed. That chain is real — and it is the row `r1-adoption-route-for-non-doc-id-name-not-run`, already on the ledger with its own verdict: the block handed to me *is* steps 2 and 3 of that row's block. The persisted binding adds no failing route of its own beyond that row's, and where no body sits at the home it costs nothing at all.
- **Scope.** Nothing here turns on scope: no clause is broken, so no bound is needed to take the finding out of one.

A defect that is real and breaks no clause: `refuted`, basis `breaks-no-clause`. It stays a row — a route's premise that is untrue of two files is a matter for design/surface-contract.md's law 1 (*nothing lies*), and whoever weighs it should weigh it there.

## Both binaries

Asked for by triage; not a regression field, since the verdict is not `confirmed`.

| arm | candidate, previous | exits | `cmp` of the captured stdout and stderr |
|---|---|---|---|
| V-1 steps 1 to 3, with the two listings | C2, P2 | 0, 0, 1, 0, 1 on both | exit 0 on every pair compared (the two listings, the two refusals) |
| the printed command, then the task over | C3, P3 | 0, 1, 0, 0, 0, 4, 0, 0, 0 on both | exit 0 on every pair compared (the refusal, the discard, the author, the listing, `validate`) |
| the undeclared section, then another title | CA, PA | 0, 1, 0, 0, 4, 0, 0, 0 on both | exit 0 on every pair compared |
| `upper.md` at the home, the second author, its route | CB, PB | 0, 1, 0, 1, 1, 0 on both | exit 0 on every pair compared |
| `upper.md` at the home, the printed command, the task over | C4, P4 | 0, 1, 0, 0, 0, 4, 0, 0 on both | exit 0 on every pair compared |

Two differences, both in what the refused call leaves on disk and neither in an exit or a printed byte: the candidate's `provenance.json` carries a `copied-in` member the previous release's does not, and the candidate leaves the empty `.jigc/state/file-state.json.lock`. The approve step's stdout was not compared: it prints the rig's own path.

## Step 5 — coverage

The finding makes no coverage claim. For the block's `pinned-by`, from the suites and not from any diff: 6 suite files under `crates/cli/tests` and `tooling-tests` name `non-reparseable`; 2 of them spell the argv word `"author"`, and in both the code appears once, in a doc comment — neither asserts it on an author call. 0 suite files hold the string `nothing was persisted`. A suite that composes either string some other way is outside these counts.

## Repro N-1 — the refusal, its records and its route, with no case fold

```yaml
claim: "`jigc doc author` refused `write.non-reparseable` prints *nothing was persisted*, leaves the task's role bound and a provenance row behind, and that breaks the refusal's route"
verdict: REFUTED
basis: breaks-no-clause        # the first three facts hold; the route works as printed
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exits, the same printed bytes
setup:
  - fixture: fresh
  - "write docs/research/has space.md = `# Spaced\n`"
  - "write docs/research/upper.md = `# Upper\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: research notes"]
repro:
  - ["jigc", "migrate", "<repo>/docs/research/has space.md", "--as", "research"]       # <task> = the id on stdout's `task minted: ` line
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin: the payload below, title "Upper"
  - ["jigc", "task", "discard", "<task>", "--force"]                                   # step 2's route, its one placeholder filled
  - ["jigc", "migrate", "<repo>/docs/research/has space.md", "--as", "research"]
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin: the same, title "Has space"
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "doc", "list"]
expect:
  - exit: 0
    stdout_contains: "task minted: "
  - exit: 1
    stderr_contains: "blocking · write.non-reparseable — write rejected: the source does not conform to the schema (required section heading `## question` is missing)"
    stderr_contains_2: "route: nothing was persisted — "
    files: ".jigc/tasks/<task>/roles.json binds `research` to `research:upper`; .jigc/tasks/<task>/docs/provenance.json holds `research:upper` as `edited-from-base`; .jigc/tasks/<task>/docs/research:upper.md does not exist"
  - exit: 0
    stdout: "discarded task <task> — dropped staged edits to: commit:<task> (transient)\n"
    files: ".jigc/tasks holds no entry"
  - exit: 0
  - exit: 0
    stdout: "research:has-space\n"
  - exit: 4
  - exit: 0
    stdout_contains: "  deleted docs/research/has space.md\n  promoted docs/research/has-space.md\n"
  - exit: 0
    stdout: "id  path  state\nresearch:has-space  docs/research/has-space.md  managed\nresearch:upper  docs/research/upper.md  unregistered\n"
  tree: "`git status --short` empty after the approve; docs/research/upper.md = `# Upper\n`"
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
where: "<W>/jigc-rig-fresh-LEnh9s (candidate, rig C4); <W>/jigc-rig-fresh-b5PKn4 (previous, rig P4)"
pinned-by: "UNPINNED: write_title_divergence::a_binding_whose_doc_is_committed_is_still_an_incumbent pins the surviving binding over a committed doc, but through an undeclared leaf over a conforming adr — no suite found asserts `write.non-reparseable` on an author call, the two records beside the route's sentence, or the route's command run after it"
```

**Pinnable as it stands: yes.** Nothing in it depends on the volume; `<repo>` and `<task>` are read back from the run. One caution for whoever converts it: the second expectation's `files:` line pins the design as it is — the binding kept. A fix that narrows the sentence leaves it green; a fix that unbinds on rollback turns it red on purpose, and would have to answer the 2026-08-15 entry first.

## Repro N-2 — the cell as handed (V-1 steps 1 to 3), with the listing and the route

```yaml
claim: "the same, in the reporter's cell: docs/research/UPPER.md authored under the title `Upper` on a volume that folds case"
verdict: REFUTED
basis: breaks-no-clause
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc; the same exits and printed bytes on 1.0.0-rc.24
requires: "a volume on which `UPPER.md` and `upper.md` are one file"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: a research note"]
repro:
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin as N-1, title "Upper"
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # the same, title "Upper notes"       (rigs C2, P2 stop here)
  - ["jigc", "task", "discard", "<task>", "--force"]                                   # step 2's route                        (rigs C3, P3: straight after step 2)
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # title "Upper notes", the task's first write
observed:
  - exit: 0
  - exit: 1
    stderr_contains: "blocking · write.non-reparseable"
    files: "roles.json binds `research` to `research:upper`; provenance.json holds `research:upper` as `edited-from-base`; no staged research:upper.md"
  - exit: 1
    stderr_contains: "blocking · write.identity-change — author rejected: this task's `research` is already `research:upper`"
    files: "no hash under .jigc differs from the one before the call"
  - exit: 0
    files: ".jigc/tasks holds no entry"
  - exit: 0
  - exit: 0
    stdout: "research:upper-notes\n"
where: "<W>/jigc-rig-fresh-J9JVNr (candidate, rig C2); <W>/jigc-rig-fresh-y5YBvY (previous, rig P2); <W>/jigc-rig-fresh-P0YJnT (candidate, rig C3); <W>/jigc-rig-fresh-q9vc9b (previous, rig P3)"
pinned-by: "UNPINNED: needs a case-folding volume; N-1 is the form to pin"
```

**Pinnable as it stands: no** — its setup reaches the cell only where the volume folds case. As written it is two runs spliced: no single rig ran step 3 and then step 4.

## Repro N-3 — the record without the hold

```yaml
claim: "a refused `jigc doc author` leaves the role bound, and that binding refuses the corrected re-author"
verdict: REFUTED
basis: does-not-reproduce      # for this claim alone: the record is left; it refuses nothing where no body sits at the bound home
setup:
  - fixture: fresh
  - "write docs/research/has space.md = `# Spaced\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: research notes"]
repro:
  - ["jigc", "migrate", "<repo>/docs/research/has space.md", "--as", "research"]
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin: title "Has space", one section `- id: bogus` / `set: {bogus: <<x>>}`
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin as N-1, title "Spaced notes"
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "validate"]
expect:
  - exit: 0
  - exit: 1
    stderr_contains: "blocking · write.unknown-section — write rejected: no section \"bogus\" declared in the schema"
    files: "roles.json binds `research` to `research:has-space`; no staged research:has-space.md"
  - exit: 0
    stdout: "research:spaced-notes\n"
    files: "roles.json binds `research` to `research:spaced-notes`"
  - exit: 4
  - exit: 0
    stdout_contains: "  deleted docs/research/has space.md\n  promoted docs/research/spaced-notes.md\n"
  - exit: 0
    stdout_contains: "no findings"
where: "<W>/jigc-rig-fresh-nRgzom (candidate, rig CA); <W>/jigc-rig-fresh-qRTRAB (previous, rig PA)"
pinned-by: write_title_divergence::a_binding_whose_doc_is_gone_is_not_an_incumbent_at_any_minting_verb   # read: it asserts the binding left, the doc not staged, the retry landing and the role re-pointed, over the role-bound doctype axis; it does not carry the retry to finalize, and whether `research` is on its axis I did not derive
```

**Pinnable as it stands: yes** — and, to the extent the comment says, pinned already.

## What I did not drive

- A volume that does not fold case, on either binary.
- `jigc rename research:upper --to 'Has space'` after rig CB's step 4 — the route that step prints.
- Any doctype but `research`, any workflow but `migrate-research`, and `jigc doc create`.
- `write.wrong-shape` on a clean task. In rig X1 it was refused on a task an earlier refusal had already bound, so that rig says nothing about whether a payload that fails the shape parse binds; design/write-commands.md ranks the shape parse ahead of the create.
- `write.target-escape`, the other refusal whose route opens with the same sentence.

## Left open — hit on the way, not pursued

1. **The chain of the adoption row's steps 6 and 7 needs no case fold.** Rigs CB and PB: with a non-conforming `docs/research/upper.md` committed (lower case, exactly) and another file migrated under the title `Upper`, the next `jigc doc author` under the right title is refused `write.identity-change`, and its printed route `jigc doc rename research:upper --to 'Has space' --task <task>` exits 1, on both binaries. The suite arm that pins this refusal as intended drives it over a committed **managed** doc; here the body at the home is a foreign file `jigc doc list` shows as `unregistered`.
2. **In that cell neither arm of `write.non-reparseable`'s route names the thing that is broken.** The message says `## question` is missing; the payload supplies it; nothing is staged. What lacks the heading is the foreign body at the home the title mints, which the route calls neither *the payload* nor *the staged source*. The discard it prints runs, and the task started over under the same title meets the same refusal.
3. **Candidate only: a refused `jigc doc author` that copied a committed body in leaves an empty `.jigc/state/file-state.json.lock`**, and it outlives `jigc task discard --force` (rig C3: 7 files under `.jigc` after the discard, 6 before the first `jigc migrate` in rig C1). The previous release leaves none. Whether a lock file that stays is by design I did not read.
4. **A provenance row for a doc that was rolled back stays through a later successful author and its finalize** (rigs CA, PA: `"research:has-space": "created"` beside `research:spaced-notes`). The adoption landed and `jigc validate` is clean; whether the by-task-id join reads such a row I did not examine. M52's axis-4 §7 records the row itself.

## How this report's drives were written

The arms are shell scripts in `<W>`. Two of them (`lib.sh`, `arm-v1.sh`) I wrote with the file tool before re-reading the rule that under a `REPORT:` line the file tool writes the report and nothing else; the rest were written from the shell. All of them are under my own scratch directory, none is in the repository, and none reaches the run's directory. Said here so that the count of files that tool wrote is not taken for one.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else, before and after. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my rigs under `<W>`: the install commit `jigc setup` makes in each, one plant commit of mine in each of the thirteen driven, and the commits the driven `jigc task finalize --approve` calls made.

<!-- end of report -->
