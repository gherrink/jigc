# Review: the byte table, task B7, part A (`a81af2bb..cabe3aff`)

Reviewer: independent and read-only. Worktree `gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`.
Commits: `7f71f44b` (the three rulings recorded), `341e1cf1` (`<unit>` / `<sub-task>`), `cabe3aff` (the spelling
switch, eight cells, a new key, entries 16 and 17).

## Verdict

- **The rulings (commit 1)** are recorded faithfully. Nothing is weakened, widened or contradicted. The 21 cells
  that lost their marks follow the ruled reading. I checked that the plant's bytes do not parse as the doctype: the
  product calls them `store.unparseable`, so ruling 1's "refuse" arm is the one that applies.
- **The own-area word (commit 2)** narrows the lines it touches, and none of those narrowings can hide anything. But
  it was applied only to `Where` places. **Three sets inside the comparison are still unit-blind**, and one of them
  already turns a lost staged doc into *holds* (F2). `A5`/`M5` should not be left on `*` (F5).
- **The spelling switch (commit 3)** is real at `task finalize`. There, `T9` replaces `T2` to `T4`, which
  `finalize.md` → *The doc-only arm*, read whole, supports. At `milestone finalize` the switch is inert: `M4` allows
  exactly what `M3` does (F4).
- **The new key and entry 16 are false** (F1). The docs answer the question. The key also rests on a factual premise
  I drove false: `task finalize` and the `squash: true` boundary also take the edge index's file away. **Entry 17 is
  answered by `command-output-contract.md`**, which the record did not quote (F3).
- **Counts** check out against the source and the suite. **Hygiene** is clean.

## What I ran

- `cargo nextest run -p jigc --test g_migration byte_table --no-fail-fast` in the worktree: **44 passed**, 149
  skipped, 68 s. This covers every fence, plus the arms the filter selects, with the expected-red list exact.
- A python count over `EXPECTED_RED` in `byte_table.rs`: 17 keys and 71 cells, all unique. A grep of the marks in
  `byte_table/*.rs`: `Mark::Silence(16)` ×3 (through `AT_THE_EDGE_INDEX`), plus `Mark::Bound(1)` and
  `Mark::Bound(2)`, which makes 5. The `(353, 269, 84)` count is fenced in `every_cell_has_its_row_…`, which is green.
- Rig probes, all on the worktree's `target/debug/jigc`. The range changes no product source; the last product change
  is `0ad081cd`. Each probe used a two-step eval with stdout only.
  - **P1**: the plant's bytes under `doc show` and `validate`.
  - **P2**: the findings' §15 repro, with a staged file added.
  - **P3**: a `squash: true` boundary.
  - **P4**: a `task finalize`, with the index built first.
- `rustfmt --check --edition 2024` on each family file, at both the base and HEAD.
- I read all three diffs whole, and these files at HEAD:
  - `byte_cell.rs`: `Acting`, `placed`, `covering`, `written_by_a_line`, the commit and index layers, the loss loop
    and `staged_docs`.
  - `task.rs`: `T1` to `T9`, `A1` and `A5`.
  - `milestone.rs`, whole.
  - `doc.rs`: `W1` and `W4`.
- In the product I read `engine/src/index.rs:172-188` (`invalidate`), `cli/src/task.rs:7972-7990` (`post_commit`),
  `engine/src/state.rs:1725-1737` (`list_active_task_ids`) and `cli/src/doc.rs:6700-6760`.
- In the docs I read `finalize.md` → *5. Stage*, *The doc-only arm* (whole, 327-337), `:210-214`, `:242`, `:248` and
  `:297-303`; `storage.md` → *Derived caches* and *Edge index lifecycle* (310-356), `:11`, `:214` and `:294`;
  `command-output-contract.md:540-546`; `write-commands.md:240`; `team-ready-state.md:115`. I also grepped five
  design docs for `invalidat`, `stay staged`, `shared checkout` and `now-clean`.

## Findings

### F1: should-fix (high). Entry 16 is answered by the docs. Its three cells and the key `r1-milestone-finalize-under-squash-false-takes-the-edge-index-away-where-its-line-rebuilds-it` are false, and the key's premise is false of the product

**Location.**
- `crates/cli/tests/byte_table/milestone.rs:552` (`AT_THE_EDGE_INDEX`), `:592,:605,:618`, and `M10, the edge index`
  at `:212-213` (`Where::Under(".jigc/index"), May::Rebuilt`).
- `byte_table.rs` `EXPECTED_RED`, the new last entry.
- `byte-table-acceptance-rows.md` §6, entry 16.
- `byte-table-findings.md` §15.
- `byte-table-readings.md` §15 and its three table lines.
- `DECISIONS.md`, the `cabe3aff` entry.

**The docs answer it.** Entry 16 says *"No sentence says whether the stamp is invalidated by rewriting it or by taking
its file away"*. The question that matters is different: may finalize leave the file absent? The docs say yes, and
nowhere say it writes the file.
- `design/finalize.md:210`: *"**Invalidate the edge-index stamp** … The next read rebuilds from the new HEAD."* The
  rebuild belongs to the next **read**, not to finalize.
- `design/finalize.md:214`: *"the system reads correctly because the edge-index stamp invalidates eventually"*.
- `design/storage.md:354`: *"phase 7 invalidates the stamp; next read rebuilds against the new HEAD … best-effort;
  cache-stamp absorbs failure"*.
- `design/storage.md:356`: *"delete `.jigc/`'s gitignored caches, rebuild from `.md`s, nothing lost" hold for the
  index too*.
- `design/storage.md:328`: the index is *"stamp-rebuildable by construction … a lost delta costs one rebuild and
  never a wrong answer"*.

No sentence asks finalize to leave a file at `.jigc/index/edges.json`. The *Rebuilt* word ("a file made or written,
none gone") is the frame's translation of "invalidate", made in §7 on 2026-10-13, and no sentence backs it. The
stricter-reading rule applies only where the docs are silent or conflict (the b6a review's F1). Here they are neither.

**The premise is false.** The record (§6 entry 16, findings §15) says *"`task finalize` and the `squash: true`
boundary write the file again"*. They do not.
- Every finalize door runs one `post_commit` (`crates/cli/src/task.rs:7972`). It calls
  `engine::index::invalidate` (`:7984`), which *"Removes `<jigc_root>/index/edges.json` outright"*
  (`crates/engine/src/index.rs:176-188`). `jigc rename` calls it too (`crates/cli/src/rename.rs:990`).
- I drove both of the "other" doors, each with the index built before the command (P3 and P4 below). Both exit 0, and
  only `edges.json.lock` is left after.
- The existing `T5`/`M10` cells hold only because their leads never build the index before the command. The three
  `squash: false` cells meet a built index because their leads run `jigc workflow sub-task` in the worktrees. This is
  fixture-topology masking: the "difference between arms" is the leads, not the product.

**Bound.** By mechanism: every door that reaches `index::invalidate`. A grep of `crates/cli/src` finds 2 call sites
(`task.rs:7984` inside `post_commit`, and `rename.rs:990`). Under the stricter reading, every cell of `task finalize`,
of both milestone modes and of `jigc rename` whose leads built the index would be red. Under the docs' reading, none
is.

**Smallest correct fix.**
- Give the edge-index halves (`T5`, `M10`) a word that lets **`.jigc/index/edges.json` be written or taken away**.
  Do not widen `May::Rebuilt` globally: `T5`/`M10`'s file-state halves sit under `.jigc/state`, whose record has *"no
  stamp and no rebuild path"* (`storage.md:314`).
- Answer entry 16 by the sentences above, and drop `Mark::Silence(16)` from the three cells. They then hold.
- Retire the key from `EXPECTED_RED` and from findings §15. Delete readings §15 and its three table lines. The counts
  become 68 under 16 keys, with 2 marked cells.

**Repro.**

```zsh
# P4 — task finalize takes the file away too (from the worktree's root; BIN = target/debug/jigc, absolute)
rig=$(dev/jigc-rig --binary "$BIN" committed-singletons) || exit; eval "$rig"; cd "$REPO"
$JIGC start --workflow single-task "add a file" > /dev/null; printf 'x\n' > a.txt; git add a.txt
$JIGC doc set-field commit:add-a-file#type --value feat --task add-a-file > /dev/null
printf 'add a file\n' > ../s; $JIGC doc set-slot commit:add-a-file#summary --from-file ../s --task add-a-file > /dev/null
$JIGC task validate add-a-file > /dev/null 2>&1; ls .jigc/index/      # edges.json  edges.json.lock
$JIGC task finalize add-a-file > /dev/null 2>&1; echo $?; ls .jigc/index/   # 0 — edges.json.lock alone

# P3 — so does a squash: true boundary
rig=$(dev/jigc-rig --binary "$BIN" fresh) || exit; eval "$rig"; cd "$REPO"; m=squash-default
$JIGC milestone create "Squash default" > /dev/null
$JIGC milestone add-task $m "Add alpha file" > /dev/null; $JIGC milestone add-task $m "Add beta file" > /dev/null
$JIGC milestone provision $m > /dev/null; $JIGC milestone execute $m > /dev/null
for t in add-alpha-file add-beta-file; do (cd .jigc/worktrees/$t && $JIGC workflow sub-task --task $t > /dev/null && printf 'code\n' > $t.txt && git add $t.txt); done
$JIGC milestone join $m > /dev/null; ls .jigc/index/                  # edges.json  edges.json.lock
$JIGC milestone finalize $m > /dev/null 2>&1; echo $?; ls .jigc/index/ # 0, one commit — edges.json.lock alone
```

### F2: should-fix (high, latent; pre-existing mechanism, in commit 2's charge). Three sets inside the comparison are still unit-blind. One turns a lost staged doc into *holds*, and one lets a commit carry a sibling's prose

Commit 2 narrowed `Where` places. A line's allowance is also read through sets that `Acting` never touches. I derived
the class by reading every site that evaluates an allowance: `placed`, `covering`, `written_by_a_line`, the
commit-layer `as_it_stood`, and the loss loop. Three such sets exist:

1. **`staged_docs`** (`byte_cell.rs:1673-1695`). It takes every unit's `.jigc/tasks/*/docs`. **When any milestone's
   `merged/docs` holds a `.md`, it returns *only* those** (`:1690-1694`). Two consumers:
   - **The loss loop** (`:1485`). Suppose a repo holds a joined milestone, and `jigc task finalize t1` regresses: it
     drops t1's staged doc without promoting it. The loop never examines t1's docs. The area's removal is excused by
     `T6` (`.jigc/tasks/<unit>`, `Removed`). The cell reads ***holds*** over a lost doc. Meanwhile the correct
     product, which promotes t1's doc, reads *written outside* because of item 2. The verdicts are inverted. `A1`
     (task discard) behaves the same way.
   - **`Tally.staged`** (`:1157`), which feeds `May::CopyOfAStagedDoc` at `:979` (file layer) and `:1014` (commit
     layer). See item 2.
2. **`CopyOfAStagedDoc`** (`T1` at `task.rs:50`, `M2` at `milestone.rs:63`). `Where::AStagedDocsHome` is "anywhere
   not under `.git`" (`:927`), and the blob is matched against item 1's set.
   - Under `task finalize t1`, a regression that also promotes sibling t2's staged doc is excused at the file layer,
     the index layer and the commit layer. It holds.
   - Under `milestone finalize m1`, another milestone's merged docs are excused the same way.
   - The record's own words for `T1` are *"wherever a staged doc **of the unit** lands"* (§7), and its quote is *"from
     **the** working area"*.
3. **`Holding::TheWorktrees`** (`byte_cell.rs:1433`; `M3` at `milestone.rs:74`, `M4` at `:95`). It accepts any
   non-main checkout's staged bytes, including a user's own linked worktree (the `Plant::Worktree` kind). A boundary
   that committed a user's worktree's staged change would be excused.

**Bound.** Three sets, found by the derivation above. Item 1's loss path is traced through the source and not run: a
cell needs a joined milestone beside an ordinary task, and building one means editing the tree. Items 2 and 3 are also
traced, not run. The mechanism dates from `fc2b13b6` (B1), not from this range. But the brief asked whether the
own-area words can hide a loss *anywhere*, and the milestone family is where a joined `merged/` stands beside other
areas.

**Smallest correct fix.**
- Key `staged_docs` on `Acting`: the unit's own `.jigc/tasks/<unit>/docs` plus `<sub-task>` areas, plus
  `.jigc/milestones/<unit>/merged/docs`, for `Tally.staged`.
- Keep the loss loop over **every** unit's docs **and** every milestone's merged docs: the union, never the
  either/or.
- Restrict `TheWorktrees` to checkouts at `.jigc/worktrees/<sub-task>`.
- Add the exclusions to `a_line_reaches_its_own_units_area_and_no_siblings`: a sibling's doc promoted under `T1` is
  not excused, and a unit's staged doc lost beside a joined milestone is *lost*.

### F3: should-fix. Entry 17 is not open. `command-output-contract.md` answers it, and the "two things" is an inconsistency of `finalize.md:242`

- **Location.** `byte-table-acceptance-rows.md` §6, entry 17; readings §16; `DECISIONS.md` (`cabe3aff`, *Two more
  places*).
- **Claim.** Entry 17 says *"nothing of `squash: false` says what becomes of"* a change the user staged in the main
  checkout. `design/command-output-contract.md:542` says it of **both** modes: *"both aggregate channels build their
  commit from the sub-task worktrees over targeted pathspecs and land by `--ff-only` — so that entry structurally
  cannot ride the boundary commit"*. `:546` adds: *"These bytes … stay staged across the boundary … this door neither
  takes nor guards the shared index"*. P2 confirms the product matches: `still_staged` names the file, and no commit
  of the chain carries it.
- **What actually disagrees.** Only `finalize.md:242` (*"through `IndexHonoring` over the now-clean index"*). That
  makes it a doc-only inconsistency for the findings' §1, not a question for the human. This is the entry-13/14
  pattern again: the record searched `finalize.md` and did not grep `command-output-contract.md`.
- **Smallest fix.** Reword entry 17 as answered by `command-output-contract.md:542-546`, and add a §1 key for
  `finalize.md:242`. No cell moves.
- **Lead for part B.** The same contract and `finalize.md:248` say `milestone create` snapshots the shared checkout's
  index for the carryover gate. A change staged **before** the milestone is minted is therefore a different need from
  the cells' post-join plant, under both spellings.

### F4: should-fix (medium). The `squash: false` switch is inert in the comparison: `M4`'s allowance is `M3`'s, so a product that ignored the knob would keep all four cells green

- **Location.** `milestone.rs:74` (`M3`) and `:95` (`M4`): both are `Allows::Commit(Holding::TheWorktrees)`.
- **Claim.** Swapping `M3` for `M4` changes no verdict. Nothing in the frame checks that the chain happened: one
  commit per code-carrying sub-task, in task-id order, then the aggregate. The record admits that it cannot say the
  order or the partition. It does not say that the spelling as a whole is indistinguishable from the plain one.
- **What I checked.** The cell's spelling does configure the product today. P2: `jigc config set
  finalize.fan-out.squash false`, committed before `milestone create`, gives `git log` three commits (two `feat: add a
  file`, then `Finalize milestone squash-off (2 sub-tasks)`). The doc-only spelling is evidenced by the cells
  themselves: its staged-elsewhere cell would be *git's state moved* under `T9` if the index had been swept.
- **Smallest fix.** A `Holding` for the chain, using the `made` list the commit layer already has: N commits whose
  changed paths each fall within one worktree's staged set, in task-id order, then one aggregate. Alternatively, at
  minimum a check on the cell that the boundary made more than one commit.

### F5: should-fix (medium). `A5`/`M5` (`docs/milestone-records/*`) let a command rewrite and commit **any** milestone's record. The vocabulary cannot narrow them as it stands

- **Location.** `task.rs:224` (`A5`) and `milestone.rs:114` (`M5`). A grep of `byte_table/*.rs` for a `Where` literal
  containing `*` finds these 2 hits plus `W4`'s `docs/*`.
- **Why it can't be fixed with today's words.** `stands_for` (`byte_cell.rs:824-830`) matches whole components, so
  `<unit>.md` is a literal and matches nothing.
- **What it excuses.** Under `M5`, `milestone finalize m1` rewriting m2's record is excused at the file layer, and its
  commit is excused through `written_by_a_line`. That is a hidden write. It is not a loss, because a planted edit's
  marker would still be caught.
- **Judgment.** Both sentences name one record, and it is computable:
  - `M5`'s is the unit's own (*"the record's `status`-flip"*).
  - `A5`'s is the record that holds the sub-task's item (*"The item's `status` flips … every sibling item is
    byte-untouched"*). That is the inverse of what `ITS_SUB_TASKS` already reads.
- **Fix before the milestone rows.**
  - Let a component be `<unit>` plus a fixed suffix.
  - Add a third word, `<its-milestone>`: the record in `before` whose `### <unit>  {#<unit>}` item names the acting
    unit. A missing record then reaches nothing, which is the stricter direction (the B6 `task discard` finding).

### F6: note. `Unit::TheTaskFlag`'s "single task that stands" counts any directory. The product counts areas that carry a base pin

- **Location.** `byte_cell.rs:779-793`, against `engine/src/state.rs:1725-1737` (`carries_base_pin`) and
  `storage.md:294` (*"A task is an area that carries its base pin"*).
- **Effect.** One live task beside a leftover directory: the product resolves the task, and the frame resolves no
  unit, so a correct write is a false red. A leftover directory alone: the product refuses, and the frame names the
  leftover as the unit. That is stricter or self-revealing in every shape I traced; none hides.
- **Fix.** Require `.jigc/tasks/<id>/base.json` to be a file in `before`.

### F7: note. The sub-task list is said to come from the "committed record", but it is read from the working tree, at a hard-coded home

- **Wording.** The doc comment at `byte_cell.rs:260-264` and the record's §7 say *committed*. The code at
  `:801-818` reads `before`'s working-tree bytes. `DECISIONS.md` says "as it stands before the command", which is
  accurate. A record edited by hand to add an `### x  {#x}` item would widen `<sub-task>`. Today's `Plant::Edit`
  appends a marker line only, so no plant can do this.
- **Home.** The path `docs/milestone-records/{unit}.md` (`:805`) ignores `docs-root`. `storage.md:214` says
  `docs-root` moves this home. Under any other `docs-root`, `<sub-task>` is empty, which is a false red, never a
  hide.
- **Fix.** Align the comment with the code, or read `HEAD`'s blob. Resolve the home as the states resolve it.

### F8: note. `T9` leaves out owner-artifacts, which the doc-only arm commits

`finalize.md:329`: *"It stages exactly the task's promoted docs … and its stageable recorded owner-artifacts, and
commits those paths alone"*. `T9` is `OnlyTheLines`, and no line writes an owner-artifact. So a doc-only task with one
would read *git's state moved*. That is stricter, and unreachable with the shipped packs: `owned-location` sits only on
`completion-record` and `dogfood-record`, and no shipped doc-only workflow creates them. Label: lead.

### F9: note. Needs are keyed by line, not by spelling, so a `doc-only` cell answers the plain spelling's need

`task-finalize-a-link-to-nothing-at-the-created-docs-home` (`task.rs:495-524`) is now spelled `doc-only` and aimed
at `T1 × link to nothing`. `answers` returns `Need::Planted(aim)` whatever the spelling. No plain-spelled cell meets
`T1 × N`: a grep of `task.rs` for `LinkToNothing` aimed at `T1` gives this one hit. It is defensible, because
*"the transaction around the commit … is the ordinary one"* (`finalize.md:329`). But that should be said, or the
need keyed per spelling.

### F10: note. Two reachable fault paths of the switch were never seen red

- `answers`' `(Layer::Refusing | Layer::BaseElsewhere, None, Some(name))` fault (`byte_table.rs:598-602`) has no
  fixture case. The loop at the end of `a_cell_is_compared_under_the_spelling_it_names` exercises the other three.
- `ran()`'s guard *"the cell names the spelling `{name}`, which the row … does not have"* (`byte_cell.rs:2122-2128`)
  is not exercised either. `answers` covers the same condition statically.
- The builder's *Not established* says the faults were held "fault by fault". That is true of three faults, not of
  these two.
- Fix: add one fixture cell per fault.

### F11: note. Ruling 1's "names what stands there" is held by no cell

The 21 cells list today's codes as substrings (`write.non-reparseable`, `write.wrong-shape`, *"carries no `# H1`"*).
A fixed product that refuses before the copy-in under the same code, without naming what stands, would hold. The
records say the listing "follows the run that shows it", which is honest. The half of the ruling about naming is a
fixer's acceptance item, not a cell's.

### F12: note. `cargo fmt --check` does not reach the family files, and the range adds four malformed imports

- The family `mod`s are declared inside `macro_rules! families!` (`byte_table.rs:153-162`), which rustfmt cannot
  see. `cargo fmt --check` therefore exits 0.
- `rustfmt --check` run on each file gives 157 diffs at `a81af2bb` and 160 at HEAD. The range adds `Unit,};`,
  `Unit, Spelling,};` and `Unit, Spelling, Mark,};` (`doc.rs:19`, `install.rs:26`, `task.rs:21`, `milestone.rs:19`).
- This is a pre-existing gate hole, now known. Fix: run rustfmt over `byte_table/*.rs` in `dev/gate`, or declare the
  modules outside the macro.

## The six checks, answered

1. **The rulings.** All three are in `DECISIONS.md` at the top, with the brief's words and nothing narrowed.
   - Ruling 1 keeps the item-23 carve-out, the "names what stands there" clause, and `doc-read-surface.md` becoming
     true.
   - Ruling 2 keeps "one rule with `author`'s" and the `(R3, F3)` closure.
   - Ruling 3 keeps "both spellings, own commit, before the milestone rows".
   - The 21 cells:
     - The six of `add-item`/`create` are listed as refusing and act today, so they are *not the listed outcome*.
     - The fifteen refuse and leave the copy, so they are *refused, and changed*.
     - Both follow the ruled reading. P1 confirms the plant's bytes do not parse: `doc show` gives
       `store.unparseable`, *"required section heading `## milestones` is missing"*.
   - The mark table drops exactly 21 + 15 lines, so 38 becomes 2.
2. **The own-area words.**
   - The narrowed lines are each grounded:
     - `W1`/`W4`: `write-commands.md:240`.
     - `T6`: `finalize.md` *"remove `.jigc/tasks/<task-id>/`"*.
     - `A1`: `write-commands.md` → *The verbs*.
     - `M1`: *"`.jigc/milestones/<id>/merged/`"*.
     - `M6`: `storage.md` → *The milestone boundary tears down two kinds of area*, with `team-ready-state.md:115`
       *"scoped to those sub-task ids"*.
   - The four cases the brief named:
     - No `--task` with several tasks: no unit, which is the stricter direction.
     - A record the command rewrote: not read, because the read is from `before`.
     - A missing record: no sub-tasks, which is the stricter direction.
     - A wrong unit: F6, which only goes the stricter way.
   - What can hide: F2, three unit-blind sets outside `Where`, one of which loses a doc under `T6`. `A5`/`M5`: F5.
3. **The spelling switch.**
   - Both cells' spellings configure the product (P2, and the doc-only cells' classes).
   - `T9` for `T2`/`T3`/`T4` is right. *The doc-only arm* says the stamp is not refreshed and a pending config
     delta is outside the path set: `T2` and `T3`'s second sentence. The index is never committed: `T4`.
   - `M4` for `M3` swaps lines that are identical (F4).
   - Unproven fault paths: F10.
4. **The new key and entries 16/17.** Entry 16 is answered by the docs. Its three cells and the key are false, and its
   premise is false of the product (F1). Entry 17 is answered by `command-output-contract.md:542-546` (F3).
5. **Counts.**
   - 353 / 269 / 84 is fenced and green.
   - 71 cells under 17 keys, counted from the source.
   - 5 marked cells: 3 of `Silence(16)` plus 2 bounds.
   - 345 + 8 = 353 and 261 + 8 = 269, consistent across the commit messages, `DECISIONS.md` and the records.
   - If F1 is taken: 68 under 16, and 2 marked.
6. **Hygiene.**
   - `git diff a81af2bb..cabe3aff` has no host path, person's name, session id or agent id. Checked by grepping the
     added lines for host prefixes, names, `session_` and run ids.
   - The commit messages carry only the brief-mandated `Claude-Session` trailer.

## Not checked

- The previous release (`1.0.0-rc.24`): no binary of it was run. The "same on both binaries" claims, the 167 s and
  174 s whole-table runs and the named-binary arm are unverified.
- The full gate, Linux, and per-cell seconds.
- F2 on a built cell: it is traced through the source only.
- That every doc-only cell's commit holds exactly the doc. This rests on the suite's classes, not on a probe.
- The rows' completeness for the `milestone` family's other leaves: that is part B.

<!-- end of report -->
