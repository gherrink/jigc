# The byte table — the four acceptance rows, written from the design docs

*Written 2026-10-12 on the branch `fix/rc24-route-catalog`, as task `B0` of [the pilot's plan](byte-table-plan.md) (§2, *The row and its allowance*; §8). A record: it runs nothing and decides nothing. No binary was run to learn what a command writes, and no list of files was taken from the product's source; every line below was read off a design doc, at the heading named.*

*Changed 2026-10-13, with the task `B2c` that lifts the rows into data
([install.rs](../../../../../crates/cli/tests/byte_table/install.rs), [task.rs](../../../../../crates/cli/tests/byte_table/task.rs), [milestone.rs](../../../../../crates/cli/tests/byte_table/milestone.rs)): the first entry of §6 is rewritten as the human ruled it on 2026-10-12, three lines stand in every row for it (§4a), and `M5` gains the sentence that names its place. No other line moved.*

*Changed 2026-10-18, after the review of the task `B6`'s first two commits: §6, entry 13 is answered by the design docs — two sentences it had not read — and says so; it was no silence. No line moved.*

*Changed 2026-10-18, with the second commit of the task `B6`, which builds the plant in the place of a file that stands and fills the needs that waited for it: one more place where a doc is silent is listed (§6, entry 13). No line moved.*

*Changed 2026-10-18, with the first commit of the task `B6`, which records the human's rulings on the install family's questions: entries 1, 3, 9, 10, 11 and 12 of §6 are ruled and say how each is carried; `setup`'s row gains `S16`, whose sentence is the ruling on entry 1 as [DECISIONS.md](../../../../../DECISIONS.md) records it, and §7 the word it is carried by. Entry 11's record said the opposite of what the ruling holds, and is corrected. No other line moved.*

*Changed 2026-10-15, with the third commit of the task `B5`, which brought the install family's cells: entry 1's first open point is met (§6, entry 1), entry 3 is seen, and three more places where a doc is silent are listed (§6, entries 10 to 12). No line moved.*

*Changed 2026-10-15, with the second commit of the task `B5`: the rows of `jigc ingest` (§2b) and `jigc unmanage` (§2c) are written from their docs, before a cell of either runs; one more place where docs say two things is listed (§6, entry 9). No line moved.*

*Changed 2026-10-15, with the first commit of the task `B5`, which records the human's rulings on the first cut: entries 2, 5, 6 and 8 of §6 are ruled and say how, entry 3 is read under the ruling on entry 2, and the plan's first question of reading is ruled (§6, *Decided here*); `S12` says it is no line and why; `uninstall`'s row gains `U9`, whose sentence is the ruling on entry 5 as [DECISIONS.md](../../../../../DECISIONS.md) records it, and §7 the word it is carried by. No other line moved.*

*Changed 2026-10-13, with the second commit of the task `B4`, which brought the family's cells: the sixth entry of §6 says what a cell found, and an eighth is listed — a change staged at a path the command stages itself — which three cells met. No line moved.*

*Changed 2026-10-13, with the first commit of the task `B4`: the row of `jigc task discard` is written from its docs (§3a), one more place where a doc is silent is listed (§6, entry 7), and three entries change the word they are carried by as data — `S4`, `T5` and `M10` are *rebuilt*, where they were *written whole* over a directory (§7, the note under its table). No sentence of a line moved.*

**What a row is.** The byte table compares a repository's bytes before and after one writing command. A leaf's *row* says what that command may change — its *allowance* — line by line, and each line rests on one sentence of the design doc that owns the command. This file holds the rows of the four commands the plan's acceptance needs: `jigc setup`, `jigc uninstall`, `jigc task finalize` and `jigc milestone finalize` — and, since the plan's task `B4`, the row of `jigc task discard`, which its family's cells are run under; since `B5`, the rows of `jigc ingest` and `jigc unmanage`, which the install family's cells are run under with `setup`'s and `uninstall`'s.

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

**T4 — one commit on the checked-out branch, holding the index as it stood and what T3 staged — nothing that was neither staged nor a line's.** When: always. On a refusal: no (T7).

> The commit set is therefore **the git index** — the agent's staged code plus jigc's just-staged docs/config — committed whole

— `design/finalize.md` → *5. Stage*

> finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay out of the commit and are surfaced

— `design/finalize.md` → *5. Stage*

**T5 — `.jigc/index/` and `.jigc/state/`: written — the edge index's stamp, the file-state hashes.** When: the commit landed. On a refusal: no — what a refusal may do there is `C1` and `C2` (§4a; §6, entry 1, as ruled).

> - **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches).

— `design/finalize.md` → *7. Post-commit (best-effort)*

> - **Update `file-state` hashes** for every committed managed doc.

— `design/finalize.md` → *7. Post-commit (best-effort)*

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

**A second spelling, with its own commit line — the doc-only arm.** A task whose workflow includes `step:finalize-doc-only` commits its own paths and never the index; which task that is cannot be read off a repository's bytes, so it is a spelling of the leaf that a cell names, and its row replaces T3 and T4 by this sentence. No cell of the acceptance is one.

> commits those paths alone (`git add -- <paths>` then `git commit -F <message> -- <paths>`), never the index

— `design/finalize.md` → *The doc-only arm*

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

**A5 — the milestone's committed record, `docs/milestone-records/<slug>.md`: written — one item's status flipped, and nothing else of it.** When: the task is a sub-task a committed record names. On a refusal: no (A8). The line as data says *written whole*: the vocabulary has no word for *one leaf of one item, and every other byte as it was*, so the line lets more than its sentence does — a stated shortfall of the row, not a reading of the doc. The place is `M5`'s, from the same sentence of `design/storage.md`.

> The item's `status` flips to `discarded` through the byte-stable item-leaf splice; **the header stays `active` and every sibling item is byte-untouched**

— `design/team-ready-state.md` → *`jigc task discard <sub-id>` — the per-sub-task terminal (M49)*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

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

**M4 — a second spelling: under `finalize.fan-out.squash: false`, one commit a sub-task that staged code, then the aggregate.** A cascade knob, so a spelling a cell names; no cell of the acceptance sets it.

> **`squash: false` — one commit per code-carrying sub-task in task-id order, then the parent's aggregate commit.**

— `design/finalize.md` → *`fan-out` finalize*

**M5 — the milestone's committed record, `docs/milestone-records/<slug>.md`: written — its status flipped, inside M3's commit.** When: always. On a refusal: no (M8). The second sentence is where the place comes from — added on 2026-10-13, when the line became data and had to name one; the home is under the docs root as the prepared states have it.

> the record's `status`-flip write **folds into the existing join commit** (path-added alongside the promoted docs), not a second commit

— `design/team-ready-state.md` → *The commit model — path-scoped commit at each milestone op*

> M39 graduates it to a committed **`milestone-record`** doctype (`docs/milestone-records/<slug>.md`)

— `design/storage.md` → *Source of truth*

**M6 — `.jigc/worktrees/<sub-task-id>/`, each sub-task's working area under `.jigc/tasks/`, and `.jigc/milestones/<id>/`: removed — and what jigc did not write in an area is moved aside to `.jigc/displaced/<unit-id>/`.** When: the boundary landed. On a refusal: no (M8).

> ephemeral per-sub-agent code worktrees (provisioned per fan-out; torn down on a LANDED boundary, or by `milestone discard` — an aborted finalize leaves them intact for the re-run)

— `design/storage.md` → *Repository layout & state location*

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

**M10 — `.jigc/index/` and `.jigc/state/`: written, as T5.** When: the boundary landed. On a refusal: no — what a refusal may do there is `C1` and `C2` (§4a; §6, entry 1, as ruled).

> 4. **Post-commit** runs once for the whole batch.

— `design/finalize.md` → *`fan-out` finalize*

**What the docs do not say of `milestone finalize`:** the caches on a refusal (§6, entry 1 — ruled on 2026-10-12, and carried by `C1` to `C3`). **Declared, and not a line here:** the two worktrees the boundary makes for itself and removes on every exit — the dedicated one it commits from and the detached one its `doc-code` check reads (*2. Validate*: *torn down on both exits*) — leave no difference between the two manifests where they are removed as declared; one left behind is a registration under `.jigc/worktrees/` that M7 does not cover on a refusal, and is red. The plan's §7 lists this command's commit as *not read for this plan*: it is read now, and M3 to M5 are what the docs declare it holds.

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

One list to rule from, one entry at a time. It decides nothing. Until an entry is ruled, a cell that hangs on it follows the stricter reading — the one under which more is red — and is marked. **Entries 1, 2, 3, 5, 6, 8, 9, 10, 11 and 12 are ruled** (2026-10-12, 2026-10-14 and 2026-10-15), each saying how it is carried; **entry 13 is answered by the design docs** (2026-10-18), and was no silence; entries 4 and 7 are open.

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
- *Ruled on 2026-10-15, the first open point*, option A, as the orchestrating session handed it on: **the product is unchanged; `design/assistant-adapter.md` → *Generated, minimal, regenerated* is to say that `setup` also lays out the workbench's cache directories.** The twenty cells this entry marked are marked off. *How it is carried*: `S16` (§1), a line whose sentence is the ruling's as `DECISIONS.md` records it, and a word for it, *laid out* (§7) — a directory made and no file in it. *What moved*: of the twenty, **thirteen hold**; six stay red under their own keys (entries 10 and 12, and two staged changes lost — [byte-table-findings.md](byte-table-findings.md) → §6); and `setup-a-hook-that-rejects` is red under a key of its own by the ruling on entry 11. The *seventeen* this entry projected read entry 10's three cells under the reading that ruling did not take. The doc's missing sentence is a doc-only finding: [byte-table-findings.md](byte-table-findings.md) → `r1-assistant-adapter-design-does-not-name-the-cache-directories-setup-lays-out`.
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
- *Ruled on 2026-10-15*, option A: **`setup` refuses without `--force` where the bytes at a path it replaces are not jigc's own**; the doc's sentence becomes *"while its bytes are not jigc's own"*; the three cells are one finding, `r1-setup-replaces-a-file-the-user-committed-at-a-path-it-writes-whole`. *How it is carried*: the three cells lose the mark and are listed as refused with `setup.dirty-install-path`, the refusal the same guard gives an uncommitted file there; on this tree they act, and stay *git's state moved outside its allowance*, now beside *not the listed outcome*.

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

**Decided here, as a builder, and the human may overturn each:**

- **The invocation log is no line of the four rows.** `design/measurement.md` → *The in-repo invocation log*: a knob, default off, *"appends one JSONL record per `jigc` invocation to `.jigc/logs/invocations.jsonl`"*. Every command writes it while the knob is on, and no prepared state switches it on; the third of the bounds of 2026-10-06 is about exactly this file. A row gains the line, with a condition read off the project's configuration, when a cell switches the log on.
- **A directory on the way to a covered write is covered with it, and so is one that empties under a covered removal.** No doc says of `.jigc/` or `.claude/skills/jigc/` that the directory is made; U5's second sentence says it of the removal. The table reads a directory as an entry, so the rule is needed and is stated once, here.
- **The plan's first question of reading is not this record's** — *bytes no git object holds*, and how far *holds* reaches. Nothing read for these rows says which reach is meant; the plan's §7 has the three readings. **Ruled on 2026-10-14**, reading (b) (*"Agree on B"*): what a ref, a registered checkout's `HEAD`, an index or the newest stash entry reaches, **and what any reflog still names**; the bare object store is not held.

---

## 7. What a line had to be able to say

The plan leaves the allowance's vocabulary to this record: `B1` builds what these rows needed and no more. Read off §1 to §4:

| a line says | which lines needed it |
|---|---|
| a place that is **one path** of the repository | S1 to S9; U2, U3, U5; T2; M5, A5 and C3, with one component left open for a milestone's id or its record's name |
| a place that is **everything under a directory** | S4; U1; T5, T6; M1, M6, M10; A1, A7; C1, C2 |
| a place that is **the `pre-commit` hook in the hooks directory git resolves** — not a fixed path | S10; U4 |
| a place that is **wherever a staged doc of the unit lands** — known by its bytes, since the path is the product's to compute | T1; M2 |
| **written whole** — made, or replaced by any bytes; over a directory, anything under it, a file gone included | S1, S5, S6, S8; T2; M1, M5; A5 |
| **merged into** — every line that stood there stays, in order | S2, S3, S7, S9, S10 |
| **taken out of** — every line that stands there afterwards stood there before, in order | U2, U3, U4 |
| **removed** | S6; U1, U4, U5; T6; M6; A1 |
| **rebuilt** — a file made or written whole, a directory made; nothing taken away (added 2026-10-13, for the ruling) | C1, C2, C3; and S4, T5, M10, A7 — see the note under this table |
| **laid out** — a directory made, and no file in it (added 2026-10-18, for the ruling on §6, entry 1) | S16 |
| **a copy of a staged doc**, as a regular file | T1; M2 |
| **moved aside** — gone from its place, and standing with the same bytes under a named directory | T6; M6 |
| **a commit** on the checked-out branch, holding **only what the row's other lines wrote** | S11; A6 |
| **a commit** holding **the index as it stood, and what the row's other lines wrote** | T4 |
| **a commit** holding **what the sub-tasks' worktrees had staged, and what the row's other lines wrote** | M3 |
| **a commit** holding **all of what the row's other lines changed at a tracked path, and nothing else — no edit of the user's that no commit held** (added 2026-10-15, for the ruling on §6, entry 5) | U9 |
| **the index staged** at the places the row's other lines wrote | T3 (and S11, M3, through their commits) |
| **a worktree's registration removed**, at a path under a named directory | U6; M7 |
| a condition read off the state before the command: **the repository has no commit** | S9 |
| a condition read off how the command ended: **it refused** (added 2026-10-13, for the ruling) | C1, C2, C3 |
| **holds on a refusal** — yes or no, from a sentence | M1 (yes); C1, C2, C3 (yes, by the ruling); every other line (no) |
| **the consent's flag**, under which a planted path named in the output may go | S14; U8; A4 |

**Corrected 2026-10-13, with the first commit of the task `B4`: `S4`, `T5` and `M10` are *rebuilt*, where they were *written whole*.** As data, *written whole* over a directory lets anything under it happen — a file of it gone too — which the acceptance's cells had no occasion to meet. Each of the three was read against its sentence again: *"initializes the in-repo project layer (`.jigc/config/`) when absent"*, *"Invalidate the edge-index stamp"* and *"Update `file-state` hashes"* let a file be made or written, and none lets one go; `M1` alone says the command **clears** its directory, and keeps the word. The entries' words did not change — each already said *written* — only the word they are carried by. **One of the three sentences can be read the other way**: a stamp can be invalidated by taking its file away. The line follows the reading under which more is red, and a cell that meets a stamp's file gone is this note's to answer for.

Three conditions a doc gives could not be put as something a manifest shows, and are words in their lines only: *where its install commit can carry the settings file* (S6), *what stands there is jigc's own copy* (S8, U5; T2), and *git tracks the settings file and the record* (U3). Each lets the command do **less** — leave a file alone — so a line without the condition allows no write the doc forbids to anything but a file somebody edited, and such a file is a plant, which no line excuses.
