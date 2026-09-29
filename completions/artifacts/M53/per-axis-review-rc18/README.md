# M53 — the SECOND PARTIAL per-axis review (axes 2 · 3), on `1.0.0-rc.18`

**What this is.** M53's acceptance, re-run. [decisions-pending.md](../../../implementation/decisions-pending.md)
→ *The rc.17 fix pass (M53)* — the heading the charter carries, marked *“re-stamped **2026-09-23** as
`1.0.0-rc.18` after the post-review fix”* — fixes both the shape and the gate:

> **The exit rule — the human's own gate.** *[Completed 2026-09-22, on the first time it fired: a
> tier-1 row the partial re-review finds **outside** the fix pass's own new code is fixed as a
> **post-review fix under the same milestone** — fix, an independent review of the diff standing in for
> the audit, the record, a new stamp, the affected axes re-driven — never a new milestone and never a
> wave; the rule's original text named only the inside case.]* *The 1.0.0 call is taken when a partial
> re-review of the fix pass's affected axes finds no tier-1 row. Tier-2 and tier-3 findings never block
> the call and are triaged into the ledger for 1.x. A finding inside a fix pass's own new code triggers
> another fix pass and a partial re-review, never a full wave.*

The first partial re-review ran on `1.0.0-rc.17` and found **one tier-1 row**, `(2, DEFECT 1)`, outside
M53's own new code. That fired the completed clause above: a post-review fix (`3c71da87`), an
independent review of the diff (1 HIGH · 3 MEDIUM · 3 LOW, all seven fixed — `7329801b` · `a6711cee` ·
`de77b686` · `7a44d85d` · `7af8d6b4` · `5da4634a` · `16da362a`), the record, the second stamp, **and the
affected axes re-driven**. **This directory is that re-drive.**

**AXES 2 · 3 ONLY.** The post-review fix's blast radius is the posture family and the destroying doors'
second leg. Axis **5** (pinned contracts) was re-driven on rc.17 and is **not** re-driven here — the fix
moved no envelope key, no `contract-version`, no schema-hash — and axes **1, 4, 6, 7, 8** were not
re-driven on rc.17 either, **by design**. Nothing in this file says any of those five held; it says they
were not asked.

**The binary.** Every row in every file here was driven on the installed release
`/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.18`**, on **2026-09-23**, with the repository at HEAD
**`271b0cb7`** (*“chore(release): 1.0.0-rc.18 — the second M53 stamp, after the post-review fix and its
review's seven fixes”*). Each axis file asserts `jigc --version` **first, before anything else ran**, and
read its registry counts by symbol at that HEAD. The binary is the **release** build, so the debug-only
`Route::mechanical` argv fence does not exist in it: a route-fence violation shows up here as a **bad
emitted command**, never as a panic — which is the posture an adopter's binary is in, and is exactly how
this run's first new finding was found. Fixtures were built with
[`dev/jigc-rig`](../../../../dev/jigc-rig) (every root from `mktemp -d`, so nothing needed teardown and
the `rm -rf $V/$D` shape appears nowhere in this review) and by driving the binary. **Nothing was fixed,
committed or edited in the working repository by any agent in this review.**

**The baseline.** [../per-axis-review/](../per-axis-review/README.md) — the **first** partial re-run,
driven on `1.0.0-rc.17` at HEAD `7e98faf1` on 2026-09-22, over axes 2 · 3 · 5. It is a dated record and
**is not edited by this run**.

**An axis matrix row** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary. **A verb is covered iff it is the door of
≥1 driven row.** A classification-only row — a leaf proven by a ⇔ fence rather than by driving — confers
**no** coverage.

---

## THE HEADLINE — the tier-1 count

> # TIER-1 ROWS FOUND: **0**
>
> **`(2, DEFECT 1)` — the rc.17 tier-1 row, and the reason the post-review fix exists — is CLOSED.**
> Driven over **all ten** `InProgress::ALL` members induced inside a **provisioned sub-task worktree**,
> at **both** `finalize.fan-out.squash` commit models, at the door **and** at its `jigc task validate
> <sub-id>` preview, on the text arm and on the `--format json` arm.
>
> ```
> setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
>        jigc milestone create "Swallow milestone"
>        jigc milestone add-task swallow-milestone "swallow sub intent"
>        jigc milestone provision swallow-milestone
>        W=$REPO/.jigc/worktrees/swallow-sub-intent
>        (a commit authored in W, reset, and re-applied with `git cherry-pick -n` — the rc.17 fixture, verbatim)
> BEFORE: head -1 .git/worktrees/swallow-sub-intent/MERGE_MSG -> the users authored pick message
>         git -C $REPO status --porcelain -> (empty)        <- the MAIN checkout is clean
>
> argv : jigc task validate swallow-sub-intent          -> exit 3  repo.operation-in-progress
> argv : jigc milestone finalize swallow-milestone      -> exit 3  the SAME finding, byte-identical
>          at: .jigc/worktrees/swallow-sub-intent
>          route: conclude it with `git -C .jigc/worktrees/swallow-sub-intent commit` …
>
> AFTER: HEAD 5201c3e7 -> 5201c3e7 (UNMOVED) · MERGE_MSG "the users authored pick message" (INTACT)
>        the milestone record still open and finalizable
> ```
>
> On rc.17 that same fixture exited **0** and committed the worktree's index under jigc's subject,
> destroying the authored message. Here nothing durable is written, the refusal is placed **before** the
> record flip, and once the emitted route has been run the boundary lands cleanly — **including under
> `squash: false`, the commit model the rc.17 record twice recorded as un-driven** ([axis-2.md](axis-2.md)
> §4.4 · §4.5 · §4.6, re-established independently by the reconciler at §12 L-3).
>
> **Both axes reach tier-1 zero independently**, each with before-controls on its own drives: axis 2 over
> ≈210 `(door, cell)` drives, axis 3 over 75 rows plus ten fresh reconciliation drives.

**Five new findings, none tier 1: two tier-2 and three tier-3.** And the one thing on this page the human
should read next:

> ### One of the two tier-2 rows sits **INSIDE** the post-review fix's own new code.
>
> **`(2, F-1)`** — `crate::repo::aim_at` (`crates/cli/src/repo.rs:669`), **minted by `3c71da87`** and
> widened to its second caller by that fix's own review (`16da362a` / `7af8d6b4`). Both callers emit
> `git -C <repo-relative-path> …` as the remedy, and driven, that command exits **128** from every cwd
> that is not the repository root — including from inside a sibling fan-out worktree, the topology the
> fix's own review used to find its MEDIUM 1. It violates the sentence the fix itself wrote into
> `design/finalize.md:35`: *“a route the caller cannot run from where they are standing is not a route.”*
>
> **The other four new rows are outside M53's own new code**, each with its datum: `(2, F-2)`'s subject
> rule is M46 Increment 2 / T1 · `(3, F-3)`'s root resolution and evaded guards are M48's and M52's ·
> `(3, F-4)`'s narration is M46's *visible, not prevented* render · `(3, F-5)` satisfies D3's stated
> claim (the area **carries** its pin) and dead-ends on M52's `milestone.terminal`.
>
> **The adjudication is the human's, and the record does not pre-empt it.** The exit rule's first clause
> — *the call is taken when the partial re-review finds no tier-1 row* — is **satisfied**. Its second —
> *tier-2 and tier-3 findings never block the call* — covers `(2, F-1)` by tier. Its third — *a finding
> inside a fix pass's own new code triggers another fix pass* — does **not** say *tier-1 finding*, and
> `(2, F-1)` is a finding inside a fix pass's own new code. **Read by tier, the call is reachable; read
> by location, the third clause fires.** That tension is stated here rather than resolved by this record.

---

## The staffing

Per axis, three agents — M51 §19's shape, unchanged through M52 and both M53 runs. **Six agents over two
axes.**

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

It earned its keep three times this run, and in three different directions:

- **Axis 2 — a lead the driver had left open, closed by driving it.** M51's `codex-1` (the `relocate` /
  `config set placement-root` seam re-probe) was recorded *not re-raced* by the rc.17 run **and** by this
  run's driver, while the Codex pass closed it on a source read. Neither is a verdict under the rule. The
  reconciler raced the **shared** `move_doc` seam through `jigc rename` with a logging shim and drove it:
  the re-probe fires inside the window, `git mv` never runs, the staged pre-image is rolled back.
  **CLOSED, driven** — a closure neither pass alone could record ([axis-2.md](axis-2.md) §12 L-5).
- **Axis 3 — a completeness sentence refuted on half its scope.** Codex's removal-site census concluded
  *“no removal bypass that can cause exit-0 loss **or repository harm** through a destroying door.”*
  Driven, `jigc uninstall` from inside a fan-out worktree removes `.git/hooks/pre-commit` — one file
  shared by every checkout — at exit 0. The *loss of adopter bytes* half survives; the *repository harm*
  half does not ([axis-3.md](axis-3.md) §2.1 C-3a, §3 X-2).
- **Axis 3 — the demotion pass as an instrument finding. 22 of the driver's 75 rows (29 %) carried no
  repro block** while the driver's own §2 asserted every row had one. All 22 were re-driven; **all 22
  reproduced**, so the table's claims were sound — but as delivered, six of its fifteen doors rested on
  rows carrying no evidence, and the doors-covered list would not have survived the rule. Axis 2's
  demotion pass demoted six row-groups on the same ground; the reconciler re-drove five of them (§12
  L-9 · L-10 · L-11) and left the sixth — a doc-text claim — **unclaimed**, because a `grep` over prose is
  a source read, not a drive.

---

## Roll-up

| axis | subject | CONFIRMED | REFUTED | OPEN | new findings (tier) | doors covered (rc.17 →) |
|---|---|---|---|---|---|---|
| 2 | posture | 13 | 0 | 1 | **2 — 1 tier-2 · 1 tier-3** | **47** (47) |
| 3 | destroying doors | 14 | 1 | 2 | **3 — 1 tier-2 · 2 tier-3** | **15** (13) |
| **total** | | **27** | **1** | **3** | **5 — 0 tier-1 · 2 tier-2 · 3 tier-3** | **47 / 47 union · uncovered: none** |

**The counting basis, stated rather than smoothed.** The triples count **ledger entries** — every Codex
claim driven to a verdict, plus every driver defect re-driven — not findings. Axis 2's thirteen are its
eleven ledger confirmations (`L-1`…`L-11`) plus its two standing driver defects; its one OPEN is `L-12`,
a bundle of four *absence* claims. Axis 3's fourteen are nine Codex-claim confirmations (`C-1`, `C-2`,
`C-3b`, `C-4`…`C-8`, `C-10`) plus its five driver defects; its one REFUTED is `C-3a`'s repository-harm
half; its two OPEN are `C-9` (a completeness claim over a source enumeration, no argv) and `C-6`'s
structural half (a code fact the reconciler neither reproduced nor contradicted). **The auditable set is
the enumeration in §FINDINGS below, not this table**, and where a number here disagrees with a number in
an axis file, **the axis file governs**, because it carries the repro.

*Instrument note on the hand-off figures, on the rc.17 record's own precedent.* The orchestration that
assembled this record carried per-axis tallies of `confirmed 9 · refuted 0 · open 7` for axis 2 and
`confirmed 11 · refuted 1 · open 2` for axis 3. **Neither reproduces from its file's own ledger.** Axis
2's §12.3 enumerates eleven Codex-claim dispositions (eleven CONFIRMED, zero REFUTED, one OPEN bundle)
plus two standing driver defects — `13 · 0 · 1`. Axis 3's §2.1 + §2.2 enumerate ten Codex claims (nine
CONFIRMED, one REFUTED-in-half) plus five driver defects, with two open halves — `14 · 1 · 2`. The table
above is the file-derived count; the hand-off figures are recorded rather than reconciled away, because
**the file is the evidence and the hand-off figure is a summary of it**. The same discrepancy, on the
same axis 2, is on the rc.17 record.

**The three headline facts of the second partial re-run:**

1. **`(2, DEFECT 1)` is CLOSED**, driven over ten members × the new subject × both commit models × the
   door and its preview, and re-driven independently by the reconciler. **No previously-closed row
   re-opened, and no regression was found on either axis.**
2. **Zero tier-1 rows** — the exit rule's first clause is satisfied for the two axes the fix touched.
3. **Five new findings, and one of them is inside the fix's own new code** — `(2, F-1)`, tier 2. The other
   four are outside it, each on a stated datum.

---

# THE ROW-BY-ROW COMPARISON — every prior §A row on axes 2 · 3, all tiers

Two layers, because both axis files re-drove both: the **rc.17 run's** own §A rows (the immediate
baseline), and the **M52 §A** rows the rc.17 run tracked for these two axes. **CLOSED** carries the argv
that settles it; **STILL-OPEN** carries the datum. Rows M53 deliberately did not fix — the charter's
tier-2/3 triage — are marked **STILL-OPEN(1.x, expected)**. A row is keyed **(axis, id)** exactly as its
record keys it.

## Layer 1 — the rc.17 run's §A, restricted to axes 2 · 3 (3 rows: 1 tier-1 · 0 tier-2 · 2 tier-3)

| rc.17 §A row | tier | verdict on rc.18 | the argv / the datum |
|---|---|---|---|
| **`(2, DEFECT 1)`** — `jigc milestone finalize` commits a provisioned sub-task worktree's un-concluded git operation at exit 0, under jigc's own subject, and destroys that operation's authored commit message, while a door run *inside* that worktree refuses one command earlier | **1** | **CLOSED** | `jigc milestone finalize <id>` with each of the **ten** `InProgress::ALL` members induced inside `.jigc/worktrees/<sub-id>` by real git commands → **exit 3** at every member, `repo.operation-in-progress` at `BreachSite::FanOutWorktree`, the site clause **leading**, `at: .jigc/worktrees/<sub-id>`, the route aimed `git -C <worktree> …`. HEAD unmoved, `MERGE_MSG` intact, the record still finalizable. `jigc task validate <sub-id>` from the main checkout forecasts the identical finding at the identical exit on the identical arm. `--format json` at both sites: `key.target` = the worktree path. **Both commit models**: `squash: false` refuses and then **lands** `078a859 feat: the e3 subject` once `git -C .jigc/worktrees/e3-area bisect reset` has been run — discharging the rc.17 record's twice-declared un-driven bound. [axis-2.md](axis-2.md) §4.4 · §4.5 · §4.6 · §4.7, §12 L-3 |
| **`(3, F-1)`** — a **symlink** wearing a staged identity is jigc's own at the read and ack surfaces and a third party's at the same door's destroying probe | **3** | **STILL-OPEN(1.x, expected)** | Reproduces byte-for-byte. `ln -s $OUT/body.md .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md` → `jigc task discard tidy-the-readme` names it under `task-discard.foreign-bytes`; `jigc doc list --task tidy-the-readme` calls the same path `adr:via-symlink … managed`; `--force` drops *“staged edits to: adr:via-symlink”* — the entry the same command just called foreign. External target **1/1 intact**. Driven independently by the Codex pass's own proposed shape (`C-1`). [axis-3.md](axis-3.md) §3 X-1, rows 38 · 73 |
| **`(3, F-2)`** — the residual note asserts a shape it did not check | **3** | **STILL-OPEN(1.x, expected)** | `ln -s $OUT .jigc/tasks/ghost` → `jigc task discard ghost` → `finalize.no-task — … \`.jigc/tasks/ghost\` **is a directory** carrying no base pin …`. It is not a directory. Outside target intact; the control over a real directory makes the sentence true there. Driven independently as Codex `C-2`. [axis-3.md](axis-3.md) §3 X-1, row 42 |

## Layer 2 — M52's §A rows for these axes, as the rc.17 run tracked them (6 rows)

| M52 §A row | tier | verdict on rc.18 | the argv / the datum |
|---|---|---|---|
| **`(3, A3-1)`** — `jigc milestone finalize` removes `.jigc/milestones/<id>/` with `remove_dir_all` and destroys every byte jigc did not write there, at exit 0, named by nothing | **1** | **still CLOSED** | `jigc milestone finalize axis-three-probe --format json` over **8 planted loci** (area root · nested area dir · `merged/top.txt` · `merged/docs/{deep.txt, provenance.json, adr.md}` · `merged/sub/` · a sub-task area) → exit 0, **8 before / 8 after** by `command grep` with a before-control, all eight pairs on `committed.displaced`, each foreign directory moved **whole** as one pair, both area roots empty. [axis-3.md](axis-3.md) §3 X-5, rows 54–56 |
| **`(3, A3-2)`** — when the displacement fails, both `Displace` doors remove the area anyway; the bytes are permanently gone at exit 0 | **1** | **still CLOSED at both doors** | `task finalize` with `.jigc/displaced` a regular **file** → exit 0, area **left standing**, before=1 after=1, one advisory `finalize.foreign-bytes` naming `Not a directory (os error 20)`. At `milestone finalize`: **3/3** plants on disk, one advisory **per standing area**, envelope keys `['committed']`, `displaced: []`, 0 host-path hits. [axis-3.md](axis-3.md) §3 X-6, rows 47 · 48 · 57 |
| **`(2, DEFECT A)`** — a clean `git cherry-pick --no-commit` is a member of no `InProgress::ALL` row, and `jigc task finalize` concludes it at exit 0, destroying the picked commit's authored message | **1** | **still CLOSED** | All four `uncommitted-pick*` git states × **all 12** acting `BEHALF_DOORS` rows → exit 1, `repo.operation-in-progress — an uncommitted cherry-pick is in progress`. After the full sweep, HEAD unmoved and `.git/MERGE_MSG` + `.git/index` **md5-identical**. `--carry-staged` / `--force` / `--dry-run` / `--approve` all refuse first. [axis-2.md](axis-2.md) §4.3 · §4.11, §12 L-4 |
| **`(3, A3-3)`** — `milestone_boundary_displacement`'s subject is the sub-task areas only, so the boundary's own area is untested | **3** | **still CLOSED (behaviourally)** | The behaviour is driven green at the boundary's **own** area (X-5) and at the move-failure cell (X-6). The structural half — *production supplies both task and milestone `WorkArea` subjects to one teardown structure* — is a **source read the reconciler neither reproduced nor contradicted**, recorded as such (`C-6`), the same disposition the rc.17 run gave it. |
| **`(2, DEFECT C)`** — a posture breach raised at the **commit seam** prints no state-truth clause and no copy-runnable re-run, while every other in-transaction failure at the same door prints both | **2** | **STILL-OPEN(1.x, expected)** | Reproduces byte-for-byte with the same shim racer. The posture arm prints one code and one `route:` line **and nothing else**, while HEAD is unmoved, the task intact, the staged docs on disk and `A work.txt` still in the index — none of which the surface states. The contrast arm one step apart (a rejecting `pre-commit`) prints the full state-truth clause **and** the copy-runnable `jigc task finalize hook-probe`. Driven independently as Codex `L-2`. [axis-2.md](axis-2.md) §4.16 · §6.2 |
| **`(2, DEFECT B)`** — a conflicted `git merge --squash` is answered by `SquashMerge`, whose predicate is false of the state | **3** | **STILL-OPEN(1.x, expected)** | `git merge --squash cb` (conflicting) → `.git` holds `MERGE_MSG` + `SQUASH_MSG`, `git ls-files -u` → **3** unmerged. `jigc milestone create CS1` → exit 1, *“a squash merge is **staged and not committed**”*. The conclude arm is command-less here. The route (`git reset --merge`) is still effective: unmerged 3 → 0, the door then exits 0. Unchanged from rc.16 and rc.17. Driven independently as Codex `L-1`. [axis-2.md](axis-2.md) §6.1 |

## Layer 3 — the M51 rows these axes carry forward, and the rc.17 run's OPEN leads

Re-driven so a regression could not hide behind a partial run.

- **Axis 2** — `D1` (rebase/bisect answer the **operation**, not the detachment) **still CLOSED**, driven
  across all 16 git states at a commit door, with `rebase-merge` and `rebase-apply` both HEAD-detached and
  still answering the operation · `D2` (`git am` vs apply-backend rebase) **still CLOSED**, two nouns, two
  abort commands · `D3` **still CLOSED**, all four uncommitted-pick cells · `D3b` **not re-driven** (stated
  un-driven, not passing: M53's fix changed no commit-seam classification and §6.2's hook arm reproduces
  intact) · **`codex-1` — the rc.17 run's OPEN lead `(2, C-8)` — is now CLOSED, driven** by the
  reconciler's race of the shared `move_doc` seam (§12 L-5).
- **Axis 3** — `C-1` (the six-door registry complete), `D-1` (milestone ownership from live base-pinned
  areas), `D-2` (teardown tracks actual removal success), `D-4` (unreadable vs non-directory leftovers as
  distinct fail-closed shapes) **carried from the driver's rows, not re-driven by the reconciler** ·
  `D-3` (destroying-door paths render relative to `jigc_home`) **re-driven**: `command grep -cE
  '/var/folders|/private/var'` over stdout+stderr → **0**.
- **The rc.17 run's third OPEN lead**, `(3, C17)` (the per-site classification of the ~25-entry removal
  census), is **still OPEN** here as `C-9`, for the identical reason: a completeness claim over a source
  enumeration has no argv.

## What the post-review fix's own seven review findings did on rc.18

Each was driven as a cell rather than assumed ([axis-2.md](axis-2.md) §7).

| # | the review's finding | verdict on rc.18 |
|---|---|---|
| **HIGH** | `milestone discard` destroys a fan-out worktree's live git operation at exit 0 — the **clean-tree** cell, where `git status --porcelain` fires no guard | **CLOSED at all three refusing doors**, driven over a worktree asserted spotless (`git status --porcelain` → empty) carrying a live `rebase -i` at a `break` and a live bisect: `milestone discard` → exit 1 `milestone.dirty-worktree`, `uninstall` → exit 1 `uninstall.dirty-worktree`, `milestone provision` → exit 1 `milestone.leftover-holds-work`; worktree, admin record and `BISECT_LOG` all on disk after. Zero-false-fire control (clean **and** concluded) → exit 0 |
| **MEDIUM 1** | the new finding leaks an absolute host path into its message, its pinned key and its route when run from inside a linked worktree | **CLOSED** — **0** host-path hits across message · `at:` · `location.address` · `key.target` · route, run from inside a sibling worktree |
| **MEDIUM 2** | one code on two declared envelope arms, two streams, two exit codes; the new target form registered in neither contract home | **CLOSED** — door and preview both on `blocked` at exit 3; `Here` the declared singleton, `FanOutWorktree` the declared filesystem-path form |
| **MEDIUM 3** | *“nine of the ten `InProgress` members”* — a count with a reason naming a non-member, in six homes | **CLOSED, behavioural half only.** All ten members are handled in a worktree (§4.4, re-established at §12 L-3). **The doc-text half carries no repro block and is not claimed** — and a `grep` over prose is a source read, not a drive |
| **LOW 1** | the site clause lands after a predicate that already ends in a prepositional phrase | **CLOSED** — the clause **leads** at all ten members, including `UnmergedIndex` and `Sequencer` |
| **LOW 2** | the acceptance's refusing cells run against an attached worktree HEAD; production worktrees are detached | **CLOSED behaviourally**, re-established by the reconciler: `git -C <provisioned worktree> symbolic-ref -q HEAD` → **DETACHED**, and the operation cell refuses there. The suite-content half is not a drive and is not claimed |
| **LOW 3** | `overlay_worktree`'s one-worktree-per-repository bound is prose only | **CLOSED by proxy** — production reaches two simultaneous breaching worktrees and answers **one finding per worktree, two discriminating keys**. The fixture-internal assertion is test content, not driven here |

---

# FINDINGS

## A · CONFIRMED — every new finding, tiered on the charter's predicate

The predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.

### Tier 1 (0)

**None.** Every tier-1 candidate on both axes was driven to a safe outcome **with a before-control**:
`(2, DEFECT 1)` (HEAD unmoved, `MERGE_MSG` intact, nothing durable at any of ten members) · A3-1 (8
before / 8 after) · A3-2 (both areas left standing) · the destroying doors' operation leg (nothing
removed, marker and worktree on disk) · `(3, F-3)` (every deleted byte tracked or jigc-regenerable, the
untracked-byte cell guarded) · `(3, F-5)` (HEAD byte-unmoved through both arms).

### Tier 2 (2)

#### `(2, F-1)` · origin **driver**, re-driven by the reconciler at both producers · **INSIDE M53's own new code** · the aimed route `git -C <repo-relative-path>` does not run from the checkout that printed it

`crate::repo::aim_at` (`crates/cli/src/repo.rs:669`) is **minted by the post-review fix `3c71da87`** and
widened to its second caller by that fix's independent review (`16da362a` / `7af8d6b4`). Both callers are
affected: `BreachSite::aim` (`repo.rs:641`) and `crate::milestone::held_here` (`milestone.rs:3523`).

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc milestone create "Cwd wave"; jigc milestone add-task cwd-wave "cw area"
       jigc milestone provision cwd-wave
       W=$REPO/.jigc/worktrees/cw-area; git -C $W bisect start; git -C $W bisect bad
       mkdir -p $REPO/docs/deep; cd $REPO/docs/deep      <- an ORDINARY subdirectory of the main checkout

argv : jigc milestone discard cwd-wave
  -> emits:  abandon it with `git -C .jigc/worktrees/cw-area bisect reset`          [held_here]
argv : jigc milestone finalize cwd-wave
  -> emits:  route: … abandon it with `git -C .jigc/worktrees/cw-area bisect reset` [BreachSite::aim]

THE EMITTED COMMAND, RUN VERBATIM FROM THE CWD THAT PRINTED IT:
  git -C .jigc/worktrees/cw-area bisect reset
    fatal: cannot change to '.jigc/worktrees/cw-area': No such file or directory
    rc=128                                            <- both producers, identically

CONTROL, the same command from the repository root:
  cd $REPO; git -C .jigc/worktrees/cw-area bisect reset   -> HEAD is now at 7f12971 …   rc=0

SECOND CWD — from inside a SIBLING fan-out worktree, the topology the fix's own review used:
  jigc milestone finalize zeta-probe -> route: `git -C .jigc/worktrees/zed-two bisect reset`  -> rc=128
```

**Contract violated — the fix's own stated rationale**, `design/finalize.md:35`: *“…the refusal says
which worktree and routes at `git -C <worktree> …`, **because a route the caller cannot run from where
they are standing is not a route**.”* `design/surface-contract.md`'s printed-path fence admits exactly
this shape as a reason an absolute **stays** (*“pasteable shell bytes — a `route:`/remedy span the
operator pastes into a shell of unknown cwd”*), and that disposition is not taken here.

**Surfaces affected, driven:** `repo.operation-in-progress` at `BreachSite::FanOutWorktree` at
`jigc milestone finalize` and at `jigc task validate <sub-id>`; the per-path abandon command in
`milestone.dirty-worktree` (`milestone discard`), `uninstall.dirty-worktree` (`uninstall`) and
`milestone.leftover-holds-work` (`milestone provision`).

**Why tier 2 and not tier 1:** nothing is lost and nothing is committed — every one of these doors
*refused*. **Why not tier 3:** it is not a surface saying something the binary does not do; it is the
binary's own stated remedy being unrunnable as printed, which is what law 2 and the route floor exist to
prevent. **The honest counter-argument, carried rather than suppressed:** the relative path is correct
*relative to the repository root*, a reader who notices that can act on it, and a bare absolute would
re-open MEDIUM 1's key leak — so the fix has to keep the **message, the `at:` locus and the key**
repo-relative while making the **command span** resolvable. That is a design call, not a one-line swap.
[axis-2.md](axis-2.md) §5 F-1, §12 D-F1.

#### `(3, F-3)` · origin **driver**, re-driven from scratch by the reconciler · **outside M53's own new code** · `jigc uninstall` inside a fan-out worktree exits 0, reports an install it did not remove, and takes the repository's shared `pre-commit` hook

```
setup: dev/jigc-rig fresh; milestone create/add-task/provision; SUBWORK > $W/f.txt; git -C $W add f.txt
       WORKBENCH-KEEP > .jigc/milestones/axis-three-probe/plant.txt
BEFORE-CONTROL: main .jigc PRESENT · .git/hooks/pre-commit PRESENT · CLAUDE.md PRESENT ·
                SKILL.md PRESENT · workbench plant 1

$ (cd $REPO)                            jigc uninstall  -> exit 1
  blocking · uninstall.dirty-worktree — `.jigc/` holds 1 fan-out sub-task worktree path(s) …
$ (cd $REPO/.jigc/worktrees/first-sub)  jigc uninstall  -> exit 0
  jigc uninstall — repo-local install removed
    - removed .jigc/   - unwired bootstrap reference ← CLAUDE.md   - removed pre-commit hook   …

AFTER (measured in the MAIN checkout):
  main .jigc STILL PRESENT · CLAUDE.md STILL PRESENT · SKILL.md STILL PRESENT
  .git/hooks/pre-commit REMOVED           <- shared by every checkout of the repository
  jigc start -> "jigc — orientation"      milestone list-tasks -> "… tasks (1): first-sub"

CONTROL that keeps it off tier 1: an untracked byte under the WORKTREE's own .jigc/
  -> exit 1  uninstall.foreign-bytes  · both plants intact
```

Three clauses of the ack are false of what happened, at a **destroying** door (law 1, *nothing lies*).
The door resolved *its* `.jigc/` from the cwd's checkout while every other door in the same binary
resolved `jigc_home`. **Why not tier 1, driven rather than assumed:** the five `.jigc/` files it deletes
are tracked and the door says so, the hook is regenerable by `jigc setup`, the main workbench survives
(plant 1 before, 1 after), and the one cell that would make it loss — an untracked byte under the
worktree's own `.jigc/` — is **guarded**. What is destroyed at exit 0 with no consent asked is repository
state (`.git/hooks/`), which is why it is not merely tier 3. **Reachability:** a human standing in a
sub-task worktree — the adapter's deny floor carries `Bash(jigc uninstall:*)`, so an adapter agent cannot
reach it. **Inside M53's own new code? NO** — the door's root resolution is pre-existing and the guards
it evades are M48's and M52's; the post-review fix added the operation leg *inside* a guard that here
never runs. [axis-3.md](axis-3.md) §3 X-2, rows 35 · 36; driver §5 F-3.

### Tier 3 (3)

#### `(2, F-2)` · origin **driver**, re-driven · **outside M53's own new code** · `jigc milestone finalize` commits a **second repository's** linked worktree's staged bytes into this repository at exit 0 and acks them as the sub-task's own work

```
setup: rig committed-singletons;  B=$(mktemp -d …); git -C $B init -b main; two commits in B
       jigc milestone create "Foreign wave"; jigc milestone add-task foreign-wave "fw area"
       # NO `jigc milestone provision` — B's own worktree is parked at the sub-task path instead:
       git -C $B worktree add -q "$REPO/.jigc/worktrees/fw-area" -b fwbranch
       printf 'B-SECRET-PAYLOAD\n' > $W/bsecret.txt; git -C $W add bsecret.txt
       printf 'B-UNTRACKED-KEEP\n'  > $W/keepme.txt
BEFORE: git -C $W rev-parse --show-toplevel -> the squatted path itself
        git -C $W symbolic-ref -q HEAD      -> refs/heads/fwbranch   (B's branch, ATTACHED)

argv : jigc milestone join foreign-wave  -> exit 0
       jigc milestone finalize foreign-wave
  -> exit 0   finalized 3e2d426 — Finalize milestone foreign-wave (1 sub-task)
                added bsecret.txt … 2 files committed
                sub-tasks: fw-area: 1 code file      <- B's bytes, acked as the sub-task's own work
AFTER: git -C $REPO cat-file -p HEAD:bsecret.txt -> B-SECRET-PAYLOAD
       $W/keepme.txt intact · B's worktree registration intact · .jigc/displaced/ absent
```

**Why tier 3, measured rather than assumed:** **no byte is lost** — B's worktree, its index, its
untracked plant and its registration all survive, and the teardown does not reach an unregistered
worktree. What is wrong is the **ack**: a file belonging to another repository is named as this
sub-task's work, which is a law-1 claim the binary cannot support. The subject rule that admits the
worktree is **M46 Increment 2 / T1** (*“`provisioned_worktrees` takes the on-disk path instead of the
registered set”*), and the shipped policy already treats that path as jigc's territory. The posture
family behaved exactly as designed — it probed the worktree and found no breach, because there is none.
**Stated so a next reader does not re-tier it silently:** a human could argue tier 1 on *repository harm
through a committing door*; the driver and the reconciler do not, because the harm is a reversible
commit, nothing is destroyed, and reachability requires a foreign repository's worktree parked at exactly
`.jigc/worktrees/<sub-task-id>` with `provision` never run. **Un-driven sibling, named:** the variant
where the squatting worktree belongs to the *same* repository. [axis-2.md](axis-2.md) §5 F-2, §12 D-F2.

#### `(3, F-4)` · origin **driver**, re-driven · **outside M53's own new code** · the fan-out teardown narrates a **deleted tracked file** as *“never staged”* and *“the only copy … not recoverable”*

```
setup: TRACKED-BYTES-AT-HEAD > keeper.md; git add + commit; milestone create/add-task/provision
       SUBWORK > $W/subwork.txt; git -C $W add subwork.txt
       rm $W/keeper.md          # delete a TRACKED file; do not stage the deletion
BEFORE-CONTROL: git -C $W status --porcelain -> " D keeper.md" …
                git show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD

$ jigc milestone join … && jigc milestone finalize axis-three-probe   -> exit 0
  stdout  discarded with the fan-out worktrees (not committed, not recoverable):
            first-sub: build/out.o (never staged) · keeper.md (never staged) · scratch.txt (never staged)
  stderr  note: the fan-out worktree is the only copy of these bytes — they are not recoverable.

AFTER: git show HEAD:keeper.md -> TRACKED-BYTES-AT-HEAD   # the bytes ARE in git
       cat $REPO/keeper.md     -> TRACKED-BYTES-AT-HEAD   # and in the main checkout
```

The same run is **correct for its true siblings on the same line** (`build/ (ignored by git)`,
`scratch.txt (never staged)`), so the defect is the **deletion cell**, not the block. No byte is lost and
no exit code is wrong; what is spent is the credibility of the one warning standing between an operator
and a real `--force` deletion. The narration is M46's *visible, not prevented* render, untouched by M53
(settle-record §14 mints no producer here). [axis-3.md](axis-3.md) §3 X-3, row 62.

#### `(3, F-5)` · origin **driver**, re-driven · **outside M53's own new code** · a settled sub-task's leftover is *“1 active task(s)”* at one door and *“a leftover, not live work”* at another, and the finalize door's route names a command that refuses

```
setup: milestone with two sub-tasks, provisioned, both staging code;
       SAVE=$(mktemp -d …); cp -R .jigc/tasks/second-sub $SAVE/     # faithful backup, pin and all
       jigc milestone join && jigc milestone finalize axis-three-probe
       cp -R $SAVE/second-sub .jigc/tasks/second-sub                # restore, as a backup would
BEFORE: docs/milestone-records/axis-three-probe.md -> `status: joined`; HEAD 8ca9aff…

$ jigc task list                       -> exit 0  "1 active task(s)" / "second-sub [sub-task]"
$ jigc task discard second-sub         -> exit 1  milestone.terminal — … already `joined` …
$ jigc task discard second-sub --force -> exit 1  cmp -s of the two outputs -> BYTE-IDENTICAL
$ jigc task finalize second-sub        -> exit 3  finalize.empty-commit
    route: … abandon it with `jigc task discard second-sub --force`     <- refuses, above
… following that route verbatim (git add late.txt; re-run):
$ jigc task finalize second-sub        -> exit 3  finalize.render-io
    route: … abandoned with `jigc task discard second-sub --force` …    <- refuses, above
AFTER: HEAD 8ca9aff… (unmoved) · record still `status: joined` · nothing committed
```

Two halves, both driven: the enumerating door asks `carries_base_pin` and never asks the committed
record's terminal state; and the door that blocks names an exit that refuses, while the door that refuses
names the by-hand exit the first one does not. **Not claimed:** that `jigc task validate second-sub`
answering *validates clean* is a defect — it is **declared** (`finalize.empty-commit` and
`finalize.render-io` sit at `Tier::LaterCause`). **Reachability bound, stated rather than smoothed:** the
state was built by restoring a `cp -R` backup of the area after the boundary landed, by the driver **and
independently by the reconciler the same way**, so the bound is **not** discharged — no jigc sequence
either drove produces it. That is why it is tier 3 and not tier 2 despite the route half. **Inside M53's
own new code? NO, on the datum** — D3's residual rule governs a **pin-less** directory and this area
carries its pin, so D3's stated claim is satisfied here. [axis-3.md](axis-3.md) §3 X-4, rows 43 · 45 ·
52 · 53 · 65 · 66.

## B · REFUTED claims — each with its falsifying datum (1)

| # | claim | pass | falsifying datum |
|---|---|---|---|
| `(3, C-3a)` | *“I found **no removal bypass that can cause exit-0 loss or repository harm** through a destroying door”* — the **repository-harm** half, as worded | codex | `jigc uninstall` run from inside a fan-out worktree exits **0**, prints *“repo-local install removed / - removed .jigc/ / - removed pre-commit hook”*, and removes `.git/hooks/pre-commit` — **one file shared by every checkout** — while the main checkout's `.jigc/`, `CLAUDE.md` and `SKILL.md` all survive and `jigc start` still orients. `uninstall` is a `DESTROYING_DOORS` member. **Bound on the refutation, carried:** the *exit-0 loss of adopter bytes* half **stands** — the five deleted `.jigc/` files are tracked and the door says so, the hook is regenerable, and the untracked-byte cell is guarded. So this is a **tier-2** refutation of a blanket negative, not a tier-1 row. ([axis-3.md](axis-3.md) §3 X-2) |

**Nothing either driver claimed was refuted, and axis 2 refuted nothing at all.** The one refutation is
of a source-pass **completeness** sentence, and only of half its scope.

## C · OPEN leads — driven as far as the state allows, promoted by nothing (3)

| # | lead | pass | why it stays open |
|---|---|---|---|
| `(2, L-12)` | four *absence* claims: `DedicatedWorktree`'s discriminant is private and its sole constructor requires a linked-worktree `.git` plus canonical location plus registered ownership · the `setup` unborn exemption does not leak · no production HEAD-changing `git switch`/`checkout` exists · the post-rc.18 fix commits move no schema-hash, `schema-version`, `contract-version` or envelope key | codex | Each is a claim about **what the source does not contain**. A drive can exhibit an instance but cannot exhibit an absence. The observable halves that *are* driveable were driven and hold: the unborn exemption is visible as `setup` moving the repository from 0 to 1 commits while every other acting door refuses, and the typed worktree exemption is visible at L-11a/b. **No promotion on the source read, and nothing dropped.** |
| `(3, C-9)` | *“I inspected **every** production `remove_dir_all` / `remove_file` / `remove_dir` … I found **no unguarded** production removal of adopter bytes”* | codex | A completeness claim over a source enumeration has no argv. Nothing driven produced a counter-instance — the nearest, X-2, removes jigc's **own** install artefacts, not adopter bytes. Recorded open rather than promoted on the source read, per the rule. **This is the rc.17 run's `(3, C17)`, unchanged.** |
| `(3, C-6, structural half)` | *production supplies both task and milestone `WorkArea` subjects to **one** teardown structure; `MILESTONE_AREA_FILES` + the `merged/` tree rule represent milestone membership* | codex | The **behavioural** half is CONFIRMED (X-5, X-6 drive the milestone area's own complement at the boundary and at the move-failure cell). The claim that the two subjects share one structure is a **code fact** the reconciler neither reproduced nor contradicted. Same disposition the rc.17 run recorded for `(3, A3-3)`. |

## D · Observations driven and carried — **not** defects

Recorded so a next reader does not re-drive them as findings. Each is graded against a written
declaration, quoted in its axis file.

- **`GIT_DIR`** — a door run in repo A with `GIT_DIR` pointing at B concludes B's cherry-pick at exit 0
  (A's HEAD unmoved, B's `MERGE_MSG` gone). This **matches the posture family's declared bound**
  (`repo.rs`'s module header: *“its subject is the path it is handed, and it passes the ambient
  environment through untouched”*) with a written reopening condition. Recorded as an amplifier on the
  declared bound, the same disposition M52 and M53 gave it. ([axis-2.md](axis-2.md) §6.4)
- **`finalize.render-io`'s host-absolute `at:` path**, hit incidentally at axis 2's §4.6 and appearing
  verbatim in axis 3's X-4. **Not reported as a finding:** `crates/engine/src/finalize.rs` is a declared
  row of `crates/cli/tests/repo_relative_paths.rs`'s `UNSWEPT_PRODUCERS` — a counted, fenced, declared
  remainder. Graded against, not re-found.
- **Axis 3's `OBS-1 … OBS-9`** are carried unchanged from the driver and **not re-driven** by the
  reconciler except `OBS-8` (the host path above). Each is declared somewhere and graded against.
- **`milestone finalize`'s stderr-only advisory · the 21-render `DEBUG_REMAINDER` · the residual cleared
  by hand · `reseed_sub_task_areas`' `.exists()` skip** — M53's own **declared bounds**. Both axes graded
  against them rather than re-finding them.

---

# COVERAGE — the two axes against the 47 `VERB_KINDS` leaves

**The leaf count, read rather than quoted.** `crates/cli/src/cli.rs:1830`'s `VERB_KINDS` carries **47**
leaf rows at HEAD `271b0cb7` — **35 `VerbKind::Write` · 12 `VerbKind::Read`** — counted by reading the
`(&[…], VerbKind::…)` rows of the const itself (`awk 'NR>=1830 && NR<=1882' crates/cli/src/cli.rs |
command grep -cE '^\s+\(&\[' → 47`). The spellings and their order are identical to the rc.17, M52 and
M51 tables, so the columns below are directly comparable row by row.

**What the table reports.** Which of the **two re-driven axes** reached each leaf as the door of ≥1
**driven** row — the union of the two files' *Doors covered* sections — against the same column from the
**rc.17 run restricted to axes 2 / 3**. `uncovered` is the set of leaves no re-driven axis reached.

> **`uncovered`: NONE — and, as on rc.17, this is the run's one result that contradicts its own brief.**
> The assembly brief stated that a partial run's `uncovered` set *will* be non-empty and that 47/47 must
> not be claimed. Driven, it is empty, for a reason that is a property of **axis 2** and not a grading
> choice: **axis 2 drove all 35 `BEHALF_DOORS ▸ Neither` leaves as controls** under an
> `uncommitted-pick` breach (34 silent with `repo.*` hits 0, and `task validate`, which answers by the
> stated preview design), on top of its 12 acting doors. One of the two re-driven axes therefore reaches
> all 47 on its own. **The number is reported as measured, and the stricter reading the brief was
> reaching for is reported beside it** — and that stricter number is **much** weaker this run than on
> rc.17, because axis 5's two uniform 47-leaf sweeps are not in this run at all.

**The stricter column.** A leaf is **substantive** in this run iff some axis drove it in a
**condition-specific** row — a posture cell at an acting door, a destroying-door cell, a residual or
enumerating cell, or a success/condition arm — rather than **only** as a silent `Neither` control.
**24 of 47 are substantive; 23 are control-only.** (rc.17's figure was 43 of 47, and the whole of the
difference is axis 5's absence.)

| # | leaf | M53 rc.18 axes (driven) | rc.17 axes, restricted to 2/3 | Δ | substantive |
|---|---|---|---|---|---|
| 1 | `start` | 2, 3 | 2, 3 | — | ✔ |
| 2 | `workflow` | 2 | 2 | — | ✔ |
| 3 | `setup` | 2 | 2 | — | ✔ |
| 4 | `uninstall` | 2, 3 | 2, 3 | — | ✔ |
| 5 | `upgrade` | 2 | 2 | — | control-only |
| 6 | `ingest` | 2 | 2 | — | control-only |
| 7 | `migrate` | 2 | 2 | — | control-only |
| 8 | `migrate-corpus` | 2 | 2 | — | ✔ |
| 9 | `unmanage` | 2 | 2 | — | control-only |
| 10 | `rename` | 2, 3 | 2, 3 | — | ✔ |
| 11 | `relocate` | 2 | 2 | — | ✔ |
| 12 | `describe` | 2 | 2 | — | control-only |
| 13 | `validate` | 2 | 2 | — | control-only |
| 14 | `doc create` | 2 | 2 | — | control-only |
| 15 | `doc add-item` | 2 | 2 | — | control-only |
| 16 | `doc remove-item` | 2 | 2 | — | control-only |
| 17 | `doc retitle-item` | 2 | 2 | — | control-only |
| 18 | `doc rename` | 2 | 2 | — | control-only |
| 19 | `doc set-field` | 2 | 2 | — | control-only |
| 20 | `doc set-slot` | 2 | 2 | — | control-only |
| 21 | `doc author` | 2 | 2 | — | control-only |
| 22 | `doc show` | 2 | 2 | — | control-only |
| 23 | `doc schema` | 2 | 2 | — | control-only |
| 24 | `doc list` | 2, 3 | 2, 3 | — | ✔ |
| 25 | `task list` | 2, 3 | 2, 3 | — | ✔ |
| 26 | `task diff` | 2, **3** | 2 | **+3** | ✔ |
| 27 | `task validate` | 2, **3** | 2 | **+3** | ✔ |
| 28 | `task discard` | 2, 3 | 2, 3 | — | ✔ |
| 29 | `task finalize` | 2, 3 | 2, 3 | — | ✔ |
| 30 | `task bind` | 2 | 2 | — | control-only |
| 31 | `config set` | 2 | 2 | — | ✔ |
| 32 | `config insert-step` | 2 | 2 | — | control-only |
| 33 | `config replace-step` | 2 | 2 | — | control-only |
| 34 | `config remove-step` | 2 | 2 | — | control-only |
| 35 | `config fill` | 2 | 2 | — | control-only |
| 36 | `config fork` | 2 | 2 | — | control-only |
| 37 | `config get` | 2 | 2 | — | ✔ |
| 38 | `config list` | 2 | 2 | — | control-only |
| 39 | `milestone create` | 2, 3 | 2, 3 | — | ✔ |
| 40 | `milestone add-task` | 2 | 2 | — | ✔ |
| 41 | `milestone add-from-spec` | 2 | 2 | — | ✔ |
| 42 | `milestone list-tasks` | 2, 3 | 2, 3 | — | ✔ |
| 43 | `milestone provision` | 2, 3 | 2, 3 | — | ✔ |
| 44 | `milestone execute` | 2, 3 | 2, 3 | — | ✔ |
| 45 | `milestone join` | 2 | 2 | — | ✔ |
| 46 | `milestone finalize` | 2, 3 | 2, 3 | — | ✔ |
| 47 | `milestone discard` | 2, 3 | 2, 3 | — | ✔ |

**`uncovered(<reason>)`: EMPTY.** Every one of the 47 leaves is the door of ≥1 driven row on axis 2, and
15 of them on axis 3 as well.

**Leaves that lost coverage against the rc.17 table restricted to axes 2/3: NONE.** Every leaf the rc.17
axes 2 and 3 reached is reached again here, by the same axis or axes. (`milestone add-task` — the one
leaf that lost an axis between M52 and rc.17 — is unchanged here: axis 2 only, and substantive on axis
2's twelve acting-door posture cells.)

**Leaves newly reached (2), both by axis 3, whose door set grew 13 → 15:** `task validate` (row 66 — a
settled sub-task's leftover answering *validates clean*, driven inside X-4) and `task diff` (row 67 —
`task diff` over that same leftover, driven inside X-10). Both are rows the driver had marked driven
**without** a repro block; both are covered because the **reconciler re-drove them**, which is the
demotion pass adding coverage rather than removing it.

**Per-axis door counts as each file states them:** axis 2 **47 / 47** (12 acting + 35 `Neither`
controls) · axis 3 **15** (the six `DESTROYING_DOORS` members + nine residual/enumerating doors). Axis
3's own coverage section additionally names **25** leaves that were **rig setup only** on that axis and
are therefore the door of no reviewed row there — recorded so its list is not read as coverage it does
not have.

**The five axes NOT re-driven — 1, 4, 6, 7, 8 — and axis 5 — are not represented in this table at all.**
A leaf's row above says what axes 2 and 3 reached on rc.18. It says nothing about the caller-token,
rollback, composed-surface, freeze/migration, adopter-docs or pinned-contract axes: axis 5 was last
driven on `1.0.0-rc.17`, the other five on `1.0.0-rc.16`, and their rows stand unchanged.

---

# HONEST BOUNDS — what each axis states it did **not** drive

Each axis file carries its own section ([axis-2.md](axis-2.md) §8 · [axis-3.md](axis-3.md) §5); this is
the roll-up, not a replacement.

**The bound that covers the whole record: this is a PARTIAL run, and narrower than the last one.** Axes
**1 (caller tokens) · 4 (transaction / rollback) · 5 (pinned contracts) · 6 (composed surfaces) · 7
(freeze & migration) · 8 (adopter docs & help) were NOT re-driven**, by design — the post-review fix's
blast radius is posture and the destroying doors' second leg. Axis 5's rc.17 rows and the other five
axes' M52 rows were not re-checked here. **Nothing in this record says they held; it says they were not
asked.**

**Axis 2 · posture**

1. **A genuine concurrent process** rather than a deterministic shim racer at any seam — the standing
   headless bound M51, M52 and M53 all declare. The commit-seam racer is a shim whose fire is verified by
   a stamp.
2. **A race positioned *inside* the boundary's own apply window** — an operation opened in a provisioned
   worktree **after** `fan_out_posture_findings` runs and **before** `git diff --cached --binary` /
   `git apply --cached`. The window exists and is ~3 git calls wide (the trace was captured); **whether
   the boundary re-probes it is not established here. Recorded as un-driven, not as passing.**
3. **`D3b`** (a git refusal not dressed as a hook rejection) — not re-driven; M53's fix changed no
   commit-seam classification and the hook arm reproduces intact.
4. **`SeamSubject::verify`'s identity-drift and expected-ref-drift legs** — only the posture leg is this
   axis's.
5. **`GIT_DIR` at 11 of the 12 acting doors** — driven at `milestone create` only; the family's
   declared-out bound with a written reopening condition.
6. **Non-`main` default branch names, submodule worktrees, `core.worktree` redirects, and git versions
   other than the installed `/usr/bin/git`** — the family's existing declared git-2.54.0 deferral.
7. **The `rebase-apply`/`Rebase` inducer inside a worktree maps imprecisely** — `git am` lands the `Am`
   member; the distinction **is** driven in the main checkout, where the rig's own builder separates them.
8. **`(2, F-2)`'s same-repository variant** (a user's own `git worktree add .jigc/worktrees/<id>`) — not
   driven; the foreign-repository cell is the harder one and is the one recorded.
9. **MEDIUM 3's doc-text half** — the claim *“the numeral is struck in six homes”* carries no repro block
   and is **not claimed**; a `grep` over prose is a source read, not a drive. Likewise **LOW 2**'s and
   **LOW 3**'s suite-content halves.

**Axis 3 · destroying doors**

1. **What the reconciler did not re-drive**, stated as un-driven rather than passing: the driver's `R-2`
   (`provision` over a second repository's linked worktree), `R-3` (the render-root / declared-target-form
   cells from inside a sibling worktree), `R-10`'s nine `LeftoverShape` × `--force` cells, `R-11`–`R-15`,
   and the M51 rows `C-1` / `D-1` / `D-2` / `D-4`. Each rests on the driver's own repro block; none is
   contradicted by the source pass.
2. **Three of the ten `InProgress` members** at the refusing doors.
3. **`(3, F-5)`'s reachability bound is not discharged** — the state was built by restoring a `cp -R`
   backup, by the driver **and** by the reconciler the same way. No jigc sequence either drove produces
   it.
4. **`(3, F-3)`'s `--force` arm** — un-driven.
5. **`Disposition::Narrate`** is empty by construction at rc.18 — an empty arm, not a skipped one.
   Likewise `task finalize` / `milestone finalize` × `--force`: the flag does not exist at either leaf.
6. **A genuine concurrent racer** against a displacement or teardown — headless by construction.
7. **Two cell variances are on the record rather than smoothed** (row 17 driven over `Bisect` + tree dirt
   rather than `Rebase` on a spotless tree; row 61's `.gitignore` committed before the pin, so two of the
   driver's three narrated items appear), and **one reconciler drive failed on its own error and is
   recorded rather than dropped** (row 61's first attempt → `finalize.base-mismatch`, exit 3).

**Instrument honesty, both axes.**

- **One exit code was read through a pipe** on axis 2 (`jigc task validate … | head -2` reported the
  *head's* `0`). Caught, re-measured **bare** (exit 1), and only the bare figure is recorded.
- **Two false-positive control cells were initially built wrong** on axis 2 (a milestone id derived with
  `ls … | head -1`, and in two rigs the worktree was never built, so the `exit 0` was a docs-only boundary
  rather than a control). Both rebuilt with a fixed id and the worktree asserted; only the rebuilt drives
  are recorded.
- **Nine argvs in axis 2's control sweep first exited 2** (clap usage — not a driven row, since dispatch
  never ran). Each was corrected against its own `--help` and re-driven; only the corrected drives count.
- **The rc.17 axis-2 instrument failure was explicitly designed against** — that run parsed a rig's stdout
  instead of eval-ing it and ran three `git` commands in the working repository. Here every rig is
  eval-ed in two steps and both roots are asserted to exist before any `git -C` runs. `git status` in the
  working repository was the same at the end of this review as at the start.
- **The demotion pass is this run's instrument finding**, on both axes: 22 of axis 3's 75 rows and six of
  axis 2's row-groups were presented as driven while carrying no repro block. All of axis 3's 22 and five
  of axis 2's six were re-driven and reproduced; the sixth — a doc-text claim — is unclaimed.

**Bounds that bind both axes.** The **release posture** means a route-fence violation shows up as a bad
emitted command rather than a panic — every drive here is under that posture, no arm asserts the debug
fences, and `(2, F-1)` is exactly what that posture makes visible. **No genuine concurrent Task-tool
spawn** was driven anywhere in this review; the racers are deterministic shims and the fan-outs are
single-process, which is the standing bound M51, M52 and M53 all declare.

---

# Files

| file | what |
|---|---|
| [axis-2.md](axis-2.md) | **posture** — the reconciled file, verbatim: the Opus driver's `(door, cell)` table (§1–§11, ≈210 drives), the reconciliation ledger (§12, `L-1`…`L-12` + `D-F1`/`D-F2`) and the reconciled doors-covered list (§13) |
| [axis-3.md](axis-3.md) | **destroying doors** — the reconciled file, verbatim: the driver's 75 rows over 15 doors with the demotion column (§1), the reconciliation ledger (§2, `C-1`…`C-10` + the five driver defects + the demotion audit), ten reconciliation repro blocks (§3, `X-1`…`X-10`), doors covered (§4) and bounds (§5) |
| [codex/axis-2-source-pass.md](codex/axis-2-source-pass.md) · [codex/axis-3-source-pass.md](codex/axis-3-source-pass.md) | the two Codex source passes, **verbatim**, so every lead is auditable against the verdict it was driven to |
| [instrument/](instrument/) | the instrument — the two Codex prompts and the Workflow script that staffed and ran this second partial re-run |

**What this record claims and nothing more:** two axes, driven on the installed `1.0.0-rc.18` on
2026-09-23, **zero tier-1 rows**, `(2, DEFECT 1)` closed over ten members × the new subject × both commit
models, five new findings (two tier-2, three tier-3) of which **one sits inside the post-review fix's own
new code**, and coverage of 47 of 47 `VERB_KINDS` leaves with 24 of them substantive. **The 1.0.0 call is
the human's**, and the exit rule's three clauses — satisfied, covered, and arguably fired — are laid out
at the headline for that call to be taken against.
