# verify-real — `r1-adoption-route-for-non-doc-id-name-not-run` (run canary-one, round 1, attempt 1)

- **key:** `r1-adoption-route-for-non-doc-id-name-not-run`
- **door:** `jigc validate` (the door that prints the adoption route); the refusals that fail are two doors further on, at `jigc doc author` and `jigc doc rename`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `confirmed` — for ONE cell of the question, stated exactly below. The other cells of the question hold, and the report says which.
- **basis:** the adoption route lands both files under addressable names in every cell but one — `docs/research/UPPER.md` authored under its own heading's title (`Upper`, slug `upper`) on a volume that folds case: there `jigc doc author` refuses, and two refusals in a row print a fully spelled route that exits 1 when run as printed; nothing is lost, the listing stays as it was, and the previous release does the same byte for byte.
- **regression:** `false` — the same block is red on the previous release, the refusal texts byte-identical (`cmp` exit 0 on every captured pair).
- **contested:** `false` — the finding argues no decision wrong, and no settled decision I found intends the failing cell.
- **class:** `instance, unbounded`. I enumerated no consumer set of the mechanism. What was driven is under *What I drove*; a volume that does not fold case was **not** driven.

`<W>` is my own scratch directory, `<scratch>/verify-p2-adoption.4HehGD`, minted with `mktemp -d` under the scratch root the prompt names. `<repo>` is the repository of whichever rig a row names.

## The binaries

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives |
| the driven binary's directory first on `PATH`, then `command -v jigc` | printed that binary's path, exit 0, in every shell that drove anything — the candidate's directory for the candidate's rigs, the previous release's for its rigs, so a bare `jigc` inside a printed route resolved to the binary that printed it |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. Previous release: `jigc 1.0.0-rc.24`. No build ran; nothing under `target/` was driven.

## What I drove

Seven fresh rigs, each `dev/jigc-rig fresh --binary <that binary's path>` with `SCRATCH=<W>` — triage's *fixture fresh*. The two plants are `docs/research/UPPER.md` = `# Upper\n` and `docs/research/has space.md` = `# Spaced\n`; rigs C and PC plant `docs/research/lower.md` = `# Lower\n` in place of the spaced file, as a control. Every exit status was read bare; stdout and stderr were captured to separate files under `<W>`; every printed route was lifted out of the emitted bytes and run through `sh -c`, never retyped.

| rig | binary | plants | what was driven |
|---|---|---|---|
| A | candidate | untracked | `validate`, `ingest`, the `migrate` route for `UPPER.md`, its staging route, then the title `Upper` |
| B | candidate | committed | both routes whole: `has space.md` under the title `Has space`, `UPPER.md` under `Upper notes` |
| C | candidate | committed | `UPPER.md` under the title `Upper`, every printed route out of the refusal, the `lower.md` control, then the way out |
| D | candidate | untracked | both routes whole, through the staging route |
| PA | previous | untracked | as D |
| PB | previous | committed | as B |
| PC | previous | committed | as C, without the control's landing and without the way out |

The payload of every `jigc doc author research --from-file - --task <task>` was the one the composed workflow prints, its three slots filled with a short sentence each and `title:` set to the title the row names.

### 1. `validate` and `ingest` (rigs A, B, D, PA, PB — the same on both binaries, untracked and committed)

| argv | exit | what it printed |
|---|---|---|
| `jigc validate` | 1 | two `schema-conformance.unadopted-instance` rows — *its name is not a doc id, so no `<type>:<slug>` address reaches it* — each routed *adopt — run `jigc ingest` to route it, or `jigc migrate <repo>/docs/research/UPPER.md --as research`* (the spaced path single-quoted) |
| `jigc ingest` | 0 | both files `needs-reconcile … → research`, `blocking · conformance.section-missing`, route `jigc migrate <path> --as research` *— it opens the `migrate-research` workflow, which rewrites the file to conformant shape and adopts it at finalize* |
| `jigc doc list` | 0 | `research:UPPER … unregistered`, `research:has space … unregistered` |

`git status --short --untracked-files=all` was the same before and after the three: the two `??` lines where the plants were untracked, empty where they were committed.

### 2. The route, as printed — `jigc migrate <path> --as research`

| cell | binary | exit | what happened |
|---|---|---|---|
| untracked, either file (A, D, PA) | both | 1 | `blocking · migrate.source-untracked`, routed *stage it with `git -C <repo> add -- <file>`, then re-run `jigc migrate <path> --as research`*. Both commands of that route, run as printed: exit 0 and exit 0 (the re-run mints the task). `git status` after the refusal: unchanged. |
| committed, either file (B, C, PB, PC) | both | 0 | `task minted: migrate-research-docs-research-…`, and the composed `migrate-research` workflow |

So the route named by `validate` and by `ingest` **works as printed** in every cell: it runs, and where it refuses, the refusal's own route runs to a minted task.

### 3. The workflow that route opens, carried to its end — the cells that land

| cell | rigs | title | `doc author` | `task finalize` | `task finalize --approve` | after |
|---|---|---|---|---|---|---|
| `has space.md`, committed | B, PB | `Has space` | 0 — `research:has-space` | 4 (the review hold) | 0 — *deleted `docs/research/has space.md`, promoted `docs/research/has-space.md`, 2 files committed* | `doc list`: `research:has-space … managed`; `doc show research:has-space` exit 0 |
| `has space.md`, staged by the route | D, PA | `Has space` | 0 | 4 | 0 — *promoted `docs/research/has-space.md`, 1 file committed* | the same row, the same read |
| `UPPER.md`, committed | B, PB | `Upper notes` | 0 — `research:upper-notes` | 4 | 0 — *deleted `docs/research/UPPER.md`, promoted `docs/research/upper-notes.md`* | `doc list`: `research:upper-notes … managed`; `jigc validate` exit 0, *no findings* |
| `UPPER.md`, staged by the route | D, PA | `Upper notes` | 0 | 4 | 0 | the same; `jigc validate` exit 0 |

In each of these `git status` is clean after the approve, and the listing no longer holds an `unregistered` row for the file. **The slug is minted from the authored `title:`, never from the file's name**, which is why a name that is not a doc id is no obstacle here. This half of the question is answered *yes, it lands the file under an addressable name*, on both binaries.

### 4. The cell that does not land — `UPPER.md` under the title `Upper`

`Upper` is the foreign file's own `# H1`. It slugs to `upper`, and `docs/research/upper.md` is, on this volume, the file `docs/research/UPPER.md` (a probe in `<W>`: a file made as `CaseProbe` answers to `caseprobe`).

Rig C (candidate, committed), in order. Rig PC (previous release) was driven through the same steps except step 3, and gave the same exit at each; its captured stdout and stderr are byte-identical to rig C's (`cmp` exit 0) at steps 2, 4, 5, 6, 7, 8 and 9.

| # | argv | exit | stderr |
|---|---|---|---|
| 1 | `jigc migrate <repo>/docs/research/UPPER.md --as research` | 0 | - (task minted) |
| 2 | `jigc doc author research --from-file - --task <task>`, `title: "Upper"` | 1 | `blocking · write.non-reparseable` — *write rejected: the source does not conform to the schema (required section heading `## question` is missing)* · at `research:upper#question` · route: *nothing was persisted — revise the payload so the result still conforms (the message names the break), then re-run the same write; if the staged source itself is what no longer parses, `jigc task discard <task-id> --force` and start the task over* |
| 3 | step 2 again, unchanged | 1 | the same |
| 4 | `jigc task finalize <task>` | 3 | `blocking · finalize.migration-no-replacement` |
| 5 | the route's second arm: `jigc task discard <task> --force` (0), step 1 again (0), step 2 again | 1 | the same `write.non-reparseable` |
| 6 | the route's first arm, the payload revised: `title: "Upper notes"` | 1 | `blocking · write.identity-change` — *author rejected: this task's `research` is already `research:upper`, and this call would mint `research:upper-notes` instead* · route: *`jigc doc rename research:upper --to 'Upper notes' --task <task>` moves the doc this task already holds onto the title (and id) you asked for* |
| 7 | that route, as printed: `jigc doc rename research:upper --to 'Upper notes' --task <task>` | **1** | `blocking · write.identity-change` — *rename rejected: `research:upper` is committed, so `--to "Upper notes"` would move its identity to `upper-notes`* · route: *`jigc rename research:upper --to 'Upper notes'` moves it for real … once this task is finalized or discarded* |
| 8 | that route, as printed: `jigc task discard <task> --force` (0), then `jigc rename research:upper --to 'Upper notes'` | **1** | *`git mv docs/research/upper.md docs/research/upper-notes.md` failed: fatal: not under version control* … *Resolve the cause above, then re-run `jigc rename research:upper --to 'Upper notes'`* — no code, no `at:`, no route but itself |
| 9 | `jigc doc list` | 0 | stdout: `research:UPPER  docs/research/UPPER.md  unregistered` — the listing as it was |

After step 8: `git status` empty, `git log` unchanged, `docs/research/UPPER.md` still `# Upper`. **No bytes were lost and nothing was committed.**

Rig A reached the same step-2 refusal from the untracked plant, after the staging route (candidate only; the untracked plant under this title was not driven on the previous release).

**The control.** In rig C, `docs/research/lower.md` under the title `Lower` — a source whose path IS its canonical destination, byte for byte — lands: `doc author` 0, finalize 4 (*this migration deletes nothing (the canonical destination IS the foreign source)*), approve 0, `research:lower … managed`. So the refusal is not the in-place migration as such; it is the in-place migration whose two paths differ only in case.

**The way out, which no surface prints.** In rig C, after step 9: `jigc migrate …` again (0), then `jigc doc author` with `title: "Upper notes"` **as the task's first write** (0 — `research:upper-notes`), finalize 4, approve 0, `jigc validate` exit 0. The route of step 2 names both halves — discard and start over; revise the payload — but its message points the revision at `## question`, and taken singly each half fails (steps 5 and 6).

## Step 2 — is it what the finding says?

The finding is a cell nobody had driven: *whether that `migrate` lands such a file under an addressable name*. Driven, the answer has three parts, and I report all three so that the verdict is not read wider than it is:

1. **The route itself works as printed** (section 2), on both binaries, untracked and committed.
2. **It lands the file under an addressable name** whenever the authored title does not slug to the file's own name folded to lower case (section 3) — always, for `has space.md`, because no slug holds a space.
3. **It does not land `UPPER.md` under the title its own heading supplies** (section 4), and the reader is then walked through two printed routes that fail as printed.

I tried the ways a result like part 3 is usually wrong. The rigs and the scratch root are mine and new, and the cell was driven a second time on a clean rig (C) after rig A first met it. No exit status was read through a pipe; the decisive outputs are whole files, not cut. The binary is the candidate by hash and by `command -v`. The title is not contrived: it is the foreign file's `# H1`, which is what the composed workflow's mapping hands a reader for `title:`. And no settled decision intends it — the two that own this ground point the other way:

- DECISIONS.md → *2026-09-15 — M51 Increment 9 / T4* closed exactly this loop for a **conformant** file whose name is not a doc id — *`validate`'s route stops looping* — with a `git mv` route driven end to end. A non-conformant file never reaches that refusal (`ingest` answers `conformance.section-missing` first, as design/validation.md's `ingest.unaddressable-identity` row places it: *refused before the `adoptable` verdict is kept*), so its way out is `migrate`, and the same promise is owed there.
- The in-place migration is a supported cell with a carve-out of its own (crates/cli/src/doc.rs, the *in-location-squatter exception*: a migration whose recorded source path IS the canonical destination stages `created` over the foreign file). The control shows the carve-out working for an exact match. A search of DECISIONS.md, design/ and implementation/decisions-pending.md for a case-folding rule or a declared bound over this cell found none; the two case-folding decisions on record (the workbench predicate, the milestone branch names) fold case **because** this volume does.

What the mechanism is, as far as I read it and no further: in rig A, after the refused rename of step 7, the task's provenance records `research:upper` as `edited-from-base` with a `copied-in` entry, and `jigc doc show research:upper --task <task>` says the staged copy *does not parse: required section heading `## question` is missing* — the foreign bytes were taken for a committed managed doc at `docs/research/upper.md`. The fixer derives the axis; I did not.

## Step 3 — does it break the clause, inside its scope?

The clause is DECISIONS.md → *2026-10-04 — The exit rule, revised*, second clause, by its instrument: *no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed*.

- **First half: not broken.** Nothing that works on the previous release stopped working; the two binaries agree in every cell driven.
- **Second half: broken, in section 4's cell.** Step 6's refusal prints one fully spelled command — no placeholder, nothing to fill — and that command, typed as printed in the task that printed it, exits 1 (step 7). Step 7's refusal prints another, and it exits 1 after the discard it asks for (step 8), with a bare `git mv` failure whose only instruction is to run the same command again. Step 2's route is a loop on the door that printed it when followed by either arm alone (steps 3, 5, 6).
- **Scope.** The layout is ordinary and nothing in it is planted beyond what triage's fixture plants: a hand-written file with an upper-case name at a doctype's home, on the default volume of the platform this repository is developed on. I found no declared bound that takes a case-folding volume out of the clause.
- **What limits it, for whoever weighs it:** one title out of the titles a reader could choose; no loss; a way out exists; and it is not a regression.

For the other file and the other titles the basis would have been `does-not-reproduce`. The verdict is `confirmed` because the cell above is inside the question I was handed — it is the first title the route's own workflow suggests for one of triage's two files — and it survived the attempt to refute it.

## Step 4 — the regression fact

`regression: false`. Rig PC, previous release, fresh rig, the same block: step 1 exit 0, steps 2, 5, 6, 7 and 8 exit 1, step 4 exit 3 — the candidate's exits — and `cmp` of the candidate's captured stdout and stderr against the previous release's is exit 0 at each of them. Comparable: every door of the block exists on the previous release.

## Step 5 — coverage

The finding's coverage claim is that the route *was not run*. From the suites, not from any diff: **78** suite files under `crates/cli/tests` and `tooling-tests` spell the argv word `"migrate"`; of those, **0** hold a literal `docs/<home>/<name>.md` whose name carries an upper-case letter (pattern `docs/[a-z-]+/…[A-Z]….md`). A suite that composes the path some other way is outside this count. So no suite I found migrates a file at a doctype home whose name differs from a slug by case, under any title. The suite that drives a non-doc-id name to adoption, `crates/cli/tests/ingest.rs`, does it over a **conformant** file and through the `git mv` route, which is a different route from this one.

## Repro V-1

```yaml
claim: "on a volume that folds case, the adoption route `jigc migrate <path> --as research` for a committed docs/research/UPPER.md, authored under the title `Upper`, refuses at `jigc doc author` (`write.non-reparseable`), and the two routes printed next — `jigc doc rename research:upper --to 'Upper notes' --task <task>`, then `jigc rename research:upper --to 'Upper notes'` — each exit 1 when run as printed"
verdict: CONFIRMED
regression: false
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
previous: 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — the same exits, the same bytes
requires: "a volume on which `UPPER.md` and `upper.md` are one file"
setup:
  - fixture: fresh
  - "write docs/research/UPPER.md = `# Upper\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: a research note"]
repro:
  - ["jigc", "migrate", "<repo>/docs/research/UPPER.md", "--as", "research"]          # <task> = the id on stdout's `task minted: ` line
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin: the payload below, title "Upper"
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]      # stdin: the same, title "Upper notes"
  - ["jigc", "doc", "rename", "research:upper", "--to", "Upper notes", "--task", "<task>"]   # step 3's route, as printed
  - ["jigc", "task", "discard", "<task>", "--force"]
  - ["jigc", "rename", "research:upper", "--to", "Upper notes"]                       # step 4's route, as printed
observed:
  - exit: 0
  - exit: 1
    stderr_contains: "blocking · write.non-reparseable — write rejected: the source does not conform to the schema (required section heading `## question` is missing)"
  - exit: 1
    stderr_contains: "blocking · write.identity-change — author rejected: this task's `research` is already `research:upper`"
  - exit: 1
    stderr_contains: "blocking · write.identity-change — rename rejected: `research:upper` is committed"
  - exit: 0
  - exit: 1
    stderr_contains: "`git mv docs/research/upper.md docs/research/upper-notes.md` failed: fatal: not under version control"
  tree: "`git status --short` empty and docs/research/UPPER.md = `# Upper\n` after the last step; `jigc doc list` still `research:UPPER  docs/research/UPPER.md  unregistered`"
expect:
  - "no route a refusal of this block prints exits non-zero when run as printed in the state that printed it"
  - "docs/research/UPPER.md is never read as a committed managed `research:upper`"
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
where: "<W>/jigc-rig-fresh-iuHUrU (candidate, labels C1 to C4); <W>/jigc-rig-fresh-QvaugA (previous, labels PC1 to PC4); first met in <W>/jigc-rig-fresh-UA4nyS"
pinned-by: "UNPINNED: no suite migrates a file at a doctype home whose name differs from a slug by case"
```

**Pinnable as it stands: no.** Two reasons. Its red depends on the volume folding case, and the runner CI uses does not, so as written the block is red on this machine and says nothing there (not driven there — a guess, flagged as one). And `expect` is a property over the routes the block prints, not a fixed byte string: what the repaired door should answer — land `research:upper` in place, or refuse with a route that runs — is the fixer's and the design's to say. A test that pins it needs either an arm that states its skip out loud where `CaseProbe` and `caseprobe` are two files, or a seam below the binary that takes the two paths as given.

## Repro V-2 — the cells that hold

```yaml
claim: "the adoption route `jigc migrate <path> --as research`, carried to its end, lands a file whose name is not a doc id under an addressable name — the slug comes from the authored title"
verdict: REFUTED          # the finding read as: the route does not land such a file
basis: does-not-reproduce
binary: candidate c1 (sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc); the same on 1.0.0-rc.24
setup:
  - fixture: fresh
  - "write docs/research/has space.md = `# Spaced\n`"
  - ["git", "add", "--", "docs/research"]
  - ["git", "commit", "-q", "-m", "docs: a research note"]
repro:
  - ["jigc", "validate"]
  - ["jigc", "migrate", "<repo>/docs/research/has space.md", "--as", "research"]     # the route step 1 printed
  - ["jigc", "doc", "author", "research", "--from-file", "-", "--task", "<task>"]     # stdin as V-1, title "Has space"
  - ["jigc", "task", "finalize", "<task>"]
  - ["jigc", "task", "finalize", "<task>", "--approve"]
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "show", "research:has-space"]
  - ["jigc", "validate"]
expect:
  - exit: 1
    stdout_contains: "schema-conformance.unadopted-instance"
  - exit: 0
    stdout_contains: "task minted: "
  - exit: 0
    stdout: "research:has-space\n"
  - exit: 4
  - exit: 0
    stdout_contains: "  deleted docs/research/has space.md\n  promoted docs/research/has-space.md\n"
  - exit: 0
    stdout: "id  path  state\nresearch:has-space  docs/research/has-space.md  managed\n"
  - exit: 0
  - exit: 0
    stdout_contains: "no findings"
  tree: "`git status --short` empty after the approve"
where: "<W>/jigc-rig-fresh-MGAl3L (candidate, labels B1); <W>/jigc-rig-fresh-iATeMT (previous, labels PB1); the staged-not-committed arm in <W>/jigc-rig-fresh-E6ggZI and <W>/jigc-rig-fresh-hXweW0"
pinned-by: "UNPINNED: no suite carries `validate`'s `migrate` route for a non-conformant file whose name is not a doc id through to the listing"
```

**Pinnable as it stands: yes, with one line to re-derive** — on the `fresh` fixture, with `<repo>` and `<task>` read back from the run; nothing in it depends on the volume. Every rig I drove held `UPPER.md` beside the spaced file, so the listing I observed after this approve carried a second, `unregistered` row and the last `validate` exited 0 only once both files had landed; the block above is the one-file form, and its sixth and eighth expectations are what the two-file runs imply for it, not bytes I captured.

## Left open — hit on the way, not pursued

1. **`write.non-reparseable` says *nothing was persisted*, and the task is bound afterwards.** After the refused `jigc doc author` of section 4, a second author under another title is refused because *this task's `research` is already `research:upper`* (rig C step 6, rig PC the same). Whether that binding is what the sentence disclaims is the design's to say.
2. **A refused `jigc doc rename … --task` leaves a staged copy.** After step 7's exit 1 the task's working area holds `research:upper.md` (the foreign bytes), `jigc doc list research --task <task>` prints `research:upper  docs/research/upper.md  managed`, and `jigc task finalize` then blocks on three `conformance.section-missing` rows naming a path git has no file at.
3. **`jigc rename` fails with a bare `git mv` error** (step 8): no finding code, no `at:`, and a closing instruction to re-run the command that just failed.
4. **`jigc task finalize --approve` prints an `unadopted-instance` advisory for the file that same call retires**, routed at `jigc migrate` (rig B, the spaced file's approve).
5. **A source staged by `migrate.source-untracked`'s route leaves no row in the adopting commit** (rigs D and PA: *1 file committed*, the foreign file gone from the worktree). The composed workflow says so in advance; I did not examine it against `no-lost-files`.
6. **A volume that does not fold case was not driven**, on either binary. There the title `Upper` would name a second path beside `UPPER.md`; what the route does then is unknown to me.
7. **The untracked plant under the title `Upper` was driven on the candidate only** (rig A), and the way out of section 4 on the candidate only (rig C).

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else, before and after. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my seven rigs under `<W>`: the install commit `jigc setup` makes in each, one of mine in each rig whose plants are committed, and the commits the driven `jigc task finalize --approve` calls made.

<!-- end of report -->
