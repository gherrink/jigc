<!-- M52 per-axis review (re-run) — axis 8 · freeze & migration — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit e519e4eb), 2026-09-21. -->

<!-- M52 per-axis review (re-run) — axis 8 · adopter docs & help — RECONCILED (driver table + reconciliation ledger) -->
<!-- reconciler: drove every Codex claim and both driver defects on the installed `jigc 1.0.0-rc.16`, 2026-09-21 -->

# M52 per-axis review — AXIS 8 · adopter docs & help — RECONCILED

**Reconciler's binary assert, before anything else:**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.16
```

**The reconciliation rule applied** (`acceptance-design.md` → The reconciliation rule): every Codex
claim entered as `lead(codex, …)` and was **driven** to a repro or to a refutation carrying the
falsifying datum; every driver defect was **re-driven once** by the reconciler before it was kept.
The Codex pass reported **no new leads** — its output is 5 M51-row dispositions plus a
consistent-source-read set, and each of those is a claim, so each was driven.

**Demotions: none.** The demotion scan is in the ledger (§R.4) — ten rows carry a `"the same …"`
back-reference instead of a literal argv, which is the table's idiom and not a missing driven
record; eight of the ten were re-driven by the reconciler and all eight reproduced. Three **datum
corrections** to otherwise-holding rows are recorded in §R.5 (rows 46, 50, and the driver's §3/§4
cross-reference), none of which changes a verdict.

Fixtures: `dev/jigc-rig {bare, committed-singletons} --binary /Users/maurice/.local/bin/jigc`,
two-step eval, every root a `mktemp -d`. No `rm -rf` on a variable path anywhere.

---

# PART 1 — THE DRIVER TABLE (unchanged)

# M52 per-axis review — AXIS 8 · adopter docs & help — DRIVER TABLE

**Binary asserted first, before anything else:**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.16
```

Release posture. Route-fence panics (`#[cfg(debug_assertions)]`) do not exist here — every row
below is what an adopter's installed binary does, **after** M52's seven audit fixes
(`ad527fa4` `6c2de03a` `33bd5692` `68d14cd3` `a83a9e60` `66af090a`, plus the close).

All fixtures built with `dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc`, two-step
eval, states `bare` · `fresh` · `committed-singletons`, one with `--start record-decision
--git-state merge`. No `rm -rf` on a variable path anywhere; every root is a `mktemp -d`. Nothing
was hand-written into `.jigc/` — every fixture state was built by driving the binary. I did **not**
read the Codex source pass for this axis.

One instrument note, recorded because it bit once and was caught: an early reading of
`jigc workflow … --preview`'s exit came through `| head -6` and reported **0**; re-driven bare
with the output redirected, the exit is **1**. Every exit in this table was captured bare or via
a redirect, never through a pipe.

---

## 0 · The door set, derived from the code (counts read at `HEAD`, not from the design doc)

| registry | file | count read | method |
|---|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs:1834` | **47** clap leaves | `(&[` rows |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs:3110` | **14** rows / **18** arms | `PathArgOccurrence{` / `PathArgArm{` |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs:2522` | **16** rows | `(&[` rows |
| `SLUG_DOORS` | `crates/cli/src/cli.rs:2829` | **6** rows | `SlugDoor{` |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs:2589` | **25** rows | `WorkUnitIdDoor{` |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs:2003` | **47** rows (total leaf classification) | `BehalfDoor{` |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:171` | **10** rows / 9 verbs | `CommittingDoor{` |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs:3275` | **6** (`[&DestroyingDoor; 6]`) — 4 `Refuse`, 2 `Displace`, 0 `Narrate` | type + `Disposition` sites |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:968` | **7** (`probe-unreliable` · `oob-rename` · `unmigrated-corpus` · `ahead-corpus` · `orphaned-instance` · **`home-vacated`** · `foreign-squatter`) | `id:` literals |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs:6030` | **64** arms | `EnvelopeArm{` |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:5415` | **4** (`store::NOT_FOUND` · `NO_SUCH_LEAF` · `UNKNOWN_TYPE` · `task::FIXED_IDENTITY`) | members |
| `ROLLBACK_POPULATIONS` | `crates/cli/src/rollback.rs:169` | **11** | `Population{` |
| `TASK_AREA_FILES` | `crates/engine/src/state.rs:129` | **13** | members |
| `RelocateRefusal::ALL` | `crates/cli/src/relocate.rs:61` | **10** variants | enum variants |
| `InProgress::ALL` | `crates/cli/src/repo.rs:173` | **9** variants | enum variants |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **3** | `Fault{` (test-side registry) |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs:757` | **18** | declared `[…; 18]` |
| `engine::validate::STORE_FAMILIES` | `crates/engine/src/validate.rs:473` | **7** | (M51-minted, unmoved) |
| `doc::WHOLE_DOC_KEYS` | `crates/cli/src/doc.rs:5344` | **6** | members |
| the `suppressed.door` set | `packs/*/workflows/*.yaml` | **22** `suppressed:` blocks, **14** carrying `door:` | grep |
| generated `*_long_about()` | `cli.rs`, `doc.rs`, `milestone.rs`, `task.rs` | **13** (was 4 at M51) | `fn .*_long_about` |

**Axis 8's own door set is derived, not enumerated by a registry** — its subject is *the
doc-sentence batch*, so its doors are the verbs those sentences name. For M52 the batch is
**Increment 10's Grouped scope** (roadmap → Milestone 52, Increment 10) plus the surfaces
Increments 3, 4, 6, 8 and 9 moved into the two `include_str!`'d guides
(`crates/cli/src/setup.rs` → `QUICKSTART.md` + `MIGRATING.md`, composed into
`.claude/skills/jigc/SKILL.md`), plus the **13** generated help texts (nine more than M51's four —
M52 moved the committing-door helps and five `doc` helps onto generators).

**Doors reached by ≥1 driven row below: 28** — `start` · `workflow` · `setup` · `uninstall` ·
`upgrade` · `ingest` · `migrate` · `migrate-corpus` · `rename` · `validate` · `describe` ·
`doc show` · `doc schema` · `doc list` · `doc rename` · `task list` · `task validate` ·
`task discard` · `task finalize` · `task bind` · `config set` · `milestone create` ·
`milestone add-task` · `milestone add-from-spec` · `milestone list-tasks` · `milestone provision` ·
`milestone join`† · `milestone discard` · `milestone finalize`. Verbs used only to build a fixture
(`doc create`, `doc set-slot`, `doc set-field`) are **not** counted — a fixture builder is not the
door of a row (M51's rule, kept). † **`milestone join` is NOT counted** — no row of mine drove it;
it is listed here only to say so explicitly, and the count above excludes it.

**Cells** (unchanged from M51's Part 2 row): **C1** the sentence's claim driven at the verb ·
**C2** the generated help text equals the registry that generates it · **C3** the guide sentence
is true after a real install.

---

## 1 · M51 §A rows — CLOSED / STILL-OPEN (the re-run's first deliverable)

M51's axis-8 ledger carried **five** §A rows (`README.md` §A → Axis 8: CX-1, CX-2, CX-3, D-1,
D-2). Every one re-driven on rc.16:

| M51 row | verdict on rc.16 | the datum |
|---|---|---|
| **CX-1** — `migrate-corpus --help`'s *"`--dry-run` writes nothing at all"* falsified by the invocation log | **CLOSED** | all three homes now carry `cli::NO_WRITE_EXCEPTION` verbatim: `migrate-corpus --help`, `validate --help`, and the **driven** `--dry-run` ack |
| **CX-2** — the installed guide's *"every `jigc setup` rewrites it"* false for a user-modified guide | **CLOSED** | the installed `SKILL.md:8` now scopes the rewrite (*"replaces it **while it is still jigc's**"*) and names `adapter-guide.user-modified`; driven, the advisory fires and the file is byte-untouched |
| **CX-3** — the guide's one install command cannot run from the repo it is installed into | **CLOSED** | the installed `SKILL.md:27` + `:33` now scope it — *"from a **clone of the jigc repository**"*, and the code block carries `# cwd: a clone of the jigc repository, not your own project` |
| **D-1** — `commit-recording.stale-title` silent in the default format at the two surfaces F-9 named | **CLOSED** | both: `doc rename --task` prints the advisory in `agent` text, and `task finalize --dry-run`'s text arm now names **every** code its JSON sibling carries (2/2, `file-state.staged-copy` included) |
| **D-2** — `jigc task discard --help` never says it commits | **CLOSED** | all **9** `COMMITTING_DOORS` verbs' helps state the commit they land; `milestone add-task`'s and `add-from-spec`'s acks name their sha |

**5 CLOSED · 0 STILL-OPEN.**

M51's five **observations** (not §A rows, re-checked because the re-run's coverage table must not
silently lose them):

| M51 obs | verdict on rc.16 |
|---|---|
| **O-1** — `setup`'s *"every path in its own install footprint"* universal vs `.git/hooks/pre-commit` | **STILL-OPEN, unchanged** — driven: a foreign hook does not stop `setup` (exit 0, body preserved verbatim, `grep -c 'ACME CORP POLICY HOOK'` → 1); QUICKSTART:105–111 still states the universal and now names the *regenerated* files as the illustrative case, not the hook. Prose precision; no byte at risk; the bound is declared at `install_candidate_paths` |
| **O-2** — `jigc task list --format json` carries no `schema_version` | **STILL-OPEN, unchanged** — driven, still a bare `[]`; the census's declared top-level-array anomaly |
| **O-3** — MIGRATING item 4 understates `jigc rename` | **STILL-OPEN, unchanged** — MIGRATING:50 still reads *"`jigc rename` … commit without asking this question at all"*; driven, `rename` refuses harder (`rename.dirty-tree`, exit 1, nothing committed) |
| **O-4** — the EC-11 generated-help count discrepancy in the planning record | **superseded** — the set is now **13** generated `*_long_about()`, so no numeral in any planning record describes it; all four of M51's originals re-driven plus four M52-new ones |
| **O-5** — `STORE_FAMILIES` has **7** members, not the Settle's five | **STILL-OPEN as a record fact, product correct** — still 7, and `validate --help` renders all seven in declaration order |

---

## 2 · The `(door, cell)` table

| # | door | cell | argv driven | exit | code \| none | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `migrate-corpus` | C2 | `jigc migrate-corpus --help` | 0 | none | none | the 18-kind list | **matches** `SchemaChangeKind::ALL` (18), same order |
| 2 | `migrate-corpus` | C2/C1 | `jigc migrate-corpus --help` + the driven `--dry-run` ack | 0 | none | none | `NO_WRITE_EXCEPTION` verbatim in both | **matches — CX-1 CLOSED** |
| 3 | `validate` | C2 | `jigc validate --help` | 0 | none | none | *"the sweep itself writes nothing: the opt-in `invocation-log` … is the one exception"* + the 7-family sweep-order list | **matches** `STORE_FAMILIES` (7) and `NO_WRITE_EXCEPTION` |
| 4 | `doc show` | C2 | `jigc doc show --help` | 0 | none | none | *"keyed by `type`, `slug`, `item-count`, `schema-version`, `fields`, `sections`"* | **matches** `WHOLE_DOC_KEYS` (6) |
| 5 | `doc show` | C2 (wire) | `jigc doc show vision:vision --format json` | 0 | none | none | top-level key set | **matches** — `['fields','item-count','schema-version','sections','slug','type']` |
| 6 | `doc schema` | C2/C3 | `jigc doc schema adr --format json` | 0 | none | none | `contract-version: 7`; `identity {kind,address}`; `home {kind,path}` | **matches** the help's *"`contract-version: 7`"* and M52 Inc 6 |
| 7 | `doc list` | C2/C1 | `jigc doc list` + `--format json` over an unregistered file | 0 | none | none | `adr:foreign-adr … unregistered`; JSON `{docs:[{id,path,state,item-count}]}` | **matches** the help's pinned shape |
| 8 | `doc list` | C1 | same, over a stamped doc no doctype claims | 0 | none | none | text `(none) docs/orphan-note.md orphaned`; JSON `id: null`, `item-count: null` | **matches** the help's *"null identity and no item count"* |
| 9 | `validate` | C1 | `jigc validate` on that same state | **1** | `schema-conformance.orphaned-instance` (blocking) | Mechanical (`jigc ingest`) + Human | the exit-flip trailer | **matches** the `doc list` help's *"`jigc validate` blocks on it"*; `STORE_EXIT_FLIPS` member 5 |
| 10 | `task discard` | C2 | `jigc task discard --help` | 0 | none | none | *"discarding one settles its milestone's committed record to `discarded` and commits it … An ordinary task's discard commits nothing"* | **matches — D-2 CLOSED** |
| 11 | `task finalize`·`milestone finalize`·`rename`·`migrate-corpus`·`milestone create`·`milestone add-task`·`milestone add-from-spec`·`milestone discard`·`task discard` | C2 | `jigc <leaf> --help` for all **9** `COMMITTING_DOORS` verbs | 0 | none | none | each states the commit it lands | **matches** — the D-2 class, swept |
| 12 | `milestone create` | C1 | `jigc milestone create "M1 the first"` | 0 | none | Informational | `record commit: 67ca085 — the record on its own; anything else you had staged stayed staged`; `HEAD` moved | **matches** its help |
| 13 | `milestone add-task` | C1 | `jigc milestone add-task m1-the-first "do the thing"` | 0 | none | Informational | `record commit: ee325a1 …`; `HEAD` moved | **matches** — the second half of D-2's fix (the ack names the sha) |
| 14 | `task discard` | C1 | `jigc task discard do-the-thing` (a sub-task) | 0 | none | Informational | `record commit: 1971787 — this sub-task's milestone record, settled to `discarded`…`; `HEAD` moved | **matches** the help and MIGRATING item 6 |
| 15 | `doc rename` | C1 | `jigc doc rename adr:use-redis-caching --to "Use Memcached caching" --task <id>` | 0 | `commit-recording.stale-title` **in agent text** | Mechanical (`doc set-slot commit:<id>#summary`) | the advisory printed beside the ack | **matches — D-1 producer half CLOSED** |
| 16 | `task validate` | C1 | `jigc task validate <id>` after that rename | 0 | `commit-recording.stale-title` + `file-state.staged-copy` | Mechanical / Informational | both advisories in text | **matches** |
| 17 | `task finalize` | C1 | `jigc task finalize <id> --dry-run` after that rename | 0 | both codes **in text** | Mechanical | the text arm's code set == the `--format json` arm's (2/2) | **matches — D-1's second half CLOSED** |
| 18 | `setup` | C3 | `jigc setup` on a `bare` rig, then read `.claude/skills/jigc/SKILL.md` | 0 | none | none | 39 894 bytes; `jigc-version: 1.0.0-rc.16`; `jigc-body-blake3: 70ba3764…`; the ack's `jigc guides →` row | **matches** — the M52 batch shipped into the installed artifact (M51 measured 34 217 bytes at rc.15) |
| 19 | `setup` | C3 | grep the installed guide for CX-3's needle | — | none | none | `:27` *"from a **clone of the jigc repository**"*, `:33` `# cwd: a clone of the jigc repository, not your own project` | **matches — CX-3 CLOSED** |
| 20 | `setup` | C3/C1 | user's own `SKILL.md` **committed**, then `jigc setup` | 0 | `adapter-guide.user-modified` (advisory) | Human (keep it, or delete + re-run) | ack carries **no** `jigc guides →` row; file 41 bytes, untouched | **matches — CX-2 CLOSED**, and the guide sentence at `:8` now describes exactly this |
| 21 | `setup` | C1 | `jigc setup` over a dirty **regenerated** path (`.jigc/AGENT.md`) | **1** | `setup.dirty-install-path` | Human + `--force` | *"nothing was installed and no install commit was made — `HEAD` is untouched"*; the user's line still on disk | **matches** QUICKSTART:105–111 (M51 row 13, held) |
| 22 | `setup` | C1 | `jigc setup` over a foreign `.git/hooks/pre-commit` | 0 | none | none | exit 0, no refusal; `grep -c 'ACME CORP POLICY HOOK'` → 1 | **matches the declared bound** — **O-1** stands (§1) |
| 23 | `setup` | C3 | `jigc setup` with a **rejecting** `pre-commit` hook planted | 0 | none | none | `install commit → 522f016`; `git log -1` = `chore(jigc): install jigc workspace config` | **matches** QUICKSTART's stated single `--no-verify` exception |
| 24 | `upgrade` | C1 | `jigc upgrade` | 0 | none | none | *"1 recorded config delta(s) re-apply clean … and the adapter's guide artifact `.claude/skills/jigc/SKILL.md` is still jigc's own"* | **matches** — the guide-artifact clause is new and true |
| 25 | `task validate` | C1/C3 | `jigc task validate <id>` under a live `git merge` (rig `--git-state merge`) | **1** | `repo.operation-in-progress` | Mechanical (`git merge --continue` / `--abort`) | *"a merge is in progress — the repository is not in a committable state"* | **matches** QUICKSTART:280–284's new posture-preview sentence |
| 26 | `task finalize` | C1 | `jigc task finalize <id>` in the same state | **1** | `repo.operation-in-progress` | Mechanical | byte-identical refusal | **matches** — preview and door agree |
| 27 | `task finalize` | C1/C3 | finalize a task whose area held 2 foreign entries | 0 | none | Informational | `note: … they were moved aside, not taken:` + `from → to` per entry | **matches** QUICKSTART:210–227's displacement paragraph |
| 28 | `task finalize` | C3 (wire) | the same with `--format json` | 0 | none | none | `committed.displaced = [{from,to}]` | **matches** the guide's *"and on the landed `--format json` envelope's `committed.displaced`"* |
| 29 | `uninstall` | C2 | `jigc uninstall --help` | 0 | none | none | about: *"**Four** states it refuses … `--force`, which deletes **all four**"*; `--force` arg: *"Inert when **all three** guards are already clean"* + a three-population list | **DEFECT N-1** (below) — the same help text contradicts itself |
| 30 | `uninstall` | C1 | `jigc uninstall` with only the 4th guard dirty (`.jigc/displaced/u-four/notes.txt`) | **1** | `uninstall.foreign-bytes` | Mechanical (`--force`) + Human (`rm -r`) | names the path; `worktrees=1`, `task list` = no active tasks | **matches the about**, falsifies the `--force` clause |
| 31 | `uninstall` | C1 | `jigc uninstall --force` on that exact state | 0 | none | Informational | `warning: removing the relocation workbench … they are not recoverable` + the 5-tracked-file warning; `.jigc/` gone | **DEFECT N-1** — not inert, and the population it destroys is the one the flag help omits |
| 32 | `uninstall` | C3 | the `--force` narration | 0 | none | none | two `warning:` blocks | **matches** MIGRATING's *"it never buys silence"* |
| 33 | `milestone discard` | C2 | `jigc milestone discard --help` | 0 | none | none | three codes named (`dirty-worktree`, `staged-prose`, `foreign-bytes`); `--force` = *"the explicit consent for all three guards … Inert when the three guards are already clean"* | **matches** — the sibling door's flag help *was* swept; the contrast is what makes N-1 an incomplete sweep |
| 34 | `milestone provision` | C2 | `jigc milestone provision --help` | 0 | none | none | *"A leftover that holds anything **refuses** … `--force` is the consent"*; flag: *"Inert when every path is empty or a live worktree"* | **matches** `DESTROYING_DOORS`' `Refuse{consent}` row |
| 35 | `validate` | C1/C3 | `rm CHANGELOG.md` (worktree-only), `jigc validate` | **1** | `schema-conformance.home-vacated` (blocking) | Human | route: *"the removal is not committed — restore it: `git checkout -- CHANGELOG.md`; the document is still in `HEAD`…"* | **matches** MIGRATING item 8's uncommitted arm; `STORE_EXIT_FLIPS` member 6 |
| 36 | `validate` | C1/C3 | the same deletion **staged** | **1** | `home-vacated` | Human | *"the deletion is staged and not committed — … `git restore --source=HEAD --staged --worktree -- CHANGELOG.md`"* | **matches** item 8's staged arm |
| 37 | `validate` | C1/C3 | the same deletion **committed** | **1** | `home-vacated` | Human | *"restore … and commit it — `5424a63` is the commit that removed it (`git show 5424a63 -- CHANGELOG.md`) — then `jigc ingest` to re-register it"* | **matches** item 8's committed arm, **the named commit included** |
| 38 | `ingest` | C1 | the route from row 37 run verbatim (`git show` → restore → commit → `jigc ingest`) | 0 | none | Informational | `adoptable CHANGELOG.md → changelog (adopted — indexed + baselined, no file moved)`; then `jigc validate` exit **0**, zero `home-vacated` | **matches** — the route is followable and it clears the finding |
| 39 | `validate` | C1/C3 | a **pre-jigc** `CHANGELOG.md` created and retired before `jigc setup` (rig `bare`) | 0 | none | none | `no findings — the committed store validates clean` | **matches** item 8's *"adopting a brownfield repository does not turn it red"* — M52 F1's fix, held |
| 40 | `validate` | C1/C3 | retire the **last ADR** (a slugged doctype) and commit | 1 | `reconciliation.rename` only — **no** `home-vacated` | Mechanical (`jigc unmanage`) | — | **matches** item 8's *"a collection that legitimately retired its last ADR stays green"* (of `home-vacated`), and the `reconciliation.rename` sentence beside it |
| 41 | `validate` | C1/C3 | a never-adopted `.md` at the `adr` home | **1** | `schema-conformance.unadopted-instance` (advisory, exit-flipping) | Mechanical (`jigc ingest` / `jigc migrate … --as adr`) | the exit-flip trailer | **matches** MIGRATING item 5's cross-verb claim (validate side) |
| 42 | `migrate-corpus` | C1/C3 | `jigc migrate-corpus` over the identical file | **0** | none (the finding rides `unadopted`) | Mechanical | `0 migrated, 0 already current, 0 blocked, 1 not adopted` | **matches** — *"`migrate-corpus` still exits 0 over them"* |
| 43 | `migrate-corpus` | C3 (wire) | the same with `--format json` | 0 | none | none | 8 top-level keys `['already_current','blocked','commit','dry_run','hook_output','migrated','unadopted','unfilled']`; `blocked: []` | **matches** MIGRATING item 5's *"five sets … and alongside them `commit`, `hook_output` and `dry_run`"* |
| 44 | `task finalize` | C3 | `task finalize` under a rejecting `pre-commit` hook | **1** | (survivable frame; log `error_code`) | Human (fix the hook) + the re-run line | *"task hook-run is intact — nothing was committed, your task's staged docs are still in `.jigc/tasks/hook-run/docs/`, and anything you had `git add`-ed is still in git's index. Fix the hook's complaint, then re-run `jigc task finalize hook-run`."* | **matches** MIGRATING item 5's per-door clause, verbatim |
| 45 | `task finalize` | C3 (log) | the invocation log after row 44 | — | `error_code: "finalize.commit-rejected"` | none | one record | **matches** *"each with its own error code in the invocation log"* |
| 46 | `milestone create` | C3 | `milestone create` under the same hook | **1** | (frame; log `error_code: "milestone-create.commit-rejected"`) | Human + re-run `jigc milestone create 'M9 rejected'` | *"the record write and the milestone workbench were both rolled back, so nothing of milestone:m9-rejected survives"*; `.jigc/milestones` and `docs/milestone-records` both absent | **matches** item 5's third per-door clause **and** M52 F3's *"state clause is a function of the rollback's outcome"*; the re-run argv is shell-quoted |
| 47 | `start` | C1 | `jigc start --workflow migrate-adr "x"` | **1** | `workflow.verb-routed` | Mechanical (`jigc migrate <path> --as adr`) | the refusal + route | **matches** Inc 9's registered code and exit |
| 48 | `workflow` | C1 | `jigc workflow milestone-execution --preview` (bare, redirected) | **1** | `workflow.verb-routed` | Mechanical (`jigc milestone execute <milestone-id>`) | text **and** `--format json` findings envelope | **matches** — both arms, exit 1 |
| 49 | `start` | C1 | `jigc start "some new intent"` (the router) | 0 | none | Informational | the catalog lists **12** workflows; **0** of the **14** `suppressed.door` members appear | **matches** Inc 10's *"the catalog stops offering a route that refuses"* |
| 50 | `describe` | C1 | `jigc describe --workflows` | 0 | none | none | 33 entries; 12 catalog, **22 off-catalog, 22 carrying a catalog reason — 0 without** | **matches** the router's closing sentence *"each of those carries the reason it is hidden from the catalog"* (the M46-audit class stays closed) |
| 51 | `start` | C1 | `jigc start` (orientation) with an open task | 0 | `file-state.staged-copy` | Informational | `Active task:` + `workflow`/`intent`/`base`/`staged`/`findings: 1 advisory` | **matches** M50's third `OrientationView` |
| 52 | `start` | C1 | `jigc start "second thing" --workflow record-decision` with a task already open | 0 | none | Informational | `also open: 1 other task was already open before this call — nothing here touched it; several open tasks are legal…` | **matches** |
| 53 | `config set` | C1 | `jigc config set docs-root ""` | 0 | none | Informational | *"set `docs-root` = `.` (you typed an empty value, which names the repository root `.` — a root knob has no unset spelling)"* | **matches** Inc 10's root-knob ack item |
| 54 | `config set` | C1 | `jigc config set docs-root "x/../y"` | 0 | none | Informational | *"= `y` (you typed `x/../y`, which folds to `y` — the one spelling every reader resolves)"* | **matches** |
| 55 | `config set` | C1/C2 | `jigc config set docs-root docs --format json` on a placement-only corpus | 0 | none | none | `relocated: []`; `docs/roadmap.md` + `docs/decisions-log.md` unmoved | **matches** `config set --help`'s EC-16 clause (*"a placement doctype's file … a `docs-root` re-point never moves it"*) |
| 56 | `milestone list-tasks` | C1 | `chmod 000 .jigc`, then `jigc milestone list-tasks m1` | **1** | none (typed error, route-bearing) | Human | *"cannot read the `.jigc/config/` cascade layer: Permission denied (os error 13) — restore read access to the `.jigc/` directory and run the command again"* | **matches** Inc 8/LD-4 — not the *"this project isn't set up"* lie |
| 57 | `start` | C1 | `jigc start` in the same unreadable state | **0** | none | Informational | *"This project isn't set up. No project config layer is present…"* | **matches the declared bound** at `locate.rs` (*"bare `jigc start`'s orientation never calls it"*) — **Observation O-6** |
| 58 | `task list` | C1 | `jigc task list` in the same state | 0 | none | Informational | `jigc task list — no active tasks` | same declared bound, second leaf — **O-6** |
| 59 | `migrate` | C1 | `jigc migrate adir --as adr` | **1** | (read-fault, route-bearing) | Mechanical | ``could not read the foreign `adr` source at `adir` `` — the token as typed | **matches** — M51's audit LOW holds in the release binary |
| 60 | `doc show` | C1 | `jigc doc show adr:nope` | 1 | `store.not-found` | Mechanical (`… --task <task-id>` + `jigc task list`) | the findings envelope, not a flattened `{error}` | **matches** M52 F4's `ENVELOPE_OWED_CODES` |
| 61 | `doc show` | C1 | `jigc doc show adr:nope --task ghost-task` | 1 | `finalize.no-task` | Mechanical (`jigc task list`) | *"no task `ghost-task`"* | **matches** |
| 62 | `task bind` | C1 | `jigc task bind nosuch adr:none <task>` | 1 | `task-bind.undeclared-role` | Mechanical (`jigc start --task <id>`) | code + route present in text; `--format json` is the **flattened** `{error}` arm | **matches** M52 F5's *declared* decision (the new codes flatten with code and route rather than joining `ENVELOPE_OWED_CODES`) |
| 63 | `rename` | C1 | `jigc rename vision:vision --to "New Vision"` with a foreign path staged | **1** | `rename.dirty-tree` | Human | *"a rename commits in place with no pathspec, so anything already in the index would ride its commit"* | **matches the literal claim** — **O-3** stands |
| 64 | `rename` | C1 | the same with a task in flight | 1 | `rename.in-flight` | Mechanical (finalize or discard, both spelled) | — | **matches** |
| 65 | `rename` | C1 | `jigc rename vision:vision --to "New Vision"` (clean, no task) | **1** | `write.identity-change` | Mechanical (`jigc rename vision:vision --to 'New Vision' --slug vision`) | *"this doctype's identity is fixed to its type … only a retitle is supported"* | **matches** M52 Inc 6; the route is shell-quoted |
| 66 | `rename` | C1 | that route **run verbatim** | 0 | none | Informational | `renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)`; `VISION.md:5` becomes `# New Vision`; commit subject `rename VISION.md -> VISION.md` | **DEFECT N-2** — the only thing that moved (the title) is named on neither the ack nor the commit |
| 67 | `rename` | C1 | the same argv a second time (the genuine no-op) | 0 | none | Informational | `no-op: vision:vision already holds the title "New Vision" at VISION.md — nothing renamed, nothing committed`; `HEAD` unmoved | **matches** M48's idempotent-ack rule — and is the datum that makes N-2 a defect rather than a design choice |
| 68 | `rename` | C1 | the same shape on a **slugged** doctype (`adr:alpha-choice --to "Alpha Choice Revisited" --slug alpha-choice`) | 0 | none | Informational | `renamed adr:alpha-choice -> adr:alpha-choice (docs/decisions/alpha-choice.md -> …)`; H1 became `# Alpha Choice Revisited` | **DEFECT N-2**, class-widening cell: not a fixed-identity quirk — every retitle-without-reslug |
| 69 | `task discard` | C1 | `jigc task discard <ordinary task>` on a freshly started task | **1** | `task-discard.staged-prose` | Mechanical (read-back / finalize / `--force`) | — | **matches** MIGRATING item 6's *"expect the refusal on any task you have started"* |
| 70 | `config set` | C1 | `jigc config set invocation-log true`, then the `--dry-run` of row 2 | 0 | none | Informational | the log file is created by the dry run (194 bytes, `binary_version: 1.0.0-rc.16`) | **matches** — CX-1's exception, driven end to end |
| 71 | `uninstall` | C1 | `jigc uninstall` over a state with **all four** guards dirty (open task staging `commit:open-one` · untracked `.jigc/config/notes.txt` · parked `.jigc/displaced/zz/parked.txt`) | **1** | `uninstall.foreign-bytes` only | Mechanical | exactly **one** blocking finding printed | **matches** the about's *"Any of them removes nothing until you re-run"* — guards are reported one at a time, by design |
| 72 | `uninstall` | C3 | `jigc uninstall --force` on that same four-dirty state | 0 | none | Informational | **four** `warning:` blocks — relocation workbench · staged docs of 1 open task · 1 file no index has a copy of · 5 tracked files | **matches** MIGRATING's *"it never buys silence"*; the run-time surface knows four populations while the `--force` help enumerates three (N-1) |

**Rows driven: 72.** Every row's argv ran on the installed `1.0.0-rc.16` and its verdict is
recorded; the repro blocks for the defects and for the load-bearing cells are in §4.

---

## 3 · Defects

### N-1 — `jigc uninstall --help` contradicts itself about its own guard count, and the `--force` clause is the half that lies

**What is contradicted.** One `--help` output, two numerals:

* the `about` (`cli.rs:290–313`): *"**Four** states it refuses instead of destroying … Any of them
  removes nothing until you re-run — or pass `--force`, which deletes **all four** with the
  install."*
* the `--force` arg help (`cli.rs:316–320`): *"Remove `.jigc/` even when it holds **a fan-out
  worktree with content, an open task's staged docs, or a workbench file no index has a copy
  of** … **Inert when all three guards are already clean.**"*

The fourth guard — `uninstall.foreign-bytes`, M52 Increment 4's own new member, covering a file
jigc did not write inside a working area **and anything parked under `.jigc/displaced/`** — is
absent from the flag's enumeration, and the flag states a false predicate about it.

**Driven, the predicate is false and the consequence is unrecoverable bytes.** In a state where
all three enumerated guards are clean (no registered worktree, `jigc task list` → *no active
tasks*, nothing else untracked under `.jigc/`), `--force` is **not** inert: it deletes the
parked file and says so.

**The sweep's own sibling shows this is an omission, not a decision.** `jigc milestone discard`'s
flag help *was* swept in the same wave — it names all three of its guards including
`foreign-bytes` and says *"the explicit consent for all three guards"*, matching its `about`.
`uninstall`'s `about` moved from *Three* (rc.15, M51 row 37) to *Four*; its flag help did not move.

**Severity: LOW.** No silent loss — the refusal fires first, the `--force` narration names every
path and says *"not recoverable"*, and the consent is explicit. It is a law-1 contradiction inside
one help text, on the consent flag of a destroying door, and the class is *one* flag.

**Two supporting facts, both driven, both recorded because they bound the finding rather than widen
it:** (a) the refusal reports **one guard at a time** — over a state with all four dirty (an open
task staging `commit:open-one`, an untracked `.jigc/config/notes.txt`, and parked bytes under
`.jigc/displaced/`), `jigc uninstall` printed exactly **one** blocking finding,
`uninstall.foreign-bytes`, and named neither of the others; the about's *"Any of them removes
nothing until you re-run"* and the route's *"then re-run"* make that consistent rather than a lie,
so it is **not** graded. (b) the `--force` **narration is complete**: over that same four-dirty
state it printed **four** warning blocks, one per population (relocation workbench · staged docs of
1 open task · 1 file no index has a copy of · 5 tracked files), which is what makes the flag help's
three-population enumeration the outlier — the door's own run-time surface already knows there are
four.

**Repro (setup · argv · observed):**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$JIGC start "u four" --workflow record-decision                       # → task u-four
$JIGC doc create adr --title "U four" --task u-four
for s in context decision consequences; do printf 'P.' | $JIGC doc set-slot adr:u-four#$s --task u-four --from-file -; done
$JIGC doc set-field commit:u-four#header/type --task u-four --value docs
printf 'u four' | $JIGC doc set-slot commit:u-four#summary --task u-four --from-file -
printf 'B.'     | $JIGC doc set-slot commit:u-four#body    --task u-four --from-file -
printf 'irreplaceable notes\n' > .jigc/tasks/u-four/notes.txt
$JIGC task finalize u-four                                            # EXIT=0, the file is displaced

$ git worktree list | wc -l          →  1          # guard 1 clean (main checkout only)
$ $JIGC task list                    →  no active tasks     # guard 2 clean
# guard 3 is proven by the door itself, below: the --force narration carries exactly TWO warning
# blocks (relocation workbench + tracked files) and neither of the other two populations' blocks.

$ $JIGC uninstall
blocking · uninstall.foreign-bytes — `.jigc/` holds 1 path(s) jigc did not write …
  .jigc/displaced/u-four/notes.txt
  route: … or, once you have confirmed they hold nothing you need, `jigc uninstall --force` deletes them with the install
EXIT=1

$ $JIGC uninstall --force
warning: removing the relocation workbench .jigc/displaced discards work that is not in git:
    .jigc/displaced/u-four/notes.txt
  note: the relocation workbench is the only copy of these bytes — they are not recoverable.
warning: removing `.jigc/` also removes 5 tracked file(s) under it:
    …
EXIT=0                                # NOT inert — the "all three guards are already clean" state
$ grep -c '^warning:' <that output>  → 2     # only the 4th population and the tracked set:
                                              # no "staged docs of N open task(s)" block, no
                                              # "no index has a copy of" block — guards 2 and 3 clean

$ jigc uninstall --help | grep -c 'Four states it refuses'        → 1
$ jigc uninstall --help | grep -c 'all three guards are already clean' → 1
```

### N-2 — every retitle-without-reslug at `jigc rename` acks a no-op and commits one, while the binary knows the title it just moved

**What is contradicted.** M52 Increment 6 mints `write.identity-change` for a fixed-identity
doctype and routes it at a **specific argv**, whose stated effect is a title rewrite:

```
route: `jigc rename vision:vision --to 'New Vision' --slug vision` keeps the identity that cannot
       move and rewrites only the title
```

Driven, that argv does rewrite the title (`VISION.md:5` → `# New Vision`) — and reports:

```
renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)
```

with the commit subject `rename VISION.md -> VISION.md`. The identity did not move, the path did
not move, and **the title — the only thing that moved, and the thing the route promised — is on
neither surface**, nor in the permanent git record the door lands.

**The binary has the datum.** Run the identical argv again and the no-op arm (M48's idempotent-ack
fix) prints the title:

```
no-op: vision:vision already holds the title "New Vision" at VISION.md — nothing renamed, nothing committed
```

So a reader cannot tell a *successful retitle* from *nothing happened* by the success ack, while
the *failure to act* ack is fully informative — the inverse of the intended asymmetry.

**The class is wider than the fixed-identity route.** Driven on a slugged doctype with an explicit
matching `--slug` (`adr:alpha-choice --to "Alpha Choice Revisited" --slug alpha-choice`), the ack
and the commit subject take the same no-op shape. The axis is *every `jigc rename` whose slug does
not move* — the fixed-identity doctype set (`placement ‖ singleton`, where `--slug <same>` is the
**only** legal form) plus every explicit same-slug retitle on a slugged doctype.

**Severity: LOW.** Nothing is lost and the write is correct; it is a law-1 surface defect on a
committing door, and it lands in git history where it cannot be re-printed.

**Repro (setup · argv · observed):**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

$ $JIGC rename vision:vision --to "New Vision"
blocking · write.identity-change — cannot reslug `vision:vision` … only a retitle is supported
  route: `jigc rename vision:vision --to 'New Vision' --slug vision` keeps the identity that cannot move and rewrites only the title
EXIT=1

$ $JIGC rename vision:vision --to 'New Vision' --slug vision          # the route, verbatim
renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)
EXIT=0
$ grep -n '^# ' VISION.md         → 5:# New Vision          # the title DID move
$ git log --oneline -1            → 4549109 rename VISION.md -> VISION.md
$ git show --stat --oneline HEAD  → VISION.md | 2 +-        # one real line changed

$ $JIGC rename vision:vision --to 'New Vision' --slug vision          # the genuine no-op
no-op: vision:vision already holds the title "New Vision" at VISION.md — nothing renamed, nothing committed
EXIT=0                                                                 # HEAD unmoved

# and the class, on a slugged doctype:
$ $JIGC rename adr:alpha-choice --to "Alpha Choice Revisited" --slug alpha-choice
renamed adr:alpha-choice -> adr:alpha-choice (docs/decisions/alpha-choice.md -> docs/decisions/alpha-choice.md), repointed 0 referrer(s)
EXIT=0
$ grep -n '^# ' docs/decisions/alpha-choice.md → 7:# Alpha Choice Revisited
```

---

## 4 · Observations (driven, not graded as defects)

- **O-1 · `setup`'s *"every path in its own install footprint"* universal** — carried from M51
  unchanged. Driven (row 22): a foreign `.git/hooks/pre-commit` does not stop `setup`, the body is
  preserved verbatim, and `install_candidate_paths`' doc-comment declares the exclusion. The ack's
  new line (*"local to this checkout — git cannot track this path, so the hook is not in the
  install commit"*) narrows the reader's exposure without amending the guide's universal.
- **O-2 · `jigc task list --format json` carries no `schema_version`** — re-driven, still `[]`;
  the declared top-level-array anomaly.
- **O-3 · MIGRATING item 4 understates `jigc rename`** — re-driven (row 63): the sentence
  *"commit without asking this question at all"* is literally true (the carryover gate is not
  asked) and the shipped behaviour is a **stricter** refusal, so the reader's natural inference is
  the opposite of the truth. Unchanged from rc.15.
- **O-4 · the generated-help set is now 13, not 4** — `migrate_corpus` · `validate` ·
  `doc show`/`create`/`rename`/`set-slot`/`set-field`/`add-item` · `milestone
  create`/`add-task`/`add-from-spec` · `task finalize`/`discard`. M51's O-4 (a 3-vs-4 count
  discrepancy between two planning artifacts) is superseded rather than closed: no record states
  the current number.
- **O-5 · `STORE_FAMILIES` is seven** — unchanged; `validate --help` renders all seven.
- **O-6 · bare `jigc start` and `jigc task list` still answer *"this project isn't set up"* /
  *"no active tasks"* at exit 0 over an unreadable `.jigc/`** (rows 57–58), while every door that
  goes through `locate::not_set_up` now tells the two states apart (row 56). This is **declared**
  in `crates/cli/src/locate.rs`'s doc-comment (*"the one arm this producer does not reach … a
  pinned-envelope move this wave's fixes-and-understandability-only boundary does not carry"*) and
  in `tests/unreadable_project_layer.rs`. Recorded, not graded: the declaration exists, but it
  lives only in a doc-comment and a test — it is **not** in M52's VERDICT *Declared bounds* list,
  so a reader of the wave's record would not find it.
- **O-7 · neither displacing door's `--help` mentions the displacement.** `DESTROYING_DOORS` has
  two `Disposition::Displace` members (`task finalize`, `milestone finalize`); the four `Refuse`
  members all state their guards in their own `--help` (rows 10, 29, 33, 34), and `uninstall`'s
  about even names `.jigc/displaced/`. The two doors that *create* that parking home say nothing
  about it in their help. Not graded a defect: the run narrates every `from → to` pair on stderr
  and on the envelope (rows 27–28), and the guide carries the paragraph (QUICKSTART:210–227), so
  nothing is ambushed at the moment it happens — but the door's own help is the one surface of the
  class left unswept, which is exactly D-2's shape one increment over.

---

## 5 · What I did NOT drive, and why

Stated plainly, never presented as driven:

1. **`(migrate-corpus, C3)` — MIGRATING item 5's per-door clause** (*"leaves the migrated bytes
   written and staged, and its re-run **lands** them"*). Reaching it needs a real schema bump
   (`--pack-from-dev --schema … --repin`) plus a rejecting hook — axis 7's fixture, and driving it
   here would prove axis 7 twice. The `migrate-corpus` frame's *other* half (the log's per-door
   `error_code`) is driven at `task finalize` and `milestone create` (rows 45–46).
2. **`(migrate-corpus, C3)` — the `unfilled` array's non-empty arm.** Same fixture, same reason.
   The key's presence and emptiness are driven (row 43).
3. **`(milestone provision | execute | join | finalize | discard, C1)` — the fan-out doors'
   behaviour.** I drove their `--help` (rows 11, 33, 34) and the count claims that name them, not
   a live fan-out. M51 drove them and recorded *matches*; they carry no §A row, and the re-run's
   first obligation is the §A rows. The second `Displace` door (`milestone finalize`) is therefore
   **un-driven** — only `task finalize`'s displacement is driven (rows 27–28).
4. **`(setup, C3)` — `jigc setup --force` over a dirty `.git/hooks/pre-commit`.** The declared
   bound at `install_candidate_paths` (*"`--force` does not probe the hook path"*) is unchanged
   since M51; I drove the plain path (row 22) only.
5. **`(setup, C3)` — `cargo install --path crates/cli` from an adopter repo** (CX-3's original
   repro). The fix scopes the *sentence*, not the command; I verified the corrected bytes in the
   **installed** artifact (row 19) and did not re-run the command that M51 drove to exit 101, nor
   `cargo install` into the operator's real `~/.cargo/bin` (it would replace the `1.0.0-rc.16`
   this review is measured against).
6. **`(upgrade, C1)` — a recorded config delta actually drifting.** Driven only in its clean case
   (row 24), over a corpus whose one delta re-applies cleanly. Manufacturing a drift is a
   multi-pack fixture (axis 7).
7. **The Tier-4 record corrections** (Increment 10's record half). No verb to drive; a
   source-reading job that belongs to the Codex pass.
8. **`ROLLBACK_POPULATIONS` (11), `PRE_DISPATCH_FAULTS` (3), `InProgress::ALL` (9),
   `RelocateRefusal::ALL` (10), `TASK_AREA_FILES` (13).** I read and report their counts (§0) and
   drove the cells where an axis-8 *sentence* reaches them — `InProgress` at the QUICKSTART
   posture-preview sentence (rows 25–26, the `merge` member), `TASK_AREA_FILES`' complement at the
   displacement sentence (rows 27–28), `ENVELOPE_OWED_CODES` at `doc show` (row 60). The other
   members are the subjects of axes 2, 4, 5 and 6; iterating them here would re-prove those axes
   and is **not** claimed.
9. **The 14 `suppressed.door` members individually.** Driven: two of them at the two compose doors
   (rows 47–48), and the **set-level** claim that none appears in the router catalog (row 49) and
   that every off-catalog workflow carries a reason (row 50, 22/22). The remaining 12 doors'
   individual refusals are axis 7's arm.

---

## 6 · What this adds over the flow-53 arms

**Axis 8 has no flow-53 arm, by an explicit decision** — the acceptance design's *"Deliberately
unrepresented"* paragraph names D10's and D11's surface batches and the record corrections, which
is precisely this axis's subject. So the honest comparison is against the arms whose behaviour
these sentences describe: **arm 3** (the destroying subject × `DESTROYING_DOORS` × `Disposition`),
**arm 6** (`ENVELOPE_ARMS` / `PRE_DISPATCH_FAULTS`), and **arm 2** (the posture family).

- **Arm 3 proves the doors' *behaviour* over the writer-set complement**: consenting doors refuse
  naming every path, the displacing doors move the complement to `.jigc/displaced/<id>/` and name
  it. It reads what the door *does*. It cannot see that the same door's `--force` help states a
  **false predicate about when it is inert** (N-1) — the behaviour arm 3 asserts is exactly the
  behaviour that falsifies the sentence, and no arm compares the two. Rows 29–34 are that
  comparison: the four `Refuse` members' help against their own guard sets, and the two `Displace`
  members' silence (O-7).
- **Arm 6 reads the wire.** It proves every leaf emits its declared key set and that a reject
  carries one JSON document. It does not read one English sentence, so it cannot see that
  `uninstall --help` counts four and three in the same breath, or that a success ack reports a
  no-op (N-2) — the `rename` envelope is conformant in both cells, which is why nothing reddens.
- **Arm 2 drives the nine git states at every acting door.** Rows 25–26 drive the *adopter's
  sentence about* that machinery — QUICKSTART's new claim that `jigc task validate` **previews**
  the posture finalize refuses under — which is a cross-verb claim (preview door vs. real door)
  that no single arm's registry carries and that a reader acts on before committing.
- **No arm installs the guide.** Rows 18–20 are the only place the composed artifact is read after
  a real `jigc setup`: the stamp moved to `1.0.0-rc.16`, the body hash moved
  (`70ba3764…`, 39 894 bytes against rc.15's 34 217), CX-2's and CX-3's corrected sentences are in
  the shipped bytes, and the refuse-to-clobber behaviour the sentence now describes is driven at
  the same door. A guide sentence that ships into every adopter repo is product surface; flow 53
  never opens it.
- **And rows 35–40 are the wave's own centrepiece read as an adopter reads it.** Arm 4 proves
  `home-vacated` fires on every vacated exact home and stays silent on the control. MIGRATING
  item 8 makes four further claims an arm does not: that the **route differs by removal state**
  (three states, three different routes — rows 35–37), that the committed arm **names the commit
  that removed it** and that following the route end to end **clears the finding** (row 38), that a
  brownfield repo does not go red (row 39, M52 F1's fix), and that a slugged collection retiring
  its last instance stays green (row 40). Those are the sentences an adopter gates CI on.

The general statement, unchanged from M51: flow 53's arms iterate **sets the code can enumerate**.
Axis 8's subject is the set of **sentences the code cannot enumerate** — which is why its door set
had to be derived from the wave's own edit set, and why both defects it found are one `--help`
string and one success ack rather than a registry member.

---

# PART 2 — RECONCILIATION LEDGER

The Codex source pass (`codex/axis8-codex.md`) opens: *"No new source-pass leads. I found no
adopter-doc/help door omitted from the stated axis and no bypass of the generated-help or
installed-guide seams."* Its substance is therefore **18 claims** — five M51-row dispositions, a
twelve-item consistent-source-read set, and one blanket completeness claim. Every one is entered
below as a lead and driven.

## R.1 · Codex claims → driven verdicts

### C-1 — `lead(codex, "CX-1 is CLOSED: the three universal no-write surfaces render the shared NO_WRITE_EXCEPTION")` → **CONFIRMED (repro)**

Driven on the installed binary. The const has one home (`crates/cli/src/cli.rs:65`) and its exact
bytes appear in both help surfaces; the `--dry-run` write it exempts was driven end to end.

```
$ jigc migrate-corpus --help | grep -cF "the opt-in \`invocation-log\` knob's gitignored \`.jigc/logs/invocations.jsonl\` record is the one exception"
1
$ jigc validate --help       | grep -cF "<same needle>"
1
# and the exception is real, not decorative — driven on a committed-singletons rig:
$ jigc config set invocation-log true ; jigc migrate-corpus --dry-run   # EXIT=0
$ ls -l .jigc/logs/invocations.jsonl   →  the file exists, binary_version: 1.0.0-rc.16
```

Also driven: `doc show --help` and `task finalize --help` do **not** carry the needle (0 each) —
correct, since neither claims to write nothing.

### C-2 — `lead(codex, "CX-2 is CLOSED: the quickstart conditions replacement on the body hash and names adapter-guide.user-modified")` → **CONFIRMED (repro)**

Driven at the door, not read in the source. The sentence is in the *installed* artifact and the
behaviour it describes fires:

```
rig=$(dev/jigc-rig bare --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"; cd "$REPO"
$ jigc setup                                        # EXIT=0
$ sed -n '8p' .claude/skills/jigc/SKILL.md
`jigc setup` wrote this file from jigc 1.0.0-rc.16 and replaces it **while it is still jigc's** —
while the body below still hashes to the `jigc-body-blake3:` line above. … Edit it and it becomes
yours: every later `setup` leaves it byte-identical and says so — `adapter-guide.user-modified` …

$ printf 'my own guide, hand-written and mine\n' > .claude/skills/jigc/SKILL.md
$ git add … && git commit -m "make the guide mine" --no-verify
$ BEFORE=$(shasum -a256 .claude/skills/jigc/SKILL.md)
$ jigc setup                                        # EXIT=0
advisory · adapter-guide.user-modified — `.claude/skills/jigc/SKILL.md` no longer carries the bytes
  jigc wrote, so jigc left it untouched rather than clobber your edits …
  route: keep your copy … or delete … and re-run `jigc setup` …
$ AFTER=$(shasum -a256 .claude/skills/jigc/SKILL.md);  [ "$BEFORE" = "$AFTER" ]  →  TRUE (byte-untouched)
$ grep -c 'jigc guides' <that ack>                  →  0     # the install row is withheld
```

### C-3 — `lead(codex, "CX-3 is CLOSED: the install command is scoped to a clone of the jigc repository")` → **CONFIRMED (repro)**

Read out of the **installed** artifact, at the two lines Codex cites in the source:

```
$ grep -n 'clone of the jigc repository' .claude/skills/jigc/SKILL.md
27:from a **clone of the jigc repository** — `crates/cli` is a path in *that* tree, so
33:# cwd: a clone of the jigc repository, not your own project
```

Bound, unchanged from the driver's §5.5: the fix scopes the **sentence**, not the command;
`cargo install --path crates/cli` from an adopter repo was not re-run (it would replace the binary
this review is measured against).

### C-4 — `lead(codex, "D-1 is CLOSED: both default-text paths pass carried findings through one renderer")` → **CONFIRMED (repro)**

Driven at both surfaces on one task, with the text arm's code set compared to its JSON sibling's:

```
rig=$(dev/jigc-rig committed-singletons …); … ; jigc start "use redis caching" --workflow record-decision
# … author adr:use-redis-caching + its commit doc …
$ jigc doc rename adr:use-redis-caching --to "Use Memcached caching" --task use-redis-caching
adr:use-memcached-caching (renamed to "Use Memcached caching" from adr:use-redis-caching)
advisory · commit-recording.stale-title — the staged commit summary still names `Use Redis caching` …
  at: commit:use-redis-caching#summary · line 10
  route: `jigc doc set-slot commit:use-redis-caching#summary --task use-redis-caching --from-file -`
EXIT=0                                              # the advisory IS in the default (agent) text

$ jigc task finalize use-redis-caching --dry-run                 # TEXT arm, EXIT=0
TEXT codes:  commit-recording.stale-title, file-state.staged-copy
$ jigc task finalize use-redis-caching --dry-run --format json   # JSON arm, EXIT=0
JSON codes: ['commit-recording.stale-title', 'file-state.staged-copy']     # 2/2 — parity holds
```

### C-5 — `lead(codex, "D-2 is CLOSED: task discard --help states it commits; COMMITTING_DOORS still has exactly ten rows")` → **CONFIRMED (repro)**

Both halves. The registry: `crates/cli/src/invocation_log.rs` carries **10** `CommittingDoor {`
rows over **9** verbs (`jigc milestone finalize` appears twice, `squash: true` / `squash: false`) —
which is the driver's §0 reading, not a conflict with Codex's "ten rows". The surface: all nine
verbs' `--help` were driven and each states the commit it lands.

```
$ jigc task discard --help | sed -n '3p'
A **sub-task** of a milestone is not workbench-local: discarding one settles its milestone's
committed record to `discarded` and commits it before the area goes, so `HEAD` moves and the ack
names the sha. … An ordinary task's discard commits nothing.
# and all nine, each exit 0, each stating its commit:
task finalize · milestone finalize · rename · migrate-corpus · milestone create · milestone add-task ·
milestone add-from-spec · milestone discard · task discard
```

### C-6 — `lead(codex, "migrate-corpus --help derives all 18 change kinds from SchemaChangeKind::ALL")` → **CONFIRMED (repro)**

Mechanically compared, not eyeballed: the help bytes against the registry's wire names.

```
ALL declared: 18   variants read: 18
wire names NOT in the emitted help: []
in-help order == ALL declaration order: True
```

### C-7 — `lead(codex, "validate --help derives its families from STORE_FAMILIES")` → **CONFIRMED (repro)**

```
STORE_FAMILIES count: 7
['doc↔code','workflow↔refs','file↔CLI-state','forward-ref integrity','schema-completeness',
 'mention integrity','schema-conformance']
missing from help: []     order matches: True
```
(The driver's **O-5** — the set is seven, not the Settle's five — is a *record* fact and is
unaffected: the product is correct and the help renders all seven.)

### C-8 — `lead(codex, "task finalize --help renders whats_left_coverage() rather than a second list")` → **CONFIRMED (repro)**

`gate_coverage.rs` carries five `Tier::Previewed` fragments and three `Tier::LaterSummary`
fragments; the emitted help reproduces both runs verbatim and in declaration order:

```
$ jigc task finalize --help | sed -n '3p'
`jigc task validate <id>` previews part of the finalize gate: the repository posture finalize
refuses under, this task's content findings, the carryover gate, the owner-artifact causes that
need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the
commit surface at finalize. …
```

### C-9 — `lead(codex, "doc show --help derives the committed JSON keys from WHOLE_DOC_KEYS and separately names STAGED_KEY")` → **CONFIRMED (repro)**

```
WHOLE_DOC_KEYS = [type, slug, item-count, schema-version, fields, sections]      (6)
$ jigc doc show --help | sed -n '5p'
… a whole-doc object keyed by `type`, `slug`, `item-count`, `schema-version`, `fields`, `sections`
— a staged serve adds the one `staged` key carrying the task id …
```

### C-10 — `lead(codex, "the installed SKILL.md body is built from both guides with include_str!, preceded by the conditional ownership text and stamped with version/body hash")` → **CONFIRMED (repro)**

Driven at `jigc setup` on a `bare` rig and read off disk:

```
$ wc -c .claude/skills/jigc/SKILL.md            →  39894
jigc-version: 1.0.0-rc.16
jigc-body-blake3: 70ba3764b3eafec753ed343d329cbb042ffb214f6040468aeab1d4203b7ee322
# and the composed-from-two-guides sentence is in the shipped bytes at line 10:
"It carries the two guides that ship with that binary, one after the other — the quickstart loop,
 then the migration field notes."
```

### C-11 — `lead(codex, "the M52 guide additions agree with their registries: task-area foreign bytes are the complement of TASK_AREA_FILES plus the staged-doc rule")` → **CONFIRMED (repro)**

Driven at the door the sentence is about, not read:

```
$ printf 'irreplaceable notes\n' > .jigc/tasks/u-four/notes.txt
$ jigc task finalize u-four                      # EXIT=0
note: the working area held 1 entry jigc did not write, and removing it would have destroyed bytes
      no commit has a copy of — they were moved aside, not taken:
    .jigc/tasks/u-four/notes.txt → .jigc/displaced/u-four/notes.txt
```

### C-12 — `lead(codex, "DESTROYING_DOORS has six disposition-bearing members")` → **CONFIRMED (repro)**

`crates/cli/src/milestone.rs`: `pub const DESTROYING_DOORS: [&DestroyingDoor; 6]` — **4**
`Disposition::Refuse{consent}` + **2** `Disposition::Displace` + **0** `Narrate`, the last of which
the enum's own doc-comment states in terms (*"No member holds this disposition today"*). This
matches the driver's §0 exactly.

### C-13 — `lead(codex, "schema-conformance.home-vacated is the SEVENTH store-exit member")` → **CONFIRMED in substance · ordinal REFUTED (datum)**

Confirmed: `STORE_EXIT_FLIPS` has **7** members and `home-vacated` is one of them, blocking and
exit-flipping — driven over all three removal states plus the route (§R.2, rows 35–38).

Refuted as an ordinal. Declaration order puts it at **position 6**, ahead of `foreign-squatter`,
and that position is a **deliberate decision the source states**:

```
$ awk '/pub const STORE_EXIT_FLIPS/,/^\];/' crates/cli/src/render.rs | grep -n 'id:'
5:  probe-unreliable   27: oob-rename   52: unmigrated-corpus   70: ahead-corpus
95: orphaned-instance  126: home-vacated  158: foreign-squatter
```
and the doc-comment immediately above the member: *"**A missing document outranks non-adoption**,
so this member precedes it."* The driver's *"member 6"* is right; "seventh" is only true read as
*seventh to be minted*. **No product consequence** — neither reading changes a byte.

### C-14 — `lead(codex, "the workflow help correctly describes suppressed.door and its workflow.verb-routed refusal")` → **CONFIRMED (repro)**

Driven at both compose doors, both output arms, both exit 1:

```
$ jigc start --workflow migrate-adr "x"                                   # EXIT=1
blocking · workflow.verb-routed — workflow `migrate-adr` is not composed by name: verb-routed …
  route: `jigc migrate <path> --as adr`
$ jigc workflow milestone-execution --preview                             # EXIT=1 (bare, redirected)
blocking · workflow.verb-routed — … composes a degenerate walk …
  route: `jigc milestone execute <milestone-id>`
$ jigc workflow milestone-execution --preview --format json               # EXIT=1
{"schema_version":3,"findings":[{… "code":"workflow.verb-routed",
  "key":{"code":"workflow.verb-routed","target":"workflow:milestone-execution"} …}]}
# and the pack side: 22 `suppressed:` blocks, 14 carrying `door:` — the driver's §0 counts, re-read.
```

### C-15 — `lead(codex, "COMMITTING_DOORS still contains exactly ten rows")` → **CONFIRMED** — folded into C-5.

### C-16 — `lead(codex, "I found no contradictory numeral, default, gate subject, exit-code claim, or 'never' assertion in the inspected help strings and guides")` → **REFUTED (datum)**

This is the source pass's strongest claim and it is **false on the axis's own subject**. The
falsifying datum is one `--help` output that states two different numerals about its own guard set,
and a driven run in which the smaller numeral's predicate is false:

```
$ jigc uninstall --help
  … Four states it refuses instead of destroying …
  … Any of them removes nothing until you re-run — or pass `--force`, which deletes all four …
  --force   Remove `.jigc/` even when it holds a fan-out worktree with content, an open task's
            staged docs, or a workbench file no index has a copy of … Inert when all three guards
            are already clean
$ jigc uninstall --help | grep -c 'Four states it refuses'             → 1
$ jigc uninstall --help | grep -c 'all three guards are already clean' → 1
```

and, in a state where **all three enumerated guards are clean** (`git worktree list | wc -l` → 1;
`jigc task list` → *no active tasks*; the `--force` narration itself carries neither the staged-docs
block nor the no-index-copy block):

```
$ jigc uninstall            # EXIT=1 — blocking · uninstall.foreign-bytes (.jigc/displaced/u-four/notes.txt)
$ jigc uninstall --force    # EXIT=0 — NOT inert
warning: removing the relocation workbench .jigc/displaced discards work that is not in git:
    .jigc/displaced/u-four/notes.txt
  note: the relocation workbench is the only copy of these bytes — they are not recoverable.
warning: removing `.jigc/` also removes 5 tracked file(s) under it: …
$ grep -c '^warning:' <that output>  → 2     # exactly the 4th population + the tracked set
$ test -e .jigc/displaced/u-four/notes.txt   → gone
```

That is defect **N-1**, driven independently by the reconciler. Codex's C-16 is refuted by it.

Second, narrower datum under the same claim, which the source pass also did not reach — the two
`Disposition::Displace` doors' help is silent about the displacement they perform (driver **O-7**):

```
$ jigc task finalize --help      | grep -icE 'displac|moved aside|parked'   → 0
$ jigc milestone finalize --help | grep -icE 'displac|moved aside|parked'   → 0
$ jigc uninstall --help          | grep -c  'displaced'                     → 1   # the control
```

### C-17 — `lead(codex, "zero schema-hash movement 577a0099..HEAD on both shipped manifests")` → **CONFIRMED (repro)**

```
$ git diff 577a0099..HEAD -- crates/cli/pack/config/schema-manifest.yaml \
                             packs/methodology/config/schema-manifest.yaml
(empty)
```

### C-18 — `lead(codex, "no adopter-doc/help door is omitted from the stated axis, and no generated-help or installed-guide seam is bypassed")` → **CONFIRMED as scoped**

Taken as the completeness question the source pass was asked (*is there a door the axis's set does
not carry?*), this holds: the driver derived its door set from M52's own edit set and reached 28
clap leaves, the four EC-11 generators were each compared to their registry (C-6…C-9), and the
guide seam was read out of the installed artifact (C-10). Recorded with its limit, because the two
are easy to conflate: **C-18 says nothing about whether the sentences are true.** C-16 was the
claim that said that, and C-16 is refuted.

## R.2 · Driver defects → status after re-driving

| driver defect | status | how the reconciler settled it |
|---|---|---|
| **N-1** — `jigc uninstall --help` states *"Four states it refuses"* / *"deletes all four"* in the about and *"Inert when all three guards are already clean"* over a three-population enumeration on the `--force` flag, and `--force` is driven **not** inert in exactly that state | **CONFIRMED · origin driver · severity LOW** | Re-driven end to end from a fresh `committed-singletons` rig (repro in C-16). Reproduced exactly: exit 1 on the refusal, exit 0 on `--force`, `.jigc/displaced/u-four/notes.txt` destroyed and named, **2** warning blocks — proving guards 2 and 3 clean. Codex's C-16 asserted no such contradiction exists; **that source claim is refuted by this repro**, not the other way round. The sibling contrast also re-read: `jigc milestone discard --help` names all three of *its* guards and says *"the explicit consent for all three guards"*, matching its own about — so N-1 is an **incomplete sweep of one flag help**, not a design choice. |
| **N-2** — every retitle-without-reslug at `jigc rename` acks and commits a no-op (`X -> X`) while the title — the only thing that moved, and the thing the refusal's route promised — is on neither the ack nor the commit subject | **CONFIRMED · origin driver · severity LOW** | Re-driven on a fresh rig, and the **class boundary sharpened with a control the driver did not run**. Fixed-identity arm and slugged-explicit-same-slug arm both reproduce; the *slug-moving* arm is informative, which is what makes the silence a defect rather than a house style. Codex is **silent** on N-2 — no refutation to record. |

**N-2, re-driven, with the new control as the last block:**

```
rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"; cd "$REPO"

$ jigc rename vision:vision --to "New Vision"                              # EXIT=1
blocking · write.identity-change — cannot reslug `vision:vision` — this doctype's identity is fixed
  to its type … only a retitle is supported
  route: `jigc rename vision:vision --to 'New Vision' --slug vision` keeps the identity that cannot
         move and rewrites only the title

$ jigc rename vision:vision --to 'New Vision' --slug vision                # the route, verbatim — EXIT=0
renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)
$ grep -n '^# ' VISION.md          → 5:# New Vision            # the title DID move
$ git log --oneline -1             → 7a36848 rename VISION.md -> VISION.md
$ git show --stat --oneline HEAD   → VISION.md | 2 +-          # one real line changed

$ jigc rename vision:vision --to 'New Vision' --slug vision                # the genuine no-op — EXIT=0
no-op: vision:vision already holds the title "New Vision" at VISION.md — nothing renamed, nothing committed
                                                            # HEAD unmoved — the FAILURE ack is the informative one

# the class, on a slugged doctype with an explicit matching --slug:
$ jigc rename adr:alpha-choice --to "Alpha Choice Revisited" --slug alpha-choice          # EXIT=0
renamed adr:alpha-choice -> adr:alpha-choice (docs/decisions/alpha-choice.md -> docs/decisions/alpha-choice.md)
$ git log --oneline -1  → e8faa8b rename docs/decisions/alpha-choice.md -> docs/decisions/alpha-choice.md

# NEW (reconciler's control) — when the slug DOES move, both surfaces are informative:
$ jigc rename adr:alpha-choice --to "Alpha Choice Third"                                  # EXIT=0
renamed adr:alpha-choice -> adr:alpha-choice-third (docs/decisions/alpha-choice.md -> docs/decisions/alpha-choice-third.md)
$ git log --oneline -1  → 8c153da rename docs/decisions/alpha-choice.md -> docs/decisions/alpha-choice-third.md
```

The control fixes the axis precisely: **the ack and the commit subject are a function of the path,
not of what changed** — so they are informative exactly when the slug moves and degenerate exactly
when it does not, which is the whole retitle-without-reslug set (`placement ‖ singleton`, where
`--slug <same>` is the only legal form, plus every explicit same-slug retitle on a slugged doctype).

## R.3 · Driver observations → status

| obs | status after the re-run | the reconciler's datum |
|---|---|---|
| **O-1** — `setup`'s *"every path in its own install footprint"* universal vs the excluded `.git/hooks/pre-commit` | **carried, STILL-OPEN** | Not re-driven (driver's row 22 stands, bound declared at `install_candidate_paths`). Prose precision, no byte at risk. |
| **O-2** — `jigc task list --format json` carries no `schema_version` | **carried, re-driven** | `$ jigc task list --format json` → a bare top-level `[` array (non-empty, one live task), no envelope key. |
| **O-3** — MIGRATING item 4 understates `jigc rename` | **carried, STILL-OPEN** | Driver row 63 stands; the shipped behaviour is a *stricter* refusal than the sentence implies. |
| **O-4** — the generated-help set is now **13**, named by no record | **carried, superseded** | Re-counted independently: 13 `*_long_about()` generators against M51's four. |
| **O-5** — `STORE_FAMILIES` is seven, not the Settle's five | **carried, record-only** | Re-driven under C-7: seven, and the help renders all seven in order. |
| **O-6** — bare `jigc start` / `task list` answer *"isn't set up"* / *"no active tasks"* at exit 0 over an unreadable `.jigc/` | **carried, STILL-OPEN** | Not re-driven. The driver's own note is the load-bearing half and is kept: the bound is declared **only** in a doc-comment and a test, and is **not** in M52's VERDICT *Declared bounds*. |
| **O-7** — neither `Disposition::Displace` door's `--help` mentions the displacement | **carried, re-driven** | `task finalize --help` and `milestone finalize --help` → 0 displacement words each; `uninstall --help` → 1 (the control). Folded into C-16's second datum. Not graded: the run narrates every `from → to` on stderr and on the envelope, and the guide carries the paragraph. |
| **O-8 (new, reconciler)** — a hook-rejected `milestone create` leaves its two parent directories behind, empty | **new observation, not graded** | See §R.5, correction 1. |

## R.4 · Demotion scan — rows marked driven that carry no repro record

All 72 table rows were scanned mechanically for a missing argv or a missing exit. **Ten** rows
carry a `"the same …"` / `"that route …"` back-reference to the preceding row's literal argv rather
than repeating it: rows **8, 19, 27, 36, 37, 40, 45, 64, 66, 67**. That is the table's idiom, not a
gap — each still carries its own exit, code, route kind, surface and verdict, and the argv it
inherits is literal in the row above it. To settle it rather than assert it, the reconciler
**re-drove eight of the ten** (8, 19, 27, 36, 37, 40, 45, 64 — plus 66 and 67 inside the N-2
repro, so in fact all ten). **Every one reproduced.** The two rows whose *exit* cell reads `—`
(19 and 45) are an artifact read and a log read respectively, not door invocations; both were
re-performed and both hold.

**Demotions: none.**

Two further scoping notes that are not demotions:

- **Row 11** is one table row covering nine verbs' `--help`. All nine were re-driven individually
  by the reconciler; the row stands as written.
- **Row 19**'s "argv driven" is a `grep`, so it adds no door of its own — it is a C3 assertion over
  the artifact row 18's driven `jigc setup` produced, and is counted under `setup`.

## R.5 · Datum corrections to rows that otherwise hold

Three, none of which changes a verdict; each carries the falsifying datum.

1. **Row 46's parenthetical.** The row asserts that after a hook-rejected `jigc milestone create`,
   *"`.jigc/milestones` and `docs/milestone-records` both absent"*. Re-driven, the **surface claim
   holds** and the **parenthetical does not** — both parent directories are **present and empty**:

   ```
   $ jigc milestone create 'M9 rejected'                    # EXIT=1
   `git commit` was rejected (no commit was made):  policy: nope
   nothing was committed — the record write and the milestone workbench were both rolled back, so
   nothing of milestone:m9-rejected survives. Fix the hook's complaint, then re-run
   `jigc milestone create 'M9 rejected'`.
   $ find .jigc/milestones docs/milestone-records      →  .jigc/milestones
                                                          docs/milestone-records     (both EMPTY)
   $ find . -path ./.git -prune -o -name '*m9*' -print →  (nothing)
   $ git status --porcelain                            →  only an unrelated untracked file
   $ git log --oneline -1                              →  HEAD unmoved
   ```

   The frame's actual sentence — *"nothing of milestone:m9-rejected survives"* — is **true**: no
   `m9` byte survives and `HEAD` is unmoved. What survives is two empty parent directories git
   cannot track and `git status` never shows. Recorded as **O-8** (new observation, not graded).

2. **Row 50's numeral.** The row reads *"33 entries; 12 catalog, 22 off-catalog, 22 carrying a
   catalog reason — 0 without"*, which does not add up (12 + 22 = 34). Re-driven, the entry count
   is **34**, not 33; every other number in the row is exactly right:

   ```
   $ jigc describe --workflows    # EXIT=0
   distinct entries: 34 · router catalog: 12 · entries not in catalog: 22
   "It is hidden from the router catalog:" occurrences: 22   →  22 of 22 carry a reason, 0 without
   ```

   The likely cause is an instrument artefact, recorded so it is not mistaken for a product fact:
   the first entry (`architecture-documentation`) is emitted **glued to the preamble paragraph**
   rather than starting its own line, so a line-anchored count of entries drops it. The row's
   substantive claim — *the catalog stops offering a route that refuses, and every off-catalog
   workflow carries its reason* — is **confirmed** (and row 49's companion claim re-driven: the
   catalog lists 12, and **0** of the 14 `suppressed.door` members appear in it).

3. **The driver's §3/§4 cross-reference.** §2 closes *"the repro blocks … are in §4"*; §4 is
   *Observations* and contains no repro block. The repro blocks are in **§3**. A pointer error,
   corrected here rather than in the driver table, which Part 1 keeps unchanged.

## R.5b · What the reconciler did NOT drive

Stated plainly, so a gap is visible rather than implied. The driver's §5 list is carried forward
unchanged (all seven items), and the reconciler additionally did not re-drive: row 22 (`setup` over
a foreign `pre-commit`), row 24 (`jigc upgrade`), rows 25–26 (the `--git-state merge` posture pair),
rows 41–43 (the `unadopted-instance` / `migrate-corpus` triage pair), rows 51–58 and 60–62. Those
rows are the driver's, carry their own argv and exit, and no Codex claim contested them — under the
reconciliation rule a row neither pass disputes needs no third drive.

## R.6 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling (the registry read
at HEAD: `crates/cli/src/cli.rs`, **47** leaves). Verbs used only to build a fixture — `doc create`,
`doc set-slot`, `doc set-field` — are **not** counted; a fixture builder is not the door of a row
(M51's rule, kept). The reconciler's own re-drives introduced **no new door**.

**28 doors:**

`start` · `workflow` · `setup` · `uninstall` · `upgrade` · `ingest` · `migrate` · `migrate-corpus` ·
`rename` · `validate` · `describe` · `doc show` · `doc schema` · `doc list` · `doc rename` ·
`task list` · `task validate` · `task discard` · `task finalize` · `task bind` · `config set` ·
`milestone create` · `milestone add-task` · `milestone add-from-spec` · `milestone list-tasks` ·
`milestone provision` · `milestone finalize` · `milestone discard`

**`milestone join` is explicitly NOT covered** — no row of either pass drove it, and it is named
here only to say so. The remaining 18 `VERB_KINDS` leaves (`unmanage`, `relocate`, `doc add-item`,
`doc remove-item`, `doc retitle-item`, `doc author`, `doc create`, `doc set-field`, `doc set-slot`,
`task diff`, `config insert-step`/`replace-step`/`remove-step`/`fill`/`fork`/`get`/`list`,
`milestone execute`) carry no axis-8 sentence in M52's edit set and are the subjects of other axes.

---

## R.7 · Summary

| | count | |
|---|---|---|
| Codex claims entered as leads | **18** | 5 M51 dispositions · 12 consistent-read · 1 completeness |
| → CONFIRMED with a repro | **16** | C-1…C-12, C-14, C-15, C-17, C-18 |
| → REFUTED with a datum | **1** | **C-16** (falsified by N-1's live repro, and by O-7) |
| → CONFIRMED in substance, ordinal refuted | **1** | **C-13** (`home-vacated` is member **6** by declaration order, deliberately ahead of `foreign-squatter`) |
| → OPEN LEAD | **0** | every Codex claim was drivable on this binary |
| Driver defects | **2** | both **CONFIRMED**, both re-driven by the reconciler, both LOW |
| Driver rows demoted | **0** | all ten back-reference rows re-driven; all reproduced |
| Datum corrections | **3** | row 46's parenthetical · row 50's entry count · the §3/§4 pointer |
| New observations | **1** | **O-8** — empty parent dirs survive a rolled-back `milestone create` |

**The one contested point between the two passes is C-16 vs N-1, and it is settled by driving.**
The source pass read the help strings and reported no contradictory numeral; the binary prints
*"Four states it refuses"* and *"Inert when all three guards are already clean"* in one output, and
`--force` in the state the smaller numeral describes is not inert — it deletes the fourth
population and says so. A source read cannot see a predicate that is false only when run, which is
the reconciliation rule's whole warrant and the reason this row is a finding rather than a lead.
