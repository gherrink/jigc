# Review: the byte table, task B10 (`09268a5e..1ec6f8d2`)

Reviewer: independent and read-only. Worktree `gherrink-jigc-route-catalog`, branch `fix/rc24-route-catalog`. Commits:

| commit | what |
|---|---|
| `c503e280` | the three rulings, `C4`, and the §6 sweep |
| `bab8794e` | the record and its `.tsv`s |
| `eb76c13e` | the directory arm |
| `1ec6f8d2` | `pinning.md` §7 and the pending rows |

No product source changed in the range. The last product change is `0ad081cd`.

## Verdict

- **The rulings are recorded faithfully.** All three are quoted as the brief hands them on: nothing narrowed, nothing widened.
  - Each ruling names the doc it makes wrong:
    - entry 25: `design/write-commands.md` → *Sub-agent re-entry*;
    - entry 4: `design/project-setup.md`'s boundary table, plus the rule owed in `design/assistant-adapter.md` §3;
    - entry 7: `design/storage.md` → *Edge index lifecycle*.
  - The four cells of entry 25 lost their mark. Nothing else moved: in the committed print, the block `excused` gives `C4` 0 and 0 in all 30 rows.
  - Keeping two re-entry keys is sound. One cell is *lost*, the other four only adopt, and one fix closes both.
- **`C4` holds.** Every probe I aimed at it comes out red:
  - a file beside `edges.json`;
  - a door that commits nothing;
  - a reset that landed no commit.

  A refusal that commits is still red for its commit; only the index part is excused, which is correct by the ruling.
- **The directory fix `eb76c13e` is narrower than its own rule** (H1). A directory made at `edges.json`'s path still holds under:
  - `T5`, `M10` and `RN5`, which name the stamped file in `Invalidated(".jigc/index/edges.json")` over `Under(".jigc/index")`;
  - `C1` and `C2` on any refusal.
- **One arm older than the range now reaches every committing door at exit 0** (H2). A link, or an empty directory, at the index's place replaced by a regular file holds. The same link taken away is red.
- **§7 claims more for a new writing door than the fences hold** (H3). Probe: a new write leaf, given a count line *untouched* by hand and a scheduled line, passes both fences.
- **Entry 4's finding is confirmed on both binaries.**
  - The `Spawn:` line is bare, and the profile's template is never rendered.
  - jigc's process spawns are 95 `Command::new` sites:
    - 89 run `git`;
    - one is `invoke_probe(program)`, which is jigc's own image or the `JIGC_DOC_CODE_PROBE` override;
    - five are in `#[cfg(test)]` modules.
  - So jigc launches no assistant today.
  - The fix as worded leaves two things open (N8): the fence's shape, which should be an allowlist of what jigc may spawn, and where the rendering lives.
- **Counts.** Every headline count of the record re-derives exactly from the committed print and the suite:
  - 760 cells; 580 in the gate and 180 on demand;
  - 175 red on this tree, 161 of them in the gate;
  - on rc.24, 269 red and 2 not run;
  - 43 keys with cells, and 65 keys listed;
  - 298 lines, 179 of them excusing nothing;
  - 680 needs, 11 of them scheduled;
  - 174 cells red on both binaries;
  - one regression, and two cells lost only here.

  The per-key, per-group, per-leaf and per-state tables also re-derive. I matched the keys list against `EXPECTED_RED` key for key, and against the findings' headings.
- **The record's "regression" definition hides real ones** (N1). Four cells have a harm class on this tree that rc.24 did not show. The `reg.` column marks none of them, and two are not mentioned at all.
- **"No entry of §6 is open" is not what entry 1's body says** (N4, Q1). Its open points (ii) and (iii) are unruled.
- **Hygiene is clean.**

## What I ran

- **Binaries.**
  - rc.24: the rebuild from `91834b5e`, sha256 `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d`, checked.
  - This tree: the worktree's `target/debug/jigc`, sha256 `a71a816e…`. It is current, since no product source changed after `0ad081cd`.
  - Both are copied into the scratch dir.
- **Frame probes.** In a `git archive` copy of `1ec6f8d2`, with two probe modules appended to `crates/cli/tests/byte_table.rs`, run with `cargo test -p jigc --test g_migration review_probe_b10 -- --nocapture` and a scratch target directory. The four new or touched fences pass in that copy:
  - `a_commit_lets_…`;
  - `a_line_over_one_files_place_…`;
  - the two leaf fences.
- **Rigs.** `dev/jigc-rig --binary <bin>`, stdout captured, evaluated in two steps. Scripts D1–D3.
- **Counting.** The record's §9 commands, run as printed, plus python over the committed `.tsv`s and a parse of `EXPECTED_RED`.

## Frame holes

### H1. Medium-low. `eb76c13e`'s rule is not held at a cache's file inside a directory-wide line

**Rule.** *A line over one file's place lets no directory be made there.* `C4`'s path is covered. The same file's path inside an `Under` line is not.

**Location.** The arm at `byte_cell.rs:2615-2619` admits a directory made where none stood whenever the line's place is `Where::Under(_)`. It does not ask whether the path is the cache's own file. `T5`, `M10` and `RN5` even name that file: `May::Invalidated(".jigc/index/edges.json")` (`task.rs:166`, `milestone.rs:338`, `movers.rs:195`).

**What it excuses.** A door whose run makes a directory at `.jigc/index/edges.json`:
- at `task finalize`, `milestone finalize` or `rename`, at exit 0;
- at any door's refusal, under `C1`.

The same holds for `.jigc/state/file-state.json` under `C2`. The commit says *"no other line names one file's place with them"*. That is true of `Where::Path`, and false of the stamped file `Invalidated` names.

**Probes** (`review_probe_b10_c4`):

| case | came to |
|---|---|
| D: a `T5`-shaped line plus a commit, a directory made at `edges.json` where none stood | **holds** |
| E: `ON_A_REFUSAL`, exit 1, a directory made at `edges.json` | **refuses, nothing changed** |

**Bound.** The mechanism is one arm. The lines exposed to it are the `Rebuilt`/`Invalidated`/`LaidOut` lines over `Under(".jigc/index")` or `Under(".jigc/state")`. That is **29 definitions** by `grep -A1 'Where::Under(".jigc/\(index\|state\)")' … | grep -c 'Rebuilt\|Invalidated\|LaidOut'`. Two of them are `C1` and `C2`, which stand in all 30 rows.

**Fix.** In the directory arm, refuse when `may == Invalidated(stamped)` and the path is `stamped`. For `C1`/`C2`, add a short list of the caches' own file names, kept beside `NO_CACHE` (`edges.json`, `file-state.json`), at which no directory may be made.

### H2. Low. *Rebuilt*/*invalidated* takes any entry that stood before: a link or an empty directory at the index's place, replaced by a regular file, holds — and `C4` carries this to every committing door at exit 0

**Location.** `byte_cell.rs:2610`: `(May::Rebuilt | May::Invalidated(_), _, Some(Entry::File { .. })) => true`. The `_` takes a `Link` or a `Dir` before.

**Probes** (`review_probe_b10_c4`):

| case | came to |
|---|---|
| A: `.jigc/index/edges.json` a link to `../../notes.md`, then a commit door replaces it with a regular file | **holds** |
| A2: the same link, taken away | *written outside its allowance* |
| B: an empty directory at that path, replaced by a file | **holds** |

The asymmetry is the hole: the same link is red when taken away and green when replaced.

**Why it is low.** The arm predates the range, and no cell reaches it:
- `aimed` refuses any plant at `C4`'s place (`byte_cell.rs:1063-1070`, new in `c503e280`);
- an `Under` place draws only **U**.

Before `C4`, the arm was reached only on a refusal (`C1`) or at three doors. Now it covers every committing door at exit 0.

**Fix.** Match `(…, None | Some(Entry::File { .. }), Some(Entry::File { .. }))`. A link or a directory standing where a cache file goes is the user's, which is entry 23's and entry 25's principle.

### H3. Medium. §7 says a new writing door "comes with its row"; the leaf fence lets one be scheduled by a line written by hand

**The claim** (`implementation/pinning.md` §7, *How a new writing door is added*). A scheduled line *"must be a leaf the committed count holds untouched since the previous release … which a new leaf never is — so a writing command the product gains comes with its row"*.

**What the fence reads.** `every_scheduled_leaf_is_one_the_count_holds_untouched` (`byte_table.rs:2724`) reads the column of a markdown table, `byte-table-leaves-touched-since-rc24.md`. A person writes that table, and nothing derives it from code.

**Probe** (`review_probe_b10_a_new_leaf_scheduled_as_untouched`). The fixture:
- a new leaf `task brand-new`;
- its count line `| jigc task brand-new | untouched | untouched | - |`;
- a scheduled line for it.

Result: `uncounted` returns `[]` and `unanswered` returns `[]`. Both fences are silent.

**Second half (a lead).** The count was read at `aeeb0839`. A later change to the handler of a scheduled leaf (`task bind`, the five `config` leaves) is seen by no fence.

**Fix.** Hold the count's leaves to a frozen list of rc.24's 36 write leaves, which the count already enumerates. A leaf not on that list was not in rc.24, so it is touched by definition and cannot be scheduled. Alternatively, §7 says plainly that the escape is a count line a person writes. The second half needs the count re-read at each release, or a hash of each scheduled handler's file.

## Record notes

1. **(High.) The record's regression is "red here, clean on rc.24". That definition hides a class worsening.**
   - *How the count was derived.* A python pass over the committed print, comparing column 7 (rc.24) with column 10 (this tree). It looked only at the harm classes: *lost*, *git moved*, *written outside*, *refused and changed*.
   - *Result.* **Eight cells** show a harm class here that rc.24 did not show:
     - **1** is the counted regression: the `placement-root` link to nothing.
     - **2** are the lost-only-here cells. They are flagged in prose, but marked `reg. no`:
       - `task-finalize-a-hand-edit-before-the-first-write-to-the-same-part`;
       - `milestone-finalize-a-hand-edit-before-a-sub-tasks-first-write-to-the-same-part`.

       rc.24 refused at exit 3 and kept the line. This tree loses it at exit 0. Under the exit rule's first clause, that is a regression.
     - **2 are worse here, and the record does not mention them:**
       - `setup-at-the-settings-record-a-committed-file`. rc.24 acted at exit 0, *not listed* only, and left the user's committed file alone, since rc.24 has no settings record. This tree replaces it: *git moved*.
       - `relocate-a-link-to-nothing-at-the-notes-new-home`. rc.24 was *not listed* only. This tree gives *git moved* and *written outside*, by the same `5f5b273a` mechanism as the regression (B9 review, H5).
     - **3 change kind without getting worse:**
       - two `config set` cells in the decisions' place, from *git moved + outside* to *refused and changed*;
       - `setup-at-the-settings-record-a-change-staged-and-not-in-the-tree`, from acting to refusing with the install left. The install stands either way.
   - *Fix.* Define a regression as **a harm class on this tree that rc.24 did not show for the same cell**. Read it off columns 7 and 10, and add the command to §9. The `reg.` column then marks four cells under four keys, plus the one it marks now:

     | key | cells |
     |---|---|
     | `r1-task-finalize-a-hand-edit-before-the-tasks-first-write-…` | 1 |
     | `r1-milestone-finalize-a-hand-edit-before-a-sub-tasks-first-write-…` | 1 |
     | `r1-setup-replaces-a-file-the-user-committed-…` | 1 |
     | `r1-relocate-parks-an-untracked-occupant-of-a-new-home` | 1 |
2. **Numbers in `c503e280`'s `DECISIONS.md` entry** (`DECISIONS.md:79`):
   - *"Lines 299, where 269"*. The print says **298, where 268**: the mint family's committed excused block has 268 lines, and that plus 30 is 298.
   - *"271 on the previous release (two not run)"* counts the not-run cells as red. The next commit's entry corrects exactly that error in the mint record.
   - The fence's doc comment (`byte_table.rs:9634`) says *"Red first, twice"* and then lists three cases. The commit says three.
3. **Previous-release words that are not on the committed print.** The print has no words column, so these words rest on older runs. Either cite those runs or drop the words.
   - *"rc.24 refused them (`reconciliation.conflict-block`)"* (record §3.3; `bab8794e`'s entry). The print holds exit 3 and the class. The code rests on findings §4's run (`byte-table-findings.md:346`).
   - *"parked at exit 0 here"* for the regression. That word rests on the B9 review's D1.
4. **"No entry of §6 is open" is false by the entry's own body.** The claim stands in the rows' record §6 header, in `c503e280`'s sweep text and in record §1. Entry 1, *What the ruling leaves open* (`byte-table-acceptance-rows.md:1743`), says otherwise:
   - **(ii)** *"whether that is what was meant is the human's"*;
   - **(iii)** *"stays red, cell by cell, until ruled"*.

   The heading's *"its first open point"* covers (i) for `setup` alone. No ruling settles (ii). (iii) is carried de facto by findings keys whose `ruling` column is `—`. See Q1.
5. **The reason given for the two "should make" keys is not exact** (record §3.9; findings → `r1-migrate-corpus-no-commit-…`). The reason reads *"a line lets a write happen and never demands one"*. But the frame does demand a commit: `must_land` (`byte_cell.rs:3762-3767`) is what makes `uninstall`'s 15 cells red under `U9`. A sibling demand, *what the lines changed stands staged*, can be built the same way. So both keys have a buildable cell (`--no-commit`'s staging, and the root knob's staged half), and it is owed.
6. **`wf-byte-table-rest` does not hold exactly what remains.** Missing:
   - two items of the plan's scheduled list (`byte-table-plan.md:377`):
     - *"the catalog's states as refusal cells"*: record §5 says *"are not built"*, and they are carried nowhere;
     - *"two plants at once"*: dropped as *"no need of the frame"*. That reason is circular, since the frame derives only single-plant needs. Dropping a plan item is the human's to decide, or it should be carried.
   - Record §4's *"`task discard` of a settled sub-task whose area stands again … Buildable"*;
   - the *staged* word of N5;
   - H3's two halves and L1's plant kind (a link on the way to a place).
7. **§7 otherwise reads true to the frame, and it points rather than restates.** Its named fences exist and pass. The nextest group is as described. One thing is unfenced: *"a key … has its entry in byte-table-findings.md"*. No test reads the findings file. All 65 keys have a heading today, by my check: 69 headings, of which 65 are live and 4 retired.
8. **The spawn finding (§27) needs these sharpenings so that its fix holds the human's rule:**
   - *The probe program.* `DECISIONS.md` (`c503e280`) calls the doc-code probe *"the pack's probe program"*. It is jigc's own image (self-exec), or `JIGC_DOC_CODE_PROBE` (`invoke.rs:79-84, 100-105`).
   - *The fence.* *"A source fence that no jigc code launches an assistant program"* has no shape. The robust shape is an **allowlist** of what production code may spawn: `git`, the own image, and the override while it exists. It runs over every `Command::new` outside `#[cfg(test)]`, which today is 95 sites (by grep): 89 `git`, 1 `invoke_probe`, 5 in tests. A denylist of assistant names would miss the next spawn.
   - *Where rendering lives.* The fix should say the **CLI** renders, because profiles ride in `cli` and the engine stays adapter-free (`module-layout.md`). It should also say what the `Spawn:` line looks like once rendered: the route catalog runs its payload, and `workflow-dialect.md` defines the `` Spawn: `<launch-line>` `` grammar.
   - *The doc count.* §27 names two docs. By grep, the class is five sentences in four docs: `assistant-adapter.md:127`, `workflow-dialect.md:129` and `:389`, `write-commands.md:231`, `worked-examples.md:618`. None is wrong once the product renders.
   - *The §27 repro.* Run as printed on both binaries, every answer matches its comment: exit 0, the bare line counted once, *Use your Task tool* absent from the output and present in the profile.
9. **`EXCUSES_NOTHING` gives `NO_COMMIT_TOOK_THE_INDEX` to doors that never commit.** The reason reads *"a cell that lands a commit … none does"*. That is vacuous at the eight `doc` leaves, `start`, `workflow`, `task amend` and `relocate`. Their exact reason is *this door lands no commit*. Low.
10. **How the byte table reaches `fix/rc24-tier1`: nothing owes it.**
    - `crates/cli/tests/byte_table.rs` and `route_catalog.rs` are absent from the tree of `origin/fix/rc24-tier1` (checked with `git ls-tree`). The merge-base is `f505dc2a`.
    - Plan §9 speaks of *"the join"* only for a CI job.
    - `wf-route-catalog-rest` mentions *"that branch's merge"* with no trigger.
    - No record or pending row owes merging `fix/rc24-route-catalog` into the loop branch before the run's opening, and the table is the run's instrument there.
11. **Hygiene is clean.** No host path, name or `he`/`him` in any `+` line or commit message. Each commit carries the harness's `Claude-Session` trailer, as main's history does. That is attribution, not something leaked by this range.

## Leads

- **L1. A link at the sub-task's docs directory → the first re-entry writes outside the repository at exit 0, on both binaries** (D2).
  - *Setup.* `.jigc/tasks/add-alpha/docs` is a link to a directory outside the repository.
  - *What happens.* `jigc workflow sub-task --task add-alpha` exits 0. `commit:add-alpha.md` and `provenance.json` appear in the outside directory.
  - *Why no cell sees it.* `aimed` gives an `Under` place only **U**, so no need is derived (the B9 review's H4 lead, at a parent).
  - *The ruling.* Entry 25 says *"the check follows no link"* at the commit doc's place. Its fix should take in the area's directories too.
  - *Bound.* Instance, unbounded: every provisioning write under a directory it does not check.
- **L2. Which profile renders the `Spawn:` line where no adapter is installed** — or where a second adapter is installed. The finding does not say.
- **L3.** H3's second half: the count cannot see a later change to a scheduled leaf.

## Questions for the human

**Q1. Entry 1 (ii). May a refusal write the file↔state hashes?** One doc says two things.

- **Doc, one side.** `design/storage.md:312`, *Derived caches*: *"The edge index and the file↔state hashes are **rebuildable from the committed docs**"*. `C1` and `C2` quote this sentence.
- **Doc, other side.** `design/storage.md:314`, the same section: *"The **file↔state hashes** carry **no stamp and no rebuild path**"*.
- **Why it matters.** Under `C2`, a refusal may write `.jigc/state/`. In the print, `C2` excused 22 differences in `setup`'s cells here (26 on rc.24) and 1 in `rename`'s. A baseline recorded over a user's hand edit makes that edit read as jigc's own to the next drift check. That is where entry 1's *"bound that is not held would bite"*.
- **Robust answer.** The hashes are a baseline, the authority on what counts as an out-of-band edit; they are not a cache. A refusal leaves them as they were. `storage.md:312` is wrong to group them as rebuildable, and `C2` narrows to what is rebuildable.
- **Which is wrong.** The doc (312), and the frame's `C2`. Whether the code is wrong is not established: I did not find a refusal that writes a baseline over an edit (D3: a refused `setup` writes nothing there).
- **Until ruled.** Entry 1 is listed as open again.

**(iii)**, the minted work unit, needs no question. The robust answer — a refusal leaves no area — is what the findings keys already carry. It wants a sentence that says it is ruled, or is to be ruled.

## Counts re-derived

| count | how | result |
|---|---|---|
| the record's §9 commands | run as printed | 760 · 175 · 580 · 161 · 180 · 269 · 2 · the regression · 2 lost-only-here · 43 · bound 1 · 680 · 11 · 298 · 179 · 174 |
| `EXPECTED_RED` | python parse | 45 entries, 43 keys, 161 cells, all unique |
| keys in the record | against `EXPECTED_RED` | all 43 listed; per key, the cells red in the gate match |
| keys red here / on rc.24 | against the print | every one of the record's per-key counts matches |
| group heads of §2 | summed | match (`config set`: 17 here / 16 on rc.24) |
| `C4` in the block `excused` | 30 lines | 0 / 0 |
| own lines / cache lines | | 148 (35 excusing nothing) / 150 (144) |
| differences excused | summed | 5,923 here / 6,512 on rc.24 |
| per-leaf and per-state tables (§3.4, §3.5) | from the print | every cell matches |
| marks in the print | | 1 (bound 1); no `Silence(25)` left in the source |

## Repros

```zsh
# Frame probes: appended to a scratch copy of 1ec6f8d2's crates/cli/tests/byte_table.rs
#   (modules review_probe_b10, review_probe_b10_new_door), then
#   CARGO_TARGET_DIR=<scratch>/target cargo test -p jigc --test g_migration review_probe_b10 -- --nocapture
# PROBE A  link-at-index-replaced-by-file (C4, commit):         holds                              (H2)
# PROBE A2 link-at-index-removed (C4, commit):                  written outside its allowance
# PROBE B  empty-dir-at-index-replaced-by-file (C4, commit):    holds                              (H2)
# PROBE C  temp-file-beside-index (C4, commit):                 written outside its allowance      (C4 holds)
# PROBE D  T5-shape line, dir made at edges.json, commit:       holds                              (H1)
# PROBE E  C1 on a refusal, dir made at edges.json:             refuses, nothing changed           (H1)
# PROBE F  refusal that committed, index gone:                  refused, and changed (the commit; the index excused by C4)
# PROBE G  HEAD reset back, index gone:                         lost                               (C4 holds)
# PROBE new-leaf: count fence says []; leaf fence says []                                          (H3)
```

```zsh
# D1: §27's repro, as printed (both binaries: exit=0 · 1 · 0 · 1)
root=<worktree>; rig=$("$root"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit; eval "$rig"; cd "$REPO"
$JIGC milestone create "Ship it" >/dev/null && $JIGC milestone add-task ship-it "add alpha" >/dev/null && $JIGC milestone provision ship-it >/dev/null
$JIGC milestone execute ship-it > "$RIG/out" 2>&1; echo "exit=$?"
command grep -c '^Spawn: `cd .* && jigc workflow sub-task --task add-alpha`$' "$RIG/out"; command grep -c 'Use your Task tool' "$RIG/out"
```

```zsh
# D2 / L1: a link at the docs directory (both binaries: exit=0; commit:add-alpha.md and provenance.json land outside)
rig=$("$root"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit; eval "$rig"; cd "$REPO"
$JIGC milestone create "Ship it" >/dev/null && $JIGC milestone add-task ship-it "add alpha" >/dev/null && $JIGC milestone provision ship-it >/dev/null
O=$(mktemp -d "$RIG/outside.XXXXXX"); ln -s "$O" .jigc/tasks/add-alpha/docs
(cd .jigc/worktrees/add-alpha && $JIGC workflow sub-task --task add-alpha >/dev/null; echo "exit=$?")
ls -A "$O"          # commit:add-alpha.md  provenance.json
```

```zsh
# D3: a refused setup (pre-write) leaves nothing under .jigc/state on either binary
rig=$("$root"/dev/jigc-rig --binary "$BIN" bare 2>/dev/null) || exit; eval "$rig"; cd "$REPO"
mkdir -p .jigc && echo "the user's own notes" > .jigc/AGENT.md
$JIGC setup; echo "exit=$?"; find .jigc -type f     # exit=1 setup.dirty-install-path; .jigc/AGENT.md only
```

```zsh
# N1: cells with a harm class on this tree that rc.24 did not show (from the committed print)
python3 -I - <<'EOF'
import csv
rows=list(csv.reader(open('byte-table-record-cells.tsv'),delimiter='\t'))[1:]
H={"lost","git's state moved outside its allowance","written outside its allowance","refused, and changed what a refusal may not"}
cls=lambda s:{x.strip().removeprefix('also ') for x in s.split(';')}&H
for r in rows:
    if cls(r[9])-cls(r[6]): print(r[0],sorted(cls(r[6])),'->',sorted(cls(r[9])))
EOF
# 8 lines
```

## Not checked

- No `dev/gate`, no whole-table run, and no Linux run. The cells were not re-run; the counts come from the committed print.
- Which `rename` cell `C2` excused, and what the `setup` cells under `C2` wrote. Not driven.
- Seconds.

## Scratch copy, for the fixers

`<scratch>` = `<scratch>/b10-review.mXmotS`

| item | where |
|---|---|
| the two binaries | `rc24/jigc`, `tree/jigc` |
| the probe tree | `src/` |
| the rig scripts | `d1-spawn.sh`, `d2-docs-dir-link.sh`, `d3-setup-refusal-state.sh` |
| the logs | `probe1.log`, `probe2.log`, `fences.log` |

<!-- end of report -->
