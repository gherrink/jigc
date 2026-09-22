<!-- M53 PARTIAL per-axis review · axis 3 · RECONCILED (Opus driver table + reconciliation ledger) — every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.17`, repo HEAD `75ab77ca`, 2026-09-22. -->

# M53 partial per-axis review — AXIS 3 · destroying doors — the Opus driver's `(door, cell)` table

**The binary, asserted first.**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.17
```

Release posture: the debug-only `debug_assert!` route fences do **not** exist in this build, so a
route-fence violation would surface here as a bad emitted command, never as a panic — the posture an
adopter's binary is in. Every row below ran on that binary, in throwaway rigs built with
`dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc` (two-step eval; every root from
`mktemp -d`, so nothing needed teardown and the `rm -rf $V/$D` shape never appears). **Nothing was
written into the working repository, no fix was applied, no commit was made.** I did **not** read the
Codex source pass for this axis.

**Instrument note, applied throughout.** This harness's `grep` is a shell function honouring
`.gitignore`, and the whole axis lives under a gitignored `.jigc/`. Every loss/survival claim below
uses **`command grep`** with a **before-control** that finds the plant before the drive — a scan that
finds nothing before proves nothing after. Where a count is quoted, the before-count is quoted beside
it. **Exit codes are read bare**, never through a pipe: one drive in this session (`milestone
provision` over a terminal milestone) reported `exit=0` through `| head -8` and `exit=1` bare, and the
bare reading is the one recorded.

**What I grade against rather than re-find** (M53 [VERDICT](../../../../completions/artifacts/M53/VERDICT.md)
→ *Declared bounds*, and the settle record's D1/D2 bounds):

- the new `finalize.foreign-bytes` advisory is **stderr-only at `milestone finalize`** until 2.0 (the
  landed arm's envelope is pinned at `Object(["committed"])`);
- the 21-render `DEBUG_REMAINDER` in `cli/milestone.rs`, and `remove_worktrees`' two
  **`DeclaredAbsolute`** host-path warnings (the `git worktree remove --force <path>` remedy resolves
  against the caller's cwd);
- `reseed_sub_task_areas`' `.exists()` skip and `staged_task_prose`'s own `read_dir` (the deliberate
  keep-too-much rule);
- a foreign **directory** inside `merged/` moves **whole**, never merged; a milestone id and a task id
  can collide under one `.jigc/displaced/<id>/` (suffix-resolved, no loss);
- `task discard` over an area a landed finalize left standing refuses under the shipped
  `task-discard.staged-prose`;
- no rollback population is raced by a genuine concurrent process (headless by construction).

---

## 1 · The door set and the cell set, derived from the code

Counted by a balanced-bracket top-level-item parse of each declaration at `HEAD = 75ab77ca`
(comments stripped). **The numbers below are what I read, not what a design doc states.**

| registry | file:symbol | count read |
|---|---|---|
| **`DESTROYING_DOORS`** | `crates/cli/src/milestone.rs:3338` | **6** (`[&DestroyingDoor; 6]`) |
| `WORKTREE_DOORS` | `crates/cli/src/milestone.rs:3355` | **4** |
| `Disposition` | `crates/cli/src/milestone.rs:3112` | **3** variants — `Refuse{consent}` · `Narrate` (**no member holds it**) · `Displace` |
| `LEFTOVER_VERDICTS` | `crates/cli/src/milestone.rs:3099` | **3** |
| `LeftoverShape` | `crates/cli/src/milestone.rs:3399` | **3** variants |
| `TASK_AREA_FILES` | `crates/engine/src/state.rs:163` | **14** (13 at rc.16 + `FINALIZE_MESSAGE_FILE`, M53 audit fix 1) |
| `MILESTONE_AREA_FILES` | `crates/engine/src/state.rs:222` | **6** (5 + `FINALIZE_MESSAGE_FILE`) |
| `TASK_DOCS_FILES` | `crates/engine/src/state.rs:195` | **1** |
| `MINT_DOORS` | `crates/engine/src/state.rs` | **5** |
| `InProgress::ALL` | `crates/cli/src/repo.rs:272` | **10** (the tenth is M53 D4's `UncommittedCherryPick`) |

Other registries read as instructed (none is axis 3's subject, so each is a count and the method, not a
driven row): `VERB_KINDS` **47** · `BEHALF_DOORS` **47** · `PATH_ARG_OCCURRENCES` **14** ·
`DOCTYPE_DOORS` **16** · `SLUG_DOORS` **6** · `WORK_UNIT_ID_DOORS` **25** · `COMMITTING_DOORS` **10** ·
`ENVELOPE_ARMS` **64** · `STORE_EXIT_FLIPS` **7** · `ENVELOPE_OWED_CODES` **4** ·
`SchemaChangeKind::ALL` **18**.

**The rc.16 → rc.17 door-set diff.** The door set is **unchanged at 6**; `Narrate` still has **no**
member. What moved inside the axis is the *subject*: both area registries gained
`FINALIZE_MESSAGE_FILE`, `foreign_area_paths`/`unwind_area`'s milestone arm gained the **`merged/`
walk** (two levels, `staged_doc_id` alone inside `merged/docs/`), the two `Displace` doors gained
`finalize.foreign-bytes` while keeping `codes: &[]`, and `engine::state::carries_base_pin` became the
predicate that decides whether a directory under `.jigc/tasks/` or `.jigc/milestones/` is a work unit
at all.

**The six doors, with the identities each carries** (read from the code, then driven):

| door (VERB_KINDS spelling) | disposition | `codes` |
|---|---|---|
| `milestone provision` | `Refuse{--force}` | `milestone.leftover-holds-work` |
| `milestone discard` | `Refuse{--force}` | `milestone.dirty-worktree` · `milestone.staged-prose` · `milestone.foreign-bytes` |
| `uninstall` | `Refuse{--force}` | `uninstall.dirty-worktree` · `uninstall.staged-prose` · `uninstall.foreign-bytes` · `uninstall.untracked-workbench-file` |
| `task discard` | `Refuse{--force}` | `task-discard.staged-prose` · `task-discard.foreign-bytes` |
| `task finalize` | **`Displace`** | `&[]` (advisory `finalize.foreign-bytes` on `findings`) |
| `milestone finalize` | **`Displace`** | `&[]` (advisory `finalize.foreign-bytes`, stderr-only — declared) |

**The cell set driven.** M51's `LeftoverShape × {staged prose · untracked workbench file · clean} ×
{no --force, --force}`, expanded on the axes M52 minted (S12–S15) and the three M53 minted:

- **S1** `Directory`/`NoOwnLinkage` non-empty · **S2** `Directory`/`Unverifiable` (dangling `.git`) ·
  **S3** own **dirty** worktree · **S4** own **clean** worktree · **S5** `File` (and a symlink — a
  `File` whatever it points at) · **S6** `Unreadable` (`chmod 000`) · **S7** staged prose · **S8**
  untracked workbench file · **S9** clean · **S10** the fail-closed *probe* cells · **S11** the
  removal-fails cells · **S12** the `TASK_AREA_FILES` complement in a **task** area · **S13** the
  complement in the **milestone** area · **S14** displacement-destination-unusable · **S15**
  `.jigc/displaced/` non-empty, and a plain file at an `ENTRIES` name
- **S16 (new, M53 D1)** the complement **under `merged/`** — `merged/` top · `merged/docs/` beside a
  materialized body · `merged/docs/provenance.json` · `merged/docs/<colon-less>.md` ·
  `merged/<dir>/**` · `merged/docs/<ty>:<slug>.md`
- **S17 (new, M53 D2)** the move-outcome axis — `{all move · some move · none move}` × `{unwind ok ·
  unwind faults}`, and the hook-writes-during-the-commit cell
- **S18 (new, M53 D3)** the **residual** — a directory carrying no base pin — × `{plain id · sub-task
  of a joined milestone}`, at the by-id, enumerating and mint doors

---

## 2 · The table — 78 driven rows over 13 doors

Route kind read off the rendered finding: `Mechanical` = the `route:` opens with a backticked argv ·
`Human` = prose naming the acts · `Informational` = a `note:`/`warning:` with neither · `none` = no
finding.

### 2.1 · `milestone provision` (11 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 1 | S1 dir, no `--force` | `jigc milestone provision axis-three-probe` | 1 | `milestone.leftover-holds-work` | Mechanical | `…: precious.txt — git reports no worktree of its own there`; plant on disk after | matches contract |
| 2 | S2 dangling `.git` | same | 1 | same | Mechanical | `…: .git, wip.txt` | matches contract |
| 3 | S5 plain file | same | 1 | same | Mechanical | `…: the file itself — it is a file, not a worktree` | matches contract |
| 4 | S5 symlink | same | 1 | same | Mechanical | identical *the file itself* wording | matches contract |
| 5 | S6 unreadable | same | 1 | same | Mechanical | `…: unknown — could not read the leftover directory …: Permission denied (os error 13)` | matches contract |
| 6 | S1, `--force` | `… --force` | 0 | none | none | `warning: removing the leftover directory … precious.txt` + `not recoverable`, then `provisioned 2 worktree(s)` | matches contract |
| 7 | S5 symlink → tree **outside** the repo, `--force` | `… --force` | 0 | none | none | `warning: removing the leftover file …`; outside tree **2/2 bytes intact** | matches contract |
| 8 | S11 removal fails (read-only **parent**), `--force` | `… --force` | 1 | `milestone.provision-failed` | Mechanical | loss warning names `precious.txt`, which **is** gone; `0 of 2 worktree(s) landed` | matches contract (outcome-keyed) |
| 9 | S11 removal fails (read-only **leftover**), `--force` | `… --force` | 1 | `milestone.provision-failed` | Mechanical | **no loss warning at all**; `precious.txt` survives | matches contract |
| 10 | refusal, `--format json` | `… --format json` | 1 | flattened | Human (in message) | stderr `{"error": …}`, stdout 0 B | matches contract (declared flattened, worked-examples.md:3680) |
| 11 | malformed / empty id | `jigc milestone provision ""` · `"../.."` | 1 | `work-unit.malformed-id` | Human | `.jigc/` intact | matches contract |

### 2.2 · `milestone discard` (12 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 12 | S3 dirty worktree (precedence over S7/S12/S13) | `jigc milestone discard axis-three-probe` | 1 | `milestone.dirty-worktree` | Human | worktree arm wins; both worktrees named with their `git status` codes | matches contract |
| 13 | S7 staged prose on a **sub-task** | same | 1 | `milestone.staged-prose` | Human | `first-sub: commit:first-sub`; route names `jigc milestone finalize axis-three-probe` | matches contract (**D-1 CLOSED**) |
| 14 | **S16** complement under `merged/` (6 loci) + S12 + S13, no `--force` | same | 1 | `milestone.foreign-bytes` | Human | **all 7** named — `merged/docs/adr.md`, `…/deep.txt`, `…/provenance.json`, `merged/sub` (whole), `merged/top.txt`, `mnotes.txt`, `tasks/first-sub/tnotes.txt`; before=7 after=7 | matches contract (**A3-1 cell (i) CLOSED**) |
| 15 | same, `--force` | `… --force` | 0 | none | Informational | **two** `warning: removing the working area …` blocks, one per area, each naming its own paths + `not recoverable`; ack `workbench removed` | matches contract (§3 `Take`) |
| 16 | S2 unverifiable **unregistered** leftover, no `--force` | same | 0 | none | none | exit 0; the stray survives byte-intact and is not this milestone's subject | matches contract |
| 17 | S9 clean | same | 0 | none | none | `discarded milestone:… (2 sub-task(s); workbench removed)` | matches contract |
| 18 | **S11 teardown fails** (`chmod 500 .jigc/worktrees`), `--force` | `… --force` | 0 | none (2 warnings) | Informational (`remedy:` ×2) | ack reads `workbench **NOT fully removed** — the warnings above name what is left`; both worktrees on disk and named | matches contract (**D-2 CLOSED**) |
| 19 | S10 fail-closed (`chmod 000` a sub-task `docs/`) | same | 1 | `milestone.foreign-bytes` | Human | `cannot check … Permission denied`; **no host path**; route names both dirs repo-relatively and `--force` | matches contract (**D-3 CLOSED**) |
| 20 | refusal, `--format json` | `… --format json` | 1 | `milestone.foreign-bytes` | Human | stderr `{findings, schema_version}`, stdout 0 B | matches contract (moved to the envelope since rc.16) |
| 21 | malformed / empty id | `jigc milestone discard ""` · `"../.."` | 1 | `work-unit.malformed-id` | Human | repo intact | matches contract |
| 22 | **S18** terminal milestone | `jigc milestone discard axis-three-probe` after a landed boundary | 1 | `milestone.terminal` | Human | `is \`joined\` — a settled milestone is over and has no workbench` | matches contract |
| 23 | **control** — only `finalize-message.tmp` planted in both areas | same | 0 | none | none | `workbench removed`; **`foreign-bytes` does not fire** over jigc's own transient | matches contract (M53 audit fix 1) |

### 2.3 · `uninstall` (12 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 24 | S3 dirty worktree (precedence 1 of 4) | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | Human | route names `milestone discard <id> --force` **and** `uninstall --force` | matches contract |
| 25 | S7 staged prose on a sub-task | same | 1 | `uninstall.staged-prose` | Human | `first-sub: commit:first-sub — a sub-task of milestone …, which \`jigc task finalize\` refuses` | matches contract (**D-1 CLOSED**) |
| 26 | **S16** the `merged/` complement + S12 | same | 1 | `uninstall.foreign-bytes` | Human | all 7 named, identical set to row 14 | matches contract |
| 27 | S8 untracked workbench file | same | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/notes.txt`; route names `git add <path>` | matches contract |
| 28 | S15 plain **file** at an `ENTRIES` name (`.jigc/logs`) | same | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/logs` named | matches contract |
| 29 | S15 `.jigc/tasks` is a plain **file** | same | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/tasks` named (the `workbench_paths` subject, not an area) | matches contract |
| 30 | S15 non-empty `.jigc/displaced/` | same | 1 | `uninstall.foreign-bytes` | Human | `.jigc/displaced/some-task/keep.txt` | matches contract |
| 31 | S15, `--force` | `… --force` | 0 | none | Informational | `warning: removing the relocation workbench .jigc/displaced …` names the byte; then the tracked-file block | matches contract |
| 32 | S10 fail-closed **worktrees root** (`chmod 000 .jigc/worktrees`) | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | Human | route: *this failure is a `read_dir` of that directory, **not a git fault*** + `--force` | matches contract (**D-4 CLOSED**) |
| 33 | S9 clean, `--format json` | `… --format json` | 0 | none | none | stdout `{allowlist_file, findings, line_file, removed}` | matches contract |
| 34 | S9 second run (idempotency) | `jigc uninstall` | 0 | none | none | `(nothing to remove — no repo-local jigc install was present)` | matches contract |
| 35 | symlink inside a task area → outside tree, `--force` | `… --force` | 0 | none | Informational | outside tree byte-intact after | matches contract |

### 2.4 · `task discard` (10 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 36 | **S12** the whole complement shape space (8 shapes) | `jigc task discard tidy-the-readme` | 1 | `task-discard.foreign-bytes` | Human | **all 8** named: dotfile · symlink at an address-shaped name · non-`.md` under `docs/` · un-staged `docs/*.md` · foreign root file · symlink · nested dir · `.md` outside `docs/` | matches contract |
| 37 | S12, `--force` | `… --force` | 0 | none | Informational | `warning: removing the working area … discards work that is not in git:` naming all 8 + `not recoverable`, then the ack | matches contract — **see F-2** |
| 38 | S7 staged prose, ordinary task | `jigc task discard tidy-the-readme` | 1 | `task-discard.staged-prose` | Human | route names `jigc task finalize <id> (which refuses while a required slot is empty)` | matches contract |
| 39 | S7 on a **milestone sub-task** | `jigc task discard first-sub` | 1 | `task-discard.staged-prose` | Human | route names `jigc milestone finalize <m>` and says *so `jigc task finalize` refuses it* | matches contract (**D-1 CLOSED**) |
| 40 | S10 fail-closed (`chmod 000 …/docs`) | `jigc task discard tidy-the-readme` | 1 | `task-discard.foreign-bytes` | Human | **no host path** (0 hits for `/var/folders`); route repo-relative + `--force` | matches contract (**D-3 CLOSED**) |
| 41 | **control** — jigc's own files only | same | 1 | `task-discard.staged-prose` | Human | `foreign-bytes` **does not fire** over the 14-member set | matches contract (zero false fire) |
| 42 | **control** — only `finalize-message.tmp` | same | 1 | `task-discard.staged-prose` only | Human | the transient is jigc's; not named foreign | matches contract (M53 audit fix 1) |
| 43 | malformed / empty id | `jigc task discard ""` · `"../.."` | 1 | `work-unit.malformed-id` | Human | `.jigc/` intact (`AGENT.md config milestones state tasks version`) | matches contract |
| 44 | **S18** residual (bare `mkdir` under a joined milestone) | `jigc task discard first-sub` | 1 | `finalize.no-task` | Human | residual sentence + repo-relative route; **HEAD byte-unmoved**, no record commit | matches contract (**D2.6 CLOSED**) |
| 45 | **S18′** a **whole** leftover area of a settled sub-task (pin intact) | `jigc task discard second-sub` | 1 | `milestone.terminal` at target **`task:second-sub`** | Human | *sub-task `second-sub` … is already `joined` on the committed record — its working area is a leftover, not live work*; HEAD unmoved | matches contract (§8 CLOSED) |

### 2.5 · `task finalize` — `Displace` (11 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 46 | **S17 all move** — complement present | `jigc task finalize tidy-the-readme` | 0 | none | Informational | `note: … moved aside, not taken:` one `<from> → <to>` per entry; `committed.displaced` carries the pairs | matches contract |
| 47 | **S17 none move**, cell (a): `.jigc/displaced` is a regular **file** | `… --format json` | 0 | `finalize.foreign-bytes` (advisory, on `findings`) | Human | area **left standing**; before=1 after=1; message names the path, the count, the failing reason (`Not a directory (os error 20)`) | matches contract (**A3-2 CLOSED**) |
| 48 | **S17 none move**, cell (b): read-only `.jigc/displaced/` | `jigc task finalize tidy-the-readme` | 0 | `finalize.foreign-bytes` | Human | both plants left standing, both named, `Permission denied (os error 13)`; before=2 after=2 | matches contract |
| 49 | **S17 some move** — parking home occupied at one entry's parent | `… --format json` | 0 | `finalize.foreign-bytes` | Human | one narration block holding **both** halves: `held 2 entries` (the **complement's** count), `1 of them moved aside`, `and 1 could not be moved` with `File exists (os error 17)`; `committed.displaced` carries the moved pair **alone** | matches contract (§2 count rule) |
| 50 | the **hook writes into the area during the commit** | finalize with a `pre-commit` hook writing `hookfile.txt` into the area | 0 | none | Informational | the hook's byte is **displaced**, the area removed clean | matches contract — see O-4 |
| 51 | commit **rejected** by a hook, S12 present | finalize with a rejecting `pre-commit` | 1 | (rejection frame) | Human | *nothing was committed … your task's staged docs are still in `.jigc/tasks/<id>/docs/`*; before=1 after=1; `.jigc/displaced` **not created** | matches contract |
| 52 | S12 **collision** (same id displaced twice) | finalize twice | 0 | none | Informational | second lands `notes.txt.2`; first copy reads `FIRST-COPY` unchanged | matches contract |
| 53 | **control** — ordinary lifecycle, no plant | `… --format json` | 0 | none | none | `displaced: []`, `findings: []`, `.jigc/displaced` **does not exist**, area gone | matches contract (§13 zero-false-fire) |
| 54 | **S18** residual | `jigc task finalize first-sub` | 1 | `finalize.no-task` | Human | residual sentence, no host path | matches contract |
| 55 | a **symlinked** staged body (`docs/adr:outside-secret.md` → a tree outside the repo) | `jigc task finalize link-probe` | 0 | none | Informational | **not promoted** (`1 file committed`; `git grep -l OUTSIDE-SECRET-BYTES HEAD` → rc 1) — **displaced** instead | matches contract — the promotion plan is shape-aware (see **F-1**) |
| 56 | a user file named `docs/<ty>:<slug>.md` jigc did not stage | `jigc task finalize tidy-the-readme` | **3** | `schema-conformance.*` | Mechanical | the boundary **blocks** rather than taking it; before=1 after=1 | matches contract (keep-too-much, task branch) |

### 2.6 · `milestone finalize` — `Displace` (13 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|
| 57 | **S13 + S16**, the headline re-drive: 6 loci under `merged/` + area root + nested dir + a sub-task area | `jigc milestone finalize axis-three-probe --format json` | 0 | none | Informational | **8/8 planted bytes survive** at `.jigc/displaced/<unit>/<relative>`; `committed.displaced` carries all 8 pairs sorted by `from`; two `note:` blocks, one per area | **A3-1 CLOSED** |
| 58 | the same, stderr channel | same | 0 | none | Informational | `note: the working area held 7 entries … moved aside, not taken:` + a second block `held 1 entry` for `first-sub` | matches contract |
| 59 | **S16 foreign directory** `merged/sub/` | same | 0 | none | Informational | moved **whole** as one entry (`merged/sub → .jigc/displaced/<m>/merged/sub`), `nested.txt` inside it intact | matches contract (declared unit rule) |
| 60 | **§1's cell** — a colon-less `merged/docs/adr.md` | same | 0 | none | Informational | **not** resolved to doctype `adr`, **not** copied into the gate area, **no** `schema-conformance.unknown-type`; displaced at phase 7 | matches contract (`split_once(':')`) |
| 61 | **§2's cell** — `merged/docs/provenance.json` | same | 0 | none | Informational | called a third party's and displaced, **not** inherited from `TASK_DOCS_FILES` | matches contract |
| 62 | **blocked boundary (exit 3)** with 6 plants | `jigc milestone finalize axis-three-probe` over unauthored commit docs, `squash: false` | **3** | `schema-conformance.field-value-conformant` · `…required-slot-present` | Mechanical | all 6 plants **on disk**, `.jigc/displaced` **not created**, and the boundary is **not blocked by them** | matches contract |
| 63 | the **re-run** after authoring | same | 0 | none | Informational | `materialize`'s clear is **selective**: the stale `commit:*.md` bodies go, all 6 plants are displaced; 3 commits (`chore: add first-sub work`, `chore: add second-sub work`, the boundary) | matches contract (D1.3 + §1) |
| 64 | **D1×D2** — a `merged/` byte whose **move failed** survives a landed boundary | finalize with `.jigc/displaced` a regular file | 0 | `finalize.foreign-bytes` ×2 (stderr-only) | Human | **3/3 plants on disk**; one advisory **per standing area**, each keyed at its own unit (`milestone:axis-three-probe`, `task:first-sub`); envelope top-level keys = `['committed']`, `displaced: []` | **A3-2 CLOSED at the second door** |
| 65 | worktree narration incl. the `--ignored` axis | finalize with an unstaged file, a `.gitignore` and an ignored `build/` | 0 | none | Informational | `warning: removing the fan-out worktree …` naming `.gitignore (never staged)`, `build/ (ignored by git)`, `scratch.txt (never staged)`; `sub_tasks[].discarded` carries the same three with `state` | matches contract (M46's *visible, not prevented*) |
| 66 | S1+S5 **unregistered** leftovers under `.jigc/worktrees/` | same | 0 | none | none | both survive byte-intact and are not named | matches contract (declared unreachable shape) |
| 67 | nothing to land | same | **3** | `milestone.zero-contribution` | Mechanical | names `provision` + `discard` as the two exits | matches contract |
| 68 | **control** — ordinary post-join area incl. `merged/` | `… --format json` | 0 | none | none | `displaced: []`, zero `foreign-bytes`, `.jigc/displaced` **does not exist**, both areas gone | matches contract (§13, the D1.4 half) |
| 69 | a pin-less **milestone** area while the record is live | `milestone list-tasks` · `execute` · `finalize` · `discard` | 0/0/0/1 | — / — / none / `milestone.terminal` | — | the boundary lands and displaces the plant; nothing bricks | matches contract |

### 2.7 · the residual rule’s other doors, and the surfaces a finding reached — 7 doors, 9 rows

| # | door | cell | argv | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 70 | `task list` | **S18** residual present | `jigc task list` | 0 | none | none | `no active tasks` — the residual is **not** listed | matches contract |
| 71 | `start` | **S18** residual present | `jigc start` | 0 | none | none | orientation renders `Clean`; no `also open:` row | matches contract |
| 72 | `milestone create` | **S18** record-less milestone residual, same slug | `jigc milestone create "Axis three probe"` | 1 | `milestone.serial-collision` | Human | residual sentence + repo-relative route; **HEAD unmoved** | matches contract |
| 73 | `start` (mint) | **S18** task residual, same slug | `jigc start --workflow quick-fix "tidy the readme"` | 1 | `task.serial-collision` | Human | residual sentence + route; nothing minted | matches contract |
| 74 | `rename` | **S18** task residual · milestone-area residual | `jigc rename adr:single-node-cache --to distributed-cache` | 0 | none | none | rename **proceeds** over both residuals (the M53-relayed cell, now driven) | matches contract |
| 75 | `rename` | **control** — a genuinely live task | same, with one open task | 1 | `rename.in-flight` | Mechanical | names `jigc task finalize live-task` / `jigc task discard live-task` | matches contract |
| 76 | `doc list` | a symlink wearing a staged identity, beside a directory wearing one | `jigc doc list --task tidy-the-readme` | 0 | none | none | `adr:via-symlink  docs/decisions/via-symlink.md  managed` is listed; `adr:via-directory` is **not** | **DEFECT F-1** (the same door's probe calls that entry foreign) |
| 77 | `milestone list-tasks` | a pin-less **milestone** area while the record is live | `jigc milestone list-tasks axis-three-probe` | 0 | none | none | the sub-task roster still renders; nothing bricks | matches contract |
| 78 | `milestone execute` | the same | `jigc milestone execute axis-three-probe` | 0 | none | none | composes; nothing bricks | matches contract |

*(Rows 20, 33, 47, 49, 57, 64 assert the machine envelope rather than the text and are kept as their
own rows for that reason. The table is **78 rows over 13 doors**; the six `DESTROYING_DOORS` members
carry **69** of them.)*

---

## 3 · Repro blocks

Each is `setup · argv · observed`. `mkrig <kind>` = `dev/jigc-rig fresh --binary
/Users/maurice/.local/bin/jigc`, two-step-eval'd, then `milestone create "Axis three probe"` +
two `milestone add-task` (kind `milestone`), or `jigc start --workflow quick-fix "tidy the readme"`
(kind `task`). `mkwork` = `milestone provision` + a staged code file and an authored `commit:<sub>` in
each of the two sub-task worktrees.

### R-A — **A3-1 CLOSED**: the milestone area's complement, `merged/` included, is displaced (rows 57–61)

```
setup: mkrig milestone; mkwork
       M=.jigc/milestones/axis-three-probe
       mkdir -p $M/merged/docs $M/merged/sub
       printf 'KEEP-m-top\n'    > $M/merged/top.txt
       printf 'KEEP-m-deep\n'   > $M/merged/docs/deep.txt
       printf 'KEEP-m-prov\n'   > $M/merged/docs/provenance.json
       printf 'KEEP-m-adr\n'    > $M/merged/docs/adr.md           # the §1 colon-less stem
       printf 'KEEP-m-nested\n' > $M/merged/sub/nested.txt
       printf 'KEEP-m-root\n'   > $M/mnotes.txt
       printf 'KEEP-m-sub\n'    > .jigc/tasks/first-sub/tnotes.txt
  BEFORE-CONTROL: command grep -rl 'KEEP-\|MILESTONE-AREA PRECIOUS\|DEEP' .jigc | wc -l  ->  8

$ jigc milestone finalize axis-three-probe --format json
  exit 0
  stdout "committed": { "displaced": [ 8 pairs, sorted by from ], "files": 4, … }
    .../merged/docs/adr.md            -> .jigc/displaced/axis-three-probe/merged/docs/adr.md
    .../merged/docs/deep.txt          -> …/merged/docs/deep.txt
    .../merged/docs/provenance.json   -> …/merged/docs/provenance.json
    .../merged/sub                    -> …/merged/sub            <- the directory, WHOLE
    .../merged/top.txt                -> …/merged/top.txt
    .../mnotes.txt                    -> …/mnotes.txt
    .../sub                           -> …/sub                   <- area-root dir, WHOLE
    .jigc/tasks/first-sub/tnotes.txt  -> .jigc/displaced/first-sub/tnotes.txt
  stderr  note: the working area held 7 entries jigc did not write … moved aside, not taken: (7 lines)
          note: the working area held 1 entry  jigc did not write … moved aside, not taken: (1 line)

  AFTER-CONTROL: command grep -rl … .jigc | wc -l  ->  8     # 8 before, 8 after
    every path now under .jigc/displaced/…; find .jigc/milestones .jigc/tasks -> both roots EMPTY
    git log --oneline -1 -> 9e4e7f4 Finalize milestone axis-three-probe (2 sub-tasks)
```

M52's row 68 was `exit 0 · displaced: [] · named on no stream · both bytes destroyed`. Every one of
those four facts is now the opposite. The two `merged/docs/` cells §1 and §2 name are inside this
drive: `adr.md` is **not** resolved to doctype `adr` (no `schema-conformance.unknown-type`, no gate
copy), and `provenance.json` is **not** inherited from `TASK_DOCS_FILES`.

### R-B — **A3-1 cell (i) CLOSED** at the two refusing doors (rows 14, 15, 26)

```
setup: mkrig milestone; jigc milestone provision axis-three-probe   (worktrees CLEAN, so the
       dirty-worktree arm does not pre-empt); the same 7 plants as R-A
  BEFORE-CONTROL: command grep -rl 'KEEP-m-' .jigc | wc -l -> 7

$ jigc milestone discard axis-three-probe            -> exit 1
  blocking · milestone.foreign-bytes — … its workbench holds 7 path(s) jigc did not write …
    .jigc/tasks/first-sub/tnotes.txt
    .jigc/milestones/axis-three-probe/merged/docs/adr.md
    .jigc/milestones/axis-three-probe/merged/docs/deep.txt
    .jigc/milestones/axis-three-probe/merged/docs/provenance.json
    .jigc/milestones/axis-three-probe/merged/sub
    .jigc/milestones/axis-three-probe/merged/top.txt
    .jigc/milestones/axis-three-probe/mnotes.txt
    route: … or `jigc milestone discard axis-three-probe --force` …
  AFTER: 7                                            # nothing taken

$ jigc uninstall                                      -> exit 1
  blocking · uninstall.foreign-bytes — … holds 7 path(s) …   (the identical set)
  AFTER: 7
```

This is the exact state the settle record's driven datum says exited **0** at rc.16 with *"workbench
removed"* and both `merged/` bytes gone. The consent gate is no longer defeated by the carve-out.

### R-C — `milestone discard --force` still **takes** the complement, and narrates it (row 15)

```
setup: mkrig milestone; provision; merged/top.txt, merged/docs/deep.txt, merged/sub/nested.txt,
       mnotes.txt, tasks/first-sub/tnotes.txt          BEFORE: 5
$ jigc milestone discard axis-three-probe --force      -> exit 0
  stdout  discarded milestone:axis-three-probe (2 sub-task(s); workbench removed)
  stderr  warning: removing the working area .jigc/tasks/first-sub discards work that is not in git:
              .jigc/tasks/first-sub/tnotes.txt
            note: the working area is the only copy of these bytes — they are not recoverable.
          warning: removing the working area .jigc/milestones/axis-three-probe …:
              .../merged/docs/deep.txt
              .../merged/sub
              .../merged/top.txt
              .../mnotes.txt
            note: … not recoverable.
  AFTER: 0        # taken, under the consent, and every path named first
```

§3's decision — the disposition is read off the **call** (`SubtaskComplement::Take`), not the function
— holds: this door still takes, and the `--force` is the consent.

### R-D — **A3-2 CLOSED**, all three move outcomes at `task finalize` (rows 47–49)

```
### none move, cell (a): the parking home is a regular file
setup: mkrig task; a landable commit doc; printf 'x\n' > code.txt; git add code.txt
       printf 'PRECIOUS-A3-2\n' > .jigc/tasks/tidy-the-readme/notes.txt
       printf 'NOT A DIR\n'     > .jigc/displaced
  BEFORE: command grep -rl 'PRECIOUS-A3-2' .jigc | wc -l -> 1
$ jigc task finalize tidy-the-readme --format json      -> exit 0
  stdout  "committed": { "displaced": [], … },
          "findings": [ { "code": "finalize.foreign-bytes", "severity": "advisory",
                          "key": {"code":"finalize.foreign-bytes","target":"task:tidy-the-readme"},
                          "location": {"address":"task:tidy-the-readme"},
                          "message": "`.jigc/tasks/tidy-the-readme` holds 1 path(s) jigc did not write,
                             so the working area was left standing rather than removed with them —
                             `.jigc/` is gitignored, so nothing else has a copy … ; 1 of them could not
                             be moved aside: … — could not open .jigc/displaced/tidy-the-readme to park
                             it: Not a directory (os error 20)",
                          "route": "the commit landed and nothing in it is affected. Keep what you need
                             from `.jigc/tasks/tidy-the-readme` and delete the rest — jigc mints no verb
                             that clears it …" } ]
  AFTER: 1      ls -a .jigc/tasks/tidy-the-readme -> notes.txt        # LEFT STANDING

### none move, cell (b): an ordinary read-only .jigc/displaced/   (2 plants)
  chmod 500 .jigc/displaced; BEFORE: 2 -> exit 0, both named, Permission denied (os error 13), AFTER: 2

### some move: occupy one entry's parent under the parking home
setup: root.txt at the area root + docs/stray.txt;  mkdir .jigc/displaced/tidy-the-readme
       printf 'occupied\n' > .jigc/displaced/tidy-the-readme/docs        # a FILE at the needed dir
$ jigc task finalize tidy-the-readme --format json      -> exit 0
  stderr  note: the working area held 2 entries jigc did not write … — 1 of them moved aside, not taken:
              .jigc/tasks/tidy-the-readme/root.txt → .jigc/displaced/tidy-the-readme/root.txt
            and 1 could not be moved:
              .jigc/tasks/tidy-the-readme/docs/stray.txt — could not open
                .jigc/displaced/tidy-the-readme/docs to park it: File exists (os error 17)
  stdout  displaced = [ root.txt pair ]  (the moved pair ALONE)   findings = [finalize.foreign-bytes]
  AFTER: both plants on disk (one parked, one standing)
```

`held 2 entries` is the **complement's** count, not `moved.len()` — the law-1 lie M52 drove at rc.16.

### R-E — **D1×D2**: a `merged/` byte whose move failed survives a landed boundary (row 64)

```
setup: mkrig milestone; mkwork
       mkdir -p $M/merged/docs
       printf 'KEEP-D1xD2-merged\n'  > $M/merged/docs/deep.txt
       printf 'KEEP-D1xD2-root\n'    > $M/mnotes.txt
       printf 'KEEP-D1xD2-subtask\n' > .jigc/tasks/first-sub/tnotes.txt
       printf 'NOT A DIR\n'          > .jigc/displaced
  BEFORE: 3
$ jigc milestone finalize axis-three-probe --format json     -> exit 0
  stdout top-level keys: ['committed']        committed.displaced = []     # the pinned envelope
  stderr  note: … held 2 entries … not one of them could be moved aside:  (both named, os error 20)
          note: … held 1 entry  … not one of them could be moved aside:
          advisory · finalize.foreign-bytes — `.jigc/milestones/axis-three-probe` holds 2 path(s) …
            at: milestone:axis-three-probe
          advisory · finalize.foreign-bytes — `.jigc/tasks/first-sub` holds 1 path(s) …
            at: task:first-sub
  AFTER: 3        find .jigc/milestones .jigc/tasks -> both areas LEFT STANDING with their plants
```

One advisory **per standing area**, each keyed at its own work unit; stderr-only at this door, which is
the declared bound, not a finding.

### R-F — the blocked boundary, and the selective clear (rows 62–63)

```
setup: mkrig milestone; provision; both sub-tasks stage code but author NO commit doc;
       jigc config set finalize.fan-out.squash false
       plants: merged/docs/{adr.md,deep.txt,provenance.json}, merged/top.txt, mnotes.txt,
               tasks/first-sub/tnotes.txt                           BEFORE: 6
$ jigc milestone finalize axis-three-probe                 -> exit 3
  blocking · schema-conformance.field-value-conformant  (commit:first-sub, commit:second-sub)
  blocking · schema-conformance.required-slot-present    (both)
  grep -c 'unknown-type|foreign-bytes|moved aside' -> 0
  AFTER: 6      .jigc/displaced -> No such file or directory
  merged/ tree: docs/{adr.md, commit:first-sub.md, commit:second-sub.md, deep.txt, provenance.json},
                top.txt            # materialized bodies left standing beside the plants

… then author both commit docs and re-run:
$ jigc milestone finalize axis-three-probe --format json   -> exit 0
  commits: ['chore: add first-sub work', 'chore: add second-sub work',
            'Finalize milestone axis-three-probe (2 sub-tasks)']
  displaced: the 5 milestone-area plants + tasks/first-sub/tnotes.txt
  AFTER: 6       # the stale commit:*.md bodies are gone; every plant survives
```

The blocked arm is **not blocked by** the plants, and the clear that follows is selective.

### R-G — `FINALIZE_MESSAGE_FILE` joins both registry rows, with its two controls (rows 23, 42)

```
### (a) jigc's own transient at BOTH area kinds
setup: mkrig milestone
       printf 'msg\n' > .jigc/milestones/axis-three-probe/finalize-message.tmp
       printf 'msg\n' > .jigc/tasks/first-sub/finalize-message.tmp
$ jigc milestone discard axis-three-probe   -> exit 0, "workbench removed", NO foreign-bytes

### (b) near-miss NAMES at the same two loci   (the control that proves the probe ran)
       .../axis-three-probe/finalize-message.tmpX  and  .../first-sub/finalize-message
$ jigc milestone discard axis-three-probe   -> exit 1  milestone.foreign-bytes, BOTH named

### (c) the shape leg: a DIRECTORY named finalize-message.tmp
$ jigc milestone discard axis-three-probe   -> exit 1  milestone.foreign-bytes names it
```

### R-H — the residual rule at the destroying, enumerating and mint doors (rows 44, 45, 70–73)

```
### by-id doors, over an area an unwind fault left pin-less (R-E's own leftover state)
$ jigc task discard first-sub     -> exit 1
$ jigc task finalize first-sub    -> exit 1
  blocking · finalize.no-task — no task `first-sub`: `.jigc/tasks/first-sub` is a directory carrying
    no base pin, so it is a leftover and not a work unit — either jigc never minted a task there, or
    a teardown stopped partway and left the directory behind
    at: task:first-sub
    route: nothing was changed. Keep anything you need from `.jigc/tasks/first-sub` and delete the
      rest by hand — jigc mints no verb that clears a leftover working area …
  (no host path on either surface)

### D2.6's named cell — a bare mkdir under a JOINED milestone
setup: mkrig milestone; mkwork; jigc milestone finalize axis-three-probe; mkdir -p .jigc/tasks/first-sub
$ jigc task discard first-sub     -> exit 1, the same finding
  git log --oneline -1 unchanged (b189e35 before and after)     # NO record commit

### §8's guard — a WHOLE leftover area of a settled sub-task (pin intact)
setup: mkdir -p .jigc/tasks/second-sub; printf '{}' > .jigc/tasks/second-sub/base.json
$ jigc task discard second-sub    -> exit 1
  blocking · milestone.terminal — sub-task `second-sub` of milestone `axis-three-probe` is already
    `joined` on the committed record — its working area is a leftover, not live work …
    at: task:second-sub          route: … jigc doc show milestone-record:axis-three-probe
  git log unchanged

### enumerating doors
$ jigc task list                  -> exit 0  "no active tasks"
$ jigc start                      -> orientation, no open-task row

### the two mints
$ jigc start --workflow quick-fix "tidy the readme"   (over .jigc/tasks/tidy-the-readme + a foreign file)
  -> exit 1  task.serial-collision  + the residual sentence + the same Human route
$ jigc milestone create "Axis three probe"            (over a record-less .jigc/milestones/… residual)
  -> exit 1  milestone.serial-collision + the residual sentence;  HEAD unmoved
```

M52's driven false-flip — `jigc task discard first-sub` at exit 0 with `record commit: d53299d` and the
record reading `status: joined` over `- status: discarded` — is closed at resolution, before the
splice.

### R-I — `rename` over both residual kinds, with its live-task control (rows 74–75)

```
setup: dev/jigc-rig committed-singletons; a record-decision task that lands adr:single-node-cache
       mkdir -p .jigc/tasks/leftover-area;   printf 'F\n' > .jigc/tasks/leftover-area/f.txt
$ jigc rename adr:single-node-cache --to distributed-cache   -> exit 0
  renamed adr:single-node-cache -> adr:distributed-cache (…), repointed 0 referrer(s)
       mkdir -p .jigc/milestones/leftover-ms; printf 'F\n' > .jigc/milestones/leftover-ms/f.txt
$ jigc rename adr:distributed-cache --to node-cache          -> exit 0
CONTROL:  jigc start --workflow quick-fix "live task"; jigc rename adr:node-cache --to cache-again
  -> exit 1  rename.in-flight — cannot rename while task `live-task` is in flight …
```

This is the cell the settle record marked **relayed, not re-driven** ("the orchestrator's corpus held
singletons only"). Driven now, both halves.

### R-J — the `LeftoverShape` × `--force` matrix at `milestone provision` (rows 1–9)

```
setup: mkrig milestone, one fresh rig per shape, plant at .jigc/worktrees/first-sub:
  dir        mkdir + precious.txt          -> 1 milestone.leftover-holds-work  "…: precious.txt — git
                                                reports no worktree of its own there"
  dangling   mkdir .git + wip.txt          -> 1  "…: .git, wip.txt — git cannot read a repository there"
  file       a plain file                  -> 1  "…: the file itself — it is a file, not a worktree"
  symlink    ln -s /etc/hosts              -> 1  identical wording
  unreadable chmod 000                     -> 1  "…: unknown — could not read the leftover directory
                                                   …: Permission denied (os error 13)"
  --force over dir                         -> 0  warning names precious.txt + "not recoverable",
                                                then "provisioned 2 worktree(s) … at base 4f496e2"
  --force over a symlink to an OUTSIDE dir -> 0  "removing the leftover file …"; outside tree 1/1 intact
  --force, read-only PARENT                -> 1  milestone.provision-failed; the loss warning names
                                                precious.txt and it IS gone (outcome-keyed)
  --force, read-only LEFTOVER              -> 1  milestone.provision-failed; NO loss warning; the
                                                plant survives
```

### R-K — `uninstall`'s subjects and their precedence (rows 24–35)

```
mkrig fresh + one plant each, one rig per row:
  .jigc/notes.txt                       -> 1 uninstall.untracked-workbench-file   (route: git add <path>)
  .jigc/logs  as a plain FILE           -> 1 uninstall.untracked-workbench-file
  .jigc/tasks as a plain FILE           -> 1 uninstall.untracked-workbench-file
  .jigc/displaced/some-task/keep.txt    -> 1 uninstall.foreign-bytes
     … --force                          -> 0 "warning: removing the relocation workbench .jigc/displaced
                                             discards work that is not in git:  … keep.txt"; AFTER 0
  clean, --format json                  -> 0 {allowlist_file, findings, line_file, removed}
  clean, second run                     -> 0 "(nothing to remove — no repo-local jigc install …)"
  chmod 000 .jigc/worktrees             -> 1 uninstall.dirty-worktree; route: "this failure is a
                                             `read_dir` of that directory, not a git fault" + --force
  refusal --format json                 -> 1 stderr {findings, schema_version}; stdout 0 bytes
precedence, all four subjects at once   -> 1 uninstall.dirty-worktree (1 of 4)
```

### R-L — `task discard`'s complement shape space, its consent and its controls (rows 36–43)

```
setup: mkrig task; eight plants in .jigc/tasks/tidy-the-readme/
  .hidden · docs/adr:linked.md (SYMLINK) · docs/notes.txt · docs/unstaged.md · root.txt · link
  (symlink) · nested/deep.txt · loose.md
$ jigc task discard tidy-the-readme               -> exit 1
  blocking · task-discard.foreign-bytes — … holds 8 path(s) jigc did not write …  (all eight listed)
  route: move what you need out of `.jigc/tasks/tidy-the-readme/` … or `--force`
  area intact: 10 entries
$ jigc task discard tidy-the-readme --force       -> exit 0
  warning: removing the working area … discards work that is not in git:  (all eight) + not recoverable
  discarded task tidy-the-readme — dropped staged edits to: adr:linked, commit:tidy-the-readme (transient)
                                                        ^^^^^^^^^^ see F-2
CONTROLS:
  jigc's own 14-member set only          -> 1 task-discard.staged-prose; foreign-bytes does NOT fire
  chmod 000 …/docs                       -> 1 task-discard.foreign-bytes, fail-closed, 0 host paths
  "" and "../.."                         -> 1 work-unit.malformed-id; .jigc/ intact
```

### R-M — the hook cells at `task finalize` (rows 50–51)

```
### a SUCCEEDING pre-commit hook that writes into the area during the commit
setup: mkrig task; a landable commit doc; .git/hooks/pre-commit writes
       "$(git rev-parse --show-toplevel)/.jigc/tasks/tidy-the-readme/hookfile.txt"
$ jigc task finalize tidy-the-readme --format json   -> exit 0
  displaced = [{from: .jigc/tasks/tidy-the-readme/hookfile.txt,
                to:   .jigc/displaced/tidy-the-readme/hookfile.txt}]      findings = []
  area removed clean; the hook's byte parked         # see O-4

### a REJECTING pre-commit hook, with a plant present
  BEFORE 1 -> exit 1
  `git commit` was rejected (no commit was made): hook says no
  task tidy-the-readme is intact — nothing was committed, your task's staged docs are still in
  `.jigc/tasks/tidy-the-readme/docs/`, and anything you had `git add`-ed is still in git's index.
  AFTER 1; .jigc/displaced not created; finalize-message.tmp not left behind
```

### R-N — the two zero-false-fire controls (rows 53, 68)

```
### task door
setup: mkrig task; a landable commit doc; code.txt staged; no plant
$ jigc task finalize tidy-the-readme --format json  -> 0
  displaced [] · findings [] · stderr has 0 lines matching 'moved aside|foreign-bytes'
  ls -d .jigc/displaced -> No such file or directory ;  .jigc/tasks empty

### boundary door, INCLUDING merged/ after D1.4
setup: mkrig milestone; mkwork; no plant
$ jigc milestone finalize axis-three-probe --format json -> 0
  displaced [] · 0 matching stderr lines · .jigc/displaced absent · both area roots empty
```

### R-O — the displacement's collision and its `.2` suffix (row 52)

```
setup: mkrig task; finalize once with notes.txt = FIRST-COPY; re-mint the same slug; notes.txt = SECOND-COPY
$ jigc task finalize tidy-the-readme (twice)    -> 0, 0
  second: … notes.txt → .jigc/displaced/tidy-the-readme/notes.txt.2
  find .jigc/displaced -type f -> notes.txt, notes.txt.2 ; cat …/notes.txt -> FIRST-COPY
```

### R-P — **F-1**: a symlink wearing a staged identity is jigc's at the read/ack surfaces and a third
party's at the destroying probe (rows 36, 37)

```
setup: mkrig task
       OUT=$(mktemp -d …); printf 'TARGET\n' > "$OUT"/body.md
       ln -s "$OUT/body.md" .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
       mkdir -p            .jigc/tasks/tidy-the-readme/docs/adr:via-directory.md

$ jigc task discard tidy-the-readme               -> exit 1
  blocking · task-discard.foreign-bytes — … holds 2 path(s) jigc did not write …
    .jigc/tasks/tidy-the-readme/docs/adr:via-directory.md
    .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md          <- FOREIGN here

$ jigc doc list --task tidy-the-readme
  id                      path                            state
  adr:via-symlink         docs/decisions/via-symlink.md   managed      <- JIGC'S OWN here
  commit:tidy-the-readme  commit:tidy-the-readme          managed
  (adr:via-directory is correctly absent — M52 Inc 4 / T1's fix holds on the directory axis)

$ jigc uninstall                                  -> exit 1 uninstall.foreign-bytes, same 2 paths

# and the ack, from R-L's --force run:
  discarded task tidy-the-readme — dropped staged edits to: adr:linked, commit:tidy-the-readme
    ^ adr:linked was a SYMLINK the same door had just called foreign
```

Cause, read at the line after driving: `engine::state::foreign_area_paths`
(`crates/engine/src/state.rs:422`) reads shape with `DirEntry::file_type()` — **no-follow** — while
`crate::task::staged_doc_ids` (`crates/cli/src/task.rs:716`) reads it with `std::fs::metadata`, which
**follows**. The promotion path is on the engine's side of the split: driven, a symlinked
`adr:outside-secret.md` was **not** promoted (`1 file committed`, `git grep -l OUTSIDE-SECRET-BYTES
HEAD` → rc 1) and was displaced instead — so no byte of a third party's tree reaches the store and none
is lost. The contradiction is confined to what the surfaces **say**.

### R-Q — **F-2**: the residual note calls a symlink "a directory" (row 54's sibling)

```
setup: mkrig fresh; OUT=$(mktemp -d …); printf 'OUTSIDE2-KEEP\n' > $OUT/keep.txt
       mkdir -p .jigc/tasks; ln -s "$OUT" .jigc/tasks/ghost
$ jigc task discard ghost               -> exit 1
  blocking · finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` is a directory carrying no base
    pin, so it is a leftover and not a work unit …
                                  ^^^^^^^^^^^^ it is a SYMLINK:
  ls -l .jigc/tasks -> lrwxr-xr-x  ghost -> /var/folders/…/outside2.NvFgrv
$ jigc task discard ghost --force       -> exit 1, identical
  outside tree: 1/1 bytes intact
```

### R-R — **OBS-1**: at `merged/docs/`, a third party's file named `<ty>:<slug>.md` dies at exit 0

```
setup: mkrig milestone; mkwork; mkdir -p $M/merged/docs
       printf 'USER-BODY-KEEP\n'  > $M/merged/docs/adr:user-authored.md
       printf 'USER-PLAIN-KEEP\n' > $M/merged/docs/plain.txt
  BEFORE: 1 body-named, 1 plain
$ jigc milestone finalize axis-three-probe --format json   -> exit 0
  stderr  note: the working area held 1 entry jigc did not write … moved aside, not taken:
              .../merged/docs/plain.txt → .jigc/displaced/axis-three-probe/merged/docs/plain.txt
  displaced = [plain.txt only]
  AFTER: body-named = 0   plain = 1
  mentioned on any stream?  stderr 0 hits, stdout 0 hits for 'user-authored'
```

### R-S — **OBS-3**: a symlinked working area is neither refused over nor narrated

```
setup: mkrig fresh; OUT=$(mktemp -d …) holding base.json, workflow, intent, precious.txt
       ln -s "$OUT" .jigc/tasks/ghost-task
$ jigc task list            -> 1 active task(s):  ghost-task  [quick-fix]  probe
$ jigc task discard ghost-task   -> exit 0   "discarded task ghost-task"
  foreign-bytes did NOT fire over precious.txt;  the symlink is unlinked; the target tree is 4/4 intact
```

`crate::task::foreign_areas`' own doc-comment declares this: *"an area that is absent, a plain file, or
a **symlink** is not a working area … A door must neither claim nor refuse over what it will not
take."* Driven, the sink destroys nothing at that path. Recorded so reconciliation is not re-driving
it.

---

## 4 · `(door, cell)` pairs I did **not** drive, and why

Stated plainly rather than presented as covered.

1. **`task finalize` / `milestone finalize` × any cell × `--force`** — *the force axis does not exist
   at either leaf* (`--help` lists `--approve`/`--format`/`--dry-run`/`--carry-staged`, and
   `--carry-staged`/`--format` respectively). `DESTROYING_DOORS`' own doc-comment says that cell "is
   never enumerated because no member carries a consent it does not have". Not driven, by construction.
2. **`Disposition::Narrate` × anything** — **no member holds it** at rc.17 (read from the code, and
   `FINALIZE_DOOR`'s doc-comment says so). An empty arm, not a skipped one.
3. **`milestone provision` × {S7, S8, S12, S13, S16}** — not this door's subject; the no-`--force`
   rows drove that a staged doc, an untracked workbench file and a foreign byte in either area leave it
   at exit 0 with its ordinary ack. The forced cell cannot differ, because `--force` only widens what it
   may remove **at a worktree path**.
4. **`task discard` × {S1–S6, S13, S16}** — no such cell exists at that door: its subject is
   `.jigc/tasks/<id>/`, it reads no worktree path and no milestone area. Row 19 is the driven evidence
   that the scoping is real (a milestone-scoped probe over an unrelated area).
5. **`milestone discard` / `uninstall` × S5 `File` at a *registered* worktree path** — unreachable:
   `git worktree add` cannot register a non-directory. The reachable branch is rows 16 and 66.
6. **Any cell × `OwnWorktree` × `Unreadable`** — empty by construction: `git rev-parse
   --show-toplevel` cannot run inside a `chmod 000` directory, so the verdict is `Unverifiable`.
7. **The record-commit rejection cells at `milestone discard` / `task discard`** — that is axis 4. Row
   51 drives the rejection at `task finalize` only because the question there is *did the destroying
   half run*, which is this axis's.
8. **A genuine concurrent racer** against a displacement or a teardown — headless by construction, the
   bound M51's and M53's own e2e halves declare.
9. **`--ignored` build output as a *refusal*** — M46's measured *visible, not prevented* decision;
   driving it as a defect would report a decided bound as a finding. Its **narration** is driven (row
   65).
10. **`InProgress::ALL` × the destroying doors** — that is axis 2's matrix. I drove none of the ten
    git states here; the only posture fact this axis needs (a landed vs blocked boundary) is driven
    directly.
11. **A second concurrent `.jigc/displaced/<id>` writer** producing a `.3`/`.4` suffix — row 52 drives
    the collision primitive once; `free_displacement_path`'s unbounded loop is a declared 1.x row.
12. **An undecodable (non-UTF-8) filename** in either area — the declared `task.rs:5845` bound,
    unchanged by M53.

---

## 5 · Findings

### F-1 · **tier 3** — a **symlink** wearing a staged identity is jigc's own at the read and ack surfaces and a third party's at the same door's destroying probe

**Repro:** §3 R-P. **Doors:** `task discard` (refusal *and* `--force` ack), `doc list --task`,
`uninstall`.

`engine::state::foreign_area_paths` reads entry shape with `DirEntry::file_type()` (**no-follow**) and
its doc-comment states the rule and its provenance: *"Shape is part of membership … the shape is read
**without following symlinks** … This is L-3's lesson at the door's own question: a directory named
`<type>:<slug>.md` was a staged identity at four surfaces until M52 Increment 4 / T1."*
`crate::task::staged_doc_ids` (`crates/cli/src/task.rs:716`) reads it with `std::fs::metadata`, which
**follows**. So M52 Increment 4 / T1's shape fence closed the **directory** axis of its class and left
the **symlink** axis open — the complete-fix lens on M52's own work, at the axis M53's own `merged/`
walk re-states one layer over.

Driven consequences, all on what the surfaces *say*: one entry is listed by `jigc doc list --task` as a
`managed` doc at `docs/decisions/via-symlink.md` **while the same door's refusal names it as a path
jigc did not write**, and `jigc task discard --force` acks `dropped staged edits to: adr:linked` for an
entry it did not stage and did not drop.

**Why it is not tier 1, stated rather than assumed.** I drove the two paths that could make it loss or
harm and both are safe: the promotion plan is on the engine's side of the split, so a symlinked
`adr:outside-secret.md` was **not** promoted (`1 file committed`; `git grep -l OUTSIDE-SECRET-BYTES
HEAD` → rc 1) and was **displaced** instead; and `remove_dir_all` unlinks a symlink rather than
descending it, so the target tree is byte-intact after `--force` (§3 R-L, R-S). The guard side
over-claims, which is the safe direction.

### F-2 · **tier 3** — the residual note asserts a shape it did not check: a symlink is called "a directory"

**Repro:** §3 R-Q. **Door:** every by-id door that reaches `engine::state::residual_area_note`
(driven at `task discard`, with and without `--force`).

`residual_area_note` hard-codes *"`{listed}` is a **directory** carrying no base pin"*. Driven,
`.jigc/tasks/ghost` is a symlink (`ls -l` → `lrwxr-xr-x ghost -> …`) and the sentence says directory.
Law 1 (`design/surface-contract.md` → *nothing lies*) binds the sentence, and the predicate the door
actually asked is `carries_base_pin`, which is about `<area>/base.json` and says nothing about the
area's own shape. The route is still followable and nothing is destroyed, so it is a wording defect,
not a behaviour one — but it is the same *shape-is-part-of-membership* axis F-1 sits on, one sentence
over.

---

## 5.1 · Observations — driven, **not** defects (recorded so reconciliation is not re-driving them)

- **OBS-1 · at `merged/docs/`, a third party's file named `<ty>:<slug>.md` is destroyed at exit 0, on
  no stream** (§3 R-R: `adr:user-authored.md` gone, its sibling `plain.txt` displaced, zero mentions on
  either stream). **This is the binary doing exactly what its own stated rule says** — `settle-record`
  §2 and `MILESTONE_AREA_FILES`' doc-comment fix the `merged/docs/` membership as **`staged_doc_id`
  alone**, with the reason (inheriting `TASK_DOCS_FILES` would call a foreign `provenance.json` jigc's),
  and `staged_doc_id`'s own doc-comment prices the trade (*"the exact discriminator's failure mode is
  losing authored prose, and the looser one's is keeping a file too many"*). M52's conversion-ledger
  citation pins the behaviour as **expected output** (*"no colon-bearing materialized body is ever
  parked"*). So it contradicts no stated contract and is **not** a defect under this review's
  definition. I surface it because it is the one cell of M53's new `merged/` walk where a third party's
  bytes die at exit 0 unnamed, and because the loss cell itself is stated on **no** user-facing surface
  — a 1.x ledger row, not a tier-1 blocker. The sibling cell one door over is safe by the *opposite*
  rule: under a **task**'s `docs/`, the same shape is kept-too-much and `task finalize` **blocks at
  exit 3** rather than taking it (row 56).
- **OBS-2 · `uninstall.foreign-bytes`' route always names `.jigc/displaced/`**, even when no listed
  path is under it (driven at rows 26 and 27: three task/milestone-area paths, and the parenthetical
  still reads *"`rm -r` takes them — jigc has no verb that clears `.jigc/displaced/`"*). The producer's
  own doc-comment distinguishes the two cases (*"For a working area's complement the operator can also
  move the bytes out; for `.jigc/displaced/` there is nothing else"*) while the emitted string does not.
  Every clause is **true**, so law 1 is not broken; it is misdirection, not a lie. Tier-3 clarity at
  most.
- **OBS-3 · a symlinked working area is neither refused over nor narrated** (§3 R-S) — declared in
  `crate::task::foreign_areas`' doc-comment, and the sink destroys nothing there. Not a defect.
- **OBS-4 · the settle record's §6 motivating cell is not reachable with a git hook on rc.17.** §6
  reasons from *"a succeeding hook writes into the area during the commit, where no probe can see
  it"*. Driven (§3 R-M), a `pre-commit` hook's file **is** seen: the displacement runs after `git
  commit` returns, so the byte is parked and the area unwinds clean. The design decision §6 reached
  (take the finding's path set from a `foreign_area_paths` re-read **after** the unwind) is unaffected
  and correct; only the cell's stated reachability is narrower than written. The genuinely un-seeable
  racer is a concurrent process, which is declared out.
- **OBS-5 · `milestone.leftover-holds-work` is the one destroying-door refusal still on the flattened
  `{"error": …}` arm** while `milestone discard`, `task discard` and `uninstall` refusals now carry
  `{findings, schema_version}` (rows 10, 20; driven side by side). `design/worked-examples.md:3680`
  declares exactly this — *"`milestone.leftover-holds-work`, M48's code, rides the flattened `{error}`
  arm exactly as it did before, stated rather than silently accepted"* — so it matches contract. Noted
  because a reader diffing the family will see the split.
- **OBS-6 · `milestone finalize` lands at exit 0 over unauthored sub-task commit docs under the
  default `squash: true`** (driven: both `commit:<sub>` docs carry `type: ""` and an empty required
  `summary`, `jigc task validate <sub>` blocks on both, and the boundary commits). This is
  `1799a2d`'s **stated carve-out** — under `squash: true` the aggregate message is CLI-synthesized and
  no commit doc is read — and the gate does bind under `squash: false` (driven: exit 3, four findings,
  §3 R-F). Recorded because the un-driven reading of the fold-back sentence looks like a regression and
  is not.
- **OBS-7 · `remove_worktrees`' two warnings print host-absolute paths** (driven at row 18). Declared
  `DeclaredAbsolute` in `crates/cli/tests/repo_relative_paths.rs` with its reason (git resolves a
  worktree path against the caller's cwd). Graded against, not re-found.

---

## 6 · M52 §A rows for this axis: CLOSED / STILL-OPEN

The comparison is row by row, each re-driven on `1.0.0-rc.17` with its argv.

| M52 §A row | tier | verdict on rc.17 | the argv + what it shows |
|---|---|---|---|
| **`(3, A3-1)`** — `jigc milestone finalize` destroys the milestone's own working-area complement at exit 0, named by nothing | HIGH / tier 1 | **CLOSED** | `jigc milestone finalize axis-three-probe --format json` over 8 planted loci (area root · a nested area dir · `merged/top.txt` · `merged/docs/deep.txt` · `merged/docs/provenance.json` · `merged/docs/adr.md` · `merged/sub/nested.txt` · a sub-task area): exit 0, **8 before / 8 after** by `command grep`, every byte at `.jigc/displaced/<unit>/<relative>`, all 8 pairs on `committed.displaced`, two `note:` blocks naming each `<from> → <to>`. §3 R-A |
| — its **cell (i)**, the `merged/**` region at the refusing doors | — | **CLOSED** | `jigc milestone discard axis-three-probe` and `jigc uninstall` over the same plants: exit 1 under `milestone.foreign-bytes` / `uninstall.foreign-bytes`, **all 7** named (a foreign directory returned whole), nothing taken; `--force` narrates all 7 and then takes them. The settle record's falsifying datum (exit 0, *"workbench removed"*, both `merged/` bytes gone) does not reproduce. §3 R-B, R-C |
| — its **cell (ii)**, `materialize` on a boundary that does **not** land | — | **CLOSED** | `jigc milestone finalize axis-three-probe` over unauthored commit docs: exit 3 on its own two `schema-conformance.*` codes, **6/6 plants on disk**, `.jigc/displaced` not created, zero `unknown-type`; the re-run after authoring clears only the stale `commit:*.md` bodies and displaces all 6. §3 R-F |
| **`(3, A3-2)`** — when the displacement fails, both `Displace` doors remove the area anyway | HIGH / tier 1 | **CLOSED at both doors** | `jigc task finalize tidy-the-readme --format json` with `.jigc/displaced` a regular file → exit 0, **area left standing**, before=1 after=1, one advisory `finalize.foreign-bytes` on `findings` naming the path, the count and `Not a directory (os error 20)`, route `Human`. The read-only-`displaced/` cell is identical (`Permission denied`). At `milestone finalize`: 3/3 plants survive, one advisory **per standing area** keyed at its own unit, stderr-only, envelope still `['committed']`. §3 R-D, R-E |
| — its **cell (i)**, the partial move | — | **CLOSED** | occupy one entry's parent under the parking home → one narration block carrying `held 2 entries` (the **complement's** count), the move it made, the entry it could not move, and `File exists (os error 17)`; `committed.displaced` carries the moved pair alone. §3 R-D |
| — its **cell (ii)**, the non-atomic removal skeleton | — | **CLOSED** | the fault-on-a-later-member cell is driven end to end in R-E's own aftermath: `.jigc/tasks/first-sub` is left standing **without** its pin, and every by-id door then answers `finalize.no-task` with the residual sentence rather than resolving it. §3 R-E → R-H |
| — its **cell (iii)**, `cleanup_subtask_areas`' ignored `all_gone` | — | **CLOSED** | R-E's boundary settles two areas and emits **two** `finalize.foreign-bytes`, one per area, keyed `milestone:axis-three-probe` and `task:first-sub`; and `jigc milestone discard --force` still **takes** a sub-task area's foreign byte, still acks `workbench removed`, and mints no advisory (§3 R-C) — the disposition read off the call, not the function |
| **`(3, A3-3)`** — `milestone_boundary_displacement`'s subject was the sub-task areas only | LOW / tier 3 | **CLOSED** (behaviourally; the suite half is not this driver's to assert) | the behaviour A3-3 said was untested is now driven green at the boundary's **own** area (row 57) and at the move-failure cell (row 64). The suite-shape half — `grep -c 'milestones' milestone_boundary_displacement.rs` → 0 at rc.16 — is a source fact; M53's conversion ledger cites `milestone_boundary_displacement::a_landed_milestone_boundary_keeps_every_byte_of_its_own_area_it_did_not_write` for it, and verifying a test's content is the reconciler's read, not a drive |

**The M51 rows M52's axis-3 §6 carried forward, all re-driven here and all still CLOSED:**
**C-1** (rows 14, 26, 36, 46, 57 — five doors) · **D-1** (rows 13, 25, 39) · **D-2** (row 18) ·
**D-3** (rows 19, 40) · **D-4** (row 32).

**STILL-OPEN: none on this axis.** No M52 row regressed, no closed row re-opened, and the two tier-1
rows are closed over the class's axis rather than over their reported repro — the `merged/**` region,
the blocked-boundary arm, the three move outcomes, the two unwind-fault positions and the
per-area advisory keying are each driven, not inferred.

---

## 7 · What this adds over flow-54 arms 1, 2 and 3

Flow 54's arms 1 and 2 own this axis in the suite; arm 3 owns the residual rule. What a driven review
adds is not more cells — it is the cells a suite **cannot** hold, plus the cross-arm compositions no
single arm is the subject of.

1. **The release posture.** Every arm runs against the debug binary, where the route fences are
   `debug_assert!`s. Rows 1–75 ran on the **release** build an adopter installs, where a route-fence
   violation would be a bad emitted command rather than a panic. Nothing here panicked and every
   emitted route parsed.
2. **The composition arms 1 and 2 are each half of.** Arm 1 iterates plant loci at doors that succeed
   or refuse; arm 2 iterates move outcomes. Row 64 is **both at once** — a `merged/`-region plant whose
   move fails at a *landed* boundary — and rows 62→63 are the **sequence** (a boundary that blocks at
   exit 3 over six plants, then the same boundary landing after the authoring is fixed, with the
   selective clear in between). A suite arm asserts a state; this asserts the path between two.
3. **Cross-door precedence, driven rather than declared.** Rows 12, 24, 36 and 38 establish the order
   the refusals actually fire in (`dirty-worktree` > `foreign-bytes` > `staged-prose` at the milestone
   doors; `foreign-bytes` > `staged-prose` at `task discard`), which is what an operator meets and what
   no per-door arm can state.
4. **The controls, driven as negatives.** Arm 2's `zero-false-fire` controls are asserted in-suite;
   here they are driven **beside** the positive cells in the same rig family, including the two that
   exist only because M53 moved the registries: `finalize-message.tmp` at both area kinds, with a
   near-miss-name control (`finalize-message.tmpX`, `finalize-message`) and a **shape** control (a
   directory wearing the name) that together prove the probe ran rather than simply stayed quiet.
5. **The relayed cell, now driven.** `jigc rename` over a task residual **and** over a milestone-area
   residual is the one D3 home the settle record marks *relayed, not re-driven*; §3 R-I drives both
   with a live-task control.
6. **The two findings are both outside every arm's subject.** F-1 is a disagreement **between** two
   surfaces of one door — the sort of thing a suite that asserts each surface separately is green over
   by construction — and F-2 is a sentence, not a state. Neither is reachable by an arm that iterates a
   registry, because neither is a registry row.
7. **What the arms hold that this does not.** The order-invariant N-process fan-out, the applied-mutant
   evidence, and the ⇔ fences against the clap tree are all suite property, and this review asserts
   none of them. The genuine concurrent Task-tool spawn is nobody's here either — the same bound M51
   and M53 both declare.

---

# 8 · Reconciliation ledger

**Reconciled 2026-09-22 against `/…/axis-review/codex/axis3-codex.md`** (source pass at
`10e3286c81fa39d4f5464820a2c7591483cfa8e2`, explicitly **not binary-driven**). Every drive below ran on
the same binary the driver used — `/Users/maurice/.local/bin/jigc`, asserted `jigc 1.0.0-rc.17` — in
fresh `dev/jigc-rig` rigs (two-step eval, every root from `mktemp -d`, no teardown, nothing written to
the working repository, no fix, no commit). Exit codes read **bare**; every loss/survival claim carries
a **before-control** taken with `command grep`.

**The rule applied** (acceptance-design.md → *The reconciliation rule*): a claim by one pass that the
other cannot reproduce is a **lead**, not a finding. Every Codex claim was entered as
`lead(codex, …)` and then **driven** — to a repro block, or to a refutation carrying the falsifying
datum. The Codex pass reports **no numbered defect claim**, so its claims are all *closure* and
*absence* claims; those are driven by attempting the behaviour they say is closed.

## 8.1 · The Codex pass's own framing

It asserts **no unregistered destroying door, no removal bypassing the applicable seam, and no
source-grounded tier-1 path to exit-0 byte loss or repository harm**, at *"high confidence from source
inspection; not binary-driven"*. Driven, its **tier-1 half is CONFIRMED** — this axis found no tier-1
row either, and both of the driver's findings are tier 3. It is **silent** on F-1 and F-2; under the
rule, silence leaves a driven defect standing, so both were **re-driven here** and both stand.

## 8.2 · Codex claim → verdict

| # | Codex claim | verdict | the drive, or the falsifying datum |
|---|---|---|---|
| **C1** | no source-grounded **tier-1** path to exit-0 byte loss or repository harm | **CONFIRMED** (independently) | Every tier-1 candidate on this axis was driven to a safe outcome: R-A/§8.3-A (8→8 survivors), R-D/§8.3-B (area left standing), R-E, and the two findings are surface-tier. No drive in either pass reached exit-0 loss through a *guarded* seam. Silent on F-1/F-2 — see §8.4 |
| **C2** | **A3-1 CLOSED** — milestone finalize supplies its own area to the shared teardown in **both** chain and squash arms; proposed regression: plant at the area root, `merged/top.txt`, `merged/docs/provenance.json`, a directory under `merged/`, run both strategies | **CONFIRMED (repro)** | §8.3-A. Driven at **`squash: true` and `squash: false`** separately. Both arms: exit 0, **4 before / 4 after**, every survivor byte-identical under `.jigc/displaced/axis-three-probe/<relative>`, all four pairs on `committed.displaced` (the directory `merged/sub` moved **whole** as one pair), both area roots empty |
| **C3** | **A3-2 CLOSED** — displacement then `unwind_area`, never `remove_dir_all`; unmoved foreign bytes structurally prevent area removal; proposed regression: occupy `.jigc/displaced/<id>` so renames fail, expect exit 0, bytes standing, one `finalize.foreign-bytes` | **CONFIRMED (repro)** | §8.3-B. Driven at Codex's **exact** shape (the per-unit dir occupied, not the parent — a cell the driver's R-D did not use): exit 0 with the commit landed (`b8b2392`), **both** plants standing in the area, area standing, exactly **one** `finalize.foreign-bytes` naming both paths, both failure reasons (`os error 20`, `os error 17`), `displaced: []` |
| **C4** | **A3-3 CLOSED** — the boundary subject expressly includes the milestone area | **CONFIRMED** | Behaviour: §8.3-A + driver row 64. Suite half read rather than driven (it is a test-content fact): `crates/cli/tests/milestone_boundary_displacement.rs:444` carries `a_landed_milestone_boundary_keeps_every_byte_of_its_own_area_it_did_not_write`, and the file mentions `milestones` **8** times — against M52's driven `grep -c 'milestones' → 0` |
| **C5** | **C-1 CLOSED** — the six-door registry is complete | **CONFIRMED** | `crates/cli/src/milestone.rs:3338` → `pub const DESTROYING_DOORS: [&DestroyingDoor; 6]`. All six are doors of driven rows (§9) |
| **C6** | **D-1 CLOSED** — task discard distinguishes milestone-owned tasks and routes to milestone finalize; uninstall renders the per-task milestone route | **CONFIRMED (repro)** | §8.3-C. `jigc task discard first-sub` → `task-discard.staged-prose`, route names ``jigc milestone finalize axis-three-probe`` *"whose boundary is the only one that commits it, so `jigc task finalize` refuses it"*; `jigc uninstall` → `uninstall.staged-prose`, per-task line *"a sub-task of milestone `axis-three-probe`, which `jigc task finalize` refuses"* naming both exits |
| **C7** | **D-2 CLOSED** — `cleanup_subtask_areas` aggregates actual removal success; discard's ack is chosen from that outcome | **CONFIRMED (repro)** | §8.3-C. `chmod 500 .jigc/worktrees` + `milestone discard --force` → exit 0, ack reads ``discarded milestone:axis-three-probe (2 sub-task(s); workbench **NOT fully removed** — the warnings above name what is left)``, both worktrees still on disk and both named with a `remedy:` |
| **C8** | **D-3 CLOSED** — staged-prose/probe failures render through the repo-relative seam | **CONFIRMED (repro)** | §8.3-C. `chmod 000 .jigc/tasks/first-sub/docs` + `milestone discard` → `milestone.foreign-bytes`, *"cannot check … Permission denied (os error 13)"*, route names `.jigc/tasks/` and `.jigc/milestones/axis-three-probe/` repo-relatively. **`command grep -c '/var/folders'` → 0** |
| **C9** | **D-4 CLOSED** — the unreadable-worktrees-root route names a `read_dir` failure, rejects the git diagnosis, names `--force` | **CONFIRMED (repro)** | §8.3-C. `chmod 000 .jigc/worktrees` + `uninstall` → `uninstall.dirty-worktree`, route reads *"this failure is a `read_dir` of that directory, **not a git fault**"*, then `git worktree list`/`remove`, then `jigc uninstall --force` |
| **C10** | the residual rule is centralized on `carries_base_pin` and consumed by active-task listing, by-id task resolution, milestone resolution/ownership, and rename's first-live-milestone scan | **CONFIRMED** | Driver R-H, R-I (rows 44, 45, 70–75) — re-driven here in part: §8.3-D shows `jigc task list` → *"no active tasks"* over a pin-less directory, and §8.3-F shows the by-id doors answering `finalize.no-task` |
| **C11** | **uninstall still sees residual directories as teardown subjects** rather than silently excluding them — it enumerates every child under `tasks/` and `milestones/`, then applies the foreign complement guard | **CONFIRMED (repro)** | §8.3-D. This is the one seam bullet **no driver row covered**. Pin-less directories planted under **both** `.jigc/tasks/` and `.jigc/milestones/`, each holding a foreign byte → `jigc uninstall` exit **1**, `uninstall.foreign-bytes`, **both** paths named, both bytes intact after |
| **C12** | `FINALIZE_MESSAGE_FILE` is a member of **both** writer rows | **CONFIRMED** | `crates/engine/src/state.rs:166` (task row) and `:224` (milestone row); behaviour driven at driver R-G with its near-miss-name and shape controls |
| **C13** | `LeftoverShape` is shape-complete; removal dispatches directories to `remove_dir_all` and every other present/fail-closed leaf shape to `remove_file` | **CONFIRMED** | Driver R-J drives all five shapes at `milestone provision` (dir · dangling `.git` · file · symlink · unreadable) plus both `--force` removal arms; §8.3-E re-drives the symlink arm at `uninstall --force` — the outside tree is **1/1 intact**, so the symlink was unlinked, not descended |
| **C14** | `--force` belongs only to refusing dispositions; **both finalize rows are `Displace` with no consent flag** | **CONFIRMED (repro)** | Driven bare: `jigc task finalize zzz --force` → **exit 2**, `error: unexpected argument '--force' found`; `jigc milestone finalize zzz --force` → **exit 2**, same. Neither `--help` lists `--force` |
| **C15** | removal-site census: *"I found **no unguarded production removal of adopter bytes**"* | **REFUTED as worded** | §8.3-G. A third party's `merged/docs/adr:user-authored.md` (`USER-BODY-KEEP`) is **destroyed at exit 0**, named on **neither** stream (`grep -c 'user-authored'` → **0** on stdout *and* stderr), absent from the whole repo and from git (`git grep -l … HEAD` → rc 1), while its sibling `plain.txt` is displaced. The site is the one Codex's own census lists as benign (`engine/milestone.rs:2228`, *"selective materialized-body removal"*). **This does not make it a defect** — it is the driver's OBS-1 and it matches the *stated* `merged/docs/` membership rule (`staged_doc_id` alone), so it contradicts no contract. What is refuted is the census sentence's **scope**, not the behaviour's correctness |
| **C16** | the diff `a3eb026b`..fixed rc.17 moves **no** schema manifest, schema file or schema JSON | **CONFIRMED** | `git diff a3eb026b..HEAD -- '*schema-manifest.yaml' '*/schemas/*' '*schema-snapshots/*' \| wc -l` → **0** |
| **C17** | the per-site classifications of the ~25-entry removal census (each named file:line read as owned/guarded/verified-empty) | **OPEN LEAD** | Not driveable per site through the binary: most sites are internal cleanup with no caller-reachable cell that distinguishes "guarded" from "never exercised", and the rig builds no state that reaches them individually. The **aggregate** claim is driven (C15). Recorded open rather than promoted on the source read |

## 8.3 · The reconciler's own repro blocks

### §8.3-A — C2, A3-1's proposed regression, **both strategies** (Codex claim 1)

```
setup: mkrig milestone; mkwork; jigc config set finalize.fan-out.squash <true|false>
       M=.jigc/milestones/axis-three-probe;  mkdir -p $M/merged/docs $M/merged/sub
       KEEP-m-top  > $M/merged/top.txt            KEEP-m-prov   > $M/merged/docs/provenance.json
       KEEP-m-root > $M/mnotes.txt                KEEP-m-nested > $M/merged/sub/nested.txt
  BEFORE-CONTROL: command grep -rl 'KEEP-' .jigc | wc -l  ->  4     (both runs)

$ jigc milestone finalize axis-three-probe --format json      -> exit 0   (squash=true)
$ jigc milestone finalize axis-three-probe --format json      -> exit 0   (squash=false)
  AFTER-CONTROL: 4                                                  (both runs, identical)
  surviving paths + bytes (identical in both arms):
    .jigc/displaced/axis-three-probe/merged/docs/provenance.json :: KEEP-m-prov
    .jigc/displaced/axis-three-probe/merged/sub/nested.txt       :: KEEP-m-nested
    .jigc/displaced/axis-three-probe/merged/top.txt              :: KEEP-m-top
    .jigc/displaced/axis-three-probe/mnotes.txt                  :: KEEP-m-root
  committed.displaced = 4 pairs, `merged/sub` carried as ONE pair (the directory, whole)
  TOPKEYS ['committed']        find .jigc/milestones .jigc/tasks -> both roots EMPTY
```

Codex's expected outcome — *"exit 0, byte-identical survivors under `.jigc/displaced/<milestone>/`, all
moved pairs in `committed.displaced`"* — holds in **both** arms, byte for byte.

### §8.3-B — C3, A3-2's proposed regression at Codex's **exact** shape

```
setup: mkrig task; a landable commit doc; code.txt staged
       PRECIOUS-A3-2-ROOT > .jigc/tasks/tidy-the-readme/notes.txt
       PRECIOUS-A3-2-DOCS > .jigc/tasks/tidy-the-readme/docs/stray.txt
       mkdir -p .jigc/displaced;  printf 'OCCUPIED\n' > .jigc/displaced/tidy-the-readme   # the PER-UNIT dir
  BEFORE: 2

$ jigc task finalize tidy-the-readme --format json            -> exit 0
  git log --oneline -1 -> b8b2392 chore: tidy the readme        # the commit LANDED
  AFTER: .jigc/tasks/tidy-the-readme/docs/stray.txt , .jigc/tasks/tidy-the-readme/notes.txt
  ls -a .jigc/tasks/tidy-the-readme -> docs  notes.txt          # the area is LEFT STANDING
  TOPKEYS ['committed','findings','schema_version']   displaced []
  findings = exactly 1:
    finalize.foreign-bytes · advisory · key {code, target:"task:tidy-the-readme"}
    "… holds 2 path(s) jigc did not write, so the working area was left standing rather than removed
     with them … 2 of them could not be moved aside: …/docs/stray.txt — could not open
     .jigc/displaced/tidy-the-readme/docs to park it: Not a directory (os error 20), …/notes.txt —
     could not open .jigc/displaced/tidy-the-readme to park it: File exists (os error 17)"
    route: "the commit landed and nothing in it is affected. Keep what you need … jigc mints no verb
     that clears it, because what is in there is not jigc's to judge"
```

All four of Codex's expectations hold, and `n_findings == 1` is the literal *"one
`finalize.foreign-bytes` identifies it"*.

### §8.3-C — C6/C7/C8/C9, the four M51 dispositions

```
### D-1  (mkrig milestone; provision; author commit:first-sub's type only -> staged prose)
$ jigc task discard first-sub        -> 1  task-discard.staged-prose
   route: … or land them with `jigc milestone finalize axis-three-probe` — this task is a sub-task of
   milestone `axis-three-probe`, whose boundary is the only one that commits it, so `jigc task
   finalize` refuses it — or … `jigc task discard first-sub --force` … settles this sub-task as
   `discarded` in the committed milestone record
$ jigc milestone discard axis-three-probe -> 1  milestone.staged-prose   (first-sub: commit:first-sub)
$ jigc uninstall                     -> 1  uninstall.staged-prose
   first-sub: commit:first-sub — a sub-task of milestone `axis-three-probe`, which `jigc task
   finalize` refuses: land it with `jigc milestone finalize axis-three-probe`, or drop it with
   `jigc task discard first-sub --force`, which also settles it as `discarded` …

### D-2  (provision; chmod 500 .jigc/worktrees)
$ jigc milestone discard axis-three-probe --force   -> exit 0
  stdout  discarded milestone:axis-three-probe (2 sub-task(s); workbench NOT fully removed — the
          warnings above name what is left)
  stderr  2 × "warning: could not remove the fan-out worktree …: Permission denied" + remedy:
  ls .jigc/worktrees -> first-sub second-sub          # both still on disk, both named
  (the warnings' host-absolute paths are the DeclaredAbsolute bound — OBS-7, graded against)

### D-3  (provision; chmod 000 .jigc/tasks/first-sub/docs)
$ jigc milestone discard axis-three-probe     -> 1  milestone.foreign-bytes
  "cannot check milestone:axis-three-probe's workbench for files jigc did not write … Permission
   denied (os error 13)";  route names `.jigc/tasks/` and `.jigc/milestones/axis-three-probe/`
  command grep -c '/var/folders' -> 0                 # no host path

### D-4  (provision; chmod 000 .jigc/worktrees)
$ jigc uninstall                              -> 1  uninstall.dirty-worktree
  route: "make sure `.jigc/worktrees/` is readable — this failure is a `read_dir` of that directory,
   not a git fault — then re-run `jigc uninstall`; or remove the worktrees yourself (`git worktree
   list`, then `git worktree remove`) … `jigc uninstall --force` deletes them with the install"
```

### §8.3-D — C11, `uninstall` over **residual** (pin-less) areas — the seam bullet no driver row covered

```
setup: mkrig fresh
       mkdir -p .jigc/tasks/ghost-residual .jigc/milestones/ghost-ms-residual      # NO base pin
       RESIDUAL-KEEP-TASK > .jigc/tasks/ghost-residual/precious.txt
       RESIDUAL-KEEP-MS   > .jigc/milestones/ghost-ms-residual/precious.txt
  BEFORE: both present

$ jigc task list      -> exit 0  "jigc task list — no active tasks"      # C10's listing leg
$ jigc uninstall      -> exit 1
  blocking · uninstall.foreign-bytes — `.jigc/` holds 2 path(s) jigc did not write — the tree is
    gitignored, so removing it would destroy bytes nothing else has a copy of:
      .jigc/tasks/ghost-residual/precious.txt
      .jigc/milestones/ghost-ms-residual/precious.txt
  AFTER: both bytes intact
```

A residual is invisible as a *work unit* and fully visible as a *teardown subject* — exactly the split
Codex claims. (This drive also re-confirms the driver's **OBS-2**: the route's parenthetical names
`.jigc/displaced/` although neither listed path is under it.)

### §8.3-E — C13's symlink arm, and driver row 35

```
setup: mkrig task; OUT=$(mktemp -d …); OUTSIDE3-KEEP > $OUT/keep.txt
       ln -s "$OUT" .jigc/tasks/tidy-the-readme/linkdir
$ jigc uninstall --force      -> exit 0
  outside tree: keep.txt :: OUTSIDE3-KEEP        # 1/1 intact — unlinked, not descended
```

### §8.3-F — F-1 and F-2 re-driven (§8.4)

Blocks inline at §8.4.

### §8.3-G — C15's falsifying datum (= the driver's OBS-1, re-driven)

```
setup: mkrig milestone; mkwork; mkdir -p $M/merged/docs
       USER-BODY-KEEP  > $M/merged/docs/adr:user-authored.md      # a third party's, colon-bearing
       USER-PLAIN-KEEP > $M/merged/docs/plain.txt                 # a third party's, plain
  BEFORE: both present

$ jigc milestone finalize axis-three-probe --format json     -> exit 0
  AFTER: .jigc/displaced/axis-three-probe/merged/docs/plain.txt      # the sibling survives
  command grep -rl 'USER-BODY-KEEP' .   -> (nothing, anywhere in the repo)
  git grep -l 'USER-BODY-KEEP' HEAD     -> rc 1                      # not in git either
  named on stdout? 0     named on stderr? 0
  stderr names only plain.txt's move;  displaced = [plain.txt pair]
```

## 8.4 · The driver's defects — status after re-drive

Codex's pass is **silent** on both (it reports no defect claim at all and its census does not reach
either surface). Under the rule they stay findings; both were re-driven here and both reproduce.

### F-1 — **CONFIRMED (re-driven)** · tier 3

```
setup: mkrig task; OUT=$(mktemp -d …); TARGET-BYTES > $OUT/body.md
       ln -s "$OUT/body.md" .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
       mkdir -p             .jigc/tasks/tidy-the-readme/docs/adr:via-directory.md
  ls -l -> lrwxr-xr-x  adr:via-symlink.md -> /var/folders/…/body.md

$ jigc task discard tidy-the-readme      -> exit 1
  blocking · task-discard.foreign-bytes — … holds 2 path(s) jigc did not write:
    .jigc/tasks/tidy-the-readme/docs/adr:via-directory.md
    .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md        <- FOREIGN here
$ jigc doc list --task tidy-the-readme   -> exit 0
  adr:via-symlink  docs/decisions/via-symlink.md  managed      <- JIGC'S OWN here
  (adr:via-directory correctly absent — M52 Inc 4 / T1 holds on the directory axis)
$ jigc uninstall                         -> exit 1  uninstall.foreign-bytes, the same 2 paths
$ jigc task discard tidy-the-readme --force -> exit 0
  stdout  discarded task tidy-the-readme — dropped staged edits to: adr:via-symlink,
          commit:tidy-the-readme (transient)
                                            ^ the entry the same command just called foreign
  stderr  warning: … discards work that is not in git:  (both paths) + not recoverable
  outside tree after: body.md :: TARGET-BYTES                  # 1/1 intact -> surface-tier, not loss
```

Three surfaces of one door disagree about one entry, in a single run. The safe direction is preserved
(the guard over-claims; the outside tree survives), which is why it stays tier 3.

### F-2 — **CONFIRMED (re-driven)** · tier 3

```
setup: mkrig fresh; OUT=$(mktemp -d …); OUTSIDE2-KEEP > $OUT/keep.txt
       mkdir -p .jigc/tasks; ln -s "$OUT" .jigc/tasks/ghost
  ls -l .jigc/tasks -> lrwxr-xr-x  ghost -> /var/folders/…        # a SYMLINK

$ jigc task discard ghost           -> exit 1
  blocking · finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` is a directory carrying no base
    pin, so it is a leftover and not a work unit …
                                  ^^^^^^^^^ it is not a directory
$ jigc task discard ghost --force   -> exit 1, byte-identical sentence
  outside tree: keep.txt :: OUTSIDE2-KEEP                          # nothing destroyed
CONTROL (the sentence is true for a real directory):
$ mkdir -p .jigc/tasks/ghost-dir; jigc task discard ghost-dir
  -> "`.jigc/tasks/ghost-dir` is a directory carrying no base pin …"
```

### The driver's observations

**OBS-1 re-driven and CONFIRMED** (§8.3-G) — and it is the datum that refutes **C15** as worded. It
remains an **observation, not a defect**: the behaviour matches the stated `merged/docs/` membership
rule, so it contradicts no contract. **OBS-2 re-driven and CONFIRMED** incidentally at §8.3-D. **OBS-7
re-driven and CONFIRMED** incidentally at §8.3-C's D-2 block (host-absolute worktree paths), and is a
declared bound, graded against. **OBS-3, OBS-4, OBS-5, OBS-6** are carried as the driver recorded them
— each driven by the driver, none contradicted by the source pass, none re-driven here.

## 8.5 · Rows marked driven that carry no §3 repro block

**Twenty** of the 78 rows rest on the table cell alone with no block in §3: **10, 11, 12, 16, 17, 18,
19, 20, 21, 22, 25, 35, 46, 56, 65, 66, 67, 69, 77, 78**.

Rather than demote them unexamined, **all twenty were re-driven here** (§8.3-C covers 18/19/25;
§8.3-D covers the residual leg; §8.3-E covers 35; the rest are in the blocks below). **Every one
reproduced**, so **no row is demoted** — each is now evidenced by a drive rather than by its cell.

```
row 10  jigc milestone provision … --format json (leftover present) -> exit 1, stdout 0 B,
        stderr flattened {"error": "blocking · milestone.leftover-holds-work …"}   (OBS-5's split)
row 11  provision "" / "../.."       -> 1 work-unit.malformed-id; .jigc intact (AGENT.md config
        milestones state tasks version worktrees)
row 12  dirty worktree + a foreign byte both present at milestone discard -> 1, and the ONLY code
        emitted is milestone.dirty-worktree (precedence driven, not declared)
row 16  an unregistered dangling-.git stray under .jigc/worktrees/ at milestone discard -> exit 0,
        "workbench removed", the stray's byte survives (S)
row 17  clean milestone discard      -> 0  "discarded milestone:axis-three-probe (2 sub-task(s);
        workbench removed)"
row 20  milestone discard --format json over a foreign byte -> 1, stdout 0 B, stderr keys
        ['findings','schema_version'], code milestone.foreign-bytes
row 21  milestone discard ""         -> 1 work-unit.malformed-id
row 22  discard a settled milestone  -> 1 milestone.terminal  "is `discarded` — a settled milestone is
        over and has no workbench"    [the driver's cell drove the `joined` variant; row 69 below
        drives `joined`. Same code, both settled states.]
row 46  task finalize, one complement entry, parking home usable -> 0, displaced = [that pair],
        findings 0, stderr names the <from> → <to>, the area is GONE
row 56  a user-made docs/adr:user-made.md at a TASK area -> exit 3, before=1 after=1
        [CORRECTION to the driver's cell: the observed codes are `conformance.section-missing`
         (×2), not `schema-conformance.*`. The row's verdict — the boundary blocks rather than
         taking it — is unaffected.]
row 65  milestone finalize narration -> "warning: removing the fan-out worktree .jigc/worktrees/
        first-sub discards work that is not in git:  .gitignore (never staged) / build/ (ignored by
        git) / scratch.txt (never staged)"
row 66  an unregistered dir AND an unregistered file under .jigc/worktrees/ -> both survive
        (STRAY-KEEP, STRAY-FILE-KEEP), named on neither stream (0 hits on stdout and stderr)
row 67  milestone finalize with nothing to land -> exit 3 milestone.zero-contribution, naming
        provision and discard as the two exits
row 69  a pin-less milestone area (base.json removed) holding PINLESS-KEEP:
          finalize -> exit 0, the plant DISPLACED to .jigc/displaced/axis-three-probe/plant.txt
          discard  -> exit 1 milestone.terminal "is `joined`"
row 77  milestone list-tasks over that area -> 0  "milestone:axis-three-probe tasks (2): first-sub,
        second-sub"
row 78  milestone execute over that area    -> 0, composes the provision step text
```

## 8.6 · Net position for this axis

- **CONFIRMED findings: 2**, both origin **driver**, both tier 3, both re-driven here (F-1, F-2).
- **CONFIRMED findings, origin codex: 0** — the source pass raised no defect claim to drive.
- **REFUTED: 1** — C15's census sentence, by the OBS-1 drive. Nothing the *driver* claimed was refuted.
- **OPEN LEADS: 1** — C17, the census's per-site classifications (not binary-reachable per site).
- **Demotions: 0** (all twenty block-less rows re-driven and reproduced); **1 table correction**
  (row 56's code cell).
- **No tier-1 row on this axis, from either pass.** Both M52 §A tier-1 rows (A3-1, A3-2) are
  **CLOSED**, each now driven twice — once by the driver on its own cells, once here on Codex's
  independently-proposed regression shapes, which reached one cell the driver's did not
  (`.jigc/displaced/<id>` occupied rather than its parent).

---

# 9 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row, in `VERB_KINDS` spelling. Thirteen leaves; the
six `DESTROYING_DOORS` members are starred.

| door | rows |
|---|---|
| `milestone provision` ★ | 1–11 · §8.3-A/C setup |
| `milestone discard` ★ | 12–23 · §8.3-C (D-1/D-2/D-3) · §8.5 rows 12/16/17/20/21/22 |
| `uninstall` ★ | 24–35 · §8.3-C (D-1/D-4) · §8.3-D · §8.3-E · §8.4 F-1 |
| `task discard` ★ | 36–45 · §8.3-C (D-1) · §8.4 F-1, F-2 |
| `task finalize` ★ | 46–56 · §8.3-B · §8.5 rows 46/56 |
| `milestone finalize` ★ | 57–69 · §8.3-A · §8.3-G · §8.5 rows 65/66/67/69 |
| `task list` | 70 · §8.3-D |
| `start` | 71, 73 |
| `milestone create` | 72 |
| `rename` | 74, 75 |
| `doc list` | 76 · §8.4 F-1 |
| `milestone list-tasks` | 77 · §8.5 row 77 |
| `milestone execute` | 78 · §8.5 row 78 |

Not doors of any reviewed row (rig setup only, recorded so the list is not read as coverage):
`milestone add-task`, `workflow`, `doc set-field`, `doc set-slot`, `config set`, `config get`.
