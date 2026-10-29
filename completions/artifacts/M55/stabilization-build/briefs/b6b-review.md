# Review: the byte table, task B6, the doc family (`42a0776f..52fca6e6`)

Reviewer: independent and read-only. Worktree `gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`.
Commits: `26705372` (the rows), `8fba32a5` (the six edit leaves, plus the frame's base-pin rule), `ed8484f0` (create and author), `52fca6e6` (the numbers, plus the whole-table print on rc.24).

## Verdict

- **Rows (W1 to W6).** Every quoted sentence is real and stands under the heading it names. The quote fence passes.
- **`W3` / `May::CopiedFrom`.** Grounded in the docs. It cannot excuse a write or a loss.
- **Cells.** Built as their lines say. The suite is green and the expected-red list is exact.
- **The rc.24 print.** Internally consistent: 344 cells, and every this-tree count follows from it.

**Three things are wrong enough to fix before this family's records go to the human:**

1. **Entry 14 is not a silence. The docs say two things there.** The record quotes neither side's strongest sentences. That includes a human ruling (2026-10-06, item 23) that cuts against the "robust answer" the record leads with.
2. **The frame's new base-pin rule (`8fba32a5`) can hide a real loss.** A staged doc deleted inside a unit whose pin stands is now never classed *lost*. Under the four row lines that remove anything under `.jigc/tasks/*` (`T6`, `A1`, `M6`, `W4`), it *holds*. `B7`'s milestone rows will inherit this.
3. **`"nothing was persisted"` is a law-1 claim that the same invocation falsifies**, whatever entry 14's ruling. I drove it.

## What I ran

- `cargo nextest run -p jigc --test g_migration -E 'test(/byte_table::/)' --no-fail-fast` in the worktree: **42 passed**, 149 skipped, 65.6 s. This covers every fence plus the `fresh`, `bare` and `committed_singletons` arms, with the expected-red list exact.
- The worktree's `target/debug/jigc` is current. The last product-source change is `0ad081cd` (2026-10-07), and the range changes no product source.
- Rig probe 1 (`dev/jigc-rig --binary <worktree debug jigc> committed-singletons`, two-step eval, stdout only): `remove-item` over a user's file at `docs/roadmap.md`, then `add-item`, then `task validate`. Repro under F3.
- Rig probe 2 (the same state): `doc author vision` over a user's file at `VISION.md`, then `doc create vision`, then `task validate`. Repro under F3.
- `awk` tallies over `byte-table-doc-family-cells.tsv`: classes in both binary columns, per-command red counts, and doc-family cells compared column against column.
- A hygiene grep of the whole diff and the four commit messages.
- I read all four diffs in full; `doc.rs` (rows and cell lists); the `byte_cell.rs` changes (`aimed`, `Tally::covering`, `written_by_a_line`, the staged-doc rule, `staged_docs`, `under`); and the `a_staged_doc_is_accounted_for` fence.
- Design docs read: `write-commands.md` 18–100 and 238–340; `reconciliation.md` 1–176 and 201–217; `finalize.md` 139–150; `storage.md` 284–362 by grep; `auto-migration.md` 33, 82, 110 and 150–156; and `validation.md` 84–90.
- Design docs grepped: `doc-read-surface.md` and `surface-contract.md`, for squatter, unparseable, foreign file and law 1.
- `DECISIONS.md` 4253–4449 (the item-23 ruling).

## Findings

### F1: should-fix (high). Entry 14 is not a silence: the docs say two things, the record quotes neither side's decisive sentences, and its "robust answer" runs against a human ruling it does not cite

**Location**
- `completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md:797-803` (§6, entry 14) and `:551` (§3b, *What the docs do not say*).
- `byte-table-readings.md:83` (§13).
- `crates/cli/tests/byte_table/doc.rs:431` (*"no sentence decides"*).
- `byte-table-findings.md` §10 and §11.
- `DECISIONS.md:46ff`.

**The claim.** Entry 14 says *"No sentence read says what a write does where the file it would copy in is no doc of its type at all"*. Both sides are in fact written.

**For copy-in taking what stands, with the block left to a later sweep or the commit (the record's "other reading"):**
- `design/reconciliation.md:216`, *Detection timing*. The write row *"classifies nothing itself — the block the row promises is raised by the next sweep (`task validate`, a finalize preflight, `start --task`), **never by the write verb**: a second write to an already-staged doc reads the staged copy…"*. `W3` quotes the tail of this very sentence (after the colon) and leaves out the clause before it.
- `reconciliation.md:47`, *Baseline adoption*, one of the copy-in's bounds: *"**A non-conformant doc is not baselined** … It stays `UNKNOWN` and the backstop below decides."* This presupposes that the copy-in stages a non-conformant file. `:143` counts parse failure as non-conformance (*"Parse fails OR schema fails → conformance-block"*).
- `reconciliation.md:71`, *What a task copied in*: *"A file that does not conform is not recorded and draws no finding here: the staged copy that replaces it is validated in-task"*.
- `design/auto-migration.md:82`. For a foreign file at the canonical path, *"the **idempotent-create copy-in** … reads the committed non-conformant squatter in as the edit base"*. The M24 fix is gated on the migration source path, *"**never** singleton-ness, so the M16 clobber-fix stays intact for non-migration callers"*. That is exactly the `doc create` case, and the docs say it copies in.
- `design/finalize.md:139`: *"it is why the copy-in still reads through a live link: reading the doc is not the harm, and refusing the read would turn an edit into a blank mint"*.
- `design/finalize.md:143` (4): *"refused at the committing door, not at the create … adding a refusal there is a decision, not a fix"*.
- `design/write-commands.md:98`: *"A source that no longer parses answers 'is the item present?' with 'cannot tell', which is not a yes: it falls through too"*.
- `design/validation.md:86`: the `write.non-reparseable` route exists for *"if the staged source itself is what no longer parses"*.
- **The human's ruling.** `DECISIONS.md:4437` (item 23), restated at `reconciliation.md:75`: an uncommitted edit before the first write is carried, *"**Over** … refusing the first write over an uncommitted edit, which blocks an ordinary way of working."* The plant `InPlaceOf(File)` (index entry dropped, a file of the user's put in) and a link in the doc's place are both uncommitted changes at the home before the first write. The record's robust answer, "refuse before the copy-in", is a narrower form of the refusal the human declined. It may still be right, but the human must see the earlier ruling beside it.

**For refusing, or calling the file a foreign file to adopt (the record's reading in force):**
- `design/doc-read-surface.md:16` and `:197`: *"a staged working copy is jigc-written by construction, never a foreign squatter"*. Probe 1 falsifies this: the user's `notes of the user, no roadmap` is staged as `roadmap:roadmap`, and `task validate` calls it *"this task's in-flight version of the doc"*.
- `design/surface-contract.md:206`: *"the at-home squatter keeps the one-story adopt-it rule"*.

**The record also misreads `W5`.** *"A write verb resolves its addressed leaf against the declaring schema before touching bytes"* (`write-commands.md:94`) is about **address** resolution against the schema. It is not about whether the source conforms. §13's *"`W5`'s sentences have a write refuse before it touches bytes"* generalises it beyond what it says.

**Why it matters.** This is the entry-13 pattern again: the record read half a paragraph. Here the open point is real, but it is in §6's "says two things" category, not "silent". The human will rule from this text, and the text leaves out a ruling of theirs that weighs against the recommendation.

**Bound.** Of the 24 red cells, **6 turn on this question**: `add-item` ×3 and `create` ×3, the key `r1-doc-write-acts-over-a-home-that-is-no-doc-of-its-type`. The other 18 are red under both readings (F4).

**Smallest fix**
- Rewrite entry 14 and readings §13 as *"the docs say two things"*, quoting both lists above, item 23 among them, and finalize (4)'s *"a decision, not a fix"*.
- Split the entry into two questions:
  - (a) does the copy-in take a file that is no doc of its type (6 cells, still marked, the stricter reading allowed because the docs conflict);
  - (b) may a write that copied in and then refuses keep what it copied (the 18).
- Correct `doc.rs:431`'s *"no sentence decides"*.

### F2: should-fix (high, latent). The staged-doc rule keyed on the base pin can turn a lost staged doc into *holds*, in this family and in `B7`

**Location**
- `crates/cli/tests/support/byte_cell.rs:1280-1286`: `let pin = path.splitn(5,'/').take(4)…; if after.files.contains_key("<unit>/base.json") { continue; }`.
- `:809`: `(May::Removed, Some(_), None) => true` in `Tally::covering`.
- `:698-702`: `under` matches `*` against any one component.

**Trace.**
1. `staged_docs(before)` (`:1464`) collects every `.jigc/tasks/*/docs/*.md`, minus commit docs, of every unit, not only the one the command acts on.
2. Since `8fba32a5`, a staged doc whose file is gone is skipped by the loss rule whenever **its own unit's** `base.json` still stands.
3. It then reaches the file layer like any other file. There, a line `Allows::Files(Under(".jigc/tasks/*"), May::Removed)` excuses its removal.
4. Without such a line it falls to *written outside its allowance*. It is never *lost*, because the `own` exemption treats lead-in-written bytes as "never a loss".

**The lines that excuse it.** I found them with `grep -n 'May::Removed\|May::MovedAsideTo' crates/cli/tests/byte_table/*.rs`: 10 hits, of which 4 are `Removed` over the task areas.
- `task.rs:127-129` `T6` (task finalize).
- `task.rs:176-178` `A1` (task discard).
- `milestone.rs:115-117` `M6, the sub-tasks' areas` (milestone finalize).
- `doc.rs:105-107` `W4`, which is `Path(".jigc/tasks/*/docs/*")`: any staged file of any task, `provenance.json` included.

**Concrete shape (traced, not executed).**
- Two live tasks: `t1` and `t2`, with `t2` holding staged prose.
- `jigc task finalize t1` exits 0, and a regression in it removes `.jigc/tasks/t2/docs/x.md` but leaves `t2/base.json`.
- Before `8fba32a5`: *lost*. Now: *holds*, excused by `T6`.
- The same happens under `doc rename`'s row if its staged-referrer repoint (*"every staged doc's `ref` fields"*) deletes a referrer.
- `B7`'s rows (milestone `discard` and `join`) will almost certainly carry a `Removed` line over `.jigc/tasks/*`.

**Why the fence did not catch it.** The new red-first case in `byte_table.rs` `a_staged_doc_is_accounted_for` (`RE_SLUGS`) holds only the positive (a re-slug inside a standing task holds) and pin-gone (lost). It has no case for *another doc, or another unit's doc, removed while its pin stands*. *"No cell of the 208 moved"* holds because no cell has two live units.

**Bound.** By mechanism: every staged doc of a standing unit, under every row. It reads *holds* under the 4 lines listed. Elsewhere it is demoted from *lost* to *written outside its allowance*, a different clause of the closing condition.

**Smallest fix.** Key the exemption on the move, not on the pin. Give `W4` a word of its own (for example `May::ReSlugged`). Skip the loss rule only for a path that word covers, and only where the same unit gained a staged doc it did not hold before. Then add the exclusion to the fence: another doc of a standing unit removed is still *lost*.

### F3: should-fix. `"nothing was persisted"` is false of the invocation that prints it. This is a law-1 finding regardless of entry 14

**Location**
- `crates/engine/src/write.rs:6318-6330`: the route of `write.non-reparseable` and `write.target-escape`.
- `grep -rn 'nothing was persisted' crates/*/src` finds these 2 sites and no other.
- Law 1: `design/surface-contract.md:11`, *"Every claim a surface makes is generated from the thing it describes, or asserted against it."*

**Probe 1, run.**

```sh
rig=$(dev/jigc-rig --binary <worktree>/target/debug/jigc committed-singletons) || exit; eval "$rig"
$JIGC start --workflow single-task "sharpen the roadmap"
git rm --cached -q docs/roadmap.md; printf 'notes of the user, no roadmap\n' > docs/roadmap.md
$JIGC doc remove-item roadmap:roadmap#milestones/m-alpha --task sharpen-the-roadmap   # exit 1
#  blocking · write.non-reparseable … route: nothing was persisted — revise the payload …
cat .jigc/tasks/sharpen-the-roadmap/docs/roadmap:roadmap.md    # notes of the user, no roadmap
cat .jigc/tasks/sharpen-the-roadmap/docs/provenance.json       # "roadmap:roadmap": "edited-from-base", plus a copied-in witness
```

**Probe 2, run.** `doc author vision --from-file …` over a user's `VISION.md`:
- exits 1 with `write.non-reparseable` and the same route;
- the staged copy is rolled back;
- `roles.json` is **made** (`"vision": "vision:vision"`);
- `docs/provenance.json` is written (`edited-from-base` plus `copied-in`) for a doc that is no longer staged.

**What makes it worse.** The route's escape branch (*"if the staged source itself is what no longer parses"*) points at a staged source the caller never staged: this same call made it.

**Bound.**
- By mechanism: both route codes, at every copy-in door, wherever the first-touch copy-in precedes the refusal in one invocation. That reaches beyond entry 14's states, for example a conformant-but-fragile draft whose splice result does not re-parse.
- In this family's cells it is printed in **9 of the 24 red cells**: `remove-item`, `set-slot` and `author`, ×3 each, per their listed codes.
- The `author` half also contradicts `write-commands.md:43`: *"persisting **once** at the end (rollback = discard the buffer; persist only on full success)"*.

**Smallest fix.** Record it as a key of its own for the run's ledger (surface contract, law 1), independent of how entry 14 is ruled. Today it is a clause inside two §10/§11 keys whose fate hangs on the entry.

### F4: note. 18 of the 24 cells carry `Mark::Silence(14)` but do not turn on it, and the "other reading" contradicts itself

- **Location.** The `doc.rs` cells listed `refuses(1, "write.non-reparseable" | "write.wrong-shape" | "carries no # H1")`. Readings §13. Rows entry 14's *other reading*.
- **The readings' own rule.** A cell is marked when it *turns on* the question. The record itself says the 15 refusing edit cells and the 3 `author` cells *stay red* under either reading. So their marks claim a dependence that is not there.
- **`author` has a governing sentence.** Its 3 cells are held against one: `write-commands.md:43`, *persist only on full success*. §11 cites three others, not this one.
- **The inconsistency.** The "other reading" says *"a refusal then leaves what it copied in, and the route says so"*. Under that reading the 15 would be sanctioned, yet the record has them *"stay red, not the listed outcome as well"*.
- **Smallest fix.** After F1's split, mark the 18 with question (b) only. Quote `write-commands.md:43` for `author`.

### F5: note. Count slip: "Fifty-one hold" should be 48

- **Location.** `DECISIONS.md:54`. `byte-table-findings.md:379` (§10).
- **Derivation, from the print.** Each edit leaf has 17 cells: 6 on demand, 11 in the gate, 3 red. That leaves 8 gate cells holding per leaf. Across 6 leaves that is 48, which equals 66 − 18. The minting doors' *"Sixteen hold"* is right: 22 − 6.

### F6: note. "127 of the 344 are red there" counts the two not-built cells as red

- **Location.** `byte-table-doc-family.md:37`. `byte-table-install-family.md:32` (*"75 are red there … and two do not build there"*).
- **Derivation.** The print's named-binary column holds 125 red cells (14 git-moved, 17 lost, 34 not-listed, 42 refused-and-changed, 18 outside) and 2 *not run*. Per command, setup through unmanage is 73 red plus 2 not run, which makes 75.
- **Smallest fix.** Write "125 red and two not built (127 not clean)", and "73 and two" for the install family.

### F7: note. `W1` and `W4` reach wider than their sentences, and no cell can see it

- **Location.** `doc.rs` `W1` `Under(".jigc/tasks/*")` and `W4` `Path(".jigc/tasks/*/docs/*")`.
- **The gap.** The quoted sentences speak of *"the current task's working area"*. `write-commands.md:240` adds *"a sub-agent physically cannot stage into a sibling's area"*. The lines let a doc write rebuild, or under `W4` remove, any task's files.
- **Why no cell sees it.** Every cell has one live task, so none can tell. This is the vocabulary's limit (§7: *"one component left open"*), and it is what makes F2 bite.
- **Smallest fix.** One cell, or a scheduled row with its reason, with a sibling task's staged doc present (a `W1` need of kind "another unit's area"). Name the limit in §3b.

### F8: note. Create or author over the doc's file *deleted* is listed *Acts* by extending finalize (4) to "the same free home"

- **Location.** `doc.rs:1519`. Cells `doc-create-the-vision-the-doc-deleted-before-the-first-write`, `doc-author-…`.
- **The gap.** finalize (4) speaks of *"an entry with no doc body (a dangling link, a directory)"*. A deleted committed file is no entry, and the home is not free: git still holds the doc.
- **What does ground the listing.** `design/command-output-contract.md:356`: `finalize.promote-clobber` refuses *"a path git still holds — a committed doc deleted from the worktree"*. The refusal is at the committing door.
- **Smallest fix.** Cite that sentence in the comment and in §3b in place of *"the same free home"*.

## The six checks, answered

1. **Rows from the docs.** Every quote is verbatim and under its named heading (grep, plus `a_lines_quote_is_held_to_its_docs_text` green).
   - Left out: `reconciliation.md:216`'s *"It classifies nothing itself … never by the write verb"* (F1), and `write-commands.md:43`'s *"persist only on full success"* (F4).
   - Not read by the builder, by its own account: *Task origination*, the workflow-provisioned arm, `validation.md`.
   - Not read by me in full: `team-ready-state.md` and `validation.md` beyond 84–90.
2. **The two frame decisions.**
   - (a) `W3` / `CopiedFrom` is grounded: the copy-in reads the home (three quotes), and `W6`'s *"Only `finalize` produces committed changes"*. It excuses nothing at the file layer (`covering` falls to `_ => false`) or at the commit layer (`byte_cell.rs:845`). It cannot excuse a write or a loss. Its fence (`the_home_a_write_copies_in_from_is_read_and_never_written`) is green. **Sound.**
   - (b) **Can hide a loss** (F2).
3. **The cells.** Plants are aimed at each doc's real home: `docs/roadmap.md`, `CHANGELOG.md`, `VISION.md`, `docs/decisions/one-way.md`. Each plant comes after the task's start and before the first write, or after it for E. Classes and keys are right under the stricter reading. The marks are over-applied (F4).
4. **Silence 14.** Not a silence; the docs say two things (F1). The law-1 question: **yes**, regardless of the entry (F3).
5. **The print.** 344 rows, unique names. The this-tree column gives 72 red (68 gate + 4 on demand), matching `EXPECTED_RED`. The doc family's 136 cells come out the same in both columns, cell for cell. Count slips: F5, F6. The *not established* notes are replaced only in install-family *The table on 2026-10-18* and findings §7 and §8, each now pointing at the print. The older `DECISIONS.md` entries keep theirs as history.
6. **Hygiene.** The diff and the TSV hold no host path, person's name, or session or agent id. The commit messages carry the brief-mandated `Claude-Session` trailer.

## Not checked

- The rc.24 column of the print. No rc.24 binary was run, so the previous-release classifications are taken from the committed TSV as printed.
- The full gate, the `bare` arm's timings, Linux, and per-cell seconds.
- F2 on a built cell. It is traced through source and not run: adding a test would have meant editing the tree.
- That any `doc` write leaves `.jigc/index/` alone in every cell. This follows from the suite only.

<!-- end of report -->
