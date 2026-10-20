# The byte table — the four acceptance rows, written from the design docs

*Written 2026-10-12 on the branch `fix/rc24-route-catalog`, as task `B0` of [the pilot's plan](byte-table-plan.md) (§2, *The row and its allowance*; §8). A record: it runs nothing and decides nothing. No binary was run to learn what a command writes, and no list of files was taken from the product's source; every line below was read off a design doc, at the heading named.*

*Changed 2026-10-13, with the task `B2c` that lifts the rows into data
([install.rs](../../../../../crates/cli/tests/byte_table/install.rs), [task.rs](../../../../../crates/cli/tests/byte_table/task.rs), [milestone.rs](../../../../../crates/cli/tests/byte_table/milestone.rs)): the first entry of §6 is rewritten as the human ruled it on 2026-10-12, three lines stand in every row for it (§4a), and `M5` gains the sentence that names its place. No other line moved.*

*Changed 2026-10-20, with the second commit of the mint family's task (`B9`): the rows of `jigc start`, `jigc workflow`, `jigc migrate` and `jigc task amend` are written from their docs (§4d), before a cell of any of them runs; §7 gains the words they needed — the task a command mints (`<minted-task>`), *provisioned*, and the source a command reads (`<the-source>`). No other line moved.*

*Changed 2026-10-20, with the first commit of the mint family's task (`B9`), which records the human's two rulings of that day: §6, entry 23 is **ruled**, stricter than its option A, its question kept — jigc never moves or overwrites a file to make room for its own; **any file at a mover's destination, committed or untracked, managed or not, refuses**: `relocate` blocks that doc, moves the others, names the path to clear and exits non-zero, and a root knob refuses its whole re-point, the knob unchanged; an unmanaged doc elsewhere in the target folder blocks nothing. And the review of `B8`'s R1, which the eighth fix had recorded as a key and no entry, is ruled: an acting sweep exits non-zero where an act it was asked for did not happen. §4c: `RL3` and `CS3`, the lines that parked a destination's occupant, are struck; `RL1, at its new home` and `CS2, at its new home` are lines, the destination a place of its own, `<a-new-home>`; `relocate`'s lines that write hold on a refusal that names a doc its sweep left (RL5). §7 says *into its home* at both ends, *moved aside* under a directory only, the new place and the new condition.*

*Changed 2026-10-20, by the eighth of the fixes after the review of the movers family's task (`B8`), its second commit (the review's R1, R2, R6 and R7, and the seventh fix's left-opens): §6, entry 23 is reframed — the design answers it on purpose for these doors, parked, *"the settled **foreign-only** bound"*, and the robust answer reverses it: block that doc at `relocate`, refuse the whole re-point at a root knob — where it was framed as two docs saying two things, with one doc blocked as the answer everywhere; it quotes `design/reconciliation.md`'s bound, its decision of 2026-07-07 and the exit `design/validation.md` gives a blocked doc, and names `config set placement-root`, which parks. RL5 says the docs choose exit 0 for a blocked doc on purpose, and the findings judge them. §4c's *What the docs do not say* no longer lists a destination's occupant, foreign or managed, which the docs say (R7), and gains two things a line cannot say: what a cache holds on a refusal that committed, and a refusal after more than one commit. No line moved.*

*Changed 2026-10-20, by the eighth of the fixes after the review of the movers family's task (`B8`), its first commit (the review's R3, R5, R9 and R11, the fifth and seventh fixes' left-opens, and the human's rulings of 2026-10-19 on entries 21 and 22): §6, entries 21 and 22 are **ruled**, each option A, their questions kept — `migrate-corpus --no-commit` stages exactly the paths the migration touched, both halves of a move, and commits nothing; a `migrate-corpus` whose commit a hook rejects rolls back. §4c's `MG4` is a line now, the staging of the spelling `--no-commit`, on the sentence of `design/write-commands.md` the ruling found right; MG1's condition reads the ruling on entry 22. §7's *moved aside* keeps the relative path the bytes had in the place they left, one component of it under the first free `<name>.<n>` where the name stood, the rule `design/storage.md` states. Entry 22 quotes the door's state-truth clause in `design/finalize.md` and the two decisions built on it (R3); entry 21's robust answer says why its staged halves are not the half-commit §23 of the findings rejects (R11); the header names entry 23 open (R5). No other line moved.*

*Changed 2026-10-20, by the seventh of the fixes after the review of the movers family's task (`B8`) (its H8, H9 and H10): three of §7's words are held to their sentences. *It committed* lets a refusal change what git tracks only as its commit landed it, so the doc the refusal reported blocked is not written; *its knob set* holds the knob at the value the command names, under the `scalar:` map, made only where none stood; and *moved aside* holds an arrival under its own unit's place — a component the place and the directory both name is one unit — and `T6`, `M6` and `M6, the milestone's area moved aside` name the unit's place under `.jigc/displaced/`, as their sentences do. No other line moved.*

*Changed 2026-10-20, by the fourth of the fixes after the review of the movers family's task (`B8`) (its H4, H11 and H7): `jigc rename`'s row gains `RN1, at its new home`, whose place, `<the-new-doc>`, is where the doc stands under the slug the command gives — `--slug`'s, else the one `--to`'s title gives — so that what squats a rename's destination is a need of the row, and RN1's arrival is held to that slug where any free path of the home directory held; §7's *retitled* is read at its two places, and *repointed* holds a changed line to a ref-field of the front matter, the old identity a whole value of it, re-emitted in the field grammar's canonical form — where a line holding the old identity anywhere, as a substring, could take the new one. No other line moved.*

*Changed 2026-10-19, by the third of the fixes after the review of the movers family's task (`B8`) (its H3): §7's *migrated* holds every byte the migration's sentences do not let it change, read off the doctype's two schemas — where it held the prose lines alone, so that a migration dropping a `- key:` bullet, a sub-heading of the author's or a `supersedes` edge, or flipping a status, held. No other line moved.*

*Changed 2026-10-19, with the second commit of the movers family's task (`B8`): the rows of `jigc rename`, `jigc relocate`, `jigc migrate-corpus` and `jigc config set` are written from their docs (§4c), before a cell of any of them runs; §6 gains **entry 21**, where two docs say two things of `migrate-corpus --no-commit`; and §7 gains the words the rows needed — a doc of the store at its doctype's home, read as the cascade resolves it, the doc an address names, the prior home `--from` names, *retitled*, *repointed*, *into its home*, *migrated*, *its knob set* — and says that a commit is read by the word its line has. A place where the docs agree on what makes no sense for a user is a finding and no line (§4c, *What the docs do not say*). No other line moved.*

*Changed 2026-10-19, with the first commit of the movers family's task (`B8`), which records the human's three rulings of that day: §6, entries 18 and 20 are **ruled**, each option A, their questions kept — a landed boundary moves a sub-task worktree's unstaged and untracked files that git does not ignore into `.jigc/displaced/<sub-task-id>/`, and `milestone provision` skips a sub-task the record has settled — and the second of the human's declared bounds of 2026-10-06 is **retired**: a `doc` write that would replace a part holding an uncommitted hand edit the copy-in carried refuses at the write. §4 gains `M6, the worktrees moved aside`, whose sentence is the ruling on entry 18 as [DECISIONS.md](../../../../../DECISIONS.md) records it; §7's *moved aside* names a place whose components may stand for the unit and its sub-tasks, and `<unprovisioned>` reads the ruling on entry 20. No other line moved.*

*Changed 2026-10-19, by the seventh of the fixes after the review of the milestone family's task (`B7`), part B (its fourth, seventh, tenth and eleventh findings, the second and sixth fixes' left-opens, and the human's ruling of that day): §6, entry 19 is **ruled** — a finalize never carries a change under `.jigc/config` that nobody staged — its question kept, its two cells unmarked and red under their keys; §6 gains **entry 20**, open — whether `provision` may make a worktree for a sub-task the record has settled, which `<unprovisioned>` now excludes as the stricter reading; `MP1` reads *from the pinned base's tree* in place of *rebuilt*; the re-seed's areas' half holds on a refusal (*"before their own guards"*) at every door that re-seeds; MS4's commits before the last hold the record partway under *merged into*, read off the commit's bytes; the rendered message's file stands under no line; §4b's *What the docs do not say* gains the cache's width as a row, and §7 the words. No sentence of a line moved; one quote is added to the re-seed's areas' lines.*

*Changed 2026-10-19, by the sixth of the fixes after the review of the milestone family's task (`B7`), part B (its eighth and ninth findings): §7's *a place the command mints* is narrowed — `<minted>` is **the one id the committed record gains** (an item of the unit's record, or a record at the home), at which nothing stood in the working tree **or in what git held** (`HEAD`'s tree and the index); a record deleted from the working tree that `HEAD` still held had counted as nothing stood, and a mint of two, an area at an id no record gained and an area made for a settled sub-task had each held. `add-from-spec`, which mints one a criterion, reads the new word **`<each-minted>`**, any id the record gains (MS1). §7's *a worktree's registration added* asks the checkout too: its `HEAD` detached at the base the unit's committed record pins, its index that commit's tree (MP2). §4b's *A mint of two* is corrected where it stands. No line was added or removed, and no quote moved.*

*Changed 2026-10-19, by the fourth of the fixes after the review of the milestone family's task (`B7`), part B (its fifth finding): `jigc milestone join`'s row gains **MJ4**, the committed edge index its ref walk reads, *rebuilt* where its file is absent or its stamp stale (§4b). Driven on both binaries, the join wrote the file again at exit 0, and no line covered it; every cell's leads had built the index first. §4b's *"What `execute` and `join` write"* is corrected where it stands, and §7 names the line. Of the family's other doors, only `finalize` reads the index, and `M10` carries it.*

*Changed 2026-10-19, by the third of the fixes after the review of the milestone family's task (`B7`), part B (its third finding): §6, entry 18 is rewritten with every sentence that bears on it. For the narration: `design/finalize.md`'s restatement of 2026-10-06 and its build entry, both the builder's; `design/storage.md`'s sentence naming narration the worktrees' warrant; the pinned `sub_tasks[].discarded` key. Against it: the human's *"Over take-and-name"*, and their rulings on items 10 and 4 and on item 6. Its *"predates the rule"* is corrected, since the narration was restated after the rule. The question is narrowed to a file git does not ignore, since the human's rulings answer the ignored half. It stays open: no ruling of theirs speaks of a landed sub-task's unstaged or untracked bytes. The robust answer no longer calls the move the disposition the boundary already gives, because `design/storage.md` gives the worktrees narration on purpose. No line moved, and the cell's mark is as it was.*

*Changed 2026-10-19, by the first of the fixes after the review of the milestone family's task (`B7`), part B (its first finding): `M3, the config` held under any `HEAD`, where its sentences say *first commit* — so a file of the user's nobody staged under `.jigc/config`, swept into a landed boundary, held. It is in force on the layer's first commit alone, a condition of its own (§7, *the place's first commit*), and `jigc task finalize`'s row gains the same two lines, `T3, the config` and its ignore file's half, so that the two rows read the stage's sentence one way (§3, §4); the doc-only arm's G8 sentence is struck as a ground of the line. Whether a pending delta rides a commit that is not the layer's first is §6, entry 19 — two docs say two things — where §4 and the family's record had called it answered.*

*Changed 2026-10-19, with the fourth commit of the milestone family's own, which brings the cells of `discard` and of `milestone finalize`'s plain spelling: `M6` gains its moved-aside half for the milestone's own area, whose complement its second sentence keeps too — the line as data had moved aside the sub-tasks' areas' alone (§4); and §6 gains entry 18, where the design doc's narrated loss at a landed boundary meets the exit rule. No other line moved.*

*Changed 2026-10-19, with the second commit of the milestone family's own: §4b's note on the message file of a record commit says what the record doors' cells showed. No line moved.*

*Changed 2026-10-19, with the first commit of the milestone family's own (the task `B7`, its second part): the rows of `jigc milestone create`, `add-task`, `add-from-spec`, `provision`, `execute`, `join` and `discard` are written from their docs (§4b), before a cell of any of them runs; `milestone finalize`'s row gains `M3, the config` — its own sentence's *first-commit config*, which the line as data had held nothing of (§4); and §7 gains the three words they needed — a place the command mints, a worktree's registration added, and carried as it stands. No other line moved.*

*Changed 2026-10-19, by the fifth of the fixes after the review of the milestone family's task (`B7`), part A, which reconciles the records' counts: §3's paragraph on the doc-only arm said no cell of the acceptance is one, and one is; corrected where it stands. No line moved.*

*Changed 2026-10-18, by the fourth of the fixes after the review of the milestone family's task (`B7`), part A (its first and third findings): §6, entries 16 and 17 are answered by the design docs, and were no silences. Entry 16 — the edge index's file may be gone after a finalize, whose next read rebuilds it — is carried by a word of its own, *invalidated* (§7), on `T5`'s and `M10`'s edge-index halves, which quote three more sentences of `design/storage.md`; its false premise, that `task finalize` and the `squash: true` boundary write the file again, is corrected. Entry 17 is answered by `design/command-output-contract.md`, and the one sentence of `design/finalize.md` that says otherwise is a doc-only finding.*

*Changed 2026-10-18, by the third of the fixes after the review of the milestone family's task (`B7`), part A (its fourth, eighth and ninth findings): the `squash: false` spelling checks its chain. `M4` held what `M3` holds, so a product that ignored the knob kept the spelling's cells green; it is carried by a word of its own now (§7, *a chain*) — a commit for each sub-task whose worktree staged code, in task-id order, each holding what that worktree staged and nothing else, then the aggregate, holding only what the row's other lines wrote — and `M4`'s *What it cannot say* is said. `T9` names what it leaves out, the doc-only arm's owner-artifacts, a lead. And a need of a row is keyed by the spelling as well as the line: a cell of `doc-only` aimed at `T1` answers `T1` under `doc-only` and no longer the plain spelling's need, which has a cell of its own.*

*Changed 2026-10-18, by the second of the fixes after the review of the milestone family's task (`B7`), part A (its fifth, sixth and seventh findings): how a line names its unit. `M5` and `A5` read `docs/milestone-records/*` — any milestone's record — and read `<docs-root>/milestone-records/<unit>.md` and `<docs-root>/milestone-records/<its-milestone>.md` now: the milestone's own record, and the one record that names the discarded sub-task, each under the docs root the cascade resolves, each on a sentence it now quotes; §7 gains the fixed suffix, `<its-milestone>` and `<docs-root>`, and says what *a task that stands* (an area carrying its base pin) and *a committed record* (the bytes `HEAD` holds at the record's resolved home) are. No sentence of a line moved; two lines gained quotes.*

*Changed 2026-10-18, by the first of the fixes after the review of the milestone family's task (`B7`), part A (its second finding): three sets a line is read through beside its place were blind to the unit the command acts on, and are not now. §7's *a copy of a staged doc* is a doc staged **in the unit** — its own task area, its sub-tasks' areas, its own merged set — so a sibling's doc promoted, or another milestone's merged doc, is no copy `T1` or `M2` lets land; `M3`'s and `M4`'s commit holds what the checkouts at `.jigc/worktrees/<sub-task>` staged, and no other checkout's, on a third sentence `M3` quotes; and `T8`'s rule reads every unit's staged docs and every merged set together, where a joined milestone in the repository had left a task's own docs unread. No sentence of a line moved.*

*Changed 2026-10-18, with the third commit of the milestone family's task (`B7`), which builds the spelling switch: §3's doc-only arm is the line `T9`, which takes `T2`, `T3` and `T4` out of force — `T2` too, read whole, where the paragraph had said `T3` and `T4` — and `M4` takes `M3` out of force, with a second sentence; §6 gains entries 16 and 17; §7 the word a spelling is carried by.*

*Changed 2026-10-18, with the second commit of the milestone family's task (`B7`): §7 gains a place one component of which is the work unit the command acts on, or a sub-task of it, and §3b's *What a line cannot say* is said; `W1`, `W4`, `T6`, `A1`, `M1` and `M6` are carried by it as data. No sentence of a line moved.*

*Changed 2026-10-18, with the first commit of the milestone family's task (`B7`), which records the human's three rulings of that day: §6, entries 14 and 15 are ruled, each option A, and say how each is carried; and the two second spellings — the doc-only arm of `task finalize` (§3) and `finalize.fan-out.squash: false` (§4, `M4`) — are ruled to be built, by a switch with which a cell names a spelling of its leaf, in a commit of their own. No line moved.*

*Changed 2026-10-18, by the third of the fixes after the doc family's review: §6, entry 14 called a place silent where the docs say two things, and held two questions. It is rewritten as two docs saying two things, quoting both sides, and split — (a), whether a write's copy-in takes a file that is no doc of its type, stays entry 14; (b), whether a write that copied in and then refuses keeps the copy, is entry 15, answered at `doc author` by its own doc and open at the five edit leaves. `W5` says that each of its sentences is a refusal before the copy-in, and §3b that the docs are not silent there. No line moved.*

*Changed 2026-10-18, with the third commit of the doc family's task (`B6`), which brought the cells of its two minting doors: §6, entry 14 says what they met. No line moved.*

*Changed 2026-10-18, with the second commit of the doc family's task (`B6`), which brought the cells of its six edit leaves: one more place where the docs are silent is listed (§6, entry 14). No line moved.*

*Changed 2026-10-18, with the first commit of the task `B6`'s doc family: the rows of the eight `jigc doc` write leaves are written from their docs (§3b), before a cell of any of them runs, and §7 gains the word their third line is carried by — *read, and nothing written*. No other line moved.*

*Changed 2026-10-18, by the fourth of the fixes after the review of the task `B6`'s first two commits: §6, entry 10's ruling reads in the human's words, with the table's scope as a note beside it, and entry 1 says which four cells lie between the *seventeen* it projected and the thirteen that hold. No line moved.*

*Changed 2026-10-18, after the review of the task `B6`'s first two commits: §6, entry 13 is answered by the design docs — two sentences it had not read — and says so; it was no silence. No line moved.*

*Changed 2026-10-18, with the second commit of the task `B6`, which builds the plant in the place of a file that stands and fills the needs that waited for it: one more place where a doc is silent is listed (§6, entry 13). No line moved.*

*Changed 2026-10-18, with the first commit of the task `B6`, which records the human's rulings on the install family's questions: entries 1, 3, 9, 10, 11 and 12 of §6 are ruled and say how each is carried; `setup`'s row gains `S16`, whose sentence is the ruling on entry 1 as [DECISIONS.md](../../../../../DECISIONS.md) records it, and §7 the word it is carried by. Entry 11's record said the opposite of what the ruling holds, and is corrected. No other line moved.*

*Changed 2026-10-15, with the third commit of the task `B5`, which brought the install family's cells: entry 1's first open point is met (§6, entry 1), entry 3 is seen, and three more places where a doc is silent are listed (§6, entries 10 to 12). No line moved.*

*Changed 2026-10-15, with the second commit of the task `B5`: the rows of `jigc ingest` (§2b) and `jigc unmanage` (§2c) are written from their docs, before a cell of either runs; one more place where docs say two things is listed (§6, entry 9). No line moved.*

*Changed 2026-10-15, with the first commit of the task `B5`, which records the human's rulings on the first cut: entries 2, 5, 6 and 8 of §6 are ruled and say how, entry 3 is read under the ruling on entry 2, and the plan's first question of reading is ruled (§6, *Decided here*); `S12` says it is no line and why; `uninstall`'s row gains `U9`, whose sentence is the ruling on entry 5 as [DECISIONS.md](../../../../../DECISIONS.md) records it, and §7 the word it is carried by. No other line moved.*

*Changed 2026-10-13, with the second commit of the task `B4`, which brought the family's cells: the sixth entry of §6 says what a cell found, and an eighth is listed — a change staged at a path the command stages itself — which three cells met. No line moved.*

*Changed 2026-10-13, with the first commit of the task `B4`: the row of `jigc task discard` is written from its docs (§3a), one more place where a doc is silent is listed (§6, entry 7), and three entries change the word they are carried by as data — `S4`, `T5` and `M10` are *rebuilt*, where they were *written whole* over a directory (§7, the note under its table). No sentence of a line moved.*

**What a row is.** The byte table compares a repository's bytes before and after one writing command. A leaf's *row* says what that command may change — its *allowance* — line by line, and each line rests on one sentence of the design doc that owns the command. This file holds the rows of the four commands the plan's acceptance needs: `jigc setup`, `jigc uninstall`, `jigc task finalize` and `jigc milestone finalize` — and, since the plan's task `B4`, the row of `jigc task discard`, which its family's cells are run under; since `B5`, the rows of `jigc ingest` and `jigc unmanage`, which the install family's cells are run under with `setup`'s and `uninstall`'s; since `B6`, the rows of the eight `jigc doc` write leaves, the doc family (§3b); since `B7`, the rows of the seven other leaves of the milestone family (§4b); and since `B8`, the rows of the movers family's four (§4c); and since `B9`, the rows of the mint family's four (§4d).

**How a line is written.** Its number; the place it speaks of; what may happen there; when, where the doc gives a condition; and whether it still holds when the command refuses. Under it, set off as a quotation, **the sentence the line rests on — a run of the doc's own bytes, markup included, never a paraphrase** — and under that the doc and the heading the sentence stands under. A line that needs two sentences quotes both, each with its own source. The quotation is what `B1`'s fence holds against the doc: a doc that moves reddens the row.

**How a silence is written.** Where no sentence was found for something a command plainly has to do, the row says *the doc is silent*, no line is made up for it, and it is a numbered entry of §6 — the list for the human, in the form of [the route catalog's list](route-catalog-doubts.md): the words, the readings, what each makes of which cells.

**What was read.** `design/project-setup.md` (whole), `design/assistant-adapter.md` (*Generated, minimal, regenerated* to *2. Make `jigc` frictionless*), `design/finalize.md` (*What `finalize` is*; phases 2 to 7; *Rollback discipline*, its table's rows 4 to 6; *`fan-out` finalize*; *The doc-only arm*; *What `finalize` does NOT do*), `design/storage.md` (*Source of truth*; *Repository layout & state location*; *The per-task working area (staging)*; *Derived caches*; *Concurrent writers*, its last part), `design/team-ready-state.md` (*The commit model*; *The working area's two populations*; *`jigc milestone discard <id>`*), `design/surface-contract.md` (the carryover gate's paragraph on `setup`), `design/measurement.md` (*The in-repo invocation log*, its first paragraph). **Not read:** `design/finalize.md` → *1. Preflight*, *The amend arm* and *Commit-doc rendering*; `design/reconciliation.md`; `design/validation.md`; `design/write-commands.md` — §6 says where a row may be short for it.

---

## 1. `jigc setup`

Owner docs: `design/assistant-adapter.md`, `design/project-setup.md`.

**S1 — `.jigc/AGENT.md`: written whole.** When: always. On a refusal: no (S13).

> a stable one-line pointer (Claude Code: an `@.jigc/AGENT.md` import) to a **managed bootstrap file** the adapter writes under `.jigc/` and **regenerates on every `setup`**

— `design/assistant-adapter.md` → *1. Inject the bootstrap — hook (primary) + line (floor)*

**S2 — `CLAUDE.md`: merged into — the one import line is added, and every line that stood there stays.** When: always. On a refusal: no (S13).

> The always-loaded file is touched only to ensure the pointer is present (idempotent on the exact reference line); the content lives in the managed file.

— `design/assistant-adapter.md` → *1. Inject the bootstrap — hook (primary) + line (floor)*

**S3 — `.claude/settings.json`: merged into — the permit, the `deny` floor and the `SessionStart` hook are added where absent, and every entry that stood there stays.** When: always. On a refusal: no (S13).

> a pre-existing non-trivial `CLAUDE.md` / `.claude/settings.json` is **merged, never clobbered** (structure-aware)

— `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation*

**S4 — `.jigc/config/`: the project layer is made where absent; its files `.jigc/config/.gitkeep` and `.jigc/config/packs.yaml` are written whole.** When: always. On a refusal: no (S13).

> `jigc setup` also **initializes the in-repo project layer** (`.jigc/config/`) when absent

— `design/assistant-adapter.md` → *Generated, minimal, regenerated*

> a file at a path it **replaces** (`.jigc/AGENT.md`, `.jigc/config/.gitkeep`, `.jigc/config/packs.yaml`, the settings record `.jigc/settings-entries.json`, a `.jigc/version` that is not jigc's own one-line stamp) is refused before the first write while no commit holds its bytes, on an unborn `HEAD` as on a born one

— `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation*

**S5 — `.jigc/version`: written whole.** When: always. On a refusal: no (S13). The sentence is S11's second, which names it a member of the install; that it is replaced and not merged is S4's second sentence.

**S6 — `.jigc/settings-entries.json`: written whole, or removed.** When: the doc's condition is *where its install commit can carry the settings file* — written there, removed elsewhere. On a refusal: no (S13).

> `setup` writes it only where its install commit can carry the settings file, and removes a record that stands where it cannot

— `design/assistant-adapter.md` → *2. Make `jigc` frictionless*

**S7 — `.jigc/.gitignore`: merged into.** When: always. On a refusal: no (S13).

> `.jigc/.gitignore` is the one merged-into member a link is never followed at — its writer amends in place

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S8 — the guide, `.claude/skills/jigc/SKILL.md`: written whole.** When: nothing stands there, or what stands there is jigc's own copy; a copy somebody edited is left as it is. On a refusal: no (S13).

> **rewritten whole on every `setup`**, the `.jigc/AGENT.md` mold

— `design/assistant-adapter.md` → *The adapter's owned artifacts (M48)*

> `.claude/skills/jigc/SKILL.md` is Claude Code's skill convention, not jigc's

— `design/assistant-adapter.md` → *The adapter's owned artifacts (M48)*

> and is left **byte-identical**, by `setup` and by `uninstall` alike

— `design/assistant-adapter.md` → *The adapter's owned artifacts (M48)*

**S9 — the root `.gitignore`: made, or merged into.** When: **the repository has no commit** — read off the state before the command. On a refusal: no (S13).

> Setup seeds the floor **iff the repo has zero commits**

— `design/project-setup.md` → *The secrets-floor `.gitignore` — fresh-repo install only (M36)*

> Absent → create; present → append-if-absent; re-run → byte-stable no-op.

— `design/project-setup.md` → *The secrets-floor `.gitignore` — fresh-repo install only (M36)*

**S10 — the `pre-commit` hook, in the hooks directory git resolves: made, or merged into — jigc's block is added and every other line stays.** When: always. On a refusal: no (S13).

> `jigc setup` installs a `pre-commit` hook (into the hooks dir git resolves — usually `.git/hooks/`, see the install discipline below)

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

> **non-destructive** (preserves/wraps any existing `pre-commit` hook)

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S11 — one commit on the checked-out branch, holding the paths of S1 to S10 that are there and that git does not ignore, and nothing else: not the index as it stood.** When: always. On a refusal: no.

> `jigc setup` lands its footprint as a dedicated `chore(jigc): install jigc workspace config` commit so the scaffolding never rides the user's first `finalize`

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

> The pathspec is **enumerated, never a blanket `git add -A`**: the always-loaded host file (`CLAUDE.md`), the settings file (`.claude/settings.json`), the managed `.jigc/AGENT.md`, `.jigc/.gitignore`, the `.jigc/version` provenance stamp, the project layer (`.jigc/config/`)

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S12 — the index, at the paths of S1 to S10: staged.** When: on a refusal that came **after** the install's first write, and then the written files of S1 to S10 stand too. On a refusal: yes, that one, as the doc has it. **No line of the row** — by the human's ruling of 2026-10-14 on §6, entry 2: every refusal of `setup` leaves the repository as it was, caches aside, so the row holds `S13` on every refusal, and this sentence is a design inconsistency, recorded in [byte-table-findings.md](byte-table-findings.md) and corrected with the product's fix.

> records the install footprint (M51's `record_install_footprint`) over the paths it wrote in that run, clean against `HEAD` before it and dirty at the failure, and **stages them**

— `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation*

**S13 — a refusal before the first write changes nothing.** This is the sentence S1 to S11 take their *on a refusal: no* from — and, by the ruling on §6, entry 2, the rule for **every** refusal of `setup`, after the first write too.

> **And because the ask is pre-write, the refusal installs nothing**

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S14 — the consent is `--force`, and what it destroys is named.** A cell under the consent spelling is exempt from the plant's rule only where the command's output names the planted path.

> **`--force` is consent, not preservation:** it lets the install run, so at a whole-rewrite path the adopter's bytes are replaced on the way to the commit — which the install then **says**, as an advisory `setup.forced-install-path` naming every path the consent was spent on

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S15 — a condition with two sides, which the plan's acceptance runs on both: `HEAD` born or unborn.** No line of its own: it says which of S2, S3, S7, S9 and S10 a file somebody left at the path rides the first commit under, and that S1, S4, S5 and S6 refuse over one on either side.

> on an unborn `HEAD` an *untracked* file at a path the install **merges into** rides the repository's first commit instead of refusing — the on-ramp — while a path it **replaces** refuses there too

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

**S16 — `.jigc/state/` and `.jigc/index/`, the workbench's cache directories: laid out — each directory made, and no file written into it.** When: always. On a refusal: no (S13); what a refusal may do there is `C1` and `C2` (§4a). The sentence is the human's ruling of 2026-10-15 on §6, entry 1, as `DECISIONS.md` records it — the doc is silent, and the ruling says what the command does; `design/assistant-adapter.md` → *Generated, minimal, regenerated* is to say it, a doc-only finding. The two places are the directories `design/storage.md` declares the workbench's caches (§4a's `C1` and `C2`); a milestone's `{base,tasks}.json` stand under a milestone's own area, which no `setup` makes. As data the line is two, one a directory.

> **`jigc setup` also lays out the workbench's cache directories**

— `DECISIONS.md` → *2026-10-18 — The human's seven rulings on the install family's questions, recorded: `setup` refuses over a path whose bytes are not jigc's own; it lays out the workbench's cache directories; it refuses a link at the root `.gitignore` on any `HEAD`; `unmanage` never commits; the install commit runs the user's hooks; the install-footprint record is no cache; and `B6` builds the plant in the place of a file that stands (the byte table, task `B6`, the first of its commits)*

**What the docs do not say of `setup`:** where the record of S12 is kept (§6, entry 3 — **ruled on 2026-10-15**: it moves out of `.jigc/state/`, and the table checks the path it was seen under); where *the spawn launch-template* of the boundary table is written (§6, entry 4); whether a `setup` that exits 0 writes under `.jigc/index/` or `.jigc/state/` (§6, entry 1 — **ruled on 2026-10-15**, and carried by `S16`). **Declared, and not a line here:** a `CLAUDE.md` or a settings file that is a link is followed to a regular file inside the checkout, which then becomes a member of the install (*`jigc setup` has assistant-*neutral* install responsibilities too*, the paragraph on a file the install replaces) — the row gains that line with its sentence when a cell plants a link there, which no cell of the first cut does.

---

## 2. `jigc uninstall`

Owner doc: `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*; every sentence below stands under that heading unless another is named.

**U1 — `.jigc/` and everything under it: removed.** When: none of the four refusing states stands (U7). On a refusal: no (U7).

> **`uninstall` removes exactly the enumerated repo-local setup-created set — now *complete*: `.jigc/`, the import line, all three `.claude/settings.json` writes (permit + `SessionStart` hook + `deny` floor), the `pre-commit` hook, and the adapter's owned guide artifact**

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U2 — `CLAUDE.md`: taken out of — the import line leaves, and every other line stays.** When: always. On a refusal: no (U7).

> unwire the `CLAUDE.md` import line

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U3 — `.claude/settings.json`: taken out of — the entries `setup` recorded leave, and every other entry stays.** When: git tracks the settings file and the record; elsewhere none of the three kinds leaves without the consent. On a refusal: no (U7).

> `setup` now records the entries it adds in one committed file, `.jigc/settings-entries.json`, and the teardown removes exactly those, reading the record **only where git tracks the settings file**

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U4 — the `pre-commit` hook: taken out of — the lines between jigc's two markers leave; the file is removed only where nothing else is left.** When: always. On a refusal: no (U7).

> **`uninstall` removes exactly the lines between jigc's two markers — whatever version wrote them, whatever stands around them**: what is left is restored byte for byte, and the file is removed only where nothing is left but the frame jigc itself puts around its block

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U5 — the guide, `.claude/skills/jigc/SKILL.md`: removed, with the directories jigc made for it once they are empty.** When: what stands there is jigc's own copy. On a refusal: no (U7).

> jigc's own copy is removed, and a copy the user has edited is **left in place and reported**

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

> The **directories jigc created to hold it** go with it once they empty, stopping at any directory that houses another host file

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U6 — git's registrations of worktrees: removed, at a path under `.jigc/worktrees/` and at no other.** When: always. On a refusal: no — the sentence says so itself.

> A door removes a registration from `.git/worktrees/` **only at a path it created** under `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>`, or a boundary's own `.combine-*` — **by path**, one record at a time (`git worktree remove --force <path>`), and **a run that refuses removes none**

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**U7 — a refusal removes nothing.** This is the sentence U1 to U5 take their *on a refusal: no* from. The four states it speaks of are a worktree-shaped path under `.jigc/worktrees/` that holds content, an open task's staged doc, a file under `.jigc/` that no index has a copy of, and a file jigc did not write in a working area, under `.jigc/displaced/`, or under a directory of `.jigc/` nobody has stated an owner for.

> All four remove nothing, name what they found, and route at the honest exits

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U8 — the consent is `--force`, and what it takes is named.**

> the one way past it is **`--force`**, the operator's explicit consent to delete, so the destruction is asked for rather than assumed

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

> so a staged file is **narrated, not refused**, and what the guards let through this door still names before it removes it

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**U9 — one commit on the checked-out branch, holding what U1 to U5 changed at a path a commit held, all of it, and nothing else: not the index as it stood, and no edit of the user's that no commit holds.** When: always. On a refusal: no (U7). The sentence is the human's ruling of 2026-10-14 on §6, entry 5, as `DECISIONS.md` records it — the doc is silent, and the ruling says what the command does. A removal or a taking-out at a tracked path that the working tree keeps with no commit carrying it is outside this line.

> **The teardown lands as one dedicated, path-scoped commit of what it removed**, the inverse of `setup`'s install commit, and **never carries the user's own uncommitted edits** at a path it touches

— `DECISIONS.md` → *2026-10-15 — The human's six rulings on the byte table's first cut, recorded: the families `B5` to `B9` are built now; what is held reaches what a reflog names; a change staged at a path jigc stages itself is a finding beyond the one declared bound; every refusal of `setup` leaves the repository as it was; `uninstall` lands its teardown in one path-scoped commit; a rejected record commit's message file is a finding (the byte table, task `B5`, the first of its commits)*

**What the docs do not say of `uninstall`:** what it does to git's index, and whether it commits (§6, entry 5) — **ruled on 2026-10-14**, and carried by `U9`. Whether it refuses over an edit of the user's at a path it touches, or commits only jigc's part of that path, the ruling leaves to the fixer; the row holds what both answers share — no such edit rides the commit.

---

## 2b. `jigc ingest`

*Written 2026-10-15, as the first commit of the install family's cells (the plan's task `B5`): the row from its docs, before a cell of it runs. No binary was run to learn what the command writes, and no list of files was taken from the product's source.*

Owner doc: `design/project-setup.md` → *Flow 2 — existing project: bounded detect-and-route ingestion*, its sections *The bounded slice — three net-new but tractable pieces* and *Idempotency & irreversibility — the acceptance obligation*.

**The command adopts what it finds conformant, and nothing else changes.** Adoption is a registration: the doc's edges into the edge index and its hash into the file-state record. Which candidate is adopted is the classifier's — a manifest shows only what was registered — and the cells construct a candidate of each verdict.

**I1 — `.jigc/state/` and everything under it: rebuilt — the adopted doc's file-state hash, recorded.** When: a candidate was adoptable. On a refusal: no — what a refusal may do there is `C2` (§4a).

> Adopting a foreign `adoptable` doc is a distinct action: **parse → conformance-gate → `index.absorb_doc` (so an adopted `adr`'s `supersedes` edge is indexed) → record the file-state hash**.

— `design/project-setup.md` → *The bounded slice — three net-new but tractable pieces*

**I2 — `.jigc/index/` and everything under it: rebuilt — the adopted doc's edges, indexed.** When: a candidate was adoptable. On a refusal: no — `C1`. The sentence is I1's.

**I3 — adoption moves and rewrites no file.** Not a line: it is why the row has no line for any file outside `.jigc/index/` and `.jigc/state/`, a candidate's own included.

> Adopt is **register-only — it never moves or rewrites a file**

— `design/project-setup.md` → *The bounded slice — three net-new but tractable pieces*

**I4 — it adopts nothing it has not schema-checked, and leaves the rest untouched.** Not a line: the rule a non-conformant candidate's cell is held to — routed, not registered.

> `jigc ingest` **adopts nothing it has not schema-checked** (closing the `baseline-adopt` hole) and **rewrites no prose** (detect-and-route, never auto-migrate). The non-conformant case routes to a human; the unmanaged case is left untouched.

— `design/project-setup.md` → *Idempotency & irreversibility — the acceptance obligation*

**What the docs do not say of `ingest`:** whether it commits — no sentence read says so, and the row has no commit line; whether it refuses at all inside a repository jigc is set up in — *"ingest already hard-requires a git repo"* is its only refusal named, and every prepared state is one.

---

## 2c. `jigc unmanage`

*Written 2026-10-15, with §2b.*

Owner doc: `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*, the item *Teardown / cleanup (G5)*. Its letter is `X`, for *the detach report* `design/command-output-contract.md` calls its output: `U` is `uninstall`'s.

**X1 — `.jigc/state/` and everything under it: rebuilt — the doc's baseline dropped from the file-state record.** When: the path is a managed doc. On a refusal: no — `C2`.

> **(a) un-manage a doc** — drop a single managed doc from the file-state record + its `edges.json` entries, leaving the file on disk (the inverse of `adopt`'s register-only mutation)

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

> un-manage touches only jigc's own index/state (asserted: the doc's bytes on disk are unchanged) and is idempotent (re-run on an already-unmanaged doc is a clean no-op, not an error)

— `design/project-setup.md` → *Flow 2 hardening — full ingest/adopt/cleanup (M21)*

**X2 — `.jigc/index/` and everything under it: rebuilt — the doc's edges dropped.** When: the path is a managed doc. On a refusal: no — `C1`. The sentences are X1's.

**What the docs do not say of `unmanage`, or say two ways:** whether it commits — **ruled on 2026-10-15: it never commits** (§6, entry 9), and the row has no commit line, as it had. `design/project-setup.md` says it *"touches only jigc's own index/state"*, both of which are gitignored, so there is nothing for a commit to hold; `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)* calls it one of *"the other top-level direct-store ops"* beside a verb that is *"task-less and self-committing"*, and → *`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)* says `migrate-corpus` *"unlike them … has **never committed its own writes**"* — `rename` and `unmanage`. §6, entry 9. The row has no commit line.

---

## 3. `jigc task finalize`

Owner doc: `design/finalize.md`.

**T1 — the canonical path of each doc staged in the task's working area: a regular file that is a copy of the staged doc — made, or written over the doc's own committed home.** When: always; a doc the task minted never lands on a file that stands at its destination. On a refusal: no (T7).

> - **Copy** (not move) the file from the working area to its canonical path.

— `design/finalize.md` → *4. Promote managed docs*

> **A promote lands a regular file at exactly its canonical path, or the transaction refuses before anything is written

— `design/finalize.md` → *4. Promote managed docs*

> A `created` doc over an occupied destination blocks with **`finalize.promote-clobber`**, keyed at the destination path, and nothing is committed; an `edited-from-base` doc re-promotes over its own home, which is the update path.

— `design/finalize.md` → *4. Promote managed docs*

**T2 — `.jigc/version`: written whole.** When: what stands there is jigc's own stamp. On a refusal: no (T7).

> the `.jigc/version` stamp, refreshed to the running build first — **where it is jigc's to write**

— `design/finalize.md` → *5. Stage*

**T3 — the index, at the paths of T1 and T2, at `.jigc/config` and `.jigc/.gitignore`, and at every recorded owner-artifact path: staged.** When: always. On a refusal: no (T7).

> This phase stages **jigc's own** managed artifacts into that existing index

— `design/finalize.md` → *5. Stage*

> on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),

— `design/finalize.md` → *5. Stage*

**`T3, the config` and `T3, the config's ignore file` — `.jigc/config` and `.jigc/.gitignore`: carried into the commit as they stand in the working tree, and written nothing.** When: the layer's first commit — `HEAD` holds nothing at the place before the command. On a refusal: no (T7). *Added 2026-10-19, by the first of the fixes after the review of `B7`'s part B (its first finding).* `T3`'s second sentence stages the layer *"on a first commit"*, and as data `T3` stages only at the places a line of the file layer wrote — none named the config, so this row read the sentence as granting nothing, where `M3, the config` read it as granting every commit. Both rows now read it one way: the layer carried as it stands (§7, *carried as it stands*), on its first commit (§7, *the place's first commit*); `T3`'s *at `.jigc/config` and `.jigc/.gitignore`* is staged where these carry it. Since M48 `setup`'s install commit lands the layer, so no prepared state is such a commit and the two lines excuse nothing in any cell; a pending delta at a later commit is §6, entry 19. The spelling `doc-only` replaces them, with `T3`.

> on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),

— `design/finalize.md` → *5. Stage*

> jigc's promotion (docs + first-commit config) — and finalize unions exactly those, never sweeping unstaged/untracked working-tree WIP

— `design/storage.md` → *The per-task working area (staging)*

**T4 — one commit on the checked-out branch, holding the index as it stood and what T3 staged — nothing that was neither staged nor a line's.** When: always. On a refusal: no (T7).

> The commit set is therefore **the git index** — the agent's staged code plus jigc's just-staged docs/config — committed whole

— `design/finalize.md` → *5. Stage*

> finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay out of the commit and are surfaced

— `design/finalize.md` → *5. Stage*

**T5 — `.jigc/index/` and `.jigc/state/`: written — the edge index's stamp, the file-state hashes; the stamp's own file, `.jigc/index/edges.json`, written or taken away.** When: the commit landed. On a refusal: no — what a refusal may do there is `C1` and `C2` (§4a; §6, entry 1, as ruled).

> - **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches). The next read rebuilds from the new HEAD.

— `design/finalize.md` → *7. Post-commit (best-effort)*

> - **Update `file-state` hashes** for every committed managed doc.

— `design/finalize.md` → *7. Post-commit (best-effort)*

  The stamp's file, and what its loss costs — the index is rebuilt by the read after the commit, never by the finalize:

> **`index/edges.json` — locked, not merged.** A decision, not an omission: the index is **stamp-rebuildable by construction** (a stamp mismatch rebuilds it from the committed `.md`s, per Derived caches above), so a lost delta costs one rebuild and never a wrong answer

— `design/storage.md` → *Concurrent writers*

> | **finalize commit** | phase 7 invalidates the stamp; next read rebuilds against the new HEAD ([finalize.md](finalize.md)) | best-effort; cache-stamp absorbs failure |

— `design/storage.md` → *Edge index lifecycle*

> only `finalize` writes them through (indirectly, via stamp invalidation → next read rebuilds against the new HEAD). This is what makes "delete `.jigc/`'s gitignored caches, rebuild from `.md`s, nothing lost" hold for the index too

— `design/storage.md` → *Edge index lifecycle*

  **Corrected 2026-10-18, by the fourth of the fixes after the review of `B7`'s part A (its first finding):** the edge index's half was *rebuilt* — a file made or written, none gone — on §7's reading of 2026-10-13, which no sentence backs: the docs leave the rebuild to the **next read**, and say of the cache whole that its loss costs one rebuild. Every door that invalidates takes the file away (§6, entry 16, *answered*). The half is carried by its own word, *invalidated* (§7): rebuilt, or the one stamped file, `.jigc/index/edges.json`, taken away; every other file under `.jigc/index/` as before, and the file-state half stays *rebuilt* — that record has *"no stamp and no rebuild path"* (`design/storage.md` → *Derived caches*).

**T6 — the task's working area, `.jigc/tasks/<task-id>/`: removed — jigc's own files; what jigc did not write there is moved aside to `.jigc/displaced/<task-id>/`, under the path it had.** When: the commit landed. On a refusal: no (T7).

> - **Keep what jigc did not write, then remove `.jigc/tasks/<task-id>/`.**

— `design/finalize.md` → *7. Post-commit (best-effort)*

> The removal's subject is jigc's own files only: the area's **complement** ([team-ready-state.md](team-ready-state.md) → The working area's two populations) is moved aside first, to `.jigc/displaced/<task-id>/` with its relative paths intact

— `design/finalize.md` → *7. Post-commit (best-effort)*

**T7 — a refusal leaves the working area and the canonical paths as they were.** This is the sentence T1 to T4 and T6 take their *on a refusal: no* from. It names two places; of the caches of T5 it says nothing.

> The transaction is **atomic up through `git commit`**: any failure before that point rolls back to "as if `finalize` was never called" — the working area is intact, canonical paths unchanged.

— `design/finalize.md` → *What `finalize` is*

**T8 — the rule for prose handed to jigc.** Not a line: it is what the plan's §2 holds a staged doc to once its working area is gone — T1's copy, in T4's commit, as a regular file. The commit doc is the exception the doc names.

> - The commit doc (`commit:<task-id>`) is **not** promoted — its sink is the git message rendered in phase 3, not a repo file

— `design/finalize.md` → *4. Promote managed docs*

**A second spelling, with its own commit line — the doc-only arm.** A task whose workflow includes `step:finalize-doc-only` commits its own paths and never the index; which task that is cannot be read off a repository's bytes, so it is a spelling of the leaf that a cell names. No cell of the acceptance is one. *[Not so — corrected 2026-10-19, by the fifth of the fixes after the review of `B7`'s part A: one is. `task-finalize-a-link-to-nothing-at-the-created-docs-home` runs a task of `report-inconsistency`, whose workflow includes `step:finalize-doc-only`, and has since the acceptance's cells were built (2026-10-13); it names the spelling since `B7`'s third commit ([the findings](byte-table-findings.md) → §15).]* **Ruled 2026-10-18**, option A: `B7` builds the switch by which a cell names a spelling of its leaf, in a commit of its own before the milestone rows, with cells for this spelling and for `M4`'s. **Built 2026-10-18**, by that task's third commit: the spelling `doc-only`, which takes `T2`, `T3` and `T4` out of force and puts `T9` in their place — and, since 2026-10-19, `T3, the config` and its ignore file's half with `T3`. *Corrected the same day, by the same commit:* this paragraph said the arm's row replaces `T3` and `T4`; read whole, *The doc-only arm* leaves the stamp out of its path set as well — `T2`'s write — and a pending config delta, `T3`'s second sentence. So all three.

**T9 — under the spelling `doc-only`: one commit on the checked-out branch, holding only the paths the row's other lines wrote — the task's promoted docs — and never the index as it stood; whatever else is staged stays staged; the stamp is not refreshed and no pending config delta rides.** When: the task's composed workflow includes `step:finalize-doc-only`, a condition no manifest shows, carried as the spelling a cell names. On a refusal: no (T7).

> commits those paths alone (`git add -- <paths>` then `git commit -F <message> -- <paths>`), never the index

— `design/finalize.md` → *The doc-only arm*

> Whatever else is staged in the checkout — by a code task open beside it, before or after this task was minted — **stays staged**

— `design/finalize.md` → *The doc-only arm*

> **Outside the path set by construction:** a pending `.jigc/config` delta (an ordinary code-less finalize commits it beside the doc — gap G8) and the `.jigc/version` stamp, which this arm does not refresh

— `design/finalize.md` → *The doc-only arm*

**What `T9` leaves out — a lead, not a line (added 2026-10-18, by the third of the fixes after the review of `B7`'s part A, its eighth finding).** *The doc-only arm* stages *"exactly the task's promoted docs … and its stageable recorded owner-artifacts"*; `T9` lets its commit hold only what the row's other lines wrote, and no line writes an owner-artifact, so a doc-only task with one would read *git's state moved outside its allowance* — the stricter direction. No cell can reach it with the shipped packs: an owner-artifact is an `owned-location` field of a doc in the task's working area, which only `completion-record` and `dogfood-record` declare, and a doc-only task holds one only by a create its workflow's gate allows — none of the four shipped doc-only workflows (`report-` and `triage-`, of `jigc-feedback` and of `inconsistency`) allows either — or by copying a committed one in, which no prepared state holds. Held in [the findings](byte-table-findings.md) → §15 as a lead.

**What the docs do not say of `task finalize`:** whether a refused finalize may leave `.jigc/index/` or `.jigc/state/` changed (§6, entry 1 — ruled on 2026-10-12, and carried by `C1` and `C2`); where the rendered message's temporary file stands (*6. Commit* says *a temp file, deleted after* and names no directory — §6, entry 6). **Not read, so the row may be short:** the amend arm, and `--dry-run`.

---

## 3a. `jigc task discard`

*Written 2026-10-13, as the first of the three commits of the plan's task `B4`: the row from its docs, before a cell of it runs. No binary was run to learn what the command writes, and no list of files was taken from the product's source.*

Owner docs: `design/write-commands.md` → *The verbs* (the paragraph *Abandoning a task*), `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*, with *The working area's two populations* and *The commit model*.

**The command is two commands by what it is handed.** An ordinary task is named by no committed record, and its discard removes a working area and nothing else (A9). A milestone's sub-task is named by one, and its discard settles the record first — one item's status, in a commit of the record alone — and then removes the area (A5 to A8). Which of the two a cell runs is the cell's construction, and no line carries it as a condition: A5 to A7 let an ordinary task's discard write a record that is not there, commit what was not written and advance a baseline — each lets the command do nothing where the doc says it does nothing, so a line without the condition allows no write the doc forbids.

**A1 — the task's working area, `.jigc/tasks/<task-id>/`: removed.** When: nothing in it is staged that no commit has a copy of, and nothing in it is a file jigc did not write — or under the consent (A4). On a refusal: no (A2, A3, A8).

> `discard` destroys a working area, and `.jigc/tasks/<id>/docs/*.md` is the only copy of every doc staged in it.

— `design/write-commands.md` → *The verbs*

**A2 — it refuses over a staged doc no commit has a copy of, and removes nothing.** Not a line: it is one of the two states A1's *when* names, and where *on a refusal: no* comes from. The second sentence is why an ordinary task's discard refuses from the moment the task exists.

> a working area staging a `docs/*.md` blocks with **`task-discard.staged-prose`** and removes nothing

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

> **The consequence is stated, not discovered: an ordinary discard needs the consent from the moment the task exists.**

— `design/write-commands.md` → *The verbs*

**A3 — it refuses over a file jigc did not write in the working area, naming every path.** Not a line: the other state of A1's *when*.

> That is **`task-discard.foreign-bytes`**, over jigc's own writer registry's complement ([team-ready-state.md](team-ready-state.md) → The working area's two populations), naming every path and routing at *move it out / delete it / consent*.

— `design/write-commands.md` → *The verbs*

**A4 — the consent is `--force`, for both, and what it takes is named.** A cell under the consent spelling is exempt from the plant's rule only where the command's output names the planted path; and a doc staged in the area that goes under the consent is held to the same: the output names it.

> `--force` is the single consent for both, inert where nothing is staged and nothing is foreign

— `design/write-commands.md` → *The verbs*

> What a door *does* with it is that door's own disposition — refuse without a consent, narrate under one, or move it aside

— `design/team-ready-state.md` → *The working area's two populations — jigc's writer set, and its complement (M52)*

**A5 — the milestone's committed record, `docs/milestone-records/<slug>.md`: written — one item's status flipped, and nothing else of it.** When: the task is a sub-task a committed record names. On a refusal: no (A8). The line as data says *written whole*: the vocabulary has no word for *one leaf of one item, and every other byte as it was*, so the line lets more than its sentence does — a stated shortfall of the row, not a reading of the doc. The place is `M5`'s, from the same sentence of `design/storage.md`. **[Narrowed 2026-10-18, after the review of `B7`'s part A (its fifth finding): the line read `docs/milestone-records/*` — any milestone's record — where the first sentence names one, the record that holds the sub-task's item. It reads `<docs-root>/milestone-records/<its-milestone>.md`: the record of the one milestone whose committed record names the task, on the fourth sentence — a task no record names, or two do, reaches no record — under the docs root the cascade resolves, on the third (§7).]**

> The item's `status` flips to `discarded` through the byte-stable item-leaf splice; **the header stays `active` and every sibling item is byte-untouched**

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

> The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load

— `design/storage.md` → *Config layout*

> The owning milestone is resolved from **what the committed record names**, never from the gitignored `.jigc/` cache

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

**A6 — one commit on the checked-out branch, holding the record of A5 and nothing else: not the index as it stood.** When: the task is a sub-task a committed record names. On a refusal: no (A8).

> it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

**A7 — `.jigc/state/`: written — the record's baseline, advanced to what the commit landed.** When: the task is a sub-task a committed record names, and the commit landed. On a refusal: no — what a refusal may do there is `C2` (§4a). The first sentence is A6's second, which names *the baseline advance*; the second says where a baseline stands.

> the hash jigc recorded at its last landed record write — lives in `.jigc/state/`, which no clone carries

— `design/team-ready-state.md` → *Engine capability 1 (write) — `set: on-transition` machine-maintained leaves*

**A8 — it settles before it destroys, and a rejected commit leaves both as they were.** Not a line: it is where A5 to A7 take their *on a refusal: no* from, and it is the order a cell's refusal is read against — a refusal leaves the record **and** the working area.

> The record commit runs **before** the working area is removed, so a rejected `pre-commit` hook leaves the record *and* the task exactly as the frame says

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

**A9 — an ordinary task's discard writes no record and lands no commit.** Not a line: the condition A5 to A7 carry in words.

> An ordinary task is named by no record, and a dev-only project (no methodology pack) resolves no `milestone-record` doctype at all: both discard exactly as before, writing no record and landing no commit.

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

**A10 — a sub-task nobody has re-entered stages nothing, and its discard needs no consent.** Not a line: it is the one construction in which the command acts without its consent flag, which is what the family's base cell is built from.

> `milestone add-task` stages nothing at mint, so settling a never-re-entered sub-task needs no consent

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

**What the docs do not say of `task discard`:** whether a discard that exits 0 writes under `.jigc/index/` — the record it rewrites is a managed doc, and `task finalize`'s own doc declares the edge index's stamp invalidated after its commit (`T5`) while this command's says nothing of it (§6, entry 7); what a rejected record commit leaves of the message it rendered (§6, entry 6, which this command's doc does not answer either: *the survivable frame on a rejecting hook* is named and not described); whether the milestone's `tasks.json` cache is rewritten when one item settles (the ruling's line `C3` covers a refusal, and the row has no line for exit 0 — §6, entry 1, *what the ruling leaves open*, (i)). **Declared, and not a line here:** a settled sub-task's worktree is not this command's to remove — the row has no line for a registration, so one that went would be red; and a sub-task the record has already settled is refused (*The terminal is terminal*), which is a state a cell constructs.

---

## 3b. The doc family — `jigc doc create`, `doc author`, `doc add-item`, `doc remove-item`, `doc retitle-item`, `doc rename`, `doc set-field`, `doc set-slot`

*Written 2026-10-18, as the first commit of the doc family's own (the plan's task `B6`, after the plant in the place of a file that stands and the plant of a file simply deleted, each with its fixes): the eight rows from their docs, before a cell of any of them runs. No binary was run to learn what a command writes, and no list of files was taken from the product's source.*

Owner doc: `design/write-commands.md` — *The verbs* with its sections on `doc retitle-item`, `doc rename` and `doc set-field --unset`, *Every write resolves its address before it moves bytes (M47)*, *Staging and the transaction model*, *Instance provisioning* with *The create-gate*, and *The write-time `--task`-scoped barrier (M8)* with its paragraph *Editing a base-committed doc — copy-on-first-touch (M8)*; with `design/reconciliation.md` → *Baseline adoption* and *Detection timing*, `design/storage.md` → *The per-task working area (staging)* and *Concurrent writers*, and `design/finalize.md` → *4. Promote managed docs*, its *Declared bounds*.

**The family in one sentence.** A `doc` write stages: it writes the task's working area and, where it copies a committed doc in, that doc's baseline under `.jigc/state/` — and nothing else. **Only `finalize` produces committed changes**, so no line of these rows lets a write change a doc's home, git's index or a commit, and a change there is red.

**The two parts the family's cells are built in**, by the docs the leaves share — the split `design/reconciliation.md` makes itself when it names the copy-in's two doors, *"the six edit leaves on their first touch, and `doc create` / `doc author` over an existing doc"*:

- **the six edit leaves — `doc add-item`, `remove-item`, `retitle-item`, `rename`, `set-field`, `set-slot`**: the per-leaf sections of *The verbs*, *Every write resolves its address before it moves bytes*, and the copy-on-first-touch paragraph — each writes one leaf, item or heading of a doc the task stages, copying a committed doc in on its first touch;
- **the two minting doors — `doc create` and `doc author`**: *Instance provisioning*, *The create-gate*, the shared title pre-check and the four-way write over a committed doc of *The verbs*, and the batch's atomic payload — each mints a doc into the task, or copies one in for update where it is already a doc.

The rows are one row eight times: W1 to W3 with `C1` to `C3` (§4a), and `doc rename`'s has W4 as well. **Their letter is `W`**, for *write*: no line of the table is lettered `W` (the plan's plant **W** is a worktree, and no line of these rows names one). **No row names a consent**: no `doc` verb takes one. **The home of every row is `committed-singletons`** — the smallest prepared state in which W2's write happens and W3's place stands, since the copy-in needs a committed managed doc and `fresh` holds none; W1's write happens there too.

**W1 — the task's working area, `.jigc/tasks/<task-id>/` and everything under it: rebuilt — the staged doc made or written whole, its provenance, the roles it is bound to; nothing of it taken away.** When: always. On a refusal: no (W5). No sentence read lets a file of the area go — but `doc rename`'s, which is W4 — so the word is *rebuilt* (§7), which lets a file be made or written and none be gone.

> There is no separate proposal object: every write lands in the current task's working area immediately (no `begin`), and "propose / review / apply" is just `write / validate / finalize` over that area.

— `design/write-commands.md` → *Staging and the transaction model*

> **Where a write may be typed (the rc.24 fix pass).** Every write verb stages into the task's working area, which lives under the main checkout from every cwd

— `design/write-commands.md` → *The write-time `--task`-scoped barrier (M8)*

**W2 — `.jigc/state/` and everything under it: rebuilt — the baseline of a committed doc the write copies in, where the record holds none, and the lock its save takes.** When: the write copies a committed doc in — its first touch of the doc, or a `create` or `author` over a doc that already exists at its home; a condition no manifest shows, so a word of the line only, and it lets the command do less. On a refusal: no (W5); what a refusal may do there is `C2` (§4a).

> **The copy-in also records the doc's `file-state` baseline when it has none (the rc.24 fix pass)**

— `design/write-commands.md` → *The write-time `--task`-scoped barrier (M8)*

> A write verb that stages a committed doc — the **copy-in**: the six edit leaves on their first touch, and `doc create` / `doc author` over an existing doc — reads the file at its home, and that read records the doc's baseline when the record holds none

— `design/reconciliation.md` → *Baseline adoption*

> `engine::state::with_save_lock` runs the whole critical section under an advisory lock — taken on a **stable `<filename>.lock` sibling**

— `design/storage.md` → *Concurrent writers*

**W3 — the home of a committed doc the write copies in: read, and nothing written there.** When: the write copies a committed doc in. On a refusal: no (W5). It lets no difference happen at the home and no commit or index hold one — W6 is why — and it names the place the copy is read from, which is what a plant put in a doc's place is aimed at: a file of the user's, a link, a directory where the doc's file stood, and the doc's file edited or deleted on either side of the task's first write. A read through a link is declared; nothing is declared written through one. A second write to a doc the task stages reads the staged copy, so the home is read by the first write alone.

> The first write to such a doc **copies the committed body into the sub-area** (`copy_in` → `edited-from-base`), then splices

— `design/write-commands.md` → *The write-time `--task`-scoped barrier (M8)*

> (2) **Reads still follow a link at a home** — `jigc doc show` serves a linked doc and the copy-in reads one

— `design/finalize.md` → *4. Promote managed docs*

> a second write to an already-staged doc reads the staged copy and does not look at the home again.

— `design/reconciliation.md` → *Detection timing*

**W4 — the staged body of a doc the task minted, under the name a re-slug leaves: re-slugged — `doc rename`'s alone.** When: the doc was never committed, and `--to` mints another slug. On a refusal: no (W5). The body under its new name is W1's. **Re-slugged, not removed** (2026-10-18, after the family's review, its second finding): the line lets a staged doc go only where the same task, its base pin standing, gained a doc with the gone one's lines but its `# H1` — the sentence below moves *"the staged body"*, and lets no other doc of the task, nor any doc of another task, go.

> `--to` rewrites the `# H1` **and** moves the identity: the staged body, its `docs/provenance.json` entry (re-keyed, its recorded provenance intact — the by-task-id join classifies by it), every `roles.json` binding, every **staged doc's** `ref` fields, and a migration task's recorded `slug-override`.

— `design/write-commands.md` → *`jigc doc rename` — the in-task title change, split on committed-store identity (M48)*

**W5 — a refusal stages nothing and records nothing.** Not a line: it is where W1 to W4 take their *on a refusal: no* from, read off sentences that each say it for one way a `doc` write refuses — an address it cannot resolve, a lock it cannot take, a title or a payload it refuses, a create-only entry over an occupied home. Every one of them speaks of a refusal that comes **before** anything is copied in; what a refusal that comes after the copy-in leaves is §6, entry 15, and the first sentence below is about an **address**, not about what the home holds (2026-10-18, after the family's review). **Ruled 2026-10-18** (§6, entries 14 and 15): a write refuses before its copy-in where the home's bytes do not parse as the doctype, and every `doc` write that refuses leaves the task's area as it found it — so the rule holds of every refusal, before the copy-in or after it.

> **A write verb resolves its addressed leaf against the declaring schema *before touching bytes*** — an undeclared address is **rejected, never written**.

— `design/write-commands.md` → *Every write resolves its address before it moves bytes (M47)*

> **At the copy-in door a timed-out save fails the write, before anything is staged**

— `design/storage.md` → *Concurrent writers*

> Both minting doors — `jigc doc create` and `jigc doc author` — now share **one pre-check**, running **before either persists**, so the whole payload is rejected with nothing staged and nothing to roll back.

— `design/write-commands.md` → *The verbs*

> a corrupt payload is rejected whole (atomic), never half-applied.

— `design/write-commands.md` → *The verbs*

> the create is refused **before anything is copied in**, so nothing is staged and no role is bound

— `design/write-commands.md` → *The create-gate*

**W6 — only `finalize` produces committed changes.** Not a line: it is why no row of the family names a doc's home as written, git's index or a commit.

> Only `finalize` produces committed changes; the working area itself is never committed

— `design/storage.md` → *The per-task working area (staging)*

**What the docs do not say of the family:** whether a `doc` write may rebuild the edge index — `design/storage.md` → *Edge index lifecycle* rebuilds it at the *first read after a stamp mismatch* and names its five sites, none a `doc` write, and no sentence read says whether a write reads it; no line is written for it, and a cell that finds one is red and an entry of §6, as `task discard`'s is (§6, entry 7). What a `doc` write does where the home it copies in from holds no doc of its type — a file of the user's, a link to one, a link to nothing, a directory, nothing at all — is said for the minting doors over a link to nothing and a directory (`design/finalize.md` → *4. Promote managed docs*, *Declared bounds*, (4): such a create *"still mints fresh and is refused at the committing door, not at the create"*), and for no other leaf or shape; the cells say where they meet it. *Corrected 2026-10-18, after the family's review:* where the home holds a file that is no doc of its type, the docs do say — two things — and §6, entries 14 and 15, quote both sides.

*Corrected 2026-10-18, by the fourth of the fixes after the family's review (its eighth finding):* **the doc's file deleted is no free home, and the minting doors' cells over it are not listed by extending (4).** (4) speaks of *"an entry with no doc body"*; a committed doc whose file was deleted is no entry, and git still holds it. The docs say what the create does there in a paragraph of their own — it reads the home as free and mints, and the committing door refuses:

> Every probe that asks the disk reads that home as free — the create included, so the doc is `created` — and landing it replaced what git holds under its own path

— `design/finalize.md` → *4. Promote managed docs*

> A `created` doc whose home is free on disk and held by git blocks with the same `finalize.promote-clobber`, keyed at the same destination path

— `design/finalize.md` → *4. Promote managed docs*

and the key's own record names the path — `design/command-output-contract.md` → *The membership test*, the `finalize.*` table's `promote-clobber` row:

  > or, with nothing on disk there, **a path git still holds** — a committed doc deleted from the worktree, or a file staged and taken out of it — which the commit, or the promote's own `git add`, would have replaced

So `doc-create-the-vision-the-doc-deleted-before-the-first-write` and `doc-author-…` are listed *acts*, as they were, on these sentences. No listing or classification moves.

**What a line cannot say, and the cells cannot see — `W1` and `W4` over another task's area** (2026-10-18, the same fix, its seventh finding). The quoted sentences speak of *"the current task's working area"*, and `design/write-commands.md` → *The write-time `--task`-scoped barrier (M8)* adds that *"a sub-agent physically cannot stage into a sibling's area"*. A line names a place with one component left open for a task's id (`.jigc/tasks/*`, §7), since one row serves every task its cells name — so `W1` lets a `doc` write rebuild a file of **any** task's area, and `W4` names any task's staged docs. **What the frame still sees:** a staged doc of another task taken away is *lost* whatever a line says — `W4`'s word lets a staged doc go only where its own task gained its re-slugged body (the frame's rule for prose handed to jigc, as the first of these fixes left it) — and a file of the user's there is a plant, lost if it goes. **What it cannot see:** another task's staged doc **rewritten**, which `W1` lets go. One cell stands there now — `doc-add-a-milestone-beside-a-sibling-task-that-stages-the-roadmap`, a first touch of the roadmap with a second live task holding a staged roadmap of its own — and holds on this tree; a regression that rewrote the sibling's copy would hold there too. Narrowing the line to the command's own task is a word the vocabulary does not have (a place keyed on the task the command names), and no finding asks for it yet. **[Said since 2026-10-18, by the task `B7`'s second commit: a line names the acting unit's own area — a place's component `<unit>` stands for the task the command names, by `--task <id>` or else the single task that stands, the very resolution the sentence's section gives (*"explicit `--task <id>` wins; else the single active task"*) — so `W1` is `.jigc/tasks/<unit>/` and `W4` `.jigc/tasks/<unit>/docs/*`, and a sibling's staged doc rewritten is *written outside its allowance*. The cell holds, as it did. §7 has the word.]**

---

## 4. `jigc milestone finalize`

Owner doc: `design/finalize.md` → *`fan-out` finalize*, with `design/team-ready-state.md` and `design/storage.md`.

**M1 — `.jigc/milestones/<id>/merged/`: cleared and written again.** When: always. On a refusal: **yes** — the sentence says so itself.

> `materialize` writes only into the gitignored `.jigc/milestones/<id>/merged/`, which it **clears and rebuilds every call**, so blocking after it still writes nothing durable

— `design/finalize.md` → *2. Validate*

**M2 — the canonical path of each doc of the merged set: a regular file that is a copy of it — as T1.** When: always. On a refusal: no (M8).

> The promote plan is one planner for both modes, and it carries the task door's clobber guard: a sub-task's `created` doc never promotes over a file already at its destination, and the refusal is taken before either mode lays down a commit

— `design/finalize.md` → *`fan-out` finalize*

**M3 — one commit on the checked-out branch, holding what each sub-task staged in its own worktree, the docs of M2, the first commit's config and the record of M5 — and nothing of the main checkout's index or files.** When: always (`finalize.fan-out.squash: true`, the default). On a refusal: no (M8).

> The CLI folds the N worktrees' staged code-sets into one combined tree **off-line** (a throwaway index), overlays the promoted docs + first-commit config, and commits that tree from a **clean dedicated worktree**

— `design/finalize.md` → *`fan-out` finalize*

> The main checkout is never mutated until that clean fast-forward, so a blocked combine or a hook rejection lands nothing and leaves unrelated main-checkout WIP intact (WIP-safe).

— `design/finalize.md` → *`fan-out` finalize*

The worktrees are the sub-tasks' own, at the place this sentence names — `.jigc/worktrees/<sub-task>`, a sub-task the milestone's record names — and no other checkout's: a task's the record does not name, or a user's own linked worktree, is none of them. `M4` quotes it too. (Added 2026-10-18, after the review of `B7`'s part A: the commit line had held what *every* other checkout staged.)

> a sub-agent's **code** lives in its own git worktree (`.jigc/worktrees/<sub-id>`, `git add`ed there)

— `design/finalize.md` → *`fan-out` finalize*

**`M3, the config` — the project config layer: carried into the aggregate commit as it stands in the working tree, and written nothing.** When: always, under both spellings — `squash: false` replaces `M3` and not this line, whose aggregate *"ride[s] the parent's aggregate commit"* too. On a refusal: no (M8). *Added 2026-10-19, with the milestone family's rows (the task `B7`'s second part), for the lead of 2026-10-18 that a pending `.jigc/config` delta rides the aggregate:* `M3`'s own first sentence has the boundary overlay *"the promoted docs + first-commit config"*, and the line as data held nothing of the config, so a boundary that carried a pending delta read *git's state moved* for what its sentence declares. The config is the layer `T3` names, and a pending delta is what an ordinary finalize commits — the doc-only arm leaves it out *"by construction"* where *"an ordinary code-less finalize commits it beside the doc"*. Carried by a word of its own (§7, *carried as it stands*): a commit may hold at the place the bytes the working tree held there before the command, and the command writes nothing there. **[Narrowed 2026-10-19, by the first of the fixes after the review of `B7`'s part B (its first finding): the line held under any `HEAD` — so a file of the user's nobody staged under `.jigc/config`, which a landed boundary sweeps into its commit at exit 0 (driven on this tree's debug build and on the previous release's, the same), held. Its sentences say less. The first overlays *"first-commit config"*, the second stages the layer *"on a first commit"*; the third is the doc-only arm's, speaks of `task finalize`, and calls the commit of a pending delta a *gap* — G8 — and it is one side of a question the docs answer two ways (§6, entry 19), not a ground for this line: it is struck from the line's quotes, and stands below for the record. *"The config is the layer `T3` names, and a pending delta is what an ordinary finalize commits"* above is withdrawn with it. The line is in force on the layer's first commit alone (§7, *the place's first commit*), as `T3, the config` is at the task's door; `setup`'s install commit lands the layer since M48, so it excuses nothing in any cell, and the cell of a pending delta is red and marked with entry 19. When, above: the layer's first commit, under both spellings.]**

> overlays the promoted docs + first-commit config

— `design/finalize.md` → *`fan-out` finalize*

> on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),

— `design/finalize.md` → *5. Stage*

*Struck as a ground of the line, 2026-10-19 — one side of §6, entry 19:*

> **Outside the path set by construction:** a pending `.jigc/config` delta (an ordinary code-less finalize commits it beside the doc — gap G8) and the `.jigc/version` stamp, which this arm does not refresh

— `design/finalize.md` → *The doc-only arm*

**M4 — a second spelling: under `finalize.fan-out.squash: false`, one commit a sub-task that staged code, then the aggregate.** A cascade knob, so a spelling a cell names; no cell of the acceptance sets it. **Ruled 2026-10-18**, option A: the spelling switch `B7` builds names it, with cells (§3, the doc-only arm, the same ruling). **Built 2026-10-18**, by that task's third commit: the spelling `squash: false`, which takes `M3` out of force and puts `M4` in its place — a commit line holding what the sub-tasks' worktrees staged and what the row's other lines wrote, as `M3`'s does, now over the commits of a chain. ~~*What it cannot say*: that the chain is in task-id order, and that each sub-task's commit holds its own worktree's code and no other's — a line holds what the commits the command made hold together.~~ The second sentence is added with it. **[Said 2026-10-18, by the third of the fixes after the review of `B7`'s part A (its fourth finding): `M4` held exactly what `M3` holds, so the switch changed no verdict and a product that ignored the knob — one combined commit — kept every cell of the spelling green. `M4` is carried by a word of its own (§7, *a chain*), read off its first two sentences: the commits the boundary made are, oldest first, one for each sub-task whose worktree at `.jigc/worktrees/<sub-task>` staged code — staged against that worktree's own `HEAD` — in the order of the sub-task ids, each holding what that one worktree staged and nothing else, then one more, the aggregate, holding only what the row's other lines wrote; *"A sub-task that staged no code contributes no commit"*, the same paragraph. A single combined commit, a chain out of order, a sub-task's commit holding a sibling's code, a chain with no aggregate, code in the aggregate and no commit at all are each *git's state moved outside its allowance*. *Not read*: the commits' messages — a manifest holds none — so that each sub-task's commit carries its own authored doc is not held.]**

> **`squash: false` — one commit per code-carrying sub-task in task-id order, then the parent's aggregate commit.**

— `design/finalize.md` → *`fan-out` finalize*

> Each per-sub-task commit applies **that one worktree's** staged code onto the checkout and `git commit -F`s the sub-task's own authored `commit` doc

— `design/finalize.md` → *`fan-out` finalize*

**M5 — the milestone's committed record, `docs/milestone-records/<slug>.md`: written — its status flipped, inside M3's commit.** When: always. On a refusal: no (M8). The second sentence is where the place comes from — added on 2026-10-13, when the line became data and had to name one; the home is under the docs root as the prepared states have it. **[Narrowed 2026-10-18, after the review of `B7`'s part A (its fifth finding): the line read `docs/milestone-records/*` — any milestone's record — where the first sentence names the milestone's own. It reads `<docs-root>/milestone-records/<unit>.md`: the record of the milestone the command finalizes, under the docs root the cascade resolves — the third sentence — and no longer the docs root the prepared states happen to have (§7).]**

> the record's `status`-flip write **folds into the existing join commit** (path-added alongside the promoted docs), not a second commit

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

> The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load

— `design/storage.md` → *Config layout*

**M6 — `.jigc/worktrees/<sub-task-id>/`, each sub-task's working area under `.jigc/tasks/`, and `.jigc/milestones/<id>/`: removed — and what jigc did not write in an area is moved aside to `.jigc/displaced/<unit-id>/`.** When: the boundary landed. On a refusal: no (M8).

> ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)

— `design/storage.md` → *Repository layout & state location*

> Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`

— `design/storage.md` → *The per-task working area (staging)*

**`M6, the worktrees moved aside` — what git neither ignores nor holds in a landed sub-task's worktree, `.jigc/worktrees/<sub-task-id>/` — its unstaged edits and its untracked files: moved aside to `.jigc/displaced/<sub-task-id>/`.** When: the boundary landed. On a refusal: no (M8). *Added 2026-10-19, with the first commit of the movers family's task (`B8`), for the human's ruling of that day on §6, entry 18:* the ruling is the line's sentence until the design docs say it — their change is a finding of the run's ledger — as `U9`'s and `S16`'s are. The first sentence names the place, the second what may happen there:

> ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)

— `design/storage.md` → *Repository layout & state location*

> **Before removing a landed sub-task's worktree, the boundary moves its unstaged and untracked files that git does not ignore into `.jigc/displaced/<sub-task-id>/` and names them**

— `DECISIONS.md` → *2026-10-19 — The human's three rulings of the day, recorded: a landed boundary moves a sub-task worktree's unstaged and untracked files that git does not ignore aside, where it took them; `milestone provision` skips a sub-task the record has settled; and bound 2 is retired — a `doc` write over a part that holds an uncommitted hand edit refuses at the write; four marks leave their cells, one line is added, and no classification moves (the byte table, task B8, the first of its commits)*

**`M6, the milestone's area moved aside` — what jigc did not write in `.jigc/milestones/<id>/`, the join's `merged/` tree included: moved aside to `.jigc/displaced/<id>/`.** When: the boundary landed. On a refusal: no (M8). *Added 2026-10-19, with the cells of `milestone finalize`'s plain spelling (the task `B7`, its second part):* `M6`'s second sentence keeps the complement of **both** kinds of area — *"Both keep their complement"* — and the line as data moved aside the sub-tasks' areas' alone, so a file of the user's in the milestone's area, kept at `.jigc/displaced/`, read *written outside its allowance* for what the sentence declares. The sentence that names the milestone's area as the second subject:

> A landed `jigc milestone finalize` removes each sub-task's `tasks/<sub-id>/` *and* the milestone's own `milestones/<id>/`

— `design/storage.md` → *The per-task working area (staging)*

> Both keep their complement, to `.jigc/displaced/<unit-id>/<relative>`

— `design/storage.md` → *The per-task working area (staging)*

**M7 — git's registrations of worktrees: removed, at a path under `.jigc/worktrees/` and at no other; every other registration is as it was.** When: the boundary landed. On a refusal: no. The first sentence is U6's.

> So every other registration — live, `prunable` or locked; a human's, or jigc's own under another path (a moved repository's sub-task records name the old one, and are exactly what `git worktree repair` needs) — is left byte-identical

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**M8 — a refusal tears nothing down.** This is the sentence M2, M3, M5 and M6 take their *on a refusal: no* from, with M3's second.

> So **no aborted finalize arm tears down**: the whole working area — sub-task areas and worktrees alike — survives for the retry.

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**M9 — git's object store: objects are added, also on a refusal.** Not a line of the allowance: the table holds the store to one rule, that no object there before is absent after, and an object more is no difference. The sentence is here because a refused boundary leaves the store larger and a reader will ask.

> it creates a temp index and runs `read-tree` / `apply --cached` / `write-tree`, writing loose objects into the ODB

— `design/finalize.md` → *2. Validate*

**M10 — `.jigc/index/` and `.jigc/state/`: written, as T5 — the stamp's own file, `.jigc/index/edges.json`, written or taken away.** When: the boundary landed. On a refusal: no — what a refusal may do there is `C1` and `C2` (§4a; §6, entry 1, as ruled).

> 4. **Post-commit** runs once for the whole batch.

— `design/finalize.md` → *`fan-out` finalize*

  The post-commit it runs is `T5`'s, and the line quotes `T5`'s sentences of the edge index's stamp with it. **Corrected 2026-10-18, with `T5`** (§6, entry 16, *answered*): the edge index's half is *invalidated* (§7), where it was *rebuilt*; under both spellings, since `squash: false` replaces `M3` alone.

**M11 — the milestone's area: rebuilt — the cache re-derived where absent or stale.** As MP3. When: always. On a refusal: no (M8).

**M12 — the re-seed of the sub-tasks' areas: rebuilt where absent, for a sub-task the record has not settled.** As MA6. When: always. On a refusal: **yes** — as MA6, since the seventh of the fixes after the review of `B7`'s part B: the re-seed runs before the door's own guards, so what it made stands when a guard refuses; M8's *"tears nothing down"* is of the boundary's own teardown.

*Both added 2026-10-19, after the review of `B7`'s part B (its tenth finding): the fourth sentence, which the re-seed's lines quote at every door, names this door among the seven that pass through the re-seed, and the row lacked them. What they rebuild the boundary's own teardown (M6) removes where it lands, so in a cell they excuse nothing.* **[Corrected 2026-10-19, by the seventh of those fixes: M12 excuses the area the re-seed rebuilt where the boundary refuses — the cell `milestone-finalize-a-sub-tasks-area-gone-and-no-work-to-land`, which refuses `milestone.zero-contribution` with a sub-task's area taken away first. M11 still excuses nothing: no cell of the door takes the cache away.]**

> the cache is **re-derived from the committed record when absent or stale**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

**What the docs do not say of `milestone finalize`:** the caches on a refusal (§6, entry 1 — ruled on 2026-10-12, and carried by `C1` to `C3`). **Declared, and not a line here:** the two worktrees the boundary makes for itself and removes on every exit — the dedicated one it commits from and the detached one its `doc-code` check reads (*2. Validate*: *torn down on both exits*) — leave no difference between the two manifests where they are removed as declared; one left behind is a registration under `.jigc/worktrees/` that M7 does not cover on a refusal, and is red. The plan's §7 lists this command's commit as *not read for this plan*: it is read now, and M3 to M5 are what the docs declare it holds.

---

## 4b. The milestone family — `jigc milestone create`, `add-task`, `add-from-spec`, `provision`, `execute`, `join`, `discard`

*Written 2026-10-19, as the first commit of the milestone family's own (the plan's task `B7`, its second part, after the human's three rulings, the own-area word and the spelling switch of its first part, and the five fixes after their review): the seven rows from their docs, before a cell of any of them runs. No binary was run to learn what a command writes, and no list of files was taken from the product's source. `jigc milestone finalize`'s row is §4, and gains one line here (`M3, the config`); its cells are owed to this family too.*

Owner docs: `design/write-commands.md` → *Minting a milestone + its task list (M7)*; `design/team-ready-state.md` → *The commit model*, *Engine capability 1* and *2*, *The lifecycle*, *The terminal is terminal*, *What a fan-out worktree isolates*; `design/storage.md` → *Source of truth*, *Config layout*, *Repository layout & state location*; `design/finalize.md` → *Dirty-tree policy*, the carryover gate. **Each leaf names its unit as its grammar has it**: `create` none, since the milestone it mints is named by a title whose slug is its id — its places are where nothing stood (§7, *a place the command mints*); the other six the milestone, the first word after the leaf; and `add-task` and `add-from-spec` mint sub-tasks no argv word names by its id.

**The split the cells are built in, by the docs the leaves share** — three parts, as the plan's §8 has it:

1. **The record doors** — `create`, `add-task`, `add-from-spec`: each mints and lands one path-scoped commit of the record alone (`design/team-ready-state.md` → *The commit model*), and none touches a worktree.
2. **The fan-out doors** — `provision`, `execute`, `join`: they write no record and commit nothing; what they share is the re-seed (*Engine capability 2*), and `provision` the worktrees.
3. **The terminals** — `discard` and `milestone finalize`: each settles the record and tears the workbench down (*The lifecycle*; `design/finalize.md` → *`fan-out` finalize*), and the plain spelling of `finalize`'s cells is owed to this family.

### `jigc milestone create`

**MC1 — the milestone's area, `.jigc/milestones/<id>/`: made — its base pin, its empty task list, and the snapshot of what was staged.** When: always. On a refusal: no (MC5).

> opens its area with an empty task list, and **pins one shared base** (= HEAD) that every sub-task inherits

— `design/write-commands.md` → *Minting a milestone + its task list (M7)*

> snapshots the pre-task staged state — the staged set (path + blob) **and the staged-deletion set** — into the working area

— `design/finalize.md` → *Dirty-tree policy*

> both mints still refuse a pre-existing id before creating anything, which is exactly what makes removing what this call minted safe

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MC2 — the milestone's committed record, `<docs-root>/milestone-records/<id>.md`: made.** When: always. On a refusal: no (MC5). The second and third sentences are `M5`'s, for the home.

> **`create` / `add-task`** — a **separate** record-only path-scoped commit each.

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

> The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load

— `design/storage.md` → *Config layout*

**MC3 — one commit on the checked-out branch, holding the record of MC2 and nothing else.** When: always. On a refusal: no (MC5). `A6`'s sentence.

> it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MC4 — `.jigc/state/`: written — the record's baseline, advanced to what the commit landed.** When: the commit landed. On a refusal: no — `C2` (§4a). `A7`'s two sentences: a sub-task's discard has *"the family's whole discipline rather than a second one"*, and the baseline advance is a member of it.

> the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

> the hash jigc recorded at its last landed record write — lives in `.jigc/state/`, which no clone carries

— `design/team-ready-state.md` → *Engine capability 1 (write) — `set: on-transition` machine-maintained leaves*

**MC5 — a refusal leaves no residue: the record's pre-image restored, and the area the door minted unwound.** Not a line: it is where MC1 to MC4 take their *on a refusal: no*, for a refusal of the commit as for one before it.

> every door that writes the committed record and commits it captures the record's pre-image before writing and restores it — worktree *and* index — on any failure of that commit**, leaving no residue and re-baselining nothing

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> `create` removes the milestone area *it* created, `add-task` removes the sub-task area and restores the task-list cache to its captured pre-append bytes

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MC6 — it refuses where a record already owns the id, and where the record's home holds no regular file.** Not a line: the states a cell plants at MC2's place are refused, and the refusal's words are the docs'.

> `create` refuses when a record already owns the slug (`milestone.record-exists`)

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

### `jigc milestone add-task`

**MA1 — the sub-task's area, `.jigc/tasks/<sub-task-id>/`: made.** When: always. On a refusal: no (MC5, whose second sentence names this door's unwind). The third sentence is MC1's.

> mints a sub-task under the milestone (a task work-unit: id = slug from intent, pinned to the **milestone's** base, isolated `tasks/<sub>/` area) and appends it to the milestone's task list

— `design/write-commands.md` → *Minting a milestone + its task list (M7)*

> both mints still refuse a pre-existing id before creating anything, which is exactly what makes removing what this call minted safe

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MA2 — the milestone's area, `.jigc/milestones/<milestone-id>/`: rebuilt — the task list appended to, and the cache re-derived where absent or stale.** When: always. On a refusal: no (MC5). The first sentence is MA1's.

> mints a sub-task under the milestone (a task work-unit: id = slug from intent, pinned to the **milestone's** base, isolated `tasks/<sub>/` area) and appends it to the milestone's task list

— `design/write-commands.md` → *Minting a milestone + its task list (M7)*

> the cache is **re-derived from the committed record when absent or stale**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

**MA3 — the milestone's committed record, `<docs-root>/milestone-records/<milestone-id>.md`: merged into — one item appended, every line that stood staying, in order.** When: always. On a refusal: no (MC5). The last three sentences are MC2's.

> **`add-task` — append.** Appends one `tasks` item (`id`/`intent`/`status: active`) via the M16 `add-item` primitive. Ordered, byte-stable.

— `design/team-ready-state.md` → *Engine capability 1 (write) — `set: on-transition` machine-maintained leaves*

> **`create` / `add-task`** — a **separate** record-only path-scoped commit each.

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

> The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load

— `design/storage.md` → *Config layout*

**MA4 — one commit on the checked-out branch, holding the record of MA3 and nothing else.** When: always. On a refusal: no (MC5). MC3's sentence.

> it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MA5 — `.jigc/state/`: written — the record's baseline, advanced.** As MC4.

> the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

> the hash jigc recorded at its last landed record write — lives in `.jigc/state/`, which no clone carries

— `design/team-ready-state.md` → *Engine capability 1 (write) — `set: on-transition` machine-maintained leaves*

**MA6 — the working area of each sub-task the record names, `.jigc/tasks/<sub-task-id>/`: rebuilt where it is absent — the re-seed.** When: always. On a refusal: **yes** (since the seventh of the fixes after the review of `B7`'s part B; the line had said *no (MC5)*). Every operating door of the family passes through one re-seed, which rebuilds the milestone's cache — MA2 — and one area a sub-task, and skips an area that stands. **[Narrowed 2026-10-19, after the review of `B7`'s part B (its second finding): the line as data read `.jigc/tasks/<sub-task>` — any area of a sub-task the record names, standing or not — so a rewrite of a standing sub-task's staged prose or base pin held at every door that re-seeds, where the second sentence says the area *"is skipped untouched"*. It reads `.jigc/tasks/<re-seeded>`: an area where nothing stood, of a sub-task whose item the record has not settled — the second sentence quoted whole now, since it says the settled one is skipped too (§7). The same at MS6, MP4, ME2, MJ2, and at the two lines the terminals gain, M12 and MD8.]** **[Read on a refusal too since 2026-10-19, by the seventh of those fixes (the second fix's left-open): the third sentence has every operating door pass through the re-seed *"before their own guards"*, so a door that refuses at its own guard leaves the area the re-seed made, and the fourth names the refusal the rebuilt area is for — the zero-contribution refusal routed a fresh clone into a loop the area closes. MC5's unwind is of the area the door **minted**, on a failure of its record commit, and is no re-seed's. Searched for the other side: §4a's *"a task's working area is *a transient working copy*"* says why the area is no cache of the ruling on entry 1, which this line does not lean on; `design/finalize.md`'s *"the working area is intact"* (T7) is of the finalizing task's own area. Driven on both binaries: `jigc milestone finalize` over a milestone with no work to land and a sub-task's area taken away refuses `milestone.zero-contribution` and leaves the area rebuilt — base pin, intent and workflow. The same at MS6, MP4, ME2, MJ2, M12 and MD8; the cache's half keeps *no*, a refusal's cache being `C3`'s.]**

> So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

> A **loop**, and the one the [finalize](finalize.md) zero-contribution refusal routes a fresh clone into.

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

### `jigc milestone add-from-spec`

`add-task`'s row with the spec's criteria in place of one intent: MS1 to MS6 are MA1 to MA6, on the sentences that name this door. Its home is `vendored`, the one prepared state that holds a committed spec.

**MS1 — each sub-task's area, `.jigc/tasks/<sub-task-id>/`: made, one a criterion.** When: always. On a refusal: no (MC5; and MS7). **[As data since 2026-10-19, by the sixth of the fixes after the review of `B7`'s part B: `<each-minted>` — any sub-task the record gains, where MA1's `<minted>` is the one (§7).]**

> seeds the task list from a committed spec by **enumerating its repeatable `criteria` items** (the parse-items read path — `parse_sections` → the section's `items`, *not* the `jigc task bind` slice path) and minting one sub-task per criterion, the criterion's text as that sub-task's intent

— `design/write-commands.md` → *Minting a milestone + its task list (M7)*

> both mints still refuse a pre-existing id before creating anything, which is exactly what makes removing what this call minted safe

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MS2 — the milestone's area: rebuilt — its task list seeded, its cache re-derived.** As MA2.

> seeds the task list from a committed spec by **enumerating its repeatable `criteria` items** (the parse-items read path — `parse_sections` → the section's `items`, *not* the `jigc task bind` slice path) and minting one sub-task per criterion, the criterion's text as that sub-task's intent

— `design/write-commands.md` → *Minting a milestone + its task list (M7)*

> the cache is **re-derived from the committed record when absent or stale**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

**MS3 — the milestone's committed record: merged into — one item a criterion appended.** As MA3. The first sentence names the door among the five that write the record.

> the **five record-only doors** are `cli::rollback::ROLLBACK_POPULATIONS`' `milestone-record` row — the four milestone ops (`create`, `add-task`, `add-from-spec`, `discard`)

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> **`add-task` — append.** Appends one `tasks` item (`id`/`intent`/`status: active`) via the M16 `add-item` primitive. Ordered, byte-stable.

— `design/team-ready-state.md` → *Engine capability 1 (write) — `set: on-transition` machine-maintained leaves*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

> The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load

— `design/storage.md` → *Config layout*

**MS4 — commits on the checked-out branch, one a sub-task, each holding the record and nothing else.** When: always. On a refusal: no (MS7). **[Read whole since 2026-10-19, by the seventh of the fixes after the review of `B7`'s part B (its tenth finding): a commit before the last holds the record partway — every line the record held before the command, and no line it does not hold after it — under MS3's *merged into* (§7); the frame reads that commit's bytes. The cell `milestone-add-from-spec-a-spec-of-two-criteria` lands two record commits, and MS3 excuses both.]**

> It mints one sub-task per spec criterion and commits the record once per sub-task

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MS5 — `.jigc/state/`: written — the record's baseline, advanced.** As MC4.

> the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

> the hash jigc recorded at its last landed record write — lives in `.jigc/state/`, which no clone carries

— `design/team-ready-state.md` → *Engine capability 1 (write) — `set: on-transition` machine-maintained leaves*

**MS6 — the re-seed: rebuilt where absent.** As MA6.

> So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

**MS7 — a rejection of the k-th commit leaves the k−1 before it, and the workbench names exactly what the record names.** Not a line: it is where MS1 to MS6 take their *on a refusal* — a refusal may leave commits in history, which no cell of this family constructs (its home's spec has one criterion; the cell of two criteria, added by the seventh of the fixes after part B's review, lands both).

> Its atomicity is therefore the property that *is* reachable: **the workbench names exactly what the record names**

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

### `jigc milestone provision`

**MP1 — each sub-task's worktree, `.jigc/worktrees/<sub-task-id>/`: made — the tracked files of the milestone's base.** When: always. On a refusal: no (MP5). `M6`'s first sentence names the directory. **[Narrowed 2026-10-19, after the review of `B7`'s part B (its second finding): the line as data read `.jigc/worktrees/<sub-task>`, so a rewrite inside a live sub-task's worktree held, where MP5 has the door reuse it untouched. It reads `.jigc/worktrees/<unprovisioned>`: a sub-task's path at which no checkout stood (§7) — the third sentence. A leftover directory there that is no checkout the door clears before the add — the fourth sentence, MP5's — so the place stands for it, and the worktree made into it holds.]** **[Narrowed again 2026-10-19, by the seventh of those fixes (the sixth's left-open): what the line lets stand there is the pinned base's tree — its second sentence, *"populates from the pinned base's *tree*"*: a directory made, the checkout's own `.git` file, and a regular file whose path under the checkout and bytes the tree of the base the milestone's committed record pins holds (§7, *from the pinned base's tree*). It had read *rebuilt*, any bytes. And `<unprovisioned>` is of a sub-task the record has not settled — the stricter reading of §6, entry 20, open.]**

> ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)

— `design/storage.md` → *Repository layout & state location*

> **A provisioned worktree carries tracked files only.** `git worktree add --detach` populates from the pinned base's *tree*

— `design/team-ready-state.md` → *What a fan-out worktree isolates — and what it does not (M45)*

> a **registered** worktree is reused untouched and never probed at all

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

> The three doors standing where **no commit has carried those bytes** refuse — `provision` (it clears a leftover before `git worktree add`), `discard` (this teardown), `uninstall` (it takes `.jigc/` and everything under it)

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**MP2 — git's registration of each of those worktrees: added, at its path, and at no other.** When: always. On a refusal: no (MP5). **[Read whole since 2026-10-19, by the sixth of the fixes after the review of `B7`'s part B (its ninth finding): the checkout it registers stands as its sentence's *`git worktree add --detach`* leaves one at *the pinned base* — its `HEAD` detached at the base the milestone's committed record pins, its index that commit's tree. The line had excused a new checkout by its path alone (§7).]** **[At `<unprovisioned>` since 2026-10-19, by the seventh of those fixes: a place no checkout stood, of a sub-task the record has not settled — `MP1`'s, under §6, entry 20.]**

> **A provisioned worktree carries tracked files only.** `git worktree add --detach` populates from the pinned base's *tree*

— `design/team-ready-state.md` → *What a fan-out worktree isolates — and what it does not (M45)*

**MP3 — the milestone's area: rebuilt — the cache re-derived where absent or stale.** When: always. On a refusal: no (MP5). MA2's second sentence and MA6's third.

> the cache is **re-derived from the committed record when absent or stale**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

**MP4 — the re-seed of the sub-tasks' areas: rebuilt where absent.** As MA6.

> So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

**MP5 — it refuses over what stands at a sub-task's worktree path that it cannot prove disposable, and a registered worktree it reuses untouched.** Not a line: the state a cell plants at MP1's place is refused, and a re-run over a provision that landed adds nothing.

> The three doors standing where **no commit has carried those bytes** refuse — `provision` (it clears a leftover before `git worktree add`), `discard` (this teardown), `uninstall` (it takes `.jigc/` and everything under it)

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

### `jigc milestone execute`

**ME1 — the milestone's area: rebuilt — the cache re-derived where absent or stale.** As MP3, on its first and second sentences below.

**ME2 — the re-seed of the sub-tasks' areas.** As MA6, on the last three. Nothing else: the door composes the fan-out walk and mints nothing — ME3, no line.

> the cache is **re-derived from the committed record when absent or stale**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

**ME3 — it mints nothing.** Not a line.

> **minting nothing**

— `design/write-commands.md` → *Minting a milestone + its task list (M7)*

### `jigc milestone join`

**MJ1 — the milestone's area: rebuilt — the cache re-derived where absent or stale.** As MP3, on its first and second sentences below.

**MJ2 — the re-seed of the sub-tasks' areas.** As MA6, on the last three. Nothing else: the join is a preview, and its overlay is held in memory — MJ3, no line.

> the cache is **re-derived from the committed record when absent or stale**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

**MJ3 — it commits nothing, and its merge is never persisted.** Not a line.

> The overlay is derived in-memory for the run, never persisted with task deltas in it

— `design/storage.md` → *The by-task-id join (M7)*

**MJ4 — `.jigc/index/`: rebuilt — the committed edge index the join's ref walk reads, made where its file is absent or written where its stamp is stale.** When: always. On a refusal: no — what a refusal may do there is `C1` (§4a). *Rebuilt*, not *invalidated* (§7): a read makes or rewrites the cache and takes nothing of it away; the stamp's file gone is a finalize's alone (`M10`). The overlay MJ3 names is held in memory, and the index this line writes has no task delta in it — the rebuild is of the committed `.md`s.

> the join runs the forward-ref check **once per sub-area**, resolving that area's outgoing edges against `committed ∪ that one area`

— `design/storage.md` → *The by-task-id join (M7)*

> | **committed rebuild** | first read after a stamp mismatch (branch switch, pull, rebase) — rebuild from committed `.md`s | atomic; persists in `.jigc/index/` with the new stamp |

— `design/storage.md` → *Edge index lifecycle*

> only `finalize` writes them through (indirectly, via stamp invalidation → next read rebuilds against the new HEAD). This is what makes "delete `.jigc/`'s gitignored caches, rebuild from `.md`s, nothing lost" hold for the index too

— `design/storage.md` → *Edge index lifecycle*

  *Added 2026-10-19, by the fourth of the fixes after the review of `B7`'s part B (its fifth finding).* Driven on both binaries (this tree's debug build and the previous release's rebuild, `accf3996…ab5d`): with `.jigc/index/edges.json` moved away, `jigc milestone join` exits 0 and writes it again, beside its lock. No line covered it, and no cell met it: every cell of the door re-enters its sub-tasks first, and that re-entry builds the index. The row as written said *"Nothing else"*. The other doors of the family were driven the same way: `create`, `add-task`, `add-from-spec`, `provision`, `execute` and `discard` leave an absent index absent, and `finalize` rebuilds it and then takes it away, which `M10` already carries. So `join` is the one door that needed the line. The sentences above say the read that finds the file gone or stale rebuilds it, and none says a join leaves the index as it stood: *"commits nothing"* (`design/storage.md`, `design/validation.md`) is about commits, and MJ3's sentence is about the overlay.

### `jigc milestone discard`

**MD1 — the milestone's committed record: written — the header and every item not joined flipped to `discarded`.** When: always. On a refusal: no (MD5). The line as data says *written whole*, as `A5`'s does: the vocabulary has no word for *these leaves of the record, and every other byte as it was*. The second and third sentences are `M5`'s.

> Every **non-joined** item flips to `discarded`; the header flips to `discarded`.

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

> The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load

— `design/storage.md` → *Config layout*

**MD2 — one commit on the checked-out branch, holding the record of MD1 and nothing else.** When: always. On a refusal: no (MD5). The first sentence names the door among the five record-only doors; the second is MC3's.

> the **five record-only doors** are `cli::rollback::ROLLBACK_POPULATIONS`' `milestone-record` row — the four milestone ops (`create`, `add-task`, `add-from-spec`, `discard`)

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

**MD3 — `.jigc/state/`: written — the record's baseline, advanced.** As MC4.

> the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

> the hash jigc recorded at its last landed record write — lives in `.jigc/state/`, which no clone carries

— `design/team-ready-state.md` → *Engine capability 1 (write) — `set: on-transition` machine-maintained leaves*

**MD4 — the workbench: removed — the milestone's area `.jigc/milestones/<id>/`, each of its sub-tasks' areas, and each of their worktrees.** When: the record commit landed. On a refusal: no (MD5). Three lines as data, a place each; the second sentence is `M6`'s first.

> **The workbench is actually removed.** `discard` removes the sub-task areas **and `.jigc/milestones/<id>/`**

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

> ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)

— `design/storage.md` → *Repository layout & state location*

**MD5 — it refuses on three subjects, and removes nothing; `--force` is the single consent for all of them.** Not a line: it is where MD1 to MD4 take their *on a refusal: no*, and the row's consent. A cell under the consent spelling is exempt from the plant's rule only where the command's output names the planted path.

> It **refuses on three subjects, and `--force` is the single consent for all of them**

— `design/write-commands.md` → *Minting a milestone + its task list (M7)*

**MD6 — git's registrations of the sub-tasks' worktrees: removed, at a path under `.jigc/worktrees/` and at no other.** When: the record commit landed. On a refusal: no (MD5). `M7`'s sentences.

> A door removes a registration from `.git/worktrees/` **only at a path it created** under `<jigc_home>/.jigc/worktrees/` — a sub-task's `<sub-task-id>`, or a boundary's own `.combine-*` — **by path**, one record at a time (`git worktree remove --force <path>`), and **a run that refuses removes none**

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

> So every other registration — live, `prunable` or locked; a human's, or jigc's own under another path (a moved repository's sub-task records name the old one, and are exactly what `git worktree repair` needs) — is left byte-identical

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

**MD7 — the milestone's area: rebuilt — the cache re-derived where absent or stale.** As MP3. When: always. On a refusal: no (MD5).

**MD8 — the re-seed of the sub-tasks' areas: rebuilt where absent, for a sub-task the record has not settled.** As MA6. When: always. On a refusal: no (MD5).

*Both added 2026-10-19, as M11 and M12 were: the fourth sentence names `discard` among the seven. What they rebuild MD4 removes where the door acts.*

> the cache is **re-derived from the committed record when absent or stale**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> So the re-seed has **two halves at one site**: the milestone cache, and one standard working area per sub-task the record names

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

> **Every verb, one site.** `add-task` · `add-from-spec` · `provision` · `execute` · `join` · `finalize` · `discard` all pass through the reseed before their own guards

— `design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*

### What the docs do not say of the family, and what a line cannot

- **The message file of a record commit.** *"(a temp file, deleted after)"* is `design/finalize.md` → *6. Commit*'s, and the human's ruling of 2026-10-14 on §6, entry 6 has the file *deleted whether or not the commit landed*. MC1 and MA2 let the milestone's area be rebuilt, whole: a message file a record door leaves in that area at exit 0 is covered, and no cell can see it — the area is the docs' granularity, and naming the files the docs name (`base.json`, `tasks.json`) would leave the snapshot, which the docs name by no path, uncovered. Held where the docs are: a finding seen by hand — and a run shows one, at exit 0 after each record door, and held by the cells of a rejected record commit, whose refusal rewrites the file ([byte-table-findings.md](byte-table-findings.md) → §17, `r1-milestone-record-doors-leave-their-message-file-in-the-milestones-area`; added 2026-10-19, with the cells). **[Seen by the cells since 2026-10-19, by the seventh of the fixes after the review of `B7`'s part B (its tenth finding): the rendered message's file stands under no line, made or rewritten, by its name — `record-commit-msg.txt`, and the finalize's `finalize-message.tmp` — under `.jigc/` (§7, *deleted after*), on that sentence and that ruling. Every record door that lands leaves it, so eighteen cells of `create`, `add-task` and `add-from-spec` in the gate are *written outside their allowance* under the key, beside its three refusals.]**
- **The width of the cache's half, beside the message file.** MA2, MS2, MP3, ME1, MJ1, M11, MD7 read `.jigc/milestones/<unit>/` whole, and MC1 the minted area whole: a door other than `create` that rewrote `staged-snapshot.json`, or wrote under `merged/`, would be covered where its sentence names the cache alone. No cell meets either — the snapshot is `create`'s alone (MC1's second sentence), and `merged/` the boundary's (M1). Naming the cache's two files is C3's way, two lines a place; here it would be two lines at each of seven doors, each with its needs, and is left — a record row, open ([byte-table-milestone-family.md](byte-table-milestone-family.md) → §4).
- **The commits of `add-from-spec` beyond its first.** MS4's commit line holds what a line wrote, read against the working tree after the command: a record commit before the last holds a record the working tree no longer has, which the comparison reads as *git's state moved* — the frame's bound, stated. The home state's spec has one criterion, so its cells land one commit. **[Lifted 2026-10-19, by the seventh of the fixes after the review of `B7`'s part B (its tenth finding): the manifest holds the bytes of each blob a commit within reach changed a file to, and a commit before the last holding a *merged into* line's file partway holds (MS4, above). The cell `milestone-add-from-spec-a-spec-of-two-criteria` builds the spec of two in `vendored` — a task adds the criterion and lands it — and the door mints two sub-tasks over two record commits. **Still not told**: how many sub-tasks the spec's criteria should make — `<each-minted>` takes any id the record gains, and counting a spec's criteria would have the frame parse the spec, which is the product's to do.]**
- **A mint of two.** *A place the command mints* (§7) is any id at which nothing stood: a command that minted a second unit beside its own is not seen. **[Corrected 2026-10-19, by the sixth of the fixes after the review of `B7`'s part B (its eighth finding): `<minted>` is the one id the committed record gains, so a second unit, an area at an id no record gains, and an area made for a sub-task the record already named are each outside the line now; at `add-from-spec`, `<each-minted>` cannot tell how many sub-tasks the spec's criteria should make — the frame counts no criteria.]**
- **What `execute` and `join` write.** Nothing but the re-seed (ME3, MJ3); a write either makes outside it is outside the row. **[Corrected 2026-10-19, by the fourth of the fixes after the review of `B7`'s part B (its fifth finding): false for `join`, whose ref walk reads the committed edge index and rebuilds it where it is absent or stale — MJ4. True for `execute`, driven with the index absent on both binaries.]**
- **`add-from-spec` reads the spec**, and writes nothing at its home; no line names that home, and no plant is put there — a read place as a line (`May::CopiedFrom`) is the doc family's, for a copy-in.

---

## 4c. The movers family — `jigc rename`, `jigc relocate`, `jigc migrate-corpus`, `jigc config set`

*Written 2026-10-19, as the second commit of the movers family's task (the plan's `B8`), after the human's three rulings of that day (its first commit): the four rows from their docs, before a cell of any of them runs. No binary was run to learn what a command writes, and no list of files was taken from the product's source.*

**What the four share, and how they split.** Each writes the **committed store where it stands** — no task's working area, no promote — and each answers a home that is no regular file with one code (`design/finalize.md` → *4. Promote managed docs*, *The doors that write a committed home in place*). They split by the one act the docs make them differ on — **whether the door commits** (`design/finalize.md` → *1. Preflight*: *"The class is decided at the **act**"*):

1. **The doors that commit** — `rename` (*"**task-less and self-committing**"*) and `migrate-corpus` (*"A pathspec-limited self-commit"*): each lands its writes in one commit of its own, holding what its lines wrote and nothing else.
2. **The movers** — `relocate` and `config set`: each `git mv`s committed docs, stages them, and commits nothing (*"the two leaves that `git mv` a committed file and commit nothing"*); `config set` writes the project config layer besides, which it commits neither.

**Each leaf names its unit as its grammar has it.** `rename` names a doc by its address, `<type>:<slug>` — the first word after the leaf; `relocate` a doctype, the first word after the leaf, and the doctype's prior home, `--from <prior-home>`; `config set` a knob, the first word after the leaf, and its value, the second; `migrate-corpus` none — its subject is the stale corpus.

**Where a doc lives, read by the frame as the docs say it is resolved** — the word every row of the family needs, and no row before needed (§7, *a doctype's home*): a doctype's home is its schema's `location:` under the docs root (*"The resolved `docs-root` is prepended to every persisted doctype's `location:` at schema load"*), or its `placement:` file, re-rooted under `placement-root` where it carries a leading directory (`design/storage.md` → *Placement — direct-file and root-located homes (M38)*); a doc is `<home>/<slug>.md`. The schemas are read from the packs' own files — the shipped packs', and a listed pack's from the project's `packs.yaml` — as the docs root is read from the cascade (§7).

### `jigc rename`

Owner docs: `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*; `design/reconciliation.md` → *Rename detection*; `design/finalize.md` → *4. Promote managed docs* (the store doors); `design/validation.md` → the rollback-conflict and `file-state.absorbed` rows. Unit: the doc its address names. Home: `fresh`, with the leads that commit a doc whose id can move and a doc that refers to it — no prepared state holds a committed doc another refers to (`committed-singletons`' docs are singletons, whose identity is fixed; `vendored`'s spec and architecture doc refer to nothing).

**RN1 — the renamed doc's home, `<the-doc>`: retitled — rewritten with its `# ` title line changed and every other line as it stood; where the slug changes, moved to its new home, and gone from its own.** When: always. On a refusal: no (RN6).

> **Rewrites the moved doc's H1** to `<New Title>` (the unified retitle), then **`git mv`** old→new path

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

> rename **degrades to a retitle-only**: rewrite the H1 and commit, with **no `git mv` and no referrer repoint**

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

**`RN1, at its new home` — the renamed doc's new home, `<the-new-doc>`, `<slug>.md` in its doctype's home under the slug the command gives — `--slug`'s, else the one the `--to` title gives: retitled — the doc arrived there, where nothing stood, its `# ` title line changed and every other line as it stood; where the slug is the doc's own, the place is RN1's.** When: always. On a refusal: no (RN6). What squats the new home — a file of the user's, one committed, a link, a directory — is a need of the row there (added 2026-10-20, by the fourth of the fixes after the review of `B8`, its H7 and H11).

> **Rewrites the moved doc's H1** to `<New Title>` (the unified retitle), then **`git mv`** old→new path

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

> **Derives the new slug** = `slugify("<New Title>")` (the same `engine::slug::slugify` the `create` path uses via `id-from: title`, so a renamed doc is byte-identical to a freshly-created one of that title), unless `--slug` overrides

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

**RN2 — every committed doc that refers to it: repointed — each line that changes is a ref-field of its front matter that held the old identity as a whole value, and holds the new one in its place, re-emitted in the canonical form; nothing else changes.** When: the slug changes. On a refusal: no (RN6).

> rewrites each referrer's structured ref-field old→new across the **three persisted ref-fields** (`adr.supersedes`, `arch-doc.cites`, `spec.derived-from`)

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

> the CLI rewrites **structured ref-fields in the managed store** and nothing else

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

**RN3 — one commit on the checked-out branch, holding what RN1 and RN2 wrote and nothing else.** When: always. On a refusal: no (RN6). The door refuses a tree that is not clean first (RN7), so what the index held is `HEAD`'s and the commit holds the lines' writes alone.

> it is **task-less and self-committing**, mutating the committed store directly

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

> **re-baselines `file-state`** (`forget(old)` + `record(new)` + re-record every rewritten referrer) and rebuilds the edge index, stages, runs the user's git hooks (never `--no-verify`), commits

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

**RN4 — `.jigc/state/`: rebuilt — the file-state baseline re-keyed.** When: always. On a refusal: no — `C2` (§4a). RN3's second sentence, and:

> then re-baseline `file-state` (`forget(old)` + `record(new)` + re-record each rewritten referrer) and invalidate/rebuild the index

— `design/reconciliation.md` → *Rename detection*

**RN5 — `.jigc/index/`: invalidated — rebuilt, or its stamped file taken away for the next read to rebuild.** When: always. On a refusal: no — `C1`. RN4's sentence — *"invalidate/rebuild the index"* — and the three sentences `T5` and `M10` quote of `design/storage.md` (§3, §4).

**RN6 — a refusal leaves the store as it was.** Not a line: it is where RN1 to RN5 take their *on a refusal: no*.

> on **any** pre-commit failure restores all captured bytes, the `git mv`, **and** the file-state record (file-state is *inside* the rollback inventory, not a best-effort post-commit step — so a failed rename leaves the store byte-and-record identical to before)

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

**RN7 — what it refuses before the transaction.** Not a line: a target that does not exist, a tree that is not clean, a task or milestone in flight, a slug another doc holds, a destination git cannot record, a home whose entry is no regular file, an identity that is fixed. The quote that names the first three:

> **Validates** up front: target exists · tree clean

— `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*

**What of the docs is no line.** The prose report (*"reports … never rewrites"*) writes nothing. A rollback that meets a racer parks its pre-image under `.jigc/displaced/rename/` (`rename.rollback-conflict`): a racer is a writer inside the transaction's window, and no plant of the frame races — a cell that met the park would be *refused, and changed what a refusal may not*, the stricter reading.

### `jigc relocate`

Owner docs: `design/corpus-migration.md` → *The freeze-exempt sibling — the parallel relocation path (M39 inc-5)*; `design/reconciliation.md` → *Relocation collisions — two resolutions for one collision (M39)*; `design/finalize.md` → *1. Preflight* (the movers). Unit: the doctype, and its prior home `--from`. Home: `fresh`, with the leads that make the door's domain — a doctype no manifest governs, from a listed pack (*"the path is reachable only for a doctype supplied by a pack that ships no manifest entry for it"*), and a committed instance stranded at a prior home.

**RL1 — every committed `.md` under the prior home: moved, the same bytes, into the doctype's current home — gone from its place here.** When: always. On a refusal: **yes, where the refusal names a doc the sweep left** (RL5; §7, *a doc of the sweep left, and named*) — *[2026-10-20, the human's two rulings of that day: it read *no*]*.

> the door sweeps every committed `.md` under it into the doctype's current home

— `design/corpus-migration.md` → *The freeze-exempt sibling — the parallel relocation path (M39 inc-5)*

> The move is the **T1 standalone move primitive** (`relocate::move_doc` — `git mv old→new` + file-state re-key)

— `design/corpus-migration.md` → *The freeze-exempt sibling — the parallel relocation path (M39 inc-5)*

> `relocate` blocks that doc, moves the others, names the path to clear and exits non-zero

— `DECISIONS.md` → *2026-10-20 — The human's two rulings of the day, recorded: jigc never moves or overwrites a file to make room for its own, so a mover refuses over anything that stands at a destination — `relocate` blocking that doc, a root knob its whole re-point — and an acting sweep exits non-zero where an act it was asked for did not happen; the parking lines go, a mover's destination is a line's place, and `relocate`'s lines hold on a refusal that names the doc it left (the byte table, task B9, the first of its commits)*

> **An acting sweep does all it can, then exits non-zero if any act it was asked for did not happen; a reporting door keeps exit 0 for what it finds.**

— `DECISIONS.md` → *2026-10-20 — The human's two rulings of the day, recorded: jigc never moves or overwrites a file to make room for its own, so a mover refuses over anything that stands at a destination — `relocate` blocking that doc, a root knob its whole re-point — and an acting sweep exits non-zero where an act it was asked for did not happen; the parking lines go, a mover's destination is a line's place, and `relocate`'s lines hold on a refusal that names the doc it left (the byte table, task B9, the first of its commits)*

**`RL1, at its new home` — the destination, `<a-new-home>`: a doc under the prior home arrived at its name in the doctype's current home, the same entry, where nothing stood before — in the working tree or in what git held.** When: always. On a refusal: yes, as RL1. What stands at the destination — a file of the user's, one committed, a link, a directory — blocks the doc, and is a need of the row there (added 2026-10-20, with the first commit of `B9`, for the human's ruling on §6, entry 23: `RL3` parked it). RL1's first sentence and the two rulings', and:

> **Any file at a mover's destination — committed or untracked, managed or not — refuses**

— `DECISIONS.md` → *2026-10-20 — The human's two rulings of the day, recorded: jigc never moves or overwrites a file to make room for its own, so a mover refuses over anything that stands at a destination — `relocate` blocking that doc, a root knob its whole re-point — and an acting sweep exits non-zero where an act it was asked for did not happen; the parking lines go, a mover's destination is a line's place, and `relocate`'s lines hold on a refusal that names the doc it left (the byte table, task B9, the first of its commits)*

**RL2 — the main checkout's index staged at the places RL1 and `RL1, at its new home` wrote, and no commit.** When: always. On a refusal: yes, as RL1. Its sentence, and the two rulings':

> a `git mv` lands in the index and which commit it joins stays the user's to place

— `design/finalize.md` → *1. Preflight*

~~**RL3 — a foreign file at a destination: moved aside into `.jigc/displaced/`, under its name or the first free `<name>.<n>`, its index slot freed.**~~ When: a destination holds a file the store does not manage. On a refusal: no (RL5). **Struck 2026-10-20**, with the first commit of `B9`: the human's ruling on §6, entry 23 — *jigc moves a file aside only from a place it is about to remove, never from a destination*; what stands at a destination blocks the doc (`RL1, at its new home`). Its three sentences stay quoted here, as the design's answer the ruling reverses — findings of the run's ledger ([byte-table-findings.md](byte-table-findings.md) → §25):

> It is **relocated into the gitignored `.jigc/` workbench** (a dedicated `displaced/` subpath

— `design/reconciliation.md` → *Relocation collisions — two resolutions for one collision (M39)*

> a committed-but-unmanaged squatter's tracked-path index slot is freed so the managed `git mv` into the vacated destination succeeds

— `design/reconciliation.md` → *Relocation collisions — two resolutions for one collision (M39)*

> A name already taken in `.jigc/displaced/` gets the first free `<name>.<n>` beside it

— `design/reconciliation.md` → *Relocation collisions — two resolutions for one collision (M39)*

**RL4 — `.jigc/state/`: rebuilt — the moved docs' baselines re-keyed.** When: always. On a refusal: yes, as RL1 — and on any refusal, `C2`. RL1's second sentence and the two rulings'.

**RL5 — a refusal that names no doc it left moves nothing.** Not a line: the door's argument and doctype refusals are taken before the corpus is walked; a per-document refusal is a `blocked` row inside a report the door prints at exit 0 (*"This **`blocked` is not an exit rule**"*, `design/command-output-contract.md`), so it is the lines' to judge. *[2026-10-20, the eighth of the fixes after the review of `B8` (its R1): the docs choose exit 0 for a blocked doc on purpose — `design/validation.md` too, *"Its exit is therefore **unchanged**"* — and a cell of one lists it as refusing, exit 1: the docs agree on what makes no sense for a user, which is a finding against them, not a line's allowance ([byte-table-findings.md](byte-table-findings.md) → §25, `r1-relocate-refuses-a-doc-and-exits-0`).]* *[2026-10-20, the first commit of `B9`: **ruled** by the human that day (R1), option A — an acting sweep does all it can, then exits non-zero if any act it was asked for did not happen. So a refusal of one doc moves the others: RL1, `RL1, at its new home`, RL2 and RL4 hold on a refusal that names a doc the sweep left where it stood — its path, or the path to clear (the ruling on entry 23) — and a refusal that names none, an argument's or a doctype's, moves nothing.]*

### `jigc migrate-corpus`

Owner docs: `design/corpus-migration.md` → *The determinism boundary holds*, *Relocation — the file-move + display-title transform (M38)*, *The commit boundary — the verb lands its own migration (M42)*; `design/write-commands.md` → *`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)*; `design/finalize.md` → *4. Promote managed docs* (its one-doc refusal). Unit: none. Home: `fresh`, with the leads that commit a doc of the managed corpus below its doctype's current schema version — no prepared state holds a stale doc.

**MG1 — a stale managed doc at its doctype's current home: migrated — rewritten so that its prose stands as it stood, in order, and only its structure moves.** When: always. On a refusal: **yes, where the run committed** — the second sentence; a refusal that committed nothing changes nothing — §6, entry 22, ruled on 2026-10-19: a commit a hook rejects rolls back.

> The prose is **unchanged**

— `design/corpus-migration.md` → *The determinism boundary holds (the auto-migration distinction)*

> It is a refusal of **one doc**: like every refusal this command makes outside the transform's fold, the doc joins `blocked`, the run's other docs still migrate and commit, and the run exits non-zero

— `design/finalize.md` → *4. Promote managed docs*

**`MG1, from a prior home` — a stale managed doc at a home its doctype had at an earlier schema version: moved to the current home, migrated as MG1 has it, and gone from the prior home.** When: always. On a refusal: as MG1.

> **write once to `to`** → `remove(from)` → re-key file-state `from → to`

— `design/corpus-migration.md` → *Relocation — the file-move + display-title transform (M38)*

**MG2 — `.jigc/state/`: rebuilt — the migrated docs' baselines flipped, a moved doc's re-keyed.** When: always. On a refusal: as MG1.

> (`engine::state::persist` is an internal call that writes the bytes and flips the file-state baseline in one motion)

— `design/corpus-migration.md` → *The commit boundary — the verb lands its own migration (M42)*

**MG3 — one commit on the checked-out branch, holding what MG1 wrote — both halves of a move — and nothing else.** When: a doc migrated. On a refusal: as MG1.

> `git add` over exactly the paths the migration touched (added, modified, and both halves of a move) — explicitly **never a blanket `git add -A`**, so an ambient dirty tree is never swept in

— `design/corpus-migration.md` → *The commit boundary — the verb lands its own migration (M42)*

**MG4 — under the spelling `--no-commit`: the main checkout's index staged at the places MG1 and `MG1, from a prior home` wrote — both halves of a move — and no commit.** In place of MG3. When: always. On a refusal: no. *Since 2026-10-19, the human's ruling on §6, entry 21; until it, not a line*: the spelling took MG3 out of force and added none of its own, so nothing was staged — the reading under which more was red. The sentence the ruling found right:

> **`--no-commit`** performs the migration and leaves the changes staged for the operator to commit

— `design/write-commands.md` → *`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)*

A line lets a write happen and does not demand it: the product, which leaves the migration unstaged, holds under MG4 as it held without it. What the ruling asks of the product is a finding no cell can see ([byte-table-findings.md](byte-table-findings.md) → §24).

**MG5 — under the spelling `--dry-run`: the corpus read, and nothing written.** In place of MG1, `MG1, from a prior home`, MG2 and MG3. On a refusal: no. Its place is every doc of the store, which is what a plant that stands in a doc's place is aimed at, as at the doc family's `W3`.

> **`--dry-run`** reports what the migration *would* do and **writes nothing** — no disk write, no commit

— `design/write-commands.md` → *`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)*

### `jigc config set`

Owner docs: `design/overrides.md` → *Authoring deltas — the `jigc config` verbs*; `design/storage.md` → *Config layout*, *Placement — direct-file and root-located homes (M38)*; `design/validation.md` → the M52 registrations' rollback-conflict family; `design/finalize.md` → *1. Preflight* (the movers). Unit: the knob, and its value. Home: `fresh`.

**CS1 — the project manifest, `.jigc/config/manifest.yaml`: its knob set — every line that stood stays, in order, but a line of the knob it sets, and every line it adds is of that knob.** When: always. On a refusal: no (CS6).

> | `jigc config set <key> <value>` | a `scalar-set` |

— `design/overrides.md` → *Authoring deltas — the `jigc config` verbs*

> **by hand** (edit `manifest.yaml` + drop a native file

— `design/overrides.md` → *Authoring deltas — the `jigc config` verbs*

  The second is why the line keeps every other line: the manifest is the one place a hand edit is first-class, so the user's own lines in it are theirs.

**CS2 — a committed doc a root knob's re-point would strand: moved, the same bytes, to its doctype's home as the cascade resolves it with the knob set — gone from its place here.** When: the knob is `docs-root` or `placement-root`. On a refusal: no (CS6).

> `config set docs-root` now detects + routes + **auto-moves** each stranded instance

— `design/storage.md` → *Placement — direct-file and root-located homes (M38)*

> `jigc config set placement-root` **detects + routes + moves** every committed doc at a placement doctype's prior home *before* the knob lands

— `design/storage.md` → *Placement — direct-file and root-located homes (M38)*

**`CS2, at its new home` — the destination, `<a-new-home>`: the doc arrived at its doctype's home as the cascade resolves it with the knob set, the same entry, where nothing stood before — in the working tree or in what git held.** When: as CS2. On a refusal: no (CS6) — a root knob refuses its whole re-point, the knob unchanged. What stands at the destination refuses the re-point, and is a need of the row there (added 2026-10-20, with the first commit of `B9`, for the human's ruling on §6, entry 23: `CS3` parked it). CS2's first sentence, and:

> **Any file at a mover's destination — committed or untracked, managed or not — refuses**

— `DECISIONS.md` → *2026-10-20 — The human's two rulings of the day, recorded: jigc never moves or overwrites a file to make room for its own, so a mover refuses over anything that stands at a destination — `relocate` blocking that doc, a root knob its whole re-point — and an acting sweep exits non-zero where an act it was asked for did not happen; the parking lines go, a mover's destination is a line's place, and `relocate`'s lines hold on a refusal that names the doc it left (the byte table, task B9, the first of its commits)*

~~**CS3 — a foreign file at a new home: moved aside into `.jigc/displaced/`.**~~ When: as CS2. On a refusal: no (CS6). **Struck 2026-10-20**, with the first commit of `B9`, as RL3. RL3's three sentences, and:

> delegating to `relocate::relocate_stranded` so a foreign file squatting the new home is parked in the gitignored `.jigc/displaced/` workbench rather than clobbered

— `design/storage.md` → *Placement — direct-file and root-located homes (M38)*

**CS4 — `.jigc/state/`: rebuilt — the moved docs' baselines re-keyed.** When: as CS2. On a refusal: no — `C2`.

> the gitignored `.jigc/state/file-state.json` the move primitive re-keys on every call

— `design/validation.md` → *The M52 registrations — Increment 5: the rollback-conflict family*

**CS5 — the main checkout's index staged at the places CS2 and `CS2, at its new home` wrote, and no commit.** When: as CS2. On a refusal: no (CS6). RL2's sentence, and:

> none of the six verbs commits anything

— `design/overrides.md` → *Authoring deltas — the `jigc config` verbs*

**CS6 — a refusal leaves the store and the knob as they were.** Not a line: the root knobs' value rules are asked *"before anything moves and before the knob lands"*, and a re-point that fails after a move undoes every move — and, by the human's ruling of 2026-10-20 on §6, entry 23, a re-point over anything at a destination refuses whole (`config.repoint-failed`, `config.rollback-conflict`), parking a racer's bytes under `.jigc/displaced/config/` — no plant races, as at RN6.

### What the docs do not say of the family, and what a line cannot

- **Whether the provenance stamp is refreshed.** *"`jigc setup` writes it and store-writing ops refresh it"* (`design/storage.md` → *Store provenance — the binary-provenance stamp (M36)*) names no op of this family. No line lets one write `.jigc/version`; a door that does is *written outside its allowance*, the stricter reading, and a cell that meets it raises the question.
- **What a door that commits nothing, or `migrate-corpus`, may do to the edge index.** No sentence of `relocate`, `migrate-corpus` or `config set` says it writes or invalidates the index; the same.
- **`migrate-corpus --no-commit` — the migrated paths staged, or left as the verb left them before it committed?** Two docs said two things: §6, entry 21, ruled on 2026-10-19 — staged, and nothing committed (`MG4`).
- **A root knob's moves staged and the knob they follow left unstaged.** The docs agree, and what they agree on makes no sense for a user: a commit of what is staged lands the moves without the knob that names their homes, and a clone of it finds its docs at homes its cascade does not resolve. Recorded as a finding of the docs and the product, not as an allowance the rows keep: [byte-table-findings.md](byte-table-findings.md) → §23. The rows let the knob be staged with the moves (CS5 stages what CS1 writes), so the sensible behaviour holds under them; no cell can see the knob left unstaged, a manifest holding no *must*.
- **`relocate` and `config set` find a destination by the cascade, and the frame reads it the same way** (§7, *a doctype's home*): the shipped packs' schemas, a listed pack's from `packs.yaml` by a path from the repository's root, a project shadow. *Not read*: a pack named by an absolute path, a listed pack's own knobs.
- **Which bytes `rename` repoints.** The lines that hold the old identity, each holding the new one in its place; a referrer whose ref the product re-emits in another form — the whole list reordered — would read *written outside its allowance*, the stricter reading.
- ~~**Whether a `relocate` destination's occupant is foreign or managed.**~~ **The docs say it** — *"A *managed* instance already at the destination … **surfaces the collision as a block**"*, *"Classification is by **baseline**"* (`design/reconciliation.md` → *Relocation collisions*) — and it was listed here, as what they do not say, until the review of `B8` (its R7). Since the second of the fixes after that review the frame reads the file-state baseline the command found, and `RL3` and `CS3` move aside only a destination's foreign occupant (§7, *moved aside*); a managed one is held by a cell at each mover. *[2026-10-20, the first commit of `B9`: since the human's ruling on §6, entry 23, the occupant blocks the move whether the baseline records it or not, `RL3` and `CS3` are struck, and the frame reads the baseline no more.]*
- **What a cache holds on a refusal that committed.** `MG2` lets the file-state baseline be rebuilt whatever its bytes, so on a refusal that landed a commit a baseline flipped for the refused doc holds. Not a line's to say: the baseline is gitignored and rebuildable (`design/storage.md` → *Source of truth*), and the frame reads no cache's content at any door (the seventh of the fixes after the review of `B8`, its left-open).
- **A refusal after more than one commit.** *It committed* reads what git tracks against the tree `HEAD` holds after the command, so a refusal that landed two commits would have its first read against its last. No door of the table does so — `migrate-corpus` makes one commit — and a line cannot say what no cell can meet; a door that comes to would need the word to read each commit against the one before it, as *merged into* does for a record (§7) (the seventh fix's left-open).

---

## 4d. The mint family — `jigc start`, `jigc workflow`, `jigc migrate`, `jigc task amend`

*Written 2026-10-20, as the second commit of the mint family's task (the plan's `B9`), after the human's two rulings of that day (its first commit): the four rows from their docs, before a cell of any of them runs. Each leaf was driven once with nothing planted on this tree's debug build, after the rows were read off the docs, to see what it writes beside them (the self-check the review of `B8` asks for) — and no line was written from a run: every line below rests on a sentence.*

**What the four share.** Each starts work or re-enters it, and **commits nothing**: *"Only `finalize` produces committed changes; the working area itself is never committed"* (`design/storage.md` → *The per-task working area (staging)*). Three mint a task — `start` in its intent forms, `migrate`, `task amend` — and the task is **an area that carries its base pin** (*"its presence is what makes a directory under `tasks/` a task at all"*, the same section), so the place a mint writes is `.jigc/tasks/<minted-task>` (§7): the one id at which an area carries its pin after the command, where nothing stood before. One, `workflow`, re-enters a task by its `--task`, and on a milestone sub-task's first entry provisions its area. And each compose reads the committed docs through the edge index, whose read rebuilds it where its file is gone or its stamp stale.

**Each leaf names its unit as its grammar has it.** `start` and `workflow` by `--task <id>`, and else the single task that stands (`start`'s resume and orientation); `migrate` and `task amend` name none — each mints one, and the source `migrate` reads is the first word after the leaf, `<the-source>` (§7).

**Where the four split** — into the cells' parts, by the docs they share: the three mints, whose sentences are `design/write-commands.md` → *Task origination* and the working area's in `design/storage.md` (`start`, `migrate`, `task amend`); and the re-entry, `design/write-commands.md` → *Sub-agent re-entry* (`workflow`).

### `jigc start`

Owner docs: `design/write-commands.md` → *Task origination*; `design/bootstrap.md` → *Orientation output examples*; `design/storage.md` → *The per-task working area (staging)*, *Edge index lifecycle*; `design/finalize.md` → *Dirty-tree policy* (the staged snapshot). Unit: the task `--task` names, else the single task that stands. Home: `fresh`. **Two spellings**: `bare` — the orientation, *"Never mints, never composes a side-effectful workflow"* — and `--task`, the resume, *"resumes an existing task: loads its working area, recomposes the task's own persisted workflow, shows where it is. No minting."*; each takes ST1 out of force.

**ST1 — `.jigc/tasks/<minted-task>`: rebuilt — the minted task's working area made where nothing stood: its base pin, intent and workflow, the commit doc it stages, the staged snapshot.** When: always, in the intent forms. On a refusal: no — a refusal mints nothing.

> frozen, collision-suffixed under the same discipline as artifacts; pinned to base = HEAD; opens `.jigc/tasks/<id>/`.

— `design/write-commands.md` → *Task origination*

> `jigc start` itself stages `commit:<id>.md`, so the population the guard fires over is every minted task, pristine skeleton included

— `design/write-commands.md` → *The verbs*

> Each of the three task-minting doors (`jigc start` compose forms, `jigc migrate`, `jigc milestone create` for the shared checkout's aggregate index) snapshots the pre-task staged state — the staged set (path + blob) **and the staged-deletion set** — into the working area.

— `design/finalize.md` → *Dirty-tree policy*

**ST2 — `.jigc/index/`: rebuilt — the edge index the sweep and the compose read.** When: always. On a refusal: no — `C1`.

> Never mints, never composes a side-effectful workflow; reporting a live task's findings does run the task-scope sweep, so it reads git and materializes the derived edge-index cache.

— `design/write-commands.md` → *Task origination*

> | **committed rebuild** | first read after a stamp mismatch (branch switch, pull, rebase) — rebuild from committed `.md`s | atomic; persists in `.jigc/index/` with the new stamp |

— `design/storage.md` → *Edge index lifecycle*

**ST3 — what it refuses before it mints.** Not a line: an active task at the slugged id (*"→ **reject**, with the existing task's status surfaced"*), a residual at the area path (*"→ **reject under the same code and the same stable key**"*, *"The mint keeps refusing rather than adopting the directory"*), a title that slugs to nothing, an unknown or verb-routed workflow, a doc-only mint from a linked worktree (*"`jigc start --workflow <W>` therefore refuses **before it mints**"*, `design/finalize.md` → *The doc-only arm*).

### `jigc workflow`

Owner docs: `design/write-commands.md` → *Sub-agent re-entry — `jigc workflow <W> --task <id>` (M8)*, *The write-time `--task`-scoped barrier (M8)*; `design/storage.md` → *Edge index lifecycle*; `design/worked-examples.md` → the M44 wave's flow 45 (`--preview`). Unit: the task `--task` names. Home: `fresh`, with the leads that mint a milestone of one sub-task and provision its worktree — the re-entry runs there. **One spelling**: `--preview`, *"The preview composes step text without minting"* and *"provisions **no** `.jigc/tasks/*` directory"* (flow 45, item 3); it takes both lines out of force.

**WF1 — `.jigc/tasks/<unit>`: provisioned — made where nothing stood: the sub-workflow's commit doc and the provenance manifest that records it, on a sub-task's first entry; nothing that stood rewritten.** When: always. On a refusal: no.

> The first `jigc workflow <W> --task <id>` re-entry **provisions** the sub-workflow's deterministic instances (the commit doc, any workflow-provisioned docs) into the sub-area — mirroring how `jigc start` provisions at mint time, just deferred to the sub-agent's first entry rather than at `add-task` (so areas that are never spawned are never provisioned).

— `design/write-commands.md` → *Sub-agent re-entry — `jigc workflow <W> --task <id>` (M8)*

> `provision_commit_doc` likewise records `created` provenance under M8 — a per-sub-task commit doc must appear in the provenance manifest the join consumes

— `design/write-commands.md` → *The write-time `--task`-scoped barrier (M8)*

**WF2 — `.jigc/index/`: rebuilt — the edge index the compose reads.** When: always. On a refusal: no — `C1`. ST2's second sentence.

**WF3 — what it refuses.** Not a line: a `<W>` other than the one the sub-task recorded (*"a mismatch is rejected"*), a sub-task whose base the checkout it is typed in has moved off (*"a **milestone sub-task** keeps the blanket refusal at **every** per-task read door"*, `design/storage.md`), a task that is not live.

### `jigc migrate`

Owner docs: `design/auto-migration.md` → *The `jigc migrate` verb (the trigger)*, *The source seam (foreign content → composed context)*, *Retire-the-foreign-original (the first destructive write)*; `design/storage.md` → *Source of truth*; `design/finalize.md` → *Dirty-tree policy*. Unit: none — it mints one. Home: `fresh`, with the leads that commit a foreign file, `HISTORY.md`.

**MI1 — `.jigc/tasks/<minted-task>`: rebuilt — the migration task's working area made where nothing stood: its base pin, intent and workflow, the staged source and its recorded path, the commit doc, the staged snapshot.** When: always. On a refusal: no.

> Mints an **off-router migration task** (mirrors `record-change`: `creates-task: true`, `selectable: false`, `allows-create: [{type: changelog, as: changelog}]`), recording the foreign source path on the task.

— `design/auto-migration.md` → *The `jigc migrate` verb (the trigger)*

> **Settled: a CLI-owned source seam** — the `migrate` verb stages the foreign bytes into the task (a task-scoped source artifact) and a new placeholder surfaces them into the composed migration workflow as deterministic context.

— `design/auto-migration.md` → *The source seam (foreign content → composed context)*

  And the staged snapshot's sentence, ST1's third.

**MI2 — `<the-source>`: read, and nothing written — the foreign file the migration is of.** When: always. On a refusal: no. The retire of the source is `jigc task finalize --approve`'s, never the mint's.

> Reads the foreign file's bytes.

— `design/auto-migration.md` → *The `jigc migrate` verb (the trigger)*

**MI3 — what it refuses before it mints.** Not a line: a token carrying git's pathspec magic, an absolute path, a path the shipped predicates refuse (`migrate.source-untrackable`), a source git holds no copy of (`migrate.source-untracked`), a linked worktree (*"it now refuses **before it reads, mints or stages anything**"*), a live migration of the same source (`task.serial-collision`).

### `jigc task amend`

Owner docs: `design/write-commands.md` → *Task origination*; `design/finalize.md` → *The amend arm*. Unit: none — it mints one. Home: `fresh`, whose `HEAD` is a commit with one parent.

**AM1 — `.jigc/tasks/<minted-task>`: rebuilt — the amend task's working area made where nothing stood: its base pin at `HEAD`, the `amend` marker, the empty commit doc.** When: always. On a refusal: no.

> **`jigc task amend ["<intent>"]`** mints a task whose base pin is **`HEAD`**, writes an `amend` marker holding that commit's sha, provisions an **empty** `commit` doc, and composes the dev pack's verb-routed `amend` workflow

— `design/write-commands.md` → *Task origination*

> It **commits nothing**: it is `BEHALF_DOORS`' `Neither`, and `jigc task finalize <id>` is what rewrites the commit, on its own second commit model

— `design/write-commands.md` → *Task origination*

**AM2 — what it refuses before it mints.** Not a line: a `HEAD` an amend cannot rewrite — *"`HEAD` shapes an amend cannot rewrite — unborn, a root commit, a merge commit — are refused with `amend.head-shape` **before anything mints**"* — and a title that slugs to nothing.

### What the docs do not say of the family, and what a line cannot

- **Whether bare `jigc start` writes anything — answered, not two things.** *"reporting a live task's findings does run the task-scope sweep, so it reads git and materializes the derived edge-index cache"* (ST2) stands beside `design/reconciliation.md`'s *"`validate`, `start`, and read-path sweeps stay pure readers (no write side effects on a read verb)"*; the second's paragraph is *Persistence of the shifted baseline* — what a sweep **persists of the file-state baseline** — and says so of itself (*"this is a rule about what a **sweep** persists"*). The index is a rebuildable cache the first sentence names; no line lets any of the four write `.jigc/state/`.
- **A slug that collides — suffixed or refused? Answered, not two things.** ST1's sentence says the id is *"frozen, collision-suffixed under the same discipline as artifacts"*; the same section's collision paragraph says a serial collision is *"→ **reject**, with the existing task's status surfaced"*, *"*Never silently suffixed, never silently reused*"*, and gives the rule that joins them: *"The principle: **suffix only in the deterministic parallel case**"* — a fan-out's join, which no door of this family is. So a mint over a live task's id refuses (ST3), and ST1's place is the one area where nothing stood either way; `design/auto-migration.md` says the same of `migrate`'s mint (*"`mint_task` hard-rejects a collision, never suffixes"*).
- **Whether `jigc task amend` writes the staged snapshot.** `design/finalize.md` → *Dirty-tree policy* names *"the three task-minting doors"* — `start`, `migrate`, `milestone create` — and the amend door, which mints a task too, is not among them, while the product's mint registry gives it a snapshot (`engine::state::MINT_DOORS`). AM1's place is the minted area whole, so the snapshot stands under it whichever is meant; the sentence is a lead for the docs, not a line's to settle.
- **Whether a mint refreshes the provenance stamp** — *"store-writing ops refresh it"* (`design/storage.md` → *Store provenance*) names none of the four; a mint writes no store. No line lets one write `.jigc/version`.
- **Where a re-entry runs.** A milestone sub-task's re-entry is typed in its worktree, and its task area stands under the main checkout — *"the committed **doc-store** and the shared **`.jigc/`** (config, task areas, caches) resolve against **jigc_home** — the main checkout"* (`design/storage.md` → *CLI and git*) — so WF1's place is the main checkout's, and the command runs at the worktree.

---

## 4a. The caches, on a refusal — three lines that stand in every row

Written on 2026-10-13 from the human's ruling of 2026-10-12 on §6, entry 1, which that entry now quotes. **What a cache is comes from the doc and not from the ruling's two examples**: `design/storage.md` declares three things rebuildable derived state — the edge index, the file↔state hashes, and a milestone's `{base,tasks}.json` — and names where each stands in its layout. Each line below quotes the sentence that declares the cache, and what the layout says of its directory: the comment that stands on the line of `index/`, of `state/` and of `milestones/` in that block, from its first word (the line's own columns of spaces are left out of the quotation). Nothing else under `.jigc/` is declared so: a task's working area is *a transient working copy* and one line of it is *authority* (→ *Source of truth*, the paragraph on `source-path`); `displaced/` is *the one gitignored subdir whose contents no operation can reconstruct*; a milestone's `merged/` has its own line, `M1`.

**What *rebuilt* lets a refusal do, and what it does not.** A file of a cache may be made, or written whole; a directory of one may be made. **No file of a cache may be taken away and left gone** — that is no rebuild — and nothing outside the three places is touched by these lines. They hold **when the command refused, and only then**: what a command that exits 0 may write there is its own row's to say (`T5`, `M10`; `U1` removes the workbench whole), and where a row says nothing — `setup` — the ruling does not speak (§6, entry 1, *what the ruling leaves open*).

**C1 — `.jigc/index/` and everything under it: rebuilt.** When: the command refused. On a refusal: yes.

> The edge index and the file↔state hashes are **rebuildable from the committed docs**, so they are gitignored, never committed

— `design/storage.md` → *Derived caches*

> gitignored — edge index (rebuildable cache)

— `design/storage.md` → *Repository layout & state location*

**C2 — `.jigc/state/` and everything under it: rebuilt.** When: the command refused. On a refusal: yes. The first sentence is C1's first.

> gitignored — file↔CLI-state hashes (rebuildable)

— `design/storage.md` → *Repository layout & state location*

**C3 — `.jigc/milestones/<id>/base.json` and `.jigc/milestones/<id>/tasks.json`: rebuilt.** When: the command refused. On a refusal: yes.

> **demotes** that JSON to exactly a *rebuildable cache* (re-derived from the record when absent or stale)

— `design/storage.md` → *Source of truth*

> gitignored — milestone WIP (merge staging, worktrees) + the {base,tasks}.json cache re-derived from the committed milestone-record (M39; team-ready-state.md)

— `design/storage.md` → *Repository layout & state location*

---

## 5. The acceptance's lines, read against the rows

Each line of the plan's §5, with the rule or the line that makes it red on `1.0.0-rc.24` as the plan's own runs describe that binary — read by hand, nothing run. *The plant's rule* is the plan's §2: a planted entry is the user's, and where its bytes stand nowhere after exit 0 the cell is *lost* whatever the row declares.

| the cell | what the plan says `rc.24` did | red by |
|---|---|---|
| `setup` · `bare`, `HEAD` unborn · an untracked file at `.jigc/AGENT.md`, at `.jigc/config/packs.yaml` | exit 0; both replaced, the bytes in no object | **the plant's rule** — *lost*. S1 and S4 let the command write those paths, and S4's second sentence says it refuses over a file no commit holds *on an unborn `HEAD` as on a born one*; no line is needed to make it red, and none can make it green |
| — the same, `HEAD` born | exit 1, nothing changed | not red: S13 |
| `task finalize` · `committed-singletons`, no file-state baseline · a line added by hand after the task's first write | exit 0; the line in no object | **the plant's rule** — *lost*. T1 lets the command write the doc's home; the line that stood there is the user's |
| — the same, the baseline present | a refusal | not red, if the refusal changed nothing but what `C1` and `C2` let a refusal rebuild (ruled on 2026-10-12; this cell read *what §6, entry 1 asks about* until then) |
| `milestone finalize` · `fresh` · an untracked file at the id the join numbered onto | exit 0; the bytes in no object | **the plant's rule** — *lost*; and M2's sentence, which says the refusal is taken there |
| `milestone finalize` · `fresh` · a committed doc at that id, committed before `milestone create` | exit 0; the doc's content replaced, history holds it | **M2 and M3** — *git's state moved outside its allowance*: the commit changes a path a commit already held to a doc the unit minted, which M2's sentence says never happens. The planted bytes stand in history, so it is not *lost* |
| `task finalize` · `fresh` · a link to nothing at the created doc's home | exit 0; the commit holds a link; the prose at the link's target | **T1 and T8** — *git's state moved outside its allowance*: T1's second sentence declares *a regular file at exactly its canonical path*, and the commit holds a link there; the staged doc stands in no commit. The prose stands at another path, so it is not *lost*; that path is also a write no line covers |
| `uninstall` · `fresh` · a file somebody put under `.jigc/logs/`, `.jigc/state/`, `.jigc/index/` | exit 0; gone, in no object | **the plant's rule** — *lost*, three times. U1 lets the command remove `.jigc/` and what is under it, and U7 names the state as one that refuses |
| — the same at `.jigc/`'s root | exit 1, nothing changed | not red: U7 |
| `milestone finalize` · `fresh` · the user's own linked worktree, its directory moved away | exit 0; the registration gone, a commit no longer reachable | **M7** — *git's state moved outside its allowance*: a registration at a path that is not under `.jigc/worktrees/` |

Six of the seven red lines are red by the plant's rule or by a sentence that names the state; **none needed a line widened**, and the one row a wrong allowance could have turned green — `uninstall`, whose doc declares that `.jigc/` is removed — is held by the plant's rule and not by U1.

---

## 6. Where a doc is silent, or says two things — the list for the human

One list to rule from, one entry at a time. It decides nothing. Until an entry is ruled, a cell that hangs on it follows the stricter reading — the one under which more is red — and is marked. **Entries 1, 2, 3, 5, 6, 8, 9, 10, 11, 12, 14, 15, 18, 19, 20, 21, 22 and 23 are ruled** (2026-10-12, 2026-10-14, 2026-10-15, 2026-10-18, 2026-10-19 and 2026-10-20), each saying how it is carried; **entries 13, 16 and 17 are answered by the design docs** (2026-10-18), and were no silences; entries 4 and 7 are open. *The human's declared bound 2 of 2026-10-06 — an uncommitted edit carried into a task and replaced by its write to the same part — is retired (2026-10-19): such a write refuses; [DECISIONS.md](../../../../../DECISIONS.md), the entry of that day.*

**1. *Either a rebuildable cache or a transient working copy* — may a command that refuses leave the caches rebuilt? Ruled on 2026-10-12: it may.**

- *The words.* `design/storage.md` → *Source of truth*: *"Every other artifact — the staging working area, the edge index, the file↔state hashes — is either a rebuildable cache or a transient working copy."* And `design/finalize.md` → *What `finalize` is*, T7's sentence: a failure before the commit *"rolls back to "as if `finalize` was never called" — the working area is intact, canonical paths unchanged"*; → *2. Validate*: *"**Blocking findings present** → abort with the finding list. No side effects."*
- *The two readings it was put with.* (a) What can be rebuilt may be rebuilt by a command that refuses: a refusal that leaves a cache changed is no finding. (b) The sentence of `storage.md` says what those files **are**, and not which command may write them when; two sentences of `finalize.md` say a refusal leaves no side effect; under it the change is *refused, and changed what a refusal may not*.
- *The ruling* — the human's, of 2026-10-12, option A of two, as the orchestrating session handed it on:

  > **A refusal may rebuild a cache: nothing is lost, and such a cell is not red for it.** The exit rule's first clause is about lost files, and a rebuilt cache loses none. The sentence *No side effects* in `design/finalize.md` is an inconsistency between two design docs, to be recorded as one and corrected — not a rule the table holds a command to.
  > **Not taken:** *a refusal leaves every byte as it was, caches included* — under which most refusing cells are red for a behaviour the storage design intends.

- *How it is carried.* By three lines that stand in every row, `C1` to `C3` (§4a): the places are the ones `design/storage.md` declares rebuildable derived state — **three, not the two the question named**: the edge index under `.jigc/index/`, the file↔state hashes under `.jigc/state/`, and a milestone's `base.json` and `tasks.json` under `.jigc/milestones/<id>/`. `M1` is as it was. `T5` and `M10` are as they were, for a command that exits 0.
- *Bounded by what rebuilt means.* A refusal may write a cache whole from what the tree holds. Under these lines it may not take a cache file away and leave it gone — **held**: the frame's word for the lines covers a file made or written and no file gone. Nor may it write a cache that the next acting command would read as other than the tree says — **not held by the frame**: a manifest shows that a cache was written, never whether what was written is true. The check that would hold it is the same command run again once the refusal's cause is gone, coming out as it does on a tree whose cache nobody touched; removing the cause is the route's act, and the byte table follows no route. One narrower form the frame could run and no cell does: the refused command run a second time over what the first left — it must refuse again, and the plant must still stand.
- *What the ruling leaves open.* **(i) Exit 0.** The ruling speaks of a refusal. Whether a command that exits 0 may write a cache its own doc names no write to — `setup` is the one of the four — is not said; the lines hold on a refusal only, so such a write is *written outside its allowance* until somebody rules. No cell of the first cut turns on it. **(ii) The file↔state hashes are called rebuildable and have no rebuild.** The sentence C1 and C2 quote says both caches are *rebuildable from the committed docs*; the next paragraph of the same section says the hashes *"carry **no stamp and no rebuild path**"*. So a refusal that writes under `.jigc/state/` is not rebuilding from the tree: it is recording a baseline, and a baseline recorded over a file somebody edited is exactly a cache that says other than the commit does. `C2` lets the write, by the doc's first sentence and the ruling's word; whether that is what was meant is the human's, and it is where the bound that is not held would bite. **(iii) A minted work unit is no cache.** A task area or a milestone area that a refusal leaves behind, and that a later command reads as authority, is covered by none of the three lines and stays red, cell by cell, until ruled (the plan's §7, question 2).
- *Met on 2026-10-15, by the install family's cells — the first open point.* **Every `setup` that acts makes `.jigc/state/`**, an empty directory no sentence of its doc names a write to; so each of the twenty cells in which it acts is *written outside its allowance* for it, and carries this entry's mark. *What the product should do, by the project's principles*: nothing different — an empty directory of jigc's own gitignored workbench loses no byte and is no authority; **the doc sentence that would change** is `design/assistant-adapter.md` → *Generated, minimal, regenerated*, which names what `setup` lays down, to say it lays out the workbench's cache directories too — or the product makes the directory when it first writes there. The ruling would take the mark off twenty cells; seventeen of them would hold.
- *Ruled on 2026-10-15, the first open point*, option A, as the orchestrating session handed it on: **the product is unchanged; `design/assistant-adapter.md` → *Generated, minimal, regenerated* is to say that `setup` also lays out the workbench's cache directories.** The twenty cells this entry marked are marked off. *How it is carried*: `S16` (§1), a line whose sentence is the ruling's as `DECISIONS.md` records it, and a word for it, *laid out* (§7) — a directory made and no file in it. *What moved*: of the twenty, **thirteen hold**; six stay red under their own keys (entries 10 and 12, and two staged changes lost — [byte-table-findings.md](byte-table-findings.md) → §6); and `setup-a-hook-that-rejects` is red under a key of its own by the ruling on entry 11. The *seventeen* this entry projected are four more than hold: entry 10's three cells, read under the reading that ruling did not take, and `setup-a-hook-that-rejects`, which the ruling on entry 11 moved to a key of its own. The doc's missing sentence is a doc-only finding: [byte-table-findings.md](byte-table-findings.md) → `r1-assistant-adapter-design-does-not-name-the-cache-directories-setup-lays-out`.
- *The inconsistency* between the two docs is recorded, and fixed by nobody here: [byte-table-findings.md](byte-table-findings.md) → `r1-finalize-design-says-no-side-effects-where-storage-design-says-caches-are-rebuilt`.

**2. `setup` refuses in two ways, and the docs tell them apart by the moment, which no manifest shows. Ruled on 2026-10-14: every refusal of `setup` leaves the repository as it was.**

- *The words.* S13: *"because the ask is pre-write, the refusal installs nothing"*. S12: a run that *"fails after its first write"* *"records the install footprint … over the paths it wrote in that run … and **stages them**"*. And of the ask repeated before the commit, in the same paragraph as S13: *"where that ask fires the install is written and staged"*. One finding code, `setup.dirty-install-path`, is the first refusal's and the third's.
- *Reading (a)* — a refusal of `setup` may leave S1 to S10 written and staged: S12 holds on every refusal. A refusal before the first write that had nevertheless written would pass.
- *Reading (b)* — a refusal of `setup` leaves nothing: S13 holds on every refusal. A failure after the first write — a hook that rejects the install commit, a hooks directory that cannot be written — is then red although S12 declares it.
- *A third way, which is a change to the plan and so the human's:* the cell says which of the two refusals it constructs, as it says whether it expects a refusal at all, and the row carries a line for each.
- *What each makes of the cells.* No cell of the first cut turns on it: the acceptance's two `setup` cells refuse before the first write, and their plant is held by the plant's rule under either reading. The cells it decides are `B5`'s — a rejecting hook at `setup`'s commit.
- *Until ruled:* (b). S12 is recorded and is no line of the row as data.
- *The ruling* — the human's, of 2026-10-14, after asking for the robust and future-proofed answer (*"Yes agreed."*), as the orchestrating session handed it on: **every refusal of `setup` leaves the repository as it was** (caches aside, by the ruling on entry 1). A failure after the first write **rolls back** every path the run wrote — each was clean against `HEAD`, absent, or an untracked file `setup` read to merge into, so its bytes before the run are known — instead of staging them; the install-footprint record keeps only the job of a run that was killed mid-write. It is reading (b), and more: not a reading of the two sentences but the rule the product is held to.
- *How it is carried.* The row holds `S13` on every refusal — its lines say *on a refusal: no*, as they did — and `S12` is no line. **S12's sentence is a design inconsistency**, recorded in [byte-table-findings.md](byte-table-findings.md) → `r1-setup-design-stages-a-failed-install-where-the-rule-is-that-a-refusal-changes-nothing`, to be corrected with the product's fix. A cell that shows staging after a late failure is red, and a finding.
- *What each makes of the cells.* The cells it decides are the late failures — a hook that rejects the install's commit, and a hooks directory that cannot be written; `B5` builds them.

**3. Where `setup` keeps the record of a failed run — read under the ruling on entry 2.** S12's sentence names *the install footprint* and M51's function, and no path. `design/project-setup.md` → *Flow 2 hardening* lists *the install-footprint record a failed first `setup` writes* among jigc's own files under `.jigc/index/`, `.jigc/state/` and `.jigc/logs/` without saying which.

- *What it becomes.* Under the ruling, **no refusal leaves the record**: a refusal rolls back, and the record keeps only the job of a run killed mid-write — which is no cell, since the table's command always ends. So a record a refusal of `setup` leaves is a write a refusal may not make, wherever it stands.
- *What the table can and cannot hold of it.* No doc names its path. If it stands under `.jigc/state/` or `.jigc/index/`, the lines `C1` and `C2` — written as places, *everything under the directory* — would excuse it on a refusal as a rebuilt cache, which it is not: it is rebuilt from nothing committed, and it records one run's own job. **The table cannot tell it apart from a cache without a path no doc gives**, and taking one from the product's source is what a row may not do. So a cell that left only the record would read *refuses, nothing changed*; the late-failure cells are red on other grounds today — the paths written and staged — and that is what holds them. It is put to the human as a question of `B5`.
- *Seen on 2026-10-15*: each of the twelve late failures of the install family leaves `.jigc/state/setup-install-footprint` — the record, under `C2`'s place — and `C2` excuses it there, beside the install written and staged.
- *Ruled on 2026-10-15*, option A: **the record is named in the doc and moves out of `.jigc/state/`, since it is not rebuildable; a refusal leaves none; the table checks its path; it is part of the setup-rollback finding; the fixer settles how the old path is read.** *How it is carried*: the frame holds the path it was seen under as no cache (`byte_cell::NO_CACHE`) — no line that lets a cache be rebuilt covers it — so each of the twelve late failures now names it among what a refusal may not change, under `r1-setup-refuses-after-its-first-write-and-leaves-the-install-written`, whose class it already had. Where the record moves to is the doc's to name and the fixer's to build; at any path outside the caches no line covers it either.

**4. *The spawn launch-template + its install-time validation*.** `design/project-setup.md` → *The boundary that frames everything: command vs workflow* lists it among what `jigc setup` generates. No sentence read says where it is written — whether it is a file of its own or part of one S1 to S8 already name. No line is written for it; a file `setup` writes that no line covers is red, and would be this entry.

**5. `uninstall`, git's index, and a commit. Ruled on 2026-10-14: one dedicated, path-scoped commit of what it removed.** The doc says what leaves the working tree. It does not say whether the entries of the removed files leave the index, or whether the teardown commits anything. *Reading (a)*: the working tree only — the removals are the user's to commit, as `relocate`'s move is. *Reading (b)*: it may stage what it removed. Under (a) a staged removal is *git's state moved outside its allowance*; under (b) it is covered. Every acting `uninstall` cell turns on it.

- *The ruling* — the human's, of 2026-10-14, option A (*"Agree on A"*), as the orchestrating session handed it on: **the teardown lands as one dedicated, path-scoped commit of what it removed, the inverse of `setup`'s install commit, and never carries the user's own uncommitted edits at a path it touches** — it refuses over them, or commits only jigc's part: which, the fixer settles. Neither reading as put; a third.
- *How it is carried.* `U9` (§2), a commit line whose sentence is the ruling's, and a word for it in the vocabulary (§7): what the row's lines change at a tracked path lands in the commit, all of it, and no bytes of the user's that no commit held ride it. **Today's teardown, which leaves its removals in the working tree, is red in every acting cell** — *written outside its allowance*, under one finding, and no loss: what it removed from the tree, a commit still holds.

**6. The rendered message's temporary file.** `design/finalize.md` → *6. Commit*: *"`git commit -F <rendered-message-file>` (a temp file, deleted after)"*. A file that is made and deleted leaves no difference; one left behind is a write no line covers, wherever it stands — under the system's temporary directory, which the table reads, or under `.jigc/`. No reading is needed unless a cell finds one.

  *A cell found one, on 2026-10-13* — `task-discard-a-hook-that-rejects`: a sub-task's discard whose record commit a `pre-commit` hook rejects leaves `record-commit-msg.txt` in the sub-task's working area, on this tree and on the previous release. *Reading (a)*: the message's file is *deleted after* whether or not the commit landed, and a rejected commit *"leaves the record **and** the task exactly as the frame says"* (`A8`) — the file is a write a refusal may not make. *Reading (b)*: it is part of *the survivable frame on a rejecting hook*, which the doc names and does not describe — kept for the re-run the refusal prints. Under (a) the cell is *refused, and changed what a refusal may not*; under (b) it is covered, and the row gains a line with the sentence that says so, which no doc has yet. *Until ruled:* (a). A task's finalize over the same hook leaves no such file.

  *Ruled on 2026-10-14*, option A (*"Agree on A"*): **the file is deleted whether or not the commit landed**, as `design/finalize.md` says and as `task finalize` does. The leftover `record-commit-msg.txt` is a finding — [byte-table-findings.md](byte-table-findings.md) → `r1-task-discard-a-rejected-record-commit-leaves-its-message-file-in-the-working-area` — and the cell loses its mark.

**7. `task discard`, and the edge index when it settles a sub-task.**

- *The words.* `design/finalize.md` → *7. Post-commit (best-effort)* declares, of a task's finalize, that the edge index's stamp is invalidated after its commit (`T5`). `design/team-ready-state.md` gives a sub-task's discard *"the family's whole discipline rather than a second one"* and lists it — *"the reconcile preflight, the captured pre-image, the path-scoped commit of the record alone, the baseline advance, the survivable frame on a rejecting hook"* — and the edge index is not on the list.
- *Reading (a)* — the list is the discipline, whole: a discard that exits 0 writes nothing under `.jigc/index/`.
- *Reading (b)* — a command that commits a managed doc invalidates the stamp, as a finalize does; the list names what is the record doors' own.
- *What each makes of the cells.* Under (a) such a write is *written outside its allowance*; under (b) it is covered. The cells it decides are the ones in which a sub-task's discard acts.
- *Until ruled:* (a) — the row has no line for `.jigc/index/` at exit 0.

**8. A change staged at a path the command stages itself, and not in the working file.**

- *The words.* `design/finalize.md` → *5. Stage* (`T3`): *"This phase stages **jigc's own** managed artifacts into that existing index"*. `design/team-ready-state.md` → *The commit model* (`A6`): *"it stages and commits **only the record file** (`git add <record> && git commit -- <record>`, index-restricted), never sweeping the agent's in-flight index/worktree WIP"*. Neither says what becomes of a change the user staged **at that very path** and did not keep in the working file. One case of it is a declared bound — the first of the human's four of 2026-10-06: *content staged for a doc a task is updating, while the working file does not carry it, is replaced at finalize*.
- *Reading (a)* — the bound is the rule wherever jigc stages a path of its own: what the user staged there and did not keep in the file is replaced, at the milestone's record and at `.jigc/version` as at a task's doc.
- *Reading (b)* — the bound is one case, declared for a doc a task is updating. At the record, a staged change is *in-flight index WIP* the door *never sweeps*; at the stamp, it is no artifact of jigc's own: the command refuses, or keeps it.
- *What each makes of the cells.* Three cells stage such a change and come out *lost* on this tree — the bytes are in the index no longer and in no object within reach. A bound takes no cell off the list of the red ones, so all three are red under either reading; under (a) all three stand on a declared bound, under (b) one does — the task's doc — and the other two are findings.
- *Until ruled:* (b) — the two cells are marked with this entry, and the third with the bound.
- *Ruled on 2026-10-14*, option B (*"Yes agree on B"*): **the declared bound (1) stays one case, a doc a task is updating.** The staged change replaced at `.jigc/version` at `task finalize`, and at the milestone's record at `task discard`, are **findings** — each a key and a row of [byte-table-findings.md](byte-table-findings.md) — and their cells lose this entry's mark. The third keeps the bound's.

**9. `unmanage`, and a commit — two docs say two things.**

- *What the product should do, by the project's principles.* **Not commit.** The command writes only jigc's own gitignored caches — the file-state record and the edge index — so a commit would hold nothing of what it did; an empty commit, or one that sweeps the index, is git's state moved for no write of the command's. *One command behaves as its siblings* points the same way once the siblings are read for what they write: `rename` and `migrate-corpus` move committed docs, and commit them; `unmanage` moves none. **The doc sentence that would change** is `design/write-commands.md` → *`jigc rename` — the CLI-owned identity refactor (M35)*, which counts `unmanage` among *"the other top-level direct-store ops"* of a verb that is *"task-less and self-committing"*, and → *`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)*, which says `migrate-corpus` *"unlike them … has **never committed its own writes**"*. **The product change that follows**: none, if the product commits nothing — a correction of the two sentences only.
- *The words.* `design/project-setup.md` → *Flow 2 hardening*: *"un-manage touches only jigc's own index/state (asserted: the doc's bytes on disk are unchanged)"* — both gitignored. `design/write-commands.md`, the two sentences above.
- *Reading (a)* — it touches only the caches, and commits nothing: a commit is *git's state moved outside its allowance*. *Reading (b)* — it is self-committing, as its siblings: a commit of what it wrote would be covered — and it wrote nothing a commit can hold.
- *What each makes of the cells.* The cells in which `unmanage` acts; under (a) red only where it commits. *Until ruled:* (a) — the row has no commit line.
- *Ruled on 2026-10-15*, option A (*"stick with what git does"*): **`unmanage` never commits**; the two sentences of `design/write-commands.md` are corrected — a doc-only inconsistency, recorded in [byte-table-findings.md](byte-table-findings.md) → `r1-write-commands-design-counts-unmanage-among-the-self-committing-ops-where-it-never-commits`. The row is as it was; no cell moves — it committed nothing in any of its ten.

**10. A file the user committed at a path `setup` replaces — replaced on the way to the install commit.**

- *What the product should do, by the project's principles.* **Refuse without the consent, as its siblings do.** `task finalize` refuses a created doc over a file at its destination (`finalize.promote-clobber`), `milestone finalize` the same (`M2`), and `setup` itself leaves a guide somebody edited *byte-identical* (`S8`) and refuses a dirty install path; a file of the user's at `.jigc/AGENT.md`, `.jigc/version` or `.jigc/settings-entries.json` that is not jigc's own is the guide's case, committed or not. The bytes stand in history, so nothing is lost — but the install commit replaces the user's committed content without their asking, which is the exit rule's *commits content the user did not ask for*. **The doc sentence that would change**: `design/project-setup.md` → *Idempotency & irreversibility*, *"is refused before the first write while no commit holds its bytes"* — to *while its bytes are not jigc's own*, as the stamp's own clause already reads; **the product change that follows**: the guard asks the bytes, not the commit. **The other reading** keeps the doc: a committed file is held, its replacement lands in a commit the user can revert, and the table's plant rule — a planted file gone from every path, held by an object, is *git's state moved outside its allowance* — is what changes, for a line that writes the place whole and a commit that carries it.
- *The words.* `S4`'s second sentence, above; and the plan's §2: *"A planted entry is the user's"*.
- *What each makes of the cells.* Three cells — a committed file at the bootstrap, the stamp and the settings record — act, and are *git's state moved outside its allowance* under the reading in force; under the other, they would be red only by entry 1's directory. *Until ruled*: the plant rule, and the mark.
- *Ruled on 2026-10-15*, option A, in the human's words: **`setup` refuses without `--force` when the bytes at a path are not jigc's own**; the doc's sentence becomes *"while its bytes are not jigc's own"*; the three cells are one finding, `r1-setup-replaces-a-file-the-user-committed-at-a-path-it-writes-whole`. *How it is carried*: the three cells lose the mark and are listed as refused with `setup.dirty-install-path`, the refusal the same guard gives an uncommitted file there; on this tree they act, and stay *git's state moved outside its allowance*, now beside *not the listed outcome*. *The scope, as the table reads it — a note, not the ruling:* this entry asked of a path `setup` **replaces**, and that is where the table carries the ruling; a path the install **merges into** keeps what it finds, so no bytes there are replaced, and an uncommitted change there is refused already by the bytes rule of `design/assistant-adapter.md` → *The install commit's pathspec*.

**11. `setup`'s install commit runs no hook.** The commit is made with `--no-verify`; a code comment says why — it carries only bytes `setup` wrote, since the dirty-path refusal took every other — and no design doc says so. *The cells*: `setup-a-hook-that-rejects` acts, and the hook's lines stay.

- *Ruled on 2026-10-15*, option A (*"we should not work around the users existing setup"*): **`setup`'s install commit runs the user's hooks; jigc's own hook is kept from triggering itself; a rejection is a late failure, rolled back as the ruling on entry 2 has it.** One finding: [byte-table-findings.md](byte-table-findings.md) → `r1-setup-runs-no-hook-at-its-install-commit`; the doc sentence that changes is `design/assistant-adapter.md` → *`jigc setup` has assistant-neutral install responsibilities too*, to say the install commit runs the user's hooks. **Corrected with the ruling:** this entry said *what the product should do* was *as it does* — the opposite of what the human ruled; that sentence is struck. *How it is carried*: the cell is listed as refused with `setup.install-commit`, the code the door gives a failed install commit; on this tree it acts — *not the listed outcome*, under the new key.

**12. A link at the root `.gitignore`, on an unborn `HEAD`, is followed and its target committed.** `setup` merges into the file the link leads to, inside the checkout, and the install commit holds the link and that file — a file of the user's at another path, in no commit before. The doc declares that following for *"a `CLAUDE.md` or a settings file"* (§1, *Declared, and not a line here*), and not for the root `.gitignore`, whose sibling `.jigc/.gitignore` *"a link is never followed at"*. *What the product should do, by one command behaving as its siblings*: **refuse a link there, as on a born `HEAD`** — where the host file's link into the checkout is refused as dirty — or follow it for every file it merges into and say so in the doc; not the one without the other. *The cells*: `setup-head-unborn-at-the-root-ignore-a-link-to-a-file-in-the-repository` — *git's state moved outside its allowance* — under a key of its own.

- *Ruled on 2026-10-15*, option A: **`setup` refuses a link at the root `.gitignore` on any `HEAD`**; the doc lists it beside `.jigc/.gitignore` as never followed. Its one cell is a finding, the key it had. *How it is carried*: the cell is listed as refused with `setup.secrets-gitignore`, its two sibling links' refusal there; on this tree it acts, and stays *git's state moved outside its allowance*, now beside *not the listed outcome*.

**13. A file the user staged at a path `setup` merges into, on an unborn `HEAD`, with an edit beside it. Answered on 2026-10-18, by the design docs: it refuses.**

- *What was asked.* S15 lets *"an *untracked* file at a path the install **merges into**"* ride the repository's first commit on an unborn `HEAD`, and S4's second sentence refuses a file at a path the install **replaces** there too; on a born `HEAD` a file in no commit at a path it merges into is refused (the carryover gate). The entry read no sentence that says what becomes of a file **staged** at a path it merges into on an unborn `HEAD` — on an unborn `HEAD` no file is in a commit, so a file the user `git add`ed after `git init` is the one that stands there — with an edit in the working file beside it. Two readings were put: (a) S15 covers it, the file rides the first commit, and today's refusal is *not the listed outcome*; (b) it refuses as dirty, as a file in no commit does on a born `HEAD`. The entry led with (b) as what the product should do, and the cell followed (a) until ruled, marked.
- *Answered — the sentences the entry had not read.* `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*, the paragraph *The install commit's pathspec*, which S15's own sentence ends:

  > a path whose index *or* worktree bytes differ from `HEAD` **before the install writes anything** refuses — one blocking routed `setup.dirty-install-path` over the whole sorted set, with **`jigc setup --force`** the single consent … **One stated exemption, and it is per member:** on an unborn `HEAD` an *untracked* file at a path the install **merges into** rides the repository's first commit instead of refusing

  And `design/surface-contract.md` → *The carryover gate (law 1 + 3 at the commit boundary)*:

  > on an unborn `HEAD`, an *untracked* file at a path whose writer **preserves** what it finds rides the repository's first commit — the on-ramp

  A file `git add`ed after `git init` is **tracked**, and its index bytes differ from an unborn `HEAD`: the first sentence refuses it, and the exemption, which names an *untracked* file, does not reach it. Reading (b) is the docs', and no doc sentence changes. It was no silence — the review of the task `B6`'s first two commits found it.
- *How it is carried.* `setup-head-unborn-an-edit-to-the-root-ignore`, the one `S9 × E` cell, is listed as refused with `setup.dirty-install-path`, as its sibling `setup-head-unborn-at-the-root-ignore-a-change-staged-and-not-in-the-tree` is, and loses this entry's mark; it holds — exit 1, nothing changed. The key it was red under, `r1-setup-refuses-a-staged-root-ignore-on-an-unborn-head-where-an-untracked-one-rides`, retires ([byte-table-findings.md](byte-table-findings.md) → §7).
- *The one sentence that says otherwise* is a guide's, not a design doc's: `crates/cli/guides/QUICKSTART.md`, *"Every file is untracked there"*, of a repository with no commit — a doc-only finding, [byte-table-findings.md](byte-table-findings.md) → §1, `r1-quickstart-guide-says-every-file-is-untracked-before-the-first-commit-where-a-staged-one-is-tracked-and-refused`.

**14. Does a `doc` write's copy-in take a file at the home that is no doc of its type — the user's own file where the committed doc stood, or a file a live link there leads to? Two docs said two things — the question (a) of the two this entry held. Ruled on 2026-10-18: it refuses before the copy-in, staging and binding nothing.**

- *Split on 2026-10-18, after the family's review (its first and fourth findings).* Until then this entry called the place a silence and asked one question of it, where read whole the docs say two things and the entry held two questions: **(a)**, this one — whether the copy-in takes such a file at all — and **(b)**, whether a write that copied in and then refuses keeps what it copied, now entry 15. The entry had also read `W5`'s first sentence as a write refusing *"before touching bytes"* over what the source holds; that sentence is about resolving an **address** against the schema, and says nothing of what a home holds.
- *For taking what stands — the copy-in reads the home, and what it read is judged later, in the task or at the committing door.* The copy-in reads the file at the home and does not adopt a non-conformant one, which is left for the backstop:

> **A non-conformant doc is not baselined** — the gate this arm has always run. It stays `UNKNOWN` and the backstop below decides.

— `design/reconciliation.md` → *Baseline adoption*

  and a file that does not parse is non-conformant there:

> **Parse fails OR schema fails** → **conformance-block**:

— `design/reconciliation.md` → *OOB edit — the parse classifier*

  At the committing door, of the file a task copied in:

> A file that does not conform is not recorded and draws no finding here: the staged copy that replaces it is validated in-task

— `design/reconciliation.md` → *What a task copied in*

  The write row of the probing table, whole — `W3` quotes its tail and the record had left out the clause before it:

> It classifies nothing itself — the block the row promises is raised by the next sweep (`task validate`, a finalize preflight, `start --task`), never by the write verb: a second write to an already-staged doc reads the staged copy and does not look at the home again.

— `design/reconciliation.md` → *Detection timing*

  The create over a foreign file at its canonical home, in so many words — `design/auto-migration.md` → *Retire-the-foreign-original (the first destructive write)*, the path-collision guard, which seeds a migration blank and keeps the copy-in for every other caller:

  > the **idempotent-create copy-in** (`crates/engine/src/state.rs` `create` step 4, the M16 clobber-fix) seeds the migration working area by copying the *committed* doc at the canonical path in as the edit base, so a non-conformant squatter is read as the base

  > the discriminator is the migration source-path match, **never** singleton-ness, so the M16 clobber-fix stays intact for non-migration callers

  Through a link, and the create over a home with no doc body:

> and it is why the copy-in still reads through a live link: reading the doc is not the harm, and refusing the read would turn an edit into a blank mint

— `design/finalize.md` → *4. Promote managed docs*

> (4) A *plain* create over an entry with no doc body (a dangling link, a directory) still mints fresh and is refused at the committing door, not at the create — there is no create-side code that says so honestly, and adding a refusal there is a decision, not a fix.

— `design/finalize.md` → *4. Promote managed docs*

  A source that does not parse, at a write:

> A source that no longer parses answers *"is the item present?"* with *"cannot tell"*, which is not a yes: it falls through too

— `design/write-commands.md` → *Every write resolves its address before it moves bytes (M47)*

  and `design/validation.md` → *The `write.*` route split*, of the two dead-end rejects:

  > "re-run the same write" is the recovery only when the *payload* is what broke, never when the **staged source** is

  **And the human's ruling** of 2026-10-06 on the fix pass's item 23 — the declared bound `design/reconciliation.md` → *What a task copied in*, 1. restates — that an uncommitted edit made before a task's first write is carried into it, ruled:

> and over refusing the first write over an uncommitted edit, which blocks an ordinary way of working.

— `DECISIONS.md` → *2026-10-06 — Ahead of the run's opening: round 1 hunts with the seven review rows and one cross-cutting reviewer and no per-area audit, two rows get a cross-model pass, the regression set has two parts, the blind trial has three arms and a written bar for passing — and the human's ruling on each of the twenty-three items the fix pass left him (the human's rulings, one question at a time, each relayed by the orchestrator; a record only — nothing built, and no run opened)*

  A user's file put in the doc's place — its index entry dropped, or a link — is an uncommitted change at the home before the task's first write; refusing the write over it is the refusal that ruling declined, in a narrower form.
- *For refusing it, or calling it a foreign file to adopt.* `design/doc-read-surface.md` → *What it reads — committed by default; the staged copy on `--task` (the M43 revision)*, and the same words again under *`jigc doc list` — the fourth read surface (M42)*:

  > a staged working copy is jigc-written by construction, never a foreign squatter

  and `design/surface-contract.md` → *The surface style guide (the judgment tier)*:

  > the at-home squatter keeps the one-story adopt-it rule

- *What disagrees.* The first set has the copy-in stage what stands at the home — for a create over a foreign file at its canonical path, in so many words — and leaves the verdict to the task's validation and the committing door; the second says a staged copy is never a foreign squatter, and the copy-in of the user's file makes that false (the review drove it: `task validate` calls the user's file *"this task's in-flight version of the doc"*). An inconsistency between design docs, recorded as one: [byte-table-findings.md](byte-table-findings.md) → §1, `r1-auto-migration-design-copies-a-squatter-in-as-the-edit-base-where-doc-read-surface-design-says-a-staged-copy-is-never-one`.
- *What the product does.* It copies the file in. `doc add-item` splices its item into it and exits 0; `doc create` takes it as the doc it updates — *"already existed — copied in for update"*, exit 0. The other five edit leaves and `doc author` refuse after the copy: entry 15.
- *What the product should do — robust.* **Refuse at the copy-in where the bytes at the home do not parse as the doctype, staging nothing and binding nothing, and say what stands there** — an at-home squatter, whose one story is adoption, or a file of the user's to put back; **a file that parses and does not yet conform is still copied in.** *Why:* the line falls between the two sets rather than through one of them. The hand-started draft, the uncommitted hand edit of item 23 and the live-link edit path each copy in a doc that parses, and keep their copy-in; what is refused is bytes no write can work on — five of the six edit leaves and `author` already refuse on them, after copying, so there the refusal moves ahead of the copy and leaves nothing behind, and entry 15 never arises for this shape; and the staged copy is then never a foreign squatter, as the read surface says. *Its price, the human's to weigh:* a refusal new at `add-item` and `create`, which `finalize.md`'s (4) calls *a decision, not a fix*; and it stands beside item 23 — a file that no longer parses as its doc is that ruling's uncommitted edit taken to its far end, and the human may read it as the case he ruled. *The other answer:* the copy-in takes what stands; the two read-surface sentences and the style guide's are corrected to say a staged copy may hold a foreign file, which the committing door blocks.
- *Until ruled*, the reading under which more is red: **the copy-in refuses, and the write stages nothing.** *The cells* — at the user's file in the doc's place, and a link there to a file in the repository and to one outside it:
  - the three of `doc add-item` and the three of `doc create`, *not the listed outcome*, under `r1-doc-write-acts-over-a-home-that-is-no-doc-of-its-type` ([byte-table-findings.md](byte-table-findings.md) → §10, §11) — under the other reading they hold; each carries this entry's mark;
  - the fifteen of the five edit leaves that refuse after the copy carry this mark **beside entry 15's**: under this reading they are red whatever entry 15 says, and under the other their fate is entry 15's;
  - the three of `doc author` are red under every reading of either entry — entry 15 — and carry neither mark.

  **No byte of the user's is lost** in any of the twenty-four: the file stands at the home, and its bytes in a staged copy too where one was made.
- *The ruling* — the human's, of 2026-10-18, option A, as the orchestrating session handed it on: **a `doc` write refuses before its copy-in when the bytes at the doc's home do not parse as its doctype: it stages and binds nothing, and names what stands there — a squatter to adopt, or a file to restore.** A file that parses but does not yet conform is still copied in, as now — the human's ruling on item 23 stands untouched: an uncommitted edit, a hand-started draft and the live-link edit path all parse. `design/doc-read-surface.md`'s *"never a foreign squatter"* becomes true. *Why*: the line falls at bytes that can be no doc of the type at all. It is the robust answer above, taken whole.
- *How it is carried.* The reading the cells followed until ruled is the ruling's, so **no cell's listing, construction or classification moves**: the six of `add-item` and `create` stay listed as refusing, and come out *not the listed outcome*; the fifteen stay *refused, and changed what a refusal may not*; each is a finding now, of a product change — [byte-table-findings.md](byte-table-findings.md) → §10, §11 — and the mark of this entry leaves all twenty-one. A refusal is listed with the code its siblings refuse with today (`write.non-reparseable`, `write.wrong-shape`, *"carries no `# H1`"*): what the fixed product's refusal says — it is to name what stands at the home — is the fixer's, and the cell's listing follows the run that shows it. **The doc corrections the ruling asks for are findings of the run's ledger, made by nobody in this lane**: the inconsistency of the two docs ([byte-table-findings.md](byte-table-findings.md) → §1) is decided — `design/auto-migration.md`'s copy-in of a non-conformant squatter *"for non-migration callers"* gives way where the squatter does not parse, and the read surface's sentence and the style guide's stand.

**15. A `doc` write that copied a committed doc in and then refuses — does it keep what it copied? Answered at `doc author` by its own doc; at the five edit leaves that refuse no sentence spoke of a refusal that comes after the copy-in — the question (b) entry 14 held until 2026-10-18. Ruled on 2026-10-18, for every `doc` write: it keeps nothing.**

- *At `doc author` — answered.* The door's own sentences:

> persisting **once** at the end (rollback = discard the buffer; persist only on full success)

— `design/write-commands.md` → *The verbs*

> because the chain persists once at the end, the refusal rejects the **whole payload** and stages nothing (the `create_gated` pre-image is rolled back too)

— `design/write-commands.md` → *The verbs*

  and the create-gate's refusals, which come before the copy-in, say the same of the binding: *"so nothing is staged and no role is bound"* (`W5`). **What pulls the other way, and why it does not answer.** M48 met the binding a failed author leaves, and fixed the dead end it caused, not the binding:

> Fixed by a **state-derived** probe that holds however the orphan arose

— `DECISIONS.md` → *2026-08-15 — M48 complete: the audit's four findings all confirmed and fixed, and two lessons encoded*

  — and its test, `write_title_divergence::a_binding_whose_doc_is_gone_is_not_an_incumbent_at_any_minting_verb`, makes the orphan with a failed author as its fixture. And the copy-in's own paragraph says what a later copy-in does over provenance left behind:

> provenance is write-once, but a copy-in that stages the doc again after its earlier staging was undone records the bytes it read then

— `design/write-commands.md` → *The write-time `--task`-scoped barrier (M8)*

  Each says what happens where a residue stands; neither says a failed author may leave one. So `author`'s three cells are red under every reading of this entry and of entry 14 — *refused, and changed what a refusal may not*, under `r1-doc-author-refuses-over-a-home-that-is-no-doc-of-its-type-and-leaves-its-role-bound` ([byte-table-findings.md](byte-table-findings.md) → §11), held against the two sentences above — and carry no mark.
- *At the five edit leaves — open.* `remove-item`, `retitle-item`, `set-field`, `set-slot` and `rename` copy the file in and refuse — `write.non-reparseable`, `write.wrong-shape`, *"carries no `# H1` to retitle"* — leaving the copy staged, its provenance and witness written, and at `rename` the role bound. What leans towards *nothing kept*: each of `W5`'s sentences, every one of them for a refusal that comes **before** the copy-in — a lock not taken, an unreadable record, a create-only entry occupied, the minting doors' pre-check; and the sweep of every miss shape (`design/write-commands.md` → *Every write resolves its address before it moves bytes (M47)*), which asserts *"that the staged bytes are unchanged"* — of a doc staged before the miss, not of a first touch. What leans towards *kept*: the route of `write.non-reparseable`, whose escape is for *"the staged source itself"* that *"no longer parses"*, and whose `design/validation.md` paragraph says *"the state that reaches either reject is one whose task stages a doc"*; and the copy-in's paragraph above, of a staging *undone*. None says what a refusal that comes after the copy-in leaves — a silence.
- *What the product should do — robust, one rule with `author`'s.* **A write that refuses leaves the task's area as it found it**: its own copy-in rolled back, with the provenance and the witness written for it and any role it bound. *Why:* it is the rule the docs state for every refusal they name and for the batch, so one rule holds at all eight leaves; the route's *"nothing was persisted"* is then true ([byte-table-findings.md](byte-table-findings.md) → §12 closes with it); the next write reads the home again, so a repair the user makes there is what it copies, never a stale copy; and the task's witness never names bytes the task did not keep, against which its finalize would otherwise compare the home. *The other answer:* the copy survives the refusal, the route says so, and `W1` gains the copy-in on a refusal.
- *Until ruled*, the reading under which more is red: **nothing kept** — `W5`'s rule. *The cells* — the fifteen, *refused, and changed what a refusal may not*, under `r1-doc-write-refuses-over-a-home-that-is-no-doc-of-its-type-and-leaves-its-copy-staged` ([byte-table-findings.md](byte-table-findings.md) → §10), each carrying this entry's mark and entry 14's: they hold only if both are ruled the other way — the copy-in takes what stands, and a refusal keeps it.
- *Beyond the table's plants.* The question is not about a foreign file: every refusal that comes after a copy-in meets it — the fix pass's ledger row `(R3, F3)`, a conformant doc whose `set-slot` fails conformance, is one ([byte-table-findings.md](byte-table-findings.md) → §12).
- *The ruling* — the human's, of 2026-10-18, option A, as the orchestrating session handed it on: **every `doc` write that refuses leaves the task's area as it found it — the copy, its provenance, its witness and any role it bound are rolled back**: one rule with `author`'s (`design/write-commands.md` → *The verbs*, *"persist only on full success"*, and the refusal that *"stages nothing (the `create_gated` pre-image is rolled back too)"*). *Why*: the rule the docs state for every refusal they name; it makes *"nothing was persisted"* true — closing [byte-table-findings.md](byte-table-findings.md) → §12 and the fix pass's row `(R3, F3)` — and removes §13's dead end. It is the robust answer above, taken whole, and `W5`'s rule now speaks of every refusal, before the copy-in or after it.
- *How it is carried.* No line moves: `W1` to `W4` hold on no refusal, and `W5` is no line. The fifteen cells were red under this reading, and stay red, under their key, a finding now; the three of `author` were red under every reading, as they were. The mark of this entry leaves the fifteen. A refusal under entry 14's ruling comes before the copy-in, so for the table's plants the two rulings meet one shape; this one reaches every other refusal after a copy-in — the class of §12 and §13.

**16. Is the edge index's stamp invalidated by taking its file away? Answered on 2026-10-18, by the design docs: a finalize leaves the rebuild to the next read, and the file may be gone after it. It was no silence.**

- *The words.* `T5`'s and `M10`'s edge-index halves rest on *"invalidate the edge-index stamp"*, and the docs say how the next read takes it:

> - **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches). The next read rebuilds from the new HEAD.

— `design/finalize.md` → *7. Post-commit (best-effort)*

> | **finalize commit** | phase 7 invalidates the stamp; next read rebuilds against the new HEAD ([finalize.md](finalize.md)) | best-effort; cache-stamp absorbs failure |

— `design/storage.md` → *Edge index lifecycle*

  and of the cache whole, the next sentence of the same section:

> This is what makes "delete `.jigc/`'s gitignored caches, rebuild from `.md`s, nothing lost" hold for the index too

— `design/storage.md` → *Edge index lifecycle*

- *What the entry asked, and what it should have asked.* It said *"No sentence says whether the stamp is invalidated by rewriting it or by taking its file away"*, and took §7's reading of 2026-10-13 — *rebuilt*, a file made or written and none gone. The question a line turns on is another: **may a finalize leave the file absent?** The docs answer it. The rebuild is the **next read**'s (*"The next read rebuilds"*, *"next read rebuilds against the new HEAD"*), not the finalize's; the index is *"stamp-rebuildable by construction … a lost delta costs one rebuild and never a wrong answer"* (`design/storage.md` → *Concurrent writers*, quoted under `T5`); *"delete `.jigc/`'s gitignored caches … nothing lost"* holds for the index too; and no sentence asks a finalize to leave a file at `.jigc/index/edges.json`. *Rebuilt* was the frame's translation of *invalidate*, which no sentence backs; the stricter reading is for a place where the docs are silent or say two things, and here they are neither. The review of `B7`'s part A found it (its first finding).
- *What a cell met — corrected.* The entry said that under `squash: false` a boundary that lands leaves `.jigc/index/edges.json` gone, ~~where `task finalize` and the `squash: true` boundary write it again~~ — **false of the product**. Every finalize door runs one post-commit, which removes the file outright (`engine::index::invalidate`, *"Removes `<jigc_root>/index/edges.json` outright"*, called from `post_commit` in `crates/cli/src/task.rs`), and so does `jigc rename` (`crates/cli/src/rename.rs`). Driven over a built index on this tree's debug build and the previous release (`1.0.0-rc.24`, rebuilt from `91834b5e`, sha256 `accf3996…ab5d`): `task finalize` and a `squash: true` boundary each exit 0 with `edges.json.lock` alone left. What differed between the arms was the cells' leads: the three `squash: false` cells run `jigc workflow sub-task` in the worktrees, which builds the index; no other cell's leads built it, so no other cell met the file taken away.
- *How it is carried.* `T5`'s and `M10`'s edge-index halves take a word of their own, *invalidated* (§7): rebuilt, or the one stamped file, `.jigc/index/edges.json`, taken away — nothing else under `.jigc/index/`, and never the file-state halves, whose record has *"no stamp and no rebuild path"* (`design/storage.md` → *Derived caches*); on a refusal `C1` is as it was, and takes nothing away. The three `squash: false` cells lose this entry's mark and hold, on both binaries; the key `r1-milestone-finalize-under-squash-false-takes-the-edge-index-away-where-its-line-rebuilds-it` retires ([byte-table-findings.md](byte-table-findings.md) → §15). `task-finalize-a-foreign-file-under-the-edge-index`, the cell aimed at `T5`'s edge-index half, now builds the index before the command (`jigc task validate`) and holds on both binaries — the first cell of `task finalize` that meets the file taken away.
- *Bound.* Every door that reaches `index::invalidate`: two call sites in `crates/cli/src`, `post_commit` — every finalize door, `task finalize` and both of `milestone finalize`'s modes — and `jigc rename`, whose row is owed to `B8`; its edge-index line is owed the same word (a lead, [byte-table-findings.md](byte-table-findings.md) → §15). *[2026-10-19: `rename`'s row has it — `RN5`, *invalidated* (§4c) — and its cell over a built index holds ([byte-table-findings.md](byte-table-findings.md) → §24).]*

**17. What does a `squash: false` boundary's chain hold of the main checkout's index? Answered on 2026-10-18, by the design docs: nothing — `design/command-output-contract.md` says it of both modes. One sentence of `design/finalize.md` says otherwise, a doc-only inconsistency.**

- *The words the entry read.* Of `squash: true`, the WIP-safe sentence `M3` quotes — the main checkout is never mutated until the clean fast-forward, and *"leaves unrelated main-checkout WIP intact (WIP-safe)"*. Of `squash: false`, `M4`'s second sentence — each sub-task's commit *"applies **that one worktree's** staged code onto the checkout and `git commit -F`s"* its own doc — and the staging policy's, which commits the aggregate through the policy whose commit is *the git index … committed whole*:

> **`squash: false`** commits each sub-task's worktree code per-sub-task (hooks run + relay on each) and then promotes the merged docs through `IndexHonoring` over the now-clean index

— `design/finalize.md` → *Dirty-tree policy*

  Read so, a change the user staged in the main checkout rides the chain; *"the now-clean index"* presumes there is none. The entry found no sentence of `squash: false` that says what becomes of one.
- *Answered — the sentences the entry had not read.* `design/command-output-contract.md` → *Evolution posture (declared)*, the paragraph that declares `still_staged` on the landed milestone boundary, says it of **both** modes:

  > both aggregate channels build their commit from the sub-task worktrees over targeted pathspecs and land by `--ff-only` — so that entry structurally cannot ride the boundary commit

  and, two paragraphs on, of the bytes that stay:

  > These bytes are exactly where the sub-agent put them and stay staged across the boundary

  > this door neither takes nor guards the shared index

  A change staged in the shared checkout rides no commit of either mode, and stays staged. The product agrees: it says so — *"staged in the shared checkout (not committed by this boundary — the aggregate commit is built from the sub-task worktrees; these stay staged)"* — and `milestone-finalize-squash-false-a-file-staged-elsewhere` and the cell of a name git quotes ride no commit of the chain. The review of `B7`'s part A found it (its third finding), as entries 13 and 14 were found: the entry searched `design/finalize.md` and did not read `design/command-output-contract.md`.
- *What disagrees.* `design/finalize.md` → *Dirty-tree policy*, *"through `IndexHonoring` over the now-clean index"*, alone: `IndexHonoring` commits the index whole, and the contract says the boundary takes nothing of it. A doc-only finding: [byte-table-findings.md](byte-table-findings.md) → §1, `r1-finalize-design-says-the-unsquashed-aggregate-commits-the-index-where-the-output-contract-says-the-boundary-takes-none-of-it`.
- *How it is carried.* As it was — `M4` holds what the sub-tasks' worktrees staged and what the row's other lines wrote (§7, *a chain*), and not the main checkout's index. No cell moves; none was marked.
- *A neighbour, for the milestone rows.* The same contract and `design/finalize.md` → *Dirty-tree policy*, the carryover gate, say `jigc milestone create` snapshots the shared checkout's index for its aggregate; a change staged **before** the milestone is minted meets that gate, under both spellings — a need other than the cells' post-join plant (a lead, [byte-table-findings.md](byte-table-findings.md) → §15).

**18. A landed milestone boundary over a live sub-task's worktree that holds a file nobody staged — may it take the file? For a file git ignores, the human's rulings answer it: named, not refused. For one git does not ignore, the design docs and the exit rule said two things, and no ruling of the human's answered it. Ruled on 2026-10-19: the boundary moves it aside to `.jigc/displaced/<sub-task-id>/` and names it.**

*Rewritten 2026-10-19, by the third of the fixes after the review of the milestone family's task (`B7`), part B (its third finding). The entry quoted the design doc on one side and the exit rule on the other. It left out a restatement of the narration made after the rule, the sentence that names narration the worktrees' own warrant, and three rulings of the human's. It also said the decision *"predates the rule"*, where it was restated after it. Every sentence the review found is quoted below. The robust answer no longer calls a move aside the disposition the boundary already gives, and the question is narrowed to what the cell plants.*

- *The question, narrowed.* The teardown removes three kinds of byte a landed sub-task's worktree holds and no commit carries: what git ignores, unstaged edits, and untracked files git does not ignore. **What git ignores is answered by the human**, twice. At M46 their decision, as [the razor's ledger](../../../M46/razor-ledger.md) → §2f records it, is *"**DECIDED (the human, 2026-08-18): extend the narration arm to all four doors.**"*, with its cost stated over that axis alone: *"A gitignored secret inside a fan-out worktree is still destroyed; it is named first."* Their ruling on item 6 keeps it so where they widened the refusal: *"ignored files — build output — stay named, not refused"*. **The open question is the other two kinds** — an unstaged edit, or an untracked file git does not ignore, in the worktree of a sub-task the boundary lands. The cell plants the second.
- *The words for taking it, narrated.* Two passages of the design doc:

> **Bound, stated not glossed: visible, not prevented**

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

> It narrates rather than refuses **on measured evidence**

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

  — the evidence being build output in every worktree that did its job, a refusal on which *"fires on the ordinary fan-out **success** path and trains `--force` into reflex"* (the same paragraph); and `M6`'s own sentence tears the worktrees down *"on a LANDED boundary"*. Restated on 2026-10-06, two days after the exit rule:

> Files git ignores — build output — stay named by the teardown and are never refused over, and a *landed* sub-task's unstaged and untracked leftovers stay named too.

— `design/finalize.md` → *Commit-doc rendering*

> A sub-task the boundary lands is asked no bytes: its staged set was taken, and its unstaged and untracked leftovers stay named, on the M46 ruling this one does not reopen.

— `DECISIONS.md` → *2026-10-06 — `jigc milestone finalize` also refuses over a discarded sub-task's unstaged edits and untracked files that git does not ignore, under `milestone.unlanded-work`; ignored files stay named (the build of the human's ruling on the fix pass's item 6; the builder's choices inside it)*

  And narration is named the worktrees' own warrant, in the sentence that gives the task's working area the move instead:

> It is the one disposition available here: the door takes no consent flag, and it cannot narrate the loss on the *landed-boundary* warrant its siblings use, because the commit carries the promoted docs and the index and takes nothing at all out of the working area.

— `design/storage.md` → *The per-task working area (staging)*

  The narration is a pinned key of the landed envelope: `design/command-output-contract.md` → *Evolution posture (declared)* names the worktree the teardown removes, *"swept at M47 (`sub_tasks[].discarded`, bytes **destroyed**)"*.
- *Whose words those are, and when.* None of them is the human's, for these bytes.
  - The narration of unstaged and untracked bytes at a landed boundary was born at M47, as a re-scope taken mid-build through the planning gates:

> (3) The landed-path loss takes the **law-1 minimum** — warn before removing, name what was discarded — with the honest bound written down: *visible, not prevented*; the `discard`-style refusal is new surface → **M46**.

— `DECISIONS.md` → *2026-07-26 — M47 Increment 3 halt resolution: the fan-out abort stops destroying sub-agent code (a mid-build re-scope, taken through the planning gates)*

  - M46's planning refused the refusal. Its entry records the re-cut as the razor pass's, not as the human's. The guard-killer it measured is `--ignored`:

> That re-cut Fork 7 from *"build a blocking refusal + `--force` at a fourth destroying door"* (new surface, and measured as a guard-killer — `--ignored` on a refusal fires on the ordinary fan-out success path, training `--force` into reflex) to **"make the narration true over the `--ignored` axis"**

— `DECISIONS.md` → *2026-08-18 — M46 planned: the reconciliation wave, its razor guarded by agents, and four decisions taken on driven evidence*

  - The human's decision of that day, quoted above, speaks of ignored bytes.
  - The two sentences of 2026-10-06 are the builder's, inside the human's ruling on item 6, which was about a **discarded** sub-task. The sheet they ruled from, [fixer-decisions.md](../pre-opening/fixer-decisions.md) → item 6, says the fixer left a landed sub-task's leftovers to *"the human's M46 ruling that a *landed* sub-task's leftover bytes — build output — are narrated, not refused"*. That is the sheet's account of the ruling, whose recorded words are of ignored bytes, and they were asked nothing of the landed case.
  - The storage sentence is M52's (2026-09-17).
- *The words against it.* The exit rule's first clause, the human's, which the table holds every cell to (the plan's §1.1):

> no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for

— `DECISIONS.md` → *2026-10-04 — The exit rule, revised: four clauses, each with a scope and one instrument — a finding blocks the call only if it breaks a clause inside its scope (the human's)*

  Three rulings of the human's on the same boundary's other losses, after the rule. The first is on work a worktree registration holds:

> **Over take-and-name.** *Why:* take-and-name would knowingly ship an exit-0 loss of a sub-agent's deliverable

— `DECISIONS.md` → *2026-10-04 — rc.24 fix pass, fork on one of jigc's own sub-task worktree registrations that still holds work: the doors refuse, `--force` the consent (the human's decision)*

  That ruling's own sentence had *"the landed `jigc milestone finalize` **names what it drops**"*. Their ruling on items 10 and 4 replaced it for the work the boundary would drop, and refused before landing:

> **Over** reversing to the wording of the registration ruling — land at exit 0 and name what was dropped

— `DECISIONS.md` → *2026-10-06 — Ahead of the run's opening: round 1 hunts with the seven review rows and one cross-cutting reviewer and no per-area audit, two rows get a cross-model pass, the regression set has two parts, the blind trial has three arms and a written bar for passing — and the human's ruling on each of the twenty-three items the fix pass left him (the human's rulings, one question at a time, each relayed by the orchestrator; a record only — nothing built, and no run opened)*

  And for these very bytes, at a sub-task `jigc task discard` settled, they ruled the refusal over a bound:

> **Over** moving the question to `jigc task discard` (item 5, 5d) and over declaring it a bound.

— `DECISIONS.md` → *2026-10-06 — Ahead of the run's opening: round 1 hunts with the seven review rows and one cross-cutting reviewer and no per-area audit, two rows get a cross-model pass, the regression set has two parts, the blind trial has three arms and a written bar for passing — and the human's ruling on each of the twenty-three items the fix pass left him (the human's rulings, one question at a time, each relayed by the orchestrator; a record only — nothing built, and no run opened)*

  The narration is none of the bounds the human declared on 2026-10-06 and 2026-10-11. The list of declared bounds with their reach is owed and not written (`implementation/decisions-pending.md` → *The declared bounds, as one list with reach*), so no bound on it can be cited.
- *Does a ruling of the human's already answer it?* **No — it stays open**, for a file git does not ignore in a landed sub-task's worktree.
  - Their decision at M46 and their ruling on item 6 settle what git ignores.
  - Their rulings on the registrations and on items 10 and 4 refuse over a staged path or a commit the boundary would **drop**. A landed sub-task's staged set is not dropped: it lands.
  - Their ruling on item 6 is about a **discarded** sub-task. It reached them on a sheet that placed the landed case under an M46 ruling of theirs, and their words say nothing of the landed case.
  - What all three rulings share — a sub-agent's bytes the boundary would lose at exit 0 are kept, and a bound for them is declined — points one way. It is not a ruling on this case.
- *What the product does.* `jigc milestone finalize` lands, removes the worktree, and names the file — *"discarded with the fan-out worktrees (not committed, not recoverable): add-alpha-file: notes.txt (never staged)"* — at exit 0, with no `.jigc/displaced/` made. Driven on this tree's debug build and on the previous release's rebuild (`accf3996…ab5d`), the same on both (2026-10-19, [byte-table-findings.md](byte-table-findings.md) → §19's repro, as printed).
- *What the product should do — robust, revised.* **Keep the bytes and keep the success path free.**
  - *The change.* Before removing a landed sub-task's worktree, move what git neither ignores nor holds — its unstaged edits and its untracked files — aside to `.jigc/displaced/<sub-task-id>/`, and keep narrating what git ignores.
  - *Why.* The clause. The human's own reason against take-and-name, an exit-0 loss of a sub-agent's deliverable. And their refusal to declare the same bytes a bound at a discarded sub-task. The measured warrant is against a *refusal* on the success path, and was measured on `--ignored`; a move refuses nothing and loses nothing.
  - *Revised.* The entry called this *"the disposition the same boundary gives a working area's complement"*. `design/storage.md` undercuts that. It gives the working area the move *because* that door cannot use the landed-boundary warrant, and it names narration the warrant of the worktrees. So the docs chose two dispositions on purpose. The answer **changes the worktrees' warrant**; it does not extend a rule they already follow.
  - *What it changes.* That sentence of `design/storage.md`. The non-ignored half of the bound in `design/team-ready-state.md`. The last clause of the `design/finalize.md` sentence above. And the reach of two pinned keys: `sub_tasks[].discarded` would name ignored bytes only, and `committed.displaced` would carry the worktrees' moves beside the areas'. Whether a pinned key's new reach is a new mechanism, and so a halt, is the human's to say.
  - *Its price.* Build output that is not ignored lands in `displaced/`, which nothing cleans.
- *The other answers.*
  - **Refuse, item 6's shape extended to a landed sub-task.** It fires on the success path wherever a sub-agent leaves a scratch file or build output that is not ignored — the trap M46 measured, on `--ignored`.
  - **The clause yields.** The narrated loss becomes a declared bound of the 1.x line, written into the bounds' list with its reach — the answer the human declined for the same bytes at a discarded sub-task.
- *Until ruled*, the reading under which more is red: the clause. The cell `milestone-finalize-a-file-of-the-users-in-a-sub-tasks-worktree` is *lost*, under `r1-milestone-finalize-takes-a-file-nobody-staged-from-a-landed-sub-tasks-worktree`, and carries this entry's mark ([byte-table-readings.md](byte-table-readings.md) → §17), on both binaries.
- *Ruled on 2026-10-19*, option A — the human's, as the orchestrating session handed it on: **before removing a landed sub-task's worktree, the boundary moves its unstaged and untracked files that git does not ignore into `.jigc/displaced/<sub-task-id>/` and names them; what git ignores is named and removed**, as ruled at M46 and on item 6. Nothing is refused, nothing is lost. *Why*: the exit rule, and the human's *"Over take-and-name"* — an exit-0 loss of a sub-agent's deliverable; it reuses the boundary's existing move-aside. **The question is kept above, as it was put.** *How it is carried*: `M6, the worktrees moved aside` (§4), whose sentence is the ruling as [DECISIONS.md](../../../../../DECISIONS.md) records it, until the design docs say it — a place `.jigc/displaced/<sub-task>` (§7, *moved aside*). The cell, re-read under the ruling, stays *lost* on this tree's binary and on the previous release's, under its key ([byte-table-findings.md](byte-table-findings.md) → §19), a finding of the product now, and **loses its mark**; the product still takes the file with the checkout. The doc change the ruling asks for — `design/storage.md`'s sentence that names narration the worktrees' warrant, the non-ignored half of `design/team-ready-state.md`'s bound, the last clause of `design/finalize.md`'s sentence above, and the reach of `sub_tasks[].discarded` and `committed.displaced` — is a finding of the run's ledger, under the same key, not a change in this lane.

**19. A pending `.jigc/config` delta at a commit that is not the layer's first — does a finalize carry it? Two docs said two things. Ruled on 2026-10-19: a finalize never carries a change under `.jigc/config` that nobody staged.**

*Recorded 2026-10-19, by the first of the fixes after the review of the milestone family's task (`B7`), part B (its first finding), which found the question called answered by the docs where they answer it both ways.*

- *The question.* `jigc config set` writes the project config layer — `.jigc/config/manifest.yaml` — and stages nothing. Since M48 `setup`'s install commit lands the layer, so every finalize after it meets such a delta at a commit that is not the layer's first. Both doors stage the layer by its directory, `git add -- .jigc/config .jigc/.gitignore` — `task finalize` in its stage (`stage_index_honoring`, and `stage_migration` for a migration task), the milestone boundary in the overlay both its spellings commit through (`overlay_docs_commit_and_ff`), all in `crates/cli/src/task.rs` — so the delta rides the commit, and so does **any** file under `.jigc/config` nobody staged, a file of the user's included. Driven on this tree's debug build and on the previous release's rebuild, the same on both: a knob set, and in another run a note of the user's left untracked under `.jigc/config`, are each in the commit `task finalize` lands, and in the one a landed boundary lands, at exit 0 ([byte-table-findings.md](byte-table-findings.md) → §20, the repros).
- *The words for carrying it.*

> **Outside the path set by construction:** a pending `.jigc/config` delta (an ordinary code-less finalize commits it beside the doc — gap G8) and the `.jigc/version` stamp, which this arm does not refresh

— `design/finalize.md` → *The doc-only arm*

> Both wait for the next ordinary finalize.

— `design/finalize.md` → *The doc-only arm*

> the two stage functions `git add` this jigc-owned set every finalize

— `design/finalize.md` → *Rollback discipline*

- *The words against it.*

> on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),

— `design/finalize.md` → *5. Stage*

> finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay out of the commit and are surfaced

— `design/finalize.md` → *5. Stage*

> jigc's promotion (docs + first-commit config) — and finalize unions exactly those, never sweeping unstaged/untracked working-tree WIP

— `design/storage.md` → *The per-task working area (staging)*

> so the scaffolding never rides the user's first `finalize`

— `design/assistant-adapter.md` → *`jigc setup` has assistant-*neutral* install responsibilities too*

  — and the milestone boundary's *"overlays the promoted docs + first-commit config"* (`M3`) says *first-commit* too. The G8 sentence's own name for what it describes is a gap: M55's gap list has *"Code-less finalize commits anything staged during the task + unstaged .jigc/config deltas"* (`completions/artifacts/M55/gap-list.md`, G8), which the doc-only arm was settled to close (`completions/artifacts/M55/settle-log.md`, R1). And where the product itself routes a config delta, it has the user stage it: `jigc uninstall`'s refusal over the invocation log prints `jigc config set invocation-log false`, then `git -C <home> add -- .jigc/config` (`DECISIONS.md` → *2026-10-06 — `jigc uninstall`'s refusal over the invocation log says the order itself*).
- *What the product should do — robust.* **Commit no change under the config layer that nobody staged.** A finalize commits the task's index and what this transaction itself wrote; a change under `.jigc/config` it did not write — a knob set earlier with `jigc config set`, or a file of the user's — stays out of the commit and is named in the left-out list, where the ordinary finalize already names untracked and unstaged paths, and the user stages it, as the uninstall route above already tells them to. *Why*: the exit rule's first clause, *"commits content the user did not ask for"*; and one command behaving as its siblings — the doc-only arm leaves the delta out *"by construction"*, the ordinary arm promises it *"never runs `git add --all`"*, and a pathspec that is a directory is a sweep of that directory. *Its price*: a knob set and never staged reaches no teammate until the user commits it — which is how every other change to a repository's configuration travels. **The doc sentences that would change**: `design/finalize.md` → *The doc-only arm*, the parenthesis and *"Both wait for the next ordinary finalize"* (the stamp's half stays); → *Rollback discipline*, *"every finalize"*. **The product change that follows**: the three sites above stop staging the config layer by its directory, staging at most a path the transaction itself wrote there.
- *The other answer*: the delta rides, as the G8 sentence has it — and then only jigc's own. A file under `.jigc/config` nobody wrote through jigc is content the user did not ask for under either answer, so the stage would have to tell the two apart (the knobs file alone, or the layer's files `setup` and `config set` write).
- *What each makes of the cells.* `task-finalize-a-pending-config-delta` and `milestone-finalize-a-pending-config-delta`: under the first answer each is *git's state moved outside its allowance* — the commit holds a path no line wrote and no index held; under the second each holds. A file of the user's nobody staged under `.jigc/config` is red under both; no cell plants one — no kind of plant targets a carried line (`byte_cell::aimed`) — and its drive is a repro under the two keys.
- *Until ruled*, the reading under which more is red: the first. `T3, the config` and `M3, the config` are in force on the layer's first commit alone (§7, *the place's first commit*), and the two cells are red, each under a key of its door — `r1-task-finalize-commits-a-change-under-the-config-layer-nobody-staged`, `r1-milestone-finalize-commits-a-change-under-the-config-layer-nobody-staged`, one mechanism — and marked with this entry ([byte-table-readings.md](byte-table-readings.md) → §18). The two docs' disagreement is [byte-table-findings.md](byte-table-findings.md) → §1, `r1-finalize-design-says-an-ordinary-finalize-commits-a-pending-config-delta-where-its-stage-takes-the-config-layer-on-a-first-commit`.
- *Ruled on 2026-10-19*, option A — the human's, as the orchestrating session handed it on: **a finalize never carries a change under `.jigc/config` that nobody staged; it names it among what it left out.** In the human's words:

  > In general we should never add unrelated files into a commit. Also a config change need to be committed. Either when it belongs into a task or individual.

  So no unrelated file ever rides a commit, and a config change must reach a commit — with the task it belongs to, or on its own — and `jigc config set` says so when it writes. **The question is kept above, as it was put.** *How it is carried*: as the entry's first answer, which was the reading until ruled — `T3, the config` and `M3, the config` keep their condition, *the place's first commit*; the two cells, `task-finalize-a-pending-config-delta` and `milestone-finalize-a-pending-config-delta`, re-read under the ruling, stay *git's state moved outside its allowance* on this tree's binary and on the previous release's, each under its key ([byte-table-findings.md](byte-table-findings.md) → §20), a finding of the product now, and **lose their mark**. The key of the two docs' disagreement stays — `r1-finalize-design-says-an-ordinary-finalize-commits-a-pending-config-delta-where-its-stage-takes-the-config-layer-on-a-first-commit`, §1 — and the doc change the ruling asks for (the doc-only arm's parenthesis and *"Both wait for the next ordinary finalize"*, *Rollback discipline*'s *"every finalize"*, and a sentence of `jigc config set`'s that a config change must be committed) is a finding of the run's ledger, not a change in this lane. **Not ruled, and the product fix's to design**: how a config change is shown to belong to a task, so that the task's finalize may carry it. Recorded by the seventh of the fixes after the review of `B7`'s part B.

**20. A sub-task the record has settled, at `provision` — may it get a worktree? No sentence said. Ruled on 2026-10-19: no — `provision` skips it, and the milestone design is to say so.**

*Recorded 2026-10-19, by the seventh of the fixes after the review of the milestone family's task (`B7`), part B — the second fix's left-open.*

- *The question.* `MP1` and `MP2` read `<unprovisioned>`: a sub-task the record names at whose path no checkout stood. The product skips a sub-task whose item the record has settled — driven on this tree's debug build and on the previous release's rebuild (`accf3996…ab5d`): with one of two sub-tasks discarded by `jigc task discard`, `jigc milestone provision` makes the live one's worktree and no other. No line said so, so a regression that made one for the settled sub-task would have held.
- *What the docs say near it.* For skipping: the re-seed's own *"**and so is one the record has settled**"* (MA6's second sentence), which is of the area and not the worktree; and

> **"Continue" means resume, not WIP recovery.** A teammate on a fresh clone re-derives the milestone's shape (base, task list, intents, statuses) from the record and **resumes un-joined tasks from scratch**

— `design/team-ready-state.md` → *Engine capability 2 (read-back) — the committed record is the engine's authority*

  — whose *"un-joined"* takes in a discarded item too. The one sentence that names the case narrates it, as the defect the abandon's teardown was built against, in the present tense of M42:

> `milestone provision` will happily re-provision a worktree for a sub-task that was discarded, because it reads the cache rather than the live areas

— `design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling op*

  Searched for the other side — every design doc and `DECISIONS.md`, for *provision* on a line with *settled*, *discarded* or *joined*: no sentence says `provision` makes a worktree for a settled sub-task, and none that it skips one. The nearest, `design/worked-examples.md`'s *"…and the settled milestone STAYS settled"*, is of a milestone whose record is settled, which every operating door refuses (`milestone.terminal`), not of one item.
- *A question, not a finding.* The product does what the stricter reading asks, and loses nothing; what is missing is the sentence. The narrating sentence describes the state before M42's fix and is no rule.
- *What the product should do — robust.* As it does, said in the doc: `provision` makes a worktree for each sub-task whose item the record has not settled, as the re-seed makes an area — *"Joined tasks are done"*, and a discarded one is over. *The doc sentence that would change*: one beside the re-seed's, under *Engine capability 2*, and the narrating sentence marked as the M42 defect it is.
- *Until ruled*, the reading under which more is red: `<unprovisioned>` is a sub-task the record has not settled (§7), and the cell `milestone-provision-a-sub-task-the-record-has-settled` carries this entry's mark ([byte-table-readings.md](byte-table-readings.md) → §19); it holds on both binaries.
- *Ruled on 2026-10-19*, option A — the human's, as the orchestrating session handed it on: **`milestone provision` skips sub-tasks the record has settled, and the milestone design says so, as the re-seed does.** *Why*: a settled sub-task has nothing to work on. **The question is kept above, as it was put.** *How it is carried*: as the reading until ruled, which was the ruling's — `<unprovisioned>` is a sub-task the record has not settled (§7); `MP1` and `MP2` are as they were. The cell, re-read under the ruling, holds on both binaries and **loses its mark**. The doc's missing sentence, and the narrating one marked as the M42 defect it is, are a doc-only finding of the run's ledger: [byte-table-findings.md](byte-table-findings.md) → §1, `r1-team-ready-state-design-does-not-say-provision-skips-a-sub-task-the-record-has-settled`.

**21. `migrate-corpus --no-commit` — are the migrated paths left staged, or as the fold wrote them? Two docs said two things. Ruled on 2026-10-19: staged — exactly the paths the migration touched — and nothing committed.**

*Recorded 2026-10-19, with the movers family's rows (the task `B8`, its second commit).*

- *The words for staging them.*

> **`--no-commit`** performs the migration and leaves the changes staged for the operator to commit

— `design/write-commands.md` → *`jigc migrate-corpus` — the commit boundary and its two opt-outs (M42)*

- *The words for leaving them unstaged.* The verb's own design doc, whose *"today"* is the verb before it committed — *"an unstaged delete, an unstaged modify, and an **untracked** relocated file"*, the same section:

> **`--no-commit`** — leave the tree exactly as the verb leaves it today

— `design/corpus-migration.md` → *The commit boundary — the verb lands its own migration (M42)*

  and the guide, `crates/cli/guides/MIGRATING.md` → item 4: *"`--no-commit` keeps the writes in the working tree instead"*; and the build of M42, `DECISIONS.md` → its Increment 1, T3: *"`--no-commit` performs the migration and **leaves the tree exactly as HEAD-today leaves it** (T2's staging + commit suppressed)"*. Searched for more: `design/command-output-contract.md` says only that a `--no-commit` run's `commit` is `null`.
- *What the product does.* It leaves them unstaged — driven on this tree's debug build over a decision at schema version 1: exit 0, ` M docs/decisions/one-way.md`, and *"written but NOT committed (`--no-commit`) — the migrated paths are on disk and unstaged; stage and commit them yourself"*.
- *What the product should do — robust; which side is wrong.* **Stage them**: the pathspec-limited `git add` of exactly the paths the migration touched — both halves of a move — and no commit. *Why*: unstaged, a relocation is an untracked file beside an unstaged deletion, and landing it takes a `git add` of paths the operator must find — *"a half-landed migration — rewritten docs beside a removed relocation source, only some of them staged — is the failure mode that closes"* (`MIGRATING.md`, the same item); staged, the move is one `R` the operator reads with `git diff --cached` and folds into the larger commit the flag exists for, and nothing rides a commit the operator did not ask for — the flag is the asking (the human's ruling of 2026-10-19 on entry 19). **What is wrong**: `design/corpus-migration.md`'s sentence, the guide's, and the product; `design/write-commands.md` is right. *Beside the findings' §23* (the review of `B8`, its R11), which rejects **staging** a root knob beside its moves because a commit that names paths can still take one half: the exposure is the same here — a path-limited commit can take one half of a staged move — and the answer differs on purpose. `config set` was asked for a config change, and nobody asked it to leave the change in halves; `--no-commit` is the operator asking for exactly this — the migration in the index, the commit theirs to make — and an `R` in the index is the one shape in which a move reads as one change. Unstaged, the halves are not one change in any shape.
- *The other answer*: unstaged, as the product has it — the write-commands sentence changes, and the operator stages the paths the ack names.
- *Until ruled*, the reading under which more is red: unstaged. `--no-commit` is a spelling of the row that takes MG3 out of force and stages nothing (MG4); a cell of it that stages is red, and carries this entry's mark ([byte-table-readings.md](byte-table-readings.md) → §20).
- *Ruled on 2026-10-19*, option A — the human's, as the orchestrating session handed it on: **`migrate-corpus --no-commit` stages exactly the paths the migration touched — both halves of a move — and commits nothing.** `design/corpus-migration.md`'s sentence, the guide's (`crates/cli/guides/MIGRATING.md`, item 4) and the product are wrong; `design/write-commands.md` is right. **The question is kept above, as it was put.** *How it is carried*: `MG4` is a line of the spelling — the index staged at the places the row's lines wrote, and no commit (§4c), on `design/write-commands.md`'s sentence — red first: a `--no-commit` run that staged its migration had come out *git's state moved outside its allowance*. The cell `migrate-corpus-no-commit-a-decision-below-its-version`, re-read under the ruling, holds on both binaries — the product leaves the migration unstaged, and a line lets a write happen and does not demand it — and **loses its mark**. The doc and product change is a finding of the run's ledger: [byte-table-findings.md](byte-table-findings.md) → §24, `r1-migrate-corpus-no-commit-leaves-the-migration-unstaged`, held by no cell.

**22. A `migrate-corpus` whose commit a hook rejects — may it leave the migration written and staged? The design doc declares it, a builder's bound; the human's ruling on `setup`'s refusals, and the family's other committing door, say otherwise. Ruled on 2026-10-19: it rolls back.**

*Recorded 2026-10-19, with the movers family's rows (the task `B8`, its second commit).*

- *The words for leaving it.*

> *Declared bound:* the refused run's staged residue is **stated** by the door's rejection frame, not removed — a `migrate-corpus` rollback is not in M47's scope.

— `design/corpus-migration.md` → *The commit boundary — the verb lands its own migration (M42)*

  — after *"a `pre-commit` rejection leaves the migration **written and staged but unlanded**"*, and the re-run that recovers it. A bound of M47's build, and none of the human's: the list the human declared on 2026-10-06 (the plan's §1.1) does not hold it. Its origin is the build's scope, not a judgment of what a user should get:

> *Declared bound:* the refused run's staged residue is **stated** by T7's frame, not removed — a `migrate-corpus` rollback is not in this increment's approved scope.

— `DECISIONS.md` → *2026-08-03 — M47 Inc 3 T6 (N2): `migrate-corpus`' re-run recovers instead of reporting "already current"*

  *Added 2026-10-20, by the eighth of the fixes after the review of `B8` (its R3)*: the bound is the committing family's **state-truth clause** for this door, which a second decision builds on — the side of leaving it, quoted where it stands:

> `migrate-corpus` leaves the migrated bytes written and staged (and the corpus recorded unmigrated)

— `design/finalize.md` → *6. Commit*

> `migrate-corpus`' hook-cell clause says the migrated bytes are *"written and staged"*, and in the non-hook cell **the stage is exactly what failed**

— `DECISIONS.md` → *2026-09-14 — M51 Increment 2 / T5: a non-hook boundary refusal keeps the frame that was built for it (N20)*
- *The words against it.* The human's ruling of 2026-10-14 on §6, entry 2, for `setup`'s refusals:

> **every refusal of `setup` leaves the repository as it was** (caches aside, by the ruling of 2026-10-12)

— `DECISIONS.md` → *2026-10-15 — The human's six rulings on the byte table's first cut, recorded: the families `B5` to `B9` are built now; what is held reaches what a reflog names; a change staged at a path jigc stages itself is a finding beyond the one declared bound; every refusal of `setup` leaves the repository as it was; `uninstall` lands its teardown in one path-scoped commit; a rejected record commit's message file is a finding (the byte table, task `B5`, the first of its commits)*

  and the family's other door that commits its own writes, `rename` — *"on **any** pre-commit failure restores all captured bytes, the `git mv`, **and** the file-state record"* (RN6). Searched for more: `design/finalize.md`'s *Rollback discipline* is the finalize doors'; `design/validation.md`'s rollback-conflict family names `migrate-corpus` nowhere.
- *What the product does* — not driven for this entry; the cell of a hook that rejects shows it (the family's cells).
- *What the product should do — robust; which side is wrong.* **Roll back**: a rejected migration commit restores what the run wrote — the docs, a relocation's both halves, the baseline, the index — and the re-run migrates again, which the verb's idempotence makes cheap. *Why*: a refusal that leaves the repository changed is a state the user did not ask for and may commit by accident; one rule at the two doors of the family that commit; the human's ruling on `setup`'s refusals, made for the same reason. **What is wrong**: the declared bound, and the product if it stages. *Its price*: the recovery arm (*"a re-run after any rejection recovers"*) has less to recover — only a run killed mid-write.
- *The other answer*: the bound stands, the human's to declare, with its reach.
- *Until ruled*, the reading under which more is red: the refusal changes nothing. MG1, `MG1, from a prior home`, MG2 and MG3 hold on a refusal **where the run committed** — *"the run's other docs still migrate and commit"* — and on no other (§7, *it committed*); the cell of a hook that rejects is red where the migration stays written, and carries this entry's mark ([byte-table-readings.md](byte-table-readings.md) → §21).
- *Ruled on 2026-10-19*, option A — the human's, as the orchestrating session handed it on: **a `migrate-corpus` whose commit a hook rejects rolls back — every migrated doc restored, nothing left staged, the repository as it was — and the refusal names the hook's output**; to inspect what it would have committed, the operator re-runs with `--no-commit`, which by the ruling on entry 21 stages exactly the migrated paths. *Why*: `CLAUDE.md`'s invariant *writes are transactional*, the human's *every refusal of `setup` leaves the repository as it was*, and `rename`'s restore on any pre-commit failure. **What is wrong**: the declared bound in `design/corpus-migration.md`, its echo in `design/finalize.md`'s state-truth clause for this door and the second clause `DECISIONS.md` builds on it (the 2026-09-14 entry above), and the product. **The question is kept above, as it was put.** *How it is carried*: as the reading until ruled, which was the ruling's — no line moves. The cell `migrate-corpus-a-hook-that-rejects`, re-read under the ruling, stays *refused, and changed what a refusal may not* on both binaries under its key ([byte-table-findings.md](byte-table-findings.md) → §24), a finding of the product now, and **loses its mark**; the doc change folds into the same key, a finding of the run's ledger, not a change in this lane.

**23. A file the user committed at a mover's new home — is it parked in the workbench, or is the move blocked? The design answers it on purpose for these doors — parked, the settled foreign-only bound — and what makes sense for a user reverses it. Ruled on 2026-10-20, stricter than the answer put: any file at a mover's destination refuses, committed or not — `relocate` blocks that doc, a root knob refuses its whole re-point.**

*Recorded 2026-10-19, with the movers family's cells (the task `B8`, its fourth commit). Reframed 2026-10-20, by the eighth of the fixes after the review of `B8` (its R2, with R1 and the fifth fix's cells): it was framed as two docs saying two things, where the design answers it for exactly these doors and calls the answer settled; its robust answer was one doc blocked, which at a root knob strands that doc; and it named `config set docs-root` as the config door's whole answer, where `placement-root` parks.*

- *The design's answer, for these doors.* `design/reconciliation.md` → *Relocation collisions* scopes itself to *"the freeze-exempt relocation floor + `jigc config set docs-root`"*, classifies an occupant by baseline — *"a managed instance is recorded in the `file-state` map … a foreign file is not"* — and parks a committed one on purpose:

> The foreign bytes are preserved verbatim in the workbench; a committed-but-unmanaged squatter's tracked-path index slot is freed so the managed `git mv` into the vacated destination succeeds.

— `design/reconciliation.md` → *Relocation collisions — two resolutions for one collision (M39)*

> The workbench move-into is the settled **foreign-only** bound

— `design/reconciliation.md` → *Relocation collisions — two resolutions for one collision (M39)*

  — with its decision, *"The bound is settled **foreign-only** (the safe half — a foreign file has no identity/referrers)"* (`DECISIONS.md` → *2026-07-07 — M39 Increment 5 / T5: the foreign-squatter move-into-workbench arm + the two-resolutions reconcile*); the other root knob's sentence, which `CS3` quotes, *"delegating to `relocate::relocate_stranded` so a foreign file squatting the new home is parked"* (`design/storage.md` → *Placement — direct-file and root-located homes (M38)*); and the decision of 2026-08-13 that names the move for what it is and keeps it:

> **The genuine hole was `relocate`'s foreign-squatter displacement.** It `git rm --cached`s a user's committed-but-unmanaged file at the destination and renames it into the **gitignored** `.jigc/` workbench — F3's shape exactly one door over, except that here the bytes survive. Which is what makes **the ack naming it the entire safety property**: a move into a gitignored tree is indistinguishable from a deletion to anyone who does not read the output

— `DECISIONS.md` → *2026-08-13 — The latent-surface sweep: one arm, and the honest reason there is only one*

  — a fence of what the ack prints, not a ruling that the move is right; the fix pass's ledger holds the parking's other faults, fixed (`CPL-4`, a basename overwritten; `CPL-8`, a refused `placement-root` re-point that leaves its squatter parked), and none of the question.
- *The sibling's answer to the same act* — `migrate-corpus`'s, whose walk is its own and not these doors':

> the destination holds **anything else** ⇒ a genuine collision. The doc is **blocked with a route**; nothing is written, nothing is removed.

— `design/corpus-migration.md` → *The corpus walk — the placement branches become a union (M42)*

- *And the exit of a blocked doc at `relocate`*, which the answer meets: *"**`jigc relocate` takes the identical shape and for the identical reason** … Its exit is therefore **unchanged**"* (`design/validation.md` → *The M51 registrations — Increment 9: the surface batch's mints*, `ingest.unaddressable-identity`'s exit), with *"This **`blocked` is not an exit rule**"* (`design/command-output-contract.md`) — the docs choose exit 0 for a doc `relocate` blocks, which the findings judge against them (§25, `r1-relocate-refuses-a-doc-and-exits-0`).
- *What the product does* — driven by the cells: `relocate` and `config set placement-root` park the committed file under `.jigc/displaced/`, free its index entry and stage the move over it, exit 0, the file named; `config set docs-root` refuses the whole re-point, `config.repoint-failed`, in git's words — *"fatal: destination exists"* — as it refuses every squatter (§25 of the findings, `r1-config-set-refuses-a-squatter-at-a-new-home-its-doc-parks`). One mover, two answers by knob.
- *What makes sense for a user — robust; which is wrong.* A file the user committed is their content in the tree; a move that stages it out of the index into a workbench no commit carries and no clone sees takes it out of the repository at their next commit, where they asked for a doc to move and nothing to leave. The design's reason for the bound — *"a foreign file has no identity/referrers"* — holds of a file nobody tracks, not of one git tracks: a committed file has a history and readers outside the store. And `CLAUDE.md`'s invariant has a conflict *"blocked and routed to a human, never silently merged"*. So: **at `relocate`, block that doc** — named, nothing written for it, the other docs moved — and exit non-zero, as a sweep that refuses a doc does at `migrate-corpus`; **at a root knob, refuse the whole re-point**, naming the new home, the knob unchanged — a re-point that blocked the one doc and let the knob land would leave it at a home the cascade no longer resolves, and the door already refuses whole for every doc its sweep cannot move (`config.repoint-failed`, *"the re-point was undone"*). An untracked squatter stays parked, as now: it was in no commit, and the workbench keeps it. **What is wrong**: `design/reconciliation.md`'s committed-squatter sentence and the reach of its settled bound, `design/storage.md`'s *Placement* sentence where it reaches a committed file, and the product at `relocate` and `config set placement-root`. `docs-root`'s refusal is the robust end state already, in git's words — a key of its own.
- *The other answer*: the bound stands — park a committed squatter at every relocating door, `docs-root`'s sweep included — and `design/corpus-migration.md` says why its own walk blocks.
- *Until ruled*, the reading under which more is red: the move is blocked. The three cells of a file the user committed at a new home — `relocate`'s, `config set docs-root`'s and `config set placement-root`'s — are listed as refusing, exit 1, naming the new home, and carry this entry's mark ([byte-table-readings.md](byte-table-readings.md) → §22): `relocate`'s and `placement-root`'s red on both binaries, `docs-root`'s refusing having changed nothing.
- *Ruled on 2026-10-20* — the human's, as the orchestrating session handed it on, **stricter than option A** (*"block that doc"* for a committed file, *"an untracked squatter stays parked"*): **jigc never moves or overwrites a file to make room for its own.** The ruling, as `DECISIONS.md` records it:

> **Any file at a mover's destination — committed or untracked, managed or not — refuses**

— `DECISIONS.md` → *2026-10-20 — The human's two rulings of the day, recorded: jigc never moves or overwrites a file to make room for its own, so a mover refuses over anything that stands at a destination — `relocate` blocking that doc, a root knob its whole re-point — and an acting sweep exits non-zero where an act it was asked for did not happen; the parking lines go, a mover's destination is a line's place, and `relocate`'s lines hold on a refusal that names the doc it left (the byte table, task B9, the first of its commits)*

  — `relocate` blocks that doc, moves the others, names the path to clear and exits non-zero; a root knob refuses the whole re-point, the knob unchanged; a file is moved aside only from a place jigc is about to remove (entry 18's landed worktree), never from a destination; where the occupant is a doc, the refusal's route offers adopting it as well as clearing it; and **(A)**, an unmanaged doc elsewhere in the target folder blocks nothing — the output names it with the adopt route. **What is wrong**: `design/reconciliation.md` → *Relocation collisions* (the settled foreign-only bound), `design/storage.md` → *Placement*'s parking of a squatter, `config set docs-root`'s refusal in git's words (owed a jigc code and a route), and the product at `relocate` and `config set placement-root`. *Why*: the invariant *"conflicts blocked and routed to a human, never silently merged"*; a parked file is the user's bytes at a path they never chose, after a command that reported success. **The question is kept above, as it was put.** *How it is carried*: `RL3` and `CS3`, which parked a destination's occupant, are struck (§4c), and a frame fence names any line that moves aside from one file's place; the destination is a line's place, `<a-new-home>` — `RL1, at its new home`, `CS2, at its new home` — where *into its home* lands where nothing stood, in the working tree or in what git held, and what stands there is a need of the row (§7). The three marked cells lose the mark: `relocate`'s and `placement-root`'s red under their keys, `docs-root`'s refusing having changed nothing, as before. **Every other cell with an occupant at a destination** — a file of the user's, the three links, a directory, at each of the three — **is listed refusing**, exit 1, naming the new home: `relocate`'s five and `placement-root`'s five red, each door under a key of its own beside the committed file's — one mechanism, one fix closes both — and `docs-root`'s five refusing having changed nothing, the refusal the ruling asks, in git's words, a key of words. Three cells hold (A) at its doors, a doc of the user's beside each destination, listed acting. The doc and product changes are findings of the run's ledger ([byte-table-findings.md](byte-table-findings.md) → §25), not changes in this lane.

**Decided here, as a builder, and the human may overturn each:**

- **The invocation log is no line of the four rows.** `design/measurement.md` → *The in-repo invocation log*: a knob, default off, *"appends one JSONL record per `jigc` invocation to `.jigc/logs/invocations.jsonl`"*. Every command writes it while the knob is on, and no prepared state switches it on; the third of the bounds of 2026-10-06 is about exactly this file. A row gains the line, with a condition read off the project's configuration, when a cell switches the log on.
- **A directory on the way to a covered write is covered with it, and so is one that empties under a covered removal.** No doc says of `.jigc/` or `.claude/skills/jigc/` that the directory is made; U5's second sentence says it of the removal. The table reads a directory as an entry, so the rule is needed and is stated once, here.
- **The plan's first question of reading is not this record's** — *bytes no git object holds*, and how far *holds* reaches. Nothing read for these rows says which reach is meant; the plan's §7 has the three readings. **Ruled on 2026-10-14**, reading (b) (*"Agree on B"*): what a ref, a registered checkout's `HEAD`, an index or the newest stash entry reaches, **and what any reflog still names**; the bare object store is not held.

---

## 7. What a line had to be able to say

The plan leaves the allowance's vocabulary to this record: `B1` builds what these rows needed and no more. Read off §1 to §4:

| a line says | which lines needed it |
|---|---|
| a place that is **one path** of the repository | S1 to S9; U2, U3, U5; T2; C3, with one component left open for a milestone's id; M5 and A5, a milestone's record, named by the words of the next line (until 2026-10-18 left open for any record's name); W4, with one left open for a staged doc's name; MC2, MA3, MS3, MD1, under the docs root |
| a place one component of which is **the work unit the command acts on** — `<unit>`, the id the row reads off its command: `--task <id>` or else the single task that stands, for a `doc` write; the first word after the leaf, for `task finalize`, `task discard` and `milestone finalize` — or **any sub-task of that unit** — `<sub-task>`, an id the unit's committed record names among its tasks (added 2026-10-18, by the task `B7`'s second commit: a line read `.jigc/tasks/*` and reached a sibling's area, where *"a sub-agent physically cannot stage into a sibling's area"*) — or **the milestone of that unit** — `<its-milestone>`, the one milestone whose committed record names the unit among its tasks; each word a whole component or one with a fixed suffix (`<unit>.md`). A place may open with **the docs root** — `<docs-root>`, the directory the cascade resolves `docs-root` to: the project layer's value, else the team layer's, else the dev pack's default, `.` the repository's root. *A task that stands* is an area that carries its base pin (`design/storage.md` → *The per-task working area (staging)*), and a leftover directory is none; *a committed record* is `<docs-root>/milestone-records/<id>.md` with the bytes `HEAD` holds there — a record with a change nobody committed names no task, the stricter direction, since the product reads the file and conflict-blocks one edited out of band (`design/team-ready-state.md` → *Engine capability 1 (write)*, its *No-silent-overwrite discipline*: *"conflict-blocks on drift"*). (The suffix, `<its-milestone>`, the docs root and the base pin added 2026-10-18, after the review of `B7`'s part A, its fifth, sixth and seventh findings.) | W1, W4; T6; A1; M1, M6 — `<unit>`; M6's worktrees, sub-tasks' areas and what is moved aside from them — `<sub-task>`; M5 — `<docs-root>/milestone-records/<unit>.md`; A5 — `<docs-root>/milestone-records/<its-milestone>.md`; and the milestone family's (§4b): MA2, MA3, MS2, MS3, MP3, ME1, MJ1, MD1, MD4 — `<unit>`; MP2, MD4 — `<sub-task>` (and MA6, MS6, MP1, MP4, ME2, MJ2 until 2026-10-19 — the next line) |
| a place one component of which is **a work unit the command mints** — `<minted>`, alone or with a fixed suffix: an id at which nothing stood before the command, the path up to and with that component holding no entry in the manifest taken before it — *"both mints still refuse a pre-existing id before creating anything"* (MC1's third sentence). For a unit no argv word names by its id: the milestone `create` mints from a title, the sub-task `add-task` mints from an intent, those `add-from-spec` mints from a spec's criteria. ~~It cannot tell one mint from two~~ (added 2026-10-19, with the milestone family's rows). **Narrowed 2026-10-19, by the sixth of the fixes after the review of `B7`'s part B (its eighth finding)**: *nothing stood* is in the working tree **and in what git held** — the tree `HEAD` held and the index, each path with the directories above it — *"a home with nothing on disk is a free id only when git holds nothing there either"* (`design/team-ready-state.md` → *The terminal is terminal — a settled record has no workbench (M42 completion audit)*); and the id is **the one the committed record gains**, read as `HEAD` holds it after the command beside before it — an item of the unit's record (*"Appends one `tasks` item"*, MA3's first sentence), or for a command that acts on no unit a record at the home — and the place is nowhere where the command minted two. **`<each-minted>`**: the same, of any id the record gains — *"minting one sub-task per criterion"* (MS1's first sentence); it cannot tell how many the spec's criteria should make | MC1, MC2; MA1 — `<minted>`; MS1 — `<each-minted>` (and `<minted>` until the sixth fix) |
| a place one component of which is **a sub-task of the unit where a thing of it did not stand** — `<re-seeded>`: a sub-task the unit's committed record names, whose item it has not settled (its `status` leaf `joined` or `discarded`), at which nothing stood before the command, the path up to and with that component holding no entry, as under `<minted>` — *"A sub-task whose area already exists is skipped untouched, so a live session is never clobbered — **and so is one the record has settled**"* (MA6's second sentence); and `<unprovisioned>`: a sub-task the record names at whose path no linked checkout stood — *"a **registered** worktree is reused untouched"* (MP1's third), a leftover directory that is no checkout being one the door clears (added 2026-10-19, by the second of the fixes after the review of `B7`'s part B: the re-seed's lines and MP1 read `<sub-task>`, so a rewrite under a standing area or a live worktree held) **`<unprovisioned>` is of a sub-task whose item the record has not settled too since 2026-10-19**, by the seventh of the fixes after the review of `B7`'s part B — the stricter reading of §6, entry 20, then open: no sentence says `provision` skips a settled sub-task, and the product does; **ruled the same day by the human**: `provision` skips it, and the design is to say so. | MA6, MS6, MP4, ME2, MJ2, M12, MD8 — `<re-seeded>`; MP1 — `<unprovisioned>` |
| a place that is **everything under a directory** | S4; U1; T5, T6; M1, M6, M10; A1, A7; C1, C2; W1, W2; MC1, MC4, MA1, MA2, MA5, MA6, MS1, MS2, MS5, MS6, MP1, MP3, MP4, ME1, ME2, MJ1, MJ2, MJ4, MD3, MD4, MD7, MD8; M11, M12; `M3, the config`; ST1, ST2, WF1, WF2, MI1, AM1 |
| a place that is **the `pre-commit` hook in the hooks directory git resolves** — not a fixed path | S10; U4 |
| a place that is **wherever a staged doc of the unit lands** — known by its bytes, since the path is the product's to compute | T1; M2; and W3, the same home before the task holds the doc |
| **written whole** — made, or replaced by any bytes; over a directory, anything under it, a file gone included | S1, S5, S6, S8; T2; M1, M5; A5; MC2, MD1 |
| **merged into** — every line that stood there stays, in order; **and a commit the command made before its last that changes the place again holds it partway** — every line the place held before the command, and no line it does not hold after it, read off the commit's bytes (added 2026-10-19, by the seventh of the fixes after the review of `B7`'s part B, its tenth finding: *"commits the record once per sub-task"*, MS4's sentence) | S2, S3, S7, S9, S10; MA3, MS3 |
| **taken out of** — every line that stands there afterwards stood there before, in order | U2, U3, U4 |
| **removed** | S6; U1, U4, U5; T6; M6; A1; MD4 |
| **re-slugged** — gone from its name and standing under another in the same unit, whose base pin stands: a doc it did not hold, with the gone doc's lines but its `# ` title line; the one word under which a staged doc gone from a unit that stands is not *lost* (added 2026-10-18, after the doc family's review: `W4` was *removed*, which let any staged doc of any task go while its pin stood) | W4 |
| **rebuilt** — a file made or written whole, a directory made; nothing taken away (added 2026-10-13, for the ruling) | C1, C2, C3; and S4, A7, and T5's and M10's file-state halves — see the note under this table; W1, W2; and the milestone family's places it mints or re-seeds and its baselines — MC1, MC4, MA1, MA2, MA5, MA6, MS1, MS2, MS5, MS6, MP3, MP4, ME1, ME2, MJ1, MJ2, MD3, MD7, MD8; M11, M12 (MP1 until the seventh of the fixes after part B's review, below); and MJ4, the edge index a read rebuilds (added 2026-10-19, by the fourth of the fixes after the review of `B7`'s part B); ST1, MI1, AM1, the area a mint makes, and ST2, WF2, the edge index a read rebuilds (the mint family's rows) |
| **invalidated** — rebuilt, or one stamped file, named by the line, taken away: a cache whose stamp the command invalidates and whose next read rebuilds it (added 2026-10-18, by the fourth of the fixes after the review of `B7`'s part A, its first finding: §6, entry 16, answered) | T5's and M10's edge-index halves, the file `.jigc/index/edges.json` |
| **laid out** — a directory made, and no file in it (added 2026-10-18, for the ruling on §6, entry 1) | S16 |
| **a copy of a staged doc of the unit** — staged in its own task area, a sub-task's area, or its own merged set (narrowed 2026-10-18, after the review of `B7`'s part A: it had been a doc staged in *any* unit, so a sibling's doc promoted was let land) — as a regular file | T1; M2 |
| **read, and nothing written** — the place a copy is read from, which the command leaves as it stands; no commit or index may hold anything there (added 2026-10-18, for the doc family: it names where the plants that stand in a doc's place are aimed) | W3; MI2 (since the mint family's rows) |
| **provisioned** — made where nothing stood, a file or a directory; nothing that stood rewritten, nothing gone (added 2026-10-20, with the mint family's rows: *"The first … re-entry **provisions**"* — a second finds them and writes nothing) | WF1 |
| **carried as it stands** — a place the command writes nothing at, whose bytes as the working tree held them before the command a commit it makes may hold (added 2026-10-19, with the milestone family's rows; under the condition *the place's first commit* since the first of the fixes after part B's review, which gave `T3` the same lines) | `M3, the config`; `T3, the config` — each with its ignore file's half |
| **from the pinned base's tree** — a directory made where nothing stood; the checkout's own `.git` file at its root; and a regular file whose path under the checkout, its bytes and its executable bit are the ones the tree of the base the unit's committed record pins holds there — nothing changed, nothing gone, no link (added 2026-10-19, by the seventh of the fixes after the review of `B7`'s part B, the sixth fix's left-open: *"`git worktree add --detach` populates from the pinned base's *tree*"*, MP1's second sentence; it had read *rebuilt*) | MP1 |
| a whole place that is **a doc of the store** — `<a-doc>`: `<slug>.md` directly in a doctype's home directory, or a placement doctype's file, the homes resolved by the cascade before the command or after it (a root knob's re-point moves them), read off the packs' own schemas — the shipped packs', the methodology pack's where `packs.yaml` composes it, a listed pack's under the directory `packs.yaml` names, a project shadow — under the docs root and `placement-root` as `design/storage.md` has them; **a doc at an earlier home**, `<a-prior-doc>`, the same over the homes the packs' schema snapshots declare, where no current home holds the path; **the doc the address names**, `<the-doc>`, from the unit `<type>:<slug>`; **its new home**, `<the-new-doc>`, the same doctype's home under the slug the command gives — `--slug`'s, else `slugify` of the `--to` title (added 2026-10-20, by the fourth of the fixes after the review of `B8`); and **the prior home `--from` names**, `<prior-home>`, that file or anything under that directory (added 2026-10-19, with the movers family's rows: every door of the family writes the committed store where it stands) | RN2, MG1, MG5, CS2 — `<a-doc>` (and CS3, RL3 until struck on 2026-10-20); `MG1, from a prior home` — `<a-prior-doc>`; RN1 — `<the-doc>`; `RN1, at its new home` — `<the-new-doc>`; RL1 — `<prior-home>`; `RL1, at its new home`, `CS2, at its new home` — `<a-new-home>` |
| a whole place that is **a mover's destination** — `<a-new-home>`: for a doc that stood before the command where a move into its home takes it — a `<name>.md` under the prior home `--from` names, of the doctype the command names, or a doc at a doctype's home whose home the command moves — its name in that doctype's home as the cascade resolves it after the command, where that is another path (added 2026-10-20, with the first commit of `B9`, for the human's ruling on §6, entry 23: the destination had been no place, and its occupant `RL3`'s and `CS3`'s) | `RL1, at its new home`; `CS2, at its new home` |
| a place one component of which is **the task the command mints** — `<minted-task>`: the one id at which an area under `.jigc/tasks/` carries its base pin after the command, where nothing stood before, in the working tree or in what git held; where two came to carry one, nowhere (added 2026-10-20, with the mint family's rows: `<minted>` reads a mint off the committed record, and a task has none) | ST1; MI1; AM1 |
| a whole place that is **the source the command reads** — `<the-source>`: the path the first word after the leaf names, from the repository's root; it names where plants stand, and its line lets nothing happen there (added 2026-10-20, with the mint family's rows) | MI2 |
| **retitled** — the doc the address names rewritten so that its lines are the ones it held but its `# ` title line, which is the title `--to` gives; where its slug changes, standing so at its new home — `<the-new-doc>`, the slug the command gives — at which nothing stood before (in the working tree or in what git held), its own gone; read at each end, the old home gone or rewritten in place, the new one arrived at (added 2026-10-19, with the movers family's rows; at the slug the command gives, and the arrival at a place of its own, by the fourth of the fixes after the review of `B8`) | RN1; `RN1, at its new home` |
| **repointed** — a doc of the store whose lines that changed are each a ref-field of its front matter — a field its doctype's schema declares `type: ref` to the moved doc's doctype — that held the old identity of the doc **retitled** moved as one whole value, and hold the field's values with that one the new identity — `<type>:<the new slug>` — in the field grammar's canonical form (`key: value`, a list `key: [a, b]`), none added or gone; nothing where the doc did not move (added 2026-10-19, with the movers family's rows; a ref-field's whole value, not a substring of any line, by the fourth of the fixes after the review of `B8`) | RN2 |
| **into its home** — gone from its place and standing, the same entry, at its doctype's home as the cascade resolves it after the command, under its own name — the doctype the command names for a place `--from` names, and else the one whose home held the path (added 2026-10-19, with the movers family's rows); **read at both ends** — the departure at the line's place, the arrival at the destination, `<a-new-home>`, **where nothing stood before, in the working tree or in what git held** (2026-10-20, with the first commit of `B9`, for the human's ruling on §6, entry 23: until it, *where something stood there, it stands moved aside under `.jigc/displaced/`*) | RL1, CS2 — the departure; `RL1, at its new home`, `CS2, at its new home` — the arrival |
| **migrated** — a doc of the store below its doctype's current schema version (its stamp below the version its pack's manifest declares, or absent) rewritten at its home, or moved there from an earlier home, so that its stamp is the current version and every byte the migration's sentences do not let it change — its front matter's fields and values, its `- key:` bullets, its headings, its prose — stands in it as it stood, in order; what may change is read off the doctype's two schemas, the one its stamp names and the current one: the stamp, the `# ` title where the `display-title:` moved, and the `## ` heading of a section the current schema adds, once (added 2026-10-19, with the movers family's rows; every byte, not every prose line, by the third of the fixes after the review of `B8`) | MG1; `MG1, from a prior home` |
| **its knob set** — the project manifest made or rewritten so that every line that is no line of the knob the unit names stands as it stood, in order — but the `scalar:` line that holds the knob, where none stood — and one line of it names the knob, set, under `scalar:`, to the value the command names, a string as the cascade reads one (added 2026-10-19, with the movers family's rows; the value and the `scalar:` line held by the seventh of the fixes after the review of `B8`) | CS1 |
| **moved aside** — gone from its place, and standing with the same bytes under a named directory — whose components may stand for the unit and its sub-tasks as a place's do, **a component the place names too standing for the unit the bytes left**, and the directories on the way there with it (added 2026-10-19, with the first commit of `B8`, for the human's ruling on §6, entry 18: a landed sub-task worktree's leftovers go to `.jigc/displaced/<sub-task-id>/`; held to the unit the bytes left by the seventh of the fixes after the review of `B8`, where it did not ask that the sub-task a file arrives under is the one whose worktree it left) — **with the relative path the bytes had in the place they left**: under a directory's place, the path below the last component of it the line's directory names too, else below its fixed head; ~~at one file's place, the parked entry's name and what lies in it;~~ one component of it may read `<name>.<n>`, `n` from 2, where `<name>` stands after the command — *"under the first free `<name>.<n>` where that name is already taken, so nothing parked is ever replaced"* (`design/storage.md` → *Repository layout & state location*) (added 2026-10-20, by the eighth of the fixes after the review of `B8`, the seventh fix's left-open: a unit's file under its own place at another name held) | T6 — `.jigc/displaced/<unit>`; M6 — `.jigc/displaced/<sub-task>`, and `.jigc/displaced/<unit>` for the milestone's area; `M6, the worktrees moved aside` — `.jigc/displaced/<sub-task>`; ~~RL3, CS3 — **its place left** where something else stands there now, a destination a move landed on (added 2026-10-19, with the movers family's rows)~~ — **only from a place under a directory, which the command removes**: at one file's place the word moves nothing aside, and no line may say it there (2026-10-20, with the first commit of `B9`: the human's ruling on §6, entry 23, *jigc moves a file aside only from a place it is about to remove, never from a destination*) |
| **a commit** on the checked-out branch, holding **only what the row's other lines wrote** | S11; A6; T9; MC3, MA4, MS4, MD2 |
| **a commit** holding **the index as it stood, and what the row's other lines wrote** | T4 |
| **a commit** holding **what the sub-tasks' worktrees had staged, and what the row's other lines wrote** — the checkouts at a place the line names, `.jigc/worktrees/<sub-task>`, and no other (narrowed 2026-10-18, after the review of `B7`'s part A: it had been every checkout but the main one, a user's own included) | M3 (and M4 until the third fix, below) |
| **a chain** — the commits the command made are, oldest first, one for each checkout at a place the line names that staged code against its own `HEAD`, in the order of its sub-task's id, each holding what that checkout staged and nothing else, then one aggregate holding only what the row's other lines wrote (added 2026-10-18, by the third of the fixes after the review of `B7`'s part A, its fourth finding: `M4` had been `M3`'s word, so the spelling changed no verdict) | M4 |
| **a commit** holding **all of what the row's other lines changed at a tracked path, and nothing else — no edit of the user's that no commit held** (added 2026-10-15, for the ruling on §6, entry 5) | U9 |
| **the index staged** at the places the row's other lines wrote | T3 (and S11, M3, through their commits); RL2, CS5; MG4, under `--no-commit` (since the human's ruling of 2026-10-19 on §6, entry 21) |
| **a worktree's registration removed**, at a path under a named directory | U6; M7; MD6 |
| **a worktree's registration added**, with its checkout, at a path under a named place — whose components may stand for the unit and its sub-tasks (added 2026-10-19, with the milestone family's rows) — **and the checkout as the add leaves one at the pinned base**: its `HEAD` detached at the base the unit's committed record pins (the record's `base:`), its index that commit's tree, nothing staged against it (added 2026-10-19, by the sixth of the fixes after the review of `B7`'s part B, its ninth finding: *"`git worktree add --detach` populates from the pinned base's *tree*"*, MP2's sentence; the checkout's files stay MP1's) | MP2 |
| a condition read off the state before the command: **the repository has no commit** | S9 |
| a condition read off the state before the command: **the place's first commit** — the tree `HEAD` holds has nothing at the line's place, an unborn `HEAD` included (added 2026-10-19, by the first of the fixes after the review of `B7`'s part B: *"on a first commit, the project config layer"*) | `T3, the config`; `M3, the config` — each with its ignore file's half |
| a condition read off how the command ended: **it refused** (added 2026-10-13, for the ruling) | C1, C2, C3 |
| a condition read off how the command ended: **a doc of the sweep left, and named** — it exited 0, or it refused having left a doc its sweep was asked to move where it stood, and said so: an entry with a doc's name stood under the prior home `--from` names before the command and stands there after as it stood, and the command's output names its path or the path its move would have landed at (added 2026-10-20, with the first commit of `B9`, for the human's two rulings of that day: *an acting sweep does all it can, then exits non-zero*, and `relocate` *blocks that doc, moves the others, names the path to clear and exits non-zero*) | RL1, `RL1, at its new home`, RL2, RL4 |
| a condition read off how the command ended: **it committed** — it exited 0, or it refused having landed a commit of its own on the checked-out branch, and then what git tracks — a path `HEAD` held before or after the command, or the index did — stands in the working tree and the index as the commit left it: the doc the refusal reported *"joins `blocked`"* and is not written (added 2026-10-19, with the movers family's rows: *"the run's other docs still migrate and commit, and the run exits non-zero"*, and §6, entry 22, until it is ruled; held to the commit by the seventh of the fixes after the review of `B8`) | MG1, `MG1, from a prior home`, MG2, MG3 |
| a line **of a spelling** — in force only where a cell names that spelling of its leaf, and the lines the spelling replaces out of force there (added 2026-10-18, by the task `B7`'s third commit, for the human's ruling on the two spellings) | T9 (`doc-only`, replacing T2, T3, T4); M4 (`squash: false`, replacing M3); MG5 (`--dry-run`, replacing MG1, `MG1, from a prior home`, MG2, MG3); and a spelling that only takes a line out of force — `--no-commit`, replacing MG3 (added 2026-10-19, with the movers family's rows: a flag whose doc promises fewer writes) |
| **holds on a refusal** — yes or no, from a sentence | M1 (yes); C1, C2, C3 (yes, by the ruling); the re-seed's areas — MA6, MS6, MP4, ME2, MJ2, M12, MD8 (yes, since the seventh of the fixes after the review of `B7`'s part B: *"before their own guards"*); MG1, `MG1, from a prior home`, MG2, MG3 (yes, where the run committed — *"the run's other docs still migrate and commit"*); RL1, `RL1, at its new home`, RL2, RL4 (yes, where the refusal names a doc the sweep left — since the human's two rulings of 2026-10-20); every other line (no) |
| **deleted after** — not a word of a line but of the frame: the rendered message's file, by its name (`record-commit-msg.txt`, `finalize-message.tmp`) under `.jigc/`, made or rewritten, stands under no line at exit 0 or on a refusal; gone, it is as the place's line has it (added 2026-10-19, by the seventh of the fixes after the review of `B7`'s part B, its tenth finding: *"(a temp file, deleted after)"*, `design/finalize.md` → *6. Commit*, and the human's ruling on §6, entry 6) | every line whose place is a unit's area under `.jigc/` — the milestone family's MC1, MA2, MS2, MP3, ME1, MJ1, M11, MD7 among them |
| **the consent's flag**, under which a planted path named in the output may go | S14; U8; A4 |

**Corrected 2026-10-13, with the first commit of the task `B4`: `S4`, `T5` and `M10` are *rebuilt*, where they were *written whole*.** As data, *written whole* over a directory lets anything under it happen — a file of it gone too — which the acceptance's cells had no occasion to meet. Each of the three was read against its sentence again: *"initializes the in-repo project layer (`.jigc/config/`) when absent"*, *"Invalidate the edge-index stamp"* and *"Update `file-state` hashes"* let a file be made or written, and none lets one go; `M1` alone says the command **clears** its directory, and keeps the word. The entries' words did not change — each already said *written* — only the word they are carried by. **One of the three sentences can be read the other way**: a stamp can be invalidated by taking its file away. The line follows the reading under which more is red, and a cell that meets a stamp's file gone is this note's to answer for. **[Answered 2026-10-18, by the fourth of the fixes after the review of `B7`'s part A: the docs do not leave it to a reading — the rebuild is the next read's, and the file may be gone after a finalize (§6, entry 16). `T5`'s and `M10`'s edge-index halves are *invalidated*, a word of their own; their file-state halves, and `S4`, stay *rebuilt*.]**

**Read 2026-10-19, with the movers family's rows: a commit the command makes holds, at a path, what the line's word lets stand there.** A word that says what a file may become — *merged into*, *taken out of*, *repointed*, *migrated*, *its knob set* — was read at the file layer only: a file the working tree held as a commit the command made was excused under the commit's line, and the commit's path under any line whose place it was, whatever its bytes. So a merge that dropped a line and committed it held, and so would a migration that changed its prose and committed it. The commit's file is now held to the word **over what `HEAD` held there** — so a commit carries no bytes of the user's that no commit held where the word does not let them in — and is *git's state moved outside its allowance* where the word does not let it stand. What moved by it on either binary is the commit's `DECISIONS.md` entry's to say.

Three conditions a doc gives could not be put as something a manifest shows, and are words in their lines only: *where its install commit can carry the settings file* (S6), *what stands there is jigc's own copy* (S8, U5; T2), and *git tracks the settings file and the record* (U3). Each lets the command do **less** — leave a file alone — so a line without the condition allows no write the doc forbids to anything but a file somebody edited, and such a file is a plant, which no line excuses.
