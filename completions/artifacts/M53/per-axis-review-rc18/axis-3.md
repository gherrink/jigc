<!-- AXIS 3 · destroying doors — RECONCILED. Every row driven on the installed `/Users/maurice/.local/bin/jigc` -> `jigc 1.0.0-rc.18` (repo HEAD `271b0cb7`), 2026-09-23. Copied verbatim from the reconciler; this header line is the only addition. -->
<!-- M53 SECOND partial per-axis review · axis 3 · destroying doors · RECONCILED (Opus driver table × Codex
     source pass). Every reconciliation drive ran on /Users/maurice/.local/bin/jigc -> `jigc 1.0.0-rc.18`,
     repo HEAD `271b0cb7`, 2026-09-23. No fix applied, no commit made, nothing written into the repository. -->

# M53 second partial per-axis review — AXIS 3 · destroying doors — RECONCILED

**The binary, asserted first, before anything else ran.**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.18
```

**Inputs.** The Opus driver's `(door, cell)` table (`driver/axis3.md`, 75 rows over 15 doors, its §3 repro
blocks R-1 … R-16 and its §4–§9 bounds) and the Codex source pass (`codex/axis3-codex.md`, reviewed at
`1cef812d`, no writes/builds/driving, prompt at
`completions/artifacts/M53/per-axis-review-rc18/instrument/axis3-prompt.md`).

**The rule applied** (acceptance-design.md → *The reconciliation rule*): a claim by one that the other
cannot reproduce is a **lead**, not a finding. Every Codex claim was entered as `lead(codex, …)` and then
**driven** — to a repro block (CONFIRMED, origin codex) or to a refutation with the falsifying datum. Every
driver defect the source pass contradicts or is silent on was **re-driven by me**, and its own repro block
confirmed. Nothing was promoted on a source read.

**Reconciliation drives.** Ten repro blocks, X-1 … X-10 (§3 below), all in throwaway rigs from
`dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc` (two-step eval; every root from `mktemp -d`,
so nothing needed teardown and the `rm -rf $V/$D` shape appears nowhere). Exit codes read **bare**. Every
loss/survival claim uses `command grep` with a **before-control** quoted beside the after-count.

**Headline of the reconciliation.**

- **Both Codex claims CONFIRMED** on my own drives (F-1, F-2 — both the driver's own still-open tier-3
  rows, so the two instruments agree, and the agreement is now double-driven).
- **All three driver defects CONFIRMED** on my own drives (F-3, F-4, F-5). The source pass is silent on
  F-4 and F-5; its one blanket negative is **REFUTED on its repository-harm half** by F-3's repro.
- **22 of the driver's 75 rows carried no repro block** and are demoted by the stated rule. I re-drove
  **all 22**; every one matched the driver's stated surface, so all 22 are **restored** as
  CONFIRMED-by-reconciler with a repro block of their own. **Doors covered stays 15.**
- **Tier-1 rows on this axis after reconciliation: 0.** Every tier-1 candidate reached a safe outcome with
  a before-control on *my* drives too (A3-1: 8 before / 8 after; A3-2: both areas left standing; the
  operation leg: nothing removed, marker and worktree on disk; F-3: every deleted byte tracked or
  jigc-regenerable; F-5: HEAD byte-unmoved).

---

## 1 · The driver's `(door, cell)` table — carried unchanged, with the repro column that records the demotions

The only change to the driver's table is the **last column**. It names the repro block that evidences each
row: `R-n` = the driver's own §3 block; `X-n` = a reconciliation block in §3 below. A row that carried
**no** repro block in `driver/axis3.md` is marked **DEMOTED** — per the rule, such a row is not driven — and,
where I re-drove it, the same cell records the restoring block. Every demoted row was re-driven; none was
refuted.

### 1.1 · `milestone provision` (13 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 1 | S1 dir, no `--force` | `jigc milestone provision axis-three-probe` | 1 | `milestone.leftover-holds-work` | Mechanical | `…: precious.txt — git reports no worktree of its own there…`; plant on disk after | matches contract | R-10 |
| 2 | S2 dangling `.git` | same | 1 | same | Mechanical | `…: .git, wip.txt — git cannot read a repository there` | matches contract | R-10 |
| 3 | S5 plain file | same | 1 | same | Mechanical | `…: the file itself — it is a file, not a worktree` | matches contract | R-10 |
| 4 | S5 symlink | same | 1 | same | Mechanical | identical *the file itself* wording; outside tree 1/1 intact | matches contract | R-10 |
| 5 | S6 unreadable (`chmod 000`) | same | 1 | same | Mechanical | `…: unknown — could not read the leftover directory …: Permission denied (os error 13)` | matches contract | R-10 |
| 6 | S1, `--force` | `… --force` | 0 | none | Informational | `warning: removing the leftover directory … precious.txt` + `not recoverable`, then `provisioned 2 worktree(s)` | matches contract | R-10 |
| 7 | S5 symlink → outside tree, `--force` | `… --force` | 0 | none | Informational | `warning: removing the leftover file …`; outside tree **1/1 intact** | matches contract | R-10 |
| 8 | S11 removal fails (read-only **parent**), `--force` | `… --force` | 1 | `milestone.provision-failed` | Mechanical | loss warning names `precious.txt`, which **is** gone; `0 of 2 worktree(s) landed` | matches contract | R-10 |
| 9 | S11 removal fails (read-only **leftover**), `--force` | `… --force` | 1 | `milestone.provision-failed` | Mechanical | **no loss warning at all**; `precious.txt` survives | matches contract | R-10 |
| 10 | **S19** unregistered path = **a second repository's** linked worktree, mid-bisect, tree clean | `jigc milestone provision axis-three-probe` | 1 | `milestone.leftover-holds-work` | Mechanical | `…: a bisect git has left un-concluded (abandon it with \`git -C … bisect reset\`) …`; the other repo's worktree + admin record intact | the review's largest cell — CLOSED | R-2 |
| 11 | **S19** the same, `--force` | `… --force` | 0 | none | Informational | `warning: removing the fan-out worktree …` / `a bisect git had left un-concluded …` / `not recoverable`; the other repo's bytes intact | matches contract | R-2 |
| 12 | **S19** a **registered** worktree carrying an operation (idempotent re-provision) | `jigc milestone provision axis-three-probe` | 0 | none | none | `provisioned 2 worktree(s) …`; `BISECT_LOG` and the worktree **untouched** | matches contract | **DEMOTED → re-driven X-9 · RESTORED** |
| 13 | malformed / empty id | `jigc milestone provision ""` · `"../.."` | 1 | `work-unit.malformed-id` | Human | `.jigc/` intact | matches contract | **DEMOTED → re-driven X-9 · RESTORED** |

### 1.2 · `milestone discard` (12 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 14 | **S19** own worktree, **spotless tree**, `Bisect` | `jigc milestone discard axis-three-probe` | 1 | `milestone.dirty-worktree` | Human | the hold line + `registered here, so the teardown removes it and this content is destroyed`; worktree + marker on disk after | the post-review HIGH — CLOSED | R-1 · **re-driven X-7** |
| 15 | **S19** the same, `Rebase` at a `break` | same | 1 | same | Human | `a rebase git has left un-concluded (abandon it with \`git -C … rebase --abort\`)` | CLOSED | R-1 |
| 16 | **S19** the same, **dangling `sequencer/`** | same | 1 | same | Human | `a cherry-pick or revert git has left un-concluded (… \`cherry-pick --quit\`)` | CLOSED | R-1 |
| 17 | **S19** `Rebase`, `--force` | `… --force` | 0 | none | Informational | the operation narrated + `not recoverable`; then `workbench removed` | matches contract (consent) | **DEMOTED → re-driven X-9 · RESTORED** (cell variance: driven over `Bisect` + tree dirt, not `Rebase`; the row's claim holds) |
| 18 | **control** — clean **and concluded** worktree | same | 0 | none | none | `discarded milestone:axis-three-probe (… workbench removed)` — zero false fire | matches contract | R-1 · **re-driven X-7** |
| 19 | S3 dirty worktree (precedence over foreign/prose) | same | 1 | `milestone.dirty-worktree` | Human | both worktrees named with their `git status` codes; dirt **and** operation composed on one line | matches contract | **DEMOTED → re-driven X-9 · RESTORED** |
| 20 | **S16** the `merged/` complement + S12 + S13, no `--force` | same | 1 | `milestone.foreign-bytes` | Human | **all 7** named; before=7 after=7 | A3-1 cell (i) CLOSED | R-5 |
| 21 | same, `--force` | `… --force` | 0 | none | Informational | **two** `warning: removing the working area …` blocks, one per area; after=0 | matches contract | R-5 |
| 22 | **S18** terminal milestone | `jigc milestone discard axis-three-probe` after a landed boundary | 1 | `milestone.terminal` | Human | `is \`joined\` — a settled milestone is over and has no workbench` | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 23 | **control** — only `finalize-message.tmp` in **both** areas | same | 0 | none | none | `workbench removed`; `foreign-bytes` **does not fire** | matches contract | R-11 |
| 24 | **control** — near-miss **names** | same | 1 | `milestone.foreign-bytes` | Human | **both** named | matches contract | R-11 |
| 25 | **control** — a **directory** wearing the name | same | 1 | `milestone.foreign-bytes` | Human | named | matches contract | R-11 |

### 1.3 · `uninstall` (11 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 26 | **S19** own worktree, spotless, `Bisect` | `jigc uninstall` | 1 | `uninstall.dirty-worktree` | Human | the hold line, `registered as a worktree of this repository`; route carries the operation clause | CLOSED | R-1 · **re-driven X-7** |
| 27 | **S19** the same, `--force` | `… --force` | 0 | none | Informational | the operation narrated before the removal; worktree dir gone, **admin record survives** | matches contract | **DEMOTED → re-driven X-9 · RESTORED** |
| 28 | **S16** the `merged/` complement + S12 | `jigc uninstall` | 1 | `uninstall.foreign-bytes` | Human | all 7 named, identical set to row 20 | matches contract | R-5 |
| 29 | S8 untracked workbench file | same | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/notes.txt`; route names `git add <path>` | matches contract | R-13 |
| 30 | S15 plain **file** at an `ENTRIES` name | same | 1 | `uninstall.untracked-workbench-file` | Human | `.jigc/logs` named | matches contract | R-13 |
| 31 | S15 non-empty `.jigc/displaced/` | same | 1 | `uninstall.foreign-bytes` | Human | `.jigc/displaced/some-task/keep.txt` | matches contract | R-13 |
| 32 | S15, `--force` | `… --force` | 0 | none | Informational | `warning: removing the relocation workbench …`; after=0 | matches contract | R-13 |
| 33 | S9 clean, second run (idempotency) | `jigc uninstall` | 0 | none | none | the header line, no removals | matches contract | R-13 |
| 34 | symlink inside a task area → outside tree, `--force` | `… --force` | 0 | none | Informational | outside tree byte-intact after | matches contract | **DEMOTED → re-driven X-10 · RESTORED** |
| 35 | **S20** run from **inside a fan-out worktree** | `jigc uninstall` (cwd = `.jigc/worktrees/first-sub`) | **0** | none | none | claims the install was removed; main `.jigc/`, `CLAUDE.md`, `SKILL.md` still present; the **shared** `.git/hooks/pre-commit` **gone** | **DEFECT F-3** | R-7 · **re-driven X-2** |
| 36 | **S20** the same, over untracked bytes under the **worktree's own** `.jigc/` | same | 1 | `uninstall.foreign-bytes` | Human | `.jigc/tasks/hand-made/notes.txt`; both plants intact after | matches contract (keeps F-3 off tier 1) | R-7 · **re-driven X-2** |

### 1.4 · `task discard` (9 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 37 | **S12** the whole complement shape space (8 shapes) | `jigc task discard tidy-the-readme` | 1 | `task-discard.foreign-bytes` | Human | **all 8** named | matches contract | R-12 |
| 38 | S12, `--force` | `… --force` | 0 | none | Informational | all 8 + `not recoverable`, then the ack; outside tree intact | matches contract — see F-1 | R-12 · R-16 · **re-driven X-1** |
| 39 | **control** — jigc's own files only | `jigc task discard tidy-the-readme` | 1 | `task-discard.staged-prose` only | Human | `foreign-bytes` **does not fire** | matches contract | R-12 |
| 40 | **control** — only `finalize-message.tmp` added | same | 1 | `task-discard.staged-prose` only | Human | the transient is jigc's | matches contract | R-11 · R-12 |
| 41 | malformed / empty id | `jigc task discard ""` · `"../.."` | 1 | `work-unit.malformed-id` | Human | `.jigc/` intact | matches contract | R-12 |
| 42 | **S18** residual (bare `mkdir`) | `jigc task discard ghost-residual` | 1 | `finalize.no-task` | Human | residual sentence + repo-relative route; HEAD unmoved | matches contract | R-16 (control only) · **re-driven X-1** (full surface) |
| 43 | **S18′** a settled sub-task's leftover, pin intact, **no** foreign byte | `jigc task discard second-sub` | 1 | `milestone.terminal` at **`task:second-sub`** | Human | *already `joined` … a leftover, not live work*; HEAD unmoved | matches contract | R-9 · **re-driven X-4** |
| 44 | the same **with** a foreign byte (precedence) | same | 1 | `task-discard.foreign-bytes` | Human | the foreign arm wins over the terminal arm | matches contract | **DEMOTED → re-driven X-10 · RESTORED** |
| 45 | **S18′** `--force` at a settled sub-task | `… --force` | 1 | `milestone.terminal` | Human | byte-identical refusal; `--force` does **not** consent past a terminal record | matches contract — makes F-5's route a dead end | R-9 · **re-driven X-4** |

### 1.5 · `task finalize` — `Displace` (8 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 46 | **S17 all move** | `jigc task finalize tidy-the-readme --format json` | 0 | none | Informational | `committed.displaced` carries the pair; `<from> → <to>` on stderr; area gone | matches contract | **DEMOTED → re-driven X-10 · RESTORED** |
| 47 | **S17 none move**, `.jigc/displaced` a regular **file** | `… --format json` | 0 | `finalize.foreign-bytes` (advisory) | Human | area **left standing**; before=1 after=1; `Not a directory (os error 20)`; key `{code, target:"task:tidy-the-readme"}` | A3-2 CLOSED | R-6 |
| 48 | **S17 some move**, one entry's parent occupied | `… --format json` | 0 | `finalize.foreign-bytes` | Human | one block holding `held 2 entries`, the move made and the one it could not, `File exists (os error 17)` | matches contract | R-6 |
| 49 | the **hook writes into the area during the commit** | finalize with a succeeding `pre-commit` | 0 | none | Informational | `displaced = [hookfile.txt pair]`, `findings = []`, area removed clean | matches contract — see OBS-4 | R-14 |
| 50 | commit **rejected** by a hook, plant present | finalize with a rejecting `pre-commit` | 1 | (rejection frame) | Human | *nothing was committed …*; before=1 after=1; `.jigc/displaced` **not created** | matches contract | R-14 |
| 51 | **control** — ordinary lifecycle, no plant | `… --format json` | 0 | none | none | `displaced: []`, `findings: []`, `.jigc/displaced` **absent**, `.jigc/tasks` empty | matches contract | **DEMOTED → re-driven X-10 · RESTORED** |
| 52 | **S18′** a settled sub-task's leftover, no commit doc | `jigc task finalize second-sub` | 3 | `finalize.render-io` | Human | route names `jigc task discard second-sub --force`, which row 45 shows **refuses**; `at:`/`key.target` carry a **host-absolute path** | **DEFECT F-5**; host path = OBS-8 (declared) | R-9 · **re-driven X-4** |
| 53 | **S18′** the same after *"make a change, then re-run"* | same with `late.txt` staged | 3 | `finalize.render-io` | Human | HEAD **byte-unmoved**, record still `joined` | safe; the route is F-5 | R-9 · **re-driven X-4** |

### 1.6 · `milestone finalize` — `Displace` (10 rows)

| # | cell | argv | exit | code | route | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 54 | **S13 + S16**, the A3-1 re-drive: 8 planted loci | `jigc milestone finalize axis-three-probe --format json` | 0 | none | Informational | **8 before / 8 after**; all 8 pairs on `committed.displaced` sorted by `from`; both area roots empty | A3-1 CLOSED | R-4 · **re-driven X-5** |
| 55 | **S16 foreign directory** `merged/sub/` and the area-root `sub/` | same | 0 | none | Informational | each moved **whole** as one pair; inner bytes intact | matches contract | R-4 · **re-driven X-5** |
| 56 | the colon-less `merged/docs/adr.md` / `provenance.json` cells | same | 0 | none | Informational | neither resolved to a doctype; both displaced | matches contract | R-4 · **re-driven X-5** |
| 57 | **D1×D2** — a `merged/` byte whose **move failed** survives a landed boundary | finalize with `.jigc/displaced` a regular file | 0 | `finalize.foreign-bytes` ×2 (**stderr-only**) | Human | 3/3 plants on disk; one advisory **per standing area**; envelope keys `['committed']`, `displaced: []`; 0 host paths | A3-2 CLOSED at the second door | R-6 · **re-driven X-6** |
| 58 | **S19** a provisioned sub-task worktree mid-bisect, tree clean | `jigc milestone finalize zeta-probe` | 3 | `repo.operation-in-progress` | Human | the site clause **leads**; `at: .jigc/worktrees/zed-two` | the post-review fix's own claim — holds | R-3 |
| 59 | **S20/S21** the same, `--format json`, from the main checkout | `jigc --format json milestone finalize zeta-probe` | 3 | same | Human | `key.target` = `.jigc/worktrees/zed-two`; route `git -C … bisect reset` | matches contract | R-3 |
| 60 | **S20** the same, run from **inside** a sibling worktree | same (cwd = `.jigc/worktrees/zed-one`) | 3 | same | Human | **0 host-path hits** across message · `at:` · `location.address` · `key.target` · route | post-review MEDIUM 1 — CLOSED | R-3 |
| 61 | worktree narration incl. the `--ignored` axis | finalize with `.gitignore`, an ignored `build/`, an unstaged file | 0 | none | Informational | `build/ (ignored by git)`, `scratch.txt (never staged)` on both surfaces | matches contract | **DEMOTED → re-driven X-10 · RESTORED** |
| 62 | the same narration over a **deleted tracked file** | finalize after `rm keeper.md` in the worktree | 0 | none | Informational | `keeper.md (never staged)` + *"the only copy … not recoverable"* — driven false | **DEFECT F-4** | R-8 · **re-driven X-3** |
| 63 | nothing to land · unregistered strays · zero-false-fire | `milestone finalize` with nothing staged / with strays / with no plant | 3 / 0 / 0 | `milestone.zero-contribution` / none / none | Mechanical / none / none | zero-contribution names `provision` + `discard`; strays survive, named on neither stream | matches contract | **DEMOTED → re-driven X-10 · RESTORED** |

### 1.7 · the residual rule's other doors — 9 doors

| # | door | cell | argv | exit | code | surface asserted | verdict | repro |
|---|---|---|---|---|---|---|---|---|
| 64 | `task list` | **S18** residual present | `jigc task list` | 0 | none | `no active tasks` — the residual is **not** listed | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 65 | `task list` | **S18′** settled sub-task's leftover, pin intact | `jigc task list` | 0 | none | `1 active task(s)` / `second-sub [sub-task]` while `task discard` answers `milestone.terminal` | **DEFECT F-5** (second half) | R-9 · **re-driven X-4** |
| 66 | `task validate` | the same | `jigc task validate second-sub` | 0 | none | `no findings — the task validates clean` | n/a — declared (`Tier::LaterCause`) | **DEMOTED → re-driven X-4 · RESTORED** |
| 67 | `task diff` | the same | `jigc task diff second-sub` | 0 | none | renders a diff against the settled base | matches contract | **DEMOTED → re-driven X-10 · RESTORED** |
| 68 | `start` | **S18** residual present | `jigc start` | 0 | none | orientation renders clean; no `also open:` row | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 69 | `start` (mint) | **S18** task residual, same slug | `jigc start --workflow quick-fix "ghost residual"` | 1 | `task.serial-collision` | residual sentence + Human route; nothing minted | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 70 | `milestone create` | a settled milestone's id | `jigc milestone create "Axis three probe"` | 1 | `milestone.record-exists` | `its record reads \`joined\``; route names a different title | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 71 | `rename` | over a task residual **and** a milestone-area residual | `jigc rename research:context-loss --to renamed-probe` | 0 | none | rename **proceeds**; `repointed 0 referrer(s)`; both residuals still on disk | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 72 | `rename` | **control** — a genuinely live task | same, with one open task | 1 | `rename.in-flight` | names `jigc task finalize …` / `jigc task discard …` | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 73 | `doc list` | a symlink wearing a staged identity | `jigc doc list --task tidy-the-readme` | 0 | none | `adr:via-symlink … managed` listed; `adr:via-directory` is **not** | **F-1 STILL-OPEN** | R-16 · **re-driven X-1** |
| 74 | `milestone list-tasks` | a pin-less **milestone** area while the record is live | `jigc milestone list-tasks axis-three-probe` | 0 | none | the sub-task roster still renders; nothing bricks | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |
| 75 | `milestone execute` | the same | `jigc milestone execute axis-three-probe` | 0 | none | composes the provision step text; nothing bricks | matches contract | **DEMOTED → re-driven X-8 · RESTORED** |

---

## 2 · Reconciliation ledger

### 2.1 · Codex claims — each entered as a lead, then driven

| # | `lead(codex, …)` | disposition | evidence |
|---|---|---|---|
| **C-1** | **F-1 remains open**: a symlink with a staged-document filename is simultaneously classified as staged prose and as foreign bytes (`staged_doc_ids` follows with `fs::metadata`, `foreign_area_paths` does not with `DirEntry::file_type`) | **CONFIRMED (origin codex — and independently the driver's F-1)** | §3 **X-1**. Driven exactly as Codex proposed: `task discard` names it under `task-discard.foreign-bytes`; `doc list --task` calls the same path `adr:via-symlink … managed`; `--force` drops it *"staged edits to: adr:via-symlink"*. The external target is 1/1 intact. |
| **C-2** | **F-2 remains open**: `residual_area_note` asserts *"is a directory"* although `carries_base_pin` tests only `<area>/<member-0>` with `symlink_metadata` | **CONFIRMED (origin codex — and independently the driver's F-2)** | §3 **X-1**. `.jigc/tasks/ghost` a symlink → `finalize.no-task — … \`.jigc/tasks/ghost\` **is a directory** carrying no base pin …`. Outside target intact; the control over a real directory makes the sentence true there. |
| **C-3a** | *"I found **no removal bypass that can cause exit-0 loss or repository harm** through a destroying door"* | **REFUTED (repository-harm half)** | §3 **X-2**. `jigc uninstall` run from inside a fan-out worktree exits **0**, prints *"repo-local install removed / - removed .jigc/ / - removed pre-commit hook"*, and removes `.git/hooks/pre-commit` — **one file shared by every checkout** — while the main checkout's `.jigc/`, `CLAUDE.md` and `SKILL.md` all survive and `jigc start` still orients. `uninstall` is a `DESTROYING_DOORS` member. **Bound on the refutation:** the driver graded this **tier 2**, not tier 1, and my drive agrees on the datum — the five `.jigc/` files deleted are tracked (the door says so), the hook is regenerable by `jigc setup`, and the untracked-byte cell is guarded (row 36, re-driven). So the *exit-0 loss of adopter bytes* half of C-3a stands; the *repository harm* half does not. |
| **C-3b** | *"No new completeness defect or **tier-1 lead** found"* | **CONFIRMED (not contradicted by any drive)** | Thirty reconciliation drives over six destroying doors reached **no** exit-0 loss of bytes nothing else has a copy of: X-5 (8 before / 8 after), X-6 (3 before / 3 after, both areas left standing), X-7 (nothing removed, `BISECT_LOG` on disk), X-4 (HEAD byte-unmoved), X-10 (strays 2 before / 2 after). Negative existential — confirmed only to the extent driven. |
| **C-4** | **A3-1 CLOSED**: the `merged/` walk recognizes only regular staged bodies as jigc-written; the removal twin is non-recursive and preserves foreign entries | **CONFIRMED** | §3 **X-5**. Eight planted loci (area root · nested area dir · `merged/top.txt` · `merged/docs/{deep.txt, provenance.json, adr.md}` · `merged/sub/` · a sub-task area) → exit 0, **8 before / 8 after**, all eight pairs on `committed.displaced`, each foreign directory moved **whole**, both area roots empty. |
| **C-5** | **A3-2 CLOSED**: finalization uses registry-bounded `unwind_area`; a non-empty area becomes `AreaUnwind::Foreign` and is not recursively removed; both `Foreign` and unwind errors emit `finalize.foreign-bytes` and leave the area standing | **CONFIRMED** | §3 **X-6**. `.jigc/displaced` a regular file → exit 0, **3 before / 3 after**, both areas left standing, one advisory **per standing area** keyed `milestone:axis-three-probe` / `task:first-sub`, each naming the fault `Not a directory (os error 20)`; envelope top-level keys `['committed']`, `displaced: []`, **0 host-path hits**. |
| **C-6** | **A3-3 CLOSED**: production supplies both task and milestone `WorkArea` subjects to one teardown structure; `MILESTONE_AREA_FILES` + the `merged/` tree rule represent milestone membership | **CONFIRMED behaviourally; the structural half is a source read I did not re-derive** | X-5 and X-6 drive the milestone area's *own* complement at the boundary and at the move-failure cell. The claim that the two subjects share one structure is a code fact; I neither reproduced nor contradicted it. Same disposition the driver recorded. |
| **C-7** | **Post-review operation-bearing-worktree HIGH CLOSED**: `probe_leftover` admits clearance only when status entries **and** operation are both empty; `held_operation` delegates to `adjudicated_breach`; provision/discard/uninstall share the classifier | **CONFIRMED** | §3 **X-7**. A **spotless** worktree (`git status --porcelain` → 0 lines) carrying a live bisect: `milestone discard` → exit 1 `milestone.dirty-worktree`, `uninstall` → exit 1 `uninstall.dirty-worktree`, both naming the operation and its abandon command; worktree dir, admin record and `BISECT_LOG` all present after. Zero-false-fire control (clean **and** concluded) → exit 0, `workbench removed`. Provision's arm is the driver's R-2, re-read not re-driven. |
| **C-8** | **M51 rows CLOSED**: C-1 (six-door registry) · D-1 (milestone ownership from live base-pinned areas) · D-2 (teardown tracks actual removal success) · D-3 (destroying-door paths render relative to `jigc_home`) · D-4 (unreadable/non-directory leftovers are distinct fail-closed shapes) | **CONFIRMED where driven; C-1/D-1/D-2 carried from the driver** | D-3 driven by me: `command grep -cE '/var/folders|/private/var'` over stdout+stderr → **0** at X-6 (and the driver's rows 47/57/60). D-4's shape leg is the driver's row 5 (`Permission denied (os error 13)`), re-read not re-driven. C-1, D-1, D-2 are the driver's rows 39/43 and 21; I did not re-drive them and neither instrument contradicts the other. |
| **C-9** | *"I inspected **every** production `remove_dir_all` / `remove_file` / `remove_dir` … I found **no unguarded** production removal of adopter bytes"* | **OPEN LEAD — not drivable as stated** | A completeness claim over a source enumeration has no argv. Nothing I drove produced a counter-instance (the nearest, X-2, removes jigc's **own** install artefacts, not adopter bytes). Recorded open rather than promoted on the source read, per the rule. |
| **C-10** | *"No schema, schema-manifest, or schema-hash movement appears in the post-M53 source range. The zero-schema-hash boundary holds."* | **CONFIRMED (verified against the repository, not the binary)** | `git diff 3c71da87..HEAD --stat` over `crates/cli/pack/config/schema-manifest.yaml`, `packs/methodology/config/schema-manifest.yaml`, `crates/cli/pack/schemas`, `packs/methodology/schemas` → **empty**; `git diff --name-only 3c71da87..HEAD | grep -E 'schema-manifest|/schemas/'` → **0**. The six `schema-hash` occurrences in the range are prose (`CLAUDE.md` ×3, the two axis prompts, `decisions-pending.md`). |

### 2.2 · Driver defects — each re-driven by me

| finding | tier | status after reconciliation | what the source pass says | evidence |
|---|---|---|---|---|
| **F-3** — `jigc uninstall` inside a fan-out worktree exits 0, reports an install it did not remove, and takes the repository's shared `pre-commit` hook | 2 | **CONFIRMED — stands** | **Contradicted** by C-3a's blanket negative; that clause is **REFUTED** on this repro (the pass reached it by a source read and never drove the cwd axis) | §3 **X-2**, driven from scratch: main-checkout control → exit **1** `uninstall.dirty-worktree`; the identical command from inside the worktree → exit **0**, hook **REMOVED**, main `.jigc/`/`CLAUDE.md`/`SKILL.md` **STILL PRESENT**, `jigc start` still orients, `milestone list-tasks` still names the sub-task. The tier-1-excluding control (row 36) re-driven: exit 1, both plants intact. |
| **F-4** — the fan-out teardown narrates a **deleted tracked file** as *"never staged"* and *"the only copy … not recoverable"* | 3 | **CONFIRMED — stands** | **Silent** (the pass's subject is removal guards, not narration truth) | §3 **X-3**: `git -C $W status --porcelain` → ` D keeper.md`; both surfaces say `keeper.md (never staged)` under *"not committed, not recoverable"*; after the run `git show HEAD:keeper.md` → `TRACKED-BYTES-AT-HEAD` and the main checkout's copy is untouched. |
| **F-5** — a settled sub-task's leftover is *"1 active task(s)"* at `task list` and *"a leftover, not live work"* at `task discard`, and the finalize door's route names a command that refuses | 3 | **CONFIRMED — stands, both halves** | **Silent** | §3 **X-4**: `task list` → `1 active task(s)`; `task discard second-sub` → exit 1 `milestone.terminal`; `--force` → **byte-identical** refusal (`cmp -s` → YES); `task finalize` → exit 3 `finalize.empty-commit` routing at the `--force` that refuses, then after following the route verbatim exit 3 `finalize.render-io` routing at the same refusing command. HEAD byte-unmoved, record still `joined`. The reachability bound stands: I too built the state by restoring a `cp -R` backup. |
| **F-1** — a symlink wearing a staged identity is jigc's own at the read/ack surfaces and a third party's at the destroying probe | 3 | **CONFIRMED — stands (both instruments)** | **Agrees** (C-1) | §3 **X-1** |
| **F-2** — the residual note asserts a shape it did not check | 3 | **CONFIRMED — stands (both instruments)** | **Agrees** (C-2) | §3 **X-1** |
| **OBS-1 … OBS-9** (observations, not findings) | — | **carried unchanged, not re-driven** except OBS-8, whose host path appears verbatim in my X-4 (`at: /private/var/folders/…/docs/commit:second-sub.md`) | the pass does not address them | driver §5.1 |

### 2.3 · The demotion audit — rows marked driven that carried no repro block

`driver/axis3.md` §2 states *"Every row's repro block is in §3 unless the row says otherwise."* Reading §3's
sixteen blocks against the 75 rows, **22 rows are evidenced by no block**: 12, 13, 17, 19, 22, 27, 34, 44,
46, 51, 61, 63, 64, 66, 67, 68, 69, 70, 71, 72, 74, 75. By the rule they are **demoted — not driven**.

I re-drove **all 22** (X-8, X-9, X-10, plus row 66 inside X-4). **Every one matched the driver's stated
surface**; none was refuted, so all 22 are **restored** as CONFIRMED-by-reconciler. Two cell variances are on
the record rather than smoothed:

- **row 17** — the driver's cell is `Rebase` + `--force` on a **spotless** tree; mine was `Bisect` + tree
  dirt + `--force`. The row's claim (Informational; the operation narrated verbatim; `not recoverable`;
  then `workbench removed`) is confirmed, the *rebase-on-a-spotless-tree* cell is not.
- **row 61** — the driver lists three narrated items including `.gitignore (never staged)`; in my rig the
  ignore rule was committed **before** the milestone pin, so the two load-bearing items appear
  (`build/ (ignored by git)` · `scratch.txt (never staged)`) and `.gitignore` does not. First attempt was
  an **instrument fault of mine** (ignore rule committed *after* the pin → `finalize.base-mismatch`, exit
  3), re-driven correctly and recorded.

Consequence for coverage: **none**. Every one of the 15 doors keeps ≥1 row carrying a repro block, so
`doors_covered` is unchanged at 15 — but five doors (`task validate`, `task diff`, `start`,
`milestone create`, `rename`, `milestone execute`) would have dropped out had the demotions not been
re-driven, since the driver's only rows for them were the un-evidenced ones.

---

## 3 · Reconciliation repro blocks

All ten ran on `jigc 1.0.0-rc.18`, each in its own `dev/jigc-rig` throwaway repo.

### X-1 — **Codex claims C-1 and C-2, both CONFIRMED** (rows 38, 42, 73)

```
setup: dev/jigc-rig fresh; jigc start --workflow quick-fix "tidy the readme"
       OUT=$(mktemp -d …); TARGET-BYTES > $OUT/body.md
       ln -s $OUT/body.md .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
BEFORE-CONTROL: lrwxr-xr-x … adr:via-symlink.md -> /var/folders/…/body.md ; target hits 1

$ jigc task discard tidy-the-readme                                     -> exit 1
  blocking · task-discard.foreign-bytes — task `tidy-the-readme`'s working area holds 1 path(s) jigc
    did not write …:  .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md        <- FOREIGN here
$ jigc doc list --task tidy-the-readme                                  -> exit 0
  adr:via-symlink  docs/decisions/via-symlink.md  managed                        <- JIGC'S OWN here
$ jigc task discard tidy-the-readme --force                             -> exit 0
  warning: removing the working area … discards work that is not in git:
      .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
  discarded task tidy-the-readme — dropped staged edits to: adr:via-symlink, commit:… (transient)
AFTER: outside target TARGET-BYTES (1/1 intact); task area GONE

### C-2 — the residual note's shape claim over a SYMLINK
setup: OUT2=$(mktemp -d …); OUTSIDE2-KEEP > $OUT2/keep.txt; ln -s $OUT2 .jigc/tasks/ghost
  ls -l -> lrwxr-xr-x  ghost -> /var/folders/…
$ jigc task discard ghost                                               -> exit 1
  blocking · finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` IS A DIRECTORY carrying no base
    pin, so it is a leftover and not a work unit …
AFTER: $OUT2/keep.txt -> OUTSIDE2-KEEP (intact)
CONTROL (a real directory, row 42's cell): `.jigc/tasks/ghost-dir` -> exit 1, finalize.no-task,
  same sentence, at: task:ghost-dir, route repo-relative — true there
```

### X-2 — **F-3 CONFIRMED, and C-3a REFUTED on its repository-harm half** (rows 35, 36)

```
setup: dev/jigc-rig fresh; milestone create/add-task/provision; SUBWORK > $W/f.txt; git -C $W add f.txt
       WORKBENCH-KEEP > .jigc/milestones/axis-three-probe/plant.txt
BEFORE-CONTROL: main .jigc PRESENT · .git/hooks/pre-commit PRESENT · CLAUDE.md PRESENT ·
                SKILL.md PRESENT · workbench plant 1

$ (cd $REPO)               jigc uninstall                               -> exit 1
  blocking · uninstall.dirty-worktree — `.jigc/` holds 1 fan-out sub-task worktree path(s) …
    .jigc/worktrees/first-sub: A  f.txt — it is a live git worktree holding uncommitted work …

$ (cd $REPO/.jigc/worktrees/first-sub)  jigc uninstall                  -> exit 0
  warning: removing `.jigc/` also removes 5 tracked file(s) under it: … (note: each is in the index)
  jigc uninstall — repo-local install removed
    - removed .jigc/            - unwired bootstrap reference ← CLAUDE.md
    - removed jigc allowlist permit / SessionStart hook / deny safety floor ← .claude/settings.json
    - removed pre-commit hook   - removed jigc guide artifact

AFTER (measured in the MAIN checkout):
  main .jigc            STILL PRESENT      CLAUDE.md STILL PRESENT     SKILL.md STILL PRESENT
  .git/hooks/pre-commit REMOVED            <- shared by every checkout of the repository
  worktree .jigc        REMOVED            workbench plant 1 (survives)
  jigc start            -> "jigc — orientation"
  milestone list-tasks  -> "milestone:axis-three-probe tasks (1): first-sub"

CONTROL that keeps it off tier 1 (row 36): an untracked byte under the WORKTREE's own .jigc/
  .jigc/tasks/hand-made/notes.txt -> jigc uninstall (cwd = worktree) -> exit 1
    blocking · uninstall.foreign-bytes — … .jigc/tasks/hand-made/notes.txt
    plant after 1 · main hook PRESENT
```

### X-3 — **F-4 CONFIRMED** (row 62)

```
setup: dev/jigc-rig fresh; TRACKED-BYTES-AT-HEAD > keeper.md; git add + commit
       milestone create/add-task/provision; SUBWORK > $W/subwork.txt; git -C $W add subwork.txt
       rm $W/keeper.md            # delete a TRACKED file; do not stage the deletion
BEFORE-CONTROL: git -C $W status --porcelain -> " D keeper.md" / "A  subwork.txt" / ?? …
                git show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD
                cat $REPO/keeper.md     -> TRACKED-BYTES-AT-HEAD

$ jigc milestone join … && jigc milestone finalize axis-three-probe     -> exit 0
  stdout  discarded with the fan-out worktrees (not committed, not recoverable):
            first-sub: build/out.o (never staged) · ignored (never staged) ·
                       keeper.md (never staged) · scratch.txt (never staged)
  stderr  warning: removing the fan-out worktree … discards work that is not in git:
            … keeper.md (never staged) …
          note: the fan-out worktree is the only copy of these bytes — they are not recoverable.

AFTER — is the claim true?
  git show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD        # the bytes ARE in git
  cat $REPO/keeper.md     -> TRACKED-BYTES-AT-HEAD        # and in the main checkout
```

### X-4 — **F-5 CONFIRMED, both halves** (rows 43, 45, 52, 53, 65, 66)

```
setup: dev/jigc-rig fresh; milestone create + two sub-tasks; provision; both stage code;
       SAVE=$(mktemp -d …); cp -R .jigc/tasks/second-sub $SAVE/     # faithful backup, pin and all
       jigc milestone join && jigc milestone finalize axis-three-probe
       cp -R $SAVE/second-sub .jigc/tasks/second-sub                # restore, as a backup would
BEFORE: docs/milestone-records/axis-three-probe.md -> `status: joined`; HEAD 8ca9aff…

$ jigc task list                       -> exit 0  "1 active task(s)" / "second-sub [sub-task]"
$ jigc task discard second-sub         -> exit 1  milestone.terminal — … already `joined` …
$ jigc task discard second-sub --force -> exit 1  cmp -s of the two outputs -> BYTE-IDENTICAL
$ jigc task finalize second-sub        -> exit 3  finalize.empty-commit
    route: … abandon it with `jigc task discard second-sub --force`        <- refuses, above
… following that route verbatim (git add late.txt; re-run):
$ jigc task finalize second-sub        -> exit 3  finalize.render-io
    at: /private/var/folders/…/repo/.jigc/tasks/second-sub/docs/commit:second-sub.md   <- OBS-8
    route: … abandoned with `jigc task discard second-sub --force` …       <- refuses, above
$ jigc task validate second-sub        -> exit 0  "no findings — the task validates clean"  (row 66)
AFTER: HEAD 8ca9aff… (unmoved YES) · record still `status: joined` · nothing committed
```

### X-5 — **C-4 / A3-1 CONFIRMED** (rows 54–56)

```
setup: fresh; two sub-tasks; provision; both stage code; join
       8 plants: merged/{top.txt, docs/deep.txt, docs/provenance.json, docs/adr.md, sub/nested.txt},
                 area-root {sub/inner.txt, mnotes.txt}, .jigc/tasks/first-sub/tnotes.txt
BEFORE-CONTROL: command grep -rl 'KEEP-m-' .jigc | wc -l  -> 8

$ jigc --format json milestone finalize axis-three-probe                -> exit 0
  top-level keys ['committed'];  committed.displaced = 8 pairs, sorted by from:
    merged/docs/adr.md · merged/docs/deep.txt · merged/docs/provenance.json ·
    merged/sub (directory, WHOLE) · merged/top.txt · mnotes.txt · sub (WHOLE) ·
    .jigc/tasks/first-sub/tnotes.txt  -> each at .jigc/displaced/<unit>/<relative>
AFTER-CONTROL: 8          area roots: milestones=[] tasks=[]
```

### X-6 — **C-5 / A3-2 CONFIRMED at `milestone finalize`** (row 57)

```
setup: fresh; two sub-tasks; provision; both stage code; join
       KEEP-A32-merged > merged/docs/deep.txt · KEEP-A32-root > mnotes.txt ·
       KEEP-A32-sub > .jigc/tasks/first-sub/tnotes.txt ·  'NOT A DIR' > .jigc/displaced
BEFORE: 3

$ jigc --format json milestone finalize axis-three-probe                -> exit 0
  top-level keys ['committed']            committed.displaced = []        # the pinned envelope
  stderr: advisory · finalize.foreign-bytes — `.jigc/milestones/axis-three-probe` holds 2 path(s) …
            left standing rather than removed …; 2 of them could not be moved aside: …
            Not a directory (os error 20)          at: milestone:axis-three-probe
          advisory · finalize.foreign-bytes — `.jigc/tasks/first-sub` holds 1 path(s) …
                                                   at: task:first-sub
AFTER: 3    both areas LEFT STANDING     host-path hits over stdout+stderr: 0
```

### X-7 — **C-7 / the post-review HIGH CONFIRMED on a SPOTLESS worktree** (rows 14, 26, 18)

```
setup: fresh; milestone create + one sub-task; provision;
       git -C $W bisect start / bad / good HEAD~1     (NO staged code — the tree stays spotless)
BEFORE-CONTROL: git -C $W status --porcelain | wc -l -> 0   <- SPOTLESS
                .git/worktrees/first-sub BISECT_LOG -> 1    main-repo op markers -> 0

$ jigc milestone discard axis-three-probe                               -> exit 1
  blocking · milestone.dirty-worktree — … 1 sub-task worktree path(s) hold something the abandon
    cannot prove is disposable …
    .jigc/worktrees/first-sub: a bisect git has left un-concluded (abandon it with
      `git -C .jigc/worktrees/first-sub bisect reset`) — it is a live git worktree git has left
      mid-operation …; registered here, so the teardown removes it and this content is destroyed
    route: … A path listed above as mid-operation holds no bytes to move: conclude or abandon …
$ jigc uninstall                                                        -> exit 1
  blocking · uninstall.dirty-worktree — the identical hold line, `registered as a worktree of this
    repository`; the same operation clause on the route tail
AFTER: worktree dir PRESENT · admin record PRESENT · BISECT_LOG 1

ZERO-FALSE-FIRE control (row 18): git -C $W bisect reset; jigc milestone discard … -> exit 0
  "discarded milestone:axis-three-probe (1 sub-task(s); workbench removed)"
```

### X-8 — demoted rows restored: the residual-state and record-state doors (22, 64, 68–72, 74, 75)

```
### rows 64, 68, 69 — a bare-mkdir task residual
  jigc task list                                  -> 0  "no active tasks"          (row 64)
  jigc start                                      -> 0  "jigc — orientation", no `also open:` (68)
  jigc start --workflow quick-fix "ghost residual"-> 1  task.serial-collision — `.jigc/tasks/
      ghost-residual` is a directory carrying no base pin …; nothing minted, residual on disk (69)
### rows 71, 72 — rename (rig: refs-post-hoc, whose $RIG_TASK is live)
  with the live task open:  jigc rename research:context-loss --to renamed-probe
                                                  -> 1  rename.in-flight — … finalize or discard it
                                                        first; route names both verbs        (row 72)
  after discarding it, with a task residual AND a milestone-area residual planted:
                                                  -> 0  "renamed research:context-loss ->
                                                        research:renamed-probe …, repointed 0
                                                        referrer(s)"; both residuals PRESENT (row 71)
### row 70, row 22 — a settled milestone (joined record on disk)
  jigc milestone create "Axis three probe"        -> 1  milestone.record-exists — "(its record reads
                                                        `joined`)"; route names a different title
  jigc milestone discard axis-three-probe         -> 1  milestone.terminal — "is `joined` — a settled
                                                        milestone is over and has no workbench"
### rows 74, 75 — a PIN-LESS milestone area while the record is live (base.json removed)
  jigc milestone list-tasks axis-three-probe      -> 0  "milestone:axis-three-probe tasks (1): first-sub"
  jigc milestone execute axis-three-probe         -> 0  composes the provision step text; nothing bricks
```

### X-9 — demoted rows restored: `provision` / `discard` / `uninstall` (12, 13, 17, 19, 27)

```
### row 12 — a REGISTERED worktree carrying an operation, idempotent re-provision
  BEFORE: registered here 1 · tree dirt 0 · BISECT_LOG 1
  jigc milestone provision axis-three-probe -> 0 "provisioned 2 worktree(s) … at base fc34620"
  AFTER:  BISECT_LOG 1 · worktree dir PRESENT            <- neither probed nor cleared
### row 19 — dirt AND an operation composed on one hold line
  jigc milestone discard axis-three-probe -> 1 milestone.dirty-worktree, 2 paths:
    first-sub:  "?? dirt.txt; and a bisect git has left un-concluded (abandon it with …)"
    second-sub: "?? dirt2.txt — it is a live git worktree holding uncommitted work …"
### row 17 — the same state, --force              (cell variance: Bisect+dirt, not Rebase-spotless)
  jigc milestone discard axis-three-probe --force -> 0
    two warning blocks, one per worktree; first-sub's names "dirt.txt (never staged)" AND
    "a bisect git had left un-concluded — it can only be concluded or abandoned from this checkout";
    both + "not recoverable"; then "discarded … (2 sub-task(s); workbench removed)"; worktrees []
### row 27 — uninstall --force over a spotless operation-bearing worktree
  BEFORE: dirt 0 · admin record PRESENT
  jigc uninstall --force -> 0  the operation narrated verbatim before the removal
  AFTER:  worktree dir GONE · admin record SURVIVES      <- the weaker arm, as the driver states
### row 13 — malformed / empty id at provision
  jigc milestone provision ""      -> 1 work-unit.malformed-id — "" is not a valid work-unit id
  jigc milestone provision "../.." -> 1 work-unit.malformed-id
  .jigc after: [AGENT.md config milestones state tasks version]
```

### X-10 — demoted rows restored: the two `Displace` doors and the leftover-precedence cells (34, 44, 46, 51, 61, 63, 67)

```
### row 46 — task finalize, S17 ALL MOVE          BEFORE 1
  jigc --format json task finalize tidy-the-readme -> 0
    displaced = [{from .jigc/tasks/tidy-the-readme/notes.txt, to .jigc/displaced/tidy-the-readme/
                  notes.txt}]   findings = []
    stderr: "… held 1 entry jigc did not write … moved aside, not taken:" + the `→` pair
  AFTER 1 · area GONE
### row 51 — control, ordinary lifecycle, no plant
  -> 0   displaced []   findings []   stderr 'moved aside|foreign-bytes' matches 0
         .jigc/displaced ABSENT   .jigc/tasks []
### row 34 — uninstall --force over a symlink inside a task area
  -> 0   "warning: removing the working area … .jigc/tasks/tidy-the-readme/link" + not recoverable
  AFTER: outside tree OUTSIDE34-KEEP (1/1 intact) · .jigc REMOVED
### row 67 — task diff over a settled sub-task's leftover (X-4's state)
  jigc task diff second-sub -> 0  "# code changes vs base e4d97d9" + the record's diff
### row 44 — the same state WITH a foreign byte (precedence over the terminal arm)
  jigc task discard second-sub -> 1  task-discard.foreign-bytes — … .jigc/tasks/second-sub/
    foreign.txt      (the foreign arm wins over milestone.terminal)   plant after 1
### row 61 — the teardown narration's --ignored axis   (.gitignore committed BEFORE the pin)
  BEFORE: status [A first-sub.txt; ?? scratch.txt]   status --ignored adds [!! build/]
  jigc milestone finalize -> 0
    stdout  first-sub: build/ (ignored by git) · scratch.txt (never staged)
    stderr  the same two + "note: the fan-out worktree is the only copy of these bytes …"
  (first attempt was MY instrument fault: .gitignore committed AFTER the pin -> exit 3
   finalize.base-mismatch; re-driven correctly, recorded rather than dropped)
### row 63a — nothing to land
  jigc milestone finalize -> 3  milestone.zero-contribution — … the boundary would commit only jigc's
    own bookkeeping and flip the record to the terminal `joined` …; route names provision + discard
### row 63b — unregistered strays under .jigc/worktrees/
  BEFORE strays 2 (a stray dir + a stray file)   jigc milestone finalize -> 0
  AFTER strays 2 · named on stdout 0 · named on stderr 0
```

---

## 4 · Doors covered

Every clap leaf that is the door of ≥1 **driven** row (post-demotion, post-restoration), in `VERB_KINDS`
spelling. **Fifteen** leaves; the six `DESTROYING_DOORS` members starred.

| door | rows | evidenced by |
|---|---|---|
| `milestone provision` ★ | 1–13 | R-2 · R-10 · **X-9** |
| `milestone discard` ★ | 14–25 | R-1 · R-5 · R-11 · **X-7 · X-8 · X-9** |
| `uninstall` ★ | 26–36 | R-1 · R-5 · R-7 · R-13 · **X-2 · X-7 · X-9 · X-10** |
| `task discard` ★ | 37–45 | R-9 · R-12 · R-16 · **X-1 · X-4 · X-10** |
| `task finalize` ★ | 46–53 | R-6 · R-9 · R-14 · **X-4 · X-10** |
| `milestone finalize` ★ | 54–63 | R-3 · R-4 · R-6 · R-8 · R-15 · **X-3 · X-5 · X-6 · X-10** |
| `task list` | 64, 65 | R-9 · **X-4 · X-8** |
| `task validate` | 66 | R-3 · **X-4** |
| `task diff` | 67 | **X-10** |
| `start` | 68, 69 | **X-8** |
| `milestone create` | 70 | **X-8** |
| `rename` | 71, 72 | **X-8** |
| `doc list` | 73 | R-16 · **X-1** |
| `milestone list-tasks` | 74 | R-7 · **X-2 · X-8** |
| `milestone execute` | 75 | **X-8** |

**Not doors of any reviewed row** (rig setup only, recorded so the list is not read as coverage):
`milestone add-task`, `milestone join`, `workflow`, `doc set-field`, `doc set-slot`, `config set`,
`describe`, `doc show`, `ingest`, `setup`, `migrate`, `migrate-corpus`, `validate`, `task bind`,
`doc create`, `doc author`, `unmanage`, `relocate`, `config get`, `config list`, `doc schema`,
`doc add-item`, `doc rename`, `doc retitle-item`, `milestone add-from-spec`.

---

## 5 · Notes and bounds carried out of the reconciliation

1. **Tier-1 rows on this axis after reconciliation: 0.** Both instruments reach that independently — the
   driver by 75 rows, the source pass by its removal-site enumeration, and this reconciliation by ten
   fresh drives with before-controls. The exit rule's question is answered *for this axis only*.
2. **The one genuine disagreement is C-3a**, and it is narrow: the source pass asserts no destroying-door
   removal bypass can cause exit-0 **loss or repository harm**; F-3's repro (X-2) removes the repository's
   shared `pre-commit` hook at exit 0 and prints three false clauses about what it removed. The *loss of
   adopter bytes* half of the claim survives — the deleted `.jigc/` files are tracked and the door says so,
   and the untracked-byte cell is guarded. So this is a **tier-2** refutation of a blanket negative, not a
   tier-1 row.
3. **C-9 is the only OPEN lead** and it is open by construction: *"every production removal site is
   guarded"* is a completeness claim over source with no argv. It cannot be promoted on a source read and
   it was not; nothing driven produced a counter-instance.
4. **The demotion audit is the instrument finding of this reconciliation.** 22 of 75 rows (29 %) carried no
   repro block while §2 of the driver's file asserts every row has one. All 22 re-drove true, so the table's
   *claims* were sound — but as delivered, six of its fifteen doors rested on rows with no evidence
   attached, and the doors-covered list would not have survived the rule.
5. **What I did not re-drive**, stated as un-driven rather than passing: the driver's R-2 (`provision` over a
   second repository's linked worktree), R-3 (the render-root / declared-target-form cells at
   `milestone finalize` + `task validate` from inside a sibling worktree), R-10's nine `LeftoverShape` ×
   `--force` cells, R-11, R-12, R-13, R-14, R-15 and the M51 rows C-1 / D-1 / D-2 / D-4. Each is carried on
   the driver's own repro block; none is contradicted by the source pass.
6. **The driver's declared bounds are carried unchanged** (its §4 and §9): three of ten `InProgress`
   members at the refusing doors; F-5's state built by restoring a backup rather than by a jigc sequence —
   which my X-4 reproduces the same way and therefore does not discharge; F-3's `--force` arm un-driven; no
   genuine concurrent racer; `Disposition::Narrate` empty by construction at rc.18.
7. **Instrument honesty.** One of my drives failed on my own error and is recorded rather than dropped
   (row 61's first attempt: `.gitignore` committed *after* the milestone pin → `finalize.base-mismatch`,
   exit 3). One drive narrowed a driver cell rather than reproducing it exactly (row 17). Exit codes were
   read bare in every verdict; `command grep` with a before-control carries every loss/survival claim,
   because this harness's `grep` honours `.gitignore` and the whole axis lives under a gitignored `.jigc/`.
8. **Scope.** This reconciles **axis 3 only**, on `1.0.0-rc.18` at repo HEAD `271b0cb7`. No fix was applied,
   no commit made, nothing written into the working repository.
