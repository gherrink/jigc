# Review of `pre-opening-plan.md`: everything owed before the first stabilization run's opening

*Adversarial and read-only. 2026-10-27, against `fix/rc24-tier1` @ `0bd8a5b5`, with a clean tree. I checked every finding against the files. I ran no gate and no docker. The probes I did run: a `gitleaks` scan over `origin/main..HEAD` and over `DECISIONS.md`; a join of `scheduled.tsv` to `sites.tsv`; and `gh run view` on the last eight CI runs of the branch.*

**Counts:** 5 blocking · 11 should-fix · 9 notes.

**The forks:**
- **Fork 4 is genuine only in a narrower form than the plan gives it.** It is best put to the human together with a fact the plan leaves out (B2).
- **Decision (a)** is the human's, ordered on 2026-10-27.
- **Forks 1, 3, 5, 6 and 7 are settled.** Each is settled by a ruling or a record, named below.
- **Fork 2 is the order of the work**, not a choice of design.
- **(b) and (c) are not genuine forks.**

---

## Blocking

### B1. §0 names the wrong `not-counted` row. `C17` then promises cases for three sites that are the bodies of `Finding`'s own constructors.

**Evidence:**
- `scheduled.tsv` has 144 rows classed `unchanged` and one classed `not-counted`. The `not-counted` row is **`crates/engine/src/finding.rs::graded::dynamic#1`**, with the note *"it is the body of `Finding::graded`, the constructor every graded site calls"*. It is not `author.rs::reject::dynamic#1`.
- `author.rs::reject` is an ordinary row: blocking, classed `unchanged` (`sites.tsv`: `unassigned … reject dynamic 1 graded blocking`).
- `C17`'s `finding` 3 are `finding.rs::block`, `::blocking` and `::graded` (from the join of the two files). These are the `Self { … }` literals inside `Finding::block`, `Finding::blocking` and `Finding::graded` (`crates/engine/src/finding.rs` lines 1771, 1804 and 1828). Every refusal of the product passes through them.

**Why this blocks.** No refusal or route can be followed "at the constructor". So `C17` cannot do what it promises, and P1's wording does not hold. P1 says the `not-counted` class is refused for any site *"except the one row standing today, which C17 gives a case"*. Fork 3's option A would also read, for these three sites, *"every code the product mints"*.

**Fix.** Before `C16`, decide how these three are answered. Either:
- the site scan (`support/refusal_sites.rs`, test code) leaves out the bodies of `Finding`'s own constructors, and the list is regenerated, which makes the count 142; or
- each gets a ruled reason, *"the constructor every site calls; its callers are the sites"*. This would be one of P1's batched halts.

Then correct §0, P1 and the size of `C17`.

### B2. The regression base (`RB1`–`RB3`) is built for a path that cannot run before the `fix` half is repaired. As written, `RB2`'s red-first arms cannot be built, and two of its parts are aimed at the wrong place.

**Evidence:**
- **The `fix` stage refuses to start** (`NOT_FIT`, `stabilize.js:1356`). The workflow doc says *"`test` is sound for round 1 and for what follows it without a fix, and no further"* (`stabilization-workflow.md:655`, `:739`).
- **Round 1 has no base**, which is `RB1`'s own arm. Rounds 2 and later, and the audit of a fix diff, both need a landed round, and only the `fix` stage lands one.
- **The simulation has no round-2 scenario at all.** No test of `stabilize_simulation.rs` gets past round 1. So `in_round_two_a_new_door_broken_by_a_fix_is_a_regression_and_goes_to_a_fixer_unasked` needs a round-1 landing that some fixture fakes, and the plan names none. *"Goes to a fixer"* is also unreachable while `fix` is refused.
- **`RB2`'s *"`VERDICT_SCHEMA` gains `base`"* and *"`NEEDS` rows"* go against the filed-verdict design.** On the path with a queue, *"its verdict is one file, filed by the call its prompt spells … NO FIELD OF IT IS NEEDED BY THE STAGE"* (`stabilize.js:2423-2427`). The self-test pins `VERDICT_SCHEMA`'s keys exactly (`:3433`). The base belongs to the filing, which is `RB1`'s `verdict` act, with its binary's hash, and nowhere else.
- **Fork 4(A)'s fix-stage half cannot be built now.** `T10` would still discharge `wf-regression-base` as a whole. That would record a half that does not exist as built.

**Fix:**
- Tell the human, in `D0`, that ruling 2 has no effect in round 1 and does nothing until the `fix` half's repair.
- Keep `RB1` (the record script, testable on its own).
- Take `VERDICT_SCHEMA` and `NEEDS` out of `RB2`. Its harness half is then only the third build and the `BASE:` line.
- Either name the fixture that reaches round 2 (a landed round written by the record script's writers directly, which `wf-writers` says a direct caller can do), or move `RB2`'s round-2 scenarios and the fix-audit half to the `fix` half's repair, under a pending row whose trigger is that repair.
- Which of those two is the human's choice, because it moves a ruled task. See fork 4.

### B3. `T2`'s partition contradicts `T8`'s split into parts.

**What conflicts:**
- `T2` holds that *"one row per site of the family"* and that the partition is held to `projection.tsv`.
- `T8` holds that *"items over 12 sites are split into parts by file (F6)"*.
- `T1` gives an item a **family** and nothing else.

So an item that hunts part 2 of `task finalize` hands over rows for its own files. `T2` then refuses them as leaving the family's other sites out. Nothing tells `T2` which subset the part owes. A builder will either halt here or reach for a workaround.

**Fix.** Make the part data. Two ways:
- the projection defines each family's parts deterministically (by file, at most N sites each), and an item names `family` and `part`; or
- an item carries its file set.

In either case `T2` holds the partition to the item's own site set, and `T1`'s cell grows to match. Say in `T1`, `T2` and `T8` which way was chosen.

### B4. CI is red at the plan's own base, and the plan does not say so. A byte-table cell on Linux is not deterministic.

**Evidence:**
- `gh run view 40591137834` (head `0bd8a5b5`) fails `test (g_migration)` on `byte_table::the_cells_are_run_and_the_expected_red_list_is_exact::fresh`.
- Two cells failed:
  - `relocate-a-link-to-a-directory-outside-at-the-notes-new-directory` came out *"not the listed outcome"*;
  - `config-set-a-link-to-a-directory-outside-at-the-decisions-new-directory` came out *"refuses, nothing changed"*.
- `0bd8a5b5` touches no product or test code: agent definitions, docs and `tooling_index.rs` only.
- The two cells were added by `09270673`. That commit is an ancestor of `c9a22385`, whose CI run `40585095772` was **green**.
- So identical cells were green on one run and red on the next. This is the only CI run of the branch that was green since 2026-10-04.

**Why this blocks.**
- The human's ruling of 2026-10-27 (option A) fixed CI's Linux red *"so CI checks every commit on Linux again"*.
- The byte table is the instrument for the exit rule's first clause. An instrument whose answer changes from run to run is in the class *"records falsely"*, and the scale ruling of 2026-10-07 repairs that class before the opening.

**Fix.** Add a task ahead of the P lane's Linux runs:
1. Reproduce the `fresh` arm in `dev/runner-faithful`, repeated (after R1, so in the runner's posture).
2. Find the source of the nondeterminism.
3. Fix it red first.

`R1`'s *"`g_migration` … green, or each difference listed"* will meet the same failure. Point it at this task.

### B5. `J2` as written cannot meet the doc's condition.

**Evidence:**
- The diff `J2` would review starts at the last re-review of the `test` half (2026-10-07; first commit since, `6e78975b`) and runs to HEAD.
- That diff is already **194 commits; 327 files, +193 k lines; of which `.claude`, `dev`, `tooling-tests` and `implementation` alone are 61 files, +59.8 k lines.**
- This plan then adds `C1`–`C18`, `T8`, `K6`, `K7b`, `T11`, `T12` and the regression base.
- `J2` gives all of it to one `milestone-code-reviewer`.
- *"Fixes go back through the lane that owns each file"*, and nothing reviews those fixes again.
- Decision (a)'s supervised hunt can lead to edits of the hunter's prompt or definition after `J2`.

**Fix:**
- Limit `J2` to what the condition covers: the `test` stage's harness, its scripts, the definitions it launches, and the simulation and fences. The route catalog and byte-table cases are product tests and are held by their own fences.
- Split it by area, the way the audit is split: harness · record script · step tool · definitions · simulation and fences, plus one pass across all of them.
- Add a scoped re-review of the diff of `J2`'s fixes.
- Say that anything decision (a) changes in a harness, script or definition goes through that scoped re-review before the opening.

---

## Should-fix

### S1. The size is understated by several times.

**Evidence:**
- **The 3–7 minutes a site leave out gates, frame pieces, Linux runs and named-binary runs.** The plan's own source says so (`DECISIONS.md:3962` table note). The pilot's own ratio puts clocked minutes at **29.3 % and 39.5 %** of wall time (`route-catalog-pilot.md` §10, lines 489–491).
- **The dynamic sites mint many codes, each owed a case.** `MINTED_AT` alone holds at least 73 codes of catalogued prefixes for ten scheduled dynamic sites:
  - `compose.rs::blocking_workflow_refs` 27;
  - `write.rs::blocking_write` 10;
  - `migrate_corpus` 10;
  - `store.rs::block` 7;
  - `render`, `rollback`, `rename`, `validate` and `milestone`, 3 each;
  - `author` and `relocate`, 2 each.

  The codes of uncatalogued prefixes come on top (for example `rollback`'s five `ConflictDoor` values). The catalog's recorded rule gives *"a case a code it mints"* (fork 3).
- **The machine queue is not counted.** Every `C`-task's done-when runs the named-binary arm over the **whole** catalog (408–1029 s today, §16–18 of the pilot), plus one Linux run and one gate. With lane T's roughly 20 gates and `O-def`'s gate per edit, that is roughly 70 gates, 18 Linux runs and 18 whole-catalog runs, all in the one-at-a-time queue.

**Fix:**
- Price the plan from the pilot's wall-time ratio and from `C16`–`C18`'s code count.
- Run the named-binary arm filtered to the family, or once at `P-Z`.
- Say which lane is the critical path (P, plus the queue).

### S2. `g_migration` will pass CI's 10-minute bound partway through lane P, not at `P-Z`.

**Evidence:**
- Today's job ran 4.7 min (`c9a22385`) and 7.6 min (`0bd8a5b5`) against `timeout-minutes: 10`, with about 650 cases and the byte table. The tests alone took 342 s.
- About 145 sites at one or more cases a code is roughly double the catalog.
- `ci.yml`'s header rule sets a leg's bound at max(10, 2 × its four-CPU proxy).

**Fix:**
- Each `C`-task records its runner-faithful wall time for `g_migration`.
- Split the leg in the commit that takes its proxy past 5 minutes, using `ci_matrix_fence`'s parts pattern (the `include` rows of `g_tooling`). Not later, at `P-Z`.

### S3. Lane P cannot use lane R's faithful tool without a merge in the other direction that the plan does not list.

- `dev/runner-faithful` runs the script of the checkout it is called from.
- Its `--commit` must be *"reachable from HEAD"* (header, line 80).
- P's worktree is forked from `D0`. M-R only merges R into the loop branch, so P's `C1` would run the old, unfaithful image, or name a commit the loop checkout cannot reach.

**Fix:** after M-R, merge the loop branch into P (`--no-ff`, by the git agent) before `C1`, and add that merge to §1's list.

### S4. `P-NC` has no bound, and it comes after the family tasks, which causes rework.

- Lane P runs `C1`–`C18`, then `P-NC`. A `C`-builder therefore writes `NotConstructible` for a cause that a frame piece built later would have made plantable, and `P-NC` reopens finished families.
- `P-NC` has no per-piece stop rule; the `C`-tasks have *"any site over about an hour"*.
- The texts already number about 145 at cause level (154 `NotConstructible(` in the tables, 9 of them site-level), and the `C`-tasks will add many more.
- `setup.rs:3990` (*"git cannot be run"*) is no frame gap. Its reason is that the refusal happens earlier, under `setup.unverified-install-path`. Only `:4107` needs the per-command environment.

**Fix:**
- Split it. First classify the existing texts and build the shared frame pieces before `C1`. A residual sweep at the end covers only the new texts.
- Add a stop rule per piece.
- Correct the `:3990` claim.

### S5. Ruling 5's leftover is wider than `T0`.

- The *Tools* paragraph says *"under **Tools**, the last section of your report, or in your return where you write no report"*.
- On the path with a queue, the one triage writes no report of its own: it is rendered from its entries (`finding-triage.md:55`). Its return is `FILED_SCHEMA` (`stabilize.js:2393`), which has no free-text field.
- `T2`'s hunt writer renders the hunter's report the same way.
- So two roles that drive or judge have no channel for the *Tools* note.

**Fix:** have `T0`, or `T2` and the triage writer, take an optional `tools` text, scanned for hygiene and rendered as the report's last section. Hold it with an arm.

### S6. Decision (b), the gitleaks hit: the reason given is false, and the rewrite option is not viable.

**What I checked:**
- `gitleaks git … --log-opts origin/main..HEAD` finds exactly one hit: `85661425`, `DECISIONS.md:99`, `generic-api-key`.
- At HEAD only `DECISIONS.md:7754` is flagged (`gitleaks dir`). Line `7748` is not.
- The rule fires on `"key": "<64-hex>"`.
- **`fc8f7594` (2026-10-15) already renamed that field of the regression tool to `sha256`**, because the record script refused it. The shape therefore does not recur in the run's records.
- Had it recurred, it would be refused **when the record writes it**, not at the PR. Every batch is scanned with gitleaks (`dev/stabilize-record` header, step 4), and so is every push range (`vet --range`).

**Why a rewrite is not viable.** It would invalidate the 18 `commit:` pointers in `regression-set/intended-changes.tsv`, which the tool refuses when a pointer names nothing. It would also invalidate every sha the records cite after `85661425`.

**Why the toml entry and not a fingerprint.** `DECISIONS.md` is prepended, so a fingerprint keyed to a line drifts with every new entry.

**Fix.** One `.gitleaks.toml` entry keyed to `DECISIONS.md` and the regression-set JSON line's `"key": "<64-hex>"` shape. That follows the repository's own rule in that file: the exact file and the exact line shape. Do it before the PR, as an act with no fork.

### S7. `R1` makes `CLAUDE.md:10` false, and nobody owns that edit.

- `CLAUDE.md:10` says the image has *"stock git … no git config at any level — asserted at build time"*.
- `R1`'s write-set includes `dev-workflow.md`, but `CLAUDE.md` belongs to L0.
- `D0`'s new row `wf-runner-faithful-posture` is not discharged by `R1` or by `T10`.

**Fix:** an L0 edit after M-R. Either `R1` discharges that row, or the row is not created, since `R1` builds the thing now.

### S8. `pinning.md` §6 is shared by two lanes.

- `T8`'s `kit --role hunt` reads §6 by its heading and exits 27 *"when the heading has moved"*.
- P1, `P-NC` and `P-Z` edit §6, and they reach the loop branch at M-P, after `T8`.

**Fix:** declare the heading of §6 frozen across the build, or have `T8`'s arm read the heading as it stands after M-P0. Name §6 among §1's declared overlaps.

### S9. `T8` must change one assertion of the fence that the plan does not name.

- `launched_under` (`stabilize_harness_fence.rs:5752-5760`) asserts that a chain's prompt is either `crossModelPrompt` or `unitPrompt`.
- A `huntPrompt` for a `CHAINS['route-hunt']` step breaks that assertion.
- Even if it did not, the fence would credit the hunt step to `unitPrompt`, and then fail it on the kit's first-call rule.
- The `K1b` coupling itself is confirmed by reading:
  - `assert_eq!(kit_drives, drives)` (`:5821`);
  - `kit.is_subset(&names)` (`:5827`).

  So `KIT_DRIVES`/`KIT_ROLES` gain `hunt` in the same commit as `ROLES.hunt`. That is right as planned.

**Fix:** name the change to `launched_under` and its fixture in `T8`'s write-set and red-first list.

### S10. The loop branch's checkout is shared.

- L0's edits (`O-def`, `RB3`, `O5`, the rulings) and every T-task are made in the same checkout.
- The git agent's merges M-P0, M3 and M-P also land there.

**Fix:** a merge into the loop branch, or an L0 edit, starts only when no T-task is uncommitted. Say so in §1.

### S11. The forks gate early tasks, and the plan does not say when each is asked.

| Fork | Gates |
|---|---|
| 7 | R1 |
| 1 and 2 | P1 |
| 4 | RB1 |
| 3 | C16 |

**Fix:** ask them in that order, one at a time, right after `D0`.

---

## Notes

- **N1.** §0's *"about 154 … inside `READINGS`"* includes the 9 site-level reasons; cause-level is about 145. *"7748 and 7754"*: only 7754 is flagged at HEAD.
- **N2. Decision (c) is no conflict, and it is not decided before the opening.**
  - Ruling 1 (*"1.0.0 in this run"*) names what the run prepares.
  - `CLAUDE.md:7` says the human's *call* to release has not been taken, and ruling 3 puts that call at the close.
  - `run-set` holds no target version: its fact is `previous` (`--help`, line 784).
  - The *"release being prepared"* is one of the opening's own decisions (`stabilization-workflow.md:92`), gathered by the fact sheet.
- **N3. Part 4 of the staged canary ran.** The fifth root, part 4: see `DECISIONS.md:1734` and `:1744` (*"Part 4 of the fifth root ran as written"*). No entry reads its outcome whole. `K1b` was already built under the order of 2026-10-27, so the condition of 2026-10-12 has been treated as met. Record that in `D0`.
- **N4. `T4` has no use in this plan.** The census descending into `items` was review S5's fix for a `site` added to `FINDING`. This plan adds none: `site` travels in the triage writer's file. It is harmless, but optional.
- **N5. The harness's size.** The generated harness is 389,986 bytes against the 458,752-byte bound. `T8`, `K7b`, `K6` and `RB2` add to it before `T12` deletes the old path. The bound is held by `dev_stabilize_harness`. Watch it, and say so in `T8`.
- **N6. `O6` is settled.** `CLAUDE.md:27`, with the `tooling.md` rows for the route catalog and the site list, covers it. Adding the two sentences would restate those rows. Record that and close it.
- **N7. R1's evidence.** Its static arms only check the Dockerfile's text; the real evidence is the ignored docker arm. Before the image asserts *"exactly `safe.directory=*`"*, observe the runner's system config once: fork 7's CI arm printing `git config --system --list --show-origin`. What runner-images configures beyond that line is not established.
- **N8. P1's `ruled_reasons.tsv` gives a reason a second home.** The site is in `REASONS` and in the TSV. Prefer a `ruled:` field (the `DECISIONS.md` heading) on `Reason`, beside the reason, held to the log the way `THE_RULING_ON_FAMILIES` is.
- **N9. `T12` takes `VERIFY_SCHEMA` with it.** Its only user is `triagePasses` (`stabilize.js:4223`), the old loop that `T12` deletes, while `runFix` is pointed at the new functions. That is why fork 6 is moot.

---

## The forks, one line each

1. **How a reason is ruled.** Settled by ruling 3: **A**. Build it as a field on `Reason`, not a second file (N8).
2. **When the fence is tightened.** Not a design fork: the end state is the same either way. It is a question of how the work is paced. Engineer's call: **A**, because it avoids reopening finished families. If per-task halts would leave the critical path waiting on the human, let unruled reasons be named and refused only at `P-Z`.
3. **What answers a dynamic site that mints several codes.** Settled by the catalog's recorded rule: **A**, a case per code. The rule is in `MINTED_AT`'s why-texts (*"to get a case a code it mints before the port"*) and the `IN_PART` claims. The count sizes the work and decides nothing. `Finding`'s constructors are excluded (B1).
4. **Which binary is the base. Genuine, but not as A against B.** B is not a reading anyone could mean: nothing can be green on the binary under test. Proposed reading, which matches ruling 4's *"a regression the run's own fixes introduced"*:
   - in round *n*'s `test`, the base is the candidate that round *n*−1 started from, the one its landed fixes were made on;
   - in the audit of a fix stage, the base is the round's own candidate;
   - round 1 has none.

   **What is left for the human:**
   - (i) **Robust: every earlier candidate of the run, newest first; green on any of them is a regression.** This catches a break that showed up one round late, and the `false` value of a re-broken fix. It costs at most two more builds under the three-round bound, and builds are held by commit and reused.
   - (ii) Only the previous candidate.

   Put it together with B2's fact: whether `RB2` is built now or with the `fix` half's repair.
5. **Where the hunt count is shown.** Settled by the scale ruling of 2026-10-07: **B**, with a row due at the first stop. The count decides nothing, because `T2`'s partition already makes a partial hunt impossible to hide.
6. **The mixed-type schema enum.** Moot. `T12` deletes `VERIFY_SCHEMA` (N9), and on the path with a queue the regression and base facts are filed, not returned. `RB2` should not touch `VERDICT_SCHEMA` (B2).
7. **How faithful the runner image is.** Settled by ruling 6: **A**, git from the PPA plus a CI arm that watches for drift. Tell the human that the drift arm turns CI red when GitHub changes the runner image, which is its purpose.

**Decisions after `J2`:**
- **(a)** Genuine, and the human's: the decision of 2026-10-27, item 4.
- **(b)** Settled: one `.gitleaks.toml` entry, no rewrite (S6).
- **(c)** Not a fork before the opening (N2).

## Tools

- **The join of the scheduled list to the site list**, which answers *how many scheduled sites each family and file has*. Run from `crates/cli/tests/route_catalog/`; it writes `siteids.tsv` and `sched.txt` to the scratch directory, then prints the counts:

  ```sh
  grep -v '^#' sites.tsv | awk -F'\t' '{print $2"::"$3"::"$4"#"$5"\t"$1"\t"$2}' > siteids.tsv
  grep -v '^#' scheduled.tsv | cut -f1 > sched.txt
  awk -F'\t' 'NR==FNR{s[$1]=1;next} ($1 in s){print $2" | "$3}' sched.txt siteids.tsv | sort | uniq -c
  ```

  This is a candidate for the projection (`P0`), which would answer it directly.
- **Scanning the range a PR to `main` would carry with gitleaks:** `gitleaks git . --config .gitleaks.toml --redact --log-opts "origin/main..HEAD" --report-format json --report-path <f> --exit-code 7`.
- **A tool that is missing:** one that lists the jobs of a branch's recent CI runs that did not succeed, which would make a flaky leg like B4's visible. I did it with a loop over `gh run view`, and it is a candidate for `dev/`.
