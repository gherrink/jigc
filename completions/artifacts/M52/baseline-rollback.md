# M52 baseline — area `rollback` (capture / restore populations in jigc's transactions)

**Provenance.** Binary `/Users/maurice/.local/bin/jigc` — `jigc 1.0.0-rc.15`, sha256
`126f1584f183636bb6cd9e782b1dc26aca28dd1afaf2fb83da1fd0e5febc8fa9`, release posture. Repo HEAD
`7637a46f1908af7cffd83cc2e5e96dc2365744a0`, tree clean, nothing written into it. Date 2026-09-16/17.
Rig states used: `committed-singletons` (×5 roots), `refs-post-hoc` (×3 roots) — every root from
`mktemp -d`, no teardown, no `rm -rf` on a variable path, no cargo run.

**Assigned rows:** C1 · C2 · C3 · C4 · DEFECT 1 · DEFECT 2 of
`completions/artifacts/M51/per-axis-review/README.md` §A ▸ Axis 4 (full table `axis-4.md` §5, §R1–R6).
Per the brief the §A rows were **not** re-verified. What follows is the **class** behind them.

**Headline.** The review's R7 states the class as **four** worktree-restore populations
(config-layer CAS · promote · retire · milestone record). Driven, it is **nine**, of which exactly
**one** is compare-and-swap. Five of the nine are *outside* the finalize executor entirely
(`rollback_rename`, `CreatedDoc::rollback`, `unwind_mint`, `unwind_unrecorded_seeds`, and the
`config set <root-knob>` relocation, which has no rollback at all) and are reached by doors the
review's 13-door axis-4 set does not include (`jigc rename`, `jigc doc author`, `jigc config set`).
Two further class facts: **C4 is two doors, not one** (`milestone create` amends and fails
unacknowledged at its own `milestone.record-exists` refusal, a cell the review never drove), and
**DEFECT 1 is two producers at two door families, not one** (`config set docs-root --format json`
breaks the same stream contract from the *leading* side).

---

## §1 · The class enumeration

### The greps, their hit counts, and what each would miss

| # | grep (run at HEAD, `--include='*.rs'`, `crates/cli/src` + `crates/engine/src`) | hits | what it misses |
|---|---|---|---|
| G1 | `-rniE 'rollback\|restore\|pre_image\|preimage\|capture\|unwind\|displaced'` | **858** lines (**299** non-comment) across 36 files | a restore written without any of those words — e.g. `git restore --source=HEAD` reached via a `git_run(&["restore", …])` array (caught by G6), and `fs::write(path, captured_bytes)` where the variable is named `bytes`/`pre` |
| G2 | `-rnE '^\s*(pub… )?fn [a-z_]*(rollback\|restore\|capture\|unwind\|pre_image\|displace)[a-z_]*'` | **29** fn defs (10 production, 19 `#[cfg(test)]`) | the two restores that are **not functions**: `Drop for RecordFlipGuard` (`milestone.rs:4449`) and the inline `fs::write` inside `rollback_promotions`' promote arm |
| G3 | `-rn 'gitignore::ensure'` | **4** production call sites (= `IGNORE_DOORS`, fenced ⇔ by `crates/cli/tests/gitignore_writer_acks.rs`) | nothing for `.gitignore`; it says nothing about the *other* jigc-owned files a door amends before failing (C4's real class) |
| G4 | `-n 'capture_record_pre_image\|commit_record_transaction\|unwind_mint\|RecordFlipGuard'` | **4** `capture_record_pre_image` sites · **4** `commit_record_transaction` sites · **3** `unwind_mint` sites · 1 `Drop` | the caller *doors*: the 4 capture sites serve **5** doors (`append_and_commit_record` serves both `add-task` and `add-from-spec`) |
| G5 | `-rn 'eprintln!\|eprint!'` (non-comment) | **71** sites | DEFECT 1's subject — of these, **12** print to stderr **without consulting `Format`** (3 × `finding_line`, 5 × `note:`, 2 × `warning:`, and the config relocation block's 7 lines counted as one producer pair) |
| G6 | `-rn '"restore"\|"reset"\|"checkout"'` | **6** (4 production) | `git update-index --cacheinfo/--force-remove`, which is how every **index** axis restores |
| G7 | `-rn 'fs::write'` / `'remove_dir_all'` in `crates/cli/src` | **170** / **44** | too coarse to be a class list; used only to confirm G1/G2 found no production restore they missed |

### The member list — every capture/restore population, driven or classified

**A · worktree-restore populations** (a failed op putting bytes back on disk). Column *racer* = what
happens to a third party's edit made inside the window.

| # | population | site | doors that reach it | restore rule | racer | git-recoverable? |
|---|---|---|---|---|---|---|
| 1 | `ConfigLayerWorktree` (`.jigc/.gitignore`, `.jigc/version`) | `task.rs:4192` | `task finalize`, `milestone finalize` (**both** arms — squash:false driven here) | **compare-and-swap** | bytes stand · pre-image parked · blocking `finalize.rollback-conflict` | tracked (after first finalize) |
| 2 | promote destination | `task.rs:4875-4899` | `task finalize`, `milestone finalize` | unconditional `fs::write` of displaced bytes · `git restore --source=HEAD` · `remove_file` for a new dest | **destroyed silently** | dest tracked → yes; same-path untracked foreign → **no** |
| 3 | retired original | `task.rs:4907-4923` | `task finalize --approve` **only** | restore **only while absent** | jigc's deletion **survives silently** | yes (`migrate` refuses an untracked source, M51 Inc 1) |
| 4 | `RecordPreImage` (milestone record) | `milestone.rs:816` | **5 doors**: `milestone create` · `add-task` · `add-from-spec` · `milestone discard` · **`task discard`** | unconditional `fs::write`, or **`remove_file`** when the pre-image was absent | **destroyed silently** (incl. a file the racer created at that path) | yes when the record was committed; **no** on the `create` arm |
| 5 | `RecordFlipGuard::drop` | `milestone.rs:4449` | `milestone finalize` (**both** arms) | unconditional `fs::write` | **destroyed silently** | yes |
| 6 | `rollback_rename` | `rename.rs:812` | `jigc rename` | `git restore --staged --worktree` (→ HEAD) for old+referrers · `remove_file` for the new path · unconditional `fs::write`/`remove_file` for `.jigc/state/file-state.json` | **destroyed silently** | new-path bytes **no**; referrer bytes yes; file-state **no** (gitignored) |
| 7 | `CreatedDoc::rollback` (staged working-area create) | `engine/src/state.rs:1174` | `jigc doc author` (and any multi-step write that persists a create) | captured pre-image, `fs::write` or `remove_file` | in-process only — no third-party window | **no** (`.jigc/tasks/` gitignored) |
| 8 | `unwind_mint` (minted work area + `tasks.json`) | `milestone.rs:852` | `milestone create`, `milestone add-task` | `remove_dir_all(area)` unconditional + `engine::state::persist(tasks.json, captured)` unconditional | **destroyed silently** (driven) | **no** (gitignored workbench) |
| 9 | `unwind_unrecorded_seeds` | `milestone.rs:1562` | `milestone add-from-spec` | `unwind_mint(area, None)` per unrecorded seed | same as 8 | **no** |
| — | `config set docs-root` / `placement-root` relocation | `config.rs:942` / `1053` | `jigc config set` | **no rollback of any kind** | n/a — the moves are not undone even when the door's own write fails | tracked (staged `R`) |

**B · index-restore populations** (5, all `git update-index`-keyed, none CAS): `owner_index` ·
`promo_index` · `config_index` (`Option`-wrapped; its drop arm is a set difference) · `record_index` ·
the retire `staged` un-stage, plus `RecordPreImage.index` reusing the third-axis primitive.
None of these is a byte-destroying axis and none was found defective here.

**C · declared no-rollback populations** (present in the class, correct by a stated rule):
`migrate-corpus` (the frame states *"the migrated bytes are written and staged"* — reviewed at A6/C4)
· `jigc setup` (M51 Inc 3 replaced a rollback with a **pre-write** refusal, `setup.dirty-install-path`)
· `gitignore::ensure`'s three non-transaction callers (`design/finalize.md` → Rollback discipline,
the M51 row's **declared bound**: *"covered by the amend … not by a rollback"*).

---

## §2 · The drives

Every row: argv, exit, observed lines, classification. Rigs as noted; `$REPO`/`$JIGC` from
`dev/jigc-rig <state> --binary ~/.local/bin/jigc`, two-step eval.

### 2.1 · Population 4 (`RecordPreImage`) — the review's C2, driven at **all five** doors

The review drove `milestone add-task` and stated the siblings as "un-driven breadth". All four
remaining doors are driven here. The racing instrument is the one `design/finalize.md` names: a
`pre-commit` hook that edits the record and exits 1.

**2.1a — `milestone create` (the *absent* pre-image arm, restore = DELETE).** `committed-singletons`.

```
hook: printf '\n<!-- raced by a concurrent editor -->\n' >> "$REPO/docs/milestone-records/axis-four-create.md"; echo HOOKNO >&2; exit 1
$ jigc milestone create 'Axis four create'                                         → exit 1
  `git commit` was rejected (no commit was made):
  HOOKNO
  nothing was committed — the record write and the milestone workbench were both rolled back,
  so nothing of milestone:axis-four-create survives. …
$ ls -l docs/milestone-records/          → total 0        (the racer's file is GONE)
$ git status --short                     → (only the unrelated ?? .jigc/config/manifest.yaml)
  rollback-conflict in output: 0
```

**Classification: latent defect (class member of C2, harder cell).** On this arm the restore is
`std::fs::remove_file` (`milestone.rs:822`), so it does not merely overwrite a third party's edit —
it **deletes a file at a path a third party wrote**, with no finding. Population 1's CAS handles the
identical *absent-pre-image* shape correctly (`ConfigLayerWorktree::restore`'s `None` arm deletes
**only while the file still holds jigc's bytes**, `task.rs:4220-4228`) — so the two sibling
populations already disagree on the same cell, in one binary.

**2.1b — `jigc task discard <sub-task> --force` (the door `design/finalize.md`'s row does not name).**

```
$ jigc task discard axis-four-sub-task --force                                      → exit 1
  `git commit` was rejected (no commit was made): HOOKNO
  nothing was committed — the milestone record still names task:axis-four-sub-task as it did,
  and the task's working area is intact. …
$ grep -c 'raced by a concurrent editor' docs/milestone-records/axis-four-milestone.md   → 0
$ diff <pre> <post>                                                                  → IDENTICAL
  rollback-conflict: 0 · .jigc/displaced: No such file or directory
```

**Classification: latent defect (a fifth caller the design row omits).** `design/finalize.md` →
Rollback discipline, the M47 row, enumerates *"`create` / `add-task` / `add-from-spec` / `discard`"* —
four doors. The code has a fifth: `task.rs:543` → `milestone::settle_discarded_sub_task`
(`milestone.rs:1778`) calls `capture_record_pre_image` + `commit_record_transaction`. **A fix cut over
the design row's list misses `jigc task discard`.**

**2.1c — `jigc milestone discard <id> --force`.**

```
$ jigc milestone discard axis-four-milestone --force                                 → exit 1
  … milestone:axis-four-milestone's record is still at its pre-discard state and its workbench
  is untouched. …
$ grep -c 'raced …' <record> → 0 ; diff vs pre → IDENTICAL ; rollback-conflict: 0
```
**Classification: latent defect (C2's class, driven).**

**2.1d — `milestone add-from-spec`.** **Not driven** — see §5.

### 2.2 · Population 5 (`RecordFlipGuard`) — the review's C3, `squash: false` arm

The review recorded the `squash: false` arm as un-driven breadth. Driven here on a milestone built
entirely by driving (create → add-task → provision → `doc create adr` + 3 × `doc set-slot` from
inside `.jigc/worktrees/area-low`), with `finalize.fan-out.squash=false`:

```
$ jigc config set finalize.fan-out.squash false
hook: appends '<!-- raced by a concurrent editor -->' to $REPO/docs/milestone-records/axis-four-squashless.md, exit 1
$ jigc milestone finalize axis-four-squashless                                       → exit 1
  `git commit` was rejected (no commit was made): HOOKNO
  milestone:axis-four-squashless is intact — nothing was committed, HEAD is at its pre-finalize
  commit, and every provisioned sub-task worktree still holds its staged code. …
$ grep -c 'raced by a concurrent editor' <record>   → 0        (the racer's line is GONE)
$ diff <pre-finalize record> <record>               → IDENTICAL
$ grep -m1 '^status:' <record>                      → status: active
  rollback-conflict: 0 · .jigc/displaced: No such file or directory
```
**Classification: latent defect (C3's un-driven arm, now driven — both commit models behave identically).**

**Control on the same fixture and the same arm** — population 1 is CAS at `squash: false` too:

```
hook: printf '# raced\n' >> "$REPO/.jigc/.gitignore"; exit 1
$ jigc milestone finalize axis-four-squashless                                        → exit 1
  … milestone:… is intact — nothing was committed, HEAD is at its pre-finalize commit …
  blocking · finalize.rollback-conflict — `.jigc/.gitignore` changed while this finalize was
    running, so the rollback did not restore it: the bytes on disk are not the ones jigc wrote
    at: .jigc/.gitignore
    route: nothing was committed and both versions are on disk: … `.jigc/displaced/.gitignore.pre-image.1789595838133263000` …
$ grep -c '^# raced' .jigc/.gitignore      → 1   $ grep -c 'my private line' .jigc/.gitignore → 1
$ find .jigc/displaced -type f             → .jigc/.gitignore.pre-image.1789595838133263000 (exists, 68 B)
```
**Classification: built + proven.** One command, one arm: CAS on two paths, silent clobber on the
record — the review's "sharpest single-command statement" reproduced at the *other* commit model.

### 2.3 · Population 6 (`rollback_rename`) — a population in **no** review row

**2.3a — the pre-existing-dirt cell is guarded (control).** `committed-singletons`.

```
$ printf '\n<!-- UNCOMMITTED USER EDIT -->\n' >> docs/decisions-log.md
$ jigc rename decisions-log:decisions-log --to 'Axis Four Decisions' --slug decisions-log   → exit 1
  blocking · rename.dirty-tree — cannot rename with a dirty working tree … : docs/decisions-log.md
  route: commit those tracked changes, or `git stash` them, then re-run the rename …
$ grep -c 'UNCOMMITTED USER EDIT' docs/decisions-log.md   → 1   (diff vs pre: IDENTICAL)
```
**Classification: built + proven.** The `git restore --staged --worktree` (→ HEAD) restore is
admissible **only** because this guard makes the pre-run worktree == HEAD for every path in
`tracked_restore`. That is a measured constraint on any fix: relaxing `rename.dirty-tree` would turn
population 6's restore into an unconditional HEAD reset over user bytes.

**2.3b — the racer cell.** `refs-post-hoc` (open task discarded first so `mid_fan_out_marker` clears).

```
hook: appends '<!-- raced by a concurrent editor -->' to $R/docs/research/axis-four-research.md
      and '<!-- racer touched the referrer -->' to $R/VISION.md ; exit 1
$ jigc rename research:context-loss --to 'Axis Four Research' --slug axis-four-research   → exit 1
  `git commit` was rejected (no commit was made): HOOKNO
  nothing was committed — the rename was rolled back, so `research:context-loss` still holds its
  original identity and every referrer still points at it. …
$ ls docs/research/                                → context-loss.md     (new path REMOVED)
$ [ -f docs/research/axis-four-research.md ]       → NO  (the racer's bytes went with it)
$ diff <pre> docs/research/context-loss.md         → IDENTICAL
  rollback-conflict: 0 ; .jigc/displaced: absent
```
**Classification: latent defect — population 6 is the same un-swept rule as populations 2/4/5, at a
door outside the review's axis-4 set.** `rename.rs:825-828` removes the new path unconditionally when
it is not at HEAD; `rename.rs:832-838` rewrites/removes `.jigc/state/file-state.json` from a captured
pre-image unconditionally. The referrer half of the claim is **not** established by this drive — this
fixture has no referrer (the ref-creating task was discarded), so `tracked_restore` held only the old
doc path; see §5.

### 2.4 · Population 8 (`unwind_mint`) — a population in **no** review row

```
hook: mkdir -p "$REPO/.jigc/tasks/area-high/docs"
      printf 'THIRD PARTY PROSE\n' > "$REPO/.jigc/tasks/area-high/docs/authored.md"
      printf '\n' >> "$REPO/.jigc/milestones/axis-four-squashless/tasks.json" ; exit 1
$ jigc milestone add-task axis-four-squashless 'area high'                            → exit 1
  `git commit` was rejected (no commit was made): HOOKNO
  nothing was committed — the record append and the sub-task mint were both rolled back,
  so milestone:axis-four-squashless is unchanged. …
$ [ -f .jigc/tasks/area-high/docs/authored.md ]   → NO — DESTROYED
$ [ -d .jigc/tasks/area-high ]                    → NO
$ diff <tasks.json pre> <tasks.json post>         → IDENTICAL   (the concurrent append is gone)
  note:/rollback-conflict lines in output: 0
```
**Classification: latent defect, and the most severe byte-loss cell in the whole class.** The
destroyed path is `.jigc/tasks/<id>/docs/` — the **gitignored** area an agent authoring through jigc
writes into, so the bytes exist in no object DB. This is the exact locus and the exact warrant the
human used in M50's `milestone discard` adjudication (*"an agent authoring through jigc writes into
`.jigc/tasks/<id>/docs/`, which the worktree probe cannot see"*), applied one door over.
`tasks.json`'s unconditional restore is separately covered by a **declared** bound
(`design/storage.md` → Concurrent writers: `tasks.json` is *atomic-but-unmerged*), so I grade the
`tasks.json` half **declared** and the working-area half **latent defect**.

### 2.5 · C4's class — every door that amends a jigc-owned file **before** its own validation can fail

Instrument: a full-repo file+sha snapshot (`find … | shasum -a 1`) before and after each refusal,
with `.jigc/.gitignore` reset to a user-trimmed 4-line file before every cell so an amend is visible.
`committed-singletons`, one milestone pre-created.

| door + cell | exit | code | `.jigc/.gitignore` after | door said so? |
|---|---|---|---|---|
| `milestone create 'Axis four base'` (duplicate) | 1 | `milestone.record-exists` | **AMENDED** — `milestones/ worktrees/ logs/ displaced/` appended | **no** (0 mentions on either stream) |
| `milestone provision nonexistent-milestone` | 1 | `milestone.unknown` | **AMENDED** — same four lines | **no** (0 mentions) — the review's C4 |
| `milestone add-task nonexistent-milestone 'x'` | 1 | `milestone.unknown` | untouched | n/a (not an `IGNORE_DOORS` member) |
| `jigc setup` over a modified `.jigc/AGENT.md` | 1 | `setup.dirty-install-path` | **untouched** | n/a — M51 Inc 3's pre-write guard holds |
| `task finalize nonexistent-task` | 1 | `finalize.no-task` | untouched | n/a |
| `rename bogus:bogus --to X` | 1 | `store.unknown-type` | untouched | n/a |
| `migrate-corpus` (all current) | 0 | — | untouched | n/a |
| `validate` | 0 | advisory only | untouched | n/a |
| `milestone join nonexistent-milestone` | 1 | `milestone.unknown` | untouched — but **writes `.jigc/index/edges.json`** | n/a (a rebuildable derived cache; M49's read-verb rule permits exactly this shape) |

**Classification: C4 is a 2-door class, not 1 — `milestone create` is a latent defect in its own
right at a cell the review never drove.** The review's R4 re-graded driver row A13 (`create` +
*hook rejection*) as C4's `create` instance. The **cheapest and most likely** cell is not a hook at
all: a duplicate title, exit 1, `milestone.record-exists`, four lines appended, zero notice. The
`IGNORE_DOORS` row for `create` declares its ack to be *"the `minted milestone:` ack itself"*, which
that run never prints. The survival itself is the **declared bound** in `design/finalize.md`
(the M51 CAS row's closing sentence) — so the finding is the *disclosure*, not the amend.

**The class's fourth member is not a `.gitignore` door at all** (§2.6).

### 2.6 · LATENT DEFECT — `jigc config set <root-knob>` moves committed docs, then fails, and nothing puts them back

`refs-post-hoc`. `write_scalar` is forced to fail by `chmod -w .jigc/config` (the door's last act,
`config.rs:514`, after both relocation floors have already run).

```
$ chmod -w .jigc/config
$ jigc config set docs-root papers                                                    → exit 1
  (stderr) relocating 1 committed doc(s) stranded by the `docs-root` re-point to `papers` …
  (stderr)   - docs/research/context-loss.md → papers/research/context-loss.md
  (stderr) could not write /private/var/.../repo/.jigc/config/manifest.yaml: Permission denied (os error 13)
$ chmod +w .jigc/config
$ git status --short        → R  docs/research/context-loss.md -> papers/research/context-loss.md
$ jigc config get docs-root → docs-root = docs/  (pack-default)      ← the knob did NOT land
$ jigc doc list             → 4 rows; research:context-loss is GONE from the store index
$ jigc doc show research:context-loss                                                 → exit 1
  blocking · store.not-found — could not read `research:context-loss` at
  `docs/research/context-loss.md`: No such file or directory (os error 2)
$ grep -o '"[^"]*context-loss[^"]*"' .jigc/state/file-state.json → "papers/research/context-loss.md"
$ jigc validate                                                                       → exit 0
  advisory · file-state.orphaned-doc — committed doc `papers/research/context-loss.md` sits outside
    the resolved doctype roots — a `docs-root` change likely stranded it …
```

**Classification: latent defect, in no §A row.** The door mutates the worktree, the index **and**
the file-state baseline before the write that can fail, has **no** rollback, and its failure message
names only the manifest write. The end state is a managed doc unreachable through every jigc read
surface at exit 1, graded **advisory at exit 0** by `jigc validate`. It is recoverable (the route
names the re-point), so this is store-inconsistency rather than data loss — but it is the same
*shape* as C4 one severity tier up, and `config set` is not in the review's axis-4 door set.
The absolute host path in the error is **declared** — `crates/cli/src/config.rs` is an
`UNSWEPT_PRODUCERS` member (`crates/cli/tests/repo_relative_paths.rs:764`, reason: *"`with_context`
I/O faults on cascade-layer writes — an error channel, not a finding"*).

### 2.7 · DEFECT 1's class — every producer that prints to stderr without consulting `Format`

`grep -rn 'eprintln!\|eprint!'` → **71** non-comment sites; of those, the ones that can co-occur with
a **stderr-borne JSON document** (the reject class, where stdout is empty) are:

| producer | site | reachable with an envelope on stderr? |
|---|---|---|
| `carry_rollback_conflicts` | `task.rs:4330` | **yes — DEFECT 1**, driven by the review at both finalize doors |
| the `docs-root` / `placement-root` relocation narration | `config.rs:983,1007,1014` / `1103,1112,1119,1122` | **yes — driven here** |
| `milestone join`'s blocking loop | `milestone.rs:3753` | no (join's JSON ack is on stdout — swept below) |
| `milestone::blocked`'s text arm | `milestone.rs:4697` | no (it branches on `Format::Json`) |
| `note:` ×5 (`post_commit` ×3, `unwind_mint` ×2) | `task.rs:4943-4949`, `milestone.rs:856,861` | `post_commit`'s three are post-commit (stdout ack); `unwind_mint`'s two fire **on the reject path** — see §5 |
| `warning:` ×2 (`pack.rs:1786,1793`), `warning: fan-out worktree path …` (`milestone.rs:4595`) | — | not driven |

**Driven — the second instance, at a door family outside axis 4:**

```
$ chmod -w .jigc/config
$ jigc config set docs-root papers --format json > out.json 2> err.txt ; echo $?     → 1
$ wc -c < out.json                     → 0            (stdout empty: the reject class)
$ cat err.txt
  relocating 1 committed doc(s) stranded by the `docs-root` re-point to `papers` (…):
    - docs/research/context-loss.md → papers/research/context-loss.md
  {
    "error": "could not write /private/var/.../manifest.yaml: Permission denied (os error 13)"
  }
$ python3 -c "import json;json.load(open('err.txt'))"
  json.decoder.JSONDecodeError: Expecting value: line 1 column 1 (char 0)
```

**Classification: latent defect — DEFECT 1's class is ≥2 producers at ≥2 door families, and the two
break the contract from opposite sides.** `carry_rollback_conflicts` appends → `Extra data: line 4
column 1`; the relocation narration prepends → `Expecting value: line 1 column 1`. A fix keyed on
`carry_rollback_conflicts`' doc-comment premise fixes one of them. Contract:
`design/command-output-contract.md` → *Stream discipline* (*"the JSON-bearing stream parses as
exactly one document"*) plus the pinned driver predicate *"parse stdout; if stdout is empty, parse
stderr."*

**Control sweep — the reject class is otherwise clean** (same rig, one command each):

```
milestone provision nonexistent --format json  exit=1  stdout 0B   stderr 552B parses OK
milestone create <dup>          --format json  exit=1  stdout 0B   stderr 290B parses OK
task finalize nonexistent-task  --format json  exit=1  stdout 0B   stderr 488B parses OK
rename bogus:bogus --to X       --format json  exit=1  stdout 0B   stderr 148B parses OK
milestone join nonexistent      --format json  exit=1  stdout 0B   stderr 552B parses OK
validate                        --format json  exit=0  stdout 813B parses OK, stderr 0B
```

### 2.8 · Population 7 (`CreatedDoc::rollback`) — built + proven

Driven on the `area-low` sub-task worktree of the milestone fixture, over a **same-identity staged
copy** (the M47 Inc 6 cell):

```
$ printf 'title: Eviction policy\nsections:\n  - id: context\n    set:\n      context: |-\n        <<SECOND ATTEMPT CONTEXT>>\n  - id: decision\n    set:\n      decision: BAD-NO-MARKERS\n' \
  | jigc doc author adr --from-file - --task area-low                                → exit 1
  blocking · write.malformed-value — write rejected: the value for slot `decision/decision`
    must be wrapped in <<…>> … — got: BAD-NO-MARKERS
    route: retry `jigc doc author adr` with a conforming value
$ grep -c 'axis four context prose' .jigc/tasks/area-low/docs/adr:eviction-policy.md   → 1
$ grep -c 'SECOND ATTEMPT CONTEXT' …                                                   → 0
$ diff <pre> …                                                                         → IDENTICAL
```
**Classification: built + proven.** The captured-pre-image discipline holds; the editing session's
prior work survives byte-identical and the failed attempt leaks nothing. Its window is intra-process,
so it has no third-party racer cell.

---

## §3 · What changed against the review's rows

1. **The class is nine worktree-restore populations, not four** (R7's table). Five are outside the
   finalize executor: `rollback_rename`, `CreatedDoc::rollback`, `unwind_mint`,
   `unwind_unrecorded_seeds`, and the rollback-less `config set <root>` relocation. Three doors that
   reach them — `jigc rename`, `jigc doc author`, `jigc config set` — are **not in the review's
   13-door axis-4 set**, so a fix scoped to that set misses them by construction.
2. **C2's door count is five, not four, and `design/finalize.md` carries the four.** The M47 row
   enumerates *"`create` / `add-task` / `add-from-spec` / `discard`"*; `jigc task discard`'s
   sub-task settlement (`task.rs:543` → `milestone.rs:1778`) is the fifth, driven at §2.1b. **A
   locked artifact's enumeration is incomplete at HEAD** — the first-rank shape.
3. **C2's `create` arm is a different mechanism from its `add-task` arm**, and the harder one: the
   pre-image is *absent*, so the restore is `remove_file` — jigc **deletes** a file a third party
   created at that path. Population 1's CAS already answers this exact cell correctly, in the same
   binary, twelve hundred lines away.
4. **C3's `squash: false` arm is driven and behaves identically** (the review recorded it as
   un-driven breadth). So is population 1's CAS control at that arm — one `milestone finalize
   --squash=false` run CAS-protects two files and silently clobbers the record.
5. **C4 is two doors and ≥6 failure points, not one door.** `milestone create` amends and refuses
   unacknowledged at `milestone.record-exists` — a duplicate title, no hook, exit 1. The review only
   ever drove `create`'s hook cell (A13/R4).
6. **DEFECT 1 is two producers and breaks the stream from both sides** (§2.7). A leading-garbage
   instance exists at `jigc config set <root-knob> --format json`.
7. **`retire`'s already-absent arm reaches exactly one door.** `plan.retirements` is populated only
   by a migration plan (`task.rs:2262`, `3279`, `4907`); `task finalize --approve` is the sole door
   that runs `retire`. I did **not** drive a "no retire phase" proof at the other twelve — the
   review's E4 records that and the source agrees, so I carry it as a source-derived constraint,
   marked as such. Within that one door the arm is silent at every cell, as the review drove.
8. **Measured constraint on "one shared primitive" (not a recommendation).** Three things a single
   CAS primitive would have to absorb, each measured:
   - **Subject shape.** Populations 8 and 9 restore a **directory tree**, not a byte string
     (`remove_dir_all`). That is the same shape `design/finalize.md` uses to justify excluding
     `.jigc/config` from `CONFIG_LAYER_SPECS ▸ Rewritten` (*"a worktree pre-image over a directory
     is a different shape — absent-means-delete over N files, including files this run never saw"*),
     so the stated reason for one exclusion applies verbatim to two more populations.
   - **Trackedness changes what a conflict *means*, not whether one exists.** Populations 1, 2
     (tracked dest), 3, 4 (committed record), 5 and 6's referrer half restore paths git tracks — a
     lost racer edit is recoverable from the object DB (the review's R6 bound). Populations 6's
     file-state half, 7, 8, 9 and 2's same-path-untracked-foreign cell restore **gitignored or
     untracked** paths where nothing recovers the bytes. The park-and-name mechanism
     (`.jigc/displaced/`, itself gitignored) works for both; it is the **severity ordering** that
     differs, not the applicability.
   - **Two populations already restore correctly under a *door* guard rather than a CAS.**
     `rename`'s HEAD-sourced restore is admissible only because `rename.dirty-tree` refuses first
     (§2.3a), and `setup` replaced a rollback with a pre-write refusal (M51 Inc 3). So the class
     admits two shapes of answer, and "make them all CAS" is not forced by what I drove.
9. **What the review's R7 bound understates.** R7 says the third party is "a hook or a concurrent
   process, not an interactive edit". For populations 8 and 9 the third party is neither: it is
   **another jigc agent** writing into `.jigc/tasks/<id>/docs/` in a fan-out — the population whose
   isolation `design/storage.md` → *Concurrent writers* already carves out.

---

## §4 · Latent defects not in any §A row

| # | one line | argv (full repro in §2) |
|---|---|---|
| L1 | `milestone create`'s record rollback **deletes** a file a racer created at the record path (absent-pre-image arm), no finding | `jigc milestone create 'Axis four create'` under a hook that writes the record path and exits 1 → §2.1a |
| L2 | `jigc task discard <sub-task> --force` is a **fifth** `RecordPreImage` caller, unnamed in `design/finalize.md`'s row; racer's record bytes destroyed silently | `jigc task discard axis-four-sub-task --force` under the record-racing hook → §2.1b |
| L3 | `milestone discard` same | `jigc milestone discard axis-four-milestone --force` → §2.1c |
| L4 | `RecordFlipGuard` clobbers identically at **`squash: false`** (the review's un-driven arm) | `jigc config set finalize.fan-out.squash false; jigc milestone finalize axis-four-squashless` → §2.2 |
| L5 | `jigc rename`'s rollback (`rollback_rename`) is a **sixth** population — the new path is removed unconditionally with the racer's bytes in it; `.jigc/state/file-state.json` is rewritten unconditionally; no finding | `jigc rename research:context-loss --to 'Axis Four Research' --slug axis-four-research` under a hook that writes the new path → §2.3b |
| L6 | `unwind_mint` `remove_dir_all`s the minted area, **destroying a third party's authored prose in the gitignored `.jigc/tasks/<id>/docs/`** — no git copy, no note, no finding | `jigc milestone add-task axis-four-squashless 'area high'` under a hook that writes into `.jigc/tasks/area-high/docs/` → §2.4 |
| L7 | `milestone create` amends `.jigc/.gitignore` then refuses at **`milestone.record-exists`** (no hook needed) with zero notice — C4's second door | `jigc milestone create '<existing title>'` over a trimmed `.jigc/.gitignore` → §2.5 |
| L8 | `jigc config set <root-knob>` stages the `git mv`s and re-keys file-state, then fails its own write with **no rollback** — the doc leaves `doc list`/`doc show` at exit 1 while `validate` grades it advisory at exit 0 | `chmod -w .jigc/config; jigc config set docs-root papers` → §2.6 |
| L9 | DEFECT 1's class has a **second producer**: `config set <root-knob> --format json` prepends relocation prose to a stderr-borne `{"error": …}` → `Expecting value: line 1 column 1` | `chmod -w .jigc/config; jigc config set docs-root papers --format json` → §2.7 |

---

## §5 · Honest bounds

1. **`milestone add-from-spec` was not driven at any cell of this area.** It needs a committed
   `spec` doc the rig states do not carry, and building one costs a full author→finalize arc. Its
   `RecordPreImage` half shares `append_and_commit_record` with `add-task` (driven by the review) and
   its `unwind_unrecorded_seeds` half shares `unwind_mint` (driven at §2.4) — but neither is a drive
   of that door, and I do not present it as one.
2. **`rollback_rename`'s referrer half is not driven.** The `refs-post-hoc` fixture's ref-creating
   task had to be discarded to clear `mid_fan_out_marker`, which left the corpus with **no**
   referrer, so `tracked_restore` carried only the old doc path. The `git restore --staged --worktree
   <referrer>` claim is a source read at `rename.rs:822`, not a measurement.
3. **`unwind_mint`'s two `note:` lines were not driven.** They fire only when `remove_dir_all` or
   `engine::state::persist` *itself* fails, which needs a manufactured I/O fault I did not build. So
   their interaction with the stderr JSON envelope (a third DEFECT-1-class candidate) is a lead.
4. **No population was driven against a genuine concurrent *process*.** Every racer here is the
   user's own `pre-commit` hook — the instrument `design/finalize.md` names — so nothing here
   measures the scheduler, and nothing here measures the real fan-out cell of L6 (N agents against
   one `.jigc/`), only its shape.
5. **The promote axis at a *new* destination raced was driven by the review at `milestone finalize`
   (R1) and not re-driven at `task finalize`** — I spent the fixture budget on the five undriven
   populations instead. Same code (`task.rs:4895`), but that is inference, not a drive.
6. **`.jigc/version` was not raced at any door** beyond the review's B2. The doc-comment's claim that
   the fan-out arms never reach `refresh_version_stamp` is a source read here; my squash:false
   control raced `.jigc/.gitignore` only.
7. **I did not drive the twelve doors that "carry no retire phase"** (§3.7) — I read
   `plan.retirements`' one population site and carried the review's E4.
8. **The index-restore populations (B in §1) were not driven at all.** They are not byte-destroying
   and no assigned row touches them; `git ls-files -s` comparisons were left to the review's C-cells.
9. **`chmod -w` is a manufactured failure point.** It is the only deterministic way I found to fail
   `write_scalar` after the relocation floors run. The *relocation* half of §2.6 and §2.7 is ordinary
   behaviour; only the failing write is manufactured, and it is declared here rather than hidden.
10. **No cargo was run and no test was read for its assertions** — every classification above rests
    on the installed release binary plus the design sentence cited beside it.
