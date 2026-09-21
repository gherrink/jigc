I have the evidence. Writing the case.

---

# Robust case — fork `(3, A3-1)`: what the charter's fix LEAVES under `merged/`

Binary: `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.16`, code-identical to HEAD (`978577ec`). Every fixture built by driving that binary from `dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc` (two-step eval); only plants written by hand; no edits, no `cargo`. All loss claims use `command grep -rlE` with a before-control that finds the plant.

## 1 · The predicate behind `team-ready-state.md:92`, tested rather than obeyed

The exclusion reads: *"The milestone row's `merged/` is a tree the same way — the join's staging area, jigc's wholesale, never walked."* (`design/team-ready-state.md:92`; the code's own wording at `crates/engine/src/state.rs:165-167`: *"it is jigc's wholesale, and nothing inside it is walked"*.)

Grammatically this is the exact shape `methodology-docs.md:62`'s `foreclosed-by-doc` gate names: **an exclusion phrased as "outside the rule by satisfying it" is a claim about behaviour, not a boundary — read the predicate and drive it.** And the gate's own motivating case is *this same door one layer out*: M46 admitted `milestone finalize` at the predicate after `team-ready-state.md` had excluded it by name and the narration turned out false.

The predicate is **"nothing but jigc's bytes are in `merged/`."** Four drives falsify it.

**(a) `merged/` is not transient — it persists across a blocked finalize, populated.** DRIVEN:

```
$ jigc milestone finalize join-probe          # unfilled adr slots
exit=3   blocking · schema-conformance.required-slot-present     # nothing committed
$ find .jigc/milestones/join-probe
  …/merged/docs/adr:undecided-thing.md
  …/merged/docs/commit:alpha-sub.md          # materialized bodies, left on disk at exit 3
```

So the ordinary blocked-finalize path leaves a directory of readable merged doc bodies sitting in the workbench for as long as the fix takes. That is the state in which a human opens one (`.swp`), an agent drops a note, or a tool writes beside it.

**(b) A `pre-commit` hook writes there, and jigc's own record says hooks do exactly this.** DRIVEN — hook writes `merged/hook-note.txt` + `merged/docs/hook-deep.txt`, exits 0:

```
$ jigc milestone finalize hook-probe
exit=0   finalized d30c6b7 — … 2 files committed
$ command grep -rlE 'HOOK-WROTE-THIS|HOOK-IN-DOCS' .     # → nothing.  .jigc/displaced never created
```

`crates/engine/src/state.rs:214-218` records the identical writer, driven at `1d0bd171`, as the reason M52 narrowed `staged_doc_id`: *"a `pre-commit` hook that wrote `docs/agent-notes.md` into the area a `jigc milestone add-task` had just minted had that file removed by `unwind_docs` at exit 1 … one run, two answers."* M52 judged that shape a loss worth an audit fix. It is live, unfixed, one registry row over.

**(c) The M52 source pass already looked at the site and foreclosed it by the same phrase.** `completions/artifacts/M52/per-axis-review/codex/axis-3-source-pass.md:55` lists `engine/milestone.rs:1892` as *"rebuild of the registry-owned `merged/docs` staging tree"* and `engine/state.rs:415` as *"registry-bounded unwind"*, then closes with the positive claim *"I found no unguarded production removal of adopter bytes outside an ownership, transaction, cache, temporary-file, refusal, narration, or displacement seam."* That claim is **falsified** by §2 below. The exclusion was inherited from the doc, not tested — which is the gate's failure mode verbatim.

**(d) Honest counter-weight, conceded.** No pack step, workflow, guide or `SKILL.md` byte points an agent at `merged/` — I grepped `packs/`, `crates/cli/pack/`, `crates/cli/adapters/`, `QUICKSTART.md`, `MIGRATING.md`: the only hits are prose ("the merged effective state"), never a path. The one surface that prints the path is an error context (`crates/cli/src/milestone.rs:5948`, `validating the merged effective state under {staging_dir:?}`). So *"jigc invites writes there"* is **false** and I do not argue it. The claim I argue is the weaker, sufficient one: **third parties demonstrably write into gitignored working areas, and this one is the only such area where every door's guard is blind.**

## 2 · The cell table — door × plant location (all DRIVEN on rc.16 unless marked)

| # | door | plant | exit | survives | named |
|---|---|---|---|---|---|
| 1 | `milestone join` ×2 | merged/* | 0 | **yes** | n/a — join materializes nothing; `merged/` is not even created |
| 2 | `milestone finalize` **blocked** (exit 3, commits nothing) | `merged/docs/notes.txt`, `merged/docs/agent-notes.md`, `merged/docs/.adr:undecided-thing.md.swp` | 3 | **NO** | **no** — 0 mentions on either stream |
| 3 | same run | `merged/top.txt`, `merged/sub/nested/x.txt` | 3 | yes | — |
| 4 | `milestone finalize` **landed**, `squash:true` | `merged/top.txt`, `merged/sub/nested/x.txt` | **0** | **NO** | **no**; `.jigc/displaced` not created |
| 5 | `milestone finalize` **landed**, hook-written | `merged/hook-note.txt`, `merged/docs/hook-deep.txt` | **0** | **NO** | **no** |
| 6 | `milestone finalize` landed, `squash:false` | any merged plant | 0 | NO | no — **READ**, not driven: same removal site `task.rs:5981` reached from `milestone.rs:5228`; baseline §1 drove the arm for the area-root plant |
| 7 | **`milestone discard`**, **no `--force`** | `merged/top.txt`, `merged/docs/deep.txt`, `merged/sub/nested/x.txt` | **0** | **NO (all 3)** | **no** — stdout `discarded milestone:discard-probe (2 sub-task(s); workbench removed)`, **stderr empty** |
| 8 | `milestone discard --force` | `merged/top.txt`, `merged/docs/deep.txt` | 0 | **NO** | **no** — the `--force` loss narration listed **zero of two** |
| 9 | **`uninstall`** — control, same byte one level **up** | `.jigc/milestones/u-probe/root-control.txt` | **1** | **yes** | **`uninstall.foreign-bytes`**, names the path, gives the route |
| 10 | **`uninstall`** — same byte one level **in** | `merged/top.txt`, `merged/docs/deep.txt` | **0** | **NO** | **no** — and it *did* narrate the five **tracked** files it removed, which `git checkout` restores, while saying nothing about the two gitignored ones nothing restores |
| 11 | `unwind_area(Milestone)` at mint rollback | — | — | — | `state.rs:415` is a bare `remove_dir_all(merged/)`, but **unreachable**: the only `WorkArea::Milestone` unwind is `milestone.rs:707`, on the area `mint_milestone` just created, where `merged/` cannot exist. **READ. Conceded: not a live cell.** |

Rows 9↔10 are the whole case in two commands: **one run, two answers, decided by a directory level.**

Rows 7/8/10 are strictly worse than row `(3, A3-1)` itself. A3-1 is a door with *no* guard. These are the two doors that **do** carry a consent gate — `Disposition::Refuse`, `--force` the single consent — and the `merged/` carve-out **defeats the consent gate entirely**: bytes die at exit 0 with no consent asked and none given. The review graded both doors SAFE on this axis because the plant sat at the area root.

Row 2 is the one the landed-boundary warrant cannot even reach: the door **committed nothing** (exit 3) and still took three files, one of them an editor swap file, one of them named exactly like M52's own driven precedent.

## 3 · Would the re-review's axis-3 driver find it? Yes — the instrument points straight at it

`completions/artifacts/M52/per-axis-review/instrument/axis3-prompt.md` (READ) instructs the source pass: *"EVERY `remove_dir_all` / `remove_file` / `fs::remove_*` site in `crates/cli/src` **and `crates/engine/src`** (grep; list each with the guard that precedes it or **'unguarded'**)"*, and the hunt list includes *"a leftover shape the classifier cannot see (M49's `is_dir()` precedent)"* and *"a `--force` that consents to more than the door names."* Row 8 is that second hunt item exactly; rows 7/10 are the first.

More than "would" — it **already did and mis-graded it** (§1c). The re-run's brief additionally tells the source pass to re-dispose its own prior rows against the new source. After M53 ships the charter's fix, the driver's obvious next act is to re-plant in the milestone area to confirm the fix; `merged/` is then the *one* subtree where the plant still dies. And the Opus driver's cell axis (`LeftoverShape::Directory` × *untracked workbench file*) reaches it by construction.

**The sharpest consequence, and it is created by the fix rather than left by it:** post-fix, `jigc milestone finalize` populates the 1.0-pinned `committed.displaced` key from `foreign_area_paths`. With a `merged/` plant that probe returns empty, so the door will print **`"displaced": []`** — an *affirmative* completeness claim, on a pinned envelope — while destroying bytes. Today the door is silent; after the cheap arm it **lies**. The review's own A3-3 row names this exact shape at the test level (*"its empty-key assertion is true of a tree it never looks at"*); the cheap arm moves that false completeness from the test into the product surface, under a `Disposition::Displace` row whose registry doc-comment (`crates/cli/src/milestone.rs:3097`) reads *"Every door **answers for what it removes** … a door that destroys what it never named is the law-1 half-truth."*

By the charter's own tier-1 definition — *exit-0 loss or repository harm through a committing, destroying or moving door* — rows 4, 5, 7, 8, 10 are tier-1 rows, and the cheap arm ships them into the re-review that is the human's gate on the 1.0.0 call.

## 4 · Pricing against THE BOUNDARY, honestly

**What the robust arm needs, itemized.** The complement probe `engine::state::foreign_area_paths` is the **single source of truth all four doors read** — `milestone finalize` through `displace_foreign_area` (`task.rs:5856`), `discard`/`uninstall` through `task.rs:5728` (`AreaKind::Milestone`). One change there fixes rows 4, 5, 7, 8, 10 at once. `materialize` (row 2) is a second, independent one-site change.

| does it mint…? | answer |
|---|---|
| a new registry | **No.** `MILESTONE_AREA_FILES` exists; membership does not change. |
| a new finding code | **No.** `milestone.foreign-bytes` / `uninstall.foreign-bytes` / `task-discard.foreign-bytes` already fire on this probe's output. |
| a new disposition | **No.** `FINALIZE_DOOR` is already `Disposition::Displace`. |
| a new envelope key | **No.** `displaced`'s *values* widen; the pin is `Object(&["committed"])` (`render.rs:6721`). |
| a new membership predicate | **No.** `engine::state::staged_doc_id` is `pub`, and it is the exact inverse of `instance_path`, which is what `materialize` uses to name every body it writes (`engine/milestone.rs:1909`). |
| a new primitive | **No.** `displace_foreign_area` already `create_dir_all`s the destination parent (`task.rs:5874-5882`), so a two-level relative entry like `merged/docs/x.txt` is already handled, depth-agnostic. |
| a new *walk* | **This is the honest delta.** The walked-tree-with-membership mechanism exists twice (`foreign_area_paths`' `docs/` branch at `state.rs:285-297`; `unwind_docs` at `state.rs:440`), but it is hard-coded **one level** and gated on `kind == WorkArea::Task`. `merged/` needs **two** levels (`merged/` → `docs/` → bodies). A generous reading calls this *a condition on a guard that already exists*; a strict reading calls a second depth a new rule. **This is the halt the boundary asks for, and it is why this fork is in front of you.** |

**The objection that would kill it, checked and dead.** If a foreign file left in `merged/docs/` could reach the commit, selective clearing would be worse than wholesale. It cannot (READ): `engine::finalize::plan_promotions` (`finalize.rs:1141-1149`) skips any stem without a `<ty>:<slug>` split and any unknown `ty`; the gate copy loop (`milestone.rs:5843-5862`) applies the same filter before copying into its scratch tree. A left-behind `agent-notes.md` is promoted by nothing and committed by nothing.

**Fences touched.** `crates/cli/tests/task_area_writer_registry.rs:227-235` counts *writers* against the registry union — widening a *reader's* walk reddens nothing. Its only `WorkArea::Milestone` test (`:828-854`) drives a **pre-materialize** area with no `merged/` at all, so it is unaffected. `milestone_boundary_displacement.rs` never contains the string `milestones` (review A3-3), so it is unaffected. `flow53_acceptance.rs` arm 3 binds one `.jigc/tasks/<unit>` area per door, so the boundary's own area is outside it. **No shipped test pins "`merged/` is never walked."** The doc homes that change: `design/team-ready-state.md:92`, `crates/engine/src/state.rs:165-167`, and `render.rs:4836`'s *"a sub-task's working area"* doc-comment — the last is falsified by the charter's fix anyway. Zero schema-hash movement; zero engine determinism exposure (the order-invariance pins are over the *bodies jigc writes*, which are untouched — only which foreign entries survive changes).

**Sibling cells each arm creates or leaves.**
- *Cheap arm leaves:* rows 2, 4, 5, 6, 7, 8, 10 — **7 live loss cells**, five of them exit-0, two of them through doors that refuse over the identical byte one level up — **plus** a newly-minted false `displaced: []` on a 1.0-pinned envelope.
- *Robust arm leaves:* the undecodable-filename bound already declared at `task.rs:5845-5848`; a foreign directory inside `merged/` returned whole and moved whole (the shipped unit rule); `unwind_area`'s `WorkArea::Milestone` arm (`state.rs:415`) unchanged and still wholesale — **conceded, unreachable today**, and a written trigger is the correct disposition, not a fix.

## 5 · Where I concede against my own interest

- `unwind_area`'s Milestone `remove_dir_all` (row 11) is **not** a loss cell. Do not fix it; carry it as a trigger.
- No pack, workflow, guide or adapter byte points an agent at `merged/`. The "an agent was told to look there" theory is **unsupported** and I drop it.
- `milestone join` is clean (row 1) — the baseline's claim holds, re-driven.
- Row 6 (`squash:false`) is **READ-inferred**, not driven by me.
- If the human rules that a second walk depth *is* new mechanism, that ruling is defensible and this fork should then be a **halt, not a build**.

## 6 · Recommended shape (≤8 lines)

1. `engine::state::foreign_area_paths` — the `docs/` walk branch becomes a `match kind`: `Task` keeps its one-level `docs/` walk; `Milestone` walks `merged/` (its `docs/` member one level, using the *same* `staged_doc_id` predicate; every other `merged/` entry foreign, returned whole).
2. `engine::milestone::materialize` — replace `remove_dir_all(merged/docs)` with "remove every entry `staged_doc_id` recognizes", same predicate; verified safe (§4) because nothing un-addressed reaches the promote plan or the gate.
3. Nothing else. No new registry, code, disposition, envelope key, primitive or destination; the charter's `WorkArea::Milestone` parameter widening carries rows 4/5/6 through the shipped `displace_foreign_area`, and rows 7/8/10 fall out of (1) at the doors' existing `foreign-bytes` identities.
4. Acceptance iterates the axis already in this report: `{merged/ top · merged/docs/ · merged/<dir>/**} × {materialize (exit 3) · landed finalize (both squash arms) · discard ±force · uninstall}`.

**Bounds it still declares:** undecodable filenames (pre-existing, `task.rs:5845`); `unwind_area(Milestone)` left wholesale with a trigger; a foreign directory is moved whole, never merged; and milestone/task id collision under one `.jigc/displaced/<id>/` is suffix-resolved, not disambiguated (baseline §4, already owed by the charter's fix).

**If you take the cheap arm instead**, the declared bound must be written so the re-review's driver grades *against* it rather than re-finding it — and it cannot be the current sentence, because the current sentence is the falsified predicate:

> **Declared bound (M53, driven on `1.0.0-rc.16`).** `.jigc/milestones/<id>/merged/**` is excluded from the complement at every door, **knowingly**, and the exclusion is not that nothing foreign is there — it is that we accept the loss. Driven: a `pre-commit` hook's `merged/hook-note.txt` + `merged/docs/hook-deep.txt` died at a landed `milestone finalize`, exit 0, named by nothing, `.jigc/displaced` not created; three plants under `merged/` died at `jigc milestone discard` with **no `--force`**, exit 0, empty stderr, while the identical byte at the area root refuses at exit 1 with `milestone.foreign-bytes`; the same two plants died at `jigc uninstall` at exit 0 while the area-root control refused at exit 1 with `uninstall.foreign-bytes`; and `materialize`'s `remove_dir_all` (`crates/engine/src/milestone.rs:1892`) took `merged/docs/agent-notes.md` and an editor `.swp` on a finalize that **blocked at exit 3 and committed nothing**. `committed.displaced` therefore reports `[]` over a tree no door looks at. *Trigger: any adopter or trial report of a byte lost under a milestone working area, or the next wave that touches `foreign_area_paths` or `materialize`.*

---

**Verdict: robust-now.** The exclusion at `team-ready-state.md:92` is a predicate, not a boundary, and it is false in four driven ways; the cells it leaves are exit-0 loss through a **destroying** door with its consent gate defeated by a directory level — the charter's own tier-1 definition — at two doors the review graded SAFE only because the plant sat one level up; the instrument that is the human's gate already lists the site and mis-graded it by inheriting the doc's phrasing, so shipping the cheap arm ships a tier-1 row into the re-review that decides the 1.0.0 call; and the fix reuses one shipped `pub` predicate at one probe plus one removal site, minting no registry, code, disposition, key or primitive — the sole new thing being a **second walk depth**, which is the one item honestly owed to you as a halt.
