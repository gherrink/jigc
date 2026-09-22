# M53 — the rc.17 fix pass: completion verdict

**Verdict: COMPLETE — built, audited, seven findings confirmed live and fixed axis-complete, re-verified
3904 passed / 0 failed.** Written 2026-09-22 after the last fix commit (`5702bf4f`), before the version
bump — the bump is the next act and is not claimed here.

| | |
|---|---|
| Planned | 2026-09-21 at `5d9fd714` ([settle record](settle-record.md) D1–D5 + §1–§14 · [gate-record](planning-gate-record.md) · [acceptance design](acceptance-design.md)) |
| Built | 36 commits, six increments, each independently validated clean by the harness (`wf_6b6da729-238`; one transient halt on a 529, resumed with the same args, no dirty tree) — the build's close at `45427083`, fold-back reading *built, not audited* |
| Audited | code review ([audit/code-review.md](audit/code-review.md)): **1 MEDIUM · 4 LOW**, no HIGH; e2e ([audit/e2e.md](audit/e2e.md)): **7 of 8** scenarios green, **1 defect** |
| Fixed | seven commits, four fixers one at a time: `4a8cec2d` · `620c1644` · `1e68662f` · `33ab0f76` · `2cb2c29f` · `c00fdd97` · `5702bf4f` — every finding confirmed by driving the built binary before any fix, the two headliners re-driven by the orchestrator; the seventh is a carried bound the human chose to close rather than carry |
| Re-verified | `dev/gate` bare: probe · fmt · clippy · build · test all exit 0 — **3904 passed / 0 failed** over 17 test binaries (build close 3898; +6 are the fixes' arms) |
| The boundary | `git diff 5d9fd714..HEAD --stat` over both packs' schemas, both manifests and both `schema-snapshots/` trees: **empty**. Zero schema-hash movement, zero `schema-version`s, zero corpora, zero `contract-version`s, zero envelope keys; the **634 goldens byte-unmoved** across the whole pass (the code reviewer's own check) |

## What the pass claimed, and whether it is true of what shipped

*The four tier-1 rows of the rc.16 per-axis review are closed, each over its class's axis, so that axes
2 · 3 · 5 re-driven on `1.0.0-rc.17` find no tier-1 row.* The code reviewer drove all five increments'
headline claims through the real binary and every one held; the e2e auditor drove all five flow-54 arms
in throwaway rigs, and the 3-process fan-out committed byte-identical output in three divergent orders.
Whether the re-review finds a tier-1 row is the re-review's to say — it is the human's gate, and this
verdict claims nothing about it.

| row | what shipped | audited |
|---|---|---|
| `(3, A3-1)` | the milestone area's complement — `merged/` two levels in — is displaced at a landed `milestone finalize` to `.jigc/displaced/<milestone-id>/`, refused at `discard`/`uninstall`, narrated under `--force`; a blocked boundary takes nothing; `materialize` clears only what it wrote | held, both squash arms, six plant loci |
| `(3, A3-2)` | a failed displacement (whole · partial · a hook writing during the commit) leaves every un-moved byte, at exit 0, one `finalize.foreign-bytes` per area with the area's count — on `findings` at the task door, stderr-only at the boundary whose envelope stays `Object(["committed"])` | held — and its own fault model surfaced the e2e defect (below) |
| the residual rule (D3) | a pin-less directory is a work unit at none of the 25 `WORK_UNIT_ID_DOORS` and none of the enumerating doors, over 3 shapes × 3 placements, no host path on any surface | held — with one un-swept reader (the MEDIUM, below) |
| `(2, DEFECT A)` | an uncommitted cherry-pick, in all four states, refused at all 12 acting `BEHALF_DOORS` rows with `MERGE_MSG`, index and HEAD byte-unchanged; `git reset` runs clean | held |
| `(5, DEFECT 1)` | `write.unslugable-title` at all five prose mints incl. `add-from-spec` and `jigc start`, before any write | held |

## The audit — seven findings, seven fixed, and two were larger classes than reported

**The sixth-wave signature, twice.** Both headline findings sit inside M53's own new code — exactly what
the exit rule anticipates for a fix pass — and both were found by the audit's instruments rather than by
the pass's own acceptance, which was green over each.

**E2E defect (graded HIGH by the orchestrator) — jigc's own `finalize-message.tmp` was foreign to the
registries.** Written as a string literal at `task.rs:3961`, in neither `TASK_AREA_FILES` nor
`MILESTONE_AREA_FILES`; when its best-effort removal fails under Increment 2's own hook racer, the new
`finalize.foreign-bytes` fired over jigc's own file and `task discard` + `uninstall` **refused at exit 1
over nothing foreign**, at both area kinds, with the narration's count off by one. Planning saw the name
and disposed it on *"removed before phase 7"* — falsified by the pass's own fault model. Flow 54 was green
because arm 2 counted plants, never the complement's contents. **Fixed at `4a8cec2d`:** `FINALIZE_MESSAGE_FILE`
joins both registry rows, the writer uses the constant, three tests proven red first (one of them had
**pinned the defect as expected output**), arm 2 gains *an area left standing holds no file jigc wrote*.
**Class derived, not taken:** exactly one literal outside the registries; `write_atomic`'s
`<name>.<pid>.<nanos>.tmp` sibling disposed as a runtime-named structural member no constant can carry.
The secondary host-absolute path on the same statement fixed through `render::repo_relative`.

**MEDIUM — `owning_milestone` still asked `is_dir()`.** The one production reader of `.jigc/milestones/`
Increment 3 did not convert: a pin-less milestone directory holding a `tasks.json` claimed a **live** task
as its sub-task, so `task finalize` refused and routed at a `milestone finalize` that answered *leftover*
— a live task with staged prose had no commit boundary but `task discard --force`. **Fixed at `620c1644`**
at the seam (the shared `carries_base_pin`, never a second copy); the class held at **one** un-fixed
reader, but the finding's **five consumers were ten**. Proven red through a real landed boundary whose
teardown faulted after the pin — no hand plant — with a control that a pinned milestone still claims.

**The four LOWs** (`1e68662f` · `33ab0f76` · `2cb2c29f` · `c00fdd97`): flow 54 arm 5's `add-from-spec`
cell was vacuous (one criterion, the degenerate one — now three, degenerate second, the unwind asserted
both halves and falsified by stubbing it); the abandon-route qualifier's *"the operation is all it
touches"* was **false at six of ten members** (driven: `git *--abort` and `git reset --merge` **destroy**
unrelated pre-staged work; `git reset` unstages it) — three clauses reworded, a driven fence over every
member's emitted route; one host-path render was **six** in the same function and the fence's `.display()`
predicate could not see `{path:?}` — widened to the `Debug` render of a path-named binding module-wide,
which caught **21 more**; `unslugable_title_finding` narrowed to private (the only one of 31 `pub fn`s in
`engine::state` with no external reference).

## The e2e half — 8 scenarios, 7 green

The failing scenario is the defect above; the seven green ones are the five arms, the order-invariant
join and the displacement fault axis. The auditor states its own bound: the genuine concurrent Task-tool
spawn is the orchestrator's main-session artifact it did **not** run — its spawn evidence is an N-process
binary sim plus the rendered template executed verbatim, which is not that proof (the M51 bound, unchanged).

## The instrument, honestly

- **Planning corrected itself on the record, five ways, all the M52 signature** (a claim that a shipped
  predicate, producer or registry transfers, false at the line) — caught by the pre-decompose review, not
  by the build. Then the audit found two more of the same shape *inside the build*.
- This harness's `grep` honours `.gitignore`. Two auditors found it independently; the M52 review's R-I
  loss evidence has that shape (its verdict stands, re-proved). Every driver was told.
- `finalize_family_registry.rs` refused the roadmap for naming a `finalize.` code with no producer — a
  plan is a live doc. Correct, and cheap to have learned at planning rather than at the close.

## Declared bounds, carried in writing

- ~~**Five pinned route lines under-state what `git *--abort` does to unrelated staged work.**~~ **Closed
  at `5702bf4f` on the human's call (2026-09-22)**: driven `Discarded`, the five owed a clause by law 1
  and were left by the L2 fixer only because `SHIPPED_ROUTE_LINES` pins their bytes. Put to the human as a
  surface decision; reworded — one clause per *fate*, not per member — the five pins re-blessed, the
  driven fence made **total** over `InProgress::ALL` (eight members owe a clause and carry one; two are
  driven `Untouched` and their empty clause is asserted). Red first: the fence named exactly the five.
  Zero goldens moved; the two inventory tables did not move (they derive noun + command only).
- **21 `Debug` path renders across 14 functions in `cli/milestone.rs`**, every one a `with_context` on an
  `anyhow` error channel — the class `UNSWEPT_PRODUCERS` already declares out for three other modules.
  Ship as a counted `DEBUG_REMAINDER` measured against the source in both directions; closing one reddens.
- Stderr-only at `milestone finalize` for the new advisory until 2.0 (the pinned envelope).
- `task discard` over an area a landed finalize left standing refuses under the shipped
  `task-discard.staged-prose` (the commit doc the teardown left) — its own code and route; whether a
  rendered commit doc is still *prose no commit has a copy of* is an open question, not this pass's.
- `reseed_sub_task_areas`' `.exists()` skip and `staged_task_prose`'s own `read_dir`, declared at D3.
- **No completeness fence over "readers that decide work-unit-ness by `is_dir()`"** — the class is
  disposed in prose (DECISIONS + the seam's doc-comment); a membership scan is new scope, named as a
  retrospective candidate rather than smuggled in. Same for the `write_atomic` name-shape rule.
- git 2.54.0's marker contract, under the family's existing deferral.

## What is next

The bump to `1.0.0-rc.17` + install, the ten version-bearing goldens regenerated and their diff checked
to carry nothing but the version string, then the **partial** per-axis review (axes 2 · 3 · 5) on the
installed binary with M52's instrument re-pointed, persisted under `per-axis-review/`, compared row by
row against M52's §A — and then stop. The 1.0.0 call is the human's.
