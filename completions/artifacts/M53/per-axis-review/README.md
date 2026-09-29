# M53 — the PARTIAL per-axis review (axes 2 · 3 · 5)

**What this is.** M53's acceptance, and the last act the exit rule names before the 1.0.0 call.
[decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.17 fix pass (M53)* fixes
both the shape and the gate:

> **The exit rule — the human's own gate.** *The 1.0.0 call is taken when a partial re-review of the fix
> pass's affected axes finds no tier-1 row. Tier-2 and tier-3 findings never block the call and are
> triaged into the ledger for 1.x. A finding inside a fix pass's own new code triggers another fix pass
> and a partial re-review, never a full wave.*

> **Acceptance is the three affected axes re-driven (2 · 3 · 5) on the installed `1.0.0-rc.17` with
> M52's re-pointed instrument, not eight.**

So this is a **PARTIAL re-run of M52's instrument — axes 2 · 3 · 5 only**. Axes 1, 4, 6, 7 and 8 were
**not** re-driven, **by design**: the pass fixed four rows, and those rows live on three axes.
M52's record at [M52/per-axis-review/](../../M52/per-axis-review/README.md) is the baseline for the
row-by-row comparison and **is not edited by this run** — it is a dated record of what was driven on
`1.0.0-rc.16`, and this file is a dated record of what was driven on its successor, over a third of
the surface.

This directory is that instrument's **persisted record**: what was driven, what it found, what it
refused to claim, and which of the 47 `VERB_KINDS` leaves the three axes reached. The re-pointed
instrument itself (the three Codex prompts and the Workflow script) is preserved beside it at
[instrument/](instrument/).

**The binary.** Every row in every file here was driven on the installed release
`/Users/maurice/.local/bin/jigc` → **`jigc 1.0.0-rc.17`**, on **2026-09-22**, with the repository at
HEAD **`7e98faf1`** (*"chore(release): 1.0.0-rc.17 — stamped after the M53 audit's seven fixes, not
before"*). Each of the three axis files asserts `jigc --version` **first, before anything else ran**,
and each read its registry counts by symbol at that HEAD. *Stated rather than smoothed:* the binary's
provenance is asserted **by its version string and by the seven audit fixes being visible in driven
behaviour**, not by a rebuild from the sha. The binary is the **release** build, so the debug-only
`debug_assert!` route fences do not exist in it — a route-fence violation shows up here as a *bad
emitted command*, never as a panic, which is the posture an adopter's binary is in. Fixtures were built
with [`dev/jigc-rig`](../../../../dev/jigc-rig) (every root from `mktemp -d`, so nothing needed teardown
and the `rm -rf $V/$D` shape appears nowhere in this review) and by driving the binary. **Nothing was
written into the working repository, no fix was applied, and no commit was made by any agent in this
review** — with one instrument failure on the record rather than buried (axis 2 §9: a rig's stdout was
*parsed* instead of *eval*-ed, two variables came back empty, and three `git` commands ran in the
working repository; nothing was staged or committed, HEAD stayed `7e98faf1`, and the stray branch and
`MERGE_MSG` were removed and re-verified immediately).

**An axis matrix row** is `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted,
verdict}`. A row is **driven** iff its argv ran on that binary. **A verb is covered iff it is the door
of ≥1 driven row.** A classification-only row — a leaf proven by a ⇔ fence rather than by driving —
confers **no** coverage.

---

## THE HEADLINE — the tier-1 count

> # TIER-1 ROWS FOUND: **1**
>
> **`(2, DEFECT 1)` — `jigc milestone finalize` commits a provisioned sub-task worktree's
> un-concluded git operation at exit 0, under jigc's own subject, and destroys that operation's
> authored commit message — while a door run *inside* that same worktree refuses one command earlier.**
>
> **Is it inside M53's own new code? NO — outside it, on the datum below.** The exit rule's clause
> *"a finding inside a fix pass's own new code triggers another fix pass"* is therefore **not** the
> clause this row fires; the clause it does fire is the first one — *the call is taken when the partial
> re-review finds no tier-1 row* — and it finds one.

**The datum for *outside*, stated so it can be checked rather than believed.** M53's own new code is
enumerated in [settle-record.md](../settle-record.md) → *Review amendments* §14 (the corrected ledger):
one finding code (`finalize.foreign-bytes`) · one enum variant (the tenth `InProgress` member) · four
finding producers under shipped codes · one per-member qualifier (the conclude phrase) · one call to a
shipped probe from a new site (`foreign_area_paths` re-read after the unwind) · two widenings by the
human's halt (D3 the residual rule · D1 the `merged/` walk) · **zero** registries, dispositions,
families, gates, envelope keys, `contract-version`s, schema-hashes, `schema-version`s or corpora.
The defect's mechanism — the milestone boundary reading `git diff --cached --binary` out of each
provisioned worktree's index and applying it off-line, with **no** posture probe whose subject is that
worktree — is **none of those rows**. And it **reproduces on the `Merge` member**, which has shipped
since M51 (axis 2 §10.2.1 arm 2), so it is not created by D4's new member and it is not created by D1's
`merged/` walk.

**The honest caveat, carried rather than dropped:** the defect's *door* **is** a door M53 edited
(`milestone finalize` took D1's `merged/` walk and D2's conditioned removal). What the pass added there
governs the area's **byte complement after the commit**; the defect is in the **posture subject before
it**. The row is at a touched door, in untouched code.

**What kind of row it is.** Axis 2 names it *M52's `(2, DEFECT A)` damage shape reappearing on an
un-swept axis — the **subject** axis rather than the **member** axis*: the seventh consecutive
appearance of the complete-fix signature, one layer out. M53's acceptance (flow 54 arm 4) iterates
`GitState::ALL × BEHALF_DOORS`' acting rows with the state induced **in the main checkout**, and an arm
crossing members with doors cannot discover a *second subject*.

**Everything else found is tier 3.** Three rows: axis 3 `F-1` and `F-2`, axis 5 `DEFECT A`. No tier-2
row was minted by this run (the two still-open tier-2 rows are M52's, expected). **Zero data-loss rows
in the classes M53 fixed** — all four of M52's tier-1 rows are CLOSED, each re-driven twice.

---

## The staffing

Per axis, three agents, unchanged from M51 §19 and M52:

- **One Opus driver** owns the `(door, cell)` table. It drives every row on the installed binary and
  records exit, code, route, surface and a repro block per row. **It may not mark a row driven from a
  source read.** Each driver was told not to read its axis's Codex pass, and each says so in its file.
- **One Codex source pass** ([codex/axis-N-source-pass.md](codex/), verbatim, so every lead is
  auditable) owns **completeness of the row set** — the question driving cannot answer: *is there a
  door the registry does not carry, or a bypass of the seam this axis is about?* It reads; it drives
  nothing.
- **One reconciler** (a separate Opus agent, which did **not** author the driver file it reconciles)
  merges the two: it re-drives every driver defect, enters every Codex claim as a lead and drives it,
  and audits the driver's table for rows marked driven that carry no repro block (**demotion pass**).

## The reconciliation rule

> *A claim by one that the other cannot reproduce is a **lead, not a finding**.*

A Codex claim with no driven repro enters the table as `lead(codex, <claim>)` and is either **driven to
a repro block** — at which point it is a finding — or **recorded REFUTED with its falsifying datum**.
An Opus row the source pass says cannot happen **stays a finding** (it was driven), and the source claim
is recorded refuted with the datum. Silence is not refutation.

It earned its keep twice this run, both times against a **completeness** sentence:

- **Axis 2.** Codex's census of production commit/move acts concluded that each carries an immediately
  preceding `verify`. The claim is **true of every act it lists and false of the list**: the milestone
  boundary's apply-a-worktree's-staged-diff path is not in it, and that is exactly where the run's one
  tier-1 row lives. The reconciler refuted the sentence with its own re-drive of the defect, on two
  different `InProgress` members.
- **Axis 3.** Codex's removal-site census concluded *"no unguarded production removal of adopter
  bytes"*. Driven, a third party's `merged/docs/adr:user-authored.md` is destroyed at exit 0, named on
  neither stream. The **behaviour** matches the stated `merged/docs/` membership rule, so it is an
  observation and not a defect — what is refuted is the census sentence's **scope**.

And once in the other direction: **axis 5**'s two Codex leads were both driven to repro blocks, and
both landed on M52 §A rows that were already open — a second, independent derivation of `(5, D1)` and
`(5, C1)` from the source side.

---

## Roll-up

| axis | subject | CONFIRMED | REFUTED | OPEN | new findings (tier) | doors covered (M52 →) |
|---|---|---|---|---|---|---|
| 2 | posture | 10 | 1 | 1 | **1 (tier 1)** | **47** (47) |
| 3 | destroying doors | 17 | 1 | 1 | 2 (tier 3 ×2) | **13** (7) |
| 5 | pinned contracts | 3 | 0 | 1 | 1 (tier 3) | **47** (47) |
| **total** | | **30** | **2** | **3** | **4 — 1 tier-1 · 0 tier-2 · 3 tier-3** | **47 / 47 union · uncovered: none** |

**The counting basis, stated rather than smoothed.** The triples count **ledger entries** — every Codex
claim driven to a verdict, plus every driver defect re-driven — not findings. Axis 2's ten confirmed
are its nine Codex-claim confirmations plus its one driver defect; axis 3's seventeen are fifteen
Codex-claim confirmations plus its two driver defects; axis 5's three are two driven Codex leads plus
its one driver defect. **The auditable set is the enumeration in §FINDINGS below, not this table**, and
where a number here disagrees with a number in an axis file, **the axis file governs**, because it
carries the repro.

*Instrument note on the hand-off figures.* The orchestration that assembled this record carried
per-axis tallies of `confirmed 7 · refuted 1 · open 6` for axis 2, `17 · 1 · 1` for axis 3 and
`3 · 0 · 1` for axis 5. Axis 3's and axis 5's reproduce exactly from their files' own net-position
sections; **axis 2's does not** — its §10.1 ledger enumerates eleven Codex claims (nine CONFIRMED, one
REFUTED, one OPEN LEAD) plus one standing driver defect, which is `10 · 1 · 1`. The table above is the
file-derived count. The discrepancy is recorded rather than reconciled away, because the file is the
evidence and the hand-off figure is a summary of it.

**The three headline facts of the partial re-run:**

1. **All four of M52's tier-1 rows on these axes are CLOSED**, each re-driven with its argv and each
   driven **twice** — once by its axis driver on its own cells, once by its reconciler on an
   independently-proposed regression shape. On axis 3 the Codex-proposed shape reached a cell the
   driver's did not (`.jigc/displaced/<id>` occupied rather than its parent). **Nothing regressed, no
   closed row re-opened, and the new posture member introduced zero false positives** across an
   eight-cell hunt over states that write `MERGE_MSG` or unmerged entries without being an uncommitted
   pick.
2. **One tier-1 row, outside M53's new code** — the headline above.
3. **Seven M52 rows are STILL-OPEN and every one is expected**: one tier-2 and six tier-3, all triaged
   to the 1.x ledger by the charter, none re-tiered and none silently dropped. One is **wider than M52
   reported** (`(5, DEFECT 3)`'s code-less address bail is also at `doc show` and the `doc` write
   verbs, under a second wording), which is recorded on the existing row rather than minted as a new
   finding.

---

# THE ROW-BY-ROW COMPARISON — M52 §A, restricted to axes 2 · 3 · 5

The first deliverable. M52's §A carries **39** rows across eight axes; **12** of them are on axes 2, 3
and 5, and all twelve were re-driven on `1.0.0-rc.17`. **CLOSED** carries the argv that settles it;
**STILL-OPEN** carries the datum. Rows M53 deliberately did not fix — the charter's tier-2/3 triage —
are marked **STILL-OPEN(1.x, expected)**. A row is keyed **(axis, id)** exactly as M52 keys it.

## The four rows M53 was chartered to fix — **4 / 4 CLOSED**

| M52 §A row | tier | verdict on rc.17 | the argv / the datum |
|---|---|---|---|
| **`(3, A3-1)`** — `jigc milestone finalize` removes `.jigc/milestones/<id>/` with `remove_dir_all` and destroys every byte jigc did not write there, at exit 0, named by nothing | 1 (HIGH) | **CLOSED** | `jigc milestone finalize axis-three-probe --format json` over **8 planted loci** (area root · a nested area dir · `merged/top.txt` · `merged/docs/deep.txt` · `merged/docs/provenance.json` · `merged/docs/adr.md` · `merged/sub/nested.txt` · a sub-task area) → exit 0, **8 before / 8 after** by `command grep` with a before-control, every byte at `.jigc/displaced/<unit>/<relative>`, all 8 pairs on `committed.displaced`, two `note:` blocks naming each `<from> → <to>`. Re-driven by the reconciler at Codex's own proposed shape under **both** commit models (`squash: true` and `squash: false`): 4 before / 4 after in each, the foreign directory moved **whole** as one pair. [axis-3.md](axis-3.md) §3 R-A, §8.3-A |
| — its **cell (i)**, `merged/**` at the refusing doors | — | **CLOSED** | `jigc milestone discard axis-three-probe` and `jigc uninstall` over the same plants → exit 1 under `milestone.foreign-bytes` / `uninstall.foreign-bytes`, **all 7** named, nothing taken; `--force` narrates all 7 and then takes them. The settle record's falsifying datum (exit 0, *"workbench removed"*, both `merged/` bytes gone) **does not reproduce**. §3 R-B, R-C |
| — its **cell (ii)**, `materialize` on a boundary that does **not** land | — | **CLOSED** | `jigc milestone finalize` over unauthored commit docs → exit 3 on its own `schema-conformance.*` codes, **6/6 plants on disk**, `.jigc/displaced` not created, zero `unknown-type`; the re-run after authoring clears only the stale `commit:*.md` bodies and displaces all 6. §3 R-F |
| **`(3, A3-2)`** — when the displacement fails, both `Displace` doors remove the area anyway; the bytes are permanently gone at exit 0 | 1 (HIGH) | **CLOSED at both doors** | `jigc task finalize tidy-the-readme --format json` with `.jigc/displaced` a regular file → exit 0, **the area left standing**, before=1 after=1, one advisory `finalize.foreign-bytes` on `findings` naming the path, the count and `Not a directory (os error 20)`, route `Human`. Read-only-`displaced/` cell identical (`Permission denied`). At `milestone finalize`: 3/3 plants survive, one advisory **per standing area**, stderr-only, envelope still `['committed']`. Reconciler re-drove Codex's exact shape (the **per-unit** dir occupied): exit 0, commit landed, both plants standing, exactly **one** finding, `displaced: []`. §3 R-D, R-E, §8.3-B |
| — its **cells (i)–(iii)**, the partial move · the non-atomic removal skeleton · `cleanup_subtask_areas`' ignored `all_gone` | — | **CLOSED** | (i) occupy one entry's parent → one narration block carrying `held 2 entries`, the move it made, the entry it could not, and `File exists (os error 17)`. (ii) driven end to end in R-E's aftermath: the area is left standing **without** its pin and every by-id door then answers `finalize.no-task` with the residual sentence. (iii) two areas settle → **two** `finalize.foreign-bytes`, one per area, keyed `milestone:…` and `task:…`. §3 R-D, R-E, R-H |
| **`(2, DEFECT A)`** — a clean `git cherry-pick --no-commit` is a member of no `InProgress::ALL` row, and `jigc task finalize` concludes it at exit 0, destroying the picked commit's authored message | 1 (by consequence) | **CLOSED** | `dev/jigc-rig committed-singletons --git-state uncommitted-pick` (and `-range`, `-conflicted`, `-resolved`): all **four** answer `repo.operation-in-progress — an uncommitted cherry-pick is in progress`, driven at **all 12** acting `BEHALF_DOORS` rows, exit 1 each; after the full 12-door sweep HEAD, `.git/MERGE_MSG` and the index are **byte-identical**. **Both orderings** closed: with the pick pre-dating the task, `--carry-staged` now refuses first, so rc.16's second exit-0 path is gone. The route's **two arms were run**, not read: `git commit` lands the pick's own subject (`posture fixture: the picked commit`) and exits 128 on a conflicted pick; `git reset` leaves both the picked file and an unrelated staged file on disk, unstaged. [axis-2.md](axis-2.md) §3.4–§3.9, §10.1.3 |
| **`(5, DEFECT 1)`** — two minting doors accept a title that slugs to nothing and **commit** a record at a fabricated `<ty>:<ty>` identity at exit 0 | 1 (repository harm) | **CLOSED** | `jigc milestone create` / `milestone add-task` / `milestone add-from-spec` / `jigc start --workflow` over `{"" · "   " · "!!!" · "日本語" · "the of a"}` → **13 degenerate-mint cells, exit 1 each**, `write.unslugable-title` (five titles at `milestone create`, the sibling doors over a subset); `git rev-parse HEAD` unmoved, `git status --porcelain` empty, `.jigc/tasks/` absent. **Re-driven by the reconciler with the HEAD isolation tightened** — the driver's transcript interleaved a *successful* mint, which does move HEAD, so the reconciler split the segments and measured the unmoved-HEAD claim over refusals only; the claim holds. [axis-5.md](axis-5.md) §2.1, §14.3.e |

## The eight rows M53 deliberately did not fix — **1 CLOSED · 7 STILL-OPEN(1.x, expected)**

| M52 §A row | tier | verdict on rc.17 | the argv / the datum |
|---|---|---|---|
| **`(3, A3-3)`** — `milestone_boundary_displacement`'s subject is the sub-task areas only, so the boundary's own area is untested | 3 (LOW) | **CLOSED** (behaviourally) | The behaviour A3-3 said was untested is driven green at the boundary's **own** area and at the move-failure cell. The suite half is a test-content fact, not a drive: `milestone_boundary_displacement.rs:444` carries `a_landed_milestone_boundary_keeps_every_byte_of_its_own_area_it_did_not_write`, and the file mentions `milestones` **8** times against M52's driven `grep -c → 0`. §6, §8.2 C4 |
| **`(2, DEFECT C)`** — a posture breach raised at the **commit seam** prints no state-truth clause and no copy-runnable re-run, while every other in-transaction failure at the same door prints both | 2 | **STILL-OPEN(1.x, expected)** | Same rig, same shim racer. The posture arm prints `blocking · repo.operation-in-progress — a bisect is in progress` + one `route:` line **and nothing else**, while the state it does not state is verified on disk (HEAD unmoved, 1 active task, `work.txt` still `A` in the index, jigc's own staging rolled back). The reconciler added **ARM A′** — a *git* refusal (a missing `gpg.program`) — which neither pass had driven: it prints `` `git commit` failed (no commit was made): `` **plus** the full state-truth clause and the copy-runnable `jigc task finalize <id>`. Three in-transaction causes at one seam; two print the frame, the posture one does not. §5, §10.1.2 |
| **`(5, DEFECT 2)`** — `config remove-step` / `replace-step` refuse a step that **is** in the resolved include list | 3 | **STILL-OPEN(1.x, expected)** | The resolved include list prints `probe-step` one command earlier; `remove-step` → `config.anchor-absent — no step \`probe-step\` body to fork`, route *"name a step id present in the workflow's resolved include list"* — already satisfied. `replace-step` now answers a **different** wrong code, `config.step-id-collision`. §2.2 |
| **`(2, DEFECT B)`** — a conflicted `git merge --squash` is answered by `SquashMerge`, whose predicate is false of the state | 3 | **STILL-OPEN(1.x, expected)** | `git merge --squash cb` (conflicting) → `.git` holds `MERGE_MSG` + `SQUASH_MSG`, `git ls-files -u` → **3**; `jigc milestone create CS1` → exit 1, *"a squash merge is **staged** and not committed"*. The conclude arm is also command-less here, and the `git commit` it elides would exit 128 on this state. The route is still effective. §3.14, §5, §10.1.1 |
| **`(5, DEFECT 3)`** — `jigc rename`'s bare-slug refusal is outside `RefusalKind` and carries no code | 3 | **STILL-OPEN(1.x, expected), and wider than reported** | `jigc rename vision --to "New Vision"` → `{"error": "\`vision\` is not a \`<type>:<slug>\` address …"}`, no `blocking · <code>` prefix. The same code-less bail is at **`doc show` and the `doc` write verbs**, under a second wording — recorded on this row, not minted as a new finding. §2.3, §10.4 |
| **`(5, DEFECT 4)`** — `doc show` blocks a **declared but unpopulated** optional leaf with `store.no-such-leaf` | 3 | **STILL-OPEN(1.x, expected)** | `doc schema vision` advertises `vision:<slug>#meta/grounded-in` `required: false`; `doc show vision:vision#meta/grounded-in` → exit 1, `store.no-such-leaf`, *"names no leaf `grounded-in` in section `meta`"*, against a control `#meta/schema-version` → exit 0. §2.4, §14.1 |
| **`(5, C1)`** — `start`'s two orientation rows declare `next_steps`, and a reachable composition omits it | 3 | **STILL-OPEN(1.x, expected), both arms** | `fresh --pack-from-dev` with the one remaining off-catalog workflow **moved** out (a `mv`, no `rm`): `Clean` driven keys `['header','schema_version','state','workflows']`, `ActiveTask` `['header','schema_version','state','tasks','workflows']`; both declared sets carry `next_steps` (`render.rs:6079-6085`, `:6096-6103`). Independently re-derived from the source side by Codex. §2.5, §14.1 |
| **`(5, D1)`** — `jigc start --explain` emits a production `--format json` stdout arm `ENVELOPE_ARMS` does not carry | 3 | **STILL-OPEN(1.x, expected)** | exit 0, stdout 1218 bytes, stderr 0, top-level keys `['collision_winners','overrides_applied','pack_inputs','schema_version','steps','workflow','workflow_layer']`; `ENVELOPE_ARMS` carries exactly four `start` rows and none is it. **Widened by the reconciler:** the **bare** `jigc --format json start --explain` emits the identical unregistered shape. §2.6, §14.1 |

## The M51 rows these axes carried forward

Re-driven here so a regression could not hide behind a partial run.

- **Axis 2** — `D1` (rebase/bisect answer the operation, not the detachment) **still CLOSED**, driven
  across all fifteen git states at a commit door · `D2` (`am` vs `rebase --apply`) **still CLOSED**,
  both directions · `D3` **still CLOSED**, including M52's formerly-open clean-`-n` cell · `D3b` (a git
  refusal not dressed as a hook rejection) **CLOSED** — the reconciler's ARM A′ drove it · `codex-1`
  (the `relocate` seam re-probe) — **OPEN LEAD**, unraced by either pass (§C below).
- **Axis 3** — `C-1` (the six-door registry complete) · `D-1` (milestone-owned task routing) · `D-2`
  (`cleanup_subtask_areas` aggregating real removal success) · `D-3` (repo-relative rendering at the
  probe failures, `command grep -c '/var/folders'` → 0) · `D-4` (the unreadable-worktrees route naming
  a `read_dir` fault, not a git one) — **all five still CLOSED, all re-driven**.
- **Axis 5** — M51's `DEFECT A` (setup/uninstall's undeclared envelope) **still CLOSED**, 47/47 in a
  declared arm · `DEFECT B` (`doc show`'s undeclared shapes) **still CLOSED**, 8 registry rows + 9
  deeper dispatch cells, 0 undeclared shapes · `DEFECT C` **still CLOSED by declaration** · `DEFECT D`
  (the pre-dispatch `current_dir()` bypass) **still CLOSED**, 47/47.

---

# FINDINGS

## A · CONFIRMED — every new finding, tiered on the charter's predicate

The predicate, quoted: **tier 1** = exit-0 loss or repository harm through a committing, destroying or
moving door · **tier 2** = a posture or route dead end · **tier 3** = a surface says something the
binary does not do.

### Tier 1 (1)

#### `(2, DEFECT 1)` · origin **driver**, re-driven by the reconciler on two members · **outside M53's own new code**

`jigc milestone finalize` commits a provisioned sub-task worktree's **un-concluded git operation** at
exit 0, under jigc's own subject, destroys the operation's authored message, and narrates the swallowed
payload as the sub-task's own work — while the *same* worktree refuses its own `task finalize` one
command earlier.

**Contracts violated.** `crates/cli/src/repo.rs`'s module header (*"the probe is the family's one home
so the doors that read it cannot each hand-enumerate a different three"* — the boundary reads the probe
for the **main checkout** and commits from a working tree it never asks about) · `design/finalize.md`
→ *Preflight* (*"No in-progress merge/rebase/bisect"*) · `BEHALF_DOORS`, where `milestone finalize` is
`CommitsOnBehalf` and every acting door refuses under every member · and law 1, because the ack calls
the swallowed bytes *"sub-tasks: `<sub>`: 2 code files"*.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"
       jigc milestone create "Swallow milestone"
       jigc milestone add-task swallow-milestone "swallow sub intent"
       jigc milestone provision swallow-milestone
       W=$REPO/.jigc/worktrees/swallow-sub-intent
       printf 'sub work\n'       > $W/subwork.txt; git -C $W add subwork.txt   # the sub-task's own work
       printf 'PICKED PAYLOAD\n' > $W/picked.txt;  git -C $W add picked.txt
       git -C $W commit -m 'the users authored pick message'; P=$(git -C $W rev-parse HEAD)
       git -C $W reset --hard HEAD~1
       printf 'sub work\n' > $W/subwork.txt; git -C $W add subwork.txt
       git -C $W cherry-pick -n "$P"                     -> rc=0

BEFORE (measured, not assumed):
  head -1 .git/worktrees/swallow-sub-intent/MERGE_MSG  -> the users authored pick message
  git -C $W diff --cached --name-only                  -> picked.txt  subwork.txt
  git -C $REPO status --porcelain                      -> (empty)     <- the MAIN checkout is clean
  ls .git | grep -E 'MERGE_MSG|MERGE_HEAD|CHERRY_PICK_HEAD'  -> (none) <- and carries no marker
  (cd $W && jigc task finalize swallow-sub-intent)
     -> exit 1  blocking · repo.operation-in-progress — an uncommitted cherry-pick is in progress

argv : jigc milestone join     swallow-milestone        -> exit 0, "0 doc(s) merged"
       jigc milestone finalize swallow-milestone

observed:
  finalized 19d3b94 — Finalize milestone swallow-milestone (1 sub-task)
    modified docs/milestone-records/swallow-milestone.md
    added picked.txt                      <- the USER's un-concluded pick
    added subwork.txt
    3 files committed
    sub-tasks: swallow-sub-intent: 2 code files
  exit=0

AFTER:
  git cat-file -p HEAD:picked.txt              -> PICKED PAYLOAD
  git log --all --oneline                      -> the pick's own commit is NOWHERE
  .git/worktrees/swallow-sub-intent/MERGE_MSG  -> No such file or directory
     i.e. "the users authored pick message" is destroyed and recoverable from no git object
```

**The class, not the member** — the identical drive with a `git merge --no-commit --no-ff` left
un-concluded in the sub-task worktree lands `merged.txt` in the boundary commit at exit 0 and takes
`MERGE_HEAD` + `MERGE_MSG` with it. Two members, two detection styles (marker-negated and
marker-backed), one shape: **the door that commits a worktree's index never asks that worktree's
posture.**

**Zero-false-fire control, driven:** the same fan-out with no operation anywhere finalizes clean —
exit 0, `2 files committed`, `sub-tasks: clean-sub-intent: 1 code file`. The defect is the *subject*,
not the door.

**Bound on the finding, carried by both passes:** the **non-squash** commit model
(`config set finalize.fan-out.squash false`) is **not driven to a landed commit** — two attempts
stopped earlier at `finalize.render-io`. It is recorded **un-driven, not passing**. Second bound: neither
arm was driven with the operation's *own conflict* unresolved in the worktree.

Full row: [axis-2.md](axis-2.md) §4, §10.2.1.

### Tier 2 (0)

None minted by this run.

### Tier 3 (3)

#### `(3, F-1)` · origin **driver**, re-driven · a **symlink** wearing a staged identity is jigc's own at the read and ack surfaces and a third party's at the same door's destroying probe

`engine::state::foreign_area_paths` reads entry shape with `DirEntry::file_type()` (**no-follow**);
`crate::task::staged_doc_ids` (`crates/cli/src/task.rs:716`) reads it with `std::fs::metadata`, which
**follows**. M52 Increment 4 / T1's shape fence closed the **directory** axis of this class and left the
**symlink** axis open.

```
setup: mkrig task; OUT=$(mktemp -d …); TARGET-BYTES > $OUT/body.md
       ln -s "$OUT/body.md" .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md
       mkdir -p             .jigc/tasks/tidy-the-readme/docs/adr:via-directory.md

$ jigc task discard tidy-the-readme      -> exit 1
  blocking · task-discard.foreign-bytes — … holds 2 path(s) jigc did not write:
    .jigc/tasks/tidy-the-readme/docs/adr:via-directory.md
    .jigc/tasks/tidy-the-readme/docs/adr:via-symlink.md        <- FOREIGN here
$ jigc doc list --task tidy-the-readme   -> exit 0
  adr:via-symlink  docs/decisions/via-symlink.md  managed      <- JIGC'S OWN here
$ jigc uninstall                         -> exit 1  uninstall.foreign-bytes, the same 2 paths
$ jigc task discard tidy-the-readme --force -> exit 0
  stdout  discarded task tidy-the-readme — dropped staged edits to: adr:via-symlink, …
                                            ^ the entry the same command just called foreign
  outside tree after: body.md :: TARGET-BYTES                  # 1/1 intact
```

**Why tier 3 rather than tier 1, driven rather than assumed:** the promotion plan is on the engine's
side of the split, so a symlinked `adr:outside-secret.md` was **not** promoted and was displaced
instead; and `remove_dir_all` unlinks a symlink rather than descending it, so the target tree is
byte-intact after `--force`. The guard **over-claims**, which is the safe direction.
[axis-3.md](axis-3.md) §3 R-P, §5 F-1, §8.4.

#### `(3, F-2)` · origin **driver**, re-driven · the residual note asserts a shape it did not check

`engine::state::residual_area_note` hard-codes *"`{listed}` is a **directory** carrying no base pin"*.
The predicate the door actually asked is `carries_base_pin`, which says nothing about the area's shape.

```
setup: OUT=$(mktemp -d …); OUTSIDE2-KEEP > $OUT/keep.txt
       mkdir -p .jigc/tasks; ln -s "$OUT" .jigc/tasks/ghost      # a SYMLINK
$ jigc task discard ghost           -> exit 1
  blocking · finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` is a directory carrying no base
    pin, so it is a leftover and not a work unit …
                                  ^^^^^^^^^ it is not a directory
$ jigc task discard ghost --force   -> exit 1, byte-identical sentence
  outside tree: keep.txt :: OUTSIDE2-KEEP                        # nothing destroyed
CONTROL: mkdir .jigc/tasks/ghost-dir; jigc task discard ghost-dir -> the sentence is true there
```

[axis-3.md](axis-3.md) §3 R-Q, §5 F-2, §8.4.

#### `(5, DEFECT A)` · origin **driver**, re-driven · `store.unknown-type` emits a **URI-shaped** target at `jigc doc show` where the contract fixes the bare doctype id

```
rig: committed-singletons
$ jigc --format json doc show 'nosuchtype:x'
  findings[0].key -> {"code":"store.unknown-type","target":"nosuchtype:x"}      <- URI-shaped
$ jigc --format json doc show 'nosuchtype:x#meta'                 -> target "nosuchtype:x"
$ jigc --format json doc show 'nosuchtype:x' --task <t>           -> target "nosuchtype:x"

the same code, same condition, NINE sibling producers, all BARE:
  doc schema · doc set-field · doc set-slot · doc add-item · doc remove-item · doc retitle-item ·
  doc rename · rename · relocate · migrate                        -> target "nosuchtype"
```

`design/command-output-contract.md:237` fixes the form **and names `doc show`'s own read path as the
precedent for it** — *"It is deliberately **not** URI-shaped — a bare id cannot be mistaken for a doc
address a driver could `doc show`."* Driven, the one door that sentence cites is the one door that does
not obey it, and the key it emits is exactly the doc-address shape the sentence forbids. Cost to a
driver: `(code, target)` is the contract's stable per-instance identity, so an acknowledge-ledger sees
two keys for one condition. Exit 1 at a read door, no write, no loss.
[axis-5.md](axis-5.md) §7.2, §11, §14.2.

## B · REFUTED claims — each with its falsifying datum (2)

| # | claim | pass | falsifying datum |
|---|---|---|---|
| `(2, C-10)` | *"no production HEAD-changing `git switch`/`checkout` was found, and the enumerated production acts (`git commit`, the setup commit, `git merge --ff-only`, `git mv`, `git rm --cached`) each carry an immediately preceding `verify`"* — as a **completeness** claim | codex | The enumeration is **incomplete**: the milestone boundary's commit act reads `git diff --cached --binary` out of a *provisioned sub-task worktree's* index and applies it off-line, and **no `verify` probes that worktree**. The datum is the reconciler's own re-drive of `(2, DEFECT 1)`: main checkout clean, operation in the worktree, `jigc milestone finalize` commits its payload at exit 0. **The claim is true of each listed act and false of the list.** |
| `(3, C15)` | *"I found **no unguarded production removal of adopter bytes"*** — as worded | codex | A third party's `merged/docs/adr:user-authored.md` (`USER-BODY-KEEP`) is **destroyed at exit 0**, named on **neither** stream (`grep -c 'user-authored'` → 0 on stdout *and* stderr), absent from the repo and from git (`git grep -l … HEAD` → rc 1), while its sibling `plain.txt` is displaced — at the very site the census lists as benign (`engine/milestone.rs:2228`). **This does not make it a defect**: it matches the *stated* `merged/docs/` membership rule (`staged_doc_id` alone), whose trade its own doc-comment prices, and M52's conversion ledger pins the behaviour as expected output. What is refuted is the census sentence's **scope**, not the behaviour's correctness. |

**Nothing either driver claimed was refuted.** Both refutations are of source-pass **completeness**
sentences, and axis 5 refuted nothing at all.

## C · OPEN leads — driven as far as the state allows, promoted by nothing (3)

| # | lead | pass | why it stays open |
|---|---|---|---|
| `(2, C-8)` | M51's `codex-1` — `displace_foreign_squatter` calls `verify(Move)` immediately before `git rm --cached`, and the later `git mv` re-probes | codex | Driving it needs a racer positioned **inside the mover seam**; the reconciler's shim fires on the first `git add` of a run, which the relocate path does not necessarily reach in that window, and the driver did not re-race it either. **Neither pass drove it, so it becomes neither a finding on a source read nor a closure on one.** Standing mitigation, stated rather than relied on: M52 closed these two sites with repros and nothing in M53 touches `relocate.rs`'s seam. |
| `(3, C17)` | the per-site classifications of the ~25-entry removal census (each named `file:line` read as owned / guarded / verified-empty) | codex | Not driveable per site through the binary: most sites are internal cleanup with no caller-reachable cell that distinguishes *guarded* from *never exercised*, and the rig builds no state that reaches them individually. The **aggregate** claim is driven (and refuted as worded, §B). |
| `(5, census)` | *"the sole uncovered stdout producer is `render::explain`"* | codex | **A negative universal over all production stdout producers is not establishable by driving a finite argv set**, and the claim is explicitly a source read. Its *positive* half is confirmed (it is `(5, D1)`). The driven evidence that bears on it is recorded rather than spent: two uniform 47-leaf sweeps (94 cells, 0 NOT-JSON, 0 undeclared shapes), 10 nested `doc show` depths inside the six declared `ArmShape` members, and a 14-cell alternate-run-mode sweep — every cell inside a declared arm except the two `--explain` cells. That is evidence, not proof. |

## D · Observations driven and carried — **not** defects

Recorded so a next reader does not re-drive them as findings. Each is graded against a written
declaration, quoted in its axis file.

- **`(3, OBS-1)`** — at `merged/docs/`, a third party's file named `<ty>:<slug>.md` is destroyed at
  exit 0, on no stream. Matches the stated membership rule; the loss **cell** is stated on no
  user-facing surface, which is a 1.x ledger row, not a blocker. (This is §B's falsifying datum.)
- **`(3, OBS-2)`** — `uninstall.foreign-bytes`' route always names `.jigc/displaced/`, even when no
  listed path is under it. Every clause is true; misdirection, not a lie.
- **`(3, OBS-4)`** — the settle record's §6 motivating cell (*a succeeding hook writes into the area
  during the commit, where no probe can see it*) is **not reachable with a git hook on rc.17**: the
  displacement runs after `git commit` returns, so the byte is parked and the area unwinds clean. The
  design decision §6 reached is unaffected; only the cell's stated reachability is narrower than
  written.
- **`(3, OBS-6)`** — `milestone finalize` lands at exit 0 over unauthored sub-task commit docs under
  the default `squash: true`. This is `d854e25`'s **stated carve-out**, and the gate does bind under
  `squash: false` (driven: exit 3, four findings). Recorded because the un-driven reading of the
  fold-back sentence looks like a regression and is not.
- **`(3, OBS-3/5/7)`** and **axis 5's six observations** (the flattened `write.unslugable-title` arm ·
  the two `serial-collision` codes flattening beside enveloping siblings · the stderr-only
  `milestone finalize` advisory · the widened code-less address bail · `uninstall` refusing over
  `.jigc/displaced/` · `doc show --task`'s `staged` key) are each declared somewhere and graded
  against, not re-found.

---

# COVERAGE — the three axes against the 47 `VERB_KINDS` leaves

**The leaf count, read rather than quoted.** `crates/cli/src/cli.rs:1834`'s `VERB_KINDS` carries
**47** leaf rows at HEAD `7e98faf1` — **35 `VerbKind::Write` · 12 `VerbKind::Read`** — extracted by
reading the `(&[…], VerbKind::…)` rows of the const itself. The spellings and their order are identical
to M52's and M51's tables, so the columns below are directly comparable row by row.

**What the table reports.** Which of the **three re-driven axes** reached each leaf as the door of ≥1
**driven** row (the union of the three files' *Doors covered* sections), against the same column from
M52 **restricted to axes 2 / 3 / 5**. `uncovered` is the set of leaves no re-driven axis reached.

> **`uncovered`: NONE — and this is the run's one result that contradicts its own brief.** The
> assembly brief stated that a partial run's `uncovered` set *will* be non-empty and that 47/47 must
> not be claimed. Driven, it is empty, for a reason that is a property of these two axes and not a
> grading choice: **axis 2 drove all 35 `BEHALF_DOORS ▸ Neither` leaves as controls** under the
> `uncommitted-pick` breach (34 silent, `task validate` answering by design), and **axis 5's cell set
> is two uniform 47-leaf sweeps** (outside-a-repo and deleted-cwd) in which every leaf is a door by
> construction. Two of the three re-driven axes therefore each reach all 47 on their own. The brief's
> expectation is falsified by the files; the number is reported as measured, and the **stricter**
> reading it was reaching for is reported beside it in the last column.

**The stricter column, because 47/47 here is weaker than 47/47 in M52.** A leaf is **substantive** in
this run iff some axis drove it in a **condition-specific** row — a posture cell at an acting door, a
destroying-door cell, or a success/condition arm — rather than only as a silent control or a uniform
fault sweep. **43 of 47 are substantive; 4 are not**: `ingest` · `upgrade` · `doc author` ·
`config fork`, each of which is reached by axis 2's `Neither` control sweep, axis 5's two uniform
sweeps, and one axis-5 registry arm that axis 5's own demotion pass marks **construction-backed rather
than per-row evidenced**.

| # | leaf | M53 axes (driven) | M52 axes, restricted to 2/3/5 | Δ | substantive |
|---|---|---|---|---|---|
| 1 | `start` | 2, 3, 5 | 2, 5 | **+3** | ✔ |
| 2 | `workflow` | 2, 5 | 2, 5 | — | ✔ |
| 3 | `setup` | 2, 5 | 2, 5 | — | ✔ |
| 4 | `uninstall` | 2, 3, 5 | 2, 3, 5 | — | ✔ |
| 5 | `upgrade` | 2, 5 | 2, 5 | — | control-only |
| 6 | `ingest` | 2, 5 | 2, 5 | — | control-only |
| 7 | `migrate` | 2, 5 | 2, 5 | — | ✔ |
| 8 | `migrate-corpus` | 2, 5 | 2, 5 | — | ✔ |
| 9 | `unmanage` | 2, 5 | 2, 5 | — | ✔ |
| 10 | `rename` | 2, 3, 5 | 2, 5 | **+3** | ✔ |
| 11 | `relocate` | 2, 5 | 2, 5 | — | ✔ |
| 12 | `describe` | 2, 5 | 2, 5 | — | ✔ |
| 13 | `validate` | 2, 5 | 2, 5 | — | ✔ |
| 14 | `doc create` | 2, 5 | 2, 5 | — | ✔ |
| 15 | `doc add-item` | 2, 5 | 2, 5 | — | ✔ |
| 16 | `doc remove-item` | 2, 5 | 2, 5 | — | ✔ |
| 17 | `doc retitle-item` | 2, 5 | 2, 5 | — | ✔ |
| 18 | `doc rename` | 2, 5 | 2, 5 | — | ✔ |
| 19 | `doc set-field` | 2, 5 | 2, 5 | — | ✔ |
| 20 | `doc set-slot` | 2, 5 | 2, 5 | — | ✔ |
| 21 | `doc author` | 2, 5 | 2, 5 | — | control-only |
| 22 | `doc show` | 2, 5 | 2, 5 | — | ✔ |
| 23 | `doc schema` | 2, 5 | 2, 5 | — | ✔ |
| 24 | `doc list` | 2, 3, 5 | 2, 5 | **+3** | ✔ |
| 25 | `task list` | 2, 3, 5 | 2, 5 | **+3** | ✔ |
| 26 | `task diff` | 2, 5 | 2, 5 | — | ✔ |
| 27 | `task validate` | 2, 5 | 2, 5 | — | ✔ |
| 28 | `task discard` | 2, 3, 5 | 2, 3, 5 | — | ✔ |
| 29 | `task finalize` | 2, 3, 5 | 2, 3, 5 | — | ✔ |
| 30 | `task bind` | 2, 5 | 2, 5 | — | ✔ |
| 31 | `config set` | 2, 5 | 2, 5 | — | ✔ |
| 32 | `config insert-step` | 2, 5 | 2, 5 | — | ✔ |
| 33 | `config replace-step` | 2, 5 | 2, 5 | — | ✔ |
| 34 | `config remove-step` | 2, 5 | 2, 5 | — | ✔ |
| 35 | `config fill` | 2, 5 | 2, 5 | — | ✔ |
| 36 | `config fork` | 2, 5 | 2, 5 | — | control-only |
| 37 | `config get` | 2, 5 | 2, 5 | — | ✔ |
| 38 | `config list` | 2, 5 | 2, 5 | — | ✔ |
| 39 | `milestone create` | 2, 3, 5 | 2, 5 | **+3** | ✔ |
| 40 | `milestone add-task` | 2, 5 | 2, **3**, 5 | **−3** | ✔ |
| 41 | `milestone add-from-spec` | 2, 5 | 2, 5 | — | ✔ |
| 42 | `milestone list-tasks` | 2, 3, 5 | 2, 5 | **+3** | ✔ |
| 43 | `milestone provision` | 2, 3, 5 | 2, 3, 5 | — | ✔ |
| 44 | `milestone execute` | 2, 3, 5 | 2, 5 | **+3** | ✔ |
| 45 | `milestone join` | 2, 5 | 2, 5 | — | ✔ |
| 46 | `milestone finalize` | 2, 3, 5 | 2, 3, 5 | — | ✔ |
| 47 | `milestone discard` | 2, 3, 5 | 2, 3, 5 | — | ✔ |

**`uncovered(<reason>)`: EMPTY.** Every one of the 47 leaves is the door of ≥1 driven row in ≥2 of the
three re-driven axes.

**Leaves that lost coverage against M52's restricted table: ONE, and it is an axis-internal move, not
a hole.**

- **`milestone add-task` — lost axis 3** (M52: axes 2, 3, 5 → M53: axes 2, 5). The reason is stated in
  axis 3's own coverage section: on this run `milestone add-task` is **rig setup only** and is the door
  of no reviewed row, so listing it would read as coverage it does not have. The leaf keeps axis 2 (all
  twelve acting-door posture cells, and the degenerate-mint refusal) and axis 5 (the `["hook_output",
  "text"]` registry arm, re-driven by the reconciler), so **no leaf is left with fewer than two
  axes**.

**Leaves newly reached (7), all by axis 3, whose door set grew 7 → 13:** `start` · `rename` ·
`doc list` · `task list` · `milestone create` · `milestone list-tasks` · `milestone execute` — the
residual rule (D3) and the `merged/` walk (D1) gave axis 3 doors it did not have on rc.16.

**Per-axis door counts as each file states them:** axis 2 **47 / 47** (12 acting + 35 `Neither`
controls) · axis 3 **13** (the six `DESTROYING_DOORS` members + seven residual/enumerating doors) ·
axis 5 **47 / 47** (43 with a condition-specific row beyond the two uniform sweeps).

**The five axes NOT re-driven — 1, 4, 6, 7, 8 — are not represented in this table at all.** A leaf's
row above says what axes 2, 3 and 5 reached on rc.17; it says nothing about the caller-token, rollback,
composed-surface, freeze/migration or adopter-docs axes, which were last driven on `1.0.0-rc.16` and
whose M52 rows stand unchanged.

---

# HONEST BOUNDS — what each axis states it did **not** drive

Each axis file carries its own section; this is the roll-up, not a replacement.

**The bound that covers the whole record: this is a PARTIAL run.** Axes **1 (caller tokens) · 4
(transaction / rollback) · 6 (composed surfaces) · 7 (freeze & migration) · 8 (adopter docs & help)
were NOT re-driven, by design** — the charter's acceptance is *"the three affected axes re-driven
(2 · 3 · 5) … not eight"*. M52's rows on those five axes were not re-checked on rc.17. Nothing here
says they held; it says they were not asked.

**Axis 2 · posture** ([axis-2.md](axis-2.md) §6, §12)

1. `(2, DEFECT 1)`'s **non-squash** commit model is **un-driven to a landed commit** by both passes
   (two attempts stopped at `finalize.render-io`) — recorded un-driven, not passing.
2. `(2, DEFECT 1)` was **not** driven with a genuinely conflicted sub-task index.
3. **`GIT_DIR` redirect at 11 of the 12 acting doors** — driven at `milestone create` only, and the
   row is the family's **declared-out** bound with a written reopening condition (driven: a door run
   in repo A with `GIT_DIR` pointing at B concludes B's pick at exit 0).
4. The **`relocate` / `config set placement-root` seam re-probe** was not re-raced (§C's open lead).
5. `SeamSubject::verify`'s identity-drift and expected-ref-drift legs — not driven; only the posture
   leg is this axis's.
6. **A genuine concurrent process** rather than a deterministic shim racer at any seam — not drivable
   here, the standing headless bound.
7. `InProgress` states inside a sub-task worktree **other than** merge and uncommitted-pick.
8. Non-`main` default branch names, submodule worktrees, `core.worktree` redirects, git versions other
   than the installed one — git's on-disk marker contract is the family's existing declared deferral.
9. The 34-silent half of the `Neither` control sweep rests on the **driver's** run alone; the
   reconciler re-drove only the one leaf that answers.
10. The invocation-log row and the `GIT_DIR` row were not re-driven by the reconciler.

**Axis 3 · destroying doors** ([axis-3.md](axis-3.md) §4, §8.5)

1. `task finalize` / `milestone finalize` × `--force` — **the flag does not exist** at either leaf; an
   empty arm, not a skipped one. Likewise `Disposition::Narrate`, which no member holds at rc.17.
2. `milestone discard` / `uninstall` × a `File` at a **registered** worktree path — unreachable
   (`git worktree add` cannot register a non-directory).
3. Any cell × `OwnWorktree` × `Unreadable` — empty by construction.
4. The record-commit rejection cells at `milestone discard` / `task discard` (axis 4's) and
   `InProgress::ALL` × the destroying doors (axis 2's).
5. **A genuine concurrent racer** against a displacement or teardown — headless by construction.
6. `--ignored` build output as a **refusal** — M46's measured *visible, not prevented* decision; its
   narration is driven.
7. A second concurrent `.jigc/displaced/<id>` writer producing a `.3`/`.4` suffix
   (`free_displacement_path`'s unbounded loop is a declared 1.x row); an undecodable filename (the
   declared `task.rs:5845` bound).
8. **Graded against, not re-found:** the stderr-only advisory at `milestone finalize`, the 21-render
   `DEBUG_REMAINDER`, `remove_worktrees`' two `DeclaredAbsolute` host-path warnings,
   `reseed_sub_task_areas`' `.exists()` skip, a foreign directory inside `merged/` moving whole, the
   residual cleared by hand.
9. One **table correction** on the record: row 56's observed codes are `conformance.section-missing`
   (×2), not `schema-conformance.*`; the row's verdict is unaffected. **Zero demotions** — all twenty
   block-less rows were re-driven and all twenty reproduced.

**Axis 5 · pinned contracts** ([axis-5.md](axis-5.md) §9, §14.4)

1. **`STORE_EXIT_FLIPS` at 5 of its 7 members** (`probe-unreliable`, `unmigrated-corpus`,
   `ahead-corpus`, `orphaned-instance`, `foreign-squatter`) — axis 7's fixture work; what this axis
   owes (key-set invariance under the flip) is driven at the two reachable members.
2. `ENVELOPE_OWED_CODES` at `task bind` and `milestone add-from-spec` — both answered an earlier guard
   on every buildable argv; **not presented as driven**.
3. `RelocateRefusal::ALL` / `RefusalKind::ALL` member-by-member; `ManifestKind::ALL` and
   `SchemaChangeKind::ALL × LOCI` (they shape values *inside* an envelope, not its key set).
4. `DESTROYING_DOORS` at `milestone provision` (axis 3's row); `ENVELOPE_ARMS` under a project-layer
   pack shadow (axis 7's); the `hook_output` **value** across the whole producer axis (axis 4's).
5. **A genuine concurrent / Task-tool fan-out envelope** — both `milestone finalize` arms were driven
   single-process through real provisioned worktrees.
6. **Four driver rows were demoted** for carrying no repro block; all four were re-driven and all four
   hold — one carried a real counting error (14 reported, **15** enumerated, 15 driven). **The 42
   un-spot-checked `ENVELOPE_ARMS` rows of §3 stay construction-backed, not per-row evidenced.**

**Bounds that bind all three.** The **release posture** means a route-fence violation shows up as a bad
emitted command rather than a panic — every drive here is under that posture, and no arm asserts the
debug fences. **No genuine concurrent Task-tool spawn** was driven anywhere in this review; the
racers are deterministic shims and the fan-outs are single-process, which is the standing bound M51,
M52 and M53 all declare.

---

# Files

| file | what |
|---|---|
| [axis-2.md](axis-2.md) | **posture** — the reconciled file: the Opus driver's `(door, cell)` table (§1–§9), the reconciliation ledger (§10), the reconciled doors-covered list (§11) and the reconciler's method and bounds (§12) |
| [axis-3.md](axis-3.md) | **destroying doors** — the driver's 78 rows over 13 doors (§1–§7), the reconciliation ledger (§8) and doors covered (§9) |
| [axis-5.md](axis-5.md) | **pinned contracts** — the driver's ≈303 rows over the 64 `ENVELOPE_ARMS` (§0–§13), the reconciliation ledger (§14) and doors covered (§15) |
| [codex/axis-2-source-pass.md](codex/axis-2-source-pass.md) · [codex/axis-3-source-pass.md](codex/axis-3-source-pass.md) · [codex/axis-5-source-pass.md](codex/axis-5-source-pass.md) | the three Codex source passes, **verbatim**, so every lead is auditable against the verdict it was driven to |
| [instrument/](instrument/) | the re-pointed instrument — the three Codex prompts and the Workflow script that staffed and ran this partial re-run |

**What this record claims and nothing more:** three axes, driven on the installed `1.0.0-rc.17` on
2026-09-22, one tier-1 row found and located outside M53's own new code, three tier-3 rows, all four of
M52's tier-1 rows on these axes closed. **The 1.0.0 call is the human's**, and the exit rule's first
clause is the one the tier-1 count above speaks to.
