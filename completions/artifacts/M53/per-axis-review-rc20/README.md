# M53 — the FOURTH PARTIAL per-axis review (axes 2 · 3 · 5), on `1.0.0-rc.20`

**What this is.** M53's acceptance, re-run a **fourth** time — after the **pre-v1 usability batch** and
**F-10 `jigc task amend`**. [decisions-pending.md](../../../implementation/decisions-pending.md) →
*The rc.17 fix pass (M53)* carries the gate this record is written against:

> **The exit rule — the human's own gate.** *The 1.0.0 call is taken when a partial re-review of the fix
> pass's affected axes finds no tier-1 row. Tier-2 and tier-3 findings never block the call and are
> triaged into the ledger for 1.x. A finding inside a fix pass's own new code triggers another fix pass
> and a partial re-review, never a full wave.* *[Completed 2026-09-22, on the first time it fired: a
> tier-1 row found **outside** the fix pass's own new code is fixed as a **post-review fix under the same
> milestone** — fix, an independent review of the diff standing in for the audit, the record, a new
> stamp, the affected axes re-driven.]*

The first partial re-review ran on `1.0.0-rc.17`, the second on `rc.18`, the third on `rc.19` after the
**cwd-dependence arc** ([per-axis-review-rc19/](../per-axis-review-rc19/README.md) — **this run's
baseline**, and **not edited by it**). That third run found **zero tier-1 rows** and six new findings, two
of them inside the arc's own new code and both on the installed pre-commit hook. What has happened since
is on the record at [VERDICT.md](../VERDICT.md) → **Addendum 3**: the **pre-v1 usability batch**
(`be40738e`…`b6b1a18f`, six surface rows — two of which set out to close rc.19's `(2, N-1)` and
`(2, N-2)`) and **one new capability, F-10 `jigc task amend`** (`407ebf08`…`bb252c25`), whose design of
record is [f10-amend-settle.md](../f10-amend-settle.md), whose driven baseline is
[f10-amend-baseline.md](../f10-amend-baseline.md), and whose independent review
([audit/f10-code-review.md](../audit/f10-code-review.md)) raised **1 HIGH · 3 MEDIUM · 4 LOW**, all
declared fixed. **This directory is the re-drive of that range.**

**AXES 2 · 3 · 5.** The range's blast radius picked the axis set: **2** (posture — a new committing arm
with its own refusal family), **3** (destroying doors — a new `TASK_AREA_FILES` member, a new mint door,
a fourth `Displace` subject), **5** (pinned contracts — two new `ENVELOPE_ARMS` rows, an eleventh
`COMMITTING_DOORS` row, a twelfth error code and a third `AMBUSH_CONTRACTS` disposition).
**Axes 1, 4, 6, 7 and 8 were NOT re-driven, by design** — caller tokens, transaction/rollback, composed
surfaces, freeze/migration and adopter docs/help. Axis 6's rows stand where rc.19 left them; axes 1, 4, 7
and 8's stand where **M52's** run left them, on `1.0.0-rc.16`. **Nothing in this file says those five
held; it says they were not asked.**

**The binary.** Every row in every file here was driven on the installed release
`/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.20`**, on **2026-09-27**, with the repository at HEAD
**`4d3175c3`** (*“chore(release): 1.0.0-rc.20 — the fourth M53 stamp, after the usability batch and
F-10's review fixes”*). Each of the six agents asserted `jigc --version` **first, before anything else
ran**, and read its registry counts **by symbol** at that HEAD. The binary is the **release** build, so
the debug-only fences (`Route::mechanical`'s argv fence, `unaimed_git_span`, `unbased_migrate_span`, the
two quoting fences) **do not exist in it**: a fence violation shows up here as a **bad emitted command**,
never as a panic. That is the posture an adopter's binary is in, and it is the only posture in which the
central question — *does the emitted route run?* — can be asked at all. Fixtures came from
[`dev/jigc-rig`](../../../dev/jigc-rig) (every root from `mktemp -d`, so nothing needed teardown and the
`rm -rf $V/$D` shape appears nowhere in this review) and from driving the binary; the non-rig fixtures a
rig cannot express are named at their cells (a rejecting hook behind `core.hooksPath`, a deterministic
`git` shim on `PATH`, a second repository, a repository root containing a space, a mutated throwaway
pack). **One exception to *nothing was written to the working repository* is on the record rather than
hidden — see HONEST BOUNDS → *The instrument's own fault*.**

**The registry deltas the range produced, read by symbol at `4d3175c3` and agreed by all three axes:**

| registry | rc.19 | rc.20 | what moved |
|---|---|---|---|
| `VERB_KINDS` / `BEHALF_DOORS` | 47 / 47 | **48 / 48** | `task amend`, a `Write` leaf classified `Neither` |
| `ENVELOPE_ARMS` | 64 | **66** | `task amend \| Composed` · `task finalize \| LandedAmend` |
| `COMMITTING_DOORS` | 10 | **11** | `jigc task finalize (amend)` |
| `ERROR_CODE_REGISTRY` | 11 | **12** | `ERROR_AMEND_REJECTED` = `finalize.amend-rejected` |
| `TASK_AREA_FILES` | 14 | **15** | `AMEND_PIN_FILE` (`amend`) |
| `MINT_DOORS` | 5 | **6** | `jigc task amend ["<intent>"]` |
| `AMBUSH_CONTRACTS` | 4 | **6** over **3** dispositions | the new `DeclaredWhereReachable`, both rows F-10's |
| `CONSTRAINT_REQUIRED_TOKENS` | 6 | **8** | the two amend rows |
| goldens | 634 | **646** | the one composed member the range minted |
| **unmoved** | | | `STORE_EXIT_FLIPS` 7 · `DESTROYING_DOORS` 6 · `WORK_UNIT_ID_DOORS` 25 · `SLUG_DOORS` 6 · `PATH_ARG_OCCURRENCES` 14 · `ROLLBACK_POPULATIONS` 11 · `doc schema` `contract-version` 7 · `doc show` `schema-version` 1 · both schema manifests byte-identical over `609da011..4d3175c3` |

**An axis matrix row** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary. **A verb is covered iff it is the door of
≥1 driven row.** A classification-only row — a leaf proven by a ⇔ fence rather than by driving — confers
**no** coverage.

---

## THE HEADLINE — the tier-1 count

> # TIER-1 ROWS FOUND: **0**
>
> Tier 1, quoted from the charter: **exit-0 loss or repository harm through a committing, destroying or
> moving door.** **No driven row on any of the three axes reached it**, and each axis reached that zero
> independently, with before-controls on its own drives:
>
> - **axis 2** — the new committing arm refused at exit 1 under **all sixteen** buildable git states with
>   `HEAD` **byte-identical** each time; refused at **exit 3** over a non-empty index across
>   add / modify / delete / rename with HEAD *and* tree unmoved; refused at **all eight** `doc` write
>   leaves before any managed doc was staged (`git status --porcelain` **empty** in every cell); refused
>   on a moved HEAD; and under a rejecting `pre-commit` **and** `commit-msg` hook left `HEAD`, its message
>   and its tree byte-identical with the task still live. On the happy path the tree and the parent were
>   **byte-identical** and the author date preserved.
> - **axis 3** — **115 driven rows over 28 doors**: no plant died at exit 0 anywhere but where `--force`
>   consented and the door said so first. `command grep` before-controls on every loss/survival claim.
>   The amend arm displaces repo-relative, leaves the area standing when the park fails, and leaves the
>   plant in the area on a hook-rejected amend.
> - **axis 5** — **~500 driven rows**; every arm reporting a landed commit checked against
>   `git rev-parse HEAD`, `HEAD^{tree}` and `HEAD^`; **65 / 65** driven `ENVELOPE_ARMS` arms
>   declared == driven; **96 / 96** hostile-cwd cells in a declared arm; **0** undeclared envelope shapes.
>
> **Both rc.19 findings the usability batch set out to close are CLOSED, driven, by two axes
> independently.** `(2, N-1)` — the pre-commit hook announcing an out-of-band rename on a commit that
> contained none — prints **nothing** now on a committed deletion. `(2, N-2)` — the *blocking* backstop
> structurally unreachable for the entire placement family — blocks at **4 / 4** singletons.
>
> ```
> N-1  rig committed-singletons
>      BEFORE-CONTROL: printf 'ordinary\n' > ord.txt; git add; git commit   -> rc=0, `^jigc:` lines 0
>      git rm -q VISION.md ; git status --porcelain -> "D  VISION.md"   (a DELETION, no rename)
>      git commit -m "probe: delete a managed singleton"                -> rc=0, `^jigc:` lines 0
>      (on rc.19: "jigc: an out-of-band managed-doc rename exists in the committed tree … (not staged
>       in this commit; commit not blocked)." — on every commit from there on)
>
> N-2  four fresh rigs, one per placement singleton:  git mv <A> <B> ; git commit    (rc read bare)
>      docs/roadmap.md -> docs/roadmap-oob.md      | staged R | rc=1  BLOCKED
>      VISION.md       -> VISION-OOB.md            | staged R | rc=1  BLOCKED
>      CHANGELOG.md    -> CHANGELOG-OOB.md         | staged R | rc=1  BLOCKED
>      docs/decisions-log.md -> docs/dl-oob.md     | staged R | rc=1  BLOCKED
>      each: "jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses
>             jigc identity tracking; use `jigc rename` instead (commit blocked)."
>      CONTROL, a LOCATION doctype (milestone-record):  rc=1, byte-identical line
>      (on rc.19 all four placement rows were rc=0 with the "not staged in this commit" sentence)
> ```
>
> **And rc.19's one tier-2/3 row, `(3, F-A)`, is CLOSED**: `jigc task finalize`'s Displace surfaces from a
> branch-attached linked worktree are **0 host-absolute hits** on stdout **and** stderr, on the all-move
> cell *and* the blocked-destination cell, including on the **1.0-pinned** `committed.displaced[].from` /
> `.to` keys and on `finalize.foreign-bytes`'s `message` and `route`. Driven by the axis-3 driver, again
> by its reconciler on Codex's own proposed argv, and a third time by axis 5 from a different linked
> worktree.

**Seven new finding rows — five distinct defects, because two were found twice by different axes. One is
tier 2; four are tier 3. None is tier 1.** And the thing on this page the human should read next:

> ### Where the five sit relative to the range under review
>
> **Because the tier-1 count is 0, the exit rule's first clause is satisfied and this section is
> not about a tier-1 row — there is none to place. It places every new finding instead**, because the
> exit rule's *third* clause turns on **location**, not tier.
>
> | # | finding | tier | inside F-10's new code? | inside the six-row usability batch? |
> |---|---|---|---|---|
> | 1 | `(2, A2-2)` = `(5, DEFECT 1 · rc.20)` — a provisioned fan-out worktree: the preview says clean, the door refuses, and the refusal's route exits **128** | **2** | **NO** — reproduces identically on the **ordinary** `task finalize` arm; the exemption is M51's `posture_subject`, the seam's `live` subject is M52's | **the `jigc start` half YES** — `be663712`, the batch's row 2 (*“a live task's findings carry the repository posture finalize refuses under”*) is silent in exactly this cell |
> | 2 | `(2, A2-3)` = `(3, F-D)` — F-10's design of record states `jigc task amend --format json` *“carries the pinned sha under `text`”*; it carries **no sha anywhere** | 3 | **YES** — it is a row of F-10's own settle, on the one row marked ***verify*** | no |
> | 3 | `(2, A2-1)` — `task finalize --dry-run`'s help promises its `findings` are `task validate`'s set; **two** of the four gates it refuses on are outside that set | 3 | **partly** — the amend arm contributes the *second* member (`finalize.base-mismatch`); the class was **already false** pre-F-10 on the ordinary arm's `finalize.empty-commit` | no |
> | 4 | `(3, F-C)` — `write.unslugable-title`'s route never names `jigc task amend`'s own `amend-<sha7>` fallback, the one exit that door uniquely has among `MINT_DOORS` | 3 | **YES**, in the complete-fix sense — `MINT_DOORS` grew a **sixth** member whose exit set is wider and the shared route was not re-derived over it | no |
> | 5 | `(5, DEFECT 2 · rc.20)` — `amend.head-shape`'s locus prints `work-unit:<id>`, a spelling no grammar in the product declares | 3 | **YES** — a code minted this wave | no |
>
> **So the exit rule's third clause is engaged, by three rows inside F-10's new code and one half-row
> inside the usability batch — and by nothing tier-1.** Read by **tier**, all five are below the line and
> none blocks the call. Read by **location**, the third clause fires, exactly as it did on rc.19 and
> rc.18. **That tension is stated here, not resolved: the adjudication is the human's and this record
> does not pre-empt it.**
>
> **A keying note, because axis 5's file collides with its own baseline.** `axis-5.md` numbers its two new
> defects `DEFECT 1` and `DEFECT 2`, and the same file carries the rc.17 baseline row **`(5, DEFECT 1)`**
> (a mint door committing at a fabricated identity, tier 1) as **CLOSED**. This record keys the new ones
> **`(5, DEFECT 1 · rc.20)`** and **`(5, DEFECT 2 · rc.20)`** and says so rather than silently
> renumbering an axis file it copies verbatim.

---

## The staffing

Per axis, three agents — M51 §19's shape, unchanged through M52 and all four M53 runs. **Nine agents over
three axes.**

- **One Opus driver** owns the `(door, cell)` table. It drives every row on the installed binary and
  records exit, code, route, surface and a repro block per row. **It may not mark a row driven from a
  source read.** Each driver was told not to read its axis's Codex pass, and each says so in its file.
- **One Codex source pass** ([codex/](codex/), verbatim, so every lead is auditable) owns **completeness
  of the row set** — the question driving cannot answer: *is there a door the registry does not carry, or
  a bypass of the seam this axis is about?* It reads; it drives nothing; it made no writes and no builds.
  Each pass states its own bound, and axis 5's is the sharpest: *“no binary behavior, tier-2/3 repro,
  release-only behavior, hostile cwd, or hook execution was independently driven.”*
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

It earned its keep four times this run, and the first two are the ones worth reading:

- **Axis 5 — the demotion pass refuted a count instead of a row.** The driver's §3 preamble states the
  `ENVELOPE_ARMS` partition as *“60 `Success` / 3 `Adjudicated` / 2 `Reject` · `Pinned` 60 /
  `Unpinned` 6”*. Read by symbol: **61 / 3 / 2** and **Pinned 59 / Unpinned 7** — two off-by-ones in the
  same direction, and *self-evidently* wrong, since `60+3+2 = 65` and `60+6 = 66` cannot both partition
  one 66-row registry. **No row verdict moves** (all seven Unpinned rows are driven and all seven are
  `= declared`), and the correction **corroborates Codex's C-15**, whose characterization of the Unpinned
  set is the right one. Recorded because this axis's subject is *counted registries*, so a count the
  review states wrong is a datum the next wave inherits ([axis-5.md](axis-5.md) → §A).
- **Axis 5 — a baseline count corrected against rc.19.** `DOCTYPE_DOORS` was recorded as **17** by the
  rc.19 run; the symbol carries **16** at this HEAD and the block is byte-unchanged in
  `a8904637..4d3175c3`, so the rc.19 figure counted one `DoctypeArg::` occurrence inside a doc-comment.
  Stated rather than silently re-asserted.
- **Axis 2 — twenty rows filed without a repro block, all twenty re-driven rather than discarded.** The
  driver's 50-row table carried 20 rows backed by argv-and-observation lines rather than fenced blocks.
  The reconciler drove **all twenty**; every one reproduced, so **none was demoted in the end** and each
  now stands on the reconciler's own repro ([axis-2.md](axis-2.md) → §A, §D). The *filing* was thin even
  though the *facts* held, which is why it is on the record.
- **Axis 3 — the one row where a drive and an independent source read met from opposite directions.**
  `(3, F-B)` was reached by the driver from the `--force` narration and by Codex from
  `milestone.rs:6986`'s `DoomedLine` construction over the leftover file's own basename, each with its
  own proposed argv. Every other finding on that axis was reached by driving alone.

---

## Roll-up

| axis | subject | Codex leads → CONFIRMED / REFUTED / OPEN | driver defects re-driven | demotions | new findings (tier) | doors covered (rc.19 →) |
|---|---|---|---|---|---|---|
| 2 | posture | 17 → **13 / 0 / 4** | **3**, all CONFIRMED | 0 (20 re-driven) | **3 — 1 tier-2 · 2 tier-3** | **35** (rc.19: 25) |
| 3 | destroying doors | 12 → **10 / 0 / 2** | **7**, all CONFIRMED | 0 | **2 — both tier-3** | **28** (rc.19: 18) |
| 5 | pinned contracts | 20 → **18 / 0 / 2** | **2**, both CONFIRMED | 0 | **2 — 1 tier-2 · 1 tier-3** | **48** (rc.19: 47) |
| **total** | | **49 → 41 / 0 / 8** | **12** | **0** | **7 rows = 5 distinct — 0 tier-1** | **48 / 48 union · uncovered: none** |

**The counting basis, stated rather than smoothed** (the rc.18 and rc.19 records' own precedent). The
orchestration's hand-off tallies for the three axes were `13/0/4`, `7/0/3` and `18/1/3`. Two of them do
not reproduce from their file's own ledger: axis 3's *confirmed 7* counts the **driver defects**
re-driven, not its Codex ledger (which is 10 / 0 / 2), and its *open 3* exceeds the two open leads its
§6 enumerates; axis 5's *refuted 1* is the **driver-count refutation** in §A (a count, not a behaviour) —
its Codex ledger refuted nothing — and its *open 3* likewise exceeds the two its §B names.
**The auditable set is the enumeration in §FINDINGS below, not this table, and where a number here
disagrees with a number in an axis file, the axis file governs, because it carries the repro.**

**No behavioural claim by either pass was refuted on any of the three axes.** The single refutation is a
count. **Axes 2 and 3 refuted nothing at all**, and neither pass contradicted a driven row anywhere.

**The four headline facts of the fourth partial re-run:**

1. **Zero tier-1 rows on all three axes**, each reached independently, each with before-controls — and
   the new committing arm was probed for the tier-1 shape at every cell it has (a byte lost, a tree
   moved, a doc promoted without being committed, a false green on `jigc validate`) and reached it
   nowhere.
2. **The usability batch's two target rows and rc.19's one tier-2/3 row are CLOSED, driven** — `(2, N-1)`,
   `(2, N-2)` and `(3, F-A)`, each by at least two of the three axes independently.
3. **All eight F-10 review findings verified closed on the installed release binary** — including HIGH-1
   over its **whole eight-leaf class** (the review drove three) and MEDIUM-4 **by mutation**, token by
   token, on a manifest-keeping throwaway pack. LOW-6 is recorded as *not a drivable surface* (a test
   comment) rather than as closed.
4. **Seven new finding rows, five distinct defects, none tier 1** — of which **three sit inside F-10's own
   new code** and **one half-row inside the usability batch's**.

---

# THE ROW-BY-ROW COMPARISON — every rc.19 §A row, all tiers, restricted to axes 2 · 3 · 5

Three layers, one per axis, each against **rc.19's §A layer for that axis** (which itself carried the
M52/M51/rc.17/rc.18 rows forward). **CLOSED** carries the argv that settles it; **STILL-OPEN** carries the
datum. Rows M53 deliberately did not fix — the charter's tier-2/3 triage into the 1.x ledger — are marked
**STILL-OPEN(1.x, expected)**, which is a *measurement*, not a finding. A row is keyed **(axis, id)**
exactly as its record keys it. **rc.19's Layer D (axis 6) has no layer here: axis 6 was not re-driven.**

## Layer A — axis 2, against [per-axis-review-rc19](../per-axis-review-rc19/README.md) §A Layer A + its axis-2 findings

| baseline row | tier | verdict on rc.20 | the argv / the datum |
|---|---|---|---|
| **`(2, N-1)`** (rc.19, = `(6, A6-R1)`) — the installed pre-commit hook announces an out-of-band **rename** on a commit containing none, and calls a staged change *“not staged in this commit”* | 3 | **CLOSED** | `git rm -q VISION.md; git commit` → rc **0** and the hook prints **nothing**; the before-control (an ordinary commit) is silent too. Driven by the axis-2 driver (§4.1), re-driven by its reconciler (§B.5), and independently by axis 5 (§5.4). [axis-2.md](axis-2.md) §4.1 · [axis-5.md](axis-5.md) §5.4 |
| **`(2, N-2)`** (rc.19) — the hook's *blocking* rename backstop is structurally unreachable for the entire **placement** family | 3 | **CLOSED** | `git mv` of **all four** placement singletons → rc **1** ×4, *“out-of-band managed-doc rename staged in this commit … (commit blocked)”*; the location-doctype control is byte-identical. Driven ×2 on axis 2, once on axis 5 over both families. [axis-2.md](axis-2.md) §4.2 · [axis-5.md](axis-5.md) §5.4 |
| **`(2, F-1)`** (rc.18) — the aimed route `git -C <repo-relative-path> …` does not run from the checkout that printed it | 2 | **still CLOSED** | Both `aim_at` callers × **4 doors** (`milestone finalize` · `task validate <sub>` · `milestone discard` · `uninstall`) × cwds (a)(b)(d) = **12 cells, one byte-identical span**; run **verbatim** rc **0** from **five** cwds including `/` and the fan-out worktree, and rc **0** again under a spaced repository root where `shell_operand` single-quotes it. [axis-2.md](axis-2.md) §4.3, §B.4 |
| **`(2, DEFECT A)`** (M52) — a clean `git cherry-pick --no-commit` is a member of no `InProgress` row | **1** | **still CLOSED** | All four `uncommitted-pick*` states answer `UncommittedCherryPick` at exit 1 across **8** acting doors **and** the new amend arm; the discriminating cell (`uncommitted-pick-conflicted`, index genuinely conflicted) still answers the **pick**, not `UnmergedIndex`. No consent flag bypasses (the no-override cross: `--carry-staged`, `--dry-run`, `--force` ×2, `--format json`). [axis-2.md](axis-2.md) §3.2, §4.5, §B.2 |
| **`(2, DEFECT 1)`** (rc.17's tier-1 row) — the boundary commits a provisioned worktree's un-concluded operation at exit 0 | **1** | **still CLOSED** | The worktree subject refuses at `bisect` from (a)(b)(d) at both producers, the refusal placed **before** the record flip; main HEAD unmoved, the milestone still finalizable. [axis-2.md](axis-2.md) §4.3, §B.4 |
| M51 **`D1`** · **`D2`** · **`D3`** · **`D3b`** | — | **still CLOSED** | The 16-state sweep: `rebase-merge`/`rebase-apply` are HEAD-detached in git and still answer *a rebase*; only the bare `detached` member reaches `repo.head-detached`; `am` and apply-backend rebase keep two nouns and two abort commands; `D3b` is driven — the amend arm's shared seam **did** re-probe and **did** refuse, HEAD unmoved. [axis-2.md](axis-2.md) §3.2, §B.10b |
| **`(2, DEFECT C)`** (M52) — a posture breach at the **commit seam** prints no state-truth clause and no copy-runnable re-run | 2 | **STILL-OPEN(1.x, expected)** — **door axis now wider by one** | Reproduces byte-for-byte with a deterministic `git` shim racing a bisect into the finalize transaction, on the **ordinary** arm *and* on the **amend** arm (shared `git_commit_capture`): one code, one `route:` line, nothing else, while HEAD is unmoved, the work still staged and the task's docs intact. The contrast arm one step over (a rejecting `pre-commit`) prints the full state-truth clause **and** the copy-runnable re-run. [axis-2.md](axis-2.md) §4.4, §3.6, §B.10 |
| **`(2, DEFECT B)`** (M52) — a conflicted `git merge --squash` is answered by `SquashMerge`, whose predicate is false of the state | 3 | **STILL-OPEN(1.x, expected)** | `.git` holds `MERGE_MSG` + `SQUASH_MSG` (no `MERGE_HEAD`), `git ls-files -u` → **3** unmerged, `git status --porcelain` → `UU conf.txt`; `jigc milestone create` → exit 1, *“a squash merge is **staged and not committed**”*. Nothing is staged. [axis-2.md](axis-2.md) §4.5, §B.9 |
| **`(2, F-2)`** (rc.18) — a foreign repository's worktree parked at `.jigc/worktrees/<sub-id>` is committed as the sub-task's own work | 3 | **STILL-OPEN(1.x, expected)** | `join` → 0, `finalize` → 0, *“added bsecret.txt … sub-tasks: fw-area: 1 code file”*; `git cat-file -p HEAD:bsecret.txt` → `B-SECRET-PAYLOAD`. **No byte lost** — B's untracked plant intact, B's 2 worktree registrations intact, `.jigc/displaced/` absent. The **one** row Codex reached independently on this axis, on its own proposed argv. [axis-2.md](axis-2.md) §4.6, §B.1 |
| **`(2, N-3)`** (rc.19) — `milestone provision` acks *“at base &lt;pin&gt;”* over a worktree it **reused** at a different commit | 3 | **STILL-OPEN(1.x, expected)** | Pin `cbc661c`, parked worktree HEAD `313b93d`; `provision` → exit 0 *“at base cbc661c”*; the worktree's HEAD **unchanged**; the boundary then repeats *“the sub-task worktrees were cut from `cbc661c…`”* at exit 3. Two law-1 claims the binary cannot support. [axis-2.md](axis-2.md) §4.7, §B.11 |
| **`(2, N-4)`** (rc.19, tier corrected 2→3 by the rc.19 reconciler) — a promoting `task finalize` from a linked worktree leaves the main checkout with a finding naming an out-of-band edit that never happened | 3 | **STILL-OPEN(1.x, expected), exactly as corrected** | The doc lands on `lwbr`; main's `jigc validate` prints `blocking (gates at finalize) · file-state.hash-matches` with the route *“re-author it through the owning workflow”* — and the **escalation leg still does not fire**: main's next `task finalize` **absorbs** it (`advisory · reconciliation.absorb`, route *no action needed*) and lands at exit 0. [axis-2.md](axis-2.md) §4.8, §B.12 |

## Layer B — axis 3, against [per-axis-review-rc19](../per-axis-review-rc19/README.md) §A Layer B + its axis-3 findings

| baseline row | tier | verdict on rc.20 | the argv / the datum |
|---|---|---|---|
| **`(3, F-A)`** (rc.19) — `jigc task finalize`'s Displace surfaces print host-absolute paths from a linked worktree, **including on the 1.0-pinned envelope** | 2/3 | **CLOSED** | From a branch-attached linked worktree, both cells: all-move → `displaced [{from ".jigc/tasks/fa-probe/notes.txt", to ".jigc/displaced/fa-probe/notes.txt"}]`, **0** host-absolute hits over stdout+stderr; blocked destination (`.jigc/displaced` a regular file) → `displaced []`, area standing, plant **1/1**, three path occurrences in `findings[0].message` all repo-relative, **0** host-absolute hits. Driven by the driver, re-driven by the reconciler on Codex's own argv, and a third time by axis 5 (§5.6). [axis-3.md](axis-3.md) R-6 · R-8 · RX-1 · [axis-5.md](axis-5.md) §5.6 |
| **`(3, F-3)`** (rc.18) — `jigc uninstall` inside a fan-out worktree exits 0 and reports an install it did not remove | 2 | **still CLOSED** | The **main** install removed in full, git's worktree registrations **pruned** (`git worktree list` 2 → 1, `.git/worktrees/` gone), the site line naming the standing worktree's removal, and all four WIP guards (`dirty-worktree`, `untracked-workbench-file`, `foreign-bytes`, `staged-prose`) **byte-identical from the worktree and from the root**. [axis-3.md](axis-3.md) R-27 · R-28 · R-29 |
| **`(3, A3-1)`** (M52) — `milestone finalize` destroys every byte jigc did not write in `.jigc/milestones/<id>/` at exit 0 | **1** | **still CLOSED** | 8 planted loci → exit 0, **8 before / 8 after**, 8 pairs on `committed.displaced` sorted by `from`, each foreign **directory** moved whole, both area roots empty, **0** host paths. [axis-3.md](axis-3.md) R-35 |
| **`(3, A3-2)`** (M52) — when the displacement fails, both `Displace` doors remove the area anyway | **1** | **still CLOSED at both doors — and now at a third arm** | `.jigc/displaced` a regular **file**: `task finalize` 1/1 with the area standing · `milestone finalize` **3 before / 3 after**, both areas standing, one keyed advisory per standing area, `displaced: []`, 0 host paths · and the **amend** arm 1/1, area standing. [axis-3.md](axis-3.md) R-4 · R-36 |
| **`(3, A3-3)`** (M52) — the boundary's own area is untested | 3 | **still CLOSED behaviourally** | The boundary driven from **four cwds** including the fan-out worktree: full landed envelope each, main HEAD advances in all four, areas cleared, 0 host paths. The structural half remains a source read (Codex `C-10`), the same disposition rc.17/rc.18/rc.19 gave it. [axis-3.md](axis-3.md) R-38 |
| the rc.18 post-review **HIGH** — a *spotless* worktree carrying an un-concluded operation | — | **holds at all three worktree doors** | `provision` → `milestone.leftover-holds-work`, `discard` → `milestone.dirty-worktree`, `uninstall` → `uninstall.dirty-worktree`, each naming the operation and routing `git -C <ABS> bisect reset` — and that route runs verbatim from three cwds. [axis-3.md](axis-3.md) R-20 · R-22 · R-28 |
| the rc.18 post-review **MEDIUM 1** — 0 host paths at `milestone finalize` from a sibling worktree | — | **holds, and its un-swept sibling is now swept** | Both `Displace` doors give 0 host paths in the identical cell from a linked worktree — which is `(3, F-A)`'s closure. [axis-3.md](axis-3.md) R-6 · R-37 |
| census **C2-06** · **C2-07** (= F-3) · **C1-06 / C1-14** | — | **all still CLOSED** | The boundary lands from four cwds; the `git -C <ABS>` spans run verbatim from three cwds **and** under a spaced repository root with a spaced filename operand, both single-quoted. [axis-3.md](axis-3.md) R-17 · R-22 · R-27 · R-38 |
| **`(3, F-B)`** (rc.19) — the `LeftoverShape::File` consent arm lists the leftover's **own basename** as though it were a child entry | 3 | **STILL-OPEN(1.x, expected)** | `warning: removing the leftover file .jigc/worktrees/area-one-work discards work that is not in git:` then `area-one-work` — the path's own basename in the child position; the `Directory` control in the same rig lists a real child (`precious.txt`). **The one row both passes reached independently.** [axis-3.md](axis-3.md) R-20 · RX-3 · C-3 |
| **`(3, F-1)`** (rc.18) — a symlink wearing a staged identity is jigc's own at the read/ack surfaces and a third party's at the same door's destroying probe | 3 | **STILL-OPEN(1.x, expected)**, all **three** surfaces, byte-for-byte — **and a fourth half** | `task discard` → `task-discard.foreign-bytes` naming `adr:via-symlink.md`; `doc list --task` → *`adr:via-symlink … managed`*; `--force` → *“dropped staged edits to: adr:via-symlink”*. External target **1/1** intact. The fourth half, driven by the reconciler: the `--force` warning says *“the only copy of these bytes — they are not recoverable”* over a symlink whose target survives. [axis-3.md](axis-3.md) R-32 · RX-10 |
| **`(3, F-2)`** (rc.18) — the residual note asserts *“is a directory”* over a shape it did not check | 3 | **STILL-OPEN(1.x, expected)** at **three** doors | `ln -s $OUT .jigc/tasks/ghost` → `task discard ghost`, `task validate ghost` and `task diff ghost` each exit 1 with the **byte-identical** sentence (one producer, `engine::state::residual_area_note`). The real-directory control makes the sentence true there. [axis-3.md](axis-3.md) R-33 · RX-11 |
| **`(3, F-4)`** (rc.19) — the fan-out teardown narrates a **deleted tracked** file as *“never staged”* under *“the only copy … not recoverable”* | 3 | **STILL-OPEN(1.x, expected)** | `area-one: keeper.md (never staged)` printed under the unrecoverable-bytes note, while after the run `git show HEAD:keeper.md` returns the bytes and the main checkout's copy is untouched. [axis-3.md](axis-3.md) R-39 · RX-12 |
| **`(3, F-5)`** (rc.18) — a settled sub-task's leftover is *“1 active task(s)”* at one door and *“a leftover, not live work”* at another, and the route names a command that refuses | 3 | **STILL-OPEN(1.x, expected), all three halves** | `task list` → *1 active task(s)* / `area-two [sub-task]`; `task discard area-two` → `task-discard.staged-prose` whose route names `jigc task discard area-two --force`; that route **run verbatim** → exit 1 `milestone.terminal`. HEAD unmoved. **Reachability still undischarged** — built by restoring a `cp -R` backup, as rc.18, rc.19 and both agents here did. [axis-3.md](axis-3.md) R-34 · RX-13 |
| **`OBS-1`** (rc.19, = OBS-E) — `uninstall.dirty-worktree`'s route carries an **unfilled** `<milestone-id>` placeholder | obs | **still open** | The route says ``jigc milestone discard <milestone-id> --force`` from both cwds while the `staged-prose` sibling one screen over names the real milestone. Placeholders are the house style, so it stays an inconsistency. [axis-3.md](axis-3.md) R-28 |
| **`OBS-2`** (rc.19) — `milestone finalize` from a branch-attached linked worktree commits onto **main** and says nothing about it | obs | **unchanged** | `CommitSite::differing`'s own prescription; from `linked` the boundary lands on `main` and `feat` is unmoved. [axis-3.md](axis-3.md) R-38 |
| **`OBS-4`** (rc.19) — `Disposition::Narrate` is empty by construction | obs | **unchanged** (OBS-F) | No `DESTROYING_DOORS` member holds it at `4d3175c3`, so the arm still has no drivable cell — corroborated by Codex `C-10`'s symbol read. |

## Layer C — axis 5, against [per-axis-review-rc19](../per-axis-review-rc19/README.md) §A Layer C

| baseline row | tier | verdict on rc.20 | the argv / the datum |
|---|---|---|---|
| **`(5, DEFECT 1)`** (rc.17) — a mint door commits a record at a fabricated identity | **1** | **CLOSED (stays closed)** — **and the new mint door joins the guard** | `milestone create ""` → exit 1 `write.unslugable-title`; **`task amend ""`** → exit 1, same code, *“cannot mint a task”*; `ls .jigc/tasks` → absent; `git status --porcelain` empty; HEAD unmoved. Driven by the driver and re-driven by the reconciler on Codex's `C-3`. [axis-5.md](axis-5.md) §2.1 · C-3 |
| **`(5, DEFECT 2)`** (rc.17) — `config remove-step`/`replace-step` refuse a step that **is** in the resolved include list | 3 | **STILL-OPEN(1.x, expected)** | On a virgin rig: `insert-step` lands `probe-step`, `start --explain` prints it in the resolved list, `remove-step` → `config.anchor-absent — no step \`probe-step\` body to fork`, `replace-step` → `config.step-id-collision`; control `remove-step …#implement` → 0. The route names the exact precondition the state satisfies. [axis-5.md](axis-5.md) §2.2 · C-4 |
| **`(5, DEFECT 3)`** (rc.17) — the colon-less-address bail is outside `RefusalKind` and carries no code | 3 | **STILL-OPEN(1.x, expected), still two producers under two wordings** | `rename vision --to "New Vision"` → `{"error":"\`vision\` is not a \`<type>:<slug>\` address …"}`; `doc show nosuchtype` → the same bail under a **second** wording. No `blocking · <code>`, no key, at either door. [axis-5.md](axis-5.md) §2.3 · C-5 |
| **`(5, DEFECT 4)`** (rc.17) — `doc show` blocks a **declared but unpopulated** optional leaf | 3 | **STILL-OPEN(1.x, expected)** | `doc schema vision --format json` advertises `vision:<slug>#meta/grounded-in` `required: false`; `doc show` of it → exit 1, `store.no-such-leaf`. The read surface denies a leaf its own schema surface advertises one screen earlier. [axis-5.md](axis-5.md) §2.4 · C-6 |
| **`(5, C1)`** (rc.19) — the pinned orientation rows declare `next_steps`, a reachable composition omits it | 3 | **STILL-OPEN(1.x, expected)**, driven at **both** declared arms, by **both** passes independently | On a pack-set with neither off-catalog verb (a `mv` into a `mktemp -d` stash, restored and the restoration verified): `Clean` drives `[header, schema_version, state, workflows]` against a declared set carrying `next_steps`, and **`ActiveTask` omits it too** — declared ⊋ driven at two of the four `start` arms, at exit 0, with nothing on either stream saying so. [axis-5.md](axis-5.md) §2.5 · C-1 |
| **`(5, D1)`** (rc.19) — `start --explain` emits a production `--format json` stdout arm `ENVELOPE_ARMS` does not carry | 3 | **STILL-OPEN(1.x, expected)** — and **sharper than rc.19 recorded** | Driver: exit 0, **eight** keys incl. `scalar_overrides` (after a `config set placement-root`). Reconciler, virgin rig: **seven** keys, no `scalar_overrides`. **Both are right, and the difference is the row's sharpest datum**: the undeclared arm's key set is not merely undeclared, it is **not fixed** — it varies with cascade state, which is exactly what a pinned row would have had to say. [axis-5.md](axis-5.md) §2.6 · C-2 |
| **rc.17 `DEFECT A`** — `store.unknown-type` emits a **URI-shaped** target at `doc show` where the contract fixes the bare doctype id | 3 | **STILL-OPEN(1.x, expected)** | `doc show 'nosuchtype:x'` → key `{store.unknown-type, "nosuchtype:x"}`; `doc schema nosuchtype` → `{store.unknown-type, "nosuchtype"}`. One code, two target shapes, so the stable key does not discriminate the same condition the same way at two doors. [axis-5.md](axis-5.md) §2.7 · C-9 |
| M51 **`DEFECT A · B · C · D`** | — | **all still CLOSED**, with the new 48th leaf inside the set | Two uniform sweeps over **48** leaves: **outside any git repository** 48/48 land in a declared arm (46 `Reject::Error`, 2 `Reject::Findings` — `setup`/`uninstall`), 0 NOT-JSON, 0 bare-`Finding` roots, stdout 0 bytes on all 48; **cwd deleted under the process** 48/48 `Reject::Error` with one identical `{"error": "cannot determine the current directory: …"}`. `task amend`, the 48th leaf, is inside both. [axis-5.md](axis-5.md) §6.1 · §6.2 · C-10 · C-11 |

---

# FINDINGS

## A · CONFIRMED — every new finding, tiered on the charter's predicate

The predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary
does not do.

### Tier 1 (0)

**None.** Every tier-1 candidate on every axis was driven to a safe outcome **with a before-control**:
the amend arm's sixteen posture cells, five index-shape cells, moved-HEAD cell and two hook-rejected
cells (HEAD byte-identical, raw object digest compared, in every one) · the eight `doc` write leaves
(`git status --porcelain` empty after each, and after the whole sweep) · `(3, A3-1)`'s 8 before / 8 after
· `(3, A3-2)`'s both-areas-standing at three arms · `(3, F-A)`'s plant 1/1 in every cell · the three
refusing destroying doors over one foreign complement (2 before / 2 after) · `(5, DEFECT 1)`'s HEAD
byte-unmoved over the refusals · and `(2, F-2)`, measured: nothing lost.

### Tier 2 (1)

#### `(2, A2-2)` = `(5, DEFECT 1 · rc.20)` · origin **two independent drivers**, re-driven by **both** reconcilers on fixtures built from nothing · **outside F-10's new code; its `jigc start` half INSIDE the usability batch's** · inside a provisioned fan-out worktree the three preview surfaces report a clean repository posture while the committing door refuses `repo.head-detached` — and that refusal's route exits **128** from the checkout that printed it

```
setup: rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       jigc milestone create "cwd wave"            -> 0   jigc milestone add-task cwd-wave "area one" -> 0
       jigc milestone provision cwd-wave           -> 0   "provisioned 1 worktree at base cea1417 (area-one)"
       git worktree list -> …/repo b06b237 [main] · …/repo/.jigc/worktrees/area-one cea1417 (detached HEAD)
       cd $REPO/.jigc/worktrees/area-one
       git status --porcelain=v2 -b | grep '^# branch.head'   -> # branch.head (detached)
       jigc start --workflow single-task "ordinary in worktree" -> 0   (an ORDINARY task, not a sub-task)
       commit doc authored; echo k > k.txt; git add k.txt

THE THREE SURFACES OF ONE GATE, all from that same cwd:
  jigc task validate ordinary-in-worktree              -> EXIT 0   advisory · changelog-recording… ONLY
                                                                   NO repo.* finding
  jigc task finalize ordinary-in-worktree --dry-run    -> EXIT 0   "would commit — … would commit in the
                                                                   linked worktree at `.jigc/worktrees/area-one`"
  jigc task finalize ordinary-in-worktree              -> EXIT 1
      blocking · repo.head-detached — HEAD is detached — a commit made here would belong to no branch,
        and the next checkout would leave it unreachable
        route: re-attach HEAD with `git switch <branch>`, then re-run this command
  git rev-parse --short HEAD -> cea1417 (unmoved by the refusal)

THE ORIENTATION HALF — M53's own usability row `be663712`, same cwd:
  jigc --format json start | jq -c '.tasks[]|{id, findings:…}'
      {"id":"area-one","findings":[]}
      {"id":"ordinary-in-worktree","findings":["changelog-recording.gate-granted-unused"]}
                                    ^^^ NO repo.head-detached on any live task

THE ROUTE, RUN VERBATIM from the checkout that printed it:
  cwd = $REPO/.jigc/worktrees/pw-one ;  argv: git switch main
  observed: fatal: 'main' is already used by worktree at '<ABS>/repo'        rc=128
  git branch -a -> * (no branch) / + main     <- there is NO <branch> the placeholder can take

CONTROLS that isolate the cell to a worktree jigc itself provisioned:
  (a) the STANDING checkout detached (rig --git-state detached): validate 1 · dry-run 1 · finalize 1  — agree
  (b) an ORDINARY LINKED detached worktree (git worktree add --detach): validate 1 · finalize 1       — agree
      and `start`'s tasks[].findings DO carry repo.head-detached there                                — the working half
  (c) a bisect additionally running in the MAIN checkout, asked from the fan-out worktree:
      task validate -> 0  (neither the main's bisect nor the worktree's detached HEAD)
  AN AMEND TASK in the same cwd: mint 0 · validate 0 · dry-run 0 · finalize 1 repo.head-detached — identical
AFTER every cell: main HEAD unmoved, worktree HEAD unmoved, nothing committed, nothing destroyed
```

**Contract contradicted, in four locked homes** — `design/finalize.md:76` (*“the preview asks the
**committing door's own guard**, with that door's own exemptions, and renders the identical finding at the
identical exit code”*) · `design/command-output-contract.md:488` (*“…at the identical exit code: **1**”*)
· `crates/cli/src/cli.rs:640-653` (`finalize_posture_refusal`'s doc-comment: *“the finding, its route and
its exit code are the committing door's, byte for byte”*) · and the composed `what's-left:` line every
task prints (*“previews … **the repository posture finalize refuses under**”*). Driven, it is exit **0**
against exit **1**, and **silence against a coded refusal**. **The mechanism, marked as a source read
after the cells were driven:** the preview layers ask `cli::posture_breach_in` → `repo::adjudicated_breach`
over `repo::posture_subject`, whose `PostureSubject::adjudicates` **exempts a `Checkout::Dedicated` from
`PostureMember::HeadDetached` and from that member only** — a deliberate carve-out so jigc's own
`--detach` provisioning is not refused; the **commit seam** does not take it, re-probing with a
`SeamSubject` built `live` at the standing checkout. Two askers, two answers, in the one cell where the
exemption bites. **Why not tier 1:** the gate holds, HEAD is unmoved in every refusing cell, the task is
recoverable, and the same task finalizes correctly from an attached cwd. **Why tier 2 and not tier 3:**
both legs are dead ends — a preview an agent reads to decide whether to finalize, and a route that is
textually perfect and returns 128. **Two axes, two fixtures, one result**, and **neither Codex pass is
silent in a way that contradicts it**: axis 5's pass explicitly bounds itself out of driving, and its one
negative (*“no new F-10-specific completeness defect”*) is about `ENVELOPE_ARMS` completeness, while this
is a preview-contract divergence **wider than F-10**. **Which door is wrong is deliberately not
prescribed** — axis 2's driver wrote both sides and says so. [axis-2.md](axis-2.md) §6 A2-2, §C.2 ·
[axis-5.md](axis-5.md) §8 DEFECT 1, §C.

### Tier 3 (4)

#### `(2, A2-3)` = `(3, F-D)` · origin **two independent drivers**, re-driven by both reconcilers · **INSIDE F-10's arc** (a record row, not a behaviour row) · F-10's design of record states that `jigc task amend --format json` carries the pinned sha under `text`; the envelope carries **no sha anywhere**

```
setup: rig committed-singletons ; HEAD short 8c73310 / full 8c733106ece180b875dba96493ddd2af1054617f
$ jigc --format json task amend "json probe"                              -> exit 0
  keys              : ['task', 'text']    (the declared ArmShape::Object — the SHAPE is right)
  'amending:' in text            -> False       'already been pushed' in text -> False
  \b[0-9a-f]{7,40}\b over text            -> set()
  \b[0-9a-f]{7,40}\b over the WHOLE envelope -> set()          stderr: empty
CONTROL, the TEXT arm of the same door: short sha 1 hit · 'amending:' 1 hit
CONTROL, the RESUME doors, text arm:   start --task 2 hits · workflow amend --task ≥1 hit
CONTROL, the same resume door, JSON:   0 hits
CONTROL, an ORDINARY start — the ack is a text-arm-only header at EVERY composing door:
  jigc start "…" --workflow quick-fix            -> TEXT: "task minted:" ×1
  jigc --format json start "…" --workflow quick-fix -> keys ['task','text'] ; "task minted:" ×0
```

**The stated rule**, `f10-amend-settle.md:58` → *Envelopes*: *“`jigc task amend` → the composed
`{task, text}` arm …; **`--format json` carries the pinned sha under `text`**, no new key needed —
**verify**; if a key is needed it is declared in the additive-key paragraph.”* **The binary is internally
consistent and the settle row is the false statement** — which is why this is filed as a record row. The
mitigating datum that decides the tier, and it is a real one: the row ends in *“— **verify**”*, the
verification came back negative and **was acted on** (the MEDIUM-2 fix commit `c65d3495` records the
decision in full, and the authority is a code-side census row at
`crates/cli/tests/text_json_parity_axis.rs` — `Disposition::DeclaredOut` with a three-clause reason, the
sha and subject being facts about the **repository**, readable by any driver from `git log -1 HEAD`). So
the binary matches its authoritative declaration; what is stale is the settle row — **unstruck, while
lines 43 and 53 of the same table carry dated `2026-09-26` correction brackets added by the same review
pass.** This file's own convention was applied to two rows and not to the third. **A second, smaller half
rides with it**, driven by both reconcilers: that census row's *subject* sentence describes the surface as
*“prose, pinned as {task, text}, **led by the `amending:` block**…”* while the `DeclaredOut` beneath it
declares that block out of the very arm the row is about — **the two halves of one row describe different
surfaces.** **Why tier 3 and not tier 2:** nothing is a dead end; the JSON driver has a runnable command
for the one fact it is missing, and the `task` key it does get is what every subsequent call needs.
[axis-2.md](axis-2.md) §6 A2-3, §C.3 · [axis-3.md](axis-3.md) §2 F-D, RX-9.

#### `(2, A2-1)` · origin **driver**, re-driven over **both** cells of its class · **partly inside F-10's new code, but the class is pre-existing** · `jigc task finalize --dry-run`'s help says its `findings` are the set `jigc task validate <id>` reports, and **two** of the four gates it refuses on are outside that set

The help text, quoted from the binary (`jigc task finalize --help`, the `--dry-run` arg):

> It *refuses* on **three gates**: this task's validation findings, the empty-commit guard, and the
> carryover gate … On an **amend** task the last two read differently … **Its `findings` are the set
> `jigc task validate <id>` reports**, the staging-independent `owner-artifact` causes included —
> reported here, decided at the real finalize.

```
CELL 1 — the AMEND arm's finalize.base-mismatch:
setup: rig committed-singletons ; jigc task amend "moved head" ; author type+summary
       jigc milestone create "Move head wave"   -> exit 0, HEAD 398ffeb -> 638cb9c
  jigc task validate moved-head                          -> exit 0  (BARE)  "no findings — the task validates clean"
  jigc --format json task validate moved-head            -> exit 0  {"schema_version": 3, "findings": []}
  jigc task finalize moved-head --dry-run                -> exit 3  (BARE)
    blocking · finalize.base-mismatch — `HEAD` is no longer the commit this amend was minted against …
      at: task:moved-head    route: (`jigc task discard moved-head --force`, then `jigc task amend`) …
  jigc --format json task finalize moved-head --dry-run  -> exit 3
    findings[0].key = {"code":"finalize.base-mismatch","target":"task:moved-head"}
  jigc task finalize moved-head                          -> exit 3, the identical finding
AFTER: HEAD unmoved by any of the four

CELL 2 — the ORDINARY arm's finalize.empty-commit, the member that makes this PRE-EXISTING:
setup: rig committed-singletons --start quick-fix "empty commit probe" ; author ; stage NOTHING
  jigc task validate empty-commit-probe                  -> exit 0  (BARE)  "validates clean"
  jigc --format json task validate empty-commit-probe    -> {"schema_version":3,"findings":[]}
  jigc task finalize empty-commit-probe --dry-run        -> exit 3  (BARE)
    blocking · finalize.empty-commit — task validated but produced no diff — nothing to finalize
  findings[0].key = {"code":"finalize.empty-commit","target":"task:empty-commit-probe"}

CONTROL — the ordinary arm does NOT reach base-mismatch: a plain quick-fix task with HEAD advanced
  out of band finalized at exit 0 ("it LANDS").
```

The sentence contradicts its own paragraph three lines above it: *“It refuses on three gates: this task's
validation findings, **the empty-commit guard**, and the carryover gate”* and then *“Its `findings` are
the set `jigc task validate <id>` reports”*. Of the four gates the forecast actually refuses on, **two**
are outside `task validate`'s set. **The preview's silence is a decision, not an oversight, and the code
says so** (`crates/cli/src/task.rs:2644`: *“the base pin is finalize-only by the M47 Settle, Decision 1”*;
`GATE_COVERAGE` carries no `base-mismatch` row at any tier; the composed `what's-left:` line names five
members of which this is not one). **The single thing that is false is the arg help** — its *three gates*
count and its parity sentence. **Why not tier 2:** the finalize refuses, HEAD is unmoved, and every
refusal's route is runnable. **The axis the fix is owed over is the whole set `--dry-run` refuses on**,
not either cell — a fix naming only `base-mismatch` would ship the same incomplete sweep one member over.
**The driver filed this one member too narrow and corrected it before writing it up**, which is on the
record in its own instrument-honesty section. [axis-2.md](axis-2.md) §6 A2-1, §C.1.

#### `(3, F-C)` · origin **driver**, re-driven with a sibling control · **INSIDE F-10's new code** (the complete-fix sense) · `write.unslugable-title`'s route never names `jigc task amend`'s own `amend-<sha7>` fallback — the one exit that door uniquely has among `MINT_DOORS`

```
setup: rig committed-singletons
$ jigc task amend ""                                                      -> exit 1
  blocking · write.unslugable-title — cannot mint a task: its id is slugged from the title, and this
    title slugs to nothing — ids are built from ASCII letters and digits …
    at: task
    route: re-run with a title carrying ASCII letters or digits — the task id is slugged from it
  same for "   " / "###" / "的的的"  (exit 1, byte-identical) ; "the of and" -> exit 0, mints `and`
  after each refusal: jigc task list -> "no active tasks"        <- NOTHING minted, git status clean

$ git rev-parse --short=7 HEAD  -> 1227894
$ jigc task amend                (no intent)                              -> exit 0
  task minted: amend-1227894                          <- THE EXIT THE ROUTE NEVER NAMES
$ jigc task amend --help
  "[INTENT] … Omitted, the task is named after the commit it rewrites (`amend-<sha7>`)"

SIBLING CONTROL:
$ jigc milestone create ""                                                -> exit 1
  the same code, route "… the milestone id is slugged from it" — CORRECT there: there IS no fallback
```

**Tier 3** in the *nothing hides* direction: no byte is lost, the door refuses **before any write**.
**Why it is the incomplete-sweep shape, not a wording slip:** `write.unslugable-title` is a **shared
producer** across `MINT_DOORS`' prose rows, and the advice is *correct* at every pre-F-10 member
(`jigc start ""`, `jigc milestone create ""`, `jigc milestone add-task … ""` have no fallback).
`MINT_DOORS` grew a **sixth** member whose exit set is wider, and the shared route was not re-derived over
the widened set — M45's complete-fix lens turned on M53's own new code, with the registry as the axis.
**Class bounded by drive:** one producer, one route string; the cell is `(jigc task amend, unslugable
intent)`. A neighbouring datum the reconciler recorded rather than filed: `jigc start ""` exits **0** (the
orientation arm), so it is the one door of that family where an empty title is not an error at all.
[axis-3.md](axis-3.md) §2 F-C, R-16 · RX-8.

#### `(5, DEFECT 2 · rc.20)` · origin **driver**, re-driven · **INSIDE F-10's new code** (a code minted this wave) · `amend.head-shape`'s locus prints `work-unit:<id>`, a work-unit address spelling no grammar in the product declares

```
rig: dev/jigc-rig --git-state unborn, then `jigc setup` (which births a ROOT HEAD)
$ jigc --format json task amend "unborn probe"           -> rc=1
  {"error":"blocking · amend.head-shape — … nothing was minted\n  at: work-unit:unborn-probe\n  route: …"}
$ jigc task amend "root probe"     (HEAD a root commit)  -> rc=1   at: work-unit:root-probe
$ jigc task amend                  (no intent)           -> rc=1   at: work-unit:amend-9b5667c
$ ls .jigc/tasks                                         -> No such file or directory  (nothing minted)

the sole producer, by symbol:
$ grep -rn 'format!("work-unit:' crates/cli/src crates/engine/src
  crates/cli/src/start.rs:358:            format!("work-unit:{id_hint}"),
what the locked docs declare:
  design/structural-grammar.md:98      — "addresses are `type:name` … (`task:add-rate-limiter`, `milestone:m1`)"
  design/command-output-contract.md:249 — "the work-unit ref `task:<id>` / `milestone:<id>`"
and no read surface resolves the third spelling:
$ jigc --format json doc show work-unit:root-probe       -> rc=1
  findings[0].key {"code":"store.unknown-type","target":"work-unit:root-probe"}
```

**Graded honestly, and the contradiction is thin — the axis file says so itself.** The finding **projects
no key** (it flattens through `finding_to_err`, which `command-output-contract.md` → *The membership test*
puts outside the envelope), so no declared **target-form** row is falsified; what the spelling reaches is
the printed `at:` locus alone. And the code's own comment gives a real rationale: the id is one the mint
*would* have taken, so `task:<id>` would name a task that does not exist. The row is that **nothing
declares the third spelling**, on a code minted this wave, while the two declared ones are enumerated in
two locked docs. Tier 3, surface-tier, reversible after 1.0. [axis-5.md](axis-5.md) §8 DEFECT 2, §C.

---

## B · REFUTED — each with its falsifying datum (1)

| # | claim | pass | falsifying datum |
|---|---|---|---|
| axis 5 — the driver's §3 partition line | *“**60 `Success` / 3 `Adjudicated` / 2 `Reject`** · **`Pinned` 60 / `Unpinned` 6**”* over the 66-row `ENVELOPE_ARMS` registry | driver | Read by symbol at `4d3175c3`: `grep -c 'outcome: ArmOutcome::Success'` → **61** · `Adjudicated` 3 · `Reject` 2; `ArmStatus::Pinned` → **59** · `Unpinned` → **7**. Two off-by-ones in the same direction, and self-evidently wrong: `60+3+2 = 65` and `60+6 = 66` cannot both partition one 66-row registry. The seven `Unpinned` rows by symbol are `describe \| Menu` + the five `milestone … \| RecordOnlyAck` + `milestone list-tasks \| Listing` — exactly the set **Codex's C-15** names, so the correction **corroborates** the source pass. **No row verdict moves**: all seven are driven and all seven are `= declared`. [axis-5.md](axis-5.md) §A |

**Nothing either Codex pass claimed was refuted on any of the three axes, and no driven row was
contradicted by any source read.** The one refutation above is a **count**, not a behaviour, and it was
found by the demotion pass rather than by a drive. Two further **data corrections** ride beside it and are
recorded rather than filed:

- **`DOCTYPE_DOORS` was 17 in the rc.19 record and is 16 by symbol** — the block is byte-unchanged in
  `a8904637..4d3175c3`, so the rc.19 figure counted one `DoctypeArg::` occurrence inside a doc-comment.
  ([axis-5.md](axis-5.md) → Notes.)
- **Codex's `C-20` reads as though `609da011` were a file hash**; it is a **commit** (the rc.19 review
  commit), and `e15d332b…` is the file digest. Read that way the claim is exactly right: both schema
  manifests are byte-identical across `609da011..4d3175c3`. ([axis-5.md](axis-5.md) → C-20.)
- **One immaterial driver/reconciler deviation on axis 2**, recorded for honesty: `jigc workflow amend
  --task <id>` was measured at **1** hit of `amending` by the driver and **2** by the reconciler. Both are
  non-zero, so the claim (*the resume doors do carry it on the text arm*) holds either way; the count is
  not load-bearing and is not filed.

## C · OPEN leads — driven as far as the state allows, promoted by nothing (8)

| # | lead | pass | why it stays open |
|---|---|---|---|
| `(2, B.13)` | both `git mv` and the displacement `git rm --cached` have an immediate `verify(Move)` | codex | The displacing doors' subject is the M52 *writer-set complement*, not a repository posture, and no rig state produces the cell without hand-writing into `.jigc/`. **It is a no-bypass claim, not a defect claim**, so leaving it open asserts no defect. |
| `(2, B.14)` | `DedicatedWorktree` is unforgeable outside `task.rs` — private fields, private constructor, typed `SeamSubject::dedicated` | codex | A claim about what **cannot be written**. Corroborated by a source read at HEAD; it has no behavioural cell a driver can reach, and the reconciler declined to promote a source read to a finding-grade CONFIRMED. |
| `(2, B.15)` — the **re-probe** half | the fan-out fast-forward merge has an immediate commit re-probe (`task.rs:7871`) | codex | Present in source with its own doc-comment; racing it needs a shim fired **inside the boundary's own combine transaction**, and M52's record declares a genuine concurrent racer out of this axis's scope. *(The claim's **absence** half — no production HEAD-changing `switch`/`checkout` exists — was split out and **CONFIRMED** on a complete grep: exactly two hits, both string literals in `GIT_NON_PATH_COMMANDS`.)* |
| `(2, B.16)` | the named M52 registries and their consumers add no commit/move act and bypass no posture seam | codex | A breadth claim with no single cell. The closest behavioural evidence is the 48-row `BEHALF_DOORS` total classification plus the 16×8 refusal sweep; recorded open rather than promoted. |
| `(3, C-7)` | M51 `D-3` CLOSED — the staged-prose enumeration renders its unreadable `docs/` path through `repo_relative` (`task.rs:1259`) | codex | **The cited site is unreachable through any door the reconciler could build.** With `.jigc/tasks/<id>/docs` at mode `000`, all three doors fail closed **earlier**, at the foreign-bytes readability guard. The claim's **consequence** is corroborated (every one of those messages is repo-relative, 0 host-absolute hits), but the enumeration itself was never entered. Reaching it needs the foreign scan to succeed while the docs read fails — a race, not a state a rig builds. |
| `(3, C-11)` — the exhaustive half | the removal-site census: **no unguarded production removal of foreign/user bytes** anywhere | codex | A **negative existential over all production paths**; no drive can establish it, and Codex bounds it itself. Its one checkable half is **CONFIRMED** (`ls crates/cli/src/uninstall.rs` → *No such file or directory*), and the reconciler found no counter-example in any cell it drove. |
| `(5, C-13)` | *“success rows falsely promising invariant exit 0”* is CLOSED as a prose claim | codex | **Not drivable.** The subject is the wording of the registry's own doc-comment about its status/outcome model, not a byte any invocation emits. A source read agrees with Codex, but that is a second source read, not a drive, and this ledger does not promote a source read to a finding. |
| `(5, C-18)` | *“the four proof fences remain”* (`format_json_success_axis.rs:1272-1575`) | codex | **Not drivable in this posture.** These are `#[test]` targets; observing them means building and running the debug test target, and this review's binary is the **installed release**. Named rather than promoted, and rather than dropped. **Recoverable at zero cost by the party that owns the tree.** |

**On C-19, the convergence claim, stated rather than implied.** Two passes with **disjoint methods** —
Codex by a source census of every command stdout JSON path, the driver by **96** uniform hostile-cwd cells
over all 48 leaves plus **216** four-cwd cells — found the **same single** undeclared arm (`start
--explain`, = `(5, D1)`), and the reconciler's own spot-drives added none. **That is the strongest
completeness evidence this instrument produces, and it is still not a proof**: the registry's own declared
bound (`render.rs:6560-6568`) says an arm reachable only under a state neither pass built appears in
neither. The honest summary is *no third undeclared arm was found by either method*, **not** *there is no
third undeclared arm*.

## D · Observations driven and carried — **not** defects

Recorded so a next reader does not re-drive them as findings. Each is graded against a written
declaration, quoted in its axis file.

- **`AmbushDisposition::DeclaredWhereReachable` is weaker at runtime than its two siblings** — dropping the
  two amend codes from `amend-message.yaml`'s `states-constraints:` leaves pack-load **green**, because the
  disposition is held out of `ambush_class_codes()` **by design** so a methodology-alone composition does
  not redden. Two production homes and `design/surface-contract.md:118`/`:143` state the outcome in as many
  words; what binds the `declarer:` claim is `crates/cli/tests/stated_at_fence.rs`, a **repo test**, not a
  runtime fence. **Filing it would be filing the decision, not a divergence.** ([axis-2.md](axis-2.md) §5,
  §8.1 · [axis-3.md](axis-3.md) OBS-G.)
- **`--pack-from-dev` makes the named-fact tier vacuous, and a reviewer reaching for it measures a false
  green.** Two mutants that redden at five doors on a `--repin` rig exit **0** on a `--pack-from-dev` rig,
  because that flag **drops the freeze manifest** and the fence keeps the manifest subject by its own
  recorded design. **This is an instrument measurement, and it caught a real mis-filing in flight**: axis
  3's first MEDIUM-4 mutation ran under `--pack-from-dev`, came back green, and would have filed MEDIUM-4
  as still open. ([axis-2.md](axis-2.md) §5, §8.2 · [axis-3.md](axis-3.md) §6, R-19 · [axis-5.md](axis-5.md)
  §4.13.)
- **Amending jigc's own structural commits at HEAD is allowed by design and is driven harmless** — a
  milestone **record-only** commit and a landed milestone **boundary** commit both reworded at exit 0, tree
  identical, record md5 intact, `base:` line intact, `validate` 0, `doc show` 0, `milestone add-task`
  afterwards 0. The settle refuses to invent a discriminator that does not exist, the composed step names
  the three shapes, and the mint ack prints HEAD's subject line. ([axis-2.md](axis-2.md) §8.4, §D.)
- **A bisect begun inside the same `git` process that performs the amend lands at exit 0** — a genuine race
  window, outside any predicate jigc can ask, and M52's record names the hook as the design's racer.
  ([axis-2.md](axis-2.md) §8.3.)
- **`OBS-A` / `OBS-B`** — the amend mint door refuses HEAD's **shape** and not its **attachment**, so a task
  minted in a fan-out worktree can only be discarded from there (the behavioural half of A2-2); and the
  mint ack calls a provisioned fan-out worktree *“the linked worktree at `.jigc/worktrees/area-one`”*,
  which git's own vocabulary makes true and which renders **repo-relative**. ([axis-3.md](axis-3.md).)
- **`OBS-C`** — `milestone finalize`'s landed envelope is `Object(["committed"])`, so its
  `finalize.foreign-bytes` advisories reach **stderr text only** while the identical cell at `task finalize`
  puts them on `findings`. The **declared, `Pinned`** shape, with the reason carried on the registry row —
  recorded because a driver reading only the machine surface sees two `Displace` doors answer one cell
  differently. ([axis-3.md](axis-3.md) OBS-C.)
- **`OBS-D`** — orientation's per-task coverage line says *“the carryover gate”* on an amend task, where the
  answering check is `finalize.amend-index-dirty`. The **declared bound** [VERDICT.md](../VERDICT.md) →
  Addendum 3 carries verbatim; the composed `what's-left:` line **is** model-aware and `task finalize
  --help` states the substitution. Graded **against** the bound. ([axis-3.md](axis-3.md) OBS-D.)
- **`OBS-R2`** — an over-warning shape, now countable at **three** instances on axis 3 alone: `F-4`, the
  fourth half of `F-1`, and the C-6 cell where *“the only copy … not recoverable”* prints, the removal then
  **fails**, and the bytes are still on disk one line later where the ack says so. Recorded so the shape is
  countable, not filed as a fourth finding. ([axis-3.md](axis-3.md) §3.)
- **`OBS-R1`** — `jigc uninstall --force` over a `.jigc` subtree containing an unreadable directory exits 1
  with `uninstall.remove-jigc` **after a partial teardown**. A `chmod 000` state the agent manufactured, the
  consent given, and the failure carries a code and a route — the failure-*point* axis, which the
  confidence-audit wave already treats as its own class. ([axis-3.md](axis-3.md) §3.)
- **The pre-dispatch fault outranks every guard** — a deleted cwd answers with one identical
  `Reject::Error` line at all **48** leaves, ahead of every guard, carrying no code and no route. The
  shipped pre-dispatch-fault shape (M52 Increment 1), outside the findings envelope and therefore outside
  the route floor. ([axis-5.md](axis-5.md) §6.2.)
- **`task finalize --force` and `milestone finalize --force` do not exist** (driven bare, exit 2), and
  `Disposition::Narrate` is **empty by construction** at rc.20 — an empty arm, not a skipped one.

---

# COVERAGE — the three axes against the 48 `VERB_KINDS` leaves

**The leaf count, read rather than quoted.** `crates/cli/src/cli.rs:1877`'s `VERB_KINDS` carries **48**
leaf rows at HEAD `4d3175c3` — **47 → 48**, `task amend` joining as the 28th row, a `Write` leaf. Counted
by reading the `(&[…], VerbKind::…)` rows of the const itself over its own span
(`awk 'NR>1877 { if ($0 ~ /^\];/) exit; print }' crates/cli/src/cli.rs | command grep -cE '^\s+\(&\[' →
48`), and **all three axis files read the same 48 independently, by symbol**. The spellings and their
order are the rc.19 table's with one row inserted, so the columns below are directly comparable.

**What the table reports.** Which of the **three re-driven axes** reached each leaf as the door of ≥1
**driven** row — the union of the three files' *Doors covered* sections — against the same column from the
rc.19 run **restricted to axes 2 / 3 / 5**. `uncovered` is the set of leaves no re-driven axis reached.

> **`uncovered`: NONE — 48 / 48**, and it is carried by **axis 5 alone**, which is the door of ≥1 driven
> row at every leaf in each of its two uniform 48-leaf hostile-cwd sweeps. The stricter readings are
> reported beside it rather than smoothed into it.

**The stricter columns, two of them, because one number would hide the gradient:**

- **substantive** — the leaf is the door of ≥1 row whose assertion is **specific to that leaf**: a captured
  key set at a declared envelope arm, a posture/destroying/residual cell, or a success or refusal arm with
  its own code and route. **48 / 48.** The mechanism, stated so it can be checked: axis 5's §3 drives
  **65 of the 66 `ENVELOPE_ARMS`** arm by arm with the key set captured and compared against the declared
  `ArmShape` extracted **mechanically** from `render.rs`, and those 65 arms span every one of the 48
  leaves. The one arm not driven is `doc show | CompoundFieldSlice`, unreachable at HEAD (§9.1) — and
  `doc show` is carried by its seven other driven arms, so no leaf loses substantive coverage to it.
  *Stated rather than smoothed:* axis 5's own §12 words this as *“**47** are additionally the door of a
  success-arm or condition-specific row … the one leaf whose coverage beyond the sweeps rests on a
  **partial** arm set is `doc show`”* — the 47 is that partial-arm caveat, not a leaf with no
  leaf-specific row.
- **behavioural beyond the pinned-envelope axis** — the leaf is the door of ≥1 row on axis **2 or 3**, i.e.
  a posture or destroying-door cell rather than a pinned-contract arm. **36 / 48** (rc.19, restricted:
  **27**). The **12** that are axis 5's alone: `upgrade` · `ingest` · `migrate` · `unmanage` · `task bind` ·
  `config insert-step` · `config replace-step` · `config remove-step` · `config fill` · `config fork` ·
  `config get` · `config list`.

| # | leaf | rc.20 axes (driven) | rc.19 (2/3/5) | Δ | substantive | beyond envelope |
|---|---|---|---|---|---|---|
| 1 | `start` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 2 | `workflow` | 2, 3, 5 | 2, 5 | **+3** | ✔ | ✔ |
| 3 | `setup` | 2, 5 | 2, 5 | — | ✔ | ✔ |
| 4 | `uninstall` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 5 | `upgrade` | 5 | 5 | — | ✔ | — |
| 6 | `ingest` | 5 | 5 | — | ✔ | — |
| 7 | `migrate` | 5 | 5 | — | ✔ | — |
| 8 | `migrate-corpus` | 2, 5 | 2, 5 | — | ✔ | ✔ |
| 9 | `unmanage` | 5 | 5 | — | ✔ | — |
| 10 | `rename` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 11 | `relocate` | 2, 5 | 2, 5 | — | ✔ | ✔ |
| 12 | `describe` | 2, 5 | 2, 5 | — | ✔ | ✔ |
| 13 | `validate` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 14 | `doc create` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 15 | `doc add-item` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 16 | `doc remove-item` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 17 | `doc retitle-item` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 18 | `doc rename` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 19 | `doc set-field` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 20 | `doc set-slot` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 21 | `doc author` | 2, 3, 5 | 5 | **+2, +3** | ✔ | ✔ |
| 22 | `doc show` | 2, 5 | 2, 5 | — | ✔ | ✔ |
| 23 | `doc schema` | 2, 5 | 5 | **+2** | ✔ | ✔ |
| 24 | `doc list` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 25 | `task list` | 3, 5 | 2, 3, 5 | **−2** | ✔ | ✔ |
| 26 | `task diff` | 2, 3, 5 | 3, 5 | **+2** | ✔ | ✔ |
| 27 | `task validate` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| **28** | **`task amend`** *(new leaf)* | **2, 3, 5** | — *(did not exist)* | **NEW** | ✔ | ✔ |
| 29 | `task discard` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 30 | `task finalize` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 31 | `task bind` | 5 | 5 | — | ✔ (driven at **success** this run) | — |
| 32 | `config set` | 2, 3, 5 | 2, 5 | **+3** | ✔ | ✔ |
| 33 | `config insert-step` | 5 | 5 | — | ✔ | — |
| 34 | `config replace-step` | 5 | 5 | — | ✔ | — |
| 35 | `config remove-step` | 5 | 5 | — | ✔ | — |
| 36 | `config fill` | 5 | 5 | — | ✔ | — |
| 37 | `config fork` | 5 | 5 | — | ✔ | — |
| 38 | `config get` | 5 | 2, 5 | **−2** | ✔ | — |
| 39 | `config list` | 5 | 5 | — | ✔ | — |
| 40 | `milestone create` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 41 | `milestone add-task` | 2, 5 | 2, 3, 5 | **−3** | ✔ | ✔ |
| 42 | `milestone add-from-spec` | 2, 5 | 2, 5 | — | ✔ (driven at **success** this run) | ✔ |
| 43 | `milestone list-tasks` | 2, 3, 5 | 3, 5 | **+2** | ✔ | ✔ |
| 44 | `milestone provision` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 45 | `milestone execute` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 46 | `milestone join` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 47 | `milestone finalize` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |
| 48 | `milestone discard` | 2, 3, 5 | 2, 3, 5 | — | ✔ | ✔ |

**`uncovered(<reason>)`: EMPTY.** Every one of the 48 leaves is the door of ≥1 driven row on axis 5; **35**
of them on axis 2, **28** on axis 3. **`task amend`, the new 48th leaf, is reached by all three** — as the
mint door of a posture sweep, a `MINT_DOORS` row, a destroying-door subject, an `ENVELOPE_ARMS` row and a
refusal cell in both uniform sweeps.

**Leaves that lost coverage against the rc.19 baseline (restricted to axes 2/3/5): NONE at the union.**
Every leaf reached by rc.19's axis 2, 3 or 5 is reached again here. **Per axis the picture is not flat,
and the three losses are named rather than netted away:**

- **Axis 2: 25 → 35.** Lost **`task list`** and **`config get`** — both were rc.19 axis-2 rows and are
  **deliberately not claimed** this run: axis 2's §11 states the rule it kept, that leaves used only to
  build fixtures or read state carry no verdict row. **Both are reached here by axis 5** (at a
  captured-key-set envelope arm and in both uniform sweeps), and `task list` additionally by **axis 3**
  (rows 109–110, where it is the door of an `F-5` verdict). Gained: the eight `doc` write leaves,
  `doc schema`, `task diff`, `milestone list-tasks`, `workflow` and `task amend`.
- **Axis 3: 18 → 28.** Lost **`milestone add-task`** — rc.19's axis 3 counted it, and this run's axis 3
  explicitly does **not**, on the same fixture-only rule (its §1 row-count note names `jigc setup` and
  `jigc milestone add-task` as the two verbs appearing only in fixture construction). **Reached here by
  axes 2 and 5.** Gained: `task amend`, `workflow`, `config set`, the eight `doc` write leaves and
  `doc author`.
- **Axis 5: 47 → 48.** No loss; it gained the new leaf, and two leaves rc.19 could drive only at their
  **reject** arms — **`task bind`** and **`milestone add-from-spec`** — are **driven at success this run**.

**The five axes NOT re-driven — 1, 4, 6, 7 and 8 — are not represented in this table at all.** A leaf's row
above says what axes 2, 3 and 5 reached on rc.20. It says nothing about the caller-token,
transaction/rollback, composed-surface, freeze/migration or adopter-docs axes: axis 6's rows were last
driven on `1.0.0-rc.19`, and axes 1, 4, 7 and 8's on `1.0.0-rc.16` (M52's run). Their rows stand unchanged.

---

# HONEST BOUNDS — what each axis states it did **not** drive

Each axis file carries its own section ([axis-2.md](axis-2.md) §7 + §8 + §12 + reconciliation §G ·
[axis-3.md](axis-3.md) §6 + reconciliation §3 · [axis-5.md](axis-5.md) §9 + §13 + reconciliation §E); this
is the roll-up, not a replacement.

**The bound that covers the whole record: this is a PARTIAL run.** Axes **1 (caller tokens) · 4
(transaction / rollback) · 6 (composed surfaces) · 7 (freeze & migration) · 8 (adopter docs & help) were
NOT re-driven**, by design. Their rows were not re-checked here. **Nothing in this record says they held;
it says they were not asked.**

**The instrument's own fault, stated because a review of doors that commit at exit 0 has no business
hiding one.** Early in axis 5's reconciliation a `rig=$(…); eval "$rig"` left `$REPO` **empty** (the rig
script's banner lines were swallowed by a stdout redirect on the `eval`), and the subsequent `cd "$REPO"`
was a **no-op in zsh rather than an error** — so a `mkdir -p docs/deep`, a `git add` and a `git commit -q`
landed **in the working repository**, creating commit `344c08e6` *“chore: deep dir”* on `main`. It was
undone in the same minute — `git reset --mixed 4d3175c3`, then `find docs -mindepth 1 -delete && rmdir
docs` (no `rm -rf`, no variable path) — and the **state was restored and verified**: `HEAD = 4d3175c3`,
working tree carrying only the one pre-existing modification
(`completions/artifacts/M53/per-axis-review-rc20/instrument/per-axis-review.workflow.js`), `docs/` absent,
no stray object reachable from any ref. Every later rig eval in that run carries an explicit
`[ -n "$REPO" ] || exit` guard. **With that single, undone exception, nothing was fixed, committed or
edited in the working repository by any agent in this review.**

**Axis 2 · posture**

1. **`InProgress::ALL` × each of the 12 acting doors as a full cross.** Driven: **16 states × 8 doors**
   (one fresh rig per state), **1 state (`bisect`) × all 12 doors**, and **16 states × the amend arm**. The
   un-driven cells are the 15 non-`bisect` states at the four doors of the second block; the member is
   decided by one shared `adjudicated_breach` composition and the door only filters on `PostureMember`, so
   the cross is not independent and both projections are driven.
2. **`InProgress::ALL` × the fan-out-worktree subject, member by member** — driven at `bisect` and at
   `detached` (A2-2). **Both `finalize.fan-out.squash` commit models** at the fan-out posture cell — the
   default only.
3. **`(2, DEFECT C)` at the *milestone boundary's* seam** — the `task finalize` seam was raced on both arms;
   the boundary's own was not. **The `GIT_DIR` redirect** — the family's declared-out bound. **A genuinely
   concurrent racer** — out of scope and a declared M52 bound; the shim is a deterministic single-process
   *timing* fixture, not concurrency.
4. **An apostrophe- or `#`-bearing repository root** (the **space** axis was driven, on two rigs). **A
   pushed commit** — the settle declares pushed-ness undetectable and ships advice rather than a fence.
5. **Every `--format json` arm of every acting door under every member** — driven at five doors; the rest
   are axis 5's table and are not claimed. **The registry tables of `PATH_ARG_OCCURRENCES`,
   `DOCTYPE_DOORS`, `SLUG_DOORS`, `WORK_UNIT_ID_DOORS`, `SchemaChangeKind`, `ManifestKind`** — counts read,
   rows not driven; axes 5 and 7 own them. **The `stated_at_fence.rs` / `count_fences.rs` suites** — the
   fences were driven *through the binary* with mutated packs; the Rust suites were not run, and a green
   suite is not a driven row.
6. **Four Codex claims left OPEN rather than promoted on the source read** (§C above); three are *no-bypass*
   claims, so leaving them open asserts no defect.

**Axis 3 · destroying doors**

1. **`Disposition::Narrate`** has no `DESTROYING_DOORS` member at rc.20 — an empty arm, not a skipped one.
   **`jigc task finalize --force`** does not exist on either commit model (the F-10 settle refuses one for
   the amend arm; Addendum 3 carries it as a declared bound).
2. **The remaining five `InProgress` members at the amend arm** — `{merge, bisect, revert, unmerged-index,
   detached}` were driven here; the others are **axis 2's** subject and were driven there (16/16).
3. **A genuine concurrent racer** at any destroying door and at the amend arm's rollback — the doors were
   driven serially; nothing here discharges `design/storage.md` → *Concurrent writers*. **`GIT_DIR`** —
   M51's declared posture residual.
4. **`LeftoverVerdict::OwnWorktree` at `uninstall` and `task discard` specifically** — driven at
   `milestone provision`/`milestone discard` and at `uninstall`'s dirty/bisect cells; the shared-classifier
   claim for the remaining pair is a source fact not re-derived.
5. **`F-5`'s reachability** — built by restoring a `cp -R` backup, as rc.18, rc.19 and both agents here did.
   **Behaviour driven; reachability undischarged.**
6. **`jigc task amend` under a spaced root at the *mint* door's checkout clause** — the spaced root was
   driven at the index route, where bytes reach a shell; the mint ack's checkout clause carries a *declared*
   absolute and no runnable span. **The F-10 acceptance suite's own arms** — LOW-6 is a claim inside a test
   comment, not a drivable user surface.
7. **Six instrument faults of its own are recorded rather than dropped** — the `--pack-from-dev` mutation
   that measured a false green (which would have mis-filed MEDIUM-4); a mutant that removed the front
   matter with the paragraph; **four early cells whose `$?` was read through `| head`**, every one
   re-driven **bare** before being tabled; a `staged-prose` fixture built through the wrong door; and two
   fixtures needing `mkdir -p .jigc/tasks` on a `fresh` rig.

**Axis 5 · pinned contracts**

1. **1 of the 66 `ENVELOPE_ARMS` was not driven** — `doc show | CompoundFieldSlice`, because **no shipped
   doctype declares a field of that shape**, verified across all **fourteen** shipped doctypes. **65 / 65
   driven arms: declared == driven.**
2. **`STORE_EXIT_FLIPS` at 6 of its 7 members** — the exit-flip members need a manufactured pack, a
   down-stamped corpus or a removed doctype (**axis 7's fixture work**); the registry is byte-unchanged in
   the range, which is **stated, not driven**. **`ManifestKind::ALL`** and **`SchemaChangeKind::ALL × LOCI`**
   shape values *inside* two envelopes whose carriers are driven — **axes 4 and 7 own them**.
3. **`setup` / `uninstall` excluded from the four-cwd divergence sweep** as destructive; both driven at
   their own cells and in both hostile-cwd sweeps.
4. **The `Route` *kind*** is not observable in release posture — every route here is recorded by the
   **bytes it emitted** and, where it is a command, by **running it**; the kind is never asserted.
5. **`finalize.stage-failed`'s copy-runnable route** was not driven (it needs a manufactured index fault),
   and **`doc show` over a *relocated* doc** was **not driven at its cell** — the fixture did not reach the
   relocated state, so what ran was an ordinary `store.not-found`. Both stated as un-driven rather than
   reported.
6. **A genuine concurrent / Task-tool fan-out envelope** — `milestone join` and both `milestone finalize`
   arms were driven **single-process** through real provisioned worktrees. The genuine spawn is the standing
   honest bound M51's VERDICT carries, unchanged.
7. **The `DeclaredWhereReachable` reason's *negative* half** (*“`Owed` would redden pack-load for the
   methodology pack”*) was not driven — it asks what a **different** registry value would do, which needs a
   rebuild. Its **positive** half was driven.
8. **`codex/axis5-prompt.md` was named to the reconciler and does not exist in the scratchpad `codex/`
   directory** *(the three prompts live in [instrument/](instrument/) and are published here)*, so that
   pass was reconciled on its own text with no sight of the instruction that produced it. Stated rather
   than worked around.

**Bounds that bind all three axes.**

- **Release posture.** Every drive ran on the installed release binary, where the debug route, span and
  quoting fences do not exist. A fence violation is a **bad emitted command**, not a panic — which is how
  rc.18 found `(2, F-1)` and how this run found A2-2's `rc=128`. No arm here asserts a debug fence.
- **No genuine concurrent Task-tool spawn** was driven anywhere in this review. The racers are
  deterministic `git` shims and the fan-outs are single-process — the standing bound M51, M52 and all four
  M53 runs declare.
- **Exit codes.** Every exit code in every axis file was read **bare**. **Six were first read through a
  pipe** (two on axis 2, four on axis 3 — `| head` returning head's `0`), caught, and re-measured; only the
  bare figures are recorded, and each instance is named in its file.
- **Fixtures.** `dev/jigc-rig` only, two-step eval (`rig=$(dev/jigc-rig <state> --binary …) || exit;
  eval "$rig"`), every root from `mktemp -d`, no teardown, and **`rm -rf` on a variable path appears
  nowhere in this review.** The non-rig fixtures are named at their cells and each is a thing a real
  operator's repository can have.
- **Source reads are labelled as source reads.** Where an axis states a mechanism it did not drive (A2-2's
  `posture_subject` carve-out, `(3, C-11)`'s census, `(2, F-2)`'s `subtask_worktrees` classification), the
  file says so at the point of assertion and the **behaviour** is what is claimed.

---

# Files

| file | what |
|---|---|
| [axis-2.md](axis-2.md) | **posture** — the reconciled file, verbatim: the Opus driver's **50-row** `(door, cell)` table over the amend arm's new code (§3), the registry read (§2), the eleven baseline rows re-driven (§4), the `AMBUSH_CONTRACTS` mutation work (§5), the three findings (§6), what was not driven (§7), observations (§8), the M52 §A disposition (§9), what this adds over flow-54/flow-55 (§10), doors covered (§11), instrument honesty (§12) — then the reconciliation: the demotion audit (§A, **20 rows re-driven**), the 17-lead Codex ledger (§B), the three driver defects re-driven (§C), the twenty replacement repros (§D), counts (§E), doors (§F), the reconciler's own honesty (§G) |
| [axis-3.md](axis-3.md) | **destroying doors** — the reconciled file, verbatim: **115 driven rows over 28 doors** across ten sub-tables (§1.1–§1.10) including the new mint door, the amend arm's 22 rows, the 8-leaf `finalize.amend-staged-doc` class and the `AMBUSH_CONTRACTS` mutation rows; the two new findings and the five still-open rc.19 rows (§2); **all eight F-10 review findings verified closed** (§3); 42 repro blocks `R-1`…`R-42` (§4); the baseline comparison (§5); bounds and six instrument faults (§6); what this adds over flow-54 (§7) — then the reconciliation: zero demotions (§0), the 12-lead Codex ledger (§1), the seven driver defects re-driven (§2), three observations (§3), doors (§4), 13 reconciler repros `RX-1`…`RX-13` (§5), totals (§6) |
| [axis-5.md](axis-5.md) | **pinned contracts** — the reconciled file, verbatim: the registry read (§0), the seven baseline rows re-driven (§1–§2), **all 66 `ENVELOPE_ARMS` rows with 65 key sets captured** (§3), the fifteen F-10 cells (§4), the usability-batch surfaces (§5), the two 48-leaf hostile-cwd sweeps (§6), the **216-cell** four-cwd divergence sweep (§7), the two new defects (§8), what was not driven (§9), counts (§10), what this adds over flow-54 and `task_amend::` (§11), doors (§12), notes (§13) — then the reconciliation: the demotion audit with **eight rows independently re-driven** and the partition-count refutation (§A), the **20-lead** Codex ledger (§B), the two driver defects re-driven (§C), doors (§D), the reconciler's notes incl. the undone accidental commit (§E) |
| [codex/](codex/) | the three **unseeded Codex source passes**, verbatim, so every lead is auditable against the verdict it was driven to |
| [instrument/](instrument/) | the instrument — the three Codex prompts and the Workflow script that staffed and ran this fourth partial re-run |

*The three axis files are copied **verbatim** from the reconciled originals, with one added header line
naming the binary and the date. A handful of their internal links were written against the scratchpad and
point at repository paths; they are left as written rather than rewritten, because editing a reconciled
record's body would falsify the word “verbatim”.*

**What this record claims and nothing more:** three axes, driven on the installed `1.0.0-rc.20` on
2026-09-27, **zero tier-1 rows on every one of them**; the usability batch's two target rows `(2, N-1)` and
`(2, N-2)` **CLOSED**, and rc.19's one tier-2/3 row `(3, F-A)` **CLOSED**, each by at least two axes
independently; **all eight F-10 review findings verified closed on the installed release binary**, HIGH-1
over its whole eight-leaf class and MEDIUM-4 by mutation; **seven new finding rows = five distinct
defects, none tier 1** — one tier 2, four tier 3 — of which **three sit inside F-10's own new code** and
**one half-row inside the usability batch's**; **41 Codex leads driven to CONFIRMED, 12 driver defect rows
re-driven and CONFIRMED, 8 open leads, and exactly one refutation — of a count, not a behaviour**; **zero
demotions**; and coverage of **48 of 48 `VERB_KINDS` leaves, all 48 substantive, 36 of them beyond the
pinned-envelope axis, with `task amend` reached by all three**. **The 1.0.0 call is the human's**, and the
exit rule's three clauses — **satisfied** on tier 1, **covered by tier** for all five defects, and **fired
by location** for four of them — are laid out at the headline for that call to be taken against.
