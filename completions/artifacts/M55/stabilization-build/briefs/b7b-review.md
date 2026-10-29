# Review: the byte table, task B7, part B — the `milestone` family (`fff27d7b..ed440461`)

Reviewer: independent and read-only. Worktree `gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`.
Commits: `be152fad` (the seven rows, `M3, the config`, three words), `ffe4abe4` (record doors' cells), `17caeecb`
(fan-out doors' cells), `9b190346` (terminals' cells, `M6`'s moved-aside half, entry 18; amended from `c6a3c076`),
`ed440461` (numbers record, the whole-table rc.24 print). No product source changes in the range (the last one is
`0ad081cd`).

## Verdict

- **Counts are right except one.** The whole-table numbers, the family's numbers, the expected-red list, the marks,
  the per-commit cell counts and the rc.24 print all check out against the suite and the committed `.tsv`. The
  amend changed the message only, and the in-place slip corrections are right. **One count is wrong:** the family
  record's "7 rows, 57 lines; `finalize` 21 lines, where 18" (F4).
- **Two frame/row widenings can turn a red into *holds*:**
  - `M3, the config` / `May::Carried` is wider than every sentence it quotes. It makes a sweep that **I drove on
    both binaries** read as *holds*: the landed boundary commits a user's **untracked** file under `.jigc/config/`
    (F1).
  - The re-seed's `Rebuilt` lines cover standing areas, where their own quoted sentence says *"skipped untouched"*.
    So a rewrite of a standing sub-task's staged prose or base pin would hold at five doors (F2).
- **Entry 18 is a genuine "two things", but it is under-quoted on both sides.** Its claim that the narration
  *"predates the rule"* is incomplete: the narration was re-stated in `finalize.md` on 2026-10-06, after the exit
  rule, in the build of a human ruling (F3).
- **The other "answered" items:**
  - *A change staged before `milestone create`*: answered. `command-output-contract.md:542` says *"The carryover gate
    states exactly this for the pre-milestone staged set and refuses over it"*.
  - *The milestone area's complement*: answered, `storage.md:304`.
  - *A pending config delta*: **not answered — the docs say two things** (F1).
- **The four new keys each reproduce, the same way on this build and on rc.24** (repros below). Two of them share
  one writer with the task family's keys (F7).
- **Hygiene is clean.** `cargo fmt --check` exits 0 and reaches the family files, and `owed.rs` lost exactly the
  seven leaves.

## What I ran

- **Binaries.**
  - This build: the worktree's `target/debug/jigc`, current because no product source changed in the range.
  - rc.24: the rebuild at `<scratch>/rc24/jigc`. `shasum -a 256` gives
    `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, which matches.
- **Rigs.** Every rig was built with `dev/jigc-rig --binary <bin> <state>`, a two-step eval, capturing stdout only.
- **Probes** (scripts in the session scratchpad `b7b/`):
  - `probe-config.sh`: an untracked file under `.jigc/config/` at the landed boundary.
  - `probe-m2e.sh`: the scheduled `M2 × E`, before and after the first write, in `committed-singletons`.
  - `probe-join-index.sh`: the edge index absent before `execute`, `join` and `provision`.
  - `keys.sh`: the four keys.
- **Counting.**
  - `awk` over `byte-table-milestone-family-cells.tsv` and `byte-table-on-demand-cells.tsv`.
  - A python parse of `EXPECTED_RED`: 20 keys, 80 cells, all unique.
  - A grep of `Mark::`.
  - A comment-stripping parse of every `Row`'s `lines` at `fff27d7b` and at `ed440461`.
- **Other checks.** `cargo fmt --check`, with rc=0. `git diff c6a3c076 9b190346 --stat` is empty, so the amend
  changed the message only.
- **Read:**
  - The whole diff.
  - In `byte_cell.rs`: `compare`, `covering`, `written_by_a_line`, `is_at_for`, `Acting::minted`, the registration
    and checkout layers, and `Plant`.
  - `milestone.rs`, rows and cell macros.
  - In `cli/src/milestone.rs`: `commit_record_only` at `:999-1012`, and the callers of `commit_record_transaction`
    at `:960`, `:2121`, `:2705` and `:5871`.
- **Docs read:**
  - `finalize.md` 152-166, 284, 296-300, 335, 374 and 378.
  - `storage.md` 296-308 and 354-356.
  - `team-ready-state.md` 110-116.
  - `write-commands.md` → *Minting a milestone (M7)*, whole.
  - `command-output-contract.md` 538-546.
  - `DECISIONS.md` 4559-4590, 4681-4780 (5a, 5b, 5c) and 5497.
  - Plan §1.1.
  - `gap-list.md` G8 and settle-log R1.

## Findings

### F1. Blocking. `M3, the config` (`May::Carried`) is wider than every sentence it quotes, and makes a driven sweep of a user's untracked file into the boundary commit read *holds*. The record calls the question "answered by the docs", but the docs say two things

**Location.**
- `crates/cli/tests/byte_table/milestone.rs:150-158`. These are the two lines: a grep for `May::Carried` in
  `byte_table/*.rs` finds 2 hits, both here.
- `byte_cell.rs:1292-1297`, the `Carried` arm of `written_by_a_line`.
- Rows record §4, `M3, the config`.
- Family record §3: *"Answered by the docs … whether a pending config delta rides the boundary"*.

**The sentences say less than the line.**
- **What `Carried` allows.** Any bytes under `.jigc/config` (and `.jigc/.gitignore`) that stood in the working tree
  may ride any commit, under `When::Always`.
- **The quoted sentences.**
  - The first: *"overlays the promoted docs + **first-commit** config"*.
  - The second: *"**on a first commit**, the project config layer (`.jigc/config`, `.jigc/.gitignore`)"*,
    `finalize.md:160`.
  - The third: *"an ordinary code-less finalize commits it beside the doc — **gap G8**"*. It describes `task
    finalize`, not this door. G8 is a *gap*: `gap-list.md` G8 has *"Code-less finalize commits … unstaged
    .jigc/config deltas"*, and settle-log R1 says the doc-only arm *"closes … gap G8"*.
- **Unquoted sentences on the other side.**
  - `finalize.md:166`: *"finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay
    out of the commit"*.
  - `storage.md:308`: *"jigc's promotion (docs + first-commit config) … never sweeping unstaged/untracked
    working-tree WIP"*.
- **The task family's reading of the same sentence.** The door the G8 sentence actually describes is `task
  finalize`, and it has **no** such line. `T3` is `Allows::Staged` over places a line wrote, and `T4` is the index
  as it stood. So the same sweep there reads *git's state moved*, traced through `as_it_stood`. The two families
  read one sentence two ways.

**Driven, on both binaries.** I used a `fresh` rig and a milestone with one code-carrying sub-task, joined. Then
`printf … > .jigc/config/notes-of-the-user.txt`, left untracked (`?? .jigc/config/notes-of-the-user.txt`), then
`jigc milestone finalize land-it`.
- Result: **exit 0**, and HEAD carries `.jigc/config/notes-of-the-user.txt`, beside `a.txt` and the record.
- rc.24 gives the same.
- `.jigc/.gitignore` does not ignore `config/`.

**What the frame does with it** (traced through `compare`, not run as a cell):
- The plant stands at its place.
- The commit's path is excused by `written_by_a_line`'s `Carried` arm, because the blob equals the working file
  before the command.
- The index entry after the fast-forward is excused through the same arm.
- No `AllTheLinesChanged` "swept into git" check applies under `TheWorktrees`.

The result is ***holds***. Before `be152fad` the same commit was *git's state moved*. The fence
`a_commit_carries_a_config_delta_as_it_stands` (`byte_table.rs`) has no exclusion case for foreign bytes under the
place. The cell `milestone-finalize-a-pending-config-delta` writes its delta with `jigc config set` in a lead-in,
so it is the product's own bytes and never a plant.

**Bound.** By mechanism: every commit any row lets carry a `Carried` place. Today that is the 2 lines of the one
row, under both spellings (`squash: false` replaces `M3` only).

**Smallest fix.**
1. Make the pending-delta question an entry of §6 (docs say two things), with *Until ruled* the stricter reading:
   no line, so the cell is red and marked.
2. Then add the exclusion case to the fence: a foreign file under `.jigc/config` committed is not excused.

The product's sweep of an untracked foreign file is a candidate finding of its own
(*"commits content the user did not ask for"*).

### F2. Blocking (latent). The re-seed's `Rebuilt` lines cover a standing area, where their own quoted sentence says *"skipped untouched"*. A rewrite of a standing sub-task's staged prose or base pin would read *holds*

**Location.**
- `milestone.rs:470-483` (`the_re_seed`). It is instantiated 5× for the sub-tasks' areas (`MA6`, `MS6`, `MP4`,
  `ME2`, `MJ2`, at `.jigc/tasks/<sub-task>`) and 5× for the milestone's area (`MA2`, `MS2`, `MP3`, `ME1`, `MJ1`, at
  `.jigc/milestones/<unit>`).
- `milestone.rs:668` (`MP1`, `.jigc/worktrees/<sub-task>`).
- Derivation: `grep -n May::Rebuilt milestone.rs` gives 11 hits. Three are `<minted>` lines, five are baselines
  (`.jigc/state`), and three are these source sites.

**Against.**
- `MA6`'s own quote: *"A sub-task whose area already exists is skipped untouched"*.
- `MP5`: *"a registered worktree it reuses untouched"*.
- The cache half's sentence is *"re-derived … when absent or stale"*. It names the cache, and not the snapshot or
  `merged/` that the line also covers (the builder's declared choice).

**Trace.** `covering`'s `(May::Rebuilt, _, Some(Entry::File{..})) => true` (`byte_cell.rs:1258`) excuses any
rewrite under the place, whatever stood. The staged-doc loss rule (`byte_cell.rs:1809`) skips a doc whose path still
exists.

**Concrete case.** Take `milestone-join-two-re-entered-sub-tasks`, or any `add-task` cell with a first sub-task
holding staged prose. A regression in which the re-seed re-mints a standing area — resetting
`.jigc/tasks/add-alpha/docs/commit:add-alpha.md` or `base.json` — is excused by `MJ2`/`MA6`, and the cell **holds**.
The prose is gone. The records say only that these lines *"excuse nothing"* in any cell, which is true of the cells
and hides what the lines would excuse.

**Smallest fix.** A word for "an area the record names, where nothing stood" (`<sub-task>` with `<minted>`'s
condition), or a `May` "made where absent". Use it on the five area lines and on `MP1`. Add a fence case: a standing
sub-task's staged doc rewritten under the re-seed is not excused.

### F3. Blocking (record the human rules from). Entry 18 is a genuine "two things", but it is under-quoted on both sides, and its claim *"the design decision predates the rule (M46, 2026-08)"* is incomplete

**Location.** Rows record §6, entry 18; readings §17; findings §19; the family record §3; `DECISIONS.md` (the
`9b190346` and `ed440461` entries).

**For narrating, and later than the exit rule (not quoted).**
- `finalize.md:378`: *"Files git ignores — build output — stay named by the teardown and are never refused over,
  **and a *landed* sub-task's unstaged and untracked leftovers stay named too**"*. `git log -S` dates this to
  `855fef44`, 2026-10-06: two days after the exit rule, in the build of the human's ruling on item 6.
- `DECISIONS.md:4577` (that build's entry): *"A sub-task the boundary lands is asked no bytes: its staged set was
  taken, and its unstaged and untracked leftovers stay named, on the M46 ruling this one does not reopen."* This is
  the builder's choice, not the human's words, so it should be said as such.
- `storage.md:300`: the working area's complement is moved aside **because** that door *"cannot narrate the loss on
  the *landed-boundary* warrant its siblings use"*. That sentence names the landed-boundary warrant as the
  worktrees', which weakens the robust answer's *"the disposition the same boundary gives a working area's
  complement"*.
- `command-output-contract.md:540-542`: the narrated destruction is a pinned key (`sub_tasks[].discarded`, *"bytes
  **destroyed**"*).

**Against, the human's own (not quoted).**
- `DECISIONS.md:5499` (2026-10-04): *"**Over take-and-name.** *Why:* take-and-name would knowingly ship an exit-0
  loss of a sub-agent's deliverable"*.
- Items 10/4 (`DECISIONS.md:4719`): *"Over reversing to the wording of the registration ruling — land at exit 0 and
  name what was dropped"*.
- Item 6 (`:4730`): *"Over … declaring it a bound"*, for the discarded-sub-task case.

**Answer.** Genuine two-things. The text should quote these, say that the 2026-10-06 sentence is a builder's
restatement and not a ruling, and correct "predates". The cell's red-and-marked reading is right.

### F4. Blocking (count). The family record says "7 rows, 57 lines; `finalize` 21 lines, where 18". The data say 59 and 19, where 16

**Location.** `byte-table-milestone-family.md:15`, source "(suite)".

**Derivation.** A comment-stripping parse of each `Row`'s `lines` array:

| row | lines |
|---|---:|
| `CREATE` | 8 |
| `ADD_TASK` | 10 |
| `ADD_FROM_SPEC` | 10 |
| `PROVISION` | 8 |
| `EXECUTE` | 6 |
| `JOIN` | 6 |
| `DISCARD` | 11 |
| **the seven rows** | **59** |
| `FINALIZE`, at `fff27d7b` | 16 |
| `FINALIZE`, at `ed440461` | 19 |

The ids, `C` lines included, cross-check this: `FINALIZE` has 12 ids plus 4 `ON_A_REFUSAL` at the base, and 15
plus 4 at HEAD. The same record's whole-table figure, 198 lines (= the doc family's 134 + part A's 2 + **62**), is
consistent with 59 + 3, not with 57 + 3.

### F5. Should-fix. `join`'s row lacks the exit-0 edge-index rebuild its docs give. The record says *"execute and join write nothing but the re-seed"*, but every `join` cell pre-builds the index (fixture-topology masking)

**Driven on this build** (`probe-join-index.sh`):
1. Two re-entered sub-tasks.
2. `mv .jigc/index/edges.json` away.
3. `milestone execute` exits 0 and leaves the index absent.
4. **`milestone join` exits 0 and writes `edges.json`.**

**Docs.** `storage.md:354`/`finalize.md:210`: *"next read rebuilds"*. `JOIN`'s lines are `MJ1`, `MJ2` and the four
`C` lines, which apply on a refusal only. So the cell that would show this reads *written outside* where the docs
allow it.

**Masking.** Every `join` cell's leads re-enter the sub-tasks (`re_entered!`, `milestone.rs:2039`), which builds the
index first. The record's §4b *"What `execute` and `join` write. Nothing but the re-seed"* and findings §18 rest on
that.

**Bound.** An instance: one door driven, and the mechanism is unbounded over doors that read the index.

**Fix.** An exit-0 index line on the rows of doors that read, from the same sentences as `T5`/`M10`. Add a cell
with the index absent.

### F6. Should-fix (low). The two scheduled `M2 × E` needs are buildable now, and a cell would see something the `T1` cells do not

**The scheduled reason.** *"`fresh` holds no committed doc a sub-task copies in … the task family's `T1` cells hold
the promote the boundary shares"*.

**Why it does not hold.** `committed-singletons` is already a gate arm. I drove `probe-m2e.sh`: a sub-task in its
worktree sets `vision:vision#thesis`, and the user appends a line to `VISION.md` in the main checkout.

| | this build | rc.24 |
|---|---|---|
| before the first write | **exit 0**: the boundary commit carries the user's uncommitted line beside the sub-task's thesis | **exit 3**, `reconciliation.conflict-block` |
| after the first write | exit 3, conflict-block | exit 3, conflict-block |

So the cell is buildable today. It meets bound 2 (item 23) across checkouts, which is worth a mark, and it differs
between binaries.

### F7. Note. Two of the four new keys share one writer with the task family's keys, and one key has no repro block

- **The shared writer.** `commit_record_only` (`cli/src/milestone.rs:999-1012`) does the `git add -- <record>`
  (`:1006`) and writes `record-commit-msg.txt` (`:1009`). It is called through `commit_record_transaction` by
  `create` (`:960`), `add-task`/`add-from-spec` (`:2121`), `discard` (`:5871`) **and `task discard`'s
  `settle_discarded_sub_task`** (`:2705`).
- **What overlaps.**
  - `r1-milestone-record-doors-leave-their-message-file-in-the-milestones-area` is one mechanism with
    `r1-task-discard-a-rejected-record-commit-leaves-its-message-file-in-the-working-area`.
  - Three of the four cells of `r1-milestone-record-doors-replace-a-change-staged-at-the-record` are one mechanism
    with `r1-task-discard-a-change-staged-at-the-record-and-not-in-the-file-is-replaced`.
  - The fourth cell, the boundary's flip, goes through a different path.
- **Consequence.** One fix closes each pair. Triage should see them cross-referenced as one mechanism, not counted
  as four.
- **Missing repro.** `r1-milestone-record-doors-refuse-and-leave-the-directories-they-made` has no repro block in
  findings §17 (`pinning.md` §3). Mine is below, and it reproduces on both binaries.

### F8. Note. `<minted>` reads the working tree only, and cannot tell one mint from two

- **The working-tree read.** `Acting::minted` (`byte_cell.rs:1001-1007`) asks `before.files`. An id that HEAD's
  committed record owns but the worktree lacks — a record deleted from disk — counts as "nothing stood". That is
  wider than *"refuse a pre-existing id"*. Today the one such cell is guarded only by its listed outcome
  (`milestone.record-exists`).
- **One mint from two.** This bound is declared. Two concrete shapes hold:
  - `add-task` minting two areas and appending two items: `MA1` plus `MA3`'s `MergeInto` plus `MA4`.
  - `create` minting an area and a record under different ids.
- **Narrowing available now.** Require one distinct minted id per line for `create`/`add-task`, and one that the
  record after the command names.

### F9. Note. `Allows::Registered` checks neither the new worktree's `HEAD` nor its index

`byte_cell.rs:1875-1880` and `:1898-1901` excuse the new checkout by its path alone. The quoted sentence is
*"populates from the pinned base's *tree*"*: a worktree at the wrong base would hold. No byte is lost by it.

### F10. Note. The leads' reasons

- **`git_home_claims`.** *"The states that would (a permission bit …) are declared bounds"* misreads plan §1.1. That
  section puts *"git fails, the index is unreadable"* **inside** the clause, and lists hand-corrupted content among
  *"Not bounds"*. The true reason no cell exists is the frame's own: the manifest reads git's index, so a corrupt
  index breaks the before-picture.
- **`add-from-spec` with more than one criterion.** The stated *"frame's bound"* is liftable now: a `MergeInto`
  commit whose blob's lines stand in the after-file.
- **The message file after success.** A negative word, "must not stand after", is buildable.
- **The re-seed of an absent area.** It is buildable with the existing `Plant::Deleted` (for example
  `.jigc/milestones/<id>/tasks.json`).
- **Missing re-seed lines.** `discard`'s and `finalize`'s rows lack the re-seed lines that their own quoted
  *"Every verb, one site … `finalize` · `discard`"* gives them.

### F11. Note. One cost figure is labelled measured but is a cross-run difference

The *"+ ≈ 83 s"* in the family record's §2 table row *"the family, measured"* is a difference between two gate runs
at different loads (this one at load 18–36). Label it a cross-run delta. The projection rows are labelled correctly.

## Silences and open questions — verdicts

| item | verdict |
|---|---|
| Entry 18 (a landed boundary takes an unstaged or untracked file from a sub-task's worktree) | **Two docs saying two things, genuine** — but under-quoted (F3). |
| A pending config delta rides the boundary (record: "answered") | **Two docs saying two things, not answered** (F1): `finalize.md:160`/`:166`, `storage.md:308` against the G8 sentence of `task finalize`. |
| A change staged before `milestone create` | **Answered**, `command-output-contract.md:542`. |
| The milestone's area keeps its complement (`M6`'s new half) | **Answered**, `storage.md:304`: *"Both keep their complement"*. Grounded, and as narrow as the existing sub-task half: any path under `.jigc/displaced`. |
| *What `execute` and `join` write: nothing but the re-seed* | **False for `join`** (F5). |
| *A mint of two*; *`add-from-spec`'s later commits*; *the message file at exit 0* | Frame bounds, honestly stated; each could be narrowed now (F8, F10). |
| `MJ3`, `ME3` (no line) | Answered by their sentences. |

## Counts re-derived (all correct but F4)

- **The table.** 519 cells = 387 in the gate + 132 on demand (133 lines of the on-demand `.tsv` less its header).
  The fence is `(519, 387, 132)` at `byte_table.rs:8305`.
- **The family.** 165 cells = 117 in the gate + 48 on demand. That is the 172 `jigc milestone` rows of the print
  less 7 that stood before: the acceptance's three and the four `squash: false` cells. By commit:

  | commit | cells | in the gate | on demand |
  |---|---:|---:|---:|
  | `ffe4abe4` | 69 | 51 | 18 |
  | `17caeecb` | 31 | 13 | 18 |
  | `9b190346` | 65 | 53 | 12 |

- **Red on this tree.** 12 red under 4 keys, cell for cell the same on rc.24. The rc.24 print has 42 red among the
  172, which is 39 of the family (12 + 27) plus the acceptance's 3.
- **The whole table, this tree.** 225 hold + 210 refuse-nothing-changed, so **84** red.
- **The whole table, rc.24.** 209 + 143, so 167 not clean, of which 2 *not run*, so **165** red.
- **The expected-red list.** 80 cells under 20 keys (python parse); 68 + 8 + 0 + 4.
- **Marks.** 3: `Silence(18)`, `Bound(1)`, `Bound(2)`.
- **The amend.** `c6a3c076` became `9b190346`, message only. "five" became "four" and 81 became 80, both right. The
  entry's "49 hold" is 53 − 4, right. The family record's "fresh 101 gate cells" is right: one `add-from-spec`
  refusing cell is in `fresh`.

## Repros

```zsh
# from the worktree's root; BIN = the binary under test, absolute
# --- F1: an untracked file under .jigc/config rides the landed boundary (exit 0, both binaries)
rig=$(dev/jigc-rig --binary "$BIN" fresh) || exit; eval "$rig"; cd "$REPO"
mkdir -p ../outside; printf 'add a file\n' > ../outside/summary
$JIGC milestone create "Land it" > /dev/null; $JIGC milestone add-task land-it "Add alpha file" > /dev/null
$JIGC milestone provision land-it > /dev/null
(cd .jigc/worktrees/add-alpha-file && $JIGC workflow sub-task --task add-alpha-file > /dev/null \
  && $JIGC doc set-field commit:add-alpha-file#type --value feat --task add-alpha-file > /dev/null \
  && $JIGC doc set-slot commit:add-alpha-file#summary --from-file ../../../../outside/summary --task add-alpha-file > /dev/null \
  && printf 'code\n' > a.txt && git add a.txt)
$JIGC milestone join land-it > /dev/null
printf 'a note of the user\n' > .jigc/config/notes-of-the-user.txt
$JIGC milestone finalize land-it > /dev/null 2>&1; echo "exit=$?"            # exit=0
git cat-file -e HEAD:.jigc/config/notes-of-the-user.txt && echo committed     # committed
```

```zsh
# --- F5: join rebuilds an absent edge index at exit 0 (this build)
rig=$(dev/jigc-rig --binary "$BIN" fresh) || exit; eval "$rig"; cd "$REPO"
$JIGC milestone create "Ship it" > /dev/null
$JIGC milestone add-task ship-it "Add alpha" > /dev/null; $JIGC milestone add-task ship-it "Add beta" > /dev/null
$JIGC milestone provision ship-it > /dev/null
for t in add-alpha add-beta; do (cd .jigc/worktrees/$t && $JIGC workflow sub-task --task $t > /dev/null); done
mv .jigc/index/edges.json ../edges.was
$JIGC milestone join ship-it > /dev/null 2>&1; echo "exit=$?"; ls .jigc/index/   # exit=0; edges.json is back
```

```zsh
# --- F6: M2 x E before the first write, in committed-singletons
#     this build: exit 0, the user's line in the commit; rc.24: exit 3, conflict-block
rig=$(dev/jigc-rig --binary "$BIN" committed-singletons) || exit; eval "$rig"; cd "$REPO"
mkdir -p ../outside; printf 'add a file\n' > ../outside/summary; printf 'A sharper thesis.\n' > ../outside/thesis
$JIGC milestone create "Land it" > /dev/null; $JIGC milestone add-task land-it "Sharpen the thesis" > /dev/null
$JIGC milestone provision land-it > /dev/null
printf 'a line of the user\n' >> VISION.md
(cd .jigc/worktrees/sharpen-the-thesis && $JIGC workflow sub-task --task sharpen-the-thesis > /dev/null \
  && $JIGC doc set-field commit:sharpen-the-thesis#type --value docs --task sharpen-the-thesis > /dev/null \
  && $JIGC doc set-slot commit:sharpen-the-thesis#summary --from-file ../../../../outside/summary --task sharpen-the-thesis > /dev/null \
  && $JIGC doc set-slot vision:vision#thesis --from-file ../../../../outside/thesis --task sharpen-the-thesis > /dev/null)
$JIGC milestone join land-it > /dev/null 2>&1
$JIGC milestone finalize land-it > /dev/null 2>&1; echo "exit=$?"; git status --porcelain
```

```zsh
# --- F7: r1-milestone-record-doors-refuse-and-leave-the-directories-they-made (both binaries)
rig=$(dev/jigc-rig --binary "$BIN" fresh) || exit; eval "$rig"; cd "$REPO"
printf '#!/bin/sh\nexit 1\n' > .git/hooks/pre-commit; chmod +x .git/hooks/pre-commit
$JIGC milestone create ship-it > /dev/null 2>&1; echo "exit=$?"     # exit=1
find .jigc/milestones docs -print                                   # three empty directories, none before
```

The other three keys' repros in findings §17 and §19 were run as committed. Each reproduced on both binaries:

| key | result |
|---|---|
| key 1 | exit 0; nothing cached; 0 commits hold the line |
| key 2 | exit 1; rewritten by the refusal |
| key 4 | exit 0; `notes.txt (never staged)`; the worktree gone |

## Not checked

- No `dev/gate` was run, and no `cargo test`.
- F2's hide is traced through the source, not built as a cell.
- The `needs` block of the print: the family record's 167 and 165 needs.
- Linux.

<!-- end of report -->
