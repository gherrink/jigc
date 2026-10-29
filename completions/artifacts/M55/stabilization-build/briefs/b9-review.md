# Review: the byte table, task B9 — the mint family (`002b2090..5b528b33`)

Reviewer: independent and read-only. Worktree `gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`.
Commits: `f83b98e1` (the human's rulings on entry 23 and R1), `3c4f642a` (rows, three words), `643e953e` (cells,
`Cell::at`, owed lists removed), `5b528b33` (numbers, whole-table print). No product source changes in the range
(the last one is `0ad081cd`).

## Verdict

- **The rulings commit is faithful.** Entry 23 and R1 are quoted in the human's terms, the questions are kept,
  and the doc and product changes are routed to the ledger as findings. `RL3` and `CS3` are struck in the records
  and gone from the frame. `MovedAsideTo` at one file's place is fenced
  (`a_line_moves_aside_only_from_a_place_it_removes`), and the baseline reading is removed. Two cell-group doc
  comments in `movers.rs` still describe the parking and the marks (N4).
- **One ruling is not held at its own door** (H1). *"Does all it can … moves the others"* — R1, and entry 23's
  relocate clause — has one cell. That cell comes out *refuses, nothing changed* on a product that refuses the
  whole sweep. No relocate cell has an occupant at a destination **and** a second note.
- **Three frame holes, each shown by a probe that comes out *holds*:**
  - *into its home* takes a never-committed `.md` and a link (H2);
  - the catalog cells of `start` and `workflow` are judged under the plain spelling's lines (H3);
  - *provisioned* takes a link or a special file (H4).
- **The regression is confirmed, but its cause is misattributed** (H5, N1). On rc.24,
  `config-set-placement-root-a-link-to-nothing-at-the-roadmaps-new-home` refuses and changes nothing. On this tree
  it parks the link at exit 0. `5f5b273a` introduced it, by changing the occupancy probe in
  `displace_foreign_squatter`. The routing through `relocate_stranded` did not: rc.24 already routes there.
- **The §26 lead was called undriveable for a reason that does not hold.** Driven, it is **data loss on both
  binaries** (H6). `migrate` followed by `task finalize --approve` deletes a committed source together with its
  uncommitted edit, and the edit's bytes end up in no file and no object.
- **Rows are complete for what each door writes.** I drove each of the 13 leaf forms with nothing planted, on this
  tree. Every write falls under a line (§ Driven, D2).
- **Counts.** The suite's counts and the print's red counts re-derive exactly. Two record counts do not
  (N2):
  - "twenty-one cells moved" is **eighteen** in the committed prints;
  - "the other 662" is **665**.
- **Hygiene is clean.** `cargo fmt` reaches `mint.rs`.

## What I ran

- **Binaries.**
  - This tree: the worktree's `target/debug/jigc`, sha256 `a71a816e…` (current: no product source since
    `0ad081cd`).
  - rc.24: the scratch rebuild, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`,
    checked.
  - Both are copied into my scratch dir as `tree/jigc` and `rc24/jigc`.
- **Frame probes.** In a `git archive` copy of `5b528b33` under my scratch dir, with three probe modules appended to
  `crates/cli/tests/byte_table.rs`. Run with
  `CARGO_TARGET_DIR=<scratch>/target cargo test -p jigc --test g_migration review_probe_b9 -- --nocapture`. The
  worktree was not touched.
- **Rigs.** `dev/jigc-rig --binary <bin> fresh`, with a two-step eval and stdout captured. Scripts D1–D6 are in the
  scratch dir.
- **Counting.** `awk`/python over the committed `.tsv` files, and a python parse of `EXPECTED_RED`.
- **Formatting.** A misformat added to `mint.rs` made `cargo fmt --check` exit 1 and name the file.

## Frame holes

### H1. Medium-high. R1's *"does all it can"* and entry 23's *"moves the others"* are held by no cell. The one cell at R1's door holds on a product that refuses the whole sweep

**Location.** The cell is `relocate-a-link-to-nothing-in-one-stranded-notes-place-and-a-second-note`
(`movers.rs:1644-1654`): `refuses(1, "store.home-not-regular-file")`, aimed `RL1 × L`. The condition
`When::ADocLeftAndNamed` is at `byte_cell.rs:2212` and `:2217-2239`.

**Against.** The ruling, as recorded:
- *"An acting sweep **does all it can**, then exits non-zero"*;
- `relocate` *"blocks that doc, **moves the others**, names the path to clear and exits non-zero"*.

`DECISIONS.md` (f83b98e1) says *"A cell holds it at its door"*. `EXPECTED_RED`'s comment says *"the second note's
cell holds its *does all it can*"*.

**Probe** (`review_probe_b9_whole_refusal_holds_at_the_r1_door`). Setup: a link to nothing at note 1's place and
note 2 committed. The product moves nothing, exits 1, and names the link with the code. Result: **refuses, nothing
changed**, which is green.

The frame judges allowances. Note 2 left unmoved is no difference, so nothing reads it, and the listing's words are
only the code.

**Second half.** Every relocate occupant cell (six at `docs/notes/cache-benchmarks.md`) has a single note. So
entry 23's *"moves the others"* has no cell at all. A fixed product that refuses `relocate` whole on any occupant
turns all of them green, and the distinction the ruling draws from the root knobs (*"refuses the whole re-point"*)
is held nowhere.

**Bound.** By enumeration of the relocate cells (`RELOCATE_CELLS`, 26). Two cells have a second note, and only
this one has a blocked first doc.

**Fix.** Make the moved note an obligation of the listing. For example, list the refusal with the moved note's
destination `docs/notes/load-tests.md`, which a whole refusal never prints. Alternatively add a frame "must have
arrived" check for `A_NEW_HOME` under `ADocLeftAndNamed`. Also add a twin cell with an occupant at note 1's new home
and a second note.

### H2. Medium. *into its home* (both ends, `RL1`, `RL1, at its new home`, `CS2`, `CS2, at its new home`) asks neither that the doc was committed nor that it is a regular file

**Location.**
- `byte_cell.rs:2422` is the departure: `moved_home(rel)` takes any entry whose bytes stand at the destination.
- `:2423-2425` and `arrived_home` (`:2631-2641`) are the arrival: `Acting::destination` (`:1452-1465`) takes any
  `<name>.md` under `--from`, whatever its kind and whether git ever held it.

**Against.**
- RL1: *"every **committed** `.md` under the prior home"*.
- RL5 / `design/finalize.md`: *"A relocation **lands a regular file** or refuses"*.

**Probes.**

| case | came to |
|---|---|
| `review_probe_b9_relocate_moves_an_untracked_md`: the honest move, plus a never-committed `docs/legacy-notes/scratch.md` swept to `docs/notes/scratch.md` | **holds** |
| the same, with the user's file also `git add`ed into the index | **holds** |
| `review_probe_b9_relocate_carries_a_link`: a dangling link under `--from` carried into the home, exit 0 | **holds** |

**Why it matters.** The staged variant publishes a user's private scratch file at their next commit, after a
command that reported success. The product does the right thing today (D4, both binaries: `scratch.md` is left in
place). So the hole is latent, and no cell plants an extra untracked `.md` under `--from`. RL1's plants are
`InPlaceOf` the committed note (`movers.rs:1528`), and a carried plant is excused by the `IntoItsHome` arm of
`compare` (`byte_cell.rs:3403-3407`).

**Bound.** `grep May::IntoItsHome crates/cli/tests/byte_table/*.rs` gives 4 lines (`movers.rs:249, 259, 443,
459`), one mechanism.

**Fix.** The departure requires the path in `HEAD` or the index before the command. Both ends require
`Entry::File`.

### H3. Medium-low. A catalog cell typed in a spelling is judged under the plain spelling's lines: `start-as-the-catalog-names-it` holds on a bare `jigc start` that mints

**Location.** `mint.rs:400-406`: `argv ["start"]`, spelling `None`. `answers()` refuses a spelling on the Refusing
layer (`byte_table.rs:610-616`), so the cell cannot carry `bare`. `ST1` is in force under the plain spelling.

**Against.** The `bare` spelling's own sentence (`ST2`'s quote): *"**Never mints**"*.

**Probe** (`review_probe_b9_bare_start_minting_under_the_catalog_cell`). An area with a pin made by bare
`jigc start` over an empty `.jigc`:
- with spelling `None` → **holds**;
- with spelling `bare` → *written outside its allowance*.

`start-bare-over-a-task-it-reports` covers a mint over a live task, but nothing covers a mint in a project with
none.

**Bound.** By enumeration, there are 5 argv spellings across the rows (`bare`, `--task`, `--preview`,
`--no-commit`, `--dry-run`) and 7 catalog cells in the movers and mint files. Two catalog cells type a spelling:
- `start-as-the-catalog-names-it`;
- `workflow-as-the-catalog-names-it` (`--preview`, `mint.rs:526-534`). Under plain lines `WF2` (index `Rebuilt`) is
  in force, so a preview that rebuilt the index would hold. This one is by trace, not probed.

**Fix.** Let the Refusing layer carry the spelling its argv types, or judge a catalog cell under the spelling its
argv matches.

### H4. Low. *provisioned* (`May::Provisioned`) holds a link or a special file made in the area; its own doc says "a file or a directory"

**Location.** `byte_cell.rs:2428`: `(May::Provisioned, None, Some(_)) => true`.

**Probe** (`review_probe_b9_provisioned_link`). `commit:s1.md` made as a link to `/etc/passwd`, plus a FIFO beside
it → **holds**.

**Bound.** 1 line (`WF1`), from `grep May::Provisioned`.

**Fix.** Match `Some(Entry::File { .. } | Entry::Dir(_))`.

**Lead, same door.** `aimed` gives an `Under` place only **U**, so `WF1` has no **L**/**K** need. Driven (D5, both
binaries, identical): a dangling link at `.jigc/tasks/add-alpha/docs/commit:add-alpha.md` is **replaced** by jigc's
regular file at exit 0. `provision_on_first_entry` asks `path.exists()`, which follows the link
(`crates/cli/src/start.rs:762`). No cell can see it today.

### H5. The regression — confirmed, and its cause is `5f5b273a`, not the routing

**Cell.** `config-set-placement-root-a-link-to-nothing-at-the-roadmaps-new-home`.

**Driven (D1).**

| | rc.24 (`accf3996…`) | this tree (`a71a816e…`) |
|---|---|---|
| exit | 1 | **0** |
| said | `blocking · config.repoint-failed — … git mv docs/roadmap.md notes/roadmap.md failed: fatal: destination exists … the re-point was undone` | `notes/roadmap.md → .jigc/displaced/roadmap.md (a foreign file at the new home, parked out of the way)`, `config: set placement-root = notes` |
| after | the link in place, `?? notes/`, no manifest | `R docs/roadmap.md -> notes/roadmap.md`, `?? .jigc/config/manifest.yaml`, the link under `.jigc/displaced/` |

**Cause.**
- `git log -L '/fn displace_foreign_squatter/,+30:crates/cli/src/relocate.rs' 91834b5e..HEAD` names one commit:
  `5f5b273a` (2026-10-04, *"fix(rename): a door that writes a committed doc in place never writes through or moves a
  link"*).
- It changed the occupancy test from `if !dest_abs.exists()` (rc.24, `relocate.rs:762`) to
  `engine::store::home_entry(&dest_abs) == HomeEntry::Free` (`relocate.rs:855-860`). A dangling link was "free" to
  rc.24, so `git mv` refused. Now it is a squatter, and it is parked.
- rc.24 already routed `placement-root` through `relocate_stranded` (`git grep relocate_stranded 91834b5e --
  crates/cli/src` → `config.rs:1513`).
- The fix was deliberate under the then-settled foreign-only park bound. The human's ruling of 2026-10-20 reverses
  that bound, so the change regressed against the behaviour that makes sense.

**Bound.** `displace_foreign_squatter` has one caller, `relocate_one` (`relocate.rs:784`), reached by `jigc
relocate` and `config set placement-root`. `docs-root` calls `move_doc` directly (`config.rs:1419`). So the class
is two cells:
- this one;
- `relocate-a-link-to-nothing-at-the-notes-new-home`, which moved the same way (rc.24: git's code-less blocked row
  at exit 0; this tree: parked at exit 0). It is red on both binaries, so it is not counted as a regression.

The fix to entry 23 closes both.

### H6. High (product; outside the family's cells). `migrate` + `task finalize --approve` deletes a committed source together with its uncommitted edit — bytes in no file and no object, both binaries

**Lead.** §26 *Seen*: *"not driven to the retire (it needs an authored changelog)"*. The authored changelog is one
heredoc. The plan's rule asks for *"the exact reason no cell can see it"*, and "it needs an authored changelog" is
not that reason.

**Driven (D6, both binaries identical).**
1. A committed `HISTORY.md`, plus a line `- An uncommitted line of the user's.` appended in no commit.
2. `migrate` (exit 0). The control finds the line in `HISTORY.md` and in `.jigc/tasks/<id>/source`.
3. `doc author` without that line.
4. `task finalize <id> --approve` → exit 0, `finalized … adopt HISTORY.md as a managed changelog`.
5. After: `HISTORY.md` is gone, `command grep -r` finds the line nowhere in the tree (`.jigc` included), and a scan
   of every loose object under `.git/objects` finds it in none.

The approval hold does show the dropped line in its fidelity diff. But the adapter's deny floor blanket-permits
`finalize --approve` for an agent on the warrant *"a guarded migrate destroys only a reviewed, in-repo,
**git-recoverable** file"* (`design/auto-migration.md:76`), and the trackedness test there asks whether git holds a
copy **of the path** (index or `HEAD`), not of the bytes it deletes.

This is the more common shape. The lead's shape (index entry gone, a file of the user's in its place) is another
instance.

**Bound.** One mechanism, the retire's trackedness test. The two shapes are an instance pair; unbounded beyond them.

**Fix (robust).** The retire refuses where the working bytes differ from what `HEAD` (or the index) holds, naming
the path and routing at committing the edit first. Better still, `migrate` refuses such a source up front, as it
refuses an untracked one. The doc's path test is wrong, and so is the code.

**For the table.** A task-family cell over this construction, listed refusing; it is *lost* on both binaries today.

## Record notes

- **N1 (high for the fixer). The regression is misattributed.**
  - `byte-table-findings.md` §25 (line 1428): *"the rc.24 fix pass routed `placement-root`'s re-point through
    `relocate::relocate_stranded`, whose displacement parks what rc.24's plain `git mv` refused"*.
  - The `f83b98e1` `DECISIONS.md` entry and the mint record §1 say the same.
  - rc.24 already routed there, and rc.24's displacement already parked every other squatter (the other four cells
    are *"the same on both binaries"*). The cause is `5f5b273a`'s occupancy probe (H5).
- **N2. Two counts do not re-derive from the committed prints.**
  - A diff of `byte-table-movers-family-cells.tsv` (683) against `byte-table-mint-family-cells.tsv` (738), every
    column but seconds, gives **18 moved** (relocate 6, `docs-root` 6, `placement-root` 6) **+ 55 new** (51 mint and
    4 movers).
  - The `f83b98e1` message and `DECISIONS.md` say *"twenty-one cells moved"*, and the mint record §1 says *"twenty-one
    … and the other 662"*. Both should read 18 and **665**.
  - The commit's own breakdown (*"relocate's six … placement-root's six … docs-root's six"*) sums to 18.
- **N3. The §26 repro block does not reproduce.** It plants with
  `echo … > ".jigc/tasks/add-alpha/docs/commit:add-alpha.md"`, but `docs/` does not exist before the first re-entry
  (`milestone add-task` makes none, `design/write-commands.md:236`). Run as printed:
  - the echo fails (`no such file or directory`);
  - the re-entry then provisions normally (`commit:add-alpha.md provenance.json`), contradicting its comment
    *"the user's file alone"*.

  It needs `mkdir -p`. The cell's own plant is fine, and D3 reproduces the finding with the directory made.
- **N4. Stale doc comments in `movers.rs`** describe the struck lines and the removed marks:
  - `:1500-1504`: *"what squats it is parked under `.jigc/displaced/` (`RL3`) … marked"*;
  - `:1830-1846`: *"what squats it parked (`CS3`) … the reading of the record's §6, entry 23, marked"*.
- **N5. The §26 lead's reason.** *"not driven … (it needs an authored changelog)"* is a cost, not a reason no cell
  can see it (H6).
- **N6. The mint record's sources.** It cites *"(print), its blocks `cells`, `needs` and `excused`"*, but only the
  `cells` block is committed. The excused numbers (`ST1` 50 … `AM1` 66) cannot be re-derived from it. Needs 49 I
  re-derived by hand: 13 + 11 + 16 + 9, with one scheduled.
- **N7. `CS6` in the rows' record**, edited by `f83b98e1`, keeps *"parking a racer's bytes under
  `.jigc/displaced/config/`"*. `relocate.rs:324-326` says the racer's bytes **stand** and jigc's own pre-image parks.
  Under a ruling that says *"never from a destination"*, the wording matters. The clause predates this range.
- **N8. A listing that would turn red on its fix.** `migrate-a-directory-in-the-sources-place` lists the run's words
  (*"could not read the foreign `changelog` source"*). The fix the lead asks for (a jigc code) changes the words, so
  the cell would go red, not green. The self-check's item 7 runs the other way. Low.
- **N9. A pattern-wide lead (B8 has it too).** 12 of the family's 24 on-demand cells — the base cells of
  `workflow` and `migrate` in the other prepared states — are refusals of a missing precondition (`add-alpha`,
  `HISTORY.md`), so the state's character never meets the leaf.
- **N10. (A)'s twins plant only an untracked doc.** The ruling's *"an unmanaged doc"* includes a committed-unmanaged
  one, and that case has no twin. Low.
- **N11. `MI2 × E, before` is scheduled** as *"the same construction"*. It is consistent with the precedent of `M11`,
  `M12`, `MD7` and `MD8`, and buildable now as a second cell. Not a hole.

## Questions for the human

**Q-a. The re-entry over a file at the commit doc's place** (`r1-workflow-re-entry-takes-a-file-of-the-users-as-the-sub-tasks-commit-doc`).

- **Doc.** *"The first `jigc workflow <W> --task <id>` re-entry **provisions** the sub-workflow's deterministic
  instances (the commit doc …) into the sub-area"* (`design/write-commands.md:236`). It says nothing of what stands
  there.
- **Code.** *"**Idempotent / first-entry-only** — keyed on the **absence** of the `docs/commit:<id>.md` skeleton …
  so a sub-agent's in-progress edits survive a later re-entry"* (`crates/cli/src/start.rs:745-763`, `path.exists()`).
- **Verdict.** It is a genuine silence, and the builder's judgement (code wrong) is sound in direction. Its stated
  reason, *"mirrors the mint, which refuses what stands at its area"*, is weak: the mint refuses a residual
  **area**, while the re-entry provisions into an area that legitimately exists.
- **Robust answer.** "First entry" is keyed on jigc's own record (the provenance manifest's `created` entry for the
  commit doc). A commit doc that stands without it refuses, naming the path. A sub-agent's authored doc, which has
  provenance, is left. The commit doc's place is asked about without following a link (H4's lead).
- **Which is wrong.** The code is wrong, and the doc owes a sentence.
- **Until ruled.** Listed refusing, as built.

**Q-b. The migration retire of an edited source** (H6). It may be filed as a key rather than a question.

- **Doc.** *"the retire deletes only a file git holds a copy of"* — the index or `HEAD`, a path test
  (`design/auto-migration.md:76`).
- **Against it.** The same paragraph's warrant: *"a guarded migrate destroys only a reviewed, in-repo,
  **git-recoverable** file"*. That warrant is what keeps the agent's blanket permit for `finalize --approve`.
- **Robust answer.** Refuse where the working bytes are in no object (at `migrate`, or at the retire), with a route
  at committing first.
- **Which is wrong.** The doc's test and the code.

## Counts re-derived

**Suite.**
- `EXPECTED_RED` (python parse): **42 entries, 40 keys, 150 cells, all unique**.
- The fence's `(738, 558, 180)` is at `byte_table.rs:11440`.
- `scheduled-cells.tsv`: **11** non-comment lines. `scheduled.tsv`: 6.
- On-demand `.tsv`: 181 six-column lines (180 cells plus the header).

**Print** (`byte-table-mint-family-cells.tsv`, 738 rows, 12 columns).

| scope | cells | red on rc.24 | red on this tree |
|---|---:|---:|---:|
| the whole table | 738 | **259** (2 *not run*) | **164** |
| the mint family | 51 (start 15, workflow 11, migrate 15, amend 10) | **1** | **1** (the same cell) |
| the movers family | 155 | **57** | **46** |

- Marks: **1** (bound 1).
- Red on this tree and clean on rc.24: **exactly one** cell (H5).

**Needs.** 49 by hand (above). I did not re-derive 670 for the whole table.

## The builder's self-check, verified

| item | verdict |
|---|---|
| 1. Words wider than their sentence | **Not fine**: H2, H3, H4. `<minted-task>` holds against everything I tried (two areas, a residual, a sibling), and the builder's fence covers those. |
| 2. Compares less | **Not fine**: H2 (committedness and regular file are not read). |
| 3. Unit-blind sets | Fine. `minted_tasks`, `new_homes` and `left_a_doc_and_named_it` are keyed on the id, `--from` and the unit. |
| 4. Silences | **Fine**: bare `start`'s index (`reconciliation.md:96` and its amendment), the serial collision (`write-commands.md:206`). Q-a is genuine. |
| 5. Cells that cannot fail | **Not fine**: H1, H3. A lead was called undriveable when it could be driven (H6, N5). |
| 6. Rows missing writes | **Fine** (D2). |
| 7. A ruling at its door | **Not fine**: H1. Also N8. |
| 8. Refusal as exit 0 | Fine. The mint lines are off on a refusal, and the RL lines are bounded by `ADocLeftAndNamed`. |
| 9. Counts | **N2, N6.** |
| 10. Re-read on both binaries | The print backs it, apart from the count (N2). |
| 11. Sense-check | Q-a; H6. |
| 12. Hygiene | Clean: no host path, name or id in any `+` line or message; `fmt` reaches `mint.rs`. |

## Repros

```zsh
# Frame probes — appended to a scratch copy of 5b528b33's crates/cli/tests/byte_table.rs (modules
# review_probe_b9, review_probe_b9_r1, review_probe_b9_catalog), then
#   CARGO_TARGET_DIR=<scratch>/target cargo test -p jigc --test g_migration review_probe_b9 -- --nocapture
# PROBE relocate-carries-a-link-exit-0: holds                       (H2)
# PROBE relocate-moves-untracked-md: holds                          (H2)
# PROBE relocate-moves-and-stages-untracked-md: holds               (H2)
# PROBE provisioned-link-and-special: holds                         (H4)
# PROBE r1-door-whole-refusal: refuses, nothing changed             (H1)
# PROBE bare-start-mints spelling=None: holds                       (H3)
# PROBE bare-start-mints spelling=Some("bare"): written outside its allowance
# PROBE left-and-named-free-destination: refuses, nothing changed   (by the builder's design; noted in H1)
```

```zsh
# D1 / H5 — placement-root over a link to nothing at the roadmap's new home (both binaries)
W=<worktree>; rig=$($W/dev/jigc-rig --binary "$BIN" fresh) || exit 1; eval "$rig"; cd "$REPO"
mkdir -p docs && printf -- '---\nschema-version: 1\n---\n\n# Roadmap\n\n## Milestones\n' > docs/roadmap.md
git add -- docs && git commit -q -m "a roadmap"
mkdir -p notes && ln -s nowhere notes/roadmap.md
$JIGC config set placement-root notes; echo "exit=$?"   # rc.24: 1, config.repoint-failed · tree: 0, parked
git status --porcelain                                  # tree: R docs/roadmap.md -> notes/roadmap.md
```

```zsh
# D6 / H6 — the retire deletes an uncommitted edit (both binaries); `command grep`, the control first
rig=$($W/dev/jigc-rig --binary "$BIN" fresh) || exit 1; eval "$rig"; cd "$REPO"
printf "# History\n\n## 1.0.0\n\n- Added a thing.\n" > HISTORY.md && git add HISTORY.md && git commit -qm h
printf -- "- An uncommitted line of the user's.\n" >> HISTORY.md
T=migrate-changelog-history-3268e06b69e1
command grep -rl "An uncommitted line" .                 # ./HISTORY.md
$JIGC migrate HISTORY.md --as changelog >/dev/null
printf 'title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: "1.0.0"\n        sections:\n          - id: changes\n            items:\n              - title: "added"\n                set:\n                  notes: |-\n                    <<- Added a thing.>>\n' \
  | $JIGC doc author changelog --from-file - --task $T
$JIGC task finalize $T --approve; echo "exit=$?"         # 0
command grep -rl "An uncommitted line" .                 # nothing
find .git/objects -type f -path '*/objects/??/*' | while read f; do o=${f#.git/objects/}; o=${o/\//}; git cat-file -p $o 2>/dev/null | command grep -q "An uncommitted line" && echo $o; done   # nothing
```

```zsh
# D3 — the WF1 red cell, with the directory the §26 repro omits (both binaries: exit 0, the user's file kept as the commit doc)
$JIGC milestone create "Ship it" && $JIGC milestone add-task ship-it "add alpha" && $JIGC milestone provision ship-it
mkdir -p .jigc/tasks/add-alpha/docs && echo "the user's own notes" > ".jigc/tasks/add-alpha/docs/commit:add-alpha.md"
(cd .jigc/worktrees/add-alpha && $JIGC workflow sub-task --task add-alpha >/dev/null; echo "exit=$?")   # 0
# D5: the same with `ln -s <outside>/target.md` there — replaced by jigc's regular file, exit 0, both binaries
# D4: relocate over a stranded note plus an untracked docs/legacy-notes/scratch.md — left in place, both binaries
# D2: each of 13 leaf forms over `fresh` with nothing planted, snapshot diff — every write under a row's line
```

## Not checked

- No `dev/gate` and no whole-table run. The cells were not re-run, and counts are from the committed print.
- `workflow --preview`'s catalog cell under plain lines (H3) is by trace only.
- The gate seconds and the arm seconds.
- Whether any acting sweep other than `relocate` and `migrate-corpus` falls under R1. The ruling names those two,
  and I did not enumerate further.
- Linux.

## Scratch copy, for the fixers

`<scratch>/b9-review.1vHyAN`.

| item | where |
|---|---|
| the two binaries | `tree/jigc`, `rc24/jigc` |
| the probe tree, with its built target | `src/`, `target/` |
| the rig scripts | `d1-placement-link.sh`, `d2-leaves.sh` (with `snap.sh`), `d3-reentry-plant.sh`, `d4-relocate-untracked.sh`, `d5-reentry-link.sh`, `d6-migrate-retire-edit.sh` |
| the outputs | `out/`, `probe*.log` |

<!-- end of report -->
