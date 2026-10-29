# Build plan v2: everything owed before the first stabilization run opens

Read-only planning. Based on `fix/rc24-tier1` @ `7c34a222`, clean, CI run `40767224323` green. It revises `pre-opening-plan.md` (v1, built on `0bd8a5b5`) using:
- its review (`pre-opening-plan-review.md`: 5 blocking, 11 should-fix, 9 notes);
- the review of `f99658b8` (`pre-opening-1-review.md`: H1, N1 to N4);
- the human's rulings of 2026-10-27 and 2026-10-28.

Every count below was taken at `7c34a222` unless it says otherwise.

## 0. What changed since v1

### 0.1 Facts that moved

- **The link-on-the-way fix is done** (ruling B, 2026-10-28). It is in `63ce9ca5` through `a383608d`, merged at `7c34a222`. It took four review cycles: the human allowed a fourth beyond the bound of three (ruling A, 2026-10-28).
- **Lane R is mostly done** in `22f06d31`:
  - the image installs git from the git-core PPA;
  - it carries the runner's `[safe] directory = *`;
  - its build-time assertion checks the new shape;
  - its header prints the git that ran;
  - `dev-workflow.md` → Gate and the `tooling.md` row describe it;
  - the `CLAUDE.md` sentence was corrected by the main session in `63ce9ca5`.
  
  Two things are left: the tool's own test suite (its `tooling.md` row says *"no suite of its own"*) and the CI check for drift in the runner's git config (fork 7, A). These are U1 and U2.
- **CI's Linux red is fixed.** v1's B4 ("a byte-table cell is nondeterministic") was not flakiness. git 2.56 refuses a `git mv` past a link, and git 2.55 and older follow the link. That was a product finding, and the guard fixed it. CI is green at HEAD.
- **`test (g_migration)` takes 8 min 24 s on CI at HEAD**, against `timeout-minutes: 10` (job 20:56:08–21:04:32). Its split is owed before the first C-task (U0), not when its proxy passes 5 minutes as v1 said.
- **`Why` has four words**, since `NotBuilt` was added on 2026-10-28. v1 said three.
- **The route catalog at HEAD:**
  - 145 scheduled rows: 144 classed `unchanged`, plus one `not-counted` row, `crates/engine/src/finding.rs::graded::dynamic#1`. v1 named the wrong row.
  - 125 static sites and 20 dynamic sites. Three of the dynamic sites are the bodies of `Finding`'s own constructors (`finding.rs::block`, `::blocking`, `::graded`).
  - 9 site-level `Reason {` entries: config 1, setup 4, task_finalize 2, unassigned 2.
  - 157 `NotConstructible(` entries (9 of them site-level), 7 `NotBuilt(`, 2 `Platform(`.
  - `MINTED_AT` lists 73 codes over 11 dynamic sites: `compose.rs::blocking_workflow_refs` 27, `write.rs::blocking_write` 10, `migrate_corpus` 10, `store.rs::block` 7, five sites with 3 each, and two with 2 each. Codes with uncatalogued prefixes come on top (for example `rollback`'s `ConflictDoor`, and `cascade` ×2).
- **The record script already has a round fact named `base`**: *"the commit the round's change is measured from"* (`dev/stabilize-record`, `ROUND_FACTS`, line 2527; its help text at line 718). v1's RB1 `base` field on a verdict would give one word two meanings, so RB1 names its field `earlier`.
- **The invoked harness is 389,986 bytes**, against the 458,752-byte bound.
- **Two scratch scripts still exist** in the session scratchpad: `record-print.py` (83 lines) and `rawwrites.py` (27 lines). U4 and U5 read them for reference only and rebuild the tools; nothing is copied. A scratch path is a host path, so none appears in committed text.
- **Eight new keys for the opening ledger** were recorded on 2026-10-28: `byte-table-findings.md` §30 (four) and `route-catalog-findings.md` §12 (four).
  - The opening ledger takes them all. Only one is owed a task before the opening: `r1-catalog-link-acts-never-check-the-route-asks-for-them`. It is a defect in the catalog's frame of the "records falsely" kind (a case can stay green while following a route the product no longer prints), so it becomes P-F1.
  - The extent of `r1-config-set-writes-through-a-link-at-the-manifests-own-place` (*"unbounded until …"*) is measured by U5's census. The key's fix is the run's.

### 0.2 Each review finding and what this plan does with it

| Finding | What v2 does with it |
|---|---|
| **B1** — wrong `not-counted` row; `C17` promises cases for `Finding`'s constructors | §0.1 corrected. **P1, commit 1:** the site scan leaves out the bodies of `Finding`'s own constructors. The list is regenerated: 145 → 142 scheduled rows, and the `not-counted` row goes. This is settled as an engineering call: a constructor body is not a refusal site, so listing it makes the list wrong. C17 is re-cut. |
| **B2** — the regression base is aimed at a path that cannot run | Ruling (c): only RB1 (the record script's half) is built now. The harness half, its round-2 simulation scenarios, the derivation for the fix-stage audit and the verifier's definition step all move under a new pending row, `wf-regression-base`, which comes due with the `fix` half's repair. Nothing touches `VERDICT_SCHEMA` or `NEEDS`. D0 records the fact the review says the human must be told: ruling (c) has no effect in round 1, and nothing in the harness acts on it until that repair. T10 does not discharge the row. |
| **B3** — `T2`'s partition contradicts `T8`'s parts | Parts become data. P0's projection divides each family into parts deterministically: by file, at most 12 sites each, with a file over 12 split in id order. T1's item cell names `family` and optionally `part`. T2 holds the partition to the item's own set of sites. T8 names the part. |
| **B4** — CI red at the base | Overtaken: it was git 2.56, fixed by the guard, and CI is green at HEAD. The faithful image is done. No task. |
| **B5** — J2 cannot meet the condition | J2 is limited to the `test` stage's harness, record script, step tool, definitions, simulation and fences. It is split into five areas plus one pass across all of them. The diff of its fixes gets a scoped re-review (J2r). Anything the canary decision changes in a harness, script or definition goes through a scoped re-review (J3) before the opening. |
| **S1** — size understated | Re-priced in §4 from the pilot's wall-time ratio and from code counts. Review-and-fix gates are included. PC counts the dynamic codes before the C-tasks start, and the plan is re-priced at the end of Phase 1. The named-binary arm runs filtered to the family in each C-task (P0 adds the filter) and runs whole once, at P-Z. |
| **S2** — `g_migration` passes CI's bound | It is already at 8.4 of 10 minutes. U0 splits it before C1. Each C-task records the runner-faithful wall time of its part. |
| **S3** — P cannot reach R's faithful tool | Mostly moot: R is merged into the loop branch and P forks after it. One merge is still needed: loop → P after M-U0, before C1 (§1). |
| **S4** — `P-NC` unbounded and late | Split. P-F0 classifies the existing texts and builds the shared frame pieces before C1, with a stop rule per piece. P-NC2 sweeps only the new texts, at the end. The `setup.rs:3990` claim is corrected: only the "no git on `PATH`" site needs the per-command environment. |
| **S5** — the *Tools* channel is missing for triage and hunt | T0 adds an optional `tools` text to the triage writer. T2 adds one to the hunt writer. Each is scanned for hygiene, rendered as the report's last section, and held by an arm. |
| **S6** — gitleaks: false reason, rewrite not viable | Put to the human as H2. Recommendation A: one `.gitleaks.toml` entry keyed to `DECISIONS.md` and the line shape `"key": "<64-hex>"`, with no rewrite. Built as U7 once answered. |
| **S7** — the `CLAUDE.md` runner sentence has no owner | Done in `63ce9ca5`. The row `wf-runner-faithful-posture` is not created. |
| **S8** — `pinning.md` §6 shared by two lanes | Declared overlap: §6's heading is frozen for the whole build. P edits its body only. T8's kit arm reads the heading. |
| **S9** — `T8` must change `launched_under` | Named in T8's write-set and red-first list. |
| **S10** — the loop checkout is shared | T gets a worktree of its own. The loop checkout takes only merges, Phase 0, the join, and main-session edits, and those start only when no lane writer is mid-task there. |
| **S11** — when each fork is asked | Forks 1–7 are now settled (§5). The remaining human questions are placed in order in §3. |
| N1 (the §0 counts) | Corrected in §0.1. |
| N2 (the version is no conflict) | Put to the human as H4, at the opening, after the fact sheet, because the release being prepared is one of the opening's own decisions. Recommended reading: A. |
| N3 (part 4 of the staged canary ran) | D0 records it. |
| N4 (`T4` has no use here) | `T4` is dropped. |
| N5 (the harness's size) | Every harness task prints the generated size. A task that would take it past 440,000 bytes stops, so the cut can be planned. |
| N6 (`O6` is settled) | D0 closes it, with the reason. |
| N7 (R's evidence) | U2's check prints the runner's system config on every CI run. |
| N8 (ruled reasons as a second file) | Fork 1 A, built as a `ruled:` field on `Reason`. No second file. |
| N9 (`T12` deletes `VERIFY_SCHEMA`) | Fork 6 is moot. D0 records why. |
| pre-1 **H1** — the wrong reasoning sentences | D0 adds a bracketed correction to the 2026-10-27 *not comparable* entry: *"inside by construction"* holds for round 1 only. What the old reasoning missed is handled by ruling (c). |
| pre-1 **N1, N2** — `stabilization-workflow.md` lines 706, 140 and 275 | D0 fixes them now. They are read by every agent in the meantime, so they are not left for T10. |
| pre-1 **N3** — the mixed-type enum | Moot: `T12` deletes `VERIFY_SCHEMA`. Recorded in D0. |
| pre-1 **N4** — "door" in the verifier's step 4 | Main-session edit in the definition batch: *"the door, or a step of the block, the previous release does not have"*. |

### 0.3 The human's rulings, folded in (D0 records every one)

| Ruling | Where it lands in the plan |
|---|---|
| (a) The catalog first: finished now, its fence tightened, its frame extended | Lane P is the critical path. P1 tightens the fence. P-F0 and P-F1 extend the frame. `wf-route-catalog-rest` is due before the opening. Red cases go to the opening ledger by key, never fixed in P. |
| (b) The hunter stays on top of the catalog, shape A | P0 builds the projection. T1 and T2: the record script holds the rows and computes the count. Each plantable cause the hunter finds becomes a catalog case: the `build-fixer` definition says so, and the `fix` half's repair holds it (new row `wf-hunt-cause-to-case`). |
| (c) The regression base: every earlier candidate of the run, newest first | RB1 now (the field `earlier`). The rest goes under `wf-regression-base`, due with the `fix` half's repair. |
| (d) The link-on-the-way fix (B) | Done at `7c34a222`. Not a task. |
| (e) A fourth review cycle was allowed | Recorded as a data point for the workflow's bound of three, put to the human at the first stop. It also sets the plan's conditional stop (§3). |

## 1. Lanes, branches, merges

| Lane | Branch / checkout | Owns |
|---|---|---|
| **Phase 0 / join** | the loop checkout `fix/rc24-tier1` | `DECISIONS.md` and `decisions-pending.md` entries for rulings; `stabilization-workflow.md` (D0, T10); `tooling.md` → Candidates (D0) |
| **U** — tools | `work/rc24-tools` (worktree) | `.github/workflows/ci.yml`, `tooling-tests/ci_matrix_fence.rs`, `dev/runner-faithful` plus its new suite, `dev/jigc-rig` plus its arms, the new `dev/byte-table-record`, the new `dev/ci-runs`, the raw-write census, `.gitleaks.toml`, each with its `tooling.md` row |
| **K** — trial image | `fix/rc24-trial-image-layer` | `completions/trial-harness/` |
| **P** — route catalog | `fix/rc24-route-catalog-rest` | `crates/cli/tests/route_catalog*`, `support/route_case.rs`, `support/refusal_sites.rs`, `refusal_site_list.rs`, the projection, `pinning.md` §6 body, `route-catalog-findings.md`, `route-catalog-pilot.md` |
| **T** — hunter and test stage | `fix/rc24-route-hunter` | the record script, the step tool, the harness (source and generated file), the simulation and its runtime fixture, the harness fence, the canary tool; the main session's definition edits for this lane are made here, in lane order |

**Rules:**
- **The main session only orchestrates.** Its own edits are limited to `.claude/agents/` and `CLAUDE.md`, each behind its own gate.
- **Who builds:** builders are Opus `build-executor`, fixers are Opus `build-fixer`, each with a self-check. Every task is followed by a read-only review and its fixers, at most three cycles. A fourth cycle is a human stop (§3).
- **Git steps** (worktrees, merges with `--no-ff`, pushes by name) are run by Sonnet `build-git`.
- **One full `dev/gate` at a time** on the machine, before every commit. Worktrees use `--private-target`. `dev/runner-faithful` runs and docker builds are queued the same way.
- **A review can overlap the next task in its lane.** The reviewer reads only commits. That review's fixes are then the lane's next writer, after the task in flight has committed.
- **Push** every lane's branch at the end of each phase, and read its CI (with `dev/ci-runs` once U6 lands).
- **Nothing touches `main`.**
- **Declared overlaps:**
  - `DECISIONS.md` is resolved by `dev/merge-logs` at each merge.
  - `tooling.md` and `tooling-tests/groups/g_tooling.rs` get different rows in U, P and T, and are resolved by hand.
  - The heading of `pinning.md` §6 is frozen.
  - The generated harness conflicts are resolved in the source, then regenerated.

**Merges** (all `--no-ff`, by `build-git`):
1. **Fork** P, T, U and K from Phase 0's commit.
2. **M-U0:** U → loop, after U0. Then **loop → P**, before C1, so P's pushes run under the split CI legs.
3. **M-P0:** P → loop, after P1. Then **loop → T**, before T1.
4. **M3:** K → T, before K7b.
5. **M-U:** U → loop, after U7.
6. **M-T:** T → loop, after O5.
7. **M-P:** P → loop, after P-Z. Then the join runs on the loop checkout.

## 2. Tasks, in order

The **gates** line of each task counts the gates it needs: its build commit(s), plus the fix gates its review is expected to need.

### Phase 0 — loop checkout, serial; push at its end

**D0 — Record the rulings and the corrections.** Builder (a document task). The brief carries the rulings word for word.
- **Writes:**
  - one `DECISIONS.md` entry with the rulings (a)–(e) of 0.3, including B2's fact;
  - the bracketed H1 correction on the 2026-10-27 *not comparable* entry;
  - N3 (part 4 ran; the 2026-10-12 condition has been treated as met since `K1b`);
  - O6 closed (review N6);
  - forks 1, 3, 5 and 7 settled, and 6 moot (review N9; pre-1 N3);
  - `decisions-pending.md`:
    - `wf-route-catalog-rest`: due **before the opening**; what it finds goes to the opening ledger;
    - `wf-route-walk`: annotated with (b), shape A;
    - new **`wf-regression-base`**: RB2, RB3, the round-2 scenarios and the fix-audit derivation; due with the `fix` half's repair;
    - new **`wf-hunt-cause-to-case`**: due with the `fix` half's repair;
    - new **`wf-hunt-count-view`**: fork 5 B; due at the first stop;
    - `wf-test-review-real-run`: annotated with J2's scope (B5);
    - `wf-whole-canary-after-parts`: annotated with N3 and its place in this order;
    - the first-stop row: annotated with (e), the fourth cycle as a data point;
  - `stabilization-workflow.md` lines 140, 275 and 706 (pre-1 N1, N2);
  - `tooling.md` → Candidates, with the triage in the table below.
- **Done when:** `doc_link_fence` and `tooling_index` are green, and a full gate is green.
- **Gates:** 1 + 1.

| Candidate | Triage |
|---|---|
| (i) `dev/jigc-rig` plants a link at a named place (asked for by five agents) | Build: U3 |
| (ii) A byte-table record tool: split a print, recount, diff cells against a base with the timing columns blanked | Build: U4 |
| (iii) A census of raw writes that bypass the shared write primitives | Build: U5 (form: fork F2) |
| (iv) A CI-job git-version compare | Build: U6, merged with the review's "list a branch's failing CI jobs" |
| The review's join of `scheduled.tsv` to `sites.tsv` | Declined: P0's projection answers it |
| The review's gitleaks scan over a PR range | Declined as a tool: `dev/stabilize-record vet --range` and CI's `hygiene` job already scan ranges |

**D0-def — Main-session edit to `stabilize-opening.md`.** The sheet's section 0 (what is owed before the opening) gains **the tasks held back in build plans**: every task a plan under `completions/artifacts/` holds back or leaves unbuilt, with its plan, its id and its trigger. Neither the *Never* paragraph nor the *Tools* paragraph is touched.
- **Gates:** 1.

**PC — Count the codes each dynamic site mints.** A read-only agent; no gate. For each of the 17 dynamic sites left after B1: every code it mints, under catalogued and uncatalogued prefixes alike, each with its source line.
- **Output:** the C-dynamic cut (below) and the re-price at the end of Phase 1.

### Lane U — tools, serial

**U0 — Split `g_migration` on CI.**
- **Writes:** `ci.yml`, plus rows in `ci_matrix_fence.rs`.
- **What:** split into parts: `route_catalog`, `byte_table`, and the rest, which skips exactly those two. Each part's `minutes` is max(10, 2 × its proxy), measured in `dev/runner-faithful` on four CPUs. This follows the precedent of `72ca156a`.
- **Red first:** the fence's arm for the new rows.
- **Gates:** 1 + 1.
- **Then:** M-U0.

**U1 — `dev/runner-faithful` gets its own suite, and a `--posture` print.**
- **Writes:** the new `tooling-tests/dev_runner_faithful.rs` in `g_tooling`; the *Held by* cell of its `tooling.md` row.
- **Static arms, red first:**
  - the image carries the runner's system config and no other;
  - the posture refuses an image without the runner's `safe.directory`;
  - a usage error exits 2 before docker is asked;
  - `--help` prints the header;
  - `--posture` prints the declared posture.
- **Ignored docker arm:** `0bd8a5b5` is red on CI's two cells in the image, and its child is green. This makes the 2026-10-28 evidence the tool's own. It runs once, with its lines in the commit.
- **Gates:** 1 + 1.

**U2 — A CI check for runner drift** (fork 7 A).
- **What:** one step that prints `git --version` and `git config --system --list --show-origin`, and holds the config to `dev/runner-faithful --posture`. The posture has one home, the tool. The git version is printed and not held: the PPA's newest moves every week.
- **Red first:** the fence's arm.
- **Gates:** 1 + 1.

**U3 — `dev/jigc-rig` plants a link.**
- **What:** a link at a named place of the corpus, aimed at one of the guard's target classes:
  - `inside:<path>` — a real directory in the repository;
  - `outside` — a directory under the rig's own `mktemp` root, never anywhere else;
  - `nothing` — a dangling link;
  - `git` — into `.git`.
  
  These are the classes the route chooses by (2026-10-28, S6).
- **Arms** (in `dev_rig_parity.rs`, or a new suite it names):
  - each class is planted and read back;
  - a place outside the corpus is refused, exit 2, before anything is written;
  - an outside target lies under the rig's root;
  - a planted `.jigc/index` link draws `store.link-on-the-way`.
- **Writes also:** the call cell of its `tooling.md` row.
- **Gates:** 1 + 1.

**U4 — `dev/byte-table-record`.**
- **What:**
  - `split <print> <dir>` writes the record's four files;
  - `counts` prints every count the record keeps;
  - `diff --base <rev>` diffs cells against a base revision with the timing columns blanked.
- **Suite:** `tooling-tests/dev_byte_table_record.rs`, with a fixture print. Under fork F3 A, the suite also holds the committed record's files equal to the tool's split of the committed print.
- **Its row** is owned by `pinning.md` §7.
- **Gates:** 1 + 1.

**U5 — The raw-write census** (fork F2 A).
- **What:** a generated, committed list of raw write sites, built from the site scan's own reader of production source (so there is one notion of "test code"). Each id is shaped like a site id (`file::function::pattern#n`) so it does not churn with line moves. It is unclassified. `UPDATE_GOLDENS=1` regenerates it, and a test in `g_finalize` holds it equal to the scan.
- **Red first:** a fixture source with each pattern, a `#[cfg(test)]` module, a comment and a string literal.
- **The commit records** the count at HEAD (the second re-review counted 213), and the census's row.
- **Gates:** 1 + 1.

**U6 — `dev/ci-runs`.**
- **What:** for a branch's last N runs, per job: conclusion, minutes against the job's bound, the runner image, and the git version read from the job's log. `--compare A B` names the jobs that differ in conclusion or git.
- **Suite:** a stub `gh` on `PATH` serving fixtures; no network.
- **Rules:** it never prints an environment value.
- **Gates:** 1 + 1.

**U7 — The gitleaks entry** (after H2, if A).
- **What:** one `.gitleaks.toml` entry, exact file and exact line shape.
- **Done when:** `gitleaks git --log-opts "origin/main..HEAD"` exits 0, and the entry's own line in `.gitleaks.toml` says what it covers.
- **Gates:** 1.
- **Then:** M-U.

### Lane K

**K7 — The trial image's base layer**, as specified in the reshape plan §5.
- **Done when:** its two red-first checks hold, and the image's seven checks plus the pair check still hold.
- **Queue:** its two docker builds (about 25 minutes each) are queued like gates.
- **Gates:** 1 + 1.
- **Then:** M3, K → T.

### Lane P — route catalog, serial (fork F1 A)

The lane writes no product source, and a red case is listed with a key, never fixed here.

**P0 — The projection.**
- **What:** one generated, committed file, held equal by the suite. It holds:
  - a vocabulary header: `Classification`'s words and `Why`'s four, read from their `Display`;
  - per family: its doors (`FAMILIES`), its **parts** (B3), and every site, with claimed, `MINTED_AT` and `IN_PART` dynamic sites in the family's rows;
  - per site, its answer: each case with its cause; or the reason (word, text, `ruled` once P1 lands); or `scheduled`;
  - `without_case` and `not_followed`, with their `Why`;
  - the residue.
- **Also:** the named-binary arm takes `JIGC_ROUTE_CATALOG_FAMILY`.
- **Red first:**
  - the committed projection is the catalog's answers;
  - a claimed dynamic site is in its family's rows;
  - the vocabulary header is the frame's;
  - a family's parts are its sites by file, none over twelve;
  - the named arm runs one family when one is named;
  - mutant: a case dropped from a table makes the file differ.
- **Gates:** 1 + 1.

**P1 — B1, then the fence.** Two commits.
- **Commit 1:** the scan leaves out the bodies of `Finding`'s constructors. Red first: `a_constructor_of_finding_is_no_refusal_site`. The class is counted (every `Self { … }` of a `Finding` constructor or builder). `sites.tsv` and `scheduled.tsv` are regenerated: 142 rows.
- **Commit 2:** `Reason` gains `ruled: Option<heading>`.
  - A heading missing from `DECISIONS.md` is named, red.
  - `ruled: None` is named as owed a ruling, but not refused until P-Z (fork F4 A).
  - The `not-counted` class is refused.
  - `pinning.md` §6's body is updated.
- **Red first:**
  - `a_reason_nobody_ruled_is_named_as_owed`;
  - `a_ruled_reasons_heading_that_is_gone_is_named`;
  - `a_not_counted_row_is_refused`;
  - one mutant.
- **Then:** M-P0. **H1** follows P1's review. The ruling is recorded in P-F0's first commit.
- **Gates:** 2 + 1.

**P-F0 — Classify the reason texts, and build the shared frame pieces.**
- **What:** read every `NotConstructible` and `NotBuilt` text in the projection (about 164). Classify each as:
  - plantable with frame piece X;
  - not constructible, with its reason standing;
  - not built: plantable now, the case simply not written.
  
  Commit the classification table. Build each shared piece red first, together with the case that needs it. The first piece is the per-command environment for "no git on `PATH`" at `setup` (only that site).
- **Stop rule:** a piece over about 3 hours of wall time, or one that needs product source, halts to the main session.
- **Gates:** 2–4 + 1.

**P-F1 — The catalog's link acts check that the route asks for them.**
- **What:** `ReplaceWithACopy`, `MoveDirectoryIn`, `ReplaceWithDirectory` and `PutTheFileItself` read their act from the printed route by span, as an operand is read, and are refused where the route does not ask for that act.
- **Red first:** the §12 repro (changing `WayOut`'s choice) turns a case red. Every link-act case is re-run, and every classification that moves is listed.
- **The key** is struck in §12, with its commit.
- **Gates:** 1 + 1.
- **Then:** loop → P (U0's split).

**C1–C15 — The static families**, cut as in v1, 125 sites. **C16a, C16b, C17, C18 — the dynamic families**, re-cut from PC's count: a case for every code (fork 3 A); no task over about 35 codes; `compose`'s 27 a task of its own.
- **Writes:** that family's file, `scheduled.tsv` rows, `EXPECTED_RED`, `CATALOGUED`, `CLAIMS` and `MINTED_AT` as needed, the projection regenerated, keys minted in `route-catalog-findings.md`, one `DECISIONS.md` entry with the 2026-10-12 table.
- **Commits:** about 10 sites or codes each, each behind its own gate.
- **Red first:** every case is run and seen in its classification before it is listed. Any frame vocabulary a case needs is added red first.
- **Done when:**
  - the family's rows are gone from `scheduled.tsv`;
  - the strict expected-red list holds;
  - the projection is regenerated;
  - one `dev/runner-faithful` run of the `route_catalog` part is green, with its line and wall seconds in the commit;
  - the named-binary arm, filtered to the family, has run on rc.24, with its table in the record;
  - the suite's seconds are in the commit;
  - each new site-level reason carries `ruled: None` and is named in the task's return.
- **Stop rules:**
  - a site over about an hour stops the task;
  - a frame piece P-F0 did not build is added red first within P-F0's stop rule, or the task halts.
- **Gates:** 22–26 commits + 10–19 fix gates; about 22 Linux runs.

**P-NC2 — Sweep the new texts.** Only the texts the C-tasks wrote, by P-F0's rule.
- **Gates:** 1 + 1.

**P-Z — Close the catalog.**
- **What:** `Scheduled`, `SCHEDULED`, `THE_COUNT` and its arm, and `scheduled.tsv` are removed. The fence answers a site only with a case or a ruled reason: `ruled: None` is refused, so every reason batch has to be in by then. `route-catalog-pilot.md` gets §14's numbers. The named-binary arm runs whole on rc.24. The bounds of the `g_migration` parts are measured again.
- **Red first:** `the_fence_answers_a_site_only_with_a_case_or_a_ruled_reason`.
- **Gates:** 1 + 1.
- **Then:** M-P.

### Lane T — serial, in its own worktree

**T0 — The *Tools* channel.**
- **What:** `proposalPrompt` and `crossModelPrompt` carry the *Tools* paragraph, with repository-relative links. The triage writer takes an optional `tools` text, scanned for hygiene and rendered as the report's last section (S5).
- **Red first:** the prompt arm, plus `a_triage_tools_text_is_its_reports_last_section`.
- **Gates:** 1 + 1.

**RB1 — The regression base, the record script's half.** Additive: a run without the new field answers exactly as before.
- **What:**
  - A filing beside `confirmed` may carry **`earlier`**: an array, **newest first**, with one entry per earlier candidate of the run, each `{round, binary sha256, fact: true | false | not-comparable}`.
  - The script derives the list from the round records (the `candidate` and `binary` of each round before N). Each entry's hash must equal that round's recorded `binary`.
  - The list must be complete for the fact to be known. A partial list makes the fact unknown, and `regression_unknown` names it.
  - Grade `regression` = the previous release `true`, or any earlier candidate `true`. `route_of` sends such a finding to a fixer.
  - The view and the ledger get a cell for it.
- **Red first:**
  - `a_finding_green_on_an_earlier_candidate_is_a_regression_of_the_run`;
  - `a_not_comparable_finding_green_earlier_is_a_regression`;
  - `an_earlier_list_out_of_order_or_with_a_wrong_hash_is_refused`;
  - `a_partial_earlier_list_is_unknown`;
  - `round_one_has_no_earlier_candidate`;
  - `a_run_without_the_field_writes_the_bytes_it_wrote`;
  - three mutants.
- **Fixtures:** multi-round records written through the script's own writers (`wf-writers`). If the state refuses them, the builder halts and reports; no back door.
- **Gates:** 1 + 1.

*Waits for M-P0 → loop → T.*

**T1 — An item names `family` and optionally `part`**, held to the projection read at the round's candidate commit.
- **Red first:** v1's four arms, plus `a_part_the_projection_lacks_is_refused`.
- **Gates:** 1 + 1.

**T2 — The hunt writer, `hunt --from FILE`.**
- **What:**
  - One row per site of the item's own set of sites.
  - Each row carries `cause` and `plantable`.
  - Words are held to the projection's vocabulary header.
  - The script computes the count.
  - It takes an optional `tools` text.
  - The step tool's vet learns the filed class.
- **Red first:** route-hunter plan H3's seven arms, pointed at the projection, plus:
  - a claimed dynamic site is a row a hunt owes;
  - a part's hunt owes its part's sites and no other's.
- **Done when:** four mutants are red.
- **Gates:** 1–2 + 1.

**Definition edits — main session, in T's worktree.** Each edit behind its own gate, then reviewed as one batch.
- `O4`: the tester and the reviewer — what a finding carries, the kit sentence, and the hunt contract bound to a `HUNT:` line (E9).
- `finding-triage.md`: a finding that names a site carries `site`.
- `stabilize-opening.md`: one `route-hunt` item per family part, from the projection.
- `build-fixer.md`: a plantable cause a hunt found gets its catalog case in the fix, red first; a new blocking site comes with its case.
- `finding-verifier.md` step 4: pre-1 N4's clause.
- **Gates:** 5 + 0–1.

**T8 — The route hunter, as an instrument.**
- **What:**
  - `ROLES.hunt`, together with the kit's `hunt` in the **first** commit (the `K1b` fence);
  - `CHAINS['route-hunt']`, `HUNT_SCHEMA`, `NEEDS.hunt`;
  - `huntPrompt`: the projection at the candidate's commit, the family and part, the catalog's cases (not hunted again), the not-constructible causes to try by hand, Bash timeouts, T2's call, and "say whether each cause can be planted".
- **The fence:** `launched_under` and its fixture change (S9).
- **Size:** the generated size is printed. Over 440 KB, the task stops.
- **Red first:** route-hunter plan H4's five simulation arms; `kit --role hunt` prints §6 by its heading and exits 27 when the heading has moved; `the_files_the_hunters_prompt_names_exist`.
- **Gates:** 2 + 1.

*Waits for M3.*

**K7b, then K6**, as in the reshape plan §5. Each prints the harness's size.
- **Gates:** each 1–2 + 1.

**T9 — The rest of it:** `its_doors_are_the_review_rows_the_hunts_and_the_arms` and `its_expectation_names_what_it_prices`. The other two of its arms are built.
- **Gates:** 1 + 1.

**T11 — Move the scenarios left on the old path.** The builder counts first how many of the 92 still open without an order, then moves about ten a commit.
- **Gates:** 3–6 + 1.

**T12 — Delete the old path**, as specified. `runFix` stops pointing at `VERIFY_PASSES`, and `VERIFY_SCHEMA` goes.
- **Gates:** 1–2 + 1.

**O5 — Main-session edits**, as the reshape plan specifies.
- **Gates:** 4.
- **Then:** M-T.

### The join — loop checkout, serial

**T10 — The documents.**
- **What:** the reshape plan's T10; the hunter's sections, with parts; the regression base as built (the record half only); pronoun lines.
- **Pending rows discharged:** `wf-route-walk`, `wf-route-catalog-rest`, and the reshape plan's set. **Not** `wf-regression-base`, `wf-hunt-cause-to-case` or `wf-hunt-count-view`.
- **Done when:** `doc_link_fence` is green.
- **Gates:** 1 + 1.

**J1** — as specified, plus any of T8's arm it did not absorb.
- **Gates:** 1 + 1.

**J2 — The `test` stage reviewed for a real run.**
- **What:** six read-only `milestone-code-reviewer`s in parallel, one per area — harness · record script · step tool · definitions · simulation and fences — plus one pass across all of them. Each reads the diff from `6e78975b` to HEAD over its area. Product tests are out of scope.
- **Fixes:** serial on the loop branch, then **J2r**, a scoped re-review of the fix diff.
- **Done when:** every finding is fixed or ruled, and `wf-test-review-real-run` is discharged.
- **Gates:** 4–8 + 0–3.

**H3, then the canary if ruled, then J3** (a scoped re-review of anything the canary changed).
- **Gates:** 0–4.

**The opening.**
1. `stabilize-opening` writes the fact sheet.
2. **H4** — the questions, one at a time.
3. `CLAUDE.md` is edited per the version answer (main session).
4. The opening commit, behind the gate, and the push.
- **Gates:** 2.

## 3. Human stops, in order (one at a time, each with its recommendation)

1. **H1** — after P1's review: the nine site-level reasons that already stand, as one batch.
   - *Why now:* they set the model for every C-task.
   - *Blocks:* nothing until P-Z.
2. **H2** — the gitleaks hit at `85661425`, `DECISIONS.md:99`.
   - **A (recommended):** one `.gitleaks.toml` entry keyed to `DECISIONS.md` and the `"key": "<64-hex>"` line shape.
   - **B:** a `.gitleaksignore` fingerprint. It drifts, because `DECISIONS.md` is prepended.
   - **C:** a history rewrite. Not viable: it breaks 18 `commit:` pointers and every sha the records cite.
   - *Why before the opening:* only C has a deadline, and it is before the opening.
   - *Told with it:* U2's check turns CI red when GitHub changes the runner's git config, which is its purpose.
3. **H-r1…r3** — conditional: batches of new site-level reasons at the end of each part of Phase 2 (after C8, after C15, after the dynamic tasks). Each batch is pushed before it is put.
4. **H-cycle** — conditional: any task whose review reaches a fourth cycle. Precedent: 2026-10-28.
5. **H3** — `wf-whole-canary-after-parts`.
   - **A (recommended):** run what no part showed (staged-canary §11, the nine things), including the measured hunt at a small family. It costs about 190–330 minutes and $60–105 at the first canary's rates.
   - **B:** the measured hunt only.
   - **C:** none.
   - A hunt rate measured this way is a floor, not an estimate.
6. **H4** — at the opening, after the fact sheet, among the opening's questions: the release being prepared.
   - **A (recommended):** `1.0.0` is prepared (ruling 1), and the call to publish stays the human's, at the close (ruling 3). One clause in `CLAUDE.md:7` says so, so the sheet records no disagreement.
   - **B:** the next release candidate (rc) is prepared, and the 1.0.0 decision is left to a later run.

## 4. Price

| Lane | Tasks | Full gates | Other machine time | Wall time (continuous running) |
|---|---|---|---|---|
| Phase 0 | 3 | 2–3 | — | 4–6 h |
| U | 8 | 8–15 | 2 docker runs | 20–35 h, in parallel |
| K | 1 | 1–2 | 2 image builds × 25 min | 6–10 h, in parallel |
| P | 25 | 45–60 | about 22 Linux runs; rc.24 arms | **75–140 h** |
| T | 12 | 33–38 | — | 60–100 h, in parallel |
| Join and opening | 6 | 9–19 | canary 0–15 h | 30–65 h |
| **Total** | **55** | **about 100–135** | | |

**How the numbers were reached:**
- **Units of work:** about 220–260 for P (125 static sites, plus about 95–135 dynamic codes; PC fixes this number).
- **Per unit:** 14–22 minutes of wall time. This is measured: the 2026-10-12 families took about 14 minutes a site, and the pilot about 22 a site including its frame. Reviews and fixes add 25–40 % on top.
- **Gates:** 20–30 minutes each, so 35–65 hours of gate time, serialized on one machine.
- **Critical path:** Phase 0 → **P** → M-P → T10/J1 → J2/J2r → H3 → canary → J3 → opening. That is about 110–220 hours of running time.
- **The queue is the real limit:** while P and T run together, every gate waits for the one before it. T is within about 20 % of P.
- **Calendar:** at the pace the log shows (commits round the clock, about 16–20 hours a day), and with the human stops: **3–5 weeks.** About 2.5 weeks if every task lands in one review cycle and PC counts about 95 codes.
- **Re-pricing:** the plan is re-priced at the end of Phase 1, from PC and from the first two C-tasks.

## 5. Forks

**Already settled, not forks:**
- 1 → A, as a field (N8);
- 2 → tighten first (pacing in F4);
- 3 → A, a case per code;
- 4 → ruling (c);
- 5 → B, with a row;
- 6 → moot;
- 7 → A;
- (b) → toml, still put to the human as H2;
- (c) → no conflict, still put at the opening as H4;
- B1 → A, an engineering call;
- whether agents may edit `ci.yml` → yes, by the precedent of `72ca156a`.

**Still open.** The main session takes these with the recommendation unless it puts them to the human.

| # | Fork | A (robust, recommended) | B |
|---|---|---|---|
| **F1** | How lane P is run | One serial lane. No conflicts in the shared tables, and one tree for the strict list. | Two sub-lanes by family file. About 30 % faster, if the gate queue allows it, but with conflicts in `route_catalog.rs`, `scheduled.tsv` and the projection. |
| **F2** | The raw-write census | A generated list held equal to a scan by the site scan's own reader. A new raw write shows up in the diff. | A `dev/` script and its suite only. |
| **F3** | The byte-table record | Its committed files are held equal to the tool's split of the committed print. | The tool and its suite only. |
| **F4** | Pacing of new reasons | Batched at phase ends. `ruled: None` is named, and refused only at P-Z. | A halt to the human per C-task. |
| **F5** | The rig link plant | Build it, with the guard's target classes. | Decline it, and leave it to `wf-pre-driver-kit` at the first stop. |
| **F6** | `dev/ci-runs` | Build it, merged with the failing-jobs lister. | Decline it: the drift check covers git. |

The human's forks are H2 to H4 (§3).

## 6. Not established

- **The code counts of the dynamic sites.** PC establishes them.
- **Whether the state takes a round-2 record written through the writers** before the opening is done (RB1's fixtures).
- **Whether `the_committed_projection_is_the_catalogs_answers` can be refused under CI** in the same way the goldens are; P0 has to check how the goldens do it.
- **Whether the print the byte-table record is read off is committed** (F3).
- **How long the `g_migration` parts take** after about 250 more cases. U0 sets the bounds now; P-Z measures them again.
