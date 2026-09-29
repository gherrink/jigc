# M53 — the THIRD PARTIAL per-axis review (axes 2 · 3 · 5 · 6), on `1.0.0-rc.19`

**What this is.** M53's acceptance, re-run a third time — after the **cwd-dependence arc**.
[decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.17 fix pass (M53)* carries
the gate this record is written against:

> **The exit rule — the human's own gate.** *The 1.0.0 call is taken when a partial re-review of the fix
> pass's affected axes finds no tier-1 row. Tier-2 and tier-3 findings never block the call and are
> triaged into the ledger for 1.x. A finding inside a fix pass's own new code triggers another fix pass
> and a partial re-review, never a full wave.* *[Completed 2026-09-22, on the first time it fired: a
> tier-1 row found **outside** the fix pass's own new code is fixed as a **post-review fix under the same
> milestone** — fix, an independent review of the diff standing in for the audit, the record, a new
> stamp, the affected axes re-driven.]*

The first partial re-review ran on `1.0.0-rc.17` and found one tier-1 row, `(2, DEFECT 1)`. The
**second** ran on `1.0.0-rc.18` after the post-review fix, found **zero** tier-1 rows — and found
`(2, F-1)`, a tier-2 row **inside that fix's own new code**: an aimed route
`git -C <repo-relative-path> …` that exits 128 from every cwd but the repository root. **The human read
that row and asked the wider question — agents swap directories, so what else does cwd break?** — which
was answered by a **driven census** ([cwd-census.md](../cwd-census.md)), decided as two classes on one
root cause, built by two fixers, reviewed twice, and stamped as `1.0.0-rc.19`
([VERDICT.md](../VERDICT.md) → *Addendum 2*; the four 2026-09-23 [DECISIONS.md](../../../DECISIONS.md)
entries). **This directory is the re-drive of that arc.**

**AXES 2 · 3 · 5 · 6.** The arc's blast radius is wider than rc.18's was, and the axis set was chosen
for it, on the record before the run ([VERDICT.md](../VERDICT.md) → Addendum 2's closing line): **2**
(posture — the arc rewrote every posture route's operand), **3** (destroying doors — `uninstall` and the
worktree doors changed subject), **5** (pinned contracts — `task diff`'s envelope and the
`PATH_ARG_OCCURRENCES` bases moved), **6** (composed surfaces — the `Spawn:` line became absolute).
**Axes 1, 4, 7 and 8 were NOT re-driven, by design** — caller tokens, transaction/rollback,
freeze/migration and adopter docs/help. Their rows stand where they were last driven (M52's run, on
`1.0.0-rc.16`). **Nothing in this file says those four held; it says they were not asked.**

**The binary.** Every row in every file here was driven on the installed release
`/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.19`**, on **2026-09-23**, with the repository at HEAD
**`a8904637`** (*“chore(release): 1.0.0-rc.19 — the third M53 stamp, after the cwd-dependence arc”*).
Each axis file asserts `jigc --version` **first, before anything else ran**, and read its registry counts
**by symbol** at that HEAD. The binary is the **release** build, so the debug-only fences
(`Route::mechanical`'s argv fence, `unaimed_git_span`, `unbased_migrate_span`, the two quoting fences)
**do not exist in it**: a fence violation shows up here as a **bad emitted command**, never as a panic.
That is the posture an adopter's binary is in, and it is the only posture in which this run's central
question — *does the emitted route run?* — can be asked at all. Fixtures were built with
[`dev/jigc-rig`](../../../dev/jigc-rig) (every root from `mktemp -d`, so nothing needed teardown and the
`rm -rf $V/$D` shape appears nowhere in this review) and by driving the binary. **Nothing was fixed,
committed or edited in the working repository by any agent in this review.**

**The baselines, one per axis, each a dated record that is NOT edited by this run:**

| axis | baseline | binary | driven |
|---|---|---|---|
| 2 · posture | [per-axis-review-rc18/axis-2.md](../per-axis-review-rc18/axis-2.md) | `1.0.0-rc.18` (`271b0cb7`) | 2026-09-23 |
| 3 · destroying doors | [per-axis-review-rc18/axis-3.md](../per-axis-review-rc18/axis-3.md) | `1.0.0-rc.18` (`271b0cb7`) | 2026-09-23 |
| 5 · pinned contracts | [per-axis-review/axis-5.md](../per-axis-review/axis-5.md) (the rc.17 run) | `1.0.0-rc.17` (`7e98faf1`) | 2026-09-22 |
| 6 · composed surfaces | [M52/per-axis-review/axis-6.md](../../M52/per-axis-review/axis-6.md) | `1.0.0-rc.16` | 2026-09-21 |

**The four cwds this run adds as a cell axis** — the census's own shape, re-driven at every applicable
row: **(a)** the repository root · **(b)** an ordinary subdirectory (`docs/deep`) · **(c)** a provisioned
**fan-out** worktree (`.jigc/worktrees/<sub>`) · **(d)** an ordinary **linked** worktree outside `.jigc/`.
A fifth, **`/`** — outside the repository entirely — was used wherever the row is about *a route the
operator pastes into a shell of unknown cwd*, and **shell-hostile repository roots** (one containing a
space, one containing an apostrophe and a `#`) were used wherever the row is about bytes that reach a
shell.

**An axis matrix row** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary. **A verb is covered iff it is the door of
≥1 driven row.** A classification-only row — a leaf proven by a ⇔ fence rather than by driving — confers
**no** coverage.

---

## THE HEADLINE — the tier-1 count

> # TIER-1 ROWS FOUND: **0**
>
> Tier 1, quoted from the charter: **exit-0 loss or repository harm through a committing, destroying or
> moving door.** **No driven row on any of the four axes reached it**, and each axis reached that zero
> independently, with a before-control on its own drives:
>
> - **axis 2** — every posture breach refused before anything durable was written, across ten
>   `InProgress::ALL` members × five acting doors + one member × all twelve acting doors, four cwds, two
>   spaced-path rigs and the foreign-worktree cell. HEAD unmoved in every refusing cell.
> - **axis 3** — 95 driven rows over 18 doors: no plant died at exit 0 anywhere but where `--force`
>   consented and the door said so first. `command grep` before-controls on every loss/survival claim.
> - **axis 5** — every arm reporting a landed commit checked against `git rev-parse HEAD`; **0** new
>   defects; the two cells the census flagged as tier-1 candidates (the base-mismatch gate not firing
>   from a worktree; `uninstall`'s half-teardown) are both **closed**.
> - **axis 6** — zero data loss, zero corruption; the one new defect is surface-tier.
>
> **`(2, F-1)` — the rc.18 tier-2 row, and the reason the whole cwd arc exists — is CLOSED**, driven at
> **both** `aim_at` callers, from **five** cwds including outside the repository, and under a spaced
> repository path. The emitted command was **run verbatim** and returned `rc=0` from every one of them.
>
> ```
> setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
>        jigc milestone create "Cwd wave"; jigc milestone add-task cwd-wave "cw area"
>        jigc milestone provision cwd-wave
>        W=$REPO/.jigc/worktrees/cw-area; git -C $W bisect start; git -C $W bisect bad
> argv : jigc milestone finalize cwd-wave     from (a) root · (b) docs/deep · (d) a linked worktree
>   -> exit 3, byte-identical:  blocking · repo.operation-in-progress
>        at: .jigc/worktrees/cw-area                                   <- repo-relative, as pinned
>        route: … abandon it with `git -C /private/.../repo/.jigc/worktrees/cw-area bisect reset`
> THE EMITTED COMMAND, RUN VERBATIM:  (a) rc=0 · (b) rc=0 · (c) rc=0 · (d) rc=0 · / rc=0
>        (on rc.18 every one of these but (a) exited 128, `fatal: cannot change to '.jigc/…'`)
> SPACED ROOT: git -C '/private/.../jigc space.VFl1fk/.../.jigc/worktrees/sp-area' bisect reset
>        run verbatim from (a)(b)(c) and /  -> rc=0, rc=0, rc=0, rc=0
> AND THE BOUNDARY LANDS once the route has been run: finalized c1f98ec, 2 files, main HEAD advanced.
> ```

**Six new findings, none of them tier 1: one filed tier 2/3, five tier 3.** And the thing on this page
the human should read next:

> ### Two of the six sit **INSIDE** the cwd arc's own new code — and they are one printed surface.
>
> **`(2, N-1)` = `(6, A6-R1)`** — *the same defect, found independently by two axes*. The installed
> **pre-commit hook** announces *“an out-of-band managed-doc rename exists in the committed tree”* on
> commits that contain **no rename**, and says the change is *“not staged in this commit”* when it **is**.
> The cause is the arc's own commit `65de53f5`: the hook's extraction grep was widened from
> `grep -o 'git mv …'` to `grep -o 'git -C …'` **because the arc made every operator-facing git span
> `git -C <absolute> …`** — and `git -C ` is now the prefix of *every* route in the report, not just a
> rename revert. `crates/cli/src/setup.rs:620` (`PRECOMMIT_RENAME_BLOCK`), gate at `:634`.
>
> **`(2, N-2)`** — the same surface, a **second, independent** cause: the hook's *blocking* backstop is
> **structurally unreachable for the entire placement-doctype family** (`VISION.md`, `CHANGELOG.md`,
> `docs/roadmap.md`, `docs/decisions-log.md` — every managed singleton a stock corpus has), because
> `reconciliation.rename` emits an `mv` pair only for a **`location`** doctype, and the false sentence
> above is what prints in its place. Filed separately from N-1 **because they have two different fixes**,
> and filing them as one row would have hidden whichever fix was not taken.
>
> **The other four are outside the arc**, each with its datum: `(2, N-3)`'s subject rule is **M49**'s
> `provision_worktrees` reuse · `(2, N-4)`'s baseline sharing is **M46**'s *Concurrent writers* ·
> `(3, F-A)` is the law-1 printed-path class the **rc.18 post-review review's MEDIUM 1** closed at
> `milestone finalize` and left un-swept at `task finalize` · `(3, F-B)`'s narration is the shipped
> `LeftoverHold` consent arm.
>
> **The adjudication is the human's, and this record does not pre-empt it.** The exit rule's first clause
> — *the call is taken when the partial re-review finds no tier-1 row* — is **satisfied on all four
> axes**. Its second — *tier-2 and tier-3 findings never block the call* — covers all six by tier. Its
> third — *a finding inside a fix pass's own new code triggers another fix pass* — does **not** say
> *tier-1 finding*, and N-1/N-2 are findings inside the fix pass's own new code, on the one artifact jigc
> installs into the developer's own commit path. **Read by tier, the call is reachable; read by location,
> the third clause fires.** That is the same tension the rc.18 record recorded, one fix pass on, and it is
> stated here rather than resolved.

---

## The staffing

Per axis, three agents — M51 §19's shape, unchanged through M52 and all three M53 runs. **Twelve agents
over four axes.**

- **One Opus driver** owns the `(door, cell)` table. It drives every row on the installed binary and
  records exit, code, route, surface and a repro block per row. **It may not mark a row driven from a
  source read.** Each driver was told not to read its axis's Codex pass, and each says so in its file.
- **One Codex source pass** ([codex/](codex/), verbatim, so every lead is auditable) owns **completeness
  of the row set** — the question driving cannot answer: *is there a door the registry does not carry, or
  a bypass of the seam this axis is about?* It reads; it drives nothing; it made no writes and no builds.
- **One reconciler** (a separate Opus agent, which did **not** author the driver file it reconciles)
  merges the two: it re-drives every driver defect, enters every Codex claim as a lead and drives it, and
  audits the driver's table for rows marked driven that carry **no repro block** (the **demotion pass**).

## The reconciliation rule

> *A claim by one that the other cannot reproduce is a **lead, not a finding**.*

A Codex claim with no driven repro enters the table as `lead(codex, <claim>)` and is either **driven to a
repro block** — at which point it is a finding — or **recorded REFUTED with its falsifying datum**. An
Opus row the source pass says cannot happen **stays a finding** (it was driven), and the source claim is
recorded refuted with the datum. **Silence is not refutation.** A row a reconciler cannot drive at all
stays an **OPEN LEAD** with the reason.

It earned its keep four times this run, in four different directions:

- **Axis 5 — a row the driver did not drive, driven, and wider than either input.** `(5, C1)` was filed
  STILL-OPEN on an unchanged producer plus a static read; the driver's own §2.5 and §9.8 say so. Under
  the rule that is not a driven row, so it was **demoted** — and then re-manufactured and driven at
  **both** declared arms, where it holds and is **wider than stated**: `ActiveTask` omits `next_steps`
  too, not only `Clean` ([axis-5.md](axis-5.md) → L-C2).
- **Axis 2 — a tier correction against the driver's own escalation.** `(2, N-4)` was filed **tier 2** on
  the claim that the main checkout's next `task finalize` is gated by the false finding. Driven twice on
  two fresh rigs, that door **absorbs** the drift (`advisory · reconciliation.absorb`, route *no action
  needed*) and lands at exit 0 — so the row survives as **tier 3**, and a supporting clause of it is
  falsified at the door itself ([axis-2.md](axis-2.md) → F-4).
- **Axis 3 — a source-pass completeness sentence refuted by a driven row.** Codex's `C-11` concluded that
  destroying-door paths render relative to `jigc_home`, from the rule's **one home**
  (`render::repo_relative`). Driven, `jigc task finalize`'s Displace surfaces **do not reach that home**
  from a linked worktree — the falsifying datum is `F-A` ([axis-3.md](axis-3.md) → §7.3).
- **Axis 6 — a blanket negative refuted.** Codex's *“no other grounded completeness defect was found”* is
  refuted by `A6-R1`, re-driven in full by the reconciler with both controls
  ([axis-6.md](axis-6.md) → §B claim 2, §C-1).

And the demotion pass fired on two of the four axes: **axis 2 demoted five driver row-groups** (four
re-driven and holding; one — `migrate.source-untracked`'s `git_at` span — **driven by neither pass and
still demoted**), **axis 3 demoted four rows** (all four re-driven at their own doors, all four landing
where the driver filed them), **axis 5 demoted two** (`(5, C1)`, restored by a new repro; and §10's
*“11 / 11 + 7 / 7 cwd-review findings re-verified closed”* count, corrected to **6 / 11** and **3 / 7**
driven-with-a-repro, the rest *not observable in release posture* rather than *closed*), and **axis 6
demoted none** (every one of its 45 rows carries argv + exit + asserted surface inline, and a 12-row
sample was re-driven independently).

---

## Roll-up

| axis | subject | CONFIRMED | REFUTED | OPEN | new findings (tier) | doors covered (baseline →) |
|---|---|---|---|---|---|---|
| 2 | posture | 22 | 2 | 6 | **4 — 0 tier-2 · 4 tier-3** (2 inside the arc) | **25** (rc.18: 47) |
| 3 | destroying doors | 13 | 1 | 3 | **2 — 1 filed tier 2/3 · 1 tier-3** | **18** (rc.18: 15) |
| 5 | pinned contracts | 6 | 0 | 0 | **0** | **47** (rc.17: 47) |
| 6 | composed surfaces | 14 | 1 | 7 | **1 — tier 3** (= axis 2's N-1; inside the arc) | **26** (M52: 24) |
| **total** | | **55** | **4** | **16** | **6 distinct — 0 tier-1** | **47 / 47 union · uncovered: none** |

**The counting basis, stated rather than smoothed** (the rc.18 record's own precedent). The triples above
are the **orchestration's hand-off tallies**. They count ledger *entries* — every Codex claim driven to a
verdict, plus every driver defect re-driven — not findings, and **two of them do not reproduce from their
file's own ledger**: axis 2's file states *“twenty-one claims … 17 CONFIRMED · 0 REFUTED · 4 OPEN LEAD”*
while its ledger carries **22** headings (`CX-1`…`CX-21` with `CX-7` split into `7a`/`7b`) of which
**19 are CONFIRMED and 3 OPEN** — the hand-off's *refuted 2* is F-4's two falsifications (the tier-2
escalation leg and the *“no surface declares”* clause), and its *open 6* is §5's six unsettled items, of
which three are Codex claims and three are the reconciler's own. Axis 5's *confirmed 6* is its six carried
defects, not its ledger's rows (3 Codex claims + 7 driver rows + 6 prior-row dispositions).
**The auditable set is the enumeration in §FINDINGS below, not this table**, and where a number here
disagrees with a number in an axis file, **the axis file governs**, because it carries the repro.

**The four headline facts of the third partial re-run:**

1. **Zero tier-1 rows on all four axes**, each reached independently, each with before-controls.
2. **The cwd arc's own promises are CLOSED, driven** — not asserted: the aimed route runs from five cwds
   and from two shell-hostile roots; `milestone finalize` lands from inside a fan-out worktree; the
   base-mismatch gate fires from a worktree; `uninstall` takes the **main** install, prunes git's worktree
   admin and says so; the `Spawn:` `cd` is absolute and POSIX-quoted; the pack-set no longer vanishes
   inside a worktree; `migrate <PATH>` is cwd-based with its git half aimed.
3. **No previously-closed row re-opened on any axis, and no regression was found** — with one exception
   that is itself a new finding: `A6-R1`/`N-1` is a **law-1 regression the arc introduced**.
4. **Six new findings, two of them inside the arc's own new code**, both on the installed pre-commit hook.

---

# THE ROW-BY-ROW COMPARISON — every baseline §A row, all tiers

Four layers, one per axis, each against **that axis's own baseline**. **CLOSED** carries the argv that
settles it; **STILL-OPEN** carries the datum. Rows M53 deliberately did not fix — the charter's tier-2/3
triage into the 1.x ledger — are marked **STILL-OPEN(1.x, expected)**, which is a *measurement*, not a
finding. A row is keyed **(axis, id)** exactly as its record keys it.

## Layer A — axis 2 against [per-axis-review-rc18](../per-axis-review-rc18/README.md) §A (and the M52 rows it carried)

| baseline row | tier | verdict on rc.19 | the argv / the datum |
|---|---|---|---|
| **`(2, F-1)`** — the aimed route `git -C <repo-relative-path> …` does not run from the checkout that printed it | **2** | **CLOSED** | Driven at **both** `aim_at` callers (`BreachSite::aim`, `milestone::held_here`) across **four** doors — `milestone finalize`, `task validate <sub>`, `milestone discard`, `uninstall` — from cwds (a)(b)(c)(d); the emitted span run **verbatim** from five cwds including `/`, **rc=0 every time**, and again under a spaced repository root where `shell_operand` single-quotes it. Re-driven independently by the reconciler (CX-8). [axis-2.md](axis-2.md) §3.1, §12 CX-8 |
| **`(2, F-2)`** — a foreign repository's worktree parked at `.jigc/worktrees/<sub-id>` is committed as the sub-task's own work | 3 | **STILL-OPEN(1.x, expected)** | Reproduces exactly, and independently by the reconciler on its own second repository: `join` → 0, `finalize` → 0, `added bsecret.txt … sub-tasks: fw-area: 1 code file`; `git cat-file -p HEAD:bsecret.txt` → `B-SECRET-PAYLOAD`. **No byte lost** — B's untracked plant and worktree registrations intact, `.jigc/displaced/` absent. [axis-2.md](axis-2.md) §4.3, §12 CX-1 |
| **`(2, DEFECT A)`** (M52) — a clean `git cherry-pick --no-commit` is a member of no `InProgress` row and `task finalize` concludes it at exit 0 | **1** | **still CLOSED** | All four `uncommitted-pick*` git states × 5 acting doors → exit 1, `repo.operation-in-progress`, *an uncommitted cherry-pick is in progress*. The discriminating cell is `uncommitted-pick-conflicted`: its index **is** conflicted and it still answers the pick, not `UnmergedIndex`. No consent flag bypasses (§3.6). [axis-2.md](axis-2.md) §3.3, §12 CX-2/CX-3 |
| **`(2, DEFECT C)`** (M52) — a posture breach at the **commit seam** prints no state-truth clause and no copy-runnable re-run | 2 | **STILL-OPEN(1.x, expected)** | Reproduces byte-for-byte with a deterministic `git` shim racing a bisect into the finalize transaction: the posture arm prints one code and one `route:` line **and nothing else**, while HEAD is unmoved, `work.txt` still staged, the task's docs intact. The contrast arm one step apart (a rejecting `pre-commit`) prints the full state-truth clause **and** `jigc task finalize hook-probe`. [axis-2.md](axis-2.md) §4.1, §12 CX-6/CX-11 |
| **`(2, DEFECT B)`** (M52) — a conflicted `git merge --squash` is answered by `SquashMerge`, whose predicate is false of the state | 3 | **STILL-OPEN(1.x, expected)** | `git merge --squash cb` (conflicting) → `.git` holds `MERGE_MSG` + `SQUASH_MSG`, `git ls-files -u` → 3 unmerged; `jigc milestone create` → exit 1, *“a squash merge is **staged and not committed**”*. Nothing is staged. [axis-2.md](axis-2.md) §4.2, §12 CX-10 |
| **`(2, DEFECT 1)`** (rc.17's tier-1 row) — the boundary commits a provisioned worktree's un-concluded operation at exit 0 | **1** | **still CLOSED**, with a stated narrowing | The worktree subject refuses at `bisect` across all four cwds and at both producers, the refusal placed **before** the record flip (record still finalizable, HEAD unmoved, staged `subwork.txt` intact); the ten `InProgress::ALL` members are re-driven member-by-member at the **main-checkout** subject. **Narrowing, declared:** rc.18 drove all ten members × the *worktree* subject; this run drove one member there, on the ground that the arc changed the **render**, not the detection ([axis-2.md](axis-2.md) §5.2) |
| M51 **`D1`** (rebase/bisect answer the operation, not the detachment) · **`D2`** (`am` vs apply-backend rebase disjoint) | — | **still CLOSED** | §3.3's 16-state sweep: `rebase-merge` and `rebase-apply` are HEAD-detached in git and still answer *a rebase*; only the bare `detached` state reaches `repo.head-detached`. Two nouns, two abort commands. [axis-2.md](axis-2.md) §3.3, §12 CX-4/CX-5 |

## Layer B — axis 3 against [per-axis-review-rc18](../per-axis-review-rc18/README.md) §A (and the M52 rows it carried)

| baseline row | tier | verdict on rc.19 | the argv / the datum |
|---|---|---|---|
| **`(3, F-3)`** — `jigc uninstall` inside a fan-out worktree exits 0, reports an install it did not remove, and takes the repository's shared `pre-commit` hook | **2** | **CLOSED** | From inside the worktree the door now binds `jigc_home`: the **main** install is removed in full (`.jigc/`, `CLAUDE.md`, `SKILL.md`, hook), git's worktree registrations are **pruned** (`git worktree list` 2 → 1), and the site line says ``removed at `<ABS main>` — … and that workbench held the worktree you are standing in, which this removed``. All four WIP guards fire from there — `uninstall.dirty-worktree`, `uninstall.untracked-workbench-file`, `uninstall.foreign-bytes`, `uninstall.staged-prose` — **byte-identical from root and from the worktree**. [axis-3.md](axis-3.md) rows 31–38, R-2 · R-11 |
| **`(3, F-1)`** — a symlink wearing a staged identity is jigc's own at the read/ack surfaces and a third party's at the same door's destroying probe | 3 | **STILL-OPEN(1.x, expected)** — and now carrying a **third** surface | `task discard` → `task-discard.foreign-bytes` naming `adr:via-symlink.md`; `doc list --task` → *`adr:via-symlink … managed`*; and the cell Codex predicted and the reconciler drove: `task discard --force` → *“dropped staged edits to: **adr:via-symlink**”* — jigc's own, again. External target **1/1 intact**. [axis-3.md](axis-3.md) R-20 · R-23a |
| **`(3, F-2)`** — the residual note asserts *“is a directory”* over a shape it did not check | 3 | **STILL-OPEN(1.x, expected)** — and now driven at a **second door** | `ln -s $OUT .jigc/tasks/ghost` → `task discard ghost` → ``no task `ghost`: `.jigc/tasks/ghost` **is a directory** carrying no base pin``; `task validate ghost` → the **byte-identical** sentence (one producer, `engine::state::residual_area_note`). Real-directory control makes the sentence true there. [axis-3.md](axis-3.md) R-20 · R-23b |
| **`(3, F-4)`** — the fan-out teardown narrates a **deleted tracked** file as *“never staged”* under *“the only copy … not recoverable”* | 3 | **STILL-OPEN(1.x, expected)** | `keeper.md (never staged)` printed under the unrecoverable-bytes note, while after the run `git show HEAD:keeper.md` returns the bytes and the main checkout's copy is untouched. [axis-3.md](axis-3.md) row 76, R-7 |
| **`(3, F-5)`** — a settled sub-task's leftover is *“1 active task(s)”* at one door and *“a leftover, not live work”* at another, and the finalize door's route names a command that refuses | 3 | **STILL-OPEN(1.x, expected), all three halves** | `task list` → *1 active task(s)* / `area-two [sub-task]`; `task discard area-two` → `milestone.terminal`; `--force` → refuses identically; `task finalize` → `finalize.empty-commit` whose route names that refusing command. HEAD unmoved. **Reachability still undischarged** — the state is built by restoring a `cp -R` backup; no jigc sequence reaches it. [axis-3.md](axis-3.md) rows 59, 82, R-14 |
| **`(3, A3-1)`** (M52) — `milestone finalize` destroys every byte jigc did not write in `.jigc/milestones/<id>/` at exit 0 | **1** | **still CLOSED** | 8 planted loci (area root · nested dir · `merged/top.txt` · `merged/docs/{deep,provenance,adr}` · `merged/sub/` · a sub-task area) → exit 0, **8 before / 8 after**, 8 pairs on `committed.displaced` sorted by `from`, each foreign directory moved **whole**, both area roots empty. [axis-3.md](axis-3.md) row 69, R-13a |
| **`(3, A3-2)`** (M52) — when the displacement fails, both `Displace` doors remove the area anyway | **1** | **still CLOSED at both doors** | `.jigc/displaced` a regular **file**: `task finalize` → exit 0, area **left standing**, 1 before / 1 after, advisory `finalize.foreign-bytes` naming `Not a directory (os error 20)`; `milestone finalize` → 3/3 on disk, one advisory **per standing area**, `displaced: []`, 0 host-path hits. [axis-3.md](axis-3.md) rows 61, 70, R-13b · R-24b |
| **`(3, A3-3)`** (M52) — the boundary's own area is untested | 3 | **still CLOSED behaviourally** | The same plant set finalized **from inside a fan-out worktree**: 3 before / 3 after, all parked, HEAD moved on **main**, 0 host paths. The structural half remains a source read (Codex `C-6`), the same disposition rc.17 and rc.18 gave it. [axis-3.md](axis-3.md) row 71, R-13c |
| the rc.18 post-review **HIGH** — a *spotless* worktree carrying an un-concluded operation | — | **holds at all three worktree doors** | `provision` → `milestone.leftover-holds-work`, `discard` → `milestone.dirty-worktree`, `uninstall` → `uninstall.dirty-worktree`, each naming the operation and routing `git -C <ABS> bisect reset` — **and that route now runs verbatim from three cwds and from a spaced root**. [axis-3.md](axis-3.md) rows 9–10, 20–21, 38, R-5 · R-6 · R-12 |
| the rc.18 post-review **MEDIUM 1** — 0 host paths at `milestone finalize` from a sibling worktree | — | **holds** — **and is the class `F-A` shows un-swept at `task finalize`** | `milestone finalize`'s Displace from a linked worktree: `committed.displaced` and the stderr note **repo-relative**, 0 host paths (row 75). The identical cell at `task finalize` from the same cwd: **3 host-absolute hits**, including on the 1.0-pinned key. [axis-3.md](axis-3.md) R-21 · R-24a |
| census **C2-06** (`milestone finalize` unreachable from any worktree) · **C2-07** (= F-3) · **C1-06 / C1-14** (the `git -C <rel>` dead end; the unquoted `cd`) | — | **all CLOSED** | C2-06: the full landed envelope from inside the worktree, `main` HEAD advanced, both areas cleared. C1-06/C1-14: both spans run verbatim, including under a spaced root. [axis-3.md](axis-3.md) R-4 · R-2 · R-5 · R-6 |

## Layer C — axis 5 against [the rc.17 run](../per-axis-review/README.md) §A

| baseline row | tier | verdict on rc.19 | the argv / the datum |
|---|---|---|---|
| **`(5, DEFECT 1)`** — a mint door commits a record at a fabricated identity | **1** | **CLOSED (stays closed)** | 5 unslugable titles × `milestone create` → exit 1 each, `write.unslugable-title`; `git rev-parse --short HEAD` **`46c3233 → 46c3233`**, `git status --porcelain` empty, `.jigc/milestones` absent. [axis-5.md](axis-5.md) §2.1 |
| **`(5, DEFECT 2)`** — `config remove-step`/`replace-step` refuse a step that **is** in the resolved include list | 3 | **STILL-OPEN(1.x, expected)** — and its repro is now **confound-free** | On a virgin `fresh` rig: `insert-step` lands `probe-step`, `start --explain` prints it in the include list, `remove-step` → `config.anchor-absent — no step \`probe-step\` body to fork`, `replace-step` → `config.step-id-collision`; control `remove-step …#implement` → 0. The driver's block had its control run first; the reconciliation removed that confound. [axis-5.md](axis-5.md) §2.2 + ledger |
| **`(5, DEFECT 3)`** — the colon-less-address bail is outside `RefusalKind` and carries no code | 3 | **STILL-OPEN(1.x, expected), still wider than reported** | `rename vision --to "New Vision"` → `{"error":"\`vision\` is not a \`<type>:<slug>\` address …"}`; `doc show nosuchtype` → the same bail under a **second wording**. No `blocking · <code>`, no key, at either door — **two producers, not the one `rename.rs` bail Codex cited**. [axis-5.md](axis-5.md) §2.3 |
| **`(5, DEFECT 4)`** — `doc show` blocks a **declared but unpopulated** optional leaf | 3 | **STILL-OPEN(1.x, expected)** | `doc schema vision --format json` advertises `vision:<slug>#meta/grounded-in` `required: false`; `doc show` of it → exit 1, `store.no-such-leaf` — the schema surface advertises the address the read surface calls nonexistent. [axis-5.md](axis-5.md) §2.4 |
| **`(5, C1)`** — the pinned orientation rows declare `next_steps`, a reachable composition omits it | 3 | **STILL-OPEN(1.x, expected)** — **demoted as filed, then driven, and wider** | On a pack-set with neither off-catalog verb (a `mv` into a `mktemp -d` stash, not a delete): `Clean` drives `[header, schema_version, state, workflows]` against a declared set carrying `next_steps`; **`ActiveTask` omits it too** — declared ⊋ driven at **two** of the four `start` arms. The producer is deliberate and unit-tested, so it is the **declaration** that overstates. [axis-5.md](axis-5.md) L-C2 |
| **`(5, D1)`** — `start --explain` emits a production `--format json` stdout arm `ENVELOPE_ARMS` does not carry | 3 | **STILL-OPEN(1.x, expected)**, widened | exit 0, stdout 1218 bytes, keys `[collision_winners, overrides_applied, pack_inputs, schema_version, steps, workflow, workflow_layer]`; the registry's four `start` rows are three `OrientationView` variants + `Composed`. **Widened: the bare `start --explain` emits the same undeclared shape** — not conditional on `--workflow`. [axis-5.md](axis-5.md) §2.6, L-C1 |
| **rc.17 `DEFECT A`** — `store.unknown-type` emits a **URI-shaped** target at `doc show` where the contract fixes the bare doctype id | 3 | **STILL-OPEN** (unfixed — no M53 wave addressed it) | `doc show 'nosuchtype:x'` → key `{store.unknown-type, "nosuchtype:x"}`; `doc schema nosuchtype` → `{store.unknown-type, "nosuchtype"}`. One code, two target shapes, so the stable key does not discriminate the same condition the same way at two doors. [axis-5.md](axis-5.md) §2.7, L-C3 |
| M51 **`DEFECT A · B · C · D`** | — | **all still CLOSED** | The two uniform hostile-cwd sweeps: **outside any git repository** 47/47 land in a declared arm (45 `Reject::Error`, 2 `Reject::Findings` — `setup`/`uninstall`), 0 NOT-JSON, 0 bare-`Finding` roots; **cwd deleted under the process** 47/47 `Reject::Error` with one identical `{"error": "cannot determine the current directory: …"}`. 10 cells independently spot-driven by the reconciler. [axis-5.md](axis-5.md) §6.1 · §6.2 |

## Layer D — axis 6 against [M52's per-axis review](../../M52/per-axis-review/README.md) §A, **all tiers**

| baseline row | tier | verdict on rc.19 | the argv / the datum |
|---|---|---|---|
| **`(6, D-1)`** — orientation reports a live task's findings **without** the repository posture, while its own route line promises the posture | 2 | **STILL-OPEN(1.x, expected)** | `git bisect start` → `jigc start` exit 0 with three findings and `grep -c 'operation-in-progress'` = **0** on the text arm **and** on the pinned wire; `jigc task validate posture-parity` → exit 1 `repo.operation-in-progress`. Re-driven by both driver and reconciler. [axis-6.md](axis-6.md) R-D1, §C-3 |
| **`(6, D-2)`** — `fix-task` is composable by name and the composed walk's **forbidden** door is the only one that works | 2 | **STILL-OPEN(1.x, expected)** | The composed text line 25: *“Never `git commit` and never `jigc task finalize` here.”* → fill the commit doc → `git add fixed.txt` → `jigc task finalize fix-the-thing` → **exit 0**, *“finalized … fix: fix the thing · 1 file committed”*. [axis-6.md](axis-6.md) R-D2, §C-3 |
| **`(6, D-3)`** — the orientation `Preview:` footer states the pre-M52 rule that `milestone-execution` falsifies | 3 | **STILL-OPEN(1.x, expected)** | Footer: *“a workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly”*; `jigc start --workflow milestone-execution` → exit 1 `workflow.verb-routed`; `--workflow router` → 0 (the control). The **swept sibling sentence** at `describe --workflows` is correct, which is why this one reads as an un-swept remainder. [axis-6.md](axis-6.md) R-D3, §C-3 |
| **`(6, D-4)`** — a legitimately-empty `milestone execute` walk does not state its empty case | 3 | **STILL-OPEN(1.x, expected)** | `milestone create "Empty probe"` → `milestone execute empty-probe` → exit 0, `grep -c '^Spawn:'` = **0**, stderr **0 bytes**, no sentence naming the empty case; `milestone finalize` then exits 3 `milestone.zero-contribution`. [axis-6.md](axis-6.md) R-D4, §C-3 |
| M51 **`A6-1`** (the `resume:` line discriminates the posture it is composed in) | — | **still CLOSED** | Driven from five cwds for one sub-task: its **own** worktree (*“run it here”*, exit 0), a **sibling** worktree (*“this checkout is not that worktree — run it from …”*, exit 0), and root/subdir/linked (all off the pin) — the base-pin refusal, **byte-identical** across the three, routing at a quoted absolute `cd`. [axis-6.md](axis-6.md) row 7, §B claim 3 |
| M51 **`A6-2`** (named composition refuses verb-routed workflows) | — | **still CLOSED** | The **re-derived** 14-member `suppressed.door` set × 3 argv forms = **42 drives, 42 exits of 1, 42 `workflow.verb-routed`, zero mints** (`task list` unchanged before/after). [axis-6.md](axis-6.md) row 3, §B claim 4 |
| M51 **`A6-3`** (the empty-case clauses stand) | — | **still CLOSED**, and its second half **upgraded from a source read to a drive** | `implement-from-spec` over a zero-spec corpus states both empty cases; `task bind`'s `store.not-found` / `store.unknown-type` **driven** onto the findings envelope with their `(code, target)` keys — the driver had established that half by reading the constructor. [axis-6.md](axis-6.md) §B claim 5 |
| M52's open leads — **O-6** (the `suppressed:` guarantee is manifest-scoped) · **O-1** (`jigc migrate` mints silently over open work) · **O-2** (the malformed-address refusal carries no code) | — | **all STILL-OPEN, unchanged**, each inside a declared bound | O-6: a manifest-less pack's `creates-task: false` workflow loads clean, `describe` narrates it with no reason. O-7 here: `migrate` minted with **15** tasks already live → **0** `also open:` bytes, while the next `start` renders *“also open: 16 other tasks”*. O-8: `doc show padding` → the malformed-address bail, no code, on both arms. [axis-6.md](axis-6.md) §6 |

---

# FINDINGS

## A · CONFIRMED — every new finding, tiered on the charter's predicate

The predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.

### Tier 1 (0)

**None.** Every tier-1 candidate on every axis was driven to a safe outcome **with a before-control**:
`(2, F-1)`'s whole cell (HEAD unmoved, record finalizable, staged work intact at every member) ·
`(3, A3-1)` (8 before / 8 after) · `(3, A3-2)` (both areas left standing at both doors) · the destroying
doors' operation leg (nothing removed, marker and worktree on disk) · `(3, F-3)`'s closure (the untracked
cell **guarded**, the removals tracked and named) · `(5, DEFECT 1)` (HEAD byte-unmoved over five
refusals) · `(6, A6-R1)` (no byte dies, no commit wrongly blocked) · `(2, F-2)` and `(2, N-4)` (measured:
nothing lost in either).

### Tier 2 / 3 — the one row neither pass would collapse (1)

#### `(3, F-A)` · origin **driver**, re-driven from scratch by the reconciler with a cwd control · **outside the cwd arc's new code** · `jigc task finalize`'s Displace surfaces print host-absolute paths from a linked worktree, **including on the 1.0-pinned envelope**

```
setup: dev/jigc-rig fresh; git -C $REPO worktree add -b feat $REPO/../linked
       (cd linked) jigc start --workflow quick-fix "tidy the readme"; echo code > linked/src.txt; git add
       the commit doc filled from `linked`;  KEEPHP > $REPO/.jigc/tasks/tidy-the-readme/notes.txt
BEFORE plant: 1

$ (cd linked) jigc --format json task finalize tidy-the-readme            -> exit 0
  "committed": { "displaced": [ {
      "from": "/private/var/folders/.../repo/.jigc/tasks/tidy-the-readme/notes.txt",
      "to":   "/private/var/folders/.../repo/.jigc/displaced/tidy-the-readme/notes.txt" } ] }
  stderr  "… they were moved aside, not taken:"  + the same two absolutes
  host-absolute hits (stdout+stderr): 3                      AFTER plant: 1   <- NO LOSS

$ the NONE-MOVE cell (printf 'NOT A DIR' > .jigc/displaced), same cwd      -> exit 0
  findings[0].message  three host-absolute path occurrences
  findings[0].route    one host-absolute path occurrence
  findings[0].key      {"code":"finalize.foreign-bytes","target":"task:tidy-the-readme"}   <- CLEAN
  findings[0].location {"address":"task:tidy-the-readme", …}                               <- CLEAN

CONTROL — the identical fixture from the REPO ROOT                         -> exit 0
  committed.displaced [{"from": ".jigc/tasks/…/notes.txt", "to": ".jigc/displaced/…/notes.txt"}]
  host-absolute hits: 0                                      <- THE CWD IS THE VARIABLE
CLEAN SIBLING — the identical cell at `milestone finalize` from `linked`   -> hits 0
REACHABILITY BOUND, driven — from a FAN-OUT worktree the door refuses first:
  (cd W) jigc --format json task finalize …  -> exit 1  repo.head-detached ; plant 1/1
```

**Contract contradicted:** `design/surface-contract.md` → *The printed-path fence (law 1)* — one home,
`crate::render::repo_relative`, repo-relative and `/`-separated, with the honest absolute reserved for *“a
path genuinely outside the repository”*. `.jigc/tasks/<id>/notes.txt` is genuinely **inside** the
repository; it is outside only the **standing checkout**. **Why not tier 1, measured rather than
assumed:** the plant is 1/1 before and after in every cell, the move succeeds, and the failed-move cell
leaves the area standing exactly as contracted. **Why the record does not collapse it to tier 3:** two of
the three affected surfaces are the **1.0-pinned** `committed.displaced[].from`/`.to` keys, so the wrong
value rides a contract that closes at 1.0 — which is more than a sentence being wrong. **Inside the cwd
arc's new code? NO** — the arc did not touch this render, and the cwd it fires from (a branch-attached
linked worktree at `task finalize`) was reachable before it. It is the **un-swept sibling of M53's own
rc.18 post-review MEDIUM 1**, which closed this class at `milestone finalize`; whether *that* makes it a
finding inside a fix pass's own new code is an adjudication this record leaves to the human.
[axis-3.md](axis-3.md) §2 F-A, R-21 · R-22 · R-24a · R-24b; §7.3.

### Tier 3 (5)

#### `(2, N-1)` = `(6, A6-R1)` · origin **two independent drivers**, re-driven by both reconcilers · **INSIDE the cwd arc's own new code** · the installed pre-commit hook announces an out-of-band **rename** on a commit that contains none, and calls a staged change *“not staged in this commit”*

```
# rig: committed-singletons (fresh; `jigc validate` exits 0 and the first commit is silent)
BEFORE-CONTROL: printf 'ordinary\n' > ord.txt; git add ord.txt; git commit -m "probe"
  -> rc=0, the hook prints NOTHING

CELL 1 — a committed DELETION makes every later commit lie:
  git rm -q VISION.md ;  git status --porcelain -> "D  VISION.md"   (a deletion; no rename anywhere)
  git commit -m "probe: delete a managed singleton"                 -> rc=0
  > jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for
  >   details (not staged in this commit; commit not blocked).
  printf 'b\n' > f2.txt; git add f2.txt; git commit -m "unrelated 2" -> rc=0
  > …the same sentence again, and on every commit from here on.
  the report the guard grepped:  grep -c 'git mv' -> 0   grep -o 'git -C ' -> 1
  the single span:  git -C <ABS>/repo show <sha> -- VISION.md      <- a `show`, not a move

CELL 2 — a STAGED placement-doc `git mv` is told it is not staged:
  git mv docs/roadmap.md docs/plan.md ; git commit -m "…"           -> rc=0
  > …(NOT STAGED IN THIS COMMIT; commit not blocked).
  git show --name-status --find-renames --format= HEAD -> R100 docs/roadmap.md docs/plan.md

CONTROLS — the genuine block is intact, and the space axis really is closed:
  a LOCATION doctype (adr) at the default docs-root:  git mv … ; git commit -> rc=1, commit BLOCKED
  the same under `jigc config set docs-root "my docs"`:              git commit -> rc=1, BLOCKED
```

**The cause, read after it was driven.** `crates/cli/src/setup.rs:620`'s `PRECOMMIT_RENAME_BLOCK` gates on
`moves="$(printf '%s' "$report" | grep -o 'git -C [^\`]*')"`. Before the arc it was
`grep -o 'git mv [^\`]*'`; commit **`65de53f5`** (*“every operator-facing `git` span names the checkout it
runs in”*) rendered the rename revert as `git -C <abs> mv <new> <old>`, so the grep was widened to
`git -C` — which is now the prefix of **every** operator-facing span in the report (`home-vacated`'s
`show`, `owner-artifact`'s `add`, `file_state`'s restores). The gate `[ -n "$moves" ]` therefore passes
for reports containing no rename route at all, the `awk` correctly finds no `mv` pair, and the **`else`**
branch prints the sentence unconditionally. **Why not tier 1:** nothing is destroyed, no commit is
wrongly blocked, and `jigc validate` catches the real condition and **flips the exit** (exit 1, measured
bare). **The axis, derived rather than reported:** *every finding whose route carries an aimed `git` span
and is not a rename revert* — at HEAD that is most of census 1. **The smallest correct predicate** is the
one the pre-M53 grep had: match the **`mv` subcommand**, not the `-C` prefix — or keep the wide grep and
move the `[ -n "$moves" ]` gate **after** the awk. [axis-2.md](axis-2.md) §6 N-1, §12 F-1 ·
[axis-6.md](axis-6.md) §5 A6-R1, §C-1.

#### `(2, N-2)` · origin **driver**, re-driven · **INSIDE the cwd arc's own new code** (the covering sentence; the inertness is older) · the hook's *blocking* rename backstop is structurally unreachable for the **entire placement-doctype family**

```
setup (×4, one fresh `committed-singletons` rig each)
argv  (×4): git mv <A> <B> ; git status --porcelain ; git commit -m "probe <A>"

  VISION.md             -> VISION-OOB.md          | staged: R … | rc=0
  CHANGELOG.md          -> CHANGELOG-OOB.md       | staged: R … | rc=0
  docs/roadmap.md       -> docs/roadmap-oob.md    | staged: R … | rc=0
  docs/decisions-log.md -> docs/dl-oob.md         | staged: R … | rc=0
each printing "…an out-of-band managed-doc rename exists in the committed tree … (not staged in this
              commit; commit not blocked)."          <- the sentence that covers for the missing block

CONTRAST — a LOCATION doctype (milestone-record), same binary, same hook:
  docs/milestone-records/leftover-wave.md -> …/zz-oob.md | rc=1, commit BLOCKED

THE ASYMMETRY AT ITS SOURCE (`jigc validate`, one rig per family):
  location : "… is missing; …/zz-oob.md has the same content hash — likely renamed via `git mv`"
             route: … or revert the move: `git -C <abs> mv <new> <old>`      <- the pair the awk needs
  placement: "tracked managed doc vision:vision (VISION.md) is missing"
             route: restore VISION.md, or … `jigc unmanage VISION.md`        <- NO pair, ever
```

`reconciliation.rename` pairs old↔new by content hash and emits the `mv` revert route **only for a
`location` doctype**, so the hook's `W[i]=="mv" && W[i+1] in S && W[i+2] in S` predicate can never be
satisfied for **any** placement doc, at any repository path — and every managed singleton a stock corpus
ships is a placement doctype. **Why not tier 1:** the renamed file is on disk, nothing is destroyed, and
`jigc validate` raises `reconciliation.rename` **and** `schema-conformance.home-vacated` and exits
non-zero (measured bare). The defect is that a **blocking** backstop is silently inert over half its
subject while printing a line that asserts the opposite. **Filed separately from N-1 on purpose:** one
surface, two independent causes, two different fixes. [axis-2.md](axis-2.md) §6 N-2, §12 F-2.

#### `(2, N-3)` · origin **driver**, re-driven · **outside the cwd arc's new code** (M49's reuse rule) · `jigc milestone provision` acks *“at base &lt;pin&gt;”* over a worktree it **reused** at a different commit, and the boundary's later refusal repeats the false claim

```
setup: rig committed-singletons
       jigc milestone create "Reuse wave"     -> "minted milestone:reuse-wave (shared base 621706f)"
       jigc milestone add-task reuse-wave "ru one"
       printf 'x\n' > extra.txt; git add extra.txt; git -c core.hooksPath=/dev/null commit -m "chore: advance"
       git worktree add -q --detach "$REPO/.jigc/worktrees/ru-one"       (NOT provisioned by jigc)
BEFORE: milestone pin 621706f   ·   the parked worktree's HEAD 75ff78b
argv : jigc milestone provision reuse-wave                               -> exit 0
  "provisioned 1 worktree(s) for milestone:reuse-wave at base 621706f (ru-one)"
AFTER: git -C .jigc/worktrees/ru-one rev-parse --short HEAD -> 75ff78b    <- NOT 621706f
then : jigc milestone finalize reuse-wave                                -> exit 3
  blocking · finalize.base-mismatch — … the sub-task worktrees were cut from `621706f…`
```

The reuse itself is **declared** (`provision_route`'s doc-comment, M49: *“a worktree already registered at
a sub-task's path is reused untouched”*). What is not declared is the ack stating the milestone's pin as
the base of a worktree standing somewhere else — and the boundary's refusal then repeating it. Two law-1
claims the binary cannot support. **Why not tier 1:** the boundary **blocks** at exit 3, nothing lands,
no byte dies. [axis-2.md](axis-2.md) §6 N-3, §12 F-3.

#### `(2, N-4)` · origin **driver** (filed tier 2), **tier corrected to 3 by the reconciler**, confirmed in part · **outside the cwd arc's new code** (M46's *Concurrent writers*) · a promoting `task finalize` from a linked worktree leaves the main checkout with a finding that names an out-of-band edit that never happened

```
setup: rig; git -C $REPO worktree add -q $RIG/lw -b lwbr ; cd $RIG/lw
       jigc start "record the linked decision" --workflow decided-task
       jigc doc add-item decisions-log#entries --task … --title "Linked branch decision"  (copied in)
BEFORE: md5 $REPO/docs/decisions-log.md == md5 $RIG/lw/docs/decisions-log.md
argv : jigc task finalize record-the-linked-decision                     -> exit 0
AFTER: main HEAD UNMOVED [main] · lw HEAD d4cae97 [lwbr]
       grep -c 'Linked branch decision' $REPO/docs/decisions-log.md  -> 0
       grep -c … $RIG/lw/docs/decisions-log.md                       -> 1
then : (cd $REPO && jigc validate)   -> exit 0 (store scope), carrying:
  blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions-log.md`
    differs from the recorded state
    route: review the out-of-band edit … and re-author it through the owning workflow

THE TIER-2 LEG, DRIVEN TWICE ON TWO FRESH RIGS — AND IT DOES NOT REPRODUCE:
  (in the MAIN checkout) jigc task validate main-real-work   -> exit 0
      advisory · reconciliation.absorb — external edit absorbed: `docs/decisions-log.md`
      route: no action needed — the external edit was absorbed into the baseline
                         jigc task finalize main-real-work   -> exit 0, landed
```

There **was** no out-of-band edit: jigc itself wrote that baseline into the shared `.jigc/state/` from
another checkout at exit 0. **What the reconciler corrected:** the driver's escalation (*“the main
checkout's next `task finalize` is gated and the route does not clear it”*) is false — the committing
door **absorbs** the drift, which is `design/reconciliation.md`'s designed behaviour — and the driver's
supporting clause (*“no surface declares that a commit target and a baseline home can be two different
checkouts”*) is falsified at the door itself, whose rc.19 ack says ``committed in the linked worktree at
`<ABS>` on branch `lwbr` — not in the main checkout jigc's workbench binds to``. **So the residual worth
carrying is the diagnosis, not a dead end** — tier 3, not tier 2. [axis-2.md](axis-2.md) §6 N-4, §12 F-4.

#### `(3, F-B)` · origin **driver**, re-driven with its shape control · **outside the cwd arc's new code** · the `LeftoverShape::File` consent arm lists the leftover's **own basename** as though it were a child entry

```
setup: fresh; milestone create "Quebec probe" + one sub-task (NOT provisioned)
       printf 'LEFTOVER-FILE-BYTES\n' > .jigc/worktrees/area-one      (file -b -> "ASCII text")

$ jigc milestone provision quebec-probe                                -> exit 1
  .jigc/worktrees/area-one: the file itself — it is a file, not a worktree, and nothing can say
    those bytes are disposable                                          <- the refusal arm, CORRECT
$ jigc milestone provision quebec-probe --force                        -> exit 0
  warning: removing the leftover file .jigc/worktrees/area-one discards work that is not in git:
      area-one                                    <- THE PATH'S OWN BASENAME, in the child position
CONTROL, the same arm over a DIRECTORY leftover holding precious.txt:
      precious.txt                                <- a real child
```

`milestone.rs`'s `LeftoverHold.entries` doc-comment states the rule the refusal arm obeys — *“**Empty for
the two shapes with no inside** … because listing an empty set beside either would read as ‘it holds
nothing’”* — and the `--force` arm does not obey it, one screen over from the wording built to prevent
that exact misread. Law 1, *nothing lies*. [axis-3.md](axis-3.md) §2 F-B, R-19 · R-24c.

## B · REFUTED claims — each with its falsifying datum (4)

| # | claim | pass | falsifying datum |
|---|---|---|---|
| `(3, C-11)` | *“paths [at the destroying doors] are rendered relative to `jigc_home`”* — the M51 row, as the source pass re-derived it from the rule's one home (`render::repo_relative`) | codex | **`F-A`.** `jigc task finalize`'s Displace does **not** reach that home from a branch-attached linked worktree: `committed.displaced[].from`/`.to` and the stderr note come back host-absolute (3 hits), while the **identical fixture from the repo root** gives 0. The reconciler records this as the shape M50's planning rule names — *a claim about how the composed product behaves is not established by reading the files it is composed from*. The rest of `C-11` (shape classification, outcome-filtered acks) is **CONFIRMED**. |
| `(6, CLAIM 2)` | *“No other grounded completeness defect was found”* — the source pass's blanket negative | codex | **`A6-R1`**, re-driven in full by the reconciler with both controls. The defect lives in a **shell script embedded in a Rust string literal** whose *consumer* behaviour is wrong, which the pass's own stated bound (*“source completeness, not runtime prose/help equivalence”*) puts outside what it could see. |
| `(2, N-4)`'s escalation leg | *“the main checkout's next `task finalize` is now gated by that finding, and the emitted route does not clear it”* | driver | Driven **twice**, on two fresh rigs, with a real code diff so the task is not empty: `task validate` → exit 0 with `advisory · reconciliation.absorb` (route *no action needed*), `task finalize` → **exit 0, landed**. |
| `(2, N-4)`'s supporting clause | *“what no surface declares is that a commit target and a baseline home can be two different checkouts at the same door”* | driver | The rc.19 ack declares exactly the commit-target half, in the same run: ``committed in the linked worktree at `<ABS>` on branch `lwbr` — not in the main checkout jigc's workbench binds to``. |

**Nothing either driver claimed was refuted outright** — the two driver refutations are a tier escalation
and one supporting sentence inside a row that otherwise stands. **Axis 5 refuted nothing at all: its two
passes disagree about nothing.**

## C · OPEN leads — driven as far as the state allows, promoted by nothing (16)

| # | lead | pass | why it stays open |
|---|---|---|---|
| `(2, CX-7b)` | M51 `codex-1`, the **displacement `git rm --cached`** leg: does an immediate re-probe precede it? | codex | Driving it needs **two** things at once — a foreign-squatter displacement state at a relocation destination, *and* a racer landing the posture inside that seam's own window. The rig builds no such state and the shim can only fire on a git call. **Not promoted on the source read.** (The sibling `git mv` leg **was** driven and is CONFIRMED — CX-7a.) |
| `(2, CX-17)` | `DedicatedWorktree` is unforgeable through public construction (private fields, private `add`) | codex | A claim about Rust item visibility. No state a driven binary can be put into exhibits or falsifies it; a fence for it is a compile-time fact. |
| `(2, CX-19)` | the M52 registries introduce no axis-2 bypass | codex | A negative over source structure with no named state to reach. Its one **drivable** projection — the pre-dispatch fault registry outranking the posture guard — is already a driven row with its own repro (axis 2 §3.8). |
| `(2, D-5c)` | `migrate.source-untracked`'s `git_at` span | — | **Driven by neither pass. Stays demoted** — the row claimed it and carried no block. |
| `(2, DEFECT C)` at the **milestone boundary's** seam | both passes raced `task finalize`'s seam only | — | The boundary's own commit seam is unraced by either pass. |
| `(2, concurrency)` | a genuinely concurrent racer on any rollback population | — | Out of this axis's scope and a declared bound in M52's own record; the shim is a deterministic injection at a known git call, not a process. |
| `(3, C-14)` | *“production removal sites are fully accounted for — unguarded removals confined to minted temporaries / empty-directory pruning”* | codex | A **completeness assertion over the whole production surface**, not reducible to an argv. Codex bounds it itself. The reconciler's spot census is **consistent with it and did not falsify it**: 111 `remove_dir_all` textual hits over both crates' `src/`, overwhelmingly the in-module `#[cfg(test)]` temp-root destructor; the one site Codex does not name (`invocation_log.rs:738`) is under `#[cfg(test)]` (line 565). |
| `(3, F-5)`'s reachability | no jigc sequence produces a settled sub-task's restorable leftover | — | The state was built by restoring a `cp -R` backup — by the rc.18 driver, its reconciler, **and** both agents here, the same way. Behaviour driven; **reachability undischarged**. |
| `(3, C-6)`'s structural half | production supplies both task and milestone `WorkArea` subjects to one teardown structure | codex | The **behavioural** half is CONFIRMED (the boundary's own area driven at both the success and move-failure cells). The shared-structure claim is a code fact the reconciler neither reproduced nor contradicted — the same disposition rc.17 and rc.18 recorded. |
| `(6, L-1)` = Codex claim 1 | a **non-UTF-8** repository path silently restores the cwd-dependent **relative** `Spawn:` form | codex | The trigger state is **unbuildable in this environment**, driven: `os.mkdir(b'nonutf8-\xff-dir')` → `OSError [Errno 92] Illegal byte sequence` (APFS), every mount APFS, and `hdiutil create -fs "MS-DOS FAT32"` → *Operation not permitted*. Three things the reconciler **could** drive bound it: the source leg is exact and the branch uniquely reachable in production; the one drivable neighbouring degradation (an **unprovisioned** sub-task) does **not** trigger it — the `Spawn:` line stays absolute; and the *“contradicts the adjacent contract”* leg is **partly falsified**, because both cited sites declare the fallback in the same breath as the guarantee. **So the open question is a declared degradation that is silent at runtime, not an undeclared contradiction.** |
| `(6, L-2)` = O-6 | the `suppressed:` guarantee is **manifest-scoped** | driver | Driven (a manifest-less pack's `creates-task: false` workflow loads clean and narrates with no reason); declared at `design/surface-contract.md`:127 / `design/introspection.md`:90. Unchanged from M52's open lead 1. |
| `(6, L-3)` = O-2 | the route-span carve-out enumerates **two** span kinds; the arc minted a **third** (`cd <abs> && jigc workflow …`) | driver | The **binary is right** and states its reason at the site (`compose.rs:1771-1793`); what is missing is that the disposition table and the carve-out do not carry the third kind, so the next producer emitting an absolute into a composed line is fenced by nothing. |
| `(6, L-4)` = O-1 | a store door binding `jigc_home` prints a repo-relative path the reader's own checkout also has | driver | Law 1's **declared** convention; recorded because the arc *created* this readership — before it, those doors acted on the checkout you stood in. |
| `(6, L-5)` = O-3 / O-7 / O-8 | the absolute host path inside the pinned `findings_unavailable` key · `jigc migrate` minting silently over open work · the malformed-address refusal carrying no code | driver | All three driven and unchanged from M52, each inside a **declared** bound (`UNSWEPT_PRODUCERS` · `design/write-commands.md`:189 · `design/command-output-contract.md`:294). |
| `(6, L-6)` | `pack-resource-missing` × this axis's doors | — | Declared not re-driven; M52 drove it at two doors and nothing in the arc touches `JIGC_PACK_DIR` resolution. |
| `(6, L-7)` | a project **pack** listed in `packs.yaml` (as opposed to the project config layer) | — | Declared not driven — reaching it means hand-writing a pack tree, and the brief's rule is to build fixtures with the binary. The **same seam** is driven both directions (the config-layer knob, and a malformed `packs.yaml` seen identically from all four cwds). |

**Axis 5 carries no open leads** — all three Codex claims were driven to repro blocks, the census counts
were verified by symbol, and the only undrivable half (the pass's *negative* that no other undeclared arm
exists anywhere in the code) is recorded as **corroborated by 274 driven hostile-cwd and four-cwd cells
rather than promoted or dropped**.

## D · Observations driven and carried — **not** defects

Recorded so a next reader does not re-drive them as findings. Each is graded against a written
declaration, quoted in its axis file.

- **The pre-dispatch fault outranks the posture guard** — a deleted cwd answers with one identical line at
  every leaf, ahead of every guard, carrying **no code and no route**. That is the shipped
  pre-dispatch-fault shape (M52 Increment 1), outside the findings envelope and therefore outside the
  route floor. Driven at 4 leaves on axis 2, at **5** doors on axis 6 (including the one cell M52 declared
  *not driven*), and at all **47** leaves on axis 5.
- **`BreachSite::Here`'s exit is 1 and the boundary gate's is 3** — both inside the declared taxonomy
  (`design/command-output-contract.md`: 3 = *blocking findings at a task-scope gate*, 1 = *every reject
  that is not*). Recorded, not filed.
- **`GIT_DIR`** — the posture family's **declared-out** residual with a written reopening condition; not
  re-driven here (rc.18 re-drove it and its disposition is written).
- **`OBS-1`** (`uninstall.dirty-worktree`'s route carries an **unfilled** `<milestone-id>` placeholder
  while the `staged-prose` sibling one screen over names the real milestone) — placeholders are the house
  style, so this is an inconsistency, not a contract break. Driven at its own line by the reconciler.
- **`OBS-2`** (`milestone finalize` from a branch-attached linked worktree commits onto **main** and says
  nothing about it) — exactly what `render::CommitSite`'s doc-comment prescribes. Recorded because a
  reader standing on `feat` watching `main` move is the cell a future review will want on the record.
- **`Disposition::Narrate` is empty by construction at rc.19** — an empty arm, not a skipped one. Likewise
  **`task finalize --force` and `milestone finalize --force` do not exist**: driven bare, both → exit 2,
  `error: unexpected argument '--force' found`.
- **The two four-cwd envelope divergences on axis 5 are declared-correct** — `config insert-step` /
  `config replace-step` from inside a fan-out worktree answer `config.step-source-untrackable` because the
  `file` argument's declared base is `Cwd` and the workbench-root guard fires first. **43 of 45 leaves are
  byte-identical across all four working directories**, code, key, route and exit included.
- **`finalize.render-io`'s host-absolute `at:` path** — a declared row of `repo_relative_paths.rs`'
  `UNSWEPT_PRODUCERS`: a counted, fenced, declared remainder. Graded against, not re-found.

---

# COVERAGE — the four axes against the 47 `VERB_KINDS` leaves

**The leaf count, read rather than quoted.** `crates/cli/src/cli.rs:1835`'s `VERB_KINDS` carries **47**
leaf rows at HEAD `a8904637`, counted by reading the `(&[…], VerbKind::…)` rows of the const itself
(`awk 'NR>=1830 && NR<=1890' crates/cli/src/cli.rs | command grep -cE '^\s+\(&\[' → 47`), and all four
axis files read the same **47** independently, by symbol. The spellings and their order are identical to
the rc.18, rc.17, M52 and M51 tables, so the columns below are directly comparable row by row.

**What the table reports.** Which of the **four re-driven axes** reached each leaf as the door of ≥1
**driven** row — the union of the four files' *Doors covered* sections — against the same column from each
axis's **own baseline**. `uncovered` is the set of leaves no re-driven axis reached.

> **`uncovered`: NONE — 47 / 47**, and it is carried by **axis 5 alone**, which is the door of ≥1 driven
> row at every leaf in each of its two uniform 47-leaf hostile-cwd sweeps. The stricter reading is
> reported beside it rather than smoothed into it.

**The stricter columns, two of them, because one number would hide the gradient:**

- **substantive** — the leaf is the door of ≥1 row whose assertion is **specific to that leaf**: a
  captured key set at a declared envelope arm, a posture/destroying/residual/composed cell, or a success
  or refusal arm with its own code and route. **47 / 47.** The mechanism, stated so it can be checked:
  axis 5's §3 drives **61 of the 64 `ENVELOPE_ARMS`** arm by arm with the key set captured and compared
  against the declared `ArmShape`, covering every leaf but two; the two — **`task bind`** and
  **`milestone add-from-spec`** — are carried by axis 6's four `task bind` refusal drives and axis 2's
  `milestone add-from-spec` posture cell respectively.
- **behavioural beyond the envelope** — the leaf is the door of ≥1 row on axis **2, 3 or 6**, i.e. a
  posture, destroying-door or composed-surface cell rather than a pinned-contract arm. **33 / 47.** The
  14 that are axis 5's alone: `upgrade` · `ingest` · `unmanage` · `doc add-item` · `doc remove-item` ·
  `doc retitle-item` · `doc rename` · `doc author` · `doc schema` · `config insert-step` ·
  `config replace-step` · `config remove-step` · `config fill` · `config fork`.

| # | leaf | rc.19 axes (driven) | rc.18 (2/3) | rc.17 (5) | M52 (6) | Δ | substantive | beyond envelope |
|---|---|---|---|---|---|---|---|---|
| 1 | `start` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 2 | `workflow` | 2, 5, 6 | 2 | 5 | 6 | — | ✔ | ✔ |
| 3 | `setup` | 2, 5, 6 | 2 | 5 | 6 | — | ✔ | ✔ |
| 4 | `uninstall` | 2, 3, 5, 6 | 2, 3 | 5 | — | **+6** | ✔ | ✔ |
| 5 | `upgrade` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 6 | `ingest` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 7 | `migrate` | 5, 6 | 2 | 5 | 6 | **−2** | ✔ | ✔ |
| 8 | `migrate-corpus` | 2, 5 | 2 | 5 | — | — | ✔ | ✔ |
| 9 | `unmanage` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 10 | `rename` | 2, 3, 5 | 2, 3 | 5 | — | — | ✔ | ✔ |
| 11 | `relocate` | 2, 5 | 2 | 5 | — | — | ✔ | ✔ |
| 12 | `describe` | 2, 5, 6 | 2 | 5 | 6 | — | ✔ | ✔ |
| 13 | `validate` | 2, 3, 5, 6 | 2 | 5 | — | **+3, +6** | ✔ | ✔ |
| 14 | `doc create` | 5, 6 | 2 | 5 | 6 | **−2** | ✔ | ✔ |
| 15 | `doc add-item` | 5 | 2 | 5 | 6 | **−2, −6** | ✔ | — |
| 16 | `doc remove-item` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 17 | `doc retitle-item` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 18 | `doc rename` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 19 | `doc set-field` | 5, 6 | 2 | 5 | 6 | **−2** | ✔ | ✔ |
| 20 | `doc set-slot` | 5, 6 | 2 | 5 | 6 | **−2** | ✔ | ✔ |
| 21 | `doc author` | 5 | 2 | 5 | 6 | **−2, −6** | ✔ | — |
| 22 | `doc show` | 2, 5, 6 | 2 | 5 | 6 | — | ✔ | ✔ |
| 23 | `doc schema` | 5 | 2 | 5 | 6 | **−2, −6** | ✔ | — |
| 24 | `doc list` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 25 | `task list` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 26 | `task diff` | 3, 5 | 2, 3 | 5 | — | **−2** | ✔ | ✔ |
| 27 | `task validate` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 28 | `task discard` | 2, 3, 5 | 2, 3 | 5 | — | — | ✔ | ✔ |
| 29 | `task finalize` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 30 | `task bind` | 5, 6 | 2 | 5 | 6 | **−2** | ✔ | ✔ |
| 31 | `config set` | 2, 5, 6 | 2 | 5 | 6 | — | ✔ | ✔ |
| 32 | `config insert-step` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 33 | `config replace-step` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 34 | `config remove-step` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 35 | `config fill` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 36 | `config fork` | 5 | 2 | 5 | — | **−2** | ✔ | — |
| 37 | `config get` | 2, 5, 6 | 2 | 5 | — | **+6** | ✔ | ✔ |
| 38 | `config list` | 5, 6 | 2 | 5 | — | **−2, +6** | ✔ | ✔ |
| 39 | `milestone create` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 40 | `milestone add-task` | 2, 3, 5, 6 | 2 | 5 | 6 | **+3** | ✔ | ✔ |
| 41 | `milestone add-from-spec` | 2, 5 | 2 | 5 | — | — | ✔ | ✔ |
| 42 | `milestone list-tasks` | 3, 5, 6 | 2, 3 | 5 | — | **−2, +6** | ✔ | ✔ |
| 43 | `milestone provision` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 44 | `milestone execute` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 45 | `milestone join` | 2, 3, 5, 6 | 2 | 5 | 6 | **+3** | ✔ | ✔ |
| 46 | `milestone finalize` | 2, 3, 5, 6 | 2, 3 | 5 | 6 | — | ✔ | ✔ |
| 47 | `milestone discard` | 2, 3, 5 | 2, 3 | 5 | — | — | ✔ | ✔ |

**`uncovered(<reason>)`: EMPTY.** Every one of the 47 leaves is the door of ≥1 driven row on axis 5; 25
of them on axis 2, 18 on axis 3 and 26 on axis 6.

**Leaves that lost coverage against any baseline: NONE at the union.** Every leaf reached by rc.18's axes
2/3, by rc.17's axis 5 or by M52's axis 6 is reached again here. **Per axis the picture is not flat, and
the losses are named rather than netted away:**

- **Axis 2: 47 → 25.** rc.18's axis 2 reached all 47 because it drove the **35 `Neither` leaves as
  silent controls** under an `uncommitted-pick` breach. This run's axis 2 drove **7** of those controls
  (`describe`, `validate`, `task list`, `doc list`, `config get`, `doc show`, `start` — CX-13) and spent
  its budget on the cwd axis instead: four cwds × both `aim_at` producers, ten `InProgress` members ×
  five doors, one member × all twelve acting doors, two spaced-path rigs. **The 22 leaves it no longer
  reaches are `upgrade` · `ingest` · `migrate` · `unmanage` · the seven `doc` write verbs · `doc schema` ·
  `task diff` · `task bind` · the six `config` step/fill/fork/list verbs · `milestone list-tasks`** —
  every one of them reached here by **axis 5** (in both 47-leaf sweeps and, for all but `task bind`, at a
  captured-key-set envelope arm), and eight of them additionally by axis 3 or 6. **No leaf is reached
  only as a silent control this run.**
- **Axis 3: 15 → 18** (+`validate`, +`milestone add-task`, +`milestone join`). No loss.
- **Axis 5: 47 → 47.** No loss; the two demotions removed no door (`(5, C1)`'s door is `start`, which the
  reconciliation itself drove, and §10's corrected count is a tally, not a row).
- **Axis 6: 24 → 26.** Gained `uninstall`, `validate`, `config get`, `config list`, `milestone list-tasks`
  (the cwd axis reached them). **Lost `doc add-item`, `doc author`, `doc schema`** — M52 reached those
  through composed-chain drives this run did not repeat; **all three are reached here by axis 5 at a
  declared envelope arm with its key set captured** (rows 18, 25 and 34, the last at
  `contract-version 7`).

**The four axes NOT re-driven — 1, 4, 7 and 8 — are not represented in this table at all.** A leaf's row
above says what axes 2, 3, 5 and 6 reached on rc.19. It says nothing about the caller-token,
transaction/rollback, freeze/migration or adopter-docs axes: those were last driven on `1.0.0-rc.16`
(M52's run), and their rows stand unchanged.

---

# HONEST BOUNDS — what each axis states it did **not** drive

Each axis file carries its own section ([axis-2.md](axis-2.md) §5 + §9 + reconciliation §5 ·
[axis-3.md](axis-3.md) §5 + §7.5 · [axis-5.md](axis-5.md) §9 + Notes · [axis-6.md](axis-6.md) §4 + §D);
this is the roll-up, not a replacement.

**The bound that covers the whole record: this is a PARTIAL run.** Axes **1 (caller tokens) · 4
(transaction / rollback) · 7 (freeze & migration) · 8 (adopter docs & help) were NOT re-driven**, by
design. Their M52 rows were not re-checked here. **Nothing in this record says they held; it says they
were not asked.**

**Axis 2 · posture**

1. **`InProgress::ALL` × the 12 acting doors as a full 10×12 cross.** Driven: 10 members × 5 doors (one
   fresh rig per member) and 1 member × all 12 doors. The un-driven cells are the 9 non-`bisect` members
   at the 7 doors of the second block; the reason given is that the member is decided by one shared
   `adjudicated_breach` composition and the door only filters on `PostureMember`.
2. **`InProgress::ALL` × the fan-out-worktree subject, member by member** — driven at `bisect` only
   (rc.18 drove all ten there). The arc changed the **render**, not the detection.
3. **Both `finalize.fan-out.squash` commit models** at the fan-out posture cell — the default model only
   (rc.18 drove both).
4. **The `GIT_DIR` redirect** · **`(2, DEFECT C)` at the milestone boundary's seam** · **a genuinely
   concurrent racer** on any rollback population · **the apostrophe/`#` repository root on this axis**
   (axis 6 drove it; axis 2 drove the space axis on two rigs).
5. **CX-7b · CX-17 · CX-19 · `migrate.source-untracked`'s span** — carried as OPEN LEADS above.

**Axis 3 · destroying doors**

1. **`Disposition::Narrate`** has no member at rc.19 — an empty arm, not a skipped one. **`task finalize
   --force`** does not exist (driven, exit 2).
2. **Hook-rejection cells at the two `Displace` doors** — axis 4's subject; the baseline's verdict is
   carried, not re-driven.
3. **`milestone finalize` with a rejecting hook under a spaced root** — axis 2/4's subject.
4. **`LeftoverVerdict::NoOwnLinkage` at `uninstall` and `milestone discard`** specifically — driven at
   `milestone provision` and **inferred** at the other two from the shared classifier, which is a source
   fact the driver did not re-derive.
5. **A genuine concurrent racer** at any destroying door · the **`GIT_DIR`** residual · **`F-5`'s
   reachability**, undischarged.
6. **Three instrument faults are recorded rather than dropped** (two wrong milestone ids under the
   edge-stopword slug rule; one cell refused because the rig had two active tasks; one `--ignored` cell
   built with the ignore rule committed after the pin). All re-driven; only the corrected drives count.

**Axis 5 · pinned contracts**

1. **3 of the 64 `ENVELOPE_ARMS` were not driven**, each with its reason: `doc show | CompoundFieldSlice`
   (**no shipped doctype declares a compound field** — verified across nine doctypes), `task bind | Bound`
   and `milestone add-from-spec | RecordOnlyAck` at their **success** arms (both driven at their reject
   arms). **61 / 61 driven arms: declared == driven.**
2. **`STORE_EXIT_FLIPS` at 5 of its 7 members** — driven at `reconciliation.rename` and
   `schema-conformance.home-vacated`, which is what this axis owes (key-set invariance under the flip);
   the other five need a manufactured pack or a down-stamped corpus — **axis 7's fixture work**.
3. **`setup` / `uninstall` excluded from the four-cwd divergence sweep** as destructive; both driven at
   their own cwd cells.
4. **The `Route` *kind*** is not observable in release posture — every route here is recorded by the
   **bytes it emitted** and, where it is a command, by **running it**; the kind is never asserted.
5. **`(5, C1)`'s empty-`next_steps` composition** was not re-manufactured **by the driver** — demoted for
   exactly that, then manufactured and driven by the reconciler.
6. **The §10 count correction stands as a bound**: the cwd-fix reviews' findings re-verified *by driving*
   are **6 / 11** (review 1) and **3 / 7** (review 2); the rest are transitive, doc-comment, fence-internal
   or record-count items with **no release-posture surface** — *not observable* is not *closed*.
7. **No `axis5-prompt.md` exists** beside that Codex pass, so what the source reviewer was asked could not
   be checked against what it answered. Stated, not worked around.

**Axis 6 · composed surfaces**

1. **C1 end-to-end to a landed commit for 8 of the 20 composed workflows.** Twelve workflows were composed
   and checked **mechanically** (every `jigc <token>` in 3026 composed lines is a real clap leaf — 30
   distinct forms, 25 leaves, 5 prose fragments), but their chains were **not** run to a commit.
2. **C4 at `describe` for the 34 narration bodies and 27 of the 31 command hints** — four hints driven
   against behaviour, all 22 off-catalog reasons driven for presence and partition, the rest not.
3. **A project *pack* listed in `packs.yaml`** · **`pack-resource-missing`** · **`migrate … --approve`**
   (axis 7's) · **`task diff`'s envelope and the `PATH_ARG_OCCURRENCES` bases** (axis 5's) ·
   **`ROLLBACK_POPULATIONS` / `TASK_AREA_FILES` / the `InProgress` family beyond `bisect`** (axes 2/3/4's).
4. **The non-UTF-8 root** — unbuildable here, carried as `L-1` with its three driven bounds.

**Bounds that bind all four axes.**

- **Release posture.** Every drive ran on the installed release binary, where the debug route, span and
  quoting fences do not exist. A fence violation is a **bad emitted command**, not a panic — which is how
  rc.18 found `(2, F-1)` and how this run found `A6-R1`. No arm here asserts a debug fence.
- **No genuine concurrent Task-tool spawn** was driven anywhere in this review. The racers are
  deterministic `git` shims and the fan-outs are single-process — the standing bound M51, M52 and all
  three M53 runs declare.
- **Exit codes.** Every exit code in every axis file was read **bare**. Three were first read through a
  pipe (`| head` returning head's `0`), caught, and re-measured; only the bare figures are recorded, and
  each instance is named in its file.
- **Fixtures.** `dev/jigc-rig` only, two-step eval (`rig=$(dev/jigc-rig <state> --binary …) || exit;
  eval "$rig"`), every root from `mktemp -d`, no teardown, and **`rm -rf` on a variable path appears
  nowhere in this review.** Nothing was fixed, committed or edited in the working repository.

---

# Files

| file | what |
|---|---|
| [axis-2.md](axis-2.md) | **posture** — the reconciled file, verbatim: the Opus driver's `(door, cell)` table (§1–§9, four cwds × 12 acting doors × 16 git states), the reconciliation ledger (§1, `CX-1`…`CX-21`), the four driver findings re-driven (`F-1`…`F-4`), the five demotions (§3) and the reconciled doors-covered list (§4) |
| [axis-3.md](axis-3.md) | **destroying doors** — the reconciled file, verbatim: **95 driven rows over 18 doors** with the demotion column (§1), the two new findings (§2), 22 driver repro blocks (§3, `R-1`…`R-22`), the baseline comparison (§4), bounds (§5), the reconciliation ledger (§7, `C-1`…`C-14` + the demotions) and four reconciler repro blocks (§8, `R-23`…`R-26`) |
| [axis-5.md](axis-5.md) | **pinned contracts** — the reconciled file, verbatim: the seven baseline rows re-driven (§1–§2), **61 of 64 `ENVELOPE_ARMS` driven arm by arm from a subdirectory** (§3), the cwd-arc cells (§4), `PATH_ARG_OCCURRENCES` × declared `base` (§5), the two 47-leaf hostile-cwd sweeps (§6), the **45 × 4** four-cwd divergence sweep (§7), counts (§10) and the reconciliation ledger with its two demotions |
| [axis-6.md](axis-6.md) | **composed surfaces** — the reconciled file, verbatim: the render-seam door derivation (§1), M52's four §A rows re-driven (§2), the **45-row (door, cell) table** over 8 doors × 6 cells including the new cwd cell (§3), the one new defect (§5), eight observations (§6), doors covered (§8) and the reconciliation ledger (10 Codex claims, 1 refuted, 1 open; §A–§F) |
| [codex/](codex/) | the four **unseeded Codex source passes**, verbatim, so every lead is auditable against the verdict it was driven to |
| [instrument/](instrument/) | the instrument — the four Codex prompts and the Workflow script that staffed and ran this third partial re-run |

*The four axis files are copied **verbatim** from the reconciled originals, with one added header line
naming the binary and the date. A handful of their internal links were written against the scratchpad and
point at repository paths; they are left as written rather than rewritten, because editing a reconciled
record's body would falsify the word “verbatim”.*

**What this record claims and nothing more:** four axes, driven on the installed `1.0.0-rc.19` on
2026-09-23, **zero tier-1 rows on every one of them**; `(2, F-1)` and `(3, F-3)` — rc.18's two tier-2 rows
— both **CLOSED**, and the cwd arc's own promises closed by **running** the bytes it emits, from five
working directories and two shell-hostile repository roots; **six new findings, none tier 1**, of which
**two sit inside the arc's own new code** and are one printed surface with two causes; **55 ledger
confirmations, 4 refutations with their data, 16 open leads**; and coverage of **47 of 47 `VERB_KINDS`
leaves, all 47 substantive, 33 of them beyond the pinned-envelope axis**. **The 1.0.0 call is the
human's**, and the exit rule's three clauses — satisfied, covered by tier, and fired by location — are
laid out at the headline for that call to be taken against.
